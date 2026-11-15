---
file: ../pdfs/2020-oopsla-turonova-counting-set-automata.pdf
citation: "Regex Matching with Counting-Set Automata — Turoňová (Holíková), Holík, Lengál, Saarikivi, Veanes, Vojnar — OOPSLA (PACMPL 4) 2020"
doi: 10.1145/3428286
source: "https://doi.org/10.1145/3428286 (publisher version, added manually)"
topics: [counting_and_derivatives, performance_hardware]
pages: 30
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Regex Matching with Counting-Set Automata
LENKA TUROŇOVÁ∗ , Brno University of Technology, Czech Republic
LUKÁŠ HOLÍK, Brno University of Technology, Czech Republic
ONDŘEJ LENGÁL, Brno University of Technology, Czech Republic
OLLI SAARIKIVI, Microsoft, USA
MARGUS VEANES, Microsoft, USA
TOMÁŠ VOJNAR, Brno University of Technology, Czech Republic
We propose a solution to the problem of e�cient matching regular expressions (regexes) with bounded
repetition, such as (ab){1,100}, using deterministic automata. For this, we introduce novel counting-set
automata (CsAs), automata with registers that can hold sets of bounded integers and can be manipulated by
a limited portfolio of constant-time operations. We present an algorithm that compiles a large sub-class of
regexes to deterministic CsAs. This includes (1) a novel Antimirov-style translation of regexes with counting
to counting automata (CAs), nondeterministic automata with bounded counters, and (2) our main technical
contribution, a determinization of CAs that outputs CsAs. The main advantage of this work�ow is that the
size of the produced CsAs does not depend on the repetition bounds used in the regex (while the size of
the DFA is exponential to them). Our experimental results con�rm that deterministic CsAs produced from
practical regexes with repetition are indeed vastly smaller than the corresponding DFAs. More importantly,
our prototype matcher based on CsA simulation handles practical regexes with repetition regardless of sizes
of counter bounds. It easily copes with regexes with repetition where state-of-the-art matchers struggle.
CCS Concepts: • Theory of computation ! Regular languages; Quantitative automata; • Security and
privacy ! Denial-of-service attacks; • Applied computing ! Document searching.
Additional Key Words and Phrases: regular expression matching, bounded repetition, ReDos, determinization,
Antimirov’s derivatives, counting automata, counting-set automata
ACM Reference Format:
Lenka Turoňová, Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Margus Veanes, and Tomáš Vojnar. 2020. Regex
Matching with Counting-Set Automata. Proc. ACM Program. Lang. 4, OOPSLA, Article 218 (November 2020),
30 pages. https://doi.org/10.1145/3428286

1 INTRODUCTION
Matching regexes (regular expressions) is a ubiquitous component of software, used, e.g., for
searching, data validation, parsing, �nding and replacing, data scraping, or syntax highlighting. It
∗ The main part of the work was done when the �rst author was a summer intern at Microsoft Research in Redmond in 2019.

Authors’ addresses: Lenka Turoňová, Faculty of Information Technology, Brno University of Technology, Božetěchova 2,
Brno, 612 00, Czech Republic, ituronova@�t.vutbr.cz; Lukáš Holík, Faculty of Information Technology, Brno University
of Technology, Božetěchova 2, Brno, 612 00, Czech Republic, holik@�t.vutbr.cz; Ondřej Lengál, Faculty of Information
Technology, Brno University of Technology, Božetěchova 2, Brno, 612 00, Czech Republic, lengal@�t.vutbr.cz; Olli Saarikivi,
MSR, Microsoft, One Microsoft Way, Redmond, 98052, USA, Olli.Saarikivi@microsoft.com; Margus Veanes, MSR, Microsoft,
One Microsoft Way, Redmond, 98052, USA, margus@microsoft.com; Tomáš Vojnar, Faculty of Information Technology,
Brno University of Technology, Božetěchova 2, Brno, 612 00, Czech Republic, vojnar@�t.vutbr.cz.

This work is licensed under a Creative Commons Attribution 4.0 International License.
© 2020 Copyright held by the owner/author(s).
2475-1421/2020/11-ART218
https://doi.org/10.1145/3428286
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

218

<!-- page 2 -->

218:2

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

is commonly used and natively supported in most programming languages [contributors 2019]. For
instance, about 30–40 % of Java, JavaScript, and Python software use regex matching (as reported
in multiple studies, see, e.g., [Davis 2019]).
The e�ciency of regex matching engines has a signi�cant impact on the overall usability of
software applications. Unpredictability of a matcher’s performance may lead to catastrophic consequences, witnessed by events such as the recent catastrophic outage of Cloud�are services [GrahamCumming 2019], caused by a single poorly written regex, and it is a doorway for the so-called ReDoS
attack, a denial of service attack based on overwhelming a regex matching engine by providing a
specially crafted regex or text. For instance, in 2016, ReDoS caused an outage of StackOver�ow [Exchange 2016] or rendered vulnerable websites that used the popular Express.js framework [Baldwin
2016]. Works such as [Davis 2019; Davis et al. 2018] give arguments that ReDoS is not just a niche
problem but rather a common and serious threat.
Failures of matching are mostly caused by the so-called “catastrophic backtracking”, a situation
when variants of Spencer’s simulation of a nondeterministic �nite automaton (NFA) by backtracking [Spencer 1994] exhibit a behaviour super-linear to the length of the text. Matching algorithms
based on backtracking are probably the most often implemented ones, their performance is, however,
at worst exponential to the text length. An alternative with a much lower worst-case complexity (wrt
the length of the text) is to use deterministic �nite automata (DFAs). In the ideal case, the DFA is precomputed; matching can then be linear to the text length, with each input symbol processed in constant time. This is the so-called static DFA simulation [Sipser 2006]. The major drawback of static DFA
simulation is that the DFA construction may explode, rendering the method unusable in practice.
Variants of Thompson’s algorithm [Thompson 1968] (sometimes called NFA simulation or NFA-toDFA simulation) avoid the explosion by working directly with the NFA. They essentially run the determinization by subset construction on the �y, always remembering only the current DFA state. On
reading a character, a successor DFA state is computed and used to replace the current state. The disadvantage of Thompson’s algorithm is that, for a highly nondeterministic NFA, the DFA states—sets
of the states of the NFA—may get large and computing a DFA-state successor over a symbol becomes
expensive, linear to the size of the NFA (compared to the constant time of static DFA simulation).
Modern matchers therefore use caching of already visited parts of the DFA. Making a step
within the cached part is then as fast as with the explicitly determinized automaton. Extremely
e�cient implementations of Thompson’s algorithm with caching are used in RE2 [Google [n.d.]]
and GNU grep [Haertel et al. [n.d.]]. Their close cousin, an on-the-�y Brzozowski’s derivative
construction, is implemented in the tool SRM [Saarikivi et al. 2019]. Highly nondeterministic
regexes1 that lead to exploding determinization are, however, problematic for all variants, explicit
determinization as well as NFA simulation, with or without caching.
In this paper, we focus on eliminating a frequent cause of a DFA explosion—a use of the counting
operator, also known as the operator of bounded repetition. It succinctly expresses repeated patterns
such as (ab){1,100}, representing 1 to 100 consecutive repetitions of ab. Such expressions are
very common (cf. [Björklund et al. 2015]), e.g., in the RegExLib library [RegExLib.com [n.d.]], which
collects expressions for recognizing URIs, markup code, pieces of Java code, or SQL queries; in
the Snort rules [M. Roesch et al. [n.d.]] used for detecting attacks in network tra�c; in real-life
XML schemas, with the counter bounds being as large as 10 million [Björklund et al. 2015]; or in
detecting information leakage from tra�c logs [Holík et al. 2019].
1 Loosely speaking, a “highly nondeterministic regex” is one for which the determinization of the NFA created by some of

the usual algorithms explodes. Determinism of regexes closely corresponds to the notion of 1-unambiguity of the regex
[Brüggemann-Klein and Wood 1998; Hovland 2009]: when matching a text from left to right against the regex, it is always
clear which letter of the regex matches the text character.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 3 -->

Regex Matching with Counting-Set Automata

218:3

To illustrate the principal di�culty with matching bounded repetitions, especially when combined
with a high degree of nondeterminism, consider the regex *a {k} where k 2 N (the regex denotes
strings where the symbol a appears k positions from the end of the word). Already the NFA will
have at least k states, which is exponential to the regex size because k is written in decimal. Due
to the inherent nondeterminism of this regex, determinization then adds a second level of the
exponential explosion. Indeed, the minimal DFA accepting the language has 2k +1 states because it
must remember all the positions where the symbol a was seen during the last k+1 steps. This requires
a �nite memory of k + 1 bits and thus 2k +1 reachable DFA states. Determinizing the NFA explicitly
is thus out of question for even moderate values of k. The pure Thompson’s NFA simulation is
feasible but very slow, as reading each character may in the worst case require processing the entire
NFA. Moreover, caching of the DFA state space, used in industrial matchers like RE2 [Google [n.d.]]
or GNU grep [Haertel et al. [n.d.]], may also be ine�ective due to the size of the state space and
low cache utilization. At the same time, combinations of nondeterminism and counting are fairly
common. A high degree of nondeterminism is, for instance, usual when searching for a pattern
“anywhere on the line” (corresponding to pre�xing the pattern with *), which is the standard
behaviour for GNU grep and similar programs when start/end of line anchors are not used.
To facilitate e�cient matching of such nondeterministic counting, we propose a translation from
regexes with repetition to deterministic machines that are succinct and can perform matching with
nearly constant character complexity. The novel succinct and fast deterministic machine, called
the counting-set automaton (CsA), is the key to the result. It is a deterministic �nite automaton
with a special type of registers that can hold values called counting sets—a set of bounded integer
values—and support a limited selection of simple set operations. Crucially for the e�ciency of our
approach, we show that, using a suitable data structure, all the set operations can be implemented
to run in constant time regardless of the size of the set.
Our compilation from regexes to CsAs proceeds in two steps. First, we compile the regexes
into nondeterministic counting automata (CAs), automata with counters whose values are a priori
bounded. Variants of CAs have been used in several other works under di�erent names, e.g., [Björklund et al. 2015; Gelade et al. 2012; Holík et al. 2019; Hovland 2009; Kilpeläinen and Tuhkanen
2007; Smith et al. 2008b; Sperberg-McQueen [n.d.]]. The compilation from regexes is cheap and
produces automata whose size is independent of the counter bounds and linear in the size of the
regex. We present a novel translation of regexes to CAs that generalizes the Antimirov’s derivative
construction [Antimirov 1996]. Our translation has several advantages over the existing alternatives,
such as absence of -transitions in the output CA and succinctness. The result of translating the
regex *a {k} into a CA is illustrated in Fig. 1a.
The main step forward we make in this paper is a solution of e�cient matching for a large class
of highly nondeterministic regexes with counting that are quite common in practice. The main
technical problem we have solved is a succinct transformation of a (nondeterministic) CA into
a deterministic CsA. Our algorithm produces a CsA in time independent of the counter bounds. We
note that this has been a known open problem (emphasized, e.g., in [Sperberg-McQueen [n.d.]]).
Works on matching of bounded repetition such as [Björklund et al. 2015; Gelade et al. 2012; Holík
et al. 2019; Hovland 2009; Kilpeläinen and Tuhkanen 2003; Kilpeläinen and Tuhkanen 2007; Smith
et al. 2008a] mostly focus on deterministic regexes and do not propose practical solutions for the
nondeterministic case. We have carried out an extensive experimental evaluation of our algorithm
on a large sample of regexes used for pattern matching in various applications. The experiments
show that our algorithm, although also limited to a sub-class of regexes, handles over 90 % of regexes
with counting we collected. The obtained data con�rm that our CsAs are indeed far smaller and can
be constructed faster than corresponding DFAs. Most importantly, we demonstrate the practical

..

.

..

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 4 -->

218:4

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

c := 0

.^ c < 100/c := c + 1
.
a
q : .*a.{100}
s : .{100}
{c

100}

[^a]
c := {0}

{q}

{q, s} {Max(c)
[^a] ^ Min(c)

..

(a) The CA for the regex *a {k}

a/c := {0} [ c + 1

a / c := {0}

100

100}

[^a] ^ Min(c) < 100/
c := c + 1

(b) The CsA from determinization of (a)

..

Fig. 1. The nondeterministic CA and the deterministic CsA for *a {100}. The transitions are labeled by
their guard, which gives the character class (in the standard POSIX regex notation, where, e.g., stands for
“any character”) and possibly restricts counter values, delimited by “/” from the counter update. If a counter
does not have the update specified, then the transition does not change its value. In (b), the notation c + 1
stands for the set of values obtained by incrementing each value in c and then removing values larger than
the upper bound 100 of the counter. The edges denoting initial states are labelled with initial values of the
counters. Final states are labelled with an acceptance condition, e.g. {c 100} in (a).
The formal counter operations ��c presented later in Section 4 are in (a) shown as follows: the guard of ��c is
shown in conjunction with the character guard on the le� of the “/”, the update of ��c is shown on the
right of “/” in the form of an assignment to c, where ����c appears as the right value c + 1, ���� as 0, ����1
as 1, and ���� is omi�ed.

.

relevance of our algorithm for pattern matching. We have implemented a matching algorithm
based on CsA simulation2 and compared it with several state-of-the-art matchers, namely, grep
[Haertel et al. [n.d.]], RE2 [Google [n.d.]], SRM [Saarikivi et al. 2019], and .NET [Microsoft 2020].
Our results show that problematic highly nondeterministic regexes with counting indeed appear
in practice and can also be easily crafted as a ReDoS attack, and that CsAs can e�ciently solve
most of such problematic cases. For instance, the regex (_a){64999}_a from [Davis et al. 2019] can
cause state-of-the-art matchers exceed any reasonable time limit (when searching for the pattern
anywhere on the line, with the implicit * in front). Already with the counter bounds lowered to
1,000, the matchers take from 6 to 34 seconds on 500 KiB of text, but our algorithm needs only 1
second even with the original bound 64,999.
We summarize the technical contributions of this work as follows:
(1) A novel Antimirov style regex-to-CA translation.
(2) A novel notion of the counting-set automaton, a deterministic machine that allows for
succinct representation of counting constraints and fast matching.
(3) CA-to-CsA determinization that runs in time independent of counter bounds.
(4) A regex-matching algorithm interconnecting the above, e�cient regardless of counter bounds
especially on regexes that combine counting with nondeterminism.
(5) Implementation and extensive experimental evaluation of the above.

.

2 OVERVIEW
We give a brief overview of our conversion of a regex with counting into a deterministic CsA. We use
the example regex R = *a {100}, discussed already in the introduction and representing strings
where the symbol a appears 100 positions from the end, with the corresponding minimum DFA having 2101 states. The conversion proceeds in two steps. First, R is translated into a nondeterministic
CA (Fig. 1a), denoted as CA(R); second, CA(R) is converted into a deterministic CsA (Fig. 1b). The
size of both is independent of the counter bounds (both of the automata will have 2 states only).

..

2 We use a pre-computed deterministic CsA. While on-the-�y determinization is also possible, it was not needed in our

experimentation since we have not witnessed problems with CsA state space explosion.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 5 -->

Regex Matching with Counting-Set Automata

218:5

Counting-Set Data Structure. Before looking into the conversion from regular expressions to
CsAs it is useful to �rst understand why the resulting CsA can be used e�ciently for matching
in the �rst place. The main enabler behind this is the use of our counting-set data structure, say
c, representing sets Sc ✓ {0, . . . , maxc } where the upper bound maxc is a �xed positive integer.
A runtime value of c is a tuple (o, `) where o 2 N is called an o�set and ` is a queue of strictly
increasing natural numbers such that Sc = {o n | n 2 `}.
The data structure supports constant-time implementations of the following operations, assuming
constant-time access to the �rst and the last element of the queue (the queue may be implemented
as a doubly-linked list).
• The minimum and the maximum of Sc are obtained as o last(`) and o �rst(`), respectively.
• Insert 0: if o last(`) > 0, then append o at the end of ` (similarly for inserting 1).
• Increment all, up to maxc : o := o + 1; if o �rst(`) > maxc , then remove �rst(`).
• Reset to {0}: ` := 0; o := 0 (similarly for reset to {1}).
The independence of the running time of these operations of maxc enables our major achievement:
the independence of the running time of pattern matching of the counter bounds.
Let us now illustrate how this data structure works during matching. We run the CsA in Fig. 1b, assuming the meaning of the operations provided above, over the sample input word aa0(10) aab(87) dfa.
The con�gurations of the au- pre�x
state (o, `)
Sc
tomaton after processing pre{q}
(0, [0])
{0}
�xes of the word are shown in
a
{q, s} (0, [0])
{0}
the table: the control state, the
aa
{q, s} (1, [0, 1])
{1, 0}
counting-set run-time con�gu- aa0(10)
{q, s} (11, [0, 1])
{11, 10}
{q, s} (13, [0, 1, 12, 13])
{13, 12, 1, 0}
ration (o, `), and the value Sc it aa0(10) aa
{q, s} (100, [0, 1, 12, 13]) {100, 99, 88, 87}
represents. The state {q, s} ful- aa0(10) aab(87)
{q, s} (101, [1, 12, 13])
{100, 89, 88}
�lls the accepting condition after aa0(10) aab(87) d
aa0(10) aab(87) df
{q, s} (102, [12, 13])
{90, 89}
processing the 6th and the 7th
aa0(10) aab(87) dfa {q, s} (103, [12, 13, 103]) {91, 90, 0}
pre�x since the maximum of Sc
at these points is indeed at least 100.
From a Nondeterministic CA to a Deterministic CsA. The idea of our CA-to-CsA determinization
is best explained by comparison with the naive determinization of a CA, which would create a DFA
by the explicit textbook-style subset construction. The states of the DFA would then be sets of
runtime con�gurations of the CA where each CA-con�guration would consist of a control state
and a counter valuation. Counter valuations would hence be “unfolded”—they would become an
explicit part of the DFA control states—and the succinctness provided by counters would be lost.
For instance, the run of the CA in Fig. 1a on the word aa0 . . . generates “powerstates”
{(q, c = 0)}, {(q, c = 0), (s, c = 0)}, {(q, c = 0), (s, c = 0), (s, c = 1)}, {(q, c = 0), (s, c = 1), (s, c = 2)}, . . .
which are essentially subsets of {q, s} ⇥ {0, . . . , 100}. In the worst case, the size of the DFA would
be exponential in counter bounds because s can be paired with any subset of {0, . . . , 100} recording
possible values of c. In contrast to this, as illustrated above, our CsA represents the counter
valuations implicitly: it computes them dynamically on the �y and stores them as counting sets—i.e.,
the valuation of a counter changes from an integer to a set of integers. The counter valuations are
hence not a part of control states, and their overall number in�uences neither the size of the CsA
nor the time needed to build them.
Fig. 1b shows the CsA obtained from determinization of the CA in Fig. 1a. The runtime con�gurations of the CsA reached for the word aa0 are
({q}, c 2 {0}), ({q, s}, c 2 {0}), ({q, s}, c 2 {0, 1}), ({q, s}, c 2 {1, 2}).
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 6 -->

218:6

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

They encode the �rst three states reached by the sample DFA run above. Namely, the control states
are kept in the �rst component and the counter values are in the second component, i.e., the set Sc
given by the run-time values of the counting-set c. In this encoding, the value of the counting-set is
not relevant for the states where the counter is never active (state q in our example). The counter’s
value in these states is always implicitly 0. In the example, the value Sc of the counting-set therefore
only records the values of c at state s and is thus relevant only in the CsA state {q, s}. We note
that for simplicity, we initialise all counting-sets uniformly with {0}, even if their value is initially
irrelevant, as in the case of the CsA state {q} in the example.
We note that some DFA powerstates cannot be encoded as CsA con�gurations due to the involved
Cartesian abstraction: essentially, any state in the powerset is paired with any counter value from
the counting set. Hence, our approach does not handle the full class of regexes. Fortunately, as our
empirical evidence shows, regexes that fall out from the supported class are rare in practice.
From Regexes to Nondeterministic CAs. To translate a regex into a CA, we propose a generalization
of the Antimirov’s derivative construction [Antimirov 1996] to symbolic counting. In Antimirov’s
setting, a derivative of a regex R wrt a character class is a set of regexes that together capture
all tails of words in L(R) whose head character is from . In particular, according to [Saarikivi
et al. 2019], which generalizes [Antimirov 1996] to explicit counting, the derivatives of the regex
R = *a {100} wrt the classes a and [^a] are {R, {100}} and {R}, respectively. Further, for 1  k 
100, the derivative of {k} wrt both a and [^a] is { {k 1}}. The derivatives become the states of the
resulting NFA, with R being initial and {0} �nal, and with -transitions from each regex to all its
-derivatives (for being either a or [^a]). The obtained NFA is already large, it has 102 states.
In our new construction, the counting is kept implicit using symbolic counters. Instead of modifying the counter bound of the derivative (by, e.g., deriving {99} from {100}), we keep the original
bound unchanged and use a counter c to keep track of the di�erence between the original value and
the current value. Our conditional derivative operator @α (·) then equips the produced derivatives
with conditional counter updates to keep the counters up-to-date. For instance, @a ( {100}) returns
the same regex {100}, but it is paired with conditional counter updates for c, namely, “if c < 100,
then increment c; and if c 100, then exit the counting loop”. The CA we obtain this way is shown
in Fig. 1a, where the �rst conditional update translates to the self loop on the state {100} and the
second to the acceptance condition. The size of the CA does not depend on the counter bounds.

..

.

.

.
.

.

.

.

.

.

3 PRELIMINARIES
We cast our de�nitions in the framework of symbolic automata [D’Antoni and Veanes 2020], a
natural succinct representations of �nite-state transition relations over large alphabets of labels.
Symbolic automata work over alphabets equipped with a so-called e�ective Boolean algebra, which
de�nes the needed interface for handling large sets of labels on automata transitions.
Before providing the formal de�nition of an e�ective Boolean algebra, we start with an intuitive
example, which is also going to be the alphabet algebra used throughout the paper, including the
experiments. Later on, we will further leverage the general de�nition to work with algebras over
counter and counting-set predicates.
Example 3.1. Regular expressions in practice use character classes as basic building blocks. To
simplify the discussion, let us restrict our attention to ASCII as the character universe D. In other
words, D is the set {n | 0  n < 27 } of all 7-bit characters represented using their character
codes. Then, for example, the character classes [0-9] and [A-Z] denote, respectively, the set
[[[0-9]]] = {n | 48  n  57} of all digit codes, and the set [[[A-Z]]] = {n | 65  n  90} of
all upper-case letter codes. Character classes made up of individual symbols such as @ denote
singleton sets, e.g., [[@]] = {64}. Character classes can also be used to form unions, they can be
complemented, and even subtracted from each other, and are in general closed under Boolean
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 7 -->

Regex Matching with Counting-Set Automata

218:7

operations. There are therefore many di�erent ways how to represent the same character sets, e.g.,
[[[0-9]]] = [[[0-45-9]]] = [[[0-4]]] [ [[[5-9]]]. To illustrate the complement, for example, [^0-9]
denotes the set of all non-digits, as does [\x00-\x2F\x3A-\x7F]. The set of all character classes is
then an example of the set Ψ of all predicates of a Boolean algebra, and checking satis�ability of a
predicate 2 Ψ means to decide whether denotes a nonempty set. For example, the predicate []
is unsatis�able because [[[]]] = ; and denotes the true predicate because [[ ]] = D. Further, note
that a character class can, without loss of generality, be represented as a Boolean combination of
intervals or even as a union of intervals if normalized.
⇤

.

.

3.1 E�ective Boolean Algebras
An e�ective Boolean algebra A has components (D, Ψ, [[_]], ?, >, _, ^, ¬) where D is a universe of
underlying domain elements. Ψ is a set of unary predicates closed under the Boolean connectives
_, ^ : Ψ ⇥ Ψ ! Ψ and ¬ : Ψ ! Ψ; and ?, > 2 Ψ are the false and true predicates. Values of
the algebra are sets of domain elements, and the denotation function [[_]] : Ψ ! 2D satis�es that
[[?]] = ;, [[>]] = D, and for all , 2 Ψ, [[ _ ]] = [[ ]] [ [[ ]], [[ ^ ]] = [[ ]] \ [[ ]], and
[[¬ ]] = D \ [[ ]]. For 2 Ψ, we write Sat( ) when [[ ]] , ;, and we say that is satis�able. We
require that Sat as well as _, ^, and ¬ are computable as a part of the de�nition of an e�ective
Boolean algebra. We write x |= for x 2 [[ ]] and we use A as a subscript of a component when it
is not clear from the context, e.g., [[_]]A : ΨA ! 2DA .
3.2

Words and Regexes

The basic building blocks of regexes are predicates from an e�ective Boolean algebra CharClass
of character classes, such as the class of digits, written as \d. Let D = DCharClass . A word over D
is a sequence of symbols a 1 · · · an 2 D⇤ and a language L over D is a subset of D⇤ . We use to
denote the empty word. The concatenation of words u and is denoted as u · (often abbreviated
to u ) and is lifted to sets as usual. We call a 2 D the head of the word a.w and w 2 D⇤ its tail.
def
def
Furthermore, we write L n for the n-th power of L ✓ D⇤ with L 0 = { } and L n+1 = L n · L.
The syntax of regexes is the following, with 2 ΨCharClass and n, m 2 N, 0  n, 0 < m, n  m:
R1 · R2

R 1 |R 2

R{n, m}

R⇤

where R 1 · R 2 is called a concat node and R 1 |R 2 is called a choice node. The semantics of a regex R is
def
def
def
de�ned as a subset of D⇤ in the following way: L( ) = [[ ]], L( ) = { }, L(R 1 R 2 ) = L(R 1 ) · L(R 2 ),
def
def –m
def
i
⇤
L(R 1 |R 2 ) = L(R 1 ) [ L(R 2 ), L(R{n, m}) = i=n (L(R)) , and L(R⇤) = L(R) . R is nullable if
2 L(R). We will also need to refer to the number of character-class leaf nodes of a regex R, denoted
by ]Ψ (R) and de�ned as follows: ]Ψ ( ) = 0, ] Ψ ( ) = 1, ] Ψ (R 1 · R 2 ) = ]Ψ (R 1 |R 2 ) = ]Ψ (R 1 ) + ] Ψ (R 2 ),
]Ψ (R{n, m}) = ]Ψ (R⇤) = ] Ψ (R).
3.3 Minterms
Let Preds(R) be the set of all predicates that occur in a regex R, and let Minterms(R) denote the set
of minterms of Preds(R). Intuitively, Minterms(R) is a set of non-overlapping predicates that can be
treated as a concrete �nite alphabet. Each minterm is essentially a region in the Venn diagram of the
”
predicates in R: it is a satis�able conjunction ψ 2Preds(R) 0 where 0 2 { , ¬ }. For example, if R =
[0-z]{4}[0-8]{5}, then Preds(R) = {[0-8], [0-z]} and Minterms(R) = {[0-8], [9-z], [^0-z]}.
Formally, if 2 Minterms(R), then Sat( ) and 8 2 Preds(R): [[ ]] \ [[ ]] , ; ) [[ ]] ✓ [[ ]].
Note that although the number of minterms of a general set X of predicates may be exponential
in |X |, it is only linear if X consists of intervals of symbols used in regexes, such as [a-zA-Z] or
[^a-zA-Z] (the former denotes two intervals while the latter their complement, which is equivalent
to the union of three intervals). Intervals of numbers generate only a linear number of minterms.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 8 -->

218:8

3.4

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

Symbolic Automata

We use symbolic �nite automata (FAs), whose alphabet is given by an e�ective Boolean algebra, as
a generalization of classical �nite automata. Formally, an FA is a tuple A = (I, Q, q 0, F, ∆) where I is
an e�ective Boolean algebra called the input algebra, Q is a �nite set of states, q 0 2 Q is the initial
state, F ✓ Q is the set of �nal states, and ∆ ✓ Q ⇥ ΨI ⇥ Q is a �nite set of transitions. A transition
(q, , r ) 2 ∆ will be also written as q (α)
!r .
A run of A from a state p0 over a word a 1 · · · an is a sequence of transitions (pi 1 (α i )
!pi )ni=1 with
ai 2 [[ i ]]; the run is accepting if pn 2 F . The language of a state q, denoted LA (q), is the set of words
over which A has an accepting run from q. The language of A, denoted L(A), is LA (q 0 ). A classical
�nite automaton can be understood as an FA where the basic predicates have singleton set semantics,
i.e., when for each concrete letter a there is a predicate a such that [[ a ]] = {a}. A is deterministic i�
for all p 2 Q and all transitions p (α)
!q and p (α 0)
!r , it holds that if ^ 0 is satis�able, then q = r .
4 COUNTING AUTOMATA
Counting automata (CAs) are a natural and compact automata counterpart for regexes with counting.
They are essentially a limited sub-class of classical counter automata, in which counters are only
supposed to count the number of passes through some of its parts (corresponding to a counted
sub-expression of a regex) and guards on transitions enforce a speci�ed number of repetitions of
that part before the automaton is allowed to move on.
Counter algebra. A counter algebra is an e�ective Boolean algebra C associated with a �nite set
C of counters. The counters play the role of bounded loop variables associated with a lower bound
minc 0 and an upper bound maxc > 0 such that minc  maxc . DC is the set of interpretations
m : C ! N, called counter memories such that 0  m(c)  maxc for all c 2 C. ΨC contains Boolean
combinations of basic predicates ���E���c and ���I���c , for c 2 C, whose semantics is given by
m |= ���E���c () m(c)

minc ,

m |= ���I���c () m(c) < maxc .

Counting automata. A counting automaton (CA) is a tuple A = (I, C, Q, q 0, F, ∆) where I is an
e�ective Boolean algebra called the input algebra, C is a �nite set of counters with an associated
counter algebra C, Q is a �nite set of states, q 0 2 Q is the initial state, F : Q ! ΨC is the
�nal-state condition, and ∆ ✓ Q ⇥ ΨI ⇥ (C ! O) ⇥ Q is the (�nite) transition relation, where
O = {����, ����, ����1, ����} is the set of counter operations. The component f of a transition
p (α ,f )
!q 2 ∆ is its (counter) operator. We often view f as a set of indexed operations ��c to denote
the operation assigned to the counter c, f (c) = ��.
def
Semantics of CAs. The semantics of the CA grd (����c ) def
= >C
upd (����) = λn.n
def
A is de�ned through its con�guration automa- grd (���� ) def
= ���I���c upd (����) = λn.n + 1
c
ton FA(A), an FA whose states are A’s con�gudef
def
grd
(����
)
= ���E���c upd (����) = λn.0
c
rations, i.e., pairs (q, m) 2 Q ⇥DC consisting of
def
def
a state q and a counter memory m. To de�ne grd (����1c ) = ���E���c upd (����1) = λn.1
FA(A), we must �rst de�ne the semantics of counter operators f , which occur on transitions. For
this, we associate with each (indexed) operation ��c a counter guard grd (��c ) and a counter update
upd (��) as shown on the right. Intuitively, the operation ���� does not modify the counter’s value
and is always enabled. The operation ���� increments the counter and is enabled if the counter has
not yet reached its upper bound. The operation ���� resets the counter to 0 on exit from the counting loop and is enabled when the counter reaches its lower bound. The operation ����1 executes
���� followed by ����. The guard of a counter operator f : C ! O is then a predicate f 2 ΨC over

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 9 -->

Regex Matching with Counting-Set Automata

218:9

counter memories, and its update f : DC [ {?} ! DC [ {?} is a counter-memory transformer:
⇢
€
if m |= f
def
def λc.upd (f (c))(m(c))
grd (��c )
f(m) =
f =
?
otherwise
��c 2f

Intuitively, f updates all counters in a counter memory m by their corresponding operations if m
satis�es the guard, otherwise the result is ?.
We now de�ne the con�guration automaton of A, denoted as FA(A), which de�nes the language
semantics of the CA A. The states of FA(A) are the con�gurations of A (there are �nitely many of
them), and the initial state of FA(A) is the initial con�guration (q 0, {c 7! 0 | c 2 C}) of A. A state
(p, m) of FA(A) is �nal i� m |= F (p). The transition relation ∆FA(A) of FA(A) is de�ned as
∆FA(A) = {(p, m) (α)
! (q, f(m)) | p (α ,f )
! q 2 ∆, m |=

f }.

Deterministic and simple CAs. A is deterministic i� the following holds for every state p 2 Q and
every two transitions p (α 1 ,f 1)
!q1 , p (α 2 ,f2)
!q2 2 ∆: if both 1 ^ 2 and f1 ^ f2 are satis�able,
then f 1 = f 2 and q 1 = q 2 . It follows from the de�nitions that, if A is deterministic, then FA(A) is
deterministic too. A is simple if for any two transitions q (α ,f )
!r and q 0 (α 0 ,f 0)
!r 0, either = 0 or
0
n o \ n o = ;. That is, di�erent character guards do not overlap and can be mostly treated as
plain symbols. We also require that all guards are satis�able. CAs constructed from regexes by the
algorithm in Section 5 will be simple.
Example 4.1. Fig. 1a shows a CA in an intuitive notation, with the initial state q and �nal
conditions F (q) = ?, F (s) = ����c , where minc = maxc = 100. The same notation is used in Fig. 2.
Fig. 3a shows a CA in a notation following the formal development more closely.
⇤
5 FROM A REGEX TO A CA VIA CONDITIONAL PARTIAL DERIVATIVES
We introduce a generalization of the Antimirov’s partial derivative construction [Antimirov 1996]
to symbolic counting, which allows one to replace a verbose NFA by a succinct CA. The di�erence
between the older variant of [Antimirov 1996] with explicit counting [Saarikivi et al. 2019] and
our new version was already illustrated in Section 2. To recall it brie�y using the example of the
regex {100}: from 100 partial derivatives @ ( {i}) = {i-1}, 1  i  100, and an NFA with 100
) {i-1}, the new construction will take us to the single derivative
states and transitions {i} ( !
@ ( {100}) = { {100}} associated with a conditional counter update which induce an NFA with a
) {100}.
single state and the transition {100} (α ,����c!
We apply the construction on regexes that are normalized using the below rules where X { Y
denotes that X is rewritten to Y :
• All nested concat nodes are rewritten to the �attened right-associative list form, which is
always maintained throughout the construction, using the rules: (X · Y ) · Z { X · (Y · Z ),
· Z { Z , and Z · { Z .
• If S is nullable, then S {`, k} { S {0, k}. Moreover, in the nullable context S {0, k }, S can be
considered as if it was not nullable.
Observe that the normalization does not increase the size of the regex (it may decrease the size).
Let R be a �xed normalized regex. A sub-expression of R that is of the form X = S {`, k} is called
a counting loop. We consider each counting loop to represent a counter whose name is the counting
loop itself and whose upper bound is maxX = k and lower bound is minX = `. For example, ( {9})⇤
contains the counter X = {9} with minX = maxX = 9. In the following, let C stand for the set of
all counters that occur in R, also denoted by Counters(R).
We use the convention that the juxtaposition XY of normalized regexes X and Y is again a
normalized regex of the equivalent concat node X · Y : e.g., if X = a · b and Y = (a · b)⇤, then

.

..

.

..

.

.

.

..

.

.

.

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 10 -->

218:10

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

XY = a · (b · (a · b)⇤). Observe in particular that X = X . In other words, we treat concatenated
elements as sequences, and a singleton sequence equals to the element itself.
Our construction will work over the set Σ = Minterms(R) of minterms of R and produce simple
CA that use minterms of Σ on transitions.
5.1

Parametric Languages

We de�ne the language of a normalized
def
Lm ( ) = { }
(1)
regex starting with a counting loop reldef
m
m
L ( Z ) = [[ ]] · L (Z )
(2)
ative to a counter value. For that, we
def
m
m
m
lift the de�nition of languages to be L ((W |Y )Z ) = L (W Z ) [ L (YZ )
(3)
parametric in counter memories m, but
def
m
m
⇤
m
L (S⇤Z ) = L (S) · L (Z )
(4)
regexes other than the above are treated
def
m
m
����S {`,k } (m)
(S {`, k}Z ) [ (5)
as before with the memory m passed L (S {`, k }Z ) = L (S) · L
through unchanged.
L����S {`,k } (m) (Z )
Recall that if f is a counter operator
def
L? (X ) = ; (for all X )
(6)
and m a counter memory, then f(m) denotes the appropriately updated memory where f(m) = ? when f is not enabled in m. Below,
if there is a single counter c 2 C such that f (c) , ����, we sometimes identify f with ��c and
use ��c (m) to represent the updated memory f(m). Speci�cally, ����X (if enabled) increments
the counter value of X by 1, and ����X (if enabled) resets the counter value of X to 0. Let m be
a counter memory. Then Cases (1)–(6) de�ne the parametric languages of regexes. The intuition
behind Case (4) is that all counters that may be present in S are inactive on the level of S⇤. Note
also that Case (5) is well-de�ned since, for X = S {`, k} and m 0 = ����X (m), k m 0(X ) < k m(X )
if m(X ) < k, and m 0 = ? if m(X ) = k.
def
Let 0 = λc.0 denote the initial memory that maps all counters to 0. The below theorem, proven
in [Turoňová et al. 2020], relates Lm (R) with the non-parametric de�nition of regular languages.
T������ 5.1. Let R be a normalized regex. Then L0 (R) = L(R).
5.2 Conditional Derivation
We will now introduce our conditional derivative construction formally.
A partial conditional derivative is a pair hf , X i where f is a counter operator and X a normalized
def
regex. Given a counter memory m, we let hf , X i de�ne the language Lm (hf , X i) = Lf (m) (X ). In
other words, f is �rst applied to the counter memory m and then the regex is evaluated in the
updated memory. If f is not enabled in m, then the denoted language is empty.
A conditional derivative is a �nite set of partial conditional derivatives. The language de�ned by
a conditional derivative D in a counter memory m is de�ned as the union of the languages of the
def –
partial conditional derivatives in D: Lm (D) = d 2D Lm (d).
To de�ne how conditional derivatives of a given regex looks like, we need a notion of a sequential
def
composition of conditional derivatives D ⌦ E = {hf ; , X ·Y i | hf , X i 2 D, h , Y i 2 E, f ; , ?}
where f ; , ? is the composed counter operator such that f; g(m) = g(f(m)). The case when
f ; = ? is discussed later on.
Conditional derivatives of a normalized regex are de�ned as shown on the right assuming that
concatenations X ·Y are normalized to the list form explained above, 2 Σ, �� denotes the identity
function λx .x, and X = S {`, k } is a counting loop. Observe that, in @α (S) ⌦ {h����X , XZ i}, the
operation ����X gets composed with ����X , yielding ����X again, because S {`, k } cannot occur in
S. It is possible that in {h����X , i}⌦@α (Z ), X is in scope of Z (e.g., Z starts with X ). The composition
can then contain the operation ����X ; ����X that corresponds to ����1X because ����X is trivially
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 11 -->

Regex Matching with Counting-Set Automata

218:11

enabled when the counter value of X
def
@α ( ) = ;
is 0. The only other possible composi⇢
{h��, Z i} if ^ is satis�able
def
tion of individual operations that can
@α ( Z ) =
;
otherwise
appear in this case is ����X ; ����X . If
minX = 0, ����X ; ����X = ����X , which @ ((W |Y )Z ) def
= @α (W Z ) [ @α (YZ )
α
is well-de�ned because ����X is always
def
@α (S⇤Z ) = @α (S) ⌦ {h��, S⇤Z i} [ @α (Z )
enabled for minX = 0. If minX >
def
0, then ����X ; ����X is unde�ned, and
@α (XZ ) = @α (S) ⌦ {h����X , XZ i} [
����X ; ����X does not contribute any{h����X , i} ⌦ @α (Z )
thing to the composition. However, this is
correct since X is not nullable, and the second ����X is not enabled after the counter value of X is reset to 0. Intuitively, the second occurrence of X cannot be exited without iterating X at least once.

.

Example 5.2. Consider the regex R = *a{1,3}a{1,3}a. Let X be the counting loop a{1,3}. R
has two minterms a and [^a]. We get the following conditional derivatives of R, starting with the
case for @α (S⇤Z ) due to the normal form assumption:
@a (R) =
=
@a (XX a)

=
=

@a (a) ⌦ {h����X , XX ai} [ {h����X , i} ⌦ @a (X a)
{h����X , XX ai} [ {h����X , i} ⌦ {h����X , X ai, h����X , i}

=

{h����X , XX ai, h����1X , X ai}

@a (X a) =
=
@a (a) = @a ( ) = @[^a] ( ) =

.

.

@a ( ) ⌦ {h��, Ri} [ @a (XX a)
{h��, Ri, h����X , XX ai, h����1X , X ai}

.

@[^a] (a) =

@a (a) ⌦ {h����X , X ai} [ {h����X , i} ⌦ @a (a)
{h����X , X ai, h����X , i}
{h��, i}
;

Above, the composition ����X ; ����X in @a (XX a) is unde�ned and thus removed. We also get that
@[^a] (R) = {h��, Ri} where @[^a] (a) = ; and consequently @[^a] (XX a) = ; and @[^a] (X a) = ;.
If we now consider, for example, the language de�ned by @a (X a) in a valid counter memory m,
it is the union of the languages L����X (m) (X a) and L����X (m) ( ). These correspond to the cases of
continuing to iterate the loop X (if the counter value of X is below 3) or exiting the loop (if the
counter value of X is at least 1) and accepting { }.
⇤

.

Example 5.3. Consider the regex ( {9})*, whose CA is in
c:=0
Fig. 2. Here, is the only input predicate and denotes the set
of all characters. We explain the use of some of the counter
(.{9})*
operations in the CA of Fig. 2 by showing how they arise
F: ꓔ
through the partial-derivative-based construction of CAs as discussed above. The initial state is the regex itself. The (only) par•/c:=1
tial derivative of the state ( {9})* is {9}( {9})* where the
.{9}(.{9})*
body of the counting loop is exited but also incremented once, •c<9/c++,
•c≥9/c:=1
F: c≥9
so ����1 is applied to c under the guard ���E���c (which is
shown as c
9/c:=1 in the �gure). The state {9}( {9})* has
Fig. 2. CA(( {9})*)
two cases of partial derivatives both leading back to {9}( {9})*.
The �rst case is when c < 9 (���I���c holds), in which case c is incremented (shown as
c < 9/c++ in the �gure). The second case is when the counting loop is conditionally nullable
and is exited under the condition ���E���c (i.e. c 9), the value of c is reset to 0, and then c is

.

.

. .

. .
. .

.

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 12 -->

218:12

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

.

incremented as a result of taking the partial derivative of ( {9})*. Thus, ����1 arises as a sequential
composition of exiting the loop, followed by resetting the counter to 0, and then incrementing it.
Therefore, ���E���c must hold, while the increment condition holds trivially after a reset to 0. The
initial state is unconditionally �nal in Fig. 2, while the other state is �nal only when ���E���c
holds as marked by “F :”.
⇤
We now state the correctness theorem of conditional derivatives. For that, we de�ne ���E���R
as the predicate shown above for a normalized regex R, assuming that X stands for a counting loop.
8
>C
if R = ,
>
>
>
>
< ���E���Z
else if R = YZ and Y is nullable,
def
���E���R =
���E���X ^ ���E���Z else if R = XZ ,
>
>
>
> ?C
otherwise.
:
Note that Y above may also be a counting loop. However, since it is nullable, minY must be 0, and
then ���E���Y is always true. (If minY > 0, then Y cannot be nullable as R is normalized.)
We further need the following additional notions too. A counter X is visible in R if either R = YZ
and X = Y , or else if X does not occur in Y and X is visible in Z . A counter memory m is valid
for R if m(X ) = 0 for all invisible counters X that occur in R. Correctness of the construction of
conditional derivatives is stated in Theorem 5.4—see [Turoňová et al. 2020] for a detailed proof.
T������ 5.4. Let R be a normalized regex and
– let Σ = Minterms(Θ) where Θ is some �nite superset
of Preds(R). If m is valid for R, then Lm (R) = α 2Σ [[ ]] · Lm (@α (R)) [ { | m |= ���E���R }.

5.3

Constructing CAs from Conditional Derivatives

We convert a normalized regex R to the counting automaton CA(R) whose set of states is the
smallest set containing R as the initial state and all those regexes that arise in conditional derivatives
constructed from R by repeated derivation wrt Σ. Given a state represented by a regex S, for each
2 Σ and each partial conditional derivative hf ,T i 2 @α (S), there is a transition S (α ,f )
!T in
CA(R). The �nal condition F (S) of a state S of CA(R) is ���E���S . Observe that F (S) = ?C when S
is not nullable and has no visible counters, which corresponds to the classical case.
As shown in [Turoňová et al. 2020] the following result can be proved using Theorem 5.4.
T������ 5.5. Let R be a normalized regex and A = FA(CA(R)). Then, for all hm, Si 2 Q A ,
LA (hm, Si) = Lm (S).
The construction of CA(R) terminates, and the number of states of CA(R) is linear in ] Ψ (R).
T������ 5.6. Let R be a normalized regex. Then |Q CA(R) |  ] Ψ (R) + 1.
A proof of Theorem 5.6 is in [Turoňová et al. 2020]. We get the following �nal correctness result
as a corollary of Theorem 5.5, Theorem 5.1, and Theorem 5.6.
C�������� 5.7. Let R be a normalized regex. Then L(R) = L(CA(R)).
P����. First, Q CA(R) is �nite, and thus well-de�ned by using Theorem 5.6. Use Theorem 5.5
with hm, Si as the initial state h0, Ri of A. It follows that L(A) = L0 (R). Then use Theorem 5.1 for
L0 (R) = L(R) and L(CA(R)) = L(A) holds by de�nition.
⇤
A further important aspect of CA(R) is that, although the number of input minterms may potentially be exponential in the number of predicates in R, in the case of predicates being represented
as a �nite union of intervals (as is done typically for character classes), the size of a single predicate
representation can be estimated to be proportional to the number of interval borders in the union.
In this case, the total size of all the minterms remains linear in the total size of all the predicates
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 13 -->

Regex Matching with Counting-Set Automata

218:13

because the total number of interval borders will remain the same in minterms as in the original
set of predicates. In other words, mintermization based on character classes does not blow up the
number of transition in CA(R). We have also validated this fact experimentally.
6 FROM COUNTING AUTOMATA TO COUNTING-SET AUTOMATA
CAs obtained through conditional derivatives as shown in the previous section are nondeterministic.
As one of the main contributions of this work, we now propose an approach for determinizing
them into a form that can be used e�ciently for regex matching. The approach from which we
start and to which we contrast our new method is the naive determinization of CAs to DFAs: The
given CA is �rst converted to its underlying NFA, by making the counter memories an explicit part
of control states. The NFA is in turn determinized by the textbook subset construction.
Already the �rst step, the construction of the NFA, oftentimes explodes since it sacri�ces the
succinctness of symbolic counters (it is linear to the counter bounds). This initial blow-up is then
much ampli�ed in the subset construction, which is exponential to the size of the NFA and hence
also to the counter bounds (as, e.g., in the case of the regex *a {k} with its CA in Fig. 1a).
Our answer to this problem is a direct determinization of the CA into a novel type of automata,
which we call counting-set automata (CsAs). Control states of counting-set automata produced
by our determinization are essentially the states of the corresponding DFA but with the counter
memories removed. In order to be able to simulate a run of the DFA, they are equipped with special
registers that can hold sets of integers, and they use them to compute the right counter memories
at runtime. This completely avoids the state space explosion of the naive construction caused
by wiring counter memories into control states. Moreover, the simulation is fast because all the
manipulations with a counting set can be done in constant time.

..

6.1 Counting-Set Automata
We now formalize the idea of counting-set automata outlined above. We use the notion of a combined
Boolean algebra I⇥S, which allows us to manipulate pairs of predicates from the input algebra I and
the counting-set algebra S. For the purposes of this paper, we assume that predicates in ΨI⇥S have the
form ^ where 2 ΨI and 2 ΨS . The conjuction ( ^ ) ^I⇥S ( 0 ^ 0) has the usual meaning of
( ^I 0)^( ^S 0) and ^ is satis�able if both and are satis�able in their respective algebras.
Counting sets. We consider a set-based interpretation of counters where the value of a counter c
is a �nite set rather than a single value. A counter under such an interpretation is referred to as a
counting set. A (counting-)set memory for C is a function s : C ! P�n (N) such that, for all c 2 C,
Max(s(c))  maxc .3 Observe that the set of all set memories for C is �nite. Counting-set predicates
over C form an e�ective Boolean algebra SC called the counting-set algebra over C, also denoted
just S when C is clear from the context, whose domain DS is the set of all set memories for C. The
set of predicates ΨS is the Boolean closure of the basic predicates ���I���c and ���E���c , hence
syntactically the same as in the counter algebra C, but with a di�erent semantics under S:
s |= ���E���c () Max(s(c))

minc

and s |= ���I���c () Min(s(c)) < maxc

where Min(·) and Max(·) are the set minimum and maximum, respectively. Intuitively, the conditions
test existence of a set element satisfying the same counter condition.
Counting-set automata. A counting-set automaton (CsA) is a tuple A = (I, C, Q, q 0, F, ∆) where:
I is an e�ective Boolean algebra called the input algebra. C is a �nite set of counters associated
with the counting-set algebra S. Q is a �nite set of states with q 0 2 Q being the initial state.
3 We write P (X ) for the powerset of X restricted to �nite nonempty sets.
�n

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 14 -->

218:14

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

F : Q ! ΨS is the �nal-state condition. ∆ ✓ Q ⇥ ΨI⇥S ⇥ (C ! P(O)) ⇥ Q is a �nite set of transitions.
The second component is its guard. The third component is the counting-set operator in which
O = {I���, N���, R��, R��1} is the set of counting-set operations. They are essentially counter
operations lifted to sets (note the use of the larger initial letters to distinguish them from the
counter operations). We also use the di�erent names R�� and R��1 for the lifting of ���� and ����1
to stress their di�erent usage (not only for exiting a loop but also for initialisation when entering
the loop as will become clear in Eq. (7)). Sets of counting-set operations assigned to every counter
by the counting-set operator are called combined (counting-set) operations.
The CsA A is deterministic i� the following holds for every two transitions p (ψ 1 ,f 1)
!q1 and
p (ψ 2 ,f 2)
!q2 in ∆: if 1 ^ 2 is satis�able, then f1 = f2 and q1 = q2 .
Semantics of CsAs. The semantics of an indexed counting-set operation ��c 2 O is the set
transformer upd (��c ) de�ned as follows:
upd (I���c ) = λS.{n + 1 | n 2 S ^ n < maxc }
upd (N���c ) = λS.S

upd (R��c ) = λS.{0}
upd (R��1c ) = λS.{1}

Then, the counting-set operator f : C ! P(O) is assigned the counting-set-memory transformer
f : DS ! DS de�ned as follows:
⇢ –
def
�� 2f (c) upd (��c )(s(c)) if f (c) , ;
f(s) = λc.
{0}
if f (c) = ;
That is, (1) if f (c) , ;, then the value s(c) of each counting set c is transformed into the union of the
counting sets that result from applying the operations from f (c) on s(c), and (2) if f (c) = ;, then c
is implicitly reset to {0} (an implicit R��). Our determinization procedure creates such transitions
when the value of c is irrelevant (when c is a dead variable).
Note that, unlike counter operators of a CA, a counting-set operator f does not induce any
guard. The guard is rather a separate component of the transition. This is because CsA transitions
produced in the CA-to-CsA determinization need guards that are partially independent of the
operations of f . In particular, we will we need to distinguish cases such as ¬���E���c ^ ���I���c ,
���E���c ^ ¬���I���c , or ���E���c ^ ���I���c . The guard hence cannot be induced by f alone.
Note also that, unlike in CAs, the updates are de�ned for indexed operations. The reason is that
the semantics of the I��� operation is restricted to never produce values greater than maxc .
Finally, the language of the CsA A is de�ned through its underlying con�guration FA, FA(A),
as L(A) := L(FA(A)). The states of FA(A) are con�gurations of A, namely, tuples of the form
(q, s) 2 Q ⇥ DS consisting of a state q and a counting-set memory s. There are �nitely many
such con�gurations. The initial state of FA(A) is the initial con�guration (q 0, {c 7! {0}}c 2C ) of A.
) q 2 ∆ is enabled in a con�guration (p, s) i� is satis�able and s 2 [[ ]]S ,
A transition = p (α ^β ,f!
meaning that s satis�es the counter guard . If is enabled in (p, s), then FA(A) contains the
transition (p, s) (α)
!(q, f(s)). Finally, a state (q, s) of FA(A) is �nal i� s |= F (q).
Example 6.1. An example of a CsA is in Fig. 1b. It uses intuitive notations that were also introduced
in Section 2 as abbreviations for the operations of the counting-set data structure. Counting-set
operators are depicted as assignments to c, R�� is represented as {0} on the right of the assignment,
R��1 is represented by {1}, I��� by c + 1, and N��� by c. Multiple transitions between the same
states and with the same updates are merged into one with a simpli�ed guard. An example whose
notation closely follows the formal development is in Fig. 3.
⇤
Runtime e�ciency of counting sets. A major reason for choosing CsAs as the target kind of
machine for determinization of CAs is that pattern matching with CsAs is fast. Using the data
structure explained in Section 2, all the basic counting-set tests and updates, namely, ���I���c ,
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 15 -->

Regex Matching with Counting-Set Automata

218:15

���E���c , N���, I���, R��, and R��1, can be implemented to run in constant time regardless of
the size of the counting set and the value maxc (assuming constant-time complexity of integer
arithmetic operations). Moreover, almost all combined counting-set operations can be implemented
to run in constant time too. In particular, when at most one counting-set operation of a given
combined operation returns a set other than {0} or {1}, their union can be computed in constant
time. However, the union of two general sets, other than {0} and {1}, would take time linear to the
size of the sets (which is at most maxc ). The only operations that may return sets other than {0}
or {1} are N��� and I���. We therefore call a transition slow if its counting-set operator f assigns
to some counter c the result of a combined operation f (c) that contains both N��� and I���. A
CsA that has slow transitions is called slow, and a CsA that does not have them is called fast. Slow
CsAs are fortunately rare in practice (cf. Section 7).
When a fast CsA is used in pattern matching, tests and updates of one counting set then take
O(1) time, which in turn gives O(|C |) for all counting sets and their unions. This is our major
achievement: the independence of the running time from the counter bounds.
6.2 Encoding DFA Powerstates as CsA Configurations
In order to build intuition needed for understanding our determinization algorithm, we will �rst
concretize how the con�gurations of a CsA can encode states of a DFA corresponding to the NFA
FA(A) underlying a given CA A = (I, C, Q, q 0, F, ∆). First, recall that, since A is converted into FA(A)
by making the counter memories explicit parts of control states, the states of FA(A) are pairs (p, m)
consisting of a state p of A and a counter memory m. Second, assume that FA(A) is determinized
using the textbook subset construction.4 We denote the result as DFA(A) from now on. Then, the
states of DFA(A) are sets of states of FA(A), i.e., sets of pairs (p, m), which we will call powerstates.
The control states of the CsA A0 built by our CA-to-CsA determinization will be subsets of the set
Q of states of the CA A. The con�gurations of A0 will thus be pairs (R, s) where R ✓ Q is a CsA
control state, i.e., a set of states of A, and s : C ! P�n (N) is a counting-set memory. Let us now
consider how s can be interpreted in this context.
Naive encoding. A naive interpretation of a CsA con�guration (R, s) is a DFA state containing all
pairs (r, m) such that r 2 R and, for all c 2 C, m(c) can be any
Œ value from s(c). The set of the counter
memories m is then isomorphic to the Cartesian product c 2C s(c) of the sets s(c) assigned to the
counters, and the entire powerstate is the Cartesian product R ⇥ m of the set of states and the set
of counter memories. The naive interpretation, however, is too impractical as it cannot express any
dependence of a counter memory on the CA state (every state can be paired with each considered
memory) nor any mutual dependence of values of di�erent counters within a counter memory
(every possible value of a counter c can be paired with every possible value of any other counter
d). Most DFAs compiled from real-life regexes do not �t into this representation. For instance, the
DFA con�guration {(q, c = 0), (s, c = 0), (s, c = 1)} of the CA from Fig. 1 in Section 2 could not be
represented by a CsA con�guration because q and s appear with di�erent sets of counter values.
Encoding with counter scopes. Our key observation how to resolve the above problem (at least for
many real-life scenarios) is to take advantage of that not every counter is “used” at every CA state.
In fact, the value of a counter is usually implicitly 0 at most states except a few. If these states are
known, the implicit zeros do not have to be remembered explicitly in the counting sets, and the
encoding becomes much more �exible. To formalize this, we introduce the notion of the scope of a
4 The DFA produced by the textbook subset construction from a simple FA A = (I, Q , q

0 , F , ∆) will have P(Q ) as the set of
states, transitions S (α )
! {r 2 Q | s (α )
!r 2 ∆, s 2 S }, the initial state {q0 }, and as the �nal states all those intersecting
F . We note that to determinize a CA which is not simple, one could start from the more sophisticated version of the subset
construction for symbolic automata of [Veanes et al. 2010], which avoids explicit generation of all minterms.

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 16 -->

218:16

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

counter that over-approximates the set of states where a counter c can have a non-zero value and
that is easy to compute.5 The scope is de�ned inductively as the smallest set of states (c) such that
(1) q 2 (c) if there is a transition to q with either ����c or ����1c , or
(2) there is a transition to q from a state in (c) with the ����c operation.
In other words, the scope of c spreads from an increment of c along the transition relation until a
transition with ����c .
The DFA powerstate encoded by a CsA con�guration (R, s) can then be formally de�ned as the
set (R, s)DFA of con�gurations (r, m) of the CA A such that r 2 R and, for all c 2 C, m(c) 2 s(c) if
c 2 (r ), else m(c) = 0. We call the powerstates of DFA(A) that can be encoded by CsA con�gurations
Cartesian, and call the entire DFA Cartesian if all its powerstates are Cartesian.
Example 6.2. The powerstates of the DFA(A) of the CA A from Fig. 1a are indeed Cartesian (as
discussed in Section 2) because q 0 is not in the scope of c. The encoding of powerstates by CsA
con�gurations is also illustrated in Section 2 and later also in Example 6.4.
⇤
The Cartesian encoding still cannot express all kinds of DFA powerstates. In particular, it cannot
express more subtle dependencies of counter values on the state, and dependencies of counter
values of di�erent counters on each other, which mainly concerns CAs with nested counting
loops compiled from regexes with nested counting sub-expressions. Example 6.5 discusses a regex
that leads to a non-Cartesian CA. However, we later present a strong empirical evidence that a
signi�cant majority of real-life regexes lead to Cartesian CA.
6.3 Generalized Subset Construction
We will now describe the core of our CA-to-CsA determinization. It is built on top of the textbook
subset construction for NFAs. We use the CA from Fig. 3a as a running example through the section.
We make a simplifying assumption that the input CAs are simple (di�erent character classes on
their transitions do not overlap). This is satis�ed by CAs generated by the derivative construction
from Section 5 since their transitions are labeled by minterms of the original regex. The assumption
could be dropped and the construction could be relatively easily generalized in the style of symbolic
automata determinization of [Veanes et al. 2010].
Let A = (I, C, Q, q 0, F, ∆) be a simple CA with the scope function : Q ! P(C). The algorithm
produces the deterministic CsA A0 = (I, C, Q 0, S 0, F 0, ∆ 0) whose components are constructed as
described below. Namely, control states of A0, called powerstates, are subsets of Q, i.e., Q 0 ✓ P(Q).
The initial powerstate is S 0 = {q 0 }. A powerstate S 2 Q 0 is �nal i� the �nal condition holds for
def ‘
some of its elements, i.e., F 0(S) = q 2S F (q). The sets ∆ 0 and Q 0 are constructed by a �xpoint
computation that explores the state space reachable from S 0 . During the construction, transitions
starting from previously reached powerstates are constructed and included together with their
target states into ∆ 0 and Q 0, respectively, until no new powerstates can be reached.
Transitions starting from a given control state R of the CsA A0 are constructed to update the
runtime values of counting sets such that they simulate transitions of the DFA corresponding
to the CA A. Assume a CsA con�guration (R, s) and a DFA transition (R, s)DFA (α)
!P from the
DFA powerstate encoded by (R, s) over an input minterm . The simulating CsA transition must
transform (R, s) into (R 0, s 0) with (R 0, s 0)DFA = P. The simulated DFA transition was constructed
from -transitions of the NFA FA(A) that are actually instantiations of the CA -transitions enabled
5 Computing the precise set of states where a counter c can have a non-zero value would require a reachability analysis in the

general case (since some of the transitions may never be executable—think of simultaneously counting with counters c and
d such that ���I���c < ���E���d , then the exit transition for d will never be taken). For CAs produced by our derivative
construction, the scope, however, corresponds to this set precisely—no transitions that are never executable are generated.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 17 -->

Regex Matching with Counting-Set Automata

218:17

in con�gurations (r, m) 2 (R, s)DFA . The simulating CsA transition will be constructed from these
CA transitions. They can be identi�ed by (1) their source state, which must be in R, (2) an alphabet
minterm 2 Σ where Σ is the set of minterms over all input predicates in the CA A, and (3) their
compatibility with a particular set of enabled/disabled counter guards. This set of guards belongs
to the set of minterms ΓR,α of the set of counter guards on the -transitions originating in R:
def

ΓR,α = Minterms({grd (��c ) | r (α ,f )
! s 2 ∆, r 2 R ^ c 2 (r ), ��c 2 f }).
Hence, the CsA will have a transition leaving R for each 2 Σ and 2 ΓR,α , and the transition will
be built from the set of CA -transitions originating in R and consistent with :
def

∆R,α ,β = {r (α ,f )
! s 2 ∆ | r 2R, Sat( f ^ )}.
Its target is the set T of all target states of the transitions in ∆R,α ,β , and its guard is ^ .6
The remaining component is the counting-set operator f 0. It must summarize the updates of the
counter values on transitions of ∆R,α ,β as updates of the respective counting sets. The values of
counters that are out of scope, hence implicitly zero, will not be tracked in counting sets. Tracking
the value of a counter hence starts when A0 simulates a transition of A entering the scope of the
counter, and ends when no state from the scope is present in the target CsA state.
def
Let ∆R,α ,β (c) be the set of transitions in ∆R,α ,β
op( p (α ,f )
!q , c) =
with the target state in the scope of c. The counting8
def
N��� if f (c) = ���� ^ p 2 (c)
>
>
set operator f 0 is built in the form f 0(c) = {op( , c) |
>
>
I��� if f (c) = ���� ^ p 2 (c)
>
>
2 ∆R,α ,β (c)}. Here, op( , c) denotes the counting>
>
< R��
if f (c) = ���� ^ p < (c)
def
set operation that, given a CA transition
= =
(7)
R��1
if f (c) = ���� ^ p < (c)
>
p (α ,f )
!q, transforms the set of possible values of >>>
R��
if f (c) = ����
>
>
the counter c at the state p to the set of values ob>
> R��1 if f (c) = ����1
:
tained at q after taking the transition. It is de�ned
in Eq. (7) on the right. The set operation induced by the CA transition corresponds to the counter
operation on the transition. In the third and fourth case, when the CA transition comes from out
of the scope, it is certain that the counter can only have the value 0, which is the same value as
produced by ���� (or ����1 when the counter is immediately incremented). The resulting CsA
transition is therefore S (α ^β ,f 0)
!T . Note that f 0(c) ends up empty when the target powerstate is
fully out of the scope of c, which semantically corresponds to the implicit reset to {0}.
Observe that A0 is deterministic since, for any two distinct transitions S (α ,f )
!T and S (α 0 ,f 0)
!T 0,
0
the condition ^ is unsatis�able by virtue of minterms.
T������ 6.3. For the CA A and the CsA A0 above, we have L(A0) ◆ L(A) and |Q 0 |  2 |Q | .
P���� (����). The language inclusion is proved by showing that the con�guration automaton
FA(A0) of A0 simulates DFA(A), more concretely, that each con�guration (R, s) of A0, a state of
FA(A0), simulates the powerstate (R, s) of DFA(A). The bound on the size of the state space follows
from that states of the CsA are sets of states of the CA.
⇤
Example 6.4. Consider the CA in Fig. 3a that has states q 0 , q 1 , and q 2 . The state q 0 is initial, the
�nal condition of q 2 is >, and it is ? for q 0 and q 1 . The set of counters is C = {c} with (c) = {q 1 }
(i.e., c is not used and hence implicitly 0 in q 0 and q 2 ). Finally, Σ = {a, [^a]}. In Fig. 3a, we compactly
represent transitions over all minterms from Σ using . The determinization starts exploring the
CsA from its initial state S 0 = {q 0 }.
Let us focus on the transitions for the input minterm = a. Two transitions are leaving q 0 , namely
!q0 and 2 = q0 (a,����c )
!q1 , both with no guard on c, hence ΓS0 ,α = {>}. The
1 = q 0 (a,����c )

.

6 Recall that the predicates in Ψ and Ψ are syntactically the same.
C
S

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 18 -->

218:18

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar
c:={0}
[^a]

{q₀}

c0

[^a]ξ(c)¬ι(c)
•

a

• /INCR(c)

q₁
(c)
a/EXIT(c)

•

a/RST(c)

q₀

q₂
F: ꓔ

..

(a) The CA for *a {4,8}a

[^a]ξ(c)¬ι(c)

[^a]¬ξ(c)ι(c)/INCR(c),
[^a]ξ(c)ι(c)/INCR(c),
a¬ξ(c)ι(c)/INCR(c), RST(c)

{q₀,q₁}

[^a]¬ξ(c)ι(c)/INCR(c),
[^a]ξ(c)ι(c)/INCR(c),
a¬ξ(c)ι(c)/INCR(c), RST(c)
aξ(c)ι(c)/INCR(c), RST(c),
aξ(c)¬ι(c)/RST(c)

aξ(c)ι(c)/INCR(c), RST(c),
aξ(c)¬ι(c)/RST(c)

{q₀,q₁,q₂}
F: ꓔ

(b) Determinization of the CA into a CsA

Fig. 3. From a regex via a CA to a deterministic CsA. We are using a notation closely following the formal
development. We only use ��(c) instead of ��c and abbreviate ���E���c by (c) and ���I���c by (c).

guard > is thus the only choice for the counter minterm . The set ∆R,α ,β of transitions consistent
with and then contains both a-transitions 1 and 2 originating from q 0 . Since 2 is entering the
scope of c, it generates the counting-set operation R��c according to the third case of Eq. (7). Since 1
stays out of the scope, it does not generate any counting-set operations. We obtain the counting-set
operator f 0 = {R��c } and generate the CsA transition 1 = {q 0 } (a^β , {R��c })
!{q0, q1 }.
Next, let us focus on the a-transitions from S 1 = {q 0, q 1 }. Here, ΓS1 ,a has the following three
satis�able elements: ���E���c ^ ���I���c , ¬���E���c ^ ���I���c , and ���E���c ^ ¬���I���c
(the guard ¬���E���c ^ ¬���I���c is excluded as it is never satis�ed for non-empty sets of
positive integers). Let us generate a transition for the second case, = ¬���E���c ^ ���I���c . We
obtain ∆S1 ,a,β = {q 0 (a,����c )
!q0 , q0 (a,����c )
!q1 , q1 (a,����c )
!q1 }. As before, the �rst transition
does not contribute to f 0 as it stays out of the scope, and the second transition adds R��c . The
third transition adds I���c (the second case of Eq. (7)). The resulting CsA transition is thus 2 =
S 1 (a^¬���E���c ^���I���c , {I���c ,R��c })
!S 1 . The rest of the construction is analogous.
Last, let us also illustrate the simulation of DFA(A) by the constructed CsA transitions. On the
word aa, the DFA would execute the run {(q 0, c = 0)} (a)
!{(q0, c = 0), (q1, c = 0)} (a)
!{(q0, c =
0), (q 1, c = 0), (q 1, c = 1)}. The simulating run of our CsA would start in the initial con�guration
{{q 0 }, c 2 {0}}. The transition 1 would produce the con�guration {{q 0, q 1 }, c 2 {0}} (since
R��({0}) = {0}) from where 2 would produce {{q 0, q 1 }, c 2 {0, 1}} (since I���({0}) = {1} and
R��({0}) = {0}). The sequence of con�gurations precisely encodes the sequence of the DFA
powerstates, that is, the sequnce ({q 0 }, c 2 {0})DFA = {(q 0, c = 0)}; ({q 0, q 1 }, c 2 {0})DFA =
{(q 0, c = 0), (q 1, c = 0)}; and ({q 0, q 1 }, c 2 {0, 1})DFA = {(q 0, c = 0), (q 1, c = 0), (q 1, c = 1)} (recall
that q 0 is not in the scope of c, hence c has implicitly the value 0 there).
⇤
6.4 Uniformity: A Su�icient Semantic Correctness Criterion
Given a CA A, we produce a CsA A0 that may overapproximate A in terms of the language. We
explain how this may happen and present conditions under which the language stays unchanged.
In particular, the overapproximation is caused by non-Cartesian powerstates of DFA(A). (Recall
that, in a Cartesian powerstate, states in the scope of a counter must appear with the same set
of values of that counter.) A con�guration of the CsA cannot encode a non-Cartesian powerstate
precisely, it can only overapproximate it. A larger powerstate may then accept a larger language.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 19 -->

Regex Matching with Counting-Set Automata

218:19

Example 6.5. Take R = (a|aa){5} and the CA(R) shown in Fig. 4.
After reading the word aa, DFA(CA(R)) reaches the powerstate
{(q 0, c = 1), (q 0, c = 2), (q 1, c = 2)}, which is not Cartesian because both states are in the scope of the counter c but are paired
with di�erent counter values. Our CsA would reach the con�guration ({q 0, q 1 }, c 2 {0, 1, 2}), which encodes the larger powerstate
{(q 0, c = 0), (q 0, c = 1), (q 0, c = 2), (q 1, c = 0), (q 1, c = 1), (q 1, c =
2)} where both states appear with both counter values.
⇤

Fig. 4. CA((a|aa){5}).

Uniformity. We now introduce the so-called uniformity of a CA as a property under which
determinization preserves the language. Uniformity prevents creation of non-Cartesian powerstates.
It includes two conditions.
The �rst condition prevents the kind of scenario from Example 6.5. For each DFA transition 0, it
requires that every CA state q that is in the scope of some counter c within the DFA state to which
0 leads receives the same set of values of c. This requires testing whether the sets of transitions
covered by 0 and incoming to every such CA state q induce the same CsA operations for c.
The second condition prohibits two counters from being active at once, a scenario which arises
from regexes with nested counting. Indeed, the relation between values of two simultaneously active
counters may easily become more intricate than what can be expressed by a Cartesian product of
two sets (consider, e.g., the regex a?(a{1}a){2} and the word aaa). The condition requires testing
that no state appears in the scope of two counters.
0)
Formally, given a CsA transition 0 = S (α ^β ,f !
T , a counter c, and a CA state q 2 (c), we
de�ne the set fq0(c) of incoming CsA operations for c induced by the incoming transitions of q from
which 0 is built ( -transitions consistent with originating in S) as follows:
def

fq0(c) = {op( , c) |

2 ∆S ,α ,β (c) ^ the target of is q} .

0 uniform i�, for each counter c 2 C, any two states q, r 2

We call the transition
(c) \ T have the
same sets of incoming CsA operations, i.e., fq0(c) = fr0(c). The CA A is then uniform if all transitions
of A0 are uniform and if no state of A appears in the scope of two counters.
T������ 6.6. If a CA A is uniform, then L(A) = L(A0).
P���� (����). By showing bisimilarity between states q of FA(A0), i.e., con�gurations of the
CsA A0 and powerstates q DFA of DFA(A).
⇤
Uniformity can be checked on the �y, while constructing A0. It is also automatically implied
when the CA is constructed from certain classes of regexes, as discussed below.
6.5 Syntactic Correctness Criteria
Uniformity is only a semantic property. Below, we show examples of actual regexes that do and do
not lead to uniform CAs and discuss some simple syntactic classes of regexes that imply uniformity.
A detailed study of syntactic classes of regexes that guarantee uniformity is, however, beyond the
scope of this paper and a part of our future work.
The regexes that induce non-uniform CAs are often those where, intuitively, there is a position in
some input text that may either be matched against the �rst character of a counted sub-expression
or against some inner character of the same sub-expression. In such a situation, there may be two
runs of the induced CA: one that increments the associated counter (the increment happens) at
that position and moves to some state q, and the other that leaves the counter as it is, while in its
scope, and moves into a di�erent state r . The counter value then depends on the state: it is di�erent
in q and in r . The corresponding DFA state is then non-Cartesian and the CA is non-uniform.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 20 -->

218:20

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

Example 6.7. We present several commented examples of regexes with non-uniform CAs where
our determinization overapproximates the language of the obtained CsA.
• (a|ab|ba){5} — the string aba could be matched as “a” followed by “ba”, having incremented
the counter twice, or as “ab” that is followed by the pre�x “a” of “ab”, having incremented
the counter once only.
• a{1,3}a{3} — this case can be explained similarly as the previous one. Alternatively, note
that, assuming that our translation to a CA produces two counters, say c 1 and c 2 , then after
reading n letters a, the CA needs to remember that c 1 + c 2 = n. Such non-trivial relations
between counter values are not Cartesian.
• *(aa){6} — assuming a sequence of a’s on the input, the counter may be either incremented
on odd characters and left unchanged on even ones, or the other way around. As the counter
values depend on the position within the “aa” (and hence on the CA state), the CA cannot be
uniform. Note that the pre�x * is quite usual as it corresponds to searching for the regex
(aa){6} anywhere in the input string.
• *(a{2}){2} — after reading aa, if the value of the outer counter is 1, then the value of
the inner counter must be 0. This is a non-trivial relation between the values of the two
counters, which is not Cartesian. Nested counting is often problematic, however, many of
such examples may still be solved quite e�ciently by unfolding one of the counters.
⇤

.

.

.

Syntactic classes of regexes that guarantee uniformity. A simple class of regexes that guarantees uniformity is a generalization of the class of monadic regexes of [Holík et al. 2019] (where counting is allowed over character classes only). Namely, it consists of regexes with counting loops of the form
( 1...

n ){`, k} s.t. n 1 o is disjoint from every n i o, 1 < i  n.

Intuitively, the disjointness with 1 ensures that the generated CA will only be able to process 1
through an increment transition at the beginning of a new iteration of the loop, with no possibility
of having a con�icting ���� transition that could read the same symbol inside the body of the loop
(which is exactly what happens with the second symbol a in Example 6.5). The CsA compiled form
this class are also guaranteed to be fast.
7 EXPERIMENTAL EVALUATION
We have implemented our approach in a C# prototype called CA available at [Turoňová et al.
[n.d.]] (see [Turoňová et al. 2020] for details how to e�ciently implement CsAs) and evaluated its pattern matching capabilities against other state-of-the-art regex matchers on patterns
that use the counting operator. We focused on comparison against Google’s RE2 library [Google
[n.d.]]7 , an automata-based matcher designed to be fast, predictable, and resilient against ReDoS attacks. We also include other three e�cient matchers into the comparison, namely the
standard GNU grep program [Haertel et al. [n.d.]] (version 3.3), the .NET standard library regex
matcher from System.Text.RegularExpressions [Microsoft 2020], and Symbolic Regex Matcher
(SRM) [Saarikivi et al. 2019].
Let us shortly summarize how the tools work. The main algorithms of RE2 and grep implement
optimized versions of the Thompson’s on-the-�y determinization where the constructed DFA states
are cached. The construction has a bound on the size of the DFA—if the bound is reached, the
so-far constructed DFA states are �ushed to avoid consuming too much memory. In some situations
when caching is found ine�ectual, RE2 turns the caching o�, and the performance can drop even
lower (see the description in [Cox 2010] for details). We note that RE2 rejects an input regex if it
contains a counting operator with a bound bigger than 1,000. SRM is based on symbolic derivatives
7 We used the version 2019-01-01 of RE2 via the command line interface re2g from https://github.com/akamai/re2g.

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 21 -->

Regex Matching with Counting-Set Automata

218:21

constructed on the �y, also in the spirit of the Thompson’s algorithm, and, likewise, bases its
e�ciency on caching (in fact, SRM is quite close to an implementation of the Thompson’s algorithm
over CAs with caching). The .NET matcher uses a backtracking algorithm over NFAs, while our
CA eagerly constructs a deterministic CsA for the input regex. The former four are mature tools,
and especially RE2 and grep contain many high- and low-level optimizations, such as using the
Boyer-Moore algorithm [Boyer and Moore 1977] to skip over many characters that are known
to not be a part of a match. RE2 and grep are compiled programs while CA, SRM, and .NET run
within the .NET Framework (therefore, they have some inherent overhead due to the just-in-time
compilation at start-up and its inability to use advanced code optimizations, as well as garbage
collection). Note that even though the tools based on the on-the-�y subset construction (RE2, grep,
and SRM) are linear to the lenght of the text, they still take space exponential to the counter bounds
in the worst case, by creating sets of the size linear to the counter bounds, exponential to their
decadic encoding used in the regex.
We run our benchmarks on a machine with the Intel(R) Xeon(R) CPU E3-1240 v3 @ 3.40 GHz
running Debian GNU/Linux (we use the Mono platform [project [n.d.]] to run .NET tools). To avoid
issues with generating exact matches, which might di�er for di�erent tools, the tools were run in the
setting where they counted the number of lines matching8 the given regex (e.g. the -c �ag of grep).
7.1

ReDoS Resiliency

Our main experiment focuses on the resilience of the matching engines against ReDoS attacks. The
regexes used for this experiment were selected (1) from the database of over 500,000 real-world
regexes coming from an Internet-wide analysis of regexes collected from over 190,000 software
projects [Davis et al. 2019]; (2) from databases of regexes used by network intrusion detection systems
(NIDSes), in particular, Snort [M. Roesch et al. [n.d.]], Bro [Robin Sommer et al. [n.d.]], Sagan [The
Sagan team [n.d.]], and the academic papers [Češka et al. 2018; Yang et al. 2010]; (4) the RegExLib
database of regexes [RegExLib.com [n.d.]]; and (5) industrial regexes from [Holík et al. 2019], used
for security purposes. From these, we created our set of benchmarks by the following steps:
(1) We selected regexes that contained counting loops whose sum of upper bounds was larger
than 20. This let us focus on regexes where the use of counting makes sense (there are surprisingly many regexes occurring in practice where the use of a counting loop is unnecessary, e.g.,
regexes containing sub-expressions similar to a{0,1} or even just a{1}). Moreover, we also
removed all except 26 regexes with counters bigger than 1,000, which cannot be handled by RE2.
We left the 26 regexes as representatives of “large” counters. This left us with 5,000 regexes.
(2) Then, we �ltered out regexes R such that either CA(R) was not uniform (i.e., the CsA produced
by our algorithm was not precise, cf. Section 6.4), or such that the CsA was not fast (i.e. not
all counting-set operators were constant-time, cf. Section 6.1). After this step, a vast majority,
4,429 of the regexes, remained.
(3) For the regexes that remained, we used a lightweight ReDoS generator designed to exploit
counting (cf. Section 7.3) to generate ⇠10 MiB long input texts. In particular, we managed
to generate “adversarial” input texts for 1,789 regexes (for the rest of the regexes, either the
underlying state space was too small, so the generator could not construct the text, or the
generation hit the timeout of 600 s). Our benchmark data set is available at [Holík et al. 2020].
We ran all tools on the generated benchmarks (counting the number of lines of the input text
matching the regex) and give scatter plots comparing the running times of the tools in Fig. 5a
8 We consider the standard semantics of “matching” used by grep, i.e., a line matches a regex R if it contains a string that is

in L(R), unless it contains start-of-line (ˆ) or end-of-line ($) anchors, in which case the matched string needs to occur at
the start and/or at the end of the line respectively.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 22 -->

218:22

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

100.0

.NET
30

CA

time [s]

10

0
0.1

1.0

10.0

RE2
SRM

1.0

10.0

20

0.1

CA [s]

grep

100.0

250

500

RE2 [s]

10.0 100.0

10.0 100.0
0.1

1.0

CA [s]

10.0 100.0
0.1

1.0

1000

(b) Running times of the tools on the regex
“(_a ){k}_a” where k is a parameter

1.0

CA [s]

10.0 100.0
0.1

1.0

CA [s]

(a) The comparison of running times of CA and RE2
on our benchmark set (CA wins: 287/1,789)

0.1

750

k

0.1

1.0

10.0 100.0

0.1

1.0

10.0 100.0

grep [s]

.NET [s]

SRM [s]

(CA wins: 862/1,425)

(CA wins: 708/1,789)

(CA wins: 345/1,789)

(c) The comparison of running times of CA with grep, .NET, and SRM on our benchmark set
Fig. 5. Graphs with results of our experiments. Note that, in (a) and (c), the axes are logarithmic, the dashed
lines denote the timeout (600 s), and the data points between the dashed lines and the edge of a plot represent
benchmarks where the tool did not run successfully. We also provide the number of times CA won.

and Fig. 5c (the timeout was 600 s). On the bottom and the left-hand side of every plot, there are
rug plots illustrating the distribution of the data points. Note that the axes are logarithmic, so the
di�erence between data points grows as these points are away from zero (in particular, di�erences
of values smaller than 1 s are negligible). The semantics of regexes supported by grep di�ers from
the one supported by other tools, so we only considered the cases when the number of matches
was the same when comparing with grep). In the plots, the data points between the dashed lines
and edges of the plots represent errors, e.g. due to the regex being rejected (for counters >1,000 for
RE2) or being interpreted using a di�erent semantics (in the case of grep).
In Fig. 5a, we compare CA with RE2. We wish to point out the following interesting observations.
Although RE2 wins more often on the whole benchmark set (our prototype does not include the
many advanced optimizations present in RE2), there is a number of benchmarks (287) where its
performance signi�cantly deteriorates, and CA is faster. In particular, there are 89 benchmarks
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 23 -->

Regex Matching with Counting-Set Automata

218:23

Table 1. Statistics for the graphs in Fig. 5 (times are given in seconds). For CA, we provide several times: “total”
is the total time, “CA” is the time for translating a regex into a (nondeterministic) CA, “CsA” is the time of
determinization of the CA into a CsA, and “match” is the time spent when matching the input text.
RE2
mean
median
std. dev
timeouts

36.11
0.10
157.05
1

grep
34.38
0.70
147.17
11

.NET
9.12
0.76
52.10
8

SRM
26.78
0.73
106.16
16

CA
total

CA

CsA

match

1.73
1.03
7.27
0

0.05
0.03
0.29

0.23
0.04
2.73

0.69
0.68
0.29

where the time of RE2 is bigger than 10 s, i.e., its speed drops below 1 MiB/s (we consider this
speed of processing denotes a successful ReDoS attack, even though the limit may be signi�cantly
larger in practice9 ). For CA, the number of benchmarks that took over 10 s was only 22; in fact, all
except 3 benchmarks �nished within 100 s—the blow-up in these 3 benchmarks is not caused by
the counters but rather by many “|” and “?” operators, so over 70 % of the total time is spent by
constructing the CsA. If used, e.g., in an NIDS, the CsA would be created only once and then used
for matching giga-/terabytes of data, so the initial overhead could be neglected.
Comparing with the other tools (Fig. 5c) and also clearly visible in the corresponding rug plots
and the statistics in Table 1, we can observe that the performance of CA is much more robust than
the performance of the other tools; the mean time and standard deviation of CA is signi�cantly lower
than the rest of the tools. In particular, from the benchmarks where CA was faster than RE2, the time
of CA on all except two benchmarks was almost the same (including them, the standard deviation
was 0.37). We provide four times for CA: “total”: the total user time of matching (measured using the
GNU time utility), “CA”: the time for translating the input regex into a CA, “CsA”: the time it took
to determinize the CA into a CsA, and “match”: the time of matching the input text with the CsA.
Note that, in the tables, there is a noticeable discrepancy between the sum “CA” + “CsA” + “match”
and “total”, which is due to the .NET Framework overhead, such as just-in-time compilation and
(in particular) the garbage collector.
In Table 2, we give a selection of interesting benchmarks. These contain benchmarks that are
di�cult for usually more than one tool. We emphasize the benchmarks coming from the NIDSes
Snort and Bro. Notice that, for most of them, matching using RE2 (and also other tools) gets
extremely slow. Slow matching over these regexes can have disastrous consequences for network
security, potentially completely eliminating a given NIDS.
The CsAs produced by CA were also much smaller than the corresponding DFAs. The CsAs have
on average 29 states (median: 7) and 306 transitions (median: 11). On the other hand, classical NFAs
constructed from the regexes have on average 112 states (median: 52), and when determinized,
the resulting DFAs have on average 2,802 states (median: 67) and 10,384 transitions (median: 107).
Using CsAs signi�cantly lowers the chance that determinization explodes.
7.1.1 The E�ect of Nondeterministic Counting. We say that a regex contains nondeterministic
counting if, when translated into a CA A using the algorithm in Section 5, there is a word w such
that A can over w reach two con�gurations with di�erent values of some counter.
Regexes with nondeterministic counting are the main focus of our benchmark. Namely, they
constitute 67 % of the 1,789 regexes used. From the 1,284 regexes that were at least slightly problematic for some of the other tools except CA (it took some tool 1 s), 73 % of them were with
9 The required processing speed depends on the application. NIDSes performing deep packet inspection may require a
line-processing speed of units or tens of GiB/s [Češka et al. 2018], while application servers validating user inputs may
su�ce with units or tens of MiB/s.

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 24 -->

218:24

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

Table 2. Selection of interesting benchmarks. “TO” denotes a timeout (600 s) and “—” denotes an error. Due
to space constraints, in the “Regex” column, “. . . ” denotes omi�ed parts of the regexes (we tried to preserve
the parts containing occurrences of the repetition operator) and “⇠” denotes breaking a regex into two lines.
In the column source, Sw denotes the regexes collected in [Davis et al. 2019] from so�ware projects.
Source

Snort
Snort
Snort
Snort
Snort
Snort
Bro
Sw
Sw
Sw
Sw
Sw

Regex

RE2

grep

.*[aA][uU][tT][hH]. . . [iI][cC] ⇠
11.27
7.8
⇠[^\x0A]{512}
\x20[^\x21\x22]{500}
439.98
0.11
^RCPT TO\x20\s*[\w\s@\.]{200,}⇠
340.7
—
⇠\x20[\w\s@\.]{200,}. . .
php.*\x20[^\n]{256}
176.75
0.10
^(NT|CallBack|SID|TimeOut)\s*⇠ 164.11
0.12
⇠\x20\s*[^\n]{512}
.*[nN][eE][wW]. . . [^\x20]{100}
0.13
1.26
^[nN][aA][mM][eE]=s*[^\r\n\x3b⇠
128.57
12.24
⇠\x20\x09\x0b\x2c]{300}
_.{39}
22.96 225.34
(.{1,980}[,])\s+(\S)
260.59
TO
(_a ){64999}_a
—
—
\[{50000}a\]{50000}
—
—
^QS([NDR])(.{4})(.{6})(\d{8}). . . ⇠
0.12
0.10
⇠(.{4})(.{6})(.{8})(.{8})(.)$

.NET

SRM

CA
total

CA

CsA

match

361.1

555.56

1.04

0.03

0.05

0.31

2.20

TO

1.08

0.03

0.04

0.83

TO

TO

1.68

0.03

0.07

0.89

1.22
14.59

TO
229.41

1.08
1.07

0.04
0.03

0.07
0.07

0.74
0.72

39.92

0.74

0.81

0.03

0.04

0.65

0.51

76.48

1.15

0.03

0.04

0.94

1.94 357.68
308.66
0.63
TO
TO
4.36
TO

1.12
1.07
0.96
5.13

0.03
0.03
0.03
0.02

0.04
0.05
0.04
0.02

0.79
0.59
0.51
0.41

96.20

0.04

81.64

0.65

1.03

0.85

nondeterministic counting. From the 454 regexes that were signi�cantly problematic for some of
the other tools (it took some tool 10 s), 85 % of them had nondeterministic counting. From the 109
regexes that were problematic for all other tools ( 1 s), 100 % were with nondeterministic counting.
As shown in the results above, our approach can deal with nondeterministic counting quite well.
7.1.2 Adversarial Regexes. Another ReDoS scenario is when the attacker can control the regex
to be used for matching. Creating a counting regex causing e�ciency problems for a given text
is easier than generating adversarial texts. For instance, the regex [a-zA-Z().,’ ]*[a-zA-Z ]
[a-zA-Z(),’
˙ ]{250} was obtained as a modi�cation of the running example “ *a {k}” (where
a appears k positions from the end). When run on a ⇠4 MiB English text with su�ciently long
lines, RE2 took 86 s, grep took 26 s, while CA took only 1.1 s. Similar examples could be obtained
from regexes from Section 7.1 for which some speci�c di�cult text can be generated, namely by
widening their character classes.. Our approach solves a large class of the dangerous cases, allowing
one to signi�cantly alleviate restrictions put on the user for security/e�ciency reasons.

..

7.2 Robustness wrt Counter Values
This experiment measures the ability of the tools to cope with increasing counter bounds. For this,
we selected the regex “(_a ){k}_a” where k is a parameter (the original regex (_a ){64999}_a
comes from [Davis et al. 2019]) and measured the time the tools took on a ⇠500 KiB text created by
our generator for increasing values of k. We give the results in Fig. 5b (the timeout was 40 s).
With the increasing value of k, the time needed by CA stays constant, around 0.35 s, while the
time needed by other tools grows. In particular, .NET and SRM have cubic trends wrt the value of k,
while RE2 and grep grow linearly. Notice that, for RE2 and grep, their matching time is low (around
0.01 s) until they reach a threshold from which they start behaving linearly. This corresponds to
the situation when the size of the cache for storing states of the NFA-to-DFA construction is not
enough to accommodate the DFA states exercised by the input adversarial text. This yields repeated
�ushing of the cache, making it ine�ectual.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 25 -->

Regex Matching with Counting-Set Automata

218:25

7.3 Adversarial Text Generation
RE2 and grep store powerstates of the NFA-to-DFA construction in a cache. In typical cases, the
amount of cache misses is low and almost the entire text is processed using the cache, which is
extremely fast. If the cache, however, exceeds a given size, it is �ushed. If the input text is such that
the DFA run sees many di�erent states, then cache misses are frequent, so large powerstates need
to be constructed often, and the performance of the matching drops.
Therefore, we focus on generating texts that force exploration of many new large powerstates.
In essence, we explore the con�guration space of the CsA with the goal of �nding as many large
con�gurations as possible, with the focus on generating large counting sets. We partially drive the
search towards loops in the CsA structure that have a potential to create large counting sets: the
loops use counters with large bounds, do not contain exits, and contain R�� or R��1 operations.
For space reasons, we omit the technical details here; perfecting this method for stress testing
automata-based matchers is, however, one of our future goals.
7.4

A Note on the Maturity of the Tools

The aim of our experiments is comparing algorithms rather than tools, and it should be noted that
CA is much less optimized than the rest. This holds especially for RE2 and grep, which have both
been actively developed for over 10 years and the amount of engineering e�ort invested into making
them fast is substantial. The optimizations are both high-level, such as using the Boyer-Moore
algorithm for skipping sections of the input text, and low-level, such as using C/C++, on-the-�y
determinization, or optimizing memory accesses [Cox 2010; Haertel [n.d.]]. On the other hand,
although there have been some optimizations done in CA (such as �nding a start of a match), their
nature is still quite simple. The three tools are, however, all based on the same principle of using
deterministic automata, and many of the optimizations and heuristics in RE2 and grep (at least
all of those mentioned above) could be directly re-applied in our setting. SRM builds on the .NET
framework and reuses the .NET regex parser while replacing the built-in backtracking back-end
matcher with a matching engine based on Brzozowski-style symbolic derivatives to create the DFA
on the �y. In fact, CA builds on the open-source codebase of SRM and extends it with counters.
8 RELATED WORK
Regexes and their derivatives. Brzozowski derivatives [Brzozowski 1964] provide a practical approach to incrementally creating a DFA from a regex and can be used for e�cient matching [Fischer
et al. 2010; Owens et al. 2009] and match generation [Saarikivi et al. 2019]. E�cient determinization
based on Brzozowski derivatives was �rst investigated in [Berry and Sethi 1986]. In the classical
setting, Antimirov derivatives [Antimirov 1996] are used to construct NFAs from regexes, and
may in some cases result in exponentially more succinct automata than the corresponding DFAs
constructed with Brzozowski derivatives. The precise connection between conditional derivatives
de�ned in Section 5 and Antimirov derivatives is that, without counting loops, {D | h��, Di 2 @a (S)}
is exactly the Antimirov derivative of R for a. The Antimirov construction has also been generalized to extended regexes [Caron et al. 2011] allowing Boolean operators such as complement and
intersection. Basic theoretical properties between various automata formalisms and derivatives are
discussed in [Allauzen and Mohri 2006].
Automata with counting. This work is a continuation of our recent work [Holík et al. 2019].
In [Holík et al. 2019], we propose a general determinization of CAs that can produce smaller
automata than the naive explicit determinization but has the same worst-case complexity, which
depends on the counters with the factor (K +1) |C | where C is the set of counters and K the maximum
counter upper bound. It also proposes a more e�cient algorithm for the class of monadic regexes
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 26 -->

218:26

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

(single-state-scoped counters and counting on self-loops only), but it can still generate (K + 1) |Q |
states (e.g., it would generate K + 1 states for the regex from Fig. 1)—while the complexity of our
determinization does not depend on K. The work [Holík et al. 2019] does also not present a derivative
construction for translating regexes into CAs nor an application of CAs in pattern matching.
The use of counters has also been investigated in [Björklund et al. 2015] for regexes with bounded
repetition, building on the formalism of counter automata called CNFAs [Gelade et al. 2012]. A CA
in the current paper is essentially a symbolic generalization of a CNFA with some small technical
di�erences, such as counters being 0-based as opposed to 1-based in a CNFA. The latter di�erence is
mainly due to our use of a generalized Antimirov construction of CAs, as opposed to a generalized
Glushkov construction used in [Gelade et al. 2012], which is algorithmically quite di�erent. The
work in [Björklund et al. 2015] focuses mostly on deterministic regexes and on a di�erent problem,
namely, the so-called incremental matching in the context of database queries (a query is repeatedly
evaluated on a gradually changing word). For standard matching, it uses a variant of the Thompson’s
algorithm applied directly on a CA instead of an NFA (hence the translation of the regex to an
automaton does not depend on the counter bounds, but each text character is processed with the
same cost as with the original Thompson’s algorithm, at worst linear to the size of the NFA and the
counter bounds). This algorithm is indeed fast on deterministic regexes from practice but can slow
down signi�cantly on nondeterministic ones (which we witnessed in several experiments with the
prototype implementation of [Björklund et al. 2015] on several of our regexes).
The work in [Kilpeläinen and Tuhkanen 2003] is a theoretical study of matching regexes with
counting. It proposes a matching algorithm based on dynamic programming that runs in time
at worst quadratic to the length of the text (while determinization and NFA-simulation-based
algorithms run in time linear to the text length). The experimental comparison of [Björklund
et al. 2015] with their variant of Thompson’s algorithm suggests that the matching algorithm of
[Kilpeläinen and Tuhkanen 2003] is indeed not competitive in practice.
Extended FAs (XFAs) augment classical automata with a scratch memory of bits [Smith et al.
2008a,b] that can represent counters. Regexes are compiled into deterministic XFAs by �rst using
an extended version of the Thompson’s algorithm, followed by an extended version of the classical
powerset construction and minimization. Although a small XFA may exist, the determinization
algorithm incurs an intermediate exponential blowup of the search space for inputs such as *a {k}.
R-automata [Abdulla et al. 2008] are also related to our CAs, but their counters need not have
upper bounds and cannot be tested or compared. Further, there are various notions of extended
�nite state machines whose expressive power goes beyond regular languages, e.g., [Bardin et al.
2008; Cheng and Krishnakumar 1993; Shiple et al. 1998; Smith et al. 2008b]. Such automata are,
however, not suitable for the problem of pattern matching considered here.

..

Regexes with counting. Regexes with counters are also discussed in [Gelade et al. 2007; Hovland
2009; Kilpeläinen and Tuhkanen 2007]. The automata with counters used in [Hovland 2009], called
FACs, are close to our CAs, but we allow symbolic character predicates and more kinds of counter
updates. The conversion from regexes to FACs proposed in [Hovland 2009] uses a variant of
Glushkov automata [Glushkov 1961] and the first-last-follow construction [Berstel and Pin
1996; Brüggemann-Klein and Wood 1998]. For us, the Antimirov-derivative-based construction
was easier to implement and provides bene�ts that are not available otherwise. Namely, it allows
subsumption checking between regexes, and it generates fewer counters (one per distinct counter
sub-expression rather than one per counter position in the regex abstract syntax tree). While all
these algorithms generate -free automata, they di�er in complexity [Allauzen and Mohri 2006] and
are thus not merely di�erent disguises of the same technique. In particular, the Antimirov automaton
is in general smaller than the Glushkov automaton with up to n + 1 states and up to n 2 transitions.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 27 -->

Regex Matching with Counting-Set Automata

218:27

The Antimirov automaton is in fact a quotient of the Glushkov automaton [Champarnaud and
Ziadi 2001; Ilie and Yu 2003]. Another generalization of Antimirov derivatives [Lombardy and
Sakarovitch 2005] introduces expressions kR where R is a rational expression and k a multiplicity
from a semiring such as Q; this generalization is unrelated to counters.
An open question is whether the generalized Antimirov construction can be extended to work
with Brzozowski derivatives [Brzozowski 1964]; we believe that such an extension, if it exists, is not
straightforward because it would give rise to a direct and incremental determinization algorithm.
There are also works on regexes with counting that translate deterministic regexes to CAs and
work with di�erent notions of determinism [Chen and Lu 2015; Gelade et al. 2012]. A central
result in [Hovland 2009] is that counter-1-unambiguous regexes can be compiled into deterministic
FACs and that checking determinism of FACs can be done in polynomial time. The related work
in [Hovland 2012] studies membership in regexes with counting. None of these papers addresses
the problem of determinizing nondeterministic CAs.
Pattern matching of regexes with counting. The counting operator often appears in regexes in
practice. In particular, our analysis of the 537k real-world regexes obtained in the study performed
by Davis et al. [Davis et al. 2019] showed that over 33k regexes contained the counting operator.
GNU grep [Haertel et al. [n.d.]] (written in C) and RE2 [Google [n.d.]] (written in C++) are
extremely optimized regex matchers. Both are based on translating the regex into an NFA and
performing an on-the-�y determinization during the matching, avoiding a costly a priori determinization, while keeping a good performance by avoiding backtracking. (The translation into
FAs is only allowed when the regex does not include back-references, which allow one to express
some context-free properties). Both engines process the counting operator by �rst rewriting a
regex of the form <re>{n, m} into <re>. . . <re><re>{0,m n}. The regex <re>{0,k} is then
transformed into (<re>(<re>(. . . <re>?)?)?)? (see [Cox 2010] for more details).
In the .NET ecosystem, we are aware of two regex matchers. The �rst one is the standard .NET
regex matcher provided in System.Text.RegularExpressions, which is based on a backtracking
search. The other one is Symbolic Regex Matcher (SRM) of [Saarikivi et al. 2019] based on the socalled symbolic derivatives, which provide a backtracking-free search (without an explicit conversion
into a DFA) and can deal more e�ciently with the counting operator.
9 CONCLUSIONS AND FUTURE WORK
We have presented a framework for e�cient pattern matching of regexes with counting, which
includes a derivative construction to compile regexes to counting automata, their subsequent
determinization into novel counting-set automata, and a fast matching algorithm. The resources
needed to build the CsAs are independent of counter bounds. It handles a majority of regexes with
counting found in practice, with a much more stable performance than other matchers.
In the future, we intend to explore the limits of the idea of counting sets to enlarge and clearly
delimit the class of regexes and counting automata that can be succinctly determinized while
preserving fast matching. We also plan to explore possible usage of CsAs as a replacement of classical
automata in other applications where automata are used, for instance, as symbolic representations
of state spaces. For this, we intend to develop CsA counterparts of essential automata techniques,
such as Boolean operations and minimization/size-reduction techniques. We also wish to elaborate
on our method for generating texts for stress-testing matchers on regexes with counting.
ACKNOWLEDGMENTS
We thank the anonymous reviewers and also Juraj Síč for their valuable comments and suggestions.
This work is supported by the Czech Ministry of Education, Youth and Sports project LL1908 of the
ERC.CZ programme, and the FIT BUT internal project FIT-S-20-6427.
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 28 -->

218:28

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

REFERENCES
Parosh Aziz Abdulla, Pavel Krčál, and Wang Yi. 2008. R-Automata. In CONCUR’08 (LNCS, Vol. 5201). Springer, 67–81.
https://doi.org/10.1007/978-3-540-85361-9_9
Cyril Allauzen and Mehryar Mohri. 2006. A Uni�ed Construction of the Glushkov, Follow, and Antimirov Automata. In
Mathematical Foundations of Computer Science 2006. Springer Berlin Heidelberg, Berlin, Heidelberg, 110–121. https:
//doi.org/10.1007/11821069_10
Valentin Antimirov. 1996. Partial derivatives of regular expressions and �nite automaton constructions. Theoretical Computer
Science 155, 2 (1996), 291 – 319. https://doi.org/10.1016/0304-3975(95)00182-4
Adam Baldwin. 2016. Regular Expression Denial of Service a�ecting Express.js. http://web.archive.org/web/20170116160113/
https://medium.com/node-security/regular-expression-denial-of-service-a�ecting-express-js-9c397c164c43
Sébastien Bardin, Alain Finkel, Jérôme Leroux, and Laure Petrucci. 2008. FAST: acceleration from theory to practice. STTT
10, 5 (2008), 401–424. https://doi.org/10.1007/s10009-008-0064-3
Gerard Berry and Ravi Sethi. 1986. From regular expressions to deterministic automata. Theoretical Computer Science 48, 3
(1986), 117–126. https://doi.org/10.1016/0304-3975(86)90088-5
Jean Berstel and Jean-Éric Pin. 1996. Local languages and the Berry-Sethi algorithm. Theoret. Comput. Sci. 155, 2 (1996),
439–446. https://doi.org/10.1016/0304-3975(95)00104-2
Henrik Björklund, Wim Martens, and Thomas Timm. 2015. E�cient Incremental Evaluation of Succinct Regular Expressions.
In CIKM’15 (ACM). https://doi.org/10.1145/2806416.2806434
Robert S. Boyer and J. Strother Moore. 1977. A Fast String Searching Algorithm. Commun. ACM 20, 10 (Oct. 1977), 762–772.
https://doi.org/10.1145/359842.359859
Anne Brüggemann-Klein and Derick Wood. 1998. One-unambiguous regular languages. Information and Computation 140,
2 (1998), 229–253. https://doi.org/10.1006/inco.1997.2695
Janusz A. Brzozowski. 1964. Derivatives of Regular Expressions. J. ACM 11, 4 (1964), 481–494. https://doi.org/10.1145/
321239.321249
Pascal Caron, Jean-Marc Champarnaud, and Ludovic Mignot. 2011. Partial Derivatives of an Extended Regular Expression.
In Language and Automata Theory and Applications. Springer Berlin Heidelberg, Berlin, Heidelberg, 179–191. https:
//doi.org/10.1007/978-3-642-21254-3_13
Jean-Marc Champarnaud and Djelloul Ziadi. 2001. Computing the equation automaton of a regular expression in O (s 2 ) space
and time. In Proceedings of CPM 2001 (LNCS, Vol. 2089). Springer, 157–168. https://doi.org/10.1007/3-540-48194-X_15
Haiming Chen and Ping Lu. 2015. Checking determinism of regular expressions with counting. Information and Computation
241 (2015), 302 – 320. https://doi.org/10.1016/j.ic.2014.12.001
Kwang-Ting Cheng and A. S. Krishnakumar. 1993. Automatic Functional Test Generation Using the Extended Finite State
Machine Model. In Proceedings of the 30th Design Automation Conference. Dallas, Texas, USA, June 14-18, 1993. ACM Press,
86–91. https://doi.org/10.1145/157485.164585
Wikipedia contributors. 2019. Regular expression—Wikipedia. https://en.wikipedia.org/w/index.php?title=Regular_
expression&%20oldid=852858998
Russ Cox. 2010. Regular Expression Matching in the Wild. https://swtch.com/~rsc/regexp/regexp3.html.
Loris D’Antoni and Margus Veanes. 2020. Automata Modulo Theories. Commun. ACM (2020).
James C. Davis. 2019. Rethinking Regex Engines to Address ReDoS. In Proceedings of ESEC/FSE’19 (Tallinn, Estonia) (ESEC/FSE
2019). ACM, New York, NY, USA, 1256–1258. https://doi.org/10.1145/3338906.3342509
James C. Davis, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. 2018. The Impact of Regular Expression Denial
of Service (ReDoS) in Practice: An Empirical Study at the Ecosystem Scale. In Proceedings of ESEC/FSE’18 (Lake Buena
Vista, FL, USA) (ESEC/FSE 2018). ACM, New York, NY, USA, 246–256. https://doi.org/10.1145/3236024.3236027
James C. Davis, Louis G. Michael IV, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. 2019. Why Aren’t Regular
Expressions a Lingua Franca? An Empirical Study on the Re-use and Portability of Regular Expressions. In Proceedings of
ESEC/FSE’19 (Tallinn, Estonia) (ESEC/FSE 2019). ACM, New York, NY, USA, 1256–1258. https://doi.org/10.1145/3338906.
3338909
Stack Exchange. 2016. Outage Postmortem. http://stackstatus.net/post/147710624694/outage-postmortem-july-20-2016
Sebastian Fischer, Frank Huch, and Thomas Wilke. 2010. A Play on Regular Expressions: Functional Pearl. SIGPLAN Not. 45,
9 (2010), 357–368. https://doi.org/10.1145/1863543.1863594
Wouter Gelade, Marc Gyssens, and Wim Martens. 2012. Regular Expressions with Counting: Weak versus Strong Determinism. SIAM J. Comput. 41, 1 (2012), 160–190. https://doi.org/10.1137/100814196 Extended version of paper in
MFCS’09.
Wouter Gelade, Wim Martens, and Frank Neven. 2007. Optimizing schema languages for XML: Numerical constraints and
interleaving. In Proceedings of ICDT’07 (LNCS, Vol. 4353). Springer, 269–283. https://doi.org/10.1007/11965893_19
V. M. Glushkov. 1961. The abstract theory of automata. Russian Math. Surveys 16 (1961), 1–53. https://doi.org/10.1070/
RM1961v016n05ABEH004112
Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 29 -->

Regex Matching with Counting-Set Automata

218:29

Google. [n.d.]. RE2. https://github.com/google/re2.
John Graham-Cumming. 2019. Details of the Cloud�are outage on July 2, 2019. https://blog.cloud�are.com/details-of-thecloud�are-outage-on-july-2-2019/
Mike Haertel. [n.d.]. why GNU grep is fast. https://lists.freebsd.org/pipermail/freebsd-current/2010-August/019310.html.
Mike Haertel et al. [n.d.]. GNU grep. https://www.gnu.org/software/grep/.
Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Lenka Turoňová, Margus Veanes, and Tomáš Vojnar. 2019. Succinct Determinisation of Counting Automata via Sphere Construction. In Proc. of APLAS’19 (LNCS, Vol. 11893). Springer, 468–489.
https://doi.org/10.1007/978-3-030-34175-6_24
Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Lenka Turoňová, Margus Veanes, and Tomáš Vojnar. 2020. Dataset for the
OOPSLA’20 paper “Regex Matching with Counting-Set Automata”. https://doi.org/10.5281/zenodo.3974360
Dag Hovland. 2009. Regular Expressions with Numerical Constraints and Automata with Counters. In ICTAC (LNCS,
Vol. 5684). Springer, 231–245. https://doi.org/10.1007/978-3-642-03466-4_15
Dag Hovland. 2012. The Membership Problem for Regular Expressions with Unordered Concatenation and Numerical
Constraints. In Language and Automata Theory and Applications. Springer Berlin Heidelberg, Berlin, Heidelberg, 313–324.
https://doi.org/10.1007/978-3-642-28332-1_27
Lucian Ilie and Sheng Yu. 2003. Follow automata. Information and Computation 186, 1 (2003), 146–162. https://doi.org/10.
1016/S0890-5401(03)00090-7
Pekka Kilpeläinen and Rauno Tuhkanen. 2003. Regular Expressions with Numerical Occurrence Indicators - preliminary
results. In Proceedings of the Eighth Symposium on Programming Languages and Software Tools, SPLST’03, Kuopio, Finland,
June 17-18, 2003. University of Kuopio, Department of Computer Science, 163–173.
Pekka Kilpeläinen and Rauno Tuhkanen. 2007. One-unambiguity of regular expressions with numeric occurrence indicators.
Information and Computation 205, 6 (2007), 890–916. https://doi.org/10.1016/j.ic.2006.12.003
Sylvain Lombardy and Jacques Sakarovitch. 2005. Derivatives of rational expressions with multiplicity. Theoretical Computer
Science 332, 1 (2005), 141 – 177. https://doi.org/10.1016/j.tcs.2004.10.016
M. Roesch et al. [n.d.]. Snort: A Network Intrusion Detection and Prevention System,. http://www.snort.org.
Microsoft. 2020. . https://docs.microsoft.com/en-us/dotnet/api/system.text.regularexpressions.regex.match
Scott Owens, John Reppy, and Aaron Turon. 2009. Regular-expression Derivatives Re-examined. J. Funct. Program. 19, 2
(2009), 173–190. https://doi.org/10.1017/S0956796808007090
Mono project. [n.d.]. Mono. https://www.mono-project.com/.
RegExLib.com. [n.d.]. The Internet’s �rst Regular Expression Library, . http://regexlib.com/.
Robin Sommer et al. [n.d.]. The Bro Network Security Monitor. http://www.bro.org.
Olli Saarikivi, Margus Veanes, Tiki Wan, and Eric Xu. 2019. Symbolic Regex Matcher. In TACAS’2019 (LNCS, Vol. 11427),
Tomás Vojnar and Lijun Zhang (Eds.). Springer, 372–378. https://doi.org/10.1007/978-3-030-17462-0_24
Thomas R. Shiple, James H. Kukula, and Rajeev K. Ranjan. 1998. A Comparison of Presburger Engines for EFSM Reachability.
In Computer Aided Veri�cation, 10th International Conference, CAV ’98, Vancouver, BC, Canada, June 28 - July 2, 1998,
Proceedings (Lecture Notes in Computer Science, Vol. 1427). Springer, 280–292. https://doi.org/10.1007/BFb0028752
Michael Sipser. 2006. Introduction to Theory of Computation. Vol. 2. Thomson Course Technology Boston. https://doi.org/10.
1145/230514.571645
Randy Smith, Cristian Estan, and Somesh Jha. 2008a. XFA: Faster Signature Matching with Extended Automata. In IEEE
Symposium on Security and Privacy. IEEE. https://doi.org/10.1109/SP.2008.14
Randy Smith, Cristian Estan, Somesh Jha, and Ida Siahaan. 2008b. Fast Signature Matching Using Extended Finite Automaton
(XFA). In ICISS’08 (LNCS, Vol. 5352). Springer, 158–172. https://doi.org/10.1007/978-3-540-89862-7_15
Henry Spencer. 1994. Software Solutions in C. Academic Press Professional, Inc., San Diego, CA, USA, Chapter A Regularexpression Matcher, 35–71. http://dl.acm.org/citation.cfm?id=156626.184689
Michael Sperberg-McQueen. [n.d.]. Notes on �nite state automata with counters. https://www.w3.org/XML/2004/05/msmcfa.html Accessed: 2018-08-08.
The Sagan team. [n.d.]. The Sagan Log Analysis Engine. https://quadrantsec.com/sagan_log_analysis_engine/.
Ken Thompson. 1968. Programming Techniques: Regular Expression Search Algorithm. Commun. ACM 11, 6 (June 1968),
419–422. https://doi.org/10.1145/363347.363387
Lenka Turoňová, Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Margus Veanes, and Tomáš Vojnar. [n.d.]. Automata library.
https://pajda.�t.vutbr.cz/ituronova/countingautomata.
Lenka Turoňová, Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Margus Veanes, and Tomáš Vojnar. 2020. Regex Matching with
Counting-Set Automata. Technical Report MSR-TR-2020-31. Microsoft. https://doi.org/10.5281/zenodo.3975566
Milan Češka, Vojtěch Havlena, Lukáš Holík, Ondřej Lengál, and Tomáš Vojnar. 2018. Approximate Reduction of Finite
Automata for High-Speed Network Intrusion Detection. In Proc. of TACAS’18 (LNCS, Vol. 10806). Springer. https:
//doi.org/10.1007/978-3-319-89963-3_9

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.

<!-- page 30 -->

218:30

L. Turoňová, L. Holík, O. Lengál, O. Saarikivi, M. Veanes, and T. Vojnar

Margus Veanes, Peli de Halleux, and Nikolai Tillmann. 2010. Rex: Symbolic Regular Expression Explorer. In Third International
Conference on Software Testing, Veri�cation and Validation, ICST 2010, Paris, France, April 7-9, 2010. 498–507. https:
//doi.org/10.1109/ICST.2010.15
Liu Yang, Rezwana Karim, Vinod Ganapathy, and Randy Smith. 2010. Improving NFA-Based Signature Matching Using
Ordered Binary Decision Diagrams. In Recent Advances in Intrusion Detection. Springer Berlin Heidelberg, Berlin,
Heidelberg, 58–78. https://doi.org/10.1007/978-3-642-15512-3_4

Proc. ACM Program. Lang., Vol. 4, No. OOPSLA, Article 218. Publication date: November 2020.
