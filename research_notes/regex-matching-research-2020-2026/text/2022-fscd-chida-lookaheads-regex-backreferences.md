---
file: ../pdfs/2022-fscd-chida-lookaheads-regex-backreferences.pdf
citation: "On Lookaheads in Regular Expressions with Backreferences. Chida, Terauchi. FSCD 2022"
doi: 
source: "https://drops.dagstuhl.de/storage/00lipics/lipics-vol228-fscd2022/LIPIcs.FSCD.2022.15/LIPIcs.FSCD.2022.15.pdf"
topics: [redos_backrefs_verification]
pages: 18
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

On Lookaheads in Regular Expressions with
Backreferences
Nariyoshi Chida #
NTT Corporation, Tokyo, Japan
Waseda University, Tokyo, Japan

Tachio Terauchi #
Waseda University, Tokyo, Japan

Abstract
Many modern regular expression engines employ various extensions to give more expressive support
for real-world usages. Among the major extensions employed by many of the modern regular
expression engines are backreferences and lookaheads. A question of interest about these extended
regular expressions is their expressive power. Previous works have shown that (i) the extension by
lookaheads does not enhance the expressive power, i.e., the expressive power of regular expressions
with lookaheads is still regular, and that (ii) the extension by backreferences enhances the expressive
power, i.e., the expressive power of regular expressions with backreferences (abbreviated as rewb)
is no longer regular. This raises the following natural question: Does the extension of regular
expressions with backreferences by lookaheads enhance the expressive power of regular expressions
with backreferences? This paper answers the question positively by proving that adding either
positive lookaheads or negative lookaheads increases the expressive power of rewb (the former
abbreviated as rewblp and the latter as rewbln ). A consequence of our result is that neither the class
of finite state automata nor that of memory automata (MFA) of Schmid [14] (which corresponds to
regular expressions with backreferenes but without lookaheads) corresponds to rewblp or rewbln . To
fill the void, as a first step toward building such automata, we propose a new class of automata called
memory automata with positive lookaheads (PLMFA) that corresponds to rewblp . The key idea of
PLMFA is to extend MFA with a new kind of memories, called positive-lookahead memory, that is used
to simulate the backtracking behavior of positive lookaheads. Interestingly, our positive-lookahead
memories are almost perfectly symmetric to the capturing-group memories of MFA. Therefore, our
PLMFA can be seen as a natural extension of MFA that can be obtained independently of its original
intended purpose of simulating rewblp .
2012 ACM Subject Classification Theory of computation → Formal languages and automata theory
Keywords and phrases Regular expressions, Lookaheads, Backreferences, Memory automata
Digital Object Identifier 10.4230/LIPIcs.FSCD.2022.15
Funding This work was supported by JSPS KAKENHI Grant Numbers 17H01720, 18K19787,
20H04162, 20K20625, and 22H03570.

1

Introduction

Regular expressions, introduced by Kleene [9], and the extensions employed by many of the
modern regular expression engines are widely studied in formal language theory. Among
the major extensions are backreferences and lookaheads. Previous works on formal language
theory have studied the two features mostly in isolation. Morihata [12] and Berglund et al. [3]
showed that extending regular expressions by lookaheads does not enhance their expressive
power. Their proofs are by a translation to boolean finite automata [4] whose expressive
power is regular. The formal study of regular expressions with backreferences (rewb) dates
back to the seminal work by Aho [1]. More recently, a formal semantics and a pumping
lemma were given by Câmpeanu et al. [5], and Berglund and van der Merwe [2] showed that
different variants of backreference semantics give rise to differences in expressive powers.
Schmid [14] proposed memory automata (MFA) and showed that the expressive power of the
automata is equivalent to that of rewb.
© Nariyoshi Chida and Tachio Terauchi;
licensed under Creative Commons License CC-BY 4.0
7th International Conference on Formal Structures for Computation and Deduction (FSCD 2022).
Editor: Amy P. Felty; Article No. 15; pp. 15:1–15:18
Leibniz International Proceedings in Informatics
Schloss Dagstuhl – Leibniz-Zentrum für Informatik, Dagstuhl Publishing, Germany

<!-- page 2 -->

15:2

On Lookaheads in Regular Expressions with Backreferences

In this paper, we initiate a formal study of regular expression with backreferences and
lookaheads (rewbl for short). We call the fragment containing only positive (resp. negative)
lookaheads rewblp (resp. rewbln ). We show that both rewblp and rewbln are more expressive
than rewb, and also prove some language-theoretic properties of rewbl. One consequence of
the results is the undecidability of a problem tackled in a recent work [11].
Another consequence of our results is that neither the class of finite state automata nor that
of memory automata (MFA) of Schmid [14] (which corresponds to regular expressions with
backreferences but without lookaheads) corresponds to rewblp or rewbln . As remarked above,
prior works [3, 12] have applied translation to boolean finite automata [4] (or alternating
finite automata [7]) to build automata equivalent to regular expressions with lookaheads.
They simulate lookaheads by executing multiple runs simultaneously without backtracking.
Unfortunately, the interaction of lookaheads with backreferences prevents us from applying
the approaches to rewbl. Namely, rewbl permits cross-lookahead backreferences whereby a
string captured outside of a lookahead is referred from inside of the lookahead, or vice versa
(only the former is allowed for negative lookaheads whereas both are allowed for positive
lookaheads). Such cross-lookahead backreferences intrinsically require backtracking. In our
work, as a first step toward building automata equivalent to rewbl, we introduce a new class
of automata called memory automata with positive lookaheads (PLMFAs). We prove that
PLMFAs are equivalent to rewblp in expressive power. A key component of PLMFAs is
a new kind of memories, called a positive-lookahead memory, that is used to simulate the
backtracking behavior of positive lookaheads. Interestingly, our positive-lookahead memories
are almost perfectly symmetric to the capturing-group memories of MFA. Therefore, our
PLMFA can be seen as a natural extension of MFA that can be obtained independently of
its original intended purpose of simulating rewblp .
In summary, this paper makes the following contributions:
We show that the extension of rewb by either positive or negative lookaheads enhances
the expressive power. Additionally, we prove some language-theoretic properties of rewbl.
(Sec. 3)
We introduce memory automata with positive lookaheads (PLMFAs), a new class of
automata that we prove to be equivalent in expressive power to rewblp . A key component
of PLMFAs is a new kind of memories called positive-lookahead memory, which is almost
perfectly symmetric to capturing-group memory of MFA. (Sec. 4)
We believe that our work leads to interesting future developments in both theoretical and
practical fronts: interesting practically because backreferences and lookaheads are practically
motivated by real-world needs, and interesting theoretically because, as we shall show, rewbl
does not appear to correspond to any known formal language classes.

2

Preliminaries

In this section, we introduce the preliminary notations (Sec. 2.1) and present the syntax and
the semantics of rewbl (Sec. 2.2).

2.1

Notation

We write N for the set of natural numbers and [i] for the set {1, 2, . . . , i} where i ∈ N. For
a sequence l, we write |l| for its length, l[i] (for 1 ≤ i ≤ |l|) for its ith element, l[i..j] for
the sub-sequence from the ith element to the jth element (for 1 ≤ i ≤ j ≤ |l|). We write
l1 :: l2 for the concatenation of l1 and l2 . We abbreviate it as l1 l2 if clear from the context.
We write v ∈ l to denote that l contains v. We write Σ for a finite alphabet; a, b ∈ Σ for a

<!-- page 3 -->

N. Chida and T. Terauchi

r

::=
|
|
|
|

a
∅
ϵ
rr
r|r

character
empty set
empty string
concatenation
union

15:3

|
|
|
|
|

r∗
(i r)i
\i
(?=r)
(?!r)

repetition
capturing group
backreference
pos-lookahead
neg-lookahead

Figure 1 The syntax of rewbl expressions.

character; x, y ∈ Σ∗ for a sequence of characters (i.e., string); ϵ for the empty string; Σϵ for
Σ ∪ {ϵ}; In what follows, we fix a finite alphabet Σ. For 1 ≤ i < j ≤ |x|, we define x[i..j) to
be x[i...j − 1]. For x, y ∈ Σ∗ , we define x\y to be the left quotient of x divided by y, i.e.,
v where yv = x. Dually, the right quotient of x divided y, x/y, is v where vy = x. S ⊂ U
denotes that S is a proper subset of U , i.e., S ⊆ U ∧ S ̸= U . For a partial map f , we write
dom(f ) for the domain of f . P(S) denotes that the power set of a set S. For f a (partial)
function, f [α 7→ β] denotes the (partial) function that maps α to β and behaves as f for all
other arguments. We write f (α) = ⊥ if f is undefined at α.

2.2

Regular Expressions with Backreferences and Lookaheads

The syntax of regular expressions with backreferences and lookaheads (rewbl) is given by
Fig. 1. The semantics of the pure regular expression constructs (i.e., the first six constructs
of Fig. 1) is standard. We write · for the rewbl that matches any character (i.e., a1 | . . . |an
where Σ = {a1 , . . . , an }). The precedence order of the operators is as follows: Kleene-*,
concatenation, and union. The left has a higher precedence. For example, the expression
a∗ |bc∗ means ((a∗ )|(b(c∗ ))) due to the priority.
The remaining constructs, i.e., capturing groups, backreferences, and lookaheads, are
the extensions considered in this paper. In conformance with the nomenclature from the
literature [10], we call the fragment of rewbl without lookaheads rewb. We call the fragment
of rewbl without negative (resp. positive) lookaheads rewblp (resp. rewbln ). In what follows,
we explain the semantics of the extended features informally in terms of the standard
backtracking-based matching algorithm which attempts to match the given regular expression
with the given string and backtracks when the attempt fails. A capturing group (i r)i (or (r)i
if no ambiguity arises) attempts to match r, and if successful, stores the matched substring in
the storage identified by the index i. Otherwise, the match fails and the algorithm backtracks.
A backreference \i refers to the substring matched to the corresponding capturing group
(i r)i , and attempts to match the same substring if the capture had succeeded. If the capture
had not succeeded, i.e., is an unassigned backreference, or the matching against the captured
substring fails, then the algorithm backtracks. Capturing groups in practice often do not have
explicit indexes, but we write them here for readability. A positive (resp. negative) lookahead
(?=r) (resp. (?!r)) attempts to match r without any character consumption, proceeds if the
match succeeds (resp. fails), and backtracks otherwise.
More formally, the semantics is defined by the matching relation ⇝ that models the
behavior of backtracking matching algorithms. The full rules for deriving the matching
relation ⇝ is shown in Fig. 2. The semantics is same as the one defined in our recent work [8]
except for specializing the set-of-characters rules to the rules for a single character and the
empty set. We define ite(true, A, B) = A and ite(false, A, B) = B.
A matching relation is of the form (r, w, p, Λ) ⇝ N where p is a position on the string w
such that 1 ≤ p ≤ |w| + 1, Λ, called an environment, is a function that maps each capturing
group index to a string captured by the corresponding capturing group, and N is a set of

FSCD 2022

<!-- page 4 -->

15:4

On Lookaheads in Regular Expressions with Backreferences

p ≤ |w|
w[p] = a
(Character)
(a, w, p, Λ) ⇝ {(p + 1, Λ)}
p > |w| ∨ w[p] ̸= a
(Character Failure)
(a, w, p, Λ) ⇝ ∅
(Empty Set)

(∅, w, p, Λ) ⇝ ∅

(Empty String)

(ϵ, w, p, Λ) ⇝ {(p, Λ)}
(r1 , w, p, Λ) ⇝ N
∀(pi , Λi ) ∈ N , (r2 , w, pi , Λi ) ⇝ Ni
(r1 r2 , w, p, Λ) ⇝

S

Ni
(Concatenation)

0≤i<|N |

(r1 , w, p, Λ) ⇝ N
(r2 , w, p, Λ) ⇝ N ′
(Union)
(r1 |r2 , w, p, Λ) ⇝ N ∪ N ′
(r, w, p, Λ) ⇝ N
∀(pi , Λi ) ∈ (N \{(p, Λ)}), (r∗ , w, pi , Λi ) ⇝ Ni
(r∗ , w, p, Λ) ⇝ {(p, Λ)} ∪

S

Ni
(Repetition)

(r, w, p, Λ) ⇝ N
((r)j , w, p, Λ) ⇝ {(pi , Λi [j 7→ w[p..pi )]) | (pi , Λi ) ∈ N }
(Capturing Group)
Λ(i) ̸= ⊥
(Λ(i), w, p, Λ) ⇝ N
(Backreference)
(\i, w, p, Λ) ⇝ N
Λ(i) = ⊥
(Backreference Failure)
(\i, w, p, Λ) ⇝ ∅
(r, w, p, Λ) ⇝ N
((?=r), w, p, Λ) ⇝ {(p, Λ′ ) | (_, Λ′ ) ∈ N }
(Positive Lookahead)
(r, w, p, Λ) ⇝ N
N ′ = ite(N ̸= ∅, ∅, {(p, Λ)})
((?!r), w, p, Λ) ⇝ N ′
(Negative Lookahead)

0≤i<|(N \{(p,Λ)})|

Figure 2 Rules of the matching relation ⇝.

matching results. A matching result is a pair of a position and an environment. Roughly,
(r, w, p, Λ) ⇝ N is read: a rewbl expression r tries to match the string w from the position p,
with the environment Λ and, if (p′ , Λ′ ) ∈ N , r consumed p′ − p characters and updated the
environment to Λ′ . Additionally, if N = ∅, it means that the matching failed.
In the two rules for a character, the rewbl a tries to match the string w at the position p
with the function capturing Λ. If the pth character w[p] is a, then the matching succeeds
returning the matching result (p + 1, Λ) (Character). Otherwise, the character w[p] does
not match or the position is at the end of the string, and ∅ is returned as the matching result
indicating the match failure (Character Failure).
The rules (Empty Set), (Empty String), (Concatenation), (Union) and
(Repetition) are self explanatory. Note that we avoid self looping in (Repetition) by not
repeating the match from the same position.
In the rule (Capturing Group), we first get the matching result N from matching w
against r at the current position p. And for each matching result (pi , Λi ) ∈ N (if any), we
record the matched substring w[p..pi ) in the corresponding environment Λi at the index i.
The rule (Backreference) looks up the captured substring and tries to match it with the
input at the current position. The match fails if the corresponding capture has failed as
stipulated by the rule (Backreference Failure).
In the rule (Positive Lookahead), the expression r is matched against the given
string w at the current position p to obtain the matching results N . Then, for every match
result (p′ , Λ′ ) ∈ N (if any), we reset the position from p′ to p. This models the behavior of
lookaheads which does not consume the string. The rule (Negative Lookahead) is similar,
except that we reset and proceed when there is no match. Note that captures made inside
of a negative lookahead cannot be referred outside of the lookahead, which agrees with the
behavior of regular expression engines in practice.
▶ Definition 1 (Language). The language of a rewbl r is defined as L(r) = {w | (r, w, 1, ∅) ⇝
N ∧ ∃Λ.(|w| + 1, Λ) ∈ N }.
Recall that one subtle aspect of rewbl is that backreferences can cross lookahead boundaries
(cf. Sec. 1). We next show some examples of cross-lookahead backreferences.
▶ Example 2. Consider the expression (1 ·∗ z)1 (?=\1)·∗ . Its language is {xzxzy | x, y ∈ Σ∗ }.
For example, when the input string is azazbc, the expression captures the prefix az and
refers it from the inside of the positive lookahead (?=\1).

<!-- page 5 -->

N. Chida and T. Terauchi

15:5

▶ Example 3. Consider the expression (1 ·)1 (?!\1)·∗ . The language is {a | a ∈ Σ} ∪ {abx |
a, b ∈ Σ ∧ a ̸= b ∧ x ∈ Σ∗ }.
▶ Example 4. Consider the expression (?=(1 ·∗ )1 z)·\1. The language is {zx | x ∈ {z}∗ }.
Example 2 (resp. 3) shows an example where a string captured outside of a positive (resp. negative) lookahead is backreferenced in the lookahead. Example 4 shows an example where a
string captured inside of a positive lookahead is backreferenced from outside of the lookahead.

2.2.1

Conventions on Syntax and Semantics

We review the conventions regarding capturing groups and unassigned references. The
conventions are proposed in prior works on rewb, and as shown by [2], they affect the
expressive power of rewb. Here, we simply present the conventions and refer interested
readers to [2] for the expressive power differences.
There are two convensions regarding capturing groups: no label repetitions (NLR) and
may repeat labels (MRL). NLR requires the indexes of capturing groups to be distinct, whereas
MRL imposes no such restrictions. For example, (·∗ )1 \1 satisfies NLR, but ((·∗ )1 |(·∗ )1 )\1
does not because the capturing group with index 1 appears twice. NLR is assumed in the
prior works by Câmpeanu et al. [5] and Carle and Narendran [6] on the expressive power of
rewb.
There are two conventions regarding unassigned references: the ϵ semantics and the ∅
semantics. The ϵ (resp. ∅) semantics defines that unassigned references are handled as an
empty string ϵ (resp. a failure ∅). For example, for r = a\1, L(r) = {a} with the ϵ semantics
but L(r) = ∅ with the ∅ semantics. Additionally, prior works have proposed a condition called
no unassigned reference (NUR). The NUR condition does not allow unassigned references in
expressions, i.e., all expressions with unassigned references are to be excluded (see below for
the formal definition). For example, r = a\1 does not satisfy the NUR condition because \1
is an unassigned references. Note that the ϵ semantics and the ∅ semantics coincide under
the NUR condition because there would be no unassigned references. The condition is also
assumed in [5, 6] ([6] incorrectly remarks that [5] does not assume the condition).
In the rest of this section, we give a formal definition of the NUR condition that we shall
also use later in our proofs. We note that prior works that proposed the condition did not
provide a formal definition of it [5, 6]. First, we define the function Capture from rewbls to
the set of capturing group indexes that can be referred from their continuations:

∅
(if r = a, ∅, ϵ, r1∗ , \i, or (?!r1 ))




 Capture(r1 ) ∪ Capture(r2 ) (if r = r1 r2 )
Capture(r) =
Capture(r1 ) ∩ Capture(r2 ) (if r = r1 |r2 )



Capture(r1 ) ∪ {i}
(if r = (i r1 )i )


Capture(r1 )
(if r = (?=r1 ))
With this, we can define the predicate NUR(S,r) that says that r satisfies NUR condition if
it occurs in a context where the capturing group indexes in S can be referred:

true
(if r = a, ∅, or ϵ)



 NUR(S, r1 ) ∧ NUR(S ∪ Capture(r1 ), r2 ) (if r = r1 r2 )
NUR(S, r) =

NUR(S, r1 ) ∧ NUR(S, r2 )




 NUR(S, r1 )
i∈S

(if r = r1 |r2 )
(if r = r1∗ , (i r1 )i , (?=r1 ), or (?!r1 ))
(if r = \i)

FSCD 2022

<!-- page 6 -->

15:6

On Lookaheads in Regular Expressions with Backreferences

Then, r can be said to satisfy the NUR condition iff NUR(∅, r) = true. For example, the
expression r = (1 a)1 \1 satisfies the NUR condition because NUR(∅, r) = NUR(∅, (1 a)1 ) ∧
NUR(∅∪Capture((1 a)1 ), \1) = NUR(∅, a) ∧ NUR({1}, \1) = true. As another example, r =
(1 a\1)1 does not satisfy the NUR condition because NUR(∅, r) = NUR(∅, a\1) = NUR(∅,
a) ∧ NUR(∅∪Capture(a), \1) = false. In what follows, unless explicitly stated otherwise, we
assume that a rewbl that satisfies NLR and NUR.

3

Language Properties of Rewbl

In this section, we prove some salient language properties of rewbl. Importantly, we show
that both rewblp and rewbln is strictly more expressive than rewb, thus showing that the
extension by either posistive or negative lookaheads changes the expressive power of rewb.
In the following, we denote by LB , LBL , LBLn , and LBLp the class of languages matched by
rewb, rewbl, rewbln , and rewblp , respectively.
Our first result states that LBLn is closed under union, intersection, and complement.
▶ Theorem 5. LBLn is closed under union, intersection, and complement.
Proof. Suppose we have rewbl expressions r1 and r2 . Then, rewbl expressions that accept
the union of L(r1 ) and L(r2 ), the intersection of L(r1 ) and L(r2 ), and the complement of
L(r1 ) can be constructed respectively as follows.
Union: L(r1 ) ∪ L(r2 ) = L(r1 |r2 );
Intersection: L(r1 ) ∩ L(r2 ) = L((?!(?!r1 (?!·)))r2 ); and
Complement: L(r1 )c ≜ Σ∗ \L(r1 ) = L((?!r1 (?!·))·∗ ).
In Intersection and Complement, a subtle point is that a negative lookahead (?!r)
accepts a string even if the expression r rejects only a prefix of the string. For example,
L((?!(?!r1 )))r2 ) is the set of strings in L(r2 ) that have a prefix that belongs to L(r1 ), rather
than the intersection of L(r1 ) and L(r2 ). To force whole matching, the negative lookahead
(?!·) is appended.
◀
Of course, we could alternatively show Intersection from Union and Complement by
applying De Morgan’s laws: L(r1 ) ∩ L(r2 ) = (L(r1 )c ∪ L(r2 )c )c . The above proof gives a
direct construction which shows that the intersection can be obtained by a short rewbln
expression.
We next show that LBL is also closed under union, intersection, and complement.
▶ Theorem 6. LBL is closed under union, intersection, and complement.
Proof. The proof is the same as Theorem 5. Or, for Intersection, an even shorter proof is
possible: L(r1 ) ∩ L(r2 ) = L((?=r1 (?!·))r2 ).
◀
A consequence of Theorem 5 is that rewbl and rewbln are more expressive than rewb.
▶ Corollary 7. LB ⊂ LBLn ⊆ LBL .
Proof. Immediate from Theorem 5 and Lemma 3 of [6] which showed that rewb is not closed
under intersection.
◀
We show that adding just positive lookaheads also increases the expressive power of rewb.
▶ Theorem 8. LB ⊂ LBLp .

<!-- page 7 -->

N. Chida and T. Terauchi

15:7

Table 1 Summary of closure properties. The rows highlighted in gray present our new results. ?
indicates that the problem is open.

Regular
LB
LBLp
LBLn
LBL

∪
Yes
Yes
Yes
Yes
Yes

∩
Yes
No
?
Yes
Yes

Closure under
Complement Concatenation
Yes
Yes
No
Yes
?
Yes
Yes
Yes
Yes
Yes

Kleene-*
Yes
Yes
Yes
Yes
Yes

Proof. From [6], the language S = {ai bai+1 bak | k = i(i + 1)k ′ , k ′ > 0, and i > 0} is not in
LB . Let S ′ = {ai bai+1 bak c | k = i(i + 1)k ′ , k ′ > 0, and i > 0}. We can prove that S ′ is also
not in LB in a manner similar to the proof that S is not in LB [6].
Now, S ′ is the intersection of L(r1 c) and L(r2 c) where r1 and r2 are (aa∗ )1 b\1ab\1\1∗
and (aa∗ )1 b(\1a)2 b\2\2∗ , respectively. Then, the intersection of the languages of r1 c and
r2 c is L((?=r1 c)r2 c) = S ′ ∈ LBLp .
◀
A summary of the results of closure properties can be found in Table 1. Additionally,
the diagram below summarizes the results of the hierarchy of the language classes. Here,
LL denotes the class of languages matched by regular expressions with (both positive and
negative) lookaheads, which is known to be equivalent in expressive power to the set of
regular languages [3, 12].

From Theorems 5 and 6, we also obtain the following result.
▶ Theorem 9. The emptiness problems of rewbl and rewbln are undecidable.
Proof. Suppose for contradiction that the emptiness problem of rewbl is decidable. By
Theorem 6, we know that the language of rewbl is closed under intersection. Therefore, the
emptiness problem of intersection of two rewb expressions is also decidable. However, this
contradicts a result from [6] which states that the latter problem is undecidable. The proof
for rewbln is similar by using Theorem 5.
◀
A recent work [11] has proposed a method for symbolically executing programs containing
rewbl. Their method generates and tries to solve constraints of the form x ∈ L(r) where r
is a rewbl expression and x is a variable for which the method tries to find an assignment
that satisfies the constraint. Theorem 9 implies that their constraint solving problem is
undecidable.
▶ Corollary 10. The constraint solving problem of [11] is undecidable.
▶ Remark 11. The constructions used to show Cor. 7 and Theorem 8 do not contain
backreferences that cross lookahead boundaries (recall the discussion from Sec. 1 and Sec. 2.2).
Thus, our results show that lookaheads enhance the expressive power of rewb even without
cross-lookahead backreferences. We leave for future work to investigate whether there are
expressive power changes from allowing or disallowing cross-lookahead backreferences.

FSCD 2022

<!-- page 8 -->

15:8

On Lookaheads in Regular Expressions with Backreferences

3.1

Restricted Label Repetitions

As pointed out in [2], allowing rewbs to repeat labels of backreferences affects their expressive
powers. In this sub-section, we introduce new conditions called restricted may repeat labels
(RMRL) and no self-capturing reference (NSR). RMRL allows repeating the labels but only
in a restricted way. NSR enforces that there is no reference that is nested by the capturing
group of the same index. We use RMRL and NSR as an intermediary in the construction of
automata equivalent in expressive power to rewblp (with NUR and NLR) in the next section.
But the new conditions may also be of independent interest.
Informally, RMRL requires the capturing group that is referred to by any reference is
uniquely determined. For example, (1 a)1 (1 b)1 \1 satisfies RMRL because the reference \1
refers to the capturing group (1 b)1 while ((1 a)1 |(1 b)1 )\1 does not satisfy RMRL because the
reference \1 can refer to the capturing groups (1 a)1 and (1 b)1 . To represent the repetitions
of indexes, we use a multiset. A multiset, denoted by {{ · · ·}}, is a collection of elements
with repetitions. For example, {{a, a, b}} is a multiset that has two a’s and one b. We use
the notation for sets as that of multisets, e.g., we use ∪ for the union of multisets, e.g.,
{{a}} ∪ {{a, b}} = {{a, a, b}}, and | · | for the number of elements, e.g., |{{a, a, b}}| = 3. Formally,
we define RMRL by first defining NumCaps that takes a multiset S and a rewbl expression r
and returns a multiset that represents the number of ways to capture for each index.

S




 NumCaps(NumCaps(S, r1 ), r2 )
NumCaps(S, r) =
NumCaps(S, r1 ) ∪ NumCaps(S, r2 )



{
{j | j ∈ NumCaps(S, r1 ) ∧ j ̸= i}} ∪ {{i}}


NumCaps(S, r1 )

(if r = a, ∅, ϵ, \i, or (?!r1 ))
(if r = r1 r2 )
(if r = r1 |r2 )
(if r = (i r1 )i )
(if r = (?=r1 ) or r1∗ )

With this, we define RMRL(S,r) that says that r satisfies the RMRL condition if it occurs in
a context where the capturing group indexes in S can be referred:

RMRL(S, r) =


true
(if r = a, ∅, or ϵ)



 RMRL(S, r1 )∧RMRL(NumCaps(S, r1 ), r2 ) (if r = r1 r2 )
RMRL(S, r1 )∧RMRL(S, r2 )




 RMRL(S, r1 )

|{{i | i ∈ S}}| ≤ 1

(if r = r1 |r2 )
(if r = r1∗ , (i r1 )i , (?=r1 ), or (?!r1 ))
(if r = \i)

We say that a rewbl expression r satisfies RMRL iff RMRL(∅, r) = true.
Next, we explain NSR. NSR requires that for every reference \i, the reference is not
nested by the capturing group whose index is i. For example, (1 a\1)1 (2 \1)2 does not satisfy
NSR because \1 appears in its capturing group. Formally, we define NSR as follows.

true
(if r = a, ∅, or ϵ)




NSR(S,
r
)
∧
NSR(S,
r
)
(if r = r1 r2 or r1 |r2 )

1
2
NSR(S, r) =
NSR(S, r1 )
(if r = r1∗ , (?=r1 ), or (?!r1 ))



NSR(S ∪ {i}, r1 )
(if r = (i r1 )i )


i∈
/S
(if r = \i)
We say that a rewbl expression r satisfies NSR iff NSR(∅, r) = true. We show that
RMRL ∧ NSR is equivalent to NLR in expressive powers.
▶ Lemma 12. (1) For any NLR r there exists a NSR and RMRL r′ such that L(r) = L(r′ ),
and (2) for any NSR and RMRL r there exists a NLR rewbl r′ such that L(r) = L(r′ ).

<!-- page 9 -->

N. Chida and T. Terauchi

15:9

Proof. (1) is immediate since NLR implies both RMRL and NSR (under the NUR assumption). To see (2), if r satisfies RMRL and NSR, then capturing groups that are referred
are uniquely determined and are closed when they are referred. Thus, we can construct r′
by replacing indexes of reference i in r and the capturing group referred to i with unique
indexes and removing all unreferred capturing groups.
◀

4

Memory Automata with Positive Lookaheads

This section present PLMFA, a new class of automata that we prove to be equivalent to
rewblp . PLMFA is obtained by extending MFA of Schmid [14] that is equivalent to rewb. The
key extension is the addition of a new kind of memories called positive-lookahead memories.
Roughly, a PLMFA is a non-deterministic finite state automata augmented with a list
of capturing-group memories and a list of positive-lookahead memories. The former also
exists in MFA and stores strings captured by capturing groups to simulate the behavior
of backreferences. The latter stores strings matched by positive lookaheads and is used to
simulate the behavior of positive lookaheads.

4.1

Formal Definition

A memory is a tuple (x, s ) of a string x ∈ Σ∗ and a status s . A status is either open (O )
or close (C ). The statuses are changed by memory instructions (instructions for short)
Θ = {o, c, ⋄} as follows: s ⊕ o = O , s ⊕ c = C , and s ⊕ ⋄ = s . Roughly, O means that
the string in the memory is modified by appending consumed strings, while C means that
the string in the memory is unmodified. Changing the status from C to O (resp. from
O to C ) representing to an entering (resp. exiting) a capturing group if the memory is
a capturing-group memory and otherwise (i.e., if positive-lookahead memory) a positive
lookahead. At computation steps corresponding to backreferences, the strings in capturing
group memories are used to left-divide the input string and appended to strings stored in
any open memories. Symmetrically, when the strings in positively lookahead memories are
used, they are prepended to the input string and used to right-divide strings stored in any
open memories. A positive lookahead memory is used when it gets closed. For a memory
t = (x, s), we write t.word for x and t.status for s. We define PLMFAs as follows.
▶ Definition 13 (PLMFA). For (kc , kp ) ∈ N2 , a (kc , kp )-memory automaton with positive
lookaheads, PLMFA(kc , kp ), is a tuple (Q, δ, q0 , F ) such that
1. Q is a finite set of states,
2. δ : Q × (Σϵ ∪ [kc ]) → P(Q × Θkc × Θkp ) is the transition function,
3. q0 ∈ Q is the initial state, and
4. F ⊆ Q is the set of accepting states.
Here, kc and kp represent the number of capturing-group memories and positive-lookahead
memories, respectively. Next, we define configurations of PLMFAs.
▶ Definition 14 (Configuration). A configuration of a PLMFA M is a tuple (q, w, σc , σp )
where q is a state of M , w is an input string, and σc (resp. σp ) is a list of memories that
represents a list of capturing-group (resp. positive-lookahead) memories.
▶ Definition 15 (Computation step). For a PLMFA(kc , kp ) M and ℓ ∈ Σϵ ∪ [kc ], a step
ℓ

ℓ

of computation of M is a binary relation on configurations −→ (or −
→ or → if irrelevant),
M

ℓ

defined as follows: (q, w, σc , σp ) −→ (q ′ , w′ , σc′ , σp′ ) iff there is a transition δ(q, ℓ) ∋ (q ′ , irc , irp )
M

satisfying the following conditions.

FSCD 2022

<!-- page 10 -->

15:10

On Lookaheads in Regular Expressions with Backreferences

(1) If ℓ ∈ Σϵ , v = ℓ and otherwise (i.e., if ℓ ∈ [kc ]) if σc [ℓ].status = C then v = σc [ℓ].word.
(2) ∀τ ∈ {c, p}.∀i ∈ [kτ ].στ′ [i].status = στ [i].status ⊕ irτ [i].
(3) ∀τ ∈ {c, p}.∀i ∈ [kτ ].

(if O ⇒τ,i O )
 u :: v
στ′ [i].word =
στ [i].word (if _ ⇒τ,i C )

v
(if C ⇒τ,i O )
where στ [i].status ⇒τ,i στ′ [i].status, bts = σp [j].word where j = argmaxj∈J |σp [j].word| if
J = {j ∈ [kp ] | O ⇒p,j C } ̸= ∅ and otherwise bts = ϵ, and u = στ [i].word/bts if bts is a
suffix of στ [i].word and otherwise u = ϵ.
(4) w′ = (bts :: w)\v.
We write (q, w, σc , σp ) −
→∗ (q ′ , w′ , σc′ , σp′ ) for (q, w, σc , σp ) −
→ ··· −
→ (q ′ , w′ , σc′ , σp′ ). We give
an intuitive reading of the definition. Roughly, v is the string to be consumed by the step,
and (1) says that if ℓ ∈ Σϵ then ℓ is consumed, and otherwise ℓ is a capturing-group index
(i.e., ℓ ∈ [kc ]) and the string stored at the corresponding memory, i.e., σc [ℓ].word, is consumed
provided that the memory is closed. (2) and (3) stipulate how the statuses and strings of
the memories are updated, respectively. Importantly, (3) defines the backtrack string bts
to be used for backtracking caused by a closure of a positive lookahead memory (if any
happens in the step). Namely, bts is set to be the longest string stored in positive lookahead
memories closed by the step (bts = ϵ if no such closures happen), and is used in (3) to reset
the content of the memories that are open and remain so (i.e., those satisfying O ⇒τ,i O ) by
right-division (cf. the definition of u). The string contents remain unchanged for memories
that are or remain closed by the step (i.e., those satisfying _ ⇒τ,i C ), and for the rest of the
memories, the consumed string v is appended to their strings. (4) defines w′ , which is the
input string in the post configuration (i.e., the string to be consumed in the continuation of
the step), by prepending bts to the previous configuration’s input string w to account for any
backtracking that happens in the step, and consuming v by left-division. We remark that, for
any configuration reachable from an initial configuration (see below), σp [i].word is a prefix of
σp [j].word or vice versa for any i, j ∈ [kc ], thus ensuring that bts is uniquely determined.
An initial configuration is (q0 , w, σc,0 , σp,0 ), where στ,0 [i] = (ϵ, C ) for all i ∈ [kτ ] and
τ ∈ {c, p}. That is, every memory is initially closed and stores the empty string. A run of a
ℓ

PLMFA M is a sequence π such that π[1] is an initial configuration and π[i] −→ π[i + 1] for all
M

1 ≤ i < |π|. A run π is accepting if π[|π|] = (q, ϵ, _, _) for some q ∈ F . A string w is accepted
by M if M has an accepting run. The language of M , denoted by L(M ), is the set of strings
accepted by M . That is, L(M ) = {w ∈ Σ∗ | (q0 , w, σc,0 , σp,0 ) −
→∗ (q, ϵ, σc , σp ) ∧ q ∈ F }. We
note that when kp = 0, a PLMFA(kc , kp ) is a kc -memory automaton (MFA(kc ) or simply
MFA if kc is irrelevant) introduced by Schmid [14].
▶ Example 16. As an example, consider a run of the PLMFA M shown in Fig. 3, which
is equivalent to the rewblp (?=(1 ·∗ )1 z)·\1 described in Example 4. In the figure, the
(resp. double) circles represent (resp. accepting) states. The arrows represent transitions
and the words on the labels represent labels on the transitions except for (?=, (1 , )1 ,
and ). The arrow with the word (?= (resp. )) represents the transition δ(q1 , ϵ) ∋ (q2 , ⋄, o)
(resp. δ(q5 , ϵ) ∋ (q6 , ⋄, c)). Additionally, the arrow with the word (1 (resp. )1 ) represents the
transition δ(q2 , ϵ) ∋ (q3 , o, ⋄) (resp. δ(q3 , ϵ) ∋ (q4 , c, ⋄)). The rewblp expression contains just
one backreference and positive lookahead. For simplicity, we abbreviate the lists of memories
[(x, s )] as (x, s ).

<!-- page 11 -->

N. Chida and T. Terauchi

15:11

Figure 3 A PLMFA equivalent to the rewblp expression (?=(1 ·∗ )1 z)·\1.

Given an input string w=zz, the run of M is as follows: The run begins with
the initial configuration (q0 , zz, (ϵ, C ), (ϵ, C )). First, the initial configuration changes to
(q3 , zz, (ϵ, O ), (ϵ, O )) by applying the transitions δ(q1 , ϵ) ∋ (q2 , ⋄, o) and δ(q2 , ϵ) ∋ (q3 , o, ⋄)
in this order. The transition δ(q1 , ϵ) ∋ (q2 , ⋄, o) opens the positive-lookahead memory
and the configuration changes to (q2 , zz, (ϵ, C ), (ϵ, O )). The transition δ(q2 , ϵ) ∋ (q3 , o, ⋄)
opens the capturing-group memory and the configuration changes to (q3 , zz, (ϵ, O ), (ϵ, O )).
Next, the transitions δ(q3 , ·) ∋ (q3 , ⋄, ⋄), δ(q3 , ϵ) ∋ (q4 , c, ⋄), and δ(q4 , z) ∋ (q5 , ⋄, ⋄) are
applied in this order. For the first transition, the configuration changes to the configuration
(q3 , z, (z, O ), (z, O )) by consuming a character z. For the second transition, the configuration
changes to the configuration (q4 , z, (z, C ), (z, O )) by closing the capturing-group memory.
For the third transition, the configuration changes to the configuration (q5 , ϵ, (z, C ), (zz, O ))
by consuming a character z.
Then, the configuration changes to (q6 , zz, (z, C ), (zz, C )) by applying the transition
(q5 , ϵ) ∋ (q6 , ⋄, c). The transition closes the positive-lookahead memory and therefore it
simulates the backtracking behavior of the positive lookahead, i.e., it prepends the word of
the positive-lookahead memory zz to the input string. Finally, the configuration changes to
(q8 , ϵ, (z, C ), (zz, C )) by applying the transitions δ(q6 , ·) ∋ (q7 , ⋄, ⋄) and δ(q7 , 1) ∋ (q8 , ⋄, ⋄) in
this order. The transition δ(q6 , ·) ∋ (q7 , ⋄, ⋄) consumes a character z and the configuration
changes to (q7 , z, (z, C ), (zz, C )). Next, the current state q7 has the transition of the
backreference \1, i.e., δ(q7 , 1) ∋ (q8 , ⋄, ⋄). The transition tries to match the captured string,
i.e., z, with the current input string. Since the match succeeds, the configuration changes to
(q8 , ϵ, (z, C ), (zz, C )). Now, the current state q8 is an accepting state and the current input
string is ϵ, w is accepted by M .
▶ Remark 17. As seen above, capturing-group memories and positive-lookahead memories
exhibit an interesting symmetry: at their use, the content of a capturing-group (resp. positivelookahead) memory is left-divided from (resp. prepended to) the input string, and appended
to (resp. right-divided from) the strings stored in memories. The symmetry is imperfect
because positive-lookahead memories do not have “triggers” corresponding to backreferences
of capturing-group memories and a use of a positive-lookahead memory is always synchronous
with its closure. A perfect symmetry can be obtained by extending PLMFA with a new kind
of transitions that trigger positive-lookahead memory uses, disassociating them from closures.
The extension certainly does not decrease the expressive power of PLMFA and we conjecture
that it will strictly increase the expressive power.
We define conditions on PLMFA that correspond to RMRL and NUR of rewbl (cf. Sec. 2).
Note that we do not define conditions on PLMFA that correspond to NSR of rewbl because
PLMFAs already satisfy such a condition, i.e., PLMFAs do not allow to refer to the memory

FSCD 2022

<!-- page 12 -->

15:12

On Lookaheads in Regular Expressions with Backreferences

whose status is open. For convenience, we simply call these conditions RMRL and NUR.
Informally, a PLMFA satisfies RMRL if for all capturing-group memory (index) i and a state
q from which the ith memory can be backreferenced, there exist a unique pair of transitions
a and b that opened and closed the ith memory respectively so that the content of the ith
memory when the computation reaches q is what was recorded between a and b. Intuitively,
the pair of transitions correspond to the capturing group opening (i and closing )i of rewbl.
Next, we formalize RMRL for PLMFA. For a configuration ϖ and i ∈ [kc ], let us write
status(ϖ, i) for the status of the ith capturing-group memory of ϖ, i.e., σc [i].status where
ϖ = (_, _, σc , _).
▶ Definition 18 (Opening and Closing Transitions Pair). For q ∈ Q and i ∈ [kc ], a pair of
transitions (δ(p′ , ℓ′ ) ∋ (q ′ , irc′ , irp′ ), δ(p′′ , ℓ′′ ) ∋ (q ′′ , irc′′ , irp′′ )) = (a, b) is called an opening-andclosing-transitions pair of index i at state q if there exist a run π and 1 ≤ j1 < j2 < |π| such
that (1) the step from π[j1 ] to π[j1 + 1] takes the transition a and status(π[j1 ], i) = C , (2)
the step from π[j2 ] to π[j2 + 1] takes the transition b and status(π[j2 + 1], i) = C , (3) for all
j1 < l ≤ j2 , status(π[l], i) = O , and for all j2 < l ≤ |π|, status(π[l], i) = C , and (4) the state
of π[|π|] is q. We define RefSet M,i (q) (or RefSet i (q) if there is no danger of ambiguity) as
the set of opening-and-closing-transitions pairs of i at q on M .
▶ Definition 19 (RMRL-PLMFA). A PLMFA(kc , kp ) M = (Q, δ, q0 , F ) is called restricted
may repeat labels (RMRL) if for all (q, i) ∈ Q × [kc ] such that δ(q, i) ̸= ∅, |RefSet i (q)| ≤ 1.
Next, we define NUR for PLMFA. Informally, a PLMFA satisfies NUR if no capturinggroup memory can be backreferenced without capturing a word. Formally, for (q, i) ∈
Q × [kc ], we say that q is assigned with respect to index i on M , written Assigned M (q, i) (or
Assigned(q, i) if there is no danger of ambiguity), if for all runs π such that the state of π[|π|]
is q, there exists 1 ≤ j < |π| such that status(π[j], i) = O and status(π[j + 1], i) = C .
▶ Definition 20 (NUR-PLMFA). A PLMFA(kc , kp ) M = (Q, δ, q0 , F ) is no unassigned
reference (NUR) if for all (q, i) ∈ Q × [kc ] such that δ(q, i) ̸= ∅, Assigned(q, i) = true.
In what follows, we assume that PLMFAs satisfy RMRL and NUR.

4.2

Normal Forms and Nested Forms

We show that a PLMFA can be converted into certain forms. The normal form enforces two
restrictions: (1) only ϵ transitions can change memory statuses and at most one status of
the memory at a time, and (2) no transitions open (resp. close) a memory that is already
opened (resp. closed).
▶ Definition 21 (Normal Form). A PLMFA is in normal form if the following properties are
satisfied. For every transition δ(q, ℓ) ∋ (q ′ , irc , irp ), τ ∈ {c, p}, and j ∈ [kτ ], (1) if irτ [j] ̸= ⋄,
then ℓ = ϵ and irτ ′ [l].status = ⋄ for all (τ ′ , l) ∈ {c, p} × [kτ ′ ] such that (τ ′ , l) ̸= (τ, j), and
(2) there is no run π such that π[|π|] = (_, _, σc , σp ) where irτ [j] = o and στ [j].status = O
or irτ [j] = c and στ [j].status = C .
▶ Lemma 22. Any PLMFA M can be converted to a normal form PLMFA M ′ such that
L(M ) = L(M ′ ).
The proof is by adopting an analogous conversion of [14] and works by extending the states
of M to record memory statuses and splitting simultaneous memory updates to multiple
transitions. We remark that the conversion preserves RMRL and NUR.

<!-- page 13 -->

N. Chida and T. Terauchi

15:13

Next, we define nested form which enforces that there are no overlaps in any runs. A
run π is said to have an overlap if there exist 1 ≤ j1 < j2 < j3 < j4 < |π| such that
some memory is opened and closed respectively at the j1 th and the j3 th steps and some
memory (possibly the same) is opened and closed respectively at the j2 th step and the j4 th
step. There are four types of overlaps, cc, cp, pc, and pp, depending on the types of the
first and the second memories (e.g., cp-overlap is when the first memory is capturing-group
and the second is positive-lookahead). Intuitively, an overlap corresponds to an invalid
expression that has an overlap of capturing groups or positive lookaheads. For example,
cc-overlap corresponds to invalid expressions (i r1 (j r2 )i r3 )j and pc-overlap corresponds to
invalid expressions (?=r1 (i r2 )r3 )i .
▶ Definition 23 (Nested Form). A PLMFA M is in nested form if there are no overlaps in
any runs on M .
We show that we can transform a PLMFA to the nested form.
▶ Lemma 24. Any PLMFA M can be converted to a normal and nested form PLMFA M ′
such that L(M ) = L(M ′ ).
The proof is by adding new transitions that close the fragments of the memories which
are open before opening the transitions that cause overlaps and open the next fragments of
the memories after that. For cc-overlaps, the conversion coincides with the analogous one
for MFAs [14]. In what follows, without loss of generality, we assume that PLMFAs are in
normal and nested form.

4.3

From Rewblp to PLMFA

We show that given a rewblp expression r, we can construct a PLMFA M such that
L(r) = L(M ). The construction of the PLMFA extends the standard Thompson construction
from a pure regular expression to an NFA [15] with backreferences and lookaheads in a
mostly straightforward manner. For space, the construction and the proof of correctness is
omitted.
▶ Theorem 25. Let a rewblp r include kc capturing groups and kp positive lookaheads. Then,
there exists a PLMFA(kc , kp ) M such that L(r) = L(M ).

4.4

From PLMFA to Rewblp

Now, we show that given a PLMFA M , we can construct a rewblp expression r such that
L(M ) = L(r). We first give the conversion and then show the correctness. For space, we
only show the correctness of the language equivalence and omit the fact that the conditions
NUR, NSR, and RMRL are satisfied by the resulting rewblp . However, they can be proved
similarly to the language equivalence.
The conversion, referred to as PtoR, is inspired by that of MFAs to rewbs [14]. The idea
of the conversion is to use the nested relation of PLMFAs. Since PLMFAs are nested form,
the transitions of the opening-and-closing-transitions pairs have a nested structure, i.e., they
form a directed acyclic graph (DAG). Therefore, we can iteratively convert (sub)automaton
corresponding to the part of the given PLMFA delimited by each such pairs to the rewblp
expression in a topological order starting from the pairs that nest nothing. Each step of
the conversion makes an extended PLMFA (ePLMFA) whose labels are rewblp expressions.
That is, labels ℓ on transitions of PLMFAs are treated as rewblp expressions ℓ of ePLMFAs if

FSCD 2022

<!-- page 14 -->

15:14

On Lookaheads in Regular Expressions with Backreferences

ℓ ∈ Σϵ . Additionally, if ℓ = i ∈ [kc ] on the transitions of PLMFAs, ℓ = \i on the transitions
of ePLMFAs. For an ePLMFA M and a rewblp expression ℓ, a step of computation of
ℓ

M reading ℓ is defined as follows: (q, w, σc , σp ) −
→ (q ′ , w′ , σc′ , σp′ ) iff either ℓ = ϵ and
ϵ
′
′
′
′
→ (q , w , σc , σp ) according to Def. 15, or ℓ ̸= ϵ and there exist a transition
(q, w, σc , σp ) −
′ kc +kp
δ(q, ℓ) ∋ (q , ⋄
) and steps of computations from (q0′ , w, σc , σp ) to (qf′ , w′ , σc′ , σp′ ) of
′
′
′
M = (_, _, q0 , {qf }) obtained from ℓ by the construction from a rewblp expression to a
PLMFA mentioned in Sec. 4.3.
We also extend the NUR and RMRL conditions for ePLMFAs as follows. For all transition,
we reconstruct PLMFA from the rewblp label by applying the construction mentioned in
Sec. 4.3 and replace the transition with the PLMFA by replacing it with ϵ-labeled ⋄-only
transitions to and from the initial and the final state of the PLMFA. We say that an ePLMFA
satisfies NUR and RMRL if the reconstructed PLMFA satisfies NUR and RMRL, respectively.
We define the nesting relation. Firstly, for an ePLMFA and inst ∈ {o, c}, we define
Qτ,i,inst as the set of states q such that δ(q, _) ∋ (_, irc , irp ) where irτ [i] = inst. Let
Φ = {(τ, i, q, q ′ ) | τ ∈ {c, p} ∧ i ∈ [kτ ] ∧ q ∈ Qτ,i,o ∧ q ′ ∈ Qτ,i,c }. The nesting relation
≺ ⊆ Φ × Φ is defined as follows: (τ1 , i1 , q1 , q1′ ) ≺ (τ2 , i2 , q2 , q2′ ) iff there exist a run π
and s < t < u < v < π[|π|] such that the step from π[s] to π[s + 1] takes a transition
δ(q2 , _) ∋ (_, irc , irp ) where irτ2 [i2 ] = o, the step from π[t] to π[t + 1] takes a transition
δ(q1 , _) ∋ (_, irc , irp ) where irτ1 [i1 ] = o, the step from π[u] to π[u + 1] takes a transition
δ(q1′ , _) ∋ (_, irc , irp ) where irτ1 [i1 ] = c, and the step from π[v] to π[v + 1] takes a transition
δ(q2′ , _) ∋ (_, irc , irp ) where irτ2 [i2 ] = c.
For an ePLMFA M , the procedure of PtoR(M ) is defined as follows. Let us initialize
∆ : Φ → P(Φ) as follows: ∆(τ1 , i1 , q1 , q1′ ) = {(τ2 , i2 , q2 , q2′ ) | (τ2 , i2 , q2 , q2′ ) ≺ (τ1 , i1 , q1 , q1′ )}.
We iteratively update M , Φ, and ∆ by the following steps.
1. Find (τ, i, q, q ′ ) ∈ Φ such that ∆(τ, i, q, q ′ ) = ∅. Let δ(q, _) ∋ (qs , _) (resp. δ(q ′ , _) ∋
(qe , _)) be the opening (resp. closing) transition of (τ, i, q, q ′ ). Then, construct an ePLMFA
M ′ from M by replacing the initial state and set of accepting states with qs and {q ′ },
respectively, and deleting all transitions that open or close memories.
2. Convert M ′ to a rewblp expression r using the standard state elimination method.
3. Delete (τ, i, q, q ′ ) from Φ and (the domain and the range of) ∆, and add the transition
δ(q, (i r)i ) ∋ (qe , ⋄kc +kp ) if τ = c and otherwise δ(q, (?=r)) ∋ (qe , ⋄kc +kp ) to M .
4. Repeat steps 1 to 3 until convergence.
5. Delete all transitions that open or close memories from M .
6. Convert M to a rewblp expression r by the state elimination method and return r.
At step 1, we can find such a tuple (τ, i, q, q ′ ) since it is in nested form. The state elimination
method used in steps 2 and 6 is a straightforward adoption of the standard state elimination
method (see, e.g., [13]) that interprets the rewblp expression that appear as labels as ordinary
regular expressions.
We proceed to the proof of correctness.
▶ Definition 26. The language of an ePLMFA M = (Q, δ, q0 , F ) parameterized by a string
w ∈ Σ∗ , a capturing-group memory σc , and a positive-lookahed memory σp , denoted by
L(M, w, σc , σp ), is defined as
L(M, w, σc , σp )

=

{(w′ , σc′ , σp′ ) | ∃q ′ ∈ F. (q0 , w, σc , σp ) −
→∗ (q ′ , w′ , σc′ , σp′ )}.

For an ePLMFA M = (Q, δ, q0 , F ), we write M (q ′ , F ′ ) for the ePLMFA M ′ = (Q, δ, q ′ , F ′ )
where q ′ ∈ Q and F ′ ⊆ Q. We show that eliminating a state from an ePLMFA by applying
one step of the state elimination method does not change the parameterized language of the
ePLMFA. The state elimination method eliminates a state by deleting and adding transitions
as shown in Fig. 4.

<!-- page 15 -->

N. Chida and T. Terauchi

15:15

Figure 4 (A) Before eliminating the state q2 . (B) After eliminating the state q2 .

▶ Lemma 27. Let M = (Q, δ, q0 , F ) be an ePLMFA(kc , kp ) such that for all transitions
δ(q, ℓ) ∋ (q ′ , irc , irp ), i ∈ [kc ], and j ∈ [kp ], irc [i] = ⋄ and irp [j] = ⋄. Additionally, let
M ′ = (Q′ , δ ′ , q0′ , F ′ ) be the ePLMFA obtained by eliminating a state in Q from M by the state
elimination method. Then, for all w ∈ Σ∗ , σc , and σp , L(M, w, σc , σp ) = L(M ′ , w, σc , σp ).
Proof. Let us assume that q1 and q3 are in Q and Q′ , q2 ∈ Q is the state eliminated
by the state elimination method as shown in Fig. 4. Since the only difference between
M and M ′ comes from the state q2 , it suffices to show that L(M (q1 , {q3 }), w, σc , σp ) =
L(M ′ (q1 , {q3 }), w, σc , σp ). It is immediate from the construction.
◀
▶ Theorem 28. For an ePLMFA M , let Mj be the ePLMFA obtained after the jth iteration
of steps 1 to 3 of PtoR(M ). We assume Mj = M if j = 0. Then, for all w ∈ Σ∗ , σc , and
σp , L(M, w, σc , σp ) = L(Mj , w, σc , σp ).
Proof. The proof is by induction on the number of the iteration j of step 1 to 3. The
base step j = 0 is immediate since Mj = M . For the induction step, let j > 0. The
inductive hypothesis is that L(M, w, σc , σp ) = L(Mj , w, σc , σp ) holds for all j, w, σc , and
σp . We then consider Mj+1 . We show L(Mj , w, σc , σp ) = L(Mj+1 , w, σc , σp ). As described
in the step 3, for the ePLMFA Mj = (Q, δ, q0 , F ), Mj+1 = (Q, δ ∪ {t}, q0 , F ) where t =
δ(q, r′ ) ∋ (qe , ⋄kc +kp ). Recall that r′ = (i r)i or (?=r) and r is the rewblp expression
obtained at step 2. For this, L(Mj , w, σc , σp ) ⊆ L(Mj+1 , w, σc , σp ) is immediate. For
L(Mj+1 , w, σc , σp ) ⊆ L(Mj , w, σc , σp ), it suffices to show that for all w, σc , and σp , if
r′

there exists (q, w, σc , σp ) −→ (qe , w′ , σc′ , σp′ ) on Mj+1 , then there exists (q, w, σc , σp ) −
→∗
r′

(qe , w′ , σc′ , σp′ ) on Mj . We assume that there exists π = (q, w, σc , σp ) −→ (qe , w′ , σc′ , σp′ )
on Mj+1 . The computation of the transition whose label is r′ is defined by that of the
ePLMFA Mr′ obtained from r′ by the construction mentioned in Sec. 4.3. If r′ = (i r)i ,
Mr′ = (Qr′ , δr′ , q, {qe }) where
Qr′ = {q, qe } ∪ Qr ; and
δr′ = δr ∪ {((q, ϵ), {(q0,r , irc , ⋄kp )})} ∪ {((qF,r , ϵ), {(qe , irc′ , ⋄kp )})} where irc [k] = o and
irc′ [k] = c if k = i and otherwise irc [k] = ⋄ and irc′ [k] = ⋄ for k ∈ [kc ]
with the ePLMFA for r Mr = (Qr , δr , q0,r , {qF,r }). By the transitions in δr′ , π =
r
(q, w, σc , σp ) −
→ (q0,r , w, σc′′ , σp ) −
→ (qF,r , w′ , σc′′′ , σp′ ) −
→ (qe , w′ , σc′ , σp′ ) where σc′′ [k] = (ϵ, O )
′′′
′
and σc [k] = (σc [k].word, O ) if k = i and otherwise σc′′ [k] = σc [k] and σc′′′ [k] = σc′ [k]. We
r
→ (qF,r , w′ , σc′′′ , σp′ ). The label r is constructed
focus on the computation (q0,r , w, σc′′ , σp ) −
from M ′ by the state elimination method at step 2. Additionally, by Lemma 27, the state
elimination method preserves the parameterized language equivalence. For this, there exists
(qs , w, σc′′ , σp ) −
→∗ (q ′ , w′ , σc′′′ , σp′ ) on M ′ . Since the transition function of Mj includes all
transitions of M ′ , there also exists (qs , w, σc′′ , σp ) −
→∗ (q ′ , w′ , σc′′′ , σp′ ) on Mj . As described
at step 1, Mj has a transition from q to qs that opens the ith capturing-group memory

FSCD 2022

<!-- page 16 -->

15:16

On Lookaheads in Regular Expressions with Backreferences

and a transition from q ′ to qe that closes the ith capturing-group memory. Therefore,
there exists (q, w, σc , σp ) −
→ (qs , w, σc′′ , σp ) −
→∗ (q ′ , w′ , σc′′′ , σp′ ) −
→ (qe , w′ , σc′ , σp′ ). For this,
L(Mj , w, σc , σp ) = L(Mj+1 , w, σc , σp ). By the inductive hypothesis, L(Mj , w, σc , σp ) =
L(M, w, σc , σp ). Thus, L(M, w, σc , σp ) = L(Mj+1 , w, σc , σp ) for the case r′ = (i r)i . The case
of r′ = (?=r) is analogous.
◀
Now, we obtain our main result.
▶ Theorem 29. Let M be an ePLMFA. Then, L(M ) = L(r) where r = PtoR(M ).
Proof. Let M ′ be the ePLMFA at the last step of the state elimination method at step 6 of
PtoR. Then, M ′ = ({q, q ′ }, {((q, r), {(q ′ , ⋄kc +kp )})}, q, {q ′ }). By Theorem 25, L(M ′ ) = L(r).
By Theorem 28, for all w ∈ Σ∗ , L(M ′ , w, σc,0 , σp,0 ) = L(M, w, σc,0 , σp,0 ), i.e., L(M ′ ) = L(M ).
Thus, L(M ) = L(M ′ ) = L(r).
◀
As a corollary of Theorem 25 and Theorem 29, we obtain the following result.
▶ Corollary 30. The expressive power of PLMFA is equivalent to that of rewblp .

5

Related Work

Among the major extensions employed by modern real-world regular expression engines are
backreferences and lookaheads. However, the previous works on formal language theory have
studied the two features mostly in isolation, and to the best of our knowledge, our work is
the first formal study of regular expressions extended with both features. Next, we discuss
previous works that studied the features in isolation.
Prior works by Morihata and Berglund et al. [12, 3] showed that extending regular
expressions by lookaheads does not enhance their expressive power. Their proofs are by
a translation to boolean finite automata [4] whose expressive power is regular. However,
adopting such an approach to defining an equivalent automata is difficult in the presence
of backreferences because boolean automata express lookaheads by running several states
simultaneously without backtracking, while the combination of lookaheads and backreferences
intrinsically requires backtracking. For example, to match against (?=·(·)1 )\1·, a boolean
approach would run (?=·(·)1 ) and \1· simultaneously, but then the automaton would get
stuck while trying to process the backreference \1 as it is unassigned at that point. By
constrast, our PLMFA uses the novel positive-lookahead memories to store enough information
to simulate the backtracking behavior of positive lookaheads.
A formal study of regular expressions with backreferences (rewb) dates back to the
seminal work by Aho [1]. More recently, a formal semantics and a pumping lemma were
given by Câmpeanu et al. [5]. Berglund and van der Merwe [2] showed that different variants
of backreference semantics give rise to differences in expressive powers. Our works adopts
and formalizes the no-label-repetitions (NLR) and no-unassigned-reference (NUR) semantics
which is also used in [5, 6]. Schmid [14] proposed MFA, and showed that the expressive power
of the automata is equivalent to that of rewb. Our PLMFA builds on MFA and extends
it with positive-lookahead memories to handle positive lookaheads. As remarked before
(cf. Remark 17), our positive-lookahead memory exhibits an interesting symmetry to the
capturing-group memory of MFA.

<!-- page 17 -->

N. Chida and T. Terauchi

6

15:17

Conclusion

We have studied the expressive powers of regular expressions with the popular backreferences
and lookaheads extensions. We have shown that extending rewb by positive or negative
lookaheads enhance their expressive power. Additionally, we have presented languagetheoretic properties of rewb extended by the two forms of lookaheads, and have presented a
new class of automata called PLMFA that is equivalent in expressive power to rewblp . We
have introduced a new kind of memories called a positive-lookahead memory, which is almost
perfectly symmetric to capturing-group memory of MFA, as a key component of PLMFA.
Despite the popularity of the backreference and lookaheads extensions in practice, to our
knowledge, our work is the first formal study on regular expressions with both extensions.
We hope that our results pave the way for more work on the topic.
References
1

2

3
4

5

6

7
8

9
10

11

Alfred V. Aho. Chapter 5 - algorithms for finding patterns in strings. In Jan van Leeuwen,
editor, Algorithms and Complexity, Handbook of Theoretical Computer Science, pages 255–300.
Elsevier, Amsterdam, 1990. doi:10.1016/B978-0-444-88071-0.50010-2.
Martin Berglund and Brink van der Merwe. Regular expressions with backreferences reexamined. In Jan Holub and Jan Zdárek, editors, Proceedings of the Prague Stringology
Conference 2017, Prague, Czech Republic, August 28-30, 2017, pages 30–41. Department of
Theoretical Computer Science, Faculty of Information Technology, Czech Technical University
in Prague, 2017. URL: http://www.stringology.org/event/2017/p04.html.
Martin Berglund, Brink van der Merwe, and Steyn van Litsenborgh. Regular expressions with
lookahead. J. Univers. Comput. Sci., 27(1):324–340, 2021. doi:10.3897/jucs.66330.
Janusz A. Brzozowski and Ernst L. Leiss. On equations for regular languages, finite automata,
and sequential networks. Theor. Comput. Sci., 10:19–35, 1980. URL: http://dblp.uni-trier.
de/db/journals/tcs/tcs10.html#BrzozowskiL80.
Cezar Câmpeanu, Kai Salomaa, and Sheng Yu. A formal study of practical regular expressions.
International Journal of Foundations of Computer Science, 14(06):1007–1018, 2003. doi:
10.1142/S012905410300214X.
Benjamin Carle and Paliath Narendran. On extended regular expressions. In Adrian-Horia
Dediu, Armand-Mihai Ionescu, and Carlos Martín-Vide, editors, Language and Automata
Theory and Applications, Third International Conference, LATA 2009, Tarragona, Spain, April
2-8, 2009. Proceedings, volume 5457 of Lecture Notes in Computer Science, pages 279–289.
Springer, 2009. doi:10.1007/978-3-642-00982-2_24.
Ashok K. Chandra and Larry J. Stockmeyer. Alternation. In 17th Annual Symposium on
Foundations of Computer Science (sfcs 1976), pages 98–108, 1976. doi:10.1109/SFCS.1976.4.
N. Chida and T. Terauchi. Repairing dos vulnerability of real-world regexes. In Proceedings of
the 43rd IEEE Symposium on Security and Privacy, SP 2022, pages 1049–1066, Los Alamitos,
CA, USA, May 2022. IEEE Computer Society. doi:10.1109/SP46214.2022.00061.
S. C. Kleene. Representation of events in nerve nets and finite automata. In Automata Studies.
1956.
Vladimir Komendantsky. Matching problem for regular expressions with variables. In HansWolfgang Loidl and Ricardo Peña, editors, Trends in Functional Programming - 13th International Symposium, TFP 2012, St. Andrews, UK, June 12-14, 2012, Revised Selected
Papers, volume 7829 of Lecture Notes in Computer Science, pages 149–166. Springer, 2012.
doi:10.1007/978-3-642-40447-4_10.
Blake Loring, Duncan Mitchell, and Johannes Kinder. Sound regular expression semantics
for dynamic symbolic execution of javascript. In Proceedings of the 40th ACM SIGPLAN
Conference on Programming Language Design and Implementation, PLDI 2019, pages 425–438,
New York, NY, USA, 2019. Association for Computing Machinery. doi:10.1145/3314221.
3314645.

FSCD 2022

<!-- page 18 -->

15:18

On Lookaheads in Regular Expressions with Backreferences

12
13
14
15

Akimasa Morihata. Translation of regular expression with lookahead into finite state automaton.
Computer Software, 29(1):1_147–1_158, 2012. doi:10.11309/jssst.29.1_147.
Jacques Sakarovitch. Elements of Automata Theory. Cambridge University Press, USA, 2009.
Markus L. Schmid. Characterising REGEX languages by regular languages equipped with
factor-referencing. Inf. Comput., 249:1–17, 2016. doi:10.1016/j.ic.2016.02.003.
Ken Thompson. Programming techniques: Regular expression search algorithm. Commun.
ACM, 11(6):419–422, June 1968. doi:10.1145/363347.363387.
