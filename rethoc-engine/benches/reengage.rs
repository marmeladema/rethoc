//! Criterion benchmarks for prefilter re-engagement in the counting tiers.
//!
//! Wall-clock throughput on streams where a counter has run and expired,
//! where prefilter candidates fail at once, and on control paths.  See
//! `common/reengage_cases.rs` for the cases.  The same cases are measured
//! in instruction counts, with a regression limit, by `reengage_profile`.
//!
//! Run with: `cargo bench --bench reengage`
//! One case: `cargo bench --bench reengage -- tier3_dense_candidates`

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use rethoc_engine::MatcherMemory;

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

    group.finish();
}

criterion_group!(benches, bench_reengage);
criterion_main!(benches);
