# AGENTS.md

rethoc: a streaming Thompson-NFA regex engine with counting-DFA tiers.
Cargo workspace, Rust 2024:

- `rethoc-engine/` — the library. Module docs (`//!`) describe each
  module; start with `src/lib.rs` and `src/dfa/mod.rs`.
- `rethoc-cli/` — the `rethoc` debugging CLI (`rethoc --help`).
- `fuzz/` — cargo-fuzz targets (separate workspace, nightly).
- `scripts/` — `capped` (cargo runner), `bless_memory.py`, `build-bisect-bins.sh`.
- `docs/` — design notes; `docs/bugs/` — one post-mortem per bug.

## Invariants

Check every change against these.

1. **Streaming.** Input arrives through `chunk(&[u8])` calls split at
   arbitrary points. All matcher state must survive chunk boundaries.
   Test whole-input and byte-at-a-time feeding.
2. **Untrusted patterns.** Compilation must not panic, hang or use
   unbounded memory. Every new pattern feature needs a compile-time limit.
3. **Untrusted input.** Matching is linear in the input; per-byte work is
   bounded by the compiled pattern, never by input content.
4. **Matching never fails.** No match-time memory, state or time budgets
   that return an error. A blow-up at match time is an engine bug to fix.
   Panicking checks for impossible states are fine.
5. **All tiers agree.** Every eligible tier must give the same answer as
   the NFA (tier 0) and the `regex` crate.

## Tiers

Eligibility is decided per tier at compile time from pattern analysis.
`Regex::min_tier()` is the fastest eligible tier, and
`MatcherMemory::matcher_for_tier()` refuses an ineligible one.

- **0** — NFA simulation. Always available; the reference.
- **1** — lazy DFA, counter-free patterns.
- **2** — differential counters, non-nested fixed-length bodies.
- **3** — conditional transitions, non-nested variable-length bodies.
- **4** — counter programs, nested repetitions.

Repetitions are unrolled up to `max_unroll_states` and same-body
repetitions merged, so a pattern may need counters only with unrolling or
merging disabled. Many counter-tier bugs only show up that way.

## Commands

```bash
cargo test                           # debug build: debug assertions run
cargo test --release
cargo test <substring>               # filter by test name
cargo clippy --all-targets -- -D clippy::all   # must pass
cargo fmt --check                    # default rustfmt, no config
python3 scripts/bless_memory.py --check        # memory: values in match_tests!
```

After a change to the NFA compiler or `State` layout, run
`python3 scripts/bless_memory.py` to update the `memory:` values (it
never touches `min_tier:`; a tier change is a regression to look at).

### Resource caps

Everything runs under a memory and time cap. A regressed engine bug can
allocate without bound and take the machine down with it.

- **Automatic.** `.cargo/config.toml` makes `scripts/capped` the cargo
  runner: every binary run by `cargo test`, `cargo bench` and `cargo run`
  is capped (default 8 GB of virtual memory; 600 s, or 3600 s for
  benches). The compiler is not. cargo-fuzz (ASan) builds are left out.
- **Anything else** goes through the script directly:
  `scripts/capped [--mem KIB] [--time SECS] -- <command>`.
- **Changing the caps:** `RETHOC_CAP_MEM=<KiB>`, `RETHOC_CAP_TIME=<s>`;
  `0` disables one, for example for a long `cargo run -- grep`.
- **When a cap is hit** the script prints a `RESOURCE CAP HIT` banner
  naming the cap (MEMORY or TIME) and the command, and the run fails. A
  memory hit in `cargo test` aborts the whole test binary; rerun the
  tests one by one, or with a filter, to find the culprit.
- `rethoc fuzz --batch` and `rethoc sweep` also cap each of their child
  processes (`--mem-limit`, `--timeout`), so one pattern that blows up
  is counted and the run goes on.

## Debugging with the CLI

```bash
cargo run --release -- info '<pattern>'        # tier, memory, counters
cargo run --release -- match --debug --chunk-size 1 '<pattern>' '<input>'
cargo run --release -- match --tier 3 --unroll-limit 0 --no-merge '<pattern>' '<input>'
cargo run --release -- dump --dfa '<pattern>'  # NFA, classes, tier analysis
```

`--debug` prints matcher state after every chunk. Run the same input with
`--tier 0` and the suspect tier and look for the first step where they
diverge. `--unroll-limit 0 --no-merge` forces counter execution.
Options go after the command name; `rethoc <command> --help` lists them.

## Differential tools

`brute`, `fuzz`, `shrink` and `sweep` check rethoc against the `regex`
crate: the NFA and every eligible tier, each fed whole and byte by byte.
Unless given `--unroll-limit` or `--no-merge`, they compile each pattern
three ways: default, unrolling off, unrolling and merging off. Inputs are
read and printed with byte escapes (`\n`, `\xNN`), and every mismatch
comes with a `repro:` command. Exit codes: 0 clean, 1 mismatch, 2 usage,
3 aborted (watchdog, timeout, OOM, crash), 4 pattern rejected.

```bash
rethoc brute --alphabet 'ab ' --max-len 6 '<pattern>'  # every input up to length 6
rethoc shrink --unroll-limit 0 '<pattern>' '<input>'    # minimise a mismatching input
rethoc fuzz 0 500                                       # seeds 0..500, in process
rethoc fuzz --batch 250 --allow-slow ci/known-slow-seeds.txt 0 9000   # capped children
rethoc fuzz --shared-memory 0 500     # one MatcherMemory for every regex of the run
rethoc sweep nullable-counter         # brute over a pattern family, each run capped
rethoc sweep tier3-assert
rethoc sweep --tier 3 tier3-assert    # only the NFA and Tier 3
rethoc sweep --list tier3-assert      # print the runs instead
```

- `fuzz` seed numbers are stable: a seed is a pattern plus inputs anyone
  can re-run. `--trace` prints every step.
- Seeds that are correct but slower than the watchdog go in
  `ci/known-slow-seeds.txt` with their pattern. `fuzz --batch` reports
  them as `KNOWN-SLOW`, and says when one no longer is.
- Run `fuzz` with at most one job per core (the default): a seed near the
  watchdog limit aborts under load.

## CI

GitHub Actions, in `.github/workflows/`:

- `ci.yml` (every PR and push to `main`): `cargo fmt --check` and
  clippy (both workspaces), `cargo test` in release and debug,
  `bless_memory.py --check`.
- `differential.yml` (same triggers): `rethoc fuzz --batch` over a fixed
  seed range with `ci/known-slow-seeds.txt`, both `rethoc sweep`
  families, and on PRs the `reengage_profile` Callgrind gate against the
  base branch.
- `fuzz-nightly.yml` (daily, or on demand): cargo-fuzz targets.

Third-party actions are pinned by commit SHA with the version in a
comment; Dependabot (`.github/dependabot.yml`) opens weekly PRs to update
them. Everything runs under the caps of `scripts/capped`. CI follows the latest
stable Rust, so a new clippy lint can fail an unrelated PR; fix it there.
Jobs allowed to fail, and steps commented out, say why in the workflow
and name the issue that will re-enable them.

## Bug-fix workflow

1. One bug at a time.
2. Reproduce with the CLI, minimise the pattern and input.
3. Add a regression entry to `match_tests!` (not a handwritten test).
4. Fix, then run the full test suite and clippy.
5. Write `docs/bugs/NNN-short-name.md`: summary (pattern, input,
   expected vs actual, tiers), root cause, how it was found, what made it
   hard, and tooling that would have helped.
6. Fix, test and post-mortem go in one commit. Tooling changes go in
   separate commits. Ask before committing.
7. Before committing a tier or compiler change, run the sweeps and a
   `fuzz --batch` range. They catch what the unit tests do not.
8. New fields on matcher or analysis structs must show up in `Display` /
   `Debug` and `dump.rs`. The `--debug` trace is the main debugging tool.

## Tests

Tests are inline `#[cfg(test)] mod tests` at the bottom of each file. The
data-driven ones use the `match_tests!` macro in `rethoc-engine/src/lib.rs`:

```rust
test_name {
    pattern: r"...",
    memory: 1234,      // compiled size; maintained by bless_memory.py
    min_tier: 2,       // asserted, never blessed
    inputs: [("abc", true), ("ab", false)],
}
```

Each entry checks every eligible tier, whole and byte by byte, against
the `regex` crate. It then re-runs everything with unrolling off, and
with unrolling and merging off (the same three configurations as the
`rethoc` checking commands). So a Tier 3 failure can come from a
`min_tier: 1` entry; the failure message names the configuration.

Tiers 1–3 are also run at the minimum DFA state cap, where the cache is
emptied every few bytes; the failure message names the cap. A behaviour
that switches at a threshold needs tests on both sides of it.

Entries compile with the engine's default compile limit
(`max_estimated_states`), like user code. A pattern estimated above it
needs `max_states: N` (after `unroll_limit:`, if any) and a comment
saying why; `bless_memory.py` reads it too.

**Never write `unroll_limit: 0`**: the macro already runs that
configuration, and setting it skips the default one.

## Design rules

- **Precompute at build time.** Anything derivable from the NFA is
  computed in `RegexBuilder::build()` and stored on `Regex`. Hot paths do
  not recompute invariants.
- **A cache never changes an answer.** The DFA state tables of tiers
  1–3 are caches of something that can always be recomputed. A full
  cache is emptied and refilled (`DfaCache::evict`); a value that cannot
  be stored is never replaced by another one. More generally, a limit
  must not turn into a result.
- **Reuse memory.** Caches and scratch buffers are allocated once and
  reset with `clear()`; no allocation in the per-byte loop. Keep scratch
  `Vec`s' capacity: drain them, do not `mem::take` them.
- **Flat data.** Prefer flat arrays with index ranges or arenas over
  `Vec<Vec<_>>` / `HashMap<_, Vec<_>>`.

## Code style

- Imports in three groups: `std`, third-party, crate-internal. No
  wildcard imports outside test modules.
- Index types are newtypes with `.idx()` and sentinel constants
  (`StateIdx`, `DfaStateId`, …). Diagnostic types end in `Info`.
- Visibility: `pub` only for the public API, otherwise `pub(crate)` /
  `pub(super)` / private.
- Errors: `?` in compilation paths. The CLI exits with a message instead
  of panicking. Tests prefer `.expect("why")`. Use `debug_assert!`
  liberally for internal invariants.
- Docs: `//!` on every module, `///` on public and `pub(crate)` items.
  **Public (`pub`) docs must not mention internals**: no internal type or
  function names, no tier numbers, no NFA/DFA mechanics, no test
  infrastructure. `pub(crate)` docs may.
- Lints: no crate-level lint config. Suppress narrowly with
  `#[expect(lint, reason = "…")]` on the item, never blanket-allow.
- `unsafe` only for SIMD in `memrange.rs` / `memclass.rs`, behind
  `cfg(target_arch)` with a scalar fallback. Every `unsafe` block has a
  `// SAFETY:` comment.
- A struct is immediately followed by its inherent `impl`, then trait
  impls; tests last.

## Benchmarks

Callgrind benches (`flamegraph`, `pathological_profile`,
`reengage_profile`) need `valgrind` and `gungraun-runner` at the version
of the `gungraun` dev-dependency. Criterion benches: `memrange`,
`memclass`, `pathological`, `reengage`. Each bench's header describes it.

`reengage_profile` is the **performance gate**. Run it after any change
to scanning loops, prefilter handling or other engine hot paths. It fails
on regressions above the thresholds set in the bench:

```bash
git checkout main   && cargo bench --bench reengage_profile -- --save-baseline=main
git checkout branch && cargo bench --bench reengage_profile -- --baseline=main
```

## Fuzzing

`rethoc_engine::fuzz_gen` generates a pattern and pattern-aware inputs
from a seed; the oracle is the `regex` crate or the NFA.

```bash
cargo test test_fuzz -- --ignored     # proptest: oracle + cross-tier
cargo +nightly fuzz run fuzz_match         # vs regex crate
cargo +nightly fuzz run fuzz_differential  # tiers vs NFA
cargo +nightly fuzz run fuzz_compile       # compilation robustness
cargo +nightly fuzz run fuzz_match fuzz/artifacts/fuzz_match/<artifact>   # replay
```

The cargo-fuzz targets use the first half of the input as the pattern
seed and the second half as the input seed:

```rust
use rethoc_engine::fuzz_gen::{FuzzRng, generate_inputs, generate_pattern};
let mid = data.len() / 2;
let (pattern, ast) = generate_pattern(&mut FuzzRng::new(&data[..mid]));
let inputs = generate_inputs(&mut FuzzRng::new(&data[mid..]), &ast);
```

Fuzz after any change to the compiler, a tier or counter logic.
