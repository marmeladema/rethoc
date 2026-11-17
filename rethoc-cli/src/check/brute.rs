//! `rethoc brute`: every input over an alphabet, up to a length.
//!
//! Inputs are enumerated by increasing length, so the first mismatch
//! printed for a configuration is a shortest one.

use bpaf::Bpaf;

use super::{
    EXIT_MISMATCH, EXIT_OK, EXIT_REJECTED, EXIT_USAGE, Failure, Reference, Subject, describe,
    escape, new_memory, shrink_command, unescape,
};
use crate::{CompileArgs, compile_args};

/// Arguments of `rethoc brute`.
#[derive(Bpaf, Clone, Debug)]
#[bpaf(ignore_rustdoc)]
pub(crate) struct BruteArgs {
    /// Bytes to build inputs from, with byte escapes (e.g. 'ab \n')
    #[bpaf(long, argument("BYTES"))]
    pub(crate) alphabet: String,
    /// Longest input to try
    #[bpaf(long, argument("N"), fallback(5), display_fallback)]
    pub(crate) max_len: usize,
    /// Check only the NFA and this tier (default: every eligible tier)
    #[bpaf(long, argument("TIER"))]
    pub(crate) tier: Option<u8>,
    /// Check every eligible tier but this one
    #[bpaf(long, argument("TIER"))]
    pub(crate) skip_tier: Option<u8>,
    /// Mismatching inputs to print per configuration; stops after the
    /// length where this many were found
    #[bpaf(long, argument("N"), fallback(3), display_fallback)]
    pub(crate) show: usize,
    /// Without --unroll-limit/--no-merge, check the three standard configurations
    #[bpaf(external(compile_args))]
    pub(crate) compile: CompileArgs,
    /// Pattern to check
    #[bpaf(positional("PATTERN"))]
    pub(crate) pattern: String,
}

/// Run `rethoc brute`; returns the exit code.
pub(crate) fn run(args: &BruteArgs) -> i32 {
    let alphabet = match unescape(&args.alphabet) {
        Ok(a) if !a.is_empty() => a,
        Ok(_) => {
            eprintln!("error: empty alphabet");
            return EXIT_USAGE;
        }
        Err(e) => {
            eprintln!("error: --alphabet: {e}");
            return EXIT_USAGE;
        }
    };
    let pattern = &args.pattern;
    let reference = match Reference::new(pattern) {
        Ok(r) => r,
        Err(e) => {
            println!("REJECTED `{pattern}` by the regex crate: {e}");
            return EXIT_REJECTED;
        }
    };

    let mut memory = new_memory();
    let mut failures: Vec<Failure> = Vec::new();
    let (mut checked, mut mismatched) = (0usize, 0usize);
    for config in args.compile.check_configs() {
        let choice = super::TierChoice::new(args.tier, args.skip_tier);
        let subject = match Subject::compile(pattern, &config, choice, &mut memory) {
            Ok(s) => s,
            Err(e) => {
                println!("REJECTED `{pattern}` {}: {e}", describe(&config));
                continue;
            }
        };
        checked += 1;

        let (mut total, mut bad, mut nfa_bad) = (0u64, 0u64, 0u64);
        let mut first_bad: Option<Vec<u8>> = None;
        let mut stopped_at = None;
        let mut input = Vec::with_capacity(args.max_len);
        for len in 0..=args.max_len {
            let mut idx = vec![0usize; len];
            loop {
                input.clear();
                input.extend(idx.iter().map(|&i| alphabet[i]));
                let expected = reference.is_match(&input);
                subject.check(&mut memory, &input, expected, &mut failures);
                total += 1;
                if !failures.is_empty() {
                    bad += 1;
                    if failures.iter().any(|f| f.tier == 0) {
                        nfa_bad += 1;
                    }
                    if bad as usize <= args.show {
                        let list: Vec<String> = failures.iter().map(|f| f.to_string()).collect();
                        println!(
                            "  MISMATCH {} input=\"{}\" expected={expected}: {}",
                            describe(&config),
                            escape(&input),
                            list.join(" ")
                        );
                    }
                    first_bad.get_or_insert_with(|| input.clone());
                }
                if !next(&mut idx, alphabet.len()) {
                    break;
                }
            }
            if bad > 0 && bad as usize >= args.show && len < args.max_len {
                stopped_at = Some(len);
                break;
            }
        }

        let stopped = stopped_at.map_or(String::new(), |l| format!(" (stopped after length {l})"));
        println!(
            "`{pattern}` {} min_tier={} tiers={:?}: {bad} mismatches / {total} inputs, nfa: {nfa_bad}{stopped}",
            describe(&config),
            subject.regex.min_tier(),
            subject.tiers,
        );
        if let Some(input) = first_bad {
            mismatched += 1;
            // The inputs above share one `MatcherMemory`, like the
            // streams of a real user.  A mismatch can therefore depend on
            // what earlier inputs left in the DFA caches; such an input
            // passes when run alone, and `shrink` would find nothing.
            let expected = reference.is_match(&input);
            subject.check(&mut new_memory(), &input, expected, &mut failures);
            if failures.is_empty() {
                println!(
                    "  note: input=\"{}\" passes on a fresh MatcherMemory; the mismatch depends on \
                     the DFA caches left by earlier inputs.  Rerun this command to reproduce it.",
                    escape(&input)
                );
            } else {
                println!("  repro: {}", shrink_command(pattern, &config, &input));
            }
        }
    }

    if mismatched > 0 {
        EXIT_MISMATCH
    } else if checked == 0 {
        EXIT_REJECTED
    } else {
        EXIT_OK
    }
}

/// Advance an odometer over `base` digits; `false` once it wraps around.
fn next(idx: &mut [usize], base: usize) -> bool {
    for d in idx.iter_mut().rev() {
        *d += 1;
        if *d < base {
            return true;
        }
        *d = 0;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::next;

    #[test]
    fn odometer_visits_every_combination() {
        let mut idx = vec![0; 3];
        let mut n = 1;
        while next(&mut idx, 4) {
            n += 1;
        }
        assert_eq!(n, 64);
        assert_eq!(idx, [0, 0, 0]);
        assert!(!next(&mut [], 4));
    }
}
