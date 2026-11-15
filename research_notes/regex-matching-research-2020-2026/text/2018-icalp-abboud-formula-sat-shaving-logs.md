---
file: ../pdfs/2018-icalp-abboud-formula-sat-shaving-logs.pdf
citation: "Tighter Connections Between Formula-SAT and Shaving Logs — Abboud, Bringmann — ICALP 2018"
doi: 
source: "https://arxiv.org/pdf/1804.08978"
topics: [counting_and_derivatives]
pages: 36
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Tighter Connections Between Formula-SAT and Shaving Logs∗

arXiv:1804.08978v2 [cs.CC] 30 Apr 2018

Amir Abboud†

Karl Bringmann‡

Abstract
A noticeable fraction of Algorithms papers in the last few decades improve the running time
of well-known algorithms for fundamental problems by logarithmic factors. For example, the
O(n2 ) dynamic programming solution to the Longest Common Subsequence problem (LCS) was
improved to O(n2 / log2 n) in several ways and using a variety of ingenious tricks. This line of
research, also known as the art of shaving log factors, lacks a tool for proving negative results.
Specifically, how can we show that it is unlikely that LCS can be solved in time O(n2 / log3 n)?
Perhaps the only approach for such results was suggested in a recent paper of Abboud,
Hansen, Vassilevska W. and Williams (STOC’16). The authors blame the hardness of shaving
logs on the hardness of solving satisfiability on boolean formulas (Formula-SAT) faster than
exhaustive search. They show that an O(n2 / log1000 n) algorithm for LCS would imply a major
advance in circuit lower bounds. Whether this approach can lead to tighter barriers was unclear.
In this paper, we push this approach to its limit and, in particular, prove that a well-known
barrier from complexity theory stands in the way for shaving five additional log factors for
fundamental combinatorial problems. For LCS, regular expression pattern matching, as well as
the Fréchet distance problem from Computational Geometry, we show that an O(n2 / log7+ε n)
runtime would imply new Formula-SAT algorithms.
Our main result is a reduction from SAT on formulas of size s over n variables to LCS on
sequences of length N = 2n/2 · s1+o(1) . Our reduction is essentially as efficient as possible, and it
greatly improves the previously known reduction for LCS with N = 2n/2 · sc , for some c ≥ 100.

1

Introduction

Since the early days of Algorithms research, a noticeable fraction of papers each year shave log
factors for fundamental problems: they reduce the best known upper bound on the time complexity
from T (n) to T (n)/ logc n, for some c > 0. While in some cases a cynic would call such results
“hacks” and “bit tricks”, there is no doubt that they often involve ingenious algorithmic ideas and
suggest fundamental new ways to look at the problem at hand. In his survey, Timothy Chan calls
this kind of research “The Art of Shaving Logs” [37]. In many cases, we witness a race of shaving
logs for some problem, in which a new upper bound is found every few months, without giving any
hints on when this race is going to halt. For example, in the last few years, the upper bound for
combinatorial Boolean Matrix Multiplication dropped from O(n3 / log2 n) [16], to O(n3 / log2.25 n)
∗
Part of the work was performed while visiting the Simons Institute for the Theory of Computing, Berkeley, CA.
The work was completed when A.A. was at Stanford University and was supported by Virginia Vassilevska Williams’
NSF Grants CCF-1417238 and CCF-1514339, and BSF Grant BSF:2012338.
†
IBM Almaden Research Center, abboud@cs.stanford.edu
‡
Max Planck Institute for Informatics, Saarland Informatics Campus, Germany, kbringma@mpi-inf.mpg.de

1

<!-- page 2 -->

[20], to O(n3 / log3 n) [38], and most recently to O(n3 / log4 n) [102]. Perhaps the single most
important missing technology for this kind of research is a tool for proving lower bounds.
Consider the problem of computing the Longest Common Subsequence (LCS) of two strings
of length n. LCS has a simple O(n2 ) time dynamic programming algorithm [95, 47]. Several
approaches have been utilized in order to shave log factors such as the “Four Russians” technique
[16, 63, 76, 23, 60], utilizing bit-parallelism [10, 48, 64], and working with compressed strings
[49, 56]. The best known upper bounds are O(n2 / log2 n) for constant size alphabets [76], and
O(n2 log log n/ log2 n) for large alphabets [60]. But can we do better? Can we solve LCS in
O(n2 / log3 n) time? While the mathematical intrigue is obvious, we remark that even such mild
speedups for LCS could be significant in practice. Besides its use as the diff operation in unix, LCS
is at the core of highly impactful similarity measures between biological data. A heuristic algorithm
called BLAST for a generalized version of LCS (namely, the Local Alignment problem [89]) has been
cited more than sixty thousand times [14]. While such heurisitics are much faster than the nearquadratic time algorithms above, they are not guaranteed to return an optimal solution and are thus
useless in many applications, and biologists often fall back to (highly optimized implementations
of) the quadratic solutions, see, e.g. [73, 74].
How would one show that it is hard to shave logs for some problem? A successful line of work,
inspired by NP-hardness, utilizes “fine-grained reductions” to prove statements of the form: a small
improvement over the known runtime for problem A implies a breakthrough algorithm for problem
B, refuting a plausible hypothesis about the complexity of B. For example, it has been shown that if
LCS can be solved in O(n2−ε ) time, where ε > 0, then there is a breakthrough (2−δ)n algorithm for
CNF-SAT, and the Strong Exponential Time Hypothesis (SETH, defined below) is refuted [2, 29].
Another conjecture that has been used to derive interesting lower bounds states that the 3-SUM
problem1 cannot be solved in O(n2−ε ) time. It is natural to ask: can we use these conjectures to
rule out log-factor improvements for problems like LCS? And even more optimistically, one might
hope to base the hardness of LCS on a more standard assumption like P 6= NP. Unfortunately, we
can formally prove that these assumptions are not sharp enough to lead to any consequences for
log-factor improvements, if only Turing reductions are used. In Section 3 we prove the following
theorem which also shows that an O(f (n)/ logc (f (n))) time algorithm for problem A cannot imply,
via a fine-grained reduction, an O(g(n)1−ε ) algorithm for problem B, unless B is (unconditionally)
solvable in O(g(n)1−δ ) time.
Theorem 1.1 (Informally). If for some c > 0 there is a fine-grained reduction proving that LCS
is not in O(n2 / logc n) time unless SETH fails, then SETH is false.
Note that it also does not suffice to simply make SETH stronger by postulating a higher running time lower bound for CNF-SAT, since superpolynomial improvements are known for this problem [83, 34, 50, 8]. Similarly, we cannot base a study of log-factor improvements on the APSP
conjecture, since superlogarithmic improvements are known for APSP [99]. (However, 3SUM
could be a candidate to base higher lower bounds on, since only log-factor improvements are
known [61, 55, 58, 21], see Section A for a discussion.)
Thus, in a time when super-linear lower bounds for problems like LCS are far out of reach,
and our only viable approach to obtaining such negative results is reductions-based, we are left
with two options. We could either leave the study of log-factor improvements in limbo, without a
1
3-SUM asks, given a list of n numbers, to find three that sum to zero. The best known upper bound is
O((n2 / log 2 n)(log log n)O(1) ) for real numbers [61, 55, 58, 39] and O(n2 (log log n/ log n)2 ) for integers [21].

2

<!-- page 3 -->

technology for proving negative results, or we could search for natural and convincing assumptions
that are more fine-grained than SETH that could serve as the basis for the negative results we desire.
Such assumptions were recently proposed by Abboud, Hansen, Vassilevska Williams and Williams
[3]. The authors blame the hardness of shaving logs on the hardness of solving satisfiability on
boolean formulas (Formula-SAT) faster than exhaustive search2 , by polynomial factors (which are
log-factors in the runtime), a task for which there are well known “circuit lower bound” barriers
in complexity theory. They show that an O(n2 / log1000 n) algorithm for LCS would imply a major
advance in circuit lower bounds. In the final section of this paper, we give a more detailed argument
in favor of this approach. Whether one should expect it to lead to tight barriers, i.e. explaining the
lack of O(n2 / log3 n) algorithms for LCS or any other natural problem, was completely unclear.
The Machine Model We use the Word-RAM model on words of size Θ(log n), where there is a
set of operations on words that can be performed in time O(1). Most papers do not fix the concrete
set of allowed operations, and instead refer to “typical Boolean and arithmetic operations”. In
this paper, we choose a set of operations P that is robust with respect to changing the word size:
For any operation ◦ ∈ P, given two words a, b (of size Θ(log n)) we can compute a ◦ b in time
(log n)1+o(1) on a Word RAM with word size Θ(log log n) and operation set P. In other words, if
we split a, b into Θ(log n/ log log n) words of size Θ(log log n) then a ◦ b can still be computed very
efficiently.
This robustness in particular holds for the following standard set of operations: initializing a cell
with a constant, bitwise AND, OR, NOT, shift, addition, subtraction, multiplication, and division
with remainder (since multiplication and division have near-linear time algorithms).
The results in this paper will get gradually weaker as we relax the restriction on near-linear time
per operation to higher runtimes, however, even with this restriction, to the best of our knowledge
this model captures all log shaving results in the literature (on the “standard” Word RAM model
without fancy word operations).
Formula-SAT A boolean formula over n input variables can be viewed as a tree in which every
leaf is marked by an input variable or its negation and every internal node or gate represents some
basic boolean operation. Throughout this introduction we will only talk about deMorgan formulas,
in which every gate is from the set {∧, ∨}. The size of the formula is defined to be the number of
leaves in the tree.
In the Formula-SAT problem we are given a formula F of size s over n inputs, and we have to
decide whether there is an input {0, 1}n that makes it output 1. A naive algorithm takes O(2n · s)
time, since evaluating the formula on some input takes O(s) time. Can we do better? We will call
n
a SAT algorithm non-trivial 3 if it has a runtime at most O( 2nε ), for some ε > 0.
It seems like a clever algorithm must look at the given formula F and try to gain a speedup by
analyzing it. The more complicated F can be, the harder the problem becomes. Indeed, Dantsin
and Hirsch [50] survey dozens of algorithms for SAT on CNF formulas which exploit their structure.
For k-CNF formulas of size s there are 2n s/2Ω(n/k) time algorithms (e.g. [83]), and for general CNF
2

In [3] the authors focus on SAT on Branching Programs (BPs) rather than formulas, but due to standard
transformations between BPs and formulas, the two problems are equivalent up to polynomial factors. Focusing on
Formula-SAT will be crucial to the progress we make in this paper.
3
Some works on SAT algorithms used this term for runtimes of the form 2n poly(s)/nω(1) . In our context, we need
to be a bit more fine-grained.

3

<!-- page 4 -->

formulas the bound is 2n s/2Ω(n/ log ∆) where ∆ = s/n is the clause-to-variable ratio [34, 50, 8].
The popular SETH [68, 35] essentially says that this is close to optimal, and that there is no
2n s/2Ω(n) algorithm for CNF-SAT. For arbitrary deMorgan formulas, the upper bounds are much
worse. A FOCS’10 paper by Santhanam [86] and several recent improvements [42, 44, 43, 72, 93]
ε
solve Formula-SAT on formulas of size s = n3−16ε in time 2n sO(1) /2n , which is non-trivial only
for s = o(n3 ), and going beyond cubic seems extremely difficult. This leads us to the first barrier
which we will transform into a barrier for shaving logs.
Hypothesis 1.2. There is no algorithm that can solve SAT on deMorgan formulas of size s =
n
n3+Ω(1) in O( 2nε ) time, for some ε > 0, in the Word-RAM model.
Perhaps the main reason to believe this hypothesis is that despite extensive algorithmic attacks
on variants of SAT (perhaps the most extensively studied problem in computer science) over decades,
none of the ideas that anyone has ever come up with seem sufficient to refute it. Recent years have
been particularly productive in non-trivial algorithms designed for special cases of Circuit-SAT
[86, 88, 66, 35, 101, 22, 41, 69, 65, 45, 85, 59] (in addition to the algorithms for deMorgan formulas
above) and this hypothesis still stands.
A well-known “circuit lower bounds” barrier seems to be in the way for refuting Hypothesis 1.2:
can we find an explicit boolean function that cannot be computed by deMorgan formulas of cubic
size? Functions that require formulas of size Ω(n1.5 ) [91] and Ω(n2 ) [71] have been known since
the 60’s and 70’s, respectively. In the late 80’s, Andreev [15] proved an Ω(n2.5 ) which was later
gradually improved to Ω(n2.55 ) by Nisan and Wigderson [67] and to Ω(n2.63 ) by Paterson and Zwick
[81] until Håstad proved his n3−o(1) lower bound in FOCS’93 [62] (a recent result by Tal improves
the no(1) term [92]). All these lower bound results use the “random restrictions” technique, first
introduced in this context by Subbotovskaya in 1961 [91], and it is known that a substantially
different approach must be taken in order to go beyond the cubic barrier. What does this have
to do with Formula-SAT algorithms? Interestingly, this same “random restrictions” technique was
crucial to all the non-trivial Formula-SAT algorithms mentioned above. This is not a coincidence,
but only one out of the many examples of the intimate connection between the task of designing
non-trivial algorithms for SAT on a certain class F of formulas or circuits and the task of proving
lower bounds against F. This connection is highlighted in many recent works and in several surveys
[87, 80, 98]. The intuition is that both of these tasks seem to require identifying a strong structural
property of functions in F. There is even a formal connection shown by Williams [97], which in our
context implies that solving Formula-SAT on formulas of size O(n3.1 ) in O(2n /n10 ) time (which is
only slightly stronger than refuting Hypothesis 1.2) is sufficient in order to prove that there is a
function in the class ENP that cannot be computed by formulas of size O(n3.1 ) (see [3] for more
details). This consequence would be the first polynomial progress on the fundamental question of
worst case formula lower bounds since Håstad’s result.

1.1

Our Results: New Reductions

Many recent papers have reduced CNF-SAT to fundamental problems in P to prove SETH-based
lower bounds (e.g. [82, 84, 6, 4, 27, 18, 7, 1, 33, 5, 19, 77, 40]). Abboud et al. [3] show that even
SAT on formulas, circuits, and more, can be efficiently reduced to combinatorial problems in P. In
particular, they show that Formula-SAT on formulas of size s over n inputs can be reduced to an
instance of LCS on sequences of length N = O(2n/2 · s1000 ). This acts as a barrier for shaving logs
4

<!-- page 5 -->

as follows. A hypothetical O(N 2 / logc N ) time algorithm for LCS can be turned into an
n1+o(1) · (2n/2 · s1000 )2 /(log 2Ω(n) )c = O(2n · s2000 /nc−1 )
time algorithm for Formula-SAT, which for a large enough c ≥ 2001 would refute Hypothesis 1.2.
The first n1+o(1) factor in the runtime comes from the jump from n to N = 2n and our WordRAM machine model: whenever the LCS algorithm wants to perform a unit-cost operation on
words of size Θ(log N ) (this is much more than the word size of our SAT algorithm which is only
Θ(log n) = Θ(log log N )), the SAT algorithm can simulate it in (log N )1+o(1) = n1+o(1) time in the
Word-RAM model with words of size Θ(log n).
Our main result is a much more efficient reduction to LCS. For large but constant size alphabets,
we get a near-linear dependence on the formula size, reducing the s1000 factor to just s1+o(1) .
Theorem 1.3. Formula-SAT on formulas of size s on n inputs can be reduced to an instance of
LCS on two sequences over an alphabet of size σ of length N = 2n/2 · s1+O(1/ log log σ) , in O(N ) time.
Thus, if LCS on sequences of length N and alphabet of size ω(1) can be solved in O(N 2 / logc N )
2+o(1)
time, then Formula-SAT can be solved in 2n · s nc · n1+o(1) time. Recall that the known upper
bound for LCS is O(n2 / logc n) for any constant alphabet size, with c = 2, and we can now report
that the barrier of cubic formulas stands in the way of improving it to c > 7 (see Corollary 1.6
below).
The novelty in the proof of Theorem 1.3 over [3] is discussed in Section 2. As an alternative
to Theorem 1.3, in Section D we present another reduction to LCS which is much simpler than all
previously known reductions, but uses a larger alphabet.

Fréchet Distance An important primitive in computational geometry is to judge how similar
are two basic geometric objects, such as polygonal curves, represented as sequences of points in
d-dimensional Euclidean space. Such curves are ubiquitous, since they arise naturally as trajectory
data of moving objects, or as time-series data of stock prices and other measures. The most
popular similarity measure for curves in computational geometry is the Fréchet distance, also
known as dog-leash-distance. For formal definitions see Section F. The Fréchet distance has found
many applications (see, e.g., [78, 26, 30]) and developed to a rich field of research with many
generalizations and variants (see, e.g., [11, 17, 13, 53, 36, 46, 32, 52, 75, 70]).
This distance measure comes in two variants: the continuous and the discrete. A classic algorithm by Alt and Godau [12, 57] computes the continuous Fréchet distance in time O(n2 log n) for
two given curves with n vertices. The fastest known algorithm runs in time O(n2 (log log n)2 ) (on
the Word RAM) [31]. If we only want to decide whether the Fréchet distance is at most a given
value δ, this algorithm runs in time O(n2 (log log n)2 / log n). For the discrete Fréchet distance, the
original algorithm has running time O(n2 ) [54], which was improved to O(n2 log log n/ log n) by
Agarwal et al. [9]. Their algorithm runs in time O(n2 log log n/ log2 n) for the decision version. It
is known that both versions of the Fréchet distance are SETH-hard [27]. However, this does not
rule out log factor improvements. In particular, no reduction from versions of SETH on formulas
or branching programs is known.
In this paper we focus on the decision version of the discrete Fréchet distance (which we simply
call “Fréchet distance” from now on). We show that Fréchet distance suffers from the same barriers
for shaving logs like LCS. In particular, this reduction allows us to base the usual Ω(n2−ε ) lower
bound on a weaker assumption than SETH, such as NC-SETH (see the discussion in [3]). This is
the first NC-SETH hardness for a problem that does not admit alignment gadgets (as in [29]).
5

<!-- page 6 -->

Theorem 1.4. Formula-SAT on formulas of size s on n inputs can be reduced to an instance of
the Fréchet distance on two curves of length N = O(2n/2 · s), in O(N ) time.
Regular Expression Pattern Matching Our final example is the fundamental Regular Expression Pattern Matching problem: Decide whether a given regular expression of length m matches a
substring of a text of length n. Again, there is a classical O(nm) algorithm [94], and the applicability and interest in this problem resulted in algorithms shaving log factors; the first one by Myers
[79] was improved by Bille and Thorup [24] to time O(mn/ log1.5 n). Recently, Backurs and Indyk
proved an n2−o(1) SETH lower bound [19], and performed an impressive study of the exact time
complexity of the problem with respect to the complexity of the regular expression. This study was
essentially completed by Bringmann, Grønlund, and Larsen [28], up to no(1) factors. In Section E
we show that this problem is also capable of efficiently simulating formulas and thus has the same
barriers as LCS and Fréchet distance.
Theorem 1.5. Formula-SAT on formulas of size s on n inputs can be reduced to an instance of
Regular Expression Pattern Matching on text and pattern of length N = O(2n/2 · s log s) over a
constant size alphabet, in O(N ) time.
Consequences of the Cubic Formula Barrier We believe that SAT on formulas can be tightly
connected to many other natural problems in P. As we discuss in the next section, such reductions
seem to require problem-specific engineering and are left for future work. The main point of
this paper is to demonstrate the possibility of basing such ultra fine-grained lower bounds on one
common barrier. Our conditional lower bounds are summarized in the following corollary, which
shows that current log-shaving algorithms are very close to the well-known barrier from complexity
theory of cubic formula lower bounds.
Corollary 1.6. For all ε > 0, solving any of the following problems in O(n2 / log7+ε n) time refutes
Hypothesis 1.2, and solving them in O(n2 / log17+ε n) time implies that ENP cannot be computed by
non-uniform formulas of cubic size:
• LCS over alphabets of size ω(1)
• The Fréchet distance on two curves in the plane
• Regular Expression Pattern Matching over constant size alphabets.
The main reason that our lower bounds above are not tight (the gap between 2 and 7) is that
we need to start from SAT on cubic size formulas rather than linear size ones, due to the fact
that clever algorithms do exist for smaller formulas. We remark that throughout the paper we will
work with a class of formulas we call F1 (see Section B), also known as bipartite formulas, that
are more powerful than deMorgan formulas yet our reduction to LCS can support them as well.
This makes our results stronger, since F1 -Formula-SAT could be a harder problem than SAT on
deMorgan formulas. In fact, in an earlier version of the paper we had suggested the hypothesis that
F1 -Formula-SAT does not have non-trivial algorithms even on linear size formulas. This stronger
hypothesis would give higher lower bounds. However, Avishay Tal (personal communication) told
us about such a non-trivial algorithm for formulas of size up to n2−Ω(1) using tools from quantum
query complexity. We are optimistic that one could borrow such ideas or the “random restrictions”
technique from SAT algorithms in order to shave more logs for combinatorial problems such as
LCS. This is an intriguing direction for future work.
6

<!-- page 7 -->

2

Technical Overview and the Reduction to LCS

All the reductions from SAT to problems in P mentioned above start with a split-and-list reduction
to some “pair finding” problem. In the SETH lower bounds, CNF-SAT is reduced to the OrthogonalVectors problem of finding a pair a ∈ A, b ∈ B, A, B ⊆ {0, 1}d that are orthogonal [96]. When
starting from Formula-SAT, we get a more complex pair-finding problem. In Section B we show a
simple reduction from SAT on formulas from the class F1 (which contains deMorgan formulas) to
the following problem.
Definition 2.1 (Formula-Pair Problem). Given a deMorgan formula over 2m variables F =
F (x1 , . . . , xm , y1 , . . . , ym ) (each appearing once in F ), and two sets of vectors A, B ⊆ {0, 1}m of
size n, decide if there is a pair a ∈ A, b ∈ B such that F (a1 , . . . , am , b1 , . . . , bm ) = true.
In Section B we show a Four-Russians type algorithm that solves Formula-Pair in O(n2 m/ log2 n)
time, and even when m = |F | = (log n)1+o(1) no O(n2 / log1+ε n) upper bound is known. By our
reduction, such an upper bound would imply a non-trivial algorithm for SAT on formulas from F1 .
Moreover, Hypothesis 1.2 implies that we cannot solve Formula-Pair in O(n2 / logε n) time, for
m = (log n)3+Ω(1) . In the next sections, we reduce Formula-Pair to LCS, from which Theorem 1.3
follows. A simpler reduction using much larger alphabet size can be found in Section D.
Theorem 2.2. Formula-Pair on formulas of size s and lists of size n can be reduced to an
instance of LCS on two strings over alphabet of size σ ≥ 2 of length O(n · s1+O(1/ log log σ) ), in linear
time.
The reduction constructs strings x, y and a number ρ such that LCS(x, y) ≥ ρ holds if and
only if the given Formula-Pair instance (F, A, B) is satisfiable. The approach is similar to the
reductions from Orthogonal-Vectors to sequence alignment problems (e.g. [6, 27, 18, 2, 29]). The
big difference is that our formula F can be much more complicated than a CNF, and so we will need
more powerful gadgets. Sequence gadgets that are able to simulate the evaluation of deMorgan
formulas were (implicitly) constructed in [3] with a recursive approach. Our main contribution is
an extremely efficient implementation of such gadgets with LCS.
The main part of the reduction is to construct gate gadgets: for any vectors a, b ∈ {0, 1}m
and any gate g of F , we construct strings x(g, a) and y(g, b) whose LCS determines whether gate
g evaluates to true for input (a, b) to F (see Section 2.1). Once we have this, to find a pair of
vectors a ∈ A, b ∈ B satisfying F , we combine the strings x(r, a), y(r, b), constructed for the root
r of F , using a known construction of so-called alignment gadgets [2, 29] from previous work (see
Section C.1).
Let us quickly explain how [3] constructed gate gadgets and the main ideas that go into our new
construction. There are two kinds of gadgets, corresponding to the two types of gates in F : AND
and OR gates. Since the AND gadgets will be relatively simple, let us consider the OR gadgets.
Fix two inputs a, b, and let g = (g1 ∨ g2 ) be an OR gate, and assume that we already constructed
gate gadgets for g1 , g2 , namely x1 = x(g1 , a), y1 = y(g1 , b), x2 = x(g2 , a), y2 = y(g2 , b) so that for
i ∈ {1, 2} we have that LCS(xi , yi ) is large if the gate gi outputs true on input (a, b), and it is
smaller otherwise. In [3], these gadgets were combined as follows. Let β be an upper bound on the
total length of the gadgets xi , yi . We add a carefully chosen padding of 0’s and 1’s, so that any
optimal matching of the two strings will have to match either x1 , y1 or x2 , y2 but not both.

7

<!-- page 8 -->

x :=

04β x1 1β x2 04β

y :=

y1 1β 04β 1β y2

One then argues that, in any optimal LCS matching of x, y, the 04β block of y must be matched
either left or right. If it’s matched left, then the total score will be equal to 4β + β + LCS(x2 , y2 )
while if it’s matched right, we will get 4β + β + LCS(x1 , y1 ). Thus, LCS(x, y) is determined by
the OR of g1 , g2 . The blowup of this construction is a multiplicative factor of 11 with every level of
the formula, and the length of the gadget of the root will end up roughly 11depth(F ) . To obtain our
tight lower bounds, we will need to decrease this blowup to 1 + εσ at every level, where εσ goes to
0 when the alphabet size σ tends to infinity. With the above construction, decreasing the length
of the padding will allow the optimal LCS matching to cheat, e.g. by matching y1 to both x1 and
x2 , and no longer corresponding to the OR of g1 , g2 .
Our first trick is an ultra-efficient OR gadget in case we are allowed unbounded alphabet size.
We take x1 , y1 and transform all their letters into a new alphabet Σg1 , and we take x2 , y2 and
transform their letters into a disjoint alphabet Σg2 . Then our OR gadget does not require any
padding at all:
x :=

x1 x2

y :=

y2 y1

The crossing structure of this construction means that any LCS matching that matches letters from
x1 , y1 cannot also match letters from x2 , y2 , and vice versa, while the disjoint alphabets make sure
that there can be no matches between x1 , y2 or x2 , y1 . With such gadgets we can encode a formula
of size s with O(s) letters, for details see Section D.
But how would such an idea work for constant size alphabets? Once we allow x1 and y2 to share
even a single letter, this argument breaks. Natural attempts to simulate this construction with
smaller alphabets, e.g. by replacing each letter with a random sequence, do not seem to work, and
we do not know how to construct such an OR gadget with a smaller alphabet in a black box way.
The major part of our proof will be a careful examination of the formula and the sub-gadgets g1 , g2
in order to reduce the alphabet size to a large enough constant, while using padding that is only
1 + εσ times the length of the sub-gadgets. We achieve this by combining this crossing gadget with
a small padding that will reuse letters from alphabets that were used much deeper in the formula,
and we will argue that the noise we get from recycling letters is dominated by our paddings, in any
optimal matching.
We remark that the reduction of [3] can be implemented in a generic way with any problem that
admits alignment gadgets as defined in [29], giving formula-gadgets of size sO(1) . The list of such
problems includes LCS and Edit-Distance on binary strings. However, to get gadgets of length
s1+o(1) it seems that problem-specific reductions are necessary. A big open question left by our
work is to find the most efficient reduction from Formula-SAT to Edit-Distance. A very efficient
OR gadget, even if the alphabet is unbounded, might be (provably) impossible. Can we use this
intuition to shave more log factors for Edit-Distance?
Fréchet Distance falls outside the alignment gadgets framework of [29] and no reduction from
Formula-SAT was known before. In Section F we prove such a reduction by a significant boosting
of the SETH-lower bound construction of [27]. In order to implement recursive AND/OR gadgets,
8

<!-- page 9 -->

our new proof utilizes the geometry of the curves, in contrast to [27] which only used ten different
points in the plane.
In the remainder of this section we present the details of the reduction to LCS. Some missing
proofs can be found in Section C.

2.1

Implementing Gates

Fix vectors a, b ∈ {0, 1}m (where 2m is the number of inputs to F ). In this section we prove the
following lemma which demonstrates our main construction.
Lemma 2.3. For any sufficiently large σ > 0 let τ = (log σ)1/4 . We can inductively construct, for
each gate g of F , strings x(g) = x(g, a) and y(g) = y(g, b) over alphabet size 5σ 2 and a number
ρ(g) such that for L(g) := LCS(x(g), y(g)) we have (1) L(g) ≤ ρ(g) and (2) L(g) = ρ(g) if and
only if gate g evaluates to true on input (a, b) to F . Moreover, we have |x(g)| = |y(g)| = n(g) ≤
6τ · |Fg |(1 + 7/τ )depth(Fg ) , where Fg is the subformula of F below g.
In this construction, we use disjoint size-5 alphabets Σ1 , . . . , Σσ2 , determining the total alphabet
size as 5σ 2 . Each gate g is assigned an alphabet Σf (g) . We fix the function f later.
In the following, consider any gate g of F , and write the gate alphabet as Σf (g) = {0, 1, 2, 3, 4}.
For readability, we write x = x(g) and similarly define y, n, L, ρ. If g has fanin 2, write g1 , g2 for
the children of g. Moreover, let x1 = x(g1 ) and similarly define y1 , n1 , L1 , ρ1 and x2 , y2 , n2 , L2 , ρ2 .
Input Gate The base case is an input bit ai to F (input bits bj are symmetric). Interpreting ai
as a string of length 1 over alphabet {0, 1}, note that LCS(ai , 1) = ai . Hence, the strings x = ai
and y = 1, with n = ρ = 1, trivially simulate the input bit ai .
AND Gates Consider an AND gate g and let β := ⌈(n1 + n2 )/τ 2 ⌉. We construct strings x, y as
x :=

x1 0β 1β x2

y :=

y1 0β 1β y2

Lemma 2.4. If LCS(x2 , y1 ), LCS(x1 , y2 ) ≤ β/4 and the symbols 0, 1 appear at most β/16 times in
each of x1 , x2 , y1 , and y2 , then we have L = LCS(x, y) = 2β + L1 + L2 .
Later we will choose the gate alphabets Σf (g) such that the precondition of the above lemma
is satisfied. Setting ρ := 2β + ρ1 + ρ2 we thus inductively obtain (1) L ≤ ρ and (2) L = ρ if and
only if g1 and g2 both evaluate to true. Thus, we correctly simulated the AND gate g. It remains
to prove the lemma.
Proof. Clearly, we have L ≥ LCS(x1 , y1 ) + LCS(0β , 0β ) + LCS(1β , 1β ) + LCS(x2 , y2 ) = 2β + L1 + L2 .
For the other direction, consider any LCS z of x, y. If z does not match any symbol of the left half
of x, x1 0β , with any symbol of the right half of y, 1β y2 , and it does not match any symbol of the
right half of x, 1β x2 , with any symbol of the left half of y, y1 0β , then we can split both strings in
the middle and obtain
L = |z| ≤ LCS(x1 0β , y1 0β ) + LCS(1β x2 , 1β y2 ).
9

<!-- page 10 -->

Greedy suffix/prefix matching now yields


L ≤ LCS(x1 , y1 ) + β + β + LCS(x2 , y2 ) = 2β + L1 + L2 .
In the remaining case, there is a matching from some left half to some right half. By symmetry,
we can assume that there is a matching from the left half of x to the right half of y. We can
moreover assume that z matches a symbol of x1 with a symbol of 1β y2 , since the case that z
matches a symbol of y2 with a symbol of x1 0β is symmetric. Now no symbol in 0β in x can be
matched with a symbol in 0β in y. We obtain a rough upper bound on L = |z| by summing up
the LCS length of all remaining 4 · 4 − 1 = 15 pairs of a part x′ ∈ {x1 , 0β , 1β , x2 } in x and a part
y ′ ∈ {y1 , 0β , 1β , y2 } in y. This yields L ≤ L1 + L2 + β + 2 · β/4 + 8 · β/16 = 2β + L1 + L2 , finishing
the proof.
OR Gates Consider an OR gate g and again let β := ⌈(n1 +n2 )/τ 2 ⌉. We first make the LCS target
values equal by adding 4|ρ1 −ρ2 | to the shorter of x2 /y2 and x1 /y1 , i.e., we set x′1 := 4max{0,ρ2 −ρ1 } x1
and similarly y1′ := 4max{0,ρ2 −ρ1 } y1 , x′2 := 4max{0,ρ1 −ρ2 } x2 , y2′ := 4max{0,ρ1 −ρ2 } y2 . Note that the
resulting strings satisfy L′1 := LCS(x′1 , y1′ ) ≤ ρ′ := max{ρ1 , ρ2 } and L′1 = ρ′ if and only if g1
evaluates to true, and similarly L′2 := LCS(x′2 , y2′ ) ≤ ρ′ and L′2 = ρ′ if and only if g2 evaluates to
true. We construct the strings x, y as
x :=

0β 1β x′1 2β 3β x′2 0β 1β

y :=

2β 3β y2′ 0β 1β y1′ 2β 3β

Lemma 2.5. If LCS(x2 , y1 ), LCS(x1 , y2 ) ≤ β/8 and the symbols 0, 1, 2, 3 appear at most β/48 times
in each of x1 , x2 , y1 , and y2 , then L = LCS(x, y) = 4β + max{L′1 , L′2 }.
Later we will choose the gate alphabets Σf (g) such that the precondition of the above lemma
is satisfied. Setting ρ := 4β + ρ′ = 4β + max{ρ1 , ρ2 } we thus inductively obtain (1) L ≤ ρ and (2)
L = ρ if and only if at least one of g1 and g2 evaluates to true, so we correctly simulated the OR
gate g. The proof of the Lemma is in Section C.
Analyzing the Length Note that the above constructions inductively yields strings x(g), y(g)
simulating each gate g. We inductively prove bounds for n(g) and ρ(g). See Section C.
Lemma 2.6. We have n(g) ≤ 6τ · |Fg |(1 + 7/τ )depth(Fg ) and ρ(g) ≤ 6|Fg |(1 + 7/τ )depth(Fg ) for any
gate g, where Fg is the subformula of F below g.
Fixing the Gate Alphabets Now we fix the gate alphabet
S Σf (g) for any gate g. Again let
Σ(i,j), i, j ∈ [σ], be disjoint alphabets of size 5, and let Σ := i,j Σ(i,j) . For any gate g of F , we
call its distance to the root the height h(g). For any h, order the gates with height h from left to
right, and let ι(g) be the index of gate g in this order, for any gate g with height h. Note that
(h(g), ι(g)) is a unique identifier of gate g. We define f (g) := (h(g) mod σ, ι(g) mod σ), i.e., we set
the gate alphabet of g to Σf (g) = Σ(h(g) mod σ,ι(g) mod σ) . Note that the overall alphabet Σ has size
5σ 2 . Recall that we set τ := (log σ)1/4 .
It remains to show that the preconditions of Lemmas 2.4 and 2.5 are satisfied. Specifically, consider a gate g with children g1 , g2 . As before, let x, y, n be the strings and string length constructed
for gate g, and let xi , yi , ni be the corresponding objects for gi , i ∈ {1, 2}. We need to show:
10

<!-- page 11 -->

(1) LCS(x2 , y1 ), LCS(x1 , y2 ) ≤ (n1 + n2 )/(8τ 2 ), and
(2) each symbol c ∈ Σf (g) appears at most (n1 + n2 )/(48τ 2 ) times in each of x1 , x2 , y1 , and y2 .
We call a gate g ′ in the subformula Fg d-deep if h(g ′ ) ≥ h(g) + d, and d-shallow otherwise.
For each symbol c in x or y we can trace our construction to find the gate g ′ in Fg at which we
introduced c to x or y. In other words, each symbol in x, y stems from some gate g′ below g.
First consider (2). Observe that all symbols in x, y stemming from σ-shallow gates do not belong
to the gate alphabet Σf (g) , since the function f (g′ ) has (h(g′ ) mod σ) as the first component, which
repeats only every σ levels. Thus, if a symbol c ∈ Σf (g) occurs in xi or yi , then this occurence
stems from a σ-deep gate. We now argue that only few symbols in x, y stem from deep gates.
For any d > 0, let Nd be the number of symbols in x (or,Pequivalently, y) steming from d-deep
gates. Note that Nd is equal to the total string length
n(g′ ), summed over all gates g′ in
Fg with height h(g′ ) = h(g) + d. Observe that our construction increases the string lengths in
each step by at least a factor 1 + 1/τ 2 , i.e., Nd ≥ (1 + 1/τ 2 )Nd+1 holds for any d. It follows
that Nσ ≤ N1 /(1 + 1/τ 2 )σ−1 = (n1 + n2 )/(1 + 1/τ 2 )σ−1 . Hence, each symbol in Σf (g) appears
at most (n1 + n2 )/(1√+ 1/τ 2 )σ−1√ times in each of x1 , x2 , y1 , y2 . Since τ = (log σ)1/4 , we have
(1 + 1/τ 2 )σ−1 = 2Ω(σ/ log σ) ≥ 48 log σ = 48τ 2 for sufficiently large σ. This proves (2).
For (1), remove all log(σ)-deep symbols from x1 and y2 to obtain strings x′1 , y2′ . Note that we
removed exactly Nlog σ symbols from each of x1 , y2 . This yields LCS(x1 , y2 ) ≤ 2Nlog σ + LCS(x′1 , y2′ ).
For x′1 , y2′ , we claim that any log(σ)-shallow gates g1′ 6= g2′ in Fg have disjoint alphabets Σf (g1′ ) , Σf (g2′ ) .
Indeed, if h(g1′ ) 6= h(g2′ ) then since the first component (h(g ′ ) mod σ) of f (g′ ) repeats only every
σ levels we have f (g1′ ) 6= f (g2′ ). If h(g1′ ) = h(g2′ ) =: h, then note that each gate g′ in height h
has a unique label ι(g′ ) mod σ, since there are σ such labels and there are at most 2h−h(g) < σ
gates with height h in Fg . Hence, x′1 and y2′ use disjoint alphabets, and we obtain LCS(x′1 , y2′ ) = 0.
Thus, LCS(x1 , y2 ) ≤ 2Nlog σ . As above, we bound Nlog σ ≤ (n1 + n2 )/(1 + 1/τ 2 )log σ−1 , so that
2 log σ−1 . Since τ = (log σ)1/4 , we have (1 + 1/τ 2 )log σ−1 /2 =
LCS(x
√ 1 , y2 ) ≤√2(n1 + n2 )/(1 + 1/τ )
log
σ)
2
Ω(
≥ 8 log σ = 8τ for sufficiently large σ. This yields (1), since the strings x2 , y1 are
2
symmetric. This finishes the proof of Lemma 2.3.
Finalizing the Proof Let us sketch how we complete the proof of Theorem 2.2. The full details
are in Section C.1. First, for all vectors a ∈ A, b ∈ B we construct gate gadgets for the output gate
of the formula, i.e. formula gadgets, by invoking Lemma 2.3. Then we combine all these gadgets
by applying a standard alignment gadget [2, 29] to get our final sequences of length O nτ |F |(1 +

7/τ )depth(F ) and with alphabet of size O(σ 2 ). The LCS of the final sequence will be determined by
the existence of a satisfying pair. Since a priori the depth of F could be as large as |F |, the factor
(1 + 7/τ )depth(F ) in our length bound is not yet satisfactory. Thus, as a preprocessing before the
above construction, we decrease the depth of F using a depth-reduction result of Bonet and Buss
[90, 25]: for all k ≥ 2 there is an equivalent formula F ′ with depth at most (3k ln 2) log |F | and
size |F ′ | ≤ |F |1+1/(1+log(k−1))
. Choosing the parameters correctly, we get final sequences of length

O n|F |1+O(1/ log log σ) .

3

On the Limitations of Fine-Grained Reductions

With the increasingly complex web of reductions and conjectures used in the “Hardness in P”
research, one might oppose to our use of nonstandard assumptions. Why can’t we base the hardness
11

<!-- page 12 -->

of shaving logs on one of the more established assumptions such as SETH, or even better, on
P 6= NP? We conclude the paper with a proof that such results are not possible if one is restricted
to fine-grained reductions, which is essentially the only tool we have in this line of research.
Let A be a problem with best known upper bound of TA (n) on inputs of size n, and let B be
a problem with best known upper bound of TB (n) on inputs of size n. Throughout this section we
assume that these runtime are non-decreasing functions, such as 2n or n2 . A fine-grained reduction
from “solving A in time TA (n)/g(n)” to “solving B in time TB (n)/f (n)” proves that improving
TB (n) to TB (n)/f (n) improves TA to TA (n)/g(n). Formally, it is an algorithm X that solves A
and it is allowed to call an oracle for problem B, as long as the following bound holds. Let ni
be the size of the instance in the ith call to problem B that our algorithm performs, where i ≤ t
for some value t, and let TX (n) be the runtime of X P
excluding the time it takes to answer all
the instances of problem B. It must be that TX (n) + ti=1 TB (ni )/f (ni ) ≤ TA (n)/g(n). This is
a natural adaptation of the definition of fine-grained reductions from previous works, where the
improvements were restricted to be by polynomial factors.
We can now give a formal version of Theorem 1.1 from the introduction. Note that k-SAT on
n variables and m clauses can be solved in time poly(n, m)2n .
Theorem 3.1. If for some c, ε > 0 and all k ≥ 2 there is a fine-grained reduction from solving
k-SAT in time poly(n, m)2n /2εn to solving LCS in time O(n2 / logc n), then SETH is false.
Proof. Assume there was a fine-grained reduction from k-SAT to LCS as above. This means that
there is an algorithm X for k-SAT that makes t calls to LCS with instances of size n1 , . . . , nt such
that:
t
X
n2i / logc ni = O(poly(n, m)2n /2εn )
TX (n) +
i=1

But then consider algorithm X ′ which simulates X and whenever X makes a call to the LCS
oracle with an instance of size ni , our algorithm will execute the known quadratic time solution
for LCS that takes O(n2i ) time. Let nmax be the size of the largest instance we call, and note that
nmax < 2n . Simple calculations show that X ′ solves k-SAT and has a running time of
TX (n) +

t
X
i=1


n2i = O poly(n, m)2n /2εn · logc nmax = O(poly(n, m)2n /2εn )

for all k, refuting SETH.

Acknowledgements We are grateful to Avishay Tal for telling us about his algorithm for SAT
on bipartite formulas, and for very helpful discussions. We also thank Mohan Paturi, Rahul Santhanam, Srikanth Srinivasan, and Ryan Williams for answering our questions about the state of
the art of Formula-SAT algorithms, and Arturs Backurs, Piotr Indyk, Mikkel Thorup, and Virginia
Vassilevska Williams for helpful discussions regarding regular expressions. We also thank an anonymous reviewer for ideas leading to shaving off a second log-factor for Formula-Pair, and other
reviewers for helpful suggestions.

12

<!-- page 13 -->

References
[1] A. Abboud, A. Backurs, T. D. Hansen, V. Vassilevska Williams, and O. Zamir. Subtree
isomorphism revisited. In Proc. of 27th SODA, pages 1256–1271, 2016.
[2] A. Abboud, A. Backurs, and V. Vassilevska Williams. Tight Hardness Results for LCS and
other Sequence Similarity Measures. In Proc. of 56th FOCS, pages 59–78, 2015.
[3] A. Abboud, T. D. Hansen, V. V. Williams, and R. Williams. Simulating branching programs
with edit distance and friends: or: a polylog shaved is a lower bound made. In Proc. of the
48th STOC, pages 375–388, 2016.
[4] A. Abboud and V. Vassilevska Williams. Popular conjectures imply strong lower bounds for
dynamic problems. In Proc. of 55th FOCS, pages 434–443, 2014.
[5] A. Abboud, V. Vassilevska Williams, and J. R. Wang. Approximation and fixed parameter
subquadratic algorithms for radius and diameter in sparse graphs. In Proc. of 27th SODA,
pages 377–391, 2016.
[6] A. Abboud, V. Vassilevska Williams, and O. Weimann. Consequences of faster sequence
alignment. In Proc. of 41st ICALP, pages 39–51, 2014.
[7] A. Abboud, V. Vassilevska Williams, and H. Yu. Matching triangles and basing hardness on
an extremely popular conjecture. In Proc. of 47th STOC, pages 41–50, 2015.
[8] A. Abboud, R. Williams, and H. Yu. More applications of the polynomial method to algorithm
design. In Proc. of 26th SODA, pages 218–230, 2015.
[9] P. Agarwal, R. B. Avraham, H. Kaplan, and M. Sharir. Computing the discrete Fréchet
distance in subquadratic time. In Proc. 24th ACM-SIAM Symposium on Discrete Algorithms
(SODA’13), pages 156–167, 2013.
[10] L. Allison and T. I. Dix. A bit-string longest-common-subsequence algorithm. Information
Processing Letters, 23(5):305–310, 1986.
[11] H. Alt and M. Buchin. Can we compute the similarity between surfaces? Discrete & Computational Geometry, 43(1):78–99, 2010.
[12] H. Alt and M. Godau. Computing the Fréchet distance between two polygonal curves. International Journal of Computational Geometry & Applications, 5(1-2):78–99, 1995.
[13] H. Alt, C. Knauer, and C. Wenk. Comparison of distance measures for planar curves. Algorithmica, 38(1):45–58, 2004.
[14] S. F. Altschul, T. L. Madden, A. A. Schäffer, J. Zhang, Z. Zhang, W. Miller, and D. J.
Lipman. Gapped blast and psi-blast: a new generation of protein database search programs.
Nucleic acids research, 25(17):3389–3402, 1997.
[15] A. E. Andreev. About one method of obtaining more than quadratic effective lower bounds
of complexity of pi-schemes, 1987.

13

<!-- page 14 -->

[16] V. Arlazarov, E. Dinic, I. Faradzev, and M. Kronrod. On economic construction of the
transitive closure of a direct graph. In Sov. Math (Doklady), volume 11, pages 1209–1210,
1970.
[17] B. Aronov, S. Har-Peled, C. Knauer, Y. Wang, and C. Wenk. Fréchet distance for curves,
revisited. In Proc. 14th Annual European Symposium on Algorithms (ESA’06), volume 4168
of LNCS, pages 52–63. Springer, 2006.
[18] A. Backurs and P. Indyk. Edit Distance Cannot Be Computed in Strongly Subquadratic
Time (unless SETH is false). In Proc. of 47th STOC, pages 51–58, 2015.
[19] A. Backurs and P. Indyk. Which regular expression patterns are hard to match? In FOCS,
2016.
[20] N. Bansal and R. Williams. Regularity lemmas and combinatorial algorithms. In Proc. of
50th FOCS, pages 745–754, 2009.
[21] I. Baran, E. D. Demaine, and M. Patrascu. Subquadratic algorithms for 3sum. Algorithmica,
50(4):584–596, 2008.
[22] P. Beame, R. Impagliazzo, and S. Srinivasan. Approximating acˆ 0 by small height decision
trees and a deterministic algorithm for# acˆ 0sat. In Computational Complexity (CCC), 2012
IEEE 27th Annual Conference on, pages 117–125. IEEE, 2012.
[23] P. Bille and M. Farach-Colton. Fast and compact regular expression matching. Theoretical
Computer Science, 409(3):486 – 496, 2008.
[24] P. Bille and M. Thorup. Faster regular expression matching. In International Colloquium on
Automata, Languages, and Programming, pages 171–182. Springer, 2009.
[25] M. L. Bonet and S. R. Buss. Size-depth tradeoffs for boolean fomulae. Inf. Process. Lett.,
49(3):151–155, 1994.
[26] S. Brakatsoulas, D. Pfoser, R. Salas, and C. Wenk. On map-matching vehicle tracking data.
In Proc. 31st International Conference on Very Large Data Bases (VLDB’05), pages 853–864,
2005.
[27] K. Bringmann. Why walking the dog takes time: Frechet distance has no strongly subquadratic algorithms unless seth fails. In Proc. of 55th FOCS, pages 661–670, 2014.
[28] K. Bringmann, A. Grønlund, and K. G. Larsen. A dichotomy for regular expression membership testing. CoRR, abs/1611.00918, 2016.
[29] K. Bringmann and M. Künnemann. Quadratic Conditional Lower Bounds for String Problems
and Dynamic Time Warping. In Proc. of 56th FOCS, pages 79–97, 2015.
[30] K. Buchin, M. Buchin, J. Gudmundsson, M. Löffler, and J. Luo. Detecting commuting
patterns by clustering subtrajectories. International Journal of Computational Geometry &
Applications, 21(3):253–282, 2011.

14

<!-- page 15 -->

[31] K. Buchin, M. Buchin, W. Meulemans, and W. Mulzer. Four soviets walk the dog - with an
application to Alt’s conjecture. In Proc. 25th ACM-SIAM Symposium on Discrete Algorithms
(SODA’14), pages 1399–1413, 2014.
[32] K. Buchin, M. Buchin, and Y. Wang. Exact algorithms for partial curve matching via the
Fréchet distance. In Proc. 20th ACM-SIAM Symposium on Discrete Algorithms (SODA’09),
pages 645–654, 2009.
[33] M. Cairo, R. Grossi, and R. Rizzi. New bounds for approximating extremal distances in
undirected graphs. In Proceedings of the Twenty-Seventh Annual ACM-SIAM Symposium on
Discrete Algorithms, SODA 2016, Arlington, VA, USA, January 10-12, 2016, pages 363–376,
2016.
[34] C. Calabro, R. Impagliazzo, and R. Paturi. A duality between clause width and clause density
for SAT. In Proc. of 21st CCC, pages 252–260, 2006.
[35] C. Calabro, R. Impagliazzo, and R. Paturi. The complexity of satisfiability of small depth
circuits. In Proc. of 4th IWPEC, pages 75–85, 2009.
[36] E. W. Chambers, É. Colin de Verdière, J. Erickson, S. Lazard, F. Lazarus, and S. Thite.
Homotopic Fréchet distance between curves or, walking your dog in the woods in polynomial
time. Computational Geometry, 43(3):295–311, 2010.
[37] T. M. Chan. The art of shaving logs. In Proc. of the 13th WADS, page 231, 2013.
[38] T. M. Chan. Speeding up the four russians algorithm by about one more logarithmic factor.
In Proc. of 26th SODA, pages 212–217, 2015.
[39] T. M. Chan. More logarithmic-factor speedups for 3sum, (median,+)-convolution, and some
geometric 3sum-hard problems. In 29th Annual ACM-SIAM Symposium on Discrete Algorithms (SODA’18), pages 881–897, 2018.
[40] K. Chatterjee, W. Dvorák, M. Henzinger, and V. Loitzenbauer. Model and objective separation with conditional lower bounds: Disjunction is harder than conjunction. CoRR,
abs/1602.02670, 2016.
[41] R. Chen. Satisfiability algorithms and lower bounds for boolean formulas over finite bases. In
International Symposium on Mathematical Foundations of Computer Science, pages 223–234.
Springer, 2015.
[42] R. Chen and V. Kabanets. Correlation bounds and #sat algorithms for small linear-size
circuits. In Computing and Combinatorics - 21st International Conference, COCOON 2015,
Beijing, China, August 4-6, 2015, Proceedings, pages 211–222, 2015.
[43] R. Chen, V. Kabanets, A. Kolokolova, R. Shaltiel, and D. Zuckerman. Mining circuit lower
bound proofs for meta-algorithms. computational complexity, 24(2):333–392, 2015.
[44] R. Chen, V. Kabanets, and N. Saurabh. An improved deterministic# sat algorithm for small
de morgan formulas. In International Symposium on Mathematical Foundations of Computer
Science, pages 165–176. Springer, 2014.
15

<!-- page 16 -->

[45] R. Chen and R. Santhanam. Satisfiability on mixed instances. In Proceedings of the 2016
ACM Conference on Innovations in Theoretical Computer Science, Cambridge, MA, USA,
January 14-16, 2016, pages 393–402, 2016.
[46] A. F. Cook and C. Wenk. Geodesic Fréchet distance inside a simple polygon. ACM Transactions on Algorithms, 7(1):193–204, 2010.
[47] T. H. Cormen, C. E. Leiserson, R. L. Rivest, and C. Stein. Introduction to algorithms,
volume 6. MIT press Cambridge, 2001.
[48] M. Crochemore, C. S. Iliopoulos, Y. J. Pinzon, and J. F. Reid. A fast and practical bit-vector
algorithm for the longest common subsequence problem. Information Processing Letters,
80(6):279–285, 2001.
[49] M. Crochemore, G. M. Landau, and M. Ziv-Ukelson. A subquadratic sequence alignment
algorithm for unrestricted scoring matrices. SIAM journal on computing, 32(6):1654–1673,
2003.
[50] E. Dantsin and E. A. Hirsch. Worst-case upper bounds. In Handbook of Satisfiability, pages
403–424. 2009.
[51] E. Dantsin and A. Wolpert. Exponential complexity of satisfiability testing for linear-size
boolean formulas. In International Conference on Algorithms and Complexity, pages 110–
121. Springer, 2013.
[52] A. Driemel and S. Har-Peled. Jaywalking your dog: computing the Fréchet distance with
shortcuts. SIAM Journal on Computing, 42(5):1830–1866, 2013.
[53] A. Driemel, S. Har-Peled, and C. Wenk. Approximating the Fréchet distance for realistic
curves in near linear time. Discrete & Computational Geometry, 48(1):94–127, 2012.
[54] T. Eiter and H. Mannila. Computing discrete Fréchet distance. Technical Report CD-TR
94/64, Christian Doppler Laboratory for Expert Systems, TU Vienna, Austria, 1994.
[55] A. Freund. Improved subquadratic 3sum. Algorithmica, 77(2):440–458, 2017.
[56] P. Gawrychowski. Faster algorithm for computing the edit distance between slp-compressed
strings. In International Symposium on String Processing and Information Retrieval, pages
229–236. Springer, 2012.
[57] M. Godau. A natural metric for curves - computing the distance for polygonal chains and
approximation algorithms. In Proc. 8th Symposium on Theoretical Aspects of Computer
Science (STACS’91), volume 480 of LNCS, pages 127–136. Springer, 1991.
[58] O. Gold and M. Sharir. Improved Bounds for 3SUM, k-SUM, and Linear Degeneracy. In
25th Annual European Symposium on Algorithms (ESA 2017), volume 87, pages 42:1–42:13,
2017.
[59] A. Golovnev, A. S. Kulikov, A. Smal, and S. Tamaki. Circuit size lower bounds and# sat
upper bounds through a general framework. In Electronic Colloquium on Computational
Complexity (ECCC), volume 23, page 22, 2016.
16

<!-- page 17 -->

[60] S. Grabowski. New tabulation and sparse dynamic programming based techniques for sequence similarity problems. In Stringology, pages 202–211, 2014.
[61] A. Grønlund and S. Pettie. Threesomes, degenerates, and love triangles. In 55th IEEE
Annual Symposium on Foundations of Computer Science, FOCS 2014, Philadelphia, PA,
USA, October 18-21, 2014, pages 621–630, 2014.
[62] J. Håstad. The shrinkage exponent of de morgan formulas is 2. SIAM J. Comput., 27(1):48–64,
1998.
[63] J. Hopcroft, W. Paul, and L. Valiant. On time versus space. Journal of the ACM (JACM),
24(2):332–337, 1977.
[64] H. Hyyrö. Bit-parallel lcs-length computation revisited. In Proc. 15th Australasian Workshop
on Combinatorial Algorithms (AWOCA 2004), pages 16–27. Citeseer, 2004.
[65] R. Impagliazzo, S. Lovett, R. Paturi, and S. Schneider. 0-1 integer linear programming with a
linear number of constraints. Electronic Colloquium on Computational Complexity (ECCC),
21:24, 2014.
[66] R. Impagliazzo, W. Matthews, and R. Paturi. A satisfiability algorithm for ac 0. In Proceedings of the twenty-third annual ACM-SIAM symposium on Discrete Algorithms, pages
961–972. SIAM, 2012.
[67] R. Impagliazzo and N. Nisan. The effect of random restrictions on formula size. Random
Structures & Algorithms, 4(2):121–133, 1993.
[68] R. Impagliazzo and R. Paturi. On the complexity of k-sat. Journal of Computer and System
Sciences, 62(2):367–375, 2001.
[69] R. Impagliazzo, R. Paturi, and S. Schneider. A satisfiability algorithm for sparse depth two
threshold circuits. In 54th Annual IEEE Symposium on Foundations of Computer Science,
FOCS 2013, 26-29 October, 2013, Berkeley, CA, USA, pages 479–488, 2013.
[70] P. Indyk. Approximate nearest neighbor algorithms for Fréchet distance via product metrics.
In Proc. 18th Annual Symposium on Computational Geometry (SoCG’02), pages 102–106,
2002.
[71] V. M. Khrapchenko. Method of determining lower bounds for the complexity of p-schemes.
Mathematical Notes, 10(1):474–479, 1971.
[72] I. Komargodski, R. Raz, and A. Tal. Improved average-case lower bounds for demorgan formula size. In Foundations of Computer Science (FOCS), 2013 IEEE 54th Annual Symposium
on, pages 588–597. IEEE, 2013.
[73] I. T. Li, W. Shum, and K. Truong. 160-fold acceleration of the smith-waterman algorithm
using a field programmable gate array (fpga). BMC bioinformatics, 8(1):1, 2007.
[74] Y. Liu, A. Wirawan, and B. Schmidt. Cudasw++ 3.0: accelerating smith-waterman protein
database search by coupling cpu and gpu simd instructions. BMC bioinformatics, 14(1):1,
2013.
17

<!-- page 18 -->

[75] A. Maheshwari, J.-R. Sack, K. Shahbaz, and H. Zarrabi-Zadeh. Fréchet distance with speed
limits. Computational Geometry, 44(2):110–120, 2011.
[76] W. J. Masek and M. S. Paterson. A faster algorithm computing string edit distances. Journal
of Computer and System sciences, 20(1):18–31, 1980.
[77] D. Moeller, R. Paturi, and S. Schneider. Subquadratic algorithms for succinct stable matching. In Computer Science - Theory and Applications - 11th International Computer Science
Symposium in Russia, CSR 2016, St. Petersburg, Russia, June 9-13, 2016, Proceedings, pages
294–308, 2016.
[78] M. E. Munich and P. Perona. Continuous dynamic time warping for translation-invariant
curve alignment with applications to signature verification. In Proc. 7th IEEE International
Conference on Computer Vision, volume 1, pages 108–115, 1999.
[79] G. Myers. A four russians algorithm for regular expression pattern matching. Journal of the
ACM (JACM), 39(2):432–448, 1992.
[80] I. C. Oliveira. Algorithms versus circuit lower bounds. arXiv preprint arXiv:1309.0249, 2013.
[81] M. S. Paterson and U. Zwick. Shrinkage of de morgan formulae under restriction. Random
Structures & Algorithms, 4(2):135–150, 1993.
[82] M. Patrascu and R. Williams. On the possibility of faster SAT algorithms. In Proc. of 21st
SODA, pages 1065–1075, 2010.
[83] R. Paturi, P. Pudlák, M. E. Saks, and F. Zane. An improved exponential-time algorithm for
k -sat. J. ACM, 52(3):337–364, 2005.
[84] L. Roditty and V. Vassilevska Williams. Fast approximation algorithms for the diameter and
radius of sparse graphs. In Proc. of 45th STOC, pages 515–524, 2013.
[85] T. Sakai, K. Seto, S. Tamaki, and J. Teruyama. A satisfiability algorithm for depth-2 circuits
with a symmetric gate at the top and and gates at the bottom. In Electronic Colloquium on
Computational Complexity (ECCC), 2015.
[86] R. Santhanam. Fighting perebor: New and improved algorithms for formula and QBF satisfiability. In Proc. of the 51th FOCS, pages 183–192, 2010.
[87] R. Santhanam et al. Ironic complicity: Satisfiability algorithms and circuit lower bounds.
Bulletin of EATCS, 1(106), 2013.
[88] K. Seto and S. Tamaki. A satisfiability algorithm and average-case hardness for formulas over
the full binary basis. computational complexity, 22(2):245–274, 2013.
[89] T. F. Smith and M. S. Waterman. Identification of common molecular subsequences. Journal
of molecular biology, 147(1):195–197, 1981.
[90] P. M. Spira. On time-hardware complexity tradeoffs for boolean functions. In Proceedings of
the 4th Hawaii Symposium on System Sciences, pages 525–527, 1971.

18

<!-- page 19 -->

[91] B. A. Subbotovskaya. Realizations of linear functions by formulas using+. Doklady Akademii
Nauk SSSR, 136(3):553–555, 1961.
[92] A. Tal. Shrinkage of de morgan formulae by spectral techniques. In Foundations of Computer
Science (FOCS), 2014 IEEE 55th Annual Symposium on, pages 551–560. IEEE, 2014.
[93] A. Tal. #sat algorithms from shrinkage. Electronic Colloquium on Computational Complexity
(ECCC), 22:114, 2015.
[94] K. Thompson. Programming techniques: Regular expression search algorithm. Communications of the ACM, 11(6):419–422, 1968.
[95] R. A. Wagner and M. J. Fischer. The string-to-string correction problem. Journal of the
ACM (JACM), 21(1):168–173, 1974.
[96] R. Williams. A new algorithm for optimal 2-constraint satisfaction and its implications.
Theoretical Computer Science, 348(2):357–365, 2005.
[97] R. Williams. Improving exhaustive search implies superpolynomial lower bounds. SIAM
Journal on Computing, 42(3):1218–1244, 2013.
[98] R. Williams. Algorithms for Circuits and Circuits for Algorithms: Connecting the Tractable
and Intractable. In Proceedings of the International Congress of Mathematicians, 2014.
[99] R. Williams. Faster all-pairs shortest paths via circuit complexity. In Proc. of 46th STOC,
pages 664–673, 2014.
[100] R. Williams. New algorithms and lower bounds for circuits with linear threshold gates. In
Proceedings of the 46th Annual ACM Symposium on Theory of Computing, pages 194–202.
ACM, 2014.
[101] R. Williams. Nonuniform ACC circuit lower bounds. J. ACM, 61(1):2:1–2:32, 2014.
[102] H. Yu. An improved combinatorial algorithm for boolean matrix multiplication. In Proc. of
42nd ICALP, pages 1094–1105, 2015.

A

Discussion

As shown above, the popular conjectures are not fine-grained enough for our purposes and our only
viable option is to start from assumptions about the hardness of shaving logs for some problem.
The approach taken in this paper and in [3] is to start with variants of SAT. Another option would
have been to conjecture that 3-SUM cannot be solved in O(n2 / log2+ε n) time, but SAT has several
advantages. First, SAT is deeply connected to fundamental topics in complexity theory, which
allows us to borrow barriers that complexity theorists have faced for decades. Moreover, there is
a vast number of combinatorial problems that we can reduce SAT to, whereas 3-SUM seems more
useful in geometric contexts, e.g. 3-SUM-hardness for LCS and Frechet might be impossible [31].
Thus, for the task of proving barriers for shaving logs, our approach seems as good as any.
Conditional lower bounds can even lead to better algorithms, by suggesting regimes of possible
improvements. Phrased this way, our results leave the open problem of finding a O(n2 /(log n)2+ε )
19

<!-- page 20 -->

time algorithms for LCS, and perhaps more interestingly, shaving many more logs for the relatedbut-different Edit-Distance problem. The longstanding upper bound for Edit-Distance is O(n2 / log2 n)
[76] and our approach does not give barriers higher than Ω(n2 / log20 n).
Finally, regardless of the consequences of our reductions, we think that the statements themselves are intrinsically interesting as they reveal a surprisingly close connection between FormulaSAT (a problem typically studied by complexity theorists) and combinatorial problems that are
typically studied by stringologists, computational biologists, and computational geometers, which
are a priori completely different creatures. The runtime of the standard algorithm for SAT can
be recovered almost exactly by encoding the formula into an LCS, Fréchet, or Pattern Matching
instance and running the standard dynamic programming algorithms!

B

From Formula-SAT to Formula-Pair

In this section we show a chain of simple reductions starting from variants of Formula-SAT, which
have 2n time complexity, and ending at n2 time variants of a problem we call Formula-Pair.
A formula F of size s over n variables x1 , . . . , xn is in the class F1 iff it has the following
properties. The gates in the first layer (nodes in the tree whose children are all leaves) compute
arbitrary functions C : {0, 1}n/2 → {0, 1}, as long as C can be computed in 2o(n) time and all
children of a gate are marked with variables in {x1 , . . . , xn/2 } or with variables in {xn/2+1 , . . . , xn }
but not with both. W.l.o.g. we can assume that the inputs are only connected to nodes in the first
layer. The gates in the other layers compute deMorgan gates, i.e., OR and AND gates. The size
of F is considered to be the number of gates in the first layer. Since F is a formula and thus has
fanout 1, our size measure is up to constant factors equal to the total number of all gates except the
inputs. Note that the complexity of the functions in the first layer and their number of incoming
wires, i.e. the number of leaves in the tree, do not count towards the size of F .
F1 -Formula-SAT
Input:
Formula F = F (x1 , . . . , xn ) of size s with n inputs from the class F1
Question: Exist x1 , . . . , xn ∈ {0, 1} such that F (x1 , . . . , xn ) = true?
Complexity: O(2n (s/n + 1)), even restricted to s ≤ n1+o(1)
Note that many techniques developed for deMorgan formulas (and generalizations) are not
applicable to F1 -Formula-SAT, since the first layer is so general. In particular, techniques based on
collapses by random restrictions [44, 43, 93] do not seem to work, as the first layer can be resistant
to such collapses.
A simple algorithm achieves O(2n · s) runtime: Preprocess F to create a table that allows one
to quickly lookup the value of each one of the first-layer gates on a given input. Constructing the
table takes time s · 2n/2 · 2o(n) , and after we have it, a brute-force SAT algorithm takes O(2n · s)
time.
However, one can improve upon this simple algorithm and obtain time O(2n s/n). This is the
best time complexity we are aware of, up to log-factors, unless we restrict the formula size to
s ≤ n2−Ω(1) . Our tightest barriers for shaving logs for LCS can be based on the assumption that
F1 -Formula-SAT cannot be solved in time O(2n /n1+ε ), for any ε > 0, even when s = n2+o(1) . Such
upper bounds seem out of reach of current techniques.
20

<!-- page 21 -->

Theorem B.1. F1 -Formula-SAT can be solved in time O(2n (s/n + 1)).
Proof. This follows by combining Lemma B.2 and Theorem B.4 below, noting that due to the
change in the machine model we lose one log-factor. It is also easy to directly design an algorithm
with the claimed running time by following the ideas in Theorem B.4, noting that reading a packed
word now takes time O(n) instead of O(1), and all accesses in the (exponential size) precomputed
table now take time O(n).
Next, we move from 2n to n2 with a simple split-and-list reduction (similar to [96]) to the
following “pair finding” problem.

Formula-Pair
Input:
A deMorgan Formula F = F (x1 , . . . , xm , y1 , . . . , ym ) of size 2m where each input
is used exactly once, and A, B ⊆ {0, 1}m of size n
Question: Exist a ∈ A, b ∈ B such that F (a, b) = F (a1 , . . . , am , b1 , . . . , bm ) = true?
Complexity: O(n2 m/ log2 n + n2 / log n) (Theorem B.4), even restricted to m ≤ (log n)1+o(1)
We remark that the assumption that F reads any input exactly once is w.l.o.g., since the sets
A, B allow us to “copy” an input xi to xj by simply ensuring that ai = aj for all a ∈ A.
Lemma B.2. An instance of F1 -Formula-SAT on a formula of size s over n inputs can be reduced
to an instance of Formula-Pair on two sets of size O(2n/2 ) and a formula of size m = O(s), in
linear time.
Proof. Let C1 , . . . , Cm be the gates of the first layer of F that compute functions of the first half of
′ be the rest of the gates of the first layer, for some k ≤ s. For each
the variables, and let C1′ , . . . , Cm
partial assignment to the first n/2 variables of the formula we compute a bit-string a ∈ {0, 1}m
such that ai is 1 iff Ci outputs 1 on the corresponding partial assignment. Similarly, we compute
a bit-string b ∈ {0, 1}m for each partial assignment to the second half of the variables. We define
our deMorgan formula F ′ to be equivalent to the layers of F that are above the first one. Now, to
conclude the reduction, we observe that for any two partial assignments α, β to F , the corresponding
bit strings a, b contain the values of the first layer of F on the assignment (αβ) and so the value of
F ′ on (ab) is exactly the value of F on (αβ). Thus, F is satisfiable iff there is a pair a ∈ A, b ∈ B
that satisfies F ′ . Since F has fanout 1, every input of F ′ is used exactly once.
A trivial algorithm for Formula-Pair takes O(n2 · m) time, and in Section B.1 we show how
to shave two log factors to O(n2 m/ log2 n), for m = Ω(log n). Via the above reduction, this yields
an O(2n s/n) algorithm for F1 -Formula-SAT for s = Ω(n), due to the loss of a log(n) factor in the
jump from 2n to n and the machine model.
In the other sections we reduce Formula-Pair to LCS and show that any improvement by
more than another log-factor on its running time directly yields improved runtimes for all problems
above, including F1 -Formula-SAT. While we chose to present the barriers for shaving logs for LCS
in terms of barriers for solving SAT faster, it is only safer to conjecture that the possibly harder
Formula-Pair problem cannot be solved faster.
Next, we consider different variants that we will be able to reduce to the Fréchet distance. A
formula F of size s over n variables x1 , . . . , xn is in the class F2 iff it has the following properties.
21

<!-- page 22 -->

The gates in the first layer (nodes in the tree whose children are all leaves) are as in F1 . The
n
gates in the second layer
Pn compute threshold functions τ : {0, 1} → {0, 1} of their inputs, i.e.
τ (x1 , . . . , xn ) = true iff i=1 ci xi ≤ T where c1 , . . . , cn , T ∈ {−M, −M + 1 . . . , M }. W.l.o.g. we can
assume that the inputs are only connected to nodes in the first layer, and first layer nodes are only
connected to second layer nodes. The gates in the other layers compute deMorgan gates, i.e., OR
and AND gates. The size of F is considered to be the number of gates in the first plus second layer.
Note that the complexity of the functions in the first layer and their number of incoming wires, i.e.
the number of leaves in the tree, do not count towards the size of F .
F2 -Formula-SAT
Input:
Formula F = F (x1 , . . . , xn ) of size s with n inputs from the class F2 with coordinates bounded by M ≤ 2O(n)
Question: Exist x1 , . . . , xn ∈ {0, 1} such that F (x1 , . . . , xn ) = true?
Complexity: O(2n s · no(1) ), even restricted to s ≤ n1+o(1)
The O(2n s · no(1) ) algorithm follows from Lemma B.3 and the algorithm for Ineq-FormulaPair below, again noting that we lose one log-factor due to the change in the machine model. Our
tight barriers for shaving logs for Fréchet will be based on the assumption that F2 -Formula-SAT
cannot be solved in 2n /nε , for some ε > 0, even when s = n1+o(1) . Since F2 is even more expressive
than F1 , such upper bounds seem out of reach.
Remark: Any formula in F2 (of size s over n variables) can be transformed into a formula in
F1 over the same variables of size poly(s), since arithmetic has efficient circuits. However, since
we care about polynomial factors in s in this paper, the difference between F1 and F2 is nonnegligible for our purposes. In fact, the fastest running times that we know for F1 -Formula-SAT
and F2 -Formula-SAT differ by a factor n1+o(1) .
We will work with an intermediate “pair finding” problem:
Ineq-Formula-Pair
Input:
Formula F = F (x1 , . . . , xm ) where each input is used exactly once, and A, B ⊆
{−M, . . . , M }m of size n, where M ≤ nO(1)
Question: Exist a ∈ A, b ∈ B such that F ([a1 ≤ b1 ], . . . , [am ≤ bm ]) = true?
Complexity: O(n2 m logloglogn n ), even restricted to m ≤ (log n)1+o(1)
Similarly to the reductions above, we can prove the following. Similar reductions have been
used in [100, 65].
Lemma B.3. An instance of F2 -Formula-SAT on a formula of size s over n inputs can be reduced
to an instance of Ineq-Formula-Pair on two sets of size O(2n/2 ) and a formula of size m = O(s),
in linear time.
The claimed running time O(n2 m logloglogn n ) follows by combining our reduction from Ineq-FormulaPair to the Fréchet distance with the fastest known algorithm for deciding the Fréchet distance [9].
We think that one can also obtain this running time more directly by “Four Russians” tricks, as
have been used for LCS for large alphabet size. The further tricks that shaved a second log-factor
off the running time of Formula-Pair do not seem to work for Ineq-Formula-Pair.
22

<!-- page 23 -->

B.1

Algorithm for Formula-Pair

Theorem B.4. Formula-Pair can be solved in time O(n2 m/ log2 n + n2 / log n).
The main trick used to prove the above theorem is the following decomposition of formulas.
Lemma B.5. Given a deMorgan formula F of size m and a number L ≥ 2, in polynomial time we
can compute a decoposition D = {F1 , . . . , Fk } such that
1. each Fi is a subformula of F , i.e., a set of gates of F forming a subtree,
2. for each wire e of F , exactly one Fi contains both endpoints of e,
3. each Fi contains less that 3L gates, and
4. |D| ≤ 4m/L + 1.
Furthermore, call a gate in Fi special if it is the root of another subformula Fj , j 6= i. In our
decomposition each Fi has at most 2 special gates.
Similar decompositions where known before, see, e.g., [51], however, we are not aware of a
decomposition bounding the number of special gates by a constant.
Proof. Note that we can assume F to have indegrees bounded by 2. Initialize D = ∅. We assign to
each gate g of F a weight w(g) in {1, L}; initially all weights are 1, and inner nodes will always have
weight 1. Now repeatedly perform the following procedure. Start at the root of F and repeatedly
go to the child whose subtree has larger total weight, until reaching a gate g whose subtree has
total weight in [ 54 L, 3L). Add the subformula computed by g to D. Finally, remove all gates below
g from F and set the weight of g to L. Repeat this procedure until the total weight of F is less
than 45 L. Add the remaining formula F to D. This defines a decomposition D.
Note that we always find a gate whose subtree has total weight in [ 45 L, 3L). Indeed, if the
current gate g has total subtree weight w ≥ 3L, then g is an inner node and thus has weight 1, and
g’s children together have total subtree weight at least w − 1. Since we go to the heavier child, we
obtain total subtree weight at least (w − 1)/2 ≥ (3L − 1)/2 ≥ 54 L, where we used L ≥ 2. Hence,
we will land in the interval [ 54 L, 3L).
Also note that |D| ≤ 4m/L + 1, since in each call of the procedure we remove weight at least
5
L
(by deleting the gates below g) and then increase the weight of the chosen gate g by at most L,
4
so we lose weight at least L/4 in every call (except possibly in the very last step).
Let D = {F1 , . . . , Fk }. Each subformula Fi of F has total weight less than 3L, and thus it
consists of less than 3L gates. Moreover, the gates g in Fi with weight L exactly correspond to the
special gates of Fi . Since each such gate g has weight L, there are less than 3L/L special gates,
and thus each Fi has at most 2 special gates.
Proof of Theorem B.4. To solve Formula-Pair in time O(n2 m/ log2 n + n2 / log n), we use the
“Four Russians” trick as well as packing log(n) bits in a word. Let D = {F1 , . . . , Fk } be a decomposition of F as in Lemma B.5, where L := ε log n, for a sufficiently small constant ε > 0. Note
that each subfomula Fi has size O(ε log n) and thus contains O(ε log n) input gates of F .
For each subformula Fi and each assignment β to the O(ε log n) b-variables appearing in Fi , let
β
Fi be the simplified subformula after fixing the b-variables. Compute the value of Fiβ on all vectors
in A and all possible assignments to the special gates in Fi . We store these values in memory as
23

<!-- page 24 -->

follows. For each 1 ≤ ℓ ≤ n divisible by L and all length-L bitstrings x, x′ , store the values of Fiβ
on the ℓ + j-th vector in A, with the input of the special gates set to xj , x′j (recall that there are
at most 2 special gates). Store these L bits, for 0 ≤ j < L, in one word. We thus stored for each
of O(m/ log n) subformulas, each of nO(ε) assignments to the b-variables, each of O(n/L) bundles
of vectors in A (indexed by ℓ), and each of nO(ε) assignments to the special gates, one word in
memory. Computing these values takes time mn1+O(ε) and is thus negligible.
Now we can evaluate F on all pairs a ∈ A, b ∈ B as follows. Iterate over all b ∈ B, and over all
bundles of L consecutive vectors A′ ⊆ A (indexed by ℓ, divisible by L). Iterate over the subformulas
Fi in a topological order (from bottom to top). For each subformula Fi , if Fi has no special gates
then access Fiβ , the simplification of Fi under b, and determine by one table lookup the output of
Fiβ on all vectors a ∈ A′ . If Fi does have special gates, then it depends on the output of some
subformulas Fj , for which we have already computed the output on b and any a ∈ A′ . Plugging in
these outputs as the vectors x, x′ above, we can again determine by one table lookup the output of
Fiβ on all vectors a ∈ A′ . Repeating this for all subformulas Fi eventually yields the output of F
on b and any a ∈ A′ .
Note that for each vector b and bundle A′ this procedure takes time O(|D|) = O(m/ log n + 1).
Since there are n vectors b and O(n/ log n) bundles A′ , the total running time is O(n2 m/ log2 n +
n2 / log n). In the end, we check whether the output on any pair a ∈ A, b ∈ B is 1 to decide the
given Formula-Pair instance.

C

Missing Details in the LCS Proof

This section contains the missing details for completing the reduction from Formula-Pair to LCS
that we presented in Section 2.
We will make use of the fact that for any strings x, y, symbol c, and k ≥ 1 we have LCS(1k x, 1k y) =
LCS(x1k , y1k ) = k + LCS(x, y), which allows to greedily match prefixes and suffixes (see, e.g., [29,
Fact 7.1.(1)]).
Proof of Lemma 2.5. Clearly, we have L ≥ LCS(0β 1β , 0β 1β ) + LCS(x′1 , y1′ ) + LCS(2β 3β , 2β 3β ) =
4β + L′1 and L ≥ LCS(2β 3β , 2β 3β ) + LCS(x′2 , y2′ ) + LCS(0β 1β , 0β 1β ) = 4β + L′2 . For the other
direction, note that no LCS of x, y can match symbols both in the 0β 1β -prefix of x and in the
2β 3β -prefix of y, since these prefixes use disjoint alphabets and thus any such matchings would
cross. Hence,

L ≤ max LCS(x′1 2β 3β x′2 0β 1β , y), LCS(x, y2′ 0β 1β y1′ 2β 3β ) .
(1)
Consider the first term, LCS(x̃, y) for x̃ = x′1 2β 3β x′2 0β 1β . By the same argument as above, no LCS
of x̃, y can match symbols both in the 0β 1β -suffix of x̃ and in the 2β 3β -suffix of y. Thus,

(2)
LCS(x̃, y) = max LCS(x′1 2β 3β x′2 , y), LCS(x̃, 2β 3β y2′ 0β 1β y1′ ) .
Note that LCS(x′1 2β 3β x′2 , y) ≤ LCS(2β 3β , y) + LCS(x′1 x′2 , y) = 2β + LCS(x′1 x′2 , y). We bound
LCS(x′1 x′2 , y) by summing up the LCS length of all pairs of a part x′ ∈ {x′1 , x′2 } in the first string
and a part y ′ ∈ {2β , 3β , y2′ , 0β , 1β , y1′ , 2β , 3β } in y, noting that we can either match x′1 and y1′ or x′2
and y2′ , so that the two pairs (x′1 , y1′ ), (x′2 , y2′ ) together contribute at most max{L′1 , L′2 }. This yields
LCS(x′1 x′2 , y) ≤ max{L′1 , L′2 } + 2 · β/8 + 12 · β/48 ≤ β/2 + max{L′1 , L′2 }.
24

<!-- page 25 -->

Together we obtain, as desired,
LCS(x′1 2β 3β x′2 , y) ≤ 25 β + max{L′1 , L′2 }.

(3)

For the other term LCS(x̃, ỹ) = LCS(x̃, 2β 3β y2′ 0β 1β y1′ ) of equation (2), let z be an LCS and
consider the number k of symbols in the 2β 3β -prefix of ỹ matched by z. Note that we can
bound LCS(x̃, ỹ) ≤ k + LCS(x̃, y1′ 0β 1β y2′ ) ≤ k + LCS(x, y1′ 0β 1β y2′ ). By symmetry, we can bound
LCS(x, y1′ 0β 1β y2′ ) in the same way as equation (3) by 52 β + max{L′1 , L′2 }. Thus, for k ≤ 23 β we
obtain LCS(x̃, ỹ) ≤ 4β + max{L′1 , L′2 }, as desired.
In the remaining case k > 23 β, we argue that we can split LCS(x̃, ỹ) = LCS(x′1 2β 3β , 2β 3β ) +
LCS(x′2 0β 1β , y2′ 0β 1β y2′ ). Note that at least k − β > β/2 symbols 2 of the 2β 3β -prefix of ỹ are
matched. Since x′1 and x′2 contain at most β/48 symbols 2 by assumption, at least one symbol of
the 2β -block in ỹ is matched to the 2β -block in x̃. Similarly, it follows that at least one symbol of
the 3β -block in ỹ is matched to the 3β -block in x̃. Thus, the LCS z splits between 2β and 3β , i.e.,
we have
LCS(x̃, ỹ) = LCS(x′1 2β , 2β ) + LCS(3β x′2 0β 1β , 3β y2′ 0β 1β y1′ ).
Using greedy prefix matching, we can remove the prefix 3β of the second term to get
LCS(x̃, ỹ) = LCS(x′1 2β , 2β ) + LCS(3β , 3β ) + LCS(x′2 0β 1β , y2′ 0β 1β y1′ )
= LCS(x′1 2β 3β , 2β 3β ) + LCS(x′2 0β 1β , y2′ 0β 1β y1′ ).
Hence, there exists an LCS of x̃, ỹ matching only symbols in the prefixes x′1 2β 3β and 2β 3β and
symbols in the suffixes x′2 0β 1β and y2′ 0β 1β y1′ . By symmetry, we can also split off the suffixes 0β 1β
and 0β 1β y2′ . It follows that
LCS(x̃, ỹ) = LCS(x′1 2β 3β , 2β 3β ) + LCS(x′2 , y2′ ) + LCS(0β 1β , 0β 1β y2′ ) = 4β + L′2 ,
as desired. This finishes the analysis of the first term in equation (1). The second term gives the
symmetric bound 4β + L′1 , so in total we obtain the desired bound L ≤ 4β + max{L′1 , L′2 }.
Proof of Lemma 2.6. In the base case we construct strings of length 1, so that n(g), ρ(g) = 1.
For an inner gate g, we recursively construct strings of length n1 , n2 and LCS bounds ρ1 , ρ2 for
the children on subformulas F1 , F2 . If g is an AND gate we have
n(g) = n1 + n2 + 2β = n1 + n2 + 2⌈(n1 + n2 )/τ 2 ⌉ ≤ (n1 + n2 )(1 + 2/τ 2 ) + 2,
ρ(g) = ρ1 + ρ2 + 2β ≤ ρ1 + ρ2 + 2(n1 + n2 )/τ 2 + 2.

If g is an OR gate we pad the strings to the same LCS length, thus increasing n(g) by |ρ1 − ρ2 |,
n(g) = n1 + n2 + |ρ1 − ρ2 | + 6β ≤ (n1 + n2 )(1 + 6/τ 2 ) + 6 + ρ1 + ρ2 ,
ρ(g) = 4β + max{ρ1 , ρ2 } ≤ ρ1 + ρ2 + 4(n1 + n2 )/τ 2 + 4.

In both cases, we have
n(g) ≤ (n1 + n2 )(1 + 6/τ 2 ) + 6 + ρ1 + ρ2 ,
ρ(g) ≤ ρ1 + ρ2 + 4(n1 + n2 )/τ 2 + 4.
25

<!-- page 26 -->

Plugging in the inductive hypothesis yields, since F1 , F2 have depth at most depth(Fg ) − 1,
n(g) ≤ 6τ (|F1 | + |F2 |)(1 + 7/τ )depth(Fg )−1 (1 + 6/τ 2 ) + 6 + 6(|F1 | + |F2 |)(1 + 7/τ )depth(Fg )−1 .
We use the fact |Fg | = |F1 | + |F2 | + 1 to cancel the additive term +6 at the cost of increasing the
first |F1 | + |F2 | to |Fg |. We obtain
n(g) ≤ 6τ |Fg |(1 + 7/τ )depth(Fg )−1 (1 + 6/τ 2 ) + 6|Fg |(1 + 7/τ )depth(Fg )−1
=

1 + 6/τ 2 + 1/τ
· 6τ |Fg |(1 + 7/τ )depth(Fg ) .
1 + 7/τ

Since τ = (log σ)1/4 ≥ 1, as σ is sufficiently large, we have 1 + 6/τ 2 + 1/τ ≤ 1 + 7/τ , which yields
the desired bound. For ρ(g) we similarly obtain
ρ(g) ≤ 6(|F1 | + |F2 |)(1 + 7/τ )depth(Fg )−1 + 4 · 6τ (|F1 | + |F2 |)(1 + 7/τ )depth(Fg )−1 /τ 2 + 4
Again we cancel the additive +4 by increasing |F1 | + |F2 | to |Fg |. We obtain
ρ(g) ≤ 6|Fg |(1 + 7/τ )depth(Fg )−1 + 4 · 6τ |Fg |(1 + 7/τ )depth(Fg )−1 /τ 2
1 + 4/τ
=
· 6|Fg |(1 + 7/τ )depth(Fg ) ,
1 + 7/τ
which yields the desired bound and finishes the proof.

C.1

The Outer OR - Finalizing the Proof

We now finish the reduction from Formula-Pair to LCS using alignment gadgets [2, 29]. The
following result is implicit in [2, Lemmas 3 and 4], and a similar (independently found) result is
contained in [29] under the name of “alignment gadget”.
Lemma C.1. Let x1 , . . . , xn and y1 , . . . , yn be strings of length λ. Let ρ ≥ LCS(xi , yj ) for all i, j.
Then one can construct strings x, y of length O(nλ) and an integer ρ′ such that (1) LCS(x, y) ≤ ρ′
and (2) LCS(x, y) = ρ′ if and only if there exist i, j with LCS(xi , yj ) = ρ. If the strings xi , yj use
alphabet Σ, then x, y use alphabet size at most |Σ| + 4.
For any vectors a ∈ A, b ∈ B, invoking Lemma 2.3 for the output gate of F , we obtain strings
x(a), y(b) and a number ρ such that (1) LCS(x(a), y(b)) ≤ ρ and (2) LCS(x(a), y(b)) = ρ if and
only if F (a, b) evaluates to true. Tracing the proof of Lemma 2.3 one can see that ρ does not
depend on the choice of a, b, and similarly x(a) does not depend on b and y(b) does not depend on
a. Hence, we obtain n strings x(a), a ∈ A and n strings y(b), b ∈ B and a threshold ρ on which
we can invoke Lemma C.1. This yields strings x, y and a threshold ρ′ such that (1) LCS(x, y) ≤ ρ′
and (2) LCS(x, y) = ρ′ if and only if there exist a ∈ A, b ∈ B such that F (a, b) evaluates to true,
so indeed from the LCS of the constructed strings x, y one can solve the Formula-Pair instance.
The strings x, y use alphabet size 5σ 2 + 4. Using the length bound of Lemma 2.3, x, y have
length O nτ |F |(1 + 7/τ )depth(F ) . Here, as in Lemma 2.3, we may choose any sufficiently large
σ > 0 and set τ = (log σ)1/4 .
Since a priori the depth of F could be as large as |F |, the factor (1 + 7/τ )depth(F ) in our length
bound is not yet satisfactory. Thus, as a preprocessing before the above construction, we decrease
the depth of F using the following result of Bonet and Buss [90, 25].
26

<!-- page 27 -->

Theorem C.2 ([25]). Let F be a formula. Then for all k ≥ 2 there is an equivalent formula F ′ with
depth at most (3k ln 2) log |F | and size |F ′ | ≤ |F |1+1/(1+log(k−1)) . This fomula can be constructed in
time |F |O(1) .
√
Given an instance (F, A, B) of Formula-Pair, we first run Bonet-Buss with k := τ to obtain
a formula F ′ . Then we use the reductionto LCS on F ′ . This yields an equivalent LCS instance
′
(x, y) of length O nτ |F ′ |(1 + 7/τ )depth(F ) , as shown above. Plugging in the bounds on |F ′ | from
Bonet-Buss, we obtain length




O n|F |1+1/(1+log(k−1)) (1 + 7/τ )(3k ln 2) log |F | = O n|F |1+1/(1+log(k−1))+O(k/τ ) ,


√
where we used 1 + x ≤ exp(x) = 2O(x) . Since we set k = τ , this evaluates to O n|F |1+O(1/ log τ ) .

Since furthermore τ = (log σ)1/4 , we obtain length bound O n|F |1+O(1/ log log σ) . This finishes the
proof of Theorem 2.2 for any sufficiently large alphabet size 5σ 2 + 4. Note that it suffices to prove
Theorem 2.2 for sufficiently large alphabet size, since for any constant C ≥ 2 and any alphabet size
|Σ| ≤ C the statement follows from the reduction in [3], which has length bound O(n|F |O(1) ) (with
an explicit constant O(1) in the exponent).
Now assume that LCS on strings of length n over sufficiently large alphabet size σ has an
algorithm in time T (n) = O(n2 /(log n)2+C/ log log σ+ε ), for some sufficiently large C > 0 and any
(small) ε > 0. Then from the above reduction we obtain an algorithm for Formula-Pair on formulas
of size |F | ≤ (log n)1+o(1) in time




T n(log n)1+O(1/ log log σ)+o(1) = O n2 (log n)2+O(1/ log log σ)+o(1) /(log n)2+C/ log log σ+ε ,

which is bounded by O n2 /(log n)ε/2 . This would break the Formula-Pair barrier, so we obtain a
conditional lower bound for LCS of n2 /(log n)2+O(1/ log log σ)+o(1) . This implies Corollary 1.6.

D

Simple Reduction to LCS using Large Alphabet

In this section we present a simpler alternative to the reduction from Formula-Pair to LCS given
in Sections 2 and C, using however a much larger alphabet size.
Theorem D.1. Formula-Pair on formulas of size s and lists of size n can √
be reduced to an
instance of LCS on two strings over alphabet of size Θ(s) of length O(n · s · 2O( log s) ), in linear
time.
Similarly as before, we construct strings x, y and a number ρ such that LCS(x, y) ≥ ρ holds
if and only if the given Formula-Pair instance (F, A, B) is satisfiable. Fix vectors a, b ∈ {0, 1}m
(where 2m is the number of inputs to F and thus m = Θ(s)). For any gate g of F , we construct
strings as follows.
Lemma D.2. We can inductively construct, for each gate g of F , strings x(g) = x(g, a) and y(g) =
y(g, b) over alphabet size |Fg | (where Fg is the subformula of F below g) and a number ρ(g) such
that for L(g) := LCS(x(g), y(g)) we have L(g) ≤ ρ(g), with equality if and only if gate g evaluates
to true on input (a, b) to F . Moreover, we have |x(g)| = |y(g)| = n(g) ≤ (depth(Fg ) + 1) · |Fg |.
Proof. Consider any gate g of F , and write for readability x = x(g) and similarly define y, n, L, ρ.
If g has fanin 2, write g1 , g2 for the children of g. Moreover, let x1 = x(g1 ) and similarly define
y1 , n1 , L1 , ρ1 and x2 , y2 , n2 , L2 , ρ2 .
27

<!-- page 28 -->

Input Gate The base case is an input bit ai to F (input bits bj are symmetric). Interpreting ai
as a string of length 1 over alphabet {0, 1}, note that LCS(ai , 1) = ai . Hence, the strings x = ai
and y = 1, with n = ρ = 1, trivially simulate the input bit ai .
AND Gates For a gate g = (g1 ∧ g2 ) we first replace all symbols in x2 , y2 by fresh symbols, so
that x1 y1 and x2 y2 use disjoint alphabets. We then construct
x := x1 x2
y := y1 y2 .
Since these strings use disjoint alphabets, it is easy to see that LCS(x, y) = LCS(x1 , y1 )+LCS(x2 , y2 ).
Hence, inductively we obtain for ρ := ρ1 + ρ2 that L = LCS(x, y) ≤ ρ, with equality if and only if
both gates g1 and g2 evaluate to true. Thus, we correctly implemented an AND gate. Note that
n = n1 + n2 .
OR Gates For a gate g = (g1 ∨ g2 ) we first replace all symbols in x2 , y2 by fresh symbols, so that
x1 y1 and x2 y2 use disjoint alphabets. Without loss of generality assume that ρ1 ≥ ρ2 (the other
case is symmetric). Let δ := ρ1 − ρ2 and let $ be a fresh symbol. We construct the strings
x := x1

x2 $δ

y := y2 $δ y1 .
Since x1 , y1 and x2 $δ , y2 $δ use disjoint alphabets, we can either match symbols in the first pair or
in the second. It follows that LCS(x, y) = max{LCS(x1 , y1 ), LCS(x2 $δ , y2 $δ )}. Since $ is a fresh
symbol, we moreover have LCS(x2 $δ , y2 $δ ) = LCS(x2 , y2 )+δ. Together, we obtain L = LCS(x, y) ≤
max{ρ1 , ρ2 + δ} = ρ1 , with equality if and only if g1 or g2 evaluate to true. Thus, we correctly
implemented an OR gate. Note that n = n1 + n2 + |ρ1 − ρ2 | and ρ = max{ρ1 , ρ2 }.
Analyzing the Length Note that both for AND and OR gates, the inequality ρ ≤ ρ1 + ρ2 holds,
with the base case ρ = 1 at input gates. This yields ρ(g) ≤ |Fg |. Furthermore, for both AND and
OR gates the inequality n ≤ n1 + n2 + |ρ1 − ρ2 | holds. Since |ρ1 − ρ2 | ≤ ρ1 + ρ2 ≤ |Fg1 | + |Fg2 | ≤ |Fg |,
we obtain n ≤ n1 + n2 + |Fg |. It now follows inductively that n ≤ (depth(Fg ) + 1) · |Fg |, since
n ≤ n1 + n2 + |Fg | ≤ (depth(Fg1 ) + 1) · |Fg1 | + (depth(Fg2 ) + 1) · |Fg2 | + |Fg |

≤ depth(Fg ) · (|Fg1 | + |Fg2 |) + |Fg | ≤ (depth(Fg ) + 1) · |Fg |.

Also note that since each gate can introduce one new symbol the alphabet size is at most |Fg |. This
proves the lemma.
Outer OR We now finish the reduction from Formula-Pair to LCS using alignment gadgets, see
Lemma D.2. For any vectors a ∈ A, b ∈ B, invoking Lemma D.2 for the output gate of F , we
obtain strings x(a), y(b) and a number ρ such that LCS(x(a), y(b)) ≤ ρ, with equality if and only if
F (a, b) evaluates to true. Tracing the proof of Lemma D.2 one can see that ρ does not depend on
the choice of a, b, and similarly x(a) does not depend on b and y(b) does not depend on a. Hence,
we obtain n strings x(a), a ∈ A and n strings y(b), b ∈ B and a threshold ρ on which we can invoke
Lemma C.1. This yields strings x, y and a threshold ρ′ such that LCS(x, y) ≤ ρ′ , with equality if
28

<!-- page 29 -->

and only if there exist a ∈ A, b ∈ B such that F (a, b) evaluates to true, so indeed from the LCS of
the constructed strings x, y one can solve the Formula-Pair instance. Note that so far the resulting
strings have length O(n · depth(F ) · |F |).
Finalizing the Proof Since a priori the depth of F could be as large as |F |, the factor depth(F )
in our length bound is not yet satisfactory. Thus, as a preprocessing before the above construction,
we use the depth reduction of Bonet and Buss, see Theorem
C.2. Given an instance (F, A, B) of
√
Θ( log(|F |))
to obtain a formula F ′ . Then we
Formula-Pair, we first run Bonet-Buss with k := 2
′
use the reduction to LCS on F . This yields an equivalent LCS instance (x, y) of length O(n ·
(depth(F ′ ) + 1) · |F ′ |),√as shown above. Plugging in the bounds on |F ′ | from Bonet-Buss, we obtain
length O(n · |F | · 2Θ( log(|F |)) ) = O(n · |F |1+o(1) ). This finishes the proof of Theorem D.1.

E

The Reduction to Pattern Matching

In this section we reduce the Formula-Pair problem to regular expression pattern matching. By
the reductions in Section B, this gives us barriers based on F1 -Formula-SAT and proves Theorem 1.5.
Let us start with a formal definition of the problem.
Regular Expression Pattern Matching Regular expressions over alphabet Σ are inductively
defined as follows. For any c ∈ Σ, the regular expression c matches only itself, i.e., L(c) = {c}.
The regular expression r = r1 ◦ r2 matches all concatenations of strings matched by r1 and r2 , i.e.,
L(r) = {s1 s2 | s1 ∈ L(r1 ), s2 ∈ L(r2 )}. We often abbreviate r1 ◦ r2 by r1 r2 . The regular expression
r = r1 |r2 matches the union of all strings matched by r1 and r2 , i.e., L(r) = L(r1 ) ∪ L(r2 ). Finally,
the regular expression r = r1∗ matches all sequences of strings matched by r1 , i.e., L(r) = {s1 . . . sk |
k ≥ 0, s1 , . . . , sk ∈ L(r1 )}. The regular expression pattern matching problem is, given a string t (the
text) and a regular expression p (the pattern), to decide whether p matches some substring of t.
We will prove the following theorem. Together with the reductions in Section B, this implies
Theorem 1.5.
Theorem E.1. Formula-Pair on formulas of size s and lists of size n can be reduced to an
instance of regular expression pattern matching with text length and pattern size O(n · s log s), in
linear time. The alphabet is Σ = {0, 1}.
In our construction, we will use the following encoding of numbers. For any integer m ≥ 0, let
m1 . . . mk be its representation in binary, where k = max{1, ⌊log 2 m⌋ + 1}. We define the string
#(m) := 1 1 0 m1 0 m2 0 . . . 0 mk−1 0 mk 0 1 1.
We will make use of the following simple observation: For any x, y1 , . . . , yℓ ≥ 0 the string #(x)
is a substring of the concatenation #(y1 ) . . . #(yk ) if and only if we have x = yi for some i. (In
particular, #(x) cannot appear as a suffix of #(y1 ) followed by a prefix of #(y2 ).)
For each gate g in the formula F and vectors a, b ∈ {0, 1}k we design a text t(g, a) and a pattern
p(g, b) such that p(g, b) matches t(g, a) if and only if on input a, b in the formula F gate g evaluates
to true. In our construction we ensure that for any gate g of height h the text t(g, a) can be written
as #(y1 ) . . . #(yk ) for some k ≥ 1 and 0 ≤ y1 , . . . , yk ≤ h + 1.
29

<!-- page 30 -->

In the following, fix a gate g of F with children g1 and g2 . For simplicity, we write t = t(g, a), p =
p(g, b) and similarly t1 , p1 , t2 , p2 for g1 and g2 . We denote by h the height of g, i.e., the length of
the longest path from g to any of its descendants.
Input Gates The base case is an input bit ai to F (input bits bj are symmetric). Since ai is a
number in {0, 1}, we can set t := #(ai ) and p := #(1). Then the pattern matches the text if and
only if ai = 1. Note that the height is 0 and we indeed only used #(0) and #(1).
AND Gates Consider a gate g = (g1 ∧ g2 ) of height h ≥ 1 and let h′ := h + 1. We set
t := t1 #(h′ ) t2 ,

p := p1 #(h′ ) p2 .
Since t1 and t2 only contain #(y) with y < h′ , the only way for p to match t is to match the
occurences of #(h′ ) in p and t in the natural way, so that p1 must match t1 and p2 must match t2 .
Thus, we correctly implemented an AND gate.
OR Gates Consider a gate g = (g1 ∨ g2 ) of height h ≥ 1 and let h′ := h + 1. We set
t := t1 #(h′ ) t2 ,

p := p1 #(h′ ) (0|1)∗

(0|1)∗ #(h′ ) p2 .

This time we can choose which of the two sub-expressions of p we want to match. In any case, there
is an occurence of #(h′ ) in the pattern, and since t1 and t2 only contain #(y) with y < h′ , the
only way for the pattern to match t is to match the occurences of #(h′ ) in p and t in the natural
way. Since (0|1)∗ matches any string, in particular t1 and t2 , we obtain that p1 matches t1 or p2
matches t2 , depending on which sub-expression of p we choose. Thus, we correctly implemented an
OR gate.
This finishes the construction of the texts t(g, a) and patterns p(g, a).
Outer OR Invoking this construction for the root r of F yields a text t(a) := t(r, a) and a pattern
p(b) := p(r, b) such that p(b) matches t(a) if and only if on input a, b the formula F evaluates to
true. Let H be two plus the height of r, and let A = {a1 , . . . , an } and B = {b1 , . . . , bn }. We
construct the final text and pattern as
t = #(H) t(a1 ) #(H)

...

#(H) t(an ) #(H),

p = #(H) p(b1 ) #(H)

...

#(H) p(bn ) #(H).

To match the pattern p, we have to choose one of the sub-expressions pj := #(H) p(bj ) #(H). Since
#(H) does not appear in any t(ai ), in order to match pj to a substring of t we must match the two
occurences of #(H) in pj to two top-level occurences of #(H) in t, and thus we must match p(bj ) to
some t(ai ). As this is possible if and only if F is satisfied by ai , bj , we have constructed an instance
of regular expression pattern matching that is equivalent to the given instance of Formula-Pair.
Note that the height of any gate is bounded by O(s), where s is the size of the formula F , and
thus any encoding #(y) appearing in our construction has length O(log s). The text length and
pattern length both satisfy a recursion of the type
L ≤ L1 + L2 + O(log s),
30

<!-- page 31 -->

which yields the bound O(s log s) on the length of t(a) and the size of p(b). In total, we obtain
length and size O(n · s log s). This proves Theorem 1.5.

F

The Reduction to Fréchet Distance

In this section we reduce the Ineq-Formula-Pair problem to the Fréchet distance. By the
reductions in Section B, this gives us barriers based on F2 -Formula-SAT and proves Theorem 1.4.
Familiarity with the SETH-hardness proof of [27] will help to understand our proof. Let us start
with a formal definition of the problem.
Fréchet distance A curve P is a sequence (p1 , . . . , pn ) of points in the Euclidean space Rd .
We call p1 , . . . , pn the vertices of P . In the construction in this paper, we will always work in
dimension d = 2, and the coordinates of vertices will be rationals with small bitlength. For two
curves P = (p1 , . . . , pn ) and Q = (q1 , . . . , qm ) and any monotone non-decreasing and onto functions
φ : {1, . . . , n + m} → {1, . . . , n} and ψ : {1, . . . , n + m} → {1, . . . , m} we call τ = (φ, ψ) a traversal.
We say that at time step t the traversal τ is at vertex pφ(t) in P and at vertex qψ(t) in Q. Observe
that τ describes one way of a man and its dog walking along P and Q, respectively, from their
starting vertices to their ending vertices, where in each time step man and dog may step to the
next vertex on their curve or stay at their current vertex. We say that the traversal τ stays in
distance δ if at any time step 1 ≤ t ≤ n + m the current vertices pφ(t) and qψ(t) are within distance δ
of each other. Finally, the Fréchet distance dF (P, Q) is the minimal number δ such that there exists
a traversal τ of P, Q staying in distance δ.
We will prove the following theorem. Together with the reductions in Section B, this implies
Theorem 1.4.
Theorem F.1. Ineq-Formula-Pair on formulas of size s and lists of size n can be reduced to
an instance of Fréchet on two curves of length O(n · s), in linear time.

F.1

Outline of the Reduction

We say that curves P, Q are δ-placed if
• the x-coordinates of all vertices of P and Q are in [−δ, δ],
• the y-coordinates of all vertices of P are in [1 − δ2 , 1 + δ2 ], and
• the y-coordinates of all vertices of Q are in [−δ2 , δ2 ].
This ensures that P is contained in a small region around the point (1, 0), and Q is close to (0, 0).
Lemma F.2. For each gate g in the formula F and vectors a, b ∈ {−M, . . . , M }k , and any δ ∈
(0, 1), we design curves P = Pδ (g, a) and Q = Qδ (g, b) such that
(P1) dF (P, Q) ≤ 1 if and only if on input a, b in the formula F gate g evaluates to true, and
(P2) P, Q are δ-placed.
The curves P and Q have O(|F |) vertices. Coordinates of the vertices of P, Q use O(log(1/δ) + h +
log M ) bits, where h is the height of gate g.
31

<!-- page 32 -->

Property (P1) makes sure that we correctly simulate formula F by the Fréchet distance. We
use property (P2) to gain control over the Fréchet distance of recursively defined subcurves. Before
proving the above lemma, in Section F.2 we first design the “outer OR” of the reduction from IneqFormula-Pair, which chooses a pair of vectors a ∈ A, b ∈ B. We then show how to implement
the different types of gates in Section F.3, proving Lemma F.2.

F.2

Outer OR

Let r be the root of formula F and consider the curves Pa := P1/16 (r, a) and Qb := Q1/16 (r, b)
for vectors a ∈ A, b ∈ B given by Lemma F.2. We want to construct curves P, Q such that
dF (P, Q) ≤ 1 if and only if there exist a ∈ A, b ∈ B with dF (Pa , Qb ) ≤ 1, which holds if and only if
F (a, b) evaluates to true.
We study this situation slightly more generally as follows. Let δ ∈ (0, 1/2] and let P1 , Q1 , . . . , Pk , Qk
be δ/8-placed curves. Define the following auxiliary points:
sP := (−δ/2, 1 − δ2 ),

sQ := (−δ/2, δ2 ),

bP := (−δ/2, 1),

bQ := (−δ/2, 0),

eP := (δ/2, 1),

eQ := (δ/2, 0).

tP := (δ/2, 1 − δ2 ),

tQ := (δ/2, δ2 ),

s∗Q := (−δ/2, −δ2 ),
t∗Q := (δ/2, −δ2 ),

We define the curve P = PδOR (P1 , . . . , Pk ) as
P :=

k
ℓ=1 sP ◦ bP ◦ Pℓ ◦ eP ◦ tP ,

i.e. we start with vertices in sP and bP , then follow P1 , then add vertices at eP , tP , sP , and bP ,
then follow P2 and so on. We define Q = QOR
δ (Q1 , . . . , Qk ) by
 ∗
k
Q := sQ ◦ s∗Q ◦
ℓ=1 bQ ◦ Qℓ ◦ eQ ◦ tQ ◦ tQ .
Lemma F.3 (Outer OR). For any δ ∈ (0, 1/2] and any δ/8-placed curves P1 , Q1 , . . . , Pk , Qk , the
curves P = PδOR (P1 , . . . , Pk ) and Q = QOR
δ (Q1 , . . . , Qk ) constructed above are δ-placed and satisfy
dF (P, Q) ≤ 1 if and only if there are 1 ≤ i, j ≤ k with dF (Pi , Qj ) ≤ 1.
Note that for δ := 1/2 the curves Pa = P1/16 (r, a) and Qb = Q1/16 (r, b) satisfy the requirements
of the above construction, in particular they are (δ/8 = 1/16)-placed, and thus we obtain curves
P, Q such that dF (P, Q) ≤ 1 if and only if there exist a ∈ A, b ∈ B with dF (Pa , Qb ) ≤ 1. This
yields a reduction from Ineq-Formula-Pair on n vectors and formula F to the Fréchet distance
on curves of length O(n|F |), as Pa , Qb have length O(|F |) by Lemma F.2. The hypothesis that
for |F | = (log n)1+o(1) Ineq-Formula-Pair has no n2 /(log n)ε algorithm now implies that (the
′
decision variant of) the Fréchet distance has no n2 /(log n)2+ε algorithm, as desired. Also note
that the coordinates of the vertices of the resulting curves use O(depth(F ) + log M ) bits. Since
by Bonet and Buss’ depth reduction (Theorem C.2) we can assume that depth(F ) = Oε (log |F |)
and |F | = (log n)1+o(1) , these coordinates fit into a memory cell for any M = nO(1) . Thus, the
reduction not only works on the Real RAM, but also on the Word RAM.
Proof of Lemma F.3. From the definition of the auxiliary points and since Pℓ , Qℓ are δ/8-placed
for all ℓ, it follows that P, Q are δ-placed.
32

<!-- page 33 -->

If there are i, j such that dF (Pi , Qj ) ≤ 1 then we find a traversal of P, Q staying in distance 1 as
(ℓ)
follows. Denote by sP the ℓ-th occurrence of sP on curve P , i.e. the copy of sP that comes before
(ℓ) (ℓ) (ℓ) (ℓ) (ℓ)
Pℓ in P , and similarly define tP , bP , eP , bQ , eQ . Note that sQ , s∗Q , tQ , t∗Q appear only once in Q,
(1)

so they do not get a superscript. Any traversal of P, Q starts in sP and sQ . We stay at sQ and walk
(i)
(i)
along P until we reach sP , staying in distance 1 by claim (U1) below. Then we stay in sP and walk
(j)
(i)
along Q until we reach bQ , using (U2). Now we step to bP in P , using (U4). After a simultaneous
step in P, Q we are at the starting vertices of Pi , Qj and we proceed by optimally traversing them,
staying in distance 1 since dF (Pi , Qj ) ≤ 1 by assumption. We then make a simultaneous step to
(i)
(j)
(i)
eP and eQ , using (U4). Now we make a single step in P to tP , using (U3). We stay in this vertex
of P and walk along Q until its final vertex tQ , using (U3). Finally, we stay in tQ and walk along
(k)
P until its final vertex tP , using (U1). It remains to show the following.
Claim F.4. We have the following upper bounds on vertex distances:
(U1) sQ and tQ are in distance 1 of any vertex of P ,
(U2) sP is in distance 1 of any vertex of Q except for t∗Q ,
(U3) tP is in distance 1 of any vertex of Q except for s∗Q ,
(U4) bP and bQ as well as eP and eQ are in distance 1.
Proof. For (U1) consider sQ , since tQ is symmetric. Observe that every vertex on P has y-coordinate
at most 1 + (δ/8)2 ≤ 1 + δ2 /2 (and at least 1 − δ2 ) and x-coordinate at most δ/2 (and at least
−δ/2). Thus, their squared distance to sQ = (−δ/2, δ2 ) is at most
2
2
− δ/2 − δ/2 + (1 + δ2 /2) − δ2 = δ2 /16 + (1 − δ2 + δ4 /4) ≤ 1,
proving (U1). For (U2) we similarly use that all vertices on Q, except for s∗Q and t∗Q have ycoordinate at least −(δ/8)2 to obtain that sP is within distance 1 of these vertices. The distance
between sP and s∗Q is exactly 1 by definition. The only exception is t∗Q , and indeed the distance
between sP and t∗Q is larger than 1. Thus, sP is in distance 1 of any point on Q except for t∗Q .
Claim (U3) is symmetric, and (U4) follows directly from the definitions.
For the other direction, assume that there is a traversal τ of P, Q staying in distance 1. We
want to show that dF (Pi , Qj ) ≤ 1 holds for some i, j. Consider any time step where τ is in s∗Q on
(i)

Q and some vertex x on P . By claim (L1) below, x has to be a vertex sP , for some 1 ≤ i ≤ k. We
(i)
(i)
(i)
may stay in sP for some steps, until we at some point in time make a step from sP to bP . We
(j)
show that at this time we have to be in bQ for some j. Consider the possible vertices of Q where
(i)

we can be. By (L1) and (L2) the points s∗Q and t∗Q are too far from bP . The vertices sQ and tQ are
(i)

both in distance 1 of bP , however, we already visited s∗Q , and since sQ comes before s∗Q on Q we
(i)

cannot be at sQ . Moreover, since the point in time at which we were at (sP , s∗Q ) we only visited
(i)

(i)

the vertices sP and bP on P , and both have distance more than 1 to the bottleneck vertex t∗Q , so
(i)

we cannot walk to tQ , which comes after t∗Q on Q. Hence, while being at bQ we cannot be at sQ
or tQ . The vertices of Qℓ , for any ℓ, are also not in distance 1, by (L3). Finally, by (L5) we cannot
33

<!-- page 34 -->

(j)

(i)

be at eQ . Hence, while vertex at bP we have to be at bQ for some j. By (L3) and (L4), the next
step is simultaneous in P and Q, since the distance from bP to Qj as well as from bQ to Pi is larger
than 1. After this simultaneous step we are at the starting vertices of Pi and Qj . By (L3) and (L4),
we cannot make a step to eP while staying in Qj (or to eQ while staying in Pi ), since their distance
is too large. Hence, τ contains a traversal τ ′ of Pi , Qj , after which it makes a simultaneous step to
(i) (j)
eP , eQ . Since τ stays in distance 1, also τ ′ stays in distance 1, and we obtain dF (Pi , Qj ) ≤ 1 (for
some i, j). It remains to show the following.
Claim F.5. We have the following lower bounds on vertex distances:
(L1) s∗Q has distance larger than 1 to any vertex of P except sP ,
(L2) t∗Q has distance larger than 1 to any vertex of P except tP ,
(L3) bP and eP have distance larger than 1 to any vertex of Qℓ for all ℓ,
(L4) bQ and eQ have distance larger than 1 to any vertex of Pℓ for all ℓ,
(L5) bP and eQ as well as eP and bQ have distance larger than 1,
Proof. For (L1), since s∗Q = (−δ/2, −δ2 ) and all vertices of P have y-coordinate at least 1 − δ2 ,
vertices in distance 1 to s∗Q on P are of the form (−δ/2, 1 − δ2 ), which is sP . (L2) is symmetric.
For (L3), since Pℓ , Qℓ are δ/8-placed, all vertices of Qℓ have x-coordinate at least −δ/8 and
y-coordinate at most (δ/8)2 . Thus, the squared distance from any point on Qℓ to bP = (−δ/2, 1) is
2
2
≥ δ/2 − δ/8 + 1 − δ2 /64 ≥ δ2 /16 + (1 − δ2 /32 + δ4 /4096) > 1.
The distance to eP is symmetric, as is (L4). Finally, (L5) follows directly from the definitions.

F.3

Implementing Gates

It remains to prove Lemma F.2, i.e. to implement the different types of gates. Let δ ∈ (0, 1) and
fix vectors a ∈ A, b ∈ B.
Comparison Gate First consider an input gate g to formula F , i.e., a comparison [ai ≤ bi ] for
some i. We define P = Pδ (g, a) and Q = Qδ (g, b) as degenerate
curves consisting only

 of a single
2
2
vertex. Specifically, we set Pδ (g, a) := (0, 1 + δ ai /M ) and Qδ (g, b) := (0, δ bi /M ) .
Lemma F.6 (Comparison Gate). The curves Pδ (g, a), Qδ (g, b) satisfy (P1) and (P2).
Proof. Note that the Fréchet distance of degenerate curves consisting of one vertex is simply the
Euclidean distance of the vertices. Thus, dF (P, Q) = |1 + δ2 ai /M − δ2 bi /M |. Since δ ∈ (0, 1) and
ai , bi ∈ {−M, . . . , M }, this equals 1 + δ2 (ai − bi )/M , which is at most 1 if and only if ai ≤ bi . This
shows property (P1), and property (P2) holds by definition.

34

<!-- page 35 -->

AND Gate Now consider an AND gate g in F , and let g1 , . . . , gk be the children of g in F . We
(inductively) construct the curves Pℓ := Pδ′ (gℓ , a) and Qℓ := Qδ′ (gℓ , b) for 1 ≤ ℓ ≤ k and some
δ′ ≤ δ/4. The only facts about Pℓ , Qℓ that we will use are property (P1) and that Pℓ , Qℓ are
δ/4-placed (by (P2) and δ′ ≤ δ/4). We translate the curves Pℓ and Qℓ along the x-axis by +δ/2,
if ℓ is odd, and by −δ/2, if ℓ is even, i.e. we add δ/2 or −δ/2 to the x-coordinate of every vertex
of Pℓ and Qℓ . This results in curves Pℓ′ , Q′ℓ . Note that translating both curves does not change the
Fréchet distance and thus we have dF (Pℓ′ , Q′ℓ ) ≤ 1 if and only if gℓ evaluates to true, by (P1). We
form the final curve P = Pδ (g, a) by concatenating P1′ , . . . , Pk′ , and similarly we form Q = Qδ (g, b)
by concatenating Q′1 , . . . , Q′k .
Lemma F.7 (AND Gate). The curves Pδ (g, a), Qδ (g, b) satisfy (P1) and (P2).
Proof. The curves Pℓ , Qℓ are δ/4-placed since δ′ ≤ δ/4. The translation by ±δ/2 is sufficiently
small to ensure that Pℓ′ , Q′ℓ and thus also P, Q are δ-placed, so property (P2) holds.
For property (P1), first note that if all gates gi evaluate to true then dF (Pi′ , Q′i ) ≤ 1 for all i, so
we can traverse Pi′ and Q′i staying in distance 1, and concatenating these traversals for 1 ≤ i ≤ k
′ , Q′ ) yields a traversal of P, Q
(with a simultaneous step in P and Q between any Pi′ , Q′i and Pi+1
i+1
staying in distance 1. Thus, if g evaluates to true then dF (P, Q) ≤ 1.
For the other direction, we first show that any vertices in Pi′ and Q′j have distance larger than 1
for i 6≡ j (mod 2). For simplicity, say that i is odd and j is even. Since we translated Pi by +δ/2
and Qj by −δ/2, we obtain that Pi′ is contained in [δ/2 − δ/4, δ/2 + δ/4] × [1 − (δ/4)2 , 1 + (δ/4)2 ]
and Q′j is contained in [−δ/2 − δ/4, −δ/2 + δ/4] × [−(δ/4)2 , (δ/4)2 ]. Thus, a lower bound for the
distance between any two vertices of Pi′ and Q′j is the distance from the lower left corner of the
region containing Pi′ and the upper right corner of the region containing Q′j , which is


2 1/2
2
1/2
= δ2 /4 + 1 − 4(δ/4)2 + 4(δ/4)4
(δ/2 − δ/4) − (−δ/2 + δ/4) + 1 − 2(δ/4)2
> 1.

Hence, no traversal staying in distance 1 can be simultaneously in Pi′ and Q′j , for i 6≡ j (mod 2).
Now assume that dF (P, Q) ≤ 1 and consider a traversal τ realizing the Fréchet distance. Note
that τ starts in the first vertices of P1′ , Q′1 , and consider the first time where the traversal reaches
a vertex in P2′ or Q′2 . Observe that the previous move has to be a simultaneous move from the
last vertices in P1′ , Q′1 to the first vertices in P2′ , Q′2 , since we cannot be in P1′ and Q′2 at the same
time, or symmetrically in P2′ and Q′1 , as 1 6≡ 2 (mod 2). Thus, we can split the traversal τ into a
traversal of P1′ , Q′1 and a traversal of the remainder of P, Q. Proceeding inductively, we can split τ
into traversals τi of Pi′ , Q′i for 1 ≤ i ≤ k. Since τ stays in distance 1, each τi also stays in distance 1.
This shows dF (Pi′ , Q′i ) ≤ 1, and by dF (Pi′ , Q′i ) = dF (Pi , Qi ) and property (P1) we obtain that all
gates gi evaluate to true. This proves correctness of the AND gate construction.
OR Gate We simulate OR gates by a combination of all previous constructions in Section F.
Consider any OR gate g in F , and let g1 , . . . , gk be its children in F , corresponding to subformulas
F1 , . . . , Fk . Note that the outer OR construction from Lemma F.3 does not suffice to simulate an
OR gate: If we recursively build curves Pℓ , Qℓ simulating subformula Fℓ and combine them with
an outer OR then it is possible to pair up Pi and Qj for i 6= j, thus not respecting the structure
of the formula. To prevent these mismatches, we adapt the subformulas Fℓ as follows. We first
present a sketch of this adaptation, for the details see Lemma F.8. We consider the auxiliary
formulas Fℓ′ = [ℓ ≤ ℓ] ∧ Fℓ ∧ [−ℓ ≤ −ℓ], and construct the curves Pℓ′ , Q′ℓ that our conversion yields
35

<!-- page 36 -->

when run on Fℓ′ . This might seem like an unnecesasry step at first sight, since the comparisons
[ℓ ≤ ℓ], [−ℓ ≤ −ℓ] are always true, so Fℓ′ simplifies to Fℓ . However, consider i 6= j and the
constructed curves Pi′ and Q′j . Then the comparison [ℓ ≤ ℓ] becomes [i ≤ j], since the left operand
of the comparison is encoded in P and the right operand is encoded in Q. Similarly, the comparsion
[−ℓ ≤ −ℓ] becomes [−i ≤ −j]. One of these comparisons is wrong for i 6= j, so that the AND
evaluates to false. Thus, the Fréchet distance of Pi′ , Q′j is larger than 1 for any i 6= j.
This trick allows us to use the outer OR construction to finish our OR gate construction. Indeed,
running Lemma F.3 on the curves P1′ , Q′1 , . . . , Pk′ , Q′k yields curves P, Q such that dF (P, Q) ≤ 1 if
and only if there are i, j with dF (Pi′ , Q′j ) ≤ 1. Since by Lemma F.8.(1) we must have i = j for
Fréchet distance at most 1, we have dF (P, Q) ≤ 1 if and only if dF (Pℓ′ , Q′ℓ ) ≤ 1 for some ℓ. Finally,
by Lemma F.8.(2) we have dF (P, Q) ≤ 1 if and only if Fℓ evaluates to true for some ℓ. Hence, we
correctly simulated the OR gate g.
Lemma F.8. We can construct δ/8-placed curves P1′ , Q′1 , . . . , Pk′ , Q′k satisfying (1) dF (Pi′ , Q′j ) > 1
for i 6= j and (2) dF (Pℓ′ , Q′ℓ ) ≤ 1 if and only if subformula Fℓ evaluates to true on (a, b).
Proof. As above, Pℓ′ , Q′ℓ are the curves produced by our conversion when run on Fℓ′ = [ℓ ≤ ℓ] ∧
Fℓ ∧ [−ℓ ≤ −ℓ]. More precisely, we recursively construct the curves Pℓ := Pδ/32 (gℓ , a) and Qℓ :=
Qδ/32 (gℓ , b) for 1 ≤ ℓ ≤ k. For the comparison [ℓ ≤ ℓ] we construct the degenerate curves CℓP =


(0, 1 + δ/32 · ℓ/n) and CℓQ = (0, δ/32 · ℓ/n) (see Lemma F.6), and for the comparison [−ℓ ≤ −ℓ]


we construct the degenerate curves DℓP = (0, 1 − δ/32 · ℓ/n) and DℓQ = (0, −δ/32 · ℓ/n) . Note
that CℓP , CℓQ , DℓP , DℓQ , Pℓ , Qℓ are δ/32-placed, so we can apply the AND gate construction resulting
in δ/8-placed curves, see Lemma F.7. Specifically, we translate CℓP , CℓQ , DℓP , DℓQ by +δ/16 along
the x-axis, and Pℓ , Qℓ by −δ/16. Finally, we concatenate (the translated versions of) CℓP , Pℓ , DℓP
to obtain Pℓ′ and we concatenate (the translated versions of) CℓQ , Qℓ , DℓQ to obtain Q′ℓ . Note that
Pℓ′ , Q′ℓ are δ/8-placed, so we can indeed use Lemma F.3 to combine P1′ , Q′1 , . . . , Pk′ , Q′k to δ-placed
curves P, Q as in the paragraph before the lemma.
From the correctness of the AND gate and comparison constructions we obtain that Pℓ′ , Q′ℓ
simulate Fℓ′ , i.e. dF (Pℓ′ , Q′ℓ ) ≤ 1 if and only if Fℓ′ (a, b) evaluates to true. Since Fℓ′ simplifies to Fℓ ,
this yields (2). For (1), consider Pi′ , Q′j for i 6= j. One of the comparisons [i ≤ j], [−i ≤ −j] is
wrong, so we obtain dF (CiP , CjQ ) > 1 or dF (DiP , DjQ ) > 1. Since CiP , CjQ and DiP , DjQ and Pi , Qj
are δ/32-placed (before we translate them), the correctness argument of the AND gate, Lemma F.7,
yields dF (Pi′ , Q′j ) > 1, as desired.
Finishing the Proof We obtain Lemma F.2 by putting together the gate constructions of this
section. This yields a recursive construction of curves Pδ (g, a), Qδ (g, a) simulating gate g, correctness follows from a simple inductive argument. Note that if g has fanin k then we add O(k) vertices
to the recursively constructed curves corresponding to the k children, yielding a bound of O(|F |)
on the length of the constructed curves. Finally, for the factor by which we decrease δ from one
gate to its children we note that 1/32 is sufficiently small. Thus, we may produce δ/32d -placed
curves for each gate in depth d below g. The coordinates of our AND and OR gate constructions
thus use O(log(1/δ) + d) bits. The only exception is the comparison gate construction, for which
we need O(log M ) = O(log n) additional bits to represent numbers in {−M, . . . , M }. Together this
yields Lemma F.2.

36
