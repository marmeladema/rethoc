//! Newly supported assertions, measured separately from existing-case gates.
//!
//! Run `cargo bench --bench tier1_assertions_new` after Tier 1 admission.
//! Each workload compares forced NFA and Tier 1 against the same oracle-checked
//! input. These cases have no old Tier 1 performance baseline: that engine was
//! either ineligible or incorrect. Warm caches are reused across streams.

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rethoc_engine::{MatcherMemory, Regex};

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("tier1_assertions_new");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(2));
    let cases: &[(&str, &str, &[u8], &[u8])] = &[
        ("crlf_lines", r"(?Rm)^foo$", b"fooX\r\nfooX\r", b"\rfoo\r\n"),
        ("crlf_end", r"foo(?Rm:$)", b"fooX\r\nfooX\r", b"foo\r\n"),
        ("crlf_chain", r"\b(?Rm:^)foo", b"xfooX\r\n", b"\rfoo"),
        ("repaired_lf_chain", r"\b(?m:^)foo", b"xfooX\r\n", b"\nfoo"),
        ("repaired_eof", r"a$\B", b"aaa", b"a"),
        ("repaired_empty", r"\B(?m:^)", b"", b""),
    ];
    for &(name, pattern, repeated, tail) in cases {
        let mut input: Vec<_> = repeated.iter().copied().cycle().take(64 * 1024).collect();
        input.extend_from_slice(tail);
        let re = Regex::new(pattern).expect("new assertion pattern compiles");
        let oracle = regex::bytes::Regex::new(&format!("(?s-u){pattern}"))
            .expect("oracle accepts assertion pattern");
        let expected = oracle.is_match(&input);
        group.throughput(if input.is_empty() {
            Throughput::Elements(1)
        } else {
            Throughput::Bytes(input.len() as u64)
        });
        for tier in [0, 1] {
            for chunk_size in [1, 16 * 1024] {
                let mut memory = MatcherMemory::default();
                let scan = |memory: &mut MatcherMemory| {
                    let mut matcher = memory
                        .matcher_for_tier(&re, tier)
                        .expect("new assertions must be eligible on forced engine");
                    for chunk in input.chunks(chunk_size) {
                        matcher.chunk(chunk);
                    }
                    matcher.finish()
                };
                assert_eq!(
                    scan(&mut memory),
                    expected,
                    "{name}/tier{tier}/{chunk_size}"
                );
                group.bench_function(format!("{name}/tier{tier}/chunk{chunk_size}"), |b| {
                    b.iter(|| black_box(scan(&mut memory)))
                });
            }
        }
    }
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
