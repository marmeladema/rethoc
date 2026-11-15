# High-performance regex matching research 2020–2026: SIMD, bit-parallelism, prefiltering, GPU, FPGA/ASIC/in-memory, streaming

Scope note: about 20 search/fetch calls. Every entry below was checked against a primary or near-primary page (arXiv abstract, PAR/NSF accepted manuscript, ACM proceedings PDF, author homepage, or MSR publication page) unless it is marked otherwise. "Applicability" bullets are my inferences for a CPU-based Rust regex engine. They are listed under Inferences, not Cited Findings. The repo is working on a bounded-gap/counting engine, so counting-related papers are covered in more depth.

## 1. SIMD and bit-parallel CPU matching (Hyperscan baseline and follow-ups)

### Takeaway
Hyperscan (NSDI 2019) is still the reference design for SIMD regex on CPUs: SIMD literal matchers plus bit-parallel NFA, used as a prefilter-and-decompose system. Peer-reviewed SIMD-regex papers from 2020–2026 are sparse. The clearest recent one is Hyperflex (arXiv Dec 2025, by Hyperscan authors), which accelerates selected DFA regions with SIMD, reports up to 2.27x over McClellan DFAs, and has been deployed in Hyperscan. Bit-parallel NFA ideas (shift-and generalized to regexes, bit-vector counters) are most active in the Rice/Mamouras group's work.

### Cited Findings
- **Hyperscan: A Fast Multi-pattern Regex Matcher for Modern CPUs.** Xiang Wang, Yang Hong, Harry Chang, KyoungSoo Park, Geoff Langdale, Jiayu Hu, Heqing Zhu. NSDI 2019 (open access). Hyperscan accelerates both string matching and finite-automata matching with SIMD operations, which gives a substantial throughput improvement. It decomposes regexes into literal strings and FA components. — [USENIX page](https://www.usenix.org/conference/nsdi19/presentation/wang-xiang), [PDF](https://usenix.org/system/files/nsdi19-wang-xiang.pdf)
- **Hyperflex: A SIMD-based DFA Model for Deep Packet Inspection.** Yang Liu, Wenjun Zhu, Harry Chang, Yang Hong, Geoff Langdale, Kun Qiu, Jin Zhao. arXiv:2512.07123, 8 Dec 2025 (preprint, open).
  - Technique: a region-detection algorithm finds parts of the DFA graph where SIMD instructions can perform state transitions. A hybrid transition algorithm switches between SIMD-accelerated regions and normal regions.
  - Results: 8.89 Gbit/s throughput and up to 2.27x over McClellan, Hyperscan's default DFA model.
  - Hyperflex has been deployed in Hyperscan.
  - [arXiv](https://arxiv.org/abs/2512.07123v1)
- **Bitwise data parallelism in regular expression matching (Parabix grep).** Cameron, Shermer, Shriraman, Herdy, Lin, Hull, Lin. PACT 2014.
  - Technique: input is transposed into bitstreams (one bit per byte position), and matching uses bitwise operations plus "MatchStar" built on long-stream addition, with SSE2/AVX2.
  - Results: one slide deck reports about 20x over pcregrep and 70x over GNU egrep on a 110 MB Arabic file.
  - [PDF](https://www2.cs.sfu.ca/~ashriram/papers/2014_PACT_GREP.pdf), [Semantic Scholar](https://www.semanticscholar.org/paper/Bitwise-data-parallelism-in-regular-expression-Cameron-Shermer/ef8459998865e58f7935c374dfee0475d7d736e9)
- **icgrep (ICA3PP 2015).** Cameron, Medforth, Lin, Denis, Sumner. Applies the bitstream approach to a full Unicode-capable grep. — [Springer](https://link.springer.com/chapter/10.1007/978-3-319-27122-4_26) (paywalled)
- **Thesis on bounded and unbounded repetition in Parabix/icgrep.** An SFU thesis by D. Xue covers bounded repetition handling in the icgrep architecture (open PDF). This is directly relevant to bounded-gap work. — [SFU thesis PDF](https://summit.sfu.ca/_flysystem/fedora/sfu_migrate/17719/etd10390_DXUE.pdf)
- **Regular Expression Matching using Bit Vector Automata.** Alexis Le Glaunec, Lingkun Kong, Konstantinos Mamouras. OOPSLA 2023, PACMPL, DOI 10.1145/3586044.
  - Technique: introduces nondeterministic bit vector automata (NBVA). They are expressively equivalent to nondeterministic counter automata with bounded counters, and they represent r{m,n} compactly rather than by unfolding it.
  - This is a software (CPU) matching paper.
  - [SPLASH page](https://2023.splashcon.org/details/splash-2023-oopsla/18/Regular-Expression-Matching-using-Bit-Vector-Automata), [author PDF](https://ohyoukillkenny.github.io/source/BVA.pdf), [PAR](https://par.nsf.gov/biblio/10417102-regular-expression-matching-using-bit-vector-automata)
- **Regex Matching with Counting-Set Automata.** Turoňová, Holík, Lengál, Saarikivi, Veanes, Vojnar. OOPSLA 2020, PACMPL 4(OOPSLA).
  - Technique: counting-set automata (CsA) have registers holding sets of bounded integers, manipulated by constant-time operations. Compilation goes regex → nondeterministic counting automaton → determinized CsA.
  - Results: CsA size does not grow with the repetition bounds, whereas the equivalent DFA is exponential in them.
  - A follow-up implemented CsA inside RE2 (Holíková, Horký, Síč, LNCS 2023).
  - [MSR page](https://www.microsoft.com/en-us/research/publication/counting-set-automata/), [SPLASH](https://2020.splashcon.org/details/splash-2020-oopsla/94/Regex-Matching-with-Counting-Set-Automata)
- **RE#: High Performance Derivative-Based Regex Matching with Intersection, Complement, and Restricted Lookarounds.** Ian Erik Varatalu, Margus Veanes, Juhan Ernits. POPL 2025, PACMPL 9, DOI 10.1145/3704837.
  - Technique: lazy DFA built from symbolic derivatives, with derivatives formalized over locations in words. This allows anchors and restricted lookarounds while keeping linear-time search.
  - Results: the authors report RE# is over 71% faster than the next fastest Rust regex engine on the baseline (rebar-style) benchmarks, and often orders of magnitude faster on the extended benchmarks. These are the authors' own measurements.
  - [arXiv 2407.20479](https://arxiv.org/abs/2407.20479), [POPL page](https://popl25.sigplan.org/details/POPL-2025-popl-research-papers/1/RE-High-Performance-Derivative-Based-Regex-Matching-with-Intersection-Complement-a), [author blog](https://www.iev.ee/blog/resharp-how-we-built-the-fastest-regex-in-fsharp)

### Inferences
- What Hyperflex suggests for a Rust lazy DFA: profile which DFA regions are "SIMD-friendly", for example states whose self-loop is taken on most bytes or that have few outgoing classes. Use vectorized scanning there (memchr-style skip-ahead, like regex-automata's existing "accelerated states") and fall back to table transitions elsewhere. Rust regex-automata already does a limited form of this for states with ≤3 escape bytes. Hyperflex generalizes the region idea.
- Bitstream (Parabix) matching is attractive for bounded repetition: r{m,n} over a byte class becomes shifts and run-length operations on bit vectors. This contrasts with unfolding, which blows up NFA/DFA size. NBVA and CsA give the formal basis for a counter- or bit-vector-based bounded-gap state.
- RE# is the closest academic competitor to a Rust engine and benchmarks against rust/regex using rebar-derived workloads.

### Gaps
- I found no peer-reviewed 2022–2026 paper that applies simdjson-style techniques (structural-character bitmaps, carry-less multiply for quote masking) to general regex. Becchi-lab SIMD/vectorized DFA papers from 2020–2026 did not surface either.
- I did not verify the exact venue (DOI) of the Teddy/FDR SIMD literal matchers. They are described in the Hyperscan NSDI paper and the Hyperscan/Rust aho-corasick source code, not in a separate paper.
- I could not find any 2022–2026 Cameron/Parabix paper in my searches.

## 2. Prefiltering, literal decomposition, and n-gram indexing

### Takeaway
2023–2026 database research treats regex acceleration as query planning. BLARE (SIGMOD 2023) splits a regex into literal and regex components and picks an evaluation order adaptively with multi-armed bandits, giving 1.6–3.7x over RE2. For indexed search, a PVLDB 2025 evaluation of n-gram selection (FREE, BEST, LPMS, VGGraph) finds no universal winner, and FREE is the practical default. A SIGMOD 2026 paper from the same group targets log regex indexing. Industry practice is moving from trigrams to "sparse n-grams" (Cursor and GitHub-style code search).

### Cited Findings
- **Exploiting Structure in Regular Expression Queries (BLARE).** Ling Zhang, Shaleen Deep, Avrilia Floratou, Anja Gruenheid, Jignesh M. Patel, Yiwen Zhu. SIGMOD 2023.
  - Technique: splits a regex into regex and string-literal components and can evaluate them in an order other than left-to-right. It chooses a plan at runtime with a multi-armed bandit instead of cost-based optimization. It is modular and sits on top of any regex library.
  - Strategies (from the slides): the regex is split into a prefix, a middle regex, and a literal suffix, and BLARE chooses among 3-way, multi-way, and direct strategies.
  - Results:
    - 1.6–3.7x faster than RE2 and 3.4–7.9x faster than Boost Regex.
    - Improves PCRE2 by 3.1x to over 100x on two workloads (PCRE2 did not finish on the third).
    - 61.7x for ICU on the open-source dataset.
  - Code: implemented on RE2, PCRE2, Boost, and ICU. Code is public.
  - [MSR page](https://www.microsoft.com/en-us/research/publication/exploiting-structure-in-regular-expression-queries/), [PDF](https://mush-zhang.github.io/homepage/publication/blare/BLARE.pdf), [slides](https://mush-zhang.github.io/homepage/publication/blare/blare_new_slides.pdf), [code](https://github.com/mush-zhang/Blare)
- **An Evaluation of N-Gram Selection Strategies for Regular Expression Indexing in Contemporary Text Analysis Tasks.** Ling Zhang, Shaleen Deep, Jignesh M. Patel, Karthikeyan Sankaralingam. PVLDB vol. 18 (VLDB 2025), extended version arXiv:2504.12251. Open access.
  - Strategies compared: FREE (frequency/selectivity-based), BEST (coverage-optimized and query-aware), LPMS (LP relaxation), and VGGraph, in a unified framework.
  - FREE builds indexes much faster. On Webpages, 1.7×10⁵ keys took 1,184 s versus 8,587 s for BEST.
  - BEST wins at small key budgets (DBLP precision 0.235 vs 0.051 with ≤150 keys) but has very high construction cost and did not finish on SQL-Srvr.
  - LPMS on SQL-Srvr with 50 keys had about 10x FREE's precision and 2.43x faster queries, with 3.4x longer build time.
  - The paper notes that Postgres (pg_trgm) and GitLab skip selection and index all trigrams or fixed n-grams.
  - Datasets: Webpages (~47 GB Common Crawl), DBLP, Prosite, US-Acc, SQL-Srvr (~102M logs), Enron, ADX (~890M logs), and a Synthetic set.
  - Conclusion: no single method is best across all workloads.
  - [PVLDB PDF](https://www.vldb.org/pvldb/vol18/p5703-zhang.pdf), [arXiv](https://arxiv.org/html/2504.12251v2)
- **Regular Expression Indexing for Log Analysis.** Same group. Listed as SIGMOD 2026. It combines n-gram indexing with compact log storage. A secondary summary reports up to 14x faster regex queries with about 2.1% space overhead. These figures are not verified against the paper. — [listing](https://rmarcus.info/dbscholar/papers/h5e7ce476058648e7)
- **Sparse n-grams (industry, not peer-reviewed).** Split points are chosen with a byte-pair weight table: a substring is a gram if its edge pair-weights exceed all interior pair-weights. This yields fewer, longer, more selective grams than trigrams. The query plan uses AND for concatenation and OR for alternation, then verifies candidates with the full regex.
  - Rust implementations: syntext and fastripgrep.
  - Counterpoint: the ngi project reports 12M+ sparse grams vs 181K trigrams on CPython, so the index can grow much larger.
  - [Cursor blog](https://cursor.com/blog/fast-regex-search), [syntext architecture](https://docs.rs/crate/syntext/1.0.0/source/docs/ARCHITECTURE.md), [ngi](https://github.com/erogol/ngi)

### Inferences
- What BLARE suggests for an in-memory engine (no index): when a regex contains several required literals, it can pay to search for the rarest literal first, in any position, and then verify outward with reverse and forward DFAs. This generalizes Rust regex's existing "reverse suffix" and "reverse inner" strategies. The bandit-based choice among strategies at runtime is cheap and could replace fixed heuristics, for example by sampling the first N KB of a haystack.
- The FREE/BEST findings mostly matter for index-backed tools. For a single-pass engine, the analogous problem is literal-set selection for prefilters (Teddy/Aho-Corasick), where rarity and selectivity heuristics (like Rust's byte-frequency rank table) play the role of FREE.

### Gaps
- I did not find the Google Code Search trigram write-up (Russ Cox 2012, blog only) or a Zoekt academic paper in this session. Neither is peer-reviewed.
- I did not verify the speedup figures in the SIGMOD 2026 paper.

## 3. GPU regex and automata processing

### Takeaway
GPU work from 2020–2026 comes mainly from two lines. Hongyuan Liu's group fixed GPU NFA under-utilization: AsyncAP (SIGMETRICS 2023) starts threads at different input offsets, and ngAP (ASPLOS 2024) uses non-blocking execution across symbols and reports 7.9x average over the prior GPU state of the art. Rice's HybridSA (OOPSLA 2024) uses bit-parallel shift-and generalized to regexes on GPU and reports 4–60x over the best CPU engine. Bit-parallelism is the idea that transfers best to a CPU engine.

### Cited Findings
- **ngAP: Non-blocking Large-scale Automata Processing on GPUs.** Tianao Ge, Tong Zhang, Hongyuan Liu. ASPLOS 2024, DOI 10.1145/3617232.3624848.
  - Technique: prior engines advance one input symbol per iteration and leave threads idle. ngAP lets states matched on different symbols run concurrently and adds a memoization table that replaces repeated computation with lookups.
  - Results: thread utilization rises from 32.6% to 83.6%. Average 7.9x (up to 901x) over the prior GPU state of the art on 20 applications. The slides report an average 11.5x over Hyperscan on CPU.
  - [PDF](https://getianao.github.io/papers/asplos24ngap.pdf), [slides](https://getianao.github.io/files/asplos24ngap-slides.pdf), [artifact](https://github.com/getianao/ngAP)
- **Asynchronous Automata Processing on GPUs (AsyncAP).** Hongyuan Liu, Sreepathi Pai, Adwait Jog. Proc. ACM Meas. Anal. Comput. Syst. 7(1), Art. 27, March 2023 (DOI 10.1145/3579453). Also a SIGMETRICS 2023 abstract (10.1145/3578338.3593524).
  - Technique: exploits input-level parallelism by having GPU threads start at different input positions rather than reading the input serially.
  - Results: 2.4x average speedup over the prior GPU engine on 12 applications.
  - [PAR](https://par.nsf.gov/biblio/10488575-asynchronous-automata-processing-gpus), [PDF](https://par.nsf.gov/servlets/purl/10488575)
- **HybridSA: GPU Acceleration of Multi-pattern Regex Matching using Bit Parallelism.** Alexis Le Glaunec, Lingkun Kong, Konstantinos Mamouras. OOPSLA 2024, PACMPL 8(OOPSLA2), Art. 331, DOI 10.1145/3689771.
  - Technique: extends shift-and to a large class of regexes, so NFA simulation becomes a few bitwise operations per byte. A compiler emits bit masks and selects among shift-and variants. Patterns with random memory access go to the CPU in parallel.
  - Results: 4–60x throughput over the best CPU engine and 4–233x over the best GPU engine.
  - [PAR](https://par.nsf.gov/biblio/10618058), [author PDF](https://alexis51151.github.io/assets/pdf/hybridsa.pdf)
- **GPGPU '24 study (Xu, Chang, Zhu, Hong, Langdale, Qiu, Zhao).** Revisits GPU regex designs and finds that naive ports to modern GPUs suffer low thread occupancy. Seen only through a search snippet; not fetched. — [unpaywall DOI 10.1145/3649411.3649416](https://unpaywall.org/10.1145%2F3649411.3649416)
- **Older baselines:** iNFAnt (2010) is the original GPU NFA engine. Multi-striding NFA on GPUs (Avalle, Risso, Sisto, IEEE/ACM ToN 2016) reports almost an order of magnitude improvement. — [PDF](https://iris.polito.it/retrieve/handle/11583/2612560/65210/15TON-mstride.pdf)

### Inferences
- HybridSA's compiler (regex → shift-and masks, including generalizations for classes, optional parts, and bounded repetition) maps directly onto a CPU bit-parallel NFA using u64 or u128/AVX2 words. It is a candidate engine for small regexes (≤64–256 NFA positions) that would otherwise use a PikeVM. This is the classic Glushkov bit-parallel NFA (Navarro–Raffinot). It is also a natural fit for bounded gaps such as `.{m,n}`.
- AsyncAP's and ngAP's input-offset parallelism is the GPU analogue of chunked or speculative parallel matching (Section 5).

### Gaps
- I found no verified 2025–2026 peer-reviewed GPU regex paper beyond the above.
- I did not verify GSpecPal (a GPU speculative FSM paper) or its year.

## 4. FPGA, ASIC, and in-memory automata processors

### Takeaway
After the Micron AP, the in-memory automata line went through Cache Automaton, Impala (HPCA 2020), CA, eAP, and CAMA (HPCA 2022, CAM-based). In 2022–2024 the Rice group targeted bounded repetition specifically: PLDI 2022 added counter and bit-vector modules, and BVAP (ASPLOS 2024) builds AH-NBVA on top of CAMA. BVAP reports 67–95% higher energy efficiency and 42–68% lower area than CA, eAP, and CAMA. The key lesson: unfolding r{m,n} wastes about 16,384 memory bits for a {..1024} counter when about 10 bits of information suffice.

### Cited Findings
- **BVAP: Energy and Memory Efficient Automata Processing for Regular Expressions with Bounded Repetitions.** Ziyuan Wen, Lingkun Kong, Alexis Le Glaunec, Konstantinos Mamouras, Kaiyuan Yang. ASPLOS 2024 (Vol. 2), DOI 10.1145/3620665.3640412.
  - Technique: a software-hardware co-designed in-memory bit vector automata processor, based on Action-Homogeneous Nondeterministic Bit Vector Automata (AH-NBVA), with a compiler from regexes to hardware configuration. It extends CAMA with bit-vector modules.
  - Results: a cycle-accurate simulator in 28 nm CMOS shows 67–95% higher energy efficiency and 42–68% lower area than CA, eAP, and CAMA on real-world benchmarks.
  - [open PDF (PAR)](https://par.nsf.gov/servlets/purl/10534634)
- **Software-Hardware Codesign for Efficient In-Memory Regular Pattern Matching.** Lingkun Kong, Qixuan Yu, Agnishom Chattopadhyay, Alexis Le Glaunec, Yi Huang, Konstantinos Mamouras, Kaiyuan Yang. PLDI 2022.
  - Technique: an in-memory architecture with counter and bit-vector elements for bounded repetition. Static analysis of the regex bounds the memory each counting occurrence needs.
  - Evaluation: SPICE-calibrated simulator on TSMC 28 nm.
  - Motivation from this line of work: a counting operator with upper bound 1024 needs at least 16,384 memory bits when unfolded, while the information required may be only 10 bits.
  - [PLDI page](https://pldi22.sigplan.org/details/pldi-2022-pldi/31/Software-Hardware-Codesign-for-Efficient-In-Memory-Regular-Pattern-Matching), [PAR PDF](https://par.nsf.gov/servlets/purl/10330072), [arXiv 2209.05686](https://arxiv.org/pdf/2209.05686)
- **CAMA: Energy and Memory Efficient Automata Processing in Content-Addressable Memories.** Yi Huang, Zhiyu Chen, Dai Li, Kaiyuan Yang. HPCA 2022.
  - Technique: a 16×256 8T CAM array for state matching on homogeneous NFAs, replacing 256×256 6T SRAM.
  - Results: CAMA-E uses 2.1x, 2.8x, and 2.04x less energy than CA, 2-stride Impala, and eAP. CAMA-T has 2.68x, 3.87x, and 2.62x higher compute density than 2-stride Impala, CA, and eAP.
  - [arXiv 2112.00267](https://arxiv.org/pdf/2112.00267)
- **Impala: Algorithm/Architecture Co-Design for In-Memory Multi-Stride Pattern Matching.** Elaheh Sadredini, Reza Rahimi, Marzieh Lenjani, Mircea Stan, Kevin Skadron. HPCA 2020. A multi-stride in-memory automata architecture that processes several symbols per cycle by squashing nibble-based automata. — [PDF](https://www.cs.ucr.edu/~elaheh/papers/Impala_HPCA2020.pdf)
- **FPGA work, 2023–2024 (secondary listings only):**
  - PTME: a speculative and enumerative two-character matching engine on FPGA, ACM TRETS 2024. — [listing](https://bohrium.dp.tech/paper/arxiv/548906c145ce471f90a32e3c)
  - XAV: FPGA-CPU architecture, up to 75 Gbps on Snort rules, arXiv 2024. — [arXiv 2403.16533](https://export.arxiv.org/abs/2403.16533)
  - OD-REM: on-demand regex matching on FPGAs for DPI, ICFPT 2023 (citation only).
  - NAPOLY: an NFA processor overlay, TRETS 2023 (citation only).
  - CICERO: a domain-specific regex architecture (Politecnico di Milano). — [PDF](https://re.public.polimi.it/bitstream/11311/1184919/1/CICERO_CASES%281%29.pdf)

### Inferences
- The hardware papers' main transferable idea for a CPU engine: represent bounded repetition r{m,n} as a bit vector (shift register) of "active offsets" or as a counter set, instead of unfolding it into m..n NFA states. AH-NBVA's "action-homogeneous" restriction means all transitions into a state perform the same bit-vector action. That restriction keeps per-byte updates as a single shift/OR/AND, which is the same property that makes it cheap in SIMD registers.
- Multi-stride (Impala) is analogous to processing 2 bytes per step with a larger alphabet table. That idea has mostly lost out to SIMD skip-ahead on CPUs.

### Gaps
- I did not verify a "Sunder" paper (likely Sadredini et al., about reconfigurable in-memory automata) or RAPID (Angstadt et al., ASPLOS 2016, a language for automata) in this session.
- I found no 2025–2026 automata-processor paper. I did not confirm "RegexDB".
- I did not search MICRO, ISCA, or FCCM 2024–2025 proceedings directly. Search engines returned no hits for those venues.

## 5. Streaming, chunked, and speculative parallel matching of a single input

### Takeaway
The foundations are older: Mytkowicz et al. (ASPLOS 2014) and Ko et al. (2012). 2020–2026 follow-ups are incremental, for example SimdFSM (PDCAT 2022) adaptively vectorizes speculative FSM execution, and GPU engines (AsyncAP, ngAP) apply offset-parallelism. No major new CPU-regex breakthrough in this area surfaced.

### Cited Findings
- **Data-Parallel Finite-State Machines.** Mytkowicz, Musuvathi, Schulte. ASPLOS 2014.
  - Technique: enumerative execution, where all states run in parallel using shuffle instructions, plus convergence.
  - Results: up to 3x over optimized sequential code on a single core (regex, Huffman decoding, HTML tokenization).
  - [MSR](https://www.microsoft.com/en-us/research/publication/data-parallel-finite-state-machines/), [PDF](https://microsoft.com/en-us/research/wp-content/uploads/2016/02/asplos302-mytkowicz.pdf)
- **A Speculative Parallel DFA Membership Test for Multicore, SIMD and Cloud Computing Environments.** Ko, Jung, Han, Burgstaller. arXiv 2012 (later IJPP).
  - Technique: split the input into chunks, match chunks in parallel, and combine the results. It is failure-free: sequential semantics hold and there are no slowdowns.
  - Results: 2.3–38.8x for PCRE DFAs up to 512 states on 40 cores.
  - [arXiv 1210.5093](https://arxiv.org/pdf/1210.5093)
- **Simultaneous Finite Automata (SFA).** Sin'ya et al., ICPP 2017. A data-parallel model for regex matching on multicores with SIMD kernels. — [preprint](https://elc.yonsei.ac.kr/assets/papers/pdf/ICPP_SFA_preprint.pdf), [arXiv 1405.0562](https://arxiv.org/pdf/1405.0562)
- **SimdFSM: An Adaptive Vectorization of Finite State Machines for Speculative Execution.** Li and Taura. PDCAT 2022, LNCS 13798 (2023). — [DOI via unpaywall](https://unpaywall.org/10.1007%2F978-3-031-29927-8_37)
- **Jiang and Agrawal (2017).** Combines SIMD with many-core enumerative speculation. Up to 2.5x from SIMD on one core and up to 95x with 60 Xeon Phi cores. — [listing](https://augusta.elsevierpure.com/en/publications/combining-simd-and-manymulti-core-parallelism-for-finite-state-ma-3/)

### Inferences
- For a Rust engine, chunked parallel search is mainly useful for very large haystacks (multi-GB, as in grep on huge files). Lazy DFAs converge quickly, so a Mytkowicz-style approach works: run each chunk from the "all-states" or start set and stitch the results. In practice, ripgrep gets parallelism across files instead.

### Gaps
- I did not verify a 2023–2026 CPU speculative-regex paper. I did not fetch the BoostFSM paper (25.8x on 64 cores, seen in a snippet).

## 6. Benchmarks and datasets

### Takeaway
Hardware and GPU papers standardize on ANMLZoo (2016) and AutomataZoo (2018), which include Snort, ClamAV, Brill, PowerEN, Protomata, YARA, and CRISPR. Software and database papers use workload-specific sets: production logs, Prosite, DBLP, and Enron. CPU-engine work, including RE#, uses rebar (Gallant) and its derivatives.

### Cited Findings
- **ANMLZoo.** Wadden et al., IISWC 2016. The de facto automata benchmark suite. Its errata warns that the regex sets may not match the generated automata exactly. — [GitHub](https://www.github.com/jackwadden/ANMLZoo)
- **AutomataZoo.** Wadden et al., IISWC 2018. 24 benchmarks with standard large inputs and full kernels, so automata and non-automata approaches can be compared. Includes Snort, CRISPR/Cas9, YARA, file carving, sequence matching, and entity resolution. — [SIGARCH announcement](https://www.sigarch.org/other-announcements/automatazoo-a-benchmark-suite-for-automata-processing/), repo github.com/hplp/AutomataZoo
- **Benchmarks used by specific papers:**
  - ngAP uses 20 applications and AsyncAP uses 12, drawn from ANMLZoo- and AutomataZoo-style suites. — [ngAP PDF](https://getianao.github.io/papers/asplos24ngap.pdf)
  - BLARE and the n-gram evaluation use production logs (SQL-Srvr, ADX), Prosite, DBLP, US-Acc, Enron, and Webpages. — [arXiv 2504.12251](https://arxiv.org/html/2504.12251v2)
  - RE# reports results on a baseline benchmark plus extensions. — [arXiv 2407.20479](https://arxiv.org/abs/2407.20479)

### Inferences
- For a Rust engine, rebar (github.com/BurntSushi/rebar) is the most relevant harness. Adding ANMLZoo- or AutomataZoo-derived regex sets (Snort, ClamAV, Protomata) would allow comparison with hardware and GPU papers, and Protomata/Prosite-style inputs exercise bounded repetition heavily.

### Gaps
- I did not fetch the rebar repository in this session, so its contents and methodology are not cited here. I also did not check the exact ANMLZoo benchmark list.
