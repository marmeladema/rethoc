//! Tests of the grouped NFA: admission structure and budgets, transfers
//! against `counter_step`, configurations against an independent scalar
//! interpreter, and match results against Tier 0 and the `regex` crate.

use std::collections::BTreeSet;

use super::analysis::{self, Limits, Rejection, Scope};
use super::exact::{self, Boundary, BoundaryCtx};
use super::{
    GroupedNfaError, GroupedNfaPlan, GroupedScratch, GroupedStream, Increment, Row, Sweep,
};
use crate::{
    AssertEval, ByteMap, CounterIdx, MatcherMemory, Regex, RegexConfig, State, StateIdx,
    byte_match_ci, counter_step,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The three compile configurations of the test suite: default unrolling,
/// unrolling off (counters forced), unrolling and merging off.
pub(crate) const CONFIGS: [(usize, bool); 3] = [
    (crate::DEFAULT_MAX_UNROLL_STATES, true),
    (0, true),
    (0, false),
];

pub(crate) fn compile(pattern: &str, unroll: usize, merge: bool) -> Regex {
    Regex::with_config(
        pattern,
        RegexConfig {
            max_unroll_states: unroll,
            merge_repetitions: merge,
            grouped_nfa: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| panic!("`{pattern}` should compile: {e}"))
}

/// Counters forced: no unrolling, no merging.
pub(crate) fn forced(pattern: &str) -> Regex {
    compile(pattern, 0, false)
}

pub(crate) fn plan(re: &Regex) -> &GroupedNfaPlan {
    re.grouped_plan()
        .unwrap_or_else(|e| panic!("expected admission: {e}"))
}

pub(crate) fn rejection(re: &Regex) -> Rejection {
    match re.grouped_plan() {
        Err(GroupedNfaError::Rejected(r)) => r,
        other => panic!("expected a rejection, got {other:?}"),
    }
}

fn st(i: u32) -> StateIdx {
    StateIdx(i)
}

fn c(i: u8) -> CounterIdx {
    CounterIdx(i)
}

/// Analyse a hand-built graph starting at state 0.
fn analyze_fixture(
    states: &[State],
    num_counters: usize,
    limits: &Limits,
) -> Result<GroupedNfaPlan, Rejection> {
    analysis::analyze(states, &[], st(0), num_counters, &[], limits)
}

fn byte(b: u8, out: u32) -> State {
    State::Byte {
        byte: b,
        out: st(out),
        out_exit: StateIdx::NONE,
    }
}

fn cinc(counter: u8, out: u32, out1: u32, min: usize, max: usize) -> State {
    State::CounterIncrement {
        counter: c(counter),
        out: st(out),
        out1: st(out1),
        min,
        max,
    }
}

/// A small deterministic PRNG (xorshift64*).
pub(crate) struct Rng(pub(crate) u64);

impl Rng {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    pub(crate) fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

// ---------------------------------------------------------------------------
// Admission: patterns
// ---------------------------------------------------------------------------

/// The scope of each state with a row in `plan`.
fn scope_of(plan: &GroupedNfaPlan, state: StateIdx) -> Option<Scope> {
    plan.rows_iter()
        .find(|&(s, _, _)| s == state)
        .map(|(_, scope, _)| scope)
}

#[test]
fn admits_flat_counter_shapes() {
    for pattern in [
        "x(a|aa){2,5}z",
        "(a|aa){2}b(a|aa){2}",
        "(a{2,3}b)*c",
        "a{0,2}b",
        "x(ab*){2,5}z",
        "x(ab|cd|ef){2,5}z",
        "a{63}b",
        "(a{2,4}|b{3,5})c",
        "a{2,4}b{3,5}",
    ] {
        let re = forced(pattern);
        let plan = plan(&re);
        assert!(re.num_counters > 0, "`{pattern}` should use counters");
        // Every counter instance's successor is in that counter's scope.
        for (i, s) in re.states.iter().enumerate() {
            if let State::CounterInstance { counter, out } = *s
                && let Some(scope) = scope_of(plan, st(i as u32))
            {
                assert_eq!(scope, Scope::Outside, "`{pattern}`: CI {i}");
                assert_eq!(scope_of(plan, out), Some(Scope::Counter(counter)));
            }
            if let State::CounterIncrement { out1, .. } = *s
                && scope_of(plan, st(i as u32)).is_some()
            {
                assert_eq!(scope_of(plan, out1), Some(Scope::Outside), "`{pattern}`");
            }
        }
    }
}

#[test]
fn default_unrolling_and_counter_free_graphs_are_admitted() {
    // Unrolled: consuming states with fused out_exit edges.
    let re = compile("a{2,5}b", crate::DEFAULT_MAX_UNROLL_STATES, true);
    assert_eq!(re.num_counters, 0);
    assert!(
        re.states
            .iter()
            .any(|s| matches!(s, State::Byte { out_exit, .. } if *out_exit != StateIdx::NONE))
    );
    plan(&re);
    plan(&compile("abc|abd", 32, true));
    plan(&compile("", 32, true));
}

#[test]
fn rejects_outside_the_admitted_class() {
    assert!(matches!(
        rejection(&forced("x((a|aa){2,3}){2,3}z")),
        Rejection::NestedCounter { .. }
    ));
    assert!(matches!(
        rejection(&forced("x(a?){2,5}z")),
        Rejection::ZeroWidthCycle { .. }
    ));
    // The state named is on the cycle (Split 1 -> CInc 2 -> Split 1), not
    // the consuming state 0 that the Split feeds.
    let re = compile("(?:a?){2,3}b", 0, true);
    let Rejection::ZeroWidthCycle { state } = rejection(&re) else {
        panic!("expected a zero-width cycle")
    };
    assert!(
        matches!(re.states[st(state)], State::Split { .. }),
        "state {state}: {:?}",
        re.states[st(state)]
    );
    // A zero-width cycle is reported before the assertion exclusion.
    // (`(?:\b){2,3}` itself compiles to a lone `\b`, without a counter.)
    assert!(matches!(
        rejection(&forced(r"(?:\b|a){2,3}")),
        Rejection::ZeroWidthCycle { .. }
    ));
}

#[test]
fn every_assertion_kind_is_reviewed() {
    for &kind in crate::AssertKind::ALL {
        assert!(analysis::reviewed(kind), "{kind:?}");
    }
    plan(&forced(r"\Ax(a|aa){2}z\z"));
}

/// The row of a counted scope: `0..=max` (finite) or `0..=min`
/// (saturating) in `top / 64 + 1` words.
#[test]
fn counted_rows_span_words() {
    for (pattern, words) in [
        ("a{63}b", 1),
        ("a{64}b", 2),
        ("a{65}b", 2),
        ("a{127}b", 2),
        ("a{128}b", 3),
        ("a{129}b", 3),
        ("a{2,}b", 1),
        ("a{63,}b", 1),
        ("a{64,}b", 2),
        ("a{129,}b", 3),
    ] {
        let re = forced(pattern);
        let plan = plan(&re);
        let widths: BTreeSet<u32> = plan
            .rows_iter()
            .filter(|(_, scope, _)| matches!(scope, Scope::Counter(_)))
            .map(|(_, _, row)| row.words)
            .collect();
        assert_eq!(widths, BTreeSet::from([words]), "`{pattern}`");
    }
    // A counter is admitted as long as its rows fit the frontier budget.
    let re = forced("a{1000}b");
    let n = plan(&re).words();
    let narrow = Limits {
        max_row_words: n - 1,
        ..Limits::DEFAULT
    };
    assert_eq!(
        analysis::analyze(
            &re.states,
            &re.byte_tables,
            re.start,
            1,
            &re.counter_info,
            &narrow
        )
        .err(),
        Some(Rejection::TooManyRowWords {
            words: n,
            limit: n - 1
        })
    );
}

#[test]
fn not_requested_is_reported() {
    let re = Regex::new("x(a|aa){2,5}z").unwrap();
    assert!(re.grouped_nfa.is_none());
    assert_eq!(
        re.grouped_nfa_admission(),
        Err(GroupedNfaError::NotRequested)
    );
    let mut scratch = GroupedScratch::new();
    assert_eq!(
        GroupedStream::new(&re, &mut scratch).err(),
        Some(GroupedNfaError::NotRequested)
    );
    let rejected = forced("x((a|aa){2,3}){2,3}z");
    assert!(matches!(
        GroupedStream::new(&rejected, &mut scratch).err(),
        Some(GroupedNfaError::Rejected(Rejection::NestedCounter { .. }))
    ));
}

/// The option adds a plan; it does not change the compiled NFA or the
/// tier selection.
#[test]
fn option_leaves_the_nfa_unchanged() {
    for pattern in ["x(a|aa){2,5}z", "a{2,}b", r"\b(a|aa){2,5}\b", "(a{2,3}b)*c"] {
        for (unroll, merge) in CONFIGS {
            let with = compile(pattern, unroll, merge);
            let without = Regex::with_config(
                pattern,
                RegexConfig {
                    max_unroll_states: unroll,
                    merge_repetitions: merge,
                    ..Default::default()
                },
            )
            .unwrap();
            assert_eq!(
                format!("{:?}", with.states),
                format!("{:?}", without.states)
            );
            assert_eq!(with.min_tier(), without.min_tier());
            assert_eq!(
                with.memory_size(),
                without.memory_size() + with.grouped_nfa_bytes(),
                "`{pattern}`"
            );
            assert!(with.grouped_nfa_bytes() > 0);
            assert_eq!(with.info().memory.grouped_nfa, with.grouped_nfa_bytes());
        }
    }
}

#[test]
fn info_and_dump_report_the_plan() {
    let re = forced("x(a|aa){2,5}z");
    let info = re.info();
    let Some(crate::GroupedNfaInfo::Admitted {
        plan_bytes,
        stream_bytes,
        scratch_bytes,
        ..
    }) = info.grouped_nfa
    else {
        panic!("expected admission: {:?}", info.grouped_nfa)
    };
    assert_eq!(plan_bytes, plan(&re).heap_bytes());
    // The sizes reported are those a stream and a scratch hold.
    let mut scratch = GroupedScratch::new();
    let stream = GroupedStream::new(&re, &mut scratch).unwrap();
    assert_eq!(stream_bytes, stream.memory_bytes());
    assert_eq!(scratch_bytes, scratch.memory_bytes());
    assert!(
        info.to_string()
            .contains("Grouped NFA (experimental): admitted")
    );
    assert!(re.dump(false).to_string().contains("Grouped NFA: admitted"));

    let re = forced("x((a|aa){2,3}){2,3}z");
    let info = re.info();
    let Some(crate::GroupedNfaInfo::Rejected { reason }) = &info.grouped_nfa else {
        panic!("expected a rejection: {:?}", info.grouped_nfa)
    };
    assert!(reason.contains("nested"), "{reason}");
    assert!(re.dump(false).to_string().contains("Grouped NFA: rejected"));

    assert!(Regex::new("a").unwrap().info().grouped_nfa.is_none());
}

// ---------------------------------------------------------------------------
// Admission: hand-built graphs
// ---------------------------------------------------------------------------

#[test]
fn fixture_conflicting_scope() {
    // State 3 is the body of c0 (through CI 1) and also reachable outside
    // (through byte 2).  The CI branch is explored first, so the second
    // label is the conflict rather than an increment outside its scope.
    let states = [
        State::Split {
            out: st(2),
            out1: st(1),
        },
        State::CounterInstance {
            counter: c(0),
            out: st(3),
        },
        byte(b'a', 3),
        byte(b'b', 4),
        cinc(0, 3, 5, 1, 2),
        State::Match,
    ];
    assert_eq!(
        analyze_fixture(&states, 1, &Limits::DEFAULT).err(),
        Some(Rejection::ConflictingScope { state: 3 })
    );
}

#[test]
fn fixture_scope_errors() {
    // Increment of another counter.
    let states = [
        State::CounterInstance {
            counter: c(0),
            out: st(1),
        },
        byte(b'a', 2),
        cinc(1, 1, 3, 1, 2),
        State::Match,
    ];
    assert_eq!(
        analyze_fixture(&states, 2, &Limits::DEFAULT).err(),
        Some(Rejection::IncrementScopeMismatch { state: 2 })
    );
    // Match inside a scope.
    let states = [
        State::CounterInstance {
            counter: c(0),
            out: st(1),
        },
        State::Match,
    ];
    assert_eq!(
        analyze_fixture(&states, 1, &Limits::DEFAULT).err(),
        Some(Rejection::MatchInScope { state: 1 })
    );
    // An instance inside a scope.
    let states = [
        State::CounterInstance {
            counter: c(0),
            out: st(1),
        },
        State::CounterInstance {
            counter: c(1),
            out: st(2),
        },
        byte(b'a', 3),
        cinc(1, 2, 4, 1, 2),
        cinc(0, 1, 5, 1, 2),
        State::Match,
    ];
    assert_eq!(
        analyze_fixture(&states, 2, &Limits::DEFAULT).err(),
        Some(Rejection::NestedCounter { state: 1 })
    );
}

#[test]
fn fixture_bounds() {
    // Two increments of c0 with different bounds.
    let states = [
        State::Split {
            out: st(1),
            out1: st(4),
        },
        State::CounterInstance {
            counter: c(0),
            out: st(2),
        },
        byte(b'a', 3),
        cinc(0, 2, 7, 1, 2),
        State::CounterInstance {
            counter: c(0),
            out: st(5),
        },
        byte(b'b', 6),
        cinc(0, 5, 7, 1, 3),
        State::Match,
    ];
    assert_eq!(
        analyze_fixture(&states, 1, &Limits::DEFAULT).err(),
        Some(Rejection::CounterBoundsMismatch { counter: 0 })
    );
    for (min, max) in [(0, 0), (3, 2)] {
        let states = [
            State::CounterInstance {
                counter: c(0),
                out: st(1),
            },
            byte(b'a', 2),
            cinc(0, 1, 3, min, max),
            State::Match,
        ];
        assert_eq!(
            analyze_fixture(&states, 1, &Limits::DEFAULT).err(),
            Some(Rejection::UnsupportedBounds {
                counter: 0,
                min,
                max
            })
        );
    }
}

#[test]
fn fixture_zero_width_graph() {
    // A split cycle without a counter.
    let states = [
        State::Split {
            out: st(1),
            out1: st(2),
        },
        State::Split {
            out: st(0),
            out1: st(2),
        },
        State::Match,
    ];
    assert_eq!(
        analyze_fixture(&states, 0, &Limits::DEFAULT).err(),
        Some(Rejection::ZeroWidthCycle { state: 0 })
    );
    // Duplicate edges are fine, and an unreachable remnant (here one that
    // would conflict) is ignored.
    let states = [
        State::Split {
            out: st(1),
            out1: st(1),
        },
        byte(b'a', 2),
        State::Match,
        State::CounterInstance {
            counter: c(0),
            out: st(2),
        },
    ];
    let plan = analyze_fixture(&states, 1, &Limits::DEFAULT).unwrap();
    assert_eq!(plan.rows(), 3);
    assert_eq!(plan.row_of_state(st(3)), None);
}

#[test]
fn budgets_below_at_and_above() {
    let re = forced("x(a|aa){2,5}z(b|bb){1,3}");
    let states: &[State] = &re.states;
    let tables: &[ByteMap] = &re.byte_tables;
    let n_counters = re.num_counters;
    let run = |limits: Limits| {
        analysis::analyze(
            states,
            tables,
            re.start,
            n_counters,
            &re.counter_info,
            &limits,
        )
    };
    let full = run(Limits::DEFAULT).unwrap();
    let (words, bytes) = (full.words(), full.heap_bytes());
    let n = states.len();
    let analysis = analysis::analysis_bytes(n, n_counters);
    let edges: usize = states
        .iter()
        .map(|s| match s {
            State::ByteTable { .. } => 256,
            State::Match => 0,
            _ => 2,
        })
        .sum();
    // Every state of a fresh forced compile is reachable except the dead
    // sentinels that the byte-table pass leaves; count the reachable ones.
    let reachable_edges: usize = full
        .rows_iter()
        .map(|(s, _, _)| match states[s] {
            State::ByteTable { .. } => 256,
            State::Match => 0,
            _ => 2,
        })
        .sum();
    assert!(reachable_edges <= edges);

    let with = |f: &dyn Fn(&mut Limits)| {
        let mut l = Limits::DEFAULT;
        f(&mut l);
        run(l)
    };
    assert!(with(&|l| l.max_states = n).is_ok());
    assert_eq!(
        with(&|l| l.max_states = n - 1).err(),
        Some(Rejection::TooManyStates {
            states: n,
            limit: n - 1
        })
    );
    assert!(with(&|l| l.max_edges = reachable_edges).is_ok());
    assert_eq!(
        with(&|l| l.max_edges = reachable_edges - 1).err(),
        Some(Rejection::TooManyEdges {
            limit: reachable_edges - 1
        })
    );
    assert!(with(&|l| l.max_row_words = words).is_ok());
    assert_eq!(
        with(&|l| l.max_row_words = words - 1).err(),
        Some(Rejection::TooManyRowWords {
            words,
            limit: words - 1
        })
    );
    assert!(with(&|l| l.max_plan_bytes = bytes).is_ok());
    assert_eq!(
        with(&|l| l.max_plan_bytes = bytes - 1).err(),
        Some(Rejection::PlanTooLarge {
            bytes,
            limit: bytes - 1
        })
    );
    assert!(with(&|l| l.max_analysis_bytes = analysis).is_ok());
    assert_eq!(
        with(&|l| l.max_analysis_bytes = analysis - 1).err(),
        Some(Rejection::AnalysisTooLarge {
            bytes: analysis,
            limit: analysis - 1
        })
    );
}

/// A plan-budget rejection leaves normal compilation and dispatch usable.
#[test]
fn rejection_keeps_regex_usable() {
    let re = forced("x((a|aa){2,3}){2,3}z");
    assert!(re.grouped_plan().is_err());
    let mut mem = MatcherMemory::default();
    let mut m = mem.matcher(&re);
    m.chunk(b"qxaaaaaaz");
    assert!(m.finish());
}

// ---------------------------------------------------------------------------
// Transfers
// ---------------------------------------------------------------------------

/// Run the grouped increment on the count set `counts`, with `prefill`
/// already in the continue row, and compare with `counter_step` applied
/// to each count.
fn check_increment(min: usize, max: usize, counts: &BTreeSet<usize>, prefill: &BTreeSet<usize>) {
    let inc = Increment::new(min, max);
    let w = inc.words();
    let src = Row {
        off: 0,
        words: w as u32,
    };
    let out = Row {
        off: w as u32,
        words: w as u32,
    };
    let mut rows = vec![0u64; 2 * w];
    for &v in counts {
        rows[v / 64] |= 1 << (v % 64);
    }
    for &v in prefill {
        rows[w + v / 64] |= 1 << (v % 64);
    }
    let before = rows[..w].to_vec();
    let (any_cont, brk) = exact::increment(&mut rows, src, out, inc);

    let mut cont = prefill.clone();
    let (mut expected_cont, mut expected_brk) = (false, false);
    for &v in counts {
        let nv = counter_step(v, min, max);
        if nv < max {
            cont.insert(nv);
            expected_cont = true;
        }
        expected_brk |= nv >= min;
    }
    let got: BTreeSet<usize> = (0..64 * w)
        .filter(|&v| rows[w + v / 64] >> (v % 64) & 1 != 0)
        .collect();
    let label = format!("{inc} S={counts:?}");
    assert_eq!(got, cont, "{label}: continue (padding included)");
    assert_eq!(brk, expected_brk, "{label}: break");
    assert_eq!(any_cont, expected_cont, "{label}: anything continued");
    assert_eq!(rows[..w], before[..], "{label}: source row changed");
}

/// The members of `bits` as a set.
fn set_of(bits: u64) -> BTreeSet<usize> {
    (0..64).filter(|&v| bits >> v & 1 != 0).collect()
}

#[test]
fn finite_increment_matches_counter_step_exhaustively() {
    for max in 1..=10usize {
        for min in 0..=max {
            // Every subset of the incoming domain 0..max.
            for s in 0..(1u64 << max) {
                check_increment(min, max, &set_of(s), &BTreeSet::new());
            }
        }
    }
}

#[test]
fn saturating_increment_matches_counter_step_exhaustively() {
    for min in 0..=9usize {
        // Every subset of the domain 0..=min.
        for s in 0..(1u64 << (min + 1)) {
            check_increment(min, usize::MAX, &set_of(s), &BTreeSet::new());
        }
    }
}

/// A random subset of `0..n`, dense or sparse, plus the edge counts.
fn random_sets(rng: &mut Rng, n: usize) -> Vec<BTreeSet<usize>> {
    let mut sets = vec![
        BTreeSet::new(),
        (0..n).collect(),
        BTreeSet::from([0]),
        BTreeSet::from([n - 1]),
    ];
    for v in [62, 63, 64, 65, 127, 128] {
        if v < n {
            sets.push(BTreeSet::from([v]));
            sets.push((v.saturating_sub(1)..=v).collect());
        }
    }
    for k in 0..200 {
        let density = [2, 8, 64][k % 3];
        sets.push((0..n).filter(|_| rng.below(density) == 0).collect());
    }
    sets
}

/// Multiword rows: carries across words, the last word's padding, and
/// break ranges that straddle words.
#[test]
fn increment_matches_counter_step_across_words() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for max in [31usize, 63, 64, 65, 127, 128, 129, 200] {
        for min in [0, 1, 63, 64, max / 2, max - 1, max] {
            if min > max {
                continue;
            }
            for set in random_sets(&mut rng, max) {
                let density = [2, 8, 64][rng.below(3)];
                let prefill: BTreeSet<usize> =
                    (0..max).filter(|_| rng.below(density) == 0).collect();
                check_increment(min, max, &set, &prefill);
            }
        }
    }
    for min in [62usize, 63, 64, 65, 127, 128, 129] {
        for set in random_sets(&mut rng, min + 1) {
            check_increment(min, usize::MAX, &set, &BTreeSet::new());
        }
    }
}

#[test]
#[should_panic(expected = "reached its maximum")]
fn increment_of_the_maximum_is_impossible() {
    check_increment(2, 65, &BTreeSet::from([65]), &BTreeSet::new());
}

// ---------------------------------------------------------------------------
// Independent scalar interpreter
// ---------------------------------------------------------------------------

/// A configuration: a state and its count (`None` outside any counter).
type Config = (StateIdx, Option<usize>);

/// Scalar zero-width closure of `roots` plus the start state at boundary
/// `ctx`, by a plain visited-set search; an assertion is followed on
/// `Pass` of the shared evaluator.  Returns every visited configuration and
/// whether `Match` was reached.
fn scalar_close(
    re: &Regex,
    roots: &BTreeSet<Config>,
    ctx: BoundaryCtx,
) -> (BTreeSet<Config>, bool) {
    let (at_end, next) = match ctx.boundary {
        Boundary::UnknownNext => (false, None),
        Boundary::BeforeByte(b) => (false, Some(b)),
        Boundary::EndOfInput => (true, None),
    };
    let mut seen = BTreeSet::new();
    let mut matched = false;
    let mut stack: Vec<Config> = roots.iter().copied().collect();
    stack.push((re.start, None));
    while let Some(cfg) = stack.pop() {
        if !seen.insert(cfg) {
            continue;
        }
        let (s, count) = cfg;
        match re.states[s] {
            State::Split { out, out1 } => {
                stack.push((out, count));
                stack.push((out1, count));
            }
            State::CounterInstance { out, .. } => {
                assert_eq!(count, None, "flat graphs only");
                stack.push((out, Some(0)));
            }
            State::CounterIncrement {
                out,
                out1,
                min,
                max,
                ..
            } => {
                let v = count.expect("active at its increment");
                let nv = counter_step(v, min, max);
                if nv < max {
                    stack.push((out, Some(nv)));
                }
                if nv >= min {
                    stack.push((out1, None));
                }
            }
            State::Assert { kind, out } => {
                if kind.eval(ctx.at_start, at_end, ctx.prev, next) == AssertEval::Pass {
                    stack.push((out, count));
                }
            }
            State::Match => matched = true,
            _ => {}
        }
    }
    (seen, matched)
}

/// Scalar consumption of `b` from a closure.
fn scalar_consume(re: &Regex, closure: &BTreeSet<Config>, b: u8) -> BTreeSet<Config> {
    let mut next = BTreeSet::new();
    for &(s, count) in closure {
        let (out, out_exit) = match re.states[s] {
            State::Byte {
                byte,
                out,
                out_exit,
            } if byte == b => (out, out_exit),
            State::ByteCI {
                byte,
                out,
                out_exit,
            } if byte_match_ci(b, byte) => (out, out_exit),
            State::Wildcard { out, out_exit } => (out, out_exit),
            State::ByteClassStatic {
                table,
                out,
                out_exit,
            } if table[b as usize] => (out, out_exit),
            State::ByteClassCustom {
                class,
                out,
                out_exit,
            } if re.classes[class.idx()].contains(b) => (out, out_exit),
            State::ByteTable { table } => (re.byte_tables[table][b], StateIdx::NONE),
            _ => continue,
        };
        for t in [out, out_exit] {
            if t != StateIdx::NONE {
                next.insert((t, count));
            }
        }
    }
    next
}

/// Check the active-row invariant: a node's bit is set iff its row is
/// nonempty.
fn check_active(plan: &GroupedNfaPlan, rows: &[u64], active: &[u64], label: &str) {
    for (n, (state, _, row)) in plan.rows_iter().enumerate() {
        let bit = active[n / 64] >> (n % 64) & 1 != 0;
        let nonempty = rows[row.range()].iter().any(|&w| w != 0);
        assert_eq!(bit, nonempty, "{label}: active bit of state {state}");
    }
}

/// Expand grouped rows into configurations.
fn expand(plan: &GroupedNfaPlan, rows: &[u64]) -> BTreeSet<Config> {
    let mut out = BTreeSet::new();
    for (state, scope, row) in plan.rows_iter() {
        for v in 0..row.words as usize * 64 {
            if rows[row.off as usize + v / 64] >> (v % 64) & 1 == 0 {
                continue;
            }
            match scope {
                Scope::Outside => {
                    assert_eq!(v, 0, "an Outside row uses bit 0 only");
                    out.insert((state, None));
                }
                Scope::Counter(_) => {
                    out.insert((state, Some(v)));
                }
            }
        }
    }
    out
}

/// Run `input` byte by byte on the grouped stream and the scalar
/// interpreter; compare raw roots, closures before each byte, and the
/// match flag after construction, after each byte and at the end.
pub(crate) fn check_against_scalar(re: &Regex, input: &[u8], label: &str) {
    let plan = plan(re);
    // One stream per pass, in lockstep with one scalar run.
    let mut streams: Vec<(Sweep, GroupedScratch, GroupedStream)> = SWEEPS
        .iter()
        .map(|&sweep| {
            let mut scratch = GroupedScratch::new();
            scratch.set_sweep(sweep);
            let stream = GroupedStream::new(re, &mut scratch).unwrap();
            (sweep, scratch, stream)
        })
        .collect();
    let mut raw: BTreeSet<Config> = BTreeSet::new();
    let ctx = |i: usize, boundary| BoundaryCtx {
        at_start: i == 0,
        prev: i.checked_sub(1).map(|j| input[j]),
        boundary,
    };
    let (_, mut matched) = scalar_close(re, &raw, ctx(0, Boundary::UnknownNext));
    for (sweep, _, stream) in &streams {
        assert_eq!(
            stream.ismatch(),
            matched,
            "{label} {sweep:?}: ismatch at start"
        );
    }
    for (i, &b) in input.iter().enumerate() {
        let (closure, m) = scalar_close(re, &raw, ctx(i, Boundary::BeforeByte(b)));
        // Both closure passes, from the first stream's raw rows.
        let stream = &streams[0].2;
        assert_eq!(expand(plan, stream.raw()), raw, "{label}: raw roots at {i}");
        check_active(plan, stream.raw(), stream.raw_active(), label);
        let mut rows = vec![0u64; plan.words()];
        let mut active = vec![0u64; plan.active_words()];
        let c = ctx(i, Boundary::BeforeByte(b));
        let gm = exact::close(
            plan,
            stream.raw(),
            stream.raw_active(),
            &mut rows,
            &mut active,
            c,
        );
        assert_eq!(
            expand(plan, &rows),
            closure,
            "{label}: active closure at {i}"
        );
        check_active(plan, &rows, &active, label);
        assert_eq!(gm, m, "{label}: active closure match at {i}");
        let mut rows = vec![0u64; plan.words()];
        let gm = exact::close_full(plan, stream.raw(), &mut rows, c);
        assert_eq!(expand(plan, &rows), closure, "{label}: full closure at {i}");
        assert_eq!(gm, m, "{label}: full closure match at {i}");

        let done = matched || m;
        if !done {
            raw = scalar_consume(re, &closure, b);
            let (_, m) = scalar_close(re, &raw, ctx(i + 1, Boundary::UnknownNext));
            matched |= m;
        }
        for (sweep, scratch, stream) in &mut streams {
            stream.chunk(re, scratch, &[b]);
            assert!(
                scratch.is_clean(),
                "{label} {sweep:?}: scratch left dirty at {i}"
            );
            if done {
                assert!(stream.ismatch(), "{label} {sweep:?}: ismatch at {i}");
                assert!(stream.finish(re, scratch), "{label} {sweep:?}: finish");
            } else {
                assert_eq!(
                    expand(plan, stream.raw()),
                    raw,
                    "{label} {sweep:?}: raw after {i}"
                );
                check_active(plan, stream.raw(), stream.raw_active(), label);
                assert_eq!(
                    stream.ismatch(),
                    matched,
                    "{label} {sweep:?}: ismatch after {i}"
                );
            }
        }
        if done {
            return;
        }
    }
    let (_, m) = scalar_close(re, &raw, ctx(input.len(), Boundary::EndOfInput));
    for (sweep, scratch, stream) in &mut streams {
        assert_eq!(
            stream.finish(re, scratch),
            matched || m,
            "{label} {sweep:?}: finish"
        );
    }
}

/// Every string over `alphabet` up to `max_len`.
pub(crate) fn all_strings(alphabet: &[u8], max_len: usize) -> Vec<Vec<u8>> {
    let mut out = vec![Vec::new()];
    let mut layer = vec![Vec::new()];
    for _ in 0..max_len {
        let mut next = Vec::new();
        for s in &layer {
            for &b in alphabet {
                let mut t: Vec<u8> = s.clone();
                t.push(b);
                next.push(t);
            }
        }
        out.extend(next.iter().cloned());
        layer = next;
    }
    out
}

/// Patterns admitted by P1 under at least one configuration, with an
/// alphabet that reaches their interesting configurations.
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
    ("a{2,}b", b"ab"),
    ("x(a|aa){2,}z", b"xaz"),
    ("(ab){3,}c", b"abc"),
    ("a{3,}|b{2,}", b"ab"),
    ("abc|abd", b"abcd"),
    (r"\Ax(a|aaa){3}z\z", b"xaz"),
    (r"\A(a|aa){2}b(a|aa){2}\z", b"ab"),
    (r"\Ax(a|aa){2}\z", b"xa"),
    (r"\b(a|aa){2,5}\b", b"a -"),
    (r"(?mR:^)(a|aa){2,5}(?mR:$)", b"a\r\n"),
    (r"(?m:^)a{2,3}(?m:$)", b"a\n"),
    (r"x(a\B|b){1,3}y", b"xaby"),
    (r"\b{start}(ab){1,3}\b{end}", b"ab "),
    ("", b"a"),
];

#[test]
fn configurations_match_scalar_interpreter() {
    for &(pattern, alphabet) in PATTERNS {
        for (unroll, merge) in CONFIGS {
            let re = compile(pattern, unroll, merge);
            if re.grouped_plan().is_err() {
                continue;
            }
            let label = format!("`{pattern}` unroll={unroll} merge={merge}");
            // Up to length 6, at most about 1,000 strings per alphabet.
            let len = (1..=6)
                .take_while(|&l| alphabet.len().pow(l as u32) <= 1024)
                .last()
                .unwrap_or(1);
            for input in all_strings(alphabet, len) {
                check_against_scalar(&re, &input, &format!("{label} on {input:?}"));
            }
        }
    }
}

#[test]
fn configurations_match_scalar_interpreter_on_long_inputs() {
    let mut rng = Rng(42);
    for pattern in [
        "a{63}b",
        "a{64}b",
        "x(a|aa){2,5}z",
        "a(a|aa){1,40}b",
        "a(a|aa){1,130}b",
        "x(a|aa){60,70}z",
        "(a{2,3}b)*c",
        "a{2,}b",
        "(a|aa){65,}b",
    ] {
        let re = forced(pattern);
        for _ in 0..30 {
            let len = 40 + rng.below(100);
            let input: Vec<u8> = (0..len)
                .map(|_| match rng.below(20) {
                    0 => b'b',
                    1 => b'x',
                    2 => b'z',
                    3 => b'c',
                    _ => b'a',
                })
                .collect();
            check_against_scalar(&re, &input, &format!("`{pattern}` on {input:?}"));
        }
    }
}

// ---------------------------------------------------------------------------
// Match results against Tier 0 and the regex crate
// ---------------------------------------------------------------------------

pub(crate) fn oracle(pattern: &str) -> regex::bytes::Regex {
    regex::bytes::Regex::new(&format!("(?s-u){pattern}")).unwrap()
}

/// Feed `chunks` to the grouped stream and to Tier 0 side by side;
/// `ismatch` must agree after construction and every call, `finish` must
/// agree with both.
pub(crate) fn run_chunks(re: &Regex, chunks: &[&[u8]], expected: bool, label: &str) {
    // One stream per pass, each with its own scratch.
    let mut streams: Vec<(Sweep, GroupedScratch, GroupedStream)> = SWEEPS
        .iter()
        .map(|&sweep| {
            let mut scratch = GroupedScratch::new();
            scratch.set_sweep(sweep);
            let stream = GroupedStream::new(re, &mut scratch).unwrap();
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
        }
    }
    assert_eq!(nfa.finish(), expected, "{label}: tier 0");
    for (sweep, scratch, stream) in &mut streams {
        assert_eq!(
            stream.finish(re, scratch),
            expected,
            "{label} {sweep:?}: grouped"
        );
        assert!(scratch.is_clean(), "{label} {sweep:?}: scratch left dirty");
    }
}

/// Every pass a stream can be forced to.
const SWEEPS: [Sweep; 3] = [Sweep::Auto, Sweep::Full, Sweep::Active];

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
        let n = 1 + rng.below(4);
        let end = (i + n).min(input.len());
        parts.push(&input[i..end]);
        if rng.below(3) == 0 {
            parts.push(&[]);
        }
        i = end;
    }
    run_chunks(re, &parts, expected, &format!("{label} chunked {input:?}"));
}

#[test]
fn results_match_tier0_and_regex_crate() {
    for &(pattern, alphabet) in PATTERNS {
        let oracle = oracle(pattern);
        for (unroll, merge) in CONFIGS {
            let re = compile(pattern, unroll, merge);
            if re.grouped_plan().is_err() {
                continue;
            }
            let label = format!("`{pattern}` unroll={unroll} merge={merge}");
            for (k, input) in all_strings(alphabet, 5).iter().enumerate() {
                check_input(&re, &oracle, input, k as u64, &label);
            }
        }
    }
}

/// Feed `input` whole and bytewise to the grouped stream only.
fn check_grouped_only(re: &Regex, expected: bool, input: &[u8], label: &str) {
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(re, &mut scratch).unwrap();
    s.chunk(re, &mut scratch, input);
    assert_eq!(
        s.finish(re, &mut scratch),
        expected,
        "{label} whole {input:?}"
    );
    s.reset(re, &mut scratch);
    for b in input.chunks(1) {
        s.chunk(re, &mut scratch, b);
    }
    assert_eq!(
        s.finish(re, &mut scratch),
        expected,
        "{label} bytewise {input:?}"
    );
}

/// The grouped part of a `match_tests!` entry, for one of its builds:
/// compile the same configuration with the grouped option; when admitted,
/// check that the NFA is the one the other tiers ran on, then run the
/// listed inputs (against Tier 0 and the oracle, with chunk splits) and
/// the generated ones (against the oracle).  Returns whether the build was
/// admitted.
pub(crate) fn check_match_test_build(
    pattern: &str,
    config: &RegexConfig,
    re: &Regex,
    oracle: &regex::bytes::Regex,
    inputs: &[(&str, bool)],
    generated: &[Vec<u8>],
    label: &str,
) -> bool {
    let grouped = Regex::with_config(
        pattern,
        RegexConfig {
            grouped_nfa: true,
            ..config.clone()
        },
    )
    .expect("compiles like the build without the option");
    if grouped.grouped_plan().is_err() {
        return false;
    }
    assert_eq!(
        format!("{:?}", grouped.states),
        format!("{:?}", re.states),
        "`{pattern}` ({label}): the grouped option changed the NFA"
    );
    let label = format!("`{pattern}` ({label}) grouped");
    for (k, &(input, _)) in inputs.iter().enumerate() {
        check_input(&grouped, oracle, input.as_bytes(), k as u64, &label);
    }
    for input in generated {
        check_grouped_only(&grouped, oracle.is_match(input), input, &label);
    }
    true
}

#[test]
fn word_boundary_patterns() {
    for k in [63usize, 64, 65, 127, 128, 129] {
        for pattern in [
            format!("a{{{k}}}b"),
            format!("x(a|aa){{{k}}}z"),
            format!("a{{{k},}}b"),
        ] {
            let re = forced(&pattern);
            let oracle = oracle(&pattern);
            for n in [0, 1, k - 2, k - 1, k, k + 1, k + 2, 2 * k + 3] {
                for input in [
                    [vec![b'a'; n], vec![b'b']].concat(),
                    [vec![b'x'], vec![b'a'; n], vec![b'z']].concat(),
                ] {
                    check_input(&re, &oracle, &input, n as u64, &pattern);
                }
            }
        }
    }
}

/// Saturated counters on long streams: the domain stays `0..=min`, and
/// nothing grows after the first call.
#[test]
fn saturated_counters_on_long_streams() {
    for (pattern, fill, end, expected) in [
        ("a{2,}b", b'a', b'b', true),
        ("x(a|aa){100,}z", b'a', b'z', false), // no `x`
        ("(a|aa){100,}b", b'a', b'b', true),
        ("(ab){70,}c", b'a', b'c', false),
    ] {
        let re = forced(pattern);
        let mut scratch = GroupedScratch::new();
        let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
        let (stream_bytes, scratch_bytes) = (s.memory_bytes(), scratch.memory_bytes());
        let block = vec![fill; 4096];
        for _ in 0..16 {
            s.chunk(&re, &mut scratch, &block);
        }
        s.chunk(&re, &mut scratch, &[end]);
        assert_eq!(s.finish(&re, &mut scratch), expected, "`{pattern}`");
        assert_eq!(s.memory_bytes(), stream_bytes);
        assert_eq!(scratch.memory_bytes(), scratch_bytes);
        let mut input = block.repeat(16);
        input.push(end);
        assert_eq!(
            oracle(pattern).is_match(&input),
            expected,
            "`{pattern}` oracle"
        );
    }
    // The phase-alternating case matches.
    let re = forced("(ab){70,}c");
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
    for _ in 0..1000 {
        s.chunk(&re, &mut scratch, b"ab");
    }
    s.chunk(&re, &mut scratch, b"c");
    assert!(s.finish(&re, &mut scratch));
}

// ---------------------------------------------------------------------------
// Work, lifecycle and memory
// ---------------------------------------------------------------------------

/// One predicate test per active consuming state, however many counts it
/// holds.
#[test]
fn one_predicate_test_per_active_state() {
    let re = forced("x(a|aa){2,9}z");
    let plan = plan(&re);
    let (w, a) = (plan.words(), plan.active_words());
    let mut raw = vec![0u64; w];
    let mut raw_active = vec![0u64; a];
    let mut saw_grouping = false;
    let mut prev = None;
    for (i, &b) in b"xaaaaaaaaz".iter().enumerate() {
        let ctx = BoundaryCtx {
            at_start: i == 0,
            prev,
            boundary: Boundary::BeforeByte(b),
        };
        let (mut closure, mut active) = (vec![0u64; w], vec![0u64; a]);
        exact::close(plan, &raw, &raw_active, &mut closure, &mut active, ctx);
        let consumers: Vec<_> = plan
            .nodes
            .iter()
            .filter(|n| matches!(n.op, super::Op::Edge { .. } | super::Op::Table(_)))
            .filter(|n| closure[n.row.range()].iter().any(|&w| w != 0))
            .collect();
        let counts: u32 = consumers
            .iter()
            .map(|n| {
                closure[n.row.range()]
                    .iter()
                    .map(|w| w.count_ones())
                    .sum::<u32>()
            })
            .sum();
        saw_grouping |= counts as usize > consumers.len();
        let (mut next, mut next_active) = (vec![0u64; w], vec![0u64; a]);
        let tests = exact::consume(
            plan,
            &re,
            &mut closure,
            &mut active,
            &mut next,
            &mut next_active,
            b,
        );
        assert_eq!(tests, consumers.len());
        raw = next;
        raw_active = next_active;
        prev = Some(b);
    }
    assert!(saw_grouping, "some state should hold several counts");
}

#[test]
fn reset_empty_chunks_and_early_success() {
    let re = forced("x(a|aa){2,5}z");
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, b"xaa");
    let before = (s.raw().to_vec(), s.display(&re).to_string());
    s.chunk(&re, &mut scratch, b"");
    assert_eq!((s.raw().to_vec(), s.display(&re).to_string()), before);
    assert!(before.1.contains(":c0={"), "{}", before.1);
    s.chunk(&re, &mut scratch, b"z");
    assert!(s.ismatch());
    // Early success: the rest of the input is not needed.
    s.chunk(&re, &mut scratch, b"qqqq");
    assert!(s.finish(&re, &mut scratch));

    s.reset(&re, &mut scratch);
    assert!(!s.ismatch());
    assert!(s.raw().iter().all(|&w| w == 0));
    s.chunk(&re, &mut scratch, b"xaz");
    assert!(!s.finish(&re, &mut scratch));

    // A pattern matching the empty string matches before any input.
    let re = forced("a{0,3}");
    let s = GroupedStream::new(&re, &mut scratch).unwrap();
    assert!(s.ismatch());
}

#[test]
#[should_panic(expected = "finished stream")]
fn chunk_after_finish_panics() {
    let re = forced("a{2,3}b");
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
    s.finish(&re, &mut scratch);
    s.chunk(&re, &mut scratch, b"a");
}

#[test]
#[should_panic(expected = "different regex")]
fn stream_with_another_regex_panics() {
    let re = forced("a{2,3}b");
    let other = forced("a{2,3}b");
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
    s.chunk(&other, &mut scratch, b"a");
}

/// Streams of a large and a small plan interleaved on one scratch: each
/// stream keeps exactly its own plan's words, and results are unaffected.
#[test]
fn interleaved_streams_share_scratch() {
    let big = forced("x(a|aa|b){3,60}z(c|cc){1,60}y(d|dd){2,60}w");
    let small = forced("a{2,3}b");
    assert!(plan(&big).words() > 2 * plan(&small).words());
    let mut scratch = GroupedScratch::new();
    let mut s_big = GroupedStream::new(&big, &mut scratch).unwrap();
    let high_water = scratch.memory_bytes();
    let mut s_small = GroupedStream::new(&small, &mut scratch).unwrap();
    let small_bytes = s_small.memory_bytes();
    assert_eq!(s_small.raw().len(), plan(&small).words());
    let mut s_small2 = GroupedStream::new(&small, &mut scratch).unwrap();

    let input_big = b"xaabaz";
    let input_small = b"qaab";
    for i in 0..input_small.len() {
        s_big.chunk(&big, &mut scratch, &input_big[i..i + 1]);
        s_small.chunk(&small, &mut scratch, &input_small[i..i + 1]);
        s_small2.chunk(&small, &mut scratch, b"q");
    }
    s_big.chunk(&big, &mut scratch, &input_big[input_small.len()..]);
    s_big.chunk(&big, &mut scratch, b"ccydddw");
    assert!(s_big.finish(&big, &mut scratch));
    assert!(s_small.finish(&small, &mut scratch));
    assert!(!s_small2.finish(&small, &mut scratch));
    assert_eq!(s_small.memory_bytes(), small_bytes);
    assert_eq!(
        scratch.memory_bytes(),
        high_water,
        "scratch keeps its high water"
    );
}

// ---------------------------------------------------------------------------
// Skipping while idle
// ---------------------------------------------------------------------------

/// Whether streams of `re` skip bytes with the prefilter while idle.
fn skips(re: &Regex) -> bool {
    plan(re).skip_when_idle && !matches!(re.prefilter, crate::Prefilter::None)
}

/// Mostly bytes the prefilter rules out, with a few candidates and short
/// runs that can match.
fn sparse_input(rng: &mut Rng, pieces: &[&[u8]]) -> Vec<u8> {
    let mut input = Vec::new();
    for _ in 0..1 + rng.below(6) {
        input.extend(std::iter::repeat_n(b'q', rng.below(40)));
        input.extend_from_slice(pieces[rng.below(pieces.len())]);
    }
    input
}

#[test]
fn skipping_while_idle_keeps_results() {
    let cases: &[(&str, &[&[u8]])] = &[
        (
            "a(a|aa){1,5}b",
            &[b"a", b"aab", b"aaaab", b"ab", b"aaaaaaaaab"],
        ),
        (r"x(a|aa){2,5}\b", &[b"x", b"xaa", b"xaaa ", b"xaaq", b"xa"]),
        ("(ab|cd){1,8}e", &[b"ab", b"abcde", b"cdcdcde", b"e"]),
        ("[a-c]{2,4}x", &[b"ab", b"abcx", b"cx", b"aaaaax"]),
        (
            r"(?m:^)?a{2,3}(?m:$)",
            &[b"aa", b"aa\n", b"aaaa\n", b"\naa"],
        ),
    ];
    let mut rng = Rng(0x5eed);
    for &(pattern, pieces) in cases {
        let oracle = oracle(pattern);
        for (unroll, merge) in CONFIGS {
            let re = compile(pattern, unroll, merge);
            if re.grouped_plan().is_err() {
                continue;
            }
            let label = format!("`{pattern}` unroll={unroll} merge={merge}");
            for k in 0..60 {
                let input = sparse_input(&mut rng, pieces);
                check_input(&re, &oracle, &input, k, &label);
                check_against_scalar(&re, &input, &label);
            }
        }
    }
}

#[test]
fn skipping_is_used_only_when_sound() {
    // A plain counted pattern: the start closure is static.
    assert!(skips(&forced("a(a|aa){1,5}b")));
    // An assertion or a match in the start closure depends on the
    // boundary: no skipping.
    assert!(!plan(&forced(r"\ba(a|aa){1,5}")).skip_when_idle);
    assert!(!plan(&forced(r"(?m:^)a{2,3}")).skip_when_idle);
    assert!(!plan(&forced("a{0,3}")).skip_when_idle);
}

/// Skipped bytes still advance the boundary: `prev` and the at-start flag
/// are those of the last byte, skipped or not.
#[test]
fn skipping_tracks_the_boundary() {
    let re = forced("a(a|aa){1,5}b");
    assert!(skips(&re));
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, b"qqqz");
    assert_eq!(s.boundary_state(), (Some(b'z'), false));
    assert!(s.raw_active().iter().all(|&w| w == 0));
    s.chunk(&re, &mut scratch, b"qqa");
    assert_eq!(s.boundary_state(), (Some(b'a'), false));
    assert!(s.raw_active().iter().any(|&w| w != 0));
    s.chunk(&re, &mut scratch, b"aq");
    assert_eq!(s.boundary_state(), (Some(b'q'), false));
    s.chunk(&re, &mut scratch, b"qqqqaaab");
    assert!(s.ismatch());
    assert!(scratch.is_clean());
}

/// On a plan of more than 32 rows, the automatic choice switches between
/// the two passes as activity rises and falls, also within one chunk; the
/// scratch must be clean for either at every byte.
#[test]
fn automatic_choice_switches_within_a_chunk() {
    // Each capital letter opens one branch and continues the others, so a
    // run of capitals keeps many branches alive at once.
    let pattern: String = (0..12u8)
        .map(|i| format!("{}[A-Za-z]{{2,9}}!", (b'A' + i) as char))
        .collect::<Vec<_>>()
        .join("|");
    let re = forced(&pattern);
    assert!(plan(&re).rows() > 32);
    let oracle = oracle(&pattern);
    let mut rng = Rng(99);
    let pieces: [&[u8]; 6] = [b"ABCDEFGHIJKL", b"!", b"Abc!", b"qqqq", b"ABq", b"AbcdefG!"];
    let mut passes = [0, 0];
    for k in 0..200 {
        let input: Vec<u8> = (0..1 + rng.below(8))
            .flat_map(|_| pieces[rng.below(pieces.len())].iter().copied())
            .collect();
        let mut scratch = GroupedScratch::new();
        let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
        s.chunk(&re, &mut scratch, &input);
        assert!(scratch.is_clean());
        assert_eq!(
            s.finish(&re, &mut scratch),
            oracle.is_match(&input),
            "{input:?}"
        );
        passes[0] += scratch.passes[0];
        passes[1] += scratch.passes[1];
        check_input(&re, &oracle, &input, k, &format!("`{pattern}`"));
    }
    assert!(passes[0] > 0 && passes[1] > 0, "{passes:?}");
}
