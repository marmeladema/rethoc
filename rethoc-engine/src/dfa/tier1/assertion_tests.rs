//! Generated assertion checks across streaming boundaries and cache lifetimes.

use super::*;
use crate::{MatcherMemory, RegexConfig};

const ASSERTIONS: [&str; 10] = [
    "^",
    "$",
    "(?m:^)",
    "(?m:$)",
    "(?Rm:^)",
    "(?Rm:$)",
    r"\b",
    r"\B",
    r"\b{start}",
    r"\b{end}",
];
const ALPHABET: [u8; 5] = [b'a', b' ', b'\r', b'\n', 0xff];

/// The table follows the compiled graph. Either eligible DFA can request it.
#[test]
fn assertion_table_tracks_unrolling_and_retained_counters() {
    for max_unroll_states in [32, 0] {
        for (pattern, eligible, empty) in [
            ("", true, true),
            ("abc", true, true),
            (r"\babc\b", true, false),
            (r"\ba{2}", max_unroll_states > 0, false),
            ("(?Rm)^a{2}$", max_unroll_states > 0, max_unroll_states == 0),
        ] {
            let regex = Regex::with_config(
                pattern,
                RegexConfig {
                    max_unroll_states,
                    ..Default::default()
                },
            )
            .expect("analysis representation fixture must compile");
            assert_eq!(regex.dfa_eligible, eligible, "{pattern}");
            assert_eq!(regex.dfa_assertions.is_empty(), empty, "{pattern}");
            assert_eq!(
                regex.dfa_assertions.memory_size(),
                if empty { 0 } else { 2 * regex.states.len() },
                "{pattern}",
            );
        }
    }
}

/// All length-zero-to-two inputs, plus longer line-boundary combinations.
fn boundary_inputs() -> Vec<Vec<u8>> {
    let mut inputs = vec![Vec::new()];
    for first in ALPHABET {
        inputs.push(vec![first]);
        for second in ALPHABET {
            inputs.push(vec![first, second]);
        }
    }
    inputs.extend(
        [
            &b"\r\na"[..],
            b"a\r\n",
            b"\na\r",
            b"\ra\n",
            b"a\r\r",
            b"\r\n\r\n",
            b"\r\na\n",
            b"a\n\ra",
            b"\n a",
            b"\xff\r\n",
        ]
        .map(<[u8]>::to_vec),
    );
    inputs
}

/// Feed one partition, including empty chunks around every nonempty chunk.
fn partition_answer(
    memory: &mut MatcherMemory,
    regex: &Regex,
    input: &[u8],
    cuts: usize,
    tier: u8,
) -> bool {
    let mut matcher = memory
        .matcher_for_tier(regex, tier)
        .expect("generated counter-free pattern must be admitted");
    matcher.chunk(&[]);
    let mut start = 0;
    for end in 1..=input.len() {
        if end == input.len() || cuts & (1 << (end - 1)) != 0 {
            matcher.chunk(&input[start..end]);
            matcher.chunk(&[]);
            start = end;
        }
    }
    matcher.chunk(&[]);
    matcher.finish()
}

/// All ordered pairs exercise next-byte and EOF resolution, joins and cycles.
/// Caches stay alive across both input changes and different compiled regexes.
#[test]
fn assertion_pairs_all_partitions_and_reused_memory() {
    let mut inputs = boundary_inputs();
    for byte_class_min_states in [usize::MAX, 0] {
        for cap in [super::super::DFA_MAX_STATES, super::super::DFA_MIN_STATES] {
            let mut tier_memory = MatcherMemory::default();
            tier_memory.set_dfa_state_cap(Some(cap));
            let mut nfa_memory = MatcherMemory::default();
            for first in ASSERTIONS {
                for second in ASSERTIONS {
                    for pattern in [
                        format!("{first}{second}a"),
                        format!(".{first}{second}"),
                        format!(r"(?:{first}{second}a|\r{second}{first})"),
                        // A consuming alternative prevents normalization of
                        // an assertion-only star to an optional assertion.
                        format!("(?:{first}{second}|a)*.{second}{first}"),
                    ] {
                        let regex = Regex::with_config(
                            &pattern,
                            RegexConfig {
                                byte_class_min_states,
                                ..Default::default()
                            },
                        )
                        .expect("generated assertion pair must compile");
                        let reference = regex::bytes::Regex::new(&format!("(?s-u){pattern}"))
                            .expect("reference must accept generated assertion pair");
                        assert!(regex.dfa_eligible, "Tier 1 rejected {pattern}");
                        for input in &inputs {
                            let expected = reference.is_match(input);
                            let partitions = 1 << input.len().saturating_sub(1);
                            for cuts in 0..partitions {
                                for (tier, memory) in [(0, &mut nfa_memory), (1, &mut tier_memory)]
                                {
                                    assert_eq!(
                                        partition_answer(memory, &regex, input, cuts, tier),
                                        expected,
                                        "pattern={pattern} input={input:?} cuts={cuts} \
                                         tier={tier} cap={cap} classes={byte_class_min_states}"
                                    );
                                }
                            }
                        }
                        // Exercise preceding-input cache history in both orders.
                        inputs.reverse();
                    }
                }
            }
        }
    }
}

/// Keep the cycle generator honest: a pure assertion star is normalized to
/// optional by regex-syntax, but this generated consuming alternative must
/// retain an actual reachable Assert/Split cycle in the compiled graph.
#[test]
fn generated_loop_retains_a_compiled_epsilon_cycle() {
    let first = r"\b";
    let second = "(?m:^)";
    let pattern = format!("(?:{first}{second}|a)*.{second}{first}");
    let regex = Regex::new(&pattern).expect("cycle fixture must compile");
    let mut color = vec![0; regex.states.len()];
    let mut stack = vec![(regex.start, false)];
    let mut found_cycle = false;
    while let Some((state, leaving)) = stack.pop() {
        if leaving {
            color[state.idx()] = 2;
            continue;
        }
        match color[state.idx()] {
            1 => {
                found_cycle = true;
                break;
            }
            2 => continue,
            _ => {}
        }
        color[state.idx()] = 1;
        stack.push((state, true));
        match regex.states[state] {
            State::Assert { out, .. } => stack.push((out, false)),
            State::Split { out, out1 } => {
                stack.push((out1, false));
                stack.push((out, false));
            }
            _ => {}
        }
    }
    assert!(
        found_cycle,
        "generated loop lost its epsilon cycle: {pattern}"
    );
}

/// The context key must distinguish only properties the parked tail can read.
/// In particular, EndLF must not split by previous CR/LF, and ordinary word
/// assertions must retain precisely the previous wordness dimension.
#[test]
fn assertion_context_keys_keep_only_required_properties() {
    for byte_class_min_states in [usize::MAX, 0] {
        for (pattern, required) in [
            (".a", 0),
            (".(?m:$)a", 0),
            (".$", 0),
            (r".\ba", crate::ByteProps::WORD.0),
            (r".(?Rm:$)\n", crate::ByteProps::CR.0),
        ] {
            let regex = Regex::with_config(
                pattern,
                RegexConfig {
                    byte_class_min_states,
                    ..Default::default()
                },
            )
            .expect("context fixture must compile");
            if pattern == ".a" {
                assert!(regex.dfa_assertions.is_empty());
            }
            let mut memory = DfaMemory::default();
            let mut cache = Tier1DfaCache::new();
            cache.prepare(&mut memory, &regex);
            let start = cache.inner.start_id;
            let mut representatives = [None; 8];
            for byte in 0..=255 {
                let current = cache.transition(&mut memory, start, byte, &regex);
                assert_ne!(current, DfaStateId::DEAD);
                let state = &cache.inner.states[current.idx()];
                let expected = crate::ByteProps::of(byte).0 & required;
                assert_eq!(state.context.0, expected, "{pattern} byte={byte}");
                // All these patterns have the same post-consumption frontier;
                // no other property may manufacture a distinct cached state.
                let representative = &mut representatives[expected as usize];
                if let Some(previous) = representative {
                    assert_eq!(*previous, current, "{pattern} byte={byte}");
                } else {
                    *representative = Some(current);
                }
            }
            for state in &cache.inner.states {
                if state.deferred_asserts.is_empty() {
                    assert_eq!(state.context.0, 0, "{pattern}");
                }
            }
        }
    }
}

/// Evict exactly between parking and resolution. This exercises real ID
/// remapping, retained context/EOF flags, and transition rebuilding, which
/// the Python design model's tuple-key cache cannot cover.
#[test]
fn eviction_preserves_pending_assertion_context_and_eof() {
    let mut pending_states_checked = 0;
    for assertion in ASSERTIONS {
        let pattern = format!(".{assertion}a?");
        let regex = Regex::new(&pattern).expect("eviction fixture must compile");
        let reference = regex::bytes::Regex::new(&format!("(?s-u){pattern}"))
            .expect("reference must accept eviction fixture");
        for previous in ALPHABET {
            let mut memory = DfaMemory::default();
            let mut cache = Tier1DfaCache::new();
            cache.set_max_states(super::super::DFA_MIN_STATES);
            cache.prepare(&mut memory, &regex);
            let current = cache.transition(&mut memory, cache.inner.start_id, previous, &regex);
            if current == DfaStateId::DEAD {
                continue;
            }
            let saved = cache.inner.states[current.idx()].clone();
            if saved.deferred_asserts.is_empty() {
                continue;
            }
            pending_states_checked += 1;
            let remapped = cache.evict(current);
            assert_eq!(cache.inner.states[remapped.idx()], saved);
            assert!(
                cache
                    .transitions
                    .iter()
                    .all(|&next| next == super::Tier1StateId::UNKNOWN)
            );
            // The current EOF answer must survive eviction without seeing a
            // next byte; each next-byte answer then rebuilds one transition.
            for next in std::iter::once(None).chain(ALPHABET.map(Some)) {
                let remapped = cache.tagged(remapped).current();
                let mut matcher = DfaMatcher::new(&mut cache, &mut memory, &regex);
                matcher.current = remapped;
                matcher.ever_matched = saved.is_match;
                let mut input = vec![previous];
                matcher.chunk(&[]);
                if let Some(byte) = next {
                    input.push(byte);
                    matcher.chunk(&[byte]);
                }
                matcher.chunk(&[]);
                assert_eq!(
                    matcher.finish(),
                    reference.is_match(&input),
                    "{pattern} input={input:?}"
                );
            }
        }
    }
    assert!(
        pending_states_checked > 0,
        "must actually evict parked assertions"
    );
}

/// DEAD transitions can point at live context-bearing states. Their IDs must
/// not survive eviction or be reused after the cache switches regexes.
#[test]
fn dead_transition_ids_clear_on_eviction_and_regex_change() {
    for byte_class_min_states in [usize::MAX, 0] {
        let compile = |pattern| {
            Regex::with_config(
                pattern,
                RegexConfig {
                    byte_class_min_states,
                    ..Default::default()
                },
            )
            .expect("dead-transition fixture must compile")
        };
        let crlf = compile("(?Rm:^)a");
        let lf = compile("(?m:^)b");
        let mut memory = DfaMemory::default();
        let mut cache = Tier1DfaCache::new();
        cache.prepare(&mut memory, &crlf);
        assert!(cache.dead_transitions.is_empty());
        let dead = cache.transition(&mut memory, cache.inner.start_id, b'x', &crlf);
        assert_eq!(dead, DfaStateId::DEAD);
        let pending = cache.transition(&mut memory, dead, b'\r', &crlf);
        assert_ne!(pending, DfaStateId::DEAD);
        assert_ne!(pending, cache.inner.start_id);
        let saved = cache.inner.states[pending.idx()].clone();
        assert!(!saved.deferred_asserts.is_empty());
        assert!(cache.dead_transitions.contains(&cache.tagged(pending)));

        let remapped = cache.evict(pending);
        assert_eq!(cache.inner.states[remapped.idx()], saved);
        assert!(cache.dead_transitions.is_empty());
        assert_eq!(
            cache.transition(&mut memory, DfaStateId::DEAD, b'\r', &crlf),
            remapped
        );
        assert!(cache.dead_transitions.contains(&cache.tagged(remapped)));

        cache.prepare(&mut memory, &lf);
        assert!(cache.dead_transitions.is_empty());
        let restarted = cache.transition(&mut memory, DfaStateId::DEAD, b'\n', &lf);
        assert_ne!(restarted, DfaStateId::DEAD);
        let matched = cache.transition(&mut memory, restarted, b'b', &lf);
        assert_ne!(matched, DfaStateId::DEAD);
        assert!(cache.inner.states[matched.idx()].is_match);
        cache.prepare(&mut memory, &crlf);
        assert!(cache.dead_transitions.is_empty());
    }
}

/// Use a genuine working set larger than the minimum cache rather than only
/// setting a small cap on patterns whose entire DFA fits within it.
#[test]
fn minimum_cache_actually_evicts_during_streaming() {
    let mut input = b"aaaaaabaaaabbaababababbbabbbbbb".repeat(64);
    let reference = regex::bytes::Regex::new("[ab]*a[ab]{5}z")
        .expect("reference must accept cache-churn fixture");
    for byte_class_min_states in [usize::MAX, 0] {
        let regex = Regex::with_config(
            "[ab]*a[ab]{5}z",
            RegexConfig {
                byte_class_min_states,
                ..Default::default()
            },
        )
        .expect("cache-churn fixture must compile");
        assert!(regex.dfa_eligible);
        let mut memory = DfaMemory::default();
        let mut cache = Tier1DfaCache::new();
        cache.set_max_states(super::super::DFA_MIN_STATES);
        cache.prepare(&mut memory, &regex);
        for chunk_size in [1, 17, input.len()] {
            let mut matcher = DfaMatcher::new(&mut cache, &mut memory, &regex);
            for chunk in input.chunks(chunk_size) {
                matcher.chunk(&[]);
                matcher.chunk(chunk);
            }
            assert_eq!(matcher.finish(), reference.is_match(&input));
        }
        assert!(cache.evictions() > 0, "fixture must actually evict states");
        input.extend_from_slice(b"aaaaaaz");
        let mut matcher = DfaMatcher::new(&mut cache, &mut memory, &regex);
        matcher.chunk(&input);
        assert_eq!(matcher.finish(), reference.is_match(&input));
        input.truncate(input.len() - 7);
    }
}
