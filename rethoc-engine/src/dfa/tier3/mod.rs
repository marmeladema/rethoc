//! Tier 3: DFA with conditional transitions for non-nested counters.
//!
//! Unlike Tier 4 (which unions both CInc paths into one DFA successor and
//! replays compiled counter programs at runtime), Tier 3 precomputes **two
//! DFA successor states** per counting transition: one that follows only
//! the continue path (`NoBreak`), and one that follows both paths
//! (`WithBreak`).  At runtime, the matcher evaluates counter values to
//! pick the correct successor.
//!
//! **Eligibility**: patterns with bounded repetitions where no counter is
//! nested inside another counter's body.  Body can be any length/structure.
//!
//! # Build-time analysis
//!
//! The [`Tier3Analysis`] struct, built once at regex compile time by
//! [`compute_tier3_analysis`], precomputes three tables that eliminate
//! per-transition DFS walks during DFA population:
//!
//! - **Per-target actions** (`targets`): for each NFA state that can be
//!   reached after consuming a byte, the structural action (advance or
//!   increment) is determined once and cached.
//! - **CI-output origins** (`ci_origins`): for each CounterInstance output,
//!   the consuming states reachable through epsilon transitions.
//! - **Break seeds** (`break_seeds`): global table of counter seeds that
//!   are gated on a specific counter's break path being taken.
//!
//! # Terminology: "origin"
//!
//! An **origin** is the NFA consuming state (`Byte`, `ByteCI`, `ByteClass`,
//! or `ByteTable`) where a counter instance is currently parked, waiting to
//! consume the next input byte.
//!
//! When an instance is first created at a CI (`CounterInstance`) node, its
//! origin is the first consuming state in the counter body.  As input
//! bytes are consumed, the instance's origin advances through the body's
//! consuming states.  When the body's last consuming state is consumed,
//! the epsilon closure reaches CInc and the counter value increments.
//!
//! Origins bridge the DFA (which tracks NFA state subsets) and per-counter
//! instance tracking (which tracks repetition counts).  A cached DFA
//! transition maps each origin to a target NFA state whose structural action
//! (`Option<Tier3OriginKind>`) is looked up from [`Tier3Analysis::targets`].

use std::fmt;

use crate::{
    AssertEval, AssertKind, CounterIdx, Prefilter, Regex, State, StateIdx, byte_match_ci,
    is_word_byte,
};

pub(crate) mod effects;

use super::{DfaCache, DfaMemory, DfaStateId, IdleStates};
use effects as tier3_effects;

// ---------------------------------------------------------------------------
// Precomputed NFA analysis (built once at regex compile time)
// ---------------------------------------------------------------------------

/// Precomputed NFA analysis for Tier 3 patterns.
///
/// Built once at regex compile time by [`compute_tier3_analysis`],
/// so that DFA population can look up structural actions directly
/// instead of running DFS walks per transition.
///
/// **Indexed by target state**: each entry in [`targets`] corresponds to
/// a post-consumption NFA state (the state reached *after* a byte is
/// consumed).  For `Byte`/`ByteCI`/`ByteClass` origins, the target is
/// the fixed `.out` field.  For `ByteTable` origins, each distinct
/// non-NONE table entry is a target.  This unifies all consuming-state
/// types under a single lookup.
#[derive(Clone, Debug)]
pub(crate) struct Tier3Analysis {
    /// Per-target-state precomputed origin action.
    ///
    /// Indexed by NFA state index.  `targets[idx]` is `Some(kind)` when
    /// `idx` is a known post-consumption target; `None` for states that
    /// are never reached as byte-consumption targets.
    ///
    /// **Build-time only** after Patch 8E — used by `populate()` and
    /// [`compile_target_effects()`](tier3_effects::compile_target_effects)
    /// during compilation, but not read at runtime.  Runtime reads
    /// [`target_effects`](Self::target_effects) instead.  Also used by
    /// `dump.rs`.
    pub(crate) targets: Box<[Option<Tier3OriginKind>]>,

    /// Per-CI-output consuming states.
    ///
    /// `ci_origins[idx]` lists the consuming NFA states reachable from
    /// state `idx` through epsilon transitions.  Used to resolve CI
    /// seed pairs at DFA populate time.  Empty for non-CI-output states.
    pub(crate) ci_origins: Box<[Box<[StateIdx]>]>,

    /// Global break-triggered seeds.
    ///
    /// Each entry records a seed that is only applied when a specific
    /// counter breaks: the seed for `counter` at `origin` is applied
    /// when `trigger`'s CInc break path is taken.
    ///
    /// **Build-time only** after Patch 8F — consumed by
    /// [`compile_target_effects()`](tier3_effects::compile_target_effects)
    /// to produce `AddSeed` atoms in `on_break`/`guarded`, but not read
    /// at runtime.  Also used by `dump.rs`.
    pub(crate) break_seeds: Box<[Tier3BreakSeed]>,

    /// Maximum number of distinct consuming NFA states in any single
    /// counter body.  Used as the stride for flat range-compressed
    /// instance storage in [`Tier3DfaMatcher`].  Zero when there are
    /// no counters.
    pub(crate) max_body_origins: usize,

    /// Maximum number of live [`Instance`] entries per counter in the
    /// per-instance fallback path.  Equals `max(counter_max * body_origins)`
    /// across all counters.  Used as the stride for the flat
    /// [`InstanceCounters`] storage.  Zero when there are no counters or
    /// when [`all_counters_rangeable`](Self::all_counters_rangeable) is true.
    pub(crate) max_instance_stride: usize,

    /// Per-consuming-state flag: true if the NFA state's byte-consumption
    /// target reaches `$ → Match` through epsilon transitions.
    ///
    /// Indexed by NFA state index.  Only meaningful for consuming states;
    /// `false` for all others.
    ///
    /// **Build-time only** — used by `populate()` for `counter_free_mae`
    /// computation.  Runtime reads go through
    /// [`origin_effects`](Self::origin_effects) instead.
    pub(crate) target_is_match_at_end: Box<[bool]>,

    /// Indexed by NFA state index.  True if consuming a byte at this state
    /// leads directly to `Match` through epsilon transitions (Split, Assert,
    /// CI — but NOT through CInc).
    ///
    /// **Build-time only** — used by `populate()`.  Runtime reads go
    /// through [`origin_effects`](Self::origin_effects) instead.
    pub(crate) target_is_match: Box<[bool]>,

    /// Per-consuming-state deferred assertions on the epsilon path from
    /// the byte-consumption target to further consuming states or Match.
    ///
    /// **Build-time only** — consumed by
    /// [`compile_all_target_effects`](tier3_effects::compile_all_target_effects)
    /// to populate [`origin_effects`](Self::origin_effects).  Not read
    /// at runtime.
    pub(crate) target_deferred_asserts: Box<[Box<[StateIdx]>]>,

    /// Per-NFA-state flag: true if the state is reachable from the start
    /// state by following only CInc *continue* paths (not break paths).
    ///
    /// Used by `counter_free_match_at_end` to distinguish consuming
    /// states that are always present (no counter break required) from
    /// states that only enter the DFA state after a counter breaks.
    /// Only the former can contribute to counter-free match-at-end.
    pub(crate) reachable_without_break: Box<[bool]>,

    /// `true` when all `CounterInstance` nodes are epsilon-reachable from
    /// the start state — i.e. reachable by following only `Split` and
    /// `CounterInstance` edges (no consuming states, no `Assert` nodes,
    /// no `CounterIncrement` nodes).
    ///
    /// When this holds, the unanchored start-state re-seeding injects
    /// value 0 into every counter body on every input byte, guaranteeing
    /// that values at each origin always form a contiguous range anchored
    /// at 0.  This enables the range-compressed fast path
    /// ([`RangeCounters`]).
    ///
    /// When `false`, some counter body is gated behind a consuming prefix
    /// or an assertion (e.g. `^`, `\b`) that blocks re-seeding.  Values
    /// can become non-contiguous and range compression would
    /// over-approximate, so the per-instance fallback is used instead.
    pub(crate) all_counters_rangeable: bool,

    /// `false` when [`target_effects`](Self::target_effects) could not be
    /// compiled exactly: a counter's break path or some `Advance` has too
    /// many alternative assertion sets in front of its consuming states
    /// to enumerate.  Such a pattern is not Tier 3 eligible (see
    /// [`tier3_effects::check_break_deferred_soundness`]).
    pub(crate) effects_exact: bool,

    /// Typed effects for each target state — **runtime authoritative source**
    /// for all nonlocal Tier 3 effects (since Patch 8E).
    ///
    /// Populated by [`tier3_effects::compile_all_target_effects`] from
    /// the `targets` and `break_effects` data.  At runtime, the step_slow
    /// macro, effect resolution, and finish() read `TargetStep` for local
    /// counter stepping and `on_break`/`guarded`/`immediate` for nonlocal
    /// effects (matches, tails, seeds, deferred assertions).
    pub(crate) target_effects: Box<[Option<tier3_effects::CompiledTargetEffects>]>,

    /// Per-consuming-state compiled origin effects.
    ///
    /// Consolidates match flags (`is_match`, `is_match_at_end`) and
    /// deferred assertion chain IDs (previously `target_assert_chain_ids`)
    /// into a single per-origin structure.  Indexed by NFA state index.
    ///
    /// Populated alongside `target_effects` during
    /// [`compile_all_target_effects`](tier3_effects::compile_all_target_effects).
    pub(crate) origin_effects: Box<[tier3_effects::CompiledOriginEffects]>,

    /// Arena of interned assertion chains referenced by effect guards.
    pub(crate) assert_chain_arena: tier3_effects::AssertChainArena,

    /// Pre-interned break-path effect data, indexed by
    /// [`BreakEffectsId`](tier3_effects::BreakEffectsId).
    ///
    /// Each entry corresponds to one counter's CInc break path.  All
    /// `Tier3OriginKind::Increment` entries for the same counter share
    /// the same `BreakEffectsId`.  Built at the end of
    /// [`compute_tier3_analysis`] after all break-* fields are populated.
    ///
    /// **Build-time only** after Patch 8E — consumed by
    /// [`compile_target_effects()`](tier3_effects::compile_target_effects)
    /// to produce [`CompiledTargetEffects`](tier3_effects::CompiledTargetEffects),
    /// but not read at runtime.  Also used by `dump.rs`.
    pub(crate) break_effects: Box<[tier3_effects::BreakEffects]>,
}

/// What happens structurally when a byte is consumed at a given target.
///
/// Dead targets are represented as `None` in `Tier3Analysis::targets`.
#[derive(Clone, Debug)]
#[allow(clippy::type_complexity)]
pub(crate) enum Tier3OriginKind {
    /// Epsilon closure from the target did NOT reach CInc.
    /// The instance keeps its counter value and moves to `new_origins`.
    Advance {
        new_origins: Box<[StateIdx]>,
        /// True if the epsilon closure ALSO reaches `$ → Match`
        /// (i.e. in addition to consuming states).  Used by the
        /// post-break tail tracker to set `match_at_end` when a
        /// tail advances through this path (Bug 37: byte-specific
        /// flag, replacing the static `target_is_match_at_end` which
        /// cannot handle ByteTable origins with per-byte targets).
        is_match_at_end: bool,
        /// True if the epsilon closure reaches `Match` directly
        /// (without `$`).  Bug 47: byte-specific flag for ByteTable
        /// tails whose per-byte target reaches Match directly.
        /// Replaces the static `target_is_match` which skips ByteTable.
        is_match: bool,
    },
    /// Epsilon closure from the target reached CInc.  Counter is
    /// incremented; min/max determine continue vs. break.
    ///
    /// Break-path data (deferred assertions, consuming tails, etc.) is
    /// stored in the [`BreakEffects`](tier3_effects::BreakEffects) table
    /// on [`Tier3Analysis`] and looked up by [`break_effects_id`] at
    /// runtime.  All `Increment` entries for the same counter share the
    /// same `BreakEffectsId`.
    Increment {
        /// The counter being incremented.
        counter: CounterIdx,
        /// Consuming states reachable WITHOUT going through CInc.
        advance_origins: Box<[StateIdx]>,
        min: u32,
        max: u32,
        /// Consuming states on the CInc continue path.
        continue_origins: Box<[StateIdx]>,
        /// True if the break path reaches `Match` directly.
        break_is_match: bool,
        /// True if the break path reaches `Match` through `$`.
        break_is_match_at_end: bool,
        /// Index into [`Tier3Analysis::break_effects`] for the pre-interned
        /// break-path data (deferred assertions, consuming tails, etc.).
        /// All `Increment` entries for the same counter share the same ID.
        /// Initialized to
        /// [`BreakEffectsId::NONE`](tier3_effects::BreakEffectsId::NONE) in
        /// `analyze_target()` and populated during
        /// [`compute_tier3_analysis`].
        break_effects_id: tier3_effects::BreakEffectsId,
    },
}

/// A break-triggered seed: applied only when `trigger` counter breaks.
#[derive(Clone, Debug)]
pub(crate) struct Tier3BreakSeed {
    /// The counter whose CInc break path leads to this seed.
    pub(crate) trigger: CounterIdx,
    /// The counter that gets a new instance.
    pub(crate) counter: CounterIdx,
    /// The consuming NFA state where the new instance starts.
    pub(crate) origin: StateIdx,
    /// Deferred assertion NFA state indices on the path from the trigger's
    /// CInc break output to this seed's CI.  These assertions must pass at
    /// the break position before the seed is applied (Bug 27).  Empty when
    /// the path has no assertions.
    ///
    /// **Semantics: PathChain (AND).**  One ordered path from the trigger's
    /// break output to this seed's CI.  All assertions in the chain must
    /// pass in sequence.
    ///
    /// **OR across seeds:** Multiple `Tier3BreakSeed` entries can share the
    /// same `(trigger, counter, origin)` but carry different assertion
    /// chains — e.g. when a Split in the epsilon closure creates paths
    /// that both reach the same CI, one through an assertion and one not.
    /// Each entry is compiled into a separate effect atom (`on_break` if
    /// ungated, `guarded` if gated); any one firing suffices (Bug 52).
    pub(crate) deferred_asserts: Box<[StateIdx]>,
    /// Interned assertion chain ID for the deferred assertions.
    /// [`AssertChainId::NONE`] when `deferred_asserts` is empty.
    /// Populated during [`compute_tier3_analysis`] after the assertion
    /// chain arena is built.
    pub(crate) assert_chain_id: tier3_effects::AssertChainId,
}

/// Build the [`Tier3Analysis`] for a tier-3-eligible pattern.
///
/// Walks the NFA state array to precompute:
/// 1. Per-target-state origin actions (what happens when a byte is consumed).
/// 2. Per-CI-output consuming states (for resolving CI seed pairs).
/// 3. Global break-triggered seeds (for counter break-path seeding).
/// 4. Max body origins per counter (stride for flat instance storage).
///
/// Uses only pure DFS — no DFA cache or matcher state is needed.
pub(crate) fn compute_tier3_analysis(
    states: &[State],
    byte_tables: &[crate::ByteMap],
    state_can_reach_match: &[bool],
    start: StateIdx,
) -> Tier3Analysis {
    let n = states.len();

    // -- Step 1: identify all target states -----------------------------------
    // A "target" is a post-consumption NFA state index.
    let mut is_target = vec![false; n];
    for (i, state) in states.iter().enumerate() {
        // Skip dead states (patched-away ByteTable interior nodes use Match
        // as a tombstone, but their out fields are irrelevant).
        let _ = i;
        match *state {
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                if out != StateIdx::NONE {
                    is_target[out.idx()] = true;
                }
                if out_exit != StateIdx::NONE {
                    is_target[out_exit.idx()] = true;
                }
            }
            State::ByteTable { table } => {
                let map = &byte_tables[table.idx()];
                for b in 0u16..256 {
                    let t = map[b as u8];
                    if t != StateIdx::NONE {
                        is_target[t.idx()] = true;
                    }
                }
            }
            _ => {}
        }
    }

    // -- Step 2: compute per-target origin actions ----------------------------
    let mut targets_vec: Vec<Option<Tier3OriginKind>> = vec![None; n];

    for idx in 0..n {
        if !is_target[idx] {
            continue;
        }
        let target = StateIdx(idx as u32);
        targets_vec[idx] = analyze_target(target, states, state_can_reach_match);
    }

    // -- Step 2b: compute per-counter break data (post-pass) --------------------
    // Now that all targets are known, re-derive each Increment target's
    // CInc break outputs and compute "true tail" consuming states — those
    // whose byte-consumption target does NOT lead to another CInc.
    //
    // Break-path data is stored in a per-counter side table and later
    // interned into the `BreakEffects` table.  All Increment entries for
    // the same counter share the same CInc break path, so we compute once
    // per counter.
    struct PerCounterBreakData {
        deferred_asserts: Box<[StateIdx]>,
        all_tails: Box<[StateIdx]>,
        pure_tails: Box<[StateIdx]>,
        per_tail_deferred: Box<[tier3_effects::DeferredTail]>,
    }
    let mut per_counter_break: Vec<Option<PerCounterBreakData>> = (0..64).map(|_| None).collect();
    let mut break_tails_exact = true;

    for idx in 0..n {
        if let Some(Tier3OriginKind::Increment { counter, .. }) = &targets_vec[idx] {
            let c = counter.idx();
            if per_counter_break[c].is_some() {
                // Already computed for this counter.
                continue;
            }
            // Walk from the target state itself (not a consuming state's
            // `out`) through epsilon transitions to find CInc break outputs.
            // `idx` is already the post-consumption target — it may be a
            // CInc directly, or reachable through Split/Assert/CI chains.
            let mut cinc_break_outs = Vec::new();
            let mut stack = vec![StateIdx(idx as u32)];
            let mut visited = vec![false; n];
            while let Some(s) = stack.pop() {
                let i = s.idx();
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                match states[s] {
                    State::Split { out, out1 } => {
                        stack.push(out1);
                        stack.push(out);
                    }
                    State::Assert { out, .. } => stack.push(out),
                    State::CounterInstance { out, .. } => stack.push(out),
                    State::CounterIncrement { out1, .. } => {
                        cinc_break_outs.push(out1);
                    }
                    _ => {}
                }
            }
            // Compute break closure (deferred asserts) and consuming tails.
            let bc = break_closure(&cinc_break_outs, states, state_can_reach_match);
            let tails = if cinc_break_outs.is_empty() {
                None
            } else {
                let tails = break_consuming_tails(&cinc_break_outs, states, &targets_vec);
                // Bug 59: too many alternative assertion sets on the
                // break path.  The analysis is left without tails and
                // flagged, which keeps the pattern off Tier 3.
                break_tails_exact &= tails.is_some();
                tails
            };
            let (all_tails, pure_tails, per_tail_deferred) = if let Some((a, p, d)) = tails {
                (
                    a.into_boxed_slice(),
                    p.into_boxed_slice(),
                    d.into_boxed_slice(),
                )
            } else {
                (
                    Box::new([]) as Box<[StateIdx]>,
                    Box::new([]) as Box<[StateIdx]>,
                    Box::new([]) as Box<[tier3_effects::DeferredTail]>,
                )
            };
            per_counter_break[c] = Some(PerCounterBreakData {
                deferred_asserts: bc.deferred_asserts,
                all_tails,
                pure_tails,
                per_tail_deferred,
            });
        }
    }

    // -- Step 3: compute per-CI-output consuming states -----------------------
    let mut ci_origins_vec: Vec<Box<[StateIdx]>> = vec![Box::new([]); n];
    for state in states {
        if let State::CounterInstance { out, .. } = *state
            && out != StateIdx::NONE
            && ci_origins_vec[out.idx()].is_empty()
        {
            let consuming = consuming_states_from(out, states);
            ci_origins_vec[out.idx()] = consuming.into_boxed_slice();
        }
    }

    // -- Step 4: compute global break seeds -----------------------------------
    // Collect ALL CInc nodes in the NFA, then walk each break path for CI
    // seeds.
    let mut all_cinc_nodes: Vec<(CounterIdx, StateIdx)> = Vec::new();
    for state in states {
        if let State::CounterIncrement { counter, out1, .. } = *state {
            all_cinc_nodes.push((counter, out1));
        }
    }

    let mut break_seeds_raw: Vec<(CounterIdx, CounterIdx, StateIdx, Vec<StateIdx>)> = Vec::new();
    for &(trigger, break_target) in &all_cinc_nodes {
        // Phase 1: walk epsilon transitions from break target (Split,
        // Assert, CI) — same as the original code.  Self-referential
        // seeds (counter == trigger) are allowed here because they
        // represent the outer loop re-entering the counter body
        // through epsilon transitions (e.g. `(a{2,3})+`).
        //
        // The stack carries accumulated deferred assert state indices
        // (Bug 27): non-End Assert states on the path from break_target
        // to a CI are gating conditions that must pass at break time.
        let mut ci_stack: Vec<(StateIdx, Vec<StateIdx>)> = vec![(break_target, Vec::new())];
        let mut ci_visited = vec![false; n];
        // Collect consuming states reachable via epsilon from the break
        // path — these are the "hop points" for phase 2.
        let mut break_consuming: Vec<StateIdx> = Vec::new();
        while let Some((idx, deferred)) = ci_stack.pop() {
            let i = idx.idx();
            let already_visited = ci_visited[i];
            ci_visited[i] = true;
            match states[idx] {
                State::CounterInstance { counter, out } => {
                    // Bug 52: always record seeds for every arrival path,
                    // even if this CI was already visited via a different
                    // path.  A Split in the closure can route both through
                    // an assertion and directly to the same CI, producing
                    // OR-alternative seeds with different deferred assertions.
                    let ci_consuming = consuming_states_from(out, states);
                    for c in ci_consuming {
                        break_seeds_raw.push((trigger, counter, c, deferred.clone()));
                    }
                    // Only explore past the CI once — the downstream graph
                    // is identical regardless of which path reached this CI.
                    if !already_visited {
                        ci_stack.push((out, deferred));
                    }
                }
                _ if already_visited => continue,
                State::Split { out, out1 } => {
                    ci_stack.push((out1, deferred.clone()));
                    ci_stack.push((out, deferred));
                }
                State::Assert { kind, out } => {
                    let mut d = deferred;
                    // Non-End asserts become guards, evaluated at the next
                    // byte: by then both sides of the position are known.
                    // That is one byte later than `^` and `(?m:^)` need,
                    // which changes nothing: the pending effect remembers
                    // what they read of the byte behind them.  End asserts
                    // are handled separately by the break closure
                    // (break_is_match_at_end).
                    if kind != AssertKind::End {
                        d.push(idx);
                    }
                    ci_stack.push((out, d));
                }
                State::Byte { .. }
                | State::ByteCI { .. }
                | State::Wildcard { .. }
                | State::ByteClassStatic { .. }
                | State::ByteClassCustom { .. }
                | State::ByteTable { .. } => {
                    break_consuming.push(idx);
                }
                _ => {}
            }
        }

        // Phase 2 was previously here: it followed consuming states'
        // targets to find CI nodes reachable after one byte consumption
        // ("multi-hop break seeds").  However, these seeds fire on the
        // same byte as the counter break, one step too early — the
        // intervening consuming state hasn't consumed its byte yet.
        // The tail mechanism (break_consuming_tails + post_break_tails)
        // now correctly handles these cases: the consuming state is
        // tracked as a tail, consumes its byte on the next step, and
        // seeds the downstream counter with proper byte timing.
        //
        // Bug 38: removing phase 2 fixes false positives on patterns
        // like `^e{4,5}e{4,5}ee{4,5}$` where the break path crosses
        // a consuming state before reaching the next counter's CI.
    }
    // Dedup break seeds by (trigger, counter, origin, deferred_asserts).
    //
    // A Split in the epsilon closure can create multiple paths from the
    // same CInc break to the same CI, some through assertions and some
    // bypassing them.  For example, `^a{1,2}(\b)?.{4,16}$` produces:
    //
    //   CInc(c0) break → Split → Assert(\b) → CI(c1)   [gated by \b]
    //   CInc(c0) break → Split → CI(c1)                 [ungated]
    //
    // Both paths share (trigger=c0, counter=c1, origin=body_of_c1) but
    // have different deferred assertion lists.  These are OR-alternatives:
    // the seed should fire if ANY path's assertions pass.  An ungated path
    // means the seed fires unconditionally.
    //
    // We preserve all distinct (key, assertions) pairs and let
    // `compile_target_effects()` emit them as separate `on_break` (ungated)
    // or `guarded` (gated) `AddSeed` atoms.  True duplicates (same key
    // AND same assertions) are still removed.
    //
    // Bug 52: the previous dedup compared only the key, silently discarding
    // ungated paths when a gated path for the same seed existed.
    for entry in &mut break_seeds_raw {
        entry.3.sort_unstable();
        entry.3.dedup();
    }
    break_seeds_raw.sort_by_key(|e| (e.0, e.1, e.2));
    break_seeds_raw.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3);

    let break_seeds: Vec<Tier3BreakSeed> = break_seeds_raw
        .into_iter()
        .map(|(trigger, counter, origin, deferred)| {
            // deferred is already sorted + deduped (done above for the
            // dedup_by comparison).
            Tier3BreakSeed {
                trigger,
                counter,
                origin,
                deferred_asserts: deferred.into_boxed_slice(),
                // Placeholder — populated after the assertion chain arena
                // is built in Step 9.
                assert_chain_id: tier3_effects::AssertChainId::NONE,
            }
        })
        .collect();

    // -- Step 5: compute max body origins per counter -------------------------
    // For each counter, walk from CI.out through the body (stopping at CInc
    // for the same counter) and count the distinct consuming NFA states.
    // The maximum across all counters becomes the stride for the flat
    // range-compressed instance storage.
    //
    // Also compute `max_instance_stride`: for each counter, the worst-case
    // number of live `Instance` entries is `counter_max * body_origins`.
    // The maximum across all counters becomes the stride for the flat
    // per-instance fallback storage.
    let mut max_body_origins: usize = 0;
    let mut max_instance_stride: usize = 0;
    for state in states {
        if let State::CounterInstance { counter, out } = *state {
            let mut count: usize = 0;
            let mut counter_max: usize = 0;
            let mut stack = vec![out];
            let mut visited = vec![false; n];
            while let Some(idx) = stack.pop() {
                let i = idx.idx();
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                match states[idx] {
                    State::CounterIncrement {
                        counter: c, max, ..
                    } if c == counter => {
                        counter_max = max;
                    }
                    State::Split { out, out1 } => {
                        stack.push(out1);
                        stack.push(out);
                    }
                    State::Assert { out, .. } | State::CounterInstance { out, .. } => {
                        stack.push(out);
                    }
                    State::Byte { out, out_exit, .. }
                    | State::ByteCI { out, out_exit, .. }
                    | State::Wildcard { out, out_exit }
                    | State::ByteClassStatic { out, out_exit, .. }
                    | State::ByteClassCustom { out, out_exit, .. } => {
                        count += 1;
                        stack.push(out);
                        if out_exit != StateIdx::NONE {
                            stack.push(out_exit);
                        }
                    }
                    State::ByteTable { table } => {
                        count += 1;
                        for &succ in &byte_tables[table.idx()].0 {
                            if succ != StateIdx::NONE {
                                stack.push(succ);
                            }
                        }
                    }
                    _ => {}
                }
            }
            max_body_origins = max_body_origins.max(count);
            // For unbounded counters (max = usize::MAX), cap the stride
            // to avoid overflow.  See `UNBOUNDED_INSTANCE_CAP`.
            let per_counter = counter_max
                .saturating_mul(count)
                .min(UNBOUNDED_INSTANCE_CAP);
            max_instance_stride = max_instance_stride.max(per_counter);
        }
    }

    // -- Step 6: compute per-consuming-state target_is_match_at_end ----------
    // For each consuming NFA state, check if its byte-consumption target
    // reaches `$ → Match` through epsilon transitions.  This is used by
    // the post-break tail tracker: when a tail's action is `None` (dead —
    // no further consuming states), this flag tells us whether the target
    // state is `$ → Match`.
    let mut target_mae = vec![false; n];
    for (i, state) in states.iter().enumerate() {
        let mut targets_buf: [Option<StateIdx>; 2] = [None, None];
        match *state {
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                if out != StateIdx::NONE {
                    targets_buf[0] = Some(out);
                }
                if out_exit != StateIdx::NONE {
                    targets_buf[1] = Some(out_exit);
                }
            }
            // ByteTable targets vary per byte; skip (overapproximation would
            // be unsound, and the tail tracker won't encounter ByteTable
            // origins in practice for tier 3 patterns).
            _ => {}
        }
        for t in targets_buf.into_iter().flatten() {
            // Walk epsilon transitions from the target to find `$ → Match`.
            let mut estack = vec![t];
            let mut evisited = vec![false; n];
            while let Some(eidx) = estack.pop() {
                let ei = eidx.idx();
                if evisited[ei] {
                    continue;
                }
                evisited[ei] = true;
                match states[eidx] {
                    State::Split { out, out1 } => {
                        estack.push(out1);
                        estack.push(out);
                    }
                    State::Assert { kind, out } => {
                        if kind == AssertKind::End && state_can_reach_match[out.idx()] {
                            target_mae[i] = true;
                        }
                    }
                    State::CounterInstance { out, .. } => estack.push(out),
                    _ => {}
                }
            }
        }
    }

    // -- Step 6b: compute per-consuming-state target_is_match -------------------
    // Similar to target_mae but checks for direct `Match` reachability (not
    // `$ → Match`).  Used by the post-break tail tracker to detect matches
    // when a tail consumes a byte and its target leads to Match without any
    // intervening `Assert(End)` (Bug 25).
    let mut target_match = vec![false; n];
    for (i, state) in states.iter().enumerate() {
        let mut targets_buf: [Option<StateIdx>; 2] = [None, None];
        match *state {
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                if out != StateIdx::NONE {
                    targets_buf[0] = Some(out);
                }
                if out_exit != StateIdx::NONE {
                    targets_buf[1] = Some(out_exit);
                }
            }
            _ => {}
        }
        for t in targets_buf.into_iter().flatten() {
            let mut estack = vec![t];
            let mut evisited = vec![false; n];
            while let Some(eidx) = estack.pop() {
                let ei = eidx.idx();
                if evisited[ei] {
                    continue;
                }
                evisited[ei] = true;
                match states[eidx] {
                    State::Split { out, out1 } => {
                        estack.push(out1);
                        estack.push(out);
                    }
                    // Do NOT follow Assert states: target_is_match
                    // means Match is reachable unconditionally (no
                    // assertion gating).  Assert(End) → Match is
                    // already handled by target_is_match_at_end.
                    State::Assert { .. } => {}
                    State::CounterInstance { out, .. } => estack.push(out),
                    State::Match => {
                        target_match[i] = true;
                    }
                    _ => {}
                }
            }
        }
    }

    // -- Step 6c: compute per-consuming-state target_deferred_asserts -----------
    // For each consuming NFA state, walk epsilon transitions from its target
    // and collect the FIRST non-End Assert state indices on each reachable
    // path.  These are the "entry-point" assertions that the pending-effect
    // system will evaluate at runtime via `eval_assert_chain`;
    // subsequent chained assertions (e.g. `\b → \B`) are handled by the
    // downstream `can_reach_match_at_end` / `can_reach_match_mid` walk that
    // follows the assert's `out` edge.
    //
    // IMPORTANT: the walk does NOT follow through non-End Assert states
    // (Bug 31).  Following through would record every assert in a chain
    // independently, causing each to be evaluated as if on a separate path —
    // a chain like `\b → \B` (contradictory) would spuriously match because
    // `\B` alone can pass.
    //
    // Used by the post-break tail tracker to emit PendingEffect entries with
    // Match atoms when a tail advances through the state (Bug 30).  Each
    // individual assert is interned as a 1-element chain in
    // `target_assert_chain_ids` during `compile_all_target_effects`.
    let mut target_da: Vec<Box<[StateIdx]>> = Vec::with_capacity(n);
    for state in states {
        let mut targets_buf: [Option<StateIdx>; 2] = [None, None];
        match *state {
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                if out != StateIdx::NONE {
                    targets_buf[0] = Some(out);
                }
                if out_exit != StateIdx::NONE {
                    targets_buf[1] = Some(out_exit);
                }
            }
            _ => {}
        }
        let mut asserts = Vec::new();
        for t in targets_buf.into_iter().flatten() {
            let mut estack = vec![t];
            let mut evisited = vec![false; n];
            while let Some(eidx) = estack.pop() {
                let ei = eidx.idx();
                if evisited[ei] {
                    continue;
                }
                evisited[ei] = true;
                match states[eidx] {
                    State::Split { out, out1 } => {
                        estack.push(out1);
                        estack.push(out);
                    }
                    State::Assert { kind, .. } => {
                        // Record non-End asserts but do NOT follow `out`.
                        // Subsequent chained asserts are evaluated by
                        // `can_reach_match_at_end` / `can_reach_match_mid`
                        // at runtime (they walk from the assert's `out`).
                        // End asserts are handled by target_is_match_at_end.
                        if kind != AssertKind::End {
                            asserts.push(eidx);
                        }
                        // Do NOT push `out` — stop the walk here for this
                        // path.  See Bug 31 comment above.
                    }
                    State::CounterInstance { out, .. } => estack.push(out),
                    _ => {}
                }
            }
        }
        // Deduplicate (shouldn't be needed but defensive).
        asserts.sort_unstable();
        asserts.dedup_by_key(|s| s.0);
        target_da.push(asserts.into_boxed_slice());
    }

    // -- Step 7: compute reachable_without_break --------------------------------
    // Transitive closure from start following ALL paths EXCEPT CInc break
    // paths.  Mark consuming states reachable without any counter break.
    // States only reachable via CInc break paths are counter-dependent
    // and should NOT contribute to counter_free_match_at_end.
    //
    // Unlike a single epsilon closure, this walk also follows consuming
    // states' `.out` targets so that multi-byte chains (e.g. unrolled
    // `[0-9]{1,3}\.[0-9]{1,3}...`) are fully traversed.  The `visited`
    // array ensures each state is processed at most once.
    let mut rwb = vec![false; n];
    {
        let mut visited = vec![false; n];
        let mut stack = vec![start];
        while let Some(idx) = stack.pop() {
            let i = idx.idx();
            if i >= n || visited[i] {
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
                State::CounterIncrement { out, .. } => {
                    // Follow only the continue path (out), NOT the break
                    // path (out1).  States behind out1 are counter-dependent.
                    stack.push(out);
                }
                State::Byte { out, out_exit, .. }
                | State::ByteCI { out, out_exit, .. }
                | State::Wildcard { out, out_exit }
                | State::ByteClassStatic { out, out_exit, .. }
                | State::ByteClassCustom { out, out_exit, .. } => {
                    rwb[i] = true;
                    // Follow .out to reach consuming states deeper in the
                    // NFA graph (e.g. unrolled repetition chains).
                    stack.push(out);
                    if out_exit != StateIdx::NONE {
                        stack.push(out_exit);
                    }
                }
                State::ByteTable { table } => {
                    rwb[i] = true;
                    // ByteTable dispatches to multiple targets — push all
                    // non-NONE entries.
                    for &target in &byte_tables[table].0 {
                        if target != StateIdx::NONE {
                            stack.push(target);
                        }
                    }
                }
                State::Match => {}
            }
        }
    }

    // -- Step 8: epsilon-reachability of all CI nodes ----------------------------
    // Walk from `start` following only Split and CounterInstance edges
    // (no consuming states, no Assert, no CInc).  If every CI in the NFA
    // is reachable this way, the unanchored loop re-seeds value 0 into
    // every counter body on every byte, so range compression is safe.
    let all_counters_rangeable = {
        let mut visited = vec![false; n];
        let mut stack = vec![start];
        let mut ci_reached = vec![false; n];
        while let Some(idx) = stack.pop() {
            let i = idx.idx();
            if i >= n || visited[i] {
                continue;
            }
            visited[i] = true;
            match states[idx] {
                State::Split { out, out1 } => {
                    stack.push(out1);
                    stack.push(out);
                }
                State::CounterInstance { out, .. } => {
                    ci_reached[i] = true;
                    stack.push(out);
                }
                // Stop at consuming states, Assert, CInc, Match —
                // these break the epsilon-only path.
                _ => {}
            }
        }
        // Every CI node must have been reached.
        states
            .iter()
            .enumerate()
            .all(|(i, s)| !matches!(s, State::CounterInstance { .. }) || ci_reached[i])
    };

    let mut analysis = Tier3Analysis {
        targets: targets_vec.into_boxed_slice(),
        ci_origins: ci_origins_vec.into_boxed_slice(),
        break_seeds: break_seeds.into_boxed_slice(),
        max_body_origins,
        target_is_match_at_end: target_mae.into_boxed_slice(),
        target_is_match: target_match.into_boxed_slice(),
        target_deferred_asserts: target_da.into_boxed_slice(),
        reachable_without_break: rwb.into_boxed_slice(),
        max_instance_stride,
        all_counters_rangeable,
        effects_exact: break_tails_exact,
        // Placeholders — filled in below.
        target_effects: Box::new([]),
        origin_effects: Box::new([]),
        assert_chain_arena: tier3_effects::AssertChainArena::new(),
        break_effects: Box::new([]),
    };

    // -- Step 9: build BreakEffects table and assertion chain arena -------------
    // Build the per-counter break-effects table from the side data computed
    // in Step 2b.  Intern assertion chains into the arena so that all chain
    // IDs are available before compiled effects are generated in Step 10.
    let mut assert_chain_arena = tier3_effects::AssertChainArena::new();
    {
        let mut break_effects_vec: Vec<tier3_effects::BreakEffects> = Vec::new();
        let mut counter_to_id: Vec<tier3_effects::BreakEffectsId> =
            vec![tier3_effects::BreakEffectsId::NONE; 64];

        for target in analysis.targets.iter_mut().flatten() {
            if let Tier3OriginKind::Increment {
                counter,
                break_effects_id,
                ..
            } = target
            {
                let c = counter.idx();
                if counter_to_id[c] == tier3_effects::BreakEffectsId::NONE {
                    let pcd = per_counter_break[c]
                        .take()
                        .unwrap_or_else(|| PerCounterBreakData {
                            deferred_asserts: Box::new([]),
                            all_tails: Box::new([]),
                            pure_tails: Box::new([]),
                            per_tail_deferred: Box::new([]),
                        });

                    // Intern each deferred assert as an independent
                    // 1-element chain (OR semantics: any one passing = match).
                    // Bug 51: these must be separate chains, not one AND chain.
                    let break_deferred_chain_ids: Box<[tier3_effects::AssertChainId]> = pcd
                        .deferred_asserts
                        .iter()
                        .map(|&assert_idx| assert_chain_arena.intern(&[assert_idx]))
                        .collect::<Vec<_>>()
                        .into_boxed_slice();

                    // Intern per-tail deferred assertion chains.
                    let mut per_tail_with_chains = pcd.per_tail_deferred.into_vec();
                    for dt in per_tail_with_chains.iter_mut() {
                        dt.chain_id = assert_chain_arena.intern(&dt.assert_states);
                    }
                    let per_tail_boxed = per_tail_with_chains.into_boxed_slice();

                    let id = tier3_effects::BreakEffectsId(break_effects_vec.len() as u16);
                    break_effects_vec.push(tier3_effects::BreakEffects {
                        break_deferred_chain_ids,
                        break_consuming_pure: pcd.pure_tails,
                        break_consuming_deferred: per_tail_boxed,
                        break_consuming_states: pcd.all_tails,
                    });
                    counter_to_id[c] = id;
                }
                *break_effects_id = counter_to_id[c];
            }
        }

        analysis.break_effects = break_effects_vec.into_boxed_slice();
    }

    // Populate assert_chain_id on each break seed by interning its
    // deferred_asserts into the arena.  The arena deduplicates, so
    // chains already interned above are reused.
    for bs in analysis.break_seeds.iter_mut() {
        bs.assert_chain_id = assert_chain_arena.intern(&bs.deferred_asserts);
    }

    // -- Step 10: compile typed target effects ---------------------------------
    let (target_effects, origin_effects, effects_exact) =
        tier3_effects::compile_all_target_effects(&analysis, states, &mut assert_chain_arena);
    analysis.target_effects = target_effects;
    analysis.origin_effects = origin_effects;
    analysis.effects_exact &= effects_exact;
    analysis.assert_chain_arena = assert_chain_arena;

    // -- Validate EOI seed invariant ------------------------------------------
    // The finish() EOI seed scan checks whether ANY target with a given counter
    // has a match on its break path, ignoring the seed's specific origin.  This
    // is safe because Tier 3 has exactly one CInc per counter: all Increment
    // targets for the same counter share the same BreakEffects and thus the
    // same on_break atoms.  Validate that invariant here.
    #[cfg(debug_assertions)]
    {
        use std::collections::HashMap;
        let mut counter_break_match: HashMap<usize, Option<bool>> = HashMap::new();
        for effects in analysis.target_effects.iter().flatten() {
            if let tier3_effects::TargetStep::Increment { counter, .. } = &effects.step {
                let has_break_match = effects.on_break.iter().any(|a| {
                    matches!(
                        a,
                        tier3_effects::EffectAtom::Match | tier3_effects::EffectAtom::MatchAtEnd
                    )
                });
                match counter_break_match.get(&counter.idx()) {
                    Some(Some(prev)) => {
                        debug_assert_eq!(
                            *prev, has_break_match,
                            "EOI seed invariant violated: counter {} has targets with \
                             inconsistent break-path match reachability",
                            counter
                        );
                    }
                    _ => {
                        counter_break_match.insert(counter.idx(), Some(has_break_match));
                    }
                }
            }
        }
    }

    analysis
}

// ---------------------------------------------------------------------------
// Transition
// ---------------------------------------------------------------------------

/// Cached DFA transition for `(state, byte)`.
///
/// For non-counting transitions, `no_break` and `with_break` are identical.
/// For counting transitions, the matcher picks `with_break` when any
/// incrementing instance has `value+1 >= min`, otherwise `no_break`.
#[derive(Clone)]
struct Transition {
    /// DFA successor when no instance can break (continue-only closure).
    no_break: DfaStateId,
    no_break_is_match: bool,
    /// DFA successor when at least one instance can break (both closure).
    with_break: DfaStateId,
    with_break_is_match: bool,
    /// True if this transition crosses a CInc node.
    is_counting: bool,
    /// Seeds applied BEFORE the per-instance loop of the step.
    ///
    /// From Phase 1 deferred-assertion resolution: a thread that passes
    /// the assertion enters the counter body before this transition's
    /// byte is consumed.  Its instance is seeded at the body's first
    /// consuming state with value 0 and then takes the step like any
    /// other instance (advance, increment, or die) — on counting and
    /// non-counting transitions alike (Bug 55).
    pre_seeds: Box<[(CounterIdx, StateIdx, u32)]>,
    /// New counter instances from CI nodes reachable WITHOUT following
    /// any CInc break path (always applied after counter increment).
    /// The third element is the initial counter value (0 for fresh seeds,
    /// 1 for seeds from resolved deferred assertions on non-counting
    /// transitions whose origin already consumed the resolving byte).
    seeds: Box<[(CounterIdx, StateIdx, u32)]>,
    /// Parallel arrays: `origin_keys[i]` is the consuming NFA state;
    /// `origin_targets[i]` is the post-consumption NFA target state.
    /// The target's structural action is looked up from
    /// [`Tier3Analysis::target_effects`] at runtime via
    /// `target_effects[origin_targets[i].idx()]`.  The NFA state index
    /// serves as the lookup key into the compiled effects table.
    origin_keys: Box<[StateIdx]>,
    origin_targets: Box<[StateIdx]>,
    /// True if any origin's byte-consumption target reaches `$ → Match`
    /// without going through a CInc node.  This is the "counter-free"
    /// subset of the DFA state's `is_match_at_end` — safe to propagate
    /// for counting transitions, because the `$ → Match` path doesn't
    /// depend on any counter reaching its minimum.
    counter_free_match_at_end: bool,
    /// True if the no-break closure's `is_match_at_end` comes from
    /// counter-free paths (targets of `reachable_without_break` origins,
    /// or epsilon `$ → Match` paths from the start state).
    ///
    /// Used for both counting and non-counting transitions in `step_slow`
    /// and `step_from_dead` (Bug 23), as well as in `finish()`.  The raw
    /// DFA state's `is_match_at_end` may include `$ → Match` from
    /// counter-dependent origins that entered the DFA state via previous
    /// counter breaks (Bug 13) or from post-break tail NFA states that
    /// are only valid when all upstream counters have broken (Bug 23).
    nb_counter_free_mae: bool,
}

impl Transition {
    fn empty() -> Self {
        Self {
            no_break: DfaStateId::UNPOPULATED,
            no_break_is_match: false,
            with_break: DfaStateId::UNPOPULATED,
            with_break_is_match: false,
            is_counting: false,
            pre_seeds: Box::new([]),
            seeds: Box::new([]),
            origin_keys: Box::new([]),
            origin_targets: Box::new([]),
            counter_free_match_at_end: false,
            nb_counter_free_mae: false,
        }
    }
}

// ---------------------------------------------------------------------------
// CounterStorage trait — abstracts over range-compressed and per-instance paths
// ---------------------------------------------------------------------------

/// Trait abstracting the counter-specific operations that differ between
/// range-compressed and per-instance slow paths.
///
/// [`RangeCounters`] implements this with range-compressed entries (one
/// `[min_val, max_val]` range per origin), while [`InstanceCounters`]
/// implements it with individual `(value, origin)` pairs.  The
/// [`step_slow_impl`] macro uses these methods for the operations that
/// diverge between the two paths; shared operations (clear, entries,
/// num_counters, any_live) are called directly as inherent methods
/// since both storage types provide them with identical signatures.
///
/// All methods are `#[inline]` to ensure monomorphization produces code
/// equivalent to the previous hand-written per-path methods.
///
/// Methods that logically operate on entries rather than the storage
/// (e.g. `can_continue`, `can_break`) take `&self` to enable
/// trait-method dispatch from the macro, where the concrete type is
/// inferred from the field receiver.
trait CounterStorage {
    /// The element type stored per counter slot.  Must have an `origin:
    /// StateIdx` field accessible in the macro.
    type Entry;

    /// Seed a new entry at counter `ci` with the given origin and value.
    ///
    /// For [`RangeCounters`] this inserts a single-value range `[v, v]`
    /// (merging if the origin already exists).  For [`InstanceCounters`]
    /// this pushes a new instance after a dedup check.
    fn seed(&mut self, ci: usize, origin: StateIdx, value: u32);

    /// Advance an existing entry to a new origin, preserving its
    /// value / range.  Inserts into `self` (the "next" buffer).
    fn advance(&mut self, ci: usize, entry: &Self::Entry, new_origin: StateIdx);

    /// Returns true if the entry has any value that can continue
    /// (i.e. value + 1 < max after incrementing).
    fn can_continue(&self, entry: &Self::Entry, max: u32) -> bool;

    /// Returns true if the entry has any value that triggers a break
    /// (i.e. value + 1 >= min after incrementing).
    fn can_break(&self, entry: &Self::Entry, min: u32) -> bool;

    /// Insert the continued (incremented) portion of an entry at
    /// `new_origin`.  Only called when [`can_continue`](Self::can_continue)
    /// returned true.
    ///
    /// For ranges: inserts `[min_val+1, min(max_val+1, max-1)]`.
    /// For instances: inserts `Instance { value: value+1, origin }`.
    fn insert_continued(&mut self, ci: usize, entry: &Self::Entry, new_origin: StateIdx, max: u32);
}

// ---------------------------------------------------------------------------
// Per-instance tracking (fallback when range compression is unsound)
// ---------------------------------------------------------------------------

/// A single active counter instance.
///
/// Used by the per-instance fallback path when
/// [`Tier3Analysis::all_counters_rangeable`] is `false` — i.e. when at
/// least one counter body is not epsilon-reachable from the start state
/// and value-0 seeds are not injected on every byte.
#[derive(Clone, Debug)]
struct Instance {
    /// Number of completed iterations (0 when freshly seeded at CI).
    value: u32,
    /// The NFA consuming state this instance is waiting at.
    origin: StateIdx,
}

/// Flat storage for per-instance counter tracking.
///
/// Uses a single flat `Vec<Instance>` with
/// fixed-stride slots per counter.  Counter `i` occupies
/// `data[i * stride .. i * stride + counts[i]]` where `stride` is
/// [`Tier3Analysis::max_instance_stride`].  For bounded repetitions this
/// equals `counter_max * body_origins`; for unbounded repetitions the
/// stride is capped at a compile-time constant, and [`push`](Self::push)
/// is a no-op when the slot is full (safe because unbounded counters
/// break at every increment once `value >= min`, so excess instances
/// are redundant).
///
/// This eliminates inner `Vec` heap allocations, making memory reuse
/// trivial: [`clear`](Self::clear) just zeroes the `counts` array.
/// Double-buffering via [`std::mem::swap`] swaps two flat buffers in O(1).
struct InstanceCounters {
    /// Flat backing storage.  Length = `num_counters * stride`.
    data: Vec<Instance>,
    /// Number of live entries per counter.
    counts: Vec<u32>,
    /// Fixed number of slots per counter (= `max_instance_stride`).
    stride: usize,
}

/// Cap for per-counter instance slots when the repetition is unbounded.
///
/// Unbounded repetitions (`{n,}`, `+`, `*`) have `max = usize::MAX`,
/// so `max * body_origins` overflows.  We cap at a generous value that
/// exceeds any realistic number of live instances for a single counter
/// in the non-rangeable path (where seeds are NOT injected every byte).
const UNBOUNDED_INSTANCE_CAP: usize = 4096;

impl InstanceCounters {
    /// Create a new `InstanceCounters` for `num_counters` counters with
    /// the given stride (max instances per counter).
    fn new(num_counters: usize, stride: usize) -> Self {
        let total = num_counters * stride;
        let mut data = Vec::with_capacity(total);
        // Fill with dummy entries — only `counts[i]` entries are live.
        data.resize(
            total,
            Instance {
                value: 0,
                origin: StateIdx::NONE,
            },
        );
        Self {
            data,
            counts: vec![0; num_counters],
            stride,
        }
    }

    /// Clear all counters (reset live counts to zero).
    #[inline]
    fn clear(&mut self) {
        self.counts.fill(0);
    }

    /// Returns the number of counters.
    #[inline]
    fn num_counters(&self) -> usize {
        self.counts.len()
    }

    /// Returns the live entries for counter `ci`.
    #[inline]
    fn entries(&self, ci: usize) -> &[Instance] {
        let base = ci * self.stride;
        &self.data[base..base + self.counts[ci] as usize]
    }

    /// Returns true if any counter has live entries.
    #[inline]
    fn any_live(&self) -> bool {
        self.counts.iter().any(|&c| c != 0)
    }

    /// Push an instance into counter `ci`.
    ///
    /// If the counter's slot is full (can only happen for unbounded
    /// repetitions capped at [`UNBOUNDED_INSTANCE_CAP`]), the push is
    /// silently dropped.  This is safe because unbounded counters break
    /// at every increment once `value >= min`, so the oldest instances
    /// (which would have already broken) are the ones effectively lost.
    #[inline]
    fn push(&mut self, ci: usize, inst: Instance) {
        let count = self.counts[ci] as usize;
        if count >= self.stride {
            return;
        }
        let base = ci * self.stride;
        self.data[base + count] = inst;
        self.counts[ci] = (count + 1) as u32;
    }

    /// Returns true if counter `ci` already contains an instance with the
    /// given `value` and `origin`.
    #[inline]
    fn contains(&self, ci: usize, value: u32, origin: StateIdx) -> bool {
        let base = ci * self.stride;
        let count = self.counts[ci] as usize;
        for i in 0..count {
            let inst = &self.data[base + i];
            if inst.value == value && inst.origin == origin {
                return true;
            }
        }
        false
    }
}

impl CounterStorage for InstanceCounters {
    type Entry = Instance;

    #[inline]
    fn seed(&mut self, ci: usize, origin: StateIdx, value: u32) {
        if !self.contains(ci, value, origin) {
            self.push(ci, Instance { value, origin });
        }
    }

    #[inline]
    fn advance(&mut self, ci: usize, entry: &Instance, new_origin: StateIdx) {
        if !self.contains(ci, entry.value, new_origin) {
            self.push(
                ci,
                Instance {
                    value: entry.value,
                    origin: new_origin,
                },
            );
        }
    }

    #[inline]
    fn can_continue(&self, entry: &Instance, max: u32) -> bool {
        entry.value + 1 < max
    }

    #[inline]
    fn can_break(&self, entry: &Instance, min: u32) -> bool {
        entry.value + 1 >= min
    }

    #[inline]
    fn insert_continued(&mut self, ci: usize, entry: &Instance, new_origin: StateIdx, _max: u32) {
        if !self.contains(ci, entry.value + 1, new_origin) {
            self.push(
                ci,
                Instance {
                    value: entry.value + 1,
                    origin: new_origin,
                },
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Range-compressed instance tracking
// ---------------------------------------------------------------------------

/// A contiguous range of counter values at a single origin.
///
/// Represents the set of instances `{(v, origin) | min_val <= v <= max_val}`.
/// At steady state, each origin within a counter body accumulates a
/// contiguous range `[0, max-1]`.
///
/// ## When range compression is sound (contiguity proof)
///
/// Range compression is sound when **every `CounterInstance` node is
/// epsilon-reachable from `regex.start`** — i.e. reachable via only
/// `Split` and `CounterInstance` edges, with no consuming states, no
/// `Assert` nodes, and no `CounterIncrement` nodes on the path.  When
/// this holds, the unanchored `.*` loop at the start injects a fresh
/// seed with value 0 at the counter entry on every byte.  The argument
/// then proceeds by induction:
///
/// 1. **Seed injection**: Every step, the unanchored loop injects a
///    fresh seed with value 0 at origin 0 (the body entry point).
///    This establishes value 0 at origin 0 on every step.
///
/// 2. **Advance**: When a byte is consumed, an instance at
///    `(value, origin_i)` moves to `(value, origin_j)` for each
///    `origin_j` reachable from `origin_i` via epsilon closure.
///    Since the set of values at `origin_i` is contiguous `[a, b]`
///    by the inductive hypothesis, the set of values arriving at
///    `origin_j` from this source is also `[a, b]`.  Multiple
///    sources merging at `origin_j` are each contiguous and all
///    anchored at 0, so their union is still contiguous.
///
/// 3. **Continue (increment)**: When a counter body completes, the
///    instance at `(value, final_origin)` becomes `(value + 1, origin_0)`.
///    By the inductive hypothesis, `final_origin` holds `[a, b]`, so
///    origin\_0 receives `[a+1, b+1]`.  Combined with the seed's
///    value 0 already at origin\_0 (from step 1), origin\_0 now has
///    `[0, b+1]` — still contiguous.
///
/// 4. **Break (counter exhausted)**: When `value + 1 > max`, the
///    instance exits the counter entirely.  This removes the top
///    of the range but doesn't fragment it.
///
/// The key insight is that all ranges are "anchored at 0" — they always
/// include value 0 because of the continuous seed injection.  Two ranges
/// that both start at 0 can never have a gap between them; their union
/// is simply `[0, max(b1, b2)]`, which [`RangeCounters::insert`] computes
/// via min/max.  This reduces per-byte cost from O(max\_count) to
/// O(num\_origins) — typically single-digit even for complex bodies.
///
/// ## When range compression is unsound
///
/// The proof breaks down when a `CounterInstance` node is **not**
/// epsilon-reachable from `regex.start`.  In that case, value-0 seeds
/// are not injected on every byte, so ranges need not be anchored at 0
/// and can fragment.  Merging disjoint ranges via min/max creates
/// "phantom" values that do not correspond to any real NFA thread,
/// causing false positives.
///
/// Known cases where epsilon-reachability fails:
///
/// - **`^`-anchored patterns** (e.g. `^(a|aaa){4,4}b`): the
///   `Assert(Start)` node blocks re-seeding after byte 0.
/// - **Consuming prefix before counter** (e.g. `c(a|aaa){4,4}b`):
///   seeds only enter the counter when the prefix byte `c` is consumed.
/// - **Any assertion on the path** to a `CounterInstance` node that can
///   evaluate to `Fail` on some bytes.
///
/// The [`Tier3Analysis::all_counters_rangeable`] flag detects these
/// cases at compile time.  When `false`, the matcher falls back to
/// per-instance tracking via [`Instance`] (O(max\_count) per byte
/// but always correct).
#[derive(Clone, Debug)]
struct InstanceRange {
    /// The NFA consuming state these instances are waiting at.
    origin: StateIdx,
    /// Minimum counter value (inclusive).
    min_val: u32,
    /// Maximum counter value (inclusive).
    max_val: u32,
}

/// Flat storage for range-compressed counter instances.
///
/// Uses a single flat `Vec<InstanceRange>`
/// using fixed-stride slots per counter.  Counter `i` occupies
/// `data[i * stride .. i * stride + counts[i]]` where `stride` is
/// [`Tier3Analysis::max_body_origins`] (the maximum number of distinct
/// consuming NFA states in any counter body).
///
/// This eliminates inner `Vec` heap allocations, making memory reuse
/// trivial: [`clear`](Self::clear) just zeroes the `counts` array.
/// Double-buffering via [`std::mem::swap`] swaps two flat buffers in O(1).
struct RangeCounters {
    /// Flat backing storage.  Length = `num_counters * stride`.
    data: Vec<InstanceRange>,
    /// Number of live entries per counter.
    counts: Vec<u8>,
    /// Fixed number of slots per counter (= `max_body_origins`).
    stride: usize,
}

impl RangeCounters {
    /// Create a new `RangeCounters` for `num_counters` counters with
    /// the given stride (max origins per counter body).
    ///
    /// # Panics
    ///
    /// Panics if `stride > 255`.  The per-counter entry count is stored
    /// as `u8` for compactness (one byte of overhead per counter instead
    /// of eight).  A stride of 256+ would overflow the `u8` count on
    /// insertion.  In practice this requires ≥256 distinct consuming NFA
    /// states inside a single counter body, which is unreachable for any
    /// realistic pattern — alternations of single bytes compile to one
    /// `ByteClass`, and even deeply nested multi-byte alternations
    /// rarely exceed a handful of consuming positions.
    fn new(num_counters: usize, stride: usize) -> Self {
        assert!(
            stride <= u8::MAX as usize,
            "RangeCounters: stride {} exceeds u8::MAX (255); the pattern has \
             too many consuming NFA states in a single counter body",
            stride,
        );
        let total = num_counters * stride;
        let mut data = Vec::with_capacity(total);
        // Fill with dummy entries — only `counts[i]` entries are live.
        data.resize(
            total,
            InstanceRange {
                origin: StateIdx::NONE,
                min_val: 0,
                max_val: 0,
            },
        );
        Self {
            data,
            counts: vec![0; num_counters],
            stride,
        }
    }

    /// Clear all counters (reset live counts to zero).
    #[inline]
    fn clear(&mut self) {
        self.counts.fill(0);
    }

    /// Returns the number of counters.
    #[inline]
    fn num_counters(&self) -> usize {
        self.counts.len()
    }

    /// Returns the live entries for counter `ci`.
    #[inline]
    fn entries(&self, ci: usize) -> &[InstanceRange] {
        let base = ci * self.stride;
        &self.data[base..base + self.counts[ci] as usize]
    }

    /// Returns true if any counter has live entries.
    #[inline]
    fn any_live(&self) -> bool {
        self.counts.iter().any(|&c| c != 0)
    }

    /// Merge a `[min_val, max_val]` range into counter `ci`.
    ///
    /// If an entry with the same origin already exists, fuses the ranges.
    /// Otherwise, appends a new entry.  Each origin appears at most once.
    #[inline]
    fn insert(&mut self, ci: usize, origin: StateIdx, min_val: u32, max_val: u32) {
        debug_assert!(min_val <= max_val);
        let base = ci * self.stride;
        let count = self.counts[ci] as usize;
        // Linear scan — at most `stride` entries (typically 1-4).
        for i in 0..count {
            let r = &mut self.data[base + i];
            if r.origin == origin {
                r.min_val = r.min_val.min(min_val);
                r.max_val = r.max_val.max(max_val);
                return;
            }
        }
        debug_assert!(
            count < self.stride,
            "RangeCounters overflow: counter {ci} has {count} entries, stride = {}",
            self.stride,
        );
        self.data[base + count] = InstanceRange {
            origin,
            min_val,
            max_val,
        };
        self.counts[ci] = (count + 1) as u8;
    }
}

impl CounterStorage for RangeCounters {
    type Entry = InstanceRange;

    #[inline]
    fn seed(&mut self, ci: usize, origin: StateIdx, value: u32) {
        self.insert(ci, origin, value, value);
    }

    #[inline]
    fn advance(&mut self, ci: usize, entry: &InstanceRange, new_origin: StateIdx) {
        self.insert(ci, new_origin, entry.min_val, entry.max_val);
    }

    #[inline]
    fn can_continue(&self, entry: &InstanceRange, max: u32) -> bool {
        // Incremented range is [min_val+1, max_val+1].
        // Continue cap is max - 1 (highest value that can stay in the loop).
        // Can continue if new_min <= continue_cap, i.e. min_val < max - 1.
        entry.min_val < max - 1
    }

    #[inline]
    fn can_break(&self, entry: &InstanceRange, min: u32) -> bool {
        // Any value in the incremented range [min_val+1, max_val+1] >= min.
        entry.max_val + 1 >= min
    }

    #[inline]
    fn insert_continued(
        &mut self,
        ci: usize,
        entry: &InstanceRange,
        new_origin: StateIdx,
        max: u32,
    ) {
        let new_min = entry.min_val + 1;
        let new_max = entry.max_val + 1;
        let continue_cap = max - 1;
        let cont_max = new_max.min(continue_cap);
        self.insert(ci, new_origin, new_min, cont_max);
    }
}

// ---------------------------------------------------------------------------
// Tier 3 DFA cache
// ---------------------------------------------------------------------------

/// Lazy DFA cache for Tier 3.
pub(crate) struct Tier3DfaCache {
    inner: DfaCache,
    stride: usize,
    transitions: Vec<Transition>,
    start_seeds: Box<[(CounterIdx, StateIdx, u32)]>,
    /// Which DFA states are idle (start state plus dead body states).
    idle: IdleStates,
}

impl fmt::Debug for Tier3DfaCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tier3DfaCache")
            .field("num_states", &self.inner.states.len())
            .field("max_states", &self.inner.max_states)
            .field("evictions", &self.inner.evictions)
            .finish()
    }
}

impl Tier3DfaCache {
    pub(crate) fn new() -> Self {
        Self {
            inner: DfaCache::new(),
            stride: 256,
            transitions: Vec::new(),
            start_seeds: Box::new([]),
            idle: IdleStates::new(),
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

    /// Whether the states that computing `transitions` transitions can
    /// add still fit: a break and a no-break successor each.
    #[inline]
    fn has_room_for(&self, transitions: usize) -> bool {
        self.inner.has_room(2 * transitions)
    }

    /// Empty the cache, keeping the start state and the states in `keep`,
    /// whose ids are rewritten in place (see [`DfaCache::evict`]).
    #[cold]
    fn evict(&mut self, keep: &mut [DfaStateId]) {
        let stride = self.stride;
        self.transitions.clear();
        self.idle.clear();
        self.inner.evict(keep, || {
            self.transitions
                .extend(std::iter::repeat_with(Transition::empty).take(stride));
        });
    }

    /// Look up or insert a DFA state.  The matcher makes room before it
    /// computes a transition, so the state always fits.
    fn intern_state(
        &mut self,
        nfa_states: &[StateIdx],
        deferred_asserts: &[StateIdx],
        is_match: bool,
        is_match_at_end: bool,
        prev_was_word: bool,
    ) -> DfaStateId {
        let stride = self.stride;
        self.inner
            .intern_state(
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
            .expect("DFA state cap reached although room was made")
    }

    // -----------------------------------------------------------------------
    // Epsilon closure
    // -----------------------------------------------------------------------

    /// Compute epsilon closure.
    ///
    /// `follow_break`: whether to follow the CInc break path (out1).
    /// The continue path (out) is always followed.
    ///
    /// Also tracks CI traversals: when a CI is visited, consuming states
    /// reachable from CI.out are recorded as seed instances.
    #[allow(clippy::too_many_arguments)]
    fn epsilon_closure(
        &mut self,
        memory: &mut DfaMemory,
        seeds: impl Iterator<Item = StateIdx>,
        regex: &Regex,
        analysis: &Tier3Analysis,
        at_start: bool,
        prev_byte: Option<u8>,
        next_byte: Option<u8>,
        follow_break: bool,
    ) -> ClosureResult {
        let mut encountered_cinc = false;

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
            |mem, _, out, out1, _, _| {
                encountered_cinc = true;
                mem.closure_stack.push(out);
                if follow_break {
                    mem.closure_stack.push(out1);
                }
            },
        );

        // Resolve CI seed pairs to (counter, consuming_state) pairs
        // using the precomputed ci_origins table.
        let mut seed_instances: Vec<(CounterIdx, StateIdx)> = Vec::new();
        for &(counter, ci_out) in &memory.closure_seeds {
            for &c in &analysis.ci_origins[ci_out.idx()] {
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

    /// Compute the transition for `(from_state, byte)`.
    fn populate(
        &mut self,
        memory: &mut DfaMemory,
        from: DfaStateId,
        byte: u8,
        regex: &Regex,
        analysis: &Tier3Analysis,
    ) -> Transition {
        // Phase 1: collect targets — NFA states reached after consuming `byte`.
        let mut targets_per_origin: Vec<(StateIdx, Vec<StateIdx>)> = Vec::new();

        let mut resolved_seeds: Vec<(CounterIdx, StateIdx, u32)> = Vec::new();
        let mut resolved_cinc = false;
        let mut resolved_is_match = false;
        // Note: we do NOT track resolved_is_match_at_end here.  That flag
        // means "the deferred assertion passed mid-stream (with next=byte) and
        // $ → Match was reachable."  But the assertion was evaluated with
        // next=Some(byte) — at actual end-of-input, next is None and word-
        // boundary conditions may flip.  finish() correctly re-evaluates via
        // resolve_deferred_at_end().

        if from != DfaStateId::DEAD {
            let from_state = &self.inner.states[from.idx()];

            // Resolve deferred assertions.
            let extra = from_state.resolve_deferred(byte, regex);
            if !extra.is_empty() {
                let resolved_prev = from_state.prev_byte_representative();
                let cr = self.epsilon_closure(
                    memory,
                    extra.into_iter(),
                    regex,
                    analysis,
                    false,
                    resolved_prev,
                    Some(byte),
                    true, // follow_break=true for resolved assert closure
                );
                resolved_seeds = cr
                    .seed_instances
                    .iter()
                    .map(|&(c, s)| (c, s, 0u32))
                    .collect();
                resolved_cinc = cr.encountered_cinc;
                resolved_is_match = cr.is_match;
                // Discard cr.is_match_at_end — see comment above.
                //
                // Phase 1 consumes the resolved closure's NFA states
                // against `byte`.  Their targets go into the transition
                // like any other origin's, which is what lets the
                // instances seeded from `resolved_seeds` take this very
                // step (see `resolved_pre_seeds` below).
                for &idx in &cr.nfa_states {
                    if let Some((t, te)) = consume_byte(idx, byte, regex) {
                        let mut ts = vec![t];
                        if te != StateIdx::NONE {
                            ts.push(te);
                        }
                        targets_per_origin.push((idx, ts));
                    }
                }
            }

            // Phase 2: consuming states in `from` consume `byte`.
            let nfa_states = &self.inner.states[from.idx()].nfa_states;
            for &idx in nfa_states {
                if let Some((t, te)) = consume_byte(idx, byte, regex) {
                    let mut ts = vec![t];
                    if te != StateIdx::NONE {
                        ts.push(te);
                    }
                    targets_per_origin.push((idx, ts));
                }
            }
        }

        let all_targets: Vec<StateIdx> = targets_per_origin
            .iter()
            .flat_map(|(_, ts)| ts.iter().copied())
            .collect();

        // Probe: full "both" closure to detect CInc and collect seeds.
        let probe = self.epsilon_closure(
            memory,
            all_targets
                .iter()
                .copied()
                .chain(std::iter::once(regex.start)),
            regex,
            analysis,
            false,
            Some(byte),
            None,
            true, // follow_break=true
        );

        let is_counting = probe.encountered_cinc || resolved_cinc;

        // Build per-origin target indices from the precomputed analysis.
        //
        // The analysis is indexed by *target* state (the NFA state
        // reached after byte consumption), not by origin.  With dense
        // single-atom encoding, one origin may produce two targets
        // (out + out_exit), so origin_keys may contain duplicates.
        let mut origin_keys = Vec::new();
        let mut origin_targets_vec = Vec::new();
        for &(origin, ref targets) in &targets_per_origin {
            for &target in targets {
                origin_keys.push(origin);
                origin_targets_vec.push(target);
            }
        }

        // Seeds from resolved deferred assertions (e.g. `\b` in front of
        // a counted group) — Bug 55.
        //
        // The assertion was parked on the previous byte and resolves now
        // that `byte` is known, so a thread that passes it enters the
        // counter body *before* `byte` is consumed: its instance must be
        // seeded at the body's first consuming state with value 0 and then
        // take this step like every other instance.  That is exactly what
        // `pre_seeds` do — they are inserted before the per-instance loop
        // of the step, which then advances the instance through this
        // transition's `origin_keys` / `origin_targets`:
        //
        // - the first body state consumes `byte` and the body goes on
        //   (possibly through a split): the instance advances to every
        //   new origin;
        // - it consumes `byte` and completes the body (single-byte body):
        //   the instance is incremented, and can continue or break;
        // - it cannot consume `byte`: the thread is dead and the instance
        //   is dropped.
        //
        // Earlier versions seeded these instances AFTER the step, at an
        // origin patched up case by case (single-byte bodies as pre_seeds,
        // Bug 16's remap to the consumed byte's target, Bug 50's exception
        // for non-consuming targets).  That left the instance at a stale
        // origin whenever the first byte's target was a split, and kept
        // instances alive whose first byte did not match.
        //
        // Break-gated origins are excluded (Bug 40): an origin that is
        // NOT `reachable_without_break` sits behind some counter's break
        // path, and the deferred assertion that led to it may be in the
        // DFA state only because that state over-approximates.  Those
        // seeds are delivered per instance, when the counter actually
        // breaks, through the guarded atoms of `CompiledTargetEffects`.
        // An origin that is reachable without a break is seeded here even
        // if it also appears on a break path (Bug 17).
        let resolved_pre_seeds: Vec<(CounterIdx, StateIdx, u32)> = resolved_seeds
            .iter()
            .filter(|rs| analysis.reachable_without_break[rs.1.idx()])
            .cloned()
            .collect();

        // Compute DFA successors: no_break and with_break.
        if is_counting {
            // Two separate closures.
            let cr_nb = self.epsilon_closure(
                memory,
                all_targets
                    .iter()
                    .copied()
                    .chain(std::iter::once(regex.start)),
                regex,
                analysis,
                false,
                Some(byte),
                None,
                false, // follow_break=false → no_break
            );
            let nb_id = self.intern_closure_result(&cr_nb, byte);

            // with_break reuses the probe closure.
            let wb_id = self.intern_closure_result(&probe, byte);

            let (nb_m, nb_mae) = self.match_flags(nb_id);
            let (wb_m, _wb_mae) = self.match_flags(wb_id);

            let nb_counter_free_mae = self.counter_free_nb_mae(
                memory,
                nb_mae,
                &targets_per_origin,
                regex,
                analysis,
                byte,
            );

            // Unconditional seeds: seeds that are reachable from
            // counter-free origins only.  When the DFA state includes
            // counter-dependent NFA states (from a previous with_break
            // closure), the no-break closure `cr_nb` may produce seeds
            // from CI nodes that are structurally reachable but
            // semantically counter-dependent (Bug 15).
            //
            // To distinguish truly unconditional seeds from
            // counter-dependent ones, we run a separate counter-free
            // no-break closure from only `reachable_without_break`
            // origins (analogous to `counter_free_nb_mae` for
            // match-at-end).  Seeds appearing in that closure are
            // unconditional; seeds in `cr_nb` but NOT in the
            // counter-free closure are counter-dependent and left for
            // the break_seeds mechanism and the post-break tail
            // CInc handoff (which seeds the counter when a
            // post-break tail consumes a byte and reaches CInc).
            let counter_free_seeds: Box<[(CounterIdx, StateIdx)]> = {
                let cf_targets: Vec<StateIdx> = targets_per_origin
                    .iter()
                    .filter(|(origin, _)| analysis.reachable_without_break[origin.idx()])
                    .flat_map(|(_, ts)| ts.iter().copied())
                    .collect();
                let cr_cf_nb = self.epsilon_closure(
                    memory,
                    cf_targets
                        .iter()
                        .copied()
                        .chain(std::iter::once(regex.start)),
                    regex,
                    analysis,
                    false,
                    Some(byte),
                    None,
                    false, // follow_break=false
                );
                cr_cf_nb.seed_instances
            };
            let seeds: Vec<(CounterIdx, StateIdx, u32)> = cr_nb
                .seed_instances
                .iter()
                .filter(|si| counter_free_seeds.iter().any(|cf| cf == *si))
                .map(|&(c, s)| (c, s, 0u32))
                .collect();
            // Break-only seeds: reachable only through CInc break paths.
            // Each seed is tagged with the counter whose CInc break leads
            // to it — the seed is only applied when that specific counter's
            // instance actually breaks.
            //
            // Break seeds are now handled per-instance through the
            // on_break/guarded atoms in CompiledTargetEffects (8F).

            // Compute counter-free match-at-end: true if any origin's
            // target reaches `$ → Match` without going through CInc,
            // AND the origin itself is reachable from the start state
            // without following any CInc break path.
            //
            // The second condition (reachable_without_break) is critical
            // for multi-counter patterns: an origin like Byte('a') in
            // `^c{2,12}.{6,6}(a?)?$` enters the DFA state only after
            // both counters break.  Its `$ → Match` path is locally
            // counter-free, but the origin is counter-dependent.
            // Propagating match_at_end from it would bypass per-instance
            // counter checks, causing false positives (Bug 13).
            //
            // We check all non-Increment origins (`None` and `Advance`).
            // `None` (dead) means the target has no consuming states at all;
            // `Advance` means the target has consuming states but may
            // ALSO reach `$ → Match` via epsilon transitions.  In both
            // cases, the `$ → Match` path doesn't cross any CInc.
            let counter_free_mae =
                origin_keys
                    .iter()
                    .zip(origin_targets_vec.iter())
                    .any(|(&origin, &target)| {
                        !matches!(
                            analysis.targets[target.idx()],
                            Some(Tier3OriginKind::Increment { .. })
                        ) && analysis.target_is_match_at_end[origin.idx()]
                            && analysis.reachable_without_break[origin.idx()]
                    });

            // Fold resolved deferred assertion matches into both
            // successors' flags.  `resolved_is_match` applies
            // unconditionally (the DFA state that was transitioned FROM
            // already encodes the correct counter-aware path).
            Transition {
                no_break: nb_id,
                no_break_is_match: nb_m || resolved_is_match,
                with_break: wb_id,
                with_break_is_match: wb_m || resolved_is_match,
                is_counting: true,
                pre_seeds: resolved_pre_seeds.into(),
                seeds: seeds.into(),
                origin_keys: origin_keys.into_boxed_slice(),
                origin_targets: origin_targets_vec.into_boxed_slice(),
                counter_free_match_at_end: counter_free_mae,
                nb_counter_free_mae,
            }
        } else {
            // Non-counting: both successors are the same.  All seeds are
            // unconditional (no CInc break distinction).
            let id = self.intern_closure_result(&probe, byte);
            let (m, mae) = self.match_flags(id);

            let seeds: Vec<(CounterIdx, StateIdx, u32)> = probe
                .seed_instances
                .iter()
                .map(|&(c, s)| (c, s, 0u32))
                .collect();

            let nb_counter_free_mae =
                self.counter_free_nb_mae(memory, mae, &targets_per_origin, regex, analysis, byte);

            Transition {
                no_break: id,
                no_break_is_match: m || resolved_is_match,
                with_break: id,
                with_break_is_match: m || resolved_is_match,
                is_counting: false,
                pre_seeds: resolved_pre_seeds.into(),
                seeds: seeds.into(),
                origin_keys: origin_keys.into_boxed_slice(),
                origin_targets: origin_targets_vec.into_boxed_slice(),
                counter_free_match_at_end: false, // Not used for non-counting transitions.
                nb_counter_free_mae,
            }
        }
    }

    /// Compute counter-free no-break `is_match_at_end`.
    ///
    /// Filters `targets_per_origin` to only counter-free origins
    /// ([`Tier3Analysis::reachable_without_break`]), runs an epsilon closure
    /// on those targets, and returns `is_match_at_end` from the result.
    /// Returns `false` immediately when `mae` is `false` (the full no-break
    /// closure already has no match-at-end, so the filtered version can't
    /// either).
    ///
    /// Used by both the counting and non-counting branches of [`populate`]
    /// to avoid false positives from origins that entered the DFA state
    /// via previous counter breaks (Bug 13).
    fn counter_free_nb_mae(
        &mut self,
        memory: &mut DfaMemory,
        mae: bool,
        targets_per_origin: &[(StateIdx, Vec<StateIdx>)],
        regex: &Regex,
        analysis: &Tier3Analysis,
        byte: u8,
    ) -> bool {
        if !mae {
            return false;
        }
        let cf_targets: Vec<StateIdx> = targets_per_origin
            .iter()
            .filter(|(origin, _)| analysis.reachable_without_break[origin.idx()])
            .flat_map(|(_, ts)| ts.iter().copied())
            .collect();
        // Note: we intentionally do NOT bail out when cf_targets is empty.
        // The re-seeded `regex.start` is always counter-free (it represents
        // the unanchored match restart, not a counter break), so its epsilon
        // closure may contribute `$ → Match` even when no counter-free
        // consuming origins exist.  The previous guard checked
        // `reachable_without_break[regex.start.idx()]`, but that array only
        // marks *consuming* states; the start state (typically a Split) is
        // never marked, causing false negatives for patterns like
        // `([b-ed-ie-f].{4,43})?$` where the `?`-skip to `$` is the only
        // counter-free path to Match.
        let cr_cf = self.epsilon_closure(
            memory,
            cf_targets
                .iter()
                .copied()
                .chain(std::iter::once(regex.start)),
            regex,
            analysis,
            false,
            Some(byte),
            None,
            false,
        );
        cr_cf.is_match_at_end
    }

    /// Intern a closure result into the state table.  Returns DEAD if
    /// it is empty.
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
    }

    /// Get match flags for a DFA state ID.
    fn match_flags(&self, id: DfaStateId) -> (bool, bool) {
        if id == DfaStateId::DEAD {
            (false, false)
        } else {
            let s = &self.inner.states[id.idx()];
            (s.is_match, s.is_match_at_end)
        }
    }

    // -----------------------------------------------------------------------
    // Cache management
    // -----------------------------------------------------------------------

    fn clear(&mut self, memory: &mut DfaMemory, num_nfa_states: usize, stride: usize) {
        self.inner.clear();
        self.stride = stride;
        memory.clear(num_nfa_states);
        self.transitions.clear();
        self.start_seeds = Box::new([]);
        self.idle.clear();
    }

    /// Prepare the cache for `regex`.
    pub(crate) fn prepare(
        &mut self,
        memory: &mut DfaMemory,
        regex: &Regex,
        analysis: &Tier3Analysis,
    ) {
        let id = regex.id;
        if self.inner.regex_id == id && self.inner.start_id != DfaStateId::DEAD {
            return;
        }
        self.clear(memory, regex.states.len(), regex.num_byte_classes);
        self.inner.regex_id = id;

        let cr = self.epsilon_closure(
            memory,
            std::iter::once(regex.start),
            regex,
            analysis,
            true,
            None,
            None,
            true, // follow_break=true for start closure
        );
        self.inner.start_id = self.intern_state(
            &cr.nfa_states,
            &cr.deferred_asserts,
            cr.is_match,
            cr.is_match_at_end,
            false,
        );
        self.inner.start_is_match = self.inner.states[self.inner.start_id.idx()].is_match;
        self.inner.start_is_match_at_end =
            self.inner.states[self.inner.start_id.idx()].is_match_at_end;
        self.start_seeds = cr
            .seed_instances
            .iter()
            .map(|&(c, s)| (c, s, 0u32))
            .collect();
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

/// Try to consume `byte` at NFA state `idx`.  Returns `(out, out_exit)`
/// where `out_exit` is `StateIdx::NONE` when there is no exit branch.
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

/// Find all consuming NFA states reachable from `start` through epsilon
/// transitions.
fn consuming_states_from(start: StateIdx, states: &[State]) -> Vec<StateIdx> {
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
    result
}

/// Analyze a single post-consumption target state.
///
/// Performs one DFS from `target` through epsilon transitions (Split,
/// Assert, CI) to determine the structural action:
///
/// - If a `CounterIncrement` is reachable, returns `Increment` with
///   advance origins (consuming states before CInc), continue origins
///   (consuming states on the CInc continue path), and break match flags.
/// - Otherwise, returns `Advance` with consuming states reachable, or
///   `None` if dead (no consuming states reachable).
fn analyze_target(
    target: StateIdx,
    states: &[State],
    can_reach_match: &[bool],
) -> Option<Tier3OriginKind> {
    // Single DFS: walk through Split, Assert, CI.
    // - Consuming states reached without crossing CInc → advance_origins.
    // - CInc encountered → record (min, max, continue_out, break_out).
    let mut advance_origins = Vec::new();
    let mut found_cinc: Option<(CounterIdx, usize, usize)> = None;
    let mut cinc_continue_outs = Vec::new();
    let mut cinc_break_outs = Vec::new();

    let mut stack = vec![target];
    let mut visited = vec![false; states.len()];
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
            State::CounterIncrement {
                counter,
                min,
                max,
                out,
                out1,
            } => {
                found_cinc = Some((counter, min, max));
                cinc_continue_outs.push(out);
                cinc_break_outs.push(out1);
            }
            State::Byte { .. }
            | State::ByteCI { .. }
            | State::Wildcard { .. }
            | State::ByteClassStatic { .. }
            | State::ByteClassCustom { .. }
            | State::ByteTable { .. } => {
                advance_origins.push(idx);
            }
            State::Match => {}
        }
    }

    // Bug 37: compute `$ → Match` reachability for the Advance case
    // with a separate epsilon walk that mirrors the static
    // target_is_match_at_end logic.  The walk follows Split and CI
    // but does NOT follow non-End Assert states (they may block the
    // path, e.g. `\B` before `$`).  Only Assert(End) with
    // can_reach_match sets the flag.
    let advance_is_match_at_end = {
        let mut mae = false;
        let mut estack = vec![target];
        let mut evisited = vec![false; states.len()];
        while let Some(eidx) = estack.pop() {
            let ei = eidx.idx();
            if evisited[ei] {
                continue;
            }
            evisited[ei] = true;
            match states[eidx] {
                State::Split { out, out1 } => {
                    estack.push(out1);
                    estack.push(out);
                }
                State::Assert { kind, out } => {
                    if kind == AssertKind::End && can_reach_match[out.idx()] {
                        mae = true;
                    }
                    // Do NOT follow non-End Assert states: they may
                    // block the `$ → Match` path (e.g. `\B$` only
                    // matches when `\B` passes).
                }
                State::CounterInstance { out, .. } => estack.push(out),
                _ => {} // stop at consuming states, CInc, Match
            }
        }
        mae
    };

    // Bug 47: compute direct `Match` reachability for the Advance case.
    // Mirrors the static target_is_match logic but works per-target,
    // handling ByteTable origins whose per-byte targets differ.
    // The walk follows Split and CI but stops at Assert states (Match
    // behind assertions is NOT unconditional).
    let advance_is_match = {
        let mut im = false;
        let mut estack = vec![target];
        let mut evisited = vec![false; states.len()];
        while let Some(eidx) = estack.pop() {
            let ei = eidx.idx();
            if evisited[ei] {
                continue;
            }
            evisited[ei] = true;
            match states[eidx] {
                State::Split { out, out1 } => {
                    estack.push(out1);
                    estack.push(out);
                }
                State::CounterInstance { out, .. } => estack.push(out),
                State::Match => {
                    im = true;
                }
                // Stop at Assert, consuming states, CInc.
                _ => {}
            }
        }
        im
    };

    if let Some((counter, min, max)) = found_cinc {
        advance_origins.sort_unstable();
        advance_origins.dedup();

        // Continue origins: consuming states reachable from CInc continue
        // outputs (no nested CInc in tier 3).
        let mut continue_origins = Vec::new();
        for co in &cinc_continue_outs {
            let consuming = consuming_states_from(*co, states);
            continue_origins.extend(consuming);
        }
        continue_origins.sort_unstable();
        continue_origins.dedup();

        // Break match flags: walk from CInc break outputs through epsilon
        // transitions to find Match / ($ → Match).
        let bc = break_closure(&cinc_break_outs, states, can_reach_match);

        Some(Tier3OriginKind::Increment {
            counter,
            advance_origins: advance_origins.into_boxed_slice(),
            min: min as u32,
            max: max as u32,
            continue_origins: continue_origins.into_boxed_slice(),
            break_is_match: bc.is_match,
            break_is_match_at_end: bc.is_match_at_end,
            // Populated during compute_tier3_analysis after the
            // BreakEffects table is built.
            break_effects_id: tier3_effects::BreakEffectsId::NONE,
        })
    } else if advance_origins.is_empty() {
        if advance_is_match_at_end || advance_is_match {
            // Bug 39/47: no consuming states downstream but `$ → Match`
            // or `Match` IS reachable.  Return an Advance with empty
            // origins so that post-break tails pick up the byte-specific
            // `is_match_at_end` / `is_match` flags.  Previously returned
            // `None`, which lost the information for ByteTable tails
            // (the static arrays skip ByteTable states).
            Some(Tier3OriginKind::Advance {
                new_origins: Box::new([]),
                is_match_at_end: advance_is_match_at_end,
                is_match: advance_is_match,
            })
        } else {
            // Truly dead — no consuming states and no `$ → Match`.
            None
        }
    } else {
        Some(Tier3OriginKind::Advance {
            new_origins: advance_origins.into_boxed_slice(),
            is_match_at_end: advance_is_match_at_end,
            is_match: advance_is_match,
        })
    }
}

/// Epsilon closure from CInc break targets only.
///
/// Returns `(is_match, is_match_at_end)` — whether `Match` or `$ → Match`
/// is reachable from the given break-path seeds **without** passing through
/// another `CounterInstance`.
///
/// Stopping at `CounterInstance` is critical for multi-counter patterns:
/// when counter A breaks, the break path may enter counter B's region
/// (via CI → body → CInc → break → `$` → `Match`).  But counter B has
/// not yet accumulated any iterations — its break condition hasn't been
/// met.  Propagating `break_is_match_at_end` through CI would cause a
/// false positive: the break of counter A would immediately claim "match
/// at end" even though counter B's minimum count hasn't been reached.
///
/// The runtime counter machinery handles downstream counters precisely:
/// `break_seeds` creates new counter instances for B when A breaks, and
/// `post_break_tails` tracks non-counter consuming states for deferred
/// `$ → Match` detection after additional bytes are consumed.
/// Result of walking the CInc break-path epsilon closure.
struct BreakClosureResult {
    /// True if `Match` is directly reachable without going through any
    /// deferred assertion (\b, \B, etc.).
    is_match: bool,
    /// True if `$ → Match` is reachable without going through any
    /// deferred assertion.  At runtime, this fires unconditionally when
    /// the counter breaks with enough value.
    is_match_at_end: bool,
    /// Deferred assertion NFA state indices found on break paths.
    ///
    /// These are assertions (e.g. `\b`, `\B`) encountered while walking
    /// epsilon transitions from CInc break outputs.  They may lead to
    /// either `Match` directly (e.g. `\b → Match`) or to `$ → Match`
    /// (e.g. `\b → $ → Match`).
    ///
    /// At runtime, when the counter breaks with enough value, these are
    /// emitted as [`PendingEffect`](tier3_effects::PendingEffect) entries
    /// with `EffectAtom::Match` and the assertion chain as a guard.  The
    /// pending-effect system evaluates them at the next byte boundary
    /// (mid-input) or at end-of-input via `eval_assert_chain` with
    /// downstream reachability checks.
    deferred_asserts: Box<[StateIdx]>,
}

fn break_closure(
    break_seeds: &[StateIdx],
    states: &[State],
    can_reach_match: &[bool],
) -> BreakClosureResult {
    let mut is_match = false;
    let mut is_match_at_end = false;
    let mut deferred_asserts = Vec::new();

    // Walk epsilon transitions from break seeds.
    //
    // Track whether we've passed through a deferred assertion to
    // correctly attribute `Match` and `$ → Match` discoveries:
    // - Pure paths (no deferred assertion): set is_match / is_match_at_end
    // - Paths through deferred assertions: record the assertion indices
    //   for runtime evaluation; don't set the static flags.
    let mut stack: Vec<(StateIdx, bool)> = break_seeds.iter().map(|&s| (s, false)).collect();

    let n = states.len();
    let mut visited_pure = vec![false; n];
    let mut visited_deferred = vec![false; n];

    while let Some((idx, through_deferred)) = stack.pop() {
        let i = idx.idx();
        let visited = if through_deferred {
            &mut visited_deferred
        } else {
            &mut visited_pure
        };
        if visited[i] {
            continue;
        }
        visited[i] = true;

        match states[idx] {
            State::Split { out, out1 } => {
                stack.push((out1, through_deferred));
                stack.push((out, through_deferred));
            }
            State::Assert { kind, out } => {
                if kind == AssertKind::End {
                    if !through_deferred && can_reach_match[out.idx()] {
                        // Pure $ → Match — safe to fire unconditionally.
                        is_match_at_end = true;
                    }
                    // $ → Match through a deferred assertion is handled
                    // by the deferred_asserts list + finish() evaluation.
                } else if can_reach_match[out.idx()] {
                    if !through_deferred {
                        // First deferred assertion on this path — record
                        // it as the entry point.  Subsequent assertions
                        // deeper in the chain (e.g. \B → \b → $) are
                        // evaluated dynamically by can_reach_match_at_end
                        // when this entry point is checked in finish().
                        deferred_asserts.push(idx);
                    }
                    // Follow through regardless, so we detect Match /
                    // $ → Match reachability from deeper in the chain.
                    stack.push((out, true));
                }
            }
            State::Match => {
                if !through_deferred {
                    is_match = true;
                }
                // Match through a deferred assertion: handled by the
                // deferred_asserts list + finish() evaluation.
            }
            // Do NOT follow through CounterInstance — downstream
            // counters have not accumulated their required count.
            State::CounterInstance { .. } => {}
            _ => {}
        }
    }

    deferred_asserts.sort_unstable();
    deferred_asserts.dedup();
    BreakClosureResult {
        is_match,
        is_match_at_end,
        deferred_asserts: deferred_asserts.into_boxed_slice(),
    }
}

/// Collect consuming NFA states on a CInc break path that are "true tail"
/// states — their byte-consumption target does NOT lead to another CInc.
///
/// These states need runtime tracking to detect deferred `$ → Match` after
/// additional bytes.  Consuming states whose targets lead to CInc are
/// handled by the counter instance seeding/tracking machinery instead.
/// Returns `(all_consuming, pure_consuming, per_tail_deferred)`:
/// - `all_consuming`: all consuming states reachable from the break path
///   (including those behind deferred assertions).
/// - `pure_consuming`: consuming states reachable WITHOUT passing through
///   any non-End Assert state (i.e. on pure epsilon paths).
/// - `per_tail_deferred`: for each consuming state in `all_consuming`,
///   the deferred assertion NFA state indices on the path to reach it.
///   Bug 46: used at runtime to gate each tail individually, not just
///   by the top-level `break_deferred_asserts`.
///   Multiple entries may share the same `origin` with different assertion
///   chains (OR alternatives — Bug 51/52 lesson applied to tails).
///   Each entry becomes a separate `GuardedEffect` with `AddTail`.
///   Only minimal assertion sets are listed (Bug 49): a chain that
///   contains another chain for the same origin is implied by it.
///
/// Bug 45: pure consuming states should be deposited immediately at runtime,
/// while deferred-only states need assertion resolution first.
///
/// Returns `None` when the break path has too many alternative assertion
/// sets to enumerate (see
/// [`tier3_effects::consuming_by_assertion_set`]).  Bug 59: the walk used
/// to prune by chain length only, which neither bounded it nor ended it
/// on an epsilon cycle.
#[allow(clippy::type_complexity)]
fn break_consuming_tails(
    break_seeds: &[StateIdx],
    states: &[State],
    targets: &[Option<Tier3OriginKind>],
) -> Option<(
    Vec<StateIdx>,
    Vec<StateIdx>,
    Vec<tier3_effects::DeferredTail>,
)> {
    let reached = tier3_effects::consuming_by_assertion_set(break_seeds, states, |kind| {
        // End assertions are NOT deferred — they're handled statically
        // by break_is_match_at_end.  Every other kind becomes a guard,
        // evaluated at the next byte.
        kind != AssertKind::End
    })?;

    let mut all_result = Vec::new();
    let mut pure_result = Vec::new();
    let mut per_tail: Vec<tier3_effects::DeferredTail> = Vec::new();
    for r in reached {
        // Only include a state if at least one of its targets is NOT an
        // Increment.  Increment targets are handled by counter seeding.
        let is_tail = match states[r.origin] {
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => {
                let all_targets_inc =
                    matches!(targets[out.idx()], Some(Tier3OriginKind::Increment { .. }))
                        && (out_exit == StateIdx::NONE
                            || matches!(
                                targets[out_exit.idx()],
                                Some(Tier3OriginKind::Increment { .. })
                            ));
                !all_targets_inc
            }
            _ => true,
        };
        if !is_tail {
            continue;
        }
        if r.asserts.is_empty() && !pure_result.contains(&r.origin) {
            pure_result.push(r.origin);
        }
        if !all_result.contains(&r.origin) {
            all_result.push(r.origin);
        }
        // The chain_id is a placeholder (NONE) — populated after the
        // assertion chain arena is built in Step 9.
        per_tail.push(tier3_effects::DeferredTail {
            origin: r.origin,
            assert_states: r.asserts.into_boxed_slice(),
            chain_id: tier3_effects::AssertChainId::NONE,
        });
    }
    all_result.sort_unstable();
    pure_result.sort_unstable();
    Some((all_result, pure_result, per_tail))
}

// ---------------------------------------------------------------------------
// Tier 3 Matcher
// ---------------------------------------------------------------------------

/// Number of consecutive prefilter probes over which its effectiveness is
/// judged (see [`Tier3DfaMatcher::rearm_cooldown`]).
const PREFILTER_PROBE_WINDOW: u32 = 32;

/// Minimum average number of bytes a prefilter probe must skip for the
/// prefilter to stay armed.  Below this the bookkeeping around each probe
/// costs more than stepping the skipped bytes through the DFA.
const PREFILTER_MIN_AVG_SKIP: u32 = 4;

/// Number of input bytes for which an ineffective prefilter is left
/// disarmed before it is tried again.
const PREFILTER_COOLDOWN: usize = 4096;

/// Tier 3 DFA matcher.
///
/// Two instance-tracking strategies are available, selected once at
/// construction by [`Tier3Analysis::all_counters_rangeable`]:
///
/// - **Range-compressed** ([`RangeCounters`]):  O(num_origins) per byte.
///   Used when every `CounterInstance` is epsilon-reachable from start,
///   guaranteeing contiguous value ranges (see proof on [`InstanceRange`]).
/// - **Per-instance** ([`InstanceCounters`]):  O(live_instances) per byte.
///   Fallback when some counter is gated behind a consuming prefix or
///   assertion that blocks continuous seed injection.
pub struct Tier3DfaMatcher<'a> {
    cache: &'a mut Tier3DfaCache,
    memory: &'a mut DfaMemory,
    regex: &'a Regex,
    analysis: &'a Tier3Analysis,
    current: DfaStateId,
    /// The no-break DFA state from the last transition.  Used in `finish()`
    /// to resolve deferred assertions at end-of-input.
    ///
    /// The with-break DFA state includes deferred assertions (e.g. `\b`,
    /// `\B`) from ALL CInc break paths — including counters that haven't
    /// reached their minimum.  `resolve_deferred_at_end` on the with-break
    /// state would evaluate those assertions and follow through to
    /// `$ → Match`, causing false positives.  The no-break state only
    /// includes deferred assertions reachable without this transition's
    /// counter breaks, which is the counter-free subset safe for EOI
    /// resolution.  Counter-dependent `$ → Match` through deferred
    /// assertions is handled by per-instance `break_is_match_at_end`.
    no_break_current: DfaStateId,
    /// Counter-free subset of the no-break DFA state's `is_match_at_end`.
    /// Only includes `$ → Match` from targets of `reachable_without_break`
    /// origins.  Used in `finish()` instead of the raw DFA state flag
    /// to avoid false positives from counter-dependent origins (Bug 13).
    last_nb_counter_free_mae: bool,
    /// `true` when range-compressed instance tracking is active.
    use_ranges: bool,
    /// Range-compressed instance storage — flat per-counter slots.
    /// Active when `use_ranges` is true.
    ranged_counters: RangeCounters,
    /// Scratch buffer for the next ranged step (double-buffered).
    next_ranged: RangeCounters,
    /// Per-instance flat storage — fixed-stride slots per counter.
    /// Active when `use_ranges` is false.
    inst_counters: InstanceCounters,
    /// Scratch buffer for the next per-instance step (double-buffered).
    next_instances: InstanceCounters,
    ever_matched: bool,
    match_at_end: bool,
    has_live_instances: bool,
    /// NFA consuming states currently active from a counter break path.
    /// These track the post-counter "tail" (e.g. a trailing `.` or
    /// literal before `$ → Match`).  Updated each step: each tail is
    /// advanced through its target action.  When a tail's action is
    /// `None` (dead), match flags are checked via `origin_effects`.
    ///
    /// This replaces the use of DFA-level `is_match_at_end` for counting
    /// transitions, which is overly optimistic because the DFA state
    /// includes post-break states before counters have actually broken.
    post_break_tails: Vec<StateIdx>,
    /// Scratch buffer for advancing `post_break_tails` (double-buffered).
    next_post_break_tails: Vec<StateIdx>,
    // Break seeds, break tails, resolved tails/mae, and deferred assertion
    // match signals are all handled by `pending_effects_current` and
    // `pending_effects_next` via the typed effect system.  See
    // `tier3_effects::resolve_pending()` and `tier3_effects::PendingEffect`.
    /// True when the current DFA state (`self.current`) was reached via a
    /// `with_break` transition and may contain NFA consuming states that
    /// entered through a counter break path.  When true, the precomputed
    /// `counter_free_match_at_end` and `nb_counter_free_mae` flags on
    /// transitions FROM this state are unreliable — they may include
    /// `$ → Match` paths from origins that are statically
    /// `reachable_without_break` but entered this DFA state via a counter
    /// break (Bug 19).
    current_has_break_extras: bool,
    /// The "clean" no-break DFA state chain: always computed from the
    /// previous clean_nb state, never from a contaminated state.
    /// Used as the reference for which NFA states are genuinely reachable
    /// without counter breaks (Bug 22).
    /// Updated each step: `clean_nb = no_break_successor(clean_nb, byte)`.
    clean_nb: DfaStateId,
    /// The `nb_counter_free_mae` from the clean chain's last transition.
    /// When contaminated, this replaces the old `clean_counter_free_mae(t)`
    /// which was incorrectly checking origins against the post-transition
    /// `clean_nb` state (Bug 24).  The clean chain's transition is from
    /// a guaranteed-clean DFA state, so its `nb_counter_free_mae` correctly
    /// indicates whether a counter-free `$ → Match` path exists.
    clean_nb_cf_mae: bool,
    /// The `no_break_is_match` from the clean chain's last transition.
    /// When contaminated, the main transition's `no_break_is_match` may
    /// include `Match` from break-gated origins (e.g. `Byte('f') → Match`
    /// entered via a counter break) that haven't met their counter minimum
    /// (Bug 25).  The clean chain's transition is from a guaranteed-clean
    /// DFA state, so its `no_break_is_match` only reflects genuinely
    /// counter-free `Match` paths.
    clean_nb_is_match: bool,
    /// Transition-table slot for the clean_nb chain's transition on the
    /// current byte.  Set when the current state is contaminated
    /// (`current_has_break_extras`) and used in `step_slow` to filter
    /// unconditional seeds on **counting** transitions: only seeds that
    /// also appear in the clean_nb transition are truly counter-free
    /// (Bug 32).  `None` when clean_nb is DEAD or the state is not
    /// contaminated.
    clean_nb_trans_slot: Option<usize>,
    prefilter: Prefilter,
    /// True when the matcher holds exactly the start state's threads, so
    /// that the next byte may be handled as if the stream began there:
    /// the DFA state is put back to the start state and, if that byte
    /// cannot begin a match, the prefilter skips ahead.
    ///
    /// Set at construction and whenever the matcher becomes quiescent
    /// (see [`settle`](Self::settle)); consumed at the next byte.  Nothing
    /// is rewritten when the flag is set — only when it is consumed — so
    /// [`finish`](Self::finish) never observes a half-restarted matcher.
    fresh: bool,
    /// True when the matcher may become [`fresh`](Self::fresh) again
    /// mid-stream.  This requires a start state that is the same at every
    /// input position and owns no counter instance: the start closure has
    /// no assertions and no counters, and cannot match empty.  Patterns
    /// that start with a counter are not restartable — their initial state
    /// carries seeded instances — so their prefilter stays one-shot.
    restartable: bool,
    /// `restartable` and a prefilter exists: landing on the start state is
    /// worth noticing because the prefilter can then skip input.  Without
    /// a prefilter the start state needs no special handling.
    rearm: bool,
    /// Number of probes in the current effectiveness window.
    prefilter_probes: u32,
    /// Bytes skipped by the probes of the current window.
    prefilter_skipped: u32,
    /// Input bytes still to be stepped through the DFA before the start
    /// state is watched again.
    ///
    /// Re-arming the prefilter pays off when candidates are sparse.  On
    /// input where most bytes are candidates that fail at once, every
    /// probe skips nothing and the data-dependent "back at start?" test
    /// only adds work.  After [`PREFILTER_PROBE_WINDOW`] probes that
    /// skipped fewer than [`PREFILTER_MIN_AVG_SKIP`] bytes on average,
    /// the matcher steps the next [`PREFILTER_COOLDOWN`] bytes without
    /// looking for the start state, then tries again.
    rearm_cooldown: usize,
    /// Pending effects to resolve at the next byte boundary.
    ///
    /// This buffer is the **read side** of a swap-buffer pair.  At each
    /// byte boundary in `chunk()`:
    ///
    /// 1. `pending_effects_current` is resolved by
    ///    [`tier3_effects::resolve_pending()`].
    /// 2. Any second-order deferred effects deposited during resolution
    ///    go into [`pending_effects_next`].
    /// 3. `pending_effects_current` is cleared, then swapped with
    ///    `pending_effects_next` — so second-order effects survive to
    ///    the following boundary.
    /// 4. `step_slow` appends first-order effects to
    ///    `pending_effects_current` (which now contains only surviving
    ///    second-order effects from step 3).
    ///
    /// This two-buffer design prevents second-order deferred effects
    /// (deposited during resolution of a resolved tail or seed) from
    /// being dropped by the post-resolution clear.
    pending_effects_current: Vec<tier3_effects::PendingEffect>,
    /// Pending effects deposited **during resolution** of
    /// `pending_effects_current`.
    ///
    /// This is the **write side** of the swap-buffer pair.  After
    /// resolution completes, it is swapped into `pending_effects_current`
    /// for the next boundary.  See [`pending_effects_current`] for the
    /// full lifecycle.
    pending_effects_next: Vec<tier3_effects::PendingEffect>,
    /// Reusable scratch for epsilon-walk reachability checks in
    /// `eval_assert_chain` → `can_reach_match_{mid,at_end}`.
    reach_scratch: super::ReachScratch,
    /// Reusable scratch for tails resolved from pending effects.
    /// Cleared per byte in `chunk()` instead of reallocated.
    resolved_tails: Vec<StateIdx>,
    /// Reusable scratch for the output of [`tier3_effects::resolve_pending()`].
    /// Cleared per call instead of reallocated.
    resolved_actions: tier3_effects::ResolvedActions,
    /// Epoch-stamped membership array for O(1) tail dedup.
    /// Indexed by NFA state index.  `tail_seen[i] == tail_epoch` means
    /// state `i` is already present in the current dedup scope.
    tail_seen: Vec<u32>,
    /// Current epoch for tail dedup.  Bumped at the start of each
    /// dedup scope (next_post_break_tails, resolved_tails, injection).
    tail_epoch: u32,
}

// ---------------------------------------------------------------------------
// step_slow_impl! — generates step_slow_ranged / step_slow_instances
// ---------------------------------------------------------------------------

/// Generates a `step_slow_*` method for a specific [`CounterStorage`] backend.
///
/// Both the range-compressed and per-instance slow paths share identical
/// control flow: advance post-break tails, apply pre-seeds, iterate
/// counter entries, select the DFA successor, swap buffers, and apply
/// seeds.  Only the counter storage operations differ (range merging vs
/// individual instance tracking), which are abstracted through the
/// [`CounterStorage`] trait.
///
/// Parameters:
/// - `$method`:  generated method name (`step_slow_ranged` or `step_slow_instances`)
/// - `$current`: field name for the current-step counter buffer
/// - `$next`:    field name for the next-step scratch buffer
macro_rules! step_slow_impl {
    ($method:ident, $current:ident, $next:ident) => {
        #[inline(never)]
        fn $method(&mut self, slot: usize, byte: u8) {
            let t = &self.cache.transitions[slot];

            // Reset next buffer and match flags.
            // Note: pending_effects_current is NOT cleared here — it may
            // contain second-order deferred effects from the resolution
            // phase that precedes step_slow in the chunk() loop.  New
            // deposits from this step are appended to pending_effects_current.
            self.$next.clear();
            self.match_at_end = false;

            // Advance existing post-break tails through this transition.
            // Each tail is a consuming NFA state from a previous counter
            // break.  If it consumed this byte and its target reaches
            // `$ → Match`, set match_at_end.
            //
            // Dead-target and per-origin effects now use the same
            // immediate/guarded vocabulary as target effects.  Match flags
            // are applied via apply_immediate_atoms(), deferred assertions
            // via enqueue_guarded_effects().
            let mut any_can_break = false;
            // counter_broke bitmask removed in 8F — was only used by the
            // transition-level break_seeds loop.  any_can_break is still
            // used for DFA successor selection.

            self.next_post_break_tails.clear();
            self.tail_epoch = self.tail_epoch.wrapping_add(1);
            for &pbo in &self.post_break_tails {
                // Collect ALL target indices for this origin (may be >1
                // with dense out_exit encoding).
                let target_indices: Vec<usize> = t
                    .origin_keys
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &k)| if k == pbo { Some(i) } else { None })
                    .collect();
                if target_indices.is_empty() {
                    // Origin not found — skip.
                    continue;
                }
                for &pos in &target_indices {
                    match &self.analysis.target_effects[t.origin_targets[pos].idx()] {
                        Some(effects) => {
                            match &effects.step {
                                tier3_effects::TargetStep::Advance { new_origins } => {
                                    for &new_o in new_origins {
                                        if self.tail_seen[new_o.idx()] != self.tail_epoch {
                                            self.tail_seen[new_o.idx()] = self.tail_epoch;
                                            self.next_post_break_tails.push(new_o);
                                        }
                                    }
                                    // Bug 30: deposit per-origin deferred asserts.
                                    tier3_effects::enqueue_guarded_effects(
                                        &mut self.pending_effects_current,
                                        &self.analysis.origin_effects[pbo.idx()].guarded,
                                        byte,
                                    );
                                    // Bug 57: origins behind a deferrable
                                    // assertion are reached only if it holds.
                                    tier3_effects::enqueue_guarded_effects(
                                        &mut self.pending_effects_current,
                                        &effects.guarded,
                                        byte,
                                    );
                                    // Bug 37 + Bug 47: apply byte-specific
                                    // immediate effects from the compiled Advance.
                                    tier3_effects::apply_immediate_atoms(
                                        &effects.immediate,
                                        &mut self.ever_matched,
                                        &mut self.match_at_end,
                                    );
                                }
                                tier3_effects::TargetStep::Increment {
                                    counter,
                                    min,
                                    max,
                                    continue_origins,
                                    ..
                                } => {
                                    // Tail hit a CInc — hand off to the counter
                                    // instance machinery.  The post-break tail
                                    // consumed this byte, entering the CInc for the
                                    // first time (value 0 → incremented to 1).
                                    //
                                    // Insert the continued instance directly into
                                    // `$next` to avoid double-counting: the counter
                                    // loop (below) processes `$current`, and we
                                    // don't want this byte to be counted twice.
                                    let pbt_value: u32 = 0;
                                    // Check continue (value after increment < max).
                                    if pbt_value + 1 < *max {
                                        for &new_o in continue_origins {
                                            self.$next.seed(counter.idx(), new_o, pbt_value + 1);
                                        }
                                    }
                                    // Check break (value after increment >= min).
                                    // Bug 36: the tail→CInc handoff IS a counter
                                    // break — set any_can_break so the DFA
                                    // selects with_break.
                                    if pbt_value + 1 >= *min {
                                        any_can_break = true;
                                        // Apply on_break atoms from CompiledTargetEffects.
                                        for atom in &effects.on_break {
                                            match atom {
                                                tier3_effects::EffectAtom::Match => {
                                                    self.ever_matched = true;
                                                }
                                                tier3_effects::EffectAtom::MatchAtEnd => {
                                                    self.match_at_end = true;
                                                }
                                                tier3_effects::EffectAtom::AddTail { origin } => {
                                                    if self.tail_seen[origin.idx()]
                                                        != self.tail_epoch
                                                    {
                                                        self.tail_seen[origin.idx()] =
                                                            self.tail_epoch;
                                                        self.next_post_break_tails.push(*origin);
                                                    }
                                                }
                                                tier3_effects::EffectAtom::AddSeed {
                                                    counter: sc,
                                                    origin: so,
                                                    value: sv,
                                                } => {
                                                    self.$next.seed(sc.idx(), *so, *sv);
                                                }
                                            }
                                        }
                                        tier3_effects::enqueue_guarded_effects(
                                            &mut self.pending_effects_current,
                                            &effects.guarded,
                                            byte,
                                        );
                                    }
                                }
                            }
                        }
                        None => {
                            // `None` target — apply origin effects (Bug 34).
                            let oe = &self.analysis.origin_effects[pbo.idx()];
                            tier3_effects::apply_immediate_atoms(
                                &oe.immediate,
                                &mut self.ever_matched,
                                &mut self.match_at_end,
                            );
                            tier3_effects::enqueue_guarded_effects(
                                &mut self.pending_effects_current,
                                &oe.guarded,
                                byte,
                            );
                        }
                    }
                } // for &pos in &target_indices
            }

            // Apply pre_seeds BEFORE the per-instance loop, so that the
            // instances they create take this step (see
            // `resolved_pre_seeds` in `populate()`).
            //
            // Bug 48: mirror the Bug 32 guard for unconditional seeds.
            // When the current DFA state is contaminated, pre_seeds may
            // include seeds for counters whose CI state entered the DFA
            // closure only via a counter break (e.g. c1 seeded from a
            // with_break closure that includes c0's break path).  Filter
            // these against the clean no-break chain's pre_seeds: only
            // seeds that also appear in the clean chain are truly
            // counter-free.
            //
            // Bug 56: when the clean chain is dead (`clean_nb_trans_slot`
            // is `None` although the state is contaminated), no thread is
            // counter-free at all, so no pre_seed is.
            let pre_seed_contaminated =
                self.current_has_break_extras && self.regex.num_counters > 1;
            for &(counter, origin, value) in &t.pre_seeds {
                if pre_seed_contaminated {
                    let Some(cn_slot) = self.clean_nb_trans_slot else {
                        continue;
                    };
                    let cn_t = &self.cache.transitions[cn_slot];
                    if !cn_t
                        .pre_seeds
                        .iter()
                        .any(|s| s.0 == counter && s.1 == origin)
                    {
                        continue;
                    }
                }
                self.$current.seed(counter.idx(), origin, value);
            }

            // Note: any_can_break is declared ABOVE the tail loop (Bug 36)
            // so that tail→CInc handoffs that produce a break can set it
            // before the counter loop.
            let num_counters = self.$current.num_counters();
            debug_assert!(num_counters <= 64);

            #[allow(clippy::needless_range_loop)]
            for c_idx in 0..num_counters {
                for entry in self.$current.entries(c_idx) {
                    let origin = entry.origin;
                    // Collect ALL target effects for this origin (may
                    // be >1 with dense out_exit encoding).
                    let effect_indices: Vec<usize> = t
                        .origin_keys
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &k)| if k == origin { Some(i) } else { None })
                        .collect();

                    for &ei in &effect_indices {
                        let effects =
                            self.analysis.target_effects[t.origin_targets[ei].idx()].as_ref();

                        match effects {
                            Some(effects) => match &effects.step {
                                tier3_effects::TargetStep::Advance { new_origins } => {
                                    // An instance advances inside its
                                    // counter body, where Tier 3 admits no
                                    // deferrable assertion.
                                    debug_assert!(effects.guarded.is_empty());
                                    for &new_o in new_origins {
                                        self.$next.advance(c_idx, entry, new_o);
                                    }
                                }
                                tier3_effects::TargetStep::Increment {
                                    advance_origins,
                                    min,
                                    max,
                                    continue_origins,
                                    ..
                                } => {
                                    // Advance-or-increment: entry survives at
                                    // advance_origins with the same values.
                                    for &new_o in advance_origins {
                                        self.$next.advance(c_idx, entry, new_o);
                                    }

                                    // Continue: incremented entry stays in the loop.
                                    if self.$current.can_continue(entry, *max) {
                                        for &new_o in continue_origins {
                                            self.$next.insert_continued(c_idx, entry, new_o, *max);
                                        }
                                    }

                                    // Break: entry has reached the minimum threshold.
                                    if self.$current.can_break(entry, *min) {
                                        any_can_break = true;
                                        // Apply on_break atoms from CompiledTargetEffects.
                                        for atom in &effects.on_break {
                                            match atom {
                                                tier3_effects::EffectAtom::Match => {
                                                    self.ever_matched = true;
                                                }
                                                tier3_effects::EffectAtom::MatchAtEnd => {
                                                    self.match_at_end = true;
                                                }
                                                tier3_effects::EffectAtom::AddTail { origin } => {
                                                    if self.tail_seen[origin.idx()]
                                                        != self.tail_epoch
                                                    {
                                                        self.tail_seen[origin.idx()] =
                                                            self.tail_epoch;
                                                        self.next_post_break_tails.push(*origin);
                                                    }
                                                }
                                                tier3_effects::EffectAtom::AddSeed {
                                                    counter: sc,
                                                    origin: so,
                                                    value: sv,
                                                } => {
                                                    self.$next.seed(sc.idx(), *so, *sv);
                                                }
                                            }
                                        }
                                        tier3_effects::enqueue_guarded_effects(
                                            &mut self.pending_effects_current,
                                            &effects.guarded,
                                            byte,
                                        );
                                    }
                                }
                            },
                            None => {}
                        }
                    } // for &ei in &effect_indices
                }
            }

            // Select DFA successor.
            //
            // Contamination check (Bug 19/22): When the FROM state was
            // reached via a with_break transition and the pattern has ≥2
            // counters, origins from downstream counter break chains may
            // be in the DFA state before those downstream counters have
            // actually reached their minimums.
            self.no_break_current = t.no_break;
            let from_contaminated = self.current_has_break_extras && self.regex.num_counters > 1;
            self.last_nb_counter_free_mae = if from_contaminated {
                self.clean_nb_cf_mae
            } else {
                t.nb_counter_free_mae
            };
            if t.is_counting && any_can_break {
                self.current = t.with_break;
                self.current_has_break_extras = true;
            } else {
                self.current = t.no_break;
                // Propagate contamination: the no-break successor of a
                // contaminated state inherits NFA states that originally
                // entered via counter breaks (Bug 22).
                if !from_contaminated {
                    self.current_has_break_extras = false;
                }
            }
            if !t.is_counting {
                // For non-counting transitions, use counter-free mae just
                // like the fast path (Bug 23).  The raw
                // no_break_is_match_at_end may include $ → Match paths
                // from post-break tail NFA states that only become active
                // after counter breaks — but those counters may not have
                // met their minimums.
                let mae_before = self.match_at_end;
                let cf_mae = if from_contaminated {
                    self.clean_nb_cf_mae
                } else {
                    t.nb_counter_free_mae
                };
                if cf_mae {
                    self.match_at_end = true;
                }
                // Bug 23 guard: match_at_end must only become true via
                // cf_mae, never from the raw DFA state is_match_at_end.
                debug_assert!(
                    !self.match_at_end || mae_before || cf_mae,
                    "Bug 23 guard: non-counting transition set match_at_end \
                     without cf_mae (state={:?})",
                    self.current,
                );
                let m = if any_can_break {
                    t.with_break_is_match
                } else if from_contaminated {
                    // When contaminated and no counter broke, the
                    // no_break_is_match flag may include Match from
                    // break-gated origins that never reached their
                    // counter minimum (Bug 25).  Use the clean chain's
                    // flag instead.  Post-break tail matches are handled
                    // separately via target_is_match in the tail loop.
                    self.clean_nb_is_match
                } else {
                    t.no_break_is_match
                };
                if m {
                    self.ever_matched = true;
                }
            } else {
                // For counting transitions, propagate only no_break_is_match
                // and counter_free_match_at_end.  The full
                // no_break_is_match_at_end may include $ → Match paths from
                // post-break origins that haven't met their counter minimums.
                // Counter-dependent paths are handled by per-instance break
                // checks and the post_break_tails mechanism.
                //
                // When contaminated, the no_break_is_match flag may
                // include Match from break-gated origins that never
                // reached their counter minimum (Bug 25).  Use the
                // clean chain's flag instead.  Per-instance break
                // checks (above) already set ever_matched when an
                // actual counter breaks with break_is_match=true,
                // so no signal is lost.
                let m = if from_contaminated {
                    self.clean_nb_is_match
                } else {
                    t.no_break_is_match
                };
                if m {
                    self.ever_matched = true;
                }
                let cf_mae = if from_contaminated {
                    self.clean_nb_cf_mae
                } else {
                    t.counter_free_match_at_end
                };
                if cf_mae {
                    self.match_at_end = true;
                }
            }

            // Swap buffers (O(1) — just swaps flat Vec pointers).
            std::mem::swap(&mut self.$current, &mut self.$next);
            std::mem::swap(&mut self.post_break_tails, &mut self.next_post_break_tails);

            // Seed new instances (unconditional).
            //
            // Bug 32: on COUNTING transitions when the current DFA state
            // is contaminated (has break extras), the `counter_free_seeds`
            // computation in `populate()` may include seeds from origins
            // that are globally `reachable_without_break` but entered THIS
            // DFA state only via a counter break.  Gate these seeds on the
            // clean_nb chain's transition: only seeds that also appear in
            // the clean chain are truly counter-free.
            //
            // Non-counting transitions are NOT filtered: their seeds come
            // from the full probe closure (follow_break=true).  Filtering
            // them would kill legitimate downstream counter seeds
            // (regression on `^(a{1,17}b){2,3}$` / "abab").
            //
            // Suppressed seeds on counting transitions are still reachable
            // through the per-instance on_break/guarded AddSeed atoms in
            // CompiledTargetEffects (Bug 32 part 2: the per-instance break
            // path fires when the triggering counter actually breaks).
            // When counting + contaminated, build an epoch-stamped set of
            // clean-chain seed origins for O(1) membership instead of
            // O(|cn_t.seeds|) per main seed.
            //
            // Bug 56: the filter applies whenever the FROM state was
            // contaminated, also when the clean chain is dead
            // (`clean_nb_trans_slot` is `None`).  A dead clean chain means
            // that no thread is counter-free, so the set stays empty and
            // every seed is suppressed.  Skipping the filter in that case
            // let a state that is in the DFA state only because of a
            // break that never happened seed a counter.
            let filter_seeds = t.is_counting && from_contaminated;
            if filter_seeds {
                self.tail_epoch = self.tail_epoch.wrapping_add(1);
                if let Some(cn_slot) = self.clean_nb_trans_slot {
                    let cn_t = &self.cache.transitions[cn_slot];
                    for &(_, cn_origin, _) in &cn_t.seeds {
                        self.tail_seen[cn_origin.idx()] = self.tail_epoch;
                    }
                }
            }
            for &(counter, origin, value) in &t.seeds {
                if filter_seeds && self.tail_seen[origin.idx()] != self.tail_epoch {
                    continue;
                }
                self.$current.seed(counter.idx(), origin, value);
            }
            // Break seeds: now handled per-instance through on_break
            // (ungated AddSeed) and guarded (gated AddSeed) atoms in
            // CompiledTargetEffects.  Transition.break_seeds removed in 8F.

            // Update has_live_instances flag.
            self.has_live_instances = self.$current.any_live();
        }
    };
}

impl<'a> Tier3DfaMatcher<'a> {
    pub(crate) fn new(
        cache: &'a mut Tier3DfaCache,
        memory: &'a mut DfaMemory,
        regex: &'a Regex,
        analysis: &'a Tier3Analysis,
    ) -> Self {
        let nc = regex.num_counters;
        let stride = analysis.max_body_origins;
        let use_ranges = analysis.all_counters_rangeable;

        let mut ranged_counters = RangeCounters::new(nc, stride);
        let next_ranged = RangeCounters::new(nc, stride);
        let inst_stride = analysis.max_instance_stride;
        let mut inst_counters = InstanceCounters::new(nc, inst_stride);
        let next_instances = InstanceCounters::new(nc, inst_stride);

        for &(counter, origin, value) in &cache.start_seeds {
            if use_ranges {
                ranged_counters.insert(counter.idx(), origin, value, value);
            } else {
                inst_counters.push(counter.idx(), Instance { value, origin });
            }
        }

        let ever_matched = cache.inner.start_is_match;
        let has_live_instances = !cache.start_seeds.is_empty();
        let restartable = !regex.start_closure.is_empty()
            && cache.start_seeds.is_empty()
            && !cache.inner.start_is_match
            && !cache.inner.start_is_match_at_end;
        let rearm = restartable && !matches!(regex.prefilter, Prefilter::None);

        let mut matcher = Tier3DfaMatcher {
            // The start-state fields below are placeholders: they are set
            // by `enter_start_state()` and `reset_position()`, the same
            // code that puts a quiescent matcher back at the start.
            current: DfaStateId::DEAD,
            no_break_current: DfaStateId::DEAD,
            last_nb_counter_free_mae: false,
            use_ranges,
            ever_matched,
            match_at_end: false,
            has_live_instances,
            post_break_tails: Vec::new(),
            next_post_break_tails: Vec::new(),
            current_has_break_extras: false,
            clean_nb: DfaStateId::DEAD,
            clean_nb_cf_mae: false,
            clean_nb_is_match: false,
            clean_nb_trans_slot: None,
            cache,
            memory,
            regex,
            analysis,
            ranged_counters,
            next_ranged,
            inst_counters,
            next_instances,
            prefilter: regex.prefilter,
            fresh: true,
            restartable,
            rearm,
            prefilter_probes: 0,
            prefilter_skipped: 0,
            rearm_cooldown: 0,
            pending_effects_current: Vec::new(),
            pending_effects_next: Vec::new(),
            reach_scratch: super::ReachScratch::new(),
            resolved_tails: Vec::new(),
            resolved_actions: tier3_effects::ResolvedActions::default(),
            tail_seen: vec![0; regex.states.len()],
            tail_epoch: 0,
        };
        matcher.enter_start_state();
        matcher.reset_position();
        matcher
    }

    /// Point the DFA state and the contamination tracking at the start
    /// state.
    ///
    /// Used at construction and when a [`fresh`](Self::fresh) matcher
    /// consumes its next byte.  In the latter case the matcher already
    /// holds exactly the start state's threads; this drops dead
    /// counter-body states from the DFA state and forgets contamination,
    /// which only describes threads that no longer exist.
    ///
    /// Counter instances, tails and pending effects are not touched: at
    /// construction they hold the start seeds, and a matcher only becomes
    /// fresh again when all of them are empty.
    #[inline]
    fn enter_start_state(&mut self) {
        let inner = &self.cache.inner;
        self.current = inner.start_id;
        // The start state may include NFA states from break paths (the
        // start closure is computed with follow_break=true), but these
        // are freshly seeded — not from a previous counter break.  Treat
        // the start state as uncontaminated.
        self.current_has_break_extras = false;
        self.clean_nb = inner.start_id;
        self.clean_nb_cf_mae = inner.start_is_match_at_end;
        self.clean_nb_is_match = inner.start_is_match;
        self.clean_nb_trans_slot = None;
    }

    /// Give the end-of-input side channels the start state's answers.
    ///
    /// Used at construction and when the prefilter skips input: those
    /// fields describe the last byte stepped through the DFA, which is no
    /// longer the last byte of the input.
    #[inline]
    fn reset_position(&mut self) {
        let inner = &self.cache.inner;
        self.no_break_current = inner.start_id;
        self.last_nb_counter_free_mae = inner.start_is_match_at_end;
        self.match_at_end = inner.start_is_match_at_end;
    }

    step_slow_impl!(step_slow_ranged, ranged_counters, next_ranged);
    step_slow_impl!(step_slow_instances, inst_counters, next_instances);

    // `clean_counter_free_mae` was removed in Bug 24.  The method
    // compared origins against `clean_nb`'s *post-transition* NFA states,
    // which could be empty when the transition's only target was an
    // Assert(End) with no consuming successors — losing the mae signal
    // from legitimate counter-free origins.  Replaced by `clean_nb_cf_mae`,
    // the clean chain transition's `nb_counter_free_mae`, which is
    // computed against the *pre-transition* clean state.

    /// Empty the DFA cache because it is full, keeping the states this
    /// matcher stands on.  Every state id and transition slot the matcher
    /// holds is refreshed or dropped here.
    #[cold]
    fn evict_cache(&mut self) {
        let mut keep = [self.current, self.no_break_current, self.clean_nb];
        self.cache.evict(&mut keep);
        [self.current, self.no_break_current, self.clean_nb] = keep;
        self.clean_nb_trans_slot = None;
    }

    /// Before computing a missing main transition: make room for it, and
    /// for the clean chain's transition if this step takes one, so that
    /// one byte empties the cache at most once.  Returns whether the
    /// cache was emptied.  Out of line: the scanning loop is sensitive to
    /// the code around its lookups.
    #[cold]
    #[inline(never)]
    fn make_room_for_step(&mut self) -> bool {
        let transitions = if self.clean_chain_steps() { 2 } else { 1 };
        if self.cache.has_room_for(transitions) {
            return false;
        }
        self.evict_cache();
        true
    }

    /// Before computing a missing clean-chain transition: make room for
    /// it.  This only finds none when the main transition of this byte
    /// was cached (a miss there makes room for both).  Emptying the cache
    /// drops that transition, so it is computed again here.  Returns
    /// whether the cache was emptied.
    #[cold]
    #[inline(never)]
    fn make_room_for_clean_chain(&mut self, byte: u8, class: usize) -> bool {
        if self.cache.has_room_for(1) {
            return false;
        }
        self.evict_cache();
        let slot = self.current.idx() * self.cache.stride + class;
        let trans = self
            .cache
            .populate(self.memory, self.current, byte, self.regex, self.analysis);
        self.cache.transitions[slot] = trans;
        true
    }

    /// Whether this step advances the clean no-break chain with a
    /// transition of its own, next to the main one.
    #[inline]
    fn clean_chain_steps(&self) -> bool {
        self.current_has_break_extras
            && self.regex.num_counters > 1
            && self.clean_nb != DfaStateId::DEAD
    }

    fn step_from_dead(&mut self, byte: u8) {
        if self.has_live_instances {
            if self.use_ranges {
                self.ranged_counters.clear();
            } else {
                self.inst_counters.clear();
            }
            self.has_live_instances = false;
        }
        self.match_at_end = false;
        self.post_break_tails.clear();
        self.pending_effects_current.clear();
        self.pending_effects_next.clear();

        if !self.cache.has_room_for(1) {
            // The matcher stands on no state, and the other ids it holds
            // are overwritten below: nothing to keep.
            self.cache.evict(&mut []);
            self.no_break_current = DfaStateId::DEAD;
            self.clean_nb = DfaStateId::DEAD;
            self.clean_nb_trans_slot = None;
        }
        let trans = self.cache.populate(
            self.memory,
            DfaStateId::DEAD,
            byte,
            self.regex,
            self.analysis,
        );

        // From DEAD, no instances exist, so use no_break successor.
        self.current = trans.no_break;
        self.no_break_current = trans.no_break;
        self.last_nb_counter_free_mae = trans.nb_counter_free_mae;
        self.current_has_break_extras = false;
        self.clean_nb = trans.no_break;
        self.clean_nb_cf_mae = trans.nb_counter_free_mae;
        self.clean_nb_is_match = trans.no_break_is_match;

        if !trans.is_counting {
            if trans.no_break_is_match {
                self.ever_matched = true;
            }
            // Use counter-free mae, not raw no_break_is_match_at_end,
            // for the same reason as in step_slow (Bug 23).
            // From DEAD, no contamination possible.
            if trans.nb_counter_free_mae {
                self.match_at_end = true;
            }
            // Bug 23 guard: match_at_end was reset to false above, so
            // it can only be true here if nb_counter_free_mae is true.
            debug_assert!(
                !self.match_at_end || trans.nb_counter_free_mae,
                "Bug 23 guard: step_from_dead set match_at_end without \
                 nb_counter_free_mae (state={:?})",
                self.current,
            );
        }

        // Seed instances.
        if self.use_ranges {
            for &(counter, origin, value) in &trans.seeds {
                self.ranged_counters
                    .insert(counter.idx(), origin, value, value);
            }
        } else {
            for &(counter, origin, value) in &trans.seeds {
                self.inst_counters
                    .push(counter.idx(), Instance { value, origin });
            }
        }
        if !trans.seeds.is_empty() {
            self.has_live_instances = true;
        }
    }

    /// True when no thread is tracked outside the DFA state and the
    /// counter instances: no post-break tail, no pending effect.
    #[inline(always)]
    fn side_channels_empty(&self) -> bool {
        self.post_break_tails.is_empty()
            && self.pending_effects_current.is_empty()
            && self.pending_effects_next.is_empty()
    }

    /// After a slow step that left no live instance: mark the matcher
    /// [`fresh`](Self::fresh) if no other thread is live either.
    ///
    /// Every thread that entered a counter is tracked by an instance, a
    /// post-break tail, or a pending effect; with all three empty, only
    /// counter-free threads remain, and those are exactly the non-body NFA
    /// states of the current DFA state.  When that set equals the start
    /// state's (the state is the start state or is
    /// [idle](super::IdleStates::is_idle)), the matcher is equivalent to a
    /// new one.
    ///
    /// Kept out of line: the caller is the per-byte loop, and this only
    /// runs on slow steps with no live instance.
    #[inline(never)]
    fn settle(&mut self) {
        debug_assert!(self.restartable && !self.has_live_instances);
        if self.side_channels_empty()
            && self.current != DfaStateId::DEAD
            && (self.current == self.cache.inner.start_id
                || self
                    .cache
                    .idle
                    .is_idle(&self.cache.inner, self.current, self.regex))
        {
            self.fresh = true;
        }
    }

    #[inline(always)]
    pub fn chunk(&mut self, input: &[u8]) {
        if self.ever_matched {
            return;
        }

        // Dispatch once per chunk so the prefilter is monomorphized into
        // the scanning loop: a byte-set test for the byte at hand and a
        // SIMD finder for skipping.  Without a prefilter every byte is a
        // candidate and the finder is never called.
        match self.prefilter {
            Prefilter::None => self.chunk_impl::<false>(input, |_| true, |_| Some(0)),
            Prefilter::Memchr1(b) => {
                self.chunk_impl::<true>(input, |x| x == b, |hay| memchr::memchr(b, hay));
            }
            Prefilter::Memchr2(b1, b2) => {
                self.chunk_impl::<true>(
                    input,
                    |x| x == b1 || x == b2,
                    |hay| memchr::memchr2(b1, b2, hay),
                );
            }
            Prefilter::Memchr3(b1, b2, b3) => {
                self.chunk_impl::<true>(
                    input,
                    |x| x == b1 || x == b2 || x == b3,
                    |hay| memchr::memchr3(b1, b2, b3, hay),
                );
            }
            Prefilter::Memclass(t) => {
                self.chunk_impl::<true>(
                    input,
                    |x| t.contains(x),
                    |hay| crate::memclass::memclass(&t, hay),
                );
            }
            Prefilter::Range(lo, hi) => {
                self.chunk_impl::<true>(
                    input,
                    |x| lo <= x && x <= hi,
                    |hay| crate::memrange::memrange(lo, hi, hay),
                );
            }
        }
    }

    /// Prefilter-integrated scanning loop.
    ///
    /// Whenever the matcher is [`fresh`](Self::fresh) — at the start of
    /// the stream, and again each time it becomes quiescent — the next
    /// byte is handled from the start state.  If it cannot begin a match
    /// (`is_candidate`), `finder` skips ahead to the next byte that can;
    /// testing the byte first keeps input with many failed candidates from
    /// paying for a finder call that skips nothing.  All other bytes are
    /// stepped through the DFA normally.
    ///
    /// `PREFILTER` is `false` for the instantiation without a prefilter,
    /// which lets the compiler drop the start-state test from its fast
    /// path altogether.
    ///
    /// Deliberately not `#[inline(always)]`: there is one instantiation
    /// per prefilter kind, and forcing all of them into the caller (the
    /// tier dispatcher) measurably slows this loop and the other tiers'.
    fn chunk_impl<const PREFILTER: bool>(
        &mut self,
        input: &[u8],
        is_candidate: impl Fn(u8) -> bool,
        finder: impl Fn(&[u8]) -> Option<usize>,
    ) {
        let stride = self.cache.stride;
        let start_id = self.cache.inner.start_id;
        debug_assert!(PREFILTER || !self.rearm);
        // Index in `input` from which landing on the start state is
        // noticed again (see `rearm_cooldown`); never, if the matcher
        // cannot re-arm.  Folding both into one local keeps the per-byte
        // test in the fast path down to register compares.
        let mut rearm_from = if self.rearm {
            self.rearm_cooldown
        } else {
            usize::MAX
        };

        let mut i = 0;
        while i < input.len() {
            if self.ever_matched {
                break;
            }
            if self.fresh {
                // Already in an uncontaminated start state (the usual
                // case after a failed candidate): nothing to rewrite.
                if self.current != start_id || self.current_has_break_extras {
                    self.enter_start_state();
                }
                let mut skipped = 0;
                if !is_candidate(input[i]) {
                    // At least one byte is skipped, so the end-of-input
                    // answers of the last stepped byte no longer apply.
                    self.reset_position();
                    match finder(&input[i..]) {
                        Some(offset) => skipped = offset,
                        // No candidate in the rest of this chunk: the
                        // skipped bytes leave a fresh matcher fresh.
                        None => break,
                    }
                    i += skipped;
                }
                self.fresh = false;

                // Judge the prefilter over a window of probes and disarm
                // it for a while when it is not skipping enough.
                if self.rearm {
                    self.prefilter_probes += 1;
                    self.prefilter_skipped = self
                        .prefilter_skipped
                        .saturating_add(skipped.min(u32::MAX as usize) as u32);
                    if self.prefilter_probes == PREFILTER_PROBE_WINDOW {
                        if self.prefilter_skipped < PREFILTER_PROBE_WINDOW * PREFILTER_MIN_AVG_SKIP
                        {
                            rearm_from = i + PREFILTER_COOLDOWN;
                        }
                        self.prefilter_probes = 0;
                        self.prefilter_skipped = 0;
                    }
                }
            }
            let b = input[i];
            i += 1;

            // Resolve pending effects from the PREVIOUS step (Bug 26,
            // Bug 28, Bug 42, Bug 44, Bug 46).
            //
            // This unified block resolves ALL pending NextByte effects:
            // - AddSeed: apply seeds to counter storage
            // - AddTail: consume byte `b` through the tail using static
            //   analysis targets (Advance/Increment/None).  Results
            //   are stored in local
            //   variables and injected after step_slow.
            // - Match: counter-break deferred asserts (e.g. `\B → Match`)
            //   and target deferred asserts — set `ever_matched` when the
            //   assertion chain passes and downstream Match is reachable.
            // - MatchAtEnd: set `resolved_mae` for end-of-input match.
            //
            // Tails that can't consume `b` are dropped: the thread was
            // waiting for exactly this byte (Bug 58).
            self.resolved_tails.clear();
            self.tail_epoch = self.tail_epoch.wrapping_add(1);
            let resolved_mae = if !self.pending_effects_current.is_empty() {
                self.resolve_pending_effects(b)
            } else {
                false
            };

            // --- Inline fast path ---
            if self.current == DfaStateId::DEAD {
                self.step_from_dead(b);
                continue;
            }

            let class = if stride == 256 {
                b as usize
            } else {
                self.regex.byte_classes[b as usize] as usize
            };
            let mut slot = self.current.idx() * stride + class;
            if self.cache.transitions[slot].no_break == DfaStateId::UNPOPULATED {
                // Emptying the cache changes the ids of the states the
                // matcher holds, so the slot is looked up again.
                if self.make_room_for_step() {
                    slot = self.current.idx() * stride + class;
                }
                let trans =
                    self.cache
                        .populate(self.memory, self.current, b, self.regex, self.analysis);
                self.cache.transitions[slot] = trans;
            }

            // Advance the clean no-break chain (Bug 22) BEFORE borrowing
            // the main transition `t`, since populate() mutates the cache.
            // When contaminated, advance `clean_nb` independently from the
            // previous clean state.  When not contaminated, defer update
            // to after reading `t.no_break` (see below).
            //
            // Bug 32: save the clean_nb transition slot so step_slow can
            // filter unconditional seeds on counting transitions against
            // the clean chain's seeds.
            if self.current_has_break_extras && self.regex.num_counters > 1 {
                if self.clean_nb != DfaStateId::DEAD {
                    let mut cn_slot = self.clean_nb.idx() * stride + class;
                    if self.cache.transitions[cn_slot].no_break == DfaStateId::UNPOPULATED {
                        // Emptying the cache changes the ids of the
                        // states the matcher holds: look both slots up
                        // again.
                        if self.make_room_for_clean_chain(b, class) {
                            slot = self.current.idx() * stride + class;
                            cn_slot = self.clean_nb.idx() * stride + class;
                        }
                        let cn_trans = self.cache.populate(
                            self.memory,
                            self.clean_nb,
                            b,
                            self.regex,
                            self.analysis,
                        );
                        self.cache.transitions[cn_slot] = cn_trans;
                    }
                    self.clean_nb_trans_slot = Some(cn_slot);
                    let cn_trans = &self.cache.transitions[cn_slot];
                    self.clean_nb_cf_mae = cn_trans.nb_counter_free_mae;
                    self.clean_nb_is_match = cn_trans.no_break_is_match;
                    self.clean_nb = cn_trans.no_break;
                } else {
                    self.clean_nb_cf_mae = false;
                    self.clean_nb_is_match = false;
                    self.clean_nb_trans_slot = None;
                }
            } else {
                self.clean_nb_trans_slot = None;
            }

            let t = &self.cache.transitions[slot];

            // When not contaminated, clean_nb simply tracks no_break.
            if !(self.current_has_break_extras && self.regex.num_counters > 1) {
                self.clean_nb = t.no_break;
                self.clean_nb_cf_mae = t.nb_counter_free_mae;
                self.clean_nb_is_match = t.no_break_is_match;
            }

            if !t.is_counting
                && t.pre_seeds.is_empty()
                && t.seeds.is_empty()
                && !self.has_live_instances
                && self.post_break_tails.is_empty()
            {
                let from_contaminated =
                    self.current_has_break_extras && self.regex.num_counters > 1;
                let next = t.no_break;
                self.current = next;
                self.no_break_current = next;
                let cf_mae = if from_contaminated {
                    self.clean_nb_cf_mae
                } else {
                    t.nb_counter_free_mae
                };
                self.last_nb_counter_free_mae = cf_mae;
                self.match_at_end = cf_mae;
                // Propagate contamination through no-break successors
                // (Bug 22): the no-break closure of a contaminated state
                // inherits NFA states from counter breaks.
                if !from_contaminated {
                    self.current_has_break_extras = false;
                }
                // When contaminated, `t.no_break_is_match` may include
                // Match from break-gated origins (Bug 25).  Use the
                // clean chain's transition flag instead.
                let m = if from_contaminated {
                    self.clean_nb_is_match
                } else {
                    t.no_break_is_match
                };
                if m {
                    self.ever_matched = true;
                }
                // Bug 45: inject pre-step resolved results.  The fast
                // path overwrites match_at_end, so re-apply.
                if resolved_mae {
                    self.match_at_end = true;
                }
                if !self.resolved_tails.is_empty() {
                    self.tail_epoch = self.tail_epoch.wrapping_add(1);
                    for &existing in &self.post_break_tails {
                        self.tail_seen[existing.idx()] = self.tail_epoch;
                    }
                    for &tail in &self.resolved_tails {
                        if self.tail_seen[tail.idx()] != self.tail_epoch {
                            self.tail_seen[tail.idx()] = self.tail_epoch;
                            self.post_break_tails.push(tail);
                        }
                    }
                }
                // Back in the start state with no live instance (a
                // fast-path precondition) and nothing else pending: the
                // prefilter can take over from the next byte.  The
                // cooldown test comes first on purpose: it is predictable,
                // whereas "did this byte land on the start state?" depends
                // on the input.
                if PREFILTER && i >= rearm_from && next == start_id && self.side_channels_empty() {
                    self.fresh = true;
                }
                continue;
            }

            if self.use_ranges {
                self.step_slow_ranged(slot, b);
            } else {
                self.step_slow_instances(slot, b);
            }
            // Bug 44: inject tails and flags that were computed during
            // the pre-step effect resolution.  These results couldn't
            // be injected before step_slow because step_slow clears
            // match_at_end and post_break_tails at its start.
            if resolved_mae {
                self.match_at_end = true;
            }
            if !self.resolved_tails.is_empty() {
                self.tail_epoch = self.tail_epoch.wrapping_add(1);
                for &existing in &self.post_break_tails {
                    self.tail_seen[existing.idx()] = self.tail_epoch;
                }
                for &t in &self.resolved_tails {
                    if self.tail_seen[t.idx()] != self.tail_epoch {
                        self.tail_seen[t.idx()] = self.tail_epoch;
                        self.post_break_tails.push(t);
                    }
                }
            }
            if self.restartable && !self.has_live_instances {
                self.settle();
            }
        }
        if self.rearm {
            self.rearm_cooldown = rearm_from.saturating_sub(i);
        }
    }

    /// true if `Match` is reachable with all assertions passing.
    ///
    /// Similar to [`DfaState::can_reach_match_at_end`] but uses `at_end=false`
    /// and `next=Some(byte)` for mid-input assertion evaluation.  This correctly
    /// rejects paths through `$` or `\B` that only pass at end-of-input.
    pub(super) fn can_reach_match_mid(
        start: StateIdx,
        prev: Option<u8>,
        next: Option<u8>,
        regex: &Regex,
        scratch: &mut super::ReachScratch,
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
                    if kind.eval(false, false, prev, next) == AssertEval::Pass {
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
                // Consuming states don't fire here — the assert resolution
                // is about reaching Match through epsilon transitions only.
                _ => {}
            }
        }
        false
    }

    /// Resolve pending effects from the previous step.
    ///
    /// This is the cold path for bytes where `pending_effects_current` is
    /// non-empty.  Extracted from the hot `chunk()` loop to keep the loop
    /// body compact and improve instruction-cache behavior on the common
    /// (no-pending-effects) path.
    ///
    /// Returns `true` if any resolved effect set `match_at_end`.
    #[cold]
    fn resolve_pending_effects(&mut self, b: u8) -> bool {
        let mut resolved_mae = false;
        tier3_effects::resolve_pending(
            &self.pending_effects_current,
            tier3_effects::EffectTiming::NextByte,
            &self.analysis.assert_chain_arena,
            false,   // at_end = false (mid-input)
            Some(b), // next byte is known
            self.regex,
            &mut self.reach_scratch,
            &mut self.resolved_actions,
        );
        let actions = &self.resolved_actions;
        // Apply match signals from deferred assertions.
        if actions.set_match {
            self.ever_matched = true;
        }
        if actions.set_match_at_end {
            resolved_mae = true;
        }
        // Apply resolved seeds.
        for &(counter, origin, value) in &actions.seeds {
            if self.use_ranges {
                self.ranged_counters
                    .insert(counter.idx(), origin, value, value);
            } else {
                self.inst_counters.seed(counter.idx(), origin, value);
            }
            self.has_live_instances = true;
        }
        // Process resolved tails: consume byte `b` through each
        // tail using static analysis targets.
        for &tail in &actions.tails {
            if let Some((target, exit_target)) = consume_byte(tail, b, self.regex) {
                // Process both the primary target and the exit target
                // (when present).
                let targets_to_process: [StateIdx; 2] = [target, exit_target];
                for &tgt in targets_to_process.iter().filter(|t| **t != StateIdx::NONE) {
                    match &self.analysis.target_effects[tgt.idx()] {
                        Some(effects) => match &effects.step {
                            tier3_effects::TargetStep::Advance { new_origins } => {
                                for &new_o in new_origins {
                                    if self.tail_seen[new_o.idx()] != self.tail_epoch {
                                        self.tail_seen[new_o.idx()] = self.tail_epoch;
                                        self.resolved_tails.push(new_o);
                                    }
                                }
                                // Apply immediate atoms from compiled Advance.
                                tier3_effects::apply_immediate_atoms(
                                    &effects.immediate,
                                    &mut self.ever_matched,
                                    &mut resolved_mae,
                                );
                                // Second-order: origin deferred asserts
                                // go to pending_effects_next.
                                tier3_effects::enqueue_guarded_effects(
                                    &mut self.pending_effects_next,
                                    &self.analysis.origin_effects[tail.idx()].guarded,
                                    b,
                                );
                                // Bug 57: so do the origins behind a
                                // deferrable assertion.
                                tier3_effects::enqueue_guarded_effects(
                                    &mut self.pending_effects_next,
                                    &effects.guarded,
                                    b,
                                );
                            }
                            tier3_effects::TargetStep::Increment {
                                counter,
                                min,
                                max,
                                continue_origins,
                                ..
                            } => {
                                // Tail hit CInc — hand off to counter.
                                // Value starts at 0, increment to 1.
                                let pbt_value: u32 = 0;
                                if pbt_value + 1 < *max {
                                    for &new_o in continue_origins {
                                        if self.use_ranges {
                                            self.ranged_counters.insert(
                                                counter.idx(),
                                                new_o,
                                                pbt_value + 1,
                                                pbt_value + 1,
                                            );
                                        } else {
                                            self.inst_counters.seed(
                                                counter.idx(),
                                                new_o,
                                                pbt_value + 1,
                                            );
                                        }
                                        self.has_live_instances = true;
                                    }
                                }
                                if pbt_value + 1 >= *min {
                                    // Apply on_break atoms from CompiledTargetEffects.
                                    for atom in &effects.on_break {
                                        match atom {
                                            tier3_effects::EffectAtom::Match => {
                                                self.ever_matched = true;
                                            }
                                            tier3_effects::EffectAtom::MatchAtEnd => {
                                                self.match_at_end = true;
                                            }
                                            tier3_effects::EffectAtom::AddTail { origin } => {
                                                if self.tail_seen[origin.idx()] != self.tail_epoch {
                                                    self.tail_seen[origin.idx()] = self.tail_epoch;
                                                    self.next_post_break_tails.push(*origin);
                                                }
                                            }
                                            tier3_effects::EffectAtom::AddSeed {
                                                counter: sc,
                                                origin: so,
                                                value: sv,
                                            } => {
                                                if self.use_ranges {
                                                    self.ranged_counters.insert(
                                                        sc.idx(),
                                                        *so,
                                                        *sv,
                                                        *sv,
                                                    );
                                                } else {
                                                    self.inst_counters.seed(sc.idx(), *so, *sv);
                                                }
                                                self.has_live_instances = true;
                                            }
                                        }
                                    }
                                    // Deposit guarded effects as PendingEffects.
                                    // Second-order: goes to pending_effects_next.
                                    tier3_effects::enqueue_guarded_effects(
                                        &mut self.pending_effects_next,
                                        &effects.guarded,
                                        b,
                                    );
                                }
                            }
                        },
                        None => {
                            let oe = &self.analysis.origin_effects[tail.idx()];
                            tier3_effects::apply_immediate_atoms(
                                &oe.immediate,
                                &mut self.ever_matched,
                                &mut resolved_mae,
                            );
                            // Second-order: origin deferred asserts
                            // go to pending_effects_next.
                            tier3_effects::enqueue_guarded_effects(
                                &mut self.pending_effects_next,
                                &oe.guarded,
                                b,
                            );
                        }
                    }
                } // end for tgt in targets_to_process
            }
            // Bug 58: a tail that cannot consume this byte is dead.  The
            // assertion in front of it was resolved against `b`, so the
            // thread stands right before `b` and has no other byte to
            // take.  Keeping it (as was done) let it consume a later
            // byte instead, starting a second thread one position late.
        }
        // Clear resolved effects, then swap next→current so
        // second-order deferred effects survive to the next
        // boundary.  step_slow will append first-order effects
        // to pending_effects_current after this.
        self.pending_effects_current.clear();
        std::mem::swap(
            &mut self.pending_effects_current,
            &mut self.pending_effects_next,
        );
        resolved_mae
    }

    pub fn finish(mut self) -> bool {
        if self.ever_matched {
            return true;
        }
        if self.match_at_end {
            return true;
        }
        // Note: we intentionally do NOT check `state.is_match_at_end` here.
        // For non-counting transitions, `self.match_at_end` already captures
        // the DFA state's match-at-end flag (set in the fast path or
        // step_slow).  For counting transitions, the DFA state's flag is
        // overly optimistic: the with_break closure follows ALL CInc break
        // paths (including downstream counters that haven't actually reached
        // their min), so `is_match_at_end` may be set even when no valid
        // break chain leads to `$ → Match`.  Only `self.match_at_end`
        // (computed per-instance in step_slow) correctly reflects whether a
        // specific counter instance actually broke with enough value.
        //
        // We use `no_break_current` (the no-break DFA state from the last
        // transition) for deferred assertion resolution.  The with-break
        // state includes deferred assertions from ALL CInc break paths
        // (e.g. `\b` after counter 1's CInc break in `.{6,39}.{7,31}\b$`).
        // Those assertions fire here even when the counter never reached
        // its minimum, causing false positives.  The no-break state only
        // includes deferred assertions reachable without this transition's
        // counter breaks — the counter-free subset safe for EOI resolution.
        //
        // For counters that actually broke with enough value and had a
        // pure `$ → Match` break path, per-instance `break_is_match_at_end`
        // in step_slow already sets `match_at_end` (checked above).
        //
        // Counter-free match-at-end: use the transition's filtered flag
        // instead of the raw DFA state's `is_match_at_end`.  The raw flag
        // includes `$ → Match` from ALL targets in the no-break closure,
        // including targets from counter-dependent origins that entered
        // the DFA state via previous counter breaks.  The filtered flag
        // only includes `$ → Match` from targets of `reachable_without_break`
        // origins, which are genuinely counter-free.
        //
        // This catches cases like `(0{2,2}|1*)$` on "0" where the start
        // closure's `1* → $ → Match` path is counter-free, but avoids
        // false positives from patterns like `^c{2,12}.{6,6}(a?)?$` on
        // "ccca" where the `(a?)?$` suffix is counter-dependent (Bug 13).
        if self.last_nb_counter_free_mae {
            return true;
        }
        // Deferred assertions from the no-break state (legacy DFA-state path).
        //
        // These are DFA-state deferred assertions from epsilon_closure(),
        // NOT typed pending effects.  They coexist with the effect path
        // below because they come from different sources:
        //   - DFA-state: counter-free assertions from the no-break closure
        //   - Pending effects: counter-dependent assertions from break paths
        // See docs/tier3-eoi-unification-analysis.md for a detailed analysis
        // of why these two paths coexist and how to unify them.
        //
        // Bug 30: when contaminated (break_extras && num_counters > 1),
        // `no_break_current` may include deferred assertions from
        // break-gated origins (e.g. `\b` after `Byte('f')` which is only
        // reachable via c1's break path).  These spurious deferred asserts
        // fire at EOI even when the gating counter never reached its min.
        //
        // Fix: when contaminated, skip `no_break_current`'s deferred asserts
        // entirely.  Legitimate counter-break deferred asserts are deposited
        // as PendingEffect entries with Match atoms and resolved
        // via the pending-effect path below.
        // Non-contaminated deferred asserts from `clean_nb` are handled below.
        let from_contaminated = self.current_has_break_extras && self.regex.num_counters > 1;
        if self.no_break_current != DfaStateId::DEAD && !from_contaminated {
            let nb_state = &self.cache.inner.states[self.no_break_current.idx()];
            if nb_state.resolve_deferred_at_end(self.regex, &mut self.reach_scratch) {
                return true;
            }
        }
        // When contaminated, fall back to clean_nb for counter-free deferred
        // asserts (clean_nb excludes break-gated origins).
        if from_contaminated && self.clean_nb != DfaStateId::DEAD {
            let clean_state = &self.cache.inner.states[self.clean_nb.idx()];
            if clean_state.resolve_deferred_at_end(self.regex, &mut self.reach_scratch) {
                return true;
            }
        }
        // Pending effects at end-of-input (Bug 28 + Bug 42 + Bug 46):
        // Resolve all NextByte effects with EOI context.  This handles:
        // - Break seeds whose deferred assertions couldn't be evaluated
        //   mid-input (if the assertion passes AND the seeded counter can
        //   immediately break with value >= min, this is a match).
        // - Break tails whose deferred assertions need EOI context.
        // - Counter-break deferred assertions: Match atoms
        //   whose assertion chains are evaluated with at_end=true and
        //   downstream reachability checked via can_reach_match_at_end.
        if !self.pending_effects_current.is_empty() {
            tier3_effects::resolve_pending(
                &self.pending_effects_current,
                tier3_effects::EffectTiming::NextByte,
                &self.analysis.assert_chain_arena,
                true, // at_end = true
                None, // no next byte
                self.regex,
                &mut self.reach_scratch,
                &mut self.resolved_actions,
            );
            let actions = &self.resolved_actions;
            // Seeds at EOI: check immediate-break match.
            //
            // Over-approximate scan: we iterate ALL target_effects looking for
            // any Increment target with the same counter whose min is met and
            // whose on_break has Match/MatchAtEnd.  The seed's `_origin` is
            // ignored — we can't look up by origin because at EOI there's no
            // byte to consume (can't call consume_byte to find the target).
            //
            // This is safe because Tier 3 has exactly one CInc per counter,
            // so all Increment targets for the same counter share the same
            // BreakEffects and thus the same on_break atoms.  The invariant
            // is validated by a debug_assert in compute_tier3_analysis().
            for &(counter, _origin, value) in &actions.seeds {
                for effects in self.analysis.target_effects.iter().flatten() {
                    if let tier3_effects::TargetStep::Increment {
                        counter: tc, min, ..
                    } = &effects.step
                        && *tc == counter
                        && value >= *min
                        && effects.on_break.iter().any(|a| {
                            matches!(
                                a,
                                tier3_effects::EffectAtom::Match
                                    | tier3_effects::EffectAtom::MatchAtEnd
                            )
                        })
                    {
                        return true;
                    }
                }
            }
            // Tails at EOI: tails are unconsumed consuming NFA states.
            // At EOI, no byte can be consumed, so tails cannot advance
            // to their post-consumption target.  Do NOT check
            // target_is_match_at_end here — that would claim a match
            // based on a byte consumption that never happened.
            //
            // Previously this block checked target_is_match_at_end for
            // each resolved tail, which was safe only because deferred
            // tails rarely appeared in pending effects at EOI.  After 8E
            // (authority flip), guarded AddTail atoms resolve at EOI more
            // often, exposing this false positive.
            // Counter-dependent deferred assertions are resolved as
            // Match effects within the same resolve_pending
            // call.  If any assertion chain passed with downstream
            // reachability, set_match or set_match_at_end is set.
            if actions.set_match || actions.set_match_at_end {
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

impl fmt::Debug for Tier3DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Tier3DfaMatcher");
        s.field("current", &self.current);
        if self.current != DfaStateId::DEAD {
            let state = &self.cache.inner.states[self.current.idx()];
            s.field("nfa_states", &state.nfa_states);
            s.field("is_match", &state.is_match);
            s.field("is_match_at_end", &state.is_match_at_end);
        }
        s.field("no_break_current", &self.no_break_current)
            .field("last_nb_counter_free_mae", &self.last_nb_counter_free_mae)
            .field("ever_matched", &self.ever_matched)
            .field("match_at_end", &self.match_at_end)
            .field("has_live_instances", &self.has_live_instances)
            .field("fresh", &self.fresh)
            .field("restartable", &self.restartable)
            .field("rearm", &self.rearm)
            .field("prefilter_probes", &self.prefilter_probes)
            .field("prefilter_skipped", &self.prefilter_skipped)
            .field("rearm_cooldown", &self.rearm_cooldown)
            .field("current_has_break_extras", &self.current_has_break_extras)
            .field("clean_nb", &self.clean_nb)
            .field("clean_nb_cf_mae", &self.clean_nb_cf_mae)
            .field("clean_nb_is_match", &self.clean_nb_is_match)
            .field("clean_nb_trans_slot", &self.clean_nb_trans_slot)
            .field("post_break_tails_len", &self.post_break_tails.len())
            .field("use_ranges", &self.use_ranges)
            .field(
                "pending_effects_current_len",
                &self.pending_effects_current.len(),
            )
            .field("pending_effects_next_len", &self.pending_effects_next.len());
        s.finish()
    }
}

impl fmt::Display for Tier3DfaMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // --- Line 1: core DFA state ---
        if self.current == DfaStateId::DEAD {
            write!(f, "DFA[T3] state=DEAD")?;
        } else {
            let state = &self.cache.inner.states[self.current.idx()];
            write!(
                f,
                "DFA[T3] state={} nfa={{{}}} is_match={}",
                self.current.0,
                state
                    .nfa_states
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                state.is_match,
            )?;
        }
        // Always-show flags (never hidden when false/empty).
        write!(
            f,
            " matched={} mae={} cf_mae={} break_extras={} live={} fresh={}",
            self.ever_matched,
            self.match_at_end,
            self.last_nb_counter_free_mae,
            self.current_has_break_extras,
            self.has_live_instances,
            self.fresh,
        )?;
        // No-break DFA state (always shown).
        if self.no_break_current == DfaStateId::DEAD {
            write!(f, " nb=DEAD")?;
        } else {
            write!(f, " nb={}", self.no_break_current.0)?;
        }
        // Clean chain state (always shown when multi-counter pattern has
        // break extras; shown as "n/a" otherwise so absence is explicit).
        if self.current_has_break_extras && self.regex.num_counters > 1 {
            if self.clean_nb == DfaStateId::DEAD {
                write!(f, " clean_nb=DEAD")?;
            } else {
                let clean_state = &self.cache.inner.states[self.clean_nb.idx()];
                write!(
                    f,
                    " clean_nb={{{}}}",
                    clean_state
                        .nfa_states
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                )?;
            }
            write!(
                f,
                " clean_is_match={} clean_cf_mae={}",
                self.clean_nb_is_match, self.clean_nb_cf_mae,
            )?;
        }
        // --- Counter summary (always shown, even when empty) ---
        if self.use_ranges {
            let rc = &self.ranged_counters;
            for ci in 0..rc.num_counters() {
                let entries = rc.entries(ci);
                let count = entries.len();
                write!(f, "\n  c{ci}: {count} range(s) [")?;
                for (j, entry) in entries.iter().enumerate() {
                    if j > 0 {
                        write!(f, ", ")?;
                    }
                    write!(
                        f,
                        "origin:{}={}-{}",
                        entry.origin, entry.min_val, entry.max_val
                    )?;
                }
                write!(f, "]")?;
            }
        } else {
            let ic = &self.inst_counters;
            for ci in 0..ic.num_counters() {
                let entries = ic.entries(ci);
                let count = entries.len();
                write!(f, "\n  c{ci}: {count} inst [")?;
                for (j, entry) in entries.iter().enumerate() {
                    if j > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}@{}", entry.origin, entry.value)?;
                }
                write!(f, "]")?;
            }
        }
        // --- Tails (always shown) ---
        write!(
            f,
            "\n  tails: [{}]",
            self.post_break_tails
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )?;
        // --- Effect queues (always shown) ---
        // Break seeds, break tails, and deferred assertion match
        // signals are all carried as pending effects.  Two-buffer
        // swap pattern: current is resolved, next receives second-order.
        {
            write!(f, "\n  eff_current: [")?;
            for (i, pe) in self.pending_effects_current.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{pe}")?;
            }
            write!(f, "]")?;
        }
        if !self.pending_effects_next.is_empty() {
            write!(f, "\n  eff_next: [")?;
            for (i, pe) in self.pending_effects_next.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{pe}")?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RegexConfig;

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

    /// Run `f` on a tier-3 matcher for `pattern` that has consumed each
    /// of `chunks` in turn.
    fn with_matcher<R>(
        pattern: &str,
        chunks: &[&[u8]],
        f: impl FnOnce(&mut Tier3DfaMatcher) -> R,
    ) -> R {
        let re = build(pattern);
        assert!(re.tier3_eligible, "`{pattern}` must be tier 3 eligible");
        let analysis = re.tier3_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier3DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        for chunk in chunks {
            m.chunk(chunk);
        }
        f(&mut m)
    }

    /// Bug 64: emptying a full cache refreshes every state id the matcher
    /// holds and drops every table and slot indexed by one.
    #[test]
    fn test_evict_cache_refreshes_every_state_id_the_matcher_holds() {
        with_matcher(
            "a{2,3}b(?:x|yy){2,3}c",
            &[b"z", b"a", b"a", b"b", b"x"],
            |m| {
                let n = m.cache.inner.states.len();
                assert!(n >= 4, "need four distinct states, have {n}");
                // Three different states, none of them the start state.
                m.current = DfaStateId(3);
                m.no_break_current = DfaStateId(1);
                m.clean_nb = DfaStateId(2);
                let want = [3, 1, 2].map(|i| m.cache.inner.states[i].clone());
                m.clean_nb_trans_slot = Some(0);
                // Classify a state so the idle table is not empty.
                m.cache.idle.is_idle(&m.cache.inner, DfaStateId(1), m.regex);
                assert!(!m.cache.idle.class.is_empty());

                m.evict_cache();

                let ids = [m.current, m.no_break_current, m.clean_nb];
                for (id, want) in ids.iter().zip(&want) {
                    assert_eq!(&m.cache.inner.states[id.idx()], want);
                }
                assert_eq!(m.cache.inner.states.len(), 4);
                assert_eq!(m.clean_nb_trans_slot, None);
                assert!(m.cache.idle.class.is_empty());
                assert_eq!(m.cache.transitions.len(), 4 * m.cache.stride);
                assert!(
                    m.cache
                        .transitions
                        .iter()
                        .all(|t| t.no_break == DfaStateId::UNPOPULATED)
                );
                assert_eq!(m.cache.evictions(), 1);
            },
        );
    }

    /// Bug 64: one input byte empties the cache at most once, even when
    /// both the main transition and the clean chain's are missing.  The
    /// main lookup makes room for both.
    #[test]
    fn test_one_byte_evicts_at_most_once() {
        let mut steps_with_clean_chain = 0;
        for pattern in [
            "a{2,3}b(?:x|yy){2,3}c",
            "(?:a|bb){2,3}c(?:x|yy){2,3}d",
            ".{2,4}1\\b.{2,3}z",
        ] {
            let re = build(pattern);
            assert!(re.tier3_eligible, "`{pattern}` must be tier 3 eligible");
            let analysis = re.tier3_analysis.as_ref().unwrap();
            let mut memory = DfaMemory::default();
            let mut cache = Tier3DfaCache::new();
            cache.set_max_states(crate::dfa::DFA_MIN_STATES);
            cache.prepare(&mut memory, &re, analysis);
            let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
            let input = b"aabxyyc aabbbcxyyd abbaaabxxyyx 111 1 11 aaabyyxxc bbabbcyyxd";
            for _ in 0..4 {
                for &b in input.iter() {
                    let before = m.cache.evictions();
                    if m.clean_chain_steps() {
                        steps_with_clean_chain += 1;
                    }
                    m.chunk(&[b]);
                    let evicted = m.cache.evictions() - before;
                    assert!(evicted <= 1, "`{pattern}`: {evicted} evictions on one byte");
                }
            }
            assert!(m.cache.evictions() > 0, "`{pattern}`: no eviction at all");
        }
        assert!(steps_with_clean_chain > 0, "no step used the clean chain");
    }

    /// Bug 64: the rarest eviction.  The main transition is cached, the
    /// clean chain's is not, and there is no room for it: the cache is
    /// emptied between the two lookups of one byte, and the main
    /// transition has to be computed again.
    ///
    /// Ordinary inputs almost never get there, so the situation is set
    /// up: whenever the clean chain is about to take a transition that is
    /// not cached, the main one is cached by hand and the cap is lowered
    /// to the current size.  After every byte the matcher must stand on
    /// the same states as one that never evicts.
    #[test]
    fn test_eviction_between_main_and_clean_lookup() {
        // What a matcher stands on, by content: ids differ between caches.
        fn standing(m: &Tier3DfaMatcher) -> [Option<crate::dfa::DfaState>; 3] {
            [m.current, m.no_break_current, m.clean_nb]
                .map(|id| (id != DfaStateId::DEAD).then(|| m.cache.inner.states[id.idx()].clone()))
        }

        let mut forced = 0;
        // Each alphabet leaves out the pattern's last byte: no match, so
        // the matchers keep running.
        for (pattern, alphabet) in [
            ("a{2,3}b(?:x|yy){2,3}c", &b"aabbxy1 dz"[..]),
            ("(?:a|bb){2,3}c(?:x|yy){2,3}d", &b"aabbxyc1 z"[..]),
            (".{2,4}1\\b.{2,3}z", &b"aabbxyc1 d"[..]),
        ] {
            let re = build(pattern);
            assert!(re.tier3_eligible, "`{pattern}` must be tier 3 eligible");
            let analysis = re.tier3_analysis.as_ref().unwrap();
            let (mut memory, mut ref_memory) = (DfaMemory::default(), DfaMemory::default());
            let (mut cache, mut ref_cache) = (Tier3DfaCache::new(), Tier3DfaCache::new());
            cache.prepare(&mut memory, &re, analysis);
            ref_cache.prepare(&mut ref_memory, &re, analysis);
            let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
            let mut reference =
                Tier3DfaMatcher::new(&mut ref_cache, &mut ref_memory, &re, analysis);

            let mut rng: u64 = 0x9E37_79B9_7F4A_7C15;
            for i in 0..5_000 {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                let b = alphabet[(rng >> 33) as usize % alphabet.len()];

                // Back to the default cap, and forget the cached
                // transitions now and then so that lookups keep missing.
                m.cache.set_max_states(crate::dfa::DFA_MAX_STATES);
                if i % 5 == 0 {
                    m.evict_cache();
                }
                // Then force the situation if this byte allows it.
                if m.current != DfaStateId::DEAD && m.clean_chain_steps() {
                    let stride = m.cache.stride;
                    let class = if stride == 256 {
                        b as usize
                    } else {
                        m.regex.byte_classes[b as usize] as usize
                    };
                    let clean_slot = m.clean_nb.idx() * stride + class;
                    let clean_cached =
                        m.cache.transitions[clean_slot].no_break != DfaStateId::UNPOPULATED;
                    if !clean_cached && m.clean_nb != m.current {
                        let slot = m.current.idx() * stride + class;
                        if m.cache.transitions[slot].no_break == DfaStateId::UNPOPULATED {
                            let trans = m
                                .cache
                                .populate(m.memory, m.current, b, m.regex, m.analysis);
                            m.cache.transitions[slot] = trans;
                        }
                        // Pad the table to the smallest cap with states
                        // nothing refers to, then leave no room.
                        let mut filler = 0;
                        while m.cache.inner.states.len() < crate::dfa::DFA_MIN_STATES {
                            m.cache.intern_state(
                                &[StateIdx(0), StateIdx(0), StateIdx(filler)],
                                &[],
                                false,
                                false,
                                false,
                            );
                            filler += 1;
                        }
                        m.cache.inner.max_states = m.cache.inner.states.len();
                        forced += 1;
                        let before = m.cache.evictions();
                        m.chunk(&[b]);
                        assert_eq!(m.cache.evictions(), before + 1, "`{pattern}` byte {i}");
                    } else {
                        m.chunk(&[b]);
                    }
                } else {
                    m.chunk(&[b]);
                }
                reference.chunk(&[b]);

                assert!(!reference.ismatch(), "`{pattern}` must not match");
                assert_eq!(
                    (
                        standing(&m),
                        m.ismatch(),
                        m.match_at_end,
                        m.has_live_instances
                    ),
                    (
                        standing(&reference),
                        false,
                        reference.match_at_end,
                        reference.has_live_instances
                    ),
                    "`{pattern}` diverges at byte {i}"
                );
            }
            assert_eq!(reference.cache.evictions(), 0);
        }
        assert!(forced > 20, "the situation came up only {forced} times");
    }

    /// Bug 64: stepping out of the dead state with a full cache empties
    /// it and leaves no stale id or slot behind.
    #[test]
    fn test_eviction_from_the_dead_state() {
        let re = build("^a{2,3}b(?:x|yy){2,3}c|q{2,3}r");
        assert!(re.tier3_eligible);
        let analysis = re.tier3_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier3DfaCache::new();
        cache.set_max_states(crate::dfa::DFA_MIN_STATES);
        cache.prepare(&mut memory, &re, analysis);
        let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        // Fill the table with states nothing refers to.
        let mut filler = 0;
        while m.cache.has_room_for(1) {
            m.cache
                .intern_state(&[StateIdx(0), StateIdx(filler)], &[], false, false, false);
            filler += 1;
        }
        m.current = DfaStateId::DEAD;
        m.no_break_current = DfaStateId(2);
        m.clean_nb = DfaStateId(3);
        m.clean_nb_trans_slot = Some(7);

        m.step_from_dead(b'q');

        assert_eq!(m.cache.evictions(), 1);
        assert_eq!(m.clean_nb_trans_slot, None);
        let n = m.cache.inner.states.len();
        assert!(
            n <= 3,
            "start state plus one transition's successors, have {n}"
        );
        for id in [m.current, m.no_break_current, m.clean_nb] {
            assert!(id == DfaStateId::DEAD || id.idx() < n, "stale id {id:?}");
        }
        assert_eq!(m.cache.transitions.len(), n * m.cache.stride);
    }

    /// Debug rendering of the matcher's state, with the prefilter
    /// effectiveness counters zeroed: they are a heuristic about past
    /// input, not part of what the matcher will answer.
    fn state_of(m: &mut Tier3DfaMatcher) -> String {
        m.prefilter_probes = 0;
        m.prefilter_skipped = 0;
        m.rearm_cooldown = 0;
        format!("{m:?}")
    }

    /// State of a matcher that has consumed nothing.
    fn initial_state(pattern: &str) -> String {
        with_matcher(pattern, &[], state_of)
    }

    /// Match result for `chunks` on a new tier-3 matcher.
    fn run(pattern: &str, chunks: &[&[u8]]) -> bool {
        let re = build(pattern);
        let analysis = re.tier3_analysis.as_ref().unwrap();
        let mut memory = DfaMemory::default();
        let mut cache = Tier3DfaCache::new();
        cache.prepare(&mut memory, &re, analysis);
        let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &re, analysis);
        for chunk in chunks {
            m.chunk(chunk);
        }
        m.finish()
    }

    const TWO_GAPS: &str = "foo.{0,5}bar.{0,5}baz";

    // ── Becoming fresh again ──

    #[test]
    fn test_fresh_after_failed_candidate() {
        // `f` is a prefilter candidate that leads nowhere.  Mid-candidate
        // the matcher is busy; once the DFA is back in the start state it
        // is fresh, and the prefilter skips what follows.
        with_matcher(TWO_GAPS, &[], |m| {
            assert!(m.fresh && m.restartable && m.rearm);
        });
        with_matcher(TWO_GAPS, &[b"f"], |m| assert!(!m.fresh));
        with_matcher(TWO_GAPS, &[b"fo"], |m| assert!(!m.fresh));
        with_matcher(TWO_GAPS, &[b"fx"], |m| assert!(m.fresh));
        with_matcher(TWO_GAPS, &[b"fxyyyy"], |m| assert!(m.fresh));
    }

    #[test]
    fn test_fresh_after_gap_expires() {
        // First gap open: an instance is live.
        with_matcher(TWO_GAPS, &[b"fooxx"], |m| {
            assert!(m.has_live_instances);
            assert!(!m.fresh);
        });
        // Last iteration: the instance is gone, but its break left a
        // post-break tail waiting for `b`.
        with_matcher(TWO_GAPS, &[b"fooxxxxx"], |m| {
            assert!(!m.has_live_instances);
            assert!(!m.fresh);
        });
        // Gap expired.  The dead body state is still in the DFA state —
        // nothing is rewritten until the next byte arrives — but the
        // matcher is marked fresh.
        with_matcher(TWO_GAPS, &[b"fooxxxxxx"], |m| {
            assert!(!m.has_live_instances);
            assert_ne!(m.current, m.cache.inner.start_id);
            assert!(m.fresh);
        });
        // The next byte puts the DFA back in the start state.
        with_matcher(TWO_GAPS, &[b"fooxxxxxxy"], |m| {
            assert!(m.fresh);
            assert_eq!(m.current, m.cache.inner.start_id);
        });
        // Same after the second gap expires.
        with_matcher(TWO_GAPS, &[b"fooxbarxxxxxxxxy"], |m| assert!(m.fresh));
    }

    #[test]
    fn test_fresh_matcher_equals_initial_state() {
        // Once the prefilter has skipped input, a fresh matcher is
        // indistinguishable from a new one.
        let initial = initial_state(TWO_GAPS);
        for input in [
            &b"fxyyyy"[..],
            b"fooxxxxxxxy",
            b"fooxbarxxxxxxxxy",
            b"fofofooxbaxxxxxxxxxxxxfooxxbarxbaxxxxxxxxxxx",
        ] {
            let after = with_matcher(TWO_GAPS, &[input], state_of);
            assert_eq!(after, initial, "input {:?}", String::from_utf8_lossy(input));
        }
    }

    #[test]
    fn test_fresh_waits_for_tails_and_pending_effects() {
        // The first gap has expired, but `ba` of the second anchor was
        // consumed in time and lives on as a post-break tail.
        with_matcher(TWO_GAPS, &[b"fooxxxxba"], |m| {
            assert!(!m.has_live_instances);
            assert!(!m.post_break_tails.is_empty());
            assert!(!m.fresh);
        });
        with_matcher(TWO_GAPS, &[b"fooxxxxbay", b"y"], |m| assert!(m.fresh));
        // A deferred word-boundary assertion keeps the state busy until
        // it resolves.
        with_matcher("foo\\b.{0,5}bar", &[b"foo"], |m| assert!(!m.fresh));
        with_matcher("foo\\b.{0,5}bar", &[b"fooxxxxxxxxxxy"], |m| {
            assert!(m.fresh)
        });
    }

    #[test]
    fn test_settle_requires_empty_side_channels() {
        // A tail or a pending effect is a live thread even when the DFA
        // state looks idle (it may sit on a counter-body state).
        let pending = tier3_effects::PendingEffect {
            timing: tier3_effects::EffectTiming::NextByte,
            guard: tier3_effects::EffectGuard::ALWAYS,
            atom: tier3_effects::EffectAtom::Match,
            prev: crate::ByteProps::NONE,
        };
        with_matcher(TWO_GAPS, &[b"fooxxxxxx"], |m| {
            // Idle DFA state, nothing live: `settle()` marked it fresh.
            assert!(m.fresh);
            assert_ne!(m.current, m.cache.inner.start_id);
            let idle_state = m.current;

            let body = (0..m.regex.states.len())
                .find(|&i| m.regex.counter_body_states[i])
                .expect("pattern has a counter body");

            m.fresh = false;
            m.post_break_tails.push(StateIdx(body as u32));
            m.settle();
            assert!(!m.fresh, "post-break tail");
            m.post_break_tails.clear();

            m.pending_effects_current.push(pending);
            m.settle();
            assert!(!m.fresh, "pending effect (current)");
            m.pending_effects_current.clear();

            m.pending_effects_next.push(pending);
            m.settle();
            assert!(!m.fresh, "pending effect (next)");
            m.pending_effects_next.clear();

            m.current = DfaStateId::DEAD;
            m.settle();
            assert!(!m.fresh, "dead state");

            m.current = idle_state;
            m.settle();
            assert!(m.fresh, "idle state, nothing live");

            m.fresh = false;
            m.current = m.cache.inner.start_id;
            m.settle();
            assert!(m.fresh, "start state, nothing live");
        });
    }

    #[test]
    fn test_fast_path_does_not_rearm_with_a_live_tail() {
        // The DFA fast path can land on the start state while a tail
        // resolved from a pending effect is still alive.  That tail is a
        // thread the prefilter knows nothing about, so the matcher must
        // not be marked fresh.
        with_matcher(TWO_GAPS, &[b"fx"], |m| {
            assert!(m.fresh, "plain landing on the start state re-arms");
            let start_id = m.cache.inner.start_id;
            // A consuming state outside the start closure: the `b` of the
            // second anchor.  Consuming `b` moves the tail on to the `a`
            // that follows, while the DFA goes from the start state back
            // to the start state.
            let tail = (0..m.regex.states.len() as u32)
                .map(StateIdx)
                .find(|&s| {
                    consume_byte(s, b'b', m.regex).is_some()
                        && consume_byte(s, b'x', m.regex).is_none()
                })
                .expect("pattern has a `b` state");
            let (after_b, _) = consume_byte(tail, b'b', m.regex).unwrap();

            m.fresh = false;
            m.pending_effects_current
                .push(tier3_effects::PendingEffect {
                    timing: tier3_effects::EffectTiming::NextByte,
                    guard: tier3_effects::EffectGuard::ALWAYS,
                    atom: tier3_effects::EffectAtom::AddTail { origin: tail },
                    prev: crate::ByteProps::NONE,
                });
            m.chunk(b"b");
            assert_eq!(m.current, start_id);
            assert_eq!(m.post_break_tails, vec![after_b]);
            assert!(!m.fresh);
        });
    }

    #[test]
    fn test_resolved_tail_that_cannot_consume_is_dropped() {
        // A guarded tail is resolved against the byte it has to consume.
        // If it cannot take that byte the thread is dead; it must not
        // wait for a later byte (Bug 58).
        with_matcher(TWO_GAPS, &[b"fx"], |m| {
            let tail = (0..m.regex.states.len() as u32)
                .map(StateIdx)
                .find(|&s| {
                    consume_byte(s, b'b', m.regex).is_some()
                        && consume_byte(s, b'x', m.regex).is_none()
                })
                .expect("pattern has a `b` state");
            m.fresh = false;
            m.pending_effects_current
                .push(tier3_effects::PendingEffect {
                    timing: tier3_effects::EffectTiming::NextByte,
                    guard: tier3_effects::EffectGuard::ALWAYS,
                    atom: tier3_effects::EffectAtom::AddTail { origin: tail },
                    prev: crate::ByteProps::NONE,
                });
            m.chunk(b"x");
            assert!(m.post_break_tails.is_empty());
            assert!(m.pending_effects_current.is_empty());
            assert!(m.fresh, "nothing is live any more");
        });
    }

    #[test]
    fn test_start_state_helpers_reset_every_side_channel() {
        // `enter_start_state()` and `reset_position()` initialise a new
        // matcher and put a fresh one back at the start.  Between them
        // they must restore every field that describes live threads or
        // the current position, whatever it held before.
        let initial = initial_state(TWO_GAPS);
        let after = with_matcher(TWO_GAPS, &[b"fooxxxxxx"], |m| {
            assert!(m.fresh);
            m.match_at_end = true;
            m.last_nb_counter_free_mae = true;
            m.current_has_break_extras = true;
            m.clean_nb = m.current;
            m.clean_nb_cf_mae = true;
            m.clean_nb_is_match = true;
            m.clean_nb_trans_slot = Some(0);
            m.no_break_current = m.current;
            m.enter_start_state();
            m.reset_position();
            state_of(m)
        });
        assert_eq!(after, initial);

        // The same through the public path: the fields are scribbled on a
        // fresh matcher, then one skipped byte must clean all of them.
        let after = with_matcher(TWO_GAPS, &[b"fooxxxxxx"], |m| {
            m.match_at_end = true;
            m.last_nb_counter_free_mae = true;
            m.current_has_break_extras = true;
            m.clean_nb_cf_mae = true;
            m.clean_nb_is_match = true;
            m.clean_nb_trans_slot = Some(0);
            m.chunk(b"y");
            state_of(m)
        });
        assert_eq!(after, initial);
    }

    #[test]
    fn test_fresh_across_chunk_boundaries() {
        let initial = initial_state(TWO_GAPS);
        let input = b"fooxbarxxxxxxxxfxfooxxxxxxxxxxy";
        for split in 0..=input.len() {
            let after = with_matcher(TWO_GAPS, &[&input[..split], &input[split..]], state_of);
            assert_eq!(after, initial, "split at {split}");
        }
        // A fresh matcher stays fresh over chunks without a candidate.
        with_matcher(TWO_GAPS, &[b"xxxx", b"", b"yyyy"], |m| assert!(m.fresh));
    }

    #[test]
    fn test_skipped_input_drops_stale_match_at_end() {
        // `match_at_end` describes the position where the chunk stopped.
        // If more input follows and is skipped by the prefilter, the flag
        // must not survive.
        let pattern = "foo.{0,5}bar$|qux$";
        assert!(run(pattern, &[b"fooxbar"]));
        assert!(!run(pattern, &[b"fooxbar", b"z"]));
        assert!(!run(pattern, &[b"fooxbar", b"zzzzzzzzzzzz"]));
        assert!(run(pattern, &[b"fooxbar", b"zzzzzzzzzzzz", b"qux"]));
        assert!(!run(pattern, &[b"qux", b"zzzz"]));
        assert!(run(pattern, &[b"fooxxxxxxxxxxx", b"zzz", b"foobar"]));
    }

    // ── Patterns that cannot become fresh again ──

    #[test]
    fn test_not_restartable_when_start_owns_instances() {
        // The start closure enters the counter, so the initial state has
        // seeded instances: the prefilter runs once and is not re-armed.
        with_matcher("(ab|c){2,9}x", &[], |m| {
            assert!(m.fresh);
            assert!(!m.restartable && !m.rearm);
        });
        with_matcher("(ab|c){2,9}x", &[b"zzzz"], |m| assert!(m.fresh));
        with_matcher("(ab|c){2,9}x", &[b"zzabzzzzzzzz"], |m| assert!(!m.fresh));
    }

    #[test]
    fn test_not_restartable_when_start_is_position_dependent() {
        // `^` in the start closure: the start state is only valid at
        // offset 0.
        with_matcher("^foo.{0,5}bar|baz.{0,5}qux", &[b"bazxxxxxxxxxxy"], |m| {
            assert!(!m.restartable && !m.rearm);
            assert!(!m.fresh);
        });
    }

    #[test]
    fn test_no_prefilter_still_leaves_dead_body_states() {
        // Too many start bytes for a prefilter: there is nothing to skip,
        // so landing on the start state needs no handling (`rearm` is
        // off), but dead counter-body states are still dropped so that
        // the matcher leaves the counting slow path.
        let pattern = "[a-z0-9]oo.{0,5}bar.{0,5}baz";
        with_matcher(pattern, &[], |m| {
            assert!(matches!(m.prefilter, Prefilter::None));
            assert!(m.restartable && !m.rearm);
        });
        with_matcher(pattern, &[b"aoo!!!!!!"], |m| {
            assert!(m.fresh);
            assert_ne!(m.current, m.cache.inner.start_id);
        });
        with_matcher(pattern, &[b"aoo!!!!!!!!!!"], |m| {
            assert!(!m.fresh, "the start state is not re-armed");
            assert_eq!(m.current, m.cache.inner.start_id);
        });
        assert!(run(pattern, &[b"aoo!!!!!!!!!!", b"zoo-bar-baz"]));
        assert!(!run(pattern, &[b"aoo!!!!!!!!!!", b"bar-baz"]));
    }

    // ── Prefilter back-off ──

    #[test]
    fn test_prefilter_backs_off_on_dense_candidates() {
        // Every other byte is a candidate that fails at once, so each
        // probe skips nothing.  After one window the matcher stops
        // watching for the start state.
        let dense = b"fx".repeat(PREFILTER_PROBE_WINDOW as usize + 8);
        with_matcher(TWO_GAPS, &[&dense], |m| {
            assert!(m.rearm_cooldown > 0);
            assert!(!m.fresh, "back in the start state, but not re-armed");
            assert_eq!(m.current, m.cache.inner.start_id);
        });

        // The cooldown is counted in input bytes and survives chunk
        // boundaries; when it has passed, the prefilter is armed again.
        let quiet = vec![b'x'; PREFILTER_COOLDOWN];
        with_matcher(TWO_GAPS, &[&dense, &quiet[..100]], |m| {
            assert!(m.rearm_cooldown > 0);
            assert!(!m.fresh);
        });
        with_matcher(TWO_GAPS, &[&dense, &quiet[..100], &quiet], |m| {
            assert_eq!(m.rearm_cooldown, 0);
            assert!(m.fresh);
        });
    }

    #[test]
    fn test_prefilter_stays_armed_on_sparse_candidates() {
        // Failed candidates followed by long skips: the prefilter pays.
        let sparse = b"fxxxxxxxxxxxxxxxxxxxx".repeat(4 * PREFILTER_PROBE_WINDOW as usize);
        with_matcher(TWO_GAPS, &[&sparse], |m| {
            assert_eq!(m.rearm_cooldown, 0);
            assert!(m.fresh);
        });
    }

    #[test]
    fn test_prefilter_back_off_does_not_change_results() {
        let dense = b"fx".repeat(3 * PREFILTER_PROBE_WINDOW as usize);
        let mut hit = dense.clone();
        hit.extend_from_slice(b"fooxbarxbaz");
        let mut miss = dense.clone();
        miss.extend_from_slice(b"fooxbarxxxxxxbaz");
        let mut late = dense.clone();
        late.extend_from_slice(&vec![b'x'; PREFILTER_COOLDOWN + 50]);
        late.extend_from_slice(b"foobarbaz");

        for chunk_size in [1, 7, 64, usize::MAX] {
            let chunked = |input: &[u8]| -> bool {
                let chunks: Vec<&[u8]> = input.chunks(chunk_size.min(input.len())).collect();
                run(TWO_GAPS, &chunks)
            };
            assert!(chunked(&hit), "chunk size {chunk_size}");
            assert!(!chunked(&miss), "chunk size {chunk_size}");
            assert!(chunked(&late), "chunk size {chunk_size}");
            assert!(!chunked(&dense), "chunk size {chunk_size}");
        }
    }

    // ── Cache reuse across regexes ──

    #[test]
    fn test_idle_table_cleared_when_cache_switches_regex() {
        // The idle classification is indexed by DFA state id, which means
        // nothing for another regex.  Stale entries would restart the
        // second regex in the middle of a match.
        let first = build(TWO_GAPS);
        let second = build("fooxxxxxxxxy|a.{0,5}b.{0,5}c");
        assert!(second.tier3_eligible);
        let mut memory = DfaMemory::default();
        let mut cache = Tier3DfaCache::new();

        let analysis = first.tier3_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &first, analysis);
        let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &first, analysis);
        m.chunk(b"fooxxxxxxxxxxxxqq");
        assert!(!m.finish());
        assert!(
            cache.idle.class.contains(&super::super::Idle::Yes),
            "the first regex reached an idle state"
        );

        let analysis = second.tier3_analysis.as_ref().unwrap();
        cache.prepare(&mut memory, &second, analysis);
        assert!(cache.idle.class.is_empty());
        let mut m = Tier3DfaMatcher::new(&mut cache, &mut memory, &second, analysis);
        m.chunk(b"fooxxxxxxxxy");
        assert!(m.finish());
    }
}
