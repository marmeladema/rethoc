//! Process-isolated counting probe. See docs/research/counting-benchmark-method.md.

use std::hint::black_box;
use std::time::{Duration, Instant};

use rethoc_engine::{MatcherMemory, Regex, RegexConfig};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(
        args.len(),
        7,
        "family bound bytes tier chunk default|counters"
    );
    let family = &args[1];
    let bound: usize = args[2].parse().expect("bound");
    let bytes: usize = args[3].parse().expect("bytes");
    let tier: u8 = args[4].parse().expect("tier");
    let chunk: usize = args[5].parse().expect("chunk");
    let counters = args[6] == "counters";
    let pattern = match family.as_str() {
        "flat" => format!("a.{{{bound}}}b"),
        "phase" => format!("(ab){{1,{bound}}}c"),
        "variable" => format!("(a|aa){{1,{bound}}}b"),
        "prefix" => format!("a(a|aa){{1,{bound}}}b"),
        "nested" => format!("((a|aa){{1,{bound}}}){{1,{bound}}}b"),
        _ => panic!("unknown family"),
    };
    let config = RegexConfig {
        max_estimated_states: 65536,
        max_unroll_states: if counters { 0 } else { 32 },
        merge_repetitions: !counters,
        ..RegexConfig::default()
    };
    let start = Instant::now();
    let re = Regex::with_config(&pattern, config).expect("compile probe");
    let compile_us = start.elapsed().as_secs_f64() * 1e6;
    let mut hay = vec![b'a'; bytes];
    if family == "phase" {
        for byte in hay.iter_mut().skip(1).step_by(2) {
            *byte = b'b';
        }
    }
    let oracle = regex::bytes::RegexBuilder::new(&pattern)
        .unicode(false)
        .dot_matches_new_line(true)
        .build()
        .expect("oracle compile");
    let expected = oracle.is_match(&hay);
    assert!(
        !expected,
        "probe must consume a complete unsuccessful stream"
    );
    let mut memory = MatcherMemory::default();
    if memory.matcher_for_tier(&re, tier).is_err() {
        println!(
            "{family},{bound},{bytes},{tier},{chunk},{},ineligible",
            args[6]
        );
        return;
    }
    let mut run = || {
        let mut matcher = memory.matcher_for_tier(&re, tier).expect("eligible");
        for part in black_box(&hay).chunks(chunk) {
            matcher.chunk(part);
        }
        assert_eq!(black_box(matcher.finish()), expected);
    };
    let start = Instant::now();
    run();
    let cold_us = start.elapsed().as_secs_f64() * 1e6;
    let start = Instant::now();
    let mut runs = 0_u64;
    loop {
        run();
        runs += 1;
        if start.elapsed() >= Duration::from_millis(40) {
            break;
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    println!(
        "{family},{bound},{bytes},{tier},{chunk},{},{},{},{compile_us:.3},{cold_us:.3},{runs},{:.3},{:.3}",
        args[6],
        re.min_tier(),
        re.memory_size(),
        elapsed * 1e9 / (runs as f64 * bytes as f64),
        elapsed * 1e6 / runs as f64
    );
}
