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
// DFA cache (Tier 1)
// ---------------------------------------------------------------------------

/// Lazy DFA cache: append-only state table + flat transition table.
///
/// The transition table is a flat `Vec<DfaStateId>` indexed by
/// `state.0 * stride + byte_class`, where `stride` is the number
/// of byte equivalence classes ([`Regex::num_byte_classes`]).
/// Unpopulated slots contain `DfaStateId::UNPOPULATED`.  When the
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
    /// Flat transition table: `transitions[state.0 * stride + byte_class]`.
    /// Unpopulated slots contain `DfaStateId::UNPOPULATED`.
    transitions: Vec<DfaStateId>,
    stride: usize,
    /// Lazily allocated restart transitions from DEAD; ids expire on eviction.
    dead_transitions: Vec<DfaStateId>,
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
        self.inner.set_max_states(max_states);
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
                .extend(std::iter::repeat_n(DfaStateId::UNPOPULATED, stride));
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
                        .extend(std::iter::repeat_n(DfaStateId::UNPOPULATED, stride));
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

    /// Compute the DFA transition for `(from_state, byte)`, using byte-class
    /// compression (stride < 256).
    #[inline(always)]
    fn transition(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        if from == DfaStateId::DEAD {
            return self.transition_from_dead(memory, byte, regex);
        }
        let class = regex.byte_classes[byte as usize] as usize;
        let slot = from.idx() * self.stride + class;
        let cached = self.transitions[slot];
        if cached != DfaStateId::UNPOPULATED {
            return cached;
        }
        self.transition_miss(memory, from, byte, class, regex)
    }

    /// Compute the DFA transition for `(from_state, byte)`, using the
    /// identity mapping (stride=256, no byte-class indirection).
    #[inline(always)]
    fn transition_direct(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        if from == DfaStateId::DEAD {
            return self.transition_from_dead(memory, byte, regex);
        }
        let slot = from.idx() * 256 + byte as usize;
        let cached = self.transitions[slot];
        if cached != DfaStateId::UNPOPULATED {
            return cached;
        }
        self.transition_miss(memory, from, byte, byte as usize, regex)
    }

    /// Cache miss: compute the transition out of `from` on `byte` (whose
    /// column in the table is `class`) and store it.  Kept out of line so
    /// the lookup above stays small.
    #[cold]
    #[inline(never)]
    fn transition_miss(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        class: usize,
        regex: &Regex,
    ) -> DfaStateId {
        // `from` gets a new id if the cache has to be emptied.
        let from = self.make_room(from);
        let to = self.populate(memory, from, byte, regex);
        self.transitions[from.idx() * self.stride + class] = to;
        to
    }

    /// Transition from the DEAD state (re-seed from start).
    /// Separated to keep the hot path of `transition()` small.
    #[inline(never)]
    fn transition_from_dead(
        &mut self,
        memory: &mut DfaMemory,
        byte: u8,
        regex: &Regex,
    ) -> DfaStateId {
        let class = regex.byte_classes[byte as usize] as usize;
        if let Some(&cached) = self.dead_transitions.get(class)
            && cached != DfaStateId::UNPOPULATED
        {
            return cached;
        }
        self.make_room(DfaStateId::DEAD);
        let next = self.populate(memory, DfaStateId::DEAD, byte, regex);
        self.dead_transitions
            .resize(self.stride, DfaStateId::UNPOPULATED);
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
        self.clear(memory, regex.states.len(), regex.num_byte_classes);
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
    current: DfaStateId,
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
        DfaMatcher {
            current: cache.inner.start_id,
            ever_matched: cache.inner.start_is_match,
            cache,
            memory,
            regex,
            prefilter: regex.prefilter,
        }
    }

    #[inline(always)]
    pub fn step(&mut self, byte: u8) {
        if self.cache.stride == 256 {
            self.current =
                self.cache
                    .transition_direct(self.memory, self.current, byte, self.regex);
        } else {
            self.current = self
                .cache
                .transition(self.memory, self.current, byte, self.regex);
        }
        if self.current != DfaStateId::DEAD && self.cache.inner.states[self.current.idx()].is_match
        {
            self.ever_matched = true;
        }
    }

    #[inline(always)]
    pub fn chunk(&mut self, input: &[u8]) {
        if self.ever_matched {
            return;
        }
        let start_id = self.cache.inner.start_id;

        // Prefilter-aware scanning: when the DFA is in the start state and
        // a prefilter is available, use memchr to skip ahead to the next
        // candidate byte instead of stepping byte-by-byte.  This is
        // re-engaged every time the DFA returns to the start state, not
        // just once at chunk entry.
        crate::prefilter::with_prefilter!(self.prefilter, self.chunk_no_prefilter(input), |find| {
            self.chunk_prefilter(input, start_id, find)
        })
    }

    #[inline(always)]
    fn chunk_no_prefilter(&mut self, input: &[u8]) {
        if self.cache.stride == 256 {
            for &b in input {
                if self.ever_matched {
                    return;
                }
                self.current =
                    self.cache
                        .transition_direct(self.memory, self.current, b, self.regex);
                if self.current != DfaStateId::DEAD
                    && self.cache.inner.states[self.current.idx()].is_match
                {
                    self.ever_matched = true;
                }
            }
        } else {
            for &b in input {
                if self.ever_matched {
                    return;
                }
                self.current = self
                    .cache
                    .transition(self.memory, self.current, b, self.regex);
                if self.current != DfaStateId::DEAD
                    && self.cache.inner.states[self.current.idx()].is_match
                {
                    self.ever_matched = true;
                }
            }
        }
    }

    /// Prefilter-integrated scanning loop.
    ///
    /// When `current == start_id`, the DFA is in its start state where
    /// non-matching bytes loop back to start.  Instead of stepping through
    /// those bytes one by one, we use the `finder` (memchr) to skip ahead
    /// to the next candidate byte.  Once a candidate is found, we step
    /// through the DFA normally until it returns to the start state (or
    /// matches / reaches end of input).
    #[inline(always)]
    fn chunk_prefilter(
        &mut self,
        input: &[u8],
        start_id: DfaStateId,
        finder: impl Fn(&[u8]) -> Option<usize>,
    ) {
        let mut i = 0;
        while i < input.len() {
            if self.ever_matched {
                return;
            }
            // When at the start state, skip ahead with memchr.
            if self.current == start_id {
                if let Some(offset) = finder(&input[i..]) {
                    i += offset;
                } else {
                    return;
                }
            }
            if self.cache.stride == 256 {
                self.current =
                    self.cache
                        .transition_direct(self.memory, self.current, input[i], self.regex);
            } else {
                self.current =
                    self.cache
                        .transition(self.memory, self.current, input[i], self.regex);
            }
            if self.current != DfaStateId::DEAD
                && self.cache.inner.states[self.current.idx()].is_match
            {
                self.ever_matched = true;
            }
            i += 1;
        }
    }

    #[inline]
    pub fn finish(self) -> bool {
        if self.ever_matched {
            return true;
        }
        if self.current != DfaStateId::DEAD {
            let state = &self.cache.inner.states[self.current.idx()];
            if state.is_match_at_end {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn ismatch(&self) -> bool {
        self.ever_matched
    }
}

impl fmt::Debug for DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("DfaMatcher");
        s.field("current", &self.current);
        if self.current != DfaStateId::DEAD {
            let state = &self.cache.inner.states[self.current.idx()];
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
        if self.current == DfaStateId::DEAD {
            return write!(f, "DFA[T1] state=DEAD matched={}", self.ever_matched);
        }
        let state = &self.cache.inner.states[self.current.idx()];
        write!(
            f,
            "DFA[T1] state={} nfa={{{}}} matched={}",
            self.current.0,
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
                .all(|&t| t == DfaStateId::UNPOPULATED)
        );
        assert_eq!(cache.evictions(), 1);

        // The dead state has no entry to keep.
        assert_eq!(cache.evict(DfaStateId::DEAD), DfaStateId::DEAD);
        assert_eq!(cache.inner.states.len(), 1);
        assert_eq!(cache.transitions.len(), cache.stride);
    }
}
