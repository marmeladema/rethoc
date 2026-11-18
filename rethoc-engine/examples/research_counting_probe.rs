//! Process-isolated counting probe. See docs/research/counting-benchmark-method.md.
//!
//! `tier` is `auto` for automatic selection (`MatcherMemory::matcher`), 0-4
//! for a forced tier, or `grouped` for the grouped NFA forced (compiled with
//! `GroupedNfaMode::Always`; `ineligible` when it does not admit the
//! pattern), or `grouped-full` / `grouped-active` for the grouped NFA with
//! one of its two passes forced, or `sparse` for its experimental sparse
//! counting variant (`plans/sparse-counting-sets.md`).  The last column is
//! the engine automatic selection picks.
//!
//! A seventh argument `stats` prints, instead of timings, what one warm
//! scan does: admission, sizes, allocations and the grouped NFA's operation
//! counters (`plans/sparse-counting-sets.md`, S0).  The allocations and the
//! counters need a build with `--features op-stats`; never time that build.

#[allow(dead_code)]
#[path = "common/counting_alloc.rs"]
mod counting_alloc;

#[cfg(feature = "op-stats")]
#[global_allocator]
static ALLOC: counting_alloc::Counting = counting_alloc::Counting;

use std::hint::black_box;
use std::time::{Duration, Instant};

use rethoc_engine::grouped_op_stats::{self, OpStats};
use rethoc_engine::{
    GroupedNfaMode, GroupedScratch, GroupedStream, GroupedSweep, MatcherMemory, Regex, RegexConfig,
    SparseScratch, SparseStream,
};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(
        args.len() == 7 || (args.len() == 8 && args[7] == "stats"),
        "family bound bytes tier chunk default|counters [stats]"
    );
    let stats = args.len() == 8;
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
    let auto = tier_arg == "auto";
    let sparse = tier_arg == "sparse";
    let tier: u8 = if grouped || auto || sparse {
        0
    } else {
        tier_arg
            .parse()
            .expect("tier: auto, 0-4, grouped or sparse")
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
        // The families of plans/sparse-counting-sets.md, §10: a wide, a
        // middle, a narrow and an exact range behind a consuming prefix.
        "range" => format!("x(a|aa|b){{1,{bound}}}z"),
        "rangemid" => format!("x(a|aa|b){{{},{bound}}}z", bound / 2),
        "rangenarrow" => format!("x(a|aa|b){{{},{bound}}}z", bound - bound / 10),
        "rangeexact" => format!("x(a|aa|b){{{bound}}}z"),
        // Sustained entry: new prefixes do not kill the older body paths.
        "sustained" => format!("a(a|aa){{1,{bound}}}z"),
        "sustainedx" => format!("x(a|aa|x){{{},{bound}}}z", bound / 2),
        // Holey count sets (`bound` unused).
        "holey" => "x(a|aaa){2,8}z".to_string(),
        // Two scopes in sequence.
        "seq" => format!("x(a|aa){{1,{bound}}}y(b|bb){{1,{bound}}}z"),
        // A loop around a completed scope (`bound` unused).
        "loop" => "(a{2,5}b)*c".to_string(),
        // Exclusions of the sparse experiment (`nested` is above).
        "nullable" => "(a?){2,5}z".to_string(),
        "unbounded" => format!("x(a|aa){{{bound},}}z"),
        _ => panic!("unknown family"),
    };
    let config = RegexConfig {
        max_estimated_states: 65536,
        max_unroll_states: if counters { 0 } else { 32 },
        merge_repetitions: !counters,
        grouped_nfa: if grouped {
            GroupedNfaMode::Always
        } else {
            GroupedNfaMode::Auto
        },
        sparse_counting: sparse,
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
        // A new `x` every `bound` bytes over `aab`: each episode lasts until
        // the next one starts.
        "range" | "rangemid" | "rangenarrow" | "rangeexact" => {
            for (i, byte) in hay.iter_mut().enumerate() {
                *byte = match (i % bound.max(2), i % 3) {
                    (0, _) => b'x',
                    (_, 2) => b'b',
                    _ => b'a',
                };
            }
        }
        // `x` every third byte: it starts an episode and stays in the body.
        "sustainedx" => {
            for byte in hay.iter_mut().step_by(3) {
                *byte = b'x';
            }
        }
        "holey" | "nullable" | "unbounded" => {
            for byte in hay.iter_mut().step_by(64) {
                *byte = b'x';
            }
        }
        // `x`, `a`s to half the period, `y`, then `b`s.
        "seq" => {
            let period = 2 * bound.max(2);
            for (i, byte) in hay.iter_mut().enumerate() {
                *byte = match i % period {
                    0 => b'x',
                    p if p < period / 2 => b'a',
                    p if p == period / 2 => b'y',
                    _ => b'b',
                };
            }
        }
        "loop" => {
            for byte in hay.iter_mut().skip(3).step_by(4) {
                *byte = b'b';
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
    } else if sparse {
        re.sparse_counting_admission().is_ok()
    } else if auto {
        true
    } else {
        memory.matcher_for_tier(&re, tier).is_ok()
    };
    if stats {
        println!("{}", stats_header());
    }
    if !eligible {
        let reason = match (re.grouped_nfa_admission(), re.sparse_counting_admission()) {
            (Err(e), _) if grouped && stats => format!(": {e}").replace(',', ";"),
            (_, Err(e)) if sparse && stats => format!(": {e}").replace(',', ";"),
            _ => String::new(),
        };
        println!(
            "{family},{bound},{bytes},{tier_arg},{chunk},{},ineligible{reason}",
            args[6]
        );
        return;
    }
    // The grouped NFA: one stream, reset between scans like a reused
    // matcher; its scratch is kept like a `MatcherMemory`.
    let mut scratch = GroupedScratch::new();
    scratch.set_sweep(sweep);
    let mut stream = grouped.then(|| GroupedStream::new(&re, &mut scratch).expect("admitted"));
    // The sparse variant, likewise.
    let mut sparse_scratch = SparseScratch::new();
    let mut sparse_stream =
        sparse.then(|| SparseStream::new(&re, &mut sparse_scratch).expect("admitted"));
    let mut run = || {
        if let Some(stream) = sparse_stream.as_mut() {
            stream.reset(&re, &mut sparse_scratch);
            for part in black_box(&hay).chunks(chunk) {
                stream.chunk(&re, &mut sparse_scratch, part);
            }
            assert_eq!(black_box(stream.finish(&re, &mut sparse_scratch)), expected);
            return;
        }
        if let Some(stream) = stream.as_mut() {
            stream.reset(&re, &mut scratch);
            for part in black_box(&hay).chunks(chunk) {
                stream.chunk(&re, &mut scratch, part);
            }
            assert_eq!(black_box(stream.finish(&re, &mut scratch)), expected);
            return;
        }
        let mut matcher = if auto {
            memory.matcher(&re)
        } else {
            memory.matcher_for_tier(&re, tier).expect("eligible")
        };
        for part in black_box(&hay).chunks(chunk) {
            matcher.chunk(part);
        }
        assert_eq!(black_box(matcher.finish()), expected);
    };
    if stats {
        // One scan to grow every buffer, then the measured one.
        run();
        grouped_op_stats::take();
        let before = counting_alloc::snapshot();
        run();
        let after = counting_alloc::snapshot();
        let ops = grouped_op_stats::take();
        let info = re.info();
        let counters: Vec<String> = info
            .counters
            .iter()
            .map(|c| match c.max {
                usize::MAX => format!("{}..", c.min),
                max => format!("{}..{max}", c.min),
            })
            .collect();
        let (plan, stream_bytes, scratch_bytes) = match (info.grouped_nfa, info.sparse_counting) {
            (
                Some(rethoc_engine::GroupedNfaInfo::Admitted {
                    plan_bytes,
                    stream_bytes,
                    scratch_bytes,
                    ..
                }),
                _,
            ) if grouped => (plan_bytes, stream_bytes, scratch_bytes),
            (
                _,
                Some(rethoc_engine::SparseCountingInfo::Admitted {
                    plan_bytes,
                    stream_bytes,
                    scratch_bytes,
                    ..
                }),
            ) if sparse => (plan_bytes, stream_bytes, scratch_bytes),
            _ => (0, 0, 0),
        };
        let ops: Vec<String> = ops.values().iter().map(u64::to_string).collect();
        println!(
            "{family},{bound},{bytes},{tier_arg},{chunk},{},ok,{},{},{},{plan},{stream_bytes},{scratch_bytes},{},{},{},{}",
            args[6],
            counters.join(";"),
            info.memory.num_states,
            re.memory_size(),
            grouped_op_stats::ENABLED,
            after.allocs - before.allocs,
            after.alloc_bytes - before.alloc_bytes,
            ops.join(",")
        );
        return;
    }
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
        "{family},{bound},{bytes},{tier_arg},{chunk},{},{},{},{compile_us:.3},{cold_us:.3},{runs},{:.3},{:.3},{}",
        args[6],
        re.min_tier(),
        re.memory_size(),
        elapsed * 1e9 / (runs as f64 * bytes as f64),
        elapsed * 1e6 / runs as f64,
        re.engine()
    );
}

/// The CSV header of the `stats` rows.
fn stats_header() -> String {
    format!(
        "family,bound,bytes,tier,chunk,config,status,counters,states,regex_bytes,plan_bytes,\
         stream_bytes,scratch_bytes,instrumented,scan_allocs,scan_alloc_bytes,{}",
        OpStats::FIELDS.join(",")
    )
}
