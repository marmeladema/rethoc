//! Tier 2 eligibility analysis.
//!
//! Determines whether a non-nested counter pattern qualifies for Tier 2's
//! differential-counter DFA.  The analysis is purely compile-time and does
//! not affect the Tier 2 runtime.
//!
//! # Requirements for Tier 2 eligibility
//!
//! All of the following must hold (given `non_nested_eligible == true`):
//!
//! 1. Every counter body has a **fixed byte length** > 0.
//! 2. No counter with body length > 1 contains an assertion that looks
//!    ahead ([`AssertKind::looks_ahead`](crate::AssertKind::looks_ahead)).
//! 3. If any counter body contains such an assertion, there is exactly
//!    one counter.
//! 4. The number of counters is ≤ [`MAX_TIER2_COUNTERS`](super::MAX_TIER2_COUNTERS) (64).
//! 5. Counter body byte sets are pairwise disjoint, or their overlapping
//!    increments pass the common-terminal-break check in `overlap`.
//! 6. Every increment **directly follows a consumed byte**: no assertion
//!    that looks ahead sits between the last byte of a body and its
//!    `CInc` (see [`BodyAssertions::increment_behind_lookahead`]).
//!
//! Requirement 6 is the tier's central assumption: a counter is
//! incremented on the transition that consumes the last byte of its body,
//! once per byte, for all threads of a phase together.  `(?:a\B){2,3}`
//! breaks it.  The increment is only decided one byte later, and that
//! transition may consume a byte and reach the increment again.  An
//! assertion *before* the body's byte, as in `(?:\Ba){2,3}`, is fine: a
//! thread waiting there has not counted anything yet.

use crate::{ByteClassBits, ByteMap, CounterIdx, State, StateIdx};

// ---------------------------------------------------------------------------
// Eligibility result
// ---------------------------------------------------------------------------

/// Summary of Tier 2 eligibility analysis.
///
/// Returned by [`compute_tier2_eligibility()`] and consumed by
/// `RegexBuilder::build()` to decide `tier2_eligible`.
#[derive(Debug)]
pub(crate) struct Tier2Eligibility {
    /// Per-counter `(min, max, body_byte_length)`.
    /// `body_byte_length` is 0 if the body has variable length.
    pub(crate) counter_info: Box<[crate::CounterInfo]>,
    /// True when the byte sets of different counter bodies are pairwise
    /// disjoint.
    pub(crate) disjoint_bytes: bool,
    /// True when every counter body has a fixed byte length > 0.
    pub(crate) all_fixed_length: bool,
    /// True when a counter with body_length > 1 contains an assertion
    /// that looks ahead in its body.
    pub(crate) has_lookahead_in_long_body: bool,
    /// True when an assertion that looks ahead exists in any counter body
    /// AND there is more than one counter.
    pub(crate) has_lookahead_in_multi_counter_body: bool,
    /// True when any two counters have identical body byte sets.
    /// This causes Tier 2's runtime to overcount (both counters advance
    /// simultaneously and `counter_reset` never fires).
    pub(crate) has_identical_body_bytes: bool,
    /// What the assertions inside the counter bodies are and where they
    /// sit.  All false when the counters are nested: no tier that reads
    /// it takes such a pattern.
    pub(crate) body: BodyAssertions,
}

impl Tier2Eligibility {
    /// Returns true when the pattern passes all Tier 2 requirements
    /// except the byte-overlap check.
    pub(crate) fn base_eligible(&self) -> bool {
        self.all_fixed_length
            && !self.has_lookahead_in_long_body
            && !self.has_lookahead_in_multi_counter_body
            && !self.body.increment_behind_lookahead
            && !self.counter_info.is_empty()
            && self.counter_info.len() <= super::MAX_TIER2_COUNTERS
    }

    /// Returns true under the strict (disjoint bytes) rule.
    pub(crate) fn is_eligible(&self) -> bool {
        self.base_eligible() && self.disjoint_bytes
    }

    /// Whether multiple counters may break together with the same effect.
    pub(crate) fn admits_overlap(&self, states: &[State], counter_body_states: &[bool]) -> bool {
        self.base_eligible()
            && !self.disjoint_bytes
            && !self.has_identical_body_bytes
            && super::overlap::common_terminal_break(states, counter_body_states)
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Compute Tier 2 eligibility.
///
/// `non_nested` is the caller's `non_nested_eligible`.  When it is false
/// no counting DFA tier takes the pattern, and the counter bodies are not
/// examined for assertions.
pub(crate) fn compute_tier2_eligibility(
    states: &[State],
    classes: &indexmap::set::IndexSet<ByteClassBits>,
    byte_tables: &[ByteMap],
    counters_len: usize,
    non_nested: bool,
) -> Tier2Eligibility {
    let counter_info = build_counter_info(states, byte_tables, counters_len);
    let disjoint_bytes = counter_bodies_have_disjoint_bytes(states, classes, byte_tables);

    let all_fixed_length =
        !counter_info.is_empty() && counter_info.iter().all(|c| c.body_byte_length > 0);

    let mut body = BodyAssertions::default();
    let mut has_lookahead_in_long_body = false;
    if non_nested {
        for state in states {
            if let State::CounterInstance { counter, out } = *state {
                let one = body_assertions(out, counter, states, byte_tables);
                let body_len = counter_info
                    .get(counter.idx())
                    .map_or(0, |c| c.body_byte_length);
                has_lookahead_in_long_body |= one.lookahead && body_len > 1;
                body.lookahead |= one.lookahead;
                body.increment_behind_lookahead |= one.increment_behind_lookahead;
            }
        }
    }

    let has_lookahead_in_multi_counter_body = body.lookahead && counter_info.len() > 1;

    let has_identical_body_bytes =
        counter_bodies_have_identical_bytes(states, classes, byte_tables);

    Tier2Eligibility {
        counter_info,
        disjoint_bytes,
        all_fixed_length,
        has_lookahead_in_long_body,
        has_lookahead_in_multi_counter_body,
        has_identical_body_bytes,
        body,
    }
}

// ---------------------------------------------------------------------------
// Counter info
// ---------------------------------------------------------------------------

/// Build per-counter `(min, max, body_byte_length)`.
fn build_counter_info(
    states: &[State],
    byte_tables: &[ByteMap],
    counters_len: usize,
) -> Box<[crate::CounterInfo]> {
    let mut info: Vec<crate::CounterInfo> = (0..counters_len)
        .map(|i| crate::CounterInfo {
            index: i,
            min: 0,
            max: 0,
            body_byte_length: 0,
        })
        .collect();
    // Collect min/max from CInc states.
    for s in states {
        if let State::CounterIncrement {
            counter, min, max, ..
        } = s
        {
            let ci = counter.idx();
            info[ci].min = *min;
            info[ci].max = *max;
        }
    }
    // Compute body lengths from CI states.
    for s in states {
        if let State::CounterInstance { counter, out } = s {
            let ci = counter.idx();
            let body_len = counter_body_length(*out, *counter, states, byte_tables).unwrap_or(0);
            info[ci].body_byte_length = body_len;
        }
    }
    info.into_boxed_slice()
}

// ---------------------------------------------------------------------------
// Byte-disjointness check
// ---------------------------------------------------------------------------

/// Check that the byte sets matched by different counter bodies are
/// pairwise disjoint.  When this holds, at most one counter fires `CInc`
/// on any DFA transition, so the binary `with_break` / `no_break` split
/// is correct for multiple counters.
pub(crate) fn counter_bodies_have_disjoint_bytes(
    states: &[State],
    classes: &indexmap::set::IndexSet<ByteClassBits>,
    byte_tables: &[ByteMap],
) -> bool {
    let counter_bytes = collect_counter_byte_sets(states, classes, byte_tables);
    for i in 0..counter_bytes.len() {
        for j in (i + 1)..counter_bytes.len() {
            if counter_bytes[i].0 == counter_bytes[j].0 {
                debug_assert!(false, "duplicate counter pair in byte overlap check");
                continue;
            }
            for b in 0..256 {
                if counter_bytes[i].1[b] && counter_bytes[j].1[b] {
                    return false;
                }
            }
        }
    }
    true
}

/// Check if any two counters have identical body byte sets.
///
/// When two counters have identical body bytes, both advance on every byte,
/// and `counter_reset` never fires for either (both have body-interior
/// progress).  This causes Tier 2's differential counter model to overcount.
pub(crate) fn counter_bodies_have_identical_bytes(
    states: &[State],
    classes: &indexmap::set::IndexSet<ByteClassBits>,
    byte_tables: &[ByteMap],
) -> bool {
    let counter_bytes = collect_counter_byte_sets(states, classes, byte_tables);
    for i in 0..counter_bytes.len() {
        for j in (i + 1)..counter_bytes.len() {
            if counter_bytes[i].0 == counter_bytes[j].0 {
                continue;
            }
            if counter_bytes[i].1 == counter_bytes[j].1 {
                return true;
            }
        }
    }
    false
}

/// Collect per-counter byte sets (shared by disjoint and identical checks).
fn collect_counter_byte_sets(
    states: &[State],
    classes: &indexmap::set::IndexSet<ByteClassBits>,
    byte_tables: &[ByteMap],
) -> Vec<(CounterIdx, [bool; 256])> {
    let mut counter_bytes: Vec<(CounterIdx, [bool; 256])> = Vec::new();
    for s in states {
        if let State::CounterInstance { counter, out } = s {
            let mut bytes = [false; 256];
            let mut stack = vec![*out];
            let mut visited = vec![false; states.len()];
            while let Some(idx) = stack.pop() {
                let i = idx.idx();
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                match states[idx] {
                    State::CounterIncrement { counter: c, .. } if c == *counter => {}
                    State::Split { out, out1 } => {
                        stack.push(out1);
                        stack.push(out);
                    }
                    State::Assert { out, .. } | State::CounterInstance { out, .. } => {
                        stack.push(out);
                    }
                    State::Byte {
                        byte,
                        out,
                        out_exit,
                        ..
                    } => {
                        bytes[byte as usize] = true;
                        stack.push(out);
                        if out_exit != StateIdx::NONE {
                            stack.push(out_exit);
                        }
                    }
                    State::ByteCI {
                        byte,
                        out,
                        out_exit,
                        ..
                    } => {
                        bytes[byte as usize] = true;
                        bytes[(byte ^ 0x20) as usize] = true;
                        stack.push(out);
                        if out_exit != StateIdx::NONE {
                            stack.push(out_exit);
                        }
                    }
                    State::Wildcard { out, out_exit } => {
                        for b in 0..=255u8 {
                            bytes[b as usize] = true;
                        }
                        stack.push(out);
                        if out_exit != StateIdx::NONE {
                            stack.push(out_exit);
                        }
                    }
                    State::ByteClassStatic {
                        table,
                        out,
                        out_exit,
                    } => {
                        for b in 0..=255u8 {
                            if table[b as usize] {
                                bytes[b as usize] = true;
                            }
                        }
                        stack.push(out);
                        if out_exit != StateIdx::NONE {
                            stack.push(out_exit);
                        }
                    }
                    State::ByteClassCustom {
                        class,
                        out,
                        out_exit,
                    } => {
                        let bc = &classes[class.idx()];
                        for b in 0..=255u8 {
                            if bc.contains(b) {
                                bytes[b as usize] = true;
                            }
                        }
                        stack.push(out);
                        if out_exit != StateIdx::NONE {
                            stack.push(out_exit);
                        }
                    }
                    State::ByteTable { table } => {
                        let map = &byte_tables[table.idx()];
                        for b in 0..=255u8 {
                            if map[b] != StateIdx::NONE {
                                bytes[b as usize] = true;
                                stack.push(map[b]);
                            }
                        }
                    }
                    _ => {}
                }
            }
            counter_bytes.push((*counter, bytes));
        }
    }
    counter_bytes
}

// ---------------------------------------------------------------------------
// Body-length computation
// ---------------------------------------------------------------------------

/// Compute the fixed byte-length of a counter body.
///
/// Returns `Some(len)` if all paths through the body consume exactly
/// `len` bytes, `None` if variable-length.
pub(crate) fn counter_body_length(
    ci_out: StateIdx,
    own_counter: CounterIdx,
    states: &[State],
    byte_tables: &[ByteMap],
) -> Option<usize> {
    let mut result: Option<usize> = None;
    let mut stack: Vec<(StateIdx, usize)> = vec![(ci_out, 0)];
    let mut visited: Vec<Option<usize>> = vec![None; states.len()];
    visited[ci_out.idx()] = Some(0);
    while let Some((idx, depth)) = stack.pop() {
        let mut enqueue = |succ: StateIdx, depth| {
            match visited[succ.idx()] {
                Some(prev) if prev != depth => return None,
                Some(_) => {}
                None => {
                    visited[succ.idx()] = Some(depth);
                    stack.push((succ, depth));
                }
            }
            Some(())
        };
        match states[idx] {
            State::CounterIncrement { counter, .. } if counter == own_counter => match result {
                None => result = Some(depth),
                Some(prev) if prev != depth => return None,
                _ => {}
            },
            State::Split { out, out1 } => {
                for succ in [out, out1] {
                    enqueue(succ, depth)?;
                }
            }
            State::Assert { out, .. } | State::CounterInstance { out, .. } => {
                enqueue(out, depth)?;
            }
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                let nd = depth + 1;
                enqueue(out, nd)?;
                // The exit branch re-enters at the same depth as the
                // consuming step (it branches post-consumption).
                if out_exit != StateIdx::NONE {
                    enqueue(out_exit, nd)?;
                }
            }
            State::ByteTable { table } => {
                let nd = depth + 1;
                for &succ in &byte_tables[table.idx()].0 {
                    if succ != StateIdx::NONE {
                        enqueue(succ, nd)?;
                    }
                }
            }
            _ => {
                debug_assert!(
                    !matches!(states[idx], State::Match),
                    "Match state reachable in counter body walk from state {}",
                    idx.idx()
                );
            }
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Assertions inside counter bodies
// ---------------------------------------------------------------------------

/// Visit the states of the counter body that starts at `start` (the
/// `out` of the counter's `CounterInstance`), each once, up to but not
/// including the counter's own `CounterIncrement`.  Stops and returns
/// true as soon as `visit` does.
///
/// Every check on what a body contains goes through this walk, so that
/// they agree on what the body is.
pub(crate) fn any_in_body(
    start: StateIdx,
    own_counter: CounterIdx,
    states: &[State],
    byte_tables: &[ByteMap],
    mut visit: impl FnMut(&State) -> bool,
) -> bool {
    let mut stack = vec![start];
    let mut visited = vec![false; states.len()];
    while let Some(idx) = stack.pop() {
        if std::mem::replace(&mut visited[idx.idx()], true) {
            continue;
        }
        let state = &states[idx];
        if matches!(state, State::CounterIncrement { counter, .. } if *counter == own_counter) {
            continue;
        }
        if visit(state) {
            return true;
        }
        match *state {
            State::Split { out, out1 } => {
                stack.push(out1);
                stack.push(out);
            }
            State::Assert { out, .. } | State::CounterInstance { out, .. } => stack.push(out),
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                stack.push(out);
                if out_exit != StateIdx::NONE {
                    stack.push(out_exit);
                }
            }
            State::ByteTable { table } => {
                stack.extend(
                    byte_tables[table.idx()]
                        .0
                        .iter()
                        .copied()
                        .filter(|&succ| succ != StateIdx::NONE),
                );
            }
            _ => {}
        }
    }
    false
}

/// Whether the increment of `counter` can be reached from one of `starts`
/// through epsilon transitions only: splits, assertions and counter
/// entries.  Consuming states, other increments and `Match` end a path.
///
/// One walk with one visited set, however many starts there are.
pub(crate) fn epsilon_reaches_increment(
    starts: impl IntoIterator<Item = StateIdx>,
    counter: CounterIdx,
    states: &[State],
) -> bool {
    let mut stack: Vec<StateIdx> = starts.into_iter().collect();
    let mut visited = vec![false; states.len()];
    while let Some(idx) = stack.pop() {
        if std::mem::replace(&mut visited[idx.idx()], true) {
            continue;
        }
        match states[idx] {
            State::CounterIncrement { counter: c, .. } if c == counter => return true,
            State::Split { out, out1 } => {
                stack.push(out1);
                stack.push(out);
            }
            State::Assert { out, .. } | State::CounterInstance { out, .. } => stack.push(out),
            _ => {}
        }
    }
    false
}

/// What the assertions inside counter bodies are and where they sit: the
/// input of the counting tiers' eligibility rules.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BodyAssertions {
    /// A body contains an assertion that looks ahead
    /// ([`AssertKind::looks_ahead`](crate::AssertKind::looks_ahead): the
    /// word-boundary family, `(?m:$)`, `$`, the CRLF anchors).
    ///
    /// `^` and `(?m:^)` do not count: they are decided by what is behind
    /// them, so a thread never waits on them.
    pub(crate) lookahead: bool,
    /// Inside a body, the increment can be reached without consuming a
    /// byte from an assertion that looks ahead, as in `(?:a\B){2,3}`,
    /// `(?:a\b|b){2,3}` or `(?:a$){1,2}`.
    ///
    /// Such an increment does not happen on the transition that consumes
    /// the body's last byte: whether the iteration counts is decided one
    /// byte later.  Tier 2 excludes it (requirement 6 in the module
    /// docs, Bug 68).  An assertion in front of the body's byte is not
    /// one of these: `(?:\Ba){2,3}` reaches a consuming state first.
    ///
    /// `^` and `(?m:^)` in that position are not reported: tier 2
    /// settles them when it computes the transition.
    pub(crate) increment_behind_lookahead: bool,
}

/// Examine the body of `own_counter`, which starts at `start`: one walk
/// over the body, then one epsilon walk from its assertions.
pub(crate) fn body_assertions(
    start: StateIdx,
    own_counter: CounterIdx,
    states: &[State],
    byte_tables: &[ByteMap],
) -> BodyAssertions {
    // Where the body's assertions that look ahead lead.
    let mut behind_lookahead = Vec::new();
    any_in_body(start, own_counter, states, byte_tables, |state| {
        if let State::Assert { kind, out } = *state
            && kind.looks_ahead()
        {
            behind_lookahead.push(out);
        }
        false
    });

    let lookahead = !behind_lookahead.is_empty();
    let increment_behind_lookahead =
        lookahead && epsilon_reaches_increment(behind_lookahead, own_counter, states);
    BodyAssertions {
        lookahead,
        increment_behind_lookahead,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Regex, RegexConfig};

    /// Compile without unrolling so small repetitions keep their counters.
    fn build(pattern: &str) -> Regex {
        Regex::with_config(
            pattern,
            RegexConfig {
                max_unroll_states: 0,
                ..Default::default()
            },
        )
        .unwrap()
    }

    /// What `body_assertions` says about the body of the pattern's first
    /// counter.
    fn first_body(pattern: &str) -> BodyAssertions {
        let re = build(pattern);
        re.states
            .iter()
            .find_map(|s| match s {
                State::CounterInstance { counter, out } => {
                    Some(body_assertions(*out, *counter, &re.states, &re.byte_tables))
                }
                _ => None,
            })
            .expect("the pattern has a counter")
    }

    /// Bug 68: an increment is "behind an assertion" when an assertion
    /// that looks ahead sits between the last byte of the body and the
    /// increment, on any path.
    #[test]
    fn test_increment_behind_assertion() {
        for (pattern, expected) in [
            // After the byte.
            (r"(?:a\b){2,3}c", true),
            (r"(?:a\B){2,3}c", true),
            (r"(?:[ab]\b{end}){2,3}c", true),
            (r"(?:a(?m:$)){2,3}", true),
            (r"(?:a$){1,2}", true),
            // On one branch only.
            (r"(?:a\B|b){2,3}c", true),
            (r"(?:b|a$){2,3}", true),
            (r"(?:a(?:b|c\b)){2,3}d", true),
            // Behind a second assertion, or an optional one.
            (r"(?:a\b\B){2,3}c", true),
            (r"(?:a(?m:^)\b){2,3}c", true),
            (r"(?:a\b(?m:^)){2,3}c", true),
            (r"(?:ab\B){2,3}c", true),
            (r"(?:a(?:\b|b)){2,3}c", true),
            // In front of the byte: the increment follows a consumed byte.
            (r"(?:\ba){2,3}c", false),
            (r"(?:\B[ab]){2,3}c", false),
            (r"(?:\Ba|b){2,3}c", false),
            (r"(?:$a|b){2,3}c", false),
            (r"(?:\ba\Bb){2,3}c", false),
            (r"(?:(?m:^)a+\n){2,3}c", false),
            (r"(?:^a|bb){2,3}c", false),
            // Between two bytes of the body.
            (r"(?:\n(?m:^)a|bb){2,3}c", false),
            (r"(?:[ab](?m:^)c){2,3}", false),
            (r"(?:a(?:(?m:^)b|c)){2,3}d", false),
            // Behind an assertion that is decided by what is behind it.
            (r"(?:\n(?m:^)){2,3}c", false),
            (r"(?:a^){2,3}c", false),
            (r"(?:a(?m:^)|\n\n){2,3}c", false),
            (r"(?:\n(?m:^)|aa){2,3}c", false),
            (r"(?:\Ba(?m:^)|bb){2,3}c", false),
            // No assertion in the body at all.
            (r"\b(?:a|b){2,3}\bc", false),
            (r"a{2,3}$", false),
            (r"(?m:^)(?:a|bb){2,3}$", false),
        ] {
            assert_eq!(
                first_body(pattern).increment_behind_lookahead,
                expected,
                "{pattern}"
            );
        }
    }

    /// Bug 67: `$` counts as an assertion that looks ahead inside a body,
    /// like the word-boundary family and `(?m:$)`; `^` and `(?m:^)` do
    /// not.
    #[test]
    fn test_body_has_lookahead() {
        for (pattern, expected) in [
            (r"(?:a$|bb){2,3}", true),
            (r"(?:$a|b){2,3}c", true),
            (r"(?:a\b){2,3}c", true),
            (r"(?:\Ba){2,3}c", true),
            (r"(?:a(?m:$)){2,3}", true),
            (r"(?:\b{start}a.){2,3}c", true),
            (r"(?:\n(?m:^)){2,3}c", false),
            (r"(?:^a|b){2,3}c", false),
            (r"\b(?:a|b){2,3}\bc$", false),
        ] {
            assert_eq!(first_body(pattern).lookahead, expected, "{pattern}");
        }
    }

    /// The body flags are only computed for patterns a counting DFA tier
    /// can take.
    #[test]
    fn test_body_assertions_skipped_for_nested_counters() {
        let body_of = |pattern: &str, non_nested: bool| {
            let re = build(pattern);
            let classes: indexmap::IndexSet<ByteClassBits> = re.classes.iter().copied().collect();
            compute_tier2_eligibility(
                &re.states,
                &classes,
                &re.byte_tables,
                re.num_counters,
                non_nested,
            )
            .body
        };
        let pattern = r"(?:a\b){2,3}c";
        assert_eq!(
            body_of(pattern, true),
            BodyAssertions {
                lookahead: true,
                increment_behind_lookahead: true,
            }
        );
        assert_eq!(body_of(pattern, false), BodyAssertions::default());
    }

    /// What the checks mean for the tiers.
    #[test]
    fn test_lookahead_in_body_tiers() {
        for (pattern, tier2, tier3, tier4) in [
            // In front of a one-byte body, one counter: tier 2.
            (r"(?:\B[ab]){2,3}c", true, false, false),
            (r"(?:\Ba|b){2,3}c", true, false, false),
            // Behind the byte: no counting DFA tier takes a word boundary
            // in a body; `$` is left to tier 4.
            (r"(?:[ab]\B){2,3}c", false, false, false),
            (r"(?:a\B|b){2,3}c", false, false, false),
            (r"(?:a$|b){2,3}", false, false, true),
            (r"(?:a$|bb){2,3}", false, false, true),
            (r"(?:ab$|ba){2,3}", false, false, true),
            // `^` and `(?m:^)` anywhere in a body: every tier that takes
            // the body takes them.  (Tier 3 is not always right on them:
            // see the `tier3_broken` entries in `match_tests!`.)
            (r"(?:(?m:^)a+\n){2,3}c", false, true, true),
            (r"(?:^a|bb){2,3}c", false, true, true),
            (r"(?:(?m:^)a|b){2,3}c", true, true, true),
            (r"(?:\n(?m:^)a|bb){2,3}c", true, true, true),
            (r"(?:[ab\n](?m:^)c){2,3}", true, true, true),
            (r"(?:\n(?m:^)){2,3}c", true, true, true),
            (r"(?:a(?m:^)|\n\n){2,3}c", false, true, true),
            (r"(?:\na(?m:^)|bb){2,3}c", true, true, true),
            // Outside the body: unaffected.
            (r"\b[ab]{2,3}\bc", true, true, false),
            (r"(?m:^)[ab]{2,3}$", true, true, true),
            (r"(?:a|bb){2,3}\n(?m:^)c", false, true, true),
        ] {
            let re = build(pattern);
            assert_eq!(
                (re.tier2_eligible, re.tier3_eligible, re.tier4_eligible),
                (tier2, tier3, tier4),
                "{pattern}"
            );
        }
    }
}
