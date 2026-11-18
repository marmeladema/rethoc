//! Tests of the sparse counting variant (`plans/sparse-counting-sets.md`,
//! §9).

use super::super::analysis::{self, Limits, Rejection};
use super::super::exact;
use super::super::tests::forced;
use super::super::{Increment, Row, SparseCountingError};
use super::*;
use crate::{Regex, RegexConfig};

// ---------------------------------------------------------------------------
// Algebra: exhaustive over small bounds
// ---------------------------------------------------------------------------

/// The largest maximum of the exhaustive tests: every subset of `1..=h`
/// for `h` up to it, and every pair of subsets.
const MAX_H: u32 = 8;

/// The counts of `mask`: bit `u - 1` is count `u`.
fn members(mask: u32) -> Vec<u32> {
    (1..=32).filter(|u| mask >> (u - 1) & 1 != 0).collect()
}

fn mask_of(values: &[u32]) -> u32 {
    values.iter().fold(0, |m, u| m | 1 << (u - 1))
}

/// What the future can observe of a set of counts in `{l,h}`: for every
/// number `t` of further increments, whether an exit is then possible
/// (bit `t` of the first word) and whether a count is left (second word).
/// Past `t = h` both are false.
fn observations(values: &[u32], l: u32, h: u32) -> (u32, u32) {
    let (mut exits, mut live) = (0, 0);
    for t in 0..=h {
        if values.iter().any(|u| l <= u + t && u + t <= h) {
            exits |= 1 << t;
        }
        if values.iter().any(|u| u + t <= h) {
            live |= 1 << t;
        }
    }
    (exits, live)
}

fn reduced(values: &[u32], k: u32) -> Vec<u32> {
    let mut list = values.to_vec();
    let n = reduce(&mut list, k);
    list.truncate(n);
    list
}

/// [`union`] into a scratch of exactly the room it is promised: the row
/// capacity and one more.
fn union_of(a: &[u32], b: &[u32], shift: u32, h: u32, k: u32, d: usize) -> Vec<u32> {
    let mut out = vec![0; d + 1];
    let max = if shift == 0 { u32::MAX } else { h };
    let n = union(a, b, shift, max, k, &mut out);
    out.truncate(n);
    out
}

/// A reduced list: ascending, a subset of `exact`, within the capacity,
/// with the spacing that bounds it, the same least count and the same
/// observations.  The greatest count need not be kept: `{1,2,3}` in
/// `{0,3}` is `[1,3]`, and one increment later `[2]` stands for `{2,3}`.
fn check_reduced(list: &[u32], exact: &[u32], l: u32, h: u32, k: u32, d: usize, what: &str) {
    let label = format!("{what}: {{{l},{h}}} exact {exact:?} sparse {list:?}");
    assert!(list.windows(2).all(|w| w[0] < w[1]), "{label}: order");
    assert!(list.iter().all(|u| exact.contains(u)), "{label}: subset");
    assert!(list.len() <= d, "{label}: capacity {d}");
    assert!(list.windows(3).all(|w| w[2] - w[0] > k), "{label}: spacing");
    assert_eq!(list.first(), exact.first(), "{label}: least count");
    assert_eq!(
        observations(list, l, h),
        observations(exact, l, h),
        "{label}: observations"
    );
}

/// Every `{l,h}` up to [`MAX_H`], with its spacing and capacity.
fn small_bounds() -> impl Iterator<Item = (u32, u32, u32, usize)> {
    (1..=MAX_H).flat_map(|h| {
        (0..=h).map(move |l| {
            let (k, d) = representatives(l as usize, h as usize).expect("small bounds");
            assert_eq!(k, h - l + 1);
            (l, h, k, d)
        })
    })
}

#[test]
fn reduction_keeps_every_future_observation() {
    let mut cases = 0;
    for (l, h, k, d) in small_bounds() {
        for mask in 0..1u32 << h {
            let exact = members(mask);
            let sparse = reduced(&exact, k);
            check_reduced(&sparse, &exact, l, h, k, d, "reduce");
            // Reducing a set keeps both of its ends.
            assert_eq!(sparse.last(), exact.last());
            // Reducing again changes nothing.
            assert_eq!(reduced(&sparse, k), sparse);
            cases += 1;
        }
    }
    assert_eq!(cases, 4096);
}

/// The transfers of a `CounterIncrement` on a sparse row against the exact
/// grouped [`exact::increment`] on the set it stands for, whose rows hold
/// completed iterations `c = u - 1`.
#[test]
fn increment_agrees_with_the_exact_rows() {
    for (l, h, k, d) in small_bounds() {
        let inc = Increment::Finite { min: l, max: h };
        for mask in 1..1u32 << h {
            let exact = members(mask);
            let sparse = reduced(&exact, k);
            // Exact: row 0 is the source, row 1 the continuation.  Bit `c`
            // of a row is count `c`, which is bit `u - 1`: the mask itself.
            let mut rows = [u64::from(mask), 0];
            let (src, out) = (Row { off: 0, words: 1 }, Row { off: 1, words: 1 });
            let (cont, brk) = exact::increment(&mut rows, src, out, inc);
            let continued = members(rows[1] as u32);
            assert_eq!(cont, !continued.is_empty());

            // The exit, read before the increment.
            assert_eq!(exits(&exact, l), brk, "{{{l},{h}}} {exact:?}");
            assert_eq!(
                exits(&sparse, l),
                brk,
                "{{{l},{h}}} {exact:?} as {sparse:?}"
            );
            assert_eq!(brk, observations(&exact, l, h).0 & 1 != 0);

            // The continuation: `u + 1` for the counts below `h`.
            let stepped = union_of(&[], &exact, 1, h, k, h as usize);
            let expected: Vec<u32> = exact.iter().filter(|&&u| u < h).map(|u| u + 1).collect();
            assert_eq!(continued, expected);
            let sparse_stepped = union_of(&[], &sparse, 1, h, k, d);
            // Incrementing keeps a reduced list reduced, and stands for the
            // incremented set: its observations are the old ones, one
            // iteration later.
            check_reduced(&sparse_stepped, &expected, l, h, k, d, "increment");
            assert_eq!(reduced(&stepped, k), reduced(&expected, k));
            let (e, n) = observations(&exact, l, h);
            assert_eq!(observations(&sparse_stepped, l, h), (e >> 1, n >> 1));
            // The source is only read.
            assert_eq!(rows[0], u64::from(mask));
        }
    }
}

/// A row survives exactly as long as its set: counting to exhaustion, the
/// sparse row empties at the same step, and exits at the same steps.
#[test]
fn continuation_to_exhaustion() {
    for (l, h, k, d) in small_bounds() {
        for mask in 1..1u32 << h {
            let mut exact = members(mask);
            let mut sparse = reduced(&exact, k);
            for step in 0..=h {
                assert_eq!(sparse.is_empty(), exact.is_empty(), "step {step}");
                if exact.is_empty() {
                    break;
                }
                assert_eq!(exits(&sparse, l), exact.iter().any(|&u| u >= l));
                exact = exact.iter().filter(|&&u| u < h).map(|u| u + 1).collect();
                sparse = union_of(&[], &sparse, 1, h, k, d);
                check_reduced(&sparse, &exact, l, h, k, d, "exhaustion");
            }
            assert!(exact.is_empty(), "every count passes h within h steps");
        }
    }
}

/// A `CounterInstance` (or the reentry of a completed counter) inserts
/// count one into whatever the row holds.
#[test]
fn seeding_inserts_count_one() {
    for (l, h, k, d) in small_bounds() {
        for mask in 0..1u32 << h {
            let exact = members(mask);
            let sparse = reduced(&exact, k);
            let seeded = union_of(&sparse, &[1], 0, h, k, d);
            check_reduced(&seeded, &members(mask | 1), l, h, k, d, "seed");
        }
    }
}

/// Every pair of sets, each reduced on its own, merged: the result stands
/// for the union.  Repeated merges and duplicates are pairs too.
#[test]
fn merging_reduced_rows_keeps_observations() {
    let mut pairs = 0u64;
    for (l, h, k, d) in small_bounds() {
        let size = 1u32 << h;
        let lists: Vec<Vec<u32>> = (0..size).map(|m| reduced(&members(m), k)).collect();
        let observed: Vec<(u32, u32)> =
            (0..size).map(|m| observations(&members(m), l, h)).collect();
        let mut out = vec![0u32; d + 1];
        for left in 0..size {
            for right in 0..size {
                let n = union(
                    &lists[left as usize],
                    &lists[right as usize],
                    0,
                    u32::MAX,
                    k,
                    &mut out,
                );
                let merged = &out[..n];
                assert!(n <= d, "{{{l},{h}}} {left:#b} | {right:#b}: capacity");
                let both = left | right;
                assert_eq!(mask_of(merged) & !both, 0, "subset of the union");
                assert_eq!(
                    observations(merged, l, h),
                    observed[both as usize],
                    "{{{l},{h}}} {left:#b} | {right:#b} as {merged:?}"
                );
                assert!(merged.windows(3).all(|w| w[2] - w[0] > k));
                pairs += 1;
            }
        }
    }
    assert_eq!(pairs, 757_304);
}

/// Random long unions and increments on wide bounds, against bitsets.
#[test]
fn random_operations_on_wide_bounds() {
    let mut rng = super::super::tests::Rng(0x5eed_5a11);
    for &(l, h) in &[
        (1u32, 1000u32),
        (500, 1000),
        (900, 1000),
        (0, 300),
        (64, 64),
        (37, 41),
    ] {
        let (k, d) = representatives(l as usize, h as usize).unwrap();
        let mut exact: Vec<bool> = vec![false; h as usize + 1];
        let mut sparse: Vec<u32> = Vec::new();
        for _ in 0..4000 {
            match rng.below(4) {
                // Seed.
                0 => {
                    exact[1] = true;
                    sparse = union_of(&sparse, &[1], 0, h, k, d);
                }
                // Merge a few random counts, as another path would bring.
                1 => {
                    let mut other: Vec<u32> = (0..rng.below(4))
                        .map(|_| 1 + rng.below(h as usize) as u32)
                        .collect();
                    other.sort_unstable();
                    other.dedup();
                    for &u in &other {
                        exact[u as usize] = true;
                    }
                    sparse = union_of(&sparse, &reduced(&other, k), 0, h, k, d);
                }
                // Increment.
                _ => {
                    for u in (1..h as usize).rev() {
                        exact[u + 1] = exact[u];
                    }
                    exact[1] = false;
                    sparse = union_of(&[], &sparse, 1, h, k, d);
                }
            }
            let set: Vec<u32> = (1..=h).filter(|&u| exact[u as usize]).collect();
            assert!(sparse.len() <= d && sparse.iter().all(|u| set.contains(u)));
            assert_eq!(sparse.first(), set.first());
            // The exit profile: a union of intervals, compared as sets of
            // `t` by sweeping both.
            let profile = |values: &[u32]| -> Vec<bool> {
                let mut can = vec![false; h as usize + 1];
                for &u in values {
                    for t in l.saturating_sub(u)..=h - u {
                        can[t as usize] = true;
                    }
                }
                can
            };
            assert_eq!(profile(&sparse), profile(&set), "{{{l},{h}}}");
        }
    }
}

// ---------------------------------------------------------------------------
// Mutations the tests above must catch
// ---------------------------------------------------------------------------

/// Pruning at `k + 1` instead of `k` loses an observation.
#[test]
fn mutation_pruning_one_too_wide_is_caught() {
    let (l, h) = (3, 3);
    let (k, _) = representatives(3, 3).unwrap();
    let exact = [1, 2, 3];
    assert_eq!(reduced(&exact, k), exact, "nothing is redundant");
    let wrong = reduced(&exact, k + 1);
    assert_eq!(wrong, [1, 3]);
    assert_ne!(observations(&wrong, l, h), observations(&exact, l, h));
}

/// Keeping only the least and the greatest count of a holey set invents
/// nothing but loses the counts between them.
#[test]
fn mutation_extrema_only_is_caught() {
    let (l, h) = (6, 8);
    let (k, _) = representatives(6, 8).unwrap();
    let exact = [1, 4, 7];
    assert_eq!(reduced(&exact, k), exact);
    assert_ne!(observations(&[1, 7], l, h), observations(&exact, l, h));
}

/// Testing the exit after the increment misses the exit at `h`.
#[test]
fn mutation_exit_after_increment_is_caught() {
    let (k, d) = representatives(3, 3).unwrap();
    let row = [3];
    assert!(exits(&row, 3));
    let stepped = union_of(&[], &row, 1, 3, k, d);
    assert!(stepped.is_empty());
    assert!(!exits(&stepped, 3));
}

/// A filter on the counts (here "at most 7") tells a reduced row from its
/// set: such a transfer does not exist.
#[test]
fn mutation_a_count_filter_would_be_unsound() {
    let (l, h) = (5, 10);
    let (k, _) = representatives(5, 10).unwrap();
    let exact: Vec<u32> = (4..=10).collect();
    let sparse = reduced(&exact, k);
    assert_eq!(sparse, [4, 10]);
    let filter = |v: &[u32]| -> Vec<u32> { v.iter().copied().filter(|&u| u <= 7).collect() };
    assert_ne!(
        observations(&filter(&sparse), l, h),
        observations(&filter(&exact), l, h)
    );
}

// ---------------------------------------------------------------------------
// The capacity
// ---------------------------------------------------------------------------

#[test]
fn capacity_formula() {
    // The plan's examples (§10).
    assert_eq!(representatives(1, 1000), Some((1000, 2)));
    assert_eq!(representatives(500, 1000), Some((501, 4)));
    assert_eq!(representatives(900, 1000), Some((101, 20)));
    assert_eq!(representatives(1000, 1000), Some((1, 1000)));
    assert_eq!(representatives(2, 8), Some((7, 2)));
    assert_eq!(representatives(0, 5), Some((6, 2)));
    // The trivial bound: no more values than counts.
    assert_eq!(representatives(0, 1), Some((2, 1)));
    assert_eq!(representatives(1, 1), Some((1, 1)));
    // Not defined.
    assert_eq!(representatives(0, 0), None);
    assert_eq!(representatives(3, 2), None);
    assert_eq!(representatives(1, usize::MAX), None);
    assert_eq!(representatives(1, u32::MAX as usize), None);
    // The largest maximum: `k = max + 1` still fits.
    let top = u32::MAX as usize - 1;
    assert_eq!(representatives(0, top), Some((u32::MAX, 2)));
    assert_eq!(representatives(top, top), Some((1, top)));
}

/// The capacity bounds every reduced list, and is not slack everywhere:
/// some bounds reach it.
#[test]
fn capacity_bounds_reduced_lists_and_is_reached() {
    let mut reached = 0;
    for (_, h, k, d) in small_bounds() {
        let most = (0..1u32 << h)
            .map(|m| reduced(&members(m), k).len())
            .max()
            .unwrap();
        assert!(most <= d, "h={h} k={k}: {most} values, capacity {d}");
        reached += usize::from(most == d);
    }
    assert!(reached > 20, "reached for {reached} bounds");
}

// ---------------------------------------------------------------------------
// Admission
// ---------------------------------------------------------------------------

/// Compile with sparse counting analysed.
pub(crate) fn sparse_compile(pattern: &str, unroll: usize, merge: bool) -> Regex {
    Regex::with_config(
        pattern,
        RegexConfig {
            max_unroll_states: unroll,
            merge_repetitions: merge,
            max_estimated_states: 65_536,
            grouped_nfa: crate::GroupedNfaMode::Always,
            sparse_counting: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| panic!("`{pattern}` should compile: {e}"))
}

/// Counters forced: no unrolling, no merging.
pub(crate) fn sparse_forced(pattern: &str) -> Regex {
    sparse_compile(pattern, 0, false)
}

fn sparse_plan(re: &Regex) -> &SparsePlan {
    re.sparse_plan()
        .unwrap_or_else(|e| panic!("expected admission: {e}"))
}

fn sparse_rejection(re: &Regex) -> SparseRejection {
    match re.sparse_plan() {
        Err(SparseCountingError::Rejected(r)) => r,
        other => panic!("expected a rejection, got {other:?}"),
    }
}

/// The capacities of the counted rows of a plan, without repeats.
fn capacities(plan: &SparsePlan) -> Vec<u32> {
    let mut caps: Vec<u32> = plan
        .nodes
        .iter()
        .filter(|n| n.scope != Scope::Outside)
        .map(|n| n.cap)
        .collect();
    caps.sort_unstable();
    caps.dedup();
    caps
}

#[test]
fn admits_finite_flat_counters() {
    for (pattern, caps) in [
        ("x(a|aa|b){1,1000}z", vec![2]),
        ("x(a|aa|b){500,1000}z", vec![4]),
        ("x(a|aa|b){900,1000}z", vec![20]),
        ("a(a|aa){1,1000}z", vec![2]),
        ("x(a|aaa){2,8}z", vec![2]),
        ("x(a|aa){1,100}y(b|bb){40,100}z", vec![2, 4]),
        ("(a{2,5}b)*c", vec![2]),
        ("(ab){1,1000}z", vec![2]),
        ("x(a|aa){0,7}z", vec![2]),
        ("x(a|aa){3}z", vec![3]),
    ] {
        let re = sparse_forced(pattern);
        let plan = sparse_plan(&re);
        assert_eq!(capacities(plan), caps, "`{pattern}`");
        // The layout: rows in node order, Outside rows without slots.
        let mut off = 0;
        for node in plan.nodes.iter() {
            assert_eq!(node.off, off, "`{pattern}`");
            assert_eq!(node.cap == 0, node.scope == Scope::Outside);
            off += node.cap;
        }
        assert_eq!(off as usize, plan.slots);
        assert_eq!(plan.max_cap, *caps.last().unwrap() as usize);
        assert_eq!(plan.rows(), plan.nodes.len());
    }
    // Counter-free graphs are admitted, without a slot.
    for pattern in ["abc|abd", "", "x(a|aa){1,3}z"] {
        let re = sparse_compile(pattern, 32, true);
        let plan = sparse_plan(&re);
        assert_eq!((plan.slots, plan.max_cap), (0, 0), "`{pattern}`");
    }
}

#[test]
fn rejects_outside_the_admitted_class() {
    // Exact counts: the lists cannot be shorter than the set.
    assert_eq!(
        sparse_rejection(&sparse_forced("x(a|aa|b){1000}z")),
        SparseRejection::TooManyRepresentatives {
            counter: 0,
            needed: 1000,
            limit: 32
        }
    );
    assert!(matches!(
        sparse_rejection(&sparse_forced("x(a|aa){5,}z")),
        SparseRejection::UnboundedCounter { counter: 0 }
    ));
    assert!(matches!(
        sparse_rejection(&sparse_forced("x((a|aa){1,10}b){1,10}z")),
        SparseRejection::Structure(Rejection::NestedCounter { .. })
    ));
    assert!(matches!(
        sparse_rejection(&sparse_forced("(a?){2,5}z")),
        SparseRejection::Structure(Rejection::ZeroWidthCycle { .. })
    ));
    // A counted body that an assertion makes nullable is a zero-width
    // cycle, like any other.
    assert!(matches!(
        sparse_rejection(&sparse_forced(r"(?:\b|a){2,3}")),
        SparseRejection::Structure(Rejection::ZeroWidthCycle { .. })
    ));
    // Assertions are admitted, anywhere.
    for pattern in [
        r"x(a|aa){2,5}\bz",
        r"\bx(a|aa){2,5}z",
        r"x(a\b|aa){2,5}z",
        "^abc",
    ] {
        sparse_plan(&sparse_forced(pattern));
    }
}

/// The exact grouped NFA and the sparse variant are admitted separately:
/// each can reject what the other runs.
#[test]
fn admission_is_independent_of_the_exact_rows() {
    // Admitted by both, rejected by sparse only, by exact only.
    let re = sparse_forced("x(a|aa|b){1,1000}z");
    assert!(re.grouped_plan().is_ok() && re.sparse_plan().is_ok());
    let re = sparse_forced("x(a|aa){5,}z");
    assert!(re.grouped_plan().is_ok() && re.sparse_plan().is_err());

    // Eight alternatives of a hundred bytes each: some 800 counted rows
    // of 16 words, over the exact frontier of 8,192 words, and two sparse
    // slots each.
    let body: Vec<String> = (0..8u8)
        .map(|i| ((b'a' + i) as char).to_string().repeat(100))
        .collect();
    let pattern = format!("x({}){{1,1000}}!", body.join("|"));
    let re = Regex::with_config(
        &pattern,
        RegexConfig {
            max_unroll_states: 0,
            merge_repetitions: false,
            max_estimated_states: 1 << 20,
            grouped_nfa: crate::GroupedNfaMode::Always,
            sparse_counting: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(
        matches!(
            super::super::tests::rejection(&re),
            Rejection::TooManyRowWords { .. }
        ),
        "{:?}",
        re.grouped_plan().err()
    );
    let plan = sparse_plan(&re);
    assert_eq!(capacities(plan), [2]);
    assert!((1500..1700).contains(&plan.slots), "{} slots", plan.slots);
}

/// The same structure under a tiny exact row budget: only the exact
/// layout rejects.
#[test]
fn sparse_layout_reads_the_structure_before_the_row_budget() {
    let re = forced("x(a|aa){100,200}z");
    let structure = analysis::structure(
        &re.states,
        &re.byte_tables,
        re.start,
        re.num_counters,
        &[],
        &Limits::DEFAULT,
    )
    .unwrap();
    let narrow = Limits {
        max_row_words: 8,
        ..Limits::DEFAULT
    };
    assert!(matches!(
        analysis::layout(
            &re.states,
            &re.classes,
            &re.byte_tables,
            re.start,
            &structure,
            &narrow
        ),
        Err(Rejection::TooManyRowWords { .. })
    ));
    let plan = analyze(
        &re.states,
        &re.classes,
        &re.byte_tables,
        re.start,
        Ok(&structure),
        &SparseLimits::DEFAULT,
    )
    .unwrap();
    assert_eq!(capacities(&plan), [4]);
}

/// Each limit below, at and above its boundary.
#[test]
fn limits_below_at_and_above() {
    let re = sparse_forced("x(a|aa){900,1000}z");
    let structure = analysis::structure(
        &re.states,
        &re.byte_tables,
        re.start,
        re.num_counters,
        &[],
        &Limits::DEFAULT,
    )
    .unwrap();
    let run = |limits: SparseLimits| {
        analyze(
            &re.states,
            &re.classes,
            &re.byte_tables,
            re.start,
            Ok(&structure),
            &limits,
        )
    };
    let full = run(SparseLimits::DEFAULT).unwrap();
    assert_eq!(full.max_cap, 20);

    let with = |f: &dyn Fn(&mut SparseLimits)| {
        let mut l = SparseLimits::DEFAULT;
        f(&mut l);
        run(l).map(|_| ())
    };
    assert_eq!(with(&|l| l.max_reps = 20), Ok(()));
    assert_eq!(
        with(&|l| l.max_reps = 19),
        Err(SparseRejection::TooManyRepresentatives {
            counter: 0,
            needed: 20,
            limit: 19
        })
    );
    assert_eq!(with(&|l| l.max_slots = full.slots), Ok(()));
    assert_eq!(
        with(&|l| l.max_slots = full.slots - 1),
        Err(SparseRejection::TooManySlots {
            slots: full.slots,
            limit: full.slots - 1
        })
    );
    // The plan budget drops the optional idle table first.
    let bytes = full.heap_bytes();
    assert!(full.idle.is_some());
    assert_eq!(with(&|l| l.max_plan_bytes = bytes), Ok(()));
    let without = run(SparseLimits {
        max_plan_bytes: bytes - 1,
        ..SparseLimits::DEFAULT
    })
    .unwrap();
    assert!(without.idle.is_none());
    assert_eq!(without.heap_bytes(), bytes - 256);
    assert_eq!(with(&|l| l.max_plan_bytes = bytes - 256), Ok(()));
    assert_eq!(
        with(&|l| l.max_plan_bytes = bytes - 257),
        Err(SparseRejection::PlanTooLarge {
            bytes: bytes - 256,
            limit: bytes - 257
        })
    );
    let analysis = sparse_analysis_bytes(re.states.len(), re.num_counters);
    assert_eq!(with(&|l| l.max_analysis_bytes = analysis), Ok(()));
    assert_eq!(
        with(&|l| l.max_analysis_bytes = analysis - 1),
        Err(SparseRejection::AnalysisTooLarge {
            bytes: analysis,
            limit: analysis - 1
        })
    );
    // A raised capacity admits an exact count, for the negative controls.
    let exact = forced("x(a|aa){40}z");
    let structure = analysis::structure(
        &exact.states,
        &exact.byte_tables,
        exact.start,
        exact.num_counters,
        &[],
        &Limits::DEFAULT,
    )
    .unwrap();
    let raised = SparseLimits {
        max_reps: 40,
        ..SparseLimits::DEFAULT
    };
    assert_eq!(
        analyze(
            &exact.states,
            &exact.classes,
            &exact.byte_tables,
            exact.start,
            Ok(&structure),
            &raised
        )
        .unwrap()
        .max_cap,
        40
    );
    // A structural rejection is passed on.
    assert_eq!(
        analyze(
            &re.states,
            &re.classes,
            &re.byte_tables,
            re.start,
            Err(Rejection::TooManyEdges { limit: 1 }),
            &SparseLimits::DEFAULT
        )
        .err(),
        Some(SparseRejection::Structure(Rejection::TooManyEdges {
            limit: 1
        }))
    );
}

/// Requesting the analysis changes neither the NFA, nor the selected
/// engine, nor the exact plan; without the request there is no plan.
#[test]
fn the_option_changes_nothing_else() {
    for pattern in [
        "x(a|aa|b){1,1000}z",
        "abc|abd",
        "x(a|aa){5,}z",
        r"x(a|aa){2,5}\bz",
    ] {
        for &(unroll, merge) in &[(32usize, true), (0, false)] {
            let config = RegexConfig {
                max_unroll_states: unroll,
                merge_repetitions: merge,
                max_estimated_states: 65_536,
                grouped_nfa: crate::GroupedNfaMode::Always,
                ..Default::default()
            };
            let plain = Regex::with_config(pattern, config.clone()).unwrap();
            let re = sparse_compile(pattern, unroll, merge);
            assert!(!config.sparse_counting);
            assert_eq!(format!("{:?}", re.states), format!("{:?}", plain.states));
            assert_eq!(re.engine(), plain.engine(), "`{pattern}`");
            assert_eq!(
                re.grouped_nfa_admission(),
                plain.grouped_nfa_admission(),
                "`{pattern}`"
            );
            assert_eq!(
                plain.sparse_counting_admission(),
                Err(SparseCountingError::NotRequested)
            );
            assert_ne!(
                re.sparse_counting_admission(),
                Err(SparseCountingError::NotRequested)
            );
            assert!(re.memory_size() >= plain.memory_size());
        }
    }
}

// ---------------------------------------------------------------------------
// Matching: against Tier 0, the regex crate and the exact rows
// ---------------------------------------------------------------------------

use super::super::exact::Sweep;
use super::super::tests::{CONFIGS, Rng, all_strings, oracle};
use super::super::{GroupedScratch, GroupedStream};
use crate::{CounterIdx, MatcherMemory, StateIdx};

/// Every pass a stream can be forced to.  `Auto` also skips while idle;
/// `Full` and `Active` run the general step on every byte.
pub(crate) const SWEEPS: [Sweep; 3] = [Sweep::Auto, Sweep::Full, Sweep::Active];

/// Feed `chunks` to a sparse stream of every pass and to Tier 0 side by
/// side: `ismatch` must agree after construction and after every call
/// (Tier 0 unfinished: a prefix is not an input), and `finish` with both
/// and `expected`.  The raw frontier keeps its invariants and the scratch
/// is left clean.
pub(crate) fn run_chunks(re: &Regex, chunks: &[&[u8]], expected: bool, label: &str) {
    let mut streams: Vec<(Sweep, SparseScratch, SparseStream)> = SWEEPS
        .iter()
        .map(|&sweep| {
            let mut scratch = SparseScratch::new();
            scratch.set_sweep(sweep);
            let stream = SparseStream::new(re, &mut scratch).unwrap();
            (sweep, scratch, stream)
        })
        .collect();
    let mut mem = MatcherMemory::default();
    let mut nfa = mem.matcher_for_tier(re, 0).unwrap();
    for (sweep, _, stream) in &streams {
        assert_eq!(
            stream.ismatch(),
            nfa.ismatch(),
            "{label} {sweep:?}: before input"
        );
    }
    for (i, chunk) in chunks.iter().enumerate() {
        nfa.chunk(chunk);
        for (sweep, scratch, stream) in &mut streams {
            stream.chunk(re, scratch, chunk);
            assert_eq!(
                stream.ismatch(),
                nfa.ismatch(),
                "{label} {sweep:?}: after chunk {i}"
            );
            stream.check_invariants(re, label);
            assert!(scratch.is_clean(), "{label} {sweep:?}: scratch left dirty");
        }
    }
    assert_eq!(nfa.finish(), expected, "{label}: tier 0");
    for (sweep, scratch, stream) in &mut streams {
        assert_eq!(
            stream.finish(re, scratch),
            expected,
            "{label} {sweep:?}: sparse"
        );
        assert!(scratch.is_clean(), "{label} {sweep:?}: scratch left dirty");
    }
}

/// Whole, bytewise, and a random partition with empty chunks.
pub(crate) fn check_input(
    re: &Regex,
    oracle: &regex::bytes::Regex,
    input: &[u8],
    seed: u64,
    label: &str,
) {
    let expected = oracle.is_match(input);
    run_chunks(re, &[input], expected, &format!("{label} whole {input:?}"));
    let bytes: Vec<&[u8]> = input.chunks(1).collect();
    run_chunks(re, &bytes, expected, &format!("{label} bytewise {input:?}"));
    let mut rng = Rng(seed | 1);
    let mut parts: Vec<&[u8]> = vec![&[]];
    let mut i = 0;
    while i < input.len() {
        let end = (i + 1 + rng.below(4)).min(input.len());
        parts.push(&input[i..end]);
        if rng.below(3) == 0 {
            parts.push(&[]);
        }
        i = end;
    }
    run_chunks(re, &parts, expected, &format!("{label} chunked {input:?}"));
}

/// Feed `input` whole and bytewise to a sparse stream of every pass,
/// without Tier 0.
fn check_sparse_only(re: &Regex, expected: bool, input: &[u8], label: &str) {
    for sweep in SWEEPS {
        let mut scratch = SparseScratch::new();
        scratch.set_sweep(sweep);
        let mut s = SparseStream::new(re, &mut scratch).unwrap();
        s.chunk(re, &mut scratch, input);
        assert_eq!(
            s.finish(re, &mut scratch),
            expected,
            "{label} {sweep:?} whole {input:?}"
        );
        s.reset(re, &mut scratch);
        for b in input.chunks(1) {
            s.chunk(re, &mut scratch, b);
        }
        assert_eq!(
            s.finish(re, &mut scratch),
            expected,
            "{label} {sweep:?} bytewise {input:?}"
        );
    }
}

/// The sparse part of a `match_tests!` entry, for one of its builds
/// (`re`, compiled with `config`): compile the same configuration with
/// the sparse option; when admitted, check that the NFA is the one the
/// other engines ran on, then run the listed inputs (against Tier 0, the
/// oracle and the exact rows, with chunk splits) and the generated ones
/// (against the oracle).  Returns whether the build was admitted.
pub(crate) fn check_match_test_build(
    pattern: &str,
    config: &RegexConfig,
    re: &Regex,
    oracle: &regex::bytes::Regex,
    inputs: &[(&str, bool)],
    generated: &[Vec<u8>],
    label: &str,
) -> bool {
    let sparse = Regex::with_config(
        pattern,
        RegexConfig {
            grouped_nfa: crate::GroupedNfaMode::Always,
            sparse_counting: true,
            ..config.clone()
        },
    )
    .expect("compiles like the build without the option");
    if sparse.sparse_plan().is_err() {
        return false;
    }
    assert_eq!(
        format!("{:?}", sparse.states),
        format!("{:?}", re.states),
        "`{pattern}` ({label}): the sparse option changed the NFA"
    );
    let label = format!("`{pattern}` ({label}) sparse");
    for (k, &(input, _)) in inputs.iter().enumerate() {
        check_input(&sparse, oracle, input.as_bytes(), k as u64, &label);
        check_profiles(&sparse, input.as_bytes(), &label);
    }
    for input in generated {
        check_sparse_only(&sparse, oracle.is_match(input), input, &label);
    }
    true
}

/// The bounds of each counter of a sparse plan, from its increments.
fn counter_bounds(plan: &SparsePlan) -> Vec<Option<(u32, u32)>> {
    let mut bounds = vec![None; 256];
    for node in plan.nodes.iter() {
        if let (Scope::Counter(c), SparseOp::Increment { min, max, .. }) = (node.scope, node.op) {
            bounds[c.idx()] = Some((min, max));
        }
    }
    bounds
}

/// For every `t`, whether a set of counts can exit after `t` more
/// iterations.
fn exit_profile(values: &[u32], l: u32, h: u32) -> Vec<bool> {
    // The union of the intervals `[max(0, l - u), h - u]`, by their ends.
    let mut delta = vec![0i32; h as usize + 2];
    for &u in values {
        delta[l.saturating_sub(u) as usize] += 1;
        delta[(h - u) as usize + 1] -= 1;
    }
    let mut open = 0;
    delta[..=h as usize]
        .iter()
        .map(|d| {
            open += d;
            open > 0
        })
        .collect()
}

/// Run the exact grouped NFA and the sparse variant side by side, a byte
/// at a time, and compare their raw rows after every byte: the same rows
/// are active, and every counted sparse row is a subset of the exact
/// counts (as `u = c + 1`) with the same least count and the same exit
/// profile.  Skipped when the exact grouped NFA does not admit the graph.
pub(crate) fn check_profiles(re: &Regex, input: &[u8], label: &str) {
    let Ok(exact_plan) = re.grouped_plan() else {
        return;
    };
    let plan = re.sparse_plan().unwrap();
    let bounds = counter_bounds(plan);
    let mut exact_scratch = GroupedScratch::new();
    let mut exact = GroupedStream::new(re, &mut exact_scratch).unwrap();
    // One sparse stream per pass.
    let mut streams: Vec<(Sweep, SparseScratch, SparseStream)> = SWEEPS
        .iter()
        .map(|&sweep| {
            let mut scratch = SparseScratch::new();
            scratch.set_sweep(sweep);
            let stream = SparseStream::new(re, &mut scratch).unwrap();
            (sweep, scratch, stream)
        })
        .collect();
    let mut counts: Vec<u32> = Vec::new();
    for (i, b) in input.chunks(1).enumerate() {
        exact.chunk(re, &mut exact_scratch, b);
        for (sweep, scratch, sparse) in &mut streams {
            sparse.chunk(re, scratch, b);
            assert_eq!(
                sparse.ismatch(),
                exact.ismatch(),
                "{label} {sweep:?} {input:?} after byte {i}: match"
            );
        }
        if exact.ismatch() {
            // All stopped before this byte.
            return;
        }
        for (state, scope, row) in exact_plan.rows_iter() {
            let words = &exact.raw()[row.off as usize..(row.off + row.words) as usize];
            counts.clear();
            for (w, &word) in words.iter().enumerate() {
                let mut bits = word;
                while bits != 0 {
                    counts.push(w as u32 * 64 + bits.trailing_zeros() + 1);
                    bits &= bits - 1;
                }
            }
            for (sweep, _, sparse) in &streams {
                assert_eq!(
                    sparse.raw_active(re, state),
                    !counts.is_empty(),
                    "{label} {sweep:?} {input:?} after byte {i}: state {state} activity"
                );
                let Scope::Counter(c) = scope else { continue };
                let (l, h) = bounds[c.idx()].expect("a counted scope has an increment");
                let reps = sparse.raw_counts(re, state).unwrap();
                let ok = reps.windows(2).all(|w| w[0] < w[1])
                    && reps.iter().all(|u| counts.binary_search(u).is_ok())
                    && counts.iter().all(|&u| (1..=h).contains(&u))
                    && reps.first() == counts.first()
                    && exit_profile(reps, l, h) == exit_profile(&counts, l, h);
                assert!(
                    ok,
                    "{label} {sweep:?} {input:?} after byte {i}: state {state} {{{l},{h}}}: \
                     sparse {reps:?} does not stand for exact {counts:?} (order, subset, range, \
                     least count or exit profile)"
                );
            }
        }
    }
}

/// Assertion-free patterns with finite counters, and an alphabet that
/// reaches their interesting configurations.
const PATTERNS: &[(&str, &[u8])] = &[
    ("x(a|aa){2,5}z", b"xaz"),
    ("x(a|aaa){3}z", b"xaz"),
    ("(a|aa){2}b(a|aa){2}", b"ab"),
    ("(a{2,3}b)*c", b"abc"),
    ("a{0,2}b", b"ab"),
    ("x(ab*){2,5}z", b"xabz"),
    ("x(ab|cd|ef){2,5}z", b"xabcz"),
    ("a{2,5}b", b"ab"),
    ("a{3}b{2,4}", b"ab"),
    ("(a{2,4}|b{3,5})c", b"abc"),
    ("x(a|aa|b){3,6}z", b"xabz"),
    ("[a-c]{2,4}x", b"abx"),
    ("(?i)k{2,3}z", b"kKz"),
    (".{3,5}a", b"ab"),
    ("(ab|cd){1,8}e", b"abcde"),
    ("a(a|aa){1,6}b", b"ab"),
    ("a{0,3}", b"ab"),
    ("abc|abd", b"abcd"),
    ("", b"a"),
    // Holey count sets.
    ("x(a|aaa){2,8}z", b"xaz"),
    // Two scopes in sequence, and in two branches.
    ("x(a|aa){1,4}y(b|bb){2,4}z", b"xaybz"),
    ("x[ab]{4,6}y|x[ab]{2,3}z", b"xabyz"),
    // A loop around a completed scope: reentry resets to one.
    ("(a{2,5}b)*c", b"abc"),
    ("(x(a|aa){2,4}y)+z", b"xayz"),
    // Entries that do not kill the older ones.
    ("x(a|aa|x){3,6}z", b"xaz"),
    ("a(a|aa){2,7}z", b"az"),
    // A zero minimum: the bypass stays Outside.
    ("x(a|aa){0,3}z", b"xaz"),
    // Paths of different counts meeting at one state.
    ("x(a|b|ab|ba){2,7}z", b"xabz"),
    ("x(a|aa|aaa){4,9}z", b"xaz"),
    // The exit at the minimum, the continuation gone at the maximum.
    ("x(a|aa){4}z", b"xaz"),
    ("x(a|aa){5,6}z", b"xaz"),
];

#[test]
fn results_match_tier0_and_regex_crate() {
    let mut admitted = 0;
    for &(pattern, alphabet) in PATTERNS {
        let oracle = oracle(pattern);
        for (unroll, merge) in CONFIGS {
            let re = sparse_compile(pattern, unroll, merge);
            if re.sparse_plan().is_err() {
                // Only under default compilation, where a pattern can end
                // without a counter... and then it is admitted too.
                panic!(
                    "`{pattern}` unroll={unroll} merge={merge}: {:?}",
                    re.sparse_plan().err()
                );
            }
            admitted += 1;
            let label = format!("`{pattern}` unroll={unroll} merge={merge}");
            for (k, input) in all_strings(alphabet, 5).iter().enumerate() {
                check_input(&re, &oracle, input, k as u64, &label);
            }
        }
    }
    assert_eq!(admitted, PATTERNS.len() * CONFIGS.len());
}

#[test]
fn raw_rows_stand_for_the_exact_rows() {
    for &(pattern, alphabet) in PATTERNS {
        for (unroll, merge) in CONFIGS {
            let re = sparse_compile(pattern, unroll, merge);
            let label = format!("`{pattern}` unroll={unroll} merge={merge}");
            let max_len = if alphabet.len() <= 3 { 7 } else { 5 };
            for input in all_strings(alphabet, max_len) {
                check_profiles(&re, &input, &label);
            }
        }
    }
}

/// Random input over `pieces`, some of them several bytes long.
fn random_input(rng: &mut Rng, pieces: &[&[u8]], len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(len + 8);
    while out.len() < len {
        out.extend_from_slice(pieces[rng.below(pieces.len())]);
    }
    out
}

/// Wide bounds on long inputs: results against the oracle and Tier 0,
/// rows against the exact grouped NFA.  Half of the inputs are random;
/// the others plant one episode whose length is near a bound: `start`,
/// a run of `first`, then `mid` and a run of `second`, then `end`.
#[test]
fn wide_bounds_on_long_inputs() {
    struct Case {
        pattern: &'static str,
        /// The bounds the planted lengths are chosen around.
        bounds: (usize, usize),
        start: &'static [u8],
        first: &'static [&'static [u8]],
        mid: &'static [u8],
        second: &'static [&'static [u8]],
        end: &'static [u8],
    }
    let case = |pattern, bounds, start, first, end| Case {
        pattern,
        bounds,
        start,
        first,
        mid: b"",
        second: &[],
        end,
    };
    let a: &[&[u8]] = &[b"a"];
    let ab: &[&[u8]] = &[b"a", b"a", b"aa", b"b"];
    let cases = [
        case("x(a|aa|b){1,1000}z", (1, 1000), b"x", ab, b"z"),
        case("x(a|aa|b){500,1000}z", (500, 1000), b"x", ab, b"z"),
        case("x(a|aa|b){900,1000}z", (900, 1000), b"x", ab, b"z"),
        case("x(a|aa|b){200,300}z", (200, 300), b"x", a, b"z"),
        case("a(a|aa){1,1000}z", (1, 1000), b"a", a, b"z"),
        case(
            "x(a|aa|x){150,300}z",
            (150, 300),
            b"x",
            &[b"a", b"aa", b"x"],
            b"z",
        ),
        case("x(a|aaa){2,8}z", (2, 8), b"x", a, b"z"),
        case("(ab){1,1000}z", (1, 1000), b"", &[b"ab"], b"z"),
        case(
            "(a{2,5}b)*c",
            (2, 5),
            b"",
            &[b"aab", b"aaaaab", b"aaab"],
            b"c",
        ),
        Case {
            pattern: "x(a|aa){1,100}y(b|bb){40,100}z",
            bounds: (40, 100),
            start: b"x",
            first: a,
            mid: b"y",
            second: &[b"b"],
            end: b"z",
        },
    ];
    let noise: &[&[u8]] = &[
        b"a",
        b"a",
        b"aa",
        b"aaaaaaaa",
        b"b",
        b"x",
        b"z",
        b"q",
        b"y",
        b"c",
    ];
    let mut rng = Rng(0x00c0_ffee);
    for c in &cases {
        let oracle = oracle(c.pattern);
        let re = sparse_forced(c.pattern);
        sparse_plan(&re);
        let (l, h) = c.bounds;
        // Byte lengths around the bounds, for bodies of one or two bytes.
        let lengths = [
            0,
            1,
            l.saturating_sub(1),
            l,
            l + 1,
            h - 1,
            h,
            h + 1,
            2 * h - 1,
            2 * h,
            2 * h + 1,
            2 * h + 2,
        ];
        let (mut matches, mut misses) = (0, 0);
        for round in 0..24 {
            let input = if round % 2 == 0 {
                random_input(&mut rng, noise, [20, 200, 500, 900][round / 2 % 4])
            } else {
                let lead = rng.below(4);
                let mut input = random_input(&mut rng, &[b"q", b"qq"], lead);
                input.extend_from_slice(c.start);
                let n = lengths[round / 2 % lengths.len()];
                let run = random_input(&mut rng, c.first, n);
                input.extend_from_slice(&run[..n.min(run.len())]);
                if !c.second.is_empty() {
                    input.extend_from_slice(c.mid);
                    let m = lengths[rng.below(lengths.len())];
                    input.extend(random_input(&mut rng, c.second, m));
                }
                // One round in four stops short of the end.
                if round % 8 != 7 {
                    input.extend_from_slice(c.end);
                }
                let tail = rng.below(4);
                input.extend(random_input(&mut rng, &[b"q", b"a"], tail));
                input
            };
            let expected = oracle.is_match(&input);
            if expected {
                matches += 1;
            } else {
                misses += 1;
            }
            let label = format!("`{}` round {round}", c.pattern);
            check_sparse_only(&re, expected, &input, &label);
            // Tier 0 keeps one thread per count: on the shorter inputs.
            if input.len() <= 300 {
                let parts: Vec<&[u8]> = input.chunks(1 + rng.below(40)).collect();
                run_chunks(&re, &parts, expected, &label);
            }
            check_profiles(&re, &input, &label);
        }
        assert!(
            matches >= 2 && misses >= 3,
            "`{}`: {matches} matches, {misses} misses",
            c.pattern
        );
    }
}

/// Every partition of short inputs into chunks.
#[test]
fn every_partition_of_short_inputs() {
    for (pattern, alphabet) in [
        ("x(a|aa){2,5}z", b"xaz".as_slice()),
        ("(x(a|aa){2,4}y)+z", b"xayz"),
        ("a{0,2}b", b"ab"),
    ] {
        let oracle = oracle(pattern);
        let re = sparse_forced(pattern);
        for input in all_strings(alphabet, 6) {
            let expected = oracle.is_match(&input);
            let cuts = input.len().saturating_sub(1);
            for mask in 0..1u32 << cuts {
                let mut parts: Vec<&[u8]> = Vec::new();
                let mut from = 0;
                for cut in 0..cuts {
                    if mask >> cut & 1 != 0 {
                        parts.push(&input[from..=cut]);
                        from = cut + 1;
                    }
                }
                parts.push(&input[from..]);
                run_chunks(
                    &re,
                    &parts,
                    expected,
                    &format!("`{pattern}` {input:?} cuts {mask:#b}"),
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Transfers on hand-picked rows
// ---------------------------------------------------------------------------

/// The state of `re` matching `pred`, which must be unique.
fn state_where(re: &Regex, pred: impl Fn(&State) -> bool) -> StateIdx {
    let mut found = (0..re.states.len() as u32)
        .map(StateIdx)
        .filter(|&s| pred(&re.states[s]));
    let state = found.next().expect("a state");
    assert!(found.next().is_none(), "several states");
    state
}

/// The raw counts of every counted row after `input`, by state.
fn raw_rows(re: &Regex, input: &[u8]) -> Vec<(u32, Vec<u32>)> {
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(re, &mut scratch).unwrap();
    s.chunk(re, &mut scratch, input);
    (0..re.states.len() as u32)
        .filter_map(|i| {
            let counts = s.raw_counts(re, StateIdx(i))?;
            (!counts.is_empty()).then(|| (i, counts.to_vec()))
        })
        .collect()
}

/// Counts are active iteration numbers from one; zero never appears, and
/// an entry seeds exactly `{1}`.
#[test]
fn entry_seeds_count_one() {
    let re = sparse_forced("x(a|aa){2,5}z");
    assert!(raw_rows(&re, b"").is_empty());
    assert!(
        raw_rows(&re, b"x").is_empty(),
        "the body is entered by the closure"
    );
    let rows = raw_rows(&re, b"xa");
    assert!(!rows.is_empty());
    for (state, counts) in &rows {
        assert!(counts.iter().all(|&u| u >= 1), "state {state}: {counts:?}");
    }
    // After `xa`: the first iteration is done (`a`), or half done (`aa`).
    let all: Vec<u32> = rows.iter().flat_map(|(_, c)| c.clone()).collect();
    assert!(all.iter().all(|&u| u == 1), "{rows:?}");
}

/// A wide range keeps two representatives however many counts are
/// active; the exact rows hold them all.
#[test]
fn wide_ranges_stay_small() {
    let re = sparse_forced("a(a|aa){1,1000}z");
    let input = vec![b'a'; 1500];
    let rows = raw_rows(&re, &input);
    assert!(!rows.is_empty());
    for (state, counts) in &rows {
        assert!(counts.len() <= 2, "state {state}: {counts:?}");
    }
    let mut scratch = GroupedScratch::new();
    let mut exact = GroupedStream::new(&re, &mut scratch).unwrap();
    exact.chunk(&re, &mut scratch, &input);
    let most = re
        .grouped_plan()
        .unwrap()
        .rows_iter()
        .map(|(_, _, row)| {
            exact.raw()[row.off as usize..(row.off + row.words) as usize]
                .iter()
                .map(|w| w.count_ones())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    assert!(most >= 990, "exact rows hold {most} counts");
}

/// Holes are kept: no count is invented between two representatives.
#[test]
fn holey_sets_keep_their_holes() {
    // `{6,8}`: k = 3, so counts 1, 4, 7 are all needed.
    let re = sparse_forced("x(a|aaaa){6,8}z");
    let rows = raw_rows(&re, b"xaaaaaaa");
    let all: Vec<&Vec<u32>> = rows.iter().map(|(_, c)| c).collect();
    assert!(
        all.iter().any(|c| c.windows(2).any(|w| w[1] - w[0] >= 3)),
        "{rows:?}"
    );
    check_profiles(&re, b"xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaz", "holey");
}

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

#[test]
fn reset_empty_chunks_and_early_success() {
    let re = sparse_forced("x(a|aa){2,5}z");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    // Empty chunks do nothing, also before any input.
    s.chunk(&re, &mut scratch, b"");
    s.chunk(&re, &mut scratch, b"xa");
    s.chunk(&re, &mut scratch, b"");
    s.chunk(&re, &mut scratch, b"az");
    // The match is known before the end.
    assert!(s.ismatch());
    s.chunk(&re, &mut scratch, b"qqqq");
    assert!(s.finish(&re, &mut scratch));

    s.reset(&re, &mut scratch);
    assert!(!s.ismatch());
    s.chunk(&re, &mut scratch, b"xaz");
    assert!(!s.finish(&re, &mut scratch), "one iteration is not enough");

    // A match only `finish` can see, and the empty input.
    let empty = sparse_forced("a{0,3}");
    let mut s = SparseStream::new(&empty, &mut scratch).unwrap();
    assert!(s.ismatch(), "the empty match is known at construction");
    assert!(s.finish(&empty, &mut scratch));
    // An extra byte before the end.
    let re = sparse_forced("x(a|aa){2,3}");
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, b"xa");
    assert!(!s.ismatch());
    s.chunk(&re, &mut scratch, b"a");
    assert!(s.ismatch());
    assert!(s.finish(&re, &mut scratch));
}

#[test]
#[should_panic(expected = "chunk on a finished stream")]
fn chunk_after_finish_panics() {
    let re = sparse_forced("x(a|aa){2,5}z");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, b"xaaz");
    assert!(s.finish(&re, &mut scratch));
    // Also after a match.
    s.chunk(&re, &mut scratch, b"a");
}

#[test]
#[should_panic(expected = "finish on a finished stream")]
fn finish_twice_panics() {
    let re = sparse_forced("x(a|aa){2,5}z");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    s.finish(&re, &mut scratch);
    s.finish(&re, &mut scratch);
}

#[test]
#[should_panic(expected = "sparse stream used with a different regex")]
fn stream_with_another_regex_panics() {
    let (a, b) = (sparse_forced("a{2,5}b"), sparse_forced("a{2,5}b"));
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&a, &mut scratch).unwrap();
    s.chunk(&b, &mut scratch, b"aab");
}

/// Streams of several regexes interleaved on one scratch, byte by byte.
#[test]
fn interleaved_streams_share_scratch() {
    let patterns = [
        "x(a|aa){2,5}z",
        "a(a|aa){1,300}z",
        "x(a|aa|b){100,200}z",
        "(a{2,5}b)*c",
        "abc|abd",
    ];
    let regexes: Vec<Regex> = patterns.iter().map(|p| sparse_forced(p)).collect();
    let oracles: Vec<_> = patterns.iter().map(|p| oracle(p)).collect();
    let mut rng = Rng(0x1234_5678);
    let pieces: &[&[u8]] = &[b"a", b"aa", b"aaaaa", b"b", b"x", b"z", b"c", b"ab"];
    let inputs: Vec<Vec<u8>> = (0..12)
        .map(|_| random_input(&mut rng, pieces, 400))
        .collect();
    let mut scratch = SparseScratch::new();
    // One stream per (regex, input).
    let mut streams: Vec<SparseStream> = Vec::new();
    for re in &regexes {
        for _ in &inputs {
            streams.push(SparseStream::new(re, &mut scratch).unwrap());
        }
    }
    let longest = inputs.iter().map(Vec::len).max().unwrap();
    for at in 0..longest {
        for (r, re) in regexes.iter().enumerate() {
            for (i, input) in inputs.iter().enumerate() {
                if let Some(b) = input.get(at) {
                    streams[r * inputs.len() + i].chunk(re, &mut scratch, &[*b]);
                }
            }
        }
    }
    for (r, re) in regexes.iter().enumerate() {
        for (i, input) in inputs.iter().enumerate() {
            assert_eq!(
                streams[r * inputs.len() + i].finish(re, &mut scratch),
                oracles[r].is_match(input),
                "`{}` input {i}",
                patterns[r]
            );
        }
    }
    // The scratch grew to the largest plan and no further.
    let largest = regexes
        .iter()
        .map(|re| sparse_plan(re).scratch_bytes())
        .max()
        .unwrap();
    assert!(scratch.memory_bytes() >= largest);
    let sum: usize = regexes
        .iter()
        .map(|re| sparse_plan(re).scratch_bytes())
        .sum();
    assert!(scratch.memory_bytes() <= sum);
}

/// Rebinding reuses the stream; a rejected rebind leaves it as it was,
/// and a later one works.
#[test]
fn rebind_and_rejected_rebind() {
    let small = sparse_forced("x(a|aa){2,5}z");
    let large = sparse_forced("x(a|aa|b){900,1000}z");
    let rejected = sparse_forced("x(a|aa){5,}z");
    let unrequested = forced("x(a|aa){2,5}z");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&small, &mut scratch).unwrap();
    assert_eq!(s.memory_bytes(), sparse_plan(&small).stream_bytes());
    s.chunk(&small, &mut scratch, b"xaa");
    let before = format!("{}", s.display(&small));

    assert!(matches!(
        s.rebind(&rejected, &mut scratch),
        Err(SparseCountingError::Rejected(
            SparseRejection::UnboundedCounter { .. }
        ))
    ));
    assert_eq!(
        s.rebind(&unrequested, &mut scratch),
        Err(SparseCountingError::NotRequested)
    );
    // Untouched: the same rows, and the stream goes on.
    assert_eq!(format!("{}", s.display(&small)), before);
    s.chunk(&small, &mut scratch, b"z");
    assert!(s.finish(&small, &mut scratch));

    s.rebind(&large, &mut scratch).unwrap();
    assert_eq!(s.memory_bytes(), sparse_plan(&large).stream_bytes());
    assert!(!s.ismatch());
    let mut input = b"x".to_vec();
    input.extend(std::iter::repeat_n(b'a', 900));
    input.push(b'z');
    s.chunk(&large, &mut scratch, &input);
    assert!(s.finish(&large, &mut scratch));

    // Back to the small plan: the allocation is kept, the state is new.
    s.rebind(&small, &mut scratch).unwrap();
    assert!(s.memory_bytes() >= sparse_plan(&large).stream_bytes());
    s.chunk(&small, &mut scratch, b"xaz");
    assert!(!s.finish(&small, &mut scratch));
    // The same regex again: a reset.
    s.rebind(&small, &mut scratch).unwrap();
    s.chunk(&small, &mut scratch, b"xaaz");
    assert!(s.finish(&small, &mut scratch));
}

/// `MatcherMemory::matcher_sparse`: forced, never a fallback, reusable
/// across regexes and after a rejection.
#[test]
fn matcher_memory_forces_the_sparse_variant() {
    let re = sparse_forced("x(a|aa){2,5}z");
    let other = sparse_forced("a(a|aa){1,300}z");
    let rejected = sparse_forced("x(a|aa){2,}z");
    let mut memory = MatcherMemory::default();
    for _ in 0..2 {
        let mut m = memory.matcher_sparse(&re).unwrap();
        assert!(matches!(m, crate::AnyMatcher::Sparse(_)));
        m.chunk(b"xa");
        assert!(format!("{m}").starts_with("sparse slots="), "{m}");
        m.chunk(b"az");
        assert!(m.finish());
        assert!(matches!(
            memory.matcher_sparse(&rejected),
            Err(SparseCountingError::Rejected(
                SparseRejection::UnboundedCounter { .. }
            ))
        ));
        let mut m = memory.matcher_sparse(&other).unwrap();
        m.chunk(b"aaaa");
        assert!(!m.finish());
    }
    // Automatic selection never picks it.
    assert!(!matches!(memory.matcher(&re), crate::AnyMatcher::Sparse(_)));
    assert_eq!(
        re.sparse_is_match(&mut SparseScratch::new(), b"xaaz"),
        Some(true)
    );
    assert_eq!(
        forced("x(a|aa){2,5}z").sparse_is_match(&mut SparseScratch::new(), b"xaaz"),
        None
    );
}

/// The debug view names representatives of `u`, with the plan's sizes.
#[test]
fn display_shows_representatives() {
    let re = sparse_forced("a(a|aa){1,1000}z");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, &[b'a'; 40]);
    let shown = format!("{}", s.display(&re));
    assert!(shown.contains("u~{"), "{shown}");
    assert!(shown.contains("prev=0x61"), "{shown}");
    let _ = (
        CounterIdx(0),
        state_where(&re, |s| matches!(s, State::Match)),
    );
}

// ---------------------------------------------------------------------------
// The passes and the idle skip
// ---------------------------------------------------------------------------

/// The idle table exists exactly when the start closure is the same at
/// every boundary and cannot match.
#[test]
fn idle_table_only_where_sound() {
    for (pattern, idle) in [
        ("x(a|aa){2,5}z", true),
        ("(a|b){2,5}z", true),
        ("x(a|aa){0,3}z", true),
        // A match in the start closure: the empty input matches.
        ("a{0,3}", false),
        ("", false),
        // An assertion in the start closure: it depends on the boundary.
        (r"\bx(a|aa){2,5}", false),
        (r"(?m:^)x(a|aa){2,5}", false),
        (r"(?:\b|q)x(a|aa){2,5}", false),
        // An assertion further on does not matter.
        (r"x(a|aa){2,5}\b", true),
        (r"x\b(a| ){2,5}", true),
    ] {
        let re = sparse_forced(pattern);
        let plan = sparse_plan(&re);
        assert_eq!(plan.idle.is_some(), idle, "`{pattern}`");
        let Some(starts) = plan.idle.as_deref() else {
            continue;
        };
        // Exactly the first bytes of the pattern.
        let first: Vec<u8> = (0..=255u8).filter(|&b| starts[b as usize]).collect();
        let expected: &[u8] = if pattern.starts_with("(a|b)") {
            b"ab"
        } else {
            b"x"
        };
        assert_eq!(first, expected, "`{pattern}`");
    }
}

/// The passes a scan runs: the automatic choice skips while idle and
/// takes the full sweep on a small plan; the forced passes step every
/// byte.  All give the same answer and the same rows.
#[test]
fn passes_and_idle_skipping() {
    let pattern = r"x(a|aa){2,5}\bz";
    let re = sparse_forced(pattern);
    let oracle = oracle(pattern);
    let mut rng = Rng(0x1d1e);
    let pieces: &[&[u8]] = &[
        b"qqqqqqqqqqqq",
        b"q",
        b"xaa",
        b"xaaaaaaa",
        b"x",
        b"a",
        b"z",
        b" ",
    ];
    for round in 0..40 {
        let input = random_input(&mut rng, pieces, 200 + round * 5);
        let expected = oracle.is_match(&input);
        let mut rows = Vec::new();
        for sweep in SWEEPS {
            let mut scratch = SparseScratch::new();
            scratch.set_sweep(sweep);
            let mut s = SparseStream::new(&re, &mut scratch).unwrap();
            for part in input.chunks(1 + round % 9) {
                s.chunk(&re, &mut scratch, part);
                s.check_invariants(&re, pattern);
            }
            rows.push(format!("{}", s.display(&re)));
            let [full, active, skipped] = scratch.passes;
            match sweep {
                Sweep::Auto => {
                    assert_eq!(active, 0, "a small plan takes the full sweep");
                    assert!(skipped > 0, "round {round}: nothing skipped");
                    assert!(full + skipped <= input.len());
                }
                Sweep::Full => assert_eq!((active, skipped), (0, 0)),
                Sweep::Active => assert_eq!((full, skipped), (0, 0)),
            }
            if !s.ismatch() {
                assert_eq!(full + active + skipped, input.len(), "{sweep:?}");
            }
            assert_eq!(s.finish(&re, &mut scratch), expected, "{sweep:?} {input:?}");
        }
        // The same raw frontier and boundary, however it was reached.
        assert!(
            rows.iter().all(|r| r == &rows[0]),
            "round {round}: {rows:?}"
        );
    }
}

/// A wide plan with few active rows takes the active pass under the
/// automatic choice, and the full sweep when many rows are active.
#[test]
fn automatic_choice_follows_the_active_rows() {
    // Twelve counted alternatives of which an input activates one.
    let pattern: String = (0..12u8)
        .map(|i| {
            let (p, a) = ((b'A' + i) as char, (b'a' + i) as char);
            format!("{p}({a}|{a}{a}){{2,60}}!")
        })
        .collect::<Vec<_>>()
        .join("|");
    let re = sparse_forced(&pattern);
    let plan = sparse_plan(&re);
    assert!(plan.rows() > 64, "{} rows", plan.rows());
    let oracle = oracle(&pattern);
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    let mut input = b"qqqqA".to_vec();
    input.extend(std::iter::repeat_n(b'a', 50));
    s.chunk(&re, &mut scratch, &input);
    let [full, active, skipped] = scratch.passes;
    assert_eq!(skipped, 4);
    assert!(active >= 50 && full == 0, "full={full} active={active}");
    assert!(!s.ismatch());
    s.chunk(&re, &mut scratch, b"!");
    assert!(s.finish(&re, &mut scratch));
    for input in [
        b"Aaaaa!".as_slice(),
        b"BbbbbbbbbbbA!",
        b"qqLllllllllllllllllllllllllllll!",
        b"Aa!",
        b"KkkkkAaaaaaaBbbb!",
    ] {
        run_chunks(&re, &[input], oracle.is_match(input), "wide");
        check_profiles(&re, input, "wide");
    }
}

/// Skipping keeps the boundary: an assertion right after a skipped
/// stretch sees the byte before it, and a chunk that ends while idle
/// leaves the stream at the right place.
#[test]
fn skipping_tracks_the_boundary() {
    let pattern = r"x\b(a| ){2,3}\b";
    let re = sparse_forced(pattern);
    assert!(sparse_plan(&re).idle.is_some());
    let oracle = oracle(pattern);
    let mut scratch = SparseScratch::new();
    for input in [
        b"qqqqqqx a".as_slice(),
        b"qqqqqqxa a",
        b"qqqqqq",
        b"qqqqqqqqqqqqx  a ",
        b"qqx",
        b"x aqqqqqqx  ",
    ] {
        for cut in 0..=input.len() {
            let mut s = SparseStream::new(&re, &mut scratch).unwrap();
            s.chunk(&re, &mut scratch, &input[..cut]);
            let prev = cut.checked_sub(1).map(|i| input[i]);
            if !s.ismatch() {
                assert_eq!(s.boundary_state(), (prev, cut == 0), "{input:?} cut {cut}");
            }
            s.chunk(&re, &mut scratch, &input[cut..]);
            assert_eq!(
                s.finish(&re, &mut scratch),
                oracle.is_match(input),
                "{input:?} cut {cut}"
            );
        }
    }
}
