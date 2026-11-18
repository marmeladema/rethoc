# Complete Tier 1 assertion support

Status: implementation plan, not implemented. Source baseline:
`97adc64e7e3f6ce720541498a79e4b8d476e5cc9` (2026-11-18).
Verification evidence and its limits are recorded below and in
[the plan audit](verification/tier1-assertions.md).

## 1. Scope and acceptance contract

Make every assertion kind currently accepted by the compiler work in Tier 1
when the **compiled graph has no counters**. In particular, admit `StartCRLF`
and `EndCRLF` (`(?Rm:^)`, `(?Rm:$)`). Fix the existing Tier 1 boundary-context
and EOF-chain defects as prerequisites. Do not add assertion-chain eligibility
restrictions to hide them.

This does not add Unicode boundaries, lookaround syntax, counter support in
Tier 1, or CRLF support in Tiers 2–4. Unrolling can make a counted source pattern
counter-free; disabling unrolling can still require the NFA for that pattern.

Correctness is mandatory across arbitrary chunk boundaries, empty chunks,
empty streams, cache eviction, and cache reuse. Matching must remain linear
in input length with pattern-bounded work, no match-time failure budget, and
no input buffering. Newly admitted CRLF patterns and repaired, previously
incorrect combinations may cost more. Existing-correct use cases must pass
the performance criteria in Section 7 individually, not only in an aggregate.

## 2. Verified starting point

Source links name files and symbols rather than unstable line numbers.

| Fact | Source | Consequence |
|---|---|---|
| Tier 1 checks `!has_counters && !has_crlf_assert` | [lib.rs](../rethoc-engine/src/lib.rs), `RegexBuilder::build` | Only two of the ten accepted assertion kinds are excluded specifically by Tier 1 |
| CRLF anchors inspect adjacent bytes and stream boundaries only | `AssertKind::eval` | Finite boundary context suffices; no engine redesign or unbounded lookahead |
| Pending assertion state retains only `prev_was_word` | [dfa/mod.rs](../rethoc-engine/src/dfa/mod.rs), `DfaState`, `prev_stand_in` | CR, LF, and input-start information can be lost |
| Deferred mini-closure uses `at_start=false` | [dfa/tier1.rs](../rethoc-engine/src/dfa/tier1.rs), `populate` | An assertion after a parked assertion can receive the wrong boundary |
| `$` sets `is_match_at_end` using `state_can_reach_match` | `DfaMemory::epsilon_closure` | Optimistic graph reachability bypasses downstream assertions |
| That reachability intentionally walks through every assertion | `RegexBuilder::compute_can_reach_match` | Keep it for its other uses; do not treat it as an EOF proof |
| Byte classes include assertion properties, already including CR/LF | `AssertKind::byte_props`, `compute_byte_classes` | Preserve this derivation; no hand-maintained CRLF byte-class exception |
| DFA state, borrowed key, interning, and closure are shared | `dfa/mod.rs`; `tier4.rs` also constructs state keys | Separate Tier 1 semantics from mechanical shared representation changes |
| `is_dfa_deferrable` is used by Tier 3 effect analysis | [tier3/effects.rs](../rethoc-engine/src/dfa/tier3/effects.rs) | Do not globally broaden this predicate to enable Tier 1 |
| Prefilter construction stops at assertions before consumption | [prefilter.rs](../rethoc-engine/src/prefilter.rs), `compute_prefilter` | Keep the current skipping policy; do not add an assertion-aware prefilter here |

CLI verification: `(?Rm)^foo$` selects the NFA and refuses forced Tier 1;
`(?m)^foo$` selects Tier 1. All 14 existing `test_crlf` tests passed.
These tests exercise current admitted engines, not CRLF execution in Tier 1.

Verified prerequisite failures (oracle and NFA agree; Tier 1 fails both whole
and bytewise feeding under all three compile configurations):

| Pattern | Input | Expected | Current Tier 1 | Defect |
|---|---|---|---|---|
| `\b(?m:^)a` | `a` | true | false | Lost start context during next-byte resolution |
| `\B(?m:^)` | empty | true | false | Lost start context during EOF resolution |
| `a$\B` | `a` | false | true | Optimistic EOF shortcut |
| `$\b` | empty | false | true | Optimistic EOF shortcut |
| `$^` | `a` | false | true | Optimistic EOF shortcut |

Minimal repro examples:

```sh
cargo run --quiet -- brute --alphabet 'a\n\r' --max-len 1 '\b(?m:^)a'
cargo run --quiet -- brute --alphabet 'a' --max-len 0 '\B(?m:^)'
cargo run --quiet -- brute --alphabet 'a' --max-len 1 'a$\B'
```

## 3. Compile exact, bounded assertion metadata

Proposed names below are new interfaces, not existing APIs. Add a Tier 1
analysis module, for example `dfa/tier1/analysis.rs`, analyzing the final graph
after byte-table optimization. Build immutable per-state metadata only when
Tier 1 is eligible and assertions need it. Do not run this analysis on counter
graphs. Store `Box<Tier1Assertions>` on `Regex`, with an empty entries slice
when no analysis is needed. The outer owner is always allocated, keeping this
field one pointer wide and avoiding optional access during matching; empty
entries allocate no payload. Account for the boxed owner unconditionally in
`Regex::memory_size`. A nonempty table covers every state in the compiled graph.

### One definition of context dependencies

Extend the assertion dependency declaration to distinguish previous-byte
properties, next-byte properties, and input-start dependence. Derive existing
`byte_props()` as the union of the two byte sides. Keep one authoritative
declaration, checked exhaustively against actual `eval`, rather than a separate
CRLF list in the DFA implementation.

Previous-byte needs are WORD for word boundaries, LF for `StartLF`, CR|LF for
`StartCRLF`, CR for `EndCRLF`, and none for `Start`, `End`, or `EndLF`.
`Start`, `StartLF`, and `StartCRLF` read input-start. `EndLF` reads the next
byte, **not the previous byte**: retaining LF context for it would split
existing states unnecessarily. EOF is supplied explicitly by the closure mode.

For every potentially parked assertion, compute the union of dependencies
along its entire zero-width continuation, including the assertion itself.
Follow Split and Assert edges, stop at consuming states and Match. For this
**known-next-byte** analysis, stop at End: it fails when a next byte exists.
This must cover assertions behind other assertions and alternative paths,
not just the first parked kind. For example, `\b(?m:^)` needs LF and start
context as well as wordness.

Use reverse edges and a finite monotone worklist (or SCC propagation), not a
fresh graph traversal from each assertion. Four dependency bits can each
propagate only a bounded number of times per edge. Cycles are legal; neither
analysis nor matching may recurse indefinitely. Construction is O(V+E) with
a fixed small factor and flat storage.

### Exact EOF acceptance

There are five physical contexts before EOF: input start (no previous byte),
previous WORD, previous CR, previous LF, and previous OTHER. Use representatives
`None`, `a`, `\r`, `\n`, and space. The existing `ByteProps::stand_in` and its
exhaustive tests establish that byte categories suffice when start/end flags
are preserved.

Precompute `eof_accept[state]`, a five-bit mask: bit C is set exactly when Match
is reachable from that state without consuming a byte at EOF in context C.
For each context:

1. Seed Match as accepting.
2. Propagate backward through Split edges and Assert edges whose **actual
   `eval(at_start, true, prev, None)` returns Pass**.
3. Stop at every consuming state. Counters are absent by construction.

A reverse worklist computes the least fixed point, including zero-width
cycles. A cycle with no path to Match must not become accepting. Complexity
is O(5(V+E)); storage is one byte per state plus bounded temporary graph data.
Do not reuse optimistic `state_can_reach_match` as this mask.

Budget these arrays against the bounded compiled graph; use checked size
arithmetic, account for allocations in `Regex::memory_size`, and measure large
patterns near the configured compile limit. The proposed per-state dependency
mask and EOF mask total two payload bytes per state, excluding owners,
alignment and construction scratch. No independent per-state DFS or new
unbounded compile-time exploration is permitted.

## 4. Exact Tier 1 transition algorithm

A cached state represents a boundary, with consuming states, parked assertions,
a match flag, an **exact EOF acceptance flag for that boundary**, and only the
context needed to resolve its parked assertions when a byte arrives.

Use two explicit closure modes. Do not share a boolean-heavy call site that
can accidentally apply provisional EOF logic to the previous boundary.

### Provisional closure: next byte unknown

Given raw roots and the actual boundary context:

For the mandatory assertion-free `analysis.is_empty()` path, ordinary closure Match
is also the exact EOF answer: no epsilon edge depends on EOF. Use that result
without table lookups or context-mask aggregation. The steps below describe
the assertion-aware path.

1. Compute EOF acceptance by OR-ing `eof_accept[root]` for those roots at this
   context, **before discarding any context**. This includes paths blocked by
   End, CRLF, word assertions, and arbitrary subsequent assertion chains.
2. Walk the ordinary epsilon closure with `at_end=false`, `next=None`.
   Follow Pass, park Defer, drop Fail; End stops here. Keep consuming states
   and unconditional Match. Use visited state indices to terminate cycles.
3. OR the precomputed dependency masks of the parked assertions. Canonicalize
   the actual previous-byte/start context by this union before interning.

This closure is used for initialization and after consumption/reseeding.
The EOF flag already includes parked assertions, so Tier 1 `finish()` needs
no second graph walk and no new scratch allocation.

### Known-next-byte closure and transition

For byte b entering a cached state:

1. Resolve all parked assertions with that state's reconstructed previous
   context, preserved input-start flag, `at_end=false`, and `next=Some(b)`.
   Follow their complete epsilon continuations at **the same boundary**.
   End fails. Every assertion resolves Pass or Fail, never Defer.
2. Collect any match found before b and all newly reachable consuming states.
   Both those consumers and the state's original consumers consume b exactly
   once. Preserve fused `out_exit` and ByteTable semantics.
3. Run provisional closure on consumed successors plus `regex.start`, now
   with `prev=Some(b)`, `at_start=false`, and unknown next byte. Combine the
   pre-b unconditional match with the successor match flag.
4. Cache the successor. Never carry the old EOF flag or an EOF hypothesis
   from the known-next closure to the new boundary.

At initialization use roots `[regex.start]`, `prev=None`, `at_start=true`.
`finish()` returns an already observed match or the current exact EOF flag;
EOF is not a consuming sentinel. Empty `chunk()` calls do nothing. A state is
DEAD only if it has no consumers, pending assertions, match, or EOF acceptance;
retain the current restart behavior from DEAD. Do not infer input-start from
`current == start_id`: an ordinary start state may recur later in the stream.

Correctness obligations: provisional Pass is stable under revelation of a
byte or EOF; every parked continuation receives equivalent context; raw-root
EOF masks equal complete closure; transitions consume exactly one byte;
reseeding occurs at every boundary; state-key equality implies equivalent
transitions and EOF answers. Tests/model checks must target each obligation.

## 5. Representation, isolation, and fast paths

Use a compact `u8` context newtype in place of the shared wordness boolean,
encoding WORD/LF/CR/start. Preserve 0/1 for legacy nonword/word contexts.
Canonicalization clears irrelevant bits; no parked assertions means zero
context. Reconstruct `None` only when retained start context says start;
otherwise use the representative byte. A masked-away start is equivalent
only because all relevant continuations have been proved not to read it.

Keep a legacy boolean interning wrapper and existing assertion-resolution
semantics for Tiers 2–4. Tier 1 uses a new context-aware interning entry point
and its own closure policy. Migrate direct state/key constructors mechanically,
including Tier 4, and keep owned/borrowed hash and equality definitions aligned.
Update eviction preservation and all diagnostic formatting. Verify layout and
hash behavior; replacing a bool with a newtype is not itself evidence of zero
cost. Do not change the other tiers' CRLF gates or known-defect policy.

Do not broaden `is_dfa_deferrable` globally. Give Tier 1 an explicit policy
based on all currently supported `can_defer()` kinds; retain the counter
tiers' policy and make CLI information report the selected backend accurately.

Keep `transition`, `transition_direct`, and the cached scanning loops free of
new per-byte assertion checks, context calculations, or table indirections.
Perform new work at build, prepare, or transition miss time. Preserve the
existing prefilter policy. Reuse closure/target scratch buffers; do not add
allocation per byte or `mem::take` scratch capacities. Existing miss-path Vec
cloning is not permission to multiply it; any wider scratch refactor is a
separate measured change.

Exact analysis permits these fast paths without weaker semantics:

- Assertion-free patterns retain no analysis allocation and no context.
- Word-only pending continuations retain only wordness, as before.
- Ordinary EndLF-only pending continuations retain no previous-byte context.
- End-only acceptance remains a cached boolean, with no pending End list or
  graph replay at `finish()`.

For patterns with no EOF-sensitive assertions, an implementation may use the
ordinary match flag instead of an EOF table, but only with a compile-time proof
and differential tests. Start with the exact algorithm; do not introduce an
unchecked legacy fast path for complex chains.

## 6. Correctness test matrix and tooling

Add wrong-answer regressions through `match_tests!`, with the appropriate
postmortems. Do not set `unroll_limit: 0`; the macro already runs default,
no-unroll, and no-unroll/no-merge configurations, default/minimum DFA caches,
and forced byte classes. Update CRLF `min_tier` expectations deliberately;
memory blessing must never change them. Counter-bearing configurations still
exercise their existing eligible engines.

Required coverage:

| Area | Cases |
|---|---|
| CRLF boundaries | Bare CR, bare LF, CRLF, repeated/mixed terminators; never match between CR and LF; CR at EOF; each anchor alone and both |
| Assertion combinations | All 100 ordered pairs of the ten kinds; selected triples mixing word/LF/CRLF/start/End; assertion before/after a consumer; alternative and optional paths |
| EOF | Empty stream, End before/after another assertion, impossible chains, EOF-only success, deferred success before a byte versus at EOF |
| Streaming | Whole input, bytewise, every partition of short inputs, empty chunks before/between/after; explicitly split CR and LF |
| Context | Start versus ordinary nonword; word versus nonword; CR versus LF versus space; all 256 byte values in evaluator/context checks |
| Graph | Epsilon cycles, consuming loops, Split joins, Byte/ByteCI/classes/wildcard/ByteTable and fused exits |
| Cache | Fresh and warm; input A then B and reverse; eviction immediately before pending resolution; default cap and 8; different Regex objects sharing MatcherMemory |
| Dispatch | Forced Tier 1 accepts counter-free CRLF; automatic selects it; counter forms remain ineligible when not unrolled; other tier eligibility unchanged |

Retain macro tests for concrete bugs. Extend reusable test helpers for all
short chunk partitions and empty chunks; do not duplicate a handwritten
matcher regression per pattern. Add structural unit tests for mask fixed
points, context equivalence, hashes/borrowed keys, eviction, and absence of
unnecessary state splitting. Test both sides of the byte-class threshold.

Add a targeted `tier1-assertions` sweep in `rethoc-cli/src/check/sweep.rs`
(**new command family**), with counter-free graphs/patterns, all ten assertion
kinds, and alphabet `a \r\n\xFF`. Include pair placement, alternation, cycles,
and counted source patterns that unroll, with eligibility checks explicit.
Keep generated runs bounded and individually capped. The existing
`assert-edges` sweep has no CRLF assertions or CR in its alphabet; the stable
fuzz generator also lacks LF/CRLF anchors. Those existing tools alone cannot
validate this feature. Add dedicated deterministic generation rather than
silently changing existing fuzz seed mappings.

Once implemented, run:

```sh
cargo fmt --check
cargo test
cargo test --release
cargo clippy --all-targets -- -D clippy::all
cargo clippy --manifest-path fuzz/Cargo.toml --all-targets -- -D clippy::all
python3 scripts/bless_memory.py --check

# New family, after it is implemented; cargo uses the resource-cap runner.
cargo run --release -- sweep --tier 1 tier1-assertions
RETHOC_DFA_STATES=8 RETHOC_BYTE_CLASS_MIN_STATES=0 \
  cargo run --release -- sweep --tier 1 tier1-assertions
cargo run -- sweep --tier 1 --max-len 4 tier1-assertions
cargo run --release -- fuzz --batch 250 --allow-slow ci/known-slow-seeds.txt 0 9000
RETHOC_DFA_STATES=8 RETHOC_BYTE_CLASS_MIN_STATES=0 \
  cargo run --release -- fuzz --batch 250 --allow-slow ci/known-slow-seeds.txt 0 9000
```

Also run existing sweep families in their CI tier configurations, default,
extreme, and short debug regimes. Known Tier 3 failures remain separately
reported; do not mask new failures in the other engines. Run ignored fuzz
proptests and targeted CRLF differential generation. Regenerate memory values
when analysis storage changes, reviewing the cause of each change.

## 7. Performance evidence required before merge

The existing `reengage_profile` gate has only two Tier 1 controls: literal and
assertion-free no-prefilter. Its +5% instruction and +100% simulated conditional
branch-miss limits remain required but do not cover assertion behavior.

Before engine changes, add identical focused benchmark sources to baseline
and candidate builds. Use a separate baseline worktree and the named-baseline
workflow already in `.github/workflows/differential.yml`; do not switch the
active worktree between revisions. Record the exact baseline/candidate hashes,
toolchain, runner version, CPU, corpus, flags, and output artifacts.

Measure each existing-correct family: literal with sparse/dense prefilter
candidates; assertion-free no-prefilter; `\bfoo\b`, `\Bfoo\B`, directional
word boundaries; `(?m)^foo$`, `foo(?m:$)`; `foo$`, `^foo$`; and oracle-checked
mixed assertion chains. Include full-scan nonmatches and late/EOF matches so
early success cannot hide scanning cost.

Run fresh and warm caches, one-byte and 16 KiB chunks, direct byte tables and
forced classes, default and minimum cache caps. Include long scans, repeated
short streams, and working sets that actually evict. Separately measure build
time and retained/peak memory, preparation cost, scan instruction counts,
branch misses, Criterion throughput, state counts and evictions. Use identical
haystacks on both revisions. Instrumentation for cache metrics must stay out
of the production cached transition loop.

Memory comparisons include fixed `Regex` size (including any optional analysis
owner), `MatcherMemory` and `DfaState` layout, not just table payloads. No table
allocation does not by itself mean unchanged retained memory. Because shared
key plumbing changes, also measure preparation and cold-cache costs for
existing-correct workloads in Tiers 2–4, beyond the existing long-scan gate.

Acceptance criteria:

1. Existing `reengage_profile` limits pass unchanged, including other tiers.
2. Apply the same instruction/branch limits per existing-correct case in the
   new focused Callgrind gate. Investigate smaller consistent regressions too;
   a threshold is not a license to spend it everywhere.
3. No reproducible throughput regression outside measurement uncertainty is
   accepted without an explicit recorded tradeoff. Re-run noisy comparisons;
   do not pool them with faster cases. Record compilation and short-stream
   regressions as well as steady-state results.
4. Assertion-free state sizes, allocations and state diversity must not grow
   without justification. Word-only cases retain equivalent context diversity;
   EndLF alone must not split by previous CR/LF. Exact EOF booleans may merge
   or distinguish states differently; explain observed differences.
5. Report new CRLF and repaired previously-wrong cases separately, against
   Tier 0 and the reference where useful. They need correctness and bounded
   resource use, not parity with simpler pre-existing cases. Never compare
   speed against a baseline that computes the wrong answer.

No speedup, zero-overhead claim, total state multiplier, or exact struct-size
claim is established by this plan. Additional context has finitely many
categories, but total cache behavior is pattern-dependent and must be measured.

## 8. Implementation sequence and review gates

1. **Baseline and tooling.** Land focused benchmark fixtures and the targeted
   assertion sweep separately from engine fixes. Capture existing-correct
   performance and the small wrong-answer repros. A sweep exposing existing
   defects is diagnostic until the fixes land; do not install a failing
   blocking job without documenting that status.
2. **Boundary-context fix.** Add dependency metadata and context-aware Tier 1
   resolution, preserving other tiers. Fix the start/previous-byte chain
   defect with `match_tests!` regressions and a numbered bug postmortem.
   Keep CRLF eligibility closed. Run full tests, clippy and performance checks.
3. **Exact EOF fix.** Add EOF masks and provisional/known-next closure modes;
   make Tier 1 finish use exact acceptance. Fix the optimistic `$` defect with
   its own regressions and postmortem. Keep CRLF eligibility closed. Run the
   complete correctness and existing-performance gates again.
4. **CRLF admission.** Remove only Tier 1's CRLF restriction after both fixes.
   Add CRLF regressions, review tier expectations, update diagnostics/docs and
   activate the new sweep in CI. Run the full matrix and report new-case cost.
5. **Final review.** Check all five engine invariants, metadata memory accounting,
   diagnostics (`Display`/`Debug`, `dump.rs`, CLI info), and public docs. Ensure
   no public docs expose newly introduced implementation mechanics. Verify
   the original counter-tier exclusions and tests remain intact.

Fixes, their tests and their postmortems stay together; tooling is separate.
Before each tier/compiler commit, run sweeps and a batch fuzz range, and ask
before committing as required by AGENTS.md. This plan authorizes no commits.
