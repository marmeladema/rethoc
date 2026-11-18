//! Callgrind gate for existing-correct assertion behavior and shared-tier preparation.
//!
//! Run `cargo bench --bench tier1_assertions_profile -- --save-baseline=main`
//! on an isolated base worktree containing identical fixtures, then compare with
//! `--baseline=main`. Requires the matching gungraun-runner and valgrind.
//! Setup validates results against regex and primes only warm measurements.

use std::hint::black_box;

use gungraun::{
    Callgrind, CallgrindMetrics, EventKind, LibraryBenchmarkConfig, library_benchmark,
    library_benchmark_group, main,
};

#[path = "common/tier1_assertion_cases.rs"]
mod cases;

fn setup(name: &str, mode: &str, warm: bool) -> cases::Prepared {
    cases::Prepared::new(name, mode, warm, 64 * 1024)
}

#[library_benchmark]
#[bench::literal_sparse_direct_cold(args = ("literal_sparse", "direct", false), setup = setup)]
#[bench::literal_sparse_direct_warm(args = ("literal_sparse", "direct", true), setup = setup)]
#[bench::literal_dense_direct_cold(args = ("literal_dense", "direct", false), setup = setup)]
#[bench::literal_dense_direct_warm(args = ("literal_dense", "direct", true), setup = setup)]
#[bench::no_prefilter_direct_cold(args = ("no_prefilter", "direct", false), setup = setup)]
#[bench::no_prefilter_direct_warm(args = ("no_prefilter", "direct", true), setup = setup)]
#[bench::word_direct_cold(args = ("word", "direct", false), setup = setup)]
#[bench::word_direct_warm(args = ("word", "direct", true), setup = setup)]
#[bench::word_late_direct_cold(args = ("word_late", "direct", false), setup = setup)]
#[bench::word_late_direct_warm(args = ("word_late", "direct", true), setup = setup)]
#[bench::nonword_direct_cold(args = ("nonword", "direct", false), setup = setup)]
#[bench::nonword_direct_warm(args = ("nonword", "direct", true), setup = setup)]
#[bench::directional_direct_cold(args = ("directional", "direct", false), setup = setup)]
#[bench::directional_direct_warm(args = ("directional", "direct", true), setup = setup)]
#[bench::lf_direct_cold(args = ("lf", "direct", false), setup = setup)]
#[bench::lf_direct_warm(args = ("lf", "direct", true), setup = setup)]
#[bench::lf_late_direct_cold(args = ("lf_late", "direct", false), setup = setup)]
#[bench::lf_late_direct_warm(args = ("lf_late", "direct", true), setup = setup)]
#[bench::endlf_direct_cold(args = ("endlf", "direct", false), setup = setup)]
#[bench::endlf_direct_warm(args = ("endlf", "direct", true), setup = setup)]
#[bench::end_direct_cold(args = ("end", "direct", false), setup = setup)]
#[bench::end_direct_warm(args = ("end", "direct", true), setup = setup)]
#[bench::end_late_direct_cold(args = ("end_late", "direct", false), setup = setup)]
#[bench::end_late_direct_warm(args = ("end_late", "direct", true), setup = setup)]
#[bench::anchored_direct_cold(args = ("anchored", "direct", false), setup = setup)]
#[bench::anchored_direct_warm(args = ("anchored", "direct", true), setup = setup)]
#[bench::mixed_direct_cold(args = ("mixed", "direct", false), setup = setup)]
#[bench::mixed_direct_warm(args = ("mixed", "direct", true), setup = setup)]
#[bench::churn_direct_cold(args = ("churn", "direct", false), setup = setup)]
#[bench::churn_direct_warm(args = ("churn", "direct", true), setup = setup)]
#[bench::word_bytewise_warm(args = ("word", "bytewise", true), setup = setup)]
#[bench::lf_bytewise_warm(args = ("lf", "bytewise", true), setup = setup)]
#[bench::end_bytewise_warm(args = ("end", "bytewise", true), setup = setup)]
#[bench::mixed_bytewise_warm(args = ("mixed", "bytewise", true), setup = setup)]
#[bench::word_classes_warm(args = ("word", "classes", true), setup = setup)]
#[bench::lf_classes_warm(args = ("lf", "classes", true), setup = setup)]
#[bench::end_classes_warm(args = ("end", "classes", true), setup = setup)]
#[bench::mixed_classes_warm(args = ("mixed", "classes", true), setup = setup)]
#[bench::churn_tiny_cold(args = ("churn", "tiny", false), setup = setup)]
#[bench::churn_tiny_warm(args = ("churn", "tiny", true), setup = setup)]
#[bench::word_short_cold(args = ("word", "short", false), setup = setup)]
#[bench::lf_short_cold(args = ("lf", "short", false), setup = setup)]
#[bench::end_short_cold(args = ("end", "short", false), setup = setup)]
#[bench::mixed_short_cold(args = ("mixed", "short", false), setup = setup)]
#[bench::tier2_short_cold(args = ("tier2", "short", false), setup = setup)]
#[bench::tier3_short_cold(args = ("tier3", "short", false), setup = setup)]
#[bench::tier4_short_cold(args = ("tier4", "short", false), setup = setup)]
fn bench_scan(mut prepared: cases::Prepared) -> bool {
    black_box(prepared.run())
}

#[library_benchmark]
#[bench::literal(args = ("literal_sparse"))]
#[bench::word(args = ("word"))]
#[bench::lf(args = ("lf"))]
#[bench::end(args = ("end"))]
#[bench::mixed(args = ("mixed"))]
#[bench::tier2(args = ("tier2"))]
#[bench::tier3(args = ("tier3"))]
#[bench::tier4(args = ("tier4"))]
fn bench_compile(name: &str) -> usize {
    black_box(cases::Case::named(name).compile(false)).memory_size()
}

library_benchmark_group!(name = assertions, benchmarks = [bench_scan, bench_compile]);

main!(
    config = LibraryBenchmarkConfig::default().tool(
        Callgrind::with_args(["--branch-sim=yes"])
            .format([CallgrindMetrics::Default, CallgrindMetrics::BranchSim])
            .soft_limits([(EventKind::Ir, 5.0), (EventKind::Bcm, 100.0)])
    );
    library_benchmark_groups = assertions
);
