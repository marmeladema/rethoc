//! Tier 1: Lazy DFA for counter-free patterns.
//!
//! Standard lazy subset construction with support for deferred
//! assertions, including CRLF anchors, with exact boundary and EOF context.

use std::fmt;

use crate::{AssertEval, Prefilter, Regex, State, StateIdx, byte_match_ci};

use super::{BoundaryContext, DfaCache, DfaMemory, DfaStateId};

#[cfg(test)]
mod assertion_tests;

/// Push consumption targets for NFA state `idx` when it consumes `byte`.
///
/// For consuming states with `out_exit != NONE`, both `out` and `out_exit`
/// are pushed (fused consume-and-branch semantics).
fn push_consume_targets(state: &State, byte: u8, regex: &Regex, targets: &mut Vec<StateIdx>) {
    match *state {
        State::Byte {
            byte: b2,
            out,
            out_exit,
        } if byte == b2 => {
            targets.push(out);
            if out_exit != StateIdx::NONE {
                targets.push(out_exit);
            }
        }
        State::ByteCI {
            byte: b2,
            out,
            out_exit,
        } if byte_match_ci(byte, b2) => {
            targets.push(out);
            if out_exit != StateIdx::NONE {
                targets.push(out_exit);
            }
        }
        State::Wildcard { out, out_exit } => {
            targets.push(out);
            if out_exit != StateIdx::NONE {
                targets.push(out_exit);
            }
        }
        State::ByteClassStatic {
            table,
            out,
            out_exit,
        } if table[byte as usize] => {
            targets.push(out);
            if out_exit != StateIdx::NONE {
                targets.push(out_exit);
            }
        }
        State::ByteClassCustom {
            class,
            out,
            out_exit,
        } if regex.classes[class.idx()].contains(byte) => {
            targets.push(out);
            if out_exit != StateIdx::NONE {
                targets.push(out_exit);
            }
        }
        State::ByteTable { table } => {
            let t = regex.byte_tables[table][byte];
            if t != StateIdx::NONE {
                targets.push(t);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Tagged state ids (Tier 1)
// ---------------------------------------------------------------------------

/// A Tier 1 transition-table entry: a row offset with tag bits on top.
///
/// The low bits are the state's index in [`DfaCache::states`] multiplied by
/// the row width ([`Tier1DfaCache::stride`]), that is the offset of the
/// state's row in the transition table: a lookup adds the byte's column
/// and nothing else.  The tags say everything the
/// scanning loop has to know about the entry, so that one comparison
/// ([`is_tagged`](Self::is_tagged)) tells an ordinary transition from the
/// special ones: a slot that is not computed yet, a match state and, when
/// the regex has a prefilter, the start state.
///
/// There is no dead state among them.  A state with no live NFA state is
/// interned like any other (the empty state): its row holds the
/// transitions that restart from the start closure, and an input that can
/// no longer match loops on it inside the ordinary run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Tier1StateId(u32);

impl Tier1StateId {
    /// Tag of the start state in the table of a regex with a prefilter:
    /// entering it hands the input over to the prefilter.  The matcher's
    /// current state never carries it.
    const START_TAG: u32 = 1 << 28;
    /// The start state as a current state: state 0, so row offset 0, also
    /// after the cache is emptied.
    const START: Self = Self(0);
    /// Tag of a state whose `is_match` flag is set; the offset is kept.
    const MATCH: u32 = 1 << 29;
    /// A transition that is not computed yet.
    const UNKNOWN: Self = Self(1 << 31);
    /// Largest untagged value, and the mask of the offset bits.
    const MAX: u32 = Self::START_TAG - 1;

    /// Whether any tag is set: unknown, match or start.
    #[inline(always)]
    fn is_tagged(self) -> bool {
        self.0 > Self::MAX
    }

    #[inline(always)]
    fn is_match(self) -> bool {
        self.0 & Self::MATCH != 0
    }

    #[inline(always)]
    fn is_start(self) -> bool {
        self.0 & Self::START_TAG != 0
    }

    /// The entry as a current state: without the start tag.
    #[inline(always)]
    fn current(self) -> Self {
        Self(self.0 & !Self::START_TAG)
    }

    /// Offset of the state's row in the transition table.  Not for
    /// `UNKNOWN`.
    #[inline(always)]
    fn row(self) -> usize {
        (self.0 & Self::MAX) as usize
    }
}

/// Most states a Tier 1 cache can be asked to hold: every row offset has
/// to fit below the tag bits, at the widest row (256 columns).
pub(crate) const TIER1_MAX_STATES: usize = (Tier1StateId::MAX as usize + 1) / 256;

// ---------------------------------------------------------------------------
// DFA cache (Tier 1)
// ---------------------------------------------------------------------------

/// Lazy DFA cache: append-only state table + flat transition table.
///
/// The transition table is a flat `Vec<Tier1StateId>` indexed by
/// `state index * stride + byte_class`, where `stride` is the number
/// of byte equivalence classes ([`Regex::num_byte_classes`]) rounded up to
/// a power of two, so that entries can hold row offsets.
/// Unpopulated slots contain `Tier1StateId::UNKNOWN`.  When the
/// state count reaches the cap, the cache is emptied and refilled on
/// demand (see [`DfaCache`]); answers do not depend on what is cached.
///
/// The cache is **persisted across `matcher()` calls** for the same
/// `Regex`.  A unique regex ID detects when a different regex is used
/// and clears the cache.
// The scanning loop reads only the transition vector's pointer and length:
// entries hold row offsets, so `stride` is no longer read per byte.  The
// field order dates from when it was; nothing depends on it.
#[repr(C)]
pub(crate) struct Tier1DfaCache {
    inner: DfaCache,
    /// Flat transition table: `transitions[row offset + byte_class]`.
    /// Unpopulated slots contain `Tier1StateId::UNKNOWN`.
    transitions: Vec<Tier1StateId>,
    /// Row width: a power of two, at least the number of byte classes.
    stride: usize,
    /// Whether transitions into the start state carry
    /// [`Tier1StateId::START_TAG`]: the regex has a prefilter.
    tag_start: bool,
    /// Reusable post-consumption roots, used only on cache misses.
    targets: Vec<StateIdx>,
}

impl Tier1DfaCache {
    pub(crate) fn new() -> Self {
        Self {
            inner: DfaCache::new(),
            transitions: Vec::new(),
            stride: 256,
            tag_start: false,
            targets: Vec::new(),
        }
    }

    /// Set the state cap (see [`DfaCache::set_max_states`]).
    pub(crate) fn set_max_states(&mut self, max_states: usize) {
        self.inner.set_max_states(max_states.min(TIER1_MAX_STATES));
    }

    /// The table entry for a state: its row offset, tagged when it is a
    /// match.
    #[inline]
    fn tagged(&self, id: DfaStateId) -> Tier1StateId {
        debug_assert!(id != DfaStateId::DEAD, "Tier 1 interns the empty state");
        let mut row = id.0 << self.stride.trailing_zeros();
        debug_assert!(row <= Tier1StateId::MAX);
        if self.tag_start && id == DfaStateId::START {
            row |= Tier1StateId::START_TAG;
        }
        if self.inner.states[id.idx()].is_match {
            row |= Tier1StateId::MATCH;
        }
        Tier1StateId(row)
    }

    /// The plain id of the state behind a table entry.
    #[inline]
    fn untagged(&self, id: Tier1StateId) -> DfaStateId {
        DfaStateId((id.row() >> self.stride.trailing_zeros()) as u32)
    }

    /// The state behind a table entry.  Not for `UNKNOWN`.
    #[inline]
    fn state(&self, id: Tier1StateId) -> &super::DfaState {
        &self.inner.states[id.row() >> self.stride.trailing_zeros()]
    }

    /// Number of times the cache was emptied because it was full.
    #[cfg(test)]
    pub(crate) fn evictions(&self) -> u64 {
        self.inner.evictions
    }

    /// The state cap in effect.
    #[cfg(test)]
    pub(crate) fn max_states(&self) -> usize {
        self.inner.max_states
    }

    /// Make room for the state a missing transition out of `from` can
    /// add: when the cache is full, empty it and put `from` back.
    /// Returns the id `from` has afterwards.
    #[inline]
    fn make_room(&mut self, from: DfaStateId) -> DfaStateId {
        if self.inner.has_room(1) {
            from
        } else {
            self.evict(from)
        }
    }

    #[cold]
    fn evict(&mut self, from: DfaStateId) -> DfaStateId {
        let stride = self.stride;
        let mut keep = [from];
        self.transitions.clear();
        self.inner.evict(&mut keep, || {
            self.transitions
                .extend(std::iter::repeat_n(Tier1StateId::UNKNOWN, stride));
        });
        keep[0]
    }

    /// Look up or insert a DFA state for the given sorted NFA state set.
    /// Callers make room first ([`make_room`](Self::make_room)), so the
    /// state always fits.
    fn intern_state(
        &mut self,
        nfa_states: &[StateIdx],
        deferred_asserts: &[StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        context: BoundaryContext,
    ) -> DfaStateId {
        let stride = self.stride;
        self.inner
            .intern_context(
                nfa_states,
                deferred_asserts,
                is_match,
                is_match_at_end,
                context,
                || {
                    self.transitions
                        .extend(std::iter::repeat_n(Tier1StateId::UNKNOWN, stride));
                },
            )
            .expect("DFA state cap reached although room was made")
    }

    /// Close an unknown-next or known-next boundary without consuming bytes.
    /// Exact EOF acceptance comes from raw roots before context is reduced.
    /// Known-next closure never predicts EOF at the old position.
    fn closure<const ASSERTIONS: bool>(
        memory: &mut DfaMemory,
        seeds: impl Iterator<Item = StateIdx>,
        regex: &Regex,
        context: BoundaryContext,
        next: Option<u8>,
    ) -> (bool, bool, BoundaryContext) {
        let assertions = &regex.dfa_assertions;
        debug_assert_eq!(ASSERTIONS, !assertions.is_empty());
        memory.closure_stack.clear();
        memory.closure_result.clear();
        memory.closure_deferred.clear();
        memory.closure_visited.fill(false);
        let mut at_end = false;
        for seed in seeds {
            if ASSERTIONS && next.is_none() {
                at_end |= assertions.eof_accept(seed, context.0);
            }
            memory.closure_stack.push(seed);
        }
        let mut matched = false;
        let mut needs = 0;
        while let Some(idx) = memory.closure_stack.pop() {
            if memory.closure_visited[idx.idx()] {
                continue;
            }
            memory.closure_visited[idx.idx()] = true;
            match regex.states[idx] {
                State::Split { out, out1 } => {
                    memory.closure_stack.push(out1);
                    memory.closure_stack.push(out);
                }
                State::Assert { kind, out } => {
                    debug_assert!(ASSERTIONS);
                    match kind.eval(context.at_start(), false, context.previous(), next) {
                        AssertEval::Pass => memory.closure_stack.push(out),
                        AssertEval::Fail => {}
                        AssertEval::Defer => {
                            debug_assert!(next.is_none());
                            memory.closure_deferred.push(idx);
                            needs |= assertions.requirements(idx);
                        }
                    }
                }
                State::Match => matched = true,
                State::CounterInstance { .. } | State::CounterIncrement { .. } => {
                    unreachable!("Tier 1 has no counters")
                }
                State::Byte { .. }
                | State::ByteCI { .. }
                | State::Wildcard { .. }
                | State::ByteClassStatic { .. }
                | State::ByteClassCustom { .. }
                | State::ByteTable { .. } => memory.closure_result.push(idx),
            }
        }
        memory.closure_result.sort_unstable();
        memory.closure_deferred.sort_unstable();
        if !ASSERTIONS {
            at_end = matched;
        }
        (matched, at_end, context.masked(needs))
    }

    /// The transition out of `from` on `byte`, computed if it is missing.
    /// The result is never `UNKNOWN`.  Matchers go through
    /// [`DfaMatcher::scan`]; this is the same lookup one byte at a time,
    /// for tests that walk the table.
    #[cfg(test)]
    fn transition_tagged(
        &mut self,
        memory: &mut DfaMemory,
        from: Tier1StateId,
        byte: u8,
        regex: &Regex,
    ) -> Tier1StateId {
        let class = regex.byte_classes[byte as usize] as usize;
        let cached = self.transitions[from.row() + class];
        if cached != Tier1StateId::UNKNOWN {
            return cached;
        }
        self.transition_miss(memory, from, byte, class, regex)
    }

    /// [`transition_tagged`](Self::transition_tagged) on plain state ids.
    #[cfg(test)]
    fn transition(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        let from = self.tagged(from);
        let to = self.transition_tagged(memory, from, byte, regex);
        self.untagged(to)
    }

    /// Cache miss: compute the transition out of `from` on `byte` (whose
    /// column in the table is `class`) and store it.  Kept out of line so
    /// the lookup stays small.
    #[cold]
    #[inline(never)]
    fn transition_miss(
        &mut self,
        memory: &mut DfaMemory,
        from: Tier1StateId,
        byte: u8,
        class: usize,
        regex: &Regex,
    ) -> Tier1StateId {
        // `from` gets a new id if the cache has to be emptied.
        let from = self.make_room(self.untagged(from));
        let to = self.populate(memory, from, byte, regex);
        let to = self.tagged(to);
        self.transitions[from.idx() * self.stride + class] = to;
        to
    }

    /// Select the assertion policy only on a transition miss.
    fn populate(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        if !regex.dfa_assertions.is_empty() {
            self.populate_inner::<true>(memory, from, byte, regex)
        } else {
            self.populate_inner::<false>(memory, from, byte, regex)
        }
    }

    /// Resolve the old boundary, consume exactly once, then seed the new one.
    ///
    /// The result can be the empty state: nothing live, no match.  It is
    /// interned too, and its own transitions come out of this function
    /// with nothing to consume, that is from the start closure alone.
    fn populate_inner<const ASSERTIONS: bool>(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        self.targets.clear();
        let mut resolved_match = false;
        let state = &self.inner.states[from.idx()];
        if ASSERTIONS && !state.deferred_asserts.is_empty() {
            (resolved_match, _, _) = Self::closure::<true>(
                memory,
                state.deferred_asserts.iter().copied(),
                regex,
                state.context,
                Some(byte),
            );
            for &idx in &memory.closure_result {
                push_consume_targets(&regex.states[idx], byte, regex, &mut self.targets);
            }
        }
        for &idx in &state.nfa_states {
            push_consume_targets(&regex.states[idx], byte, regex, &mut self.targets);
        }
        let context = if ASSERTIONS {
            BoundaryContext::new(Some(byte))
        } else {
            BoundaryContext::legacy(false)
        };
        let (matched, at_end, context) = Self::closure::<ASSERTIONS>(
            memory,
            self.targets
                .iter()
                .copied()
                .chain(std::iter::once(regex.start)),
            regex,
            context,
            None,
        );
        let matched = matched || resolved_match;
        self.intern_state(
            &memory.closure_result,
            &memory.closure_deferred,
            matched,
            at_end,
            context,
        )
    }

    /// Reset the cache for reuse with a new regex.
    fn clear(&mut self, memory: &mut DfaMemory, num_nfa_states: usize, stride: usize) {
        self.inner.clear();
        self.targets.clear();
        self.stride = stride;
        memory.clear(num_nfa_states);
        self.transitions.clear();
        // Room for the smallest cache a matcher can run with, so that the
        // first states of a fresh cache do not each move the tables: a
        // quarter of the instructions of a 32-byte scan on a fresh cache.
        self.transitions.reserve(stride * super::DFA_MIN_STATES);
        self.inner.states.reserve(super::DFA_MIN_STATES);
    }

    /// Prepare the cache for use with `regex`.
    #[inline]
    pub(crate) fn prepare(&mut self, memory: &mut DfaMemory, regex: &Regex) {
        let id = regex.id;
        if self.inner.regex_id == id && self.inner.start_id != DfaStateId::DEAD {
            return;
        }
        self.clear(
            memory,
            regex.states.len(),
            regex.num_byte_classes.next_power_of_two(),
        );
        self.inner.regex_id = id;
        self.tag_start = !matches!(regex.prefilter, Prefilter::None);

        // Keep real start context until pending continuations are known.
        let (is_match, is_match_at_end, context) = if !regex.dfa_assertions.is_empty() {
            Self::closure::<true>(
                memory,
                std::iter::once(regex.start),
                regex,
                BoundaryContext::new(None),
                None,
            )
        } else {
            Self::closure::<false>(
                memory,
                std::iter::once(regex.start),
                regex,
                BoundaryContext::legacy(false),
                None,
            )
        };
        self.inner.start_id = self.intern_state(
            &memory.closure_result,
            &memory.closure_deferred,
            is_match,
            is_match_at_end,
            context,
        );
        self.inner.start_is_match = self.inner.states[self.inner.start_id.idx()].is_match;
    }
}

impl fmt::Debug for Tier1DfaCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tier1DfaCache")
            .field("num_states", &self.inner.states.len())
            .field("max_states", &self.inner.max_states)
            .field("evictions", &self.inner.evictions)
            .field("targets_len", &self.targets.len())
            .field("targets_capacity", &self.targets.capacity())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Tier 1 DFA matcher (counter-free)
// ---------------------------------------------------------------------------

/// Whether `state` is the empty state: nothing live and no match, what the
/// other tiers call the dead state.
#[cfg(test)]
fn is_empty_state(state: &super::DfaState) -> bool {
    state.nfa_states.is_empty()
        && state.deferred_asserts.is_empty()
        && !state.is_match
        && !state.is_match_at_end
}

/// Lazy DFA matcher for Tier 1 (counter-free) patterns.
pub struct DfaMatcher<'a> {
    cache: &'a mut Tier1DfaCache,
    memory: &'a mut DfaMemory,
    regex: &'a Regex,
    /// The current state; carries the match tag once `ever_matched` is set
    /// by a transition.
    current: Tier1StateId,
    ever_matched: bool,
    prefilter: Prefilter,
}

impl<'a> DfaMatcher<'a> {
    #[inline]
    pub(crate) fn new(
        cache: &'a mut Tier1DfaCache,
        memory: &'a mut DfaMemory,
        regex: &'a Regex,
    ) -> Self {
        // The start state's tag comes from the cached flag: no table access.
        debug_assert_eq!(cache.inner.start_id, DfaStateId::START);
        let start_is_match = cache.inner.start_is_match;
        let tag = if start_is_match {
            Tier1StateId::MATCH
        } else {
            0
        };
        DfaMatcher {
            current: Tier1StateId(Tier1StateId::START.0 | tag),
            ever_matched: start_is_match,
            cache,
            memory,
            regex,
            prefilter: regex.prefilter,
        }
    }

    #[inline(always)]
    pub fn chunk(&mut self, input: &[u8]) {
        if self.ever_matched {
            return;
        }
        let regex = self.regex;
        let no_finder = |_: &[u8]| None;
        // With a prefilter, the start state hands the input over to it:
        // bytes that cannot start a match loop back to the start state, so
        // memchr skips to the next candidate instead.  This happens every
        // time the DFA returns to the start state, not only at chunk entry.
        // Bytes are their own columns only without byte classes.  A regex
        // with 129 to 255 classes also has 256-wide rows, but its columns
        // are classes.
        if regex.num_byte_classes == 256 {
            crate::prefilter::with_prefilter!(
                self.prefilter,
                self.scan::<false>(input, |b| b as usize, no_finder),
                |find| self.scan::<true>(input, |b| b as usize, find)
            )
        } else {
            let class = |b: u8| regex.byte_classes[b as usize] as usize;
            crate::prefilter::with_prefilter!(
                self.prefilter,
                self.scan::<false>(input, class, no_finder),
                |find| self.scan::<true>(input, class, find)
            )
        }
    }

    /// Scan `input` from the current state, stopping at the first match
    /// state.  `class` maps a byte to its column; `finder` is the prefilter
    /// when `PREFILTER` is set, and unused otherwise.
    ///
    /// The current state and the table live in locals; the table is
    /// borrowed again after anything that can move or empty it.
    ///
    /// Almost every transition leads to an ordinary state, so those are
    /// followed in a run that does nothing else: the entry at the current
    /// row offset plus the column, and one test that the entry has no tag.
    /// A tagged entry ends the run and is dealt with below it: a transition
    /// to compute, a match or, with a prefilter, the start state.
    ///
    /// The run is not unrolled.  It waits on the chain of table loads, so
    /// four bytes per iteration saved instructions but no time on 2 KiB
    /// lines, and cost 4% on 35-byte lines, where the leftover bytes of
    /// each line mispredict (rebar `imported/lh3lh3-reb/uri-or-email`).
    #[inline(always)]
    fn scan<const PREFILTER: bool>(
        &mut self,
        input: &[u8],
        class: impl Fn(u8) -> usize,
        finder: impl Fn(&[u8]) -> Option<usize>,
    ) {
        let len = input.len();
        let mut i = 0;
        let mut cur = self.current;
        if PREFILTER && cur == Tier1StateId::START {
            match finder(input) {
                Some(offset) => i = offset,
                None => return,
            }
        }
        let mut table = &self.cache.transitions[..];
        let lookup = |table: &[Tier1StateId], from: Tier1StateId, byte: u8| {
            table[from.0 as usize + class(byte)]
        };
        while i < len {
            // `next` is the tagged entry out of `from` on `input[i]`.
            let (from, mut next) = 'run: {
                while i < len {
                    let next = lookup(table, cur, input[i]);
                    if next.is_tagged() {
                        break 'run (cur, next);
                    }
                    cur = next;
                    i += 1;
                }
                self.current = cur;
                return;
            };
            if next == Tier1StateId::UNKNOWN {
                let byte = input[i];
                next = self
                    .cache
                    .transition_miss(self.memory, from, byte, class(byte), self.regex);
                table = &self.cache.transitions[..];
            }
            i += 1;
            if next.is_match() {
                self.current = next.current();
                self.ever_matched = true;
                return;
            }
            if PREFILTER && next.is_start() {
                cur = Tier1StateId::START;
                match finder(&input[i..]) {
                    Some(offset) => i += offset,
                    None => break,
                }
                continue;
            }
            // An ordinary state after a miss.
            cur = next;
        }
        self.current = cur;
    }

    #[inline]
    pub fn finish(self) -> bool {
        if self.ever_matched {
            return true;
        }
        self.cache.state(self.current).is_match_at_end
    }

    #[allow(dead_code)]
    pub fn ismatch(&self) -> bool {
        self.ever_matched
    }
}

impl fmt::Debug for DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("DfaMatcher");
        s.field("current", &self.cache.untagged(self.current));
        let state = self.cache.state(self.current);
        s.field("nfa_states", &state.nfa_states);
        s.field("is_match", &state.is_match);
        s.field("is_match_at_end", &state.is_match_at_end);
        s.field("context", &state.context);
        if !state.deferred_asserts.is_empty() {
            s.field("deferred_asserts", &state.deferred_asserts);
        }
        s.field("ever_matched", &self.ever_matched);
        s.finish()
    }
}

impl fmt::Display for DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.cache.state(self.current);
        write!(
            f,
            "DFA[T1] state={} nfa={{{}}} matched={}",
            self.cache.untagged(self.current).0,
            state
                .nfa_states
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(","),
            self.ever_matched,
        )?;
        if state.is_match {
            write!(f, " is_match")?;
        }
        if state.is_match_at_end {
            write!(f, " mae")?;
        }
        if !state.deferred_asserts.is_empty() {
            write!(f, " context=({})", state.context)?;
            write!(
                f,
                " deferred=[{}]",
                state
                    .deferred_asserts
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// More than 128 byte classes round the row width up to 256, the width
    /// of a table without classes.  The columns are still classes: the
    /// scanning loop and the tests' single lookup must agree on that, or
    /// one reads the slots the other filled as transitions on other bytes.
    #[test]
    fn test_padded_rows_keep_class_columns() {
        // Bytes whose class numbers differ from their values.
        let literal: Vec<u8> = (60..=200).collect();
        let pattern: String = literal.iter().map(|b| format!("\\x{b:02x}")).collect();
        let re = Regex::with_config(
            &format!("(?s-u){pattern}"),
            crate::RegexConfig {
                byte_class_min_states: 0,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(re.dfa_eligible);
        assert!((129..256).contains(&re.num_byte_classes));
        let mut memory = DfaMemory::default();
        let mut cache = Tier1DfaCache::new();
        cache.prepare(&mut memory, &re);
        assert_eq!(cache.stride, 256);
        // Fill the rows through the single lookup, then scan.
        let mut current = cache.inner.start_id;
        for &b in &literal {
            current = cache.transition(&mut memory, current, b, &re);
        }
        assert!(cache.inner.states[current.idx()].is_match);
        let mut m = DfaMatcher::new(&mut cache, &mut memory, &re);
        m.chunk(&literal);
        assert!(m.finish());
        // The byte whose value is the class of the second literal byte is
        // not that byte: it must not take its transition.  (The prefilter
        // hides the first byte's row.)
        let mut input = literal.clone();
        input[1] = re.byte_classes[literal[1] as usize];
        assert_ne!(input[1], literal[1]);
        let mut m = DfaMatcher::new(&mut cache, &mut memory, &re);
        m.chunk(&input);
        assert!(!m.finish());
    }

    /// Bug 64: emptying a full cache keeps the state the matcher stands
    /// on and drops everything else.
    #[test]
    fn test_evict_keeps_the_given_state_and_empties_the_table() {
        let re = Regex::new("a[ab]{3}c").unwrap();
        assert!(re.dfa_eligible);
        let mut memory = DfaMemory::default();
        let mut cache = Tier1DfaCache::new();
        cache.prepare(&mut memory, &re);
        let mut m = DfaMatcher::new(&mut cache, &mut memory, &re);
        m.chunk(b"abab");
        let current = m.current;
        let current = cache.untagged(current);
        assert_ne!(current, DfaStateId::DEAD);
        assert_ne!(current, cache.inner.start_id);
        let state = cache.inner.states[current.idx()].clone();
        let start = cache.inner.states[cache.inner.start_id.idx()].clone();
        assert!(cache.inner.states.len() > 2);

        let current = cache.evict(current);
        assert_eq!(cache.inner.states[current.idx()], state);
        assert_eq!(cache.inner.states[cache.inner.start_id.idx()], start);
        assert_eq!(cache.inner.states.len(), 2);
        assert_eq!(cache.transitions.len(), 2 * cache.stride);
        assert!(
            cache
                .transitions
                .iter()
                .all(|&t| t == Tier1StateId::UNKNOWN)
        );
        assert_eq!(cache.evictions(), 1);

        // The dead state has no entry to keep.
        assert_eq!(cache.evict(DfaStateId::DEAD), DfaStateId::DEAD);
        assert_eq!(cache.inner.states.len(), 1);
        assert_eq!(cache.transitions.len(), cache.stride);
    }
}
