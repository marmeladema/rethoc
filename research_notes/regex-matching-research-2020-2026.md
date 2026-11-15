# Regex matching research 2020–2026, read for rethoc

This report surveys the 2020–2026 regex-matching literature from the point of view of rethoc. The engine's goal and constraints are in [engine-goal.md](engine-goal.md):
- **Patterns and input:** untrusted patterns over untrusted streaming input.
- **Output:** a yes/no match per stream, with no offsets.
- **Deployment:** security rules on live traffic, many concurrent streams sharing 500+ compiled regexes, and multi-pattern matching likely on the roadmap.
- **Guarantees:** linear time and agreement across tiers are non-negotiable; throughput and per-stream memory come right after.

The PDFs are in [regex-matching-research-2020-2026/pdfs/](regex-matching-research-2020-2026/pdfs/README.md), with searchable full text in [text/](regex-matching-research-2020-2026/text/INDEX.md). The four topic notes files behind this report are in the same folder. Statements marked *(inference)* are my reading of how a result applies to rethoc; the papers do not say them.

## Summary

The decade's central result for this engine is that **counted repetition is now partly solved in software, class by class**.
- **Fixed-length bodies:** bit-vector automata (Le Glaunec, Kong & Mamouras, OOPSLA 2023) match `(σ0…σk−1){m,n}` in O(k) per byte regardless of m and n, in any regex context.
- **Variable-length bodies:** counting-set automata (Turoňová et al., OOPSLA 2020) and the *synchronizing* class (Holík et al., FoSSaCS 2023) give bound-independent matching for a large, syntactically recognisable class that covers "nearly all" counting in real rule sets.
- **Arbitrary nesting:** still open.

The hardware papers add a compile-time rule for when one small counter is enough versus a full bit vector, which matters directly for per-stream state. Counters are also proven to be the DoS weak point of non-backtracking engines, including Hyperscan inside Snort (USENIX Security 2022), and the attack generators double as test generators.

Three other threads matter here:
- **Multi-pattern and throughput:** Hyperscan's decomposition, its 2025 Hyperflex follow-up, HybridSA's bit-parallel compiler, and planned prefilters (BLARE).
- **Derivative engines:** .NET NonBacktracking and RE#. Their simplification rules are especially useful for a yes/no engine, but they assume whole-input access.
- **Correctness tooling:** Linden's verified PikeVM and equivalence rules, and metamorphic fuzzing (ReTest).

Lookaround, backreference, capture and backtracking-memoization research advanced too, but is mostly outside rethoc's scope. The one exception is bounded lookahead, which has a streaming-compatible technique in RE#.

## 1. Counted repetition: what is solved, per tier

**The problem.** `.*a.{n}` has a regex of size Θ(log n), a minimal NFA of Θ(n) states and a minimal DFA of Θ(2^n) states. Lazy-DFA engines (RE2, grep) thrash their caches on such patterns and fall back to NFA simulation. RE2 and Hyperscan "do not have first-class support for counting": they unroll counters, and RE2 rejects bounds above 1000 (BVA paper; .NET PLDI 2023 paper). The .NET team found that Rust's `regex` "would be a clear winner if not for the pattern `[a-q][^u-z]{13}x`", which blows up to 2^13 DFA states.

### Fixed-length bodies (Tier 2's class, and bounded gaps)

**Bit-vector automata** (OOPSLA 2023, DOI 10.1145/3586044) are nondeterministic bit-vector automata, expressively equal to bounded counter automata. Theorem 4.1 ("Efficient Flat Loops") handles `(σ0…σk−1){m,n}` in any context with:
- one bit vector per body position, where bit j means "j iterations completed";
- vectors stored as circular arrays, so a shift costs O(1);
- a rotating index instead of copying vectors;
- a per-vector witness y_i, the smallest j ≥ m with a set bit, so acceptance is the O(1) test `m ≤ y_i ≤ n`.

It extends to `r{m,n}` for any counting-free `r` whose strings all have the same length, at O(ℓ) per step. On Snort, Suricata, SpamAssassin, Protomata, ClamAV and RegexLib, BVA-Scan beats RE2 by 9–15× on random input, 11–36× on adversarial input, and 71× on average (up to 844×) on the counting-heavy subset. It is only competitive, not dominant, for small bounds (under about 128).

For rethoc:
- **Tier 2:** BVA's fixed-length class is the same class Tier 2 accepts *(inference)*, so the paper is a reference implementation and correctness baseline for it.
- **Bounded gaps:** gaps like `A[^/]{0,20}B` are the k = 1 case. The bit vector reduces to a window of anchor-end positions, and a byte outside the gap class empties it *(inference)*.
- **Per-stream cost:** BVA's state is up to n bits per body position per counter. With 500 patterns × many streams, that bill matters *(inference)*. The next point reduces it.

**Single counter versus bit vector** (Kong et al., PLDI 2022; BVAP, ASPLOS 2024; RAP, ISCA 2025):
- **The rule:** these in-memory hardware designs use a static analysis to decide, per repetition, whether a single Θ(log n) counter suffices or a bit vector is needed.
- **The cost of unrolling:** an unrolled `{..1024}` counter costs at least 16,384 memory bits where about 10 bits of information suffice.
- **Reported savings:** Kong et al. report up to 76% energy and 58% area savings, and BVAP reports 67–95% better energy efficiency than CA, eAP and CAMA.
- **For rethoc:** in software, the same analysis decides between a scalar register and a bitset or queue *per counter, per stream*. That is the most direct lever on per-stream memory for counting patterns *(inference)*.

### Variable-length, non-nested bodies (Tier 3's class)

**Counting-set automata (CsA)** (OOPSLA 2020, DOI 10.1145/3428286) build an Antimirov-style counting automaton, then determinize it into a deterministic automaton whose registers hold *sets* of counter values. Automaton size does not depend on the bounds. A counting set is stored as an offset plus a strictly increasing queue, and insert-0, increment-all, min, max and reset are all O(1) (§2). Two conditions limit it:
- **Uniformity (§6.4):** the result is exact only when, within each counter's scope, every CA state in a powerstate carries the same set of counter values (the "Cartesian" encoding), and no state is in the scope of two counters. Otherwise the CsA over-approximates the language. The BVA authors show the resulting false positives on `.*(aa){100}`.
- **Fast versus slow transitions (§6.1):** a transition that must union an Incr and a Noop of the same set costs O(n).

A simple syntactic class is guaranteed exact and fast: `(σ1…σn){l,k}` where σ1 is disjoint from every later σi (§6.5). In their benchmark, 4,429 of 5,000 counting regexes were uniform and fast.

**Synchronizing counting** (FoSSaCS 2023, open access) defines the class of counting regexes matchable in time linear in the text and independent of the bounds. Empirically it covers "nearly all" practical counting, and the paper calls the general problem open "for at least two decades". **Sparse counting-sets** (NCMA 2026) attack CsA's replication cost at branching transitions. **Automata with Bounded Repetition in RE2** (Horký, Síč & Turoňová, EUROCAST 2022) is the one paper that puts counting-set techniques inside a production lazy-DFA engine. It is the closest prior art to adding counters to a Tier-1-style engine.

For rethoc *(inference)*:
- **A diagnosis for Tier 3:** the Tier 3 redesign documents describe the current design's core defect as lost provenance. The cached DFA state cannot tell which runtime counter values belong to which NFA state. CsA's uniformity condition is a precise statement of when that information can safely be merged: a counter's value set must be identical across all states in its scope.
- **A target for eligibility:** the synchronizing class, and CsA's uniformity check, which can be done during construction, are candidate *eligibility predicates* for a rebuilt Tier 3. Accepting exactly those, and sending the rest to Tier 4 or the NFA, matches the "reject rather than mis-accept" rule.
- **A data structure that fits streaming:** offset plus queue is per-stream state, bounded by the counter bound, with O(1) updates.

### Nested repetition (Tier 4's class) and the open frontier

- **Nested counting breaks the simple encodings:** CsA's uniformity requires that no state be in two counters' scopes, which nested counting violates. Nested and non-fixed-length bodies are where BVA falls back too.
- **No general bound-independent algorithm exists:** none has been published, and I found no conditional lower bound proving one impossible. The problem is open, not proven hard.
- **The practical answer is the tier structure itself:** recognise the tractable classes precisely and fall back with bounded cost.

### Counters as an attack surface, and as a test generator

**"Counting in Regexes Considered Harmful"** (USENIX Security 2022) shows that bounded repetition is the main DoS weakness of *non-backtracking* matchers. Its GadgetCA generator slowed Hyperscan inside Snort and NVIDIA BlueField-2 regex hardware. Its generator, and the adversarial-text generator in the CsA paper (§7.3), search for inputs that maximise the number of distinct powerstates and counter values. These are the inputs that defeat lazy-DFA caches. A USENIX Security 2024 follow-up (Su et al.) extends detection to non-backtracking engines.

For rethoc *(inference)*: these generators are ready-made adversarial inputs for the fuzz targets and the pathological benchmarks, and for checking each tier's claimed per-byte bound.

## 2. Throughput and multi-pattern matching

- **Hyperscan (NSDI 2019):** still the reference multi-pattern CPU design. It decomposes rules around literal components, runs a fast multi-literal matcher, and verifies with small automata. Its streaming mode, with compact saved state per stream, is the industrial precedent for rethoc's deployment.
- **Hyperflex (arXiv, December 2025, by Hyperscan's authors):** finds DFA regions where SIMD can do the transitions. It reports up to 2.27× over Hyperscan's DFA and is deployed in Hyperscan.
- **Teddy (ICPP 2021):** the SIMD multi-literal matcher used by Hyperscan, Rust's `regex` and .NET.
- **HybridSA (OOPSLA 2024):** compiles a large regex class, including bounded repetition, into generalized shift-and bit masks for multi-pattern GPU matching. It reports 4–60× over the best CPU engine.

For rethoc *(inference)*: HybridSA's compiler maps onto a CPU bit-parallel tier (u64/u128/AVX2). Bit-parallel NFA state is also very compact per stream, which suits many streams and many patterns. Packing many small patterns into one wide state word is a natural multi-pattern design.

**Prefilters:**
- **BLARE (SIGMOD 2023):** splits a regex into literal and regex parts, evaluates them out of order, and picks a plan at runtime with a multi-armed bandit. It reports 1.6–3.7× over RE2.
- **PVLDB 2025 n-gram study:** finds no universal winner among n-gram selection methods.

For rethoc *(inference)*: with 500+ rules per stream, the main question is which *rules* can possibly match. A shared literal or n-gram prefilter across the rule set, Hyperscan-style, gates per-rule matchers so they run only when needed. For proximity rules like `A.{0,N}B`, starting from the rarer anchor is BLARE's strategy specialised to gaps.

**GPU and in-memory accelerators** (ngAP and AsyncAP, 2023–2024; the Impala, Sunder and CAMA processors):
- **Their focus:** they parallelise across input offsets or many patterns at once. They are mainly a source of state-encoding ideas.
- **Bounded repetition is central to their designs:** hardware papers put it at the centre, while CPU engines largely ignore it.

**Benchmarks:** the hardware and GPU papers use ANMLZoo and AutomataZoo, which include Snort, ClamAV and Protomata rule sets. Adding those rule sets to rethoc's benchmarks would allow direct comparison with the counting papers.

## 3. Derivative engines: rewrite ideas for a yes/no engine

**.NET NonBacktracking** (Moseley et al., PLDI 2023):
- **Design:** location-based symbolic derivatives, with a formal correctness proof, used to match with backtracking semantics.
- **Counters:** handled lazily ("loops such as `a{0,n}` end up being expanded fully only when the upper bound n has been reached in the given input"), so the worst case still equals unrolling.

**RE#** (POPL 2025, DOI 10.1145/3704837):
- **Features:** adds intersection, complement and restricted lookarounds, keeps matching input-linear, and uses leftmost-longest (POSIX) semantics.
- **Speed:** claims over 71% faster than Rust's `regex` on the rebar baseline.
- **Simplification:** it keeps the derivative automaton small with algebraic rewrites (Fig. 6). Examples are subsumption, `R{l,m}|R{k,n} → R{l,max(m,n)}`, and absorbing alternatives into `ψ*` when they use only ψ's characters.
- **Counters:** on the rebar "context" benchmark, which contains `[\s\S]{0,100}`, these rewrites put RE# ahead of other automata engines by more than 8×.
- **Caveat:** the authors note that such rewrites are invalid under ordered (backtracking) alternation.

What transfers to rethoc *(inference)*:
- **Rewrites are safe:** rethoc only answers yes/no, so alternation is effectively commutative. The whole family of union-subsumption and loop-merging rewrites is semantics-preserving for it. The rewrites could shrink NFA/DFA state counts for counter-heavy rules at compile time, independent of using derivatives at runtime.
- **The matching algorithm doesn't transfer:** RE#'s matching uses a reverse pass over the whole input, which is incompatible with streaming.
- **Open combination:** nobody has combined derivatives with counting sets or bit vectors; the RE# paper itself lists it as an open problem.

## 4. Correctness: oracles, proofs and fuzzing

rethoc's third hard guarantee is that every tier agrees with the NFA. The relevant work:
- **Linden** (Barrière, Deng & Pit-Claudel, POPL 2026, Distinguished Paper): a backtracking-tree semantics proven equivalent to Warblre (ICFP 2024, a Rocq mechanization of ECMAScript regexes), with a contextual-equivalence notion used to prove or refute published rewrite rules, and **the first formal proof of the PikeVM**. For rethoc it is a model of how to state and prove "tier X is equivalent to the NFA on its accepted class" *(inference)*.
- **Systematic testing of regex engines** (Çakar, Lee & Davis, arXiv 2026): 1,007 bugs and 156 CVEs across 22 engines. Half of the engines rely on differential testing, which dialect differences make noisy. The authors propose metamorphic fuzzing (ReTest), which found new PCRE memory-safety bugs. For rethoc *(inference)*: the NFA oracle avoids dialect noise, and equivalence-preserving pattern rewrites, such as unrolling `r{m,n}` or applying the RE# rules above, give metamorphic relations that test eligibility analysis as well as matching.
- **Verified derivative matchers:** Zhuchko et al. (CPP 2024) and the ITP 2025 finiteness proof, both in Lean.
- **Verified lookaround matching:** Chattopadhyay et al. (CPP 2025), in Coq.

## 5. Limited lookahead, if it is added

- **Mamouras & Chattopadhyay (POPL 2024):** linear lookaround via oracle NFAs, but lookahead needs a right-to-left pass over the input. The paper's one-pass variant (Fig. 5) streams only when all assertions are lookbehinds.
- **Barrière & Pit-Claudel (PLDI 2024):** also linear, and the algorithms reached V8 and a rust-lang/regex PR. Their lookahead handling likewise works on the whole input.
- **RE# (POPL 2025, §4.8):** handles lookahead *forward*. A pending lookahead `(?=A)` carries a set of offsets back to candidate match ends, the derivative of `A` advances on each byte, and the match is confirmed when `A` becomes nullable. The offset sets are stored as sorted ranges, so long contexts stay cheap.
- **For rethoc *(inference)*:** for a yes/no streaming engine with lookahead of bounded length, this is the streaming-compatible design. The delay before an answer is the lookahead's maximum length, and per-stream state is bounded by that length.

## 6. Out of scope for now (brief)

- **Backtracking defences:**
  - Memoization keeps backtracking linear (Davis et al., S&P 2021), extended to lookaround and atomic groups by Fujinami & Hasuo (ESOP 2024) and shipped in Ruby 3.2.
  - V8 switches to a linear engine after 50,000 backtracks.
- **ReDoS detection and repair:** RENGAR (S&P 2023), PUFFERDOS (S&P 2026), LLM-guided repair (EACL 2026), and a SoK (ASIA CCS 2025). These matter only for backtracking engines.
- **Backreferences:** matching is provably hard. Backreferences plus lookahead capture exactly NLOG (ICALP 2024), and real JavaScript regex matching is PSPACE-complete (2026). Polynomial algorithms exist only for restricted shapes (MFCS 2025; Bille, Gørtz & Jessen 2026).
- **Captures and match semantics:** TDFA (Borsotti & Trofimovich) and the POSIX/leftmost-first work. These matter only if offsets or captures are added.
- **Fine-grained complexity:** O(nm) for classical regexes is SETH-tight, with only log factors to shave (Schepper, ESA 2020). Bille & Gørtz (SODA 2024) explain why sparse state-set simulation is fast in practice, which supports keeping NFA fallback state sparse.
- **LLM-oriented regex work:** constrained decoding (Outlines, XGrammar) and LLM-generated ReDoS studies.

## Key papers for rethoc

All papers below are in `regex-matching-research-2020-2026/pdfs/`, with full text in `text/` under the same name.

| Paper | Venue | Why it matters for rethoc | File stem |
|---|---|---|---|
| Le Glaunec, Kong, Mamouras — Bit Vector Automata | OOPSLA 2023 | Bound-independent fixed-length counters (Tier 2, gaps) | `2023-oopsla-leglaunec-bit-vector-automata` |
| Turoňová et al. — Counting-Set Automata | OOPSLA 2020 | Counting sets, uniformity condition (Tier 3 eligibility and state) | `2020-oopsla-turonova-counting-set-automata` |
| Holík et al. — Synchronizing Counting | FoSSaCS 2023 | The tractable counting class | `2023-fossacs-holik-synchronizing-counting` |
| Horký, Síč, Turoňová — Bounded Repetition in RE2 | EUROCAST 2022 | Counting sets inside a lazy-DFA engine | `2022-eurocast-horky-bounded-repetition-re2` |
| Kong et al. — In-memory codesign | PLDI 2022 | Single counter vs bit vector analysis (per-stream memory) | `2022-pldi-kong-hardware-codesign-inmemory-matching` |
| Turoňová et al. — Counting Considered Harmful | USENIX Sec 2022 | Counter DoS against non-backtracking engines; input generator | `2022-usenixsec-turonova-counting-considered-harmful` |
| Berglund et al. — Sparse Counting-Sets | NCMA 2026 | Cheaper counting-set replication | `2026-ncma-berglund-sparse-counting-sets` |
| Wang et al. — Hyperscan | NSDI 2019 | Multi-pattern decomposition, streaming state | `2019-nsdi-wang-hyperscan-fast-multipattern-regex` |
| Liu et al. — Hyperflex | arXiv 2025 | SIMD DFA acceleration | `2025-arxiv-liu-hyperflex-simd-dfa-model` |
| Le Glaunec et al. — HybridSA | OOPSLA 2024 | Bit-parallel multi-pattern compilation | `2024-oopsla-leglaunec-hybridsa-gpu-bit-parallelism` |
| Zhang et al. — BLARE | SIGMOD 2023 | Prefilter planning | `2023-sigmod-zhang-exploiting-structure-regex-queries` |
| Varatalu et al. — RE# | POPL 2025 | Simplification rewrites; streaming lookahead offsets | `2025-popl-varatalu-resharp-derivative-regex` |
| Moseley et al. — .NET NonBacktracking | PLDI 2023 | Derivatives with counters, industrial proof | `2023-pldi-moseley-derivative-nonbacktracking-matching` |
| Barrière, Deng, Pit-Claudel — Linden | POPL 2026 | Verified PikeVM, equivalence proofs | `2025-arxiv-barriere-js-regex-formal-verification` |
| Çakar, Lee, Davis — Testing regex engines | arXiv 2026 | Metamorphic fuzzing | `2026-arxiv-cakar-systematic-testing-regex-engines` |
| Bille & Gørtz — Sparse Regex Matching | SODA 2024 | Why sparse NFA simulation is fast | `2024-soda-bille-sparse-regex-matching` |

## Conclusion

The literature supports rethoc's tiered design and gives each tier a concrete reference:
- **Tier 2 and bounded gaps:** bit-vector automata for fixed-length bodies.
- **Tier 3:** counting-set automata and the synchronizing class for non-nested variable-length bodies, with a precise statement (uniformity) of when merging counter state is safe. This is the property the Tier 3 redesign is trying to recover.
- **Tier 4 and the NFA:** honest fallback for what remains open.

For the deployment target:
- **Per-stream memory:** the most valuable results are the compact counter representations, from a scalar counter when the hardware papers' analysis allows to bit-vector and offset-plus-queue structures.
- **Multi-pattern matching:** Hyperscan-style decomposition with a shared prefilter, plus bit-parallel NFA encodings that pack many small patterns per word.

For correctness, the NFA oracle combined with metamorphic rewrites and counter-targeted adversarial inputs is a stronger test harness than differential testing alone. Linden shows how far proofs of tier equivalence can go.

What remains open: bound-independent matching for nested or non-synchronizing counting, counting inside derivative engines, and published work on per-stream state for counter automata at IDS scale.
