//! Lazy DFA (Tier 1, Tier 2, Tier 3, and Tier 4).
//!
//! **Tier 1** (counter-free patterns): standard lazy subset construction.
//! DFA-eligible patterns may include deferred assertions (`\b`, `\B`,
//! `EndLF`) whose evaluation needs the *next* byte.  These are parked
//! in the DFA state and resolved at the start of `populate()` when the
//! next input byte is known.
//!
//! Tier 1 resolves all supported assertion kinds with exact boundary context.
//! The counting tiers still exclude `StartCRLF` and `EndCRLF`.
//!
//! **Tier 2** (non-nested, fixed-length-body repetitions): Becchi-style
//! differential counters.  O(1) per byte for patterns where every counter
//! body has a fixed byte length.
//!
//! **Tier 3** (non-nested counted repetitions): DFA + conditional transitions.
//! Precomputes separate DFA successor states per counter condition instead
//! of using runtime counter programs.  Each counter tracked independently.
//!
//! **Tier 4** (nested counted repetitions): DFA + explicit counter contexts.
//! The DFA state (set of NFA consuming states) is separated from the
//! counter state (a set of `CounterCtx` values).  DFA transitions are
//! cached normally; each transition also stores a compiled *counter
//! program* that describes how to update counter values.  On a cache
//! hit the program is replayed against each active counter context.

mod assertions;
mod tier1;
pub(crate) mod tier2;
pub(crate) mod tier3;
mod tier4;

use indexmap::Equivalent;
use indexmap::IndexSet;

pub(crate) use assertions::DfaAssertions;
#[cfg(test)]
pub(crate) use tier1::TIER1_MAX_STATES;
pub(crate) use tier1::{DfaMatcher, Tier1DfaCache};
pub(crate) use tier2::{Tier2Analysis, Tier2DfaCache, Tier2DfaMatcher, compute_tier2_analysis};
pub(crate) use tier3::{
    Tier3Analysis, Tier3DfaCache, Tier3DfaMatcher, Tier3OriginKind, compute_tier3_analysis,
};
pub(crate) use tier4::{Tier4DfaCache, Tier4DfaMatcher};

use crate::{AssertEval, AssertKind, ByteMap, CounterIdx, Regex, State, StateIdx};

/// Default number of DFA states a cache holds before it is emptied (see
/// [`DfaCache::evict`]).  Bounds the flat transition table at
/// 2048 states × stride entries.
pub(crate) const DFA_MAX_STATES: usize = 2048;

/// Most states a matcher holds at once and asks an eviction to keep
/// (Tier 3: the current state, its no-break twin and the clean chain's).
const MAX_KEPT_STATES: usize = 3;

/// Smallest state cap a cache accepts: the start state, the
/// [`MAX_KEPT_STATES`] states a matcher can hold, and the four states one
/// input byte can add (two transitions with a break and a no-break
/// successor each).  Tests run at this cap, where the cache is emptied
/// every few bytes.
pub(crate) const DFA_MIN_STATES: usize = 1 + MAX_KEPT_STATES + 4;

// ---------------------------------------------------------------------------
// DFA state table (shared by all tiers)
// ---------------------------------------------------------------------------

/// Index into the DFA state table ([`DfaCache::states`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DfaStateId(u32);

impl DfaStateId {
    /// First state interned into a prepared cache; preserved by eviction.
    pub(super) const START: Self = Self(0);
    /// Sentinel: the "dead" state (no NFA states, no match possible).
    pub(crate) const DEAD: Self = Self(u32::MAX);
    /// Sentinel for unpopulated flat-table slots.
    pub(super) const UNPOPULATED: Self = Self(u32::MAX - 1);

    #[inline]
    pub(super) fn idx(self) -> usize {
        self.0 as usize
    }
}

/// Boundary facts kept only while deferred continuations need them.
/// Bits 0–2 are `ByteProps`; bit 3 distinguishes the initial boundary.
/// Tier 3 retains its word-only encoding (0 or 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct BoundaryContext(u8);

impl BoundaryContext {
    /// Full physical boundary, before masking by continuation requirements.
    fn new(previous: Option<u8>) -> Self {
        Self(previous.map_or(8, |b| crate::ByteProps::of(b).0))
    }

    /// Preserve Tier 3's word-only state keys.
    const fn legacy(word: bool) -> Self {
        Self(word as u8)
    }

    /// Retain only the facts the parked continuations can still read.
    fn masked(self, requirements: u8) -> Self {
        Self(self.0 & requirements)
    }

    /// Whether the retained context distinguishes input start.
    fn at_start(self) -> bool {
        self.0 & 8 != 0
    }

    /// Representative for every fact the pending continuation can read.
    fn previous(self) -> Option<u8> {
        (!self.at_start()).then(|| crate::ByteProps(self.0).stand_in())
    }
}

impl std::fmt::Display for BoundaryContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "prev={:?} start={}", self.previous(), self.at_start())
    }
}

/// A DFA state: a sorted, deduplicated set of NFA consuming/assert state
/// indices, plus flags derived from the epsilon closure.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct DfaState {
    /// Sorted NFA state indices (consuming states only: Byte, ByteCI,
    /// ByteClass, ByteTable).  Assert states are resolved during closure
    /// computation;
    /// Match states set the `is_match` / `is_match_at_end` flags.
    pub(super) nfa_states: Box<[StateIdx]>,
    /// NFA Assert state indices whose evaluation was deferred because the
    /// next byte was unknown. Resolved when that byte arrives; Tier 1
    /// includes CRLF assertions as well as word boundaries and `EndLF`.
    pub(super) deferred_asserts: Box<[StateIdx]>,
    /// True if `Match` is directly reachable (no pending `$` gate).
    pub(super) is_match: bool,
    /// Acceptance at EOF. Tiers 1 and 2 evaluate assertions in the physical
    /// boundary context. Tier 3 uses reachability.
    pub(super) is_match_at_end: bool,
    /// Context required by parked continuations, zero when none are pending.
    /// Tiers 1 and 2 mask to the required facts. Tier 3 keeps wordness.
    pub(super) context: BoundaryContext,
}

/// The byte a DFA state substitutes for the one that entered it, of which
/// it only remembers the word-ness.
#[inline]
pub(super) const fn prev_stand_in(prev_was_word: bool) -> u8 {
    if prev_was_word { b'a' } else { b' ' }
}

impl DfaState {
    /// Tier 3 remembers only the previous byte's wordness. This suffices
    /// for a parked word assertion itself. A continuation behind it can
    /// need other boundary facts (Bug 74).
    #[inline]
    pub(super) fn prev_byte_representative(&self) -> Option<u8> {
        debug_assert!(self.context.0 <= 1, "legacy counting-tier context only");
        Some(prev_stand_in(self.context.0 != 0))
    }

    /// Resolve deferred assertions from `from_state` given that `byte` is the
    /// next input byte.  Returns extra NFA targets (the `out` states of
    /// passing assertions) that should be followed through epsilon closure.
    ///
    /// All assertions in the chain see the boundary that parked them.
    #[inline]
    fn resolve_deferred(&self, byte: u8, regex: &Regex) -> Vec<StateIdx> {
        let mut extra = Vec::new();
        if self.deferred_asserts.is_empty() {
            return extra;
        }
        let prev = self.context.previous();
        for &assert_idx in self.deferred_asserts.iter() {
            if let State::Assert { kind, out } = regex.states[assert_idx]
                && kind.eval(self.context.at_start(), false, prev, Some(byte)) == AssertEval::Pass
            {
                extra.push(out);
            }
        }
        extra
    }

    /// Resolve deferred assertions at end-of-input.  Returns true if any
    /// deferred assertion passes and `Match` is reachable from its `out`.
    #[inline]
    fn resolve_deferred_at_end(&self, regex: &Regex, scratch: &mut ReachScratch) -> bool {
        if self.deferred_asserts.is_empty() {
            return false;
        }
        let prev = self.context.previous();
        for &assert_idx in self.deferred_asserts.iter() {
            if let State::Assert { kind, out } = regex.states[assert_idx]
                && kind.eval(self.context.at_start(), true, prev, None) == AssertEval::Pass
                && Self::can_reach_match_at_end(out, self.context.at_start(), prev, regex, scratch)
            {
                return true;
            }
        }
        false
    }

    /// Follow epsilon transitions from `start` at end-of-input, evaluating
    /// any assertions encountered.  Returns true if `Match` is reachable
    /// with all assertions passing.
    ///
    /// Unlike the static `state_can_reach_match` precomputation, this
    /// correctly handles adjacent assertions like `\b\B` which are
    /// statically reachable but dynamically impossible at the same position.
    pub(super) fn can_reach_match_at_end(
        start: StateIdx,
        at_start: bool,
        prev: Option<u8>,
        regex: &Regex,
        scratch: &mut ReachScratch,
    ) -> bool {
        let states = &regex.states;
        let num_states = states.len();
        scratch.prepare(num_states, start);
        while let Some(idx) = scratch.stack.pop() {
            let i = idx.idx();
            if i >= num_states || scratch.is_visited(i) || !regex.state_can_reach_match[i] {
                continue;
            }
            scratch.mark_visited(i);
            match states[idx] {
                State::Match => return true,
                State::Assert { kind, out } => {
                    if kind.eval(at_start, true, prev, None) == AssertEval::Pass {
                        scratch.stack.push(out);
                    }
                }
                State::Split { out, out1 } => {
                    scratch.stack.push(out);
                    scratch.stack.push(out1);
                }
                State::CounterInstance { out, .. } => {
                    scratch.stack.push(out);
                }
                _ => {}
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Reusable scratch for epsilon-walk reachability checks
// ---------------------------------------------------------------------------

/// Reusable scratch buffers for [`DfaState::can_reach_match_at_end`] and
/// [`Tier3DfaMatcher::can_reach_match_mid`].
///
/// Both functions walk epsilon transitions from a start state, checking
/// whether `Match` is reachable with assertions passing.  They need a
/// per-NFA-state visited set and a work stack.  This struct lets callers
/// allocate once and reuse across calls, avoiding per-call `vec![false; n]`
/// allocations on the hot path.
pub(super) struct ReachScratch {
    /// Epoch-stamped visited array.  `visited[i] == generation` means
    /// state `i` has been visited in the current query.
    visited: Vec<u32>,
    /// Current generation counter.
    generation: u32,
    stack: Vec<StateIdx>,
}

impl ReachScratch {
    /// Create empty scratch (no allocation until first `prepare()`).
    pub(super) fn new() -> Self {
        Self {
            visited: Vec::new(),
            generation: 0,
            stack: Vec::new(),
        }
    }

    /// Reset for a new reachability query.  Bumps the generation counter
    /// instead of clearing the visited array — O(1) instead of O(num_states).
    /// On the first call (or after the array is too small), the array is
    /// grown to `num_states` entries.
    #[inline]
    fn prepare(&mut self, num_states: usize, start: StateIdx) {
        self.generation = self.generation.wrapping_add(1);
        if self.visited.len() < num_states {
            self.visited.resize(num_states, 0);
        }
        self.stack.clear();
        self.stack.push(start);
    }

    /// Check if a state has been visited in the current generation.
    #[inline]
    fn is_visited(&self, i: usize) -> bool {
        self.visited[i] == self.generation
    }

    /// Mark a state as visited in the current generation.
    #[inline]
    fn mark_visited(&mut self, i: usize) {
        self.visited[i] = self.generation;
    }
}

/// Borrowed view of [`DfaState`] for zero-allocation [`IndexSet`] lookups.
///
/// `DfaStateRef` mirrors `DfaState` but holds `&[StateIdx]` slices instead
/// of `Box<[StateIdx]>`.  Because `Box<[T]>` and `&[T]` both delegate to
/// `[T]::hash()`, the derived `Hash` impls produce identical hashes —
/// making this safe to use with [`Equivalent`] for probing.
#[derive(Hash)]
pub(super) struct DfaStateRef<'a> {
    nfa_states: &'a [StateIdx],
    deferred_asserts: &'a [StateIdx],
    is_match: bool,
    is_match_at_end: bool,
    context: BoundaryContext,
}

impl Equivalent<DfaState> for DfaStateRef<'_> {
    fn equivalent(&self, key: &DfaState) -> bool {
        self.nfa_states == &*key.nfa_states
            && self.deferred_asserts == &*key.deferred_asserts
            && self.is_match == key.is_match
            && self.is_match_at_end == key.is_match_at_end
            && self.context == key.context
    }
}

/// Scratch space for epsilon-closure computation, shared across DFA
/// tiers 1–3.
///
/// All fields are temporary buffers that are cleared at the start of each
/// [`epsilon_closure`](Self::epsilon_closure) call.  The struct carries no
/// semantically meaningful state between calls — it exists purely to avoid
/// repeated heap allocation.
#[derive(Debug, Default)]
pub(crate) struct DfaMemory {
    // Scratch space for closure.
    pub(super) closure_stack: Vec<StateIdx>,
    pub(super) closure_result: Vec<StateIdx>,
    pub(super) closure_deferred: Vec<StateIdx>,
    pub(super) closure_visited: Vec<bool>,
    pub(super) closure_seeds: Vec<(CounterIdx, StateIdx)>,
}

impl DfaMemory {
    /// Reset the scratch buffers for reuse with a new regex.
    #[inline]
    pub(super) fn clear(&mut self, num_nfa_states: usize) {
        self.closure_deferred.clear();
        self.closure_visited.clear();
        self.closure_visited.resize(num_nfa_states, false);
        self.closure_seeds.clear();
    }

    /// Compute the epsilon closure from a set of NFA seed states.
    ///
    /// Follows `Split` and evaluable assertions (`Start`, `StartLF`).
    /// `End` (`$`) is handled specially: when not at end-of-input it sets
    /// `is_match_at_end` instead of following `out`.  Assertions that may
    /// need the next byte (`WordAscii`, `WordAsciiNegate`, `EndLF`) can
    /// return `Defer`, in which case the Assert state index is collected
    /// in `closure_deferred`.
    ///
    /// With `EXACT_EOF`, End and deferred assertions contribute their exact
    /// EOF answers at the physical boundary. Each closure includes only the
    /// counter break paths selected by its caller. Tier 3 uses optimistic
    /// EOF flags in its separate counter program.
    ///
    /// Returns `(is_match, is_match_at_end)`.  Consuming states are left
    /// in `self.closure_result`; deferred asserts in `self.closure_deferred`.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn epsilon_closure<const EXACT_EOF: bool>(
        &mut self,
        seeds: impl Iterator<Item = StateIdx>,
        regex: &Regex,
        at_start: bool,
        at_end: bool,
        prev_byte: Option<u8>,
        next_byte: Option<u8>,
        mut counter_instance: impl FnMut(&mut Self, CounterIdx, StateIdx),
        mut counter_increment: impl FnMut(&mut Self, CounterIdx, StateIdx, StateIdx, usize, usize),
    ) -> (bool, bool) {
        self.closure_stack.clear();
        self.closure_result.clear();
        self.closure_deferred.clear();
        self.closure_seeds.clear();

        for v in self.closure_visited.iter_mut() {
            *v = false;
        }

        let mut is_match = false;
        let mut is_match_at_end = false;

        self.closure_stack.extend(seeds);
        while let Some(idx) = self.closure_stack.pop() {
            let i = idx.idx();
            if self.closure_visited[i] {
                continue;
            }
            self.closure_visited[i] = true;

            match regex.states[idx] {
                State::Split { out, out1 } => {
                    self.closure_stack.push(out1);
                    self.closure_stack.push(out);
                }
                State::Assert { kind, out } => {
                    // Special handling for $ (End): it fails when not at
                    // end-of-input, but we still need to record that a
                    // match is possible if we reach end-of-input later.
                    if kind == AssertKind::End {
                        if at_end {
                            self.closure_stack.push(out);
                        } else if EXACT_EOF {
                            is_match_at_end |= next_byte.is_none()
                                && regex
                                    .dfa_assertions
                                    .eof_accept(idx, BoundaryContext::new(prev_byte).0);
                        } else if regex.state_can_reach_match[out.idx()] {
                            is_match_at_end = true;
                        }
                        continue;
                    }
                    let result = kind.eval(at_start, at_end, prev_byte, next_byte);
                    match result {
                        AssertEval::Pass => {
                            self.closure_stack.push(out);
                        }
                        AssertEval::Fail => {
                            // Assertion failed — skip.
                        }
                        AssertEval::Defer => {
                            if EXACT_EOF {
                                is_match_at_end |= regex
                                    .dfa_assertions
                                    .eof_accept(idx, BoundaryContext::new(prev_byte).0);
                            }
                            // Park this assert for deferred resolution.
                            self.closure_deferred.push(idx);
                        }
                    }
                }
                State::Match => {
                    is_match = true;
                }
                State::Byte { .. }
                | State::ByteCI { .. }
                | State::Wildcard { .. }
                | State::ByteClassStatic { .. }
                | State::ByteClassCustom { .. }
                | State::ByteTable { .. } => {
                    self.closure_result.push(idx);
                }
                State::CounterInstance { counter, out } => {
                    counter_instance(self, counter, out);
                }
                State::CounterIncrement {
                    counter,
                    out,
                    out1,
                    min,
                    max,
                } => {
                    counter_increment(self, counter, out, out1, min, max);
                }
            }
        }

        self.closure_result.sort_unstable();
        self.closure_deferred.sort_unstable();

        (is_match, is_match_at_end)
    }
}

// ---------------------------------------------------------------------------
// Counter-body membership and idle DFA states
// ---------------------------------------------------------------------------

/// Visit every NFA state of a counter body, starting from `start`
/// (a `CounterInstance.out`).
///
/// The walk follows epsilon and consuming edges alike and stops at the
/// first `CounterIncrement` on each path — for a pattern without nested
/// counters that is always the body's own, so neither the continue path
/// (back into the body) nor the break path (out of it) is followed.  The
/// `CounterIncrement` itself is not reported.
///
/// `visited` must be `states.len()` long.  States already marked are
/// skipped, so one array can be shared across the bodies of several
/// counters to keep the total cost linear in the NFA size.
fn walk_counter_body(
    states: &[State],
    byte_tables: &[ByteMap],
    start: StateIdx,
    visited: &mut [bool],
    mut visit: impl FnMut(StateIdx, &State),
) {
    let mut stack = vec![start];
    while let Some(idx) = stack.pop() {
        let i = idx.idx();
        if i >= states.len() || visited[i] {
            continue;
        }
        visited[i] = true;
        let state = &states[i];
        match *state {
            State::CounterIncrement { .. } => continue,
            State::Match => {}
            State::CounterInstance { out, .. } | State::Assert { out, .. } => stack.push(out),
            State::Split { out, out1 } => {
                stack.push(out);
                stack.push(out1);
            }
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
                for &succ in &byte_tables[table.idx()].0 {
                    if succ != StateIdx::NONE {
                        stack.push(succ);
                    }
                }
            }
        }
        visit(idx, state);
    }
}

/// True for NFA states that consume an input byte.
fn is_consuming(state: &State) -> bool {
    matches!(
        state,
        State::Byte { .. }
            | State::ByteCI { .. }
            | State::Wildcard { .. }
            | State::ByteClassStatic { .. }
            | State::ByteClassCustom { .. }
            | State::ByteTable { .. }
    )
}

/// Mark every NFA state on which a thread can rest inside a counter body.
///
/// `result[i]` is `true` iff NFA state `i` is a consuming state or an
/// assertion on a path from some `CounterInstance.out` to the matching
/// `CounterIncrement`.  A thread can only sit on such a state while it
/// owns a counter instance, so when the counter has no live instance a
/// consuming body state is *dead* even if the cached DFA state still lists
/// it.  Assertions are marked so that callers can tell whether a deferred
/// assertion belongs to a body.
///
/// Only meaningful for patterns without nested counters (see
/// [`walk_counter_body`]).
pub(crate) fn compute_counter_body_states(
    states: &[State],
    byte_tables: &[ByteMap],
) -> Box<[bool]> {
    let mut body = vec![false; states.len()];
    let mut visited = vec![false; states.len()];
    for s in states {
        if let State::CounterInstance { out, .. } = *s {
            walk_counter_body(states, byte_tables, out, &mut visited, |idx, state| {
                if is_consuming(state) || matches!(state, State::Assert { .. }) {
                    body[idx.idx()] = true;
                }
            });
        }
    }
    body.into_boxed_slice()
}

/// Lazily computed answer to "is this DFA state idle?" — see
/// [`IdleStates::is_idle`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Idle {
    /// Not classified yet.
    Unknown,
    No,
    Yes,
}

/// Per-DFA-state cache of the *idle* classification, owned by the Tier 3
/// cache alongside its [`DfaCache`].
///
/// Tier 3 only needs to know whether a state is equivalent to the start
/// state, because its DFA state over-approximates the live threads and
/// cannot be rewritten in general.  Tier 2, whose DFA state is exact,
/// instead drops dead body states from any state (see
/// `Tier2DfaCache::settled_state`).
///
/// Kept out of [`DfaCache`] on purpose: Tier 1 has no counters and must
/// not pay for the table in its state-interning path or its cache layout.
struct IdleStates {
    /// Indexed by [`DfaStateId`]; grown on demand, so states interned
    /// since the last query read as [`Idle::Unknown`].
    class: Vec<Idle>,
}

impl IdleStates {
    fn new() -> Self {
        Self { class: Vec::new() }
    }

    /// Forget every classification (the DFA cache is being rebuilt).
    #[inline]
    fn clear(&mut self) {
        self.class.clear();
    }

    /// Returns `true` if DFA state `id` of `cache` is *idle*: it differs
    /// from the start state only by NFA states that sit inside counter
    /// bodies.
    ///
    /// The counting tiers keep a counter's body states in the DFA state for
    /// as long as the input keeps matching the body, whether or not any
    /// counter instance is still alive.  Once the caller knows that no
    /// counter instance (and no other side-channel thread) is live, those
    /// body states carry no NFA thread, so an idle state is equivalent to
    /// the start state and the matcher may return to it — which is what
    /// lets the prefilter take over again.
    ///
    /// Precisely, the state is idle when all of the following hold:
    ///
    /// - The start closure is free of counters and assertions
    ///   ([`Regex::start_closure`] is non-empty), so the start state is the
    ///   same at every input position and owns no counter instance.
    /// - Neither state is a match, a match-at-end, or has deferred
    ///   assertions (a deferred assertion is a parked thread).
    /// - Dropping the counter-body states from `id` leaves exactly the
    ///   start state's NFA set.
    ///
    /// The start state itself is not reported as idle: there is nothing to
    /// return to.  The answer is cached per state.
    #[inline]
    fn is_idle(&mut self, cache: &DfaCache, id: DfaStateId, regex: &Regex) -> bool {
        match self.class.get(id.idx()) {
            Some(Idle::Yes) => true,
            Some(Idle::No) => false,
            Some(Idle::Unknown) | None => self.classify(cache, id, regex),
        }
    }

    #[cold]
    fn classify(&mut self, cache: &DfaCache, id: DfaStateId, regex: &Regex) -> bool {
        if self.class.len() < cache.states.len() {
            self.class.resize(cache.states.len(), Idle::Unknown);
        }
        let idle = Self::compute(cache, id, regex);
        self.class[id.idx()] = if idle { Idle::Yes } else { Idle::No };
        idle
    }

    fn compute(cache: &DfaCache, id: DfaStateId, regex: &Regex) -> bool {
        if id == cache.start_id || cache.start_id == DfaStateId::DEAD {
            return false;
        }
        let body = &regex.counter_body_states;
        if regex.start_closure.is_empty() || body.is_empty() {
            return false;
        }
        let start = &cache.states[cache.start_id.idx()];
        let state = &cache.states[id.idx()];
        let plain =
            |s: &DfaState| !s.is_match && !s.is_match_at_end && s.deferred_asserts.is_empty();
        plain(start)
            && plain(state)
            && state
                .nfa_states
                .iter()
                .filter(|s| !body[s.idx()])
                .eq(start.nfa_states.iter())
    }
}

/// The DFA state table shared by Tiers 1–3.
///
/// This is a **cache** of a function that can always be recomputed: which
/// DFA state follows from a state and a byte.  Entries may be dropped at
/// any time without changing an answer.  When the table is full it is
/// emptied ([`evict`](Self::evict)) and refilled on demand; a matcher
/// never trades a state it cannot store for another one.
///
/// The protocol, for every tier: before computing a missing transition,
/// the matcher checks [`has_room`](Self::has_room) for the states that
/// computation can add, and evicts if there is none.  Interning then
/// cannot fail.
struct DfaCache {
    states: IndexSet<DfaState, ahash::RandomState>,
    regex_id: u64,
    start_id: DfaStateId,
    start_is_match: bool,
    start_is_match_at_end: bool,
    /// State cap; at least [`DFA_MIN_STATES`].
    max_states: usize,
    /// Number of times the table was emptied because it was full.
    evictions: u64,
}

impl DfaCache {
    fn new() -> Self {
        Self {
            states: IndexSet::default(),
            regex_id: 0,
            start_id: DfaStateId::DEAD,
            start_is_match: false,
            start_is_match_at_end: false,
            max_states: DFA_MAX_STATES,
            evictions: 0,
        }
    }

    /// Set the state cap (raised to [`DFA_MIN_STATES`] if lower).  Takes
    /// effect at the next missing transition; never changes an answer.
    fn set_max_states(&mut self, max_states: usize) {
        self.max_states = max_states.max(DFA_MIN_STATES);
    }

    /// Whether `n` more states fit under the cap.
    #[inline]
    fn has_room(&self, n: usize) -> bool {
        self.states.len() + n <= self.max_states
    }

    /// Empty the state table, keeping only the start state and the states
    /// in `keep` (at most [`MAX_KEPT_STATES`]), whose ids are rewritten in
    /// place ([`DfaStateId::DEAD`] entries are left alone).  `new_row` is
    /// called once per state that is put back, like the callback of
    /// [`intern_state`](Self::intern_state); the caller must have cleared
    /// every table indexed by state id beforehand.
    ///
    /// **The start state keeps its id.**  It is the first state interned
    /// into an empty table, here as in `prepare`, so it is always state 0.
    /// Matchers rely on this: they copy `start_id` once per chunk and
    /// compare against it while the cache may be emptied under them.
    ///
    /// Every other [`DfaStateId`] is invalid afterwards.
    ///
    /// The kept states are moved, not copied, and the table keeps its
    /// capacity: at a small cap this runs every few bytes.
    #[cold]
    fn evict(&mut self, keep: &mut [DfaStateId], mut new_row: impl FnMut()) {
        debug_assert!(self.start_id != DfaStateId::DEAD, "cache not prepared");
        debug_assert_eq!(
            self.start_id,
            DfaStateId::START,
            "the start state is state 0"
        );
        debug_assert!(keep.len() <= MAX_KEPT_STATES);
        let start_idx = self.start_id.idx();
        let mut old = [DfaStateId::DEAD; MAX_KEPT_STATES];
        old[..keep.len()].copy_from_slice(keep);

        // Take the wanted states out of the table while it is drained.
        let mut start = None;
        let mut kept: [Option<DfaState>; MAX_KEPT_STATES] = [const { None }; MAX_KEPT_STATES];
        for (idx, state) in self.states.drain(..).enumerate() {
            if idx == start_idx {
                start = Some(state);
            } else if let Some(k) = old
                .iter()
                .position(|id| *id != DfaStateId::DEAD && id.idx() == idx)
            {
                kept[k] = Some(state);
            }
        }
        self.evictions += 1;

        let start = start.expect("the start state is in the table");
        let (idx, _) = self.states.insert_full(start);
        new_row();
        debug_assert_eq!(idx, start_idx, "the start state keeps its id");
        self.start_id = DfaStateId(idx as u32);
        for k in 0..keep.len() {
            if old[k] == DfaStateId::DEAD {
                continue;
            }
            keep[k] = if old[k].idx() == start_idx {
                self.start_id
            } else if let Some(first) = old[..k].iter().position(|id| *id == old[k]) {
                // The same state asked for twice.
                keep[first]
            } else {
                let state = kept[k].take().expect("a kept state is in the table");
                let (idx, _) = self.states.insert_full(state);
                new_row();
                DfaStateId(idx as u32)
            };
        }
    }

    /// Look up or insert a DFA state for the given sorted NFA state set.
    /// Returns `None` if the state cap has been reached and the state is
    /// not already interned.  Matchers make room before computing a
    /// transition, so for them `None` cannot happen; it is only a normal
    /// outcome for optional extra states (Tier 2's settled states).
    ///
    /// Takes borrowed slices so that cache-hit probes (the common case)
    /// are zero-allocation.  A heap-allocated [`DfaState`] is only created
    /// on a cache miss when actual insertion is needed.
    #[inline]
    fn intern_state(
        &mut self,
        nfa_states: &[StateIdx],
        deferred_asserts: &[StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        prev_was_word: bool,
        transition: impl FnMut(),
    ) -> Option<DfaStateId> {
        self.intern_context(
            nfa_states,
            deferred_asserts,
            is_match,
            is_match_at_end,
            BoundaryContext::legacy(prev_was_word),
            transition,
        )
    }

    /// Intern a state with an already canonicalized assertion context.
    #[inline]
    fn intern_context(
        &mut self,
        nfa_states: &[StateIdx],
        deferred_asserts: &[StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        context: BoundaryContext,
        mut transition: impl FnMut(),
    ) -> Option<DfaStateId> {
        let probe = DfaStateRef {
            nfa_states,
            deferred_asserts,
            is_match,
            is_match_at_end,
            context,
        };
        if let Some(idx) = self.states.get_index_of(&probe) {
            return Some(DfaStateId(idx as u32));
        }
        if self.states.len() >= self.max_states {
            return None;
        }
        let state = DfaState {
            nfa_states: nfa_states.into(),
            deferred_asserts: deferred_asserts.into(),
            is_match,
            is_match_at_end,
            context,
        };
        let (idx, _) = self.states.insert_full(state);
        transition();
        Some(DfaStateId(idx as u32))
    }

    /// Reset the cache for reuse with a new regex.
    #[inline]
    fn clear(&mut self) {
        self.states.clear();
        self.regex_id = 0;
        self.start_id = DfaStateId::DEAD;
        self.start_is_match = false;
        self.start_is_match_at_end = false;
        self.evictions = 0;
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash, Hasher};

    use super::*;

    /// Compute the 64-bit hash of a value using [`DefaultHasher`].
    fn hash_of(val: &impl Hash) -> u64 {
        let mut h = DefaultHasher::new();
        val.hash(&mut h);
        h.finish()
    }

    /// Build a [`DfaState`] from the given fields (shorthand for tests).
    fn make_state(
        nfa: &[u32],
        deferred: &[u32],
        is_match: bool,
        is_match_at_end: bool,
        prev_was_word: bool,
    ) -> DfaState {
        DfaState {
            nfa_states: nfa.iter().map(|&v| StateIdx(v)).collect(),
            deferred_asserts: deferred.iter().map(|&v| StateIdx(v)).collect(),
            is_match,
            is_match_at_end,
            context: BoundaryContext::legacy(prev_was_word),
        }
    }

    /// Build a [`DfaStateRef`] from the given slices and flags.
    fn make_ref<'a>(
        nfa: &'a [StateIdx],
        deferred: &'a [StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        prev_was_word: bool,
    ) -> DfaStateRef<'a> {
        DfaStateRef {
            nfa_states: nfa,
            deferred_asserts: deferred,
            is_match,
            is_match_at_end,
            context: BoundaryContext::legacy(prev_was_word),
        }
    }

    // -----------------------------------------------------------------------
    // Counter-body membership and idle states
    // -----------------------------------------------------------------------

    /// Compile with unrolling disabled so that small repetitions keep
    /// their counters.
    fn build_counting(pattern: &str) -> Regex {
        Regex::with_config(
            pattern,
            crate::RegexConfig {
                max_unroll_states: 0,
                ..Default::default()
            },
        )
        .expect("pattern should compile")
    }

    /// NFA indices of the consuming states that accept `byte`.
    fn states_accepting(re: &Regex, byte: u8) -> Vec<u32> {
        (0..re.states.len() as u32)
            .filter(|&i| match re.states[StateIdx(i)] {
                State::Byte { byte: b, .. } => b == byte,
                State::ByteCI { byte: b, .. } => b == byte || b ^ 0x20 == byte,
                State::Wildcard { .. } => true,
                State::ByteClassStatic { table, .. } => table[byte as usize],
                State::ByteClassCustom { class, .. } => re.classes[class.idx()].contains(byte),
                State::ByteTable { table } => {
                    re.byte_tables[table.idx()].0[byte as usize] != StateIdx::NONE
                }
                _ => false,
            })
            .collect()
    }

    /// Indices marked as counter-body states.
    fn body_indices(re: &Regex) -> Vec<u32> {
        (0..re.counter_body_states.len() as u32)
            .filter(|&i| re.counter_body_states[i as usize])
            .collect()
    }

    #[test]
    fn test_counter_body_states_single_byte_body() {
        // Only the `.` inside the repetition is a body state: the anchors
        // on either side are not.
        let re = build_counting("foo.{0,5}bar");
        assert_eq!(re.counter_body_states.len(), re.states.len());
        let body = body_indices(&re);
        assert_eq!(body.len(), 1, "exactly one body state: {body:?}");
        assert!(matches!(
            re.states[StateIdx(body[0])],
            State::Wildcard { .. }
        ));
        for b in *b"fobar" {
            for i in states_accepting(&re, b) {
                assert_eq!(
                    re.counter_body_states[i as usize],
                    body.contains(&i),
                    "state {i} accepting {:?}",
                    b as char
                );
            }
        }
    }

    #[test]
    fn test_counter_body_states_multi_byte_and_alternation_body() {
        // Every consuming state of the body is marked, including interior
        // positions and both alternation branches; `x` and `y` are not.
        let re = build_counting("x(ab|cd){2,9}y");
        let body = body_indices(&re);
        for b in *b"abcd" {
            let accepting = states_accepting(&re, b);
            assert!(!accepting.is_empty());
            for i in accepting {
                assert!(
                    body.contains(&i),
                    "state {i} ({:?}) is in the body",
                    b as char
                );
            }
        }
        for b in *b"xy" {
            for i in states_accepting(&re, b) {
                assert!(!body.contains(&i), "state {i} ({:?}) is outside", b as char);
            }
        }
    }

    #[test]
    fn test_counter_body_states_two_counters() {
        // Both bodies are marked; the literal between them is not.
        let re = build_counting("a[0-9]{2,7}b[x-z]{3,8}c");
        let body = body_indices(&re);
        assert_eq!(body.len(), 2, "one body state per counter: {body:?}");
        for b in *b"5y" {
            for i in states_accepting(&re, b) {
                assert!(body.contains(&i));
            }
        }
        for b in *b"abc" {
            for i in states_accepting(&re, b) {
                assert!(!body.contains(&i));
            }
        }
    }

    #[test]
    fn test_counter_body_states_marks_assertions_in_bodies() {
        // An assertion between the body byte and the increment is part of
        // the body: a thread can be parked on it.  The leading `\b` is
        // outside.
        let re = build_counting("\\bx(a\\b){2,6}y");
        let asserts: Vec<u32> = (0..re.states.len() as u32)
            .filter(|&i| matches!(re.states[StateIdx(i)], State::Assert { .. }))
            .collect();
        assert_eq!(asserts.len(), 2, "two word-boundary assertions");
        let marked: Vec<bool> = asserts
            .iter()
            .map(|&i| re.counter_body_states[i as usize])
            .collect();
        assert_eq!(marked.iter().filter(|&&m| m).count(), 1);
        // The body byte is marked, the anchors are not.
        for i in states_accepting(&re, b'a') {
            assert!(re.counter_body_states[i as usize]);
        }
        for b in *b"xy" {
            for i in states_accepting(&re, b) {
                assert!(!re.counter_body_states[i as usize]);
            }
        }
    }

    #[test]
    fn test_counter_body_states_empty_without_flat_counters() {
        // No counters at all.
        assert!(build_counting("foobar").counter_body_states.is_empty());
        // Nested counters: the table is not computed (tiers 2 and 3 are
        // not eligible, and the walk assumes flat bodies).
        let nested = build_counting("(a{2,5}b){3,7}c");
        assert!(!nested.tier2_eligible && !nested.tier3_eligible);
        assert!(nested.counter_body_states.is_empty());
    }

    /// Build a [`DfaCache`] whose start state is `re`'s start closure,
    /// with an empty idle table.
    fn cache_with_start(re: &Regex) -> (DfaCache, IdleStates, Vec<StateIdx>) {
        let mut start: Vec<StateIdx> = re.start_closure.to_vec();
        start.sort_unstable();
        let mut cache = DfaCache::new();
        cache.start_id = cache
            .intern_state(&start, &[], false, false, false, || {})
            .expect("start state fits");
        (cache, IdleStates::new(), start)
    }

    /// `base` plus `extra`, sorted (the DFA state key order).
    fn with_states(base: &[StateIdx], extra: &[u32]) -> Vec<StateIdx> {
        let mut v = base.to_vec();
        v.extend(extra.iter().map(|&i| StateIdx(i)));
        v.sort_unstable();
        v.dedup();
        v
    }

    #[test]
    fn test_is_idle_start_plus_body_states() {
        let re = build_counting("foo.{0,5}bar");
        let (mut cache, mut idle, start) = cache_with_start(&re);
        let body = body_indices(&re);
        let other = states_accepting(&re, b'r');

        // The start state itself: nothing to return to.
        assert!(!idle.is_idle(&cache, cache.start_id, &re));

        // Start + dead body state: idle.
        let s = with_states(&start, &body);
        let id = cache
            .intern_state(&s, &[], false, false, false, || {})
            .unwrap();
        assert!(idle.is_idle(&cache, id, &re));
        // Cached answer is stable.
        assert!(idle.is_idle(&cache, id, &re));
        assert_eq!(idle.class[id.idx()], Idle::Yes);

        // Start + body + a live non-body thread: not idle.
        let s = with_states(&with_states(&start, &body), &other);
        let id = cache
            .intern_state(&s, &[], false, false, false, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));
        assert_eq!(idle.class[id.idx()], Idle::No);

        // Body state only (start threads missing): not idle.
        let s = with_states(&[], &body);
        let id = cache
            .intern_state(&s, &[], false, false, false, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));
    }

    #[test]
    fn test_is_idle_rejects_flags_and_deferred_asserts() {
        let re = build_counting("foo.{0,5}bar");
        let (mut cache, mut idle, start) = cache_with_start(&re);
        let s = with_states(&start, &body_indices(&re));

        // A match-at-end state still has something to report.
        let id = cache
            .intern_state(&s, &[], false, true, false, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));
        // So does a match state.
        let id = cache
            .intern_state(&s, &[], true, false, false, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));
        // A deferred assertion is a parked thread.
        let parked = [StateIdx(0)];
        let id = cache
            .intern_state(&s, &parked, false, false, true, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));
    }

    #[test]
    fn test_is_idle_requires_context_free_start() {
        // `^` in the start closure: the start state is only valid at
        // offset 0, so no state is ever idle.
        let re = build_counting("^foo.{0,5}bar|baz.{0,5}qux");
        assert!(re.start_closure.is_empty());
        let mut cache = DfaCache::new();
        let mut idle = IdleStates::new();
        let start: Vec<StateIdx> = states_accepting(&re, b'f')
            .into_iter()
            .chain(states_accepting(&re, b'b'))
            .map(StateIdx)
            .collect();
        let start = with_states(&start, &[]);
        cache.start_id = cache
            .intern_state(&start, &[], false, false, false, || {})
            .unwrap();
        let s = with_states(&start, &body_indices(&re));
        let id = cache
            .intern_state(&s, &[], false, false, false, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));

        // Counter in the start closure: the start state owns instances.
        let re = build_counting("a{2,9}b");
        assert!(re.start_closure.is_empty());
        let mut cache = DfaCache::new();
        let mut idle = IdleStates::new();
        let body: Vec<StateIdx> = body_indices(&re).into_iter().map(StateIdx).collect();
        cache.start_id = cache
            .intern_state(&body, &[], false, false, false, || {})
            .unwrap();
        let s = with_states(&body, &states_accepting(&re, b'b'));
        let id = cache
            .intern_state(&s, &[], false, false, false, || {})
            .unwrap();
        assert!(!idle.is_idle(&cache, id, &re));
    }

    #[test]
    fn test_idle_table_grows_on_demand_and_clears() {
        let re = build_counting("foo.{0,5}bar");
        let (mut cache, mut idle, start) = cache_with_start(&re);
        // Nothing is classified until the first query.
        assert!(idle.class.is_empty());
        let s = with_states(&start, &body_indices(&re));
        let id = cache
            .intern_state(&s, &[], false, false, false, || {})
            .unwrap();
        assert!(idle.is_idle(&cache, id, &re));
        assert_eq!(idle.class.len(), cache.states.len());
        assert_eq!(idle.class[cache.start_id.idx()], Idle::Unknown);

        // A state interned after the table was sized is classified too.
        let other = with_states(&s, &states_accepting(&re, b'r'));
        let later = cache
            .intern_state(&other, &[], false, false, false, || {})
            .unwrap();
        assert!(later.idx() >= idle.class.len());
        assert!(!idle.is_idle(&cache, later, &re));
        assert_eq!(idle.class[later.idx()], Idle::No);
        assert!(idle.is_idle(&cache, id, &re), "earlier answers are kept");

        idle.clear();
        assert!(idle.class.is_empty());
        assert!(idle.is_idle(&cache, id, &re), "recomputed after clear");
    }

    // -----------------------------------------------------------------------
    // Hash consistency: DfaState and DfaStateRef must hash identically
    // -----------------------------------------------------------------------

    #[test]
    fn test_state_ref_hash_empty() {
        let state = make_state(&[], &[], false, false, false);
        let r = make_ref(&[], &[], false, false, false);
        assert_eq!(hash_of(&state), hash_of(&r));
    }

    #[test]
    fn test_state_ref_hash_with_nfa_states() {
        let state = make_state(&[1, 5, 42], &[], true, false, false);
        let nfa: Vec<StateIdx> = [1, 5, 42].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], true, false, false);
        assert_eq!(hash_of(&state), hash_of(&r));
    }

    #[test]
    fn test_state_ref_hash_with_deferred() {
        let state = make_state(&[3, 7], &[10, 20], false, true, true);
        let nfa: Vec<StateIdx> = [3, 7].iter().map(|&v| StateIdx(v)).collect();
        let deferred: Vec<StateIdx> = [10, 20].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &deferred, false, true, true);
        assert_eq!(hash_of(&state), hash_of(&r));
    }

    #[test]
    fn test_state_ref_hash_all_flags() {
        // Exhaustively check all 8 flag combinations.
        let nfa_raw = &[2, 4, 6];
        let def_raw = &[8];
        for m in [false, true] {
            for mae in [false, true] {
                for pw in [false, true] {
                    let state = make_state(nfa_raw, def_raw, m, mae, pw);
                    let nfa: Vec<StateIdx> = nfa_raw.iter().map(|&v| StateIdx(v)).collect();
                    let deferred: Vec<StateIdx> = def_raw.iter().map(|&v| StateIdx(v)).collect();
                    let r = make_ref(&nfa, &deferred, m, mae, pw);
                    assert_eq!(
                        hash_of(&state),
                        hash_of(&r),
                        "hash mismatch for flags ({m}, {mae}, {pw})"
                    );
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Equivalent: DfaStateRef must correctly identify equal/unequal states
    // -----------------------------------------------------------------------

    #[test]
    fn test_state_ref_equivalent_match() {
        let state = make_state(&[1, 2, 3], &[10], true, false, true);
        let nfa: Vec<StateIdx> = [1, 2, 3].iter().map(|&v| StateIdx(v)).collect();
        let deferred: Vec<StateIdx> = [10].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &deferred, true, false, true);
        assert!(r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_equivalent_empty() {
        let state = make_state(&[], &[], false, false, false);
        let r = make_ref(&[], &[], false, false, false);
        assert!(r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_not_equivalent_nfa_differs() {
        let state = make_state(&[1, 2], &[], false, false, false);
        let nfa: Vec<StateIdx> = [1, 3].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], false, false, false);
        assert!(!r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_not_equivalent_deferred_differs() {
        let state = make_state(&[1], &[10], false, false, false);
        let nfa: Vec<StateIdx> = [1].iter().map(|&v| StateIdx(v)).collect();
        let deferred: Vec<StateIdx> = [11].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &deferred, false, false, false);
        assert!(!r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_not_equivalent_is_match_differs() {
        let state = make_state(&[1], &[], true, false, false);
        let nfa: Vec<StateIdx> = [1].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], false, false, false);
        assert!(!r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_not_equivalent_is_match_at_end_differs() {
        let state = make_state(&[1], &[], false, true, false);
        let nfa: Vec<StateIdx> = [1].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], false, false, false);
        assert!(!r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_not_equivalent_prev_was_word_differs() {
        let state = make_state(&[1], &[], false, false, true);
        let nfa: Vec<StateIdx> = [1].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], false, false, false);
        assert!(!r.equivalent(&state));
    }

    #[test]
    fn test_state_ref_not_equivalent_length_differs() {
        let state = make_state(&[1, 2, 3], &[], false, false, false);
        let nfa: Vec<StateIdx> = [1, 2].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], false, false, false);
        assert!(!r.equivalent(&state));
    }

    // -----------------------------------------------------------------------
    // IndexSet round-trip: insert a DfaState, probe with DfaStateRef
    // -----------------------------------------------------------------------

    #[test]
    fn test_indexset_probe_with_ref() {
        let mut set: IndexSet<DfaState, ahash::RandomState> = IndexSet::default();
        let state = make_state(&[5, 10, 15], &[20], true, false, false);
        let (idx, inserted) = set.insert_full(state);
        assert!(inserted);
        assert_eq!(idx, 0);

        // Probe with a DfaStateRef — should find the same index.
        let nfa: Vec<StateIdx> = [5, 10, 15].iter().map(|&v| StateIdx(v)).collect();
        let deferred: Vec<StateIdx> = [20].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &deferred, true, false, false);
        assert_eq!(set.get_index_of(&r), Some(0));
    }

    #[test]
    fn test_indexset_probe_miss() {
        let mut set: IndexSet<DfaState, ahash::RandomState> = IndexSet::default();
        set.insert_full(make_state(&[1, 2], &[], false, false, false));

        // Different nfa_states — should not find.
        let nfa: Vec<StateIdx> = [1, 3].iter().map(|&v| StateIdx(v)).collect();
        let r = make_ref(&nfa, &[], false, false, false);
        assert_eq!(set.get_index_of(&r), None);
    }

    #[test]
    fn test_indexset_multiple_states() {
        let mut set: IndexSet<DfaState, ahash::RandomState> = IndexSet::default();
        set.insert_full(make_state(&[1], &[], false, false, false));
        set.insert_full(make_state(&[2], &[], false, false, false));
        set.insert_full(make_state(&[1], &[], true, false, false));

        let nfa1: Vec<StateIdx> = [1].iter().map(|&v| StateIdx(v)).collect();
        let nfa2: Vec<StateIdx> = [2].iter().map(|&v| StateIdx(v)).collect();

        // Each ref finds its corresponding state at the correct index.
        assert_eq!(
            set.get_index_of(&make_ref(&nfa1, &[], false, false, false)),
            Some(0)
        );
        assert_eq!(
            set.get_index_of(&make_ref(&nfa2, &[], false, false, false)),
            Some(1)
        );
        assert_eq!(
            set.get_index_of(&make_ref(&nfa1, &[], true, false, false)),
            Some(2)
        );

        // Non-existent combination.
        assert_eq!(
            set.get_index_of(&make_ref(&nfa2, &[], true, false, false)),
            None
        );
    }

    #[test]
    fn rich_context_hash_equality_and_layout() {
        // The new byte replaces a bool; it must not enlarge cached states.
        type LegacyLayout = (Box<[StateIdx]>, Box<[StateIdx]>, bool, bool, bool);
        assert_eq!(size_of::<DfaState>(), size_of::<LegacyLayout>());
        assert_eq!(size_of::<BoundaryContext>(), size_of::<bool>());
        let mut states = IndexSet::<DfaState>::new();
        let pending = [StateIdx(2)];
        for bits in [0, 1, 2, 4, 8] {
            let mut state = make_state(&[], &[2], false, false, false);
            state.context = BoundaryContext(bits);
            let probe = DfaStateRef {
                nfa_states: &[],
                deferred_asserts: &pending,
                is_match: false,
                is_match_at_end: false,
                context: BoundaryContext(bits),
            };
            assert_eq!(hash_of(&state), hash_of(&probe));
            assert!(probe.equivalent(&state));
            assert!(states.get_index_of(&probe).is_none());
            states.insert(state);
            assert!(states.get_index_of(&probe).is_some());
        }
        assert_eq!(states.len(), 5);
    }

    // ── Previous-byte stand-in (Bug 65) ──

    /// A DFA state keeps only the word-ness of the byte that entered it
    /// and resolves its parked assertions with a stand-in byte.  For
    /// every kind the DFA tiers defer, that must give the same outcome as
    /// the real previous byte — including no previous byte at all, which
    /// the start state records as "not a word byte".
    #[test]
    fn test_prev_stand_in_is_enough_for_deferred_assertions() {
        use crate::is_word_byte;
        let bytes = || std::iter::once(None).chain((0..=255u8).map(Some));
        let mut deferrable = 0;
        for &kind in AssertKind::ALL {
            if !kind.is_dfa_deferrable() {
                continue;
            }
            deferrable += 1;
            for prev in bytes() {
                let stand_in = Some(prev_stand_in(prev.is_some_and(is_word_byte)));
                for at_end in [false, true] {
                    for next in bytes() {
                        assert_eq!(
                            kind.eval(prev.is_none(), at_end, prev, next),
                            kind.eval(false, at_end, stand_in, next),
                            "{kind:?} at_end={at_end} prev={prev:?} next={next:?}"
                        );
                    }
                }
            }
        }
        assert_eq!(deferrable, 5);
    }

    // ── Cache eviction (Bug 64) ──

    /// A cache holding the states `[0]` (the start state), `[1]`, … `[n-1]`.
    fn cache_with_states(n: u32) -> DfaCache {
        let mut cache = DfaCache::new();
        for i in 0..n {
            let id = cache
                .intern_state(&[StateIdx(i)], &[], false, false, false, || {})
                .expect("room");
            assert_eq!(id, DfaStateId(i));
        }
        cache.start_id = DfaStateId(0);
        cache
    }

    #[test]
    fn test_state_cap_has_a_minimum() {
        let mut cache = DfaCache::new();
        assert_eq!(cache.max_states, DFA_MAX_STATES);
        cache.set_max_states(0);
        assert_eq!(cache.max_states, DFA_MIN_STATES);
        cache.set_max_states(100);
        assert_eq!(cache.max_states, 100);
    }

    #[test]
    fn test_intern_refuses_new_states_at_the_cap_but_finds_old_ones() {
        let mut cache = cache_with_states(DFA_MIN_STATES as u32);
        cache.set_max_states(DFA_MIN_STATES);
        assert!(cache.has_room(0));
        assert!(!cache.has_room(1));
        let mut rows = 0;
        assert_eq!(
            cache.intern_state(&[StateIdx(99)], &[], false, false, false, || rows += 1),
            None
        );
        assert_eq!(
            cache.intern_state(&[StateIdx(3)], &[], false, false, false, || rows += 1),
            Some(DfaStateId(3))
        );
        assert_eq!(rows, 0);
    }

    #[test]
    fn test_evict_keeps_start_and_requested_states() {
        let mut cache = cache_with_states(6);
        let mut keep = [DfaStateId(4), DfaStateId::DEAD, DfaStateId(2)];
        let mut rows = 0;
        cache.evict(&mut keep, || rows += 1);

        // Start state first, then the kept ones in order; DEAD is left alone.
        assert_eq!(cache.start_id, DfaStateId(0));
        assert_eq!(keep, [DfaStateId(1), DfaStateId::DEAD, DfaStateId(2)]);
        assert_eq!(cache.states.len(), 3);
        assert_eq!(rows, 3, "one transition row per state put back");
        assert_eq!(cache.evictions, 1);
        assert_eq!(cache.states[0], make_state(&[0], &[], false, false, false));
        assert_eq!(cache.states[1], make_state(&[4], &[], false, false, false));
        assert_eq!(cache.states[2], make_state(&[2], &[], false, false, false));
        assert!(cache.has_room(DFA_MIN_STATES - 3));
    }

    #[test]
    fn test_evict_maps_repeated_and_start_ids_to_one_state() {
        let mut cache = cache_with_states(5);
        // The same state twice, and the start state itself.
        let mut keep = [DfaStateId(3), DfaStateId(0), DfaStateId(3)];
        let mut rows = 0;
        cache.evict(&mut keep, || rows += 1);
        assert_eq!(keep, [DfaStateId(1), DfaStateId(0), DfaStateId(1)]);
        assert_eq!(cache.states.len(), 2);
        assert_eq!(rows, 2);
    }

    #[test]
    fn test_evict_preserves_state_flags() {
        let mut cache = DfaCache::new();
        cache.start_id = cache
            .intern_state(&[StateIdx(0)], &[], false, false, false, || {})
            .unwrap();
        let flagged = cache
            .intern_state(&[StateIdx(1)], &[StateIdx(7)], true, true, true, || {})
            .unwrap();
        let mut keep = [flagged];
        cache.evict(&mut keep, || {});
        assert_eq!(
            cache.states[keep[0].idx()],
            make_state(&[1], &[7], true, true, true)
        );
    }
}
