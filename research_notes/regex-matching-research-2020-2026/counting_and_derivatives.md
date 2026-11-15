# Regex matching with bounded repetition (counting) and derivative-based matching, 2020–2026

Scope note: the papers below were checked to exist via publisher or arXiv pages. Where I read the full text (BVA OOPSLA'23, .NET PLDI'23), the notes quote or paraphrase it directly. Everything else comes from the abstract or landing page, and is marked "(abstract-level)". ACM PACMPL (POPL/PLDI/OOPSLA) papers are open access (CC-BY) at dl.acm.org. ASPLOS, CPP, and USENIX author PDFs are linked where I found them. Springer (FoSSaCS) has a paywalled DOI but an open tech report on arXiv.

## Q1. Counting automata, counting-set automata, bit-vector automata, hardware counters (2020–2026)

### Takeaway
There are two main lines of work. The first is the Brno/MSR line: deterministic counting-set automata (OOPSLA'20), then the "synchronizing" class (FoSSaCS'23), then sparse counting-sets (NCMA'26). The second is the Rice line (Mamouras): nondeterministic counter and bit-vector automata (PLDI'22 hardware, OOPSLA'23 software BVA-Scan, ASPLOS'24 BVAP, OOPSLA'24 GPU HybridSA). The Rice line's key practical result is directly usable in a Rust "bounded-gap" engine. For a fixed-length body (σ0…σk−1){m,n}, keep one bit-vector per body position as a circular buffer, plus a "min witness index ≥ m" integer per vector. That gives O(k) time per input character, independent of m and n, and works as a subexpression in any context.

### Cited Findings
**Counting-set automata (CsA), OOPSLA 2020.** Holíková (Turoňová), Holík, Lengál, Saarikivi, Veanes, Vojnar, "Regex Matching with Counting-Set Automata", PACMPL 4(OOPSLA), 2020, DOI 10.1145/3428286 (open access).
- The pipeline translates the regex Antimirov-style into nondeterministic counting automata (CA), then determinizes them into CsAs. CsA registers hold *sets* of bounded integers and support a limited set of constant-time operations. "The size of the produced CsAs does not depend on the repetition bounds," whereas the DFA is exponential in the bounds. It works for "a large sub-class" of regexes only. — [FIT VUT record](https://www.fit.vut.cz/research/result/c168498/.en); [ACM DL](https://dl.acm.org/doi/10.1145/3428286)
- Limitation (critique by Le Glaunec et al.): the CA-to-CsA determinization "does not always preserve the semantics". The automata sometimes over-approximate the language, which gives false-positive matches. Patterns like `.*(aa){100}` and `.*a{0,100}a{200}` are not handled efficiently. — [BVA paper, OOPSLA'23 (open PDF)](https://ohyoukillkenny.github.io/source/BVA.pdf)

**Synchronizing counting, FoSSaCS 2023.** Holík, Síč, Turoňová, Vojnar, "Fast Matching of Regular Patterns with Synchronizing Counting", FoSSaCS 2023, LNCS (Springer, DOI 10.1007/978-3-031-30829-1_19, paywalled; open tech report arXiv:2301.12851; an open PDF is also on FIT VUT).
- The paper frames the problem: matching linear in the text and independent of the repetition bounds "has been an open problem for at least two decades". It defines a class of counting regexes called *synchronizing* that admits such fast matching, using an improved and analyzed CsA algorithm. Empirically, this class covers "nearly all" counting found in practical regex sets (abstract-level). — [arXiv 2301.12851](https://arxiv.org/abs/2301.12851); [Springer](https://link.springer.com/chapter/10.1007/978-3-031-30829-1_19); [open PDF (FIT VUT)](https://fit.vut.cz/research/publication-file/c185169/254333/978-3-031-30829-1_19.pdf)

**Sparse counting-sets, NCMA 2026.** Berglund (Umeå), van der Merwe (Stellenbosch), Sung (Yonsei), "Efficient Regex Matching with Sparse Counting-Sets", Proc. NCMA 2026, EPTCS 446, pp. 19–36; arXiv:2606.26679 (June 2026).
- The bottleneck in prior CsA matching is replicating counting-sets at branching transitions. The paper keeps only "essential" counter values (sparse sets) to cut replication overhead. The abstract states no complexity bounds and no comparisons with RE2 or Rust (abstract-level). — [arXiv 2606.26679](https://arxiv.org/abs/2606.26679)

**In-memory hardware with counters/bit-vectors, PLDI 2022.** Kong, Yu, Le Glaunec, Mamouras et al. (Rice), "Software-Hardware Codesign for Efficient In-Memory Regular Pattern Matching", PLDI 2022; arXiv:2209.05686.
- In-memory automata processors unfold counters into many states. This paper adds counter and bit-vector modules to an in-memory NFA architecture (inspired by nondeterministic counter automata, NCA). A static analysis decides per repetition whether a single Θ(log n) counter suffices ("easy" repetitions) or a bit vector is needed. In a 28nm SPICE-based simulation it reports up to 76% energy and 58% area reduction vs. traditional in-memory NFA processors. — [arXiv 2209.05686](https://arxiv.org/abs/2209.05686); [PLDI'22 page](https://pldi22.sigplan.org/details/pldi-2022-pldi/31/Software-Hardware-Codesign-for-Efficient-In-Memory-Regular-Pattern-Matching); [NSF PAR PDF](https://par.nsf.gov/servlets/purl/10330072)

**Nondeterministic bit-vector automata (NBVA) / BVA-Scan, OOPSLA 2023.** Le Glaunec, Kong, Mamouras, "Regular Expression Matching using Bit Vector Automata", PACMPL 7(OOPSLA1), Art. 92, Apr 2023, DOI 10.1145/3586044 (open access). Full text read.
- The NBVA model is expressively equivalent to NCAs with bounded counters. Main result (Thm 4.1, "Efficient Flat Loops"): for (σ0σ1…σk−1){m,n} with character-class predicates σi, used in any context inside a larger regex, there is an algorithm with O(k) time per step. The construction:
  - one bit vector v_i per body position, where bit j means "j iterations completed";
  - a circular-array shift, so the shift costs O(1);
  - a rotating index s instead of copying vectors (only the vector crossing the back edge changes each step);
  - a per-vector integer y_i = smallest j ≥ m with v_i[j]=1, so the acceptance test is `m ≤ y_i ≤ n` in O(1) instead of an Θ(n−m) OR over the bits.

  — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)
- The paper extends this to r{m,n} where r is counting-free and all strings in L(r) have the same length: O(ℓ) per step, where ℓ is the number of predicate occurrences in r. The authors claim this covers "the vast majority of challenging bounded repetitions that arise in practice". — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)
- Motivation: `.*a.{n}` has a Θ(n)-state minimal NFA and a Θ(2^n)-state minimal DFA (Meyer–Fischer 1971), while the regex has size Θ(log n). Lazy-DFA engines (grep, RE2) thrash their DFA cache and fall back to NFA simulation. — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)
- Evaluation is against RE2, PCRE, and the CA (CsA) engine on Snort, Suricata, SpamAssassin, Protomata, ClamAV, and RegexLib.
  - Average speedups on random input: 9–15× vs RE2, 9–164× vs PCRE, 125–627× vs CA. On adversarial input: 11–36× vs RE2.
  - On the counting-heavy subset: an average of 71× vs RE2 (up to 844×) and 151× vs PCRE (up to 3660×).
  - BVA-Scan time stays flat as k grows. It is competitive but not dominant for small k (<128).
  - RE2 rejects bounds over 1000. BVA-Scan also uses shift-and/shift-or for simple subpatterns, like Hyperscan's 128-bit SIMD shift-or.

  — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)

**BVAP, ASPLOS 2024.** Wen, Kong, Le Glaunec, Mamouras, Yang, "BVAP: Energy and Memory Efficient Automata Processing for Regular Expressions with Bounded Repetitions", ASPLOS '24, pp. 151–166, DOI 10.1145/3620665.3640412.
- This is an in-memory Bit Vector Automata Processor based on "Action-Homogeneous NBVA" (AH-NBVA). It extends an in-memory NFA accelerator (CAMA baseline) with bit-vector modules (abstract-level). — [NSF PAR record](https://par.nsf.gov/biblio/10534634); [NSF PAR PDF](https://par.nsf.gov/servlets/purl/10534634)

**HybridSA, OOPSLA 2024.** "HybridSA: GPU Acceleration of Multi-pattern Regex Matching using Bit Parallelism", PACMPL (OOPSLA 2024), DOI 10.1145/3689771 (Mamouras group, per the NSF PAR listing).
- A GPU multi-pattern engine using bit-parallelism that reports 4–60× higher throughput than the state-of-the-art CPU engine on real-world benchmarks (abstract-level; the ACM page returned 403 to my fetch). — [ACM DL](https://dl.acm.org/doi/full/10.1145/3689771); [NSF PAR](https://par.nsf.gov/biblio/10618058-hybridsa-gpu-acceleration-multi-pattern-regex-matching-using-bit-parallelism)

**RAP, ISCA 2025 (unverified details).** "RAP: Reconfigurable Automata Processor", DOI 10.1145/3695053.3731106. A search snippet says it "supports the NBVA model by unified storage for both character classes and bit vectors". I did not retrieve the paper. — [ACM DL](https://dl.acm.org/doi/10.1145/3695053.3731106)

**Counting as the Achilles heel of nonbacktracking engines (ReDoS), USENIX Security 2022.** Turoňová, Holík, Homoliak, Lengál, Veanes, Vojnar, "Counting in Regexes Considered Harmful: Exposing ReDoS Vulnerability of Nonbacktracking Matchers", USENIX Security '22, pp. 4165–4182.
- Bounded repetition is the main weak point of automata-based (nonbacktracking) matchers. The authors' generator (GadgetCA, based on CsA) produces inputs that slow down such matchers, including Snort with Hyperscan and NVIDIA BlueField-2 regex hardware. — [USENIX page](https://usenix.org/conference/usenixsecurity22/presentation/turonova); [author PDF](https://ondrik.github.io/publications/usenix-security22-counting-redos.pdf)
- A later USENIX Security 2024 paper reportedly states that GadgetCA's model is sound only for a limited subclass of counting regexes (search snippet, not verified). The 2024 SoK on ReDoS covers this area. — [SoK ReDoS, arXiv 2406.11618](https://arxiv.org/abs/2406.11618)

**Background (pre-2020, brief):** XFA, Smith et al. 2008 (DFA + counters); counting-NFA, Becchi & Crowley 2008; Holík et al. 2019, determinization of counter automata. All cited as related work in [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf).

### Inferences
- For a Rust "bounded-gap" engine handling `.{0,k}` / `[class]{m,n}` gaps, the BVA "flat loop" construction (one bit-vector + witness index per body position, circular shift) is the best-documented general algorithm with per-step cost independent of the bounds. A gap `.{m,n}` has k=1, so it costs O(1) per byte plus the bitset shift. With a circular buffer and a "latest-entry position" encoding, this reduces to a sliding window over entry positions.
- CsA is more general in principle (arbitrary bodies) but has documented soundness and coverage caveats and replication overhead. The FoSSaCS'23 "synchronizing" class gives the condition under which CsA is fast. The 2026 sparse-set work is the only post-2023 algorithmic follow-up I found in this line.
- Hardware papers (PLDI'22, ASPLOS'24, ISCA'25) offer a useful static analysis: classify each repetition as "single counter suffices" vs. "needs bit vector". That classification maps naturally onto choosing a cheap counter vs. a bitset in software.

### Gaps
- I could not retrieve full text for HybridSA, RAP, sparse counting-sets, or the FoSSaCS'23 per-character complexity constant.
- I found no paper that evaluates Rust `regex` specifically on counting workloads, apart from mentions in the .NET and RE# papers (see Q5).

## Q2. Theoretical lower/upper bounds for matching with bounded repetition

### Takeaway
I found no fine-grained (conditional) lower bound specific to bounded repetition. The best-known positive results are the class-restricted bound-independent algorithms (BVA flat loops; synchronizing CsA). The general problem, linear in the text and independent of the bounds for *all* counting regexes, is described as open by the FoSSaCS'23 authors. The classic fine-grained results (Backurs–Indyk; Abboud–Bringmann; Schepper ESA 2020) concern counting-free regexes, where the pattern size m already includes the unrolled repetitions.

### Cited Findings
- Holík et al. say that matching "linear in the length of the text and independent of the repetition bounds has been an open problem for at least two decades". — [arXiv 2301.12851](https://arxiv.org/abs/2301.12851)
- Schepper, "Fine-Grained Complexity of Regular Expression Pattern Matching and Membership", ESA 2020, arXiv:2008.02769. It refines the Backurs–Indyk (FOCS 2016) dichotomy of hard pattern types, and gives improved lower bounds of the form Ω(nm/log^c n) (under the Formula-SAT hypothesis, following Abboud–Bringmann ICALP 2018's nm/log^{7+ε} n bound). This holds for classical regexes, not succinct counting. — [arXiv 2008.02769](https://arxiv.org/abs/2008.02769)
- Succinctness: `.*a.{n}` has a minimal NFA of Θ(n) states and a minimal DFA of Θ(2^n) states, with regex size Θ(log n). Any NFA is therefore exponential in the regex size. — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)
- Nogami, Nakamura, Terauchi, "Hardness of Regular Expression Matching with Extensions", arXiv:2601.03020 (Jan 2026, v4 Sep 2026). It proves OV/k-OV/k-Clique conditional lower bounds for intersection, *squaring*, complement, and backreferences. The abstract does not address r{n,m} counting directly. Squaring (r²) is a succinctness operator related to counting, but I did not verify whether the bounds transfer. — [arXiv 2601.03020](https://arxiv.org/abs/2601.03020)
- "On the computational complexity of JavaScript regex matching, in Rocq", arXiv:2603.26139 (2026). It reports PSPACE-completeness results for JS regex matching including lower-bounded quantifiers. These are complexity-class results (backtracking semantics, lookarounds, captures), not counting lower bounds for classical semantics (abstract and snippet level). — [arXiv 2603.26139](https://arxiv.org/abs/2603.26139)

### Inferences
- In practice, per-step cost for a general counted subexpression r{m,n} is at best O(|r|·(n−m)/w) with naive NFA simulation over word-sized bitsets. Bound-independence is known only for restricted classes (fixed-length bodies; synchronizing). An engine should detect those classes and fall back otherwise.

### Gaps
- I found no paper proving a conditional lower bound that counting cannot be matched in time independent of the bounds. I found nothing by Bille/Gørtz on counting specifically, so their pre-2020 work (e.g., the Bille–Thorup line on regex matching) is not covered here.

## Q3. Symbolic derivatives: .NET NonBacktracking, RE#, verified derivatives, derivatives with counters

### Takeaway
Derivative engines (the .NET 7 NonBacktracking engine, PLDI'23; RE#, POPL'25) handle counters *lazily*. The derivative of R{m,n} decrements the bounds, so the state space is explored only as the input demands it. This is still not bound-independent in the worst case: the [a-q][^u-z]{13}x pattern produces 2^13 DFA states. The RE# line has extended to decision procedures (CAV'25), Lean proofs (CPP'24, ITP'25), and a Rust rewrite.

### Cited Findings
**.NET NonBacktracking, PLDI 2023.** Moseley, Nishio, Perez Rodriguez, Saarikivi, Toub, Veanes, Wan, Xu, "Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics", PACMPL 7(PLDI), Art. 148, 2023, DOI 10.1145/3591262. Open PDF from MSR; extended version MSR-TR-2023-15. Full text read.
- It uses location-based (anchor-aware) symbolic derivatives with PCRE backtracking semantics (eager/lazy loops, alternation order), proven correct. The authors call it the first industrial input-linear derivative matcher. It is shipped in .NET 7. — [PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf); [MSR-TR-2023-15](https://microsoft.com/en-us/research/uploads/prod/2023/04/MSR-TR-2023-15.pdf)
- On counters: other nonbacktracking engines (RE2, Hyperscan) "do not have first-class support for counting. Instead, counters are unwound up-front by a pre-processor… Use of large counters in general is an Achilles heel of all state-of-the-art nonbacktracking regex matchers". "Derivatives decrease counters by one, and thus loops such as a{0,n} end up being expanded fully only when the upper bound n has been reached in the given input." The linear-in-pattern complexity of Der/Null calls is stated "where any loops have been unrolled". — [PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf)
- Benchmark note: Rust regex "would be a clear winner if not for the pattern `[a-q][^u-z]{13}x`". The loop can be re-entered, causing a 2^13 blowup in DFA states. .NET's optimizations (on-demand growing state arrays, dead-end detection) made this pattern "no longer visible as an outlier". The engine has a DFA mode and falls back to an NFA mode (Antimirov-style derivatives) without prior bookkeeping. — [PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf)
- A three-phase match: forward (end), backward (start), then a linear capture pass with tagged derivatives. Unlike RE2, it never falls back to backtracking. — [PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf)

**RE#, POPL 2025.** Varatalu, Veanes, Ernits, "RE#: High Performance Derivative-Based Regex Matching with Intersection, Complement, and Restricted Lookarounds", PACMPL 9(POPL), 2025, DOI 10.1145/3704837 (open access). Preprint arXiv:2407.20479; an earlier preprint is arXiv:2309.14401, "Derivative Based Extended Regular Expression Matching Supporting Intersection, Complement and Lookarounds".
- Symbolic derivatives, no backtracking, with intersection, complement, and restricted lookarounds. Main algorithm is input-linear in theory and in experiments. Authors' claim: "over 71% faster than the next fastest regex engine in Rust on the baseline" and often orders of magnitude faster on extended benchmarks. (abstract-level). The abstract does not say which match semantics RE# uses. My recollection is leftmost-longest rather than backtracking semantics, but that is unverified and should be checked in the paper. — [ACM DL](https://dl.acm.org/doi/10.1145/3704837); [arXiv 2407.20479](https://arxiv.org/abs/2407.20479); [MSR PDF](https://www.microsoft.com/en-us/research/wp-content/uploads/2025/01/popl25-p2-final.pdf)
- Follow-up: Varatalu, Veanes, Zhuchko, Ernits, "Regex Decision Procedures in Extended RE#", CAV 2025 (online 21 Jul 2025). — mentioned in [search results / POPL'25 page](https://popl25.sigplan.org/details/POPL-2025-popl-research-papers/1/RE-High-Performance-Derivative-Based-Regex-Matching-with-Intersection-Complement-a) (I did not fetch the CAV paper itself)
- Practitioner sources: the first author's blog describes a Rust rewrite of RE# using fully symbolic derivatives. It notes the .NET version restricts lookarounds to a normalized form. These are blog posts, not peer-reviewed. — [iev.ee: symbolic derivatives and the Rust rewrite](https://iev.ee/blog/symbolic-derivatives-and-the-rust-rewrite-of-resharp/); [iev.ee: how we built RE#](https://iev.ee/blog/resharp-how-we-built-the-fastest-regex-in-fsharp/)

**Verified derivatives (Lean).**
- Zhuchko, Veanes, Ebner, "Lean Formalization of Extended Regular Expression Matching with Lookarounds", CPP 2024, pp. 118–131, DOI 10.1145/3636501.3636959. Location-based symbolic derivatives with intersection, complement, and lookarounds, mirroring the .NET engine's extension, proven in Lean 4. — [MSR page](https://www.microsoft.com/en-us/research/?p=1115526); [TalTech record](https://ws.lib.ttu.ee/digibase/en/Publ/Item/9bf14029-9eef-433f-ad2b-fb729c726c49)
- Zhuchko et al., "Finiteness of Symbolic Derivatives in Lean", ITP 2025, LIPIcs vol. 352, paper 16 (open access). It proves that symbolic location derivatives have a finite state space when quotienting only by associativity, deduplication, and idempotence, *without* commutativity of union. That matters for backtracking (ordered-alternation) semantics. — [Dagstuhl](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITP.2025.16)

### Inferences
- For counting, derivatives give "lazy unrolling": the cost is paid only along states the input actually reaches. This is not bound-independent. A state like `.{0,k}x` after many re-entries carries a set of residual counters, the same blowup as the DFA. Combining derivative engines with BVA-style bitset counters is a natural but, as far as I found, unpublished direction.

### Gaps
- I found no 2025–2026 paper that adds first-class counter-sets or bit-vectors to *derivative* engines. I did not verify the CAV'25 paper's contents.

## Q4. Linear-time lookaround matching and its relation to counting

### Takeaway
Two independent 2024 papers give linear-time (in the input) algorithms for lookarounds. Mamouras & Chattopadhyay (POPL'24) use oracle NFAs with cached lookaround queries. Barrière & Pit-Claudel (PLDI'24) handle full JavaScript, including capturing lookarounds. Neither targets counting. They are complementary: counters are typically still unrolled, so their complexity is linear in text × (unrolled) pattern.

### Cited Findings
- Mamouras, Chattopadhyay, "Efficient Matching of Regular Expressions with Lookaround Assertions", PACMPL 8(POPL), Art. 92, pp. 2761–2791, 2024, DOI 10.1145/3632934 (open access). It uses an oracle NFA with cached oracle queries to match lookarounds in linear time (abstract-level). — [ACM DL](https://dl.acm.org/doi/full/10.1145/3632934)
- Chattopadhyay, Li, Mamouras, "Verified and Efficient Matching of Regular Expressions with Lookaround", CPP 2025, DOI 10.1145/3703595.3705884. A verified version of the oracle approach (abstract-level). — [ACM DL](https://dl.acm.org/doi/10.1145/3703595.3705884)
- Barrière, Pit-Claudel, "Linear Matching of JavaScript Regular Expressions", PACMPL 8(PLDI), Art. 201, 2024, DOI 10.1145/3656431; arXiv:2311.17620. A non-backtracking linear algorithm for JS regexes with lookarounds, which the authors describe as the first linear-time matching of *capturing* lookarounds in any regex language. It relies on JS capture-reset semantics. The authors say they developed it independently of the POPL'24 work. — [arXiv 2311.17620](https://arxiv.org/abs/2311.17620); [author PDF](https://aurele-barriere.github.io/papers/linearjs.pdf); [ACM DL](https://dl.acm.org/doi/10.1145/3656431)
- "Formal Verification for JavaScript Regular Expressions: a Proven Semantics and its Applications" (extended version), arXiv:2507.13091 (2025). A mechanized JS regex semantics from the same research area (abstract-level; authors not verified here). — [arXiv 2507.13091](https://arxiv.org/abs/2507.13091)
- MFCS 2025, "Efficient Matching of Some Fundamental Regular Expressions with Backreferences" (adjacent topic; not read). — [Dagstuhl](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.MFCS.2025.81)
- BVA authors (2023) note that lookaround is regular but "not supported by any of the available automata-based regex engines (e.g., grep, RE2, Hyperscan)". — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)

### Inferences
- Lookaround and counting algorithms compose orthogonally: lookaround oracles are per-position Boolean side channels, and counters are per-state auxiliary data. None of the papers I found analyze them jointly.

### Gaps
- I did not find Miyazaki/Minamide work in my searches, so it is not covered.

## Q5. State of the art in production engines for counted repetition (per papers)

### Takeaway
According to the academic papers, RE2, Hyperscan, and (by implication) Rust regex unroll counters before building the NFA/DFA. RE2 rejects bounds over 1000. .NET 7 NonBacktracking unrolls lazily via derivatives. No production engine documented in these papers uses counter-sets or bit-vector automata for general counting. The bit-vector approach exists in research engines (BVA-Scan) and hardware (BVAP, RAP).

### Cited Findings
- RE2 and similar engines unfold r{k} into a k-fold concatenation, and "RE2 does not handle regexes of the form r{k} for k > 1000". Hyperscan uses 128-bit SIMD shift-or for some subpatterns. — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)
- RE2 and Hyperscan "do not have first-class support for counting. Instead, counters are unwound up-front by a pre-processor". .NET NonBacktracking expands loops through derivatives on demand. — [PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf)
- Rust regex's lazy DFA outlier is `[a-q][^u-z]{13}x` (a 2^13 state blowup) in the .NET benchmark. — [PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf)
- Hyperscan in Snort and NVIDIA BlueField-2 hardware were shown vulnerable to counting-based ReDoS. — [USENIX Sec'22](https://usenix.org/conference/usenixsecurity22/presentation/turonova)
- Lazy-DFA engines (grep, RE2) thrash their DFA cache on `.*a.{n}`-style patterns and revert to slow NFA simulation; "there are no known ways to mitigate it in the general case" (2023). — [BVA PDF](https://ohyoukillkenny.github.io/source/BVA.pdf)

### Inferences
- A Rust bounded-gap engine implementing BVA-style flat-loop bitsets would be ahead of the published state of the art for production engines on Snort/Suricata-style `.{n}` gaps. The published speedups over RE2 on counting-heavy regexes (average 71×, up to 844×) indicate the size of the opportunity.

### Gaps
- I found no peer-reviewed paper describing Rust `regex`'s internal handling of counted repetition. Statements about Rust here are inferred from benchmark observations in the .NET and RE# papers, not from a source on its implementation.
