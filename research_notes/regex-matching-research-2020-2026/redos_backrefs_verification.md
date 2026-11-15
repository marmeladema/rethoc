# ReDoS, Non-Regular Extensions (Backreferences, Lookarounds, Atomic Groups), and Formal Semantics/Verification of Regex Engines (2020–2026)

Method note: roughly 22 search/fetch calls. Each paper below was checked against an arXiv page, publisher or venue page, DBLP, or the authors' project page, unless it is marked UNVERIFIED. Bibliographic details for items cited only from memory are listed under Gaps.

## Q1. ReDoS defenses inside engines (memoization, linear fallbacks, derivative engines)

### Takeaway
Memoizing backtracking (Davis et al. 2021) is now in production (Ruby 3.2) and has been extended soundly to lookaround and atomic groups (Fujinami & Hasuo, ESOP 2024). The other main defense family is non-backtracking engines that keep backtracking (leftmost-greedy) semantics: .NET 7 NonBacktracking (PLDI 2023), RE# (POPL 2025), V8's linear engine plus RegElk's linear lookaround algorithms (PLDI 2024), and the Rust lookbehind fork. Bounded repetition (counters) remains a known ReDoS weakness even for non-backtracking engines.

### Cited Findings
- **Davis, Servant, Lee. "Using Selective Memoization to Defeat Regular Expression Denial of Service (ReDoS)." IEEE S&P 2021, DOI 10.1109/SP40001.2021.00032.** Open PDF: [Stony Brook PDF](https://www3.cs.stonybrook.edu/~dongyoon/papers/SP-21-Memoization.pdf). Publisher page (paywalled): [IEEE CSDL](https://computer.org/csdl/proceedings-article/sp/2021/893400a543/1oak988ThvO).
  - Problem: up to 10% of regexes are super-linear in typical engines. Full memoization of (NFA state, position) pairs is sound but uses too much space.
  - Fix: memoize only at selected vertices (in-degree > 1, or ancestors of back-edges), and compress the memo table with run-length encoding.
  - Results: memo space drops by an order of magnitude for the median regex, and RLE makes it constant for 90% of regexes. Matching is linear in |w| on the largest available corpus of super-linear regexes.
  - The paper also discusses extending the approach to backreferences and lookaround, with weaker guarantees.
  - Relevance: this is the template for adding a memo table to a Rust backtracker such as regex-automata's BoundedBacktracker, which already uses a visited bitset over (state, position).
  - Source: [Medium summary by Davis](https://davisjam.medium.com/using-selective-memoization-to-defeat-regular-expression-denial-of-service-f7acbbd34792).
- **Fujinami, Hasuo. "Efficient Matching with Memoization for Regexes with Look-around and Atomic Grouping." ESOP 2024, LNCS, pp. 90–118, DOI 10.1007/978-3-031-57267-8_4.** Open access: [arXiv:2401.12639](https://arxiv.org/abs/2401.12639). Also [Springer](https://link.springer.com/chapter/10.1007/978-3-031-57267-8_4) and [DBLP](https://dblp.dagstuhl.de/rec/conf/esop/FujinamiH24.html).
  - Extends Davis-style memoization so backtracking stays linear in the input with look-around and atomic grouping.
  - Look-around: the memo table records successes as well as failures, M : P × N ⇀ {Failure, Success}.
  - Atomic grouping: failures are recorded together with the atomic-group nesting depth, because naive memoization conflates failures at different depths.
  - The matcher works on NFAs with sub-automata (transitions labeled by nested NFAs).
  - In a survey of 537,806 real-world regexes, 17,167 (3.2%) use look-around or atomic grouping.
  - Relevance: the most directly usable algorithm for a Rust backtracker that supports `(?=…)`, `(?!…)` and `(?>…)` while keeping a linear guarantee. Possessive quantifiers desugar to atomic groups.
  - Ruby follow-up: Fujinami opened [Ruby #19725](https://bugs.ruby-lang.org/issues/19725) to extend Ruby's match cache to look-ahead and atomic groups.
- **Ruby 3.2 (Onigmo fork) cache-based optimization, [Feature #19104](https://redmine.ruby-lang.org/issues/19104), by Hiroya Fujinami (makenowjust).**
  - Records each (position, state) pair reached so that work is never repeated, citing Davis et al. 2021.
  - Benchmark: `/^(a|a)*$/` on `"a"*28+"b"` took 8.6 s before the patch and about 0.01 s after.
  - The cache table is proportional to (#cache points × input length). Regexes with too many cache points are excluded.
  - The optimization turns on lazily, once backtracking reaches about the input length.
  - About 8% of regexes in stdlib, WEBrick and Rails were not optimizable.
  - Ruby 3.2 also added `Regexp.linear_time?`.
  - Talk: [RubyKaigi 2023](https://rubykaigi.org/2023/presentations/makenowjust.html).
- **V8 experimental non-backtracking engine (v8.8+, 2021).** Source: [V8 blog](https://v8.dev/blog/non-backtracking-regexp).
  - Linear in subject length, based on the RE2/Rust-regex automata approach.
  - Enabled by `--enable-experimental-regexp-engine` (non-standard `/l` flag), or used as a fallback via `--enable-experimental-regexp-engine-on-excessive-backtracks`, with threshold `--regexp-backtracks-before-fallback` (default 50,000).
  - Does not support backreferences or lookarounds.
  - A TC39 `/l` linear-flag proposal exists only as a discussion: [es.discourse](https://es.discourse.group/t/proposal-regular-expression-linear-time-flag/2491).
  - Relevance: the fallback-on-backtrack-budget design is a cheap hybrid for Rust engines that offer both backtracking and automata paths.
- **Barrière, Pit-Claudel. "Linear Matching of JavaScript Regular Expressions." PLDI 2024, PACMPL 8(PLDI) Art. 201, DOI 10.1145/3656431.** Open access: [arXiv:2311.17620](https://arxiv.org/abs/2311.17620). Also [PLDI page](https://pldi24.sigplan.org/details/pldi-2024-papers/55/Linear-Matching-of-JavaScript-Regular-Expressions).
  - Identifies a larger linear-time-matchable subset of JS regexes, and finds that some state-of-the-art algorithms were wrong, superlinear or overly restrictive.
  - Gives the first non-backtracking linear-time algorithms for lookarounds: one for captureless lookbehinds in any regex language, and one that uses a JS-specific property to support unrestricted lookaheads and lookbehinds.
  - Also gives regex-size-linear algorithms for capture-group reset and JS quantifier semantics.
  - Prototype engine: [RegElk](https://github.com/LindenRegex/RegElk), an OCaml PikeVM extension with no backreferences ("NP-hard"). Parts were merged into V8's linear engine, per the [Linden project page](https://lin.den.re/).
  - Rust spin-off: a fork of rust-lang/regex that adds captureless lookbehinds in linear time. See [PR #1266](https://github.com/rust-lang/regex/pull/1266) and the [EPFL blog](https://systemf.epfl.ch/blog/rust-regex-lookbehinds/). An RE2 fork does the same ([re2-lookbehinds](https://github.com/LindenRegex/re2-lookbehinds)).
- **Moseley, Nishio, Perez Rodriguez, Saarikivi, Toub, Veanes, Wan, Xu. "Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics." PLDI 2023, PACMPL 7(PLDI):1026–1049, DOI 10.1145/3591262.** Open PDF: [MSR PDF](https://microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf). Extended version: [MSR-TR-2023-15](https://microsoft.com/en-us/research/uploads/prod/2023/04/MSR-TR-2023-15.pdf).
  - Symbolic-derivative theory that supports anchors and counting while preserving backtracking (PCRE-like leftmost, greedy/lazy) match semantics. It can be extended with lookarounds.
  - Shipped as .NET 7's `RegexOptions.NonBacktracking`, with what the authors describe as the first formal correctness proof for an industrial regex matcher.
  - Relevance: shows that a derivative/lazy-DFA engine can reproduce backtracking match results, which is the main obstacle to dropping a backtracker.
- **Varatalu, Veanes, Ernits. "RE#: High Performance Derivative-Based Regex Matching with Intersection, Complement, and Restricted Lookarounds." POPL 2025, PACMPL 9(POPL), DOI 10.1145/3704837.** Open access: [arXiv:2407.20479](https://arxiv.org/abs/2407.20479). Earlier version: [arXiv:2309.14401](https://arxiv.org/abs/2309.14401). Also [POPL page](https://popl25.sigplan.org/details/POPL-2025-popl-research-papers/1/RE-High-Performance-Derivative-Based-Regex-Matching-with-Intersection-Complement-a).
  - Non-backtracking, input-linear derivative engine with `&`, `~` and restricted lookarounds.
  - Reported to be over 71% faster than the next fastest Rust regex engine on baseline benchmarks, and often orders of magnitude faster on extended benchmarks.
  - Relevance: a direct competitor and benchmark target for a Rust engine.
- **Turoňová, Holík, Homoliak, Lengál, Veanes, Vojnar. "Counting in Regexes Considered Harmful: Exposing ReDoS Vulnerability of Nonbacktracking Matchers." USENIX Security 2022, pp. 4165–4182.** Open access: [USENIX page](https://usenix.org/conference/usenixsecurity22/presentation/turonova) and [author PDF](https://ondrik.github.io/publications/usenix-security22-counting-redos.pdf).
  - Bounded repetition (`x{n,m}`) causes state blow-up and ReDoS even in non-backtracking matchers (DFA/NFA simulation), including Hyperscan in SNORT and NVIDIA BlueField-2 hardware.
  - Provides a generator for attack inputs ([GadgetCA](https://fit.vut.cz/research/group/verifit/public/tools/gadgetca)).
  - Relevance: a Rust linear engine still needs counter-aware handling or limits on large `{n,m}` (cf. this repo's bounded-gap / CounterInstance work).
- **Bhuiyan, Çakar, Burmane, Davis, Staicu. "SoK: A Literature and Engineering Review of Regular Expression Denial of Service (ReDoS)." ASIA CCS 2025, pp. 1659–1675.** Open access: [arXiv:2406.11618](https://arxiv.org/abs/2406.11618).
  - Systematic review of ReDoS detection, prevention and mitigation, combined with an engineering review of the defenses in current engines.
  - Finds that many mainstream engines have added partial or full ReDoS defenses, which may make older threat models obsolete.
  - Finds that almost no studies show weaponization against real systems.
  - The best single overview of the Java, .NET, Ruby, V8 and other engine changes. Use it for per-engine details.

### Inferences
- For a Rust engine with a backtracking path, the cheapest proven ReDoS defense is a (selective) memo table keyed on (state, position). Fujinami & Hasuo show how to make it sound for lookaround (memoize success and failure) and atomic/possessive groups (key on atomic depth). Backreferences remain outside any linear guarantee.
- A hybrid "backtrack with budget, then fall back to PikeVM/lazy DFA" (the V8 approach) is cheap when the regex has no backrefs or lookarounds. The RegElk and Rust lookbehind work enlarge the subset that can fall back.

### Gaps
- I did not verify specific Java (JDK 9+ backtracking changes) or .NET 7 timeout/backtracking changes from primary sources beyond the .NET NonBacktracking paper. The SoK (arXiv:2406.11618) is the place to get them.
- I found no 2025–2026 follow-up to Fujinami & Hasuo on memoization for backreferences. Whether Ruby #19725 was merged was not checked.

## Q2. ReDoS detection/repair tools and empirical studies

### Takeaway
Detection tools from 2021–2026 are hybrid static/dynamic: Revealer, ReDoSHunter, RENGAR, and PUFFERDOS (S&P 2026). Repair has moved from rule- or synthesis-based tools (RegexScalpel, VulcanBoost) to LLM-assisted, localization-guided repair (EACL 2026). Empirical work from 2024–2026 finds that detectors disagree substantially, that LLM-generated regexes are often ReDoS-prone, and that ReDoS CVEs are rising.

### Cited Findings
- **Liu, Zhang, Meng. "Revealer: Detecting and Exploiting Regular Expression Denial-of-Service Vulnerabilities." IEEE S&P 2021.** Open PDF: [CUHK PDF](https://seclab.cse.cuhk.edu.hk/papers/sp21_redos.pdf).
  - Hybrid static/dynamic: locates vulnerable structures, then generates attack strings.
  - Also handles extended features such as lookaround and backreferences (from my memory of the paper, not verified in fetched text).
- **Li, Chen, Cao, Xu, Peng, Chen, Chen, Cheung. "ReDoSHunter: A Combined Static and Dynamic Approach for Regular Expression DoS Detection." USENIX Security 2021.** Venue and authors come from the reference list in [arXiv:2010.12450 / related refs](https://arxiv.org/pdf/2010.12450) and a [Techxplore summary](https://techxplore.com/news/2021-08-high-performance-tool-redos-vulnerability.pdf). Pinpoints multiple root causes per regex.
- **Li et al. "RegexScalpel: ReDoS Defense by Localize-and-Fix." USENIX Security 2022, pp. 4183–4200.** Open access: [USENIX](https://www.usenix.org/conference/usenixsecurity22/presentation/li-yeting).
  - Localize-and-fix repair with a 98.88% successful repair ratio, against 21.20% for the best prior work.
- **RENGAR: "Effective ReDoS Detection by Principled Vulnerability Modeling and Exploit Generation." IEEE S&P 2023** (venue inferred from the PDF filename "2023-ieeesp-rengar"). Open PDF: [author site](https://cenzhang.github.io/files/pubs/2023-ieeesp-rengar.pdf). Also [AMiner](https://aminer.org/pub/64c78b9f3fda6d7f06db9b15/effective-redos-detection-by-principled-vulnerability-modeling-and-exploit-generation).
  - Hybrid detector with new vulnerability modeling and a "disturbance-free" attack-string generator.
  - Compared against nine detectors on seven datasets, and found 69 zero-day vulnerabilities (21 CVEs).
  - Note: the brief said "USENIX Sec 2023", but the evidence points to IEEE S&P 2023.
- **Chida, Terauchi. "Repairing DoS Vulnerability of Real-World Regexes." IEEE S&P 2022** (venue from memory). Open access: [arXiv:2010.12450](https://arxiv.org/abs/2010.12450).
  - Programming-by-example repair that guarantees the output is invulnerable. It handles real-world extensions (lookaround, backreferences) by defining "real-world" ReDoS vulnerability for them.
  - The search snippet confirms the "PBE repair ... guarantees to generate only invulnerable regexes" description.
  - Relevance: its vulnerability definition for backtracking-with-extensions is a useful spec for engine-side linters.
- **Li et al. "VulcanBoost: Boosting ReDoS Fixes through Symbolic Representation and Feature Normalization." USENIX Security 2025.** Source: [USENIX](https://www.usenix.org/conference/usenixsecurity25/presentation/li-yeting). Reports a 93.95% Test Coverage Repair Success Rate on ReDoS-vulnerable NPM regexes.
- **Sung, Hahn, Han. "Repairing Regex Vulnerabilities via Localization-Guided Instructions." EACL 2026 (Long), pp. 8128–8142.** Open access: [arXiv:2510.09037](https://arxiv.org/abs/2510.09037) and [ACL Anthology PDF](https://preview.aclanthology.org/ingest-eacl/2026.eacl-long.379.pdf). Code: [LRR](https://github.com/cdltlehf/lrr).
  - A symbolic module isolates the vulnerable subpattern, and an LLM rewrites only that segment into a semantically equivalent safe form.
  - A snippet mentions a +15.4 percentage-point repair-rate gain; I did not confirm which baseline it is against.
- **Xu, Ding, Cheng, Li, Sun, Turnbull, Kan, Ma. "PUFFERDOS: Efficient and Effective Attack String Generation for ReDoS Vulnerabilities." IEEE S&P 2026 (accepted).** Open access: [arXiv:2606.19654](https://arxiv.org/abs/2606.19654).
  - Argues that prior tools' attack strings assume unrealistic input lengths and are not validated at the program level.
  - Defines three vulnerable patterns (via observation and formal verification), synthesizes strings under realistic length budgets, and validates them with ReDoS-specific compositional concolic execution.
- **Mahassadi, Khoury, Vallé, Hamou-Lhadj. "An Empirical Analysis of ReDoS Vulnerabilities and ReDoS Detection Tools." arXiv:2609.10294 (Sept 2026, preprint).** Source: [arXiv](https://arxiv.org/abs/2609.10294).
  - Evaluates five detectors and one repair tool on three datasets, and analyzes all ReDoS CVEs in NVD.
  - ReDoS vulnerabilities are becoming more prevalent and are much more likely to be exploited than other vulnerabilities. Detectors disagree substantially.
- **Siddiq, Zhang, Roney, Santos. "Re(gEx|DoS)Eval: Evaluating Generated Regular Expressions and their Proneness to DoS Attacks." ICSE-NIER 2024.** Open PDF: [preprint](https://joannacss.github.io/preprints/icse_nier24-preprint.pdf). Also [ICSE page](https://conf.researchr.org/details/icse-2024/icse-2024-new-ideas-and-emerging-results/1/Re-gEx-DoS-Eval-Evaluating-Generated-Regular-Expressions-and-their-Proneness-to-DoS-) and [code](https://github.com/s2e-lab/RegexEval).
  - 762 real-user regex prompts, with pass@k and vulnerable@k metrics for LLM-generated regexes.
  - Companion paper: "Understanding Regular Expression Denial of Service (ReDoS): Insights from LLM-Generated Regexes and Developer Forums," ICPC 2024 ([page](https://conf.researchr.org/details/icpc-2024/icpc-2024-research/8/Understanding-Regular-Expression-Denial-of-Service-ReDoS-Insights-from-LLM-Generat)). It finds that LLM-generated regexes mostly have polynomial ReDoS patterns, consistent with real-world data, and that GPT-3.5 was best among the models tested.

### Inferences
- An LLM-written regex is more likely to hit polynomial than exponential blow-up. Linear engines and memoization address both, while backtrack-step limits only bound the damage.
- Detectors disagreeing (2609.10294) is an argument for engine-level guarantees over per-regex linting.

### Gaps
- Pre-2020 seminal tools were not re-verified in this pass:
  - ReScue (Shen et al., ASE 2018)
  - Rexploiter (Wüstholz et al., TACAS 2017)
  - Weideman / Berglund / van der Merwe's ReDoS theory (e.g. "Analyzing matching time behavior of backtracking regular expression matchers by using ambiguity of NFA", CIAA 2016)
  - Davis et al. "The impact of ReDoS in practice" (FSE 2018) and "Why aren't regexes a lingua franca?" (ESEC/FSE 2019)
- No paper titled "Remedy" or "Sound static ReDoS detection" (2020–2026) was found or verified. Treat those as possibly misremembered names.

## Q3. Backreferences (and lookaheads): complexity results and algorithms

### Takeaway
Matching with backreferences is NP-complete in general. 2023–2026 theory has pinned down expressiveness (rewbs ⊊ CSL, ⊂ indexed languages, incomparable with CFL; rewbs + lookaheads = NLOG) and complexity (PSPACE-complete with lookarounds, including real JavaScript). It has also produced polynomial algorithms for restricted shapes, e.g. quadratic time for e₀(e)₁e₁\1e₂. Practical linear engines (V8 linear engine, RegElk, Rust regex) still exclude backreferences.

### Cited Findings
- **Schmid. "Regular Expressions with Backreferences: Polynomial-Time Matching Techniques."** Open access: [arXiv:1903.05896](https://arxiv.org/abs/1903.05896) (journal version in JCSS, from memory).
  - Defines memory-deterministic rewbs, matchable in O(|w|·p(|α|)).
  - Shows that bounding the "active variable degree" gives polynomial matching. General matching is NP-complete even for strongly restricted variants.
  - Relevance: the active-variable-degree parameter can be computed at compile time to choose an algorithm.
- **Schmid's memory automata (MFA) and ref-words; Freydenberger & Schmid's deterministic rewbs and trap-state MFA (TMFA).** Source: [arXiv:1802.01508](https://arxiv.org/abs/1802.01508) (2018, pre-window, seminal). Memory automata are equivalent in power to rewbs. Later work extends MFA to lookaheads: [FSCD 2022 LIPIcs PDF](https://drops.dagstuhl.de/storage/00lipics/lipics-vol228-fscd2022/LIPIcs.FSCD.2022.15/LIPIcs.FSCD.2022.15.pdf), open access.
- **Nogami, Terauchi. "On the Expressive Power of Regular Expressions with Backreferences." MFCS 2023, LIPIcs vol. 272, paper 71.** Open access: [LIPIcs PDF](https://drops.dagstuhl.de/storage/00lipics/lipics-vol272-mfcs2023/LIPIcs.MFCS.2023.71/LIPIcs.MFCS.2023.71.pdf) and [arXiv:2307.08531](https://arxiv.org/abs/2307.08531). Journal version (Information and Computation): [author PDF](https://terauchi.w.waseda.jp/papers/ic-rewb.pdf).
  - Shows that rewbs are contained in the indexed languages.
  - Recaps Berglund & van der Merwe: Schmid's rewbs are a proper subclass of CSL and incomparable with CFL. Under "may repeat labels" syntax, ε- and ∅-semantics for unset references have equal power.
- **Uezato. "Regular Expressions with Backreferences and Lookaheads Capture NLOG." ICALP 2024, LIPIcs vol. 297, paper 155.** Open access: [LIPIcs PDF](https://drops.dagstuhl.de/storage/00lipics/lipics-vol297-icalp2024/LIPIcs.ICALP.2024.155/LIPIcs.ICALP.2024.155.pdf) and [arXiv:2404.17492](https://arxiv.org/abs/2404.17492).
  - Rewbs with (positive and negative) lookaheads recognize exactly NLOG (nondeterministic log-space). Negative lookahead is handled via Immerman–Szelepcsényi.
  - The membership (matching) problem is PSPACE-complete.
  - Note: the author is Yuya Uezato, not Nogami & Terauchi as the brief suggested.
  - Relevance: in the worst case, backrefs combined with lookarounds cannot be matched in polynomial time unless P = PSPACE.
- **Nogami, Terauchi. "Efficient Matching of Some Fundamental Regular Expressions with Backreferences." MFCS 2025, LIPIcs vol. 345, paper 81.** Open access: [arXiv:2504.18247](https://arxiv.org/abs/2504.18247) and [LIPIcs HTML](https://drops.dagstuhl.de/storage/00lipics/lipics-vol345-mfcs2025/html/LIPIcs.MFCS.2025.81/LIPIcs.MFCS.2025.81.html).
  - Matches rewbs of the form e₀(e)₁e₁\1e₂ (pure regexes eᵢ, one capture and one backreference) in time quadratic in |w|, improving on the previous cubic bound.
  - Combines automata techniques (injection, summarization) with stringology (fixed right-maximal repeats and extendable prefixes).
  - Relevance: covers common practical shapes such as `(\w+)\s+\1` (doubled word) and quote matching, as a polynomial special case.
- **Deng, Barrière, Pit-Claudel. "On the Computational Complexity of JavaScript Regex Matching." arXiv:2603.26139 (Mar 2026, rev. Sept 2026, preprint).** Source: [arXiv](https://arxiv.org/abs/2603.26139).
  - Real JS regex matching (with lower-bounded quantifiers expanded) is PSPACE-complete, and remains PSPACE-hard without negative lookarounds.
  - With all lookarounds removed (backrefs only), the parsing problem is OptP-complete.
  - Mechanized in Rocq.

### Inferences
- A practical Rust design should split the work by feature:
  - backref-free regexes go to the linear engines;
  - restricted backref shapes (single backref, Schmid-bounded active variable degree) can get polynomial algorithms;
  - general backrefs need a budgeted or memoized backtracker, accepting the known NP/PSPACE hardness.

### Gaps
- I could not fetch Berglund & van der Merwe's own paper on backreference semantics ("Re-examining regular expressions with backreferences", TCS 2023, from memory). Its claims above are secondhand via Nogami & Terauchi.
- Freydenberger's 2023–2026 papers on backreferences were not surfaced. Search returned only older work.
- I found no production engine (2020–2026) that implements Schmid's or Nogami–Terauchi's polynomial algorithms.

## Q4. Formal semantics, verification, and testing of (backtracking) regex engines

### Takeaway
The EPFL/CNRS group has produced a faithful Rocq mechanization of the ECMAScript regex chapter (Warblre, ICFP 2024). It was followed by a backtracking-tree semantics proven equivalent to it, plus a verified PikeVM-based linear engine (Linden, POPL 2026 Distinguished Paper). The Rice group verified linear lookaround matching in Coq (CPP 2025), and Microsoft verified derivative-based lookaround matching in Lean 4 (CPP 2024). On testing, a 2026 study of 22 engines (1,007 bugs, 156 CVEs) proposes grammar-aware plus metamorphic fuzzing.

### Cited Findings
- **De Santo, Barrière, Pit-Claudel. "A Coq Mechanization of JavaScript Regular Expression Semantics" (Warblre). ICFP 2024, PACMPL Art. 270, DOI 10.1145/3674666.** Open access: [arXiv:2403.11919](https://arxiv.org/abs/2403.11919). Code: [GitHub](https://github.com/LindenRegex/Warblre).
  - Line-by-line faithful mechanization of ECMAScript 2023 regexes, covering backreferences, lookarounds and capture reset.
  - Extractable to OCaml and JS, and passes the relevant Test262 tests.
  - Proves termination and absence of failure.
  - The authors describe it as the first faithful mechanization of an industrial-strength regex language in an interactive theorem prover.
- **Barrière, Deng, Pit-Claudel. "Formal Verification for JavaScript Regular Expressions: a Proven Mechanized Semantics and its Applications" (Linden). POPL 2026, DOI 10.1145/3776710, Distinguished Paper.** Open access: [arXiv:2507.13091](https://arxiv.org/abs/2507.13091). Artifact: [Zenodo](https://zenodo.org/records/17305393). Project page: [lin.den.re](https://lin.den.re/).
  - Backtracking-tree semantics that captures all matches with their priorities, proven equivalent to Warblre.
  - Defines contextual equivalence of modern regexes and uses it to prove or disprove published rewrite rules.
  - Contains the first formal proof of the PikeVM algorithm, and a verified linear engine.
  - Relevance: a reference spec for checking optimizations and rewrites in a Rust PikeVM or backtracker.
- **Chattopadhyay, Li, Mamouras. "Verified and Efficient Matching of Regular Expressions with Lookaround." CPP 2025, pp. 198–213, DOI 10.1145/3703595.3705884.** Sources: [CPP page](https://popl25.sigplan.org/details/CPP-2025-papers/14/Verified-and-Efficient-Matching-of-Regular-Expressions-with-Lookaround), [DBLP](https://dblp.dagstuhl.de/rec/conf/cpp/ChattopadhyayLM25.html), [Coq code](https://github.com/Agnishom/lregex).
  - Coq formalization of regexes with lookarounds and a verified matcher running in O(|r|·|w|). Includes leftmost-longest match finding.
  - Builds on **Chattopadhyay, Li, Mamouras, "Efficient Matching of Regular Expressions with Lookaround Assertions," POPL 2024**, O(m·n) ([POPL page](https://popl24.sigplan.org/details/POPL-2024-popl-research-papers/94/Efficient-Matching-of-Regular-Expressions-with-Lookaround-Assertions); [draft PDF](https://kmamouras.github.io/papers/regex-lookaround-draft-POPL'24.pdf)).
  - Relevance: a linear algorithm for lookarounds that needs no backtracker (POSIX/leftmost-longest flavor).
- **Zhuchko, Veanes, Ebner. "Lean Formalization of Extended Regular Expression Matching with Lookarounds." CPP 2024.** Source: [MSR page](https://www.microsoft.com/en-us/research/?p=1115526). The author list is from memory and was not shown in fetched text.
  - Lean 4 formalization of a symbolic-derivative matcher with intersection, complement and lookarounds, the theory underlying RE# and .NET.
- **Çakar, Lee, Davis. "Towards the Systematic Testing of Regular Expression Engines." arXiv:2603.00311 (Feb 2026).** Open access: [arXiv](https://arxiv.org/abs/2603.00311).
  - Surveys 22 engines, 1,007 engine bugs and 156 CVEs.
  - 11 engines (50%) use differential testing, which is limited by dialect differences (false positives).
  - Proposes ReTest (grammar-aware fuzzing plus metamorphic testing): about 3x the edge coverage of existing fuzzers on PCRE and 3 new memory-safety bugs.
  - Practitioner reference: [secret.club PCRE2/JIT differential fuzzing blog, 2024](https://secret.club/2024/08/23/ring-around-the-regex-2.html).
  - Relevance: metamorphic relations, e.g. equivalence-preserving rewrites (which Linden can certify), give a dialect-independent oracle for a Rust engine's fuzz harness.

### Inferences
- For a Rust engine, a practical verification path is to differentially test against an executable spec: Warblre extracted to OCaml for JS semantics, or the backtracking-tree semantics of Linden. Linden's verified rewrite rules show which optimizations are safe under backtracking semantics.

### Gaps
- "Verbatim" (Egolf et al., verified lexer, CPP 2021/2022) and "CertiRE" were not verified in this pass. I found no 2023–2026 paper by those names.
- No Isabelle regex-matcher paper from 2023–2026 was surfaced.
- No 2020–2025 peer-reviewed differential-testing paper specific to regex engines was found. The 2026 Çakar–Lee–Davis preprint is the main academic source.
