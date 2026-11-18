//! Assertion tests of the sparse counting variant
//! (`plans/sparse-counting-sets.md`, §6 and §9).
//!
//! The stream re-evaluates assertions from its raw frontier at every
//! boundary.  Its prefix results are compared with the NFA's
//! unfinished-stream `ismatch` (a different scheduler: it parks deferred
//! assertions), its final results with the NFA and the `regex` crate, and
//! its raw rows with those of the exact grouped NFA.  The evaluator
//! itself is tested with the exact grouped NFA
//! (`assertion_tests::evaluator_contract_for_every_kind`).

use super::super::assertion_tests::{KINDS, PAIR_ALPHABET, POSITIONS, pair_pattern, partitions};
use super::super::tests::{CONFIGS, Rng, all_strings, oracle};
use super::tests::{check_profiles, run_chunks, sparse_compile, sparse_forced};
use super::*;
use crate::{AssertKind, MatcherMemory, Regex};

// ---------------------------------------------------------------------------
// Drivers
// ---------------------------------------------------------------------------

/// Every partition (against Tier 0 after each call), `finish` at every
/// prefix on a fresh stream (against the oracle), and the raw rows against
/// the exact grouped NFA.
fn check_all_cuts(re: &Regex, oracle: &regex::bytes::Regex, input: &[u8], label: &str) {
    let expected = oracle.is_match(input);
    for parts in partitions(input) {
        run_chunks(re, &parts, expected, &format!("{label} {parts:?}"));
    }
    let mut scratch = SparseScratch::new();
    for i in 0..=input.len() {
        let mut s = SparseStream::new(re, &mut scratch).unwrap();
        s.chunk(re, &mut scratch, &input[..i]);
        let prefix = &input[..i];
        assert_eq!(
            s.finish(re, &mut scratch),
            oracle.is_match(prefix),
            "{label}: finish after {prefix:?}"
        );
    }
    check_profiles(re, input, label);
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
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(re, &mut scratch).unwrap();
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

/// A gate row: the hand-written expectations under the default and the
/// counters-forced configurations, then every partition and every prefix
/// of the whole input.  Returns whether a counter was compiled under the
/// forced configuration, so that a row cannot silently test nothing.
fn gate(pattern: &str, chunks: &[&[u8]], after: &[bool], finish: bool) -> bool {
    let oracle = oracle(pattern);
    let mut counted = false;
    for (unroll, merge) in CONFIGS {
        let re = sparse_compile(pattern, unroll, merge);
        assert!(
            re.sparse_plan().is_ok(),
            "`{pattern}` unroll={unroll}: {:?}",
            re.sparse_plan().err()
        );
        counted |= re.sparse_plan().unwrap().slots > 0;
        let label = format!("`{pattern}` unroll={unroll} merge={merge}");
        gate_case(&re, pattern, chunks, after, finish, &label);
        check_all_cuts(&re, &oracle, &chunks.concat(), &label);
    }
    counted
}

// ---------------------------------------------------------------------------
// The assertion gate (the rows of the exact grouped NFA's gate)
// ---------------------------------------------------------------------------

#[test]
fn gate_rows() {
    // (pattern, chunks, ismatch after each chunk, finish)
    type Row<'a> = (&'a str, &'a [&'a [u8]], &'a [bool], bool);
    let rows: &[Row] = &[
        // Absolute end of input; a failed end does not migrate to a later
        // boundary.
        (r"\Aa{2}\z", &[b"aa"], &[false], true),
        (r"\Aa{2}\z", &[b"aa", b"b"], &[false, false], false),
        // The resolving byte feeds the tail.
        (r"\Aa{2}\b!", &[b"aa", b"!"], &[false, true], true),
        (r"\Aa{2}\Bz", &[b"aa", b"z"], &[false, true], true),
        // A failed boundary cannot revive.
        (
            r"\Aa{2}\b",
            &[b"aa", b"b", b"!"],
            &[false, false, false],
            false,
        ),
        // An assertion before the increment.
        (r"\Ax(a\B){2}z\z", &[b"xaaz"], &[false], true),
        (r"\Ax(a\B){2}z\z", &[b"xaz"], &[false], false),
        // The optional bypass is independent of the guarded body.
        (r"\Ax(a\b){0,2}z\z", &[b"xz"], &[false], true),
        (r"\Ax(a\b){0,2}z\z", &[b"xaz"], &[false], false),
        // An exit gates a downstream counter.
        (r"\Ax(a|aa){2}\Bz{2}\z", &[b"xaazz"], &[false], true),
        (r"\Ax(a|aa){2}\Bz{2}\z", &[b"xaaz"], &[false], false),
        // End of input opens a chain.
        (r"a{2}\z\b", &[b"aa"], &[false], true),
        (r"a{2}\z\b", &[b"aab"], &[false], false),
        // A contradictory chain.
        (r"a{2}\b\B", &[b"aa"], &[false], false),
        (r"a{2}\b\B", &[b"aa!"], &[false], false),
        // A CRLF line start needs the next byte.
        (r"\A\r(?mR:^)a{2}", &[b"\r", b"aa"], &[false, true], true),
        (
            r"\A\r(?mR:^)a{2}",
            &[b"\r", b"\n", b"aa"],
            &[false, false, false],
            false,
        ),
        // No line end between CR and LF.
        (
            r"\Aa{2}\r(?mR:$)\n",
            &[b"aa\r", b"\n"],
            &[false, false],
            false,
        ),
        (r"\Aa{2}\r(?m:$)\n", &[b"aa\r", b"\n"], &[false, true], true),
        // End of input cannot consume.
        (r"\Aa{2}\z\x00", &[b"aa"], &[false], false),
        (r"\Aa{2}\z\x00", &[b"aa\x00"], &[false], false),
        (r"\Aa{2}\x00\z", &[b"aa\x00"], &[false], true),
        // A guarded and an unguarded path converge.
        (r"\A(?:a{2}\b|a{2})!\z", &[b"aa!"], &[false], true),
        (
            r"\A(?:a{2}\b|a{2})!\z",
            &[b"aa", b"!"],
            &[false, false],
            true,
        ),
        // The plan's shapes: a guarded and an unguarded body branch, a
        // guarded exit, lines, the whole input, a line start in a body.
        (r"a(a\b|aa){2,5}z", &[b"aaaaa", b"z"], &[false, true], true),
        (
            r"a(a\b|aa){2,5}z",
            &[b"aa", b" a z"],
            &[false, false],
            false,
        ),
        (
            r"x(a|aa){2,5}\b-",
            &[b"xaa", b"a", b"-"],
            &[false, false, true],
            true,
        ),
        (r"x(a|aa){2,5}\bz", &[b"xaaa", b"z"], &[false, false], false),
        (
            r"(?mR:^x(a|aa){2,5}$)",
            &[b"q\r\nxaa", b"\r", b"\n"],
            &[false, true, true],
            true,
        ),
        (r"(?mR:^x(a|aa){2,5}$)", &[b"qxaa\r\n"], &[false], false),
        (r"(?mR:^x(a|aa){2,5}$)", &[b"xaaa"], &[false], true),
        (r"\Ax(a|aa){2,5}\z", &[b"xa", b"a"], &[false, false], true),
        (r"\Ax(a|aa){2,5}\z", &[b"xaa", b"q"], &[false, false], false),
        (
            r"(?:a(?m:^)|\n\n){2,3}c",
            &[b"\n\n\n", b"\nc"],
            &[false, true],
            true,
        ),
        (r"(?:a(?m:^)|\n\n){2,3}c", &[b"a\n\nc"], &[false], false),
    ];
    let mut counted = 0;
    for &(pattern, chunks, after, finish) in rows {
        counted += usize::from(gate(pattern, chunks, after, finish));
    }
    assert_eq!(counted, rows.len(), "every row runs a counter when forced");
    // The empty stream.
    gate(r"\z\B", &[], &[], true);
    gate(r"\z\b", &[], &[], false);
}

// ---------------------------------------------------------------------------
// Every kind at every position
// ---------------------------------------------------------------------------

/// Positions of an assertion `{K}` around a repetition of two to four,
/// over word (`a`, `x`), non-word (`-`) and line-break bytes: before the
/// prefix, before the entry, before, between and after the bytes of a
/// body (the last is right before the increment), after the exit, between
/// two scopes, on a branch beside an unguarded bypass, and at the end.
const SINGLE: [&str; 10] = [
    "{K}x(?:a|-a){2,4}-",
    "[x\\n\\r-]{K}(?:a|-a){2,4}x",
    "x(?:{K}a|-){2,4}x",
    "x(?:a{K}a|-){2,4}x",
    "x(?:a{K}|aa|-\\n){2,4}x",
    "x(?:a|aa){2,4}{K}[x\\r\\n-]",
    "(?:a|aa){2,3}{K}(?:-|\\n-){2,3}x",
    "x(?:(?:a|aa){2,4}{K}|a*)[x\\n-]",
    "x(?:a|-|\\r){2,4}{K}",
    "x(?:a{K}|aa){2,4}(?:-{K}|\\n){2,3}x",
];

const SINGLE_ALPHABET: &[u8] = b"xa-\r\n";

#[test]
fn every_kind_at_every_position() {
    let short = all_strings(SINGLE_ALPHABET, 3);
    let mut rng = Rng(23);
    let (mut patterns, mut matches) = (0, 0);
    for position in SINGLE {
        for kind in KINDS {
            let pattern = position.replace("{K}", kind);
            let oracle = oracle(&pattern);
            let re = sparse_forced(&pattern);
            let plan = re
                .sparse_plan()
                .unwrap_or_else(|e| panic!("`{pattern}`: {e}"));
            assert!(plan.slots > 0, "`{pattern}` compiled without a counter");
            assert!(
                re.states.iter().any(|s| matches!(s, State::Assert { .. })),
                "`{pattern}` compiled without an assertion"
            );
            patterns += 1;
            let label = format!("`{pattern}`");
            let longer: Vec<Vec<u8>> = (0..40)
                .map(|_| {
                    (0..4 + rng.below(6))
                        .map(|_| SINGLE_ALPHABET[rng.below(SINGLE_ALPHABET.len())])
                        .collect()
                })
                .collect();
            for input in short.iter().chain(&longer) {
                let expected = oracle.is_match(input);
                matches += usize::from(expected);
                run_chunks(&re, &[input], expected, &label);
                let bytes: Vec<&[u8]> = input.chunks(1).collect();
                run_chunks(&re, &bytes, expected, &label);
                check_profiles(&re, input, &label);
            }
        }
    }
    assert_eq!(patterns, SINGLE.len() * KINDS.len());
    assert!(matches > 500, "{matches} matching inputs");
}

/// All ordered pairs of kinds (contradictions included) at the positions
/// of the exact grouped NFA's matrix, with finite counters, under the
/// default and the counters-forced configurations.
#[test]
fn assertion_pairs_at_every_position() {
    let inputs = all_strings(PAIR_ALPHABET, 4);
    let mut rng = Rng(7);
    let (mut admitted, mut rejected) = (0, 0);
    for (position, counter) in POSITIONS
        .iter()
        .flat_map(|p| ["{1,3}", "{2,5}"].map(|c| (p, c)))
    {
        for a in KINDS {
            for b in KINDS {
                let pattern = pair_pattern(position, &format!("{a}{b}"), counter);
                let oracle = oracle(&pattern);
                for (unroll, merge) in [CONFIGS[0], CONFIGS[2]] {
                    let re = sparse_compile(&pattern, unroll, merge);
                    if re.sparse_plan().is_err() {
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
                        check_profiles(&re, input, &label);
                    }
                }
            }
        }
    }
    // A silent drop in coverage must fail.
    assert_eq!(rejected, 0, "{admitted} admitted");
}

/// Short inputs over the pair alphabet (word and non-word bytes, CR, LF,
/// NUL and 0xFF), every partition and prefix, for one pattern per position
/// with a contradiction-free mixed pair: every input up to 3 bytes, and a
/// sample of 4 and 5 bytes.
#[test]
fn assertion_patterns_every_cut() {
    let mut rng = Rng(11);
    let longer: Vec<Vec<u8>> = (0..100)
        .map(|_| {
            (0..4 + rng.below(2))
                .map(|_| PAIR_ALPHABET[rng.below(PAIR_ALPHABET.len())])
                .collect()
        })
        .collect();
    for position in POSITIONS {
        for pair in [r"\b(?m:$)", r"(?mR:^)\B", r"\b{end}(?mR:$)", r"\z\B"] {
            let pattern = pair_pattern(position, pair, "{2,4}");
            let oracle = oracle(&pattern);
            let re = sparse_forced(&pattern);
            for input in all_strings(PAIR_ALPHABET, 3).iter().chain(&longer) {
                check_all_cuts(&re, &oracle, input, &format!("`{pattern}`"));
            }
        }
    }
}

/// Paths with different counts meet behind guards of their own, over
/// longer inputs with word breaks and line ends: the rows hold several
/// representatives when an assertion passes or blocks them.
#[test]
fn guarded_rows_with_several_counts() {
    let mut rng = Rng(0xa55e);
    let pieces: &[&[u8]] = &[
        b"a", b"a", b"aa", b"aaa", b" ", b"-", b"\n", b"\r\n", b"x", b"z",
    ];
    for pattern in [
        r"x(a\b|aa|a){20,40}z",
        r"x(a|aa|\b ){8,30}\bz",
        r"(?m:^)x(a|aa| ){10,60}(?m:$)",
        r"x(a|aa|-\b){3,12}\b-",
        r"\b{start}(a|aa){3,50}\b{end}",
        r"(?mR:^)(a|aa|x){4,9}(?mR:$)",
    ] {
        let oracle = oracle(pattern);
        let re = sparse_forced(pattern);
        assert!(re.sparse_plan().unwrap().slots > 0, "`{pattern}`");
        let (mut matches, mut several) = (0, false);
        for round in 0..60 {
            let mut input = Vec::new();
            if round % 2 == 0 {
                input.push(b'x');
            }
            while input.len() < 20 + round * 2 {
                let piece = if rng.below(4) == 0 {
                    pieces[rng.below(pieces.len())]
                } else {
                    pieces[rng.below(4)]
                };
                input.extend_from_slice(piece);
            }
            input.extend_from_slice([b"z".as_slice(), b"\n", b" ", b"-", b""][round % 5]);
            let expected = oracle.is_match(&input);
            matches += usize::from(expected);
            let label = format!("`{pattern}` round {round}");
            let parts: Vec<&[u8]> = input.chunks(1 + rng.below(7)).collect();
            run_chunks(&re, &parts, expected, &label);
            check_profiles(&re, &input, &label);
            // Some row holds more than one representative on the way.
            let mut scratch = SparseScratch::new();
            let mut s = SparseStream::new(&re, &mut scratch).unwrap();
            for b in input.chunks(1) {
                s.chunk(&re, &mut scratch, b);
                several |= (0..re.states.len() as u32).any(|i| {
                    s.raw_counts(&re, crate::StateIdx(i))
                        .is_some_and(|c| c.len() > 1)
                });
            }
        }
        assert!(
            matches > 0 && matches < 60,
            "`{pattern}`: {matches} matches"
        );
        assert!(several, "`{pattern}`: no row ever held two representatives");
    }
}

// ---------------------------------------------------------------------------
// Streams and boundary metadata
// ---------------------------------------------------------------------------

/// Streams at different previous-byte categories, interleaved through one
/// scratch, each resolve their own boundary.
#[test]
fn interleaved_streams_resolve_their_own_boundaries() {
    let pattern = r"\b(a|aa){2,5}\b";
    let re = sparse_forced(pattern);
    let oracle = oracle(pattern);
    let inputs: [&[u8]; 5] = [b"aaa ", b"aaaaaaa", b"-aa\n", b"\raa-a", b"xaa"];
    let mut scratch = SparseScratch::new();
    let mut streams: Vec<SparseStream> = inputs
        .iter()
        .map(|_| SparseStream::new(&re, &mut scratch).unwrap())
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
/// raw frontier and the boundary metadata alone; only consumption replaces
/// them.  A speculative continuation stored back would count twice.
#[test]
fn provisional_closure_does_not_touch_raw_rows() {
    let re = sparse_forced(r"\Ax(a|aa){2,3}\b");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    let fresh = format!("{}", s.display(&re));
    assert!(fresh.ends_with("raw={}"), "{fresh}");
    assert_eq!(s.boundary_state(), (None, true));
    s.chunk(&re, &mut scratch, b"");
    assert_eq!(format!("{}", s.display(&re)), fresh);
    assert_eq!(s.boundary_state(), (None, true));

    s.chunk(&re, &mut scratch, b"xaa");
    let raw = format!("{}", s.display(&re));
    assert_eq!(s.boundary_state(), (Some(b'a'), false));
    for _ in 0..3 {
        s.chunk(&re, &mut scratch, b"");
        assert!(!s.ismatch());
        assert_eq!(format!("{}", s.display(&re)), raw);
        assert_eq!(s.boundary_state(), (Some(b'a'), false));
    }
    // `\b` defers before the next byte; a non-word byte resolves it.
    s.chunk(&re, &mut scratch, b"!");
    assert!(s.ismatch());
}

/// A match that only end of input decides: `finish` replays the raw
/// frontier with the end known, without a sentinel byte; a second stream
/// given one more byte does not match.
#[test]
fn end_of_input_is_replayed_at_finish() {
    for (pattern, input, more) in [
        (r"x(a|aa){2,5}\z", b"xaaa".as_slice(), b"q".as_slice()),
        (r"x(a|aa){2,5}$", b"xaaa", b"a"),
        (r"x(a|aa){2,5}\b", b"xaaa", b"a"),
        (r"x(a|aa){2,5}(?mR:$)", b"xaaa", b"a"),
        (r"x(a|-){2,5}\B", b"xa-", b"a"),
    ] {
        let re = sparse_forced(pattern);
        let mut scratch = SparseScratch::new();
        let mut s = SparseStream::new(&re, &mut scratch).unwrap();
        s.chunk(&re, &mut scratch, input);
        assert!(!s.ismatch(), "`{pattern}`: decided before the end");
        assert!(s.finish(&re, &mut scratch), "`{pattern}`: at end of input");

        s.reset(&re, &mut scratch);
        s.chunk(&re, &mut scratch, input);
        s.chunk(&re, &mut scratch, more);
        let mut whole = input.to_vec();
        whole.extend_from_slice(more);
        assert_eq!(
            s.finish(&re, &mut scratch),
            oracle(pattern).is_match(&whole),
            "`{pattern}` with {more:?}"
        );
    }
}

/// A reset clears the boundary: the start assertion holds again.
#[test]
fn reset_and_rebind_clear_the_boundary() {
    let re = sparse_forced(r"\Ax(a|aa){2,3}\z");
    let other = sparse_forced(r"(?m:^)a{2,4}(?m:$)");
    let mut scratch = SparseScratch::new();
    let mut s = SparseStream::new(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, b"qxaa");
    assert!(!s.finish(&re, &mut scratch));
    s.reset(&re, &mut scratch);
    assert_eq!(s.boundary_state(), (None, true));
    s.chunk(&re, &mut scratch, b"xaa");
    assert!(s.finish(&re, &mut scratch));
    s.rebind(&other, &mut scratch).unwrap();
    assert_eq!(s.boundary_state(), (None, true));
    s.chunk(&other, &mut scratch, b"aaa");
    assert!(s.finish(&other, &mut scratch));
    s.rebind(&re, &mut scratch).unwrap();
    s.chunk(&re, &mut scratch, b"xaaa");
    assert!(s.finish(&re, &mut scratch));
    let _ = AssertKind::ALL;
}
