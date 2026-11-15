---
file: ../pdfs/2026-arxiv-nogami-hardness-regex-extensions.pdf
citation: "Hardness of Regular Expression Matching with Extensions — Nogami, Nakamura, Terauchi — arXiv 2601.03020, 2026"
doi: 
source: "https://arxiv.org/pdf/2601.03020"
topics: [counting_and_derivatives, semantics_theory_access]
pages: 33
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

arXiv:2601.03020v4 [cs.CC] 6 Sep 2026

Hardness of Regular Expression Matching with Extensions
Taisei Nogami
Waseda University

Yoshiki Nakamura
Chiba University

Tachio Terauchi
Waseda University

sora410@fuji.waseda.jp

nakamura.yoshiki.ny@gmail.com

terauchi@waseda.jp

Abstract
The regular expression matching problem asks whether a given regular expression of length m
matches a given string of length n, and its time complexity is fairly well understood. Meanwhile,
regular expressions have been extended to support various extensions for both theoretical and
practical reasons, which can substantially change the time complexity of the matching problem.
In this work, we consider four well-known extensions to regular expressions called intersection,
squaring, complement and backreference, and we prove a number of novel conditional time
complexity lower bounds for regular expression matching with these extensions, assuming the
Orthogonal Vectors Conjecture (OVC), the k-Orthogonal Vectors Conjecture (k-OVC) or the
k-Clique hypotheses:
• For each of intersection and squaring (for squaring, even without Kleene star), no algorithm
runs in n2−ε poly(m) time under moderate-dimension OVC, n2−ε 2o(m) time under logdimension OVC or nω−ε 2o(m) time (2 ≤ ω < 2.3714 is the exponent of square matrix
multiplication) under the k-Clique Hypothesis, and no combinatorial algorithm runs in
n3−ε 2o(m) time under the Combinatorial k-Clique Hypothesis, for any constant ε > 0.
• For complement (even without Kleene star), no algorithm runs in n2−ε tower(o(m/ log m))
time under moderate-dimension OVC or nω−ε tower(o(m/ log m)) time under the k-Clique
Hypothesis, and no combinatorial algorithm runs in n3−ε tower(o(m/ log m)) time under
the Combinatorial k-Clique Hypothesis, for any constant ε > 0. Furthermore, we resolve
the inconsistency between our results and the O(n2 · (log n + m) · 2m )-time combinatorial
algorithm of Rosu (FoSSaCS’07) by identifying an issue with their algorithm.
• For backreference, no algorithm for expressions with at least k capturing
groups runs in
√
n2k−ε poly(m) time under moderate-dimension 2k-OVC or n2k−ε 2o( m) time under logdimension 2k-OVC, for any constants ε > 0 and k ≥ 1. Moreover, for expressions of a
straight-line form with a single capturing √
group, no algorithm runs in n2−ε poly(m) time
2−ε o( m)
under moderate-dimension OVC or n 2
time under log-dimension OVC.
Our hardness results are tight in the following sense. They show that for intersection,
squaring, complement and a certain subclass of backreference, the known upper bounds for
the problems are (conditionally) optimal in that the dependence on n cannot be reduced by
any polynomial factor while keeping the dependence on m polynomial (or even elementary, for
complement). Notably, for intersection and complement, our results show that Hopcroft and
Ullman’s approach from 1979 was already optimal in this sense, and shed light on why the
theoretical computer science community has struggled to improve the time complexity of these
problems with respect to n and m for more than 45 years.
Additionally, to obtain our results for intersection, squaring and complement in a unified
manner, we introduce novel problems called Dyck Selection, Orthogonal Dyck Selection and
Generalized Dyck Selection. They are arguably natural formal language and stringology problems that are based on Greibach’s hardest context-free language, a fundamental concept in
formal language theory, and capture the essence of why the regular-expression matching problems with these extensions are hard. Also, as a byproduct, this approach establishes that the
matching problem with squaring is LOGCFL-complete.

<!-- page 2 -->

1

Introduction

Regular expressions, originally introduced by Kleene [Kle56], are one of the most successful concepts
in computer science, both in theory and practice. Among problems related to regular expressions,
the regular expression matching problem is of foundational importance. It asks whether a given
regular expression r of length m matches a given string w of length n. As is well known, the problem
can be solved in O(nm) time using an algorithm introduced by Thompson [Tho68]. Roughly, the
algorithm converts a given regular expression r to an equivalent nondeterministic finite automaton
(NFA) and simulates it on a given string w. On the hardness side, Backurs and Indyk showed that
the problem cannot be solved in O((nm)1−ε ) time for any ε > 0 under the Strong Exponential Time
Hypothesis (SETH) [BI16], followed by further developments [BGL17, AB18, Sch20]. Therefore,
the time complexity of regular expression matching is fairly well understood.
Meanwhile, regular expression matching becomes much harder when extensions are present.
Prominent examples include intersection r1 ∩ r2 and complement ¬r, natural extensions from a
set-theoretic point of view, which allow taking intersections and complements of expressions. Note
that complement is at least as expressive as intersection because intersection can be expressed
using complement and alternation with only a linear increase in the length of the expression. In
1979, Hopcroft and Ullman showed in their textbook [HU79, Exercise 3.23] that regular expression
matching extended with complement can be solved in O(n3 m) time. For many years thereafter,
no polynomial improvement over this with respect to n and m had been obtained until the work
of Bille, Gørtz and Jessen [BGJ25]. [BGJ25] made an important observation that Hopcroft and
Ullman’s algorithm can run in O(nω m) time using fast matrix multiplication without significantly
altering the algorithm’s flow, where 2 ≤ ω < 2.3714 [ADV+ 25] is the exponent of square matrix
multiplication. In addition, it follows from [BGJ25] that the algorithm can be extended without
changing its running time to handle another extension called squaring r{2} [MS72], which denotes
the concatenation of r with itself (without doubling the length of r).
Another well-known extension is backreference, an extension that allows reusing previously
matched substrings. The extension is practically popular and supported by the regular expression
engines in the standard libraries of many popular programming languages including Java, Python,
JavaScript and more. The matching problem extended with backreference is NP-hard [Aho90], and
the problem for expressions restricted to at most k capturing groups can be solved in O(n2k+1 m)
time [Sch16, Sch24, NT25]. Recently, Uezato showed an O(n2 m2 )-time matching algorithm for
expressions of a straight-line form with a single capturing group [Uez26].
In light of the above, we pose the following question: How tight are these upper bounds? Notably,
recent studies have shown a surprising fact that the matching problem extended with lookaround,
a practically popular extension often used to mimic intersection and complement, can actually be
solved in O(nm) time [MC24, BP24], and therefore, we have tight bounds for regular expression
matching without extensions as mentioned in the first paragraph as well as for that extended with
lookaround. However, tight bounds have not yet been established for the extensions mentioned
above. Namely, for intersection or complement, the strongest negative result derivable from the
known results is that neither extension admits an O((n+m)ω−ε )-time algorithm nor a combinatorial
O((n+m)3−ε )-time algorithm under the k-Clique hypotheses, as will be discussed later in Remark 4
of Section 1.1.2. As for backreference, in a recent independent work, Kumabe and Uezato [KU26]
showed that matching with at least k capturing groups admits no O(n2k−ε )-time algorithm under
(moderate-dimension) 2k-Orthogonal Vectors Conjecture for every constant k ≥ 1, but by using a
non-standard form of backreference that is unsupported in most actual regular expression engines;
see Section 1.1.3 for details.

1

<!-- page 3 -->

1.1

Our Results

In this paper, we consider the four extensions described above, intersection, squaring, complement
and backreference, and establish novel conditional lower bounds for regular expression matching
with each extension assuming the Orthogonal Vectors Conjecture (OVC), the k-Orthogonal Vectors
Conjecture (k-OVC) or the k-Clique hypotheses. Our results significantly close the gaps between
the bounds mentioned above, with some of our lower bounds even being (conditionally) optimal.
Note that our lower bounds under OVC or k-OVC also hold under SETH [IP01] because SETH
implies OVC and k-OVC [Wil05, WY14, BCFK22].
1.1.1

Hardness of Matching with Intersection and Squaring

We refer to regular expressions extended with intersection, complement and squaring as semiextended regular expressions (semi-EREs), extended regular expressions (EREs) and RSQs, respectively. Star-free means containing no Kleene star.
Theorem 1. Let ε > 0 be an arbitrary constant. For semi-ERE matching and RSQ matching
(even for star-free RSQ matching), the following hold:
(a) Unless moderate-dimension OVC fails, no algorithm runs in n2−ε poly(m) time (with logdimension OVC, n2−ε 2o(m) time).
(b) Unless the k-Clique Hypothesis fails, no algorithm runs in nω−ε 2o(m) time. Moreover, unless
the Combinatorial k-Clique Hypothesis fails, no combinatorial algorithm runs in n3−ε 2o(m)
time.
Theorem 1(b) implies that the O(n3 m)-time combinatorial ERE matching algorithm of Hopcroft
and Ullman [HU79] is (conditionally) optimal among combinatorial semi-ERE matching algorithms in the sense that, assuming the Combinatorial k-Clique Hypothesis, the exponent of n
cannot be shaved by any constant while preserving the factor from m to be subexponential. Similarly, the O(nω m)-time algorithm using fast matrix multiplication observed by Bille, Gørtz and
Jessen [BGJ25] is optimal in this sense assuming the k-Clique Hypothesis. We note that if matrix
multiplication can be done in quadratic time (i.e., ω = 2), Theorem 1(a) also establishes such
optimality of the algorithm using fast matrix multiplication under log-dimension OVC.
Moreover, Theorem 1 formally captures the threshold in the trade-off between n and m because
the exponent of n drops sharply to 1 once an exponential dependence on m is allowed. Indeed,
by a standard construction, a regular expression extended with either intersection or squaring (or
both) can be converted into an equivalent NFA of 2O(m) states. Combining this with Thompson’s
algorithm mentioned earlier gives an n2O(m) -time matching algorithm for these expressions. Thus,
Theorem 1 gives a tight lower bound in that it establishes the conditionally optimal exponents of
n in running times of the form nc · g(m) for both semi-ERE matching and RSQ matching based on
whether g is exponential or not.
Additionally, we complement the results of Petersen [Pet02], who showed that semi-ERE matching is LOGCFL-hard and regular expression matching extended with both intersection and squaring is in LOGCFL, by showing that star-free RSQ matching is LOGCFL-hard. Hence, semi-ERE
matching and (star-free) RSQ matching are both LOGCFL-complete. We note that (star-free)
ERE matching is known to be P-complete [Pet00].
Theorem 2. Star-free RSQ matching is LOGCFL-hard.
Taken together, these results show that semi-ERE matching and RSQ matching have very
similar computational complexity.
2

<!-- page 4 -->

1.1.2

Hardness of Matching with Complement

As mentioned earlier, EREs subsume semi-EREs. Hence, the conditional lower bounds in Theorem 1
also apply to ERE matching. However, we can establish much stronger conditional lower bounds
for ERE matching in terms of the dependence on m. Let

2
··
·
m times ,
tower(m) = 22
i.e., the exponential tower function defined by tower(0) = 1 and tower(m) = 2tower(m−1) .
Theorem 3. Let ε > 0 be an arbitrary constant. For ERE matching (even for star-free ERE
matching), the following hold:
(a) Unless moderate-dimension OVC fails, no algorithm runs in n2−ε tower(o(m/ log m)) time.
(b) Unless the k-Clique Hypothesis fails, no algorithm runs in nω−ε tower(o(m/ log m)) time.
Moreover, unless the Combinatorial k-Clique Hypothesis fails, no combinatorial algorithm
runs in n3−ε tower(o(m/ log m)) time.
Theorem 3 significantly strengthens Theorem 1 for ERE matching. It follows from Theorem 3(b)
that, assuming the k-Clique hypotheses, any nω−ε g(m) algorithm or n3−ε g(m) combinatorial algorithm for ERE matching necessarily requires g to be nonelementary (i.e., not bounded by any fixedheight exponential tower). In this sense, as with semi-ERE matching, the approach of Hopcroft
and Ullman was already (conditionally) optimal assuming the Combinatorial k-Clique Hypothesis
and the improvement by [BGJ25] using fast matrix multiplication is optimal in this sense assuming
the k-Clique Hypothesis. Together with our result for the semi-ERE case (i.e., Theorem 1), this
sheds light on the mystery of why Hopcroft and Ullman’s approach from 1979 has stood the test
of time for more than 45 years.
Similar to the cases of semi-ERE and RSQ, if we allow the running time to be nonelementary
in m, ERE matching can be solved in linear time in n by converting a given ERE to an equivalent
NFA of tower(O(m)) states and running Thompson’s algorithm.1 Thus, Theorem 3(b) gives a tight
lower bound for the problem in that it establishes the conditionally optimal exponents of n in the
running times of the form nc · g(m) for ERE matching based on whether g is elementary or not.
Many previous studies claimed to have proposed an O(n2 m)-time ERE or semi-ERE matching
algorithm [Hir89, Yam00, KZ02, ISY03], which would be a significant improvement over Hopcroft
and Ullman’s algorithm. However, all of these claims were later found to be incorrect; see [Ros05,
Ros07] for more details. In fact, such an upper bound contradicts our lower bound from Theorem 1
unless the k-Clique Hypothesis fails or ω = 2. Moreover, Rosu claimed to have proposed a combinatorial ERE matching algorithm that runs in O(n2 · (log n + m) · 2m ) time [Ros07]. However,
this bound contradicts our n3−ε tower(o(m/ log m)) lower bound from Theorem 3 and therefore
one of the two must be incorrect (or the Combinatorial k-Clique Hypothesis is false). We resolve
this inconsistency by identifying an issue with their algorithm. Specifically, we show that the data
structure assumed by the algorithm does not exist in general by constructing a counterexample
pair of a string w and an ERE r for which no such data structure exists. Notably, in constructing
this counterexample, we use the encoding technique used in the proof of Theorem 3.
1

While our Theorem 3(b) implies that this method requires nonelementary running time in m assuming the
hypotheses, the fact actually holds unconditionally because any algorithm that converts a given ERE to an equivalent
NFA requires nonelementary running time [SM73, Für78].

3

<!-- page 5 -->

Remark 4. By combining reductions from previous work, we can obtain O((n + m)ω−ε ) and combinatorial O((n + m)3−ε ) lower bounds for semi-ERE matching (and hence also for ERE matching)
under the k-Clique hypotheses, which are weaker than our bounds in Theorem 1(b) (and Theorem 3(b)). In particular, they are too weak to establish optimality of existing upper bounds or a
discrepancy with them in the way discussed above. In more detail, Petersen gave a combinatorial
reduction from Greibach’s hardest context-free language L0 [Gre73] to semi-ERE matching to prove
that semi-ERE matching is LOGCFL-complete [Pet02]. Here, the term “hardest” refers to the fact
that every context-free language (CFL) may be obtained as an inverse homomorphic image of L0
(up to the empty string) [Gre73]. Combining these with the combinatorial reduction by Abboud,
Backurs and Vassilevska Williams [ABV18] from 3k-Clique to the membership problem for a fixed
CFL L, we have the chain
3k-Clique ≤ L ≤ L0 ≤ semi-ERE matching,
and this gives the lower bounds mentioned above. Notably, our reductions underlying Theorems 1,
2 and 3 are obtained in a unified manner through refining and generalizing this chain by introducing
certain novel intermediate problems. See Section 1.2 for a detailed discussion.
1.1.3

Hardness of Matching with Backreference

We refer to regular expressions extended with backreference as rewbs. We only provide an informal semantics of rewbs (see, e.g., [FS19] for a formal treatment). Backreference consists of two
constructs: a capturing group (r)i to assign a label i to a string that r matches and a reference
\i to denote an expression that matches only the string labeled i. For example, the language of
(a∗ )1 \1\1 is {a3n | n ≥ 0} and that of (a∗ )1 (b∗ )2 \2\1 is {an b2m an | n, m ≥ 0}. A rewb with at
most k capturing groups is called a k-rewb. As noted earlier, k-rewb matching can be solved in
O(n2k+1 m) time for every k ≥ 1.
Theorem 5. Let k ≥ 1 be a constant. Unless moderate-dimension 2k-OVC fails, √no k-rewb matching algorithm runs in n2k−ε poly(m) time (with log-dimension 2k-OVC, n2k−ε 2o( m) time) for any
constant ε > 0.
Recently and independently of our work, Kumabe and Uezato [KU26, Theorem 5] showed a
conditional lower bound ruling out O(n2k−ε )-time k-rewb matching algorithms assuming (moderatedimension) 2k-OVC for every constant k ≥ 1. While their result may appear stronger than Theorem 5, their result actually relies on using a non-standard form of rewbs that allows self-referential
updates to capturing groups, which is unsupported in almost all practical regular expression engines.2 By contrast, our Theorem 5 relies only on basic forms of rewbs that are commonly supported
by such engines, including those in the standard libraries of Java, Python, and JavaScript, making our result more practically relevant. Furthermore, for k = 1, the proof of Theorem 5 can be
rewritten using only straight-line 1-rewbs, which are rewbs of the form e0 (e)1 e1 \1 · · · el \1el+1 where
e0 , e, e1 , . . . , el , el+1 are pure regular expressions (i.e., regular expressions without extensions). The
rewbs are practically relevant in that many rewbs that occur in practice are known to be of this
form even with l = 1 [NT25].
Theorem 6. Unless moderate-dimension OVC fails, no straight-line
1-rewb matching algorithm
√
runs in n2−ε poly(m) time (with log-dimension OVC, n2−ε 2o( m) time) for any constant ε > 0.
2

As far as we are aware, only the .NET regular expression engine supports their rewbs [Mic21].

4

<!-- page 6 -->

Table 1: Summary of upper and lower bounds for regular expression matching with one or more of the extensions
considered in this paper. SERE denotes semi-EREs and ERE + RSQ (resp. SERE + RSQ) denotes regular expression
matching extended with both complement (resp. intersection) and squaring.

Extension(s)

Upper bound

Notes

ERE + RSQ

O(nω m)
O(n3 m)
n tower(O(m))
n2O(m)
LOGCFL
O(n2k+1 m)
O(n2 m2 )

[BGJ25]
[HU79], combinatorial
see Section 1.1.2
see Section 1.1.1
[Pet02]
[Sch16, Sch24, NT25]
[Uez26]

SERE + RSQ
k-Rewb
Straight-line 1-rewb
Extension

Lower bound

Assumption

Notes

SERE / Star-free RSQ

nω−ε 2o(m)
n3−ε 2o(m)
n2−ε 2o(m)
n2−ε poly(m)
LOGCFL-hard
nω−ε tower(o(m/ log m))
n3−ε tower(o(m/ log m))
n2−ε tower(o(m/ log m))
P-hard
O(n2k−ε √
)
n2k−ε 2o( m)
n2k−ε poly(m)
√
n2−ε 2o( m)
n2−ε poly(m)

k-Clique
Comb. k-Clique
Log-dim OVC
Mod-dim OVC
—
k-Clique
Comb. k-Clique
Mod-dim OVC
—
Mod-dim 2k-OVC
Log-dim 2k-OVC
Mod-dim 2k-OVC
Log-dim OVC
Mod-dim OVC

Thm 1
Thm 1, combinatorial
Thm 1
Thm 1
[Pet02] & Thm 2
Thm 3
Thm 3, combinatorial
Thm 3
[Pet00]
[KU26]; see Sec 1.1.3
Thm 5
Thm 5
Thm 6
Thm 6

Star-free ERE

k-Rewb (k = O(1))

Straight-line 1-rewb

A recent study by Uezato [Uez26] gives an O(n2 m2 )-time algorithm for matching straight-line 1rewbs, improving the dependence on n of the O(n3 m) bound obtained by setting k = 1 in the above
O(n2k+1 m) upper bound for k-rewb matching. Therefore, Theorem 6 gives a tight lower bound for
matching rewbs of this form in that the exponent 2 of n cannot be shaved by any constant while
preserving the factor from m to be polynomial, assuming OVC.
Table 1 summarizes the upper and lower bounds discussed above for regular expression matching
extended with one or more of intersection, squaring, complement and backreference.

1.2

Technical Overview

All lower bound results shown in this paper are of the form nc−ε g(m) and therefore do not rule out
algorithms whose running time grows asymptotically faster than g(m) (e.g., O(nc−1 2m ) time when
g(m) is either poly(m) or 2o(m) ). Nevertheless, as discussed in Section 1.1, such bounds suffice to
rule out algorithms that are efficient with respect to both n and m. Moreover, restricting the time
complexity of semi-ERE, RSQ and ERE matching with respect to m is even necessary for deriving

5

<!-- page 7 -->

a meaningful lower bound on the complexity with respect to n and m because we could improve the
complexity to be linear in n if we were to allow the complexity to be exponential or nonelementary
in m as discussed in Sections 1.1.1 and 1.1.2. Indeed, investigating this form of multivariate lower
bounds has often proven fruitful and has also been pursued in prior work. For example, Abboud,
Vassilevska Williams and Wang [AVW16] proved that under log-dimension OVC, for any ε > 0, no
n2−ε 2o(k) -time algorithm decides if the diameter of a given graph of treewidth k is no more than 2.
Additionally, Abboud, Bringmann, Hermelin and Shabtay [ABHS22] proved that under SETH, for
any ε > 0, no T 1−ε 2o(n) -time algorithm solves Subset Sum on n numbers and target T .
A key idea in our reductions is to exploit the restricted dependence on m to allow the lengths
of the regular expressions produced by the reductions to depend on the sizes of the instances
to be reduced. To illustrate this, we consider a reduction from the Orthogonal Vectors (OV)
problem to regular expression matching with extensions, where OV asks whether given sets A and
B each containing N Boolean vectors of dimension d have a pair of orthogonal vectors a ∈ A
and b ∈ B. Given such sets A and B, we encode their vectors into a string w and construct a
regular expression r that nondeterministically guesses two vectors a ∈ A and b ∈ B from w and
checks their orthogonality. In proving a lower bound under
log-dimension OVC, we may assume
√
d = Θ(log N ). Accordingly, for the n2−ε 2o(m) and n2−ε 2o( m) lower bounds, we may allow r to have
length O(d) and O(d2 ), respectively. As we shall show, such r can be constructed by exploiting the
restricted-dependence-on-m property and the respective regular expression extensions.
For ERE matching, we take this idea further to show the much stronger n2−ε tower(o(m/ log m))
lower bound. Namely, we construct a reduction from OV to ERE matching that produces extremely
short EREs by using the complement extension. The idea is as follows. To decide if the two guessed
vectors a ∈ A and b ∈ B are orthogonal, we need to check that their values a[i] and b[i] multiply
to zero at each position i ∈ {1, . . . , d}. Because the complement operator is available, this can
be done by constructing an ERE that checks if “there is a position i such that a[i] = b[i] = 1”
and taking the complement of the ERE. To this end, in constructing w, we encode each value
v[i] of a vector v together with the binary representation bin(i) of each position i of fixed length
O(log d). Then, we construct an ERE that guesses positions i and j of a and b, respectively,
and checks if bin(i) = bin(j) and a[i] = b[j] = 1. Further using complement, we can also check
if bin(i) = bin(j) by constructing an ERE that guesses positions i′ and j ′ of bin(i) and bin(j)
respectively, and checks if bin(i′ ) = bin(j ′ ) and bin(i)[i′ ] ̸= bin(j)[j ′ ] and taking its complement.
After repeating this O(log∗ d) times, we only need to construct an ERE that compares strings of
constant length, and this can be done by using intersection. Consequently, we obtain an ERE of
length O((log∗ d) log(log∗ d)) for checking the orthogonality (where the additional log(log∗ d) factor
arises for technical reasons), and the n2−ε tower(o(m/ log m)) lower bound follows. As mentioned in
Section 1.1.2, we also use this encoding technique to construct our counterexample to the algorithm
of [Ros07].
We derive our results for semi-ERE, RSQ and ERE matching (Theorems 1, 2 and 3) within a
unified framework building on the reduction chain discussed in Remark 4. Recall that the reduction
chain yields only the O((n + m)ω−ε ) and combinatorial O((n + m)3−ε ) lower bounds for semiERE matching under the k-Clique hypotheses, too weak for establishing the optimality of the
O(nω m) and combinatorial O(n3 m) upper bounds for the problem. Our key observation is that the
bottleneck lies in the middle reduction L ≤ L0 and the formulation of L0 which are too general and
do not preserve enough structure of L. To address this, we “refine” L0 into a novel problem that
we call Dyck Selection (DS) by introducing parameters that retain useful structural information
of L. Additionally, we naturally extend DS to Orthogonal Dyck Selection (ODS) so as to also
accommodate a reduction from OV in addition to those from k-Clique and CFL. We then go step
further and introduce Generalized Dyck Selection (GDS), which subsumes both DS and ODS as
6

<!-- page 8 -->

OV

3k-Clique

CFL

Lemma 11

Lemma 13

Dyck Selection

Lemma 9

Lemma 8

Orthogonal Dyck Selection

Generalized Dyck Selection
Lemma 15

Lemma 19

Semi-ERE Matching

ERE Matching

Lemma 16

RSQ Matching

Figure 1: Overview of the reductions in Section 3.

special cases.
As indicated in Figure 1, DS, ODS and GDS serve as useful intermediate problems. Indeed,
as we shall show in Section 3, by modifying Petersen’s LOGCFL-hardness reduction for semi-ERE
matching [Pet02] in a certain way so as to keep the length m of the regular expression in the output
instance small, we obtain reductions from GDS to semi-ERE and ERE matching and a reduction
from ODS to RSQ matching that yield the desired conditional lower bounds under OVC and the
k-Clique hypotheses, as well as their LOGCFL-hardness. As such, these problems, DS, ODS and
GDS, formally capture the essence of why the matching problems for semi-ERE, RSQ and ERE
are hard, and we foresee them to be also useful for investigating the complexity of other problems,
particularly ones that concern formal language theory or stringology.
The rest of the paper is organized as follows. Section 2 defines preliminary notions. Section 3
introduces the new intermediate problems DS, ODS and GDS and presents our results on semiERE, RSQ and ERE matching by proving the reductions shown in Figure 1. Section 4 reviews
the algorithm of [Ros07] and presents our counterexample. Section 5 presents our results on rewb
matching.

2

Preliminaries

For integers i, j, we write [i, j] for the set {i, i + 1, . . . , j}. All logarithms are base 2. For integer
x ≥ 1, we let log(0) x = x, and for i ≥ 1, let log(i) x = log(log(i−1) x). We denote by log∗ x the
minimum i ≥ 0 such that log(i) x ≤ 1. For a string or a vector s and an integer i ≥ 1, we write
srev for the reverse of s, |s| for the length of s and s[i] for the i-th coordinate of s. For an integer
i ≥ 0, we write si for s concatenated i times. We assume that (−)rev , (−)[i] and (−)i have higher
rev
precedence than concatenation. For example, s1 srev
2 denotes s1 (s2 ), s1 s2 [i] denotes s1 (s2 [i]) and
i
i
s1 s2 denotes s1 (s2 ) for strings s1 and s2 . We usually omit the string concatenation symbol, but
occasionally write it explicitly using ⊙. The syntax of (pure) regular expressions over an alphabet
Σ is given by the following grammar:
r ::= a | λ | ∅ | r1 r2 | r1 + r2 | r∗
where a ∈ Σ and λ denotes the empty string. As with strings, we use ⊙ as the concatenation symbol
when necessary. For a regular expression r and an integer i ≥ 0, we write ri for r concatenated
7

<!-- page 9 -->

i times. We assume the standard operator precedence: Kleene star (−)∗ , concatenation ⊙ and
alternation + (in decreasing order). We also assign (−)i the same precedence as Kleene star,
although we do not regard (−)i as a primitive operator in regular expressions. For example, r1 r2i
denotes r1 (r2i ) for regular expressions r1 and r2 . For brevity, we use the character set notation
[a1 a2 · · · ak ], which is a shorthand for the regular expression a1 + a2 + · · · + ak where a1 , . . . , ak ∈ Σ.
For any set S ⊆ Σ, we regard S as the character set consisting of all elements of S. The language
of a regular expression is defined in the standard way. A regular expression r matches a string w if
w is an element of the language of r, which is also written as w ∈ r by a slight abuse of notation.
Two regular expressions are equivalent if they define the same language. LOGCFL is the class of
all decision problems reducible to a context-free language under log-space reductions.
Let k ≥ 2 be an integer. The k-Orthogonal Vectors (k-OV) problem is the following: given k
sets A1 , . . . , Ak ⊆ {0, 1}d containing the same number n of Boolean vectors ofP
dimension
d, decide
d Qk
if there are k vectors a1 ∈ A1 , . . . , ak ∈ Ak such that they are orthogonal (i.e., l=1 i=1 ai [l] = 0).
A naive algorithm solves k-OV in O(nk d) time.
Conjecture (k-Orthogonal Vectors Conjecture, see [WY14, CW19, KW19, Wil24] for k = 2, and
[BCFK22, Hypothesis 24] for general k). For every constant ε > 0, there is a constant c ≥ 1 such
that k-OV cannot be solved in O(nk−ε ) time on instances with d = c log n.
In particular, 2-OV and 2-OVC are simply called OV and OVC, respectively. As mentioned in
the introduction, it is known that SETH implies k-OVC [Wil05, WY14, BCFK22], and therefore
bounds obtained by assuming k-OVC also hold when assuming SETH. Moreover, for k = 2, OVC
is supported by evidence beyond their implication from SETH. See, for example, [CW19, KW19].
In this paper, we assume log-dimension k-OVC in order to derive more refined lower bounds with
respect to m. If one is content with polynomial-in-m bounds, assuming the following moderatedimension version of k-OVC is enough.
Conjecture (Moderate-dimension k-OVC, see [Wil18, GIKW19]). For every constant ε > 0, k-OV
cannot be solved in nk−ε poly(d) time.
Let k ≥ 3 be an integer. The k-Clique problem asks whether a given undirected graph G
has a k-clique (i.e., the complete graph on k vertices) as a subgraph. For a fixed k, a naive
algorithm solves k-Clique in O(nk ) time. Abboud, Fischer and Shechter [AFS24] have proposed a
polylogarithmically faster combinatorial algorithm for the problem.
Hypothesis (Combinatorial k-Clique Hypothesis, see [ABV18, AFS24]). For any integer k ≥ 3
and any constant ε > 0, no combinatorial algorithm can solve k-Clique on a graph of n vertices in
O(nk−ε ) time.
Using fast matrix multiplication, Nešetřil and Poljak showed a faster algorithm that runs in
O(nω⌈k/3⌉ ) time [NP85]. Here, 2 ≤ ω < 2.3714 [ADV+ 25] denotes the exponent of square matrix
multiplication.
Hypothesis (k-Clique Hypothesis, see [ABV18, Wil18]). For any integer k ≥ 3 and any constant
ε > 0, no algorithm can solve k-Clique on a graph of n vertices in O(nωk/3−ε ) time.

3

OV, Clique and CFL to Semi-ERE, RSQ and ERE Matching

In this section, we prove Theorems 1, 2 and 3. To this end, we introduce the novel intermediate
problems Dyck Selection (DS), Orthogonal Dyck Selection (ODS) and Generalized Dyck Selection
(GDS) in Section 3.1, and present reductions to and from them in the subsequent sections.
8

<!-- page 10 -->

3.1

GDS, DS and ODS

Following [Kem96, Lie03], we first present an abstract framework for a generalization of the Dyck
language, a basic formal language consisting of strings of properly nested parentheses (see, e.g.,
¯ = {x̄ | x ∈ ∆}
[HU79]). Given a set ∆ of opening parentheses with the closing parentheses set ∆
¯ and R is defined
and a binary relation R on ∆, the generalized Dyck language with respect to ∆, ∆
∗
¯
as the set {w | w ∼R λ}, where ∼R is the smallest congruence on (∆∪ ∆) generated by the relation
{(xȳ, λ) | x, y ∈ ∆, xRy}.
Next, we instantiate this framework as follows. For a constant p ≥ 1, let Γp = {α1 , . . . , αp } and
Γ̄p = {ᾱi | αi ∈ Γp }. For notation convenience, we often write Γ and Γ̄ for Γp and Γ̄p , respectively.
For an integer β ≥ 1, let Vβ (Γ) be the set of β-dimensional vectors over Γ and Vβ (Γ̄) the set of
those over Γ̄. For x ∈ Vβ (Γ), we write x̄ for the β-dimensional vector such that x̄[i] = x[i] for each
i ∈ [1, β]. Given a binary relation R on Γ, we define DR as the generalized Dyck language with
respect to Vβ (Γ), Vβ (Γ̄) and R lifted to Vβ (Γ), i.e., xRy ⇐⇒ ∀i ∈ [1, β] : x[i] R y[i]. In particular,
if p = 1, β = 1 and R = {(α1 , α1 )}, then DR coincides with the Dyck language D1 .
A string in D1 can naturally be viewed as a rooted tree as follows. Every w ∈ D1 has a unique
decomposition w = α1 w1 ᾱ1 · · · α1 wℓ ᾱ1 for some ℓ ≥ 0 and w1 , . . . , wℓ ∈ D1 (see Lemma 35 in
Appendix A.5 for the proof). Using this decomposition, we associate w with a rooted tree T (w),
inductively defined as the tree whose root has T (w1 ), . . . , T (wℓ ) as its subtrees in this order, with
T (λ) consisting of a single leaf. We define the depth of w ∈ D1 to be the maximum depth among the
vertices of T (w), i.e., the height of T (w). We also define the degree of w ∈ D1 to be the maximum
degree among the vertices of T (w), where the degree of a vertex in a rooted tree is defined as the
number of its children. For example, ()(())(()()) has depth 2 and degree 3, where the opening and
(δ)
closing parentheses denote α1 and ᾱ1 , respectively. For integers δ, γ ≥ 0, let D1 be the set of
(δ,γ)
strings in D1 of depth at most δ, and D1
those of depth at most δ and degree at most γ.
∗
∗
Let πβ : (Vβ (Γ) ∪ Vβ (Γ̄)) → (Γ1 ∪ Γ̄1 ) be the homomorphism defined by πβ (Vβ (Γ)) = {α1 } and
πβ (Vβ (Γ̄)) = {ᾱ1 }. We are ready to define our general problem GDS and two of its special cases,
DS and ODS.
Definition 7. Let X ⊆ D1 , which we call the restriction set.
• Generalized Dyck Selection (GDS). Let R be a binary relation on Γ. Given n sets
S1 , . . . , Sn of strings over Vβ (Γ) ∪ Vβ (Γ̄), decide if there exist s1 , . . . , sn with si ∈ Si for every
i such that s1 · · · sn ∈ πβ−1 (X) ∩ DR .3
• Dyck Selection (DS). The special case of Generalized Dyck Selection where p = 2 and
R = Req = {(α1 , α1 ), (α2 , α2 )}. We denote DReq by D2 .
• Orthogonal Dyck Selection (ODS). The special case of Generalized Dyck Selection where
p = 2 and R = Rorth = {(α1 , α1 ), (α1 , α2 ), (α2 , α1 )}. We denote DRorth by J2 . For notation
convenience, we write 0, 1, 0̄, 1̄ instead of α1 , α2 , ᾱ1 , ᾱ2 when discussing this problem.
Note that setting X = D1 imposes no restriction in either problem
P(seePCorollary 28 in Appendix A.1). For both problems, we define the input size N by N = ni=1 s∈Si max{β · |s|, 1}.
See Section 1.2 for the motivation for considering these problems.
In what follows, we show the reductions indicated in Figure 1 of Section 1.2. By definition,
GDS is at least as hard as ODS and DS. We next show that ODS is at least as hard as DS.
GDS is defined using DR , the generalized Dyck language with the vector alphabets ∆ = Vβ (Γ), ¯
∆ = Vβ (Γ̄) and
¯ and a binary relation
a component-wise binary relation on these vectors, rather than the one with arbitrary ∆, ∆
between them. We do this so that GDS itself can be incorporated into our reduction framework along with its special
cases DS and ODS (see Figure 1 in Section 1.2).
3

9

<!-- page 11 -->

Lemma 8. Let X ⊆ D1 . Suppose Orthogonal Dyck Selection with restriction set X can be solved in
time T (N, β). Then Dyck Selection with restriction set X can be solved in time O(N ) + T (2N, 2β)
using constant additional space.
Proof. Let h : (Vβ (Γ) ∪ Vβ (Γ̄))∗ → (V2β (Γ) ∪ V2β (Γ̄))∗ be the unique homomorphism obtained by
applying the following substitutions component-wise to each vector symbol:
α1 7→ 01,

α2 7→ 10,

ᾱ1 7→ 1̄0̄,

ᾱ2 7→ 0̄1̄.

For example, when β = 2, h((α1 , α2 )(ᾱ1 , ᾱ2 )) = (0, 1, 1, 0)(1̄, 0̄, 0̄, 1̄).
Given a DS instance S1 , . . . , Sn ⊆ (Vβ (Γ) ∪ Vβ (Γ̄))∗ of size N , convert it into an ODS instance
h(S1 ), . . . , h(Sn ) ⊆ (V2β (Γ) ∪ V2β (Γ̄))∗ of size 2N , where h(Si ) = {h(s) | s ∈ Si }. This can be done
in O(N ) time and constant space. Let w ∈ (Vβ (Γ) ∪ Vβ (Γ̄))∗ . Observe that πβ (w) = π2β (h(w)) and
hence πβ (w) ∈ X ⇐⇒ π2β (h(w)) ∈ X. We claim that w ∈ D2 ⇐⇒ h(w) ∈ J2 by induction on the
length of w (see Appendix A.2 for details). Therefore, DS can be solved in time O(N ) + T (2N, 2β)
by solving the ODS instance.

3.2

OV, Clique and CFL to DS and ODS

Next, we show reductions from OV, 3k-Clique and every context-free language to our problems.
Lemma 9. Suppose Orthogonal Dyck Selection with some restriction set containing {α1 ᾱ1 } can be
solved in time T (N, β). Then OV can be solved in time O(nd) + T (2nd, d).
Proof. We construct a reduction from OV to ODS. Given sets A and B of d-dimensional vectors
with |A| = |B| = n, we encode them into sets S1 = A ⊆ Vd (Γ) and S2 = {ȳ | y ∈ B} ⊆ Vd (Γ̄).
The ODS instance on S1 and S2 has size 2nd and can be constructed in O(nd) time. Because
πd (xȳ) = α1 ᾱ1 for any x ∈ S1 and ȳ ∈ S2 , it suffices to take {α1 ᾱ1 } as the restriction set for this
instance. It is easy to see that x ∈ A and y ∈ B are orthogonal if and only if xȳ ∈ J2 . Therefore,
OV can be solved in time O(nd) + T (2nd, d) by solving the ODS instance.
Corollary 10. If Orthogonal Dyck Selection with some restriction set containing {α1 ᾱ1 } can be
solved in N 2−ε poly(β) time for some constant ε > 0, then moderate-dimension OVC is false. If it
can be solved in N 2−ε 2o(β) time for some constant ε > 0, then log-dimension OVC is false.
Proof. Fix a constant ε > 0 and suppose that ODS with some restriction set containing {α1 ᾱ1 } can
be solved in N 2−ε g(β) time. If g(β) = poly(β), then by Lemma 9, OV can be solved in n2−ε poly(d)
time, refuting moderate-dimension OVC. If g(β) = 2o(β) and d = c log n for some constant c ≥ 1,
then OV can be solved in n2−ε+o(1) time, refuting log-dimension OVC.
(O(k2 ),4)

Lemma 11. Let k ≥ 1. Suppose Dyck Selection with some restriction set containing D1
be solved in time T (N, β). Then 3k-Clique can be solved in time

can

O(k 3 nk+1 ) + T (O(k 3 nk+1 log n), O(log n)).
Proof. We show a combinatorial reduction from 3k-Clique to DS by adopting the approach of
Abboud, Backurs and Vassilevska Williams [ABV18] who showed a combinatorial reduction from
3k-Clique to the recognition problem for context-free languages. As in their reduction, we list all
k-cliques in a given graph and check if there are three disjoint k-cliques that are fully connected
with each other.

10

<!-- page 12 -->

Given a graph G of n vertices, we fix β = O(log n) and identify its vertices with distinct vectors
in Vβ (Γ). Let Ck be the set of all k-cliques of G. Fix an arbitrary ordering of each k-clique t
(e.g., the increasing order of vertex indices), and denote the i-th vertex in t by t[i]. We identify
each t ∈ Ck with the length-k string t[1] · · · t[k] over Vβ (Γ). Let VG(t) = t[1]k · · · t[k]k . Define the
following sets of strings over Vβ (Γ) ∪ Vβ (Γ̄):
S1 = {VG(t)VG(t) | t ∈ Ck }, S2 = {t | t ∈ Ck }, S3 = {t̄rev VG(t) | t ∈ Ck }, S4 = {t̄rev | t ∈ Ck },
Ni = {t̄rev ūt | u ∈ N (t[i]), t ∈ Ck } for i ∈ [1, k], where N (t[i]) is the neighborhood of t[i],
and form a sequence S of these sets
S = S1 , S2 , N , . . . , N , S3 , S2 , N , . . . , N , N , . . . , N , S4
| {z }
| {z } | {z }
k copies

k copies

k copies

where N = N1 , . . . , Nk . The DS instance on S has size O(k 3 nk+1 log n) and can be constructed in
(O(k2 ),4)
O(k 3 nk+1 ) time. Observe that it suffices to take D1
as the restriction set for this instance.
We claim that G has a 3k-clique if and only if this is a yes-instance (see Appendix A.3 for details).
Therefore, 3k-Clique can be solved in the claimed time by solving the DS instance.
Corollary 12. If, for some constant ε > 0, Dyck Selection with some restriction set containing
(δ,4)
D1
can be solved in N ω−ε 2o(β) time for every constant δ > 0, then the k-Clique Hypothesis is
false. If, for some constant ε > 0, it can be solved combinatorially in N 3−ε 2o(β) time for every
constant δ > 0, then the Combinatorial k-Clique Hypothesis is false.
Proof. Let c ∈ {ω, 3}. Fix a constant ε > 0 and suppose that DS with some restriction set
(δ,4)
containing D1
can be solved in N c−ε 2o(β) time for every constant δ > 0. Let k be a sufficiently
large constant such that c < (k +1)ε. By Lemma 11, we can solve 3k-Clique in time n(k+1)(c−ε)+o(1) ,
′
which is O(nck−ε ) for some ε′ > 0. Because the reduction is combinatorial, a combinatorial
algorithm remains combinatorial after the reduction, completing the proof.
Lemma 13. Every context-free language is (simultaneous) linear-time and constant-space reducible
to Dyck Selection.
Proof. The proof is almost identical to that of [Gre73] (see Appendix A.4 for details).
Corollary 14. Dyck Selection and Orthogonal Dyck Selection with restriction set D1 are LOGCFLhard, and Generalized Dyck Selection with restriction set D1 is in LOGCFL for any binary relation
R. In particular, Dyck Selection and Orthogonal Dyck Selection are LOGCFL-complete.
Proof. The LOGCFL-hardness of DS follows from Lemma 13. By Lemma 8, ODS is also LOGCFLhard. See Appendix A.4 for a proof that GDS is in LOGCFL.

3.3

GDS to Semi-ERE Matching and ODS to RSQ Matching

Next, we show reductions from Generalized Dyck Selection to semi-ERE matching and from Orthogonal Dyck Selection to RSQ matching. Semi-ERE [HU79] extends regular expressions with the
intersection construct r1 ∩ r2 . The language of a semi-ERE is defined by letting the language of
r1 ∩ r2 be the intersection of the languages of r1 and r2 . For example, (aa)∗ ∩ (aaa)∗ is equivalent
to (aaaaaa)∗ , whose language is {a6n | n ≥ 0}. We assign intersection ∩ the same precedence as
alternation +.
11

<!-- page 13 -->

Lemma 15. Suppose semi-ERE matching can be solved in time T (n, m). Then Generalized Dyck
(δ)
Selection with restriction set D1 can be solved in time O(N + βδ) + T (O(N ), O(βδ)).
Proof. We construct a reduction from GDS to semi-ERE matching. Our construction closely follows
that of Petersen [Pet02, Lemma 4]. Given a GDS instance S1 , . . . , Sn ⊆ (Vβ (Γ)∪Vβ (Γ̄))∗ , we encode
it as



n
K
K

w=
#ϕ(s) #$ ,
i=1

s∈Si

(Γ̄))∗ → (Γβ ∪ Γ̄β )∗ is the homomorphism defined by ϕ(x) = x[1] · · · x[β] and

where ϕ : (Vβ (Γ) ∪ Vβ
rev
ϕ(x̄) = ϕ(x) for x ∈ Vβ (Γ). Both the length of w and the time required for this construction are
O(N ). We then define a semi-ERE r over the alphabet Σ = Γ ∪ Γ̄ ∪ {#, $} as follows:
r = (Σ \ {$})∗ #rδ #(Σ \ {$})∗ $,
ri = r0 (rzero,β ⟨ri−1 ⟩r0 )∗ for i ∈ [1, δ],
r0 = #(Σ \ {$})∗ $(Σ \ {$})∗ # + λ,
rzero,j ⟨r′ ⟩ = Γ rzero,j−1 ⟨r′ ⟩ Γ̄ ∩ rR for j ∈ [1, β],
rR = a1 Σ∗ b̄1 + · · · + ak Σ∗ b̄k where R = {(a1 , b1 ), . . . , (ak , bk )},
rzero,0 ⟨r′ ⟩ = r′ .

Both the length of r and the time required for this construction are O(βδ). We claim that the given
GDS instance S1 , . . . , Sn is a yes-instance if and only if r matches w (see Appendix A.5 for details).
Therefore, the given GDS instance can be solved in the claimed time by solving the semi-ERE
matching instance.
We next consider RSQ matching. RSQ [MS72] extends regular expressions with the squaring
construct r{2}, which denotes an expression equivalent to rr. We assign squaring (−){2} the same
precedence as Kleene star.
Lemma 16. Suppose RSQ matching can be solved in time T (n, m). Then Orthogonal Dyck Selec(δ)
tion with restriction set D1 can be solved in time O(N + βδ) + T (O(N ), O(βδ)).
Proof. We construct a reduction similar to that in Lemma 15, but with squaring instead of intersection. Given an ODS instance S1 , . . . , Sn ⊆ (Vβ (Γ) ∪ Vβ (Γ̄))∗ , we encode it as



n
K
K

w=
#enc(ϕ(s)) #$ ,
i=1

s∈Si

where ϕ : (Vβ (Γ) ∪ Vβ (Γ̄))∗ → (Γβ ∪ Γ̄β )∗ is the homomorphism defined in the proof of Lemma 15
and enc : (Γ2 ∪ Γ̄2 )∗ → {1, ¢1 , ¢2 , ¢̄1 , ¢̄2 }∗ is the homomorphism defined by enc(0) = ¢1 1¢2 , enc(1) =
¢1 11¢2 , enc(0̄) = ¢̄2 1¢̄1 and enc(1̄) = ¢̄2 11¢̄1 . Both the length of w and the time required for this
construction are O(N ). We then define an RSQ r over the alphabet Σ = {1, ¢1 , ¢2 , ¢̄1 , ¢̄2 , #, $} as
follows:
r = (Σ \ {$})∗ #rδ #(Σ \ {$})∗ $,
ri = r0 (rzero,β ⟨ri−1 ⟩r0 )∗ for i ∈ [1, δ],
r0 = #(Σ \ {$})∗ $(Σ \ {$})∗ # + λ,
rzero,j ⟨r′ ⟩ = ¢1 (1¢2 rzero,j−1 ⟨r′ ⟩ ¢̄2 1 + 1 + λ){2}¢̄1 for j ∈ [1, β],
rzero,0 ⟨r′ ⟩ = r′ .
12

<!-- page 14 -->

(Only the definition of rzero,j ⟨r′ ⟩ differs from that in Lemma 15.) Both the length of r and the
time required for this construction are O(βδ). We claim that the given ODS instance S1 , . . . , Sn is
a yes-instance if and only if r matches w (see Appendix A.6 for details). Therefore, the given ODS
instance can be solved in the claimed time by solving the RSQ matching instance.
Corollary 17. Suppose star-free RSQ matching can be solved in time T (n, m). Then Orthogonal
(δ)
Dyck Selection with restriction set D1 can be solved in time
O(N + (β + log N )δ) + T (O(N ), O((β + log N )δ))
using O(log β + log δ + log N ) additional space.
Proof. We avoid using the Kleene star in the construction of r by adopting a technique used in
[Pet02, Lemma 2]. Let e∗ be a starred subexpression of r. Instead of e∗ , we use
(· · · (((e + λ){2}){2}) · · · ){2}
where squares are nested ⌈log |w|⌉ times. Then, r matches w if and only if it matches w after this
change, because the matching is determined by which subexpressions of r match which substrings
of w and the lengths of the latter are at most |w|. Both the length of r and the time required
for the construction increase to O((β + log N )δ). The additional O(log β + log δ + log N ) space is
required for counters needed for the construction of r and the above technique.
Theorem 2 follows from Lemma 13 and Corollary 17 by setting δ = N . We next prove Theorem 1.
Proof of Theorem 1. Let c ≥ 2 be a constant and g a function to be chosen later. Suppose that
semi-ERE matching on a string of length n and a semi-ERE of length m can be solved in nc−ε g(m)
time for some ε > 0. By choosing ε sufficiently small, we may assume that c − ε ≥ 1.
(1)
Set c = 2. By Lemma 15, we can solve ODS with restriction set D1 in N 2−ε g(O(β)) time.
(1)
Because {α1 ᾱ1 } ⊆ D1 , Corollary 10 refutes moderate-dimension OVC if g(m) = poly(m), and it
refutes log-dimension OVC if g(m) = 2o(m) . These prove part (a) of the theorem.
(δ)
For (b), set g(m) = 2o(m) . By Lemmas 8 and 15, we can solve DS with restriction set D1
(δ,4)
(δ)
for any constant δ > 0 in N c−ε 2o(β) time. Because D1
⊆ D1 , Corollary 12 with c = ω and
c = 3 refutes the k-Clique hypotheses. The proof for star-free RSQ matching is analogous, using
Corollary 17.
√

Remark 18. If one is content with the weaker nc−ε 2o( m) lower bound, we can show the hardness
of semi-ERE matching with a single use of unbounded intersection.
T Indeed, it is easy to see that
′
rzero,β ⟨r′ ⟩ in the proof of Lemma 15 is equivalent to rzero,β
⟨r′ ⟩ = ( βj=1 Γj−1 rR Γ̄j−1 )∩Γβ r′ Γ̄β , which
′
uses unbounded intersection exactly once and no other intersection. When rzero,β
⟨r′ ⟩ is used instead
of rzero,β ⟨r√′ ⟩, the length of r and its construction time become O(β 2 ). Therefore, the resulting bound
is nc−ε 2o( m) . Moreover, the n2−ε poly(m) lower bound under moderate-dimension OVC still holds
after this modification.

3.4

GDS to ERE Matching

Next, we show a reduction from Generalized Dyck Selection to ERE matching. ERE [HU79] extends
regular expressions with the complement construct ¬r. The language of ¬r is the complement of
the language of r, that is, the set of strings over the alphabet that are not matched by r. For
13

<!-- page 15 -->

example, ¬((aa)∗ ) is equivalent to a(aa)∗ over the alphabet {a}. We assign complement ¬ the
same precedence as Kleene star. We use the intersection r1 ∩ r2 as a shorthand for ¬(¬r1 + ¬r2 ).
Lemma 19. Suppose star-free ERE matching can be solved in time T (n, m). Then Generalized
(δ,γ)
Dyck Selection with restriction set D1
can be solved in time
N β 1+o(1) + O(γ δ (log∗ β) log(log∗ β)) + T (N β 1+o(1) , O(γ δ (log∗ β) log(log∗ β))).
Proof. We construct a reduction from GDS to star-free ERE matching. For integer x ≥ 1, we
define ℓ(x) = ⌊log(x) + 1⌋. Observe that any integer 1 ≤ i ≤ x has the binary representation of
length ℓ(x), which we denote by binℓ(x) (i). Let ℓ(i) (x) = ℓ(ℓ(i−1) (x)) for i ≥ 1 and ℓ(0) (x) = x.
Let Σ = Γ ∪ Γ̄ ∪ {¢1 , ¢2 , %, ¢̄1 , ¢̄2 , %̄, #, $}. Let S1 , . . . , Sn ⊆ (Vβ (Γ) ∪ Vβ (Γ̄))∗ be a given GDS
instance. If β ≤ 2, we may increase β to at least 3 by repeating each character three times, without
changing the answer of the instance (this follows by a straightforward induction using Lemma 42
from Appendix A.7). We encode the instance as



n
K
K

w=
#ench (ϕ(s)) #$ ,
i=1

s∈Si

where ϕ : (Vβ (Γ) ∪ Vβ (Γ̄))∗ → (Γβ ∪ Γ̄β )∗ is the homomorphism defined in the proof of Lemma 15,
h ≥ 1 is the smallest integer such that ℓ(h) (β) ≤ 2, and ench : (Γβ ∪ Γ̄β )∗ → (Σ \ {#, $})∗ is the
homomorphism induced by the map ench : Γβ ∪ Γ̄β → (Σ \ {#, $})∗ defined as


|v|
K
(h−i) (β)
enci (v) = 
¢1,i enci−1 (binℓ(|v|) (j)) ¢2,i v[j] %i for i ∈ [1, h] and v ∈ Γℓ
,
j=1

where ¢1,i = ¢1 binℓ(h) (i), ¢2,i = ¢2 binℓ(h) (i) and %i = % binℓ(h) (i),
enc0 (v) = v

(h) (β)

for v ∈ Γℓ
rev

enci (v̄) = enci (v)

,
(h−i) (β)

for i ∈ [0, h] and v ∈ Γℓ

.

Note that h = O(log∗ β). Indeed, we prove by induction that tower(i − 1) + 2 ≤ ℓ(h−i) (β) for all
i ∈ [1, h]. The base case i = 1 holds because ℓ(h−1) (β) ≥ 3. Assume that the inequality holds for
some i ≥ 1. Then,
tower(i − 1) + 2 ≤ ℓ(h−i) (β) = ℓ(ℓ(h−(i+1)) (β)) ≤ log(ℓ(h−(i+1)) (β)) + 1
and hence ℓ(h−(i+1)) (β) ≥ 2tower(i−1)+1 ≥ tower(i) + 2. In particular, tower(h − 1) ≤ β holds and
therefore h = O(log∗ β).
We compute the length of w. By definition, the length of enci (v) depends only on the length of

14

<!-- page 16 -->

v. Hence, for simplicity, we only present the case v = 1β . Then we have
|ench (1β )| ≤ β · |ench−1 (1ℓ(β) )| + 4(1 + ℓ(h))β
(2) (β)

≤ βℓ(β) · |ench−2 (1ℓ

)| + 4(1 + ℓ(h))(β + βℓ(β))
(3)

≤ βℓ(β)ℓ(2) (β) · |ench−3 (1ℓ (β) )| + 4(1 + ℓ(h))(β + βℓ(β) + βℓ(β)ℓ(2) (β))
..
.
!
j
h−1
h−1 Y
Y
X
(i)
ℓ(h) (β)
≤
ℓ (β) · |enc0 (1
)| + 4(1 + ℓ(h))
ℓ(i) (β)
i=0

≤ 9hℓ(h)

j=0 i=0
h
Y

ℓ(i) (β) = O(β(log2 β)(log∗ β) log(log∗ β)) = β 1+o(1) .

i=0

Qh

Here, we use that i=2 ℓ(i) (β) = O(log β) [Vol99, Exercise 1.3(2)]. Therefore, |w| = N β 1+o(1) .
Next, we analyze the time required for the construction of w. For v ∈ Γβ , we construct ench (v)
as follows. We compute ℓ(i) (β), i ∈ [1, h], and h by repeatedly counting the number of digits in
binary until it is at most 2. This can be done in time
!
!
h
h
X
Y
(i)
(i)
O
ℓ (β) ≤ O(β) + O
ℓ (β) = O(β) + O(log β) = O(β).
i=0

i=2

Then, we generate ench (v) from left to right using a counter c = (ch , . . . , c1 ), with each ci ranging
from 1 to ℓ(h−i) (β), and with c incremented in lexicographic order. Overall, β 1+o(1) time suffices to
construct ench (v). Therefore, w can be constructed in N β 1+o(1) time.
Next, we define a star-free ERE r over Σ as follows, where we use Σ∗ as shorthand for ¬∅ and
write ∩ vertically for readability:
r = rexcl,$ #rδ # rexcl,$ $,
ri = r0 (rzero,β ⟨ri−1 ⟩ r0 + λ)γ for i ∈ [1, δ],
r0 = # rexcl,$ $ rexcl,$ # + λ,





rzero,β ⟨r′ ⟩ = ¬ 
rexcl,%h ¢1,h 

req,ℓ(1) (β)
∩

rexcl,¢2,h ¢2,h ¬rR ¢′2,h rexcl,¢′2,h






 ′
¢ r

 1,h excl,%′h 

∩ ¢1,h rexcl,%h %h r′ %′h rexcl,%′h ¢′1,h ,
rR = a1 Σ∗ b̄1 + · · · + ak Σ∗ b̄k where R = {(a1 , b1 ), . . . , (ak , bk )},




req,ℓ(i+1) (β)
 ′




¢

∩
req,ℓ(i) (β) = ¬ 
rexcl,%h−i ¢1,h−i 
 1,h−i rexcl,%′h−i 
′
rexcl,¢2,h−i ¢2,h−i ¬req ¢2,h−i rexcl,¢′2,h−i
for i ∈ [1, h − 1],
where

¢1,i = ¢1 binℓ(h) (i), ¢2,i = ¢2 binℓ(h) (i), %i = % binℓ(h) (i),
rev
¢′2,i = ¢2,i rev ,
%′i = %i ,
¢′1,i = ¢1,i rev ,

req,2 = Σreq,1 Σ ∩ req,1 ,

req,1 = req = 0Σ∗ 0̄ + 1Σ∗ 1̄,

rexcl,s = ¬(Σ∗ sΣ∗ ) for s ∈ Σ∗ .
15

<!-- page 17 -->

Note that req,ℓ(h) (β) is either req,1 or req,2 because ℓ(h) (β) ≤ 2. A simple calculation gives
|r| = O(γ δ h log h) = O(γ δ (log∗ β) log(log∗ β)),
and r can also be constructed in the same time bound using the value of h, which has been already
computed during the construction of w.
We claim that the given GDS instance S1 , . . . , Sn is a yes-instance if and only if r matches w
(see Appendix A.7 for details). Therefore, the given GDS instance can be solved in time
N β 1+o(1) + O(γ δ (log∗ β) log(log∗ β)) + T (N β 1+o(1) , O(γ δ (log∗ β) log(log∗ β)))
by solving the star-free ERE matching instance.
We are now ready to prove Theorem 3.
Proof of Theorem 3. The proof is similar to that of Theorem 1, using Lemma 19. Let c ≥ 2 be a
constant to be chosen later. Suppose that star-free ERE matching on a string of length n and an
ERE of length m can be solved in nc−ε tower(o(m/ log m)) time for some ε > 0. By choosing ε
sufficiently small, we may assume that c − ε ≥ 1.
(δ,4)
By Lemma 19, we can solve ODS with restriction set D1
for any constant δ > 0 in time
!!
(log∗ β) log(log∗ β)
1+o(1) c−ε
= nc−ε poly(β).
(N β
)
tower o
∗
(2)
∗
log(log β) + log (log β)
As in the proof of Theorem 1, if c = 2, Corollary 10 refutes moderate-dimension OVC, and if
c ∈ {ω, 3}, Lemma 8 and Corollary 12 refute the k-Clique hypotheses.

4

A Counterexample to the ERE Matching Algorithm of [Ros07]

In this section, we show that the O(n2 · (log n + m) · 2m )-time ERE matching algorithm proposed
in [Ros07, Theorem 1] has a structural flaw, by providing a counterexample to a fundamental
assumption of the algorithm.
Their algorithm is as follows. Let w = a1 · · · an be a string, where a1 , . . . , an are characters,
and let r be an ERE. The basic idea of the algorithm is to calculate the (n + 1) × (n + 1) Boolean
matrix T such that for any 0 ≤ i < j ≤ n,
T [i][j] = 1

⇐⇒

ai+1 · · · aj ∈ ¬r′

for each negated subexpression ¬r′ of r. As explained in [Ros07, Proposition 3], this gives an
ERE matching algorithm that runs in O(n3 k + n2 m) time, where k is the number of complement
operators in r. Then, to obtain a more efficient algorithm with respect to n, a data structure called
jumping machine is introduced for expressing the above matrix more compactly with respect to n.
Definition ([Ros07, Definition 6]). A jumping machine P = (P, p0 , π) consists of set P of states,
an initial state p0 , and a jumping map π : [0, n − 1] × P → ([1, n] × P ) ∪ {⊥} with the property that
for any i ∈ [0, n − 1] and any p ∈ P , if π(i, p) = (j, p′ ), then i < j. Given i ∈ [0, n] and p ∈ P , we
let πp (i) = {j | ∃p′ ∈ P. ∃l ≥ 1. π l (i, p) = (j, p′ )}. Here, π l (i, p) = π l−1 (π(i, p)) if π(i, p) ̸= ⊥, and
π l (i, p) = ⊥ otherwise for l ≥ 2, and π 1 (i, p) = π(i, p). Given a word w = a1 · · · an and an ERE r,
we say that P = (P, p0 , π) is a jumping machine for w and r if for any i ∈ [0, n − 1], we have
πp0 (i) = {j ∈ [i + 1, n] | ai+1 · · · aj ∈ r}.
16

<!-- page 18 -->

The O(n2 · (log n + m) · 2m )-time ERE matching algorithm is obtained from the algorithm above
by replacing Boolean matrices with jumping machines with O(2m ) states [Ros07, Theorem 1]. At
this point, the following is assumed:
For any string w and ERE r, there is a jumping machine with O(2m ) states for w and r.

(¶)

However, we show that the claim (¶) is incorrect for general EREs.4 That is, we provide w and r
such that any jumping machine for them must have a number of states that is super-exponential
in m. We first prove the following lemma, which presents a lower bound on the number of states
of any jumping machines for w and r.
Lemma 20. Let w = a1 · · · an be a string and r be an ERE. Let j be a position such that ai+1 · · · aj ∈
r for all i ∈ [0, j − 1]. Let C(w, r, j) be the number of distinct sets in the collection {ξ(i) | i ∈
[0, j − 1]}, where
ξ(i) = {j ′ ∈ [j + 1, n] | ai+1 · · · aj ′ ∈ r}.
Then, any jumping machine for w and r must have at least C(w, r, j) states.
Proof. For each i ∈ [0, j − 1], there exist pi ∈ P and l ≥ 1 such that π l (i, p0 ) = (j, pi ) because
ai+1 · · · aj ∈ r. We claim that πpi (j) = ξ(i). This holds because both are obtained by intersecting
πp0 (i) = {j ′ ∈ [i + 1, j] | ∃p′ ∈ P. ∃l ≥ 1. π l (i, p0 ) = (j ′ , p′ )} ∪ πpi (j) and {j ′ ∈ [i + 1, n] | ai+1 · · · aj ′ ∈
r} with [j + 1, n]. Therefore, for each ξ(i), we can choose p ∈ P such that πp (j) = ξ(i). This defines
an injection ξ(i) 7→ p and hence the cardinality of P is at least C(w, r, j).
Below, we exhibit w, r and j such that C(w, r, j) is at least super-exponential in m.5 Let q ≥ 1
be an integer. Let w be the string over {0, 1, %, ¢, #, $} defined as follows:
q

q

w = #enc(0)#enc(1) · · · #enc(22 − 1)$enc(22 − 1)rev # · · · enc(1)rev #enc(0)rev #,
enc(i) = ¢%binq (0)%bin2q (i)[1] ¢%binq (1)%bin2q (i)[2]
· · · ¢%binq (2q − 1)%bin2q (i)[2q ]

q

for i ∈ [0, 22 − 1].

q

Here, for any integer i ∈ [0, 2q − 1] (resp. [0, 22 − 1]), we write binq (i) (resp. bin2q (i)) for the binary
representation of i of length q (resp. 2q ). Let j be the position of $ in w (i.e., w[j] = $). Let r be
the ERE defined as follows:
r = Σ∗ $ + #(req ∩ Σ∗ $Σ∗ )#,
req = ¬([01%¢]∗ ¢%req,q ⟨%(0Σ∗ 1 + 1Σ∗ 0)%⟩%¢[01%¢]∗ ),
req,l ⟨r′ ⟩ = Σreq,l−1 ⟨r′ ⟩Σ ∩ (0Σ∗ 0 + 1Σ∗ 1)
′

for l ∈ [1, q],

′

req,0 ⟨r ⟩ = r .
Observe that the length m of r is O(q). The ERE r satisfies the following properties:
1. For any string s ∈ Σ∗ , the ERE r matches s$.
q

2. For any substring s of w of the form #enc(i) s1 $s2 enc(i′ )rev # where i, i′ ∈ [0, 22 − 1] are
integers and s1 , s2 ∈ [01%¢#]∗ are strings, r matches s if and only if i = i′ .
Thus, the behavior of any jumping machine for w and r is as shown in Figure 2. Using this figure,
we prove the following theorem, which refutes the claim (¶).
4
The claim (¶) holds for a pure regular expression by taking P as the states of a deterministic finite automaton
equivalent to the pure regular expression.
5
In fact, by recursively applying the same technique of encoding word positions in binary, we can obtain w, r and
j such that C(w, r, j) is k-fold exponential in m for any fixed k. As mentioned in Sections 1.1.2 and 1.2, this encoding
technique was also used in the proofs of Lemma 19.

17

<!-- page 19 -->

q

q

w = #enc(0) #enc(1) · · · #enc(22 − 1) $ enc(22 − 1)rev # · · · enc(1)rev # enc(0)rev #
i1

i2

···

i′22q

j

i22q

(i1 , p0 )

i′2

···

(i′1 , p′1 )

(j, p1 )
(i2 , p0 )

(i′2 , p′2 )

(j, p2 )
..

..
.

.
(i22q , p0 )

(j, p22q )

i′1

··

·

(i′22q , p′22q )

Figure 2: Behavior of a jumping machine for w and r constructed earlier. The intermediate states p1 , . . . , p22q at
position j are pairwise distinct because so are the accepting positions i′1 , . . . , i′22q .
o(m)

, there are a string w and an ERE r of length m
Theorem 21. For any function f (m) = 22
such that any jumping machine for w and r must have at least f (m) states. Consequently, the
claim (¶) is false.
Proof. Let w, r and j be as constructed above. Let i1 , . . . , i22q , j, i′22q , . . . , i′1 be positions in w as
q
shown in Figure 2. By Property 2 above, for each l, l′ ∈ [1, 22 ], we have that i′l′ ∈ ξ(il ) if and only
q
if l = l′ . Hence, sets ξ(i1 ), . . . , ξ(i22q ) are pairwise distinct and therefore C(w, r, j) ≥ 22 . Since
Ω(m)
m = O(q), it follows that C(w, r, j) = 22
. Therefore, C(w, r, j) ≥ f (m) holds for sufficiently
large m. By Lemma 20 and Property 1 above, any jumping machine for w and r must have at least
f (m) states.

5

OV to Rewb Matching

In this section, we prove Theorems 5 and 6 by establishing the following Lemmas 22 and 23. Recall
the informal semantics of rewbs described in Section 1.1.3.
Lemma 22. Let k ≥ 1 be a constant. Suppose k-rewb matching can be solved in time T (n, m).
Then 2k-OV can be solved in time O(nd2 ) + T (nd2 , d2 ).
Proof. We construct a reduction from 2k-OV to k-rewb matching as follows. Given sets of vectors
A1 , . . . , A2k where Ai = {ai,j | 1 ≤ j ≤ n} for i ∈ [1, 2k], we encode them into a string w in the
following way:

 
 
k
n
n
K
K
K

 $
w = (s¢)d+1 where s =
a2i−1,j # $ 
#arev
2i,j
i=1

j=1

j=1

Both the length of w and the time required for this construction are O(nd2 ). We then define a
k-rewb r over the alphabet Σ = {0, 1, #, $, ¢} as follows:
!
k
K
r=
[01#]∗ (#[01#]∗ $[01#]∗ #)i [01#]∗ $ ¢rzero,1 ¢rzero,2 ¢ · · · ¢rzero,d ¢,
i=1

rzero,l = rzero,l,1 + rzero,l,2 + · · · + rzero,l,k for l ∈ [1, d],
rzero,l,i = ([01#]∗ $)2(i−1) [01#]∗ (0[01]l−1 \i + \i[01]l−1 0)[01#$]∗ for i ∈ [1, k].
18

<!-- page 20 -->

Both the length of r and the time required for this construction are O(d2 ). We claim that there exist
vectors a1 ∈ A1 , . . . , a2k ∈ A2k that are orthogonal if and only if r matches w (see Appendix A.8
for details). Therefore, the given 2k-OV instance can be solved in the claimed time by solving the
k-rewb matching instance.
Lemma 23. Suppose straight-line 1-rewb matching can be solved in time T (n, m). Then OV can
be solved in time O(nd2 ) + T (nd2 , d2 ).
Proof. We construct a reduction from OV to straight-line 1-rewb matching as follows. Given sets
of vectors A = {a1 , . . . , an } and B = {b1 , . . . , bn }, we encode them into a string w in the following
way:
rev
rev
w = (s¢)d+1 where s = a1 #a2 # · · · an #$#brev
1 #b2 · · · #bn .
Both the length of w and the time required for this construction are O(nd2 ). We then define a
straight-line 1-rewb r over the alphabet Σ = {0, 1, #, $, ¢} as follows:
r = [01#]∗ (#Σ∗ #)1 rzero,1 \1rzero,2 \1 · · · rzero,d \1[01#]∗ ¢,
rzero,l = [01]l−1 0[01#]∗ ¢[01#]∗ + [01#]∗ ¢[01#]∗ 0[01]l−1 for l ∈ [1, d].
Both the length of r and the time required for this construction are O(d2 ). We claim that there
exist vectors ai ∈ A and bj ∈ B that are orthogonal if and only if r matches w (see Appendix A.8
for details). Therefore, the given OV instance can be solved in the claimed time by solving the
straight-line 1-rewb matching instance.
Theorems 5 and 6 follow immediately from the above lemmas.

References
[AB18]

Amir Abboud and Karl Bringmann. Tighter connections between Formula-SAT and
shaving logs. In Ioannis Chatzigiannakis, Christos Kaklamanis, Dániel Marx, and Donald Sannella, editors, 45th International Colloquium on Automata, Languages, and Programming, ICALP 2018, Prague, Czech Republic, July 9-13, 2018, volume 107 of LIPIcs,
pages 8:1–8:18. Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2018.

[ABHS22] Amir Abboud, Karl Bringmann, Danny Hermelin, and Dvir Shabtay. Seth-based lower
bounds for subset sum and bicriteria path. ACM Trans. Algorithms, 18(1):6:1–6:22,
2022.
[ABV18]

Amir Abboud, Arturs Backurs, and Virginia Vassilevska Williams. If the current Clique
algorithms are optimal, so is Valiant’s parser. SIAM J. Comput., 47(6):2527–2555, 2018.

[ADV+ 25] Josh Alman, Ran Duan, Virginia Vassilevska Williams, Yinzhan Xu, Zixuan Xu, and
Renfei Zhou. More asymmetry yields faster matrix multiplication. In Yossi Azar and
Debmalya Panigrahi, editors, Proceedings of the 2025 Annual ACM-SIAM Symposium
on Discrete Algorithms, SODA 2025, New Orleans, LA, USA, January 12-15, 2025,
pages 2005–2039. SIAM, 2025.
[AFS24]

Amir Abboud, Nick Fischer, and Yarin Shechter. Faster combinatorial k-Clique algorithms. In José A. Soto and Andreas Wiese, editors, LATIN 2024: Theoretical Informatics - 16th Latin American Symposium, Puerto Varas, Chile, March 18-22, 2024,
Proceedings, Part I, volume 14578 of Lecture Notes in Computer Science, pages 193–206.
Springer, 2024.
19

<!-- page 21 -->

[Aho90]

Alfred V. Aho. Algorithms for finding patterns in strings. In Jan van Leeuwen, editor,
Handbook of Theoretical Computer Science, Volume A: Algorithms and Complexity,
pages 255–300. Elsevier and MIT Press, Cambridge, MA, USA, 1990.

[AVW16] Amir Abboud, Virginia Vassilevska Williams, and Joshua R. Wang. Approximation and
fixed parameter subquadratic algorithms for Radius and Diameter in sparse graphs. In
Robert Krauthgamer, editor, Proceedings of the Twenty-Seventh Annual ACM-SIAM
Symposium on Discrete Algorithms, SODA 2016, Arlington, VA, USA, January 10-12,
2016, pages 377–391. SIAM, 2016.
[BCFK22] Karl Bringmann, Alejandro Cassis, Nick Fischer, and Marvin Künnemann. A structural investigation of the approximability of polynomial-time problems. CoRR,
abs/2204.11681, 2022.
[BGJ25]

Philip Bille, Inge Li Gørtz, and Rikke Schjeldrup Jessen. Improved extended regular
expression matching. CoRR, abs/2510.09311v3, 2025.

[BGL17]

Karl Bringmann, Allan Grønlund, and Kasper Green Larsen. A dichotomy for regular
expression membership testing. In Chris Umans, editor, 58th IEEE Annual Symposium
on Foundations of Computer Science, FOCS 2017, Berkeley, CA, USA, October 15-17,
2017, pages 307–318. IEEE Computer Society, 2017.

[BI16]

Arturs Backurs and Piotr Indyk. Which regular expression patterns are hard to match?
In Irit Dinur, editor, IEEE 57th Annual Symposium on Foundations of Computer Science, FOCS 2016, 9-11 October 2016, Hyatt Regency, New Brunswick, New Jersey,
USA, pages 457–466. IEEE, IEEE Computer Society, 2016.

[BP24]

Aurèle Barrière and Clément Pit-Claudel. Linear matching of JavaScript regular expressions. Proc. ACM Program. Lang., 8(PLDI):1336–1360, 2024.

[CW19]

Lijie Chen and Ryan Williams. An equivalence class for orthogonal vectors. In Timothy M. Chan, editor, Proceedings of the Thirtieth Annual ACM-SIAM Symposium
on Discrete Algorithms, SODA 2019, San Diego, California, USA, January 6-9, 2019,
pages 21–40. SIAM, 2019.

[FS19]

Dominik D. Freydenberger and Markus L. Schmid. Deterministic regular expressions
with back-references. J. Comput. Syst. Sci., 105:1–39, 2019.

[Für78]

Martin Fürer. Nicht-elementare untere Schranken in der Automaten-Theorie. PhD
thesis, ETH Zürich, 1978. Written in German.

[GIKW19] Jiawei Gao, Russell Impagliazzo, Antonina Kolokolova, and Ryan Williams. Completeness for first-order properties on sparse structures with algorithmic applications. ACM
Trans. Algorithms, 15(2):23:1–23:35, 2019.
[Gre73]

Sheila A. Greibach. The hardest context-free language. SIAM J. Comput., 2(4):304–310,
1973.

[Har78]

Michael A. Harrison. Introduction to Formal Language Theory. Addison-Wesley, 1978.

[Hir89]

S. Hirst. A new algorithm solving membership of extended regular expressions. Tech.
Report, 1989.
20

<!-- page 22 -->

[HU79]

John E. Hopcroft and Jeffrey D. Ullman. Introduction to Automata Theory, Languages,
and Computation. Addison-Wesley, first edition, 1979.

[IP01]

Russell Impagliazzo and Ramamohan Paturi. On the complexity of k-SAT. J. Comput.
Syst. Sci., 62(2):367–375, 2001.

[ISY03]

Lucian Ilie, Baozhen Shan, and Sheng Yu. Fast algorithms for extended regular expression matching and searching. In Helmut Alt and Michel Habib, editors, STACS 2003,
20th Annual Symposium on Theoretical Aspects of Computer Science, Berlin, Germany,
February 27 - March 1, 2003, Proceedings, volume 2607 of Lecture Notes in Computer
Science, pages 179–190. Springer, 2003.

[Kem96]

Rainer Kemp.
On the average minimal prefix-length of the generalized semiDycklanguage. RAIRO Theor. Informatics Appl., 30(6):545–561, 1996.

[Kle56]

Stephen Cole Kleene. Representation of events in nerve nets and finite automata. In
Claude E. Shannon and John McCarthy, editors, Automata Studies, volume 34 of Annals
of Mathematics Studies, pages 3–41. Princeton University Press, 1956.

[KU26]

Soh Kumabe and Yuya Uezato. On the complexity of the matching problem of regular
expressions with backreferences. In Sayan Bhattacharya, Danupon Nanongkai, Michael
Benedikt, and Gabriele Puppis, editors, 53rd International Colloquium on Automata,
Languages, and Programming, ICALP 2026, Royal Holloway, University of London,
Egham, United Kingdom, July 7-10, 2026, volume 374 of LIPIcs, pages 135:1–135:25.
Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2026.

[KW19]

Daniel M. Kane and Richard Ryan Williams. The orthogonal vectors conjecture for
branching programs and formulas. In Avrim Blum, editor, 10th Innovations in Theoretical Computer Science Conference, ITCS 2019, San Diego, California, USA, January 10-12, 2019, volume 124 of LIPIcs, pages 48:1–48:15. Schloss Dagstuhl - LeibnizZentrum für Informatik, 2019.

[KZ02]

Orna Kupferman and Sharon Zuhovitzky. An improved algorithm for the membership problem for extended regular expressions. In Krzysztof Diks and Wojciech Rytter,
editors, Mathematical Foundations of Computer Science 2002, 27th International Symposium, MFCS 2002, Warsaw, Poland, August 26-30, 2002, Proceedings, volume 2420
of Lecture Notes in Computer Science, pages 446–458. Springer, 2002.

[Lie03]

Jens Liebehenschel. Lexicographical generation of a generalized Dyck language. SIAM
J. Comput., 32(4):880–903, 2003.

[MC24]

Konstantinos Mamouras and Agnishom Chattopadhyay. Efficient matching of regular
expressions with lookaround assertions. Proc. ACM Program. Lang., 8(POPL):2761–
2791, 2024.

[Mic21]

Microsoft.
Backreference
constructs
in
regular
expressions,
2021.
https://learn.microsoft.com/en-us/dotnet/standard/base-types/
backreference-constructs-in-regular-expressions (Accessed: 2026-08-31).

[MS72]

Albert R. Meyer and Larry J. Stockmeyer. The equivalence problem for regular expressions with squaring requires exponential space. In 13th Annual Symposium on Switching
and Automata Theory, College Park, Maryland, USA, October 25-27, 1972, pages 125–
129. IEEE Computer Society, 1972.
21

<!-- page 23 -->

[NP85]

Jaroslav Nešetřil and Svatopluk Poljak. On the complexity of the subgraph problem.
Commentationes Mathematicae Universitatis Carolinae, 26(2):415–419, 1985.

[NT25]

Taisei Nogami and Tachio Terauchi. Efficient matching of some fundamental regular
expressions with backreferences. In Pawel Gawrychowski, Filip Mazowiecki, and Michal
Skrzypczak, editors, 50th International Symposium on Mathematical Foundations of
Computer Science, MFCS 2025, Warsaw, Poland, August 25-29, 2025, volume 345 of
LIPIcs, pages 81:1–81:19. Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2025.

[Pet00]

Holger Petersen. Decision problems for generalized regular expressions. In Proceedings of
the 2nd International Workshop on Descriptional Complexity of Automata, Grammars
and Related Structures, London (Ontario), pages 22–29, 2000.

[Pet02]

Holger Petersen. The membership problem for regular expressions with intersection
is complete in LOGCFL. In Helmut Alt and Afonso Ferreira, editors, STACS 2002,
19th Annual Symposium on Theoretical Aspects of Computer Science, Antibes - Juan
les Pins, France, March 14-16, 2002, Proceedings, volume 2285 of Lecture Notes in
Computer Science, pages 513–522. Springer, 2002.

[Ros05]

Grigore Rosu. An effective algorithm for the membership problem for extended regular
expressions. Technical Report Technical Report UIUCDCS-R-2005-2964, Department
of Computer Science, University of Illinois at Urbana-Champaign, 2005.

[Ros07]

Grigore Rosu. An effective algorithm for the membership problem for extended regular
expressions. In Helmut Seidl, editor, Foundations of Software Science and Computational Structures, 10th International Conference, FOSSACS 2007, Held as Part of the
Joint European Conferences on Theory and Practice of Software, ETAPS 2007, Braga,
Portugal, March 24-April 1, 2007, Proceedings, volume 4423 of Lecture Notes in Computer Science, pages 332–345. Springer, 2007.

[Sch16]

Markus L. Schmid. Characterising REGEX languages by regular languages equipped
with factor-referencing. Inf. Comput., 249:1–17, 2016.

[Sch20]

Philipp Schepper. Fine-grained complexity of regular expression pattern matching and
membership. In Fabrizio Grandoni, Grzegorz Herman, and Peter Sanders, editors, 28th
Annual European Symposium on Algorithms, ESA 2020, Pisa, Italy (Virtual Conference), September 7-9, 2020, volume 173 of LIPIcs, pages 80:1–80:20. Schloss Dagstuhl
- Leibniz-Zentrum für Informatik, 2020.

[Sch24]

Markus L. Schmid. Regular expressions with backreferences: Polynomial-time matching
techniques. J. Autom. Lang. Comb., 29(2-4):321–357, 2024.

[SM73]

Larry J. Stockmeyer and Albert R. Meyer. Word problems requiring exponential time:
Preliminary report. In Alfred V. Aho, Allan Borodin, Robert L. Constable, Robert W.
Floyd, Michael A. Harrison, Richard M. Karp, and H. Raymond Strong, editors, Proceedings of the 5th Annual ACM Symposium on Theory of Computing, April 30 - May
2, 1973, Austin, Texas, USA, pages 1–9. ACM, 1973.

[Tho68]

Ken Thompson. Regular expression search algorithm. Commun. ACM, 11(6):419–422,
1968.

22

<!-- page 24 -->

[Uez26]

Yuya Uezato. Matching regular-typed pattern languages: Quadratic-time algorithms.
In Philip Bille and Nicola Prezza, editors, 37th Annual Symposium on Combinatorial
Pattern Matching, CPM 2026, Copenhagen, Denmark, June 15-17, 2026, volume 369
of LIPIcs, pages 11:1–11:20. Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2026.

[Vol99]

Heribert Vollmer. Introduction to Circuit Complexity - A Uniform Approach. Texts in
Theoretical Computer Science. An EATCS Series. Springer, 1999.

[Wil05]

Ryan Williams. A new algorithm for optimal 2-constraint satisfaction and its implications. Theor. Comput. Sci., 348(2-3):357–365, 2005.

[Wil18]

Virginia Vassilevska Williams. On some fine-grained questions in algorithms and complexity. In Proceedings of the international congress of mathematicians (ICM), pages
3447–3487. World Scientific, 2018.

[Wil24]

Ryan Williams. The Orthogonal Vectors Conjecture and non-uniform circuit lower
bounds. In 65th IEEE Annual Symposium on Foundations of Computer Science, FOCS
2024, Chicago, IL, USA, October 27-30, 2024, pages 1372–1387. IEEE, 2024.

[WY14]

Ryan Williams and Huacheng Yu. Finding orthogonal vectors in discrete structures. In
Chandra Chekuri, editor, Proceedings of the Twenty-Fifth Annual ACM-SIAM Symposium on Discrete Algorithms, SODA 2014, Portland, Oregon, USA, January 5-7, 2014,
pages 1867–1877. SIAM, 2014.

[Yam00]

Hiroaki Yamamoto. An automata-based recognition algorithm for semi-extended regular
expressions. In Mogens Nielsen and Branislav Rovan, editors, Mathematical Foundations
of Computer Science 2000, 25th International Symposium, MFCS 2000, Bratislava,
Slovakia, August 28 - September 1, 2000, Proceedings, volume 1893 of Lecture Notes in
Computer Science, pages 699–708. Springer, 2000.

A

Detailed Proofs

A.1

Proof preliminaries

We first introduce a useful function (cf. [Har78]). In what follows, R denotes a binary relation on
Γ, x, y denote vectors in Vβ (Γ) and u, v, w denote vector strings in (Vβ (Γ) ∪ Vβ (Γ̄))∗ . We also often
omit the subscript β, writing V (Γ), V (Γ̄) and π for Vβ (Γ), Vβ (Γ̄) and πβ , respectively.
Definition 24. Define a map µR : (V (Γ) ∪ V (Γ̄))∗ → (V (Γ) ∪ V (Γ̄))∗ by
µR (λ) = λ,
µR (wx) = µ(w)x,
(
w′
µR (wȳ) =
µ(w)ȳ

if µ(w) = w′ x for some w′ and x ∈ V (Γ) with xRy,
otherwise.

When R is clear from the context, we omit the subscript and write µ.
We will use the following properties of µ.
Lemma 25 (cf. [Har78, Propositions 10.4.1 and 10.4.2]). The following hold:

23

<!-- page 25 -->

(a) µ(µ(u)) = µ(u).
(b) µ(uv) = µ(µ(u)v) = µ(µ(u)µ(v)).
(c) If µ(wȳ) ∈ V (Γ)∗ , then µ(w) = µ(wȳ)x for some x ∈ V (Γ) with xRy.
(d) If u is a prefix of a string in µ−1 (λ), then µ(u) ∈ V (Γ)∗ .
Proof. The proofs of (a) and µ(uv) = µ(µ(u)v) are analogous to those in [Har78, Proposition
10.4.1(e) and (f)] and are omitted. We prove µ(uµ(v)) = µ(uv) by induction on |v|. The base case
v = λ is immediate. If v = v ′ x for some x ∈ V (Γ), then
µ(uµ(v)) = µ(uµ(v ′ )x) = µ(uµ(v ′ ))x = µ(uv ′ )x = µ(uv ′ x) = µ(uv).
Suppose v = v ′ ȳ for some y ∈ V (Γ). We distinguish two cases. If µ(v ′ ) = v ′′ x for some x ∈ V (Γ)
with xRy, then µ(v) = v ′′ . So
µ(uv ′ ) = µ(uµ(v ′ )) = µ(uv ′′ x) = µ(uv ′′ )x = µ(uµ(v))x
and hence µ(uv) = µ(uv ′ ȳ) = µ(uµ(v)). Otherwise, we have µ(v) = µ(v ′ )ȳ. So
µ(uµ(v)) = µ(uµ(v ′ )ȳ) = µ(µ(uµ(v ′ ))ȳ) = µ(µ(uv ′ )ȳ) = µ(uv ′ ȳ) = µ(uv),
completing the induction. Therefore, µ(µ(u)µ(v)) = µ(µ(u)v) = µ(uv). Part (c) follows directly
from the definition of µ. For (d), we prove by induction on |v| that µ(uv) = λ ⇒ µ(u) ∈ V (Γ)∗ for
all u. The base case v = λ is immediate. If v = xv ′ for some x ∈ V (Γ), then µ(ux) = µ(u)x ∈ V (Γ)∗
and hence µ(u) ∈ V (Γ)∗ . If v = ȳv ′ for some y ∈ V (Γ), then µ(uȳ) ∈ V (Γ)∗ . By (c), µ(u) = µ(uȳ)x
for some x ∈ V (Γ), and hence µ(u) ∈ V (Γ)∗ .
Lemma 26. µ(u) = µ(v) ⇐⇒ u ∼ v. In particular, DR = µ−1 (λ).
Proof. We write u ∼µ v if µ(u) = µ(v). Note that ∼µ is a congruence containing S = {(xȳ, λ) |
x, y ∈ V (Γ), xRy}. Indeed, µ(xȳ) = λ, and if µ(u) = µ(v), then for every w1 , w2 , we have
µ(w1 uw2 ) = µ(µ(w1 )µ(u)µ(w2 )) = µ(µ(w1 )µ(v)µ(w2 )) = µ(w1 vw2 ). Hence, ∼ ⊆ ∼µ . For the
converse, we show u ∼ µ(u) by induction on |u|. The base case u = λ is immediate. If u = u′ x
for some u′ and x ∈ V (Γ), we have µ(u) = µ(u′ )x ∼ u′ x = u. If u = u′ ȳ for some y ∈ V (Γ), we
distinguish two cases. If µ(u′ ) = u′′ x for some x ∈ V (Γ) with xRy, then µ(u) = u′′ ∼ u′′ xȳ =
µ(u′ )ȳ ∼ u′ ȳ = u. Otherwise, we have µ(u) = µ(u′ )ȳ ∼ u′ ȳ = u. Therefore, if µ(u) = µ(v), then
u ∼ µ(u) = µ(v) ∼ v, and hence ∼µ ⊆ ∼. This completes the proof.
Lemma 27 (inversion). Every nonempty w ∈ DR can be decomposed as w = w1 xw2 ȳ with w1 , w2 ∈
DR and xRy.
Proof. We prove the claim that if µ(w) = x1 · · · xℓ for some ℓ ≥ 0 and x1 , . . . , xℓ ∈ V (Γ), then
w = w1 x1 · · · wℓ xℓ wℓ+1 for some w1 , . . . , wℓ+1 ∈ DR , by induction on |w|. The base case w = λ
is immediate. If w = w′ x for some w′ and x ∈ V (Γ), we have µ(w) = µ(w′ )x = x1 · · · xℓ and
hence obtain x = xℓ and µ(w′ ) = x1 · · · xℓ−1 . By the induction hypothesis, w′ can be written as
w1 x1 · · · wℓ−1 xℓ−1 wℓ for some w1 , . . . , wℓ ∈ DR . So w = w′ x = w1 x1 · · · wℓ−1 xℓ−1 wℓ xℓ wℓ+1 with
wℓ+1 = λ, and the claim holds. If w = w′ ȳ for some y ∈ V (Γ), by Lemma 25(c), we have µ(w′ ) =
µ(w)x = x1 · · · xℓ x for some x with xRy. By the induction hypothesis, w′ = w1 x1 · · · wℓ xℓ wℓ+1 xwℓ+2
for some w1 , . . . , wℓ+2 ∈ DR and hence w = w1 x1 · · · wℓ xℓ wℓ+1 xwℓ+2 ȳ. Because µ(wℓ+1 xwℓ+2 ȳ) = λ,
the claim also holds. This completes the proof of the claim.
Because µ(w) = λ and w is nonempty, w is of the form w = w′ ȳ. By Lemma 25(c) and the
claim above, µ(w′ ) = x for some x with xRy and w′ = w1 xw2 for some w1 , w2 ∈ DR . Therefore,
w = w1 xw2 ȳ, which completes the proof.
24

<!-- page 26 -->

Corollary 28. DR ⊆ π −1 (D1 ).
Proof. By a straightforward induction on |w| using Lemma 27.

A.2

Proof of Lemma 8

Proof of Lemma 8. We prove that w ∈ D2 ⇐⇒ h(w) ∈ J2 by induction on |w|. The base case
w = λ is immediate, so assume w is nonempty. Suppose that w ∈ D2 . By the inversion lemma
(Lemma 27), w can be written as w = w1 xw2 x̄ for some w1 , w2 ∈ D2 and x ∈ Vβ (Γ). By the induction hypothesis, µRorth (h(w)) = µRorth (h(x)h(x̄)) = λ and hence h(w) ∈ J2 . Conversely, suppose
that h(w) ∈ J2 . Again, by the inversion lemma, h(w) can be written as h(w) = h(w1 )h(x)h(w2 )h(ȳ)
for some w1 , w2 with h(w1 ), h(w2 ) ∈ J2 and some x, y ∈ Vβ (Γ) with h(x) Rorth h(ȳ).6 It follows
easily that x = y. By the induction hypothesis, µReq (w) = µReq (xȳ) = λ and hence w ∈ D2 .

A.3

Proof of Lemma 11

Lemma 29. Let R be a binary relation on Γ and u, v, w, x, y ∈ (V (Γ) ∪ V (Γ̄))∗ .
(a) If µ(v) = λ, then µ(uvw) = µ(uw).
(b) If µ(uxȳv) = λ, then xRy and µ(uv) = λ.
Proof. Immediate from Lemma 25.
Proof of Lemma 11. We show that G has a 3k-clique if and only if the Dyck Selection instance on
S is a yes-instance. Suppose G has a 3k-clique, or equivalently, three disjoint k-cliques t1 , t2 , t3 that
are fully connected with each other. Let vi,j = ti [j] for i ∈ [1, 3] and j ∈ [1, k]. We can form
k
rev
k rev
s = VG(t1 )VG(t1 )t2 (t̄rev
2 v̄1,k t2 ) · · · (t̄2 v̄1,1 t2 ) t̄2
k
rev
k rev
k
rev
k rev
VG(t2 )t3 (t̄rev
3 v̄2,k t3 ) · · · (t̄3 v̄2,1 t3 ) (t̄3 v̄1,k t3 ) · · · (t̄3 v̄1,1 t3 ) t̄3

by choosing one string from each set of S. By Lemma 29(a),


µ(s) = µ VG(t1 )VG(t1 )(v̄1,k )k · · · (v̄1,1 )k VG(t2 )(v̄2,k )k · · · (v̄2,1 )k (v̄1,k )k · · · (v̄1,1 )k = λ
and therefore s ∈ D2 . Conversely, suppose that some s ∈ D2 is obtained by choosing one string
from each set in S. Write
s = VG(t1 )VG(t1 )
t2
rev
rev
rev
(t̄rev
3,1,1 ū3,1,1 t3,1,1 · · · t̄3,1,k ū3,1,k t3,1,k ) · · · (t̄3,k,1 ū3,k,1 t3,k,1 · · · t̄3,k,k ū3,k,k t3,k,k )

t̄rev
4 VG(t4 )
t5
rev
rev
rev
(t̄rev
6,1,1 ū6,1,1 t6,1,1 · · · t̄6,1,k ū6,1,k t6,1,k ) · · · (t̄6,k,1 ū6,k,1 t6,k,1 · · · t̄6,k,k ū6,k,k t6,k,k )
rev
rev
rev
(t̄rev
7,1,1 ū7,1,1 t7,1,1 · · · t̄7,1,k ū7,1,k t7,1,k ) · · · (t̄7,k,1 ū7,k,1 t7,k,1 · · · t̄7,k,k ū7,k,k t7,k,k )

t̄rev
8
6

Here, applying the bar twice cancels out.

25

<!-- page 27 -->

for some k-cliques t1 , t2 , t3,i,j , t4 , t5 , t6,i,j , t7,i,j , t8 and vertices u3,i,j , u6,i,j , u7,i,j , where i, j ∈ [1, k].
By Lemma 29(b) and because µ(s) = λ, we have t2 = t3,i,j = t4 and t5 = t6,i,j = t7,i,j = t8 for all
i, j ∈ [1, k]. We also have


µ VG(t1 )VG(t1 )ū3,1,1 · · · ū3,k,k VG(t2 )ū6,1,1 · · · ū6,k,k ū7,1,1 · · · ū7,k,k = λ.
Applying Lemma 29(b) again, every vertex of t1 is adjacent to every vertex of t2 . Similarly, t2 and
t5 are fully connected. Then


µ VG(t1 )ū7,1,1 · · · ū7,k,k = λ
and thus t1 and t5 are also fully connected. Therefore, t1 ∪ t2 ∪ t5 is a 3k-clique of G.

A.4

Proofs of Lemma 13 and Corollary 14

Proof of Lemma 13. We largely follow the argument in the textbook [Har78, Theorem 10.5.1]. Let
Σ be an alphabet and L ⊆ Σ∗ a context-free language. We construct a reduction from L to Dyck
Selection with β = 1. Because the empty input can be handled separately, we may assume that
λ∈
/ L.
Fix a context-free grammar G generating L. We may assume that G is in Greibach normal
form, that is, every production rule is of the form A → aξ where ξ is a (possibly empty) sequence of
nonterminals not containing the start symbol S. Enumerate the nonterminals of G as A1 , . . . , Am
with A1 = S. For each production rule p : Ai → aAj1 · · · Ajℓ , let τ (p) = ᾱ1 ᾱ2i ᾱ1 α1 α2jℓ α1 · · · α1 α2j1 α1 .
Given a nonempty string w ∈ Σ∗ , the reduction outputs a Dyck Selection instance Sw = S0 , . . . , S|w|
with S0 = {α1 α2 α1 } and Si = {τ (p) | p ∈ Pw[i] } for each 1 ≤ i ≤ |w|, where Pw[i] is the set of
production rules whose right-hand side starts with w[i]. This can be done in constant space.
It remains to show that w ∈ L if and only if Sw is a yes-instance. Let Q be the set of all
(possibly empty) sequences of nonterminals not containing S, and for each ξ = Ai1 · · · Aiℓ ∈ Q,
define enc(ξ) = α1 α2iℓ α1 · · · α1 α2i1 α1 . We write ⇒ for the one-step derivation relation of G and ⇒∗
for its reflexive transitive closure.
Claim 30. Let k ≥ 0 and a1 , . . . , ak ∈ Σ, and let ξ ∈ Q. Then S ⇒∗ a1 · · · ak ξ if and only if there
exist si ∈ Si for i ∈ [0, k] such that µ(s0 · · · sk ) = enc(ξ).
Proof. We proceed by induction on k. The base case k = 0 is clear. For k ≥ 1, we have
S ⇒∗ a1 · · · ak ξ
⇐⇒ ∃ Ai → ak ξi ∈ Pak , ∃ξj ∈ Q :
S ⇒∗ a1 · · · ak−1 Ai ξj and ξ = ξi ξj
⇐⇒ ∃ Ai → ak ξi ∈ Pak , ∃ξj ∈ Q, ∃sl ∈ Sl for l ∈ [0, k − 1] :
µ(s0 · · · sk−1 ) = enc(Ai ξj ) and ξ = ξi ξj
⇐⇒ ∃sl ∈ Sl for l ∈ [0, k] : µ(s0 · · · sk ) = enc(ξ),
as required.
Setting k = n and ξ = λ completes the proof.
Next, we prove Corollary 14. We denote by µ1 : (Γ ∪ Γ̄)∗ → (Γ ∪ Γ̄)∗ the function µ for β = 1.
Let ϕ : (Vβ (Γ) ∪ Vβ (Γ̄))∗ → (Γβ ∪ Γ̄β )∗ be the homomorphism defined in the proof of Lemma 15.

26

<!-- page 28 -->

Lemma 31. For every w ∈ (Vβ (Γ)∪Vβ (Γ̄))∗ , µ1 (ϕ(µ(w))) = µ1 (ϕ(w)). In particular, µ1 (ϕ(w)) = λ
if µ(w) = λ.
Proof. We proceed by induction on |w|. The base case w = λ is immediate. If w = w′ x for some
x ∈ Vβ (Γ), then
µ1 (ϕ(µ(w))) = µ1 (ϕ(µ(w′ )x)) = µ1 (ϕ(µ(w′ )))ϕ(x) = µ1 (ϕ(w′ ))ϕ(x) = µ1 (ϕ(w′ x)) = µ1 (ϕ(w)).
Suppose w = w′ ȳ for some y ∈ V (Γ). We distinguish two cases. If µ(w′ ) = w′′ x for some x ∈ V (Γ)
with xRy, then µ(w) = w′′ . So
µ1 (ϕ(w)) = µ1 (ϕ(w′ )ϕ(ȳ)) = µ1 (µ1 (ϕ(w′ ))ϕ(ȳ)) = µ1 (µ1 (ϕ(µ(w′ )))ϕ(ȳ))
= µ1 (ϕ(w′′ x)ϕ(ȳ)) = µ1 (ϕ(w′′ )) = µ1 (ϕ(µ(w))).
Otherwise, we have µ(w) = µ(w′ )ȳ. So
µ1 (ϕ(µ(w))) = µ1 (ϕ(µ(w′ ))ϕ(ȳ)) = µ1 (µ1 (ϕ(µ(w′ )))ϕ(ȳ))
= µ1 (µ1 (ϕ(w′ ))ϕ(ȳ)) = µ1 (ϕ(w′ )ϕ(ȳ)) = µ1 (ϕ(w)).
This completes the proof.
Lemma 32. For every w ∈ (Vβ (Γ) ∪ Vβ (Γ̄))∗ and v ∈ Vβ (Γ)∗ , if µ1 (ϕ(w)) = ϕ(v), then µ(w) = v.
In particular, µ(w) = λ if µ1 (ϕ(w)) = λ.
Proof. We proceed by induction on |w|. The base case w = λ is immediate. If w = w′ x for some
x ∈ Vβ (Γ), we have µ1 (ϕ(w)) = µ1 (ϕ(w′ ))ϕ(x) = ϕ(v). By the definition of ϕ, there is v ′ such that
v ′ x = v and ϕ(v ′ ) = µ1 (ϕ(w′ )). By the induction hypothesis, we obtain µ(w′ ) = v ′ , and hence
µ(w) = µ(w′ )x = v ′ x = v.
If w = w′ ȳ for some y ∈ V (Γ), we have µ1 (ϕ(w)) = µ1 (ϕ(w′ )ϕ(ȳ)) = ϕ(v). By repeatedly
applying Lemma 25(c), we have µ1 (ϕ(w′ )) = ϕ(v)ϕ(x) for some x ∈ Vβ (Γ) with xRy. By the
induction hypothesis, we obtain µ(w′ ) = vx, and hence µ(w) = µ(w′ ȳ) = µ(vxȳ) = µ(v) = v. This
completes the proof.
Lemma 33. L(GR ) = µ−1
1 (λ).
Proof. By a straightforward induction using Lemma 27.
Corollary 34. For every w ∈ (Vβ (Γ) ∪ Vβ (Γ̄))∗ , w ∈ DR if and only if ϕ(w) ∈ L(GR ).
Proof. Immediate from the three lemmas above and Lemma 26.
Proof of Corollary 14. We prove that GDS with restriction set D1 is in LOGCFL by reducing it
to a context-free language. Let GR be the context-free grammar with the single nonterminal S
and production rules S → λ and S → SaS b̄ for every a, b ∈ Γ satisfying aRb, and L(GR ) be the
language generated by GR . Given a GDS instance S = S1 , . . . , Sn , we encode it as



n
K
K

wS =
#ϕ(s) #$ .
i=1

s∈Si

27

<!-- page 29 -->

The string wS can be computed in O(log β) space. We then define the following language LS over
Σ = Γ ∪ Γ̄ ∪ {#, $}:


ℓ ≥ 0,






∗


ui , vi ∈ (Σ \ {$}) ,
LS = u1 #ϕ(t1 )#v1 $ · · · uℓ #ϕ(tℓ )#vℓ $
.

ti ∈ (Vβ (Γ) ∪ Vβ (Γ̄))∗ , 






ϕ(t1 ) · · · ϕ(tℓ ) ∈ L(GR )
A pushdown automaton for LS can be constructed from one for L(GR ) using O(log β) space to
generate a sequence of β states. Hence, LS is context-free. By Corollary 34, we have
S is a yes-instance ⇐⇒ ∃si ∈ Si for i ∈ [1, n] : ϕ(s1 ) · · · ϕ(sn ) ∈ L(GR ) ⇐⇒ wS ∈ LS ,
and therefore GDS with restriction set D1 is log-space reducible to LS .

A.5

Proof of Lemma 15

Lemma 35. (a) Every w ∈ DR can be decomposed as w = x1 w1 ȳ1 · · · xℓ wℓ ȳℓ with ℓ ≥ 0, xj , yj ∈
V (Γ) satisfying xj Ryj and wj ∈ DR for j ∈ [1, ℓ].
(b) If α1 w1 ᾱ1 · · · α1 wℓ ᾱ1 = α1 w1′ ᾱ1 · · · α1 wℓ′ ′ ᾱ1 for ℓ, ℓ′ ≥ 0 and w1 , . . . , wℓ , w1′ , . . . , wℓ′ ′ ∈ D1 ,
then ℓ = ℓ′ and w1 = w1′ , . . . , wℓ = wℓ′ .
Proof. A straightforward induction on |w|, using Lemma 27, proves (a). For (b), we proceed by
induction on ℓ. The base case ℓ = 0 is trivial. Suppose ℓ ≥ 1 (and hence ℓ′ ≥ 1). If w1 = w1′ ,
the claim follows immediately from the induction hypothesis. Otherwise, assume without loss of
generality that w1 ᾱ1 is a prefix of w1′ . By Lemma 25, µ(w1 ᾱ1 ) = µ(µ(w1 )ᾱ1 ) = µ(ᾱ1 ) = ᾱ1 ∈ Γ∗1 ,
a contradiction.
Corollary 36. Let δ ≥ 1. Every w ∈ D1 can be uniquely decomposed as w = α1 w1 ᾱ1 · · · α1 wℓ ᾱ1
(δ)
(δ−1)
with ℓ ≥ 0 and w1 , . . . , wℓ ∈ D1 . Moreover, if w ∈ D1 , then w1 , . . . , wℓ ∈ D1
.
(δ)

Lemma 37 (decomposition). Let δ ≥ 1. Every w ∈ π −1 (D1 ) ∩ DR can be decomposed as w =
(δ−1)
x1 w1 ȳ1 · · · xℓ wℓ ȳℓ with ℓ ≥ 0, xj , yj ∈ V (Γ) satisfying xj Ryj and wj ∈ π −1 (D1
) ∩ DR for
j ∈ [1, ℓ].
Proof. Let w = x1 w1 ȳ1 · · · xℓ wℓ ȳℓ be a decomposition given by Lemma 35(a). Applying π, we
obtain π(w) = α1 π(w1 )ᾱ1 · · · α1 π(wℓ )ᾱ1 . By Corollary 28, this gives the decomposition of π(w) in
(δ)
(δ−1)
Corollary 36. Thus, if π(w) ∈ D1 , then π(w1 ), . . . , π(wℓ ) ∈ D1
, as required.
Proof of Lemma 15. We prove that the given Generalized Dyck Selection instance S1 , . . . , Sn is a
yes-instance if and only if r matches w. We first prove the following two characterizations.
Claim 38. Let w′ ∈ Σ∗ and r′ a semi-ERE. For every j ∈ [0, β], rzero,j ⟨r′ ⟩ matches w′ if and only
if w′ = ϕ(x)uϕ(ȳ) for some u ∈ r′ and x, y ∈ Vj (Γ) with xRy.
Proof. We proceed by induction on j. The base case j = 0 is immediate. For j ≥ 1, we have
w′ ∈ rzero,j ⟨r′ ⟩
⇐⇒ ∃a, b ∈ Γ with aRb : w′ ∈ arzero,j−1 ⟨r′ ⟩b̄
⇐⇒ ∃u ∈ r′ , ∃x′ , y ′ ∈ Vj−1 (Γ) with x′ Ry ′ , ∃a, b ∈ Γ with aRb :
w′ = aϕ(x′ )uϕ(ȳ ′ )b̄
⇐⇒ ∃u ∈ r′ , ∃x, y ∈ Vj (Γ) with xRy : w′ = ϕ(x)uϕ(ȳ),
28

<!-- page 30 -->

completing the proof.
For w′ ∈ Σ∗ and v = v1 · · · vℓ ∈ (V (Γ) ∪ V (Γ̄))∗ (ℓ ≥ 0, v1 , . . . , vℓ ∈ V (Γ) ∪ V (Γ̄)), we write
′
w ⪰ v (or v ⪯ w′ ) if w′ ∈ r0 ϕ(v1 ) · · · r0 ϕ(vℓ )r0 .
Claim 39. Let w′ ∈ Σ∗ . For every i ∈ [0, δ], ri matches w′ if and only if there exists v ⪯ w′ such
(i)
that v ∈ πβ−1 (D1 ) ∩ DR .
Proof. We proceed by induction on i. The base case i = 0 clearly holds. For i ≥ 1, by applying
Lemma 37, the induction hypothesis and Claim 38 in this order, we have
(i)

∃v ∈ πβ−1 (D1 ) ∩ DR : w′ ⪰ v
(i−1)

⇐⇒ ∃ℓ ≥ 0, ∃xj , yj ∈ Vβ (Γ) with xj Ryj , ∃vj ∈ πβ−1 (D1

) ∩ DR for j ∈ [1, ℓ] :

′

w ⪰ x1 v1 ȳ1 x2 v2 ȳ2 · · · xℓ vℓ ȳℓ
⇐⇒ ∃ℓ ≥ 0, ∃xj , yj ∈ Vβ (Γ) with xj Ryj , ∃uj ∈ ri−1 , ∃z1 , . . . , zℓ+1 ∈ r0 for j ∈ [1, ℓ] :
w′ = z1 ϕ(x1 )u1 ϕ(ȳ1 ) z2 ϕ(x2 )u2 ϕ(ȳ2 ) · · · zℓ ϕ(xℓ )uℓ ϕ(ȳℓ ) zℓ+1
⇐⇒ w′ ∈ ri ,
completing the proof.
The desired equivalence now follows from Claim 39 and the construction of w.

A.6

Proof of Lemma 16

Proof of Lemma 16. We prove that the given Orthogonal Dyck Selection instance S1 , . . . , Sn is a
yes-instance if and only if r matches w. We call a string w′ ∈ Σ∗ good if it contains no string in
¢1 1∗ ¢̄1 or ¢̄2 1∗ ¢2 as a substring. Observe that w and all its substrings are good.
Claim 40. Let w′ ∈ Σ∗ be good and r′ an RSQ. For every j ∈ [0, β], rzero,j ⟨r′ ⟩ matches w′ if and
only if w′ = enc(ϕ(x)) u enc(ϕ(ȳ)) for some u ∈ r′ and orthogonal x, y ∈ Vj (Γ).
Proof. We proceed by induction on j. The base case j = 0 is clear. For j ≥ 1, we have
w′ ∈ rzero,j ⟨r′ ⟩
⇐⇒ w′ ∈ ¢1 (1¢2 rzero,j−1 ⟨r′ ⟩ ¢̄2 1 + 1¢2 rzero,j−1 ⟨r′ ⟩ ¢̄2 11 + 11¢2 rzero,j−1 ⟨r′ ⟩ ¢̄2 1)¢̄1
⇐⇒ w′ ∈ enc(a) rzero,j−1 ⟨r′ ⟩ enc(b̄) for some a, b ∈ {0, 1} with a · b = 0
⇐⇒ w′ = enc(ϕ(x)) u enc(ϕ(ȳ)) for some u ∈ r′ and orthogonal x, y ∈ Vj (Γ),
where the first and third equivalences use the fact that w′ and all its substrings are good.
For w′ ∈ Σ∗ and v = v1 · · · vℓ ∈ (V (Γ) ∪ V (Γ̄))∗ (ℓ ≥ 0, v1 , . . . , vℓ ∈ V (Γ) ∪ V (Γ̄)), we write
′
w ⪰enc v (or v ⪯enc w′ ) if w′ ∈ r0 enc(ϕ(v1 )) · · · r0 enc(ϕ(vℓ )) r0 .
Claim 41. Let w′ ∈ Σ∗ be good. For every i ∈ [0, δ], ri matches w′ if and only if there exists
(i)
v ⪯enc w′ such that v ∈ πβ−1 (D1 ) ∩ J2 .

29

<!-- page 31 -->

Proof. The proof is similar to that of Claim 39. We proceed by induction on i. The base case i = 0
clearly holds. For i ≥ 1, by applying Lemma 37, the induction hypothesis and Claim 40 in this
order, we have
(i)

∃v ∈ πβ−1 (D1 ) ∩ J2 : w′ ⪰enc v
(i−1)

⇐⇒ ∃ℓ ≥ 0, ∃ orthogonal xj , yj ∈ Vβ (Γ), ∃vj ∈ πβ−1 (D1

) ∩ J2 for j ∈ [1, ℓ] :

w′ ⪰enc x1 v1 ȳ1 x2 v2 ȳ2 · · · xℓ vℓ ȳℓ
⇐⇒ ∃ℓ ≥ 0, ∃ orthogonal xj , yj ∈ Vβ (Γ), ∃uj ∈ ri−1 , ∃z1 , . . . , zℓ+1 ∈ r0 for j ∈ [1, ℓ] :
w′ = z1 enc(ϕ(x1 )) u1 enc(ϕ(ȳ1 )) z2 · · · zℓ enc(ϕ(xℓ )) uℓ enc(ϕ(ȳℓ )) zℓ+1
⇐⇒ w′ ∈ ri ,
completing the proof.
The desired equivalence now follows from Claim 41 and the construction of w.

A.7

Proof of Lemma 19

We claim the following extension of the decomposition lemma (Lemma 37), which keeps track of
the parameter γ bounding the degree. The proof is analogous and is omitted.
(δ,γ)

Lemma 42. Let δ, γ ≥ 1. Every w ∈ π −1 (D1 ) ∩ DR can be decomposed as w = x1 w1 ȳ1 · · · xℓ wℓ ȳℓ
(δ−1,γ)
with ℓ ∈ [0, γ], xj Ryj and wj ∈ π −1 (D1
) ∩ DR for j ∈ [1, ℓ].
Proof of Lemma 19. We prove that the given Generalized Dyck Selection instance S1 , . . . , Sn is a
yes-instance if and only if r matches w. For i ∈ [0, h], we call a string w′ ∈ Σ∗ i-good if it can be
written as w′ = enci (ϕ(x)) u enci (ϕ(ȳ)) for some u ∈ Σ∗ and x, y ∈ Vℓ(h−i) (β) (Γ), in which case we
write left(w′ ) = x, mid(w′ ) = u and right(w′ ) = y, and, for j, k ∈ [1, ℓ(h−i) (β)], let w′ ⟨j, k⟩ denote
the substring
rev
′
rev ′
xj ¢1,i xj+1 · · · ¢1,i xℓ(h−i) (β) %i u %′i ȳℓrev
(h−i) (β) ¢1,i · · · ȳk+1 ¢1,i ȳk ,

where zl = enci−1 (binℓ(h−i+1) (β) (l)) ¢2,i z[l] for z ∈ {x, y} and l ∈ [1, ℓ(h−i) (β)]. Observe that w′ ⟨j, k⟩
is (i − 1)-good for all j, k.
Claim 43. Let i ∈ [0, h − 1]. Let w′ ∈ Σ∗ be i-good. Then req,ℓ(h−i) (β) matches w′ if and only if
left(w′ ) = right(w′ ).
Proof. We proceed by induction on i. The base case i = 0 is clear. For i ≥ 1, we have
w′ ∈ req,ℓ(h−i) (β)
⇐⇒ ∀j, k ∈ [1, ℓ(h−i) (β)] :
w′ ⟨j, k⟩ ∈
/ req,ℓ(h−i+1) (β) ∩ rexcl,¢2,i ¢2,i ¬req ¢′2,i rexcl,¢′2,i
⇐⇒ ∀j, k ∈ [1, ℓ(h−i) (β)] : binℓ(h−i+1) (β) (j) = binℓ(h−i+1) (β) (k) ⇒
w′ ⟨j, k⟩ ∈
/ rexcl,¢2,i ¢2,i ¬req ¢′2,i rexcl,¢′2,i
⇐⇒ ∀j ∈ [1, ℓ(h−i) (β)] : left(w′ )[j] = right(w′ )[j]
⇐⇒ left(w′ ) = right(w′ ),
completing the proof.
30

<!-- page 32 -->

For simplicity, we write enc(−) for ench (−).
Claim 44. Let w′ ∈ Σ∗ be h-good and r′ a star-free ERE. Then rzero,β ⟨r′ ⟩ matches w′ if and only
if r′ matches mid(w′ ) and left(w′ ) R right(w′ ).
Proof. By Claim 43, we have
w′ ∈ rzero,β ⟨r′ ⟩
⇐⇒ mid(w′ ) ∈ r′ and ∀i, j ∈ [1, β] : w′ ⟨i, j⟩ ∈
/ req,ℓ(β) ∩ rexcl,¢2,h ¢2,h ¬rR ¢′2,h rexcl,¢′2,h
!
∀i, j ∈ [1, β] : binℓ(β) (i) = binℓ(β) (j) ⇒
⇐⇒ mid(w′ ) ∈ r′ and
w′ ⟨i, j⟩ ∈
/ rexcl,¢2,h ¢2,h ¬rR ¢′2,h rexcl,¢′2,h
⇐⇒ mid(w′ ) ∈ r′ and ∀i ∈ [1, β] : left(w′ )[i] R right(w′ )[i],
completing the proof.
For w′ ∈ Σ∗ and v = v1 · · · vℓ ∈ (V (Γ) ∪ V (Γ̄))∗ (ℓ ≥ 0, v1 , . . . , vℓ ∈ V (Γ) ∪ V (Γ̄)), we write
w′ ⪰enc v (or v ⪯enc w′ ) if w′ ∈ r0 enc(ϕ(v1 )) · · · r0 enc(ϕ(vℓ )) r0 .
Claim 45. Let w′ be a substring of w that is empty or, for some x1 , x2 ∈ Vβ (Γ) ∪ Vβ (Γ̄), begins
with enc(ϕ(x1 )) and ends with enc(ϕ(x2 )). For every i ∈ [0, δ], ri matches w′ if and only if there
(i,γ)
exists v ⪯enc w′ such that v ∈ πβ−1 (D1 ) ∩ DR .
Proof. We proceed by induction on i. The base case i = 0 clearly holds. For i ≥ 1, by applying
Lemma 42, the induction hypothesis and Claim 44 in this order, we have
(i,γ)

∃v ∈ πβ−1 (D1

) ∩ DR : w′ ⪰enc v
(i−1,γ)

⇐⇒ ∃ℓ ∈ [0, γ], ∃xj , yj ∈ Vβ (Γ) with xj Ryj , ∃vj ∈ πβ−1 (D1

) ∩ DR for j ∈ [1, ℓ] :

w′ ⪰enc x1 v1 ȳ1 x2 v2 ȳ2 · · · xℓ vℓ ȳℓ
⇐⇒ ∃ℓ ∈ [0, γ], ∃xj , yj ∈ Vβ (Γ) with xj Ryj , ∃uj ∈ ri−1 , ∃z1 , . . . , zℓ+1 ∈ r0 for j ∈ [1, ℓ] :
w′ = z1 enc(ϕ(x1 )) u1 enc(ϕ(ȳ1 )) z2 · · · zℓ enc(ϕ(xℓ )) uℓ enc(ϕ(ȳℓ )) zℓ+1
⇐⇒ ∃ℓ ∈ [0, γ], ∃ h-good wj′ ∈ rzero,β ⟨ri−1 ⟩, ∃z1 , . . . , zℓ+1 ∈ r0 for j ∈ [1, ℓ] :
w′ = z1 w1′ z2 w2′ · · · zℓ wℓ′ zℓ+1
⇐⇒ w′ ∈ ri ,
where, in the last ⇐= direction, the h-goodness condition follows from the assumption on w′ .
The desired equivalence now follows from Claim 45 and the construction of w.

A.8

Proofs of Lemma 22 and Lemma 23

Proof of Lemma 22. We prove that there exist vectors a1 ∈ A1 , . . . , a2k ∈ A2k that are orthogonal
if and only if r matches w. For the only if direction, we can decompose s as
s=

k
K

w4(i−1) a2i−1 #w4i−3 $w4i−2 #arev
2i w4i−1 $

i=1

for some w0 , . . . , w4k−1 ∈ [01#]∗ . We let r capture #w4i−3 $w4i−2 # in the i-th capturing group.
By splitting w using the delimiter ¢, it suffices to show that for every l ∈ [1, d], the subexpression
31

<!-- page 33 -->

rzero,l,i matches s for some i ∈ [1, k]. Fix l ∈ [1, d]. Because a1 , . . . , a2k are orthogonal, we can
choose a vector ai whose (d − l + 1)-th coordinate is zero. It is straightforward to see that rzero,l,i/2
matches s when i is even, and rzero,l,(i+1)/2 matches s when i is odd.
Conversely, suppose that r matches w. The delimiter ¢ enforces that the subexpression
k
K

[01#]∗ (#[01#]∗ $[01#]∗ #)i [01#]∗ $

i=1

matches s in such a way that for each i ∈ [1, k], the i-th capturing group matches #w4i−3 $w4i−2 #
for some w4i−3J
, w4i−2 ∈ [01#]∗ . By construction of w, there are vectors a1 ∈ A1 , . . . , a2k ∈ A2k
∗
such that s = ki=1 w4i−4 a2i−1 #w4i−3 $w4i−2 #arev
2i w4i−1 $ for some w4i−4 , w4i−1 ∈ [01#] for i ∈
[1, k]. Then, for l ∈ [1, d], there is some i ∈ [1, k] such that rzero,l,i matches s so that \i matches
#w4i−3 $w4i−2 #. The delimiter $ enforces that either a2i−1 or a2i has zero in its (d − l + 1)-th
coordinate. By repeating the argument for all l ∈ [1, d], we have that a1 , . . . , a2k are orthogonal.
Proof of Lemma 23. We prove that there exist vectors ai ∈ A and bj ∈ B that are orthogonal if
and only if r matches w. For the only if direction, we can decompose w as
d rev
w = w0 ai #w1 $w2 #(brev
j w3 ¢w0 ai #w1 $w2 #) bj w3 ¢

where ai ∈ A and bj ∈ B are orthogonal, and w0 , w1 , w2 , w3 ∈ [01#]∗ . We let r capture #w1 $w2 # in
its capturing group. By splitting w using the delimiter ¢, it suffices to show that brev
j w3 ¢w0 ai ∈ rzero,l
for all l ∈ [1, d]. This indeed holds because ai and bj are orthogonal.
Conversely, suppose that r matches w. The delimiters ¢ and $ enforce that the subexpression
[01#]∗ (#Σ∗ #)1 matches a prefix of s in such a way that the capturing group matches #w1 $w2 #
for some w1 , w2 ∈ [01#]∗ . By the construction of w, there are vectors ai ∈ A and bj ∈ B such
∗
that s = w0 ai #w1 $w2 #brev
j w3 for some w0 , w3 ∈ [01#] . Then, every subsequent subexpression
rev
rzero,1 , . . . , rzero,d matches bj w3 ¢w0 ai . This implies that ai and bj are orthogonal.

32
