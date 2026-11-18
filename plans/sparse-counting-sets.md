# Sparse counting sets on the grouped NFA

Status: milestones S0–S4 are implemented (`grouped_nfa/sparse.rs`, forced
only: `RegexConfig::sparse_counting`, `--sparse-counting`); automatic selection
is unchanged. The measurements of S5 are taken with the probes of
[the benchmark method](../docs/research/counting-benchmark-method.md) and kept
out of the tree. What follows is the plan as written before implementation:
an experiment plan, not an automatic-selection change. Verified source baseline: `730eebd5abd4d176bf22a0a0eee491522e154264`,
latest `origin/main` when fetched on 2026-11-18. The branch is rebased onto it.
Since the original audit at `c84282e`, only CLI `grep` engine selection changed;
the counting runtime and assertion evaluator are unchanged.
Exact grouped execution and automatic selection have landed. This experiment
follows [the grouped plan](grouped-nfa.md) and
[engine directions](../docs/research/counting-engine-directions.md).

## 1. Decision and experiment question

Build a separately forceable **sparse execution variant of the grouped NFA**
over the existing optimized Thompson graph. Initially represent every counted
scope of an admitted pattern sparsely; outside states remain Boolean. Do not
add a numeric tier, modify Tier 3, mix exact/sparse counted scopes, or change
default dispatch during the experiment.

Question: for finite flat repetitions with wide acceptance ranges, can bounded
representative lists reduce active-count update/copy work and persistent stream
memory enough to beat optimized exact grouped execution? Include compilation,
startup, reset, assertions, sparse activation and chunking in the comparison.

The first prototype stores **direct positive count values**, not offsets that
grow with stream length. Increment, copy and merge are O(D). This deliberately
does not claim the paper's offset-based constant-time increment or amortized
merge guarantees. Offset optimization can follow if profiling justifies it.

## 2. Sources and verified scope

The full extracted paper is already in the repository:
[Berglund, van der Merwe, Sung: Efficient Regex Matching with Sparse Counting-Sets](../research_notes/regex-matching-research-2020-2026/text/2026-ncma-berglund-sparse-counting-sets.md).
Definitions 11/16 describe the operations; Lemmas 17–20 their bounds and
acceptance observations; Corollary 21 and Theorem 24 concern flat position
counter automata. Printed pp. 28–30 are PDF pages 10–12. The extraction's local
PDF path is absent; formulas were also checked against the
[same paper online](https://arxiv.org/pdf/2606.26679). No additional paper is
required for this experiment.

The finite list bound is `2 * ceil(h / (h-l+2))` for positive active values and
`h >= 1`. A middle value can be removed when its neighboring endpoints span
at most `h-l+1`. These statements do not justify arbitrary masks, active zero
values, or independently compressed nested counters. Nor does the paper's
position-automaton time bound automatically apply to our Thompson graph.

Sections 4–6 give a **repository-specific mapping and proof argument** for
finite counters, including a proposed assertion extension. Independent paper
and implementation reviews agreed on that mapping. The executable bounded
audit is documented in [verification/sparse-counting-sets.md](verification/sparse-counting-sets.md).
It is model evidence, not verification of a future Rust implementation.

| Current source, relative to `rethoc-engine/src/` | Verified behavior | Consequence |
|---|---|---|
| `grouped_nfa/analysis.rs`: `prove_scopes`, `zero_width_order`, `counter_bounds` | Unique flat scopes, acyclic zero-width order, consistent bounds | Reuse structural certificates |
| `analysis.rs`: `analyze`, `row_words` | Structure and dense row admission are combined | Separate them before sparse admission |
| `grouped_nfa/exact.rs`: `increment` | Finite body/CInc rows have completed counts below max; increment precedes tests | Sparse numbering and exit timing need an explicit mapping |
| `exact.rs`: `BoundaryCtx`, `GroupedStream::chunk`/`finish` | Replay raw roots with one boundary context | Preserve schedule, not exact row contents |
| `analysis.rs`: `reviewed` | Ten current assertion kinds are audited | Reuse evaluator; separately validate sparse replay |
| `exact.rs`: count-free stepping and `unit` | Fast path assumes exact singleton bit-zero rows | Cannot directly reuse it for positive sparse rows |
| `lib.rs`: `RegexBuilder::build`, `Engine`, `MatcherMemory::matcher` | Compile-time dispatch is Tier 1, Tier 2, grouped, Tier 4, NFA | Keep sparse experimental and explicit |

CLI integration also includes `rethoc-cli/src/main.rs::grep_matcher` and
`run_grep`: forced selection is validated even for an empty file, then reused
across lines. Sparse forcing must follow that contract.

## 3. Admission and resource limits

Analyze the final optimized NFA, including every consuming `out_exit` and all
`ByteTable` targets. Require the existing unique-scope and zero-width DAG
certificates. Consuming loops around completed counters are allowed; nested
scopes and zero-width cycles are rejected. `body_byte_length == 0` is not a
nullability test: it also denotes variable-length bodies.

The first runnable milestone admits finite counters with nonnullable bodies
and **no assertions anywhere**. Assertion support is a separate mandatory S3
milestone before promotion. Reject unbounded counted scopes for this experiment;
their saturated domains require a separate mapping. Ordinary uncounted loops
remain allowed if the structural checks pass.

Use actual compiled bounds: `{0,h}` normally becomes an outside bypass and
a positive-count body. `{0}` is compiler-eliminated. Do not evaluate the sparse
bound for an active `h=0` scope. Counter-free graphs are test controls.

For each finite scope, compute with checked arithmetic:

```
k = h - l + 1
D = min(h, 2 * ceil(h / (k + 1)))
```

The extra `min(h, ...)` is the trivial size bound for distinct values in `1..=h`.
Reject invalid bounds and values not representable by `u32`; use widened
arithmetic for capacity, spans and byte calculations. Existing global repetition
and estimated-state limits still apply. Record their rejections separately.

Initial experiment caps are engineering choices, not performance crossovers:

| Resource | Cap / accounting |
|---|---|
| Optimized state array | 4,096 |
| Examined edge entries | 1,048,576; count all 256 byte-table entries |
| Representatives per counted row | `D <= 32` |
| Total reserved representative slots | 16,384 `u32` slots per frontier, 64 KiB payload |
| Retained sparse plan | 1 MiB including every owned array |
| Transient analysis | 2 MiB including simultaneously live arrays |
| Merge workspace | `2 * max(D)` values plus bounded metadata |

Charge row lengths, outside activity, bitmaps, struct sizes and worker scratch
in addition to the payload. Each row reserves D slots; merge uses separate 2D
scratch and reduces before writing back. No scan-time allocation, truncation,
budget failure or fallback replay. Capacity violation is an implementation bug.
Count peak simultaneous structural/layout temporaries, including duplicated
certificates during refactoring; bounding each allocation individually does
not bound total analysis memory. Report separately the total Regex allocation
when exact and experimental sparse plans are both retained.

Sparse analysis must reach the structural certificate **before** exact layout's
8,192-word rejection. Otherwise it cannot cover patterns rejected only for
exact storage. Keep exact admission unchanged. Tests may raise sparse D to
exercise exact-count negative controls, with an explicit bounded test budget.

## 4. Finite count mapping and transfers

In an existing exact grouped row inside scope `[l,h]`, let `c` be the completed
iteration count before the pending CInc. Reachable finite rows satisfy
`0 <= c < h`. Sparse rows use the active iteration number **`u = c + 1`**,
so `1 <= u <= h`. Outside rows have activity only. Zero is neither an active
sparse count nor a substitute for an empty row.

| Graph operation | Sparse transfer |
|---|---|
| Outside split or consuming edge | Propagate activity along the same edge |
| CI entry from active Outside | Insert `{1}` in target counted row |
| Same-scope split, byte or table edge | Copy/union representatives, then reduce |
| CInc exit | Activate Outside exit iff **source** has any `u >= l` |
| CInc continuation | Send `{u+1 : u in source, u < h}` to continuation |
| Outside Match | Set sticky match flag |
| Assertion, after S3 | Transfer whole row iff shared evaluator returns Pass |

Exit is checked **before** sparse increment. Existing exact `increment()` is
not the sparse primitive: exact exit tests `c+1 >= l`, and continuation stores
`c+1 < h`, whose positive image is `u+1`. At `u=h`, exit may succeed but
continuation disappears. Both edges can fire; neither may mutate the source
before the other reads it. Increment happens at iteration completion, not per byte.

Exit followed by a new CI checks the old scope and seeds `1` in the new one.
Reentry of the same completed counter also resets to `1`. Preserve the ordinary
Outside bypass for zero-min repetitions. Keep source rows available for fanout.

## 5. Reduction and correctness argument

Rows are strictly ascending positive values. For consecutive `a < b < c`,
remove `b` when `c-a <= k`; preserve endpoints and deduplicate. A deterministic
left-to-right stack scan removes the penultimate value while the last triple
violates the spacing condition. Each value is pushed/popped at most once.

Merge sorted lists into scratch and reduce; insert `1` by bounded merge.
Increment values below h and drop h. Increment preserves order and spacing.
Avoid repeated arbitrary-index deletion with O(D²) shifting. Copies cost O(D).

For P reachable nodes, E examined graph edges and S reserved count slots, a
conservative full-sweep boundary step costs `O(S + (P+E)*Dmax)`, taking Dmax
at least one. Byte-table dispatch is a bounded lookup, not a per-byte scan of
all 256 entries. Persistent storage is `O(P+S)`; worker scratch is another
`O(P+S+Dmax)`. Compiled metadata is charged independently. For n bytes and q
nonempty chunk calls, raw replay costs at most
`O((n+q+1) * (S+(P+E)*Dmax))`, plus O(1) per empty call. Constants and active
rows matter in measurements. This is the proposed Thompson implementation's
bound, not a restatement of the paper's position-automaton theorem.

### Observation invariant

For a positive exact set A define its future exit profile over integers:

```
E(A) = union over u in A of [max(0, l-u), h-u]
N(A) = {t >= 0 : some u in A has u+t <= h}
```

E describes every number of future iteration increments after which exit is
possible. N describes future nonemptiness. Require at each actual state/scope:
`S subset-of A`, `E(S) == E(A)` and `N(S) == N(A)`, where A is the positive
image of exact reachable counts. This is not exact configuration equality.

For `a < b < c`, endpoint intervals cover the middle interval if
`c-a <= h-l+1`: they overlap or are adjacent as integer intervals. Clipping
their lower bounds to zero preserves coverage. Keeping the minimum preserves
N, which is `[0,h-min(A)]` for nonempty A. Thus each deletion preserves E and N.

- Union unions the profiles; normalization preserves them.
- Bounded increment shifts each profile: `E(inc(A)) = {t >= 0 : t+1 in E(A)}`,
  and likewise for N.
- Exit asks whether `0 in E(A)`; activity asks whether `0 in N(A)`.
- CI/reset inserts identical `{1}` after the same Boolean gate.
- Copy or whole-row pass/drop preserves the relation.

These properties support induction over topological closure, byte consumption,
and search reseeding. An assertion can restrict which future iteration counts
a suffix permits; equality for **every** t preserves those possibilities.
This requires guards to be count-independent except for the stated CInc
operations. It does not cover numerical subrange filters, capture histories,
priorities, count-sensitive assertions, or nested correlations.

Example: for `[5,10]`, `{4,5,6,7,8,9,10}` reduces to `{4,10}`. An extra `<=7`
filter followed by exit distinguishes them. That filter is forbidden, even as
an alleged optimization. Current-check equality alone is also insufficient.

The reduced list has `v[i+2]-v[i] > k`. Odd/even subsequences give the D bound.
This proves capacity; it does not permit truncating to D values. Merging can
remove existing representatives, so never propagate `new minus seen` values.
Process a node only after all its zero-width predecessors have contributed.

Before runtime implementation, review this argument against actual CI/CInc
lowering, every optimized edge, empty sets, alternative paths and repeated
entries. Any transfer not covered here needs a revised proof or explicit
admission exclusion. Do not substitute a guessed special case.

## 6. Assertions, streaming and reusable memory

Keep immutable raw post-consumption rows. Constructor performs provisional
closure. Before byte b, close raw with `BeforeByte(b)`, then replace raw with
consuming successors. At nonempty chunk end, close a copy with `UnknownNext`.
At finish, close with `EndOfInput`, without a sentinel byte. Empty chunks are
no-ops before finish. Matches are sticky; reset clears boundary and lifecycle
fields. Preserve the existing lifecycle: chunk or a second finish after finish
is rejected until reset/rebind, including when the match flag was already set.

Fail/Defer blocks a closure edge, not its raw root. End fails before EOI and
can later pass. Never persist a speculative closure in raw, or carry old failed
roots across consumption. An asserted path cannot inherit permission from an
unasserted path that happens to reach the same destination. Merge only after
each predecessor's own guards have passed.

Reuse `AssertKind::eval` and its reviewed-kind gate. S3 covers all ten current
kinds: Start, End, StartLF, EndLF, StartCRLF, EndCRLF, WordAscii,
WordAsciiNegate, WordStartAscii, WordEndAscii. A new kind needs explicit review.
BeforeByte/EOI must resolve; UnknownNext Pass must remain Pass for every next
byte and EOI. Existing exact assertion audits support these evaluator facts,
but do not verify sparse scheduling or the future implementation.

One stream owns a flat representative arena, row lengths, activity bitmap and
boundary flags. Reusable worker scratch owns closure rows and merge workspace.
Preallocate before scanning; successful reset/rebind retain capacity and clear
logical state after success or finish. Check admission before mutating on a
forced request/rebind: rejection must leave the prior stream intact. A later
successful reset/rebind must work normally. Independent streams may share
scratch sequentially, never their persistent rows.

Start with a full topological sweep. Add active scheduling after equivalence
tests. Existing count-free acceleration uses exact bit-zero singleton rows;
sparse singletons encode positive values. Reuse that acceleration only after
proving the corresponding singleton condition, assertion exclusion, reseeding
and handoff to general execution. Do not disable exact grouped optimizations.

## 7. Proposed code and API boundaries

Names here describe new implementation work, not existing APIs:

- `grouped_nfa/analysis.rs`: expose bounded representation-independent scope,
  order and bounds analysis. Exact layout still consumes the same certificate.
  Keep the refactor independently reviewable, with exact admission regression tests.
- `grouped_nfa/sparse.rs`: `SparsePlan`, separate row/index types, finite list
  transfers, `SparseStream` and `SparseScratch`; split modules if necessary.
  Existing `Row`/`Succ` encode exact offsets and cannot stand for sparse rows.
- `RegexBuilder::build`: opt-in sparse analysis on final graph, stored separately
  from exact admission. Proposed `sparse_counting: bool` defaults to false.
  Rejection does not reject normal regex compilation or change dispatch.
- CLI/checkers/probes: `--sparse-counting` / `sparse`, exclusive with forced
  `--tier` and `--grouped-nfa`. Forced rejection is explicit, never fallback.
  Cover `match`, `grep`, `info`, `dump` and checking/reproduction commands;
  extend existing selector-conflict rules consistently. In `grep`, validate
  the engine before reading lines, including an empty file, and preserve
  line-oriented matching, quiet mode and exit behavior. Test multiple lines
  through the same reusable memory. `info`/`dump` request sparse analysis and
  report admission without changing automatic selection.
- `info.rs`, `dump.rs`, Display/Debug: actual engine, admission reason, scope,
  bounds, D, used/reserved slots and plan/stream/scratch bytes. Developer traces
  explain `u=c+1`; they must not claim sparse rows are exact configurations.
  Public API documentation stays implementation-neutral.

Extend backend enumeration, shrink/repro, proptests and both matching fuzz
targets together. Tests explicitly request sparse analysis and report rejected
cases; a backend's omission is not a pass. Preserve Tier 3 known-defect handling
and independent Tier 0. Default Engine selection stays unchanged.

## 8. Milestones

| Stage | Deliverable | Exit gate |
|---|---|---|
| S0 | Active-count operation/allocation instrumentation and multi-stream harness | Identify bottleneck; report memory and admission exclusions |
| S1 | Shared structural analysis, sparse layout and Rust list primitives | Exact behavior/admission unchanged; algebra and checked-size tests pass |
| S2 | Explicit finite assertion-free matcher, full sweep | NFA/regex agreement and raw profile checks on branch/reset/reentry cases |
| S3 | All reviewed assertions with raw replay | Boundary/replay gates below pass in debug and release |
| S4 | Active-row and count-free/idle paths where justified; complete tooling | Optimized/general sparse agree; compare to optimized production grouped |
| S5 | Performance/memory/coverage report and adoption decision | Useful measured class; any automatic-selection change is a separate patch |

The model audit can precede S0, but does not replace profiling. Do not wait for
all fast paths to test the first runtime, or promote it using only an artificially
slowed competitor. Unbounded counts are not a prerequisite for completing S5.

## 9. Verification requirements

### Algebra and structure

- Exhaust small bounds and all subsets; duplicate unions, branch copies,
  repeated merges, continuation to exhaustion, reset and empty rows. Check
  every future exit/nonempty observation, not only the immediate exit result.
- First compare an exact positive-count model to scalar completed counts;
  then check sparse subset/profile equivalence at corresponding states. Do not
  demand exact representative equality or canonical-list associativity.
- Check D and temporary 2D capacity, source preservation, exit at l,
  continuation disappearance at h, simultaneous exit/continue, zero-min bypass.
- Cover invalid sentinels, integer and cap boundaries, all byte-table targets,
  out_exit, diamonds, repeated entry and completed-scope loops. Confirm nested,
  zero-width-cycle and unbounded rejection. Exercise sparse acceptance where
  exact fails only its row budget, subject to the global compiler limits.

### Matcher and assertion gate

Use macro reruns for default compilation, unrolling disabled, and both unrolling
and merging disabled. Never set `unroll_limit: 0` on a data-driven entry.
Structural/model tests can inspect row profiles separately. Compare against
original NFA and regex, including when exact grouped rejects a sparse-admitted
graph. Exact grouped is an additional competitor/reference, not the only oracle.

For every assertion kind, test before entry, before/after body bytes, before
CInc, after exit, between scopes, in chains and on a branch with an unasserted
bypass. Merge paths carrying different counts. Example shapes include
`a(a\b|aa){2,5}z`, `x(a|aa){2,5}\bz`, `(?mR:^x(a|aa){2,5}$)`,
`\Ax(a|aa){2,5}\z` and `(?:a(?m:^)|\n\n){2,3}c`. Inspect compiled admission;
default unrolling can remove the counters these examples intend to exercise.

Exhaust short inputs and all partitions over a/space/CR/LF/non-word bytes;
include NUL and 0xff. Compare `ismatch()` after each call to unfinished Tier 0,
not `regex.is_match(prefix)` which implies EOI. Compare final results to regex.
Cover initialization, empty chunks/input, CR|LF splits, finish-only matches,
an extra byte before finish, early match, reset, rebind, and multiple streams
interleaved through shared scratch.

Mutation sensitivity must include: exit after increment; active zero seed;
over-pruning at k+1; extrema-only compression of a holey set; arbitrary upper
mask; provisional closure stored in raw; stale roots after consumption; missing
EOI replay; and incorrect classification into a count-free fast path.

Run debug/release tests, clippy, formatting, compile fuzzing and bounded
oracle/cross-backend fuzzing after compiler/runtime milestones. Extend assertion
sweeps to explicitly include sparse; report tested/rejected counts. All direct
binaries/scripts run under `scripts/capped`. Run the reengagement performance
gate before hot-path integration. Do not weaken gates or hide Tier 3 defects.

## 10. Performance and memory experiment

Sweep `h = 8, 64, 256, 1000`; larger bounds require separately labeled explicit
compile limits. Record settings, compiler/dependencies, CPU, affinity and load.
Use at least three shuffled process repetitions on identical inputs. Store timing
results and metadata under ignored `target/` or CI artifacts; commit only
harness/methodology and deterministic verification.

For the primary representation-scaling experiment explicitly set
`max_unroll_states=0` and `merge_repetitions=false` for every competitor, and
record the actual compiled counter scopes, bounds, D and node counts. Otherwise
small bounds can unroll or merge while large ones remain counted, invalidating
the comparison of representations. These are probe configuration settings,
not `unroll_limit: 0` entries in the data-driven test macro. Run default
compilation separately as an end-to-end competitor; preserve all size/admission
limits in the record and do not call forced-counter results default performance.

| Family | Concrete example | Purpose |
|---|---|---|
| Wide range | `x(a\|aa\|b){1,1000}z` | D=2; ambiguous body behind consuming prefix |
| Middle lower bound | `x(a\|aa\|b){500,1000}z` | D=4; lower bound matters |
| Narrow window | `x(a\|aa\|b){900,1000}z` | D=20; measure crossover |
| Exact count | `x(a\|aa\|b){1000}z` | D=1000; default rejection and bounded diagnostic control |
| Sustained entry | `a(a\|aa){1,1000}z` on all a; `x(a\|aa\|x){500,1000}z` on x-rich input | New prefixes do not kill older body paths |
| Short-lived entry | `x(a\|aa\|b){500,1000}z`, x every seven bytes | x kills old bodies; not long-lived overlap |
| Holey counts | `x(a\|aaa){2,8}z` | Do not invent values between representatives |
| Sequential scopes | `x(a\|aa){1,100}y(b\|bb){1,100}z` | Scope exit/entry and resets |
| Completed-scope loop | `(a{2,5}b)*c` | Reentry without nesting |
| Specialized competitor | `(ab){1,1000}z` | Tier 2 may remain faster |
| Idle/control | `abc\|abd`, sparse-start inputs | Non-counting overhead |
| Exclusions | `x((a\|aa){1,10}b){1,10}z`, `(a?){2,5}z`, `x(a\|aa){5,}z` | Nested, nullable-cycle, unbounded |

Include successful early/late matches, near misses, fixed-random inputs, long
nonmatching streams and chunks of 1, 16, MTU-like, 16 KiB and whole input.
For one-time prefixes report live episode and idle suffix lengths; skipped
bytes must not conceal count cost. Compare general exact/sparse kernels to
isolate representation work, then optimized sparse against optimized grouped,
automatic selection and eligible tiers. Tier 3 is a performance diagnostic,
not an oracle or production selection option.

Measure compile/first-use/reset time, throughput, per-chunk latency, active rows,
values/words copied/merged/incremented, fanout, allocations, retained plan,
stream capacity and worker scratch. Run instrumentation separately from timings.
Report global compiler rejection even when sparse analysis is never reached.

The deployment harness keeps independent state for 500+ patterns per stream
and interleaves chunks across many streams. Share immutable programs and worker
scratch where APIs permit; do not share persistent rows between live streams.
Choose stream counts within the process memory cap and report actual capacity
and residency, not `memory_size()` multiplied by concurrency. Compile outside
scan timing. Check the same deterministic corpus against full-input regex oracles
in a separate correctness replay; accumulated oracle inputs must not inflate
the streaming-memory measurement or remain in the timed run. Account for input
buffers and harness overhead separately from matcher state and scratch.
Existing cross-tier MatcherMemory issue #5 is a distinct limitation: disclose
affected comparisons, without treating its repair as part of sparse research.

### Decision criteria and follow-ups

For fixed small D, reserved representative storage and count work per active
edge should not grow with h. Exact/narrow ranges are negative controls. If
control traversal dominates or word bitsets win, retain exact grouped there.
Proceed only for a documented useful class with measured memory/performance
benefits and explicit costs in compilation, startup, coverage and chunk latency.

No universal speedup threshold or numeric dispatch cutoff follows from the
paper. A later selection proposal needs a structural class, resource bounds
and crossovers measured on target hardware. Default dispatch stays
Tier 1 → Tier 2 → exact grouped → Tier 4 → NFA until separately reviewed.

Unbounded saturation, per-scope mixed representations, a new position-automaton
compiler, offset lists and shared multi-pattern scanning are separate follow-ups.
The finite experiment can succeed or fail without implementing them.
