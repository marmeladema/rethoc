---
file: ../pdfs/2019-arxiv-schmid-backreferences-polynomial-matching.pdf
citation: "Regular Expressions with Backreferences: Polynomial-Time Matching Techniques. Schmid. arXiv 2019 (rev. 2024)"
doi: 
source: "https://arxiv.org/pdf/1903.05896"
topics: [redos_backrefs_verification]
pages: 27
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Regular Expressions with Backreferences:
Polynomial-Time Matching Techniques
Markus L. Schmid1
1

Humboldt-Universität zu Berlin, Berlin, Germany, MLSchmid@MLSchmid.de

arXiv:1903.05896v2 [cs.FL] 8 Feb 2024

Abstract
Regular expressions with backreferences (regex, for short), as supported by most modern
libraries for regular expression matching, have an NP-complete matching problem. We
define a complexity parameter of regex, called active variable degree, such that regex with
this parameter bounded by a constant can be matched in polynomial-time. Moreover, we
formulate a novel type of determinism for regex (on an automaton-theoretic level), which
yields the class of memory-deterministic regex that can be matched in time O(|w|p(|α|)) for
a polynomial p (where α is the regex and w the word). Natural extensions of these concepts
lead to properties of regex that are intractable to check.

1

Introduction

Regular expressions were first introduced by Kleene in 1956 [31] as a theoretical concept (an
early implementation is due to Thompson [43]). Since then, they have been enriched with
practically motivated extensions and modifications, which is mainly due to their rather high
practical relevance (see the IEEE POSIX standard [30] or the W3C recommendations [9, 26, 29],
see [25] for an overview of the role of regular expressions as a practical tool, and also note that
variants of regular expressions are intensively investigated in the database theory community, due
to their relevance for graph databases (see, e. g., [32, 6, 8, 33, 4, 7, 36] or [5] for a survey) and the
information extraction framework of document spanners (see, e. g., [2, 19, 13, 21, 42, 39, 40] or [41]
for a survey). Regular expressions have excellent decidability- and complexity-properties, while
at the same time providing expressive power that is sufficient for many important computational
tasks. Most of the practical enhancements added over the years are mere “syntactic sugar” and
therefore preserve these positive properties. However, adding so-called backreferences drastically
increases expressive power and therefore leads to intractability and even undecidability.
A backreference in a regular expression is a possibility to repeat the subword matched to a
specific subexpression. For example, the x{. . . }-construct in the expression r = x{(a ∨ b)∗ }cx
stores in variable x whatever subword is matched to the subexpression (a ∨ b)∗ , and the following
occurrence of variable x then refers to exactly this subword (thus, r describes the non-regular
language {wcw | w ∈ {a, b}∗ }). In the following, we denote regular expressions with such
backreferences by the term regex. The matching problem of regex, i. e., deciding whether a given
regex can match a given word, is NP-complete (even for strongly restricted variants) [1, 16, 17,
15], and decision problems like inclusion, equivalence and universality are undecidable [20] (even
if the input expressions only use one variable with only a bounded number of occurrences).
Nevertheless, regular expression libraries of almost all modern programming languages (like,
e. g., Java, PERL, Python and .NET) support backreferences (although they syntactically and
even semantically slightly differ from each other (see the discussion in [24])), and they are even
part of the POSIX standard [30].

1.1

The Regex Matching Problem

The arguably most important problem for practical considerations is the matching problem.
Its general NP-completeness was shown in [1], but also follows from matching patterns with
variables [3], i. e., checking whether the variables xi in a pattern α ∈ (Σ ∪ {xi | i ∈ N})∗ can
be uniformly replaced by words from Σ∗ in order to obtain a given word (see [35] for a survey
1

<!-- page 2 -->

or the more recent publications [15, 11, 12]). These patterns are a quite successful tool for
obtaining negative results for regex,1 but the many known positive algorithmic approaches to
matching patterns (see [37, 14, 11, 12]) are tailored to the “backreferencing-aspect” and seem
unfit for handling the “regular expression-aspect” of regex. In fact, even though there are many
deep theoretical (yet negative) results about the complexity and decidability of regex, positive
algorithmic approaches are rather scarce.
In [24], deterministic regex (det-regex ) are introduced.2 Since they are characterised via a
purely deterministic automaton model, they can be matched efficiently and, if further restricted,
they have some decidable problems in static analysis (their language theoretical properties have
been thoroughly investigated in [24]). However, if efficient matchability is our main concern,
det-regex seem unnecessarily restricted, since they do not cover all regular languages. In fact,
det-regex cover very well what it means for a regex to be deterministic in the strongest possible
way, but not quite what it means to be “easily matchable”.

1.2

Our Contribution

We develop two different approaches to efficient regex matching:
• Regex with bounded active variable degree: We define a complexity parameter
of regex, called active variable degree (denoted by avd(α)), and show that regex can be
matched in time |α||w|O(avd(α)) . Note that |α||w|O(var(α)) is a trivial upper bound, where
var(α) is the total number of variables of α, and that avd(α) is always upper bounded
by var(α). Intuitively speaking, the parameter avd(α) measures the number of variables
that can be active at the same time in a match, and the algorithmic application relies in
devising a matching procedure, which, in a sense, reuses variables that are currently not
active. This approach can also be seen as a technique to reduce the number of variables
of a regex, a problem that, in its general form, is undecidable (see [20]).
• Memory-deterministic regex: We come up with a possibility to limit the inherent nondeterminism of regex to those parts that have nothing to do with backreferences. The
thus obtained class of memory-deterministic regex enforces some synchronisation between
different computational branches in a matching procedure, and therefore can be matched
in time p(|α|)|w| for a (low-degree) polynomial p. This means that matching memorydeterministic regex can be done in time linear in |w| if measured in data complexity.3 This
is worth pointing out, since the full class of regex can most likely not be matched in time
f (|α|)g(|w|) for any polynomial g and computable function f , or for any polynomial f and
computable function g (this follows from the W[1]-hardness of the problem if parameterised
by the size of the regex or by the size of the input word [17]). The concept of memory
determinism is rather complicated, since it cannot be achieved by some local and syntactical
restrictions. Hence, a main challenge is to show that memory determinism can be checked
efficiently.
These positive results are complemented with lower bounds. The active variable degree can
be improved to a much stronger complexity parameter (that also can be exploited in similar
ways), but computing it is coNP-hard. The development of memory determinism is carefully
governed by intractability results as follows: First, we show that even rather strong restrictions
of non-determinism will lead to an intractable matching problem, as long as these restrictions
are of a local and syntactical nature. This observation leads to a regex-property that is entirely
non-syntactic in the sense that it is formulated with respect to the possible matchings. While
this property is sufficient for efficient matching, it is also coNP-hard to be checked for. The
concept of memory determinism results from finding a balance between matching-complexity
and the complexity of checking the property.
1 The undecidability results of [20] also follow from the fact that regex can describe systems of patterns.
2 Deterministic (classical) regular expressions are an established concept [10, 27, 34, 28].
3 Data complexity is motivated by considerations in database theory; in this regard, a regex can be seen as a

(usually short) query that is to be evaluated on a (potentially large) data-object, i. e., the word.

2

<!-- page 3 -->

1.3

Techniques

Our main algorithmic tool is memory automata (MFA), a recently introduced automaton-based
characterisation of regex (see [38, 24]). If regex are represented as MFA, their structure is much
easier to analyse and we can conveniently abstract from the actual backreferences by interpreting
an MFA as an NFA that accepts a regular language with special meta-symbols. This point of view
is vital and provides the necessary leverage for developing our concepts and proving the respective
results. In this way, we are able to define and exploit the active variable degree by analysing the
automaton-structure underlying the regex, and the development of memory determinism will
also be done on the level of MFA. To the knowledge of the author, restricting non-determinism
by talking about computations of the automaton rather than syntactical properties is a novel
approach.

2

Preliminaries

Let N = {1, 2, 3, . . .} and [n] = {1, 2, . . . , n}, n ∈ N. For a set A, by P(A) we denote its power
set. For a string w, |w| denotes its length and, for every i ∈ [|w|], w[i] denotes the ith symbol
of w. Moreover, by w[i..j], we denote the factor of w from symbol i to symbol j, and for b ∈ Σ,
|w|b denotes the number of occurrences of b in w. The symbol ε denotes the empty word. For
an alphabet A, A+ denotes the set of non-empty words over A and A∗ = A+ ∪ {ε}; we set
Aε = A ∪ {ε} (i. e., we also use ε as a symbol denoting the empty word). For any language
descriptor D, L(D) denotes the language of D.

2.1

Regular Expressions with Backreferences

Let X denote a finite set of variables. The set RXΣ,X of regular expressions with backreferences
(over Σ and X), also denoted by regex, for short, is recursively defined as follows:
1. a ∈ RXΣ,X and var(a) = ∅, for every a ∈ Σε ,
2. (α · β) ∈ RXΣ,X , (α ∨ β) ∈ RXΣ,X , and (α)+ ∈ RXΣ,X , for every α, β ∈ RXΣ,X ;
furthermore, var((α · β)) = var((α ∨ β)) = var(α) ∪ var(β) and var((α)+ ) = var(α),
3. x ∈ RXΣ,X and var(x) = {x}, for every x ∈ X,
4. x{α} ∈ RXΣ,X and var(x{α}) = var(α) ∪ {x}, for every α ∈ RXΣ,X and x ∈ X \ var(α).
For α ∈ RXΣ,X , we set α∗ = α+ ∨ ε, and we usually omit the operator ‘·’. In a regex, we call
an occurrence of symbol x ∈ X a recall of variable x and a subexpression of the form x{α} a
definition of variable x; if we just talk about (occurrences of ) variables, then we refer to a recall
or a definition. The subset of RXΣ,X that can be created by Points 1 and 2 is exactly the set of
regular expressions over Σ, which we also call classical regular expressions.
The syntax tree T (α) of α ∈ RXΣ,X with X = [m] with nodes N (α) is defined as follows.
• If α ∈ Σε ∪ X, then T (α) is a single node labelled with [α].
• If α = (β ∨ γ) (or α = (β ·γ)), then the root of T (α) is labelled with [∨] (or [·], respectively)
and has the root of T (β) as its left and the root of T (γ) as its right child.
• If α = (β)+ (or α = (x{β})), then the root of T (α) is labelled with [+] (or [x{}], respectively) and has the root of T (β) as its only child.
See Fig. 1 for an illustration of a syntax tree. By se(t), we refer to the subexpression of α
that corresponds to a node t ∈ N (α), i. e., se(t) is the regex represented by the subtree of T (α)
rooted by t. By type(t), we refer to the label of node t ∈ N (α). In the following, we assume that
regex are always given as syntax trees T (α); in particular, we set |α| = | T (α)|.
On an intuitive level, the semantics of a regex are clear: the expression is matched to a
word as for classical regular expressions and if we encounter a definition x{β}, then the factor
v matched to β is stored in x and further occurrences of x are treated as v (as a particularity,
undefined variables are treated as ε). However, several particularities, that are easily overlooked on this intuitive level, complicate the behaviour of regex considerably. For example, in
(x{a∗ } ∨(x{b∗ }x))(x ∨ x), depending on the alternations, either the first or the second definition
of x is instantiated and either the second or third recall of x is instantiated, while the first recall
3

<!-- page 4 -->

·
·

∨
·

{x}
∨
·
a

c

+

∨

+

+

a

b

a

b

x

b

Figure 1: The syntax tree T (α) for α = (x{ab ∨ c} ∨(a+ b+ )) (a ∨ b) x+ .
is instantiated if and only if the second definition is. Moreover, the second and third recall of
x can refer to the first or the second definition of x, while the first recall can only refer to the
second definition. As a result, whether or not the second or third recall of x refers to the same
factor as the first recall depends on the alternations. The situation is even more complicated by
operator + as, e. g., in α = ((x{a∗ } ∨(y{b∗ }y))(x ∨ y))+ . Now there is a potentially unbounded
number of instances of each of the definitions and recalls of variables x and y, and the allocation
between definitions and recalls can reach over several iterations of the operator +. For example,
if the definition of x is instantiated in the first, and the definition of y in the second to fifth
iteration, then the recall of x, if instantiated in the fifth iteration, refers to the definition of x of
the first iteration. By using nesting of subexpressions in combination with operator +, rather
complicated regex can be constructed.
We refer to [38, 24] for a detailed definition of the semantics for regex; moreover, the following
automaton representations of regex, that are central for this work, will also implicitly give a
definition of the regex-languages.
For a class R ⊆ RXΣ,X , the matching problem (for R) is the problem to decide whether
w ∈ L(α) for given α ∈ R and w ∈ Σ∗ .

2.2

Memory Automata

An NFA is a tuple M = (Q, Σ, δ, q0 , F ) with a set Q of states, a finite alphabet Σ, a start state q0 ,
a set F of accepting states and a transition function δ : Q × (Σ ∪ {ε}) → P(Q). Configurations
of M (on input w) are pairs (q, u), where q ∈ Q and u is a (possibly empty) suffix of w; (q0 , w)
is the start configuration (of M on w) and a configuration (q, ε) is accepting if q ∈ F . The
transition relation ⊢M on the configurations is induced by δ in the natural way and a word w
is accepted (i. e., in the language L(M ) of M ) if (q0 , w) ⊢∗M (q, ε) with q ∈ F (where ⊢∗M is the
reflexive-transitive closure of ⊢M ).
In order to derive k-memory automata (MFA(k), for short) from NFA, we first define for every
k ∈ N an alphabet Γk = {o(x), c(x) | x ∈ [k]} and for any alphabet Σ, we set Σk = Σ ∪ [k] and
Σε,k = Σk ∪ {ε}. Syntactically, an MFA(k) is an NFA = (Q, ∆, δ, q0 , F ) with ∆ = Σε,k ∪ Γk ; the
semantics are as follows. Configurations of MFA(k) are tuples (q, w, (u1 , r1 ), . . . , (uk , rk )) with q
and w being the current state and remaining input, respectively, and (ui , ri ) is the configuration
of memory i, for every i ∈ [k], where ri ∈ {O, C} is the status and ui ∈ Σ∗ is the content of
memory i. The transition relation ⊢M is induced by δ as follows. We have c ⊢M c′ if one of the
following two cases apply:
1. c = (q, vw, (u1 , r1 ), . . . , (uk , rk )) and c′ = (p, w, (u′1 , r1 ), . . . , (u′k , rk )) with
• p ∈ δ(q, x) with either (x ∈ Σε and v = x) or (x ∈ [k], rx = C and v = ux ), and,
• for every ℓ ∈ [k], rℓ = O implies u′ℓ = uℓ v, and rℓ = C implies u′ℓ = uℓ .

2. c = (q, w, (u1 , r1 ), . . . , (uk , rk )) and c′ = (p, w, (u1 , r1 ), . . . , (u′ℓ , rℓ′ ), . . . , (uk , rk )) with p ∈
δ(q, x) with x = o(ℓ), rℓ′ = O and u′ℓ = ε, or with x = c(ℓ), rℓ′ = C and u′ℓ = uℓ .

4

<!-- page 5 -->

a

c1/o2/b

b

o1/a
c2/1
d

2

o2/a
a

c2/1

1

Figure 2: Example MFA(2).
Hence, intuitively speaking, we can consume either single symbols from the remaining input,
or the whole content ui of a memory i (although for this the memory must be closed, i. e.,
rx = C), while everything that we consume from the input is appended to the content of every
memory j that is open (i. e., rj = O). The special symbols o(i) and c(i) change the status of a
memory i, without consuming anything from the remaining input.
The initial configuration of M (on input w) is the configuration (q0 , w, (ε, C), . . . , (ε, C)), a
configuration (q, ε, (u1 , r1 ), . . . , (uk , rk )) is an accepting configuration if q ∈ F , and L(M ) is the
set of accepted inputs.
In the following, we shall denote MFA(k) by tuples (Q, Σ, δ, q0 , F ) (i. e., we only explicitly
state the “actual” terminal alphabet Σ). If the number k of memories is not important, we also
write MFA. It will often be convenient to allow a slight abuse of notation and address memories
with ‘names’ rather than integers. For every x ∈ Σε,k ∪ Γk , transitions p ∈ δ(q, x) are called
x-transitions, and we also write p ∈ δ(q, x) as (q, x) →δ p (and also drop the subscript δ if it
is clear from the context). An x-transition is a memory recall transition if x ∈ [k]. A sequence
c1 , c2 , . . . , cm of configurations with ci ⊢M ci+1 , 1 ≤ i ≤ m−1, is called computation (of M ), it is
called a computation (of M ) on input w, if c1 is the initial configuration on input w. For the sake
of convenience, we also write computations as tuples ⃗c = (c1 , c2 , . . . , cm ). For a configuration
(q, w, (u1 , r1 ), . . . , (uk , rk )), (u1 , . . . , uk ) are the memory contents and (r1 , . . . , rk ) the memory
statuses.
As usually done for NFA, we also interpret MFA as directed graphs with vertices Q and
transitions as edge-labels; the start state is marked by an incoming arrow and accepting states
are double-circled. We also label edges with several elements x1 , x2 , . . . , xn ∈ Σε,k ∪Γk , separated
by ‘/’, in order to denote a sequence of transitions in a compact way.
Example 1. Consider the M ∈ MFA(2) illustrated in Fig. 2 (note that o(x) and c(x) are compressed to ox and cx, respectively). M can either record a word from a+ in memory 1 and then
a word b+ in memory 2 (this corresponds to the ‘upper branch’), or a word from a+ in memory
2 (this corresponds to the ‘lower branch’). Then, memory 1 is recalled followed by reading d,
and these two steps can be repeated arbitrarily often. Finally, M enters an accepting state by
recalling memory 2. Note that in the lower branch, memory 1 is necessarily empty. As can
be easily verified, L(M ) = {an bm (an d)k bm | n, m, k ≥ 1} ∪ {an dm an | n, m ≥ 1}; moreover,
L(M ) = L(((x{a+ }y{b+ }) ∨ y{a+ })(xd)+ y). See also Example 23 in the Appendix.

Given an MFA(k) and a word w, we can check whether w ∈ L(M ) as follows. In the graph
that has the configurations (q, u, (r1 , u1 ), . . . , (rk , uk )) of M on w as vertices and edges given
by the relation ⊢M , we simply search for a path from the initial configuration to an accepting
configuration. This directly yields the following trivial upper bound for the matching problem
for memory automata.

Lemma 2. Given w ∈ Σ∗ and M = (Q, Σ, δ, q0 , F ) ∈ MFA(k) with |δ| = O(|Q|), we can decide
w ∈ L(M ) in time |Q||w|O(k) .

2.3

Memory Automata for Regex

For α ∈ RXΣ,X with X = [m], we transform T (α) into a directed, edge-labelled graph H(α).
Every t ∈ N (α) is replaced by nodes tin , tout if type(t) ∈ {[∨], [+], [b], [x], [x{}] | b ∈ Σε , x ∈ X},
and by nodes tin , tm , tout if type(t) = [·]. For every leaf t ∈ N (α), we add an edge (tin , tout ), and
for every non-leaf t ∈ N (α), we do the following.
5

<!-- page 6 -->

• If type(t) = [·] and r and s are the left and right children of t, respectively, then we add
edges (tin , rin ), (rout , tm ), (tm , sin ) and (sout , tout ).
• If type(t) = [∨] and r and s are the left and right children of t, respectively, then we add
edges (tin , rin ), (tin , sin ), (rout , tout ) and (sout , tout ).
• If type(t) = [+] and r is t’s child, then we add edges (tin , rin ), (rout , tout ) and (tout , tin ).
• If type(t) = [x{}] and r is t’s child, then we add edges (tin , rin ) and (rout , tout ).
Every edge (tin , tout ) with se(t) ∈ Σε ∪ X is labelled by se(t), every edge (tin , p) with type(t) =
[x{}], for some x ∈ X, is labelled with o(x), and every edge (tout , p) with type(t) = [x{}], for
some x ∈ X, is labelled with c(x). Moreover, all other edges are labelled with ε. We denote the
set of nodes of H(α) by NH (α).
The graph H(α) is a directed graph with edge labels from Σε,m ∪ Γm and every vertex
corresponds to a node of the syntax tree T (α) (see Fig. 3 for an illustration). Consequently,
H(α) can be interpreted both as an NFA over alphabet Σε,m ∪Γm or as an MFA(m) over alphabet
Σ. By R(α) we denote the NFA obtained from H(α) by defining tin to be the initial state, tout
to be the only accepting state, where t is the root of T (α), and the transition function to
be represented by the edge-labels. Analogously, M(α) is obtained by interpreting R(α) as an
MFA(m).
In principle, the transformation of α into H(α) is the Thompson-construction that obtains
an NFA from a regular expression. However, for our purpose it is convenient to keep this implicit
correspondence between states and nodes of the syntax tree.
By consulting the formal definition of the syntax of regex in [38, 24], and by considering that
H(α) is a variant of the Thompson construction, the following is immediate.
Proposition 3. For every α ∈ RXΣ,X , H(α) can be computed in time O(| H(α)|) = O(|α|).
Moreover, L(M(α)) = L(α).
With Lemma 2, this means that the matching problem for RXΣ,X can be solved in time
|α||w|O(|X|) .

3

Regex with Bounded Active Variable Degree

In this section, we define efficiently matchable classes of regex by formalising the following
observations. For regex of the form
+
+
(x1 {a+
1 } ∨ x2 {a2 } ∨ . . . ∨ xn {an })x1 x2 . . . xn or

+
+
(x1 {a+
1 } x1 ) (x2 {a2 } x2 ) . . . (xn {an } xn ) ,

it is intuitively clear that they can be matched by a procedure that only has to store the value
of one variable at a time, or, more formally, it is straightforward to construct an MFA(1). This
is due to the fact that the n different variables are independent. On the other hand,
+
+
x1 {a+
1 } x2 {a2 } . . . xn {an } x1 x2 . . . xn

does not seem to have this nice property. A more complicated instance of this behaviour can
be seen in Fig. 3: whenever variable y or u is defined, then the already defined variable x will
necessarily be redefined before it is recalled again. Moreover, as soon as x is defined again,
variables y and u are never recalled again. Consequently, when we reach a definition of y or u,
we can afford to forget x’s value, or, in the MFA-perspective, it should be possible to use the
memory for x now for y or u, and then later again for x, handling variables x, y and u with only
one memory instead of three. For the example of Fig. 3, we can actually just rename y and u
to x and obtain an equivalent regex with only two variables. Unfortunately, the situation that
variables can be reused, is not easily covered by a simple renaming of variables. For example,
consider α = x{a+ }bx(ycy{b+ })+ x{b+ }ax and β = x{a+ }bx((y{b+ }) ∨(z{b+ }cz))yax{b+ }ax.
For both these regex, it is again not necessary to store x’s value once we encounter another
variable. However, renaming y (and z in the case of β) to x produces non-equivalent regex,
6

<!-- page 7 -->

·
·

∨
·

+

a

·

{z}

{x}

·

+

{x}

b

+

b

·

x

+

·
c

·

∨
·

x

y

{y}
·

a

·

+

a

b

·

{u}
+

+

·

c

a

z

u

·
{x}

b

x

+

a

Figure 3: H(α) for α = ((z{a+ b}x{b+ }) ∨(x{a+ }cx)+ )x((y{a+ b+ }y) ∨(u{c+ }au))zx{a+ }bx.
Every t ∈ T (α) is represented by a grey square labelled with type(t), which contains the nodes
tin and tout (and tm if type(t) = [·]). In this way, the picture also implicitly shows T (α) (which
also implicitly determines the omitted edge labels of H(α)).
since it can happen that a former undefined occurrence of y becomes a defined occurrence of x,
e. g., α (with y renamed to x) cannot generate abacbbab, while β (with y renamed to x) cannot
generate ababcbabab.
We now derive a complexity measure from these observations. For our definitions and the
algorithm, we shall mainly rely on the automata-perspective, i. e., we work with R(α) and M(α).
First, we define relations ▷def ⊆ X × NH (α) and ▷call ⊆ NH (α) × X as follows:
x ▷def q ⇔ R(α) can reach q by reading a word w with |w|o(x) ≥ 1.
q ▷call x ⇔ starting in q, R(α) can read a word wx with |w|o(x) = 0.
Intuitively speaking, x ▷def q describes the situation that we can reach q such that a definition
for x is reached along the way, which means that some memory is currently storing a value for
x. Moreover, q ▷call x means that from q we can reach a point where the currently stored value
for x is recalled, i. e., we can reach an x-transition without first resetting the memory for x by
an o(x)-transition. Consequently, x ▷def q ▷call x means that we can reach q such that a memory
is currently used for storing some value for x and we cannot afford to lose this value.
For every q ∈ NH (α), the set of active variables (for q) is
avs(q) = {x | x ▷def q ▷call x} .
By avd(α) = max{| avs(tin )| | t has parent t′ with type(t′ ) = [x{}], x ∈ X}, we denote the active
variable degree of α. Finally, for every k ∈ N, we define regex with active variable degree at most
k as RXavd≤k
Σ,X = {α ∈ RXΣ,X | avd(α) ≤ k}.
Coming back to our example of Fig. 3, we observe that z ▷def tin ▷call z, where t is the child
node of t′ with se(t′ ) = [y{}]. Moreover, x ▷def tin ̸▷call x and u ̸▷def tin ̸▷call u. Thus, avs(tin ) =
{y, z}. In fact, we have | avs(sin )| ≤ 2 for every child node s of some node s′ with type(s′ ) ∈
{[x{}], [y{}], [z{}], [u{}]}, and therefore avd(α) = 2.
Lemma 4. For k ∈ N and α ∈ RXX,Σ , we can check avd(α) ≤ k in time O(|X||α|2 ).
Proof. For given x ∈ X and q ∈ NH (α), we can check whether x ▷def q in time O(|α|). Indeed,
this can be done by simply checking whether in R(α) we can reach q with a word that contains
7

<!-- page 8 -->

o(x). Analogously, we can check whether x ▷def q in time O(|α|), which means that the relations
▷def and ▷call can be computed in time O(|X||α|2 ).
Let T = {tin ∈ NH (α) | type(t) = [x{}], x ∈ X}. In order to check avd(α) ≤ k, we compute
avs(t) for every t ∈ T . Since we have computed relations ▷def and ▷call , this can be done in time
O(| NH (α)||X|) = O(|α||X|). Consequently, avd(α) = max{| avs(t)| | t ∈ T } can be computed in
total time O(|X||α|2 ).
Let t ∈ N (α) with type(t) = [x{}]. Then every word w that can be read by R(α) starting in
tin is a word that begins with o(x). Consequently, we conclude the following:
Observation 5. For every t ∈ N (α) with type(t) = [x{}], we have tin ̸▷call x.

We are now ready to prove that regex with bounded active variable degree can be matched
in polynomial-time.
Theorem 6. For k ∈ N, the RXavd≤k
Σ,X -matching problem can be solved in polynomial-time.
Proof. Let α ∈ RXavd≤k
and let w ∈ Σ∗ . We first compute the relations ▷def and ▷call as shown
Σ,X
in the proof of Lemma 4, and we construct H(α).
In the following, we transform M(α) into an equivalent Mα′ ∈ MFA(k). Intuitively speaking,
we replace every state q by states (q, M) for every M ∈ (X ∪{⊥})k . The idea of the memory lists
M contained in the states is that they store the information which of the m old memories of M(α)
are currently handled by which of the k memories of Mα′ ; more precisely, Mq [ℓ] = x if memory
ℓ of Mα′ currently plays the role of the old memory x of M(α), and Mq [ℓ] = ⊥ means that
memory ℓ is currently “not in use”. More precisely, we define Mα′ = (Q′ , Σ, δ ′ , q0′ , F ′ ) ∈ MFA(k)
with Q′ = {(q, M) | q ∈ NH (α), M ∈ (X ∪ {⊥})k }, q0′ = (q0 , (⊥, . . . , ⊥)) and F ′ = {(pf , M) |
M ∈ (X ∪ {⊥})k }, where q0′ and pf is the initial and accepting state of M(α). We note that
|Q′ | = O(|α||X|k ).
Next, we give a high-level description of the transitions of Mα′ . The general idea is that Mα′
simulates the computation of M(α). Whenever M(α) uses some memory x ∈ X, Mα′ chooses a
memory ℓ with M[ℓ] = ⊥ and uses it in order to simulate memory x. This works fine as long as
we do not run out of memories (i. e., M(α) opens a memory, but M[ℓ] ̸= ⊥ for all ℓ ∈ [k]). We
shall first define transitions such that Mα′ can simulate M(α) under the assumption that this
problem does not occur. Later on, we will show how the transitions must be transformed and
extended for the general case.
• Any b-transition of M(α) with b ∈ Σε is just simulated without changing the memory list.
• If M(α) opens memory x, then Mα′ opens some memory ℓ with M[ℓ] = ⊥ and sets M[ℓ] = x.
• If M(α) closes memory x or recalls memory x, then Mα′ does the same with respect to
memory ℓ with M[ℓ] = x and does not change the memory list.
With these transitions, Mα′ can simulate M(α) up to the situation where it performs an o(x)transition, but M[ℓ] ̸= ⊥, for every ℓ ∈ [k], or a c(x)- or x-transition, but M[ℓ] ̸= x, for every
ℓ ∈ [k].
We next modify the transitions defined so far in the following way. For every x ∈ X, whenever
Mα′ moves from a state (q, Mq ) to a state (p, Mp ) such that p ̸▷call x, then all occurrences of x
in Mp are replaced by ⊥.
We claim that it is not possible now for Mα′ to reach the situation that M[ℓ] = M[ℓ′ ] = x for
1 ≤ ℓ < ℓ′ ≤ k and x ∈ X. Initially, M only stores ⊥. The only way that x is added to M is
that an o(x)-transition is simulated. Since o(x)-transitions are only triggered by states tin with
type(t) = [x{}] and tin ̸▷call x (see Observation 5), a possible x-entry of M will be replaced by ⊥
before the next o(x)-transition is to be simulated.
We now assume that Mα′ reaches the situation that it tries to simulate an o(x)-transition of
M(α), but M[ℓ] ̸= ⊥, for every ℓ ∈ [k]. Let tin with type(t) = [x{}] be the state that triggers
this o(x)-transition, let q be the state this transition leads to, and let y1 , y2 , . . . , yk ∈ X be the
elements stored in M. We observe the following facts:

8

<!-- page 9 -->

• |{y1 , y2 , . . . , yk }| = k and x ∈
/ {y1 , y2 , . . . , yk }: As shown above, M[ℓ] = M[ℓ′ ] ̸= ⊥ for
′
1 ≤ ℓ < ℓ ≤ k is not possible, so M stores k distinct values. Furthermore, since type(t) =
[x{}], we also have tin ̸▷call x (see Observation 5), which implies that a previous x-entry of
M would have been removed. Thus, x ∈
/ {y1 , y2 , . . . , yk }.
• {y1 , y2 , . . . , yk } ⊆ avs(q): For every i ∈ [k], we have yi ▷def tin , since otherwise it is not
possible for yi to be stored in M, and we also have tin ▷call yi , since otherwise yi cannot be
in M. Moreover, avs(tin ) ⊆ avs(q) holds due to the fact that there is just one transition
from tin labelled with o(x).
• q ̸▷call x: Since x ▷def q, q ▷call x would imply x ∈ avs(q) and therefore k + 1 ≤ | avs(q)| ≤
avd(α) = k, which is a contradiction.
By definition, q ̸▷call x means that there is no word wx with |w|o(x) = 0 that can be read by
R(α) starting in q. This means that all possible further x-transitions are preceded by an o(x)transition. Consequently, if Mα′ reaches the situation that it tries to simulate an o(x)-transition
of M(α), but M[ℓ] ̸= ⊥, for every ℓ ∈ [k], then we can simply ignore this o(x)-transition of
M(α), i. e., we carry out an ε-transition instead. Moreover, we will then necessarily also reach
the situation that Mα′ tries to simulate an c(x)-transition of M(α) (namely the one triggered
by state tout ), but M[ℓ] ̸= x, for every ℓ ∈ [k]. We can also ignore this c(x)-transition and just
carry out an ε-transition instead.
We only have to discuss the situation that Mα′ tries to simulate an x-transition of M(α), but
M[ℓ] ̸= x, for every ℓ ∈ [k]. Let us first assume that this happens when no o(x)-transition has
been simulated before. Then memory x is empty, which means we can ignore the x-transition
and just carry out an ε-transition instead. Let us now assume that there has been an earlier
o(x)-transition triggered by some state q, and let us consider the o(x)-transition that is the most
recent one with respect to the x-transition to be simulated. There are two possibilities why this
o(x)-transition does not cause x to be stored in M. The first one is that for the source state p
of this o(x)-transition, we have p ̸▷call x. The second one is that in q the memory list M does
not contain any occurrence of ⊥, which, as explained above, also means that p ̸▷call x. However,
p ̸▷call x means again that there is no word wx with |w|o(x) = 0 that can be read by R(α) starting
in q, which contradicts our assumption that the considered o(x)-transition is the most recent
one.
These considerations show that Mα′ can simulate M(α) and therefore L(Mα ) = L(Mα′ ).
We can now check whether w ∈ L(α) by checking w ∈ L(Mα′ ) in time |Q′ ||w|O(k) =
|α||X|k |w|O(k) (see Lemma 2).
The parameter avd has an obvious shortcoming: if for some Y ⊆ X, we have x ▷def q for
every x ∈ Y , then this only means that for every x ∈ Y we can reach q with x defined,
but not that it is possible to reach q with all x ∈ Y defined at the same time. For example,
α = ((x{a+ }y{b+ }) ∨ z{c+ } ∨(x{b+ }u{c+ }))v{a+ }xyzuv has a maximum active variable degree
of |X| = 5, while the maximum number of variables defined at the same time is only 3 and we
can easily define an MFA(3) for α. Consequently, it seems that the active variable degree can be
strengthened by extending the relation ▷def to a relation of the form P(X) × NH (α) as follows:
{y1 , y2 , . . . , yℓ } ▷def q if and only if R(α) can reach q by reading a word w with |w|o(yi ) ≥ 1, for
every i ∈ [ℓ]. Then, we can define a strong active variable degree by savd(α) = max{| avs(tin )∩Y | |
Y ⊆ X, Y ▷def tin , t has parent t′ with type(t′ ) = [x{}]}.
Theorem 7. Deciding whether savd(α) ≤ k for given α ∈ RXΣ,X and k ∈ N is coNP-hard.
Proof. We devise a reduction from the set cover problem:
Input:
finite set U, {B1 , B2 , . . . ,S
Bn } ⊆ P(U), and k ∈ N.
Question: ∃j1 , j2 , . . . , jk ∈ [n] with i∈[k] Bji = U?

Let U = {x1 , x2 , . . . , xm }, Bi = {yi,1 , yi,2 , . . . , yi,ℓi } ⊆ U and k ∈ N be an instance of the set
cover problem. We transform this instance into an α ∈ RXΣ,X with Σ = {b} and X = U ∪ {z}

9

<!-- page 10 -->

as follows:
βi = yi,1 {ε} yi,2 {ε} . . . yi,ℓi {ε} ,
γ = (β1 ∨ β2 ∨ . . . ∨ βn )k ,

for every i ∈ [n],

α = z{ε} γ b x1 x2 . . . xm z .
S
We shall show that savd(α) > n if and only if there are j1 , j2 , . . . , jk ∈ [n] with i∈[k] Bji = U.
Let t ∈ NH (α) be the node that corresponds to the last occurrence of ε in γ. We observe
that savd(α) = max{| avs(tin ) ∩ Y | | Y ⊆ X, Y ▷def tin } and, since avs(tin ) = X obviously holds,
savd(α) > n is equivalent to X ▷def tin .
For every i ∈ [n], let ui = o(yi,1 )c(yi,1 ) o(yi,2 )c(yi,2 ) . . . o(yi,ℓi )c(yi,ℓi ). We note that L(R(α)) =
{o(z)c(z)u
S j1 uj2 . . . ujk bx1 x2 . . . xm z | ji ∈ [n], i ∈ [k]}. Consequently, there are j1 , j2 , . . . , jk ∈
[n] with i∈[k] Bji = U if and only if there is some w = ubv ∈ L(R(α)) with |u|o(y) ≥ 1, for every
y ∈ X. The second statement is, by definition of the relation ▷def , equivalent to X ▷def tin .
In addition to the hardness of computing the strong active variable degree, it is also not
entirely clear, how it could be used in the sense of Thm. 6.
By transforming the MFA(k) from Thm. 6 into a regex (see [38] for details), we obtain the
following corollary, which is worth mentioning, since deciding whether for a given k-variable
regex there is an equivalent (k − 1)-variable regex is undecidable (see [20]).
′
Corollary 8. Every α ∈ RXavd≤k
Σ,X can be effectively transformed into a β ∈ RXΣ,X ′ with |X | = k
and L(α) = L(β).

4

Memory-Deterministic Regex

Considering NFA as a matching tool for classical regular expressions, their non-determinism could
be considered harmless: the computation may branch in every step, but all parallel branches
will differ only in their current states. Consequently, we can handle all possible parallel branches
of an NFA-computation by maintaining a set of active states, which only causes a factor of |Q|
compared to the linear running time of a DFA (this can also be considered as determinising an
NFA “on-the-fly”).4 Considering the fact that transforming regular expressions to DFA may cause
exponential size blow-ups, while NFA of asymptotically the same size can be easily obtained, this
additional factor of |Q| is often acceptable.
For more complicated automata, e. g., with additional storage, it is often the case that
the deterministic variant can be handled easily (but is of weak expressive power), while nondeterminism causes undecidability or intractability. A typical way to approach this problem
is to restrict the nondeterminism, hoping to find a more appealing balance between expressive
power and complexity. Purely deterministic MFA have been used in [24] to define deterministic
regex, which can be matched efficiently (in time O(|Σ||α|2 + k|w|), where k is the number of
variables), but, on the other hand, seem to be unnecessarily restricted if efficient matchability is
our main concern: deterministic regex do not cover classical regular expressions, and the class
of deterministic regex languages does not contain the class of regular languages (note that the
latter statement is stronger than the former).
Our goal is to find a class of regex that properly extends classical regular expressions and for
which the nondeterminism is only as powerful (and therefore as harmless) as for classical regular
expressions (or NFA). Since the variables of regex (or the memories of MFA) are responsible
for intractability, the main idea is to impose determinism on memories, but allow the harmless
kind of nondeterminism observed in classical NFA. Formalising this somewhat vague objective
is not an easy task. We shall next substantiate this claim by demonstrating that even very
mild forms of nondeterminism are sufficient to make the acceptance problem of MFA intractable.
In particular, this result suggests that our goal cannot be achieved by local restrictions on a
syntactic level.
4 Technically, we only get a factor |Q| if |δ(q, a)| is constant for all q and a, but this is the case for NFA obtained
from regular expressions (see Thompson [43] and Section 2.3).

10

<!-- page 11 -->

pi−1

a
a

oxi /a
qi

cxi /cxi /b

ti

ri−1

cyi,1 /yi,1

cyi,2 /yi,2

cyi,3 /yi,3

b s
i

oxi /a

(a)

(b)

Figure 4: Illustrations for the proof of Thm. 9.
Let M = (Q, Σ, δ, q0 , F ) ∈ MFA(k). A state q ∈ Q is called deterministic if, for every
x ∈ Σε,k ∪ Γk , there is at most one x-transition for q, and it is called x-restricted for an
x ∈ Σε,k ∪ Γk , if the existence of an x-transition for q implies that q has no y-transitions for any
y ∈ (Σε,k ∪ Γk ) \ {x}. The MFA M is deterministic if all states are deterministic and, for every
x ∈ [k] ∪ Γk ∪{ε}, all states are also x-restricted.5
Theorem 9. The acceptance problem for MFA(k) is NP-complete, even if the input MFA M =
(Q, Σ, δ, q0 , F ) have the following restrictions: (1) Σ = {a, b}, (2) M has no ε-transitions, (3)
for every q ∈ Q and x ∈ [k] ∪ Γk ∪{ε}, q is x-restricted, (4) every q ∈ Q is either deterministic
or satisfies |δ(q, a)| = 2.
Proof. We conduct a reduction from 1-in-3 3SAT without negated variables. To Sthis end,
m
let C = (c1 , c2 , . . . , cm ) be a set of clauses ci = {yi,1 , yi,2 , yi,3 }, 1 ≤ i ≤ m, with i=1 ci =
{x1 , x2 , . . . , xn }. We define an MFA(2n), which, for every i ∈ [n], has a memory xi and a
memory xi . For every i ∈ [n], we construct the component shown in Fig. 4(a) and, for every
i ∈ [m], we construct the component shown in Fig. 4(b) (note that o(x) and c(x) are compressed
to ox and cx, respectively). In order to obtain M , we combine these components by joining
some of their states (joining two states means that they will be the same state in M ). More
precisely, we join every pi with ti , 0 ≤ i ≤ n − 1, we join tn with r0 , and we join every ri with
si , 1 ≤ i ≤ m − 1. Finally, we let p0 be the start state and sm the only accepting state.
Next, we show that C is 1-in-3 satisfiable if and only if (aab)n (ab)m is accepted by M . In each
computation, the MFA M will initially read the word (aab)n (which happens in the components
shown in Fig. 4(a)), and the (2i)th occurrence of a will be stored in either memory xi or xi .
Then, in the components shown in Fig. 4(b), M will read m occurrences of b, where the j th
occurrence of b is directly preceded by the contents of the memories corresponding to clause cj .
The word consumed in this second part is (ab)m if and only if every clause contains exactly one
memory that stores a.
Consequently, transforming a CNF-formula into an MFA as described above is a polynomial
reduction from 1-in-3 3SAT without negated variables to the acceptance problem for MFA.
Moreover, the MFA obtained by this reduction satisfies the structural restrictions of the statement
of the theorem (see Figures. 4(a) and 4(b)).
The MFA of Thm. 9 are quite restricted: the alphabet is binary and there are no ε-transitions;
moreover, each state has either just one outgoing transition or it has exactly 2 a-transitions and
not other transitions. Hence, they can be considered as being almost deterministic. In particular,
the nondeterminism of the MFA of Thm. 9 are especially restricted with respect to memories,
since none of the non-deterministic branching points manipulate a memory. The actual problem seems to be that the undesired memory-nondeterminism does not present itself as a local
nondeterministic choice, i. e., as two different transitions applicable in the same state that manipulate memories in different ways, but instead can arise much later in different computational
branches that were created by a nondeterministic choice that seemingly does not cause memorynondeterminism. This also suggests that a suitable restriction of the memory-nondeterminism
can probably not be formulated as a local property for each separate state. Next, we define a
property of MFA that covers our intuition of memory-determinism, but is rather complex in the
sense that it depends on all possible computations of the MFA, rather than on local properties
of the transition function.
5 This definition slightly differs from the one in [24], but yields the same model.

11

<!-- page 12 -->

4.1

Synchronised Memory Automata

In the following, we shall show that the acceptance problem for so-called synchronised MFA can
be solved efficiently. In order to do this, we first need some algorithmic preliminaries.
Our computational model is the standard unit-cost RAM with logarithmic word size. We
assume that NFA = (Q, Σ, δ, q0 , F ) (note that this includes MFA, since they are, syntactically,
NFA) are given as directed graphs with vertices Q (with special markers for the initial and
accepting states) and δ is represented as edge-labels from Σ. Moreover, we assume that the
out-degree is constant and for every vertex there is at most one symbol such that outgoing edges
are labelled with this symbol. Hence, we can assume that the directed, edge-labelled graphs
are represented by storing for each vertex the symbol for which outgoing edges exist, and also
storing a set of the (constantly many) reachable vertices. From these assumptions, we directly
conclude the following:
• The number of edges is O(|Q|), which also implies that |Σ| = O(|Q|) and therefore |M | =
O(|Q|).
• Given a vertex q and x ∈ Σ, we can retrieve all p with p ∈ δ(q, x) in constant time.
• A breadth-first search can be performed in time O(|Q|).
Remark 10. The assumptions made above generally constitute a restriction to NFA. They are
nevertheless justified in our case, since they are all satisfied for the NFA and MFA obtained from
regex, i. e., R(α) and M(α) (see Section 2).
For a sequence of memory instructions and ε-symbols C = (c1 , c2 , . . . , cn ) ∈ (Γk ∪{ε})n , we
define Φ(C) as follows. For every i ∈ [n], ci ∈ Φ(C) if and only if ci ̸= ε and ci ∈ {o(x), c(x)}
implies that cj ∈
/ {o(x), c(x)}, for every j with i < j ≤ n. Intuitively speaking, for every x ∈ [k],
we put only the very last occurrence (with respect to the sequence C) of any o(x) or c(x) into
Φ(C) and ignore all the others. Obviously, applying transitions labelled with c1 , c2 , . . . , cn in
this order has the same effect as applying the memory instructions of Φ(c1 , c2 , . . . , cn ) in any
order. A set C ⊆ Γk is reduced if, for every x ∈ [k], |{o(x), c(x)} ∩ C| ≤ 1. We note that for
every C ′ ∈ (Γk ∪{ε})n , Φ(C ′ ) is reduced.
For every q, p ∈ Q and reduced C ⊆ Γk , we write (q, C, ε) →con p if there is a path from q to p
of transitions labelled with c1 , c2 , . . . , cn ∈ Γk ∪{ε} such that Φ(c1 , c2 , . . . , cn ) = C. Furthermore,
for every x ∈ Σk , we write (q, C, x) →con p if (q, C, ε) →con p′ with p ∈ δ(p′ , x).
We call (q, C, x) →con p a contracted transition 6 and we set δcon (q, x) = {p | ∃ reduced C ⊆
Γk : (q, C, x) →con p}. In the following, let σ = |Σ| + k. A transition (q, x) → p is called
consuming if and only if x ∈ Σk .
Lemma 11. All sets δcon (q, x), q ∈ Q, x ∈ Σε,k , can be computed in time O(|Q|2 σ).
Proof. For every q ∈ Q, we proceed as follows. First, we start a breadth-first search at q that
only considers non-consuming transitions and we build the corresponding tree (i. e., the breadthfirst search tree), which requires time O(|Q|) (see the explanations from above). The states of
this tree is exactly the set δcon (q, ε). Next, for every x ∈ Σk , we initialise a set Ax = δ(q, x), and
then we traverse the tree top-down (i. e., we repeat the breadth-first search) and every time we
visit a state p, we add the set δ(p, x) to Ax for every x ∈ Σk (note that, as mentioned above,
|δ(p, x)| is constant). After termination of this traversal, the sets Ax are the sets δcon (q, x).
These computations require time O(|Q|σ) for a fixed q, and therefore we need time O(|Q|2 σ) in
order to compute all sets δcon (q, x).
The contraction of a computation ⃗c of some M ∈ MFA on some input w is obtained by
replacing every maximal sequence (⃗c[i], ⃗c[i + 1], . . . , ⃗c[j]) whose corresponding computational
steps are due to non-consuming transitions by (⃗c[i], ⃗c[j + 1]), or by (⃗c[j]), if j = |⃗c|. Obviously, if
⃗c is a contracted computation of some MFA M , then, for every i ∈ [|⃗c| − 1], M has a contracted
transition that can change ⃗c[i] to ⃗c[i + 1].
6 It is discussed in the Appendix (Section B), why we cannot afford to actually compute all those contracted
transitions.

12

<!-- page 13 -->

Two contracted computations ⃗c and c⃗′ for some M ∈ MFA are said to be synchronised if and
only if ⃗c[1] = c⃗′ [1] and, for every i ≤ min{|⃗c|, |c⃗′ |}, either ⃗c[i] = c⃗′ [i] or ⃗c[i] and c⃗′ [i] only differ
with respect to their states (i. e., their memory contents are the same, their memory statuses
are the same, and their remaining inputs are the same). An M ∈ MFA is called synchronised if
any two contracted computations of M on the same input are synchronised.
We next define some data-structures that are used by our algorithm for the acceptance
problem of synchronised MFA. Let M = (Q, Σ, δ, q0 , F ) be a fixed MFA(k). Let TM : Q ×
Σk → P(Γk ) ∪ {⊥} be a table such that, for every q ∈ Q and x ∈ Σk , TM [q, x] stores some
reduced C ⊆ Γk for which there is a state p and a contracted x-transition (q, C, x) →con p (or
TM [q, x] = ⊥, if no such C exists); note that for a fixed M , there are several valid possibilities
for table TM . When M is clear, we drop the superscript M .
Lemma 12. Table T can be computed in time O(|Q|2 σ).
Proof. We first initialise T by T[q, x] = ⊥ for every q ∈ Q and x ∈ Σk , which can be done in
time O(|Q|2 σ). Then, for every q ∈ Q, we proceed as follows. First, we start a breadth-first
search in q that only considers non-consuming transitions and we build the corresponding tree
(i. e., the breadth-first search tree), which requires time O(|Q|). Next, we compute a set Cp ⊆ Γk
for every state p in this tree, such that δcon (q, Cp , ε) →con p. This can be done as follows. We
first initialise sets Cp = ∅ for every p ∈ Q. Then we traverse the tree top-down and every time
we reach a new state p′ from a parent state p via some x-transition (note that x ∈ (Γk ∪{ε})),
we set Cp′ = Φ(Cp ∪ {x}) if x ̸= ε and we set Cp′ = Cp otherwise. This requires time O(|Q|)
(recall the explanations from the beginning of this subsection and also note that the computation
of Φ(Cp ∪ {x}) only requires to add one element or to replace an element by a new one (this
can be done by, e. g., a bit-vector implementation of the sets)). Finally, we traverse the tree
another time top-down and for every visited state p and every x ∈ Σk with δ(p, x) ̸= ∅, we set
T[q, x] = Cp . This step requires time O(|Q|σ).
It can be easily verified that this procedure computes T correctly. Moreover, the total time
required for these computations is O(|Q|2 σ).
Let R : [k] → {O, C} be a table and let C ⊆ Γk be reduced. We define


if o(i) ∈ C ,
O,
(R ∪ C)[i] = C,
if c(i) ∈ C ,


R[i], else .
The intuition here is that R describes the memory statuses of some configuration of an MFA
and C describes some memory instructions (in the form of a reduced set of memory instructions as
defined above). Then, R∪C simply describes the memory statuses after applying the instructions
from C.
For a word w ∈ Σ∗ , we store factors of w by their start and end positions, which means that
concatenating two adjacent factors or storing a factor in a program variable only requires constant
time (note that since we assume a RAM with logarithmic word size, storing and manipulating
positions of w can be done in constant time). The longest common extension data-structure
LCEw is defined such that, for i, j, 1 ≤ i < j ≤ |w|, LCEw (i, j) is the length of the longest
common prefix of w[i..|w|] and w[j..|w|] (which can be retrieved in constant time). In particular,
note that LCEw can be used in order to check in constant time whether a factor w[i..j] is a prefix
of a factor w[i′ ..j ′ ], denoted by w[i..j] ⪯pref w[i′ ..j ′ ]. In the following, we assume that we have
LCEw at our disposal, which can be constructed in linear time (see, e. g., [18]).
Next, we define an algorithm (see Fig. 5) that, for a synchronised M ∈ MFA and a word w,
decides whether or not w ∈ L(M ). Note that we use ⃗ε and ⃗C as short hand for length-k vectors
each component of which is ε and C, respectively.
Theorem 13. Let M ∈ MFA be synchronised and let w ∈ Σ∗ . On input (M, w), algorithm
SyncMembership decides whether or not w ∈ L(M ), and it can be implemented such that it
has running-time O(|w||Q|3 σ).

13

<!-- page 14 -->

ALGORITHM 1: SyncMembership
Input : Synchronised M ∈ MFA(k), w ∈ Σ∗ .
Output: Yes if and only if w ∈ L(M ).
1 For all q ∈ Q, x ∈ Σε,k , compute sets δcon (q, x), compute table T;
2 A := {q0 }, U := (u1 , . . . , uk ) := ⃗
ε, R := ⃗C, v := w, b := 0;
3 while b ≤ |Q| do
4
Λ := {x ∈ [k] | U [x] ⪯pref v} ∪ {v[1]};
5
if ∃q ∈ A, ∃x ∈ Λ : δcon (q, x) ̸= ∅ then
6
if x ∈ Σ then u := x else u := U [x];
7
if u = ε then b := b + 1 else b := 0;
8
R := R ∪ T[q, x];
9
for ℓ ∈ [k] with R[ℓ] = O do
10
U [ℓ] := U [ℓ]u;
11
v := v[|u|
S +S1..|v|];
12
A := q∈A x∈Λ δcon (q, x);
S
13
if (v = ε) ∧ ((A ∪ q∈A δcon (q, ε)) ∩ F ̸= ∅) then
14
return Yes;
15
else
16
return No;
17 return No;
Figure 5: An algorithm for solving the acceptance problem for synchronised MFA.
Proof. All the following references to certain line numbers of the algorithm refer to Algorithm 1
stated in Fig. 5.
We prove the correctness and the required running-time of the algorithm separately:
Correctness: Let m be the number of successful iterations of the main loop of the algorithm,
i. e., iterations where the condition of Line 5 is satisfied. By Λi , Ai , Ui , Ri and vi , we denote
the values of the variables Λ, A, U , R and v at the beginning of the ith iteration, and by qi and
xi , we denote the state from Ai and element from Λi , respectively, for which the condition of
Line 5 is satisfied at the ith iteration.
We note that at the ith iteration, the elements Ai , Ui , Ri and vi encode a set of configurations
Ci = {(q, vi , (Ui [1], Ri [1]), . . . , (Ui [k], Ri [k])) | q ∈ Ai }
and, initially, C1 only contains the start configuration of M on w.
Claim 1: For every i ∈ [m], Ci contains all configurations that can be reached from the initial
configuration of M on input w by (i − 1) contracted transitions.
Proof of Claim 1: The statement is obviously true for i = 1. Now let i be arbitrary with
2 ≤ i ≤ m and assume that the statement holds for i − 1. Let c, c′ be arbitrary configurations
from Ci−1 (possibly c = c′ ) with states ti−1 and t′i−1 , respectively.
Since M is synchronised and since Ci−1 contains configurations with remaining input vi−1
that can be reached from the initial configuration of M on input w by (i − 2) contracted transitions, it is not possible that there are contracted transitions (ti−1 , Ci−1 , yi−1 ) →con ti and
′
′
′
(t′i−1 , Ci−1
, yi−1
) →con t′i for c and c′ , respectively, such that Ci−1 and Ci−1
have different effects
′
on memory statuses (Ri−1 [1], . . . , Ri−1 [k]) or yi−1 and yi−1 cause different prefixes of vi−1 to be
consumed. Consequently, all contracted transitions applicable to configurations from Ci−1 are
such that they consume the same prefix of the remaining input vi−1 (although the transitions may
recall different memories, or one transition recalls a memory while the other consumes a single
input symbol) and have the same effect on the memory statuses (although the actual memory instructions of the transitions might differ), i. e., they will lead to configurations that can only differ
with respect to their states. Consequently, the set of all configurations that can be obtained by
applying a contracted transition to a configuration from Ci−1 can be obtained as follows: The new
remaining input vi and the new memory configurations (Ui [1], Ri [1]), . . . , (Ui [k], Ri [k]) (which
are the same for all the configurations that can be obtained by applying a contracted transition
14

<!-- page 15 -->

to a configuration from Ci−1 ) is obtained by carrying out one arbitrary of these transitions (i. e.,
consuming a prefix of the input according to the transition and changing the memory statues
(Ui−1 [1], Ri−1 [1]), . . . , (Ui−1 [k], Ri−1 [k]) according to the memory instructions of the transition),
while the new states can be obtained by collecting all states that are reachable by any contracted
transition from some state of a configuration from Ci−1 that is applicable (i. e., that consumes the
first symbol from the remaining input or recalls a memory that stores a prefix of the remaining
input).
We observe that in Line 4, we compute the set of all terminal symbols and memory recalls
that can be part of an applicable transition, in Line 5, we check whether there is at least one
applicable transition and if this is the case, we compute the new set Ci as described above
in Lines 8 to 12. Consequently, we can conclude that Ci contains all configurations that can
be reached from a configuration of Ci−1 by a contracted transition; thus, with the induction
hypothesis, Ci contains all configurations that can be reached from the initial configuration of
M on input w by (i − 1) contracted transitions.
(Claim 1) □
By Claim 1, the algorithm searches all possible configurations that are reachable from the initial
configuration of M on input w. If among the current configurations there is an accepting one (i. e.,
the input is completely consumed and an accepting state is reached), then, due to Lines 13 and 14,
the algorithm terminates with output Yes (note that we also have to check whether an accepting
state can be reached by non-consuming transitions). If there is no applicable transition for
any of the current configuration from Ci , then the condition in Line 5 is not satisfied and
therefore the algorithm terminates with output No. However, if the input has been completely
consumed and the conditions in Line 13 is not satisfied, we cannot necessarily conclude that
no accepting configuration is reachable, since there might be applicable transitions that recall
empty memories and that eventually lead to an accepting configuration. Therefore, we proceed
with the computation even though the remaining input is empty (note that in this case, Line 4
is interpreted as Λ = {x ∈ [k] | U [x] = ε}).
The only case not discussed so far is when we reach a loop of transitions that recall empty
memories, but none of the traversed configurations are accepting (i. e., the remaining input is
non-empty or none of the states is accepting). Due to the condition of Line 3, which is not
satisfied if the counter b exceeds |Q|, and Line 7, in which b is incremented if empty memories
are recalled and reset to 0 otherwise, the algorithm returns No if there is a sequence of at least
|Q| + 1 consecutive transitions that recall empty memories. The following claim shows that this
is correct (and therefore concludes the proof of correctness).
Claim 2: If b = |Q| + 1 in Line 3, then w ∈
/ L(M ).
Proof of Claim 2: A fundamental observation is that if for some i and every j with i + 1 ≤
j ≤ i + d, the set Cj is obtained from Cj−1 by transitions that recall empty memories, and
Si+d−1
( j=i Aj ) ∩ Ai+d ̸= ∅, then a transition that recalls an empty memory is also applicable in
iteration i + d and therefore Ai+d+1 is obtained from Ai+d by transitions that recall an empty
memory. This is due to the fact that in these iterations every empty memory stays empty and
can therefore always be recalled if a state has such a transition. If d is large enough, then, with
respect to the visited states, there must exist a loop of transitions that recall an empty memory.
More precisely, if d > |Q|, then, for every j ′ ≥ i + d, there must be some state in Aj ′ that allows
a transition that recalls an empty memory. Therefore, M is in an infinite loop of transitions
that recall empty memories. We check whether this is happening by counting in b (see Line 7)
the number of such consecutive iterations caused by transitions that recall empty memories and
interrupt the main loop (and return No) if b properly exceeds |Q|.
If this happens with vi ̸= ε, this is correct since M cannot finish to consume its input in
the loop of transitions that recall empty memories. Let us assume that in iteration i, i. e.,
when M enters the loop, the remaining input is empty. In this case, we have to check whether
an accepting state is reachable from an active one by performing transitions that recall empty
memories, followed by a sequence of non-consuming transitions. If this is the case, then it must
be possible to reach this state by at most |Q| transitions that recall empty memories (followed
by a sequence of non-consuming transitions). Consequently, it is sufficient to perform at most
|Q| more iterations and check whether accepting states can be reached from active states by
non-consuming transitions, which is done in Lines 13 and 14.
(Claim 2) □
Running-time: We estimate the running-time by estimating the time required for the pre15

<!-- page 16 -->

oxi /a

oxi /cxi /b

rj a

cxi

pi
oxi /b

qi
oxi /cxi /a

(a)

cxi

b

a
b

ozj /yj,1
ozj /yj,2
ozj /yj,3

czj

sj

a ... a u

1

a
t

z1

z2

(b)

...

u
zm 2

oz1 /b
b

(c)

Figure 6: Illustrations for the proof of Thm. 14.
processing in Line 1, the time required for one iteration of the main loop, and the maximum
number of iterations of the main loop. According to Lemmas 11 and 12, the preprocessing can
be done in time O(|Q|2 σ). Regarding an execution of the main loop, we observe that Line 7
requires constant time and, since we can concatenate and compare factors in constant time (due
to the LCEw data-structure), Lines 6 and 11 also require constant time. For executing each of
the Lines 4 and 8, as well as the complete loop of Lines 9 and 10, time O(k) is sufficient (due to
the LCEw data-structure), while the evaluation of the condition of Line 5 requires time O(|Q|k).
This leaves Line 12 and 13, which require time O(|Q|2 k) and O(|Q|2 ), respectively. Summing
up, an execution of the main loop requires time O(|Q|2 k). Next, we note that the main loop
is interrupted as soon as it is executed for |Q| + 1 times without reducing the remaining input;
thus, it can be executed for O(|w||Q|) times in the worst case. We conclude that the total
running-time of the algorithm is O(|Q|2 σ + |w||Q|3 k) = O(|w||Q|3 σ).
Technically, the class of α with synchronised M(α) can be matched efficiently. Unfortunately,
this class is of little use, since deciding membership to it is intractable.
Theorem 14. Deciding whether a given MFA is synchronised is coNP-hard.
Proof. We conduct a reduction from 3SAT to the problem of deciding whether a given MFA is not
synchronised.
Let C = (c1 , c2 , . . . , cm ) be a set of clauses ci = {yi,1 , yi,2 , yi,3 }, 1 ≤ i ≤ m, with
Sm
c
=
{x
,
x1 , x2 , x2 , . . . , xn , xn }. We define an MFA(2n + m) M , which, for every i ∈ [n],
i
1
i=1
has a memory xi and a memory xi , and, for every j ∈ [m], a memory zj . We first construct, for
every i ∈ [n], the component shown in Fig. 6(a), and, for every j ∈ [m], the component shown
in Fig. 6(b). These components are combined by joining states qi , pi+1 , 1 ≤ i ≤ n − 1, joining
states qn , r1 , and joining states sj , rj+1 , 1 ≤ j ≤ m − 1. Finally, we construct the component
shown in Fig. 6(c), we join states sm , t and let p1 be the start state (the only accepting state is
shown in Fig. 6(c)).
Obviously, this reduction can be computed in polynomial-time and it only remains to prove
its correctness.
If there is an input w, such that M can reach both state u1 and state u2 by completely
consuming w, then, since the transition with source u1 stores b in memory z1 , while the one
with source u2 does not change the content of memory z1 (which must store a, since otherwise
we cannot reach u1 and u2 with the same input word), there are non-synchronised computations
with respect to input wb. On the other hand, since the part of M that consists in the components
of Fig. 6(a) and (b) is completely deterministic and the only nondeterminism of M relies in the
state t, the only non-synchronised computations must be due to an input that can lead M into
both states u1 and u2 . Consequently, M is non-synchronised if and only if it can reach both
states u1 and u2 by consuming the same input. Obviously, such an input w exists if and only if
it is possible for M to reach state t with all memories zj , 1 ≤ j ≤ m, storing a.
Now we assume that C is satisfiable and consider the following computation of M . If xi is
assigned value true, then, in the component of Fig. 6(a), we store a in xi and b in xi , and if
xi is assigned value false, then we store b in xi and a in xi . Since every clause contains a true
literal, it is possible to traverse the components of Fig. 6(b) in such a way that every memory zj ,
1 ≤ j ≤ m, stores value a when we reach state t. On the other hand, if we can reach t with all
memories zj , 1 ≤ j ≤ m, storing a, then for every j ∈ [m], there is at least one memory among
the memories yj,1 , yj,2 , yj,3 that stores a, which, considering that the components of Fig. 6(a)
force every pairs of memories xi , xi to store complementary values (with respect to the values a
and b), directly translates into a satisfying assignment of C.
16

<!-- page 17 -->

To achieve the goal stated at the beginning of this section, we formulate a slightly weaker,
but sufficient criterion for the synchronisation property, which can be efficiently checked.

4.2

Memory-Deterministic Regex

Let M = (Q, Σ, δ, q0 , F ) ∈ MFA. Recall that, for any p, q ∈ Q and x ∈ Σk , (q, C, x) →con p means
that we can reach p from q by reading some non-consuming symbols c1 , c2 , . . . , cm ∈ Γk ∪{ε}
followed by reading x, such that Φ(c1 , c2 , . . . , cm ) = C. Moreover, for every q ∈ Q and x ∈ Σk ,
the set δcon (q, x) contains all states p such that (q, C, x) →con p holds for some C; in particular,
δcon (q, x) = ∅ means that from q we cannot read x after some sequence of non-consuming symbols.
We say that q1 ∈ Q and q2 ∈ Q are memory synchronised, denoted by q1 ≎ q2 , if the following
conditions are satisfied.
• For every reduced C1 , C2 ⊆ Γk , x ∈ Σk and p1 , p2 ∈ Q, ((q1 , C1 , x) →con p1 )∧((q2 , C2 , x) →con
p2 ) ⇒ (C1 = C2 ),
• For every x ∈ [k],
– δcon (q1 , x) ̸= ∅ ⇒ (δcon (q2 , y) = ∅ for every y ∈ Σk \ {x}).
– δcon (q2 , x) ̸= ∅ ⇒ (δcon (q1 , y) = ∅ for every y ∈ Σk \ {x}).

Intuitively speaking, these conditions mean that if for both q1 and q2 we can reach consuming
transitions reading the same symbol x, then the non-consuming transitions leading to these xtransitions must have the same effect on the memories. Moreover, if after some non-consuming
transitions from q1 we can reach a memory recall transition for x ∈ [k], then q2 is only allowed
to reach consuming transitions that also recall the same memory x (and vice versa). Also note
that ‘≎’ is not necessarily reflexive, i. e., there might be a state q with q ≎ q.
We say that q1 and q2 are synchronised reachable, denoted by q1 ⇓q2 , if there is a word w and
synchronised computations ⃗c and c⃗′ of M on input w with |⃗c| = |c⃗′ | = m and the states of ⃗c[m]
and c⃗′ [m] are q1 and q2 , respectively. An MFA(k) M = (Q, Σ, δ, q0 , F ) is memory-deterministic
(or an md-MFA(k), for short) if all states q1 , q2 ∈ Q that are synchronised reachable are also
memory synchronised, i. e., for all q1 , q2 ∈ Q, q1 ⇓q2 implies q1 ≎ q2 .
Lemma 15. Every M ∈ md-MFA is synchronised.
con,∗ ′
′
c in order to denote that c′ follows
Proof. In this proof, we use the notation c ⊢con
M c or c ⊢M
from c (in one or several steps, respectively) in a contracted computation.
Let M ∈ md-MFA and let ⃗c and c⃗′ be two different contracted computations of M on the
same input. We shall show that ⃗c and c⃗′ are synchronised.
Let i, 1 ≤ i ≤ max{|⃗c|, |c⃗′ |} − 1, be such that ⃗c[i] and c⃗′ [i] only differ in their states (or
are identical), let q1 and q2 be the states of ⃗c[i] and c⃗′ [i], and let (q1 , C1 , x1 ) →con p1 and
(q2 , C2 , x2 ) →con p2 be the contracted transitions responsible for ⃗c[i] ⊢con ⃗c[i + 1] and c⃗′ [i] ⊢con
c⃗′ [i + 1], respectively. In particular, we observe that this means q1 ⇓q2 . We can also note that
since ⃗c[1] = c⃗′ [1], such an i exists.
If x1 ∈ [k], then we have (q1 , C1 , x1 ) →con p1 with x1 ∈ [k]; thus, the memory-determinism of
M implies that δcon (q2 , y) = ∅ for every y ∈ Σk \ {x}, which implies that x1 = x2 . Consequently,
(q1 , C1 , x1 ) →con p1 and (q2 , C2 , x1 ) →con p2 , which, since q1 ≎ q2 , implies that C1 = C2 .
If, on the other hand, x1 ∈ Σ, then x2 ∈ [k] would be a contradiction to q1 ≎ q2 , which implies
that x2 ∈ Σ. Thus, the contracted transitions (q1 , C1 , x1 ) →con p1 and (q2 , C2 , x2 ) →con p2 are
both reading a symbol from the remaining input and since the remaining inputs of ⃗c[i] and c⃗′ [i]
are the same, these two symbols must be the same, i. e., x1 = x2 . As before, the memorydeterminism now implies that C1 = C2 . In both these cases, it follows that ⃗c[i + 1] and c⃗′ [i + 1]
only differ with respect to their states (or are identical) and, since q1 ⇓q2 , also p1 ⇓p2 .
Consequently, by inductive application of this argument, it follows that ⃗c and c⃗′ are synchronised.

We now define the class RXmd
Σ,X = {α ∈ RXΣ,X | M(α) ∈ md-MFA} of memory-deterministic
regex, which, according to Thm. 13 and Lem. 15, can be matched efficiently.
17

<!-- page 18 -->

3
Corollary 16. The RXmd
Σ,X -matching problem can be solved in time O(|w||α| σ).

In order to substantiate this result, we discuss a more practically motivated example of a
memory-deterministic regex in Section C in the Appendix.
We shall next see that whether an MFA (and therefore a regex) is memory-deterministic can
be checked in polynomial-time, which is the main benefit of the class RXmd
Σ,X . To this end, we
first have to compute the relation ≎. This is not entirely trivial, since it depends on contracted
transitions, which we cannot afford to explicitly compute (see the remark in Section B of the
Appendix). The idea is to first compute a data structure for answering queries of the form:
“given q ∈ Q, x ∈ Σk , c ∈ Γk , are there p ∈ Q and reduced C ⊆ Γk with (q, C, x) →con p and
c ∈ C?” These can be computed by analysing R(α) and are sufficient to evaluate ≎.
Lemma 17. The relation ≎ can be computed in time O(|Q|3 σk).

Proof. First, we compute the sets δcon (q, x), which, according to Lemma 11, requires time
O(|Q|2 σ). We recall that for fixed q1 , q2 ∈ Q, we have q1 ≎ q2 , if the following conditions
are satisfied.
• For every reduced C1 , C2 ⊆ Γk , x ∈ Σk and p1 , p2 ∈ Q, ((q1 , C1 , x) →con p1 )∧((q2 , C2 , x) →con
p2 ) ⇒ (C1 = C2 ).
• For every x ∈ [k],

– δcon (q1 , x) ̸= ∅ ⇒ (δcon (q2 , y) = ∅ for every y ∈ Σk \ {x}).
– δcon (q2 , x) ̸= ∅ ⇒ (δcon (q1 , y) = ∅ for every y ∈ Σk \ {x}).

The second property can be easily verified for all q1 , q2 ∈ Q by checking, for every x ∈ [k]
and y ∈ Σk \ {x}, whether δcon (q1 , x) ̸= ∅ and δcon (q2 , y) ̸= ∅, or whether δcon (q2 , x) ̸= ∅ and
δcon (q1 , y) ̸= ∅. Since checks of the form δcon (q, x) = ∅ can be done in constant time, this requires
a total time of O(|Q|2 kσ).
In order to show how the first property can be checked for all q1 , q2 ∈ Q, we define the
following types of queries:
Q1∈ (q, p, c):
Q2∈ (q, x, c):

does there exists a reduced set C ⊆ Γk
such that (q, C, ε) →con p and c ∈ C?
does there exists a state p ∈ Q and a reduced set C ⊆ Γk
such that (q, C, x) →con p and c ∈ C?

With Q1∈/ (q, p, c) and Q2∈/ (q, x, c), we denote the variants of these queries with c ∈
/ C instead of
c ∈ C.
Next, we discuss how queries Q1∈ (q, p, c) and Q1∈/ (q, p, c) can be evaluated. Let c = o(y)
for some y ∈ [k]. Obviously, Q1∈ (q, p, o(y)) holds if in M there is a path from q to p of
transitions that are labelled with symbols from Γk ∪{ε}, such that there is at least one o(y)transition in this path that is not followed by a c(y)-transition. Checking the existence of such
a path can be done as follows. We remove all z-transitions with z ∈ Σk , we replace all ztransition with z ∈ Γk \{o(y), c(y)} by ε-transitions, and we keep all original ε-transitions. We
have now obtained an NFA M ′ over the alphabet {o(y), c(y)} with ε-transitions. We declare q
to be the start state and p to be the only accepting state. The NFA M ′ accepts exactly the
words over {o(y), c(y)} that correspond to the memory instructions for memory y on a path of
transitions that are labelled with symbols from Γk ∪{ε} and that leads from state q to state p;
moreover, constructing M ′ can be done in time O(|Q|), and M ′ has |Q| states and at most O(|Q|)
transitions. Consequently, the property to be checked holds if and only if L(M ′ ) ∩ L(N ) ̸= ∅,
where N is an automaton for L((o(y) ∨ c(y))∗ o(y)+ ) (note that N has a constant number of
states). Let q0,N be the start state and let qf be the only accepting state of N . By constructing
the cross-product automaton of M ′ and N and checking reachability from (q, q0,N ) to (p, qf ), we
can check whether L(M ′ ) ∩ L(N ) ̸= ∅ in time O(|Q|) (recall that there are O(|Q|) transitions).
The argument for the case c = c(y) is analogous and in order to evaluate queries Q1∈/ (q, p, c),
we let N be an automaton for L(((o(y) ∨ c(y))∗ c(y)+ ) ∨ ε).
Hence, we can evaluate the Q1∈ (q, p, c)- and Q1∈/ (q, p, c)-queries for all q, p ∈ Q and c ∈ Γk
in time O(|Q|3 k). Therefore, we assume in the following that Q1∈ - and Q1∈/ -queries can be
answered in constant time.
18

<!-- page 19 -->

By using Q1∈ -queries, we can evaluate Q2∈ (q, x, c)-queries as follows. We first check if there
is a p′ ∈ δcon (q, ε) and a reduced C ⊆ Γk with (q, C, ε) →con p′ and c ∈ C, which requires O(|Q|)
Q1∈ queries. If this is the case, then we check whether there is a p ∈ δ(p′ , x), which can be done
in constant time. Analogously, we can use Q1∈/ -queries in order to evaluate Q2∈/ -queries. This
means that we can evaluate the Q2∈ (q, x, c)- and Q2∈/ (q, x, c)-queries for all q ∈ Q, x ∈ Σk and
c ∈ Γk in time O(|Q|2 σk). As for Q1∈ - and Q1∈/ -queries, we shall now also assume that Q2∈ and Q2∈/ -queries can be answered in constant time.
It remains to show how to check for every q1 , q2 ∈ Q whether the first property from above
holds, i. e., the property:
For every reduced C1 , C2 ⊆ Γk , x ∈ Σk and p1 , p2 ∈ Q, ((q1 , C1 , x) →con p1 ) ∧
((q2 , C2 , x) →con p2 ) ⇒ (C1 = C2 ).
To do this, we check for every q1 , q2 ∈ Q, x ∈ Σk and c ∈ Γk , whether Q2∈ (q1 , x, c) and
Q2∈/ (q2 , x, c) hold, or whether Q2∈/ (q1 , x, c) and Q2∈ (q2 , x, c) hold. This requires time O(|Q|2 σk).
The total time required for all these computations is O(|Q|3 σk).
For checking whether a given MFA is memory-deterministic, we have to check whether there
are states q1 and q2 such that q1 ⇓q2 and q1 ̸≎ q2 . As shown by Lemma 17, checking whether or
not q1 ≎ q2 for some q1 , q2 ∈ Q can be done in time O(|Q|3 σk). The difficulty of checking whether
q1 ⇓q2 is that this is not defined in terms of local properties of q1 and q2 , or solely in terms of
the structure of M . Thus, we define next a predicate that depends on structural properties of
M and that can be shown to be characteristic for M being non-memory-deterministic.
Definition 18. A triple (q, p1 , p2 ) ∈ Q3 is a non-synchronised branching triple, denoted by
the predicate NSB(q, p1 , p2 ), if p1 and p2 are not memory synchronised, i. e., p1 ̸≎ p2 (recall
the definition of memory synchronised states from above) and either q = p1 = p2 or there are
contracted transitions (t1,i , Ci , xi ) →con t1,i+1 and (t2,i , Ci , xi ) →con t2,i+1 , 1 ≤ i ≤ m − 1, such
that t1,1 = t2,1 = q, t1,m = p1 , t2,m = p2 , and t1,i ≎ t2,i , for every i ∈ {2, 3, . . . , m − 1}.
We first show that the existence of states q, p1 , p2 with NSB(q, p1 , p2 ) characterises nonmemory-determinism, and then we show how to check whether there are such states.
Lemma 19. (∃q1 , q2 ∈ Q : q1 ⇓q2 ∧ q1 ̸≎ q2 ) ⇐⇒ (∃q, p1 , p2 ∈ Q : NSB(q, p1 , p2 )).
Proof. We start with the only if direction and assume that there are q1 , q2 ∈ Q with q1 ⇓q2 and
q1 ̸≎ q2 . If q1 = q2 , then we have that q1 ̸≎ q1 and therefore NSB(q1 , q1 , q1 ) holds; thus, we assume
that q1 ̸= q2 in the following.
By definition, q1 ⇓q2 implies that there is a word w and synchronised computations ⃗c and c⃗′
of M on input w with |⃗c| = |c⃗′ | = m and the states of ⃗c[m] and c⃗′ [m] are q1 and q2 , respectively.
Since ⃗c[1] = c⃗′ [1] and ⃗c[m] ̸= c⃗′ [m] (this follows from q1 ̸= q2 ), there is some j with 2 ≤ j ≤ m
such that ⃗c[i] = c⃗′ [i], 1 ≤ i ≤ j − 1, and ⃗c[j] ̸= c⃗′ [j]. Since ⃗c[j] and c⃗′ [j] can only differ with
respect to their states, we can conclude that ⃗c[j] has a state t and c⃗′ [j] has a state t′ , and t ̸= t′
(note that if j = m, then t = q1 and t′ = q2 ). Moreover, let p be the common state of ⃗c[j − 1]
and c⃗′ [j − 1].
If t ̸≎ t′ , then NSB(p, t, t′ ) holds. If, on the other hand, t ≎ t′ , then, since q1 ̸≎ q2 , there must
be some ℓ with j + 1 ≤ ℓ ≤ m such that ⃗c[ℓ] and c⃗′ [ℓ] have states s and s′ with s ̸≎ s′ (note that
if ℓ = m, then s = q1 and s′ = q2 ), and, for every i with j + 1 ≤ i ≤ ℓ − 1, ⃗c[i] and c⃗′ [i] have
some states ri and ri′ with ri ≎ ri′ . This implies that NSB(p, s, s′ ) holds.
In order to prove the other direction, we assume that, for some q, p1 , p2 ∈ Q, NSB(q, p1 , p2 )
holds. By definition of NSB, this means that p1 ̸≎ p2 ; thus, it only remains to show p1 ⇓p2 .
Let v be some word that leads M to state q, i. e., on input v, M can reach a configuration (q, ε, (u1 , r1 ), . . . , (uk , rk )). Since NSB(q, p1 , p2 ) holds, there are contracted transitions
(t1,i , Ci , xi ) →con t1,i+1 and (t2,i , Ci , xi ) →con t2,i+1 , 1 ≤ i ≤ m − 1, such that t1,1 = t2,1 = q,
t1,m = p1 , t2,m = p2 , and t1,i ≎ t2,i , for every i ∈ [m − 1]. We now define a word v ′ as
follows: we initially set v ′ = ε and start M on configuration (q, ε, (u1 , r1 ), . . . , (uk , rk )) (i. e.,
the configuration that can be reached by M on input v). Then we carry out the transitions
(t1,i , Ci , xi ) →con t1,i+1 , 1 ≤ i ≤ m − 1, one by one, and after applying (t1,i , Ci , xi ) →con t1,i+1 ,
19

<!-- page 20 -->

we append xi to v ′ , if xi ∈ Σ, and we append the current content of memory xi to v ′ , if
xi ∈ [k]. Obviously, (q, v ′ , (u1 , r1 ), . . . , (uk , rk )) ⊢con,∗ (p1 , ε, (u′1 , r1′ ), . . . , (u′k , rk′ )), due to the
transitions (t1,i , Ci , xi ) →con t1,i+1 , 1 ≤ i ≤ m − 1, and, analogously, (p2 , ε, (u′1 , r1′ ), . . . , (u′k , rk′ ))
is reachable from (q, v ′ , (u1 , r1 ), . . . , (uk , rk )) via the states t2,i+1 , 1 ≤ i ≤ m − 1 (in particular, note that these two computations are synchronised). Since, (q0 , vv ′ , (C, ε), . . . , (C, ε)) ⊢con,∗
(q, v ′ , (u1 , r1 ), . . . , (uk , rk )), this directly implies that p1 ⇓p2 .
Lemma 20. Whether there are q, p1 , p2 ∈ Q with NSB(q, p1 , p2 ) can be checked in O(|Q|5 +
|Q|3 σk).
Proof. We assume that the relation ≎ is computed (which, according to Lemma 17, requires time
O(|Q|3 σk)), and we assume that we have the sets δcon (q, x) at our disposal, which, according to
Lemma 11, requires time O(|Q|2 σ).
Let q, p1 , p2 ∈ Q be fixed. Since p1 ≎ p2 implies that NSB(q, p1 , p2 ) does not hold, we only
have to check q, p1 , p2 ∈ Q with p1 ̸≎ p2 ; thus, we assume that p1 ̸≎ p2 in the following.
We construct two NFA Mq,p1 and Mq,p2 from M as follows. The automata Mq,p1 and Mq,p2
have the same states as M , and the only accepting state is p1 and p2 , respectively; the start state
of both Mq,p1 and Mq,p2 is q. Every x-transition with x ∈ Γk of Mq,p1 and Mq,p2 is replaced by
an ε-transitions and all other transitions are left unchanged, i. e., Mq,p1 and Mq,p2 are NFA over
the alphabet Σk and with ε-transitions. Next, we remove the ε-transitions of Mq,p1 and Mq,p2
by setting δ(q, x) = δcon (q, x), for every q ∈ Q and x ∈ Σk .
If NSB(q, p1 , p2 ) holds, then there are contracted transitions (t1,i , Ci , xi ) →con t1,i+1 and
(t2,i , Ci , xi ) →con t2,i+1 , 1 ≤ i ≤ m − 1, such that t1,1 = t2,1 = q, t1,m = p1 , t2,m = p2 , and
t1,i ≎ t2,i , for every i ∈ [m − 1]. In particular, this means that x1 x2 . . . xm−1 ∈ L(Mq,p1 ) ∩
L(Mq,p2 ).
On the other hand, if there is some w ∈ L(Mq,p1 )∩L(Mq,p2 ), then there are (t1,i , C1,i , w[i]) →con
t1,i+1 and (t2,i , C2,i , w[i]) →con t2,i+1 , 1 ≤ i ≤ |w|, such that t1,1 = t2,1 = q, t1,|w|+1 = p1 ,
t2,|w|+1 = p2 (note that these transitions are with respect to the original MFA M ). If, in addition, C1,i = C2,i and t1,i ≎ t2,i for every i with 1 ≤ i ≤ |w|, then NSB(q, p1 , p2 ) holds. On
the other hand, if one of these conditions is not satisfied, then let s, 1 ≤ s ≤ |w| − 1, be minimal such that C1,i = C2,i and t1,i ≎ t2,i , 1 ≤ i ≤ s. This implies that t1,s+1 ̸≎ t2,s+1 or that
C1,s+1 ̸= C2,s+1 , which also implies t1,s+1 ̸≎ t2,s+1 . In both cases, NSB(q, t1,s+1 , t2,s+1 ) holds.
Consequently, by checking L(Mq,p1 ) ∩ L(Mq,p2 ) ̸= ∅, for every q, p1 , p2 ∈ Q with p1 ̸≎ p2 , we
can check whether there are q, p1 , p2 ∈ Q such that NSB(q, p1 , p2 ) holds.
For fixed q, p1 , p2 ∈ Q with p1 ̸≎ p2 , constructing Mq,p1 and Mq,p2 requires time O(|Q|).
Next, we construct the cross-product automaton Mq,p1 ,p2 of Mq,p1 and Mq,p2 , which accepts
L(Mq,p1 ) ∩ L(Mq,p2 ). Since both Mq,p1 and Mq,p2 have |Q| states, Mq,p1 ,p2 has |Q|2 states
and therefore size O(|Q|2 ). Checking whether L(Mq,p1 ,p2 ) ̸= ∅ can therefore be done in time
O(|Mq,p1 ,p2 |) = O(|Q|2 ). We conclude that checking whether there are q, p1 , p2 ∈ Q such that
NSB(q, p1 , p2 ) holds can be done in time O(|Q|5 ). With the initial preprocessing, we get a total
running time of O(|Q|5 + |Q|3 σk).
These lemmas from above show how to check whether a given MFA is memory deterministic.
Lemma 21. Given M ∈ MFA, we can decide whether M ∈ md-MFA in time O(|Q|5 + |Q|3 σk).
Proof. We first compute the relation ≎ in time O(|Q3 |σk) (see Lemma 17). Then we check
whether there are q, p1 , p2 ∈ Q such that NSB(q, p1 , p2 ) holds in time O(|Q|5 + |Q|3 σk) (see
Lemma 20). Since there are q, p1 , p2 ∈ Q such that NSB(q, p1 , p2 ) holds if and only if M is not
memory-deterministic (see Lemma 19), we have checked whether M ∈ md-MFA in total time
O(|Q|5 + |Q|3 σk).
Finally, Proposition 3 and Lemma 21 directly yield the following.
5
Theorem 22. Given α ∈ RXΣ,X , we can decide whether α ∈ RXmd
Σ,X in time O(|α| ).

20

<!-- page 21 -->

5

Conclusions

We presented two different approaches to efficient matching of regex. Since backreferences are
the source of intractability, both approaches rely on restricting them somehow. The difference
is that the concept of the active variable degree tries to reduce the number of required backreferences (although implicitly by re-using the memories of memory automata), while the concept
of memory determinism relies on restricting how backreferences can be used (by imposing determinism for them).
The active variable degree can be considered as a complexity parameter that partitions the
class of all regex into an infinite hierarchy of increasing matching complexity, i. e., a matching
complexity that is exponential only in the active variable degree. In terms of efficiency and
possible practical application, there are two main obstacles. Firstly, matching regex with active
variable degree at most k by constructing MFA(k) (as explained in Section 3) only leads to
acceptable running-times if k is rather small. Secondly, the running times are of the form
|α||w|O(avd(α)) , which is rather problematic under the reasonable assumption that w is large and
α is small.
From a language theoretical point of view, the hierarchy induced by the active variable degree
is related to the natural hierarchy induced by the number of backreferences or variables that are
necessary to describe the language of some regex α (called minvar(α) in the following). Since
regex with k variables can be matched in time |α||w|O(k) , the parameter minvar(α) has a similar
meaning as the active variable degree. However, its algorithmic application is questionable, since
it is not computable (see [20]). In this regard, the active variable degree can be interpreted as a
computable upper bound for minvar(α).
Memory deterministic regex, on the other hand, have the nice property that they can be
matched in time O(|w||α|3 σ), i. e., in a running-time of the form p(|α|)|w| for a polynomial p,
or in linear time if measured in data-complexity. Their disadvantage is that checking memory
determinism requires time O(|α|5 ) and even though their expressive power properly extends the
one of deterministic regex (see [24]), their relevance is unclear in a practical context (however, see
Section C in the Appendix for a discussion of a more practically motivated memory-deterministic
regex). However, the fact that they properly extend classical regular expressions (unlike the
deterministic regex from [24]), which are without doubt of high practical relevance, justifies
some hope that they could be used for practical purposes.
A natural extension of memory determinism would be to allow the contents and statuses of
only few specific memories to differ in different computational branches, while all others must
be synchronised. The concept of memory-determinism could be extended accordingly, e. g., if
only memories Y ⊆ [k] are allowed to be nondeterministic, then “memory-determinism” means
that q1 ⇓q2 implies q1 ≎ q2 , but the definition of ≎ is changed such that q1 ≎ q2 , if the following
conditions are satisfied.
• For every reduced C1 , C2 ⊆ Γk , x ∈ Σk and p1 , p2 ∈ Q,
((q1 , C1 , x) →con p1 ) ∧ ((q2 , C2 , x) →con p2 ) ⇒

(C1 \ {o(j), c(j) | j ∈ Y } = C2 \ {o(j), c(j) | j ∈ Y }) .

• For every x ∈ [k] \ Y ,
– δcon (q1 , x) ̸= ∅ ⇒ (δcon (q2 , y) = ∅ for every y ∈ Σk \ {x}).

– δcon (q2 , x) ̸= ∅ ⇒ (δcon (q1 , y) = ∅ for every y ∈ Σk \ {x}).

Both the active variable degree as well as the concept of memory determinism allow obvious improvements, i. e., the strong active variable degree and synchronised memory automata,
respectively. However, as substantiated by coNP-hardness results (Theorems 7 and 14), these
improvements lead to intractability and are therefore not investigated further.
Last but not least, for all our concepts and results we heavily rely on memory automata, which
further substantiates their usefulness as a matching tool for regex. In particular, we believe that
it would be worthwhile to implement a regex matching tool that is based on memory automata,
which would also be a first step in implementing the approaches developed in this work.
21

<!-- page 22 -->

References
[1] Alfred V. Aho. Algorithms for finding patterns in strings. In Handbook of Theoretical
Computer Science, Volume A: Algorithms and Complexity (A), pages 255–300. 1990.
[2] Antoine Amarilli, Pierre Bourhis, Stefan Mengel, and Matthias Niewerth. Constantdelay enumeration for nondeterministic document spanners. ACM Trans. Database Syst.,
46(1):2:1–2:30, 2021. doi:10.1145/3436487.
[3] Dana Angluin. Finding patterns common to a set of strings. J. Comput. Syst. Sci., 21(1):46–
62, 1980.
[4] Guillaume Bagan, Angela Bonifati, and Benoı̂t Groz. A trichotomy for regular simple
path queries on graphs. In Proceedings of the 32nd ACM SIGMOD-SIGACT-SIGART
Symposium on Principles of Database Systems, PODS 2013, New York, NY, USA - June
22 - 27, 2013, pages 261–272, 2013.
[5] Pablo Barceló. Querying graph databases. In Proceedings of the 32nd ACM SIGMODSIGACT-SIGART Symposium on Principles of Database Systems, PODS 2013, New York,
NY, USA - June 22 - 27, 2013, pages 175–188, 2013.
[6] Pablo Barceló, Leonid Libkin, Anthony Widjaja Lin, and Peter T. Wood. Expressive languages for path queries over graph-structured data. ACM Transactions on Database Systems
(TODS), 37(4):31:1–31:46, 2012.
[7] Pablo Barceló, Juan L. Reutter, and Leonid Libkin. Parameterized regular expressions and
their languages. Theor. Comput. Sci., 474:21–45, 2013.
[8] Henrik Björklund, Wim Martens, and Thomas Timm. Efficient incremental evaluation of
succinct regular expressions. In Proceedings of the 24th ACM International Conference on
Information and Knowledge Management, CIKM 2015, Melbourne, VIC, Australia, October
19 - 23, 2015, pages 1541–1550, 2015.
[9] Tim Bray, Jean Paoli, C. M. Sperberg-McQueen, Eve Maler, and François Yergeau. Extensible markup language XML 1.0 (fifth edition). W3C recommendation. Technical Report
https://www.w3.org/TR/2008/REC-xml-20081126/, W3C, November 2008.
[10] Wojciech Czerwinski, Claire David, Katja Losemann, and Wim Martens. Deciding definability by deterministic regular expressions. J. Comput. Syst. Sci., 88:75–89, 2017.
[11] Joel Day, Pamela Fleischmann, Florin Manea, and Dirk Nowotka. Local patterns. In
37th IARCS Annual Conference on Foundations of Software Technology and Theoretical
Computer Science, FSTTCS 2017, December 11-15, 2017, Kanpur, India, 2017.
[12] Joel D. Day, Pamela Fleischmann, Florin Manea, Dirk Nowotka, and Markus L. Schmid.
On matching generalised repetitive patterns. In Developments in Language Theory - 22nd
International Conference, DLT 2018, Tokyo, Japan, September 10-14, 2018, Proceedings,
pages 269–281, 2018.
[13] Ronald Fagin, Benny Kimelfeld, Frederick Reiss, and Stijn Vansummeren. Document
spanners: A formal approach to information extraction. J. ACM, 62(2):12:1–12:51, 2015.
doi:10.1145/2699442.
[14] Henning Fernau, Florin Manea, Robert Mercas, and Markus L. Schmid. Pattern matching
with variables: Fast algorithms and new hardness results. In 32nd International Symposium
on Theoretical Aspects of Computer Science, STACS 2015, March 4-7, 2015, Garching,
Germany, pages 302–315, 2015.
[15] Henning Fernau, Florin Manea, Robert Mercas, and Markus L. Schmid. Pattern matching
with variables: Efficient algorithms and complexity results. ACM Trans. Comput. Theory,
12(1):6:1–6:37, 2020. doi:10.1145/3369935.

22

<!-- page 23 -->

[16] Henning Fernau and Markus L. Schmid. Pattern matching with variables: A multivariate
complexity analysis. Information and Computation (I&C), 242:287–305, 2015.
[17] Henning Fernau, Markus L. Schmid, and Yngve Villanger. On the parameterised complexity
of string morphism problems. Theory of Computing Systems (ToCS), 59(1):24–51, 2016.
[18] Johannes Fischer and Volker Heun. Theoretical and practical improvements on the rmqproblem, with applications to LCA and LCE. In Combinatorial Pattern Matching, 17th
Annual Symposium, CPM 2006, Barcelona, Spain, July 5-7, 2006, Proceedings, pages 36–
48, 2006.
[19] Fernando Florenzano, Cristian Riveros, Martı́n Ugarte, Stijn Vansummeren, and Domagoj
Vrgoc. Efficient enumeration algorithms for regular document spanners. ACM Trans.
Database Syst., 45(1):3:1–3:42, 2020. doi:10.1145/3351451.
[20] Dominik D. Freydenberger. Extended regular expressions: Succinctness and decidability.
Theory of Computing Systems (ToCS), 53(2):159–193, 2013.
[21] Dominik D. Freydenberger and Mario Holldack. Document spanners: From expressive
power to decision problems. Theory Comput. Syst., 62(4):854–898, 2018. doi:10.1007/
s00224-017-9770-0.
[22] Dominik D. Freydenberger and Markus L. Schmid. Deterministic regular expressions with
back-references. CoRR, abs/1802.01508v1, 2017. URL: https://arxiv.org/abs/1802.
01508v1.
[23] Dominik D. Freydenberger and Markus L. Schmid. Deterministic regular expressions with
back-references. In 34th Symposium on Theoretical Aspects of Computer Science, STACS
2017, March 8-11, 2017, Hannover, Germany, pages 33:1–33:14, 2017.
[24] Dominik D. Freydenberger and Markus L. Schmid. Deterministic regular expressions with
back-references. J. Comput. Syst. Sci., 105:1–39, 2019. doi:10.1016/j.jcss.2019.04.001.
[25] Jeffrey E. F. Friedl. Mastering regular expressions - understand your data and be more
productive: for Perl, PHP, Java, .NET, Ruby, and more (3. ed.). O’Reilly, 2006.
[26] Shudi (Sandy) Gao, C. M. Sperberg-McQueen, and Henry S. Thompson. W3C XML schema
definition language (XSD) 1.1 part 1: Structures. Technical Report https://www.w3.org/
TR/2012/REC-xmlschema11-1-20120405/, W3C, April 2012.
[27] Wouter Gelade, Marc Gyssens, and Wim Martens. Regular expressions with counting:
Weak versus strong determinism. SIAM J. Comput., 41(1):160–190, 2012.
[28] Benoı̂t Groz and Sebastian Maneth. Efficient testing and matching of deterministic regular
expressions. J. Comput. Syst. Sci., 89:372–399, 2017.
[29] Steve Harris and Andy Seaborne. Sparql 1.1 query language. W3C recommendation. Technical Report https://www.w3.org/TR/sparql11-query/, W3C, March 2013.
[30] The IEEE and The Open Group. IEEE std 1003.1-2008, 2016 edition, chapter 9. http:
//pubs.opengroup.org/onlinepubs/9699919799/, 2016.
[31] S.C. Kleene. Representation of events in nerve nets and finite automata. In C.E. Shannon
and J. McCarthy, editors, Automata Studies, volume 34 of Annals of Mathematics Studies,
pages 3–41. Princeton University Press, 1956.
[32] Leonid Libkin and Domagoj Vrgoc. Regular path queries on graphs with data. In 15th
International Conference on Database Theory, ICDT ’12, Berlin, Germany, March 26-29,
2012, pages 74–85, 2012.
[33] Katja Losemann and Wim Martens. The complexity of regular expressions and property
paths in SPARQL. ACM Transactions on Database Systems (TODS), 38(4):24:1–24:39,
2013.
23

<!-- page 24 -->

[34] Katja Losemann, Wim Martens, and Matthias Niewerth. Closure properties and descriptional complexity of deterministic regular expressions. Theor. Comput. Sci., 627:54–70,
2016.
[35] Florin Manea and Markus L. Schmid. Matching patterns with variables. In Combinatorics
on Words - 12th International Conference, WORDS 2019, Loughborough, UK, September
9-13, 2019, Proceedings, pages 1–27, 2019. doi:10.1007/978-3-030-28796-2\_1.
[36] Wim Martens and Tina Trautner. Evaluation and enumeration problems for regular path
queries. In 21st International Conference on Database Theory, ICDT 2018, March 26-29,
2018, Vienna, Austria, pages 19:1–19:21, 2018.
[37] Daniel Reidenbach and Markus L. Schmid. Patterns with bounded treewidth. Information
and Computation (I&C), 239:87–99, 2014.
[38] Markus L. Schmid. Characterising REGEX languages by regular languages equipped with
factor-referencing. Information and Computation (I&C), 249:1–17, 2016.
[39] Markus L. Schmid and Nicole Schweikardt. A purely regular approach to non-regular core
spanners. In 24th International Conference on Database Theory, ICDT 2021, March 23-26,
2021, Nicosia, Cyprus, pages 4:1–4:19, 2021. doi:10.4230/LIPIcs.ICDT.2021.4.
[40] Markus L. Schmid and Nicole Schweikardt. Spanner evaluation over slp-compressed documents. In PODS’21: Proceedings of the 40th ACM SIGMOD-SIGACT-SIGAI Symposium
on Principles of Database Systems, Virtual Event, China, June 20-25, 2021, pages 153–165,
2021. doi:10.1145/3452021.3458325.
[41] Markus L. Schmid and Nicole Schweikardt. Document spanners - A brief overview of
concepts, results, and recent developments. In PODS ’22: International Conference on
Management of Data, Philadelphia, PA, USA, June 12 - 17, 2022, pages 139–150, 2022.
doi:10.1145/3517804.3526069.
[42] Markus L. Schmid and Nicole Schweikardt. Query evaluation over slp-represented document
databases with complex document editing. In PODS ’22: International Conference on
Management of Data, Philadelphia, PA, USA, June 12 - 17, 2022, pages 79–89, 2022.
doi:10.1145/3517804.3524158.
[43] K. Thompson. Programming techniques: Regular expression search algorithm. Communications of the ACM, 11, 1968.

24

<!-- page 25 -->

A

A Detailed Definition of Regex

Here, we define the concept of regular expressions with backreferences in more detail and also
give a sound definition of their semantics.
Let X denote a finite set of variables (as a convention, we normally use symbols like
x, y, z, x1 , x2 , x3 , . . . to denote variables). The set RXΣ,X of regular expressions with backreferences (over Σ and X), also denoted by regex, for short, is recursively defined as follows:
• For every a ∈ Σε , a ∈ RXΣ,X and var(a) = ∅.
• For every α, β ∈ RXΣ,X .
– (α · β) ∈ RXΣ,X and var((α · β)) = var(α) ∪ var(β),

– (α ∨ β) ∈ RXΣ,X and var((α ∨ β)) = var(α) ∪ var(β),
– (α)+ ∈ RXΣ,X and var((α)+ ) = var(α).

• For every x ∈ X, x ∈ RXΣ,X and var(x) = {x}.
• For every α ∈ RXΣ,X and x ∈ X \ var(α), x{α} ∈ RXΣ,X and var(x{α}) = var(α) ∪ {x}.
For α ∈ RXΣ,X , we use α∗ as a shorthand form for α+ ∨ ε, and we usually omit the operator ‘·’,
i. e., we use juxtaposition. If the underlying alphabet Σ and set X of variables is negligible or
clear from the context, we also denote the set of regex by RX. In a regex, we call an occurrence
of symbol x ∈ X a reference to variable x and a subexpression of the form x{α} a binding
of variable x; if we just talk about (occurrences of ) variables, then we refer to a reference or a
binding. We note that the subset of RXΣ,X that can be created by the first two points from above
is exactly the set of regular expressions over Σ, which we also call classical regular expressions,
in order to distinguish them from the regex defined above, and which we denote by REΣ .
There are several ways of how the semantics of regex can be formally defined (see the discussion in [22, 24]). We use a rather simple one (introduced in [38] and also used in [23, 24])
that only relies on so-called ref-words and ref-languages, and classical regular expressions. Let
Γ = {↱x , ↰x | x ∈ X} be a set that contains a special pair of brackets for each variable. For an
α ∈ RX, the ref-version of α is the classical regular expression αref ∈ REΣ∪X∪Γ that is obtained
from α by recursively replacing each variable binding x{β} by ↱x β ↰x . The ref-language of α is
then defined by R(α) = L(αref ). For a ref-word w ∈ R(α), the dereference D(w) is obtained by
recursively replacing every occurrence of a variable x by β, where ↱x β ↰x is the next matching
pair of ↱x , ↰x brackets to the left of this occurrence of x, or ε if no such pair exists. Note that,
by definition of regex and its ref-versions, every w ∈ R(α) is well-formed with respect to each
individual pair of brackets ↱x , ↰x , and it is impossible that some x occurs between matching
pairs of ↱x , ↰x . We extend D from ref-words to ref-languages in the natural way. Finally, for
every α ∈ RX, we define L(α) = {D(w) | w ∈ R(α)}.
The following example illustrates these definitions.
Example 23. Consider α = ((x{a+ }y{b+ }) ∨ y{a+ })(xd)+ y ∈ RXΣ,X with Σ = {a, b, d}, X =
{x, y}, and the corresponding αref = ((↱x a+ ↰x ↱y b+ ↰y ) ∨(↱y a+ ↰y ))(xd)+ y ∈ REΣ∪X∪Γ with
Γ = {↱x , ↰x , ↱y , ↰y }. We note that, e. g., w1 = ↱x aa ↰x ↱y b ↰y xdy ∈ R(α) and w2 = ↱y aaa ↰y xdxdy ∈
R(α), which implies D(w1 ) = aabaadb ∈ L(α) and D(w2 ) = aaaddaaa ∈ L(α). In particular,
due to the presence of an alternation operator, R(α) can contain words with occurrences of
variable x that are not preceded by a pair of brackets ↱x , ↰x and are therefore replaced by
ε by the dereference-function D. As can be easily verified, the language described by α is
{an bm (an d)k bm | n, m, k ≥ 1} ∪ {an dk an | n, k ≥ 1}.

B

A Remark on Contracted Transitions of Memory Automata

For our definitions and results, the concept of contracted transitions are central. Computing all
those contracted transitions and replacing the normal transitions by the contracted ones could
be considered as making an MFA ε-free. More precisely, we could compute for every q ∈ Q
25

<!-- page 26 -->

0

a

a

a
Σ′

a 0
[add]

r

ox

0

ε

a

;

a
ε

a

ε

a

0
a

Σ′

a

ε

ε

a

0

t
a

0
0

a

a

t′

cx

[add]
s
x
′′

t

;
0

Σ

Figure 7: An MFA for the regex r for checking addresses. The symbol a represents the character
group [a-z] (i. e., an arc labelled with a represents individual arcs for each of the symbols in
[a-z]) and the symbol 0 represents the character group [0-9].
and x ∈ Σk the set CT (q, x) of all (p, C) such that q →con (p, C, x). However, we observe that
|CT (q, x)| is not necessarily polynomially bounded. For example, if, for every x ∈ Γk , and j,
1 ≤ j ≤ k, we have (pj−1 , x) → pj , then it can be easily verified that for every reduced C ∈ Γk ,
we have p0 →con (pk , C, x), which implies that |CT (q, x)| ≥ 2k . On the other hand, changing the
model of MFA such that it only allows what we denoted by contracted transitions is problematic,
since then these contracted transitions need to be computed in the transformation from regex
to MFA. This also shows and justifies why our concept of memory determinism is complicated
on a technical level.

C

Example of a Memory-Deterministic Regex

We discuss a more elaborate example of a memory-deterministic regex.
Assume that we are dealing with input strings that have two occurrences of a postal address
(both occurring between the symbols “[add]” and “;”), for which we want to check whether
they are identical and whether they have the right form. In the following definition of a regex
performing this task, we use character groups [a-z] to denote the expression (a ∨ b ∨ . . . ∨ z), “ ”
denotes the space character and Σ is the complete alphabet. The regular expression radd that
checks the correct format of the address is given by radd = rname ·rstreet ·rcity ·rcountry , where rname =
rcountry = [a-z]+ are simple expressions, but rstreet = ([0-9]+ [a-z]+ ∨[a-z]+ [0-9]+ ) and rcity =
([a-z]∗ [0-9]+ [a-z]+ ∨[a-z]+ [a-z]∗ [0-9]+ ) must cater for the different possible orders of street
name and street number (and also for the different possible orders of zip code and city name). The
regex that solves the task described above is then given by r = (Σ′ )∗ [add]x{radd };(Σ′ )∗ [add]x;Σ∗ ,
where Σ′ = Σ \ {[add], ;}. Consider the MFA M shown in Fig. 7 that is equivalent to the regex
r. For the sake of convenience, M differs from M(r), but it can be easily obtained from M(r)
by contracting the obvious paths of ε-transitions.
It can be verified that r is not a deterministic regex (in the sense of [24] and as defined on
page 11); this already follows from the fact that radd is not a deterministic (classical) regular
expression. Moreover, M is not a deterministic MFA. In the following, we show that M is indeed
memory-deterministic. First, we recall that M is memory-deterministic if, for all q1 , q2 ∈ Q,
q1 ⇓q2 implies q1 ≎ q2 , where q1 ⇓q2 means that there is a word w and synchronised computations
⃗c and c⃗′ of M on input w with |⃗c| = |c⃗′ | = m and the states of ⃗c[m] and c⃗′ [m] are q1 and q2 ,
respectively, and q1 ≎ q2 means that the following properties are satisfied:
• For every reduced C1 , C2 ⊆ Γk , x ∈ Σk and p1 , p2 ∈ Q, ((q1 , C1 , x) →con p1 )∧((q2 , C2 , x) →con
p2 ) ⇒ (C1 = C2 ).
• For every x ∈ [k],
– δcon (q1 , x) ̸= ∅ ⇒ (δcon (q2 , y) = ∅ for every y ∈ Σk \ {x}).

– δcon (q2 , x) ̸= ∅ ⇒ (δcon (q1 , y) = ∅ for every y ∈ Σk \ {x}).

We note that there is only one state with a memory recall transition, namely s (see Fig. 7).
Moreover, there is no other state s′ with s ̸= s′ and s⇓s′ (this is due to the fact that any word
26

<!-- page 27 -->

that leads to state s must have a suffix “;u[add]” with u ∈ (Σ′ )∗ and therefore cannot lead to
any other state different from s).
Consequently, the property of memory-determinism can only be violated by states q1 , q2 ∈ Q
with q1 ⇓q2 and x ∈ Σ, such that (q1 , C1 , x) →con p1 and (q2 , C2 , x) →con p2 with C1 ̸= C2 . For
q1 = q2 , this is obviously never the case. If {q1 , q2 } ∩ {r, t} = ∅ (where r and t are as shown in
Fig. 7), then C1 = C2 = ∅. If q1 = r, then q1 ⇓q2 and [add] ∈
/ Σ′ implies q2 = r. Finally, if q1 = t,
then x ∈ {a, ;} must hold and we observe that (t, ∅, a) →con t and (t, {c(x)}, ;) →con t′ are the
only contracted transitions for t. All other states p with some contracted transition (p, C, a) →con
p′ satisfy C = ∅ and for all other states p with some contracted transition (p, C, ;) →con p′ we
note that t̸⇓ p (note that this latter case only applies to p = t′′ ).
Consequently, M is memory-deterministic and therefore r is a memory-deterministic regex.

27
