---
file: ../pdfs/2025-arxiv-amarilli-subsequence-supersequence-regex-matching.pdf
citation: "Linear Time Subsequence and Supersequence Regex Matching; Amarilli, Dudek, Manea, Ringleb, Schmid; arXiv:2504.16288; 2025"
doi: 
source: "https://arxiv.org/pdf/2504.16288"
topics: [semantics_theory_access]
pages: 33
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Linear Time Subsequence and Supersequence Regex
Matching
Antoine Amarilli1 , Bartlomiej Dudek2 , Florin Manea3 , Tina Ringleb3 , and Markus
L. Schmid4

arXiv:2504.16288v3 [cs.DS] 12 Jun 2026

1

Univ. Lille, Inria, CNRS, Centrale Lille, UMR 9189 CRIStAL, F-59000 Lille,
France, a3nm@a3nm.net
2
University of Wroclaw, Institute of Computer Science, Joliot-Curie 15, PL-50-383
Wroclaw, Poland, bartlomiej.dudek@cs.uni.wroc.pl
3
University of Göttingen, Institute for Computer Science and CIDAS, D-37077
Göttingen, Germany, {florin.manea, tina.ringleb}@cs.uni-goettingen.de
4
Humboldt-Universität zu Berlin, Unter den Linden 6, D-10099, Berlin, Germany,
MLSchmid@MLSchmid.de
June 15, 2026
Abstract
It is well-known that checking whether a given string w matches a given regular expression r can be done in quadratic time O(|w| · |r|) and that this cannot be improved to a truly
subquadratic running time of O((|w|·|r|)1−ϵ ) assuming the strong exponential time hypothesis
(SETH). We study the related problem that asks whether w has a subsequence that matches r,
and we show that surprisingly this task admits an algorithm that runs in linear time, i.e.,
in O(|w| + |r|). We further show that the same holds if we ask for a supersequence instead
of a subsequence. Moreover, we show that the quantitative problems of computing a longest
subsequence or shortest supersequence of w that matches r can be solved with the same
complexity as the classical longest common subsequence or shortest common supersequence
problems, i.e., in O(|w| · |r|), and conditionally not in O((|w| · |r|)1−ϵ ).
By contrast, if instead of subsequences or supersequences we consider other string relations
like the infix, prefix, left-extension, or extension relations, then all the corresponding problems (both quantitative and non-quantitative) have the same complexity as classical regex
matching, i.e., they can also be solved in O(|w| · |r|), but not in O((|w| · |r|)1−ϵ ) assuming
SETH.
We last study the complexity of the universal problem that asks if all subsequences (or supersequences, infixes, prefixes, left-extensions or extensions) of an input string satisfy a given
regular expression. For these problems, we show polynomial upper bounds (along with matching conditional lower bounds) for the infix and prefix relations, but PSPACE-completeness
for the extension, left-extension and supersequence relations, and coNP-completeness for the
subsequence relation.

1

Introduction

Regular expressions (also called regex) were first introduced by Kleene in 1956 [Kle56] as a theoretical concept and quickly found their way into practice with the classical construction by Thompson [Tho68]. Nowadays, they are a standard tool for text processing and data retrieval tasks, and
they constitute computational primitives in virtually all modern programming languages (see the

1

<!-- page 2 -->

standard textbook [Fri06]). The most important problem about regular expressions is their matching problem, i.e., checking whether a given regular expression r matches a given string w. One
of the two main algorithmic approaches to this problem is still Thompson’s original construction:
transform r into a nondeterministic finite automaton with ε-transitions (or εNFA) A in linear
time, and then simulate A on w in time O(|w| · |A|). (The other approach is to transform r into
a deterministic finite automaton or DFA, whose size is generally exponential in r.)
In addition to their well-known applications as text analysis tools, regular expressions are also
used in many different areas and are at the core of several data management principles: graph
databases [AAB+ 17], information extraction [FKRV15], complex event processing [GRUV21], network monitoring [LN19], financial fraud detection [SK15], infrastructure security [KCTV07], etc.
The problem of regular expression matching (and the strongly related problem of NFA acceptance)
is also one of the base problems investigated in fine-grained complexity [BI16, BGKL24, BGL17].
A classical result in this area is that the long-standing quadratic upper bound of O(|w| · |r|) for
matching is optimal in the sense that it cannot be improved to truly subquadratic complexity of
O((|w| · |r|)1−ϵ ) unless the strong exponential time hypothesis (SETH) fails: see [BI16].1
We contribute to the research on regex matching by exploring the following new angle. Instead of asking whether the whole input string w matches the regex r, we ask whether w has a
subsequence that matches r; or a supersequence that matches r; or, more generally, whether r
matches some string u with u ⪯ w, where ⪯ is a fixed string relation (and classical regex matching
is then the case when ⪯ is the equality relation). While technically any possible string relation
⪯ instantiates a ⪯-version of the regex matching problem, we focus on relations that are relevant
for string matching: the subsequence relation (x1 x2 . . . xn ⪯sub w ⇔ w = w0 x1 w1 x2 . . . xn wn ),
supersequence relation (u ⪯sup w ⇔ w ⪯sub u), infix relation (u ⪯in w ⇔ w = vuv ′ ), prefix relation (u ⪯pre w ⇔ w = uv), left-extension relation (u ⪯lext w ⇔ u = vw), and extension relation
(u ⪯ext w ⇔ w ⪯in u).

1.1

Our Results

Our main focus is on the subsequence and supersequence relations, and our first contribution is
to show that the complexity of regular expression matching can be substantially improved when
performed with these relations. Namely, we show that given a regex r and string w, we can check
in time O(|w| + |r|) whether some subsequence of w matches r, or whether some supersequence
of w matches r. We believe that this tractability result is quite surprising, and it turns out to be
in strong contrast to the same problem for other string relations (infix, prefix, left-extension and
extension), or for regex matching in the usual sense. Indeed, in all these cases, we show that the
complexity of O(|w| · |r|) is optimal in the sense explained above (assuming SETH).
Our linear time upper bound for subsequence and supersequence matching is achieved by
transforming the regex r into an εNFA A that accepts w if and only if w has a subsequence (or
supersequence) that matches r, following a known construction in the context of so-called upward
and downward closures [BLS15]. We then exploit special properties of A that allow us to decide
whether it accepts w in linear time. It is then not very difficult to improve the bound and make
it linear in the input word: however, more effort is needed to also ensure that the complexity is
linear in the regex (in particular in the setting where the input alphabet is not constant), and
achieve the optimal complexity of O(|w| + |r|).
Motivated by this positive algorithmic result, we investigate a natural generalisation of the
matching problem: compute a maximum-length/minimum-length string u with u ⪯ w that
matches r. However, it turns out that our linear time algorithm for subsequence and supersequence regex matching does not generalise to this stronger problem, and we can in fact show that
subquadratic algorithms for these min- and max-variants are conditionally not possible. More
precisely, for the max-variant of the subsequence relation or the min-variant of the supersequence
relation, we show that truly subquadratic algorithms would imply truly subquadratic algorithms
1 Note that a running time O(m · n) for input sizes m and n is also called rectangular in the literature to
distinguish from a quadratic running time of the form O(n2 ). In this paper, we use nevertheless the term quadratic
for running time O(m · n) and subquadratic for running time O((m · n)1−ϵ ).

2

<!-- page 3 -->

subsequence
supersequence
infix
prefix
extension
left-extension

⪯-matching

min/max-variant

universal-variant

O(|w| + m)
O(|w| + m)
O(|w|m)
O(|w|m)
O(|w|m)
O(|w|m)

O(|w|m)
O(|w|m)
O(|w|m)
O(|w|m)
O(|w|m)
O(|w|m)

coNP
PSPACE
O(|w|2 m)
O(|w|m)
PSPACE
PSPACE

Table 1: Upper bounds for the different problem variants. The problem ⪯-matching is to check
whether there is a string u ⪯ w that matches the regex, where ⪯ is one of the string relations
displayed in the first column; the min/max-variant computes a smallest/longest such string, and
the universal variant checks if all strings u ⪯ w match the regex. Note that m is the size of the
εNFA (or, equivalently, the regular expression).
⪯-matching
subsequence
supersequence
infix
prefix
extension
left-extension

min/max-variant
1−ϵ

—
—
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )

no O((|w|m) )
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )
no O((|w|m)1−ϵ )

universal-variant
coNP-hard
PSPACE-hard
no O(|w|2−ϵ poly(m))
no O((|w|m)1−ϵ )
PSPACE-hard
PSPACE-hard

Table 2: Conditional lower bounds for the different problem variants. All lower bounds are
conditional on SETH. Note that m is the size of the εNFA (or, equivalently, the regular expression).
for the longest common subsequence or the shortest common supersequence problems, which is
impossible under SETH (see [BK15, ABW15]). For the min-variant of the subsequence relation or
the max-variant of the supersequence relation, we show that a truly subquadratic algorithm would
imply a faster algorithm for the Orthogonal Vectors problem, which is also impossible under SETH
[ABDN18, Wil05]. On the positive side, however, we show that all these problems can be solved
within the original matching complexity of O(|w| · |r|), in particular generalising the fact that
longest common subsequences and shortest common supersequences can be computed within this
bound. We also show that the same upper bounds apply for the infix, prefix, and (left-)extension
relations, and that they are all optimal under SETH.
In this regard, it is worth noting that both the max-variant for the subsequence relation and the
min-variant for the supersequence relation properly extend the classical regex matching problem,
since a matching maximum-length subsequence u or minimum-length supersequence v equals w if
and only if w ∈ L(r). In fact, we could interpret u and v as a fuzzy measure for regex matching:
The tighter w is sandwiched in between u and v, the more it can be interpreted as matching the
regex r, where the optimum u = v = w is reached when w ∈ L(r).
Finally, we investigate the complexity of checking whether all strings u with u ⪯ w match r.
While it is easy to see that this can be solved efficiently for the infix and prefix relations, it becomes
intractable for the (left-)extension, subsequence and supersequence relations. We pinpoint the
complexity of all these variants (conditionally on SETH).
See Tables 1 and 2 for an overview of all our upper and lower complexity bounds.

1.2

Motivations and Related Work

Subsequences and supersequences play an important role in many different areas of theoretical
computer science: in formal languages and logics (e.g., piecewise testable languages [Sim72, Sim75,
KS19, PSVV24], or subword order and downward closures [HSZ17, KZ19, Kus20, Zet16]), in
combinatorics on words [RS15, FGK+ 15, Sek12, MSY04, Sal05, SV23] or combinatorial pattern
3

<!-- page 4 -->

matching [Hir77, HS77, Mai78, Sim03, DFK+ 25, GKK+ 21], for modelling concurrency [Rid79,
Sha78, BS14], in fine-grained complexity [BC18, BK18, ABW15, AWW14, PW25], etc. See also the
surveys [BHR00, CMT03, KKMS22] and the references therein. Moreover, algorithmic problems
related to the analysis of the set of subsequences of the strings of a formal language, given as a
grammar or automaton, are studied in [AFH+ 25] and [FKM+ 24]. Closer to our topic, matching
regular expressions to the subsequences of a string is an important paradigm in event stream
processing, where we receive a stream of events that result from different processes, and where
consecutive events from the same process are represented as a subsequence in this string (see, e.g.,
[AMU+ 17, GAA+ 20, ZDI14, KSS+ 22, KSS+ 23, FK23, GRUV21]).
Our results can also be interpreted as providing a new angle to study the fine-grained complexity of the regex matching problem. In particular, the classical paper of Backurs and Indyk [BI16],
which initiated the study of the fine-grained complexity of regex matching, investigates the following question: for which regex classes can the quadratic upper bound for matching be improved,
ideally to linear time O(|w| + |r|), and for which classes is this impossible (assuming SETH).
This question is further studied in [BGL17]. In these works, the restricted classes are defined as
homogeneous regex, i.e., where the operators used at each depth of the formula are all equal. In
the present paper, we carry out a similar study, but instead of restricted regex classes, we consider different matching paradigms: we identify string relations for which regex matching becomes
linear time solvable and others for which this is conditionally not the case. However, we can
also understand our results as giving a linear time bound on the class of regex defining so-called
upward-closed and downward-closed languages, as we will explain later.
In addition to investigating how the regex matching problem could be restricted in order to
obtain subquadratic variants (either classically by restricting the class of input regex or, as we do,
by restricting the matching paradigm), it is also interesting to ask what kind of other problems
for a given regular expression and a string can be solved within the original matching complexity
of O(|w| · |r|). In this regard, our results demonstrate that in addition to checking w ∈ L(r) there
are many other relevant tasks that can be solved without an increase in complexity: checking
w for an infix, prefix, left-extension or extension that matches r, and computing a longest or
shortest subsequence, supersequence, infix, prefix, left-extension or extension of w that matches
r. Furthermore, checking if all prefixes of w match r can also be done in quadratic time, but for
infixes, this becomes cubic, while for subsequences, supersequences and extensions we even obtain
intractability. The relevance of these upper bounds is also substantiated by the tight (conditional)
matching lower bounds that we show to complement them.
To prove our linear upper bound for the subsequence regex matching problem, we first transform the regular expression into an εNFA Asub that accepts the upward closure of L(r), i.e., the
set {u | ∃v ∈ L(r) : v ⪯sub u} (note that w has a subsequence that matches r if and only if
w ∈ L(Asub )). Similarly, the upper bound for the supersequence regex matching problem is based
on constructing an εNFA Asup for the downward closure {u | ∃v ∈ L(r) : v ⪯sup u}. The downward
and upward closures are well-investigated concepts in formal language theory (see citations above).
In particular, it has been noted in [BLS15, Lemma 9] that the powerset-DFA of Asub always transitions from a state set to a superset of this state set (analogously, the powerset-DFA of Asup always
transitions from a state set to a subset of this state set). However, this property does not imply that
the powerset-DFA of Asub and Asup is necessarily small, since [Okh10] proves an exponential lower
bound on their number of states. Our notion of subsequence or supersequence matching is related
to so-called upward-closed and downward-closed languages (which are languages equal to their
upward closure or downward closure, respectively), because for such languages the usual notion
of regex matching coincides with subsequence and supersequence matching, respectively. These
languages have been investigated, e.g., for downward-closed languages in [GSZ24] or in [ABJ98]
(under the name “simple regular expressions”) and for upward-closed languages in [GLHK+ 20].
However, to our knowledge, none of the works focusing on the upward or downward closure, or on
upward-closed or downward-closed languages, have investigated the complexity of the matching
problem like we do. Thus it was not previously known that these problems could be solved in time
O(|w| + |r|), or indeed that their complexity was lower than that of standard regex matching.

4

<!-- page 5 -->

1.3

Paper Structure

In Section 2, we give preliminaries and formally define the matching problems that we study. In
Section 3, we first recall some basics about the state-set simulation of εNFAs and then explain how
our regex matching problems for the subsequence and supersequence relation reduce to the state-set
simulation of certain εNFAs. Then, in Section 4, we give the corresponding linear time algorithms.
Sections 5 and 6 are respectively concerned with the quantitative and universal problem variants
for all considered string relations. The conditional lower bounds that complement our upper
bounds are discussed in Section 7.
The present paper is the full version of the conference paper [AMRS25]. It includes all details
and the complete proofs. Furthermore, it also contains improvements of the lower bounds for
the min-variant of the subsequence matching problem, for the max-variant of the supersequence
matching problem, and for the universal variant of the infix matching problem. These improvements ensure that all complexity upper bounds given in the present version of the paper are now
complemented by matching lower bounds (assuming SETH).

2

Preliminaries and Problem Statement

2.1

Strings, Regular Expressions and Automata

We let Σ be a finite alphabet of symbols (sometimes called letters) and write Σ∗ for the set of
strings (sometimes called words) over Σ. We write ε for the empty string. For a string w ∈ Σ∗ , we
denote by |w| its length and, for every i ∈ {1, 2, . . . , |w|}, we denote by w[i] the ith symbol of w. For
i, j ∈ {1, 2, . . . , |w|} with i ≤ j, we denote by w[i : j] the factor (also called infix) w[i]w[i+1] · · · w[j];
in particular, w[i : i] = w[i]. Further, for every letter a ∈ Σ we define ∥w∥a as the number of
occurrences of letter a in the string w ∈ Σ∗ . Lastly, we denote by wR = w[|w|]w[|w| − 1] · · · w[1]
the reverse of w.
Regular expressions over Σ are defined as follows. The empty set ∅ is a regular expression
with L(∅) = ∅, and every x ∈ Σ ∪ {ε} is a regular expression with L(x) = {x}. If s and t
are regular expressions, then s · t, s ∨ t, and s∗ are regular expressions with L(s · t) = L(s) · L(t),
L(s∨t) = L(s)∪L(t), and L(s∗ ) = (L(s))∗ . Here, we S
define as usual: L1 ·L2 = {uv | u ∈ L1 , v ∈ L2 },
L0 = {ε}, Lk = Lk−1 · L for every k ≥ 1, and L∗ = k≥0 Lk . The length |s| of a regular expression
s is the total number of symbols of s considered as a string.
We work with nondeterministic finite automata with ε-transitions, called εNFA for brevity.
An εNFA is a tuple A = (Q, Σ, q0 , qf , δ) where Q is a finite set of states, q0 is the initial state, qf
is the final state, and δ ⊆ Q × (Σ ∪ {ε}) × Q is the set of transitions. A transition of the form
(p, a, q) is called an a-transition. We will also interpret an εNFA A as a graph which has vertex
set Q and which has directed edges labelled by symbols from Σ ∪ {ε} given by the transitions of
δ, i.e., any transition (p, a, q) ∈ δ is interpreted as a directed edge from p to q labelled by a. A
transition (p, a, p) with p ∈ Q and a ∈ Σ ∪ {ε} is called a self-loop. A run of A on an input string
w ∈ Σ∗ is a path (not necessarily simple) from q0 to some state p which is labelled with w, where
the label of a path is just the concatenation of all Σ-labels (ignoring ε-labels). A run is accepting
if p = qf . We write L(A) for the language accepted by A, i.e., the set of all strings for which A has
an accepting run. The size |A| of A is its number of transitions: we usually denote it by m, while
n denotes the number |Q| of states.
It is well-known that a given regular expression r can be converted in time O(|r|) into an εNFA
A such that L(A) = L(r) and |A| = O(|r|). This can be achieved, e.g., using Thompson’s construction [HMU01, Section 3.2.3]. Thus, in the sequel, we assume that all input regular expressions are
given as εNFAs, and we state our results directly for arbitrary εNFAs.
Moreover, we assume that our εNFAs are trimmed, which means that every state is reachable
from q0 and every state can reach qf . This can always be ensured in time O(m). If an εNFA is
trimmed, then we also have that the number of states of the input automaton is at most twice
the number of transitions plus one. We also assume that for each x ∈ Σ we have at least one
transition labelled with x, which means that |Σ| is in O(m).
5

<!-- page 6 -->

2.2

String Relations

A string relation (over Σ) is a subset of Σ∗ × Σ∗ . For any string relation ⪯ and w ∈ Σ∗ we
define Λ⪯ (w) = {u ∈ Σ∗ | u ⪯ w}, i.e., the set of all strings
S that are in the ⪯ relation to w; we
also lift this notation to languages L ⊆ Σ∗ , i.e., Λ⪯ (L) = w∈L Λ⪯ (w). Next, we define several
well-known string relations.2
The prefix and infix relations are denoted by ⪯pre and ⪯in : formally, we write u ⪯pre w when
uv = w for some v ∈ Σ∗ and we write u ⪯in w when vuv ′ = w for some v, v ′ ∈ Σ∗ . The leftextension and extension relations are denoted by ⪯lext and ⪯ext : formally, we write u ⪯lext w when
u = vw for some v ∈ Σ∗ and write u ⪯ext w when w ⪯in u. The subsequence and supersequence
relations are denoted by ⪯sub and ⪯sup : we write u ⪯sub w when w[j1 ]w[j2 ] . . . w[j|u| ] = u for some
1 ≤ j1 < j2 < . . . < j|u| ≤ |w|, and write u ⪯sup w when w ⪯sub u. Note that we do not study
the suffix and right-extension relations because they amount to prefix and left-extension up to
mirroring the strings.

2.3

Problem Statement: The ⪯-Matching Problem

The well-known regex matching problem is to decide whether w ∈ L(r) for a given regular expression r and input string w. As explained above, this generalises to the εNFA acceptance problem,
where we want to decide whether w ∈ L(A) for a given εNFA A and input string w. In this paper,
we study the ⪯-matching problem, where ⪯ is one of the string relations presented above: For a
given string w ∈ Σ∗ and an εNFA A, decide whether A accepts a string u with u ⪯ w, i.e., decide
whether Λ⪯ (w) ∩ L(A) ̸= ∅.
We also define quantitative variants of this matching problem. For a string relation ⪯, the
min-variant of the ⪯-matching problem is as follows: For a given string w ∈ Σ∗ and an εNFA A,
compute a shortest string u with u ⪯ w and u ∈ L(A), or report that no such string u exists. The
max-variant of the ⪯-matching problem is as follows: For a given string w ∈ Σ∗ and an εNFA A,
either report that there are arbitrarily long strings u with u ⪯ w and u ∈ L(A), or compute
a longest string u with u ⪯ w and u ∈ L(A), or report that no such string u exists. Further,
all lower bounds on the quantitative variants of the matching problem will already apply in the
setting where we are only required to compute the length of the witnessing string.
Finally, we define a universal variant of the matching problem. For a string relation ⪯, the
universal-variant of the ⪯-matching problem is as follows: For a given string w ∈ Σ∗ and an
εNFA A, decide whether A matches all strings u with u ⪯ w, i.e., decide the inclusion problem
Λ⪯ (w) ⊆ L(A).
Finally, note that if we choose ⪯ to be the string equality relation, then all the problem variants
from above are just the same as the classical regex matching problem.

2.4

Computational Model and Basic Input-Preprocessing

The computational model we use to state our algorithms is the standard unit-cost word RAM
with logarithmic word-size ω (meaning that each memory word can hold ω bits). It is assumed
that this model allows processing inputs of size n, where ω ≥ log n; in other words, the size n of
the input never exceeds (but, in the worst case, is equal to) 2ω . Intuitively, the size of the memory
word is determined by the processor, and larger inputs require a stronger processor (which can,
of course, deal with much smaller inputs as well). Indirect addressing and basic arithmetical
or bitwise operations on such memory words are assumed to work in constant time. This is a
standard computational model for the analysis of algorithms, defined in [FW93]. To make some
of our algorithms faster, it may be necessary to allocate large arrays in constant time: for this, we
use the standard technique of lazy initialisation [GJ23] to avoid spending linear time in the array
size to initialise its cells. The time needed, after the lazy initialisation, to store a value in a cell
2 Our generalised regex matching setting works for any string relation, but those that we study are all reflexive,
transitive and antisymmetric; hence, we use the symbol ⪯.

6

<!-- page 7 -->

of the array, or to check if a cell of the array was initialised and if so return the value it stores, is
O(1).
In each of our problems, the input always consists of an εNFA and a string over Σ. As we
are aiming for low complexities, it is important to define formally how the input is written: we
will always assume that it obeys the following assumptions. For an εNFA A = (Q, Σ, q0 , qf , δ)
with |Q| = n, |Σ| = σ, and m = |δ|, we assume that Q = {1, . . . , n} and Σ = {1, . . . , σ}; we can
thus assume that both Q and Σ are ordered sets, w.r.t. the canonical order on natural numbers.
It follows that the processed strings are sequences of integers (representing the symbols), each of
these integers fitting in one memory word. This is a common assumption in string algorithms:
the input alphabet is said to be an integer alphabet (see, e.g., [CHL07]). Due to the assumptions
about εNFAs made above, we also know that n, σ are in O(m). Moreover, such an automaton A
is given by its number of states, size of the input alphabet, initial and final state, and a list of m
transitions of the form (q, a, q ′ ), with q, q ′ ∈ Q and a ∈ Σ ∪ {ε}.
Further, using radix sort, we can prepare for each state q the list L[q] of outgoing transitions,
sorted first according to the symbol labelling them, and then by the target state; for states without
outgoing transitions we set L[q] ← ∅. This allows us to simultaneously construct, for each q ∈ Q
and a ∈ Σ ∪ {ε} for which q has an outgoing a-transition, the list T [q, a] of transitions of the
form (q, a, q ′ ); T [q, a] is undefined for q ∈ Q and a ∈ Σ ∪ {ε} in the case where q has no outgoing
a-transition. We will make the convention that, in the case that a self-loop (q, a, q) exists, then
(q, a, q) is the first transition stored in T [q, a]; this allows us to test in O(1) whether there exists
a self-loop (q, a, q) for any q ∈ Q and a ∈ Σ ∪ {ε}. Both L[·] and T [·, ·] are implemented as arrays
(of lists), which are lazily initialised (as mentioned above). So, computing the defined elements of
L[·] and T [·, ·] can be done in linear time, i.e., in O(m).

3

State-Set Simulation and Simple Upper Bounds

Given a string w ∈ Σ∗ and a regular expression represented as an εNFA A, we can solve the
regex matching problem of deciding whether w ∈ L(A) using the classical approach of state-set
simulation. It is well-known [BI16] that, assuming the strong exponential time hypothesis (SETH),
this yields an optimal O(|w|·|A|) algorithm. For our variants of regex matching, it is often possible
to build an εNFA A′ from A that accepts suitable strings (e.g., the subsequences or supersequences
of strings of L(A)), so that we can solve the matching problem by checking whether w ∈ L(A′ )
by state-set simulation (though this is generally not optimal). Since many of our algorithms will
nevertheless be specialised variants of state-set simulation, we review the technique in more detail
below.

3.1

State-Set Simulation for εNFA

For an NFA A without ε-transitions and an input string w, the state-set simulation starts with
S0 = {q0 } and then, for every i = 1, 2, . . . , |w|, computes the set Si = Cw[i] (Si−1 ), where, for any
state set S and symbol b ∈ Σ, we call Cb (S) the set of all states that can be reached from a state in
S by a b-transition, i.e., Cb (S) = {q | ∃p ∈ S ∧ (p, b, q) ∈ δ}. Clearly, each Si contains exactly the
states that are reachable from q0 by a w[1 : i]-labelled path. Now if we are dealing with an εNFA,
then we can use the same idea, but we also have to compute the ε-closures C∗ε (S) in between,
i.e., the set of all states that can be reached from a state in S by a path of ε-transitions. More
precisely, we start with S0 = C∗ε ({q0 }) and then, for every i = 1, 2, . . . , |w|, we compute the set
Si = C∗ε (Cw[i] (Si−1 )). Now each Si contains exactly the states that are reachable from q0 by a
path that is labelled with w[1 : i] (with ε-transitions being ignored in the path label). Computing
S0 is called the initialisation and computing Si from Si−1 is called an update step of the state-set
simulation. For every i = 0, 1, 2, . . . , |w|, the set Si is the set of active states at step i. We also
say that a state p is active at step i if p ∈ Si .
Given a state set S and a symbol b ∈ Σ, computing the set Cb (S) can be easily done in time
O(m), since we only have to compute for p ∈ S the states q with a transition (p, b, q), which are

7

<!-- page 8 -->

given by T [p, b]. Thus, we have to inspect every transition at most once. Computing the ε-closure
C∗ε (S) can also be done in time O(m) by a breadth-first search (BFS) that starts in all states of S
and only considers ε-transitions (again, the entries T [p, ε] can be used for that). This means that
the initialisation and each update step can be done in time O(m), which yields a total running time
of O(|w|m) for the whole state-set simulation. What is more, this running time is conditionally
optimal.
Lemma 3.1 ([BI16]). Given an εNFA A and string w, we can check in time O(|A| · |w|) whether
w ∈ L(A), but not in time O((|A| · |w|)1−ϵ ) for any ϵ > 0 unless SETH fails.

3.2

Solving Sub- and Supersequence Matching via State-Set Simulation

We now explain how we can use state-set simulation to solve the ⪯-matching problem for the
subsequence and supersequence relations (which are our main focus in this paper). It is easy to
see that we can transform an εNFA A into two εNFAs Asub and Asup such that w ∈ L(Asub ) ⇐⇒
Λ⪯sub (w) ∩ L(A) ̸= ∅ and w ∈ L(Asup ) ⇐⇒ Λ⪯sup (w) ∩ L(A) ̸= ∅. For example, this is done
in [BLS15, Lemma 8]. We review this construction as we adapt it later.
The εNFA Asub is obtained from A by simply adding a transition (p, b, p) for every state p
and b ∈ Σ. Intuitively speaking, these loops equip A with the general possibility to consume
symbols from w without transitioning in a new state, which corresponds to ignoring symbols from
w. Hence, the “non-ignoring” transitions of an accepting run of Asub on w spell out a subsequence
of w accepted by A. On the other hand, the εNFA Asup is obtained from A by simply adding a
transition (p, ε, q) for every existing transition (p, b, q) with b ∈ Σ. This means that while reading
w the automaton can always virtually process some symbols that do not belong to w. Hence, in
an accepting run of Asup on w, the actual transitions that read the symbols from w along with
the added ε-transitions that virtually read some symbols spell out a supersequence of w accepted
by A.
The correctness of these constructions is shown in [BLS15, Lemma 8] in slightly different
terms.3 For the sake of self-containment, we give full proofs of their correctness below:
Lemma 3.2 ([BLS15], Lemma 8). For every w ∈ Σ∗ , we have that w ∈ L(Asub ) ⇐⇒ Λ⪯sub (w) ∩
L(A) ̸= ∅.
Proof. Clearly, we have that Λ⪯sub (w) ∩ L(A) ̸= ∅ ⇐⇒ w ∈ Λ⪯sup (L(A)), so it suffices to show
that L(Asub ) = Λ⪯sup (L(A)) holds.
Let w ∈ Λ⪯sup (L(A)). This means that w[j1 ]w[j2 ] . . . w[jℓ ] ∈ L(A) for some 1 ≤ j1 < j2 < . . . <
jℓ ≤ |w|. We can accept w by Asub by using the accepting run of A on w[j1 ]w[j2 ] . . . w[jℓ ], but
whenever we reach a position i of w with i ∈
/ {j1 , j2 , . . . , jℓ }, we simply process w[i] with a loop
(p, w[i], p) of Asub . Thus, w ∈ L(Asub ).
Let w ∈ L(Asub ), i.e., Asub has an accepting run on w. Let w[j1 ]w[j2 ] . . . w[jℓ ] be the subsequence
of w where the ji are exactly the positions of w that Asub processes in its accepting run with original
transitions of A (i.e., not with the self-loops added to A in order to obtain Asub ). By construction,
there must be an accepting run of A on w[j1 ]w[j2 ] . . . w[jℓ ], which means that w[j1 ]w[j2 ] . . . w[jℓ ] ∈
L(A) and therefore w ∈ Λ⪯sup (L(A)).
Lemma 3.3 ([BLS15], Lemma 8). For every w ∈ Σ∗ , we have that w ∈ L(Asup ) ⇐⇒ Λ⪯sup (w) ∩
L(A) ̸= ∅.
Proof. We have that Λ⪯sup (w) ∩ L(A) ̸= ∅ ⇐⇒ w ∈ Λ⪯sub (L(A)), so it suffices to show that
L(Asup ) = Λ⪯sub (L(A)) holds.
Let w ∈ Λ⪯sub (L(A)). This means that u = u0 w[1]u1 w[2] . . . u|w|−1 w[|w|]u|w| ∈ L(A). We can
accept w by Asup by using the accepting run of A on u, but instead of the transitions that process
positions of uj , we simply use the corresponding ε-transitions added to A in order to obtain Asup .
Thus, w ∈ L(Asup ).
3 The work [BLS15] is about the downward-closure and upward-closure of a language L, which are exactly the
sets Λ⪯sub (L) and Λ⪯sup (L).

8

<!-- page 9 -->

Let w ∈ L(Asup ), i.e., let w be a string such that Asup has an accepting run on w. Let
u = u0 w[1]u1 w[2] . . . u|w|−1 w[|w|]u|w| be the string that we obtain by listing the transitions of this
accepting run in order, substituting each Σ-transition (p, b, q) by b, each original ε-transition of
A by ε, and every ε-transition that has been added due to some Σ-transition (p, b, q) by b (since
there might be several choices for this, we take just any of those). By construction, there must be
an accepting run of A on u, which means that u ∈ L(A) and therefore w ∈ Λ⪯sub (L(A)).
Consequently, we can solve the subsequence and supersequence matching problem by checking
whether w ∈ L(Asub ) and w ∈ L(Asup ), respectively, via state-set simulation. Since |Asub | =
O(n|Σ| + m) and |Asup | = O(m), we get the following:
Theorem 3.4. The subsequence matching problem can be solved in time O(|w|(n|Σ| + m)) and
the supersequence matching problem can be solved in time O(|w|m).

3.3

Improved Algorithms for Sub- and Supersequence Matching

It has been observed in [BLS15, Lemma 9] that the εNFAs Asub and Asup have special properties
that can be exploited in the state-set simulations to improve the bounds of Theorem 3.4.
Firstly, for Asub , whenever a state p of Asub is added to the set of active states in the stateset simulation, it will stay active until the end of the state-set simulation. This is due to the
existence of the transitions (p, b, p) for every b ∈ Σ. Consequently, for the state-set simulation
of Asub , we have S0 ⊆ S1 ⊆ . . . ⊆ S|w| . Secondly, for Asup , we can observe that the state-set
simulation starts with S0 = C∗ε ({q0 }) = Q, i.e., the set of all states, which is due to the fact that
every state in Asup is reachable from q0 by ε-transitions and therefore is in the ε-closure of q0 with
respect to Asup . Moreover, when a state q is removed, it is never added back. Indeed, assume by
contradiction that q ∈
/ Si−1 but q gets added to Si by an update step. Then q is not reachable
from Si−1 by ε-transitions, but it is reachable from Cw[i] (Si−1 ) by ε-transitions: this is impossible
because in Asup there is a transition (p, ε, q) for every existing transition (p, b, q) with b ∈ Σ. Thus,
S0 ⊇ S1 ⊇ . . . ⊇ S|w| .
This means that in the state-set simulation for Asub and Asup on an input string w, we can
encounter at most n + 1 different sets of active states. Further, for each set of active states, there
are at most |Σ| possible update steps necessary (since if C∗ε (Cw[i+1] (Si )) = Si and w[i+1] = w[j +1]
and Si = Sj for some i < j hold, then we do not need to update Sj ). Every actual update can
again be done in time O(m), which yields the following:
Theorem 3.5. There are algorithms that solve the subsequence matching problem in time O(|w| +
n|Σ| · (n|Σ| + m)) and the supersequence matching problem in time O(|w| + n|Σ|m).
Proof. According to Lemmas 3.2 and 3.3, we can solve the subsequence and supersequence matching problem by checking w ∈ L(Asub ) and w ∈ L(Asup ), respectively. The proof of the theorem
therefore follows from the fact that for both Asub and Asup a state-set simulation can be performed
in time O(|w| + n|Σ| · |Asub |) and O(|w| + n|Σ| · |Asup |) instead of O(|w| · |Asub |) and O(|w| · |Asup |),
respectively (note that n is the number of states of A, Asub and Asup ). Let us discuss this first for
the case of Asub .
Initially, we have to compute C∗ε ({q0 }), which can be done in time O(|Asub |). Now let S0 , S1 ,
S2 , . . . , S|w| be the state sets of the single steps of the state-set simulation. As observed above,
S0 ⊆ S1 ⊆ . . . ⊆ S|w| . This means that the sequence S0 , S1 , S2 , . . . , S|w| contains at most n + 1
distinct sets (recall that n is the number of states). We say that a symbol b ∈ Σ does not change
a state set S if S = C∗ε (Cb (S)). For every state set Si in S0 , S1 , S2 , . . . , S|w| , this set Si is not
changed by the symbols of some following update steps (possibly 0), until the state-set simulation
terminates or the state set is changed to another state set.
Finding out that a symbol does not change the current state set requires time O(|Asub |), and we
can then store this information. Hence, for every update step i, we either have to check whether
w[i] changes Si−1 in time O(|Asub |), or we already know that w[i] does not change Si−1 and the
update step can therefore be performed in constant time. Consequently, there can be at most

9

<!-- page 10 -->

O(n|Σ|) update steps that require time O(|Asub |), while all other update steps can be done in
constant time. This yields a total running time of O(|w| + n|Σ| · |Asub |).
The argument for Asup is similar. We now have that S0 ⊇ S1 ⊇ . . . ⊇ S|w| , which again means
that there are at most n + 1 distinct sets of active states, and again the set of active states is not
changed by the symbols of the following update steps, until the state-set simulation terminates
or the state set is changed to another state set. So by the same argument as before, we conclude
that there are at most (n + 1)|Σ| update steps that require time O(|Asup |), while all other update
steps can be done in constant time.
We conclude the proof by observing that Asub has O(n|Σ|+|A|) transitions and Asup has O(|A|)
transitions.
This is already a significant improvement over Theorem 3.4, since the running time is linear in
|w|. In the next two sections, we look deeper into the problem of subsequence and supersequence
matching and manage to lower its complexity to the optimum of O(|w| + m).

4

Subsequence and Supersequence Matching in Linear Time

In this section, we prove that the ⪯sub - and ⪯sup -matching problem can be solved in linear time
O(|w| + m). Note that this also holds for the usual regex matching problem when the input regex
or εNFA is assumed to accept an upward-closed or downward-closed language (as matching is
then equivalent to ⪯sub - or ⪯sup -matching). We first prove the result for the subsequence relation,
which is slightly easier, and then cover the supersequence relation.

4.1

Subsequence Matching in Linear Time

Theorem 4.1. Given a string w and εNFA A with n states and m transitions, the subsequence
matching problem can be solved in time O(|w| + m).
Proof. We present an algorithm to decide whether there exists a subsequence u of w such that
u ∈ L(A).
Let A = (Q, Σ, q0 , qf , δ) be the input εNFA. For this automaton, we construct the arrays L[·]
and T [·, ·] defined in Section 2. Also, we assume that every transition is initially unmarked and
can be marked during the algorithm (but marked transitions stay marked and cannot be set
unmarked again; intuitively, these transitions were explored in our algorithm, and do not need to
be considered again). One purpose of these markings is to simplify our complexity analysis: The
total running time of the algorithm will be proportional to |w| plus the total number of times we
mark a transition. Moreover, we will repeatedly compute the ε-closure C∗ε (S) of a state set S, but
only with respect to unmarked ε-transitions, i.e., we compute C∗ε (S) as discussed in Section 3, but
we simply ignore marked ε-transitions, and, furthermore, while computing the ε-closure, we will
mark all unmarked ε-transitions that we traverse.
We can now describe our algorithm, which is based on a state-set simulation of Asub , but
without explicitly constructing Asub , i.e., we work directly on A. More precisely, for every i =
0, 1, 2, . . . , |w|, we compute a state-set Si of all states reachable from q0 by a path labelled with
a subsequence of w[1 : i]. Additionally, we will compute (using the arrays L[·] and T [·, ·]) and
maintain a lazily initialised array H[·] of lists, indexed by the symbols of Σ, such that, after Si
is computed, H[a] = {(q, a, q ′ ) ∈ δ | q ∈ Si , q ′ ∈ Q, (q, a, q ′ ) is unmarked}, i.e., H[a] contains all
unmarked transitions labelled with a leaving the states contained in Si . Moreover, as explained
in Section 3, we will have S0 ⊆ S1 ⊆ . . . ⊆ S|w| .
We implement all the sets Si with a single Boolean characteristic array S, which is maintained
in our algorithm in such a way that Si−1 is the set indicated by S after w[i − 1] was processed
and before the processing of w[i] is started, and during the processing of w[i] some positions of S
(namely, |Si \ Si−1 | many) are changed from 0 to 1. This allows us to implement in constant time
the membership-testing to and insertion in the manipulated sets of states. For simplicity of the
exposition, and to mirror the computation of Theorem 3.5, we use the notation Si to refer to the
10

<!-- page 11 -->

set stored in the array S after the processing of w[i] was concluded, for all i from 1 to |w|, and we
use S0 to refer to the content of the respective set right before we start processing w[1].
Initially, we set S0 = C∗ε ({q0 }), where this ε-closure is computed as described above, i.e.,
ignoring marked ε-transitions (of which, at this step of the algorithm, none exist anyway) and
marking the traversed ε-transitions. We note that this initialisation takes time proportional to
the number of transitions that we mark.
Further, by traversing the lists L[q], for q ∈ S0 , we can also compute, for each a ∈ Σ, the list
H[a] = {(q, a, q ′ ) ∈ δ | q ∈ S0 , q ′ ∈ Q}; note that these transitions are not marked yet, as we have
only marked ε-transitions until now, and also that there are no unmarked ε-transitions leaving
the states of S0 . This takes time proportional to the sum of |S0 | and the number of transitions
inserted into the lists stored in H.
Let us now explain the update step, i.e., how Si is computed and H is updated for 1 ≤ i ≤ |w|.
We initially set Si = Si−1 and let R be an auxiliary empty queue, which will be used to collect
all the new states from Si \ Si−1 . Now, for each (q, w[i], q ′ ) ∈ H[w[i]], we mark the transition
(q, w[i], q ′ ), remove it from H[w[i]], and, if q ′ ∈
/ Si , then we add q ′ to Si and R. It is worth noting
that, after this loop concludes, the set H[w[i]] will be empty. Moreover, for all states q ∈ Si−1 ,
there is no unmarked transition (q, w[i], q ′ ) for any q ′ ∈ Q. Then we compute C∗ε (R), but again we
ignore marked ε-transitions and mark all traversed ε-transitions. All states q ∈ C∗ε (R) with q ∈
/ Si
are added to both R and Si . At this point, for all states q ∈ Si , there is no unmarked transition
(q, ε, q ′ ) for any q ′ ∈ Q. Finally, for every r ∈ R and for every transition (r, a, r′ ) ∈ L[r] with
a ∈ Σ and r′ ∈ Q, we add (r, a, r′ ) to H[a] (note that this transition could not have been marked
before, as r was not contained in any set Sj for j < i); this step takes time proportional to the
sum of |R| and the number of transitions inserted into the lists stored in H.
Once the state-set simulation is completed, we terminate and answer positively if and only if
qf ∈ S|w| .
As far as the complexity of the above algorithm is concerned, we note that we have to perform
O(|w|) individual updates. The total number of steps over all these updates is upper-bounded by
the number of transition-insertions in the lists of H[·] and transition-markings. Each transition
is either never marked, or it is marked in the initialisation, where we compute C∗ε ({q0 }), or it is
inserted in one of the lists of the array H[·] for some i, and then it may be marked at most once
and never considered again (also, it will never be reinserted in H[·] because its source state became
active when the transition was inserted so the state will never be added to the queue R in further
update steps). Thus, our algorithm runs in O(|w| + m) time.
The correctness follows from the observation that our algorithm computes exactly the sets
S0 , S1 , . . . , S|w| of the state-set simulation with respect to the automaton Asub on input w (see
Section 3). Indeed, the set S0 is correctly computed in the initialisation, where we also compute
H[a] = {(q, a, q ′ ) ∈ δ | q ∈ S0 , q ′ ∈ Q, (q, a, q ′ ) unmarked} for every a ∈ Σ. Then, assuming that
Si−1 is correctly computed and the contents of H satisfy H[a] = {(q, a, q ′ ) ∈ δ | q ∈ Si−1 , q ′ ∈
Q, (q, a, q ′ ) unmarked} for every a ∈ Σ, we compute Si by first computing Cw[i] (Si−1 ) by adding
all q ′ for every (q, w[i], q ′ ) ∈ H[w[i]]. Then we compute C∗ε (Cw[i] (Si−1 )) by adding the ε-closure
of the states from R, but only with respect to unmarked ε-transitions, which is correct, since
all marked transitions lead to states already in Si−1 and can therefore be ignored. Finally, H is
correctly updated by adding (r, a, r′ ) to H[a] for every r ∈ R and every transition (r, a, r′ ) ∈ L[r]
with a ∈ Σ; as noted before, no transition starting in r was marked, so the transitions added in
this step are all unmarked, and there are no other transitions originating in Si that do not already
belong to the lists of H.
Therefore, we have shown a correct algorithm, solving the subsequence matching problem in
O(|w| + m) time.

4.2

Supersequence Matching in Linear Time

The general idea is again to check w ∈ L(Asup ), and unlike in the previous subsection, we can
afford to build Asup in time O(m) explicitly. However, performing a state-set simulation in linear
time with Asup is more difficult. Intuitively speaking, in order to obtain Si , we have to remove
11

<!-- page 12 -->

from Si−1 all states that cannot be reached by any w[i]-labelled path from some other state from
Si−1 . It is not directly clear how this can be done by touching each transition only a constant
number of times over the whole course of the state-set simulation. One ingredient to achieve this
is to first decompose Asup into its strongly connected components (SCCs).
Recall that the SCCs of a directed graph G are the maximal subsets of vertices R such that,
for any two distinct vertices u and v in R, there is a directed path from u to v and from v to u
in G. The SCCs of an εNFA are simply its SCCs when seeing the automaton as a directed graph
(ignoring the edge labels in Σ ∪ {ε}).
The condensation of an εNFA A, denoted by cond(A), is an εNFA whose states are the SCCs
of A. For convenience, for every state p of A, we denote by SCCA [p] (or simply SCC[p] if A is
clear from the context) the SCC of A that contains p, which, by definition, is a state of cond(A).
The transitions of cond(A) are as follows: for every transition (p, a, q) of A, we have a transition
(SCCA [p], a, SCCA [q]) in cond(A). Note that SCCA [p] = SCCA [q] is possible, and note that several
transitions may be merged in cond(A). The initial state of cond(A) is SCCA [q0 ] and the final state
of cond(A) is SCCA [qf ].
Proposition 4.2. Given an εNFA A, we can construct cond(A) in time O(m).
Proof. We can compute the SCCs of A in time O(m). Now, we go over the transitions of A and
add the corresponding transitions to cond(A), again in time O(m).
Leaving aside the self-loops, the condensation of an εNFA has the convenient property of being
a directed acyclic graph (DAG). However, constructing the condensation of an automaton changes
in general the language that it accepts (e.g., if A can read some string u by going from state p to
state q and we have SCCA [p] = SCCA [q], then cond(A) could read any permutation of u, which in
general changes the accepted language). However, we shall next see that this is not the case for
the εNFA Asup , i.e., we have L(Asup ) = L(cond(Asup )).
Lemma 4.3. Let A be an εNFA and let w ∈ Σ∗ . Then L(Asup ) = L(cond(Asup )).
Before proving this lemma, let us first prove an auxiliary result:
Proposition 4.4. Let A be an εNFA and let C be some fixed state of cond(Asup ). Assume that a
string u = u[1] · · · u[|u|] can be read by cond(Asup ), starting in state C and following the sequence
of transitions (C, u[1], C), (C, u[2], C), . . . , (C, u[|u|], C). Then there is a path in Asup , labelled with
u, between any two states of C.
Proof. If there is a transition (C, b, C) in cond(Asup ), then there must be a transition (p, b, q) in
Asup with p, q ∈ C. Now let p′ , q ′ be arbitrary states from C. Since p, p′ , q, q ′ ∈ C, there is a path
from p′ to p and a path from q to q ′ in Asup that only use states from C. By construction of Asup ,
we can further assume that these paths have only ε-transitions. Hence, we can join these two paths
with the transition (p, b, q), which yields a path from p′ to q ′ with exactly one non-ε-transition,
which is a b-transition. Consequently, if cond(Asup ) has a transition (C, b, C), then, for every
p′ , q ′ ∈ C, Asup has a path from p′ to q ′ with exactly one non-ε-transition, which is a b-transition.
This directly implies that if a string u = u[1] · · · u[|u|] is read by cond(Asup ), starting in state C
and following the sequence of transitions (C, u[1], C), (C, u[2], C), . . . , (C, u[|u|], C), then there is a
path in Asup , labelled with u (each non-ε-transition being potentially surrounded by ε-transitions),
between any two states of C.
Now we can give the proof of Lemma 4.3.
Proof. For every transition (p, b, q) of Asup , the transition (SCC[p], b, SCC[q]) of cond(Asup ) is called
a move-transition if SCC[p] ̸= SCC[q] and a stay-transition if SCC[p] = SCC[q].
Let w ∈ Σ∗ . Every accepting run of Asup on w translates into an accepting run of cond(Asup )
on w in the following way: for every transition (p, b, q) of the run of Asup , we take its corresponding
transition (SCC[p], b, SCC[q]) in cond(Asup ). Thus, L(Asup ) ⊆ L(cond(Asup )).

12

<!-- page 13 -->

Now let us consider an accepting run of cond(Asup ) on w, and let us assume that in this run,
we enter some state C with a move-transition (or C = SCCAsup [q0 ] is the initial state), then we
read a factor w[i : j] by only stay-transitions, and then we leave C with a move-transition (or
C = SCCAsup [qf ] and the run is finished). Since w[i : j] is consumed by only stay-transitions,
Proposition 4.4 directly implies that w[i : j] can be read by Asup between any two states of C.
Combining these paths with the transitions of Asup that gave rise to the move-transitions of
the accepting run of cond(Asup ), we deduce that there is an accepting run of Asup on w. Thus,
L(cond(Asup )) ⊆ L(Asup ).
Sec. 3

Lem. 4.3

Hence, A accepts a supersequence of w ⇐==⇒ w ∈ L(Asup ) ⇐====⇒ w ∈ L(cond(Asup )) .
Moreover, cond(Asup ) inherits the crucial properties of Asup , namely:
Proposition 4.5. If cond(Asup ) has a transition (C, b, C ′ ) for some b ∈ Σ, then it also has a
transition (C, ε, C ′ ). Further, if S0 , S1 , . . . , S|w| are the state sets of the state-set simulation of
cond(Asup ) on w, then S0 ⊇ S1 ⊇ . . . ⊇ S|w| and S0 contains all states of cond(Asup ).
Proof. By construction, if Asup has a transition (q, b, q ′ ) for some b ∈ Σ, then it also has a transition
(q, ε, q ′ ). This property is maintained by the condensation operation. Moreover, this property
directly implies that S0 ⊇ S1 ⊇ . . . ⊇ S|w| and that S0 contains all states of cond(Asup ).
Our next goal is to show that we can implement the state-set simulation of cond(Asup ) on w
in linear time. For this, we will need to efficiently identify states C ∈ Si−1 that do not have
a self-loop for the next input symbol w[i] (since if such a self-loop exists, then C ∈ Si holds).
Identifying such states is challenging: while there are at most m self-loops, there might be up
to n|Σ| pairs (C, a) overall where C does not have a self-loop labelled a, so we cannot explicitly
materialize these pairs. Instead, we use a specific data structure:
Lemma 4.6. For an εNFA A = (Q, Σ, q0 , qf , δ), we can build in time O(1) a data structure R
storing a set of states, initially empty, and supporting the following operations:
• Push: Insert a state in R.
• Pop: Given a ∈ Σ, retrieve and remove all states q ∈ R without a self-loop labelled with a
(or indicate that no such state exists).
Over a sequence of ℓ push and pop operations where each state of A is pushed at most once, the
total running time is O(ℓ + m) where m is the number of transitions of A.
Proof. We store the states in a doubly linked list R where states are inserted at the right. Initially,
this list is empty.
We see R as implying a total order < on the states that it contains: q ′ < q means that q ′
occurs to the left of q in R. For each state q ∈ Q of the automaton and symbol a ∈ Σ, we keep a
Boolean array B[q, a], lazily initialised, intuitively indicating whether state q has been examined
for symbol a (i.e., B[q, a] = 1 if state q has been examined for symbol a). Note that, initially, no
element of B[·, ·] is initialised (meaning that B[q, a] ̸= 1 for all q ∈ Q and a ∈ Σ).
While performing push and pop operations on R, we will maintain two invariants:
1. for every a ∈ Σ and states q, q ′ ∈ R with q ′ < q, if B[q, a] = 1, then B[q ′ , a] = 1.
2. if B[q, a] = 1 then q has a self-loop labelled by a.
Invariant 1 means that if R = (q1 , q2 , . . . , qk ), then, for every a ∈ Σ, there is a ja ∈ {1, 2, . . . , k+1},
such that B[qi , a] = 1 for 1 ≤ i < ja and B[qi , a] ̸= 1 for ja ≤ i ≤ k. Moreover, Invariant 2 means
that, for every i with 1 ≤ i < ja , the state qi has a self loop labelled with a; note that this
implication is only in one direction, as there can be states qi stored in R, with i > ja such that qi
has a self-loop labelled with a. (Intuitively, such states have not yet been examined.)
Both invariants trivially hold after the initialisation (as no element of B[·, ·] was set to 1).

13

<!-- page 14 -->

To perform a push operation with a state q, we simply insert q at the (right) end of R, which
can be done in O(1) time if we maintain a pointer to the respective end of R. Invariant 1 is
maintained (as no change is made to the elements of B[·, ·] and the new state q is now maximal
for <); Invariant 2 is also clearly maintained.
For a pop operation with symbol a, we do the following: We traverse R from right to left until
we encounter a state with B[q, a] = 1 or we finish processing R. Now, for every state q ∈ R with
B[q, a] ̸= 1 which is traversed, we do the following: if q does not have a self-loop labelled with a
then we retrieve q and remove it from R, and if q has such a self-loop then q stays in R but we
set B[q, a] ← 1. This maintains Invariant 2, since we set B[q, a] ← 1 only in the case that q has
a self-loop labelled by a. That Invariant 1 is maintained follows from the fact that after the pop
operation, we have B[q, a] = 1 for every q ∈ R. Indeed, after the pop, the only states of R that
are still in R and have not been traversed are those states q ′ to the left of the rightmost state q
for which we had B[q, a] = 1 before the pop (if such a q does not exist then all states of R were
traversed); and these states q ′ also had B[q ′ , a] = 1 as Invariant 1 held before the current pop
operation.
The total running time is proportional to the total number of operations ℓ, plus the total
number of states traversed in pop operations. Now, every state q traversed in a pop operation
is either returned and removed from R, or we set B[q, a] ← 1 for the symbol a for which we are
popping. The total number of returned states, say r, is upper-bounded by the total number of
push operations (a state had to be pushed first for it to be removed); therefore, r is in O(ℓ). Now,
every time we set B[q, a] ← 1 for a state q ∈ R with B[q, a] ̸= 1 in a pop operation, this accounts
for a self-loop of q labelled with a, and we do it only once for the pair (q, a). This means that the
total number of these assignments is bounded by O(m). This concludes the complexity analysis
and concludes the proof.
We will use the above structure for A = cond(Asup ). We can finally show our main result:
Theorem 4.7. Given a string w and εNFA A with n states and m transitions, the supersequence
matching problem can be solved in time O(|w| + m).
Proof. We first construct cond(Asup ) from A, which can be done in time O(m) (see Section 3 and
Proposition 4.2). As noted in Section 3, we can solve the supersequence matching problem by
checking whether w ∈ L(Asup ), which, by Lemma 4.3, can be done by checking w ∈ L(cond(Asup )).
Since |cond(Asup )| = O(|A|), it suffices to show that we can check w ∈ L(cond(Asup )) in time
O(|w| + |cond(Asup )|).
For convenience, we set now A′ = cond(Asup ) = (Q′ , Σ, q0′ , qf′ , δ ′ ) and let n′ = |Q′ | and m′ = |δ ′ |.
For A′ , we construct the arrays L[·] and T [·, ·] defined in Section 2. We will now show how to check
w ∈ L(A′ ) in time O(|w| + m′ ). We will still use the special structure of A′ for our algorithm, i.e.,
it is a DAG (potentially with self-loops) and has the property described in Proposition 4.5.
We start by pre-computing several data structures:
• loops[·, ·] is a lazily initialised |Q′ | × |Σ| Boolean array with loops[q, a] = 1 if (q, a, q) ∈ δ ′ .
• Lq for each q ∈ Q′ is a list that contains all a ∈ Σ with loops[q, a] = 1.
• in[·] is an array with in[q] = |{(q ′ , a, q) ∈ δ ′ | q ′ ∈ Q′ \ {q}, a ∈ Σ ∪ {ε}}| for each q ∈ Q′ .
• roots is a set with roots = {q ∈ Q′ | in[q] = 0}.
• mark[·] is an array with mark[q] = 0 for all q ∈ Q′ .
Note that loops and Lq provide the information of which states have loops for which symbols,
in[q] is the number of transitions that have q as target (while originating in other states), roots
stores all roots of A′ , and mark is a Boolean array that shall be used to mark states (initially, all
states are unmarked). Note that since A′ has a DAG structure, we know that roots ̸= ∅.

14

<!-- page 15 -->

The first three data structures can be computed simultaneously by considering every transition
of A′ at most once; the last two data structures can be computed by considering each state at
most once. Thus, all these data structures can be computed in time O(m′ ).
By Lemma 4.6, we can construct in time O(|roots|) a data structure R, storing initially the set
of states in roots, and supporting the following operations:
• Push: Insert a state q in R.
• Pop: Given a ∈ Σ, retrieve all states q ∈ R such that a ∈
/ Lq and remove them from R (or
indicate that no such state exists).
The initialisation of R is done by repeatedly pushing the states of roots in it. Further, the
time needed by R to process a sequence of ℓ push and pop operations where each state of A′ is
pushed at most once in total is O(ℓ + m); this includes the initial push of the states in roots.
Our algorithm is an efficient implementation of the state-set simulation for the automaton A′
on w (see Section 3). That is, for every i = 0, 1, 2, . . . , |w|, we compute a state-set Si of all states
of A′ reachable from q0 by a path labelled with w[1 : i]. Moreover, since A′ = cond(Asup ) and
Proposition 4.5 holds, we will have Q′ = S0 ⊇ S1 ⊇ . . . ⊇ S|w| .
We implement all the sets Si with a single Boolean characteristic array, which allows us to
obtain Si from Si−1 by changing |Si−1 \ Si | many entries from 1 to 0 (due to the fact that
Si−1 ⊇ Si ).
We start with S0 = Q′ . Recall that the data structure R contains the set of states roots.
Let us first give an intuitive explanation of how Si is computed from Si−1 . Let a = w[i]. Since
Si−1 ⊇ Si , we only have to identify those states that must be deleted from Si−1 in order to obtain
Si , which are exactly the states that cannot be reached by an a-labelled path from any other state
of Si−1 . Since A′ has a DAG structure, this can be done as follows.
Every root without a self-loop with a has to be deleted, and every root with a self-loop with
a will be in Si . In particular, we can ignore the outgoing transitions of those roots that will be
in Si in the rest of the update step, since they necessarily point to states that will also be in Si .
This means that we have to be able to initially get those roots that have no self-loop with a, for
which we employ the data structure R. Now if we remove some root q, then each of its outgoing
transitions (q, x, q ′ ) with x ∈ Σ ∪ {ε} has to be removed, but we also have to process each such
transition as follows. If x = a, then q ′ will necessarily be in Si and we do not have to consider
q ′ again in this update step (we also mark q ′ accordingly). If, on the other hand, x ̸= a, then we
first check if q ′ has a self-loop with a, which is also sufficient for q ′ being in Si (again, we mark
it accordingly and do not have to consider it again). If q ′ has no self-loop with a and at least
one other incoming transition, then we do not know whether it is in Si or not, since this might
be determined by some other incoming transition from a root. If q ′ has no self-loop with a, but
becomes a root when we delete (q, x, q ′ ), then it has to be treated as one of the initial roots (i.e.,
it will be deleted at some point and its outgoing transitions have to be deleted and processed as
described above). In particular, this means that when we delete states and transitions, we have
to update the in-degrees of states to be able to identify those states that become roots. Let us
now formally define this algorithm.
We initially set Si ← Si−1 . Then we extract from R all states q such that a ∈
/ Lq (i. e, q has
no self-loop with a) and store them in a queue G. While G is not empty, we pop state q from the
queue G and remove it from A′ and Si . Then we remove from A′ each transition (q, b, q ′ ) with
b ∈ Σ ∪ {ε} in L[q] and proceed as follows:
1. If b = a, then
• set in[q ′ ] ← in[q ′ ] − 1;
• set mark[q ′ ] = i (this state needs to stay in Si , and will no longer be analysed in this
update step);
• if in[q ′ ] = 0 then insert q ′ into R (i.e., it is now a root of A′ and has to be treated like
one in the next update step).
15

<!-- page 16 -->

2. If b ̸= a, then
• set in[q ′ ] ← in[q ′ ] − 1;
• if loops[q ′ , a] = 1 then set mark[q ′ ] = i (meaning that this state needs to stay in Si , and
will no longer be analysed in this update step);
• if in[q ′ ] = 0 and mark[q ′ ] = i then insert q ′ into R;
• if in[q ′ ] = 0 and mark[q ′ ] ̸= i then push q ′ to G (this state needs to still be removed
from Si , at a later point during the processing of w[i]).
Once the set S|w| is computed, we provide a positive answer to the supersequence matching
problem if and only if S|w| contains the final state qf′ .
In order to prove the correctness of the algorithm, it is sufficient to prove that we really compute
the sets S0 , S1 , . . . , S|w| of the state-set simulation of A′ on w.
Since we start with S0 = Q′ , this is clearly true for S0 due to the property of Proposition 4.5.
Moreover, the following invariant is satisfied: for every q ∈ S0 , in[q] stores the number of transitions, other than self-loops, that have q as target and some q ′ ∈ S0 as origin, and R stores exactly
those states q ∈ S0 with in[q] = 0.
Let us next assume that Si−1 has been computed correctly for some i ∈ {1, 2, . . . , |w|}, and
that the invariant is satisfied, i.e., for every q ∈ Si−1 , in[q] stores the number of transitions, other
than self-loops, that have q as target and some q ′ ∈ Si−1 as origin, and R stores exactly those
states q ∈ Si−1 with in[q] = 0. We will now show that Si is correctly computed in such a way that
the invariant is also satisfied.
Let us call every state q ∈ Si−1 good if there is some state p ∈ Si−1 with a w[i]-labelled path
from p to q (which can also be a self-loop); and denote states from Si−1 as bad if they are not
good. This means that Si is exactly the set of good states from Si−1 . Observe that if a state q is
good then all states that can be reached from q are also good. We will show that our algorithm
constructs Si by deleting exactly the bad states from Si−1 .
Assume |Si−1 | = h and let us consider an arbitrary topological sorting q1 < . . . < qh of the
states in Si−1 , w.r.t. the transitions between states of Si−1 which are not self-loops. We will show,
by induction on j, with 1 ≤ j ≤ h, that the states removed by our algorithm from q1 < . . . < qj
are exactly the bad states of {q1 , . . . , qj }. For j = 1, this clearly holds. Indeed, q1 must be a root
of Si−1 , as it comes first in the topological sorting. It is removed if and only if it has no self-loop
labelled with w[i]; but this is equivalent to q1 being a bad state. Assume now that our claim holds
for j − 1, and let us show that it holds for j. We want to show that qj is removed if and only if qj is
bad. Assume first that qj is removed by our algorithm. If qj is a root of Si−1 , the same argument
as for q1 holds. Assume qj is not a root. This means that all its parents were also removed by our
algorithm (as a state q is removed only if in[q] = 0). But all the parents of qj must come before qj
in any topological sorting of Si−1 . They were removed, so they are bad states, by the induction
hypothesis. Moreover, when the parents of qj were removed, no w[i]-transition targeting qj and
no w[i]-self-loop of qj were discovered. Thus, qj is also bad. For the converse, assume qj is a bad
state. This means that all its parents (if any) are also bad. But, as before, all the parents of qj (if
any) must come before qj in any topological sorting of Si−1 . By the induction hypothesis, as they
are bad, they are also removed by our algorithm. During their removal, we explore all transitions
connecting them to qj , and this allows us to check whether there are any w[i]-transitions targeting
qj or any w[i]-self-loops of qj . The existence of such a transition would be a contradiction to the
fact that qj is bad. Thus, we never set mark[qj ] ← i, but we decrement in[qj ] until it becomes 0.
When this happens, qj is inserted in G, so it will be removed. This concludes the proof of our
claim. Hence, we compute Si correctly.
With respect to the invariant, we observe that the values in[q] are correctly updated whenever
we remove a transition, and whenever a state becomes a root, but is not removed (i.e., it will be
a root in Si ), then this state is explicitly added to R.
This concludes the proof of correctness.
As far as the complexity is concerned, the number of steps of the algorithm is proportional to
the sum of |w|, the number of removed edges (which is O(m′ )), and the total time spent with the
16

<!-- page 17 -->

data structure of Lemma 4.6. We spend time O(n′ ) to populate it initially, and overall the number
of operations performed is O(|w| + m′ ) (and note that each state is only pushed at most once).
In particular, it is important to efficiently access the roots of the DAG Si , and then start the
exploration of Si from these states; the fact that we can retrieve these states in time proportional
to their number is ensured by the usage of the data structure of Lemma 4.6. Therefore, our
statement holds.

5

Quantitative Problem Variants

For every x ∈ {in, pre, ext, lext, sub, sup}, we consider now the min- and max-variant of the ⪯x matching problem, i.e., computing a shortest or longest string u ∈ L(A) with u ⪯x w. We show
an upper bound of O(|w|m) for all these problem variants.4
Theorem 5.1. The min- and max-variant of the matching problem can be solved in time O(|w|m)
for all relations ⪯x with x ∈ {in, pre, ext, lext, sub, sup}.
In Section 7 we show that the O(|w|m) upper bound for all the considered variants is conditionally tight, i. e., O((|w|m)1−ϵ ) is not possible unless SETH fails. The rest of this section is
devoted to the proof of Theorem 5.1. Before giving this proof, we need some preliminaries.
Recall that a path in a directed graph may traverse the same vertex multiple times (a simple
path is a path where each vertex is traversed once). An st-path is a path from a vertex s to a
vertex t. When the edges of a directed graph are weighted, let weight(e) be the weight of edge e
and let the weight of a path be the sum of weights of the successive edges that it traverses (if an
edge is traversed multiple times then the weight is added as many times as the edge is traversed).
In the case where the edges are labelled, the label of a path is the concatenation of the labels of
the successive edges that it traverses (if the edges carry labels in Σ ∪ {ε} then the label of the
path is a string in Σ∗ ). We show the following ancillary result:
Lemma 5.2. Let G be a directed graph with edges weighted by 0 and 1, and let s and t be two
distinguished vertices. We can check in time O(|G|) whether there is an st-path in G, and if an
st-path exists, then we can compute in time O(|G|):
• an st-path with minimum weight.
• whether there exist st-paths of unbounded weight in G.
• an st-path with maximum weight, provided that the weight of st-paths is not unbounded.
Note that the lemma is stated for edge weights 0 and 1, but applies more generally to weights
bounded by a constant, as we can split every edge of weight larger than 1 into a constant number
of length-1 edges connecting auxiliary nodes. Note that the lemma also applies to labelled directed
graphs: in this case, when we obtain a path by applying the lemma, then we can retrieve its label
(in Σ∗ ) from its sequence of edges. Let us prove the lemma:
Proof of Lemma 5.2. We first eliminate all graph vertices that do not have a path from s, as well
as those that do not have a path to t. This is akin to automaton trimming, and can be done in
O(|G|) with two graph traversals. The resulting graph is empty if and only if there is no st-path.
Let us now assume that the trimmed graph is not empty and, for simplicity, let us denote this
trimmed graph by G.
Minimum weight st-path. To compute an st-path of minimum weight, we simply use Dijkstra’s
algorithm with a priority queue implemented with a table of |G| buckets. This makes it possible
to insert vertices with a given priority in O(1), lower their priority in O(1), and retrieve a vertex
of lowest priority in time O(1) per call plus an O(|G|) total cost throughout the entire algorithm
4 Of course, these upper bounds also give upper bounds on the corresponding (non-quantitative) matching
problems for ⪯in , ⪯pre , ⪯ext and ⪯lext ; see Table 1.

17

<!-- page 18 -->

to go over successive buckets. (Note that the sequence of priorities of the vertices that we extract
out of the priority queue in Dijkstra’s algorithm is nondecreasing, so we only need to go once over
the sequence of buckets throughout the algorithm.) One may remark that the path of minimum
weight that we obtain is always a simple path.
Unbounded weight st-path. To check whether there exist st-paths of unbounded weight, we compute
the condensation of G (see Section 4.2) in time O(|G|). Obviously, there is a strongly connected
component (SCC) that contains an edge with nonzero weight if and only if there are st-paths of
unbounded weight. Indeed, if there is an SCC that contains an edge with nonzero weight, then we
can repeat a cycle of the SCC containing this edge as many times as we want, and combining it
with a path from s to the SCC and from the SCC to t (these paths must exist by our preprocessing
of G at the beginning of the proof). On the other hand, if no such SCC with an edge of nonzero
weight exists, then every possible st-path in G has only cycles with weight-0 edges, which means
that there are only a finite number of possible weights for st-paths.
Maximum weight st-path. Finally, let us assume that there are no st-paths with unbounded weight
and let us show how to compute a largest weight st-path. For this purpose, we will compute a
path of maximum weight that leads to t from every node. However, the paths can be of linear size,
so we store them in a compressed way, by remembering for every node v only the first edge on a
maximum weight vt-path. Then the path can be retrieved edge-by-edge, starting from s, until we
reach node t. In addition to the first edge, we also store the total weight X[v] of the vt-path of
maximum weight.
We process all SCCs of the condensation of G in reverse topological ordering. Observe that
within every SCC all edges have weight 0, thanks to our assumption that there are no st-paths
with unbounded weight. First we process Ct = SCC[t], the SCC containing t. Clearly, for all nodes
from this component, every path to t has weight 0, so we can use any such path. Now we show
how to make a consistent choice of first edge towards t from every node. For this, let E rev be
the set of all edges of Ct , but with reversed direction, that is {(q ′ , q) | (q, q ′ ) ∈ E(G)}. We run a
depth-first search from t only on the nodes from Ct , but along the reversed edges E rev . For every
considered node v ̸= t, we store the edge to its parent in the recursion tree as the first edge on a
vt-path with maximum weight. Observe that all the nodes of Ct will be visited, as Ct is a strongly
connected component and so is its counterpart with edges reversed. Moreover, the chosen edges
correctly define paths to t, as they correspond to going up the recursion tree towards the root t.
For every other SCC C of G in reverse topological ordering, we calculate the value:
x := max

max

{weight((u, v)) + X[v]}

u∈C (u,v)∈E(G)
v/
∈C

and we set X[u] := x for all u ∈ C. Note that all such edges to outside of C point to SCCs that
have been already visited and hence the values X[v] have already been calculated for all v ∈
/ C
such that there is an edge from a node from C to v. Let z and e = (z, v) be a node and edge for
which the above maximum is attained and set e as the first edge on a zt-path with largest weight.
For all nodes u ∈ C \ {z} we find the first edge on the ut-path with maximum weight similarly as
in Ct , but now we start the DFS from z. Again, all edges within C have weight 0, so it suffices to
find any path to z.
Finally, after processing all the SCCs, X[s] stores the total weight of the st-path of maximum
weight and we can retrieve such path in linear time. This concludes the proof of the last case and
hence the lemma holds.
One graph which will often be relevant is the product graph of an automaton and a string:
Definition 5.3. The product graph GA,w of an εNFA A on states Q and a string w ∈ Σ∗ is the
labelled and weighted graph with vertex set {0, . . . , |w|} × Q constructed as follows:
• For each transition (q, ε, q ′ ) in A and for each i ∈ {0, . . . , |w|}, there is an edge from (i, q)
to (i, q ′ ) in GA,w with label ε and weight 0.
18

<!-- page 19 -->

• For each transition (q, b, q ′ ) with b ∈ Σ and for each i ∈ {1, . . . , |w|} with w[i] = b, there is
an edge from (i − 1, q) to (i, q ′ ) in GA,w with label b and weight 1.
The following points about the product graph GA,w are immediate:
• The product graph can be built in time O(|A| · |w|).
• Any path in GA,w from a vertex of the form (i, q) to a vertex of the form (j, q ′ ) must be such
that j ≥ i, and it has weight precisely j − i.
• For any two states q and q ′ and integers 0 ≤ i ≤ j ≤ |w|, there is a path from (i, q) to (j, q ′ )
in GA,w if and only if there is a run of A going from state q to state q ′ while reading the
(possibly empty) infix w[i + 1 : j].

5.1

Proof of Theorem 5.1

We are now ready to give the proof of Theorem 5.1. The proof will be structured into a part for
the infix and prefix relation, a part for the extension and left-extension relation, and a part for
the subsequence and supersequence relation.
Proof for the infix and prefix relation. For the prefix relation, given the εNFA A and string w, we
want to compute the minimum or maximum length of a prefix of w accepted by A. We build in
time O(|w|m) the product graph GA,w , then we interpret (0, q0 ) as the vertex s and we add a new
vertex t with an ε-labelled edge with weight 0 from (i, qf ) to t for each i ∈ {0, . . . , |w|}. Let us call
the resulting graph G and note that it can be constructed in time O(|w|m). It is now obvious that
paths from s to t in G are in one-to-one-correspondence with accepting runs over the prefixes of w,
and that the weight of such a path is the length of the corresponding prefix. Hence, Lemma 5.2
can be used to solve the min- and max-variants of the matching problem for the prefix relation in
time O(|G|) = O(|w|m), including the explicit computation of the witnessing string (as the label
of the resulting path).
For the infix relation, we build again the product graph GA,w and we add two vertices:
• A source vertex s with ε-labelled edges with weight 0 from s to (i, q0 ) for each i ∈ {0, . . . , |w|},
where q0 is the initial state of A.
• A target vertex t with ε-labelled edges with weight 0 from (i, qf ) to t for each i ∈ {0, . . . , |w|},
where qf is the final state of A.
We call this graph again G and note that its construction can be done in time O(|w|m). Moreover
paths from s to t in G are in one-to-one-correspondence with accepting runs over the infixes of w,
with the weight of the path being the length of the corresponding infix. Hence, we conclude again
with Lemma 5.2.
Proof for the extension and left-extension relation. For the left-extension relation, given the εNFA
A with state set Q and string w, we want to compute the minimum or maximum length of a string
vw accepted by A across all possible choices of v ∈ Σ∗ . We build in time O(|w|m) the product
graph GA,w . Further, we add to GA,w a copy of the automaton A: its vertices are {q← | q ∈ Q}
′
and for each transition (q, x, q ′ ) in A we add an edge from q← to q←
labelled x and having weight 0
if x = ε and weight 1 otherwise. Last, for each state q ∈ Q, we add to GA,w an ε-labelled edge
with weight 0 from q← to (0, q). We call the thus constructed graph G and we note that it can be
constructed in time O(|w|m). Let the source vertex s of G be (q0 )← , and let the target vertex t
of G be (|w|, qf ).
Let us now characterise the st-paths in G. Any st-path in G can be decomposed in two parts:
first a path from s to some q← with q ∈ Q, which is a path labelled with a string v ∈ Σ∗ , with
weight |v| and such that there is a run from q0 to q in A when reading v; and second a path from
(0, q) to (|w|, qf ), which is a path labelled by w, with weight |w|, and which witnesses that there
is a run from q to qf in A when reading w. Hence, vw is a left-extension of w accepted by A.
19

<!-- page 20 -->

Conversely, for any string v ∈ Σ∗ such that the left-extension vw of w is accepted by A, there is
a run of A over v going from q0 to q and a run of A over w going from q to qf , and so there is an
st-path in G going via q← and (0, q) and having weight |vw|.
This means that the lengths of left-extensions of w accepted by A correspond to the weights of
st-paths in G. Thus, we conclude with Lemma 5.2 that we can solve the min- and max-variants
of the matching problem for the left-extension relation, including the computation of witnessing
strings, in time O(|w|m).
For the extension relation, given the εNFA A with state set Q and string w, we want to compute
the minimum or maximum length of vwv ′ for v, v ′ ∈ Σ∗ such that vwv ′ is accepted by A. We
build the product graph GA,w . Further, we add to GA,w two copies of the automaton A: one on
vertices {q← | q ∈ Q} defined like previously (i.e., for each transition (q, x, q ′ ) in A we add an
′
edge from q← to q←
labelled x and having weight 0 if x = ε and weight 1 otherwise) and one on
vertices {q→ | q ∈ Q} defined in exactly the same way but replacing the “←” subscripts by “→”
subscripts. Last, for each state q ∈ Q we add an ε-labelled edge with weight 0 from q← to (0, q),
′
and for each state q ′ ∈ Q we add an ε-labelled edge with weight 0 from (|w|, q ′ ) to q→
to the
graph. We call the thus constructed graph G and we note that this construction can be done in
time O(|w|m). Let the source vertex s of G be (q0 )← and let the target vertex t of G be (qf )→ .
Let us now characterise the st-paths in G. Any st-path in G can be decomposed in three parts:
• First a path from s to q← for some q ∈ Q, which is a path labelled with a string v ∈ Σ∗ ,
with weight |v|, and such that there is a run from q0 to q in A when reading v;
• Second, a path from (0, q) to (|w|, q ′ ) for some q ′ ∈ Q, which is a path labelled by w, with
weight |w|, and that is witnessing that there is a run from q to q ′ in A when reading w;
• Third, a path from (q ′ )→ to t labelled with a string v ′ ∈ Σ∗ , with weight |v ′ | and such that
there is a run from q ′ to qf in A when reading v ′ .
Hence, vwv ′ is an extension of w accepted by A.
Conversely, for any strings v, v ′ ∈ Σ∗ such that the extension vwv ′ of w is accepted by A,
there is a run of A over v going from q0 to q, a run of A over w going from q to q ′ and a run
′
and having
of A over v ′ going from q ′ to qf , and so there is an st-path in G going via q← and q→
′
weight |v| + |w| + |v |.
This means that the lengths of extensions of w accepted by A correspond to the weights of
st-paths in G. Thus, we conclude with Lemma 5.2 that we can solve the min- and max-variants of
the matching problem for the extension relation, including the computation of witnessing strings,
in time O(|w|m).
Proof for the subsequence and supersequence relation. For the subsequence relation, given the εNFA
A with state set Q and string w, we want to compute the minimum or maximum length of a subsequence of w accepted by A. We build in time O(|w|m) the product graph GA,w and add edges
with weight 0 and label ε from (i, q) to (i + 1, q) for each state q of A and each 0 ≤ i < |w|.
These additional edges are called extra-edges in the following. Note that the extra-edges mimic
the self-loops (q, b, q) for every b ∈ Σ added in the εNFA Asub defined in Section 3, relative to the
original εNFA A. We call this graph G and note that it can be constructed in time O(|w|m). Let
the source s of G be (0, q0 ) and let the target t be (|w|, qf ).
Let us describe the st-paths in G. For any subsequence u of w accepted by A, we can build a
path of weight |u| from s to t in G following an accepting run ρ over w of the εNFA Asub defined
in Section 3. (Note that we do not actually build the automaton Asub in the construction: we only
use it for the correctness proof. This is important because Asub has size O(n|Σ| + m), so generally
we cannot afford to build it.) More precisely, when ρ takes a transition (q, x, q ′ ) of Asub and the
prefix w[1 : i] for i ∈ {0, 1, . . . , |w|} has already been consumed before the transition (q, x, q ′ ),
then there are two possible cases. Either (q, x, q ′ ) is an original transition of A, in which case
we take the corresponding x-labelled edge from (i, q) to (i + 1, q ′ ) of G if x = w[i + 1], and the
corresponding ε-labelled edge from (i, q) to (i, q ′ ) of G if x = ε. Or (q, x, q ′ ) is not a transition
20

<!-- page 21 -->

of A, which means that it is a self-loop (q, w[i + 1], q ′ ) with q = q ′ , in which case we take the
extra-edge from (i, q) to (i + 1, q) of G. The weight of this st-path corresponds to the number of
symbols read by transitions of Asub which are transitions of A, i.e., the number of symbols that
are part of the witnessing subsequence u; and so the weight of the st-path is |u| and its label is u.
Conversely, given an st-path in G, we can build an accepting run ρ of Asub over w by taking the
transitions corresponding to the edges of G. More precisely, for every x-labelled edge of G from
(i, q) to (i+1, q ′ ) or from (i, q) to (i, q ′ ) that is not an extra-edge, we take the transition (q, x, q ′ ) of
Asub , and for every extra-edge of G from (i, q) to (i + 1, q), we take the transition (q, w[i + 1], q) of
Asub . This witnesses that w has a subsequence accepted by A, namely, the subsequence u formed
of those symbols read by transitions of A in ρ. The number of such symbols, which is the length
of u, is the weight of the st-path; and the string u is the label of the st-path.
This means that the lengths of subsequences of w accepted by A correspond to the weights of
st-paths in G. Thus, we conclude with Lemma 5.2 that we can solve the min- and max-variant
of the matching problem for the subsequence relation, including the computation of witnessing
strings, in time O(|w|m).
For the supersequence relation, given the εNFA A with state set Q and string w, we want to
compute the minimum or maximum length of a supersequence of w accepted by A. We build in
time O(|w|m) the product graph GA,w and for each transition (q, b, q ′ ) in A with b ∈ Σ and each
i ∈ {0, . . . , |w|} we add a b-labelled edge with weight 1 from (i, q) to (i, q ′ ). These additional edges
are called extra-edges in the following. Note that the extra-edges mimic the ε-transitions added
in the εNFA Asup defined in Section 3, relative to the original εNFA A. We call this graph G and
note that it can be constructed in time O(|w|m). Let the source s of G be (0, q0 ) and let the target
t be (|w|, qf ).
Let us describe the st-paths in G. For any supersequence u of w accepted by A, we can build
a path of weight |u| from s to t in G following an accepting run ρ over w of the εNFA Asup . More
precisely, when ρ takes a transition (q, x, q ′ ) of Asup and the prefix w[1 : i] for i ∈ {0, 1, . . . , |w|} has
already been consumed before the transition (q, x, q ′ ), then there are two possible cases. Either
(q, x, q ′ ) is an original transition of A, in which case we take the corresponding x-labelled edge
from (i, q) to (i + 1, q ′ ) of G if x = w[i + 1], and the corresponding ε-labelled edge from (i, q)
to (i, q ′ ) of G if x = ε. Or (q, x, q ′ ) is not a transition of A, which means that it has to be an
ε-transition (q, ε, q ′ ), in which case we take an extra-edge from (i, q) to (i, q ′ ) of G. Note that for
any ε-transition (q, ε, q ′ ) of Asup that is not a transition of A, there are several (and at least one)
transitions (q, b, q ′ ) with b ∈ Σ in A; thus, there are also several (and at least one) extra-edges
from (i, q) to (i, q ′ ) that differ in their labels. We take just any of those extra-edges. The weight
of this st-path corresponds to the number of edges labelled by a symbol from Σ (i.e., transitions
in ρ that read symbols from Σ and that are transitions of A) plus the number of extra-edges (i.e.,
the number of ε-transitions of Asup in ρ that are not transitions in A). Thus, the weight of the
st-path is |u| and its label is u.
Conversely, given an st-path in G, we can build an accepting run ρ of Asup over w by taking
the transitions corresponding to the edges of G. More precisely, for every x-labelled edge of G
from (i, q) to (i + 1, q ′ ) or from (i, q) to (i, q ′ ) that is not an extra-edge, we take the transition
(q, x, q ′ ) of Asup , and for every extra-edge of G from (i, q) to (i, q ′ ) labelled with some b ∈ Σ, we
take the transition (q, ε, q ′ ) of Asup (such a transition exists by definition of Asup ). This witnesses
that w has a supersequence accepted by A, namely the label u of the st-path in G. The number
of such symbols, which is the length of u, is the weight of the st-path; and the string u is the label
of the st-path.
This means that the lengths of supersequences of w accepted by A correspond to the weights
of st-paths in G. Thus, we conclude with Lemma 5.2 that we can solve the min- and max-variant
of the matching problem for the supersequence relation, including the computation of witnessing
strings, in time O(|w|m).

21

<!-- page 22 -->

6

Universal Problem Variants

We now consider the universal variants, i.e., checking whether all strings u ⪯ w are in L(A).
Theorem 6.1. The universal-variant of the matching problem can be solved in time O(|w|m) for
2
the prefix relation and in time O(|w| m) for the infix relation. It is PSPACE-complete for the
extension and left-extension relation, coNP-complete for the subsequence relation, and PSPACEcomplete for the supersequence relation.
Proof. We will prove the statements for the different relations separately.
• The prefix relation: We perform a state-set simulation and check in every step of the simulation whether there is at least one accepting state in the set of active states.
• The infix relation: We can solve the problem by solving the prefix-case for every suffix of w,
2
which yields a running time of O(|w| m).
• The extension and left-extension relation: To solve the extension-case in PSPACE, we can
construct an εNFA A′ for Λ⪯ext (w) and then check whether L(A′ ) ⊆ L(A), which can be done
in PSPACE. The left-extension case is analogous: construct an εNFA for Λ⪯lext (w).
The PSPACE-hardness follows from the fact that for w = ε the extension- and left-extensioncase of the universal matching problem is the same as εNFA universality, which is PSPACEhard.
• The subsequence relation: The problem is obviously in coNP, since we can just guess a
subsequence of w and check whether it is rejected by the εNFA.
For the coNP-hardness, we reduce from the negation of the NP-hard Boolean satisfiability
problem on conjunctive normal form (CNF) formulas with at most 3 literals per clause,
i.e., we reduce from the coNP-hard problem of checking whether a 3-CNF formula F is not
satisfiable. Take a 3-CNF formula F with n variables. We build an εNFA A that accepts all
{0, 1}-strings of length at most n − 1, accepts all {0, 1}-strings of length between n + 1 and
2n, and accepts all length-n {0, 1}-strings that represent non-satisfying assignments of F .
Indeed, an εNFA A′ for the latter strings can easily be built by using for every clause a path
that reads exactly the assignments that do not satisfy this particular clause, and taking the
disjunction across the clauses of F . Now let w = (01)n and note that Λ⪯sub (w) contains only
{0, 1}-strings of length at most 2n, and that it does contain every {0, 1}-string of length n.
Therefore, if the εNFA A accepts all strings from Λ⪯sub (w) then A′ accepts every possible
length-n {0, 1}-string, and every such string represents a non-satisfying assignment of F ,
so we know that F is not satisfiable. If F is not satisfiable, then every possible length-n
{0, 1}-string must be accepted by the εNFA A′ , which means that the εNFA A must accept
all strings from Λ⪯sub (w).
• The supersequence relation: To show the PSPACE upper bound, we can easily construct
in PTIME an εNFA Aw,sup that accepts exactly the supersequences of the input string w –
note that this is an easy special case of [BLS15, Lemma 8] which we reviewed in Section 3.
As Aw,sup accepts Λ⪯sup (w), we can then check whether L(Aw,sup ) ⊆ L(A), which can be done
in PSPACE.
The PSPACE-hardness follows again from the fact that for w = ε the supersequence-case
of the universal matching problem is the same as εNFA universality, which is PSPACEhard.

7

Conditional Lower Bounds

We now present the conditional lower bounds that are stated in Table 2. These bounds apply to
all problems for which we showed a time complexity higher than the optimal linear complexity
22

<!-- page 23 -->

O(|w| + m). Further, they match our upper bounds. The section is structured in two parts. We
first show lower bounds for the infix, prefix, extension and left-extension relations: Theorem 7.1
covers the matching problem and its min- and max-variants, and Theorems 7.2 and 7.4 cover lower
bounds for the universal-variant with the infix and prefix relations; note that the universal-variant
for extension and left-extension was shown earlier to be PSPACE-complete (Theorem 6.1). Second, we show lower bounds for the min- and max-variants of the sub- and supersequence relations;
the case of the universal-variant was covered by Theorem 6.1, too.

7.1

Infix, Prefix, Extension and Left-Extension Relations

We first observe that SETH-based lower bounds for the ⪯in -, ⪯pre -, ⪯ext - and ⪯lext -matching
problems can be obtained by minor adaptations of the original construction from [BI16]. Moreover,
since the min- and max-variants also solve the non-quantitative variants of the matching problem,
we can conclude that these lower bounds also hold for the quantitative variants:
Theorem 7.1. If the matching problem for the infix, prefix, extension and left-extension relation
can be solved in time O((|w|m)1−ϵ ) for some ϵ > 0, then SETH fails. The same holds for the minand max-variants, even if we operate on an alphabet of constant size, and even if we only require
the length of the answer strings.
Proof. Let us first recall that if w ∈ L(M ) for a given string w and εNFA M can be decided in
time O((|w| · |M |)1−ϵ ) for some ϵ > 0, then SETH fails (see [BI16]); this result even holds on an
alphabet of constant size.
Let w be an input string and M an εNFA. In time O(|M |) we can construct an εNFA M ′ with
L(M ′ ) = {#} · L(M ) · {#}, where # is a fresh symbol (i.e., not used on any transition of M ).
Note that w ∈ L(M ) if and only if #w# ∈ L(M ′ ). Moreover, the only infix, prefix, left-extension
or extension of #w# that could possibly be accepted by M ′ is the string #w# itself. Indeed, if
M ′ accepts a proper infix or prefix, then it would accept a string that does not start or end with
#, and if M ′ accepts a proper left-extension (or extension), then M would accept a string that
contains an occurrence of #. Hence, the following statements are equivalent:
• M accepts w.
• M ′ accepts #w#.
• M ′ accepts an infix of #w#.
• M ′ accepts a prefix of #w#.
• M ′ accepts a left-extension of #w#.
• M ′ accepts an extension of #w#.
If we could solve the matching problem for the infix, prefix, extension and left-extension relation
on the instance M ′ and #w# in time O((|#w#| · |M ′ |)1−ϵ ) for some ϵ > 0, then we would have
decided whether w ∈ L(M ) in time O((|w| · |M |)1−ϵ ).
This shows that if the matching problem for the infix, prefix, extension and left-extension
relation can be solved in time O((|w| · |M |)1−ϵ ) for some ϵ > 0, then SETH fails.
Obviously, each of the min- or max-variant of the matching problem for the infix, prefix,
extension and left-extension relation implicitly also solves the matching problem for the infix,
prefix, extension and left-extension relation: this is true even if the min- and max-variants are
only required to compute the length of their answers (and not the strings themselves). Thus, the
lower bounds carry over to the quantitative variants as well.
Further, we can show that the O(|w|m) upper bound for the universal variant of the matching
problem for the prefix relation is also optimal under SETH:

23

<!-- page 24 -->

Theorem 7.2. If the universal variant of the matching problem for the prefix relation can be
solved in time O((|w|m)1−ϵ ) for some ϵ > 0, then SETH fails. This holds even if we operate on
an alphabet of constant size.
Proof. Let w be an input string and M an εNFA. In time O(|M |) we can construct an εNFA M ′
with L(M ′ ) = (L(M ) · {#}) ∪ Σ∗ for a new symbol #.
If w ∈ L(M ), then w# ∈ L(M ′ ), which means that Λ⪯pre (w#) ⊆ L(M ′ ). If Λ⪯pre (w#) ⊆ L(M ′ ),
then w# ∈ L(M ′ ), which means that w ∈ L(M ). Thus, if we could decide whether Λ⪯pre (w#) ⊆
L(M ′ ) in time O((|w#| · |M ′ |)1−ϵ ) for some ϵ > 0, then we would also have decided whether
w ∈ L(M ) in time O((|w| · |M |)1−ϵ ). This concludes using the results of [BI16].
Using a slightly more complicated reduction5 from the orthogonal vectors problem, we can also
show that the O(|w|2 m) upper bound for the universal variant of the matching problem for the
infix relation is conditionally optimal under SETH. Recall that, in the orthogonal vectors problem
(OV), we are given two sets U = {u1 , . . . , un }, V = {v1 , . . . , vn } of d-dimensional Boolean vectors,
with d = ω(log n), and want to determine whether there are two orthogonal vectors ui and vj , i.e.,
Pd
whether there exist i, j ∈ [n] such that ui · vj = k=1 ui [k] · vj [k] = 0. For convenience, we also use
the notation u ⊥ v to denote that vectors u and v are orthogonal. The following is well-known:
Lemma 7.3 ([Wil05], see, e.g., [ABDN18], Conjecture 1.1). If there is an algorithm that solves
OV in time O(n2−ϵ poly(d)) for some ϵ > 0, then SETH fails.
Using the result above, we can now show:
Theorem 7.4. If we can solve the universal variant of the ⪯in -matching problem on alphabet
Σ = {0, 1} in time O(|w|2−ϵ poly(m)) for some ϵ > 0, then SETH fails.
Proof. We reduce from OV. We are given two sets U = {u1 , . . . , un }, V = {v1 , . . . , vn } of Boolean
d-dimensional vectors, where d = ω(log n), and want to determine whether there exist i, j ∈ [n]
such that ui · vj = 0. We will build an NFA A and string w ∈ Σ∗ such that there is an infix of w
that is not in L(A) if and only if there exist i, j ∈ [n] such that ui · vj = 0. Let h : Σ∗ → Σ∗ be the
homomorphism defined by h(0) = 000 and h(1) = 010, and define B = 11011 and E = 11111 as
beginning and end markers. We can now construct
w = (B h(u1 )) (B h(u2 )) . . . (B h(un )) (h(v1R ) E) (h(v2R ) E) . . . (h(vnR ) E),
where the parentheses are only used for the convenience of the reader and are not part of the
string itself. It is not hard to see that ui · vj ̸= 0 if and only if there is some k ∈ [d] such that
h(ui )[3k − 2 : 3k] and h(vj )[3k − 2 : 3k] are both equal to h(1) = 010, namely, h(ui )[3k − 2 : 3k] =
h(vjR )[3(d − k + 1) − 2 : 3(d − k + 1)] = 010.
Let us now build the NFA A that rejects exactly the infixes of w that witness ui · vj = 0 for
some i, j ∈ [n]. We construct A as the union of three NFAs AB , AE , and A′ . The NFA AB accepts
all strings that do not start with B, and the NFA AE accepts all strings that do not end with E;
it is not hard to achieve |AB | = |AE | = O(1). Finally, the NFA A′ accepts all strings of the form
B Σ3(k−1) 010 Σ∗ 010 Σ3(k−1) E for some k ∈ [d]. By a naive construction of A′ , we can achieve
|A′ | = O(d2 ), which suffices for our purpose. Overall, we thus obtain |A| = O(d2 ).
Let us give an intuitive explanation of which infixes are accepted by the NFA A. Let (B h(ui ))
(resp., (h(vjR ) E)) be the block corresponding to ui (resp., vj ). Then, intuitively, we use AB to
match all infixes that do not start at the beginning of some ui -block. Analogously, we use AE to
match all infixes that do not end at the end of some vj -block. This means that AB and AE accept
all ‘ill-formed’ infixes, whereas A′ only has to consider infixes that consist of complete blocks, and
accepts the infix starting at the ui -block and ending at the vj -block if and only if there is some
k ∈ [d] witnessing h(ui [k]) = h(vjR [d − k + 1]) = h(vj [k]) = 010 = h(1), i.e., ui [k] · vj [k] = 1.
Let us now show that Λ⪯in (w) ̸⊆ L(A) (i.e., there is an infix u ⪯in w such that u ∈
/ L(A)) if
and only if there exist i, j ∈ [n] with ui · vj = 0. First, assume that ui and vj are orthogonal
5 We note that this bound was first discussed online, see [Ama].

24

<!-- page 25 -->

and let wij = (B h(ui )) . . . (h(vjR ) E). Since wij starts with B and ends with E, it is rejected
by both AB and AE . The NFA A′ , and therefore A, also rejects wij : assume, by contradiction,
that wij ∈ L(A′ ), then there must be a k ∈ [d] such that ui [k] = vj [k] = 1 (and wij must be of
the form B Σ3(k−1) 010 Σ∗ 010 Σ3(k−1) E), contradicting the fact that ui and vj are orthogonal.
On the other hand, assume that w′ ∈
/ L(A) for some infix w′ of w. Since w′ is rejected by AB
′
′′
and AE , we know that w = B w E for some w′′ ∈ Σ∗ . The definition of h and the definition
of B and E then ensure that any factor 11011 and 11111 of w must correspond to a B that
marks the beginning of some ui -block and an E that marks the end of some vj -block. Thus, we
know that there exist i, j ∈ [n] such that w′ = (B h(ui )) . . . (h(vjR ) E), i.e., w′ cannot start or
end in the middle of some block. Finally, w′ ∈
/ L(A′ ) yields that there is no k ∈ [d] such that
R
h(ui [k]) = h(vj [d − k + 1]) = 010 = h(1), thus, ui · vj = 0, i.e., ui and vj are orthogonal.
Now, since |w| = O(nd) and m = |A| = O(d2 ), if we can solve the universal variant of the
infix matching problem in O(|w|2−ϵ poly(m)) = O(n2−ϵ poly(d)) time, for some ϵ > 0, then SETH
fails.

7.2

Subsequence and Supersequence Relations

With respect to ⪯sub and ⪯sup , we have proven optimal linear upper bounds for the non-quantitative
variants in Section 4. However, for the min- and max-variants, we only have upper bounds of
O(|w|m), which we will now complement with conditional lower bounds.
We can easily construct εNFAs Au,sub and Au,sup that accept exactly the subsequences of
a string u (and supersequences of a string u, respectively) – note that this is an easy special
case of [BLS15, Lemma 8] which we reviewed in Section 3. This means that the max-variant
of the ⪯sub -matching problem applied to the εNFA Au,sub and a string v amounts to computing
the longest common subsequence of u and v. Likewise, the min-variant of the ⪯sup -matching
problem applied to Au,sup and v admits a reduction from the shortest common supersequence
problem. Thus, solving these problems would allow us to solve the longest common subsequence
problem (or shortest common supersequence problem, respectively) for u and v. This means
that the known SETH-conditional lower bounds on the longest common subsequence and shortest
common supersequence problems carry over to these quantitative variants of the matching problem
(see [BK15, ABW15]). Note that these lower bounds already hold when computing the length of
the sequences, and further they already hold for constant-sized alphabets so it is not a problem
that the automaton Au,sup generally has size Θ(|u| · |Σ|). Thus, we obtain:
Theorem 7.5. If the max-variant of the matching problem for the subsequence relation or the minvariant of the matching problem for the supersequence relation can be solved in time O((|w|m)1−ϵ )
for some ϵ > 0, then SETH fails. This holds even if we operate on an alphabet of constant size
and if we only require the length of the answer strings.
Proof. Let us first recall that there is a constant-sized alphabet Σ such that if we can compute
the length of the longest common subsequence for two strings u, v ∈ Σ∗ in time O((|u| · |v|)1−ϵ )
for some ϵ > 0, then SETH fails (see [BK15, ABW15]). We take Σ to be this constant-sized
alphabet in the rest of this proof. Moreover, the length p of the longest common subsequence of
u and v and the length q of the shortest common supersequence of u and v obey the relationship
p = |u| + |v| − q (see [BHR00]), so from the length of a longest common subsequence s for u and v
we can obtain in linear time the length of a shortest common supersequence of u and v and vice
versa. This implies that if we can compute the length of the shortest common supersequence for
two strings u and v in time O((|u| · |v|)1−ϵ ) for some ϵ > 0, then SETH fails as well.
Now assume that the max-variant of the matching problem for the subsequence relation can
be solved in time O((|w|m)1−ϵ ). Let u and v be two strings over Σ. We construct an εNFA Au
that accepts exactly the subsequences of u, and that has O(|u|) states and transitions. This can
be done in time O(|u|): indeed, a more general result from [BLS15, Lemma 8] was reviewed in
Section 3. Now the longest subsequence of v that is accepted by Au is exactly the longest common
subsequence of u and v. Thus, if the max-variant of the matching problem for the subsequence

25

<!-- page 26 -->

relation can be solved in time O((|v| · |Au |)1−ϵ ), then we can also compute the length of the longest
common subsequence of u and v in time O((|u| · |v|)1−ϵ ).
For the min-variant of the matching problem for the supersequence relation, assume that it can
be solved in time O((|w|m)1−ϵ ). Let u and v be two strings over Σ. We build an εNFA Au that
accepts exactly the supersequences of u, which has O(|u|) states and O(|u|·|Σ|) transitions: this can
be done in time O(|u| · |Σ|), again as a special case of the more general result from [BLS15, Lemma
8] reviewed in Section 3. As |Σ| is a constant, the construction is in O(|u|). Now, solving the
min-variant of the matching problem for the supersequence relation in the prescribed complexity
gives us the length of the shortest common supersequence of u and v in time O((|u| · |v|)1−ϵ ).
This leaves the question of the optimality of O(|w|m)-time for the min-variant of the ⪯sub matching problem and the max-variant of the ⪯sup -matching problem. For these, similarly as
in Theorem 7.4, we provide SETH-based conditional lower bounds via a reduction from the orthogonal vectors problem. However, now we consider its unbalanced variant (UOV), in which we
are given two sets U = {u1 , . . . , un }, V = {v1 , . . . , vt } of Boolean d-dimensional vectors, where
d = ω(log n) and t = nα for α ∈ (0, 1], and want to determine whether there exist i ∈ [n], j ∈ [t]
such that ui ⊥ vj . The Unbalanced Orthogonal Vector Hypothesis (UOVH) states that for every
ϵ > 0 there is no algorithm for UOV that runs in O(n1+α−ϵ poly(d)) time. As UOVH is equivalent
to the Orthogonal Vectors Hypothesis [BK18, BN21], an algorithm for UOV achieving such a
running time bound would refute SETH (recall Lemma 7.3). We can now show:
Theorem 7.6. If the max-variant of the matching problem for the supersequence relation or the
min-variant of the matching problem for the subsequence relation can be solved in time O((|w|m)1−ϵ ),
for some ϵ > 0, then SETH fails. This holds even if we operate on an alphabet of constant size
and if we only require the length of the answer strings.
Proof. In order to simplify the presentation we operate on regular expressions instead of εNFAs,
as an εNFA can be created from a regular expression in linear time. For a string s and regular
expression r, let MaxSup(s, r) be the length of the longest supersequence of s that matches r and
MinSub(s, r) be the length of the shortest subsequence of s Q
that matches r. Further, for strings
a1 , . . . , ad ∈ Σ∗ and for an integer predicate P, we denote by j∈[d]:P(j) aj the concatenation of all
∗
aj ∈ Σ∗ over the j ∈ [d] such that P holds for j. For example, let b = 010110
Q∈ {0, 1} be a string of
length 6 and let a1 , . . . , a6 be strings over some larger alphabet Σ. Then j∈[6]:b[j]=1 aj = a2 a4 a5
concatenates all strings aj for which b[j] = 1 holds.
We will present a reduction from UOV in which we are given two sets of binary vectors
U = {u1 , . . . , un }, V = {v1 , . . . , vt } ⊆ {0, 1}d where d = ω(log n) and t = nα for α ∈ (0, 1] to
be chosen later. First we consider the alphabet Σ = {#, $, c1 , c2 , . . . , cd } of non-constant size and
later show how to reduce it. Observe that the following identities follow immediately from the
properties of the subsequence relation:
Claim 7.7. Let A1 , . . . , Ak , B1 , . . . , Bk be strings over Σ \ {#} and A := A1 #A2 # . . . Ak # and
B := B1 #B2 # . . . Bk #. Then the following hold:
• A ⪯sub B iff for all i ∈ [k] we have Ai ⪯sub Bi
P
• MaxSup(A, B) = i∈[k] (MaxSup(Ai , Bi ) + 1)
P
• MinSub(A, B) = i∈[k] (MinSub(Ai , Bi ) + 1)
Note that, in the statement above and throughout the proof, we abuse notation to see strings as
regular expressions, i.e., the string B in the expressions MaxSup(A, B) and MinSub(A, B) denotes
the regular expression that accepts precisely B.
Claim 7.7 will allow us to combine independent gadgets into one big string, ensuring that
they do not affect each other. We start with the max-variant of the matching problem for the
supersequence relation.

26

<!-- page 27 -->

Max-Variant of the Matching Problem for the Supersequence Relation. Given an
instance of UOV with sets of vectors U = {u1 , . . . , un }, V = {v1 , . . . , vt } ⊆ {0, 1}d we define
Z = Πj∈[d] cj and the following two functions that operate on binary vectors:
F (v) = Πj∈[d]:v[j]=1 cj

G(u) = Πj∈[d]:u[j]=0 cj $1+∥u∥1
Note that F (v) ⪯sub Z so MaxSup(F (v), Z) = |Z| = d. Further observe that F (v) ⪯sub
G(u) ⇐⇒ v ⊥ u, so if v ⊥ u we have MaxSup(F (v), G(u)) = |G(u)| = d + 1.
Consider the regular expression Y := Z ∨ G(u1 ) ∨ G(u2 ) . . . ∨ G(un ). Then we have:
MaxSup(F (v), Y ) = d + 1[∃i∈[n] v ⊥ ui ],

(1)

where the function 1 is defined as 1[P] equals 1 if the predicate P holds, and 0 otherwise.
We now use Claim 7.7 to combine our construction for various vectors v in such a way that
they do not affect each other. Let w = F (v1 )#F (v2 )# . . . F (vt )# and R = (Y #)t . In order to
avoid clutter, whenever i is not quantified, we mean i ∈ [t]. Then we have:
MaxSup(w, R) = MaxSup (Πi (F (vi )#), Πi (Y #))
X
=
(MaxSup (F (vi ), Y ) + 1)

(definition of w and R)
(Claim 7.7)

i

=

X

d + 1[∃k∈[n] vi ⊥ uk ] + 1



(Equation 1)

i

= t(d + 1) + |{i : ∃k∈[n] vi ⊥ uk }|
To conclude, there exists an orthogonal pair of vectors (u, v) ∈ U × V iff MaxSup(w, R) > t(d + 1).
Now suppose that there exists an algorithm that solves the max-variant of the matching problem for the supersequence relation in time O((|w||R|)1−ϵ ) for some ϵ > 0. As |w| = O(td) and
|R| = O(ntd), this yields an algorithm for UOV in time O((nt2 d2 )1−ϵ ) = O(n(1+2α)(1−ϵ) poly(d)).
Choosing α := ϵ/2 we obtain an algorithm for UOV that runs in time O(n(1+ϵ)(1−ϵ) poly(d)) =
2
O(n1+α−(α+ϵ ) poly(d)) which violates UOVH. This concludes the proof for the case of nonconstant-size alphabets.
Constant-Size Alphabet. Now we show how to adjust the above construction to operate on
constant-size alphabets. Note that the only strings containing characters c⋆ in their definition are
Z, F (v), and G(u) where both F (v) and the prefix of G(u) before $’s are subsequences of Z.
Let Σ′ = {0, 1, $, #} and define a function δ by δ(1) := 10 and δ(0) := 0. We define Z ′ :=
(δ(1))d = (10)d , F ′ (v) = Πj∈[d] δ(v[j]), and G′ (u) := Πj∈[d] δ(¬u[j]) $1+∥u∥1 where ¬x = 1 − x is
the negation of a bit. Similarly as in Claim 7.7, now the 0’s serve as separators that ensure that
1’s are matched only between the corresponding positions, because we have the same number of
0’s in Z ′ , F ′ (v) and G′ (u).
Note that we have F ′ (v) ⪯sub Z ′ and MaxSup(F ′ (v), Z ′ ) = |Z ′ | = 2d. Next, we have F ′ (v) ⪯sub
G′ (u) ⇐⇒ v ⊥ u, so if v ⊥ u we have MaxSup(F ′ (v), G′ (u)) = |G′ (u)| = 2d + 1. We define Y ′ , w′
and R′ as before, but now using the new expressions Z ′ , G′ and F ′ . Then, MaxSup(F ′ (v), Y ′ ) =
2d + 1[∃i∈[n] v ⊥ ui ] and finally MaxSup(w′ , R′ ) = t(2d + 1) + |{i : ∃k∈[n] vi ⊥ uk }|. The result of
the running time analysis is unchanged for the new construction. This concludes the proof for the
case of constant-size alphabet Σ′ = {0, 1, $, #} for the max-variant of the matching problem for
the supersequence relation.
Min-Variant of the Matching Problem for the Subsequence Relation. The reduction
for this problem will be a slight adjustment of the reduction from the previous paragraphs. Again
we start with the construction for non-constant size alphabets. Given an instance of UOV, we

27

<!-- page 28 -->

define Z = $d+1 and define the following two functions that operate on binary vectors:

F (v) = Πj∈[d]:v[j]=0 cj $d+1

G(u) = Πj∈[d]:u[j]=1 cj $∥u∥0
Note that Z ⪯sub F (v) so MinSub(F (v), Z) = |Z| = d + 1. Further observe that G(u) ⪯sub
F (v) ⇐⇒ v ⊥ u so if v ⊥ u, we have MinSub(F (v), G(u)) = |G(u)| = d.
Consider the regular expression Y := Z ∨ G(u1 ) ∨ G(u2 ) . . . ∨ G(un ). Then we have:
MinSub(F (v), Y ) = d + 1 − 1[∃i∈[n] v ⊥ ui ]
Let w = Πi∈[t] (F (vi )#) and R = (Y #)t . By Claim 7.7, similarly as before, we deduce:
MinSub(w, R) = t(d + 2) − |{i : ∃k∈[n] vi ⊥ uk }|
and hence there exists an orthogonal pair of vectors (u, v) ∈ U × V iff MinSub(w, R) < t(d + 2).
The rest of the analysis is identical to that of the previous paragraphs. Namely, as |w| = O(td)
and |R| = O(ntd), this yields an algorithm for UOV in time O((nt2 d2 )1−ϵ ) = O(n(1+2α)(1−ϵ) poly(d)).
Choosing α := ϵ/2 we obtain an algorithm for UOV that runs in time O(n(1+ϵ)(1−ϵ) poly(d)) =
2
O(n1+α−(α+ϵ ) poly(d)) which violates UOVH.
Constant-Size Alphabet. Reducing the alphabet size to a constant is very similar to the case
of the max-variant for the supersequence relation. Let Σ′ := {0, 1, $, #} and define a function δ
as before by δ(1) := 10 and δ(0) := 0. We define Z ′ := 0d $d+1 , F ′ (v) := (Πj∈[d] δ(¬v[j]))$d+1 , and

G′ (u) := Πj∈[d] δ(u[j]) $∥u∥0 . Again Z ′ ⪯sub F ′ (v) but now MinSub(F ′ (v), Z ′ ) = |Z ′ | = 2d + 1.
Next, G′ (u) ⪯sub F ′ (v) ⇐⇒ v ⊥ u, so if v ⊥ u we have MinSub(F ′ (v), G′ (u)) = |G′ (u)| = 2d.
We define Y ′ , w′ and R′ as before, but now using the new expressions Z ′ , G′ and F ′ . Then,
MinSub(F ′ (v), Y ′ ) = 2d+1− 1[∃i∈[n] v ⊥ ui ] and finally MinSub(w′ , R′ ) = t(2d+2)−|{i : ∃k∈[n] vi ⊥
uk }| so MinSub(w′ , R′ ) < t(2d + 2) iff there exists (u, v) ∈ U × V such that u ⊥ v. Further analysis
of the reduction and its running time follows as in the case of the max-variant of the supersequence
relation, which concludes the proof.

8

Conclusion and Future Work

Let us start with a brief summary of the key messages of our results. Firstly, for all our considered
relations ⪯, the ⪯-matching problem and its quantitative variants are in O(|w|m) – the (conditionally optimal) complexity for conventional regex matching. Hence, this generalises standard
regex matching and the longest common subsequence/shortest common supersequence problems.
Moreover, as a surprising and unexpected special case, the non-quantitative ⪯-matching problem
for the subsequence and supersequence relation can be solved in linear time O(|w| + m); for all
other problems, the running time of O(|w|m) is optimal in the sense that O((|w|m)1−ϵ ) cannot
be achieved for any ϵ > 0 assuming SETH. Finally, the intuitively much more difficult task of
checking whether all strings u ⪯ w match the regex, i.e., the universal problem variants, leads
to a more diverse picture ranging from cases that can still be solved efficiently (e.g., O(|w|m) or
O(|w|2 m) for the prefix and infix relation, respectively) to intractable cases.
With respect to the linear time solvable cases of subsequence and supersequence matching, it
might be worthwhile to investigate whether our linear time algorithms can be used as components
within other useful string matching algorithms. For example, in big data scenarios where quadratic
running times are unrealistic, it might be possible to approximate reasonable string analysis tasks
by only performing subsequence or supersequence matching. Let us also point out that none
of our upper bounds rely on heavy algorithmic machinery or data structures and therefore our
asymptotic running times do not hide huge constant factors. Hence, we believe that our algorithms
have straightforward and practically relevant implementations (just like the classical state-set
simulation does).
28

<!-- page 29 -->

In terms of theoretical questions, since the linear time complexity for subsequence and supersequence regex matching does not extend to other natural and simple relations or to the quantitative
variants, it would be interesting to find more string relations with this property, e.g., the subsequence relation with bounded gap size of k: x1 x2 . . . xn ⪯sub,k w ⇔ w = w0 x1 w1 x2 . . . xn wn and
|wi | ≤ k for every i ∈ {1, 2, . . . , n − 1}. However, we expect that such small modifications would
make the complexity quadratic again, i.e., the classical SETH-reduction would apply.
It also seems particularly interesting that we can compute in time O(|w| · |r|) a longest subsequence and a shortest supersequence of w that match r, since this gives rise to a measure of how
much w does not match r. More precisely, we could define ψ(r, w) as (|w| − |u|) + (|v| − |w|) =
|v| − |u|, where u and v are the longest subsequence and shortest supersequence of w that match
r, respectively (and ψ(r, w) = ⊥ if u or v does not exist). Note that ψ(r, w) = 0 ⇐⇒ w ∈ L(r).
With this measure in mind, any regular expression extends from a mere predicate (i.e., a formal
language) to a partial function Σ∗ → N. On the one hand, it might be interesting to investigate
how fast this measure ψ(r, ·) can be computed for other (practically motivated) classes of regular
expressions, e.g., deterministic regular expressions or regular expressions with counters (or even
strictly more powerful language classes). On the other hand, it might be interesting to investigate
whether the measure ψ(r, w) can be employed within other string analysis tasks that go beyond
classical regex matching.
Finally, our tractability results on sub- and supersequence matching can also be seen as
tractability results for the matching problem on automata that satisfy certain conditions, namely,
their language is upward- or downward-closed. Our results imply that, on such automata, we
can perform matching in time O(|w| + m) instead of O(|w|m). One natural question is which
other restrictions on automata make it possible to achieve such improved bounds. Examples in
this direction are the regex classes studied in [BI16, BGKL24, BGL17] or automata classes where
determinisation can be done in polynomial time, e.g., NFAs of bounded co-lex width [CDPP23].

Acknowledgements
The first author was partially supported by the ANR project EQUUS ANR-19-CE48-0019 and by
the Deutsche Forschungsgemeinschaft (DFG, German Research Foundation) – 431183758. The
second author was partially supported by the Polish National Science Centre grant number
2023/51/B/ST6/01505. The third and fourth authors are supported by the German Research
Foundation (Deutsche Forschungsgemeinschaft, DFG) through the Heisenberg project 466789228
and the research project 562495345. The fifth author is supported by the German Research Foundation (Deutsche Forschungsgemeinschaft, DFG) – project number 522576760 (gefördert durch
die Deutsche Forschungsgemeinschaft (DFG) – Projektnummer 522576760).

References
[AAB+ 17]

Renzo Angles, Marcelo Arenas, Pablo Barceló, Aidan Hogan, Juan L. Reutter, and
Domagoj Vrgoc. Foundations of modern query languages for graph databases. ACM
Comput. Surv., 50(5), 2017. doi:10.1145/3104031.

[ABDN18]

Amir Abboud, Karl Bringmann, Holger Dell, and Jesper Nederlof. More consequences
of falsifying SETH and the orthogonal vectors conjecture. In STOC, 2018. doi:
10.1145/3188745.3188938.

[ABJ98]

Parosh Aziz Abdulla, Ahmed Bouajjani, and Bengt Jonsson. On-the-fly analysis
of systems with unbounded, lossy FIFO channels. In CAV, 1998. doi:10.1007/
BFB0028754.

[ABW15]

Amir Abboud, Arturs Backurs, and Virginia Vassilevska Williams. Tight hardness
results for LCS and other sequence similarity measures. In FOCS, 2015. doi:10.
1109/FOCS.2015.14.
29

<!-- page 30 -->

[AFH+ 25]

Duncan Adamson, Pamela Fleischmann, Annika Huch, Tore Koß, Florin Manea,
and Dirk Nowotka. k-universality of regular languages. Inf. Comput., 307, 2025.
doi:10.1016/J.IC.2025.105357.

[Ama]

Antoine Amarilli. Efficiently finding an infix of a word that is rejected by an
NFA. Theoretical Computer Science Stack Exchange. URL: https://cstheory.
stackexchange.com/q/55773.

[AMRS25]

Antoine Amarilli, Florin Manea, Tina Ringleb, and Markus L. Schmid. Linear time
subsequence and supersequence regex matching. In Pawel Gawrychowski, Filip Mazowiecki, and Michal Skrzypczak, editors, MFCS, LIPIcs. Schloss Dagstuhl - LeibnizZentrum für Informatik, 2025. URL: https://doi.org/10.4230/LIPIcs.MFCS.
2025.9, doi:10.4230/LIPICS.MFCS.2025.9.

[AMU+ 17]

Alexander Artikis, Alessandro Margara, Martı́n Ugarte, Stijn Vansummeren, and
Matthias Weidlich. Complex event recognition languages: Tutorial. In DEBS, 2017.
doi:10.1145/3093742.3095106.

[AWW14]

Amir Abboud, Virginia Vassilevska Williams, and Oren Weimann. Consequences of
faster alignment of sequences. In ICALP, 2014. doi:10.1007/978-3-662-43948-7\
_4.

[BC18]

Karl Bringmann and Bhaskar Ray Chaudhury. Sketching, streaming, and fine-grained
complexity of (weighted) LCS. In FSTTCS, 2018. doi:10.4230/LIPICS.FSTTCS.
2018.40.

[BGKL24]

Karl Bringmann, Allan Grønlund, Marvin Künnemann, and Kasper Green Larsen.
The NFA acceptance hypothesis: Non-combinatorial and dynamic lower bounds. TheoretiCS, 3, 2024. doi:10.46298/THEORETICS.24.22.

[BGL17]

Karl Bringmann, Allan Grønlund, and Kasper Green Larsen. A dichotomy for regular
expression membership testing. In FOCS, 2017. doi:10.1109/FOCS.2017.36.

[BHR00]

Lasse Bergroth, Harri Hakonen, and Timo Raita. A survey of longest common subsequence algorithms. In SPIRE, 2000. doi:10.1109/SPIRE.2000.878178.

[BI16]

Arturs Backurs and Piotr Indyk. Which regular expression patterns are hard to
match? In FOCS, 2016. doi:10.1109/FOCS.2016.56.

[BK15]

Karl Bringmann and Marvin Künnemann. Quadratic conditional lower bounds for
string problems and dynamic time warping. In FOCS, 2015. doi:10.1109/FOCS.
2015.15.

[BK18]

Karl Bringmann and Marvin Künnemann. Multivariate fine-grained complexity of
longest common subsequence. In SODA, 2018. doi:10.1137/1.9781611975031.79.

[BLS15]

Georg Bachmeier, Michael Luttenberger, and Maximilian Schlund. Finite automata
for the sub- and superword closure of CFLs: Descriptional and computational complexity. In LATA, 2015. doi:10.1007/978-3-319-15579-1\_37.

[BN21]

Karl Bringmann and André Nusser. Translating Hausdorff is hard: fine-grained lower
bounds for Hausdorff distance under translation. J. Comput. Geom., 13(2), 2021.
doi:10.20382/JOCG.V13I2A3.

[BS14]

Sam Buss and Michael Soltys. Unshuffling a square is NP-hard. J. Comput. Syst.
Sci., 80(4), 2014. doi:10.1016/J.JCSS.2013.11.002.

[CDPP23]

Nicola Cotumaccio, Giovanna D’Agostino, Alberto Policriti, and Nicola Prezza. Colexicographically ordering automata and regular languages - Part I. J. ACM, 70(4),
2023. doi:10.1145/3607471.
30

<!-- page 31 -->

[CHL07]

Maxime Crochemore, Christophe Hancart, and Thierry Lecroq.
strings. Cambridge University Press, 2007.

Algorithms on

[CMT03]

Maxime Crochemore, Borivoj Melichar, and Zdenek Tronı́cek. Directed acyclic subsequence graph — overview. J. Discrete Algorithms, 1(3-4), 2003. doi:10.1016/
S1570-8667(03)00029-7.

[DFK+ 25]

Joel D. Day, Pamela Fleischmann, Maria Kosche, Tore Koß, Florin Manea, and Stefan
Siemer. The edit distance to k-subsequence universality. J. Comput. Syst. Sci., 154,
2025. doi:10.1016/J.JCSS.2025.103681.

[FGK+ 15]

Dominik D. Freydenberger, Pawel Gawrychowski, Juhani Karhumäki, Florin Manea,
and Wojciech Rytter. Testing k-binomial equivalence. In Multidisciplinary Creativity,
a collection of papers dedicated to G. Păun 65th birthday, 2015. Available in CoRR
abs/1509.00622.

[FK23]

André Frochaux and Sarah Kleest-Meißner. Puzzling over subsequence-query extensions: Disjunction and generalised gaps. In AMW, 2023. URL: https://ceur-ws.
org/Vol-3409/paper3.pdf.

[FKM+ 24]

Szilárd Zsolt Fazekas, Tore Koß, Florin Manea, Robert Mercas, and Timo Specht.
Subsequence matching and analysis problems for formal languages. In ISAAC, volume
322, 2024. doi:10.4230/LIPICS.ISAAC.2024.28.

[FKRV15]

Ronald Fagin, Benny Kimelfeld, Frederick Reiss, and Stijn Vansummeren. Document
spanners: A formal approach to information extraction. J. ACM, 62(2), 2015. doi:
10.1145/2699442.

[Fri06]

Jeffrey E. F. Friedl. Mastering regular expressions - Understand your data and be
more productive: for Perl, PHP, Java, .NET, Ruby, and more. O’Reilly, 2006. URL:
http://www.oreilly.de/catalog/regex3/index.html.

[FW93]

Michael L. Fredman and Dan E. Willard. Surpassing the information theoretic bound
with fusion trees. J. Comput. Syst. Sci., 47(3), 1993. doi:10.1016/0022-0000(93)
90040-4.

[GAA+ 20]

Nikos Giatrakos, Elias Alevizos, Alexander Artikis, Antonios Deligiannakis, and Minos N. Garofalakis. Complex event recognition in the big data era: A survey. VLDB
J., 29(1), 2020. doi:10.1007/S00778-019-00557-W.

[GJ23]

Étienne Grandjean and Louis Jachiet. Which arithmetic operations can be performed in constant time in the RAM model with addition?, 2023. Preprint:
arXiv:2206.13851.

[GKK+ 21]

Pawel Gawrychowski, Maria Kosche, Tore Koß, Florin Manea, and Stefan Siemer.
Efficiently testing Simon’s congruence. In STACS, volume 187, 2021. doi:10.4230/
LIPICS.STACS.2021.34.

[GLHK+ 20] Jean Goubault-Larrecq, Simon Halfon, Prateek Karandikar, K Narayan Kumar, and
Philippe Schnoebelen. The ideal approach to computing closed subsets in well-quasiorderings. Well-Quasi Orders in Computation, Logic, Language and Reasoning: A
Unifying Concept of Proof Theory, Automata Theory, Formal Languages and Descriptive Set Theory, 2020. doi:10.1007/978-3-030-30229-0_3.
[GRUV21]

Alejandro Grez, Cristian Riveros, Martı́n Ugarte, and Stijn Vansummeren. A formal
framework for complex event recognition. ACM Trans. Database Syst., 46(4), 2021.
doi:10.1145/3485463.

31

<!-- page 32 -->

[GSZ24]

Moses Ganardi, Irmak Sağlam, and Georg Zetzsche. Directed regular and context-free
languages. In STACS, 2024. doi:10.4230/LIPICS.STACS.2024.36.

[Hir77]

Daniel S. Hirschberg. Algorithms for the longest common subsequence problem. J.
ACM, 24(4), 1977. doi:10.1145/322033.322044.

[HMU01]

John E. Hopcroft, Rajeev Motwani, and Jeffrey D. Ullman. Introduction to Automata
Theory, Languages, and Computation. Addison-Wesley, 2001.

[HS77]

James W. Hunt and Thomas G. Szymanski. A fast algorithm for computing longest
subsequences. Communications of the ACM, 20(5), 1977. doi:10.1145/359581.
359603.

[HSZ17]

Simon Halfon, Philippe Schnoebelen, and Georg Zetzsche. Decidability, complexity,
and expressiveness of first-order logic over the subword ordering. In LICS, 2017.
doi:10.1109/LICS.2017.8005141.

[KCTV07]

Sailesh Kumar, Balakrishnan Chandrasekaran, Jonathan S. Turner, and George
Varghese. Curing regular expressions matching algorithms from insomnia, amnesia,
and acalculia. In ANCS, 2007. doi:10.1145/1323548.1323574.

[KKMS22]

Maria Kosche, Tore Koß, Florin Manea, and Stefan Siemer. Combinatorial algorithms
for subsequence matching: A survey. In NCMA, volume 367 of EPTCS, 2022. doi:
10.4204/EPTCS.367.2.

[Kle56]

S.C. Kleene. Representation of events in nerve nets and finite automata. In Automata
Studies, volume 34 of Annals of Mathematics Studies. Princeton University Press,
1956.

[KS19]

Prateek Karandikar and Philippe Schnoebelen. The height of piecewise-testable languages and the complexity of the logic of subwords. Log. Methods Comput. Sci.,
15(2), 2019. doi:10.23638/LMCS-15(2:6)2019.

[KSS+ 22]

Sarah Kleest-Meißner, Rebecca Sattler, Markus L. Schmid, Nicole Schweikardt, and
Matthias Weidlich. Discovering event queries from traces: Laying foundations for
subsequence-queries with wildcards and gap-size constraints. In ICDT, 2022. doi:
10.4230/LIPICS.ICDT.2022.18.

[KSS+ 23]

Sarah Kleest-Meißner, Rebecca Sattler, Markus L. Schmid, Nicole Schweikardt, and
Matthias Weidlich. Discovering multi-dimensional subsequence queries from traces from theory to practice. In BTW, 2023. doi:10.18420/BTW2023-24.

[Kus20]

Dietrich Kuske. The subtrace order and counting first-order logic. In CSR, volume
12159 of LNCS, 2020. doi:10.1007/978-3-030-50026-9\_21.

[KZ19]

Dietrich Kuske and Georg Zetzsche. Languages ordered by the subword order. In
FOSSACS, volume 11425 of LNCS, 2019. doi:10.1007/978-3-030-17127-8\_20.

[LN19]

Alex X. Liu and Eric Norige. A de-compositional approach to regular expression
matching for network security. IEEE/ACM Trans. Netw., 27(6), 2019. doi:10.
1109/TNET.2019.2941920.

[Mai78]

David Maier. The complexity of some problems on subsequences and supersequences.
J. ACM, 25(2), 1978. doi:10.1145/322063.322075.

[MSY04]

Alexandru Mateescu, Arto Salomaa, and Sheng Yu. Subword histories and Parikh
matrices. J. Comput. Syst. Sci., 68(1), 2004. doi:10.1016/J.JCSS.2003.04.001.

[Okh10]

Alexander Okhotin. On the state complexity of scattered substrings and superstrings.
Fundamenta Informaticae, 99(3), 2010. doi:10.3233/FI-2010-252.
32

<!-- page 33 -->

[PSVV24]

M. Praveen, Philippe Schnoebelen, Julien Veron, and Isa Vialard. On the piecewise complexity of words and periodic words. In SOFSEM, 2024. doi:10.1007/
978-3-031-52113-3\_32.

[PW25]

Steven Purtzel and Matthias Weidlich. Suse: Summary selection for regular expression subsequence aggregation over streams. Proc. ACM Manag. Data, 3(3), 2025.
doi:10.1145/3725359.

[Rid79]

William E. Riddle. An approach to software system modelling and analysis. Comput.
Lang., 4(1), 1979. doi:10.1016/0096-0551(79)90009-2.

[RS15]

Michel Rigo and Pavel Salimov. Another generalization of abelian equivalence:
Binomial complexity of infinite words. Theor. Comput. Sci., 601, 2015. doi:
10.1016/J.TCS.2015.07.025.

[Sal05]

Arto Salomaa. Connections between subwords and certain matrix mappings. Theoret.
Comput. Sci., 340(2), 2005. doi:10.1016/J.TCS.2005.03.024.

[Sek12]

Shinnosuke Seki. Absoluteness of subword inequality is undecidable. Theor. Comput.
Sci., 418, 2012. doi:10.1016/J.TCS.2011.10.017.

[Sha78]

Alan C. Shaw. Software descriptions with flow expressions. IEEE Trans. Software
Eng., 4(3), 1978. doi:10.1109/TSE.1978.231501.

[Sim72]

Imre Simon. Hierarchies of events with dot-depth one. PhD thesis, University of
Waterloo, 1972.

[Sim75]

Imre Simon. Piecewise testable events. In Automata Theory and Formal Languages,
2nd GI Conference, volume 33 of LNCS, 1975. doi:10.1007/3-540-07407-4\_23.

[Sim03]

Imre Simon. Words distinguished by their subwords (Extended abstract). In WORDS,
volume 27 of TUCS General Publication, 2003.

[SK15]

Peter Snyder and Chris Kanich. No please, after you: Detecting fraud in affiliate marketing networks. In WEIS, 2015. URL: http://www.econinfosec.org/archive/
weis2015/papers/WEIS_2015_snyder.pdf.

[SV23]

Philippe Schnoebelen and Julien Veron. On arch factorization and subword universality for words and compressed words. In WORDS, 2023. doi:10.1007/
978-3-031-33180-0\_21.

[Tho68]

K. Thompson. Programming techniques: Regular expression search algorithm. Communications of the ACM, 11, 1968. doi:10.1145/363347.363387.

[Wil05]

Ryan Williams. A new algorithm for optimal 2-constraint satisfaction and its implications. Theor. Comput. Sci., 348(2-3), 2005. doi:10.1016/j.tcs.2005.09.023.

[ZDI14]

Haopeng Zhang, Yanlei Diao, and Neil Immerman. On complexity and optimization
of expensive queries in complex event processing. In SIGMOD, 2014. doi:10.1145/
2588555.2593671.

[Zet16]

Georg Zetzsche. The complexity of downward closure comparisons. In ICALP, volume 55, 2016. doi:10.4230/LIPICS.ICALP.2016.123.

This work is licensed under a Creative Commons “Attribution 4.0 International” license.

33
