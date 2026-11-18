//! Automatic selection (plan P5): every branch of the order Tier 1,
//! Tier 2, grouped NFA, Tier 4, NFA, on the compiled eligibility; the
//! configuration modes; forced selection; diagnostics; and memory reuse
//! across regexes and engines.

use super::tests::Rng;
use crate::{
    AnyMatcher, Engine, GroupedNfaError, GroupedNfaMode, MatcherMemory, Regex, RegexConfig,
};

fn build(pattern: &str, unroll: usize, mode: GroupedNfaMode) -> Regex {
    Regex::with_config(
        pattern,
        RegexConfig {
            max_unroll_states: unroll,
            merge_repetitions: unroll != 0,
            grouped_nfa: mode,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| panic!("`{pattern}`: {e}"))
}

/// Tier eligibility as compiled: (1, 2, 3, 4).
fn tiers(re: &Regex) -> [bool; 4] {
    [
        re.dfa_eligible,
        re.tier2_eligible,
        re.tier3_eligible,
        re.tier4_eligible,
    ]
}

fn admitted(re: &Regex) -> Option<bool> {
    re.grouped_plan_or_rejection().map(|g| g.is_ok())
}

fn oracle(pattern: &str) -> regex::bytes::Regex {
    regex::bytes::Regex::new(&format!("(?s-u){pattern}")).unwrap()
}

/// Every branch of the selection order, each checked against the
/// eligibility the compiler actually computed.
#[test]
fn every_selection_branch() {
    use GroupedNfaMode::Auto;
    // Tier 1; no grouped plan is built for it.
    let re = build("abc|abd", 32, Auto);
    assert!(tiers(&re)[0]);
    assert_eq!(re.engine(), Engine::Tier1);
    assert_eq!(admitted(&re), None);
    // Tier 2; no plan either.
    let re = build("^.*a.{3}bc$", 0, Auto);
    assert_eq!(tiers(&re)[..2], [false, true]);
    assert_eq!(re.engine(), Engine::Tier2);
    assert_eq!(admitted(&re), None);
    // The grouped NFA over eligible Tier 3 and Tier 4.
    let re = build("(?:a|bb){2,3}c", 0, Auto);
    assert_eq!(tiers(&re), [false, false, true, true]);
    assert_eq!(re.min_tier(), 3);
    assert_eq!(re.engine(), Engine::GroupedNfa);
    // The grouped NFA where the NFA was the only path.
    let re = build(r"x(a|aa){2,5}\bz?", 0, Auto);
    assert_eq!(tiers(&re), [false; 4]);
    assert_eq!(re.engine(), Engine::GroupedNfa);
    // Counter-free CRLF now goes to Tier 1, without a grouped plan.
    let re = build(r"(?Rm)^abc$", 32, Auto);
    assert_eq!(tiers(&re), [true, false, false, false]);
    assert_eq!(re.engine(), Engine::Tier1);
    assert_eq!(admitted(&re), None);
    // CRLF with retained counters still uses the grouped fallback.
    let re = build(r"(?Rm)^a{2,3}$", 0, Auto);
    assert_eq!(tiers(&re), [false; 4]);
    assert_eq!(re.engine(), Engine::GroupedNfa);
    // Grouped rejection (nested), then Tier 4.
    let re = build("^((a|bc){1,2}){2,3}$", 0, Auto);
    assert_eq!(tiers(&re), [false, false, false, true]);
    assert_eq!(admitted(&re), Some(false));
    assert_eq!(re.engine(), Engine::Tier4);
    // Grouped rejection (nested), then the NFA: an assertion that looks
    // ahead keeps Tier 4 out.
    let re = build(r"x((a|aa){2,3}\b){2,3}", 0, Auto);
    assert_eq!(tiers(&re), [false; 4]);
    assert_eq!(admitted(&re), Some(false));
    assert_eq!(re.engine(), Engine::Nfa);
}

/// Tier 3 is never selected, and the automatic matcher is never a Tier 3
/// matcher, whatever the configuration; a regex Tier 3 could run goes to
/// the grouped NFA, or to Tier 4 or the NFA when grouped is disabled.
#[test]
fn tier3_is_never_selected() {
    let patterns = [
        "(?:a|bb){2,3}c",
        "x(a|aa){2,5}z",
        "foo.{0,100}bar.{0,100}baz",
        "(?i)(select|union|insert).{0,40}(from|into).{0,40}(users|passwd)",
        r"(?:a\B|b){1,2}c",
        "^(a|bc){1,2}$",
    ];
    let mut memory = MatcherMemory::default();
    let mut tier3_seen = 0;
    for pattern in patterns {
        for unroll in [0, 32] {
            for mode in [
                GroupedNfaMode::Auto,
                GroupedNfaMode::Disabled,
                GroupedNfaMode::Always,
            ] {
                let re = build(pattern, unroll, mode);
                tier3_seen +=
                    usize::from(re.tier3_eligible && !re.dfa_eligible && !re.tier2_eligible);
                let m = memory.matcher(&re);
                assert!(
                    !matches!(m, AnyMatcher::Tier3Dfa(_)),
                    "`{pattern}` {mode:?}"
                );
                if mode == GroupedNfaMode::Disabled {
                    assert_ne!(re.engine(), Engine::GroupedNfa);
                }
                // Tier 3 stays available when asked for.
                if re.tier3_eligible {
                    assert!(matches!(
                        memory.matcher_for_tier(&re, 3),
                        Ok(AnyMatcher::Tier3Dfa(_))
                    ));
                }
            }
        }
    }
    assert!(
        tier3_seen > 0,
        "some pattern should be one Tier 3 used to run"
    );
}

#[test]
fn configuration_modes() {
    // Disabled: no analysis, Tier 4 instead, and grouped cannot be forced.
    let re = build("(?:a|bb){2,3}c", 0, GroupedNfaMode::Disabled);
    assert_eq!(admitted(&re), None);
    assert_eq!(re.engine(), Engine::Tier4);
    let mut memory = MatcherMemory::default();
    assert_eq!(
        memory.matcher_grouped(&re).err(),
        Some(GroupedNfaError::NotRequested)
    );
    // Auto with a Tier 1 winner: no plan, so not forceable either.
    let re = build("abc|abd", 32, GroupedNfaMode::Auto);
    assert_eq!(
        memory.matcher_grouped(&re).err(),
        Some(GroupedNfaError::NotRequested)
    );
    // Always: the plan is built, Tier 1 still selected, grouped forceable.
    let re = build("abc|abd", 32, GroupedNfaMode::Always);
    assert_eq!(admitted(&re), Some(true));
    assert_eq!(re.engine(), Engine::Tier1);
    let mut m = memory.matcher_grouped(&re).unwrap();
    assert!(matches!(m, AnyMatcher::Grouped(_)));
    m.chunk(b"xxabd");
    assert!(m.finish());
    // A rejection cannot be forced either: no silent fallback.
    let re = build("^((a|bc){1,2}){2,3}$", 0, GroupedNfaMode::Always);
    assert!(matches!(
        memory.matcher_grouped(&re).err(),
        Some(GroupedNfaError::Rejected(_))
    ));
    // The default is Auto.
    assert_eq!(RegexConfig::default().grouped_nfa, GroupedNfaMode::Auto);
}

/// Explicit Tier 0 runs the original NFA even where the grouped NFA is
/// selected, and both agree.
#[test]
fn forced_tier0_is_the_reference() {
    let pattern = r"x(a|aa){2,5}\bz?";
    let re = build(pattern, 0, GroupedNfaMode::Auto);
    assert_eq!(re.engine(), Engine::GroupedNfa);
    let mut memory = MatcherMemory::default();
    assert!(matches!(
        memory.matcher_for_tier(&re, 0),
        Ok(AnyMatcher::Nfa(_))
    ));
    assert!(matches!(memory.matcher(&re), AnyMatcher::Grouped(_)));
    let oracle = oracle(pattern);
    for input in [&b"xaaaa!"[..], b"xa", b"xaaaaaaaaaaaaz", b"qxaaz"] {
        let mut a = memory.matcher(&re);
        a.chunk(input);
        let auto = a.finish();
        let mut n = memory.matcher_for_tier(&re, 0).unwrap();
        n.chunk(input);
        assert_eq!(auto, n.finish(), "{input:?}");
        assert_eq!(auto, oracle.is_match(input), "{input:?}");
    }
}

#[test]
fn diagnostics_report_the_selected_engine() {
    let re = build("(?:a|bb){2,3}c", 0, GroupedNfaMode::Auto);
    let info = re.info();
    assert_eq!(info.execution.engine, "grouped");
    assert_eq!(info.execution.min_tier, 3);
    assert_eq!(
        info.execution.eligible,
        ["nfa", "tier3", "tier4", "grouped"]
    );
    let text = info.to_string();
    assert!(text.contains("Execution: Grouped NFA"), "{text}");
    assert!(
        text.contains("Eligible engines: nfa, tier3, tier4, grouped"),
        "{text}"
    );
    let dump = re.dump(false).to_string();
    assert!(dump.contains("Selected engine: grouped"), "{dump}");
    let re = build("^((a|bc){1,2}){2,3}$", 0, GroupedNfaMode::Auto);
    let info = re.info();
    assert_eq!(info.execution.engine, "tier4");
    assert!(info.to_string().contains("Grouped NFA: rejected"));
    let mut memory = MatcherMemory::default();
    let re = build("(?:a|bb){2,3}c", 0, GroupedNfaMode::Auto);
    let mut m = memory.matcher(&re);
    m.chunk(b"bba");
    assert!(m.to_string().starts_with("grouped"), "{m}");
    assert!(format!("{m:?}").starts_with("AnyMatcher::Grouped"));
}

/// One memory across regexes of several engines, interleaved; grouped
/// stream storage is reused, also when a smaller plan follows a larger one.
/// One DFA tier only among 1-3: a memory shared between regexes on two of
/// them panics in the shared DFA scratch (issue #5, open, not this code).
#[test]
fn memory_reuse_across_regexes_and_engines() {
    let patterns = [
        ("abc|abd", 32),
        ("x(a|aa|b){3,60}z(c|cc){1,60}y", 0),
        ("^((a|bc){1,2}){2,3}$", 0),
        (r"x((a|aa){2,3}\b){2,3}", 0),
        ("(?:a|bb){2,3}c", 0),
        (r"(?Rm)^ab{2,3}$", 0),
    ];
    let compiled: Vec<(Regex, regex::bytes::Regex)> = patterns
        .iter()
        .map(|&(p, u)| (build(p, u, GroupedNfaMode::Auto), oracle(p)))
        .collect();
    let engines: std::collections::BTreeSet<&str> =
        compiled.iter().map(|(r, _)| r.engine().name()).collect();
    assert!(engines.len() >= 4, "{engines:?}");
    let mut memory = MatcherMemory::default();
    let mut rng = Rng(31);
    let alphabet = b"abcxyz\r\n ";
    for _ in 0..400 {
        let (re, oracle) = &compiled[rng.below(compiled.len())];
        let input: Vec<u8> = (0..rng.below(40))
            .map(|_| alphabet[rng.below(alphabet.len())])
            .collect();
        let expected = oracle.is_match(&input);
        let mut m = memory.matcher(re);
        let mut i = 0;
        while i < input.len() {
            let n = rng.below(5);
            m.chunk(&input[i..(i + n).min(input.len())]);
            i += n;
        }
        assert_eq!(m.finish(), expected, "{input:?} on {}", re.engine());
    }
    // The large grouped plan, then a small one: the storage is kept.
    let (big, small) = (&compiled[1].0, &compiled[4].0);
    assert!(big.grouped_plan().unwrap().words() > small.grouped_plan().unwrap().words());
    drop(memory.matcher(big));
    let ptr = memory.grouped_stream.as_ref().unwrap().raw().as_ptr();
    drop(memory.matcher(small));
    assert_eq!(memory.grouped_stream.as_ref().unwrap().raw().as_ptr(), ptr);
}
