# Counting engine directions

Research baseline: `b0bb981e8c6cba7b23df176d79f741c3b61089a7` (main),
2026-11-17. The merged literature corpus is commit `67e5035`.
No engine implementation changes or existing Tier 1/2/3 bug investigations.
Proposals below are not validated implementations.

## Recommendation

The objective is a best-in-class production engine for internet-scale traffic:
correct streaming behavior, predictable adversarial performance, bounded memory,
and high throughput across many rules and concurrent connections. Publication
novelty is not a selection criterion. Reuse established techniques where they
work, and invent where measured gaps remain. A general theory of compression
must not block practical improvements.

Start with **one grouped nondeterministic execution core**, using exact count
sets first. Evaluate sparse compression separately once exact costs and admission
coverage are measured. Build the deployment-scale
benchmark and complete memory accounting alongside it. Then evaluate rotating
BVA, delimiter-based execution, and shared literal scanning where they address
remaining gaps. Do not implement every outlined approach before measuring the
first. Keep competitive existing paths; select among eligible representations
using work and stream-state costs, not tier number.

The architectural direction is a compiler selecting a counting representation
for each compatible region. Initially keep the experimental grouped NFA separate
from production dispatch and retain today's NFA as an independent oracle.
Establish each representation's semantics and performance before mixing execution.
Mixing counter representations without a joint invariant can lose correlations.
For nesting, investigate unique iteration boundaries and shared correlated
contexts separately. None of the checked papers supplies the desired general
fast nested-count algorithm.

The proposed selection, composition, and streaming adaptations need experiments
and correctness arguments. Their value is the production behavior they deliver.

A new deterministic tier is not a prerequisite: sparse counting-set algorithms
and NBVA execution offer direct NFA-level paths. Prioritize the representation
of active configurations before deciding whether to cache control transitions.
See Sections 6 and 7 for the NFA plan and Hyperscan comparison.

The concrete [grouped NFA implementation plan](../../plans/grouped-nfa.md)
specifies exact transfers, eligibility, streaming, memory limits, and milestones.
It is independently complete without sparse compression.

### Prioritized engineering plan

1. **Grouped NFA and deployment measurements.** Aggregate configurations by
   control position, add exact count sets, and compare performance on
   prefix-gated variable-body workloads. Account separately for immutable programs,
   worker scratch, and persistent stream state from the first prototype.
2. **Reduce baseline execution overhead.** Compile structural transfers and
   reduce context-copy/hash work. Measure startup, reuse, and adversarial costs
   independently of the counting representation.
3. **Add specializations where they earn their cost.** Compare rotating BVA
   against Tier 2, and evaluate shared scanning and repeat kernels using
   Hyperscan as architectural prior art. Retain default unrolling as a competitor.
4. **Address nested counting separately.** Start with narrower context layouts
   and shared joint contexts. More general compression remains an experiment
   until it demonstrates bounded work and useful memory savings.

Each step must produce evidence that determines the next investment. A design
that wins one synthetic family but worsens aggregate stream memory or common
traffic latency is not automatically a production improvement.

### How to read the examples

The patterns below use byte semantics and boolean search unless explicitly
anchored with `\A`/`\z`. They illustrate semantic eligibility, not measured
dispatch decisions or promised speedups. Some need a higher estimated-state
compile limit; force counters when testing a counting representation, and retain
default compilation as a competitor. `N`, `l`, and `h` are placeholders in the
earlier scaling families; the tables below use concrete regexes.

## Evidence and current costs

The [deployment brief](../../research_notes/engine-goal.md) targets many concurrent
streams and 500+ patterns per stream. Stream state is therefore a first-class
cost. Compilation must be bounded; matching cannot fail on a runtime budget.

The [benchmark method](counting-benchmark-method.md) describes a reproducible
probe of live counters on false-match streams. Keep its machine-specific results
and execution metadata as local or CI artifacts. Compare candidates on the same
hardware and build configuration; do not assume tier ordering is a speed ranking.
The probe does not measure total per-stream allocations or establish production
performance.

Source-grounded costs at this baseline:

- **Tier 2:** `rethoc-engine/src/dfa/tier2/mod.rs:53` uses a `Vec<u32>` delta
  ring per body phase. `reserve` at 87 preallocates a power-of-two capacity from
  the bound, initially capped at 4096; growth allocates and copies. `populate`
  at 568 creates body-length phases. Reset visits phases, and the counting path
  advances every counter's phase. Ring contents alone cost approximately
  `4 * sum(body_length * ring_capacity)` bytes. Bound-independent amortized
  arithmetic is not zero allocation latency or independence from pattern size.
- **Tier 2 eligibility:** `tier2/overlap.rs:1` requires nonempty break subsets
  to agree on successors/flags, with seed/reset consistency. Its bounded probe
  is not simply a test of fixed body length. Independent loop exits in a BVA
  could address a real coverage gap; optimizing deltas alone would not.
- **Tier 3:** `tier3/mod.rs:1350` performs linear membership search on instance
  insertion. Repeated distinct insertions can be quadratic in live entries.
  Range insertion at 1570 searches origins; live entries scan origin keys and
  collect effect vectors around 3187. Calling this path just `O(live instances)`
  omits work. Range eligibility at 925 depends on all counter entries being
  reachable without a consuming prefix/assertion. Construction at 3448 allocates
  both instance and range buffers, including the unused representation.
- **Tier 4:** `tier4.rs:1276` executes programs for explicit origin/valuation
  pairs. Branching can copy contexts; pool slots contain all counter slots
  (`lib.rs:3312`). A coarse domain bound involves the product of counter domains,
  not their sum. Not every pattern realizes that domain. State interning at
  `tier4.rs:424` lacks the Tier 1–3 state-cap mechanism; include this in resource
  accounting rather than assuming all caches have the same bound.
- **Ownership:** `lib.rs:3553` places mutable DFA caches in `MatcherMemory`,
  exclusively borrowed by the active matcher, not shared in `Regex`. Switching
  regexes rebuilds caches. One memory object does not preserve warm caches for
  500 patterns. Some counter buffers belong to the matcher itself.
- **Accounting/admission:** `Regex::memory_size()` at `lib.rs:1096` omits analysis
  heaps and runtime caches; it is not total deployment memory. Compilation at
  2610 checks the fully-unrolled estimate before compact construction. A future
  specialized backend needs an admission model for its actual representation,
  not automatic rejection using the expanded estimate.

These source-level costs motivate investigating representations and ownership, alongside
cheaper origin lookup and allocation reuse in existing tiers. These latter
optimizations would not by themselves eliminate growing count domains.

## 1. Sparse counting sets: classify by range width

The NCMA 2026 paper gives a stronger result than the survey's abstract-only
summary. For flat counted expressions its algorithm retains at most

```
d(l,h) = 2 * ceil(h / (h-l+2))     finite h >= 1
d(l,infinity) = 2
```

values per control-position set. Total state also scales with the number of
these positions. Theorem 24 gives `O(d * p² * L)` time, where `d` is the maximum
of these bounds across counters, `p` is predicate occurrences, and `L` input
length. Exact counts still have bound-dependent state;
wide ranges can have constant `d`. Normalize `h=0` separately. The guarantee
assumes the paper's flat automaton and operations, not arbitrary pruning of
existing counter contexts.

Source: [Sparse Counting-Sets, Lemma 18 and Theorem 24](https://arxiv.org/abs/2606.26679),
[committed full text](../../research_notes/regex-matching-research-2020-2026/text/2026-ncma-berglund-sparse-counting-sets.md).

Proposed backend: construct a flat position automaton with explicit counter
scopes. Store offset-based representative lists in preallocated slices. Implement
the paper's sparse addition and merge, keeping control-position identity on
branches. Never replace a set with its minimum/maximum interval: that fills
holes and invents configurations. Compile an upper bound on representative and
transition counts; initially admit small `d` only.

The opportunity is prefix-gated variable bodies: a consuming prefix blocks
today's range path without changing repetition slack. The proposed bound is
independent of that particular start-reachability restriction. This is an
inference about the proposed design, not a measured rethoc speedup.

Compare `x(a|aa){1,h}b`, `x(a|aa){h/2,h}b`, and `x(a|aa){h}b` as `h` grows.
The exact-count case is a negative control. Add holey sets, sequential counters,
and repeated entries. Initially exclude assertions inside bodies and nullable
bodies; extend only with a precise transfer rule and streaming oracle tests.

| Example pattern | Why it is useful | Expected limit |
|---|---|---|
| `x(a\|aa\|b){1,1000}z` | Flat, ambiguous variable body; `d=2` despite a consuming prefix | Each control position still needs its own representatives |
| `x(a\|aa\|b){500,1000}z` | Lower bound matters, but `d=4` | One latest count is insufficient |
| `x(a\|aa\|b){900,1000}z` | Narrower acceptance window gives `d=20` | Crossover case against exact bitsets |
| `x(a\|aa\|b){1000}z` | Same body, exact count | `d=1000`: no bound-independent sparse advantage |
| `x(a\|aa){1,100}y(b\|bb){1,100}z` | Two sequential flat scopes, each with `d=2` | Preserve scope and control-position identity |
| `x((a\|aa){1,10}b){1,10}z` | Nested control case | Outside this flat-count theorem |

For a small concrete case, `\Ax(a|aa|b){2,3}z\z` accepts `xaaz`
(`a` then `a`) and rejects `xaz`. After `aa`, viable paths can have completed
one or two iterations. One scalar does not preserve the exact reachable-count
set; any reduction needs a continuation-equivalence proof. The third alternative
makes the larger family more than a unary length example.

## 2. Rotating BVA columns: compete with Tier 2

OOPSLA 2023 supports counting-free fixed-length bodies, including alternatives,
with `O(p)` work per symbol independent of the repetition bounds. Repeated
activation from surrounding context is supported. Compatible states share
circular arrays by body-position column and carry prefix views; this avoids
large general vector merges. Storage is roughly `k*h` bits for body length `k`
and upper bound `h`, plus metadata.

Source: [BVA, Theorems 4.1/4.3 and Figures 7–8](https://ohyoukillkenny.github.io/source/BVA.pdf),
[committed full text](../../research_notes/regex-matching-research-2020-2026/text/2023-oopsla-leglaunec-bit-vector-automata.md).

Prototype class sequences, then fixed-length alternatives such as `(ab|ba){l,h}`.
Compare to Tier 2 even where Tier 2 is eligible. Dense history may favor bits
over `u32` deltas, while sparse seeds may favor deltas. Also test patterns whose
compound exits fail the current overlap proof.

Compile length columns and compatible views under a bounded analysis. Persist
rotation, validity lengths, and acceptance witnesses across chunks. Do not clear
the whole history on each activation. Arbitrary dense OR is not constant time;
reject the fast representation if compatible views cannot be proved. A fast
local loop still needs a correct enclosing execution model.

Memory can rule this backend out: a length-one history of one million bits is
about 125,000 bytes. Five hundred such histories total about 62.5 MB per stream
before metadata. This is arithmetic, not a measured allocation result, and
exceeds today's default repetition cap of 1000. Small
domains may fit word bitsets; wide ranges may favor sparse sets; proved
dominance may allow one witness.

Some exact-count memory is intrinsic. For `\A[ab]*a[ab]{h}\z`, any two distinct
length-`h+1` histories can be distinguished by appending an appropriate number
of `b` bytes and ending the stream: their differing symbol becomes the symbol
`h` places before the end. A deterministic exact streaming recognizer therefore
needs at least `h+1` bits to distinguish those histories. This elementary
distinguishable-prefix argument concerns memory, not time per byte. Boolean
output does not eliminate the history requirement.

| Example pattern | BVA structure | What it tests |
|---|---|---|
| `a.{256}b` | One body position, repeatedly activated by `a` | Exact delay with dense starts; compare with Tier 2 |
| `x(ab){100,1000}z` | Two fixed-length positions | Rotation across iteration and chunk boundaries |
| `x(ab\|ba){100,1000}z` | Two columns with alternative paths | Compatible-column sharing beyond literal sequences |
| `x([a-c][0-9]){100,1000}z` | Two byte predicates | Predicate masks rather than literal-only specialization |
| `a.{8,64}b.{8,64}c` | Two flat loops connected by ordinary control | Independent exits and downstream activation |
| `x(a\|aa){100,1000}z` | Variable iteration length | Outside the fixed-length fast theorem |
| `x((ab){2,4}){3,5}z` | Nested counted body | Not covered by the counting-free-body theorem as written |

For `\Ax(ab|ba){2}z\z`, `xabbaz` matches and `xabaz` does not.
Splitting the valid input as `xab` / `ba` / `z` must preserve both the body
position and completed-count history across chunks.

## 3. Synchronizing counting: expose iteration events

FoSSaCS 2023 gives fast non-replicating counting-set simulation for flat
synchronizing expressions. Fixed length is sufficient, but not necessary;
exactly one marker letter per iteration also suffices. `ac*` and `[^;]*;`
qualify, while `(a|aa)` does not. General determinization has a potentially
double-exponential structural cost; bound-independent simulation alone is not
a safe compiler design for untrusted patterns.

Source: [Synchronizing Counting, Sections 6–7](https://arxiv.org/abs/2301.12851),
[committed full text](../../research_notes/regex-matching-research-2020-2026/text/2023-fossacs-holik-synchronizing-counting.md).

A narrower experiment should certify a unique marker class per body iteration,
then derive a shared iteration clock using the paper's register sharing and
delayed increments. Arithmetic could occur at iteration events while a scanner
handles payload bytes. This clock is a proposed adaptation requiring proof:
counting all delimiters globally does not establish which runs share a clock.

Keep body recognition across chunks. Skip bytes only when they cannot produce
other control effects. Compare long payloads, empty payloads, and every-byte
markers; dense events must still have bounded work. Event sparsity is a typical
throughput opportunity, not a worst-case guarantee.

| Example pattern | Certificate to investigate | Boundary of the claim |
|---|---|---|
| `x([^;]*;){10,1000}z` | One `;` per iteration | Body validation and overlapping starts still matter |
| `x(ac*){10,1000}z` | One `a` per iteration, variable trailing payload | The marker is not necessarily the iteration's last byte |
| `x([0-9]+\.){4,100}z` | One dot per iteration with nonempty numeric payload | Counting dots alone admits invalid payloads |
| `x(ab\|ba){10,1000}z` | Uniform body length | Overlaps the BVA class; compare both |
| `x(a\|aa){10,1000}z` | No synchronization certificate of this kind | Parses can advance counts differently |

For `\Ax([^;]*;){2}z\z`, `xfoo;bar;z` and `x;;z` match; `xfoo;z`
does not. A chunk ending after `;` must preserve completion and downstream
activation. The body is nonnullable even though its payload can be empty.

## 4. Selection and shared event processing

| Certified property | Candidate | Main competing cost |
|---|---|---|
| Small count domain | Word bitset | Domain width |
| Fixed-length compatible columns | Rotating BVA | History memory |
| Flat, small `d(l,h)` | Sparse set | Transition fanout and merges |
| Flat synchronization certificate | Shared registers/clock | Analysis and clock identity |
| Continuation-language dominance | Extremal witness | Narrow eligibility |
| General nested counts | Correlated contexts | Joint configuration domain |

Hyperscan supplies production prior art for choosing repeat representations by
semantics, activation density, and storage cost. Section 7 details that machinery
and its scope; rings and latest-offset specializations are not new by themselves.

Investigate deriving a minimum activation distance from anchor overlap, then
bounding event history by that distance. Literal length alone is insufficient:
`aaa` can activate every byte. Separately, share scanners and immutable programs
across patterns, with stream-local activation state. Dense events still require
work for all interested regions; sharing a scanner does not eliminate fanout.

Report a plan's worst-case stream bytes, update work, compilation budget, and
eligibility certificate. Use measurements to choose between eligible plans.
Fast average throughput cannot override a failed semantic/resource certificate.

| Approach | Example | Proof or control case |
|---|---|---|
| Small count set | `x(a\|aa\|b){5,20}z` | Exact word bitmap per position; count bits differ from NFA-state bits |
| Earliest activation dominates | `a.{100,}b` | Without an upper bound, an older live start becomes eligible no later and never expires |
| Latest activation dominates | `a.{0,100}b` | Without a positive lower bound, the newest start remains usable at least as long |
| Neither extremum suffices alone | `a.{20,100}b` | Newest can be too young; oldest can expire while a newer start qualifies |
| Sparse retriggering | `abcd.{100,1000}z` | Exact `abcd` occurrences cannot overlap: ends are at least four bytes apart |
| Overlap negative control | `aaaa.{100,1000}z` | Ends can be one byte apart; length is not the minimum period |
| Shared scanner | `login.{0,100}failed` and `login.{20,200}locked` | Share `login` detection; retain distinct gap obligations |

Extremal-witness examples assume the same unrestricted gap and continuation for
grouped starts. Class-restricted gaps can kill candidates on a bad byte; general
body counts need not order continuation languages. Prove dominance rather than
applying gap rules to arbitrary counted subexpressions.

## 5. Nested counting: preserve correlations

Marginals `{1,2}` and `{2,5}` do not represent the joint set
`{(outer=1,inner=5), (outer=2,inner=2)}`: their Cartesian product invents runs.
Neither sparse marginal sets nor independent BVA counters solves this problem.

Two restricted experiments are justified:

1. **Unique iteration boundaries.** Start with anchored unambiguous nested
   bodies, such as `\A(([^;#]*;){2,4}#){2,4}\z`. Prove that active runs agree
   on the inner iteration/scope before sharing an inner recognizer. Investigate
   unanchored activation and overlapping groups separately. Delimiters alone
   do not prove uniqueness. This is a proposed restricted optimization, not a
   general theorem from the reviewed papers.
2. **Shared joint contexts.** Represent valuations by a canonical trie or
   decision DAG, with shared suffixes and compiled bulk transformations.
   Measure nodes, edges visited, hashing, and retained bytes against Tier 4.
   Sharing may be ineffective or the structure exponential. A conservative
   domain/operation bound is needed for admission; never truncate the set or
   return a runtime budget failure. Without a practical bound, keep this as
   research rather than a production execution path.

Arithmetic normalization is another narrow opportunity. `(a{2}){2,3}` allows
lengths 4 and 6, not 5: replacing it with `a{4,6}` is invalid. Preserving bounded
unions of arithmetic progressions may help, but the number of pieces must be
bounded too. This does not generalize to arbitrary inner languages.

| Nested approach | Example | Evidence to seek / negative control |
|---|---|---|
| Unique inner boundaries | `\A(([^;#]*;){2,4}#){2,4}\z` | `;` ends a field, `#` a group; prove one current scope before scalarizing |
| Joint-context sharing | `x((a\|aa){2,4}b){2,4}z` | Measure shared outer contexts across different inner counts |
| Ambiguous nesting | `x((a\|aa){2,4}){2,4}z` | Both boundary levels can vary; independent marginal counts are invalid |
| Exact normalization | `\A(a{2}){3}\z` | Equivalent to six `a` bytes |
| Progression, not interval | `\A(a{2}){2,3}\z` | Four or six `a` bytes match; five do not |

The first pattern accepts `a;b;#c;d;#` and rejects `a;#c;d;#`: its first group
has only one field. The joint-context example is a candidate workload, not
evidence that a DAG necessarily saves space on it.

## 6. Improve the NFA directly

**Change the active-configuration representation before adding another
deterministic tier.** The sparse paper's Section 2.5 and Algorithm 1 simulate a
nondeterministic position counting automaton. No DFA subset construction is
required. Likewise, NBVA denotes a nondeterministic model; determinization and
counter representation are separate decisions.

### Current NFA costs

Today's lists contain individual `(StateIdx, CounterCtx)` pairs
(`rethoc-engine/src/lib.rs:3866`). `drain_addstack` at 4010 follows structural
and dynamic transitions per pair. Nonempty contexts are hashed and copied into
the visited collection at 4027; Split branches clone them at 4048. The consuming
loop at 4217 tests the byte predicate per thread, even for threads at the same
state with different counts. Context slots have pattern-wide width (`3312`),
which hashing also visits (`3359`).

Start-closure precomputation and some fused consuming branches already exist
(`3972`, `4257`). Counter exit already removes the completed counter (`4112`,
`4128`). The opportunities below extend beyond those existing optimizations.

### A. Aggregate counts at each control position

Replace `(q,7), (q,8), (q,10)` with `q -> {7,8,10}` for a flat counter scope.
Initially maintain an exact invariant: the set contains precisely the counts
reachable at `q` after the consumed prefix. Test the byte predicate once, then
transfer the set using increment, guard, and union operations. Add the published
sparse abstraction only under its hypotheses. This avoids repeated predicate,
hash, and copy work without caching DFA subsets. Bounded-size sparse sets can
also change asymptotic count handling rather than just interpreter constants.

Example: `x(a|aa|b){1,1000}z` motivates sparse sets. For
`x(a|aa|b){16,32}z`, exact word bitmaps are a simpler first implementation:
shift to increment, OR to union, and mask to test eligible exits. Increment at
iteration boundaries, not at every byte. Larger exact domains span multiple
words and generic unions then cost proportionally more. Nested scopes require
sets of joint tuples. Unbounded counts need justified saturation or appropriate
normalization, not forever-growing integer offsets.

### B. Compile structural traversal, retaining dynamic barriers

In `x(ab|cd|ef){2,100}z`, the alternatives' Split topology is unchanged across
counts. Precompute Split-only reachability or compact transfer instructions from
consuming states to the next assertion/counter operation. Execute dynamic guards
and updates explicitly rather than flattening them into unconditional edges.

`x(a?){2,100}z` is a nullable-body exclusion test: zero-consumption paths can
change counts. A compiled instruction DAG can share paths; enumerating every
epsilon path could itself explode. Bound analysis and emitted edges. Position
automata remove epsilon traversal but can have quadratically many edges, so
this trades runtime visits for potentially more compiled memory.

### C. Bit-parallel ordinary control with counting operations

`HEAD(ab|cd|ef)*TAIL` is a counter-free baseline. Represent active positions by
bits, compile common edges into masks/shifts, and intersect successors with the
current byte's predicate mask. For `HEAD(ab|cd|ef){10,100}TAIL`, attach explicit
counter-set operations to completion/entry edges. A single active bit per
position does not encode its different count valuations.

This gives bit-parallel NFA regions with counter-set interfaces, not necessarily
a whole-pattern DFA. Retain list-based control as a competitor for sparse
frontiers or unfavorable graph layouts. Measure transition-mask count and
exceptional-edge frequency. Hyperscan supplies concrete prior art below.

### D. Narrow context layouts and shared outer contexts

In `(ab){2,100}x(cd){2,100}y(ef){2,100}z`, a run has only one active counter
scope at a time, yet full-width storage reserves slots for all three. Scope-local
layouts with explicit identity/remapping could reduce copied/hashed bytes. This
is narrower storage, not newly removing counts that exit already removes.

For `((a|aa){2,10}b){2,10}c`, investigate immutable outer-context sharing across
inner branches. Canonical IDs make equality cheap only if construction,
interning, reclamation, and transformation costs remain bounded. Sharing can
reduce copies without reducing the number of reachable joint valuations.

### Implementation order

Keep today's Tier 0 as an independent oracle initially. Add an experimental
grouped flat NFA with exact small bitsets, then sparse sets. Compile structural
transfers next. Evaluate bit-parallel control and narrow nested contexts as
independent changes so their effects remain measurable.

Compare identical compilation options/workloads across grouped NFA, Tier 0,
Tiers 2–4, and default unrolling. Count predicate evaluations, epsilon visits,
context-copy/hash bytes, set-merge work, and allocations alongside timing and
memory. Improving this baseline is valuable even if it does not win every case.

## 7. What Hyperscan does

Implementation references are pinned to official Intel Hyperscan revision
`017268f8455fd6095b7045fd85f2efd26a3c5604`. This is a source inspection, not a
Hyperscan benchmark or a compiler-dispatch trace. Example patterns illustrate
the problems addressed by each component; they do not assert which component
that exact pattern would select after optimization.

### Decompose patterns, then choose small execution components

The NSDI 2019 paper describes Glushkov position NFAs and decomposition into
literal and automaton components. Literal matches participate in execution,
rather than merely filtering before a second complete regex scan. For example,
`(abc|def).*ghi` exposes literal events and a relation between them. Smaller
residual components may use DFA or NFA execution. This is a multi-pattern
architecture, not one universal automaton algorithm.
[Paper](https://www.usenix.org/conference/nsdi19/presentation/wang-xiang),
[committed text, Sections 3 and 4](../../research_notes/regex-matching-research-2020-2026/text/2019-nsdi-wang-hyperscan-fast-multipattern-regex.md).

For rethoc, `login.{0,100}failed` and `login.{20,200}locked` illustrate the
opportunity: scan the common literal once and dispatch events to separate
continuation obligations. Any streaming adaptation must retain enough history
and state to connect components across chunks.

### LimEx: bit-parallel NFA with exceptional operations

LimEx uses an active-state bit vector. Its inner loop combines masked shifts for
common transitions, handles exceptional transitions, and applies the current
byte's reach mask. It does not enumerate every ordinary active state through a
general interpreter. Unusual edges still cost exceptional work; arbitrary graphs
do not become a constant number of shifts.
[Runtime loop](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/limex_runtime_impl.h#L203),
[state layout](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/limex_internal.h#L27).

Bounded-repeat operations can remain inside that NFA. `POS` records a repeat
activation offset; `TUG` tests whether an activation can satisfy the repeat at
the current position. Stale repeat state can deactivate its cyclic control
state. These are explicit repeat operations alongside fast ordinary transitions,
not determinization of every possible count history.
[Repeat operations](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/limex_ring.h#L44),
[exception integration](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/limex_exceptional.h#L122).

### Specialized repeat histories

| Model | Illustrative pattern | Stored information / reason |
|---|---|---|
| `FIRST` | `a[ab]{100,}c` | Earliest surviving activation for an unbounded upper limit |
| `LAST` | `a[ab]{0,100}c` | Latest activation for zero lower bound |
| `BITMAP` | `a[ab]{10,40}c` | Small exact activation history in a machine word |
| `TRAILER` | `a[ab]{8,1000}c` | Recent match-window offset plus a small bitmap for earlier matches |
| `RANGE` | `a[ab]{100,1000}c` | Small ordered array of offsets for wide windows |
| `RING` | `a[ab]{1000}c` | History for dense starts and a narrow/exact window |
| `SPARSE_OPTIMAL_P` | `abcdefgh[abcdefghx]{1000}z` | Compressed history using a proved minimum retrigger period |

`chooseRepeatType` handles infinite bounds, zero minima/resetting triggers, and
small domains first, then compares suitable representations' stream sizes. In
this revision sparse consideration requires a minimum period above six, and
the range representation permits at most sixteen slots. These are engineering
choices, not mathematical eligibility constants to copy unquestioningly.
[Model definitions](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/repeat_internal.h#L41),
[selection and sizing](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/repeatcompile.cpp#L245),
[trailer operations](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/repeat.c#L637).

The crucial boundary: the inspected repeat recognizer groups connected vertices
with the **same character reachability**. This specializes homogeneous byte-class
runs such as `[ab]{m,n}`, including gaps. It does not supply general iteration
counting for `(ab|c){m,n}`, `(a|aa){m,n}`, or arbitrary nested bodies. The latter
can sometimes simplify or be compiled through other machinery, but no general
fast-counting guarantee follows from these repeat models. Also, Hyperscan's
`RANGE` is not evidence that rethoc's Tier 3 ranges implement the same abstraction.
[Repeat recognizer](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfagraph/ng_repeat.cpp#L76).

### LBR, Castle, and the memory split

LBR provides specialized scanning for large bounded repeats, including wildcard
and character-class escape scans. Castle groups repeat engines sharing character
reachability and can share storage for proved mutually exclusive subengines.
For example, the repeat portions of `foo[ab]{100,200}x` and
`bar[ab]{300,400}y` suggest shared `[ab]` scanning, while their bounds and
activations remain distinct. This example is not a verified Castle dispatch.
[LBR layout](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/lbr_internal.h#L28),
[Castle layout and exclusivity](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/src/nfa/castle_internal.h#L66).

Hyperscan separates the compiled database, persistent stream state sized at
compile time, and preallocated scratch reused by sequential scans on a worker.
Concurrent or recursive scans need separate scratch. Stream compression is an
additional facility for inactive streams, not free compression on every chunk.
This separation is directly relevant to hundreds of rules across many streams.
[Streaming and scratch documentation](https://github.com/intel/hyperscan/blob/017268f8455fd6095b7045fd85f2efd26a3c5604/doc/dev-reference/runtime.rst#L60).

### What to borrow, and what remains research

Borrow the architecture: compile ordinary control efficiently, attach explicit
repeat kernels, choose storage by proven semantics and cost, share scanning,
and separate scratch from persistent state. Extend beyond homogeneous byte runs
using the sparse-counting and fixed-length BVA results, rather than assuming
Hyperscan's repeat offsets count arbitrary body iterations. The survey's blanket
inference that production engines only unroll counting is too strong; the
opposite claim that Hyperscan solves general nested counting is also unsupported.

## 8. Streaming and resource contracts

Separate immutable plan, stream state, and reusable scratch. A shared lazy cache
needs synchronization, stable identities, and eviction semantics; merely moving
today's mutable cache onto `Regex` is insufficient. Stream suspension must not
invalidate counter or control state.

Specify byte consumption and timed entry, completion, exit, and assertion
events. Persist next-byte assertions across chunks; evaluate end assertions at
`finish()`. Initially choose another exact backend for unsupported assertions.
No whole-input reverse pass or unbounded input buffering is available.

Unbounded repeats cannot use offsets that increase forever in machine integers.
Saturate when justified, or rebase relative coordinates while preserving all
differences. Include rebase/wrap handling in proofs and tests. Report reset and
startup cost separately, including hidden history clearing.

Budget states, edges, analysis steps, numeric sizes, and planned stream bytes
at compilation. If a certificate fails, select another exact backend. If no
backend meets a future advertised resource contract, reject at compile time.
This is a proposed policy, not a claim that today's NFA has small practical
memory requirements for every accepted pattern.

## 9. Experiments and decision gates

| Axis | Required cases |
|---|---|
| Bounds | exact, width 1, half-width, `{0,h}`, `{l,}`; geometric scaling |
| Body | class sequence, equal-length alternatives, `a\|aa`, marked, nullable, nested |
| Activation | anchored, consuming prefix, sparse restart, every-byte restart |
| Outcome | early success, late success, dense near miss, scan-only rejection |
| Chunks | whole, bytewise, 64/1500-byte, irregular, empty, delimiter splits |
| Lifetime | cold compile/cache, warmed reuse, fresh short streams, suspended streams |
| Deployment | hundreds of rules, thousands of interleaved streams, active/idle mixtures, concurrency scaling |
| Baselines | default, forced counters, every eligible tier, Rust regex, Hyperscan/Vectorscan where semantics and platform support align |

Record compile time, full plan bytes, allocated stream bytes and peak scratch,
population cost, steady ns/byte, startup ns/stream, and operation counts. Use
process caps for experiments; timeouts are censored observations, not answers.
Fix seeds and record machine/compiler/build identity. A 500-pattern workload
needs actual combined state/throughput measurements, not multiplication of an
isolated peak GB/s number.

### Production acceptance criteria

The supplied probe exercises individual patterns; its results cannot establish
production readiness or best-in-class performance. Extend the evaluation with realistic
rule sets and traffic, including deliberately dense activations and adversarial
near misses. Compare external engines on a documented common semantic subset:
byte/newline behavior, streaming versus block mode, boolean versus reporting
work, and compilation limits must be explicit. Report rejected or unsupported
patterns separately rather than silently excluding them from aggregate results.

Measure these together:

- Sustained throughput and CPU cost for the full rule set, including shared
  scanning, scheduling, and dense event fanout.
- Per-chunk and per-stream latency distributions, including p99/p99.9 where
  sample counts support them, and maximum observed adversarial processing time.
  Observed maxima do not replace a bound on algorithmic work.
- Persistent bytes per stream, total memory at target concurrency, worker
  scratch, compiled database/cache memory, and allocation behavior during scans.
- Cold compilation and stream startup, reset/reuse, and cache churn when many
  patterns and streams are interleaved.
- Correctness under arbitrary chunking, resource-bounded compilation, and exact
  execution without match-time truncation or budget failures.

Numerical throughput, latency, and memory budgets must be set against the target
hardware and deployment envelope before production selection; this investigation
has not established those budgets. Record tradeoffs and regressions against
existing paths and external competitors. Claims of being best in class require
that evidence for the stated workloads and limits.

Decision gates:

- Sparse: fixed-`d` work/state should stay stable as `h` grows, including
  prefix-gated entry. Exact-count negative controls should expose the limit.
- BVA: update work should be independent of `h`, while history storage follows
  the predicted growth. Compare dense and sparse activation against Tier 2.
- Synchronization: require a cheap certificate, event-density scaling, and
  bounded dense-event work.
- Nested sharing: require reductions in visited nodes and retained bytes on
  adversarial cases, not just small illustrative DAGs.

Only integrate a prototype after it wins on a meaningful family without hiding
startup or memory costs. Evidence does not yet support a universal numeric
speedup threshold. Exhaust small patterns/inputs against NFA and `regex`, then
fuzz whole/chunked execution, reset/reuse, empty chunks, and cache eviction.
Those are requirements for future backends, not validation claimed by this
research. No new execution backend has been implemented here.
