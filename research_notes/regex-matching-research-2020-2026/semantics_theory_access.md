# Regex match semantics, submatch extraction, fine-grained complexity, LLM-era regex, and paper access (2020–2026)

Method note: I checked metadata (title, authors, date) for the arXiv IDs below with arXiv abstract-page meta tags or search results. DBLP and the Semantic Scholar API returned HTTP 429 (rate-limited) from this host, so I could not use them for cross-checks. Items marked **[not re-verified this session]** come from background knowledge. They are almost certainly real, but the report writer should check the link before citing them.

## Q1. Submatch extraction and disambiguation (POSIX, greedy/PCRE, JS semantics, derivatives, SMT)

### Takeaway
For a Rust engine, the most directly usable recent work is in three groups:
- **Tagged automata:** Borsotti & Trofimovich give POSIX and leftmost-greedy captures on tagged DFAs/NFAs (re2c).
- **Derivative engines from Microsoft/TalTech:** the .NET NonBacktracking engine (PLDI 2023) and RE# (POPL 2025) give linear-time matching that keeps backtracking (PCRE) match semantics, with extensions.
- **EPFL/CNRS Linden group:** linear-time JS regex with lookarounds (PLDI 2024), a mechanized ECMAScript semantics (ICFP 2024), and the first proof of PikeVM correctness (POPL 2026). The group has also sent a lookbehind PR to rust-lang/regex.

### Cited Findings

**Tagged automata / POSIX (re2c line)**
- **Borsotti & Trofimovich, "A closer look at TDFA"**, arXiv:2206.01398 (2022; v3 revised Jan 2026). It presents what the authors call "the first practical submatch extraction and parsing algorithm based on deterministic finite automata that is capable of both POSIX and leftmost greedy disambiguation." It covers single-pass TDFA for ahead-of-time determinization and multi-pass TDFA for JIT determinization.
  - Relevance: this is the blueprint for adding DFA-speed capture extraction (instead of only a PikeVM/backtracker fallback) and for supporting both POSIX and Perl semantics from one construction. — [arXiv](https://arxiv.org/abs/2206.01398v2), [HTML v3](https://arxiv.org/html/2206.01398v3)
- **Borsotti & Trofimovich, "Efficient POSIX submatch extraction on nondeterministic finite automata"**, *Software: Practice and Experience* 51(2):159–192, 2021 (online 2020), DOI 10.1002/spe.2881. It compares NFA-based POSIX (leftmost-longest) submatch algorithms and combines Laurikari's tagged NFA with an extended Okui–Suzuki disambiguation, which proved faster than Kuklewicz's approach.
  - Paywalled at Wiley. Check the re2c docs or author pages for preprints.
  - Relevance: a reference design if you ever offer a POSIX mode on the PikeVM. — [ScienceGate/DOI record](https://next.sciencegate.app/document/10.1002/spe.2881), [Bohrium abstract](https://www.bohrium.com/paper-details/efficient-posix-submatch-extraction-on-nondeterministic-finite-automata/812551503469346817-2713)
- **Trofimovich, "Tagged Deterministic Finite Automata with Lookahead"**, arXiv:1907.08837 (2017/2019). One-symbol lookahead greatly reduces the number of tag variables and operations on them. The paper also formalizes Kuklewicz's POSIX disambiguation and shows the resulting TDFAs are as efficient as Laurikari or leftmost-greedy TDFAs. This is the basis of re2c's submatch support. — [arXiv PDF](https://arxiv.org/pdf/1907.08837)
- Seminal background **[not re-verified this session]**: Okui & Suzuki, "Disambiguation in Regular Expression Matching via Position Automata with Augmented Transitions", CIAA 2010 (LNCS 6482). It was the first efficient automaton-based POSIX disambiguation; Borsotti & Trofimovich extend it.

**Boost / POSIX formalization**
- **Berglund, Bester & van der Merwe, "Formalising Boost POSIX Regular Expression Matching"**, ICTAC 2018 (LNCS). Boost maximizes the leftmost match with respect to *capturing groups* rather than all subexpressions. The paper gives the first formalization of Boost semantics and a complexity analysis of POSIX vs Boost matching.
  - Relevance: "POSIX" captures differ across libraries (glibc, musl, TRE, Boost), so a conformance test oracle needs a precise formal definition. — [PDF (KCL mirror)](https://cflmark.nms.kcl.ac.uk/hg/lexing/raw-file/tip/Literature/boost-semantics.pdf), [CSIR record](https://researchspace.csir.co.za/signposting/describedby/29f25cc4-503e-4f2c-bf64-b6ffe7e5291d)

**Derivative-based engines with backtracking (PCRE-priority) semantics**
- **Moseley, Nishio, Perez Rodriguez, Saarikivi, Toub, Veanes, Wan, Xu, "Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics"**, PACMPL 7(PLDI):1026–1049, 2023, DOI 10.1145/3591262. It is a derivative-based nonbacktracking matcher that supports anchors and counting and formally preserves backtracking (leftmost, priority-based) semantics. It ships as .NET's `RegexOptions.NonBacktracking` backend.
  - Relevance: an alternative to Thompson/PikeVM for finding the same match boundaries as a backtracker, plus a 3-phase scheme: find the end, then reverse to find the start, then extract groups. — [Open PDF (MSR)](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf), [Tech report MSR-TR-2023-15](https://microsoft.com/en-us/research/uploads/prod/2023/04/MSR-TR-2023-15.pdf), [PLDI page](https://pldi23.sigplan.org/details/pldi-2023-pldi/43/Derivative-Based-Nonbacktracking-Real-World-Regex-Matching-with-Backtracking-Semantic)
- **Varatalu, Veanes & Ernits, "RE#: High Performance Derivative-Based Regex Matching with Intersection, Complement, and Restricted Lookarounds"**, PACMPL 9(POPL), 2025, DOI 10.1145/3704837. It uses symbolic derivatives with no backtracking and supports complement, intersection and restricted lookarounds. The authors claim input-linear complexity and report it is ">71% faster than the next fastest regex engine in Rust on the baseline" (the authors' own benchmark).
  - Relevance: a direct competitor and benchmark for a Rust engine, with a model for adding `&`/`~` operators. — [arXiv 2407.20479](https://arxiv.org/abs/2407.20479), [POPL page](https://popl25.sigplan.org/details/POPL-2025-popl-research-papers/1/RE-High-Performance-Derivative-Based-Regex-Matching-with-Intersection-Complement-a), [MSR page](https://www.microsoft.com/en-us/research/publication/re-high-performance-derivative-based-regex-matching-with-intersection-complement-and-restricted-lookarounds/)
- **Varatalu, Veanes & Ernits, "Derivative Based Extended Regular Expression Matching Supporting Intersection, Complement and Lookarounds"**, arXiv:2309.14401 (Sep 2023). This is the precursor to RE#. — [arXiv](https://arxiv.org/abs/2309.14401)

**Lookaround in linear time**
- **Mamouras & Chattopadhyay, "Efficient Matching of Regular Expressions with Lookaround Assertions"**, POPL 2024 (PACMPL 8). It runs in O(m·n) using bottom-up evaluation of lookarounds and avoids PCRE's super-linear blowup; the authors report it is several times faster.
  - Rust regex maintainers discussed it and noted that it loses streaming when lookahead and lookbehind are combined. — [POPL page](https://popl24.sigplan.org/details/POPL-2024-popl-research-papers/94/Efficient-Matching-of-Regular-Expressions-with-Lookaround-Assertions), [Draft PDF](https://kmamouras.github.io/papers/regex-lookaround-draft-POPL'24.pdf), [rust-lang/regex discussion #1153](https://redirect.github.com/rust-lang/regex/discussions/1153)
- **Chattopadhyay, Li & Mamouras, "Verified and Efficient Matching of Regular Expressions with Lookaround"**, CPP 2025. It is a Coq-verified algorithm, linear in both regex size and input length, building on the POPL'24 work. — [CPP page](https://popl25.sigplan.org/details/CPP-2025-papers/14/Verified-and-Efficient-Matching-of-Regular-Expressions-with-Lookaround), [Code](https://github.com/Agnishom/lregex/)
- **Barrière & Pit-Claudel, "Linear Matching of JavaScript Regular Expressions"**, PLDI 2024, DOI 10.1145/3656431. It gives linear-time matching for JS regexes, including lookarounds and JS-specific capture semantics, implemented in the OCaml engine RegElk. Some of its techniques reportedly fed into V8's linear engine **[not re-verified this session]**. — [arXiv 2311.17620](https://arxiv.org/abs/2311.17620), [Code](https://github.com/LindenRegex/RegElk), [Linden project page](https://lin.den.re/)

**Mechanized semantics (JS/ECMAScript), PikeVM proof**
- **De Santo, Barrière & Pit-Claudel, "A Coq Mechanization of JavaScript Regular Expression Semantics"** (Warblre), ICFP 2024, DOI 10.1145/3674666. It is a line-by-line Coq/Rocq translation of more than 33 pages of ECMAScript 2023 regex pseudocode. The authors prove it terminates and never hits an assertion failure, and the extracted engine passes the relevant Test262 tests. — [arXiv 2403.11919](https://arxiv.org/abs/2403.11919), [Code](https://github.com/LindenRegex/Warblre)
- **Barrière, Deng & Pit-Claudel, "Formal Verification for JavaScript Regular Expressions: a Proven Mechanized Semantics and its Applications"**, POPL 2026, DOI 10.1145/3776710; Distinguished Paper Award. Its semantics keeps the full backtracking tree of all matches and their priorities, and is proven equivalent to Warblre. It contains the **first formal proof of the PikeVM algorithm** and defines contextual equivalence for modern regexes, used to check rewrites.
  - Relevance: this is highly relevant if your engine is PikeVM-based. It precisely states the invariants your capture-priority handling must preserve. — [arXiv 2507.13091](https://arxiv.org/abs/2507.13091), [POPL page](https://popl26.sigplan.org/details/POPL-2026-popl-research-papers/70/Formal-Verification-for-JavaScript-Regular-Expressions-A-Proven-Mechanized-Semantics), [Rocq code](https://github.com/LindenRegex/Linden)
- The same group's tools page lists **captureless linear-time lookbehinds** for RE2 (fork) and rust-lang/regex (PR #1266, blog "Adding lookbehinds to rust-lang/regex"). It also lists an MSc thesis "Verification of Realistic Regex Matching" (Wojnarowski, 2026). — [Linden project page](https://lin.den.re/), [rust-lang/regex PR #1266](https://github.com/rust-lang/regex/pull/1266)

**Regex in symbolic execution / SMT**
- **Loring, Mitchell & Kinder, "Sound Regular Expression Semantics for Dynamic Symbolic Execution of JavaScript"**, PLDI 2019 **[not re-verified this session]**. It models captures, backreferences, lookarounds and greedy/lazy quantifiers for the ExpoSE symbolic executor using refinement over SMT string constraints. Its successor in spirit is the OSTRICH work below.
- **Stanford, Veanes & Bjørner, "Symbolic Boolean Derivatives for Efficiently Solving Extended Regular Expression Constraints"**, PLDI 2021, DOI 10.1145/3453483.3454066. It is a derivative theory for symbolic *extended* regexes (with complement and intersection) using symbolic conditionals, implemented in Z3. Its regex SMT benchmarks are public. — [MSR page (open PDF)](https://www.microsoft.com/en-us/research/publication/symbolic-boolean-derivatives-for-efficiently-solving-extended-regular-expression-constraints/), [Unpaywall](https://unpaywall.org/10.1145%2F3453483.3454066), [Benchmarks](https://github-redirect.dependabot.com/cdstanford/regex-smt-benchmarks)
- **Chen, Flores-Lamas, Hague, Han, Hu, Kan, Lin, Rümmer & Wu, "Solving String Constraints with Regex-Dependent Functions through Transducers with Priorities and Variables"**, PACMPL 6(POPL), Art. 45, 2022. It introduces *prioritized streaming string transducers* (PSST) to model capture groups, backreference-style replacement and match priority, and is implemented in OSTRICH (`str.replace_cg`, `str.replace_cg_all`).
  - Relevance: it formalizes the leftmost/priority capture semantics as a transducer, which you could use as a cross-check oracle. — [Long version PDF](https://www.cs.rhul.ac.uk/home/uxac009/files/papers/popl2022-long.pdf), [OSTRICH](https://scaladex.scala-lang.org/uuverifiers/ostrich), [MOSCA'23 slides](https://mosca2023.github.io/slides/mosca23_slides_matthew.html)

### Inferences
- The "find end with forward DFA, find start with reverse DFA, then run captures on the narrowed span" strategy (used by RE2 and rust-regex) has a rigorous published counterpart only in the derivative world (Moseley et al. PLDI 2023 proves backtracking semantics are preserved). For Thompson/PikeVM, the POPL 2026 Linden paper is now the closest formal treatment.
- TDFA ("A closer look at TDFA") is the most mature route to capture extraction at DFA speed. Its multi-pass variant suits a lazy/hybrid DFA design like regex-automata's.

### Gaps
- Mamouras's streaming-regex/streaming-capture papers beyond lookaround (e.g., StreamQL-style work) were not searched in this session.
- I found no paper that specifically formalizes rust-regex/RE2's reverse-DFA match-start technique; it is documented mainly in Russ Cox's articles and Andrew Gallant's blog ("Regex engine internals as a library", 2023) **[not re-verified]**.
- I did not search for Loring follow-ups after 2019 beyond OSTRICH.

## Q2. Earliest / all / overlapping matches and "leftmost" search semantics

### Takeaway
Two formal sources speak to how match boundaries are defined: the derivative engines (PLDI 2023 / RE#) for leftmost-backtracking boundaries, and the Linden backtracking-tree semantics for the full priority order. The "all matches" problem has its own database-theory literature (REmatch, enumeration), which I covered only lightly.

### Cited Findings
- Moseley et al. (PLDI 2023) formally prove that their derivative matcher yields backtracking-semantics match boundaries, including with anchors. — [MSR-TR-2023-15](https://microsoft.com/en-us/research/uploads/prod/2023/04/MSR-TR-2023-15.pdf)
- The Linden POPL 2026 semantics keeps "a full backtracking tree of all possible matches and their priorities" rather than just the top-priority match. That makes it a formal base for defining leftmost-first, earliest and all-matches variants. — [arXiv 2507.13091](https://arxiv.org/abs/2507.13091)
- **REmatch: a regex engine for finding all matches** (Riveros, Van Sint Jan, Vrgoč; VLDB 2023 **[venue not re-verified]**). It enumerates *all* (including overlapping) spans matching a regex with output-linear delay, based on document-spanner theory. — [index page](https://rmarcus.info/dbscholar/papers/h3e895fda3ff98a2b)
- The search also surfaced "Optimal Enumeration of Regular Pattern Matches" as related enumeration work; authors and venue were not verified. — [index page](https://rmarcus.info/dbscholar/papers/h71a9cb819d1ccaa3)

### Inferences
- If you need an "all overlapping matches" API (as Aho–Corasick crates offer for literals), the document-spanner and REmatch literature is the right theory. Leftmost-first and leftmost-longest are covered by the tagged-automata and derivative work above.

### Gaps
- I did not verify REmatch's exact venue or the authors of the enumeration paper.
- I found no 2020–2026 academic paper dedicated to "earliest match" semantics as in Hyperscan or regex-automata's `earliest` mode.

## Q3. Fine-grained complexity / theory 2020–2026

### Takeaway
The O(nm) barrier for regex matching is SETH-tight (Backurs–Indyk 2016). Bounded-depth pattern types have a full dichotomy (Bringmann–Grønlund–Larsen 2017; Schepper 2020). Sparse matching gets O(Δ log log(nm/Δ) + n + m) with a matching SETH bound (Bille–Gørtz SODA 2024). The 2025–2026 frontier is extensions: backreferences, intersection/complement, JS regex (PSPACE-hard), and subsequence-style matching.

### Cited Findings
- **Bille & Gørtz, "Sparse Regular Expression Matching"**, SODA 2024, pp. 3354–3375. It defines a density parameter Δ (the amount of nondeterminism in the state-set simulation) and gives O(Δ log log(nm/Δ) + n + m) time. It also proves no O(Δ^(1−ε)) algorithm exists under SETH. Technique: a linear-space position-automaton representation plus batched predecessor.
  - Relevance: it gives a theory explanation for why sparse-state-set PikeVM/bitset simulations are fast in practice. — [arXiv 1907.04752](https://arxiv.org/abs/1907.04752v7), [DTU Orbit](https://orbit.dtu.dk/en/publications/sparse-regular-expression-matching/)
- **Bille, Gørtz & Jessen, "Improved Regular Expression Matching with Simple Backreferences"**, arXiv:2609.16914 (Sep 2026). It gives improved algorithms for regexes with simple backreferences. — [arXiv](https://arxiv.org/abs/2609.16914)
- **Schepper, "Fine-Grained Complexity of Regular Expression Pattern Matching and Membership"**, ESA 2020 (LIPIcs vol. 173, open access), arXiv:2008.02769. It refines the Backurs–Indyk / BGL classification: few pattern types allow super-polylog speedups; the rest can only improve by a constant number of log factors under the Formula-SAT hypothesis. — [arXiv](https://arxiv.org/pdf/2008.02769), [LIPIcs PDF](https://drops.dagstuhl.de/storage/00lipics/lipics-vol173-esa2020/LIPIcs.ESA.2020.80/LIPIcs.ESA.2020.80.pdf)
- **Bringmann, Grønlund & Larsen, "A Dichotomy for Regular Expression Membership Testing"**, FOCS 2017 (seminal, pre-2020). It gives matching upper and lower bounds for every bounded-depth homogeneous regex type. Word Break is the intermediate case. — [arXiv 1611.00918](https://arxiv.org/abs/1611.00918v2), [FOCS PDF](https://ieee-focs.org/FOCS-2017-Papers/3464a307.pdf)
- **Backurs & Indyk, "Which regular expression patterns are hard to match?"**, FOCS 2016 (seminal). Under SETH, there is no O((nm)^(1−ε)) algorithm for regex matching. (Summarized in Bille–Gørtz's abstract.) — [arXiv 1907.04752](https://arxiv.org/abs/1907.04752)
- **Gibney & Thankachan, "Text Indexing for Regular Expression Matching"**, *Algorithms* 14(5):133, 2021 (MDPI, open access). Under the OMv conjecture, even polynomial preprocessing of the text cannot give strongly sublinear regex query time. The exact bound in the search snippet was garbled, so verify it in the PDF. — [PDF](https://res.mdpi.com/d_attachment/algorithms/algorithms-14-00133/article_deploy/algorithms-14-00133.pdf), [DOAJ](https://doaj.org/article/2fcf1a1482f44c4ca2f943e4b6f59620), [DBLP](https://dblp.uni-trier.de/rec/journals/algorithms/GibneyT21.html)
- **Amarilli, Dudek, Manea, Ringleb & Schmid, "Linear Time Subsequence and Supersequence Regex Matching"**, arXiv:2504.16288 (Apr 2025). Subsequence and supersequence matching variants are solvable in linear time. For infix, prefix, extension and left-extension variants, an O((|w|m)^(1−ε)) algorithm would refute SETH. — [arXiv](https://arxiv.org/abs/2504.16288)
- **Nogami, Nakamura & Terauchi, "Hardness of Regular Expression Matching with Extensions"**, arXiv:2601.03020 (Jan 2026). It proves fine-grained lower bounds of the form n^(c−ε)·g(m) for regexes with backreferences, intersection and complement.
  - Relevance: it quantifies the cost of adding `&`/`~` (as RE# does) or backrefs. — [arXiv](https://arxiv.org/abs/2601.03020)
- **Kumabe & Uezato, "On the Complexity of the Matching Problem of Regular Expressions with Backreferences"**, arXiv:2605.07289 (May 2026). For k-use backreference patterns, there is no O(n^(2k−ε)) algorithm under SETH, and parameterized hardness is strengthened to W[2]. — [arXiv](https://arxiv.org/abs/2605.07289)
- **Deng, Barrière & Pit-Claudel, "On the computational complexity of JavaScript regex matching"**, arXiv:2603.26139 (Mar 2026). JS regex matching is PSPACE-hard even without negative lookarounds, and OptP-hard. Deng's poster on this work won the ICFP'26 SRC gold medal. — [arXiv](https://arxiv.org/abs/2603.26139), [Linden page](https://lin.den.re/)
- "Subsequences With Gap Constraints: Complexity Bounds for Matching and Analysis Problems", arXiv:2206.13896 (2022), is related bounded-gap matching work. I did not read it in detail.
  - Relevance: this repo has a bounded-gap engine design in progress (recent commits). — [arXiv](https://arxiv.org/pdf/2206.13896)
- Also surfaced: "Fine-Grained Complexity of Regular Path Queries", arXiv:2101.01945 (2021), the graph analogue of these results. — [arXiv](https://arxiv.org/pdf/2101.01945)

### Inferences
- No algorithm beats O(nm) in general, and none will unless SETH falls. Practical wins come from parameters: sparsity Δ (Bille–Gørtz), log-factor bit-parallel speedups (Schepper's classification says only constant-many log factors are possible for most types), or restricted classes (linear-time subsequence variants).
- Lower bounds for intersection/complement and for backrefs (2026) support keeping such features out of the linear-time core, or restricting them as RE# does.

### Gaps
- I did not locate a specific 2020–2026 paper on "regular expression matching with small space" or on regex matching in grammar-compressed text. Searches did not surface them; treat as unverified.
- I found no specific Abboud-authored regex paper for 2020–2026 in this session.
- Gibney–Thankachan's exact lower-bound statement needs a check against the PDF.

## Q4. Regex in the LLM era (constrained decoding)

### Takeaway
Constrained decoding compiles a regex to a DFA and precomputes, per DFA state, which vocabulary tokens are allowed (Outlines). XGrammar extends this to context-free grammars with a byte-level pushdown automaton and caches masks for tokens that don't depend on context. Both are effectively "regex/DFA over a token alphabet" problems where DFA construction speed and state counts matter.

### Cited Findings
- **Willard & Louf, "Efficient Guided Generation for Large Language Models"**, arXiv:2307.09702 (Jul 2023). It frames guided generation as FSM state transitions, builds an index from DFA states to the vocabulary for regex and CFG constraints, and is implemented in Outlines. — [arXiv](https://arxiv.org/abs/2307.09702)
- **Dong, Ruan et al., "XGrammar: Flexible and Efficient Structured Generation Engine for Large Language Models"**, arXiv:2411.15100 (Nov 2024, v3 May 2025). It uses a byte-level PDA for CFGs, precomputes masks for context-independent tokens in an adaptive cache, and checks the remaining tokens at runtime. The authors report up to 80x end-to-end speedup on H100 for Llama-3.1 (other summaries cite up to 100x, probably a different metric). — [arXiv](https://arxiv.org/abs/2411.15100)

### Inferences
- A Rust regex crate that exposes DFA states and transitions (as regex-automata does) is directly reusable for token-mask indexing. The Rust `outlines-core` crate is evidence of this **[not verified this session]**.

### Gaps
- Regex synthesis/learning (2023–2026) and token-level automata papers beyond these two were not searched. Coverage is deliberately brief.

## Q5. Practical access to papers (legal options)

### Takeaway
As of 1 January 2026, the whole ACM Digital Library (including PACMPL: PLDI/POPL/OOPSLA/ICFP) is free to read in the "DL Basic" tier. Theory papers are almost always on arXiv, and ESA/ICALP/CPM/STACS papers are in open-access LIPIcs. SIAM (SODA) and Wiley/Springer remain paywalled, so use arXiv preprints, author pages or Unpaywall for those.

### Cited Findings
- From 1 January 2026, ACM is a fully open-access publisher. DL content is free without a subscription via the free "DL Basic" tier, and "DL Premium" adds paid value-added tools. ACM Open institutions get Premium and APC-free publishing. ACM's own earlier pages said the transition would complete by the end of 2025. — [ACM Open Access](https://acm.org/publications/openaccess), [SMU Library note](https://library.smu.edu.sg/topics-insights/acm-digital-library-now-fully-open-access), [InfoToday](https://www.infotoday.eu/Articles/News/Featured-News/The-Association-for-Computing-Machinery-ACM-publications-are-now-fully-Open-Access-173312.aspx), [ACM libraries: subscriptions](https://libraries.acm.org/subscriptions-access)
- So DOIs 10.1145/3591262 (PLDI'23), 10.1145/3704837 (POPL'25), 10.1145/3656431 (PLDI'24), 10.1145/3674666 (ICFP'24) and 10.1145/3776710 (POPL'26) should all be readable for free at dl.acm.org. PACMPL papers were largely gold open access even before 2026 (inference from the ACM OA policy pages above).
- LIPIcs (Dagstuhl DROPS) is fully open access. Example: Schepper's ESA 2020 paper PDF. — [LIPIcs PDF](https://drops.dagstuhl.de/storage/00lipics/lipics-vol173-esa2020/LIPIcs.ESA.2020.80/LIPIcs.ESA.2020.80.pdf)
- Unpaywall resolves a DOI to a legal OA copy. — [Unpaywall example](https://unpaywall.org/10.1145%2F3453483.3454066)
- Author and lab pages often host final PDFs: Microsoft Research (Veanes et al.), Mamouras's homepage, the Linden project, RHUL (Hague), and DTU Orbit (Bille–Gørtz). — [MSR PLDI'23 PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf), [Mamouras draft](https://kmamouras.github.io/papers/regex-lookaround-draft-POPL'24.pdf), [Linden](https://lin.den.re/), [RHUL PDF](https://www.cs.rhul.ac.uk/home/uxac009/files/papers/popl2022-long.pdf), [DTU Orbit](https://orbit.dtu.dk/en/publications/sparse-regular-expression-matching/)
- Paywalled examples from this list:
  - Borsotti & Trofimovich, SPE (Wiley), DOI 10.1002/spe.2881.
  - Bille–Gørtz, SODA 2024 (SIAM); use arXiv 1907.04752 instead.
  - Berglund et al., ICTAC 2018 (Springer LNCS); use the KCL mirror or the CSIR repository instead.

### Inferences
- Recommended lookup order: arXiv, then the ACM DL (free since 2026) or LIPIcs, then Unpaywall on the DOI, then the author's homepage or institutional repository (DTU Orbit, EPFL Infoscience, CSIR ResearchSpace), and only then free-account sites (ResearchGate / Academia.edu, where authors self-upload). DBLP (dblp.org) is the best index for complete author publication lists, and the Semantic Scholar API is useful for citation graphs (rate-limited; it returned HTTP 429 here).
- For discovery, a forward-citation search on Backurs–Indyk 2016, Bille–Gørtz 2024 and Moseley et al. 2023 via Semantic Scholar or Google Scholar would find further 2025–2026 follow-ups.

### Gaps
- I did not verify current free-registration terms for IEEE Xplore or ResearchGate. They generally require a free account, and access depends on author uploads.
- I did not confirm from the PDF whether the CDL "ACM Open License 2026–2028" document changes anything for non-affiliated readers. — [CDL license PDF](https://cdlib.org/services-groups/collections/licensed_resources/redacted_licenses/ACM_Open_License_2026-2028_Redacted.pdf)
