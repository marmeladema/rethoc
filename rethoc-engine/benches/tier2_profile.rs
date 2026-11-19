//! Callgrind gate for the Tier 2 costs of one-byte streaming chunks.
//!
//! Run `cargo bench --bench tier2_profile`. The bytewise cases share their
//! pattern and input with the Tier 2 Criterion controls in the assertion suite.

use std::hint::black_box;

use gungraun::{
    Callgrind, CallgrindMetrics, EventKind, LibraryBenchmarkConfig, library_benchmark,
    library_benchmark_group, main,
};

#[path = "common/assertion_cases.rs"]
mod cases;

fn setup(warm: bool) -> cases::Prepared {
    cases::Prepared::new("tier2", "bytewise", warm, 64 * 1024)
}

#[library_benchmark]
#[bench::bytewise_cold(args = (false), setup = setup)]
#[bench::bytewise_warm(args = (true), setup = setup)]
fn bench_scan(mut prepared: cases::Prepared) -> bool {
    black_box(prepared.run())
}

library_benchmark_group!(name = tier2, benchmarks = bench_scan);

main!(
    config = LibraryBenchmarkConfig::default().tool(
        Callgrind::with_args(["--branch-sim=yes"])
            .format([CallgrindMetrics::Default, CallgrindMetrics::BranchSim])
            .soft_limits([(EventKind::Ir, 5.0), (EventKind::Bcm, 100.0)])
    );
    library_benchmark_groups = tier2
);
