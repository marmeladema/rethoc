//! Tier 2 can share a break decision among overlapping counters when every
//! counter exits to the same terminal action. Every nonempty subset of breaks
//! then adds exactly that action to the ordinary continue closure. No exit
//! can seed a counter, add a body state, or change a reset or phase advance.
//! This property holds for every transition, including consuming exit edges
//! and byte-table dispatches, without simulating any transitions at build time.

use crate::{AssertKind, State};

/// Check the shared break condition for an otherwise eligible flat graph.
/// With multiple counts firing at once the runtime's static increment mask
/// must be exact even if an increment of a different counter succeeds.
/// An assertion inside a body could gate one of these increments.
pub(super) fn common_terminal_break(states: &[State], counter_body_states: &[bool]) -> bool {
    debug_assert_eq!(states.len(), counter_body_states.len());
    let mut terminal = None;
    for (state, &in_body) in states.iter().zip(counter_body_states) {
        match *state {
            State::Assert { .. } if in_body => return false,
            State::CounterIncrement { out1, .. } => {
                if let Some(known) = terminal {
                    if out1 != known {
                        return false;
                    }
                } else {
                    match states[out1.idx()] {
                        State::Match => {}
                        State::Assert {
                            kind: AssertKind::End,
                            out,
                        } if matches!(states[out.idx()], State::Match) => {}
                        _ => return false,
                    }
                    terminal = Some(out1);
                }
            }
            _ => {}
        }
    }
    terminal.is_some()
}

#[cfg(test)]
mod tests {
    use crate::Engine;

    #[test]
    fn overlapping_counter_eligibility() {
        let mut mem = crate::MatcherMemory::default();
        for (pattern, admitted) in [
            (r"^([ab]{5,26}|[bc]{5,26})$", true),
            (r"([ab]{2,3}|(?:b[bc]){2,4})", true),
            (r"^([ab]{4}$|[bc]{2}$x)", false),
            (r"^([ab]{4}|[bc]{2}$)", false),
            (r"^([ab]{4}x|[bc]{2}y)", false),
            (r"^(a{2,4}|(?:a(?m:^)|b){2,4})$", false),
            (r"^\w{3}\d{2}$", false),
            (r"^[0-9A-F]{4}[A-F]{2}$", false),
            (r"^.{0,2}.{0,2}$", false),
            (r"^(f{5,26}|f{5,20})$", false),
        ] {
            let regex = build_checked(pattern).expect("valid overlap test pattern");
            assert_eq!(regex.engine() == Engine::Tier2, admitted, "{pattern}");
            assert_eq!(
                mem.matcher_for_tier(&regex, 2).is_ok(),
                admitted,
                "{pattern}"
            );
        }
    }

    /// Generate flat branches with a common terminal action. Each byte of a
    /// body accepts b, and one position also accepts a byte private to that
    /// branch. This gives us simultaneous increments and branch-specific exits.
    fn overlap_case(
        rng: &mut proptest::test_runner::TestRng,
        num_counters: usize,
        prefix: &str,
        terminal: &str,
    ) -> (String, Vec<String>) {
        use proptest::prelude::RngExt;

        let mut branches = Vec::new();
        let mut inputs = vec![String::new()];
        let mut longest = 0;
        let mut witness = String::new();
        for letter in ['a', 'c', 'd'].into_iter().take(num_counters) {
            let width = rng.random_range(1..=3);
            let pos = rng.random_range(0..width);
            let lead = "b".repeat(pos);
            let tail = "b".repeat(width - pos - 1);
            let min = rng.random_range(2..=5);
            let max = rng.random_range(min..=min + 4);
            branches.push(format!("(?:{lead}[b{letter}]{tail}){{{min},{max}}}"));
            longest = longest.max(max * width);

            let body = format!("{lead}{letter}{tail}");
            for count in [min - 1, min, max, max + 1] {
                inputs.push(format!("x{}", body.repeat(count)));
            }
            witness = body.repeat(min);
        }
        for len in 0..=longest + 1 {
            inputs.push(format!("x{}", "b".repeat(len)));
        }
        // Expire all counters, then start a fresh attempt at a line boundary.
        inputs.push(format!("x{}!\nx{witness}", "b".repeat(longest + 1)));
        let mut mixed = String::from("x");
        for _ in 0..=longest {
            let bytes = b"bbbbacd!x";
            mixed.push(char::from(bytes[rng.random_range(0..bytes.len())]));
        }
        inputs.push(mixed);
        (
            format!("{prefix}(?:{}){terminal}", branches.join("|")),
            inputs,
        )
    }

    fn check_generated_overlaps(patterns: usize) {
        use proptest::test_runner::{RngAlgorithm, TestRng};

        let mut rng = TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let mut admitted = 0;
        let mut saw_mixed_widths = false;
        let mut comparisons = 0;
        for case in 0..patterns {
            let num_counters = 2 + case % 2;
            let prefix = ["x", "^x", r"\bx", "(?m:^x)"][case / 2 % 4];
            let terminal = ["", "$"][case / 8 % 2];
            let (pattern, inputs) = overlap_case(&mut rng, num_counters, prefix, terminal);
            let oracle = regex::bytes::Regex::new(&pattern).expect("valid generated overlap");
            for byte_classes in [usize::MAX, 0] {
                let re = crate::Regex::with_config(
                    &pattern,
                    crate::RegexConfig {
                        max_unroll_states: 0,
                        merge_repetitions: false,
                        byte_class_min_states: byte_classes,
                        ..Default::default()
                    },
                )
                .expect("small generated overlap compiles");
                assert_eq!(re.counter_info.len(), num_counters, "{pattern}");
                assert!(re.tier2_overlap_proven, "{pattern}");
                assert_eq!(re.engine(), Engine::Tier2, "{pattern}");
                admitted += 1;
                saw_mixed_widths |= re
                    .counter_info
                    .windows(2)
                    .any(|pair| pair[0].body_byte_length != pair[1].body_byte_length);

                let mut mem = crate::MatcherMemory::default();
                if byte_classes == 0 {
                    mem.set_dfa_state_cap(Some(crate::dfa::DFA_MIN_STATES));
                }
                for input in &inputs {
                    let expected = oracle.is_match(input.as_bytes());
                    for tier in [0, 2] {
                        for chunk_size in [input.len().max(1), 1] {
                            let mut m = mem.matcher_for_tier(&re, tier).expect("eligible tier");
                            for chunk in input.as_bytes().chunks(chunk_size) {
                                m.chunk(chunk);
                            }
                            comparisons += 1;
                            assert_eq!(
                                m.finish(),
                                expected,
                                "{pattern} on {input:?}, tier {tier}, chunk {chunk_size}, class threshold {byte_classes}"
                            );
                        }
                    }
                }
            }
        }
        eprintln!(
            "{patterns} patterns, {admitted} admitted configurations, {comparisons} oracle comparisons"
        );
        assert!(
            admitted > 0,
            "must exercise admitted multi-counter patterns"
        );
        assert!(
            saw_mixed_widths,
            "must exercise overlapping counters of different widths"
        );
    }

    #[test]
    fn generated_overlap_smoke() {
        check_generated_overlaps(16);
    }

    #[test]
    #[ignore] // Broader deterministic coverage of counter bounds and body widths.
    fn test_fuzz_tier2_overlap() {
        check_generated_overlaps(500);
    }

    // -- Exhaustive adversarial mini-model tests (Patch 8) --

    /// Enumerate two-counter overlap patterns over a tiny alphabet,
    /// verify that any pattern admitted by the proof matches correctly
    /// against Tier 0 (NFA).
    #[test]
    #[ignore] // slow: ~30 seconds
    fn test_exhaustive_overlap_mini_model() {
        // Alphabet subsets to combine.
        let sets: &[(&str, &str)] = &[
            ("[ab]", "ab"),
            ("[bc]", "bc"),
            ("[abc]", "abc"),
            ("[a0]", "a0"),
            ("[01]", "01"),
        ];
        let bounds: &[(usize, usize)] = &[(1, 2), (2, 3), (1, 3)];
        let max_input_len = 4;

        let mut tested = 0;
        let mut admitted = 0;

        for &(s1, alpha1) in sets {
            for &(s2, alpha2) in sets {
                for &(m1, n1) in bounds {
                    for &(m2, n2) in bounds {
                        let pattern = format!("^({s1}{{{m1},{n1}}}|{s2}{{{m2},{n2}}})$");
                        let regex = match build_checked(&pattern) {
                            Some(r) => r,
                            None => continue,
                        };
                        tested += 1;

                        if !regex.tier2_overlap_proven {
                            continue;
                        }

                        // Pattern admitted — verify Tier 2 vs Tier 0.
                        admitted += 1;

                        // Build input alphabet.
                        let mut alphabet: Vec<u8> = Vec::new();
                        for c in alpha1.bytes().chain(alpha2.bytes()) {
                            if !alphabet.contains(&c) {
                                alphabet.push(c);
                            }
                        }
                        alphabet.push(b'x'); // non-matching char

                        // Enumerate all inputs up to max_input_len.
                        let mut input = Vec::new();
                        check_inputs(&regex, &alphabet, max_input_len, &mut input, &pattern);
                    }
                }
            }
        }

        eprintln!("exhaustive overlap mini-model: tested {tested} patterns, {admitted} admitted");
        assert!(admitted > 0, "should exercise overlapping counters");
    }

    fn build_checked(pattern: &str) -> Option<crate::Regex> {
        crate::Regex::with_config(
            pattern,
            crate::RegexConfig {
                max_unroll_states: 0,
                merge_repetitions: false,
                ..Default::default()
            },
        )
        .ok()
    }

    fn check_inputs(
        regex: &crate::Regex,
        alphabet: &[u8],
        max_len: usize,
        input: &mut Vec<u8>,
        pattern: &str,
    ) {
        // Check current input.
        let mut mem = crate::MatcherMemory::default();

        // Tier 0 (NFA oracle).
        let mut m0 = mem.matcher_for_tier(regex, 0).unwrap();
        m0.chunk(input);
        let nfa = m0.finish();

        // Tier 2 (forced).
        let mut m2 = mem.matcher_for_tier(regex, 2).expect("admitted overlap");
        m2.chunk(input);
        assert_eq!(nfa, m2.finish(), "{pattern} on {input:?}");

        // Recurse.
        if input.len() < max_len {
            for &b in alphabet {
                input.push(b);
                check_inputs(regex, alphabet, max_len, input, pattern);
                input.pop();
            }
        }
    }
}
