---
file: ../pdfs/2026-arxiv-cakar-systematic-testing-regex-engines.pdf
citation: "Towards the Systematic Testing of Regular Expression Engines. Çakar, Lee, Davis. arXiv 2026"
doi: 
source: "https://arxiv.org/pdf/2603.00311"
topics: [redos_backrefs_verification]
pages: 10
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Towards the Systematic Testing of Regular Expression Engines
Berk Çakar

arXiv:2603.00311v1 [cs.SE] 27 Feb 2026

Electrical and Computer Engineering
Purdue University
West Lafayette, IN, USA
bcakar@purdue.edu

Dongyoon Lee

Computer Science
Stony Brook University
Stony Brook, NY, USA
dongyoon@cs.stonybrook.edu

James C. Davis

Electrical and Computer Engineering
Purdue University
West Lafayette, IN, USA
davisjam@purdue.edu

Abstract

1

Software engineers use regular expressions (regexes) across a wide
range of domains and tasks. To support regexes, software projects
must integrate a regex engine, whether provided natively by the
language runtime (e.g., Python’s re) or included as an external dependency (e.g., PCRE). However, these engines may contain bugs
and introduce vulnerabilities. A common strategy for testing regex
engines involves differential testing—comparing outputs across
different implementations. However, this approach is concerning
because regex syntax and semantics vary significantly between
dialects (e.g., POSIX vs. PCRE). Fuzzing is also utilized to ease testing of feature-rich regex implementations to expose defects, but
naive byte-level mutations generate syntactically invalid inputs
that exercise only parsing logic, not matching internals.
In this work, we describe our progress towards ReTest, a framework that systematically tests regular expression engines by combining grammar-aware fuzzing for high code coverage with metamorphic testing to generate dialect-independent test oracles. So
far, we have surveyed testing practices across 22 regex engines,
analyzed 1,007 regex engine bugs and 156 CVEs to characterize
failure modes, and curated 16 metamorphic relations for regexes
derived from Kleene algebra. Our preliminary evaluation on PCRE
shows that ReTest achieves 3× higher edge coverage than existing
fuzzing approaches and has identified three new memory safety
defects. We conclude by describing our next steps toward our ultimate goal: helping regex engine developers identify bugs without
depending on a consistent cross-implementation standard.

In this paper, we are working towards improving the testing techniques for a critical class of software infrastructure: regular expression (regex) engines. Regexes are a fundamental tool in software
engineering, used extensively for pattern matching [25], input validation [1, 3], text processing [24, 43], and data extraction [5, 6]. In
software projects, regexes are supported through regex engines—
software components that interpret regex patterns and execute
matching operations against input strings. These engines are either built into language runtimes (e.g., Python’s re module [23],
JavaScript’s (V8) Irregexp [28]) or provided as external libraries
(e.g., PCRE [46], RE2 [17], Oniguruma [33]). However, regex engines are not immune to bugs. One of the most widely used regex
engines, PCRE2, has had 10+ CVEs (all rated high or critical) and
140+ confirmed bug reports since 2015 (§5).
Current testing practices for regular expression engines are
largely ad hoc and inconsistent. Our analysis of prominent regex engine implementations (§2.1) shows that most engines rely primarily
on unit and regression testing, which demand substantial manual
effort. Differential testing is widely used to identify semantic discrepancies, but its effectiveness is limited by the lack of a universal
regex standard: engines implement distinct dialects with subtle
semantic differences, causing differential testing to generate false
positives that reflect expected dialect variation rather than genuine
defects (§4.1). Fuzzing is also utilized for the discovery of safety
violations, yet existing approaches are typically naive byte-level
mutations that offer limited coverage of matching logic. Ultimately,
existing testing practices suffer from the absence of both a unified,
systematic testing methodology and a common semantic oracle.
In this paper, we present our work towards ReTest, a systematic testing framework that combines grammar-aware fuzzing with
metamorphic testing. To automate the discovery of code paths,
ReTest employs a grammar-aware mutation strategy: it parses regexes into abstract syntax trees (ASTs) and performs structured
subtree replacements seeded from a large-scale corpus of realworld patterns [12]. Unlike naive byte-level fuzzing—which is likely
to generate invalid inputs that are rejected during parsing—this
structure-aware approach ensures generated patterns are syntactically valid, thereby exercising the engine’s matching logic [2, 63].
While fuzzing with runtime sanitizers [27, 54, 56] effectively identifies safety violations (e.g., buffer overflows, use-after-free errors,
integer overflows, and undefined behavior), it cannot detect noncrashing semantic errors. Therefore, ReTest supplements crash
oracles with metamorphic testing [16, 53]: by verifying algebraic
properties (e.g., 𝑟 ∗∗ ≡ 𝑟 ∗ ) within a single engine, ReTest provides a
semantic oracle without the false positives in differential testing.

CCS Concepts
• Software and its engineering → Software testing and debugging; • Security and privacy → Software security engineering.

Keywords
Regex engines, fuzzing, metamorphic testing, software testing
ACM Reference Format:
Berk Çakar, Dongyoon Lee, and James C. Davis. 2026. Towards the Systematic Testing of Regular Expression Engines. In Journal Ahead Workshop
(JAWs) 2026. 10 pages.

This work is licensed under a Creative Commons Attribution 4.0 International License.
JAWs 2026, Rio de Janeiro, Brazil
© 2026 Copyright held by the owner/author(s).

Introduction

<!-- page 2 -->

JAWs 2026, April 2026, Rio de Janeiro, Brazil

Our contributions so far are:
• Empirical Study of Testing Practices. We survey 22 regex
engines and characterize 12 testing techniques employed in practice, revealing that 82% of bugs are user-reported and only 2 of
22 engines employ grammar-aware fuzzing (§4).
• Bug and CVE Analysis. We analyze 1,007 bugs across 11 thirdparty engines and 156 CVEs across the regex engine landscape,
finding that semantic bugs (35%) and memory safety vulnerabilities (52% of CVEs) dominate (§5).
• ReTest Framework. We design and prototype ReTest, combining grammar-aware fuzzing with metamorphic testing (§6).
• Metamorphic Relations for Regexes. We compile a catalog of
16 metamorphic relations, providing dialect-independent oracles
for semantic bug detection (§6).
• Preliminary Evaluation. We demonstrate ReTest prototype’s
feasibility on PCRE, achieving 3× higher coverage than baselines
and discovering three memory safety bugs (§7).
In §8, we describe our planned future work.

2

al. [18] observed, even among regexes that compile successfully in
multiple languages, a significant fraction produce divergent results.
Regexes are thus far from being a lingua franca [20]—practitioners
cannot assume that a pattern tested in one environment will behave
identically elsewhere.

2.2

Research on Regex Engine Testing

Prior work on regular expressions focuses predominantly on testing
patterns, not engines. Research on test string generation [35, 37],
ReDoS detection [19, 38, 40], and pattern synthesis/repair [39, 45]
all assume engine correctness while validating developer-written
patterns. Empirical studies of regex bugs likewise examine patternlevel faults rather than implementation defects [22, 60, 61].
More broadly, testing language implementations and DSL engines offers transferable techniques. Csmith’s [62] random program
generation found 325+ compiler bugs, while grammar-based fuzzing
has proven effective for language implementations (e.g., [2, 30, 41]).
For DBMS, Rigger and Su [48, 49, 50] used metamorphic oracles to
detect logic bugs. We portray regex engine testing in practice in §4.

Background and Related Work

We introduce regex engines (§2.1), prior research on regex engine
testing (§2.2), and metamorphic testing (§2.3).

2.1

Çakar, Lee, and Davis

Regex Engines

A regex engine is a software component that compiles a regex pattern 𝑟 into an intermediate representation (e.g., an automaton) and
applies it to determine whether an input string 𝑠 matches 𝑟 . Most
regex engines employ one of three algorithms [10]1 : (1) backtracking, which simulates an NFA via recursive exploration with exponential worst-case complexity but supports expressive features like
backreferences and lookarounds; (2) automata-based, which uses
Thompson’s lockstep algorithm [58] for linear-time matching guarantees but cannot support expressive (i.e., non-regular) features;
or (3) hybrid, combines both strategies, employing automata-based
matching wherever possible and falling back to backtracking for
patterns requiring advanced features.
Formally, regex patterns restricted to concatenation, alternation,
and Kleene star are called K-regexes [32] and correspond to regular
languages; patterns with additional features like backreferences,
lookarounds, and atomic groups are called E-regexes (extended
regexes) and may recognize non-regular languages. [13].
Despite this theoretical foundation, no unified specification governs regex syntax and semantics. Each engine implements its own
dialect (or flavor) [31] of regexes. Most dialects draw from two
influential sources: the POSIX standard [57] and PCRE [46] (PerlCompatible Regular Expressions). These sources differ in both constructs (e.g., PCRE supports lookahead assertions and non-capturing
groups absent from POSIX) and match semantics (e.g., POSIX mandates leftmost-longest matching, whereas PCRE follows leftmostfirst semantics based on alternative ordering). Engines may adopt
constructs from one, both, or a selective combination of these specifications, and may even introduce their own. This lack of standardization results in non-portable regular expressions: the same
pattern may exhibit different behavior across engines. As Davis et
1 Microsoft’s C# has explored [44, 51] Brzozowski derivatives [11] as a fourth approach.

2.3

Metamorphic Testing

Metamorphic testing (MT) addresses the oracle problem—where
expected outputs are unknown, by verifying relationships between
multiple executions rather than individual results [15]. Given a
source input and its output, metamorphic relations (MRs) define
how transformed follow-up inputs should relate to the original
output. MT has proven effective for testing compilers and language
implementations: Le et al.’s Equivalence Modulo Inputs (EMI) [36]
detected 147 bugs in GCC and LLVM. For DSLs specifically, Mansur
et al. [42] applied MT to Datalog engines, finding 13 query bugs.
In the context of regexes, an MR specifies a transformation 𝑇
where 𝑇 (𝑟 ) ≡ 𝑟 , asserting that Match(𝑟, 𝑠) ∼ Match(𝑇 (𝑟 ), 𝑠) for
all inputs 𝑠, where ∼ denotes the relation checked.

3

Research Overview

Building the first testing framework for regex engines requires
a comprehensive understanding of both the current of state of
practice and the nature of defects in these systems. We structure
our research around two phases and three questions that progress
from empirical investigation to system design and evaluation.
Phase 1: Empirical Investigation. Existing literature provides little
insight into how regex engines are validated in the wild or what
types of bugs persist despite these efforts. To inform the design
of systematic testing framework, we first establish a baseline of
current practices and failure modes.
RQ1 How are regular expression engines tested in practice?
We survey regex engines to determine the prevalence of testing techniques. This exploratory question sheds light on engineering practices across leading representatives of this class
of software.
RQ2 What are the failure modes and their causes in regex engines?
By analyzing historical bug reports and CVEs, we learn whether
existing testing gaps lead to reliability or security issues (e.g.,
semantic errors vs. memory safety violations).

<!-- page 3 -->

Towards the Systematic Testing of Regular Expression Engines

JAWs 2026, April 2026, Rio de Janeiro, Brazil

Table 1: Feature comparison of 22 regex engines based on commonly referred regex features and constructs. The features
and constructs are compiled from [21, 31, 47, 57]. For brevity, we inspect only the existence of advanced features (e.g.,
callouts) rather than specific syntax. Notation: BT =Backtracking, Auto=Automata, Hyb=Hybrid.

(?s)

(?=)
(?!)
(?<=)
(?<!)
\n — \gn — \g{n}
\g±n — \g{±n}

Named backrefs.

Subroutines & recursion
Conditionals
BT Control Verbs
Callouts

<regex>
BT ✓ ✓ ✗ ✗ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗
Auto ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗
regexp
S.T.Regex BT ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗
regex
Auto ✓ ✓ ✗ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗

(?x)

C++
Go𝑎
C#
Rust𝑎

L’arounds Backrefs. Advnc’d

✗

✓ ✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓ ✗ ✓ ✗ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓

✓

✓ ✓✓? ? ✓ ✗

✓

✗ ✓ ✗ ✗

(?i)

✓

Options

(?J)

✓ ✓ ✓ ✗

BT ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓

Irregexp

Capturing
\b — \B
\K
(...)
(?:...)
(?|...)
Named capt. groups
(?#...)

✗

j.u.regex

JS/TS

\G

[...]
[ˆ...]
[x-y]
[[:xxx:]]

BT ✓ ✓ ✗ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗ ✗

Java

Anchors

? — * — +
?? — *? — +?
?+ — *+ — ++
{n}
{n,m}
{n,} — {,m}
(?>...)
ˆ
$
\A
\Z
\z

\P{xx}

Python re

Engine

Quantifiers

(?m)

Unicode Character Classes Alt.
expr|expr|expr...

Character Types

[[:ˆxxx:]]

Characters

x
\x
\Q...\E
\a
\cx
\e
\f
\n
\r
\t
\0dd–\o{ddd...}
\xhh–\x{hhh...}
.
\C
\d — \D
\h — \H
\s — \S
\v — \V
\w — \W
\N
\R
\X
\p{xx}

Nature of the Engine

Literals

First-Party (Language Standard Library)
✓ ✓ ✓ ✗‡

BT ✓ ✓ ✗ ✗ ✗ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗ ✓† ✓† ✓ ✓ ✓ ✗

✗‡

✓ ✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓ ✓ ✓ ✗ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓

✗

✓ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓ ✓ ✗ ✗ ✗ ?‡ ✓ ✗ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓
✗
✗
✓
✗

✓ ✓✓ ✓ ✓ ✓ ✗

✓

✗ ✗ ✗ ✗

✗ ✓✓ ✓ ✓ ✓ ✗

✓

✗ ✗ ✗ ✗

✗
✗
✓
✗

✗ ✗ ✗ ✗
✗ ✗ – ✗
✗ ✓ ✗ ✗
✗ ✗ – ✗

✓ ✗ ✓ ✓ ✗ ✗ ✗ ✓‡ ✗ ✓‡ ✓‡ ✓‡ ✓ ✓ ✗ ✗ ✓ ✗
✓ ✗ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✓ ✓ ✗ ✗ ✗ ✗ ✗ ✗
✓ ✗ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓✓ ✓ ✓ ✓ ✗
✓ ✗ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✓ ✓ ✗ ✗ ✗ ✗ ✗ ✗

✗
✓
✓
✓

✗
✓
✓
✓

✓ ✓ ✓ ✓
✓ ✓ ✓ ✓
✓ ✓ ✓ ✗
✓ ✓ ✓ ✓

✓
✓
✗
✓

✓
✓
✓
✓

BT ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
BT ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✓ ✓
Auto ✓ ✓ ✗ ✗ ✗ ✗ ✗ ✓ ✓ ✓ ✗ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗ ✗

✓
✓
✗

✓ ✓ ✓ ✓
✓ ✓ ✓ ✓
✓ ✓ ✓ ✓

✓
✓
✓

✓ ✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓ ✓ ✓ ✓ ✓ ✓✓ ✓ ✓ ✓ ✓ ✓
✓ ✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓
✓ ✓ ✗ – ✓✓✓ – ✓✓ ✗ ✗ ✗ ✗ ✗ ✗ ✓ ✗ ✗ ✗ ✗ ✗ ✗ ✗

✓
✓
✗

✓ ✓✓ ✓ ✓ ✓✓ ✓
✓ ✓✓ ✓ ✓ ✓✓ ✓
✗ ✗ ✗ ✗ ✗ ✓ ✗ ✗

✓ ✓ ✓ ✓
✓ ✓ ✗ ✗
✗ ✗ – ✗

✓
✓
✓
✓
✓

✓
✓
✓
✓
✓

✓ ✓ ✓ ✓
✓ ✓ ✓ ✓
✓ ✓ ✓ ✓
✓ ✓ ✓ ✓
✓ ✓ ✓ ✓

✓
✓
✓
✓
✓

✓
✓
✓
✓
✓

✓
✓
✓
✓
✓

✓
✓
✓
✓
✓

✓
✓
✗
✓
✓

✓✓ ✓ ✓ ✓✓
✓✓ ✓ ✓ ✓✓
✗ ✗ ✗ ✗ ✗ ✗
✓✓ ✓ ✓ ✓✓
✓✓ ✓ ✓ ✓✓

✓
✓
✗
✓
✓

✓ ✓ ✓ ✓
✓ ✓ ✓ ✓
✗ ✗ – ✗
✓ ✓? ✓
✓ ✓ ✗ ✗

✓

✓ ✓? ✗ ✓✓✓ ✗ ✓✓✓✓✓ ✗ ? ✗ ? ✓ ✗ ? ✓ ✓ ✗ ✓

✓

✓ ✗ ✗ ✗ ✗ ✗ ✗

✗

TRE
Auto ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗ ✗
mrab-regex BT ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✓ ✓

✗
✓

✓ ✓ ✓ ✓
✓ ✓ ✓ ✓

regexp2
BT ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗ ✓
fancy-regex Hyb ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✗ ✗ ✗ ✓

✓
✓

✓ ✓ ✓ ✓†
✓ ✓ ✓ ✓

✓†
✓

Auto ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✗ ✗ ✗ ✗
sregex
tiny-regex-c BT ✓ ✓ ✗ ✗ ✗ ✗ ✗ ✓ ✓ ✓ ✗ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗ ✗

✗
✗

✓ ✓ ✓ ✗
✓? ✓ ✗

✗
✗

PHP𝑏 preg_*
Ruby𝑐 Regexp
Shell [[ =∼ ]]

✓ ✓ ✗ ✓✓✓ ✗ ✓✓ ✗ ✗ ✗
✓ ✓ – ✓✓✓ – ✓✓✓ ✗ ✓
✓ ✓ ✗ ✓✓✓✓ ✓✓✓✓✓
✓ ✓ – ✓✓✓ – ✓✓✓ ✗ ✓

✓
✓

Third-Party Libraries
PCRE
Hyb ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
PCRE2
Hyb ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
RE2
Auto ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✗ ✗
Oniguruma BT ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Onigmo
BT ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✗ ✓ ✓
Hyperscan

Hyb ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✗ ✗ ✗ ✓† ✓† ✓ ✓ ✓ ✓

✗
✓

✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓
✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓
✓ ✓ – ✓✓✓ – ✓✓✓ ✗ ✓
✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓
✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓

✓
✓
✗
✓
✓

✓ ✓ ✓ ✓✓ ✓ ✓
✓ ✓ ✓ ✓✓ ✓ ✓
✓ ✗ ✓ ✓ ✗ ✓ ✗
✓ ✓ ✓ ✓ ✗ ✓ ✓
✓ ✓ ✓ ✓ ✗ ✓ ✓

✓
✓
✓
✓
✓

✓
✓
✗
✗
✗

✓ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✓ ✗ ✗ ✓ ?‡ ✗ ?‡ ?‡ ✗ ✗ ✗ ✗ ✗ ✓ ✗ ✗
✓ ✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓ ✓ ✓ ✓ ✓ ✓✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✓✓ ✓ ✓ ✓✓ ✓
✓ ✓ ✓ ✗ ✓✓✓✓ ✓✓✓✓✓ ✓ ✓ ✗ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓
✓ ✓ ✓ ✓✓✓✓✓ ✓✓✓✓✓ ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✓ ✓ ✗ ✓

✓
✓

✗ ✗ ✗ ✗
✗ ✗ ✗ ✗
✓ ✓ ✓ ✗

✓ ✓✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✗ ✗
✓ ✓✓ ✓ ✓ ✓✓ ✓ ? ✓? ✗

✓ ✓ ✓ – ✓ ✓ ✓ – ✓ ✓ ✓ ✓ ✓ ✗ ✓ ✗ ✓ ✓ ✗ ✗ ✗ ?‡ ✗ ?‡ ?‡ ✗ ✗ ✗ ✗ ✗ ✗ ✗
✗ ✓ ✗ ✗ ✗ ✗ ✗ ✗ ✓✓ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗ ✗

✗
✗

✗ ✗ – ✗
✗ ✗ ✗ ✗

✓=Yes, ✗=No, ?=Partial (i.e., implementation is incomplete, buggy, or feature deliberately ignored), –=N/A (non-backtracking engine cannot support).

† This feature is not enabled by default and requires explicit opt-in flags to be passed to the regex engine. ‡ The same functionality can be achieved via another construct/API.
𝑎 Go’s regexp and Rust’s regex crate are based on RE2’s design and syntax. 𝑏 PHP’s preg_* functions wrap PCRE2. 𝑐 Ruby’s Regexp is based on Onigmo, but Ruby maintains a separate fork in its source tree.

Phase 2: Design and Evaluation of the ReTest Framework. Based
on the findings from RQ1 and RQ2, we propose ReTest. This framework is designed to close these gaps by combining grammar-aware
fuzzing (to address coverage limitations) with metamorphic testing
(to provide a dialect-independent semantic oracle).
RQ3 How can we achieve a systematic, dialect-independent testing
framework for regex engines? We assess the design and efficacy
of ReTest through three sub-questions:
RQ3a Can we define a catalog of metamorphic relations that
serve as effective semantic oracles?
RQ3b How does ReTest compare to other baselines?
RQ3c Is the framework capable of replicating previous bugs
and discovering new bugs in regex engines?

4

Testing Practices in Regex Engines

Many programming languages provide built-in regex support through
their standard libraries, while others delegate to external opensource implementations. To characterize the contemporary regex
engine landscape, we conducted a systematic engineering review
of 22 widely used engines across these two categories.
Methodology. For first-party engines (standard library implementations), we surveyed the top 10 programming languages by GitHub
push activity [9]: Python, Java, JavaScript, TypeScript, C, C++, PHP,
Go, Ruby, and C#. After consolidating languages that share the
same compiler or runtime infrastructure (i.e., JavaScript/TypeScript
and C/C++), the list reduced to eight distinct engines; we therefore
added Shell and Rust, the next highest-ranked languages, to obtain

10 first-party engines. For third-party engines, we surveyed opensource regex engines on GitHub with at least 500 stars, yielding 11
additional engines: PCRE2, RE2, Oniguruma, Onigmo, Hyperscan,
TRE, mrab-regex, regexp2, fancy-regex, sregex, and tiny-regex-c.
We additionally included PCRE’s first version, given its historical
influence on regex semantics.
To compile a representative feature set, we aggregated regex constructs referenced in at least two of four authoritative sources [21,
31, 47, 57], ensuring coverage of commonly expected regex features. We then verified the presence or absence of each construct
in every engine by consulting official documentation and, where
documentation was ambiguous or incomplete, inspecting source
code directly.
Table 1 summarizes our review across 68 syntactic features spanning 13 categories. We emphasize that this survey captures feature
availability based on syntax support; even when two engines support the same construct, subtle semantic differences may exist [20].

4.1

The Testing Challenge

Current literature provides no guidance on “how should we test
regex engines?” or “how people test regex engines?”. To understand
the state of practice, we conducted an engineering review of the
testing infrastructure in the 22 regex engines from §2.1.
Methodology. For each engine, we inspected the test suite in its
source repository and recorded all distinct testing strategies employed. From the full set of observed techniques, we retained those

<!-- page 4 -->

JAWs 2026, April 2026, Rio de Janeiro, Brazil

Çakar, Lee, and Davis

Boundary

Integration

Exception Safety

Concurrency

Negative

✗
✗
✗
✓
✓
✓
✓
✗
✗
✗

✗
✗
✓
✗
✗
✗
✓
✗
✗
✗

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✗
✗
✗
✗
✗
✗
✗
✗
✓
✓

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✗
✗
✗
✓
✗
✗
✗
✗
✗
✗

✓
✓
✓
✗
✗
✗
✗
✗
✓
✗

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✗
✓
✓
✓
✗
✗
✓
✗
✓
✓
✗
✗

✓
✓
✓
✗
✗
✓
✗
✗
✓
✓
✓
✓

✗
✗
✓
✗
✗
✗
✗
✗
✗
✓
✗
✓

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✓
✓
✓
✓
✗
✗
✓
✗
✗
✗
✓
✗

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✗
✗
✗
✗
✗
✗
✗
✗
✗
✗
✗
✗

✗
✗
✓
✗
✗
✗
✗
✗
✗
✗
✗
✗

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

Differential

✗
✗
✓
✗
✗
✗
✓
✓
✗
✗

Fuzzing

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

Regression

Engine

Memory Safety

Crash Oracles. Fuzzing paired with runtime sanitizers effectively
detects memory safety violations and undefined behavior. When
the engine crashes or a sanitizer reports an error, the oracle is
clear: a bug exists. However, crash oracles cannot detect semantic
bugs—incorrect match results that do not trigger crashes.

Performance

Table 2: Testing techniques employed by 22 regex engines,
among those used in ≥ 2 engines.
Property-Based

precise, this approach scales poorly: exhaustively specifying expected behavior for all combinations of regex features and edge
cases is infeasible.

Unit

that appear in at least two engines to capture common practice,
yielding 12 techniques. Table 2 summarizes our findings.

First-Party (Language Standard Library)
Python re
Java j.u.regex
JS/TS Irregexp
C++ <regex>
Go regexp
C# S.T.Regex
Rust regex
PHP preg_*
Ruby Regexp
Shell glibc

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

Third-Party Libraries
PCRE
PCRE2
RE2
Oniguruma
Onigmo
Hyperscan
TRE
mrab-regex
regexp2
fancy-regex
sregex
tiny-regex-c

✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓
✓

✓=Present, ✗=Not present or not explicitly documented.

Observations. Six techniques—unit testing, regression testing,
performance testing, boundary testing, integration testing, and negative testing—are universally adopted across all 22 engines. While
foundational, these techniques are among the standard testing practices for software projects; they demand substantial manual effort
to achieve comprehensive coverage in the context of regex features
and edge cases.
Differential testing is employed by 11 engines (50%) as an attempt
to establish a semantic oracle. When an engine’s output diverges
from a reference implementation, developers investigate whether
the discrepancy indicates a bug. PCRE is the most widely used
oracle (used by RE2, Hyperscan, sregex, and regexp2), followed by
Perl (used by PCRE, PCRE2 and sregex).
Fuzzing is employed by 10 engines (45%) to maximize code coverage and discover edge cases that manual testing misses. However, the sophistication of fuzzing approaches varies considerably.
Only two engines employ grammar-aware fuzzing: JS/TS’s Irregexp and Rust’s regex crate. Most engines rely instead on mutationbased fuzzing (e.g., via LibFuzzer [55], OSS-Fuzz [29]) that generates
inputs through random byte-level mutations. Without structural
awareness, such mutations are unlikely to produce syntactically
valid patterns, limiting coverage of matching logic.
4.1.1 The Oracle Problem. The test oracle problem [4] refers to the
difficulty of determining whether a program’s output is correct. For
regex engine testing, different testing techniques employ different
oracles. Here, we summarize our observations.
Developer-Defined Oracles. Unit tests and regression tests rely
on manually specified expected outputs. Developers write assertions such as “pattern a+ on input aaa should match aaa.” While

Differential Oracles. Differential testing uses a reference implementation as the oracle: if the engine under test produces different
output than the reference, a potential bug is flagged.
However, as discussed in §2.1, regex engines implement different dialects with intentional semantic differences. Consequently,
differential testing generates many false positives—discrepancies
that reflect expected dialect divergence rather than bugs.
Specifically, Davis et al. [18] cataloged three classes of such
divergence: (1) same syntax mapped to different features (e.g., \h
matches a tab in PCRE but the literal "h" in Java), (2) same syntax
with different semantics (e.g., ^a on "x\na" matches in Ruby but not
JavaScript), and (3) subtle behavioral differences in shared features
(e.g., ((a*)+) on "aa" captures "" in Java but "aa" in Python).
In all three cases, a true semantic bug would be indistinguishable from expected dialect variation without deep knowledge and
manual triage. This limitation motivates our use of metamorphic
testing (§6.2): we need an oracle that validates the semantics within
a single engine, independent of cross-implementation comparison.
Our Preliminary Answer to RQ1
Regex engines rely on manual testing (unit, regression, boundary, and negative tests). Differential testing is used by 50% of engines but suffers from dialect divergence. Fuzzing is employed
by 45% of engines, yet only 2 of 22 engines employ grammaraware fuzzing for valid inputs. Overall, current practices lack
systematic input generation and a dialect-independent semantic oracle.

5

Bugs and Vulnerabilities in Regex Engines

We analyze regex engine bugs from their issue trackers (§5.1) and
security vulnerabilities from CVE records (§5.2).

5.1

Review of Regex Engine Bugs

To understand the prevalence and nature of bugs in regex engines,
we conducted a systematic review of issues from 11 of the thirdparty regex engine implementations from §2.1.2 At the current
stage of our research, we focus on third-party engines. Our future
work (§8) will also include first-party regex engines and examine
failure modes more deeply.
5.1.1 Methodology. We collected 2,342 issue reports across the 11
engines via the GitHub API and classified each using an LLM-based
analysis pipeline using OpenAI’s gpt-5.1-mini model. Then, we
manually verified that each classification output conformed to our
predefined schema. The system prompt we used for classifying
regex engine issue reports is available in our artifact (§9).
2 PCRE’s issue tracker is no longer available to the public. Hence, we excluded it.

<!-- page 5 -->

Towards the Systematic Testing of Regular Expression Engines

For each issue, our classification pipeline determines: (1) whether
it represents a real bug—a confirmed defect in the engine’s implementation, excluding user misunderstandings, feature requests,
and API misuse; (2) the bug type: semantic (incorrect match results, whether identified against a specification, a prior version,
another engine, or user expectations), crash (abnormal termination,
e.g., panics, uncaught exceptions), memory (leaks, buffer errors),
performance (catastrophic backtracking, timeouts), documentation
(incorrect or misleading docs), or other (confirmed bugs not fitting prior categories, e.g., build failures, installation issues); and
(3) the testing strategy used to discover the bug (e.g., manual review
(user-reported), fuzzing, differential testing).
5.1.2 Results. Of the 2,342 issues analyzed, our classification pipeline
labeled 1,007 as real bugs (43.0%), spanning issue reports from 2011
to 2025 (91.8% of the issue reports are from between 2015 and 2025).
The proportion classified as bugs varies by engine, from 8.7% in
sregex to 59.3% in Oniguruma. Detailed per-engine statistics are
available in our artifact (§9).
Figure 1 shows the distribution of bug types across engines. The
three most common bug categories overall are semantic bugs at
35.3% (355), followed by crash bugs at 14.3% (144) and memoryrelated bugs at 11.4% (115). While semantic bugs form the largest
single category across all engines combined, the distribution varies
by engine; for instance, RE2 and Hyperscan show higher proportions of memory and crash bugs.

5.2.1 Methodology. For each regex engine listed in §2.1 (both firstparty and third-party), we queried the NVD API for CVEs that
mention the engine, capturing both CVEs assigned directly to the
engine and CVEs assigned to downstream projects affected by defects in the engine itself. Next, we manually investigated the query
results and manually removed the false positive matches (e.g., CVEs
from an unrelated software also named RE2).
5.2.2 Results. We identified 156 CVEs across 15 of the 22 surveyed
engines, spanning the period 2005–2025. In terms of severity levels,
32 CVEs (21%) were rated critical and 45 (29%) were rated high,
meaning over half of all regex engine CVEs pose significant security
risks. The remaining vulnerabilities are rated medium severity (19%)
or lack a CVSS score (31%).
PCRE
PHP
Python
Oniguruma
PCRE2
Ruby
Bash
JS/TS
TRE
Onigmo
Java
Hyperscan
Rust
Go
C#
mrab-regex
C/C++
sregex
fancy-regex
tiny-regex-c
regexp2
RE2
0

mrab-regex
PCRE2
Oniguruma
RE2
Hyperscan
Onigmo
TRE
fancy-regex
tiny-regex-c
regexp2
sregex

10

20

30

40

Number of CVEs

Memory Safety
ReDoS

50

Code Injection
Other

Figure 2: Distribution of CVE types across regex engines.

0

50

SEMANTIC
CRASH

100 150 200 250 300

Number of Bugs
MEMORY
PERF

DOC
OTHER

Figure 1: Distribution of bug types across 11 third-party regex
engines. Semantic bugs form a plurality (35.3%) of all bugs.
According to our analysis, 82.1% of bugs were reported by users,
while only 6.3% were discovered via fuzzing and 5.4% through
differential testing.3 This discovery gap highlights an opportunity
for automated testing to uncover bugs before users encounter them
in production. 55.5% of the identified bugs were subsequently fixed.

5.2

JAWs 2026, April 2026, Rio de Janeiro, Brazil

Review of Regex Engine CVEs

To complement our bug study with an analysis of security vulnerabilities, we collected CVEs (Common Vulnerabilities and Exposures)
associated with regex engines from the NIST National Vulnerability
Database (NVD).
3 Bugs found internally by engine developers may be fixed without public issue reports.

However, we assume significant defects—whether found internally or externally—to
be documented in issue trackers.

Figure 2 shows the distribution of CVE types across engines,
categorized by their CWE (Common Weakness Enumeration) classifications. Memory safety vulnerabilities dominate, accounting
for 52% of all CVEs. These include buffer overflows (CWE-119),
out-of-bounds reads (CWE-125), out-of-bounds writes (CWE-787),
integer overflows (CWE-190), and use-after-free errors (CWE-416).
ReDoS-related vulnerabilities (CWE-1333, CWE-400) constitute 17%
of CVEs. PCRE alone accounts for 50 CVEs (32% of the total). PHP’s
PCRE bindings contribute an additional 27 CVEs, many related to
how PHP exposes PCRE functionality. Seven engines in our survey
have zero reported CVEs.
As motivation, these findings indicate two things. First, memory
safety is the dominant vulnerability class in regex engines, and
should be a target of any testing approach. Second, “where there
is smoke, there is fire”—the abundance of CVEs in the PCRE and
PCRE2 engines suggests that systematic testing of other widelydeployed libraries could yield significant security benefits.
Our Preliminary Answer to RQ2
Semantic bugs are the most common defect type (35%) overall,
followed by crashes (14%) and memory errors (11%). 82% of
bugs are user-reported; only 6% are found via fuzzing. Among
156 CVEs, memory safety vulnerabilities dominate (52%), with
50% rated high or critical severity. PCRE alone accounts for 32%
of regex engine CVEs.

<!-- page 6 -->

JAWs 2026, April 2026, Rio de Janeiro, Brazil

6

Design of ReTest

ReTest is a systematic testing framework for regex engines that
combines two complementary techniques: (1) grammar-aware fuzzing for generating syntactically valid test inputs that achieve high
code coverage, and (2) metamorphic testing for providing a dialectindependent semantic oracle. Figure 3 illustrates the high-level
architecture.
Our utilization of these two techniques is motivated by two findings from our empirical analysis. First, the choice of a fuzzing-based
architecture is necessitated by the expansive grammar of modern
regex engines (§2.1); the combinatorial explosion of feature interactions renders exhaustive manual verification cost-prohibitive.
This design choice aligns with the current practices, as ∼50% of
the engines surveyed in §4 already employ fuzzing to mitigate this
scalability challenge. However, these implementations predominantly rely on naive, byte-level mutation strategies; consequently,
generated inputs frequently violate syntax rules and are rejected
early by the parser, leaving matching logic unexplored. Second,
the oracle problem (§4.1.1) limits the utility of differential testing:
dialect divergences may lead to false-positive alerts, necessitating
a dialect-independent verification strategy.

6.1

Table 3: Metamorphic relations implemented in ReTest,
curated from [34, 52]. All relations apply to K-regexes
(Kleene fragments). 𝜀 denotes the empty string, and ∅ denotes an always-failing pattern (e.g., (?!)). “Assertion” indicates whether the oracle checks match existence (match) or
also that match spans are identical (match+span).
Category

Relation

Equivalence

Assertion

Alternation

Associativity
Commutativity
Idempotence
Zero Identity

𝑟 1 | (𝑟 2 |𝑟 3 ) ≡ (𝑟 1 |𝑟 2 ) |𝑟 3
𝑟 1 |𝑟 2 ≡ 𝑟 2 |𝑟 1
𝑟 ≡ 𝑟 |𝑟
𝑟 |∅ ≡ 𝑟

match
match
match
match

Concatenation

Associativity
One Identity
Zero Identity

(𝑟 1𝑟 2 )𝑟 3 ≡ 𝑟 1 (𝑟 2𝑟 3 )
𝑟 ·𝜀 ≡𝑟
𝑟·∅≡∅

match
match
no match

Distributivity

Left
Right

𝑟 1 (𝑟 2 |𝑟 3 ) ≡ 𝑟 1𝑟 2 |𝑟 1𝑟 3
(𝑟 1 |𝑟 2 )𝑟 3 ≡ 𝑟 1𝑟 3 |𝑟 2𝑟 3

match
match

Kleene Star

Left Unrolling
Right Unrolling
Collapse
Expand

𝑟 ∗ ≡ (𝜀 |𝑟𝑟 ∗ )
𝑟 ∗ ≡ (𝜀 |𝑟 ∗𝑟 )
(𝑟 ∗ ) ∗ ≡ 𝑟 ∗
𝑟 ∗ ≡ (𝑟 ∗ ) ∗

match
match
match+span
match+span

Star Laws

Sum-Star Left
Sum-Star Right
Product-Star

(𝑟 1 |𝑟 2 ) ∗ ≡ 𝑟 1∗ (𝑟 2𝑟 1∗ ) ∗
(𝑟 1 |𝑟 2 ) ∗ ≡ (𝑟 1∗𝑟 2 ) ∗𝑟 1∗
(𝑟 1𝑟 2 ) ∗ ≡ 𝜀 |𝑟 1 (𝑟 2𝑟 1 ) ∗𝑟 2

match
match
match

Input Generation

Regex engine testing requires generating two types of inputs: (1) regex patterns that exercise diverse engine code paths, and (2) input
strings that trigger matching behavior for those patterns. Our analysis of existing fuzzing harnesses (§4.1) shows that most regex
engines rely on randomly generated regexes and input strings,
which limits coverage of real code paths due to the high likelihood
of producing syntactically invalid inputs. ReTest addresses this
through corpus-based seeding with grammar-aware augmentation
and pattern-derived string synthesis.
6.1.1 Regex Pattern Generation. ReTest can draw seeds from a corpus of real-world regex patterns collected from production software.
We use a dataset of over 500,000 unique patterns extracted from
open-source projects and internet sources [12], filtered to retain
only patterns that compile successfully on the target engine. This
seeding strategy ensures that initial inputs reflect realistic usage
patterns and exercise code paths for production workloads.
6.1.2 Input String Generation. Given a regex pattern 𝑟 , ReTest
generates input strings to exercise the engine’s matching logic. Our
goal is to produce strings that approximate the language 𝐿(𝑟 ) and its
complement 𝐿(𝑟 ), ensuring coverage of both accepting and rejecting paths through the engine’s internal automaton. For this purpose,
we use EGRET [35] to generate a complete set of strings covering
all automaton edges, thereby providing coverage guarantees for
both matching and non-matching code paths.

6.2

Çakar, Lee, and Davis

Oracle Suite

As discussed in §4.1, one of the challenges in regex engine testing
is the oracle problem: “given a pattern 𝑟 and input 𝑠, how do we
determine whether the engine’s output is correct?” ReTest addresses
this through a multi-layered oracle suite combining metamorphic
testing for semantic correctness and runtime sanitizers for safety.

6.2.1 Metamorphic Relations. While different engines may disagree on specific behaviors, they must all satisfy certain algebraic
properties derived from Kleene algebra [32]. Rather than comparing
across engines, we validate that a single engine is consistent with
itself under semantics-preserving transformations. For this, we use
metamorphic relations (§2.3).
ReTest implements 16 metamorphic relations derived from
Kleene algebra, organized into five categories. We compile these
relations from Salomaa [52] and Kozen [34]. Table 3 presents this catalog. These relations are implementation-independent: any formally
correct regex engine, regardless of dialect, must satisfy them. At
the current stage of our research, our MRs cover only the K-regex
fragment. Still, ReTest can apply them to K-regex sub-patterns
within E-regexes, enabling partial validation of extended patterns.
We are planning to expand our MR catalog for E-regexes too (§8).
Metamorphic Transformation and Validation. Each metamorphic
relation is implemented as an AST visitor that identifies applicable
patterns and applies the transformation. For example, we do not
apply Kleene star rules if 𝑟 does not use the ∗ operator. Algorithm 1
presents the algorithm.
Algorithm 1 Metamorphic Testing for Regexes
Require: Pattern 𝑟 , Input 𝑠, Engine 𝐸, Relations M R
Ensure: Bug report or ⊥
1: for each relation mr ∈ M R do
2:
if not mr.Precondition(𝑟 ) then continue
3:
𝑟 ′ ← mr.Transform(𝑟 )
4:
resultbase ← 𝐸.Search(𝑟, 𝑠 ); resultvariant ← 𝐸.Search(𝑟 ′ , 𝑠 )
5:
if not mr.Assert(resultbase , resultvariant ) then
6:
return Bug(mr, 𝑟 , 𝑟 ′ , 𝑠)
7:
end if
8: end for
9: return ⊥

<!-- page 7 -->

Towards the Systematic Testing of Regular Expression Engines

JAWs 2026, April 2026, Rio de Janeiro, Brazil

Figure 3: Overview of the ReTest framework. The input generation module seeds fuzzing and maintains a subtree pool for
grammar-aware AST mutations. The fuzzing loop generates test cases and tracks edge coverage to guide mutation. The oracle
suite combines runtime sanitizers for crash detection with metamorphic testing for semantic bug detection.
6.2.2 Sanitizer-Based Oracles. While metamorphic testing is effective at detecting semantic bugs (i.e., incorrect match results), it is
not intended to uncover undefined behaviors (e.g., crashes) or other
violations (e.g., memory-safety issues) that do not affect program
output. Together, these issues characterize the current landscape of
regex engine bugs and vulnerabilities, alongside semantic bugs, as
discussed in §5.
To detect safety violations, ReTest integrates three standard runtime sanitizers: AddressSanitizer (ASan) [54] and MemorySanitizer
(MSan) [56] and UndefinedBehaviorSanitizer (UBSan) [27] for undefined behaviors such as signed integer overflows and null pointer
dereferences, When a sanitizer detects a violation, this comprises a
crash oracle, accompanied by diagnostic information.

6.3

Guidance & Mutations

While corpus seeding and dictionary injection provide diverse initial inputs, systematic exploration of engine internals requires
coverage-guided mutation—evolving patterns based on feedback
about which code paths have been exercised. ReTest combines
coverage tracking with context-aware grammar-based mutation to
iteratively discover inputs that reach deep engine states.
6.3.1 Coverage Tracking. ReTest tracks edge coverage in regex
engine binaries. The coverage tracker maintains a bitmap of observed edges, updated after each engine execution. When a test
input triggers previously-unseen edges, it is marked as “interesting”
and used to guide subsequent mutation.
6.3.2 AST-Based Subtree Replacement. For deep exploration of engine internals, ReTest performs grammar-aware AST-based mutation on patterns that trigger new coverage. The mutator parses patterns into abstract syntax trees and performs structured modifications—
replacing subtrees with semantically compatible alternatives from
a learned pool.
Influenced from [59], each subtree in the pool is indexed by its
syntactic context—the types of its ancestor nodes in the original
AST. When selecting a replacement for a mutation target, ReTest
retrieves subtrees that appeared in similar contexts, ensuring semantic compatibility (e.g., a quantifier body is replaced with another
valid atom, not an anchor). Algorithm 2 presents the mutation

algorithm. An optimization we applied is copy-on-write AST manipulation: rather than deep-copying the entire tree for each mutation,
we copy only the path from root to the mutation target, leaving
untouched subtrees as shared references.
Algorithm 2 Grammar-Aware Mutation
Require: Pattern 𝑝, Subtree Pool P
Ensure: Mutated pattern 𝑝 ′
1: ast ← Parse(𝑝 )
2: nodes ← CollectInternalNodes(ast)
3: target ← RandomSelect(nodes)
4: ctx ← ExtractContext(target)
5: repl ← P.Get(Type(target), ctx)
6: ast′ ← Replace(ast, target, repl)
7: return Serialize(ast′ )

6.3.3 Coverage-Driven Subtree Pool. The subtree pool stores AST
fragments organized by node type and syntactic context, enabling
efficient lookup of compatible replacements. Rather than populating
the pool statically, ReTest updates it dynamically based on coverage
feedback: when a mutated pattern triggers new code paths, its
subtrees are extracted and added to the pool. This coverage-driven
approach ensures the pool accumulates “interesting” subtrees—
those that exercise diverse engine behaviors.

7

Preliminary Evaluation of ReTest

At the current prototyping stage of ReTest, we conducted a preliminary evaluation to explore its capabilities, particularly for RQ3b
(coverage and fault detection). We compare ReTest against two
baselines representing current practice in regex engine fuzzing.
Baselines. We compare three fuzzing strategies:
(1) B1: Naive Fuzzing. We use the OSS-Fuzz [29] harness for PCRE
with LibFuzzer’s default mutation strategy, which applies random bit-flips, byte insertions, and deletions without awareness
of regex syntax.
(2) B2: Grammar-Aware Fuzzing. We adapt the grammar-based
fuzzing approach from JS/TS (V8)’s Irregexp test suite, which
generates patterns by random production rule expansion and replaces AST subtrees with subtrees of the same node type. While

<!-- page 8 -->

JAWs 2026, April 2026, Rio de Janeiro, Brazil

Çakar, Lee, and Davis

this ensures syntactic validity, type-only matching ignores syntactic context and can produce semantically malformed patterns
(e.g., replacing a quantifier body with an anchor). This represents the state of practice for structure-aware fuzzing (§4).
(3) ReTest. Our approach bootstraps fuzzing with 1,000 seed patterns randomly sampled from our 500K regex corpus and applies
coverage-guided, context-aware AST mutation that indexes subtrees by their syntactic context (§6.3).
Setup. We evaluate all three strategies on PCRE v8.45, compiled
with ASan and UBSan. Each configuration runs for 1 hour on a
dedicated allocation of 4 cores from an Intel Xeon W-2295 CPU with
188GB RAM. We measure unique edges covered and the number of
unique bugs discovered.
Results. Figure 4 shows edge coverage over time.

Edge Coverage (%)

40
30
20
10
0

0

5

10
Naive

15

20

Time (minutes)
Grammar

25

30

ReTest

Figure 4: Edge coverage over time for three strategies on
PCRE v8.45.
ReTest achieves 40.12% edge coverage, compared to 12.66% for
V8-style grammar-aware fuzzing and 11.81% for OSS-Fuzz-style
naive fuzzing. ReTest’s corpus seeds provide an obvious advantage
of 29.4% near-immediate initial coverage, while other baselines start
near zero. Notably, ReTest’s context-aware mutations contribute
an additional ∼11 percentage points (from 29% to 40%), which is
comparable to the total coverage achieved by baselines—despite
diminishing returns at higher coverage levels where novel edges
become increasingly sparse.
The iteration counts reflect different efficiency characteristics.
Naive fuzzing executes ∼156M iterations; grammar-aware fuzzing
reduces this to ∼5.3M; ReTest executes ∼40K iterations. ReTest’s
coverage plateaus around minute 12, indicating room for improvement for the full submission of our work (§8).
Bugs Found. During our prototyping of ReTest, we discovered
three bugs in PCRE v8.45. Two patterns trigger heap memory corruption: one involving the \C escape sequence in UTF-8 mode
(\P{Z}+:gra]sh*:p(\CR)(?s)), and another combining \C with
property-like syntax (\p{Han}*\n*\t*\.\CP{}*\n*\t*body) (CWE122, CWE-787). Both cause heap corruption at compile-time or
JIT study-time. A third pattern involving zero-quantified recursion ((x{1,3}|\p{L}++|(([^>]*(?1){0}(?1)?)))+) triggers a
global buffer overflow during compilation (CWE-125). These bugs
are related to some known vulnerabilities but not the same (e.g.,
CVE-2015-2325, CVE-2015-2328). Neither of the other two baselines
revealed any bugs in our experiments. Detailed analysis is available
in our artifact (§9).

8

Future Work

We regard RQ1 (§4) and RQ2 (§5) as close to mature contributions.
We plan to expand our bug analysis to include first-party regex
engines, which requires filtering to isolate regex engine-specific
defects from broader runtime issues in repositories like CPython
and OpenJDK. We will also improve the soundness of our method
through sampling for deeper manual analysis. Additionally, we
will document the code coverage achieved by each testing strategy
observed in Table 2 (e.g., unit tests, regression tests) to quantify the
effectiveness of current practices and identify coverage gaps that
ReTest can address.
For RQ3 (§6, §7), future work remains. Our metamorphic relation catalog (Table 3) currently covers only K-regexes. Drawing on
recent formalization efforts [8, 7, 14, 26], we will extend the catalog
to E-regex constructs, enabling validation of a broader class of patterns. Additionally, our preliminary evaluation of ReTest reveals
coverage plateaus at ∼40% edge coverage mark; we will investigate
the reasons in-depth and develop strategies to break through these
limitations. Our current conjecture is that automaton coverage is an
inadequate heuristic for selecting strings to accompany the regexes.
The full evaluation will address all RQ3 sub-questions comprehensively. For RQ3a, we will compare our metamorphic oracle
against differential testing, measuring false positive rates. For RQ3b,
we will scale fuzzing experiments to all engines with 24-hour campaigns, comparing ReTest against naive and grammar-aware baselines with statistical rigor. We will also measure ReTest’s and other
baselines’ ability to replicate known bugs and vulnerabilities from
our review (§5). For RQ3c, we will report all discovered bugs with
responsible disclosure and maintainer response tracking.
Finally, we will ablate the contributions of ReTest parameters,
e.g., the initial corpus size and subtree pool size.

9

Conclusion

Regular expression engines are critical infrastructure, yet our empirical analysis reveals significant testing gaps: 82% of bugs are
user-reported rather than caught by systematic testing, and existing approaches lack a dialect-independent semantic oracle. We presented ReTest, a framework that addresses these gaps by combining grammar-aware fuzzing with metamorphic testing grounded in
Kleene algebra. Our preliminary evaluation on PCRE v8.45 demonstrates that ReTest achieves 3× higher edge coverage than inpractice approaches and has already uncovered three memory safety
bugs. We plan to extend this work with a comprehensive evaluation across 22 regex engines, an expanded metamorphic relation
catalog for E-regex features, and ablation studies to guide future
regex engine testing research.

Data Availability
An artifact associated with this paper is available at https://github
.com/PurdueDualityLab/ReTest.

Acknowledgments
This work was supported by the US National Science Foundation
(NSF) under grants SaTC-2135156 and SaTC-2135157. We thank
August Shi for his valuable feedback at an early stage of this work.

<!-- page 9 -->

Towards the Systematic Testing of Regular Expression Engines

JAWs 2026, April 2026, Rio de Janeiro, Brazil

References
[1]

[2]

[3]
[4]
[5]

[6]

[7]
[8]

[9]
[10]

[11]
[12]

[13]
[14]

[15]
[16]

[17]
[18]

[19]

[20]

Muath Alkhalaf, Tevfik Bultan, and Jose L. Gallegos. 2012. Verifying client-side
input validation functions using string analysis. In Proceedings of the 34th
International Conference on Software Engineering (ICSE ’12). IEEE Press, Zurich,
Switzerland, 947–957. isbn: 9781467310673.
Cornelius Aschermann, Tommaso Frassetto, Thorsten Holz, Patrick Jauernig,
Ahmad-Reza Sadeghi, and Daniel Teuchert. 2019. Nautilus: fishing for deep
bugs with grammars. In Proceedings of the 26th Annual Network and Distributed
System Security Symposium (NDSS). Accessed: 2026-01-23. The Internet Society.
doi:10.14722/ndss.2019.23412.
Efe Barlas, Xin Du, and James C. Davis. 2022. Exploiting input sanitization for
regex denial of service. In Proceedings of the 44th International Conference on
Software Engineering (ICSE ’22).
Earl T. Barr, Mark Harman, Phil McMinn, Muzammil Shahbaz, and Shin Yoo.
2015. The oracle problem in software testing: a survey. IEEE Transactions on
Software Engineering, 41, 5, 507–525. doi:10.1109/TSE.2014.2372785.
Alberto Bartoli, Andrea De Lorenzo, Eric Medvet, and Fabiano Tarlao. 2016.
Inference of regular expressions for text extraction from examples. IEEE Transactions on Knowledge and Data Engineering, 28, 5, 1217–1230. doi:10 . 1109
/TKDE.2016.2515587.
Andrew Begel, Yit Phang Khoo, and Thomas Zimmermann. 2010. Codebook:
discovering and exploiting relationships in software repositories. In Proceedings
of the 32nd ACM/IEEE International Conference on Software Engineering - Volume
1 (ICSE ’10). Association for Computing Machinery, Cape Town, South Africa,
125–134. isbn: 9781605587196. doi:10.1145/1806799.1806821.
Martin Berglund, Brink Van Der Merwe, and Steyn Van Litsenborgh. 2021.
Regular Expressions with Lookahead. JUCS - Journal of Universal Computer
Science, 27, 4, 324–340. doi:10.3897/jucs.66330.
Martin Berglund, Brink Van Der Merwe, Bruce Watson, and Nicolaas Weideman.
2017. On the Semantics of Atomic Subgroups in Practical Regular Expressions.
In Implementation and Application of Automata. Vol. 10329. Arnaud Carayol
and Cyril Nicaud, (Eds.) Springer International Publishing, Cham, 14–26. isbn:
978-3-319-60133-5 978-3-319-60134-2. doi:10.1007/978-3-319-60134-2_2.
Fabian Beuke. 2024. GitHut 2.0 – GitHub language statistics. https://madnight
.github.io/githut/. Q1 2024 data, Accessed: 2025-01-20.
Masudul Hasan Masud Bhuiyan, Berk Çakar, Ethan H. Burmane, James C.
Davis, and Cristian-Alexandru Staicu. 2025. Sok: a literature and engineering
review of regular expression denial of service (redos). In Proceedings of the 20th
ACM Asia Conference on Computer and Communications Security (ASIA CCS
’25). Association for Computing Machinery, 1659–1675. isbn: 9798400714108.
doi:10.1145/3708821.3733912.
Janusz A. Brzozowski. 1964. Derivatives of Regular Expressions. Journal of the
Association for Computing Machinery, 11, 4, 481–494.
Berk Çakar, Charles M. Sale, Sophie Chen, Dongyoon Lee, and James C. Davis.
2025. Is reuse all you need? a systematic comparison of regular expression
composition strategies. https://arxiv.org/abs/2503.20579 arXiv: 2503.20579
[cs.SE].
Cezar Câmpeanu, Kai Salomaa, and Sheng Yu. 2003. A formal study of practical
regular expressions. en. International Journal of Foundations of Computer Science,
14, 06, 1007–1018. doi:10.1142/S012905410300214X.
Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras. 2025.
Verified and Efficient Matching of Regular Expressions with Lookaround. In
Proceedings of the 14th ACM SIGPLAN International Conference on Certified
Programs and Proofs (CPP ’25). Association for Computing Machinery, New
York, NY, USA, 198–213. isbn: 979-8-4007-1347-7. doi:10.1145/3703595.3705884.
Tsong Yueh Chen, Shing Chi Cheung, and Shiu Ming Yiu. 1998. Metamorphic
Testing: A New Approach for Generating Next Test Cases. Tech. rep. HKUSTCS98-01. Hong Kong University of Science and Technology.
Tsong Yueh Chen, Fei-Ching Kuo, Huai Liu, Pak-Lok Poon, Dave Towey, T. H.
Tse, and Zhi Quan Zhou. 2018. Metamorphic testing: a review of challenges
and opportunities. ACM Comput. Surv., 51, 1, Article 4, 27 pages. doi:10.1145/3
143561.
Russ Cox. 2025. RE2. https://github.com/google/re2.
James C Davis, Daniel Moyer, Ayaan M Kazerouni, and Dongyoon Lee. 2019.
Testing regex generalizability and its implications: a large-scale many-language
measurement study. In 2019 34th IEEE/ACM International Conference on Automated Software Engineering (ASE), 427–439. doi:10.1109/ASE.2019.00048.
James C. Davis, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee.
2018. The impact of regular expression denial of service (ReDoS) in practice:
an empirical study at the ecosystem scale. In Proceedings of the 2018 26th ACM
Joint Meeting on European Software Engineering Conference and Symposium
on the Foundations of Software Engineering (ESEC/FSE 2018). Association for
Computing Machinery, New York, NY, USA, 246–256. isbn: 978-1-4503-5573-5.
doi:10.1145/3236024.3236027.
James C. Davis, Louis G. Michael IV, Christy A. Coghlan, Francisco Servant,
and Dongyoon Lee. 2019. Why aren’t regular expressions a lingua franca? an

[21]
[22]
[23]
[24]
[25]
[26]

[27]
[28]
[29]
[30]

[31]
[32]
[33]
[34]
[35]
[36]

[37]

[38]

[39]

[40]

[41]

[42]

[43]

empirical study on the re-use and portability of regular expressions. In Proceedings of the 2019 27th ACM Joint Meeting on European Software Engineering
Conference and Symposium on the Foundations of Software Engineering.
James Collins Davis. 2020. On the Impact and Defeat of Regular Expression
Denial of Service. Ph.D. Dissertation. Virginia Tech, Blacksburg, Virginia, USA.
Aryaz Eghbali and Michael Pradel. 2020. No Strings Attached: An Empirical
Study of String-related Software Bugs. In 2020 35th IEEE/ACM International
Conference on Automated Software Engineering (ASE), 956–967.
Python Software Foundation. 2026. Re – regular expression operations - python.
https://docs.python.org/3/library/re.html.
Free Software Foundation. 2025. Regexps — gnu emacs manual. Accessed: 202601-23. GNU Project. https://www.gnu.org/software/emacs/manual/html_node
/emacs/Regexps.html.
Jeffrey E. F. Friedl. 2002. Mastering Regular Expressions. (2nd ed ed.). O’Reilly,
Beijing ; Sebastopol, CA. isbn: 978-0-596-00289-3.
Hiroya Fujinami and Ichiro Hasuo. 2024. Efficient Matching with Memoization for Regexes with Look-around and Atomic Grouping. In Programming
Languages and Systems. Vol. 14577. Stephanie Weirich, (Ed.) Springer Nature
Switzerland, Cham, 90–118. isbn: 978-3-031-57266-1 978-3-031-57267-8. doi:10
.1007/978-3-031-57267-8_4.
Google. 2026. UndefinedBehaviorSanitizer. https://clang.llvm.org/docs/Undefi
nedBehaviorSanitizer.html.
Google. 2026. V8: regexp class reference. https://v8docs.nodesource.com/node25.0/d8/da7/classv8_1_1_reg_exp.html.
Google OSS-Fuzz Project Contributors. 2026. OSS-Fuzz PCRE2 project. https://g
ithub.com/google/oss-fuzz/tree/master/projects/pcre2. Accessed: 2026-01-26.
Christian Holler, Kim Herzig, and Andreas Zeller. 2012. Fuzzing with code
fragments. In 21st USENIX Security Symposium (USENIX Security 12). USENIX
Association, Bellevue, WA, 445–458. isbn: 978-931971-95-9. https://www.useni
x.org/conference/usenixsecurity12/technical-sessions/presentation/holler.
Jan Goyvaerts. 2025. Regular-expressions.info — regex flavor comparison. Accessed: 2026-01-24. regular-expressions.info. https://www.regular-expressions
.info/refflavors.html.
S. C. Kleene. 1951. Representation of events in nerve nets and finite automata.
Automata Studies, 3–41.
K. Kosako. 2024. oniguruma. https://github.com/kkos/oniguruma/graphs/contr
ibutors.
D. Kozen. 1994. A completeness theorem for kleene algebras and the algebra
of regular events. Inf. Comput., 110, 2, 366–390. doi:10.1006/inco.1994.1037.
Eric Larson and Anna Kirk. 2016. Generating Evil Test Strings for Regular Expressions. In 2016 IEEE International Conference on Software Testing, Verification
and Validation (ICST).
Vu Le, Mehrdad Afshari, and Zhendong Su. 2014. Compiler validation via equivalence modulo inputs. In Proceedings of the 35th ACM SIGPLAN Conference
on Programming Language Design and Implementation (PLDI ’14). Association for Computing Machinery, Edinburgh, United Kingdom, 216–226. isbn:
9781450327848. doi:10.1145/2594291.2594334.
Nuo Li, Tao Xie, Nikolai Tillmann, Jonathan de Halleux, and Wolfram Schulte.
2009. Reggae: automated test generation for programs using complex regular
expressions. In 2009 IEEE/ACM International Conference on Automated Software
Engineering, 515–519. doi:10.1109/ASE.2009.67.
Yeting Li, Zixuan Chen, Jialun Cao, Zhiwu Xu, Qiancheng Peng, Haiming Chen,
Liyuan Chen, and Shing-Chi Cheung. 2021. ReDoSHunter: A Combined Static
and Dynamic Approach for Regular Expression DoS Detection. In Proceedings
of the 30th USENIX Conference on Security Symposium.
Yeting Li, Shuaimin Li, Zhiwu Xu, Jialun Cao, Zixuan Chen, Yun Hu, Haiming
Chen, and Shing-Chi Cheung. 2021. TransRegex: Multi-modal Regular Expression Synthesis by Generate-and-Repair. In 2021 IEEE/ACM 43rd International
Conference on Software Engineering (ICSE).
Yeting Li et al. 2022. RegexScalpel: regular expression denial of service (ReDoS)
defense by Localize-and-Fix. In 31st USENIX Security Symposium (USENIX
Security 22). USENIX Association, Boston, MA, 4183–4200. isbn: 978-1-93913331-1. https://www.usenix.org/conference/usenixsecurity22/presentation/li-y
eting.
Wen-jie Lu, Zhicong Huang, Qizhi Zhang, Yuchen Wang, and Cheng Hong.
2023. Squirrel: a scalable secure Two-Party computation framework for training
gradient boosting decision tree. In 32nd USENIX Security Symposium (USENIX
Security 23). USENIX Association, Anaheim, CA, 6435–6451. isbn: 978-1-93913337-3. https://www.usenix.org/conference/usenixsecurity23/presentation/lu.
Muhammad Numair Mansur, Maria Christakis, and Valentin Wüstholz. 2021.
Metamorphic testing of datalog engines. In Proceedings of the 29th ACM Joint
Meeting on European Software Engineering Conference and Symposium on the
Foundations of Software Engineering (ESEC/FSE 2021). Association for Computing Machinery, Athens, Greece, 639–650. isbn: 9781450385626. doi:10.1145/346
8264.3468573.
Microsoft Corporation. 2025. Use regular expressions in visual studio. Accessed:
2026-01-23. Microsoft Learn. https://learn.microsoft.com/en-us/visualstudio/id
e/using-regular-expressions-in-visual-studio?view=visualstudio.

<!-- page 10 -->

JAWs 2026, April 2026, Rio de Janeiro, Brazil

[44]

[45]
[46]
[47]
[48]

[49]
[50]

[51]

[52]
[53]
[54]

Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub,
Margus Veanes, Tiki Wan, and Eric Xu. 2023. Derivative based nonbacktracking
real-world regex matching with backtracking semantics. Proc. ACM Program.
Lang., 7, PLDI, Article 148, 24 pages. doi:10.1145/3591262.
Rong Pan, Qinheping Hu, Gaowei Xu, and Loris D’Antoni. 2019. Automatic
repair of regular expressions. RFixer: a tool for repairing complex regular expressions using examples, 3, OOPSLA, 139:1–139:29. doi:10.1145/3360565.
PCRE2 Project. 2024. PCRE2 - Perl-Compatible Regular Expressions. https://w
ww.pcre.org/.
Philip Hazel. 2025. Pcre: perl compatible regular expressions - reference manual.
Accessed: 2026-01-24. PCRE Project. https://pcre.org/pcre.txt.
Manuel Rigger and Zhendong Su. 2020. Detecting optimization bugs in database
engines via non-optimizing reference engine construction. In Proceedings of
the 28th ACM Joint Meeting on European Software Engineering Conference and
Symposium on the Foundations of Software Engineering. ACM, Virtual Event
USA, 1140–1152. isbn: 978-1-4503-7043-1. doi:10.1145/3368089.3409710.
Manuel Rigger and Zhendong Su. 2020. Finding bugs in database systems via
query partitioning. OOPSLA 20 Artifact for "Finding Bugs in Database Systems
via Query Partitioning", 4, OOPSLA, 211:1–211:30. doi:10.1145/3428279.
Manuel Rigger and Zhendong Su. 2020. Testing Database Engines via Pivoted
Query Synthesis. In 14th USENIX Symposium on Operating Systems Design and
Implementation (OSDI 20), 667–682. isbn: 978-1-939133-19-9. Retrieved Sept. 21,
2025 from.
Olli Saarikivi, Margus Veanes, Tiki Wan, and Eric Xu. 2019. Symbolic regex
matcher. en. In Tools and Algorithms for the Construction and Analysis of Systems.
Tomáš Vojnar and Lijun Zhang, (Eds.) Springer International Publishing, Cham,
372–378. isbn: 9783030174620. doi:10.1007/978-3-030-17462-0_24.
Arto Salomaa. 1966. Two Complete Axiom Systems for the Algebra of Regular
Events. Journal of the ACM, 13, 1, 158–169. doi:10.1145/321312.321326.
Sergio Segura, Gordon Fraser, Ana B. Sanchez, and Antonio Ruiz-Cortés. 2016.
A survey on metamorphic testing. IEEE Transactions on Software Engineering,
42, 9, 805–824. doi:10.1109/TSE.2016.2532875.
Konstantin Serebryany, Derek Bruening, Alexander Potapenko, and Dmitry
Vyukov. 2012. Addresssanitizer: a fast address sanity checker. In Proceedings of
the 2012 USENIX Conference on Annual Technical Conference (USENIX ATC’12).
USENIX Association, Boston, MA, 28.

Çakar, Lee, and Davis

[55]
[56]

[57]

[58]
[59]
[60]

[61]

[62]

[63]

Kostya Serebryany. 2015. Libfuzzer – a library for coverage-guided fuzz testing.
https://llvm.org/docs/LibFuzzer.html. LLVM Project.
Evgeniy Stepanov and Konstantin Serebryany. 2015. Memorysanitizer: fast
detector of uninitialized memory use in c++. In 2015 IEEE/ACM International
Symposium on Code Generation and Optimization (CGO), 46–55. doi:10.1109
/CGO.2015.7054186.
The Open Group and IEEE. 2018. Regular Expressions — The Open Group Base
Specifications, Issue 7, 2018 Edition. Accessed: 2026-01-24; Part of POSIX.1 (IEEE
Std 1003.1-2017 / The Open Group Base Specifications). The Open Group &
IEEE. https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap
09.html.
Ken Thompson. 1968. Programming Techniques: Regular expression search
algorithm. en. Communications of the ACM, 11, 6, 419–422. Retrieved May 31,
2024 from https://dl.acm.org/doi/10.1145/363347.363387.
Junjie Wang, Bihuan Chen, Lei Wei, and Yang Liu. 2019. Superion: grammaraware greybox fuzzing. In 2019 IEEE/ACM 41st International Conference on
Software Engineering (ICSE), 724–735. doi:10.1109/ICSE.2019.00081.
Peipei Wang, Chris Brown, Jamie A. Jennings, and Kathryn T. Stolee. 2020.
An Empirical Study on Regular Expression Bugs. In Proceedings of the 17th
International Conference on Mining Software Repositories (MSR ’20). isbn: 978-14503-7517-7.
Peipei Wang, Chris Brown, Jamie A. Jennings, and Kathryn T. Stolee. 2022.
Demystifying regular expression bugs: A comprehensive study on regular
expression bug causes, fixes, and testing. en. Empirical Software Engineering,
27, 1, 21. doi:10.1007/s10664-021-10033-1.
Xuejun Yang, Yang Chen, Eric Eide, and John Regehr. 2011. Finding and understanding bugs in c compilers. In Proceedings of the 32nd ACM SIGPLAN
Conference on Programming Language Design and Implementation (PLDI ’11).
Association for Computing Machinery, San Jose, California, USA, 283–294.
isbn: 9781450306638. doi:10.1145/1993498.1993532.
Andreas Zeller, Rahul Gopinath, Marcel Böhme, Gordon Fraser, and Christian
Holler. 2024. Fuzzing with grammars. In The Fuzzing Book: Tools and Techniques
for Generating Software Tests. Accessed: 2026-01-23. CISPA Helmholtz Center
for Information Security. https://www.fuzzingbook.org/html/Grammars.html.
