# Rebar benchmarks

From the rethoc checkout, with Git, Cargo and Python 3.11 or newer:

```sh
scripts/rebar check
scripts/rebar measure
```

Both commands fetch the latest commit on rebar's default branch and build rebar,
its stock `rust/regex` runner, and the rethoc runner in release mode with locked
dependencies. The disposable checkout and generated engine registrations live
in `bench/rebar-integration/`. The registrations select published rebar `grep`
cases with their original patterns, haystacks, and expected counts.

`check` verifies the published matching-line counts for both engines on eight
workloads and rebar's `test/model/grep` smoke test. `measure` runs those checks
before timing the eight workloads. It writes the raw rebar CSV, `rebar cmp`
output, and a Markdown summary into a new results directory. The time ratio is
rethoc's median divided by the stock runner's median. Values below 1 mean
rethoc took less time.

The GitHub workflow runs weekly and on manual dispatch. The job summary has the
per-case times and ratios, the exact rebar and rethoc revisions, the baseline
version, the Rust toolchain, and runner details. Download the workflow artifact
for the raw results. Account for changes in the rebar revision, baseline version,
or runner before comparing timings across runs.

The standalone adapter uses `benchmarks/rebar/Cargo.lock`. If a dependency
change makes its locked build fail, refresh the lockfile in the generated
`bench/rebar-integration/upstream/engines/rethoc/` directory, copy it back here,
and run `scripts/rebar check`.
