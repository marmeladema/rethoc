# Counting benchmark method

The repository keeps the probe and methodology, not machine-specific timing
results. Run comparisons on the target hardware with the same compiler,
dependencies, build settings, and workload. Store output under ignored `target/`
or as benchmark/CI artifacts outside the source tree.

## Run locally

From the repository root:

```sh
cargo build --release -p rethoc-engine --example research_counting_probe
mkdir -p target/research-counting
python3 scripts/research_counting_probe.py > target/research-counting/results.csv
rustc -Vv > target/research-counting/compiler.txt
git rev-parse HEAD > target/research-counting/revision.txt
```

For the grouped NFA comparison (`plans/grouped-nfa.md`, §8), run
`python3 scripts/research_counting_probe.py --grouped` instead: every
family and bound on tiers 0-4 and on the grouped NFA (`tier` column
`grouped`), under forced counters and default compilation, plus the
`exact`, `mixed`, `sparse` (rare activation) and `control` (counter-free)
families, and `wide` (12 counted alternatives, one kept alive).  Path
`auto` runs automatic selection (`MatcherMemory::matcher`); the last CSV
column is the engine it picks.  Paths
`grouped-full` and `grouped-active` force one of the grouped NFA's two
passes.  The grouped NFA is timed on one stream reset between scans, with
its scratch kept, like a reused `MatcherMemory`.

### Sparse counting experiment (`plans/sparse-counting-sets.md`)

`python3 scripts/research_counting_probe.py --sparse` runs the families of
the plan's §10 under forced counters, 64 KiB per scan: `range`, `rangemid`,
`rangenarrow` and `rangeexact` (`x(a|aa|b){l,N}z`, a new `x` every `N`
bytes), `sustained` and `sustainedx` (new entries do not kill the older
ones), `mixed` (short-lived entries), `seq`, `holey`, `loop`, `phase`,
`control`, and the exclusions `nested`, `nullable` and `unbounded`. Path
`sparse` is the sparse counting variant forced (`ineligible` where it does
not admit the pattern); `sparse-full` and `sparse-active` force one of its
two passes, which also turns off its idle skip and count-free step. The set also times four chunk sizes at bound 1000
and default compilation as the end-to-end competitor; `--reps 3 --seed 7`
runs every case three times in a shuffled, reproducible order.

Timings say how long a scan takes, not what it does. For that, build the
probe a second time with the engine's `op-stats` feature and add `--stats`:

```sh
cargo build --release -p rethoc-engine --examples --features op-stats
python3 scripts/research_counting_probe.py --sparse --stats > target/research-counting/stats.csv
```

Each row is one warm scan: admission (with the rejection reason), the
compiled counters, plan, stream and scratch bytes, the allocations made
during the scan, and the grouped NFA's operation counters
(`rethoc_engine::grouped_op_stats::OpStats`): bytes per pass, words of
count rows copied, ORed, shifted and cleared, predicate tests, rows
written, and the size of the count sets after every byte (rows, words,
values, runs of consecutive values). The counters and the counting
allocator exist only in that build and slow it down: **never take a time
from a build with `op-stats`**, and rebuild without the feature before
timing (`target/release/examples` holds whichever was built last).

### Many patterns on many streams

`research_multistream ENGINE PATTERNS STREAMS BYTES CHUNK default|counters LIMIT [check]`
keeps independent state for every pattern on every stream and feeds the
streams chunk by chunk, interleaved. The patterns are the families above
with rotating bounds (8, 64, 256, 1000) and letters; the inputs are runs of
`a`-`d` with single `x`, `y` and a rare `z`, fixed by the stream number.
One pattern in 16 can match; the others scan every stream to its end.

```sh
scripts/capped --mem 6000000 --time 400 -- \
  target/release/examples/research_multistream grouped 520 32 16384 1500 counters 65536 check
```

It reports, separately: patterns rejected by the compiler (`LIMIT` is
`max_estimated_states`; 2048 is the default) or by the engine, with the
reason; bytes of the compiled programs; bytes of the inputs; stream state
as the engine reports it and as the allocator saw it (`op-stats` build);
allocations and growth during the scan; resident memory; time per byte
and the latency of one chunk of one stream over all patterns. `check`
compares every result with `regex` on the whole input after the
measurements, so the oracle is in neither the time nor the memory.

The grouped NFA and its sparse variant (engine `sparse`) share the
compiled plans and one worker scratch between all streams. The other engines have no stream type apart from
`MatcherMemory`, so the harness gives every (stream, pattern) pair its
own: their memory figures measure that API, not the least an engine needs.

Also record CPU/architecture, OS, dependency resolution, build configuration,
CPU affinity, and relevant machine load with the results. Cargo.lock is ignored
by this repository; retain a copy with local results to identify dependencies.
Do not compare runs on different machines as though they measure a code change.

The driver isolates each configuration through
`scripts/capped --mem 1000000 --time 5`. Report timeouts and rejected/ineligible
cases alongside timings. These protect the experiment, not the matcher API.

## Workloads and measurements

The families are `a.{N}b`, `(ab){1,N}c`, `(a|aa){1,N}b`,
`a(a|aa){1,N}b`, and `((a|aa){1,N}){1,N}b`. They vary count bounds,
body length/ambiguity, consuming prefixes, nesting, input lengths, and chunks.
Compare all eligible paths rather than treating tier order as a speed ranking.

The Rust example compiles once per process and reuses one `MatcherMemory`.
It times the first complete scan separately, then repeats scans until at least
40 ms has elapsed. Answers are checked against `regex::bytes::Regex`; oracle
matching is outside scan timing. Inputs are non-matches, avoiding successful
early returns. All-a input exercises counted bodies; the phase family uses
alternating ab bytes. These are synthetic dense-activation workloads, not a
representative traffic distribution.

The eligibility check constructs and drops a matcher before timing. Therefore
`cold_us` excludes some initial setup. Warm timings include matcher construction/
reset, chunk calls, finalization, and a result assertion; they are not isolated
inner-loop timings. Some slow configurations may complete only one warm scan.

`counters` disables unrolling and repetition merging. `default` retains their
defaults, but both modes raise `max_estimated_states` to 65536. Thus the latter
is not entirely default compilation. The repetition cap remains 1000. `tier`
forces an eligible path; `min_tier` records the normal dispatch choice.

`compiled_bytes` comes from `Regex::memory_size()`. It is not per-stream memory
and does not account for every heap allocation inside tier analyses. This probe
does not measure matcher allocation or full deployment memory.

## Interpretation and extensions

Treat this as an exploratory cost probe. Repeat measurements under controlled
conditions and report variability before drawing performance conclusions.
Timing changes alone do not establish an asymptotic bound or identify which
data structure caused the cost. Correct answers here are not a correctness proof.

For design decisions, add sparse/irregular activation, mixed predicates, nested
bodies that do not algebraically flatten, and early/late success. Measure live
configurations, set operations, cache churn, allocations, and instruction counts.
Compare default unrolling as well as forced counters on identical input.

Production evaluation also needs hundreds of rules across many interleaved
streams, complete stream/scratch/compiled memory, startup and reset costs, and
adversarial chunk latency. Keep numerical results with their execution metadata
as local or CI artifacts, not fixed claims in the research documents.
