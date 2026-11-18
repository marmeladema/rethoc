//! Assertion tests of the grouped NFA (plan §5 and the P3 release gate).
//!
//! The grouped stream re-evaluates assertions from its raw rows at every
//! boundary.  Its prefix results are compared with the unchanged NFA's
//! unfinished-stream `ismatch` (a different scheduler: it parks deferred
//! assertions), its final results with the NFA and the `regex` crate, and
//! its configurations with the scalar interpreter of [`super::tests`].

use super::tests::{
    CONFIGS, Rng, all_strings, check_against_scalar, compile, forced, oracle, plan, run_chunks,
};
use super::{GroupedScratch, GroupedStream};
use crate::{AssertEval, AssertKind, MatcherMemory, Regex, State, StateIdx, is_word_byte};

// ---------------------------------------------------------------------------
// The evaluator contract
// ---------------------------------------------------------------------------

/// The plan's table (§5, "All current assertion kinds"): the result with
/// the next byte unknown, and at end of input, from stream start `s` and
/// the previous byte.
fn spec(kind: AssertKind, s: bool, prev: Option<u8>, at_end: bool) -> AssertEval {
    use AssertEval::{Defer as D, Fail as F, Pass as P};
    let w = prev.is_some_and(is_word_byte);
    let p_if = |c: bool| if c { P } else { F };
    match (kind, at_end) {
        (AssertKind::Start, _) => p_if(s),
        (AssertKind::End, false) => F,
        (AssertKind::End, true) => P,
        (AssertKind::StartLF, _) => p_if(s || prev == Some(b'\n')),
        (AssertKind::EndLF | AssertKind::EndCRLF, false) => D,
        (AssertKind::EndLF | AssertKind::EndCRLF, true) => P,
        (AssertKind::StartCRLF, false) => {
            if s || prev == Some(b'\n') {
                P
            } else if prev == Some(b'\r') {
                D
            } else {
                F
            }
        }
        (AssertKind::StartCRLF, true) => p_if(s || matches!(prev, Some(b'\n' | b'\r'))),
        (AssertKind::WordAscii | AssertKind::WordAsciiNegate, false) => D,
        (AssertKind::WordAscii, true) => p_if(w),
        (AssertKind::WordAsciiNegate, true) => p_if(!w),
        (AssertKind::WordStartAscii, false) => {
            if w {
                F
            } else {
                D
            }
        }
        (AssertKind::WordStartAscii, true) => F,
        (AssertKind::WordEndAscii, false) => {
            if w {
                D
            } else {
                F
            }
        }
        (AssertKind::WordEndAscii, true) => p_if(w),
    }
}

/// Every boundary a stream can be at: stream start, or after any byte.
fn boundaries() -> impl Iterator<Item = (bool, Option<u8>)> {
    std::iter::once((true, None)).chain((0..=255u8).map(|b| (false, Some(b))))
}

/// The properties raw replay relies on, for every kind: the evaluator
/// follows the plan's table; a known next byte or end of input never
/// defers; a pass with the next byte unknown stays a pass for every next
/// byte and at end of input.  A new kind fails here until reviewed.
#[test]
fn evaluator_contract_for_every_kind() {
    for &kind in AssertKind::ALL {
        for (s, prev) in boundaries() {
            let unknown = kind.eval(s, false, prev, None);
            let eoi = kind.eval(s, true, prev, None);
            assert_eq!(unknown, spec(kind, s, prev, false), "{kind:?} {prev:?}");
            assert_eq!(eoi, spec(kind, s, prev, true), "{kind:?} {prev:?} EOI");
            assert_ne!(eoi, AssertEval::Defer, "{kind:?} defers at EOI");
            if unknown == AssertEval::Pass {
                assert_eq!(eoi, AssertEval::Pass, "{kind:?} {prev:?}: unstable at EOI");
            }
            for next in 0..=255u8 {
                let known = kind.eval(s, false, prev, Some(next));
                assert_ne!(known, AssertEval::Defer, "{kind:?} defers on {next}");
                if unknown == AssertEval::Pass {
                    assert_eq!(
                        known,
                        AssertEval::Pass,
                        "{kind:?} {prev:?}: unstable on {next}"
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Drivers
// ---------------------------------------------------------------------------

/// Every partition of `input` into nonempty chunks, with an empty call
/// before, between and after the data calls.
pub(crate) fn partitions(input: &[u8]) -> Vec<Vec<&[u8]>> {
    let n = input.len();
    let cuts = n.saturating_sub(1);
    (0..1u32 << cuts)
        .map(|mask| {
            let mut parts: Vec<&[u8]> = vec![&[]];
            let mut start = 0;
            for i in 1..=n {
                if i == n || mask >> (i - 1) & 1 != 0 {
                    parts.push(&input[start..i]);
                    parts.push(&[]);
                    start = i;
                }
            }
            parts
        })
        .collect()
}

/// Every partition (against Tier 0 after each call), and `finish` at every
/// prefix on a fresh stream (against the oracle).
fn check_all_cuts(re: &Regex, oracle: &regex::bytes::Regex, input: &[u8], label: &str) {
    let expected = oracle.is_match(input);
    for parts in partitions(input) {
        run_chunks(re, &parts, expected, &format!("{label} {parts:?}"));
    }
    let mut scratch = GroupedScratch::new();
    for i in 0..=input.len() {
        let mut s = GroupedStream::new(re, &mut scratch).unwrap();
        s.chunk(re, &mut scratch, &input[..i]);
        let prefix = &input[..i];
        assert_eq!(
            s.finish(re, &mut scratch),
            oracle.is_match(prefix),
            "{label}: finish after {prefix:?}"
        );
    }
}

/// Run `chunks` with empty calls between them, and check `ismatch` after
/// construction and after every call against `after` (one entry per data
/// chunk; empty calls keep the previous value) and against Tier 0, then
/// `finish` against `finish`, Tier 0 and the oracle.
fn gate_case(
    re: &Regex,
    pattern: &str,
    chunks: &[&[u8]],
    after: &[bool],
    finish: bool,
    label: &str,
) {
    assert_eq!(chunks.len(), after.len());
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(re, &mut scratch).unwrap();
    let mut mem = MatcherMemory::default();
    let mut nfa = mem.matcher_for_tier(re, 0).unwrap();
    let mut expected = false;
    assert_eq!(s.ismatch(), expected, "{label}: at start");
    assert_eq!(nfa.ismatch(), expected, "{label}: tier 0 at start");
    for (i, (&chunk, &want)) in chunks.iter().zip(after).enumerate() {
        for c in [chunk, &[][..]] {
            if c.is_empty() {
                expected = expected || want;
            } else {
                expected = want;
            }
            s.chunk(re, &mut scratch, c);
            nfa.chunk(c);
            assert_eq!(s.ismatch(), expected, "{label}: after chunk {i} {c:?}");
            assert_eq!(
                nfa.ismatch(),
                expected,
                "{label}: tier 0 after chunk {i} {c:?}"
            );
        }
    }
    assert_eq!(s.finish(re, &mut scratch), finish, "{label}: finish");
    assert_eq!(nfa.finish(), finish, "{label}: tier 0 finish");
    assert_eq!(
        oracle(pattern).is_match(&chunks.concat()),
        finish,
        "{label}: oracle"
    );
}

/// A release-gate row: the hand-written expectations under the default
/// and the counters-forced configuration, then every partition and every
/// prefix of the whole input.
fn gate(pattern: &str, chunks: &[&[u8]], after: &[bool], finish: bool) {
    let oracle = oracle(pattern);
    for (unroll, merge) in CONFIGS {
        let re = compile(pattern, unroll, merge);
        plan(&re);
        let label = format!("`{pattern}` unroll={unroll} merge={merge}");
        gate_case(&re, pattern, chunks, after, finish, &label);
        let whole = chunks.concat();
        check_all_cuts(&re, &oracle, &whole, &label);
        check_against_scalar(&re, &whole, &label);
    }
}

// ---------------------------------------------------------------------------
// The assertion-specific release gate (plan §8)
// ---------------------------------------------------------------------------

#[test]
fn gate_absolute_end_of_input() {
    gate(r"\Aa{2}\z", &[b"aa"], &[false], true);
    // A failed end assertion does not migrate to a later boundary.
    gate(r"\Aa{2}\z", &[b"aa", b"b"], &[false, false], false);
}

#[test]
fn gate_resolving_byte_feeds_tail() {
    gate(r"\Aa{2}\b!", &[b"aa", b"!"], &[false, true], true);
    gate(r"\Aa{2}\Bz", &[b"aa", b"z"], &[false, true], true);
}

#[test]
fn gate_failed_boundary_cannot_revive() {
    gate(
        r"\Aa{2}\b",
        &[b"aa", b"b", b"!"],
        &[false, false, false],
        false,
    );
}

#[test]
fn gate_assertion_before_increment() {
    gate(r"\Ax(a\B){2}z\z", &[b"xaaz"], &[false], true);
    gate(r"\Ax(a\B){2}z\z", &[b"xaz"], &[false], false);
}

#[test]
fn gate_optional_bypass_independent() {
    gate(r"\Ax(a\b){0,2}z\z", &[b"xz"], &[false], true);
    gate(r"\Ax(a\b){0,2}z\z", &[b"xaz"], &[false], false);
}

#[test]
fn gate_exit_gates_downstream_counter() {
    gate(r"\Ax(a|aa){2}\Bz{2}\z", &[b"xaazz"], &[false], true);
    gate(r"\Ax(a|aa){2}\Bz{2}\z", &[b"xaaz"], &[false], false);
}

#[test]
fn gate_end_of_input_opens_a_chain() {
    gate(r"a{2}\z\b", &[b"aa"], &[false], true);
    gate(r"a{2}\z\b", &[b"aab"], &[false], false);
}

#[test]
fn gate_contradictory_chain() {
    gate(r"a{2}\b\B", &[b"aa"], &[false], false);
    gate(r"a{2}\b\B", &[b"aa!"], &[false], false);
}

#[test]
fn gate_empty_stream_chains() {
    gate(r"\z\B", &[], &[], true);
    gate(r"\z\b", &[], &[], false);
}

#[test]
fn gate_crlf_line_start_needs_lookahead() {
    gate(r"\A\r(?mR:^)a", &[b"\r", b"a"], &[false, true], true);
    gate(
        r"\A\r(?mR:^)a",
        &[b"\r", b"\n", b"a"],
        &[false, false, false],
        false,
    );
    // End of input resolves the line start after CR.
    gate(r"\A\r(?mR:^)", &[b"\r"], &[false], true);
    gate(r"\A\r(?mR:^)", &[b"\r", b"\n"], &[false, false], false);
}

#[test]
fn gate_no_interior_crlf_line_end() {
    gate(r"\A\r(?mR:$)\n", &[b"\r", b"\n"], &[false, false], false);
    gate(r"\A\r(?m:$)\n", &[b"\r", b"\n"], &[false, true], true);
}

#[test]
fn gate_end_of_input_cannot_consume() {
    gate(r"\Aa{2}\z\x00", &[b"aa"], &[false], false);
    gate(r"\Aa{2}\z\x00", &[b"aa\x00"], &[false], false);
    gate(r"\Aa{2}\x00\z", &[b"aa\x00"], &[false], true);
}

#[test]
fn gate_guarded_and_unconditional_convergence() {
    gate(r"\A(?:a{2}\b|a{2})!\z", &[b"aa!"], &[false], true);
    gate(
        r"\A(?:a{2}\b|a{2})!\z",
        &[b"aa", b"!"],
        &[false, false],
        true,
    );
}

/// `(?:\b){2,3}` compiles to a lone `\b`; a counted body whose only
/// non-consuming path runs through an assertion is a zero-width cycle and
/// is rejected rather than deferred indefinitely.
#[test]
fn gate_assertion_only_counted_body() {
    let re = forced(r"(?:\b){2,3}");
    assert_eq!(re.num_counters, 0);
    plan(&re);
    let re = forced(r"(?:\b|a){2,3}");
    assert!(matches!(
        re.grouped_plan(),
        Err(super::GroupedNfaError::Rejected(
            super::Rejection::ZeroWidthCycle { .. }
        ))
    ));
}

// ---------------------------------------------------------------------------
// Ordered pairs of assertion kinds
// ---------------------------------------------------------------------------

/// Pattern syntax of every assertion kind.
pub(crate) const KINDS: [&str; 10] = [
    r"\A",
    r"\z",
    r"(?m:^)",
    r"(?m:$)",
    r"(?mR:^)",
    r"(?mR:$)",
    r"\b",
    r"\B",
    r"\b{start}",
    r"\b{end}",
];

/// Positions of an assertion pair `{P}` relative to a counted repetition
/// with bounds `{C}`, over word (`a`, `b`), non-word (`-`), NUL, 0xFF and
/// line-break bytes.
pub(crate) const POSITIONS: [&str; 5] = [
    "{P}(?:a|b-|\\x00){C}",                // head
    "[a\\n\\r\\x00\\xff-]{P}(?:a|b-){C}",  // entry
    "(?:a{P}|-|\\xff){C}b",                // body
    "(?:a|-b|\\x00){C}{P}[a\\r\\n\\xff-]", // exit
    "(?:a|-|\\xff){C}{P}",                 // tail
];

/// Counted domains: finite, and unbounded (counts saturate at `min`).
const COUNTERS: [&str; 2] = ["{1,3}", "{2,}"];

pub(crate) const PAIR_ALPHABET: &[u8] = b"ab-\r\n\x00\xff";

/// A pattern of the pair matrix.
pub(crate) fn pair_pattern(position: &str, pair: &str, counter: &str) -> String {
    position.replace("{P}", pair).replace("{C}", counter)
}

/// All ordered pairs (contradictions included) at every position, under
/// the default and the counters-forced configurations: grouped against
/// Tier 0 after every call and at `finish`, against the oracle, and against
/// the scalar interpreter.
#[test]
fn assertion_pairs_at_every_position() {
    // Every kind is exercised: the syntax list covers `AssertKind::ALL`.
    let mut seen = std::collections::BTreeSet::new();
    for k in KINDS {
        let re = forced(k);
        for s in re.states.iter() {
            if let crate::State::Assert { kind, .. } = s {
                seen.insert(format!("{kind:?}"));
            }
        }
    }
    assert_eq!(seen.len(), AssertKind::ALL.len(), "{seen:?}");

    let inputs = all_strings(PAIR_ALPHABET, 4);
    let mut rng = Rng(7);
    let (mut admitted, mut rejected) = (0, 0);
    for (position, counter) in POSITIONS.iter().flat_map(|p| COUNTERS.map(|c| (p, c))) {
        for a in KINDS {
            for b in KINDS {
                let pattern = pair_pattern(position, &format!("{a}{b}"), counter);
                let oracle = oracle(&pattern);
                for (unroll, merge) in [CONFIGS[0], CONFIGS[2]] {
                    let re = compile(&pattern, unroll, merge);
                    if re.grouped_plan().is_err() {
                        rejected += 1;
                        continue;
                    }
                    admitted += 1;
                    let label = format!("`{pattern}` unroll={unroll}");
                    for _ in 0..16 {
                        let input = &inputs[rng.below(inputs.len())];
                        let expected = oracle.is_match(input);
                        run_chunks(&re, &[input], expected, &label);
                        let bytes: Vec<&[u8]> = input.chunks(1).collect();
                        run_chunks(&re, &bytes, expected, &label);
                    }
                    for _ in 0..4 {
                        let input = &inputs[rng.below(inputs.len())];
                        check_against_scalar(&re, input, &format!("{label} {input:?}"));
                    }
                }
            }
        }
    }
    // Admission may reject a graph (a nested or cyclic one); none of these
    // should be, and a silent drop in coverage must fail.
    assert_eq!(rejected, 0, "{admitted} admitted");
}

/// Short inputs over the pair alphabet, every partition and prefix, for
/// one pattern per position and counted domain with a contradiction-free
/// mixed pair: every input up to 3 bytes, and a sample of 4 and 5 bytes.
#[test]
fn assertion_patterns_every_cut() {
    let mut rng = Rng(11);
    let longer: Vec<Vec<u8>> = (0..150)
        .map(|_| {
            (0..4 + rng.below(2))
                .map(|_| PAIR_ALPHABET[rng.below(PAIR_ALPHABET.len())])
                .collect()
        })
        .collect();
    for (position, counter) in POSITIONS.iter().flat_map(|p| COUNTERS.map(|c| (p, c))) {
        for pair in [r"\b(?m:$)", r"(?mR:^)\B", r"\b{end}(?mR:$)", r"\z\B"] {
            let pattern = pair_pattern(position, pair, counter);
            let oracle = oracle(&pattern);
            let re = forced(&pattern);
            for input in all_strings(PAIR_ALPHABET, 3).iter().chain(&longer) {
                check_all_cuts(&re, &oracle, input, &format!("`{pattern}`"));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Hand-built graphs
// ---------------------------------------------------------------------------

/// A regex whose NFA is `states`, started at state 0, for shapes that
/// pattern normalisation might never produce.  Every DFA tier is off: only
/// Tier 0, the grouped NFA and the scalar interpreter run on it.
pub(crate) fn fixture_regex(states: Vec<State>, num_counters: usize) -> Regex {
    let mut re = compile("a", 0, false);
    re.states = crate::StateList(states.into_boxed_slice());
    re.start = StateIdx(0);
    re.num_counters = num_counters;
    re.counter_info = Box::default();
    re.start_closure = Box::default();
    re.start_closure_matches = false;
    re.dfa_eligible = false;
    re.tier2_eligible = false;
    re.tier2_overlap_proven = false;
    re.tier2_analysis = None;
    re.tier3_eligible = false;
    re.tier3_analysis = None;
    re.tier4_eligible = false;
    re.prefilter = crate::Prefilter::None;
    re.state_can_reach_match = vec![false; re.states.len()].into_boxed_slice();
    re.counter_break_can_match = Box::default();
    re.counter_body_states = Box::default();
    let analysis = super::analyze(
        &re.states,
        &re.classes,
        &re.byte_tables,
        re.start,
        num_counters,
        &[],
        &super::Limits::DEFAULT,
    );
    re.grouped_nfa = Some(Box::new(match analysis {
        Ok(plan) => super::GroupedNfa::Admitted(plan),
        Err(r) => super::GroupedNfa::Rejected(r),
    }));
    // The sparse counting variant of the same graph, where it is admitted.
    let structure = super::structure(
        &re.states,
        &re.byte_tables,
        re.start,
        num_counters,
        &[],
        &super::Limits::DEFAULT,
    );
    re.sparse_counting = Some(Box::new(
        match super::sparse::analyze(
            &re.states,
            &re.classes,
            &re.byte_tables,
            re.start,
            structure.as_ref().map_err(|r| *r),
            &super::sparse::SparseLimits::DEFAULT,
        ) {
            Ok(plan) => super::SparseCounting::Admitted(plan),
            Err(r) => super::SparseCounting::Rejected(r),
        },
    ));
    re
}

pub(crate) fn by(b: u8, out: u32) -> State {
    State::Byte {
        byte: b,
        out: StateIdx(out),
        out_exit: StateIdx::NONE,
    }
}

pub(crate) fn split(out: u32, out1: u32) -> State {
    State::Split {
        out: StateIdx(out),
        out1: StateIdx(out1),
    }
}

pub(crate) fn assert_(kind: AssertKind, out: u32) -> State {
    State::Assert {
        kind,
        out: StateIdx(out),
    }
}

pub(crate) fn ci(counter: u8, out: u32) -> State {
    State::CounterInstance {
        counter: crate::CounterIdx(counter),
        out: StateIdx(out),
    }
}

pub(crate) fn cinc(counter: u8, min: usize, max: usize, out: u32, out1: u32) -> State {
    State::CounterIncrement {
        counter: crate::CounterIdx(counter),
        out: StateIdx(out),
        out1: StateIdx(out1),
        min,
        max,
    }
}

/// Run a fixture on every input up to 4 bytes over `alphabet`: every
/// partition (with empty calls) against Tier 0 after each call, with the
/// final answer from Tier 0 fed whole, and the scalar interpreter at every
/// boundary.  Both references are independent of the grouped NFA, and
/// Tier 0 parks deferred assertions where the grouped NFA replays them.
/// Some inputs must match and some must not, so that a fixture cannot pass
/// vacuously.
fn check_fixture(re: &Regex, alphabet: &[u8], label: &str) {
    plan(re);
    let mut mem = MatcherMemory::default();
    let mut matching = 0;
    let inputs = all_strings(alphabet, 4);
    for input in &inputs {
        let mut nfa = mem.matcher_for_tier(re, 0).unwrap();
        nfa.chunk(input);
        let expected = nfa.finish();
        matching += usize::from(expected);
        for parts in partitions(input) {
            run_chunks(re, &parts, expected, &format!("{label} {parts:?}"));
        }
        check_against_scalar(re, input, &format!("{label} {input:?}"));
        // The sparse counting variant, on the fixtures it admits (all but
        // the one with an unbounded counter).
        if re.sparse_plan().is_ok() {
            for parts in partitions(input) {
                super::sparse::tests::run_chunks(
                    re,
                    &parts,
                    expected,
                    &format!("{label} sparse {parts:?}"),
                );
            }
            super::sparse::tests::check_profiles(re, input, &format!("{label} sparse"));
        }
    }
    assert_eq!(
        re.sparse_plan().is_ok(),
        re.states
            .iter()
            .all(|s| !matches!(s, State::CounterIncrement { max, .. } if *max == usize::MAX)),
        "{label}: sparse admission"
    );
    assert!(
        0 < matching && matching < inputs.len(),
        "{label}: {matching} of {} inputs match",
        inputs.len()
    );
}

/// A guarded exit of the counter and a guarded bypass around it converge on
/// one tail state: `(?:a{1,3}\b|\B)-`, with the tail shared.
#[test]
fn fixture_guarded_bypass_and_exit_share_the_tail() {
    let re = fixture_regex(
        vec![
            split(1, 5),
            ci(0, 2),
            by(b'a', 3),
            cinc(0, 1, 3, 2, 4),
            assert_(AssertKind::WordAscii, 6),
            assert_(AssertKind::WordAsciiNegate, 6),
            by(b'-', 7),
            State::Match,
        ],
        1,
    );
    check_fixture(&re, b"a- x", "shared tail");
}

/// A guarded and an unguarded body branch share the increment:
/// `(?:a\b|b){2,3}!` with one `CounterIncrement` for both.
#[test]
fn fixture_body_branches_share_the_increment() {
    let re = fixture_regex(
        vec![
            ci(0, 1),
            split(2, 4),
            by(b'a', 3),
            assert_(AssertKind::WordAscii, 5),
            by(b'b', 5),
            cinc(0, 2, 3, 1, 6),
            by(b'!', 7),
            State::Match,
        ],
        1,
    );
    check_fixture(&re, b"ab! ", "shared increment");
}

/// Contradictory guards after a counter converge on one shared assertion:
/// `a{1,4}(?:\b|\B)(?m:$)`, both guards feeding the same `(?m:$)` node.
#[test]
fn fixture_contradictory_guards_share_an_assertion() {
    let re = fixture_regex(
        vec![
            ci(0, 1),
            by(b'a', 2),
            cinc(0, 1, 4, 1, 3),
            split(4, 5),
            assert_(AssertKind::WordAscii, 6),
            assert_(AssertKind::WordAsciiNegate, 6),
            assert_(AssertKind::EndLF, 7),
            State::Match,
        ],
        1,
    );
    check_fixture(&re, b"a\n -", "shared assertion");
}

/// A guarded and a consuming path enter the same counter instance:
/// `(?:(?m:^)|x)a{2}b`, one `CounterInstance` for both.
#[test]
fn fixture_guarded_and_consuming_entries_share_the_instance() {
    let re = fixture_regex(
        vec![
            split(1, 2),
            assert_(AssertKind::StartLF, 3),
            by(b'x', 3),
            ci(0, 4),
            by(b'a', 5),
            cinc(0, 2, 2, 4, 6),
            by(b'b', 7),
            State::Match,
        ],
        1,
    );
    check_fixture(&re, b"axb\n", "shared instance");
}

/// An assertion between one counter's break and the next counter's
/// instance, with a saturating second counter: `a{1,2}\Bb{2,}`.
#[test]
fn fixture_guard_between_counters() {
    let re = fixture_regex(
        vec![
            ci(0, 1),
            by(b'a', 2),
            cinc(0, 1, 2, 1, 3),
            assert_(AssertKind::WordAsciiNegate, 4),
            ci(1, 5),
            by(b'b', 6),
            cinc(1, 2, usize::MAX, 5, 7),
            State::Match,
        ],
        2,
    );
    check_fixture(&re, b"ab- ", "guard between counters");
}

// ---------------------------------------------------------------------------
// Streams and boundary metadata
// ---------------------------------------------------------------------------

/// Streams at different previous-byte categories, interleaved through one
/// scratch, each resolve their own boundary.
#[test]
fn interleaved_streams_resolve_their_own_boundaries() {
    let pattern = r"\b(a|aa){2,5}\b";
    let re = forced(pattern);
    let oracle = oracle(pattern);
    let inputs: [&[u8]; 5] = [b"aaa ", b"aaaaaaa", b"-aa\n", b"\raa-a", b"xaa"];
    let mut scratch = GroupedScratch::new();
    let mut streams: Vec<GroupedStream> = inputs
        .iter()
        .map(|_| GroupedStream::new(&re, &mut scratch).unwrap())
        .collect();
    let longest = inputs.iter().map(|i| i.len()).max().unwrap();
    for i in 0..longest {
        for (s, input) in streams.iter_mut().zip(inputs) {
            if let Some(b) = input.get(i) {
                s.chunk(&re, &mut scratch, std::slice::from_ref(b));
            }
        }
    }
    for (s, input) in streams.iter_mut().zip(inputs) {
        assert_eq!(
            s.finish(&re, &mut scratch),
            oracle.is_match(input),
            "{input:?}"
        );
    }
}

/// Provisional closures (constructor, end of chunk, empty calls) leave the
/// raw rows and boundary metadata alone; only consumption replaces them.
#[test]
fn provisional_closure_does_not_touch_raw_rows() {
    let re = forced(r"\Ax(a|aa){2,3}\b");
    let mut scratch = GroupedScratch::new();
    let mut s = GroupedStream::new(&re, &mut scratch).unwrap();
    assert!(s.raw().iter().all(|&w| w == 0));
    assert_eq!(s.boundary_state(), (None, true));
    s.chunk(&re, &mut scratch, b"");
    assert!(s.raw().iter().all(|&w| w == 0));
    assert_eq!(s.boundary_state(), (None, true));

    s.chunk(&re, &mut scratch, b"xaa");
    let raw = s.raw().to_vec();
    assert_eq!(s.boundary_state(), (Some(b'a'), false));
    for _ in 0..3 {
        s.chunk(&re, &mut scratch, b"");
        assert!(!s.ismatch());
        assert_eq!(s.raw(), &raw[..]);
        assert_eq!(s.boundary_state(), (Some(b'a'), false));
    }
    // `\b` defers before the next byte; a non-word byte resolves it.
    s.chunk(&re, &mut scratch, b"!");
    assert!(s.ismatch());
}
