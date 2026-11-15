---
file: ../pdfs/2010-ciaa-okui-disambiguation-position-automata.pdf
citation: "Disambiguation in Regular Expression Matching via Position Automata with Augmented Transitions; Okui, Suzuki; CIAA 2010 (LNCS 6482)"
doi: 
source: "https://u-aizu.ac.jp/files/page/research/techreport/2013-002.pdf"
topics: [semantics_theory_access]
pages: 33
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Technical Report 2013-002

Disambiguation in Regular Expression Matching via
Position Automata with Augmented Transitions

Satoshi Okui1 and Taro Suzuki2
June 5, 2013

1

Depertment of Computer Science
Chubu University
Matsumoto 1200, Kasugai City
Aichi, 487-8501 Japan

2

Department of Computer Science
and Engineering
The University of Aizu
Tsuruga, Ikki-Machi,
Aizu-Wakamatsu City
Fukushima, 965-8580 Japan

<!-- page 2 -->

Technical Report 2013-002
Title:
Disambiguation in Regular Expression Matching via Position Automata with Augmented
Transitions

Authors:
Satoshi Okui and Taro Suzuki
Key Words and Phrases:
regular expression, automata, pattern matching

Abstract:
This paper offers a new efficient regular expression matching algorithm which follows the
leftmost-longest rule specified in POSIX 1003.2 standard. The algorithm basically emulates
the subset construction without backtracking, so that its computational cost even in the worst
case does not explode exponentially; the time complexity of the algorithm is O(m(n2 +c)), where
m is the length of a given input string, n the number of occurrences of the most frequently
used letters in a given regular expression and c the number of subexpressions to be used for
capturing substrings plus the number of repetition operators. The correctness of the algorithm
is given with respect to a formalization of the leftmost-longest semantics by means of a priority
order on parse trees.

Report Date:
6/5/2013

Written Language:
English

Any Other Identifying Information of this Report:

Distribution Statement:
First Issue: 5 copies

Supplementary Notes:

Language Processing Systems Laboratory
The University of Aizu
Aizu-Wakamatsu
Fukushima 965-8580
Japan

<!-- page 3 -->

Disambiguation in Regular Expression
Matching via Position Automata with
Augmented Transitions ∗
Satoshi Okui
Department of Computer Science, Chubu University
okui@cs.chubu.ac.jp
Taro Suzuki
School of Computer Science and Engineering, The University of Aizu
taro@u-aizu.ac.jp

Abstract
This paper offers a new efficient regular expression matching algorithm
which follows the leftmost-longest rule specified in POSIX 1003.2 standard.
The algorithm basically emulates the subset construction without backtracking, so that its computational cost even in the worst case does not explode
exponentially; the time complexity of the algorithm is O(m(n2 + c)), where
m is the length of a given input string, n the number of occurrences of the
most frequently used letters in a given regular expression and c the number
of subexpressions to be used for capturing substrings plus the number of repetition operators. The correctness of the algorithm is given with respect to a
formalization of the leftmost-longest semantics by means of a priority order
on parse trees.

∗ This work is supported by Japan Society for Promotion of Science,

No.22500019.

1

Basic Research (C)

<!-- page 4 -->

1 Introduction
Disambiguation in regular expression matching is crucial for many applications
such as string replacement in document processing. Although POSIX 1003.2 standard requires the ambiguity of regular expressions to be resolved by following
the leftmost-longest rule, most POSIX adopting implementations actually indicate,
more or less, different behaviors. This is, in part, due to the description of the
leftmost-longest rule, which is informally given in natural language, not in any
formal method.
For implementations that rely on backtracking [12], strictly following the leftmostlongest rule forces loss of efficiency; the greedy, or the first match, policy is easier
for such systems to follow. For this problem, some recent implementations [10,
9, 3] take approaches based on automata, realising the leftmost-longest semantics very efficiently. While those implementations are notable for their production
level quality, theoretical considerations supporting the correctness of those approaches have not yet given sufficiently; we can find such a fundamental study [6]
for the greedy semantics though.
This paper presents a new regular expression matching algorithm which follows the leftmost-longest semantics. We focus on reliability issues so that a significant part of our study is devoted to proving the correctness of the algorithm
as well as formalizing the leftmost-longest semantics. The time complexity of our
algorithm is O(m(n2 + c)) where m is the length of a given input string, n the number of occurrences of the most frequently used letters in a given regular expression
and c the number of subexpressions to be used for capturing portions of the input
string plus the number of repetition operators.
Our discussion for achieving that algorithm proceeds in three steps as follows.
First, in Sec. 2, we formalize the leftmost-longest rule based on parse trees. A parse
tree for a given input string represents a way of accepting that string; e.g., which
alternative is selected or how many times each iteration is performed, and so forth.
Imposing a priority order on parse trees results in a straightforward interpretation
of the leftmost-longest rule, which serves as a basis for our later discussion. We
restrict ourselves to consider canonical parse trees for avoiding the matches which
is unacceptable in the leftmost-longest semantics. Next, Sec. 3 introduces a slight
extension of traditional position automata [11, 7] in order to enumerate canonical
parse trees. In Sec. 4, we introduce the regular expression matching algorithm,
which basically performs subset construction at runtime. This algorithm incrementally compares, in each step, the priority of any two paths to eliminate unnecessary ones, so that the number of paths in each step never exceeds the number of
states. The validity of this incremental pruning is stated with respect to the formal
definition of the leftmost-longest semantics. Finally, Sec. 5 is devoted to a brief
comparison with other studies.

2

<!-- page 5 -->

2 Formalizing the leftmost-longest semantics
2.1 Basics
A regular expression is an expression of the form: r ::= 1 | a | r ·r | r +r | r ∗ where a is
a letter in an alphabet Σ. Our definition does not include the regular expression 0
that denotes the empty language. We consider the regular expressions as abstract
syntax trees (AST) and do not take care of operator precedence. The set of parse
trees for r, written PT(r ), is defined as follows:
PT(1) = {1}
PT( a) = { a}
PT(r1 ·r2 ) = {t1 ·t2 | ti ∈ PT(ri ) for i = 1, 2}
PT(r1 +r2 ) = {L(t) | t ∈ PT(r1 )} ∪ {R(t) | t ∈ PT(r2 )}
PT(r ∗) =

∪

{I(t1 , . . . , tn ) | t1 , . . . , tn ∈ PT(r )}

n ≥0

Note that parse trees are unranked in the sense that an I-node has an arbitrary
number of children (possibly none). Using unranked trees has more benefits than
using ranked trees; especially, it makes a close relationship with the original regular expression much clearer. Basically, parse trees are obtained from the AST of
a regular expression by horizontally expanding children of ∗-nodes. Hence, the
height of parse trees for r never exceeds that of r.
Obviously, a leaf of a parse tree is either 1 or a letter in Σ. Reading those
letters from left to right, we obtain a string derived from that parse tree. We write
PT(r, w) for the set of canonical parse trees deriving w. We sometimes specify each
character of w by 0-based index; we write w[i ] (0 ≤ i < |w|) for the i-th character.
Likewise, w[i, j] stands for a substring w[i ] . . . w[ j − 1] of w.
A position is a possibly empty sequence of natural numbers, which specify the
“path” from the root node to a subtree. The root position is the empty sequence
denoted as Λ. Following the standard manner, , e.g., [1], we assign a position to
each node of parse trees, except that the child of the R-node has a position starting
with 2 rather than 1. (In other words, we assume some place holder at the place
of the first sibling of R-node.) For other kind of nodes, the first sibling always
has a position starting with 1. For a parse tree t, we denote the set of positions
in t as Pos(t) and the subterm of t at position p as t| p . The root node of t| p and
the number of children (immediate subtrees) of t| p are respectively denoted by
nodet ( p) and arityt ( p).
Likewise, we consider Pos(r ) and r | p for the AST of a regular expression r.
We write p1 l p2 iff p1 precedes p2 in the lexicographic order, while p1 4 p2 if
p1 is a prefix of p2 , i.e., p1 q = p2 for some possibly empty q. We also write p1 ≺ p2
for p1 4 p2 and p1 , p2 .
3

<!-- page 6 -->

A binary relation R on a set A is called strict total order if it is (1) transitive, i.e.,
aRb & bRc implies aRc for any a, b, c ∈ A, and is (2) trichotomous, i.e., exactly one
of aRb, bRa and a = b holds. Note that any irreflexive (i.e., aRa does not holds for
any a ∈ A), trasitive binary relation R on A is a strict total order if either aRb or
bRa holds for any a, b ∈ A such that a , b.
Let t be a parse tree. The norm of t at p ∈ Pos(t), written ∥ t ∥ p , is the number
of letters in Σ that occur in t| p . As a convention, we define ∥ t ∥ p = −1 for p not
in Pos(t). ∥ t ∥Λ is abbreviated as ∥ t ∥.
Let s, t ∈ PT(r, w). A position p ∈ Pos(s) ∪ Pos(t) is called a distinct position
of s and t unless p ∈ Pos(s) ∩ Pos(t). For a distinct position of s and t, we have
∥ s ∥ p > ∥ t ∥ p = −1 or else ∥ t ∥ p > ∥ s ∥ p = −1. The following lemma is just the
contraposition of this:
Lemma 1 Let s, t ∈ PT(r, w) be parse trees for a regular expression r and w a string. For
any p ∈ Pos(s) ∪ Pos(t), ∥ s ∥ p = ∥ t ∥ p implies p ∈ Pos(s) ∩ Pos(t).
The following lemma is used everywhere in this paper.
Lemma 2 Let s, t ∈ PT(r, w) be parse trees for a regular expression r and a string w. The
following statements are equivalent:
1. s = t
2. ∥ s ∥ p = ∥ t ∥ p for any p ∈ Pos(s) ∪ Pos(t)
3. Pos(s) = Pos(t)
P ROOF 1 ⇒ 2 is obvious. 2 ⇒ 3 follows by L EMMA 1. What remains is 3 ⇒ 1,
which is shown by structual induction on r. Suppose Pos(s) = Pos(t). The cases
noder (Λ) = 1 and noder (Λ) = · are trivial. In case noder (Λ) = +, s and t should
have the same root symbol, say L: s = L(s1 ) and t = L(t1 ). On the other hand, if
noder (Λ) = ∗, the root of s and t should have the same arity: s = I(s1 , . . . , sn ) and
t = I(t1 , . . . , tn ). In both cases, the result follows by induction hypothesis. 

2.2 Formalizing Leftmost-Longest Matching
As we have mentioned, the POSIX specification presents the leftmost-longest rule
rater informally. We first rephrase it in a more formal way in order to make our
discussion more accurate.
First of all, we consider the case of whole matching; i.e., when a given regular
expression matches over the whole string. In this case, the POSIX specification [8]
requires that “each subpatterns shall match the longest possible string.” This leads the
following definition of priority:
4

<!-- page 7 -->

Definition 1 (priority of parse trees) Let r be a regular expression, and w a string. For
any t1 , t2 ∈ PT(r, w), we say t1 is prior to t2 with respect to the decision position
p ∈ Pos(t1 ), written t1 l p t2 , if the following conditions are satisfied:
1. ∥ t1 ∥ p > ∥ t2 ∥ p
2. ∥ t1 ∥q = ∥ t2 ∥q for any position q ∈ Pos(t1 ) ∪ Pos(t2 ) such that q l p
We write t1 l t2 if t1 l p t2 for some p.
Recall that we have defined ∥ t ∥ p = −1 for p < Pos(t), which corresponds to
the requirement [8]: “a null string shall be considered to be longer than no match at all.”
The definition of priority is top-down. We next give another characterization
allowing us to compare parse trees in rather left-to-right manner.
Definition 2 (first dictinct position) Let r be a regular expression, w a string, and s, t ∈
PT(r, w). A distinct position p of s and t is called the first distinct position if it satifies
p l p′ for any other distinct position p′ of s and t.
Proposition 1 Let s, t ∈ PT(r, w) be parse trees for a regular expression r and a string w
such that s l p t for some decision position p. The first distinct position q of s and t satisfies
the following:
1. s|r = t|r for any r such that r l q but r ̸4 q, and
2. p 4 q.
P ROOF We show the first statement; the second then follows immediately. Consider a position r such that r l q but r ̸4 q. Since r is not a distinct position, i.e.,
r ∈ Pos(s) ∩ Pos(t), there exist both s|r and t|r . Consider a position r ′ ∈ Pos(s|r ).
Since rr ′ is not a distinct position, either, we have rr ′ ∈ Pos(t), hence, r ′ ∈ Pos(t|r ).
Nemely, we have Pos(s|r ) ⊆ Pos(t|r ). Similaly, we have Pos(t|r ) ⊆ Pos(s|r ).
Therefore, L EMMA 2 gives s|r = t|r . 
We need one more requirement for parse trees. The POSIX specification [8]
states that “a subexpression repeated by ’*’ shall not match a null expression unless this
is the only match for the repetition.” For this, we introduce the notion of canonical
parse tree, which plays a very important role in our discussion: 1
Definition 3 (canonical parse tree) A parse tree t is called canonical if any subterm of t
such as I(t1 , . . . , tn ), n ≥ 2, satisfies ∥ ti ∥ > 0 for any 1 ≤ i ≤ n.
1 The interval expression (bounded repetition), if it would be introduced directly to our frame-

work, should be distingished from the repetition; otherwise, 1{2} would match the empty string,
yielding a non-canonical parse tree I(11).

5

<!-- page 8 -->

We write CPT(r ) for the canonical parse trees in PT(r ). More formally, it is given
as follows:
CPT(1) = {1}
CPT( a) = { a}
CPT(r1 ·r2 ) = {t1 ·t2 | ti ∈ CPT(ri ) for i = 1, 2}
CPT(r1 +r2 ) = {L(t) | t ∈ CPT(r1 )} ∪ {R(t) | t ∈ CPT(r2 )}
CPT(r ∗) =

∪

n ≥0

{I(α1 . . . αn ) | α1 , . . . , αn ∈ CPT(r ),
and ∥ a1 ∥ > 0, . . . , ∥ an ∥ > 0 if n ≥ 2 }

Likewise, CPT(r, w) means the canonical parse trees inPT(r, w).
Unlike PT(r, w), the set CPT(r, w) is finite. More specifically, we have the following:
Lemma 3 Let r be a regular expression r, w a string and t a parse tree in CPT(r, w). Any
node of t has at most |w| + 2 children.
P ROOF We only have to check I-nodes. Suppose, on the contrary, an I-node at
a position p ∈ Pos(t) has more than |w| + 2 children. Since t is canonical and
|w| + 2 ≥ 2, we have ∥ t ∥ ≥ ∥ t ∥ p > |w| + 2, which contradicts to ∥ t ∥ = |w|.

In practice, partial matching, matching for a substring of a given string, is used
more extensively than whole matching. On partial matching, the specification
states that “the search for a matching sequence starts at the beginning of a string and stops
when the first sequence matching the expression is found” and that “if the pattern permits
a variable number of matching characters and thus there is more than one such sequence
starting at that point, the longest such sequence is matched,” which we formalize as
follows.
Given a regular expression r, a parse configuration for r is defined as a triple
⟨u, t, v⟩ where u, v ∈ Σ∗ and t ∈ CPT(r ). We say a parse configuration ⟨u, t, v⟩
derives a string uwv if the canonical parse tree t derives the string w. The set
of parse configurations for r is denoted by PC(r ), and, likewise, the set of parse
configurations deriving w by PC(r, w). For any parse configurations ⟨u1 , t1 , v1 ⟩
and ⟨u2 , t2 , v2 ⟩ in PC(r, w), we write ⟨u1 , t1 , v1 ⟩ l ⟨u2 , t2 , v2 ⟩ if (1) |u1 | < |u2 | or (2)
|u1 | = |u2 | and ∥ t1 ∥ > ∥ t2 ∥ or (3) |u1 | = |u2 |, ∥ t1 ∥ = ∥ t2 ∥ and t1 l t2 .
Now, we have the main result of this section.
Theorem 1 PC(r, w) is a finite and strict total order set for any regular expression r and
a string w.

6

<!-- page 9 -->

P ROOF The finiteness of PC(r, w) immediately follows by the finiteness of CPT(r, w).
This is, in turn, a consequence of L EMMA 3 since the height of parse trees in
PC(r, w) does not exceed the height of (the AST of) r.
We show that PT(r, w) is a strict total order set; the result then follows since
the ordering l on PC(r, w) is an lexicographic extension of that on PT(r, w). To
show the transitivety, consider parse trees t1 , t2 , t3 in PT(r, w) such that t1 l p1 t2
and t2 l p2 t3 . Suppose p1 l p2 or p1 = p2 . Then, we have ∥ t1 ∥ p1 > ∥ t2 ∥ p1 ≥
∥ t3 ∥ p1 . Moreover, for any position q ∈ Pos(t1 ) ∪ Pos(t3 ) such that q l p1 , we have
∥ t1 ∥q = ∥ t2 ∥q = ∥ t3 ∥q . Hence, D EFINITION 1 gives us t1 l p1 t3 . Similary, for
the case p2 l p1 we also have t1 l p2 t3 . Therefore, the transitivity has been shown.
The irriflexivity is an consequence of L EMMA 2. This lemma also implies that
either t1 l t2 or t2 l t1 is true for any t1 , t2 ∈ PC(r, w) such that t1 , t2 . Therefore,
(PC(r, w), l) is trichotomous, hence, a strict total order set. 
Corollary 1 PC(r, w) has the least (that is, the most prior) parse configuration.
We think of the least element as representing the matching that follows the POSIXtype leftmost-longest semantics.
Note that the least element may not exist for arbitrary parse trees; e.g., in
PC(1∗, ε), we would have an infinite decreasing sequence of arbitrary parse trees:
I() m I(1) m I(11) m I(111) m . . .

2.3 Capturing submatches
A pattern matching procedure needs to report, if the mathcing succeeds, not only
the result of whole matching but also the result of submatch for each subpatterns.
In our formalization, this is nothing but specifying the correspodence between
positions between a regular expression and the least canonical parse tree for the
regular expression. The problem here is that this correspondence is one-to-many
as the subtree of ∗-node in the regular expression is extended to zero or many
subtrees in the parse tree. POSIX solution for this is to focus on the latest iteration.
To formalize this, we consider unfolding of positions:
Definition 4 Let r be a regular expression and t ∈ CPT(r ). For p in Pos(r ), we define

7

<!-- page 10 -->

unfold( p, t) in Pos(t) as follows:
unfold(Λ, t) = Λ
unfold(1.p, L(t)) = 1.unfold( p, t)
unfold(2.p, R(t)) = 2.unfold( p, t)
unfold(i.p, t1 ·t2 ) = i.unfold( p, ti ) i = 1, 2
unfold(1.p, I(t1 , . . . , tn )) = n.unfold( p, tn ) n ≥ 1
Definition 5 Let t be a parse tree and p ∈ Pos(t). The location loc( p, t) is defined as
follows:
loc(Λ, t) = 0
loc(1.p, L(t)) = loc( p, t)
loc(2.p, R(t)) = loc( p, t)
loc(1.p, t1 ·t2 ) = loc( p, t1 )
loc(2.p, t1 ·t2 ) = ∥ t1 ∥ + loc( p, t2 )
i −1

loc(i.p, I(t1 , . . . , tn )) = ∑ ∥ t j ∥ + loc( p, ti )
j =1

For a parse configuration c = ⟨u, t, v⟩ where t ∈ CPT(r, w) and a position p ∈
Pos(r ), we say c captures the interval (i, j) at a position p, where 0 ≤ i ≤ j ≤ |uw|,
if q = unfold( p, t) ∈ Pos(t), i = |u| + loc(q, t), and j = i + ∥ t|q ∥; otherwise, c
captures nothing at the position p.
Definition 6 (POSIX-type pattern matching problem) Let r be a regular expression and
P a subset of Pos(r ) called capturing positions. A POSIX-type pattern matching problem is, given a string w, to find the least canonical parse configuration ⟨u, t, v⟩, where
t ∈ CPT(r ), and to report intervals captured by positions in P.

8

<!-- page 11 -->

3 Enumerating Parse Trees via Position Automata
3.1 Correctly Nested Parentheses
A parenthesis expression means a sequence of either parentheses, each of which is
indexed by a position, or a letter in Σ. For a parenthesis expression α, its norm
∥ α ∥ is defined as the number of letters in α.
For each parse tree t in PT(r ), we assign, as its string representation, a parenthesis expression Φ( p, t) as follows:
Φ( p, 1) = ( p )p
Φ( p, a) = ( p a )p where a ∈ Σ
Φ( p, t1 ·t2 ) = ( p Φ( p.1, t1 ) Φ( p.2, t2 ) )p
Φ( p, L(t)) = ( p Φ( p.1, t) )p
Φ( p, R(t)) = ( p Φ( p.2, t) )p
Φ( p, I(t1 , . . . , tn )) = ( p Φ( p.1, t1 ) . . . Φ( p.1, tn ) )p
Definition 7 (correctlly nested parenthesis expression) A parenthesis expression α is correctlly nested if α = Φ( p, t) for some regular expression r, some position p and t ∈
CPT(r ).
Obviously, a correctlly nested parenthesis expression Φ( p, t) either of the form
( p a)p or ( p α1 . . . αn )p (n ≥ 0) and α1 , . . . , αn contains indexes less than p.
Definition 8 For a correctlly nested parenthesis expression α, the set Pos(α) of positions
of α is defined as follows:
Pos(( p a)p ) = {Λ}
Pos(( p )p ) = {Λ}
Pos(( p α)p ) = {Λ} ∪ {i.q | α = ( p.i α′ )p.i for some α′ , q ∈ Pos(α′ )}
Pos(( p α1 . . . αn )p ) = {Λ} ∪ {i.qi | qi ∈ Pos(αi ), 1 ≤ i ≤ n}

( n ≥ 2)

Lemma 4 Let t be a parse tree in PT(r, w) for a regular expression r and a string w. Then,
Pos(t) = Pos(Φ( p, t)) for any p.
P ROOF

By a straightforward structual induction on r.



Lemma 5 For any regular expression r and position p, Φ( p, ) is an injection from
PT(r, w) to correctlly nested parenthesis expressions.
P ROOF Suppose Φ( p, s) = Φ( p, t). L EMMA 4 implies Pos(s) = Pos(t). It follows
by L EMMA 2 that s = t. 
9

<!-- page 12 -->

For a parse configuration ⟨u, t, v⟩ where t ∈ PT(r ), we define Φ(⟨u, t, v⟩) =
u Φ(Λ, t) v. We denote the set {Φ( p, t) | t ∈ CPT(r, w)} as CPTΦ (r, p, w). Likewise,
PCΦ (r, w) stands for the set {Φ(c) | c ∈ PC(r, w)}. We also write
CPTΦ (r, p) =

∪

CPTΦ (r, p, w)

PCΦ (r ) =

∪

PCΦ (r, w)

w∈Σ∗

w∈Σ∗

From the definitions of Φ and CPT(r, w), we have the following:
Proposition 2
CPTΦ (1, p) = {( p )p }
CPTΦ ( a, p) = {( p a)p }
CPTΦ (r1 ·r2 , p) = {( p α1 α2 )p | αi ∈ CPTΦ (ri , p.i ), for i = 1, 2 }
CPTΦ (r1 +r2 , p) = {( p α1 )p | α1 ∈ CPTΦ (r1 , p.1)} ∪ {( p α2 )p | α2 ∈ CPTΦ (r2 , p.2)}
CPTΦ (r ∗, p) =

∪

{( p α1 . . . αn )p | α1 , . . . , αn ∈ CPTΦ (r, p.1),
n ≥0
and ∥ a1 ∥ > 0, . . . , ∥ an ∥ > 0 if n ≥ 2 }

Unlike the Dyck language, any expression in PCΦ (r ) has a bounded nesting
depth; it does not exceed the height of (the AST of) r indeed. This means that
PCΦ (r ) is a regular language, thereby, recognizable (or equivalently, enumerable)
by an automaton. In the next section, we will construct such automata from a
regular expression r, where any “path” corresponds to a prefix of a parenthesis
expression in PCΦ (r ). We write the prefix closure of PCΦ (r ) as PCΦ (r ).
Let α be an arbitrary parenthesis expression that is not necessarily in a range
of the function Φ. We denote the number of letters in α as ∥ α ∥. It is sometimes
convenient to “packetize” parenthesis expressions, considering the occurrences of
letters as separators. For any parenthesis expression α, which is always written
in the form β 0 a1 β 1 a2 . . . β n−1 an β n where a1 . . . an are letters and β 0 . . . β n possibly
empty sequences of parentheses, we call β i (0 ≤ i ≤ n) the i-th frame of α. As a
consequence, a correctly nexted parenthesis expression consists of only one frame
if it has no letter in it. Let α0 , . . . , αn and β 0 , . . . , β n be the sequences of frames of α
and β respectively. If αk and β k make the first distinction (that is, k is the greatest
index such that αi = β i for any 0 ≤ i < k), then the index k is called the fork of α
and β.
For each parse tree t in PT(r ), consider a parenthesis expression Φ′ ( p, t) defined

10

<!-- page 13 -->

as follows:
Φ′ ( p, 1) = ( p )p
Φ′ ( p, a) = ( p a )p
′

where a ∈ Σ

′

Φ ( p, t1 ·t2 ) = ( p Φ ( p.1, t1 ) Φ′ ( p.2, t2 ) )p
Φ′ ( p, L(t)) = ( p Φ′ ( p.1, t) )p
Φ′ ( p, R(t)) = ( p Φ′ ( p.2, t) )p
Φ′ ( p, I(t1 , . . . , tn )) = ( p str(t1 ) . . . str(tn−1 )Φ( p.1, tn ) )p
where str(t) is the string derived from t. For a parse configuration ⟨u, t, v⟩ where
t ∈ PT(r ), we define Φ′ (⟨u, t, v⟩) = u Φ′ (Λ, t) v.
Lemma 6 Let r be a regular expression and c ∈ PC(r ). c captures an interval (i, j) at a
position p if and only if ( p (resp. )q ) occurs in the i-th (resp. j-th) frame of Φ′ (c).

3.2 Position NFAs with Augmented Transitions
A position automaton with augmented transitions (PAT, in short) is a 6-tuple

⟨Σ, Q, T, ∆, q∧ , q$ ⟩
where Σ is a finite set of letters, Q a finite set of states, T a finite set of tags, ∆ a
subset of Q × Σ × Q × T ∗ , q∧ an initial state in Q and q$ a final state in Q. We call
an element of ∆ a transition.
For a PAT M = ⟨Σ, Q, T, ∆, q∧ , q$ ⟩ and an input string w, a configuration is a
triple ⟨u, p, α⟩ where u is a suffix of w, p a state in Q, and α a sequence of tags. For
a transition δ = ⟨ p1 , a, p2 , τ ⟩ in ∆, we write ⟨u1 , p1 , α1 ⟩ ⊢δ ⟨u2 , p2 , α2 ⟩ if u1 = au2
and α2 = α1 τa. The initial configuration is ⟨w, q∧ , ε⟩ (where ε denotes the empty
sequence), while a finial configuration is of the form ⟨ε, q$ , α⟩ for some α. The
subscript of ⊢ is sometimes omitted.
Given an input string w such that w = uv for some strings u, v, we say a PAT M
reaches to a state q in n-steps, yielding a sequence α (for a prefix u of w) if there exists
a sequence of length (i.e., the number of ⊢) n such that ⟨uv, q∧ , ε⟩ ⊢ · · · ⊢ ⟨v, q, α⟩
where uv = w. Moreover, we say a PAT M accepts the string u if v = ε and q = q$ .
Let r be a regular expression, and p a position. The PAT M(r, p) for r with
respect to p is recursively constructed according to the structure of r as follows:
1. M(1, p) is ⟨Σ, {q∧ , q$ }, {( p , )p }, {⟨q∧ , a, q$ , ( p )p ⟩ | a ∈ Σ}, q∧ , q$ ⟩.
2. For a ∈ Σ, M( a, p) is ⟨Σ, {q∧ , p, q$ }, {( p , )p }, ∆, q∧ , q$ ⟩ where ∆ consists of
the transitions ⟨q∧ , a, p, ( p ⟩ and ⟨ p, b, q$ , )p ⟩ for any b ∈ Σ.

11

<!-- page 14 -->

%&'!!!

!!"

+&',

%&'!

%&'.

%&'!1
q

%&'"

)&'!!
)&'!"

!"!

Λ

%&'/

#$

%&'0
&'(
)&'*

'!2)34&3!&3!!&3!!!
'"2)34&3!&3!"&3!"!

'.2)5!!"&5!!&5!&3!&3!"&3!"!
'/2)5!"!&3!"!

'(2)34&54
'*2)34&3!&3!"&5!"&5!&54

'02)5!"!&5!"&5!&3!&3!"&3!"!
'!125!"!&5!"&5!&3!&3!!&3!!!

',2)5!!!&3!!"
'-2)5!!"&5!!&5!&3!&3!!&3!!!

'!!25!!"&5!!&5!&54
'!"25!"!&5!"&5!&54

Figure 1: The position automata with augmented transitions generated from a
regular expression (a·b+a*)*
3. If M(ri , p.i ) is ⟨Σ, Qi , Ti , ∆i , q∧ , q$ ⟩ for i = 1, 2, then M(r1 +r2 , p) is

⟨Σ, Q1 ∪ Q2 , T1 ∪ T2 ∪ {( p , )p }, [∆1 ∪ ∆2 ] p , q∧ , q$ ⟩
4. If M(ri , p.i ) is ⟨Σ, Qi , Ti , ∆i , q∧ , q$ ⟩ for i = 1, 2, then M(r1 ·r2 , p) is

⟨Σ, Q1 ∪ Q2 , T1 ∪ T2 ∪ {( p , )p }, [∆1 · ∆2 ] p , q∧ , q$ ⟩.
5. If M(r, p.1) is ⟨Σ, Q, T, ∆, q∧ , q$ ⟩, then M (r ∗, p) is ⟨Σ, Q, T ∪ {( p , )p }, [∆∗] p , q∧ , q$ ⟩.
Finally, for M(r, Λ) = ⟨Σ, Q, T, ∆, q∧ , q$ ⟩, we define the PAT M (r ) for a regular expression r as ⟨Σ, Q, T, ∆ ∪ ∆0 , q∧ , q$ ⟩ where ∆0 consists of the transitions
⟨q∧ , a, q∧ , ε⟩ for any a ∈ Σ and ⟨q$ , a, q$ , ε⟩ for any a ∈ Σ.
In the above construction, [∆] p , ∆1 · ∆2 and ∆∗ are respectively given as follows:

[∆] p = {⟨q1 , a, q2 , τ ⟩ ∈ ∆ | q1 , q∧ , q2 , q$ } ∪ {⟨q∧ , a, q$ , ( p τ)p ⟩ | ⟨q∧ , a, q$ , τ ⟩ ∈ ∆}
∪ {⟨q∧ , a, q, ( p τ ⟩ | ⟨q∧ , a, q, τ ⟩ ∈ ∆, q , q$ }
∪ {⟨q, a, q$ , τ)p ⟩ | ⟨q, a, q$ , τ ⟩ ∈ ∆, q , q∧ }
12

<!-- page 15 -->

∆1 · ∆2 = {⟨q1 , a, q2 , τ ⟩ ∈ ∆1 | q2 , q$ } ∪ {⟨q1 , a, q2 , τ ⟩ ∈ ∆2 | q1 , q∧ }
∪ {⟨q1 , a, q2 , τ1 τ2 ⟩ | ⟨q1 , , q$ , τ1 ⟩ ∈ ∆1 , ⟨q∧ , a, q2 , τ2 ⟩ ∈ ∆2 }
∆∗ = ∆ ∪ {⟨q∧ , a, q$ , ε⟩ | a ∈ Σ}
∪ {⟨q1 , a, q2 , τ1 τ2 ⟩ | ⟨q1 , , q$ , τ1 ⟩ ∈ ∆, ⟨q∧ , a, q2 , τ2 ⟩ ∈ ∆, q1 , q∧ , q2 , q$ }
Note that the set T of tags in M (r ) is given by the parentheses, each of which
has a position as its index, so that M(r ) yields parenthesis expressions.
Fig. 1 shows the PAT M((a · b + a∗)∗) obtained by the above construction,
where the symbol  stands for arbitrary letters in Σ; that is, a transition with 
actually represents several transitions obtained by replacing  with a letter in Σ.
Notice that the PATs constructed above requires a look-ahead symbol. We assume that Σ includes an extra symbol $ for indicating the “end of string,” and that
any string given to a PAT M (r ) has a trailing $.
We denote the set of parenthesis expressions that a PAT M yields by accepting
an input string w$ as PE( M, w). To be precise, we ignore the trailing $ in resulting expressions; i.e., PE( M, w) = {α | M accepts w$, yielding α$ }. Note that
PE( M (r ), w) is obtained by appending an arbtrary string, as a prefix or a suffix;
i.e., we have
PE( M(r ), w) = {uαv | α ∈ PE( M(r, Λ), w′ ), u, w′ , v ∈ Σ∗ such that uw′ v = w}.
We also write
PE( M (r, p)) =

∪

PE( M(r, p), w)

w∈Σ∗

PE( M(r )) =

∪

PE( M(r ), w)

w∈Σ∗

It is not difficult to see, from the construction of PAT’s, that the following equations are satisfied:
Lemma 7
PE( M (1, p)) = {( p )p }
PE( M ( a, p)) = {( p a)p }
PE( M (r1 ·r2 , p)) = {( p α1 α2 )p | αi ∈ PE( M(ri , p.i )) for i = 1, 2}
PE( M (r1 +r2 , p)) = {( p α1 )p | α1 ∈ PE( M (r1 , p.1))} ∪ {( p α2 )p | α2 ∈ PE( M(r2 , p.2))}
PE( M (r ∗, p)) =

∪

n ≥0

{( p α1 . . . αn )p | α1 , . . . , αn ∈ PE( M(r, p.1)),
and ∥ a1 ∥ > 0, . . . , ∥ an ∥ > 0 if n ≥ 2 }

Lemma 8 CPTΦ (r, p) = PE( M(r, p)) for any regular expression r and position p.
P ROOF The proof is based on the structural induction on the given regular expression r with P ROPOSITION 2 and L EMMA 7. 
The following theorem states that a PAT is capable of exactly enumerating any,
and only canonical parse configurations:
13

<!-- page 16 -->

Theorem 2 PCΦ (r ) = PE( M (r )) for any regular expression r.
P ROOF

From L EMMA 8, the following equation can be derived:
PCΦ (r ) = {uαv | α ∈ CPTΦ (r ), u, v ∈ Σ∗ }
= {uαv | α ∈ PE( M(r, Λ)), u, v ∈ Σ∗ }
= PE( M(r ), w).

Corollary 2 PCΦ (r, w) = PE( M(r ), w) for any regular expression r and string w.

14

<!-- page 17 -->

4 Developing a Matching Algorithm
4.1 Basic Idea
As mentioned before, our matching algorithm basically emulates the subset construction on the fly. The only but crucial difference is that we need to choose, in
each step, the most prior one when paths converge on the same state.
The structure of a parse tree is represented in a parenthesis expression as nesting structure, so that the norm of a subtree corresponds with the distance, the
number of frames, from a opening parenthesis to the corresponding closing parenthesis. The problem here is how to reduce the efforts of comparing those distances
as well as possible.
For this, we focus on the first step at which two paths are branching. Our
key observation is that the opening parentheses that occur after the branching
can be entirely ignored. We have alreadly seen in a previous chapter, at least one
of the first disagreement pair of parentheses is always an opening parenthesis,
and this open parenthsis corresponds with a subtree which trivially has a larger
norm since its counterpart has the least norm −1; Since open parentheses occur
in order of precedence, we can safely ignore the open parentheses that occur after
this branching. This observation considerably reduces the number of parentheses
to be considered. Moreover, since the opening parentheses in consideration occur
exactly at the same position in the common prefix of those paths, we can compare
the norms by just looking at the position of closing parentheses.
Further reduction is possible. Some open parentheses in the common prefix
have already been closed before the branching, and hence can be ignored, too.
The open parentheses to be compared are easily identified by using stack. Consider imaginary stacks, one for each path. Along each path, opening parentheses
are pushed on the stack in order of occurrence, and are eventually removed when
the corresponding closing parentheses are encountered. Then, the contents of
stacks at the moment of branching indicates the open parentheses to be compared.
To distinguish this content, we prepare, for each stack, another stack pointer that
we call bottom pointer, which initially designates the top of the content (equivalently, the bottom of the forthcoming parentheses) and decreases when the corresponding closing parentheses are encountered. Comparing the bottom pointers at
each step allows us to know exactly when the corresponding closing parentheses
appear.
Actually, the particular content of each stack does not matter since we already
know that parentheses are correctly nested; what’s really important is the minimum record of each bottom pointer that have ever achieved within the steps from
the beginning to the current step. Moreover, we no longer need stack (top) pointers since they are indicated by the index of a parenthesis.
The above consideration allows us to develop a rather simple way of compar15

<!-- page 18 -->

ing the priority of paths only based on operations of the bottom pointers, without
concerning each of particular parentheses at runtime. We fomarlize this idea in
Section 4.2 below.

4.2 Formalization
First, we define the height of an opening parenthesis ( p as | p| + 1 while the height
of a closing parenthesis )p as p. Intuitively, the height of a parenthesis is the
value of the (imaginary) stack pointer immediately after processing the parenthesis, which we discussed above.
For any sequences α and β of parentheses, α ⊓ β denotes the longest common
prefix of α and β. For any sequence α of parentheses and a prefix α′ of α, α\α′
denotes the remaining sequence obtained by removing α′ from α. In case α\(α ⊓ β)
is non-empty, we denote the first element of α\(α ⊓ β) as α/β. In other words, α/β
is the first (that is, the leftmost) symbol in α which does not appear in β. Obviously,
at least one of α/β or β/α exists if α , β.
Let α and β be parenthesis expressions whose frames are α0 , . . . , αn and β 0 , . . . , β n
respectively. Suppose that k is the fork (i.e., the index of the first different frames)
of α and β.
We define the trace of α with respect to β, written trβ (α), as a sequence ρ0 , . . . , ρn
of integers as follows:


(i < k)
 −1
ρi = min{lastsp(αk ⊓ β k ), minsp(αk \(αk ⊓ β k ))} (i = k)


min{ρi−1 , minsp(αi )}
(i > k)
where lastsp(γ) denotes the height of the last (that is, the rightmost) parenthesis
in γ, while minsp(γ) is the minimal height of the parentheses in γ for non-empty
γ; we define lastsp(γ) = minsp(γ) = 0 if γ is empty. Intuitively, The i-th value
ρi of trβ (α) (i > k where k is the fork) means the minimal record of the bottom
pointer for α within the steps from the fork to the i-th step. The negative number
−1 just means that the bottom pointer is not yet set. We denote the initial value
min{lastsp(α ⊓ β), minsp(α\(α ⊓ β))} as bp0β (α).
Usually, it is convenient to treat trβ (α) and trα ( β) together. We write
[ ]
[ ] [ ]
a
α
a0
... n ,
tr
=
bn
b0
β
or more concisely, tr(α; β) = ( a0 , a0 ), . . . , (bn , bn ) where a0 , . . . , an and b0 , . . . , bn
are respectively trβ (α) and trα ( β). Likewise, bp0(α; β) denotes (bp0β (α), bp0α ( β)).
Let α and β be parenthesis expressions. For tr(α; β) = (ρ0 , ρ0′ ), . . . , (ρn , ρ′n ), we
write α @ β if ρi > ρi′ for the least i such that ρ j = ρ′j for any j > i. We write α ∼ β
16

<!-- page 19 -->

if ρi = ρi′ for any 0 ≤ i ≤ n. In particular, we immediatelly obtain α ∼ β if α and β
do not have any non-empty common prefix.
Let α and β be correctlly nested parenthesis expressions, k their fork, and α′
(resp. β′ ) is the k-th frame of α (resp. β). We write α @′k β, if the following conditions, (1) and (2), hold for some position p; (1) there exists α′ /β′ = ( p ; (2) if there
exists β′ /α′ = (q for some q then p l q. We simply write α @′ β if α @′k β for some
k.
Definition 9 Let α and β be parenthesis expressions. We say α is prior to β, written
α l β, if either α @ β or else α ∼ β and α @′ β.
Note that parenthesis expressions can be compared only if they have the same
number of frames (or equivalently, the same number of letters).
In the following, we will prove that the relation l we have just defined is inndeed an ordering that is essentially the same as the precedence l on parse trees
that we have previously introduced.
Our next aim is to show the compatibility of the above definition with the basic
definition of priority in Section 2. We begin with a few auxiliary propositions, all
of which immediately follow from the above definition.
Lemma 9 The relation l on PCΦ (r, w) is anti-symmetric; i.e., s l t implies t ̸ls for any s
and t in PCΦ (r, w).
P ROOF

Obvious from D EFINITION 9.



Lemma 10 Let r be a regular expression, w a string, and s, t ∈ CPT(r, w) such that s l t.
p
p
Let q be the first distinct position of s and t. If |q| = 1, then both Φr (s) ∼ Φr (t) and
p
p
Φr (s) @′ Φr (t) holds for any p.
P ROOF Since |q| = 1, q is an integer, say, i. We have two cases:
[1] In case r = r1 +r2 , s and t are respectively of the forms L(s1 ) and R(t1 ) for
p
p.1
p
p.2
some s1 and t1 . Then, we have Φr (s) = ( p Φr1 (s1 ))p and Φr (t) = ( p Φr2 (t1 ))p .
p
p
Since ∥ s1 ∥ = ∥ t1 ∥, we have Φr (s) ∼ Φr (t).
[2] In case r = r1 ∗, P ROPOSOTION 1 tells us that s and t are respectively of
the forms I(u1 , . . . , ui−1 , si ) and I(u1 , . . . , ui−1 ) where the sequence u1 , . . . , ui−1 is
p
p
possibly empty. Since ∥ si ∥ = 0, it is not difficult to see that Φr (s) ∼ Φr (t).
p
p
In both cases, Φr (s) @′ Φr (t) is obviously true. 
Lemma 11 Let r be a regular expression, w a string, and s, t ∈ CPT(r, w) such that s lq t.
p
p
If |q| = 1 and q is not the first distinct position, then Φr (s) @ Φr (t) holds for any p.

17

<!-- page 20 -->

P ROOF Since |q| = 1, q is an integer, say, i. P ROPOSITION 1 lets us know that s
and t are respectively of the forms: f (u1 , . . . , ui−1 , si , . . . , sn ) and f (u1 , . . . , ui−1 , ti , . . . , tn′ )
where f is either · or I, and the common sequence u1 , . . . , ui−1 is possibly empty.
p
p
Since ∥ si ∥ > ∥ ti ∥ but ∥ s ∥ = ∥ t ∥, it is not difficult to see that Φr (s) @ Φr (t).

Lemma 12 Let r be a regular expression, w a string, and s, t ∈ CPT(r, w) such that
p.i
p.i
Φr (s|i ) l Φr (t|i ) for some integer position i. If s|k = t|k for any integer k < i such
p
p
that k ∈ Pos(s) ∪ Pos(t), then Φr (s) l Φr (t).
P ROOF

p

p

Let α = Φr (s) and β = Φr (t). They are given by
α = ( p γ1 . . . γi−1 αi . . . αn )p ,
β = ( p γ1 . . . γi−1 β i . . . β n′ )p
p.k

p.k

p.k

where γk = Φr (s|k )(= Φr (t|k )) (k < i), αk = Φr (s|k ) (i ≤ k ≤ n), β k =
p.k
Φr (t|k ) (i ≤ k ≤ n′ ), and the common sequence γ1 . . . γi−1 may be empty. Hence,
αi @′ β i implies α @′ β. It is also not difficult to see α @ β when αi @ β i . 
The following key lemma states that the function Φ is order-preserving:
Lemma 13 Let r be a regular expression, w a string, and s, t ∈ CPT(r, w). For any p,
s l t implies Φr (s, p) l Φr (t, p).
P ROOF The proof is based on structual induction on r. Let q be the first distinct
position of s and t. We distinguish two cases according with the length of q(>
0). [1] In case |q| = 1, the result follows by L EMMA 10. [2] Otherwise, let q′
be the decision position of s and t. If |q′ | = 1, the result follows by L EMMA 11.
Otherwise, put q′ as i.q′′ for some integer i. Since s|i l t|i , We have, by induction
p.i
p.i
hypothesis, Φr| (s|i ) l Φr| (t|i ) for any p where j = fold(i ). Since P ROPOSITION 1
j

j

tells us s|k = t|k for any k < i such that k ∈ Pos(s) ∪ Pos(t), the result follows by
L EMMA 12. 
The converse is also true:

Lemma 14 Let r be a regular expression, w a string, and s, t ∈ CPT(r, w) such that
∥ s ∥ = ∥ t ∥. For any p, Φ(s, p) l Φ(t, p) implies s l t.
P ROOF Suppose Φ(s) l Φ(t) but s ̸lt. Since Φ(s) and Φ(t) are different, s and t
is not identical. Hence, by T HEOREM 1, we have t l s, which implies Φ(t) l Φ(s).

18

<!-- page 21 -->

However, this contradicts the fact that l is asymmetric on the set of parenthesis
expressions (L EMMA ??).
We conclude that the stepwise comparison is compatible with the basic definition of priority:
Lemma 15 Let r be a regular expression, and w a string. For any u, v ∈ PC(r, w) we
have u l v if and only if Φ(u) l Φ(v).
P ROOF (⇒) Consider u = ⟨u1 , s, u2 ⟩ and v = ⟨v1 , t, v2 ⟩ in PC(r, w) such that u l v.
We distinguish two cases.
[1] Suppose |u1 | < |v1 |. We have Φ(u) = u1 Φ(s)u2 and Φ(v) = v1 Φ(s)v2 .
(⇐)
The order we have just defined above is essentially the same as the priority
order on parse configurations:
Theorem 3 Let r be a regular expression, and w a string. ⟨PCΦ (r, w), l⟩ is an order set
isomorphic to ⟨PC(r, w), l⟩.
P ROOF Strict totality has been shown in P ROPOSOTION ??. Transitivity of ⟨PCΦ (r, w), l⟩
follows from L EMMA 15. Irreflexivity is an consequence of transitivity and asymmetry (L EMMA ??).
Next, we show that Φ gives a bijection from PC(r, w) to PCΦ (r, w). This means
that PC(r, w) and PCΦ (r, w) are isomorphic since they are finite total order sets.
Consider s, t ∈ CPT(r, w) such that s , t. By T HEOREM ??, we have either s l t or
t l s. Without loss of generarity, we assume s l t, which implies Φ(s) l Φ(t) by
L EMMA 15. Finally, we have Φ(s) , Φ(t) by irreflexivity.

19

<!-- page 22 -->

Algorithm 1 Match a string w against a regular expression r
1: K : = {q∧ }
2: Let P [q∧ ] point an empty list
3: for all n : = 0, . . . , |w| do
4:
Read the next letter a in w$ (left to right)
5:
proceed one step(n, effective transitions( a))
6:
if K contains the final state q$ then
7:
From K drop q∧ and all q such that L[q] > L[q$ ]
8:
break if K contains only q$
9:
end if
10: end for
11: if K contains the final state q$ then
12:
Report the captured positions via report( P[q$ ])
13: else
14:
Report FAILURE
15: end if

5 Matching Algorithm and Its Correctness
Algorithm 2 effective transitions( a)
1: trans : = ∅
2: for all state q ∈ Q a such that tag( p, q ) , ⊥ for some p ∈ K do
3:
K ′ : = K \{ p}
4:
for all state p′ ∈ K ′ such that tag( p′ , q) , ⊥ do
5:
ρ : =min{ B[ p][ p′ ], minsp(tag( p, q))}; ρ′ : =min{ B[ p′ ][ p], minsp(tag( p′ , q))};
6:
p : = p′ if ρ < ρ′ or ρ = ρ′ and D [ p′ ][ p] = 1
7:
end for
8:
Add ⟨ p, q, tag( p, q)⟩ to trans
9: end for
10: return trans
Based on the above discussion, we provide a regular expression matching algorithm. Algorithm 1 shows the main routine of the algorithm, which takes, apart
from a PAT M (r ) built from a regular expression r, an input string w$ then answers
whether w can match against r or not. If the matching succeeds, the algorithm tells
us the positions of substrings captured by subexpressions.
Throughout the entire algorithm, a couple of global variables are maintained:
K, sp, B, D, P and L. K stores the set of current states. Let α and β be the paths
getting to states p and q in K respectively. Then, B[ p][q] designates the stack bottom pointers for α and β. D [ p][q] remembers which path is prior to the other.
P[ p] stores a pointer to the list, each element of which is a sequence of tags in Tcap
20

<!-- page 23 -->

Algorithm 3 proceed one step(n, trans)
1: K : = { q | ⟨ , q, ⟩ ∈ trans }
2: Create copies D ′ , P′ , B′ of D, P, B respectively.
3: for all ⟨ p, q, α⟩, ⟨ p′ , q′ , α′ ⟩ ∈ trans such that q l q′ do
4:
if p = p′ then
5:
D [q][q′ ] : = 1 if α @′ α′ ; D [q][q′ ] : = −1 if α′ @′ α
6:
ρ : = bp0α′ (α); ρ′ : = bp0α (α′ )
7:
else
8:
D [q][q′ ] : = D ′ [ p][ p′ ]
9:
ρ : = min{ B′ [ p][ p′ ], minsp(α)}; ρ′ : = min{ B′ [ p′ ][ p], minsp(α′ )};
10:
end if
11:
D [q][q′ ] : = 1 if ρ > ρ′ ; D [q][q′ ] : = −1 if ρ < ρ′ ; D [q′ ][q] : = − D [q][q′ ]
12:
B[q][q′ ] : = ρ; B[q′ ][q] : = ρ′ ;
13: end for
14: for all ⟨ p, q, α⟩ ∈ trans do
15:
P[q] : = P′ [ p]
16:
Add α′ to the end of P[q] where α′ contains tags t in α such that t ∈ Tcap ∪ T∗
17:
L[q] : = n if α contains (Λ
18: end for

Algorithm 4 report(ℓ)
1: for each tag t in the m-th element of ℓ (in reverse order) do
2:
if t is a closed parenthesis ever seen before, say )p then
3:
Skip until the corresponding tag ( p occurrs, and continue to the next tag
4:
else
5:
Report m as the captured position for t if t ∈ Tcap .
6:
Remember t if it is a closed tag
7:
end if
8: end for

21

<!-- page 24 -->

or T∗ appearing in each frame in α. Finally, L[q] stores the position at which the
parenthesis (Λ appears in α.
The main routine calls two other subroutines; effective transitions( a) (Alg. 2)
takes a letter a of the input string then returns a set of transitions actually used for
that step, pruning less prior, thus ineffective, transitions, while proceed one step(n, trans)
(Alg. 3) updates the values of the global variables for the next step of computation.
In case Algorithm 1 succeeds, we call { P[q$ ].bu f [0], . . . , P[q$ ].bu f [ N − 1]} a
match where N be the latest value of P[q$ ].idx.
Theorem 4 Let r be a regular expression,w a string and C be a set of capturing positions
in r. If PC(r, w) is non-empty, the algorithm 1 succeeds. Otherwise, the algorithm 1
fails. In the former case, let c be the least parse configuration in PC(r, w), and M the
match. Then, M include (( p , i ) and ()p , j) if and only if c captures an interval (i, j) at the
position p.
Since bp0(), minsp(), and the ordering @ can be computed, in advance, at compile time, namely, before a particular input string is given, the time complexity
of effective transitions() is O(n2 ), where n is the number of occurrences of
the most frequently used letter in a given regular expression and O(n(n + c)) for
proceed one step() where c is the number of capturing positions plus the number of ∗-nodes. Therefore, for the length m of a input string, the time complexity
of the whole algorithm is given as follows:
Theorem 5 The time complexity of the algorithm is O(mn(n + c)).

22

<!-- page 25 -->

6 Concluding Remarks
We have presented a regular expression matching algorithm that follows the leftmostlongest rule. We also have been pursuing the correctness of the algorithm by offering a formalization of the rule based on canonical parse trees. The worst case
computational cost, O(mn(n + c)), is acceptable in most practical applications because n and c remains rather small, while the length m of an input string could
increase substantially in most cases. One controversial aspect of our formal interpretation of the leftmost-longest rule is whether it is acceptable for a common
understanding of the community. For this, we state that it is consistent with an
extensively accepted interpretation [5] which investigates the POSIX specification
very closely.
The idea of realizing regular expression matching by emulating subset construction at runtime goes back to early days; see [2] for the history. The idea of
using tags for representing the syntactical structure of regular expression is indebted to the study [10], in which Laurikari have introduced tagged automata (the
basis of TRE library) in order to formulate the priority of paths. Unfortunately, it
is somewhat incompatible with today’s interpretation [5] as for the treatment of
repetition.
Dubé and Feeley have given a way of generating the entire set of parse trees
from regular expressions [4] by using a grammatical representation. Frisch and
Cardelli have also considered an ordering on parse trees [6], which is completely
different from ours since they focus on the greedy, or first match, semantics. They
also focus on a problem of ε-transition loop, which does not occur in our case since
we are based on position automata. Vansummeren [13] have also given a stable
theoretical framework for the leftmost-longest matching, although capturing inside repetitions is not considered.
An implementation taking a similar approach to ours is Kuklewicz’s Haskell
TDFA library [9]. Although it is also based on position automata, the idea of
using orbit tags for the comparison of paths is completely different from our approach. Another similar one is Google’s new regular expression library called
RE2 [3] which has come out just before we finish the preparation of this paper.
TRE, TDFA, RE2 and our algorithm are all based on automata, so that, while their
scope is limited to regular expressions without back references, they all enable of
avoiding the computational explosion.
Acknowledgements: This work is supported by Japan Society for Promotion
of Science, Basic Research (C) No.22500019.

23

<!-- page 26 -->

References
[1] F. Baader and T.Nipkow. Term Rewriting and All That. Cambridge University
Press, 1998.
[2] R. Cox. Regular expression matching can be simple and fast. Available online:
http://swtch.com/∼rsc/regexp/regexp1.html, 2007.
[3] R. Cox. Regular expression matching in the wild. Available online: http:
//swtch.com/∼rsc/regexp/regexp3.html, 2010.
[4] D. Dubé and M. Feeley. Efficiently building a parse tree from a regular expression. Acta Infomatica, 37(2):121–144, 2000.
[5] G. Fowler.
An interpretation of the POSIX regex standard.
Available online: http://www2.research.att.com/∼gsf/testregex/
re-interpretation.html, 2003.
[6] A. Frisch and L. Cardelli. Greedy regular expression matching. In ICALP04
(LNCS 3142), pages 618–629, 2004.
[7] V. M. Glushkov. The abstract theory of automata. Russian Mathematical Surveys, 16(5):1–53, 1961.
[8] The IEEE and The Open Group. The open group base specification Issue 6
IEEE Std 1003.1 2004 Edition. Avaiable online:http://www.opengroup.
org/onlinepubs/009695399/basedefs/xbd_chap09.html, 2004.
[9] C. Kuklewicz. Regular expressions / Bounded space proposal. Available
online:http://www.haskell.org/haskellwiki/Regular_
expressions/Bounded_space_proposal, 2007.
[10] V. Laurikari. Efficient submatch addressing for regular expressions. Master’s
thesis, Helsinki University of Technology, 2000.
[11] R. McNaughton and H Yamada. Regular expressions and state graphs for
automata. IEEE Transactions on Electronic Computers, 9:39–47, 1960.
[12] H. Spencer. A regular-expression matcher. In Software Solutions in C. Academic Press, 1994.
[13] S. Vansummeren. Type inference for unique pattern matching. ACM Transactions on Programming Languages and Systems, 28(3):389–428, 2006.

24

<!-- page 27 -->

A Proof of Theorem 4
Lemma 16 Let ∆ be a set of transitions in M (r, p) for some regular expression r and
position p. Then ∆ satisfies the property that if distinct transitions ⟨s, a, q1 , τ1 ⟩ and
⟨s, a, q2 , τ2 ⟩ with q1 , q2 belong to ∆, then both τ1 /τ2 and τ2 /τ1 exist and either τ1 @′ τ2
or τ2 @′ τ1 holds.
P ROOF Structural induction on r. The base case, where r = 1 or r ∈ Σ, is obvious.
In the induction step, we denote the set of transitions of M (ri , p.i ) by ∆i (i = 1, 2).
Note that if ∆ satisfies the property then so does [∆] p for any p.
[1] Suppose r = r1 ·r2 . It suffices to show that ∆1 · ∆2 satisfies the property. We
only consider the following two cases because the others immediately follow
from the induction hypothesis: (1) Consider ⟨s, a, q1 , ττ1 ⟩ and ⟨s, a, q2 , ττ2 ⟩
obtained from ⟨s, , q$ , τ ⟩ ∈ ∆1 and ⟨q∧ , a, qi , τi ⟩ ∈ ∆2 (i = 1, 2). Then ττ1 ⊓
ττ2 = τ (τ1 ⊓ τ2 ) holds. Since induction hypothesis gurantees that both τ1 /τ2
and τ2 /τ1 exist, so do ττ1 /ττ2 and ττ2 /ττ1 and they are equal to τ1 /τ2 and
τ2 /τ1 respectively. Thus, from the induction hypothesis, either ττ1 @′ ττ2 or
ττ2 @′ ττ1 holds; (2) Consider ⟨s, a, q1 , τ1 ⟩ ∈ ∆1 and ⟨s, a, q2 , τ2 τ ⟩ obtained
from ⟨s, a, q$ , τ2 ⟩ ∈ ∆1 and ⟨q∧ , a, q2 , τ ⟩ ∈ ∆2 . The induction hypothesis
assures both τ1 /τ2 and τ2 /τ1 exist. Thus τ1 /τ2 τ and τ2 τ/τ1 are equal to
τ1 /τ2 and τ2 /τ1 respectively. Therefore, from induction hypothesis, either
ττ1 @′ ττ2 or ττ2 @′ ττ1 holds.
[2] Suppose r = r1 +r2 . We only show ∆1 ∪ ∆2 satisfies the property. We only
consider the case where ⟨q∧ , a, qi , τi ⟩ is from ∆i (i = 1, 2). An easy induction reveals that for any regular expression r and position p the transitions
leaving from q∧ in M(r, p) start with ( p . Hence τi starts with ( p.i (i = 1, 2).
It follows that the property holds for ∆1 ∪ ∆2 . The other cases immediately
follow from the induction hypothesis.
[3] Suppose r = r1 ∗. We show that [∆1 ∗] p satisfies the property. We only consider the following two cases because the others are proved similary to the
case of r = r1 ·r2 : (1) Consider ⟨q, a, q$ , τ)p ⟩ and ⟨q, a, q′ , ττ ′ ⟩ obtained from
⟨q, a, q$ , τ ⟩ and ⟨q∧ , a, q′ , τ ′ ⟩ in ∆1 . We have τ)p /ττ ′ = )p . As shown above,
τ ′ starts with ( p.1 . Thus ττ ′ /τ)p = ( p.1 . Therefore ττ ′ @′ τ)p ; (2) Consider
⟨q∧ , a, q$ , ( p )p ⟩ and ⟨q∧ , a, q, ( p τ ⟩ obtained from ⟨q∧ , a, q, τ ⟩ ∈ ∆1 , where
q , q$ . Since τ starts with ( p.1 , we have ( p )p /( p τ = )p and ( p τ/( p )p = ( p.1 .
Therefore ( p τ @′ ( p )p .

Lemma 17 Let r be a regular expression. For any distinct transitions ⟨s, a, q, τ ⟩ and
⟨s, a, q′ , τ ′ ⟩ with q , q′ in M(r ), either τ @′ τ ′ or τ ′ @′ τ holds.
P ROOF If ⟨q∧ , a, q, τ ⟩ and ⟨q∧ , a, q∧ , ε⟩ with q , q∧ , by definition we have
τ @′ ε. Then the result follows from L EMMA 16.

25

<!-- page 28 -->

We show the correctness of the algorithm described by Algorithms 1–3.
Lemma 18 Let α and β be parenthesis expressions and ⟨ρ, ρ′ ⟩ the last element of tr(α; β).
If α and β are written as α′ aγ and β′ aδ respectively with some parenthesis expressions α′
and β′ , a letter a and frames γ and δ then α l β if and only if the one of the following
property holds.
1. ρ > ρ′
2. ρ = ρ′ and α′ l β′
3. ρ = ρ′ , α′ = β′ and γ @′ δ.
P ROOF Let α′ = α0 a1 α1 . . . an αn and β′ = β 0 a1 β 1 . . . an β n for some n ≥ 0, and
tr(α′ ; β′ ) = ⟨ρ1 , ρ1′ ⟩ · · · ⟨ρn , ρ′n ⟩. By definition α l β if and only if one of the following holds.
1. ρ > ρ′
2. ρ = ρ′ and there exists j with ρ j > ρ′j and ρi = ρi′ for any j < i ≤ n
3. ρ = ρ′ , α′ = β′ and γ @′ δ
4. ρ = ρ′ and there exists a fork k ≤ n with αk @′ β k
The result follows because the disjunction of the second and the fourth conditions
is equivalent to ρ = ρ′ and α′ l β′ .

Lemma 19 Let α and β be parenthesis expressions, γ a sequence of parentheses and a ∈
Σ. If α l β then αaγ l βaγ.
P ROOF Since α l β, they have the same number of frames more than zero. Let
⟨ρ, ρ′ ⟩ and ⟨σ, σ′ ⟩ be the last elements of tr(α; β) and tr(αaγ; βaγ), respectively.
Since α l β, we have σ = min(ρ, minsp(γ)) and σ′ = min(ρ′ , minsp(γ)). From
L EMMA 18 ρ ≥ ρ′ follows. We distinguish the following three cases.
– If minsp(γ) > ρ ≥ ρ′ then σ = ρ ≥ ρ′ = σ′ .
– If ρ ≥ minsp(γ) ≥ ρ′ then σ = minsp(γ) ≥ ρ′ = σ′ .
– If ρ ≥ ρ′ ≥ minsp(γ) then σ = minsp(γ) = σ′ .
Hence σ ≥ σ′ . By L EMMA 18, α l β induces αaγ l βaγ.



Lemma 20 Let α, β and γ be sequences of parentheses. If α @′ β and β @′ γ then α @′ γ.

26

<!-- page 29 -->

P ROOF If α ⊓ β contains more parentheses than β ⊓ γ, then ( β ⊓ γ)( β/γ) is a
prefix of α ⊓ β. Hence we have α @′ γ. The same holds for the opposite case.
Suppose α ⊓ β = β ⊓ γ. Because α @′ β and β @′ γ, we have α/β = ( p1 and
β/α = β/γ = ( p2 with p1 l p2 . If γ/β = ( p3 then p2 l p3 . Thus α @′ γ follows
from p1 l p3 ; If not, α @′ γ obviously holds.

Hereafter, r and string w denote a regular expression and a string fixed arbitrary. Let M(r ) = ⟨Σ, Q, T, ∆, q∧ , q$ ⟩. For the sake of convenience, we introduce
the following: a0 stands for ε, ai denotes the i-th letter in w (1 ≤ i ≤ |w|) and a|w|+1
means $. Thus, w$ is denoted by a0 . . . a|w|+1 .
The relation ⊢n on the configurations is defined as follows: c ⊢0 c′ if c = c′ ;
c ⊢n+1 c′ if there exists a configuration c′′ such that c ⊢n c′′ and c′′ ⊢ c′ . We
denote a set of reachable states with n steps by reach(n), i.e., reach(n) = {q |
⟨w$, q∧ , ε⟩ ⊢n ⟨ , q, ⟩}. The set { p ∈ reach(n) | ∃τ.⟨ p, an , q, τ ⟩ ∈ ∆} for n ≥ 0
and q ∈ Q is denoted by reach(n)|q . The set of parenthesis expressions obtained
by the paths reaching q with n steps is presented by pe(q, n). Precisely, pe(q, n) =
{α | ∃v.⟨w$, q∧ , ε⟩ ⊢n ⟨v, q, αan ⟩}. Note that for q ∈ Q and n ≥ 0 every element of
pe(q, n), if any, is of the form a0 α0 . . . an−1 αn−1 because we let a0 = ε. Especially,
pe(q∧ , 0) = {ε} and pe(q, 0) = ∅ if q , q∧ .
Lemma 21 Let α and β be in pe(q, n) for some q ∈ reach(n) and n ≥ 0. If α , β then
either α l β or β l α holds.
P ROOF If α ̸∼ β then by definition α l β or β l α holds. Suppose α ∼ β and α , β.
Since every state in Q can reach q$ , there exists a parenthesis expression γ such
that αan γ, βan γ ∈ PE( M (r ), v) for some v ∈ Σ∗ . Therefore, L EMMA ?? yields α l β
or β l α.

Lemma 22 For any state q ∈ Q and n ≥ 0, the relation l between parenthesis expressions is a strict total order on pe(q, n).
P ROOF Irreflexivity immediately follows from the definition of l. Suppose α l β
and β l γ for α, β, γ ∈ pe(q, n). Since every state in Q can reach q$ , there exists
a parenthesis expression δ such that αan δ, βan δ, γan δ ∈ PE( M(r ), w). Repetitive
applications of L EMMA 19 results in αan δ l βan δ and βan δ l γan δ. From T HEO REMS 1, 2 and 3, ⟨PE( M (r ), w ), l⟩ is a strict total order set. It follows αan δ l γan δ.
Suppose that α l γ does not hold. It should not be α = γ; otherwise, the contradiction occurs because α l β and β l γ = α. Thus, by L EMMA 21 γ l α holds.
Applying L EMMA 19 repeatedly, we have γδ l αδ. It contradicts with αδ l γδ.
Hence α l γ. Finally, from L EMMA 21 l is strict total.

Lemma 23 Let n > 0. For α1 ∈ pe(q1 , n) and α2 ∈ pe(q2 , n) with q1 , q2 , we have
either α1 l α2 or α2 l α1 .
27

<!-- page 30 -->

P ROOF If α1 ∼ α2 does not hold then the result obviously holds. Suppose α1 ∼
α2 . Because q1 , q2 both α1 and α2 has a common prefix β yielded by a path from
q∧ to a state p, and there exist transitions ⟨ p, a, pi , τi ⟩ (i = 1, 2) where βτi is a prefix
of αi . Then, from L EMMA 17, either τ1 @′ τ2 or τ2 @′ τ1 holds. Therefore, we
immediately obtaine the desired result.

L EMMA 22 guarantees that for n ≥ 0 and q ∈ reach(n) the set pe(q, n) has the
minimal element. We denote it by pe(q, n)↓. The following property is useful.
Lemma 24 Let n ≥ 0. If q ∈ reach(n + 1) then there exists a state p ∈ reach(n) such
that pe(q, n + 1)↓= pe( p, n)↓ an tag( p, q).
P ROOF Since pe(q, n + 1) ↓∈ pe(q, n + 1), there exist p ∈ reach(n) and α ∈
pe( p, n) such that pe(q, n + 1)↓= αan tag( p, q). We show α = pe( p, n)↓. Suppose there exists β ∈ pe( p, n) such that β l α. L EMMA 19 yields βan tag( p, q) l
αan tag( p, q). It contradicts with pe(q, n + 1)↓= αan tag( p, q) because βan tag( p, q) ∈
pe(q, n + 1). Therefore α = pe( p, n)↓.

The invariant of main loop of our algorithm is stated as follows.
Definition 10 For n ≥ 0 and P ⊆ Q, Inv(n) is a conjunction of the following properties.
(a) p ∈ reach(n) for any p ∈ K.
(b) For any p < K, if p ∈ reach(n) then pe(q, n)↓ lpe( p, n)↓ and the last element of
the trace of pe( p, n)↓ wrt. pe(q, n)↓ is zero for any q ∈ K.
(c) For any p, q ∈ K with p , q, pe( p, n)↓ lpe(q, n)↓ iff D [ p][q] = 1 iff D [q][ p] =
−1.
(d) For any p, q ∈ K with p , q, B[ p][q] is equal to the last element of the trace of
pe( p, n)↓ wrt. pe(q, n)↓.
(e) For any p ∈ K, P[ p][t] gives the greatest j such that t is included in α j for any
t ∈ Tcap that occurs in pe( p, n)↓; P[ p][t] = −1 for the other t ∈ Tcap , where
pe( p, n)↓= α0 a1 α1 . . . an αn .
The key property to show the correctness of our algorithm is that the parenthesis expression reaching for a state with n + 1 steps, yielded as the result of
Algorithm 2, is the least among ones reaching the state with n + 1 steps.
Lemma 25 Suppose Inv(n) holds at line 4 of Algorithm 1. Then the following properties
hold when returned from Algorithm 2:
1. pe(q, n + 1)↓= pe( p, n)↓ an α for any ⟨ p, q, α⟩ ∈ trans,

28

<!-- page 31 -->

2. trans1 ⊆ K, where trans1 = { p | ⟨ p, q, α⟩ ∈ trans}, where trans2 = {q |
⟨ p, q, α⟩ ∈ trans}.
P ROOF Because only the first property is non-trivial, we only show that for every
iteration of the outer loop (Lines 2–10) of Algorithm 2 the values of p and q at Line
9 satisfy pe(q, n + 1)↓= pe( p, n)↓ an tag( p, q) for any n ≥ 0, which immediately
concludes that the first property holds at the exit of Algorithm 2. L EMMA 24 implies that pe(q, n + 1)↓= min {pe(s, n)↓ an tag(s, q) | s ∈ reach(n), tag(s, q) , ⊥}.
Thus, we show the right-hand side of this equation should be pe( p, n)↓an tag( p, q).
For the sake of conciseness, we often refer to pe(s, n)↓ an tag(s, q) as µs for any
s ∈ reach(n) with tag(s, q) , ⊥.
Let S = {s ∈ K | tag(s, q) , ⊥}. Note that S ⊆ reach(n) from the property
(a) of Inv(n). We first show that the value of p in Line 9 satisfies µ p l µs for any
s ∈ S. Because l on pe(q, n + 1) is a strict total order from L EMMA 22, it is enough
to see that in the line 7 the value of p is updated with that of p′ if and only if
µ p′ l µ p ; if it holds, it is easy to see that the for all loop in Lines 3–7 computes the
minimal parenthesis expression in {µs | s ∈ S}. Note that p , p′ in Lines 5. Thus
L EMMA 23 impiles pe( p, n)↓, pe( p′ , n)↓. From the property (d) of Inv(n), the pair
⟨ B[ p][ p′ ], B[ p′ ][ p]⟩ is the last element of tr(pe( p, n)↓; pe( p′ , n)↓). Hence ⟨ρ, ρ′ ⟩ obtained in the line 5 is the last element of tr(µ p ; µ p′ ). On the other hand, the property (c) of Inv(n) shows that D [ p′ ][ p] = 1 is equivalent to pe( p′ , n)↓ lpe( p, n)↓.
Suppose the value of p is replaced with that of p′ . It follows that the condition
of the if statement in Line 7 should hold. Then L EMMA 18 immediately yields
µ p′ l µ p . Suppose µ p′ l µ p . Because pe( p, n)↓, pe( p′ , n)↓, L EMMA 18 yields that
the if statement in Line 7 holds. Therefore the value of p is replaced with that of
p′ at Line 7.
We next show that the value of p in Line 9 satisfies µ p l µs for s ∈ reach(n)|q \K.
From Inv(n) (b) the last element of tr(pe(s, n)↓; pe( p, n)↓) is ⟨0, 0⟩ and pe( p, n)↓
l pe(s, n)↓, and hence the last element of tr(µs ; µ p ) is also ⟨0, 0⟩. Therefore we
have µ p l µs .

Now we turn to the analysis of the algorithm 3. We denote the value of K at
the entry point of Algorithm 3 by K ′ . Inv′ (n) is introduced to denote the similar
property to Inv(n) except that the value of K ′ , D ′ , B′ and P′ are used instead of K,
D, B and P respectively.
Lemma 26 Suppose Inv(n) holds at Line 4 of Algorithm 1. Then the properties (a) and
(b) of Inv(n + 1) hold when returned from Algorithm 3.
P ROOF By construction of trans in Algorithm 2, it is easy to see that the second
component of each element in trans belongs to reach(n + 1). Thus the property (a)
of Inv(n + 1) immediately holds from the line 1 of Algorithm 3.
Next we show that the property (b) of Inv(n + 1) holds at the exit of Algorithm 3. Let q be in reach(n + 1)\K at the exit of Algorithm 3. By L EMMA 24, there
29

<!-- page 32 -->

exists a state p such that pe(q, n + 1)↓= pe( p, n)↓an tag( p, q). By construction of K
in Algorithm 3 and trans in Algorithm 2, we have p ∈ reach(n)\K ′ . Let q′ ∈ K.
Then, by construction of K, there exists a state p′ such that ⟨ p′ , q′ , tag( p′ , q′ )⟩ ∈
trans. L EMMA 25 yields pe(q′ , n + 1)↓= pe( p′ , n)↓ an tag( p′ , q′ ). Let ⟨σ, σ′ ⟩ be
the last element of tr(pe(q, n + 1)↓; pe(q′ , n + 1)↓). By construction of trans, we
have p′ ∈ K ′ . Thus, from the property (b) of Inv′ (n), the trace of pe( p, n)↓ wrt.
pe( p′ , n)↓ ends in 0. Then, by the definition of trace we obtain σ = 0 and hence
σ ≤ σ′ . The property (b) of Inv′ (n) also yields pe( p′ , n)↓ l pe( p, n)↓. Therefore,
from L EMMA 18 pe(q, n + 1)↓ l pe(q′ , n + 1)↓ holds.

Lemma 27 Suppose Inv(n) holds at Line 4 of Algorithm 1. Then the property (d) of
Inv(n + 1) holds when returned from Algorithm 3.
P ROOF We show that for any iteration of Lines 3–13 in Algorithm 3 the pair of
B[q][q′ ] and B[q′ ][q] is the last element of tr(pe(q, n + 1)↓; pe(q′ , n + 1)↓). From
L EMMA 25 we have pe(q, n + 1)↓= pe( p, n)↓ an α and pe(q′ , n + 1)↓= pe( p′ , n)↓ an α′
for each iteration. The result is proved as in the proof of L EMMA 25 when p , p′ .
If p = p′ then pe( p, n)↓= pe( p′ , n)↓. By construction of trans, both α and α′ are
associated with disinct transitions leaving from the same state. Thus L EMMA 16
implies α , α′ . Therefore the result follows from Line 6 of Algorithm 3.

Lemma 28 Suppose Inv(n) holds at Line 4 of Algorithm 1. Then the property (c) of
Inv(n + 1) holds when returned from Algorithm 3.
P ROOF For each iteration of Lines 3–13 in Algorithm 3 we show that D [q][q′ ] =
1 iff D [q′ ][q] = −1 iff pe(q, n + 1)↓ l pe(q′ , n + 1)↓ and that D [q′ ][q] = 1 iff
D [q][q′ ] = −1 iff pe(q′ , n + 1)↓ l pe(q, n + 1)↓.
By definition β l γ and γ l β never hold at once for any parenthesis expressions
β and γ. Thus D [q][q′ ] = 1, D [q′ ][q] = −1 and pe(q, n + 1)↓ l pe(q′ , n + 1)↓ imply
that neither D [q′ ][q] = 1, D [q][q′ ] = −1 nor pe(q′ , n + 1)↓ l pe(q, n + 1)↓ hold,
and vice versa. Furthermore, from Line 11 we have D [q][q′ ] = i iff D [q′ ][q] = −i
where i = 1 or i = −1. Thus it suffices to show that either D [q][q′ ] = 1 and
pe(q, n + 1)↓ l pe(q′ , n + 1)↓, or D [q][q′ ] = −1 and pe(q′ , n + 1)↓ l pe(q, n + 1)↓.
As shown in the previous lemma, the pair of ρ and ρ′ is the last element of
tr(pe(q, n + 1)↓; pe(q′ , n + 1)↓). We distinguish the following three cases according
to the relationship between ρ and ρ′ .
[1] Suppose ρ > ρ′ . Then from Line 11 we have D [q][q′ ] = 1. L EMMA 18 yields
pe(q, n + 1)↓ l pe(q′ , n + 1)↓.
[2] If ρ < ρ′ then, as in the previous case, we have D [q][q′ ] = −1 and pe(q′ , n + 1)↓
l pe(q, n + 1)↓.
[3] Suppose ρ = ρ′ . From L EMMA 25 we have pe(q, n + 1)↓= pe( p, n)↓ an α and
pe(q′ , n + 1)↓= pe( p′ , n)↓ an α′ for p, p′ , α and α′ in the iteration. Suppose
30

<!-- page 33 -->

p = p′ . Then pe( p, n)↓= pe( p′ , n)↓ and that both α and α′ are associated
with disinct transitions leaving from the same state. Thus, from L EMMA 16,
either α @′ α′ or α′ @′ α holds. If the former holds, D [q][q′ ] = 1 and, from
Lemma 18, pe(q, n)↓ l pe(q′ , n)↓. Otherwise, D [q][q′ ] = −1 and pe(q′ , n)↓
l pe(q, n)↓. Suppose p , p′ . From L EMMA 23 one of the properties pe( p, n)↓
l pe( p′ , n)↓ or pe( p′ , n)↓ l pe( p, n)↓ holds. By assumption Inv(n) holds and
so does Inv′ (n). Hence the property (c) of Inv′ (n) yields either D [ p][ p′ ] = 1
or D [ p][ p′ ] = −1. If the former holds, D [q][q′ ] = 1 and, from Lemma 18,
pe(q, n)↓ l pe(q′ , n)↓. Otherwise, D [q][q′ ] = −1 and pe(q′ , n)↓ l pe(q, n)↓. 

31
