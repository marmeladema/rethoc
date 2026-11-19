//! Wall-clock benchmarks for Tier 2 compilation and matching.
//!
//! Run `cargo bench --bench tier2` to measure compilation of a counter
//! behind a parked assertion and bytewise scanning with fresh matcher memory.

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rethoc_engine::{Engine, MatcherMemory, Regex};

#[path = "common/scan.rs"]
mod scan;

fn tier2_assertions(c: &mut Criterion) {
    const PATTERN: &str = r"\b(?m:^)a{2,40}";
    // Space and CR permit a word boundary but fail the line-start assertion.
    // The first line-start candidate has one 'a'; the last one has two.
    const INPUT: &[u8] = b" aa\na!\raa\naa";
    let re = Regex::new(PATTERN).expect("guarded counter compiles");
    assert_eq!(re.engine(), Engine::Tier2);

    let mut group = c.benchmark_group("tier2/parked_boundary");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(2));
    group.bench_function("compile", |b| {
        b.iter(|| black_box(Regex::new(black_box(PATTERN)).expect("guarded counter compiles")))
    });
    group.throughput(Throughput::Bytes(INPUT.len() as u64));
    group.bench_function("cold_scan", |b| {
        let scan = |input: &[u8]| scan::scan(&mut MatcherMemory::default(), &re, input, 1, 2);
        let oracle = regex::bytes::Regex::new(&format!("(?s-u){PATTERN}"))
            .expect("oracle accepts guarded counter");
        for (input, expected) in [(&INPUT[..INPUT.len() - 1], false), (INPUT, true)] {
            assert_eq!(oracle.is_match(input), expected);
            assert_eq!(scan(input), expected);
        }
        b.iter(|| black_box(scan(black_box(INPUT))))
    });
    group.finish();
}

criterion_group!(benches, tier2_assertions);
criterion_main!(benches);
