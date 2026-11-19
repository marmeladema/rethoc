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

    // -- Overlap-focused property test (Patch 9) --

    /// Generate random overlapping two-counter patterns and verify
    /// that any admitted by the proof match correctly against Tier 0.
    #[test]
    #[ignore] // slow: uses random patterns
    fn test_fuzz_tier2_overlap() {
        use crate::fuzz_gen::{FuzzRng, generate_pattern};

        let mut admitted = 0;
        let mut tested = 0;

        // Use a deterministic seed sequence.
        for seed in 0u64..500 {
            let seed_bytes = seed.to_le_bytes();
            let mut rng = FuzzRng::new(&seed_bytes);
            let (pattern_str, ast) = generate_pattern(&mut rng);

            let regex = match build_checked(&pattern_str) {
                Some(r) => r,
                None => continue,
            };

            if regex.counter_info.len() < 2 {
                continue; // only test multi-counter patterns
            }
            tested += 1;

            if !regex.tier2_overlap_proven {
                continue;
            }

            // Admitted — test on boundary inputs.
            admitted += 1;
            let mut mem = crate::MatcherMemory::default();

            // Generate some boundary inputs.
            let inputs = crate::fuzz_gen::generate_inputs(&mut FuzzRng::new(&seed_bytes), &ast);
            let inputs2 = crate::fuzz_gen::generate_inputs(
                &mut FuzzRng::new(&(seed + 1000).to_le_bytes()),
                &ast,
            );

            for input in inputs.iter().chain(inputs2.iter()) {
                let mut m0 = mem.matcher_for_tier(&regex, 0).unwrap();
                m0.chunk(input);
                let nfa = m0.finish();

                let mut m2 = mem.matcher_for_tier(&regex, 2).expect("admitted overlap");
                m2.chunk(input);
                assert_eq!(nfa, m2.finish(), "{pattern_str} on {input:?}");
            }
        }

        eprintln!(
            "fuzz tier2 overlap: {tested} multi-counter patterns tested, {admitted} admitted"
        );
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
