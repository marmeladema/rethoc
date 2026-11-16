//! `rethoc fuzz`: deterministic seed ranges over the fuzz generator.
//!
//! Each seed gives a pattern and inputs through
//! `rethoc_engine::fuzz_gen`; a failure is a seed number anyone can
//! re-run.  The seed derivation must stay stable: known-clean ranges and
//! the slow-seed allowlist refer to it.
//!
//! Without `--batch` the range runs in this process, with a watchdog.
//! With `--batch`, it is split into batches run as capped child
//! processes; a failing batch is re-run one seed at a time to name the
//! seeds.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use bpaf::Bpaf;
use rethoc_engine::MatcherMemory;
use rethoc_engine::fuzz_gen::{FuzzRng, generate_inputs, generate_pattern};

use super::cap::{CapArgs, Run, Verdict, cap_args, parallel, run_capped};
use super::{
    EXIT_ABORT, EXIT_MISMATCH, EXIT_OK, EXIT_USAGE, Failure, Reference, Subject, describe, escape,
    shrink_command,
};
use crate::{CompileArgs, compile_args};

/// Mismatch lines printed per process.
const MAX_REPORTED: u64 = 20;

/// Arguments of `rethoc fuzz`.
#[derive(Bpaf, Clone, Debug)]
#[bpaf(ignore_rustdoc)]
pub(crate) struct FuzzArgs {
    /// Check only the NFA and this tier (default: every eligible tier)
    #[bpaf(long, argument("TIER"))]
    pub(crate) tier: Option<u8>,
    /// Use one MatcherMemory for every pattern of the run
    #[bpaf(long)]
    pub(crate) shared_memory: bool,
    /// Print every step to stderr
    #[bpaf(long)]
    pub(crate) trace: bool,
    /// Report a seed as SLOW after this many seconds
    #[bpaf(long, argument("SECS"), fallback(5), display_fallback)]
    pub(crate) slow_after: u64,
    /// Abort (exit 3) when one seed runs this many seconds
    #[bpaf(long, argument("SECS"), fallback(40), display_fallback)]
    pub(crate) abort_after: u64,
    /// Run in batches of N seeds, each in a capped child process
    #[bpaf(long, argument("N"))]
    pub(crate) batch: Option<u64>,
    /// File of known-slow seeds (one per line, `#` comments): with
    /// --batch, they run alone and an abort is reported, not failed
    #[bpaf(long, argument("FILE"))]
    pub(crate) allow_slow: Option<String>,
    /// Seed to leave out (repeatable)
    #[bpaf(long, argument("SEED"), hide)]
    pub(crate) skip: Vec<u64>,
    /// Caps and parallelism of the child processes
    #[bpaf(external(cap_args))]
    pub(crate) cap: CapArgs,
    /// Without --unroll-limit/--no-merge, check the three standard configurations
    #[bpaf(external(compile_args))]
    pub(crate) compile: CompileArgs,
    /// First seed
    #[bpaf(positional("FIRST"))]
    pub(crate) first: u64,
    /// Number of seeds
    #[bpaf(positional("COUNT"))]
    pub(crate) count: u64,
}

/// Run `rethoc fuzz`; returns the exit code.
pub(crate) fn run(args: &FuzzArgs) -> i32 {
    if args.batch.is_some() {
        return supervise(args);
    }
    if args.allow_slow.is_some() {
        eprintln!("error: --allow-slow needs --batch");
        return EXIT_USAGE;
    }
    run_in_process(args)
}

/// The pattern seed and input seed for `seed`.
///
/// Do not change: seed numbers in bug reports, CI ranges and the
/// slow-seed allowlist depend on it.
fn seed_bytes(seed: u64) -> (Vec<u8>, Vec<u8>) {
    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }
    let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let plen = (xorshift(&mut s) % 64) as usize;
    let pattern: Vec<u8> = (0..plen).map(|_| xorshift(&mut s) as u8).collect();
    let ilen = (xorshift(&mut s) % 128) as usize;
    let input: Vec<u8> = (0..ilen).map(|_| xorshift(&mut s) as u8).collect();
    (pattern, input)
}

// ---------------------------------------------------------------------------
// In-process run
// ---------------------------------------------------------------------------

/// What the watchdog sees: seed, what is running, since when.
type Current = Arc<Mutex<(u64, String, Instant)>>;

/// Tell the watchdog what is running now.
fn set_current(current: &Current, seed: u64, what: String) {
    *current.lock().expect("watchdog lock") = (seed, what, Instant::now());
}

/// Start a thread that reports a seed as SLOW after `slow_after` and
/// exits with [`EXIT_ABORT`] after `abort_after`.
fn start_watchdog(slow_after: Duration, abort_after: Duration) -> Current {
    let current: Current = Arc::new(Mutex::new((0, String::new(), Instant::now())));
    let watched = current.clone();
    std::thread::spawn(move || {
        let mut reported = None;
        loop {
            std::thread::sleep(Duration::from_millis(200));
            let (seed, what, since) = watched.lock().expect("watchdog lock").clone();
            let elapsed = since.elapsed();
            if elapsed >= slow_after && reported != Some(seed) {
                println!(
                    "SLOW seed={seed} running for {}s: {what}",
                    elapsed.as_secs()
                );
                reported = Some(seed);
            }
            if elapsed >= abort_after {
                println!(
                    "ABORT seed={seed} still running after {}s: {what}",
                    elapsed.as_secs()
                );
                std::process::exit(EXIT_ABORT);
            }
        }
    });
    current
}

/// Run the seed range in this process, under the watchdog.
fn run_in_process(args: &FuzzArgs) -> i32 {
    let current = start_watchdog(
        Duration::from_secs(args.slow_after),
        Duration::from_secs(args.abort_after),
    );
    // Name the seed when the engine panics.
    let on_panic = current.clone();
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(guard) = on_panic.try_lock() {
            println!("PANIC seed={}: {}", guard.0, guard.1);
        }
        default_hook(info);
    }));
    let configs = args.compile.check_configs();
    let skip: BTreeSet<u64> = args.skip.iter().copied().collect();
    let end = args.first.saturating_add(args.count);

    let mut shared = MatcherMemory::default();
    let mut failures: Vec<Failure> = Vec::new();
    let (mut cases, mut skipped, mut compiled, mut checks, mut mismatches) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut per_tier = [0u64; 5];
    for seed in args.first..end {
        if skip.contains(&seed) {
            continue;
        }
        let (pseed, iseed) = seed_bytes(seed);
        set_current(&current, seed, "generating pattern".into());
        let (pattern, ast) = generate_pattern(&mut FuzzRng::new(&pseed));
        set_current(&current, seed, format!("generating inputs for `{pattern}`"));
        let inputs = generate_inputs(&mut FuzzRng::new(&iseed), &ast);
        set_current(&current, seed, format!("`{pattern}`"));
        cases += 1;
        if args.trace {
            let longest = inputs.iter().map(Vec::len).max().unwrap_or(0);
            eprintln!(
                "seed {seed} `{pattern}` inputs={} longest={longest}",
                inputs.len()
            );
        }
        let Ok(reference) = Reference::new(&pattern) else {
            skipped += 1;
            continue;
        };
        // Compile every configuration, then run each input through all of
        // them in turn.  With --shared-memory one memory thus alternates
        // between differently sized regexes; otherwise each compiled
        // regex has a memory of its own.
        let mut subjects = Vec::new();
        for config in &configs {
            let mut own = MatcherMemory::default();
            let memory = if args.shared_memory {
                &mut shared
            } else {
                &mut own
            };
            if let Ok(subject) = Subject::compile(&pattern, config, args.tier, memory) {
                if args.trace {
                    eprintln!(
                        "  {} min_tier={} tiers={:?}",
                        describe(config),
                        subject.regex.min_tier(),
                        subject.tiers
                    );
                }
                subjects.push((subject, own));
            }
        }
        compiled += subjects.len() as u64;
        for input in &inputs {
            if args.trace {
                eprintln!("    input=\"{}\"", escape(input));
            }
            let expected = reference.is_match(input);
            for (subject, own) in &mut subjects {
                let memory = if args.shared_memory { &mut shared } else { own };
                subject.check(memory, input, expected, &mut failures);
                checks += 2 * subject.tiers.len() as u64;
                for &t in &subject.tiers {
                    per_tier[t as usize] += 2;
                }
                if failures.is_empty() {
                    continue;
                }
                mismatches += 1;
                if mismatches <= MAX_REPORTED {
                    let list: Vec<String> = failures.iter().map(|f| f.to_string()).collect();
                    println!(
                        "MISMATCH seed={seed} {} expected={expected} {} pattern=`{pattern}` input=\"{}\"",
                        describe(&subject.config),
                        list.join(" "),
                        escape(input)
                    );
                    println!(
                        "  repro: {}",
                        shrink_command(&pattern, &subject.config, input)
                    );
                }
            }
        }
    }
    println!(
        "seeds {}..{end} cases={cases} reference-rejected={skipped} compiled={compiled} checks={checks} per-tier={per_tier:?} mismatches={mismatches}",
        args.first
    );
    if mismatches > 0 {
        EXIT_MISMATCH
    } else {
        EXIT_OK
    }
}

// ---------------------------------------------------------------------------
// Supervised run
// ---------------------------------------------------------------------------

/// Read an allowlist: one seed per line, `#` starts a comment.
fn read_allowlist(path: &str) -> Result<BTreeSet<u64>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut seeds = BTreeSet::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let seed = line
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("{path}:{}: expected a seed number", n + 1))?;
        seeds.insert(seed);
    }
    Ok(seeds)
}

/// One unit of work for a child process.
enum Task {
    /// Seeds `first..first + count`, minus the allowlisted ones.
    Batch { first: u64, count: u64 },
    /// One allowlisted seed.
    KnownSlow(u64),
}

/// What a task found.
#[derive(Default)]
struct Outcome {
    /// Report lines, printed when the task completes.
    lines: Vec<String>,
    /// Seeds that failed, with their verdict.
    failed: Vec<(u64, Verdict)>,
    /// Allowlisted seeds that hit the watchdog, as expected.
    known_slow: Vec<u64>,
    /// Allowlisted seeds that passed.
    no_longer_slow: Vec<u64>,
}

/// Run the seed range as capped child processes (`--batch`).
fn supervise(args: &FuzzArgs) -> i32 {
    let batch = args.batch.unwrap_or(1).max(1);
    let allow = match &args.allow_slow {
        Some(path) => match read_allowlist(path) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("error: --allow-slow {e}");
                return EXIT_USAGE;
            }
        },
        None => BTreeSet::new(),
    };
    let end = args.first.saturating_add(args.count);
    let skip: BTreeSet<u64> = args.skip.iter().copied().collect();
    // Each child stops itself after --abort-after on one seed, so the
    // timeout is only a backstop: by default, enough for every seed of
    // the child to run just under the watchdog.
    let caps_for = |count: u64| {
        let default = count.saturating_mul(args.abort_after).saturating_add(30);
        args.cap.caps(1_500_000, default)
    };

    // Allowlisted seeds first: they are the longest tasks.
    let mut tasks: Vec<Task> = allow
        .range(args.first..end)
        .filter(|s| !skip.contains(s))
        .map(|&s| Task::KnownSlow(s))
        .collect();
    let mut first = args.first;
    while first < end {
        let count = batch.min(end - first);
        tasks.push(Task::Batch { first, count });
        first += count;
    }

    // Arguments every child gets, before the seed range.
    let mut common = vec!["fuzz".to_string()];
    common.extend(args.compile.to_args());
    if let Some(t) = args.tier {
        common.extend(["--tier".into(), t.to_string()]);
    }
    if args.shared_memory {
        common.push("--shared-memory".into());
    }
    common.extend([
        "--slow-after".into(),
        args.slow_after.to_string(),
        "--abort-after".into(),
        args.abort_after.to_string(),
    ]);
    let excluded: Vec<u64> = allow.union(&skip).copied().collect();
    // A batch leaves out the allowlisted and skipped seeds; a seed run on
    // its own never does.
    let batch_args = |first: u64, count: u64| {
        let mut a = common.clone();
        for s in excluded
            .iter()
            .filter(|&&s| s >= first && s - first < count)
        {
            a.extend(["--skip".into(), s.to_string()]);
        }
        a.extend(["--".into(), first.to_string(), count.to_string()]);
        a
    };
    let seed_args = |seed: u64| {
        let mut a = common.clone();
        a.extend(["--".into(), seed.to_string(), "1".into()]);
        a
    };

    let work = |task: &Task| -> Outcome {
        let mut out = Outcome::default();
        match *task {
            Task::Batch { first, count } => {
                let run = run_capped(&batch_args(first, count), caps_for(count));
                if run.verdict == Verdict::Ok {
                    out.lines.push(summary_line(&run));
                    return out;
                }
                out.lines.push(format!(
                    "batch {first}..{} {} after {:.1}s; re-running seed by seed",
                    first + count,
                    run.verdict.label(),
                    run.elapsed.as_secs_f64()
                ));
                let failed_before = out.failed.len();
                for seed in first..first + count {
                    if excluded.contains(&seed) {
                        continue;
                    }
                    let run = run_capped(&seed_args(seed), caps_for(1));
                    if run.verdict != Verdict::Ok {
                        out.lines.push(format!(
                            "FAIL seed={seed} {}: {}",
                            run.verdict.label(),
                            run.detail()
                        ));
                        out.lines.extend(
                            run.stdout
                                .lines()
                                .filter(|l| l.trim_start().starts_with("repro:"))
                                .take(1)
                                .map(str::to_string),
                        );
                        out.failed.push((seed, run.verdict));
                    }
                }
                if out.failed.len() == failed_before {
                    // Typically a seed close to --abort-after under load.
                    out.lines.push(format!(
                        "FAIL batch {first}..{} {} but every seed passes alone; rerun with fewer --jobs",
                        first + count,
                        run.verdict.label()
                    ));
                    out.failed.push((first, run.verdict));
                }
            }
            Task::KnownSlow(seed) => {
                let run = run_capped(&seed_args(seed), caps_for(1));
                match run.verdict {
                    Verdict::Ok => {
                        out.lines.push(format!(
                            "NOTE seed={seed} is on the slow allowlist but passed in {:.1}s (watchdog {}s); remove it if that holds",
                            run.elapsed.as_secs_f64(),
                            args.abort_after
                        ));
                        out.no_longer_slow.push(seed);
                    }
                    Verdict::Watchdog | Verdict::Timeout => {
                        out.lines.push(format!(
                            "KNOWN-SLOW seed={seed} {}: {}",
                            run.verdict.label(),
                            run.detail()
                        ));
                        out.known_slow.push(seed);
                    }
                    v => {
                        out.lines.push(format!(
                            "FAIL seed={seed} {} (allowlisted as slow only): {}",
                            v.label(),
                            run.detail()
                        ));
                        out.failed.push((seed, v));
                    }
                }
            }
        }
        out
    };

    let start = Instant::now();
    let mut total = Outcome::default();
    parallel(&tasks, args.cap.jobs(), work, |_, out| {
        for line in &out.lines {
            println!("{line}");
        }
        total.failed.extend(out.failed);
        total.known_slow.extend(out.known_slow);
        total.no_longer_slow.extend(out.no_longer_slow);
    });
    total.failed.sort_unstable_by_key(|f| f.0);
    total.known_slow.sort_unstable();
    let failed: Vec<String> = total
        .failed
        .iter()
        .map(|(s, v)| format!("{s}:{}", v.label()))
        .collect();
    println!(
        "fuzz seeds {}..{end}: failed=[{}] known-slow={:?} no-longer-slow={:?} in {:.0}s",
        args.first,
        failed.join(", "),
        total.known_slow,
        total.no_longer_slow,
        start.elapsed().as_secs_f64()
    );
    if total.failed.iter().any(|f| f.1 == Verdict::Mismatch) {
        EXIT_MISMATCH
    } else if !total.failed.is_empty() {
        EXIT_ABORT
    } else {
        EXIT_OK
    }
}

/// The child's final `seeds ...` line, or a placeholder.
fn summary_line(run: &Run) -> String {
    run.stdout
        .lines()
        .rev()
        .find(|l| l.starts_with("seeds "))
        .unwrap_or("(no summary)")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Seed numbers must keep naming the same patterns: this one is in
    /// the slow-seed allowlist and in issue #6.
    #[test]
    fn seed_derivation_is_stable() {
        let (pseed, _) = seed_bytes(6357);
        let (pattern, _) = generate_pattern(&mut FuzzRng::new(&pseed));
        assert_eq!(
            pattern,
            "^ {1,35}{3,12}*((a?a?)?(a?a?)?)?((a?a?)?(a?a?)?)?((a?a?)?(a?a?)?)?((a?a?)?(a?a?)?)?$"
        );
    }
}
