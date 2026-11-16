//! `rethoc sweep`: `brute` over a generated family of patterns.
//!
//! Each run is a child `rethoc brute` under a memory and time cap, so a
//! pattern that blows up is counted and the sweep goes on.

use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

use bpaf::Bpaf;

use super::cap::{CapArgs, Verdict, cap_args, parallel, run_capped};
use super::{EXIT_ABORT, EXIT_MISMATCH, EXIT_OK, shell_join};

/// A generated family of patterns.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Family {
    /// See [`nullable_counter`].
    NullableCounter,
    /// See [`tier3_assert`].
    Tier3Assert,
}

impl Family {
    /// Name on the command line.
    fn name(self) -> &'static str {
        match self {
            Self::NullableCounter => "nullable-counter",
            Self::Tier3Assert => "tier3-assert",
        }
    }
}

impl FromStr for Family {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nullable-counter" => Ok(Self::NullableCounter),
            "tier3-assert" => Ok(Self::Tier3Assert),
            other => Err(format!(
                "unknown family `{other}` (expected nullable-counter or tier3-assert)"
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
    /// count as rejected
    #[bpaf(long, argument("TIER"))]
    pub(crate) tier: Option<u8>,
    /// Caps and parallelism of the child processes
    #[bpaf(external(cap_args))]
    pub(crate) cap: CapArgs,
    /// nullable-counter or tier3-assert
    #[bpaf(positional("FAMILY"))]
    pub(crate) family: Family,
}

/// Arguments of one `rethoc brute` run.
struct SweepRun {
    pattern: String,
    unroll: usize,
    /// Arguments after `rethoc`.
    args: Vec<String>,
}

impl SweepRun {
    fn new(pattern: String, alphabet: &str, max_len: usize, unroll: usize) -> Self {
        let mut args: Vec<String> = ["brute", "--show", "1", "--alphabet", alphabet, "--max-len"]
            .map(String::from)
            .into();
        args.push(max_len.to_string());
        args.extend(["--unroll-limit".into(), unroll.to_string()]);
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
        .map(|p| SweepRun::new(p, "abc ", 6, 0))
        .collect()
}

/// Every `brute` run of a family.
fn runs(family: Family) -> Vec<SweepRun> {
    match family {
        Family::NullableCounter => nullable_counter(),
        Family::Tier3Assert => tier3_assert(),
    }
}

/// Run `rethoc sweep`; returns the exit code.
pub(crate) fn run(args: &SweepArgs) -> i32 {
    let mut runs = runs(args.family);
    if let Some(t) = args.tier {
        for r in &mut runs {
            r.args.splice(1..1, ["--tier".to_string(), t.to_string()]);
        }
    }
    if args.list {
        let mut out = std::io::stdout().lock();
        for r in &runs {
            let mut cmd = vec!["rethoc".to_string()];
            cmd.extend(r.args.iter().cloned());
            // Stop quietly when the reader goes away (`| head`).
            if writeln!(out, "{}", shell_join(&cmd)).is_err() {
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
            if !matches!(result.verdict, Verdict::Ok | Verdict::Rejected) {
                println!(
                    "{} unroll={} `{}`: {}",
                    result.verdict.label(),
                    r.unroll,
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
        "sweep {}: runs={} ok={} mismatch={} rejected={} aborted={aborted} (timeout={} oom={} panic={} crash={}) in {:.0}s",
        args.family.name(),
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
    } else {
        EXIT_OK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nullable_counter_matches_the_original_sweep() {
        assert_eq!(nullable_counter().len(), 3920);
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
}
