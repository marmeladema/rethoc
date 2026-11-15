---
file: ../pdfs/2025-ic-nogami-measuring-expressive-power-stacking-automata.pdf
citation: "Measuring the Expressive Power of Practical Regular Expressions by Classical Stacking Automata Models (journal version of MFCS 2023). Nogami, Terauchi. Information and Computation, preprint 2025"
doi: 
source: "https://terauchi.w.waseda.jp/papers/ic-rewb.pdf"
topics: [redos_backrefs_verification]
pages: 22
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Measuring the Expressive Power of Practical Regular Expressions
by Classical Stacking Automata Models⋆
Taisei Nogamia,∗, Tachio Terauchia,∗
a Waseda University, 3-4-1 Okubo, Shinjuku-ku, Tokyo 169-8555, Japan

Abstract
A rewb is a regular expression extended with a feature called backreference. It is broadly known that backreference
is a practical extension of regular expressions, and is supported by most modern regular expression engines, such as
those in the standard libraries of Java, Python, and more. Meanwhile, indexed languages are the languages generated
by indexed grammars, a formal grammar class proposed by A.V.Aho. We show that these two models’ expressive
powers are related in the following way: every language described by a rewb is an indexed language. As the smallest
formal grammar class previously known to contain rewbs is the class of context sensitive languages, our result strictly
improves the known upper-bound. Moreover, we prove the following four claims: (1) there exists a rewb whose
language does not belong to the class of stack languages, which is a proper subclass of indexed languages, (2) the
language described by a rewb without a captured reference is in the class of nonerasing stack languages, which is a
proper subclass of stack languages, (3) there exists a rewb that describes a stack language but not a nonerasing stack
language, and (4) a rewb extended with another practical extension called lookaheads can describe a non-indexed
language. Finally, we show that the hierarchy investigated in a prior study, which separates the expressive power of
rewbs by the notion of nested levels, is within the class of nonerasing stack languages.
Keywords: Regular expressions, Backreferences, Lookaheads, Expressive power

1. Introduction
A rewb is a regular expression empowered with a certain extension, called backreference, that allows preceding
substrings to be used later. It is closer to practical regular expressions than the pure ones, and supported by the
standard libraries of most modern programming languages. A typical example of a rewb follows:
Example 1. Let Σ be the alphabet {a, b}. The language L(α) described by the rewb α = (1 (a+b)∗ )1 \1 is { ww | w ∈ Σ∗ }.
Intuitively, α first captures a preceding string w ∈ L((a + b)∗ ) by (1 )1 , and second references that w by following \1.
Therefore, α matches ww. Because this L(α) is a textbook example of a non-context-free language (and therefore
non-regular), the expressive power of rewbs exceeds that of the pure ones.
In 1968, A.V.Aho discovered indexed languages with characterizations by two equivalent models: indexed grammars and (1N1 ) nested stack automata (Nested-SA) [1, 2]. The class of indexed languages is a proper superclass of
context free languages (CFL), and a proper subclass of context sensitive languages (CSL) [1].
Berglund and van der Merwe [4], and Câmpeanu et al. [6] have shown that the class of rewbs is incomparable
with the class of CFLs and is a proper subclass of CSLs. As the first main contribution of this paper, we prove that
the language described by a rewb is an indexed language. Since the class of CSLs was the previously known best
upper-bound of rewbs, our result gives a novel and strictly tighter upper-bound.
⋆ This work was supported by JSPS KAKENHI Grant Numbers JP20K20625 and JP23K24826.
∗ Corresponding authors.
1 One-way nondeterministic. “One-way” means that the input cursor will not move back to left. The antonym is “two-way.”

Preprint submitted to Elsevier

May 9, 2025

<!-- page 2 -->

Meanwhile, there is a class of the languages called stack languages [13, 12]. This class corresponds to the model
(1N) stack automata (SA), a restriction of Nested-SA. Hence, it trivially follows that the class of stack languages is
a subclass of indexed languages. Actually, this containment is known to be proper [2]. Furthermore, a model called
nonerasing stack automata (NESA) has been studied in papers such as [13, 16, 20], and its language class is known to
be a proper subclass of stack languages [20].
In this paper, we show that every rewb without a captured reference (that is, one in which no reference \i appears
as a subexpression of an expression of the form ( j α) j ) describes a nonerasing stack language. Given our result, the
following question is natural: does every rewb describe a (nonerasing) stack language? We show that the answer is
no. Namely, we show a rewb that describes a non-stack language. Simultaneously, we also show a rewb that describes
a stack language but not a nonerasing stack language.2
Additionally, we investigate the relationship between stacking automata models and rewb extended with another
popular practical extension called lookaheads.3 Namely, does every rewb with lookaheads describe an indexed language? We again answer the question negatively.
Finally, Larsen [17] has proposed a notion called nested levels of a rewb and showed that they give rise to a
concrete increasing hierarchy of expressive powers of rewbs by exhibiting, for each nested level i ∈ N, a language Li
that is expressible by a rewb at level i but not at any levels below i. We show that this hierarchy is within the class
of nonerasing stack languages, that is, there exists an NESA Ai recognizing Li for every nested level i. Below, we
summarize the main contributions of the paper.
(a) Every rewb describes an indexed language. (Section 4, Corollary 24)
(b) Every rewb without a captured reference describes a nonerasing stack language.
(Section 4, Corollary 25)
(c) There exists a rewb that describes a non-stack language. (Section 5, Theorem 26 and Corollary 31)
(d) There exists a rewb that describes a stack language but not a nonerasing stack language. (Section 5, Corollary 33)
(e) A rewb with lookaheads can describe a non-indexed language. (Section 6, Corollary 39)
(f) The hierarchy given by Larsen [17] is within the class of nonerasing stack languages.
(Section 7, Theorem 40)
Note that by (b) and (c), it follows that there is a rewb language that no rewb without a captured reference can describe
(Section 5, Corollaries 27 and 32). See also Figure 4 for a summary of the results.
The rest of the paper is organized as follows. Section 2 discusses related work. Section 3 defines preliminary
notions used in the paper such as the syntax and semantics of rewb, SA, NESA, and Nested-SA. Sections 4, 5, 6, and
7 formally state and prove the paper’s main contributions listed above. Section 8 concludes the paper with a discussion
on future work.
This paper is an extended version of the conference paper [19] and contains the following additional contributions:
1. We present a more natural example of a rewb that describes a non-stack language. The non-stack rewb α0 =
((1 \4 a)1 (2 \3)2 (3 \2 a)3 (4 \1\3)4 )∗ given in the conference version deeply relied on mutual backreferences and
was somewhat artificial. We present a new non-stack rewb that does not depend on such a complex structure.
Furthermore, we show that a slight modification of the rewb yields the first rewb whose language belongs to the
stack languages but not to the nonerasing stack languages.
2. We have shown that the language class of regular expressions extended with backreferences is contained in the
class of indexed languages, but does the containment still hold if they are further extended with lookaheads?
We settle this question negatively, that is, the languages described by rewbs extended with lookaheads are not
always indexed languages.
2. Related work
First, we discuss related work on rewbs. There are several variants of the syntax and semantics of rewbs since
they first appeared in the seminal work by Aho [3]. A recent study by Berglund and van der Merwe [4] summarizes the variants and the relations between them. In sum, there are two variants of the syntax, whether or not a
2 Note that this language witnesses the separation between the classes of stack languages and nonerasing stack languages, but this separation has
been already shown by Ogden [20].
3 See Example 36 for an example of lookaheads.

2

<!-- page 3 -->

same label may appear as the index of more than one capture (“may repeat labels”, “no label repetitions”), and two
variants of the semantics, whether an unbound reference is interpreted as the empty string or an undefined factor
(ε-semantics, ∅-semantics). As shown in [4], there is no difference in the expressive powers between these two semantics under the “may repeat labels” syntax (therefore, there are three classes with different expressive powers,
namely “no label repetitions” with ∅-semantics, “no label repetitions” with ε-semantics, and “may repeat labels”). In
this paper, we focus on the “may repeat labels” formalization, which has the highest expressive power of the three
and is often studied in formal language theory. We adopt the ε-semantics as the semantics of rewbs. Note that the
pioneering formalization of rewbs given by Aho [3] has the equivalent expressive power as this class. The rewbs
with “may repeat labels” with ε-semantics was recently proposed by Schmid with the notions of a ref-word and a
dereferencing function [21]. Simultaneously, he proposed a class of automata called memory automata (MFA), and
showed that its expressive power is equivalent to that of rewbs. Freydenberger and Schmid extended MFA to MFA
with trap-state [10]. Berglund and van der Merwe [4] showed that the class of Schmid’s rewbs is a proper subclass of
CSLs, and is incomparable with the class of CFLs. Note that there is a pumping lemma for the formalization given
by Câmpeanu et al. [6] but it is known not to work for Schmid’s rewbs. As mentioned above, Larsen introduced the
notion of nested levels and showed that increase in the levels increases the expressive powers of rewbs [17]. As for
lookaheads, Morihata [18] and Berglund et al. [5] showed that regular expressions with lookaheads are still regular,
but Chida and Terauchi [8] showed that rewbs with lookaheads are strictly more powerful than rewbs.
Next, we discuss related work on the three automata models used throughout the paper, namely SA, NESA, and
Nested-SA. Ginsburg et al. introduced SA as a mathematical model that is more powerful than pushdown automaton (PDA), and NESA as a restricted version of SA [13]. Hopcroft and Ullman discovered a type of Turing machine
corresponding to the class of two-way NESA [16]. Ogden proposed a pumping lemma for stack languages and nonerasing stack languages [20]. Aho proposed Nested-SA with a proof of the fact that (1N) Nested-SA and indexed
grammars given by himself in [1] are equivalent in their expressive powers, and recognized PDA and SA as special
cases of Nested-SA [2]. Aho also showed that the class of indexed languages is a proper superclass of CFLs, and a
proper subclass of CSLs [1]. Hayashi proposed a pumping lemma for indexed languages [14].
3. Preliminaries
3.1. Syntax and semantics of rewbs
In this section, we formalize the syntax and the semantics of rewbs following the formalization given in [10]. We
begin with the syntax. Let Σε = Σ ⊎ {ε} and [k] = {1, 2, . . . , k}, where the symbol ⊎ denotes a disjoint union.
Definition 2. For each natural number k ≥ 1, the set of k-rewbs over Σ, also written k-rewb by abuse of notation, and
the mapping var : k-rewb → P([k]) are defined as follows, where a ∈ Σε and i ∈ [k]:
(α, var(α)) ::=(a, ∅) | (\i, {i}) | (α0 α1 , var(α0 ) ∪ var(α1 )) | (α0 + α1 , var(α0 ) ∪ var(α1 ))
| (α∗0 , var(α0 )) | (( j α0 ) j , var(α0 ) ⊎ { j}) where j ∈ [k]\ var(α0 ).
S
Let 0-rewb denote the set of all regular expressions over Σ. Then, the set of all rewbs is k≥0 k-rewb.
Example 3. For example, ε, a, \1, a∗ \1, (1 a∗ )1 , ((1 a∗ )1 )∗ , (2 a∗ )2 \2, (1 a∗ )1 (2 b∗ )2 (\1 + \2), (2 (1 (a + b)∗ )1 \1)2 \2 (2 \1)2 ∗ ,
((1 \4 a)1 (2 \3)2 (3 \2 a)3 (4 \1\3)4 )∗ are rewbs. On the other hand, (1 (1 a∗ )1 )1 , (1 a∗ \1)1 , (1 (2 (1 a∗ )1 )2 )1 are not rewbs.
Note that this syntax allows multiple occurrences of captures with the same label, that is, we adopt the “may repeat
labels” convention. Next, we define the semantics.
Definition 4. Let Bk = { [i , ]i | i ∈ [k]}. The mapping Rk : k-rewb → P((Σ ⊎ Bk ⊎ [k])∗ ) is defined as follows, where
a ∈ Σε and i ∈ [k]:
Rk (a) = {a} , Rk (\i) = {i} , Rk (α0 α1 ) = Rk (α0 )Rk (α1 ),
Rk (α0 + α1 ) = Rk (α0 ) ∪ Rk (α1 ), Rk (α∗ ) = Rk (α)∗ , Rk ((i α)i ) = {[i } Rk (α) {]i } .
We let Σ[∗]
k denote

S

α∈k-rewb Rk (α).

3

<!-- page 4 -->

Example 5. Rk ((1 (a + b)∗ )1 \1) = {[1 } {a, b}∗ {]1 } {1} = { [1 w ]1 1 | w ∈ {a, b}∗ } .
That is, we first regard a rewb α over Σ as a regular expression over Σ ⊎ Bk ⊎ [k], deducing the language Rk (α).
The second step, described next, is to apply the dereferencing (partial) function Dk : (Σ ⊎ Bk ⊎ [k])∗ ⇀ Σ∗ to each of
its element.
We give an intuitive description of Dk . First, Dk scans its input string from the beginning toward the end, seeking
i ∈ [k]. If such i is found, Dk replaces this i with the substring obtained by removing the brackets in v that comes
from the preceding [i v ]i if [i exists (if this [i has no corresponding ]i , Dk becomes undefined). Otherwise, Dk replaces
this i with ε. The dereferencing function Dk repeats this procedure until all elements of [k] appearing in the string
are exhausted, then removes all remaining brackets. We let v[r] denote the string which Dk scans at the rth number
nr ∈ [k] at the rth loop.
1. [1 a [2 b]2 2 ]1 1. In this example, Dk encounters n1 = 2 first, and this 2 corresponds the preceding [2 b]2 , therefore
this 2 is replaced with v[1] = b. As a result, the input string becomes [1 a [2 b]2 b ]1 1. Dk repeats this process again.
Now, Dk locates n2 = 1 corresponding the preceding [1 a [2 b]2 b ]1 , so this 1 is replaced with v[2] = a[2 b]2 b but with
the brackets erased. Therefore, we gain [1 a [2 b]2 b ]1 abb. Finally, Dk removes all remaining brackets and produces
abbabb. Here is the diagram: [1 a [2 b]2 2 ]1 1 → [1 a [2 b]2 b ]1 1 → [1 a [2 b]2 b ]1 abb → abbabb.
2. [1 a]1 1 [1 bb]1 1. In this example, n1 = n2 = 1, v[1] = a, v[2] = bb, and
[1 a]1 1 [1 bb]1 1 → [1 a]1 a [1 bb]1 1 → [1 a]1 a [1 bb]1 bb → aabbbb.
3. abc 1 2. In this example, n1 = n2 = 1, v[1] = v[2] = ε, and abc 1 2 → abc 2 → abc.
Note that an unbound reference is replaced with the empty string ε, that is, we adopt the ε-semantics. However, as
mentioned in Section 2, this semantics’ expressive power is equivalent to that of the ∅-semantics under the “may repeat
labels” convention (see [4] for the proof). We define the language L(α) denoted by a k-rewb α to be Dk (Rk (α)) =
{ Dk (v) | v ∈ Rk (α)} (Lemmas 6 and 8 ensure that L(α) is well-defined), and define the language class of rewbs as the
union of the language classes of k-rewbs for all k.
Let g : (Σ ⊎ Bk )∗ → Σ∗ denote the free monoid homomorphism where g(x) is x for each x ∈ Σ, and ε for each
x ∈ Bk . Every v ∈ (Σ ⊎ Bk ⊎ [k])∗ can be written uniquely in the form v = v0 n1 v1 · · · nm vm , where m ≥ 0 (denoted
by cnt v), and vr ∈ (Σ ⊎ Bk )∗ and nr ∈ [k] for each r ∈ {0, . . . , m}. Here, let y0 ≜ v0 and for each r ∈ {1, . . . , m},
yr ≜ v0 n1 v1 · · · nr vr . A string v = v0 n1 v1 · · · nm vm over Σ ⊎ Bk ⊎ [k] is said to be matching if
∀r ∈ {1, . . . , m} . ∀x1 , x2 . yr−1 = x1 [nr x2 =⇒ (∃x2′ , x3 .x2 = x2′ ]nr x3 ∧ x2′ /∋ [nr , ]nr )
holds. Intuitively, a string v being matching means that for all nr ∈ [k] in v, if there exists a left bracket [nr in the string
immediately before nr , then there is a right bracket ]nr in between this [nr and nr . The following three lemmas follow.
Lemma 6. Given a matching string v, Dk (v) = g(v0 ) g(v[1] ) g(v1 ) · · · g(v[m] ) g(vm ).
Lemma 7. A prefix of a matching string is matching. That is, if we decompose a string v into v = xy, x is matching.
Moreover, x[r] = v[r] holds for each r = 1, . . . , cnt x (≤ cnt v).
Lemma 8. Every v ∈ Σ[∗]
k is matching.
In the rest of this subsection, we formally define the dereferencing function Dk and the notation v[r] , and prove the
three lemmas above by observing some basic features of Dk . The reader may skip and go to the next subsection.
Definition 9. Suppose that ⊥ < Σ ⊎ Bk ⊎ [k], and we define a Turing machine with one tape Dk as follows:
Dk = “On input string v ∈ (Σ ⊎ Bk ⊎ [k])∗ ,
Step 1. Move the head to the right until it reads an i ∈ [k], then go to Step 2. If such i does not exist, go to
Step 6.
Step 2. The assertion P2 ≜ ‘The symbol the head points out now is the leftmost natural number i ∈ [k] on
the tape’ holds. Move the head to the left until it reads a [i , then go to Step 3. If such [i does not
exist, go to Step 5.
4

<!-- page 5 -->

Step 3. Let P3 be ‘There is a right bracket ]i between the current head position and this i’. If P3 holds, go
to Step 4. Otherwise, go to Step 7.
Step 4. Move the head to the right one by one, seeking a bracket ]i . Note that no j ∈ [k] can appear in this
scan since P2 and P3 holds. Now, if the symbol written on the tape cell that the head points to is
a ∈ Σ, then insert a into the position immediately preceding this i, and go right; if it is b ∈ Bk \ {]i },
simply go right; if it is ]i , go to Step 5.
Step 5. Go back to Step 1 and remove this i.
Step 6. P6 ≜ ‘No j ∈ [k] is written on the tape’ holds. Again scan the tape from the beginning, and remove
all brackets.
Step 7. Erase all symbols written on the tape, and write the symbol ⊥.”
The order of execution of Dk is (12(345 + 5))∗ 1(6 + 237). Observe that Dk halts for any input because Step 5
cannot be taken arbitrarily many times. Thus, we think of Dk as a computable function (Σ ⊎ Bk ⊎ [k])∗ → Σ∗ ⊎ {⊥}.
Lemma 10. Suppose that the loop unit (namely Steps 1,2,3,4,5 or Steps 1,2,5) is executed exactly m′ times when an
input string v = v0 n1 v1 · · · nm vm (m = cnt v) is given to Dk . Then, for each r ∈ {0, 1, . . . , m′ }, let v(r) be the string
written on the tape immediately after the rth loop, and let v[r] be the string over Σ ⊎ Bk defined as follows:



the string bracketed in [i and ]i scanned at Step 4, (if Steps 1,2,3,4,5 are executed)
v[r] ≜ 

ε.
(if Steps 1,2,5 are executed)
Under the assumptions above, for each r ∈ {0, 1, . . . , m′ } the following equality holds:
v(r) = v0 g(v[1] ) v1 · · · g(v[r] ) vr nr+1 vr+1 · · · nm vm .
Proof. When r = 0, the left side is v(0) and the right side is v0 n1 v1 · · · nm vm = v, as required. Recall that immediately
before the (r + 1)st loop, the string written on the tape is v(r) . It continues as follows:
• At Step 1, the head of Dk is placed at immediately after g(v[r] ) in
v(r) = v0 g(v[1] ) v1 · · · g(v[r] ) vr nr+1 vr+1 · · · nm vm ,
and moves to nr+1 . Henceforth, we write simply sr for v0 g(v[1] ) v1 · · · g(v[r] ) vr , which appears before nr+1 .
• At Step 2, there are two cases:
– When Dk follows Steps 3,4,5, since it moves from Step 2 to Step 3 and P3 holds, sr is of the form
x0 [nr+1 v[r+1] ]nr+1 x1 . At Step 4 and Step 5, the strings written on the tape after each step are
Step 4: sr g(v[r+1] ) nr+1 vr+1 · · · nm vm , and Step 5: sr g(v[r+1] ) vr+1 · · · nm vm .
By definition, this string after Step 5 is v(r+1) .
– When Dk follows Step 5, since v[r+1] = ε, the string immediately after the execution of Step 5, namely
v(r+1) , is sr vr+1 · · · nm vm = sr g(v[r+1] ) vr+1 · · · nm vm .
In both cases, the equation holds for r + 1.
This completes the proof.
Lemma 11. For a matching string v, the loop of Dk runs exactly m = cnt v times. Hence, Dk (v) ∈ Σ∗ .
Proof. Let m′ be the number of loop iterations. Because obviously m′ ≤ m, we suppose m′ < m for contradiction. If
so, since it is the case that the execution moves from Steps 1,2,3 to Step 7 at the (m′ + 1)st loop, P3 does not hold for
′
v(m ) . By Lemma 10,
′
v(m ) = v0 g(v[1] ) v1 · · · g(v[m′ ] ) vm′ nm′ +1 vm′ +1 · · · nm vm
|
{z
}
sm ′

follows. Since i in the (m′ + 1)st loop is this nm′ +1 and sm′ ∋ [nm′ +1 , there is v j among v0 , . . . , vm′ such that v j ∋ [nm′ +1 .
This v j can be written in the form u0 [nm′ +1 u1 . Because v is matching and ym′ ⊇ v j ∋ [nm′ +1 , one of u1 , v j+1 , . . . , vm′
contains ]nm′ +1 . This contradicts the fact that P3 does not hold at Step 3.
5

<!-- page 6 -->

Proof of Lemma 6. In Lemma 11, the string v(m) , which is written on the tape immediately after the mth loop, becomes
g(v(m) ) at Step 6, and therefore Dk halts, as required.
Proof of Lemma 7. Immediate from Lemma 6.
∗
Finally, we prove that every v ∈ Σ[∗]
k (see Definition 4) is matching, concluding L(α) ⊆ Σ with Lemma 11.

Lemma 12. The following facts hold where α ∈ k-rewb and i ∈ [k]:
n
o

∗
(a) Rk (α) ⊆ Σ ⊎ [ j , ] j j ∈ var (α) ⊎ var (α) ,
(b) ∀v1 , v2 . v1 [i v2 ∈ Rk (α) =⇒ ∃v′2 , v3 . v2 = v′2 ]i v3 ∧ v′2 /∋ [i , ]i , i.
Proof. Immediate from the definitions of Rk (α) and var (α).
Proof of Lemma 8. There exists α ∈ k-rewb such that v ∈ Rk (α). Hereafter, we let m = cnt v and v = v0 n1 v1 · · · nm vm .
For each r ∈ {1, . . . , m}, if yr−1 can be written in the form x1 [nr x2 , we obtain v = x1 [nr x2 nr vr · · · nm vm and by Lemma 12
(b), x2 nr vr · · · nm vm can be written in the form x2′ ]nr x3 with x2′ /∋ [nr , ]nr , nr . Here x2′ ]nr /∋ nr holds. Therefore, x2′ ]nr is a
prefix of x2 and of course x2′ /∋ [nr , ]nr . Hence, v is matching.
3.2. Classical automata models
Next, we recall the notions of SA, NESA, and Nested-SA. In this paper, we unify their definitions based on [2, 12]
to clarify the different capabilities of these models. First, we review NFA. Here is the definition in the textbook by
Hopcroft et al. [15]:
Definition 13 ([15], p.57). A nondeterministic finite automaton N is a 5-tuple (Q, Σ, δ, q0 , F), where Q is a finite set
of states, Σ a finite set of input symbols (also called alphabet), q0 ∈ Q a start state, F ⊆ Q a set of final states, and
δ : Q × Σ → P(Q) a transition function.
As well known, the transition function δ can be extended to δ̂ : Q × Σ∗ → P(Q) where δ̂(q, w) represents the set of
w
a
all states reachable from q via w. Let q −
→ q′ denote q′ ∈ δ(q, a), and q =
⇒ q′ denote q′ ∈ δ̂(q, w). With this notation,
N
N


w
the language of an NFA N can be written as follows: L(N) = w ∈ Σ∗ ∃q f ∈ F. q0 =
⇒ qf .
N

A pushdown automaton (PDA) is an NFA equipped with a stack such that the PDA may write and read its stack
top with a transition. A stack automaton (SA) is “an extended PDA”, which can reference not only the top but inner
content of the stack. That is, while the stack pointer of a PDA is fixed to the top, an SA allows its pointer to move
left and right and read a stack symbol pointed to by the pointer. However, the only place on the stack that can
be rewritten is the top, as in PDA. Formally, a (1N) SA A is a 10-tuple (Q, Σ, Γ, δ, q0 , Z0 , ⊣, #, $, F) satisfying the
following conditions: the components Q, Σ, q0 and F are the same as those of NFA. Γ (, ∅) is a finite set of stack
symbols, and Z0 ∈ Γ is an initial stack symbol. The symbol ⊣ < Σ is the endmarker of the input and the symbol
# < Σ ∪ Γ (resp. $ < Σ ∪ Γ) is always and only written at the leftmost (bottom) (resp. the right most (top)) of the stack.4
The transition function δ has the following two modes, where L, S, R < (Σ ∪ Γ) ⊎ {⊣, #, $}, ∆i ≜ {S, R}, ∆ s ≜ {L, S, R}
and Σ′ ≜ Σ ⊎ {⊣}:
(i) (pushdown mode) Q × Σ′ × Γ$ → P(Q × ∆i × Γ∗ $),
(ii) (stack reading mode) (a) Q × Σ′ × Γ$ → P(Q × ∆i × {L, S}), (b) Q × Σ′ × Γ → P(Q × ∆i × ∆ s ), (c) Q × Σ′ × {#} →
P(Q × ∆i × {S, R}).
4 These special symbols #, $ representing “bottom” and “top” of the stack respectively do not appear in [12] and are introduced anew in this
paper to define NESA and Nested-SA, which will be defined later, in the style of [2]. In fact, SA defined in [12] is not capable of directly discerning
whether the stack pointer is at the top or not. Although it is not difficult to see that directly adding the ability does not increase the expressive power
of SA, the ability is directly in NESA as seen in [16, 20]. Therefore, to make it easy to see that NESA is a restriction of SA, we define SA to also
directly have the ability.

6

<!-- page 7 -->

Intuitively, δ works as follows (Definition 14 provides the formal semantics). (i) The statement (q′ , d, w$) ∈
δ(q, a, Z$) says that whenever the current state is q, the input symbol is a, and the pointer references the top symbol Z,
the machine can move to the state q′ , move the input cursor along d, and replace Z with the string w. (ii) The statement
(b) (q′ , d, e) ∈ δ(q, a, Z) says that whenever the current state is q, the input symbol is a, and the pointer references
the symbol Z, the machine can move to the state q′ , move the input cursor along d, and move the pointer along e.
The statements (a) and (c) are similar to (b) except that the direction in which the pointer can move is restricted lest
the pointer go out of the stack. In particular, an SA that cannot erase a symbol once written on the stack is called a
nonerasing stack automaton (NESA). That is, a (1N) nonerasing stack automaton is an SA whose transition function
δ satisfies the condition that, in (i) (pushdown mode), (q′ , d, y$) ∈ δ(q, a, Z$) implies y ∈ ZΓ∗ . To formally describe
how SA works, we define a tuple called instantaneous description (ID), which consists of a state, an input string, and
a string representation of the stack, and define the binary relation ⊢A over the set of these tuples. Let L = −1, S = 0,
and R = 1.
Definition 14. Let A be an SA (Q, Σ, Γ, δ, q0 , Z0 , ⊣, #, $, F). An element of the set I = Q × Σ∗ {⊣} × {#} (Γ ⊎ {↿})∗ {$} is
called instantaneous description, where the stack symbol ↿ < Γ stands for the position of stack pointer. Moreover, let
⊢A (or ⊢ when A is clear) be the smallest binary relation over I satisfying the following conditions for all q, q′ ∈ Q,
k ≥ 0, a0 , . . . , ak ∈ Σ ⊎ {⊣}, Z, Z1 , . . . , Zn ∈ Γ, γ, y ∈ Γ∗ , d ∈ ∆i and e ∈ ∆ s with k = 0 → d , R:
(i) (q, a0 · · · ak , #γZ ↿ $) ⊢A (q′ , ad · · · ak , #γy ↿ $) if (q′ , d, y$) ∈ δ(q, a0 , Z$).
(ii) (a) (q, a0 · · · ak , #γZ ↿ $) ⊢A (q′ , ad · · · ak , #γ ↿ Z$) if (q′ , d, L) ∈ δ(q, a0 , Z$), and
(q, a0 · · · ak , #γZ ↿ $) ⊢A (q′ , ad · · · ak , #γZ ↿ $) if (q′ , d, S) ∈ δ(q, a0 , Z$).
(b) if (q′ , d, e) ∈ δ(q, a0 , Z) , Z = Z j and 1 ≤ j < n with j = 1 → e , L and j = n → e , R, then
(q, a0 · · · ak , #Z1 · · · Z j ↿ · · · Zn $) ⊢A (q′ , ad · · · ak , #Z1 · · · Z j+e ↿ · · · Zn $).
(c) (q, a0 · · · ak , # ↿ Zγ $) ⊢A (q′ , ad · · · ak , #Z ↿ γ $) if (q′ , d, R) ∈ δ(q, a0 , #), and
(q, a0 · · · ak , # ↿ Zγ $) ⊢A (q′ , ad · · · ak , # ↿ Zγ $) if (q′ , d, S) ∈ δ(q, a0 , #).
Note that L < ∆i , which means the input cursor will not move back to left. We say that A accepts w ∈ Σ∗ if there
exist γ1 , γ2 ∈ Γ∗ , and q f ∈ F such that (q0 , w⊣, #Z0 ↿ $) ⊢∗A (q f , ⊣, #γ1 ↿ γ2 $). Let L(A) denote the set of all strings
accepted by A.
Notation 15. We introduce some arrow notations to draw SA diagrams. We denote the rule of (i) (q′ , d, y$) ∈
a,d/Z$→y$

a,d/□,e

δ(q, a, Z$) as q −−−−−−−−→ q′ ; (ii) (q′ , d, e) ∈ δ(q, a, □) as q −−−−−→ q′ for □ = Z$, Z or #. If d = R, we simply
write a, d/· · · as a/· · · . For convenience, we also introduce intuitive syntax sugars to represent multiple transitions at
once. For instance, an ε-transition ε/· · · consists of putting c, S/· · · for all c ∈ Σ, and we may omit the upper part in
front of / and write the part · · · only. In (i), $ → Z$ is a shorthand for putting Z ′ $ → Z ′ Z$ for all Z ′ ∈ Γ. In (ii), for
a finite subset U ⊆ Γ, the notation · · ·/U, e (resp. · · ·/U$, e) means putting · · ·/u, e (resp. · · ·/u$, e) for all u ∈ U. In
particular, U = Γ, we omit U and merely type · · ·/e. The negation ¬Z is a shorthand for Γ\ {Z}.
We next define nested stack automaton (Nested-SA) which is SA extended with the capability to create and remove
substacks. For instance, suppose that the stack is #a1 a2 ↿ a3 $ and we are to create a new substack containing b1 b2 :
#a1 ¢ b1 b2 ↿ $ a2 a3 $.

(1)

Note that the new substack ¢ b1 b2 $ is embedded below the symbol a2 indicated by the stack pointer, and the
pointer moves to the top of the created substack. The creation of the inner substack narrows the range within which
the stack pointer can move as indicated by the underlined part #a1 ¢ b1 b2 ↿ $. While the bottom of the entire stack is
always fixed by the leftmost symbol #, the top of the embedded substack is regarded as the top of the entire stack. The
inner substacks are allowed to be embedded endlessly and everywhere, whereas the writing in the pushdown mode is
still restricted to the top of the stack:
L

create

L

create

→ #a1 ¢ b1 ↿ b2 $ a2 a3 $ −−−−→ #a1 ¢ ¢ c1 c2 ↿ $ b1 b2 $ a2 a3 $,
#a1 ¢ b1 b2 ↿ $ a2 a3 $ −
→ #a1 ↿ ¢ b1 b2 $ a2 a3 $ −−−−→ #¢ c1 c2 ↿ $ a1 ¢ b1 b2 $ a2 a3 $.
#a1 ¢ ↿ b1 b2 $ a2 a3 $ −
7

(2)
(3)

<!-- page 8 -->

We must empty the inner substack and then remove itself in advance whenever we want to reference the right side of
the inner substack such as a2 , a3 . For example, let us empty the inner substack by popping twice from (1) and then
removing it:
pop

pop

destruct

#a1 ¢ b1 b2 ↿ $ a2 a3 $ −−→ #a1 ¢ b1 ↿ $ a2 a3 $ −−→ #a1 ¢ ↿ $ a2 a3 $ −−−−−→ #a1 a2 ↿ a3 $.

(4)

Notice that the stack pointer moves to the right after removing the inner substack. We now define Nested-SA formally.
A (1N) nested stack automaton A is a 10-tuple (Q, Σ, Γ, δ, q0 , Z0 , #, ¢, $, F) satisfying the following conditions: the
components Q, Σ, Γ, q0 , Z0 , #, $ and F are the same as those of SA. The stack symbol ¢ < Σ ∪ Γ represents the bottom
of a substack.5 The transition function δ has the following four modes, where Σ′ ≜ Σ ⊎ {⊣} and Γ′ ≜ Γ ⊎ {¢}:
(i) (pushdown mode) Q × Σ′ × Γ$ → P(Q × ∆i × Γ∗ $).
(ii) (stack reading mode) (a) Q × Σ′ × Γ′ $ → P(Q × ∆i × {L, S}), (b) Q × Σ′ × Γ′ → P(Q × ∆i × ∆ s ), (c) Q × Σ′ × {#} →
P(Q × ∆i × {S, R}).
(iii) (stack creation mode) Q × Σ′ × (Γ′ ⊎ Γ′ $) → P(Q × ∆i × {¢} Γ∗ $).
(iv) (stack destruction mode) Q × Σ′ × {¢} $ → P(Q × ∆i ).
Moreover, we define how Nested-SA works with ID and ⊢ in the same manner as SA. Given a Nested-SA
A = (Q, Σ, Γ, δ, q0 , Z0 , ⊣, #, ¢, $, F), we define ID, ⊢A , and L(A) in the same way as Definition 14 (however, we let
I be Q × Σ∗ {⊣} × {#} (Γ ⊎ {¢, $, ↿})∗ {$}). Here, we only give the rules corresponding to (iii) and (iv) in the definition of
δ (the others are essentially the same as those of SA):
(iii) if (q′ , d, ¢y$) ∈ δ(q, a0 , Z) and Z = Z j , 1 ≤ j < n, then
(q, a0 · · · ak , #Z1 · · · Z j ↿ · · · Zn $) ⊢A (q′ , ai+d · · · ak , #Z1 · · · ¢y ↿ $Z j · · · Zn $),
and (q, a0 · · · ak , #γZ ↿ $) ⊢A (q′ , ad · · · ak , #γ ¢y ↿ $Z$) if (q′ , d, ¢y$) ∈ δ(q, a0 , Z$).
(iv) (q, a0 · · · ak , #γ1 ¢ ↿ $Zγ2 $) ⊢A (q′ , ad · · · ak , #γ1 Z ↿ γ2 $) if (q′ , d) ∈ δ(q, a0 , ¢$).
4. Every rewb describes an indexed language
4.1. Main theorem
As described above, to obtain the language L(α) described by a k-rewb α, we derive the regular language Rk (α)
over the alphabet Σ ⊎ Bk ⊎ [k] first, then apply the dereferencing function Dk to every element of Rk (α). Using this
observation, we construct a Nested-SA Aα recognizing the language L(α) as follows.
The Nested-SA Aα is based on an NFA N recognizing the language Rk (α), in the sense that each transition in Aα
comes from a corresponding transition of N. The NFA N has the alphabet Σ ⊎ Bk ⊎ [k], and so handles three types of
a
characters. For each transition q −
→ q′ with a ∈ Σ, i.e., moving from q to q′ by an input symbol a, Aα also has the same
N

a/$→a$

b

transition except pushing a to the stack, denoted by q −−−−−−→ q′ . For each transition q −
→ q′ with b ∈ Bk , i.e., moving
N

ε/$→b$

by a bracket b, Aα has the transition pushing b without consuming input symbols, denoted by q −−−−−−→ q′ .6 For each
i

transition q −
→ q′ with i ∈ [k], Aα has a large “transition” that consists of several transitions. In this “transition,”
N

Aα first seeks the left bracket [i of the bracketed string [i v]i within the stack, and checks if the input from the cursor
position matches v character by character while consuming the input, and finally moves to q′ if all characters of v
matched.
A difficult yet interesting point is that Nested-SA cannot check v against the stack and push v onto the stack at the
same time, that is, after checking a character c of v, if Aα wants to push c to the stack, Aα must leave from v, climb
up the stack toward the top, and write c. However, after the push, Aα becomes lost by not knowing where to go back
to. How about marking the place where Aα should return in advance? Unfortunately, that does not work; Nested-SA
can insert such marks anywhere by creating substacks, but due to the restriction of Nested-SA, it cannot go above
5 Note that the bottom of the entire stack is always represented by # and not ¢, as mentioned above.
6 See also Notation 15 for the ε-transition notation.

8

<!-- page 9 -->

the position of the mark, much less climb up to the top. Therefore, Nested-SA cannot directly push the result of a
dereferencing onto the stack.
We cope with this problem as follows. We allow j ∈ [k] to appear in v, and for each appearance of j in the checking
of v, Aα pauses the checking and puts a substack containing the current state as a marker at the stack pointer position.
Then, Aα searches down the stack for the corresponding bracketed string [ j v′ ] j , and begins checking v′ if it is found.
By repeating this process, Aα eventually reaches a string v′′ ∈ (Σ ⊎ Bk )∗ containing no characters of [k]. Once done
with the check of v′′ , Aα climbs up toward the stack top, finds a marker p denoting the state to return to, and resumes
from p after deleting the substack containing the marker. By repeating this, if Aα returns to the position where it
initially found j, it has successfully consumed the substring of the input string corresponding to the dereferencing of
j. The following lemma is immediate.
Lemma 16. Let k ≥ 1 and α ∈ k-rewb. There exists an NFA (Q, Σ ⊎ Bk ⊎ [k], δ, q0 , F) over Σ ⊎ Bk ⊎ [k] recognizing
w
Rk (α) all of whose states can reach some final state, that is, ∀q ∈ Q. ∃w ∈ (Σ ⊎ Bk ⊎ [k])∗ . ∃q f ∈ F. q =
⇒ qf .
N

w

Corollary 17. Let N be the NFA in Lemma 16. For all q ∈ Q and for all w ∈ (Σ ⊎ Bk ⊎ [k])∗ , if q0 =
⇒ q then w is
N

matching.
w

w′

N

N

Proof. There are a string w′ ∈ (Σ ⊎ Bk ⊎ [k])∗ and a final state q f ∈ F such that q0 =
⇒ q =⇒ q f . Hence, ww′ ∈ L(N) =
Rk (α) follows. By Lemma 8 the string ww′ is matching, therefore by Lemma 7 its prefix w is matching.
We show the main theorem (the proof sketch is coming later):
Theorem 18. For every rewb α, there exists a Nested-SA that recognizes L(α).
The claim obviously holds when α is a pure regular expression (i.e., α ∈ 0-rewb). Suppose that α ∈ k-rewb with
k ≥ 1. By Lemma 16, there is an NFA N = (QN , Σ ⊎ Bk ⊎ [k], δN , q0 , F) that recognizes Rk (α) and satisfies Corollary
17. We construct
a Nested-SA Aα = (Q, Σ, Γ, δ, q0 , Z0o, #, ¢, $, F) as follows. Let Q ≜ QN ⊎ { ci , ei , ri | i ∈ [k]} ⊎
n
o n
Wq q ∈ QN ⊎ E p,i , L p,i p ∈ QN ⊎ { ei | i ∈ [k]} , i ∈ [k] , Γ ≜ Σ ⊎ Bk ⊎ [k] ⊎ Q ⊎ {Z0 }, and let δ be the smallest
relation that, for all a ∈ Σ, b ∈ Bk , c ∈ Σ ⊎ {⊣}, i, j ∈ [k], q, q′ ∈ QN , Z ∈ Γ and p ∈ QN ⊎ { ei | i ∈ [k]}, satisfies the
following conditions:
(1) δN (q, a) ∋ q′ =⇒ δ(q, a, Z$) ∋ (q′ , R, Za$)
(2) δN (q, b) ∋ q′ =⇒ δ(q, c, Z$) ∋ (q′ , S, Zb$)
(3) δN (q, i) ∋ q′ =⇒ δ(q, c, Z$) ∋ (Wq′ , S, Zi$)
(4) δ(Wq , c, i$) = {(ci , S, ¢q$)}
(5) δ(ci , c, p$) = {(ci , S, L)}
(6) δ(ci , c, Z) = {(ci , S, L)} where Z , [i , Z0
(7) δ(ci , c, Z0 ) = {(ri , S, R)}
(8) δ(ci , c, [i ) = {(ei , S, R)}
(9) δ(ei , a, a) = {(ei , R, R)}

a

(10) δ(ei , c, [ j ) = {(ei , S, R)} where i , j



{(ri , S, R)} (i = j)
(11) δ(ei , c, ] j ) = 

{(ei , S, R)} (i , j)
n
o
(12) δ(ei , c, j) = (c j , S, ¢ei $) where i , j
(13) δ(ri , c, Z) = {(r
n i , S, R)} o
(14) δ(ri , c, p$) = (E p,i , S, $)
n
o
(15) δ(E p,i , c, ¢$) = (L p,i , S)
n
o
(16) δ(Le j ,i , c, i) = (e j , S, R)
(17) δ(Lq,i , c, i$) = {(q, S, S)}

a/$→a$

b

ε/$→b$

Rule (1) translates q −
→ q′ into q −−−−−−→ q′ , (2) translates q −
→ q′ into q −−−−−−→ q′ , and rules (3)–(17) translate
N

N

i

q−
→ q′ into a large “transition” to consume the string that corresponds to the dereferencing of i. The details of the
N

“transition” are as follows. By looking at the underlying N with rule (3), Aα finds a state q′ that it should go back
to after going throughout the “transition,” and goes to the state Wq′ by pushing i to the stack. At Wq′ , by rule (4),
Aα inserts ¢q′ $ just below i, and goes to the state ci . The state ci represents the call mode in which Aα looks for the
left-nearest [i by rules (5) and (6) and proceeds to the state ei (execution mode) by (8) if it finds [i . Otherwise (i.e.,
the case when Aα arrives at the bottom of the stack), it proceeds to the state ri (return mode) by rule (7). At ei , Aα
consumes input symbols by checking them against the symbols on the stack (rules (9)–(12)). In particular, rule (9)
9

<!-- page 10 -->

handles the case when the symbols match. Rules (10) and (11) handle the cases when brackets are read from the stack.
The first case of (11) handles the case when the right bracket ]i is read, and the rules handle the other brackets (i.e.,
[ j or ] j with i , j) by simply skipping them (note that [ j = [i cannot happen since we started from the left-nearest [i ).
Reading j ∈ [k], by rule (12), Aα inserts ¢ei $ just below j and goes to c j to locate the corresponding [ j (here, j , i
holds by the definition of the syntax). At ri , Aα proceeds to return to the state p that passed the control to ci (rules
(13)–(17)). Since this p was pushed at the stack top, Aα first climbs up to the stack top by rule (13), transits to the
state E p,i popping p by (14), then goes to L p,i removing the embedded substack by (15), and finally goes back to p by
(16) and (17). A subtle point in the last step is that where the stack pointer should be placed depends on whether p
is a state e j (for some j ∈ [k]) or in QN . In the former case, after (15) removes the embedded substack ¢e j $ that was
created just below the call to i, the stack pointer points to i. However, the stack pointer should shift one more to the
right, lest Aα begins to repeat the call reading i again by (12). Therefore, (16) correctly handles the case by doing the
shift. In the latter case, as stipulated by (17), the stack pointer should point to the stack top symbol i since p is the
state stored at (3).
Proof sketch of Theorem 18. For proving L(α) ⊆ L(Aα ), we take w ∈ L(α) and v ∈ Rk (α) such that w = Dk (v).
Decomposing v into v0 n1 v1 · · · nm vm (where m = cnt v), we obtain a transition sequence in the underlying NFA N,
v0
nm v m
n1 v1
n2 v 2
denoted by q0 =⇒ q(0) ===⇒ q(1) ===⇒ · · · ===⇒ q(m) ∈ F. We prove by induction on r = 0, . . . , m that Aα can reach q(r)
N

N

N

N

while consuming zr = g(v0 ) g(v[1] ) g(v1 ) · · · g(v[r] ) g(vr ) from the input and pushing yr = v0 n1 v1 · · · nr vr to the stack.
Conversely, we suppose a calculation in Aα , denoted by C(1) = (q0 , w⊣, #Z0 ↿ $) ⊢ · · · ⊢ C(r) ⊢ · · · ⊢ C(m) = (pm , ⊣, #βm $),
where pm ∈ F and C(r) = (pr , wr ⊣, #βr $) for each r ∈ {1, . . . , m}. By induction on r = 1, . . . , m, we extract an
γr

underlying transition q0 =⇒ pr step by step while maintaining the invariants γr ∈ (Σ ⊎ Bk ⊎ [k])∗ and w = Dk (γr ) wr ,
N

as long as pr ∈ QN .
Hereafter, we shall refine this proof sketch. We first state two lemmas used to write our full proof of Theorem 18.
Let ⊢(n) denote the subrelation of ⊢ derived from the rule numbered (n). The following lemma is immediate from the
definition of ⊢(n) .
Lemma 19. For all q, q′ ∈ QN , w, w′ ∈ Σ∗ and γ, γ′ ∈ Γ∗ ,
a

(i) (a) for each a ∈ Σ, (q, aw⊣, #Z0 γ ↿ $) ⊢(1) (q′ , w⊣, #Z0 γa ↿ $) if q −
→ q′ ,
N

a

(b) ∃a ∈ Σ. q −
→ q′ ∧ w = aw′ ∧ β = Z0 γa ↿ if (q, w⊣, #Z0 γ ↿ $) ⊢(1) (q′ , w′ ⊣, #β$),
N

b

(ii) (a) for each b ∈ Bk , (q, w⊣, #Z0 γ ↿ $) ⊢(2) (q′ , w⊣, #Z0 γb ↿ $) if q −
→ q′ ,
N

b

(b) ∃b ∈ Bk . q −
→ q′ ∧ w = w′ ∧ β = Z0 γb ↿ if (q, w⊣, #Z0 γ ↿ $) ⊢(2) (q′ , w′ ⊣, #β$).
N

In particular, letting ⊢(1),(2) = ⊢(1) ⊎ ⊢(2) , we obtain the following statement by repeating (i)(a) and (ii)(a) zero or more
v
times: For all v ∈ (Σ ⊎ Bk )∗ , (q, g(v) w⊣, #Z0 γ ↿ $) ⊢∗(1),(2) (q′ , w⊣, #Z0 γv ↿ $) if q =
⇒ q′ .
N

i

Lemma 20. Suppose that q −
→ q′ , and γi is matching. Let m = cnt (γi). For all p ∈ QN , w, w′ ∈ Σ∗ and β ∈
N

(Γ ⊎ {¢, $, ↿})∗ , the following (a) and (b) are equivalent:
(a) p = q′ , w = g((γi)[m] ) w′ , and β = Z0 γi ↿.
(b) (q, w⊣, #Z0 γ ↿ $) ⊢(3) (Wq′ , w⊣, #Z0 γi ↿ $) ⊢ · · · ⊢ (p, w′ ⊣, #β$), where no ID with a state in QN appears in the
calculation · · · .
For the proof of Lemma 20, we recall the notations v[r] and v(r) informally (see Section 3 for the formal definitions
of v[r] and v(r) ). Let k be a positive integer and v = v0 n1 v1 . . . nm vm (m = cnt v) a matching string over Σ ⊎ Bk ⊎ [k].
For each r = 1, 2, . . . , the notation v[r] denotes the string which Dk scans at the rth number nr and v(r) the string
immediately after the rth replacement (also we let v(0) = v). For example, in the case of v = [1 a [2 b]2 2 ]1 1, Dk
processes v as v(0) = [1 a [2 b]2 2 ]1 1 → v(1) = [1 a [2 b]2 b ]1 1 → v(2) = [1 a [2 b]2 b ]1 abb → abbabb, and therefore
10

<!-- page 11 -->

v[1] = b and v[2] = a[2 b]2 b. We can easily prove the following relationship between two notations v[r] and v(r) (see
Lemma 10 for the formal proof): For each r ∈ {0, 1, . . . , m},
v(r) = v0 g(v[1] ) v1 · · · g(v[r] ) vr nr+1 vr+1 · · · nm vm .
We continue preparing some more notations for the proof. Let Σ∗⊥ ≜ Σ∗ ⊎ {⊥} and for every w ∈ Σ∗⊥ and s ∈ Σ∗ ,
let w/s denote the string w but with the suffix s erased if w ends with s, and otherwise ⊥. To use this notation, we
expand the set of all IDs I = Q × Σ∗ {⊣} × {#} (Γ ⊎ {¢, $, ↿})∗ {$} to I⊥ ≜ Q × Σ∗ {⊣} ⊎ {⊥} × {#} (Γ ⊎ {¢, $,↿})∗ {$}, and we
let C(u) denote an ID C = (·, u, · · · ) and ⊢′(n) denote the following binary relation over I⊥ (we regard a0 · · · a−1 as the
empty string ε):
o
n
⊢′(n) ≜ (C(u), C ′ (u/(a0 · · · ad−1 ))) C(a0 · · · ak ) ⊢(n) C ′ (ad · · · ak ), u ∈ Σ∗ {⊣} ⊎ {⊥} .
We define ⊢′ ≜

′
′
n ⊢(n) . Then, ⊢ ⊆ ⊢ is immediate and we show that the converse partially holds, in the sense that:

S

Lemma 21. For every string w ∈ Σ∗ and w′ ∈ Σ∗ , C(w⊣) ⊢′ C ′ (w′ ⊣) implies C(w⊣) ⊢ C ′ (w′ ⊣).
Proof. By the definition of ⊢′ , there is (C(a0 · · · ak ), C ′ (ad · · · ak )) ∈ ⊢ such that w′ ⊣ = w⊣/(a0 · · · ad−1 ). By the
definition of ⊢, C(w⊣) = C(a0 · · · ad−1 w′ ⊣) ⊢ C ′ (w′ ⊣) holds.
Definition 22. Given C, C ′ ∈ I⊥ , we write C |=(n) C ′ if ∀ j, C ′′ .C ⊢′( j) C ′′ ⇐⇒ j = n ∧ C ′′ = C ′ . We often omit
the subscript (n) and simply write C |= C ′ . Note that C |= C ′ implies not only C ⊢′ C ′ but also determinism:
∀C ′′ ∈ I⊥ . C ⊢′ C ′′ =⇒ C ′ = C ′′ .
Lemma 23. Suppose that γ ∈ (Σ ⊎ Bk ⊎ [k])∗ , i ∈ [k], w ∈ Σ∗ , β ∈ (Γ ⊎ {¢, $})∗ and p ∈ QN ⊎ { ei | i ∈ [k]}. Let
m = cnt (γi) (≥ 1). If γi is matching,
(ci , w⊣, #Z0 γ¢p ↿ $iβ$) |= · · · |= (ri , w⊣/g((γi)[m] ), #Z0 γ¢p ↿ $iβ$)
holds, where no ID with a state in QN appears in the calculation · · · .
Proof. In this proof, we sometimes write the stack representation # · · · Z ↿ · · · $ as # · · · ↾ Z · · · $ with the head-reversed
arrow ↾. First, if γ /∋ [i , it holds that
(ci , w⊣, #Z0 γ¢p ↿ $iβ$) |=∗ (ci , w⊣, #Z0 ↿ γ¢p$iβ$)
|= (ri , w⊣, #Z0 ↾ γ¢p$iβ$) |=∗ (ri , w⊣, #Z0 γ¢p ↿ $iβ$),
and by (γi)[m] = ε, we have w = w/g((γi)[m] ), as required. Henceforth, we assume that γ ∋ [i and the decomposition
γ = γ0 [i γ1 (γ1 /∋ [i ). Moreover, we can further decompose γ1 = γ2 ]i γ3 (γ2 /∋ [i , ]i , γ3 /∋ [i ) because γi is matching. We
prove by induction on m.
Case m = 1: By cnt γ = 0, γ2 ∈ (Σ ⊎ Bk )∗ follows. Letting w′ ≜ w/g(γ2 ), we have
(ci , w⊣, #Z0 γ0 [i γ1 ¢p ↿ $iβ$) |=∗ (ci , w⊣, #Z0 γ0 [i ↿ γ1 ¢p$iβ$) |= (ei , w⊣, #Z0 γ0 [i ↾ γ1 ¢p$iβ$)
|=∗ (ei , w′ ⊣, #Z0 γ0 [i γ2 ]i ↿ γ3 ¢p$iβ$) |= (ri , w′ ⊣, #Z0 γ0 [i γ2 ]i ↾ γ3 ¢p$iβ$)
|=∗ (ri , w′ ⊣, #Z0 γ0 [i γ2 ]i γ3 ¢p ↿ $iβ$).
Therefore, the claim holds since no ID with a state in QN appears in this calculation and γ2 = (γi)[m] follows from
(γi)(0) = γi = γ0 [i γ2 ]i γ3 i, γ3 /∋ [i .
Case {1, . . . , m} =⇒ m + 1: Let m0 ≜ cnt γ0 and l ≜ cnt γ2 (≥ 0). Now, m0 + l ≤ m = cnt γ holds and we write
γ2 = λ0 nm0 +1 λ1 · · · nm0 +l λl . We also define ηr ≜ γ0 [i λ0 nm0 +1 · · · λr−1 nm0 +r for each r ∈ {1, . . . , l}. By ηr being a prefix
of γi and Lemma 7, ηr is matching and (ηr )[m0 +r] = (γi)[m0 +r] , r ∈ {1, . . . , l} holds. In particular, it follows that nm0 +r , i
for every r (if there is r such that nm0 +r = i, γ2 ⊇ λ0 nm0 +1 · · · λr−1 ∋ ]i holds but this contradicts γ2 /∋ ]i ). Thus, letting
u0 ≜ w⊣, u′r ≜ ur−1 /g(λr−1 ), ur ≜ u′r /g((ηr )[m0 +r] ) and u′ = ul /g(λl ), we have
(ci , w⊣, #Z0 γ¢p ↿ $iβ$)
11

<!-- page 12 -->

|=∗ (ci , w⊣, #Z0 γ0 [i ↿ λ0 nm0 +1 λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|= (ei , u0 , #Z0 γ0 [i ↾ λ0 nm0 +1 λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|=∗ (ei , u′1 , #Z0 γ0 [i λ0 nm0 +1 ↿ λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|= (cnm0 +1 , u′1 , #Z0 γ0 [i λ0 ¢ri ↿ $nm0 +1 λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|=∗ (rnm0 +1 , u1 , #Z0 γ0 [i λ0 ¢ri ↿ $nm0 +1 λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
(by η1 being matching and induction hypothesis)
|= (Eei ,nm0 +1 , u1 , #Z0 γ0 [i λ0 ¢ ↿ $nm0 +1 λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|= (Lei ,nm0 +1 , u1 , #Z0 γ0 [i λ0 nm0 +1 ↿ λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|= (ei , u1 , #Z0 γ0 [i λ0 nm0 +1 ↾ λ1 · · · nm0 +l λl ]i γ3 ¢p$iβ$)
|= · · · |=∗ (ei , ul , #Z0 γ0 [i λ0 nm0 +1 λ1 · · · nm0 +l ↾ λl ]i γ3 ¢p$iβ$)
∗

(by similar calculation and induction hypothesis)
|=∗ (ei , u′ , #Z0 γ0 [i λ0 nm0 +1 λ1 · · · nm0 +l λl ]i ↿ γ3 ¢p$iβ$)
|= (ri , u′ , #Z0 γ0 [i λ0 nm0 +1 λ1 · · · nm0 +l λl ]i ↾ γ3 ¢p$iβ$)
|=∗ (ri , u′ , #Z0 γ0 [i λ0 nm0 +1 λ1 · · · nm0 +l λl ]i γ3 ¢p ↿ $iβ$),
and
u′ = w⊣/g(λ0 ) g((η1 )[m0 +1] ) g(λ1 ) · · · g((ηl )[m0 +l] ) g(λl )
= w⊣/g(λ0 ) g((γi)[m0 +1] ) g(λ1 ) · · · g((γi)[m0 +l] ) g(λl ),
where no ID with a state in QN appears in this calculation. Here, we write
γ = γ0 [i λ0 nm0 +1 λ1 · · · nm0 +l λl ]i γ3 = v0 n1 v1 · · · nm vm
and decompose its substrings as
vm0 = χ0 [i λ0 ,

vm0 +l = λl ]i χ1 , and γ3 = χ1 nm0 +l+1 vm0 +l+1 · · · nm vm .

Then, by Lemma 10 (or the relationship between two notations v[r] and v(r) mentioned above), we can write (γi)(m) as
γ3′ ≜

=(γi)[m+1]

z
}|
{ z
}|
{
v0 · · · χ0 [i λ0 g((γi)[m0 +1] ) vm0 +1 · · · g((γi)[m0 +l] ) λl ]i χ1 g((γi)[m0 +l+1] )vm0 +l+1 · · · g((γi)[m] ) vm i.
|{z}
|{z}
vm0

vm0 +l

That is, it holds that γ3′ ≜ χ1 g((γi)[m0 +l+1] )vm0 +l+1 · · · g((γi)[m] ) vm /∋ [i by γ3 /∋ [i , and we obtain (γi)[m+1] =
λ0 g((γi)[m0 +1] ) λ1 · · · g((γi)[m0 +l] ) λl , as shown above. Therefore, the claim holds for m + 1 since u′ = w⊣/g((γi)[m+1] ).
Proof of Lemma 20. For an arbitrary w ∈ Σ∗ , by Lemma 23,
(q, w⊣, #Z0 γ ↿ $) ⊢(3) (Wq′ , w⊣, #Z0 γi ↿ $) |=(4) (ci , w⊣, #Z0 γ¢q′ ↿ $i$)
|=∗ (ri , w⊣/g((γi)[m] ), #Z0 γ¢q′ ↿ $i$) |=(14) (Eq′ ,i , w⊣/g((γi)[m] ), #Z0 γ¢ ↿ $i$)
|=(15) (Lq′ ,i , w⊣/g((γi)[m] ), #Z0 γi ↿ $) |=(17) (q′ , w⊣/g((γi)[m] ), #Z0 γi ↿ $)

(∗)

holds. Assuming (a), we can replace |= in equation (∗) with ⊢ by Lemma 21 because w/g((γi)[m] ) = w′ ∈ Σ∗ holds,
and therefore, (b) follows. Supposing (b) conversely, we have (q, w⊣, #Z0 γ ↿ $) ⊢(3) (Wq′ , w⊣, #Z0 γi ↿ $) ⊢′ · · · ⊢′
(p, w′ ⊣, #β$), where no ID with a state in QN appears in either this calculation or (∗) except in their leftmost and
rightmost IDs. Therefore, their two calculations coincide by the determinism of |= . In particular, we obtain p = q′ ,
w′ ⊣ = w⊣/g((γi)[m] ) and β = Z0 γi ↿by the equality of their rightmost IDs, and thus, (a) follows because w′ ∈ Σ∗ .
12

<!-- page 13 -->

Proof of Theorem 18. Taking any w ∈ L(α), we have v = v0 n1 v1 · · · nm vm ∈ Rk (α) = L(N) (here, m = cnt v) such
v0
nm vm
n1 v1
n2 v2
that w = Dk (v). Now, suppose that q0 =⇒ q(0) ===⇒ q(1) ===⇒ · · · ===⇒ q(m) ∈ F. Letting yr ≜ v0 n1 v1 · · · nr vr and
N

N

N

N

zr ≜ g(v0 ) g(v[1] ) g(v1 ) · · · g(v[r] ) g(vr ) for each r ∈ {0, 1, . . . , m}, we show by induction on r that for any w′ ∈ Σ∗ ,
(q0 , zr w′ ⊣, #Z0 ↿ $) ⊢∗ (q(r) , w′ ⊣, #Z0 yr ↿ $) holds.
v0

Case r = 0: It holds that q0 =⇒ q(0) and y0 = v0 ∈ (Σ ⊎ Bk )∗ . Letting γ = ε in Lemma 19, we obtain
N

(q0 , g(v0 )w′ ⊣, #Z0 ↿ $) ⊢∗ (q(0) , w′ ⊣, #Z0 y0 ↿ $), as required.
Case r ⇒ r + 1: By the induction hypothesis, it holds that (q0 , zr+1 w′ ⊣, #Z0 ↿ $) ⊢∗ (q(r) , g(v[r+1] ) g(vr+1 ) w′ ⊣, #Z0 yr ↿
nr+1 vr+1

$). By Lemma 8, v is matching. Hence, v’s prefix yr nr+1 is also matching by Lemma 7. Let q(r) =====⇒ q(r+1) be
N

nr+1

vr+1

N

N

q(r) ===⇒ q′ ===⇒ q(r+1) . Because cnt (yr nr+1 ) = r + 1, the following calculation holds by Lemma 20 (a) =⇒ (b):
(q(r) , g((yr nr+1 )[r+1] ) g(vr+1 ) w′ ⊣, #Z0 yr ↿ $)
⊢(3) (Wq′ , g((yr nr+1 )[r+1] ) g(vr+1 ) w′ ⊣, #Z0 yr nr+1 ↿ $)
⊢∗ (q′ , g(vr+1 ) w′ ⊣, #Z0 yr nr+1 ↿ $) ⊢∗ (q(r+1) , w′ ⊣, #Z0 yr nr+1 vr+1 ↿ $).
By Lemma 7, it holds that (yr nr+1 )[r+1] = v[r+1] and yr nr+1 vr+1 = yr+1 , as required. In particular, letting r = m and
w′ = ε in the claim, we obtain (q0 , w⊣, #Z0 ↿ $) ⊢∗ (q(m) , ⊣, #Z0 ym ↿ $) because w = Dk (v) = zm by Lemma 6. Therefore,
w ∈ L(Aα ) holds since q(m) ∈ F.
Conversely, take any w ∈ L(Aα ). There exist m and an ID C(r) = (pr , wr ⊣, #βr $) for each r ∈ {1, . . . , m} such that
C(1) = (q0 , w⊣, #Z0 ↿ $) ⊢ · · · ⊢ C(r) ⊢ · · · ⊢ C(m) = (pm , ⊣, #βm $), where pm ∈ F. We show by induction on r that
for each r = 1, . . . , m, the following claim holds: if pr ∈ QN , there is γr such that C(r) = (pr , wr ⊣, #Z0 γr ↿ $), (a)
γr

γr ∈ (Σ ⊎ Bk ⊎ [k])∗ , (b) w = Dk (γr ) wr and (c) q0 =⇒ pr .
N

Case r = 1: It holds that p1 = q0 ∈ QN and C(1) = (q0 , w⊣, #Z0 ↿ $). Letting p1 = q0 , w1 = w and γ1 = ε, we have
γ1

(a) γ1 = ε ∈ (Σ ⊎ Bk ⊎ [k])∗ , (b) w = Dk (γ1 ) w1 and (c) q0 =⇒ p1 = q0 , as required.

N
n
o
Case {1, . . . , r} ⇒ r + 1: Suppose that pr+1 ∈ QN . We can define j as the maximum of the set 1 ≤ j ≤ r p j ∈ QN
since p1 ∈ QN . Rules that can be applied to C( j) are limited to (1), (2) and (3) because p j ∈ QN .
In the case of (1), j = r holds because p j+1 ∈ QN . By pr ∈ QN and the induction hypothesis, we have C(r) =
(pr , wr ⊣, #Z0 γr ↿ $), and pr , wr and γr satisfy conditions (a), (b) and (c). Hence, by Lemma 19 (i)(b), there is a ∈ Σ
a
such that pr −
→ pr+1 , wr = awr+1 and βr+1 = Z0 γr a ↿. Now, we let γr+1 = γr a. Since βr+1 = Z0 γr+1↿ , (a) γr+1 = γr a ∈
N
∗

γr+1

(Σ ⊎ Bk ⊎ [k]) because γr ∈ (Σ ⊎ Bk ⊎ [k])∗ , (b) Dk (γr+1 )wr+1 = Dk (γr )awr+1 = Dk (γr )wr = w, and (c) q0 ===⇒ pr+1
N

γr

a

N

N

because q0 =⇒ pr −
→ pr+1 . The case of (2) follows similarly as the case of (1) with Lemma 19 (ii)(b). In the case of (3),
i

there is q ∈ QN such that p j −
→ q′ . By p j ∈ QN and the induction hypothesis, we have C( j) = (p j , w j ⊣, #Z0 γ j ↿ $), and
′

N

γj

i

N

N

p j , w j and γ j satisfy the conditions (a), (b) and (c). Because q0 =⇒ p j −
→ q′ , γ j i is matching by Corollary 17. Besides,
it holds that pr+1 ∈ QN and (p j , w j ⊣, #Z0 γ j ↿ $) ⊢(3) (Wq′ , w j ⊣, #Z0 γ j i ↿ $) ⊢ · · · ⊢ C(r+1) , where no ID with a state in
QN appears in the calculation · · · . Hence, by Lemma 20 (b) =⇒ (a), we have pr+1 = q′ , w j = g((γ j i)[m] )wr+1 , and
βr+1 = Z0 γ j i ↿. Now, we let γr+1 = γ j i. Since βr+1 = Z0 γr+1 ↿, (a) γr+1 = γr i ∈ (Σ⊎Bk ⊎[k])∗ because γr ∈ (Σ⊎Bk ⊎[k])∗ ,
γr+1

γj

i

N

N

N

(b) Dk (γr+1 )wr+1 = Dk (γ j ) g((γ j i)[m] ) wr+1 = Dk (γ j )w j = w, and (c) q0 ===⇒ pr+1 because q0 =⇒ p j −
→ pr+1 .
Therefore, the claim holds for the case of r + 1. In particular, letting r = m in the claim, we have C(m) =
(pm , wm ⊣, #Z0 γm↿$), and pm , wm and γm satisfy (a), (b) and (c) (note that pm ∈ F ⊆ QN ). Because wm = ε, it holds that
γm
w = Dk (γm ) and that q0 ==⇒ pm ∈ F, or γm ∈ L(N) = Rk (α). Therefore, we have w ∈ Dk (Rk (α)) = L(α).
N

Corollary 24. Every rewb describes an indexed language, but not vice versa.
Proof. The first half follows by Theorem 18 since 1N Nested-SA and indexed grammars are equivalent [2]. The
second half also follows since the class of CSLs is a subclass of indexed languages [1], and the language class of
rewbs and that of CFLs are incomparable [4].
13

<!-- page 14 -->

4.2. The case without a captured reference
In the case of a rewb α without a captured reference (that is, one in which no reference \i appears as a subexpression of an expression of the form ( j . . . ) j ), we can transform Aα into an NESA A′′α recognizing L(α), i.e., one that
neither uses substacks nor pops its stack. First, we transform Aα to a Nested-SA without substacks (i.e., SA) A′α .
Inspecting how substacks are used in Aα , we can drop rules (12) and (16) in A′α because there is no captured reference
in α. We also remove the uses of substacks from rules (3) and (4), which correspond to calling, and rules (14), (15)
and (17), which correspond to returning. Namely, while Aα , upon a call, stores the substack ¢q′ $ that consists of just
the state q′ where the control should return, A′α simply pushes q′ to the stack top. That is, we remove (4), (15) and
(17), and change (3) and (14) to the following (3’) and (14’), respectively:
(3’) δN (q, i) ∋ q′ =⇒ δ(q, c, Z$) ∋ (ci , S, Ziq′ $),

(14’) δ(ri , c, q$) = {(q, S, $)} .

Furthermore, we transform A′α to an SA without stack popping (i.e., NESA) A′′α . Observe that A′α pops only when
returning via (14’) and popping a state that was pushed in a preceding call. Thus, A′′α , rather than popping q′ , leaves
it on the stack, and has the modes ci , ei and ri skip all state symbols on the stack except the ones at the top. Here, we
only need to modify ei since Aα already skips them at ci and ri (rules (6) and (13)). In short, we add the new rule (9*)
and change (14’) to (14”), as follows:
(9*) δ(ei , c, q) = {(ei , S, R)} ,

(14”) δ(ri , c, q$) = {(q, S, q$)} .

This NESA A′′α whose transition function consists of the rules (1),(2),(3’),(5)–(9),(9*),(10),
(11), (13) and (14”) recognizes L(α). Therefore,
Corollary 25. Every rewb without a captured reference describes a nonerasing stack language, but not vice versa.
For the latter part of Corollary 25, we can take the language T ≜ { an bn | n ∈ N} that can be described by an NESA
but not by any rewb [4]. Here, we show that T is described by the NESA A given in Figure 1.
b/⋆, L
b/⋆$, L

a/$ → ⋆$

q0

ε/S

q1

ε/Z0 , S

q2

Figure 1: An NESA that recognizes T = { an bn | n ∈ N} (see Notation 15 for the notations)

The set of stack symbols Γ is {Z0 , ⋆}, where ⋆ is a distinguished character. Intuitively, A first pushes ⋆ while
consuming the input a to count the occurrences of a and leaves q0 for q1 nondeterministically. At q1 , A moves its
stack pointer leftward while consuming the input b, and leaves there for the accepting state q2 if and only if the stack
pointer reaches the bottom of the stack. Finally, A halts at q2 . Here is the proof of L(A) = T :
Proof. The inclusion T ⊆ L(A) follows from the following calculation for any n ∈ N:
(q0 , an bn ⊣, #Z0 ↿ $) ⊢n (q0 , bn ⊣, #Z0 ⋆n ↿ $)
⊢ (q1 , bn ⊣, #Z0 ⋆n ↿ $) ⊢n (q1 , ⊣, #Z0 ↿ ⋆n $) ⊢ (q2 , ⊣, #Z0 ↿ ⋆n $).
Conversely, take any w ∈ L(A). By the construction of A, we can assume that w = an bm (where n, m ∈ N) and its
accepting calculation is in the following form for some l ∈ N:
(q0 , an bm ⊣, #Z0 ↿ $) ⊢l (q0 , an−l bm ⊣, #Z0 ⋆l ↿ $)
⊢ (q1 , an−l bm ⊣, #Z0 ⋆l ↿ $) ⊢∗ (q1 , ⊣, #Z0 ↿ ⋆l $) ⊢ (q2 , ⊣, #Z0 ↿ ⋆l $).
In this calculation, the steps ⊢∗ indicate n = l = m.
Notice that there exists a rewb with a captured reference that describes a nonerasing stack language. The rewb
(1 a)1 (2 \1)2 \2 is a simple counterexample. In addition, as shown later in Section 7, NESA can recognize nontrivial
language (hierarchy) with a captured reference such as Larsen’s hierarchy [17].
14

<!-- page 15 -->

5. A rewb that describes a non-stack language
We just showed that every rewb describes an indexed language and in particular every rewb without a captured
reference describes a nonerasing stack language. So, a natural question is whether every rewb describes a (nonerasing)
stack language. We show that the answer is no. That is, there is a rewb that describes a non-stack
n 3 language.
o
Ogden has proposed a pumping lemma for stack languages and shown that the language an n ∈ N is a non-stack
language as an application (see [20], Theorem 2). A key point in the proof is that the exponential
n3 oof a is a cubic
n
f (n)
polynomial, and we can show that for every cubic polynomial f : N → N, the language a
n ∈ N is also a nonstack language by the same proof. Thus, a rewb that describes a language in this form is a counterexample.
borrow
n We
o
2
the technique in [10] (Example 1) which shows that the rewb α = ((1 \2)1 (2 \1a)2 )∗ describes L(α) = an n ∈ N .
2

This follows since Dk (([1 2]1 [2 1 a]2 )n ) = an holds by recording the iteration count of the Kleene star, n, in the capture
(2 )2 as an , and extending the length by 2n + 1, as shown below:
Dk (([1 2]1 [2 1 a]2 )n+1 ) = Dk (([1 2]1 [2 1 a]2 )n [1 2]1 [2 1 a]2 ) = Dk (· · · [2 an ]2 [1 2]1 [2 1 a]2 )
2

2

= Dk (· · · [2 an ]2 [1 an ]1 [2 1 a]2 ) = Dk (· · · [2 an ]2 [1 an ]1 [2 an+1 ]2 ) = an a2n+1 = a(n+1) .
Theorem 26. There exists a rewb that describes a non-stack language.
n
o
Proof. The rewb α0 ≜ ((1 \4 a)1 (2 \3)2 (3 \2 a)3 (4 \1\3)4 )∗ describes L0 ≜ an(n+7)(2n+1)/6 n ∈ N and extends the
length by a quadratic in n instead. Let k = 4. It is easy to see that Rk (α0 ) = { ([1 4 a]1 [2 3]2 [3 2 a]3 [4 1 3]4 )n | n ∈ N}.
Let bn denote ([1 4 a]1 [2 3]2 [3 2 a]3 [4 1 3]4 )n (therefore L(α0 ) = { Dk (bn ) | n ∈ N}). We show by induction that Dk (bn ) =
Dk (c1 · · · cn ) where cn ≜ [1 an(n+1)/2 ]1 [2 an−1 ]2 [3 an ]3 [4 an(n+3)/2 ]4 for every n ≥ 1. First, when n = 1,
Dk (b1 ) = Dk ([1 4 a]1 [2 3]2 [3 2 a]3 [4 1 3]4 ) = Dk ([1 a]1 [2 3]2 [3 2 a]3 [4 1 3]4 )
= Dk ([1 a]1 [2 ]2 [3 2 a]3 [4 1 3]4 ) = Dk ([1 a]1 [2 ]2 [3 a]3 [4 1 3]4 )
= Dk ([1 a]1 [2 ]2 [3 a]3 [4 a 3]4 ) = Dk ([1 a]1 [2 ]2 [3 a]3 [4 a a]4 ) = Dk (c1 ).
Next, in the case of n + 1,
Dk (bn+1 ) = Dk (bn [1 4 a]1 [2 3]2 [3 2 a]3 [4 1 3]4 )
= Dk (c1 · · · cn [1 4 a]1 [2 3]2 [3 2 a]3 [4 1 3]4 )
= Dk (c1 · · · cn [1 an(n+3)/2 a]1 [2 an ]2 [3 an a]3 [4 an(n+3)/2 a an a]4 )
= Dk (c1 · · · cn [1 a(n+1)(n+2)/2 ]1 [2 an ]2 [3 an+1 ]3 [4 a(n+1)(n+4)/2 ]4 )
= Dk (c1 · · · cn cn+1 ).
∴ |Dk (bn )| =

n
X
i=1

|g(ci )| =

n
X

(i2 + 4i − 1) =

i=1

n(n + 7)(2n + 1)
. (also for n = 0)
6

Therefore, we have L(α0 ) = L0 .
From this theorem and Corollary 25, this rewb α0 needs a captured reference, in the sense that:
Corollary 27. There exists a rewb that describes a language that no rewb without a captured reference can describe.
The rewb α0 consists of mutual references by a Kleene star and the fact that it describes L0 deeply depends on
the formalization of rewbs that we have adopted.7 However, we can show that, even with only more mundane uses of
7 For instance, if we adopt the semantics of rewbs given in the ECMAScript 2023 specification [9], α would describe a different language
0
because that semantics processes a Kleene star expression α∗ by iterating the matching of the input string against α and “resets” the strings
captured within α to ϵ before each iteration.

15

<!-- page 16 -->

backreferences that do not involve mutual references, rewbs can go beyond the class of stack languages. In addition,
we illustrate that backreferences can be used to easily cross over the gaps between the nonerasing stack, stack, and
nested stack (i.e., indexed) languages. Berglund and van der Merwe formalized mutual references on a rewb with the
name circular [4]:
Definition 28. Let α be a rewb. We define the relationship →α as j →α i means the rewb α has a subexpression in the
form (i · · · \ j · · · )i and →+α as the transitive closure. The rewb α is called circular if there is a number i with i →+α i.
Definition 29. Let α1 , α2 and α3 be the following non-circular rewbs, and L1 , L2 and L3 denote their languages:
α1 ≜ (1 a∗ )1 c (\1#)∗ ,
α2 ≜ (1 a∗ )1 c (2 (\1#)∗ )2 c \2,
α3 ≜ (1 a∗ )1 c (2 (\1#)∗ )2 c \2 c \2,

L1 ≜ L(α1 ) = { wc(w#)n | w ∈ {a}∗ , n ∈ N} ,
L2 ≜ L(α2 ) = { wc(w#)n c(w#)n | w ∈ {a}∗ , n ∈ N} ,
L3 ≜ L(α3 ) = { wc(w#)n c(w#)n c(w#)n | w ∈ {a}∗ , n ∈ N} .

Lemma 30. L1 is nonerasing stack, L2 is stack but not nonerasing stack, and L3 is non-stack.
The proof is coming later. Observe that the lemma portrays a surprising power of backreferences: a slight modification of the expressions by using a simple backreference to copy a substring crosses the borders between the
nonerasing stack, stack, and nested stack (i.e., indexed) languages. We obtain the following three corollaries. The
first and second corollaries are stronger versions of Theorem 26 and Corollary 27, and the third corollary claims the
existence of a rewb language that is located between the stack and nonerasing stack languages:
Corollary 31. There exists a non-circular rewb that describes a non-stack language.
Corollary 32. There exists a non-circular rewb that describes a language that no rewb without a captured reference
can describe.
Corollary 33. There exists a non-circular rewb that describes a stack language but not a nonerasing stack language.
In the rest of the section, we shall prove Lemma 30. The first claim follows from Corollary 25 because α1 is
free of captured references. For the other claims, we need Ogden’s pumping lemmas for the stack languages and the
nonerasing stack languages [20]:8
Theorem 34. For an arbitrary (one-way) SA A, there exists some integer k satisfying the following conditions: for
an arbitrary word ξ0 ∈ L(A) with the length no less than k, there exist strings µ, ν and ρi , σi , τi (i ≥ 0) that satisfy the
following conditions:
(i) ξ0 = µρ0 σ0 τ0 ν,
(ii) for each j > 0, it holds that ξ j = µ ρ0 · · · ρ j σ j τ j · · · τ0 ν ∈ L(A),
(iii) there are integers 0 < m < n, p > 0 such that ρi , τi , σi are of the following forms:9
ρ0 =
ρi+1 =

θ0
α0 βi1 γ1

δ1
δ1

θ1
γ2 βi2 α1 βi3 γ3

···
···

δm−1
δm−1

θm−1
γ2m−2 βi2m−2 αm−1 ,

τ0 =
τi+1 =

θm
αm βi2m−1 γ2m−1

δm
δm

θm+1
γ2m βi2m αm+1 βi2m+1 γ2m+1

···
···

δn−2
δn−2

θn−1
γ2n−4 βi2n−4 αn−1 ,

σ0 =
σi+1 =

χ0
χ0

ψi+1
1

χ1
χ1

···
···

ψi+1
p−1

χ p−1
χ p−1 ,

where α j , β j , γ j , δ j , θ j , ψ j , χ j ∈ Σ∗ ,
(iv) |ρ0 | (resp. |τ0 |) = 0 ⇐⇒ |ρi | (resp. |τi |) = 0 (i > 0),
8 Our excerpts (Theorems 34 and 35) work as limited versions of the original pumping lemmas, which are more general statements.
9 The ellipsis in ρ

i
i
i
i
i+1 continues as follows: δ2 γ4 β4 α2 β5 γ5 δ3 · · · δm−2 γ2m−4 β2m−4 αm−2 β2m−3 γ2m−3 . As well for τi+1 .

16

<!-- page 17 -->

(v) ψ j > 0 and δ j > 0,
(vi) At least one of the following conditions holds:
(a) |ρ0 | > 0,
(b) |τ0 | > 0,
(c) p ≥ 2 and at least two χ j > 0,
(vii) The following inequality holds:
X

βj +

j

X

ψ j < k.

j

We obtain the pumping lemma for the nonerasing stack languages by trimming τ j and ν from this theorem:
Theorem 35. For an arbitrary (one-way) NESA A, there exists some integer k satisfying the following conditions:
for an arbitrary word ξ0 ∈ L(A) with the length no less than k, there exist strings µ and ρi , σi (i ≥ 0) that satisfy the
following conditions:
(i) ξ0 = µρ0 σ0 ,
(ii) for each j > 0, it holds that ξ j = µ ρ0 · · · ρ j σ j ∈ L(A),
(iii) there are integers m, p > 0 such that ρi , σi are of the following forms:
ρ0 =
θ0
ρi+1 = α0 βi1 γ1

δ1
δ1

θ1
γ2 βi2 α1 βi3 γ3

···
···

δm−1
δm−1

θm−1
γ2m−2 βi2m−2 αm−1 ,

σ0 =
σi+1 =

ψi+1
1

χ1
χ1

···
···

ψi+1
p−1

χ p−1
χ p−1 ,

χ0
χ0

where α j , β j , γ j , δ j , θ j , ψ j , χ j ∈ Σ∗ ,
(iv) |ρ0 | (resp. |τ0 |) = 0 ⇐⇒ |ρi | (resp. |τi |) = 0 (i > 0),
(v) ψ j > 0 and δ j > 0,
(vi) At least one of the following conditions holds:
(a) |ρ0 | > 0,
(b) p ≥ 2 and at least two χ j > 0,
(vii) The following inequality holds:
X
X
βj +
ψ j < k.
j

j

Let us explain the proof idea for L2 and L3 . Assume that L2 is a nonerasing stack language and fix an integer k
satisfying the conditions of Theorem 35. Let ξ0 ≜ wc(w#)k c(w#)k ∈ L2 where w is a string of length no less than k, for
example w = ak . The first key point is considering the position of the second c of ξ0 . A word in L2 has the property
that the prefix up to the second c determines the entire string. Therefore, the second c of ξ0 is not in µ or ρ0 ; if so,
ξ1 = ξ0 would hold because both ξ1 and ξ0 would have a common prefix up to the second c, but ξ1 must be strictly
longer than ξ0 because of the conditions of Theorem 35. Thus, it must be the case that c ∈ σ0 . In essence, in the case
of L2 , the substring (w#)n has all information of the word it accepts, hence there is a prefix property in the sense that
any two words that have this substring in common must be equal. Moreover, because ξ j ’s have a common prefix µ0 ρ0 ,
the second c must not be in this prefix. For proving the claim that L3 is not a stack language, we also use a suffix
property of L3 and derive that both the second and third c must be in σ0 . The second key point is that the informative
substring (w#)n will be included in σ0 . From this, we can show that σ1 must be longer than σ0 by at least k. However,
this contradicts the last condition of Theorem 35. A similar argument will also be used in the proof for L3 .
Let s, t be strings. We write s ⪯ t if there is a partition s = s1 · · · sr and strings u0 , . . . , ur such that t =
u0 s1 u1 · · · sr ur .
Proof of Lemma 30. We only prove that L2 is a stack but not nonerasing stack language (L3 being non-stack is quite
similar). Assume to the contrary that L2 is a nonerasing stack language and fix k given by Theorem 35. Let ξ0 =
wc(w#)k c(w#)k ∈ L2 where w = ak . As |ξ0 | ≥ k, there exist µ, ρi and σi (i ≥ 0) that satisfy the conditions of
Theorem 35. By conditions (iii), (iv) and (v), the inequality (∗) |ξ0 | < |ξ1 | easily follows.
17

<!-- page 18 -->

Henceforth, we write the first and second c of ξ0 = µρ0 σ0 = wc(w#)k c(w#)k as c1 , c2 respectively. Let us prove
c2 ∈ σ0 . Assume c2 ∈ µ. Now, µ is of the form wc(w#)k c · · · . However, ξ1 = µρ0 ρ1 σ1 = wc(w#)k c · · · ∈ L2 and so
ξ1 = ξ0 , contrary to (∗). Similarly, c2 ∈ ρ0 also does not hold. Thus, σ0 is of the form η0 c(w#)k . Because σ0 ⪯ σ1 ,
c ∈ σ1 . Now, we can derive |σ1 | − |σ0 | ≥ k as follows.
If c1 ∈ µρ0 , both ξ0 and ξ1 have the prefix wc in common. Because ξ1 ∈ L2 , σ1 can be written as σ1 = η1 c(w#)m
where η0 ⪯ η1 and c < η0 , η1 . Inequality (∗) requires m > k. Then, |σ1 | − |σ0 | = |η1 | − |η0 | + |(w#)m | − (w#)k ≥ k.
Otherwise, c1 ∈ σ0 and we can write µρ0 = u0 , σ0 = v0 c(w#)k c(w#)k and w = u0 v0 . Because σ0 ⪯ σ1 and ξ1 ∈ L2 ,
σ1 is of the form v1 c(x#)m c(x#)m , v0 ⪯ v1 . Because ξ1 = µρ0 ρ1 σ1 = u0 · · · v1 c(x#)m c(x#)m = xc(x#)m c(x#)m , |x| ≥ |w|
follows. On the other hand, since σ0 ⪯ σ1 , (w#)k ⪯ (x#)m holds and m ≥ k by counting #. Inequality (∗) ensures that
either |x| > |w| or m > k holds. In either case, |σ1 | − |σ0 | ≥ k.10
However, this contradicts the last condition of the pumping lemma. Therefore, L2 must not be nonerasing stack.
The fact that L2 is stack follows by the SA shown in Figure 2.
a/$ → a$
c/$ → ⊔$
ε/$ → ⋆$

ε/ ⋆ $, S

ε/¬Z0 , L

ε/⋆, R
ε/Z0 , R

ε/ ⊔ $, S

c/S
ε/⋆$ → $

ε/ ⋆ $, S

ε/¬Z0 , L

ε/⋆, R
ε/Z0 , R

#/⊔, R

a/a, R

#/⊔, R

#/ ⊔ $, S

a/a, R

Figure 2: A stack automaton that recognizes L2 (see Notation 15 for the notations)

6. Backreferences and lookaheads go beyond indexed languages
Chida and Terauchi formalized rewbl, a regular expression extended with backreferences and lookaheads, and
showed that its expressive power strictly exceeds that of rewbs [8]. There are two kinds of lookaheads: positive
lookaheads ?= and negative lookaheads ?!. In particular, a rewblp (resp. rewbln ) is a rewb with positive (resp. negative)
lookaheads. We write rewbl for a rewb with both positive and negative lookaheads. In what follows, for brevity, we
use the character set notation, which regards a nonempty finite set of characters C = {a1 , . . . , an } ⊆ Σ as a shorthand
for the expression a1 + · · · + an .
Example 36. Let Σ denote the alphabet {a, b}. The rewblp α = (?= ab) Σ Σ Σ describes the language L(α) = {aba, abb}
(i.e., the set of all three length words starting with ab). This is because the subexpression (?= ab) first “filters” a prefix
of the input string by matching with the regular expression abΣ∗ without consuming any input. Subsequently, the
expression Σ Σ Σ will match any unfiltered three length strings. In short, (?= α) tests for free if the matching between
αΣ∗ and the rest of the input string succeeds or not. Similarly, (?! α) tests if the matching fails or not.
In this section, in contrast with the fact that every rewb language is indexed (Section 4, Corollary 24), we show that
it is not the case for rewbs extended with lookaheads. We do not give a formal semantics of rewbs with lookaheads
(see [8] for such a semantics) and only assume the following claim which is sufficient for our purpose:
Claim 37. Let α1 and α2 denote rewbs. Without loss of generality, we assume α1 and α2 have no common capturing group indexes. Fix a fresh symbol $ not appearing in α1 and α2 . The rewblp (?= α1 $) α2 $ (resp. rewbln
(?! (?! α1 $)) α2 $) describes the language L(α1 $) ∩ L(α2 $).
10 One may think that the proof can be simplified by only comparing the suffix (w#)k ⪯ (x#)m to derive both |x| ≥ |w| and m ≥ k at once. However,
that does not derive the former inequality |x| ≥ |w| because, for example, aaa# ⪯ aa(#a)a# = (aa#)2 but not |aaa| ≤ |aa|.

18

<!-- page 19 -->

Note that whether the language class of rewblp s is closed under intersections remains open but those of rewbln s
and rewbls are known to be closed under intersections [8].
Lemma 38. For an arbitrary recursively enumerable language E, there exist a rewblp α and a GSM mapping11 g
such that E = g(L(α)).
Proof. This proof is quite similar to the proofs in [12, 7]. Fix a Turing machine M = (Q, Σ, Γ, δ, q0 , ⊔, F) that
recognizes E, where ⊔ ∈ Γ is the blank symbol and F is a set of accepting states (see [15] for the definition of the
other symbols and semantics). Without loss of generality, we can assume that the tape of M is semi-infinite and M
W
never attempts to move left at the leftmost tape position. Let c, ¢, $ be fresh symbols and i∈[l] ei denote e1 +e2 +· · ·+el .
Define rewbs




_

 _
∗
∗
∗
∗
∗
(1 Γ )1 (2 Γ)2 qa(3 Γ )3 c \1p\2b\3 c +
(1 Γ )1 qa(2 Γ )2 c \1bp\2 ⊔ c ¢Σ∗ $,
α1 ≜ 


q∈Q,a∈Γ
q∈Q,a∈Γ
δ(q,a)=(p,b,L)
δ(q,a)=(p,b,R)

∗
α2 ≜ q0 (1 Σ∗ )1 ⊔ c (2 Γ ⊎ Q)2 c \2 c Γ∗ FΓ∗ c ¢\1$.
Notice that a string that α1 matches stands for a valid computation of M that is separated by the delimiter c and ends
with ¢ . . . $, and α2 imposes three constraints on the strings it matches: (1) the first component must be equal to
the initial configuration of M, (2) for each m ≥ 1 both 2mth and (2m + 1)st components must coincide, and (3) the
penultimate component must be an accepting configuration. A subtle trick is the extra copy of the first component
surrounded by ¢ and $, which lies in the last component. It is used to indicate by ¢ the cutting position for a GSM and
by $ where the string ends for a positive lookahead. Thus, we can easily show that L(α1 ) ∩ L(α2 ) is
n
o
q0 w ⊔ c t1 c t1 c . . . tm−1 c tm−1 c tm c ¢w$ q0 w⊔ ⊢ t1 ⊢ · · · ⊢ tm is an accepting computation, m ≥ 1, w ∈ Σ∗ .
Let g be the GSM mapping defined as follows:
V/ε
c/ε
$/ε

N/ε
a/a for all a ∈ Σ
c/ε
¢/ε
$/ε

¢/ε

By Claim 37, E = g(L((?= α1 ) α2 )) holds.
Corollary 39. None of the classes of rewblp , rewbln , and rewbl is a subclass of indexed languages.
Proof. Since the class of indexed languages is closed under GSM mappings [1], the class of rewblp cannot be a
subclass of indexed languages by Lemma 38. As for rewbln , replace (?= α1 ) with (?! (?! α1 )). The last is immediate.

7. Larsen’s hierarchy is within the class of nonerasing stack language
In this section, we construct
L(xi ), where the rewb xi is given by Larsen [17] and
n an NESA Ai that describes
o
l
m r l
m r
r ∗
l
m
r
∗
defined over the alphabet Σ = a0 , a0 , a0 , a1 , a1 , a1 , . . . as follows: x0 ≜ (al0 am
0 a0 ) , xi+1 ≜ (ai+1 (i xi )i ai+1 \i ai+1 )
(i ≥ 0). Our result implies that Larsen’s hierarchy is within the class of nonerasing stack languages. Since Larsen
showed that no rewb with its nested level less than i can describe L(xi ) [17], it also implies that for every i ∈ N, there
is a nonerasing stack language that needs a rewb of nested level at least i.12
11 A generalized sequential machine (GSM) [11] is a 6-tuple (Q, Σ, ∆, δ, λ, q ), where Q is a finite set of states, Σ a finite set of input symbols, ∆
0
a finite set of output symbols, δ : Q × Σ → Q a transition function, λ : Q × Σ → ∆∗ an output function, and q0 ∈ Q a start state. The functions δ and
λ can be extended to δ̂ : Q × Σ∗ → Q and λ̂ : Q × Σ∗ → ∆∗ with δ̂(q, ε) = q; δ̂(q, wa) = δ(δ̂(q, w), a) and λ̂(q, ε) = ε; λ̂(q, wa) = λ̂(q, w)λ(δ̂(q, w), a),
a/x

respectively. We write q −−−→ q′ for δ(q, a) = q′ and λ(q, a) = x. A GSM mapping is a mapping Σ∗ → ∆∗ , w 7→ λ̂(q0 , w) for some GSM
(Q, Σ, ∆, δ, λ, q0 ).
12 Technically, Larsen [17] adopts a syntax that excludes unbound references, and so this implied result applies only to rewbs with no unbound
references.

19

<!-- page 20 -->

q10

ar1 /$ → ar1 $

al1 /$ → al1 $

m
am
1 /$ → a1 $

q00
$ → [0 $

$ → ]0 $

r
l m r
al0 am
0 a0 /$ → a0 a0 a0 $

q00

$ → 0$

r01

[0 , R

]0 , R

¬[0 , R

r
l m r
al0 am
0 a0 /$ → a0 a0 a0 $

0$, S

c10

¬0, R

e10

a/a, R

ar2 /$ → ar2 $

q20
al2 /$ → al2 $

m
am
2 /$ → a2 $

$ → [1 $

$ → ]1 $

q10

$ → 1$

al1 /$ → al1 $

$ → ]0 $

q00
r
l m r
al0 am
0 a0 /$ → a0 a0 a0 $

r12

[1 , R

m
am
1 /$ → a1 $

$ → [0 $

1$, S

c21

¬[1 , L

ar1 /$ → ar1 $

r01

[0 , R

]0 , R

a/a, R

0, R

c20

r02

[0 , R

]0 , R

¬0, R
¬[0 , L

e10

{[0 , ]0 } , R

0, L

0$, S

c10

¬[0 , R

]1 , R

e21

a/a, R

$ → 0$

¬1, R

a/a, R

¬0, R

e20

Figure 3: A0 (upper left), A1 (upper right), A2 (lower) (see Notation 15 for the notations)

The NESA Ai has the start state qi0 which is also its only final state. Figure 3 depicts A0 , A1 , and A2 . A0 is easy.
A1 is obtained by connecting the eight states to q00 and making q10 the start/final state, as shown in the figure. The
five states on the right handle the dereferencing of \0 in x1 . That is, at c10 , A1 first seeks the left-nearest [0 , passes the
control to e10 , checks the input string against the stack at e10 , passes the control to r01 , and at r01 , finally goes back to the
right-nearest 0 which must be written on the stack top. In much the same way, A2 is obtained from A1 but we must be
sensitive to the handling of the dereferencing of \1 because A2 must handle the dereferencing of not only \1 but also
\0 that appears in a string captured by [1 ]1 whereas no backreference appears in a string captured by [0 ]0 in the case of
A1 . To deal with this issue, we connect the three new states c20 , e20 and r02 to e21 . At e21 , if A2 encounters 0 in a checking,
A2 suspends the checking and first goes to c20 to seek [0 , goes to e20 to check the input against the stack by reading out
a ]0 (no number appears in this checking), and finally goes to r02 to go back to 0 which passed the control to c20 . We
repeat this modification until Ai is obtained. (Thus, Ai has such states cij , eij , rij for each j ∈ {0, . . . , i − 1}.) Therefore,
Theorem 40. There exists an NESA Ai that recognizes L(xi ).
8. Conclusions
In this paper, we have shown the following seven results: (1) that every rewb describes an indexed language
(Corollary 24), (2) in particular that every rewb without a captured reference describes a nonerasing stack language
(Corollary 25), however that there exists (3) a (non-circular) rewb that describes a non-stack language (Theorem 26
and Corollary 31) and (4) a (non-circular) rewb that describes a stack language but not a nonerasing stack language
(Corollary 33), (5) therefore that there exists a (non-circular) rewb that needs a captured reference (Corollaries 27
and 32), (6) in contrast with (1) that a rewb with lookaheads can describe a non-indexed language (Corollary 39),
and (7) finally that Larsen’s hierarchy { L(xi ) | i ∈ N} given in [17] is within the class of nonerasing stack languages
(Theorem 40). We have obtained the results by using three automata models, namely NESA, SA, and Nested-SA,
and using the semantics of rewbs given in [21, 10] that treats a rewb as a regular expression allowing us to obtain the
underlying NFA.
20

<!-- page 21 -->

[18, 5]

REG = REwL

CFL

SL

[2]

[1]

IL

CSL

[20]
Corollary
24

NESL
/

Corollary
25
Theorem 40

Corollary
31

Corollary 32

REwB\cr

Corollary
39

/

/

/

[4]

/

REwB\circ

[4, 6]
[4]

REwB

[8]

REwBLp
REwBLn
REwBL

Larsen’s hierarchy [17]

Figure 4: The inclusion relations between the language classes

Figure 4 depicts the inclusion relations between the classes mentioned in the paper. Here, REG stands for the class
of regular languages, CFL context free languages, NESL nonerasing stack languages, SL stack languages, IL indexed
languages, CSL context sensitive languages, REwL the language class of regular expressions with lookaheads, REwB
the language class of rewbs, REwB\cr the language class of rewbs without a captured reference, REwB\circ the
language class of rewbs without a circular, REwBLp (resp. REwBLn , REwBL) the language class of rewbs with
positive (resp. negative, both positive and negative) lookaheads. For the arrows, A → B stands for A ⊆ B, A ↠ B for
A ⊊ B, and A ↛ B for A ⊈ B, respectively. A label on an arrow refers to the evidence. A red dashed arrow indicates a
novel result proved in this paper, where for a strict inclusion, we show for the first time the inclusion itself in addition
to the fact that it is strict.
As future work, we would like to investigate the use of the pumping lemma for rewbs without a captured reference
that can be derived from the contraposition of our Corollary 25 and a pumping lemma for NESA [20]. We expect it to
be a useful tool for discerning which rewbs need captured references. Additionally, we suspect that our construction
of NESA in Theorem 40 is useful for not just xi of [17] but also for more general rewbs that have only one \i for each
(i )i , and we would like to investigate further uses of the construction.
References
[1] Alfred V Aho. Indexed grammars—an extension of context-free grammars. Journal of the ACM (JACM), 15(4):647–671, 1968.
[2] Alfred V Aho. Nested stack automata. Journal of the ACM (JACM), 16(3):383–406, 1969.
[3] Alfred V. Aho. Algorithms for finding patterns in strings, page 255–300. MIT Press, Cambridge, MA, USA, 1991.
[4] Martin Berglund and Brink van der Merwe. Re-examining regular expressions with backreferences. Theoretical Computer Science, 940:66–
80, 2023.
[5] Martin Berglund, Brink van Der Merwe, and Steyn van Litsenborgh. Regular expressions with lookahead. Journal of universal computer
science (Online), 27(4):324–340, 2021.
[6] Cezar Câmpeanu, Kai Salomaa, and Sheng Yu. A formal study of practical regular expressions. International Journal of Foundations of
Computer Science, 14(06):1007–1018, 2003.
[7] Benjamin Carle and Paliath Narendran. On extended regular expressions. In Language and Automata Theory and Applications: Third
International Conference, LATA 2009, Tarragona, Spain, April 2-8, 2009. Proceedings 3, pages 279–289. Springer, 2009.
[8] Nariyoshi Chida and Tachio Terauchi. On lookaheads in regular expressions with backreferences. In 7th International Conference on Formal
Structures for Computation and Deduction (FSCD 2022). Schloss Dagstuhl-Leibniz-Zentrum für Informatik, 2022.
[9] ECMA International. Ecmascript® 2023 language specification, 2022. https://tc39.es/ecma262/multipage/\#sec-intro.
[10] Dominik D Freydenberger and Markus L Schmid. Deterministic regular expressions with back-references. Journal of Computer and System
Sciences, 105:1–39, 2019.
[11] Seymour Ginsburg. The mathematical theory of context-free languages. McGraw-Hill Book, 1966.
[12] Seymour Ginsburg, Sheila A Greibach, and Michael A Harrison. One-way stack automata. Journal of the ACM (JACM), 14(2):389–418,
1967.
[13] Seymour Ginsburg, Sheila A Greibach, and Michael A Harrison. Stack automata and compiling. Journal of the ACM (JACM), 14(1):172–201,
1967.
[14] Takeshi Hayashi. On derivation trees of indexed grammars—an extension of the uvwxy-theorem—. Publications of the Research Institute
for Mathematical Sciences, 9(1):61–92, 1973.
[15] John E Hopcroft, Rajeev Motwani, and Jeffrey D Ullman. Introduction to automata theory, languages, and computation, 2nd Edition.
Addison-Wesley, 2001.

21

<!-- page 22 -->

[16] John E. Hopcroft and Jeffrey D. Ullman. Nonerasing stack automata. Journal of Computer and System Sciences, 1(2):166–186, 1967.
[17] Kim S Larsen. Regular expressions with nested levels of back referencing form a hierarchy. Information Processing Letters, 65(4):169–172,
1998.
[18] Akimasa Morihata. Translation of regular expression with lookahead into finite state automaton. Computer Software, 29(1):147–158, 2012.
[19] Taisei Nogami and Tachio Terauchi. On the expressive power of regular expressions with backreferences. In 48th International Symposium
on Mathematical Foundations of Computer Science (MFCS 2023). Schloss Dagstuhl-Leibniz-Zentrum für Informatik, 2023.
[20] William F Ogden. Intercalation theorems for stack languages. In Proceedings of the first annual ACM symposium on Theory of computing,
pages 31–42, 1969.
[21] Markus L Schmid. Characterising regex languages by regular languages equipped with factor-referencing. Information and Computation,
249:1–17, 2016.

22
