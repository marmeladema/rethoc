//! `rethoc shrink`: make a mismatching input smaller.
//!
//! Deletes runs of bytes (halving the run length down to one byte) while
//! the input keeps failing [`Subject::check`] in the same configuration,
//! on at least one of the tier and feed combinations it failed at first.

use bpaf::Bpaf;

use super::{
    EXIT_MISMATCH, EXIT_OK, EXIT_REJECTED, EXIT_USAGE, Failure, Feed, Reference, Subject,
    config_args, describe, escape, new_memory, shell_join, unescape,
};
use crate::{CompileArgs, compile_args};

/// Arguments of `rethoc shrink`.
#[derive(Bpaf, Clone, Debug)]
#[bpaf(ignore_rustdoc)]
pub(crate) struct ShrinkArgs {
    /// Check only the NFA and this tier (default: every eligible tier)
    #[bpaf(long, argument("TIER"))]
    pub(crate) tier: Option<u8>,
    /// Without --unroll-limit/--no-merge, use the first of the three
    /// standard configurations where the input mismatches
    #[bpaf(external(compile_args))]
    pub(crate) compile: CompileArgs,
    /// Pattern to check
    #[bpaf(positional("PATTERN"))]
    pub(crate) pattern: String,
    /// Mismatching input, with byte escapes
    #[bpaf(positional("INPUT"))]
    pub(crate) input: String,
}

/// Run `rethoc shrink`; returns the exit code (1 when a smaller
/// mismatching input was printed).
pub(crate) fn run(args: &ShrinkArgs) -> i32 {
    let mut input = match unescape(&args.input) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: INPUT: {e}");
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

    // The first configuration where the input mismatches.
    let mut subject = None;
    let mut compiled = 0;
    for config in args.compile.check_configs() {
        match Subject::compile(pattern, &config, args.tier, &mut memory) {
            Ok(s) => {
                compiled += 1;
                s.check(
                    &mut memory,
                    &input,
                    reference.is_match(&input),
                    &mut failures,
                );
                if !failures.is_empty() {
                    subject = Some(s);
                    break;
                }
            }
            Err(e) => println!("REJECTED `{pattern}` {}: {e}", describe(&config)),
        }
    }
    let Some(subject) = subject else {
        if compiled == 0 {
            return EXIT_REJECTED;
        }
        println!("no mismatch for `{pattern}` on \"{}\"", escape(&input));
        return EXIT_OK;
    };

    // Keep chasing the same bug: a candidate must still fail at least
    // one of the original (tier, feed) checks.
    let wanted: Vec<(u8, Feed)> = failures.iter().map(|f| (f.tier, f.feed)).collect();
    let mut fails = |candidate: &[u8]| {
        subject.check(
            &mut memory,
            candidate,
            reference.is_match(candidate),
            &mut failures,
        );
        failures.iter().any(|f| wanted.contains(&(f.tier, f.feed)))
    };
    let original = input.len();
    let mut run = (input.len() / 2).max(1);
    loop {
        let mut changed = false;
        let mut i = 0;
        while i < input.len() {
            let end = (i + run).min(input.len());
            let candidate: Vec<u8> = input[..i].iter().chain(&input[end..]).copied().collect();
            if fails(&candidate) {
                input = candidate;
                changed = true;
            } else {
                i += run;
            }
        }
        if run == 1 && !changed {
            break;
        }
        if !changed {
            run = (run / 2).max(1);
        }
    }

    let expected = reference.is_match(&input);
    subject.check(&mut memory, &input, expected, &mut failures);
    let list: Vec<String> = failures.iter().map(|f| f.to_string()).collect();
    println!(
        "`{pattern}` {}: shrunk {original} -> {} bytes: input=\"{}\" expected={expected}: {}",
        describe(&subject.config),
        input.len(),
        escape(&input),
        list.join(" ")
    );
    // `match` takes literal inputs; offer a trace command when possible.
    // Trace a failing DFA tier if there is one, with the failing feed.
    let traced = failures
        .iter()
        .find(|f| f.tier != 0 && wanted.contains(&(f.tier, f.feed)))
        .or_else(|| failures.iter().find(|f| wanted.contains(&(f.tier, f.feed))));
    if let Some(traced) = traced
        && input.iter().all(|b| b.is_ascii_graphic() || *b == b' ')
    {
        let mut cmd = vec!["rethoc".to_string(), "match".into(), "--debug".into()];
        if traced.feed == Feed::Bytewise {
            cmd.extend(["--chunk-size".into(), "1".into()]);
        }
        cmd.extend(["--tier".into(), traced.tier.to_string()]);
        cmd.extend(config_args(&subject.config));
        if pattern.starts_with('-') || input.starts_with(b"-") {
            cmd.push("--".into());
        }
        cmd.push(pattern.clone());
        cmd.push(String::from_utf8_lossy(&input).into_owned());
        println!("  trace: {}", shell_join(&cmd));
    }
    EXIT_MISMATCH
}
