# Tier 1 assertion implementation record

Status: implemented and verified, including the final private cache-layout
adjustment. Full suites, blocking sweeps, batch fuzzing, and isolated runtime
performance gates passed. The measured near-limit compilation cost is accepted.

Design: [complete Tier 1 assertion support](../tier1-assertions.md).
The earlier [bounded design audit](tier1-assertions.md) remains separate evidence;
its model results are not counted as production-engine tests here.

Implementation base: `e654f78` on `main`. The branch was rebased onto that
revision before engine changes. Benchmark environment: aarch64, Rust 1.100,
Valgrind 3.25.1, and gungraun-runner 0.20.0. The original measured engine was
commit `c25c3a81359f8214f08e487580eacf27ea30483a`; measurements were captured
before that commit. The first review follow-ups amended the engine to
`2c9a3b974bdf1ad05e89bd3a273740a50b14b0a3` with diagnostic-only changes.
The [performance artifact](tier1-performance.json) records
source and executable hashes, commands, fixture parameters, confidence
intervals, and individual results. The retained source hashes identify the
measured engine, benchmark fixtures, and CLI tooling. The
[validation artifact](tier1-validation.json) identifies the tested binaries
and distinguishes completed checks from verification after later edits.

## Implemented behavior

Tier 1 now admits every assertion kind accepted by the compiler when the
compiled graph is counter-free, including `StartCRLF` and `EndCRLF`. Automatic
selection chooses Tier 1 for counter-free CRLF patterns. CRLF patterns with
retained counters keep the existing grouped/NFA fallback behavior, and the
counting tiers' assertion eligibility and evaluation policies are unchanged.

The prerequisite fixes are documented separately:

- [Bug 70: deferred boundary context](../../docs/bugs/070-tier1-deferred-boundary-context.md),
  including `\b(?m:^)a` on `a` and `\B(?m:^)` on empty input.
- [Bug 71: exact EOF chains](../../docs/bugs/071-tier1-eof-assertion-chain.md),
  including false positives for `a$\B`, `$\b`, and `$^`.

`Regex` now stores `Box<Tier1Assertions>` without an `Option`. Assertion-free
and counter-bearing graphs have an empty entries slice; the outer owner is
still allocated. Matching selects the assertion-free specialization for an
empty table and directly borrows populated analysis without `.expect()`.
The latest memory and performance measurements are recorded separately below.

The implementation follows the plan's boundary invariants:

1. A single side-specific assertion dependency declaration supplies previous
   and next byte properties and input-start dependence. Byte classes derive
   their combined requirements from it.
2. Pending-context requirements include the complete zero-width continuation,
   stopping at consuming states and at End for known-next-byte resolution.
3. Provisional closure computes exact EOF acceptance from its raw roots using
   the actual boundary context, before canonicalizing the cached context.
4. Known-next-byte closure evaluates the pending chain at its original
   boundary, consumes the incoming byte once, and computes a new boundary.
   It does not carry an old EOF hypothesis across consumption.
5. Cached context retains only the required WORD/LF/CR/start facts. Ordinary
   word-only assertions retain wordness; EndLF alone needs no previous-byte
   context. Tier 1 `finish()` reads the exact cached EOF flag.

Analysis has three exact construction paths. Isolated assertions with no
Splits use direct metadata construction. A graph whose every epsilon edge is
verified to point to a greater state index uses one reverse index scan. Other
graphs use flat reverse adjacency and finite-bit worklists: four dependency
bits and five EOF-acceptance bits. The general EOF algorithm propagates the
five bits together through precomputed evaluator-derived assertion gates.
Cycles terminate at the least fixed point. Tests compare optimized analysis
with the general algorithm and an independent forward EOF walk.

Counter-free patterns no longer build the old optimistic reachability table
or its per-counter break summary: Tier 1 has no remaining consumer of them.
This removes a redundant reverse-graph construction and one retained byte per
state. Patterns with retained counters still build both tables, including
patterns whose automatic selection uses another engine. Forced counting-tier
behavior therefore keeps its existing analyses.

The normal cached scanning loops keep their existing table-lookup structure.
Transition misses reuse target scratch. Repeated transitions from DEAD now
have a lazily allocated cache row, which is cleared on eviction and regex
changes. This avoids repeatedly rebuilding the same restart closure while
preserving exact answers and the existing DFA state cap. Tests check its ID
lifetimes explicitly. Ordinary misses store their transition using the source
ID returned after any eviction/remapping.

The private Tier 1 cache now uses `repr(C)` with its transition vector adjacent
to its stride, followed by the DEAD row and target scratch. Its size remains
200 bytes on this target. This layout restores the baseline adjacency of the
two hot metadata loads; verification of the generated code and performance is
recorded below. Correctness does not depend on the internal field order of
Rust's `Vec`.

## Completed correctness checks

Recorded debug and release full-suite results after the final private
cache-layout adjustment:

| Suite | Result |
|---|---|
| CLI unit tests | 14 passed in each build |
| Engine unit tests | 934 passed, 4 ignored in each build |
| Documentation tests | 4 passed, 2 ignored in each build |
| Ignored fuzz proptests | 3 passed with seed `1234567` |
| Corrected `tier1-assert` sweep | 1,260 clean runs in each of default release, extreme release, and short debug |
| Focused runtime assertion tests | 6 passed |
| Focused CLI assertion-sweep tests | 2 passed |
| Clippy | Both main and fuzz workspaces passed after final engine edits |
| Formatting | `cargo fmt --check` passed |
| Memory expectations | 506 checked after final blessing |

Concrete wrong-answer regressions use `match_tests!`; their checks include the
existing compile configurations, whole/bytewise feeding, minimum/default cache
caps, and forced byte classes. Additional generated checks compare all ordered
assertion pairs against the NFA and `regex` across every partition of bounded
inputs, including empty chunks, invalid UTF-8 bytes, and memory reused across
inputs and regex objects. Structural tests cover context canonicalization,
owned/borrowed key hashes and layout, rich-context eviction, DEAD transitions,
and an input that actually exceeds the minimum cache's state working set.

The final matrix completed 12 blocking sweep commands, totaling 64,044 runs,
with no mismatch, rejection, or abort. The batch fuzz range `0..9000` completed
in both default and extreme regimes without correctness failures or unexpected
aborts. Each regime completed 36 batches with 8,332,674 checks and zero
mismatches; these batch totals exclude the two separately run allowlisted
seeds. Seed 6357 retained its existing known-slow timeout classification;
seed 6581 passed faster than its previous known-slow classification. The
allowlist was not changed. These final binaries include the redundant-table
omission and cache-layout adjustment; earlier results are retained separately
in the validation artifact.

The separate Tier 3 report-only sweeps, captured before the final layout-only
edit, produced 100/100/0 mismatching runs
for `tier3-assert` and 480/480/393 for `assert-in-body` in default/extreme/debug
regimes. All 2,320 distinct whole/bytewise wrong-answer observations reproduced
on baseline `e654f78`; none was a new failure. These known defects are not
reported as passing matches, and no eligibility exclusion was added to hide
them.

The original targeted assertion family completed 1,260 runs in each of the
default release, extreme release, and short debug regimes. A later source
review found that its pure-assertion starred body was normalized to optional
by regex-syntax, so it did not provide the intended production epsilon-cycle
coverage. The corrected family includes a consuming alternative, preserving
the star while retaining zero-width paths through assertions, and its three
regimes now pass. A structural runtime test verifies a reachable compiled
Assert/Split cycle for `(?:\b(?m:^)|a)*.(?m:^)\b`; the generated family uses
the same construction. The sweep deliberately remains counter-free under all
three compile configurations. Counted source patterns that become counter-free
through default unrolling are covered by `match_tests!` and engine-selection
tests, including the retained-counter CRLF fallback.

## Performance and memory

Final Callgrind comparisons passed all 55 focused assertion cases and all 19
existing reengagement cases, using the unchanged 5% instruction and 100%
branch-miss limits. The largest instruction increases were 0.033% for the
focused Tier 3 short/cold case and 3.32% for the existing `tier3_rule_text`
workload, whose automatic selection uses the grouped engine.
The common counter-free compile cases improved instruction counts by 1.35%
to 6.19% after the redundant-table omission. These are measured per-case
results, not a claim of zero overhead for every input. The remaining grouped
workload increase is an explicit measured cross-engine cost below the
unchanged gate. Profile attribution places 158,283 of its 158,366 additional
instructions (99.95%) in changed prefilter/NEON inlining: the candidate uses
an out-of-line SIMD routine where the baseline inlines it. No SIMD source
was changed. The grouped chunk itself accounts for 80 additional instructions.

Disassembly and line-level Callgrind attribution identified the 2.79% mixed
forced-class increase as one additional scalar load per cached transition.
The baseline loads transition-vector length and stride with one AArch64
paired load; the previous candidate layout required two scalar loads. The
65,537 extra load instructions account for 99.3% of its 65,998-instruction
increase, with the remaining 461 instructions not isolated. This is a real
generated per-byte cost even though the source scanning loop is unchanged and
the result passed the 5% gate. The final private layout restores the paired
load, verified in disassembly. Its mixed/classed instruction increase is now
0.02052%, leaving 485 additional instructions per measured 64 KiB stream that
were not individually isolated. A fresh forced-class native comparison was
2.93% faster (95% interval: 1.48–4.77% faster).

Both Cargo target directories and `build.build-dir` were isolated between
baseline and candidate. A shared global build-directory setting had caused
earlier cross-checkout executable reuse; those preliminary runs were
discarded. Independent sanity checks confirmed the baseline's 752-byte
`Regex` and CRLF Tier 1 rejection, and the candidate's 760-byte `Regex` and
new-support oracle checks. Only baseline result data, not build artifacts,
was copied between the isolated targets for comparison.

The final quiet Criterion comparison covered 14 existing-correct direct-table
warm/cold scan cases and four compilation cases, plus the forced-class mixed
comparison above. Two initial direct warm-scan increases, 3.03% for nonword
and 3.52% for mixed, did not reproduce in one fresh paired comparison: their
changes were -0.83% within the noise threshold and -0.94% with no detected
change. The small End compilation case measured +3.32% in the fresh pair,
with interval +0.96–7.10% and classification within the noise threshold,
despite its reduced instruction count. These small native differences remain
uncertain. The directional and LF fixtures improved substantially because
repeated DEAD transitions are now cached. Both rounds, confidence intervals,
and classifications are retained; no further repeat was selected to obtain a
desired result. These timings come from one aarch64 machine.

Original retained-memory measurements on this aarch64 build, before the
boxed empty analysis follow-up below:

| Object or pattern | Baseline bytes | Candidate bytes |
|---|---:|---:|
| `Regex` object | 752 | 760 |
| `MatcherMemory` object | 1,592 | 1,640 |
| `foo` compiled regex | 884 | 888 |
| `\bfoo\b` compiled regex | 950 | 980 |
| `(?m)^foo$` compiled regex | 950 | 980 |
| `foo$` compiled regex | 917 | 946 |
| `\bfoo\b(?m:$)` compiled regex | 983 | 1,014 |
| `(?Rm)^foo$` compiled regex | 1,646 | 980 |

In that snapshot, the fixed `Regex` increase is its optional analysis pointer.
Assertion metadata uses a 16-byte boxed owner and two payload bytes per NFA state on this target;
assertion-free regexes allocate neither. The removed optimistic table offsets
one byte per state, so assertion-bearing counter-free graphs add one net
payload byte per state, plus the fixed owners. Cached DFA state size is unchanged:
the richer one-byte context replaces the old boolean. The two new reusable
Vec owners account for `MatcherMemory`'s fixed increase; target scratch retains
its capacity, and the DEAD row is allocated only when needed, at most 1,024
payload bytes for 256 entries. These figures exclude allocator overhead.

Across the measured 16 KiB streams, run twice with both byte-class modes and
both cache caps, ordinary word, literal, mixed, and churn fixtures retained
the same state counts as baseline. An EndLF context fixture used two states
instead of three in direct mode. The churn fixture retained 30 states with
the default cap; both implementations evicted 2,642 times on the first stream
and 5,284 cumulatively on the second at cap 8. These bounded fixtures do not
prove identical state counts for every pattern and input.

Counter-free CRLF previously selected the grouped engine under the rebased
baseline's automatic policy. Its smaller compiled size therefore reflects
different selected-engine analysis, not an across-the-board memory reduction.

Newly supported CRLF and repaired cases have a separate
`tier1_assertions_new` benchmark (24 cases), separate from the existing-correct
regression gates. On its fixed 64 KiB fixtures, Tier 1 was 3.72–13.81 times as
fast as Tier 0, depending on pattern and chunk size; the repaired empty case
was about 3.1 times as fast. These comparisons measure the new correct
behavior against the NFA, not against the old incorrect Tier 1 answers.

The near-limit compilation benchmark validates actual compiled size, counter
absence, and engine selection before timing with default compile limits:

| Graph | States | Baseline/candidate retained bytes |
|---|---:|---:|
| Isolated assertion tails | 1,801 | 60,185 / 62,010 |
| Acyclic branching | 1,801 | 60,185 / 62,010 |
| Genuine epsilon cycles | 1,751 | 58,535 / 60,310 |

The simple-tail fixture has a measured compilation cost: its fresh paired
means were 786.83/836.41 microseconds baseline/candidate, an increase of
49.58 microseconds or 6.30% (95% interval: +3.13–9.25%). The initial final-source
comparison also increased, by 21.69%, with outliers and a wider interval.
Both final-source rounds are retained. This near-limit graph with 900
assertions has an accepted compile-time cost; process variability affects its
magnitude, and the result is not presented as unchanged performance.
The fresh branching comparison detected no change, and the cycle comparison
improved 7.86%. Criterion's printed timing estimates use regression slopes,
while its percent changes compare means; the artifact retains both and the
simple-tail means above. Peak RSS for the harness including all three fixture
setups was 5,100/4,732 KiB baseline/candidate;
this is a process measurement, not an isolated analysis-allocation maximum.

## Review follow-up verification

[PR 24 review findings 1, 2, 3, 8, and 9](https://github.com/marmeladema/rethoc/pull/24#issuecomment-6986103088)
are addressed:

- `sweep tier1-assert` defaults to the NFA and Tier 1, so an eligibility
  regression fails instead of silently omitting Tier 1. Explicit backend
  selectors still override the default.
- Compilation flags precede the optional `--` for dash-leading patterns.
- The assertion benchmark comparison requires successful baseline generation
  and copying. A failure of the separate reengagement comparison does not
  suppress it.
- Dump output keeps tier eligibility together, then prints the assertion
  table with deliberate section spacing. Cache Debug reports scratch-vector
  lengths and capacities instead of their contents.

After these edits, debug and release each passed 17 CLI tests, 934 engine
tests, and 4 doctests; Clippy passed for both workspaces, and formatting
passed. The default `tier1-assert` command, without an explicit tier selector,
passed all 1,260 runs in each of default release, extreme release, and short
debug (maximum input length 4). All three had zero mismatches, rejections,
or aborts. Real-parser regression tests cover explicit selectors, rejection
of a counter-retaining pattern, and flags before dash-leading patterns.

Sixteen old/new command-list comparisons confirmed unchanged behavior for
other sweep families. The default Tier 1 list equals explicit `--tier 1`,
and five explicit backend overrides retain their previous lists. A dump
smoke check and a public-API cache Debug probe confirmed the output changes.
The workflow YAML, prerequisite step IDs, and guard expression passed a
64-combination static check; this was not a GitHub runner simulation.

The engine follow-up changes only formatting implementations, with no
matching, scanning, cache-layout, or analysis changes. The original benchmark
and large differential/fuzz records above remain historical measurements;
those matrices and benchmarks were not repeated for this follow-up. The
validation artifact records the new checks separately.

### Assertion-analysis lookup hoist (finding 7)

The later engine revision `026d6e9b1641f51df4cb74fc3261f57b4c217253` also resolves the
assertion analysis once per assertion-enabled closure call and reuses the
reference in both loops. The assertion-free specialization does not access
it. EOF evaluation, deferred-context requirements, and traversal are unchanged.

The full debug/release suites, Clippy for both workspaces, and formatting
passed again. Three Tier 1 sweeps passed 3,780 runs. Batch fuzz seeds `0..500`
passed in default and extreme regimes, with 465,734 differential checks per
regime and no failures.

Fresh isolated Callgrind comparisons against the preceding PR head
`b5968d651fa39b2aefcc9a5b2a20cebb16c5d75b` passed all 55 assertion cases and all
19 reengagement cases with the existing limits. Assertion-case instruction
changes ranged from -0.24793% (tiny-cache cold churn) to +0.16902% (mixed
assertions on a short cold stream); reengagement changes ranged from
-0.00118% to +0.00187%. The largest branch-miss increases were 9 misses
(+9.89%) and 58 misses (+11.22%), respectively, below the unchanged 100%
limit. This is a small cleanup, not evidence of a significant throughput gain.
Commands, source hashes, and comparison extrema are retained under
`assertion_lookup_hoist` in the performance and validation artifacts.

### Boxed assertion analysis without Option

Engine revision `e7676d0dd0064786624fb9579c2c68119040bc37` retains the outer box and
uses an empty entries slice where analysis was previously absent. Documentation
on the field and analysis type distinguishes the always-allocated owner from
the allocation-free empty payload. A populated table still covers every NFA
state. Empty tables select the assertion-free closure, whose EOF result comes
from traversal; indexed queries never substitute default metadata.

Measured object sizes remain unchanged: `Regex` 760 bytes, `DfaMatcher` 64,
`AnyMatcher` 608, and `MatcherMemory` 1,640. The empty analysis adds one retained
16-byte owner to assertion-free and counter-bearing regexes, excluding
allocator overhead. Populated-analysis memory is unchanged. For example, `abc`
increases from 888 to 904 compiled bytes, while `\babc\b` and `(?Rm)^abc$`
remain 980 bytes. Memory blessing changed exactly 173 expectations by +16
bytes; all 506 expectations passed, and no tier expectation changed.

Debug and release each passed 17 CLI tests, 935 engine tests, and 4 doctests.
Clippy for both workspaces and formatting passed. The new structural test
covers assertions with default unrolling and retained counters, including CRLF.
Three Tier 1 sweeps passed 3,780 runs; fuzz seeds `0..500` passed in default and
extreme regimes with 465,734 differential checks each and no failures.

Fresh isolated Callgrind comparisons against preceding PR head
`e959fdbcd569b5c7be1e18810a3e6c0196710d2b` passed all 55 assertion cases
and 19 reengagement cases with unchanged limits. The largest Tier 1 scan
instruction increase was 0.91437%; including the counting-tier fixtures,
the largest focused scan increase was 1.29202%. Compilation increased at most
2.09100% (the literal fixture), and reengagement instructions increased at most
0.36395%. The largest branch-miss increases were 15.87% and 17.61%, below the
100% gate. Scan fixtures own and drop their regex, so these totals include
teardown and the extra empty-owner deallocation; they do not isolate per-byte
costs. The `boxed_empty_analysis` sections in both artifacts retain commands,
source and binary hashes, memory results, and performance extrema.

## Completion and scope

All required implementation checks are complete. The performance and
validation artifacts retain commands, binary/source identifiers, per-case
results, and the distinction between final and superseded measurements.
The engine implementation and verification tooling are committed separately.

Existing issue #5 remains out of scope: one `MatcherMemory` reused across
different numbered tiers among 1–3 can panic through shared scratch reuse.
The new same-tier/different-regex tests do not claim to fix or validate that
mixed-tier path. Known Tier 3 design defects remain separately reported, and
the change does not add eligibility exclusions to hide them.
