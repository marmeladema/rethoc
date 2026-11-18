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
