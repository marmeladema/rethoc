//! `rethoc`: inspect, run and cross-check the rethoc regex engine.
//!
//! The inspection commands (`info`, `dot`, `dump`, `match`, `grep`) are
//! defined here.  The differential-checking commands (`brute`, `fuzz`,
//! `shrink`, `sweep`) live in [`check`].

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::str::FromStr;

use bpaf::{Args, Bpaf};
use rethoc_engine::{AnyMatcher, GroupedNfaMode, MatcherMemory, Regex, RegexConfig};

mod check;

fn parse_pattern(pattern: &str, config: RegexConfig) -> Regex {
    Regex::with_config(pattern, config).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    })
}

/// Output format for the `info` and `dump` commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    Text,
    Json,
    /// Rust `{:#?}` Debug output.
    Debug,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "debug" => Ok(Self::Debug),
            other => Err(format!(
                "unknown format `{other}` (expected text, json or debug)"
            )),
        }
    }
}

// ---------------------------------------------------------------------------
// Command line
// ---------------------------------------------------------------------------

/// Compile options:
#[derive(Bpaf, Clone, Debug)]
pub(crate) struct CompileArgs {
    /// Max NFA states for repetition unrolling (0 disables unrolling, default 32)
    #[bpaf(long, argument("N"))]
    pub(crate) unroll_limit: Option<usize>,
    /// Do not merge adjacent repetitions with the same body
    #[bpaf(long)]
    pub(crate) no_merge: bool,
    /// Max estimated fully-unrolled states (default 2048)
    #[bpaf(long, argument("N"))]
    pub(crate) max_states: Option<usize>,
}

impl CompileArgs {
    /// Flags that select exactly `config`; `--max-states` only when it
    /// differs from the default.
    pub(crate) fn exact(config: &RegexConfig) -> Self {
        Self {
            unroll_limit: Some(config.max_unroll_states),
            no_merge: !config.merge_repetitions,
            max_states: Some(config.max_estimated_states)
                .filter(|&m| m != RegexConfig::default().max_estimated_states),
        }
    }

    /// The single configuration these flags describe.
    pub(crate) fn config(&self) -> RegexConfig {
        let mut config = RegexConfig::default();
        if let Some(u) = self.unroll_limit {
            config.max_unroll_states = u;
        }
        if let Some(m) = self.max_states {
            config.max_estimated_states = m;
        }
        config.merge_repetitions = !self.no_merge;
        if let Some(n) = check::regime().byte_class_min_states {
            config.byte_class_min_states = n;
        }
        config
    }

    /// Configurations the checking commands compile a pattern with.
    ///
    /// Without `--unroll-limit` or `--no-merge`, these are the three the
    /// test suite and the fuzzers use: default unrolling, unrolling off,
    /// and unrolling and merging off.  Either flag selects that single
    /// configuration instead.
    pub(crate) fn check_configs(&self) -> Vec<RegexConfig> {
        if self.unroll_limit.is_some() || self.no_merge {
            return vec![self.config()];
        }
        [(None, true), (Some(0), true), (Some(0), false)]
            .into_iter()
            .map(|(unroll, merge)| {
                CompileArgs {
                    unroll_limit: unroll,
                    no_merge: !merge,
                    max_states: self.max_states,
                }
                .config()
            })
            .collect()
    }

    /// The flags again, as command-line arguments (only those that were
    /// set, so a child process gets the same defaults).
    pub(crate) fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(u) = self.unroll_limit {
            args.extend(["--unroll-limit".into(), u.to_string()]);
        }
        if self.no_merge {
            args.push("--no-merge".into());
        }
        if let Some(m) = self.max_states {
            args.extend(["--max-states".into(), m.to_string()]);
        }
        args
    }
}

/// Inspect, run and cross-check the rethoc regex engine.
///
/// Inputs given to `brute`, `fuzz`, `shrink` and printed by them use
/// Rust byte escapes (`\n`, `\t`, `\\`, `\xNN`, ...).  Exit codes of the
/// checking commands: 0 clean, 1 mismatch, 2 usage error, 3 aborted
/// (watchdog, timeout, out of memory or crash), 4 pattern rejected.
#[derive(Bpaf, Clone, Debug)]
#[bpaf(options)]
enum Opts {
    /// Print diagnostic information about the compiled regex
    #[bpaf(command)]
    Info {
        /// Output format: text, json or debug
        #[bpaf(long, argument("FORMAT"), fallback(Format::Text))]
        format: Format,
        /// Also analyse the grouped NFA when Tier 1 or 2 runs the pattern
        #[bpaf(long)]
        grouped_nfa: bool,
        #[bpaf(external(compile_args))]
        compile: CompileArgs,
        #[bpaf(positional("PATTERN"))]
        pattern: String,
    },
    /// Output the NFA as a Graphviz DOT graph
    #[bpaf(command)]
    Dot {
        #[bpaf(external(compile_args))]
        compile: CompileArgs,
        #[bpaf(positional("PATTERN"))]
        pattern: String,
    },
    /// Match a pattern against one or more inputs (exits 1 if any does not match)
    #[bpaf(command("match"))]
    Match {
        /// Feed input in chunks of N bytes (default: whole input at once)
        #[bpaf(long, argument("N"))]
        chunk_size: Option<usize>,
        /// Force an execution tier (0 = NFA, 1-4 = DFA tiers)
        #[bpaf(long, argument("TIER"))]
        tier: Option<u8>,
        /// Force the grouped NFA (automatic selection otherwise picks the
        /// engine; `--tier` forces a numbered tier)
        #[bpaf(long)]
        grouped_nfa: bool,
        /// Print matcher state after each step
        #[bpaf(long)]
        debug: bool,
        #[bpaf(external(compile_args))]
        compile: CompileArgs,
        #[bpaf(positional("PATTERN"))]
        pattern: String,
        /// Literal input strings
        #[bpaf(positional("INPUT"), some("match needs at least one input"))]
        inputs: Vec<String>,
    },
    /// Print matching lines of a file, or of stdin (exits 1 if none matches)
    #[bpaf(command)]
    Grep {
        /// Force an execution tier (0 = NFA, 1-4 = DFA tiers)
        #[bpaf(long, argument("TIER"))]
        tier: Option<u8>,
        /// Force the grouped NFA
        #[bpaf(long)]
        grouped_nfa: bool,
        /// Print nothing; stop at the first matching line
        #[bpaf(short, long)]
        quiet: bool,
        #[bpaf(external(compile_args))]
        compile: CompileArgs,
        #[bpaf(positional("PATTERN"))]
        pattern: String,
        #[bpaf(positional("FILE"))]
        file: Option<String>,
    },
    /// Dump compiled NFA states, counters and analysis
    #[bpaf(command)]
    Dump {
        /// Output format: text or debug
        #[bpaf(long, argument("FORMAT"), fallback(Format::Text))]
        format: Format,
        /// Include tier-specific DFA analysis
        #[bpaf(long)]
        dfa: bool,
        /// Also analyse the grouped NFA when Tier 1 or 2 runs the pattern
        #[bpaf(long)]
        grouped_nfa: bool,
        #[bpaf(external(compile_args))]
        compile: CompileArgs,
        #[bpaf(positional("PATTERN"))]
        pattern: String,
    },
    /// Check every input over an alphabet up to a length against the regex crate
    #[bpaf(command)]
    Brute(#[bpaf(external(check::brute::brute_args))] check::brute::BruteArgs),
    /// Check a range of fuzz-generator seeds against the regex crate
    #[bpaf(command)]
    Fuzz(#[bpaf(external(check::fuzz::fuzz_args))] check::fuzz::FuzzArgs),
    /// Delete bytes from a mismatching input while it keeps mismatching
    #[bpaf(command)]
    Shrink(#[bpaf(external(check::shrink::shrink_args))] check::shrink::ShrinkArgs),
    /// Run `brute` over a family of generated patterns, each under a cap
    #[bpaf(command)]
    Sweep(#[bpaf(external(check::sweep::sweep_args))] check::sweep::SweepArgs),
}

fn parse_args() -> Opts {
    match opts().run_inner(Args::current_args()) {
        Ok(opts) => opts,
        Err(err) => {
            err.print_message(100);
            let code = err.exit_code();
            process::exit(if code == 0 { 0 } else { check::EXIT_USAGE });
        }
    }
}

/// Exit with a usage error unless `tier` is a valid tier number.
fn validate_tier(tier: Option<u8>) {
    if let Some(t) = tier
        && t > 4
    {
        eprintln!("error: --tier must be 0, 1, 2, 3, or 4");
        process::exit(check::EXIT_USAGE);
    }
}

/// Exit with a usage error if `--grouped-nfa` is combined with a tier
/// selection: it selects a backend that is not a tier.
fn validate_grouped(grouped: bool, tier: Option<u8>, skip_tier: Option<u8>) {
    if grouped && (tier.is_some() || skip_tier.is_some()) {
        eprintln!("error: --grouped-nfa cannot be combined with --tier or --skip-tier");
        process::exit(check::EXIT_USAGE);
    }
}

/// Exit with a usage error unless `--skip-tier` names a DFA tier and is
/// not combined with `--tier`.
fn validate_skip_tier(tier: Option<u8>, skip_tier: Option<u8>) {
    let Some(skip) = skip_tier else { return };
    if tier.is_some() {
        eprintln!("error: --tier and --skip-tier cannot be combined");
        process::exit(check::EXIT_USAGE);
    }
    if !(1..=4).contains(&skip) {
        eprintln!("error: --skip-tier must be 1, 2, 3, or 4");
        process::exit(check::EXIT_USAGE);
    }
}

fn run_info(pattern: &str, config: RegexConfig, format: Format) {
    let regex = parse_pattern(pattern, config);
    let info = regex.info();
    match format {
        Format::Text => {
            println!("Pattern: {pattern}");
            println!();
            println!("{info}");
        }
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string(&info).expect("RegexInfo should serialize to JSON")
            );
        }
        Format::Debug => {
            println!("{:#?}", regex);
        }
    }
}

fn run_dot(pattern: &str, config: RegexConfig) {
    let regex = parse_pattern(pattern, config);
    let stdout = io::stdout();
    let mut out = stdout.lock();
    regex.to_dot(&mut out);
    out.flush().unwrap();
}

fn run_dump(pattern: &str, config: RegexConfig, format: Format, dfa: bool) {
    let regex = parse_pattern(pattern, config);
    match format {
        Format::Text => {
            print!("{}", regex.dump(dfa));
        }
        Format::Debug => {
            println!("{:#?}", regex);
        }
        Format::Json => {
            eprintln!("error: JSON format is not supported for 'dump' (use 'info' instead)");
            process::exit(check::EXIT_USAGE);
        }
    }
}

/// `match`: automatic selection, or a forced tier or the grouped NFA.
fn run_match(
    pattern: &str,
    inputs: &[String],
    chunk_size: Option<usize>,
    tier: Option<u8>,
    grouped: bool,
    mut config: RegexConfig,
    debug: bool,
) {
    if grouped {
        config.grouped_nfa = GroupedNfaMode::Always;
    }
    let regex = parse_pattern(pattern, config);
    let mut memory = check::new_memory();

    eprintln!("pattern: {pattern}");
    eprintln!("memory_size: {} bytes", regex.memory_size());
    if let Some(cs) = chunk_size {
        eprintln!("chunk_size: {cs}");
    }
    match (tier, grouped) {
        (Some(t), _) => eprintln!("tier: {t} (forced)"),
        (None, true) => eprintln!("engine: grouped (forced)"),
        (None, false) => eprintln!("engine: {} (automatic)", regex.engine()),
    }
    if grouped && let Err(e) = regex.grouped_nfa_admission() {
        eprintln!("error: {e}");
        process::exit(1);
    }
    // The plan summary whenever the grouped NFA was analysed (forced, or
    // under automatic selection when neither Tier 1 nor Tier 2 applies).
    if let Some(g) = &regex.info().grouped_nfa {
        eprintln!("grouped NFA: {g}");
    }
    eprintln!();

    let mut any_failed = false;
    for input in inputs {
        let bytes = input.as_bytes();
        let mut matcher = match tier {
            Some(t) => memory.matcher_for_tier(&regex, t).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                process::exit(1);
            }),
            None if grouped => memory.matcher_grouped(&regex).expect("admitted"),
            None => memory.matcher(&regex),
        };

        if debug {
            eprintln!("--- input: {:?} ---", input);
            eprintln!("[init] {:#?}", matcher);
        }

        match chunk_size {
            None => {
                // Feed entire input at once.
                matcher.chunk(bytes);
                if debug {
                    eprintln!("[after chunk({:?})] {}", input, matcher);
                }
            }
            Some(cs) => {
                // Feed in chunks of cs bytes.
                for (i, chunk) in bytes.chunks(cs).enumerate() {
                    matcher.chunk(chunk);
                    if debug {
                        let chunk_str = String::from_utf8_lossy(chunk);
                        eprintln!(
                            "[after chunk #{} {:?} (bytes {}..{})] {}",
                            i,
                            chunk_str,
                            i * cs,
                            i * cs + chunk.len(),
                            matcher
                        );
                    }
                }
            }
        }

        let matched = matcher.finish();

        if matched {
            println!("  \x1b[32mMATCH\x1b[0m  {:?}", input);
        } else {
            println!("  \x1b[31mNO MATCH\x1b[0m  {:?}", input);
            any_failed = true;
        }
    }

    if any_failed {
        process::exit(1);
    }
}

fn grep_matcher<'a>(
    memory: &'a mut MatcherMemory,
    regex: &'a Regex,
    tier: Option<u8>,
    grouped: bool,
) -> AnyMatcher<'a> {
    let result = match tier {
        Some(tier) => memory.matcher_for_tier(regex, tier),
        None if grouped => memory.matcher_grouped(regex).map_err(|e| e.to_string()),
        None => return memory.matcher(regex),
    };
    result.unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    })
}

fn run_grep(
    pattern: &str,
    file: Option<&str>,
    mut config: RegexConfig,
    quiet: bool,
    tier: Option<u8>,
    grouped: bool,
) {
    if grouped {
        config.grouped_nfa = GroupedNfaMode::Always;
    }
    let regex = parse_pattern(pattern, config);
    let mut memory = check::new_memory();
    if tier.is_some() || grouped {
        grep_matcher(&mut memory, &regex, tier, grouped);
    }

    let reader: Box<dyn BufRead> = match file {
        Some(path) => {
            let f = File::open(path).unwrap_or_else(|e| {
                eprintln!("error: cannot open {path}: {e}");
                process::exit(1);
            });
            Box::new(BufReader::new(f))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut any_matched = false;

    for line_result in reader.lines() {
        let line = line_result.unwrap_or_else(|e| {
            eprintln!("error: reading input: {e}");
            process::exit(1);
        });
        let mut matcher = grep_matcher(&mut memory, &regex, tier, grouped);
        matcher.chunk(line.as_bytes());
        if matcher.finish() {
            if !quiet {
                writeln!(out, "{}", line).unwrap();
            }
            any_matched = true;
            if quiet {
                break;
            }
        }
    }

    if !any_matched {
        process::exit(1);
    }
}

fn main() {
    // Before any work, and before any child process is started.
    if let Err(message) = check::init_regime() {
        eprintln!("error: {message}");
        process::exit(check::EXIT_USAGE);
    }
    match parse_args() {
        Opts::Info {
            format,
            grouped_nfa,
            compile,
            pattern,
        } => {
            let mut config = compile.config();
            if grouped_nfa {
                config.grouped_nfa = GroupedNfaMode::Always;
            }
            run_info(&pattern, config, format)
        }
        Opts::Dot { compile, pattern } => run_dot(&pattern, compile.config()),
        Opts::Match {
            chunk_size,
            tier,
            grouped_nfa,
            debug,
            compile,
            pattern,
            inputs,
        } => {
            validate_tier(tier);
            validate_grouped(grouped_nfa, tier, None);
            if chunk_size == Some(0) {
                eprintln!("error: --chunk-size must be > 0");
                process::exit(check::EXIT_USAGE);
            }
            run_match(
                &pattern,
                &inputs,
                chunk_size,
                tier,
                grouped_nfa,
                compile.config(),
                debug,
            )
        }
        Opts::Grep {
            tier,
            grouped_nfa,
            quiet,
            compile,
            pattern,
            file,
        } => {
            validate_tier(tier);
            validate_grouped(grouped_nfa, tier, None);
            run_grep(
                &pattern,
                file.as_deref(),
                compile.config(),
                quiet,
                tier,
                grouped_nfa,
            )
        }
        Opts::Dump {
            format,
            dfa,
            grouped_nfa,
            compile,
            pattern,
        } => {
            let mut config = compile.config();
            if grouped_nfa {
                config.grouped_nfa = GroupedNfaMode::Always;
            }
            run_dump(&pattern, config, format, dfa)
        }
        Opts::Brute(args) => {
            validate_tier(args.tier);
            validate_skip_tier(args.tier, args.skip_tier);
            validate_grouped(args.grouped_nfa, args.tier, args.skip_tier);
            process::exit(check::brute::run(&args))
        }
        Opts::Fuzz(args) => {
            validate_tier(args.tier);
            validate_grouped(args.grouped_nfa, args.tier, None);
            process::exit(check::fuzz::run(&args))
        }
        Opts::Shrink(args) => {
            validate_tier(args.tier);
            validate_grouped(args.grouped_nfa, args.tier, None);
            process::exit(check::shrink::run(&args))
        }
        Opts::Sweep(args) => {
            validate_tier(args.tier);
            validate_skip_tier(args.tier, args.skip_tier);
            validate_grouped(args.grouped_nfa, args.tier, args.skip_tier);
            process::exit(check::sweep::run(&args))
        }
    }
}
