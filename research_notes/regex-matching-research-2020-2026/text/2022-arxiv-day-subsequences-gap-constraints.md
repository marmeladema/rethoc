---
file: ../pdfs/2022-arxiv-day-subsequences-gap-constraints.pdf
citation: "Subsequences With Gap Constraints: Complexity Bounds for Matching and Analysis Problems; Day, Kosche, Manea, Schmid; arXiv:2206.13896; 2022"
doi: 
source: "https://arxiv.org/pdf/2206.13896"
topics: [semantics_theory_access]
pages: 44
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Subsequences With Gap Constraints: Complexity Bounds for
Matching and Analysis Problems
Joel D. Day1 , Maria Kosche2 , Florin Manea2 , and Markus L. Schmid3
1

Loughborough University, UK, J.Day@lboro.ac.uk
Computer Science Department, Universität Göttingen, Germany,
maria.kosche@cs.uni-goettingen.de,
florin.manea@cs.informatik.uni-goettingen.de
3
Humboldt-Universität zu Berlin, Germany, MLSchmid@MLSchmid.de

arXiv:2206.13896v1 [cs.CC] 28 Jun 2022

2

Abstract
We consider subsequences with gap constraints, i. e., length-k subsequences p that can
be embedded into a string w such that the induced gaps (i. e., the factors of w between the
positions to which p is mapped to) satisfy given gap constraints gc = (C1 , C2 , . . . , Ck−1 ); we
call p a gc-subsequence of w. In the case where the gap constraints gc are defined by lower
+
2
and upper length bounds Ci = (L−
i , Li ) ∈ N and/or regular languages Ci ∈ REG, we prove
tight (conditional on the orthogonal vectors (OV) hypothesis) complexity bounds for checking
whether a given p is a gc-subsequence of a string w. We also consider the whole set of all
gc-subsequences of a string, and investigate the complexity of the universality, equivalence and
containment problems for these sets of gc-subsequences.

1

Introduction

For a string v = v1 v2 . . . vn , where each vi is a single symbol from some alphabet Σ, any string
u = vi1 vi2 . . . vik with k ≤ n and 1 ≤ i1 ≤ i2 ≤ . . . ≤ ik ≤ n is called a subsequence (or scattered
factor or subword ) of v (denoted by u  v). This is formalised by the embedding from the positions
of u to the positions of v, i. e., the increasing mapping e : {1, 2, . . . , k} → {1, 2, . . . , n} with j 7→ ij
(we use the notation u e v to denote that u is a subsequence of v via embedding e). For example,
the string a b a c b b a has among its subsequences a a a, a b c a, c b a, and a b a b b a. With respect
to a a a, there exists just one embedding, namely 1 7→ 1, 2 7→ 3, and 3 7→ 7, but there are two
embeddings for c b a.
In this paper, we are interested in subsequences with gap constraints that can be embedded
in such a way that the gaps of the embedding, i. e., the factors ve(i)+1 ve(i)+2 . . . ve(i+1)−1 between
the images of the mapping, satisfy certain properties. We begin by discussing why the concept
of classical subsequences (i. e., without gap constraints) is a central one in computer science, and
then we will motivate and describe in detail our approach.
The concept of subsequences is employed in many different areas of computer science: in formal
languages and logics (e. g., piecewise testable languages [71, 72, 48, 49, 50], or subword order and
downward closures [42, 55, 54, 78]), in combinatorics on words [65, 29, 57, 56, 69, 63, 66], for
modelling concurrency [64, 70, 20], in database theory (especially event stream processing [5, 38,
79]). Moreover, many classical algorithmic problems are based on subsequences, e. g., longest
common subsequence [8] or shortest common supersequence [62]. Note that the longest common
subsequence problem, in particular, has recently regained substantial interest in the context of
fine-grained complexity (see [18, 19, 1, 2]).
There are two main types of algorithmic problems for subsequences investigated in the literature. Firstly, matching: the problem to decide whether a string u is a subsequence of a string
v, i. e., whether u  v (the term matching is motivated by the point of view that u is a pattern that is to be matched with the string v). Secondly, the analysis problems are concerned
1

<!-- page 2 -->

with the sets SubSeq(k, v) of all length-k subsequences of a given string v. More precisely, for
given string v ∈ Σ+ and integer k ∈ N, we want to decide whether SubSeq(gc, v) = Σk (universality), or, for an additional string v 0 , whether SubSeq(gc, v) ⊆ SubSeq(gc, v 0 ) (containment)
or SubSeq(gc, v) = SubSeq(gc, v 0 ) (equivalence). For classical subsequences (as defined above),
the matching problem is trivial, while the analysis problems are well-investigated and relatively
well-understood. For instance, the equivalence problem was introduced by Imre Simon in his
PhD thesis [71], and was intensely studied in the combinatorial pattern matching community (see
[43, 36, 73, 75, 24, 28] and the references therein), before being optimally solved in 2021 [37]. In
this work, we consider these problems with respect to an extended setting of subsequences, which
we shall explain and motivate next.
Motivation for Our Setting. In the theoretical literature, problems on subsequences are
usually considered in the setting where the embeddings (as witnesses for subsequences) can be
arbitrary. This means that any subsequence u of string v is witnessed by a canonical embedding
e that greedily maps each position i of u to the leftmost occurrence of symbol ui in the suffix
ve(i−1) ve(i−1)+1 . . . vn . For example, u = a b a can be embedded into v = a b a c b b a in six different
ways, but the canonical embedding maps u to the prefix v[1..3]. This makes it often rather simple
to deal with subsequences algorithmically: matching can be decided greedily in linear time; the set
of all subsequences of a string v can be represented by a deterministic automaton of size O(|v||Σ|)
(which means that the analysis problems can be solved in polynomial time, although much more
efficient methods exist in certain cases [37]).
For practical scenarios, on the other hand, it seems reasonable to also postulate some properties with respect to the gaps that are induced by the embedding. For example, if we model the
scheduling of several threads on a single processor by shuffling several sequences into one string,
then a-priori knowledge about the scheduling strategy may tell us that the subsequences describing
the single threads will not have huge gaps (any kind of fairness property of the scheduling strategy implies this). Another example is finding alignments of bio-sequences by computing longest
common subsequences. While any common subsequence of two strings can be interpreted as an
alignment, it is questionable if this interpretation is still useful if roughly half of the positions of the
common subsequence are mapped to the beginning of the strings, while the other half is mapped
to the end of the strings, with a huge gap (say thousands of symbols) in between. This situation
should rather be seen as two individual alignments. In fact, in this scenario the optimisation goal of
finding a longest common subsequence, without further constraints, even seems counterproductive,
since it may favour alignments that are to a large extent disconnected and are therefore less likely
to describe relevant properties. In the context of complex event processing, it might be desirable
to describe the situation that between the events of a job A only events associated to a job B
appear (e. g., due to unknown side-effects this leads to a failure of job A). In this case, we are
interested in embedding a string as a subsequence such that the gaps only contain symbols from
a certain subset of the alphabet (i. e., the events associated to job B). So, in practice, it makes
sense to reason both about the length and the actual content of gaps induced by embeddings.
The large algorithmic tool box for problems based on subsequences is not always capable of
handling the practically relevant scenarios, where we are interested in subsequences that can be
embedded not just in any way, but in some specific way that is reasonable for the application
scenario. We therefore investigate basic problems on subsequences in the setting where the gaps
of the subsequences (or rather of the embeddings) have certain constraints.
Related Work. Subsequences with various types of gap constraints are considered in different
contexts. Not unexpectedly, one of the main areas in which such subsequences were investigated is
combinatorial pattern matching with biological motivations, see [13] and the references therein. In
[58, 59], mining such subsequences is presented as a typical data-mining problem with applications
in classification and clustering algorithms. In [52], a query class for event streams is introduced,
which is based on subsequences with upper and lower length bounds as gap constraints. The
longest common subsequence problem has also been extended to the case where the gaps have
length constraints (see, e. g., [45] and the references therein).
Coming back to [13], a rather well-researched problem that is related to our setting is that of
matching variable length gap patterns. In this setting, a pattern, defined as the string
u1 (p1 , q1 )u2 (p2 , q2 ) . . . um−1 (pm−1 , qm−1 )um
2

<!-- page 3 -->

with ui ∈ Σ+ , (pi , qi ) ∈ N2 and 0 ≤ pi ≤ qi , matches a string w if w = w0 u1 w1 . . . um−1 wm−1 um wm
with pi ≤ |wi | ≤ qi . For this special pattern matching problem many algorithmic results exist
(see [13] and the references therein); moreover, it has also been investigated in more practical papers that provide experimental evaluations of algorithms solving it, see, e. g., [7, 21]. The above can
be seen as special variants of the matching problem for subsequences with gap-length constraints.
While the works above address mostly patterns with length constraints, the area of string
constraint solving (with applications in formal verification, and a strong algorithm engineering
component, see [3]) addresses the problem of aligning two strings containing constants (or contiguous sequences of one or more letters) and variables (or gaps). In general (see the aforementioned
survey [3] and the references therein), the variables/gaps are subject to conjunctions of pairwise
string-equality, length, or regular constraints (see also Appendices A.1 and A.2). Moreover, the
problem of checking whether factors of words are part of a given regular language were addressed
in the context of sliding window algorithms [31, 32, 33, 34, 35] or in the streaming model [10, 26].
On the other hand, we are not aware of any works that are concerned with (non-trivial) gapconstrained variants of the analysis problems (i. e., universality, containment, and equivalence).
Let us now formally define the setting considered in this paper.
Subsequences With Gap Constraints. Since the gaps induced by an embedding are essentially strings (or words), it seems natural to formalise gap constraints for length-k subsequences
by (k − 1)-tuples of sets of strings (i. e., languages) gc = (C1 , . . . , Ck−1 ), where Ci ⊆ Σ∗ for every
i ∈ {1, . . . , k − 1}; we denote |gc| = k − 1. A length-k subsequence u = u1 u2 . . . uk of v = v1 v2 . . . vn
satisfies gc (i. e., it is a gc-subsequence) if u e v for an embedding e that satisfies gc in the sense
that, for every i ∈ [k − 1], ve(i)+1 . . . ve(i+1)−1 ∈ Ci . By SubSeq(gc, v) we denote the set of all
gc-subsequences of v. In this setting, we consider:
• the matching problem Match: decide, for given strings p, w, and gap constraints gc with
|gc| = |p| − 1, whether p is a gc-subsequence of w (i. e., whether u ∈ SubSeq(gc, v));
• the universality problem Uni: decide, for given string w and gap constraints gc with |gc| =
k − 1 , whether SubSeq(gc, w) = Σk ;
• the equivalence problem Equ (respectively, the containment problem Con): decide, for given
strings w, w0 , and gap constraints gc, whether SubSeq(gc, w) = SubSeq(gc, w0 ) (respectively,
SubSeq(gc, w) ⊆ SubSeq(gc, w0 )).
Our formalisation of gap constraints is as general as possible. In order to obtain meaningful results
we focus on regular constraints, where each Ci is a regular language, and on length constraints,
where each Ci has the form {v ∈ Σ∗ | L− (i) ≤ |v| ≤ L+ (i)} with L− (i), L+ (i) ∈ N ∪ {0, +∞}
and is represented as the pair (L− (i), L+ (i)). We also consider conjunctions (Ci , (L− (i), L+ (i))) of
regular and length constraints, i. e., the gap must be from Ci and of length between L− (i) and L+ (i)
(note that simply “pushing” the length constraint into the regular language Ci would increase Ci ’s
representation by a factor L+ (i), which is exponential in L+ (i)’s binary representation). These
constraints cover the existing cases in the literature.
Our Contribution. We provide a comprehensive picture of the computational complexity of
both the matching and the analysis problems, proving tight upper and lower bounds for them,
with a focus on the latter.
With respect to matching, we show that we can check whether u is a gc-subsequence of v in
rectangular time O(|v||gc|), where, if each Ci is the conjunction of a regular constraint and a length
constraint, |gc| is the number of states of the DFAs that represent the regular constraints. In the
absence of regular constraints (so, for length constraints only), such rectangular upper bounds are
already reported in the literature (see [45]). Moreover, the case when length constraints are absent
(so, we have regular constraints only) is rather straightforward. Our algorithm dealing with the
case of conjunctions of regular and length constraints requires, however, a non-trivial extension of
the existing approaches. Nevertheless, our main contribution in this area is that we can also prove a
conditional lower bound that essentially states that these running times of those algorithms cannot
be improved unless the orthogonal vectors hypothesis fails. More precisely, adding length or regular
constraints to subsequences changes the matching problem from a trivial problem to a problem
with provably rectangular complexity. Additionally, this proves also a conditional lower bound for
matching variable length gap patterns (mentioned above), for which many upper bounds, but no
3

<!-- page 4 -->

matching lower bound were known before. It is also worth noting that the lower bound holds for
the case of a constant alphabet and constant length constraints.
With respect to the problems of universality, equivalence, and containment, we show strong
intractability results for both the cases of length constraints and of regular constraints. More
precisely, these problems are NP-complete even for a fixed binary alphabet and for small, constant
length (or regular) constraints (note that the problems are trivial for a unary alphabet). Moreover,
for any fixed constant alphabet, the problems can be solved by brute-force algorithms in exponential
time 2O(k) |gc|` (recall that k is the length of subsequences; ` is the maximum length of the input
strings), and we can show that for alphabets of size at least 3, the exponent can neither be lowered
to any o(k) (unless the exponential time hypothesis fails), nor to k(1 − ) for any  ≥ 1 (unless
the strong exponential time hypothesis fails), and these lower bounds even hold for small constant
length constraints. If we parameterise by both |Σ| and k, then the brute-force algorithm is a trivial
fpt-algorithm. However, we can exclude fpt-running times for the cases where we parameterise by
only |Σ|, or by only k (based on the assumptions P 6= NP and FPT 6= W[1], respectively). Note
that for classical subsequences all these problems can be easily solved in polynomial time, so our
results emphasise the fundamentally different nature of constrained subsequences.
Additionally (and only in Appendix G, due to space constraints), we investigate some natural extensions of both the matching problem, involving gap-lengths equality, and the analysis
problems, which involve counting the number of occurrences of subsequences.

2

Preliminaries

Let N = {1, 2, . . .} and [n] = {1, . . . , n} for n ∈ N. By P(S), we denote the power set of a set S.
For a finite alphabet Σ, Σ+ denotes the set of non-empty words over Σ and Σ∗ = Σ+ ∪ {ε}
(where ε is the empty word). For a word w ∈ Σ∗ , |w| denotes its length (in particular, | ε | = 0); for
every b ∈ Σ, |w|b denotes the number of occurrences of b in w; we set w1 = w and wk = wwk−1 for
every k ≥ 2. For a string w = w1 w2 . . . wn with wi ∈ Σ for every i ∈ [n], and for every i, j ∈ [|w|]
with i ≤ j, we define w[i..j] = wi wi+1 . . . wj ; moreover, we use w[i] as shorthand for w[i..i]. For
any string w ∈ Σ∗ , we define alph(w) = {b ∈ Σ | |w|b ≥ 1}. A factor of a string w ∈ Σ∗ is a string
v ∈ Σ∗ such that w = uvu0 for u, u0 ∈ Σ∗ ; if u = ε, then v is called a prefix of w, and if u0 = ε,
then v is called a suffix of w.
By REG, we denote the class of regular languages (see [44] for more details). For the considered
algorithmic problems we use as computational model the standard unit-cost RAM with logarithmic
word size, with inputs over integer alphabets (see Appendix B.1).
Hypotheses. We now recall some basic computational problems and respective algorithmic
hypotheses. We shall use these hypotheses to obtain our conditional lower bounds.
The problem CNF-Sat gets as input a Boolean formula F in conjunctive normal form as a set
of clauses F = {c1 , c2 , . . . , cm } over a set of variables V = {v1 , v2 , . . . , vn }, i. e., for every i ∈ [m],
we have ci ⊆ {v1 , ¬v1 , . . . , vn , ¬vn }. The question is whether F is satisfiable. By k-CNF-Sat, we
denote the variant where |ci | ≤ k for every i ∈ [m].
The Orthogonal Vectors problem (OV for short) is defined as follows: Given sets A, B each
containing n Boolean-vectors of dimension d, check whether there are vectors ~a ∈ A and ~b ∈ B
that are orthogonal, i. e., ~a[i] · ~b[i] = 0 for every i ∈ [d].
We shall use the following algorithmic hypotheses based on CNF-Sat and OV that are common
for obtaining conditional lower bounds in fine-grained complexity (see the literature mentioned
below for further details). In the following, poly is any fixed polynomial function.
• Exponential Time Hypothesis (ETH) [47, 61]: 3-CNF-Sat cannot be solved in time 2o(n) poly(n+
m).
• Strong Exponential Time Hypothesis (SETH) [46, 77]: For every  > 0 there exists a k such
that k-CNF-Sat cannot be decided in O(2n(1−) poly(n)).
• Orthogonal Vectors Hypothesis (OVH) [16, 17, 77]: For every  > 0 there is no algorithm
solving OV in time O(n2− poly(d)).

4

<!-- page 5 -->

Subsequences With Gap Constraints. We now define subsequences with gap constraints
(see also the introduction). In the following, let Σ be a finite alphabet. Recall that for a string w,
an embedding is a function e : [k] → [|w|] such that i < j implies e(i) < e(j) for all i, j ∈ [k], and
it induces the subsequence subseqe (w) = w[e(1)]w[e(2)] . . . w[e(k)] of w. For every j ∈ [k − 1], the
j th gap of w induced by e is the string gape (w, j) = w[e(j) + 1..e(j + 1) − 1]. We say that e is the
embedding of subseqe (w) in w.
An `-tuple of gap constraints is a tuple gc = (C1 , C2 , . . . , C` ) with Ci ⊆ Σ∗ for every i ∈ [`].
For convenience, we set gc[i] = Ci for every i ∈ [`]. We say that an embedding e satisfies a
(k − 1)-tuple of gap constraints gc with respect to a string w if it has the form e : [k] → [|w|],
and, for every i ∈ [k − 1], gape (w, i) ∈ Ci . Moreover, for a (k − 1)-tuple gc of gap constraints,
the set SubSeq(gc, w) contains all subsequences of w induced by embeddings that satisfy gc, i. e.,
SubSeq(gc, w) = {subseqe (w) | e is an embedding that satisfies gc w. r. t. w}. The elements of
SubSeq(gc, w) are also called the gc-subsequences of w. Note that tuples of gap constraints do not
have constraints for the prefix w[1..e(1)] or suffix w[e(k)..|w|]. However, our formalism can model
this case too (for details, see Appendix C.2). For a (|u| − 1)-tuple gc of gap constraints, we write
u gc v to denote that u e v for some embedding e : [|u|] → [|v|] that satisfies gc with respect to
v, i. e., u gc v means that u is a gc-subsequence of v. We note that for tuples of gap constraints
gc = (C1 , C2 , . . . , Ck−1 ) with Ci = Σ∗ for every i ∈ [k − 1], the set SubSeq(gc, w) is just the set of
all length-k subsequences of w.
Special Types of Gap Constraints. We now define the types of gap constraints that are
relevant for our work. We say that the gap constraints gc = (C1 , . . . , Ck−1 ) are
• regular constraints if Ci ∈ REG for every i ∈ [k − 1]. For every i ∈ [k − 1], we represent the
regular constraint Ci by a deterministic finite automaton (for short, DFA) Ai accepting it.
See Appendix A.1 for a discussion on the choice of DFAs to represent regular constraints.
• length constraints if, for every i ∈ [k − 1], there are L− (i), L+ (i) ∈ N ∪ {0, +∞} with
L− (i) ≤ L+ (i), such that Ci = {v ∈ Σ∗ | L− (i) ≤ |v| ≤ L+ (i)}. We represent length
constraints succinctly by pairs of numbers (L− (i), L+ (i)), i ∈ [k − 1], in binary encoding.
• reg-len constraints if, for every i ∈ [k − 1], Ci is the conjunction of a regular constraint Ci0
and a length constraint (L− (i), L+ (i)), i. e., Ci = Ci0 ∩ {v ∈ Σ∗ | L− (i) ≤ |v| ≤ L+ (i)}. We
represent such constraints by ((L− (i), L+ (i)), A0i ), where A0i is a DFA accepting Ci0 .
A gap constraint Ci is a zero-gap if and only if Ci = {ε}. Let nz(gc) be the number of non-zerogaps of gc (that is, the number of positions i such that Ci 6= {ε}). For a tuple of regular or reg-len
gap constraints gc, let size(gc) be the size of the overall representation of the respective constraints
(total size of the automata defining the constraints) and let states(gc) be the total number of states
of the DFAs Ai , for i ∈ [k − 1], corresponding to the non-zero gaps of gc.
Clearly, length constraints are the simplest type of gap constraints considered above. In particular, length constraints, and therefore reg-len constraints, can also be seen as a particular case of
regular constraints. However, transforming length or reg-len constraints into a single automaton
may cause an exponential size increase.
Problems for Subsequences With Gap Constraints. In this paper, we investigate the
matching problem Match and the analysis problems Uni, Con, and Equ (see definitions in the
introduction). For simplicity, the pairs (p, gc), which play the role of the patterns in Match, will
be called gap-constrained sequences, or simply gapped sequences for short. By MatchΣ , we denote
the problem variant where all instances are over the fixed alphabet Σ; for some class C of gap
constraints, we use “Match with C-constraints” to refer to the variant where the constraints are
from C. We use analogous notations for the analysis problems.
If gc = (Σ∗ , Σ∗ , . . . , Σ∗ ), then Match boils down to the simple task of checking whether a given
string is a subsequence of another string. The equivalence problem for such trivial gap constraints,
on the other hand, boils down to the well-known problem of deciding the Simon congruence for two
strings (see the discussion in the introduction). Our setting naturally models many other classical
problems; some are discussed in Appendix C.3. Finally, even though our framework allows arbitrary
gap constraints, we will stick to the specific natural and relevant types of constraints defined above
(i. e., length, regular, reg-len constraints).

5

<!-- page 6 -->

3

Matching Gapped Subsequences

This section contains two main results. Firstly, we show that Match with reg-len constraints
can be solved in O(|w| states(gc) + size(gc)) time, which implies also rectangular upper bounds for
Match with either length or regular constraints. Secondly, we show that, assuming OVH holds,
there are no algorithms solving any of these problems polynomially faster.
Note that, when dealing with length constraints, a constraint (L− (i), L+ (i)) is equivalent to
the regular language Ci = {x ∈ Σ∗ | L− (i) ≤ |x| ≤ L+ (i)}, which is accepted by a DFA with
Θ(L+ (i)) states. So, we could also interpret a tuple gc of reg-len constraints as a tuple of regular
constraints only, by considering in each component of gc the intersection of the regular constraint
with the regular language defined by the length constraints. However, this would lead to a growth
in the number of states needed to model gc, and, as we will see in the following, to a less efficient
algorithm for Match. In this setting, we state our first main result. The full proof is given in
Appendix D. To emphasise the merits of our approach, we overview in Appendix D several simpler
approaches and their complexity (and shortcomings).
Theorem 3.1. Match with reg-len constraints can be solved in O(|w| states(gc) + size(gc)) time.
Proof Sketch. Assume |w| = n, |p| = m, and gc = ((L− (1), L+ (1)), A1 ), . . . , (L− (m − 1), L+ (m −
1)), Am−1 )), where Ai = (Qi , q0,i , Fi , δi ) are DFAs defining the regular constraints and (L− (i), L+ (i))
are pairs of numbers defining the length constraints. Let i1 , . . . , ik−1 ∈ [m − 1] be such that
Ci 6= {ε} (i. e., Ci is a non-zero constraint of gc), for all i ∈ {i1 , . . . , ik−1 }, and Ci = {ε}, for all
Pk−1
i∈
/ {i1 , . . . , ik−1 }. Clearly, states(gc) = j=1 |Qij | ≥ nz(gc). With i0 = 0 and ik = m, we compute the words pj = p[(ij−1 + 1)..ij ], for j ∈ [k], and we construct in linear time longest common
extension data structures (see [25] and the references therein) for the word x = wp, allowing us to
check in constant time whether w[i + 1..i + |pj |] = pj , for i ≤ n.
After this preprocessing part, the main part of our algorithm consists in a dynamic programming
approach. We compute a two-dimensional n × k array D[·][·], where D[i][`] = 1 if and only if
p[1..|p1 · · · p` |] can be embedded in w[1..i] and this embedding satisfies the first ` − 1 non-zero
constraints of gc and maps p` to the suffix of length |p` | of w[1..i]. Otherwise, D[i][`] = 0. To start
the computation of D, we initialize all the elements of D with 0. We then set D[i][1] = 1 if and
only if w[1..i] ends with p1 , i. e., w[i − |p1 | + 1..i] = p1 .
Further, assume that, for some t ∈ [k − 1], we have computed D[·][`], for all ` ≤ t, and we want
to compute D[·][t + 1]. This is the most involved and deep part of our algorithm and its main
component is computing an array ft+1 [·], with n elements, such that ft+1 [i] = 1 iff there exists a
position j for which D[j][t] = 1, w[j + 1..i] ∈ L(At ), and L− (t) ≤ |w[j + 1..i]| ≤ L+ (t). A full
description of this part of the algorithm is given in Appendix D; here we just sketch it.
We first collect in a list Lt+1 = j1 < . . . < jr (increasingly sorted) all the positions j of w
with D[j][t] = 1. Then, we compute a graph Gt+1 that has nodes of the form (i, q), with i ∈ [n]
and q ∈ Qt , and consists of the union, over j ∈ Lt+1 , of the (not necessarily disjoint) paths
j
j
[(j, q0,t ), (j + 1, q1j ), . . . , (n, qn−j
)], where δt (q0,t , w[j + 1]) = q1j and δt (qrj , w[j + r + 1]) = qr+1
, for
all r ∈ [n − j − 1]. Intuitively, such a path records the trace of the computation of At on the input
w[j + 1..n]. For efficiency, these paths (and, therefore, the graph Gt+1 ) can be simultaneously
constructed to avoid redundant computations. An important observation is that if two such paths
intersect, then they are identical after their first common node; this is, indeed, true because At is
a deterministic finite automaton. Consequently, Gt+1 is a collection of disjoint trees T1 , T2 , . . . , Tz .
As there are no edges between any pair of nodes (n, q) and (n, q 0 ), with q, q 0 ∈ Qt , each such tree
Ti can be seen as a rooted tree, whose root is its single node of the form (n, q) and whose leaves
are some of the nodes (j, q0,t ), with j ∈ Lt+1 .
Then, based on a series of efficient data structures and further insights, we efficiently mark,
for each tree Ti and for each leaf (j, q0,t ) of Ti , all the ancestors (d, q) of (j, q0,t ) such that
L− (t) ≤ |w[j + 1..d]| = d − j ≤ L+ (t). Once we have completed the marking for tree Ti , a
node (j, q) is marked if and only if there exists a path P of length `, with L− (t) ≤ ` ≤ L+ (t),
which connects a leaf (j 0 , q0,t ) of Ti to (j, q). Or, in other words, δt (q0,t , w[j 0 +
= q. The trees
P1..j])
p
Ti , with i ∈ [z], are computed in O(n|Qt |) time, while the marking takes O( i=1 |Ti |) time.
Finally, we simply set, for i from 1 to n, ft+1 [i] = 1 if and only if there exists a state q ∈ Ft
such that the node (i, q) is marked. This means that ft+1 [i] = 1 if and only if there exists a word
6

<!-- page 7 -->

w[j + 1..i] of length `, with L− (t) ≤ ` ≤ L+ (t), such that j ∈ Lt+1 and δt (q0,t , w[j + 1..i]) is a final
state (i. e., w[j + 1..i] ∈ L(At )).
Coming now back to the computation of the elements of D, we set D[i][t + 1] = 1 if and only
if w[i − |pt+1 | + 1..i] = pt+1 and ft+1 [i − |pt+1 |] = 1. Clearly, D[·][t + 1] is correctly computed.
After D is computed, we decide that p gc w if and only if there exists j with D[j][k] = 1. The
whole process can be implemented in O(|w| states(gc) + size(gc)) time.
The next results are now immediate. Note that for these particular cases (but, to the best of
our knowledge, not for their conjunction, covered in Theorem 3.1) simpler algorithms exist.
Corollary 3.2. (1). Match with length constraints can be solved in O(|w| nz(gc)) time.
(2). Match with regular constraints can be solved in O(|w| states(gc) + size(gc)) time.
It is worth noting that the matching problem can be solved in O(|w|) time when gc only defines
constraints that are {ε} or Σ∗ , which covers, e. g., the cases of subsequence matching or string
matching. In particular, the greedy strategy used for matching regular patterns with variables
(see, e. g., [27]) can be easily adapted to solve Match with length constraints in linear time, when
the upper bounds on each gap are trivial (i. e., they are all greater or equal to the length of the
input word). So, as far as length constraints are concerned, it seems that non-trivial upper bounds
lead to an increase in the difficulty of the Match problem; a particularly efficient approach for
subsequences with general length constraints is given in [13], but, in the worst case, it still has
rectangular complexity. However, even when non-trivial length upper bounds are used, there are
still some simpler particular cases. For instance, when working with strings with don’t cares (or
partial words), where each gap has a fixed length (i. e., the lower and upper bounds are the same),
Match can be solved in time O(|w| log |p|) [22].
A gapped sequence (p, gc) with reg-len constraints can be represented as a classical regular
expression r(p,gc) , so Match can be solved by a textbook algorithm in O(|w||r(p,gc) |) [74], which
is optimal w. r. t. polynomial speed-ups, conditional on OVH [6]. However, including the string p
and the length constraints in the regular expression might, once more, lead to a slower algorithm
compared to our direct approach, as |rp,gc | may be much larger than states(gc).
To summarise, at an intuitive level, we could say that as long as we have non-trivial length
or regular constraints, Match seems to become more difficult than its counterpart for classical
subsequences. This intuitive remark is confirmed by our second main result.
Theorem 3.3. Match with length constraints cannot be solved in O(|w|h nz(gc)g ) time with h +
g = 2 −  for some  > 0, unless OVH fails. This holds even if |Σ| = 4 and all length constraints
are (0, `) with ` ≤ 6.
Proof Sketch. Let A = {~a1 , . . . , ~an } and B = {~b1 , . . . , ~bn }, with A, B ⊂ {0, 1}d be an OV-instance.
We transform A into a string w ∈ Σ∗ = {0, 1, #, @}∗ and B into a string p ∈ Σ∗ and a (|p| − 1)tuple gc of length constraints. For convenience, we represent the gapped sequence (p, gc) with
gc[1]

gc[2]

p = p[1] · · · p[m] by writing the length constraints in between the symbols, i. e., p[1] ↔ p[2] ↔
···

gc[m−1]

↔

gc[i]

p[m], and we omit ↔ if gc[i] = (0, 0). For example, if p = abab and gc[1] = (0, 0),
(1,5)

≤6

gc[2] = (1, 5), and gc[3] = (0, 6), we use the notation ab ↔ a ↔ b.
Let ~ai = (a1i , . . . , adi ) and ~bi = (b1i , . . . , bdi ), for all i ∈ [n]. We shall represent the vectors from A
and B by different encodings Ca (·) and Cb (·), respectively. The 0 and 1 entries in the A-vectors are
encoded by Ca (0) = 010 and Ca (1) = 100, and the 0 and 1 entries in the B-vectors are encoded by
Cb (0) = 10 and Cb (1) = 01. We note that for every x, y ∈ {0, 1}, Cb (x) is a factor of Ca (y) if and
only if x · y = 0. This means that the orthogonality of ~ai and ~bi0 is characterised by the situation
that, for every j ∈ [d], Cb (bji0 ) is a factor of Ca (aji ).
We represent each bit aji of ~ai ∈ A as the string # Ca (0)## Ca (aji )## Ca (0)#, and the whole
Qd
vector ~ai as the concatenation Ca (~ai ) = j=1 ([1 # Ca (0)#]1 [2 # Ca (aji )#]2 [3 # Ca (0)#]3 ) , where
the brackets [1 . . .]1 , [2 . . .]2 , [3 . . .]3 are not actual symbols of the gadget, but serve the only purpose
to illustrate that Ca (~ai ) has three individual tracks, where track 1 and 3 correspond to d occurrences of # Ca (0)# (representing the all-0 vector), while track 2 represents the actual vector ~ai .
These three tracks play a central role in the correctness of the reduction.

7

<!-- page 8 -->

For i ∈ [n], every vector ~bi ∈ B is also represented by listing all bit encodings Cb (bji ), but in a
slightly different
way and, most importantly, as a gapped sequence (in the notation defined above):
Q
≤1
≤3
≤3
≤1
≤1
≤1
d−1
~
(Cb (bi ), gci ) =
(# ↔ Cb (bj ) ↔ ## ↔ ## ↔ #) # ↔ Cb (bd ) ↔ #.
i

i

j=1

462

It can be shown (see Appendix D.1) that if Cb (~bi ) e Ca (~a` ) and e satisfies gci , then the embedding e maps each Cb (bji ) to the Ca (0) of Ca (~a` )’s first track, or each Cb (bji ) to the Ca (aj` ) of Ca (~a` )’s
second track, or each Cb (bji ) to the Ca (0) of Ca (~a` )’s third track. More precisely, due to how we
use the symbols #, the factor Cb (b1i ) must be mapped to [1 # Ca (0)#]1 or to [2 # Ca (a1i )#]2 or to
[3 # Ca (0)#]3 . Since we have 4 occurrences of # between each Cb (bji ) and Cb (bj+1
), and between
i
two consecutive parts of the same track in Ca (~ai ), all the following factors Cb (b2i ), Cb (b3i ), . . . must
be mapped to the same track Cb (b1i ) is mapped to. This is illustrated in Figure 1. Based on these
considerations, it is clear that Cb (~bi ) e Ca (~a` ) with e mapping Cb (~bi ) to Ca (~a` )’s second track is
possible if and only if ~a` and ~bi are orthogonal.
Cb (˛bi ) =
Cb (˛bi ) =

#

¡ Cb (b1i ) ¡

Æ1

Æ1

#

#

Cb (˛bi ) =
Ca (˛a¸ ) =

#

Ca (0)

#

Æ3

#

#

¡

#

Æ1
¡ Cb (b1i ) ¡

#

Ca (aji )

#

Æ1

#

Æ3

#

Æ3

#
#

#

¡
¡

¡ Cb (bi1 ) ¡

Æ1

Ca (0)

Æ1

#

¡ Cb (b2i ) ¡

#

Æ3

¡

#

#

Æ1

Æ1

#

#

#

#

Æ3

#

#

#

#

Ca (0)

#

#

¡

Æ3

#

#

...

¡ Cb (b2i ) ¡

#

...

Æ3

#

...

Ca (aji )

#

...

Æ1

¡
¡

Æ1

Figure 1: Possible embeddings of Cb (~bi ) in Ca (~a` ), selecting its first, second, or third track.
The remaining challenge is to combine the gadgets Ca (~ai ) into a string w, and the gadgets
(Cb (~bi ), gci ) into a gapped sequence (p, gc), such that p e w for an embedding e satisfying gc if
and only if e is such that every (Cb (~bi ), gci ) is mapped to some Ca (~a` ), and there is necessarily
at least one pair i, ` ∈ [n] such that (Cb (~bi ), gci ) is embedded into Ca (~a` )’s second track. We next
define w and (p, gc),
why they
from above:

Q satisfy the property

Q and then discuss
n−1
n−1
ai ) @ Ca (~an )
ai ) @ ,
w=
i=1 @ Ca (~
i=1 @ Ca (~


≤5 Qn−1
≤1
≤3
≤1
≤3
≤5
~bj ) ↔ # ↔ # ↔ # ↔ # ≤6
(p, gc) = @ ↔
C
(
↔
Cb (~bn ) ↔ @.
b
j=1
If p e w for an embedding e satisfying gc, then the first @-symbol of p is mapped to the @symbolof w occurringbefore an occurrence of Ca (~a`1 ) for some `1 , and this occurrence is in the
Qn−1
prefix
ai ) of w. By reasoning about the occurrences of symbols # and the length
i=1 @ Ca (~
constraints (see Appendix D.1), we can show that Cb (~b1 ) must be embedded in Ca (~a`1 ) in the way
discussed above (i. e., gc1 is satisfied and Cb (~b1 ) is entirely mapped to some track q ∈ {1, 2, 3} of
≤1

≤3

≤1

≤3

≤6

Ca (~a`1 )). For simplicity, assume that `1 ≤ n − 1. The factor ↔ # ↔ # ↔ # ↔ # ↔ between
(Cb (~b1 ), gc1 ) and the next part (Cb (~b2 ), gc2 ) will enforce that Cb (~b2 ) is embedded in Ca (~a`1 +1 ), and,
moreover, it will be mapped to Ca (~a`1 +1 )’s track q or q + 1 (as there can be at most 18 symbols
between Cb (~b1 ) and Cb (~b2 ), track 3 cannot be reached in the case q = 1).
By repeating this argument, we can show that if (Cb (~bj ), gcj ) is embedded in track s of Ca (~a`j )
(with `j ≤ n−1), then (Cb (~bj+1 ), gcj+1 ) is embedded in track s or s+1 of Ca (~a`j +1 ) in case that s ∈
{1, 2}, and it is necessarily embedded in track s of Ca (~a`j +1 ) in case that s = 3. If `j = n, then analQn−1
ogously (Cb (~bj+1 ), gcj+1 ) is mapped to Ca (~a1 ) of w’s suffix ( i=1 @ Ca (~ai ))@. Consequently, each
Cb (~bj ) is mapped to a track of Ca (~a`j ), and the tracks to which these Cb (~bj ) are mapped may start
with track 1 or 2, and then can only increase until we possibly map some Cb (~bj ) to track 3. However,
after having mapped the last occurrence of # in Cb (~bn ) to an occurrence of # in Ca (~a`n ), we can afford a gap of length at most 5 before mapping the last symbol @ of (p, gc) to an occurence of @ in w.
By the structure of (p, gc) and w, this is only possible if Cb (~bn ) is mapped to track 2 or 3 of Ca (~a`n ).
We conclude that if p gc w, then, for some j, `j ∈ [n], (Cb (~bj ), gcj ) is mapped to track 2 of
Ca (~a`j ); thus, ~a`j and ~bj are orthogonal. On the other hand, the explanations from above show
that if ~a`j and ~bj are orthogonal vectors, then p can be embedded into w by an embedding that
satisfies gc, i. e., an embedding that maps (Cb (~bj ), gcj ) to track 2 of Ca (~a`j ), all (Cb (~bj 0 ), gcj 0 ) with
1 ≤ j 0 < j to the first tracks of some Ca (~a`j0 ), and all (Cb (~bj 0 ), gcj 0 ) with j < j 0 ≤ n to the third
tracks of some Ca (~a`j0 ).
8

<!-- page 9 -->

In this reduction, we have |Σ| = 4, all constraints are (0, `) with ` ≤ 6, and |w|, |p| ∈ Θ(nd). If
Match can be solved in O(|w|g |p|h ) with g + h = 2 −  for some  > 0, then OV can be solved in
O(nd + (nd)2− ). Since nz(gc) ∈ Θ(nd), solving Match in O(|w|g nz(gc)h ) with g + h = 2 −  for
some  < 0 also contradicts OVH.
We emphasise that, according to our proof, these lower bounds hold for Match with length
constraints even if we only have constant upper bounds on the length of the gaps.
Corollary 3.4. Match with regular constraints cannot be solved in O(|w|h states(gcp )g ) time with
h+g = 2− for some  > 0, unless OVH fails. This holds even if |Σ| = 4 and all regular constraints
are expressed by constant size DFAs.
From Theorem 3.3 and Corollary 3.4 we also get that Match with length, regular, or reglen constraints cannot be solved in O(|w|h |p|g ) time, with h + g = 2 − , nor in O(|w|2− ) time.
Moreover (see Appendix D.3) we can show similar lower bounds for |Σ| = 2 as well.
Compared to the OVH-bound for regular expression matching of [6], we provide a lower bound
for a much more restricted problem (i. e., matching gapped sequences with length constraints, a
subclass of regular expressions that still seems to have a significant practical relevance); thus, a
stronger lower bound (this is also why our OV-reduction has a significantly different structure and
is technically more involved than that of [6]). In particular, our lower bound applies (unlike those
from [6]) to the case of matching variable length gap patterns, and settles the complexity of that
problem. We wrap up this section by noting that Theorem 3.3 and Corollary 3.4 show that (if OVH
holds) the algorithm of Theorem 3.1, also when used for regular constraints or length constraints
only, is optimal in the sense that there are no algorithms which can solve Match in the respective
settings polynomially faster.

4

Analysis Problems for Gapped Subsequences

Let us recall that the universality, containment and equivalence problem (denoted by Uni, Con
and Equ for short) consist in deciding SubSeq(gc, w) = Σk , SubSeq(gc, w) ⊆ SubSeq(gc, w0 ), and
SubSeq(gc, w) = SubSeq(gc, w0 ), respectively, for a given (k − 1)-tuple gc of gap constraints and
strings w, w0 ∈ Σ∗ . As mentioned before, these problems can be solved in polynomial time for
classical subsequences (see Appendix E for further details). We show next that these problems
become much harder for non-trivial length or regular constraints.
From Cor. 3.2 and Thm. 3.1, we can directly conclude the following brute-force upper bounds.
Theorem 4.1. (1) The problems Uni, Con and Equ with length (or reg-len) constraints can be
solved in time O(|Σ|k nz(gc)`) (respectively, O(|Σ|k states(gc)`)), where ` = max{|w|, |w0 |}.
(2) The problems UniΣ , ConΣ and EquΣ with length (or reg-len) constraints can be solved in time
2O(k) nz(gc)` (respectively, 2O(k) states(gc)`), where ` = max{|w|, |w0 |}.
We shall next complement these brute-force upper bounds by suitable lower bounds, which
demonstrate that significantly faster algorithms are unlikely to exist. For convenience, we state
our complexity results for the complement problems, i. e., non-universality problem (NUni), noncontainment problem (NCon), and non-equivalence problem (NEqu). Moreover, we state the
lower bounds for the case of length constraints only. By simply interpreting the length constraints
as regular constraints, all the lower bounds also apply to the case of regular constraints (this does
not cause an exponential size increase of the instances, see Appendix C.1).
Our first result establishes the general NP-completeness (even for small constant alphabets
and length constraints), and that the exponent O(k) of Theorem 4.1(2) cannot be significantly
improved, unless ETH or SETH fail. We will discuss some proof ideas later on.
Theorem 4.2. For every fixed alphabet Σ with |Σ| ≥ 3, NUniΣ , NConΣ and NEquΣ with length
constraints are NP-complete, even if all length constraints are (1, 5). Moreover,
• they cannot be solved in subexponential time 2o(k) poly(|w|, k)) (unless ETH fails),
• they cannot be solved in time O(2k(1−) poly(|w|, k)) (unless SETH fails).
9

<!-- page 10 -->



{a}
{c, d}

 {a}
{e}


{b}
{a, c, d}
{b}
{a} 

{b, c, d, e}
{d} 
{b}
{c, e}



{0}
 {1}
{0, 1}

{1}
{0} {0, 1} {0, 1} {0, 1}
{1} {0, 1} {0, 1} {0, 1} {0}
{0, 1} {0}
{0}
{0} {0, 1}




Figure 2: Left side: example instance of MetaNUni for Γ = {a, b, c, d, e}, q = 4, and k = 3.
Note that, e. g., W3,2 = {b, c, d, e} and W4,1 = {e}; moreover, L(W1 ) = {a} · {b} · {a, c, d} =
{a b a, a b c, a b d}. Since ∪i∈[4] L(Wi ) 6= Γ3 , this is a negative instance. Right side: the CNF-Satinstance c1 = {v1 , ¬v2 , v3 }, c2 = {¬v1 , ¬v2 , v5 }, c3 = {v3 , v4 , v5 } over the variables {v1 , v2 , . . . , v6 }
as an instance of MetaNUni . Note that 100010 ∈
/ ∪i∈[3] L(Wi ); thus, 100010 is a satisfying
assignment.
This directly leads to the question whether these problems are tractable if |Σ| ≤ 2. This is
obviously
true for unary alphabet Σ = {a} (note that in this case, SubSeq(gc, w) = Σk = {ak } if
P
−
( i∈[k] L (i)) + k ≤ |w|, and SubSeq(gc, w) = ∅ otherwise), but NP-complete for |Σ| = 2:
Theorem 4.3. For every fixed alphabet Σ with |Σ| = 2, NUniΣ , NConΣ and NEquΣ with length
constraints are NP-complete even if each length constraint is (0, 0) or (3, 9).
Let us now consider the case where Σ is not treated as a constant. Theorem 4.1 means that
NUni, NCon and NEqu with length constraints are trivially fixed parameter tractable if parameterised by both |Σ| and k. Moreover, since ` = max{|w|, |w0 |} bounds both |Σ| and k, we also have
fixed parameter tractability with respect to ` for trivial reasons. Are the problems fixed-parameter
tractable with respect to the single parameter |Σ| or the single parameter k? With respect to |Σ|,
this is answered in the negative by Theorem 4.3 (unless P = NP). With respect to parameter k,
the following result gives a negative answer as well.
Theorem 4.4. Problems NUni, NCon and NEqu with length constraints cannot be solved in
running time O(f (k) poly(|w|, k)) for any computable function f (unless FPT = W[1]).
This result only holds for unbounded alphabets and length constraints. Indeed, for constant
Σ the brute-force algorithm is an fpt-algorithm with respect to k. Moreover, if the upper length
constraints are bounded by some constant `, then we only have to enumerate at most `k−1 candidate
tuples of gap sizes and check whether one of them induces an embedding satisfying gc with respect
to w, which again would yield an fpt-algorithm with respect to k.
Proof Ideas for the Lower Bounds. We present some proof ideas and sketches for the lower
bounds mentioned above. For convenience, we only consider the non-universality problem here.
Full proof details can be found in Appendix F.
Theorem 4.2 can be proven by a reduction from CNF-Sat. In order to get the ETH and SETH
lower bounds, this reduction must yield instances with a (k − 1)-tuple of gap constraints, where k
is exactly the number of Boolean variables. Theorem 4.4 can be shown by a very similar reduction
that starts from the standard parameterisation of the independent set problem. Both reductions
can be conveniently defined by using a meta non-universality problem (MetaNUni for short) as
an intermediate step, which we define next.
Let Γ = {b1 , b2 , . . . , bm } be some alphabet, and let q, k ∈ N. An instance of the problem is
a (q × k)-matrix with the entries Wi,j , which are subsets of Γ. For every i ∈ [q], we associate
with row i of the matrix the language L(Wi ) = Wi,1 · Wi,2 · · · Wi,k , i. e., we simply represent the
elements of Wi,1 × Wi,2 × . . . × Wi,k as length-k strings over Γ in the natural way. The question is
then to decide whether ∪i∈[q] L(Wi ) 6= Γk (see Figure 2 for an example).
We next discuss, how we can reduce CNF-Sat to MetaNUni. Let F = {c1 , c2 , . . . , cq } be
a Boolean formula in CNF on variables {v1 , . . . , vk } (i. e., ci ⊆ {v1 , ¬v1 , . . . , vk , ¬vk }). We define
alphabet Γ = {0, 1} and the (q × k)-matrix with the entries Wi,j as follows. For every i ∈ [q]
and j ∈ [k], we define Wi,j = {0}, if vj ∈ ci , Wi,j = {1}, if ¬vj ∈ ci , and Wi,j = {0, 1}, if
{vj , ¬vj } ∩ ci = ∅. It can be verified with moderate effort, that for every i ∈ [q], L(Wi ) contains
exactly the Boolean assignments that do not satisfy clause ci . Hence, ∪i∈[q] L(Wi ) 6= {0, 1}k if and
only if F is satisfiable (see Figure 2 for an example).
In a rather similar way, we can also phrase the independent set problem in terms of MetaNUni.
For the independent set problem, we get an undirected graph G = (V, E) with |V | = n and
10

<!-- page 11 -->

E = {e1 , e2 , . . . , em }, and a k ∈ [|V |], and the question is whether G has a k-independent set, i. e.,
a set A ⊆ V with |A| = k and {u, u0 } ∈
/ E for every u, u0 ∈ A with u 6= u0 . This can be expressed in
terms of MetaNUni as follows. We interpret the set V of vertices as the alphabet Γ. We fix some
bijection ν : {(i, r, s) ∈ [m] × [k] × [k] | r 6= s} → [mk(k − 1)]. For every i ∈ [m] with ei = (u, v),
and every r, s, j ∈ [k] with r 6= s, we define Wν(i,r,s),j = {u}, if j = r, Wν(i,r,s),j = {v}, if j = s,
and Wν(i,r,s),j = V , else. For example, if e9 = (v3 , v7 ) and k = 4, then row ν(9, 2, 4) of the matrix
would be V {v3 } V {v7 }.
It is a bit more difficult to see why this reduction works. The idea is that we represent sets of
vertices of cardinality at most k by length-k strings over V (note that sets of cardinality strictly less
than k can be represented by strings with repeated symbols). For every edge (u, v) and for all pairs
of positions r, s ∈ [k], the language L(Wν(i,r,s) ) = Wν(i,r,s),1 Wν(i,r,s),2 . . . Wν(i,r,s),k represented by
row ν(i, r, s) of the matrix contains exactly the strings w ∈ Γk with (w[r], w[s]) = (u, v), i. e.,
strings that represent non-independent sets with edge (u, v). For the example e9 = (v3 , v7 ) and
k = 4, we have L(Wν(9,2,4) ) = {v1 v3 v1 v7 , v2 v3 v1 v7 , . . . , vn v3 v1 v7 , . . . , v1 v3 v2 v7 , v2 v3 v2 v7 , . . .}.
This whole idea works only because, in our setting, we assume that every vertex has a loop
since then strings w of V k contain an edge (w[r], w[s]) ∈ E for some r, s ∈ [k] if and only if the
corresponding set of vertices is not independent or of cardinality strictly less than k (the latter is
represented by a loop, i. e.,Sw[r] = w[s]). In summary, G has a k-independent set if and only if not
all length-k strings are in i∈[m],r,s∈[k],r6=s L(Wν(i,r,s) ).
The main technical challenge is to show a reduction from MetaNUni to NUni with length
constraints. We next give a sketch of this reduction. Let Γ = {b1 , b2 , . . . , bm }, q, k ∈ N, and,
for every i ∈ [q], j ∈ [k], let Wi,j ⊆ Γ. We transform this MetaNUni instance into an instance of NUni with length constraints as follows. We first define the alphabet Σ = Γ ∪ {#}
(with # ∈
/ Γ). Then we define a (k − 1)-tuple gc = (C1 , C2 , . . . , Ck−1 ) of gap constraints with
Ci = (L− (i), L+ (i)) = (m − 1, 3m − 1) for every i ∈ [k − 1] (recall that m is Γ’s cardinality). To conclude the reduction, we have to construct a string K(W1 , . . . , Wq ) over Σ, such that
SubSeq(gc, K(W1 , . . . , Wq )) = Σk if and only if ∪i∈[q] L(Wi ) = Γk . We do this in several steps.
For every i ∈ [q] and j ∈ [k], let wi,j ∈ Γ∗ be some string representation of Wi,j , i. e., alph(wi,j ) =
Wi,j and |wi,j | = |Wi,j | ≤ m. For every i ∈ [q], we define the string
S(Wi ) = wi,1 (#)m−1 wi,2 (#)m−1 . . . (#)m−1 wi,k .
We can show that those gc-subsequences of S(Wi ) that do not contain occurrences of symbol
# must be mapped to S(Wi ) in such a way that each j ∈ [k] is mapped to wi,j . More precisely,
for every i ∈ [q], we have that (SubSeq(gc, S(Wi )) ∩ Γ∗ ) = L(Wi ).
(†)
Next, we define a string T whose purpose it is to contain all gc-subsequences that contain at
least one occurrence of #. For every i ∈ [k], let Ti = Ti,1 Ti,2 . . . Ti,k , where, for every j ∈ [k] \ {i},
Ti,j = b1 b2 . . . bm #m , and Ti,i = #m . We define T by T = T1 (#3m )T2 (#3m ) . . . (#3m )Tk . The
idea here is that any gc-subsequence of T must be mapped entirely into some Ti , which, due to
the length constraints, forces position i to be mapped to Ti,i = #m , i. e., to an occurrence of #.
More precisely, we have SubSeq(gc, T ) = {w ∈ Σk | |w|# ≥ 1}.
()
Finally, we set K(W1 , . . . , Wq ) = T (#3m )S(W1 )(#3m )S(W2 )(#3m ) . . . (#3m )S(Wq ). By using (†) and () from above, we can now prove SubSeq(gc, K(W1 , . . . , Wq )) = Σk if and only if
∪i∈[q] L(Wi ) = Γk , which concludes the proof of correctness.
For proving Theorem 4.3, using MetaNUni as an intermediate step seems not possible, since
it introduces another symbol # to the alphabet. However, we can devise a similar reduction.
The main difference is that we represent each Boolean variable by two consecutive symbols of the
subsequence, i. e., we need a (2k − 1) tuple of length constraints (therefore, the reduction does not
yield a SETH bound as mentioned in Theorem 4.2). Since we cannot conveniently use a separator
# that is not used for expressing Boolean assignments, the constructed string is more complicated
in this reduction (see Appendix F for full details).

References
[1] Amir Abboud, Arturs Backurs, and Virginia Vassilevska Williams. Tight hardness results for
LCS and other sequence similarity measures. In IEEE 56th Annual Symposium on Foundations

11

<!-- page 12 -->

of Computer Science, FOCS 2015, Berkeley, CA, USA, 17-20 October, 2015, pages 59–78,
2015.
[2] Amir Abboud, Virginia Vassilevska Williams, and Oren Weimann. Consequences of faster
alignment of sequences. In Automata, Languages, and Programming - 41st International
Colloquium, ICALP 2014, Copenhagen, Denmark, July 8-11, 2014, Proceedings, Part I, pages
39–51, 2014.
[3] Roberto Amadini. A survey on string constraint solving. ACM Computing Surveys (CSUR),
55(1):1–38, 2021.
[4] Dana Angluin. Finding patterns common to a set of strings. J. Comput. Syst. Sci., 21(1):46–
62, 1980.
[5] Alexander Artikis, Alessandro Margara, Martı́n Ugarte, Stijn Vansummeren, and Matthias
Weidlich. Complex event recognition languages: Tutorial. In Proceedings of the 11th ACM
International Conference on Distributed and Event-based Systems, DEBS 2017, Barcelona,
Spain, June 19-23, 2017, pages 7–10, 2017.
[6] Arturs Backurs and Piotr Indyk. Which regular expression patterns are hard to match?
In IEEE 57th Annual Symposium on Foundations of Computer Science, FOCS 2016, 9-11
October 2016, Hyatt Regency, New Brunswick, New Jersey, USA, pages 457–466, 2016.
[7] Johannes Bader, Simon Gog, and Matthias Petri. Practical variable length gap pattern matching. In Experimental Algorithms - 15th International Symposium, SEA 2016, St. Petersburg,
Russia, June 5-8, 2016, Proceedings, pages 1–16, 2016.
[8] Ricardo A. Baeza-Yates. Searching subsequences. Theor. Comput. Sci., 78(2):363–376, 1991.
[9] Laura Barker, Pamela Fleischmann, Katharina Harwardt, Florin Manea, and Dirk Nowotka.
Scattered factor-universality of words. In Proc. DLT 2020, volume 12086 of Lecture Notes in
Computer Science, pages 14–28. Springer, 2020.
[10] Gabriel Bathie and Tatiana Starikovskaya. Property testing of regular languages with applications to streaming property testing of visibly pushdown languages. In ICALP, volume 198
of LIPIcs, pages 119:1–119:17. Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2021.
[11] Amir M. Ben-Amram. The Euler path to static level-ancestors. CoRR, abs/0909.1030, 2009.
[12] Michael A. Bender and Martin Farach-Colton. The level ancestor problem simplified. Theor.
Comput. Sci., 321(1):5–12, 2004.
[13] Philip Bille, Inge Li Gørtz, Hjalte Wedel Vildhøj, and David Kofoed Wind. String matching
with variable length gaps. Theor. Comput. Sci., 443:25–34, 2012.
[14] Francine Blanchet-Sadri. Algorithmic Combinatorics on Partial Words. Discrete mathematics
and its applications. CRC Press, 2008.
[15] Mikolaj Bojanczyk. Factorization forests. In Developments in Language Theory, volume 5583
of Lecture Notes in Computer Science, pages 1–17. Springer, 2009.
[16] Karl Bringmann. Why walking the dog takes time: Frechet distance has no strongly subquadratic algorithms unless SETH fails. In 55th IEEE Annual Symposium on Foundations of
Computer Science, FOCS 2014, Philadelphia, PA, USA, October 18-21, 2014, pages 661–670,
2014.
[17] Karl Bringmann. Fine-grained complexity theory (tutorial). In 36th International Symposium on Theoretical Aspects of Computer Science, STACS 2019, March 13-16, 2019, Berlin,
Germany, pages 4:1–4:7, 2019.
[18] Karl Bringmann and Bhaskar Ray Chaudhury. Sketching, streaming, and fine-grained complexity of (weighted) LCS. In Proc. FSTTCS 2018, volume 122 of LIPIcs, pages 40:1–40:16,
2018.
12

<!-- page 13 -->

[19] Karl Bringmann and Marvin Künnemann. Multivariate fine-grained complexity of longest
common subsequence. In Proc. SODA 2018, pages 1216–1235, 2018.
[20] Sam Buss and Michael Soltys. Unshuffling a square is NP-hard. J. Comput. Syst. Sci.,
80(4):766–776, 2014.
[21] Manuel Cáceres, Simon J. Puglisi, and Bella Zhukova. Fast indexes for gapped pattern matching. In SOFSEM 2020: Theory and Practice of Computer Science - 46th International Conference on Current Trends in Theory and Practice of Informatics, SOFSEM 2020, Limassol,
Cyprus, January 20-24, 2020, Proceedings, pages 493–504, 2020.
[22] Peter Clifford and Raphaël Clifford. Simple deterministic wildcard matching. Inf. Process.
Lett., 101(2):53–54, 2007.
[23] Maxime Crochemore, Christophe Hancart, and Thierry Lecroq. Algorithms on strings. Cambridge University Press, 2007.
[24] Maxime Crochemore, Borivoj Melichar, and Zdenek Tronı́cek. Directed acyclic subsequence
graph — overview. J. Discrete Algorithms, 1(3-4):255–280, 2003.
[25] Patrick Dinklage, Johannes Fischer, Alexander Herlez, Tomasz Kociumaka, and Florian
Kurpicz. Practical Performance of Space Efficient Data Structures for Longest Common
Extensions. In 28th Annual European Symposium on Algorithms (ESA 2020), volume 173 of
Leibniz International Proceedings in Informatics (LIPIcs), pages 39:1–39:20, 2020.
[26] Bartlomiej Dudek, Pawel Gawrychowski, Garance Gourdel, and Tatiana Starikovskaya.
Streaming regular expression membership and pattern matching. In SODA, pages 670–694.
SIAM, 2022.
[27] Henning Fernau, Florin Manea, Robert Mercas, and Markus L. Schmid. Pattern matching
with variables: Efficient algorithms and complexity results. ACM Trans. Comput. Theory,
12(1):6:1–6:37, 2020.
[28] Lukas Fleischer and Manfred Kufleitner. Testing Simon’s congruence. In Proc. MFCS 2018,
volume 117 of LIPIcs, pages 62:1–62:13, 2018.
[29] Dominik D. Freydenberger, Pawel Gawrychowski, Juhani Karhumäki, Florin Manea, and
Wojciech Rytter. Testing k-binomial equivalence. In Multidisciplinary Creativity, a collection of papers dedicated to G. Păun 65th birthday, pages 239–248, 2015. available in CoRR
abs/1509.00622.
[30] Dominik D Freydenberger, Pawel Gawrychowski, Juhani Karhumäki, Florin Manea, and Wojciech Rytter. Testing k-binomial equivalence. arXiv preprint arXiv:1509.00622, 2015.
[31] Moses Ganardi, Danny Hucke, Daniel König, Markus Lohrey, and Konstantinos Mamouras.
Automata theory on sliding windows. In STACS, volume 96 of LIPIcs, pages 31:1–31:14.
Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2018.
[32] Moses Ganardi, Danny Hucke, and Markus Lohrey. Querying regular languages over sliding
windows. In FSTTCS, volume 65 of LIPIcs, pages 18:1–18:14. Schloss Dagstuhl - LeibnizZentrum für Informatik, 2016.
[33] Moses Ganardi, Danny Hucke, and Markus Lohrey. Randomized sliding window algorithms
for regular languages. In ICALP, volume 107 of LIPIcs, pages 127:1–127:13. Schloss Dagstuhl
- Leibniz-Zentrum für Informatik, 2018.
[34] Moses Ganardi, Danny Hucke, and Markus Lohrey. Sliding window algorithms for regular
languages. In LATA, volume 10792 of Lecture Notes in Computer Science, pages 26–35.
Springer, 2018.

13

<!-- page 14 -->

[35] Moses Ganardi, Danny Hucke, Markus Lohrey, and Tatiana Starikovskaya. Sliding window
property testing for regular languages. In ISAAC, volume 149 of LIPIcs, pages 6:1–6:13.
Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2019.
[36] Emmanuelle Garel. Minimal separators of two words. In Proc. CPM 1993, volume 684 of
Lecture Notes in Computer Science, pages 35–53, 1993.
[37] Pawel Gawrychowski, Maria Kosche, Tore Koß, Florin Manea, and Stefan Siemer. Efficiently
testing Simon’s congruence. In 38th International Symposium on Theoretical Aspects of Computer Science, STACS 2021, March 16-19, 2021, Saarbrücken, Germany (Virtual Conference),
pages 34:1–34:18, 2021.
[38] Nikos Giatrakos, Elias Alevizos, Alexander Artikis, Antonios Deligiannakis, and Minos N.
Garofalakis. Complex event recognition in the big data era: a survey. VLDB J., 29(1):313–
352, 2020.
[39] Hermann Gruber and Markus Holzer. From finite automata to regular expressions and back A summary on descriptional complexity. Int. J. Found. Comput. Sci., 26(8):1009–1040, 2015.
[40] Dan Gusfield. Algorithms on Strings, Trees, and Sequences — Computer Science and Computational Biology. Cambridge University Press, 1997.
[41] Leonard H. Haines. On free monoids partially ordered by embedding. Journal of Combinatorial
Theory, 6(1):94–98, 1969.
[42] Simon Halfon, Philippe Schnoebelen, and Georg Zetzsche. Decidability, complexity, and expressiveness of first-order logic over the subword ordering. In Proc. LICS 2017, pages 1–12,
2017.
[43] Jean-Jacques Hebrard. An algorithm for distinguishing efficiently bit-strings by their subsequences. Theor. Comput. Sci., 82(1):35–49, 22 May 1991.
[44] John E. Hopcroft and Jeffrey D. Ullman. Introduction to Automata Theory, Languages and
Computation. Addison-Wesley, 1979.
[45] Costas S. Iliopoulos, Marcin Kubica, M. Sohel Rahman, and Tomasz Walen. Algorithms
for computing the longest parameterized common subsequence. In Combinatorial Pattern
Matching, 18th Annual Symposium, CPM 2007, London, Canada, July 9-11, 2007, Proceedings, pages 265–273, 2007.
[46] Russell Impagliazzo and Ramamohan Paturi. On the complexity of k-SAT. J. Comput. Syst.
Sci., 62(2):367–375, 2001.
[47] Russell Impagliazzo, Ramamohan Paturi, and Francis Zane. Which problems have strongly
exponential complexity? J. Comput. Syst. Sci., 63(4):512–530, 2001.
[48] Prateek Karandikar, Manfred Kufleitner, and Philippe Schnoebelen. On the index of Simon’s
congruence for piecewise testability. Inf. Process. Lett., 115(4):515–519, 2015.
[49] Prateek Karandikar and Philippe Schnoebelen. The height of piecewise-testable languages
with applications in logical complexity. In Proc. CSL 2016, volume 62 of LIPIcs, pages 37:1–
37:22, 2016.
[50] Prateek Karandikar and Philippe Schnoebelen. The height of piecewise-testable languages
and the complexity of the logic of subwords. Log. Methods Comput. Sci., 15(2), 2019.
[51] Juha Kärkkäinen, Peter Sanders, and Stefan Burkhardt. Linear work suffix array construction.
J. ACM, 53(6):918–936, 2006.
[52] Sarah Kleest-Meißner, Rebecca Sattler, Markus L. Schmid, Nicole Schweikardt, and Matthias
Weidlich. Discovering event queries from traces: Laying foundations for subsequence-queries
with wildcards and gap-size constraints. In 25th International Conference on Database Theory,
ICDT 2022, 29th March-1st April, 2022 Edinburgh, UK, 2022.
14

<!-- page 15 -->

[53] Mitja Kulczynski, Florin Manea, Dirk Nowotka, and Danny Bøgsted Poulsen. ZaligVinder:
A generic test framework for string solvers. J. Software: Evolution and Process, page e2400,
2021.
[54] Dietrich Kuske. The subtrace order and counting first-order logic. In Proc. CSR 2020, volume
12159 of Lecture Notes in Computer Science, pages 289–302, 2020.
[55] Dietrich Kuske and Georg Zetzsche. Languages ordered by the subword order. In Proc.
FOSSACS 2019, volume 11425 of Lecture Notes in Computer Science, pages 348–364, 2019.
[56] Marie Lejeune, Julien Leroy, and Michel Rigo. Computing the k-binomial complexity of the
Thue-Morse word. In Proc. DLT 2019, volume 11647 of Lecture Notes in Computer Science,
pages 278–291, 2019.
[57] Julien Leroy, Michel Rigo, and Manon Stipulanti. Generalized Pascal triangle for binomial
coefficients of words. Electron. J. Combin., 24(1.44):36 pp., 2017.
[58] Chun Li and Jianyong Wang. Efficiently mining closed subsequences with gap constraints. In
SDM, pages 313–322. SIAM, 2008.
[59] Chun Li, Qingyan Yang, Jianyong Wang, and Ming Li. Efficient mining of gap-constrained
subsequences and its various applications. ACM Trans. Knowl. Discov. Data, 6(1):2:1–2:39,
2012.
[60] T. Liang, A. Reynolds, N. Tsiskaridze, C. Tinelli, C. Barrett, and M. Deters. An efficient smt
solver for string constraints. Formal Methods in System Design, 48(3):206–234, 2016.
[61] Daniel Lokshtanov, Dániel Marx, and Saket Saurabh. Lower bounds based on the exponential
time hypothesis. Bull. EATCS, 105:41–72, 2011.
[62] David Maier. The complexity of some problems on subsequences and supersequences. J. ACM,
25(2):322–336, April 1978.
[63] Alexandru Mateescu, Arto Salomaa, and Sheng Yu. Subword histories and Parikh matrices.
J. Comput. Syst. Sci., 68(1):1–21, 2004.
[64] William E. Riddle. An approach to software system modelling and analysis. Comput. Lang.,
4(1):49–66, 1979.
[65] Michel Rigo and Pavel Salimov. Another generalization of abelian equivalence: Binomial
complexity of infinite words. Theor. Comput. Sci., 601:47–57, 2015.
[66] Arto Salomaa. Connections between subwords and certain matrix mappings. Theoret. Comput.
Sci., 340(2):188–203, 2005.
[67] P. Saxena, D. Akhawe, S. Hanna, F. Mao, S. McCamant, and D. Song. Kaluza web site.
webblaze.cs.berkeley.edu/2010/kaluza/. Accessed: 2020-01-01.
[68] P. Saxena, D. Akhawe, S. Hanna, F. Mao, S. McCamant, and D. Song. A symbolic execution
framework for JavaScript. In Proc. 31st S&P, pages 513–528, 2010.
[69] Shinnosuke Seki. Absoluteness of subword inequality is undecidable. Theor. Comput. Sci.,
418:116–120, 2012.
[70] Alan C. Shaw. Software descriptions with flow expressions. IEEE Trans. Software Eng.,
4(3):242–254, 1978.
[71] Imre Simon. Hierarchies of events with dot-depth one — Ph.D. thesis. University of Waterloo,
1972.
[72] Imre Simon. Piecewise testable events. In Autom. Theor. Form. Lang., 2nd GI Conf., volume 33 of LNCS, pages 214–222, 1975.

15

<!-- page 16 -->

[73] Imre Simon. Words distinguished by their subwords (extended abstract). In Proc. WORDS
2003, volume 27 of TUCS General Publication, pages 6–13, 2003.
[74] Ken Thompson. Regular expression search algorithm. Commun. ACM, 11(6):419–422, 1968.
[75] Zdenek Tronı́cek. Common subsequence automaton. In Proc. CIAA 2002 (Revised Papers),
volume 2608 of Lecture Notes in Computer Science, pages 270–275, 2002.
[76] Wen-Guey Tzeng. A polynomial-time algorithm for the equivalence of probabilistic automata.
SIAM J. Comput., 21(2):216–227, 1992.
[77] Virginia Vassilevska Williams. Hardness of easy problems: Basing hardness on popular conjectures such as the strong exponential time hypothesis (invited talk). In 10th International
Symposium on Parameterized and Exact Computation, IPEC 2015, September 16-18, 2015,
Patras, Greece, pages 17–29, 2015.
[78] Georg Zetzsche. The complexity of downward closure comparisons. In Proc. ICALP 2016,
volume 55 of LIPIcs, pages 123:1–123:14, 2016.
[79] Haopeng Zhang, Yanlei Diao, and Neil Immerman. On complexity and optimization of expensive queries in complex event processing. In International Conference on Management of
Data, SIGMOD 2014, Snowbird, UT, USA, June 22-27, 2014, pages 217–228, 2014.

A

Regular and Length Constraints

A.1

On our choice of Representing Regular Constraints By DFAs (and
potential complications resulting from alternative representations)

The question on how should one represent the regular constraints (both when they appear alone,
and when they appear in conjunction with length constraints) seems a valid and interesting question
to us. Natural options would have been NFAs, DFAs, or regular expressions (regexes). We have
chosen to represent them with DFAs and, in the following, we argue that this is a reasonable (and
not unusual) choice.
Impact of the choice. Before putting forward our argument, we note that this choice impacts
only two results, namely Corollary 3.2(2) and Theorem 3.1 (the algorithmic results for matching
subsequences with regular gap constraints). All the other results hold irrespective of the representation used for the regular language present in the constraints.
Motivation of the choice. As said, we have chosen to represent the regular constraints as
deterministic finite automata (DFAs), rather than representing them as regular expressions (regex)
or non-deterministic finite automata (NFAs). This representation is not unusual when dealing with
testing whether factors of words are in a regular language, see, e. g., the overview of the results
of Imre Simon regarding factorization forests and their application to such problems [15] or the
works related to sliding window algorithms for regular languages [31, 33, 34, 35].
Additionally, we note that in the case of regular constraints of constant complexity (i. e., where
O(1) states are needed for each regular constraint) there is no difference in the asymptotic complexity of our algorithms w. r. t. the chosen representation of the constraints: DFA, NFA, or regex.
Moreover, all the lower bounds would still hold as stated. To the same end, it is worth noting
that in some of the works on sliding window algorithms for regular languages, the DFAs for the
respective regular languages are assumed to have constant size [35]; a deeper discussion of such
restriction in that setting is made in [31].
Finally, regular constraints of constant (and relatively small) complexity are quite usual in
practice, as they can model simple constraints such as the presence/absence of some letters in
a string, filtering according to the presence/absence of some constant strings, restrictions on the
order in which some symbols appear in a word, etc.
The survey [3] and the references therein provide examples of small regular constraints (in the
sense that they are accepted by automata with a small number of states, or described by short
regexes) which appear and are relevant in the area of string solving, and, as such, formal verification.
As string solving is an area in which regular constraints on strings play an important role, we
16

<!-- page 17 -->

have investigated Kaulza [68, 67, 60], one the standard benchmarks containing string constraints,
developed based on practical applications of string solving (in particular, symbolic execution), and
usually used in the evaluation of string solvers (as mentioned in [53]). In this investigation, we
have focused on the regular constraints (appearing alone or in conjunction with length constraints
or other types of constraints) and their complexity (for simplicity, we present here the length of
the regex specifying them and the number of states in a minimal DFA accepting them). We have
used the BASC tool (https://b4sc.github.io/) to extract the wanted information from the
respective benchmark, and have obtained the following results.
• The Kaluza benchmark contains 47305 instances, out of which 20740, that is around 43%,
contain regular constraints.
• In total, there are 207038 regular constraints (specified as regular expressions) appearing in
these instances (there can, of course, be more constraints in each instance, sometimes even
more constraints for the same variable).
• All these regular expressions have length at most 20. The average length of the regexes
occurring in Kaluza is lower than 8. The NFAs cannonically obtained from these regexes
have, in average, 17 states.
• 99% of the minimal DFAs corresponding to these regexes have at most 20 states, and the
average number of states in these minimal DFAs is lower than 11.
DFAs, NFAs, regexes: differences in size (in theory). Folklore results show that there
are regular languages L for which the size of the shortest regular expression describing L is logarithmic w. r. t. the size of the minimal DFA accepting L. However, there are also regular languages
L for which the size of the DFA accepting L is logarithmic w. r. t. the size of the shortest regular
expression describing L [39, Example 23 in the Arxiv version]. So, among DFAs and regexes there
is no representation of a regular language which is guaranteed to be exponentially smaller than the
other one. As DFAs are particular cases of NFAs, it is clear that there is no advantage in choosing
DFAs over NFAs; however, dealing with unrestricted NFAs induced some complications which we
were not able to solve in the case of Theorem 3.1 and obtain an algorithm which matches the lower
bounds we were able to show (more on this below).
On the impacted results. So, let us address now more throughly the two results which are
impacted by the choice of the model used to describe regular gap constraints.
In the case of Corollary 3.2(2), one can easily show that Match with regular constraints can
be solved in O(|w| size(gc)) time when the constraints are specified as NFAs or regexes, using a
dynamic programming strategy (this is discussed later in the Appendix). So, the difference is that,
unsurprisingly, the total size of the automaton appears here instead of the number of states.
The case of Theorem 3.1 is more complicated (and it would make sense to revisit this paragraph
after reading its complete proof, appearing later in this appendix). In that case, the regular
constraints appear in conjunction with length constraints. The key idea used in our efficient
algorithm solving Match is that the traces of the computations of each automaton encoding the
constraints on suffixes of the word are essentially linear lists of pairs (position in the word, state
of the automaton); that is, they do not fork, because the automaton is deterministic. This allows
us to represent the totality of these traces as a forest of disjoint trees. This property is already
important for the first generic step of our algorithm, but it is crucial for the later steps: in the case
when the regular constraints would be given as regexes or NFAs, the first generic step would still
produce a graph Gt+1 , but this would be a directed acyclic graph, without the nice disjointness
properties which are used later in the algorithm to get a good running time (for instance, the
sets Sq from the simpler variant described in the proof of Theorem 3.1 would not be disjoint and
managing them would be more time consuming). Hence, the data structures and ideas we use in the
later steps of that algorithm (e. g., level ancestor data structures or our marking procedure), and
which are custom-designed for such forests of disjoint trees, seem to need non-trivial adaptations
and extensions to work for directed acyclic graphs (as it would be the case when implementing the
first generic step of the algorithm in the case of constraints defined by regexes or NFAs) within a
similar rectangular complexity.

17

<!-- page 18 -->

It seems that this is also a good point to emphasise that both the more complicated data
structures (w. r. t. those needed to get the results in Corollary 3.2) as well as the usage DFAs
in the framework of Theorem 3.1 are a consequence of the interplay between regular and length
constraints, which complicates the matching problem Match significantly.
Thus, with respect to Theorem 3.1, we preferred to stick to the representation of regular
constraints by DFAs, as, in this setting, we have obtained matching upper and lower bounds (in
Theorem 3.3) for the complexity of the matching problem, and we think that this is mathematically
interesting. It is an interesting open problem whether a similar result can be obtained when
specifying the constraints by NFAs (or regular expressions); there does not seem to be an easy way
to modify the construction below to work for nondeterministic automata or regexes.

A.2

Conjunctions of Regular and Length Constraints?

After the discussion on the way regular constraints are represented, it is maybe a good point to
also briefly mention that dealing with conjunctions of regular and length constraints is also not
unusual at all. In our investigation of Kaluza string solving benchmark, where both these types of
constraints are usual, we have noted the following.
• The Kaluza benchmark contains 47305 string solving instances. From these, 20740 instances
contain regular constraints (approximatively 43%) and 21246 instances contain length constraints (approximatively 44%).
• There is a large overlap between the instances containing regular and length constraints.
There are 19812 instances which contain both types of constraints. This corresponds to 95%
of the instances containing regular constraints, and to 93% of the instances containing length
constraints.
In general, length constraints can be more general than lower and upper bounds on the length of
the variables (which intuitively correspond to the gaps in our setting); they can be linear relations
between the lengths of variables and constants. Our result in Theorem G.1 shows that Match
becomes NP-hard in the respecting setting. However, a visual inspection of a random sample of
the Kaluza instances showed that in many cases the length constraints already are, or can be
immediately reduced, to the simpler case of only ower and upper bounds on the length of the
variables.
To conclude, at least when considering the representative Kaluza string solving benchmark, it
is very common to have combinations of regular and length constraints and, at least to a certain
extent, these are not very complex.

B

Some Details Omitted From the Preliminaries

B.1

Computational Model

The computational model we use is the standard unit-cost RAM with logarithmic word size: for
an input of size n, each memory word can hold log n bits. Arithmetic and bitwise operations with
numbers in [1 : n] are, thus, assumed to take O(1) time. Numbers larger than n, with ` bits,
are represented in O(`/ log n) memory words, and working with them takes time proportional to
the number of memory words on which they are represented. In all the problems, we assume
that we are given one word w (respectively, two words w and w0 ), with |w| = n (respectively,
|w| = n ≥ |w0 | = m), over an alphabet Σ = {1, 2, . . . , σ}, with |Σ| = σ ≤ n. Whenever a gapped
sequence (p, gc) is also part of the input of the problem, then |p| ≤ n and p is also over the same
alphabet Σ defined above. The way gc is given is discussed in the main part of the paper. So, in
general, we assume that the processed words are sequences of integers (called letters or symbols),
each fitting in O(1) memory words. This is a common assumption in string algorithms: the input
alphabet is said to be an integer alphabet. For a more detailed general discussion on this model
see, e. g., [23].

18

<!-- page 19 -->

B.2

Basic Stringology Data Structures

For a length-n word w we can build in O(n) time the suffix array structure, as well as the longest
common extension (also known as longest common prefix) data structures allowing us to retrieve
in constant time the length of the longest common prefix of any two suffxes w[1..n] and w[1..j] of
w (for details, see, e. g., [40, 51, 25], and the references therein).

C

Some Details About Gap Constrained Subsequences Omitted From The Main Part

C.1

Size Measure of Our Instances

Remark C.1. Let (p, gc) be a gapped sequence over Σ, and let w ∈ Σ∗ . If |p| > |w|, then
p is not a gc-subsequence of w, and SubSeq(gc, w) = ∅. Hence, we generally assume that for
the matching problem we always have |p| ≤ |w|. If (p, gc) is a gapped sequence with length
constraints, then L− (i) > |w| for some i ∈ [|p| − 1] directly implies that p is not a gc-subsequence
of w, and SubSeq(gc, w) = ∅, and if L+ (i) > |w| for some i ∈ [|p| − 1], we could replace L+ (i)
by |w| to obtain an equivalent instance. In particular, note that we can check L− (i) > |w| and
L+ (i) > |w| in linear time in |w|. Consequently, we shall also assume that L− (i) ≤ L+ (i) ≤ |w|
(or L− (i) ≤ L+ (i) ≤ min{|w|, |w0 |} for the non-equivalence, non-containment and non-universality
problems).
In particular, the remark above shows that in all the analysis problems we consider in Section
4, we can replace the length constraints by regular constraints without exponentially increasing
the overall size of the input instance, which consists in one word w (or two words w and w0 ), and a
gapped sequence (p, gc) (or simply a tuple of gap constraints gc). Indeed, instead of the constraint
(L− (i), L+ (i)) we can use the regular-constraint Ci = {u ∈ Σ∗ | L− (i) ≤ |u| ≤ L+ (i)}, which is
accepted by a DFA with O(L+ (i)) states. Clearly, this would lead to an exponential growth in the
size of the string describing the respective gap constraint, but as L+ (i) is always upper bounded
by the length of the input string(s), the overall growth in the size of the string describing the input
instance is just polynomial.

C.2

Constraints for the Prefix and Suffix Gaps

For a word w, with |w| = m, a (k − 1)-tuple of gap constraints gc, and an embedding e : [k] → [m]
which satisfies gc, we note that gc only restricts the form of gaps induced by e in w, i. e., of
the factors w[e(i) + 1..e(i + 1) − 1], for i ∈ [k − 1]. So, it is natural to ask how one could
also restrict the factors w[1..e(1)] and w[e(k)..m]. Our formalism can model this case too. Let
gc = (C0 , C1 , . . . , Ck−1 , Ck ) be a (k + 1)-tuple of gap constraints for some p ∈ Σk , i. e., C0 and Ck
are interpreted as constraints for the prefix w[1..e(1)] and the suffix w[e(k)..m] induced by some
embedding e with p e w. We can describe this setting by defining p0 = $p$, where $ ∈
/ Σ is a
new symbol, and by interpreting gc as a tuple gc0 of gap constraints for p0 , i. e., a tuple of gap
constraints that constrains the k + 1 actual gaps of p0 . Then, there is an embedding e that embeds
p into w such that gc is satisfied (i. e., with the prefix and suffix gap constraints) if and only if
there is an embedding e0 that embeds p0 into w such that gc0 is satisfied in the sense defined in
Section 2.
In this context, we can revisit our comments on regular pattern matching from Section 2.
Let π = w0 x1 w1 · · · xk wk be a regular (Angluin-style [4]) pattern, where xi , for i ∈ [k] are variables,
and wi ∈ Σ∗ , for i ∈ {0} ∪ [k], are constant factors. We can determine whether a word w matches
π exactly (so, whether we can replace x1 , . . . , xk with words from Σ∗ such that π becomes equal
to w) as follows. We define the gapped-sequence (p, gc) with p = $w0 w1
P·t· · wk $ where $ is a new
letter and gc[i] = Σ∗ (or (0, +∞) as length constraint), for all i = 1 + j=0 |wj | with 0 ≤ t ≤ k,
and gc[i] = {ε} (respectively, (0, 0), as length constraint) otherwise. Then we need to determine if
an embedding e exists which satisfies gc w. r. t. $w$. Note that the addition of $ forces the first
(respectively, last) symbols of p and $w$ to be aligned, and, as such, it enforces the exact matching
of w to π. Moreover, it also accounts for the case when one of w0 or wk is not ε, and essentially

19

<!-- page 20 -->

allows us to impose gap constraints on the initial and final “gaps” occurring before and after the
embedding of the constant factors of π in w.

C.3

Related Problems Expressed by Subsequences with Gap Constraints

Let us discuss some examples that show how classical string matching or formal languages problems
can be expressed in terms of subsequences with gap constraints.
• String Matching: To find all occurrences of a word p in a word w, it is enough to define the
gapped-sequence (p, gc) with gc[i] = {ε} (or, (0, 0) as length constraint), for all i ∈ [|p| − 1],
and find all the embeddings e which satisfy gc w. r. t. w.
• Regular Pattern Matching: Let π = w0 x1 w1 · · · xk wk be a regular (Angluin-style [4]) pattern,
where xi , for i ∈ [k], are variables, and wi ∈ Σ+ , for i ∈ {0} ∪ [k]. To find all matches
of π to factors of w, we would need to find all assignments of the variables x1 , . . . , xk to
words from Σ∗ such that π becomes equal to a factor of w. Therefore, we define the gapped∗
sequence (p, gc) with p =
0 w1 · · · wk and gc[i] = Σ (respectively, gc[i] = (0, +∞) as length
Pw
t
constraint), for all i = j=0 |wj | with 0 ≤ t ≤ k − 1, and gc[i] = {ε} (respectively, (0, 0),
as length constraint) otherwise. Then we need to find all the embeddings e which satisfy gc
w. r. t. w. See also Appendix C.2.
• Partial Words (also known as strings with don’t cares [14]): Consider a partial word π =
w0 `1 w1 · · · `k wk , where  is a wildcard, which can be replaced by any letter of the alphabet Σ, `i ≥ 1 for all i ∈ [k], and wi ∈ Σ+ , for i ∈ {0} ∪ [k]. To find all matches of π to factors
of w, we need to find a replacement of each  from π by a letter of Σ such that π becomes
equal to a factor of w. Therefore, we define the gapped-sequence (p, gc) with pP
= w0 w1 · · · wk
t
and gc[i] = Σ`t+1 (respectively, (`t+1 , `t+1 ) as length constraint), for all i = j=0 |wi | with
0 ≤ t ≤ k − 1, and gc[i] = {ε} (respectively, (0, 0), as length constraint) otherwise. Then we
need to find all the embeddings e which satisfy gc w. r. t. w.
• Generalisations of the Downward Closure and Simon’s Congruence: Consider the (n − 1)-tuple
of gap constraints gcn such that gcn [i] = Σ∗ with i ∈ [n − 1]. Then, SubSeq(gcn , w) is the
set of all length-n subsequences of a string w. Thus, in this context, the problem of checking
SubSeq(gc, w) = SubSeq(gc, w0 ) for some gc is a natural extension of the well-known Simon’s congruence (see the introduction). Moreover, we also note that the downward closure
of {w}, i. e., the set of subsequences of w, is ∪n≤|w| SubSeq(gcn , w). This can be extended to

languages, and we can define the downward closure of L as ∪w∈L ∪n≤|w| SubSeq(gcn , w) .
Hence, considering a set C of (more complicated) gap constraints, the set ∪w∈L (∪gc∈C SubSeq(gc, w))
can be seen as a generalisation of the downward closure of languages, the downward closure
induced by C. It is interesting to note that, although sometimes it cannot be computed
(see [78], and the references therein), the downward closure of any language is necessarily regular [41]. When considering the downward closure induced by a finite set C of gap
constraints, then ∪w∈L (∪gc∈C SubSeq(gc, w)) is also finite. However, if C is not finite, then
∪w∈L (∪gc∈C SubSeq(gc, w)) is not necessarily regular anymore. Indeed, if C is the set of
n-tuples of gap constraints gc0n such that gc0n [i] = {ε}, with i ∈ [n − 1], then the downward
closure of L induced by C describes the set of factors of the words of L, which, in general, is
not necessarily regular. We consider it an interesting open question to understand for which
sets C of gap constraints the induced downward closure is a regular language for all languages.

D

Proof Details Omitted From Section 3

Before starting the proof of Theorem 3.1, let us state a general result which would serve as an
example for a basic approach for Match for all the cases approached in this paper.
Proposition D.1. Match with polynomial constraints can be solved in polynomial time.

20

<!-- page 21 -->

Proof. Assume |w| = n, |p| = m, and gc = (C1 , . . . , Cm−1 ), where, for every i ∈ [m − 1], the
constraint Ci is given as a (black box) procedure check(i, u), which checks u ∈ Ci in time O(P (|u|))
for some polynomial P .
To solve the matching problem for the gapped sequence (p, gc) and the word w we will first
preprocess w, by computing a three dimensional n × n × m array M [·][·][·] where, for i, j ∈ [n] and
k ∈ [m], M [i][j][k] = 1 if w[i..j] ∈ Ck and M [i][j][k] = 0, otherwise. The elements of M can be
computed naı̈vely in O(n2 mP (n)) time by simply checking for all i and j whether check(i, w[i..j])
returns true.
Further, we define a two dimensional n × m array D[·][·], where D[i][k] = 1 if and only if p[1..k]
can be embedded in w[1..i] by an embedding e which satisfies gck , for gck = (C1 , . . . , Ck−1 ), and
maps position k of p to w[i]. Otherwise, D[i][k] = 0.
The elements of the matrix D can be computed easily by dynamic programming. We initially
set all elements of D to 0. Then, we set D[i][1] = 1 if and only if w[i] = p[1]. Then, for k ≥ 2, we
set D[i][k] = 1 if and only if there exists j < i such that D[j][k − 1] = 1 and M [j + 1][i − 1][k] = 1
and w[i] = p[k]. Clearly, this shows how the matrix D can be computed in O(n2 m).
Finally, we decide that p gc w if and only if there exists j ≤ n such that D[j][m] = 1. By the
above, we can decide this in O(n2 mP (n)) time.
In general, the naı̈ve result in the previous proposition can be improved, as we will see in the
following.
We now see the actual Proof of Theorem 3.1. For certain steps of the algorithm found at
the core of this proof, we will sketch how they can be implemented in a simpler but less efficient
manner. After the proof, we will also sketch how our approach can be simplified for the case when
only regular or length constraints are used. None of these simplifications seem to work in the case
when both regular and length constraints are used.
Proof. Assume |w| = n, |p| = m, and gc = (C1 , . . . , Cm−1 ) such that, for i ∈ [m − 1], Ci =
((L− (i), L+ (i)), Ai )), where Ai = (Qi , q0,i , Fi , δi ) are DFAs defining the regular constraints and
(L− (i), L+ (i)) are pairs of numbers defining the length constraints.
We begin with a simple observation: in time linear in the size of the gapped sequence-part of
the input (that is, in O(|p| + size(gc)) time), we can identify the non-zero constraints of gc. Let
i1 , . . . , ik−1 ∈ [m − 1] be numbers such that Ci 6= {ε}, for all i ∈ {i1 , . . . , ik−1 }, and Ci = {ε}, for
Pk−1
all i ∈
/ {i1 , . . . , ik−1 }. Clearly, states(gc) = j=1 |Qij | and states(gc) ≥ k − 1.
Moreover, with i0 = 0 and ik = m, we define the words pj = p[ij−1 + 1..ij ], for j ∈ [k].
For i ∈ [k], we denote ni = |p1 · · · pi | and n0 = 0. From an algorithmic point of view, one can
preprocess the word p in linear time to compute the non-empty words p1 , . . . , pk .
Further, we can construct in linear time the suffix array and the longest common extension
(also known as longest common prefix) data structures for the word x = wp. These allow us to
check in constant time whether w[j + 1..j + |pi |] = pi , for all j and i, by simply checking in O(1)
time whether the longest common prefix of x[j + 1..n + m] and x[n + |pi−1 |..n + m] has at least
length |pi |.
All the steps described above are part of a preprocessing part of our algorithm.
The main part of our algorithm consists in a dynamic programming approach.
We define and aim to compute a two dimensional n × k array D[·][·], where D[i][`] = 1 if and
only if p[1..n` ] can be embedded in w[1..i] and this embedding satisfies the first ` − 1 non-zero
constraints Ci1 , . . . , Ci`−1 of gc and maps p` to the suffix of length |p` | of w[1..i]. Otherwise,
D[i][`] = 0.
To compute the elements of the matrix D, we proceed as follows.
Firstly, we initialize all the elements of D with 0. We then set D[i][1] = 1 if and only if w[1..i]
ends with p1 , i. e., w[i − |p1 | + 1..i] = p1 .
Further, assume that, for some t ∈ [k − 1], we have computed D[·][j], for all j ≤ t, and we
want to compute D[·][t + 1]. This part is the most involved part of our algorithm and its aim is
computing an array ft+1 [·], with n elements, such that ft+1 [i] = 1 if and only if there exists a
position j such that D[j][t] = 1, w[j + 1..i] ∈ L(At ), and L− (t) ≤ |w[j + 1..i]| ≤ L+ (t).
We now present a procedure for the efficient computation of ft+1 [·]. This procedure
consists in several generic steps.

21

<!-- page 22 -->

The first generic step of this procedure is the following.
We first collect in a list Lt+1 , in increasing order, all the positions i of w such that D[i][t] = 1.
Let j1 < . . . < jr be the elements of Lt+1 .
Then, we compute a graph Gt+1 and a two dimensional n × |Qt | array Mt+1 [·][·] as follows.
Initially, all elements of Mt+1 are set to 0. We then set Mt+1 [j][q0,t ] = 1 for all j ∈ Lt+1 . Now,
we compute Gt+1 : its nodes are pairs (i, q) with i ∈ [n] and q ∈ Qt . Intuitively, the nodes of
Gt+1 will be exactly those pairs (i, q) for which M [i][q] = 1, and the edges are [(i, q), (i + 1), q 0 ]
where δt (q, w[i + 1]) = q 0 . The construction of Gt+1 is immediate: for i from j1 to n, and for
q ∈ Qt , if Mt+1 [i][q] = 1 and δt (q, w[i + 1]) = q 0 , we add the edge [(i, q), (i + 1), q 0 ] to Gt+1 and set
Mt+1 [i + 1][q 0 ] = 1.
Intuitively, Gt+1 consists in the union, over j ∈ Lt+1 , of the (not necessarily disjoint) paths
j
j
, for
)], where δt (q0,t , w[j + 1]) = q1j and δt (qrj , w[j + r + 1]) = qr+1
[(j, q0,t ), (j + 1, q1j ), . . . , (n, qn−j
all r ∈ [n − j − 1]. Intuitively, such a path records the trace of the computation of At on the input
j
w[j + 1..n]. An important observation is that if two such paths [(j, q0,t ), (j + 1, q1j ), . . . , (n, qn−j
)]
0

0

j
and [(j 0 , q0,t ), (j 0 + 1, q1j ), . . . , (n, qn−j
0 )] intersect, then they stay identical after their first common
node; this is, indeed, true because At is a deterministic finite automaton. Consequently, Gt+1 is
a collection of disjoint trees (i. e., a forest) T1 , T2 , . . . , Tz . As there are no edges between any pair
of nodes (n, q) and (n, q 0 ), with q, q 0 ∈ Qt , we can compute these trees by a series of depth first
searches starting in the nodes (n, q), with q ∈ Qt , of Gt+1 . Clearly, when computing these nodes,
we can associate to each node (j, q) the label `j,q if and only if (j, q) is contained in the tree T`j,q .
In other words, the label allows us to check quickly which tree contains each node of Gt .
From now on, each of the trees Ti will be seen as a rooted tree, whose root is the single node
of the form (n, q) contained in that tree; the root of Ti , for i ∈ [z], is denoted (n, qi ) and the leaves
of each of these trees are of the form (j, q0,t ), with j ∈ Lt+1 .
So, at the end of the first main step, we have obtained the rooted trees T1 , . . . P
Tz . For simplicity,
p
we denote by |Ti | the number of nodes in Ti (the size of Ti ). Clearly, p ≤ |Qt | and i=1 |Ti | ≤ n|Qt |.
Sketch of a simple but less efficient variant: At this point, we could use a rather direct approach
to compute ft+1 [·]. We give an informal sketch of this idea, without too many implementation
details as, anyway, we have a more efficient variant below. The role of this short interlude is to
explain the need for a heavier data structures machinery in the efficient variant. The reader not
interested in this simpler but less efficient variant, can skip directly to the description of the second
generic step of our algorithm.
In general, we want to identify each node (j, q) (of a tree Ti ) in our forest for which there
exists a path P of length `, with L− (t) ≤ ` ≤ L+ (t), and a leaf (j 0 , q0,t ) of Ti such that the
path P starts with (j 0 , q0,t ) and ends with (j, q). Or, in other words, δt (q0,t , w[j 0 + 1..j]) = q and
L− (t) ≤ |w[j 0 + 1..j]| ≤ L+ (t). For such a node, we can set ft+1 [j] = 1, as there exists a word
w[j 0 + 1..j] of length `, with L− (t) ≤ ` ≤ L+ (t), such that j 0 ∈ Lt+1 and δt (q0,t , w[j 0 + 1..j]) is a
final state.
So, we will consider j from 1 to n in increasing order and, while doing this, we maintain a
collection of disjoint sets Sq , with q ∈ Qt , included in {1, . . . , n}. Initially, all the sets in the
collection are empty. When considering j = i, we construct the sets Sq0 = ∪q0 ∈Qt ,δ(q0 ,w[i])=q Sq0 ,
for q ∈ Qt ; afterwards, we insert i in Sq0 0,t if and only if i ∈ Lt+1 . Then, we simply set Sq ← Sq0 ,
for all q ∈ Qt . Intuitively, Sq contains those positions j 0 ∈ {1, . . . , i} such that there is a path
between (j 0 , q0 ) and (i, q); note now that these sets are pairwise disjoint because the automaton
At is deterministic. Therefore, we set ft+1 [i] = 1 if and only if there exists a final state f ∈ Ft
such that Sf contains a position j 0 with L− (t) ≤ |w[j 0 + 1..j]| ≤ L+ (t). A careful implementation
of the sets Sq from our collection (e. g., based on AVL-trees, which allows us fast searching, and
an implementation of tree-union in which simply the elements of the smaller tree are inserted, one
by one, in the larger tree) would lead to a computation of ft+1 [·] in O(n|Qt | polylog(n)). In the
following, we present a more efficient variant for the computation of ft+1 [·].
In the second generic step of our efficient procedure, we construct level ancestor data structures
for the trees Ti , with i ∈ [p].
More precisely, the Level Ancestor Problem is defined as follows (see [12]). In a rooted tree T , v
is an ancestor of u if the shortest path (i. e., the only simple path) from the root to u goes through
v; the depth of a node u, denoted depth(u), is the number of edges on the shortest path from u to
the root of T . For a rooted tree T , let LAT (u, d) = v, where v is an ancestor of u and depth(v) = d,

22

<!-- page 23 -->

if such a node exists, or ↑ otherwise. The Level Ancestor Problem consists in a preprocessing phase
and a querying phase:
• Preprocessing: A rooted tree T with N vertices.
• Querying: For a node u in the rooted tree T , query levelAncestorT (u, d) returns LAT (u, d),
if it exists, and false otherwise.
A simple and elegant solution for this problem which has O(N ) preprocessing time and O(1) time
for query-answering can be found in, e. g., [12] (see also [11] for a more involved discussion).
So, for each tree Ti we can compute in O(|Ti |) time data structures allowing us to answer
levelAncestorTi queries in O(1) time.
In the third generic step, we have a procedure of marking nodes in the trees Ti , for i ∈ [z].
So, let us consider one of these trees Ti and explain how we process it. In this step, the marking
can be maintained, e. g., using a boolean n × |Qt | matrix which simply keeps track for each node
whether it was marked or not; we do not go in further details with this, at it would simply make
the exposition more heavy.
In this step we would like to mark, for any leaf (j, q0,i ) of Ti , all the ancestors (d, q) of (j, q0,i )
such that L− (t) ≤ |w[j + 1..d]| = d − j ≤ L+ (t).
At a high level, this is done as follows. For each such leaf (j, q0,t ) of Ti , we compute (j +
j
L− (t), q−
) = levelAncestorTi ((j, q0,t ), n − j − L− (t)). We mark this node and all its ancestors while
going upwards on the path from (j + L− (t), q−j ) towards the root of Ti , until we reach a node
j
j
(j + L+ (t), q+
). We also mark (j + L+ (t), q+
) and then we run the marking procedure for the next
leaf of Ti .
To avoid marking the same nodes multiple times, we can implement the marking procedure as
follows.
By going through the elements of Lt+1 (which were ordered increasingly), we can produce for
each node Ti the stack Lt+1,i containing all leaves of Ti , ordered decreasingly (top to bottom)
w. r. t. the first component of each node (j, q). Note that the sets/stacks Lt+1,i , for i ∈ [z], define
a partition of Lt+1 .
Assume, now, that the leaves of Ti are (j1,i , q0,t ), (j2,i , q0,t ), . . ., (jsi ,i , q0,t ), with j1,i > j2,i >
. . . > jsi ,i (and note that si is the number of leaves of Ti ).
g
Now, for g from 1 to si , we compute (jg,i + L− (t), q−
) = levelAncestorTi ((jg,i , q0,t ), n − jg,i −
−
L (t)). We mark this node and all its ancestors while going upwards on the path from (jg,i +
g
g
L− (t), q−
) towards the root of Ti , until we reach a node (jg,i + L+ (t), q+
) or we meet an already
g
g
+
+
marked node. If we reach (jg,i + L (t), q+ ), we also mark (jg,i + L (t), q+
), and then we continue
with the marking procedure for the next leaf of Ti ; otherwise, if we meet a marked node, we stop
the marking for this leaf and continue with the marking procedure for the next leaf of Ti .
To understand why it is correct to stop the marking for a leaf once we meet a marked node (c, q),
it is enough to note that this marked node must have been marked when another leaf (jf,i , q0,t )
with a greater first component (i. e., jf,i > jg,i ) was considered. In that case, the path of marked
f
f
f
nodes between (jf,i + L− (t), q−
) and (jf,i + L+ (t), q+
), ends with the node (jf,i + L+ (t), q+
) for
g
+
+
+
which we have jf,i + L (t) > jg,i + L (t). Thus, the node (jg,i + L (t), q+ ) is actually on the
f
g
) is already
path from (c, q) to (jf,i + L+ (t), q+
), so every node between (c, q) and (jg,i + L+ (t), q+
marked and we do not need to mark them again.
Once we have completed the marking for all trees Ti , with i ∈ [z], we have achieved the following:
a node (j, q) is marked if and only if there exists a path P of length `, with L− (t) ≤ ` ≤ L+ (t),
and a leaf (j 0 , q0,t ) of Ti such that the path P starts with (j 0 , q0,t ) and ends with (j, q). Or, in
other words, δt (q0,t , w[j 0 + 1..j]) = q and L− (t) ≤ |w[j 0 + 1..j]| ≤ L+ (t).
In the fourth, and final, generic step, we simply set, for i from 1 to n, ft+1 [i] = 1 if and only if
there exists a state q ∈ Ft such that the node (i, q) is marked. This means that ft+1 [i] = 1 if and
only if there exists a word w[j + 1..i] of length `, with L− (t) ≤ ` ≤ L+ (t), such that j ∈ Lt+1 and
δt (q0,t , w[j + 1..i]) is a final state (i. e., w[j + 1..i] ∈ Ct ).
Clearly, ft+1 [·] is correctly computed with respect to the definition given when we introduced
this array.
After we compute the array ft+1 [·], we return to the main algorithm, and move on to
the computation of the elements D[·][t + 1] from our main array D.
23

<!-- page 24 -->

We simply set D[i][t + 1] = 1 if and only if w[i − |pt+1 | + 1..i] = pt+1 and ft+1 [i − |pt+1 |] = 1.
It is not hard to see that D[·][t + 1] is correctly computed.
To conclude, in the final step of our algorithm, we decide that the input gapped sequence
(p, gc) matches the word w if and only if there exists j such that D[j][k] = 1.
We now discuss the complexity of our approach. The preprocessing part is done in O(n +
size(gc)) time. The initialization of the array D[·][·] and the computation of its first column D[·][1]
can be done in linear time O(n). Now, for each t ≥ 1, the computation of ft+1 takes O(n|Qt |)
time. Indeed,P
the first generic step of this computation takes O(n|Qt |) time. The second generic
z
step takes O( i=1 |Ti |) = O(n|Qt |) time. The third generic step takes time proportional to the
number of marked nodes. As eachPnode of each tree is marked at most once, then the time needed
z
to complete this step is also O( i=1 |Ti |) = O(n|Qt |). Finally, the fourth generic step can be
performed in O(n|Ft |) time. So, overall, the computation of ft+1 [·] takes, as claimed, O(n|Qt |)
time. Therefore, computing all the elements of D[·][t + 1] can be done in O(n|Qt |) time (including
here the computation of ft+1 [·]). Consequently, all the elements of D[·][·] can be computed in
Pk−1
O( i=1 n|Qi |) = O(n states(gc)) time. Finally, deciding whether the input gapped sequence
(p, gc) matches the word w based on the array D[·][·] can be done in O(n) time. Therefore, the
considered matching problem can be solved in O(|w| states(gc)+|w|+size(gc)) = O(|w| states(gc)+
size(gc)).
Regarding Corollary 3.2, as each length constraint can be represented as reg-len constraints
by adding the regular constraint Σ∗ (specified as DFA with one state), statement (1) follows
immediately. Statement (2) follows from Theorem 3.1.
However, let us now comment more on how the case of Match with either regular constraints
or length constraints can be handled.
In the case of length constraints only, we use the same approach and, just like in the proof
above, assume that, for some t ∈ [k − 1], we have computed D[·][j], for all j ≤ t, and we want to
compute D[·][t + 1]. We again compute an array ft+1 [·], with n elements, such that ft+1 [i] = 1 if
and only if there exists a a position j such that D[j][t] = 1 and L− (t) ≤ |w[j + 1..i]| ≤ L+ (t). This
can be done in a simple way: we go through the positions i from 1 to n and maintain in a dequeue,
in increasing order, the positions j such that D[j][t] = 1 and L− (t) ≤ |w[j +1..i]| ≤ L+ (t). For each
i, we need to simply check if the dequeue is empty or not; if not, then we can set ft+1 [i] = 1 (and
otherwise leave ft+1 [i] = 0). We then continue as in the algorithm above, after ft+1 is computed.
We obtain the time complexity stated in Corollary 3.2(1).
In the case of regular constraints only, we use the same general approach and, once more,
assume that, for some t ∈ [k − 1], we have computed D[·][j], for all j ≤ t, and we want to compute
D[·][t + 1]. We again compute an array ft+1 [·], with n elements, such that ft+1 [i] = 1 if and only
if there exists a a position j such that D[j][t] = 1 and w[j + 1..i] ∈ L(At ). This can be done in
a simple way: we go through the positions i from 1 to n and maintain an array S[·] of size |Qt |,
where S[q] = min{j ∈ Lt+1 | δ(q0,t , w[j + 1..i]) = q} (or S[q] = ∞ if the respective set is emtpy).
Clearly, if we have computed S[·] for position i − 1, we can immediately update it for position i.
Then, once S is updated for position i, we need to simply check if there exists f ∈ Ft such that
S[f ] 6= ∞; if yes, then we can set ft+1 [i] = 1 (and otherwise leave ft+1 [i] = 0). We then continue as
in the algorithm above, after ft+1 is computed. We obtain trivially the time complexity stated in
Corollary 3.2(2). In the case when the regular constraints are given as NFAs (or regexes) instead
of DFAs, the computation of ft+1 [·] can be done in time O(n|At |), where |At | is the number of
transitions of the automaton At (or the size of the regex representing L(At )).
It is worth noting that none of these simple approaches can be extended to work for the case
when regular and length constraints are combined, and, in fact, our approach for that case proposes
a data structure that creates the environment in which the main ideas in these simple approaches
can interact efficiently.

D.1

Proof of Theorem 3.3

Before stating the proof, note that the reduction is presented in a slightly different way compared
to the proof sketch from the main part of this paper. More precisely, what is called codeb (~bi ) in

24

<!-- page 25 -->

≤1
≤3
≤1
≤3
the following corresponds to the part Cb (~bi ) ↔ # ↔ # ↔ # ↔ # of the proof sketch, while the
part Cb (~bi ) from the proof sketch is defined as ui later in the proof.

Proof. We reduce OV to the matching problem for gapped sequences with length constraints. We
consider an instance of OV: A = {~a1 , . . . , ~an } and B = {~b1 , . . . , ~bn }, with A, B ⊂ {0, 1}d ; we can
assume that d ≥ 2. We transform this OV-instance into an instance of the matching problem
for gapped sequences with length constraints. We need to define a word w (which, intuitively,
corresponds to the set A), and a gapped sequence (p, gc) with length constraints (which corresponds
to the set B). Interestingly, the gapped sequence (p, gc) will additionally fulfil the property that
gc[i] = (0, ki ), where 0 ≤ ki ≤ 6, for all i ∈ [|p| − 1]. We will show that p gc w if and only if there
exist two vectors ~ai and ~bj which are orthogonal.
To simplify the exposition, when representing the gapped sequence (p, gc) with p = p[1] · · · p[m],
gc[1]

gc[2]

we will use the notation (p, gc) = p[1] ↔ p[2] ↔ · · ·

gc[m−1]

↔

p[m]. Moreover, we omit the symbol

gc[i]

≤k

↔ from the notation of (p, gc) if and only if gc[i] = (0, 0). If gc[i] = (0, k), then we write ↔
gc[i]

(0,k)

instead of the symbol ↔ = ↔ when writing the notation for (p, gc). For instance, if p = abab
(1,5)

≤6

and gc[1] = (0, 0), gc[2] = (1, 5), and gc[2] = (0, 6), we denote (p, gc) = ab ↔ a ↔ b.
So, let us now define our reduction. Once more, we start with two sets A = {~a1 , . . . , ~an }
and B = {~b1 , . . . , ~bn }, with A, B ⊂ {0, 1}d . We assume ~ai = (a1i , . . . , adi ) and ~bi = (b1i , . . . , bdi ),
for all i ∈ [n]. As usually, we define a series of gadgets, which will be combined to produce the
word w and the gapped sequence (p, gc). The alphabet over which we define the words w and p is
Σ = {0, 1, #, @}
The first set of gadgets is defined as follows:
• codea (0) = 010, codea (1) = 100;
• codeb (0) = 10, codeb (1) = 01.
The second set of gadgets is obtained based on the first set defined above:
• For i ∈ [n], codea (~ai ) is the word
!
d
Q
(# codea (0)## codea (aji )## codea (0)#) .
j=1

• For i ∈ [n − 1], codeb (~bi ) is the gapped sequence
defined as
!
d−1
Q
≤1
≤1
≤3
≤3
≤1
≤1
≤1
≤3
≤1
≤3
(# ↔ codeb (bji ) ↔ ## ↔ ## ↔ #) # ↔ codeb (bdi ) ↔ # ↔ # ↔ # ↔ # ↔ #.
j=1

• For i = n, codeb (~bn ) is the gapped sequence
! defined as
d−1
Q
≤1
≤5
≤1
≤1
≤3
≤3
≤1
(# ↔ codeb (bjn ) ↔ ## ↔ ## ↔ #) # ↔ codeb (bdn ) ↔ # ↔ .
j=1

We say that each codea (~ai ) contains three tracks. The first track of the concatenation


d
Y
j
 (# codea (0)## codea (ai )## codea (0)#)
j=1

consists in the prefix # codea (0)# of each factor of the concatenation defining codea (~ai ). The
second track consists in the factors # codea (aji )# occurring in the middle of each factor in the
concatenation above. Finally, the third track consists in the factors # codea (0)# occurring as a
suffix of each factor of the concatenation defining codea (~ai ). To ease the understanding, we can
highlight these tracks in the concatenation by placing the factors defining the ith track between
brackets [i · · · ]i :


d
Y
codea (~ai ) =  ([1 # codea (0)#]1 [2 # codea (aji )#]2 [3 # codea (0)#]3 ) .
j=1

25

<!-- page 26 -->

Clearly, just like the case of usual parentheses, these indexed brackets are not part of the string
codea (~ai ).
The final set of gadgets, corresponding to the word w and the gapped sequence (p, gc) are
defined as follows.

n−1

n−1
Q
Q
@ codea (~ai ) @ codea (~an )
@ codea (~ai ) @.
• w=
i=1

i=1
≤5

• (p, gc) = @ ↔

n−1
Q

!

≤6

codeb (~bj ) ↔ codeb (~bn )@.

j=1

We continue with the correctness proof for our reduction, i. e., the proof of the claim that
the instance of OV defined by A and B contains two orthogonal vectors ~ai and ~bj if and only if
p gc w. Note that ~ai and ~bj are orthogonal is equivalent to afi bfj = 0, for f ∈ [d].
We first note that, for g, h ∈ {0, 1}, we have g · h = 0 if and only if codeb (h) is a factor
of codea (g). Indeed codeb (0) = 10 is a factor of codea (0) = 010, codeb (1) = 01 is a factor of
codea (1) = 100, codeb (1) = 01 is a factor of codea (0) = 010, but codeb (1) = 01 is not a factor of
codea (1) = 100. See Figure 3.
codea (0) 010

10 codeb (0)

codea (1) 100

01 codeb (1)

Figure 3: Gadgets for the encoding of single bits. The edges represent “factor of” relations.
Secondly, for i ∈ [n], we define the gapped sequence
!
d−1
Q
≤1
≤3
≤3
≤1
≤1
j ≤1
(ui , gci ) =
(# ↔ codeb (bi ) ↔ ## ↔ ## ↔ #) # ↔ codeb (bdi ) ↔ #.
j=1

Please note that gci can be seen as the restriction of gc on the factor ui of p.
Let us now consider some i, ` ∈ [n]. We analyse the possible embeddings e of ui in the word
codea (~a` ), which satisfy gci . Note that codea (~a` ) contains 6d #-symbols, while ui contains 6d − 4
#-symbols. Thus, in an embedding of ui in codea (~a` ), the first three #-symbols of ui must be
aligned to some of the first seven #-symbols of codea (~a` ). By the fact that, in an embedding e
which satisfies gci , we have to embed the two symbols of {0, 1} occurring between the first two
#-symbols of ui (i. e., the string codeb (b1i )) into a factor of codea (~a` ) which ends before the seventh
# of codea (~a` ), and also because the second and third #-symbols of ui have no gap between them,
we get that the symbols codeb (b1i ) are embedded as a factor of either the first factor codea (0) of
the first track, or the first factor codea (0) of the third track of codea (~a` ), or of the factor codea (a1` )
of the second track of codea (~a` ). Now, if codeb (b1i ) is embedded, as just described, in the factor
corresponding to track q ∈ {1, 2, 3} of codea (~a` ), then, by the fact that there are exactly six #
symbols between codeb (bji ) and codeb (bj+1
), we get that all the factors codeb (bji ) will be embedded
i
in the corresponding factors of codea (~a` ) of the same track q. Such embeddings are always possible
when gci is satisfied, and no other embeddings with these properties are possible. So, in an
embedding of ui in the word codea (~a` ), the way we embed codeb (b1i ) selects a track of codea (~a` ),
in which all the factors codeb (bji ) will be embedded. We say that e selects track q of codea (~a` ) if
codeb (b1i ) is embedded in track q of codea (~a` ).
We now claim that ~a` and ~bi are orthogonal if and only if there exists an embedding e which
satisfies gci and, for all j ∈ [d], maps the factor codeb (bji ) of ui to a factor of codea (aj` ) of codea (~a` ).
In other words, ~a` and ~bi are orthogonal if and only if there is an embedding e which satisfies gci
and that selects the second track of codea (~a` ). Clearly, if there exists an embedding e which
satisfies gci and maps the factor codeb (bji ) of ui to a factor of codea (aj` ) of codea (~a` ), for all j ∈ [d],
respectively, then bji aj` = 0, for all j ∈ [d]. So, a` and bi are orthogonal. For the other implication,
we proceed as follows. If ~a` and ~bi are orthogonal, then codeb (bji ) is a factor of codea (aj` ) for
j ∈ [d]. As mentioned above, we can construct the embedding e as follows. To begin with, for
j ∈ [d], e maps the symbols of the factor codeb (bji ) of ui to a factor of codea (aj` ), and the # symbol
26

<!-- page 27 -->

578

#

(ui , gci ) =

¡ codeb (b1i ) ¡

Æ1

Æ1

#

#

(ui , gci ) =
codea (˛a¸ ) =

#

codea (0)

#

#

#

Æ3

#

#

¡ codeb (b1i ) ¡

Æ3

#

#

Æ3

#

#

codea (aji )

#

#

#

Æ1

¡

Æ1

#
#

¡
¡

¡ codeb (bi1 ) ¡

Æ1

codea (0)

Æ1

¡ codeb (b2i ) ¡

Æ1

Æ1

#

#

¡

#

#

#

#

#

#

Æ3

#

#

Æ3

#

#

codea (0)

¡

Æ3

#

...

¡ codeb (b2i ) ¡

#

...

Æ3

#

...

codea (aji )

#

...

¡

Æ1

¡

Æ1

Figure 4: Possible embeddings of ui in codea (~a` ), selecting the first, second, and, respectively,
third track of codea (~a` ).

≤1

≤1

and the first #-symbol of u1 is at most 5. Then, between the suffix # ↔ codeb (bd1 ) ↔ # of (u1 , gc1 )
≤1

≤3

≤1

≤3

≤6

and the factor (u2 , gc2 ), the gapped sequence (p, gc) contains the factor ↔ # ↔ # ↔ # ↔ # ↔.
This means that u2 will be embedded in codea (~a`+1 ), and, moreover, it will select one of the
tracks q or q + 1 (as there can be at most 18 symbols between u1 and u2 , so track 3 cannot be
reached in the case q = 1). See Tables 1 and 2. Now, for i ≥ 2, as long as ui is embedded in
track q ∈ {1, 2} of codea (~a`+i−1 ), the process continues as above, and we reach the conclusion
that ui+1 must be embedded in track q or q + 1 of codea (~a`+i ). Assume now that ui is embedded
≤1

≤1

in track 3 of codea (~a`+i−1 ). Then, between the factor # ↔ codeb (bdi ) ↔ # of (ui , gci ) and
≤1

≤3

≤1

the first symbol of (ui+1 , gci+i ), the gapped sequence (p, gci ) contains the factor ↔ # ↔ # ↔
≤3

≤6

# ↔ # ↔. This means that ui+1 can be embedded either in the third track of codea (~a`+i ), or
!
d
Q
j
in the first track of the string w0 =
([1 # codea (0)#]1 [2 # codea (a`+i )#]2 [3 # codea (0)#]3 )

j=2
≤1
1
@[1 # codea (0)#]1 [2 # codea (a`+i+1 )#]2 [3 # codea (0)#]3 . Since ui+1 ends with the string ## ↔
≤1
≤1
≤1
codeb (bdi+1 ) ↔ #, and # ↔ codeb (bdi+1 ) ↔ # must be embedded in # codea (a1`+i+1 )#, we note
≤1
that this embedding of (ui+1 , gci+1 ) is impossible: we would have to embed the factor ## ↔
≤1
codeb (bdi+1 ) ↔ # into the factor @# codea (a1`+i+1 )# of w0 , which is not possible. Thus, ui+1 is

embedded in the third track of codea (~a`+i ).
From now on, we can continue as above, and note that all sequences uj , with j > i + 1, will
be embedded only in the third track of the corresponding factors of w. We conclude that in an
embedding e of p in w we simply embed each of the gapped sequences (ui , gci ) into factors codea (~a` )
of w. However, codeb (~bn ) must be embedded in such a way that the gap between the symbol of
codea (~a` ) to which the last #-symbol of codeb (~bn ) is mapped and an @ symbol is at most 5. This
means that codeb (~bn ) must have been embedded in track 2 or 3 of some codea (~a`0 ). Therefore, at
least one of codeb (~bi ) must have been embedded in track 2 of some codea (~a`00 ). Indeed, if codeb (~bn )
is embedded in track 2 of codea (~a`0 ), then this claim holds. If codeb (~bn ) is embedded in track 3
of codea (~a`0 ), then let codeb (~bp ) be such that p is maximum with the property that codeb (~bp ) was
27

Matching Problems and Static Analysis

Thirdly, we analyse when there exists an embedding e of p in w which satisfies gc.
Cleary, in such an embedding e, we map the first @-symbol of p to an @-symbol of w. So, let us
assume that the first @-symbol of p is mapped to the @-symbol of w occurring before an occurrence
of codea (~a` ). As u1 contains 6d #-symbols, and codea (~a` ) contains 6d − 4 #-symbols, and the
length of the gap between @ and the first #-symbol of u1 is at most 5, it follows that u1 is then
embedded in codea (~a` ), and this embedding satisfies gci (as defined above). Therefore, u1 selects a
track q of codea (~a` ). It is not hard to see that i ≤ 2 holds, as, according to gc, the gap between @

Subsequences With Gap Constraints:

(ui , gci ) =

23:16

occurring before (respectively, after) codeb (bji ) in ui to the # symbol occurring before (respectively,
after) codea (aj` ) in codea (~a` ). Then, for j ∈ [d − 1], e maps the four # symbols occurring between
)# in ui to the four # symbols occurring between # codea (aj` )#
# codeb (bji )# and # codeb (bj+1
i
j+1
and # codea (a` )# in codea (~a` ). This embedding clearly satisfies the gap constraints gci .
This proves our claim.
Before moving on with our proof, it is worth recalling what we have shown. The above claim
states that ~a` and ~bi are orthogonal if and only if there exists an embedding e which satisfies gci and
maps the factors codeb (bji ) of ui to the factors defining the second track of codea (~a` ), respectively
(i. e., e selects the second track of codea (~a` )). But, as mentioned, there exist other embeddings
e of ui in codea (~a` ), which satisfy gci , even if a` and bi are not orthogonal. In all these other
embeddings either the first of the third track of codea (~a` ) are selected. See Figure 4.

<!-- page 28 -->

w=

@

(p, gc) =

@

ε
≤5

↔

w1
u1

# codea (ad1 )#
≤1

≤3

# codea (0)#

@# codea (0)#

≤3

≤6

≤1

#↔#

↔#↔#↔

↔

w2
u2

Table 1: In this example, u1 is embedded in track 1 of codea (~a1 ), and then u2 is embedded in track
2 of codea (~a2 ); we cannot embed u2 in track 3 of codea (~a2 ), as it cannot be reached. Here, w1
(respectively, w2 ) is the factor of codea (~a1 ) (respectively, codea (~a2 )) found between the symbol to
which the first symbol of u1 is mapped and the symbol to which the last symbol of u1 (respectively,
u2 ) is mapped in the respective embedding.
w=

@

(p, gc) =

# codea (0)#

@

≤5

↔

w1
u1

# codea (0)#
≤1

≤3

↔#↔#

@

# codea (0)#

# codea (a12 )#

≤1

≤3

≤6

↔

#↔#

↔

w2
u2

Table 2: In this example, u1 is embedded in track 2 of codea (~a1 ), and then u2 is embedded in
track 3 of codea (~a2 ). Here, w1 (respectively, w2 ) is the factor of codea (~a1 ) (respectively, codea (~a2 ))
found between the symbol to which the first symbol of u1 is mapped and the symbol to which the
last symbol of u1 (respectively, u2 ) is mapped in the respective embedding.
not embedded in track 3 of some codea (~ar ). By the arguments given above, codeb (~bp ) must have
been embedded in track 2 of codea (~ar ). So, our claim that at least one of the gapped sequences
codeb (~bi ), say codeb (~bj ), must be embedded in track 2 of some codea (~a`00 ) holds.
This shows that if an embedding e of p in w, which satisfies gc, exists then there exist j and `00
such that codeb (~bj ) is embedded in track 2 of codea (~a`00 ). That is, ~bj and ~a`00 are orthogonal.
Now assume that there exist ~bj and ~a`00 which are orthogonal. We can construct an embedding
of p in w as follows: we embed codeb (~bi ) in track 1 of codea (~a`00 −j+i ), for i ∈ [j − 1], codeb (~bj )
in track 2 of codea (~a`00 ), and codeb (~bj+i ) in track 3 of codea (~a`00 +i ), for i ∈ [n − j]. By our
explanations, this is clearly possible.
In conclusion, an embedding e of p in w, which satisfies gc, exists if and only if there exist j
and `00 such that codeb (~bj ) is embedded in track 2 of codea (~a`00 ). This is equivalent to saying that
~bj and ~a`00 are orthogonal.
This shows that our reduction is correct. The instance of OV defined by A and B contains two
orthogonal vectors if and only if the instance of the matching problem for gapped sequences with
length constraints defined by w and (p, gc) can be answered positively, or, equivalently, p gc w.
Moreover, the word w and the gapped sequence (p, gc) can be constructed in O(nd) time and we
have that |w|, |p| ∈ Θ(nd), and the number of bits needed to describe gc is also Θ(nd).
Assume now that there exists a solution for the matching problem for gapped sequences with
length constraints running in O(|w|g |p|h ) with g + h = 2 −  for some  < 0. This would lead to
a solution for OV running in O(nd + (nd)2− ), a contradiction to the OV-conjecture. The same
argument holds for solutions running in O(|w|2− ).
In fact, as nz(gc) ∈ Θ(nd), one can show that if there exists a solution of the considered
matching problem running in O(|w|g nz(gc)h ) with g + h = 2 −  for some  < 0, then there exists
a solution for OV running in O(nd + (nd)2− ), a contradiction to the OV-conjecture. This proves
our statement.
@

# codea (0)#

# codea (a12 )#

≤1

≤3

≤6

↔

#↔#

↔

w2
u2

@

# codea (0)#

≤1

≤3

#↔#

↔

# codea (a13 )#
≤1

≤3

↔#↔#

ε
≤6

↔

w3
u3

Table 3: Following the example in Table 1, u2 is embedded in track 3 of codea (~a2 ), and then
u3 is embedded in track 3 of codea (~a3 ). Here, w2 (respectively, w3 ) is the factor of codea (~a2 )
(respectively, codea (~a3 )) found between the symbol to which the first symbol of u2 (respectively,
u3 ) is mapped and the symbol to which the last symbol of u2 (respectively, u3 ) is mapped in the
respective embedding.

28

<!-- page 29 -->

@

# codea (0)#

# codea (a12 )#

≤1

≤3

≤6

↔

#↔#

↔

w2
u2

@

# codea (0)#

≤1

≤3

#↔#

↔

# codea (a13 )#
≤1

≤3

↔#↔#

# codea (0)#
≤6

↔

w30
u3

Table 4: Following the example in Table 1, u2 is embedded in track 3 of codea (~a2 ), and then u!3
d
Q
could be embedded in track 1 of w30 =
([1 # codea (0)#]1 [2 # codea (aj3 )#]2 [3 # codea (0)#]3 )
j=2

@[1 # codea (0)#]1 [2 # codea (a14 )#]2 [3 # codea (0)#]3 . This is, however, impossible, as it would imply
the mapping of a ## factor of u3 to a factor #@ or @# of w30 . Here, w2 is the factor of codea (~a2 )
found between the symbol to which the first symbol of u2 is mapped and the symbol to which the
last symbol of u2 is mapped in the respective embedding.

D.2

Proof of Corollary 3.4

Proof. The result follows in the same way, and based on the same reduction, as Theorem 3.3. The
only observation is that each of the length gap constraints used in that proof (which are all of the
form (0, `) for ` ≤ 6, so constant) can be expressed as a regular constraint, and encoded using a
DFA with a constant number of states. Indeed, if we have strings over Σ and the length constraint is
(k, `), for some constants k and ` with k ≤ `, then we have a DFA with `+2 states {0, 1, . . . , `, `+1}
with the transition δ(i, a) = i + 1, for all a ∈ Σ and i ∈ {0, . . . , `}, and δ(` + 1, a) = ` + 1, for all
a ∈ Σ. The final states of this DFA are {k, k + 1, . . . , `}. Clearly, this DFA has a constant number
of states. Therefore, states(gcp ) ∈ Θ(nd). The result follows.

D.3

Lower bounds for |Σ| = 2

Proof Sketch. We can actually adapt the proof of Theorem 3.3 by simply using a block encoding
of the symbols used in that reduction over a binary alphabet {a, b}.
We rewrite the symbols of w and (p, gc) according to the rules 0 ← a5 b a a a b5 , 1 ← a5 b a b a b5 ,
# ← a5 b b a a b5 , and @ ← a5 b b b a b5 ; the strings used to replace the symbols {0, 1, #, @} are
called code-blocks in the following. Very importantly, when rewriting the gapped sequence (p, gc)
by these rules, the gaps between any two consecutive symbols from {a, b} inside the encoding of
one of {0, 1, #, @} (i. e., the gaps between consecutive symbols inside a code-block) are zero.
Moreover, we adapt the non-zero gaps previously used in our reduction by multiplying all the
bounds with 14 (i.e., the length of the block codes).
Now, the proof of Theorem 3.3 works exactly as described above, as the prefixes a5 and b5
used in each encoding ensure that code-blocks can only align with corresponding code-blocks in an
embedding of (p, gc) in w.
We preferred, for the sake of accessibility, to state and prove Theorem 3.3 for |Σ| = 4, rather
than for |Σ| = 2. That is, we preferred to not complicate further a (stable) proof which is already
quite technically involved, with the added benefit being relatively minor. We think that the ideas
we gave in this sketch are convincing enough to support the claim that the result holds for |Σ| = 2
as well, and a full proof will be added in the full journal version of this paper.

E

The Analysis Problems for Subsequences

We briefly overview what is known about the three analysis problems for simple types of gap
constraints.
In the case of constraints gc = ({ε}, . . . , {ε}) or gc = (Σ`1 , . . . , Σ`k ), i. e., when the corresponding gapped sequences are words or partial words, all three problems can be solved in polynomial
time, as SubSeq(gc, w) has polynomial size in |w|. More efficient solutions can be easily obtained,
based on, e. g., string processing data structures like suffix arrays. In the case of constraints
gc = (Σ∗ , . . . , Σ∗ ), i. e., when we deal with classical subsequences, SubSeq(gc, w) is no longer of
polynomial size, but the equivalence problem corresponds to the problem of testing the Simon
congruence of two words, which can be tested in linear time [9, 37]. Universality can also be solved
in linear time [43, 9], while containment can be solved in polynomial time.
29

<!-- page 30 -->

Proposition E.1. Con for gap constraints gc = ((0, +∞), . . . , (0, +∞)) can be solved in polynomial time.
Proof. We start with our input words w and w0 . For w we construct Aw , the subsequenceautomaton [24] which accepts all the subsequences of w.
Assume |w| = n. Then, Aw is a deterministic finite automaton, which has n+2 states {0, ..., n, n+
1}. The initial state of this automaton is 0, and all states i, with i ∈ [n] are final; the state n + 1 is
an error state. The transition are defined as follows: for all i ∈ [n], k ∈ [n − i], and a ∈ Σ, we have
a transition from state i to i + k, labelled with a, if and only if w[i + k] = a and w[i + 1..i + k − 1]
does not contain the letter a. Moreover, for all i ∈ [n] and a ∈ Σ, we have a transition from state
i to state n + 1, labelled with letter a, if and only if w[i + 1..n] does not contain a. For state n + 1
we have loop-transitions for all letters a ∈ Σ. It is straightforward that Aw accepts exactly the
non-empty subsequences of w and can be constructed in O(n|Σ|) time.
For the word w0 , with |w0 | = m, we construct the automaton Aw0 , as above, and then modify
it to obtain the automaton Bw0 by simply making the state m + 1 the single final state. Clearly,
Bw0 accepts all strings which are not subsequences of w0 . It can be constructed in O(m|Σ|) time.
Now, if gc consists of k − 1 constraints, we can observe that SubSeq(gc, w) ⊆ SubSeq(gc, w0 ) if
and only if there exists a word of length at most k accepted of length by Aw which is also accepted
by Bw0 . This can be checked in O(nm|Σ|) time by simply computing the shortest word in the
intersection of the language accepted by Aw with the language accepted by Bw0 . Our statement
follows.
It remains an interesting open problem whether a linear time algorithm exists for the problem
approached in the proposition above.

F

Proof Details Omitted From Section 4

F.1

Proof of Theorem 4.1

Proof. We first consider the case of length constraints. Let gc be a (k − 1)-tuple of length constraints, let w, w0 ∈ Σ∗ . In order to check SubSeq(gc, w) ⊆ SubSeq(gc, w0 ), it is sufficient to check
for every v ∈ Σk whether v gc w implies v gc w0 . This can be done by solving |Σ|k times the
matching problem for p, gc, and w and w0 , respectively. Since, by Corollary 3.2(1), the matching
problem with length constraints can be solved in time O(|w| nz(gc)), the statement of the theorem
for problem Con follows. Moreover, since Equ can be solved by solving two Con-instances, the
statement of the theorem for problem Equ follows. Finally, for Uni, it is sufficient to check for
every v ∈ Σk whether v gc w. Hence, analogously as before, the statement of the theorem for
problem Uni follows.
The case with reg-len or just regular constraints can be handled analogously, just by applying
the matching upper bounds O(|w| states(gc)) of Theorem 3.1 and Corollary 3.2(2).

F.2

Decision Problems Used for Lower Bounds

We first define two well-known decision problems.
The problem CNF-Sat is defined as follows: The input is a Boolean formula F in CNF, i. e.,
a set of clauses F = {c1 , c2 , . . . , cq } over some set of variables V = {v1 , v2 , . . . , vk }, i. e., for every
i ∈ [q], we have ci ⊆ {v1 , ¬v1 , . . . , vk , ¬vk }. The question is whether F is satisfiable, i. e., whether
there is an assignment π : V → {0, 1} that makes at least one literal of each clause ci true.
The (parameterised) problem k-Independent Set is defined as follows: The input is an undirected graph G = (V, E) with V = {v1 , v2 , . . . , vn } and some k ∈ [n]. The question is whether
G has a k-independent set, i. e., a set A ⊆ V with |A| = k and {u, u0 } ∈
/ E for every u, u0 ∈ V
0
with u 6= u . For convenience, we assume in the following that undirected graphs G = (V, E) are
represented by symmetric directed graphs, i. e., E is a symmetric binary relation over V .
For the sake of convenience, in the following we define reductions that will prove the statements
of Theorems 4.3, 4.2, and 4.4 only for the non-universality problem NUni. We will discuss in
Section F.11 how the statements of Theorems 4.3, 4.2, and 4.4 also follow for the problems NCon
and NEqu.
30

<!-- page 31 -->

F.3

The Meta Non-Universality Problem

We now define a meta non-universality problem (MetaNUni for short) that can be easily used to
express other intractable decision problems. An instance of this meta non-universality problem is
defined as follows.
Let Γ = {b1 , b2 , . . . , bm } be some set of size m, and let q, k ∈ N. An instance of the problem is
a (q × k)-matrix the entries Wi,j of which are subsets of Γ. For every i ∈ [q], we associate with row
i of the matrix the language L(Wi ) = Wi,1 · Wi,2 · · · Wi,k , i. e., we simply represent the elements
of Wi,1 × Wi,2 × . . . × Wi,k as length-k strings over Γ in the natural way. The question is then to
decide whether ∪i∈[q] L(Wi ) 6= Γk .
As an example, let Γ = {a, b, c, d, e}, q = 4, and k = 3. Then the following matrix is a possible
instance:


{a}
{b}
{a, c, d}
{c, d}
{b}
{a} 


 {a} {b, c, d, e}
{d} 
{e}
{b}
{c, e}
We observe that, e. g., W3,2 = {b, c, d, e} and W4,1 = {e}. Moreover, L(W1 ) = {a} · {b} ·
{a, c, d} = {a b a, a b c, a b d} and L(W4 ) = {e b c, e b e}. It can be easily seen that for this instance
we have ∪i∈[4] L(Wi ) 6= Γ3 , e. g., ∪i∈[4] L(Wi ) does not contains strings that start with b.
Next, we will show how the problem CNF-Sat and the parameterised problem k-CLIQUE can
be reduced to MetaNUni, which is comparatively simple. After this, we will show how MetaNUni
reduces to NUni with length constraints, which requires more work.

F.4

From CNF-Sat to MetaNUni

Let F = {c1 , c2 , . . . , cq } be a Boolean formula in CNF on variables {v1 , v2 , . . . , vk }. We define
alphabet Γ = {0, 1} and the (q × k)-matrix with the entries Wi,j as follows (note that q and k
are already defined as the number of clauses and the number of variables, respectively). For every
i ∈ [q] and j ∈ [k], we define


if vj ∈ ci ,
{0}
Wi,j = {1}
if ¬vj ∈ ci ,


{0, 1} if {vj , ¬vj } ∩ ci = ∅.
It can be verified with moderate effort, that for every i ∈ [q], L(Wi ) contains exactly the
Boolean assignments that do not satisfy clause ci (where Boolean assignments are represented as
length-k strings over {0, 1} in the obvious way). This means that ∪i∈[q] L(Wi ) is the set of all
non-satisfying assignments. Thus, ∪i∈[q] L(Wi ) 6= {0, 1}k if and only if F is satisfiable, i. e., the
constructed MetaNUni instance is positive if and only if F is satisfiable.
As an example, consider the clauses c1 = {v1 , ¬v2 , v3 }, c2 = {¬v1 , ¬v2 , v5 }, c3 = {v3 , v4 , v5 }
over the variables {v1 , v2 , . . . , v6 }, which yields the following MetaNUni instance:


{0}
{1}
{0} {0, 1} {0, 1} {0, 1}
 {1}
{1} {0, 1} {0, 1} {0, 1} {0} 
{0, 1} {0, 1} {0}
{0}
{0} {0, 1}
In this case, 100010 ∈
/ ∪i∈[3] L(Wi ), which means that 100010 is a satisfying assignment.

F.5

From k-Independent Set to MetaNUni

Let G = (V, E) be an undirected graph with V = {v1 , v2 , . . . , vn } and E = {e1 , e2 , . . . , em }, and
let k ∈ [n]. For technical reasons, we assume that every vi ∈ V has a loop, i. e., E is reflexive
(note that this modification does not change whether a given set is an independent set or not); in
particular, these loops are explicitly included in the set E. We define Γ = V .
We generally interpret strings w ∈ Γk as sets V (w) = {w[j] | j ∈ [k]} of at most k vertices.
For every r, s ∈ [k] with r 6= s, we say that w contains edge e ∈ E at positions r, s ∈ [k] if
e = (w[r], w[s]). Every string w ∈ Γk with |V (w)| < k satisfies w[r] = w[s] for some r, s ∈ [k]
31

<!-- page 32 -->

with r 6= s, and, since every vertex has a loop, this means that w necessarily contains an edge.
Hence, for every w ∈ Γk , we have that w contains at least one edge if and only if V (w) is not a
k-independent set (i. e., it is either not an independent set or it is a set of cardinality strictly less
than k).
We fix some bijection ν : {(i, r, s) ∈ [m] × [k] × [k] | r 6= s} → [mk(k − 1)]. For every i ∈ [m]
with ei = (u, v), and every r, s, j ∈ [k] with r 6= s, we define sets


{u} if j = r,
Wν(i,r,s),j = {v} if j = s,


V
else.
This concludes the definition of the reduction. Note that here the number of columns of the
constructed matrix corresponds to the number k, while the number of rows is q := mk(k − 1). Let
us explain this reduction intuitively.
For a fixed ei ∈ E and fixed positions r, s ∈ [k] with r 6= s, the set
L(Wν(i,r,s) ) = Wν(i,r,s),1 Wν(i,r,s),2 . . . Wν(i,r,s),k
contains exactly the strings w ∈ Γk that contain the edge ei at positions r and s. This means
that L(Wν(i,r,s) ) represents exactly the sets of cardinality at most k that contain edge ei (and are
therefore not k-independent sets). Note that if ei is a loop, then V (w) might be an independent
set, but one of cardinality strictly less than k.
As an example, assume that G contains an edge e9 = (v3 , v7 ) and that k = 3, then this edge is
represented in the matrix by the following rows:
row ν(9, 1, 2) :

{v3 }

{v7 }

V

row ν(9, 1, 3) :

{v3 }

V

{v7 }

row ν(9, 2, 1) :

{v7 }

{v3 }

V

row ν(9, 2, 3) :

V

{v3 }

{v7 }

row ν(9, 3, 1) :

{v7 }

V

{v3 }

row ν(9, 3, 2) :

V

{v7 }

{v3 }

For example, L(Wν(9,3,1) ) = {v7 v1 v3 , v7 v2 v3 , v7 v3 v3 , v7 v4 v3 , . . .} represents all vertex sets of
cardinality at most 3 that contain edge (v3 , v7 ) at positions 3 and 1; observe that v7 v3 v3 represents
the vertex set {v3 , v7 }. Moreover, if e11 = (v7 , v3 ), then rows ν(9, 1, 2) and ν(11, 2, 1) are the same.
This redundancy could be avoided, but it would unnecessarily complicate the reduction.
S
Lemma F.1. i∈[m],r,s∈[k],r6=s L(Wν(i,r,s) ) 6= Γk if and only if G has a k-independent set.
Proof. If G has a k-independent set A, then A = {vt1 , vt2 , . . . , vtk } with |A| = k such that, for
every `, `0 ∈ [k] with ` 6= `0 , (vt` , vt`0 ) ∈
/ E, which means
that every w ∈ Γk with V (w) = A
S
has no edge at any positions. This means that w ∈
/ i∈[m],r,s∈[k],r6=s L(Wν(i,r,s) ). Hence, w ∈
/
S
S
k
L(W
)
for
every
w
∈
Γ
with
V
(w)
=
A,
and
therefore
L(W
ν(i,r,s)
ν(i,r,s) ) 6=
i∈[m],r,s∈[k],r6=s
i∈[m],r,s∈[k],r6=s
k
Γ .
Next, assume that G has no k-independent set and let w ∈ Γk be arbitrarily chosen. If
|V (w)| < k, then there are r, s ∈ [k] with r 6= s and w[r] = w[s] = v` for some ` ∈ [n]. This
means that w ∈ L(Wν(i,r,s) ) with ei = (v` , v` ). If |V (w)| = k, then, since V (w) is not an independent set, there is an edge ei = (v` , v`0 ) with {v` , v`0 } ⊆ V (w), and therefore there are
r, s ∈ [k] with r 6= s and (w[r], w[s]) = ei , which means that w ∈ L(Wν(i,r,s) ). Consequently,
S
S
Γk ⊆ i∈[m],r,s∈[k],r6=s L(Wν(i,r,s) ), which means that i∈[m],r,s∈[k],r6=s L(Wν(i,r,s) ) = Γk .

F.6

Reduction From MetaNUni to NUni

We no define a reduction from MetaNUni to NUni with length constraints. By doing so, we obtain
a reduction from CNF-Sat to NUni with length constraints, and a (parameterised) reduction from
k-Independent Set to NUni with length constraints.
32

<!-- page 33 -->

Let Γ = {b1 , b2 , . . . , bm }, q, k ∈ N, and, for every i ∈ [q], j ∈ [k], let Wi,j ⊆ Γ. We transform
this MetaNUni instance into an instance of NUni with length constraints (i. e., a string and a
(k − 1)-tuple of gap constraints), as follows. We first define the alphabet Σ = Γ ∪ {#} (with
# ∈
/ Γ). Then we define a (k − 1)-tuple gc = (C1 , C2 , . . . , Ck−1 ) of gap constraints with Ci =
(L− (i), L+ (i)) = (m − 1, 3m − 1) for every i ∈ [k − 1] (recall that m is Γ’s cardinality). In
order to conclude the reduction, we have to construct a string K(W1 , . . . , Wq ) over Σ, such that
SubSeq(gc, K(W1 , . . . , Wq )) = Σk if and only if ∪i∈[q] L(Wi ) = Γk .
For every i ∈ [q] and j ∈ [k], let wi,j ∈ Γ∗ be some string representation of Wi,j , i. e., alph(wi,j ) =
Wi,j and |wi,j | = |Wi,j | ≤ m. For every i ∈ [q], we define the string
S(Wi ) = wi,1 (#)m−1 wi,2 (#)m−1 . . . (#)m−1 wi,k .
Any gc-subsequence of S(Wi ) may or may not contain occurrences of symbol #. However,
those gc-subsequence of S(Wi ) that do not contain occurrences of symbol # are exactly the strings
in L(Wi ). This is stated by the next lemma.
Lemma F.2. For every i ∈ [q], (SubSeq(gc, S(Wi )) ∩ Γ∗ ) = L(Wi ).
Proof. We first prove that (SubSeq(gc, S(Wi )) ∩ Γ∗ ) ⊆ L(Wi ). Let p ∈ (SubSeq(gc, S(Wi )) ∩ Γ∗ ).
This means that there is an embedding e that satisfies gc and p e S(Wi ). If, for some ` ∈
[k − 1], ` and ` + 1 are mapped by e to positions of the same factor wi,j for some j ∈ [k], then
|gape (S(Wi ), `)| ≤ |wi,j |−2 ≤ m−2 and therefore the lower length constraint L− (`) = m−1 would
be violated. Consequently, since p ∈ Γ∗ , the embedding e must be such that, for every j ∈ [k], j
is mapped to a position of wi,j . This directly implies that p ∈ L(Wi ).
Next, we prove that L(Wi ) ⊆ (SubSeq(gc, S(Wi )) ∩ Γ∗ ). Let u ∈ L(Wi ). By construction of
S(Wi ), for every j ∈ [k], there is an occurrence of u[j] in wi,j . Thus, there is an embedding e that
maps each j ∈ [k] to such an occurrence of u[j] in wi,j . Consequently, u e S(Wi ). It only remains
to observe that e satisfies the length constraints. To this end, let j ∈ [k − 1]. This means that
e maps j to a position of the prefix wi,j of the factor wi,j (#)m−1 wi,j+1 , and e maps j + 1 to a
position of the suffix wi,j+1 of the factor wi,j (#)m−1 wi,j+1 . Since |wi,j | ≤ m and |wi,j+1 | ≤ m, we
have that m − 1 ≤ |gape (S(Wi ), j)| ≤ 3(m − 1). This means that e satisfies the length constraints.
Hence, u ∈ (SubSeq(gc, S(Wi )) ∩ Γ∗ ).
The above lemma shows that gc-subsequences over Γ of the strings S(Wi ) corresponds to the
strings of ∪i∈[q] L(Wi ). We next define a string T that satisfies that all SubSeq(gc, T ) is the set of
all length-k strings over Σ with at least one occurrence of the symbol #.
For every i ∈ [k], let Ti = Ti,1 Ti,2 . . . Ti,k , where, for every j ∈ [k] \ {i}, Ti,j = b1 b2 . . . bm #m ,
and Ti,i = #m . We define the string T by
T = T1 (#3m )T2 (#3m ) . . . (#3m )Tk .
Lemma F.3. SubSeq(gc, T ) = {w ∈ Σk | |w|# ≥ 1}.
Proof. We first prove that SubSeq(gc, T ) ⊆ {w ∈ Σk | |w|# ≥ 1}, i. e., every string from
SubSeq(gc, T ) contains at least one occurrence of symbol #. For contradiction, we assume that
there is some p ∈ SubSeq(gc, T ) without any occurrence of #, which means that there is an embedding e that satisfies the length constraints gc such that p e T . Since e satisfies the upper length
bounds L+ (j) = 3m − 1 for every j ∈ [k − 1], there must be an ` ∈ [k] such that all positions
j ∈ [k] are mapped by e to positions of the factor T` (note that otherwise either p[j] = # for some
j ∈ [k], or |gape (T, j)| ≥ 3m for some j ∈ [k − 1]). Consequently, p e0 T` for some embedding e0
that satisfies the length constraints gc. By construction, T` contains k − 1 maximal factors over
alphabet Γ. By assumption, p ∈ Γ∗ , which, by the pigeonhole principle, means that there is at
least one j ∈ [k − 1] such that both j and j + 1 are mapped by e0 to the same maximal factor over
Γ. Since each such maximal factor over Γ has size at most m, it follows that |gape0 (T` , j)| = m − 2,
which violates the lower length bound L− (j) = m − 1 and is therefore a contradiction.
Next, we prove that {w ∈ Σk | |w|# ≥ 1} ⊆ SubSeq(gc, T ). To this end, let p ∈ {w ∈ Σk |
|w|# ≥ 1} be arbitrarily chosen, and assume that u[r] = # with r ∈ [k] (since |u|# ≥ 1, such an r
must exist). We recall that Tr = Tr,1 Tr,2 . . . Tr,k , where, for every j ∈ [k]\{r}, Tr,j = b1 b2 . . . bm #m ,
33

<!-- page 34 -->

and Tr,r = #m . We will construct an embedding e : [k] → [|Tr |] that satisfies the length constraints
gc such that p e Tr . Since Tr is a factor of T , this implies that p ∈ SubSeq(gc, T ), and therefore
{w ∈ Σk | |w|# ≥ 1} ⊆ SubSeq(gc, T ).
We map r to the first position of Tr,r = #m , and, for every j ∈ [k] \ {r}, we map j to the tth
position of Tr,j if p[j] = bt , and to the (m + 1)st position of Tr,j if p[j] = #. We observe that
this embedding e satisfies that every j is mapped to an occurrence of p[j] of Tr,j , i. e., p e Tr . It
remains to prove that e satisfies the length constraints gc.
Since every j ∈ [k] \ {r} is mapped to one of the first m + 1 occurrences of the length-2m factor
Tr,j , and position r is mapped to the first possition of the length-m factor Tr,r , it can be easily
seen that m − 1 ≤ |gape (Tr , j)| ≤ 3m − 1. Hence, e satisfies the length constraints.
We are now ready to append the gadgets developed above in order to define the complete string
K(W1 , . . . , Wq ) as follows:
K(W1 , . . . , Wq ) = T (#3m )S(W1 )(#3m )S(W2 )(#3m ) . . . (#3m )S(Wq ) .
Before concluding the proof of correctness of the reduction, we show the following lemma, which
states that every gc-sequence of K(W1 , . . . , Wq ) without occurrences of # must be mapped into
one of the factors S(Wi ).
Lemma F.4. Let p ∈ SubSeq(gc, K(W1 , . . . , Wq )) with p ∈ Γk . Then, for some i ∈ [q], p ∈
SubSeq(gc, S(Wi )).
Proof. Let p ∈ SubSeq(gc, K(W1 , . . . , Wq )) and let e be an embedding that satisfies the length
constraints gc, and p e K(W1 , . . . , Wq ). Since e satisfies the upper length bounds L+ (j) = 3m − 1
for every j ∈ [k − 1], all positions j ∈ [k] are mapped by e to positions of some factor S(Wi ) with
i ∈ [q], or all positions j ∈ [k] are mapped by e to positions of the factor T (note that otherwise
either p[j] = # for some j ∈ [k], or |gape (K(W1 , . . . , Wq ), j)| ≥ 3m for some j ∈ [k − 1]). This
means that, for some embedding e0 that satisfies gc, p e0 S(Wi ) for some i ∈ [q], or p e0 T . By
Lemma F.3, we know that SubSeq(gc, T ) = {w ∈ Σk | |w|# ≥ 1}; thus, since |p|# = 0, p e0 T is
not possible. Hence, p e0 S(Wi ) for some i ∈ [q], which means that p ∈ SubSeq(gc, S(Wi )).
Finally, the following lemma concludes the proof of correctness.
Lemma F.5. SubSeq(gc, K(W1 , . . . , Wq )) = Σk ⇐⇒ ∪i∈[q] L(Wi ) = Γk .
Proof. We first show that
SubSeq(gc, K(W1 , . . . , Wq )) =

[

(SubSeq(gc, S(Wi )) ∩ Γ∗ ) ∪ SubSeq(gc, T ) .

i∈[q]

The “⊇”-direction holds, since T and every S(Wi ) for every i ∈ [q] are factors of K(W1 , . . . , Wq ).
Now let p ∈ SubSeq(gc, K(W1 , . . . , Wq )) be arbitrarily chosen. If |p# | ≥ 1, then, by Lemma F.3,
p ∈ SubSeq(gc, T ). If |p# | = 0, then p ∈ SubSeq(gc, K(W1 , . . . , Wq )) and p ∈ Γk . Thus,
Lemma F.4 implies that p ∈ SubSeq(gc, S(Wi )) for some i ∈ [q], which means that p ∈ SubSeq(gc, S(Wi ))∩
Γ∗ . This shows that the “⊆”-direction holds as well.
Now we prove the if direction of the statement of the lemma and assume that ∪i∈[q] L(Wi ) = Γk .
By Lemma F.2, we know that (SubSeq(gc, S(Wi ))∩Γ∗ ) = L(Wi ) for every i ∈ [q]. With Lemma F.3
and the observation from above, this means that
[
SubSeq(gc, K(W1 , . . . , Wq )) =
(SubSeq(gc, S(Wi )) ∩ Γ∗ ) ∪ SubSeq(gc, T )
i∈[q]




=

[

L(Wi ) ∪ SubSeq(gc, T )

i∈[q]
k

= Γ ∪ {w ∈ Σk | |w|# ≥ 1} = Σk .

34

<!-- page 35 -->

In order to prove the only if direction, we assume that SubSeq(gc, K(W1 , . . . , Wq )) = Σk . With
our observation from above, this means that
[
Σk =
(SubSeq(gc, S(Wi )) ∩ Γ∗ ) ∪ SubSeq(gc, T )
i∈[q]

Applying Lemma F.3 yields
[
Σk =
(SubSeq(gc, S(Wi )) ∩ Γ∗ ) ∪ {w ∈ Σk | |w|# ≥ 1} .
i∈[q]

Since
that

S

∗
k
i∈[q] (SubSeq(gc, S(Wi ))∩Γ ) and {w ∈ Σ | |w|# ≥ 1} are clearly disjoint, we can conclude

Σk \ {w ∈ Σk | |w|# ≥ 1} =

[

(SubSeq(gc, S(Wi )) ∩ Γ∗ )

⇐⇒

i∈[q]

Γk =

[

(SubSeq(gc, S(Wi )) ∩ Γ∗ ) .

i∈[q]

Finally, Lemma F.2 implies that Γk =

F.7

S

i∈[q] L(Wi ).

Direct Reduction from CNF-Sat to NUni

Let F = {c1 , c2 , . . . , cq } be a Boolean formula in CNF on variables {v1 , v2 , . . . , vk }. Let Σ = {a, b}.
For every i ∈ [q] and j ∈ [k], we define


if vj ∈ ci ,
{a a}
Wi,j = {b b}
if ¬vj ∈ ci ,


{a a, b b} if {vj , ¬vj } ∩ ci = ∅,
By interpreting a a and b b as Boolean values false and true, we can interpret the words form
LΣΣ = L((a a ∨ b b)k ) as assignments for F ’s variables. Moreover, the set
S L(Wi ) = Wi,1 Wi,2 . . . Wi,k
represents all assignments that do not satisfy clause ci , and therefore i∈[q] L(Wi ) is the set of all
S
non-satisfying assignments. In particular, F is satisfying if and only if i∈[q] L(Wi ) 6= LΣΣ .
We define a 2k-tuple gc = (C1 , C10 , C2 , C20 , . . . , Ck ) of length constraints, where Cj = (0, 0) for
every j ∈ [k] and Cj0 = (3, 9) for every j ∈ [k − 1], i. e., every gc-sequence corresponds to k pairs of
consecutive symbols with a gap of length at least 3 and at most 9 in between.
For every i ∈ [q] and j ∈ [k], we define the string
wi = wi,1 b a b wi,2 b a b . . . b a b wi,k ,
where



if vj ∈ ci ,
a a
wi,j = a b b a
if ¬vj ∈ ci ,


a a b b a if {vj , ¬vj } ∩ ci = ∅.

Lemma F.6. (SubSeq(gc, wi ) ∩ LΣΣ ) = L(Wi ).
Proof. We first prove (SubSeq(gc, wi )∩LΣΣ ) ⊆ L(Wi ). To this end, let p ∈ (SubSeq(gc, wi )∩LΣΣ ).
This means that there is an embedding e that satisfies gc and p e wi , and that, for every j ∈ [k],
p[2j − 1]p[2j] ∈ {a a, b b}. Since all factors wi,j start and end with symbol a and the separating
factor b a b starts and ends with symbol b, for every j ∈ [k], p[2j −1]p[2j] is mapped by e completely
inside a factor wi,j . Moreover, due to the lower bounds 3 of the length constraints Cj0 for every
j ∈ [k − 1], it is not possible that both p[2j − 1]p[2j] and p[2(j + 1) − 1]p[2(j + 1)] are mapped by
e inside the same factor wi,` . Hence, for every j ∈ [k], p[2j − 1]p[2j] is mapped by e to a factor a a
or b b inside of wi,j . By construction, for every j ∈ [k], the factor wi,j contains factor a a, but not
b b, if Wi,j = {a a}, it contains factor b b, but not a a, if Wi,j = {b b}, and it contains factor a a
and factor b b, if Wi,j = {a a, b b}. Consequently, p ∈ W1 W2 . . . Wk = L(Wi ).
35

<!-- page 36 -->

Next, we prove that L(Wi ) ⊆ (SubSeq(gc, wi ) ∩ LΣΣ ). To this end, let p ∈ L(Wi ), which
means that p[2j − 1]p[2j] ∈ Wi,j for every j ∈ [k], which also means that p ∈ LΣΣ . Therefore, by
construction of wi , p[2j − 1]p[2j] is a factor of wi,j for every j ∈ [k]. Thus, there is an embedding
e that, for every j ∈ [k], maps each 2j − 1 and 2j to consecutive positions in wi,j that correspond
to a factor p[2j − 1]p[2j], which means that p e wi . In particular, e satisfies all length constraints
Cj = (0, 0) with j ∈ [k]. Finally, we note that by construction of the factors wi,j , we have
3 ≤ |gape (wi , 2j)| ≤ 8 for every j ∈ [k − 1], which means that e also satisfies the length constraints
Cj0 with j ∈ [k − 1], and therefore p ∈ (SubSeq(gc, wi ) ∩ LΣΣ ).
For every i ∈ [k], let Ti = Ti,1 b a b Ti,2 b a b . . . b a b Ti,k , where, for every j ∈ [k] \ {i}, Ti,j =
a a b b a, and Ti,i = a b a. We define the string T by
T = T1 b(a b)5 T2 b(a b)5 . . . b(a b)5 Tk .
We define LΣΣ = {a, b}2k \ LΣΣ .
Lemma F.7. SubSeq(gc, T ) = (LΣΣ ).
Proof. We first prove SubSeq(gc, T ) ⊆ (LΣΣ ). To this end, let p ∈ SubSeq(gc, T ), which means
that there is an embedding e that satisfies gc and p e T . For contradiction, we assume that
p∈
/ (LΣΣ ), which means that p ∈ LΣΣ and therefore p[2j − 1]p[2j] ∈ {a a, b b} for every j ∈ [k].
Next we observe that all factors Ti,j start and end with symbol a, while all separating factors b a b
and b(a b)5 start and end with symbol b. In particular, this means that, for every j ∈ [k], both
2j − 1 and 2j are mapped by e to positions of some Ti,j factor. Moreover, due to the upper bound
9 of length constraints Cj0 for every j ∈ [k − 1], it is also not possible that e maps positions of p to
different factors Ti and Ti+1 for some i ∈ [k], i. e., there is some r ∈ [k] such that all positions of p
are mapped to positions of Tr . As observed above, for every j ∈ [k], factor p[2j − 1]p[2j] must be
mapped to positions of some Tr,` , i. e., to a factor a a or b b in some Tr,` . Due to the lower bounds
3 of the length constraints Cj0 for every j ∈ [k − 1], it is not possible that both p[2j − 1]p[2j] and
p[2(j + 1) − 1]p[2(j + 1)] are mapped by e to the same factor Tr,` , which means that, for every
j ∈ [k], p[2j − 1]p[2j] is mapped to Tr,j . This is a contradiction, since Tr,r does not contain any
occurrence of factor a a or b b.
Next, we prove (LΣΣ ) ⊆ SubSeq(gc, T ). To this end, let p ∈ (LΣΣ ), which means that there is
some r ∈ [k] such that p[2r − 1]p[2r] ∈ {a b, b a}. Let the embedding e be defined as follows. For
every j ∈ [k] with j 6= r, we map 2j − 1 and 2j to a factor p[2j − 1]p[2j] in Tr,j (note that since
Tr,j = a a b b a and p[2j − 1]p[2j] ∈ {a a, a b, b a, b b}, Tr,j must contain the factor p[2j − 1]p[2j]).
Furthermore, we map 2r − 1 and 2r to the factor p[2r − 1]p[2r] in Tr,r (note that this must be
possible since Tr,r = a b a and p[2r − 1]p[2r] ∈ {a b, b a}). Finally, we note that by construction of
the factors Ti,j , we have 3 ≤ |gape (wi , j)| ≤ 9, which means that e satisfies the length constraints
and therefore p ∈ SubSeq(gc, T ).
Finally, we define the string
S = T b(a b)5 w1 b(a b)5 w2 b(a b)5 . . . b(a b)5 wq .
Lemma F.8. Let p ∈ (SubSeq(gc, S) ∩ LΣΣ ). Then, for some i ∈ [q], p ∈ SubSeq(gc, wi ).
Proof. We assume that p ∈ (SubSeq(gc, S) ∩ LΣΣ ), i. e., there is an embedding e that satisfies gc
and p e S. All factors wi and T start and end with occurrences of symbol a, while the separating
factors b(a b)5 in between start and end with occurrences of symbol b. This means that all positions
of p must be mapped by e to a position of some factor wi or to a position of factor T . Due to the
upper bounds 9 of the length constraints Cj0 for every j ∈ [k − 1], it is not possible that e maps
positions of p to different such factors; thus, all positions of p are mapped to positions of some
factor wi or to factor T . However, due to Lemma F.7, SubSeq(gc, T ) = (LΣΣ ), so since p ∈ LΣΣ
it is not possible that e maps p to T . Hence, p ∈ SubSeq(gc, wi ).
Lemma F.9. SubSeq(gc, S) = {a, b}2k ⇐⇒ ∪i∈[q] L(Wi ) = LΣΣ .

36

<!-- page 37 -->

Proof. We first show that
SubSeq(gc, S) =

[

(SubSeq(gc, wi ) ∩ LΣΣ ) ∪ SubSeq(gc, T ) .

i∈[q]

The “⊇”-direction holds, since T and every wi for every i ∈ [q] are factors of S. Now let p ∈
SubSeq(gc, S) be arbitrarily chosen. If p ∈
/ LΣΣ , then, by Lemma F.7, p ∈ SubSeq(gc, T ). If
p ∈ LΣΣ , then Lemma F.8 implies that p ∈ SubSeq(gc, wi ) for some i ∈ [q], which means that
p ∈ SubSeq(gc, wi ) ∩ LΣΣ . This shows that the “⊆”-direction holds as well.
Now we prove the if direction of the statement of the lemma and assume that ∪i∈[q] L(Wi ) =
LΣΣ . By Lemma F.6, we know that (SubSeq(gc, wi ) ∩ LΣΣ ) = L(Wi ) for every i ∈ [q]. With
Lemma F.7 and the observation from above, this means that
[
SubSeq(gc, S) =
(SubSeq(gc, wi ) ∩ LΣΣ ) ∪ SubSeq(gc, T )
i∈[q]


=


[

L(Wi ) ∪ SubSeq(gc, T )

i∈[q]

= LΣΣ ∪ (LΣΣ ) = {a, b}2k .
In order to prove the only if direction, we assume that SubSeq(gc, S) = {a, b}2k . With our
observation from above, this means that
[
{a, b}2k =
(SubSeq(gc, wi ) ∩ LΣΣ ) ∪ SubSeq(gc, T )
i∈[q]

S
Applying Lemma F.7 and the fact that i∈[q] (SubSeq(gc, wi ) ∩ LΣΣ ) and LΣΣ are clearly disjoint
yields
[
{a, b}2k =
(SubSeq(gc, wi ) ∩ LΣΣ ) ∪ (LΣΣ )
⇐⇒
i∈[q]

LΣΣ =

[

(SubSeq(gc, wi ) ∩ LΣΣ ) .

i∈[q]

Finally, Lemma F.6 implies that LΣΣ =

F.8

S

i∈[q] L(Wi ).

Proof of Theorem 4.2

Theorem F.10. For every fixed alphabet Σ with |Σ| ≥ 3, NUniΣ with length constraints is NPcomplete, even if all length constraints are (1, 5). Moreover,
• it cannot be solved in subexponential time 2o(k) poly(|w|, k)) (unless ETH fails),
• it cannot be solved in time O(2k(1−) poly(|w|, k)) (unless SETH fails).
Proof. We first show that NUni with length constraints is in NP. Let w ∈ Σ∗ , let k ∈ N and let gc
be some tuple of length constraints of size k − 1. In order to check whether SubSeq(gc, w) 6= Σk ,
we guess a string p ∈ Σk in polynomial time and then check whether p gc w. According to
Corollary 3.2, this can be done in polynomial time.
The NP-hardness and the conditional lower bounds follow from the reduction obtained by
plugging together the reductions described in Sections F.4 and F.6. Let F = {c1 , c2 , . . . , cq } be an
instance of SAT over some set of variables V = {v1 , v2 , . . . , vk }. We can then use the reduction from
Section F.4 in order to reduce this SAT instance into a MetaNUni instance (Wi,j )1≤i≤q,1≤j≤k
over a binary alphabet {0, 1}. After that, we use the reduction of Section F.6 in order to reduce the
MetaNUni instance (Wi,j )1≤i≤q,1≤j≤k over {0, 1} into a NUniΣ instance gc = (C1 , C2 , . . . , Ck−1 )
and w over alphabet Σ = {0, 1, #}. Moreover, Ci = (L− (j), L+ (j)) = (m − 1, 3m − 1) for every
j ∈ [k − 1], where m is the alphabet size of the MetaNUni instance, which is |{0, 1}| = 2. This
37

<!-- page 38 -->

means that all length constraints are (m − 1, 3m − 1) = (1, 5). This proves that NUniΣ with
length constraints is NP-complete, even if |Σ| = 3 and all length constraints are (1, 5). We can
furthermore note that |w| = poly(k, m, q) = poly(k + q).
In order to prove the ETH bound, assume that NUniΣ with |Σ| = 3 and only length constraints
(1, 5) can be solved in time 2o(k) poly(|w|, k)). Let F = {c1 , c2 , . . . , cq } be an instance of 3-CNFSAT over some set of variables V = {v1 , v2 , . . . , vk0 }. If we use the above reduction on F , then
we get a NUniΣ instance (gc, w) with |gc| = k 0 − 1 and |w| = poly(k 0 + q). By assumption, this
0
instance can be solved in time 2o(k ) poly(|w|, k 0 )), which means that 3-CNF-SAT can be solved in
o(k0 )
0
time 2
poly(k + q)). Hence, ETH fails.
In order to prove the SETH bound, assume that NUniΣ with |Σ| = 3 and only length constraints
(1, 5) can be solved in timeO(2k(1−) poly(|w|, k)) for some  > 0. Let F = {c1 , c2 , . . . , cq } be
an instance of CNF-SAT over some set of variables V = {v1 , v2 , . . . , vk0 }. If we use the above
reduction on F , then we get a NUniΣ instance (gc, w) with |gc| = k 0 − 1 and |w| = poly(k 0 + q).
0
By assumption, this instance can be solved in time O(2k (1−) poly(|w|, k 0 )) for some  > 0, which
0
means that CNF-SAT can be solved in time O(2k (1−) poly(k 0 + q)). Hence, SETH fails.

F.9

Proof of Theorem 4.3

Theorem F.11. For every fixed alphabet Σ with |Σ| = 2, NUniΣ with length constraints is NPcomplete, even if each length constraint is (0, 0) or (3, 9).
Proof. This follows directly from the reduction described in Sections F.7.

F.10

Proof of Theorem 4.4

Theorem F.12. The problem NUni with length constraints cannot be solved in running time
O(f (k) poly(|w|, k)) for any computable function f (unless FPT = W[1]).
Proof. We observe that the reduction that results from plugging together the reductions described
in Sections F.5 and F.6) is a parameterised reduction from k-Independent Set to NUni with
length constraints parameterised by k. Indeed, the reduction from Section F.5 transforms an
instance (G = (V, E), k) of k-Independent Set into an MetaNUni instance over an alphabet Γ
of cardinality |V | and with a matrix of dimensions |V |k(k − 1) and k. Then, the reduction from
Section F.6 transforms this MetaNUni instance into an instance of NUni with length constraints,
where parameter k corresponds to parameter k of the original k-Independent Set instance.
Consequently, the reduction is a parameterised reduction from k-Independent Set to NUni with
length constraints parameterised by k. Therefore, NUni with length constraints parameterised by
k is W[1], which yields the statement of the theorem.

F.11

Containment and Equivalence

We now show that the statements of Theorems 4.3, 4.2, and 4.4 also hold for the problems NCon
and NEqu.
We recall the definition of the con-containment (NCon) and the non-equivalence problem for
gap constrained subsequences (NEqu): Given a (k − 1)-tuple of gap constraints gc and strings
w, w0 , decide SubSeq(gc, w) 6⊆ SubSeq(gc, w0 ), or SubSeq(gc, w) 6= SubSeq(gc, w0 ), respectively.
We now extend the reduction from Section F.6 to a reduction from MetaNUni to NEqu. Let
Γ = {b1 , b2 , . . . , bm }, q, k ∈ N, and, for every i ∈ [q], j ∈ [k], let Wi,j ⊆ Γ. Moreover, let gc be the
tuple of length constraints and let K(W1 , . . . , Wq ) be the string over Σ = Γ ∪ {#} constructed by
the reduction from Section F.6.
We now define a string that is universal for gc. For every i ∈ [k], let T 0 = T10 T20 . . . Tk0 , where,
for every j ∈ [k], Tj0 = b1 b2 . . . bm #m .
Lemma F.13. SubSeq(gc, T 0 ) = Σk .
Proof. Since T 0 is a string over Σ, we have SubSeq(gc, T 0 ) ⊆ Σk . Now let p ∈ Σk be arbitrarily
chosen, and let e be the embedding that is defined as follows. For every j ∈ [k], we map j to the
tth position of Tj0 if p[j] = bt , and to the (m + 1)st position of Tj0 if p[j] = #. We observe that this
38

<!-- page 39 -->

embedding e satisfies that every j is mapped to an occurrence of π[j] of Tj0 . Moreover, since every
j ∈ [k] is mapped to one of the first m + 1 occurrences of the length-2m factor Tj0 , it can be easily
seen that m − 1 ≤ |gape (T 0 , j)| ≤ 3m − 1. Hence, e satisfies the length constraints, and therefore
p ∈ SubSeq(gc, T 0 ). This means that Σk ⊆ SubSeq(gc, T 0 ).
Lemma F.14. SubSeq(gc, K(W1 , . . . , Wq )) = SubSeq(gc, T 0 ) ⇐⇒ ∪i∈[q] L(Wi ) = Γk .
Proof.
SubSeq(gc, K(W1 , . . . , Wq )) = SubSeq(gc, T 0 )

Lem. F.13

SubSeq(gc, K(W1 , . . . , Wq )) = Σk

Lem. F.5

⇐⇒

⇐⇒

k

∪i∈[q] L(Wi ) = Γ .

Theorem F.15. For every fixed alphabet Σ with |Σ| ≥ 3, NEquΣ and NConΣ with length constraints are NP-complete, even if all length constraints are (1, 5). Moreover,
• it cannot be solved in subexponential time 2o(k) poly(|w|, |w0 |, k)) (unless ETH fails),
• it cannot be solved in time O(2k(1−) poly(|w|, |w0 |, k)) (unless SETH fails).
Proof. This result follows analogously as in the proof of Theorem F.10, i. e., by the reduction
obtained by plugging together the reductions described in Sections F.4 and the extension of the
reduction of Section F.6 described above. That this reduction satisfies that |Σ| ≥ 3 and that all
length constraints are (1, 5) follows in exactly the same way as in the proof of Theorem F.10.
The conditional lower bounds also follow analogously as in the proof of Theorem F.10. We only
have to observe that the universal string T 0 is polynomial in k, and that the adapted reduction
still produces a NEquΣ with |gc| = k − 1, where k is the number of Boolean variables.
Theorem F.16. The problems NEqu and NCon with length constraints cannot be solved in
running time O(f (k) poly(|w|, k)) for any computable function f (unless FPT = W[1]).
Proof. Analogously to the proof of Theorem F.12, we observe that plugging together the reduction
described in Sections F.5 and the extension of the reduction from Section F.6) described above
yields a parameterised reduction from k-Independent Set to NEqu with length constraints
parameterised by k.
Next, we extend in a similar way the reduction from Section F.7 to a reduction from CNF-Sat
to NEqu. Let F = {c1 , c2 , . . . , cq } be a Boolean formula in CNF on variables {v1 , v2 , . . . , vk }.
Moreover, let gc be the tuple of length constraints and let S be the string over Σ = {a, b} constructed by the reduction from Section F.7. We now prove that the string T 0 = ((a a b b a) b3 )k is
universal for gc.
Lemma F.17. SubSeq(p, T 0 ) = {a, b}2k .
Proof. We first note that SubSeq(p, T 0 ) ⊆ {a, b}2k obviously holds. In order to show the other
direction, let p ∈ {a, b}2k . We define an embedding e as follows. For every j ∈ [k], e maps
2j − 1 and 2j to the factor p[2j − 1]p[2j] in the j th occurrence of factor a a b b a. We observe
that 3 ≤ gape (T 0 , j) ≤ 9, which means that e satisfies the length constraints, and therefore p ∈
SubSeq(p, T 0 ).
Lemma F.18. SubSeq(gc, S) = SubSeq(gc, T 0 ) ⇐⇒ ∪i∈[q] L(Wi ) = LΣΣ .
Proof.
SubSeq(gc, S) = SubSeq(gc, T 0 )

Lem. F.17

SubSeq(gc, S) = {a, b}2k

Lem. F.9

⇐⇒

⇐⇒

∪i∈[q] L(Wi ) = LΣΣ .
39

<!-- page 40 -->

Theorem F.19. For every fixed alphabet Σ with |Σ| = 2, NEquΣ and NConΣ with length constraints is NP-complete, even if each length constraint is (0, 0) or (3, 9).
Proof. This follows directly from the extension of the reduction from Sections F.7 described above.

G

Special Variants

We next consider two natural variants of our setting that have a substantial impact on the complexity of the problems investigated above.
Gap Length Equalities: We investigate whether the polynomiality of the matching problem
(see Section 3) is preserved under adding gap length equalities to the gap constraints, i. e., constraints of the form |gapi | = |gapj | which are satisfied by an embedding e with respect to w if
|gape (w, i)| = |gape (w, j)|. Our main motivation is that such length equality constraints (and more
complex ones, e. g., described by linear inequalities like 2|gap7 | + |gap3 | ≤ |gap2 |) are of interest
in the theory of string solving [3]. Unfortunately, the matching problem becomes immediately
NP-hard (the following result can be shown by adapting the NP-completeness proof for matching
patterns with variables from [4]).
Theorem G.1. Match with length constraints and gap length equalities is NP-complete, even for
binary alphabets and length constraints (0, +∞).
Proof. We first define the investigated problem variant more formally. The matching problem with
length constraints and gap length equalities is defined as follows: Given a gapped sequence (p, gc)
with length constraints, a word w, and a finite set of equations S of the form |gapi | = |gapj | with
i, j ∈ [|gc| − 1], decide whether there exists an embedding e satisfying gc such that p e w and
|gape (w, i)| = |gape (w, j)| for each equation |gapi | = |gapj | of S.
We can now continue with the proof of the theorem. The containment in NP is immediate. We
only show the lower bound.
To simplify the exposition, just as in the case of Theorem 3.3, when representing the gapped
sequence (p, gc) with p = p[1] · · · p[m] and a finite set of equations S of the form |gapi | = |gapj |, we
will use the following notations. Firstly, (p, gc) = p[1]

gc[1],`1

↔

p[2]

gc[2],`2

↔

···

gc[m−1],`n

↔

p[m], where

gc[i]

`1 , . . . , `n are symbols from a finite set X of labels. We omit the symbol ↔ from the notation of
(p, gc) if and only if gc[i] = (0, 0). In this proof, however, all length constraints are trivial, i. e.,
gc[i]

they have either the form gc[i] = (0, +∞); in this case, we simply write ↔ for ↔ . In the context
of this proof, and differently from the proof of Theorem 3.3, the gaps will have labels. We use
these labels to encode the equations in the set S: the ith gap and j th gap have the same label if
gc[i]

gc[i],x

and only if |gapi | = |gapj | is a gap length equality of S. When, ↔ has label x we write ↔ ; as
x
such, in this proof we only need to keep track of the labels of the gaps, and denote a gap as ↔,
where x is the label.
We reduce 3-CNF-SAT to the extended matching problem. Let Φ = C1 ∧ C2 ∧ . . . ∧ Cm be a
Boolean formula in CNF with variables V1 , V2 , . . . , Vn , where each clause is a conjunction of three
literals, each of which is either a variable or the negation of a variable. Without loss of generality
we choose two symbols 0, 1 ∈ Σ. Now, we construct from Φ a gapped sequence (p, gc) with length
constraints, a string s and a set of equations S as follows.
We first define a set of 4n + 2m labels: xi , x0i , yi , yi0 , zj , zj0 , where 1 ≤ i ≤ n and 1 ≤ j ≤ m. We
define for j ∈([m] and k ∈ [3]
xi
if the kth literal in Cj is Vi ,
f (j, k) =
yi
if the kth literal in Cj is Vi .
We first define the gapped sequence (p0 , gc0 ) as

! n
! m
n
0
0
0
Y
Y
Y
zj
x
y
zj
yi
x
(p0 , gc0 ) =
0 ↔i 1 ↔i
0 ↔ 1 ↔i 
0 ↔ 1 ↔
i=1

i=1

40

j=1

<!-- page 41 -->

Now, we define the gapped sequence (p00 , gc00 ) as
(p00 , gc00 ) = 0p1 0p2 · · · 0pn 0q1 0q2 · · · 0qm 0
x

yi

f (j,1)

f (j,2)

f (j,3)

zj

with pi =↔i 1 ↔ and qj = ↔ 1 ↔ 1 ↔ 1 ↔. for 1 ≤ i ≤ n and 1 ≤ j ≤ m.
Let (p, gc) = (p0 , gc0 )(p00 , gc00 ).
The system S is induced by the labels of the gaps: two gaps have the same label if and only if
they have the same length.
We now define the string w0 as

! n
! m
n
Y
Y
Y
w0 =
0di
0ei 
0fj  ,
i=1

i=1

j=1

where di = ei = 111 = 13 , and fj = 1111 = 14 , for 1 ≤ i ≤ n and 1 ≤ j ≤ m.
Finally, the string w00 is constructed as
w00 = 0s1 0s2 · · · 0sn 0t1 0t2 · · · 0tm 0
with si = 1111 = 14 and tj = 1111111111 = 110 , for 1 ≤ i ≤ n and 1 ≤ j ≤ m.
Let w = w0 w00 .
It is easy to see that the construction of gapped sequence (p, gc), w, and equation system S
can be done in polynomial time. We now show that there exists an embedding e satisfying gc such
that p e w and |gape (w, i)| = |gape (w, j)| for each equation |gapi | = |gapj | of S (or in other words,
if the respective gaps have the same label) if and only if the formula Φ is satisfiable.
Firstly, assume that Φ is satisfiable. Let α : {V1 , V2 , . . . , Vn } → {0, 1} be an assignment satisfying Φ.
We define the following mapping which defines an embedding of p into w.
First of all, the ith symbol 0 of p will be mapped to the ith symbol 0 of the word w.
We then explain how we map the symbols of p0 found between occurrences of 0. The factor
x

x0

yi

y0

↔i 1 ↔i is mapped to the factor di = 13 of w, and the factor ↔ 1 ↔i is mapped to the factor ei of
w. If α(Vi ) = 1, then let the length of the gap with the label xi in the embedding we construct
be 2 and the length of the gap with label yi be 1. Clearly, in this case, the gap labelled x0i will
have length 0, and the gap with label yi0 will have length 1. If α(Vi ) = 0, then let the length of
the gap with the label xi be exactly 1 and the length of the gap with label yi be exactly 2. The
gap labelled x0i will then have length 1, and the gap with label yi0 will have length 0. The way we
assign the length of the gaps labelled with zj will follow from the explanations below.
Now, we see how to map the symbols of p00 found between 0-symbols.
For 1 ≤ i ≤ n, each gapped sequence pi is mapped in the string si = 14 of w. From the way
the symbols of p0 are mapped to symbols of w, the mapping of pi to si is already determined: the
total length of the gaps labelled with xi and yi is 3, which of these gaps has length 2 and which
has length 1 is determined by the value of α(Vi ), and the exact mapping of the single 1-symbol of
pi to a 1-symbol of w is, as such, also determined by α(Vi ).
Further, we see how the gapped sequences qj are embedded in w. These gapped sequences are
embedded in the factors tj = 110 of w. So let us consider some j ∈ [m], and let Cj = (t1 ∨ t2 ∨ t3 )
be the corresponding clause with the three literals t1 , t2 , t3 . Since α satisfies Φ, at least one of
the three literals t1 , t2 , t3 is assigned the value 1. Therefore, at least one of the gaps with labels
f (j, 1), f (j, 2), f (j, 3) should have been assigned the length 2 in the previous steps (where the length
of each of the gaps with labels xi or yi was decided). This means that, altogether, the length of the
gaps with labels f (j, 1), f (j, 2), f (j, 3) from qj , together with the 1-symbols occurring after each of
these gaps (so three 1 symbols in total), is either 7, 8, or 9. To complement, the length of the gap
with the label zj is either 3, 2, or 1, respectively. The embedding maps the 1-symbols occurring in
qj according to the length of the gaps, which once more is essentially determined by the values of
the boolean-variables occurring in Φ. Moreover, the length we assign here to the gap with the label
zj induces the length of the gaps labelled with zj0 (and the position of w to which the 1 symbol
zj

zj0

from ↔ 1 ↔ is mapped in fj ).
41

<!-- page 42 -->

So, if Φ is satisfiable, then an embedding of p in w which satisfies both the constraints gc and
the system S (as induced by the labels of the gaps) is possible.
Now suppose that we have an embedding e such that p e w and e satisfies both the gap
constraints gc and the equations of S (induced by the labels in the definition of (p, gc)). We want
to show that there exists an assignment of the variables {V1 , . . . , Vn } which satisfies Φ.
It is not hard to note that, due to the fact that w and p have exactly the same number 0
symbols, the embedding e maps the ith 0-symbol of p to the ith 0-symbol of the word w.
x

x0

Therefore, for all i ∈ [n], ↔i 1 ↔i must be mapped to di = 13 . This means that the gaps labelled
yi

y0

with xi may have length 0, 1 or 2. Similarly, ↔ 1 ↔i must be mapped to ei = 13 , so the gaps
z

z0

labelled with yi may have length 0, 1 or 2, for i ∈ [n]. Finally, ↔i 1 ↔i must be mapped to fi = 14 .
This means that the gaps labelled with zi may have length 0, 1, 2 or 3.
Also, we have that pi must be embedded in si , for i ∈ [n], and qj must be embedded in tj , for
j ∈ [m].
From the fact that pi is embedded in si , for i ∈ [n], we essentially get an assignment `xi for
the length of the gaps with label xi and an assignment `yi for the length of the gaps with label
yi , such that `xi + `yi = 3. We have already seen that `xi ≤ 2 and `yi ≤ 2. So, `xi ∈ {1, 2} and
`yi = 3 − `xi . If `xi = 2 then we set Vi = 1 and, otherwise, we set Vi = 0.
Further, we have that qj is embedded in tj , and we already have an assignment for the lengths
`xi and `yi , for all i ∈ [n]. In order for qj to be embedded in tj = 110 , as the length `zj of the gap
with label zj is at most 3, we obtain that the gapped sequence
`

(1,2),f (j,1)

↔

1

(1,2),f (j,2)

mapped to a string 1 with ` ≥ 7. This means that at least one of the gaps

↔

1

(1,2),f (j,3)

↔

1 is

(1,2),f (j,1) (1,2),f (j,2)

↔

,

↔

, or

(1,2),f (j,3)

↔
is mapped to a string of length 2. As these gaps have labels from the set {xi , yi | i ∈ [n]},
corresponding to the variables occurring in Cj , it follows easily that at least one of the literals that
appear in Cj should be set to 1. As such, the disjunction Cj of Φ evaluates to 1.
Since the previous remark holds for all j, it follows that the assignment of the variables induced
by the choice of the lengths `xi and `yi , for all i ∈ [n], satisfies Φ.
In conclusion, there exists an embedding e satisfying gc such that p e w and |gape (w, i)| =
|gape (w, j)| for each equation |gapi | = |gapj | of S (as induced by the labels of the gaps) if and only
if the formula Φ is satisfiable. This shows that our reduction is correct, and, thus, the statement
follows.
Gap Constrained Subsequences With Multiplicities: With respect to the equivalence
problem, we can show a positive result for the following modified setting. Let us consider the
gc-subsequences of the sets SubSeq(gc, w) with multiplicities. For example, for w1 = a b b a and
w2 = a b a b, we have SubSeq(gc2 , w1 ) = SubSeq(gc2 , w2 ) = {a a, a b, b a, b b} with gc2 = (Σ∗ ).
There is exactly one way of embedding a a and b b into both w1 and w2 . On the other hand, a b
can be embedded into w1 in two different ways and into w2 in three different ways. More precisely,
the sets of gc2 -subsequences of w1 and w2 with multiplicities are {(a a, 1), (a b, 2), (b a, 2), (b b), 1}
and {(a a, 1), (a b, 3), (b a, 1), (b b), 1}, respectively.
Let us now formalise this setting. For strings u and v, and a (|u|−1)-tuple gc of gap constraints,
we denote by |u|v,gc the number of distinct embeddings e : |u| → |v| that satisfy gc and v e u. For
example, |b b a a|b a,gc2 = 4, as u[1]u[3] = u[2]u[3] = u[1]u[4] = u[2]u[4] = b a. For any (k − 1)-tuple
k
gc of gap constraints, we define the function Ψgc (·) : Σ∗ → N(Σ ) by Ψgc (w)[p] = |w|p,gc for every
p ∈ Σk .
The equivalence problem with multiplicities is to decide, for a given (k − 1)-tuple gc of gap
constraints, and strings w, w0 ∈ Σ∗ , whether Ψgc (w) = Ψgc (w0 ). Note that for the case gc =
(Σ∗ , . . . , Σ∗ ) this is called the k-binomial equivalence, and was studied in the area of combinatorics
on words (see, e. g., [65, 56, 57, 30]).
We show that equivalence with multiplicities can be decided in polynomial time (in contrast to
the NP-completeness of the case without multiplicities).
Theorem G.2. If gc = (C1 , . . . , Ck−1 ) and Ci can be decided in polynomial time then the equivalence problem with multiplicities can be solved in polynomial time.
Proof. We adapt the main idea from [30] and implement the following approach.
42

<!-- page 43 -->

We first define an algorithm constructing, for a word w with |w| = n and a (k − 1)-tuple gc
of gap constraints, a non-deterministic finite automaton Aw,gc that accepts exactly the gapped
subsequences p ∈ SubSeq(gc, w). Moreover, this automaton has exactly Ψgc (w)[p] accepting paths
labelled with the subsequence p of w.
Then, we will use this algorithm for the input words of the equivalence with multiplicities
problem and obtain two automata Aw,gc and Aw0 ,gc .
Finally, we use the algorithm of [76] to test whether Aw,gc and Aw0 ,gc are path equivalent, i. e.,
for each word p, the number of accepting paths of Aw,gc labelled with p equals the number of
accepting paths of Aw0 ,gc labelled with p. If this algorithm returns a positive answer, then we can
conclude that Ψgc (w) = Ψgc (w0 ). Otherwise, we conclude that Ψgc (w) 6= Ψgc (w0 ).
It is not hard to see that this approach is correct, as soon as we explain further how to
implement the first step. Thus, we will now describe the construction of the automaton Aw,gc , and
then compute the overall complexity of our algorithm.
Some notations first. Assume gc = (C1 , . . . , Ck−1 ), where, for every i ∈ [k − 1], the constraint
Ci is given as a (black box) procedure check(i, u), which checks u ∈ Ci in time O(P (|u|)) for some
polynomial P .
The set of states is defined by
Qw = {(0, 0)} ∪ {(i, j) | 1 ≤ i ≤ n, 1 ≤ j ≤ k} ∪ {(n + 1, k + 1)}
The state (0, 0) is the initial state, and (n + 1, k + 1) represents an error state. The states (i, j)
with j = k and i ≥ j are final. As an intuition, i represents the position in the word w, and j
represents the position in the subsequence p.
The transitions of the automaton will be defined to reflect the gap constraints:

0
0
0

{(i , j + 1) ∈ Qw | i > i, w[i ] = a,
δw ((i, j), a) =
w[i + 1..i0 − 1] ∈ Cj }


{(n + 1, k + 1)}

if this set is non-empty,
otherwise.

In both cases the automata are accepting a string p ∈ Σk if and only if there is an embedding e satisfying the constraints with p e w. The automaton Aw is accepting the words
w[i1 ], w[i2 ], . . . , w[ik0 ] with i1 < i2 < . . . < ik while satisfying the constraints with w[ij + 1..ij+1 −
1] ∈ Cj for 1 ≤ j ≤ k − 1. For this, we start in the initial state and follow the path of states
(0, 0), (i1 , 1), (i2 , 2), . . . , (ik , k)
with 1 ≤ i1 < i2 < . . . < ik and ik ≥ |w|, so the state (ik0 , k 0 ) is indeed an accepting one.
Additionally, for a word that is accepted by the automaton Aw through the path
(0, 0), (i1 , 1), (i2 , 2), . . . , (ik , k),
we get by definition that ij < ij+1 for all 1 ≤ j ≤ k − 1, and that 0 < i1 . Each transition ending
in state (ij , j) is labelled with w[ij ] and we have that w[ij + 1..ij+1 − 1] ∈ Cj .
Combined, we immediately get that the automaton accepts exactly the gapped subsequences
of length k satisfying the given constraints, and the number of distinct paths labelled with with
the particular gapped subsequence equals the number of its occurrences in w.
This concludes the description of the automaton Aw,gc . To completely prove the statement,
we need to evaluate the overall complexity of the algorithm. For that, some more implementation
details are needed.
Let N be the length of the longest of the input strings w and w0 .
For each j ∈ [k − 1] and u ∈ {w, w0 }, we can first identify (and store in a three dimensional
array Mu ) in O(N 2 P (N )) time all the factors of u which are part of Cj (that is, Mu [a][b][j] = 1 if
and only if w[a..b] ∈ Cj ). Then, the construction of the automaton can be done in O(N 2 k) time
(as we can also assume |Σ| ≤ N ).
Now, to decide if the automata Aw,gc and Aw0 ,gc are path equivalent, we use the algorithm
described in [76]. The runtime of this part of our algorithm is O((N k)4 ).
Overall, our algorithm runs in O(N 2 kP (N ) + N 4 k 4 ).
43

<!-- page 44 -->

In the case of length, regular or reg-len constraints, the equivalence problem with multiplicities
can be solved in O(max{|w|, |w0 |}4 k 4 + size(gc)) time. The containment problem with multiplicities
(i. e., deciding Ψgc (w)[p] ≤ Ψgc (w0 )[p] for all p ∈ Σk ) seems to be more difficult. To our knowledge, whether the case of classical subsequences (i. e., length constraints (0, ∞)) can be solved in
polynomial time is open. On the other hand, for the case of length constraints (0, 0) only (i. e.,
consecutive factors), or of length constraints (`, `) only (i. e., partial words), showing polynomial
time solvability is relatively simple.

44
