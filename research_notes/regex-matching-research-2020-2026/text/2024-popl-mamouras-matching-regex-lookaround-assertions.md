---
file: ../pdfs/2024-popl-mamouras-matching-regex-lookaround-assertions.pdf
citation: "Efficient Matching of Regular Expressions with Lookaround Assertions — Mamouras, Chattopadhyay — POPL (PACMPL 8) 2024"
doi: 10.1145/3632934
source: "https://doi.org/10.1145/3632934 (publisher version, added manually)"
topics: [counting_and_derivatives, redos_backrefs_verification, semantics_theory_access]
pages: 31
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Efficient Matching of Regular Expressions with Lookaround
Assertions
KONSTANTINOS MAMOURAS, Rice University, USA
AGNISHOM CHATTOPADHYAY, Rice University, USA
Regular expressions have been extended with lookaround assertions, which are subdivided into lookahead and
lookbehind assertions. These constructs are used to refine when a match for a pattern occurs in the input text
based on the surrounding context. Current implementation techniques for lookaround involve backtracking
search, which can give rise to running time that is super-linear in the length of input text. In this paper,
we first consider a formal mathematical semantics for lookaround, which complements the commonly used
operational understanding of lookaround in terms of a backtracking implementation. Our formal semantics
allows us to establish several equational properties for simplifying lookaround assertions. Additionally, we
propose a new algorithm for matching regular expressions with lookaround that has time complexity 𝑂 (𝑚 · 𝑛),
where 𝑚 is the size of the regular expression and 𝑛 is the length of the input text. The algorithm works
by evaluating lookaround assertions in a bottom-up manner. Our algorithm makes use of a new notion of
nondeterministic finite automata (NFAs), which we call oracle-NFAs. These automata are augmented with
epsilon-transitions that are guarded by oracle queries that provide the truth values of lookaround assertions at
every position in the text. We provide an implementation of our algorithm that incorporates three performance
optimizations for reducing the work performed and memory used. We present an experimental comparison
against PCRE and Java’s regex library, which are state-of-the-art regex engines that support lookaround
assertions. Our experimental results show that, in contrast to PCRE and Java, our implementation does not
suffer from super-linear running time and is several times faster.
CCS Concepts: • Theory of computation → Regular languages; • Software and its engineering →
Semantics.
Additional Key Words and Phrases: regex, automata, lookahead, lookbehind, lookaround, Kleene algebra,
regex matching, regex engine
ACM Reference Format:
Konstantinos Mamouras and Agnishom Chattopadhyay. 2024. Efficient Matching of Regular Expressions
with Lookaround Assertions. Proc. ACM Program. Lang. 8, POPL, Article 92 (January 2024), 31 pages. https:
//doi.org/10.1145/3632934

1 INTRODUCTION
Since their introduction in the 1950s, regular expressions [Kleene 1956] and finite-state automata
[Rabin and Scott 1959] have found applications in numerous domains to describe patterns over
sequences. They have been used for the lexical analysis of programs [Johnson et al. 1968] during
compilation, the search of words and patterns in text editors [Thompson 1968], and bibliographic
search [Aho and Corasick 1975]. Regular patterns are also used in network security [Yu et al. 2006]
to search for intrusion signatures in network traffic, in bioinformatics [Roy and Aluru 2016] for
Authors’ addresses: Konstantinos Mamouras, Rice University, Houston, Texas, USA, mamouras@rice.edu; Agnishom
Chattopadhyay, Rice University, Houston, Texas, USA, agnishom@rice.edu.

This work is licensed under a Creative Commons Attribution 4.0 International License.
© 2024 Copyright held by the owner/author(s).
ACM 2475-1421/2024/1-ART92
https://doi.org/10.1145/3632934
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 2 -->

92:2

Konstantinos Mamouras and Agnishom Chattopadhyay

describing protein, RNA, or DNA sequences, and in runtime verification [Bartocci et al. 2018] for
specifying safety properties.
Classical regular expressions involve constructs for nondeterministic choice 𝑟 1 +𝑟 2 , concatenation
𝑟 1 · 𝑟 2 , and Kleene star 𝑟 ∗ (repetition of 𝑟 zero or more times). In practice, the syntax of regular
expressions is often extended with more constructs that offer convenience, such as character classes
for describing sets of letters/symbols (e.g., [𝑎𝑏] and [0−9]), the construct 𝑟 ? for indicating that the
pattern 𝑟 is optional, and Kleene plus 𝑟 + (repetition of 𝑟 at least once). The construct of bounded
repetition, which is written as 𝑟 {𝑚, 𝑛} and describes the repetition of 𝑟 from 𝑚 to 𝑛 times, can be
translated using concatenation and ? but makes regular expressions exponentially more succinct.
Prior work explores the algorithmic challenges of bounded repetition [Kong et al. 2022; Le Glaunec
et al. 2023]. In this paper, we focus on another commonly-used construct called lookaround, which
goes beyond classical regular expressions by allowing one to describe not only a pattern to search
for, but also the context in which the pattern should appear. There are two kinds of lookaround:
lookahead, which we write as (?> 𝑟 ), and lookbehind, which we write as (?< 𝑟 ). The lookahead (?> 𝑟 )
asserts that the text that lies ahead (i.e., in the “future” relative to the current position) matches the
pattern 𝑟 . Similarly, the lookbehind (?< 𝑟 ) asserts that the text that lies behind (i.e., in the “past”)
matches the pattern 𝑟 . For example, the regular expression (?< [ ␣ ]) [𝐴−𝑍 ] [𝑎−𝑧] + (?> [, ; ]) can be
used to search for the occurrence of a word starting with a capital letter (the character classes [𝐴−𝑍 ]
and [𝑎−𝑧] recognize capital and lowercase letters respectively) that is preceded by a space (indicated
with [ ␣ ]) and followed by a comma or a semicolon (indicated with [, ; ]). The use of lookaround
allows us to focus only on the word described by the subpattern [𝐴−𝑍 ] [𝑎−𝑧] + , without including
in the match the context that is specified by the lookbehind (?< [ ␣]) and lookahead (?> [, ; ]).
Lookaround assertions can also be used to express some forms of intersection. For example, a
string matches the pattern (?> .∗ [0−9].∗ )(?> .∗ [𝑎−𝑧].∗ )(?> .∗ [𝐴−𝑍 ].∗ )(?> .∗ [#&@].∗ ) (where . is
notation for the character class that accepts every symbol) iff it contains at least one decimal digit,
one lowercase letter, one uppercase letter, and one of the symbols of {#, &, @}.
The main computational problem for regular expressions is the pattern matching problem:
given a regular expression 𝑟 and input text 𝑤, find the locations in the input text that match
the regular expression. A slightly more restricted variant is the membership problem, which asks
whether the entire input text 𝑤 matches the pattern 𝑟 . Several different approaches have been
used for implementing regular pattern matching. These implementations are often called regex
engines. One common class of implementations relies on backtracking search. This approach is
used in the PCRE library [The PCRE2 Developers 2023] and the regex engines of many popular
programming languages, such as Java, Python, and JavaScript. Backtracking search suffers from
exponential running time (in the length of the input text) for certain regular expressions that
cause the exploration of a large number of paths. This exponential explosion of explored paths
is often referred to as catastrophic backtracking [Berglund et al. 2014]. There are also regexes for
which the running time is polynomial but super-linear (again, in the length of the input text). The
super-linear (in some cases exponential) running time of regex engines based on backtracking
is much worse than what can be achieved using standard automata-theoretic approaches. For
example, Thompson’s classical algorithm [Thompson 1968] is essentially an implementation based
on nondeterministic finite automata (NFAs). It has time complexity 𝑂 (𝑚 · 𝑛), where 𝑚 is the size of
the regular expression and 𝑛 is the length of the input text.
Many modern regex engines are based on deterministic finite automata (DFAs), or NFAs, or a
combination of both. This includes the widely used engines grep [2023], Google’s RE2 [2023], and
Intel’s Hyperscan [2023]. In contrast to backtracking-based regex engines, automata-based engines
offer strong guarantees of performance: the running time is linear in the length of the input text.
Algorithms based on DFAs are very fast. They perform 𝑂 (1) work per input symbol, because they
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 3 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:3

only need to perform a memory lookup in the DFA transition table for each input symbol. The
problem with DFA-based implementations is that the size of the DFA can be exponential in the size
of the regex in the worst case. NFAs, on the other hand, can be exponentially more succint than
DFAs. Moreover, every regular expression that uses only the classical regular combinators can be
translated into an NFA whose state space is linear in the size of the expression. The problem with
NFA-based algorithms, compared to DFA-based algorithms, is that they may need to perform Θ(𝑚)
computation steps for each input symbol.
The current state of affairs regarding the support of lookaround in regex engines is rather
disappointing. Existing automata-based regex engines (grep, RE2, Hyperscan) do not support
lookaround at all. While it is known that the membership problem for regular expressions with
lookaround can be solved using finite-state automata (see, for example, [Morihata 2012] and
[Miyazaki and Minamide 2019]), these automata are very large due to the succinctness of lookaround.
A DFA of doubly exponential size is needed in the worst case (and therefore an NFA of exponential
size). These observations suggest that simple solutions based on automata that encode the entire
pattern will suffer from high complexity (with respect to the size of the pattern). Berglund et al.
[2021] consider the construction of alternating finite automata (AFA) from regular expressions with
lookahead assertions. In this construction, the number of states of the AFA is linear in the size of the
regular expression. A consequence of this, which is not explicitly discussed in [Berglund et al. 2021],
is that membership can be decided in 𝑂 (𝑚 · 𝑛) time with a right-to-left pass over the input string
that simulates the AFA execution “in reverse”. This approach does not handle lookbehind assertions.
Moreover, while the simulation of AFA execution can decide membership, it is not applicable to
match extraction. This is because the states for lookaheads are not distinguished from the states for
the “main” part of the regex, and therefore a disambiguation policy cannot be expressed. Several
backtracking-based regex engines support general lookahead, but only very restricted forms of
lookbehind. For example, the widely used PCRE library only supports bounded lookbehind, which
can only refer to a bounded amount of “past” text. As we will see in Section 6, the use of lookaround
in existing backtracking-based engines can easily trigger catastrophic backtracking. This means
that there is currently no efficient implementation of lookaround in the context of regular
pattern matching. This is the main problem that we tackle in this paper. Our approach is fully
general in that we allow the arbitrary nesting of unrestricted lookahead and lookbehind assertions.
The key idea of our approach is that it is possible to decompose the overall computation for
pattern matching with lookaround. First, we simplify the problem by making the assumption that
all lookaround assertions can be resolved by oracles that know the truth values of both lookaheads
and lookbehinds at each position in the text. Under this assumption, we show that it is possible to
augment NFA-based algorithms with a special kind of 𝜀-transition that is guarded by a query to
an oracle. The second step of our approach is to replace the oracles by algorithms that compute
all necessary truth values. This is done in a bottom-up manner. Consider a sub-pattern 𝑟 ′ of the
overall pattern 𝑟 whose top-level operator is a lookaround and contains no other occurrence of
lookaround. We compute whether 𝑟 ′ matches the input text at each position. After this computation
is performed, then the sub-pattern 𝑟 ′ can be replaced by an oracle query that uses the pre-computed
truth values. By following this bottom-up “compute and replace” process we ensure that the answers
for each oracle query have already been computed and are therefore available to the algorithm.
We note that our algorithm is not streaming in the general case because it performs right-to-left
passes over the input text to deal with lookahead assertions efficiently. If the regular expression
contains no lookahead assertions, then the algorithm can deal with lookbehind in a streaming
manner (single left-to-right pass over the input text). Similarly, if there are no lookbehind assertions,
then the algorithm can deal with lookahead with a single right-to-left pass over the input text.

Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 4 -->

92:4

Konstantinos Mamouras and Agnishom Chattopadhyay

Main contributions. We make the following contributions in this paper:
(1) We present a formal semantics for lookaround using a satisfaction relation that relates a
string 𝑤, a location (interval) [𝑖, 𝑗] within 𝑤, and a regular pattern 𝑟 . We show that this is
equivalent to an algebraic semantics that generalizes the classical language interpretation
of regular expressions (without lookaround). This mathematical semantics complements
existing definitions of the lookaround constructs that are operational, i.e., defined in terms of
a backtracking matching algorithm.
(2) Using our formal semantics for lookaround, we prove that regular expressions with lookaround
satisfy the equivalence properties of Kleene algebra [Kozen 1994]. Moreover, we establish a
number of equivalences involving lookaround that can be used for simplifying patterns.
(3) We introduce the notion of regular expressions and 𝜀-NFAs with oracle queries as a way to
abstract away lookaround assertions. We call them oracle-regexes and oracle-NFAs respectively.
We provide an algorithm for matching such oracle-regexes, which can be understood as a
simulation of oracle-NFA semantics. The time complexity of this algorithm is 𝑂 (𝑚 · 𝑛), where
𝑚 is the size of the oracle-regex and 𝑛 is the length of the input text.
(4) We propose a recursive algorithm for matching regular expressions with lookaround. This
algorithm is based on the bottom-up decomposition approach described earlier and makes
essential use of the algorithm for oracle-regex matching. Its time complexity is 𝑂 (𝑚 · 𝑛),
where 𝑚 is the size of the regular expression and 𝑛 is the length of the input text.
(5) We introduce three performance optimizations to the aforementioned algorithm: (i) common
assertion elimination, (ii) one-pass unidirectional evaluation to reduce the memory footprint
to 𝑂 (𝑚), and (iii) approximation to avoid the computation of some lookaround assertions.
(6) We provide an experimental evaluation of a Rust implementation of our algorithm against
PCRE and Java’s regex library, state-of-the-art regex engines that support lookaround. Our
experiments show (i) that our performance optimizations provide a significant performance
benefit, and (ii) that, in contrast to PCRE and Java, our implementation does not suffer from
super-linear (in the length of the input text) time complexity. PCRE and Java have worse
performance than our implementation over the workloads that we consider.
The current work presents the first tool for matching regular expressions with lookaround that
provides strong worst-case complexity guarantees and has competitive performance against stateof-the-art regex engines.
2 SEMANTICS OF LOOKAROUND
In this section, we present a formal mathematical semantics for regular expressions with lookaround.
There is little prior work on the formal semantics of lookaround. The most common approach is
to define it operationally: the meaning of lookaround is described by the backtracking algorithm
that performs regex matching. This approach is, however, unsatisfactory because it conflates
the specification (which should be simple and easily understandable) with the implementation
(which can be very complex) and therefore does not allow one to formally prove the correctness
of the matching algorithm. A notable exception to this is the semantic treatment of lookahead
in [Miyazaki and Minamide 2019], where the authors use languages of string pairs to interpret
regular expressions. These semantic objects are, however, insufficient for giving a semantics in the
presence of both lookahead and lookbehind.
We provide two equivalent semantic perspectives. The first one is logical and employs a ternary
satisfaction relation, which relates a string 𝑤, a location [𝑖, 𝑗] within the string 𝑤, and a regular
expression 𝑟 . The second one is algebraic and uses an algebra of “match-languages” to interpret
the regular expressions. A match-language is a set of triples of the form (𝑤, 𝑖, 𝑗), which specify a
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 5 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:5

𝑤, [𝑖, 𝑗] |= 𝜀 ⇐⇒ 𝑖 = 𝑗
𝑤, [𝑖, 𝑗] |= 𝑝 ⇐⇒ 𝑗 = 𝑖 + 1 and 𝑝 (𝑤 (𝑖)) = 1
𝑤, [𝑖, 𝑗] |= 𝑟 1 + 𝑟 2 ⇐⇒ 𝑤, [𝑖, 𝑗] |= 𝑟 1 or 𝑤, [𝑖, 𝑗] |= 𝑟 2
𝑤, [𝑖, 𝑗] |= 𝑟 1 · 𝑟 2 ⇐⇒ there is 𝑘 with 𝑖 ≤ 𝑘 ≤ 𝑗 such that 𝑤, [𝑖, 𝑘] |= 𝑟 1 and 𝑤, [𝑘, 𝑗] |= 𝑟 2
𝑤, [𝑖, 𝑗] |= 𝑟 ∗ ⇐⇒ 𝑖 = 𝑗 or there is 𝑘 with 𝑖 < 𝑘 ≤ 𝑗 such that 𝑤, [𝑖, 𝑘] |= 𝑟 and 𝑤, [𝑘, 𝑗] |= 𝑟 ∗
𝑤, [𝑖, 𝑗] |= (?> 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑖, |𝑤 |] |= 𝑟
𝑤, [𝑖, 𝑗] |= (?≯ 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑖, |𝑤 |] ̸ |= 𝑟
𝑤, [𝑖, 𝑗] |= (?< 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [0, 𝑖] |= 𝑟
𝑤, [𝑖, 𝑗] |= (?≮ 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [0, 𝑖] ̸ |= 𝑟
Fig. 1. Formal semantics of regular expressions with lookaround (lookahead and lookbehind). The satisfaction
relation |= relates a string 𝑤 ∈ Σ, a location [𝑖, 𝑗] with 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 |, and a regular expression 𝑟 .

string 𝑤 and a location [𝑖, 𝑗] within it. For each regular construct, we define an associated semantic
operation for the algebra of match-languages.
We also consider a natural notion of equivalence between regular expressions, which essentially
amounts to equality of their denotations, and establish several useful equivalences. These include
the equivalences given by Kleene algebra [Kozen 1994], as well as several other equivalences for
simplifying lookaround assertions.
Definition 1 (Regular Expressions with Lookaround). Let Σ be an alphabet, and P be a set of
decidable predicates over Σ, i.e., functions of type Σ → B, where B = {0, 1}. The set LReg(Σ) of
regular expressions (regexes) with lookaround is defined by the following grammar:
𝑟, 𝑟 1, 𝑟 2 ::= 𝜀 |
𝑝∈P|
𝑟1 + 𝑟2 |
𝑟1 · 𝑟2 |
𝑟∗ |
(?> 𝑟 ) |
(?≯ 𝑟 ) |
(?< 𝑟 ) |
(?≮ 𝑟 )

[empty string]
[character class / predicate]
[nondeterministic choice / union]
[concatenation]
[Kleene star / iteration]
[positive lookahead]
[negative lookahead]
[positive lookbehind]
[negative lookbehind]

Expressions of the form (?> 𝑟 ) and (?≯ 𝑟 ) are called lookahead assertions. Similarly, expressions of
the form (?< 𝑟 ) and (?≮ 𝑟 ) are called lookbehind assertions. Expressions of the form (?> 𝑟 ), (?≯ 𝑟 ),
(?< 𝑟 ) and (?≮ 𝑟 ) are collectively referred to as lookaround assertions.
We write |𝑤 | to denote the length of a string 𝑤. The empty string (i.e., the string of length 0) is
denoted by 𝜀. For 𝑤 ∈ Σ∗ , we will call a pair [𝑖, 𝑗] with 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 | a location in 𝑤. A position
in |𝑤 | is an index in the range 0, 1, . . . , |𝑤 |. We write 𝑤 (𝑖) for the letter at location [𝑖, 𝑖 + 1] in 𝑤.
Definition 2 (Formal Semantics of Lookaround). Let 𝑤 ∈ Σ∗ be a string, 𝑖 and 𝑗 be integers
satisfying 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 |, and 𝑟 ∈ LReg(Σ) be a regular expression. We write 𝑤, [𝑖, 𝑗] |= 𝑟 to
denote that the regular expression 𝑟 has a match in 𝑤 at the location [𝑖, 𝑗]. The relation |= is defined
by induction as shown in Fig. 1. The characteristic function 𝜒 of |= is defined as follows:
(
1, if 𝑤, [𝑖, 𝑗] |= 𝑟
𝜒 (𝑤, [𝑖, 𝑗], 𝑟 ) =
0, if 𝑤, [𝑖, 𝑗] ̸ |= 𝑟 .
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 6 -->

92:6

Konstantinos Mamouras and Agnishom Chattopadhyay

We also define ⟦𝑟 ⟧ = {𝑤 ∈ Σ∗ | 𝑤, [0, |𝑤 |] |= 𝑟 }. That is, ⟦𝑟 ⟧ is the set of all strings that match 𝑟 .
Notice in Fig. 1 that lookaround assertions can only hold at locations of the form [𝑖, 𝑖], i.e.,
locations of length 0. Such locations are essentially positions in the string. For this reason, we can
think of lookaround assertions as holding (or not) at positions within the string.
A decomposition of a location [𝑖, 𝑗] (where 𝑖 ≤ 𝑗) is a nonempty finite sequence of locations
[𝑖 1, 𝑗1 ], [𝑖 2, 𝑗2 ], . . . , [𝑖𝑛 , 𝑗𝑛 ] with 𝑖 1 = 𝑖, 𝑗𝑛 = 𝑗, and 𝑗𝑘 = 𝑖𝑘+1 for every 𝑘 = 1, 2, . . . , 𝑛 − 1.
Claim 3. Let 𝑟 ∈ LReg(Σ), 𝑤 ∈ Σ∗ , and 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 |. Then, 𝑤, [𝑖, 𝑗] |= 𝑟 ∗ iff 𝑖 = 𝑗 or there exists
a decomposition [𝑖 1, 𝑗1 ], [𝑖 2, 𝑗2 ], . . . , [𝑖𝑛 , 𝑗𝑛 ] of [𝑖, 𝑗] such that 𝑤, [𝑖𝑘 , 𝑗𝑘 ] |= 𝑟 for every 𝑘 = 1, . . . , 𝑛.
Proof. The left-to-right direction is shown by induction on 𝑗 − 𝑖. For the right-to-left direction,
we argue by induction on the size 𝑛 ≥ 1 of the decomposition. There is one important observation
in the proof for the step case: if the last pair [𝑖𝑛+1, 𝑗𝑛+1 ] of the decomposition is of length 0 (that is,
𝑖𝑛+1 = 𝑗𝑛+1 ), then it can be removed.
□
Claim 3 serves as a sanity check for the semantic definition of Fig. 1 for Kleene star. Notice that
in the definition of 𝑤, [𝑖, 𝑗] |= 𝑟 ∗ we consider 𝑟 matching only over locations of length at least 1
(we require that 𝑖 < 𝑘 in the location [𝑖, 𝑘]). Claim 3 establishes that this restriction is without loss
of generality.
Example 4. Consider the string 𝑤 = 𝑏𝑏𝑏𝑐𝑎𝑏𝑏𝑐𝑏𝑏𝑑𝑏𝑏𝑏𝑐 with |𝑤 | = 15. The regular expression
𝑟 1 = 𝑏𝑐 has 3 matches in 𝑤 at locations [2, 4], [6, 8], and [13, 15]. These matches are indicated below
by underlining the substrings corresponding to their locations:
𝑏𝑏𝑏𝑐𝑎𝑏𝑏𝑐𝑏𝑏𝑑𝑏𝑏𝑏𝑐
= (?< Σ∗𝑎Σ∗ )𝑏𝑐 has 2 matches in 𝑤 at locations [6, 8] and [13, 15]. Notice that

The expression 𝑟 1
an occurrence of 𝑏𝑐 within 𝑤 matches 𝑟 1 only if it is preceded by an occurrence of the letter 𝑎.
Similarly, the expression 𝑟 2 = 𝑏𝑐 (?> Σ∗𝑑Σ∗ ) has 2 matches in 𝑤 at locations [2, 4] and [6, 8]. An
occurrence of 𝑏𝑐 within 𝑤 matches 𝑟 2 only if it is followed by an occurrence of the letter 𝑑. Finally,
the expression 𝑟 3 = (?< Σ∗𝑎Σ∗ )𝑏𝑐 (?> Σ∗𝑑Σ∗ ) has 1 match in 𝑤 at location [6, 8].
Example 5. Suppose that 𝑎 1, 𝑎 2, . . . , 𝑎𝑛 are letters of the alphabet. The regular expression 𝑟𝑖 =
(?> Σ∗𝑎𝑖 Σ∗ ) asserts that there is some occurrence of 𝑎𝑖 from the current position to the end of the
string. Let us consider now the expression
𝑟 = 𝑟 1𝑟 2 · · · 𝑟𝑛 = (?> Σ∗𝑎 1 Σ∗ ) · (?> Σ∗𝑎 2 Σ∗ ) · · · (?> Σ∗𝑎𝑛 Σ∗ ).
For a string 𝑤, we have that 𝑤, [0, 0] |= 𝑟 iff the string 𝑤 contains at least one occurrence of each
one of the letters 𝑎 1, 𝑎 2, . . . , 𝑎𝑛 . This example shows that lookarounds can be used to encode certain
kinds of intersection: 𝑤, [0, 0] |= 𝑟 iff 𝑤 matches the regular expression (Σ∗𝑎 1 Σ∗ ) ∩ (Σ∗𝑎 2 Σ∗ ) ∩
· · · ∩ (Σ∗𝑎𝑛 Σ∗ ), where ∩ is the intersection operation on regular expresssions (which is, of course,
interpreted as intersection on languages).
For a regular expression 𝑟 , the notation 𝑟 𝑛 is an abbreviation for the concatenation 𝑟 · 𝑟 · · · 𝑟 (𝑛
times). The notation 𝑟 {𝑛} is also commonly used to describe the repetition of 𝑟 exactly 𝑛 times.
Example 6. Lookarounds are often used to extract substrings in a text by specifying constraints
involving the context that the substring occurs in. For instance, one can use the regular expression
[0−9]{3}−[0−9]{3}−[0−9]{4} to find a telephone number. Usually, the first three digits of the
phone number are the area code. To extract just the area code from a telephone number, one can use
the regular expression [0−9]{3}(?> −[0−9]{3}−[0−9]{3}) that contains a lookahead assertion.
Another example that shows the usefulness of lookbehind expressions is the extraction of an
email address domain. Suppose 𝛼 = [0−9𝐴−𝑍𝑎−𝑧] is a predicate that contains the alphanumeric
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 7 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:7

characters. One can use the regular expression 𝛼 ∗ @𝛼 ∗ .𝛼 ∗ to match email addresses. To extract the
domain of the email address, one can write the expression (?< 𝛼 ∗ @)𝛼 ∗ .𝛼 ∗ . Interestingly, this regex
is not allowed by the PCRE standard, which disallows lookbehinds that could extend over a location
of unbounded length. The algorithm we will present later in Section 4 does not have this limitation.
Definition 7 (Algebraic Semantics of Lookaround). If 𝑤 is a string over Σ and [𝑖, 𝑗] is a location
in 𝑤, then we call the triple (𝑤, 𝑖, 𝑗) a (string) slice over Σ. We define
Slices(Σ) = {(𝑤, 𝑖, 𝑗) | 𝑤 ∈ Σ∗ and 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 |},
which is the set of all string slices over Σ. A match-language over an alphabet Σ is a subset of
Slices(Σ). For match-languages 𝐴, 𝐵 we define the operations of concatenation ·, Kleene iteration ∗
and lookaround as follows:
𝐴 · 𝐵 = {(𝑤, 𝑖, 𝑘) | (𝑤, 𝑖, 𝑗) ∈ 𝐴 and (𝑤, 𝑗, 𝑘) ∈ 𝐵 for some 𝑗 }
1 M = {(𝑤, 𝑖, 𝑖) | 𝑤 ∈ Σ∗ and 0 ≤ 𝑖 ≤ |𝑤 |}
Ð
𝐴∗ = 𝑛≥0 𝐴𝑛 , where 𝐴0 = 1 M and 𝐴𝑛+1 = 𝐴𝑛 · 𝐴 for every 𝑛 ≥ 0

(?> 𝐴) = {(𝑤, 𝑖, 𝑖) | (𝑤, 𝑖, |𝑤 |) ∈ 𝐴}
(?≯ 𝐴) = {(𝑤, 𝑖, 𝑖) | (𝑤, 𝑖, |𝑤 |) ∉ 𝐴}
(?< 𝐴) = {(𝑤, 𝑖, 𝑖) | (𝑤, 0, 𝑖) ∈ 𝐴}
(?≮ 𝐴) = {(𝑤, 𝑖, 𝑖) | (𝑤, 0, 𝑖) ∉ 𝐴}

For a regular expression 𝑟 , we define its match-language M (𝑟 ) as follows:
(1)

M (𝑟 ) = {(𝑤, 𝑖, 𝑗) | 𝑤, [𝑖, 𝑗] |= 𝑟 }.

If (𝑤, 𝑖, 𝑗) ∈ M (𝑟 ), then we say that the slice (𝑤, 𝑖, 𝑗) matches the expression 𝑟 and also that 𝑟
recognizes the slice (𝑤, 𝑖, 𝑗). We say that the regular expressions 𝑟 and 𝑟 ′ are equivalent, and we
write 𝑟 ≡ 𝑟 ′ , if they have equal match-languages, i.e., M (𝑟 ) = M (𝑟 ′ ).
Lemma 8 (Match-Language). The following hold for the match-languages of regular expressions
with lookaround: M (𝜀) = 1 M , and
M (𝑝) = {(𝑤, 𝑖, 𝑖 + 1) | 𝑖 < |𝑤 | and 𝑝 (𝑤 (𝑖)) = 1}

M ((?> 𝑟 )) = (?> M (𝑟 ))

M (𝑟 + 𝑠) = M (𝑟 ) ∪ M (𝑠)

M ((?≯ 𝑟 )) = (?≯ M (𝑟 ))

M (𝑟 · 𝑠) = M (𝑟 ) · M (𝑠)

M ((?< 𝑟 )) = (?< M (𝑟 ))

∗

M (𝑟 ) = M (𝑟 )

∗

M ((?≮ 𝑟 )) = (?≮ M (𝑟 ))

for every predicate 𝑝 and all regular expressions 𝑟, 𝑠.
Proof. Let 𝑝 be a predicate over the alphabet Σ. Let RHS be the right-hand side of the equality for
M (𝑝) shown above. For an arbitrary slice (𝑤, 𝑖, 𝑗) the following equivalences hold: (𝑤, 𝑖, 𝑗) ∈ M (𝑝)
iff 𝑤, [𝑖, 𝑗] |= 𝑝 iff (𝑗 = 𝑖 + 1 and 𝑝 (𝑤 (𝑖)) = 1) iff (𝑗 = 𝑖 + 1 and (𝑤, 𝑖, 𝑖 + 1) ∈ RHS) iff (𝑤, 𝑖, 𝑗) ∈ RHS.
The rest of the cases are handled with similar arguments. We will just note that Claim 3 is useful
for establishing that M (𝑟 ∗ ) = M (𝑟 ) ∗ .
□
Informally, Lemma 8 says that the semantics of Fig. 1 is equivalent to an algebraic semantics in
terms of match-languages and operations on them (namely, the operations defined earlier).
Observation 9 (Relationship to PCRE). The lookaround semantics presented in Fig. 1 is different
from PCRE [The PCRE2 Developers 2023]. When the lookahead (?> 𝑟 ) has a match at location [𝑖, 𝑖]
in a string 𝑤, then this means that 𝑟 matches at location [𝑖, |𝑤 |] which extends to the end of the
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 8 -->

92:8

Konstantinos Mamouras and Agnishom Chattopadhyay

string. For PCRE-style positive lookahead (?= 𝑟 ), negative lookahead (?! 𝑟 ), positive lookbehind
(?<= 𝑟 ) and negative lookbehind (?<! 𝑟 ), the semantics can be expressed as follows:
𝑤, [𝑖, 𝑗] |= (?= 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑖, 𝑘] |= 𝑟 for some 𝑘 ∈ [ 𝑗, |𝑤 |]
𝑤, [𝑖, 𝑗] |= (?! 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑖, 𝑘] ̸ |= 𝑟 for every 𝑘 ∈ [ 𝑗, |𝑤 |]
𝑤, [𝑖, 𝑗] |= (?<= 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑘, 𝑗] |= 𝑟 for some 𝑘 ∈ [0, 𝑖]
𝑤, [𝑖, 𝑗] |= (?<! 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑘, 𝑗] ̸ |= 𝑟 for every 𝑘 ∈ [0, 𝑖]
In particular, this means that the PCRE-style lookahead (?= 𝑟 ) is equivalent to (?> 𝑟 · Σ∗ ). Similarly,
(?<= 𝑟 ) is equivalent to (?< Σ∗ · 𝑟 ).
From the definition of Fig. 1, we see that 𝑤, [𝑖, 𝑗] |= (?> 𝜀) iff 𝑖 = 𝑗 = |𝑤 |. This means that (?> 𝜀)
is equivalent to PCRE’s end-of-string anchor, written as $, which only matches at the end of the
string. Similarly, it holds that 𝑤, [𝑖, 𝑗] |= (?< 𝜀) iff 𝑖 = 𝑗 = 0. So, (?< 𝜀) is equivalent to PCRE’s
start-of-string anchor, written as ^, which only matches at the beginning of the string.
The previous observations mean that our lookaround can express PCRE’s lookaround and the
other way round. Here is a summary of all equivalences:
(?= 𝑟 ) ≡ (?> 𝑟 Σ∗ )

(?<= 𝑟 ) ≡ (?< Σ∗𝑟 )

(?> 𝑟 ) ≡ (?= 𝑟 $)

(?< 𝑟 ) ≡ (?<= ^𝑟 )

(?! 𝑟 ) ≡ (?≯ 𝑟 Σ∗ )

(?<! 𝑟 ) ≡ (?≮ Σ∗𝑟 )

(?≯ 𝑟 ) ≡ (?! 𝑟 $)

(?≮ 𝑟 ) ≡ (?<! ^𝑟 )

This discussion also establishes that our syntax is more economical than the syntax of PCRE,
because the anchors ^ and $ can be expressed using our lookaround constructs.
2.1

Equational Properties of Lookaround

Informally, the following lemma says that regular expressions with lookaround satisfy the properties
of Kleene algebra [Kozen 1994] for the equivalence relation ≡. We write 𝑟 1 ⊑ 𝑟 2 as abbreviation for
𝑟 1 + 𝑟 2 ≡ 𝑟 2 . It follows that 𝑟 1 ⊑ 𝑟 2 iff M (𝑟 1 ) ⊆ M (𝑟 2 ) iff (𝑤, [𝑖, 𝑗] |= 𝑟 1 implies 𝑤, [𝑖, 𝑗] |= 𝑟 2 for all
strings 𝑤 and locations [𝑖, 𝑗] in 𝑤).
Lemma 10 (Equivalences for Regular Expressions). The following properties hold for all
regular expressions 𝑟, 𝑟 1, 𝑟 2, 𝑟 3 ∈ LReg(Σ):
(1) (𝑟 1 · 𝑟 2 ) · 𝑟 3 ≡ 𝑟 1 · (𝑟 2 · 𝑟 3 ) and 𝜀 · 𝑟 ≡ 𝑟 · 𝜀 ≡ 𝑟
(2) (𝑟 1 + 𝑟 2 ) + 𝑟 3 ≡ 𝑟 1 + (𝑟 2 + 𝑟 3 ), 𝑟 1 + 𝑟 2 ≡ 𝑟 2 + 𝑟 1 , and 𝑟 + ∅ ≡ 𝑟
(3) 𝑟 1 · (𝑟 2 + 𝑟 3 ) ≡ 𝑟 1 · 𝑟 2 + 𝑟 1 · 𝑟 3 , and (𝑟 1 + 𝑟 2 ) · 𝑟 3 ≡ 𝑟 1 · 𝑟 3 + 𝑟 2 · 𝑟 3
(4) ∅ · 𝑟 ≡ 𝑟 · ∅ ≡ ∅
(5) 𝜀 + 𝑟𝑟 ∗ ⊑ 𝑟 ∗
(6) 𝜀 + 𝑟 ∗𝑟 ⊑ 𝑟 ∗
(7) 𝑟 1 · 𝑟 2 ⊑ 𝑟 2 =⇒ 𝑟 1∗ · 𝑟 2 ⊑ 𝑟 2
(8) 𝑟 2 · 𝑟 1 ⊑ 𝑟 2 =⇒ 𝑟 2 · 𝑟 1∗ ⊑ 𝑟 2
Proof. As a representative case, we consider property (5) and we leave the rest of the cases
to the reader. It suffices to show that 𝜀 ⊑ 𝑟 ∗ and 𝑟𝑟 ∗ ⊑ 𝑟 ∗ . We omit the proof of 𝜀 ⊑ 𝑟 ∗ . Let 𝑤
be an arbitrary string and [𝑖, 𝑗] be a location in 𝑤. Suppose that 𝑤, [𝑖, 𝑗] |= 𝑟𝑟 ∗ . It follows that
there exists 𝑘 such that 𝑤, [𝑖, 𝑘] |= 𝑟 and 𝑤, [𝑘, 𝑗] |= 𝑟 ∗ . So, there is a decomposition 𝑆 of [𝑘, 𝑗]
such that 𝑤, [𝑖 ′, 𝑗 ′ ] |= 𝑟 for every location [𝑖 ′, 𝑗 ′ ] in 𝑆. Define 𝑆 ′ = [𝑖, 𝑘] · 𝑆 and notice that 𝑆 ′ is a
decomposition of [𝑖, 𝑗] witnessing that 𝑤, [𝑖, 𝑗] |= 𝑟 ∗ . We have thus established that 𝑟𝑟 ∗ ⊑ 𝑟 ∗ . □
Alternatively, Lemma 10 can be established by directly considering the algebraic semantics of
regular expressions from Definition 7 and using Lemma 8. Indeed, it suffices to show that the
match-languages of Definition 7 together with the constants ∅, 1 M and the operations ∪, · and ∗
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 9 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:9

form a Kleene algebra. This means that ∅, 1 M , ∪, · satisfy the axioms of idempotent semirings, and
∗ additionally satisfies the four axioms of Kleene iteration from [Kozen 1994]. As an example, let us
consider the equivalence 𝜀 +𝑟 ·𝑟 ∗ ≡ 𝑟 ∗ . It can be immediately established by using the interpretation
M of regular expressions in the Kleene algebra of match-languages:
𝜀 + 𝑟 · 𝑟 ∗ ≡ 𝑟 ∗ ⇐⇒ M (𝜀 + 𝑟 · 𝑟 ∗ ) = M (𝑟 ∗ )
∗

[def. of ≡]
∗

⇐⇒ 1 M + M (𝑟 ) · M (𝑟 ) = M (𝑟 ) ,

[Lemma 8]

which holds because match-languages form a Kleene algebra.
Lemma 11 (Algebraic Properties of Lookaround). The following properties for lookahead
assertions hold (and completely symmetric properties hold for lookbehind assertions):
(1) Concatenation of lookarounds is commutative: (?> 𝑟 ) · (?> 𝑠) ≡ (?> 𝑠) · (?> 𝑟 ).
(2) Idempotence: (?> 𝑟 ) · (?> 𝑟 ) ≡ (?> 𝑟 ).
(3) Kleene iteration over lookarounds is equivalent to 𝜀: (?> 𝑟 ) ∗ ≡ 𝜀.
(4) Union distributes over lookarounds: (?> 𝑟 + 𝑠) ≡ (?> 𝑟 ) + (?> 𝑠).
(5) Lookaheads can be flattened: (?> (?> 𝑟 ) · 𝑠) ≡ (?> 𝑟 ) · (?> 𝑠).
(6) Lookaheads can be flattened in the presence of wildcards: (?> 𝑟 · (?> 𝑠) · Σ∗ ) ≡ (?> 𝑟 · 𝑠).
(7) The union of positive and negative lookaheads can be simplified: (?> 𝑟 ) + (?≯ 𝑟 ) ≡ 𝜀.
(8) Positive and negative lookaheads cannot be matched together: (?> 𝑟 ) · (?≯ 𝑟 ) ≡ ∅
(9) For predicates 𝑝 1 and 𝑝 2 : (?> 𝑝 1𝑟 1 )𝑝 2𝑟 2 ≡ (𝑝 1 ∩ 𝑝 2 )(?> 𝑟 1 )𝑟 2 .
Proof. These properties can be proved in a straightforward manner using the formal semantics
of lookaround expressions. To demonstrate, we prove the first one.
Suppose 𝑤, [𝑖, 𝑗] |= (?> 𝑟 ) · (?> 𝑠). Then, there exists 𝑖 ≤ 𝑘 ≤ 𝑗 such that 𝑤, [𝑖, 𝑘] |= (?> 𝑟 ) and
𝑤, [𝑘, 𝑗] |= (?> 𝑠). By the semantics of lookahead, it must be that 𝑖 = 𝑘 and 𝑘 = 𝑗. Thus, we have
𝑤, [𝑖, 𝑖] |= (?> 𝑟 ) and 𝑤, [𝑖, 𝑖] |= (?> 𝑠). From the semantics of · we obtain that 𝑤, [𝑖, 𝑖] |= (?> 𝑠) ·
(?> 𝑟 ). Since 𝑖 = 𝑗, this is the same as 𝑤, [𝑖, 𝑗] |= (?> 𝑠) · (?> 𝑟 ). This shows that M ((?> 𝑟 ) · (?> 𝑠)) ⊆
M ((?> 𝑠) · (?> 𝑟 )). By interchanging the roles of 𝑟 and 𝑠, we can prove the other direction.
□
The intuition for property (5) regarding the flattening of lookarounds is that both expressions
describe the requirement that both 𝑟 and 𝑠 have a match at location [𝑖, |𝑤 |] (if we interpret them at
position 𝑖).
We make some further observations about the simplification of lookaround assertions. We have
previously stated that the PCRE expression (?= 𝑟 ) is equivalent to (?> 𝑟 · Σ∗ ) in our syntax. On the
other hand, we can use equation (6) above to establish the equivalences
[Lemma 10]

(?> 𝑟 ) ≡ (?> 𝑟 · 𝜀)
∗

≡ (?> 𝑟 (?> 𝜀)Σ ).

[Lemma 11]

The last expression is the same as the regex (?= 𝑟 $) in PCRE notation. Thus, our syntax can be
translated to PCRE notation by adding the $ anchor. The main observation is that this fact, which
we already knew from Observation 9, is established here purely by equational reasoning, using the
properties from Lemma 10 and Lemma 11.
Note that (?> 𝑟 · (?> 𝑠)) cannot be simplified to (?> 𝑟 · 𝑠). For example, (?> 𝑎𝑏 · (?> 𝑐𝑑)) cannot
be true at any position 𝑖 because 𝑎𝑏 has to extend to the end of the string where (?> 𝑐𝑑) cannot
hold. So, this regex cannot be equivalent to (?> 𝑎𝑏𝑐𝑑).
Another caveat is the following: Suppose 𝑢 ∈ ⟦𝑟 ⟧ and 𝑣 ∈ ⟦𝑠⟧. We cannot, in general, expect
that 𝑢 · 𝑣 ∈ ⟦𝑟 · 𝑠⟧. This is because lookaround expressions are able to “view” the entire string. As a
concrete example, consider 𝑟 = (?> (𝑎𝑎) ∗ ), 𝑠 = 𝑎 ∗ , 𝑢 = 𝜀, 𝑣 = 𝑎.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 10 -->

92:10

Konstantinos Mamouras and Agnishom Chattopadhyay

It may be possible to give a complete algebraic axiomatization of simple cases of lookaround
(e.g., anchors and word boundaries) using the approach of Kleene algebra with extra equations
[Grathwohl et al. 2014b; Kozen 1997; Kozen and Mamouras 2014; Mamouras 2015, 2017]. The axiomatization of general lookaround is more challenging, as it can encode some kinds of intersection.
3 ORACLES FOR LOOKAROUND ASSERTIONS
This section is a stepping stone towards the full algorithm for matching regular expressions with
lookaround, which we will present in Section 4. Here, we will see how to match a regular expression
over a string, assuming that the truth values of lookaround assertions at all positions can be obtained
by querying oracles. To formalize this computation, we introduce a notion of regular expression
that includes oracle queries instead of lookaround assertions. We call these “oracle-regexes” or
“o-regexes” for brevity. Oracle-regexes are matched using a model of automata that is an extension
of classical NFAs. We call these automata “oracle-NFAs” or ONFAs. ONFAs contain transitions that
query the oracles. An oracle-transition is taken if and only if the oracle responds with “true”, but
it does not consume a character from the input string. So, we think of them as oracle-guarded
𝜀-transitions. Matching for o-regexes and ONFAs is not defined w.r.t. plain strings over the input
alphabet, because these strings lack information about the responses of the oracles. Instead, we
¯ where
define a semantics w.r.t. to “oracle-strings” or “o-strings”, which are pairs of the form ⟨𝑤, 𝛽⟩,
¯
𝑤 is a string over the alphabet and 𝛽 is a sequence of all oracle responses for all positions 0, 1, . . . , |𝑤 |.
Similar to classical NFAs, the simulation of an ONFA is done in a single left-to-right pass over
the input. At each step, a single character and an oracle valuation (for the position right after the
character) are consumed. Each step needs 𝑂 (𝑚) time, where 𝑚 is the size of the ONFA.
3.1

Oracle Strings and Oracle Regular Expressions

Suppose 𝑉 is a finite set of oracle names. In the later algorithmic development, we will be using
natural numbers as oracles names, because they are convenient for indexing in arrays. A 𝑉 -valuation
is a function of type 𝑉 → B, that is, a truth assignment for the oracle names 𝑉 . We also use the
notation B𝑉 for the set of 𝑉 -valuations.
Definition 12. An oracle-string or o-string is a pair ⟨𝑤, ¯
𝛽⟩ where 𝑤 ∈ Σ∗ and 𝛽¯ ∈ (B𝑉 ) ∗ such that
| 𝛽¯| = |𝑤 | + 1. The set of o-strings over the alphabet Σ and oracle names 𝑉 is denoted O (Σ, 𝑉 ).
Informally, an oracle-string is a string together with the responses for the oracle queries at
every position. Suppose 𝑤 = 𝑎 0𝑎 1 . . . 𝑎𝑛−1 and 𝛽¯ = 𝛽 0 𝛽 1 𝛽 2 . . . 𝛽𝑛 . The character 𝑎𝑖 has the oracle
valuations 𝛽𝑖 −1 and 𝛽𝑖 right before and right after it respectively.
When we slice an o-string, we must take into account the oracle valuations at the boundaries.
Suppose we have an oracle-string ⟨𝑤, ¯
𝛽⟩ ∈ O (Σ, 𝑉 ) with 𝑤 = 𝑎 0𝑎 1 . . . 𝑎𝑛−1 and 𝛽¯ = 𝛽 0 𝛽 1 . . . 𝛽𝑛 . For
0 ≤ 𝑖 ≤ 𝑗 ≤ 𝑛, we define the slice of ⟨𝑤, ¯
𝛽⟩ at location [𝑖, 𝑗], denoted by ⟨𝑤, ¯
𝛽⟩[𝑖, 𝑗], as the o-string
⟨𝑤 ′, 𝛽¯′ ⟩ where 𝛽¯′ = 𝛽𝑖 𝛽𝑖+1 . . . 𝛽 𝑗 −1 𝛽 𝑗 and 𝑤 ′ = 𝑎𝑖 𝑎𝑖+1 . . . 𝑎 𝑗 −1 . In particular, when 𝑖 = 𝑗, 𝑤 ′ is the
empty string 𝜀 and 𝛽¯′ is the singleton sequence 𝛽𝑖 .
The concatenation operation on O (Σ, 𝑉 ) needs to be defined carefully so that it matches the
way we intend to use o-strings. The concatenation of two elements of O (Σ, 𝑉 ) is only defined if
the oracle valuations agree. Formally, suppose ⟨𝑤 1, 𝛽¯1 · 𝛽⟩, ⟨𝑤 2, 𝛾 · 𝛽¯2 ⟩ ∈ O (Σ, 𝑉 ) with 𝛽, 𝛾 ∈ B𝑉 ,
then ⟨𝑤 1, 𝛽¯1 · 𝛽⟩ · ⟨𝑤 2, 𝛾 · 𝛽¯2 ⟩ is defined iff 𝛽 = 𝛾. The concatenation ⟨𝑤 1, 𝛽¯1 · 𝛽⟩ · ⟨𝑤 2, 𝛽 · 𝛽¯2 ⟩ is
defined to be ⟨𝑤 1𝑤 2, 𝛽¯1 · 𝛽 · 𝛽¯2 ⟩. We extend this definition in a natural way to concatenation of sets
of o-strings. Kleene iteration of an o-string (or a set of o-strings) is also defined in an analogous
manner, respecting the agreement of oracle valuations at concatenation boundaries.

Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 11 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:11

Definition 13 (Oracle Regular Expressions). The set OReg(Σ, 𝑉 ) of oracle regular expressions
(or o-regexes) over alphabet Σ and oracle names 𝑉 is defined with the following grammar:
𝑟, 𝑟 1, 𝑟 2 ::= 𝜀 | 𝑝 | Q𝜎𝜀 (𝑣) | 𝑟 1 + 𝑟 2 | 𝑟 1 · 𝑟 2 | 𝑟 ∗,
where 𝑝 ∈ P is a predicate over Σ (character class), 𝜎 ∈ {+, -} is the sign of an oracle query, and
𝑣 ∈ 𝑉 is an oracle name.
Instead of lookaround assertions as in LReg(Σ), oracle regular expressions have queries of the
form Q+𝜀 (𝑣) (positive queries) and Q-𝜀 (𝑣) (negative queries).
Example 14 (O-Regexes). The o-regex 𝑎𝑏𝑐𝑑 · Q+𝜀 (0) describes a pattern that includes the signature
𝑎𝑏𝑐𝑑 and additionally has to satisfy a positive lookaround assertion right after it. The o-regex does
not specify the assertion itself, it only contains a reference to an assertion that should be provided
separately. The o-regex Q-𝜀 (0) · ℎ𝑒𝑙𝑙𝑜 · Q-𝜀 (1) describes the pattern ℎ𝑒𝑙𝑙𝑜, which additionally has to
satisfy two negative lookaround assertions right before ℎ and right after 𝑜.
Let 𝛽 be an oracle valuation and Q𝜎𝜀 (𝑣) be an oracle query. We say that 𝛽 satisfies Q𝜎𝜀 (𝑣), and we
write 𝛽 |= Q𝜎𝜀 (𝑣), if (𝜎 = + and 𝛽 [𝑣] = 1) or (𝜎 = - and 𝛽 [𝑣] = 0).
Every expression 𝑟 ∈ OReg(Σ, 𝑉 ) denotes a language of oracle strings, i.e., a subset of O (Σ, 𝑉 ),
written as ⟦𝑟 ⟧. This is defined inductively as follows:
⟦𝜀⟧ = {⟨𝜀, 𝛽⟩ | 𝛽 ∈ B𝑉 }

⟦𝑟 1 + 𝑟 2 ⟧ = ⟦𝑟 1 ⟧ ∪ ⟦𝑟 2 ⟧
𝑉

⟦𝑝⟧ = {⟨𝑎, 𝛽𝛾⟩ | 𝑎 ∈ Σ with 𝑝 (𝑎) = 1 and 𝛽, 𝛾 ∈ B }
⟦Q𝜎𝜀 (𝑣)⟧ = {⟨𝜀, 𝛽⟩ | 𝛽 ∈ B𝑉 and 𝛽 |= Q𝜎𝜀 (𝑣)} for 𝜎 ∈ {+, -}

⟦𝑟 1 · 𝑟 2 ⟧ = ⟦𝑟 1 ⟧ · ⟦𝑟 2 ⟧
⟦𝑟 ∗ ⟧ = ⟦𝑟 ⟧∗

for all 𝑝 ∈ P, 𝑣 ∈ 𝑉 , and 𝑟, 𝑟 1, 𝑟 2 ∈ OReg(Σ, 𝑉 ). We also define the satisfaction relation as follows:
¯ [𝑖, 𝑗] |= 𝑟 ⇐⇒ ⟨𝑤, ¯
𝑤, 𝛽,
𝛽⟩[𝑖, 𝑗] ∈ ⟦𝑟 ⟧.
In the definition above, we are using the o-string slicing operation introduced earlier.
3.2

Choosing Appropriate Oracle Valuations

Later in this section, we will prove Lemma 21, which formalizes the connection between OReg and
LReg. This is crucial for the efficient matching algorithm described in the following section.
Suppose 𝑟 ∈ LReg(Σ) and 𝑤 ∈ Σ∗ . We say that 𝑠 is a lookaround assertion of 𝑟 if (1) 𝑠 is a
subexpression of 𝑟 , (2) 𝑠 is a lookaround assertion. We say that 𝑠 is a maximal lookaround assertion
of 𝑟 if (1) it is a lookaround assertion of 𝑟 , and (2) it does not occur underneath a lookaround
operator in 𝑟 .
Example 15. Consider the regex 𝑟 = ((?! )(?!(?<![0-9])0)[0-9 ])+ , written using PCRE
notation. The subexpressions (?! ) and (?!(?<![0-9])0) are the two maximal lookaround
assertions of 𝑟 . Notice that (?<![0-9]) is a lookaround assertion of 𝑟 , but it is not maximal.
For a type 𝐴, we write Vect(𝐴) for the type of finite vectors (i.e., sequences) over 𝐴. If 𝑥 : Vect(𝐴),
we write 𝑥 .len() for its length and 𝑥 [𝑖] for the 𝑖-th element of the vector, where 𝑖 = 0, 1, . . . , 𝑥 .len() −
1. The empty vector is written as [ ]. If 𝑥, 𝑦 : Vect(𝐴), then 𝑥 · 𝑦 : Vect(𝐴) is their concatenation.
The finite type {L2R, R2L} has two inhabitants that are used to indicate the direction of ONFA
computation over an o-string. The element L2R (resp., R2L) indicates a left-to-right (resp., right-toleft) pass, which is used for computing lookbehind (resp., lookahead) assertions.
The definition of the “shallow decomposition” of a regex (LReg) that follows (Definition 16) is
meant to separate the “main” part of the regex from the maximal lookaround assertions that it
contains. The main part is expressed as an oracle-regex (OReg) that contains references to the
maximal lookaround assertions.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 12 -->

92:12

Konstantinos Mamouras and Agnishom Chattopadhyay

Definition 16 (Shallow Decomposition). For every index 𝑖 ∈ N, we define the shallow decomposition function shallow𝑖 : LReg(Σ) → OReg(Σ) × Vect({L2R, R2L}) × Vect(LReg(Σ)) as
follows:
shallow𝑖 (𝑟 0 ) = (𝑟 0, [ ], [ ]) for 𝑟 0 = 𝜀, 𝑝
shallow𝑖 (𝑟 1 ◦ 𝑟 2 ) = (𝑠 1 ◦ 𝑠 2, 𝐷 1 · 𝐷 2, 𝑅1 · 𝑅2 ), where
(𝑠 1, 𝐷 1, 𝑅1 ) = shallow𝑖 (𝑟 1 ) and

shallow𝑖 ((?> 𝑟 )) = (Q+𝜀 (𝑖), [R2L], [𝑟 ])

(𝑠 2, 𝐷 2, 𝑅2 ) = shallow𝑖+|𝑅1 | (𝑟 𝑗 )

shallow𝑖 ((?≯ 𝑟 )) = (Q-𝜀 (𝑖), [R2L], [𝑟 ])

shallow𝑖 (𝑟 ∗ ) = (𝑠 ∗, 𝐷, 𝑅), where

shallow𝑖 ((?< 𝑟 )) = (Q+𝜀 (𝑖), [L2R], [𝑟 ])

(𝑠, 𝐷, 𝑅) = shallow𝑖 (𝑟 )

shallow𝑖 ((?≮ 𝑟 )) = (Q-𝜀 (𝑖), [L2R], [𝑟 ])

where 𝑝 is a character class and ◦ ∈ {·, +}. The parameter 𝑖 in shallow𝑖 is used to give unique names
to the oracles used in queries. We also define shallow(𝑟 ) = shallow0 (𝑟 ).
Example 17. Consider the regex 𝑟 = ((?! )(?!(?<![0-9])0)[0-9 ])+ (in PCRE notation). Using
the notation of Section 2, the regex is
𝑟 = ((?≯ ␣ ␣ Σ∗ )(?≯ (?≮ Σ∗ [0−9])0Σ∗ ) [0−9 ␣ ]) + .
Then, shallow(𝑟 ) = (𝑠, [R2L, R2L], [𝑟 0, 𝑟 1 ]), where
𝑟 0 = ␣ ␣ Σ∗

𝑟 1 = (?≮ Σ∗ [0−9])0Σ∗

𝑠 = (Q-𝜀 (0) · Q-𝜀 (1) · [0−9 ␣ ]) + .

Notice that the shallow decomposition of 𝑟 extracts signs, directions, and expressions from the
maximal lookaround assertions of 𝑟 .
The oracle-arity oarity(𝑟 ) of a regular expression 𝑟 , defined below in Definition 18, is the number
of subterms that are lookarounds that are not subterms of lookarounds (i.e., the number of maximal
lookarounds).
Definition 18 (Oracle-Arity, and Oracle-Projection, Oracle-Matrix). The oracle-arity or o-arity
of an expression 𝑟 ∈ LReg(Σ) is defined inductively as follows:
oarity(𝜀) = 0
oarity(𝑝) = 0

oarity(𝑟 1 ◦ 𝑟 2 ) = oarity(𝑟 1 ) + oarity(𝑟 2 ) for ◦ ∈ {+, ·}
oarity(𝑟 ∗ ) = oarity(𝑟 )

and oarity(𝑟 ) = 1 when 𝑟 is a lookaround assertion.
Let 𝑟 ∈ LReg(Σ) and 𝑖 ∈ N. Define oproj𝑖 (𝑟 ) = 𝑠, where (𝑠, 𝐷, 𝑅) = shallow𝑖 (𝑟 ). The oracleprojection or o-projection of 𝑟 ∈ LReg(Σ) is oproj(𝑟 ) = oproj0 (𝑟 ).
Let 𝑟 ∈ LReg(Σ) be a regular expression and (𝑠, 𝐷, 𝑅) = shallow(𝑟 ) be its shallow decomposition.
Let 𝑤 ∈ Σ∗ . We define the oracle matrix for 𝑟 and 𝑤, denoted by Mat(𝑟, 𝑤) : Vect(Vect(B)), to be
a vector of 𝑘 = oarity(𝑟 ) Boolean tapes of length |𝑤 | + 1 each. At the (𝑣, 𝑖) entry of the matrix
Mat(𝑟, 𝑤), we note whether the 𝑣-th expression matches at position 𝑖 in the string 𝑤. More formally,
(
𝜒 (𝑤, [0, 𝑖], 𝑅 [𝑣]),
if 𝐷 [𝑣] = L2R
Mat(𝑟, 𝑤) [𝑣] [𝑖] =
𝜒 (𝑤, [𝑖, |𝑤 |], 𝑅 [𝑣]), if 𝐷 [𝑣] = R2L.
for 𝑣 = 0, . . . , 𝑘 − 1 and 𝑖 = 0, . . . , |𝑤 |. Each vector Mat(𝑟, 𝑤) [𝑣] (size |𝑤 | + 1) is called an oracle tape.
Let 𝑟 ∈ LReg(Σ). Intuitively, oarity(𝑟 ) is the number of occurrences of maximal lookaround
assertions of 𝑟 . For every 𝑖 ∈ N, it holds that oarity(𝑟 ) = |𝐷 | = |𝑅|, where (𝑠, 𝐷, 𝑅) = shallow𝑖 (𝑟 ).
Let 𝑟 ∈ LReg(Σ) and 𝑤 ∈ Σ∗ . The oracle matrix Mat(𝑟, 𝑤) : Vect(Vect(B)) is a 𝑘 × (𝑛 + 1) matrix,
where 𝑘 = oarity(𝑟 ) and 𝑛 = |𝑤 |. The following properties hold:
(i) Mat(𝑟 1 ◦ 𝑟 2, 𝑤) = Mat(𝑟 1, 𝑤) · Mat(𝑟 2, 𝑤) for ◦ ∈ {+, ·}
(ii) Mat(𝑟 ∗, 𝑤) = Mat(𝑟, 𝑤)
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 13 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:13

(iii) Mat((?> 𝑟 ), 𝑤) = [𝑡], where 𝑡 [𝑖] = 𝜒 (𝑤, [𝑖, |𝑤 |], 𝑟 ) for 𝑖 = 0, 1, . . . , |𝑤 |.
(iv) Mat((?< 𝑟 ), 𝑤) = [𝑡], where 𝑡 [𝑖] = 𝜒 (𝑤, [0, 𝑖], 𝑟 ) for 𝑖 = 0, 1, . . . , |𝑤 |.
The above properties, together with Mat(𝜀, 𝑤) = [ ] and Mat(𝑝, 𝑤) = [ ], can be used as an
alternative definition for Mat.
Example 19. Consider the regex 𝑟 = (?< Σ∗𝑎Σ∗ )𝑏𝑐 (?> Σ∗𝑑Σ∗ ) from Example 4, which can be
written as (?<=a.*)bc(?=.*d) in PCRE notation. The lookaround assertions of 𝑟 are 𝑟 1 = (?< Σ∗𝑎Σ∗ )
and 𝑟 2 = (?> Σ∗𝑑Σ∗ ). Both of them are maximal. The shallow decomposition of 𝑟 is
shallow(𝑟 ) = (Q+𝜀 (0) · 𝑏𝑐 · Q+𝜀 (1), [L2R, R2L], [Σ∗𝑎Σ∗, Σ∗𝑑Σ∗ ].)
Let us use the word 𝑤 = 𝑏𝑏𝑏𝑐𝑎𝑏𝑏𝑐𝑏𝑏𝑑𝑏𝑏𝑏𝑐 considered in a previous example (Example 4) to illustrate
the oracle matrix 𝑇 = Mat(𝑟, 𝑤). As shown explicitly in the table below,
𝑇 [0] [𝑖] = 1 ⇐⇒ 𝑤, [0, 𝑖] |= Σ∗𝑎Σ∗ and
𝑇 [1] [𝑖] = 1 ⇐⇒ 𝑤, [𝑖, |𝑤 |] |= Σ∗𝑑Σ∗ .
In this case, the first expression is testing for a presence of an 𝑎 in the prefix, and the second
expression is looking for the presence of a 𝑑 in the suffix.
string 𝑤

b b b c a b b c b b d b b b c

oracle tape 𝑇 [0]

0 0 0 0 0 1 1 1 1 1 1 1 1 1 1 1

oracle tape 𝑇 [1]

1 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0

Note 20 (Oracle Matrix & Valuation Sequences). Let 𝑉𝑘 = {0, 1, . . . , 𝑘 − 1}. We use 𝑉𝑘 as a set
of oracle names that can be used to index in arrays and vectors. Recall that the type of an oracle
matrix 𝑇 is Vect(Vect(B)). More specifically, it is a 𝑘 × (𝑛 + 1) matrix, where 𝑘 is the oracle arity
and 𝑛 is the string length. Informally, the matrix is laid out in rows. If we transpose it, then it would
be laid out in columns. In this case, we can view 𝑇 as a sequence 𝛽¯ = 𝛽 0 𝛽 1 . . . 𝛽𝑛 of 𝑛, where each
𝛽𝑖 : Vect(B) has length 𝑘. So, each 𝛽𝑖 is essentially a 𝑉𝑘 -valuation, i.e., it has type B𝑉𝑘 .
Lemma 21 (Shallow Decomposition). Let 𝑟 ∈ LReg(Σ) and 𝑤 ∈ Σ∗ . For all 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 |,
𝑤, [𝑖, 𝑗] |= 𝑟 ⇐⇒ 𝑤, Mat(𝑟, 𝑤), [𝑖, 𝑗] |= oproj(𝑟 ).
Equivalently, 𝜒 (𝑤, [𝑖, 𝑗], 𝑟 ) = 𝜒 (𝑤, Mat(𝑟, 𝑤), [𝑖, 𝑗], oproj(𝑟 )).
Proof. The proof is by induction on the regular expression. Let us consider the case of a positive
lookahead assertion (?> 𝑟 ). We have that oproj((?> 𝑟 )) = Q+𝜀 (0) and Mat((?> 𝑟 ), 𝑤) = [𝑡], where
𝑡 [𝑖] = 𝜒 (𝑤, [𝑖, |𝑤 |], 𝑟 ) for every 𝑖 = 0, 1, . . . , |𝑤 |. Now, we have that
𝑤, [𝑖, 𝑗] |= (?> 𝑟 ) ⇐⇒ 𝑖 = 𝑗 and 𝑤, [𝑖, |𝑤 |] |= 𝑟
⇐⇒ 𝑖 = 𝑗 and 𝜒 (𝑤, [𝑖, |𝑤 |], 𝑟 ) = 1
⇐⇒ 𝑖 = 𝑗 and 𝑡 [𝑖] = 1
⇐⇒ 𝑖 = 𝑗 and Mat((?> 𝑟 ), 𝑤) [0] [𝑖] = 1
⇐⇒ 𝑤, Mat((?> 𝑟 ), 𝑤), [𝑖, 𝑗] |= Q+𝜀 (0)
because ⟦Q+𝜀 (0)⟧ = {⟨𝜀, 𝛽⟩} with 𝛽 (0) = 1. We leave the rest of the cases to the reader.

□

Lemma 21 says that the problem of matching a regular expression 𝑟 ∈ LReg(Σ) over a string
𝑤 can be reduced to matching its oracle-projection oproj(𝑟 ) ∈ OReg(Σ), assuming we have also
computed the oracle matrix Mat(𝑟, 𝑤). This assumption means that the truth values of all oracle
queries are available.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 14 -->

92:14

3.3

Konstantinos Mamouras and Agnishom Chattopadhyay

NFAs with Oracles Queries

Now we define a class of acceptors for subsets of O (Σ, 𝑉 ). These behave like standard nondeterministic finite automata on the part of the o-string that only involves letters from Σ, but
additionally has 𝜀-transitions which are guarded by oracle queries. We will see that these acceptors
can recognize the o-string languages that are expressed by oracle-regexes.
Definition 22 (Oracle-NFAs). Let Σ be an alphabet and P a set of predicates over Σ. Let 𝑉 be
a set of oracle names. An oracle-NFA (or ONFA) A over the alphabet Σ and oracle names 𝑉 is
a tuple (𝑄, Δ, 𝐼, 𝐹 ), where 𝑄 is a finite set of states, 𝐼 ⊆ 𝑄 is a set of initial states, 𝐹 ⊆ 𝑄 is a
set of final states, and Δ ⊆ 𝑄 × (P ∪ Q+𝜀 (𝑉 ) ∪ Q-𝜀 (𝑉 ) ∪ {𝜀}) × 𝑄 is a transition relation, where
Q+𝜀 (𝑉 ) = {Q+𝜀 (𝑣) | 𝑣 ∈ 𝑉 } and Q-𝜀 (𝑉 ) = {Q-𝜀 (𝑣) | 𝑣 ∈ 𝑉 }. We write ONFA(Σ, 𝑉 ) for the set of all
oracle-NFAs over Σ and 𝑉 .
A transition (𝑞, 𝜀, 𝑞 ′ ) ∈ Δ is an unguarded 𝜀-transition. A transition (𝑞, Q+𝜀 (𝑣), 𝑞 ′ ) is an 𝜀-transition
guarded by a positive oracle query. Similarly, a transition (𝑞, Q-𝜀 (𝑣), 𝑞 ′ ) is an 𝜀-transition guarded
by a negative oracle query. A path 𝜋 in an oracle-NFA A is a sequence
𝑥0

𝑥1

𝑥2

𝑥𝑛−2

𝑥𝑛−1

𝑞 0 −→ 𝑞 1 −→ 𝑞 2 −→ · · · −−−→ 𝑞𝑛−1 −−−→ 𝑞𝑛
of transitions such that (𝑞𝑖 , 𝑥𝑖 , 𝑞𝑖+1 ) ∈ Δ for every 𝑖 = 0, 1, . . . , 𝑛 − 1. Every transition (𝑞, 𝑥, 𝑞 ′ )
denotes a set of o-strings, denoted ⟦(𝑞, 𝑥, 𝑞 ′ )⟧, which we define as follows:
⟦(𝑞, 𝜀, 𝑞 ′ )⟧ = {⟨𝜀, 𝛽⟩ | 𝛽 ∈ B𝑉 }

⟦(𝑞, Q+𝜀 (𝑣), 𝑞 ′ )⟧ = {⟨𝜀, 𝛽⟩ | 𝛽 (𝑣) = 1}

⟦(𝑞, 𝑝, 𝑞 ′ )⟧ = {⟨𝑎, 𝛽𝛾⟩ | 𝛽, 𝛾 ∈ B𝑉 , 𝑎 ∈ Σ and 𝑝 (𝑎) = 1}

⟦(𝑞, Q-𝜀 (𝑣), 𝑞 ′ )⟧ = {⟨𝜀, 𝛽⟩ | 𝛽 (𝑣) = 0}

where 𝑞 and 𝑞 ′ are states, 𝑝 ∈ P is a character class (predicate), and 𝑣 ∈ 𝑉 is an oracle name. Similarly,
a path 𝜋 = (𝑞 0, 𝑥 0, 𝑞 1 )(𝑞 1, 𝑥 1, 𝑞 2 ) . . . (𝑞𝑛−1, 𝑥𝑛−1, 𝑞𝑛 ) in A denotes the set ⟦𝜋⟧ = ⟦(𝑞 0, 𝑥 0, 𝑞 1 )⟧ ·
⟦(𝑞 1, 𝑥 1, 𝑞 2 )⟧ · · · ⟦(𝑞𝑛−1, 𝑥𝑛−1, 𝑞𝑛 )⟧ of o-strings.
A path in A is accepting if its first Ð
state is initial and its last state is final. The set of o-strings
accepted by A is defined as ⟦A⟧ = {⟦𝜋⟧ | 𝜋 is an accepting path in A}. That is, ⟦A⟧ is the
union of the denotations of all accepting paths in A.
Lemma 23. Let 𝑟 ∈ OReg(Σ, 𝑉 ). Then, there exists some A ∈ ONFA(Σ, 𝑉 ) such that ⟦𝑟 ⟧ = ⟦A⟧.
Proof. A variant of Thompson’s construction [Thompson 1968] can be used to construct the desired ONFA. Predicates (character classes) in the regular expression would correspond to transitions
in the ONFA that are labeled predicates, and oracle queries in the regular expression would correspond to oracle-guarded 𝜀-transitions. Combinators like nondeterministic choice, concatenation
and Kleene star can be handled in the usual manner.
□
Let 𝑞, 𝑞 ′ be states of an ONFA A and 𝛽 be an oracle valuation. We say that 𝑞 ′ is 𝛽-reachable from
𝑞 if there exists a path 𝑞 0 →𝑥 0 𝑞 1 →𝑥 1 · · · →𝑥𝑛−1 𝑞𝑛 in A such that (1) 𝑞 = 𝑞 0 and 𝑞 ′ = 𝑞𝑛 , (2) every
𝑥𝑖 is either 𝜀 or an oracle query, and (3) 𝛽 |= 𝑥𝑖 for every 𝑥𝑖 that is an oracle query.
Fig. 2 shows an algorithm for matching an oracle regular expression 𝑟 by compiling it into an
ONFA A and then simulating the execution of the ONFA. We consider both left-to-right and rightto-left matching, as this will be needed later in Section 4 for evaluating lookaround assertions. One
important difference between ONFA execution and classical NFA execution is that 𝜀-closure is not
sufficient in the case of ONFAs. We have to consider 𝜀-transitions that are either unguarded (similar
to NFAs) or guarded by (positive or negative) oracle queries. In order to check which oracle-guarded
𝜀-transitions are enabled, we have to use the oracle valuation for the current position. This is why
both Initial and Next in Fig. 2 take an oracle valuation (𝛽 : Vect(B)) as an additional argument.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 15 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

1

2

92:15

// Precondition: 𝛽 is defined on 𝑉
Function Initial(A : ONFA(Σ, 𝑉 ), 𝛽 : Vect(B)):
// Return the set of NFA states that are 𝛽-reachable from the initial states 𝐼 .
// Precondition: 𝛽 is defined on 𝑉
Function Next(A : ONFA(Σ, 𝑉 ), 𝑆 ⊆ 𝑄, 𝑎 : Σ, 𝛽 : Vect(B)):
// Let 𝑆 ′ be the set of ONFA states that are obtained from 𝑆 by
// transitioning on the character 𝑎.
// Return the set of ONFA states that are 𝛽-reachable from 𝑆 ′ .

// Precondition: the oracle matrix 𝑇 is defined on 𝑉
// Returns a Boolean tape 𝑡 : Vect(B) that contains the matching output for 𝑟 over ⟨𝑤, 𝛽¯⟩.
¯ [0, 𝑖 ] |= 𝑟 .
// If dir = L2R, then 𝑡 [𝑖 ] = 1 iff 𝑤, 𝛽,
¯ [𝑖, 𝑛] |= 𝑟 , where 𝑛 = |𝑤 |.
// If dir = R2L, then 𝑡 [𝑖 ] = 1 iff 𝑤, 𝛽,
∗
3 Function Match(dir : {L2R, R2L}, 𝑟 : OReg(Σ, 𝑉 ), 𝑤 : Σ , 𝑇 : Vect(Vect(B) )):
4
N 𝑛 ← |𝑤 | // length of string
// The vector 𝑇 contains 𝑘 = oarity(𝑟 ) tapes and each tape 𝑇 [𝑖 ] : Vect(B) has length 𝑛 + 1.
// So, 𝑇 can be turned into a sequence of oracle valuations.
5
Vect(Vect(B) ) 𝛽¯ ← transpose(𝑇 ) // it is not actually necessary to explicitly transpose
// 𝛽¯ is a sequence of length 𝑛 + 1 containing 𝑉𝑘 -valuations
// ⟨𝑤, 𝛽¯⟩ is an oracle-string over Σ and 𝑉𝑘
6
assert | 𝛽¯ | = 𝑛 + 1 // we need 𝑛 + 1 oracle valuations
// if dir = R2L, then the regex has to be reversed!!!
7
ONFA(Σ) A ← Oracle-NFA for the oracle-regex ite(dir = L2R, 𝑟, rev(𝑟 ) )
8
N 𝑖 ← ite(dir = L2R, 0, 𝑛) // position at which the matching starts
9
𝑆 ← Initial( A, 𝛽¯ [𝑖 ] ) // initial powerstate
10
Vect(B) out ← [0; 𝑛 + 1] // vector of size 𝑛 + 1, initialized to 0
11
if 𝑆 ∩ 𝐹 ≠ ∅ then tape[𝑖 ] ← 1 // 𝑆 contains a final state
12
else tape[𝑖 ] ← 0
13
Z 𝑑 ← ite(dir = L2R, 1, −1) // increment/decrement index based on direction
14
for _ = 0, 1, . . . , 𝑛 − 1 do // process the input text
15
𝑆 ← Next( A, 𝑆, 𝑤 [𝑖 ], 𝛽¯ [𝑖 + 𝑑 ] ) // compute the next powerstate
16
if 𝑆 ∩ 𝐹 ≠ ∅ then tape[𝑖 ] ← 1 // 𝑆 contains a final state
17
else tape[𝑖 ] ← 0
18
𝑖 ← 𝑖 +𝑑
19
return tape

Fig. 2. Algorithm for matching oracle regular expressions using ONFA simulation.

We will continue now to prove the main correctness result for the Match algorithm of Fig. 2.
Before we can prove this, we need to consider a semantic property of matching “in reverse”. The
reverse rev(𝑟 ) of an oracle regular expression 𝑟 is defined recursively as follows:
rev(𝜀) = 𝜀

rev(𝑟 1 + 𝑟 2 ) = rev(𝑟 1 ) + rev(𝑟 2 )

rev(𝑟 ∗ ) = rev(𝑟 ) ∗

rev(𝑝) = 𝑝

rev(𝑟 1 · 𝑟 2 ) = rev(𝑟 2 ) · rev(𝑟 1 )

rev(Q𝜎𝜀 (𝑣)) = Q𝜎𝜀 (𝑣)

where 𝜎 ∈ {+, -}. For example, rev(𝑎 · Q-𝜀 (𝑣 1 ) · 𝑑 · Q+𝜀 (𝑣 2 )) = Q+𝜀 (𝑣 2 ) · 𝑑 · Q-𝜀 (𝑣 1 ) · 𝑎.
Lemma 24 (Reversal). Let 𝑠 ∈ OReg(Σ, 𝑉 ), ⟨𝑤, ¯
𝛽⟩ ∈ O (Σ, 𝑉 ), and 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 |. Then,
¯ [𝑖, 𝑗] |= 𝑠 ⇐⇒ rev(𝑤), rev( ¯
𝑤, 𝛽,
𝛽), [|𝑤 | − 𝑗, |𝑤 | − 𝑖] |= rev(𝑠).
Proof. The proof is by induction on 𝑟 . For convenience, we use the following alternative characterization of the satisfaction relation:
¯ [𝑖, 𝑗] |= 𝜀 ⇐⇒ 𝑖 = 𝑗
𝑤, 𝛽,
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 16 -->

92:16

Konstantinos Mamouras and Agnishom Chattopadhyay

¯ [𝑖, 𝑗] |= 𝑝 ⇐⇒ 𝑗 = 𝑖 + 1 and 𝑝 (𝑤 (𝑖)) = 1
𝑤, 𝛽,
¯ [𝑖, 𝑗] |= 𝑟 1 + 𝑟 2 ⇐⇒ 𝑤, 𝛽,
¯ [𝑖, 𝑗] |= 𝑟 1 or 𝑤, 𝛽,
¯ [𝑖, 𝑗] |= 𝑟 2
𝑤, 𝛽,
¯ [𝑖, 𝑗] |= 𝑟 1 · 𝑟 2 ⇐⇒ there is 𝑘 with 𝑖 ≤ 𝑘 ≤ 𝑗 such that 𝑤, 𝛽,
¯ [𝑖, 𝑘] |= 𝑟 1 and 𝑤, 𝛽,
¯ [𝑘, 𝑗] |= 𝑟 2
𝑤, 𝛽,
¯ [𝑖, 𝑗] |= 𝑟 ∗ ⇐⇒ 𝑖 = 𝑗 or there is 𝑘 with 𝑖 < 𝑘 ≤ 𝑗 s.t. 𝑤, 𝛽,
¯ [𝑖, 𝑘] |= 𝑟 and 𝑤, 𝛽,
¯ [𝑘, 𝑗] |= 𝑟 ∗
𝑤, 𝛽,
¯ [𝑖, 𝑗] |= Q𝜎 (𝑣) ⇐⇒ 𝑖 = 𝑗 and 𝛽¯[𝑖] |= Q𝜎 (𝑣)
𝑤, 𝛽,
𝜀
𝜀
We will show how to handle the representative case of oracle queries:
¯ [𝑖, 𝑗] |= Q𝜎 (𝑣) ⇐⇒ 𝑖 = 𝑗 and 𝛽¯[𝑖] |= Q𝜎 (𝑣)
𝑤, 𝛽,
𝜀

𝜀

⇐⇒ |𝑤 | − 𝑖 = |𝑤 | − 𝑗 and rev( ¯
𝛽) [|𝑤 | − 𝑖] |= Q𝜎𝜀 (𝑣)
¯ [|𝑤 | − 𝑗, |𝑤 | − 𝑖] |= rev(Q𝜎 (𝑣))
⇐⇒ rev(𝑤), rev( 𝛽),
𝜀

because rev( ¯
𝛽) [|𝑤 | − 𝑖] = 𝛽¯[𝑖] and rev(Q𝜎𝜀 (𝑣)) = Q𝜎𝜀 (𝑣). The rest of the cases are handled with
similar arguments and we therefore omit them.

□

Proposition 25 (Correctness of Matching). Let 𝑠 ∈ OReg(Σ, 𝑉 ), 𝑤 ∈ Σ∗ with 𝑛 = |𝑤 |, and 𝛽¯ be
an oracle matrix defined over 𝑉 . The following hold:
¯ [𝑖] = 𝜒 (𝑤, 𝛽,
¯ [0, 𝑖], 𝑠) and
(1) Match(L2R, 𝑠, 𝑤, 𝛽)
¯
¯ [𝑖, 𝑛], 𝑠)
(2) Match(R2L, 𝑠, 𝑤, 𝛽) [𝑖] = 𝜒 (𝑤, 𝛽,
for every 𝑖 = 0, 1, . . . , 𝑛.
Proof. Part (1) can be proved with similar arguments as those that justify the simulation of
classical NFAs. The only difference is that we need to consider the current oracle valuation in order
to see whether an oracle-guarded 𝜀-transition is enabled or not.
¯ follows the same
For Part (2), the main observation is that the execution of Match(R2L, 𝑠, 𝑤, 𝛽)
¯
ONFA simulation steps as Match(L2R, rev(𝑠), rev(𝑤), rev( 𝛽)) and stores the output bits in reverse or¯ [0, 𝑖], rev(𝑠))
der. Let 𝑡 be the output Boolean tape. From Part (1) we get that 𝑡 [𝑖] = 𝜒 (rev(𝑤), rev( 𝛽),
¯
for every 𝑖. From Lemma 24, we get that 𝑡 [𝑖] = 𝜒 (𝑤, 𝛽, [|𝑤 | − 𝑖, |𝑤 |], 𝑠) for every 𝑖. Finally, the
¯ gives as output the reverse of 𝑡, which satisfies rev(𝑡) [𝑖] = 𝑡 [|𝑤 | −𝑖] =
execution Match(R2L, 𝑠, 𝑤, 𝛽)
¯
𝜒 (𝑤, 𝛽, [𝑖, |𝑤 |], 𝑠) for every 𝑖. This concludes the proof.
□
The matching algorithm of Fig. 2 proceeds in a single left-to-right (resp., right-to-left) pass over
the input string 𝑤 when dir = L2R (resp., dir = R2L). It performs 𝑂 (𝑚) work per step, so the total
running time is 𝑂 (𝑚 · 𝑛), where 𝑚 is the size of the o-regex and 𝑛 is the length of the input text.
4 EFFICIENT MATCHING
Using the algorithm for oracle-regex matching from the previous section, we will now describe
how regular expressions with lookaround can be efficiently matched. Our algorithm operates on
the nested structure of LReg. If there are one or more levels of lookaround, our algorithm makes
multiple forward or backward passes on the input string to extract the necessary information.
We have seen in Lemma 21 that by choosing appropriate oracle valuations, we can decide
membership in expressions with lookaround, by converting them to oracle expressions. In the
previous section, we saw that oracle regular expressions can be realized as ONFAs which behave
similarly to standard NFAs but additionally have oracle-guarded 𝜀-transitions. Our algorithm is
expressed in terms of a recursive function (EvalAux in Fig. 3) which traverses the regular expression
recursively. When a lookaround expression is found, the corresponding oracle tape is computed
and the lookaround expression is replaced with an oracle query. Ultimately, the resulting o-regex
and oracle tapes, which form an oracle matrix, are passed to an ONFA for matching.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 17 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:17

// Returns a pair (𝑘, 𝑠 ), where
//
(1) 𝑘 is the oracle-arity of 𝑟 , and
//
(2) 𝑠 ∈ OReg(Σ, 𝑉 ) with 𝑉 = {𝑜, 𝑜 + 1, . . . , 𝑜 + 𝑘 − 1}.
// It must be that 𝑠 = oproj𝑜 (𝑟 ) and 𝑘 = oarity(𝑟 ).
∗
1 Function EvalAux(𝑟 : LReg(Σ), 𝑤 : Σ , 𝑜 : N, 𝑇 : &mut Vect(Vect(B) )):
2
switch r do
3
case 𝜀 do return (0, 𝜀 )
4
case 𝑝 do return (0, 𝑝 ) // predicate (character class)
5
case 𝑟 1 ◦ 𝑟 2 where ◦ ∈ {+, ·} do
6
(𝑘 1 , 𝑠 1 ) ← EvalAux(𝑟 1 , 𝑤, 𝑜,𝑇 )
7
(𝑘 2 , 𝑠 2 ) ← EvalAux(𝑟 2 , 𝑤, 𝑜 + 𝑘 1 ,𝑇 )
8
return (𝑘 1 + 𝑘 2 , 𝑠 1 ◦ 𝑠 2 )
9
case 𝑟 1∗ do
10
(𝑘 1 , 𝑠 1 ) ← EvalAux(𝑟 1 , 𝑤, 𝑜,𝑇 )
11
return (𝑘 1 , 𝑠 1∗ )
12
case (?> 𝑟 1 ) do // positive lookahead
13
𝑇 .𝑝𝑢𝑠ℎ (Eval(R2L, 𝑟 1 , 𝑤 ) )
14
return (1, Q+𝜀 (𝑜 ) )
15
case (?≯ 𝑟 1 ) do // negative lookahead
16
𝑇 .𝑝𝑢𝑠ℎ (Eval(R2L, 𝑟 1 , 𝑤 ) )
17
return (1, Q-𝜀 (𝑜 ) )
18
case (?< 𝑟 1 ) do // positive lookbehind
19
𝑇 .𝑝𝑢𝑠ℎ (Eval(L2R, 𝑟 1 , 𝑤 ) )
20
return (1, Q+𝜀 (𝑜 ) )
21
case (?≮ 𝑟 1 ) do // negative lookbehind
22
𝑇 .𝑝𝑢𝑠ℎ (Eval(L2R, 𝑟 1 , 𝑤 ) )
23
return (1, Q-𝜀 (𝑜 ) )
// Returns a tape 𝑡 : Vect(B) that contains the matching output for 𝑟 over 𝑤.
// If dir = L2R, then 𝑡 [𝑖 ] = 1 iff 𝑤, [0, 𝑖 ] |= 𝑟 .
// If dir = R2L, then 𝑡 [𝑖 ] = 1 iff 𝑤, [𝑖, 𝑛] |= 𝑟 , where 𝑛 = |𝑤 |.
∗
24 Function Eval(dir : {L2R, R2L}, 𝑟 : LReg(Σ), 𝑤 : Σ ):
25
Vect(Vect(B) ) 𝑇 ← [ ] // empty vector of tapes
26
(𝑘, 𝑠 ) ← EvalAux(𝑟, 𝑤, 0, &mut 𝑇 )
// The vector 𝑇 contains 𝑘 = oarity(𝑟 ) Boolean tapes.
// Each tape 𝑇 [𝑖 ] : Vect(B) has length 𝑛 + 1, where 𝑛 = |𝑤 |.
27
return Match(dir, 𝑠, 𝑤,𝑇 )

Fig. 3. Algorithm for matching regular expressions with lookaround assertions.

Since ONFAs are simulated by maintaining a set of active control states (similarly to NFAs),
we are able to compute 𝜒 (𝑤, [0, 𝑖], 𝑟 ) : B for each position 𝑖, by running the ONFA in a single
left-to-right pass. These truth values are useful as they form an oracle tape that could be used in
evaluating an ONFA for a larger subexpression. For lookahead expressions (?> 𝑟 ), the truth values
𝜒 (𝑤, [𝑖, |𝑤 |], 𝑟 ) : B are required. To compute these values, the ONFA is executed in reverse.
Evaluation Algorithm. The overall algorithm for matching regular expressions with lookaround
is shown in Fig. 3. The top-level function is Eval and it uses the auxiliary function EvalAux to
recursively traverse the regular expression. EvalAux is similar to the shallow decomposition of
Definition 16 and it computes both the oracle-projection and the oracle matrix. It takes four inputs:
(1) a regular expression 𝑟 ∈ LReg(Σ) to evaluate,
(2) the input string 𝑤 ∈ Σ∗ ,
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 18 -->

92:18

Konstantinos Mamouras and Agnishom Chattopadhyay

(3) an index 𝑜 ∈ N from which to start numbering the maximal lookaround assertions in 𝑟 , and
(4) a mutable reference to a vector of Boolean tapes (which will form the Boolean matrix).
The function EvalAux returns a pair (𝑘, 𝑠), where 𝑘 = oarity(𝑟 ) and 𝑠 = oproj𝑜 (𝑟 ). Moreover,
suppose 𝑇 (resp., 𝑇 ′ ) is the value of the last argument before (resp., after) the execution of EvalAux.
Then, 𝑇 ′ = 𝑇 · 𝑇1 , where 𝑇1 = [𝑡 0′ , . . . , 𝑡𝑘′ −1 ] = Mat(𝑟, 𝑤). Notice that the lookaround cases in
EvalAux call Eval, which in turn calls EvalAux to the subexpressions of the lookaround assertions.
So, intuitively, Eval applies the shallow decomposition recursively to the top-level regex as well as
all regexes of lookarounds. The overall computation can be understood as a “deep decomposition”,
which we will formalize later in Section 5.
Theorem 26 (Correctness of Matching). Let 𝑟 ∈ LReg(Σ) and 𝑤 ∈ Σ∗ . The following hold:
(1) Eval(L2R, 𝑟, 𝑤) [𝑖] = 1 iff 𝑤, [0, 𝑖] |= 𝑟 and
(2) Eval(R2L, 𝑟, 𝑤) [𝑖] = 1 iff 𝑤, [𝑖, |𝑤 |] |= 𝑟
for every position 𝑖 = 0, 1, . . . , |𝑤 |.
Proof. For the sake of the proof, we can assume w.l.o.g. that the fourth argument of EvalAux
is a matrix of type Vect(Vect(B)) (i.e., it is passed by value) and EvalAux returns a triple (𝑘, 𝑠,𝑇 ),
where 𝑇 is the updated matrix. Properties (1) and (2) can be reformulated equivalently as follows:
(1) Eval(L2R, 𝑟, 𝑤) [𝑖] = 𝜒 (𝑤, [0, 𝑖], 𝑟 ) and
(2) Eval(R2L, 𝑟, 𝑤) [𝑖] = 𝜒 (𝑤, [𝑖, |𝑤 |], 𝑟 )
for every 𝑖 = 0, 1, . . . , |𝑤 |. We also claim the following property:
(3) Let 𝑜 ∈ N and 𝑇 : Vect(Vect(B)). Suppose that (𝑘, 𝑠,𝑇 ′ ) = EvalAux(𝑟, 𝑤, 𝑜,𝑇 ). Then, 𝑘 =
oarity(𝑟 ), 𝑠 = oproj𝑜 (𝑟 ) and 𝑇 ′ = 𝑇 · Mat(𝑟, 𝑤).
First, we will show that property (3) implies (1) and (2). Let (𝑘, 𝑠,𝑇 ) = EvalAux(𝑟, 𝑤, 0, [ ]). Property
(3) says that 𝑘 = oarity(𝑟 ), 𝑠 = oproj0 (𝑟 ) = oproj(𝑟 ) and 𝑇 = Mat(𝑟, 𝑤). We have to examine
two cases for the direction argument dir ∈ {L2R, R2L}. For the case dir = L2R, we have that
Eval(L2R, 𝑟, 𝑤) = Match(L2R, 𝑠, 𝑤,𝑇 ) = Match(L2R, oproj(𝑟 ), 𝑤, Mat(𝑟, 𝑤)) = 𝑅𝐻𝑆. From Proposition 25, 𝑅𝐻𝑆 [𝑖] = 𝜒 (𝑤, Mat(𝑟, 𝑤), [0, 𝑖], oproj(𝑟 )) for every 𝑖 = 0, 1, . . . , |𝑤 |. So, from Lemma 21,
we conclude that 𝑅𝐻𝑆 [𝑖] = 𝜒 (𝑤, [0, 𝑖], 𝑟 ) for every 𝑖 = 0, 1, . . . , |𝑤 |. Now, for dir = R2L and every
𝑖 = 0, 1, . . . , |𝑤 |, we similarly have that
Eval(R2L, 𝑟, 𝑤) [𝑖] = Match(R2L, 𝑠, 𝑤,𝑇 ) [𝑖]

[algorithm Eval]

= Match(R2L, oproj(𝑟 ), 𝑤, Mat(𝑟, 𝑤)) [𝑖]

[property (3)]

= 𝜒 (𝑤, Mat(𝑟, 𝑤), [𝑖, |𝑤 |], oproj(𝑟 ))

[Proposition 25]

= 𝜒 (𝑤, [𝑖, |𝑤 |], 𝑟 ).

[Lemma 21]

Since properties (1) and (2) follow from property (3), it suffices to establish property (3). The proof
is by induction on 𝑟 . We will only consider the representative cases of nondeterministic choice and
positive lookbehind, since the arguments for the rest of the cases are similar.
For the case 𝑟 1 +𝑟 2 , let (𝑘 1, 𝑠 1,𝑇1 ) = EvalAux(𝑟 1, 𝑤, 𝑜,𝑇 ) and (𝑘 2, 𝑠 2,𝑇2 ) = EvalAux(𝑟 2, 𝑤, 𝑜 +𝑘 1,𝑇1 ).
Since 𝑘 1 = oarity(𝑟 1 ) (I.H.) and 𝑘 2 = oarity(𝑟 2 ) (I.H.), 𝑘 1 + 𝑘 2 = oarity(𝑟 1 + 𝑟 2 ). From 𝑠 1 = oproj𝑜 (𝑟 1 )
(I.H.) and 𝑠 2 = oproj𝑜+𝑘1 (I.H.), we get that 𝑠 1 + 𝑠 2 = oproj𝑜 (𝑟 1 + 𝑟 2 ). Finally, 𝑇1 = 𝑇 · Mat(𝑟 1, 𝑤) (I.H.)
and 𝑇2 = 𝑇1 · Mat(𝑟 2, 𝑤) (I.H.) give us that 𝑇2 = 𝑇 · Mat(𝑟 1, 𝑤) · Mat(𝑟 2, 𝑤) = 𝑇 · Mat(𝑟 1 + 𝑟 2, 𝑤).
For the positive lookbehind assertion (?< 𝑟 ), let (𝑘, 𝑠,𝑇 ′ ) = EvalAux((?< 𝑟 ), 𝑤, 𝑜,𝑇 ). From
the algorithm we see that 𝑘 = 1, 𝑠 = Q+𝜀 (𝑜) and 𝑇 ′ = 𝑇 · [Eval(L2R, 𝑟, 𝑤)]. Notice that 𝑘 =
oarity((?< 𝑟 )) = 1 and 𝑠 = oproj𝑜 ((?< 𝑟 )) = Q+𝜀 (𝑜). We also observe that Mat((?< 𝑟 ), 𝑤) =
[𝑡], where 𝑡 [𝑖] = 𝜒 (𝑤, [0, 𝑖], 𝑟 ) for every 𝑖 = 0, 1, . . . , |𝑤 |. From property (1) we know that
Eval(L2R, 𝑟, 𝑤) [𝑖] = 𝜒 (𝑤, [0, 𝑖], 𝑟 ) for every 𝑖. So, 𝑇 ′ = 𝑇 · Mat((?< 𝑟 ), 𝑤).
□
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 19 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:19

Theorem 27 (Efficiency). The matching algorithm Eval of Fig. 3 needs 𝑂 (𝑚 ·𝑛) time and 𝑂 (𝑚 ·𝑛)
space, where 𝑚 is the size of the regular expression and 𝑛 is the length of the input string.
Proof. For 𝑟 ∈ LReg(Σ), define 𝐿(𝑟 ) to be the number of lookaround assertions that occur in 𝑟 .
For the memory bound, notice that at most 𝐿(𝑟 ) Boolean tapes are stored during the execution of
the algorithm. Each Boolean tape requires 𝑛 bits. So, the memory footprint is 𝑂 (𝑚 · 𝑛).
For the time bound, let us start with the high-level idea of the proof. The time complexity is
dominated by the cost of ONFA matching. Eval(𝑟 ) performs 𝐿(𝑟 ) + 1 calls to Match in total. The
𝑖-th call to Match has running time 𝑂 (𝑚𝑖 · 𝑛), where 𝑚𝑖 is the size of the 𝑖-th o-regex that is used for
Í
matching. But 𝑖 𝑚𝑖 = 𝑂 (𝑚), which means that the total running time is 𝑂 (𝑚 · 𝑛). The algorithm
constructs NFAs/ONFAs of total size 𝑂 (𝑚), so these constructions do not affect the time complexity.
To make the argument more formal, we need some definitions. For 𝑟 ∈ LReg(Σ), define size1 (𝑟 ) =
|𝑠 | and size2 (𝑟 ) = |𝑟 1 | + · · · + |𝑟𝑘 |, where (𝑠, 𝐷, 𝑅) = shallow(𝑟 ) and 𝑅 = [𝑟 1, . . . , 𝑟𝑘 ]. That is, size1 (𝑟 )
is the size of the oracle-projection of 𝑟 , and size2 (𝑟 ) is the sum of the sizes of the expressions that are
inside maximal lookaround assertions. Note that the quantities size1 (𝑟 ) and size2 (𝑟 ) would be same
if we used shallow𝑖 (for any 𝑖) instead of shallow in their definitions. It can be proved by induction
on 𝑟 that |𝑟 | = size1 (𝑟 ) + size2 (𝑟 ). For example, we have that shallow((?> 𝑟 )) = (Q+𝜀 (0), [R2L], [𝑟 ])
and therefore |(?> 𝑟 )| = 1 + |𝑟 | = |Q+𝜀 (0)| + |𝑟 | = size1 ((?> 𝑟 )) + size2 ((?> 𝑟 )).
The matching algorithm of Fig. 2 needs time at most 𝑓 (𝑟 ) = 𝑐 |𝑟 |𝑛, where 𝑟 is the input o-regex, 𝑛
is the length of the input text, and 𝑐 is a constant. We claim that (1) EvalAux needs time at most
𝑔(𝑟 ) = 𝑐 · size2 (𝑟 ) · 𝑛 for matching, and (2) Eval needs time at most 𝑓 (𝑟 ) for matching.
First, we show that property (1) implies property (2). Eval(𝑟 ) calls EvalAux(𝑟 ) and performs
matching using the oracle-projection 𝑠 = oproj(𝑟 ). So, the total matching time for Eval(𝑟 ) is
bounded above by 𝑔(𝑟 ) + 𝑓 (𝑠) = 𝑐 ·size2 (𝑟 ) ·𝑛+𝑐 ·size1 (𝑟 ) ·𝑛 = 𝑐 · (size1 (𝑟 ) +size2 (𝑟 )) ·𝑛 = 𝑐 |𝑟 |𝑛 = 𝑓 (𝑟 ).
Now, we show property (1) by induction on 𝑟 . For the case 𝑟 = 𝑟 1 + 𝑟 2 of nondeterministic choice,
we see that size2 (𝑟 ) = size2 (𝑟 1 ) + size2 (𝑟 2 ) and EvalAux(𝑟 ) calls EvalAux(𝑟 1 ) and EvalAux(𝑟 2 ). So,
the total matching time is bounded above by 𝑔(𝑟 1 ) + 𝑔(𝑟 2 ) = 𝑐 · size2 (𝑟 1 ) · 𝑛 + 𝑐 · size2 (𝑟 2 ) · 𝑛 =
𝑐 · (size2 (𝑟 1 ) + size2 (𝑟 2 )) · 𝑛 = 𝑐 · size2 (𝑟 ) · 𝑛 = 𝑔(𝑟 ). For the case 𝑟 = (?> 𝑟 1 ) of a positive lookahead,
notice that size2 (𝑟 ) = |𝑟 1 | and EvalAux(𝑟 ) calls Eval(𝑟 1 ). It follows that the matching time is
bounded above by 𝑓 (𝑟 1 ) = 𝑐 |𝑟 1 |𝑛 = 𝑐size2 (𝑟 )𝑛 = 𝑔(𝑟 ). The rest of the cases use similar arguments
and we therefore leave them to the reader.
□
Intuitive Explanation for Complexity Bound. The main reason behind the 𝑂 (𝑛 · 𝑚) bound
on time and space is that our algorithm does not construct a single automaton for the whole
regular expression, which would potentially cause an exponential blowup. Instead, we construct a
collection of small automata (NFAs and ONFAs), each of which corresponds to part of the original
regex. The sum of the sizes of all these NFAs/ONFAs is 𝑂 (𝑚). This is a representation that is as
succinct as the original regex, because we do not perform constructions that give rise to large
automata. Our algorithm uses only these small automata and (potentially) additional memory for
storing oracles tapes, where each oracle tape has size Θ(𝑛).
For example, let us consider a regex of the form 𝑟 ′ (?> 𝑟 0 )(?> 𝑟 1 ) · · · (?> 𝑟𝑘 −1 ), where 𝑟 ′ and
𝑟 0, 𝑟 1, . . . , 𝑟𝑘 −1 are lookaround-free. Our algorithm constructs (1) NFAs A0 , . . . , A𝑘 −1 for the regexes
𝑟 0 , . . . , 𝑟𝑘 −1 , and (2) an ONFA A ′ for the top-level oracle-regex 𝑟 ′ · Q+𝜀 (0) · · · Q+𝜀 (𝑘 − 1). Let 𝑚𝑖 be the
size of A𝑖 and 𝑚 ′ be the size of A ′ . The total size 𝑚 0 +· · ·+𝑚𝑘 −1 +𝑚 ′ of the automata is proportional
to the size of the original regex. We do not construct a product automaton, even though the
lookarounds describe some kind of intersection, as the product automaton would be of exponential
size. Our algorithm simulates these small automata. Let 𝑛 be the length of the input text. First, each
NFA A𝑖 is reversed and simulated with a right-to-left pass over the input (work proportional to 𝑚𝑖 𝑛).
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 20 -->

92:20

Konstantinos Mamouras and Agnishom Chattopadhyay

Then, the ONFA A ′ is simulated with a left-to-right pass over the input (work proportional to 𝑚 ′𝑛).
So, the total work performed is proportional to 𝑚 0𝑛 + · · · + 𝑚𝑘 −1𝑛 + 𝑚 ′𝑛 = (𝑚 0 + · · · + 𝑚𝑘 −1 + 𝑚 ′ )𝑛.
The only pre-processing performed by our algorithm happens in the Match procedure of Fig. 2 (see
line 7). It involves Thompson-style constructions to obtain NFAs and ONFAs from oracle-regexes.
These constructions can be performed in time 𝑂 (𝑚), where 𝑚 is the size of the regex.
Extracting Matches. The literature on automata and formal languages generally focuses on the
membership problem for regular expressions: given 𝑤 ∈ Σ∗ and 𝑟 ∈ LReg(Σ), is it the case that
𝑤, [0, |𝑤 |] |= 𝑟 ? To answer this question, we can simply look at the last element of Eval(L2R, 𝑤, 𝑟 ).
However, regular expressions with lookaround are often used to specify additional constraints
on the context in which a substring appears without capturing the context itself. For instance,
telephone numbers have the form 𝑥𝑥𝑥−𝑦𝑦𝑦−𝑧𝑧𝑧𝑧, where 𝑥𝑥𝑥 is the area code. One might use the
regular expression [0-9]{3}(?=-[0-9]{3}-[0-9]{4}) to extract the area code. For such a task, the
match extraction problem is of more interest than the membership problem. A match for 𝑟 in 𝑤 is
a pair [𝑖, 𝑗] of indices with 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑤 | such that 𝑤, [𝑖, 𝑗] |= 𝑟 . The leftmost longest match is
the longest out of the leftmost matches (it can be easily seen that it is unique). The computational
problem of extracting matches (and sub-matches) has been considered before (see, e.g, the notes of
Cox [2010]). The following two-step procedure uses the algorithm of Fig. 3 to efficiently extract the
leftmost longest match for a given regular expression:
(1) Find the smallest index 𝑖 such that 𝑤, [𝑖, |𝑤 |] |= 𝑟 · Σ∗ using the output of Eval(R2L, 𝑤, 𝑟 · Σ∗ ).
(2) Then, find the largest 𝑗 with 𝑤, [𝑖, 𝑗] |= 𝑟 using Eval(L2R) on the suffix of 𝑤 starting at 𝑖.
We can also consider match extraction when a match other than the leftmost longest one is preferred.
Lookaround and Temporal Monitoring. The use of lookaround in regular expressions is reminiscent of the use of temporal connectives in temporal logic, which has found applications in
runtime verification and online monitoring [Bartocci et al. 2018]. More specifically, lookahead (resp.,
lookbehind) is similar to future-time (resp., past-time) temporal connectives. The problem of (online
or offline) temporal monitoring is analogous to the matching problem for regular expressions. It
seems possible that the compositional regex matching algorithm of Fig. 3 can be combined with
efficient and modular algorithms for temporal monitoring (see, e.g., [Chattopadhyay and Mamouras
2020; Dokhanchi et al. 2014; Maler et al. 2008; Mamouras et al. 2021a,b, 2023; Mamouras and Wang
2020; Thati and Roşu 2005]) in order to support more expressive temporal specification formalisms.
5 PERFORMANCE OPTIMIZATIONS
The algorithm of Fig. 3, presented in the previous section, provides strong worst-case performance
guarantees. The upper bound 𝑂 (𝑚 · 𝑛) for the running time is the same as the complexity of
Thompson’s algorithm [Thompson 1968], which only handles classical regular expressions (i.e., no
lookaround). In order to provide a practical implementation, we will introduce in this section three
performance optimizations that can reduce both the amount of work and memory needed for some
regular expressions. We will see later in Section 6 through an experimental evaluation that these
optimizations are significant in practice.
5.1 Common Assertion Elimination
In Definition 16, we introduced the concept of shallow decomposition of a regular expression 𝑟 ,
which allows us to reduce the evaluation of 𝑟 to the simulation of an ONFA, assuming that we
have access to oracles that resolve the truth values of the lookaround assertions. Computationally,
the algorithm of Fig. 3 performs a shallow decomposition with each invocation of Eval. In order
to compute the oracle tapes, Eval is applied recursively whenever a lookaround assertion is
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 21 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:21

// Returns a tape 𝑡 : Vect(B) that contains the matching output for 𝑟 over 𝑤.
// If dir = L2R, then 𝑡 [𝑖 ] = 1 iff 𝑤, [0, 𝑖 ] |= 𝑟 .
// If dir = R2L, then 𝑡 [𝑖 ] = 1 iff 𝑤, [𝑖, 𝑛] |= 𝑟 , where 𝑛 = |𝑤 |.
∗
1 Function EvalDeep(dir : {L2R, R2L}, 𝑟 : LReg(Σ), 𝑤 : Σ ):
// compute the deep decomposition of the regex 𝑟
2
OReg(Σ) × Vect( {L2R, R2L} ) × Vect(OReg(Σ) ) (𝑠, 𝐷, 𝑅) ← deep(𝑟 )
3
N 𝑘 ← 𝑅.len( ) // number of lookaround assertions
4
Vect(Vect(B) ) 𝑇 ← [ [ ]; 𝑘 ] // vector of 𝑘 empty Boolean tapes
5
for 𝑖 = 0, 1, . . . , 𝑘 − 1 do // process lookaround assertions in topological order
6
𝑇 [𝑖 ] ← Match(𝐷 [𝑖 ], 𝑅 [𝑖 ], 𝑤,𝑇 )
// Now, the vector 𝑇 contains 𝑘 tapes and each tape 𝑇 [𝑖 ] : Vect(B) has length 𝑛 + 1.
7
return Match(dir, 𝑠, 𝑤,𝑇 )

Fig. 4. Algorithm for matching regular expressions with lookaround assertions.

encountered. The overall effect is a decomposition that goes deeper than what the definition of
shallow𝑖 suggests. In order to illuminate this concept, we introduce here the concept of a “deep
decomposition”, which has a close correspondence to the algorithm of Fig. 3.
The deep decomposition separates all lookaround assertions, regardless of whether they are
maximal or not. This decomposition does not cause an increase in size, because oracle queries are
used to refer to lookaround assertions at all levels.
Definition 28 (Deep Decomposition). For every index 𝑖 ∈ N, we define the deep decomposition
function deep𝑖 : LReg(Σ) → OReg(Σ) × Vect({L2R, R2L}) × Vect(OReg(Σ)) as follows:
deep𝑖 (𝜀) = (𝜀, [ ], [ ]) and deep𝑖 (𝑝) = (𝑝, [ ], [ ])
deep𝑖 (𝑟 1 ◦ 𝑟 2 ) = (𝑠 1 ◦ 𝑠 2, 𝐷 1 · 𝐷 2, 𝑅1 · 𝑅2 ), where
(𝑠 1, 𝐷 1, 𝑅1 ) = deep𝑖 (𝑟 1 ) and (𝑠 2, 𝐷 2, 𝑅2 ) = deep𝑖+|𝑅1 | (𝑟 2 )
∗

deep𝑖 (𝑟 ) = (𝑠 ∗, 𝐷, 𝑅), where (𝑠, 𝐷, 𝑅) = deep𝑖 (𝑟 )
deep𝑖 ((?> 𝑟 )) = (Q+𝜀 (𝑖 + |𝑅|), 𝐷 · [R2L], 𝑅 · [𝑠]), where (𝑠, 𝐷, 𝑅) = deep𝑖 (𝑟 )
deep𝑖 ((?≯ 𝑟 )) = (Q-𝜀 (𝑖 + |𝑅|), 𝐷 · [R2L], 𝑅 · [𝑠]), where (𝑠, 𝐷, 𝑅) = deep𝑖 (𝑟 )
deep𝑖 ((?< 𝑟 )) = (Q+𝜀 (𝑖 + |𝑅|), 𝐷 · [L2R], 𝑅 · [𝑠]), where (𝑠, 𝐷, 𝑅) = deep𝑖 (𝑟 )
deep𝑖 ((?≮ 𝑟 )) = (Q-𝜀 (𝑖 + |𝑅|), 𝐷 · [L2R], 𝑅 · [𝑠]), where (𝑠, 𝐷, 𝑅) = deep𝑖 (𝑟 )
where 𝑝 is a character class and ◦ ∈ {+, ·}. Finally, we define deep(𝑟 ) = deep0 (𝑟 ).
Suppose that deep(𝑟 ) = (𝑠, 𝐷, 𝑅) with 𝑅 = [𝑠 0, 𝑠 1, . . . , 𝑠𝑘 −1 ]. Then, each o-regex 𝑠𝑖 contains oracle
queries that only refer to assertions 𝑠 𝑗 with 𝑗 < 𝑖.
Example 29. Consider the regex 𝑟 = ((?! )(?!(?<![0-9])0)[0-9 ])+ (in PCRE notation), which
is the same as 𝑟 = ((?≯ ␣ ␣ Σ∗ )(?≯ (?≮ Σ∗ [0−9])0Σ∗ ) [0−9␣]) + . Its deep decomposition is deep(𝑟 ) =
(𝑠, [R2L, L2R, R2L], [𝑠 0, 𝑠 1, 𝑠 2 ]), where
𝑠 0 = ␣ ␣ Σ∗

𝑠 1 = Σ∗ [0−9]

𝑠 2 = Q-𝜀 (1) · 0Σ∗

𝑠 = (Q-𝜀 (0) · Q-𝜀 (2) · [0−9 ␣ ]) + .

The deep decomposition of a regex 𝑟 ∈ LReg(Σ) gives us a sequence [𝑠 0, 𝑠 1, . . . , 𝑠𝑘 −1 ] of 𝑘 oregexes in topological order with respect to the evaluation dependencies that they have. The means
that they can be evaluated in the given order. The output tapes of earlier o-regexes are used as
oracle tapes for later o-regexes. This is essentially a reformulation of the algorithm Eval of Fig. 3.
See the algorithm EvalDeep of Fig. 4 for the implementation of this idea. Compared to Eval, the
version EvalDeep makes the order of evaluation of o-regexes more explicit.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 22 -->

92:22

Konstantinos Mamouras and Agnishom Chattopadhyay

// Returns a tape 𝑡 : Vect(B) that contains the matching output for 𝑟 over 𝑤.
// It holds that 𝑡 [𝑖 ] = 1 iff 𝑤, [0, 𝑖 ] |= 𝑟 .
∗
1 Function EvalL2R(𝑟 : LReg(Σ), 𝑤 : Σ ):
2
N 𝑛 ← |𝑤 | // length of input string
3
OReg(Σ) × Vect( {L2R, R2L} ) × Vect(OReg(Σ) ) (𝑠, 𝐷, 𝑅) ← deep(𝑟 )
4
N 𝑘 ← 𝑅.len( ) // number of lookaround assertions
5
for 𝑗 = 0, 1, . . . , 𝑘 − 1 do // only lookbehind assertions
6
assert 𝐷 [ 𝑗 ] = L2R
7
Vect(B) tape ← [0; 𝑛 + 1] // output vector of size 𝑛 + 1, initialized to 0
// Calculate oracle valuation for position 0.
8
Vect(B) 𝛽 ← [0; 𝑘 ] // oracle valuation initialized to 0
9
for 𝑗 = 0, 1, . . . , 𝑘 − 1 do // process lookaround assertions in topological order
10
ONFA(Σ) A 𝑗 ← Oracle-NFA for the o-regex 𝑅 [ 𝑗 ]
11
𝑆 𝑗 ← Initial( A 𝑗 , 𝛽 ) // initial powerstate
12
𝛽 [ 𝑗 ] ← ite(𝑆 𝑗 ∩ 𝐹 𝑗 ≠ ∅, 1, 0) // 𝐹 𝑗 is the set of final states of A 𝑗
13
ONFA(Σ) A ← Oracle-NFA for the top-level o-regex 𝑠
14
𝑆 ← Initial( A, 𝛽 ) // initial powerstate
15
tape[0] ← ite(𝑆 ∩ 𝐹 ≠ ∅, 1, 0) // 𝐹 is the set of final states of A
16
for 𝑖 = 0, 1, . . . , 𝑛 − 2, 𝑛 − 1 do // process the input text
17
for 𝑗 = 0, 1, . . . , 𝑘 − 1 do // process lookaround assertions in topological order
18
𝑆 𝑗 ← Next( A 𝑗 , 𝑤 [𝑖 ], 𝛽 )
19
𝛽 [ 𝑗 ] ← ite(𝑆 𝑗 ∩ 𝐹 𝑗 ≠ ∅, 1, 0)
20
𝑆 ← Next( A, 𝑆, 𝑤 [𝑖 ], 𝛽 )
21
tape[𝑖 ] ← ite(𝑆 ∩ 𝐹 ≠ ∅, 1, 0)
22
return tape

Fig. 5. Algorithm for matching regular expressions with lookbehind-only assertions. A completely symmetric
algorithm handles regular expresssions with lookahead-only assertions.

The advantage of the formulation of EvalDeep is that we can easily redefine deep in order to
avoid the duplication of lookaround assertions. As an example, consider the regex
𝑟 = (?=a(?<=c))(?=b(?<=c)) = (?> 𝑎(?< Σ∗𝑐)Σ∗ ) · (?> 𝑏 (?< Σ∗𝑐)Σ∗ ).
The algorithm of Fig. 3 computes (?<=c) twice. We can avoid this duplication of work in EvalDeep
by modifying the deep decomposition to only create a new o-regex when it encounters a new
lookaround assertion. For the example 𝑟 above, we would then have (𝑠, 𝐷, 𝑅) = deep(𝑟 ), where
𝑠 = Q+𝜀 (1) · Q+𝜀 (2), 𝐷 = [L2R, R2L, R2L], and 𝑅 = [Σ∗𝑐, 𝑎 · Q+𝜀 (0) · Σ∗, 𝑏 · Q+𝜀 (0) · Σ∗ ]. We call this
optimization common assertion elimination (similar to the common subexpression elimination used
in compiler optimization).
5.2 Improving the Memory Footprint
An important memory-saving optimization is enabled when all assertions are lookaheads or all of
them are lookbehinds. When this holds, we say that the regular expression is unidirectional. In this
case, we see in Fig. 3 and Fig. 4 that all ONFA simulations are performed in the same direction. For
this reason, we do not need to store oracle tapes with intermediate outputs. Instead, we can pipe
the output from an ONFA to be used by other ONFAs that depend on it.
This idea is implemented in the function EvalL2R of Fig. 5 for the case where all lookaround
assertions are lookbehinds. The case where all assertions are lookaheads is completely symmetric.
For every o-regex 𝑠 𝑗 of the deep decomposition, the algorithm simulates the corresponding ONFA
A 𝑗 . At every step, the ONFAs are processed in topological order in order to ensure that each ONFA
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 23 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:23

// Returns a tape 𝑡 : Vect(T) that contains the matching output for 𝑟 over 𝑤.
// If 𝑡 [𝑖 ] = 1 then 𝑤, [0, 𝑖 ] |= 𝑟 .
// If 𝑡 [𝑖 ] = 0 then 𝑤, [0, 𝑖 ] ̸ |= 𝑟 .
∗
1 Function PreEval(𝑟 : LReg(Σ), 𝑤 : Σ ):
2
N 𝑛 ← |𝑤 | // length of input string
3
OReg(Σ) 𝑠 ← oproj(𝑟 ) // top-level o-regex (“oracle-projection”)
4
ONFA(Σ) A ← oracle-NFA for the top-level o-regex 𝑠
5
NFA(Σ) A ⊤ ← Replace oracle-guarded transitions in A by 𝜀-transitions
6
NFA(Σ) A ⊥ ← Remove oracle-guarded transitions of A
// A ⊤ (resp., A ⊥ ) is an over-approximation (resp., under-approximation) of A
7
Vect(T) tape ← [0; 𝑛 + 1] // output vector of size 𝑛 + 1, initialized to 0
8
𝑆 ⊤ ← Initial( A ⊤ ) // initial powerstate
9
𝑆 ⊥ ← Initial( A ⊥ ) // initial powerstate
10
if 𝑆 ⊥ ∩ 𝐹 ≠ ∅ then tape[𝑖 ] ← 1
11
else if 𝑆 ⊤ ∩ 𝐹 ≠ ∅ then tape[𝑖 ] ← ?
12
else tape[𝑖 ] ← 0
13
for 𝑖 = 0, 1, . . . , 𝑛 − 1 do // process the input text
14
𝑆 ⊤ ← Next( A ⊤ , 𝑆 ⊤ , 𝑤 [𝑖 ] )
15
𝑆 ⊥ ← Next( A ⊤ , 𝑆 ⊥ , 𝑤 [𝑖 ] )
16
if 𝑆 ⊥ ∩ 𝐹 ≠ ∅ then tape[𝑖 ] ← 1
17
else if 𝑆 ⊤ ∩ 𝐹 ≠ ∅ then tape[𝑖 ] ← ?
18
else tape[𝑖 ] ← 0
19
return tape

Fig. 6. An approximate algorithm for matching regular expressions with lookaround assertions. If this
algorithm indicates that the output is uncertain, then one of the previous algorithms has to be used.

has the oracle valuation that it needs. The ONFA A for the top-level o-regex 𝑠 is always processed
last, as it may need the output values from all other ONFAs.
The intuition for the algorithm of Fig. 5, when compared to the algorithm of Fig. 4, is that the
evaluation of the output matrix proceeds column-by-column instead of row-by-row. Since only the
most recent column of the matrix is needed for the next steps, we do not need to store the entire
matrix. So, we store only the last column and we update it at every step. This reduces the memory
footprint from 𝑂 (𝑚 · 𝑛) to 𝑂 (𝑚).
5.3 Approximation for Saving Work
We also consider an optimization where the computation of lookaround assertions can be avoided
altogether when they are not necessary for producing the output. For example, consider a regex of
the form 𝑟 = Σ∗ · 𝑎𝑏𝑐𝑑 · 𝑟 1 , where 𝑟 contains several lookaround assertions. If the input text contains
no occurrence of the string 𝑎𝑏𝑐𝑑, then it cannot contain any match for 𝑟 . In this case, we do not
have to compute any of the lookaround assertions, because their values are not needed at all.
This idea is made more precise in the algorithm PreEval of Fig. 6. Given a regular expression
𝑟 ∈ LReg(Σ), we first compute its oracle-projection 𝑠 ∈ OReg(Σ, 𝑉𝑘 ) where 𝑘 = oarity(𝑟 ). We will
attempt to compute the output without knowing the truth values of the oracle queries. In order to
do this, we will approximate the ONFA A for 𝑠 using two NFAs. The NFA A ⊤ is obtained from A
𝜎
by replacing each oracle-guarded transition of the form 𝑞 →Q 𝜀 (𝑣) 𝑞 ′ by an 𝜀-transition 𝑞 →𝜀 𝑞 ′ . So,
it over-approximates A, that is, ⟦A⟧ ⊆ ⟦A ⊤ ⟧. The NFA A ⊥ is derived from A by removing all
oracle-guarded transitions. So, it under-approximates A, that is, ⟦A ⊥ ⟧ ⊆ ⟦A⟧. We examine cases:
(1) A ⊥ accepts: It must also be the case that A and A ⊤ accept.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 24 -->

92:24

Konstantinos Mamouras and Agnishom Chattopadhyay

(2) A ⊥ rejects and A ⊤ accepts: We do not know whether A accepts or rejects.
(3) A ⊤ rejects: It must also be the case that A and A ⊥ reject.
In cases (1) and (3) the matching output is certain to be correct. Case (2) gives rise to uncertainty
regarding the output. If case (2) never occurs during the evaluation, then we do not need to evaluate
lookaround assertions at all. If case (2) occurs, then we have to use the previous algorithm in order
to ensure the correctness of the output.
Example 30 (Over- and under-approximation). Consider the regular expression 𝑟 = Σ∗ · (𝑟 1 +𝑟 2 ),
where 𝑟 1 = bbb(?=a)((?!aaa).)+(?<=a)bbb and 𝑟 2 = bbbbb . Using the expression 𝑟 , we search for
the occurrence of substrings that match 𝑟 1 or 𝑟 2 . Intuitively, the over-approximation replaces 𝑟 1 by
bbb.+bbb , and the under-approximation replaces 𝑟 1 by ∅. The table below shows the output values
computed by the algorithm PreEval of Fig. 6 on the input aabbbaabaaabbbbbaa .
text
A⊤
A⊥

a
0
0

a
0
0

b
0
0

b
0
0

b
0
0

a
0
0

a
0
0

b
0
0

a
0
0

a
0
0

a
0
0

b
0
0

b
0
0

b
1
0

b
1
0

b
1
1

a
1
1

a
1
1

The bold output values indicate that the output of PreEval is uncertain.
6 EXPERIMENTAL EVALUATION
We have implemented the pattern matching algorithms of Sections 3, 4 and 5 using the Rust
programming language. We perform an experimental evaluation in order to answer the following
two research questions:
(1) Do the optimizations of Section 5 provide a significant performance benefit?
(2) Does our implementation have competitive performance against state-of-the-art regex engines that support lookaround assertions?
There are many implementations of matching regexes with lookaround that are based on backtracking search instead of using automata-based algorithms. It is known that backtracking suffers from
superlinear complexity (in the size of the input text). In fact, for some regexes the time complexity
is exponential. This behavior is well-known and is often called “catastrophic backtracking”. We will
compare against the PCRE (Perl Compatible Regular Expressions) library (more specifically, the
PCRE2 library) [The PCRE2 Developers 2023], which is implemented in C, and Java’s regex library.
We have chosen to compare with PCRE2 and Java’s regex library because they are widely used
regex engines that support lookaround assertions. Note that PCRE2 is implemented in a low-level
programming language (and thus avoids the overheads of virtual machines such as the JVM).
Benchmarking Datasets. We have performed experiments using two widely used datasets of
regular expressions that use PCRE syntax: (1) the Snort dataset [Snort 2023] and (2) the Suricata
dataset [Suricata 2023]. Both these datasets specify signatures for network traffic that may indicate
malicious network activity. We use real network traffic (obtained from https://archive.wrccdc.org/
pcaps/) as input strings for the experimental evaluation.
The average length of the regexes in Snort (resp., Suricata) is 109 (resp., 102). This refers to the
length of the textual representation using the PCRE syntax of the datasets.
Roughly 5.5% of regexes in Snort (7.6% in Suricata) contain lookaround. Out of the regexes with
lookaround in Snort, 97% contain lookahead and 10% contain lookbehind, which means that 7%
contain both lookahead and lookbehind. For Suricata, the corresponding percentages are 98% for
lookahead, 8% for lookbehind, and 6% for both. The average number of lookaround assertions
(resp., lookahead assertions and lookbehind assertions) in a regex that contains lookaround in Snort
is 1.96 (resp., 1.85 and 0.11). For Suricata, the corresponding average numbers are 1.84, 1.74 and
0.10 respectively (for lookaround, lookahead and lookbehind assertions). Out of the regexes with
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 25 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:25

Running time for matching (average over dataset)
snort

suricata

running time (in msec)

0.5
0.4

tool
0.3
ours_opt
0.2

ours_base

0.1
0.0
2500

5000

7500

10000

2500

5000

7500

10000

length of input text

Fig. 7. Comparison between base algorithm and optimized algorithm.
Running time for matching (average over dataset)

running time (in msec)

snort

suricata

1e+02

tool

1e+01

ours_opt
1e+00

ours_base
java

1e-01

pcre

1e-02
2500

5000

7500

10000

2500

5000

7500

10000

length of input text

Fig. 8. Comparison of our algorithms with PCRE and Java’s regex engine.

lookaround in Snort and Suricata, 96% and 97% respectively have lookaround depth 1 (which means
that 4% and 3% respectively have nested lookaround assertions).
Effect of performance optimizations. Fig. 7 shows the performance of the basic version of our
matching algorithm (called ours_base in the figure). This is the implementation of the algorithm of
Fig. 3. The version that is called ours_opt in the figure also incorporates the optimizations described
in Section 5: avoiding work duplication due to multiple occurrences of the same lookaround
assertion, one-pass matching when the lookaround assertions are unidirectional, and the use of
approximation to avoid the computation of some oracle truth values.
Fig. 7 contains two plots, one for each regex dataset, namely Snort and Suricata. The horizontal
axis of each plot shows the length of the input string. The vertical axis shows the average running
time of the regex matching algorithm in milliseconds. The average is taken over the entire dataset
of regexes. Each point is annotated with error bars that show the standard deviation of the running
time (the errors bars are too small to see). A crucial observation is that the running time of both
versions of our algorithm is linear in the length of the input string. This behavior is consistent
with the time complexity analysis of Theorem 27. The other observation is that the optimized
version of our algorithm (ours_opt) is substantially faster than the basic version (ours_base). More
specifically, the optimizations result in a speedup of at least 10× across all experiments of Fig. 7.
Comparison with PCRE and Java’s regex engine. In Fig. 8 we include the performance of PCRE2
and Java’s regex engine. The plots are similar to the ones of Fig. 7. One difference is that the vertical
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 26 -->

92:26

Konstantinos Mamouras and Agnishom Chattopadhyay

Running time for matching

running time (in msec)

micro1

micro2

micro3

1500

tool
1000

ours
java
pcre

500

0
3

6

9

3

6

9

3

6

9

parameter k

Fig. 9. Microbenchmarks: Comparison between our implementation and PCRE, Java.

axis in the plots of Fig. 8 is log-scaled. Using logarithmic scale for the running time is necessary
due to the big difference in running time between our implementation and the other tools. Our
first observation is that the running time of both PCRE2 and Java is superlinear with respect
to the length of the input string. This is witnessed by the widening gap between the green and
red curves (ours_base and ours_opt respectively) and the blue and purple curves (java and pcre
respectively) as the string length grows. The ratio between pcre and ours_opt is at least 250× for
text length 1000. It grows to at least 4000× for text length 10000. Similar observations can be made
for Java. So, our regex engine is several orders of magnitude faster than PCRE and Java across
all experiments of Fig. 8.
Microbenchmarks. We also consider microbenchmarks that focus on cases that do not trigger
super-linear behavior for backtracking engines (PCRE and Java). First, we consider the family
(𝜌𝑘 )𝑘 ≥2 of regexes of the form 𝜌𝑘 = 𝑟 (?= 𝑟 1 )(?= 𝑟 2 ) · · · (?= 𝑟𝑘 ), where 𝑟, 𝑟 1, 𝑟 2, . . . , 𝑟𝑘 are lookaroundfree signatures. We also consider the family (𝜌𝑘′ )𝑘 ≥2 of regexes of the form
𝜌𝑘′ = 𝑟 ′ (?= (.{2}) +𝑟 # )(?= (.{3}) +𝑟 # ) · · · (?= (.{𝑘 }) +𝑟 # ),

running time (in msec)

where 𝑟 # is a signature that has the role of an “end-of-block” marker. The regex family (𝜌𝑘′ )𝑘 is
inspired from the regex family 𝑇𝑛 in section 3.6 of [Miyazaki and Minamide 2019]. The regexes
(𝑇𝑛 )𝑛 witness the doubly exponential lower bound for DFAs that encode regexes with lookahead.
Finally, we define the regex family (𝜌𝑘′′ )𝑘 ≥2 by 𝜌𝑘′′ = 𝑟 ′′ ((?= 𝑟 1′′ ) + (?= 𝑟 2′′ ) + · · · + (?= 𝑟𝑘′′ )), where
𝑟 ′′, 𝑟 1′′, 𝑟 2′′, . . . , 𝑟𝑘′′ are lookaround-free signatures.
Running time for matching
The regex families 𝜌𝑘 and 𝜌𝑘′ use lookahead assertions
in a way that encodes a form of intersection. They would
pose a substantial challenge on algorithms that construct
200
dataset
a single automaton through a product construction, as
150
micro1
this would cause an exponential blowup in size. The regex
′′
micro2
family 𝜌𝑘 involves a nondeterministic choice over looka100
micro3
′′
′
head assertions. The regex families 𝜌𝑘 , 𝜌𝑘 , 𝜌𝑘 correspond
to the microbenchmarks called micro1, micro2 and mi50
cro3 respectively. Fig. 9 shows experimental results for
3
6
9
the performance of our implementation, PCRE and Java’s
parameter k
regex engine over these 3 microbenchmarks. The experiments use input text of length 106 . The horizontal axis corresponds to the parameter 𝑘. The vertical
axis shows the matching running time in milliseconds. All regex engines seem to have running
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 27 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:27

time that is linear in parameter 𝑘. Note that the running time of our implementation does not blow
up because we do not construct large automata, as explained earlier in Section 4. The plot shown
above (on the right) focuses on the performance of our implementation over all 3 microbenchmarks.
Observe that the running time of our implementation is linear in 𝑘.
Experimental Setup & Measurement Methodology. The experiments were executed in Ubuntu
20.04 on a desktop computer equipped with an Intel Xeon W-2295 CPU (18 cores) and 64 GB of
RAM. We used version 1.71.0 of the Rust compiler. The PCRE2 library was installed using the
libpcre2-dev package through usual repositories. At the time of executing the experiments, the
10.39-3ubuntu0.1 version of the package was used.
Each measurement of running time (for a matching algorithm that is given a regex and a string
as input) is taken as the average of 10 trials. The uncertainty in the measurement is quantified
using the standard deviation of the 10 trials.
7 RELATED WORK
Constructs similar to (positive and negative) lookahead assertions have been popular in the construction of parsers. Specifying a lookahead assertion in a parser can be used to reduce ambiguity
(and thus limit backtracking, for backtracking implementations). For instance, parsers for contextfree languages are often classified by the number of tokens the parser may need to peek ahead.
We also see the use of lookahead in [Sakuma et al. 2012] where it is used to transform nondeterministic transducers into deterministic ones. Regular lookahead is used in the language Bex
[Veanes 2015], which is used for specifying string transformations. The so-called “And-predicates”
and “Not-predicates” in parsing expression grammars (PEGs) [Ford 2004] correspond to positive
and negative lookahead assertions respectively. Miyazaki and Minamide [2021] have proposed
extensions of context-free grammars with lookahead.
Lookaround assertions are often used to extract data that arise in specific contexts. The language
CDuce [Benzaken et al. 2003] uses regular expression types to extract data from XML documents.
The Kleenex language [Grathwohl et al. 2016] uses regular expressions as grammars (types) to
describe string transductions that can extract data from streams. This involves a “greedy” disambiguation policy that generalizes greedy regex parsing [Frisch and Cardelli 2004; Grathwohl et al.
2013, 2014a; Nielsen and Henglein 2011].
The use of derivatives for matching regular languages is popular in functional and formally
verified implementations. The simplest form are Brzozowski’s derivatives [Brzozowski 1964] and
they lend themselves to a natural functional implementation of an implicit DFA of the underlying
regular expression. Coquand and Siles [2011] present a formally verified framework for deciding
equivalence of regular expressions based on Brzozowski derivatives. Brzozowoski has shown that
the number of derivatives are finite if they are simplified using associativity, idempotence and
commutativity rules. Recent work [Egolf et al. 2022] shows how these optimizations could be
incorporated in practice into a verified implementation. The size of a Brzozowoski derivative can
be large. Antimirov [1996] suggested using sets of partial derivatives for a more efficient algorithm.
This is also related to the technique of prebases discussed by Mirkin [1966] (see also [Brzozowski
1971] and [Champarnaud and Ziadi 2001]). Partial derivatives have been used for formally verified
implementations in [Komendantsky 2012] and [Moreira et al. 2012].
Doczkal et al. [2013] have developed a comprehensive formalization of regular languages in Coq
which encompasses regular expressions, NFAs, DFAs, and the Myhill-Nerode Theorem. We see
another NFA based formalization in [Firsov and Uustalu 2013], where NFAs are simulated using
their matrix representations in the Agda formalization.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 28 -->

92:28

Konstantinos Mamouras and Agnishom Chattopadhyay

Morihata [2012] studies the translation of regular expressions with lookahead into DFAs of
doubly exponential size. A treatment of lookahead using derivatives can be found in [Miyazaki
and Minamide 2019]. A regular expression with lookahead is interpreted as a set of pairs (𝑠, 𝑡)
of strings, where 𝑠 is the matching string and 𝑡 is the remaining string. A lookahead assertion is
interpreted as a set of pairs of the form (𝜀, 𝑡) because it constrains the remaining string without
consuming any string symbols. The finite state automata constructed using this derivative-based
technique has a similar blow-up to the one considered by Morihata [2012]. The authors provide a
lower bound argument showing that lookaround assertions can indeed cause a doubly exponential
blow-up in some cases when converted to a DFA. Note that, while our algorithm runs in linear time,
it is not a streaming algorithm, since it makes both forward and backward passes (in the case of
lookbehind and lookahead assertions, respectively) on the input. Berglund et al. [2021] establish the
semantics of lookarounds using alternating automata that can make forward or backward passes on
the string. This definition is very close to the operational definition used by practitioners. However,
alternating automata are a powerful model, and it is not easy to see how they can be simulated
efficiently. [Trofimovich 2020] suggests an implementation of regular expression matching (tool
RE2C) using automata and tagged transitions and lookahead. The tags are markers which help
extract sub-matches. Moseley et al. [2023] consider a derivative-based approach for matching
regular expressions with anchors, which are a very restricted form of lookaround assertions that
only have a lookahead or lookbehind of at most one symbol.
Bando et al. [2012] consider regular expressions with lookahead and lookbehind in the context
of deep packet inspection in networks. They propose an FPGA-based implementation and estimate
that around 25,000 regexes can be accommodated and a throughput of 34 Gbps can be achieved.
Chida and Terauchi [2022] consider the expressiveness of regular expressions with lookaround
and backreferences. They conclude that adding lookaround enhances the expressiveness of regular
expressions with backreferences. This is in contrast to classical regular expressions (i.e., without
backreferences), where adding lookaround assertions does not increase expressiveness.
8 CONCLUSION AND FUTURE WORK
We have proposed a formal semantics for regular expressions with lookaround. Many commonly
used regex engines that support lookaround resort to backtracking search. Algorithms that are based
on using one automaton for the entire pattern also seem to incur a non-trivial blow-up. Intuitively,
this is because matching lookaround information requires additional contextual information about
the remainder of the string. We have presented an algorithm that matches regexes with lookaround
in time 𝑂 (𝑚 · 𝑛), where 𝑚 is the size of regex and 𝑛 is the length of the input string. This time
complexity is the same as that of Thompson’s algorithm for classical (i.e., lookaround-free) regular
expression. We see from our empirical evaluation that the implementation of our algorithm, which
is augmented with some performance optimizations, has performance that is substantially better
than the state-of-the-art PCRE and Java engines on the real workloads that we have considered.
A worthwhile direction for future work is the extension of our implementation with more advanced operators that are useful in practice. The incorporation of some optimizations for bounded
repetition (see, for example, [Kong et al. 2022] and [Le Glaunec et al. 2023]) in our implementation seems to be feasible. Backreferences pose a challenge because they can give rise to nonregularity, but there are special cases (e.g., backreferences to bounded strings, as in the regex
(?P<q>[a-z]{3})(?P=q) ) that stay within the realm of regularity.
ACKNOWLEDGMENTS
We would like to thank the anonymous reviewers for their constructive comments. This research
was supported in part by the US National Science Foundation award CCF 2008096.
Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 29 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:29

REFERENCES
Alfred V. Aho and Margaret J. Corasick. 1975. Efficient String Matching: An Aid to Bibliographic Search. Commun. ACM 18,
6 (1975), 333–340. https://doi.org/10.1145/360825.360855
Valentin Antimirov. 1996. Partial Derivatives of Regular Expressions and Finite Automaton Constructions. Theoretical
Computer Science 155, 2 (1996), 291–319. https://doi.org/10.1016/0304-3975(95)00182-4
Masanori Bando, N. Sertac Artan, and H. Jonathan Chao. 2012. Scalable Lookahead Regular Expression Detection System
for Deep Packet Inspection. IEEE/ACM Transactions on Networking 20, 3 (2012), 699–714. https://doi.org/10.1109/TNET.
2011.2181411
Ezio Bartocci, Jyotirmoy Deshmukh, Alexandre Donzé, Georgios Fainekos, Oded Maler, Dejan Ničković, and Sriram
Sankaranarayanan. 2018. Specification-Based Monitoring of Cyber-Physical Systems: A Survey on Theory, Tools and
Applications. In Lectures on Runtime Verification: Introductory and Advanced Topics, Ezio Bartocci and Yliès Falcone (Eds.).
LNCS, Vol. 10457. Springer, Cham, 135–175. https://doi.org/10.1007/978-3-319-75632-5_5
Véronique Benzaken, Giuseppe Castagna, and Alain Frisch. 2003. CDuce: An XML-Centric General-Purpose Language. In
Proceedings of the Eighth ACM SIGPLAN International Conference on Functional Programming (ICFP ’03). ACM, New York,
NY, USA, 51–63. https://doi.org/10.1145/944705.944711
Martin Berglund, Frank Drewes, and Brink van der Merwe. 2014. Analyzing Catastrophic Backtracking Behavior in
Practical Regular Expression Matching. In Automata and Formal Languages 2014 (AFL 2014) (Electronic Proceedings
in Theoretical Computer Science (EPTCS), Vol. 151), Zoltán Ésik and Zoltán Fülöp (Eds.). Open Publishing Association,
109–123. https://doi.org/10.4204/eptcs.151.7
Martin Berglund, Brink van der Merwe, and Steyn van Litsenborgh. 2021. Regular Expressions with Lookahead. JUCS Journal of Universal Computer Science 27, 4 (2021), 324–340. https://doi.org/10.3897/jucs.66330
Janusz A. Brzozowski. 1964. Derivatives of Regular Expressions. J. ACM 11, 4 (1964), 481–494. https://doi.org/10.1145/
321239.321249
Janusz A. Brzozowski. 1971. Review of An Algorithm for Constructing a Base in a Language of Regular Expressions, by B.
G. Mirkin. The Journal of Symbolic Logic 36, 4 (1971), 694–694. https://doi.org/10.2307/2272532
Jean-Marc Champarnaud and Djelloul Ziadi. 2001. From Mirkin’s Prebases to Antimirov’s Word Partial Derivatives.
Fundamenta Informaticae 45, 3 (2001), 195–205. https://ip.ios.semcs.net/articles/fundamenta-informaticae/fi45-3-03
Agnishom Chattopadhyay and Konstantinos Mamouras. 2020. A Verified Online Monitor for Metric Temporal Logic with
Quantitative Semantics. In RV 2020 (LNCS, Vol. 12399), Jyotirmoy Deshmukh and Dejan Ničković (Eds.). Springer, Cham,
383–403. https://doi.org/10.1007/978-3-030-60508-7_21
Nariyoshi Chida and Tachio Terauchi. 2022. On Lookaheads in Regular Expressions with Backreferences. In 7th International
Conference on Formal Structures for Computation and Deduction (FSCD 2022) (Leibniz International Proceedings in Informatics
(LIPIcs), Vol. 228), Amy P. Felty (Ed.). Schloss Dagstuhl – Leibniz-Zentrum für Informatik, Dagstuhl, Germany, 15:1–15:18.
https://doi.org/10.4230/LIPIcs.FSCD.2022.15
Thierry Coquand and Vincent Siles. 2011. A Decision Procedure for Regular Expression Equivalence in Type Theory.
In CPP 2011 (LNCS, Vol. 7086), Jean-Pierre Jouannaud and Zhong Shao (Eds.). Springer, Berlin, Heidelberg, 119–134.
https://doi.org/10.1007/978-3-642-25379-9_11
Russ Cox. 2010. Regular Expression Matching in the Wild. https://swtch.com/~rsc/regexp/regexp3.html. [Online; accessed
November 14, 2023].
Christian Doczkal, Jan-Oliver Kaiser, and Gert Smolka. 2013. A Constructive Theory of Regular Languages in Coq. In CPP
2013 (LNCS, Vol. 8307), Georges Gonthier and Michael Norrish (Eds.). Springer, Cham, 82–97. https://doi.org/10.1007/9783-319-03545-1_6
Adel Dokhanchi, Bardh Hoxha, and Georgios Fainekos. 2014. On-Line Monitoring for Temporal Logic Robustness. In RV 2014
(LNCS, Vol. 8734), Borzoo Bonakdarpour and Scott A. Smolka (Eds.). Springer, Cham, 231–246. https://doi.org/10.1007/9783-319-11164-3_19
Derek Egolf, Sam Lasser, and Kathleen Fisher. 2022. Verbatim++: Verified, Optimized, and Semantically Rich Lexing with
Derivatives. In Proceedings of the 11th ACM SIGPLAN International Conference on Certified Programs and Proofs (CPP
2022). ACM, New York, NY, USA, 27–39. https://doi.org/10.1145/3497775.3503694
Denis Firsov and Tarmo Uustalu. 2013. Certified Parsing of Regular Languages. In CPP 2013 (LNCS, Vol. 8307), Georges
Gonthier and Michael Norrish (Eds.). Springer, Cham, 98–113. https://doi.org/10.1007/978-3-319-03545-1_7
Bryan Ford. 2004. Parsing Expression Grammars: A Recognition-Based Syntactic Foundation. In Proceedings of the 31st ACM
SIGPLAN-SIGACT Symposium on Principles of Programming Languages (POPL ’04). ACM, New York, NY, USA, 111–122.
https://doi.org/10.1145/964001.964011
Alain Frisch and Luca Cardelli. 2004. Greedy Regular Expression Matching. In ICALP 2004 (LNCS, Vol. 3142), Josep Díaz, Juhani
Karhumäki, Arto Lepistö, and Donald Sannella (Eds.). Springer, Berlin, Heidelberg, 618–629. https://doi.org/10.1007/9783-540-27836-8_53

Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 30 -->

92:30

Konstantinos Mamouras and Agnishom Chattopadhyay

Bjørn Bugge Grathwohl, Fritz Henglein, Ulrik Terp Rasmussen, Kristoffer Aalund Søholm, and Sebastian Paaske Tørholm.
2016. Kleenex: Compiling Nondeterministic Transducers to Deterministic Streaming Transducers. In Proceedings of the
43rd Annual ACM SIGPLAN-SIGACT Symposium on Principles of Programming Languages (POPL ’16). ACM, New York,
NY, USA, 284–297. https://doi.org/10.1145/2837614.2837647
Niels Bjørn Bugge Grathwohl, Fritz Henglein, Lasse Nielsen, and Ulrik Terp Rasmussen. 2013. Two-Pass Greedy Regular
Expression Parsing. In CIAA 2013 (LNCS, Vol. 7982), Stavros Konstantinidis (Ed.). Springer, Berlin, Heidelberg, 60–71.
https://doi.org/10.1007/978-3-642-39274-0_7
Niels Bjørn Bugge Grathwohl, Fritz Henglein, and Ulrik Terp Rasmussen. 2014a. Optimally Streaming Greedy Regular
Expression Parsing. In Theoretical Aspects of Computing – ICTAC 2014 (LNCS, Vol. 8687), Gabriel Ciobanu and Dominique
Méry (Eds.). Springer, Cham, 224–240. https://doi.org/10.1007/978-3-319-10882-7_14
Niels Bjørn Bugge Grathwohl, Dexter Kozen, and Konstantinos Mamouras. 2014b. KAT + B!. In Proceedings of the Joint Meeting
of the Twenty-Third EACSL Annual Conference on Computer Science Logic (CSL) and the Twenty-Ninth Annual ACM/IEEE
Symposium on Logic in Computer Science (LICS) (CSL-LICS ’14). ACM, New York, NY, USA, Article 44, 44:1–44:10 pages.
https://doi.org/10.1145/2603088.2603095
grep 2023. GREP - Global Regular Expression Print. https://www.gnu.org/software/grep/.
Hyperscan 2023. Intel’s Hyperscan: A high-performance multiple regex matching library. https://github.com/intel/
hyperscan.
Walter L. Johnson, James H. Porter, Stephanie I. Ackley, and Douglas T. Ross. 1968. Automatic Generation of Efficient Lexical
Processors Using Finite State Techniques. Commun. ACM 11, 12 (1968), 805–813. https://doi.org/10.1145/364175.364185
Stephen Cole Kleene. 1956. Representation of Events in Nerve Nets and Finite Automata. In Automata Studies, Claude E.
Shannon and John McCarthy (Eds.). Number 34 in Annals of Mathematics Studies. Princeton University Press, 3–41.
Vladimir Komendantsky. 2012. Reflexive Toolbox for Regular Expression Matching: Verification of Functional Programs in
Coq+Ssreflect. In Proceedings of the Sixth Workshop on Programming Languages Meets Program Verification (PLPV ’12).
ACM, New York, NY, USA, 61–70. https://doi.org/10.1145/2103776.2103784
Lingkun Kong, Qixuan Yu, Agnishom Chattopadhyay, Alexis Le Glaunec, Yi Huang, Konstantinos Mamouras, and Kaiyuan
Yang. 2022. Software-Hardware Codesign for Efficient In-Memory Regular Pattern Matching. In Proceedings of the 43rd
ACM SIGPLAN International Conference on Programming Language Design and Implementation (PLDI 2022). ACM, New
York, NY, USA, 733–748. https://doi.org/10.1145/3519939.3523456
Dexter Kozen. 1994. A Completeness Theorem for Kleene Algebras and the Algebra of Regular Events. Information and
Computation 110, 2 (1994), 366–390. https://doi.org/10.1006/inco.1994.1037
Dexter Kozen. 1997. Kleene Algebra with Tests. ACM Transactions on Programming Languages and Systems 19, 3 (1997),
427–443. https://doi.org/10.1145/256167.256195
Dexter Kozen and Konstantinos Mamouras. 2014. Kleene Algebra with Equations. In ICALP 2014 (LNCS, Vol. 8573),
Javier Esparza, Pierre Fraigniaud, Thore Husfeldt, and Elias Koutsoupias (Eds.). Springer, Berlin, Heidelberg, 280–292.
https://doi.org/10.1007/978-3-662-43951-7_24
Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras. 2023. Regular Expression Matching Using Bit Vector
Automata. Proceedings of the ACM on Programming Languages 7, OOPSLA1, Article 92 (2023), 30 pages. https:
//doi.org/10.1145/3586044
Oded Maler, Dejan Nickovic, and Amir Pnueli. 2008. Checking Temporal Properties of Discrete, Timed and Continuous
Behaviors. In Pillars of Computer Science: Essays Dedicated to Boris (Boaz) Trakhtenbrot on the Occasion of His 85th Birthday,
Arnon Avron, Nachum Dershowitz, and Alexander Rabinovich (Eds.). LNCS, Vol. 4800. Springer, Berlin, Heidelberg,
475–505. https://doi.org/10.1007/978-3-540-78127-1_26
Konstantinos Mamouras. 2015. Extensions of Kleene Algebra for Program Verification. Ph. D. Dissertation. Cornell University,
Ithaca, NY. http://hdl.handle.net/1813/40960
Konstantinos Mamouras. 2017. Equational Theories of Abnormal Termination Based on Kleene Algebra. In FoSSaCS
2017 (LNCS, Vol. 10203), Javier Esparza and Andrzej S. Murawski (Eds.). Springer, Berlin, Heidelberg, 88–105. https:
//doi.org/10.1007/978-3-662-54458-7_6
Konstantinos Mamouras, Agnishom Chattopadhyay, and Zhifu Wang. 2021a. Algebraic Quantitative Semantics for Efficient
Online Temporal Monitoring. In TACAS 2021 (LNCS, Vol. 12651), Jan Friso Groote and Kim Guldstrand Larsen (Eds.).
Springer, Cham, 330–348. https://doi.org/10.1007/978-3-030-72016-2_18
Konstantinos Mamouras, Agnishom Chattopadhyay, and Zhifu Wang. 2021b. A Compositional Framework for Quantitative
Online Monitoring over Continuous-Time Signals. In RV 2021 (LNCS, Vol. 12974), Lu Feng and Dana Fisman (Eds.).
Springer, Cham, 142–163. https://doi.org/10.1007/978-3-030-88494-9_8
Konstantinos Mamouras, Agnishom Chattopadhyay, and Zhifu Wang. 2023. A Compositional Framework for Algebraic
Quantitative Online Monitoring over Continuous-Time Signals. International Journal on Software Tools for Technology
Transfer 25, 4 (2023), 557–573. https://doi.org/10.1007/s10009-023-00719-w

Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.

<!-- page 31 -->

Efficient Matching of Regular Expressions with Lookaround Assertions

92:31

Konstantinos Mamouras and Zhifu Wang. 2020. Online Signal Monitoring with Bounded Lag. IEEE Transactions on ComputerAided Design of Integrated Circuits and Systems 39, 11 (2020), 3868–3880. https://doi.org/10.1109/TCAD.2020.3013053
B. G. Mirkin. 1966. An Algorithm for Constructing a Base in a Language of Regular Expression. Engineering Cybernetics 5
(1966), 110–116.
Takayuki Miyazaki and Yasuhiko Minamide. 2019. Derivatives of Regular Expressions with Lookahead. Journal of Information
Processing 27 (2019), 422–430. https://doi.org/10.2197/ipsjjip.27.422
Takayuki Miyazaki and Yasuhiko Minamide. 2021. Context-Free Grammars with Lookahead. In LATA 2021 (LNCS, Vol. 12638),
Alberto Leporati, Carlos Martín-Vide, Dana Shapira, and Claudio Zandron (Eds.). Springer, Cham, 213–225. https:
//doi.org/10.1007/978-3-030-68195-1_16
Nelma Moreira, David Pereira, and Simão Melo de Sousa. 2012. Deciding Regular Expressions (In-)Equivalence in Coq.
In RAMiCS 2012 (LNCS, Vol. 7560), Wolfram Kahl and Timothy G. Griffin (Eds.). Springer, Berlin, Heidelberg, 98–113.
https://doi.org/10.1007/978-3-642-33314-9_7
Akimasa Morihata. 2012. Translation of Regular Expression with Lookahead into Finite State Automaton. Computer Software
29, 1 (2012), 147–158. https://doi.org/10.11309/jssst.29.1_147
Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu. 2023.
Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics. Proceedings of the ACM on
Programming Languages 7, PLDI, Article 148 (2023), 24 pages. https://doi.org/10.1145/3591262
Lasse Nielsen and Fritz Henglein. 2011. Bit-coded Regular Expression Parsing. In LATA 2011 (LNCS, Vol. 6638), Adrian-Horia
Dediu, Shunsuke Inenaga, and Carlos Martín-Vide (Eds.). Springer, Berlin, Heidelberg, 402–413. https://doi.org/10.1007/
978-3-642-21254-3_32
Michael O. Rabin and Dana Scott. 1959. Finite Automata and their Decision Problems. IBM Journal of Research and
Development 3, 2 (1959), 114–125. https://doi.org/10.1147/rd.32.0114
RE2 2023. RE2: Google’s regular expression library. https://github.com/google/re2.
Indranil Roy and Srinivas Aluru. 2016. Discovering Motifs in Biological Sequences Using the Micron Automata Processor.
IEEE/ACM Transactions on Computational Biology and Bioinformatics 13, 1 (2016), 99–111. https://doi.org/10.1109/TCBB.
2015.2430313
Yuto Sakuma, Yasuhiko Minamide, and Andrei Voronkov. 2012. Translating Regular Expression Matching into Transducers.
Journal of Applied Logic 10, 1 (2012), 32–51. https://doi.org/10.1016/j.jal.2011.11.003 Special issue on Automated
Specification and Verification of Web Systems.
Snort 2023. Snort Intrusion Detection System. https://www.snort.org/.
Suricata 2023. Suricata Threat Detection Engine. https://suricata.io/.
Prasanna Thati and Grigore Roşu. 2005. Monitoring Algorithms for Metric Temporal Logic Specifications. Electronic Notes in
Theoretical Computer Science 113 (2005), 145–162. https://doi.org/10.1016/j.entcs.2004.01.029 Proceedings of the Fourth
Workshop on Runtime Verification (RV 2004).
The PCRE2 Developers. 2023. Perl-compatible Regular Expressions (revised API: PCRE2). https://pcre2project.github.io/
pcre2/doc/html/index.html.
Ken Thompson. 1968. Programming Techniques: Regular Expression Search Algorithm. Commun. ACM 11, 6 (1968), 419–422.
https://doi.org/10.1145/363347.363387
Ulya Trofimovich. 2020. RE2C: A Lexer Generator Based on Lookahead-TDFA. Software Impacts 6 (2020), 100027. https:
//doi.org/10.1016/j.simpa.2020.100027
Margus Veanes. 2015. Symbolic String Transformations with Regular Lookahead and Rollback. In PSI 2014 (LNCS, Vol. 8974),
Andrei Voronkov and Irina Virbitskaite (Eds.). Springer, Berlin, Heidelberg, 335–350. https://doi.org/10.1007/978-3-66246823-4_27
Fang Yu, Zhifeng Chen, Yanlei Diao, T. V. Lakshman, and Randy H. Katz. 2006. Fast and Memory-Efficient Regular Expression
Matching for Deep Packet Inspection. In Proceedings of the 2006 ACM/IEEE Symposium on Architecture for Networking
and Communications Systems (ANCS ’06). ACM, New York, NY, USA, 93–102. https://doi.org/10.1145/1185347.1185360

Received 2023-07-11; accepted 2023-11-07

Proc. ACM Program. Lang., Vol. 8, No. POPL, Article 92. Publication date: January 2024.
