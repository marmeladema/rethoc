---
file: ../pdfs/2026-ncma-berglund-sparse-counting-sets.pdf
citation: "Efficient Regex Matching with Sparse Counting-Sets — Berglund, van der Merwe, Sung — NCMA 2026 (EPTCS 446)"
doi: 
source: "https://arxiv.org/pdf/2606.26679"
topics: [counting_and_derivatives]
pages: 18
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Efficient Regex Matching with Sparse Counting-Sets
Martin Berglund

Brink van der Merwe

Sicheol Sung*

Umeå University
Umeå, Sweden
mbe@cs.umu.se

Stellenbosch University
Stellenbosch, South Africa
abvdm@sun.ac.za

Yonsei University
Seoul, Republic of Korea
sicheol.sung@yonsei.ac.kr

Regular expressions with counting operations (c-regexes) offer a compact representation of repeating
patterns by allowing numerical bounds to be added to subexpressions. Recent work introduced the
counting-set data structure, which allows simultaneous updates of multiple counter values for efficient
matching. However, this approach suffers from a performance bottleneck when counting-sets must be
replicated due to the presence of branching transitions. We propose a sparse counting-set approach,
which reduces the replication overhead by maintaining only essential counter values, thereby yielding
a more efficient matching algorithm.

1

Introduction

Counting operations make it possible to express bounded repetition, indicating that something happens
at least l times but no more than h times. Matching strings using regular expressions with counting
operations (c-regexes) can be slow due to the additional complexity introduced by counting operations.
Two typical approaches are used for matching with c-regexes. The first is counter-expansion, employed
by RE2 [3], which rewrites c-regexes into equivalent regexes without counting. However, this approach
becomes inefficient when the bounds of counting operations are large, as it constructs duplicate states for
each possible counter value [13]. The second approach uses counter automata (CAs) [6]; for example,
variants of it are used in the Java regex engine. This approach tracks the number of repetitions using
counter variables, but in the Java matcher, this approach inherits the inefficiencies of backtracking
matchers [4, 12].
During matching, a CA constructed from a c-regex r may generate a number of counting configurations,
linear in the bounds of the counting operators in r. Thus, the worst-case matching time of Thompson-like
matching algorithms, that is, matching algorithms following a breadth-first approach, on CAs depends on
the bounds of the counting operators, which can be large. Since the bounds are in practice encoded in
decimal (as opposed to unary), a counter range {l, h} contributes only O(log h) characters to the textual
length of r, yet may induce Θ(h) distinct counter configurations during matching. Consequently, the
matching time can be exponential in the textual length of r.
The counting-set data structure [8, 13] has been proposed to address this inefficiency by allowing the
simultaneous application of the increment-by-one counter operation on multiple counter values. However,
the cost of replicating counting-sets at branching transitions is expensive. In this work, we address this
problem by introducing a sparse counting-set structure, which reduces the size of counting-sets by keeping
track of only essential counter values. In summary, our contribution is not a new automaton model, but
a sparse representation for counting-sets with preserved acceptance behavior and improved bounds on
d-sparse flat c-regexes.
* Authors are listed alphabetically. While this work represents a collaborative effort, Sicheol Sung served as the lead author.

C. Câmpeanu, M. Kutrib, S. Lombardy (Eds.)
Non-Classical Models of Automata and Applications (NCMA 2026)
EPTCS 446, 2026, pp. 19–36, doi:10.4204/EPTCS.446.2

© M. Berglund, B. van der Merwe and S. Sung
This work is licensed under the
Creative Commons Attribution License.

<!-- page 2 -->

20

Efficient Matching with Sparse Counting-Sets

Related work. Compact handling of bounded repetition in pattern matching has a long history. XFA [10]
augments finite-state automata with auxiliary variables, including counters, and is motivated by intrusion
detection signatures with heavily repeating subpatterns. Björklund et al. [2] study the incremental
evaluation of succinct regular expressions with numerical constraints, giving bounds for maintaining
matches under edits. In the non-backtracking matching literature closer to our setting, counter automata [6,
13], counting-set automata [8, 13], and bit-vector automata [7] each trade expressiveness for efficiency
differently. Our work stays within the counting-set framework of [8, 13] and targets its main remaining
bottleneck: the cost of replicating large counting-sets at branching transitions, which we eliminate through
a sparse representation.

2

Preliminaries

By N we denote the set of positive integers and let N0 and N∞ be N union the singleton sets {0} and {∞},
respectively. We assume that n < ∞ for all n ∈ N0 . If M, N ⊆ N0 and k ∈ N0 , then k ⊖ M, N ⊖ k and
M ⊖ N denote {k ⊖ m | m ∈ M}, {n ⊖ k | n ∈ N} and {m ⊖ n | m ∈ M, n ∈ N}, respectively, for ⊖ any
binary operator on N0 . For l ∈ N0 and h ∈ N∞ , with l ≤ h, we denote by [l, h] the interval given by
{n ∈ N0 | l ≤ n ≤ h}. Thus, to simplify our presentation, we adopt the convention of denoting all integers
greater than or equal to l, as [l, ∞], instead of [l, ∞).
We denote a partial function f from a set X to a set Y by f : X ⇀Y . A partial function f : X ⇀Y is
often represented as {x : f (x)}x∈X . For example, {a : x, b : y} denotes a function f such that f (a) = x,
f (b) = y and f (x) is undefined for all other x ∈ X. A relation → on sets A and B is functional if for a ∈ A
there exists at most one b ∈ B such that a → b.
An alphabet A is a set of characters, and a string w = a1 a2 · · · an over A is a finite sequence of characters
with ai ∈ A. The length |w| of the string w is the number n of characters in w. Similar notation is used to
denote the cardinality of a finite set B, that is, |B|, but given notational conventions, this will not create
confusion. The empty string of length zero is denoted by ε. For strings u = a1 a2 · · · an and v = b1 b2 · · · bm ,
their concatenation u · v is the string a1 a2 · · · an b1 b2 · · · bm . Languages over the alphabet A are sets of
strings over A. For two languages L1 and L2 , their concatenation L1 · L2 is defined as {uv | u ∈ L1 , v ∈ L2 }.
For a language L, we define L0 = {ε} and Li+1 = Li · L for all i ≥ 0. The Kleene star and plus of L are
S
S
defined by L∗ = i≥0 Li and L+ = i≥1 Li , respectively. Thus, L∗ = L+ ∪ {ε}.

2.1

Symbolic regular expressions with counter operators

In practice, regexes extensively use character classes, such as \d, which are equivalent to (0|1|· · · |9)
in Perl-compatible regular expression syntax. That is, \d is a predicate matching a set of characters. To
harmonize with this, we operate on symbolic regular expressions in which counter operators might also be
present. Readers more comfortable with ordinary regular expressions over an alphabet A may, throughout,
read each predicate σ simply as a single character of A; the algorithmic content does not depend on the
predicate machinery beyond the membership test a ∈ [[σ ]]. We begin with the definition of a Boolean
algebra—see [1] for more details.
Definition 1. An effective Boolean algebra is a tuple (A, Σ, [[_]], ⊥, ⊤, ∨, ∧, ¬), where: (1) A is the
domain; (2) Σ is a set of predicates over A that is closed under the Boolean connectives ∨, ∧ and ¬,
with ⊥, ⊤ ∈ Σ; (3) [[_]] : Σ → 2A is a denotation function such that [[⊥]] = /0 and [[⊤]] = A, and for ϕ, ψ ∈ Σ,
[[ψ ∨ ϕ]] = [[ψ]] ∪ [[ϕ]], [[ψ ∧ ϕ]] = [[ψ]] ∩ [[ϕ]], and [[¬ϕ]] = A \ [[ϕ]].

<!-- page 3 -->

M. Berglund, B. van der Merwe and S. Sung

21

When the domain A is an alphabet, each predicate σ defines the set [[σ ]] ⊆ A of symbols satisfying the
predicate, which are the character classes of practical regular expressions.
Definition 2. A symbolic regular expression with counting operations (c-regex) over Σ is defined inductively as follows: (1) ε and σ ∈ Σ are c-regexes; (2) r1 · r2 and r1 | r2 are c-regexes for c-regexes r1 and r2 ;
(3) r∗ , r+ and r? are c-regexes for a c-regex r; and (4) for a c-regex r and l ∈ N0 and h ∈ N∞ , with l ≤ h,
we also have that r{l,h} is a c-regex.
By L(r) we denote the language associated with a c-regex r, which is defined inductively as follows.
We let σ ∈ Σ and r1 and r2 be c-regexes. Then, we define (1) L(ε) = {ε} and L(σ ) = [[σ ]]; (2) L(r1 · r2 ) =
L(r1 ) · L(r2 ) and L(r1 |r2 ) = L(r1 ) ∪ L(r2 ); (3) L(r1∗ ) = L(r1 )∗ , L(r1+ ) = L(r1 )+ and L(r1? ) = L(r1 ) ∪ {ε};
S
{l,h}
and (4) L(r1 ) = i∈[l,h] (L(r1 ))i .
We use |r|Σ to denote the number of occurrences of predicates in r. The counting-height of the
c-regex r is the maximum depth of nested counting operators in r. If r contains no counter operations, we
define the counting-height of r as 0. A c-regex is flat if the counting-height of the regex is at most 1. Let
H(r) denote the largest finite bound in a counter range in r (i.e. every subexpression s{l,h} has l ≤ H(r)
and, if h ̸= ∞, then h ≤ H(r)).
We use a typewriter font to denote specific c-regex instances for readability. For example, we write
‘(a|bc){2,10}’ instead of ‘(a|bc){2,10} ’. Furthermore, although we may avoid infinite counter ranges by
rewriting r{l,∞} as r{l,l} r∗ , we will see in Section 3 that this rewrite harms matching performance when
using sparse counting-sets.

2.2

Counter automata

Counter automata (CAs) are an extension of nondeterministic finite-state automata designed to match
c-regexes [9, 13]. Our definition closely aligns with the formalism presented by Turonova et al.[13].
Let C = {c1 , c2 , . . . , ck } be a finite set of counter variables. A function α : C → N0 is called an
assignment for C, and the set of all assignments for C is denoted as Assign(C). A guard over C is any
function ψ : ci ∈ C 7→ ψi , where ψi is a Boolean combination of predicates of the form ⊤ (true), ⊥ (false),
ci ≤ h and l ≤ ci with l ∈ N0 and h ∈ N∞ (note, the predicate ci ≤ ∞ is always satisfied). An assignment α
satisfies ψ if, for every counter variable ci , the boolean expression ψi (ni ), which is obtained from ψi by
substituting ci with its value ni = α(ci ) in ψi , evaluates to true; we denote this by α |= ψ. The set of all
guards over C is denoted by Guard(C).
An action over C is a function θ : ci 7→ θi (ci ), where θi ∈ {nop, reset, inc, inact}, and (1) nop : n 7→ n,
(2) reset : n 7→ 1, (3) inc : n 7→ n + 1, and (4) inact : n 7→ 0. We denote by [[α]]θ the assignment obtained
by applying θ to α. Thus, [[α]]θ (ci ) := θi (α(ci )). The set of all actions over C is denoted as Action(C).
Example 3. Consider the assignment α = {c1 : 2, c2 : 3}. We illustrate guards and actions using the
following example.
Guards: For guards ψ = {c1 : c1 ≤ 2, c2 : c2 ≥ 3} and ψ ′ = {c1 : c1 ≥ 2, c2 : c2 ≥ 4}, we have that α |= ψ
and α ̸|= ψ ′ . We implicitly conjoin the predicates in a guard; that is, we interpret a comma as a
conjunction ∧.

Actions: For actions θ = {c1 : nop, c2 : reset} and θ ′ = {c1 : inc, c2 : inact}, applying them to α results
in [[α]]θ = {c1 : 2, c2 : 1} and [[α]]θ ′ = {c1 : 3, c2 : 0}.
Definition 4. A nondeterministic symbolic counter automaton (CA) is a tuple M = (Q, Σ,C, qinit , ∆, F),
where (1) Q is a finite set of states; (2) Σ is a finite set of predicates over an effective Boolean algebra; (3) C
is a finite set of counter variables; (4) qinit is the initial state; (5) ∆ ⊆ Q × Σ × Guard(C) × Action(C) × Q
is the set of transitions; and (6) F : Q → Guard(C) is the acceptance guard.

<!-- page 4 -->

22

Efficient Matching with Sparse Counting-Sets
a; c : c < 100, inc
qinit

a; c : ⊤, reset

a; c : ⊤, nop

a; c : ⊤, nop

q1

q2

a; c : c < 100, inc

c:2≤c

c:2≤c
Figure 1: A CA equivalent to (aa*){2,100} with a single counter variable c.
It may be useful to look ahead to Example 5 with Figure 1 for a CA as we complete the definitions.
For a string w, we define when a CA M accepts w. A configuration (config) of M is a pair (q, α) ∈
Q × Assign(C). We define a transition relation →(M,a) , for a character a ∈ A, over configs, as follows:
(p, α) →(M,a) (q, β ) ⇐⇒ ∃(p, σ , ψ, θ , q) ∈ ∆ : α |= ψ ∧ a ∈ [[σ ]] ∧ β = [[α]]θ .
Then, for a string w = a1 a2 · · · an ∈ A∗ with ai ∈ A, we define →(M,w) as follows:
(q0 , α0 ) →(M,w) (qn , αn ) ⇐⇒ (q0 , α0 ) →(M,a1 ) (q1 , α1 ) →(M,a2 ) · · · →(M,an ) (qn , αn ).
We write →w for →(M,w) when M is clear from the context.
The initial config of M is (qinit , αinit ), where αinit (ci ) = 0 for all ci ∈ C, and final configs of M are
configs (q, α) satisfying α |= F(q). We call a config (q, α) reachable if (qinit , αinit ) →w (q, α) for some
string w ∈ A∗ . If the config (q, α) is final, then we say M accepts w. We define L(M) := {w ∈ A∗ |
M accepts w}.
Example 5. Consider a c-regex r = (aa*){2,100}. In reality this is equivalent to the expression aaa*,
but let us work out the counter use. For the CA M in Figure 1, L(r) = L(M), that is, M is equivalent to r.
The initial state always reads an a and goes to q1 , which corresponds to being ‘at’ the a∗ in the expression.
There the options are: reading another a by using the Kleene star (going to q2 without changing the count
c); reading another a by reentering the counter operation (incrementing c, only possible if c < 100); or
accepting (possible only if c ≥ 2). Note that aaa ∈ L(r). The following are the sequences of transition
relations for (qinit , {c : 0}) →aaa (q, α), for configs (q, α) reachable when reading aaa.
1. (qinit , {c : 0}) →a (q1 , {c : 1}) →a (q1 , {c : 2}) →a (q1 , {c : 3})
2. (qinit , {c : 0}) →a (q1 , {c : 1}) →a (q1 , {c : 2}) →a (q2 , {c : 2})

3. (qinit , {c : 0}) →a (q1 , {c : 1}) →a (q2 , {c : 1}) →a (q1 , {c : 2})

4. (qinit , {c : 0}) →a (q1 , {c : 1}) →a (q2 , {c : 1}) →a (q2 , {c : 1})
We have that aaa ∈ L(M) since (q1 , {c : 3}), (q2 , {c : 2}) and (q1 , {c : 2}) are final configs.

2.3

Thompson-like matching algorithm

For a c-regex r and a string w ∈ A∗ , a Thompson-like matching algorithm decides whether w ∈ L(r), as
follows: Consider a CA equivalent to r, which we denote by M(r). A super-config of M refers to a set
of configs of M(r). Next, we define the functional relations ⇒a and ⇒w , over super-configs, for each
super-config S, character a ∈ A and w = a1 a2 · · · an ∈ A∗ , as follows:
S ⇒(M,a) {(q, β ) ∈ Q × Assign(C) | ∃(p, α) ∈ S : (p, α) →(M,a) (q, β )},

(S ⇒(M,w) Sn ) ⇐⇒ (S ⇒(M,a1 ) S1 ⇒(M,a2 ) S2 ⇒(M,a3 ) · · · ⇒(M,an ) Sn ).

<!-- page 5 -->

M. Berglund, B. van der Merwe and S. Sung

23

We write ⇒w for ⇒(M,w) when M is clear from the context.
Note that if {(p, α)} ⇒w T , then (p, α) →w (q, β ) if and only if (q, β ) ∈ T . Finally, w ∈ L(M) if
Sinit ⇒w Sn , where Sinit := {(qinit , αinit )} is the initial super-config, and Sn contains a final config.
Example 6. For the CA M in Figure 1, we have the following derivation of super-configs.
{(qinit , {c : 0})} ⇒a {(q1 , {c : 1})}

⇒a {(q1 , {c : 2}), (q2 , {c : 1})}

⇒a {(q1 , {c : 3}), (q2 , {c : 2}), (q1 , {c : 2}), (q2 , {c : 1})}.

Note the correspondence between this derivation and the derivation sequences given in Example 5.

2.4

Flat counter automata

Next, we consider the position CA constructed from a c-regex r with the position construction as in [6]; for
example, Figure 2 shows the position CA of the c-regex a((bc){2,10})+d{3,3}. The position CA of r
has |r|Σ states (where |r|Σ denotes the number of predicates from Σ in r) and at most (|r|Σ )2 transitions,
and it has the properties outlined next (see [8]). We begin with two required definitions.
Definition 7 (See [13]). For a counter variable ci ∈ C, the scope Qi ⊆ Q of ci is the smallest subset
of Q defined inductively such that q ∈ Qi if there exists a transition (p, σ , ψ, θ , q) ∈ ∆ such that either
(1) θi = reset, or (2) p ∈ Qi and θi ̸= inact.

For each state q ∈ Q, we denote by Cq := {ci ∈ C | q ∈ Qi } the set of counter variables whose scope
S
contains q. Furthermore, we define Q0 := Q \ ci ∈C Qi as the set of states outside the scope of any counter.
Consequently, Cq = /0 for any q ∈ Q0 .

Definition 8 (Transition types). A transition type for a counter variable ci ∈ C is a guard-action
pair (ψ(ci ), θ (ci )) that falls into one of the following five types: T0: (⊤, nop), T1: (⊤, reset), T2:
(ci < hi , inc), T3: (li ≤ ci , reset), and T4: (li ≤ ci , inact).
The following two properties generalize the property presented in Appendix B.1 of Holík et al. [8].

Property 9. For any transition (p, σ , ψ, θ , q) ∈ ∆ and counter variable ci ∈ C of a position CA, the
corresponding pair (ψ(ci ), θ (ci )) must be one of the five transition types in Definition 8, determined by
the relation of state p and q to the counter’s scope Qi :
1. if p ∈
/ Qi ∧ q ∈
/ Qi then (ψ(ci ), θ (ci )) is T0;
2. if p ∈
/ Qi ∧ q ∈ Qi then (ψ(ci ), θ (ci )) is T1;

3. if p ∈ Qi ∧ q ∈ Qi then (ψ(ci ), θ (ci )) is T0, T2 or T3.

4. if p ∈ Qi ∧ q ∈
/ Qi then (ψ(ci ), θ (ci )) is T4.

Each of these follows naturally from the structure of the counter scopes. For example, in 1 we are
outside the scope, so the counter has no effect, T0; cases 2 and 3 enter and exit the scope. Only case 3
is somewhat complex, as the transition goes from a state inside the scope to another inside the scope.
This can happen either without interacting with the counter operation at all, giving us T0; by iterating the
counter operation once, giving us T2; or by exiting the counter operation and reentering it, using some
enclosing closure, resetting the counter in T3.
Property 10. For any config (q, α) of the position CA and a counter variable ci ∈ C, α(ci ) ̸= 0 if and
only if q ∈ Qi .

<!-- page 6 -->

24

Efficient Matching with Sparse Counting-Sets

qinit
c1 : ⊤, nop,
c2 : ⊤, nop,
(T0, T0)

c1 : c1 < 10, inc,
c2 : ⊤, nop,
(T2, T0)
c1 : 2 ≤ c1 , reset,
c2 : ⊤, nop,
(T3, T0)

a1
c1 : ⊤, reset,
c2 : ⊤, nop,
(T1, T0)

c1 : 2 ≤ c1 , inact,
Q2
Q1 c2 : ⊤, reset,
(T4, T1)
c3
d4
c1 : ⊤, nop,
c2 : ⊤, nop,
(T0, T0)
b2

c1 : ⊤, nop,
c2 : c2 < 3, inc,
(T0, T2);

c1 : ⊤
c2 : 3 ≤ c2

Figure 2: The position CA of a((bc){2,10})+d{3,3}. Recall that + is the “one or more” operator.
Counter scopes Q1 and Q2 are denoted by rounded boxes. The symbol read by a transition (or rather its
predicate) is indicated on the target state (once subscripts are ignored). As is usual for position automata,
there is one state for each literal predicate in the expression (plus an additional initial state).
We focus on the position CAs of flat c-regexes, referred to as flat CAs. In a flat CA, |Cq | = 1 for q ∈
/ Q0
and |Cq | = 0 otherwise. In the flat case, we simplify the notation of a config (q, α) as follows: If q ∈ Qi
for some ci ∈ C, then the config is written as (q, α(ci )); if q ∈ Q0 , it is written as (q, 0). Note that every
c-regex can be converted into a flat c-regex using counter-expansion [3] where we perform the expansion
for all but one of the nested counters.

2.5

Counter configurations

Next, we describe the counting-set data structure that is used to represent a set of counter values assigned
to a counter variable at each state, following Turoňová et al. [13]; see also Le Glaunec et al. [7] for a
related bit-parallel approach.
Definition 11. A counting-set, which is a representation based on an offset and an ordered list, is a
tuple s = (o, ℓ, l, h), where:
• l ∈ N0 and h ∈ N∞ with l ≤ h denote the interval [l, h] representing the required number of repetitions;
• o ∈ N0 is the offset; and

• ℓ = ⟨ℓ1 , ℓ2 , . . . , ℓn ⟩ with |ℓ| = n is an ordered list of strictly decreasing values with each ℓi ∈ N0 and
o − ℓi ∈ [0, h].
We only consider counting-sets that can be obtained from the initial counting-set, i.e. (0, ⟨0⟩, l, h) (representing the set {0}), using the operations inc, add1 , merge, defined below.
We use ℓhead to denote the last element ℓn of ℓ; when ℓ is empty, we define ℓhead := ⊥. A countingset s represents the set Ns := {o − ℓi | 1 ≤ i ≤ n} ⊆ [0, h]. With s = (o, ℓ, l, h), we define the following
operations on counting-sets, taking counting-sets as inputs to output a Boolean value for check and a new
counting-set for inc, add1 and merge:
• check(s) := [ℓhead ̸= ⊥ ∧ l ≤ o − ℓhead ]; this checks whether Ns ∩ [l, h] ̸= /0;
• inc(s) := (o + 1, ℓ′ , l, h) represents 1 + Ns ; when (o + 1) − ℓhead ≤ h (i.e. if ℓhead is smaller than the
upper bound h) ℓ′ := ℓ, otherwise ℓ′ := ⟨ℓ1 , ℓ2 , . . . , ℓn−1 ⟩;

<!-- page 7 -->

M. Berglund, B. van der Merwe and S. Sung

25

85
values: · · ·

82

86

90

94

100
102
98

offset o = 106, list ℓ · · ·

24

20

16

12

8

: Removed by inc

∗

4

106

0

ℓhead

Figure 3: A counting-set with l = 85 and h = 100.
• add1 (s) := (o, ℓ′ , l, h) is defined when o > 0, and represents Ns ∪ {1}; we have ℓ′ := ℓ if ℓ1 = (o − 1),
and ℓ′ := ⟨o − 1, ℓ1 , ℓ2 , . . . , ℓn ⟩, otherwise;
• merge(s1 , s2 ) := (o1 , ℓ′ , l, h) represents the union of s1 = (o1 , ℓ1 , l, h) and s2 = (o2 , ℓ2 , l, h) for
o1 ≥ o2 , where ℓ′ is the result of merging ℓ1 and the list
⟨(o1 − o2 ) + ℓ21 , (o1 − o2 ) + ℓ22 , . . . , (o1 − o2 ) + ℓ2head ⟩,
by also removing duplicates.
We can compute check(s), inc(s), and add1 (s) in time O(1). The operation merge(s1 , s2 ) runs in
O(o2 ) time because o2 bounds (from above) both the size of ℓ2 and the number of elements in ℓ1 that must
be processed during the merge. The operation merge(s1 , s2 ) is defined as a destructive update where s1
consumes s2 . Consequently, s2 becomes unavailable for further use. When computing merge(s1 , s2 ), the
number of visited elements in ℓ1 and ℓ2 is bounded by 2 · o2 . Moreover, since the offset of s2 is o2 , this
implies that there were o2 calls to inc on s2 before merging. Combined with the fact that s2 is consumed
by merge(s1 , s2 ), this implies that the amortized time complexity of merge(s1 , s2 ) is constant [8]. Figure 3
illustrates the structure of a counting-set.
Counting set automata use counting sets for efficient matching of c-regexes with CAs [7, 8, 13]. As a
basis for our proposed matching algorithm, we describe counter-configurations, which we subsequently
modify to obtain sparse counting-sets.
Definition 12. For a flat CA we work with the simplified configs (q, n) introduced at the end of Section 2 (a
flat CA has |Cq | ≤ 1 for every state q, so every assignment α in a config reduces to a single value n = α(ci )
if q ∈ Qi , and to n = 0 if q ∈ Q0 ). A counter configuration (c-config) is then a function f : Q ⇀ 2N0
that maps each state q to a set f (q) of counter values. A super-config S ⊆ Q × N0 is represented by the
c-config f defined by f (q) := {n ∈ N0 | (q, n) ∈ S}, with f (q) undefined when no such pair occurs in S.
Example 13. The following c- and super-configs represent the same set of configurations.
- c-config f = {p : {1, 2}, q : {1, 2, 3}, r : {1, 2, 3}}

- super-config S = {(p, 1), (p, 2), (q, 1), (q, 2), (q, 3), (r, 1), (r, 2), (r, 3)}

Definition 14. Let S and T be super-configs of a flat CA such that S ⇒w T for some string w ∈ A∗ . If f
and g are c-configs that represent S and T , respectively, we write f ⇒w g.

Note that the counter values associated with a state q are placed in the set f (q), which is represented
using the counting-set structure. This representation allows a c-config to increase all values simultaneously
via the operation inc.
We define a flat c-regex r to be replicating if its corresponding position CA contains a state p ∈ Qi
with at least two outgoing transitions whose operations θ (ci ) are from {nop, inc} and whose predicates σ1
and σ2 are not disjoint (i.e., [[σ1 ]] ∩ [[σ2 ]] ̸= /0). Figure 4 illustrates a CA of a replicating c-regex. In such

<!-- page 8 -->

26

Efficient Matching with Sparse Counting-Sets

a; c ≤ 100, inc

q

p
a; ⊤, nop

r

(
f (p) = {1, 2, . . . , 100},
g (q) = inc(f (p))
represented by a counting-set ⇒a
g (r ) = f (p)
(100, ⟨99, 98, . . . , 0⟩, 20, 100)

Figure 4: A schematic representation of the replication of a counting-set during a computation of the
next c-config g from a c-config f . In this case, the counting-set f (p) is used to compute both of the
counting-sets g(q) and g(r).
a case, the matching algorithm in Algorithm 1 needs to replicate the counting-set s in order to compute
s′ . This replication requires time linear in the size of s, which is at most H(r). Recall, H(r) denotes the
largest finite bound in a counter range in r (i.e., every subexpression s{l,h} has l ≤ H(r) and, if h ̸= ∞,
then h ≤ H(r)). Next, we use this observation on the cost of replicating counting sets to analyse the time
complexity of Algorithm 1.
Algorithm 1 uses a counting-set data structure and c-configs to match an input string against a CA
obtained by applying the position automaton construction to a flat c-regex. Notably, Line 10 does not
explicitly check guards for nop and inc. This is unnecessary because (1) the guard component that
corresponds to nop and inc is always ⊤ and ci < hi by Property 9, and (2) counting-set’s inc operation
implicitly filters out values that exceed the upper bound hi . For each state p, the algorithm counts the
number of outgoing transitions with nop or inc operations in Line 12. If more than one such transition
exists, the corresponding counting-set is replicated in Line 21. Finally, as guaranteed by Property 9, the
cases handled in Lines 16, 18, and 20 are exhaustive and cover all possible transitions. Although we will
not provide a formal correctness argument, note that the natural correctness invariant for Algorithm 1
is the following: After processing the first i characters of the input, the c-config f maps each state q to
exactly the set of counter values n such that the flat CA has a reachable config (q, n) after that prefix.
For the proposition below to hold, we need to rewrite r{l,∞} as r{l,l} r∗ . Alternatively, one may provide
special treatment for the case r{l,∞} (not given in the pseudocode listed for Algorithm 1). For this, we
split the transition for (ci < h, inc) of T4 in Definition 8 into (ci < l, inc) and (ci = l, nop) if h = ∞, or by
constraining the inc operation to have no effect once a counter reaches its lower bound l (if h = ∞).
Proposition 15 (See [8]). For a flat c-regex r and a string w, Algorithm 1 (modified for the case r{l,∞} ) on
the position CA of r decides if w ∈ L(r) in O(H(r) · (|r|Σ )2 |w|)-time. Furthermore, if r is non-replicating,
then Algorithm 1 runs in O((|r|Σ )2 |w|))-time.

3

Sparse counting-sets

Recall that the matching time complexity of replicating c-regexes depends on the maximum size of
counting-sets. We propose a new data structure, the sparse counting-set, which stores only useful values.
For example, for an expression E {5,10} the sets {4, 5, 6, 7, 8, 9, 10} and {4, 10} are indistinguishable under
the counting operators; they both satisfy check, and do so when applying inc up to six times, but stop
satisfying check on a seventh inc. This difference, at which any intermediary counter values become
useless, will be used throughout this section. We let k = h − l + 1, with k = ∞ when h = ∞.
Definition 16. A sparse counting-set is a tuple s = (o, ℓ, l, h) as in Definition 11, except it is produced
from the initial counting-set using only the operations inc, sparseAdd1 and sparseMerge, defined next.

<!-- page 9 -->

M. Berglund, B. van der Merwe and S. Sung

27

Algorithm 1 CA matching with counting-sets — adapted from Holík et al. [8].
1: Input: position CA M = (Q, Σ,C, qinit , ∆, F) of a flat c-regex; input string w = a1 a2 · · · an ∈ A∗

2: Output: true if w ∈ L(M), false otherwise

Notation 1: Because M is flat, for every q ∈ Q \ Q0 there is a unique counter c j ∈ C with q ∈ Q j ; we
write c(q) := c j and (l(q), h(q)) := (l j , h j ) for its bounds, and set c(q) := ⊥ for q ∈ Q0 .
Notation 2: We write replicate(s) for a copy of the counting-set s, as depicted in Figure 4.
3: f ← {qinit : (0, ⟨0⟩, 0, 0)}
4: for each character ai in w do
5:
counter_ops ← {q : /0 | q ∈ Q}
6:
replications ← {p : 0 | p ∈ Q}
7:
g ← {}
8:
for each key p of f and transition (p, σ , ψ, θ , q) ∈ ∆ do
9:
θ ′ ← θ (c(q)) if c(q) ̸= ⊥, and inact otherwise
10:
if ai ∈ [[σ ]] and (θ ′ ∈ {nop, inc} or check( f (p))) then
11:
if θ ′ ∈ {nop, inc} then
12:
replications(p) ← replications(p) + 1
13:
counter_ops(q) ← counter_ops(q) ∪ {(p, θ ′ )}
14:
for each q ∈ Q with counter_ops(q) ̸= /0 do
15:
for each (p, θ ′ ) ∈ counter_ops(q) do
16:
if c(q) = ⊥ and θ ′ ∈ {inact, nop} then
17:
g(q) ← (0, ⟨0⟩, 0, 0)
18:
else if c(q) ̸= ⊥ and θ ′ = reset then
19:
g(q) ← add1 (g(q)) if q is a key of g, and inc((0, ⟨0⟩, l(q), h(q))) otherwise
20:
else if c(q) ̸= ⊥ and θ ′ ∈ {nop, inc} then
21:
s ← replicate( f (p)) if replications(p) > 1, and f (p) otherwise
22:
replications(p) ← replications(p) − 1
23:
if θ ′ = inc then
24:
s ← inc(s)
25:
g(q) ← merge(g(q), s) if q is a key of g, and s otherwise
26:
f ← {q : g(q) | g(q) = (o, ℓ, l, h), |ℓ| > 0}
27: for each q ∈ F that is a key of f do
28:
if check( f (q)) then
29:
return true
30: return false

<!-- page 10 -->

28

Efficient Matching with Sparse Counting-Sets

Consequently, the CA matching algorithm with sparse counting-sets is exactly Algorithm 1, where the standard operations add1 and merge are replaced by their sparse counterparts sparseAdd1 and sparseMerge,
respectively.
• sparseAdd1 (s) := (o, ℓ′′ , l, h), where ℓ′′ is constructed as follows. Let add1 (s) = (o, ℓ′ , l, h) with
ℓ′ = ⟨ℓ′1 , ℓ′2 . . . , ℓ′head ⟩, noting that ℓ′1 = o − 1. Then, the ordered list ℓ′′ is obtained by removing ℓ′2
from ℓ′ if and only if ℓ′1 − ℓ′3 ≤ k.

• sparseMerge(s1 , s2 ) := (o1 , ℓ′′ , l, h), where merge(s1 , s2 ) = (o1 , ℓ′ , l, h) and ℓ′′ is obtained from ℓ′ by
iteratively checking triples ℓ′i−1 , ℓ′i , ℓ′i+1 , in order of increasing i, and removing ℓ′i if ℓ′i−1 − ℓ′i+1 ≤ k.
Assuming o2 ≤ o1 , we may stop checking triples once ℓ′i−1 < (o1 − o2 ), or once we have inspected
all triples in ℓ′ .

As the definition implies, we will consider Algorithm 1 from here with the operations replaced by
their sparse variants. The following lemmas are derived from the definitions of these operations.
Lemma 17. A sparse counting-set s = (o, ℓ, l, h) with |ℓ| ≥ 3, is such that the inequality ℓi − ℓi+2 > k
holds for any index i.
Proof. We show this by induction. A sparse counting-set must be obtained using the operations inc,
sparseAdd1 and sparseMerge. Only sparseAdd1 and sparseMerge lengthen a list ℓ. The lemma holds for
the initial counting-set. The operation sparseAdd1 either prepends an element (precisely when doing so
does not violate this inequality) or replaces the leading element. As the offset is monotonically increasing
under all operators, this new leading element must be greater than or equal to the element it replaces,
preserving this inequality. The operation sparseMerge directly removes the elements that violate this
inequality by scanning triples in the merged list, left to right. Note that after adding (o1 − o2 ) to each
element in the second list before merging, all elements of this list will be between (o1 − 1) and (o1 − o2 ),
inclusive, so we can stop checking triples once ℓ′i−1 < (o1 − o2 ).
Before stating the size bound, we record an invariant that simplifies the analysis. Although Definition 16 allows arbitrary sequences of inc, sparseAdd1 , and sparseMerge, the sparse counting-sets that
actually arise during a run of Algorithm 1 always satisfy o − ℓi ≥ 1 for every index i (i.e., the value 0
occurs only in the bare initial counting-set (0, ⟨0⟩, l, h), which has |ℓ| = 1).
The reason is as follows. sparseMerge is invoked only at line 25, in the branch c(q) ̸= ⊥ ∧ θ ′ ∈
{nop, inc}; by Property 9 this forces p, q ∈ Qi for some counter ci , and a routine induction on the outer
iteration shows that any counting-set stored at a state in some Qi has an offset of at least 1. Propagating
this through the operations: inc strictly increments every vi := o − ℓi , sparseAdd1 only ever prepends an
element with v = 1, and sparseMerge preserves the v-values of its operands. Since neither operand of a
sparseMerge in Algorithm 16 is the bare initial counting-set, the invariant vi ≥ 1 therefore propagates
throughout. We will use this fact freely in Lemma 18 and Proposition 19.
Lemma 18. For a sparse counting-set s = (o, ℓ, l, h) with h ≥ 1 arising during a run of Algorithm 1, the
size |ℓ| is bounded by

 
h
2
if h ̸= ∞,
k+1
H ′ (l, h) :=

2
if h = ∞.
Proof. For h = ∞ we have k = ∞, and |ℓ| ≤ 2 is forced by Lemma 17.
For h ̸= ∞, let ℓ = ℓ1 , . . . , ℓn and set vi := o − ℓi , so v1 < v2 < · · · < vn . The case n = 1 is trivial since
2⌈h/(k + 1)⌉ ≥ 2. So assume n ≥ 2; then s is not the bare initial counting-set (which has |ℓ| = 1), and

<!-- page 11 -->

M. Berglund, B. van der Merwe and S. Sung

29

85
values: · · ·

82

offset o = 106, list ℓ · · ·

24

: Removed by sparseAdd1

100
∗

106

90
20

16

12

8

4

0

ℓhead

Figure 5: A sparse counting-set with l = 85 and h = 100, where k = 16. We assume the sparse countingset is constructed by using sparse operations instead of the corresponding non-sparse operations as in
Figure 3.
by the invariant noted above, we have vi ≥ 1 for all i. Hence vi ∈ [1, h], and by Lemma 17 we have
vi+2 − vi ≥ k + 1.
Consider the odd-indexed subsequence v1 < v3 < · · · < v2 j−1 , where j = ⌈n/2⌉. Consecutive elements
differ by at least k + 1, so v2 j−1 ≥ v1 + ( j − 1)(k + 1) ≥ 1 + ( j − 1)(k + 1). Combined with v2 j−1 ≤ h:




h−1
h
( j − 1)(k + 1) ≤ h − 1 =⇒ j ≤
+1 =
.
k+1
k+1
The same argument applied to the even-indexed subsequence (whose smallest element is at least 2, giving
(i − 1)(k + 1) ≤ h − 2) yields ⌊n/2⌋ ≤ ⌈h/(k + 1)⌉. Summing gives n ≤ 2⌈h/(k + 1)⌉.
From the early stopping criteria in sparseMerge, and using that, we merge sparse lists (i.e. we can
apply the previous lemma to these two lists), we obtain the following proposition, which guarantees that
the time complexity of sparseMerge improves on that of merge (assuming a large enough value of k),
while preserving amortized constant time in the non-replicating case.
Proposition 19. For sparse counting-sets s1 , s2 arising during a run of Algorithm 1 with o2 ≤ o1 , the
computation of sparseMerge(s1 , s2 ) considers at most 2⌈o2 /(k + 1)⌉ elements from each of the two lists
to achieve the merge, and then investigates at most 2 · 2⌈o2 /(k + 1)⌉ triples from the merged list to restore
sparsity.
Lemma 20. Let Ns , Ns′ ⊆ N0 be sets of counter values represented by a counting-set s and corresponding
(i.e. produced with the corresponding sequence of sparse operations) sparse counting-set s′ , respectively.
Then, Ns′ ⊆ Ns and, for any n ∈ Ns \ Ns′ with n ∈ [l, h], there exists n′ ∈ Ns′ such that n′ ∈ [l, h]. That is,
check(s) = check(s′ ).
Sketch. The tuples s and s′ clearly agree in offset, l and h. Let ℓ and ℓ′ be the ordered lists in s and
s′ respectively. By construction, ℓ′ is a subsequence of ℓ. We proceed by contradiction. Assume that
check(s) ̸= check(s′ ). Then we must have check(s) true and check(s′ ) false, as s′ represents a subset of
s. Then there is some value v in ℓ that satisfies the check (i.e. l ≤ o − v ≤ h), which does not occur in ℓ′ .
The value v must have been removed either by sparseAdd1 or by sparseMerge, but each removes a value
only if it would otherwise occur in a context v′ , v, v′′ where v′ − v′′ < k + 1. However, we must then have
l ≤ o − v ≤ h, o − v′ < l and o − v′′ > h, but then the difference between v′ and v′′ would have to be at
least h − l + 2, which causes a contradiction.
Corollary 21. Let f be the c-config f of a flat CA representing the initial super-config. Given a string
w ∈ A∗ , we compute a c-config g such that f ⇒w g using counting-set operations and replication of
counting-sets. In parallel, we define another c-config g′ obtained by applying the same procedure, but

<!-- page 12 -->

30

Efficient Matching with Sparse Counting-Sets

replacing each counting-set operation with its sparse counterpart. Then, g represents a final super-config
if and only if g′ represents a final super-config.
Proof. Follows from Lemma 20, with each sparse counting-set in the configuration acting indistinguishably from its non-sparse counterpart.
Combined with Lemma 18, we derive the following definition of d-sparse. This property enables
efficient matching with c-configs that use sparse counting-sets instead of regular counting-sets, which we
refer to as sparse counter configurations (sc-configs).
Definition 22. A flat c-regex r is d-sparse if H ′ (l, h) ≤ d (with H ′ (l, h) as defined in Lemma 18) for every
subexpression of the form s{l,h} in r.
Example 23. If all subexpressions of the form s{l,h} of a c-regex r satisfy l = 0 or h = ∞, then the c-regex r
is 2-sparse.
With all this in hand, we can now state a bound on the membership problem using sparse counting-sets,
stated in terms of d. In many cases, d will be quite small, and in addition, in practice, this bound is
pessimistic, as it accounts for the possibility of replicating sets linearly many times.
Theorem 24. Let r be a d-sparse c-regex. For a given string w ∈ A∗ , we can decide whether w ∈ L(r) in
O(d · (|r|Σ )2 |w|)-time. If r is non-replicating, then the algorithm runs in O((|r|Σ )2 |w|))-time.
Proof. The automaton representing r still has (|r|Σ )2 as the number of transitions: While counting
operations create additional transitions (e.g., the two transitions from c3 to b2 in Figure 2), r being flat
means any pair of states belongs to at most two counter scopes, making only a constant number of counter
operations possible on transitions between that pair. The factor d bounds the size of the representation of
a sparse counting-set, allowing copying and/or merging sets within this bound.
Remark 25. For the sake of a simpler presentation, we did not discuss an optimization where sparse
counting-sets for ranges like r{0,h} , r{1,h} , or r{l,∞} only need to store a single value.

4

Experimental evaluation

We evaluate the efficiency of our sparse counting-set approach against baselines. Efficiency is quantified
by the total computation steps required for symbol evaluations, guard checks, and counting-set operations.
This includes (1) the number of processed counter values during merge operations and (2) the number
of counter values involved in the replication of counting-sets. Performance is assessed through targeted
analyses of hand-crafted patterns and large-scale evaluations on the Polyglot and Snort3 under adversarial
and random input scenarios.

4.1

Results on targeted regular expressions

Manually designed c-regexes have one of the following properties: (a) non-replicating, (b) 2-sparse, or
(c) neither. We evaluate and compare four distinct methods—counter-expansion followed by Thompson
NFA matching (c-expansion), Thompson-like matching with super-configs (super-config), standard
counter-configs (c-config), and counter-configs with sparse counting-sets (sc-config). For the c-expansion
method, the position CA construction is applied to the counter-expanded regex, yielding a standard
position NFA where all guards are ψ(c) = ⊤ and actions are θ (c) = nop.

<!-- page 13 -->

M. Berglund, B. van der Merwe and S. Sung

×103
4
2
0

×103

c-expansion
super-config
c-config
sc-config

6

0

31

20

40

(a) a*a{k,k}

60

80

×103

6

6

4

4

2

2

0

0

20

40

60

(b) (aa*){0,k}

80

0

0

20

40

60

80

(c) a*(a|aab){k,k}

Figure 6: A comparison of the number of computation steps for our manually designed c-regexes,
each matched against strings of the form ak . The x-axis indicates the length of the input string, which
corresponds to the value k in each pattern’s counting operator.
The results in Figure 6 highlight the advantages of our sparse counting-set approach. First, both
the counter-expansion and super-config methods fail to achieve linear-time matching. As shown in
Figure 6b, the standard c-config approach is also limited, achieving linear-time performance only on
non-replicating c-regexes. However, by incorporating the sparse counting-set, we extend this capability to
achieve linear-time matching for replicating c-regexes that are d-sparse for a bounded d. The reason our
sparse counting-set provides no advantage in Figure 6c is explained by Lemma 18. For a counting operator
such as {k,k}, the upper bound H ′ (l, h) of the number of values in a sparse counting-set becomes equal
to an upper bound h for the non-sparse one, as h − l + 1 is simply 1. As both bounds are k, any potential
benefit from sparsity is lost.
Furthermore, our experimental results demonstrate that for cases with a finite upper bound h in a
counter operation r{l,h} , the performance of the super-config method is within a constant factor of the
counter-expansion approach. This result aligns with the theoretical explanation—each config (q, n) in the
super-config approach is equivalent to a state in the position NFA generated by counter-expansion. The
constant factor difference arises from the elimination of guards and actions in the position NFA of the
expanded regex.

4.2

Results on real-world regular expressions

Next, we use c-regexes from two sources: the Polyglot regex corpus by Davis et al. [5] and the Talos
LightSPD Snort3 ruleset (snapshot-31470). Polyglot contains 511,196 patterns (30,833 with counters)
and Snort3 contains 9,755 patterns (2,837 with explicit counters). Table 1 summarizes the structural
properties of these patterns, determined via static analysis of their position counting automata.
We also compute the distribution of H ′ (l, h) for 64,792 and 4,734 individual counter operators found
in flat regexes from Polyglot and Snort3, respectively, to assess the practical bound of the parameter d on
matching performance. Figure 7 illustrates histograms of lower-bound l and upper-bound h of counter
operators and the size H ′ (l, h) of sparse counting-sets in our approach.
The bottom row of Figure 7 is the empirical distribution of H ′ (l, h), whose per-pattern maximum is
exactly the d of Theorem 24. In both corpora, the mass concentrates at small values, with only a short tail
at H ′ (l, h) ≥ 10; this is driven by the commonly occurring unbounded ranges, which give H ′ (l, h) = 2 by
Lemma 18. Hence, for most real-world patterns, d is a small constant, and the O(d · (|r|Σ )2 |w|) bound is
within a small factor of the non-replicating O((|r|Σ )2 |w|).

<!-- page 14 -->

32

Efficient Matching with Sparse Counting-Sets
Table 1: Structural properties of patterns with counters.
Property

Polyglot

Total with counters
Flat (depth d = 1)
Replicating
2-sparse

30,833
29,911
319
15,161

Snort3

(100%)
(97.0%)
(1.0%)
(49.2%)

2,837
2,749
79
1,300

(100%)
(96.9%)
(2.8%)
(45.8%)
Polyglot

Snort3

low l

103

10

102

102

101
100

100
0

1

2

3

4

5

6

7

8

9

≥ 10

high h

102

1

2

3

4

5

6

7

8

9

≥ 10

0

1

2

3

4

5

6

7

8

9

≥ 10

0

1

2

3

4

5

6

7

8

9

≥ 10

102

101
100

100
0

H ′ (l, h)

0
104

103

10

4

1

2

3

4

5

6

7

8

9

≥ 10

3

10

102

4

102

101
100

100
0

1

2

3

4

5

6

7

8

9

≥ 10

Figure 7: Histograms of the lower bound l, the upper bound h, and the maximum sparse counting-set
size H ′ (l, h) for each counter operator in flat c-regexes. All results are shown on a logarithmic scale.
Our experiments evaluate the robustness of our approach by conducting two parallel sets of experiments
simulating adversarial and random input scenarios. In both cases, we simulated a partial matching scenario
by wrapping each pattern r as .*(r).* to generate corresponding input strings. Our final dataset is
filtered to include only c-regexes that meet three criteria: they (a) contain at least one counting operation,
(b) are supported by our implementation, and (c) have a successfully generated test case.
Adversarial (Figure 8): EvilStrGen [11] is designed to generate test cases that maximize the number of
matching steps. EvilStrGen was configured to assume a non-backtracking engine and to generate string
lengths of approximately 1,000. In summary, we use 10,108 and 611 c-regexes from the Polyglot and
Snort3 datasets for the experiments with EvilStrGen, respectively.
Random (Figure 9): We simulate a random-input scenario by generating matched inputs with Xeger1 .
We use Xeger with its default limit 10, which decides the maximum number of repetitions for Kleene star,
plus, and counting operators with infinite upper bounds. For the experiments with Xeger, we use 10,141
and 620 c-regexes from the Polyglot and Snort3 datasets, respectively.

1 https://pypi.org/project/xeger/

<!-- page 15 -->

M. Berglund, B. van der Merwe and S. Sung

33

Polyglot

T/O
40000

103

101

10000

100

0

102

20000
0

T/O
20000

103

43853.03

(a) c-exp. vs. sup.-config

T/O
60000
40000
20000
0

64927.76

T/O
20000

103

101

10000

101

100

0

102

23749.17

(b) c-exp. vs. c-config

103
102
101
100

(d) sup.-config vs. c-config

T/O
60000
40000
20000
0

65882.63

102

23751.96

100

(c) c-exp. vs. sc-config

103
102
101
100

(e) sup.-config vs. sc-config

T/O
20000

103
102

10000
0

101
21737.00

100

(f) c-config vs. sc-config

Snort3

T/O

T/O

40000

101

20000
0

58782.00

100

(g) c-exp. vs. sup.-config

T/O

20000

101

10000
0

25921.00

100

(h) c-exp. vs. c-config

T/O
100000

102

50000

101

50000

101

100

0

(j) sup.-config vs. c-config

123302.04

0

25921.00

100

(i) c-exp. vs. sc-config

102

121346.16

101

10000

T/O
100000

0

20000

100

(k) sup.-config vs. sc-config

T/O
15000
10000
5000
0

102
101
16065.68

100

(l) c-config vs. sc-config

Figure 8: Comparisons of matching steps with input strings generated by EvilStrGen, presented as a
sequence of 2D histograms. Each plot is labeled in a “Method 1 vs. Method 2” format, where the x-axis
represents the step count for the first method and the y-axis for the second with the same scale. The color
of each cell represents the number of such c-regex input string pairs. The second last line denotes the 99th
percentile, and cases above (or to the right of) the “T/O” label timed out after one second.

<!-- page 16 -->

34

Efficient Matching with Sparse Counting-Sets

Polyglot

T/O
8000
6000
4000
2000
0

8279.28

103
102
101
100

T/O
2000
1000
0

(a) c-exp. vs. sup.-config

T/O
8000
6000
4000
2000
0

8071.85

2295.98

103
102
101
100

T/O
2000
1000
0

(b) c-exp. vs. c-config

103
102
101
100

(d) sup.-config vs. c-config

T/O
8000
6000
4000
2000
0

8216.60

2306.19

103
102
101
100

(c) c-exp. vs. sc-config

103
102
101
100

(e) sup.-config vs. sc-config

T/O
1000
500
0

1306.67

103
102
101
100

(f) c-config vs. sc-config

Snort3

T/O

102

10000

101

5000
0

14803.88

100

(g) c-exp. vs. sup.-config

T/O
10000
7500
5000
2500
0

102
101
10221.24

100

(h) c-exp. vs. c-config

102

T/O
10000

102

5000

101

5000

101

100

0

12681.88

(j) sup.-config vs. c-config

12681.88

102
101
10221.24

100

(i) c-exp. vs. sc-config

T/O
10000
0

T/O
10000
7500
5000
2500
0

100

(k) sup.-config vs. sc-config

T/O
6000
4000
2000
0

102
101
6407.32

100

(l) c-config vs. sc-config

Figure 9: Comparisons of matching steps with input strings generated by xeger, using the same
experimental setup as in Figure 8. The 2D histograms show pairwise comparisons of the four methods,
with cell color indicating the density of test cases.

<!-- page 17 -->

M. Berglund, B. van der Merwe and S. Sung

5

35

Conclusion and future work

We introduced c-configs, a formalization of on-the-fly matching with counting-set automata [8]. Based on
this formalization, we presented a representation of super-configs as sparse counting-sets, minimizing the
number of counter values that are required to be tracked.
One can utilize counter expansion to convert non-flat c-regexes into flat c-regexes. The choice of which
counting operator to expand influences the resulting expression. For instance, the c-regex (a{2,3}b){2,2}
can be expanded as either (aaa?b){2,2} or (a{2,3}b)(a{2,3}b). Determining which expansion form
leads to more efficient CA-based matching is left as future research. We would also like to explore the
possibility of generalizing the (sparse) counting-set data structure to support non-flat c-regexes.

Acknowledgements
The authors thank Rick Bower and River Martin for their detailed review of the experimental results.

References
[1] George Argyros & Loris D’Antoni (2018): The Learnability of Symbolic Automata. In: Computer Aided
Verification - 30th International Conference, CAV 2018, Proceedings, Part I, Lecture Notes in Computer
Science 10981, Springer, pp. 427–445, doi:10.1007/978-3-319-96145-3_23.
[2] Henrik Björklund, Wim Martens & Thomas Timm (2015): Efficient Incremental Evaluation of Succinct Regular
Expressions. In: Proceedings of the 24th ACM International Conference on Information and Knowledge
Management, CIKM 2015, Melbourne, VIC, Australia, October 19 - 23, 2015, ACM, pp. 1541–1550,
doi:10.1145/2806416.2806434.
[3] Russ Cox (2010): Regular Expression Matching in the Wild: A tour of RE2, an efficient, production regular
expression implementation. Available at https://swtch.com/~rsc/regexp/regexp3.html. Accessed:
2025-04-26.
[4] James C Davis, Christy A Coghlan, Francisco Servant & Dongyoon Lee (2018): The impact of regular
expression denial of service (ReDoS) in practice: an empirical study at the ecosystem scale. In: Proceedings
of the 2018 26th ACM joint meeting on European Software Engineering Conference and Symposium on the
Foundations of Software Engineering, pp. 246–256, doi:10.1145/3236024.3236027.
[5] James C. Davis, Louis G. Michael, IV, Christy A. Coghlan, Francisco Servant & Dongyoon Lee (2019):
Why aren’t regular expressions a lingua franca? An empirical study on the re-use and portability of regular
expressions. In: Proceedings of the ACM Joint Meeting on European Software Engineering Conference and
Symposium on the Foundations of Software Engineering, ESEC/SIGSOFT FSE 2019, Tallinn, Estonia, August
26-30, 2019, ACM, pp. 443–454, doi:10.1145/3338906.3338941.
[6] Wouter Gelade, Marc Gyssens & Wim Martens (2012): Regular Expressions with Counting: Weak versus
Strong Determinism. SIAM J. Comput. 41(1), pp. 160–190, doi:10.1137/100814196.
[7] Alexis Le Glaunec, Lingkun Kong & Konstantinos Mamouras (2023): Regular Expression Matching using Bit
Vector Automata. Proc. ACM Program. Lang. 7, pp. 492–521, doi:10.1145/3586044.
[8] Lukás Holík, Juraj Síc, Lenka Turonová & Tomás Vojnar (2023): Fast Matching of Regular Patterns with
Synchronizing Counting (Technical Report). CoRR abs/2301.12851. arXiv:2301.12851.
[9] Lingkun Kong, Qixuan Yu, Agnishom Chattopadhyay, Alexis Le Glaunec, Yi Huang, Konstantinos Mamouras
& Kaiyuan Yang (2022): Software-hardware codesign for efficient in-memory regular pattern matching. In
Ranjit Jhala & Isil Dillig, editors: PLDI ’22: 43rd ACM SIGPLAN International Conference on Programming
Language Design and Implementation, San Diego, CA, USA, June 13 - 17, 2022, ACM, pp. 733–748,
doi:10.1145/3519939.3523456.

<!-- page 18 -->

36

Efficient Matching with Sparse Counting-Sets

[10] Randy Smith, Cristian Estan & Somesh Jha (2008): XFA: Faster Signature Matching with Extended Automata.
In: 2008 IEEE Symposium on Security and Privacy (SP 2008), 18-21 May 2008, Oakland, California, USA,
IEEE Computer Society, pp. 187–201, doi:10.1109/SP.2008.14.
[11] Weihao Su, Hong Huang, Rongchen Li, Haiming Chen & Tingjian Ge (2024): Towards an Effective Method of
ReDoS Detection for Non-backtracking Engines. In: 33rd USENIX Security Symposium (USENIX Security
24), pp. 271–288.
[12] Lenka Turonová, Lukás Holík, Ivan Homoliak, Ondrej Lengál, Margus Veanes & Tomás Vojnar (2022):
Counting in Regexes Considered Harmful: Exposing ReDoS Vulnerability of Nonbacktracking Matchers.
In: 31st USENIX Security Symposium, USENIX Security 2022, Boston, MA, USA, August 10-12, 2022,
USENIX Association, pp. 4165–4182.
[13] Lenka Turonová, Lukás Holík, Ondrej Lengál, Olli Saarikivi, Margus Veanes & Tomás Vojnar (2020): Regex
matching with counting-set automata. Proc. ACM Program. Lang. 4, pp. 218:1–218:30, doi:10.1145/3428286.
