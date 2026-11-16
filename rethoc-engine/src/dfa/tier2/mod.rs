//! Tier 2: DFA with differential counters for fixed-length repetition bodies.
//!
//! Uses the Becchi differential-counter technique for O(1) per-character
//! counter management.  Supports the same deferred assertions as Tier 1.
//!
//! **Eligibility**: patterns with bounded repetitions where:
//! - No counter is nested inside another counter's body.
//! - Every counter body has a fixed byte-length (all paths through the body
//!   consume the same number of bytes).
//! - No CRLF assertions.
//! - No deferred assertions inside counter bodies.
//!
//! Counter-free patterns (Tier 1 eligible) are also Tier 2 eligible — the
//! differential counter logic is simply dormant.

pub(crate) mod eligibility;
pub(crate) mod overlap;

use std::fmt;

use crate::{
    AssertEval, AssertKind, CounterIdx, Prefilter, Regex, State, StateIdx, byte_match_ci,
    is_word_byte,
};

use super::{DfaCache, DfaMemory, DfaStateId};

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

/// Cached DFA transition for `(state, byte)`.
///
/// Non-counting transitions use `no_break` only.
/// Counting transitions select between `no_break` and `with_break`
/// based on the differential counter condition.
#[derive(Clone)]
struct Transition {
    /// DFA successor when no counter instance can break.
    no_break: DfaStateId,
    no_break_is_match: bool,
    no_break_is_match_at_end: bool,
    /// DFA successor when at least one instance can break.
    with_break: DfaStateId,
    with_break_is_match: bool,
    with_break_is_match_at_end: bool,
    /// True if this transition crosses at least one CInc node.

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
}

impl Transition {
    fn empty() -> Self {
        Self {
            no_break: DfaStateId::UNPOPULATED,
            no_break_is_match: false,
            no_break_is_match_at_end: false,
            with_break: DfaStateId::UNPOPULATED,
            with_break_is_match: false,
            with_break_is_match_at_end: false,
            counting_mask: 0,
            pre_seeds: Box::new([]),
            seeds: Box::new([]),
            counter_reset: 0,
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
    /// Flat array of NFA consuming-state indices reachable from each
    /// counter's break target (`CInc.out1`) via epsilon transitions.
    /// These are the states that need to consume the current byte when
    /// a deferred-assertion-gated counter breaks (Bug 18).
    ///
    /// Precomputed at build time — depends only on the NFA structure.
    break_consuming: Box<[StateIdx]>,
    /// Per-counter `(start, end)` range into [`break_consuming`](Self::break_consuming).
    break_consuming_ranges: Box<[(usize, usize)]>,
    /// Per-counter flag: `true` if `Match` is directly reachable (without
    /// passing through an end-of-input assertion) from the counter's break
    /// path (`CInc.out1`).  Precomputed at build time.
    break_direct_match: Box<[bool]>,
    /// Per-NFA-state bitmask of counter indices whose `CounterIncrement`
    /// is reachable via epsilon transitions.  Indexed by `StateIdx`.
    /// Replaces the runtime `find_cinc_counters` and `deferred_cinc_mask`
    /// graph walks.
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

    /// Returns `true` if `Match` is directly reachable (without an
    /// end-of-input assertion) from counter `ci`'s break path.
    pub(crate) fn break_has_direct_match(&self, ci: usize) -> bool {
        self.break_direct_match[ci]
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

    /// Returns the slice of consuming NFA states reachable from counter
    /// `ci`'s break target (`CInc.out1`) via epsilon transitions.
    pub(crate) fn break_consuming(&self, ci: usize) -> &[StateIdx] {
        let (start, end) = self.break_consuming_ranges[ci];
        &self.break_consuming[start..end]
    }
}

/// Precompute body interior data and break-path consuming states for all
/// counters in a Tier 2 pattern.
///
/// For each counter, identifies:
/// 1. **Interior states** — NFA consuming states in the "interior" of the
///    counter body (body positions 1..L-1 for a body of length L).  Used to
///    detect when a DFA successor has in-progress body instances.
/// 2. **Break consuming states** — NFA consuming states reachable from
///    `CInc.out1` (the break target) via epsilon transitions.  Used when
///    a deferred-assertion-gated counter breaks (Bug 18).
pub(crate) fn compute_tier2_analysis(
    states: &[State],
    byte_tables: &[crate::ByteMap],
    num_counters: usize,
) -> Tier2Analysis {
    let mut per_counter: Vec<Vec<u32>> = vec![Vec::new(); num_counters];
    let mut break_per_counter: Vec<Vec<StateIdx>> = vec![Vec::new(); num_counters];
    let mut break_direct: Vec<bool> = vec![false; num_counters];

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

        // Collect break-path consuming states and match reachability
        // for each CInc.
        if let State::CounterIncrement { counter, out1, .. } = *s {
            let ci = counter.idx();
            if ci < num_counters {
                break_per_counter[ci] = collect_break_consuming(out1, states);
                let (direct, _at_end) = break_path_match_kind(out1, states);
                if direct {
                    break_direct[ci] = true;
                }
            }
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

    // Flatten break consuming into a single Vec with (start, end) ranges.
    let mut break_flat: Vec<StateIdx> = Vec::new();
    let mut break_ranges: Vec<(usize, usize)> = Vec::with_capacity(num_counters);
    for v in break_per_counter {
        let start = break_flat.len();
        break_flat.extend(v);
        break_ranges.push((start, break_flat.len()));
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
        break_consuming: break_flat.into_boxed_slice(),
        break_consuming_ranges: break_ranges.into_boxed_slice(),
        break_direct_match: break_direct.into_boxed_slice(),
        cinc_reachable: cinc_reachable.into_boxed_slice(),
        ci_seed_consuming: seed_flat.into_boxed_slice(),
        ci_seed_consuming_ranges: seed_ranges.into_boxed_slice(),
    }
}

/// Collect consuming NFA states reachable from `start` via epsilon
/// transitions (Split, Assert, CounterInstance).  Stops at consuming
/// states and CInc — does not recurse into nested counters.
fn collect_break_consuming(start: StateIdx, states: &[State]) -> Vec<StateIdx> {
    let mut result = Vec::new();
    let mut stack = vec![start];
    let mut visited = vec![false; states.len()];
    while let Some(idx) = stack.pop() {
        let i = idx.idx();
        if visited[i] {
            continue;
        }
        visited[i] = true;
        match states[idx] {
            State::Split { out, out1 } => {
                stack.push(out);
                stack.push(out1);
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
    result
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

    /// Advance active_phase for every counter (called on every byte).
    #[inline]
    fn advance_all_phases(&mut self) {
        for m in &mut self.meta {
            m.active_phase = (m.active_phase + 1) % m.num_phases;
        }
    }

    /// Seed a new counter instance into the appropriate phase.
    #[inline(always)]
    fn seed(&mut self, c_idx: usize, initial_value: u32) {
        let m = &self.meta[c_idx];
        let nph = m.num_phases;
        let target_phase = (m.active_phase + nph - 1) % nph;
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
    /// [`DfaStateId::UNPOPULATED`] marks entries not computed yet.
    settled: Vec<DfaStateId>,
    /// Whether [`settled_state`](Self::settled_state) may be used for the
    /// prepared regex.  Computed once in [`prepare`](Self::prepare).
    settle_enabled: bool,
}

impl fmt::Debug for Tier2DfaCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tier2DfaCache")
            .field("num_states", &self.inner.states.len())
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

    fn intern_state(
        &mut self,
        nfa_states: &[StateIdx],
        deferred_asserts: &[StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        prev_was_word: bool,
    ) -> Option<DfaStateId> {
        let stride = self.stride;
        self.inner.intern_state(
            nfa_states,
            deferred_asserts,
            is_match,
            is_match_at_end,
            prev_was_word,
            || {
                self.transitions
                    .extend(std::iter::repeat_with(Transition::empty).take(stride));
            },
        )
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
        let mut cinc_mask: u64 = 0;

        let (is_match, is_match_at_end) = memory.epsilon_closure(
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
            |mem, counter, out, out1, _, _| {
                encountered_cinc = true;
                cinc_mask |= 1u64 << counter.idx();
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
            cinc_mask,
            seed_instances: seed_instances.into_boxed_slice(),
        }
    }

    // -----------------------------------------------------------------------
    // Transition computation
    // -----------------------------------------------------------------------

    fn populate(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
        analysis: &Tier2Analysis,
    ) -> Transition {
        let mut targets: Vec<StateIdx> = Vec::new();

        let mut resolved_cinc = false;
        let mut resolved_cinc_mask: u64 = 0;
        let mut resolved_is_match = false;
        // Note: we do NOT track resolved_is_match_at_end here.  That flag
        // means "the deferred assertion passed mid-stream (with next=byte) and
        // $ → Match was reachable."  But the assertion was evaluated with
        // next=Some(byte) — at actual end-of-input, next is None and word-
        // boundary conditions may flip.  finish() correctly re-evaluates via
        // resolve_deferred_at_end() and resolve_deferred_cinc_at_end().
        // Match reachable through CInc break path in Phase 1 resolution.
        // Only valid when any_can_break is true (counter >= min).
        let mut resolved_break_match = false;
        // Counters entered through Phase 1 resolution whose first body
        // byte was consumed on this transition (bit `i` = counter `i`).
        // Only these get a pre-seed and are spared by `counter_reset`.
        // A counter that was entered but whose body did not take the byte
        // is speculative and must not be seeded (Bug 14).  Another
        // resolved thread taking the byte — a state after the counter's
        // break, say — says nothing about the body (Bug 62).
        let mut resolved_body_mask: u64 = 0;
        // Targets from consuming states on CInc break paths, consumed on
        // this transition.  These states are only reachable when the
        // deferred assertion passes AND the counter breaks, so they are
        // added exclusively to the `with_break` DFA successor (Bug 18).
        let mut resolved_break_targets: Vec<StateIdx> = Vec::new();

        if from != DfaStateId::DEAD {
            let from_state = &self.inner.states[from.idx()];
            // Phase 1: resolve deferred assertions.
            let extra = from_state.resolve_deferred(byte, regex);
            if !extra.is_empty() {
                let resolved_prev = from_state.prev_byte_representative();
                // Use follow_break=false: the CInc break path should NOT
                // contribute to resolved_is_match.  Match via counter break
                // is handled by the with_break DFA successor, gated by
                // the counter condition in step_inner.
                let cr = self.epsilon_closure(
                    memory,
                    extra.into_iter(),
                    regex,
                    analysis,
                    false,
                    resolved_prev,
                    Some(byte),
                    false,
                );
                for &(counter, first) in cr.seed_instances.iter() {
                    if cr.nfa_states.contains(&first) && consume_byte(first, byte, regex).is_some()
                    {
                        resolved_body_mask |= 1u64 << counter.idx();
                    }
                }

                resolved_cinc = cr.encountered_cinc;
                resolved_cinc_mask = cr.cinc_mask;
                resolved_is_match = cr.is_match;
                // Discard cr.is_match_at_end — see comment above.

                // If CInc was reached via Phase 1 resolution, check if
                // the break path (CInc.out1) can reach Match directly
                // (without going through $ or end-line assertions).
                //
                // Phase 1 fires mid-stream (we have a next byte), so $
                // cannot pass at the break position.  Paths through $
                // are handled at actual end-of-input by
                // resolve_deferred_cinc_at_end() in finish().
                if cr.encountered_cinc {
                    let mut mask = cr.cinc_mask;
                    while mask != 0 {
                        let ci = mask.trailing_zeros() as usize;
                        mask &= mask - 1;

                        // Check direct (epsilon-only) match from break
                        // target — precomputed at analysis time.
                        if regex.counter_break_can_match[ci] && analysis.break_has_direct_match(ci)
                        {
                            resolved_break_match = true;
                        }

                        // Consume the current byte at precomputed
                        // break-path consuming states (Bug 18).
                        // These are only reachable when the counter
                        // breaks AND the deferred assertion passed.
                        for &bc in analysis.break_consuming(ci) {
                            if let Some((t, te)) = consume_byte(bc, byte, regex) {
                                resolved_break_targets.push(t);
                                if te != StateIdx::NONE {
                                    resolved_break_targets.push(te);
                                }
                            }
                        }
                    }
                }
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

        let is_counting = probe.encountered_cinc || resolved_cinc;

        // Find ALL counters that fire CInc at this transition.
        // Include counters reached via resolved deferred assertions
        // (Phase 1) which are not reachable from targets alone.
        // Guarded by is_counting because the precomputed cinc_reachable
        // is a static over-approximation that ignores assertion gating.
        let counting_mask = if is_counting {
            targets
                .iter()
                .fold(0u64, |acc, &t| acc | analysis.cinc_reachable_from(t))
                | resolved_cinc_mask
        } else {
            0
        };

        // Debug assertion for non-overlap-proven patterns: the disjoint-bytes
        // guarantee means counting_mask should have at most 1 bit set.
        // For overlap-proven patterns, multi-CInc is expected and was
        // validated by the binary-exactness proof at compile time.
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
            let nb_id = self.intern_closure_result(&cr_nb, byte);
            // When Phase 1 resolved a deferred assertion and reached
            // CInc, the break path's consuming states may have consumed
            // `byte`.  Those targets are only reachable when the counter
            // actually breaks, so they go into the with_break closure
            // exclusively (Bug 18).
            let wb_id = if resolved_break_targets.is_empty() {
                self.intern_closure_result(&probe, byte)
            } else {
                let wb_closure = self.epsilon_closure(
                    memory,
                    targets
                        .iter()
                        .copied()
                        .chain(resolved_break_targets.iter().copied())
                        .chain(std::iter::once(regex.start)),
                    regex,
                    analysis,
                    false,
                    Some(byte),
                    None,
                    true,
                );
                self.intern_closure_result(&wb_closure, byte)
            };
            let (nb_m, nb_mae) = self.match_flags(nb_id);
            let (wb_m, wb_mae) = self.match_flags(wb_id);

            // For counting transitions, compute counter_reset for all
            // counters EXCEPT those firing CInc (managed by increment logic).
            let counter_reset =
                self.compute_counter_reset(analysis, &probe.nfa_states, counting_mask);

            // Resolved seeds from deferred assertions must be pre_seeds
            // (applied BEFORE advance_all_phases) to get the correct
            // Tier 2 phase alignment.  Phase 1 already consumed one
            // body byte, so the instance must be placed one phase
            // ahead of what seed_counter's default targeting computes.
            // Pre-seed timing achieves this: active_phase has not yet
            // been advanced for this byte, so the target phase is
            // effectively shifted by -1 relative to a post-advance
            // seed.
            //
            // For L=1 bodies: the seed goes to pre_seeds with value 0;
            // the counter_increment on this transition provides the +1.
            // For L>1 bodies: the seed goes to pre_seeds with value 0;
            // the next CInc (after the remaining L-1 body bytes) will
            // provide the increment.
            //
            // Skip reset: the counter must NOT be cleared by
            // counter_reset, because the pre-seeded instance
            // represents valid work from the resolved assertion path.
            //
            // When the body was NOT consumed (e.g. `\B {7,12}` on `!`),
            // the resolved seeds are speculative and must be discarded
            // to avoid false positives (Bug 14).
            let pre_seed_list = Self::pre_seeds_for(resolved_body_mask);
            let counter_reset = counter_reset & !resolved_body_mask;

            Transition {
                no_break: nb_id,
                no_break_is_match: nb_m || resolved_is_match,
                no_break_is_match_at_end: nb_mae,
                with_break: wb_id,
                with_break_is_match: wb_m || resolved_is_match || resolved_break_match,
                with_break_is_match_at_end: wb_mae,
                counting_mask,
                pre_seeds: pre_seed_list.into_boxed_slice(),
                seeds: seed_list.into_boxed_slice(),
                counter_reset,
            }
        } else {
            let id = self.intern_closure_result(&probe, byte);
            let (m, mae) = self.match_flags(id);

            // For non-counting transitions, compute counter_reset for
            // all counters.
            let counter_reset = self.compute_counter_reset(analysis, &probe.nfa_states, 0);

            // Resolved seeds from deferred assertions go to pre_seeds
            // for correct phase alignment (same rationale as counting
            // transitions — see comment above).
            let pre_seed_list = Self::pre_seeds_for(resolved_body_mask);
            let counter_reset = counter_reset & !resolved_body_mask;

            // Counters reachable from deferred assertions (for pre-reset
            // snapshot in step_inner).
            let dcm = probe
                .deferred_asserts
                .iter()
                .fold(0u64, |acc, &assert_idx| {
                    if let State::Assert { out, .. } = regex.states[assert_idx] {
                        acc | analysis.cinc_reachable_from(out)
                    } else {
                        acc
                    }
                });

            // For L=1 counter bodies with deferred assertions gating
            // CInc, the probe can't see through the deferred assertion
            // to find CI → body seeds.  If counter_reset will clear
            // the counter AND dcm shows CInc is behind a deferred
            // assertion, inject a seed so the counter is re-populated
            // after clearing.  This allows the counter to accumulate
            // across transitions where the assertion always defers.
            let needs_deferred_seed = dcm & counter_reset;
            if needs_deferred_seed != 0 {
                let mut mask = needs_deferred_seed;
                while mask != 0 {
                    let ci = mask.trailing_zeros() as usize;
                    mask &= mask - 1;
                    let counter = CounterIdx(ci as u8);
                    if !seed_list.iter().any(|s| s.0 == counter) {
                        seed_list.push((counter, 0));
                    }
                }
                seed_list.sort_by_key(|&(c, _)| c);
            }

            Transition {
                no_break: id,
                no_break_is_match: m || resolved_is_match,
                no_break_is_match_at_end: mae,
                with_break: id,
                with_break_is_match: m || resolved_is_match,
                with_break_is_match_at_end: mae,
                counting_mask: 0,
                pre_seeds: pre_seed_list.into_boxed_slice(),
                seeds: seed_list.into_boxed_slice(),
                counter_reset,
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
    fn settled_state(&mut self, id: DfaStateId, regex: &Regex) -> DfaStateId {
        match self.settled.get(id.idx()) {
            Some(&s) if s != DfaStateId::UNPOPULATED => s,
            _ => self.compute_settled(id, regex),
        }
    }

    #[cold]
    fn compute_settled(&mut self, id: DfaStateId, regex: &Regex) -> DfaStateId {
        debug_assert!(self.settle_enabled);
        let target = self.without_body_states(id, regex).unwrap_or(id);
        if self.settled.len() < self.inner.states.len() {
            self.settled
                .resize(self.inner.states.len(), DfaStateId::UNPOPULATED);
        }
        self.settled[id.idx()] = target;
        // The result has no body states left, so it settles to itself.
        self.settled[target.idx()] = target;
        target
    }

    /// Intern the state `id` minus its counter-body NFA states.  `None`
    /// when `id` has none, when nothing would remain, or when the state
    /// cap is reached.
    fn without_body_states(&mut self, id: DfaStateId, regex: &Regex) -> Option<DfaStateId> {
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
        let (is_match, is_match_at_end, prev_was_word) =
            (state.is_match, state.is_match_at_end, state.prev_was_word);
        if nfa.is_empty() && deferred.is_empty() && !is_match && !is_match_at_end {
            return None;
        }
        self.intern_state(&nfa, &deferred, is_match, is_match_at_end, prev_was_word)
    }

    /// Whether dead counter-body states may be dropped for `regex`.
    ///
    /// Two classes of pattern are excluded because the "body thread ⇒ live
    /// instance" argument does not carry over to them unchanged:
    ///
    /// - **Overlap-proven patterns.**  Their Tier 2 eligibility rests on a
    ///   proof over the DFA states reachable by ordinary transitions;
    ///   states produced by dropping body states are outside that set.
    /// - **Assertions inside a counter body.**  A thread parked on a
    ///   deferred body assertion has its instance cleared and re-seeded by
    ///   `populate()`, so instance liveness is not exact for these bodies.
    fn can_settle(regex: &Regex) -> bool {
        !regex.counter_body_states.is_empty()
            && !regex.tier2_overlap_proven
            && !regex
                .states
                .iter()
                .zip(regex.counter_body_states.iter())
                .any(|(state, &in_body)| in_body && matches!(state, State::Assert { .. }))
    }

    fn intern_closure_result(&mut self, cr: &ClosureResult, byte: u8) -> DfaStateId {
        if cr.nfa_states.is_empty()
            && cr.deferred_asserts.is_empty()
            && !cr.is_match
            && !cr.is_match_at_end
        {
            return DfaStateId::DEAD;
        }
        let pw = if cr.deferred_asserts.is_empty() {
            false
        } else {
            is_word_byte(byte)
        };
        self.intern_state(
            &cr.nfa_states,
            &cr.deferred_asserts,
            cr.is_match,
            cr.is_match_at_end,
            pw,
        )
        .unwrap_or(DfaStateId::DEAD)
    }

    fn match_flags(&self, id: DfaStateId) -> (bool, bool) {
        if id == DfaStateId::DEAD {
            (false, false)
        } else {
            let s = &self.inner.states[id.idx()];
            (s.is_match, s.is_match_at_end)
        }
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
                false,
            )
            .expect("start state exceeds DFA_MAX_STATES");
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
    /// Bitmask of counters that reached CInc during this closure.
    cinc_mask: u64,
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

/// Walk from `start` through epsilon states on the CInc break path
/// (CInc.out1) and determine how `Match` is reachable:
///
/// Returns `(direct, at_end)`:
/// - `direct`: `Match` is reachable without going through `$` / end-line
///   assertions — this fires mid-stream.
/// - `at_end`: `Match` is reachable only through `$` / `(?m:$)` /
///   `(?Rm:$)` — this fires only at end-of-input.
///
/// Both can be true if there are multiple paths.
/// Walk from `start` through epsilon states on the CInc break path
/// (CInc.out1) and determine how `Match` is reachable:
///
/// Returns `(direct, at_end)`:
/// - `direct`: Match is reachable without passing through an end-like
///   assertion (`$`, `\Z`, `\z`).
/// - `at_end`: Match is reachable through an end-like assertion.
fn break_path_match_kind(start: StateIdx, states: &[State]) -> (bool, bool) {
    let mut direct = false;
    let mut at_end = false;
    let mut stack: Vec<(StateIdx, bool)> = vec![(start, false)];
    let mut visited = vec![[false; 2]; states.len()];
    while let Some((idx, through_end)) = stack.pop() {
        let i = idx.idx();
        let te = through_end as usize;
        if visited[i][te] {
            continue;
        }
        visited[i][te] = true;
        match states[idx] {
            State::Match => {
                if through_end {
                    at_end = true;
                } else {
                    direct = true;
                }
            }
            State::Split { out, out1 } => {
                stack.push((out, through_end));
                stack.push((out1, through_end));
            }
            State::Assert { kind, out } => {
                let is_end_like = matches!(
                    kind,
                    AssertKind::End | AssertKind::EndLF | AssertKind::EndCRLF
                );
                stack.push((out, through_end || is_end_like));
            }
            State::CounterInstance { out, .. } => {
                stack.push((out, through_end));
            }
            _ => {}
        }
    }
    (direct, at_end)
}

// ---------------------------------------------------------------------------
// Tier 2 Matcher
// ---------------------------------------------------------------------------

/// Tier 2 DFA matcher with differential counters.
pub struct Tier2DfaMatcher<'a> {
    cache: &'a mut Tier2DfaCache,
    memory: &'a mut DfaMemory,
    regex: &'a Regex,
    analysis: &'a Tier2Analysis,
    current: DfaStateId,
    ever_matched: bool,
    match_at_end: bool,
    has_live_counters: bool,
    /// Copy of [`Tier2DfaCache::settle_enabled`]: whether dead
    /// counter-body states may be dropped from the DFA state once no
    /// instance is live.
    settle: bool,
    prefilter: Prefilter,
}

/// Check if resolving deferred assertions at end-of-input reaches a
/// CInc whose counter, after one more increment, allows a break to
/// Match.  Free function to avoid borrow conflicts between the DFA
/// state (`cache.inner`) and the counter/memory state.
fn resolve_deferred_cinc_at_end(
    deferred: &[StateIdx],
    prev: Option<u8>,
    regex: &crate::Regex,
    counters: &CounterState,
    memory: &mut DfaMemory,
) -> bool {
    if deferred.is_empty() {
        return false;
    }
    let states = &regex.states;

    for v in memory.closure_visited.iter_mut() {
        *v = false;
    }
    memory.closure_stack.clear();
    for &assert_idx in deferred {
        if let State::Assert { kind, out } = states[assert_idx]
            && kind.eval(false, true, prev, None) == AssertEval::Pass
        {
            memory.closure_stack.push(out);
        }
    }

    while let Some(idx) = memory.closure_stack.pop() {
        let i = idx.idx();
        if memory.closure_visited[i] {
            continue;
        }
        memory.closure_visited[i] = true;
        match states[idx] {
            State::CounterIncrement { counter, min, .. } => {
                let ci = counter.idx();
                if !regex.counter_break_can_match[ci] {
                    continue;
                }
                if ci < counters.meta.len() {
                    let m = &counters.meta[ci];
                    for ph in 0..m.num_phases {
                        let phase = &counters.phases[m.phase_start + ph];
                        if !phase.is_empty() && phase.oldest + 1 >= min as u32 {
                            return true;
                        }
                    }
                }
            }
            State::Split { out, out1 } => {
                memory.closure_stack.push(out1);
                memory.closure_stack.push(out);
            }
            State::Assert { kind, out } => {
                if kind.eval(false, true, prev, None) == AssertEval::Pass {
                    memory.closure_stack.push(out);
                }
            }
            State::CounterInstance { out, .. } => memory.closure_stack.push(out),
            _ => {}
        }
    }
    false
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

        Tier2DfaMatcher {
            current: cache.inner.start_id,
            ever_matched: cache.inner.start_is_match,
            match_at_end: cache.inner.start_is_match_at_end,
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
        if self.cache.transitions[slot].no_break == DfaStateId::UNPOPULATED {
            let trans =
                self.cache
                    .populate(self.memory, self.current, byte, self.regex, self.analysis);
            self.cache.transitions[slot] = trans;
        }
        slot
    }

    /// Slot lookup using identity mapping (stride = 256, no byte-class indirection).
    #[inline(always)]
    fn ensure_transition_direct(&mut self, byte: u8) -> usize {
        let slot = self.current.idx() * 256 + byte as usize;
        if self.cache.transitions[slot].no_break == DfaStateId::UNPOPULATED {
            let trans =
                self.cache
                    .populate(self.memory, self.current, byte, self.regex, self.analysis);
            self.cache.transitions[slot] = trans;
        }
        slot
    }

    #[inline(always)]
    fn step(&mut self, byte: u8) {
        if self.current == DfaStateId::DEAD {
            self.step_from_dead(byte);
            return;
        }

        let slot = self.ensure_transition(byte);
        self.step_inner(slot);
    }

    #[inline(always)]
    fn step_direct(&mut self, byte: u8) {
        if self.current == DfaStateId::DEAD {
            self.step_from_dead(byte);
            return;
        }

        let slot = self.ensure_transition_direct(byte);
        self.step_inner(slot);
    }

    #[inline(always)]
    fn step_inner(&mut self, slot: usize) {
        let t = &self.cache.transitions[slot];

        // Fast path: non-counting, no seeds, no live counters.
        if t.counting_mask == 0
            && t.pre_seeds.is_empty()
            && t.seeds.is_empty()
            && !self.has_live_counters
        {
            self.current = t.no_break;
            self.match_at_end = t.no_break_is_match_at_end;
            self.ever_matched = self.ever_matched || t.no_break_is_match;
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
            self.current = t.with_break;
            self.ever_matched = self.ever_matched || t.with_break_is_match;
            self.match_at_end = t.with_break_is_match_at_end;
        } else {
            self.current = t.no_break;
            self.ever_matched = self.ever_matched || t.no_break_is_match;
            self.match_at_end = t.no_break_is_match_at_end;
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
        // `match_at_end` is left alone: the settled state keeps the match
        // flags, and the flag set above describes this position.
        if self.settle && !self.has_live_counters && self.current != DfaStateId::DEAD {
            self.current = self.cache.settled_state(self.current, self.regex);
        }
    }

    fn step_from_dead(&mut self, byte: u8) {
        if self.has_live_counters {
            self.cache.counters.reset();
            self.has_live_counters = false;
        }
        self.match_at_end = false;

        // The transition out of DEAD depends only on the byte (the
        // restart closure with that byte as predecessor), so it is cached
        // per byte class like any other transition.
        let class = if self.cache.stride == 256 {
            byte as usize
        } else {
            self.regex.byte_classes[byte as usize] as usize
        };
        if self.cache.dead_transitions[class].no_break == DfaStateId::UNPOPULATED {
            let trans = self.cache.populate(
                self.memory,
                DfaStateId::DEAD,
                byte,
                self.regex,
                self.analysis,
            );
            self.cache.dead_transitions[class] = trans;
        }
        let trans = &self.cache.dead_transitions[class];
        self.current = trans.no_break;

        // Still dead and nothing seeded: no counter state to maintain.
        // The phase clock only matters relative to live instances.
        if trans.no_break == DfaStateId::DEAD && trans.seeds.is_empty() {
            return;
        }

        if trans.counting_mask == 0 {
            if trans.no_break_is_match {
                self.ever_matched = true;
            }
            if trans.no_break_is_match_at_end {
                self.match_at_end = true;
            }
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
        if self.ever_matched {
            return;
        }

        let start_id = self.cache.inner.start_id;

        // Prefilter-aware scanning: when the DFA is in the start state and
        // a prefilter is available, use memchr to skip ahead to the next
        // candidate byte instead of stepping byte-by-byte.  This is
        // re-engaged every time the DFA returns to the start state, not
        // just once at chunk entry.
        match self.prefilter {
            Prefilter::None => self.chunk_no_prefilter(input),
            Prefilter::Memchr1(needle) => {
                self.chunk_prefilter(input, start_id, |hay| memchr::memchr(needle, hay));
            }
            Prefilter::Memchr2(b1, b2) => {
                self.chunk_prefilter(input, start_id, |hay| memchr::memchr2(b1, b2, hay));
            }
            Prefilter::Memchr3(b1, b2, b3) => {
                self.chunk_prefilter(input, start_id, |hay| memchr::memchr3(b1, b2, b3, hay));
            }
            Prefilter::Memclass(t) => {
                self.chunk_prefilter(input, start_id, |hay| crate::memclass::memclass(&t, hay));
            }
            Prefilter::Range(lo, hi) => {
                self.chunk_prefilter(input, start_id, |hay| {
                    crate::memrange::memrange(lo, hi, hay)
                });
            }
        }
    }

    #[inline(always)]
    fn chunk_no_prefilter(&mut self, input: &[u8]) {
        if self.cache.stride == 256 {
            for &b in input {
                if self.ever_matched {
                    return;
                }
                self.step_direct(b);
            }
        } else {
            for &b in input {
                if self.ever_matched {
                    return;
                }
                self.step(b);
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
            if self.current == start_id && !self.has_live_counters {
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
        if self.ever_matched {
            return true;
        }
        if self.match_at_end {
            return true;
        }
        if self.current != DfaStateId::DEAD {
            let state = &self.cache.inner.states[self.current.idx()];
            if state.is_match_at_end {
                return true;
            }
            // Standard deferred resolution: handles assertions outside
            // counter bodies (deferred assert -> Match path).
            let mut scratch = super::ReachScratch::new();
            if state.resolve_deferred_at_end(self.regex, &mut scratch) {
                return true;
            }
            // Tier 2 specific: handle deferred assertions inside counter
            // bodies.  When a deferred assertion resolves at end-of-input
            // and reaches CInc, we need to check whether incrementing the
            // counter allows a break to Match.
            if resolve_deferred_cinc_at_end(
                &state.deferred_asserts,
                state.prev_byte_representative(),
                self.regex,
                &self.cache.counters,
                self.memory,
            ) {
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

impl fmt::Debug for Tier2DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Tier2DfaMatcher");
        s.field("current", &self.current);
        if self.current != DfaStateId::DEAD {
            let state = &self.cache.inner.states[self.current.idx()];
            s.field("nfa_states", &state.nfa_states);
            s.field("is_match", &state.is_match);
            s.field("is_match_at_end", &state.is_match_at_end);
        }
        s.field("ever_matched", &self.ever_matched)
            .field("match_at_end", &self.match_at_end)
            .field("has_live_counters", &self.has_live_counters)
            .field("settle", &self.settle);
        s.finish()
    }
}

impl fmt::Display for Tier2DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.current == DfaStateId::DEAD {
            write!(f, "DFA[T2] state=DEAD matched={}", self.ever_matched)?;
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
            self.ever_matched,
        )?;
        if state.is_match {
            write!(f, " is_match")?;
        }
        if self.match_at_end {
            write!(f, " mae")?;
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

    /// True if the matcher sits in the start state with no live counter —
    /// the exact condition under which `chunk_prefilter` skips input.
    fn at_start(m: &Tier2DfaMatcher) -> bool {
        m.current == m.cache.inner.start_id && !m.has_live_counters
    }

    /// True if the matcher's DFA state lists a counter-body NFA state.
    fn has_body_state(m: &Tier2DfaMatcher) -> bool {
        m.current != DfaStateId::DEAD
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
            let expected = (whole.current, whole.has_live_counters, whole.match_at_end);

            let mut memory = DfaMemory::default();
            let mut cache = Tier2DfaCache::new();
            cache.prepare(&mut memory, &re, analysis);
            let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
            m.chunk(&input[..split]);
            m.chunk(&input[split..]);
            assert_eq!(
                (m.current, m.has_live_counters, m.match_at_end),
                expected,
                "split at {split}"
            );
        }
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
            assert_ne!(m.current, m.cache.inner.start_id);
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
        let body_only = cache.intern_state(&body, &[], false, false, false).unwrap();
        assert_eq!(cache.settled_state(body_only, &re), body_only);
        // With a match-at-end flag there is still something to report, so
        // the body state is dropped and the flag kept.
        let flagged = cache.intern_state(&body, &[], false, true, false).unwrap();
        let settled = cache.settled_state(flagged, &re);
        assert_ne!(settled, flagged);
        let state = &cache.inner.states[settled.idx()];
        assert!(state.nfa_states.is_empty() && state.is_match_at_end);
    }

    #[test]
    fn test_anchored_pattern_after_gap_expires() {
        // Anchored pattern: after the gap expires and the last live thread
        // dies, nothing can match any more.  The matcher ends up DEAD and
        // stays there through the cached dead transitions.
        with_matcher("^foo.{0,5}bar", b"fooxxxxxxxxxx", |m| {
            assert!(m.settle);
            assert!(!m.has_live_counters);
            assert_eq!(m.current, DfaStateId::DEAD);
            let populated = m
                .cache
                .dead_transitions
                .iter()
                .filter(|t| t.no_break != DfaStateId::UNPOPULATED)
                .count();
            assert_eq!(populated, 1, "one byte class seen while dead");
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
        let first = build("^foo.{0,5}bar|^qux");
        let second = build("^a.{0,5}b");
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        let analysis = first.tier2_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &first, analysis);
        assert_eq!(cache.dead_transitions.len(), cache.stride);
        for _ in 0..2 {
            let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &first, analysis);
            m.chunk(b"zzzzzzzzqux");
            assert_eq!(m.current, DfaStateId::DEAD);
            assert!(!m.finish());
        }
        assert!(
            cache
                .dead_transitions
                .iter()
                .any(|t| t.no_break != DfaStateId::UNPOPULATED)
        );

        let analysis = second.tier2_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &second, analysis);
        assert_eq!(cache.dead_transitions.len(), cache.stride);
        assert!(
            cache
                .dead_transitions
                .iter()
                .all(|t| t.no_break == DfaStateId::UNPOPULATED)
        );
    }

    #[test]
    fn test_settle_disabled_for_overlap_proven_patterns() {
        // Eligibility through the overlap proof rests on the states
        // reachable by ordinary transitions; states without body states
        // are not among them.  The flag is forced here: only the guard is
        // under test, not the proof.
        let mut re = build("foo.{0,5}bar");
        assert!(Tier2DfaCache::can_settle(&re));
        re.tier2_overlap_proven = true;
        assert!(!Tier2DfaCache::can_settle(&re));

        let analysis = re.tier2_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier2DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        assert!(!cache.settle_enabled);
        let mut m = Tier2DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        m.chunk(b"fooxxxxxxxxxxxxxxxx");
        assert!(!m.settle);
        assert!(m.cache.settled.is_empty());
        // The dead body state stays, as it did before settling existed.
        assert!(has_body_state(&m));
    }

    #[test]
    fn test_settle_disabled_for_assertions_inside_bodies() {
        // Instance liveness is not exact while a thread is parked on a
        // deferred assertion inside the body.
        let pattern = "x(a\\b){2,6}y";
        with_matcher(pattern, b"xa a a a a a a a a", |m| {
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
        let start = cache.inner.start_id;
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
        with_matcher("foo.{0,5}$", b"fooxxxxx", |m| assert!(m.match_at_end));
        with_matcher("foo.{0,5}$", b"fooxxxxxx", |m| {
            assert!(!m.match_at_end);
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
