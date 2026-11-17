//! Tests of the grouped NFA: admission structure and budgets, transfers
//! against `counter_step`, configurations against an independent scalar
//! interpreter, and match results against Tier 0 and the `regex` crate.

use std::collections::BTreeSet;

use super::analysis::{self, Limits, Rejection, Scope};
use super::exact::{self, Boundary, BoundaryCtx};
use super::{CincMasks, GroupedNfaError, GroupedNfaPlan, GroupedScratch, GroupedStream};
use crate::{
    ByteMap, CounterIdx, MatcherMemory, Regex, RegexConfig, State, StateIdx, byte_match_ci,
    counter_step,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The three compile configurations of the test suite: default unrolling,
/// unrolling off (counters forced), unrolling and merging off.
const CONFIGS: [(usize, bool); 3] = [
    (crate::DEFAULT_MAX_UNROLL_STATES, true),
    (0, true),
    (0, false),
];

fn compile(pattern: &str, unroll: usize, merge: bool) -> Regex {
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
fn forced(pattern: &str) -> Regex {
    compile(pattern, 0, false)
}

fn plan(re: &Regex) -> &GroupedNfaPlan {
    re.grouped_plan()
        .unwrap_or_else(|e| panic!("expected admission: {e}"))
}

fn rejection(re: &Regex) -> Rejection {
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
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

// ---------------------------------------------------------------------------
// Admission: patterns
// ---------------------------------------------------------------------------

/// The scope of each state with a row in `plan`.
fn scope_of(plan: &GroupedNfaPlan, state: StateIdx) -> Option<Scope> {
    plan.scopes
        .iter()
        .find(|(s, _)| *s == state)
        .map(|&(_, scope)| scope)
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
    // A zero-width cycle is reported before the assertion exclusion.
    // (`(?:\b){2,3}` itself compiles to a lone `\b`, without a counter.)
    assert!(matches!(
        rejection(&forced(r"(?:\b|a){2,3}")),
        Rejection::ZeroWidthCycle { .. }
    ));
}

#[test]
fn milestone_exclusions() {
    assert!(matches!(
        rejection(&forced(r"\Ax(a|aa){2}z")),
        Rejection::Assertion { .. }
    ));
    assert!(matches!(
        rejection(&forced("a{2,}b")),
        Rejection::UnboundedCounter { .. }
    ));
    // `a{63}` keeps 0..=63 in one word; `a{64}` needs a second.
    plan(&forced("a{63}b"));
    for pattern in ["a{64}b", "a{65}b"] {
        assert!(matches!(
            rejection(&forced(pattern)),
            Rejection::CounterTooWide { limit: 63, .. }
        ));
    }
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
    let rejected = forced("a{2,}b");
    assert!(matches!(
        GroupedStream::new(&rejected, &mut scratch).err(),
        Some(GroupedNfaError::Rejected(
            Rejection::UnboundedCounter { .. }
        ))
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
    let g = info.grouped_nfa.as_ref().expect("requested");
    assert!(g.admitted);
    assert_eq!(g.plan_bytes, plan(&re).heap_bytes());
    assert_eq!(g.stream_bytes, 8 * g.row_words);
    assert_eq!(g.scratch_bytes, 16 * g.row_words);
    assert!(
        info.to_string()
            .contains("Grouped NFA (experimental): admitted")
    );
    assert!(re.dump(false).to_string().contains("Grouped NFA: admitted"));

    let re = forced("a{2,}b");
    let info = re.info();
    let g = info.grouped_nfa.as_ref().expect("requested");
    assert!(!g.admitted);
    assert!(g.rejection.as_ref().unwrap().contains("unbounded"));
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
        analyze_fixture(&states, 1, &Limits::P1).err(),
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
        analyze_fixture(&states, 2, &Limits::P1).err(),
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
        analyze_fixture(&states, 1, &Limits::P1).err(),
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
        analyze_fixture(&states, 2, &Limits::P1).err(),
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
        analyze_fixture(&states, 1, &Limits::P1).err(),
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
            analyze_fixture(&states, 1, &Limits::P1).err(),
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
        analyze_fixture(&states, 0, &Limits::P1).err(),
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
    let plan = analyze_fixture(&states, 1, &Limits::P1).unwrap();
    assert_eq!(plan.rows(), 3);
    assert_eq!(plan.row_of[3], super::Row::NONE);
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
    let full = run(Limits::P1).unwrap();
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
        .scopes
        .iter()
        .map(|(s, _)| match states[*s] {
            State::ByteTable { .. } => 256,
            State::Match => 0,
            _ => 2,
        })
        .sum();
    assert!(reachable_edges <= edges);

    let with = |f: &dyn Fn(&mut Limits)| {
        let mut l = Limits::P1;
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
    assert!(with(&|l| l.max_counter_max = 5).is_ok());
    assert!(matches!(
        with(&|l| l.max_counter_max = 4).err(),
        Some(Rejection::CounterTooWide {
            max: 5,
            limit: 4,
            ..
        })
    ));
}

/// A plan-budget rejection leaves normal compilation and dispatch usable.
#[test]
fn rejection_keeps_regex_usable() {
    let re = forced("a{2,}b");
    assert!(re.grouped_plan().is_err());
    let mut mem = MatcherMemory::default();
    let mut m = mem.matcher(&re);
    m.chunk(b"xaaab");
    assert!(m.finish());
}

// ---------------------------------------------------------------------------
// Transfers
// ---------------------------------------------------------------------------

/// Scalar increment of every count in `s`, by `counter_step`.
fn scalar_increment(s: u64, min: usize, max: usize) -> (u64, bool) {
    let mut cont = 0u64;
    let mut brk = false;
    for v in 0..64 {
        if s >> v & 1 == 0 {
            continue;
        }
        let nv = counter_step(v, min, max);
        if nv < max {
            cont |= 1 << nv;
        }
        if nv >= min {
            brk = true;
        }
    }
    (cont, brk)
}

#[test]
fn increment_matches_counter_step_exhaustively() {
    for max in 1..=10usize {
        for min in 0..=max {
            let masks = CincMasks::finite(min, max);
            // Every subset of the incoming domain 0..max.
            for s in 0..(1u64 << max) {
                assert_eq!(
                    exact::increment(s, masks),
                    scalar_increment(s, min, max),
                    "{{{min},{max}}} S={s:#b}"
                );
            }
        }
    }
}

#[test]
fn increment_matches_counter_step_on_wide_words() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for max in [31usize, 32, 33, 62, 63] {
        for min in [0, 1, max / 2, max - 1, max] {
            let masks = CincMasks::finite(min, max);
            let domain = u64::MAX >> (64 - max); // 0..max
            for _ in 0..2000 {
                let s = rng.next() & rng.next() & domain;
                assert_eq!(exact::increment(s, masks), scalar_increment(s, min, max));
            }
            for s in [0, 1, domain, 1 << (max - 1)] {
                assert_eq!(exact::increment(s, masks), scalar_increment(s, min, max));
            }
        }
    }
}

#[test]
#[should_panic(expected = "reached its maximum")]
fn increment_of_the_maximum_is_impossible() {
    exact::increment(1 << 5, CincMasks::finite(2, 5));
}

// ---------------------------------------------------------------------------
// Independent scalar interpreter
// ---------------------------------------------------------------------------

/// A configuration: a state and its count (`None` outside any counter).
type Config = (StateIdx, Option<usize>);

/// Scalar zero-width closure of `roots` plus the start state, by a plain
/// visited-set search.  Returns every visited configuration and whether
/// `Match` was reached.
fn scalar_close(re: &Regex, roots: &BTreeSet<Config>) -> (BTreeSet<Config>, bool) {
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
            State::Assert { .. } => panic!("assertions are not admitted"),
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

/// Expand grouped rows into configurations.
fn expand(plan: &GroupedNfaPlan, rows: &[u64]) -> BTreeSet<Config> {
    let mut out = BTreeSet::new();
    for &(state, scope) in plan.scopes.iter() {
        let row = plan.row_of[state.idx()];
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
/// interpreter; compare raw roots, closures and match flags at every
/// boundary.
fn check_against_scalar(re: &Regex, input: &[u8], label: &str) {
    let plan = plan(re);
    let mut scratch = GroupedScratch::new();
    let mut stream = GroupedStream::new(re, &mut scratch).unwrap();
    let mut raw: BTreeSet<Config> = BTreeSet::new();
    let mut closure_rows = vec![0u64; plan.words()];
    let mut scalar_matched = false;
    for (i, &b) in input.iter().enumerate() {
        assert_eq!(expand(plan, stream.raw()), raw, "{label}: raw roots at {i}");
        let (closure, m) = scalar_close(re, &raw);
        scalar_matched |= m;
        let ctx = BoundaryCtx {
            at_start: i == 0,
            prev: i.checked_sub(1).map(|j| input[j]),
            boundary: Boundary::BeforeByte(b),
        };
        let gm = exact::close(plan, stream.raw(), &mut closure_rows, ctx);
        assert_eq!(
            expand(plan, &closure_rows),
            closure,
            "{label}: closure at {i}"
        );
        assert_eq!(gm, m, "{label}: match at {i}");
        assert_eq!(
            stream.ismatch(),
            scalar_matched,
            "{label}: ismatch before {i}"
        );
        if scalar_matched {
            return;
        }
        raw = scalar_consume(re, &closure, b);
        stream.chunk(re, &mut scratch, &[b]);
    }
    let (_, m) = scalar_close(re, &raw);
    scalar_matched |= m;
    if !stream.ismatch() {
        assert_eq!(expand(plan, stream.raw()), raw, "{label}: final raw roots");
    }
    assert_eq!(
        stream.finish(re, &mut scratch),
        scalar_matched,
        "{label}: finish"
    );
}

/// Every string over `alphabet` up to `max_len`.
fn all_strings(alphabet: &[u8], max_len: usize) -> Vec<Vec<u8>> {
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
    ("abc|abd", b"abcd"),
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
            for input in all_strings(alphabet, 6) {
                check_against_scalar(&re, &input, &format!("{label} on {input:?}"));
            }
        }
    }
}

#[test]
fn configurations_match_scalar_interpreter_on_long_inputs() {
    let mut rng = Rng(42);
    for pattern in ["a{63}b", "x(a|aa){2,5}z", "a(a|aa){1,40}b", "(a{2,3}b)*c"] {
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

fn oracle(pattern: &str) -> regex::bytes::Regex {
    regex::bytes::Regex::new(&format!("(?s-u){pattern}")).unwrap()
}

/// Feed `chunks` to the grouped stream and to Tier 0 side by side;
/// `ismatch` must agree after construction and every call, `finish` must
/// agree with both.
fn run_chunks(re: &Regex, chunks: &[&[u8]], expected: bool, label: &str) {
    let mut scratch = GroupedScratch::new();
    let mut stream = GroupedStream::new(re, &mut scratch).unwrap();
    let mut mem = MatcherMemory::default();
    let mut nfa = mem.matcher_for_tier(re, 0).unwrap();
    assert_eq!(stream.ismatch(), nfa.ismatch(), "{label}: before input");
    for (i, chunk) in chunks.iter().enumerate() {
        stream.chunk(re, &mut scratch, chunk);
        nfa.chunk(chunk);
        assert_eq!(stream.ismatch(), nfa.ismatch(), "{label}: after chunk {i}");
    }
    let got = stream.finish(re, &mut scratch);
    assert_eq!(nfa.finish(), expected, "{label}: tier 0");
    assert_eq!(got, expected, "{label}: grouped");
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
fn boundary_patterns_63() {
    let re = forced("a{63}b");
    let oracle = oracle("a{63}b");
    for n in [0, 1, 62, 63, 64, 100] {
        let mut input = vec![b'a'; n];
        input.push(b'b');
        check_input(&re, &oracle, &input, n as u64, "a{63}b");
    }
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
    let mut closure = vec![0u64; plan.words()];
    let mut next = vec![0u64; plan.words()];
    let mut raw = vec![0u64; plan.words()];
    let mut saw_grouping = false;
    let mut prev = None;
    for (i, &b) in b"xaaaaaaaaz".iter().enumerate() {
        let ctx = BoundaryCtx {
            at_start: i == 0,
            prev,
            boundary: Boundary::BeforeByte(b),
        };
        exact::close(plan, &raw, &mut closure, ctx);
        let active = plan
            .consumers
            .iter()
            .filter(|c| closure[c.src.range()].iter().any(|&w| w != 0))
            .count();
        let counts: u32 = plan
            .consumers
            .iter()
            .map(|c| {
                closure[c.src.range()]
                    .iter()
                    .map(|w| w.count_ones())
                    .sum::<u32>()
            })
            .sum();
        saw_grouping |= counts as usize > active;
        assert_eq!(exact::consume(plan, &re, &closure, &mut next, b), active);
        raw.copy_from_slice(&next);
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
