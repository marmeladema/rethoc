---
file: ../pdfs/2016-focs-backurs-hard-regex-patterns.pdf
citation: "Which Regular Expression Patterns are Hard to Match? — Backurs, Indyk — FOCS 2016"
doi: 
source: "https://arxiv.org/pdf/1511.07070"
topics: [counting_and_derivatives, semantics_theory_access]
pages: 33
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Which Regular Expression Patterns are Hard to Match?

arXiv:1511.07070v2 [cs.CC] 26 Sep 2016

Arturs Backurs∗
MIT

Piotr Indyk†
MIT

Abstract
Regular expressions constitute a fundamental notion in formal language theory and are
frequently used in computer science to define search patterns. In particular, regular expression
matching and membership testing are widely used computational primitives, employed in many
programming languages and text processing utilities. A classic algorithm for these problems
constructs and simulates a non-deterministic finite automaton corresponding to the expression,
resulting in an O(mn) running time (where m is the length of the pattern and n is the length
of the text). This running time can be improved slightly (by a polylogarithmic factor), but
no significantly faster solutions are known. At the same time, much faster algorithms exist for
various special cases of regular expressions, including dictionary matching, wildcard matching,
subset matching, word break problem etc.
In this paper, we show that the complexity of regular expression matching can be characterized based on its depth (when interpreted as a formula). Our results hold for expressions
involving concatenation, OR, Kleene star and Kleene plus. For regular expressions of depth two
(involving any combination of the above operators), we show the following dichotomy: matching and membership testing can be solved in near-linear time, except for “concatenations of
stars”, which cannot be solved in strongly sub-quadratic time assuming the Strong Exponential
Time Hypothesis (SETH). For regular expressions of depth three the picture is more complex.
Nevertheless, we show that all problems can either be solved in strongly sub-quadratic time, or
cannot be solved in strongly sub-quadratic time assuming SETH.
An intriguing special case of membership testing involves regular expressions of the form “a
star of an OR of concatenations”, e.g., [a|ab|bc]∗ . This corresponds to the so-called word
√ break
problem, for which a dynamic programming algorithm with a runtime of (roughly) O(n m) is
known. We show that the latter bound is not tight and improve the runtime to O(nm0.44... ).

∗
†

backurs@mit.edu
indyk@mit.edu

<!-- page 2 -->

1

Introduction

A regular expression (regexp) is a formula that describes a set of words over some alphabet Σ. It
consists of individual symbols from Σ, as well as operators such as OR “|” (an alternative between
several pattern arguments), Kleene star “∗” (which allows 0 or more repetitions of the pattern
argument), Kleene plus “+” (which allows 1 or more repetitions of the pattern argument), wildcard
“.” (which matches an arbitrary symbol), etc. For example, [a|b]+ describes any sequence of symbols
a and b of length at least 1. See Preliminaries for the formal definition.
In addition to being a fundamental notion in formal language theory, regular expressions are
widely used in computer science to define search patterns. Formally, given a regular expression
(pattern) p of size m and a sequence of symbols (text) t of length n, the goal of regular expression
matching is to check whether a substring of t can be derived from p. A closely related problem
is that of membership testing where the goal is to check whether the text t itself can be derived from p. Regular expression matching and membership testing are widely used computational
primitives, employed in several programming languages and text processing utilities such as Perl,
Python, JavaScript, Ruby, AWK, Tcl and Google RE2. Apart from text processing and programming languages, regular expressions are used in computer networks [KDY+ 06], databases and data
mining [GRS99], computational biology [NR03], human-computer interaction [KHDA12] etc.
A classic algorithm for both problems constructs and simulates a non-deterministic finite automaton corresponding to the expression, resulting in the “rectangular” O(mn) running time. A
sequence of improvements, first by Myers [Mye92] and then by [BT09], led to an algorithm that
achieves roughly O(mn/ log1.5 n) running time. The latter result constitutes the fastest algorithm
for this problem known to date, despite an extensive amount of research devoted to this topic. The
existence of faster algorithms is a well-known open problem ([Gal85], Problem 4).
However, significantly faster algorithms are known for various well-studied special cases of
regular expressions. For example:
1. If the pattern is a concatenation of symbols (i.e., we search for a specific sequence of symbols in
the text), the pattern matching problem corresponds to the “standard” string matching problem and can be solved in linear time, e.g., using the Knuth-Morris-Pratt algorithm [KMP77].
2. If the pattern is of the form p1 |p2 | . . . |pk where pi are sequences of symbols, then the pattern
matching problem corresponds to the dictionary matching problem that can be solved in
linear time using the Aho-Corasick algorithm [AC75].
3. If the pattern is a concatenation of symbols and single character wildcards “.” matching any
symbol, the pattern matching problem is known as the wildcard matching and can solved in
(deterministic) O(n log m) time using convolutions [FP74, Ind98, Kal02, CH02].
4. More generally, if the pattern is a concatenation of single character ORs of the form s1 |s2 | . . . |sk
for s1 , . . . , sk ∈ Σ (e.g., [a|b][a|c][b|c]), the pattern matching problem is known as superset
matching and can be solved in (deterministic) O(n log2 m) time [CH97, CH02].
5. Finally, if the goal is to test whether a text t can be derived from a pattern p of the form
(p1 |p2 | . . . |pk )+ where pi are sequences of symbols, the problem is known as the word break
problem. It is a popular interview question [Tun11, Lee], and the known solutions can be
√
implemented to run in mn logO(1) n time.

1

<!-- page 3 -->

The first two examples were already mentioned in [Gal85] as a possible reason why a faster
algorithm for the general problem might be possible. Despite the existence of such examples, any
super-polylogarithmic improvements to the algorithms of [Mye92, BT09] in the general case remain
elusive. Furthermore, we are not aware of any systematic classification of regular expressions into
“easy” and “hard” cases for the pattern matching and membership testing problems. The goal of
this paper is to address this gap.
Results Our main result is a classification of the computational complexity of regular expression
matching and membership checking for patterns that involve operators “|”, “+”, “∗” and concatenation “·”, based on the pattern depth. Our classification enables us to distinguish between the
cases that are solvable in sub-quadratic time (including the five problems listed above) and the
cases that do not have strongly sub-quadratic time algorithms under natural complexity theoretic
assumptions, such as Strong Exponential Time Hypothesis [IP01] and Orthogonal Vectors conjecture [Wil05, BK15]. Our results therefore demonstrate a non-trivial dichotomy for the complexity
of these problems.
To formulate our results, we consider pattern formulas that are homogeneous, i.e., in which the
operators at the same level of the formula are equal (note that the five aforementioned problems
involve patterns that satisfy this condition). We say that a homogeneous formula of depth k has
type o1 o2 . . . ok if for all levels i the operators at level i are equal to oi (note that, in addition to
the operators, a level might also contain leaves, i.e. symbols; for example the expression [a|b]a[b|c]
is a depth-2 formula of type “·|”). We will assume that no two consecutive operators in the type
descriptor are equal, as otherwise they can be collapsed into one operator.
Our results are described in Table 1 (for depth-2 expressions) and Table 2 (for depth-3 expressions). The main findings can be summarized as follows:
1. Almost all pattern matching and membership problems involving depth-2 expressions can be
solved in near-linear time. The lone exception involve patterns of type “·∗”, for which we
show that matching and membership problems cannot be solved in time O((mn)1−δ ) for any
constant δ > 0 and m ≤ n assuming the Strong Exponential Time Hypothesis (SETH) [IP01].
Interestingly, we show that pattern matching with a very similar depth-2 type, namely “·+”,
can be solved in O(n log2 m) time.
2. Pattern matching problems with depth-2 expressions contain a “high density” of interesting algorithmic problems, with non-trivial algorithms existing for types “·+” (this paper),
“·|” [CH02] , “|·” [AC75] and “+·” (essentially solved in [KMP77], since + can be dropped).
In contrast, membership problems with depth-2 expressions have a very restrictive structure
that makes them mostly trivially solvable in linear time, with the aforementioned exception
for the “·∗” type
3. Pattern matching problems with depth-3 expressions have a more diversified structure. All
types starting with · are SETH-hard; all types starting with | are either-SETH hard (if
followed by ·) or easily solvable in linear time; all types starting from ∗ are trivially solvable in
linear time (since ∗ allows zero repetitions); all types starting from + inherit their complexity
from the last two operators in the type description (since + allows exactly one repetition).
4. Finally, membership checking with depth-3 expressions presents the most complex picture.
As before, all types starting with · are SETH-hard. On the other hand, types starting with
2

<!-- page 4 -->

Type
·+
·∗
·|
|·
|∗
|+
∗·
∗+
∗|
+·
+|
+∗

Example
a+ ab+
a∗ ab∗
[a|b][b|c]
ab|c
a∗ |a|b∗
a+ |a|b+
[ab]∗
[a+ ]∗
[a|b]∗
[ab]+
[a|b]+
[a∗ ]+

Pattern matching
O(n log2 m) (Section 5.2)
Ω((mn)1−α ) (Section 3.3)
O(n log2 m) (superset matching [CH02])
O(n + m) (dictionary matching [AC75])
O(n + m) (immediate)
O(n + m) (immediate - reducible to “|”)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (string matching [KMP77])
O(n + m) (immediate - reducible to “|”)
O(n + m) (immediate)

Membership
O(n + m) (immediate)
Ω((mn)1−α ) (Section 4.1)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate - reducible to “+”)
O(n + m) (immediate)
O(n + m) (immediate - equivalent to “∗·”)
O(n + m) (immediate - equivalent to “∗|”)
O(n + m) (immediate - reducible to “+”)

Table 1: Classification of the complexity of the pattern matching and the membership test problems for depth-2
expressions. All lower bounds assume SETH and m ≤ n. Some upper bounds use randomization (notably hashing).
For an explanation of reducibility see “Our techniques”.

| (except for |·∗) have linear time algorithms, with difficulty levels that range from trivial
observations to undergraduate-level exercises1 . Types starting with ∗ or + include “∗|·” and
“+|·”, which correspond to the aforementioned word break problem [Tun11, Lee]. This is the
only problem in the table whose (conditional) complexity is not determined up to logarithmic
factors. However, we show that the running time of the standard dynamic-programming
based algorithm can be improved, from roughly nm0.5 to roughly nm0.5 − 1/18 .
Our techniques Our upper bounds for depth-2 expressions follow either from known near-linear
time algorithms for specific variants of regular expressions, or relatively simple constructions of
such algorithms. In particular, we observe that type “·|” expressions (concatenations of ORs)
correspond to superset matching, type “|·” expressions (OR of sequences) correspond to dictionary
matching and type “+·” reduces to “·” and thus corresponds to the standard pattern matching
problem. Furthermore, we give a near-linear time algorithm for pattern matching with type “·+”
expressions, where patterns are concatenations of expressions of the form s≥k or sk , where s≥k
denotes a sequence of symbols s repeated at least k ≥ 1 times.2 We show that this problem can
be solved in near-linear time by reducing it to one instance of subset matching and one instance
of wildcard matching. All other problems can be solved in linear time, with the exception of type
“·∗”. The latter expressions correspond to patterns obtained by concatenating patterns of the form
s≥k and sk . Unlike in the “·+” case, however, here we cannot assume that k ≥ 1, since each symbol
could be repeated zero times. We show that this simple change makes the problem SETH-hard.
This is accomplished by a reduction from an intermediate problem, namely the (unbalanced version
of the) Orthogonal Vectors Problem (OVP) [Wil05, BK15]. The problem is defined as follows: given
two sets A, B ⊆ {0, 1}d such that |A| = M
Pand |B| = N , determine whether there exists x ∈ A and
y ∈ B such that the dot product x · y = dj=1 xj yj is equal to 0.3
1

Incidentally, the second author used some of the problems from Table 2 in an undergraduate algorithms course.
For example, the expression aa+ bc+ generates all words of the form a≥2 b1 c≥1 .
3
The reduction is somewhat complex, so we will not outline it here. However, we give an overview of other
reductions from OVP in the next few paragraphs.
2

3

<!-- page 5 -->

Type
·|·
·|∗
·|+
·+·
·+|
·+∗
·∗·
·∗|
·∗+
|·|
|·∗
|·+
|∗·
|∗|
|∗+
|+·
|+|
|+∗
∗·|
∗·∗
∗·+
∗|·
∗|∗
∗|+
∗+·
∗+|
∗+∗
+·|
+·∗
+·+
+|·
+|∗
+|+
+∗·
+∗|
+∗+

Example
[a|bb][ba|b]
[a∗ |b∗ ][c∗ |b]
[a+ |b+ ][c+ |b]
[ab]+ [bca]+
[a|b]+ [a|c|d]+
[a][a+ ]∗ [b+ ]
[ab]∗ [bca]∗
[a|b]∗ [a|b|c]∗
[a∗ ]b[b+ ]∗
[(a|b)(b|c)]|[(a|c)b]
[a∗ b∗ ]|[b∗ c∗ ]
[a+ b+ ]|[b+ c+ ]
[abc]∗ |[bc]∗
[a|b|c]∗ |[b|c]∗
[a+ ]∗ |[b+ ]∗
[abc]+ |[bc]+
[a|b|c]+ |[b|c]+
[a∗ ]+ |[b∗ ]+
[[a|b][b|c]]∗
[a∗ b∗ c∗ ]∗
[a+ b+ c+ ]∗
[a|ab|bc]∗
[a∗ |b∗ |c∗ ]∗
[a+ |b+ |c+ ]∗
[[abcd]+ ]∗
[[a|b|c|d]+ ]∗
[[a∗ ]+ ]∗
[[a|b][b|c]]+
[a∗ b∗ c∗ ]+
[a+ b+ c+ ]+
[a|ab|bc]+
[a∗ |b∗ |c∗ ]+
[a+ |b+ |c+ ]+
[[abcd]∗ ]+
[[a|b|c|d]∗ ]+
[[a+ ]∗ ]+

Pattern matching
Ω((mn)1−α ) (Section 3.5)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (Section 3.7)
Ω((mn)1−α ) (Section 3.4)
Ω((mn)1−α ) (Section 3.6)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (Section 3.1)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (Section 3.2)
O(n + m) (immediate - reducible to “|”)
O(n + m) (immediate - reducible to “|”)
O(n + m) (immediate - reducible to “|”)
O(n + m) (reducible to “|·”)
O(n + m) (immediate - reducible to “|”)
O(n + m) (immediate - reducible to “|”)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n log2 m) (reducible to “·|”)
Ω((mn)1−α ) (from “·∗”)
O(n log2 m) (reducible to “·+”)
O(n + m) (reducible to “|·”)
O(n + m) (reducible to “|∗”)
O(n + m) (reducible to “|+”)
O(n + m) (reducible to “∗·”)
O(n + m) (reducible to “∗|”)
O(n + m) (reducible to “∗+”)

Membership
Ω((mn)1−α ) (Section 4.3)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (Section 4.5)
Ω((mn)1−α ) (Section 4.2)
Ω((mn)1−α ) (Section 4.4)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (from “·∗”)
Ω((mn)1−α ) (from “·∗”)
O(n + m) (immediate)
Ω((mn)1−α ) (from “·∗”)
O(n + m) (Section 5.4)
O(n + m) (Section 5.3)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (Section 5.3)
O(n + m) (same as “|∗|”)
O(n + m) (immediate)
O(n + m) (immediate)
Ω((mn)1−α ) (from “·∗”)
O(n + m) (Section 5.4)
O(nm0.44... ) (word break - Section 5.1)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (immediate)
O(n + m) (same as “∗·|”)
Ω((mn)1−α ) (same as “∗·∗”)
O(n + m) (same as “∗·+”)
O(nm0.44... ) (word break - Section 5.1)
O(n + m) (same as “∗|∗”)
O(n + m) (same as “∗|+”)
O(n + m) (same as “∗·”)
O(n + m) (same as “∗|”)
O(n + m) (same as “∗+”)

Table 2: Classification of the complexity of the pattern matching and the membership test problems for depth-3
expressions. See Figure 1 for the visualization of the table.

4

<!-- page 6 -->

Pattern matching:

·
|
∗

Membership:

hard

hard

·

hard

·

∗, +

easy

|
∗

easy

+

∗

·
|, ∗

|, +

+

hard
easy

·

|, +

easy

∗

|

|, +

·

∗, +

+

easy

·

|, +

∗

easy

hard

easy

∗, +

·

∗

∗

|

·

∗, +

hard
easy
Word Break
easy

hard
easy
Word Break
easy

easy

Figure 1: Tree diagrams visualizing Table 2. Depth 3 types are classified as “easy” (near-linear time), “hard” (nearquadratic time, assuming SETH), or “Word Break” (whose complexity is not determined). The leftmost operators
in each tree correspond to the leftmost operators in type descriptions.

Our results for depth-3 expression pattern matching are multi-fold. First, all types starting from
∗ are trivially solvable in linear time, since ∗ allows zero repetitions. Second, all types starting from
+ inherit their complexity from the last two operators in the type description, since + allows exactly
one repetition. Third, all types starting from |∗ or |+ have simple linear time solutions.
The remaining cases lead to SETH-hard problems. For six types this follows immediately from
the analogous result for type “·∗”. For the six remaining types the hardness is shown via individual
reductions from OVP. For some types, such a reduction is immediate. For example, for type “|·|”
expressions (ORs of concatenations of ORs), we form the text by concatenating all vectors in B
(separated by some special symbol), and we form the pattern by taking an OR of the vectors
in A, modified by replacing 0 with [0|1] and 1 with 0. A similar approach works for type “|·+”
expressions.
The remaining four types are grouped into two classes: “·+·” is grouped with “·|·” and “·+|”
is grouped with “·|+”. For each group, we first show hardness of the first type in the group (i.e.,
of “·+·” and “·+|”, respectively). We then show that the second type in each group is hard by
making changes to the hardness proof for the first type.
The hardness proof for “·+·” proceeds as follows. We form the pattern p by concatenating
(appropriately separated) pattern vector gadgets for each vector in A, and form the text t by
concatenating (appropriately separated) text vector gadgets for each vector in B. We then show
that if there is a pair of orthogonal vectors ai ∈ A, bj ∈ B then p can be matched to a substring of
t, and vice versa. To show this, we construct p and t so that any pair of gadgets (in particular the
gadgets for ai and bj ) can be aligned. We then show that (i) each vector gadget for a vector in A
can be matched with “most” of the gadget for the corresponding b ∈ B (ii) matching the gadgets
for orthogonal vectors ai and bj allows us to make a “smaller step”, i.e., to match the gadget for
ai with a smaller part of the gadget for bj , and (iii) at least one “smaller step” is necessary to
completely derive a substring of t from p. We then conclude that there is a pair of orthogonal
vectors ai ∈ A, bj ∈ B if and only if p can be matched to a substring of t. The hardness proof
5

<!-- page 7 -->

for “·+|” follows a similar general approach, although the technical development is different. In
particular, we construct the gadgets such that the existence of orthogonal vectors makes it possible
to make a “bigger step”, i.e., to derive a bigger part of t, and that one bigger step is necessary to
complete the derivation.
To show hardness of the second type in each group, we adapt the arguments for the first type in
the group. In particular, to show hardness for type “·|·” , we construct p and t as in the reduction
for type “·+·” and then transform p into a type “·|·” regular expression p′ . The transformation has
the property that p′ is less expressive than p (i.e., the language corresponding to p is a superset of
the language corresponding to p′ ), but the specific substrings of the text t needed for the reduction
can be still derived from p′ . The hardness proof for “·|+” is obtained via a similar transformation
of the hardness proof for “·+|”.
Finally, consider the membership checking problem for depth-3 expressions. As before, all types
starting with · are shown to be SETH-hard. The reductions are similar to those for the pattern
matching problem, but in a few cases require some modifications. On the other hand, types starting
with | (with the exception of |·∗) have linear time algorithms. The algorithms are not difficult,
but require the use of basic algorithmic notions, such as periodicity (for types “|∗·” and “|+·”)
and run-length encoding (for type “|·+”). Types starting with ∗ are mostly solvable in linear time,
with two exceptions: type “∗·∗” inherits the hardness from “·∗”4 , while the type “∗|·” corresponds
to the aforementioned word break problem which we discuss in the next paragraph. Finally, types
starting with + are analogous to those starting with ∗.
The word break problem is the only problem in the table whose (conditional) complexity is not
determined up to logarithmic factors. There are several known solutions to this problem based on
dynamic programming [Tun11, Lee]. A careful implementation of those algorithms (using substring
hashing and pruning) leads to a runtime of O(nm0.5 logO(1) n). However, we show that this bound
is not tight, and can be further improved to roughly O(nm0.5 − 1/18 ). Our new algorithm speeds
up the dynamic program by using convolutions to pre-compute information that is reused multiple
times during the execution of the algorithm. We note that the algorithm is randomized and has a
one-sided error.
Related work Our hardness results come on the heels of several recent works demonstrating
quadratic hardness of sequence alignment problems assuming SETH or other conjectures. In particular, such results have been shown for Local Alignment [AWW14], Fréchet distance [Bri14], Edit
Distance [BI15] and Longest Common Subsequence [ABVW15, BK15]. As in our case, most of
those results were achieved by a reduction from OVP, performed by concatenating appropriately
constructed gadgets for the vectors in the input sets. The technical development in our paper is,
however, quite different, since regular expression matching is not defined by a sequence similarity
measure. Instead, our gadget constructions are tailored to the specific sets of operators and expression types defining the problem variants. Furthermore, we exploit the similarity between related
expression types (such as “·+·” and “·|·”) and show how to convert a hardness proof for one type
into a hardness proof for the other type.
The reduction in Section 3.1 has been independently discovered by Kasper Larsen and Raphael
Let a regular expression p and a text t be a hard instance for the “·∗” membership problem. Let x be a symbol
that does not appear in p or t. Then p′ := (x·p·x)∗ and t := xtx is a hard instance for the membership problem
of type “∗·∗”. Since t′ starts and ends with a unique symbol, we must use the argument regular expression x·p·x
exactly once. Thus we get “·∗” membership problem.
4

6

<!-- page 8 -->

Clifford (personal communication). Conditional lower bounds (via reductions from 3SUM) for certain classes of regular expressions have been investigated in [AKL+ 15]. Estimating the complexity
of regular expression matching using specific algorithms has also been a focus of several papers.
See e.g., [WvdMBW16] and the references therein.

2

Preliminaries

Orthogonal Vectors problem Our reductions use the unbalanced version of the Orthogonal
Vectors Problem, defined as follows: given two sets A, B ⊆ {0, 1}d such that |A|P= M, |B| = N ,
determine whether there exists x ∈ A and y ∈ B such that the dot product x·y = dj=1 xj yj (taken
over the reals) is equal to 0. An equivalent formulation of this problem is: given two collections of
subsets of {1, . . . , d}, of sizes M and N , respectively, determine if there is a set in the first collection
that is contained in a set from the second collection.
It is known that, for any M = Θ(N α ) for some α ∈ (0, 1] and any constant δ > 0, any algorithm
for OVP problem with an O(M N )1−δ running time would also yield a more efficient algorithm for
SAT, violating the Strong Exponential Time Hypothesis, even in the setting when the dimension d
is arbitrary d = ω(log N ). This was shown in [Wil05] for the balanced case M = N , and extended
to the unbalanced case in [BK15]. Therefore, in this paper we show that a problem is SETH-hard
by reducing unbalanced OVP to it.
Subset Matching problem In the subset matching problem, we are given a pattern string p
and a text string t where each pattern and text location is a set of symbols drawn from some
alphabet. The pattern is said to occur at the text position i if the set pj is a subset of the set ti+j
for all j. The goal of the problem is find all positionsPwhere pPoccurs in t. The problem can be
solved in deterministic O(N log2 N ) time, where N = i |ti | + i |pi | [CH02].
Superset Matching problem This problem is analogous to subset matching except that we
require that pj is a superset of the set ti+j for all j. The aforementioned algorithm of [CH02]
applies to this problem as well.

Wildcard Matching problem In the wildcard matching problem, we are given a pattern string
p and a text string t where each pattern and text location is an element from Σ∪{.}, where “.” is the
special wildcard symbol. The pattern is said to occur at the text position i if for all j we have that
(i) one of the symbols pj and ti+j is equal to “.”, or (ii) pj = ti+j . The goal of the problem is find
all positions where p occurs in t. The problem can be solved in deterministic O(n log n) time [CH02].
Regular expressions Regular expressions over a symbol set Σ and an operator set O = {·, |, +, ∗}
are defined recursively as follows:
• a is a regular expression, for any a ∈ Σ;

• if R and S are regular expressions then so are [R]|[S], [R]·[S], [R]+ and [R]∗ .

For the sake of simplicity, in the rest of this paper we will typically omit the concatenation
operator ·, and also omit some of the parenthesis if the expression is clear from the context.
7

<!-- page 9 -->

A regular expression p determines a language L(p) over Σ. Specifically, for any regular expressions R, S and any a ∈ Σ, we have: L(a) = {a}; L(R|S) = L(R) ∪ L(S); L(R·S) = {uv : u ∈
L(R), v ∈ L(S)}; L(R+ ) = ∪i≥1 ·ij=1 L(R); and L(R∗ ) = L(R+ ) ∪ {ǫ}, where ǫ denotes the empty
word.
Regular expressions can be viewed as rooted labeled trees, with internal nodes labeled with
operators from O and leaves labeled with symbols from Σ. Note that the number of children of an
internal node is not fixed, and can range between 1 (for + and ∗) and m. We say that a regular
expression is homogeneous if all internal node labels at the same tree level are equal. Note that this
definition does not preclude expressions such as aa+ where not all leaves have the same depth. A
homogeneous formula of depth k has type o1 o2 . . . ok , oi ∈ O, if for all levels i the operators at level
i are equal to oi . For example, aa+ has type “·+”.
To state our results, it is convenient to identify depth-3 homogeneous regular expressions that
are equivalent to some depth-2 regular expressions. In particular, in all types starting from +, the
operator + can be removed, since p+ occurs in the text t if and only if p occurs in t. Similarly, in
all types starting from |+, the operator + can be removed, for the same reasons.
Notation For symbol (or sequence) z and integer i, z i denotes symbol (or sequence) z repeated
i times. For an integer n, [n] denotes {1, 2, 3, . . . n}. Given an integer d, 0d (1d , resp.) denotes the
vector with all entries equal to 0 (1, resp.) in d dimensions. For sequences s1 , s2 , . . . , sk , we write
J
k
i=1 si to denote the concatenation s1 s2 . . . sk of the sequences.

Simplifying assumptions To simplify our proofs, we will make several simplifying assumptions
about the Orthogonal Vectors instance (different assumptions are used in different proofs). In what
follows, we describe what kind of assumptions we make, and how to satisfy them:
1. The number of vectors M in set A is odd or even (depending on the proof): this can be
achieved w.l.o.g. since we can add a vector to set A consisting only of 1s.
2. The dimensionality d of vectors from sets A and B is odd or even (depending on the proof):
this can be achieved w.l.o.g. since we can add new coordinate to every vector and set this
coordinate to 0.
3. Let A = {a1 , . . . , aM } and B = {b1 , . . . , bN }. If there are i, j such that ai · bj = 0 then there
′
′
are i′ , j ′ such that ai · bj = 0 and i′ ≡ j ′ (mod 2): this can be assumed w.l.o.g. since we can
define a set A′ = {aM , a1 , a2 , a3 , . . . , aM −1 } and perform two reductions, one on the pair of
sets A and B and another one on the pair of sets A′ and B. If a pair of orthogonal vectors
exists, one of these reductions will detect it.
4. The dimensionality d is greater than 100: we can assume this since otherwise Orthogonal
Vectors problem can be solved in O(2d · N ) = O(N ) time.

5. bt1 = btd = 0 for all t ∈ [N ] and d is odd: first, we make d odd as described above. Then we
add two entries for every vector from A or B, one at the beginning and one at the end, and
set both entries to 0.

6. The first vector a1 from the set A is not orthogonal to all vectors from B: first, we can detect
whether this is the case in O(dN ) time. If a1 is orthogonal to a vector from B, we have found
a pair of orthogonal vectors. Otherwise we proceed with the reduction.

8

<!-- page 10 -->

3

Reductions for the Pattern Matching problem

We start this section by showing hardness for regular expressions of type “|·|” and “|·+”. These
hardness proofs are quite simple, and will help us introduce notation used in more complex reductions presented later. Next we present the hardness proof for regular expressions of type “·∗”.
After that we present hardness proofs for the other cases.

3.1

Hardness for type “|·|”

Theorem 1. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d , we can
construct a regular expression p and a sequence of symbols t, in O(N d) time, such that a substring
of t can be derived from p if and only if there are a ∈ A and b ∈ B such that a · b = 0. Furthermore,
p has type “|·|”, |p| ≤ O(M d) and |t| ≤ O(N d).

Proof. First, we will construct our pattern p. For an integer v ∈ {0, 1}, we construct the following
pattern coordinate gadget
(
[0|1] if v = 0;
CG(v) :=
[0]
if v = 1.
For a vector a ∈ {0, 1}d , we define a pattern vector gadget
V G(a) := CG(a1 ) CG(a2 ) CG(a3 ) . . . CG(ad ).
Our pattern p is then defined as “|” of all pattern vector gadgets:






p := V G a1 | V G a2 | V G a3 | V G a4 | . . . | V G aM −1 | V G aM .

Now we construct the text t. First, for a vector b ∈ {0, 1}d , we define text vector gadget as
concatenation of all entries of b: V G′ (b) := b1 b2 b3 b4 . . . bd . Note that we can derive V G′ (b) from
V G(a) if and only if a · b = 0. Our text t is defined as a concatenation of all text vector gadgets
with a symbol 2 in between any two neighbouring vector gadgets:






t := V G′ b1 2 V G′ b2 2 V G′ b3 2 V G′ b4 2 . . . 2 V G′ bN −1 2 V G′ bN .

We need to show that we can derive a substring of t from p if and only if there are two orthogonal
vectors in A and B. This follows from lemmas 1 and 2 below.
Lemma 1. If there are two vectors a ∈ A and b ∈ B that are orthogonal, then a substring of t can
be derived from p.

Proof. Suppose that ai · bj = 0 for some i ∈ [M ], j ∈ [N ]. We choose a pattern vector gadget
V G(ai ) from the pattern p and transform it into a text vector gadget V G′ (bj ). This is possible
because of the orthogonality and the construction of the vector gadgets.
Lemma 2. If a substring of t can be derived from p, then there are two orthogonal vectors.
Proof. By the construction of pattern p, we have to choose one pattern vector gadget, say, V G(ai ),
that is transformed into a binary substring of t of length d. The text t has the property that it
is a concatenation of binary strings of length d separated by symbols 2. This means that V G(ai )
will be transformed into binary string V G′ (bj ) for some j. This implies that ai · bj = 0 by the
construction of the vector gadgets.
9

<!-- page 11 -->

3.2

Hardness for type “|·+”

Theorem 2. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d , we can
construct a regular expression p and a sequence of symbols t, in O(N d) time, such that a substring
of t can be derived from p iff there are a ∈ A and b ∈ B such that a · b = 0. Furthermore, p has
type “|·+”, |p| ≤ O(M d) and |t| ≤ O(N d).

Proof. First, we will construct our pattern. For an integer v ∈ {0, 1}, we construct the following
pattern coordinate gadget
(
x+
if v = 0;
CG(v) :=
+
+
x x
if v = 1.

For a vector a ∈ {0, 1}d , we define a pattern vector gadget as concatenation of coordinate gadgets
for all coordinates with the symbol y in between every two neighbouring coordinate gadgets:
V G(a) := CG(a1 ) y CG(a2 ) y CG(a3 ) y . . . y CG(ad ).
Our pattern p is then defined as an OR (“|”) of all the pattern vector gadgets:






p := V G a1 | V G a2 | V G a3 | V G a4 | . . . | V G aM −1 | V G aM .

Now we proceed with the construction of our text t. For an integer v ∈ {0, 1}, we define the
following text coordinate gadget
(
xx if v = 0;
CG′ (v) :=
x
if v = 1.
For vector b ∈ {0, 1}d , we define the text vector gadget as

V G′ (b) := CG′ (b1 , 1) y CG′ (b2 , 2) y CG′ (b3 , 3) y . . . y CG′ (bd , d).

Note that we can derive V G′ (b) from V G(a) iff a · b = 0. Our text t is defined as a concatenation
of all text vector gadgets with the symbol z in between any two neighbouring vector gadgets:
t := V G′ (b1 ) z V G′ (b2 ) z V G′ (b3 ) z V G′ (b4 ) z . . . z V G′ (bN −1 ) z V G′ (bN ).
We need to show that we can derive a substring of t from p iff there are two orthogonal vectors.
This follows from lemmas 3 and 4 below.
Lemma 3. If there are two vectors a ∈ A and b ∈ B that are orthogonal, then a substring of t can
be derived from p.
Proof. Suppose that ai · bj = 0 for some i ∈ [M ], j ∈ [N ]. We choose a pattern vector gadget
V G(ai ) from the pattern p and transform it into a text vector gadget V G′ (bj ). This is possible
because of the orthogonality and the construction of the vector gadgets.
Lemma 4. If a substring of t can be derived from p, then there are two orthogonal vectors.
Proof. We call a sequence of symbols nice iff it can be derived from the regular expression x+ yx+ yx+ y . . . yx+ ,
where “x+ ” appears d times.
By the construction of pattern p, we have to choose one pattern vector gadget, say, V G(ai ), that
is transformed into a nice sequence. The text t has the property that it is a concatenation of nice
sequences separated by symbols z. This means that V G(ai ) will be transformed into a sequence
V G′ (bj ) for some j. This implies that ai · bj = 0 by the construction of vector gadgets.
10

<!-- page 12 -->

3.3

Hardness for type “·∗”

Theorem 3. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d with M ≤ N ,
we can construct the regular expression p and the text t in time O(N d), such that a substring of t
can be derived from p iff there are a ∈ A and b ∈ B such that a · b = 0. Furthermore, p is of type
“·∗”, |p| ≤ O(M d) and |t| ≤ O(N d).

Proof. W.l.o.g., we can assume that M ≡ 1(mod 2) and d ≡ 1(mod 2), d ≥ 100. Also, if there are
′
′
i ∈ [M ], j ∈ [N ] such that ai · bj = 0, then there are i′ ∈ [M ], j ′ ∈ [N ] such that ai · bj = 0 and
i′ ≡ j ′ (mod 2). Furthermore, we assume bj1 = bjd = 0 for all j ∈ [N ] and that a1 is not orthogonal
to any vector bj .
First, we will construct our pattern. For an integer v ∈ {0, 1} and an integer i ∈ [d], we
construct the following pattern coordinate gadget


yy ∗
if v = 0 and i ≡ 1(mod 2);



yyyy ∗ if v = 1 and i ≡ 1(mod 2);
CG(v, i) :=

xx∗
if v = 0 and i ≡ 0(mod 2);



xxxx∗ if v = 1 and i ≡ 0(mod 2).
For a vector a ∈ {0, 1}d , we define a pattern vector gadget

V G(a) := CG(a1 , 1) CG(a2 , 2) CG(a3 , 3) . . . CG(ad , d).
We also need another pattern vector gadget V G0 := (y ∗ x∗ )d+10 y ∗ .
Our pattern is then defined as follows:
K

x10 V G(aj ) x10 V G0 x10 V G(aM ) x10 y 6 .
p := y 6
j∈[M −1]

Now we proceed with the construction of our text. For integers v ∈ {0, 1}, i ∈ [d], we define the
following text coordinate gadget


yyy if v = 0 and i ≡ 1(mod 2);



y
if v = 1 and i ≡ 1(mod 2);
CG′ (v, i) :=

xxx if v = 0 and i ≡ 0(mod 2);



x
if v = 1 and i ≡ 0(mod 2).

For a vector b ∈ {0, 1}d and an integer j ≡ 1(mod 2), we define the text vector gadget as
V G′ (b, j) := CG′ (b1 , 1) CG′ (b2 , 2) CG′ (b3 , 3) . . . CG′ (bd , d).

We also define V G′ (b, j), when j ≡ 0(mod 2). In this case, V G′ (b, j) is equal to V G′ (b, 1) except
that we replace every occurrence of the substring y 3 with the substring y 6 .
One can verify that for any vectors a, b ∈ {0, 1}d and any integer i, V G′ (b, i) can be derived
from V G(a) iff a · b = 0.
We will also need an additional text vector gadget
V G′0 := y 3 (x3 y 3 )(d−1)/2 .
11

<!-- page 13 -->

Our text is then defined as follows:
t :=

3N
K

j=−2N


x10 V G′0 x10 V G′ (bj , j) ,

where we assume bj := 011111 . . . 111110 for j 6∈ [N ].
We have to show that we can derive a substring of t from p iff there are two orthogonal vectors.
This follows from lemmas 5 and 6 below.
Lemma 5. If there are two vectors a ∈ A and b ∈ B that are orthogonal, then a substring of t can
be derived from p.
Proof. W.l.o.g. we have that, ak · bk = 0 for some k ∈ [M ]. The proof for the case when ak · br = 0,
k ∈ [M ], r ∈ [N ], k ≡ r(mod 2) is analogous.
The pattern p starts with y 6 . We transform it into CG′ (b0d , d) appearing in V G′ (b0 , 0). We can
do this since b0d = 0.
For j = 1, 2, . . . , k − 2, we transform x10 V G(aj ) x10 V G0 into x10 V G′0 x10 V G′ (bj , j) by transforming V G(aj ) into V G′0 and V G0 into V G′ (bj , j).
Next we transform
x10 V G(ak−1 ) x10 V G0 x10 V G(ak ) x10 V G0
into
x10 V G′0 x10 V G′ (bk−1 , k − 1) x10 V G′0 x10
V G′ (bk , k) x10 V G′0 x10 V G′ (bk+1 , k + 1)

Notice that we use the fact that k ≥ 2 (we assumed that a1 is not orthogonal to any vector from
B). Note that V G′ (bk+1 , k + 1) appears in the text t even if k = N . This is because in the definition
of the text t, integer j ranges from −2N up to 3N .
We perform the transformation by performing the following steps:
1. transform V G(ak−1 ) into V G′0 ;
2. transform V G0 into V G′ (bk−1 , k − 1) x10 V G′0 ;
3. transform V G(ak ) into V G′ (bk , k) (we can do this since ak · bk = 0);
4. transform V G0 into V G′0 x10 V G′ (bk+1 , k + 1).
Now, for j = k + 1, . . . , M − 1 transform x10 V G(aj ) x10 V G0 into x10 V G′0 x10 V G′ (bj+1 , j + 1)
similarly as before. Next, transform x10 V G(aM ) x10 into x10 V G′0 x10 . Finally, transform y 6 into
+1
+1
CG′ (bM
) appearing in V G′ (bM +1 , M + 1). We can do this since bM
= 0.
1
1
Lemma 6. If a substring of t can be derived from p, then there are two orthogonal vectors.
Proof. By the construction, every substring x10 from p must be mapped to a unique substring x10
in t (there are no substrings of t that have more than 10 symbols x). Because of this, every V G(ai )
must be mapped to V G′0 or V G′ (bj , j) for some j. If the latter case occurs, the corresponding
vectors are orthogonal and we are done. It remains to consider the case that all vector gadgets
V G(ai ) get mapped to V G′0 . Consider any vector gadget V G0 in p. To the left of it we have the
12

<!-- page 14 -->

sequence x10 and to the right of it we have the sequence x10 . Each one of these two sequences x10
in p gets mapped to a unique sequence x10 in t. We call the vector gadget V G0 nice if the two
unique sequences x10 are neighbouring in t, that is, there is no other sequence x10 in t between the
two unique sequences. We consider two cases below.
Case 1 There is a vector gadget V G0 in p that is not nice. Take any vector gadget V G0 that is
′
not nice and denote it by v. The gadget v is immediately to the right of the expression V G(ai ) x10
′
in p for some i′ ∈ [M ]. V G(ai ) is mapped to V G′0 (otherwise, we have found an orthogonal pair of
′′
vectors, as per the discussion above) and this V G′0 is to the left of substring x10 V G′ (bj , j ′′ ) in t for
′′
some j ′′ . Because v is not nice, a prefix of it must map to V G′ (bj , j ′′ ) x10 V G′0 . We claim that entire
′′
v gets mapped to V G′ (bj , j ′′ ) x10 V G′0 . If this is not the case, then a prefix of v must be mapped
′′
′′
to V G′ (bj , j ′′ ) x10 V G′0 x10 V G′ (bj +1 , j ′′ + 1) (since sequence x10 in p to the right of v must be
′′
′′
mapped to x10 ). A prefix of v can’t be mapped to V G′ (bj , j ′′ ) x10 V G′0 x10 V G′ (bj +1 , j ′′ + 1) since
v = (y ∗ x∗ )d+10 y ∗ can produce sequence with at most d+11 substrings of maximal length consisting
′′
′′
entirely of symbols y but sequence V G′ (bj , j ′′ ) x10 V G′0 x10 V G′ (bj +1 , j ′′ + 1) has 3 d+1
2 > d + 11 (if
d ≥ 100) subsequences of maximal length consisting entirely of y. Therefore, we are left with the
′′
case that entire v is mapped to V G′ (bj , j ′′ ) x10 V G′0 . The gadget v is to the left of vector gadget
′′
′
′′
V G(ai +1 ) and V G′ (bj , j ′′ ) x10 V G′0 is to the left of vector gadget V G′ (bj +1 , j ′′ + 1). We conclude
′
′′
′
′′
that V G(ai +1 ) must be mapped to V G′ (bj +1 , j ′′ + 1). This implies that ai +1 · bj +1 = 0 and we
are done.
Case 2 All vector gadgets V G0 in p are nice. We start p with y 6 followed immediately by x10 .
′
This means that y 6 is mapped to CG′ (bjd , j ′ ) for some even j ′ (by the construction of coordinate
gadgets CG′ ). Consider vector gadgets in p from left to the right. We must have that V G(a1 ) is
mapped to V G′0 , that V G0 is mapped to V G′ (b1 , 1) (since every V G0 in p is nice), that V G(a2 ) is
mapped to V G′0 , that V G0 is mapped to V G′ (b2 , 2) (since every V G0 in p is nice) and so forth. Since
M is odd, we have that V G(aM ) is mapped to V G′0 and that this vector gadget V G′0 is followed
′
by x10 V G′ (bj +M , j ′ + M ). V G(aM ) is followed by x10 y 6 and this means that y 6 is mapped to the
′
′
beginning of V G′ (bj +M , j ′ + M ). This is impossible since V G′ (bj +M , j ′ + M ) does not contain
a substring of length 6 or more consisting of symbols y (observe that j ′ + M is odd and see the
construction of vector gadget V G′ ). We get that Case 2 can’t happen.

3.4

Hardness for type “·+·”

Theorem 4. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d with M ≤ N ,
we can construct a regular expression p and a sequence of symbols t, in O(N d) time, such that a
substring of t can be derived from p if and only if there are a ∈ A and b ∈ B such that a · b = 0.
Furthermore, p is a concatenation of “+” of sequences, |p| ≤ O(M d) and |t| ≤ O(N d).
Proof. W.l.o.g., d is even. First, we will construct our pattern p. For an integer v ∈ {0, 1} and an
integer i ∈ [d] (remember that [d] = {1, 2, . . . , d}), we construct the following pattern coordinate
gadget


[x]+
if v = 0 and i ≡ 1(mod 2);



[xx]+ if v = 1 and i ≡ 1(mod 2);
CG(v, i) :=

[y]+
if v = 0 and i ≡ 0(mod 2);



[yy]+ if v = 1 and i ≡ 0(mod 2).
13

<!-- page 15 -->

For a vector a ∈ {0, 1}d , we define a pattern vector gadget as [x4 ]+ [y 4 ]+ followed by the concatenation of all coordinate gadgets:
V G(a) := [x4 ]+ [y 4 ]+ CG(a1 , 1) CG(a2 , 2) CG(a3 , 3) . . . CG(ad , d).
We also need two other pattern vector gadgets
V G0 := [x4 y 4 ]+ ([x]+ [y]+ )d/2 ;

V G1 := [x4 ]+ [y 4 ]+ ([x]+ [y 8 ]+ )d/2 .

Our pattern is then defined as follows:


K

p := V G1 
V G(1d ) V G0 V G(aj ) V G0  V G(1d ) V G(1d ) V G1 .
j∈[M ]

Now we proceed with the construction of our text. For integers v ∈ {0, 1}, i ∈ [d], we define the
following text coordinate gadget


xx if v = 0 and i ≡ 1(mod 2);



x
if v = 1 and i ≡ 1(mod 2);
CG′ (v, i) :=

yy if v = 0 and i ≡ 0(mod 2);



y
if v = 1 and i ≡ 0(mod 2).

For vector b ∈ {0, 1}d , we define the text vector gadget as x4 y 4 followed by the concatenation of
all coordinate gadgets:
V G′ (b) := x4 y 4 CG′ (b1 , 1) CG′ (b2 , 2) CG′ (b3 , 3) . . . CG′ (bd , d).

In what follows, we will use the following important property of vector gadgets V G and V G′ .
First, observe that for integers u, v ∈ {0, 1}, i ∈ [d], we can derive CG′ (v, i) from CG(v, i) if and
only if uv = 0. It means that for any vectors a, b ∈ {0, 1}d , V G′ (b) can be derived from V G(a) if
and only if a · b = 0.
We will also need additional text vector gadgets
V G′0 := x4 y 4 (x4 y 4 )d/2 ;

V G′1 := x4 y 4 (x (y)8 )d/2 .

Our text is then defined as follows:
t :=

6N
K

j=−5N


V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1 ,

where we assume bj := 1d for j 6∈ [N ]. That is, bj is vector with d entries all equal to 1 for j 6∈ [N ].
We have to show that we can derive a substring of t from p if and only if there are two orthogonal
vectors in A and B. This follows from lemmas 7 and 8 below.
Lemma 7. If there are two vectors a ∈ A and b ∈ B that are orthogonal, then a substring of t can
be derived from p.

14

<!-- page 16 -->

Proof. We assume that ak · br = 0 for some k ∈ [M ], r ∈ [N ].
Observe that the pattern p starts with a prefix V G1 , which is then followed by the sequence
V G(1d ) V G0 V G(ai ) V G0
repeated M times for different i ∈ [M ] and ends with the suffix V G(1d ) V G(1d ) V G1 . We refer to
V G(1d ) V G0 V G(ai ) V G0
corresponding to a specific i ∈ [M ] as the i-th group of p. Similarly, we observe that the text t is
concatenation of sequences
V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1
corresponding to different j = −5N, . . . , 6N . We refer to that sequence corresponding to particular
j = −5N, . . . , 6N as the j-th group of t.
We transform p into the following substring t′ of t:
t′ := V G′1

MK
+r−k

j=1+r−k


V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1 .

Note that t′ starts with the prefix V G′1 which is a suffix of the (r − k)-th group of t. Then it consists
of groups 1 + r − k to M + r − k of t. We transform p into t′ from left to right. First, we transform
V G1 into V G′1 in the unique way. Then, for i = 1, . . . , k − 1, we transform the i-th group of p into
the (i + r − k)-th group of t′ . We do that as follows. For each i ∈ {1, . . . , k − 1}, we perform the
following transformations: transform V G(1d ) into V G′ (0d ); transform V G0 into V G′0 V G′ (bi+r−k );
transform V G(ai ) into V G′ (0d ); transform V G0 into V G′0 V G′1 .
Now we transform the k-th and the (k + 1)-th group of p into the prefix
V G′ (0d ) V G′0 V G′ (br ) V G′ (0d ) V G′0 V G′1 V G′ (0d ) V G′0 V G′ (br+1 )
of the remainder of t′ . This is done by transforming V G(1d ) into V G′ (0d ), V G0 into V G′0 , V G(ak )
into V G′ (br ) (which can be done because ak · br = 0), V G0 into V G′ (0d ), V G(1d ) into V G′0 , V G0
into V G′1 , V G(ak+1 ) into V G′ (0d ) and V G0 into V G′0 V G′ (br+1 ). The remainder of the pattern p
consists of groups k + 2, . . . , M and the suffix V G(1d ) V G(1d ) V G1 , while the remainder of t′ is


MK
+r−k

V G′ (0d ) V G′0 V G′1 V G′ (0d ) V G′0 V G′ (bj )  V G′ (0d ) V G′0 V G′1 .
t′′ := 
j=r+2

We transform group i = k+2, . . . , M of p into a substring V G′ (0d ) V G′0 V G′1 V G′ (0d ) V G′0 V G′ (bi+r−k ),
as follows. We transform V G(1d ) into V G′ (0d ), V G0 into V G′0 V G′1 , V G(ai ) into V G′ (0d ) and V G0
into V G′0 V G′ (bi+r−k ). It remains to transform the suffix V G(1d ) V G(1d ) V G1 of p into the suffix
V G′ (0d ) V G′0 V G′1 of t′′ , which can be done in a non-ambiguous way.

Lemma 8. If a substring of t can be derived from p, then there are two vectors a ∈ A and b ∈ B
that are orthogonal.

15

<!-- page 17 -->

Proof. Notice that p starts and ends with V G1 and that this vector gadget does not appear anywhere
else in the pattern. V G1 contains d/2 patterns [y 8 ]+ . This means that it must map to a substring
of t containing d/2 substrings consisting of only symbols y of length divisible by 8. Text t contains
vector gadgets V G′1 that has substrings of symbols y of length divisible by 8. No other vector
gadget in t has this property. Therefore, both vector gadgets V G1 of p must map to V G′1 in t.
Consider V G1 at the beginning of p. Suppose that it maps to V G′1 that comes from the r-th group
of t, for some r. The pattern p contains 2M vector gadgets V G0 . Each one of them starts with
[x4 y 4 ]+ . In the proof of Lemma 7 we transform [x4 y 4 ]+ into x4 y 4 so that V G0 maps to a single
corresponding vector gadget in t or we transform [x4 y 4 ]+ into (x4 y 4 )2+d/2 so that V G0 maps to
two vector gadgets in t. Here we consider the first copy v of the vector gadget V G0 of p such
that the corresponding [x4 y 4 ]+ does not map to (x4 y 4 )2+d/2 . If there is no such a vector gadget,
we can check that the j-th group of p must be transformed into the j + r-th group of t for all
t ∈ [M ]. That means that the suffix V G(1d ) V G(1d ) V G1 of p must be transformed into the prefix
of the M + r + 1-st group of t. This implies that V G1 is transformed into V G′ (bM +r+1 ), which
is impossible by the construction of the vector gadgets. Therefore, there must be vector gadget v
with the stated properties.
Suppose that v comes from the i-th group in p. For all vector gadgets of type V G0 to the left
of v, the prefix [x4 y 4 ]+ got mapped to (x4 y 4 )2+d/2 . This means that group k = 1, . . . , i − 1 of p
is transformed into the group k + r of t. There are two cases to consider, depending on whether v
comes before or after the vector gadget V G(ai ) in the group i of p.
Case 1: v comes after V G(ai ) We show that this case cannot occur. We start by observing
that V G(ai ) maps to V G′ (0d ). The sequence [x4 y 4 ]+ in v cannot map to 3 + d/2 or more copies
of x4 y 4 because it would imply that the first substring x (y)8 of V G′1 can be derived from [x4 y 4 ]+ ,
which is impossible. It follows that [x4 y 4 ]+ maps to 1 + d/2 or fewer copies of x4 y 4 . Note that
v is followed by V G(1d ) and V G(1d ) starts with [x4 ]+ [y 4 ]+ . This means that [x4 y 4 ]+ of v must
map to only one copy of x4 y 4 (i.e., V G0 is mapped to V G′0 ), so that [x4 ]+ [y 4 ]+ at the beginning
of V G(1d ) can map to x4 y 4 at the beginning of V G′1 . This in turn implies that V G(1d ) must be
transformed into a suffix of V G′1 , which is again impossible since V G′1 contain a single symbol x
surrounded by symbols y on both sides and the regular expression V G(1d ) can’t produce such a
substring. Thus, Case 1 cannot occur.
Case 2: v comes before V G(ai ) The argument is similar to the previous paragraph except we
will conclude that ai · bi+r = 0. Similarly as before, we can conclude that [x4 y 4 ]+ in v cannot be
mapped to 3 + d/2 or more copies of x4 y 4 . This is because V G′ (bi+r ) starts with x4 y 4 CG′ (bi+r
1 , 1)
and, if [x4 y 4 ]+ in v was mapped to 3 + d/2 or more copies of x4 y 4 , CG′ (bi+r
,
1)
would
be
a
prefix
1
4
4
+
of sequence that can be derived from [x y ] . We can verify that it is impossible. Assume that
[x4 y 4 ]+ maps to 1 + d/2 or fewer copies of x4 y 4 . Because V G(ai ) starts with [x4 ]+ [y 4 ]+ , we have
that V G(ai ) is transformed into V G′ (bi+r ). This is possible only if ai · bi+r = 0 by the construction
of the coordinate gadgets.

3.5

Hardness for type “·|·”

Theorem 5. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d with M ≤ N ,
we can construct a regular expression p and a sequence of symbols t, in O(N d) time, such that a
substring of t can be derived from p if and only if there are a ∈ A and b ∈ B such that a · b = 0.
Furthermore, p is of type “·|·”, |p| ≤ O(M d) and |t| ≤ O(N d).
16

<!-- page 18 -->

Proof. We will modify the construction for “·+·” so that it gives a hardness proof for “·|·”. The
text t remains the same. We will modify pattern p as follows. First, recall that p is a repeated
concatenation of regular expressions of the form [x]+ , [x x]+ , [y]+ , [y y]+ , [x4 ]+ , [y 4 ]+ , [x4 y 4 ]+ , [y 8 ]+
in some order. In the proof of Lemma 7, all of those sequences get repeated at most 8 times, except
for the sequence x4 y 4 which gets repeated once or 2 + d/2 times. Therefore, we replace [s]+ with
[s | s2 | s3 | s4 | s5 | s6 | s7 | s8 ] for all s, except when s = x4 y 4 . In the latter case we replace [x4 y 4 ]+
with [x4 y 4 | (x4 y 4 )2+d/2 ]. The proof of Lemma 7 goes through as before. The proof of Lemma 8
also goes through because, after these modifications, if some sequence z can be derived from the
modified pattern, it can be also derived from the initial pattern.

3.6

Hardness for type “·+|”

Theorem 6. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d with M ≤ N ,
we can construct a regular expression p and a sequence of symbols t, in O(N d) time, such that a
substring of t can be derived from p iff there are a ∈ A and b ∈ B such that a · b = 0. Furthermore
p has type “·+|”, |p| ≤ O(M d) and |t| ≤ O(N d).
Proof. W.l.o.g., M ≡ 0(mod 2) and d ≡ 0(mod 2). W.l.o.g., if there are two orthogonal vectors,
then there are ai ∈ A, bj ∈ B with ai · bj = 0 and i ≡ j(mod 2).
First, we will construct our pattern. We need the following coordinate gadget construction. For
an integers v ∈ {0, 1} and i ∈ [d],


[x | y | 0 | 1]+
if v = 0 and i ≡ 1(mod 2);



[x | y | 0]+
if v = 1 and i ≡ 1(mod 2);
CG(v, i) :=
′
′
+

[x | y | 0 | 1 ]
if v = 0 and i ≡ 0(mod 2);



[x | y | 0′ ]+
if v = 1 and i ≡ 0(mod 2).

For a vector a ∈ {0, 1}d , we define pattern vector gadget V G(a) as concatenation of all coordinate
gadgets for entries of the vector:
V G(a) := CG(a1 , 1) CG(a2 , 2) CG(a3 , 3) . . . CG(ad , d).
We also need another vector gadget
V G0 := ([0|1]+ [0′ |1′ ]+ )d/2 ,
that is, V G0 is equal to the vector gadget V G(0d ) except it can’t produce symbols x and y. Our
pattern p is then defined as follows:


K

p := x+ 
V G0 [x|y]+ V G(ai ) [x|y]+  V G0 x+ .
i∈[M ]

Now we construct our text t. First, for a vector b ∈ {0, 1}d , we define text vector gadget
V G′ (b) := b1 b′2 b3 b′4 b5 . . . b′d ,

17

<!-- page 19 -->

that is, it is concatenation of all entries and we put ′ for every second entry. Note that we can
derive V G′ (b) from V G(a) iff a · b = 0. Also, we can derive V G′ (b) from V G0 for any b. Our text
t is defined as
10N 

K
xd+10 V G′ (b2j ) y d+10 V G′ (b2j+1 ) ,
t :=
j=−9N

where, for j 6∈ [N ], we set bj := 1d (vector consisting of only 1s).
We need to show that we can derive a substring of t from p iff there are two orthogonal vectors.
This follows from lemmas 9 and 10 below.
Lemma 9. If there are two vectors a ∈ A and b ∈ B that are orthogonal, then a substring of t can
be derived from p.
Proof. We assume that ak ·bk = 0 for some k ∈ [M ]. In the general case when ak ·br = 0 for k ∈ [M ],
r ∈ [N ] and k ≡ r(mod 2), the proof is analogous. Furthermore, we assume that k ≡ 1(mod 2)
(the case k ≡ 0(mod 2) is analogous).
We transform p into the following substring t′ of t:
t′ :=x V G′ (b0 ) y d+10 V G′ (b1 ) xd+10 V G′ (b2 ) y d+10 V G′ (b3 ) xd+10 V G′ (b4 ) y d+10 V G′ (b5 ) xd+10
. . . xd+10 V G′ (bM ) y d+10 V G′ (bM +1 ) x.
Note that t′ starts and ends with x, because M ≡ 0(mod 2).
To show how to transform p into t′ , it’s helpful to write pattern p as p = p′′ p′′′ , where
p′′ :=x+ V G0 [x|y]+ V G(a1 ) [x|y]+ V G0 [x|y]+ V G(a2 ) [x|y]+
. . . V G(ak−1 ) [x|y]+ V G0 ,
p′′′ :=[x|y]+ V G(ak ) [x|y]+ V G0 [x|y]+ V G(ak+1 ) [x|y]+ V G0
. . . V G0 [x|y]+ V G(aM ) [x|y]+ V G0 x+
and text t as t = t′′ t′′′ , where
t′′ :=x V G′ (b0 ) y d+10 V G′ (b1 ) xd+10 V G′ (b2 ) y d+10 V G′ (b3 ) xd+10 V G′ (b4 )
. . . V G′ (bk−3 ) y d+10 V G′ (bk−2 ) xd+10 V G′ (bk−1 ),
t′′′ :=y d+10 V G′ (bk ) xd+10 V G′ (bk+1 ) y d+10 V G′ (bk+2 ) xd+10 V G′ (bk+3 ) y d+10 V G′ (bk+4 ) xd+10
. . . xd+10 V G′ (bM ) y d+10 V G′ (bM +1 ) x.
Now we transform p into t in two steps - transform p′′ into t′′ and p′′′ into t′′′ .
Transform p′′ into t′′ p′′ starts with x+ , which we transform into x. For the rest of p′′ , we
make transformations according to the following rules. We make transformations starting from the
beginning of p′′ . If we see V G0 , we transform it into the corresponding V G′ (bj ). If we see [x|y]+ , we
transform it into x5 or y 5 (according to which symbols are in the corresponding positions in t′′ ). If
we see V G(ai ), we transform it into xd or y d (according to which symbols are in the corresponding
positions in t′′ ).
18

<!-- page 20 -->

Transform p′′′ into t′′′ Notice that t′′′ starts with y. This is because k ≡ 1(mod 2). Now we
make the following 3 transformations from the prefix of p′′′ : transform [x|y]+ into y d+10 , V G(ak )
into V G′ (bk ) (we can do this because ak · bk = 0) and [x|y]+ into xd+10 . Now we transform the rest
of p′′′ into the remainder of t′′′ . We do that starting from the beginning of the remainder of p′′′ . If
we see V G0 , we transform it into the corresponding V G′ (bj ). If we see [x|y]+ , we transform it into
x5 or y 5 (depending on which symbols are in the corresponding positions in t). If we see V G(ai ),
we transform it into xd or y d (depending on which symbols are in the corresponding positions in
t). Finally, to finish the transformation, we transform x+ into x.
Lemma 10. If a substring of t can be derived from p, then there are two orthogonal vectors.
Proof. Recall that the pattern p consists of M + 1 vector gadgets V G0 . We enumerate the gadgets
with integers 0, 1, 2, 3, . . . , M . Each of them consists of d symbols from the alphabet {0, 1, 0′ , 1′ }.
Therefore, by the construction of t, it must be the case that every vector gadget V G0 transforms
into V G′ (bj ) for some j. Assume that the 0-th V G0 transforms into V G′ (b0 ). If it transforms into
V G′ (bj ) for some other j, j ≡ 0(mod 2), the proof is analogous. The modularity constraint on
j holds because p starts with x and the symbol x must precede V G′ (bj ). We consider two cases
below:
Case 1 There exists t ∈ {0, 1, 2, . . . , M } such that the t-th V G0 is not transformed into V G′ (bt ).
Pick smallest such t. This means that V G′ (bt ) has been derived from V G(at ). From the construction
of V G(at ) and V G′ (bt ), we conclude that at · bt = 0.

Case 2 For every t ∈ {0, 1, 2, . . . , M }, the t-th V G0 is transformed into V G′ (bt ). Consider the
M -th vector gadget V G0 . It is transformed into V G′ (bM ). The M -th vector gadget V G0 is followed
by x+ and V G′ (bM ) is followed by y. This means that we can’t derive this substring of t from p.
Thus, Case 2 cannot happen.

3.7

Hardness for type “·|+”

Theorem 7. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d with M ≤ N ,
we can construct a regular expression p and a sequence of symbols t, in O(N d) time, such that a
substring of t can be derived from p iff there are a ∈ A and b ∈ B such that a · b = 0. Furthermore
p has type “·|+”, |p| ≤ O(M d) and |t| ≤ O(N d).

Proof. We will modify the construction for “·+|” so that it gives the hardness proof for “·|+”.
Whenever we have a regular expression in p of the form [s1 |s2 |s3 | . . . |sl ]+ for l ≥ 1 symbols
+ +
+
′
s1 , s2 , . . . , sl , we replace it with [s+
1 |s2 |s3 | . . . |sl ]. Let p be the new regular expression that
we obtain this way. The text t remains unchanged. Observe that, if we can derive some sequence
x from p′ , we were able to derive x from p as well. Because of this, if a subsequence of t can be
derived from p, then we can conclude that there are two orthogonal vectors between A and B. It
remains to show that, if there are two orthogonal vectors, then a subsequence of t can be derived
from p. This follows from the proof of Lemma 9. In particular, we observe that in the proof of
Lemma 9, if we derive a sequence from [s1 |s2 |s3 | . . . |sl ]+ , such sequence is of the form sji for some
j ≥ 1 and i ∈ [l].

19

<!-- page 21 -->

4

Reductions for the Membership problem
4.1 Hardness for type “·∗”

Theorem 8. Given sets A = {a1 , . . . , aN } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d , we can
construct a regular expression p and a sequence of symbols t, in O(N d) time, such that t can be
derived from p if and only if there are a ∈ A and b ∈ B such that a · b = 0. Furthermore, p is of
type “·∗”, |p|, |t| ≤ O(N d).
Proof. We slightly modify the construction from Theorem 3. We instantiate the construction from
Theorem 3 with M = N . We obtain a pattern p′ and a text t such that a substring of t can be
derived from p′ iff there are two orthogonal vectors. We define the pattern p as follows:




|t|
|t|
K
K
(x∗ y ∗ ) .
(x∗ y ∗ ) ·p′ · 
p := 
j=1

j=1

We claim that t can be derived from p iff there are two orthogonal vectors. This follows from
construction of p and Theorem 3. We know that a substring of t can be derived from p′ iff there
J|t|
are two orthogonal vectors. Expressions j=1 (x∗ y ∗ ) allow us to derive the remaining prefix and
suffix of t.

4.2

Hardness for type “·+·”

Theorem 9. Given sets A = {a1 , . . . , aN } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d , we can
construct a regular expression p and a sequence of symbols t, in O(N d) time, such that t can be
derived from p if and only if there are a ∈ A and b ∈ B such that a · b = 0. Furthermore, p is a
concatenation of “+” of sequences, |p|, |t| ≤ O(N d).
Proof. We will adapt the construction from Theorem 4. We instantiate the construction from
Theorem 4 with M = N . We obtain a pattern p′ and a text t such that a substring of t can be derived
from p′ iff there are two orthogonal vectors. The final pattern p := p1 ·p′ ·p2 is a concatenation of
three expressions p1 , p′ , p2 . Each one of expressions p1 , p′ and p2 is a concatenation of “+” of
sequences. Clearly, if t can be derived from p, then a substring of t can be derived from p′ . By the
statement of Theorem 4, there must be two orthogonal vectors in this case. Therefore our goal is
to construct p1 and p2 such that text t can be derived from p = p1 ·p′ ·p2 if there are two orthogonal
vectors. In the rest of this section we achieve this goal.
If there are two orthogonal vectors, then by the proof of Theorem 4, the text
t=

6N
K

V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1

j=−5N



can be written as t = t1 t2 t′ t3 t4 , where the sequences t1 , t2 , t′ , t3 and t4 have the following properties.
•
t1 =

w−N
K

j=−5N


V G′ (0d ) V G′0 V G′ (1d ) V G′ (0d ) V G′0 V G′1 .

20

<!-- page 22 -->

•


t2 = 



V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1 

w−1
K

j=w−N +1

·V G′ (0d ) V G′0 V G′ (bw ) V G′ (0d ) V G′0 .

•
t

′

= V G′1

N
+w
K

V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1

j=1+w

for some w ∈ {1 − N, . . . , N − 1} and t′ can be derived from p′ .
•
t3 =

w+2N
K−1


V G′ (0d ) V G′0 V G′ (bj ) V G′ (0d ) V G′0 V G′1 .

6N
K


V G′ (0d ) V G′0 V G′ (1d ) V G′ (0d ) V G′0 V G′1 .

j=N +w+1

•
t4 =



j=w+2N

Our goal is to construct expressions p1 and p2 such that t1 t2 can be derived from p1 (independently
of the value w) and t3 t4 can be derived from p2 (independently of the value w). We construct p1
and p2 as follows.
• We note that the expression
p̂ :=

N
K
j=1


+ 
V G′ (0d ) V G′0 V G′ (1d ) V G′ (0d ) V G′0 V G′1

can derive the sequence t1 independently of the value w. Let z be an arbitrary sequence of
symbols x and y. As long as there are two neighboring symbols x (y, resp.), we replace those
two symbols by one copy of symbol x (y, resp.). Let z ′ be the resulting sequence. We define
expression f (z) as follows:
|z ′ |
K
f (z) :=
[zj′ ]+ .
j=1

That is, in the expressions f (z) we allow to repeat any symbol in z ′ one or more times. Note
that f (t2 ) can derive t2 independently of the value w and that f (t2 ) does not depend on
vectors bj . This implies that the expression p1 := p̂·f (t2 ) can derive t1 t2 which is what we
needed.

• We define p2 := f (t3 )·p̂. We can check that f (t3 ) and p̂ do not depend on the value w and
vectors bj . We can also check that we can derive t3 t4 from p2 .

21

<!-- page 23 -->

4.3

Hardness for type “·|·”

Theorem 10. Given sets A = {a1 , . . . , aN } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d , we can
construct a regular expression p′′ and a sequence of symbols t, in O(N d) time, such that t can be
derived from p′′ if and only if there are a ∈ A and b ∈ B such that a · b = 0. Furthermore, p′′ is of
type “·|·”, |p′′ |, |t| ≤ O(N d).

Proof. We will modify the construction for “·+·” (Theorem 9) so that it gives a hardness proof
for “·|·”. The text t remains the same. We will modify pattern p as follows. First, recall that
p = p1 ·p′ ·p2 . We transform p into an expression of type “·|·” in two steps.
• We transform p′ into a sequence of type “·|·” in the same way as it is done in the proof of
Theorem 5.
• Expressions p1 and p2 are repeated concatenations of expressions

+
[x]+ , [y]+ , V G′ (0d ) V G′0 V G′ (1d ) V G′ (0d ) V G′0 V G′1 .

In the proof of Theorem 9 we can repeat each one of argument expressions
x, y, V G′ (0d ) V G′0 V G′ (1d ) V G′ (0d ) V G′0 V G′1
at most 8 times so that we are still able to derive sequences t1 t2 and t3 t4 from p1 and 2
respectively. Thus, we replace [s]+ by [s | s2 | s3 | s4 | s5 | s6 | s7 | s8 ] for each
s = x, y, V G′ (0d ) V G′0 V G′ (1d ) V G′ (0d ) V G′0 V G′1
in p1 and p2 .

Let p′′ be the resulting expression. If the sequence t can be derived from p, it can still be derived
from p′′ . This follows from Theorem 5 and the construction of t. It remains to argue that if t can’t
be derived from p, then it can’t be derived from p′′ . This is true by the transformation above and
Theorem 5.

4.4

Hardness for type “·+|”

Theorem 11. Given sets A = {a1 , . . . , aM } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d with M ≤ N ,
we can construct a regular expression p and a sequence of symbols t, in O(N d) time, such that t
can be derived from p iff there are a ∈ A and b ∈ B such that a · b = 0. Furthermore p has type
“·+|”, |p| ≤ O(M d) and |t| ≤ O(N d).

Proof. We adapt the hardness proof from Theorem 6. We instantiate the construction from Theorem 6 and we obtain a pattern p′ and a text t such that a substring of t can be derived from p′ iff
there are two orthogonal vectors. We define the new pattern p as follows:
p := [0 | 1 | 0′ | 1′ | x | y]+ · p′ · [0 | 1 | 0′ | 1′ | x | y]+ .

We claim that t can be derived from p iff there are two orthogonal vectors. If t can be derived from
p, then a substring of t can be derived from p′ and by Theorem 6 there are two orthogonal vectors.
Conversely, if there are two orthogonal vectors then by Theorem 6 we can derive a substring of t
from p′ . We derive the remaining prefix and suffix of t from expressions [0 | 1 | 0′ | 1′ | x | y]+ .
22

<!-- page 24 -->

4.5

Hardness for type “·|+”

Theorem 12. Given sets A = {a1 , . . . , aN } ⊆ {0, 1}d and B = {b1 , . . . , bN } ⊆ {0, 1}d , we can
construct a regular expression p and a sequence of symbols t, in O(N d) time, such that t can be
derived from p iff there are a ∈ A and b ∈ B such that a · b = 0. Furthermore p has type “·|+”,
|p|, |t| ≤ O(N d).
Proof. We adapt the hardness proof from Theorem 7. We instantiate the construction from Theorem 7 with M = N . We obtain a pattern p′ and a text t such that a substring of t can be derived
from p′ iff there are two orthogonal vectors. From the proof of Theorem 6 we have that
10N 

K
xd+10 V G′ (b2j ) y d+10 V G′ (b2j+1 ) .
t=
j=−9N

Let k = |xd+10 V G′ (b2j ) y d+10 V G′ (b2j+1 )| be the length of sequence xd+10 V G′ (b2j ) y d+10 V G′ (b2j+1 ).
Notice that k does not depend on j. Our new sequence p is constructed as follows:


7N
7N
Kk
Kk
[0+ | 1+ | [0′ ]+ | [1′ ]+ | x+ | y + ].
[0+ | 1+ | [0′ ]+ | [1′ ]+ | x+ | y + ] · p′ ·
p := 
j=1

j=1

If t can be derived from p, then a substring of t can be derived from p′ and by Theorem 7 there
are two orthogonal vectors. If there are two orthogonal vectors then we can derive a substring of t
′
from
check that we can derive the remaining prefix and suffix of t from expressions
J7N kp . +We +can easily
′
+
′
+
+
+
j=1 [0 | 1 | [0 ] | [1 ] | x | y ].

5

Algorithms

5.1

Algorithm for the Word Break problem

Word Break problem Given
sequence t of length |t| = n and a collection of binary5
P a binary
′
sequences p with total length p′ ∈p |p | = m, decide if the sequence t can be written as a concatenation t = t1 . . . tk such that ti ∈ p for every i ∈ [k]. If t can be written in such a way, we call t
decomposable.

We will solve this problem in time Õ(n · m0.4444... ) = Õ n · m0.5−1/18 .6

√
As a warm-up, we first solve the problem in time Õ(n· m) and then we provide a Õ n · m0.5−1/18
time algorithm.
√
5.1.1 Õ(n · m) time algorithm for the Word Break problem
′
′
Let d(p)
P := {|p | : p ∈ p}. We will show how to√solve the Word Break problem in time Õ(n·|d(p)|).
Since i∈[t] |pi | = m, we have that |d(p)| ≤ O( m), which implies the upper bound.
We will use the following lemma.

Lemma 11. We can randomly choose a hash function h : {0, 1}∗ → N and preprocess t in Õ(n)
time such that the following holds:
5
W.l.o.g. we assume that all sequences are binary. If this is not so, we encode every symbol of the alphabet using
a binary sequence of length ⌈log s⌉ + 1 where s is the size of the alphabet. This increases the lengths of the sequences
by a logarithmic multiplicative factor.
6
Õ(·) notation hides a poly log factor.

23

<!-- page 25 -->

• Given any substring t′ of t, we can compute the hash h(t′ ) in Õ(1) time.
• For any two sequences t′′ 6= t′ (not necessarily substrings of t), Pr[h(t′′ ) = h(t′ )] ≤ 1/n10 .
• For any sequence t′ (necessarily substring of t), we can compute h(t′ ) in time Õ(|t′ |).
Proof. E.g., use Rabin-Karp rolling hash.
Theorem 13. The Word Break problem can be solved in time Õ(n · |d(p)|).
Proof. Preprocess t according to Lemma 11 and compute h(p) := {h(p′ ) : p′ ∈ p}.
We solve the problem using dynamic programming. We use the table D : [n + 1] → {0, 1}. The
algorithm determines the values D[n], D[n − 1], . . . , D[1] (in this order). We set D[i] = 1 iff the
sequence ti ti+1 . . . tn is decomposable.
• Set D[i] = 0 for all i = 1, . . . , n and set D[n + 1] = 1.
• For i = n, . . . , 1, set D[i] = 1 iff there exists j > i such that D[j] = 1 and (j − i) ∈ d(p), and
h(ti . . . tj−1 ) ∈ h(p).
• t is decomposable iff D[1] = 1.

Õ n · m0.5−1/18 time algorithm for the Word Break problem
√
Similarly as in the Õ(n · m) time algorithm, we fill out the table D : [n + 1] → {0, 1}:

5.1.2

• Set D[i] = 0 for all i = 1, . . . , n and set D[n + 1] = 1.
• For every i = n, . . . , 1 in this order, set D[i] = 1 iff the sequence ti ti+1 . . . tn is decomposable.
• t is decomposable iff D[1] = 1.
We will show that the second step can be performed in Õ(n · m0.5−α ) time for sufficiently small
constant α > 0. We will later show that we can set α = 1/18. For now we can think of α > 0 as a
sufficiently small constant, say, α = 0.01. We will make the following two assumptions, justified by
the next two lemmas.
Lemma 12. For all p′ ∈ p, |p′ | ≥ m0.5−α .
Proof. Let p̂ := {p′ ∈ p : |p′ | < m0.5−α }. Clearly, we have that |d(p̂)| < m0.5−α . Therefore, as
we perform the second step of the algorithm, for every i = n, . . . , 1, we set D[i] = 1 if there exists
j > i with D[j] = 1 and ti . . . tj−1 ∈ p̂ in the same way as it is done in the proof of Theorem 13.
For every i this takes Õ(|d(p̂)|) = Õ(m0.5−α ) time. Therefore, the total runtime corresponding to
processing sequences in p̂ is Õ(n · m0.5−α ).
Lemma 13. For all p′ ∈ p, |p′ | ≤ m0.5+α .
P
Proof. Let p̂ := {p′ ∈ p : |p′ | > m0.5+α }. Since p′ ∈p |p′ | = m, we have |d(p̂)| ≤ |p̂| ≤ m0.5−α .
Similarly as in the proof of Lemma 12 we can set D[i] = 1 if there exists j > i with D[j] = 1
and ti . . . tj−1 ∈ p̂. Therefore, the total runtime corresponding to processing sequences in p̂ is
Õ(n · |p̂|) ≤ Õ(n · m0.5−α ).
24

<!-- page 26 -->

In the rest of the section we will show that the second step of the algorithm can be implemented
in Õ(n ·m0.5−α ) time if α > 0 is a sufficiently small constant. By Lemmas 12 and 13, we can assume
that for all p′ ∈ p, m0.5−α ≤ |p′ | ≤ m0.5+α .
We build a trie data structure T for p. It is a binary tree where each node has two children.
Each node corresponds to a prefix of a sequence in p. The root node corresponds to an empty
sequence. If a node u corresponds to the sequence s and has two children then one of the children
corresponds to the sequence s0 (s followed by 0) and the other corresponds to the sequence s1. If u
has only one child, it corresponds to either s0 or s1. If a node u corresponds to a sequence of length
i ≥ 0, u has depth i. If a node u corresponds to a sequence p′ and p′ ∈ p, then we call the node u
marked. We preprocess T in such a way that for any node u we have a pointer to node v such that
v is a marked ancestor of u of maximal depth.
P A node is not its own ancestor. This data structure
can be constructed in O(m) time. Because p′ ∈p |p′ | = m and for all p′ ∈ p, m0.5−α ≤ |p′ |, we have
the the number of marked nodes in the tree is upper bounded by |p| ≤ m0.5+α .

Preprocessing of the tree T We further preprocess the tree T and the sequence t such that
given any index i = 1, . . . , n, we can answer the following query in Õ(1) time. Specifically, the
query algorithm outputs the maximal j > i such that ti ti+1 . . . tj−1 is a prefix of a sequence in p,
and reports the node u corresponding to ti ti+1 . . . tj−1 . We build a data structure that stores the
hash values for all non-empty prefixes of all sequences in p and supports lookups in Õ(1) time. We
use Rabin-Karp rolling hash to compute the hashes. This takes Õ(m) time. We also preprocess
the sequence t so that we can compute Rabin-Karp rolling hash value in Õ(1) time for every
substring. This takes Õ(n). The total runtime of the preprocessing steps is Õ(m + n). Given i,
the query algorithm does the binary search to find the largest j > i such that the hash of sequence
ti ti+1 . . . tj−1 is in the table. Since we can do lookups in the table in Õ(1) and there are Õ(1) binary
search steps, and we can compute the rolling hash value for every substring in Õ(1) time, we get
the required upper bound Õ(1) on the query time. We can easily augment the data structure so
that we can output the corresponding node u from the tree T .
For j = 1, . . . , n/m0.5−α we call the sequence D[n − j · m0.5−α + 1] . . . D[n − (j − 1) · m0.5−α ]
the j-th chunk of the table D. We will show how to determine the values in the j-th chunk in time
Õ(m1−2α ) assuming that we have all values for chunks with indices smaller than j. This gives the
required upper bound on the runtime because there are n/m0.5−α chunks and Õ(m1−2α )·n/m0.5−α =
Õ(n · m0.5−α ). Since for all p′ ∈ p, m0.5−α ≤ |p′ | ≤ m0.5+α , the only previously computed values of
D that are needed to compute the j-th chunk are:
D[n − (j − 1) · m0.5−α + 1] . . . D[n − (j − 1) · m0.5−α + m0.5+α ]
To simplify the notation, we relabel the table D by defining the table D ′ . Specifically, we identify
D[n − j · m0.5−α + 1] . . . D[n − (j − 1) · m0.5−α + m0.5+α ] with D ′ [1] . . . D ′ [m0.5−α + m0.5+α ]. Thus,
our goal is to find the values D ′ [1] . . . D ′ [m0.5−α ] knowing the values D ′ [m0.5−α + 1] . . . D ′ [m0.5−α +
m0.5+α ]. Let t′ be the corresponding substring of t of length m0.5−α + m0.5+α .
Intuition Suppose that we want to determine value of D ′ [i]. We look for the largest j > i such
that t′i . . . t′j−1 is in p. If D ′ [j] = 1, we set D ′ [i] = 1 and move to determine D ′ [i − 1]. However, it
might be that D ′ [j] = 0 and there are integers j ′ such that i < j ′ < j and t′i . . . t′j ′ −1 is in p. For
every such j ′ we have to check whether D ′ [j ′ ] = 1 and set D ′ [i] = 1 if this happens. If there are not
too many such j ′ , we can work through all of them. It might happen that there are many such j ′ .
In this case we build a characteristic vector of the set of such j ′ s, i.e., set the entry corresponding
25

<!-- page 27 -->

to each such j ′ to 1. We then convolve the characteristic vector with D ′ . Although this does not
reduce the runtime when working with D ′ [i], the saving will occur in the future if we will need to
determine D ′ [i′ ] such that i′ < i and t′i t′i+1 . . . and t′i′ t′i′ +1 . . . share long prefixes. We will make this
more precise below.
We determine the unknown values of D ′ in two phases - preprocessing phase and online phase.
In the preprocessing phase we preprocess the known part of D ′ together with T in Õ(m1−2α ) time.
In the online phase we determine the unknown values D ′ [i] for i = m0.5−α , . . . , 1 in this order. We
spend time O(m0.5−α ) for every D ′ [i].
Preprocessing phase In the following we will define a subset of the marked nodes that we call
special. Initially the set of special nodes is empty. We will keep the invariant that if a node is
special, then all its marked ancestors are also special. Since |p| ≤ m0.5+α , there are at most m0.5+α
leaves in the tree T . Fix an arbitrary ordering of the leaves and consider the leaves one by one. Let
ℓ be the current leaf that we consider. Let c be the number of marked ancestors of ℓ that are not
special. We can determine c in time O(c) because every node in T keeps a pointer to the marked
ancestor of maximal depth. We distinguish two cases.
Case 1: c > m0.5−3α We mark ℓ and all its marked ancestors as special. Since |p| ≤ m0.5+α ,
the number of marked nodes is at most m0.5+α . This means that we happen to be in this case at
most m0.5+α /m0.5−3α = m4α times. Let d denote the depth of the node ℓ. Since for every p′ ∈ p,
|p′ | ≤ m0.5+α , we have that d ≤ m0.5+α . Let u0 , u1 , . . . , ud = ℓ be the nodes on the path from the
root of T to ℓ. The node u0 is the root of T . Let l := m0.5−α . We define ⌊d/l⌋ binary vectors
r1 , . . . , r⌊d/l⌋ ∈ {0, 1}l as follows. For i = 1, . . . , ⌊d/l⌋ and j = 1, . . . , l we set the j-th entry ri [j] of
the vector ri to be equal to ri [j] = 1 if the node u(i−1)l+j is marked and equal to ri [j] = 0 if the node
is not marked. For every i = 1, . . . , ⌊d/l⌋, we compute the convolution between the binary vector
ri and the binary vector D ′ [m0.5−α + 1] . . . D ′ [m0.5−α + m0.5+α ] in the following sense. We output a
binary vector ci with m0.5−α + m0.5+α + l − 1 entries such that for j = 1, . . . , m0.5−α + m0.5+α + l − 1,
ci [j] :=

l
X
k=1


ri [k] · D ′ [k + j + m0.5−α − l] .

When computing ci [j], if we need to access an entry D ′ [z] with z ≤ m0.5−α or z ≥ m0.5−α +m0.5+α +
1, we assume that it is equal to 0. Computing the convolution ci for ri takes O(m0.5+α log m) time
using the Fast Fourier Transform. Since we have to compute the convolution for i = 1, . . . , ⌊d/l⌋,
in total it takes O(m0.5+α log m) · d/l = O(m0.5+3α log m) time. Since we happen to be in this case
at most m4α times, the total runtime corresponding to this case is bounded by O(m0.5+7α log m) =
Õ(m1−2α ) assuming that α ≤ 1/18.

Case 2: c ≤ m0.5−3α In this case we do not do anything. Since there are at most m0.5+α leaves,
the total time corresponding to this case is upper bounded by m0.5+α · O(c) ≤ O(m1−2α ) which is
what we wanted.
See Figure 2 for an example run of the preprocessing phase.
Notice that after the preprocessing phase, the set of marked and special nodes of T form a
rooted subtree among all marked nodes of T .
Online phase We determine the values of D ′ [i] for i = m0.5−α , . . . , 1 in this order. Fix i for
which we want to determine D ′ [i]. Let j > i be the largest integer such that t′i . . . t′j−1 corresponds
to a node u in tree T . We find j using the query algorithm described before, in Õ(1) time. Let a
26

<!-- page 28 -->

Tree T :

Depth m0.5−α

ℓ1

ℓ4
ℓ2

ℓ3

Depth m0.5+α

Figure 2: An example of the preprocessing phase of tree T and vector D′ . Each white circle denotes a marked node
that is not special. Each black circle denotes a marked node that is special. We do not depict the nodes that are not
marked. Each solid line is a sequence of marked or unmarked nodes. Notice that all marked nodes are at depth at
least m0.5−α and at most m0.5+α . The tree has 4 leaves ℓ1 , ℓ2 , ℓ3 , ℓ4 . We process them in order ℓ1 , ℓ2 , ℓ3 , ℓ4 :
• Leaf ℓ1 has c = 5 marked ancestors. We assume that m0.5−3α = 5 and therefore we happen to be in case
c ≤ m0.5−3α . We do not make the marked ancestors special.
• Leaf ℓ2 has c = 9 marked ancestors. We are in c > m0.5−3α case and we make ℓ2 and all marked ancestors
of ℓ2 special. We split the path from the root to ℓ2 into shorter paths of length l. For every shorter path
we construct a binary characteristic vector where we set entry to be equal to 1 if the corresponding node is
marked. Then we convolve each characteristic vector with binary vector D′ .
• Leaf ℓ3 has c = 6 marked ancestors that are not special and two marked ancestors that are special. We are in
c = 6 > m0.5−3α case and we make ℓ3 and all marked ancestors of ℓ3 special. Then we split the path from the
root to ℓ3 into shorter paths of length l and proceed similarly as when processing leaf ℓ2 .
• Leaf ℓ4 has c = 4 marked ancestors. We are in c ≤ m0.5−3α case and we do not make any of the marked
ancestors special.

27

<!-- page 29 -->

be the ancestor of u which is marked and special and whose depth d is the largest. Let c be the
number of marked ancestors of u that are marked but not special. By the preprocessing phase,
c ≤ m0.5−3α . For every such marked ancestor which is not special we want to determine whether
the corresponding entry of D ′ is equal to 1. This takes at most O(m0.5−3α ) total time. If we found
such an entry of D ′ equal to 1, we set D ′ [i] = 1 and move to determining the value of D ′ [i − 1].
By the choice of the node a, there are at most l marked ancestors of u that are special and are of
depth more than d′ := ⌈d/l⌉l. We can determine whether the corresponding entry of D ′ is equal to
1 for any of those nodes and set D ′ [i] = 1 if this happens. This takes at most O(l) total time. It
remains to consider the marked ancestors of u of depth at most d′ . For this we use the convolutions
that we performed in the preprocessing step. Let u0 , u1 , . . . , ud′ = a be the nodes on the path from
the root of T to l. The node u0 is the root of T . Let r1 , . . . , rd′ /l be the binary vectors of length l
constructed as follows. For k = 1, . . . , d′ /l and j = 1, . . . , l we set the j-th entry rk [j] of the vector
rk to be equal to rk [j] = 1 if the node u(k−1)l+j is marked and equal to rk [j] = 0 if the node is not
marked. We want to determine whether there is a marked node corresponding to an entry equal
to 1 in rk such that the corresponding entry in D ′ is equal to 1. We can determine whether this
is the case in O(1) because this we can check whether the corresponding entry of ck is equal to 0
or at least 1. If the entry is at least 1, we set D ′ [i] to be equal to 1 and continue with D ′ [i − 1].
We spend O(1) for each k = 1, . . . , d′ /l. The whole process takes O(d′ /l) time. The total runtime
spent on computing the value of D ′ [i] is bounded by O(m0.5−3α ) + O(l) + O(d′ /l) ≤ O(m0.5−α ).
The runtime of the algorithm We have seen that the runtime of the algorithm is bounded
 by
Õ(n · m0.5−α ) for any constant 0 < α ≤ 1/18. We obtain the required runtime Õ n · m0.5−1/18 by
setting α = 1/18.

5.2

Algorithm for type “·+”

Theorem 14. Let p be a regular expression of type “·+” and t be a text. In time O(|p| + |t|) we
can reduce the pattern matching problem on p and t to one instance of Subset Matching and one
instance of Wildcard Matching.
Proof. Below we reduce this regular expression pattern matching problem to Subset Matching and
to Wildcard Matching. Then we combine the two outputs and solve the initial regexp problem.
Reduction to Subset Matching We partition the pattern p into substrings of maximal length
such that all symbols in each substring are equal, i.e., each substrings is a concatenation of copies
of a+ or a. Consider one particular substring and suppose that it contains l copies of a or a+ .
We replace this substring with a set {(a, 1), (a, 2), . . . , (a, l)}. We perform this operation for every substring of maximal length and concatenate the resulting sets to obtain the pattern p′ for
Subset Matching. Similarly, we partition the text t into substrings of maximal length such that
all symbols in each substring are equal. As before, every such substring is replaced with a set
{(a, 1), (a, 2), . . . , (a, l)}. This yields a text t′ .
Now we run Subset Matching algorithm on the pattern p′ and the text t′ . For each position i
of t′ , we find whether p′ matches the substring of t′ that starts at i.
Reduction to Wildcard Matching Similarly as before, we partition pattern p into substrings
of maximal length such that all symbols in each substring are equal, i.e., each substrings is a
concatenation of copies of a+ or a. There are two kinds of substrings. If the substring contains
only l symbols a (i.e., there are no a+ s), we replace this substring with the symbol (a, l). If the
substring has at least one a+ , we replace it with a wildcard. We concatenate all symbols and
28

<!-- page 30 -->

wildcards into one regular expression p′′ . If p′′ starts with a symbol, we replace it with a wildcard.
Also, if p′′ ends with a symbol, we replace it with a wildcard. Similarly, we partition the text t into
substrings of maximal length such that all symbols in each substring are equal. We replace each
such substring, say al , with a symbol (a, l). The symbols are concatenated into the text t′′ .
Now we run Wildcard Matching algorithm on the pattern p′′ and the text t′′ . For each position
i of t′′ , we find whether p′′ matches the substring of t′′ that starts at i.
Combining the results A substring of t can be derived from p iff there is a position i such
that two conditions hold: p′ matches a substring of t′ starting at the position i and p′′ matches
a substring of t′′ starting at position i. The first condition (coming from the Subset Matching
instance) ensures that, if we match p to a substring of t, the number of appropriate symbols in
t is at least as large as in p. The second condition (coming from Wildcard Matching) ensures
that, if p contains a maximal substring consisting only of symbols a (no a+ s) for some a, then the
corresponding position in t contains exactly the same number of symbols a. Notice that we do not
need to satisfy this condition if p starts or ends with a maximal substring consisting only of as,
which is why we start and end p′′ with wildcards.

5.3

Algorithm for types “|∗·” and “|+·”

Theorem 15. Let p be a regular expression of type “|∗·” or “|+·” and t be a text. In time O(|p|+|t|)
we can decide if t can be derived from p.
Proof. Let t′ := tt be the sequence t repeated twice. By [Gus97] (page 196), we can use the suffix
tree and the lowest common ancestor data structures to preprocess the sequences t′ and t in O(|t|)
time such that the following holds. For any i = 1, . . . , |t|, we can in O(1) decide if the substring
t′i t′i+1 . . . t′i+|t|−1 of the sequence t′ of length |t| is equal to the sequence t.
Let p′ be an arbitrary sequence of non-zero length. We notice that the sequence t can be
derived from [p′ ]∗ (or, equivalently, from [p′ ]+ if |t| > 0) iff p′ is a prefix of t and the substring
t′|p′ |+1 t′|p′ |+2 . . . t′|p′ |+|t| is equal to t, and |t| is divisible by |p′ |. We can check the first condition in
time O(|p′ |) and the second condition in time O(1) assuming that we did the preprocessing step.
Let p = [p1 ]∗ | [p2 ]∗ | . . . | [pk ]∗ be the input pattern to the membership problem for some
integer k ≥ 1. We do the preprocessing step on t′ and t which takes O(|t|). For every pi , k ≥ i ≥ 1
we decide in O(|pi |) time if t can be derived from [pi ]∗ . This yields the required runtime.
The algorithm for the case when p = [p1 ]+ | [p2 ]+ | . . . | [pk ]+ is the same except we can’t
derive text t if |t| = 0.

5.4

Algorithms for types “|·+” and “∗·+”

Theorem 16. Let p be a regular expression of type “|·+” or “∗·+” and t be a text. In time
O(|p| + |t|) we can decide if t can be derived from p.
Proof. We do the run-length encoding of t defined as follows. Set t′ := t and initialize r to be an
empty ordered sequence of tuples. While |t′ | > 0, let a be the first symbol of t′ and let l > 0 be the
largest integer such that al (symbol a repeated l times) is a prefix of t′ . Remove the prefix al from
t′ and add tuple (a, l) at the end of the ordered sequence r, and repeat. This takes O(|t|) time in
total. Let |r| be the number of tuples in r.

29

<!-- page 31 -->

Type “|·+” Let p = p1 | . . . | pk for some integer k > 0, where each pi is a concatenation of a
and a+ for various symbols a. We want to decide if there exists an integer i = 1, . . . , k such that the
text t can be derived from the expression pi . Fix an arbitrary i = 1, . . . , k. We do the run-length
encoding of the expression pi and produce a sequence of tuples r(pi ) defined as follows. Set p′i := pi
and r(pi ) to be an empty sequence of tuples. While |p′i | > 0, choose the largest integer l > 0 such
that there exists a prefix of p′i of form aa+ a+ aa . . . (an arbitrary concatenation of a and a+ ) for
some symbol a. Let l′ ≥ 0 be such that the prefix of p′i has l′ occurrences of a and l − l′ occurrences
of a+ . If l′ = l, we add tuple (a, = l) to the end of the sequence of tuples r(pi ). Otherwise, if l′ < l,
we add tuple (a, ≥ l) to the end of the sequence r(pi ). We delete the prefix of p′i and repeat (until
|p′i | = 0). Let |r(pi )| be the number of tuples in r(pi ). We can derive t from the expression pi iff
the following two conditions hold:
• |r| = |r(pi )|.
• For all j = 1, . . . , |r|, if rj = (a, l) (the j-th tuple of r is (a, l)), then r(pi )j = (a, = l) or
r(pi )j = (a, ≥ l′ ) for some integer l′ ≤ l.
For every i = 1, . . . , k this takes O(|pi |) time and the required upper bound on the runtime follows.

Type “∗·+” We have to consider the case when p = [p′ ]∗ for some p′ . We do the run-length
encoding on the sequence p′ and get the sequence of tuples r(p′ ) (as described in the case for type
“|·+”). For every tuple (a, = l) or (a, ≥ l), let a be it’s type. Consider two subcases.
The first and the last tuple of r(p′ ) are of different types We can derive the text t from
the expression p iff the following two conditions
• |r| is divisible by |r(p′ )|.
• Let r ′ = (r(p′ ), r(p′ ), . . . , r(p′ )), where r(p′ ) is repeated |r|/|r(p′ )| time in the r.h.s.. For all
j = 1, . . . , |r|, if rj = (a, l), then rj′ = (a, = l) or rj′ = (a, ≥ l′ ) for some integer l′ ≤ l.
The first and the last tuple of r(p′ ) are of the same type If |r(p′ )| = 1 we can check if t
can be derived from p easily. If there is no integer k ≥ 1 such that |r| = k|r(p′ )| − (k − 1), then
t can’t be derived from p. Otherwise, let r ′ be r(p′ ) except the last tuple. Let r ′′ be r except we
change the first tuple of r. Let z be the first and the last tuple of r merged (in the natural way).
We replace the first tuple of r by z and get r ′′ . We define r ′′′ = (r ′ , r ′′ , r ′′ , r ′′ , . . . , r ′′ ), where r ′′ is
repeated k − 1 times. Furthermore, we add the last tuple of r(p′ ) at the end of r ′′′ . The text t
can be derived from the expression p iff for all j = 1, . . . , |r|, if rj = (a, l), then rj′′′ = (a, = l) or
rj′′′ = (a, ≥ l′ ) for some integer l′ ≤ l.
In the both subcases the runtime is upper bounded by O(|p| + |t|).

Acknowledgments
We thank Ludwig Schmidt and the reviewers for many helpful comments. This work was supported
by grants from the NSF, the MADALGO center, and the Simons Investigator award.

References
[ABVW15]

Amir Abboud, Arturs Backurs, and Virginia Vassilevska Williams. Tight Hardness
Results for LCS and other Sequence Similarity Measures. FOCS, 2015.

30

<!-- page 32 -->

[AC75]

Alfred V Aho and Margaret J Corasick. Efficient string matching: an aid to bibliographic search. Communications of the ACM, 18(6):333–340, 1975.

[AKL+ 15]

Amihood Amir, Tsvi Kopelowitz, Avivit Levy, Seth Pettie, Ely Porat, and B Riva
Shalom. Mind the gap. arXiv preprint arXiv:1503.07563, 2015.

[AWW14]

Amir Abboud, V. Vassilevska Williams, and Oren Weimann. Consequences of faster
sequence alignment. ICALP, 2014.

[BI15]

Arturs Backurs and Piotr Indyk. Edit Distance Cannot Be Computed in Strongly
Subquadratic Time (unless SETH is false). STOC, 2015.

[BK15]

Karl Bringmann and Marvin Künnemann. Quadratic conditional lower bounds for
string problems and dynamic time warping. In Foundations of Computer Science
(FOCS), 2015 IEEE 56th Annual Symposium on, pages 79–97. IEEE, 2015.

[Bri14]

Karl Bringmann. Why walking the dog takes time: Frechet distance has no strongly
subquadratic algorithms unless SETH fails. FOCS, 2014.

[BT09]

Philip Bille and Mikkel Thorup. Faster regular expression matching. In Automata,
Languages and Programming, pages 171–182. Springer, 2009.

[CH97]

Richard Cole and Ramesh Hariharan. Tree pattern matching and subset matching
in randomized O(n log3 m) time. In Proceedings of the twenty-ninth annual ACM
symposium on Theory of computing, pages 66–75. ACM, 1997.

[CH02]

Richard Cole and Ramesh Hariharan. Verifying candidate matches in sparse and
wildcard matching. In Proceedings of the Thirty-fourth annual ACM symposium on
Theory of computing, pages 592–601. ACM, 2002.

[FP74]

Michael J Fischer and Michael S Paterson. String-Matching and Other Products.
Technical report, DTIC Document, 1974.

[Gal85]

Zvi Galil. Open problems in stringology. In Combinatorial Algorithms on Words,
pages 1–8. Springer, 1985.

[GRS99]

Minos N Garofalakis, Rajeev Rastogi, and Kyuseok Shim. SPIRIT: Sequential
pattern mining with regular expression constraints. In VLDB, volume 99, pages
7–10, 1999.

[Gus97]

Dan Gusfield. Algorithms on strings, trees and sequences: computer science and
computational biology. Cambridge university press, 1997.

[Ind98]

Piotr Indyk. Faster algorithms for string matching problems: Matching the convolution bound. In Foundations of Computer Science, 1998. Proceedings. 39th Annual
Symposium on, pages 166–173. IEEE, 1998.

[IP01]

Russell Impagliazzo and Ramamohan Paturi. On the Complexity of k-SAT. Journal
of Computer and System Sciences, 62(2):367–375, 2001.

31

<!-- page 33 -->

[Kal02]

Adam Kalai. Efficient pattern-matching with don’t cares. In SODA, volume 2,
pages 655–656, 2002.

[KDY+ 06]

Sailesh Kumar, Sarang Dharmapurikar, Fang Yu, Patrick Crowley, and Jonathan
Turner. Algorithms to accelerate multiple regular expressions matching for deep
packet inspection. ACM SIGCOMM Computer Communication Review, 36(4):339–
350, 2006.

[KHDA12]

Kenrick Kin, Björn Hartmann, Tony DeRose, and Maneesh Agrawala. Proton: multitouch gestures as regular expressions. In Proceedings of the SIGCHI Conference
on Human Factors in Computing Systems, pages 2885–2894. ACM, 2012.

[KMP77]

Donald E Knuth, James H Morris, Jr, and Vaughan R Pratt. Fast pattern matching
in strings. SIAM journal on computing, 6(2):323–350, 1977.

[Lee]

LeetCode. Problem 139. Word Break. https://leetcode.com/problems/word-break/.

[Mye92]

Gene Myers. A four russians algorithm for regular expression pattern matching.
Journal of the ACM (JACM), 39(2):432–448, 1992.

[NR03]

Gonzalo Navarro and Mathieu Raffinot. Fast and simple character classes and
bounded gaps pattern matching, with applications to protein searching. Journal of
Computational Biology, 10(6):903–923, 2003.

[Tun11]

Daniel
Tunkelang.
Retiring
a
Great
Interview
Problem.
http://thenoisychannel.com/2011/08/08/retiring-a-great-interview-problem, 2011.

[Wil05]

Ryan Williams. A new algorithm for optimal 2-constraint satisfaction and its implications. Theoretical Computer Science, 348(2):357–365, 2005.

[WvdMBW16] Nicolaas Weideman, Brink van der Merwe, Martin Berglund, and Bruce Watson.
Analyzing matching time behavior of backtracking regular expression matchers by
using ambiguity of nfa. In International Conference on Implementation and Application of Automata, pages 322–334. Springer, 2016.

32
