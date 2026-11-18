//! Wall-clock companions to the assertion instruction gate.
//!
//! Run `cargo bench --bench tier1_assertions`; Criterion filters select individual
//! families. Cold measurements allocate fresh matcher memory per stream, warm
//! measurements reuse a cache primed in setup. Compilation is measured separately.

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rethoc_engine::MatcherMemory;

#[path = "common/tier1_assertion_cases.rs"]
mod cases;

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("tier1_assertions");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(2));
    for case in cases::CASES {
        for mode in ["direct", "bytewise", "classes", "tiny", "short"] {
            let mut prepared = cases::Prepared::new(case.name, mode, true, 256 * 1024);
            group.throughput(Throughput::Bytes(prepared.input.len() as u64));
            group.bench_function(format!("{}/{mode}/warm", case.name), |b| {
                b.iter(|| black_box(prepared.run()))
            });
            group.bench_function(format!("{}/{mode}/cold", case.name), |b| {
                b.iter(|| {
                    let mut memory = MatcherMemory::default();
                    if mode == "tiny" {
                        memory.set_dfa_state_cap(Some(8));
                    }
                    black_box(cases::scan(
                        &mut memory,
                        &prepared.regex,
                        &prepared.input,
                        prepared.chunk,
                        prepared.tier,
                    ))
                })
            });
        }
    }
    group.finish();
    let mut group = c.benchmark_group("tier1_assertions_compile");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(2));
    for case in cases::CASES {
        group.bench_function(case.name, |b| b.iter(|| black_box(case.compile(false))));
    }
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
