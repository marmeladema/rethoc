---
file: ../pdfs/2023-oopsla-leglaunec-bit-vector-automata.pdf
citation: "Regular Expression Matching using Bit Vector Automata — Le Glaunec, Kong, Mamouras — OOPSLA (PACMPL 7) 2023"
doi: 10.1145/3586044
source: "https://doi.org/10.1145/3586044 (publisher version, added manually)"
topics: [counting_and_derivatives, performance_hardware]
pages: 30
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Regular Expression Matching using Bit Vector Automata
ALEXIS LE GLAUNEC, Rice University, USA
LINGKUN KONG, Rice University, USA
KONSTANTINOS MAMOURAS, Rice University, USA
Regular expressions (regexes) are ubiquitous in modern software. There is a variety of implementation
techniques for regex matching, which can be roughly categorized as (1) relying on backtracking search, or (2)
being based on finite-state automata. The implementations that use backtracking are often chosen due to their
ability to support advanced pattern-matching constructs. Unfortunately, they are known to suffer from severe
performance problems. For some regular expressions, the running time for matching can be exponential in
the size of the input text. In order to provide stronger guarantees of matching efficiency, automata-based
regex matching is the preferred choice. However, even these regex engines may exhibit severe performance
degradation for some patterns. The main reason for this is that regexes used in practice are not exclusively
built from the classical regular constructs, i.e., concatenation, nondeterministic choice and Kleene’s star. They
involve additional constructs that provide succinctness and convenience of expression. The most common
such construct is bounded repetition (also called counting), which describes the repetition of the pattern a
fixed number of times.
In this paper, we propose a new algorithm for the efficient matching of regular expressions that involve
bounded repetition. Our algorithms are based on a new model of automata, which we call nondeterministic
bit vector automata (NBVA). This model is chosen to be expressively equivalent to nondeterministic counter
automata with bounded counters, a very natural model for expressing patterns with bounded repetition. We
show that there is a class of regular expressions with bounded repetition that can be matched in time that
is independent from the repetition bounds. Our algorithms are general enough to cover the vast majority
of challenging bounded repetitions that arise in practice. We provide an implementation of our approach in
a regex engine, which we call BVA-Scan. We compare BVA-Scan against state-of-the-art regex engines on
several real datasets.
CCS Concepts: • Theory of computation → Formal languages and automata theory; • Software and
its engineering;
Additional Key Words and Phrases: regex, automata theory, bounded repetition, counter automata
ACM Reference Format:
Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras. 2023. Regular Expression Matching using Bit
Vector Automata. Proc. ACM Program. Lang. 7, OOPSLA1, Article 92 (April 2023), 30 pages. https://doi.org/10.
1145/3586044

1 INTRODUCTION
Regular patterns, whether they are expressed using regular expressions or finite-state automata,
are ubiquitous in numerous application domains. They are used in text analysis for searching,
extracting, parsing or transforming pieces of text [Aho and Corasick 1975; Awk 2022; Grep 2022].
They are also used in network security [Yu et al. 2006] for describing signatures in network traffic
Authors’ addresses: Alexis Le Glaunec, Department of Computer Science, Rice University, USA, alexis.leglaunec@rice.edu;
Lingkun Kong, Department of Computer Science, Rice University, USA, klk@rice.edu; Konstantinos Mamouras, Department
of Computer Science, Rice University, USA, mamouras@rice.edu.

This work is licensed under a Creative Commons Attribution 4.0 International License.
© 2023 Copyright held by the owner/author(s).
2475-1421/2023/4-ART92
https://doi.org/10.1145/3586044
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

92

<!-- page 2 -->

92:2

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

that indicate an intrusion or other security issues, in bioinformatics [Bo et al. 2018; Roy and Aluru
2016] for specifying DNA, RNA or protein sequences, and in runtime verification [Barringer et al.
2004; Bartocci et al. 2018] for describing safety properties. The usefulness of regular patterns is also
witnessed by the widespread use of regexes in software projects. It is reported in several studies
(see, e.g., [Davis 2019]) that 30%-40% of Java, JavaScript and Python software projects use regex
matching.
There are several approaches for implementing regular pattern matching. The regex matching
engines of many programming languages such as Java, .NET, Python, and JavaScript use backtracking search. For some regexes, these engines need time that is exponential in the size of the
input text. For other regexes, backtracking engines may need time that is polynomial in the length
of the input text, but the polynomial has degree at least 2. This is a lot worse than Thompson’s
algorithm [Thompson 1968], which has time complexity 𝑂 (𝑚 · 𝑛), where 𝑚 is the size of the regular
expression and 𝑛 is the size of the input text. The benefit of backtracking is the simplicity of its
implementation and the ease with which advanced features (e.g., lookaround and backreferences)
can be added.
Many modern regex engines are based on the classical theory of automata. They employ DFAs or
NFAs, or both [Grep 2022; RE2 2023; Wang et al. 2019]. DFA-based algorithms perform a memory
lookup when receiving an input symbol to execute the transition of the automaton. This makes
them fast, because they need 𝑂 (1) time per symbol. One potential problem is that the representation
of the DFA might require a large amount of memory. For many patterns that arise in practice, their
encoding as NFAs can be substantially more compact that their encoding as DFAs. In fact, it is
known that there are families of patterns for which minimal DFAs are exponentially larger than
equivalent NFAs [Meyer and Fischer 1971]. The downside with NFAs is that their execution is less
efficient in terms of time complexity. NFA execution involves maintaining a set of active states,
which represent several possible execution paths, and performing at every step a transition for each
one of the currently active states. So, for every step, this may involve work that is proportional to
the number of states of the NFA.
Software-based (i.e., CPU-based) implementations of regular pattern matching often explore
this trade-off between time- and memory-efficiency that we described in the previous paragraph.
There are also hardware-based implementations which make use of the inherent parallelism that
is available in hardware, where circuit elements compute independently and in parallel. There
are several proposals that use field-programmable gate arrays (FPGAs) [Bispo et al. 2006] and
others that employ digital application-specific integrated circuit (ASIC) accelerators [Brodie et al.
2006]. The latest hardware technology is the in-memory architecture, which is based on NFAs
and processes the NFA transitions directly inside memories. This offers massive parallelism by
combining memory and computation at the same physical location on the circuit. The Automata
Processor (AP) from Micron [Dlugosch et al. 2014] is the first example of such an architecture.
Hardware offers the memory-efficiency of NFAs without a penalty in execution time, since NFA
transitions from active states are explored in parallel in one step. In recent work, [Kong et al. 2022]
propose an architecture based on nondeterministic counter automata (NCAs) instead of NFAs.
While an explicit NFA representation can be much more compact than an explicit DFA representation, there are patterns for which even an NFA is too large. Such patterns are very common
in practice, because regular patterns are typically specified using an extended regex syntax that
makes them highly succinct. Classical regular expressions include constructs for concatenation,
nondeterministic choice and Kleene’s star. A regular expression with these constructs can be
translated into an NFA whose size is linear in the size of the expression [Antimirov 1996; Glushkov
1961; Thompson 1968]. In practice, regular expressions also include the constructs of (1) bounded
repetition (also called counting), which is written as 𝑟 {𝑚, 𝑛}, (2) and lookaround, which includes
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 3 -->

Regular Expression Matching using Bit Vector Automata

92:3

positive/negative lookahead, and positive/negative lookbehind. Even though lookaround is a regular
construct, it is not supported by any of the available automata-based regex engines (e.g., grep,
RE2, Hyperscan). For this reason, this paper focuses on bounded repetition. Bounded repetition is
extremely common in practical use cases of regular expressions. For example, the majority of the
regexes that are found in datasets of intrusion detection rules [Snort 2023; Suricata 2023] include
bounded repetition. Regexes similar to .*a.{𝑛} arise in these datasets. It is known that the smallest
NFAs that recognize these regexes have Θ(𝑛) states, and the minimal DFAs that recognize them
have Θ(2𝑛 ) states [Meyer and Fischer 1971]. The regex .*a.{𝑛} has size Θ(log 𝑛) because the
repetition bound 𝑛 is represented in binary/decimal notation. So, any NFA that recognizes this
pattern is of size exponential in the regex size.
The exponential succinctness of bounded repetition can have a large impact on the performance
of software regex engines (grep, RE2) that generate and cache the pattern DFA on the fly. When the
DFA becomes too large, these engines start flushing the DFA cache and eventually revert to a much
slower NFA simulation. While this problem is well understood, there are no known ways to mitigate
it in the general case. The recent paper [Turoňová et al. 2020] explores an efficient algorithm for
matching regexes with bounded repetition, but only addresses patterns of a limited form. For
example, the patterns .*(aa){100} and .*a{0,100}a{200} are not handled by the algorithm of
[Turoňová et al. 2020]. More information about [Turoňová et al. 2020] can be found in Section 6.
The present paper improves upon this situation by substantially expanding the class of regular expressions with bounded repetition that can be matched efficiently. In particular, our main
contributions are the following:
(1) We propose a new efficient algorithm for matching every bounded repetition of the form
(𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛},
where each 𝜎𝑖 is a predicate (i.e., a character class). This algorithm applies to bounded
repetitions (of this particular form) that can occur as subexpressions of a larger regular
expression. The time complexity of our algorithm is 𝑂 (𝑘) per step. Our algorithm is based
on a model of automata which we call nondeterministic bit vector automata (NBVAs). These
automata have a precise correspondence to the classical model of nondeterministic counter
automata (NCAs) with bounded counters.
(2) We extend the previous result to the larger class of bounded repetitions of the form 𝑟 {𝑚, 𝑛},
where 𝑟 is a regex without bounded repetitions that recognizes strings of the same length.
The time complexity of our algorithm is 𝑂 (ℓ) per step, where ℓ is the number of predicate
(i.e., character class) occurrences in 𝑟 .
(3) We provide an implementation of a general regex engine, which we call BVA-Scan, that
supports bounded repetition and integrates our efficient algorithms. We compare our regex
engine against state-of-the-art regex engines (RE2 [RE2 2023], PCRE [Hazel and Herczeg 2022],
and CA [Library 2021]) using the real application benchmarks Snort [Roesch 1999; Snort 2023],
Suricata [Suricata 2023], Protomata [Roy and Aluru 2016; Sigrist et al. 2009], ClamAV [ClamAV
2023], SpamAssassin [SpamAssassin 2022], and RegexLib [RegexLib 2023]. Our results show
that BVA-Scan consistently outperforms all three regex engines we are comparing against.
The outperformance of our engine is substantial for cases of computationally challenging
bounded repetition.
Outline of paper: In Section 2, we provide the definitions of regular expressions with bounded
repetition and nondeterministic counter automata (NCAs) that we will use throughout the paper.
Section 3 introduces the model of nondeterministic bit vector automata (NBVAs) as an alternative
to NCAs that is more convenient for specifying regex matching algorithms. In Section 4, we present
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 4 -->

92:4

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

our algorithms for the efficient matching of a form of bounded repetition that arises frequently in
practice. Section 5 presents our experimental results against three state-of-the-art regex engines
over several real benchmarks. Section 6 contains a discussion of related work. Finally, we conclude
in Section 7 with a brief summary of our contributions.
2 PRELIMINARIES
In this section, we present a variant of nondeterministic counter automata (NCAs) that is appropriate
for implementing regular expressions with bounded repetition. We are not interested in NCAs
with unbounded counters, which can recognize non-regular languages. Instead, we focus on NCAs
with bounded counters, which are finite-state machines and can therefore only recognize regular
languages. Most definitions of NCAs in prior works consider the same set of counter registers
for each control state of the automaton. We relax this restriction by allowing each state to have a
different number of counters. This flexibility is useful for reducing the amount of memory needed
for NCA execution.
Let Σ be a finite alphabet. We define the set Regex of regular expressions (or regexes) over Σ to
be the smallest set that satisfies the following properties:
(1) Regex contains the expression 𝜀 (regex that recognizes the empty string).
(2) Regex contains every predicate 𝜎 over the alphabet (i.e., 𝜎 ⊆ Σ).
(3) For every 𝑟, 𝑟 1, 𝑟 2 ∈ Regex, we have that 𝑟 1 · 𝑟 2 ∈ Regex (concatenation), 𝑟 1 |𝑟 2 ∈ Regex
(nondeterministic choice) and 𝑟 ∗ ∈ Regex (Kleene’s star).
(4) For all integers 𝑚, 𝑛 with 0 ≤ 𝑚 ≤ 𝑛 and every 𝑟 ∈ Regex, we have that 𝑟 {𝑚, 𝑛} ∈ Regex.
The concatenation symbol is sometimes omitted, i.e., we write 𝑟 1𝑟 2 instead of 𝑟 1 · 𝑟 2 . An expression
of the form 𝑟 {𝑚, 𝑛} is called a bounded repetition, because it describes the repetition of 𝑟 from 𝑚
to 𝑛 times. We write 𝑟 {𝑛} as abbreviation for 𝑟 {𝑛, 𝑛}. The expression 𝑟 {𝑛, } = 𝑟 {𝑛}𝑟 ∗ describes
the repetition of 𝑟 at least 𝑛 times. We say that a regular expression is counting-free if it does not
contain any occurrence 𝑟 {𝑚, 𝑛} of bounded repetition. The interpretation of a regex 𝑟 is a language
L (𝑟 ) ⊆ Σ∗ , which is defined in the standard way.
Notation for character classes: A predicate over the alphabet is also called a character class. The
predicate Σ contains all symbols in the alphabet. When we use a symbol 𝑎 ∈ Σ in a regex, it should
be understood as the singleton predicate {𝑎} ⊆ Σ. The notation [𝑎 1 . . . 𝑎𝑛 ] is used for representing
the predicate {𝑎 1, . . . , 𝑎𝑛 } ⊆ Σ. We write [ˆ𝑎 1 . . . 𝑎𝑛 ] for the predicate Σ \ {𝑎 1, . . . , 𝑎𝑛 } that contains
all symbols except for 𝑎 1, . . . , 𝑎𝑛 .
In order to define counter automata, we first fix an infinite set CReg of counter registers or,
simply, counters. We typically write 𝑥, 𝑦, 𝑧, . . . to denote counter registers. For a subset 𝑉 ⊆ CReg
of counters, a function 𝛽 : 𝑉 → N, which assigns a value (more specifically, a natural number) to
each counter in 𝑉 , is called a 𝑉 -valuation.
Definition 2.1 (Counter Automata). Let Σ be a finite alphabet. A nondeterministic counter
automaton (NCA) with input alphabet Σ is a tuple A = (𝑄, 𝑅, Δ, 𝐼, 𝐹 ), where
− 𝑄 is a finite set of (control) states,
− 𝑅 : 𝑄 → P (CReg) is a function that maps each state to a finite set of counters,
− Δ is the transition relation, which contains finitely many transitions of the form (𝑝, 𝜎, 𝜑, 𝑞, 𝜗),
where 𝑝 is the source state, 𝜎 ⊆ Σ is a predicate over the alphabet, 𝜑 ⊆ (𝑅(𝑝) → N) is a predicate
over 𝑅(𝑝)-valuations, 𝑞 is the destination state, and 𝜗 : (𝑅(𝑝) → N) → (𝑅(𝑞) → N),
− 𝐼 is the initialization function, which specifies the set of initial valuations 𝐼 (𝑞) ⊆ 𝑅(𝑞) → N for
each state 𝑞, and
− 𝐹 is the finalization function, which specifies a set of final valuations 𝐹 (𝑞) ⊆ 𝑅(𝑞) → N for each
state 𝑞.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 5 -->

Regular Expression Matching using Bit Vector Automata

𝜎1

𝑞1

𝑞2

𝜎2 / 𝑥 ≔ 1

𝑞3 : 𝑥

𝜎3 / 𝑦 ≔ 1

Σ

92:5

𝜎4
𝜎5, 𝑚 ≤ 𝑦 ≤ 𝑛
𝜎 ,𝑥 = 𝑘
𝑞 4 : 𝑥, 𝑦
𝑞 5 : 𝑥, 𝑦
𝑞6 : 𝑥 6
𝑞7
𝜎3, 𝑦 < 𝑛 / 𝑦++
𝜎2, 𝑥 < 𝑘 / 𝑥++

Fig. 1. NCA with two counters (𝑥 and 𝑦) for the regex Σ∗ 𝜎1 (𝜎2 (𝜎3 𝜎4 ){𝑚, 𝑛}𝜎5 ){𝑘 }𝜎6 with 1 ≤ 𝑚 ≤ 𝑛 and
𝑘 ≥ 1. The states 𝑞 1, 𝑞 2, 𝑞 7 have no counter (i.e., they are pure). The states 𝑞 3, 𝑞 6 have 1 counter (𝑥) and the
states 𝑞 4, 𝑞 5 have 2 counters (𝑥 and 𝑦).

We say that a state 𝑞 ∈ 𝑄 is pure if 𝑅(𝑞) = ∅, that is, it has no counter associated with it. If 𝑅(𝑞) ≠ ∅,
then we say that 𝑞 is a counting state. A state 𝑞 ∈ 𝑄 is called final if 𝐹 (𝑞) ≠ ∅. Similarly, a state
𝑞 ∈ 𝑄 is said to be initial if 𝐼 (𝑞) ≠ ∅.
Our definition for NCAs is similar to the one given in [Kong et al. 2022]. One difference is that
we consider here a slightly more general initialization function that allows several different initial
valuations for an initial state.
According to Definition 2.1, the states in an NCA do not all necessarily have the same number of
counters. In fact, some states may not have any counter at all. In a transition (𝑝, 𝜎, 𝜑, 𝑞, 𝜗), we will
call the predicate 𝜑 a guard because it may restrict a transition based on the values of the counters.
The function 𝜗 of a transition is called an action, because it describes how to assign counter values
in the destination state given the counter values in the source state.
We use a variant of the Glushkov construction [Gelade et al. 2009; Glushkov 1961; Hovland 2009]
to convert a regular expression 𝑟 (with bounded repetition) to an NCA that recognizes the language
L (𝑟 ). In contrast to Thompson’s construction [Thompson 1968], Glushkov’s construction results
in 𝜀-free automata that are also homogeneous, i.e., all incoming transitions of a state are labeled
with the same predicate over the alphabet.
Example 2.2. Consider the regular expression 𝑟 = Σ∗𝜎 𝜎 𝜎 {𝑛} with 𝑛 ≥ 1, where 𝜎 , 𝜎 , 𝜎 are
1

1 2 3

1

predicates over the alphabet Σ1 . The following automaton recognizes the language of 𝑟 1 :
𝜎1

𝑞1

𝑞2

𝜎2

𝑞3

𝜎3 / 𝑥 ≔ 1

Σ

𝑞4 : 𝑥

2

3

𝑥 =𝑛

𝜎3, 𝑥 < 𝑛 / 𝑥++

The automaton shown above has four states: 𝑞 1 , 𝑞 2 , 𝑞 3 , and 𝑞 4 . We write 𝑞 4 : 𝑥 to indicate that
𝑅(𝑞 4 ) = {𝑥 }. That is, 𝑞 4 has one counter register called 𝑥. Notice that 𝑞 1 has no annotation with
counters, which means that 𝑅(𝑞 1 ) = ∅ (i.e., 𝑞 1 is pure). Each edge 𝑝 → 𝑞 is annotated with an
expression of the form 𝜎, 𝜑 / 𝜗, where
(1) 𝜎 is a predicate over Σ (i.e., a character class),
(2) 𝜑 is a guard over the counters of 𝑝, and
(3) 𝜗 is an assignment that specifies values for the counters of 𝑞 using the values for counters of 𝑝.
Here are some notational conventions that we follow when drawing diagrams of NCAs:
− When we omit the guard 𝜑, then this indicates that it is always true. More formally, if an edge
emanating from a state 𝑝 is not annotated with a guard 𝜑, then it should be understood that
𝜑 = (𝑅(𝑝) → N).
− For an edge from 𝑝 to 𝑞, we sometimes omit the action 𝜗. We only do this when 𝑅(𝑞) ⊆ 𝑅(𝑝),
and the omission of the action 𝜗 indicates that the counters 𝑅(𝑞) retain the values from the
1 In order to make the regular expression more concrete, suppose that 𝜎

1 = [𝑎𝑏 ], 𝜎2 = [𝑏𝑐𝑑 ], and 𝜎2 = [ˆ𝑎]. So, the regular
expression 𝑟 1 is the same as .∗ [ab] [bcd] [ˆa] {n} using POSIX notation [in PCRE 2022]. The expression Σ∗ has the same
meaning as .∗ in POSIX notation.

Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 6 -->

92:6

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

previous state. Alternatively, we can explicitly indicate this action that does not change counter
values by writing one or more assignments of the form “𝑥 ≔ 𝑥”.
− We write “𝑥 = 𝑛” (resp., “𝑥 < 𝑛”) for the guard that checks whether the value of counter 𝑥 is
equal to 𝑛 (resp., strictly less than 𝑛).
− We write “𝑥 ≔ 𝑐” to denote the assignment (action) of the value 𝑐 to the counter 𝑥.
− The action “𝑥++” indicates the incrementation of the counter 𝑥 by 1. So, it can be understood as
an abbreviation for the assignment “𝑥 ≔ 𝑥 + 1”.
− We use double circle notation to indicate that a state is final (see state 𝑞 4 in the diagram shown
earlier). An arrow emanating from a final state 𝑞 is annotated with the predicate 𝐹 (𝑞) over
counter valuations (recall that 𝐹 is the finalization function).
− If 𝐹 (𝑞) = (𝑅(𝑞) → N), then we may omit the finalization function from the diagram.
The examples that follow (for regexes 𝑟 2, 𝑟 3, 𝑟 4 ) are adapted from [Kong et al. 2022]. The following
automaton recognizes the language of the regex 𝑟 2 = Σ∗𝜎1 (𝜎2𝜎3 ){𝑚, 𝑛}𝜎4 , where 𝜎1 , 𝜎2 , 𝜎3 , 𝜎4 are
character classes and 1 ≤ 𝑚 ≤ 𝑛.
𝜎1

𝑞1

𝑞2

𝜎2 / 𝑥 ≔ 1

𝜎2, 𝑥 < 𝑛 / 𝑥++
𝑞3 : 𝑥

𝑞4 : 𝑥

𝜎4, 𝑚 ≤ 𝑥 ≤ 𝑛

𝑞5

𝜎3

Σ

Observe in the diagram above the omission of both the guard and the action from the edge 𝑞 3 → 𝑞 4 .
As mentioned earlier, this indicates that the guard is always true and the action is the assignment
“𝑥 ≔ 𝑥” that copies the counter value. The language of the regex 𝑟 3 = 𝜎1 {𝑚}Σ∗𝜎2 {𝑛}, where
𝑚, 𝑛 ≥ 1 and 𝜎1, 𝜎2 are character classes, is recognized by the following automaton:
𝑞1

𝜎1 / 𝑥 ≔ 1

𝑞2 : 𝑥

Σ, 𝑥 = 𝑚

𝑞3

𝜎2 / 𝑥 ≔ 1

𝑞4 : 𝑥

𝑥 =𝑛

Σ
𝜎1, 𝑥 < 𝑚 / 𝑥++

𝜎2, 𝑥 = 𝑚 / 𝑥 ≔ 1

𝜎2, 𝑥 < 𝑛 / 𝑥++

All automata that we have seen so far use at most one counter for each control state. For the regex
𝑟 4 = Σ∗𝜎1 (𝜎2 (𝜎3𝜎4 ){𝑚, 𝑛}𝜎5 ){𝑘 }𝜎6 with 1 ≤ 𝑚 ≤ 𝑛 and 𝑘 ≥ 1 we need two counters for some
states because of the nesting of bounded repetition. See Fig. 1.
Nondeterministic semantics. We will now consider a nondeterministic path semantics for
NCAs, which is analogous to the path semantics for NFAs. Let A = (𝑄, 𝑅, Δ, 𝐼, 𝐹 ) be an NCA.
A token for A is a pair (𝑞, 𝛽), where 𝑞 is a state and 𝛽 : 𝑅(𝑞) → N is a counter valuation for 𝑞.
The set of all tokens for A is denoted by Tk(A). That is,
Tk(A) = {(𝑞, 𝛽) | 𝑞 ∈ 𝑄 and 𝛽 : 𝑅(𝑞) → N}.
Simplified notation for tokens: In many cases, we only consider states that have at most one counter
register. We simplify the notation for tokens as described below:
− For a pure state 𝑞 (i.e., a state with no counter, see Definition 2.1), there is only one possible
valuation, denoted 0N : ∅ → N, which carries no information. That is, 𝑅(𝑞) = ∅ and therefore
the set of valuations 𝑅(𝑞) → N is equal to {0N }. So, we will often abuse notation and simply
write 𝑞 for the token (𝑞, 0N ).
− Let us consider now a state 𝑞 with one counter, i.e., 𝑅(𝑞) = {𝑥 } for some 𝑥 ∈ CReg. In this case, a
valuation 𝛽 (of type {𝑥 } → N) for 𝑞 specifies only one value 𝑐 = 𝛽 (𝑥) ∈ N for the unique counter
𝑥 for 𝑞. For this reason, we will sometimes write (𝑞, 𝑐) instead of (𝑞, 𝛽).
For a letter 𝑎 ∈ Σ, we define the token transition relation →𝑎 on Tk(A) as follows: (𝑝, 𝛽) →𝑎 (𝑞, 𝛾)
iff there is a transition (𝑝, 𝜎, 𝜑, 𝑞, 𝜗) ∈ Δ with 𝑎 ∈ 𝜎 such that 𝛽 ∈ 𝜑 and 𝛾 = 𝜗 (𝛽). A token (𝑞, 𝛽) is
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 7 -->

Regular Expression Matching using Bit Vector Automata

92:7

initial if 𝛽 ∈ 𝐼 (𝑞) (which, in particular, means that 𝑞 is initial). A token (𝑞, 𝛽) is final if 𝛽 ∈ 𝐹 (𝑞)
(which also means that 𝑞 is final). A run of A on an input string 𝑎 1𝑎 2 . . . 𝑎𝑛 ∈ Σ∗ is a sequence
𝑎1

𝑎2

𝑎3

𝑎𝑛

(𝑞 0, 𝛽 0 ) −→ (𝑞 1, 𝛽 1 ) −→ (𝑞 2, 𝛽 2 ) −→ · · · −→ (𝑞𝑛 , 𝛽𝑛 ),
where each (𝑞𝑖 , 𝛽𝑖 ) is a token, (𝑞 0, 𝛽 0 ) is an initial token, and (𝑞𝑖−1, 𝛽𝑖−1 ) →𝑎 (𝑞𝑖 , 𝛽𝑖 ) for every
𝑖 = 1, . . . , 𝑛. A run is accepting if it ends with a final token. The NCA A accepts a string if there is
an accepting run on it. We denote by L (A) ⊆ Σ∗ the set of all strings that are accepted by A.
For an NCA A, the set of tokens Tk(A) together with the transition relations →𝑎 forms a
labeled transition system. The family of transition relations (→𝑎 )𝑎 ∈Σ can be represented as a
ternary relation → ⊆ Tk(A) × Σ × Tk(A).
Deterministic semantics using configurations. Let A = (𝑄, 𝑅, Δ, 𝐼, 𝐹 ) be an NCA. A configuration 𝑆 for A is a set of tokens for A, that is, 𝑆 ⊆ Tk(A). We write C(A) for the set of all
configurations for A. The configuration transition function 𝛿 : C(A) × Σ → C(A) is defined by
𝛿 (𝑆, 𝑎) = {(𝑞, 𝛾) | (𝑝, 𝛽) →𝑎 (𝑞, 𝛾) for some (𝑝, 𝛽) ∈ 𝑆 }.
The transition function is extended to 𝛿 : C(A) × Σ∗ → C(A) as follows: 𝛿 (𝑆, 𝜀) = 𝑆 and
𝛿 (𝑆, 𝑥𝑎) = 𝛿 (𝛿 (𝑆, 𝑥), 𝑎) for every 𝑆 ⊆ Tk(A), 𝑥 ∈ Σ∗ and 𝑎 ∈ Σ. The initial configuration 𝑆 0 is the
set of all initial tokens. Define the function ⟦A⟧ : Σ∗ → C(A) by ⟦A⟧(𝑥) = 𝛿 (𝑆 , 𝑥). Intuitively,
0

⟦A⟧(𝑥) is the configuration of A after consuming the string 𝑥. This semantics coincides with
L (A) in the following sense: for every 𝑥 ∈ Σ∗ , 𝑥 ∈ L (A) iff ⟦A⟧(𝑥) contains some final token.
Bounded counters: NCAs with unbounded counters can recognize non-regular languages. For
this reason, we will restrict them so that there is a fixed bound on all counter values.
Let A be an NCA and 𝑛 ∈ N. A token (𝑞, 𝛽) is said to be 𝑛-bounded if 𝛽 (𝑥) ≤ 𝑛 for every counter
𝑥 ∈ 𝑅(𝑞). We also say that A (resp., a state 𝑞) is 𝑛-bounded if every token (resp., token on state 𝑞)
reachable from some initial token is 𝑛-bounded. Finally, the NCA A is said to have bounded counters
if there exists some 𝑛 ∈ N such that A is 𝑛-bounded. NCAs with bounded counters have the
same expressiveness as finite-state automata (i.e., DFAs and NFAs), but they may be more succinct.
See [Meyer and Stockmeyer 1972] and [Stockmeyer and Meyer 1973] for some implications of
succinctness due to counting.
The automata that we consider are constructed from regexes (with bounded repetition) using
the Glushkov construction. Every action of the form 𝑥++ (counter incrementation) is guarded by a
test 𝑥 < 𝑛 because it corresponds to a subexpression of the form 𝑟 {𝑚, 𝑛}. So, every automaton that
is constructed from regexes has bounded counters. Moreover, for every control state and every
counter, we can easily obtain an upper bound for the counter values by inspecting the automaton.
For example, let us consider the NCA of Fig. 1. Counter 𝑥 at states 𝑞 3, 𝑞 4, 𝑞 5, 𝑞 6 is bounded above by
𝑘 because (𝑞 6, 𝜎2, “𝑥 < 𝑘”, 𝑞 3, “𝑥++”) is the only transition that increments 𝑥. The transition guard
will not allow the value of the counter to exceed 𝑘. Similarly, counter 𝑦 at states 𝑞 4, 𝑞 5 is bounded
above by 𝑛 because (𝑞 5, 𝜎3, “𝑦 < 𝑛”, 𝑞 4, “𝑦++”) is the only transition that increments 𝑦.
If we restrict counter values according to the bounds described earlier, then we have a finite
number of counter valuations that can be associated with each control state. Under this restriction,
the set Tk(A) of tokens is finite and it can be viewed as the state space of an NFA that recognizes
the same language as the NCA A.
3 BIT VECTOR AUTOMATA
In this section, we will introduce the model of nondeterministic bit vector automata (NBVAs).
These automata are expressively equivalent to the NCAs of Section 2, assuming that the NCA
counters are bounded. NBVAs are more convenient, however, for formulating pattern matching
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 8 -->

92:8

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

algorithms. Informally, if A is an NCA, then the corresponding NBVA A ′ differs from A in how
its configuration is represented. The configuration of the NBVA A ′ specifies for each control state
𝑞 a data structure (more specifically, a bit vector) to represent the set of active tokens (for the NCA
A) of the form (𝑞, 𝛽), i.e., tokens that are on the control state 𝑞.
Let A = (𝑄, 𝑅, Δ, 𝐼, 𝐹 ) be an NCA. For a state 𝑞 ∈ 𝑄 and a subset 𝑇 ⊆ Tk(A) of tokens for the
automaton, define 𝑇 |𝑞 = 𝑇 ∩ ({𝑞} × (𝑅(𝑞) → N)). That is, 𝑇 |𝑞 contains those tokens of 𝑇 whose
first component is the state 𝑞. The operational intuition is that ⟦A⟧(𝑥)|𝑞 is the set of tokens that
we get on state 𝑞 when we execute the NCA A on input 𝑥.
Definition 3.1 (Nondeterministic Bit Vector Automaton). Let Σ be a finite alphabet. A nondeterministic bit vector automaton (NBVA) is a tuple (𝑄, 𝑤, Δ, 𝐼, 𝐹 ), where
− 𝑄 is a finite set of (control) states,
− 𝑤 : 𝑄 → {1, 2, . . .} is a function that maps each state to a strictly positive integer,
− Δ is the transition relation, which contains finitely many transitions of the form (𝑝, 𝜎, 𝑞, 𝜗), where
𝑝 is the source state, 𝜎 ⊆ Σ is a predicate over the alphabet, 𝑞 is the destination state, and
𝜗 : B𝑤 (𝑝) → B𝑤 (𝑞) ,
− 𝐼 is the initialization function, which specifies an initial vector 𝐼 (𝑞) : B𝑤 (𝑞) for each state 𝑞, and
− 𝐹 is the finalization function, which specifies a function 𝐹 (𝑞) : B𝑤 (𝑞) → B for each state 𝑞.
A state 𝑞 is initial if 𝐼 (𝑞) ≠ 0𝑤 (𝑞) , where 0𝑤 (𝑞) is the zero vector of length 𝑤 (𝑞). A state 𝑞 is final if
𝐹 (𝑞)(𝑣) = 1 for some 𝑣 ∈ B𝑤 (𝑞) . A state 𝑞 ∈ 𝑄 is pure (resp., counting) if 𝑤 (𝑞) = 1 (resp., 𝑤 (𝑞) > 1).
In contrast to the NCAs of Definition 2.1, the NBVAs of Definition 3.1 do not have transition
guards (i.e., predicates over the bit vectors). We have chosen this presentation because guards are
redundant. If we had a separate guard 𝜑 ⊆ B𝑤 (𝑝) and an action 𝜗 : B𝑤 (𝑝) → B𝑤 (𝑞) , then we could
integrate both of them into the action 𝜗𝜑 : B𝑤 (𝑝) → B𝑤 (𝑞) , given by
𝜗𝜑 (𝑣) = 𝜗 (𝑣), if 𝑣 ∈ 𝜑

𝜗𝜑 (𝑣) = 0, if 𝑣 ∉ 𝜑

for every bit vector 𝑣 ∈ B𝑤 (𝑝) .
Semantics of NBVAs. Let A = (𝑄, 𝑤, Δ, 𝐼, 𝐹 ) be an NBVA. A configuration for A is a mapping 𝐶,
which specifies a bit vector 𝐶 (𝑞) ∈ B𝑤 (𝑞) for every state 𝑞 ∈ 𝑄. Intuitively, a configuration specifies
the bit vector that each control state carries. We write C(A) to denote the set of all configurations
of A. We will define now the function ⟦A⟧ : Σ∗ → C(A), where ⟦A⟧(𝑥) is the configuration
of the automaton after consuming an input string 𝑥 ∈ Σ∗ . For the base case, we define the initial
configuration ⟦A⟧(𝜀) as follows: ⟦A⟧(𝜀)(𝑞) = 𝐼 (𝑞) for every 𝑞 ∈ 𝑄. For the step case, we define
o
Õn
⟦A⟧(𝑥𝑎)(𝑞) =
𝜗 (⟦A⟧(𝑥)(𝑝)) | (𝑝, 𝜎, 𝑞, 𝜗) ∈ Δ s.t. 𝑎 ∈ 𝜎
Í
for all 𝑥 ∈ Σ∗ , 𝑎 ∈ Σ and 𝑞 ∈ 𝑄. The symbol above is bitwise OR (disjunction). The NBVA A
accepts a string 𝑥 ∈ Σ∗ if there exists a state 𝑞 ∈ 𝑄 such that 𝐹 (𝑞)(𝑣) = 1, where 𝑣 = ⟦A⟧(𝑥)(𝑞).
Example 3.2. Consider the regex 𝑟 = Σ∗𝜎1𝜎2 {𝑛} with 𝑛 ≥ 1, where 𝜎1, 𝜎2 are predicates over the
alphabet. The following automata recognize the language of 𝑟 :
NCA for 𝑟 :

𝜎1

𝑞1

𝑞2

𝜎2 / 𝑥 ≔ 1

𝑞3 : 𝑥

𝜎2, 𝑥 < 𝑛 / 𝑥++

Σ

NBVA for 𝑟 :

𝜎1

𝑞1

𝑥 =𝑛

𝑞2

𝜎2 / 𝑣 · 0[1 ↦→ 1]

𝑞3 : 𝑛

Σ

Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

𝑣 [𝑛] = 1

𝜎2 / shft(𝑣)

<!-- page 9 -->

Regular Expression Matching using Bit Vector Automata

NCA :

𝑎

𝑞1

𝑞2

92:9
Σ/𝑥 ≔ 1

𝑞3 : 𝑥

Σ, 𝑥 < 7 / 𝑥++

Σ

NBVA :

𝑎

𝑞1

𝑞2

Σ / 𝑣 · 0[1 ↦→ 1]

𝑏
𝑎
𝑏
𝑎
𝑎
𝑏
𝑎
𝑎
𝑎
𝑎
𝑏
𝑏
𝑎
𝑏

𝑞3 : 7

𝑣 [7] = 1

Σ / shft(𝑣)

Σ

input

𝑥 =7

NCA configuration
𝑞1
𝑞1
𝑞1, 𝑞2
𝑞 1 , (𝑞 3 , 1)
𝑞 1 , 𝑞 2 , (𝑞 3 , 2)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 3 , 3)
𝑞 1 , (𝑞 3 , 1), (𝑞 3 , 2), (𝑞 3 , 4)
𝑞 1 , 𝑞 2 , (𝑞 3 , 2), (𝑞 3 , 3), (𝑞 3 , 5)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 3 , 3), (𝑞 3 , 4), (𝑞 3 , 6)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 3 , 2), (𝑞 3 , 4), (𝑞 3 , 5), (𝑞 3 , 7)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 3 , 2), (𝑞 3 , 3), (𝑞 3 , 5), (𝑞 3 , 6)
𝑞 1 , (𝑞 3 , 1), (𝑞 3 , 2), (𝑞 3 , 3), (𝑞 3 , 4), (𝑞 3 , 6), (𝑞 3 , 7)
𝑞 1 , (𝑞 3 , 2), (𝑞 3 , 3), (𝑞 3 , 4), (𝑞 3 , 5), (𝑞 3 , 7)
𝑞 1 , 𝑞 2 , (𝑞 3 , 3), (𝑞 3 , 4), (𝑞 3 , 5), (𝑞 3 , 6)
𝑞 1 , (𝑞 3 , 1), (𝑞 3 , 4), (𝑞 3 , 5), (𝑞 3 , 6), (𝑞 3 , 7)

𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,

NBVA configuration
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 0, 0, 0, 0, 0⟩
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 0, 0, 0, 0, 0⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 0, 0, 0, 0, 0, 0⟩
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 0, 0, 0, 0, 0, 0⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 1, 0, 0, 0, 0, 0⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 0, 1, 0, 0, 0, 0⟩
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 1, 0, 1, 0, 0, 0⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 1, 1, 0, 1, 0, 0⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 0, 1, 1, 0, 1, 0⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 1, 0, 1, 1, 0, 1⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 1, 1, 0, 1, 1, 0⟩
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 1, 1, 1, 0, 1, 1⟩
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 1, 1, 1, 1, 0, 1⟩
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 0, 1, 1, 1, 1, 0⟩
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 0, 0, 1, 1, 1, 1⟩

output
0
0
0
0
0
0
0
0
0
1
0
1
1
0
1

Fig. 2. Execution of the NCA and the NBVA for the regular expression .*a.{7}.

Notation: We write 0[1 ↦→ 1] to denote the bit vector that is zero everywhere, except for position 1,
where it is equal to 1. This is an instance of the more general notation 𝑣 [𝑖 ↦→ 𝑏], where 𝑣 is a bit
vector, 𝑖 is a position, and 𝑏 is a Boolean value. The bit vector 𝑣 [𝑖 ↦→ 𝑏] results from 𝑣 by modifying
it (potentially) at position 𝑖 so that it has the value 𝑏. If 𝑣 is a Boolean value and 𝑤 is a bit vector of
size 𝑛, then 𝑣 · 𝑤 is a bit vector of size 𝑛 given by 0 · 𝑤 = 0 and 1 · 𝑤 = 𝑤.
The NBVA shown earlier (for the regex 𝑟 = Σ∗𝜎1𝜎2 {𝑛}) has three states: 𝑞 1 , 𝑞 2 , and 𝑞 3 . We
write “𝑞 3 : 𝑛” to indicate that 𝑤 (𝑞 3 ) = 𝑛, i.e., 𝑞 3 carries a bit vector of size 𝑛. Notice that 𝑞 1 has no
annotation, which means that 𝑤 (𝑞 1 ) = 1 (i.e., 𝑞 1 is pure). We annotate each edge 𝑝 → 𝑞 with an
expression of the form 𝜎 / 𝜗, where 𝜎 is a predicate over Σ, and 𝜗 is a function for computing the
bit vector of 𝑞 using the bit vector of 𝑝. We use 𝑣 as a symbol that represents the bit vector of 𝑝.
Here are some explanations on the notation we use when drawing NBVA diagrams:
− The action 𝜗 is omitted only when 𝑤 (𝑝) = 𝑤 (𝑞). The omission of 𝜗 indicates that the bit vector
is propagated unchanged, i.e., 𝜗 is the identity function. We can indicate this explicitly in an
NBVA diagram by writing the expression “𝑣” that represents the identity function.
− We write “𝑣 [𝑛] = 1” to denote the function of type B𝑛 → B (which can also be viewed as a
predicate over B𝑛 ) that checks whether the value of 𝑣 at the 𝑛-th position is equal to 1.
− We write “shft(𝑣)” to denote the action (function of type B𝑛 → B𝑛 ) that shifts a bit vector by
one position. More formally, the shift operation is defined as follows:
shft(𝑣) [1] = 0

and

shft(𝑣) [𝑖] = 𝑣 [𝑖 − 1] for every 𝑖 = 2, . . . , 𝑛.

We use double circle notation to indicate that a state is final (see state 𝑞 3 above). An arrow emanating
from a final state 𝑞 is annotated with a description of the function 𝐹 (𝑞) : B𝑤 (𝑞) → B.
Fig. 2 shows the execution of the NCA and NBVA for the regex Σ∗𝑎Σ{7} (or .*a.{7} in POSIX
notation), which has the same form as 𝑟 = Σ∗𝜎1𝜎2 {𝑛}.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 10 -->

92:10

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

NCA :

𝑎

𝑞1

Σ/𝑥 ≔ 1

𝑞2

Σ

NBVA :

Σ / 𝑣 · 0[1 ↦→ 1]

𝑞2

𝑏
𝑎
𝑎
𝑎
𝑏
𝑏
𝑎
𝑏
𝑎
𝑎
𝑎
𝑎
𝑏
𝑏
𝑏
𝑏
𝑏
𝑏
𝑏

Σ

𝑞3 : 3

Σ

input

𝑞4 : 𝑥

2 ≤𝑥 ≤3

Σ, 𝑥 < 3 / 𝑥++
𝑎

𝑞1

Σ

𝑞3 : 𝑥

𝑞4 : 3

𝑣 [2..3] ≠ 0

Σ / shft(𝑣)

NCA configuration
𝑞1
𝑞1
𝑞1, 𝑞2
𝑞 1 , 𝑞 2 , (𝑞 3 , 1)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 4 , 1)
𝑞 1 , (𝑞 3 , 1), (𝑞 4 , 1), (𝑞 3 , 2)
𝑞 1 , (𝑞 4 , 1), (𝑞 3 , 2), (𝑞 4 , 2)
𝑞 1 , 𝑞 2 , (𝑞 3 , 2), (𝑞 4 , 2), (𝑞 3 , 3)
𝑞 1 , (𝑞 3 , 1), (𝑞 4 , 2), (𝑞 3 , 3), (𝑞 4 , 3)
𝑞 1 , 𝑞 2 , (𝑞 4 , 1), (𝑞 3 , 3), (𝑞 4 , 3)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 3 , 2), (𝑞 4 , 3)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 4 , 1), (𝑞 4 , 2)
𝑞 1 , 𝑞 2 , (𝑞 3 , 1), (𝑞 4 , 1), (𝑞 3 , 2), (𝑞 3 , 3)
𝑞 1 , (𝑞 3 , 1), (𝑞 4 , 1), (𝑞 3 , 2), (𝑞 4 , 2), (𝑞 4 , 3)
𝑞 1 , (𝑞 4 , 1), (𝑞 3 , 2), (𝑞 4 , 2), (𝑞 3 , 3)
𝑞 1 , (𝑞 3 , 2), (𝑞 4 , 2), (𝑞 3 , 3), (𝑞 4 , 3)
𝑞 1 , (𝑞 4 , 2), (𝑞 3 , 3), (𝑞 4 , 3)
𝑞 1 , (𝑞 3 , 3), (𝑞 4 , 3)
𝑞 1 , (𝑞 4 , 3)
𝑞1

NBVA configuration
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 0⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 0⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 0, 0⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 0, 0⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 0, 0⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 1, 0⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 1, 0⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 1, 1⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 0, 1⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨0, 0, 1⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 1, 0⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 0, 0⟩,
𝑞 2 ↦→ 1, 𝑞 3 ↦→ ⟨1, 1, 1⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨1, 1, 0⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 1, 1⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 1, 1⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 1⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 1⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 0⟩,
𝑞 2 ↦→ 0, 𝑞 3 ↦→ ⟨0, 0, 0⟩,

𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,
𝑞 1 ↦→ 1,

𝑞 4 ↦→ ⟨0, 0, 0⟩
𝑞 4 ↦→ ⟨0, 0, 0⟩
𝑞 4 ↦→ ⟨0, 0, 0⟩
𝑞 4 ↦→ ⟨0, 0, 0⟩
𝑞 4 ↦→ ⟨1, 0, 0⟩
𝑞 4 ↦→ ⟨1, 0, 0⟩
𝑞 4 ↦→ ⟨1, 1, 0⟩
𝑞 4 ↦→ ⟨0, 1, 0⟩
𝑞 4 ↦→ ⟨0, 1, 1⟩
𝑞 4 ↦→ ⟨1, 0, 1⟩
𝑞 4 ↦→ ⟨0, 0, 1⟩
𝑞 4 ↦→ ⟨1, 1, 0⟩
𝑞 4 ↦→ ⟨1, 0, 0⟩
𝑞 4 ↦→ ⟨1, 1, 1⟩
𝑞 4 ↦→ ⟨1, 1, 0⟩
𝑞 4 ↦→ ⟨0, 1, 1⟩
𝑞 4 ↦→ ⟨0, 1, 1⟩
𝑞 4 ↦→ ⟨0, 0, 1⟩
𝑞 4 ↦→ ⟨0, 0, 1⟩
𝑞 4 ↦→ ⟨0, 0, 0⟩

output
0
0
0
0
0
0
1
1
1
1
1
1
0
1
1
1
1
1
1
0

Fig. 3. Execution of the NCA and the NBVA for the regular expression .*a(..){2,3}.

Example 3.3. We consider the regex 𝑟 = Σ∗𝑎(ΣΣ){2, 3}. This is the same as .*a(..){2,3} in
POSIX notation. This regex is recognized by the automata (NCA and NBVA respectively) of Fig. 3.
The table also shows an example execution of the automata.
NBVAs are conceptually nondeterministic, as they allow several transitions to be enabled from
each state. However, the presented semantics using configurations is deterministic and hence
directly implementable. This can be seen clearly in Example 3.2 (Fig. 2) and Example 3.3 (Fig. 3). So,
we do not need to determinize an NBVA in order to perform matching.
Example 3.4. The regex 𝑟 = Σ∗𝜎 (𝜎 𝜎 𝜎 ){𝑚, 𝑛}𝜎 with 1 ≤ 𝑚 ≤ 𝑛 is recognized by the following
1

2 3 4

5

automata (NCA and NBVA):
𝜎2, 𝑥 < 𝑛 / 𝑥++

Σ
𝜎1

𝑞1

𝑞2

𝜎2 / 𝑥 := 1

𝑞3 : 𝑥

𝜎3

𝜎4

𝑞5 : 𝑥

𝜎5, 𝑚 ≤ 𝑥 ≤ 𝑛

𝑞6

𝜎2 / shft(𝑣)

Σ
𝑞1

𝑞4 : 𝑥

𝜎1

𝑞2

𝜎2 / 𝑣 · 0[1 ↦→ 1]

𝑞3 : 𝑛

𝜎3

𝑞4 : 𝑛

𝜎4

𝑞5 : 𝑛

𝜎5 / 𝑣 [𝑚..𝑛] ≠ 0

𝑞6

We use the notation 𝑣 [𝑚..𝑛] ≠ 0 as abbreviation for the test 𝑣 [𝑚] = 1 ∨ · · · ∨ 𝑣 [𝑛] = 1 (which is
encoded as an action of type B𝑛 → B). For a pure state 𝑞 that is indicated as being final with a
double circle, 𝐹 (𝑞) : B → B is typically omitted and it is assumed to be the identity function.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 11 -->

Regular Expression Matching using Bit Vector Automata

𝜎2 / 𝑥 := 1

𝑞1

Σ
𝜎1

𝑞3 : 𝑥
+

𝜎 3,

𝑞2

𝜎3 / 𝑥 := 1

𝑥<

𝑛

+
/𝑥

𝑞1

𝑛/

𝑥+

𝜎3

/s

(𝑣 )

𝜎5, 𝑚 ≤ 𝑥 ≤ 𝑛

𝜎5 / 𝑣 [𝑚..𝑛] ≠ 0
𝜎2

𝜎2 / shft(𝑣)

𝑞6

+

𝑞5 : 𝑥

𝑞3 : 𝑛
hft

𝜎3 / 𝑣 · 0[1 ↦→ 1]

𝜎2, 𝑥 < 𝑛 / 𝑥++

𝜎5, 𝑚 ≤ 𝑥 ≤ 𝑛
𝑥<

𝜎4

𝑞4 : 𝑥

𝑞2

𝜎2 ,

𝜎3, 𝑥 < 𝑛 / 𝑥++

𝜎2 / 𝑣 · 0[1 ↦→ 1]
Σ
𝜎1

92:11

/s

hft

(𝑣 )

𝑞6

𝜎3 / shft(𝑣)
𝑞4 : 𝑛

𝜎4

𝑞5 : 𝑛

𝜎5 / 𝑣 [𝑚..𝑛] ≠ 0

Fig. 4. NCA and NBVA for the regular expression 𝑟 = Σ∗ 𝜎1 (𝜎2 |𝜎3 𝜎4 ){𝑚, 𝑛}𝜎5 with 1 ≤ 𝑚 ≤ 𝑛.

Example 3.5. The regex 𝑟 = Σ∗𝜎1 (𝜎2 |𝜎3𝜎4 ){𝑚, 𝑛}𝜎5 with 1 ≤ 𝑚 ≤ 𝑛 is recognized by the
automata (NCA and NBVA) shown in Fig. 4. An important difference from previous examples is
that the states 𝑞 3 and 𝑞 4 have 3 incoming arrows each. As we will discuss later, this corresponds to
a merge operation on bit vectors that can be computationally challenging.
The operations on bit vectors in NBVAs (setting the lowest-order bit, shifting, reading high-order
bits) is similar to the use of buffers and sliding windows in runtime verification with metric temporal
logic (MTL) [Chattopadhyay and Mamouras 2020; Mamouras et al. 2021a,b; Mamouras and Wang
2020]. MTL has temporal connectives that specify time durations using intervals of the form [𝑚, 𝑛].
These connectives are similar to the bounded repetition operators {𝑚, 𝑛} used in regexes. This
similarity suggests that NBVAs could be useful in the context of runtime verification.
Translation from NCA to NBVA. We consider an NCA A = (𝑄, 𝑅, Δ, 𝐼, 𝐹 ) whose states have
at most one counter register. That is, 𝑅(𝑞) is either empty or a singleton set for every control state
𝑞 ∈ 𝑄. We will define the NBVA A ′ = (𝑄, 𝑤, Δ ′, 𝐼 ′, 𝐹 ′) to have the same set of control states. If
𝑅(𝑞) = ∅, then we put 𝑤 (𝑞) = 1 (pure state). If 𝑅(𝑞) is a singleton set, then we put 𝑤 (𝑞) = 𝑛,
where 𝑛 is the bound for the counter values of 𝑞 (recall that we only consider NCAs with bounded
counters). Suppose that (𝑝, 𝜎, 𝜑, 𝑞, 𝜗) is a transition of A. When 𝑝 is pure we can assume w.l.o.g.
that 𝜑 is true (i.e., 𝜑 is equal to 𝑅(𝑝) → N), otherwise the transition could be removed from the
NCA. We will examine four cases, based on whether 𝑝 and 𝑞 are pure or not.
− Case: both 𝑝 and 𝑞 are pure. Both the guard 𝜑 and the action 𝜗 are trivial. We put the transition
(𝑝, 𝜎, 𝑞, 𝜗 ′) in A ′, where 𝜗 ′ (0) = 0 and 𝜗 ′ (1) = 1 (i.e., 𝜗 ′ is the identity function).
− Case: only 𝑝 is pure. The guard 𝜑 is trivial and the action 𝜗 specifies a constant counter value
𝑐. We put the transition (𝑝, 𝜎, 𝑞, 𝜗 ′) in A ′, where 𝜗 ′ (0) = 0 and 𝜗 ′ (1) = 0[𝑐 ↦→ 1], the vector
which is zero everywhere except for position 𝑐.
− Case: only 𝑞 is pure. The action 𝜗 is trivial. We put the transition (𝑝, 𝜎, 𝑞, 𝜗 ′) in A ′, where
(
1, if 𝑣 [𝑖] = 1 for some 𝑖 ∈ 𝜑
′
𝜗 (𝑣) =
0, otherwise.
− Case: none of 𝑝, 𝑞 is pure. We put the transition (𝑝, 𝜎, 𝑞, 𝜗 ′) in A ′, where
o
Õn
𝜗 ′ (𝑣) =
0[𝜗 (𝑖) ↦→ 1] | 1 ≤ 𝑖 ≤ 𝑤 (𝑝) such that 𝑖 ∈ 𝜑 and 𝑣 [𝑖] = 1 .
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 12 -->

92:12

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

Í
Recall that the symbol is bitwise OR (disjunction).
We consider now the definition of the initialization function 𝐼 ′. For a pure state 𝑞, we define
′
𝐼 (𝑞) = 1 if 𝑞 is initial and 𝐼 ′ (𝑞) = 0 if 𝑞 is not initial. For a counting state 𝑞, we define
(
1, if 𝑖 ∈ 𝐼 (𝑞)
𝐼 ′ (𝑞) [𝑖] =
0, otherwise
for every position 𝑖 with 1 ≤ 𝑖 ≤ 𝑤 (𝑞).
Finally, we will define the finalization function 𝐹 ′. For a pure state 𝑞, we define the function
′
𝐹 (𝑞) : B → B as follows:
(
1, if 𝑣 = 1 and 𝑞 is final
′
𝐹 (𝑞)(𝑣) =
0, otherwise.
For a counting state 𝑞, we set 𝐹 ′ (𝑞)(𝑣) = 1 iff there is some 𝑖 with 𝑣 [𝑖] = 1 such that 𝑖 ∈ 𝐹 (𝑞).
Lemma 3.6. Let A = (𝑄, 𝑅, Δ, 𝐼, 𝐹 ) be an NCA that uses at most one counter register. Then, the
NBVA that results from translating A (according to the previous paragraphs) is equivalent to A, i.e.,
it recognizes the same language as A.
Proof. Let B = (𝑄, 𝑤, Δ ′, 𝐼 ′, 𝐹 ′) be the NBVA that results from the translation. The equivalence
is seen by using the configuration semantics for NCAs. For a state 𝑞, the bit vector 𝑣 for 𝑞 records
the set of active tokens on the state. More specifically, the following hold for the execution of A
and B on the same input string 𝑥 ∈ Σ∗ :
(1) If 𝑞 is a pure state, then 𝑞 ∈ ⟦A⟧(𝑥) iff ⟦B⟧(𝑥)(𝑞) = 1.
(2) If 𝑞 is a counting state and 𝑖 is a position 𝑖 with 1 ≤ 𝑖 ≤ 𝑤 (𝑞), then
(𝑞, 𝑖) ∈ ⟦A⟧(𝑥) ⇐⇒ ⟦B⟧(𝑥)(𝑞) [𝑖] = 1.
The proof proceeds by induction on the input string 𝑥. For the base case 𝑥 = 𝜀, we recall that the
initial configuration of A is equal to ⟦A⟧(𝜀) and it consists of all initial tokens.
(1) Let 𝑞 be a pure state. Then, 𝑞 ∈ ⟦A⟧(𝜀) iff 𝑞 is initial iff 𝐼 ′ (𝑞) = 1 iff ⟦B⟧(𝜀)(𝑞) = 1.
(2) Let 𝑞 be a counting state and 𝑖 be a position with 1 ≤ 𝑖 ≤ 𝑤 (𝑞). Then, (𝑞, 𝑖) ∈ ⟦A⟧(𝜀) iff (𝑞, 𝑖)
is an initial token iff 𝑖 ∈ 𝐼 (𝑞) iff 𝐼 ′ (𝑞) [𝑖] = 1 iff ⟦B⟧(𝜀)(𝑞) [𝑖] = 1.
For the step case, we consider the input string 𝑥𝑎. There are many cases to examine for the
transitions and the details are messy and not particularly interesting. For this reason, we will show
the argument for the case of a counting state 𝑞, for which there are incoming transitions only from
counting states. For an arbitrary position 𝑗 with 1 ≤ 𝑗 ≤ 𝑤 (𝑞), the following are equivalent:
(i) (𝑞, 𝑗) ∈ ⟦A⟧(𝑥𝑎)
(ii) (𝑞, 𝑗) ∈ 𝛿 (⟦A⟧(𝑥), 𝑎)
(iii) (𝑝, 𝑖) →𝑎 (𝑞, 𝑗) for some token (𝑝, 𝑖) ∈ ⟦A⟧(𝑥)
(iv) there exist a transition (𝑝, 𝜎, 𝜑, 𝑞, 𝜗) ∈ Δ and 𝑖 s.t. 𝑎 ∈ 𝜎, 𝑖 ∈ 𝜑, 𝑗 = 𝜗 (𝑖) and (𝑝, 𝑖) ∈ ⟦A⟧(𝑥)
[Note: from the I.H., we know that (𝑝, 𝑖) ∈ ⟦A⟧(𝑥) iff ⟦B⟧(𝑥)(𝑝) [𝑖] = 1]
(v) there exists a transition (𝑝, 𝜎, 𝑞, 𝜗 ′) ∈ Δ ′ s.t. 𝑎 ∈ 𝜎 and 𝜗 ′ (⟦B⟧(𝑥)(𝑝)) [ 𝑗] = 1
(vi) ⟦B⟧(𝑥𝑎)(𝑞) [ 𝑗] = 1
This special case shows the main idea of the argument, so we leave the other cases to the reader.
Now that we have proved the main claim, we can prove the lemma. Let 𝑥 ∈ Σ∗ be an arbitrary
string. The string 𝑥 is accepted by A iff ⟦A⟧(𝑥) contains some final token (𝑞, 𝑖) (i.e., 𝑖 ∈ 𝐹 (𝑞)) iff
⟦B⟧(𝑥)(𝑞) [𝑖] = 1 for some token (𝑞, 𝑖) with 𝑖 ∈ 𝐹 (𝑞) iff 𝐹 ′ (𝑞)(⟦B⟧(𝑥)(𝑞)) = 1 for some state 𝑞 iff
B accepts 𝑥.
□
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 13 -->

Regular Expression Matching using Bit Vector Automata

92:13

Relationship between NCAs and NBVAs. The NCAs (with bounded counters) that we defined
in Section 2 and the NBVAs of this section recognize exactly the regular languages, and therefore
have the same expressiveness. Both models are extensions of NFAs. So, they recognize at least the
regular languages. Moreover, both models are finite-state, because we consider bounded NCAs
and NBVAs with bounded-length bit vectors, and therefore recognize exactly the class of regular
languages.
Another relevant question is whether we can translate from one model to the other without a
substantial blowup in size. Informally, Definition 3.1 (NBVAs) allows the transition to depend, not
only on the counter valuations independently, but on the combination of active counter valuations.
In order to make the correspondence between NCAs and NBVAs more direct, we can require that
the actions 𝜗 in NBVAs are linear, that is: 𝜗 (0) = 0 and 𝜗 (𝑢 + 𝑣) = 𝜗 (𝑢) + 𝜗 (𝑣) for all bit vectors
𝑢, 𝑣 (where + is bitwise OR). Moreover, we can require that 𝐹 (𝑞) : B𝑤 (𝑞) → B is linear for every
state 𝑞. We say that 𝐹 is linear if 𝐹 (𝑞) is linear for every state 𝑞. The linearity of the finalization
function implies that 𝐹 (𝑞) is determined uniquely by the values 𝐹 (𝑞)(0[𝑖 ↦→ 1]) = 𝑓𝑞,𝑖 for each
𝑖 ∈ {1, . . . , 𝑤 (𝑞)}. Using linearity, we see that
𝐹 (𝑞)(𝑣) = 𝐹 (𝑞)

Ô

𝑣 [𝑖 ]=1 0[𝑖 ↦→ 1]



=

Ô

𝑣 [𝑖 ]=1 𝑓𝑞,𝑖 =

Ô𝑤 (𝑞)
𝑖=1


𝑣 [𝑖] ∧ 𝑓𝑞,𝑖 .

So, the function 𝐹 (𝑞) is determined by the bit vector [𝑓𝑞,1, 𝑓𝑞,2, . . . , 𝑓𝑞,𝑤 (𝑞) ], which is of length 𝑤 (𝑞).
All the NBVA actions and finalization functions that we consider in the examples of Section 3
are linear. We also observe that Definition 2.1 (NCAs) restricts each transition (𝑝, 𝜎, 𝜑, 𝑞, 𝜗) to a
deterministic action 𝜗 on counter valuations. That is, if the transition is enabled, then the current
counter valuation is transformed into the next counter valuation. If the action is allowed to be
nondeterministic, then the correspondence to NBVAs becomes more direct.
4 ALGORITHMS FOR EFFICIENT REGEX MATCHING
For regular expressions without bounded repetition (i.e., classical regular expressions with concatenation, choice and iteration), Thompson’s algorithm provides a strong efficiency guarantee: the
running time at each step, i.e., to process a single input symbol, is 𝑂 (𝑚), where 𝑚 is the size of the
regular expression. In general, there is no known algorithm for regexes with bounded repetition
that can offer such a good worst-case time complexity. We propose here specialized algorithms for
bounded repetition that offer efficiency similar to Thompson’s algorithm for regexes of a specific
(but widely occurring) form. First, we consider bounded repetitions of the form (𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛},
where each 𝜎𝑖 is a predicate (character class). We show in Theorem 4.1 that there is an algorithm
for matching such regexes with time complexity 𝑂 (𝑘) per step. Then, we continue to generalize
this result in Theorem 4.3 to bounded repetitions of the form 𝑟 {𝑚, 𝑛}, where 𝑟 is a counting-free
regex that recognizes strings of some fixed length 𝑘, that is, L (𝑟 ) ⊆ Σ𝑘 for some 𝑘 ≥ 0. The time
complexity in this more general case is 𝑂 (ℓ) per step, where ℓ is the number of predicate (character
class) occurrences in 𝑟 . Notice that in both cases the time complexity is the same as the complexity
that an NFA-based algorithm (e.g., Thompson’s) would have on the counting-free regex 𝑟 ∗ . So, our
results imply that we deal with bounded repetition without any (asymptotic) performance penalty
in terms of time per step compared to the classical (counting-free) setting.
One key idea for our algorithms is to use bit vectors as a compact data structure to manipulate
the configurations of NCAs/NBVAs. For NCAs/NBVAs that result from regular expressions with
bounded repetition, only the following operations arise:
(1) Creating a bit vector that is zero everywhere except for the lowest position. This results from
the NCA action 𝑥 := 1.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 14 -->

92:14

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

(2) Copying a bit vector from one control state to another. This operation corresponds to the
identity NCA action 𝑥 := 𝑥.
(3) Shifting a bit vector and filling in a zero at the lowest position. This corresponds to the
back-edge of a counting loop, which involves the 𝑥++ NCA action.
(4) Merging bit vectors using the bitwise OR operation (disjunction). This arises whenever there
are more than two incoming edges at a control state.
Example 3.5 shows an NBVA where the execution requires the merging of bit vectors that may
contain a large number of 1’s. These cases are difficult to handle when the size of the bit vectors is
large, since the merging operation requires 𝑂 (𝑛) time, where 𝑛 is the bit vector size. Example 3.2,
Example 3.3, and Example 3.4 show NBVAs where merging occurs, but it is easy to handle because
one of the bit vectors has only one bit set. State 𝑞 3 of the NBVA in Example 3.2 merges the vector
shft(𝑣) with the vector 0[1 ↦→ 1]. This operation is the same as calculating shft(𝑣) [1 ↦→ 1], which
first shifts 𝑣 and then sets the lowest position to 1. We say that this is a trivial merge, because it
requires 𝑂 (1) time to perform, as we will see later in Section 4.1.
The algorithms that we will present in the rest of the section solve the problem of evaluating a
regex 𝑟 in the context of a bigger regex 𝑟 ′, that is, when 𝑟 appears as a sub-expression of 𝑟 ′. This is
a slightly more general matching problem than top-level matching. Intuitively, at the top level, the
matcher is initialized and started only once, at the beginning of the input stream. Within the context
of a larger regex, the matcher for 𝑟 can be “restarted” several times. This restarting corresponds to a
token flowing into the corresponding sub-automaton. Consider, for example, the regex (Σ · Σ · Σ) ∗𝑟 .
The inner regex 𝑟 is restarted every 3 characters. If we think of the corresponding automaton, a
token flows into the sub-automaton for 𝑟 every 3 characters. The functions that we will use for
evaluation take two arguments: a Boolean value 𝑏 and a symbol 𝑎 ∈ Σ. The first argument 𝑏 ∈ B
indicates whether the matcher is restarted at the current step.
If a regex contains bounded repetitions that either have small repetition bounds (e.g., 𝑟 {𝑚, 𝑛}
with 𝑚 ≤ 𝑛 ≤ 64, where 64 is the size of a CPU register) or are of the forms considered in this
section (“flat bounded repetition” and “loops of fixed length”), then the overall per-symbol time
complexity is 𝑂 (ℓ), where ℓ is the number of predicate (character class) occurrences in 𝑟 . This
upper bound matches the upper bound for classical regexes given by Thompson’s algorithm.
4.1 Flat Bounded Repetition
Consider a bounded repetition of the form 𝑟 = (𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛}, which can occur as a subexpression of a larger regular expression. The regular expression 𝑟 corresponds to a sub-automaton
(NCA and NBVA respectively) of the following form:
𝜎0, 𝑥 < 𝑛 / 𝑥++
𝜎0 / 𝑥 := 1

𝑞0 : 𝑥

𝜎1

𝑞1 : 𝑥

𝜎2

···

𝜎𝑘−1

𝑞𝑘−1 : 𝑥

𝑚≤𝑥 ≤𝑛

𝜎0 / shft(𝑣)
𝜎0 / 𝑣 · 0[1 ↦→ 1]

𝑞0 : 𝑛

𝜎1

𝑞1 : 𝑛

𝜎2

···

𝜎𝑘−1

𝑞𝑘−1 : 𝑛

𝑣 [𝑚..𝑛] ≠ 0

We observe that the only merging operation occurs at state 𝑞 0 and it is a trivial merge. For the
NBVA, this merge corresponds to the computation of a bit vector shft(𝑣) ∨ 0[1 ↦→ 1], where
∨ is elementwise disjunction (i.e., bitwise OR). This can be implemented by first shifting 𝑣 and
then setting the bit at position 1 to 1. So, the efficiency of this computation solely depends on the
efficiency of the shift operation. Another important issue is how to implement the test 𝑣 [𝑚..𝑛] ≠ 0
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 15 -->

Regular Expression Matching using Bit Vector Automata

State:
bit vectors 𝑤 0, 𝑤 1, . . . , 𝑤𝑘−1 of size 𝑛,
initialized to 0
1 Fn Update1(𝑏 : B, 𝑎 : Σ):
2
𝑤 0′ ← 𝑤𝑘−1
3
for 𝑖 = 1, . . . , 𝑘 − 1 do
4
𝑤𝑖′ ← 𝑤𝑖−1
′
5
𝑤 0 ← shft(𝑤 0′ )
6
if 𝑏 = 1 then
7
𝑤 0′ [1] ← 1
8
for 𝑖 = 0, . . . , 𝑘 − 1 do
9
if 𝑎 ∉ 𝜎𝑖 then
10
𝑤𝑖′ ← 0
11
for 𝑖 = 0, . . . , 𝑘 − 1 do
12
𝑤𝑖 ← 𝑤𝑖′
Ô
13
return 𝑛𝑗=𝑚 𝑤𝑘−1 [ 𝑗]

92:15

State:
variable 𝑠 ∈ {0, 1, . . . , 𝑘 − 1},
initialized to 0
bit vectors 𝑣 0, 𝑣 1, . . . , 𝑣𝑘−1 of size 𝑛,
initialized to 0
1 Fn Update2(𝑏 : B, 𝑎 : Σ):
2
𝑠 ← (𝑠 − 1) mod 𝑘
3
𝑣𝑠 ← shft(𝑣𝑠 )
4
if 𝑏 = 1 then
5
𝑣𝑠 [1] ← 1
6
for 𝑖 = 0, . . . , 𝑘 − 1 do
7
if 𝑎 ∉ 𝜎𝑖 then
8
𝑖 ′ ← (𝑠 + 𝑖) mod 𝑘
9
𝑣𝑖 ′ ← 0
10
𝑖 ← (𝑠 − 1) mod 𝑘
Ô
11
return 𝑛𝑗=𝑚 𝑣𝑖 [ 𝑗]

Fig. 5. Simple algorithms for matching subexpressions of the form (𝜎0 𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛}.

efficiently, since the obvious implementation requires reading 𝑛−𝑚 bits and therefore takes Θ(𝑛−𝑚)
time. We will consider a sequence of algorithms that will lead to an efficient implementation.
Version 1. On the left-hand side of Fig. 5, we see a simple algorithm for implementing a bounded
repetition of the form (𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛} that can be used in any context within a larger regular
expression. This is a direct implementation of the semantics of the corresponding NBVA. The state
of the algorithm consists of 𝑘 bit vectors, each of size 𝑛, which we call 𝑤 0, 𝑤 1, . . . , 𝑤𝑘−1 . At every
step of the computation, the function Update1 is called. It takes as input a Boolean value 𝑏, which
indicates the presence (value 1) or absence (value 0) of a transition from the rest of the automaton
into the loop. The idea is that all the bit vectors have to rotate around the cycle by one position to
the right (there is a back edge from 𝑞𝑘−1 to 𝑞 0 ). If the current symbol 𝑎 ∈ Σ does not satisfy the
predicate 𝜎𝑖 , then the bit vector 𝑤𝑖 should become 0 at the end of the step. We use the bit vectors
′
𝑤 0′ , 𝑤 1′ , . . . , 𝑤𝑘−1
as temporary variables, so as to not overwrite bit vectors that need to be used
later. The case of the bit vector 𝑤 0 is the most interesting one, because control state 𝑞 0 is the “entry
point” to the loop. In particular, it is (1) the target of the back edge of the loop, and (2) the target of
transitions from the rest of the automaton into the loop. The combined effect of lines 2 & 5 (see
pseudocode of Fig. 5) is that 𝑤 0′ is set to be equal to shft(𝑤𝑘−1 ), i.e., the result of shifting 𝑤𝑘−1 (bit
vector for 𝑞𝑘−1 from the previous step) by one position. Moreover, if the Boolean variable 𝑏 is equal
to 1 (“true”), then 𝑤 0′ [1] is set to 1. The output of the function Update1 indicates whether there is
a transition from the “exit” control state 𝑞𝑘−1 to the rest of the automaton. This happens exactly
when at least one of the bits 𝑤𝑘−1 [𝑚], . . . , 𝑤𝑘−1 [𝑛] is equal to 1. This is why we specify the output
Ô
value to be the finite disjunction 𝑛𝑗=𝑚𝑤𝑘−1 [ 𝑗].
We see in Fig. 5 (left-hand side) that there are several operations that are performed on bit vectors
in Update1. The assignments in lines 2, 4 and 12 are implemented as pointer assignments. The
assignment 𝑤 0′ ← shft(𝑤 0′ ) in line 5 is implemented as a shift operation on the bit vector data
structure. The assignment 𝑤 0′ [1] ← 1 in line 7 sets the lowest position of the bit vector to 1. The
assignment 𝑤𝑖′ ← 0 in line 10 sets the bit vector to 0. Finally, for the return expression of line 13,
we need to support an operation that reads the bit from a specified position in a bit vector. Fig. 6
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 16 -->

92:16

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

Fields of Data Structure:
variable start ∈ {0, 1, . . . , 𝑛 − 1}
variable size ∈ {0, 1, . . . , 𝑛}
𝑛-bit array arr (0-based indexing)
Fn SetToZero():
start ← 0
3
size ← 0

1

2

Fn SetAtPos0(𝑏 : B):
if size = 0 then
3
size ← 1
4
arr [start] ← 𝑏

1

2

Fn Shift():
start ← (start − 1) mod 𝑛
3
arr [start] ← 0
4
if size < 𝑛 then
5
size ← size + 1

1

2

Fn Get(𝑖 : N):
if 𝑖 < size then
3
𝑗 ← (start + 𝑖) mod 𝑛
4
return arr [ 𝑗]
5
else
6
return 0

1

2

Fig. 6. A data structure for 𝑛-bit vectors that provides efficient implementations for several operations.

shows the implementation of a data structure that can support all the needed operations. Each
operation requires 𝑂 (1) time. Note that the data structure of Fig. 6 uses 0-based array indexing
(for convenience of implementation), but in Fig. 5 we use 1-based indexing for the bit vectors. The
main idea is to use a circular array to support efficient shifting. In order to set the array to zero
without filling in 0’s, we introduce the variable size. The idea is that the represented vector 𝑣 has
value 0 at all positions 𝑖 ≥ size. In particular, if size = 0, then the vector is equal to 0.
The function Update1 performs 𝑂 (𝑘 + (𝑛 − 𝑚)) work at every step. The updating of each bit
vector needs 𝑂 (1) time, because the assignments can be implemented with pointers (i.e., without
copying the bit vectors) and the shift operation is implemented efficiently by representing a bit
vector with a circular array. The generation of the output value takes Θ(𝑛 − 𝑚) time, because
it requires reading 𝑛 − 𝑚 bits. This cost, which is dependent on the repetition bounds, can be
eliminated, as we will see later.
Version 2. On the right-hand side of Fig. 5, we see a small modification of the algorithm for the
bounded repetition (𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛}. The bit vectors 𝑣 0, 𝑣 1, . . . , 𝑣𝑘−1 are not copied, but instead
a variable 𝑠 ∈ {0, . . . , 𝑘 − 1} gives the index that corresponds to the “entry” control state 𝑞 0 . In
general, the index (𝑠 + 𝑖) mod 𝑘 corresponds to the control state 𝑞𝑖 . Notice that
(𝑠 + (𝑘 − 1)) mod 𝑘 = (𝑠 − 1) mod 𝑘
is the index that corresponds to the “exit” control state 𝑞𝑘−1 . More precisely, this algorithm maintains
the following correspondence with the previous one:
𝑤𝑖 = 𝑣𝑖 ′ , where 𝑖 ′ = (𝑠 + 𝑖) mod 𝑘,
for every 𝑖 = 0, . . . , 𝑘 − 1. The time complexity of Update2 is the same as that of Update1. It does
avoid, however, the copying of bit vector pointers using this convenient trick with the indexes.
Version 3. We will see now in Fig. 7 how we can decrease the time complexity of the update
function from Θ(𝑘 + (𝑛 − 𝑚)), which depends on the repetition bounds 𝑚 and 𝑛, to Θ(𝑘), which
is independent of the repetition bounds. The idea to accomplish this is to maintain an additional
Ô
integer variable 𝑦𝑖 for every bit vector 𝑣𝑖 , which records the index of a witness for 𝑛𝑗=𝑚 𝑣𝑖 [ 𝑗]
being equal to 1 (“true”). More specifically, we record the smallest index 𝑗 ≥ 𝑚 such that 𝑣𝑖 [ 𝑗] = 1.
If no such index exists, then we put 𝑦𝑖 = ∞. By recording this information, we can replace the
Ô
computation of a disjunction 𝑛𝑗=𝑚 𝑣𝑖 [ 𝑗], which takes Θ(𝑛 − 𝑚) time, with the test 𝑚 ≤ 𝑦𝑖 ≤ 𝑛,
which takes constant time. It remains to see that the values of these variables can be updated in
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 17 -->

Regular Expression Matching using Bit Vector Automata

92:17

State:
variable 𝑠 ∈ {0, 1, . . . , 𝑘 − 1} (indicates the index for control state 𝑞 0 ), initialized to 0
bit vectors 𝑣 0, 𝑣 1, . . . , 𝑣𝑘−1 of size 𝑛 (one for each control state), initialized to 0
integer variables 𝑦0, . . . , 𝑦𝑘−1 ∈ {𝑚, . . . , 𝑛} ∪ {∞}, initialized to ∞
1 Fn Update3(𝑏 : 𝐵𝑜𝑜𝑙, 𝑎 : Σ):
2
𝑠 ← (𝑠 − 1) mod 𝑘 // rotate around the cycle by one position
3
𝑣𝑠 ← shft(𝑣𝑠 ) // shift the bit vector that is moving along the back edge
4
if 𝑏 = 1 then // incoming transition to the “entry” control state
5
𝑣𝑠 [1] ← 1
6
if 𝑣𝑠 [𝑚] = 1 then
7
𝑦𝑠 ← 𝑚 // new witness, which has value 𝑚
8
else if 𝑦𝑠 = 𝑛 then
9
𝑦𝑠 ← ∞ // the (unique) witness falls off
10
else // no new witness, 𝑦𝑠 < 𝑛 or 𝑦𝑠 = ∞
11
𝑦𝑠 ← 𝑦𝑠 + 1 // increment the witness, note that ∞ + 1 = ∞
12
for 𝑖 = 0, . . . , 𝑘 − 1 do
13
if 𝑎 ∉ 𝜎𝑖 then // transition to 𝑞𝑖 cannot be taken on symbol 𝑎
14
𝑖 ′ ← (𝑠 + 𝑖) mod 𝑘 // index for control state 𝑞𝑖
15
𝑣𝑖 ′ ← 0 // set bit vector to 0
16
𝑦𝑖 ′ ← ∞ // no witness
17
𝑖 ← (𝑠 − 1) mod 𝑘 // index for control state 𝑞𝑘−1
18
return 𝑚 ≤ 𝑦𝑖 ≤ 𝑛
Fig. 7. Efficient algorithm for matching subexpressions of the form (𝜎0 𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛}.

constant time per step. The main observation is that, at each step, only one bit vector changes,
namely the one that moves from the “exit” control state 𝑞𝑘−1 to the “entry” control state 𝑞 0 via
the back edge. This bit vector is shifted and potentially one bit is set to 1 (if the argument 𝑏 is
1). The variable 𝑦𝑠 is updated in time 𝑂 (1) to reflect these changes. The output is computed in
constant time with the Boolean expression (𝑚 ≤ 𝑦𝑖 ≤ 𝑛), where 𝑖 = (𝑠 − 1) mod 𝑘 is the index
corresponding to the control state 𝑞𝑘−1 . It follows that the total running time for every invocation
of Update3 (Fig. 7) is 𝑂 (𝑘), which is independent of the repetition bounds.
Theorem 4.1 (Efficient Flat Loops). Let 𝑟 be a regular expression of the form
(𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛},
where each 𝜎𝑖 is a predicate over the alphabet Σ. There is an algorithm for evaluating 𝑟 (where 𝑟 can
be used in any context within a larger regular expression) with time complexity 𝑂 (𝑘) for every step.
Proof. As we have already discussed, the algorithm of Fig. 7, which is based on NBVA execution,
achieves the time complexity bound of 𝑂 (𝑘) per step. This relies on an efficient implementation of
the shift operation on bit vectors. We have already mentioned that this can be achieved by using
circular arrays for representing bit vectors, where shifting corresponds to decrementing (modulo 𝑛,
the size of the bit vector) the index for the first element.
□
Memory Optimizations. Each bit vector of the algorithm of Fig. 7 can be implemented with an
array of 𝑛 bits, where 𝑛 is its size. An additional Θ(log2 𝑛) bits are needed to store an index within
the array, in order to efficiently implement the shift operation. This extra storage for the index is
negligible, as it corresponds to one memory location for all practical instances. This implementation
makes effective use of memory for dense bit vectors, that is, bit vectors that have a large number of
digits equal to 1.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 18 -->

92:18

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

For the case of sparse bit vectors, one may want to consider an implementation that uses a list of
indexes for the 1’s. If there are 𝐾 1’s, then this representation needs at least 𝐾 · log2 𝑛 bits. In fact,
there may be overheads associated with lists that demand more memory, but we will ignore them
for the sake of simplifying the analysis. If 𝐾 · log2 𝑛 < 𝑛 or, equivalently, 𝐾 < 𝑛/log2 𝑛, then it may
make sense to consider a “sparse” representation. This is the approach taken in [Turoňová et al.
2020]. We emphasize that choosing a sparse representation does not improve the time complexity.
It can only reduce the memory footprint when 𝐾 is smaller than 𝑛/log2 𝑛. If 𝐾 ≥ 𝑛/log2 𝑛, then the
“dense” representation with bit vectors is superior. In any case, the algorithms of Fig. 5 and Fig. 7
can be easily made to work with either representation of bit vectors.
4.2

Loops of fixed length

We say that a regular expression 𝑟 is of fixed length if there exists a nonnegative integer 𝑘 ≥ 0 such
that L (𝑟 ) ⊆ Σ𝑘 . That is, all strings recognized by 𝑟 are of the same length. A bounded repetition
𝑟 {𝑚, 𝑛} is said to be a fixed-length loop if 𝑟 is of fixed length. Some examples of fixed-length loops
are seen below:
𝜎 {𝑚, 𝑛}
(𝜎0𝜎1𝜎2 ){𝑚, 𝑛}

(𝜎0 |𝜎1 ){𝑚, 𝑛}

(𝜎0𝜎2𝜎4 |𝜎1𝜎3𝜎5 ){𝑚, 𝑛}

(𝜎0𝜎1 ){𝑚, 𝑛}

(𝜎0𝜎2 |𝜎1𝜎3 ){𝑚, 𝑛}

(𝜎0 (𝜎1𝜎3 |𝜎2𝜎4 )){𝑚, 𝑛}

((𝜎0𝜎2 |𝜎1𝜎3 )𝜎4 ){𝑚, 𝑛}

We will see that the algorithm of Fig. 7 can be generalized to work with fixed-length loops. This is
a strict generalization, as the examples above indicate.
Example 4.2. Consider the regular expression ((𝜎0𝜎2 |𝜎1𝜎3 )𝜎4𝜎5 ){𝑚, 𝑛}, which is a fixed-length
loop. We show below the sub-automaton (NCA and NBVA respectively) that this regular expression
would correspond to if it were used in the context of a larger regular expression:
𝜎0, 𝑥 < 𝑛 / 𝑥++
𝜎0 / 𝑥 := 1

𝑞0 : 𝑥

𝜎2

𝑞2 : 𝑥

𝜎4
𝑞4 : 𝑥

𝜎1 / 𝑥 := 1

𝑞1 : 𝑥

𝜎3

𝑞3 : 𝑥

𝜎5

𝑞5 : 𝑥

𝑚≤𝑥 ≤𝑛

𝜎4
𝜎1, 𝑥 < 𝑛 / 𝑥++
𝜎0 / shft(𝑣)

𝜎0 / 𝑣 · 0[1 ↦→ 1]

𝑞0 : 𝑛

𝜎2

𝑞2 : 𝑛

𝜎4
𝑞4 : 𝑛

𝜎1 / 𝑣 · 0[1 ↦→ 1]

𝑞1 : 𝑛

𝜎3

𝑞3 : 𝑛

𝜎5

𝑞5 : 𝑛

𝑣 [𝑚..𝑛] ≠ 0

𝜎4
𝜎1 / shft(𝑣)

We have numbered the states in “columns” from left to right. Column 0 consists of 𝑞 0 and 𝑞 1 , the
states that are reached in 1 step. Column 1 consists of 𝑞 2 and 𝑞 3 , the states that are reached in 2
steps. Column 2 (resp., column 3) consists of 𝑞 4 (resp., 𝑞 5 ), the state that is reached in 3 (resp., 4)
steps.
Theorem 4.3 (Efficient Fixed-Length Loops). Let 𝑟 be a counting-free regular expression
of the form 𝑟 {𝑚, 𝑛}, where 𝑟 is a fixed-length regular expression. There is an algorithm for evaluating
𝑟 (where 𝑟 can be used in any context within a larger regular expression) with time complexity 𝑂 (ℓ)
for every step, where ℓ is the number of predicate occurrences in 𝑟 .
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 19 -->

Regular Expression Matching using Bit Vector Automata

92:19

State:
(there are 𝑘 columns and ℓ control states, which means that ℓ ≥ 𝑘)
variable 𝑠 ∈ {0, 1, . . . , 𝑘 − 1} (indicates the index for column 0), initialized to 0
bit vectors 𝑤 0, 𝑤 1, . . . , 𝑤𝑘−1 of size 𝑛 (one for each column), initialized to 0
bit vectors 𝑣 0, 𝑣 1, . . . , 𝑣 ℓ−1 ∈ {0, 1}, initialized to 0
integer variables 𝑦0, 𝑦1, . . . , 𝑦ℓ−1 ∈ {𝑚, . . . , 𝑛} ∪ {∞} for “witnesses”, initialized to ∞
1 Fn Update(𝑏 : 𝐵𝑜𝑜𝑙, 𝑎 : Σ):
2
Update the bit vectors 𝑣 0, . . . , 𝑣 ℓ−1 and the variables 𝑦0, 𝑦1, . . . , 𝑦ℓ−1 as if they were tokens
of automata, using 𝑏 and 𝑎 (merge the bit vectors with bitwise OR & the integer witnesses
with min). (We discuss in Theorem 4.3 how this can be done efficiently.)
3
𝑠 ← (𝑠 − 1) mod 𝑘 // rotate columns around the cycle by one position
4
𝑤𝑠 ← shft(𝑤𝑠 ) // shift the bit vector that is moving along the back edge
5
if 𝑏 = 1 then // incoming transition to the “entry” control state
6
𝑤𝑠 [1] ← 1
7
Let 𝐽 ⊆ {0, . . . , ℓ − 1} be the set of state indexes for “exit” control states.
Ô
8
return 𝑗 ∈𝐽 (𝑚 ≤ 𝑦 𝑗 ≤ 𝑛)

Fig. 8. Efficient algorithm for matching subexpressions of the form 𝑟 {𝑚, 𝑛}, where 𝑟 is of fixed length 𝑘.

Proof. We invite the reader to use the automata of Example 4.2 in order to understand the main
ideas in this proof. Let 𝑠𝑖 be the size variable (of the data structure described in Fig. 6) for the bit
vector 𝑣𝑖 . The key observation is that we only need to store one bit array per column of states
(recall from Example 4.2 what state columns are). This is because for every control state 𝑞𝑖 of some
column 𝑗 the following holds: the bit vector 𝑣𝑖 is determined by the size variable 𝑠𝑖 and the bit vector
𝑤 𝑗 for column 𝑗. More precisely, the following property holds: 𝑣𝑖 [𝑡] = 𝑤 𝑗 [𝑡] for every 𝑡 = 1, . . . , 𝑠𝑖
and 𝑣𝑖 [𝑡] = 0 for 𝑡 > 𝑠𝑖 . When this property holds, we say that 𝑣𝑖 is column-compatible. Notice that
column-compatibility means that we do not need to store a bit array for 𝑣𝑖 , since 𝑣𝑖 is determined
by 𝑠𝑖 and 𝑤 𝑗 . The column-compatibility property can be proved by induction. In the beginning,
all states are inactive and their bit vectors are equal to 0. When a symbol is consumed, each bit
vector goes from some state of column 𝑗 to some state of column ( 𝑗 + 1) mod 𝑘. If any merging of
bit vectors happens, then the property is preserved, because the merging of column-compatible bit
vectors produces a column-compatible bit vector. For column 0, every state merges the bit vectors
from column 𝑘 − 1 (by the I.H., they are column-compatible) and sets the value at position 1 to
1 (if a transition from the environment to the entry states occurs). So, the property of columncompatibility is preserved for column 0 as well. The column-compatibility property justifies a key
optimization: removing the bit arrays from the (per state) vectors 𝑣 0, 𝑣 1, . . . , 𝑣 ℓ−1 and using the
corresponding bit arrays from the (per column) vectors 𝑤 0, 𝑤 1, . . . , 𝑤𝑘−1 . This optimization means
that the bit vector merging (see line 2 in Fig. 8) can involve only the size variables from the data
structures (taking their maximum) without performing any operations on bit arrays. This makes
the merging a constant-time operation. It follows that this optimized algorithm performs 𝑂 (ℓ)
work (for line 2) at each step. So, the total running time per step is 𝑂 (ℓ).
□
5 EXPERIMENTS
In this section, we compare the performance of our regex engine, BVA-Scan, against three opensource engines: RE2 [RE2 2023], PCRE [Hazel and Herczeg 2022], and CA [Turoňová et al. 2020].
RE2 is a state-of-the-art regex engine that is based on DFAs and NFAs. The main idea for the
practical efficiency of RE2 is that it generates and caches the DFA for the regular expression on the
fly. If the generated DFA stays within a predetermined size, then each step involves one memory
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 20 -->

92:20

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

lookup and is very efficient. If the DFA becomes too large, then the engine may end up reverting
to an NFA simulation algorithm, which is substantially slower. PCRE is a standard backtracking
engine. This means that some patterns and input strings can give rise to exponential running
time (in the length of the input string). The average behavior of PCRE, however, is much better
than what the worst case suggests. CA is an engine based on the simulation of the execution
of a variant of deterministic counter automata (“counting-set automata”). CA is able to handle a
subclass of regexes with large repetition bounds in constant time. However, for some regexes, CA’s
determinization algorithm overapproximates the language defined by the regular expression.
We have evaluated the performance of BVA-Scan, RE2, PCRE and CA with both microbenchmarks
(Section 5.1) and benchmarks that contain regexes used in real-world applications (Section 5.2).
BVA-Scan. We have implemented BVA-Scan using the Rust programming language. Our implementation is based on the deterministic execution of NBVAs using bit vectors (see Section 3),
and it incorporates the efficient algorithms for bounded repetition from Section 4. For a bounded
repetition 𝑟 {𝑚, 𝑛}, where 𝑟 is not flat (Theorem 4.1) or of fixed length (Theorem 4.3), it is still
possible to provide an efficient implementation using NBVAs when 𝑛 is not too large. For example,
if 𝑛 ≤ 64 then each bit vector can fit in a CPU register and therefore all NBVA operations (shift,
merge, set, read) can be performed in 𝑂 (1) time. This approach is much better than naive unfolding,
which is the same as expanding an NCA into an NFA, as it takes advantage of the bit parallelism
that the CPU can offer. We have also included other performance optimizations that are commonly
found in regex engines, such as an efficient algorithm for string search. These optimizations are
helpful because many regexes that arise in real applications are of a relatively simple form.
Experimental setup. The experiments were executed in Ubuntu 20.04 on a desktop computer
equipped with an Intel(R) Xeon(R) W-2295 CPU (18 cores) and 128 GB of RAM. We used Rust 1.59.0,
GCC/G++ 9.4.0, and the CA binary downloaded from [Library 2021]. For each experiment, we
executed 10 trials and we report the mean of the measurements.
5.1 Microbenchmarks
We have evaluated the performance of the algorithms presented in Section 4 using regexes of the following 8 forms: a.{𝑘}c, a(..){𝑘}c, a.{0,𝑘}c, a(..){0,𝑘}c, a.{𝑘,}c, a(..){𝑘,}c, a.{𝑘/2,𝑘}c,
and a(..){𝑘/2,𝑘}c. We vary the values for the parameter 𝑘 in order to examine the effect of
the repetition bounds on the running time. These regexes are similar to patterns that are used in
real-world applications. They are sometimes not supported by or cause timeouts in state-of-the-art
engines like RE2 and PCRE, especially for larger values of 𝑘. We matched these regexes over an
input string that contains 10 million characters. Each character is either the English letter ‘a’ or ‘b’,
and is chosen uniformly at random. Therefore, the last predicate c in each microbenchmark regex
ensures that all engines will be forced to search for a match over the entire input.
Fig. 9 shows the results for each benchmark. The black dashed line indicates the 10-second
timeout limit that we have set. Each point in Fig. 9 is a measurement of the time it took for a regex
engine to match a pattern over the input text. When an engine does not support a regex or the
running time exceeds the 10-second timeout limit, then there is not a corresponding measurement.
For example, RE2 does not support regexes with repetition bounds that exceed 1000. Therefore,
when 𝑘 > 1000, no point is shown for RE2. The CA engine does not support the regexes a(..){𝑘}c,
a(..){0,𝑘}c, a(..){𝑘,}c, and a(..){𝑘/2,𝑘}c, because it uses a model of counter automata that
is more restricted than our NCA model of Section 2. Moreover, when 𝑘 is small, we have observed
that CA sometimes crashes and we are therefore unable to obtain a measurement (as seen in Fig. 9).
The PCRE engine reaches the 10-second timeout limit on all microbenchmarks when 𝑘 becomes
large enough. Moreover, it times out on regexes a.{𝑘,}c and a(..){𝑘,}c for all values of 𝑘. These
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 21 -->

Regular Expression Matching using Bit Vector Automata
(a) a.{k}c

92:21

(b) a.{0,k}c

timeout

(c) a.{k,}c

timeout

(d) a.{k/2,k}c

timeout

timeout

10000
1000
100

matchers

time (ms)

10

bva_no_opt

1

bva_opt
(e) a(..){k}c

(f) a(..){0,k}c

timeout

(g) a(..){k,}c

timeout

(h) a(..){k/2,k}c

timeout

ca

timeout

10000

pcre

1000

re2

100
10
1
16

128

1024

16384

16

128

1024

16384

16

128

1024

16384

16

128

1024

16384

k

Fig. 9. Matching time (ms) of regexes in RE2, CA, PCRE, and BVA-Scan with and without optimizations for
different values of 𝑘 (timeout = 10 sec)

regexes are equivalent to a.{𝑘}.*c and a(..){𝑘}(..)*c respectively. They are similar to the
pattern a.*c, which is known to cause catastrophic backtracking [Goyvaerts 2021].
The results of the microbenchmarks show that BVA-Scan (i.e., bva_opt) provides competitive
performance against RE2 and PCRE when 𝑘 is small (e.g., 𝑘 < 128). For larger values of 𝑘, the speed
of BVA-Scan remains constant as 𝑘 increases. BVA-Scan is faster than CA by at least 2×, and faster
than RE2 and PCRE by orders of magnitude. Notice that the performance of BVA-Scan (bva_opt)
is better for smaller values of 𝑘 (i.e., 𝑘 ≤ 32) compared to larger values of 𝑘 (i.e., 𝑘 > 32). This
is because for smaller values of 𝑘 the bit vectors used in our implementation fit entirely in CPU
registers, which accelerates the bit vector operations that are performed during matching. Moreover,
the matching speed of BVA-Scan is particularly fast for regex a.{𝑘,}c (see Fig. 9(c)) as it provides
an optimization for that regex pattern. More specifically, the matching of regex a.{𝑘,}c, which is
the same as a.{𝑘}.*c, can be implemented by searching for the first occurrence of a, then skipping
𝑘 letters, followed by a search for the letter c. We have also evaluated the performance of BVA-Scan
with all optimizations disabled (i.e., bva_no_opt), in order to evaluate only the performance of the
algorithms presented in Section 4.
5.2

Application Benchmarks

We have evaluated the performance of BVA-Scan using six benchmarks, which contain regexes
collected from real applications. These benchmarks are: (1) the Snort [Roesch 1999; Snort 2023]
and (2) Suricata benchmarks [Suricata 2023] that contain patterns for network traffic, (3) the
Protomata benchmark that includes 1309 protein motifs from the PROSITE database [Roy and
Aluru 2016; Sigrist et al. 2009], (4) the ClamAV benchmark [ClamAV 2023] that contains patterns
that indicate the presence of viruses, (5) the SpamAssassin benchmark [SpamAssassin 2022] that
includes patterns for detecting spam email, and (6) the RegexLib benchmark [RegexLib 2023] that
is a collection of regexes for describing email addresses, phone numbers, and so on. In total, we
have collected around 20,000 regexes.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 22 -->

92:22

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

number of regexes

ClamAV

Protomata

RegexLib

Snort

SpamAssassin

Suricata

100

10

1
1e+02

1e+04

1e+06

10

30

100

3

10

30

100 300

10

100

1000 10000

10

100

1000

10

100

1000 10000

number of NFA states

Fig. 10. Distribution of the number of NFA states generated by unfolding regexes

number of occurrences
of bounded repetition

ClamAV

Protomata

RegexLib

Snort

SpamAssassin

Suricata

1000
100
10
1
1e+02

1e+04

1e+06

3

10

30

100

3

10

30

100 300

10

100

1000 10000

10

100

1000

10

100

1000 10000

value of bound

Fig. 11. Distribution of upper repetition bounds in regexes

5.2.1 Analysis of Regex Datasets. In the Snort, Suricata, SpamAssassin, and RegexLib benchmarks,
some of the collected regexes may contain backreferences [Backreferences 2022], which are not
regular constructs (i.e., they can give rise to non-regular languages). We filter out regexes with
backreferences from the source datasets (about 2% of all regexes) and we eliminate duplicates. The
benchmarking datasets contain the remaining regexes (64% of them contain bounded repetition).
Fig. 10 shows the distribution of the number of NFA states needed for the regexes of each dataset.
For each regex, the number of NFA states is calculated by unfolding the bounded repetitions, e.g.,
(𝑎𝑏){3} is unfolded into 𝑎𝑏𝑎𝑏𝑎𝑏. The horizontal axis is the number of NFA states, and the vertical
axis is the count of regexes. We observe that there are many regexes whose NFAs have a large
number of states. For such regexes, it would be computationally expensive to perform matching
using NFA simulation.
Fig. 11 shows the distribution of the upper repetition bounds of bounded repetitions. For the
patterns 𝑟 {𝑛} (which is abbreviation for 𝑟 {𝑛, 𝑛}), 𝑟 {𝑚, 𝑛} and 𝑟 {𝑛, } (which is equivalent to 𝑟 {𝑛}𝑟 ∗ ),
the value 𝑛 is the upper repetition bound. The horizontal axis of Fig. 11 is the upper repetition
bound, and the vertical axis is the number of occurrences of bounded repetition. We observe that all
the datasets have many regexes with large upper repetition bounds. BVA-Scan uses the algorithms
described in Section 4 to deal effectively with such regexes.
5.2.2 Benchmarking Results. We measured the running time of matching for regexes in different
application benchmarks. For each experiment we used an input sequence of 1 million bytes. We used
two different methods to generate these input sequences: (1) by choosing each byte independently
and uniformly at random and (2) using an adversarial input generator to construct the input. The
adversarial input generator analyzes a regex in order to identify a set of characters that are likely
to trigger a state transition during the execution of the automaton. Then, the generator randomly
selects characters from this set to construct the input string. We measured the running time for
regex matching using both of these methods of input generation.
Table 1 shows the ratios of unhandled cases of regex matching for RE2, CA, PCRE, and BVA-Scan,
where unhandled means that (1) the regex can be parsed but is not supported for matching (e.g., the
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 23 -->

Regular Expression Matching using Bit Vector Automata

92:23

Table 1. Ratios of unhandled cases of regex matching in different engines (x indicates the regex is unsupported,
o means the matching time exceeds 10 seconds)
Benchmark
Snort
Suricata
Protomata
ClamAV
SpamAssassin
RegexLib

# RE2 x
0.29%
0.35%
0%
7.63%
0%
0%

# RE2 o
0%
0%
0%
0%
0%
0%

# CA x
27.19%
27.99%
40.42%
4.40%
40.08%
37.35%

# CA o
0.21%
0.13%
28.57%
6.37%
1.31%
1.81%

# PCRE x
0%
0%
0%
0%
0%
0%

# PCRE o
1.60%
0.89%
0%
1.18%
1.28%
1.41%

# BVA-Scan x
0%
0%
0%
0%
0%
0%

# BVA-Scan o
0%
0%
0%
0%
0%
0%

upper repetition bound is too large), or (2) the engine times out during matching (i.e., the running
time exceeds 10 seconds).
RE2 does not support regexes with upper repetition bounds that exceed 1000. CA cannot handle
a large number of regexes from the application benchmarks. We have observed that it fails (i.e.,
crashes with an exception) in many cases where the pattern is a string or it contains sub-patterns
𝑟 {𝑚, 𝑛} with small repetition bounds, but the reason behind these failures is unclear. Moreover, CA
does not support the matching of certain classes of patterns such as .*(aa){𝑛} and .*a{0, 𝑚}a{𝑛}
due to the inherent limitations of the automata model and the determinization algorithm that it
uses. CA provides consistent speed for regexes with large bounds in counting repetition. However,
the matching of regexes that contain many occurrences of bounded repetition may cause a timeout.
Several timeouts are reported for PCRE due to catastrophic backtracking. BVA-Scan always performs
regex matching within 10 seconds, even for those regexes with huge repetition bounds.
Fig. 12 reports the running time (in milliseconds) for matching regexes (taken from 6 different
application benchmarks) over a randomly generated input string. Each plot of Fig. 12 is a performance comparison between BVA-Scan and another engine. The plots are organized in a grid with
3 × 6 = 18 plots: each column of the grid corresponds to a different regex engine (CA, PRCRE, or
RE2), and each row corresponds to a different application benchmark. Each plot in the column for
engine 𝐸 contains points for the regexes that can be handled by both BVA-Scan and 𝐸. For each
point (𝑡, 𝑡 ′) the horizontal coordinate 𝑡 (resp., the vertical coordinate 𝑡 ′) is the matching time of
BVA-Scan (resp., engine 𝐸). So, if a point is below the diagonal, then this means that BVA-Scan is
faster than engine 𝐸. We calculate the percentage of regexes for which BVA-Scan is faster than 𝐸 and
present this information on each plot (see, e.g., “wins: 100.00%”). The results show that BVA-Scan is
consistently faster than CA (100% cases), PCRE (≥ 96.72%), and RE2 (≥ 99.57%) for the vast majority
of regexes in the benchmarks. Fig. 12 also reports the percentage of regexes for which there is no
significant difference between BVA-Scan and the engine it is compared against. This information is
relevant, because measurements of running time have some inherent uncertainty. The highlighted
area in each plot (narrow band around the diagonal) contains those points for which the matching
time of BVA-Scan is between 0.5× and 2× the matching time of the other engine. In general, the
percentage of these regexes is below 5%.
Fig. 13 reports the matching time of regexes when the input text is produced with the adversarial
generation method. The results show that BVA-Scan is faster than CA (≥ 99.81%), PCRE (≥ 99.35%),
and RE2 (≥ 99.56%), and the ratio of regexes that are close to the diagonal is below 2% in general.
Table 2 (resp., Table 3) reports the average speedup of BVA-Scan over the other engines for
random (resp., adversarial) input. For matching over random input, the average speedup ranges
from 125× to 627× for CA, 9× to 164× for PCRE, and 9× to 15× for RE2. For adversarial input, it
ranges from 46× to 341× for CA, 41× to 125× for PCRE, and 11× to 36× for RE2.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 24 -->

92:24

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

CA
1e+03

PCRE

RE2

wins: 100.00%
close: 0.00%

wins: 100.00%
close: 0.00%

wins: 100.00%
close: 0.00%

wins: 99.96%
close: 0.13%

wins: 99.96%
close: 0.09%

wins: 100.00%
close: 0.00%

wins: 96.72%
close: 2.84%

wins: 99.57%
close: 3.22%

wins: 100.00%
close: 0.00%

wins: 99.63%
close: 3.63%

wins: 99.81%
close: 0.95%

wins: 100.00%
close: 0.00%

wins: 99.31%
close: 4.01%

wins: 99.75%
close: 1.09%

wins: 100.00%
close: 0.00%

wins: 99.49%
close: 4.73%

wins: 99.80%
close: 1.30%

ClamAV

wins: 100.00%
close: 0.00%

1e+00

1e-03

1e+03

Protomata

1e+00

1e-03

1e+03

RegexLib

1e-03

1e+03

Snort

time (ms) in BVA-Scan

1e+00

1e+00

1e-03

SpamAssassin

1e+03

1e+00

1e-03

1e+03

Suricata

1e+00

1e-03

1e-03

1e+00

1e+03

1e-03

1e+00

1e+03

1e-03

1e+00

1e+03

time (ms) in other engines

Fig. 12. Matching time (ms) of RE2, CA, PCRE and BVA-Scan over regexes collected from real-world applications for random input. The timeout is set to 10 seconds. The grey area contains points for which the
matching time of BVA-Scan is between 0.5× and 2× the matching time of the other engine.

Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 25 -->

Regular Expression Matching using Bit Vector Automata

CA
1e+03

92:25

PCRE

RE2

wins: 99.96%
close: 0.17%

wins: 99.96%
close: 0.16%

wins: 100.00%
close: 0.00%

wins: 100.00%
close: 0.00%

wins: 100.00%
close: 0.09%

wins: 100.00%
close: 0.37%

wins: 99.35%
close: 0.43%

wins: 100.00%
close: 0.43%

wins: 99.95%
close: 0.03%

wins: 99.98%
close: 0.23%

wins: 99.56%
close: 1.51%

wins: 99.81%
close: 0.42%

wins: 100.00%
close: 0.11%

wins: 99.64%
close: 0.82%

wins: 100.00%
close: 0.04%

wins: 99.97%
close: 0.23%

wins: 99.69%
close: 1.32%

ClamAV

wins: 100.00%
close: 0.00%

1e+00

1e-03

1e+03

Protomata

1e+00

1e-03

1e+03

RegexLib

1e-03

1e+03

Snort

time (ms) in BVA-Scan

1e+00

1e+00

1e-03

SpamAssassin

1e+03

1e+00

1e-03

1e+03

Suricata

1e+00

1e-03

1e-03

1e+00

1e+03

1e-03

1e+00

1e+03

1e-03

1e+00

1e+03

time (ms) in other engines

Fig. 13. Matching time (ms) of RE2, CA, PCRE and BVA-Scan over regexes collected from real-world applications for adversarial input. The timeout is set to 10 seconds. The grey area contains points for which the
matching time of BVA-Scan is between 0.5× and 2× the matching time of the other engine.

Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 26 -->

92:26

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

Table 2. Average speedup of BVA-Scan versus CA, RE2, PCRE over different benchmarks for random input

CA
PCRE
RE2

ClamAV
627
9
15

Protomata
125
18
11

RegexLib
142
23
9

Snort
400
164
15

SpamAssassin
217
129
10

Suricata
394
118
13

Table 3. Average speedup of BVA-Scan versus CA, RE2, PCRE over different benchmarks for adversarial input

CA
PCRE
RE2

ClamAV
160
41
22

Protomata
341
53
36

RegexLib
109
125
21

Snort
62
59
13

SpamAssassin
46
62
11

Suricata
68
54
14

5.2.3 Benchmarking Difficult Regexes. We have compiled a list of challenging regexes from our
application benchmarks in order to evaluate the benefit of the algorithms presented in Section 4. A
challenging regex typically involves a large amount of counting nondeterminism. This means that,
during matching over adversarial input, the configurations of the NCA constructed from the regex
are likely to contain a large number of different tokens. Moreover, the size of the DFA for such
a regex is usually very large compared to the size of the corresponding NFA. These challenging
regexes are usually of a form similar to the ones used in the microbenchmarks (e.g., .*a.{100}c).
We measured the average speedup of BVA-Scan against PCRE and RE2 when matching these
challenging regexes over adversarial input. For all these regexes, our algorithms from Section 4 are
instrumental in reducing the matching time compared to other approaches. Our results show that
BVA-Scan offers an average speedup of 151× against PCRE (up to 3660× for large repetition bounds)
and of 71× against RE2 (up to 844×), which is considerably larger than the average speedup for the
entire dataset (shown in Table 3).
6 RELATED WORK
There is a large number of prior works on software regex matchers that are based on finite-state
automata. Most regex engines use NFAs, DFAs, or some combination of NFAs and DFAs (e.g., RE2
and Hyperscan). PCRE2 [Hazel and Herczeg 2022] is based on backtracking search, but also provides
the function pcre2_dfa_match() that matches using a DFA. RE2 [RE2 2023] performs an NFA-to-DFA
on-the-fly determinization by caching visited states, and switches to an NFA simulation if there
are too many DFA states. PCRE2, as opposed to other high-performance regex engines, supports
backreferences at the cost of using a backtracking algorithm whose worst-case complexity is
exponential in the size of the input. SRM [Saarikivi et al. 2019] is based on Brzozowski derivatives
[Brzozowski 1964] (therefore it is similar to DFA-based approaches) extended to support bounded
repetition and match extraction.
To speed up matching on large inputs, regex engines often perform pre-filtering by extracting
simple string sub-patterns from the regex and applying string matching algorithms. There are
several efficient algorithms for string matching [Aho and Corasick 1975; Boyer and Moore 1977;
Commentz-Walter 1979; Karp and Rabin 1987; Knuth et al. 1977]. The popular network intrusion
detection system Snort [Snort 2023] uses the Aho-Corasick algorithm [Aho and Corasick 1975].
Grep [Grep 2022] skips parts of the input that cannot match the regex using the Boyer-Moore
algorithm [Boyer and Moore 1977]. Hyperscan [Wang et al. 2019] decomposes a regular expression
into a collection of strings and other more complex sub-patterns. Hyperscan uses a variant of
the shift-or algorithm [Baeza-Yates and Gonnet 1992] to match strings. String matching is further
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 27 -->

Regular Expression Matching using Bit Vector Automata

92:27

optimized in Hyperscan with a 128-bit SIMD implementation of the shift-or algorithm. Our engine,
BVA-Scan, decomposes regexes and uses specialized algorithms (including shift-or and shift-and)
for the efficient matching of strings and other simple patterns.
Some regex engines (e.g., RE2) handle bounded repetitions by unfolding them. For example,
the regular expression 𝑟 {𝑘 } is rewritten into the 𝑘-fold concatenation 𝑟 · 𝑟 · · · 𝑟 . The unfolding of
bounded repetitions can lead to an exponential blowup in size. It is therefore not surprising that RE2
does not handle regexes of the form 𝑟 {𝑘 } for 𝑘 > 1000. Various kinds of automata with counters
were proposed in prior works in order to effectively deal with regexes that contain bounded
repetitions (i.e., counting). [Smith et al. 2008] introduce an automaton called XFA, which extends
traditional DFA with counters to reduce the memory needed to represent regular expressions with
bounded repetitions. Similarly, [Becchi and Crowley 2008] propose an extension to NFA called
counting-NFA. [Holík et al. 2019] consider the determinization of a class of counter automata.
Their determinization produces smaller automata than the standard determinization into DFAs.
[Turoňová et al. 2020] extend the work of [Holík et al. 2019] and present the CA regex engine
that handles bounded repetition with a specialized algorithm. They use a class of automata with
counters (called “counting automata” or CAs), which are nondeterministic, to represent patterns.
CAs are converted into deterministic “counting-set automata” (CsAs). Unfortunately, the CA-to-CsA
determinization does not always preserve the semantics, i.e., the language that is recognized may
change. Using CsAs, they describe an algorithm that can deal with some cases of bounded repetition
efficiently. One problem with the approach of [Turoňová et al. 2020] is that the automata that
underlie the matching algorithm sometimes over-approximate the language of the pattern, which
can result in incorrect output (a “false positive”, i.e., a false match).
More recently, [Kong et al. 2022] have designed an in-memory hardware architecture that
incorporates counters and bit vector modules into the hardware design, thus enabling substantial
area and energy savings compared to Micron’s Automata Processor [Dlugosch et al. 2014]. One of the
key contributions in [Kong et al. 2022] is a static analysis for regexes that enables the identification
of bounded repetitions 𝑟 {𝑚, 𝑛} that are easy, in the sense that they can be implemented with a small
amount of memory using a single counter of size Θ(log 𝑛). This static analysis uses nondeterministic
counter automata (NCAs) with bounded counters to represent the patterns.
7 CONCLUSION
We have considered the problem of how to efficiently match regular expressions with bounded
repetition. Prior work addresses this problem for a more limited class of regexes. We expand this
class by considering bounded repetitions of the form (𝜎0𝜎1 . . . 𝜎𝑘−1 ){𝑚, 𝑛}, where each 𝜎𝑖 is a
predicate over the alphabet (character class). Then, we continue to further expand to bounded
repetitions of the form 𝑟 {𝑚, 𝑛}, where 𝑟 is a regex (without bounded repetition) that accepts strings
of some fixed length. The vast majority of challenging bounded repetitions that occur in practice fall
within the class that we consider and we can therefore handle them efficiently. We have implemented
the BVA-Scan regex engine that integrates our efficient algorithms. Our experimental results show
that BVA-Scan consistently outperforms RE2, PCRE, and CA on the application benchmarks Snort,
Suricata, SpamAssassin, Protomata, ClamAV and RegexLib. As a direction for future work, we plan
to integrate the static analysis of [Kong et al. 2022] into BVA-Scan in order to reduce the memory
used for handling bounded repetition.
ACKNOWLEDGMENTS
We would like to thank the anonymous reviewers for their constructive comments. This research
was supported in part by the US National Science Foundation award CCF 2008096 and the Rice
University Faculty Initiative Fund.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 28 -->

92:28

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

REFERENCES
Alfred V. Aho and Margaret J. Corasick. 1975. Efficient String Matching: An Aid to Bibliographic Search. Commun. ACM 18,
6 (1975), 333–340. https://doi.org/10.1145/360825.360855
Valentin Antimirov. 1996. Partial Derivatives of Regular Expressions and Finite Automaton Constructions. Theoretical
Computer Science 155, 2 (1996), 291–319. https://doi.org/10.1016/0304-3975(95)00182-4
GNU Awk. 2022. GNU Awk. https://www.gnu.org/software/gawk/ Accessed: March 11, 2023.
Backreferences. 2022. Back Reference in PCRE. https://www.pcre.org/original/doc/html/pcrepattern.html#SEC19 Accessed:
March 11, 2023.
Ricardo Baeza-Yates and Gaston H. Gonnet. 1992. A New Approach to Text Searching. Commun. ACM 35, 10 (1992), 74–82.
https://doi.org/10.1145/135239.135243
Howard Barringer, Allen Goldberg, Klaus Havelund, and Koushik Sen. 2004. Rule-Based Runtime Verification. In VMCAI
2004 (LNCS), Vol. 2937. Springer, Heidelberg, 44–57. https://doi.org/10.1007/978-3-540-24622-0_5
Ezio Bartocci, Jyotirmoy Deshmukh, Alexandre Donzé, Georgios Fainekos, Oded Maler, Dejan Ničković, and Sriram
Sankaranarayanan. 2018. Specification-Based Monitoring of Cyber-Physical Systems: A Survey on Theory, Tools and
Applications. In Lectures on Runtime Verification: Introductory and Advanced Topics, Ezio Bartocci and Yliès Falcone (Eds.).
LNCS, Vol. 10457. Springer, Cham, 135–175. https://doi.org/10.1007/978-3-319-75632-5_5
Michela Becchi and Patrick Crowley. 2008. Extending Finite Automata to Efficiently Match Perl-Compatible Regular
Expressions. In Proceedings of the 2008 ACM CoNEXT Conference (CoNEXT ’08). ACM, New York, NY, USA, Article 25,
12 pages. https://doi.org/10.1145/1544012.1544037
Joao Bispo, Ioannis Sourdis, Joao M. P. Cardoso, and Stamatis Vassiliadis. 2006. Regular Expression Matching for Reconfigurable Packet Inspection. In 2006 IEEE International Conference on Field Programmable Technology. IEEE, USA, 119–126.
https://doi.org/10.1109/FPT.2006.270302
Chunkun Bo, Vinh Dang, Elaheh Sadredini, and Kevin Skadron. 2018. Searching for Potential gRNA Off-Target Sites for
CRISPR/Cas9 Using Automata Processing Across Different Platforms. In 2018 IEEE International Symposium on High
Performance Computer Architecture (HPCA). IEEE, USA, 737–748. https://doi.org/10.1109/HPCA.2018.00068
Robert S. Boyer and J. Strother Moore. 1977. A Fast String Searching Algorithm. Commun. ACM 20, 10 (1977), 762–772.
https://doi.org/10.1145/359842.359859
Benjamin C. Brodie, David E. Taylor, and Ron K. Cytron. 2006. A Scalable Architecture For High-Throughput RegularExpression Pattern Matching. In Proceedings of the 33rd Annual International Symposium on Computer Architecture (ISCA
’06). IEEE Computer Society, USA, 191–202. https://doi.org/10.1109/ISCA.2006.7
Janusz A. Brzozowski. 1964. Derivatives of Regular Expressions. J. ACM 11, 4 (1964), 481–494. https://doi.org/10.1145/
321239.321249
Agnishom Chattopadhyay and Konstantinos Mamouras. 2020. A Verified Online Monitor for Metric Temporal Logic with
Quantitative Semantics. In RV 2020 (LNCS), Jyotirmoy Deshmukh and Dejan Ničković (Eds.), Vol. 12399. Springer, Cham,
383–403. https://doi.org/10.1007/978-3-030-60508-7_21
ClamAV. 2023. ClamAV - Open Source Antivirus Engine. Website. https://www.clamav.net/ Accessed: March 11, 2023.
Beate Commentz-Walter. 1979. A String Matching Algorithm Fast on the Average. In ICALP 1979 (LNCS), Hermann A.
Maurer (Ed.), Vol. 71. Springer, Berlin, Heidelberg, 118–132. https://doi.org/10.1007/3-540-09510-1_10
James C. Davis. 2019. Rethinking Regex Engines to Address ReDoS. In Proceedings of the 2019 27th ACM Joint Meeting on
European Software Engineering Conference and Symposium on the Foundations of Software Engineering (ESEC/FSE 2019).
ACM, New York, NY, USA, 1256–1258. https://doi.org/10.1145/3338906.3342509
Paul Dlugosch, Dave Brown, Paul Glendenning, Michael Leventhal, and Harold Noyes. 2014. An Efficient and Scalable
Semiconductor Architecture for Parallel Automata Processing. IEEE Transactions on Parallel and Distributed Systems 25,
12 (2014), 3088–3098. https://doi.org/10.1109/TPDS.2014.8
Wouter Gelade, Marc Gyssens, and Wim Martens. 2009. Regular Expressions with Counting: Weak versus Strong Determinism. In MFCS 2009 (LNCS), Rastislav Královič and Damian Niwiński (Eds.), Vol. 5734. Springer, Berlin, Heidelberg,
369–381. https://doi.org/10.1007/978-3-642-03816-7_32
Victor Mikhaylovich Glushkov. 1961. The Abstract Theory of Automata. Russian Mathematical Surveys 16, 5 (1961), 1–53.
https://doi.org/10.1070/RM1961v016n05ABEH004112
Jan Goyvaerts. 2021. Runaway Regular Expressions: Catastrophic Backtracking. https://www.regular-expressions.info/
catastrophic.html accessed March 11, 2023.
GNU Grep. 2022. GNU Grep - Global Regular Expression Print. https://www.gnu.org/software/grep/ Accessed: March 11,
2023.
Philip Hazel and Zoltan Herczeg. 2022. PCRE2 - Perl Compatible Regular Expressions v2. https://www.pcre.org/ Accessed:
March 11, 2023.
Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Lenka Turoňová, Margus Veanes, and Tomáš Vojnar. 2019. Succinct Determinisation of Counting Automata via Sphere Construction. In APLAS 2019 (LNCS), Anthony Widjaja Lin (Ed.), Vol. 11893.
Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 29 -->

Regular Expression Matching using Bit Vector Automata

92:29

Springer, Cham, 468–489. https://doi.org/10.1007/978-3-030-34175-6_24
Dag Hovland. 2009. Regular Expressions with Numerical Constraints and Automata with Counters. In ICTAC 2009 (LNCS),
Martin Leucker and Carroll Morgan (Eds.), Vol. 5684. Springer, Berlin, Heidelberg, 231–245. https://doi.org/10.1007/9783-642-03466-4_15
Posix Syntax in PCRE. 2022. Posix Syntax in PCRE. https://www.pcre.org/original/doc/html/pcrepattern.html Accessed:
March 11, 2023.
Richard M. Karp and Michael O. Rabin. 1987. Efficient Randomized Pattern-Matching Algorithms. IBM Journal of Research
and Development 31, 2 (1987), 249–260. https://doi.org/10.1147/rd.312.0249
Donald E. Knuth, James H. Morris, Jr., and Vaughan R. Pratt. 1977. Fast Pattern Matching in Strings. SIAM J. Comput. 6, 2
(1977), 323–350. https://doi.org/10.1137/0206024
Lingkun Kong, Qixuan Yu, Agnishom Chattopadhyay, Alexis Le Glaunec, Yi Huang, Konstantinos Mamouras, and Kaiyuan
Yang. 2022. Software-Hardware Codesign for Efficient In-Memory Regular Pattern Matching. In Proceedings of the 43rd
ACM SIGPLAN International Conference on Programming Language Design and Implementation (PLDI 2022). ACM, New
York, NY, USA, 733–748. https://doi.org/10.1145/3519939.3523456
CsA Automata Library. 2021. CsA Automata Library. https://pajda.fit.vutbr.cz/ituronova/countingautomata
Konstantinos Mamouras, Agnishom Chattopadhyay, and Zhifu Wang. 2021a. Algebraic Quantitative Semantics for Efficient
Online Temporal Monitoring. In TACAS 2021 (LNCS), Jan Friso Groote and Kim Guldstrand Larsen (Eds.), Vol. 12651.
Springer, Cham, 330–348. https://doi.org/10.1007/978-3-030-72016-2_18
Konstantinos Mamouras, Agnishom Chattopadhyay, and Zhifu Wang. 2021b. A Compositional Framework for Quantitative
Online Monitoring over Continuous-Time Signals. In RV 2021 (LNCS), Lu Feng and Dana Fisman (Eds.), Vol. 12974.
Springer, Cham, 142–163. https://doi.org/10.1007/978-3-030-88494-9_8
Konstantinos Mamouras and Zhifu Wang. 2020. Online Signal Monitoring with Bounded Lag. IEEE Transactions on ComputerAided Design of Integrated Circuits and Systems 39, 11 (2020), 3868–3880. https://doi.org/10.1109/TCAD.2020.3013053
Albert R. Meyer and Michael J. Fischer. 1971. Economy of Description by Automata, Grammars, and Formal Systems. In
2013 IEEE 54th Annual Symposium on Foundations of Computer Science. IEEE Computer Society, Los Alamitos, CA, USA,
188–191. https://doi.org/10.1109/SWAT.1971.11
Albert R. Meyer and Larry J. Stockmeyer. 1972. The Equivalence Problem for Regular Expressions with Squaring Requires
Exponential Space. In 13th Annual Symposium on Switching and Automata Theory (SWAT 1972). IEEE Computer Society,
Los Alamitos, CA, USA, 125–129. https://doi.org/10.1109/SWAT.1972.29
RE2. 2023. RE2: Google’s regular expression library. Website. https://github.com/google/re2 Accessed: March 11, 2023.
RegexLib. 2023. Regular Expression Library. https://regexlib.com/ Accessed: March 11, 2023.
Martin Roesch. 1999. Snort - Lightweight Intrusion Detection for Networks. In Proceedings of the 13th USENIX Conference
on System Administration (Seattle, Washington) (LISA ’99). USENIX Association, USA, 229–238. https://www.usenix.org/
legacy/publications/library/proceedings/lisa99/full_papers/roesch/roesch.pdf
Indranil Roy and Srinivas Aluru. 2016. Discovering Motifs in Biological Sequences Using the Micron Automata Processor.
IEEE/ACM Transactions on Computational Biology and Bioinformatics 13, 1 (2016), 99–111. https://doi.org/10.1109/TCBB.
2015.2430313
Olli Saarikivi, Margus Veanes, Tiki Wan, and Eric Xu. 2019. Symbolic Regex Matcher. In TACAS 2019 (LNCS), Vol. 11427.
Springer, Cham, 372–378. https://doi.org/10.1007/978-3-030-17462-0_24
Christian J. A. Sigrist, Lorenzo Cerutti, Edouard de Castro, Petra S. Langendijk-Genevaux, Virginie Bulliard, Amos Bairoch,
and Nicolas Hulo. 2009. PROSITE, A Protein Domain Database for Functional Characterization and Annotation. Nucleic
Acids Research 38, suppl_1 (2009), D161–D166. https://doi.org/10.1093/nar/gkp885
Randy Smith, Cristian Estan, and Somesh Jha. 2008. XFA: Faster Signature Matching with Extended Automata. In Proceedings
of the 2008 IEEE Symposium on Security and Privacy (SP ’08). IEEE Computer Society, USA, 187–201. https://doi.org/10.
1109/SP.2008.14
Snort. 2023. Snort - Network Intrusion Detection & Prevention System. https://www.snort.org/ Accessed: March 11, 2023.
Apache SpamAssassin. 2022. Apache SpamAssassin. https://spamassassin.apache.org/ Accessed: March 11, 2023.
Larry J. Stockmeyer and Albert R. Meyer. 1973. Word Problems Requiring Exponential Time (Preliminary Report). In
Proceedings of the Fifth Annual ACM Symposium on Theory of Computing (STOC ’73). ACM, New York, NY, USA, 1–9.
https://doi.org/10.1145/800125.804029
Suricata. 2023. Suricata - Open Source Intrusion Detection and Prevention Engine. https://suricata.io/ Accessed: March 11,
2023.
Ken Thompson. 1968. Programming Techniques: Regular Expression Search Algorithm. Commun. ACM 11, 6 (1968), 419–422.
https://doi.org/10.1145/363347.363387
Lenka Turoňová, Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Margus Veanes, and Tomáš Vojnar. 2020. Regex Matching with
Counting-Set Automata. Proceedings of the ACM on Programming Languages 4, OOPSLA, Article 218 (2020), 30 pages.
https://doi.org/10.1145/3428286

Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.

<!-- page 30 -->

92:30

Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras

Xiang Wang, Yang Hong, Harry Chang, KyoungSoo Park, Geoff Langdale, Jiayu Hu, and Heqing Zhu. 2019. Hyperscan:
A Fast Multi-Pattern Regex Matcher for Modern CPUs. In 16th USENIX Symposium on Networked Systems Design and
Implementation (NSDI ’19). USENIX Association, Boston, MA, 631–648. https://www.usenix.org/conference/nsdi19/
presentation/wang-xiang
Fang Yu, Zhifeng Chen, Yanlei Diao, T. V. Lakshman, and Randy H. Katz. 2006. Fast and Memory-Efficient Regular Expression
Matching for Deep Packet Inspection. In Proceedings of the 2006 ACM/IEEE Symposium on Architecture for Networking
and Communications Systems (ANCS ’06). ACM, New York, NY, USA, 93–102. https://doi.org/10.1145/1185347.1185360

Received 2022-10-28; accepted 2023-02-25

Proc. ACM Program. Lang., Vol. 7, No. OOPSLA1, Article 92. Publication date: April 2023.
