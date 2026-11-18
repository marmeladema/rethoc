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

Eligibility is decided per tier at compile time from pattern analysis,
and so is the engine automatic selection runs (`Regex::engine()`,
`MatcherMemory::matcher()`): Tier 1, Tier 2, the grouped NFA when it
admits the pattern, Tier 4, the NFA, in that order. **Tier 3 is never
selected automatically** (it is unsound, see below); it stays eligible and
`MatcherMemory::matcher_for_tier()` still runs it, for research and
differential testing. `Regex::min_tier()` is the legacy numbered
eligibility query: the lowest-numbered eligible tier, not the selected
engine and not a speed ranking. `matcher_for_tier()` refuses an
ineligible tier.

- **0** — NFA simulation. Always available; the reference.
- **1** — lazy DFA, counter-free patterns.
- **2** — differential counters, non-nested fixed-length bodies whose
  count directly follows a consumed byte: no assertion that looks ahead
  (`\b`, `$`, ...) in between.  `^` / `(?m:^)` may sit there.
- **3** — conditional transitions, non-nested variable-length bodies
  with no assertion that looks ahead inside. **Known to be unsound**, to
  be redesigned rather than patched: `^` / `(?m:^)` inside a body
  (Bug 69, "What is left"), seeds when another counter breaks (the
  `tier3-assert` sweep). Do not add eligibility rules to hide a Tier 3
  defect; record it (see below).
- **4** — counter programs, nested repetitions.

**Grouped NFA** (`rethoc-engine/src/grouped_nfa/`, `plans/grouped-nfa.md`):
not a numbered tier. It keeps one bitset of counts per NFA state instead
of one thread per count, and admits flat counters (no nesting, no
zero-width cycle, frontier within 8,192 words). `RegexConfig::grouped_nfa`
(`GroupedNfaMode`): `Auto` (default) analyses it when neither Tier 1 nor
Tier 2 applies and selects it when admitted; `Disabled` never does;
`Always` also analyses it for Tier 1/2 regexes so that it can be forced
(`MatcherMemory::matcher_grouped()`, `GroupedStream`). It must agree with
the NFA exactly. The tests, the checking commands, the proptests and the
cargo-fuzz targets run it on everything it admits. `MatcherMemory` shared
between regexes on two different tiers among 1–3 panics (issue #5, open).

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
cargo run --release -- match --grouped-nfa --debug --chunk-size 1 '<pattern>' '<input>'  # grouped rows per step
cargo run --release -- info --grouped-nfa '<pattern>'  # selected engine, grouped admission, plan and stream sizes
```

`--debug` prints matcher state after every chunk. Run the same input with
`--tier 0` and the suspect tier and look for the first step where they
diverge. `--unroll-limit 0 --no-merge` forces counter execution.
Options go after the command name; `rethoc <command> --help` lists them.

## Differential tools

`brute`, `fuzz`, `shrink` and `sweep` check rethoc against the `regex`
crate: the NFA, every eligible tier and the grouped NFA when it admits
the pattern (`grouped` in the backend lists, with the rejection reason
when it does not), each fed whole and byte by byte. `--grouped-nfa`
checks only the NFA and the grouped NFA.
Unless given `--unroll-limit` or `--no-merge`, they compile each pattern
three ways: default, unrolling off, unrolling and merging off. Inputs are
read and printed with byte escapes (`\n`, `\xNN`), and every mismatch
comes with a `repro:` command. Exit codes: 0 clean, 1 mismatch, 2 usage,
3 aborted (watchdog, timeout, OOM, crash), 4 pattern rejected.

`brute` and `fuzz` reuse one `MatcherMemory` across inputs, as a user
does across streams, so they also find bugs that need the DFA caches left
by earlier inputs. Such an input passes when run alone: the tools check
that and print a `note:` instead of a `repro:` line. `shrink` runs every
candidate on fresh memory, so what it prints fails on its own.

```bash
rethoc brute --alphabet 'ab ' --max-len 6 '<pattern>'  # every input up to length 6
rethoc shrink --unroll-limit 0 '<pattern>' '<input>'    # minimise a mismatching input
rethoc fuzz 0 500                                       # seeds 0..500, in process
rethoc fuzz --batch 250 --allow-slow ci/known-slow-seeds.txt 0 9000   # capped children
rethoc fuzz --shared-memory 0 500     # one MatcherMemory for every regex of the run
rethoc sweep nullable-counter         # brute over a pattern family, each run capped
rethoc sweep assert-edges             # every assertion kind around a repetition
rethoc sweep --skip-tier 3 assert-in-body   # assertions inside a counted body; Tier 3 has known failures there
rethoc sweep --max-len 4 assert-edges # shorter inputs, e.g. for a debug build
rethoc sweep tier3-assert
rethoc sweep --tier 3 tier3-assert    # only the NFA and Tier 3
rethoc sweep --list tier3-assert      # print the runs instead
rethoc fuzz --grouped-nfa --batch 250 0 9000   # only the NFA and the grouped NFA
```

- `fuzz` seed numbers are stable: a seed is a pattern plus inputs anyone
  can re-run. `--trace` prints every step.
- Two environment variables move engine thresholds for every command,
  child processes included, so that small patterns and short inputs reach
  the code behind them. Summary, result and `repro:` lines carry the
  settings.
  - `RETHOC_DFA_STATES=N`: the DFA state cap. At 8, the minimum, the
    caches are emptied every few bytes.
  - `RETHOC_BYTE_CLASS_MIN_STATES=N`: the NFA size from which the DFA
    tiers use byte equivalence classes (default 128). At 0, every
    pattern gets them.

  `RETHOC_DFA_STATES=8 RETHOC_BYTE_CLASS_MIN_STATES=0 rethoc sweep
  assert-edges` is the "extreme regime" CI runs next to the default.
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
  seed range with `ci/known-slow-seeds.txt` and every `rethoc sweep`
  family, each in the default and in the extreme regime (minimum DFA
  state cap, byte classes for every pattern), and each sweep once more
  on short inputs with a debug build. Sweeps that reach known Tier 3
  defects run twice: without Tier 3 (blocking) and on Tier 3 alone
  (reported, not blocking). On PRs the
  `reengage_profile` Callgrind gate against the base branch.
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
   A wrong answer that is a known Tier 3 design defect and is not being
   fixed gets an entry with `tier3_broken: true`: every other engine is
   checked on it, and Tier 3 must still be wrong on one of its inputs,
   so the marker has to go when the defect does.
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
emptied every few bytes, and every build is repeated with byte
equivalence classes forced on; the failure message names both. A
behaviour that switches at a threshold needs tests on both sides of it.

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
- **One definition, derived tables.** A table derived from the pattern
  (byte classes, prefilter, tier analyses) must come from the definition
  it summarises, not from its own list of cases. What an assertion reads
  from the bytes around it is declared once, in `AssertKind::byte_props`
  (an exhaustive match, checked against `eval` by a test); the byte
  classes are built from that.
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
cargo +nightly fuzz run fuzz_differential  # tiers and grouped NFA vs NFA
cargo +nightly fuzz run fuzz_compile       # compilation robustness
cargo +nightly fuzz run fuzz_match fuzz/artifacts/fuzz_match/<artifact>   # replay
```

cargo-fuzz builds use ASan, whose shadow reservation fails under the
memory cap: run them with `RETHOC_CAP_MEM=0` (time cap only) and
libFuzzer's own `-rss_limit_mb`.

The cargo-fuzz targets use the first half of the input as the pattern
seed and the second half as the input seed:

```rust
use rethoc_engine::fuzz_gen::{FuzzRng, generate_inputs, generate_pattern};
let mid = data.len() / 2;
let (pattern, ast) = generate_pattern(&mut FuzzRng::new(&data[..mid]));
let inputs = generate_inputs(&mut FuzzRng::new(&data[mid..]), &ast);
```

Fuzz after any change to the compiler, a tier or counter logic.
