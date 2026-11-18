//! Process-isolated counting probe. See docs/research/counting-benchmark-method.md.
//!
//! `tier` is 0-4, or `grouped` for the experimental grouped NFA (compiled
//! with `RegexConfig::grouped_nfa`; `ineligible` when it does not admit the
//! pattern), or `grouped-full` / `grouped-active` for the grouped NFA with
//! one of its two passes forced.

use std::hint::black_box;
use std::time::{Duration, Instant};

use rethoc_engine::{
    GroupedScratch, GroupedStream, GroupedSweep, MatcherMemory, Regex, RegexConfig,
};

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
    let tier_arg = args[4].as_str();
    let grouped = tier_arg.starts_with("grouped");
    let sweep = match tier_arg {
        "grouped" => GroupedSweep::Auto,
        "grouped-full" => GroupedSweep::Full,
        "grouped-active" => GroupedSweep::Active,
        other if grouped => {
            panic!("unknown path `{other}`: grouped, grouped-full or grouped-active")
        }
        _ => GroupedSweep::Auto,
    };
    let tier: u8 = if grouped {
        0
    } else {
        tier_arg.parse().expect("tier: 0-4 or grouped")
    };
    let chunk: usize = args[5].parse().expect("chunk");
    let counters = args[6] == "counters";
    let pattern = match family.as_str() {
        "flat" => format!("a.{{{bound}}}b"),
        "phase" => format!("(ab){{1,{bound}}}c"),
        "variable" => format!("(a|aa){{1,{bound}}}b"),
        "prefix" => format!("a(a|aa){{1,{bound}}}b"),
        "nested" => format!("((a|aa){{1,{bound}}}){{1,{bound}}}b"),
        "exact" => format!("a(a|aa){{{bound}}}b"),
        "mixed" => format!("x(a|aa|b){{{},{bound}}}z", bound / 2),
        // Sparse activation: the counted body is rarely entered.
        "sparse" => format!("a(a|aa){{1,{bound}}}b"),
        // Counter-free control (`bound` unused).
        "control" => "abc|abd".to_string(),
        // A wide plan of which the input activates one branch out of 12.
        "wide" => (0..12u8)
            .map(|i| {
                let (p, a) = ((b'A' + i) as char, (b'a' + i) as char);
                format!("{p}({a}|{a}{a}){{1,{bound}}}!")
            })
            .collect::<Vec<_>>()
            .join("|"),
        _ => panic!("unknown family"),
    };
    let config = RegexConfig {
        max_estimated_states: 65536,
        max_unroll_states: if counters { 0 } else { 32 },
        merge_repetitions: !counters,
        grouped_nfa: grouped,
        ..RegexConfig::default()
    };
    let start = Instant::now();
    let re = Regex::with_config(&pattern, config).expect("compile probe");
    let compile_us = start.elapsed().as_secs_f64() * 1e6;
    let mut hay = vec![b'a'; bytes];
    match family.as_str() {
        "phase" => {
            for byte in hay.iter_mut().skip(1).step_by(2) {
                *byte = b'b';
            }
        }
        // A new `x` every 7 bytes over `a`/`b`: many overlapping instances.
        "mixed" => {
            for (i, byte) in hay.iter_mut().enumerate() {
                *byte = match (i % 7, i % 2) {
                    (0, _) => b'x',
                    (_, 0) => b'a',
                    _ => b'b',
                };
            }
        }
        // `A` then `a`s: only the first branch is ever active.
        "wide" => hay[0] = b'A',
        // One `a` every 512 bytes of `q`.
        "sparse" | "control" => {
            for (i, byte) in hay.iter_mut().enumerate() {
                *byte = if i % 512 == 0 { b'a' } else { b'q' };
            }
        }
        _ => {}
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
    let eligible = if grouped {
        re.grouped_nfa_admission().is_ok()
    } else {
        memory.matcher_for_tier(&re, tier).is_ok()
    };
    if !eligible {
        println!(
            "{family},{bound},{bytes},{tier_arg},{chunk},{},ineligible",
            args[6]
        );
        return;
    }
    // The grouped NFA: one stream, reset between scans like a reused
    // matcher; its scratch is kept like a `MatcherMemory`.
    let mut scratch = GroupedScratch::new();
    scratch.set_sweep(sweep);
    let mut stream = grouped.then(|| GroupedStream::new(&re, &mut scratch).expect("admitted"));
    let mut run = || {
        if let Some(stream) = stream.as_mut() {
            stream.reset(&re, &mut scratch);
            for part in black_box(&hay).chunks(chunk) {
                stream.chunk(&re, &mut scratch, part);
            }
            assert_eq!(black_box(stream.finish(&re, &mut scratch)), expected);
            return;
        }
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
        "{family},{bound},{bytes},{tier_arg},{chunk},{},{},{},{compile_us:.3},{cold_us:.3},{runs},{:.3},{:.3}",
        args[6],
        re.min_tier(),
        re.memory_size(),
        elapsed * 1e9 / (runs as f64 * bytes as f64),
        elapsed * 1e6 / runs as f64
    );
}
