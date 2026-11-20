//! Tier 2: DFA with differential counters for fixed-length repetition bodies.
//!
//! Uses the Becchi differential-counter technique for O(1) per-character
//! counter management. Supports deferred ASCII word boundaries and LF line ends.
//!
//! **Eligibility**: patterns with bounded repetitions where:
//! - No counter is nested inside another counter's body.
//! - Every counter body has a fixed byte-length (all paths through the body
//!   consume the same number of bytes).
//! - No CRLF assertions.
//! - No deferred assertions inside counter bodies.
//!
//! Counter-free patterns dispatch to Tier 1, which also supports CRLF anchors.

pub(crate) mod eligibility;
mod overlap;

use std::fmt;

use crate::{CounterIdx, Prefilter, Regex, State, StateIdx, byte_match_ci};

use super::{BoundaryContext, DfaCache, DfaMemory, DfaStateId};

/// Maximum counters for tier 2: the `counting_mask` and `counter_reset`
/// fields in `Transition` are `u64` bitmasks.
pub(crate) const MAX_TIER2_COUNTERS: usize = 64;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Delta pool (arena-backed linked list for counter deltas)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Differential counter (per-phase, ring-buffer deltas)
// ---------------------------------------------------------------------------

/// Differential counter for one phase of one counter.
///
/// All instances in the same phase increment in lockstep.  We store only
/// the oldest instance's absolute value and the count of active instances.
/// Deltas between consecutive instances are stored in a power-of-two
/// ring buffer for cache-friendly FIFO access: dealloc pops from head,
/// alloc pushes to tail, and both ends wrap around the same contiguous
/// memory.
#[derive(Clone, Debug)]
struct DiffCounter {
    /// Absolute value of the oldest (highest-value) instance.
    oldest: u32,
    /// Number of active instances.
    count: u32,
    /// Sum of all deltas (oldest_value − youngest_value).
    total_delta: u32,
    /// Ring buffer of delta values between consecutive instances.
    /// Capacity is always a power of two.
    ring: Vec<u32>,
    /// Index of the oldest delta in `ring` (head of queue).
    head: u32,
    /// Bitmask for fast modular indexing: `ring.len() - 1`.
    mask: u32,
}

impl DiffCounter {
    fn new() -> Self {
        Self {
            oldest: 0,
            count: 0,
            total_delta: 0,
            ring: Vec::new(),
            head: 0,
            mask: 0,
        }
    }

    /// Pre-allocate the ring buffer.  `max_instances` is the counter's
    /// `max` value — capped to avoid huge allocations for unbounded
    /// repetitions.  The ring grows on demand if more capacity is needed.
    fn reserve(&mut self, max_instances: usize) {
        // Cap initial allocation: 4096 entries = 16 KB per phase.
        // Unbounded counters (max = usize::MAX) start small and grow.
        let cap = max_instances.clamp(1, 4096).next_power_of_two();
        if self.ring.len() < cap {
            self.ring.resize(cap, 0);
        }
        self.mask = (self.ring.len() as u32) - 1;
    }

    /// Double the ring buffer capacity, preserving existing contents
    /// in their correct logical order.
    fn grow(&mut self) {
        let old_cap = self.ring.len();
        let new_cap = (old_cap * 2).max(4);
        let old_mask = self.mask;
        let head = self.head;
        let n = self.count.saturating_sub(1) as usize; // number of deltas

        let mut new_ring = vec![0u32; new_cap];
        for (i, dst) in new_ring[..n].iter_mut().enumerate() {
            *dst = self.ring[((head.wrapping_add(i as u32)) & old_mask) as usize];
        }
        self.ring = new_ring;
        self.mask = (new_cap as u32) - 1;
        self.head = 0;
    }

    /// Youngest instance value.
    #[inline]
    fn youngest(&self) -> u32 {
        debug_assert!(self.count > 0);
        self.oldest - self.total_delta
    }

    /// Increment all instances (only need to bump oldest).
    #[inline]
    fn increment_all(&mut self) {
        debug_assert!(self.count > 0);
        self.oldest += 1;
    }

    /// Deallocate the oldest instance.  Returns `true` if instances remain.
    #[inline]
    fn dealloc_oldest(&mut self) -> bool {
        debug_assert!(self.count > 0);
        self.count -= 1;
        if self.count > 0 {
            let d = self.ring[(self.head & self.mask) as usize];
            self.head = self.head.wrapping_add(1);
            self.oldest -= d;
            self.total_delta -= d;
            true
        } else {
            self.total_delta = 0;
            false
        }
    }

    /// Allocate a new instance with a given initial value (youngest).
    #[inline]
    fn alloc_new_with_value(&mut self, value: u32) {
        if self.count == 0 {
            self.oldest = value;
            self.count = 1;
            self.total_delta = 0;
        } else {
            // Number of deltas = count - 1 (before adding the new one).
            // If the ring is full, grow it.
            let num_deltas = (self.count - 1) as usize;
            if num_deltas >= self.ring.len() {
                self.grow();
            }
            let youngest_val = self.youngest();
            let gap = youngest_val - value;
            let tail = self.head.wrapping_add(self.count - 1);
            self.ring[(tail & self.mask) as usize] = gap;
            self.total_delta += gap;
            self.count += 1;
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Reset to empty state (ring buffer capacity retained).
    #[inline]
    fn clear(&mut self) {
        self.oldest = 0;
        self.count = 0;
        self.total_delta = 0;
        self.head = 0;
    }
}

// ---------------------------------------------------------------------------
// Per-counter metadata (no heap)
// ---------------------------------------------------------------------------

/// Per-counter metadata indexing into the flat phases array.
#[derive(Clone, Debug)]
struct CounterMeta {
    /// Start index in the flat `phases` array.
    phase_start: usize,
    /// Number of phases (= body_length).
    num_phases: usize,
    /// Which phase fires CInc on the current byte (cycles 0..L-1).
    active_phase: usize,
    /// Counter parameters.
    min: u32,
    max: u32,
}

// ---------------------------------------------------------------------------
// Transition
// ---------------------------------------------------------------------------

/// A successor in a cached transition: a state index with tag bits on top.
///
/// The low bits index [`DfaCache::states`].  The tags say what the scanning
/// loop has to know about the edge, so that one comparison
/// ([`is_tagged`](Self::is_tagged)) tells an edge that only moves the
/// current state from every other kind: not computed yet, into the DEAD
/// state, a match, or one that counts or seeds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tier2StateId(u32);

impl Tier2StateId {
    // Two kinds of constant.  One of type `Self` is a complete value, which
    // an entry or the current state can be equal to: `UNKNOWN`, `START`,
    // `DEAD`.  One named `*_TAG` is a single bit, a `u32` that the methods
    // below combine with a state index and that nothing compares an id with.
    // `MAX` is neither: the largest index, and the mask of the index bits.
    //
    // A tag that Tier 1 has too has the value it has in `Tier1StateId`: the
    // match tag, and `UNKNOWN`.  Bit 29 is Tier 1's start tag and is left
    // unused here, so that a start tag in this tier can take the same bit.
    // The two tags only this tier has sit below it.

    /// A transition that is not computed yet.
    const UNKNOWN: Self = Self(1 << 31);
    /// The edge reports a match: its successor is a match state, or an
    /// assertion resolved by its byte reached one.
    const MATCH_TAG: u32 = 1 << 30;
    /// The successor is the DEAD state; the index bits are zero.
    const DEAD_TAG: u32 = 1 << 28;
    /// The edge crosses an increment or seeds a counter: it has to go
    /// through [`Tier2DfaMatcher::step_inner`] even when no instance is
    /// live.  Set on [`Transition::no_break`] only.
    const EFFECT_TAG: u32 = 1 << 27;
    /// Largest untagged value, and the mask of the index bits.
    const MAX: u32 = Self::EFFECT_TAG - 1;

    /// The start state as a current state: state 0, also after the cache
    /// is emptied.
    const START: Self = Self(0);
    /// The DEAD state as a current state.
    const DEAD: Self = Self(Self::DEAD_TAG);

    /// A current state from a plain id (which may be DEAD).
    #[inline]
    fn from_state(state: DfaStateId) -> Self {
        if state == DfaStateId::DEAD {
            Self::DEAD
        } else {
            debug_assert!(state.0 <= Self::MAX);
            Self(state.0)
        }
    }

    /// The same state with the tags of an edge into it.
    #[inline]
    fn with_tags(self, is_match: bool, effect: bool) -> Self {
        debug_assert_eq!(self, self.as_current());
        let mut id = self.0;
        if is_match {
            id |= Self::MATCH_TAG;
        }
        if effect {
            id |= Self::EFFECT_TAG;
        }
        Self(id)
    }

    /// Whether any tag is set.
    #[inline(always)]
    fn is_tagged(self) -> bool {
        self.0 > Self::MAX
    }

    #[inline(always)]
    fn is_match(self) -> bool {
        self.0 & Self::MATCH_TAG != 0
    }

    #[inline(always)]
    fn is_dead(self) -> bool {
        self.0 & Self::DEAD_TAG != 0
    }

    #[inline(always)]
    fn has_effect(self) -> bool {
        self.0 & Self::EFFECT_TAG != 0
    }

    /// The entry as the matcher's current state: where the edge leads,
    /// without what taking it involved.  One mask, on the path from one
    /// state to the next.
    #[inline(always)]
    fn as_current(self) -> Self {
        Self(self.0 & (Self::MAX | Self::DEAD_TAG))
    }

    /// Index of a current state in the state table.  Not for DEAD.
    #[inline(always)]
    fn idx(self) -> usize {
        debug_assert!(!self.is_tagged());
        self.0 as usize
    }

    /// The successor state as a plain id.  Not for `UNKNOWN`, which has
    /// none: it would come out as state 0, the start state.  There is no
    /// `==` between this type and `DfaStateId` for that reason.
    #[inline(always)]
    fn state(self) -> DfaStateId {
        debug_assert!(self != Self::UNKNOWN, "an uncomputed entry has no state");
        // All ones (`DfaStateId::DEAD`) when the DEAD tag is set, without a
        // branch: this sits on the path from one state to the next.
        let dead = ((self.0 >> Self::DEAD_TAG.trailing_zeros()) & 1).wrapping_neg();
        DfaStateId((self.0 & Self::MAX) | dead)
    }
}

/// Most states a Tier 2 cache can be asked to hold: every state index has
/// to fit below the tag bits.  A larger cap is lowered to this.
const TIER2_MAX_STATES: usize = Tier2StateId::MAX as usize + 1;

/// Cached DFA transition for `(state, byte)`.
///
/// Non-counting transitions use `no_break` only.
/// Counting transitions select between `no_break` and `with_break`
/// based on the differential counter condition.
#[derive(Clone)]
struct Transition {
    /// DFA successor when no counter instance can break, tagged with what
    /// taking this edge involves.
    no_break: Tier2StateId,
    /// DFA successor when at least one instance can break, with the match
    /// tag of taking the edge that way.  It never carries the "counts or
    /// seeds" tag: that one is read from `no_break`, before the counters
    /// decide between the two.
    with_break: Tier2StateId,
    /// Bitmask: bit `i` set means counter `i` fires CInc at this transition.
    counting_mask: u64,
    /// Seeds to apply BEFORE counter_increment.
    ///
    /// These come from Phase 1 deferred-assertion resolution on counting
    /// transitions where the resolved path consumed the L=1 counter body
    /// (initial_value > 0).  The seed must be visible to the increment
    /// so the counter can break on this very transition.
    pre_seeds: Box<[(CounterIdx, u32)]>,
    /// Seeds to apply AFTER counter_increment and counter_reset.
    ///
    /// Probe seeds (initial_value = 0) that start new counter instances
    /// for subsequent transitions.
    seeds: Box<[(CounterIdx, u32)]>,
    /// Bitmask: bit `i` set means counter `i` has NO body-progress NFA
    /// states in the DFA successor (only re-seeded first-byte states).
    /// Existing instances for that counter must be cleared before seeding.
    counter_reset: u64,
    /// Bitmask: bit `i` set means the *active phase* of counter `i` must
    /// be cleared.  The threads of the active phase stand on the last body
    /// byte: on this byte they cross the increment or die.  The bit is set
    /// for a multi-byte body when this transition does not cross counter
    /// `i`'s increment, so those threads died.  Their counts must not be
    /// inherited by the threads seeded into the same phase next (Bug 63).
    phase_reset: u64,
}

impl Transition {
    fn empty() -> Self {
        Self {
            no_break: Tier2StateId::UNKNOWN,
            with_break: Tier2StateId::UNKNOWN,
            counting_mask: 0,
            pre_seeds: Box::new([]),
            seeds: Box::new([]),
            counter_reset: 0,
            phase_reset: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Tier 2 analysis (precomputed at build time)
// ---------------------------------------------------------------------------

/// Precomputed NFA analysis for Tier 2 patterns.
///
/// Built once at regex compile time by [`compute_tier2_analysis`],
/// stored on the [`Regex`] struct, and shared by all matchers.
#[derive(Debug)]
pub(crate) struct Tier2Analysis {
    /// Flat array of NFA consuming-state indices that are in the "interior"
    /// of each counter body (body positions 1..L-1 for a body of length L).
    /// For L=1 counters, there are no interior states.
    ///
    /// Used to detect when a DFA successor has in-progress body instances:
    /// if the successor contains any interior state for counter `c`, then
    /// counter `c`'s instances should survive.  Otherwise, old instances
    /// should be cleared (only re-seeded first-byte instances remain).
    body_interior: Box<[u32]>,
    /// Per-counter `(start, end)` range into [`body_interior`](Self::body_interior).
    body_ranges: Box<[(usize, usize)]>,
    /// Per-NFA-state bitmask of counter indices whose `CounterIncrement`
    /// is reachable via epsilon transitions.  Indexed by `StateIdx`.
    /// Replaces the runtime `find_cinc_counters` graph walk.
    cinc_reachable: Box<[u64]>,
    /// Flat array of consuming NFA states reachable from each counter's
    /// `CounterInstance.out` via epsilon transitions.  Used to build
    /// seed instances at populate time without a graph walk.
    ci_seed_consuming: Box<[StateIdx]>,
    /// Per-counter `(start, end)` range into [`ci_seed_consuming`].
    ci_seed_consuming_ranges: Box<[(usize, usize)]>,
}

impl Tier2Analysis {
    /// Returns the sorted slice of interior NFA state indices for counter
    /// `ci`.  Empty for L=1 counters.
    pub(crate) fn interior(&self, ci: usize) -> &[u32] {
        let (start, end) = self.body_ranges[ci];
        &self.body_interior[start..end]
    }

    /// Returns the bitmask of counter indices whose `CounterIncrement`
    /// is reachable via epsilon transitions from NFA state `idx`.
    pub(crate) fn cinc_reachable_from(&self, idx: StateIdx) -> u64 {
        self.cinc_reachable[idx.idx()]
    }

    /// Returns the consuming NFA states reachable from counter `ci`'s
    /// `CounterInstance.out` via epsilon transitions.
    pub(crate) fn seed_consuming(&self, ci: usize) -> &[StateIdx] {
        let (start, end) = self.ci_seed_consuming_ranges[ci];
        &self.ci_seed_consuming[start..end]
    }
}

/// Precompute the per-counter and per-state tables of a Tier 2 pattern:
///
/// 1. **Interior states** — NFA consuming states in the "interior" of the
///    counter body (body positions 1..L-1 for a body of length L).  Used to
///    detect when a DFA successor has in-progress body instances.
/// 2. **Increment reachability** — per NFA state, the counters whose
///    `CInc` is reachable through epsilon transitions.
/// 3. **Seed states** — per counter, the consuming states its
///    `CounterInstance` leads to.
///
/// # Panics
///
/// If an increment is reachable, without consuming a byte, from an
/// assertion that looks ahead.  Eligibility excludes such patterns
/// (requirement 6 in [`eligibility`]) and the matcher has no code for
/// them: it only increments on the transition that consumes the last
/// byte of a body.
pub(crate) fn compute_tier2_analysis(
    states: &[State],
    byte_tables: &[crate::ByteMap],
    num_counters: usize,
) -> Tier2Analysis {
    let mut per_counter: Vec<Vec<u32>> = vec![Vec::new(); num_counters];

    for s in states.iter() {
        if let State::CounterInstance { counter, out } = s {
            let ci = counter.idx();
            // Find "first" consuming states: reachable from CI.out
            // through epsilon states only.
            let first = consuming_states_from(*out, states);
            let first_set: std::collections::HashSet<u32> = first.iter().map(|s| s.0).collect();

            // Walk the entire body from CI.out, following consuming
            // states through their successors, to find all consuming
            // states.
            let mut all_body_consuming: Vec<u32> = Vec::new();
            let mut visited = vec![false; states.len()];
            super::walk_counter_body(states, byte_tables, *out, &mut visited, |idx, state| {
                if super::is_consuming(state) {
                    all_body_consuming.push(idx.0);
                }
            });

            // Interior = all body consuming states minus the first states.
            let mut interior: Vec<u32> = all_body_consuming
                .into_iter()
                .filter(|s| !first_set.contains(s))
                .collect();
            interior.sort_unstable();
            interior.dedup();
            per_counter[ci] = interior;
        }
    }

    // Flatten body interior into a single Vec with (start, end) ranges.
    let mut flat: Vec<u32> = Vec::new();
    let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(num_counters);
    for v in per_counter {
        let start = flat.len();
        flat.extend(v);
        ranges.push((start, flat.len()));
    }

    // Precompute per-NFA-state CInc reachability bitmask.
    // For each state, walk epsilon transitions and record which
    // CounterIncrement counters are reachable.
    let n = states.len();
    let mut cinc_reachable = vec![0u64; n];
    for (si, _) in states.iter().enumerate() {
        let mut mask = 0u64;
        let mut stack = vec![StateIdx(si as u32)];
        let mut visited = vec![false; n];
        while let Some(idx) = stack.pop() {
            let i = idx.idx();
            if visited[i] {
                continue;
            }
            visited[i] = true;
            match states[idx] {
                State::CounterIncrement { counter, .. } => {
                    mask |= 1u64 << counter.idx();
                }
                State::Split { out, out1 } => {
                    stack.push(out1);
                    stack.push(out);
                }
                State::Assert { out, .. } => stack.push(out),
                State::CounterInstance { out, .. } => stack.push(out),
                _ => {}
            }
        }
        cinc_reachable[si] = mask;
    }

    // The matcher's central assumption (eligibility requirement 6): an
    // increment directly follows a consumed byte, never an assertion a
    // thread can wait on.  The eligibility rule and this table are
    // computed by different walks; a pattern on which they disagree must
    // not reach the matcher.
    for state in states {
        if let State::Assert { kind, out } = *state
            && kind.looks_ahead()
        {
            assert_eq!(
                cinc_reachable[out.idx()],
                0,
                "tier 2 pattern with an increment behind `{kind:?}`"
            );
        }
    }

    // Precompute per-counter seed consuming states: consuming NFA states
    // reachable from each CounterInstance.out via epsilon transitions.
    let mut seed_per_counter: Vec<Vec<StateIdx>> = vec![Vec::new(); num_counters];
    for s in states.iter() {
        if let State::CounterInstance { counter, out } = *s {
            let ci = counter.idx();
            if ci < num_counters && seed_per_counter[ci].is_empty() {
                seed_per_counter[ci] = consuming_states_from(out, states);
            }
        }
    }
    let mut seed_flat: Vec<StateIdx> = Vec::new();
    let mut seed_ranges: Vec<(usize, usize)> = Vec::with_capacity(num_counters);
    for v in seed_per_counter {
        let start = seed_flat.len();
        seed_flat.extend(v);
        seed_ranges.push((start, seed_flat.len()));
    }

    Tier2Analysis {
        body_interior: flat.into_boxed_slice(),
        body_ranges: ranges.into_boxed_slice(),
        cinc_reachable: cinc_reachable.into_boxed_slice(),
        ci_seed_consuming: seed_flat.into_boxed_slice(),
        ci_seed_consuming_ranges: seed_ranges.into_boxed_slice(),
    }
}

// ---------------------------------------------------------------------------
// Counter state (differential counter runtime)
// ---------------------------------------------------------------------------

/// Runtime state for all differential counters in a Tier 2 matcher.
///
/// Owns the per-phase [`DiffCounter`]s and per-counter [`CounterMeta`].
/// Extracted from [`Tier2DfaCache`] so that counter operations can be
/// called while other cache fields (e.g. `transitions`) are borrowed.
pub(crate) struct CounterState {
    /// Flat array of differential counters for all phases of all counters.
    /// Indexed via [`CounterMeta::phase_start`].
    phases: Vec<DiffCounter>,
    /// Per-counter metadata (phase range, active_phase, min, max).
    meta: Vec<CounterMeta>,
    /// Total number of phase slots (sum of num_phases across counters).
    /// Computed once in [`Tier2DfaCache::prepare`].
    total_phases: usize,
    /// Number of phases with at least one live instance.  Maintained
    /// incrementally so that [`has_live`](Self::has_live) is O(1).
    live_count: usize,
}

impl CounterState {
    fn new() -> Self {
        Self {
            phases: Vec::new(),
            meta: Vec::new(),
            total_phases: 0,
            live_count: 0,
        }
    }

    /// Populate counter metadata from a slice of `(min, max, body_len)`
    /// tuples — one per counter.  Called once per regex in
    /// Populate counter metadata and pre-allocate ring buffers.
    /// Called once per regex in [`Tier2DfaCache::prepare`].
    fn populate(&mut self, counters: &[crate::CounterInfo]) {
        self.phases.clear();
        self.meta.clear();
        for spec in counters {
            let nph = spec.body_byte_length.max(1);
            let phase_start = self.phases.len();
            for _ in 0..nph {
                let mut dc = DiffCounter::new();
                dc.reserve(spec.max);
                self.phases.push(dc);
            }
            self.meta.push(CounterMeta {
                phase_start,
                num_phases: nph,
                active_phase: 0,
                min: spec.min as u32,
                max: spec.max as u32,
            });
        }
        self.total_phases = self.phases.len();
    }

    /// Safety net: if phases were dropped (e.g. by [`clear`]), re-create
    /// them.  Normally a no-op since [`populate`] creates phases and
    /// [`reset`] preserves them.
    #[inline]
    fn ensure_phases(&mut self) {
        if !self.phases.is_empty() {
            return;
        }
        self.phases.resize(self.total_phases, DiffCounter::new());
        for m in &mut self.meta {
            m.active_phase = 0;
            let max = m.max as usize;
            for p in &mut self.phases[m.phase_start..m.phase_start + m.num_phases] {
                p.reserve(max);
            }
        }
    }

    /// Reset counter state between matches.  Clears logical state of
    /// each phase but retains ring buffer allocations for reuse.
    fn reset(&mut self) {
        for p in &mut self.phases {
            p.clear();
        }
        for m in &mut self.meta {
            m.active_phase = 0;
        }
        self.live_count = 0;
    }

    /// Clear everything including metadata (used when switching regex).
    fn clear(&mut self) {
        self.phases.clear();
        self.meta.clear();
        self.total_phases = 0;
        self.live_count = 0;
    }

    /// Increment the active phase of counter `c_idx`.
    /// Returns `true` if any instance can break (oldest >= min).
    #[inline(always)]
    fn increment(&mut self, c_idx: usize) -> bool {
        let m = &self.meta[c_idx];
        let phase_idx = m.phase_start + m.active_phase;

        let phase = &mut self.phases[phase_idx];
        if phase.is_empty() {
            return false;
        }
        phase.increment_all();
        let can_break = phase.oldest >= m.min;
        if can_break {
            while !phase.is_empty() && phase.oldest >= m.max {
                phase.dealloc_oldest();
            }
            if phase.is_empty() {
                self.live_count -= 1;
            }
        }
        can_break
    }

    /// Clear all phases of counter `c_idx`.
    #[inline]
    fn reset_counter(&mut self, c_idx: usize) {
        let start = self.meta[c_idx].phase_start;
        let end = start + self.meta[c_idx].num_phases;
        for p in &mut self.phases[start..end] {
            if !p.is_empty() {
                self.live_count -= 1;
                p.clear();
            }
        }
    }

    /// Clear the active phase of counter `c_idx`: the instances whose
    /// threads stand on the last body byte.
    #[inline]
    fn reset_active_phase(&mut self, c_idx: usize) {
        let m = &self.meta[c_idx];
        let phase = &mut self.phases[m.phase_start + m.active_phase];
        if !phase.is_empty() {
            self.live_count -= 1;
            phase.clear();
        }
    }

    /// Advance active_phase for every counter (called on every byte).
    #[inline]
    fn advance_all_phases(&mut self) {
        // Wrap by comparison: `%` is a division on every byte, also when a
        // one-byte body has a single phase.
        for m in &mut self.meta {
            let next = m.active_phase + 1;
            m.active_phase = if next == m.num_phases { 0 } else { next };
        }
    }

    /// Seed a new counter instance into the appropriate phase.
    #[inline(always)]
    fn seed(&mut self, c_idx: usize, initial_value: u32) {
        let m = &self.meta[c_idx];
        let nph = m.num_phases;
        // The phase before the active one, wrapped the same way.
        let target_phase = if m.active_phase == 0 {
            nph - 1
        } else {
            m.active_phase - 1
        };
        let phase_idx = m.phase_start + target_phase;
        let was_empty = self.phases[phase_idx].is_empty();
        self.phases[phase_idx].alloc_new_with_value(initial_value);
        if was_empty {
            self.live_count += 1;
        }
    }

    /// Check if any phase has live instances.  O(1) via incremental
    /// tracking in [`seed`], [`increment`], [`reset_counter`], and
    /// [`reset`].
    #[inline]
    fn has_live(&self) -> bool {
        self.live_count > 0
    }
}

// ---------------------------------------------------------------------------
// Tier 2 DFA cache
// ---------------------------------------------------------------------------

/// Lazy DFA cache for Tier 2 (differential counters + deferred assertions).
pub(crate) struct Tier2DfaCache {
    inner: DfaCache,
    stride: usize,
    transitions: Vec<Transition>,
    start_seeds: Box<[(CounterIdx, u32)]>,
    /// Runtime state for all differential counters.
    counters: CounterState,
    /// Cached transitions out of the DEAD state, one per byte class
    /// (`stride` entries).  The DEAD state has no row in
    /// [`transitions`](Self::transitions); without this cache every byte
    /// consumed while dead would recompute the restart closure.
    dead_transitions: Vec<Transition>,
    /// Per DFA state: the state to continue from once no counter instance
    /// is live — the same state without its counter-body NFA states (see
    /// [`settled_state`](Self::settled_state)).  Grown on demand;
    /// [`Tier2StateId::UNKNOWN`] marks entries not computed yet.
    settled: Vec<Tier2StateId>,
    /// Whether [`settled_state`](Self::settled_state) may be used for the
    /// prepared regex.  Computed once in [`prepare`](Self::prepare).
    settle_enabled: bool,
}

impl fmt::Debug for Tier2DfaCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tier2DfaCache")
            .field("num_states", &self.inner.states.len())
            .field("max_states", &self.inner.max_states)
            .field("evictions", &self.inner.evictions)
            .finish()
    }
}

impl Tier2DfaCache {
    pub(crate) fn new() -> Self {
        Self {
            inner: DfaCache::new(),
            stride: 256,
            transitions: Vec::new(),
            start_seeds: Box::new([]),
            counters: CounterState::new(),
            dead_transitions: Vec::new(),
            settled: Vec::new(),
            settle_enabled: false,
        }
    }

    /// Set the state cap (see [`DfaCache::set_max_states`]).  A cap above
    /// [`TIER2_MAX_STATES`] is lowered to it.
    pub(crate) fn set_max_states(&mut self, max_states: usize) {
        self.inner.set_max_states(max_states.min(TIER2_MAX_STATES));
    }

    /// Number of times the cache was emptied because it was full.
    #[cfg(test)]
    pub(crate) fn evictions(&self) -> u64 {
        self.inner.evictions
    }

    /// Make room for the two states (break and no-break successor) a
    /// missing transition out of `from` can add: when the cache is full,
    /// empty it and put `from` back.  Returns the id `from` has
    /// afterwards.
    ///
    /// The counters are the stream's own state and are left alone.
    #[inline]
    fn make_room(&mut self, from: Tier2StateId) -> Tier2StateId {
        if self.inner.has_room(2) {
            from
        } else {
            self.evict(from)
        }
    }

    #[cold]
    fn evict(&mut self, from: Tier2StateId) -> Tier2StateId {
        let stride = self.stride;
        // The shared table renumbers plain ids.
        let mut keep = [from.state()];
        self.transitions.clear();
        self.settled.clear();
        self.dead_transitions.clear();
        self.dead_transitions.resize_with(stride, Transition::empty);
        self.inner.evict(&mut keep, || {
            self.transitions
                .extend(std::iter::repeat_with(Transition::empty).take(stride));
        });
        Tier2StateId::from_state(keep[0])
    }

    /// Look up or insert a DFA state.  `None` when the state cap is
    /// reached; transitions make room first
    /// ([`make_room`](Self::make_room)), so only the optional settled
    /// states ever see it.
    fn intern_state(
        &mut self,
        nfa_states: &[StateIdx],
        deferred_asserts: &[StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        context: BoundaryContext,
        regex: &Regex,
    ) -> Option<Tier2StateId> {
        let stride = self.stride;
        let needs = deferred_asserts.iter().fold(0, |needs, &idx| {
            needs | regex.dfa_assertions.requirements(idx)
        });
        let id = self.inner.intern_context(
            nfa_states,
            deferred_asserts,
            is_match,
            is_match_at_end,
            context.masked(needs),
            || {
                self.transitions
                    .extend(std::iter::repeat_with(Transition::empty).take(stride));
            },
        );
        // The one place a plain id of the shared table becomes a tagged one.
        id.map(Tier2StateId::from_state)
    }

    // -----------------------------------------------------------------------
    // Epsilon closure (reuses Tier 1's approach + counter awareness)
    // -----------------------------------------------------------------------

    /// Compute epsilon closure.
    ///
    /// `follow_break`: whether to follow the CInc break path (out1).
    /// The continue path (out) is always followed.
    #[allow(clippy::too_many_arguments)]
    fn epsilon_closure(
        &mut self,
        memory: &mut DfaMemory,
        seeds: impl Iterator<Item = StateIdx>,
        regex: &Regex,
        analysis: &Tier2Analysis,
        at_start: bool,
        prev_byte: Option<u8>,
        next_byte: Option<u8>,
        follow_break: bool,
    ) -> ClosureResult {
        let mut encountered_cinc = false;

        let (is_match, is_match_at_end) = memory.epsilon_closure::<true>(
            seeds,
            regex,
            at_start,
            false,
            prev_byte,
            next_byte,
            |mem, counter, out| {
                mem.closure_stack.push(out);
                mem.closure_seeds.push((counter, out));
            },
            |mem, _, out, out1, _, _| {
                encountered_cinc = true;
                mem.closure_stack.push(out);
                if follow_break {
                    mem.closure_stack.push(out1);
                }
            },
        );

        let mut seed_instances: Vec<(CounterIdx, StateIdx)> = Vec::new();
        for &(counter, _ci_out) in &memory.closure_seeds {
            for &c in analysis.seed_consuming(counter.idx()) {
                seed_instances.push((counter, c));
            }
        }
        seed_instances.sort();
        seed_instances.dedup();

        ClosureResult {
            nfa_states: memory.closure_result.as_slice().into(),
            deferred_asserts: memory.closure_deferred.as_slice().into(),
            is_match,
            is_match_at_end,
            encountered_cinc,
            seed_instances: seed_instances.into_boxed_slice(),
        }
    }

    // -----------------------------------------------------------------------
    // Transition computation
    // -----------------------------------------------------------------------

    fn populate(
        &mut self,
        memory: &mut DfaMemory,
        from: Tier2StateId,
        byte: u8,
        regex: &Regex,
        analysis: &Tier2Analysis,
    ) -> Transition {
        let mut targets: Vec<StateIdx> = Vec::new();

        let mut resolved_is_match = false;
        // Note: we do NOT track resolved_is_match_at_end here.  That flag
        // means "the deferred assertion passed mid-stream (with next=byte) and
        // $ → Match was reachable."  But the assertion was evaluated with
        // next=Some(byte) — at actual end-of-input, next is None and word-
        // boundary conditions may flip. The parked state already carries the EOF
        // answer evaluated with its physical boundary.
        // Counters entered through Phase 1 resolution whose first body
        // byte was consumed on this transition (bit `i` = counter `i`).
        // Only these can need a pre-seed.
        // A counter that was entered but whose body did not take the byte
        // is speculative and must not be seeded (Bug 14).  Another
        // resolved thread taking the byte — a state after the counter's
        // break, say — says nothing about the body (Bug 62).
        let mut resolved_body_mask: u64 = 0;

        if !from.is_dead() {
            let from_state = &self.inner.states[from.idx()];
            // Phase 1: resolve deferred assertions.
            let extra = from_state.resolve_deferred(byte, regex);
            if !extra.is_empty() {
                let context = from_state.context;
                // Use follow_break=false: the CInc break path should NOT
                // contribute to resolved_is_match.  Match via counter break
                // is handled by the with_break DFA successor, gated by
                // the counter condition in step_inner.
                let cr = self.epsilon_closure(
                    memory,
                    extra.into_iter(),
                    regex,
                    analysis,
                    context.at_start(),
                    context.previous(),
                    Some(byte),
                    false,
                );
                for &(counter, first) in cr.seed_instances.iter() {
                    if cr.nfa_states.contains(&first) && consume_byte(first, byte, regex).is_some()
                    {
                        resolved_body_mask |= 1u64 << counter.idx();
                    }
                }

                // A resolved assertion never leads to an increment without
                // consuming a byte: eligibility requirement 6, checked
                // for every pattern in `compute_tier2_analysis()`.  So
                // increments are only ever seen by the probe below, on
                // the transition that consumes the last byte of a body.
                debug_assert!(
                    !cr.encountered_cinc,
                    "tier 2 pattern with an increment behind an assertion"
                );
                resolved_is_match = cr.is_match;
                // Discard cr.is_match_at_end — see comment above.

                for &idx in cr.nfa_states.iter() {
                    if let Some((t, te)) = consume_byte(idx, byte, regex) {
                        targets.push(t);
                        if te != StateIdx::NONE {
                            targets.push(te);
                        }
                    }
                }
            }

            // Phase 2: consuming states consume `byte`.
            for &idx in &self.inner.states[from.idx()].nfa_states {
                if let Some((t, te)) = consume_byte(idx, byte, regex) {
                    targets.push(t);
                    if te != StateIdx::NONE {
                        targets.push(te);
                    }
                }
            }
        }

        // Probe: full "both" closure to detect CInc and collect seeds.
        let probe = self.epsilon_closure(
            memory,
            targets.iter().copied().chain(std::iter::once(regex.start)),
            regex,
            analysis,
            false,
            Some(byte),
            None,
            true,
        );

        let is_counting = probe.encountered_cinc;

        // Find ALL counters that fire CInc at this transition.
        // Guarded by is_counting because the precomputed cinc_reachable
        // is a static over-approximation that ignores assertion gating.
        let counting_mask = if is_counting {
            targets
                .iter()
                .fold(0u64, |acc, &t| acc | analysis.cinc_reachable_from(t))
        } else {
            0
        };

        // Disjoint counter bodies can fire at most one increment per byte.
        // Overlapping counters admitted by common_terminal_break have no
        // body assertions and all breaks reach the same terminal action.
        debug_assert!(
            regex.tier2_overlap_proven || counting_mask.count_ones() <= 1,
            "Tier 2 disjoint-body pattern has counting_mask with {} bits set \
             on DFA state {:?} byte {}: this violates the disjoint-bytes \
             invariant (the overlap proof was not used for this pattern)",
            counting_mask.count_ones(),
            from,
            byte,
        );

        // Build the post-seed list from probe seeds.
        let mut seed_list: Vec<(CounterIdx, u32)> = probe
            .seed_instances
            .iter()
            .map(|&(c, _s)| (c, 0u32))
            .collect();
        // Resolved seeds from deferred-assertion resolution are NOT
        // merged into seed_list.  They go to pre_seeds (built below)
        // for correct phase alignment — see the pre_seed comment.
        seed_list.sort_by_key(|&(c, _)| c);
        seed_list.dedup();

        // A deferred entry consumed its first byte during resolution, so it
        // must be seeded before the increment and the phase clock advance.
        // Keep it only if the body made progress: for a one-byte body the
        // increment must be reached, including any guard after the consumed
        // byte (Bug 73). A longer body can reach an interior state instead.
        let counter_reset = self.compute_counter_reset(analysis, &probe.nfa_states, counting_mask);
        let pre_seed_list = Self::pre_seeds_for(resolved_body_mask & !counter_reset);
        let phase_reset = Self::compute_phase_reset(analysis, counting_mask, counter_reset);

        let counts_or_seeds =
            counting_mask != 0 || !pre_seed_list.is_empty() || !seed_list.is_empty();

        // Compute DFA successors.
        if counting_mask != 0 {
            let cr_nb = self.epsilon_closure(
                memory,
                targets.iter().copied().chain(std::iter::once(regex.start)),
                regex,
                analysis,
                false,
                Some(byte),
                None,
                false,
            );
            let nb_id = self.intern_closure_result(&cr_nb, byte, regex);
            let wb_id = self.intern_closure_result(&probe, byte, regex);
            let nb_m = self.is_match(nb_id);
            let wb_m = self.is_match(wb_id);

            Transition {
                no_break: nb_id.with_tags(nb_m || resolved_is_match, counts_or_seeds),
                with_break: wb_id.with_tags(wb_m || resolved_is_match, false),
                counting_mask,
                pre_seeds: pre_seed_list.into_boxed_slice(),
                seeds: seed_list.into_boxed_slice(),
                counter_reset,
                phase_reset,
            }
        } else {
            let id = self.intern_closure_result(&probe, byte, regex);
            let m = self.is_match(id);

            Transition {
                no_break: id.with_tags(m || resolved_is_match, counts_or_seeds),
                with_break: id.with_tags(m || resolved_is_match, false),
                counting_mask: 0,
                pre_seeds: pre_seed_list.into_boxed_slice(),
                seeds: seed_list.into_boxed_slice(),
                counter_reset,
                phase_reset,
            }
        }
    }

    /// One pre-seed (value 0) per counter in `mask`, in counter order.
    fn pre_seeds_for(mask: u64) -> Vec<(CounterIdx, u32)> {
        let mut seeds = Vec::new();
        let mut mask = mask;
        while mask != 0 {
            let ci = mask.trailing_zeros();
            mask &= mask - 1;
            seeds.push((CounterIdx(ci as u8), 0));
        }
        seeds
    }

    /// Compute the `phase_reset` bitmask for a transition: every counter
    /// with a multi-byte body whose increment is not crossed
    /// (`counting_mask`) and that is not cleared as a whole anyway
    /// (`counter_reset`).
    fn compute_phase_reset(
        analysis: &Tier2Analysis,
        counting_mask: u64,
        counter_reset: u64,
    ) -> u64 {
        let mut mask: u64 = 0;
        for ci in 0..analysis.body_ranges.len() {
            let bit = 1u64 << ci;
            // A one-byte body has a single phase, handled by counter_reset.
            if !analysis.interior(ci).is_empty() && (counting_mask | counter_reset) & bit == 0 {
                mask |= bit;
            }
        }
        mask
    }

    /// Compute the `counter_reset` bitmask for a transition.
    ///
    /// For each counter NOT in `skip_mask`, check if the successor's NFA
    /// states contain any "body interior" states.  If they don't, the
    /// counter's instances should be cleared (the body was interrupted).
    fn compute_counter_reset(
        &self,
        analysis: &Tier2Analysis,
        successor_nfa_states: &[StateIdx],
        skip_mask: u64,
    ) -> u64 {
        let mut mask: u64 = 0;
        for ci in 0..analysis.body_ranges.len() {
            let interior = analysis.interior(ci);
            if (skip_mask >> ci) & 1 != 0 {
                continue;
            }
            // For L=1 counters, interior is empty, so body is never
            // "in progress" → always reset on non-CInc transitions.
            if interior.is_empty() {
                mask |= 1u64 << ci;
                continue;
            }
            // Check if any of the successor's NFA states are in the interior.
            let has_interior = successor_nfa_states
                .iter()
                .any(|s| interior.binary_search(&s.0).is_ok());
            if !has_interior {
                mask |= 1u64 << ci;
            }
        }
        mask
    }

    /// The DFA state to continue from when no counter instance is live:
    /// `id` without its counter-body NFA states.
    ///
    /// A thread can only sit inside a counter body while it owns a counter
    /// instance.  The DFA state nevertheless keeps listing a counter's
    /// body states for as long as the input matches the body, so once the
    /// last instance is gone those states are dead weight: they keep every
    /// transition on the counting slow path and keep the state different
    /// from the start state, which stops the prefilter from re-engaging.
    /// Dropping them leaves exactly the live threads; match flags and
    /// deferred assertions (parked threads outside any body) are kept.
    ///
    /// Returns `id` itself when there is nothing to drop, when dropping
    /// would leave no thread at all (the DEAD state is not worth entering:
    /// it recomputes its transition on every byte), or when the state cap
    /// leaves no room for the result.  Cached per state.
    ///
    /// Must only be called when [`settle_enabled`](Self::settle_enabled).
    #[inline]
    fn settled_state(&mut self, id: Tier2StateId, regex: &Regex) -> Tier2StateId {
        match self.settled.get(id.idx()) {
            Some(&s) if s != Tier2StateId::UNKNOWN => s,
            _ => self.compute_settled(id, regex),
        }
    }

    #[cold]
    fn compute_settled(&mut self, id: Tier2StateId, regex: &Regex) -> Tier2StateId {
        debug_assert!(self.settle_enabled);
        let target = self.without_body_states(id, regex).unwrap_or(id);
        if self.settled.len() < self.inner.states.len() {
            self.settled
                .resize(self.inner.states.len(), Tier2StateId::UNKNOWN);
        }
        self.settled[id.idx()] = target;
        // The result has no body states left, so it settles to itself.
        self.settled[target.idx()] = target;
        target
    }

    /// Intern the state `id` minus its counter-body NFA states.  `None`
    /// when `id` has none, when nothing would remain, or when the state
    /// cap is reached.
    fn without_body_states(&mut self, id: Tier2StateId, regex: &Regex) -> Option<Tier2StateId> {
        let body = &regex.counter_body_states;
        let state = &self.inner.states[id.idx()];
        if !state.nfa_states.iter().any(|s| body[s.idx()]) {
            return None;
        }
        let nfa: Vec<StateIdx> = state
            .nfa_states
            .iter()
            .copied()
            .filter(|s| !body[s.idx()])
            .collect();
        let deferred = state.deferred_asserts.clone();
        let (is_match, is_match_at_end, context) =
            (state.is_match, state.is_match_at_end, state.context);
        if nfa.is_empty() && deferred.is_empty() && !is_match && !is_match_at_end {
            return None;
        }
        self.intern_state(&nfa, &deferred, is_match, is_match_at_end, context, regex)
    }

    /// Whether dead counter-body states may be dropped for `regex`.
    ///
    /// In an assertion-free body a thread always owns a counter instance.
    /// This also holds for overlapping counters: their common terminal
    /// break cannot introduce any body states or seeds. A thread parked on
    /// a body assertion can have its instance cleared and later re-seeded.
    fn can_settle(regex: &Regex) -> bool {
        !regex.counter_body_states.is_empty()
            && !regex
                .states
                .iter()
                .zip(regex.counter_body_states.iter())
                .any(|(state, &in_body)| in_body && matches!(state, State::Assert { .. }))
    }

    fn intern_closure_result(
        &mut self,
        cr: &ClosureResult,
        byte: u8,
        regex: &Regex,
    ) -> Tier2StateId {
        if cr.nfa_states.is_empty()
            && cr.deferred_asserts.is_empty()
            && !cr.is_match
            && !cr.is_match_at_end
        {
            return Tier2StateId::DEAD;
        }
        self.intern_state(
            &cr.nfa_states,
            &cr.deferred_asserts,
            cr.is_match,
            cr.is_match_at_end,
            BoundaryContext::new(Some(byte)),
            regex,
        )
        .expect("DFA state cap reached although room was made")
    }

    fn is_match(&self, id: Tier2StateId) -> bool {
        !id.is_dead() && self.inner.states[id.idx()].is_match
    }

    fn clear(&mut self, memory: &mut DfaMemory, num_nfa_states: usize, stride: usize) {
        self.inner.clear();
        self.stride = stride;
        memory.clear(num_nfa_states);
        self.transitions.clear();
        self.start_seeds = Box::new([]);
        self.counters.clear();
        self.dead_transitions.clear();
        self.dead_transitions.resize_with(stride, Transition::empty);
        self.settled.clear();
        self.settle_enabled = false;
    }

    #[inline]
    pub(crate) fn prepare(
        &mut self,
        memory: &mut DfaMemory,
        regex: &Regex,
        analysis: &Tier2Analysis,
    ) {
        let id = regex.id;
        if self.inner.regex_id == id && self.inner.start_id != DfaStateId::DEAD {
            return;
        }
        self.clear(memory, regex.states.len(), regex.num_byte_classes);
        self.inner.regex_id = id;
        self.settle_enabled = Self::can_settle(regex);

        let cr = self.epsilon_closure(
            memory,
            std::iter::once(regex.start),
            regex,
            analysis,
            true,
            None,
            None,
            true,
        );
        self.inner.start_id = self
            .intern_state(
                &cr.nfa_states,
                &cr.deferred_asserts,
                cr.is_match,
                cr.is_match_at_end,
                BoundaryContext::new(None),
                regex,
            )
            .expect("an empty cache has room for the start state")
            .state();
        // Checked in release builds too: a matcher starts from the START
        // constant and compares against it without reading `start_id`, so
        // a start state anywhere else would give wrong answers silently.
        // This runs once per prepared regex, not per stream.
        assert_eq!(
            self.inner.start_id,
            DfaStateId::START,
            "the start state is the first state interned"
        );
        self.inner.start_is_match = self.inner.states[self.inner.start_id.idx()].is_match;
        self.inner.start_is_match_at_end =
            self.inner.states[self.inner.start_id.idx()].is_match_at_end;
        self.start_seeds = cr.seed_instances.iter().map(|&(c, _s)| (c, 0u32)).collect();

        // Initialize counter state (phases + metadata).
        self.counters.populate(&regex.counter_info);
    }
}

// ---------------------------------------------------------------------------
// ClosureResult
// ---------------------------------------------------------------------------

struct ClosureResult {
    nfa_states: Box<[StateIdx]>,
    deferred_asserts: Box<[StateIdx]>,
    is_match: bool,
    is_match_at_end: bool,
    encountered_cinc: bool,
    seed_instances: Box<[(CounterIdx, StateIdx)]>,
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Consume `byte` at NFA state `idx`.  Returns `(out, out_exit)` where
/// `out_exit` is `StateIdx::NONE` when there is no exit branch.
fn consume_byte(idx: StateIdx, byte: u8, regex: &Regex) -> Option<(StateIdx, StateIdx)> {
    match regex.states[idx] {
        State::Byte {
            byte: b,
            out,
            out_exit,
        } if byte == b => Some((out, out_exit)),
        State::ByteCI {
            byte: b,
            out,
            out_exit,
        } if byte_match_ci(byte, b) => Some((out, out_exit)),
        State::Wildcard { out, out_exit } => Some((out, out_exit)),
        State::ByteClassStatic {
            table,
            out,
            out_exit,
        } if table[byte as usize] => Some((out, out_exit)),
        State::ByteClassCustom {
            class,
            out,
            out_exit,
        } if regex.classes[class.idx()].contains(byte) => Some((out, out_exit)),
        State::ByteTable { table } => {
            let t = regex.byte_tables[table][byte];
            if t != StateIdx::NONE {
                Some((t, StateIdx::NONE))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Walk epsilon transitions from `start` and collect all reachable
/// consuming states.  Allocates its own scratch buffers.
fn consuming_states_from(start: StateIdx, states: &[State]) -> Vec<StateIdx> {
    let mut result = Vec::new();
    let mut stack = vec![start];
    let mut visited = vec![false; states.len()];
    consuming_states_walk(&mut stack, &mut visited, states, &mut result);
    result
}

/// Walk epsilon transitions from `start` and append reachable
/// consuming states to `result`, using `DfaMemory` scratch buffers
/// for the visited array and work stack.
///
/// Does NOT touch `memory.closure_result` — the caller may still
/// need it for the epsilon closure result built earlier.
/// Shared walk logic for `consuming_states_from`.
fn consuming_states_walk(
    stack: &mut Vec<StateIdx>,
    visited: &mut [bool],
    states: &[State],
    result: &mut Vec<StateIdx>,
) {
    while let Some(idx) = stack.pop() {
        let i = idx.idx();
        if visited[i] {
            continue;
        }
        visited[i] = true;
        match states[idx] {
            State::Split { out, out1 } => {
                stack.push(out1);
                stack.push(out);
            }
            State::Assert { out, .. } => stack.push(out),
            State::CounterInstance { out, .. } => stack.push(out),
            State::Byte { .. }
            | State::ByteCI { .. }
            | State::Wildcard { .. }
            | State::ByteClassStatic { .. }
            | State::ByteClassCustom { .. }
            | State::ByteTable { .. } => {
                result.push(idx);
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Tier 2 Matcher
// ---------------------------------------------------------------------------

/// A search can finish early with a match, or when an absolute-start
/// pattern has lost all its threads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SearchStatus {
    Searching,
    Matched,
    Exhausted,
}

impl SearchStatus {
    #[inline]
    fn from_match_flag(is_match: bool) -> Self {
        if is_match {
            Self::Matched
        } else {
            Self::Searching
        }
    }

    /// A completed search keeps its first result.
    #[inline]
    fn record_match(&mut self, matched: bool) {
        if matched && *self == Self::Searching {
            *self = Self::Matched;
        }
    }
}

/// Tier 2 DFA matcher with differential counters.
pub struct Tier2DfaMatcher<'a> {
    cache: &'a mut Tier2DfaCache,
    memory: &'a mut DfaMemory,
    regex: &'a Regex,
    analysis: &'a Tier2Analysis,
    /// The current state: an untagged index, or the DEAD tag alone.
    current: Tier2StateId,
    status: SearchStatus,
    has_live_counters: bool,
    /// Copy of [`Tier2DfaCache::settle_enabled`]: whether dead
    /// counter-body states may be dropped from the DFA state once no
    /// instance is live.
    settle: bool,
    prefilter: Prefilter,
}

impl<'a> Tier2DfaMatcher<'a> {
    #[inline]
    pub(crate) fn new(
        cache: &'a mut Tier2DfaCache,
        memory: &'a mut DfaMemory,
        regex: &'a Regex,
        analysis: &'a Tier2Analysis,
    ) -> Self {
        cache.counters.reset();
        cache.counters.ensure_phases();

        // Seed initial counter instances (only for patterns where the
        // start state directly reaches a CounterInstance via epsilons).
        // For most patterns (e.g. AWS keys), start_seeds is empty and
        // this block is skipped entirely — no counter work at all.
        let mut has_live = false;
        if !cache.start_seeds.is_empty() {
            let num_start_seeds = cache.start_seeds.len();
            for si in 0..num_start_seeds {
                let (counter, initial_value) = cache.start_seeds[si];
                cache.counters.seed(counter.idx(), initial_value);
            }
            has_live = cache.counters.has_live();
        }

        // The start state is state 0, also after the cache is emptied, so
        // the constant is the current state.  Converting `start_id` here
        // instead looks harmless but is a compare and a conditional move on
        // every stream (a plain id can be DEAD, a tagged one says so with a
        // tag), which showed on short streams: about 1% on a 35-byte line.
        debug_assert_eq!(cache.inner.start_id, DfaStateId::START);
        Tier2DfaMatcher {
            current: Tier2StateId::START,
            status: SearchStatus::from_match_flag(cache.inner.start_is_match),
            has_live_counters: has_live,
            settle: cache.settle_enabled,
            cache,
            memory,
            regex,
            analysis,
            prefilter: regex.prefilter,
        }
    }

    /// Slot lookup using byte-class compression (stride < 256).
    #[inline(always)]
    fn ensure_transition(&mut self, byte: u8) -> usize {
        let class = self.regex.byte_classes[byte as usize] as usize;
        let slot = self.current.idx() * self.cache.stride + class;
        if self.cache.transitions[slot].no_break == Tier2StateId::UNKNOWN {
            return self.populate_transition(byte, class);
        }
        slot
    }

    /// Cache miss: compute the transition out of the current state on
    /// `byte` (whose column in the table is `class`), store it and return
    /// its slot.  Kept out of line so the lookups stay small.
    #[cold]
    #[inline(never)]
    fn populate_transition(&mut self, byte: u8, class: usize) -> usize {
        // The current state gets a new id if the cache has to be emptied.
        self.current = self.cache.make_room(self.current);
        let from = self.current;
        let slot = self.current.idx() * self.cache.stride + class;
        let trans = self
            .cache
            .populate(self.memory, from, byte, self.regex, self.analysis);
        self.cache.transitions[slot] = trans;
        slot
    }

    /// Slot lookup using identity mapping (stride = 256, no byte-class indirection).
    #[inline(always)]
    fn ensure_transition_direct(&mut self, byte: u8) -> usize {
        let slot = self.current.idx() * 256 + byte as usize;
        if self.cache.transitions[slot].no_break == Tier2StateId::UNKNOWN {
            return self.populate_transition(byte, byte as usize);
        }
        slot
    }

    #[inline(always)]
    fn step(&mut self, byte: u8) {
        if self.current.is_dead() {
            self.step_from_dead(byte);
            return;
        }

        let slot = self.ensure_transition(byte);
        self.step_inner(slot)
    }

    #[inline(always)]
    fn step_direct(&mut self, byte: u8) {
        if self.current.is_dead() {
            self.step_from_dead(byte);
            return;
        }

        let slot = self.ensure_transition_direct(byte);
        self.step_inner(slot)
    }

    /// Follow cached untagged edges from the current state and return how
    /// many bytes of `input` they consumed.  Stops at the first byte whose
    /// edge carries a tag (missing, DEAD, match, counting or seeding);
    /// [`step`](Self::step) takes that one.
    ///
    /// Must only be called with no live counter instance and a current
    /// state other than DEAD.  The current state and the table live in
    /// locals for the whole run, so a byte costs one lookup and one test.
    #[inline(always)]
    fn idle_run<const DIRECT: bool>(&mut self, input: &[u8]) -> usize {
        debug_assert!(!self.has_live_counters && !self.current.is_dead());
        let table = &self.cache.transitions[..];
        let stride = if DIRECT { 256 } else { self.cache.stride };
        let classes = &self.regex.byte_classes;
        let column = |byte: u8| {
            if DIRECT {
                byte as usize
            } else {
                classes[byte as usize] as usize
            }
        };
        let mut current = self.current.0;
        let mut taken = 0;
        while taken < input.len() {
            let row = &table[current as usize * stride..][..stride];
            let next = row[column(input[taken])].no_break;
            if next.is_tagged() {
                break;
            }
            taken += 1;
            if next.0 == current {
                // The state does not change: the following bytes need no
                // lookup that depends on the previous one, only their own
                // entry in this row.  It is compared with the current state
                // in a register, as in `dead_run`, which says why that
                // matters.
                let rest = &input[taken..];
                taken += rest
                    .iter()
                    .position(|&byte| row[column(byte)].no_break.0 != current)
                    .unwrap_or(rest.len());
            } else {
                current = next.0;
            }
        }
        let current = Tier2StateId(current);
        self.current = current;
        taken
    }

    /// Apply the cached edge.
    #[inline(always)]
    fn step_inner(&mut self, slot: usize) {
        let t = &self.cache.transitions[slot];

        // Fast path: non-counting, no seeds, no live counters.
        if !t.no_break.has_effect() && !self.has_live_counters {
            self.current = t.no_break.as_current();
            self.status.record_match(t.no_break.is_match());
            return;
        }

        let mut any_can_break = false;

        // Apply pre_seeds BEFORE counter_increment.
        // These are resolved seeds (initial_value > 0) from Phase 1
        // deferred-assertion resolution on L=1 counter bodies.  The seed
        // must be visible to the increment so the counter can break on
        // this very transition.
        for (counter, initial_value) in &t.pre_seeds {
            let c_idx = counter.idx();
            if c_idx < self.cache.counters.meta.len() {
                self.cache.counters.seed(c_idx, *initial_value);
            }
        }

        // Process ALL counters that fire CInc on this transition.
        let mut mask = t.counting_mask;
        while mask != 0 {
            let c_idx = mask.trailing_zeros() as usize;
            mask &= mask - 1; // clear lowest set bit
            if c_idx < self.cache.counters.meta.len() && self.cache.counters.increment(c_idx) {
                any_can_break = true;
            }
        }

        // Select DFA successor and record match flags.
        if any_can_break {
            self.current = t.with_break.as_current();
            self.status.record_match(t.with_break.is_match());
        } else {
            self.current = t.no_break.as_current();
            self.status.record_match(t.no_break.is_match());
        }

        // Apply counter_reset: clear instances for counters whose body
        // was interrupted (no interior body NFA states in the successor).
        let mut mask = t.counter_reset;
        while mask != 0 {
            let c_idx = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            if c_idx < self.cache.counters.meta.len() {
                self.cache.counters.reset_counter(c_idx);
            }
        }

        // Apply phase_reset: clear the active phase of counters whose
        // increment this transition does not cross.  Must run before the
        // phase clock advances and before new instances are seeded into
        // that phase.
        let mut mask = t.phase_reset;
        while mask != 0 {
            let c_idx = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            if c_idx < self.cache.counters.meta.len() {
                self.cache.counters.reset_active_phase(c_idx);
            }
        }

        // Advance active_phase for EVERY counter on EVERY byte.
        // This keeps the phase clock synchronized with byte position.
        self.cache.counters.advance_all_phases();

        // Seed new counter instances.
        // Access seeds by index: each index yields a Copy tuple, so the
        // temporary borrow on self.cache.transitions ends before we
        // call seed_counter.
        for (counter, initial_value) in &t.seeds {
            let c_idx = counter.idx();
            if c_idx < self.cache.counters.meta.len() {
                self.cache.counters.seed(c_idx, *initial_value);
            }
        }

        // Update has_live_counters flag.
        self.has_live_counters = self.cache.counters.has_live();

        // With no live instance left, counter-body NFA states still
        // listed in the successor carry no thread: continue from the same
        // state without them, so that later transitions stop counting and
        // — back in the start state — the prefilter can re-engage.
        // The settled state keeps the match flags, so the answer at end of
        // input does not change.
        if self.settle && !self.has_live_counters && !self.current.is_dead() {
            self.current = self.cache.settled_state(self.current, self.regex);
        }
    }

    /// What standing on DEAD settles before any byte is looked at: no
    /// thread is left, so counter instances still listed go, and a regex
    /// anchored at the start of input has nothing more to find.  Returns
    /// whether the search ended.
    #[inline]
    fn clear_for_dead(&mut self) -> bool {
        debug_assert_eq!(self.current, Tier2StateId::DEAD);
        let anchored = self.regex.start_anchored;
        if anchored {
            self.status = SearchStatus::Exhausted;
        }
        if self.has_live_counters {
            self.cache.counters.reset();
            self.has_live_counters = false;
        }
        anchored
    }

    /// How many bytes at the start of `input` leave the matcher dead and
    /// seed nothing: they are tested against the DEAD row alone, without a
    /// step each.  Stops at the first byte whose transition is missing or
    /// does anything else; [`step_from_dead`](Self::step_from_dead) takes
    /// that one.
    ///
    /// Out of line for two reasons.  Inside the scanning loop, how tight
    /// this loop came out depended on the code around it (64 ms or 71 ms on
    /// a pattern that is dead for most of every line); and the compare in
    /// it is only fast while the value it compares with is not a constant
    /// to the compiler (see the comment in the body).
    #[inline(never)]
    fn dead_run<const DIRECT: bool>(&self, input: &[u8]) -> usize {
        let row = &self.cache.dead_transitions[..];
        let classes = &self.regex.byte_classes;
        // "Dead again, and neither a match nor a seed" is the entry that
        // equals the DEAD state with no other tag, which is the matcher's
        // current state here.  Each entry is compared with that value, read
        // into a register, and not with the constant `DEAD`, although
        // the two are equal.  Against the constant, the loop compares
        // memory with a 32-bit immediate; the CPU's front end delivers that
        // form more slowly (same instruction count, same branch and cache
        // misses, 2.6 times the undelivered uop slots on an i5-4460), which
        // cost 13% on a pattern that is dead for most of every line: 68 ms
        // against 60 ms.
        //
        // This only holds while the compiler cannot tell that `dead` is
        // that constant.  It cannot here because this function is not
        // inlined (`#[inline(never)]` above) and reads the value from the
        // matcher.  Inlined into a caller that has just tested for DEAD, or
        // given the value as a constant argument, the loop would go back to
        // the slow form, and nothing in CI would notice: the Callgrind gate
        // counts instructions, branches and cache misses, which are the
        // same either way.
        let dead = self.current;
        debug_assert_eq!(dead, Tier2StateId::DEAD);
        input
            .iter()
            .position(|&byte| {
                let column = if DIRECT {
                    byte as usize
                } else {
                    classes[byte as usize] as usize
                };
                row[column].no_break != dead
            })
            .unwrap_or(input.len())
    }

    fn step_from_dead(&mut self, byte: u8) {
        if self.status != SearchStatus::Searching {
            return;
        }
        if self.clear_for_dead() {
            return;
        }

        // The transition out of DEAD depends only on the byte (the
        // restart closure with that byte as predecessor), so it is cached
        // per byte class like any other transition.
        let class = if self.cache.stride == 256 {
            byte as usize
        } else {
            self.regex.byte_classes[byte as usize] as usize
        };
        if self.cache.dead_transitions[class].no_break == Tier2StateId::UNKNOWN {
            self.cache.make_room(Tier2StateId::DEAD);
            let trans = self.cache.populate(
                self.memory,
                Tier2StateId::DEAD,
                byte,
                self.regex,
                self.analysis,
            );
            self.cache.dead_transitions[class] = trans;
        }
        let trans = &self.cache.dead_transitions[class];
        self.current = trans.no_break.as_current();

        // Still dead and nothing seeded: no counter state to maintain.
        // The phase clock only matters relative to live instances.
        if trans.no_break.is_dead() && trans.seeds.is_empty() {
            return;
        }

        if trans.counting_mask == 0 && trans.no_break.is_match() {
            self.status = SearchStatus::Matched;
        }

        // Ensure counter phases are populated before counter ops.
        self.cache.counters.ensure_phases();

        // Advance active_phase for every counter (every byte ticks the clock).
        self.cache.counters.advance_all_phases();

        // Seed with phase-aware placement.
        for &(counter, initial_value) in trans.seeds.iter() {
            let c_idx = counter.idx();
            if c_idx < self.cache.counters.meta.len() {
                self.cache.counters.seed(c_idx, initial_value);
            }
        }
        self.has_live_counters = self.cache.counters.has_live();
    }

    #[inline(always)]
    pub fn chunk(&mut self, input: &[u8]) {
        if self.status != SearchStatus::Searching {
            return;
        }

        if let [byte] = input
            && self.current == Tier2StateId::START
            && !self.has_live_counters
            && !self.prefilter.accepts(*byte)
        {
            // Test a singleton in place instead of starting a SIMD scan.
            return;
        }

        // Prefilter-aware scanning: when the DFA is in the start state and
        // a prefilter is available, use memchr to skip ahead to the next
        // candidate byte instead of stepping byte-by-byte.  This is
        // re-engaged every time the DFA returns to the start state, not
        // just once at chunk entry.
        let prefilter = self.prefilter;
        crate::prefilter::with_prefilter!(prefilter, self.chunk_no_prefilter(input), |find| {
            self.chunk_prefilter(input, |bytes| {
                prefilter.find_in_prefix(bytes).or_else(|| find(bytes))
            })
        })
    }

    #[inline(always)]
    fn chunk_no_prefilter(&mut self, input: &[u8]) {
        if self.cache.stride == 256 {
            self.scan_without_prefilter::<true>(input);
        } else {
            self.scan_without_prefilter::<false>(input);
        }
    }

    #[inline(always)]
    fn scan_without_prefilter<const DIRECT: bool>(&mut self, mut input: &[u8]) {
        loop {
            if self.current.is_dead() {
                if input.is_empty() {
                    return;
                }
                // As the step out of DEAD would on this byte.
                if self.clear_for_dead() {
                    return;
                }
                let skip = self.dead_run::<DIRECT>(input);
                input = &input[skip..];
            } else if !self.has_live_counters {
                // With nothing live, almost every edge only moves the state.
                let taken = self.idle_run::<DIRECT>(input);
                input = &input[taken..];
            }
            let Some((&byte, rest)) = input.split_first() else {
                return;
            };
            if DIRECT {
                self.step_direct(byte)
            } else {
                self.step(byte)
            };
            if self.status != SearchStatus::Searching {
                return;
            }
            input = rest;
        }
    }

    /// Bulk scan with the prefilter. At [`Tier2StateId::START`] (state zero)
    /// with no live counters, the finder may skip to the next candidate.
    /// The cache interns the start state first and preserves its id on eviction.
    /// Keeping this loop out of line keeps its specializations out of the
    /// common chunk dispatcher.
    #[inline(never)]
    fn chunk_prefilter(&mut self, input: &[u8], finder: impl Fn(&[u8]) -> Option<usize>) {
        let mut i = 0;
        while i < input.len() {
            if self.status != SearchStatus::Searching {
                return;
            }
            // chunk() already checked an idle singleton against the prefilter.
            if self.current == Tier2StateId::START && !self.has_live_counters && input.len() > 1 {
                if let Some(offset) = finder(&input[i..]) {
                    i += offset;
                } else {
                    return;
                }
            }
            if self.cache.stride == 256 {
                self.step_direct(input[i]);
            } else {
                self.step(input[i]);
            }
            i += 1;
        }
    }

    #[inline]
    pub fn finish(self) -> bool {
        self.ismatch() || self.match_at_end()
    }

    /// Whether the stream matches if it ends here: a property of the
    /// current state.  Every successor is entered with its own flag, a
    /// settled state keeps the flag of the state it replaces, and the DEAD
    /// state accepts nothing, so the matcher does not track it per byte.
    #[inline]
    fn match_at_end(&self) -> bool {
        // A stream that ends where it started, as most short ones do,
        // needs no lookup: the start state's flag is kept with the cache.
        if self.current == Tier2StateId::START {
            return self.cache.inner.start_is_match_at_end;
        }
        !self.current.is_dead() && self.cache.inner.states[self.current.idx()].is_match_at_end
    }

    /// Whether a match has already been found without waiting for EOF.
    pub fn ismatch(&self) -> bool {
        self.status == SearchStatus::Matched
    }
}

impl fmt::Debug for Tier2DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Tier2DfaMatcher");
        s.field("current", &self.current.state());
        if !self.current.is_dead() {
            let state = &self.cache.inner.states[self.current.idx()];
            s.field("nfa_states", &state.nfa_states);
            s.field("is_match", &state.is_match);
            s.field("is_match_at_end", &state.is_match_at_end);
            s.field("context", &state.context);
        }
        s.field("status", &self.status)
            .field("match_at_end", &self.match_at_end())
            .field("has_live_counters", &self.has_live_counters)
            .field("settle", &self.settle);
        s.finish()
    }
}

impl fmt::Display for Tier2DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.current.is_dead() {
            write!(f, "DFA[T2] state=DEAD matched={}", self.ismatch())?;
            if self.status == SearchStatus::Exhausted {
                write!(f, " exhausted")?;
            }
            if self.has_live_counters {
                write!(f, " live_counters")?;
            }
            return Ok(());
        }
        let state = &self.cache.inner.states[self.current.idx()];
        write!(
            f,
            "DFA[T2] state={} nfa={{{}}} matched={}",
            self.current.0,
            state
                .nfa_states
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(","),
            self.ismatch(),
        )?;
        if state.is_match {
            write!(f, " is_match")?;
        }
        if self.match_at_end() {
            write!(f, " mae")?;
        }
        if !state.deferred_asserts.is_empty() {
            write!(f, " context=({})", state.context)?;
        }
        // Counter summary: show per-counter active phase info.
        if self.has_live_counters && !self.cache.counters.meta.is_empty() {
            write!(f, " counters=[")?;
            for (ci, meta) in self.cache.counters.meta.iter().enumerate() {
                if ci > 0 {
                    write!(f, ", ")?;
                }
                // Find the active phase's DiffCounter.
                let phase_idx = meta.phase_start + meta.active_phase;
                if phase_idx < self.cache.counters.phases.len() {
                    let dc = &self.cache.counters.phases[phase_idx];
                    if dc.count > 0 {
                        write!(
                            f,
                            "c{ci}: {} inst oldest={} youngest={}",
                            dc.count,
                            dc.oldest,
                            dc.youngest()
                        )?;
                    } else {
                        write!(f, "c{ci}: idle")?;
                    }
                } else {
                    write!(f, "c{ci}: idle")?;
                }
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MatcherMemory, Regex, RegexConfig};

    /// Helper: build a tier-2-eligible regex.  Disables unrolling so that
    /// even small bounded repeats use counters (tier 2), and raises the
    /// estimated-states limit for large patterns.
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

    /// Helper: run a single tier-2 match and return the result.
    fn match_tier2(mem: &mut MatcherMemory, re: &Regex, input: &[u8]) -> bool {
        let mut m = mem.matcher_for_tier(re, 2).unwrap();
        m.chunk(input);
        m.finish()
    }

    #[test]
    fn private_steps_preserve_terminal_results() {
        let re = build(r"^a{2,4}b");
        let mut memory = MatcherMemory::default();
        for (input, expected) in [
            (b"aab".as_slice(), SearchStatus::Matched),
            (b"xx".as_slice(), SearchStatus::Exhausted),
        ] {
            let crate::AnyMatcher::Tier2Dfa(mut m) =
                memory.matcher_for_tier(&re, 2).expect("eligible pattern")
            else {
                unreachable!("forced Tier 2")
            };
            m.chunk(input);
            assert_eq!(m.status, expected);
            // Bypass the public chunk guard, including a step from DEAD.
            for &byte in b"xxaab" {
                m.step(byte);
                assert_eq!(m.status, expected);
            }
            assert_eq!(m.finish(), expected == SearchStatus::Matched);
        }
    }

    // ── Cache eviction (Bug 64) ──

    /// Emptying a full cache drops every table indexed by state id,
    /// keeps the state the matcher stands on, and leaves the stream's
    /// counters alone.
    #[test]
    fn test_evict_clears_state_tables_and_keeps_counters() {
        let re = build("x.{2,4}y");
        assert!(re.tier2_eligible);
        let analysis = re.tier2_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        m.chunk(b"zzxab");
        assert!(m.has_live_counters);
        let current = m.current;
        let state = m.cache.inner.states[current.idx()].clone();
        let live = m.cache.counters.live_count;
        let stride = m.cache.stride;

        // Put ids into the two side tables, as matching would.
        let n = m.cache.inner.states.len();
        m.cache.settled.resize(n, Tier2StateId::UNKNOWN);
        m.cache.settled[0] = current;
        m.cache.dead_transitions[0].no_break = current;
        m.cache.dead_transitions[0].with_break = current;

        m.current = m.cache.evict(current);
        // Emptying the cache keeps the start state where matchers expect it.
        assert_eq!(m.cache.inner.start_id, DfaStateId::START);
        assert_eq!(
            Tier2StateId::from_state(m.cache.inner.start_id),
            Tier2StateId::START
        );
        assert_eq!(m.cache.inner.states[m.current.idx()], state);
        assert_eq!(m.cache.inner.states.len(), 2);
        assert!(m.cache.settled.is_empty());
        assert_eq!(m.cache.dead_transitions.len(), stride);
        assert!(
            m.cache
                .dead_transitions
                .iter()
                .all(|t| t.no_break == Tier2StateId::UNKNOWN)
        );
        assert_eq!(m.cache.transitions.len(), 2 * stride);
        assert!(
            m.cache
                .transitions
                .iter()
                .all(|t| t.no_break == Tier2StateId::UNKNOWN)
        );
        assert_eq!(m.cache.counters.live_count, live);
        assert_eq!(m.cache.evictions(), 1);

        // The stream goes on as if nothing had happened.
        m.chunk(b"y");
        assert!(m.finish());
    }

    // ── Dropping dead counter-body states ──

    /// Run `f` on a tier-2 matcher for `pattern` (compiled without
    /// unrolling) that has consumed `input`.
    fn with_matcher<R>(pattern: &str, input: &[u8], f: impl FnOnce(&Tier2DfaMatcher) -> R) -> R {
        let re = build(pattern);
        assert!(re.tier2_eligible, "`{pattern}` must be tier 2 eligible");
        let analysis = re.tier2_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        m.chunk(input);
        f(&m)
    }

    #[test]
    fn test_finish_on_the_start_state_uses_its_flag() {
        // A stream that ends on the start state takes its answer from the
        // flag kept with the cache, not from the state table.  The two
        // agree, whether the start state accepts at end of input or not.
        for (pattern, accepts) in [("foo.{0,5}bar|$", true), ("foo.{0,5}bar", false)] {
            for input in [b"".as_slice(), b"z", b"zz"] {
                with_matcher(pattern, input, |m| {
                    assert_eq!(m.current, Tier2StateId::START, "`{pattern}` on {input:?}");
                    assert_eq!(m.cache.inner.start_is_match_at_end, accepts);
                    assert_eq!(m.cache.inner.states[0].is_match_at_end, accepts);
                    assert_eq!(m.match_at_end(), accepts, "`{pattern}` on {input:?}");
                });
            }
            // Anywhere else the state's own flag answers.
            with_matcher(pattern, b"zfo", |m| {
                assert_ne!(m.current, Tier2StateId::START);
                let state = &m.cache.inner.states[m.current.idx()];
                assert_eq!(m.match_at_end(), state.is_match_at_end);
            });
        }
    }

    #[test]
    fn end_through_split_sets_exact_flag_without_parking() {
        for (pattern, input, accepted) in [
            (r"a{2,4}$(?:x)?", b"aa".as_slice(), true),
            (r"a{2,4}$(?:x)?", b"a", false),
            (r"a{2,4}$(?:\B|x)", b"aa", false),
            (r"a{2,4}$(?:\b|x)", b"aa", true),
        ] {
            with_matcher(pattern, input, |m| {
                let state = &m.cache.inner.states[m.current.idx()];
                assert!(state.deferred_asserts.is_empty(), "{pattern}");
                assert_eq!(m.match_at_end(), accepted, "{pattern} on {input:?}");
            });
        }
    }

    #[test]
    fn parked_word_boundary_shares_equivalent_contexts() {
        let re = build(r"\b[ab]{2,40}");
        let analysis = re.tier2_analysis.as_ref().expect("Tier 2 pattern");
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        let initial = m.current;
        for byte in b" \n\r-" {
            m.step(*byte);
            assert_eq!(m.current, initial, "nonword predecessor {byte}");
        }
    }

    /// True if the matcher sits in the start state with no live counter —
    /// the exact condition under which `chunk_prefilter` skips input.
    fn at_start(m: &Tier2DfaMatcher) -> bool {
        m.current.state() == m.cache.inner.start_id && !m.has_live_counters
    }

    /// True if the matcher's DFA state lists a counter-body NFA state.
    fn has_body_state(m: &Tier2DfaMatcher) -> bool {
        !m.current.is_dead()
            && m.cache.inner.states[m.current.idx()]
                .nfa_states
                .iter()
                .any(|s| m.regex.counter_body_states[s.idx()])
    }

    #[test]
    fn test_settle_after_gap_expires() {
        // While the gap is open the counter is live.
        with_matcher("foo.{0,5}bar", b"fooxxx", |m| {
            assert!(m.settle);
            assert!(m.has_live_counters);
            assert!(has_body_state(m));
            assert!(!at_start(m));
        });
        // One byte past the bound the last instance is gone, and the dead
        // body state must not keep the DFA away from the start state.
        with_matcher("foo.{0,5}bar", b"fooxxxxxx", |m| assert!(at_start(m)));
        with_matcher("foo.{0,5}bar", b"fooxxxxxxxxxxxxxxxx", |m| {
            assert!(at_start(m))
        });
        // Exactly at the bound the instance can still break.
        with_matcher("foo.{0,5}bar", b"fooxxxxx", |m| assert!(!at_start(m)));
    }

    #[test]
    fn test_settle_keeps_live_threads() {
        // The gap has expired but `ba` of the second anchor was consumed
        // in time.  The dead body state is dropped at once; the live
        // thread stays, so the state is not the start state yet.
        with_matcher("foo.{0,5}bar", b"fooxxxxba", |m| {
            assert!(!m.has_live_counters);
            assert!(!has_body_state(m));
            assert!(!at_start(m));
        });
        // The thread still completes the match...
        let re = build("foo.{0,5}bar");
        let mut mem = MatcherMemory::default();
        assert!(match_tier2(&mut mem, &re, b"fooxxxxbar"));
        // ...and once it dies the matcher is back at the start state.
        with_matcher("foo.{0,5}bar", b"fooxxxxbaz", |m| assert!(at_start(m)));
        // A new anchor prefix after expiry is an ordinary DFA state.
        with_matcher("foo.{0,5}bar", b"fooxxxxxxxxfo", |m| {
            assert!(!at_start(m));
            assert!(!has_body_state(m));
        });
    }

    #[test]
    fn test_settle_multi_byte_body() {
        // Dead interior body states of a two-byte body.
        with_matcher("x(ab){2,4}y", b"xababab", |m| assert!(m.has_live_counters));
        with_matcher("x(ab){2,4}y", b"xababababab", |m| assert!(at_start(m)));
        with_matcher("x(ab){2,4}y", b"xabababababababab", |m| {
            assert!(at_start(m))
        });
    }

    #[test]
    fn test_settle_is_chunking_independent() {
        let re = build("foo.{0,5}bar");
        let analysis = re.tier2_analysis.as_ref().unwrap();
        let input = b"fooxxxxxxxxfoxfooxbaxxxxxxxxxxfoo";
        for split in 0..=input.len() {
            let mut memory = DfaMemory::default();
            let mut cache = Tier2DfaCache::new();
            cache.prepare(&mut memory, &re, analysis);
            let mut whole = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
            whole.chunk(input);
            let expected = (whole.current, whole.has_live_counters, whole.match_at_end());

            let mut memory = DfaMemory::default();
            let mut cache = Tier2DfaCache::new();
            cache.prepare(&mut memory, &re, analysis);
            let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
            m.chunk(&input[..split]);
            m.chunk(&input[split..]);
            assert_eq!(
                (m.current, m.has_live_counters, m.match_at_end()),
                expected,
                "split at {split}"
            );
        }
    }

    #[test]
    fn test_state_id_tags_do_not_disturb_the_state() {
        // The tags are single bits, apart from each other and from the
        // index bits.
        let tags = [
            Tier2StateId::UNKNOWN.0,
            Tier2StateId::MATCH_TAG,
            Tier2StateId::DEAD_TAG,
            Tier2StateId::EFFECT_TAG,
        ];
        for (i, &tag) in tags.iter().enumerate() {
            assert_eq!(tag.count_ones(), 1);
            assert_eq!(tag & Tier2StateId::MAX, 0);
            assert!(tag > Tier2StateId::MAX);
            for &other in &tags[..i] {
                assert_ne!(tag, other);
            }
        }
        assert_eq!(TIER2_MAX_STATES, Tier2StateId::MAX as usize + 1);

        // A state with any tags of an edge: the tags read back, and both
        // ways of dropping them give the state.
        for index in [0, 1, 0x1234, Tier2StateId::MAX] {
            let plain = DfaStateId(index);
            let id = Tier2StateId::from_state(plain);
            assert!(!id.is_tagged());
            assert_eq!(id.idx(), index as usize);
            for is_match in [false, true] {
                for effect in [false, true] {
                    let edge = id.with_tags(is_match, effect);
                    assert_eq!(edge.is_match(), is_match);
                    assert_eq!(edge.has_effect(), effect);
                    assert!(!edge.is_dead());
                    assert_eq!(edge.is_tagged(), is_match || effect);
                    assert_ne!(edge, Tier2StateId::UNKNOWN);
                    assert_eq!(edge.as_current(), id);
                    assert_eq!(edge.state(), plain);
                }
            }
        }
        assert_eq!(
            Tier2StateId::from_state(DfaStateId::START),
            Tier2StateId::START
        );

        // The DEAD state keeps its tag as a current state: it is what says
        // that there is no row to index.
        let dead = Tier2StateId::from_state(DfaStateId::DEAD);
        assert_eq!(dead, Tier2StateId::DEAD);
        assert!(dead.is_dead() && dead.is_tagged());
        for is_match in [false, true] {
            for effect in [false, true] {
                let edge = dead.with_tags(is_match, effect);
                assert!(edge.is_dead());
                assert_eq!(edge.is_match(), is_match);
                assert_eq!(edge.has_effect(), effect);
                assert_eq!(edge.as_current(), Tier2StateId::DEAD);
                assert_eq!(edge.state(), DfaStateId::DEAD);
            }
        }
    }

    #[test]
    fn test_idle_run_stops_at_every_tagged_entry() {
        // The idle run follows untagged entries, and its self-loop skip
        // compares whole entries with the current state.  A self-loop that
        // also matches, counts or seeds must end both, like an entry that
        // is missing or leads to DEAD.  The row is written by hand: one
        // plain self-loop and one of each tagged kind.
        for classes in [false, true] {
            let re = build_columns("foo.{0,5}bar", classes);
            let analysis = re.tier2_analysis.as_ref().unwrap();
            let mut memory = DfaMemory::default();
            let mut cache = Tier2DfaCache::new();
            cache.prepare(&mut memory, &re, analysis);
            // Six bytes with a column each.
            let mut bytes: Vec<u8> = Vec::new();
            let mut columns: Vec<usize> = Vec::new();
            for byte in 0..=255u8 {
                let column = if classes {
                    re.byte_classes[byte as usize] as usize
                } else {
                    byte as usize
                };
                if !columns.contains(&column) {
                    bytes.push(byte);
                    columns.push(column);
                }
            }
            assert!(bytes.len() >= 6, "`foo.{{0,5}}bar` has six byte classes");
            let [plain, other, matching, effect, dead, unknown] = bytes[..6] else {
                unreachable!()
            };
            let start = Tier2StateId::START;
            let next = cache
                .intern_state(&[], &[], false, true, BoundaryContext(0), &re)
                .unwrap();
            assert_ne!(next, start);
            let mut set = |byte: u8, to: Tier2StateId| {
                let column = columns[bytes.iter().position(|&b| b == byte).unwrap()];
                cache.transitions[start.idx() * cache.stride + column] = Transition {
                    no_break: to,
                    with_break: to,
                    ..Transition::empty()
                };
            };
            set(plain, start);
            set(other, next);
            set(matching, start.with_tags(true, false));
            set(effect, start.with_tags(false, true));
            set(dead, Tier2StateId::DEAD);
            assert_eq!(
                cache.transitions[start.idx() * cache.stride + columns[5]].no_break,
                Tier2StateId::UNKNOWN
            );

            let mut run = |input: &[u8]| {
                let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
                assert!(!m.has_live_counters);
                let taken = if classes {
                    m.idle_run::<false>(input)
                } else {
                    m.idle_run::<true>(input)
                };
                (taken, m.current)
            };
            // Plain self-loops are skipped to the end of the input.
            assert_eq!(run(&[plain; 9]), (9, start));
            // Each tagged entry ends the run where it stands, first in the
            // input, right after one self-loop, and inside the skip.
            for stop in [matching, effect, dead, unknown] {
                for before in [0, 1, 7] {
                    let mut input = vec![plain; before];
                    input.extend_from_slice(&[stop, plain, plain]);
                    assert_eq!(
                        run(&input),
                        (before, start),
                        "byte {stop} after {before} self-loops (classes: {classes})"
                    );
                }
            }
            // An untagged entry to another state ends the skip and is
            // followed.
            assert_eq!(run(&[plain, plain, plain, other]), (4, next));
        }
    }

    /// Compile without unrolling, with byte classes for every pattern or
    /// for none: the two ways the scanning loop finds a column.
    fn build_columns(pattern: &str, classes: bool) -> Regex {
        let re = Regex::with_config(
            pattern,
            RegexConfig {
                max_unroll_states: 0,
                byte_class_min_states: if classes { 0 } else { usize::MAX },
                ..Default::default()
            },
        )
        .unwrap();
        assert!(re.tier2_eligible, "`{pattern}` must be tier 2 eligible");
        assert_eq!(re.num_byte_classes < 256, classes, "`{pattern}`");
        re
    }

    /// Feed `input` in chunks of `size` and return the answer with the
    /// matcher's final state.
    fn feed(re: &Regex, input: &[u8], size: usize) -> (bool, DfaStateId, bool, SearchStatus) {
        let analysis = re.tier2_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, re, analysis);
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, re, analysis);
        for chunk in input.chunks(size.max(1)) {
            m.chunk(chunk);
        }
        let state = (m.current, m.has_live_counters, m.status);
        (m.finish(), state.0.state(), state.1, state.2)
    }

    /// Every way of splitting `input` into equal chunks gives the answer
    /// of the `regex` crate and ends on the state of the unsplit scan.
    fn assert_chunking_agrees(pattern: &str, inputs: &[&[u8]]) {
        let oracle = regex::bytes::Regex::new(&format!("(?s-u){pattern}")).unwrap();
        for classes in [false, true] {
            let re = build_columns(pattern, classes);
            assert!(
                matches!(re.prefilter, Prefilter::None),
                "`{pattern}` must scan without a prefilter"
            );
            for &input in inputs {
                let whole = feed(&re, input, input.len());
                assert_eq!(
                    whole.0,
                    oracle.is_match(input),
                    "`{pattern}` on {input:?} (classes: {classes})"
                );
                for size in [1, 2, 3, 5, 7] {
                    assert_eq!(
                        feed(&re, input, size),
                        whole,
                        "`{pattern}` on {input:?} in chunks of {size} (classes: {classes})"
                    );
                }
            }
        }
    }

    #[test]
    fn test_dead_row_skip_restarts_where_the_row_says() {
        // The matcher is DEAD from the first `z` to the next line start;
        // those bytes are skipped against the DEAD row.  The byte that
        // restarts a match attempt must not be skipped, in whichever chunk
        // it falls.
        let pattern = "(?m:^)foo.{0,5}bar";
        assert_chunking_agrees(
            pattern,
            &[
                b"zzzzzzzz\nfooxbar",
                b"zzzzzzzzfooxbar",
                b"zzzz\nzz\nfooxxxxxxbar",
                b"zz\nfozz\nfoobar",
                b"zzzzzzzzzzzz",
                b"zzzz\n",
                b"\n\n\nfoobar",
            ],
        );
        for classes in [false, true] {
            let re = build_columns(pattern, classes);
            let analysis = re.tier2_analysis.as_ref().unwrap();
            let mut memory = DfaMemory::default();
            let mut cache = Tier2DfaCache::new();
            cache.prepare(&mut memory, &re, analysis);
            let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
            m.chunk(b"zzzzzzzz");
            assert_eq!(m.current, Tier2StateId::DEAD);
            assert_eq!(m.status, SearchStatus::Searching);
            // The skip is what the row offers for `z` and not for LF.
            let column = |byte: u8| {
                if classes {
                    re.byte_classes[byte as usize] as usize
                } else {
                    byte as usize
                }
            };
            assert_eq!(
                m.cache.dead_transitions[column(b'z')].no_break,
                Tier2StateId::DEAD
            );
            m.chunk(b"zzzz\nfoo");
            assert_ne!(
                m.cache.dead_transitions[column(b'\n')].no_break,
                Tier2StateId::DEAD
            );
            assert_ne!(m.current, Tier2StateId::DEAD);
            m.chunk(b"bar");
            assert!(m.finish());

            // With every column of the DEAD row computed, a skip that read
            // the wrong column would jump over the line start.  The row is
            // filled one step at a time, which does not go through the
            // skip.
            for byte in 0..=255 {
                let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
                m.chunk(b"z");
                assert_eq!(m.current, Tier2StateId::DEAD);
                m.step(byte);
            }
            assert!(
                cache
                    .dead_transitions
                    .iter()
                    .all(|t| t.no_break != Tier2StateId::UNKNOWN)
            );
            for (input, expected) in [
                (b"zzzz\nfoobar".as_slice(), true),
                (b"zzzzfoobar", false),
                (b"zzbzz\nzfoobar", false),
            ] {
                let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
                m.chunk(input);
                assert_eq!(m.finish(), expected, "{input:?} (classes: {classes})");
            }
        }
    }

    #[test]
    fn test_dead_with_live_counters_drops_them() {
        // No thread is left on DEAD, so instances still listed are stale.
        // An anchored regex ends its search there; any other goes on
        // without them.  Both scanning paths go through the same code.
        for (pattern, anchored) in [
            ("^foo.{0,5}bar", true),
            ("(?m:^)foo.{0,5}bar", false),
            ("^foo.{0,5}bar|(?m:^)qux", false),
        ] {
            for classes in [false, true] {
                let re = build_columns(pattern, classes);
                assert_eq!(re.start_anchored, anchored, "`{pattern}`");
                let analysis = re.tier2_analysis.as_ref().unwrap();
                let mut memory = DfaMemory::default();
                let mut cache = Tier2DfaCache::new();
                cache.prepare(&mut memory, &re, analysis);
                let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
                m.current = Tier2StateId::DEAD;
                m.cache.counters.seed(0, 0);
                m.has_live_counters = true;
                // An empty chunk looks at no byte and settles nothing.
                m.chunk(b"");
                assert!(m.has_live_counters);
                assert_eq!(m.status, SearchStatus::Searching);
                m.chunk(b"zz");
                assert!(!m.has_live_counters, "`{pattern}`");
                assert_eq!(m.cache.counters.live_count, 0, "`{pattern}`");
                assert_eq!(m.current, Tier2StateId::DEAD);
                let expected = if anchored {
                    SearchStatus::Exhausted
                } else {
                    SearchStatus::Searching
                };
                assert_eq!(m.status, expected, "`{pattern}` (classes: {classes})");
                assert!(!m.finish());
            }
        }
    }

    #[test]
    fn test_idle_run_follows_chains_across_chunks() {
        // Before the gap, the literal is a chain of idle edges that are
        // not self-loops, and a failed attempt falls back along it.  The
        // run has to hand the state it reached back at every chunk end.
        assert_chunking_agrees(
            r"\babcde.{0,5}f",
            &[
                b" abcdeXXf",
                b" abcabcdeXf",
                b" abcdxabcd abcde......f",
                b"abcab abcdeabcdef",
                b" abcd",
                b"abcde",
                b" abcde.....f",
            ],
        );
    }

    #[test]
    fn test_settle_with_position_dependent_start() {
        // `^` makes the start state valid only at offset 0.  After the
        // gap of the second branch expires the dead body state is dropped,
        // but the result is not — and must not be — the start state.
        let pattern = "^foo[a-e]{0,5}bar|qux[0-9]{0,5}z";
        with_matcher(pattern, b"qux1234567890", |m| {
            assert!(m.settle);
            assert!(!m.has_live_counters);
            assert!(!has_body_state(m));
            assert_ne!(m.current.state(), m.cache.inner.start_id);
        });
        let re = build(pattern);
        let mut mem = MatcherMemory::default();
        assert!(!match_tier2(&mut mem, &re, b"qux1234567890foobar"));
        assert!(match_tier2(&mut mem, &re, b"qux1234567890qux12z"));
        assert!(match_tier2(&mut mem, &re, b"fooabcbar"));
    }

    #[test]
    fn test_settle_keeps_deferred_assertions_outside_bodies() {
        // A leading `\b` is re-parked in the DFA state on every byte.  It
        // is not part of a counter body, so it survives when the dead
        // body state is dropped.
        let pattern = "\\bfoo.{0,5}bar";
        with_matcher(pattern, b"fooxxxxxxxxxx", |m| {
            assert!(m.settle);
            assert!(!m.has_live_counters);
            assert!(!has_body_state(m));
            let state = &m.cache.inner.states[m.current.idx()];
            assert!(!state.deferred_asserts.is_empty());
        });
        let re = build(pattern);
        let mut mem = MatcherMemory::default();
        assert!(match_tier2(&mut mem, &re, b"fooxxxxxxxxxx foo bar"));
        assert!(!match_tier2(&mut mem, &re, b"fooxxxxxxxxxxfoo bar"));
        assert!(!match_tier2(&mut mem, &re, b"fooxxxxxxbar"));
    }

    #[test]
    fn test_settle_never_produces_dead_state() {
        // A state holding nothing but a dead body state has no thread at
        // all.  It is left as it is rather than turned into DEAD: the
        // ordinary transitions take care of it.
        let re = build("^foo.{0,5}bar");
        let analysis = re.tier2_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        assert!(cache.settle_enabled);
        let body: Vec<StateIdx> = (0..re.states.len() as u32)
            .map(StateIdx)
            .filter(|s| re.counter_body_states[s.idx()] && consume_byte(*s, b'x', &re).is_some())
            .collect();
        assert_eq!(body.len(), 1);
        let body_only = cache
            .intern_state(&body, &[], false, false, BoundaryContext(0), &re)
            .unwrap();
        assert_eq!(cache.settled_state(body_only, &re), body_only);
        // With a match-at-end flag there is still something to report, so
        // the body state is dropped and the flag kept.
        let flagged = cache
            .intern_state(&body, &[], false, true, BoundaryContext(0), &re)
            .unwrap();
        let settled = cache.settled_state(flagged, &re);
        assert_ne!(settled, flagged);
        let state = &cache.inner.states[settled.idx()];
        assert!(state.nfa_states.is_empty() && state.is_match_at_end);
    }

    #[test]
    fn test_anchored_pattern_after_gap_expires() {
        // Anchored pattern: after the gap expires and the last live thread
        // dies, nothing can match any more.  The matcher ends up DEAD and
        // stops scanning without populating restart transitions.
        with_matcher("^foo.{0,5}bar", b"fooxxxxxxxxxx", |m| {
            assert!(m.settle);
            assert!(!m.has_live_counters);
            assert_eq!(m.current, Tier2StateId::DEAD);
            assert_eq!(m.status, SearchStatus::Exhausted);
            let populated = m
                .cache
                .dead_transitions
                .iter()
                .filter(|t| t.no_break != Tier2StateId::UNKNOWN)
                .count();
            assert_eq!(populated, 0, "an absolute start cannot restart");
        });
        let re = build("^foo.{0,5}bar");
        let mut mem = MatcherMemory::default();
        assert!(!match_tier2(&mut mem, &re, b"fooxxxxxxxxxxbar"));
        assert!(!match_tier2(&mut mem, &re, b"fooxxxxxxxxxxfooxbar"));
        assert!(match_tier2(&mut mem, &re, b"fooxxxxxbar"));
    }

    #[test]
    fn test_dead_transitions_are_cached_and_cleared() {
        // Leaving the DEAD state must work through the cache as well: an
        // unanchored branch restarts from it.
        let first = build(r"^foo.{0,5}bar|(?m:^qux)");
        let second = build("^a.{0,5}b");
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        let analysis = first.tier2_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &first, analysis);
        assert_eq!(cache.dead_transitions.len(), cache.stride);
        for _ in 0..2 {
            let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &first, analysis);
            m.chunk(b"zzzzzzzzqux");
            assert_eq!(m.current, Tier2StateId::DEAD);
            m.chunk(b"\nqux");
            assert!(m.finish(), "line-start branch must restart at LF");
        }
        assert!(
            cache
                .dead_transitions
                .iter()
                .any(|t| t.no_break != Tier2StateId::UNKNOWN)
        );

        let analysis = second.tier2_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &second, analysis);
        assert_eq!(cache.dead_transitions.len(), cache.stride);
        assert!(
            cache
                .dead_transitions
                .iter()
                .all(|t| t.no_break == Tier2StateId::UNKNOWN)
        );
    }

    #[test]
    fn test_settle_after_overlapping_counters_expire() {
        let pattern = "x(?:[ab]{2,4}|[bc]{2,4})$";
        with_matcher(pattern, b"xbbbbb", |m| {
            assert!(m.regex.tier2_overlap_proven);
            assert!(matches!(m.regex.prefilter, Prefilter::Memchr1(b'x')));
            assert!(at_start(m), "expiry should re-engage the x prefilter");
        });

        let re = build(pattern);
        let mut memory = MatcherMemory::default();
        for (input, expected) in [
            (b"xbbbb".as_slice(), true),
            (b"xbbbbb", false),
            (b"xbbbbbxbb", true),
        ] {
            for chunk_size in [input.len(), 1] {
                let mut matcher = memory.matcher_for_tier(&re, 2).expect("Tier 2 overlap");
                for chunk in input.chunks(chunk_size) {
                    matcher.chunk(chunk);
                }
                assert_eq!(
                    matcher.finish(),
                    expected,
                    "{input:?}, chunks of {chunk_size}"
                );
            }
        }
    }

    #[test]
    fn test_settle_disabled_for_assertions_inside_bodies() {
        // Instance liveness is not exact while a thread is parked on a
        // deferred assertion inside the body.
        let pattern = "x(\\Ba){2,6}y";
        with_matcher(pattern, b"xa xaaaaaaaaa xa a", |m| {
            assert!(!m.settle);
            assert!(m.cache.settled.is_empty());
        });
    }

    #[test]
    fn test_settled_state_is_cached_and_idempotent() {
        let re = build("foo.{0,5}bar");
        let analysis = re.tier2_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        // The matcher starts from the constant without reading this id.
        assert_eq!(cache.inner.start_id, DfaStateId::START);
        let start = Tier2StateId::from_state(cache.inner.start_id);
        assert_eq!(start, Tier2StateId::START);
        // Nothing is computed until a matcher asks.
        assert!(cache.settled.is_empty());

        // The start state has no body state: it settles to itself.
        assert_eq!(cache.settled_state(start, &re), start);
        assert_eq!(cache.settled[start.idx()], start);

        // Reach the state "start + body state" by running into the gap.
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        m.chunk(b"fooxx");
        let in_gap = m.current;
        assert_ne!(in_gap, start);
        let states_before = cache.inner.states.len();

        // The bytes after `foo` still allow `b`, so the in-gap state holds
        // a third thread; without its body state it is a new state.
        let settled = cache.settled_state(in_gap, &re);
        assert_ne!(settled, in_gap);
        assert_eq!(cache.settled[in_gap.idx()], settled);
        // The result has no body state left and settles to itself.
        assert_eq!(cache.settled_state(settled, &re), settled);
        // Asking again neither recomputes nor interns anything.
        let states_after = cache.inner.states.len();
        assert!(states_after <= states_before + 1);
        assert_eq!(cache.settled_state(in_gap, &re), settled);
        assert_eq!(cache.inner.states.len(), states_after);
    }

    #[test]
    fn test_settled_table_cleared_when_cache_switches_regex() {
        // The table is indexed by DFA state id, which means nothing for
        // another regex.
        let first = build("foo.{0,5}bar");
        let second = build("fooxxxxxxxxy|a.{0,5}b");
        assert!(second.tier2_eligible);
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();

        let analysis = first.tier2_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &first, analysis);
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &first, analysis);
        m.chunk(b"fooxxxxxxxxxxxxqq");
        assert!(!m.finish());
        assert!(!cache.settled.is_empty());
        assert!(cache.settle_enabled);

        let analysis = second.tier2_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &second, analysis);
        assert!(cache.settled.is_empty());
        for (input, expected) in [
            (&b"fooxxxxxxxxy"[..], true),
            (b"axxxxxxxfooxxxxxxxxy", true),
            (b"axxxxxxxb", false),
            (b"axxxxxxxfooaxb", true),
        ] {
            let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &second, analysis);
            m.chunk(input);
            assert_eq!(m.finish(), expected, "{:?}", String::from_utf8_lossy(input));
        }
    }

    #[test]
    fn test_settle_preserves_match_at_end() {
        // The gap runs into `$`: at the bound the stream may end in a
        // match; one byte later it may not, and the matcher is at start.
        with_matcher("foo.{0,5}$", b"fooxxxxx", |m| assert!(m.match_at_end()));
        with_matcher("foo.{0,5}$", b"fooxxxxxx", |m| {
            assert!(!m.match_at_end());
            assert!(at_start(m));
        });
        let re = build("foo.{0,5}$");
        let mut mem = MatcherMemory::default();
        assert!(match_tier2(&mut mem, &re, b"fooxxxxx"));
        assert!(!match_tier2(&mut mem, &re, b"fooxxxxxx"));
        assert!(match_tier2(&mut mem, &re, b"fooxxxxxxfoo"));
    }

    // ── Ring buffer reuse across matches ──

    #[test]
    fn test_prepare_ring_reuse_same_regex() {
        let re = build(".{1,100}a");
        let mut mem = MatcherMemory::default();

        // First match — allocates phases and ring buffers.
        assert!(match_tier2(&mut mem, &re, b"xxxxa"));

        // Access the cache to check ring capacity was retained.
        let cache = mem.tier2_cache.as_ref().unwrap();
        assert!(
            !cache.counters.phases.is_empty(),
            "phases should be retained after first match"
        );
        let cap_after_first = cache.counters.phases[0].ring.len();
        assert!(cap_after_first > 0, "ring should be allocated");

        // Second match — reuses ring buffers, no reallocation.
        assert!(match_tier2(&mut mem, &re, b"ya"));

        let cache = mem.tier2_cache.as_ref().unwrap();
        assert_eq!(
            cache.counters.phases[0].ring.len(),
            cap_after_first,
            "ring capacity should be unchanged across matches"
        );
    }

    #[test]
    fn test_prepare_ring_cleared_between_matches() {
        let re = build(".{1,100}a");
        let mut mem = MatcherMemory::default();

        // Match with many instances seeded.
        assert!(match_tier2(&mut mem, &re, b"xxxxxxxxxa"));

        // Counter state should be clean for next match — a short input
        // that only seeds 1 instance should still work.
        assert!(match_tier2(&mut mem, &re, b"xa"));

        // No match.
        assert!(!match_tier2(&mut mem, &re, b"xxx"));
    }

    #[test]
    fn test_prepare_different_regex_reallocates() {
        let re1 = build(".{1,100}a");
        let re2 = build("[a-z]{1,50}x");
        let mut mem = MatcherMemory::default();

        // Warm up with re1.
        assert!(match_tier2(&mut mem, &re1, b"xxxxa"));
        let phases_after_re1 = mem.tier2_cache.as_ref().unwrap().counters.phases.len();
        assert!(phases_after_re1 > 0);

        // Switch to re2 — should clear and reallocate.
        assert!(match_tier2(&mut mem, &re2, b"abcx"));

        // Phases may have a different count depending on re2's counters.
        let phases_after_re2 = mem.tier2_cache.as_ref().unwrap().counters.phases.len();
        assert!(phases_after_re2 > 0);

        // Switch back to re1 — clear again.
        assert!(match_tier2(&mut mem, &re1, b"za"));
    }

    // ── Ring buffer growth for unbounded counters ──

    #[test]
    fn test_prepare_ring_grows_for_large_input() {
        let re = build(".{1,100}a");
        let mut mem = MatcherMemory::default();

        // Short input.
        assert!(match_tier2(&mut mem, &re, b"xxxa"));

        // Long input — ring grows to accommodate many live instances.
        let mut long_input = vec![b'x'; 8000];
        long_input.push(b'a');
        assert!(match_tier2(&mut mem, &re, &long_input));
    }

    // ── Ring buffer growth for unbounded counters ──

    #[test]
    fn test_prepare_ring_grows_for_unbounded() {
        // a{2,}b — unbounded counter, ring starts at the initial cap
        // and must grow for long inputs.
        let re = build("a{2,}b");
        let mut mem = MatcherMemory::default();

        assert!(match_tier2(&mut mem, &re, b"aab"));
        assert!(!match_tier2(&mut mem, &re, b"ab"));

        let mut long_input = vec![b'a'; 8000];
        long_input.push(b'b');
        assert!(match_tier2(&mut mem, &re, &long_input));
    }

    // ── Correctness across reused matches ──

    #[test]
    fn test_prepare_no_stale_state() {
        let re = build(".{3,5}x");
        let mut mem = MatcherMemory::default();

        // Match: 4 dots then x.
        assert!(match_tier2(&mut mem, &re, b"aaaax"));

        // No match: only 2 dots (below min=3).
        assert!(!match_tier2(&mut mem, &re, b"aax"));

        // Match again: exactly 3.
        assert!(match_tier2(&mut mem, &re, b"bbbx"));

        // No match: x without enough prefix.
        assert!(!match_tier2(&mut mem, &re, b"x"));
    }

    #[test]
    fn test_prepare_no_stale_state_small_bound() {
        // a{2,5}b with unroll disabled → tier 2 counter.
        let re = build("a{2,5}b");
        let mut mem = MatcherMemory::default();

        assert!(match_tier2(&mut mem, &re, b"aab"));
        assert!(match_tier2(&mut mem, &re, b"aaaaab"));
        assert!(!match_tier2(&mut mem, &re, b"ab"));
        assert!(!match_tier2(&mut mem, &re, b"aaa"));
        // Reuse after no-match.
        assert!(match_tier2(&mut mem, &re, b"aaab"));
    }

    // ── CounterState unit tests ──────────────────────────────────────

    /// Helper: create a CounterState from `(min, max, body_byte_length)`
    /// tuples for test convenience.
    fn make_counter_state(specs: &[(usize, usize, usize)]) -> CounterState {
        let info: Vec<crate::CounterInfo> = specs
            .iter()
            .enumerate()
            .map(|(i, &(min, max, body_byte_length))| crate::CounterInfo {
                index: i,
                min,
                max,
                body_byte_length,
            })
            .collect();
        let mut cs = CounterState::new();
        cs.populate(&info);
        cs
    }

    // ── populate / structure ──

    #[test]
    fn test_counter_state_populate_empty() {
        let cs = make_counter_state(&[]);
        assert_eq!(cs.meta.len(), 0);
        assert_eq!(cs.phases.len(), 0);
        assert!(!cs.has_live());
    }

    #[test]
    fn test_counter_state_populate_single_counter() {
        // (min=2, max=5, body_len=1) → 1 phase
        let cs = make_counter_state(&[(2, 5, 1)]);
        assert_eq!(cs.meta.len(), 1);
        assert_eq!(cs.meta[0].min, 2);
        assert_eq!(cs.meta[0].max, 5);
        assert_eq!(cs.meta[0].num_phases, 1);
        assert_eq!(cs.meta[0].phase_start, 0);
        assert_eq!(cs.phases.len(), 1);
    }

    #[test]
    fn test_counter_state_populate_multi_phase() {
        // body_len=3 → 3 phases
        let cs = make_counter_state(&[(1, 10, 3)]);
        assert_eq!(cs.meta.len(), 1);
        assert_eq!(cs.meta[0].num_phases, 3);
        assert_eq!(cs.phases.len(), 3);
    }

    #[test]
    fn test_counter_state_populate_multiple_counters() {
        // Two counters with different body lengths.
        let cs = make_counter_state(&[(1, 5, 1), (2, 10, 2)]);
        assert_eq!(cs.meta.len(), 2);
        assert_eq!(cs.phases.len(), 3); // 1 + 2
        assert_eq!(cs.meta[0].phase_start, 0);
        assert_eq!(cs.meta[0].num_phases, 1);
        assert_eq!(cs.meta[1].phase_start, 1);
        assert_eq!(cs.meta[1].num_phases, 2);
    }

    #[test]
    fn test_counter_state_populate_zero_body_len() {
        // body_len=0 → clamped to 1 phase.
        let cs = make_counter_state(&[(1, 5, 0)]);
        assert_eq!(cs.meta[0].num_phases, 1);
        assert_eq!(cs.phases.len(), 1);
    }

    // ── seed / increment / break ──

    #[test]
    fn test_counter_state_seed_and_has_live() {
        let mut cs = make_counter_state(&[(1, 5, 1)]);
        assert!(!cs.has_live());
        cs.seed(0, 0);
        assert!(cs.has_live());
    }

    #[test]
    fn test_counter_state_increment_below_min() {
        // min=3, seed at 0, increment once → value=1 < min=3, no break.
        let mut cs = make_counter_state(&[(3, 5, 1)]);
        cs.seed(0, 0);
        assert!(!cs.increment(0));
    }

    #[test]
    fn test_counter_state_increment_reaches_min() {
        // min=2, seed at 0, increment twice → value=2 >= min=2, can break.
        let mut cs = make_counter_state(&[(2, 5, 1)]);
        cs.seed(0, 0);
        assert!(!cs.increment(0)); // value=1
        assert!(cs.increment(0)); // value=2 >= min=2
    }

    #[test]
    fn test_counter_state_increment_reaches_max() {
        // min=1, max=3.  Seed, increment 3 times → oldest evicted at max.
        let mut cs = make_counter_state(&[(1, 3, 1)]);
        cs.seed(0, 0);
        assert!(cs.increment(0)); // value=1 >= min=1
        assert!(cs.increment(0)); // value=2
        assert!(cs.increment(0)); // value=3 == max → oldest evicted
        // After eviction the phase should be empty (only 1 instance).
        assert!(!cs.has_live());
    }

    #[test]
    fn test_counter_state_multiple_instances() {
        // min=2, max=5. Seed two instances, increment both.
        let mut cs = make_counter_state(&[(2, 5, 1)]);
        cs.seed(0, 0); // instance A at value 0
        cs.seed(0, 0); // instance B at value 0
        assert!(!cs.increment(0)); // A=1, B=1 — both < min=2
        assert!(cs.increment(0)); // A=2, B=2 — A >= min
    }

    #[test]
    fn test_counter_state_seed_with_initial_value() {
        // min=3, seed at value 2, one increment should break.
        let mut cs = make_counter_state(&[(3, 10, 1)]);
        cs.seed(0, 2);
        assert!(cs.increment(0)); // value=3 >= min=3
    }

    // ── reset_counter ──

    #[test]
    fn test_counter_state_reset_counter() {
        let mut cs = make_counter_state(&[(1, 10, 1)]);
        cs.seed(0, 0);
        assert!(cs.has_live());
        cs.reset_counter(0);
        assert!(!cs.has_live());
    }

    #[test]
    fn test_counter_state_reset_one_counter_preserves_other() {
        let mut cs = make_counter_state(&[(1, 10, 1), (1, 10, 1)]);
        cs.seed(0, 0);
        cs.seed(1, 0);
        assert!(cs.has_live());
        cs.reset_counter(0);
        assert!(cs.has_live()); // counter 1 still live
        cs.reset_counter(1);
        assert!(!cs.has_live());
    }

    // ── advance_all_phases ──

    #[test]
    fn test_counter_state_advance_phases() {
        // body_len=3, so 3 phases.  Advance rotates active_phase.
        let mut cs = make_counter_state(&[(1, 10, 3)]);
        assert_eq!(cs.meta[0].active_phase, 0);
        cs.advance_all_phases();
        assert_eq!(cs.meta[0].active_phase, 1);
        cs.advance_all_phases();
        assert_eq!(cs.meta[0].active_phase, 2);
        cs.advance_all_phases();
        assert_eq!(cs.meta[0].active_phase, 0); // wraps
    }

    // ── reset / clear ──

    #[test]
    fn test_counter_state_reset_clears_instances() {
        let mut cs = make_counter_state(&[(1, 10, 1)]);
        cs.seed(0, 0);
        cs.increment(0);
        assert!(cs.has_live());
        cs.reset();
        assert!(!cs.has_live());
        assert_eq!(cs.meta[0].active_phase, 0);
    }

    #[test]
    fn test_counter_state_reset_retains_ring_capacity() {
        let mut cs = make_counter_state(&[(1, 100, 1)]);
        cs.seed(0, 0);
        let cap = cs.phases[0].ring.len();
        assert!(cap > 0);
        cs.reset();
        assert_eq!(cs.phases[0].ring.len(), cap);
    }

    #[test]
    fn test_counter_state_clear_drops_everything() {
        let mut cs = make_counter_state(&[(1, 10, 1)]);
        cs.seed(0, 0);
        cs.clear();
        assert_eq!(cs.meta.len(), 0);
        assert_eq!(cs.phases.len(), 0);
        assert_eq!(cs.total_phases, 0);
    }

    // ── repopulate after clear ──

    #[test]
    fn test_counter_state_repopulate_after_clear() {
        let mut cs = make_counter_state(&[(1, 5, 1)]);
        cs.seed(0, 0);
        cs.clear();

        // Repopulate with different counters.
        cs.populate(&[
            crate::CounterInfo {
                index: 0,
                min: 2,
                max: 8,
                body_byte_length: 2,
            },
            crate::CounterInfo {
                index: 1,
                min: 1,
                max: 3,
                body_byte_length: 1,
            },
        ]);
        assert_eq!(cs.meta.len(), 2);
        assert_eq!(cs.phases.len(), 3); // 2 + 1
        assert!(!cs.has_live());

        // Seed and increment the new counters.
        cs.seed(0, 0);
        cs.seed(1, 0);
        assert!(cs.has_live());
    }

    // ── ring buffer growth ──

    #[test]
    fn test_counter_state_ring_grows() {
        // max=10000, initial ring cap is capped at 4096.
        // Seeding > 4096 instances forces a grow.
        let mut cs = make_counter_state(&[(1, 10000, 1)]);
        let initial_cap = cs.phases[0].ring.len();
        assert!(initial_cap <= 4096);

        for _ in 0..5000 {
            cs.seed(0, 0);
        }
        assert!(cs.phases[0].ring.len() > initial_cap);
        assert!(cs.has_live());

        // Increment should still work correctly after growth.
        assert!(cs.increment(0)); // oldest=1 >= min=1
    }

    // ── ensure_phases idempotent ──

    #[test]
    fn test_counter_state_ensure_phases_idempotent() {
        let mut cs = make_counter_state(&[(1, 10, 1)]);
        let ptr1 = cs.phases.as_ptr();
        cs.ensure_phases(); // second call — no-op
        let ptr2 = cs.phases.as_ptr();
        assert_eq!(ptr1, ptr2);
    }

    // ── increment on empty phase ──

    #[test]
    fn test_counter_state_increment_empty_phase() {
        // No seed — increment returns false.
        let mut cs = make_counter_state(&[(1, 10, 1)]);
        assert!(!cs.increment(0));
        assert!(!cs.has_live());
    }

    // ── live_count tracking ──

    #[test]
    fn test_counter_state_live_count_seed_into_already_live() {
        // Seeding into an already-live phase should not double-count.
        let mut cs = make_counter_state(&[(1, 100, 1)]);
        cs.seed(0, 0);
        assert!(cs.has_live());
        assert_eq!(cs.live_count, 1);
        cs.seed(0, 0); // second instance in same phase
        assert_eq!(cs.live_count, 1); // still 1 live phase
    }

    #[test]
    fn test_counter_state_live_count_increment_evicts_all() {
        // Single instance, increment to max → evicted → phase empty.
        let mut cs = make_counter_state(&[(1, 2, 1)]);
        cs.seed(0, 0);
        assert_eq!(cs.live_count, 1);
        cs.increment(0); // value=1
        assert_eq!(cs.live_count, 1); // still live
        cs.increment(0); // value=2 == max → evicted
        assert_eq!(cs.live_count, 0);
        assert!(!cs.has_live());
    }

    #[test]
    fn test_counter_state_live_count_multi_phase() {
        // body_len=2 → 2 phases.  Seed into different phases.
        let mut cs = make_counter_state(&[(1, 10, 2)]);
        // Phase 0 is active initially. seed targets (active + nph - 1) % nph = 1.
        cs.seed(0, 0);
        assert_eq!(cs.live_count, 1); // phase 1 is live
        cs.advance_all_phases(); // active = 1
        cs.seed(0, 0); // targets (1 + 2 - 1) % 2 = 0
        assert_eq!(cs.live_count, 2); // both phases live
        cs.reset_counter(0);
        assert_eq!(cs.live_count, 0);
    }

    #[test]
    fn test_counter_state_live_count_reset_counter_partial() {
        // Two counters, reset one.
        let mut cs = make_counter_state(&[(1, 10, 1), (1, 10, 1)]);
        cs.seed(0, 0);
        cs.seed(1, 0);
        assert_eq!(cs.live_count, 2);
        cs.reset_counter(0);
        assert_eq!(cs.live_count, 1);
        cs.reset_counter(1);
        assert_eq!(cs.live_count, 0);
    }

    #[test]
    fn test_counter_state_live_count_reset_already_empty() {
        // Resetting a counter that's already empty should be harmless.
        let mut cs = make_counter_state(&[(1, 10, 1)]);
        assert_eq!(cs.live_count, 0);
        cs.reset_counter(0); // no-op
        assert_eq!(cs.live_count, 0);
    }

    #[test]
    fn test_counter_state_live_count_through_full_cycle() {
        // Seed, increment past max (evict), re-seed — live_count
        // should track correctly through the full lifecycle.
        let mut cs = make_counter_state(&[(1, 2, 1)]);
        assert_eq!(cs.live_count, 0);
        cs.seed(0, 0);
        assert_eq!(cs.live_count, 1);
        cs.increment(0); // value=1
        cs.increment(0); // value=2 == max → evicted
        assert_eq!(cs.live_count, 0);
        cs.seed(0, 0); // re-seed
        assert_eq!(cs.live_count, 1);
    }
}
