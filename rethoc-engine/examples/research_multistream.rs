//! Multi-stream deployment harness (`plans/sparse-counting-sets.md`, §10;
//! docs/research/counting-benchmark-method.md).
//!
//! Many patterns, each with independent state on every one of many
//! streams, fed chunk by chunk with the streams interleaved: round `r`
//! feeds chunk `r` of stream 0 to every pattern, then of stream 1, ...
//! Compiled programs are shared by all streams.  The grouped NFA and its
//! sparse variant share one worker scratch too; the other engines have no stream type separate
//! from their working memory, so each (stream, pattern) pair owns a
//! `MatcherMemory`.
//!
//! ```text
//! research_multistream ENGINE PATTERNS STREAMS BYTES CHUNK default|counters LIMIT [check] [common]
//! ```
//!
//! `ENGINE` is `auto`, `grouped`, `sparse` (the experimental sparse
//! counting variant of the grouped NFA) or a tier 0-4; `BYTES` is per stream;
//! `LIMIT` is `RegexConfig::max_estimated_states` (2048 is the default).
//! Patterns the compiler or the engine rejects are left out and reported.
//! `check` compares every result with `regex` on the whole input, after
//! the measurements, so the oracle is in neither the time nor the memory.
//! `common` keeps only the patterns that both the grouped NFA and its
//! sparse variant admit, so that the two engines scan the same set.
//!
//! Allocator figures need a build with `--features op-stats`; take times
//! from a build without it.

#[allow(dead_code)]
#[path = "common/counting_alloc.rs"]
mod counting_alloc;

#[cfg(feature = "op-stats")]
#[global_allocator]
static ALLOC: counting_alloc::Counting = counting_alloc::Counting;

use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

use rethoc_engine::{
    AnyMatcher, GroupedNfaInfo, GroupedNfaMode, GroupedScratch, GroupedStream, MatcherMemory,
    Regex, RegexConfig, SparseCountingInfo, SparseScratch, SparseStream, grouped_op_stats,
};

/// The pattern families: the §10 table, its exclusions included.
const FAMILIES: [&str; 13] = [
    "range",
    "rangemid",
    "rangenarrow",
    "exact",
    "sustained",
    "sustainedx",
    "holey",
    "seq",
    "loop",
    "phase",
    "control",
    "nested",
    "unbounded",
];

/// Pattern `i` and its family: families in rotation, then bounds 8, 64,
/// 256 and 1000, then the letters.  One pattern in 16 ends with `z`, which
/// the inputs contain; the others with `!`, which they do not, so those
/// never match and scan every stream to its end.
fn pattern(i: usize) -> (&'static str, String) {
    let family = FAMILIES[i % FAMILIES.len()];
    let n = [8usize, 64, 256, 1000][i / FAMILIES.len() % 4];
    let v = i / (FAMILIES.len() * 4);
    let body = b"abcd";
    let a = body[v % 4] as char;
    let b = body[(v + 1 + v / 4 % 3) % 4] as char;
    let p = b"xy"[v / 12 % 2] as char;
    let q = b"yx"[v / 12 % 2] as char;
    let t = if i.is_multiple_of(16) { 'z' } else { '!' };
    let pattern = match family {
        "range" => format!("{p}({a}|{a}{a}|{b}){{1,{n}}}{t}"),
        "rangemid" => format!("{p}({a}|{a}{a}|{b}){{{},{n}}}{t}", n / 2),
        "rangenarrow" => format!("{p}({a}|{a}{a}|{b}){{{},{n}}}{t}", n - n / 10),
        "exact" => format!("{p}({a}|{a}{a}|{b}){{{n}}}{t}"),
        "sustained" => format!("{a}({a}|{a}{a}){{1,{n}}}{t}"),
        "sustainedx" => format!("{p}({a}|{a}{a}|{p}){{{},{n}}}{t}", n / 2),
        "holey" => format!("{p}({a}|{a}{a}{a}){{2,8}}{t}"),
        "seq" => format!("{p}({a}|{a}{a}){{1,{n}}}{q}({b}|{b}{b}){{1,{n}}}{t}"),
        "loop" => format!("({a}{{2,5}}{b})*{t}"),
        "phase" => format!("({a}{b}){{1,{n}}}{t}"),
        "control" => format!("{a}{b}{p}|{a}{b}{t}"),
        "nested" => format!("{p}(({a}|{a}{a}){{1,10}}{b}){{1,10}}{t}"),
        "unbounded" => format!("{p}({a}|{a}{a}){{5,}}{t}"),
        _ => unreachable!(),
    };
    (family, pattern)
}

/// The input of stream `s`: runs of `a`-`d` of 1 to 32 bytes, single `x`
/// and `y` between them, and a rare `z`.  Deterministic.
fn input(s: usize, bytes: usize) -> Vec<u8> {
    let mut state = 0x9e37_79b9_7f4a_7c15u64 ^ ((s as u64 + 1) * 0xbf58_476d_1ce4_e5b9);
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut out = Vec::with_capacity(bytes);
    while out.len() < bytes {
        let r = next();
        let (byte, run) = match r % 64 {
            0 => (b'z', 1),
            1..=8 => (b'x', 1),
            9..=14 => (b'y', 1),
            k => (b"abcd"[k as usize % 4], 1 + (r >> 8) as usize % 32),
        };
        for _ in 0..run.min(bytes - out.len()) {
            out.push(byte);
        }
    }
    out
}

/// A field of `/proc/self/status`, in kB (0 when it cannot be read).
fn proc_status_kb(field: &str) -> usize {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with(field))
                .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
        })
        .unwrap_or(0)
}

/// The states of every (stream, pattern) pair, pair `s * patterns + p`.
enum Fleet<'a> {
    Grouped {
        scratch: GroupedScratch,
        streams: Vec<GroupedStream>,
    },
    Sparse {
        scratch: SparseScratch,
        streams: Vec<SparseStream>,
    },
    Any(Vec<AnyMatcher<'a>>),
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let flags = &args[8.min(args.len())..];
    assert!(
        args.len() >= 8 && flags.iter().all(|f| f == "check" || f == "common"),
        "engine patterns streams bytes chunk default|counters limit [check] [common]"
    );
    let common = flags.iter().any(|f| f == "common");
    let engine = args[1].as_str();
    let grouped = engine == "grouped";
    let sparse = engine == "sparse";
    let tier: Option<u8> = (!grouped && !sparse && engine != "auto").then(|| {
        engine
            .parse()
            .expect("engine: auto, grouped, sparse or 0-4")
    });
    let num_patterns: usize = args[2].parse().expect("patterns");
    let num_streams: usize = args[3].parse().expect("streams");
    let bytes: usize = args[4].parse().expect("bytes");
    let chunk: usize = args[5].parse().expect("chunk");
    assert!(chunk > 0, "chunk must be at least one byte");
    let counters = match args[6].as_str() {
        "counters" => true,
        "default" => false,
        other => panic!("config `{other}`: default or counters"),
    };
    let limit: usize = args[7].parse().expect("limit");
    let check = flags.iter().any(|f| f == "check");
    println!(
        "config engine={engine} patterns={num_patterns} streams={num_streams} bytes={bytes} \
         chunk={chunk} compile={} max_estimated_states={limit} instrumented={}",
        args[6],
        grouped_op_stats::ENABLED
    );

    // Compile.  Exclusions are counted by family and reason, with the
    // state number of a reason left out so that equal reasons group.
    let defaults = RegexConfig::default();
    let config = RegexConfig {
        max_estimated_states: limit,
        max_unroll_states: if counters {
            0
        } else {
            defaults.max_unroll_states
        },
        merge_repetitions: !counters,
        grouped_nfa: if grouped || common {
            GroupedNfaMode::Always
        } else {
            GroupedNfaMode::Auto
        },
        sparse_counting: sparse || common,
        ..defaults
    };
    let mut excluded: BTreeMap<(&str, String), usize> = BTreeMap::new();
    let mut exclude = |family: &'static str, stage: &str, reason: String| {
        let reason: String = reason.chars().filter(|c| !c.is_ascii_digit()).collect();
        *excluded
            .entry((family, format!("{stage}: {reason}")))
            .or_default() += 1;
    };
    let mut regexes = Vec::new();
    let mut sources = Vec::new();
    let mut selected: BTreeMap<String, usize> = BTreeMap::new();
    let start = Instant::now();
    for i in 0..num_patterns {
        let (family, source) = pattern(i);
        let re = match Regex::with_config(&source, config.clone()) {
            Ok(re) => re,
            Err(e) => {
                exclude(family, "compile", format!("{e:?}"));
                continue;
            }
        };
        let eligible = if grouped {
            re.grouped_nfa_admission().map_err(|e| e.to_string())
        } else if sparse {
            re.sparse_counting_admission().map_err(|e| e.to_string())
        } else if let Some(tier) = tier {
            MatcherMemory::default()
                .matcher_for_tier(&re, tier)
                .map(|_| ())
        } else {
            Ok(())
        };
        if let Err(e) = eligible {
            exclude(family, "engine", e);
            continue;
        }
        if common
            && (re.grouped_nfa_admission().is_err() || re.sparse_counting_admission().is_err())
        {
            exclude(family, "common", "not admitted by both".to_string());
            continue;
        }
        *selected.entry(re.engine().to_string()).or_default() += 1;
        regexes.push(re);
        sources.push(source);
    }
    let compile_ms = start.elapsed().as_secs_f64() * 1e3;
    let e = regexes.len();
    println!("compiled patterns={e} of {num_patterns} compile_ms={compile_ms:.1}");
    println!("automatic selection of the compiled patterns: {selected:?}");
    for ((family, reason), n) in &excluded {
        println!("excluded family={family} patterns={n} {reason}");
    }
    assert!(e > 0, "no pattern left");
    let regex_bytes: usize = regexes.iter().map(Regex::memory_size).sum();
    // Both kinds of plan are reported under their own name, whatever
    // engine runs: `common` compiles both for every engine.
    let (mut grouped_plans, mut sparse_plans) = (0usize, 0usize);
    for re in &regexes {
        let info = re.info();
        if let Some(GroupedNfaInfo::Admitted { plan_bytes, .. }) = info.grouped_nfa {
            grouped_plans += plan_bytes;
        }
        if let Some(SparseCountingInfo::Admitted { plan_bytes, .. }) = info.sparse_counting {
            sparse_plans += plan_bytes;
        }
    }
    println!(
        "memory programs_bytes={regex_bytes} of_which_grouped_plans={grouped_plans} \
         of_which_sparse_plans={sparse_plans}"
    );

    let inputs: Vec<Vec<u8>> = (0..num_streams).map(|s| input(s, bytes)).collect();
    println!("memory input_bytes={}", num_streams * bytes);

    // Stream state, built before any scanning.
    let pairs = num_streams * e;
    let rounds = bytes.div_ceil(chunk);
    let mut latencies = Vec::with_capacity(rounds * num_streams);
    let before = counting_alloc::snapshot();
    let start = Instant::now();
    let mut memories: Vec<MatcherMemory> = Vec::new();
    let mut fleet = if grouped {
        let mut scratch = GroupedScratch::new();
        let streams = (0..pairs)
            .map(|k| GroupedStream::new(&regexes[k % e], &mut scratch).expect("admitted"))
            .collect();
        Fleet::Grouped { scratch, streams }
    } else if sparse {
        let mut scratch = SparseScratch::new();
        let streams = (0..pairs)
            .map(|k| SparseStream::new(&regexes[k % e], &mut scratch).expect("admitted"))
            .collect();
        Fleet::Sparse { scratch, streams }
    } else {
        memories.resize_with(pairs, MatcherMemory::default);
        Fleet::Any(
            memories
                .iter_mut()
                .enumerate()
                .map(|(k, memory)| match tier {
                    Some(tier) => memory
                        .matcher_for_tier(&regexes[k % e], tier)
                        .expect("eligible"),
                    None => memory.matcher(&regexes[k % e]),
                })
                .collect(),
        )
    };
    let setup_ms = start.elapsed().as_secs_f64() * 1e3;
    let built = counting_alloc::snapshot();
    println!(
        "setup pairs={pairs} setup_ms={setup_ms:.1} allocated_live_bytes={} bytes_per_pair={:.1}",
        built.live - before.live,
        (built.live - before.live) as f64 / pairs as f64
    );
    if let Fleet::Grouped { scratch, streams } = &fleet {
        let stream_bytes: usize = streams.iter().map(GroupedStream::memory_bytes).sum();
        println!(
            "memory reported streams_bytes={stream_bytes} bytes_per_pair={:.1} scratch_bytes={}",
            stream_bytes as f64 / pairs as f64,
            scratch.memory_bytes()
        );
    }
    if let Fleet::Sparse { scratch, streams } = &fleet {
        let stream_bytes: usize = streams.iter().map(SparseStream::memory_bytes).sum();
        println!(
            "memory reported streams_bytes={stream_bytes} bytes_per_pair={:.1} scratch_bytes={}",
            stream_bytes as f64 / pairs as f64,
            scratch.memory_bytes()
        );
    }

    // The scan: one latency sample per chunk of one stream, fed to every
    // pattern.
    grouped_op_stats::take();
    let start = Instant::now();
    for round in 0..rounds {
        for (s, input) in inputs.iter().enumerate() {
            let part = &input[round * chunk..bytes.min((round + 1) * chunk)];
            let part = black_box(part);
            let t = Instant::now();
            match &mut fleet {
                Fleet::Grouped { scratch, streams } => {
                    for (p, stream) in streams[s * e..(s + 1) * e].iter_mut().enumerate() {
                        stream.chunk(&regexes[p], scratch, part);
                    }
                }
                Fleet::Sparse { scratch, streams } => {
                    for (p, stream) in streams[s * e..(s + 1) * e].iter_mut().enumerate() {
                        stream.chunk(&regexes[p], scratch, part);
                    }
                }
                Fleet::Any(matchers) => {
                    for matcher in &mut matchers[s * e..(s + 1) * e] {
                        matcher.chunk(part);
                    }
                }
            }
            latencies.push(t.elapsed().as_nanos() as u64);
        }
    }
    let scan = start.elapsed().as_secs_f64();
    let scanned = counting_alloc::snapshot();
    let ops = grouped_op_stats::take();
    let results: Vec<bool> = match fleet {
        Fleet::Grouped {
            mut scratch,
            mut streams,
        } => streams
            .iter_mut()
            .enumerate()
            .map(|(k, stream)| stream.finish(&regexes[k % e], &mut scratch))
            .collect(),
        Fleet::Sparse {
            mut scratch,
            mut streams,
        } => streams
            .iter_mut()
            .enumerate()
            .map(|(k, stream)| stream.finish(&regexes[k % e], &mut scratch))
            .collect(),
        Fleet::Any(matchers) => matchers.into_iter().map(AnyMatcher::finish).collect(),
    };
    let matched = results.iter().filter(|&&m| m).count();
    latencies.sort_unstable();
    let at = |q: f64| latencies[((latencies.len() - 1) as f64 * q) as usize] as f64 / 1e3;
    let total = (num_streams * bytes) as f64;
    println!(
        "scan seconds={scan:.3} ns_per_stream_byte={:.1} ns_per_pattern_byte={:.3} matched_pairs={matched}",
        scan * 1e9 / total,
        scan * 1e9 / (total * e as f64)
    );
    println!(
        "latency per chunk of one stream over all patterns, us: p50={:.1} p99={:.1} max={:.1}",
        at(0.5),
        at(0.99),
        at(1.0)
    );
    println!(
        "memory scan_allocs={} scan_alloc_bytes={} live_growth_bytes={} allocator_peak_bytes={} \
         vm_rss_kb={} vm_hwm_kb={}",
        scanned.allocs - built.allocs,
        scanned.alloc_bytes - built.alloc_bytes,
        scanned.live as isize - built.live as isize,
        scanned.peak,
        proc_status_kb("VmRSS:"),
        proc_status_kb("VmHWM:")
    );
    if grouped_op_stats::ENABLED && (grouped || sparse) {
        let per_byte = |v: u64| v as f64 / (total * e as f64);
        for (name, value) in grouped_op_stats::OpStats::FIELDS.iter().zip(ops.values()) {
            println!("ops {name}={value} per_pattern_byte={:.4}", per_byte(value));
        }
    }

    if check {
        let mut mismatches = 0usize;
        for (p, source) in sources.iter().enumerate() {
            let oracle = regex::bytes::RegexBuilder::new(source)
                .unicode(false)
                .dot_matches_new_line(true)
                .size_limit(1 << 30)
                .build()
                .expect("oracle compile");
            for (s, input) in inputs.iter().enumerate() {
                if oracle.is_match(input) != results[s * e + p] {
                    mismatches += 1;
                    eprintln!("MISMATCH stream={s} pattern=`{source}`");
                }
            }
        }
        println!("check pairs={pairs} mismatches={mismatches}");
        assert_eq!(mismatches, 0, "results differ from the regex crate");
    }
}
