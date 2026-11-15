# rethoc — engine goal and constraints

Brief for agents using the literature in `regex-matching-research-2020-2026/` to propose improvements to the prototype engine. Read this first; judge every idea against it.

## What the engine is

rethoc is a streaming regex engine for **untrusted patterns over untrusted input**. It answers *does this stream match?* — not *where*. Input arrives in arbitrary chunks; all matcher state must survive chunk boundaries.

## Target deployment

- **Security rules on live network traffic** (IDS/IPS-style: Snort/Suricata-like rule sets).
- **Many concurrent streams**, all matched against the **same compiled regexes**.
- **500+ regexes per stream.**
- Consequences:
  - **Per-stream state size is a first-class cost.** Compiled structures (NFA, DFA caches, analyses) should be shared across streams; what each stream carries must be small and bounded. Multiply any per-stream byte by (streams × patterns).
  - **Throughput matters** alongside worst-case guarantees: the engine sits on the traffic path.
  - **Multi-pattern matching is likely on the roadmap** (matching the whole rule set in one pass instead of 500 independent matchers). Designs that compose to multi-pattern are preferred over ones that don't.

## Hard guarantees (non-negotiable)

1. **Linear-time matching.** Per-byte work bounded by the compiled pattern, never by input content. No backtracking.
2. **Bounded, panic-free compilation** of attacker-written patterns (time and memory); every new feature needs a resource limit.
3. **Semantic agreement** across all execution paths with the reference semantics (Thompson NFA). Each tier must be correct for exactly the class it accepts; eligibility analysis must be precise (reject rather than mis-accept).
4. **Streaming:** no assumption that the whole input is available; no unbounded buffering of input.

## Priorities (current ranking)

1. Correctness and robustness.
2. Worst-case performance (adversarial input and adversarial patterns).
3. Typical throughput — also very important, not an afterthought.
4. Memory: per-stream state, then shared compiled memory.

## The core hard problem

**Counted repetition `r{m,n}`.** It causes exponential DFA blowup and is the known weak point of non-backtracking engines (including counter-targeted ReDoS against them). The engine attacks it with compile-time-selected tiers:

| Strategy | Accepts |
|---|---|
| Tier 0 — Thompson NFA | everything (universal fallback) |
| Tier 1 — lazy DFA | counter-free patterns (with deferred `\b`, `\B`, end assertions) |
| Tier 2 — differential counters | non-nested, fixed-length bodies, O(1)/byte |
| Tier 3 — conditional transitions | non-nested, variable-length bodies (currently being redesigned; see `docs/tier3-*`) |
| Tier 4 — counter programs | nested repetition |
| Specialisations | common shapes, e.g. bounded gaps `A.{0,N}B` (see `docs/bounded-gap-*`) |

Supporting techniques: SIMD byte-range prefilters, byte-class compression, lazy DFA caching, memory reuse with no allocation on the hot path.

## Scope

- **In:** byte-oriented matching; anchors and word boundaries; counted repetition in all its forms; prefilters; multi-pattern (future); possibly **limited lookahead** (must stay streaming-compatible, i.e. bounded delay, no right-to-left pass over the whole input).
- **Out (for now):** match offsets and capture groups (unlikely), backreferences, general lookaround/lookbehind, full Unicode semantics (most likely not), intersection/complement syntax.

## How to read the literature through this lens

- **High value:** counter representations (counting-set automata, bit-vector automata, synchronizing counting, single-counter vs bit-vector classification from the in-memory hardware papers); multi-pattern engines (Hyperscan, Teddy, HybridSA, GPU/automata-processor work for ideas on shared state); prefilter planning; lazy-DFA memory bounding; streaming/chunked matching; differential testing and verified/proven matcher techniques (Linden, Warblre, ReTest); counter-ReDoS attacks (as test generators).
- **Use with care:** derivative engines (RE#, .NET NonBacktracking) — useful for rewrite/simplification ideas, but they assume whole-input access, leftmost match semantics and reverse passes; linear-time lookaround algorithms (Mamouras, Barrière) — their lookahead needs right-to-left passes, so only the bounded-lookahead ideas transfer to streaming.
- **Low value here:** submatch/capture extraction, POSIX vs leftmost-first semantics, backreference theory, LLM-oriented regex work.
