# Exact grouped NFA implementation plan

Status: P0–P4 implemented in `rethoc-engine/src/grouped_nfa/` (opt-in
`RegexConfig::grouped_nfa`; multiword and saturating count domains within the
8,192-word frontier budget; all assertion kinds by raw replay; CLI, checkers,
proptests, cargo-fuzz and the probe run it; active-row scheduling and
prefilter skipping while idle). P4's deployment measurements (allocation and
operation counters, many rules × many streams) remain open. P5 below now
specifies the proposed automatic-selection implementation; see "P4
findings" and "Idle cost". Designed against main `b0bb981e8c6cba7b23df176d79f741c3b61089a7`;
source references below re-checked at `d82066d`. This work does not
investigate existing tier correctness bugs.

## Decision: sparse sets are not required

Build an **exact grouped NFA as an independently useful backend**. Sparse sets
are an optional later optimization, with a different invariant and proof.

| Technique | Preserves | Benefit | Remaining cost |
|---|---|---|---|
| Exact grouping | Every reachable count at each control position | Eliminates per-value predicates, context copies, and hashing | Bitset work grows with the count domain |
| Sparse compression | Required acceptance observations, not every count | Fewer representatives for suitable ranges | Restricted operations and a separate abstraction proof |

Grouping changes the layout to `state -> set of valuations`. Sparsification
discards some valuations. For the flat subset below, a valuation is empty or
one scalar count. Nested scopes need joint tuples and are excluded.

Target consuming-prefix variable-body workloads, for example
`a(a|aa){1,256}b`, where the current per-instance representation warrants a
comparison. Measure the exact backend before pursuing further techniques.
Keep default unrolling, Tier 2, and all other eligible paths as competitors.

Context: [directions](../docs/research/counting-engine-directions.md),
[benchmark method](../docs/research/counting-benchmark-method.md),
[deployment requirements](../research_notes/engine-goal.md).

## 1. Evidence and verification status

Source locations are symbols in `rethoc-engine/src/lib.rs` unless specified
(line numbers drifted by up to 450 lines in two commits, so none are given).

| Verified behavior | Location | Design implication |
|---|---|---|
| NFA stores `(StateIdx, CounterCtx)` | `MatcherMemory::clist`/`nlist` | Multiple valuations repeat work at one state |
| Closure hashes/copies nonempty contexts | `NfaMatcher::drain_addstack` (dedup) | Replace with grouped unions for the admitted subset |
| Predicate dispatch occurs per thread | `NfaMatcher::step` (consumption loop) | Group before consumption |
| CI initializes count zero | `drain_addstack`, `State::CounterInstance` arm | Zero is an active count, not an inactive sentinel |
| CInc increments before testing continue/break | `drain_addstack`, `State::CounterIncrement` arm | Increment at iteration completion, not per byte |
| Continue is `< max`; break is `>= min` | same arm | Separate continuation and exit transfers |
| Unbounded counts saturate at `min` | `counter_step` | Unbounded repetition can have a finite exact domain |
| `{0,max}` lowers to optional `{1,max}` | `RegexBuilder::emit_bounded_repetition` | Preserve the existing Outside bypass |
| ByteTable consumes; atoms can have `out_exit` | `State` | Cover all consuming edges in analysis/runtime |
| Byte-table optimization precedes analysis | `RegexBuilder::build` → `optimize_byte_tables` | Analyze the final graph |
| `End` returns Fail before EOI | `AssertKind::eval` | Saving only Defer assertions loses candidates |
| CLI checker enumerates tiers 0–4 | `rethoc-cli/src/check/mod.rs`, `TierChoice { All, Only, Except }` | Explicitly integrate the experimental backend |

`CounterInfo.body_byte_length == 0` is not a nullability test: it also denotes
variable length. Do not reuse Tier 2/3 eligibility or body-length metadata as a
substitute for the scope and zero-width proofs below.

Verification performed for this plan:

- Read scalar transfers, assertion evaluation, compilation, memory ownership,
  test helpers, and CLI selection code.
- Inspected forced-counter dumps of `x(a|aaa){2,5}z`, `(a{2,3}b)*c`, and
  `a{0,2}b`: confirmed variable body, completed-scope reentry, and optional bypass.
- Exhaustively checked bitset increment/exit formulas against scalar semantics:
  finite maxima 1–8, every lower bound and legal incoming subset, plus saturated
  domains with minima 0–8. All 5,118 cases passed. This does not verify Rust
  cross-word carries, complete matching, or performance.
- Independent reviews checked scope propagation, topological closure, assertion
  replay, and the separation from the sparse paper's guarantees.

The implementation must discharge the remaining obligations below. This is
not a formal proof or evidence of speedup.

### Dedicated assertion re-audit

Three independent reviews checked evaluator semantics, graph/guard provenance,
and an executable scalar-versus-grouped scheduling model. The reproducible
[assertion audit](verification/README.md) records:

- 660,490 complete-boundary checks against the actual Rust evaluator, plus
  provisional-pass stability under next-byte revelation **and EOI**.
- 500,160 abstract-model executions and 3,417,480 prefix comparisons across
  all assertion kinds, chunk partitions, and inserted empty chunks, with no
  discrepancy. These are overlapping suites, not distinct-case counts.
- Detection of five deliberately unsafe scheduling/storage mutations.

The audit strengthened Section 5 and P3 below. It does not validate a Rust
grouped matcher, production admission, or general count-domain machinery.

## 2. Admitted class

Use the existing optimized Thompson NFA, with no second regex compiler, position
automaton conversion, DFA cache, or new numeric tier.

Require all of:

1. Each reachable state has one proven incoming scope: Outside or Counter(c).
2. The reachable zero-width graph is acyclic, conservatively including every
   assertion edge and both counter branches.
3. Exact domains and metadata fit checked compile-time limits.
4. The first runnable milestone excludes assertions and unbounded repetitions,
   and requires one-word finite domains. Later milestones lift these restrictions
   explicitly, even when an unbounded counter's saturated domain would fit.

Allow consuming prefixes, multiple sequential/alternative counters, fixed and
variable nonnullable bodies, and ordinary repetition around a completed counter
when these checks pass. `(a{2,3}b)*c` remains flat: it exits the counted scope
before the ordinary star reenters it.

Reject grouped eligibility for nesting, conflicting scope labels, zero-width
cycles, milestone exclusions, or budget exhaustion. Normal regex compilation
and existing dispatch remain available. Forced grouped selection reports the
reason; it must not silently run a different backend or return false.

Original P0–P3 non-goals: sparse sets, interval approximation, runtime truncation,
prefilter skipping, SIMD, context DAGs, cyclic closure, or changing automatic
dispatch. Idle skipping was added after P4; P5 explicitly changes dispatch.
Counter-free graphs are useful test controls, not the initial promotion target.

## 3. Compile an immutable plan

Add `rethoc-engine/src/grouped_nfa/{mod,analysis,exact}.rs`. Names and APIs in
this plan are proposed additions, not existing interfaces.

### Scope proof

Seed `Regex.start` with Outside and propagate labels over **all** edges,
including consuming edges. Each state can acquire one label. Reject a different
second label or an operation incompatible with its incoming scope.

| State | Required incoming scope | Outgoing scope |
|---|---|---|
| Split, Assert | Either | Preserve on every edge |
| Byte/ByteCI/Wildcard/classes | Either | Preserve on `out` and non-NONE `out_exit` |
| ByteTable | Either | Preserve on every non-NONE table target |
| CI(c) | Outside | Counter(c) |
| CInc(c) | Counter(c) | Continue: Counter(c); break: Outside |
| Match | Outside | Terminal |

Reject CI inside any active scope, mismatched CInc, and Match with active scope.
Ignore byte/guard feasibility conservatively. Validate identities and bounds
against actual CI/CInc nodes; cross-check counter metadata. Ignore unreachable
graph remnants. Test conflicting labels with explicit graph fixtures even if
the current compiler does not produce them.

### Topological closure schedule

Construct a second graph containing outgoing edges only from Split, Assert,
CI, and CInc. All consuming states, including ByteTable, are terminal in this
graph. Topologically sort reachable states iteratively; reject remaining cycles.
Handle duplicate edges consistently in indegrees and traversal.

A nullable counted body creates a zero-width path to CInc and a continue cycle.
Other zero-width cycles are excluded too, even if harmless. An ordinary `b*`
inside `ab*` need not create a zero-width cycle: its return path consumes `b`.

Seed all input rows before traversing the stored order. Every state's incoming
zero-width contributions are complete when processed, so closure visits it once.
No per-count visited hash or fixed-point queue is needed. A future cyclic
worklist would require a different work bound.

### Exact row layout

Use a flat u64 arena with precomputed per-state spans:

- Outside: one word; bit zero denotes a reachable empty valuation.
- Finite counter maximum h: values `0..=h`, requiring `ceil((h+1)/64)` words.
- Unbounded maximum: values `0..=min`, with existing saturation semantics.

Scope labels distinguish Outside from active count zero. Finite bit h is useful
in transfer calculations; incoming CInc values are below h because continuation
excludes h. Never derive allocation size from the unbounded `usize::MAX` sentinel.

Precompute final-word masks, guard boundaries/masks, and successor descriptors.
Use flat slices and index newtypes. Reuse existing byte tables; do not allocate
another 256-entry table per ByteTable state. No per-byte temporary allocation.

### Experimental admission limits

Initial values to make the prototype bounded, not production memory targets:

- 4,096 states in the entire optimized state array, checked before allocating
  StateIdx-indexed analysis arrays. Only reachable states receive runtime rows.
- 1,048,576 examined edge entries, including ByteTable entries.
- 8,192 total row words W: at most 64 KiB raw frontier payload per stream.
- 1 MiB of new compiled-plan arrays, accounted from the actual layout.
- 2 MiB of transient analysis arrays in addition to the retained-plan budget.

Check arithmetic and limits before allocating. The first milestone additionally
requires each counted domain endpoint <=63: `0..=64` already needs two words.
Report actual bytes, coverage, and rejection reasons. Production selection must
account for hundreds of patterns per stream; 64 KiB per pattern is not a goal.

## 4. Exact transfers

Process only nonempty rows. Transfers are unions of scalar pointwise transfers.
Do not replace sets by intervals, extrema, or sparse representatives.

| Operation | Grouped action |
|---|---|
| Union | OR rows with identical scope/domain |
| Split | Union into both successors |
| CI(c) | If Outside is present, insert count zero at its successor |
| Finite CInc | T = `{v+1 : v in S}`; continue with T intersect `[0,h-1]`; set Outside at break target iff T intersects `[min,h]` |
| Unbounded CInc | T = `{min(v+1,min) : v in S}`; continue with T; set Outside at break target iff T contains min |
| Assert | Preserve row only on existing evaluator's Pass |
| Consuming predicate | Test once per active state; on success transfer row to next-boundary successor(s) |
| ByteTable | Consume one byte and transfer to its selected non-NONE target |
| Match | If Outside is present, set sticky boolean match result |

Exit forgets the completed count because the proven scalar scope ends there.
Do not combine incompatible exit/continue payloads. For finite increments use
shift-with-carry and masks; test cross-word carries and high padding. For
saturation, retain an existing min bit as well as shifting smaller values;
min=0 must work. Use `counter_step` as a unit-test oracle.

## 5. Streaming and assertion replay

### Ownership

Persist **raw post-consumption roots**, not partially closed state. A stream
owns one fixed-size W-word frontier, previous byte, at-start flag, sticky match flag, and
lifecycle/plan identity metadata. Worker scratch owns closure and next-root
arenas of W words each and is borrowed only during a call.

After consumption copy scratch.next's W-word logical region into raw, then clear
scratch before reuse. Do not swap worker Vec allocations into streams: after a
large plan, that would spread the worker's high-water capacity into many small
streams. Use fixed-size stream storage, such as `Box<[u64]>`; count the O(W)
copy in the baseline.

Different streams can share scratch sequentially; simultaneous calls need
separate scratch. Reserve capacity before entering scanning. Frontier payload
is `8W` bytes per stream and `16W` per worker for this plan, plus metadata and
allocator overhead. Worker scratch may retain larger capacity from previous
plans; report that high-water memory separately. Test large-plan then small-plan
interleaving to ensure small stream allocations remain fixed-size.

### Closure

`close(raw, boundary)` clears closure scratch, copies all raw rows, and inserts
Outside at the pattern start for search. Seed **all rows before** the topological
pass. Apply zero-width transfers in order; consuming rows remain for consumption.

Pass boundary flags and neighboring bytes to `AssertKind::eval`. Only Pass
traverses the assertion. Fail and Defer stop this evaluation, but immutable raw
roots remain available for reevaluation. In particular, End fails before EOI
and must still become eligible at finish. Keep the raw frontier immutable under
this plan's invariant. In particular, writing incremented values back into their
original pre-increment CInc row would apply the increment again on replay.

This does not mean every closed-frontier design is incorrect: a full exact
closure retaining original roots, blocked assertions, and state identities can
have an idempotent-closure argument. That is a different representation and proof.
This plan deliberately keeps raw roots; retaining only consuming leaves or only
Defer assertions would lose candidates such as End, which currently returns Fail.

### Assertion contract: one immutable byte boundary

Every raw row belongs to the **same current byte boundary**, even when its
paths began at different positions. Every assertion in one closure, including
chains separated by counter operations, sees the identical boundary context.
Zero-width traversal never advances the position, updates prev/at_start, or
consumes the lookahead byte.

Use an explicit boundary-mode type rather than unrelated optional flags:

| Mode | Next byte | EOI | Meaning |
|---|---|---|---|
| UnknownNext | None | false | Current chunk has ended; future input is unknown |
| BeforeByte(b) | Some(b) | false | b is the next byte and is still unconsumed |
| EndOfInput | None | true | Caller has finished this stream |

Reject inconsistent internal contexts such as a known next byte together with
EOI. On unmatched streams, `at_start` remains true until the first consumed byte
and prev is None exactly at stream start. Chunk boundaries are not input starts
or ends. Empty chunks do not resolve assertions.

The current evaluator reads only this global boundary context. Thus the same
assertion at the same state has one result for all grouped counts and origins.
This is why exact grouping requires no per-origin assertion history here.
Revisit the argument before adding any assertion that reads other context.
`\B` is a boundary predicate, not a complement of the reachable-count set;
its transfer still distributes over union.

Raw roots persist only while resolving their current boundary. On consumption,
replace raw **entirely** by the consuming successors. Never append old failed
or deferred roots to the next boundary: they would be able to revive later.
Likewise, a failed body assertion gates only its own path; an unasserted bypass
must remain independent. Convergence permits union only at the same state and
scope, after the predecessor's guard has passed.

### All current assertion kinds

In this table `s` is stream-start, `w` is whether prev is an ASCII word byte,
and P/F/D mean Pass/Fail/Defer. A missing prev is neither a word byte nor CR/LF.
These are required tests against the shared evaluator, not a second production
implementation of assertion logic.

| Kind | UnknownNext | EndOfInput |
|---|---|---|
| Start (`\A`, non-multiline `^`) | P iff s, otherwise F | Same |
| End (`\z`, non-multiline `$`) | F | P |
| StartLF | P iff s or prev=LF, otherwise F | Same |
| EndLF | D | P |
| StartCRLF | P if s or prev=LF; D if prev=CR; otherwise F | P iff s or prev=LF/CR |
| EndCRLF | D | P |
| WordAscii (`\b`) | D | P iff w |
| WordAsciiNegate (`\B`) | D | P iff not w |
| WordStartAscii | F if w, otherwise D | F |
| WordEndAscii | D if w, otherwise F | P iff w |

Known-byte evaluation must have no Defer. EOI evaluation must have no Defer.
UnknownNext Pass must remain Pass for **every** next byte and for EOI. This
last condition justifies sticky matches found during provisional closure. Fail
has no such general stability guarantee: End is deliberately F before EOI.

### Call sequence

```
start/reset:
    raw = empty; at_start = true; prev = None; matched = false
    close(raw, next=None, at_end=false)

chunk(nonempty bytes):
    for b in bytes:
        close(raw, next=Some(b), at_end=false)
        if matched: return
        clear scratch.next
        consume matching closure rows into scratch.next
        copy scratch.next's logical region into fixed-size raw
        prev = Some(b); at_start = false
    close(raw, next=None, at_end=false)

chunk(empty): no semantic change

finish:
    if matched: return true
    close(raw, next=None, at_end=true)
    return matched
```

Once matched, subsequent work can short-circuit under the boolean API. Enforce
finish/reset lifecycle in the wrapper. Constructor closure makes `ismatch()`
correct before the first chunk. End-of-chunk closure discovers definite matches
without assuming EOI. Empty chunks remain O(1).

A matched stream is an absorbing result: it may stop consuming the supplied
slice and need not maintain a frontier for its unused suffix. Reset must restore
all boundary flags and rows. Every unmatched stream must obey the full frontier
invariant. At EOI perform closure only: never invent a sentinel byte or consume
a tail reached through an end assertion. In BeforeByte mode, an assertion can
open a consuming tail that uses b in that same iteration, exactly once.

For `\Ax(a|aa){2}\z`, feeding `xaa` must not yet match; finish must match;
feeding another `b` before finish must fail. Word assertions resolve when the
next byte arrives; CR/LF split across calls remains visible through prev/next.

This deliberately trades closure recomputation for a simple invariant. The first
runtime excludes assertions; enable them only after the full schedule is tested.

### Proof obligations

1. Expanding raw grouped rows gives exactly the scalar configurations immediately
   after consumption and before further zero-width traversal.
2. For fixed boundary context, topological closure equals scalar zero-width
   reachability, by induction over the order.
3. Transfers distribute over union; consumption and search reseeding preserve
   the invariant.
4. Unknown-next Pass before EOI is stable under every later byte revelation
   **and EOI** in the current evaluator. Test this property; reassess it for
   future assertion kinds. All assertions in a pass share one immutable context.
5. Next-byte/EOI reevaluation starts from original roots, not incremented closure.
6. Consuming a byte replaces the old frontier; unresolved assertions cannot
   migrate to a later boundary. Search reseeding also occurs at EOI, without
   resetting existing counts or the global at_start flag.

## 6. Cost model

Let R be reachable states, W total row words, and B the sum of row-word work
across transfers in one closure/consume phase. Count both successors, empty-row
checks, and guard reductions; multiword nonempty checks are not O(1).

- Analysis is linear in examined graph size plus descriptors, within budgets.
- A closure/consume phase costs `O(R + W + B)` with a full sweep.
- L bytes and J chunk calls cost `O((L+1)*(R+W+B) + J)`. Each nonempty chunk
  adds one provisional closure; there are at most L such chunks.
- Frontier storage is O(W) per stream and per worker, plus the shared plan.

This is input-linear and allocation-free within scanning after preparation,
**not independent of count bounds**. Full sweeps can lose on sparse frontiers.
Active-row scheduling is a later measured optimization with its own ordering
invariant, not part of the initial implementation.

Implemented after P4 (see "Idle cost"): with an active-row bitmap per
frontier, an active pass costs `O(A + live rows and their transfers)` per
byte, `A = ceil(R/64)`; the full sweep stays `O(R + W + B)`. While the raw
frontier is empty and the start closure is static, prefilter skipping makes
idle bytes cost the prefilter's scan. A frontier where no counter has
counted yet takes the count-free step instead: `O(A)` words per live node
plus one per firing consumer, from compile-time bitmaps (see
"Count-free step").

## 7. Milestones

For P0–P4, keep tiers 0–4, their oracle, and `Regex::min_tier()` unchanged.
Use an explicit backend name, not Tier 5. P5 adds automatic selection without
renumbering the tiers. Each milestone is independently reviewable.

### P0: analysis and opt-in plan

- Add `GroupedNfaPlan`, scope/row descriptors, order, masks, eligibility reasons,
  checked sizes, and structural fixtures in the new module.
- Add a default-off experimental compile option. Analyze in `RegexBuilder::build`
  after graph optimization; store the plan/result on Regex. Never analyze during
  matcher construction or scanning.
- Keep Regex layout consistent between test and non-test builds; a `cfg(test)`
  field would invalidate memory-footprint comparisons.
- Update info/dump/debug and memory accounting for all new plan allocations.
  Do not claim unrelated accounting gaps are fixed. Report requested mode,
  eligibility/rejection, persistent bytes, and scratch bytes separately.
- Gate: structural checks and forced-counter examples pass; a plan-budget
  rejection leaves existing compilation/dispatch usable.

### Implementation notes (P0/P1)

- `grouped_nfa/analysis.rs` checks the structural conditions (scope proof,
  zero-width cycles) before the milestone exclusions, so a graph that is
  outside the class for good is reported as such. `Limits` makes every
  budget a parameter; the tests cover each below/at/above.
- The `Regex` field is `Option<Box<GroupedNfa>>`: 8 bytes on every regex
  (all `memory:` values re-blessed), plan bytes only when requested.
- `run_match_test` also compiles each build with the option and, when
  admitted, runs the grouped stream on the listed inputs (against Tier 0
  prefix `ismatch` and the oracle, whole/bytewise/chunked with empty calls)
  and on the generated inputs. At P1 about 210 of 1,485 builds were
  admitted, about 1,140 rejections being assertions; at P3, 1,372 are
  admitted, and the 113 rejections are nesting (83) and zero-width cycles
  (30).
- `(?:\b){2,3}` compiles to a lone `\b` without a counter; the
  zero-width-cycle test uses `(?:\b|a){2,3}` instead.
- P2 drops the per-counter limit: a counter is admitted when the frontier
  fits `max_row_words` (8,192 words: `a{N}` has two counted rows, so N up
  to about 260,000; the default `max_repetition` of 1,000 is far below).
  The increment computes word masks on the fly rather than storing them.
- P3: admission accepts an assertion kind only if `analysis::reviewed`
  lists it; that match is exhaustive, so a new kind does not compile until
  someone decides. All ten current kinds are listed, and
  `evaluator_contract_for_every_kind` checks the §5 table, no Defer with a
  known next byte or at EOI, and Pass stability, over every boundary.
- P3 tests (`grouped_nfa/assertion_tests.rs`): every release-gate row under
  all three compile configurations, with empty calls after every data call
  and `ismatch` checked against the hand-written expectation and Tier 0;
  every partition and every prefix `finish` of each gate input; all 100
  ordered kind pairs at five positions (head, entry, body, exit, tail),
  each with a finite `{1,3}` and a saturating `{2,}` counter, under two
  configurations, over word, non-word, NUL, 0xFF, CR and LF bytes, against
  Tier 0, the oracle and the scalar interpreter (none rejected); every cut
  of short inputs for a subset. Hand-built graph fixtures cover shapes
  that pattern normalisation might not produce: a guarded exit and a
  guarded bypass sharing a tail state, guarded and unguarded body branches
  sharing an increment, contradictory guards sharing an assertion node,
  guarded and consuming entries sharing a counter instance, and a guard
  between a finite and a saturating counter. Each runs every partition of
  every input up to 4 bytes against Tier 0 (which parks deferred
  assertions) and the scalar interpreter, and must have both matching and
  non-matching inputs. Mutations
  detected by these tests: old roots kept after consumption, prev updated
  before lookahead evaluation, no EOI replay, EOI as a consumed sentinel
  byte, Defer treated as Pass.
- P4: the checkers have a `Backend { Tier(u8), Grouped }` in failures,
  shrink targets and trace commands. The default selection (and
  `--skip-tier`) includes the grouped NFA when it admits a pattern, so the
  CI fuzz and sweep jobs cover it; the backend list says why it is missing
  otherwise, and `fuzz` counts grouped checks and rejections apart from
  the tiers. `--grouped-nfa` (brute, fuzz, shrink, sweep) checks only the
  NFA and the grouped NFA and rejects patterns it does not admit; it cannot
  be combined with `--tier`/`--skip-tier`. `match`/`info`/`dump` take
  `--grouped-nfa` too. Deviation: the checkers compile one regex with the
  option for every backend instead of a second regex; the option never
  changes the NFA (`option_leaves_the_nfa_unchanged`, and
  `run_match_test` compares the two builds).

### P4 findings

From `scripts/research_counting_probe.py --grouped` (4 KiB non-matching
inputs, one machine; numbers stay in local artifacts):

- **Large wins where the existing paths scale with the number of live
  counts**: a consuming prefix before an ambiguous variable body
  (`a(a|aa){1,N}b`, `a(a|aa){N}b`). Tier 3 and the NFA grow roughly
  linearly in N per byte (Tier 3 times out at N=1000); the grouped NFA
  stays within a small factor of its N=8 cost. Byte-at-a-time chunks keep
  the advantage.
- **Modest wins** on `(a|aa){1,N}b` up to N=256 against Tier 3; a loss at
  N=1000, where the multiword rows cost more than Tier 3's ranges.
- **Losses** wherever a DFA tier applies cleanly: Tier 1 (unrolled or
  counter-free patterns), Tier 2 on fixed-length bodies (`a.{N}b`,
  `(ab){1,N}c`), Tier 3 on `x(a|aa|b){N/2,N}z` beyond small N.
- **Large losses on sparse activation and counter-free controls** (at
  P4): the runtime swept every row at every byte and had no prefilter, so
  an idle stream cost the full closure per byte while the tiers skipped
  ahead. Fixed afterwards; see "Idle cost".
- Nested families are rejected (by design).

So the grouped NFA is not a general replacement. Its case for P5 is a
dispatch rule that picks it where Tier 3/the NFA degrade with ambiguous
variable bodies, after idle-stream cost is fixed. Not yet measured:
allocation counts, operation counters, true cold setup, and hundreds of
rules over thousands of interleaved streams.

### Idle cost

Two changes, both exact (every pass gives the same rows):

- **Active rows.** Each frontier carries a bitmap of its nonempty rows,
  one bit per node (a row whose bit is clear is zero; scratch arenas are
  zero between calls). The active pass visits only set bits, always the
  lowest pending one; transfers activate only higher nodes, so nodes are
  still processed in topological order. Consumption cleans the closure as
  it reads it.
- **No next-root arena.** The closure holds a copy of every raw row, so a
  byte's consumption clears the raw rows and writes the successors straight
  into them (the early-match return comes before, leaving them intact).
  This departs from §5's ownership paragraph: worker scratch is the closure
  arena and its bitmap only, `8(W + A)` bytes, and a stream is
  `8(W + A)` bytes plus its struct, `A = ceil(R/64)`.
- **Prefilter while idle.** When the raw frontier is empty and the start
  state's zero-width closure has neither an assertion nor `Match`
  (`skip_when_idle`, decided at analysis), the closure of an empty
  frontier is the same at every boundary and never matches; bytes the
  regex's prefilter rules out leave the frontier empty and are skipped,
  with `prev`/`at_start` advanced. The prefilter itself already bails on
  assertions and empty matches.

Measured with the probe (plus a `wide` family: 12 counted alternatives,
input keeping one alive, and the forced `grouped-full`/`grouped-active`
paths):

- The active pass loses to the full sweep by up to 2× on small or dense
  plans (bookkeeping per row), and wins by up to 4× on wide plans with few
  live rows. Each byte therefore picks: the full sweep for plans of at
  most 32 rows or with at least one raw row in eight active, the active
  pass otherwise. The automatic choice stays within a few percent of the
  better pass on every family.
- Idle streams (`sparse`, `control`) drop from tens of ns/byte to the
  prefilter's speed, close to the tiers'.
- `wide` runs one to two orders of magnitude faster than Tier 3 and the
  NFA, which pay per live instance.
- Dense workloads run at the P4 full sweep's speed (median ratio 0.98),
  after successors started carrying their row offset (no node lookup per
  transfer) and the full sweep started walking contiguous copies of its
  zero-width and consuming nodes. One reproducible exception:
  `(ab){1,64}c` runs at about 1.45x the P4 time, while N = 63 and 65 run
  faster than P4 at N = 64. Callgrind counts fewer instructions and
  simulated mispredictions for N = 64 than for N = 256, so the cause is
  outside its model (alignment or store forwarding, say); not explained
  without hardware counters.

Tests run every check in all three modes (automatic, full, active),
compare both closure passes with the scalar interpreter, check the bitmap
invariant and a clean scratch after every call, and include a plan whose
automatic choice switches pass within one chunk.

### Count-free step

Investigating the regression that P5 accepts on the `reengage_profile`
gate (bounded `.{0,N}` gaps over text, where Tier 3 runs as a DFA)
showed that the grouped NFA was not slow at counting there: in the
`tier3_*` cases a counter is almost never in progress. The frontier is
empty, or holds one to a few rows with count 0, and each byte paid the
general step's fixed cost (closure pass, consumption, bookkeeping).

A raw frontier is *unit* when every active row holds exactly bit 0
(Outside, or count 0). A node is *count-free* when its zero-width
closure reaches no assertion and no `CounterIncrement`. From a unit row
such a closure is a fixed set of nodes, each holding bit 0, whatever the
boundary. So the analysis builds, at compile time
(`CountFreeTables`), when the start node is count-free and the bitmaps
are at most 8 words (512 rows):

- the closure bitmap of each count-free node, and of the start;
- per byte, the bitmap of consumers that accept it;
- per byte, whether it starts anything from an empty frontier.

While the raw frontier is unit and all its nodes are count-free, a step
is: closure = OR of the nodes' closure bitmaps and the start's;
`Match` reached if that hits a match node; successors of the consumers in
`closure AND accept[b]` become the next raw frontier, each holding bit 0.
While the frontier is empty, the stream skips to the next byte that
starts something (prefilter, or the per-byte table when there is none).
One const-generic loop serves every width from one to eight words: it
runs count-free steps back to back with the frontier as a bitmap in
registers, and writes the raw rows once when the run ends (the rows
that changed: cleared or set to bit 0). A run ends at the end of the
chunk, on a match, when the frontier empties (the idle skip takes over)
or when a node that is not count-free becomes active, so the raw rows
are exact at every call boundary and before every general step. Anything else, and every byte where a
counter has counted, takes the general step. Whether the next frontier
is unit is decided exactly during consumption: it is unit when every
consumer that moved a row moved exactly bit 0, one comparison per firing
consumer, so no step rescans the rows. The tables count in the plan
budget and in `memory_size()`: 256 bitmap words for acceptance plus one
per node, so about 2.3 KiB at one word. They are optional: a plan over
the budget only because of them is admitted without them. Acceptance
bitmaps per byte class instead of per byte would shrink the larger
tables (128+ nodes); not done.

The count-free step runs under the automatic choice only; forcing a pass
(`Sweep::Full`/`Active`, `grouped-full`/`grouped-active` in the probe)
runs the general step. The tests run every check in the three modes, so
they compare the two on every input they cover, and a dedicated test
alternates count-free and general steps within chunks. Deliberate
breakage (a dropped `out_exit`, no search reseeding, an assertion taken
as count-free, a skipped unit check) fails them.

Measured on the gate cases: the cases without a prefilter become an order
of magnitude faster than Tier 3, the rule and gap text cases come within
about 15% of it, the two dense-candidate synthetic cases (one live row,
a step per byte) stay about 1.6x slower, and the counted-instance control
is faster than Tier 3. Idle and counter-free probe families get faster;
counter-heavy ones keep their median. The exception: the two families
with five-word rows (`a(a|aa){1,256}b`, `(ab){1,256}c`) run 15-25%
slower, with the same instruction count, fewer simulated mispredictions
and the same cache misses, while `(ab){1,64}c` lost its earlier unexplained
slowdown. Accepted for now as a known cost on dense multi-word workloads.
Hardware counters (Haswell, `perf`) place it in the back end: cycles with
the reorder buffer full rise from almost none to about 70 per byte, with
equal L1, TLB and split-access counts and about a third more blocked store
forwards. Precise samples put the stall on the destination load of the
multi-word OR loop in consumption, whose instructions are identical in both
builds: the same code stalls depending on the stores just before it
(`memset` zeroing, earlier copies). Copying instead of OR-ing a first
contribution, a scalar OR loop, an out-of-line general step, 64-byte loop
alignment and heap padding did not fix it. Next leads: a top-down slot
breakdown and `perf c2c`/`perf mem` to identify the store each load waits
on, or consumption that never reads back a row written in the same step.
Keeping a count-free frontier as a bare bitmap, writing rows only when the
general step takes over, is the obvious next step for the dense-candidate
cases.

### P1: one-word exact, assertion-free runtime

- Implement finite domains <=63 and the topological runtime.
- Add experimental stream-state and worker-scratch types with explicit
  start/reset, chunk, finish, and ismatch operations. Scratch is passed per call;
  do not retain its borrow across a suspended stream. Keep this runner separate
  from AnyMatcher's current lifetime-long memory borrow initially.
- Reserve once and reuse capacity. Implement Display/Debug for raw state, scopes,
  exact counts, boundary flags, and chosen row widths.
- Cover sequential scopes, reentry, ByteTable, out_exit, empty input/chunks,
  early success, and reset. Use raw byte predicates; no prefilter or byte-class
  compression in this version.
- Gate: grouped expansion agrees with an independent scalar fixture interpreter;
  pattern results agree with unchanged Tier 0 and regex. Instrumentation confirms
  one predicate test per active state per consuming pass, not per count.

### P2: multiword and unbounded domains

- Add carry propagation, padding masks, and saturation; lift the one-word limit
  only within total-word admission.
- Test endpoints 63/64/65 and 127/128/129, and long saturated streams.
- Gate: scalar transfer/property tests and boundary patterns pass; no capacity
  growth after preparation. Raised limits in experiments are explicitly recorded.

### P3: assertions using raw replay

- Implement Section 5 in full, using the existing assertion evaluator.
- Enable all existing assertion kinds only after differential coverage, including
  body assertions, chains, and split CRLF. Keep the zero-width-cycle rejection.
- This is a mandatory milestone before production selection, not an optional
  assertion wrapper around a finished consuming engine.
- Test the full evaluator truth/stability contract using `AssertKind::ALL`.
  New kinds must fail the reviewed coverage gate until their boundary requirements
  and provisional-pass behavior have been checked.
- Gate: prefix ismatch and finish agree across all cuts of small inputs and
  randomized larger chunks. Use the unchanged NFA's unfinished-stream ismatch
  as the prefix oracle: `regex.is_match(prefix)` treats the prefix as complete
  input and is not equivalent. Test Fail-before-EOI as well as Defer cases.
- Also compare to an independent scalar parked-assertion model; do not give both
  models the same scheduler. Use the regex crate for final whole-input results.
  Resolve oracle disagreements before enabling a class rather than copying a
  suspected oracle error into the design.
- Run the assertion-specific gate below with forced counters and default
  compilation. Report graphs executed separately from graphs rejected by admission.

### P4: tools, differential coverage, and deployment evaluation

- Add explicit `--grouped-nfa` selection to match/dump/info and checking commands;
  reject simultaneous `--tier`. Existing default dispatch remains unchanged.
- Introduce a checker backend selector (`Tier(u8)` / `Grouped`) and update
  execution, failures, shrink/reproduction, and coverage counts together.
- Extend `test_all_tiers`, `test_generated`, proptests, and cargo-fuzz targets to
  run grouped when admitted. Enumerating 0..=4 alone does not test this backend.
- Preserve default memory assertions. Compile an additional opt-in Regex for
  grouped checks under each existing unroll/merge regime; confirm that its NFA
  is otherwise identical. Test plan accounting explicitly.
- Extend the probe with grouped execution, true cold setup, allocation counts,
  operation counters, and interleaved streams with shared worker scratch.
- Gate: reproducible results for the matrix below, including rejections and
  scratch reuse across plans. No silent backend omission in test summaries.

### P5: automatic selection — proposed implementation contract

Status: implemented as specified (see "P5 implementation notes" at the end
of this section). The contract below is kept as written.

This is the implementor handoff for the next change, not a claim that dispatch
has already changed. It supersedes the earlier opt-in-only requirement for
P0–P4. The initial policy is deliberately structural: no timing-derived count
thresholds, runtime benchmarking, or mid-stream switching.

#### Selection policy

Choose the first available engine in this order:

1. Tier 1, when eligible.
2. Tier 2, when eligible.
3. Grouped NFA, when admitted by its existing analysis and limits.
4. Tier 4, when eligible.
5. The original NFA, otherwise.

Tier 3 is excluded from automatic selection, including when grouped admission
fails. Keep it explicitly selectable for research, reproduction and differential
testing. Known correctness defects are a reason to exclude it from production
selection, independently of whether it wins a particular benchmark. Do not alter
its eligibility to hide defects, or investigate/fix those defects in this change.

Retain Tier 1 and Tier 2 precedence: the available probes favor them on small
unrolled and fixed-length counting workloads. This is an initial policy, not a
proof that they always win. Grouped takes precedence over Tier 4 where both are
eligible; nested or otherwise rejected patterns can still use Tier 4. Grouped
also replaces the automatic NFA fallback wherever admitted. Explicit Tier 0
must continue running the original NFA as an independent reference.

#### Compile-time decision and configuration

- Compute and store one selected-engine identifier during compilation, after
  eligibility analysis. Use a named grouped variant; do not assign Tier 5 or
  encode grouped as Tier 0. Runtime dispatch and diagnostics read this same
  decision rather than independently repeating the ordering.
- Default compilation must consider grouped after Tier 1/2 rejection. Avoid
  building an unused grouped plan for Tier 1/2 winners unless explicit grouped
  analysis/execution is requested. Admission failure selects the next candidate;
  it does not reject an otherwise compilable regex.
- Make configuration semantics explicit: normal automatic selection, an explicit
  way to disable grouped participation, and explicit analysis for forced grouped
  execution even when Tier 1/2 wins. Migrate/document the current opt-in
  `RegexConfig::grouped_nfa` boolean accordingly; do not leave default `false`
  silently preventing the new selection. API spelling is an implementation
  choice, but these behaviors and their defaults must be unambiguous.
- Preserve `min_tier()` as the legacy numbered eligibility query, with its
  limitations documented. It must no longer be presented as the selected engine
  or a speed ranking. Add a selected-engine query and update callers that need
  actual automatic dispatch. Forced numbered tiers retain independent eligibility.
- Keep existing grouped structural/resource limits. Do not introduce match-time
  budgets, approximate answers, or a new performance cutoff from local timings.

#### Matcher, diagnostics and tools

- Integrate grouped into the normal matcher interface, including `chunk`,
  provisional match queries, `finish`, and diagnostic formatting. Preserve the
  existing raw-frontier assertion contract and arbitrary chunk boundaries.
- Reuse grouped scratch and stream storage across matches through the normal
  memory-owner lifecycle. Reset correctly when switching regexes or engines;
  avoid fresh per-match/per-byte allocation once capacity suffices. Do not retain
  unrelated engine caches merely to construct a grouped matcher.
- Preserve the explicit grouped streaming API and its separation of persistent
  stream state from reusable worker scratch. Automatic integration must not
  require every independently managed stream to own worker scratch.
- Report the selected engine separately from eligible engines and grouped
  rejection reasons in info/dump/checker output. Update Display/Debug for every
  added field. Preserve explicit `--tier 0` and `--grouped-nfa` selection and
  their existing conflict/error handling; forced requests never silently fall back.
- Keep differential coverage of all eligible engines. Removing Tier 3 from
  automatic dispatch must not remove its explicit tests or known-defect markers.

#### Acceptance checks

- Assert selection for every branch: Tier 1; Tier 2; grouped over eligible Tier 3
  and Tier 4; grouped where the original NFA was the only automatic path; grouped
  rejection followed by Tier 4; and rejection followed by NFA. Assert that Tier 3
  is never automatically selected. Use actual compiled eligibility/admission,
  not assumptions based on pattern appearance.
- Test default/disabled/explicit grouped configuration, diagnostics and forced
  selection. Explicit Tier 0 must still execute the independent reference.
- Exercise the automatic matcher against the original NFA and regex oracle under
  all existing unrolling/merging regimes, whole input and byte-sized chunks.
  Include empty chunks, reset/reuse, success and failure, finalization-only
  matches, CRLF boundaries, and deferred word/line assertions. Reuse memory
  across different patterns and across engine choices.
- Run debug/release suites, clippy, formatting, grouped assertion sweeps and
  bounded differential fuzzing. Retain known Tier 3 handling in those checks;
  do not turn this integration into a Tier 3 debugging task.
- Run the existing scanning performance gate and compare automatic selection
  with forced competitors. Include successful and unsuccessful scans, small
  chunks, cold construction, reset/reuse, and allocations. Account separately for
  compiled plan, stream state and worker scratch. Update compiled-memory test
  expectations only for explained changes; preserve tier-eligibility assertions.
- Keep machine-specific measurements outside committed documents. Record remaining
  deployment-evaluation gaps explicitly: this policy does not establish a global
  throughput optimum or readiness for hundreds of interleaved rules/streams.

Deliver selection, configuration/API integration, diagnostics and regression
coverage together. Sparse count sets, changing Tier 1/2 precedence, and runtime
adaptation remain separate follow-ups requiring their own evidence.

#### P5 implementation notes

- `RegexConfig::grouped_nfa` is a `GroupedNfaMode`: `Auto` (default),
  `Disabled`, `Always`. Under `Auto` the plan is built only when neither
  Tier 1 nor Tier 2 is eligible; a rejection is kept on the regex (96
  bytes) so that diagnostics can say why.
- `RegexBuilder::build` computes `Engine` (`Tier1`, `Tier2`, `GroupedNfa`,
  `Tier4`, `Nfa`) once; `Regex::engine()` reads it; `MatcherMemory::matcher`
  dispatches on it (numbered tiers through `matcher_for_tier`), so there is
  no second copy of the order. `min_tier()` is unchanged and documented as
  the legacy eligibility query.
- `MatcherMemory` owns a grouped scratch and one stream, rebound to each
  regex it runs (`GroupedStream::rebind` reuses the allocation when its
  capacity suffices, and for the same regex only clears the active rows,
  like `reset`); `AnyMatcher::Grouped` wraps them.
  `MatcherMemory::matcher_grouped` forces the grouped NFA and fails rather
  than falling back. `GroupedStream` with a separate `GroupedScratch` stays
  available for independently managed streams.
- Diagnostics: `info` reports the selected engine, every eligible engine
  and the legacy `min_tier`; `dump` reports the selected engine; checker
  backend lists add `auto=<engine>`; `match` reports the engine it ran.
- Tests: every selection branch on actual compiled eligibility, Tier 3
  never selected nor its matcher returned, the three modes, forced
  selection without fallback, explicit Tier 0 as the reference,
  diagnostics, one memory reused across regexes of several engines
  (grouped storage kept across a smaller plan), and in every `match_tests!`
  build the automatic matcher whole, bytewise with empty calls (`ismatch`
  against Tier 0 after each call) and on the generated inputs. One test
  that measured the Tier 3 cache through `matcher()` now forces Tier 3.
- Memory: 71 `memory:` values changed, each by its regex's grouped bytes:
  62 entries now select the grouped NFA and carry its plan (count-free
  tables included), 9 carry a rejection record.
- The `reengage_profile` gate (Callgrind, against `main`) regresses on 4
  of its 19 cases, as accepted when P5 was decided: the two
  dense-candidate cases (+43% and +80% instructions) and rule and gap text
  (+14-17%). Seven `tier3_*` cases improve, the no-prefilter ones by more
  than 12x in instructions; the Tier 1 and Tier 2 cases and controls are
  unchanged. The case names still say `tier3`: they keep their
  `min_tier()` assertions and now measure automatic selection.
  After the review fixes (count-free runs that write the rows once), the
  same gate against `main`: `tier3_dense_candidates` +3.2% instructions,
  rule and gap text +0.9% and +1.2%, `tier3_candidate_run` -32%; every
  case is within the 5% instruction limit. `tier3_dense_candidates` still
  mispredicts 17x more conditional branches than Tier 3 (66K against
  3.9K, halved from 132K), over the gate's 100% limit: its frontier
  follows random input, as Tier 2's dense case does (66K). The counting
  probe: count-free families 5-35% faster than before the fixes; the
  counter-heavy ones at bound 1000 3-11% slower in time with the same
  instruction count (code layout, as above).
- Not established: a global throughput optimum, readiness for hundreds of
  interleaved rules or streams, allocation counts, or successful-scan and
  reset costs beyond the probe's unsuccessful scans. A `MatcherMemory`
  shared between regexes on two tiers among 1-3 still panics (issue #5,
  open), independently of this change.

## 8. Test and measurement matrix

Pattern regressions belong in `match_tests!`; never set `unroll_limit: 0` in
entries. Existing reruns force counters. Structural and bitset mechanism tests
belong in the new module.

| Pattern / family | Required observation |
|---|---|
| `x(a\|aa){2,5}z` | Multiple counts at one state; consuming prefix |
| `\Ax(a\|aaa){3}z\z` | `xaaaz` matches, `xaaaaz` does not; preserve holes |
| `\A(a\|aa){2}b(a\|aa){2}\z` | Sequential scopes remain distinct |
| `(a{2,3}b)*c` | Exit before ordinary-star reentry |
| `a{0,2}b` | Outside bypass and active loop |
| `x(ab*){2,5}z` | Variable/nonnullable body despite zero length metadata |
| `x(ab\|cd\|ef){2,5}z` | ByteTable and all destinations |
| `a{2,5}b` with default unrolling | Consuming out_exit edges |
| `a{63}b`, `a{64}b`, `a{65}b` | Staged eligibility and word boundaries |
| `a{2,}b` | Finite saturated domain on long streams |
| `\Ax(a\|aa){2}\z` | EOI replay without duplicate increments |
| `\b(a\|aa){2,5}\b` | Next-byte resolution across chunks |
| `(?mR:^)(a\|aa){2,5}(?mR:$)` | CRLF behavior after assertion milestone |
| `x(a?){2,5}z` with counters forced | Nullable-body/cycle rejection |
| `x((a\|aa){2,3}){2,3}z` with counters forced | Nested rejection |

Exercise every assertion kind, bytes 0x00/0xff/CR/LF and word transitions,
positive/negative and early/late outcomes, every cut on short inputs, empty
chunks, random large chunks, fresh/reused state, and interleaved streams.
Admission tests cover below/at/above each limit. Direct transfer unit tests
compare all small subsets to scalar counter_step and randomized multiword sets.

### Assertion-specific release gate

These are required behaviors, not results from an implemented grouped backend.
`/` separates chunk calls in the input column. Check ismatch after construction
and every call, including inserted empty calls; check finish separately.

| Hazard | Pattern and input | Required behavior |
|---|---|---|
| Absolute EOI | `\Aa{2}\z`, `aa` | false until finish, then true |
| Failed EOI must not migrate | Same, `aa` / `b` | finish false |
| Resolving byte feeds tail | `\Aa{2}\b!`, `aa` / `!` | false after aa; true after ! |
| Negative word assertion | `\Aa{2}\Bz`, `aa` / `z` | true after z |
| Failed boundary cannot revive | `\Aa{2}\b`, `aa` / `b` / `!` | remains false, including finish |
| Assertion before CInc | `\Ax(a\B){2}z\z` | xaaz true at finish; xaz false |
| Optional bypass independent | `\Ax(a\b){0,2}z\z` | xz true; xaz false |
| Exit gates downstream CI | `\Ax(a\|aa){2}\Bz{2}\z` | xaazz true; xaaz false |
| EOI opens a chain | `a{2}\z\b` | aa true only at finish; aab false |
| Contradictory chain | `a{2}\b\B` | aa and aa! both false |
| Empty EOI chain | `\z\B` versus `\z\b`, empty stream | first true only at finish; second false |
| CR start needs lookahead | `\A\r(?mR:^)a` | CR / a true; CR / LF / a false |
| No interior CRLF end | `\A\r(?mR:$)\n` | CR / LF false; LF-only variant `(?m:$)` true |
| EOI resolves CR start | `\A\r(?mR:^)`, CR | false provisionally, true at finish; adding LF instead fails |
| EOI cannot consume | `\Aa{2}\z\x00` | aa and aa-NUL false; `\Aa{2}\x00\z` matches aa-NUL |
| Guarded/unconditional convergence | `\A(?:a{2}\b\|a{2})!\z` | aa! true; a failed alternative must not poison a valid one |
| Assertion-only counted body | `(?:\b){2,3}` with counters forced | reject zero-width cycle rather than deferring indefinitely |

Generate all ordered assertion-kind pairs at body, entry, exit, and tail
positions, including contradictions. Exercise all ten kinds with word/nonword,
NUL, 0xff, CR, LF, and finite/saturated count domains. Add graph fixtures for
shared tails and guarded bypasses that HIR normalization might erase.

Run whole, bytewise, every partition of short inputs, and empty calls around
all data calls. Separately finish at every prefix on a fresh stream. Interleave
streams with different prev-byte categories through one scratch. Assert that
provisional closure leaves raw rows and boundary metadata unchanged and that
only consumption replaces raw with successors.

Mutation checks on the independent model must detect old raw roots carried
forward after consumption, incremented values written back to pre-increment
roots, blocked roots discarded, prev changed before lookahead evaluation,
omitted EOI replay, and EOI treated as a consuming sentinel. Do not require a
test to reject a semantically valid alternative representation merely because
it differs from this plan's storage invariant.

Performance families: `a(a|aa){1,N}b`, `a.{N}b`, `(ab){1,N}c`,
`x(a|aa|b){N/2,N}z`, and exact-count variants. Generate concrete bounds
8/64/256/1000 with recorded compiler limits. Include sparse-start and counter-free
controls to expose full-sweep overhead. Nested inputs measure rejection coverage
and existing baselines, not grouped throughput.

Report compile time/bytes, stream bytes, scratch high-water bytes, cold setup,
reset, ns/byte, chunk-latency distributions, copy/hash work avoided, and set
operation work. Include hundreds of rules and thousands of interleaved active
and idle streams. Disable instrumentation for final throughput runs.

Required implementation checks, not claimed executed for this document:

```
cargo fmt --check
cargo clippy --all-targets -- -D clippy::all
cargo test
cargo test --release
cargo build --release
python3 scripts/bless_memory.py --check
cargo test test_fuzz -- --ignored
```

Refresh memory assertions using a newly built release binary after adding Regex
fields. Run differential sweeps and a recorded `fuzz --batch` range with explicit
grouped coverage, plus matching/compile cargo-fuzz targets under repository
resource controls. Run the hot-path performance gate against the unchanged base
using isolated build directories, without switching another worktree's branch.
Existing commands without grouped coverage cannot certify grouped execution.

## 9. Optional sparse extension

The [sparse paper](../research_notes/regex-matching-research-2020-2026/text/2026-ncma-berglund-sparse-counting-sets.md)
separates exact grouping (Definition 12, Section 2.5) from sparse operations
(Definition 16, Lemma 20, Corollary 21). Its active counted positions store positive
counts; outside positions and the initial configuration use zero. Thompson CI
seeds zero and CInc follows body completion. Its guarantee
is for prescribed operations, not arbitrary integer-set manipulation.

For bounds `[5,10]`, its example represents `{4,5,6,7,8,9,10}` by `{4,10}`.
An unrelated `<=7` mask followed by a `[5,10]` check distinguishes those sets.
Sparse sets therefore cannot implement the exact bitset interface by silently
deleting values. Representatives can also disappear on sparse merge, invalidating
a naive monotone `new minus seen` worklist. Our topological schedule avoids that
worklist issue but does not establish transfer compatibility by itself.

Start a sparse extension when measurements identify domain width or merging as
a remaining bottleneck, or admission results show important patterns excluded
by exact stream-state costs. The latter cannot be timed on the rejected exact
backend; compare its estimated costs and existing admitted backends. Require:

1. A mapping from the admitted Thompson operations/zero-based convention to
   the paper's model, or a verified position-CA intermediate form.
2. An observation-preserving invariant distinct from exact reachability.
3. Proofs for reset, increment, exit, merge, assertion replay, and saturation.
4. Compile-time representative/storage bounds and fresh end-to-end measurements.

Keep exact grouped execution as an oracle and competitor. **The exact grouped
NFA is complete without this extension.**
