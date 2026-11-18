//! `rethoc sweep`: `brute` over a generated family of patterns.
//!
//! Each run is a child `rethoc brute` under a memory and time cap, so a
//! pattern that blows up is counted and the sweep goes on.
//! The `tier1-assert` family defaults to checking the NFA and Tier 1;
//! explicit backend selectors override that default.

use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

use bpaf::Bpaf;

use super::cap::{CapArgs, Verdict, cap_args, parallel, run_capped};
use super::{
    EXIT_ABORT, EXIT_MISMATCH, EXIT_OK, EXIT_REJECTED, TierChoice, regime_note, repro_env,
    shell_join,
};

/// A generated family of patterns.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Family {
    /// See [`nullable_counter`].
    NullableCounter,
    /// See [`tier3_assert`].
    Tier3Assert,
    /// See [`assert_edges`].
    AssertEdges,
    /// See [`assert_in_body`].
    AssertInBody,
    /// See [`tier1_assert`].
    Tier1Assert,
}

impl Family {
    /// Name on the command line.
    fn name(self) -> &'static str {
        match self {
            Self::NullableCounter => "nullable-counter",
            Self::Tier3Assert => "tier3-assert",
            Self::AssertEdges => "assert-edges",
            Self::AssertInBody => "assert-in-body",
            Self::Tier1Assert => "tier1-assert",
        }
    }
}

impl FromStr for Family {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nullable-counter" => Ok(Self::NullableCounter),
            "tier3-assert" => Ok(Self::Tier3Assert),
            "assert-edges" => Ok(Self::AssertEdges),
            "assert-in-body" => Ok(Self::AssertInBody),
            "tier1-assert" => Ok(Self::Tier1Assert),
            other => Err(format!(
                "unknown family `{other}` (expected nullable-counter, tier3-assert, \
                 assert-edges, assert-in-body or tier1-assert)"
            )),
        }
    }
}

/// Arguments of `rethoc sweep`.
#[derive(Bpaf, Clone, Debug)]
#[bpaf(ignore_rustdoc)]
pub(crate) struct SweepArgs {
    /// Print the `rethoc brute` command of every run instead of running them
    #[bpaf(long)]
    pub(crate) list: bool,
    /// Check only the NFA and this tier; patterns not eligible for it
    /// count as rejected. Defaults to 1 for tier1-assert unless another
    /// backend selector is given
    #[bpaf(long, argument("TIER"))]
    pub(crate) tier: Option<u8>,
    /// Check every eligible tier but this one (Tier 3 is known to be
    /// unsound: `--skip-tier 3` checks the others)
    #[bpaf(long, argument("TIER"))]
    pub(crate) skip_tier: Option<u8>,
    /// Check only the NFA and the grouped NFA; patterns it
    /// does not admit count as rejected
    #[bpaf(long)]
    pub(crate) grouped_nfa: bool,
    /// Longest input to try, instead of the family's own length (a
    /// shorter one for a quick pass, e.g. with a debug build)
    #[bpaf(long, argument("N"))]
    pub(crate) max_len: Option<usize>,
    /// Caps and parallelism of the child processes
    #[bpaf(external(cap_args))]
    pub(crate) cap: CapArgs,
    /// nullable-counter, tier3-assert, assert-edges, assert-in-body or tier1-assert
    #[bpaf(positional("FAMILY"))]
    pub(crate) family: Family,
}

impl SweepArgs {
    /// Apply the family's default only when no backend selector was supplied.
    fn choice(&self) -> TierChoice {
        if matches!(self.family, Family::Tier1Assert)
            && self.tier.is_none()
            && self.skip_tier.is_none()
            && !self.grouped_nfa
        {
            TierChoice::Only(1)
        } else {
            TierChoice::new(self.tier, self.skip_tier, self.grouped_nfa)
        }
    }
}

/// Arguments of one `rethoc brute` run.
struct SweepRun {
    pattern: String,
    unroll: usize,
    /// Arguments after `rethoc`.
    args: Vec<String>,
}

/// Position of the `--max-len` value in [`SweepRun::args`].
const MAX_LEN_ARG: usize = 6;

impl SweepRun {
    fn new(pattern: String, alphabet: &str, max_len: usize, unroll: usize, merge: bool) -> Self {
        let mut args: Vec<String> = ["brute", "--show", "1", "--alphabet", alphabet, "--max-len"]
            .map(String::from)
            .into();
        debug_assert_eq!(args.len(), MAX_LEN_ARG);
        args.push(max_len.to_string());
        args.extend(["--unroll-limit".into(), unroll.to_string()]);
        if !merge {
            args.push("--no-merge".into());
        }
        if pattern.starts_with('-') {
            args.push("--".into());
        }
        args.push(pattern.clone());
        Self {
            pattern,
            unroll,
            args,
        }
    }

    /// Override input length before inserting the effective backend selector.
    fn configure(&mut self, max_len: Option<usize>, choice: TierChoice) {
        if let Some(n) = max_len {
            self.args[MAX_LEN_ARG] = n.to_string();
        }
        // Backend flags shift MAX_LEN_ARG, so they must be inserted last.
        match choice {
            TierChoice::All => {}
            TierChoice::Only(tier) => {
                self.args.splice(1..1, ["--tier".into(), tier.to_string()]);
            }
            TierChoice::Except(tier) => {
                self.args
                    .splice(1..1, ["--skip-tier".into(), tier.to_string()]);
            }
            TierChoice::Grouped => self.args.insert(1, "--grouped-nfa".into()),
        }
    }
}

/// Counter bodies that can match empty, under every kind of quantifier,
/// between anchors and literals.  Sensitive to Bugs 60 and 61.
fn nullable_counter() -> Vec<SweepRun> {
    const BODIES: [&str; 14] = [
        "e?",
        "e*",
        "e|",
        "e*|y",
        "e?y?",
        "(?:e|y)*",
        r"\b|e",
        r"\B|e",
        "^|e",
        "$|e",
        r"e?\b",
        "(?:e?){2}",
        "(?:e*){2,}",
        "e{0,2}",
    ];
    const QUANTS: [&str; 7] = ["{1,3}", "{2,3}", "{3}", "{2,}", "{3,}", "{0,3}", "{5,9}"];
    const PREFIXES: [&str; 4] = ["", "^", "x?", "y"];
    const SUFFIXES: [&str; 5] = ["d", "", "d$", "(?:d|e)", "y{2,3}d"];
    let mut runs = Vec::new();
    for b in BODIES {
        for q in QUANTS {
            for p in PREFIXES {
                for s in SUFFIXES {
                    for unroll in [0, 32] {
                        runs.push(SweepRun::new(
                            format!("{p}(?:{b}){q}{s}"),
                            "edy ",
                            5,
                            unroll,
                            true,
                        ));
                    }
                }
            }
        }
    }
    runs
}

/// Counted parts next to assertions: `prefix counted sep counted suffix`
/// and `prefix (counted sep c)+ suffix`, unrolling off.  Sensitive to the
/// Tier 3 fixes of Bugs 56-59.
fn tier3_assert() -> Vec<SweepRun> {
    const PREFIXES: [&str; 7] = ["", "^", r"\b", r"^\b", "c", " ?", "a?"];
    const COUNTED: [&str; 7] = [
        "a{2,3}",
        "b{2,3}",
        ".{2,3}",
        ".{1,2}",
        "[ab]{1,2}",
        "(a|b ){2,3}",
        "(a|.b){2,3}",
    ];
    const SEPARATORS: [&str; 7] = ["", " ", r"\b", r"\B", "c", r" \b", r"\b "];
    const SUFFIXES: [&str; 4] = ["", r"\b", "$", r"\b$"];
    let mut patterns = Vec::new();
    for p in PREFIXES {
        for c1 in COUNTED {
            for sep in SEPARATORS {
                for s in SUFFIXES {
                    for c2 in COUNTED {
                        patterns.push(format!("{p}{c1}{sep}{c2}{s}"));
                    }
                    patterns.push(format!("{p}({c1}{sep}c)+{s}"));
                }
            }
        }
    }
    patterns
        .into_iter()
        .map(|p| SweepRun::new(p, "abc ", 6, 0, true))
        .collect()
}

/// Every assertion kind the DFA tiers support, on either side of a
/// one-byte repetition: `prefix counted assertion suffix`, with and
/// without unrolling.  One suffix is a second counter, with a body of
/// one or two bytes.  The alphabet has two word bytes (`a`, `c`), two
/// non-word bytes (space, `-`) and `\n`: the kinds of byte those
/// assertions tell apart.
///
/// Sensitive to Bug 65 (byte equivalence classes merging such bytes)
/// when run with `RETHOC_BYTE_CLASS_MIN_STATES=0`: the patterns are far
/// too small to get byte classes otherwise.  Sensitive to Bug 69: tier 3
/// lost a counter that starts behind `(?m:^)` after another counter.
fn assert_edges() -> Vec<SweepRun> {
    const PREFIXES: [&str; 8] = [
        "",
        "^",
        "(?m:^)",
        r"\b",
        r"\B",
        r"\b{start}",
        r"\b{end}",
        " ?",
    ];
    const COUNTED: [&str; 4] = ["a{2,3}", "[a-]{2,3}", "[^a]{2,3}", ".{2,}"];
    const ASSERTIONS: [&str; 8] = [
        "",
        "$",
        "(?m:$)",
        "(?m:^)",
        r"\b",
        r"\B",
        r"\b{end}",
        r"\b{start}",
    ];
    const SUFFIXES: [&str; 5] = ["", "c", " c", r"\n", "(?:c|--){2,3}"];
    let mut runs = Vec::new();
    for p in PREFIXES {
        for c in COUNTED {
            for a in ASSERTIONS {
                for s in SUFFIXES {
                    for unroll in [32, 0] {
                        runs.push(SweepRun::new(
                            format!("{p}{c}{a}{s}"),
                            r"a c\n-",
                            6,
                            unroll,
                            true,
                        ));
                    }
                }
            }
        }
    }
    runs
}

/// An assertion inside a counted body: in front of a byte, behind it,
/// between two bytes, on one branch only; word boundaries, `$`, `(?m:$)`,
/// `^` and `(?m:^)`; bodies of one byte, of two, and of variable length;
/// entered through a byte that the body also accepts or not, or through
/// a byte and a line anchor; followed by literals and by further
/// assertions, some of them behind a byte.  With and without unrolling.
///
/// Sensitive to Bugs 67, 68 and 69: before them, tier 2 took bodies with
/// an assertion behind the byte and got them wrong, tier 3 did the same
/// with `$`, and a debug build of tier 3 panicked on a line anchor behind
/// a byte anywhere in the pattern.
///
/// Tier 3 is still wrong on the bodies with `(?m:^)` right before the
/// count (480 runs): a known defect of the tier, not fixed one shape at
/// a time.  `--skip-tier 3` checks the other tiers, which must be clean.
fn assert_in_body() -> Vec<SweepRun> {
    const BODIES: [&str; 27] = [
        // One byte.
        r"a\b",
        r"a\B",
        r"[ab]\b",
        r"[ab]\B",
        r"[a ]\b",
        r"[a ]\B",
        r"\ba",
        r"\Ba",
        r"\b[a ]",
        r"\B[ab]",
        r"a\B|b",
        r"a\b| ",
        r"\Ba|b",
        "a$",
        "a$|b",
        "a(?m:$)",
        r"\n(?m:^)",
        r"a(?m:^)|\n",
        r"(?m:^)a|\n",
        // Two bytes, or one or two, or any number.
        "a$|bb",
        r"a\b|bb",
        r"a(?m:^)|\n\n",
        r"[a\n](?m:^)|bb",
        r"\na(?m:^)|bb",
        r"\n(?m:^)a|bb",
        r"[a\n](?m:^)c|bb",
        r"(?m:^)a+\n",
    ];
    const QUANTS: [&str; 4] = ["{1,2}", "{2,3}", "{2,}", "{2}"];
    const PREFIXES: [&str; 7] = ["", "^", "a", "b", " ?", r"\n(?m:^)", "c(?m:^)"];
    const TAILS: [&str; 9] = [
        "",
        "c",
        "$",
        r"\Bc",
        r"\bc",
        "(?m:$)",
        r"\n(?m:^)c",
        "a(?m:^)c",
        "a^c",
    ];
    let mut runs = Vec::new();
    for b in BODIES {
        for q in QUANTS {
            for p in PREFIXES {
                for t in TAILS {
                    for unroll in [32, 0] {
                        runs.push(SweepRun::new(
                            format!("{p}(?:{b}){q}{t}"),
                            r"ab c\n",
                            6,
                            unroll,
                            true,
                        ));
                    }
                }
            }
        }
    }
    runs
}

/// Every accepted assertion kind, including CRLF, in ordered chains before
/// and after consumption, alternatives, and zero-width cycles. Every pattern
/// remains counter-free with unrolling disabled, so forced Tier 1 must admit
/// every run. The alphabet separates word, ordinary nonword, CR, LF and raw
/// non-ASCII bytes. Five-byte inputs reach paths on both sides of CRLF pairs.
///
/// Three compile configurations mirror the checking commands' defaults;
/// explicit options keep a failure attributable to a single capped child.
fn tier1_assert() -> Vec<SweepRun> {
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
    let mut patterns = Vec::new();
    for first in ASSERTIONS {
        for second in ASSERTIONS {
            patterns.extend([
                format!("{first}{second}a"),
                format!(".{first}{second}"),
                format!(r"(?:{first}{second}a|\r{second}{first})"),
                // The consuming alternative preserves the star: regex-syntax
                // otherwise reduces an assertion-only repetition to optional.
                format!("(?:{first}{second}|a)*.{second}{first}"),
            ]);
        }
    }
    patterns.extend(
        [
            // The prerequisite context and optimistic-EOF defects.
            r"\b(?m:^)a",
            r"\B(?m:^)",
            r"a$\B",
            r"$\b",
            "$^",
            // Bare CR, CRLF middle positions, trailing CR, mixed line modes.
            r"\r(?Rm:^)a",
            r"\r(?Rm:^)\n",
            r"\r(?Rm:$)\n",
            r"(?Rm:^)$",
            r"\n\b(?Rm:^)a",
            r"\r(?Rm:$)(?m:$)\n",
            // Triples: both resolution phases must preserve the boundary.
            r"\B(?Rm:^)\B",
            r"\b(?Rm:^)\b{start}a",
            r"a\b{end}(?Rm:$)$",
            r"\r(?Rm:^)\B$",
            // Consumers and bypasses exercise class, case-folded, table and
            // fused consume/branch lowering as well as nullable continuations.
            r"(?i:a)\b(?Rm:$)",
            r"[a\r\n]\B(?Rm:^)[^a]",
            r"(?:aa| a|\ra|\na)\b(?Rm:$)",
            r"(?:a?\b|\r(?Rm:^))a?",
            r"(?:a\B|\r(?Rm:^))*a(?Rm:$)",
        ]
        .map(String::from),
    );

    let mut runs = Vec::new();
    for pattern in patterns {
        for (unroll, merge) in [(32, true), (0, true), (0, false)] {
            runs.push(SweepRun::new(
                pattern.clone(),
                r"a \r\n\xFF",
                5,
                unroll,
                merge,
            ));
        }
    }
    runs
}

/// Every `brute` run of a family.
fn runs(family: Family) -> Vec<SweepRun> {
    match family {
        Family::NullableCounter => nullable_counter(),
        Family::Tier3Assert => tier3_assert(),
        Family::AssertEdges => assert_edges(),
        Family::AssertInBody => assert_in_body(),
        Family::Tier1Assert => tier1_assert(),
    }
}

/// Run `rethoc sweep`; returns the exit code.
pub(crate) fn run(args: &SweepArgs) -> i32 {
    // This family's contract is that all its graphs are Tier 1 eligible.
    // Do not let an eligibility regression turn a blocking sweep green by
    // silently rejecting the patterns it is meant to check. Explicitly
    // selecting another backend retains the usual rejection semantics.
    let choice = args.choice();
    let require_admission =
        matches!(args.family, Family::Tier1Assert) && choice == TierChoice::Only(1);
    let mut runs = runs(args.family);
    for r in &mut runs {
        r.configure(args.max_len, choice);
    }
    if args.list {
        let mut out = std::io::stdout().lock();
        for r in &runs {
            let mut cmd = vec!["rethoc".to_string()];
            cmd.extend(r.args.iter().cloned());
            // Stop quietly when the reader goes away (`| head`).
            if writeln!(out, "{}{}", repro_env(), shell_join(&cmd)).is_err() {
                break;
            }
        }
        return EXIT_OK;
    }

    let caps = args.cap.caps(1_000_000, 20);
    let start = Instant::now();
    let mut counts = [0usize; 8];
    let index = |v: Verdict| v as usize;
    parallel(
        &runs,
        args.cap.jobs(),
        |r| run_capped(&r.args, caps),
        |r, result| {
            counts[index(result.verdict)] += 1;
            if !matches!(result.verdict, Verdict::Ok | Verdict::Rejected)
                || (require_admission && result.verdict == Verdict::Rejected)
            {
                println!(
                    "{} unroll={}{} `{}`: {}",
                    result.verdict.label(),
                    r.unroll,
                    regime_note(),
                    r.pattern,
                    result.detail()
                );
            }
        },
    );
    let count = |v: Verdict| counts[index(v)];
    let aborted: usize = [
        Verdict::Watchdog,
        Verdict::Timeout,
        Verdict::OutOfMemory,
        Verdict::Panic,
        Verdict::Crash,
    ]
    .into_iter()
    .map(count)
    .sum();
    println!(
        "sweep {}{}: runs={} ok={} mismatch={} rejected={} aborted={aborted} (timeout={} oom={} panic={} crash={}) in {:.0}s",
        args.family.name(),
        regime_note(),
        runs.len(),
        count(Verdict::Ok),
        count(Verdict::Mismatch),
        count(Verdict::Rejected),
        count(Verdict::Timeout) + count(Verdict::Watchdog),
        count(Verdict::OutOfMemory),
        count(Verdict::Panic),
        count(Verdict::Crash),
        start.elapsed().as_secs_f64()
    );
    if count(Verdict::Mismatch) > 0 {
        EXIT_MISMATCH
    } else if aborted > 0 {
        EXIT_ABORT
    } else if require_admission && count(Verdict::Rejected) > 0 {
        EXIT_REJECTED
    } else {
        EXIT_OK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_sweep(selectors: &[&str], family: &str) -> SweepArgs {
        let argv: Vec<&str> = std::iter::once("sweep")
            .chain(selectors.iter().copied())
            .chain(std::iter::once(family))
            .collect();
        let crate::Opts::Sweep(args) = crate::opts()
            .run_inner(bpaf::Args::from(argv.as_slice()))
            .expect("sweep arguments must parse")
        else {
            panic!("expected the sweep command");
        };
        args
    }

    fn parse_child(run: &SweepRun) -> crate::check::brute::BruteArgs {
        let argv: Vec<&str> = run.args.iter().map(String::as_str).collect();
        let crate::Opts::Brute(args) = crate::opts()
            .run_inner(bpaf::Args::from(argv.as_slice()))
            .expect("generated child arguments must parse")
        else {
            panic!("expected the brute command");
        };
        args
    }

    #[test]
    fn tier1_sweep_defaults_and_explicit_selectors_reach_the_child() {
        let cases: &[(&[&str], TierChoice)] = &[
            (&[], TierChoice::Only(1)),
            (&["--tier", "1"], TierChoice::Only(1)),
            (&["--tier", "0"], TierChoice::Only(0)),
            (&["--tier", "2"], TierChoice::Only(2)),
            (&["--skip-tier", "1"], TierChoice::Except(1)),
            (&["--skip-tier", "3"], TierChoice::Except(3)),
            (&["--grouped-nfa"], TierChoice::Grouped),
        ];
        for &(selectors, expected) in cases {
            let args = parse_sweep(selectors, "tier1-assert");
            let mut run = SweepRun::new("a".into(), "a", 2, 0, true);
            run.configure(args.max_len, args.choice());
            let child = parse_child(&run);
            assert_eq!(
                TierChoice::new(child.tier, child.skip_tier, child.grouped_nfa),
                expected,
                "selectors={selectors:?}",
            );
        }
        for family in [
            "nullable-counter",
            "tier3-assert",
            "assert-edges",
            "assert-in-body",
        ] {
            assert_eq!(parse_sweep(&[], family).choice(), TierChoice::All);
        }
    }

    /// A future eligibility regression must reject the child, rather than
    /// silently checking only the remaining engines and returning success.
    #[test]
    fn default_tier1_child_rejects_an_ineligible_pattern() {
        for (selectors, expected_exit) in [
            (&[][..], EXIT_REJECTED),
            (&["--tier", "1"][..], EXIT_REJECTED),
            (&["--tier", "0"][..], EXIT_OK),
        ] {
            let args = parse_sweep(selectors, "tier1-assert");
            let mut run = SweepRun::new("a{2}".into(), "a", 2, 0, true);
            run.configure(args.max_len, args.choice());
            assert_eq!(crate::check::brute::run(&parse_child(&run)), expected_exit);
        }
    }

    /// Exercise the real parser: flags must stay before the pattern's `--`,
    /// and adding them must not break the sweep's input-length override.
    #[test]
    fn compilation_flags_precede_hyphenated_patterns() {
        for pattern in ["a", "-a", "--no-merge"] {
            for merge in [true, false] {
                let mut run = SweepRun::new(pattern.into(), "a-", 7, 0, merge);
                run.configure(Some(2), TierChoice::Only(1));
                let child = parse_child(&run);
                assert_eq!(child.pattern, pattern);
                assert_eq!(child.max_len, 2);
                assert_eq!(child.compile.unroll_limit, Some(0));
                assert_eq!(child.compile.no_merge, !merge);
                assert_eq!(crate::check::brute::run(&child), EXIT_OK);
            }
        }
    }

    /// `--max-len` rewrites the argument at `MAX_LEN_ARG`.
    #[test]
    fn max_len_argument_position() {
        let run = SweepRun::new("a".into(), "ab", 6, 32, true);
        assert_eq!(run.args[MAX_LEN_ARG - 1], "--max-len");
        assert_eq!(run.args[MAX_LEN_ARG], "6");
    }

    #[test]
    fn nullable_counter_matches_the_original_sweep() {
        assert_eq!(nullable_counter().len(), 3920);
    }

    #[test]
    fn assert_edges_covers_the_bug_65_and_69_shapes() {
        let runs = assert_edges();
        assert_eq!(runs.len(), 8 * 4 * 8 * 5 * 2);
        let patterns: Vec<&str> = runs.iter().map(|r| r.pattern.as_str()).collect();
        for known in [
            r"(?m:^)a{2,3}c",
            r"a{2,3}(?m:$)",
            r"\b{start}a{2,3}c",
            r"a{2,3}\b{end}",
            // Bug 69: a counter behind a line anchor behind a counter.
            r"[^a]{2,3}(?m:^)(?:c|--){2,3}",
        ] {
            assert!(patterns.contains(&known), "missing {known}");
        }
    }

    #[test]
    fn assert_in_body_covers_the_bug_67_to_69_shapes() {
        let runs = assert_in_body();
        assert_eq!(runs.len(), 27 * 4 * 7 * 9 * 2);
        let patterns: Vec<&str> = runs.iter().map(|r| r.pattern.as_str()).collect();
        for known in [
            r"^(?:a\B|b){1,2}c",
            r"b(?:[ab]\B){2}c",
            r"(?:a\b){1,2}(?m:$)",
            r"(?:a$){1,2}",
            r"(?:a$|b){2,3}",
            r"(?:\B[ab]){2,3}c",
            r"(?:a(?m:^)|\n){2,3}c",
            // Variable-length bodies.
            r"(?:a$|bb){2,3}",
            r"(?:a(?m:^)|\n\n){2,3}c",
            r"(?:\n(?m:^)a|bb){2,3}c",
            // A line anchor behind a byte, outside the body.
            r"\n(?m:^)(?:a$|bb){2,3}c",
            r"c(?m:^)(?:a$|bb){2,3}c",
            r"(?:a$|bb){2,3}\n(?m:^)c",
            r"(?:a$|bb){2,3}a(?m:^)c",
            r"(?:a$|bb){2,3}a^c",
        ] {
            assert!(patterns.contains(&known), "missing {known}");
        }
    }

    /// Patterns from the original (lost) Tier 3 sweep that found bugs.
    #[test]
    fn tier3_assert_covers_known_patterns() {
        let patterns: Vec<String> = tier3_assert().into_iter().map(|r| r.pattern).collect();
        for known in [
            r"^\ba{2,3} a{2,3}$",
            r"c(a|.b){2,3}.{2,3}\b",
            r"a?b{2,3} \b(a|b ){2,3}$",
            r" ?(.{2,3}\bc)+$",
        ] {
            assert!(patterns.iter().any(|p| p == known), "missing {known}");
        }
    }

    #[test]
    fn tier1_assert_covers_contexts_and_compile_configs() {
        let runs = tier1_assert();
        assert_eq!(runs.len(), (10 * 10 * 4 + 20) * 3);
        for configs in runs.as_chunks::<3>().0 {
            assert_eq!(configs[0].pattern, configs[1].pattern);
            assert_eq!(configs[0].pattern, configs[2].pattern);
            assert_eq!(configs[0].unroll, 32);
            assert_eq!(configs[1].unroll, 0);
            assert_eq!(configs[2].unroll, 0);
            assert!(!configs[0].args.iter().any(|arg| arg == "--no-merge"));
            assert!(!configs[1].args.iter().any(|arg| arg == "--no-merge"));
            assert!(configs[2].args.iter().any(|arg| arg == "--no-merge"));
            for run in configs {
                assert_eq!(run.args[4], r"a \r\n\xFF");
                assert_eq!(run.args[MAX_LEN_ARG], "5");
            }
        }
        let patterns: Vec<_> = runs.iter().map(|run| run.pattern.as_str()).collect();
        for required in [
            r"\b(?m:^)a",
            r"\B(?m:^)",
            r"a$\B",
            r"\r(?Rm:^)\n",
            r"\r(?Rm:$)\n",
            r"(?:\b{end}(?Rm:^)|a)*.(?Rm:^)\b{end}",
        ] {
            assert!(patterns.contains(&required), "missing {required}");
        }
    }

    /// The family must stay useful under --tier 1: rejecting CRLF or quietly
    /// retaining a counter would otherwise remove the intended coverage.
    #[test]
    fn tier1_assert_patterns_are_admitted() {
        use rethoc_engine::{MatcherMemory, Regex, RegexConfig};

        for run in tier1_assert() {
            let re = Regex::with_config(
                &run.pattern,
                RegexConfig {
                    max_unroll_states: run.unroll,
                    merge_repetitions: !run.args.iter().any(|arg| arg == "--no-merge"),
                    ..Default::default()
                },
            )
            .expect("sweep pattern must compile");
            let mut memory = MatcherMemory::default();
            assert!(
                memory.matcher_for_tier(&re, 1).is_ok(),
                "Tier 1 rejected {} (unroll={})",
                run.pattern,
                run.unroll
            );
        }
    }
}
