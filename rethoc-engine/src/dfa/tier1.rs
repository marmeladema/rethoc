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
/// three special ones: a slot that is not computed yet, the dead state and
/// a match state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Tier1StateId(u32);

impl Tier1StateId {
    /// Tag of a state whose `is_match` flag is set; the offset is kept.
    const MATCH: u32 = 1 << 29;
    /// The dead state: no NFA state is live.
    const DEAD: Self = Self(1 << 30);
    /// A transition that is not computed yet.
    const UNKNOWN: Self = Self(1 << 31);
    /// Largest untagged value, and the mask of the offset bits.
    const MAX: u32 = Self::MATCH - 1;

    /// Whether any tag is set: unknown, dead or match.
    #[inline(always)]
    fn is_tagged(self) -> bool {
        self.0 > Self::MAX
    }

    #[inline(always)]
    fn is_match(self) -> bool {
        self.0 & Self::MATCH != 0
    }

    /// Offset of the state's row in the transition table.  Not for `DEAD`
    /// or `UNKNOWN`.
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
// Keep the transition vector and stride adjacent. On AArch64 this lets the
// measured compiler load the vector length and stride together; correctness
// does not depend on Vec's internal layout or on paired-load code generation.
#[repr(C)]
pub(crate) struct Tier1DfaCache {
    inner: DfaCache,
    /// Flat transition table: `transitions[row offset + byte_class]`.
    /// Unpopulated slots contain `Tier1StateId::UNKNOWN`.
    transitions: Vec<Tier1StateId>,
    /// Row width: a power of two, at least the number of byte classes.
    stride: usize,
    /// Lazily allocated restart transitions from DEAD; ids expire on eviction.
    dead_transitions: Vec<Tier1StateId>,
    /// Reusable post-consumption roots, used only on cache misses.
    targets: Vec<StateIdx>,
}

impl Tier1DfaCache {
    pub(crate) fn new() -> Self {
        Self {
            inner: DfaCache::new(),
            transitions: Vec::new(),
            stride: 256,
            dead_transitions: Vec::new(),
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
        if id == DfaStateId::DEAD {
            return Tier1StateId::DEAD;
        }
        let row = id.0 << self.stride.trailing_zeros();
        debug_assert!(row <= Tier1StateId::MAX);
        if self.inner.states[id.idx()].is_match {
            Tier1StateId(row | Tier1StateId::MATCH)
        } else {
            Tier1StateId(row)
        }
    }

    /// The plain id of the state behind a table entry.
    #[inline]
    fn untagged(&self, id: Tier1StateId) -> DfaStateId {
        if id == Tier1StateId::DEAD {
            DfaStateId::DEAD
        } else {
            DfaStateId((id.row() >> self.stride.trailing_zeros()) as u32)
        }
    }

    /// The state behind a table entry.  Not for `DEAD` or `UNKNOWN`.
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
        self.dead_transitions.clear();
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
    /// `from` is a state or `DEAD`; the result is never `UNKNOWN`.
    #[inline(always)]
    fn transition_tagged(
        &mut self,
        memory: &mut DfaMemory,
        from: Tier1StateId,
        byte: u8,
        regex: &Regex,
    ) -> Tier1StateId {
        if from == Tier1StateId::DEAD {
            return self.transition_from_dead(memory, byte, regex);
        }
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

    /// Transition from the DEAD state (re-seed from start).
    /// Separated to keep the scanning loop small.
    #[inline(never)]
    fn transition_from_dead(
        &mut self,
        memory: &mut DfaMemory,
        byte: u8,
        regex: &Regex,
    ) -> Tier1StateId {
        let class = regex.byte_classes[byte as usize] as usize;
        if let Some(&cached) = self.dead_transitions.get(class)
            && cached != Tier1StateId::UNKNOWN
        {
            return cached;
        }
        self.make_room(DfaStateId::DEAD);
        let next = self.populate(memory, DfaStateId::DEAD, byte, regex);
        let next = self.tagged(next);
        self.dead_transitions
            .resize(self.stride, Tier1StateId::UNKNOWN);
        self.dead_transitions[class] = next;
        next
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
    fn populate_inner<const ASSERTIONS: bool>(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        self.targets.clear();
        let mut resolved_match = false;
        if from != DfaStateId::DEAD {
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
        if memory.closure_result.is_empty()
            && memory.closure_deferred.is_empty()
            && !matched
            && !at_end
        {
            return DfaStateId::DEAD;
        }
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
        self.dead_transitions.clear();
        memory.clear(num_nfa_states);
        self.transitions.clear();
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
            .field("dead_transitions_len", &self.dead_transitions.len())
            .field(
                "dead_transitions_capacity",
                &self.dead_transitions.capacity(),
            )
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Tier 1 DFA matcher (counter-free)
// ---------------------------------------------------------------------------

/// Lazy DFA matcher for Tier 1 (counter-free) patterns.
pub struct DfaMatcher<'a> {
    cache: &'a mut Tier1DfaCache,
    memory: &'a mut DfaMemory,
    regex: &'a Regex,
    /// The current state or `DEAD`; carries the match tag once
    /// `ever_matched` is set by a transition.
    current: Tier1StateId,
    ever_matched: bool,
    prefilter: Prefilter,
}

/// Scan `$input` from `$self.current`, stopping at the first match state.
///
/// `$class` maps a byte to its column; the caller spells it out so that
/// the identity mapping costs nothing.  `$at_start` runs before each byte
/// with the position and the current state; it may move the position
/// forward or leave the loop.
///
/// The current state and the table live in locals for the whole scan; the
/// table is borrowed again after a miss, which can move or empty it.  A
/// lookup is the entry at the current row offset plus the column, and one
/// test of that entry tells an ordinary transition from a missing one, the
/// dead state and a match.
macro_rules! scan {
    ($self:ident, $input:ident, |$b:ident| $class:expr,
     |$i:ident, $cur:ident| $at_start:block) => {{
        let mut $cur = $self.current;
        let mut table = &$self.cache.transitions[..];
        let mut $i = 0;
        while $i < $input.len() {
            $at_start
            let $b = $input[$i];
            let mut next = if $cur.is_tagged() {
                // Only the dead state: a match ends the scan.
                let next = $self
                    .cache
                    .transition_from_dead($self.memory, $b, $self.regex);
                table = &$self.cache.transitions[..];
                next
            } else {
                table[$cur.0 as usize + $class]
            };
            if next.is_tagged() {
                if next == Tier1StateId::UNKNOWN {
                    next = $self.cache.transition_miss(
                        $self.memory,
                        $cur,
                        $b,
                        $class,
                        $self.regex,
                    );
                    table = &$self.cache.transitions[..];
                }
                if next.is_match() {
                    $self.current = next;
                    $self.ever_matched = true;
                    return;
                }
            }
            $cur = next;
            $i += 1;
        }
        $self.current = $cur;
    }};
}

impl<'a> DfaMatcher<'a> {
    #[inline]
    pub(crate) fn new(
        cache: &'a mut Tier1DfaCache,
        memory: &'a mut DfaMemory,
        regex: &'a Regex,
    ) -> Self {
        // The start state is state 0, so its row offset is 0; its tag comes
        // from the cached flag.  No table access here.
        debug_assert_eq!(cache.inner.start_id, DfaStateId::START);
        let start_is_match = cache.inner.start_is_match;
        let tag = if start_is_match {
            Tier1StateId::MATCH
        } else {
            0
        };
        DfaMatcher {
            current: Tier1StateId(tag),
            ever_matched: start_is_match,
            cache,
            memory,
            regex,
            prefilter: regex.prefilter,
        }
    }

    #[inline(always)]
    pub fn step(&mut self, byte: u8) {
        self.current = self
            .cache
            .transition_tagged(self.memory, self.current, byte, self.regex);
        if self.current.is_match() {
            self.ever_matched = true;
        }
    }

    #[inline(always)]
    pub fn chunk(&mut self, input: &[u8]) {
        if self.ever_matched {
            return;
        }
        // Prefilter-aware scanning: when the DFA is in the start state and
        // a prefilter is available, use memchr to skip ahead to the next
        // candidate byte instead of stepping byte-by-byte.  This is
        // re-engaged every time the DFA returns to the start state, not
        // just once at chunk entry.
        crate::prefilter::with_prefilter!(self.prefilter, self.chunk_no_prefilter(input), |find| {
            self.chunk_prefilter(input, find)
        })
    }

    #[inline(always)]
    fn chunk_no_prefilter(&mut self, input: &[u8]) {
        if self.cache.stride == 256 {
            scan!(self, input, |b| b as usize, |i, cur| {});
        } else {
            let classes = &self.regex.byte_classes;
            scan!(self, input, |b| classes[b as usize] as usize, |i, cur| {});
        }
    }

    /// Prefilter-integrated scanning loop.
    ///
    /// In the start state, bytes that cannot start a match loop back to
    /// it.  Instead of stepping through those bytes one by one, the
    /// `finder` (memchr) skips ahead to the next candidate byte.  From
    /// there the DFA steps normally until it returns to the start state
    /// (or matches / reaches end of input).
    #[inline(always)]
    fn chunk_prefilter(&mut self, input: &[u8], finder: impl Fn(&[u8]) -> Option<usize>) {
        // The start state is state 0, also after the cache is emptied.
        debug_assert_eq!(self.cache.inner.start_id, DfaStateId::START);
        let start = Tier1StateId(0);
        if self.cache.stride == 256 {
            scan!(self, input, |b| b as usize, |i, cur| {
                if cur == start {
                    match finder(&input[i..]) {
                        Some(offset) => i += offset,
                        None => {
                            self.current = cur;
                            return;
                        }
                    }
                }
            });
        } else {
            let classes = &self.regex.byte_classes;
            scan!(self, input, |b| classes[b as usize] as usize, |i, cur| {
                if cur == start {
                    match finder(&input[i..]) {
                        Some(offset) => i += offset,
                        None => {
                            self.current = cur;
                            return;
                        }
                    }
                }
            });
        }
    }

    #[inline]
    pub fn finish(self) -> bool {
        if self.ever_matched {
            return true;
        }
        self.current != Tier1StateId::DEAD && self.cache.state(self.current).is_match_at_end
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
        if self.current != Tier1StateId::DEAD {
            let state = self.cache.state(self.current);
            s.field("nfa_states", &state.nfa_states);
            s.field("is_match", &state.is_match);
            s.field("is_match_at_end", &state.is_match_at_end);
            s.field("context", &state.context);
            if !state.deferred_asserts.is_empty() {
                s.field("deferred_asserts", &state.deferred_asserts);
            }
        }
        s.field("ever_matched", &self.ever_matched);
        s.finish()
    }
}

impl fmt::Display for DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.current == Tier1StateId::DEAD {
            return write!(f, "DFA[T1] state=DEAD matched={}", self.ever_matched);
        }
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
