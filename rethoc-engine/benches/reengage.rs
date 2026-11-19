//! Criterion benchmarks for prefilter re-engagement in the counting tiers.
//!
//! Wall-clock throughput on streams where a counter has run and expired,
//! where prefilter candidates fail at once, and on control paths.  See
//! `common/reengage_cases.rs` for the cases. `reengage_profile` measures
//! instructions on the bulk fixtures. The local Tier 2 timings also cover
//! bytewise anchored scans and the dense candidate stream at 64 KiB.
//!
//! Run with: `cargo bench --bench reengage`
//! One case: `cargo bench --bench reengage -- tier3_dense_candidates`

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use rethoc_engine::{Engine, MatcherMemory};

#[path = "common/reengage_cases.rs"]
mod cases;

/// Stream length before [`cases::Case::shrink`].
const HAYSTACK_SIZE: usize = 1024 * 1024;

fn bench_reengage(c: &mut Criterion) {
    let mut group = c.benchmark_group("reengage");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(3));

    for case in cases::CASES {
        let (re, haystack) = case.compile(HAYSTACK_SIZE);
        group.throughput(Throughput::Bytes(haystack.len() as u64));
        group.bench_function(case.name, |b| {
            // One memory per case, as a caller would keep it: the DFA
            // cache is warm after the first stream.
            let mut memory = MatcherMemory::default();
            cases::scan(&mut memory, &re, &haystack);
            b.iter(|| black_box(cases::scan(&mut memory, &re, black_box(&haystack))))
        });
    }

    let anchored = cases::Case::named("tier2_anchored_gap_once");
    let dense = cases::Case::named("tier2_dense_candidates");
    // The existing tier2_dense_candidates case covers 1 MiB in 16 KiB chunks.
    for (name, case, size, chunk_size) in [
        (
            "tier2_anchored_gap_once_bytewise_64k",
            anchored,
            64 * 1024,
            1,
        ),
        (
            "tier2_anchored_gap_once_bytewise_1m",
            anchored,
            HAYSTACK_SIZE,
            1,
        ),
        ("tier2_dense_candidates_64k", dense, 64 * 1024, cases::CHUNK),
    ] {
        let (re, haystack) = case.compile(size);
        assert_eq!(re.engine(), Engine::Tier2, "{name}");
        group.throughput(Throughput::Bytes(haystack.len() as u64));
        group.bench_function(name, |b| {
            let mut memory = MatcherMemory::default();
            let mut scan = || {
                let mut matcher = memory.matcher(&re);
                for chunk in black_box(&haystack).chunks(chunk_size) {
                    matcher.chunk(chunk);
                }
                matcher.finish()
            };
            assert!(!scan(), "{name}");
            b.iter(|| black_box(scan()))
        });
    }

    for case in [cases::OverlapCase::Expiry, cases::OverlapCase::MixedLengths] {
        let (re, haystack) = case.compile(HAYSTACK_SIZE);
        group.throughput(Throughput::Bytes(haystack.len() as u64));
        group.bench_function(case.name(), |b| {
            let mut memory = MatcherMemory::default();
            cases::scan_overlap(&mut memory, &re, &haystack);
            b.iter(|| black_box(cases::scan_overlap(&mut memory, &re, black_box(&haystack))))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_reengage);
criterion_main!(benches);
