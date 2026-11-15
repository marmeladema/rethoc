---
file: ../pdfs/2019-aplas-holik-counting-automata-sphere-construction.pdf
citation: "Succinct Determinisation of Counting Automata via Sphere Construction — Holík, Lengál, Saarikivi, Turoňová, Veanes, Vojnar — APLAS 2019 (background)"
doi: 
source: "https://arxiv.org/pdf/1910.01996"
topics: [counting_and_derivatives]
pages: 19
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Succinct Determinisation of Counting Automata via
Sphere Construction (Technical Report) ⋆
Lukáš Holík1 , Ondřej Lengál1 , Olli Saarikivi2,
Lenka Turoňová1, Margus Veanes2 , Tomáš Vojnar1

arXiv:1910.01996v1 [cs.FL] 4 Oct 2019

1

FIT, Brno University of Technology, IT4Innovations Centre of Excellence, Czech Republic
2 Microsoft Research, Redmond, USA

Abstract. We propose an efficient algorithm for determinising counting automata
(CAs), i.e., finite automata extended with bounded counters. The algorithm avoids
unfolding counters into control states, unlike the naïve approach, and thus produces
much smaller deterministic automata. We also develop a simplified and faster
version of the general algorithm for the sub-class of so-called monadic CAs
(MCAs), i.e., CAs with counting loops on character classes, which are common in
practice. Our main motivation is (besides applications in verification and decision
procedures of logics) the application of deterministic (M)CAs in pattern matching
regular expressions with counting, which are very common in e.g. network traffic
processing and log analysis. We have evaluated our algorithm against practical
benchmarks from these application domains and concluded that compared to the
naïve approach, our algorithm is much less prone to explode, produces automata
that can be several orders of magnitude smaller, and is overall faster.

1 Introduction
The counting operator—also known as the operator of limited repetition—is an operator
commonly used in extended regular expressions (also called regexes). Limited repetitions
do not extend expressiveness beyond regularity, but allow one to succinctly express
patterns such as (ab){1,100} representing all words where ab appears 1–100 times.
Such expressions are very common (cf. [3]), e.g., in the RegExLib library [20], which
collects expressions for recognising URIs, markup code, pieces of Java code, or SQL
queries; in the Snort rules [17] used for finding attacks in network traffic; or in real-life
XML schemas, with the counter bounds being as large as 10 million [3]. This observation
is confirmed by our own experiments with patterns provided by Microsoft for verifying
absence of information leakage from network traffic logs. Counting constraints may
also naturally arise in other contexts, such as in automata-based verification approaches
(e.g. [11]) for describing sets of runs through a loop with some number of repetitions.
Several finite automata counterparts of regular counting constraints have appeared
in the literature (e.g. [13,15,24,25]), all essentially boiling down to variations on counter
automata with counters limited to a bounded range of values. Such counters do not
extend the expressive power beyond regularity, but bring succinctness, exactly as
⋆ This work has been supported by the Czech Science Foundation (project No. 19-24397S), the

IT4Innovations Excellence in Science (project No. LQ1602), and the FIT BUT internal project
FIT-S-17-4014.

<!-- page 2 -->

the counters in extended regular expressions. In this paper, we call these automata
counting automata (CAs).
The main contribution of this paper is a novel succinct determinisation of CAs.
Our main motivation is in pattern matching, where deterministic automata allow for
algorithms running reliably in time linear to the length of the text. However, the naïve
determinisation of CAs (and counting constraints in general)—which encodes counter
values as parts of control states, leading to classical nondeterministic finite automata
(NFAs), which are then determinised using the standard subset construction—can easily
lead to state explosion, causing the approach to fail. See, e.g., the CA in Fig. 1, for which
the minimal deterministic finite automaton (DFA) has 2k+1 states with k being the upper
bound of the counter. Backtracking-based algorithms, which can be used instead, are
slower and unpredictable, may easily require a prohibitively large time, and are even
prone to DoS attacks, cf. [19]. A viable alternative is on-the-fly determinisation, which
determinises only the part of the given NFA through which the input word passes, as
proposed already in [27]. However, the overhead during matching might be significant,
and the construction can still explode on some words, much like the full determinisation,
especially when large bounds on counters are used (which, in our experience, makes
some regex matchers to give up already the translation to NFAs).
Our algorithm, which allows one to succinctly determinise CAs, is therefore a major step towards alleviating the above problems by making the determinisation-based
algorithms applicable more widely. We note that this has been an open problem (whose
importance was stressed, e.g., in [25]) that a number of other works, such as [13,15],
have attempted to solve, but they could only cope with very restricted fragments or
alleviate the problem only partially, yielding solutions of limited practical applicability.
Our algorithm is general and often produces small results. Moreover, we also propose a version specialised to counting restricted to repetition of single characters from
some character class, called monadic counting here (e.g., [ab]{10} is monadic while
(ab){10} is not). This class is of particular practical relevance since we discovered
that most of the regular expressions with counters used in practice are of this form.
Our specialised algorithm can produce deterministic CAs exponentially more succinct
than the corresponding DFAs and its worst-case complexity is only polynomial in the
maximum values of counters (in contrast to the exponential naïve construction).
We have implemented the monadic CA determinisation and evaluated it on real-life
datasets of regular expressions with monadic counting. We found that our resulting
CAs can be much smaller than minimal DFAs, are less prone to explode, and that
our algorithm, though not optimised, is overall faster than the naïve determinisation
that unfolds counters. We also confirmed that monadic regexes present an important
subproblem, with over 95 % of regexes in the explored datasets being of this type.
c < k, c ′ = c + 1
⊤
Running example. To illustrate our algorithms,
consider the regex .*a.{k} where k ∈ N. It says
l = a, c ′ = 0
q
r {c = k}
⊤
that the (k + 1)-th letter from the end of the word
must be a. The minimal DFA accepting the lan- Fig. 1. A CA for the regex .*a.{k} with
guage has 2k+1 states since it must remember in k ∈ N, I : s = q, F : s = r ∧ c =
→q ∨ q−{l=a,c′ =0 }
→r ∨
its control states the positions of all letters a that k, and ∆ : q−{⊤,⊤}
′
{c<k,c
=c+
1
}
r−
→r.
were seen during the last k + 1 steps. For this, it
2

<!-- page 3 -->

l,a
⊤

{q 7→ 1}

l,a
p0 = 1

l , a, p1 = 1
p0′ = p0 + 1

l , a, p0 < 1
p0′ = p0 + 1

{p0 = 1} {q 7→ 1, r 7→ 1}
l=a
p0′ = 0

l , a, p1 < 1
p0′ = p0 + 1, p1′ = p1 + 1

{p1 = 1} {q 7→ 1, r 7→ 2}

l = a, p0 = 1
p0′ = 0

l = a, p0 < 1
p0′ = 0, p1′ = p0 + 1

l = a, p1 = 1
p0′ = 0, p1′ = p0 + 1

Fig. 2. The DCA generated from the CA of Fig. 1 for k = 1 by our algorithm for determinisation
of monadic CA (Section 4.2).

needs a finite memory of k + 1 bits, which has 2k+1 reachable configurations. The regex
corresponds to the nondeterministic CA of Fig. 1. In the transition labels, the predicates
over the variable l constrain the input symbol, the predicates over c constrain the current value of the counter c, and the primed variant of c, i.e., c ′, stands for the value
of c after taking the transition. The initial value of c is unrestricted, and the automaton
accepts in the state r if the value of c equals k. Our monadic determinisation algorithm,
presented in Section 4.2, then outputs the deterministic CA (DCA) of Fig. 2 (for k = 1).
Intuitively, it uses k + 1 counters to remember how far back the last k + 1 occurrences of
a appeared. Depending on k, the resulting DCA has k + 2 states, 4(k + 1) + 1 transitions,
and k + 1 counters. That is, its size is linear to k in contrast to the factor 2k in the size
of the minimal DFA.
⊔
⊓

2 Counting Automata
Preliminaries. We use N to denote the set of natural numbers {0, 1, 2, . . .}. Given
a function f : A → B, we refer to the elements of f using a 7→ b (when f (a) = b). For
the rest of the paper, we consider a fixed finite alphabet Σ of symbols. A word over Σ is
a finite sequence of symbols w = a1 · · · an ∈ Σ∗ . We use ǫ to denote the empty word.
Given a set of variables V and a set of constants Q (disjoint with N), we define a Qformula over V to be a quantifier-free formula ϕ of Presburger arithmetic extended with
constants from Q and Σ, i.e., a Boolean combination of (in-)equalities t1 = t2 or t1 ≤ t2
where t1 and t2 are constructed using +, N, and V, and predicates of the form x = a or
x = q for x ∈ V, a ∈ Σ, and q ∈ Q. An assignment M to free variables of ϕ is a model of
ϕ, denoted as M |= ϕ, if it makes ϕ true. We use sat(ϕ) to denote that ϕ has a model.
Given a formula ϕ and a (partial) map θ : terms(ϕ) → S, where terms(ϕ) denotes
the set of terms in ϕ and S is some set of terms, ϕ[θ] denotes a term substitution,
i.e., the formula ϕ with all occurrences of every term t ∈ dom(θ) replaced by θ(t).
As usual, replacing a larger term takes priority over replacing its subterms (we treat
primed variables and parameters as atomic terms, hence (p′ = 1)[{p 7→ q}] is still
p′ = 1). The substitution formula ϕθ of θ is defined as the conjunction of equalities
def Ó
of a finite
ϕθ = t ∈dom (θ) (θ(t) = t). Finally, the set of minterms
Ó set Φ of predicates is
Ó
defined as the set of all satisfiable predicates of { φ ∈Φ′ φ ∧ φ ∈Φ\Φ′ ¬φ | Φ′ ⊆ Φ}.

Labelled transition systems. We will introduce our counting automata, such as that
of Fig.1, as a specialisation of the more general model of labelled transition systems.
3

<!-- page 4 -->

This perspective and related notation allows for a more abstract and concise formulation
of our algorithms than the more standard approach, in which one would define counting automata in a more straightforward manner as an extension of the classical finite
automata.
A labelled transition system (LTS) over Σ is a tuple T = (Q, V, I, F, ∆) where Q is
a finite set of control states, V is a finite set of configuration variables, I is the initial
Q-formula over V, F is the final Q-formula over V, and ∆ is the transition Q-formula over
V ∪ V ′ ∪ {l} with V ′ = {x ′ | x ∈ V }, V ∩ V ′ = ∅, and l < V. We call l the letter/symbol
variable and allow it as the only term that can occur within a predicate l = a for a ∈ Σ,
called an atomic symbol guard.3 Moreover, l is also not allowed to occur in any other
predicates in ∆. A configuration is an assignment α : V → N ∪ Q that maps every
configuration variable to a number from N or a state from Q. Let C be the set of all
configurations. The transition formula ∆ encodes the transition relation J∆K ⊆ C × Σ × C
such that (α, a, α ′) ∈ J∆K iff α ∪ {x ′ 7→ k | α ′(x) = k} ∪ {l 7→ a} |= ∆. We use |∆| to
denote the size of J∆K. For a word w ∈ Σ∗ , we define inductively that a configuration α ′
w
ǫ
av
is a w-successor of α, written α −
→ α ′, such that α −
→ α for all α ∈ C, and α −−→ α ′ iff
a
v
α−
→ ᾱ −
→ α ′ for some ᾱ ∈ C, a ∈ Σ, and v ∈ Σ∗ . A configuration α is initial or final if
α |= I or α |= F, respectively. The outcome of T on a word w is the set out T (w) of all
w-successors of the initial configurations, and w is accepted by T if out T (w) contains a
final configuration. The language L(T ) of T is the set of all words that T accepts.
Counting automata. A counting variable (counter) is a configuration variable c whose
value ranges over N and which can appear (within ∆, I, and F) only in atomic counter
guards of the form c ≤ k, c ≥ k, (using <, =, > as syntactic sugar) or term equality tests
t1 = t2 , and in atomic counter assignments c ′ = t with t, t1, t2 being arithmetic terms
of the form d + k or k with k ∈ N and d being a counter. A control state variable is
a variable s whose value ranges over states Q and appears only in atomic state guards
s = q and atomic state assignments s′ = q for q ∈ Q. A Boolean combination of atomic
guards (counter, state, or symbol) is a guard formula and a Boolean combination of
atomic assignments is an assignment formula.
A (nondeterministic) counting automaton (CA) is a tuple A = (Q, C, I, F, ∆) such
that (Q, V, I, F, ∆) is an LTS with the following properties: (1) The set of configuration
variables V = C ∪ {s} consists of a set of counters C and a single control state variable s
s.t. s < C. (2) The transition formula ∆ is a disjunction of transitions, which are
}
where q, r ∈ Q,
conjunctions of the form s = q ∧ g ∧ f ∧ s′ = r, denoted by q−{g, f→r,
g is the transition’s guard formula over V ∪ {l}, and f is the transition’s counter
assignment formula, a conjunction of atomic assignments to counters, in which every
counter is assigned at most once. (3) There is a constant max A ∈ N such that no counter
can ever grow above that value, i.e., ∀c ∈ C ∀w ∈ Σ∗ ∀α ∈ out T (w) : α |= c ≤ max A .
The last condition in the definition of CAs is semantic and can be achieved in
different ways in practice. For instance, regular expressions can be compiled to CAs
where assignment terms are of the form c + 1, 0, or c only, and every appearance of
c + 1 is paired with a guard containing a constraint c ≤ k for some k ∈ N. In this case,
max A = K + 1 where K is the maximum constant used in the guards of the form c ≤ k.
3 To handle large or infinite sets of symbols symbolically, the predicates l = a may be generalised
to predicates from an arbitrary effective Boolean algebra, as in [6].

4

<!-- page 5 -->

Ô We will often consider the initial and final formulae of CAs given as a disjunction
q ∈Q (s = q ∧ ϕq ) where ϕq is a formula over counter guards, in which case we write
I(q) or F(q) to denote the disjunct ϕq of the initial or final formula, respectively. An
example of a CA is given in Fig. 1.
A deterministic counting automaton (DCA) is a CA A where I has at most one
model and, for every symbol a ∈ Σ, every reachable configuration α has at most one asuccessor (equivalently, the outcome of every word in A is either a singleton or the empty
set). Finally, in the special case when C = ∅, the CA is a (classical) nondeterministic
finite automaton (NFA), or a deterministic finite automaton (DFA) if it is deterministic.

3 Determinisation of Counting Automata
In this section, we discuss an algorithm for determinising CAs. A naïve determinisation
converts a given CA A into an NFA by hard-wiring counter configurations as a part of
control states, followed by the classical subset construction to determinise the obtained
NFA (the NFA is finite due to the bounds on the maximum values of counters). The
state space of the obtained DFA then consists of all reachable outcomes of A. By
determinising A in this way, the succinctness of using counters is lost, and the size
of the DFA can explode exponentially not only in the number of control states of A
but also in the number of reachable counter valuations, which makes the construction
impractical. Instead, our construction will retain counters (though their number may
grow) and represent possible word outcomes as configurations of the resulting DCA.
Spheres. In particular, the outcome of a word w ∈ Σ∗ in a CA A = (Q, C, I, F, ∆) can
be represented as a formula ϕ over equalities of the form c = k and s = q where q ∈ Q,
c ∈ C, k ∈ N. Intuitively, disjunctions can be used to obtain a single formula for the
possibly many configurations reachable in A over w. For example, the outcome of the
word aab in Fig. 1 is ϕ : s = q ∨ (s = r ∧ (c = 1 ∨ c = 2)). Generally, the outcome of
aabi , for 0 ≤ i < k, assuming k > 2, is ϕi : s = q ∨ (s = r ∧ (c = i ∨ c = i + 1)).
A crucial notion for our construction is then the notion of sphere. A sphere ψ
arises from an outcome ϕ by replacing the constants from N by parameters drawn from
a countable set P (disjoint from N, V, Q, and {l, s}). In the example above, the sphere
obtained from the ϕ is ψ : s = q ∨ (s = r ∧ (c = p0 ∨ c = p1 )), and the same sphere
arises from all outcomes ϕi with 0 ≤ i < k.
Spheres will play the role of the control states of the resulting DCA. The idea of the
construction is that the outcome of every word w in a DCA Ad will contain a single
configuration (Ad is deterministic) consisting of a sphere ψ as the control state and
a valuation of its parameters η : P → N. The construction will ensure that ψ[η] models
the outcome out A(w) of w in A. In our example, the outcome of aab in Ad would
contain the single configuration {s 7→ ψ, p0 7→ 1, p1 7→ 2}, and the outcome of each ϕi ,
for 0 ≤ i < k, would contain the single configuration {s 7→ ψ, p0 7→ i, p1 7→ i + 1}. The
example shows the advantage of our construction. Every outcome ϕi would be a control
state of the naïvely determinised automaton, with a b-transition from each ϕ j to ϕ j+1 ,
for 0 ≤ j < k − 1. In contrast to that, all these states and transitions will be in Ad
replaced by a single control state ψ with a single b-labelled self-loop that increments
both p0 and p1 . This structure can be seen in Fig. 2 (states are spheres, labelled by their
multiset representation introduced in Section 4.2).
5

<!-- page 6 -->

3.1 Determinisation by Sphere Construction
We now provide a basic version of our sphere-based determinisation, which can also
be viewed as an algorithm that constructs parametric versions of the subsets used in
subset-based determinisation. For this basic algorithm, termination is not guaranteed,
but it serves as a basis on which we will subsequently build a terminating algorithm. Let
us first introduce some needed additional notation.
Given a formula ϕ, we denote by at(ϕ) and by num(ϕ) the sets of assignment terms
and numerical constants, respectively, appearing in ϕ. We will use the set P ′ = {p′ |
p ∈ P} and the substitution θ unprime = {p′ 7→ p | p ∈ P}. We say that a formula over
Ôn
variables V ∪ V ′ ∪ {l} ∪ P is factorised wrt guards if it is a disjunction i=
1 (gi ) ∧ (ui )
of factors, each consisting of a guard gi over V ∪ {l} ∪ P and an update formula ui
over atomic assignments such that the guards of any two different factors are mutually
exclusive, i.e., gi ∧ g j is unsatisfiable for any 1 ≤ i , j ≤ n.4 For a set of variables U,
we denote by ∃∃U : ϕ a formula obtained by eliminating all variables in U from ϕ (i.e.,
a quantifier-free formula equivalent to ∃U : ϕ).5
The algorithm. The core of our determinisation algorithm is the sphere construction
described in Algorithm 1. It builds a DCA Ad = (Q d, P, I d, F d, ∆d ) whose control
states Q d are spheres. Its counters are parameters from the set P that is built during the
run of the algorithm. The initial formula I d defined on line 3 assigns to s the initial
control state ψI (obtained on line 2), which is a parametric version of I with integer
constants replaced by parameters according to the renaming θ const . Moreover, I d also
equates the parameters in ψI with the constants they are replacing in I. Hence, the
−1 ] models exactly the initial configurations of A.
formula ψI [θ const
Example 1. In the running example (Fig. 1), whenever referring to some variable that
is assigned multiple times during the run of the algorithm, we use superscripts to
distinguish the different assignments during the run. On lines 1–4, the initial sphere ψI
is assigned the formula s = q, and the initial formula I d is set to s = ψI , which specifies
that ψI is indeed the initial control state only (I does not constrain counters, hence I d
does not talk about parameters).
⊔
⊓
The remaining states of Q d and transitions of ∆d are computed by a worklist algorithm on line 5 with the worklist initialised with ψI . Every iteration computes the
outgoing transitions of a control state ψ ∈ Worklist as follows: On line 7, after eliminating C ∪ {s} from the formula ψ ∧ ∆, which describes how the next state and counter
4 A Boolean combination of atomic guards and updates can be factorised through (1) a transformation to DNF, yielding a set of clauses X; (2) writing each clause ϕ ∈ X as a conjunction
of a guard formula gϕ and an assignment formula fϕ ; (3) computing minterms of the set
{gϕ | ϕ ∈ X }; (4) creating one factor (g) ∧ ( f ) from every minterm g where f is the disjunction
of all the assignment formulae fϕ with ϕ ∈ X compatible with g (i.e., such that g ∧ fϕ is
satisfiable).
5 We note that we only need to use a specialised, simple, and cheap quantifier elimination. In
particular, we only need to eliminate counter variables c from formulae such that, in clauses of
their DNF, c always appears together with a predicate c = p where p is a parameter. Eliminating
c from such a DNF clause is then done by simply substituting occurrences of c by p. We do not
need complex algorithms such as the general quantifier elimination for Presburger arithmetic.

6

<!-- page 7 -->

Algorithm 1: Sphere-based CA determinisation (non-terminating)
Input: A CA A = (Q, C, I, F, ∆).
Output: A DCA Ad = (Q d, P, I d, F d , ∆d ) s.t. L(A) = L(Ad ).
d
d
1 Q ← Worklist ← ∅; ∆ ← ⊥;
2 ψI ← I[θ const ] for some total injection θ const : num(I) → P;
d
3 I ← s = ψI ∧ ϕθconst ;
d
4 add ψI to Q and to Worklist ;
5 while Worklist , ∅ do
6
ψ ← pop(Worklist );
Ôn
7
Let i=
∃∃C, s : ψ ∧ ∆ factorised wrt guards;
1 (gi ) ∧ (ui ) be the formula ∃
8
foreach 1 ≤ i ≤ n do
9
ψi ← ui [θ at ][θ unprime ] for a total injection θ at : at (ui ) → P ′;
→ψi to ∆d ;
10
add ψ−{gi,ϕθat }
d
11
if ψi < Q then add ψi to Q d and to Worklist ;
d
12 P ← all parameters found in Q ;
Ô
d
13 F ←
∃∃C, s : ψ ∧ F;
ψ ∈Q d s = ψ ∧ ∃
I d ← ground (I d ); ∆d ← ground (∆d );
d
d
d d d
15 return A = (Q , P, I , F , ∆ );
14

values depend on the input symbol and the current values of parameters, it is transformed
into a guard-factorised form.
Example 2. When ψI is taken from Worklist as ψ 1 on line 6, its processing starts
by factorising ∃∃ {c, s} : ψ 1 ∧ ∆ on line 7. Here, ψ 1 ∧ ∆ is the formula s = q ∧
(q−{⊤,⊤}
→q ∨ q−{l=a,c′ =0 }
→r ∨ r−{c<k,c′ =c+1 }
→r), which can be also written as
s = q ∧ (s′ = q ∨ (l = a ∧ c ′ = 0 ∧ s′ = r)) .
The elimination of {c, s} gives the formula s′ = q ∨ (l = a ∧ c ′ = 0 ∧ s′ = r). This
formula is factorised into the following two factors:
(F1 ) (l = a) ∧ (s′ = q ∨ (c ′ = 0 ∧ s′ = r)),
(F2 ) (l , a) ∧ (s′ = q).
⊔
⊓
In the for-loop on line 8, every factor (gi ) ∧ (ui ) is turned into a transition with
the guard gi ; the mutual incompatibility of the guards guarantees determinism. The
formula ui describes the target sphere in terms of the parameters of the source sphere
ψ, updated according to the transition relation. That is, it is a Boolean combination
of assignments of the form c ′ = p + k or c ′ = k for c ∈ C, p ∈ P, and k ∈ N.
Line 9 creates a sphere by substituting each of the assignment terms (of the form p + k
or k) with a parameter and replacing primed variables by their unprimed versions.6
The corresponding assignment term substitution θ at records how the values of the new
parameters are obtained from the original values of the parameters occurring in ψ. It is
used to define the assignment formula of the new transition that is added to ∆d on line 10.
6 The choice of the parameters in the image of θ at : at (ui ) → P ′ on line 9 is arbitrary, although,
in practice, it would be sensible to define some systematic parameter naming policy and reuse
existing parameters whenever possible.

7

<!-- page 8 -->

The argument justifying that the construction preserves the language is the following: if
reading w ∈ Σ∗ takes Ad to ψ with a parameter valuation η such that ψ[η] is equivalent
to out A(w), then reading a next symbol a using a transition newly created on line 10
takes Ad to ψ ′ with the parameter valuation η ′ such that ψ ′ [η ′ ] models out A(wa).
Example 3. Factor F1 of Example 2 above is processed as follows. A possible choice for
1
θ at
on line 9 is the assignment {0 7→ p0 }. Its application followed by θ unprime creates
ψ11 : s = q ∨ (c = p0 ∧ s = r).
1
From θ at
, we get the substitution formula ϕθat1 : (p0′ = 0) on line 10, and so the
→ (s = q ∨ (c = p0 ∧ s = r)) . The target ψ11
transition added to ∆d is (s = q)−{l=a, p0′ =0 }
d
of the transition is added to Q and to Worklist on line 11. Next, Factor F2 generates
→ (s = q), which ends the first iteration of the while-loop.
the self-loop (s = q)−{l,a,⊤}
Let us also walk through a part of the second iteration of the while-loop, in
which ψ11 is taken from Worklist as ψ 2 on line 6. The formula ψ 2 ∧ ∆ from line 7
is ((s = r ∧ c = p0 ) ∨ s = q) ∧ (q−{⊤,⊤}
→q ∨ q−{l=a,c′ =0 }
→r ∨ r−{c<k,c′ =c+1 }
→r), which is
′
′
′
equivalent to (s = q ∧ (s = q ∨ (l = a ∧ c = 0 ∧ s = r))) ∨ s = r ∧ c = p0 ∧ c <
k ∧ c ′ = c + 1 ∧ s′ = r . The elimination
of {c, s} on line 7 then gives the formula

s′ = q ∨ (l = a ∧ c ′ = 0 ∧ s′ = r) ∨ (p0 < k ∧ c ′ = p0 + 1 ∧ s′ = r), which is factorised
into the following four factors:
(F3 ) (l = a ∧ p0 < k) ∧ (s′ = q ∨ (c ′ = 0 ∧ s′ = r) ∨ (c ′ = p0 + 1 ∧ s′ = r)),
(F4 ) (l , a ∧ p0 < k) ∧ (s′ = q ∨ (c ′ = p0 + 1 ∧ s′ = r)),
(F5 ) (l = a ∧ p0 ≥ k) ∧ (s′ = q ∨ (c ′ = 0 ∧ s′ = r)), and
(F6 ) (l , a ∧ p0 ≥ k) ∧ (s′ = q).
In the for-loop on line 8, Factor F3 is processed as follows. Let the chosen substitution
2
θ at
on line 9 be {p0 + 1 7→ p1, 0 7→ p0 }. Its application followed by θ unprime generates

ψ12 : s = q ∨ (c = p0 ∧ s = r) ∨ (c = p1 ∧ s = r).
The substitution formula ϕθat2 on line 10 is p1′ = p0 + 1 ∧ p0′ = 0, and so ∆d
} 2 . The evaluation of the while-loop
gets the new transition ψ11−{l=a∧p0 <k, p1′ =p0 +1∧p0′ =0→ψ
1
would continue analogously.
⊔
⊓
In the final stage of the algorithm, when (and if) the while-loop terminates, line 12
collects the set P of all parameters used in the constructed parametric spheres of Q d
as new counters of Ad . Further, line 13 derives the new final formula by considering
all computed spheres, restricting them to valuations where the original final formula is
satisfied, and quantifying out the original counters. This way, final constraints on the
original counters get translated to constraints over parameters in P.
Example 4. In our running example, for the spheres discussed above, we would have
F(ψ 1 ) : ⊥, F(ψ11 ) : p0 = 1, and F(ψ12 ) : p0 = 1 ∨ p1 = 1.
⊔
⊓
Finally, line 14 applies the function ground on the initial formula and the transition formula of the constructed automaton before returning it. This step is needed in
order to avoid nondeterminism on unused and unconstrained counters. The function
ground conjuncts constraints of the form p = 0 with the initial formula and with the
guard of every transition for every parameter p ∈ P that is so far unconstrained in
8

<!-- page 9 -->

the concerned formula. Moreover, it will introduce a reset p′ = 0 to the assignment
formula of every transition for every counter p ∈ P that is so far not assigned on the
concerned transition. The while-loop of Algorithm 1 needs, however, not terminate, as
witnessed also by our example.7
Example 5. Continuing in Example 4, the DCA in Fig. 2 would be a part of the DCA
constructed by Algorithm 1, its states being the spheres ψ 1 , ψ11 , ψ12 from the left, but
the while-loop would not terminate, with ψ12 . Instead, it would eventually generate a
successor of ψ12 , the sphere
ψ13 : s = q ∨ (c = p0 ∧ s = r) ∨ (c = p1 ∧ s = r) ∨ (c = p2 ∧ s = r),
i.e., a sphere similar to ψ12 but extended by a new disjunct with a new parameter p2 .
Repeating this, the algorithm would keep generating larger and larger spheres with more
and more parameters.
⊔
⊓
3.2 Ensuring Termination of the Sphere Construction
In this section, we will discus reasons for possible non-termination of Algorithm 1 and
a way to tackle them. The main reason is that the algorithm may generate unboundedly
many parameters that correspond to different histories of a counter c when processing
the input word (including also impossible ones in which the counter exceeds the maximum value). The algorithm indeed “splits” a parameter appearing in a sphere into two
parameters in the successor sphere when the transitions of A update the counter in two
different ways.
In our terminating version of Algorithm 1, we build on the following: (1) distinguishing between histories that converge in the same counter value is not necessary, they can
be “merged”, and (2) the number of different reachable counter values is bounded (by
the definition of CAs). We thus enforce the invariant of every reachable configuration
of Ad that all parameters in the configuration have distinct values. The invariant is
enforced by testing equalities of parameters and merging parameters with equal values
on transitions of Ad . All transitions of Ad entering spheres with more than max A + 1
parameters can then be discarded because the invariant implies that they cannot be taken
at any configuration of Ad . Furthermore, we will also ensure that the algorithm does not
diverge because of generating semantically equivalent but syntactically different spheres
(because of different names of parameters or different formulae structure).
A terminating determinisation of CAs is obtained from Algorithm 1 by replacing
lines 9–11 by the code in Algorithm 2. In order to ensure that parameters have pairwise
distinct values, the transitions of Ad test equalities of the values assigned to parameters
and ensure that two parameters are never used to represent the same value. Different
histories of counters are thus merged if they converge into the same value. To achieve
7 For this step to preserve the language of the automaton, we need to assume that the input
CA does not assign nondeterministic values to live counters. We are refering to the standard
notion: a counter is live at a state if the value it holds at that state may influence satisfaction
of some guard in the future. Any CA can be transformed into this form, and CAs we compile
from regular expressions satisfy this condition by construction.

9

<!-- page 10 -->

Algorithm 2: Ensuring termination of sphere-based CA determinisation
foreach equivalence ∼ on at (ui ) s.t. sat(ϕ∼ ) and |at(ui )/∼ | ≤ max A + 1 do
let θ at : at (ui ) → P ′ be an injection;
18
ψi ← ui [θ at ][θ unprime ];
19
if ∃θ rename : P ↔ P ∃σ ∈ Q d : ψi [θ rename ] ⇔ σ then
′
]}
→σ to ∆d ;
20
add ψ−{gi ∧ϕ∼ [θat ],ϕθat [θrename
21
else
22
add ψ−{gi ∧ϕ∼ [θat ],ϕθat }
→ψi to ∆d ;
d
23
add ψi to Q and to Worklist;
16

17

this, Algorithm 2 enumerates all feasible equivalences of the assignment terms of ui on
line 16 and generates successor transitions for each of them separately. When deciding
whether an equivalence ∼ on the assignment terms is feasible, the algorithm performs
Ó
def Ó
two tests: (1) The formula ϕ∼ = t1 ∼t2,t1,t2 ∈at (ui ) (t1 = t2 ) ∧ t1 ≁t2,t1,t2 ∈at (ui ) (t1 , t2 )
is tested for satisfiability, meaning that the equivalence is not trying to merge terms that
can never be equal (such as, e.g., p and p + 1). (2) The number of equivalence classes
should be at most max A + 1 since this is the maximum number of different values that
the counters can reach due to the requirement that the values must be between 0 and
max A.
Line 17 builds a term assignment replacement θ at that maps all ∼-equivalent terms to
the same (future) parameter, and line 18 computes the target sphere, reflecting the given
merge. The test on line 19 checks whether the target sphere is equal to some already
generated sphere up to a parameter renaming (represented by a bijection θ rename :
P ↔ P). If so, the created sphere is discarded, and a new transition going to the
old sphere is generated on line 20; we need to rename the primed parameters used in
′
the transition’s assignment appropriately according to θ rename
= {p0′ 7→ p1′ | p0 7→
p1 ∈ θ rename }. Otherwise, a transition into the new sphere is added on line 22, and
the new sphere is added to Q d and Worklist . In both cases, the guard of the generated
transition is extended by the formula ϕ∼ [θ at ], which encodes the equivalence ∼, and
hence explicitly enforces that ∼ holds when the transition is taken.
Note that the test on the maximum number of equivalence classes can be optimised
if finer information about the maximum reachable values of the individual counters is
available. Such information can be obtained, e.g., by looking at the constants used in the
guards of the transitions where the different counters are increased. For any counter, one
should then not generate more parameters representing its possible values than what the
upper bound on that counter is (plus one).
Theorem 1. Algorithm 1 with the modification presented in Algorithm 2 terminates and
|C |
produces a DCA with L(A) = L(Ad ) and |Q d | ≤ 2 |Q |·(max A +1) .
Proof (idea). The fact that the algorithm indeed constructs a DCA is because line 7
of Algorithm 1 generates pairwise incompatible guards on transitions only. It is also
easy to show by induction on the length of the words that the language is preserved.
The termination then follows from the facts that (1) the algorithm has a bound on the
10

<!-- page 11 -->

maximum number of parameters in spheres (ensured by the condition over ∼ on line 16
of Algorithm 2) and (2) no spheres equal up to renaming are generated (ensured by the
check on line 19). The bound on the size follows from the structure of spheres.
⊔
⊓
The number of equivalences generated on line 16 of Algorithm 2 (and therefore
also the number of transitions leaving ψ) may be large. Many of them are, however,
infeasible (cannot be taken in any reachable configuration of Ad ), and could be removed.
In most cases, the majority of such infeasible transitions may be identified locally, taking
advantage of the invariant of all reachable configurations of Ad enforced by Algorithm 2:
namely, values of distinct parameters are always pairwise distinct. Therefore, before
building a transition for an equivalence ∼, we ask whether the ∼-equivalent assignment
terms may indeed be made equivalent assuming that the constructed transition guard gi
and—importantly—also the distinctness invariant hold right before the transition is
taken.ÓTechnically, we create new transitions only from those equivalences ∼ such that
sat( p1, p2 ∈Pψ , dist (p1, p2 ) (p1 , p2 ) ∧ gi ∧ ϕ∼ ) where Pψ is the set of parameters of ψ
and dist (p1, p2 ) holds iff p1 and p2 are distinct parameters.
3.3 Reachability-Restricted CA Determinisation
Above, we have described a terminating algorithm for CA determinisation. While it is
witnessed by our experiments that the algorithm often generates much smaller automata
than what could be obtained by transforming the automata into NFAs and determinising
them, a natural question is whether the generated DCA is always smaller or equal in
size to the DFA built by getting rid of the counters and using classical determinisation.
Unfortunately, the answer to this question is no. The reason is that the transformation
to a DCA needs not recognise that some generated transitions can never be executed
and that some spheres are not reachable. To see this, it is enough to imagine a transition
setting some counter c to zero and the only successor transition testing whether c is
positive. The latter transition would not be executed when generating the DFA due to
working with concrete values of counters, but it would be considered when constructing
the DCA (since the construction does not know the values of the counters).
In our experiments with CAs obtained from real-life regexes, the above was not
a problem, but we note that, for the price of an increased cost of the construction, one
could further improve the algorithm by taking into account some reachability information. In an extreme case, one could first generate the DFA corresponding to the given CA
and then use it when generating the DCA (as a hopefully more compact representation
of the DFA). In particular, whenever adding some new sphere into the DCA being built,
the algorithm can check whether there is a subset of states in the original CA represented
as a state of the DFA that is an instance of the sphere. If not, the sphere is not added.
The resulting DCA can then never be bigger than the DFA since each control state of the
DFA (i.e., a subset of states of the original CA) is represented by a single sphere only,
likewise each transition of the DFA is represented by a single transition of the DCA, and
there are not any unreachable spheres or transitions that cannot be executed.
Notice that the reachability pruning is an alternative to Algorithm 2. Algorithm 1
equipped with the reachability analysis is guaranteed to terminate. For example, when
run on the CA in Fig. 1, it would generate a DCA isomorphic to that from Fig. 2.
11

<!-- page 12 -->

4 Monadic Counting
We now provide a simplified and more efficient version of the determinisation algorithm.
The simplified version targets CAs that naturally arise from monadic regexes, i.e., regular
expressions extended with counting limited to character classes. Their abstract syntax
is
R ::= ∅ | ε | σ | R1 R2 | R1 + R2 | R∗ | σ{n, m}
where σ is a predicate denoting a set of alphabet symbols, i.e., a character class (σ will
be used to denote character classes from now on), and n, m ≥ 0 are integers. The
semantics is defined as usual, with σ{n, m} denoting a string w with n ≤ |w| ≤ m
symbols satisfying σ.
The specialised determinisation algorithm is of a high practical relevance since the
monadic class is very common, as witnessed by our experiments, where it covers over
95 % of the regexes with counting that we found (cf. Section 5).
4.1 Monadic Counting Automata
Monadic regexes can be easily compiled to nondeterministic monadic CAs satisfying certain structural properties summarised below.8 In particular, a (nondeterministic) monadic
counting automaton (MCA) is a CA A = (Q, C, I, F, ∆) where the following holds:
1. The set Q of control states is partitioned into a set of simple states Q s and a set of
counting states Q c , i.e., Q = Q s ⊎ Q c .
2. The set of counters C = {cq | q ∈ Q c } consists of a unique counter cq for every
counting state q ∈ Q c .
3. All transitions containing counter guards or updates must be incident with a counting state in the following manner. Every counting state q ∈ Q c has a single increment
→q with the value of cq limited by the bound
transition, a self-loop q−{σ∧cq <maxq ,cq′ =cq +1 }
→q, which set
maxq of q, and possibly several entry transitions of the form r−{ σ̄∧cq′ =0 }
cq to 0. As for exit transitions, every counting state is either exact or range, where exact
counting states have exit transitions of the form q−{σ∧cq =maxq }
→s, and range counting
→s with s ∈ Q s.t. s , q. That is, an
states have exit transitions of the form q−{σ,⊤}
exact counting state may be left only after exactly maxq repetitions of the incrementing
transition (it corresponds to a regular expression σ{k}), while a range counting state
may be left sooner (it corresponds to a regular expression σ{0,k}). We denote the set
of range counting states Qr and the set of exactÔ
counting states Ô
Q e , with Q c = Qr ⊎ Q e .
4. The initial condition I is of the form I : q ∈QsI s = q ∨ q ∈QcI (s = q ∧ cq = 0)
for some sets of initial simple and counting states Q sI ⊆ Q s and Q cI ⊆ Q c , respectively,
with the counters of initial counting states initialised to 0.
Ô
Ô
5. The final condition F is of the form F : q ∈QsF ∪QrF s = q ∨ q ∈QeF (s = q ∧ cq =
maxq ) where Q sF ⊆ Q s is a set of simple final states, QrF ⊆ Qr is a set of final range
counting states, and Q eF ⊆ Q e is a set of final exact counting states. That is, final
conditions on final states are the same as counter conditions on exit transitions.9
8 We note that we restrict ourselves to range sub-expressions of the form σ{n, n} or σ{0, n} only.
This is without loss of generality since a general range expression σ{m, n} can be rewritten as
σ{m, m}.σ{0, n − m}.
9 Notice that the guards cq < maxq on the incrementing self-loops of exact counting states could
be removed without affecting the language since when cq exceeds maxq , then the run can never

12

<!-- page 13 -->

4.2 Determinisation of MCAs
Algorithm 2 can be simplified when specialised to monadic CAs. The simplification is
based on the following observations. Observation 1. Counters are dead outside their
states. To simplify the representation of spheres, we use the fact that every counter cq of
an MCA is “active” in the state q only, while cq is “dead” in other states (i.e., its current
value has no influence on runs of the MCA that are not in q). To represent different
variants of cq , we use parameters of the form cq [i] obtained by indexing cq by an index
i, for 0 ≤ i ≤ maxq , while enforcing the invariant that, for distinct indices i and j, cq [i]
and cq [ j] always have different values. Since the value of cq ranges from 0 to maxq ,
at most maxq + 1 variants of cq are needed.10 Since spheres only need parameters to
remember values of live counters, every sphere can be equivalently written in the normal
form

Ü
Ü
Ü 
def

s=q∧

s=q ∨

ψ=

q ∈Q′s

cq = cq [i]

0 ≤i ≤max′q

q ∈Q′c

for some Q s′ ⊆ Q s , Q c′ ⊆ Q c , and maxq′ ≤ maxq . That is, a sphere ψ records which
states may be reached in the original MCA when ψ is reached in the determinised MCA
and also which variants of the counter cq may record the value of cq when q is reached.
Observation 2. Variants of exact counting states can be sorted. For dealing with
any exact counting state q ∈ Q e , we may use the following facts: (1) If executed,
the increment transition of q increments all variants of cq whose values are smaller
than maxq . (2) New variants of cq are initialised to 0 by the entry transitions. (3) Variants
whose value is maxq can take an exit transition, after which they become dead and
their values do not need to be propagated to the next configuration. It is therefore
easy to enforce that the values of the variants cq [i] stay sorted, so that i < j implies
α(cq [i]) < α(cq [ j]) in every configuration α of Ad . The sortedness invariant implies
that the variant of cq with the highest index, called highest variant, has the highest value.
This, together with the invariant of boundedness by maxq and mutual distinctness of
values of variants of cq , means that the highest variant is the only one that may satisfy
the tests cq = maxq on exit transitions or fail the test cq < maxq on the incrementing
transition. Hence, the deterministic MCA does not need to test all variants of cq but the
highest one only.
Observation 3. Only the smallest variants of range counting states are important.
For range counting states, we adapt the simulation pruning technique from [10]. The
technique optimizes the standard subset-construction-based determinisation of NFAs by
exploiting a simulation relation [7] such that any macrostate (which has the form of
a set of states of the original NFA) obtained during the determinisation can be pruned by
removing those NFA states that are simulated by other NFA states included in the same
macrostate. The pruning does not change the language: the resulting DFA is bisimilar to
the one constructed without pruning. For our DCA construction, we use the simulation
that implicitly exists between configurations α and α ′ of A with the same range counting
leave q and has thus no chance of accepting. We include these guards only to conform to the
condition on boundedness of counter values in the definition of CAs.
10 Notice that maintaining a fixed association of a parameter to a counter is a difference from
Algorithms 1 and 2, where one parameter may represent different counters.

13

<!-- page 14 -->

state q = α(s) = α ′ (s), where α(cq ) ≥ α ′(cq ) implies that α ′ simulates α.11 Hence,
the spheres only need to remember the smallest possible counter value for every range
counting state q, which may be always stored in cq [0], and discard all other variants.
Determinisation of MCAs. Observations 1–3 above allow for representing spheres using
a simple data structure, namely, a multiset of states. By a slight abuse of notation, we
use ψ for the sphere itself as well as for its multiset representation ψ : Q → N. The fact
that ψ(q) > 0 means that q is present in the sphere (i.e., s = q is a predicate in the normal
form of ψ), and for a counting state q, the counters cq [0], . . . , cq [ψ(q) − 1] are the ψ(q)
variants of cq tracked in the sphere (i.e., ψ(q) − 1 = maxq′ in the normal form of ψ).
The MCA determinisation is then an analogy of Algorithm 1 that uses the multiset
data structure and preserves the sortedness and uniqueness of variants of exact counters.
The initial sphere ψI assigns 1 to all initial states of I, and the initial configuration
I d assigns 0 to cq [0] for each counting state q in I. Further, we modify the part of
Algorithm 1 after popping a sphere ψ from Worklist in the main loop (lines 7–11).
Let ∆ψ denote the set of transitions of A originating from states q with ψ(q) > 0. Processing of ψ starts by removing guard predicates of the form cq < maxq from increment
transitions of exact counting states in ∆ψ (since they have no semantic effect as mentioned already above). Subsequently, we compute minterms of the set of guard formulae
′
}
of Ad .
of the transitions in ∆ψ . Each minterm µ then gives rise to a transition ψ−{g, f→ψ
′
The guard formula g, assignment formula f , and the target sphere ψ are constructed as
follows.
First, the guard g is obtained from the minterm µ by replacing, for all q ∈ Q c , every
occurrence of cq by cq [ψ(q)], i.e., the highest variant of cq . Intuitively, the counter
guards of transitions of ∆ψ present in µ will on the constructed transition of Ad be
testing the highest variants of the counters. This is justified since (a) only the highest
variant of cq needs to be tested for exact counting states, as concluded in Observation
2 above, and (b) we keep only a single variant of cq for range counting states (which is
also the highest one), as concluded in Observation 3.
We then initialise the target multiset ψ ′ as the empty multiset {q 7→ 0 | q ∈ Q}
and collect the set ∆µ of all transitions from ∆ψ that are compatible with the minterm µ
(recall that increment self-loops of exact states in ∆ψ have counter guards removed,
hence counter guards do not influence their inclusion in ∆µ ). The transitions of ∆µ will
be processed in the following three steps.
Step 1 (simple states). Simple states with an incoming transition in ∆µ get ψ ′ (q) = 1.
Step 2 (increment self-loops). For exact states with the increment self-loop in ∆µ ,
ψ ′ (q) is set to ψ(q) − 1 if an exit transition of q is in ∆µ , and to ψ(q) otherwise. Indeed, if
(and only if) an exit transition of q is included in ∆µ , and ∆µ is enabled in some sphere,
then the highest variant of cq has reached maxq in that sphere, and the self-loop cannot
be taken by the highest variant of cq . The lower variants of cq always have values smaller
than maxq , and hence can take the self-loop. The assignment f then gets the conjunct
cq [i]′ = cq [i] + 1 for each 0 ≤ i < ψ ′ (q) since the variants that take the self-loop are
11 The fact that this relation is indeed a simulation can be seen from that both the higher and lower
value of cq can use any exit transition of q at any moment regardless of the value of cq , but the
lower value of cq can stay in the counting loop longer.

14

<!-- page 15 -->

incremented. For range states with the increment self-loop in ∆µ , we set ψ ′ (q) to 1, and
cq [0]′ = cq [0] + 1 is added to f (only one variant is remembered).
Step 3 (entry transitions). For each counting state q with an entry transition in ∆µ ,
ψ ′ (q) is incremented by 1 and the assignment cq [0]′ = 0 of the fresh variant of cq
is added to f . If the new value of ψ ′ (q) exceeds maxq + 1, then the whole transition
generated from µ is discarded, since cq cannot have more than maxq + 1 distinct values.
Otherwise, if q is an exact counting state, then f is updated to preserve the invariant of
sorted and unique values of cq : the increments of older variants of cq are right-shifted
to make space for the fresh variant, meaning that each conjunct cq [i]′ = cq [i] + 1 in f
is replaced by cq [i + 1]′ = cq [i] + 1. If q ∈ Qr , then if the assignment cq [0]′ = cq [0] + 1
is present in f , it is removed (as the fresh variant has the smallest value 0).
Example 6. Determinising the CA from Fig. 1 using the algorithm described in this
section would result in the DCA shown in Fig. 2.
⊔
⊓
The monadic determinisation has a much lower worst-case complexity than the
general algorithm. Importantly, the number of states depends on maxA only polynomially,
which is a major difference from the exponential bounds of the naïve determinisation
and our general construction.
Theorem 2. The specialised monadic CA determinisation constructs a DCA with |Q d | ≤
(max A + 1) |Q | and |∆d | ≤ |Σ| · (4 · (max A + 1)) |Q | .
Proof (idea). The bound on the number of states is given by the number of functions
Q → {0, . . . , max A }. The bound on the number of transitions is given by the fact, that
if a sphere multiset maps a state q to n, then the successors of the sphere can map q
to 0 (when q is not a successor), n − 1, n, or n + 1. Therefore, for every symbol from Σ
and every macrostate from at most (max A + 1) |Q | many of them, there are at most 4 |Q |
successors, and |Σ| · (max A + 1) |Q | · 4 |Q | = (4 · (max A + 1)) |Q | .
⊔
⊓

5 Experimental Evaluation
The main purpose of our experimentation was to compare the proposed approach with
the naïve determinisation and confirm that our method produces significantly smaller
automata and mitigates the risk of the state space explosion causing a complete failure of
determinisation (and the implied impossibility to use the desired deterministic automaton
for the intended application, such as pattern matching). To this end, we extended the
Microsoft’s Automata library [18] with a prototype support for CAs, implemented the
algorithm from Section 4 (denoted Counting in the following), and compared it to
the standard determinisation already present in the library (denoted as DFA). For the
evaluation, we collected 2,361 regexes from a wide range of applications—namely,
those used in network intrusion detection systems (Snort [17]: 741 regexes, Yang [29]:
228 regexes, Bro [21]: 417 regexes, HomeBrewed [28]: 55 regexes), the Microsoft’s
security leak scanning system (Industrial: 17 regexes), the Sagan log analysis engine
(Sagan [26]: 14 regexes), and the pattern matching rules from RegExLib (RegExLib [20]:
889 regexes). We only selected regexes that contain an occurrence of the counting
operator, and from these, we selected only monadic ones (there were over 95 % of
15

<!-- page 16 -->

DFA

100
10

1000 100000

1

DFA

10000

them, confirming the fragment’s importance). All benchmarks were run on a Xeon E52620v2@2.4GHz CPU with 32 GiB RAM with a timeout of 1 min (we take the mean time
of 10 runs). In the following, we use µ, m, and σ to denote the statistical indicators mean,
median, and standard deviation, respectively. All times are reported in milliseconds.
The number of timeouts was 110 for Counting,
and 238 for DFA. The two methods were to some
degree complementary, there were only 62 cases in
which both timed out. This confirms that our algorithm indeed mitigates the risk of failure due to state
space explosion in determinisation. The remaining
comparisons are done only with respect to benchmarks for which neither of the methods timed out.
In Fig. 3, we compare the running times of the
conversion of an NFA for a given regex to a DFA (the
1
100
10000
DFA axis) and the determinisation of the CA for the
Counting
same regex (the Counting axis). If we exclude the
Fig. 3. Comparison of running times
easy cases where both approaches finished within
given in ms (the axes are logarithmic).
1 ms, we can see that Counting is almost always
better than DFA. Note that the axes are logarithmic, so the advantage of Counting over
DFA grows exponentially wrt the distance of the data point from the diagonal. The
statistical indicators for the running times are µ = 110, m = 0.17, σ = 1, 177 for DFA
and µ = 0.23, m = 0.13, σ = 0.09 for Counting.
In Fig. 4, we compare the number of states of
the results of the determinisation algorithms (DCA
for Counting and DFA for DFA). Also here, Counting significantly dominates DFA. The statistical indicators for the numbers of states are µ = 4, 543,
m = 41, σ = 57, 543 for DFA and µ = 241, m = 13,
σ = 800 for Counting. To better evaluate the conciseness of using DCAs, we further selected 184
benchmarks that suffered from state explosion during determinisation (our criterion for the selection
10
1000 100000
was that the number of states increased at least tenCounting
fold in DFA) and explored how the CA model can
be used to mitigate the explosion. Fig. 5 shows his- Fig. 4. Comparison of numbers of
tograms of how DCAs were more compact than states (the axes are logarithmic).
DFAs and also how much the number of counters rose during the determinisation. From
the histograms, we can see that there are indeed many cases where the use of DCAs
allows one to use a significantly more compact representation, in some cases by the factor of hundreds, thousands, or even tens of thousands. Furthermore, the other histogram
shows that, in many cases, no blow-up in the number of counters happened; though there
are also cases where the number of counters increased by the factor of hundreds.
In terms of numbers of transitions, the methods compare similarly as for numbers of
states, as shown in Fig. 6. We obtained µ = 14, 282, m = 77, σ = 213, 406 for DFA and
µ = 2, 398, m = 23, σ = 8, 475 for Counting. (We emphasize the number of states over
16

<!-- page 17 -->

50
40

20

30
20

10

10
0

0
1

10

100

1000

10000

1

10

a) |DFA| / |DCA|

100

b) counter explosion

Fig. 5. Histograms of (a) the ratio of the number of states of a DFA and of the corresponding DCA
(i.e., a bar at value x of a height h denotes that the size of the DCA was h times around x times
smaller than the size of the corresponding DFA) and (b) the ratio of the number of counters used
by a CA after and before determinisation. Note that the x-axes are logarithmic in both cases.

1000 100000
10

Unfolding to DFA

the number of transitions in our comparisons since the performance and complexity of
automata algorithms is usually more sensitive to the number of states, and large numbers
of transitions are amenable for efficient symbolic representations [12,6,16].)
Benefits of the Counting method were the most
substantial on the Industrial dataset. For the regex
".*A[^AB]{0,800}C[D-G]{43,53}DFG[^D-H]"
(which was obtained from the real one, which
is confidential, by substituting the used character
classes by characters A–H), the obtained DFA contains 200,132 states, while the DCA contains only
12 states (and 2 counters), which is 16,667 times
less. When minimised, the DFA still has 65,193
states. There were other regexes where Counting
10
1000 100000
achieved a great reduction, in total two regexes had
Counting
a reduction of over 10,000, three more regexes had
a reduction of over 1,000, and 45 more had a reduc- Fig. 6. Comparison of numbers of
transitions (the axes are logarithmic).
tion of over 100.
Additionally, we also compared our approach against the naïve determinisation followed by the standard minimisation. Due to the space restrictions and since minimisation
is not relevant to our primary target (preventing failure due to state space explosion during determinisation), we present the results only briefly. Minimisation increased the
running times of DFA by about one half (µ = 150, m = 0.35, σ = 1, 582 for the running
times of DFA followed by minimisation). The minimal DFAs were on average about
ten times smaller than the original DFAs, and about ten times larger than our DCAs
(µ = 385, m = 29, σ = 4, 195 for the numbers of states of the minimal DFAs).

6 Related Work
Our notion of CAs is close to the definition of FACs in [13], but our CAs are more general,
by allowing input predicates and more complex counter updates. Also R-automata [1]
17

<!-- page 18 -->

are related but somewhat orthogonal to CAs because counters in R-automata do not
need to have upper bounds and cannot be tested or compared. Counter systems are
also related to CAs but allow more general operations over counters through Presburger
formulas [2]. CAs can also be seen as a special case of extended finite state machines or
EFSMs [5,22,24,25], but these already go beyond regular languages.
Extended FAs (XFAs) augment classical automata with so-called scratch memory
of bits and bit-instructions [23,24], which can represent counters and also reduce nondeterminism. Regexes are compiled into deterministic XFAs by first using an extended
version of Thompson’s algorithm [27], then determinised through an extended version
of the classical powerset construction, and finally minimised. Although a small XFA
may exist, the determinisation algorithm incurs an intermediate exponential blowup of
search space for inputs such as .*a.{k} (cf. [23, Section 6.2]), i.e., the regex from our
running example, and handling of such cases remained an open problem.
Regular expressions with counters are also discussed in [13,15,8]. The automata with
counters used in [13], called FACs, correspond closely, apart from our symbolic character
predicates and transition representation, to the class of CAs considered in our work. A
central result in [13] is that for counter-1-unambiguous regexes, the translation algorithm
yields deterministic FACs and that checking determinism of FACs can be done in
polynomial time. There are also works on regular expressions with counting that translate
deterministic regexes to CAs and work with different notions of determinism [9,4]. The
related work in [14] studies membership in regexes with counting. None of these papers
addresses the problem of determinising nondeterministic CAs.

7 Future Directions
Among future directions, we will consider optimisations of the current algorithm by
means of avoiding construction of unreachable parts of DCAs or by finding efficient data structures, generalising the techniques used for monadic CAs to a larger
class of CAs, and building a competitive pattern matching engine around the current algorithm. Since we believe that CAs have a lot of potential as a general succinct automata representation, we will work towards filling in efficient CA counterparts of standard automata algorithms, such as Boolean operations, minimisation, or
emptiness test, that could also be used in other applications than pattern matching,
such as verification and decision procedures of logics.

References
1. Abdulla, P.A., Krčál, P., Yi, W.: R-Automata. In: Proc. of CONCUR’08. LNCS, vol. 5201.
Springer (2008)
2. Bardin, S., Finkel, A., Leroux, J., Petrucci, L.: FAST: Acceleration from Theory to Practice.
STTT 10(5) (2008)
3. Börklund, E., Martens, W., Timm, T.: Efficient Incremental Evaluation of Succinct Regular
Expressions. In: Proc. of CIKM’15. ACM (2015)
4. Chen, H., Lu, P.: Checking Determinism of Regular Expressions with Counting. Information
and Computation 241 (2015)

18

<!-- page 19 -->

5. Cheng, K., Krishnakumar, A.S.: Automatic Functional Test Generation Using the Extended
Finite State Machine Model. In: Proc. of DAC’93. ACM Press (1993)
6. D’Antoni, L., Veanes, M.: Minimization of Symbolic Automata. In: Proc. of POPL’14. ACM
(2014)
7. Dill, D.L., Hu, A.J., Wong-Toi, H.: Checking for language inclusion using simulation preorders. In: Proc. of CAV’91. LNCS, vol. 575. Springer (1992)
8. Gelade, W., Martens, W., Neven, F.: Optimizing Schema Languages for XML: Numerical
Constraints and Interleaving. In: Proc. of ICDT’07. LNCS, vol. 4353. Springer (2007)
9. Gelade, W., Gyssens, M., Martens, W.: Regular Expressions with Counting: Weak versus
Strong Determinism. In: Proc. of MFCS’09. LNCS, vol. 5734. Springer (2009)
10. van Glabbeek, R., Ploeger, B.: Five Determinisation Algorithms. In: Proc. of CIAA’08. LNCS,
vol. 5148. Springer (2008)
11. Heizmann, M., Hoenicke, J., Podelski, A.: Software Model Checking for People Who Love
Automata. In: Proc. of CAV’13. LNCS, vol. 8044. Springer (2013)
12. Henriksen, J., Jensen, J., Jørgensen, M., Klarlund, N., Paige, B., Rauhe, T., Sandholm, A.:
Mona: Monadic second-order logic in practice. In: TACAS’95. LNCS, vol. 1019. Springer
(1995)
13. Hovland, D.: Regular Expressions with Numerical Constraints and Automata with Counters.
In: Proc. of ICTAC’09. LNCS, vol. 5684. Springer (2009)
14. Hovland, D.: The Membership Problem for Regular Expressions with Unordered Concatenation and Numerical Constraints. In: Proc. of LATA’12. LNCS, vol. 7183. Springer (2012)
15. Kilpeläinen, P., Tuhkanen, R.: One-unambiguity of Regular Expressions with Numeric Occurrence Indicators. Information and Computation 205(6) (2007)
16. Lengál, O., Šimáček, J., Vojnar, T.: VATA: A library for efficient manipulation of nondeterministic tree automata. In: TACAS’12. LNCS, vol. 7214. Springer (2012)
17. M. Roesch et al.: Snort: A Network Intrusion Detection and Prevention System,
http://www.snort.org
18. Microsoft Automata library: Automata and Transducer Library for .NET,
https://github.com/AutomataDotNet/Automata
19. OWASP Foundation and Checkmarx: Regular Expression Denial of Service: ReDoS (2017)
20. RegExLib.com: The Internet’s First Regular Expression Library, http://regexlib.com/
21. Robin Sommer et al.: The Bro Network Security Monitor, http://www.bro.org
22. Shiple, T.R., Kukula, J.H., Ranjan, R.K.: A Comparison of Presburger Engines for EFSM
Reachability. In: Proc. of CAV’98. LNCS, vol. 1427. Springer (1998)
23. Smith, R., Estan, C., Jha, S.: XFA: Faster Signature Matching with Extended Automata. In:
Proc. of SSP’08. IEEE (2008)
24. Smith, R., Estan, C., Jha, S., Siahaan, I.: Fast Signature Matching Using Extended Finite
Automaton (XFA). In: Proc. of ICISS’08. LNCS, vol. 5352. Springer (2008)
25. Sperberg-McQueen, M.: Notes on Finite State Automata with Counters,
https://www.w3.org/XML/2004/05/msm-cfa.html, accessed: 2018-08-08
26. The
Sagan
team:
The
Sagan
Log
Analysis
Engine,
https://quadrantsec.com/sagan_log_analysis_engine/
27. Thompson, K.: Programming Techniques: Regular Expression Search Algorithm. Communications of the ACM 11(6) (1968)
28. Češka, M., Havlena, V., Holík, L., Lengál, O., Vojnar, T.: Approximate Reduction of Finite
Automata for High-Speed Network Intrusion Detection. In: TACAS’18. LNCS, vol. 10806.
Springer (2018)
29. Yang, L., Karim, R., Ganapathy, V., Smith, R.: Improving NFA-Based Signature Matching
Using Ordered Binary Decision Diagrams. In: RAID’10. LNCS, vol. 6307. Springer (2010)

19
