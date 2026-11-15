# Regex matching papers — PDF collection

104 unique PDFs (126 MB), downloaded 2026-11-15 from legal open sources (arXiv, LIPIcs, USENIX, PVLDB, NSF PAR, Microsoft Research, author homepages, university repositories, Wayback snapshots of those). All files verified to start with `%PDF` and matched against title/authors via `pdftotext`.

Per-topic indexes (paper → file → source URL → status):
- [index-counting_and_derivatives.md](index-counting_and_derivatives.md)
- [index-redos_backrefs_verification.md](index-redos_backrefs_verification.md)
- [index-performance_hardware.md](index-performance_hardware.md)
- [index-semantics_theory_access.md](index-semantics_theory_access.md)

Caveat: dl.acm.org blocks scripted downloads (HTTP 403), so ACM papers are author copies, tech reports or arXiv versions, not always the camera-ready. Open them from dl.acm.org in a browser (free since 2026-01-01) for the publisher version. Exception: final publisher versions were added manually for counting-set automata (OOPSLA 2020), lookaround (POPL 2024), RE# (POPL 2025), bit-vector automata (OOPSLA 2023), linear JS matching (PLDI 2024), HybridSA (OOPSLA 2024), BVAP (ASPLOS 2024), RAP (ISCA 2025), verified lookaround (CPP 2025) Loring et al. (PLDI 2019), Warblre (ICFP 2024), Fujinami & Hasuo (ESOP 2024) and Becchi & Crowley (CoNEXT 2008).

## Not downloaded (13)

| Paper | Reason |
|---|---|
| Economy of Description by Automata, Grammars, and Formal Systems — Meyer, Fischer — SWAT (FOCS) 1971 (background) | failed: IEEE paywall; Unpaywall has no OA location; no free copy found |
| ReScue: Crafting Regular Expression DoS Attacks. Shen, Jiang, Xu, Yu, Ma, Lu. ASE 2018 | failed: ACM returns Cloudflare HTML; Unpaywall reports no OA copy; no author copy found |
| Analyzing Matching Time Behavior of Backtracking Regular Expression Matchers by Using Ambiguity of NFA. Weideman, van der Merwe, Berglund, Watson. CIAA 2016 | failed: Springer paywalled, Unpaywall reports no OA copy; only a third-party mirror (cflmark.nms.kcl.ac.uk) found, not used |
| Re-examining Regular Expressions with Backreferences. Berglund, van der Merwe. Theoretical Computer Science 940, 2023 | failed: Elsevier paywalled, Unpaywall reports no OA copy (earlier conference version downloaded, next row) |
| "CertiRE" (named in Gaps, no authors/venue) | failed: notes give no bibliographic data and no paper by this name could be identified |
| Bitwise Data Parallelism with LLVM: The ICgrep Case Study. Cameron, Medforth, Lin, Denis, Sumner. ICA3PP 2015 (LNCS 9529) | failed: paywalled (Springer). Unpaywall has no OA copy, and web search found no author copy (slides only) |
| Regular Expressions on Modern GPGPUs. (Xu, Chang, Zhu, Hong, Langdale, Qiu, Zhao per notes; author list not confirmed). GPGPU '24 workshop, 2024 | failed: the only OA location is dl.acm.org, which blocked curl (Cloudflare). No alternative copy found |
| PTME: A Regular Expression Matching Engine Based on Speculation and Enumerative Computation on FPGA. Sun, Xie et al. ACM TRETS 18, 2024/2025 | failed: the only OA location is dl.acm.org, which blocked curl (Cloudflare) |
| OD-REM: On-Demand Regular Expression Matching on FPGAs for Efficient Deep Packet Inspection. Xu, Zhou, Zhang, Jiang, Yi. ICFPT 2023 | failed: IEEE paywalled, no OA copy found (only an AMiner abstract listing) |
| SimdFSM: An Adaptive Vectorization of Finite State Machines for Speculative Execution. Li, Taura. PDCAT 2022, LNCS 13798 (2023) | failed: paywalled (Springer). No OA copy on Unpaywall or the web |
| Combining SIMD and Many/Multi-core Parallelism for Finite State Machines with Enumerative Speculation. Jiang, Agrawal. PPoPP 2017 | failed: no OA location on Unpaywall, and no author copy found |
| ANMLZoo: A Benchmark Suite for Exploring Bottlenecks in Automata Processing Engines and Architectures. Wadden et al. IISWC 2016 | failed: IEEE paywalled. No OA copy on Unpaywall, the UVA pages or the web |
| AutomataZoo: A Modern Automata Processing Benchmark Suite. Wadden, Tracy, Sadredini et al. IISWC 2018 | failed: IEEE paywalled. No OA copy on Unpaywall, Skadron's page or the web |

## Files

- 2004-algorithmica-navarro-new-techniques-regex-searching.pdf
- 2008-conext-becchi-extending-fa-pcre.pdf
- 2008-sp-smith-xfa-extended-automata.pdf
- 2010-ccr-cascarano-infant-nfa-gpgpu.pdf
- 2010-ciaa-okui-disambiguation-position-automata.pdf
- 2012-arxiv-ko-speculative-parallel-dfa-membership.pdf
- 2014-arxiv-sinya-simultaneous-finite-automata.pdf
- 2014-asplos-mytkowicz-data-parallel-finite-state-machines.pdf
- 2014-pact-cameron-bitwise-data-parallelism-regex.pdf
- 2016-asplos-angstadt-rapid-pattern-recognition-processors.pdf
- 2016-focs-backurs-hard-regex-patterns.pdf
- 2016-ton-avalle-nfa-multistriding-gpus.pdf
- 2017-arxiv-trofimovich-tdfa-with-lookahead.pdf
- 2017-focs-bringmann-dichotomy-regex-membership.pdf
- 2017-icpp-jung-parallel-construction-simultaneous-dfa.pdf
- 2017-psc-berglund-backreferences-reexamined.pdf
- 2017-tacas-wustholz-static-detection-dos-regex.pdf
- 2017-thesis-xue-bounded-repetition-icgrep.pdf
- 2018-arxiv-freydenberger-deterministic-regex-backreferences.pdf
- 2018-esecfse-davis-impact-redos-practice.pdf
- 2018-icalp-abboud-formula-sat-shaving-logs.pdf
- 2018-ictac-berglund-formalising-boost-posix-matching.pdf
- 2019-aplas-holik-counting-automata-sphere-construction.pdf
- 2019-arxiv-schmid-backreferences-polynomial-matching.pdf
- 2019-esecfse-davis-regexes-lingua-franca.pdf
- 2019-nsdi-wang-hyperscan-fast-multipattern-regex.pdf
- 2019-pldi-loring-sound-regex-semantics-dse.pdf
- 2020-esa-schepper-finegrained-regex-matching.pdf
- 2020-hpca-sadredini-impala-multistride-pattern-matching.pdf
- 2020-oopsla-turonova-counting-set-automata.pdf
- 2021-algorithms-gibney-text-indexing-regex-matching.pdf
- 2021-arxiv-casel-finegrained-regular-path-queries.pdf
- 2021-asplos-qiu-boostfsm-path-fusion-speculation.pdf
- 2021-micro-sadredini-sunder-near-data-pattern-matching.pdf
- 2021-pldi-stanford-symbolic-boolean-derivatives.pdf
- 2021-sp-davis-selective-memoization-redos.pdf
- 2021-sp-liu-revealer-redos.pdf
- 2021-spe-borsotti-efficient-posix-submatch-nfa.pdf
- 2021-tecs-parravicini-cicero-regex-architecture.pdf
- 2021-usenixsec-li-redoshunter-static-dynamic.pdf
- 2022-arxiv-borsotti-closer-look-tdfa.pdf
- 2022-arxiv-day-subsequences-gap-constraints.pdf
- 2022-cpp-egolf-verbatimpp-verified-lexing-derivatives.pdf
- 2022-eurocast-horky-bounded-repetition-re2.pdf
- 2022-fscd-chida-lookaheads-regex-backreferences.pdf
- 2022-hpca-huang-cama-content-addressable-memories.pdf
- 2022-ipdps-wang-gspecpal-speculative-fsm-gpus.pdf
- 2022-pldi-kong-hardware-codesign-inmemory-matching.pdf
- 2022-popl-chen-string-constraints-regex-transducers.pdf
- 2022-sp-chida-repairing-dos-realworld-regexes.pdf
- 2022-usenixsec-li-regexscalpel-localize-fix.pdf
- 2022-usenixsec-turonova-counting-considered-harmful.pdf
- 2023-arxiv-varatalu-derivative-extended-regex-matching.pdf
- 2023-arxiv-willard-efficient-guided-generation-llms.pdf
- 2023-fossacs-holik-synchronizing-counting.pdf
- 2023-mfcs-nogami-expressive-power-backreferences.pdf
- 2023-msrtr-moseley-derivative-nonbacktracking-matching-extended.pdf
- 2023-oopsla-leglaunec-bit-vector-automata.pdf
- 2023-pldi-moseley-derivative-nonbacktracking-matching.pdf
- 2023-pomacs-liu-asynchronous-automata-processing-gpus.pdf
- 2023-pvldb-riveros-rematch-regex-engine-all-matches.pdf
- 2023-sigmod-zhang-exploiting-structure-regex-queries.pdf
- 2023-sp-wang-rengar-redos-detection.pdf
- 2023-trets-karakchi-napoly-automata-processor-overlay.pdf
- 2024-arxiv-bhuiyan-sok-redos.pdf
- 2024-arxiv-dong-xgrammar-structured-generation.pdf
- 2024-arxiv-zhong-xav-regex-matching-engine.pdf
- 2024-asplos-ge-ngap-nonblocking-automata-gpus.pdf
- 2024-asplos-wen-bvap-automata-processing.pdf
- 2024-cpp-zhuchko-lean-extended-regex-lookarounds.pdf
- 2024-esop-fujinami-memoization-lookaround-atomic.pdf
- 2024-icalp-uezato-backreferences-lookaheads-nlog.pdf
- 2024-icfp-desanto-coq-mechanization-javascript-regex.pdf
- 2024-icpc-siddiq-understanding-redos-llm-regexes.pdf
- 2024-icsenier-siddiq-regexdoseval-generated-regexes.pdf
- 2024-oopsla-leglaunec-hybridsa-gpu-bit-parallelism.pdf
- 2024-pldi-barriere-linear-javascript-regex.pdf
- 2024-popl-mamouras-matching-regex-lookaround-assertions.pdf
- 2024-soda-bille-sparse-regex-matching.pdf
- 2024-usenixsec-su-redos-detection-nonbacktracking.pdf
- 2025-arxiv-amarilli-subsequence-supersequence-regex-matching.pdf
- 2025-arxiv-barriere-js-regex-formal-verification.pdf
- 2025-arxiv-liu-hyperflex-simd-dfa-model.pdf
- 2025-cav-varatalu-regex-decision-procedures-resharp.pdf
- 2025-cpp-chattopadhyay-verified-matching-regex-lookaround.pdf
- 2025-ic-nogami-measuring-expressive-power-stacking-automata.pdf
- 2025-isca-wen-rap-reconfigurable-automata.pdf
- 2025-itp-zhuchko-finiteness-symbolic-derivatives-lean.pdf
- 2025-mfcs-nogami-efficient-matching-fundamental-backreferences.pdf
- 2025-popl-varatalu-resharp-derivative-regex.pdf
- 2025-pvldb-zhang-ngram-selection-regex-indexing.pdf
- 2025-usenixsec-li-vulcanboost-redos-fixes.pdf
- 2026-arxiv-bille-regex-simple-backreferences.pdf
- 2026-arxiv-cakar-systematic-testing-regex-engines.pdf
- 2026-arxiv-deng-js-regex-complexity-rocq.pdf
- 2026-arxiv-kumabe-complexity-regex-backreferences.pdf
- 2026-arxiv-mahassadi-empirical-redos-detection-tools.pdf
- 2026-arxiv-nogami-hardness-regex-extensions.pdf
- 2026-eacl-sung-repairing-regex-localization-guided.pdf
- 2026-ncma-berglund-sparse-counting-sets.pdf
- 2026-pods-gawrychowski-optimal-enumeration-regular-pattern.pdf
- 2026-sigmod-zhang-regex-indexing-log-analysis.pdf
- 2026-sp-xu-pufferdos-attack-string-generation.pdf
- 2026-thesis-wojnarowski-verification-realistic-regex-matching.pdf
