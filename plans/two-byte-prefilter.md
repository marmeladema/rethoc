# Two-byte range prefilter

## Target and baseline

Implement a compile-time-selected bulk search for two consecutive inclusive
byte ranges. Use the existing matcher to verify each candidate. The motivating
case is rebar's `imported/lh3lh3-reb/date`. The first byte is an ASCII digit.
The next byte is either a slash or a digit, one contiguous range in ASCII.

Run from the repository root. The baseline at
`3e891d8d85c3afcae116bc315f29b980bbe50e4e` was checked with:

```sh
date_pattern='([0-9][0-9]?)/([0-9][0-9]?)/([0-9][0-9]([0-9][0-9])?)'
cargo run --release -- info "$date_pattern"
cargo run --release -- dump "$date_pattern"
```

It selects Tier 1, reports `Range(48, 57)`, and has 14 NFA states. The
initial digit state leads to a split between another digit and a slash. The
candidate build must still select Tier 1 and must report a pair with endpoints
`(48, 57)` and `(47, 57)`. No parser, tier eligibility, stream API, or
user configuration change is needed.

## 1. Specify and test the scanner

Add a crate-private range-pair search in `rethoc-engine/src/memrange.rs`.
Give the four inclusive endpoints meaningful first/second names. Callers supply
valid ranges. For a slice of length n the result is the **smallest** index i
such that:

```text
i < n && A.contains(hay[i]) && (i + 1 == n || B.contains(hay[i + 1]))
```

Return None if no such index exists. Return the start, even when B also
belongs to A. A trailing A is an ordinary candidate: the matcher consumes it
now and retains the resulting state across chunks. This covers empty chunks,
byte-at-a-time input and end of stream without storing an extra byte.

Implement the scalar path first. For SIMD, use two unaligned loads starting at
i and i+1, test each against its unsigned range and AND corresponding lanes.
Only load when at least W+1 bytes remain, advance by W on a miss, and apply the
scalar contract to the entire remainder. This includes the pair crossing a
vector boundary. Reuse the existing x86 range predicates and first-set-bit
logic (including bit 31 of an AVX2 result). Keep baseline SSE2 and runtime
AVX2 detection. On NEON, combine comparison vectors before extracting the
first lane; the existing `test_neon` returns a position too early to combine
two results. Preserve the existing target cfgs and scalar fallback. Document
feature availability and both load bounds at each unsafe block. Avoid changes
to the generated single-range hot loop unless the profiles verify them.

Test each backend available on the host against an independent scalar
specification. Exercise the short and long dispatch paths and, on AVX2 hosts,
exercise SSE2 as well. Use unaligned subslices, lengths around 0, 1, 16, 17,
32, 33 and two vector widths, hits in each lane, competing hits, overlapping
hits, A and B in unrelated lanes, tail-only hits, and no hit. Include range
endpoints 0, 127, 128 and 255. Run native backend tests on x86-64 and AArch64
where available and identify any backend only cross-compiled or untested.

## 2. Derive a sound pair from the compiled NFA

Work in `rethoc-engine/src/prefilter.rs`, after the existing
`RegexBuilder::build` empty-match guard in `lib.rs`. Compute the existing
single-byte choice first. Attempt the two-depth analysis only for
`Prefilter::Range(lo, hi)`. Keep this value as the fallback. Today this means
more than eight accepted bytes with `hi - lo < 16`; holes in that interval
already produce allowed false positives.

- Walk from the same NFA start, visiting states by (state index, number of
  consumed bytes), at depths 0 and 1. At either depth follow both arms of a
  Split without consuming. If Match, Assert, CounterInstance or
  CounterIncrement is reachable before the second consumption, abort the
  refinement and return the baseline. Keep the original counter and assertion
  behavior of the single-byte analysis. States *after* the second consumption
  are irrelevant.
- At depth 0, enqueue the successors after consuming one byte. Byte, ByteCI,
  Wildcard, ByteClassStatic and ByteClassCustom have both `out` and an
  optional fused `out_exit`. Follow both valid edges. A ByteTable has one
  destination per byte; enqueue every non-NONE destination. Skipping an exit
  can hide either a valid second byte or a path accepting after only one byte.
- At depth 1, collect the union of accepted bytes from all consuming states.
  Share the byte collection logic with the existing first-byte analysis. Use
  the compiled class tables, both ByteCI values, and each occupied ByteTable
  slot. The exhaustive edge handling in
  `grouped_nfa/analysis.rs::for_each_successor` is a reference for outgoing
  edges. Keep epsilon and consuming steps distinct.
- Accept B when its union is nonempty and its max minus min is less than 16,
  including a singleton. Store the endpoints. A and B are independent
  supersets: losing branch correlation can introduce candidates, but every
  real match still satisfies A followed by B. No runtime tables are needed.
  Deduplicate states on enqueue separately at each depth. Scratch is O(N)
  for N compiled states, with at most 256 entries inspected per visited
  consuming state. Do not enumerate paths, expand repetitions, or build a
  256-by-256 pair table.

Add a compact `Prefilter::RangePair` variant with both ranges. Update Display
and the module and method docs. `Regex::info()` uses Display and `dump.rs`
uses Debug; check both outputs. Keep the single-byte variants and their
selection tests intact.

## 3. Connect the existing restart paths

| Entry point in rethoc-engine/src | Required behavior |
| --- | --- |
| `prefilter.rs::with_prefilter!` and `Prefilter::find` | Dispatch the new variant to the pair scanner. Preserve one dispatch per chunk in the macro users. |
| `Prefilter::accepts` and `find_in_prefix` | Single-byte membership tests A only. The immediate probe checks first-byte membership for a pair. Bulk scanning checks both positions. A probe miss leaves the full slice to the finder. |
| `lib.rs` NFA, `dfa/tier1.rs::scan`, `dfa/tier2/mod.rs` | Use existing skip points through the macro. Preserve Tier 1's START and tagged restart handling, both byte-column paths, and Tier 2's singleton and immediate-probe paths. |
| `dfa/tier3/mod.rs::chunk` | Add the arm in its separate exhaustive dispatch. Pass membership in A for the current-byte shortcut and the pair scanner for the bulk finder. Stepping a conservative candidate is sound. |
| `grouped_nfa/exact.rs`, `grouped_nfa/bitmap.rs::next_start`, `grouped_nfa/sparse.rs` | Use the existing direct and shared find paths only at their existing empty-frontier skip points. Preserve start-table checks, previous byte, start-of-stream flag, and skip accounting. |
| Tier 4 | Covered by cross-tier matching tests. It has no prefilter dispatch to extend. |

Leave ordinary steps with live threads, counters, or pending assertions alone.
Update comments that assume a skipped byte could never start a thread. A first
byte may start a transient thread, but the known second byte rules out every
match from that start; the unconfirmed trailing first byte is never skipped.

## 4. Prove selection and streaming behavior separately

Use inline tests in `prefilter.rs` for selection. Add the pair to
`singleton_membership_agrees_with_slice_search`. Verify these cases with
normal settings unless specified:

| Pattern | Expected result |
| --- | --- |
| Exact date fixture | Pair for digits followed by slash-through-nine. |
| `[0-9]{1,2}/` | Pair allowing both a digit and slash at offset one, including the fused exit when emitted. |
| `[0-9]{1,2}` | Single range because a path accepts after the first digit. |
| `0a\|1b\|2c\|3d\|4e\|5f\|6g\|7h\|8i\|9j` (regex alternation) | Digit range followed by a through j. Verify the compiled start ByteTable and a match on each branch, including the last destination. |
| `[0-9]\b/`, `[0-9][az]` | Original digit range for an assertion or an overly broad second span. |
| `^[0-9]/`, `[0-9]?` | Existing None for the leading assertion or empty match. |
| `[0-9]{2,5}/` with unrolling disabled | Original digit range because a counter is reached within the first two positions. |

Use `match_tests!` for end-to-end examples and the existing
`match_tier_chunked` / `test_prefilter_reengagement_stream_differential`
helpers for arbitrary cuts. Assert selection in the focused tests so a silent
fallback cannot make them pass. Compare to the regex oracle and to an
otherwise identical build with its internal prefilter set to None before
matcher construction. Use independent matcher memory for each build/backend.
Include a date at offset zero, one after several failed digit starts, a
truncated date, both year lengths, and negatives. Exercise every single cut
of the short fixtures, empty chunks, bytewise feeding, and a valid final
candidate spanning chunks after a long idle prefix. Check finish and existing
intermediate match queries under identical chunking.

The date pattern covers Tier 1 and NFA. Use `[0-9]/x{2,5}z` with unrolling
and merging disabled for Tier 2 and `[0-9]/(ab|c){2,5}z` for variable-length
Tier 3. Both admit grouped execution with that configuration. Force eligible
tiers and grouped/sparse execution with their existing test configuration;
check admission before claiming coverage. These counters occur *after* the
pair. Exercise candidate failure followed by a later match, with counters
live across a cut. Include a suffix assertion after the second byte.

## 5. Run gates and measure

1. Run `cargo fmt --check`, `cargo clippy --all-targets -- -D clippy::all`,
   `cargo test`, `cargo test --release`, and
   `python3 scripts/bless_memory.py --check`. Inspect any memory changes
   before blessing. Preserve tier expectations.
2. Run the sweep matrix in `.github/workflows/differential.yml` with its
   exact per-family tier switches, normal and extreme release regimes, and
   short debug inputs. Build the release CLI and run the workflow's
   `scripts/capped --time 1800 target/release/rethoc fuzz --batch 250 --allow-slow ci/known-slow-seeds.txt 0 9000`
   normally and with `RETHOC_DFA_STATES=8 RETHOC_BYTE_CLASS_MIN_STATES=0`.
   Also run its separate sparse-counting batch. Compare known Tier 3 failures
   with the base before attributing a new discrepancy to them.
3. Follow the same workflow's Callgrind job with a separate base worktree and
   build directories, Valgrind and the matching gungraun-runner. Save the base
   with `--save-baseline=main`, copy its `target/gungraun/` measurements,
   then use `--baseline=main` for the candidate. Run `reengage_profile` and
   the Tier 1 assertion and Tier 2 controls as in CI.
4. Measure from separate base and candidate worktrees using
   `scripts/rebar measure` in each, sequentially, in base/candidate then
   candidate/base order. It checks published grep counts on nine cases and
   times eight. The date fixture uses the lossy-UTF-8 HOWTO haystack and
   expects 668 matching lines at the inspected upstream definition. The
   adapter reuses matcher memory and feeds one whole line per match attempt.
   The wrapper fetches upstream on each run: compare the rebar revision,
   toolchain, stock runner version and host in the summaries; repeat any
   mismatched comparison. Save each results directory. Compare the
   `engine=rethoc` median for the same case across revisions in `results.csv`.
   The printed ratio compares two different engines within one run.
5. Report per-run date medians, candidate/base ratios, seven control cases,
   and Callgrind gate results. Ship automatic selection only if both run
   orders favor the candidate beyond observed baseline variation, controls
   have no reproducible slowdown beyond their variation, and correctness and
   performance gates pass. Repeat noisy cases before deciding. If the gain
   is absent, report the result and leave the existing selection in place.
   A three-byte filter for `digit slash | digit digit slash` is a separate
   experiment. This plan's success criterion is measured on the two-byte
   implementation.

## Prototype results

On AArch64 with rebar `463d00f31887e84c38467805b9e3122c314b9521`,
the date median was 25.88 ms at baseline, 35.99 ms with the initial
pair scanner, and 21.55 ms after optimizing the NEON scan and its tail.
All nine published count checks passed. The prototype has not met the
release gate. Callgrind found Tier 2 instruction regressions on the
existing single-literal filter. A trial specialization made the gate
results worse and was removed. Keep the prototype on this branch until
the control regressions are resolved and the performance checks pass.

Callgrind and AArch64 disassembly isolated the regression to Tier 2's
immediate prefix probe. The added second-byte check caused LLVM to emit
an out-of-line call where the baseline inlined the probe. Forcing inlining
alone left dense candidates at +5.28% instructions. Restoring first-byte
membership for the immediate probe and forcing inlining passed all 25
reengage cases. Dense candidates and mixed gap text each execute 16 more
instructions than baseline. Bulk searches still apply both byte ranges.
Hardware perf events were unavailable on the benchmark host.

The Callgrind-passing build measured 23.45 ms on the date workload,
9.0% below the repeated baseline of 25.77 ms. All nine published counts
passed again.
