---
file: ../pdfs/2024-icalp-uezato-backreferences-lookaheads-nlog.pdf
citation: "Regular Expressions with Backreferences and Lookaheads Capture NLOG. Uezato. ICALP 2024"
doi: 
source: "https://drops.dagstuhl.de/storage/00lipics/lipics-vol297-icalp2024/LIPIcs.ICALP.2024.155/LIPIcs.ICALP.2024.155.pdf"
topics: [redos_backrefs_verification]
pages: 20
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Regular Expressions with Backreferences and
Lookaheads Capture NLOG
Yuya Uezato #
CyberAgent, Inc., Tokyo, Japan
National Institute of Informatics, Tokyo, Japan

Abstract
Backreferences and lookaheads are vital features to make classical regular expressions (REGEX)
practical. Although these features have been widely used, understanding of the unrestricted
combination of them has been limited. Practically, most likely, no implementation fully supports
them. Theoretically, while some studies have addressed these features separately, few have dared
to combine them. Those few studies showed that the amalgamation of these features significantly
enhances the expressiveness of REGEX. However, no acceptable expressivity bound for REWBLk –
REGEX with backreferences and lookaheads – has been established. We elucidate this by establishing
that REWBLk coincides with NLOG, the class of languages accepted by log-space nondeterministic
Turing machines (NTMs). In translating REWBLk to log-space NTMs, negative lookaheads are the
most challenging part since it essentially requires complementing log-space NTMs in nondeterministic
log-space. To address this problem, we revisit Immerman–Szelepcsényi theorem. In addition, we
employ log-space nested-oracles NTMs to naturally handle nested lookaheads of REWBLk. Utilizing
such oracle machines, we also present the new result that the membership problem of REWBLk is
PSPACE-complete.
2012 ACM Subject Classification Theory of computation → Formal languages and automata theory;
Theory of computation → Complexity classes
Keywords and phrases Regular Expression, Automata Theory, Nondeterministic Log-Space
Digital Object Identifier 10.4230/LIPIcs.ICALP.2024.155
Category Track B: Automata, Logic, Semantics, and Theory of Programming
Related Version Long Version with Appendix: https://arxiv.org/abs/2404.17492
Funding This work was supported by JST, CREST Grant Number JPMJCR21M3.
Acknowledgements I thank anonymous reviewers for their detailed comments on a previous version
of this paper. I also sincerely thank anonymous reviewers for their careful reading and invaluable
comments on the current version. All comments helped me to significantly improve the presentation
of this paper and the clarity of proofs and constructions.

1

Introduction

Backreferences and lookaheads are practical extensions for classical regular expressions
(REGEX). REGEX with backreferences – REWB – can represent the non context-free
language L = {w#w : w ∈ {a, b}∗ } with the REWB expression E = ((a + b)∗ )x # \x.
Roughly telling about this expression, we save a substring matched with (a + b)∗ into the
variable x, and later, we refer back to the matched string using \x; therefore, E represents
L. REWB is a classical calculus [2], and there are some results:
1. Schmid showed that REWB, the class of languages accepted by REWB, is contained in
NLOG, the class of languages accepted by log-space nondeterministic Turing machines
(NTMs) [23, Lemma 18].
2. Recently, Nogami and Terauchi showed that REWB is contained in the class of indexed
languages, IL (an extension of context-free languages [1]) [20].
3. The membership problem of REWB – for an input REWB expression E and an input
word w, deciding if E accepts w – is NP-complete [2].
EA
TC S

© Yuya Uezato;
licensed under Creative Commons License CC-BY 4.0
51st International Colloquium on Automata, Languages, and Programming (ICALP 2024).
Editors: Karl Bringmann, Martin Grohe, Gabriele Puppis, and Ola Svensson;
Article No. 155; pp. 155:1–155:20
Leibniz International Proceedings in Informatics
Schloss Dagstuhl – Leibniz-Zentrum für Informatik, Dagstuhl Publishing, Germany

<!-- page 2 -->

155:2

Regular Expressions with Backreferences and Lookaheads Capture NLOG

On REGEX with lookaheads, there are two kinds of lookaheads. A positive lookahead
?(E) checks if the rest of the input can be matched by E without consuming the input. A
negative lookahead !(E) checks if the rest of the input cannot be matched by E without
consuming the input. For example, the expression ?(E)F + !(E)G runs an expression F if
E goes well with the rest of the input and runs an expression G otherwise. Thus, it can
be read as if E then F else G and is helpful in writing practical applications. Although
lookaheads are useful, they do not alter the REGEX expressiveness. This fact immediately
follows from the result of alternating Turing machines that the language class of alternating
finite automata corresponds to that of usual finite automata [6].
Now, it is a natural question: How expressive are REWB with lookaheads?
The class REWB with lookaheads – REWBLk – was studied by Chida and Terauchi [7, 8].
They have shown (a) REWBLk, the class of languages accepted by REWBLk, is closed
under intersection and complement (it is immediately shown using lookaheads); (b) REWB
is a proper subclass of REWB(+) – REWB with positive lookaheads – and REWB(−)
– REWB with negative lookaheads; (c) the language emptiness problem of REWB(−) is
undecidable. They also developed a new class of automata called positive lookaheads memory
automata (PLMFA) and proved that REWB(+) equals PLMFA.
However, the following two key questions remain unresolved:
1. Which known language classes are related to REWBLk?
2. What is the computational complexity of the membership problem of REWBLk– a
problem deciding if E accepts w for an input expression E and an input string w?
We solve these questions by presenting the following tight results:
(I) REWBLk = NLOG. Besides, REWB already contains an NLOG-complete language.
(II) The membership problem of REWBLk is PSPACE-complete.
Together with existing results, our results are summarized in the following table:
REGEX
+ lookaheads
REWB
REWBLk

Language Class

Membership Problem

= Regular [6]

P-c [18]

⊆ IL [20], incomparable to CFL [4, 5],
⊆ NLOG [23], ∋ NLOG-c language (I)
= NLOG (I)

NP-c [2]
PSPACE-c (II)

where NLOG-c is short for NLOG-complete, and the same applies to the others.

1.1

Difficulty in Translating REWBLk to Log-space NTMs

To investigate a hard part of translation from REWBLk to log-space NTMs, let us consider
the following language, which is a well-known NLOG-complete language:
Lreach = {s # x1 → y1 # · · · #xn → yn # t : s, t, xi , yi ∈ V ∗ , and there is a path from s to t}
where the part #x1 → y1 # · · · #xn → yn # means the directed graph with direct edges
x1 → y1 , x2 → y2 , and so on. The following REWBLk expression Ereach recognizes Lreach :
Ereach = (V ∗ )Cur # ?(Σ∗ # \Cur → (V ∗ )Cur #)

∗

Σ∗ # \Cur

where Σ = V ∪ {#, →}. It first captures s into the variable Cur and walks on graphs while
repeatedly evaluating the part ?(· · · ). Each evaluation makes a nondeterministic one-step
move on the graph. The part Σ∗ # \Cur checks if we reach the goal t.

<!-- page 3 -->

Y. Uezato

155:3

It is not difficult to structurally translate Ereach to a log-space NTM M such that L(M ) =
L(Ereach ). Now, using M , let us translate the if-then-else expression ?(Ereach )F + !(Ereach )G
for some expressions F and G to a log-space NTM N . Let w be an input word s#edges#t.
For the part ?(Ereach )F , we run M in N ; if M accepts w, we proceed to simulate F .
However, for the other part !(Ereach )G, we encounter problems:
(A) We need to check if all possible walks of M starting from s do not reach t.
(B) Walking paths starting from s and aiming for t become infinitely long, and thus there
are infinitely many walks (branching) to be checked.
Therefore, we cannot run M directly in N to handle the negative lookahead !(Ereach ).

Our Idea: Immerman–Szelepcsényi Theorem & Log-space Nested-Oracles NTMs
To address the above problems, we leverage Immerman–Szelepcsényi theorem [16, 27]. This
theorem states that the class NLOG is closed under complement; i.e., there exists a log-space
NTM M such that L(M ) = Σ∗ \ L(M ). Therefore, in our machine N , we run M for the part
!(Ereach )G: If M eventually accepts w, then we proceed to simulate G.
On the other hand, REWBLk permits nested lookaheads, such as ?(· · ·!(· · ·?(· · · ) · · · ) · · · ).
To handle them naturally, we employ log-space NTMs with nested oracles [16, 25, 19]. These
machines can easily simulate REWBLk and are translated to log-space NTMs by Immerman–
Szelepcsényi theorem; thus, REWBLk = NLOG holds. Moreover, oracle machines are
crucial to showing that the membership problem of REWBLk is PSPACE-complete. To
this end, we also give the new result that the membership problem of such machines is in
PSPACE.

Structure of Paper
The rest of the paper is structured as follows. Section 2 discusses related work. Section 3
reviews REWBLk and demonstrates that REWB already contains an NLOG-complete
language. Section 4 illustrates the expressiveness of REWBLk: (1) NLOG ⊆ REWBLk;
(2) the membership problem of REWBLk is PSPACE-hard; (3) REWB(+) and REWB(−)
represent languages ̸∈ IL; and (4) the emptiness problems of REWB(+) and REWB(−)
are undecidable even if Σ = {a}. Section 5 reviews log-space nested-oracles NTMs and
their language class (= NLOG), and shows our new result: their membership problem is in
PSPACE. Section 6 establishes REWBLk ⊆ NLOG and that the membership problem of
REWBLk is in PSPACE. Section 7 concludes this paper by giving open problems.

2

Related Work

As discussed in the Introduction, Chida and Terauchi have formalized REWBLk and its
semantics [7, 8]. To our knowledge, their study is the first theoretical exploration into
the simultaneous treatment of backreferences and lookaheads. Surprisingly, there has been
no prior theoretical research on the topic despite their longstanding and widespread use.
They introduced PLMFA (positive lookahead MFA) by expanding MFA (memory finite
automata), which Schmid presented for studying REWB in [24]. One of their main results is
the equivalence of PLMFA to REWB(+), established through translations between PLMFA
and REWBLk. Nevertheless, they did not address: (i) a relationship between REWBLk
and existing known language classes; (ii) the complexity of the membership problem of
REWBLk. In contrast, we show that REWBLk captures NLOG and the membership
problem of REWBLk is PSPACE-complete.

ICALP 2024

<!-- page 4 -->

155:4

Regular Expressions with Backreferences and Lookaheads Capture NLOG

As highlighted in the Introduction, for REWB, Schmid showed REWB ⊆ NLOG [23],
and Nogami and Terauchi showed REWB ⊆ IL [20]. Schmid also introduced MFA and
showed that MFA corresponds to REWB [24]. About the relationship between NLOG and
IL, it is worth noting that:
2n
NLOG ̸⊆ IL. It is shown as follows. The language L22n = {a2 : n ∈ N} clearly belongs
to NLOG. However, L22n ∈
/ IL by pumping lemmas for indexed languages [14, 11].
IL ̸⊆ NLOG unless NLOG = NP. It is shown as follows. IL can represent the language
L3SAT , whose words are true 3-CNF formulas [22]. However, L3SAT ∈
/ NLOG unless
NLOG = NP.
In the present paper, we take their result further and show that REWBLk = NLOG and
that REWB already contains an NLOG-complete language.
We also refer to modern REGEX engines that (partially) support both backreferences and
lookaheads. Several programming languages (for example, Perl, Python, PHP, Ruby, and
JavaScript) and .NET framework support these features. However, their support is limited
in both syntax and semantics. First, expressions like (\x\x)x and (\x + \x)x are rejected by
most implementations because the variable x appears more than once in single captures. Next,
in most implementations, the expression F = (?((\xa)x ))∗ \x, which represents {ϵ, a, a2 , . . .},
does not match with a2 and a3 , so on. It is due to conservative loop-detecting semantics.
Such a semantics is standardized in ECMAScript [9]. This semantics works for F as follows.
First, it unfolds the Kleene-* of (?((\xa)x ))∗ as (ϵ + ?((\xa)x )(?((\xa)x ))∗ ). Next, it enters
the underline part and updates the variable x without consuming any input characters.
Then, it tries to evaluate (?((\xa)x ))∗ again at the same input position. At this point, many
REGEX engines think that we enter an infinite loop and so stop unfolding the Kleene-*.
Consequently, F only matches with ϵ (without loop unfolding) and a (with a single loop
unfolding).
We can rephrase this situation as follows: (1) the amalgamation of lookaheads with
variables induces side effects without consuming any characters; (2) however, the loopdetecting semantics overlooks such side effects and changes behaviors from the naive semantics.
On the other hand, such conservative semantics work well for REGEX, REGEX with
lookaheads, and REWB since they do not induce such side effects.
This paper presents a translation between REWBLks and log-space NTMs, enabling the
development of REGEX engines that fully support backreferences and lookaheads. Notably,
such engines run in polynomial time (for a fixed expression) since NLOG ⊆ P.

3

Preliminaries: REWBLk

We review the syntax and semantics of REWBLk [7, 8] step-by-step below.

3.1

Regular Expressions with Backreferences and Lookaheads

We first give the syntax of REWBLk over an alphabet Σ and variables V:
E

::= σ | ϵ | E + E | E E | E ∗
\v
|
?(E)
|
(E)v |
|{z}
| {z }
| {z }
capture

backreference

positive lookahead

|

!(E)
|{z}

negative lookahead

where σ ∈ Σ and v ∈ V is a variable. The first line defines classical regular expressions,
REGEX. We consider the following subclasses in this paper: REWB (REGEX with captures
and backreferences), REWB(+) (REWB with positive lookaheads ?(E)), REWB(−) (REWB
with negative lookaheads !(E)).

<!-- page 5 -->

Y. Uezato

155:5

Semantics of REGEX
We first give a semantics for REGEX. To accommodate variables and lookaheads, configurations for REGEX are 4-tuples ⟨E, w, p, Λ⟩ where
E is an expression to be executed;
w is an input string and will not change throughout computation;
p is a 0-origin position on w (0 ≤ p ≤ |w|). We write w[p] for the symbol on the position
p. It should be noted that p = |w| is allowed to represent that we consume all the input.
Λ : V → Σ∗ is an assignment from variables V to substrings of w.
We write C for the set of configurations. To denote all the results obtained by computing,
we use a semantic function J·K : C → P(N × (V → Σ∗ )) where P(X) is the power set of X.
On J⟨E, w, p, Λ⟩K = {⟨p1 , Λ1 ⟩, . . . , ⟨pn , Λn ⟩}, each pair ⟨pi , Λi ⟩ means that, after executing E
on w from p under Λ, we move to the position pi and obtain an assignment Λi . On the basis
of this idea, we define a semantics for each rule of the REGEX part:
(
{⟨p + 1, Λ⟩} if p < |w| and w[p] = σ,
J⟨σ, w, p, Λ⟩K =
J⟨ϵ, w, p, Λ⟩K = {⟨p, Λ⟩},
∅
otherwise,
J⟨E1 + E2 , w, p, Λ⟩K
J⟨E1 E2 , w, p, Λ⟩K
∗

J⟨E , w, p, Λ⟩K

= J⟨E1 , w, p, Λ⟩K ∪ J⟨E2 , w, p, Λ⟩K,
[
=
J⟨E2 , w, p′ , Λ′ ⟩K,
=

⟨p′ , Λ′ ⟩∈J⟨E1 ,w,p,Λ⟩K
∞
[
i

i=0

i
0

i

z }| {

J⟨E , w, p, Λ⟩K where E = ϵ, E = EE · · · E.

We note that our semantic function J⟨E, w, p, Λ⟩K is inductively defined on the lexicographic
ordering over the star height of E and the expression size of E. The start height and
expression size of REWBLk is defined in the usual way.
We also note that each J⟨E, w, p, Λ⟩K forms a finite set because the value of each variable
x must be a substring of w, and also p is bounded as 0 ≤ p ≤ |w|.

Semantics of REWB
A capture expression (E)x attempts to match the input string with E. If it succeeds, the
matched substring is stored in the variable x.
J⟨ (E)x , w, p, Λ ⟩K =





p′ , Λ′ x 7→ w[p..p′ )

: ⟨p′ , Λ′ ⟩ ∈ J⟨E, w, p, Λ⟩K

where w[p..q) is the string w[p] w[p + 1] · · · w[q − 1].
A backreference \x refers to the substring stored previously by evaluating some (E)x .
J⟨\x, w, p, Λ⟩K = J⟨Λ(x), w, p, Λ⟩K.

Semantics of Lookaheads
Positive lookaheads ?(E) run E from the current input without consuming any input.
Although the change in head position is undone after running E, the modification to variables
in E is not. So, we can also call it destructive lookahead.

J⟨?(E), w, p, Λ⟩K = ⟨p, Λ′ ⟩ : ⟨p′ , Λ′ ⟩ ∈ J⟨E, w, p, Λ⟩K .

ICALP 2024

<!-- page 6 -->

155:6

Regular Expressions with Backreferences and Lookaheads Capture NLOG

Negative lookaheads !(E) also run E without consuming any input. If E does not match
anything, we invoke a continuation. Compared with positive lookaheads ?(E), both the
previous head position and the previous values of variables are recovered.
(
{⟨p, Λ⟩} if J⟨E, w, p, Λ⟩K = ∅,
J⟨!(E), w, p, Λ⟩K =
∅
otherwise.
Readers may wonder why positive lookaheads modify variables while negative ones do
not. This asymmetry arises because, in negative lookaheads, all computations uniformly
fail, leaving no suitable configuration for altering variables. On the other hand, using the
non-destructive property of the negative lookaheads, we can also define non-destructive
positive lookaheads by !(!(E)). This expression executes E from the current position without
any variable modifications.
Remark: Special character $. Using a negative lookahead, we define $ =!(Σ) to check the
end of the input. $ is one of the most important applications of negative lookaheads.
▶ Definition 1. The language of a REWBLk E, L(E), is defined as follows:

L(E) = w ∈ Σ∗ : ⟨p, Λ⟩ ∈ J⟨E, w, 0, ι⟩K, p = |w|
where ∀x ∈ V. ι(x) = ϵ.
We can also consider another definition using γ(x) = ⊥ instead of ι, where γ indicates that all
variables are initially undefined. Although some real-world regular expression engines adopt
that definition, we adopt the above ι-definition since it is tedious to initialize all variables
x using (ϵ)x . The results discussed in this paper will not change regardless of which one is
used. There is another formalization that excludes labels that appear multiple times in a
single group, for instance (\x\x)x . We discuss such a restriction, which we call reference
restriction, in the immediately following section.

3.2

Reference Restriction and Normalization

Our formalization of REWBLk allows that variable references appear inside their definitions;
for example, we allow expressions like (\x\x)x . On the other hand, many studies on REWB
do not allow them; i.e., expressions like (\x\x)x are prohibited [5, 23, 10, 20].
To the best of the author’s knowledge, it remains unclear whether the restriction alters
the expressive power of REWB. However, on REWBLk, the restriction does not change
the language class of REWBLk. Here, we formalize the restriction and then present our
normalization, which converts REWBLk expressions to language-equivalent restricted forms.
We use the function Var that receives an expressions E and returns the set of all the
backreference variables inside E. For example,
Var( \x\x ) = {x}, Var( (\x\y)z ) = {x, y}, Var( a b c ?( (\w(\x)z )y ) ) = {w, x}.
It can be easily defined inductively on the expression size of E.
We also define a restriction, reference restriction. An REWBLk expression E satisfies the
reference restriction condition if, for all the capture subexpressions (F )x of E, x ∈
/ Var(F )
holds. For example, the expression a b c ?((\w(\x)z )y ) satisfies the condition. On the other
hand, the expressions (\x\x)x and a b c ?((\w(\y)z )y ) do not.
▶ Theorem 2. For any REWBLk expression E, there is an expression E ′ that satisfies the
reference restriction condition and L(E) = L(E ′ ).

<!-- page 7 -->

Y. Uezato

155:7

Proof. It suffices to perform variable renaming like alpha-conversion in lambda calculus to
remove patterns (· · · x · · · )x . Formally, we apply the following renaming function R from the
innermost of E to the outermost:
(
?((F ′ )y )(\y)x where y is a fresh variable if F = (F ′ )x ,
R(F ) =
F
otherwise.
The expression ?((F ′ )y )(\y)x is equivalent to (F ′ )x because it first stores the result of
running F ′ to a fresh variable y and then puts it to the original variable x. Thus, the
innermost-to-outermost applying R changes expressions to ones, which satisfy the restriction
condition, without change their languages.
For example, an expression (aa)x (\x\x)x is translated to (aa)x ?((\x\x)y )(\y)x by introducing a variable y. Another expression (a)x (b)y (?[(\x\x)x ]aa\y)x is first translated to
(a)x (b)y (?[?((\x\x)α )(\α)x ]aa\y)x by introducing α. Then, introducing β, it is translated to
(a)x (b)y ?((?[?((\x\x)α )(\α)x ]aa\y)β )(\β)x .
◀
We will use this theorem in Section 6 to translate REWBLk to nested-oracles machines.

3.3

NLOG-Complete Language Accepted by REWB

Hartmanis and Mahaney proposed a decision problem called TAGAP, which is the topological
sorted version of the reachability problem of directed acyclic graphs (DAG) [13]. Let us
consider a word w = ⌜x1 → y1 ⌝⌜x2 → y2 ⌝ · · · ⌜xn → yn ⌝, which represents a DAG. We call w
topologically sorted if: for all pairs of a → b and b → c of w, a → b appears before b → c in w.
The following example represents a DAG and one of its topologically sorted representation:
s

a

b

c

d

e

t

,

⌜s → a⌝ ⌜a → d⌝ ⌜a → e⌝ ⌜d → t⌝ ⌜a → b⌝ ⌜d → e⌝ ⌜b → c⌝.

We define the language for TAGAP as follows:
LTAGAP = {s # R # t : R is a topologically sorted repr. of G, t is reachable from s in G}.
Hartmanis and Mahaney showed that this language is NLOG-complete [13, Theorem 3].
Since we only consider the topologically sorted representation, there is no longer a need
to explore the entire input many times. In the above example, we can reach t from s by
nondeterministically finding edges s → a, a → d, and d → t in this order by one-way scanning.
Indeed, we can show the following theorem.
▶ Theorem 3. LTAGAP ∈ REWB.
Proof. The following expression ETAGAP clearly recognizes LTAGAP :
ETAGAP = (V ∗ )Cur # Σ∗ ⌜ \Cur → (V ∗ )Cur ⌝

∗

Σ∗ # \Cur,

where V is an alphabet for vertices.

4

◀

Expressiveness of REWBLk

In this section, we present some theorems about the expressiveness of REWBLk.

ICALP 2024

<!-- page 8 -->

155:8

Regular Expressions with Backreferences and Lookaheads Capture NLOG

4.1

Unary Non-Indexed Language
k

We consider the single exponential numerical language L1exp = {a2 : k ∈ N} over the
unary alphabet Σ = {a}. The language L1exp is represented by the REWB(+) expression
E1exp =?(a)x (?(\x\x)x )∗ \x. The part ?(a)x initializes x = a (i.e., x = a0 ), and the Kleene-∗
part iteratively doubles x.
Furthermore, we can represent the doubly exponential language L2exp = {a2
by the following REWBLk expression E2exp :
∗
E(α, β) = ?[(\α a)α ] ?[(\β\β)β ] ,
(it adds a to α and doubles β)
E2exp = ?((a)m ) E(n, m) ?((a)x ) E(y, x) ?(a∗ ?(\m$) ?(\y$)) \x.

2k

: k ∈ N}

n

It searches the numbers n, m, x, y that satisfy 2n = m, 2y = x, and m = y; so, x = 22 .
While unfolding the Kleene-* of E(n, m) and E(y, x), 2n = m and 2y = x hold. The part
?(a∗ ?(\m$) ?(\y$)) checks if m =? y by utilizing the negative lookahead expression $ = !(a).
We emphasize the known result L2exp ∈
/ IL, which is shown by the pumping or shrinking
lemma for indexed languages [14, 11]. Since we can carry out a similar construction of E2exp
in REWB(+) and REWB(−), we have the following result.
▶ Theorem 4. REWB(+) and REWB(−) can represent unary non-indexed languages.
Proof (Sketch). Due to the page limitation, we provide a proof sketch for the REWB(−)
part and put the complete proof in the Appendix of the long version. Let us consider the
following REWB(−) expression:
∗
E ′ (α, β) = (\α a)α (\β\β)β ,
′
E2exp
= (a)m E ′ (n, m) (a)x E ′ (y, x) !(!(\m$)) !(!(\y$)) a∗ .
′
While the expression E2exp
resembles E2exp , it lacks positive lookaheads. Let us explain
′
E2exp step-by-step:
1. The expressions (a)m and (a)x initialize m and x by a as with E2exp .
2. The subexpression E ′ (n, m) repeatedly expands variables n and m as with E2exp . So,
executing E ′ (n, m) actually consumes inputs as follows without positive lookaheads:
1

2

3

i

a1 n a2 m a2 n a2 m a3 n a2 m · · · ai n a2 m · · ·
3. The same holds for the expression E ′ (y, x).
The part !(!(\m$)) is a non-destructive positive lookahead by \m$ that is simulated by
double negative lookaheads; therefore, the part !(!(\m$)) !(!(\y$)) requires m = y. If we pass
the assertion, we consume the rest input by a∗ . By replacing positive lookaheads with actual
′
consuming, the language L(E2exp
) grows faster (the notion fast growth is formalized in the
′
Appendix) than L2exp . This property implies that L(E2exp
) is not an indexed language. ◀
It states that, even if restricted to unary languages, positive or negative lookaheads make
REWB expressive and incomparable to IL. We can also show the undecidability of the
emptiness problem, which is checking if L(E) = ∅ for a given expression E, of REWB(+)
and REWB(−) over Σ = {a}.
▶ Theorem 5. The emptiness problems of REWB(+) over a unary alphabet and REWB(−)
over a unary alphabet are undecidable.
The two undecidability results can be shown by translating the Post Correspondence Problem
to the emptiness problems. Due to the page limitation, we put the proof of Theorem 5 in
the Appendix of the long version.

<!-- page 9 -->

Y. Uezato

4.2

155:9

Simulating Two-Way Multihead Automata by REWBLk

We show that REWBLk can simulate two-way multihead automata, which are a classical
extension of automata and capture NLOG [12].

Simulating Two-Way One-Head Automata
We start from two-way one-head automata. Let A = (Q, qinit , qacc , Σ, ∆) be a two-way
automata where Q = {q0 , q1 , . . . , q|Q|−1 } is a finite set of states, qinit ∈ Q is the initial state,
qacc ∈ Q is the accepting state, Σ is an input alphabet, and ∆ is a set of transition rules. Each
τ /θ

transition rule is of the form p −−→ q where p, q ∈ Q, τ ∈ (Σ ∪ {⊢, ⊣}), and θ ∈ {−1, 0, 1}.
The component θ indicates a head moving direction: (1) if θ = −1 (resp. θ = 1), we move
the scanning head left (resp. right); (2) if θ = 0, we do not move the scanning head.
For an input w ∈ Σ∗ , we run A on the extended string ⊢ w ⊣, which are surrounded by
the left and right end markers. A configuration of A for ⊢ w ⊣ is a tuple (q, i) where q ∈ Q
and 0 ≤ i < 2 + |w|, and thus the current scanning symbol is (⊢ w ⊣)[i].
τ /θ

δ

A transition rule δ = p −−→ q ∈ ∆ gives a labelled transition relation ⇒ as follows:
δ

(p, i) ⇒ (q, i + θ) if (⊢ w ⊣)[i] = τ and 0 ≤ i + θ < |w| + 2
The word w is accepted by A if the initial configuration (qinit , 0), reading ⊢, has a
computation path to a configuration with the accepting state qacc . We now define the
language of A as follows:
δ

δ

δ

1
2
n
L(A) = {w : (qinit , 0) ⇒
(q1 , i1 ) ⇒
··· ⇒
(qacc , in )}.

To simulate A by REWBLk, we use some variables L, R, S ∈ Σ∗ . Intuitively, each
variable means the following:
If L = w (resp. R = w), A is located on ⊣ (resp. ⊢). Otherwise, ∃σ ∈ Σ. w = LσR. So,
L (resp. R) means the left (resp. right) part of w.
The length of S denotes the index i of the current state qi of A.
We formalize the above intuition as the following simulation ∼ between (q, i) and ⟨L, R, S⟩:
(qj , 0) ∼ ⟨ϵ, w, S⟩
if |S| = j,
(qj , |w| + 1) ∼ ⟨w, ϵ, S⟩ if |S| = j,
(qj , i) ∼ ⟨L, R, S⟩
if 1 ≤ i ≤ |w|, ∃σ. w = L σ R, |L| = i − 1, and |S| = j.
To represent all states, we need |w| ≥ |Q| − 1. So, we mainly consider to represent L(A) \ L′
where L′ = {w ∈ L(A) : |w| < |Q| − 1}. For instance, our simulation proceeds as follows:
(1)
(2)
(3)
(4)
(5)
(6)

⊢q0 ab ⊣ ∼ ⟨L = ϵ, R = ab, S = ϵ⟩, (if R = w, then L = ϵ and A is on ⊢)
⊢ aq1 b ⊣ ∼ ⟨L = ϵ, R = b, S = a⟩, (move right from (1))
⊢ abq1 ⊣ ∼ ⟨L = a, R = ϵ, S = a⟩, (move right from (2))
⊢ ab ⊣q1 ∼ ⟨L = ab, R = ϵ, S = a⟩, (if L = w, then R = ϵ and A is on ⊣)
⊢ abq2 ⊣ ∼ ⟨L = a, R = ϵ, S = ab⟩, (move left from (4) and change q1 to q2 )
⊢ aq2 b ⊣ ∼ ⟨L = ϵ, R = b, S = ab⟩.

As initialization, we use the expression Einit = (ϵ)L ?((Σ∗ )R $) (ϵ)S , which sets L = S = ϵ
and R = w for the input w. To move the head right, we use the following expression:
E+1 = ?((\LΣ)L Σ (Σ∗ )R $) + ?((\LΣ)L (ϵ)R $).

ICALP 2024

<!-- page 10 -->

155:10 Regular Expressions with Backreferences and Lookaheads Capture NLOG

The part ?((\LΣ)L Σ (Σ∗ )R $) first attempts to expand L toward the right and then updates
R. Similarly, we define the expression E−1 to move the head left as follows:
E−1 = ?((Σ∗ )L Σ (Σ\R)R $) + ?((ϵ)L (Σ\R)R $).
To check if the current scanning symbol is ⊣, ⊢, or σ ∈ Σ, we use the following expressions:
E⊣ = ?(\L$), E⊢ = ?(\R$), Eσ = ?(\L σ \R$).

To check if the current state is qi , we use the expression Eqi =? Σ∗ ?(\S)$ ?(Σi )$ . To
change the current state qi to qj , we use the expression Ei-to-j = Eqi ?(Σ∗ (Σj )S $).
Now, each transition rule δ is simulated by the following expression E(δ) defined as:
τ /0

E(qi −−→ qj ) = Ei-to-j Eτ ,

τ /θ

E(qi −−→ qj ) = Ei-to-j Eτ Eθ (θ ∈ {−1, +1}).

Finally, the following expression EA simulates A, and L(EA ) = L(A) holds:
EA = EL′ + ?(Eqinit (E(δ1 ) + E(δ2 ) + · · · + E(δn ))∗ Eqacc ) Σ∗
where EL′ is a regular expression for the finite language L′ , and ∆ = {δ1 , δ2 , . . . , δn }.
We summarize the our translation as follows.
▶ Lemma 6. For a two-way one-head automata A, there is an expression EA with 3 variables
(L, R, and S) such that L(EA ) = L(A). Especially, the expression uses negative lookaheads
only in the form of $.

Simulating Two-Way Multihead Automata
We extend the above argument to two-way multihead automata M [12, 15]. Compared to
two-way one-head automata A, M has multiple-heads on input strings. We write K for the
number of heads. The difference between A and M are the following:
Each configuration of M is a tuple (q, i1 , i2 , . . . , iK ) where q is the current state and ij is
the j-th head position.
(τ1 ,...,τK )/(θ1 ,...,θK )

Each transition rule is p −−−−−−−−−−−−−−→ q where p, q ∈ Q, τj ∈ Σ ∪ {⊢, ⊣} is used for
inspecting the scanned symbol by j-th head, and θj denotes the head moving direction
for the j-th head.
(τ1 ,...,τK )/(θ1 ,...,θK )

We define a transition relation ⇒ in the same way as for A. Let δ = p −−−−−−−−−−−−−−→ q
be a rule and C = (p, i1 , i2 , . . . , iK ) be a valid configuration. If ∀1 ≤ j ≤ K. (⊢ w ⊣)[ij ] = τj ,
δ

then we have C ⇒ (q, i1 + θ1 , i2 + θ2 , . . . , iK + θK ). We define the language as follows:
δ δ

δ

1 2
n
L(M) = {w : (qinit , 0, 0, . . . , 0) ⇒
⇒ ··· ⇒
(qacc , i1 , i2 , . . . , iK )}.

We can show the following lemma by simply extending our above construction for two-way
one-head automata.
▶ Lemma 7. For a given two-way K-head automata M, we have an expression EM with
3K variables such that L(EM ) = L(M). EM uses negative lookaheads only in the form of $.
Proof. As with the two-way one-head automata, for each i-th head, we prepare variables
Li , Ri , and Si . For each i-th head, by using the variables for i, we give expressions Eπi
where π ∈ {+1, −1, σ, ⊢, ⊣}. Employing the same Eqi and Ei-to-j , we can give E(δ) for each
transition rule δ of M and so EM .
◀
It is well-known that the class of languages accepted by two-way multihead automata
corresponds to NLOG [12]; so, we have the following theorem.
▶ Theorem 8. NLOG ⊆ REWBLk.

<!-- page 11 -->

Y. Uezato

4.3

155:11

Encoding Quantified Boolean Formula Problem

We translate the PSPACE-complete problem QBF, checking if a quantified boolean formula
(QBF) is true, into the membership problem of REWBLk. We only consider QBFs in CNF
since QBF restricted to CNF remains PSPACE-complete [3]. For instance, let us consider
the following QBF Q and translate it to the equivalent one Q′ by replacing ∀ with ¬∃¬:
Q : ∀a. ∃b. ∀c.∀d. (a∨b∨c)∧(b∨c∨d) ⇒ Q′ : ¬∃a.(¬∃b.(¬∃c.(∃d.(¬((a∨b∨c)∧(b∨c∨d)))))).
In order to check if Q is true, we first structurally translate Q′ into the following EQ′ :

E(v) = (T )v (F )v + (T )v (F )v
where v is a propositional variable,
EQ′ = !(E(a) !(E(b) !(E(c) E(d) !((\a + \b + \c)(\b + \c + \d))))),
where we replace ¬ with !, ∃x with E(x), x with \x, x with \x, and ∨ with +.
We then check w = T F T F T F T F T T ∈? L(EQ′ ). We explain the string w using the
annotated version T1 F2 T3 F4 T5 F6 T7 F8 T9 T10 : (1) the first two characters T1 F2 makes the two
cases where (a = T, a = F ) or (a = F, a = T ); (2) similarly, T3 F4 (resp. T5 F6 and T7 F8 )
works for b and b (resp. c, c and d, d) ; (3) by T9 , we check if the expression (a ∨ b ∨ c) holds
(in the negative context); (4) by T10 , we also check if (b ∨ c ∨ d) holds. Thus, Q is true iff
w ∈ L(EQ ).
On the basis of the above translation using E(v), we can translate every CNF-QBF Q to
the corresponding expression EQ and give the membership problem T F T F . . . T F T T . . . T ∈?
L(EQ ) in polynomial time for the size of Q. It implies the following result.
▶ Theorem 9. The membership problem of REWBLk is PSPACE-hard.

5

Log-space Nested-Oracles Nondeterministic Turing Machines

As we have stated in the Introduction, we utilize log-space nested-oracles NTMs. We will
translate REWBLk to them in the next section.
We first review log-space NTMs. Here we especially consider c-bounded k-working-tapes
log-space NTM M = (k, c, Q, qinit , QF , Σ, Γ, □, ∆). Each component of M means:
k is the number of working tapes T1 , T2 , . . . , Tk .
c is used to bound the size of working tapes. It will be defined precisely below.
Q is a finite set of states, qinit is the initial state, and QF ⊆ Q is a set of accepting states.
Σ is an input alphabet.
Γ is a working tape alphabet. □ ∈ Γ is the blank symbol for working tapes.
τ |θ

κ 7→ κ′ | θ

∆ is a set of transition rules. Each rule is either p −−→ q or p −−−−−−→ q where
p, q ∈ Q, τ ∈ Σ ∪ {ϵ, ⊢, ⊣}, κ, κ′ ∈ Γ ∪ {⊢, ⊣}, and θ ∈ {−1, +1, 0}.

Ti

Let ⊢ w ⊣ (w ∈ Σ∗ ) be a string surrounded by the left and right end markers. Valid
configurations of M for ⊢ w ⊣ are tuples ⟨q, i, (T1 , i1 ), . . . , (Tk , ik )⟩ where
q ∈ Q is the current state. i ∈ N (0 ≤ i < |w| + 2) is the current head position on ⊢ w ⊣.
Tx ∈ (⊢ ΓC ⊣) where C = c · ⌈log |w|⌉ is the x-th working tape surrounded by the end
markers. ⌈·⌉ is the ceiling function to integers; for example, ⌈log 3⌉ = ⌈1.584 . . .⌉ = 2.
Remark: The tape capacity C is determined by the parameter c and the input w.
ix is the x-th tape head position on Tx (0 ≤ ix < C + 2).

ICALP 2024

<!-- page 12 -->

155:12 Regular Expressions with Backreferences and Lookaheads Capture NLOG

We write ValidM (w) (or, simply Valid(w)) for the set of valid configurations for the
input w. It is clear that |Valid(w)| = |Q|×(|w|+2)×(|Γ|C ×(C +2))k where C = c·⌈log |w|⌉.
For an input string w, we write I(w) to denote the initial configuration on ⊢ w ⊣:
I(w) = ⟨qinit , 0, (⊢ □C ⊣, 0), . . . , (⊢ □C ⊣, 0)⟩ where C = c · ⌈log |w|⌉.
Let ξ = ⟨p, i, (T1 , i1 ) . . . , (Tx , ix ), . . . , (Tk , ik )⟩ be a valid configuration on ⊢ w ⊣. For each
δ
transition rule δ, we define a labelled transition relation ⇒ on valid configurations as follows:
τ |θ

δ = p −−→ q

(⊢ w ⊣)[i] = τ (if τ ̸= ϵ)

δ

ξ ⇒ ⟨q, i + θ, (T1 , i1 ), . . . , (Tk , ik )⟩

κ 7→ κ′ | θ

δ = p −−−−−−→ q
Tx

κ = Tx [ix ]

δ

ξ ⇒ ⟨q, i, . . . (Tx [ix ] := κ′ , ix + θ) . . .⟩

where Tx [ix ] := κ′ is the new working tape obtained by writing κ′ to the position ix .
δ

We also simply write ξ ⇒ ξ ′ if there is a transition rule δ ∈ ∆ such that ξ ⇒ ξ ′ .
We write NLOG(c, k) for the set of c-bounded k-working-tapes log-space NTMs. If c
and k is not important, by abusing notation, we simply write NLOG. For M ∈ NLOG and
an input string w, we write M (w, ξ) to denote the set of valid and acceptable configurations
that are reachable from a valid configuration ξ on ⊢ w ⊣:
M (w, ξ) = {ξ ′ : ξ ⇒∗ ξ ′ , ξ ′ = ⟨qacc , i, T ⟩, qacc ∈ QF },
where T = (T1 , i1 ) . . . (Tk , ik ) is a sequence of pairs of a working tape and an index. Now the
language L(M ) is defined as:
L(M ) = {w : M (w, I(w)) ̸= ∅}.
Here we state a useful proposition, which will be used below sometimes.
▶ Proposition 10. Let M ∈ NLOG(c, k). For any input w, to represent a single valid
configuration or store |Valid(w)|, we need an extra O(c · k)-bounded working tape.
Proof. Since |Valid(w)| = |Q| × (|w| + 2) × (|Γ|C × (C + 2))k where C = c · ⌈log |w|⌉,
log |Valid(w)| = (k · C) log |Γ| + · · · = O(k · c) log |w|. So, we need an O(c · k)-bounded
tape.
◀
On log-space NTMs, we can solve the problem-(B) in Section 1.1.
▶ Proposition 11. Let M ∈ NLOG(c, k). There exists N ∈ NLOG(O(c · k), k + 1) such
that L(M ) = L(N ) and, for any input w, all computations of N starting from w eventually
halt.
Proof. The number of reachable configurations is bounded by B = |ValidM (w)|. So, we can
safely ignore all paths P whose length > B without changing the accepting language. To
check if the current path length > B, we need an O(c · k)-bounded tape by Proposition 10. ◀
We note that properties, like Proposition 11, are insufficient to show that a language
class is closed under complement.1 Thus, the problem-(A) in Section 1.1 is essentially hard;
indeed, it is the interesting part of Immerman–Szelepcsényi theorem [16, 27]. In the following
subsection, we revisit their theorem along with introducing log-space nested-oracles NTM.

1

For example, we can translate any nondeterministic pushdown automata to real-time ones, which do
not have ϵ-transitions. However, the class of context free languages is not closed under complement.

<!-- page 13 -->

Y. Uezato

5.1

155:13

Augmenting NTM with Nested Oracles

We extend log-space NTM with finitely nested oracles (or subroutines) to naturally handle
nested lookaheads of REWBLk. Similar to our definition of log-space NTMs, we consider
c-bounded k-tapes log-space nested oracles NTMs.
To allow nested oracle calling, we inductively define our machines. First, as the base
case, we write OLOG0 (c, k) = NLOG(c, k) to denote machines without oracles. Next, as
the induction step, we define OLOGx+1 (c, k) using OLOGx (c, k) as follows:
Each M ∈ OLOGx+1 (c, k) is a tuple (k, c, Q, qinit , QF , Σ, Γ, □, ∆).
̸∈N

∈N

There are new transition rules, oracle transition rules, of the form p −−→ q and p −−→ q
where N ∈ OLOGy (c, k) and y ≤ x. Their semantics will be defined immediately later.
S∞
We write OLOGω (c, k) for i=0 OLOGi (c, k). If c and k are not important, we simply
write as OLOGn and OLOGω . To denote the nesting level of machines M ∈ OLOGω (c, k),
we inductively define the function depth as follows:
(
0
if M ∈ OLOG0 (c, k),
depth(M ) =
∈N
∈N
/
1 + max{depth(N ) : p −−→ q, p′ −−→ q ∈ ∆(M )} otherwise.
Now, we define the semantics of oracle transition rules as follows:
∈N

p −−→ q

∈N
/

N
N (w, ⟨qinit
, i, T ⟩) ∋ ⟨r, j, U⟩
⟨p, i, T ⟩ ⇒ ⟨q, i, U⟩

N
, i, T ⟩) = ∅
p −−→ q
N (w, ⟨qinit
⟨p, i, T ⟩ ⇒ ⟨q, i, T ⟩

N
where qinit
is the initial state of N , T and U is a sequence of pairs of a working tape and an
index (T1 , i1 ) . . . (Tk , ik ), and the function

N (w, ξ) = {ξ ′ : ξ ⇒∗ ξ ′ , ξ ′ = ⟨q, i, T ⟩, q ∈ QF (N )}
is defined inductively on the depth of machines.
∈N
The semantics of p −−→ q means that: (1) we call an oracle (subroutine) N for ⊢ w ⊣
with the current position i and the current working tapes T as its initial working tapes; and
(2) if N accepts w, then we enter a state q with the original position i and working tapes U
∈N
/

of N ’s accepting configuration. The semantics of p −−→ q means that, if N does not accept
w, we enter a state q with the original position and working tapes T .2
▶ Example 12. Using log-space nested-oracles NTMs, we simulate the following REWBLk:
Ereach = (V ∗ )Cur # ?(Σ∗ # \Cur → (V ∗ )Cur #)

∗

Σ∗ # \Cur.

For the sake of simplicity, we assume that V = {⋆} is a unary alphabet. The subexpression
(Σ∗ # \Cur → (V ∗ )Cur #) is simulated by the following log-space NTM M ∈ NLOG:
Σ
qinit

2

#

Checking if the substring
from the current head position
equals to Cur

q1

q2

Store the number of
consecutive ⋆’s
to Cur
→

q3

q4

#

qacc

.

For simplicity, our oracle formalization differs from traditional treatments [17, 21, 26, 3] in some points:
(1) we omit the use of oracle tapes, and (2) we allow inheriting configurations from called oracles.
Despite these differences, our definition is adequate for Theorem 13 and for REWBLk in Section 6.

ICALP 2024

<!-- page 14 -->

155:14 Regular Expressions with Backreferences and Lookaheads Capture NLOG

Using M , we give the following machine N ∈ OLOG1 , clearly accepting L(Ereach ):

qinit

Store the number of
∈M
Σ
consecutive ⋆’s
to Cur
#
nop
⊢

q0

q1

q2

q3

Checking if the substring
from the current head position
equals to Cur
#
⊣

q4

q5

qacc

nop

where edges labelled with “nop”, −→, mean transitions that only change states and do not
nop

ϵ|0

depend scanning symbol. It just a syntax sugar because we can define p −→ q by p −−→ q.
▶ Example 13. We can also accept the language of non-reachability problems:
Lnon-reach = {s # x1 → y1 # · · · #xn → yn #t : there is no path from s to t}.
To recognize this language, we use the following Mnon-reach ∈ OLOG2 :
qinit

⊢

q0

∈ N ′′

q1

∈
/N

qacc

where N ′′ ∈ NLOG recognizes the language defined by V ∗ # (V ∗ → V ∗ #)∗ V ∗ .

5.2

Collapsing OLOGω by Immerman–Szelepcsényi Theorem

Thanks to Proposition 11, we can give a decision procedure to check w ∈? L(Mnon-reach ) for
our above examples. However, it is not clear that OLOG2 =? NLOG and more generally
OLOGω =? NLOG. For example, is there a log-space NTM N that recognizes Lnon-reach ?
Fortunately, the class NLOG is closed under complement, NLOG = co-NLOG. This
result is known as Immerman–Szelepcsényi theorem [16, 27]. We employ their proof to
collapse OLOGx for some x to log-space NTMs OLOG0 = NLOG.
▶ Lemma 14. Let M ∈ NLOG(c, k). There is a machine M ∈ NLOG(O(c · k), k + ∂)
where L(M ) = Σ∗ \L(M ) and ∂ is independent of M , c, and k.
Proof. We review Immerman’s original construction in [16]. For the reader who would like to
know more detailed explanation about his construction, we recommend some literature [26].
His construction consists of the following two parts:
Let Start be a configuration of M . First, we compute the number C, the total number
of configurations reachable from Start.
Next, using C, we check if there is a path from Start to an acceptable configuration.
The first part is accomplished by the following pseudocode [16, Lemma 2].
global w; // input string
// For configurations x and x′ , we check if x ⇒ x′
def one_stepM (x, x′ ):
foreach δ ∈ ∆(M ): // ∆(M ) is the set of transition rules of M
δ
if x ⇒ x′ : return True;
return False;
// calculate the total number of configurations reachable from Start
def countingM (Start):
cur ← 1; // the number of reachable configurations within ≤ dist steps

<!-- page 15 -->

Y. Uezato

155:15

for(dist ← 0,next ← 0; dist < |ValidM (w)|; dist += 1, cur ← next, next ← 0):
foreach x ∈ ValidM (w):
count ← 0; found_x ← false;
foreach y ∈ ValidM (w):
z ← Start; // search a path from Start to y
for(i ← 0; z ̸= y & i < dist; i += 1):
z ′ ← Nondeterministically generated configuration;
if one-step(z, z ′ ): z ← z ′ ;
else: break;
if z = y:
count += 1;
if one-step(y, x): { next += 1; found_x ← true; break; }
if ¬found_x & count ̸= cur: Halt and reject;
return cur;
These functions require extra working tapes at least for variables of δ, cur, dist, next, x,
count, y, i, z, and z ′ . By Proposition 10, for each variable, we need an O(c · k)-bound working
tape.
The second part is accomplished by the following pseudocode [16, Lemma 1].
// Judge whether an acceptable configuration can be reached from Start.
// If so, it returns such a configuration y by Some(y).
// Otherwise, we return the nothing by None.
def judgeM (Start):
C ← countingM (Start);
count ← 0;
foreach x ∈ ValidM (w):
y ← Start;
for(i ← 0; i ≤ C; i += 1):
y ′ ← Nondeterministically generated configuration;
if one-step(y, y ′ ):
y ← y′ ;
if y is an accepting configuration: return Some(y);
if y = x: { count += 1; break; }
else: break;
if count = C: return None;
else: Halt and reject;
This function also requires extra O(c · k)-bound working tapes.
Now we can build M ∈ NLOG(O(c · k), k + ∂) as a log-space NTM that simulates the
function judge and then accepts inputs if judgeM (Start) is None.
◀
Repeatedly applying Immerman’s construction collapses nested oracle machines to machines without oracles [16, Corollary 2].
▶ Theorem 15. Let M ∈ OLOGn (c, k) be a log-space n-nested-oracles NTM. There exists
a log-space NTM N ∈ NLOG(O(c · k n ), O(k · n)) such that L(M ) = L(N ).
Proof. We eliminate oracle transitions from the innermost to the outermost for M as follows.
∈N
We replace p −−→ q with N ∈ OLOG0 with multiple transition rules that perform: (1) save
the head position H to an extra tape; (2) run N ; (3) if we reach an accepting configuration

ICALP 2024

<!-- page 16 -->

155:16 Regular Expressions with Backreferences and Lookaheads Capture NLOG

∈N
/

⟨qf , T ⟩, then we continue ⟨q, H, T ⟩. Similarly, we replace p −−→ q with N ∈ OLOG0 with
multiple transition rules using N obtained by Immerman’s construction. We emphasize that
each generation of N increases c and k in the order of the statement of Lemma 14.
◀

5.3

Membership Problem of OLOGω

We now show that the membership problem of log-space nested-oracles machines is in
PSPACE. We first formally state our membership problem.
▶ Definition 16 (Membership problem of OLOGω ). The membership problem of OLOGω is
a decision problem of the following form:
Inputs Binary encoded integers c and k. A machine M ∈ OLOGω (c, k). A word w ∈ Σ∗ .
Output If w ∈? L(M ), return Yes. Otherwise, No.
To show that the problem belongs to PSPACE, we would like to employ Theorem 15.
However, it is not feasible because the theorem gives a log-space O(c · k |M | )-bounded tapes
machine N in general; i.e., N demands O(c · k |M | ) · log |w| space. Thus, we cannot simulate
N in polynomial size in c, k, |M |, and |w|. To address this problem, we adopt Immerman’s
construction for OLOGω in an interpreter style.
▶ Theorem 17 (Membership problem of OLOGω belongs to PSPACE). Let w be an input
word and M ∈ OLOGω (c, k) be an input machine where c and k are binary encoded. We
can decide if w ∈? L(M ) in polynomial space in c, k, |w|, and |M |.
Proof. First, we extend the function one-step for oracle transitions as follows:
def one-stepMi (x, x′ ): // return True if x ⇒ x′ . Otherwise, False.
foreach δ ∈ ∆(Mi ):
δ
if δ is a non-oracle transition & x ⇒ x′ : return True;
else: // δ is an oracle transition
(p, i, T ) ← x; // extract state, position, and tape contents from x
∈N
if δ = p −−→ q:
N
, i, T ⟩): // pattern matching
match judgeN (⟨qinit
′
′
case Some(⟨p , i , U⟩) -> return (x′ =? ⟨q, i, U⟩);
case None -> return False;
∈N
/
if δ = p −−→ q:
N
match judgeN (⟨qinit
, i, T ⟩):
′
′
case Some(⟨p , i , U⟩) -> return False;
case None -> return (x′ =? ⟨q, i, T ⟩);
return False;
Next, we generate the codes of one-stepMi , countingMi , and judgeMi for all oracle
machines Mi that appears in M . Such generation is carried out in polynomial-time for |M |.
The total size of generated code is also polynomial in |M |.
We can also provide an interpreter for the generated code in polynomial time for |M |.
While this interpreter needs a call stack for function calls, its depth is bounded by depth(M ) ≤
|M |. Additionally, the size of each stack frame is bounded by O((c·k) log |w|) by Proposition 10.
From the above argument, we can check w ∈? L(M ) using (nondeterministic) polynomial
space with respect to c, k, |w|, and |M |.
◀
We will use this theorem to show that the membership problem of REWBLk belongs to
PSPACE in the following section. To this end, we put remarks about this theorem.

<!-- page 17 -->

Y. Uezato

155:17

Remark (Drop k from Theorem 17). We can decide w ∈? L(M ) in polynomial space for c,
|w|, and |M |. Compared with Theorem 17, this refined version does not depend on k. This
is because k ≤ |M | holds, even when k is binary encoded.
Of course, to establish this property, we assume a natural restriction that all tapes
T1 , . . . , Tk are used in some transition rules. Indeed, if M ∈ OLOGω (c, k) has a working
tape Ti with 1 ≤ i ≤ k that is not engaged by M , a better parameter k ′ (where k ′ < k) should
be employed, ensuring M ∈ OLOGω (c, k ′ ). Given such constraints, |M | ≥ k · log(k) ≥ k
since k tapes necessitate log(k)-space for tape identification.

Remark (Cannot drop c from Theorem 17). On the other hand, we cannot drop c from
Theorem 17. Namely, we cannot decide w ∈? L(M ) in polynomial space for |w| and |M |
because c is not bounded by Poly(|M |) in general. (Please recall that k ≤ |M |.) It can be
understood from the following argument, which establishes c = Ω(2|M | ). Let us consider a
machine M that runs as follows:
First, M fills a tape T1 with N 1’s by using states q1 , q2 , . . . , qN .
Next, M interprets T1 as a binary number t1 and fills a tape T2 with t1 1’s.
Finally, M writes the contents of T2 N times to T3 .
3

For example, if N = 3, then M makes T1 = 111 and then makes T2 = 1111111 = 12 −1
3
N
and then T3 = 3 T2 = 13·(2 −1) = 1N ·(2 −1) .
The size of M satisfies |M | = O((log N )N ) since it contains binary-encoded N states; thus,
the length of T3 , |T3 |, satisfies |T3 | = Ω(2|M | ). By appending the content of T3 onto another
tape log |w| times (where w is the input word), we establish that c = Ω(2|M | ).
However, fortunately we can assume c = 1 when simulating REWBLk by OLOGω as we
will see below. It is crucial for showing that the membership problem of REWBLk belongs
to PSPACE.

6

From REWBLk to Log-space NTM via OLOGω

We finally show REWBLk ⊆ NLOG by translating REWBLks to OLOGω .
▶ Theorem 18. Given a REWBLk expression E, we can translate it to M ∈
OLOGω (1, O(|E|)) in polynomial time in the size of |E| where L(E) = L(M ).
Proof. We inductively translate a REWBLk expression E to an OLOGω T (E) such that
w ∈ L(E) if and only if T (E) successfully runs through w when started with w instead of
⊢ w ⊣.
As we will see below, the generated T (E) has a unique source state s, which does not have
incoming edges, and a unique sink state t, which does not have outgoing edges. Please recall
that E accepts an input w if it consumes all the input, i.e., if we have ⟨p, Λ⟩ ∈ J⟨E, w, 0, ι⟩K
with p = |w|. To build our M , we add two transition rules to T (E) to treat the end markers
⊢ and ⊣ as follows:
⊢ | +1

M=

i

s T (E) t

⊣| 0

f

where i and f are initial and accepting states of M .

ICALP 2024

<!-- page 18 -->

155:18 Regular Expressions with Backreferences and Lookaheads Capture NLOG

Translating REGEX Part
First, we give a translation for the REGEX as follows:
σ Z⇒ s

σ | +1

t,

ϵ Z⇒ s

ϵ|0

t,

E1 E2 Z⇒ s1 T (E1 ) t1

nop

s2 T (E2 ) t2 ,

nop
∗

(E) Z⇒ s

nop

s′ T (E) t′

nop

nop s1 T (E1 ) t1

t ,

E1 + E2 Z⇒

s

nop

t

,

nop s2 T (E ) t nop
2
2

where the edges labelled with “nop” are the same ones used in Example 12, which just change
states. This translation is identical to the McNaughton–Yamada–Thompson algorithm, which
is well-known and found in textbooks of automata theory.

Translating Backreference and Capturing Expressions
Using Theorem 2, we can assume that every variable x does not appear inside an expression
capturing x; i.e., we avoid patterns such that (· · · x · · · )x . The following translation heavily
depends on the theorem.
We now focus on the part (E)x of REWB:

(E)x Z⇒ s

save the current head position
in binary form to the tape Txl

s′ T (E) t′

save the current head position
in binary form to the tape Txr

t .

In order to keep the start position of the variable x, we first copy the current head position to
the special working tape Txl in binary form. Then, we execute the expression E by running
from the state s to t. Finally, we record the new head position into the working tape Txr .
Now, w[xl .. xr ) = w[xl ]w[xl + 1] · · · w[xr − 1] is a substring matched with the expression E
where xl and xr are the numbers corresponding to the contents of Txl and Txr .
Next, we focus on the part \x of REWB:
\x Z⇒ s

check if the current head starts with the substring of w[xl .. xr )

t .

As we have seen above, the substring w[xl .. xr ) denotes the value of the variable x. This
checking task is accomplished using an extra tape Ttmp without changing Txl and Txr .
Remark: Why do we need Theorem 2. Let us consider an expression E = (a)x (\x\x)x
and run it for an input string a. We underline a ∈
/ L(E). By interpreting (a)x , we set the
position 0 to Txl and 1 to Txr . On the translation of (\x\x)x , we first save the current head
position 1 to the tape Txl and then proceed to the part \x\x. It should be noted that, at this
point, Txl denotes 1; so, we cannot correctly recover the captured content by (a)x . When
meeting \x, we check if the substring between Txl (= 1) and Txr (= 1) starts from the current
head position. Since the substring is the empty string ϵ, we go through the part \x\x and
accepts a incorrectly.
To prevent us from this situation, we use Theorem 2. It rewrites E to (a)x ?((\x\x)y )(\y)x ,
and the rewritten expression is safely interpreted thanks to the variable y.

<!-- page 19 -->

Y. Uezato

155:19

Translating Positive and Negative Lookaheads
For our construction, we need that lookahead are augmented with a continuation K, for
instance ?(E)K and !(E)K. If a given expression does not satisfy this property, we add the
expression ϵ, which always succeeds. For example, (E1 E2 !(E3 ))∗ E4 Z⇒ (E1 E2 (!(E3 )ϵ))∗ E4 .
Now, we can easily translate ?(E)K and !(E)K using oracle transition rules as follows:
?(E)K Z⇒ s

∈ T (E)

s′ T (K) t′ ,

!(E)K Z⇒ s

∈
/ T (E)

s′ T (K) t′ .

By the semantics of our nested oracle machines, it is clear that L(E) = L(M ).

◀

As corollaries of Theorem 18, we have the following main results.
▶ Corollary 19. REWBLk = NLOG.
Proof. We already have NLOG ⊆ REWBLk by Theorem 8. It is an immediate result
from Theorem 15 and 18 that REWBLk ⊆ NLOG.
◀
▶ Corollary 20. The membership problem of REWBLk is PSPACE-complete.
Proof. By Theorem 9, the membership problem of REWBLk is PSPACE-hard.
The result that the problem belongs to PSPACE is shown as follows. Let E be an
input REWBLk expression and w be an input string. By Theorem 17, we translate
M ∈ OLOGω (1, O(|E|)) such that L(E) = L(M ). This translation is carried out in
polynomial time in |E|; so, |M | = Poly(|E|). By Theorem 17 and its remarks, we can check
if w ∈? L(M ) in polynomial space in |w| and |M |. This also derives that we can check if
w ∈? L(E) in polynomial space in |w| and |E|.
◀

7

Future Work and Conclusion

We have shown that the language class REWBLk – regular expressions plus backreferences
and lookaheads (without any restrictions) – captures the class NLOG. Our result closes
the expressiveness about REWBLk. Furthermore, we have shown that the membership
problem of REWBLk is PSPACE-complete. On the other hand, it remains unclear whether
REWB(+) and REWB(−) are proper subclasses of REWBLk. For example, we conjecture
that REWB(+) cannot recognize the language Lprime = {an : n is a prime number}.3 It is
known that the language can be represented by REWB(−). Also, we conjecture that some
NLOG-languages cannot be recognized by REWB(−).
References
1
2
3

3

A. V. Aho. Indexed grammars – an extension of context free grammars. J. ACM, 15(4):647–671,
1968.
A. V. Aho. Algorithms for finding patterns in strings. In J. V. Leeuwen, editor, Algorithms
and Complexity, Handbook of Theoretical Computer Science, pages 255–300. Elsevier, 1990.
S. Arora and B. Barak. Computational Complexity: A Modern Approach. Cambridge University
Press, 2009.

REWB(−) recognizes Lprime as follows: (1) we define Ecomposite = (aa a∗ )w \w\w∗ $, which recognizes
composite numbers; (2) then, we define Enon-prime = a$ + Ecomposite , which recognizes non-prime
numbers; (3) finally, we define Eprime = !(Enon-prime ) a∗ $. Readers interested in conducting a prime
test with Eprime can try the following expression in C#: @"^(?!((a$)|((?<W>(aa+))(\k<W>+)$))).*$".

ICALP 2024

<!-- page 20 -->

155:20 Regular Expressions with Backreferences and Lookaheads Capture NLOG

4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27

M. Berglund and B. van der Merwe. Re-examining regular expressions with backreferences.
Theoretical Computer Science, 940:66–80, 2023.
C. Câmpaanu, K. Salomaa, and S. Yu. A formal study of practical regular expressions.
International Journal of Foundations of Computer Science, 14(06):1007–1018, 2003.
A. Chandra, D. Kozen, and L. Stockmeyer. Alternation. J. ACM, 28(1):114–133, 1981.
N. Chida and T. Terauchi. On lookaheads in regular expressions with backreferences. In
FSCD 2022, volume 228, pages 15:1–15:18. Schloss Dagstuhl, 2022.
N. Chida and T. Terauchi. On lookaheads in regular expressions with backreferences. IEICE
Transactions on Information and Systems, E106.D(5):959–975, 2023.
ECMAScript community. Ecmascript 2023 language specification. https://262.ecmainternational.org/14.0/#sec-runtime-semantics-repeatmatcher-abstract-operation.
D. D. Freydenberger and M. L. Schmid. Deterministic regular expressions with back-references.
Journal of Computer and System Sciences, 105:1–39, 2019.
R. H. Gilman. A shrinking lemma for indexed languages. Theoretical Computer Science,
163(1):277–281, 1996.
J. Hartmanis. On non-determinancy in simple computing devices. Acta Informatica, 1(4):336–
344, 1972.
J. Hartmanis and S. Mahaney. Languages simultaneously complete for One-way and Two-way
log-tape automata. SIAM Journal on Computing, 10(2):383–390, 1981.
T. Hayashi. On derivation trees of indexed grammars—an extension of the uvwxy-theorem—.
Publications of the Research Institute for Mathematical Sciences, 9(1):61–92, 1973.
M. Holzer, M. Kutrib, and A. Malcher. Complexity of multi-head finite automata: Origins
and directions. Theoretical Computer Science, 412(1):83–96, 2011.
N. Immerman. Nondeterministic space is closed under complementation. SIAM Journal on
Computing, 17(5):935–938, 1988.
N. Immerman. Descriptive Complexity. Springer Verlag, 1998.
T. Jiang and B. Ravikumar. A note on the space complexity of some decision problems for
finite automata. Information Processing Letters, 40(1):25–31, 1991.
K.-J. Lange, B. Jenner, and B. Kirsig. The logarithmic alternation hierarchy collapses:
L
AΣL
2 = AΠ2 . In ICALP 87, pages 531–541. Springer, 1987.
T. Nogami and T. Terauchi. On the expressive power of regular expressions with backreferences.
In MFCS 2023, volume 272, pages 71:1–71:15. Schloss Dagstuhl, 2023.
C. H. Papadimitriou. Computational complexity. Addison-Wesley, 1994.
W. C. Rounds. Complexity of recognition in intermediate level languages. In SWAT’73, pages
145–158, 1973.
M. L. Schmid. Inside the class of regex languages. International Journal of Foundations of
Computer Science, 24(07):1117–1134, 2013.
M. L. Schmid. Characterising regex languages by regular languages equipped with factorreferencing. Information and Computation, 249:1–17, 2016.
U. Schöning and K. W. Wagner. Collapsing oracle hierarchies, census functions and logarithmically many queries. In STACS 88, pages 91–97. Springer, 1988.
M. Sipser. Introduction to the theory of computation. Cengage Learning, third edition, 2013.
R. Szelepcsényi. The method of forced enumeration for nondeterministic automata. Acta
Informatica, 26(3):279–284, 1988.
