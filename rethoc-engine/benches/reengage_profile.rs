//! Callgrind benchmarks for prefilter re-engagement and matcher setup.
//!
//! Deterministic counts for long scans and repeated short streams, with
//! regression limits: the run fails when a case, compared with a baseline
//! (the previous run, or a named one),
//!
//! - executes more than [`MAX_IR_INCREASE_PERCENT`] more instructions, or
//! - mispredicts more than [`MAX_BCM_INCREASE_PERCENT`] more conditional
//!   branches (Callgrind's branch-predictor simulation).
//!
//! The second limit exists because the cost of handling a failed prefilter
//! candidate is mostly an input-dependent branch, not extra instructions.
//! Mispredict counts are small for most cases, so that limit is loose.
//!
//! Neither count sees effects such as code layout; `reengage` (Criterion)
//! measures wall-clock throughput for the same cases.
//!
//! Run with: `cargo bench --bench reengage_profile`
//! (needs `valgrind` and a matching `gungraun-runner`).
//!
//! Compare a branch with `main`:
//!
//! ```text
//! git checkout main   && cargo bench --bench reengage_profile -- --save-baseline=main
//! git checkout branch && cargo bench --bench reengage_profile -- --baseline=main
//! ```

use std::hint::black_box;

use gungraun::{
    Callgrind, CallgrindMetrics, EntryPoint, EventKind, LibraryBenchmarkConfig, library_benchmark,
    library_benchmark_group, main,
};
use rethoc_engine::{Engine, MatcherMemory, Regex, RegexConfig};

#[path = "common/reengage_cases.rs"]
mod cases;

/// Stream length before [`cases::Case::shrink`].  Callgrind runs the code
/// some fifty times slower than natively; this keeps the whole bench under
/// a minute while each case still spends nearly all of its instructions in
/// the scanning loop rather than in DFA construction.
const HAYSTACK_SIZE: usize = 256 * 1024;

/// Instruction-count increase, in percent, above which a case is reported
/// as a regression.
const MAX_IR_INCREASE_PERCENT: f64 = 5.0;

/// Increase in mispredicted conditional branches, in percent, above which
/// a case is reported as a regression.  Loose on purpose: several cases
/// mispredict only a few hundred branches in total, so small absolute
/// changes are large relative ones.  Re-arming the prefilter on input it
/// cannot skip multiplies the count many times over.
const MAX_BCM_INCREASE_PERCENT: f64 = 100.0;

/// Compile the pattern and build the stream.  Callgrind only measures the
/// benchmark function, not setup.
fn setup(name: &str) -> (Regex, Vec<u8>) {
    cases::Case::named(name).compile(HAYSTACK_SIZE)
}

#[library_benchmark]
#[bench::tier2_gap_once(args = ("tier2_gap_once"), setup = setup)]
#[bench::tier3_candidate_once(args = ("tier3_candidate_once"), setup = setup)]
#[bench::tier3_gap_once(args = ("tier3_gap_once"), setup = setup)]
#[bench::tier3_two_gaps_once(args = ("tier3_two_gaps_once"), setup = setup)]
#[bench::tier2_word_boundary_gap_once(args = ("tier2_word_boundary_gap_once"), setup = setup)]
#[bench::tier2_anchored_gap_once(args = ("tier2_anchored_gap_once"), setup = setup)]
#[bench::tier2_anchored_dead(args = ("tier2_anchored_dead"), setup = setup)]
#[bench::tier3_no_prefilter_gap_once(args = ("tier3_no_prefilter_gap_once"), setup = setup)]
#[bench::tier3_dense_candidates(args = ("tier3_dense_candidates"), setup = setup)]
#[bench::tier3_candidate_run(args = ("tier3_candidate_run"), setup = setup)]
#[bench::tier3_no_prefilter_start(args = ("tier3_no_prefilter_start"), setup = setup)]
#[bench::tier2_dense_candidates(args = ("tier2_dense_candidates"), setup = setup)]
#[bench::tier2_gap_text(args = ("tier2_gap_text"), setup = setup)]
#[bench::tier3_gaps_text(args = ("tier3_gaps_text"), setup = setup)]
#[bench::tier3_rule_text(args = ("tier3_rule_text"), setup = setup)]
#[bench::control_tier1_literal_text(args = ("control_tier1_literal_text"), setup = setup)]
#[bench::control_tier1_no_prefilter_text(args = ("control_tier1_no_prefilter_text"), setup = setup)]
#[bench::control_tier2_live_counter(args = ("control_tier2_live_counter"), setup = setup)]
#[bench::control_tier3_live_instances(args = ("control_tier3_live_instances"), setup = setup)]
fn bench_scan((re, haystack): (Regex, Vec<u8>)) -> bool {
    let mut memory = MatcherMemory::default();
    black_box(cases::scan(&mut memory, &re, black_box(&haystack)))
}

fn setup_overlap(case: cases::OverlapCase) -> (Regex, Vec<u8>) {
    case.compile(HAYSTACK_SIZE)
}

#[library_benchmark]
#[bench::tier2_overlap_expiry(args = (cases::OverlapCase::Expiry), setup = setup_overlap)]
#[bench::tier2_overlap_mixed_lengths(args = (cases::OverlapCase::MixedLengths), setup = setup_overlap)]
fn bench_overlap((re, haystack): (Regex, Vec<u8>)) -> bool {
    let mut memory = MatcherMemory::default();
    black_box(cases::scan_overlap(&mut memory, &re, black_box(&haystack)))
}

fn setup_short_streams(pattern: &str, engine: Engine) -> (Regex, MatcherMemory) {
    let re = Regex::with_config(
        pattern,
        RegexConfig {
            max_unroll_states: 0,
            merge_repetitions: false,
            ..RegexConfig::default()
        },
    )
    .expect("short-stream pattern compiles");
    assert_eq!(re.engine(), engine, "{pattern}");
    let mut memory = MatcherMemory::default();
    let mut matcher = memory.matcher(&re);
    matcher.chunk(b"z");
    assert!(!matcher.finish());
    (re, memory)
}

#[library_benchmark(config = profile_config(EntryPoint::Custom("*::measure_stream_construction*".into())))]
#[bench::tier1(args = ("abc|abd", Engine::Tier1), setup = setup_short_streams)]
#[bench::tier2(args = ("^.*a.{3}bc$", Engine::Tier2), setup = setup_short_streams)]
#[bench::tier4(args = ("^((a|bc){1,2}){2,3}$", Engine::Tier4), setup = setup_short_streams)]
#[bench::nfa(args = (r"x((a|aa){2,3}\b){2,3}", Engine::Nfa), setup = setup_short_streams)]
fn bench_short_streams((re, mut memory): (Regex, MatcherMemory)) -> usize {
    measure_stream_construction(&re, &mut memory)
}

// Give Callgrind a distinct entry point that excludes setup and teardown.
#[inline(never)]
fn measure_stream_construction(re: &Regex, memory: &mut MatcherMemory) -> usize {
    let re = black_box(re);
    let input = black_box(b"z");
    let mut count = 0;
    for _ in 0..4096 {
        let mut matcher = memory.matcher(re);
        matcher.chunk(input);
        count += usize::from(matcher.finish());
    }
    black_box(count)
}

library_benchmark_group!(
    name = reengage,
    benchmarks = [bench_scan, bench_overlap, bench_short_streams]
);

fn profile_config(entry_point: EntryPoint) -> LibraryBenchmarkConfig {
    let mut config = LibraryBenchmarkConfig::default();
    config.tool(
        Callgrind::with_args(["--branch-sim=yes"])
            .entry_point(entry_point)
            .format([CallgrindMetrics::Default, CallgrindMetrics::BranchSim])
            .soft_limits([
                (EventKind::Ir, MAX_IR_INCREASE_PERCENT),
                (EventKind::Bcm, MAX_BCM_INCREASE_PERCENT),
            ]),
    );
    config
}

main!(
    config = profile_config(EntryPoint::Default);
    library_benchmark_groups = reengage
);
