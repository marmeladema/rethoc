---
file: ../pdfs/2022-arxiv-borsotti-closer-look-tdfa.pdf
citation: "A closer look at TDFA; Borsotti, Trofimovich; arXiv:2206.01398; 2022 (v4, Mar 2026)"
doi: 
source: "https://arxiv.org/pdf/2206.01398"
topics: [semantics_theory_access]
pages: 26
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

A closer look at TDFA
Angelo Borsotti

Ulya Trafimovich

angelo.borsotti@mail.polimi.it

skvadrik@gmail.com

2022

arXiv:2206.01398v4 [cs.FL] 29 Mar 2026

Abstract
We present an algorithm for regular expression parsing and submatch extraction based on tagged deterministic finite automata. The algorithm works with different disambiguation policies. We give detailed
pseudocode for the algorithm, covering important practical optimizations. All transformations from a regular
expression to an optimized automaton are explained on a step-by-step example. We consider both ahead-oftime and just-in-time determinization and describe variants of the algorithm suited to each setting. We provide
benchmarks showing that the algorithm is very fast in practice. Our research is based on two independent
implementations: an open-source lexer generator RE2C and an experimental Java library.

Introduction
This paper describes tagged deterministic finite automata (TDFA). To the best of our knowledge, it is the first
practical submatch extraction and parsing algorithm based on deterministic finite automata that is capable of
both POSIX and leftmost greedy disambiguation. Most of the theory behind TDFA is not new, but the previous
papers are incomplete and lack important details. This paper consolidates our previous research and provides a
comprehensive description of the algorithm. We hope that it will make TDFA easier to implement in practice.
Here is a brief history of TDFA development. In 2000 Laurikari published the original paper [1]. In 2007
Kuklewicz implemented TDFA in a Haskell library with POSIX longest-match disambiguation; he gave only an
informal description [2]. In 2016 Trafimovich presented TDFA with lookahead [3], implemented them in the
open-source lexer generator RE2C [4] and formalized Kuklewicz disambiguation algorithm. In 2017 Borsotti
implemented TDFA in an experimental Java library [5]. In 2019 Borsotti and Trafimovich adapted Okui-Suzuki
disambiguation algorithm to TDFA and showed it to be faster than Kuklewicz algorithm [6]. In 2020 Trafimovich
published an article about TDFA implementation in RE2C [7]. Finally, the present paper incorporates our past
research and adds novel findings on multi-pass TDFA that are better suited to just-in-time determinization.
Before diving into details, we recall the key concepts discussed in the paper.
Regular expressions (REs) are a notation for describing sets of strings known as regular languages, or Type-3
languages in the Chomsky hierarchy. They were first defined by Kleene [9] as sets of strings constructed from the
alphabet symbols and the empty word via the application of three basic operations: concatenation, alternative
and iteration. Later REs were formalized via the notion of Kleene algebra [10]. In practice REs have many
extensions that vary in complexity and expressive power.
Submatch extraction is an extension of REs that allows one to identify input positions matching specific
positions in a RE. Recall the difference between recognition and parsing: to recognize a string means to determine
its membership in a language, but to parse a string means to also find its derivation in a language grammar.
Submatch extraction is in between: on one extreme it approaches recognition (if there are no submatch markers
in a RE), but on the other extreme it is identical to parsing (if every position in a RE is marked). In general it
requires constructing a partial derivation, which can be implemented more efficiently than full parsing.
Finite state automata are a formalism equivalent to REs in the sense that every RE can be converted to a
deterministic finite automaton (DFA) or a nondeterministic finite automaton (NFA), and vice versa. There are
many different kinds of NFA, but there is a unique minimal DFA for a given RE. Both NFA and DFA solve the
recognition problem for REs in linear time in the length of input. In practice DFA are faster because they follow
a single path, while NFA have to track multiple paths simultaneously. NFA can be converted to DFA using a
determinization procedure, but the resulting DFA may be exponentially larger than the NFA.
1

<!-- page 2 -->

Tags are submatch markers: they mark positions in a RE that should be mapped to offsets in the input
string. When a RE is converted to an NFA, tags are placed on the NFA transitions. This effectively turns NFA
into a nondeterministic finite-state transducer that rewrites symbolic strings into tagged strings (where tags are
placed in-between symbols, marking submatch boundaries). Conversion from a RE to a tagged NFA is natural
if NFA mirrors the structure of RE, as in the case of Thompson’s construction.
Determinization of a tagged NFA is problematic, because in a DFA multiple NFA paths are collapsed into
one, causing conflicts when the same tag has different values on different NFA paths. To keep track of all possible
tag values, a DFA is augmented with registers and operations on transitions that update register values. The
number of registers and operations depends only on the RE structure and tag density, but not on the input
string, therefore it adds only a constant overhead to the DFA execution. We describe techniques that reduce
redundant operations and minimize the overhead in practice.
Ambiguity is yet another problem for submatch extraction; it means the existence of multiple different parse
trees for the same input. Ambiguity should not be confused with non-determinism, which means the existence
of multiple possibilities during matching that get canceled as more input is consumed; ambiguity is a property
of a RE. One way to resolve it is a disambiguation policy, the most notable examples being the leftmost-greedy
and the longest-match (POSIX) policies. TDFA can work with both policies, and there is no runtime overhead
on disambiguation — it is built into TDFA structure. Some RE engines provide other ways to resolve ambiguity,
such as user-defined precedence rules, but these are ad-hoc, error-prone and often difficult to reason about.
RE engines based on DFA can be divided in two groups: those using ahead-of-time (AOT) determinization
(e.g. lexer generators) and those using just-in-time (JIT) determinization (e.g. runtime libraries). The former
can spend considerable amount of time on preprocessing, but the latter face a tradeoff between the time spent
on preprocessing and the time spent on matching. Therefore it makes sense to use different variants of the
algorithm in each case. We describe single-pass TDFA that are a natural fit for ahead-of-time determinization,
and multi-pass TDFA that are better suited to just-in-time determinization.
In practice performance of a matching algorithm depends on the representation of results. The most generic
representation is a parse tree; it precisely reflects a derivation. A more lightweight representation is a list of offsets
or a single offset per submatch position in a RE (the latter is used in the POSIX regexec function). Another
representation, more suitable for transducers, is a tagged string — a sequence of input symbols interspersed with
tags. If a RE contains tags for every subexpression, then it is possible to reconstruct a parse tree from offset lists
or a tagged string (a procedure is given in [6], section 6). TDFA can be used with all the above representations,
but it is more natural to use offsets with single-pass TDFA and tagged strings with multi-pass TDFA.
The rest of the paper is structured as follows. Section 1 defines REs and their conversion to nondeterministic automata. Section 2 defines TDFA and determinization. Section 3 describes optimizations and practical
implementation details. Section 4 describes multi-pass TDFA and their application to just-in-time determinization. Section 5 provides benchmarks and comparison with other algorithms. Section 6 contains conclusions, and
section 7 contains ideas for future work.

Conventions
In this paper we use pseudocode rather than formal mathematical notation to describe algorithms. We focus
on the practical side, because we want to encourage TDFA implementation in real-world programs. The most
theoretically challenging part of the algorithm (POSIX disambiguation) is formalized in our previous paper [6],
and the core of the algorithm is based on the well-known idea of determinization via powerset construction that
does not need a formal introduction.
In the pseudocode, we try to balance between formality and clarity. We omit the definitions of basic operations
on data structures, such as “append to a list” or “push on stack”. We sometimes use set notation with predicates,
and sometimes prefer explicit loops that iterate over the elements of a set. To reduce verbosity, we assume that
function arguments are passed by reference and modifications to them are visible in the calling function (although
some functions have explicit return values).
All algorithms presented here are implemented in the open-source lexer generator RE2C and are known to
work in practice.

2

<!-- page 3 -->

1

TNFA
In this section we define regular expressions, their conversion to nondeterministic automata and matching.

Definition 1. Regular expressions (REs) over finite alphabet Σ are:
1. Empty RE ϵ, unit RE a ∈ Σ and tag t ∈ N.
(0 ≤ n ≤ m ≤ ∞) where e1 and e2 are REs over Σ.
2. Alternative e1 |e2 , concatenation e1 e2 and repetition en,m
1

Tags mark submatch positions in REs. Unlike capturing parentheses, tags are not necessarily paired. Capturing groups can be represented with tags, but the correspondence may be more complex than a pair of tags
per group, e.g. POSIX capturing groups require additional hierarchical tags [6].
Generalized repetition en,m can be bounded (m < ∞) or unbounded (m = ∞). Unbounded repetition e0,∞
is the canonical Kleene iteration, shortened as e∗ . Bounded repetition is usually desugared via concatenation,
but we avoid desugaring as it may duplicate tags and change submatch semantics in a RE.
Definition 2. Tagged Nondeterministic Finite Automaton (TNFA) is a structure (Σ, T, Q, q0 , qf , ∆), where:
Σ is a finite set of symbols (alphabet)
T is a finite set of tags
Q is a finite set of states with initial state q0 and final state qf
∆ is a transition relation that contains transitions of two kinds:
transitions on alphabet symbols (q, a, p) where q, p ∈ Q and a ∈ Σ
optionally tagged ϵ-transitions with priority (q, i, t, p) where q, p ∈ Q, i ∈ N and t ∈ T ∪ T ∪ {ϵ}

TNFA is in essence a non-deterministic finite state transducer with input alphabet Σ and output alphabet
Σ ∪ T ∪ T . T = {−t | t ∈ T } is the set of all negative tags which represent the absence of match: they appear
whenever there is a way to bypass a tagged subexpression in a RE, such as alternative or repetition with zero
lower bound. Negative tags serve a few purposes: they prevent propagation of stale submatch values from one
iteration to another, they spares the need to initialize tags, and they are needed for POSIX disambiguation [6].
Priorities are used for transition ordering during ϵ-closure construction.
Algorithm 2 on page 4 shows TNFA construction: it performs top-down structural recursion on a RE, passing
the final state on recursive descent into subexpressions and using it to connect subautomata. This is similar to
Thompson’s construction, except that non-essential ϵ-transitions are removed and tagged transitions are added.
The resulting automaton mirrors the structure of RE and preserves submatch information and ambiguity in it.
simulation (Σ, T, Q, q0 , qf , ∆), a1 . . . an



1
2

m0 : vector of offsets of size |T |
C = {(q0 , m0 )}

3
4
5
6

for k = 1, n do
C = epsilon closure(C, ∆, qf , k)
C = step on symbol(C, ∆, ak )
if C = ∅ then return ∅

7

C = epsilon closure(C, ∆, qf , n)

8
9

if ∃(q, m) in C | q = qf then return m
else return ∅

epsilon closure C, ∆, qf , k
11

C ′ : empty sequence of configurations

12
13

for (q, m) in C in reverse order do
push (q, m) on stack

14
15

while stack is not empty do
pop (q, m) from stack
append (q, m) to C ′

16
17
18
19
20
21


step on symbol C, ∆, a
10

22
23

return {(p, m) | (q, m) in C and (q, a, p) ∈ ∆}



for each (q, i, t, p) ∈ ∆ ordered by priority i do
if t > 0 then m[t] = k
else m[−t] = n
if configuration with state p is not in C ′ then
push (p, m) on stack
return {(q, m) in C ′ | q = qf or
∃(q, a, ) ∈ ∆ where a ∈ Σ}

Algorithm 1: TNFA simulation.
Algorithm 1 defines TNFA simulation on a string. It starts with a single configuration (q0 , m0 ) consisting of
the initial state q0 and an empty vector of tag values, and loops over the input symbols until all of them are
matched or the configuration set becomes empty, indicating match failure. At each step the algorithm constructs
ϵ-closure of the current configuration set, updating tag values along the way, and steps on transitions labeled
with the current input symbol. Finally, if all symbols have been matched and there is a configuration with
the final state qf , the algorithm terminates successfully and returns the final vector of tag values. Otherwise it
returns a failure. The algorithm uses leftmost greedy disambiguation; POSIX disambiguation is more complex
and requires a different ϵ-closure algorithm [6]. Figure 1 in section 2 shows an example of TNFA simulation.
3

<!-- page 4 -->

tnfa(e, qf )
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
28
29
30
31
32
33
34
35
36
37
38
39

qf

if e = ϵ then
return (Σ, ∅, {qf }, qf , qf , ∅)

tnfa(, qf )

else if e = a ∈ Σ then
return (Σ, ∅, {q0 , qf }, q0 , qf , {(q0 , a, qf )})

q0

41
42
43

qf

tnfa(a, qf ) |a ∈ Σ

else if e = t ∈ N then
return (Σ, {t}, {q0 , qf }, q0 , qf , {(q0 , 1, t, qf )})

q0

1/t

qf

tnfa(t, qf ) |t ∈ N

else if e = e1 · e2 then
(Σ, T2 , Q2 , q2 , qf , ∆2 ) = tnfa(e2 , qf )
(Σ, T1 , Q1 , q1 , q2 , ∆1 ) = tnfa(e1 , q2 )
return (Σ, T1 ∪ T2 , Q1 ∪ Q2 , q1 , qf , ∆1 ∪ ∆2 )

q1

q2

tnfa(e1 , q2 )

qf

tnfa(e2 , qf )

tnfa(e1 · e2 , qf )

else if e = e1 | e2 then
(Σ, T2 , Q2 , q2 , qf , ∆2 ) = tnfa(e2 , qf )
(Σ, T2 , Q′2 , q2′ , qf , ∆′2 ) = ntags(T2 , qf )
(Σ, T1 , Q1 , q1 , q2′ , ∆1 ) = tnfa(e1 , q2′ )
(Σ, T1 , Q′1 , q1′ , q2 , ∆′1 ) = ntags(T1 , q2 )
Q = Q1 ∪ Q′1 ∪ Q2 ∪ Q′2 ∪ {q0 }
∆ = ∆1 ∪ ∆′1 ∪ ∆2 ∪ ∆′2 ∪ {(q0 , 1, ϵ, q1 ), (q0 , 2, ϵ, q1′ )}
return (Σ, T1 ∪ T2 , Q, q0 , qf , ∆)

tnfa(e1 , q20 )

q1
1/

q20

ntag

s(T

2, q
f)

q0
2/

q10

ntags(T1 , q2 )

q2

e 2, q f
tnfa(

qf

)

tnfa(e1 | e2 , qf )

else if e = en,m
|1<n≤m≤∞ then
1
−1,m−1
(Σ, T1 , Q1 , q2 , qf , ∆1 ) = tnfa(en
, qf )
1
(Σ, T2 , Q2 , q1 , q2 , ∆2 ) = tnfa(e1 , q2 )
return (Σ, T1 ∪ T2 , Q1 ∪ Q2 , q1 , qf , ∆1 ∪ ∆2 )

q1

tnfa(en−1,m−1
, qf )
1

q2

tnfa(e1 , q2 )

qf

tnfa(en,m
, qf ) |1 < n ≤ m ≤ ∞
1

else if e = e1,m
|1<m<∞ then
1
if m = 1 then return tnfa(e1 , qf )
−1
(Σ, T1 , Q1 , q1 , qf , ∆1 ) = tnfa(e1,m
, qf )
1
(Σ, T2 , Q2 , q0 , q2 , ∆2 ) = tnfa(e1 , q1 )
∆ = ∆1 ∪ ∆2 ∪ {(q1 , 1, ϵ, qf ), (q1 , 2, ϵ, q2 )}
return (Σ, T1 ∪ T2 , Q1 ∪ Q2 , q0 , qf , ∆)

1/
q0

tnfa(e1 , q1 )

2/

q1

tnfa(e1,m−1
, qf )
1

q2

qf

tnfa(e1,m
1 , q f ) |1 < m < ∞
q1

else if e = e0,m
then
1
(Σ, T1 , Q1 , q1 , qf , ∆1 ) = tnfa(e1,m
, qf )
1
(Σ, T1 , Q′1 , q1′ , qf , ∆′1 ) = ntags(T1 , qf )
Q = Q1 ∪ Q′1 ∪ {q0 }
∆ = ∆1 ∪ ∆′1 ∪ {(q0 , 1, ϵ, q1 ), (q0 , 2, ϵ, q1′ )}
return (Σ, T1 , Q, q0 , qf , ∆)

1/

tnfa(e 1,m
1 ,qf )

2/

T 1, q f )
ntag s(

q0
q10

qf

tnfa(e0,m
1 , qf )

else if e = e1,∞
then
1
(Σ, T1 , Q1 , q0 , q1 , ∆1 ) = tnfa(e1 , q1 )
Q = Q1 ∪ {qf }
∆ = ∆1 ∪ {(q1 , 1, ϵ, q0 ), (q1 , 2, ϵ, qf )}
return (Σ, T1 , Q, q0 , qf , ∆)

1/

q0

2/

q1

tnfa(e1 , q1 )

qf

tnfa(e1,∞
, qf )
1

ntags(T, qf )
40

a/

{ti }n
i=1 = T
Q = {qi }n
i=0 where qn = qf
∆ = {(qi−1 , 1, −ti , qi )}n
i=1
return (Σ, T, Q, q0 , qf , ∆)

q0

1/ − t1

ntags(T, qf )

Algorithm 2: TNFA construction.

4

q1

...

qn−1

1/ − tn

qf

<!-- page 5 -->

2

TDFA
In this section we define TDFA and show how to convert TNFA to TDFA.

Definition 3. Tagged Deterministic Finite Automaton (TDFA) is a structure (Σ, T, S, Sf , s0 , R, Rf , δ, φ), where:
Σ is a finite set of symbols (alphabet)
T is a finite set of tags
S is a finite set of states with initial state s0 and a subset of final states Sf ⊆ S
R is a finite set of registers with a subset of final registers Rf (one per tag)
δ : S × Σ → S × O∗ is a transition function
φ : Sf → O∗ is a final function
where O is a set of register operations of the following types:
set register i to nil or to the current position: i ← v, where v ∈ {n, p}
copy register j to register i: i ← j
copy register j to register i and append history: i ← j · h, where h is a string over {n, p}

Compared to an ordinary DFA, TDFA is extended with a set of tags T , a set of registers R with one final
register per tag, and register operations that are attributed to transitions and final states (the δ and φ functions).
O∗ denotes the set of all sequences of operations over O. Operations can be of three types: set, copy, append. Set
operations are used for single-valued tags (those represented with a single offset), append operations are used for
multi-valued tags (those represented with an offset list), and copy operations are used for all tags. The decision
which tags are single-valued and which ones are multi-valued is arbitrary and individual for each tag (it may be,
but does not have to be based on whether the tag is under repetition). Register values are denoted by special
symbols n and p, which mean nil and the current position (offset from the beginning of the input string).
Recall the canonical determinization algorithm that is based on powerset construction: NFA is simulated on
all possible strings, and the subset of NFA states at each step of the simulation forms a new DFA state, which
is either mapped to an existing identical state or added to the growing set of DFA states. Since the number of
different subsets of NFA states is finite, determinization eventually terminates. The presence of tags complicates
things: it is necessary to track tag values, which depend on the offset that increases at every step. This makes
the usual powerset construction impossible: DFA states augmented with tag values are different and cannot be
mapped. As a result the set of states grows indefinitely and determinization does not terminate. To address this
problem, Laurikari used indirection: instead of storing tag values in TDFA states, he stored value locations —
registers. As long as two TDFA states have the same registers, the actual values in registers do not matter: they
change dynamically at runtime (during TDFA execution), but they do not affect TDFA structure. A similar
approach was used by Grathwohl [8], who described it as splitting the information contained in a value into static
and dynamic parts. The indirection is not free: it comes at the cost of runtime operations that update register
values. But it solves the termination problem, as the required number of registers is finite, unlike the number of
possible register values.
From the standpoint of determinization, a TDFA state is a pair. The first component is a set of configurations
(q, r, l) where q is a TNFA state, r is a vector of registers (one per tag) and l is a sequence of tags. Unlike TNFA
simulation that updates tag values immediately when it encounters a tagged transition, determinization delays
the application of tags until the next step. It records tag sequences along TNFA paths in the ϵ-closure, but
instead of applying them to the current transition, it stores them in configurations of the new TDFA state and
later applies them to the outgoing transitions. This allows filtering tags by the lookahead symbol: configurations
that have no TNFA transitions on the lookahead symbol do not contribute any register operations to TDFA
transition on that symbol. The use of the lookahead symbol is what distinguishes TDFA(1) from TDFA(0) [3];
it considerably reduces the number of operations and registers. During ϵ-closure construction configurations are
extended to four components (q, r, h, l) where h is the sequence of tags inherited from the origin TDFA state and
l is the new sequence constructed by the ϵ-closure.
The second component of TDFA state is precedence information. It is needed for ambiguity resolution: if
some TNFA state in the ϵ-closure can be reached by different paths, one path must be preferred over the others.
This affects submatch extraction, as the paths may have different tags. The form of precedence information
depends on the disambiguation policy. We keep the details scoped to functions precedence, step on symbol
and epsilon closure, so that algorithm 3 can be adapted to different policies without the need to change its
structure. In the case of leftmost greedy policy precedence information is an order on configurations, represented by precedence as a vector of TNFA states: step on symbol uses it to construct the initial closure, and
5

<!-- page 6 -->

determinization Σ, T, Q, q0 , qf , ∆



1
2
3
4

S, Sf : empty sets of states
δ : undefined transition function
φ : undefined final function
r0 = {1, ... , |T |}, Rf = {|T |+1, ... , 2|T |}, R = {r0 } ∪ Rf

5
6
7

C = epsilon closure({(q0 , r0 , ϵ, ϵ)})
P = precedence(C)
s0 = add state(S, Sf , Rf , φ, C, P, ϵ)

8
9
10
11
12
13
14
16

for each state s ∈ S do
V : map from tag and operation RHS to register
for each symbol a ∈ Σ do
B = step on symbol(s, a)
C = epsilon closure(B)
O = transition regops(C, R, V )
P = precedence(C)
s′ = add state(S, Sf , Rf , φ, C, P, O)
δ(s, a) = (s′ , O)

17

return TDFA (Σ, T, S, Sf , s0 , R, Rf , δ, φ)

15

add state S, Sf , Rf , φ, C, P, O

X = {(q, r, l) | (q, r, , l) ∈ C}
s = (X, P )

20
21

if s ∈ S then
return s

22
23

else if ∃s′ ∈ S such that map(s, s′ , O) then
return s′

29

30
31
32
33

B : empty sequence of configurations
for (q, r, l) ∈ X ordered by q in the order of P do
if ∃(q, a, p) ∈ ∆ | a ∈ Σ then
append (p, r, l, ϵ) to B

54

return B

55

C : empty sequence of configurations

56
57

for (q, r, h, ϵ) in B in reverse order do
push (q, r, h, ϵ) on stack

58
59
60
61
62
63

while stack is not empty do
pop (q, r, h, l) from stack
append (q, r, h, l) to C
for each (q, i, t, p) ∈ ∆ ordered by priority i do
if configuration with state p is not in C then
push (p, r, h, lt) on stack

64
65

return {(q, r, h, l) in C | q = qf or
∃(q, a, ) ∈ ∆ where a ∈ Σ}

66
67
68
69
70
71
72
73
74
75
76
77
78



79

if X and X ′ have different subsets of TNFA states
or different lookahead tags for some TNFA state
or precedence is different: P ̸= P ′ then
return false

f inal regops Rf , r, l
80
81
82
83
84
85

43
44
45

for each operation i ← in O do
replace register i with M [i]
remove pair (i, M [i]) from M

46
47

for each pair (j, i) ∈ M where j ̸= i do
prepend copy operation i ← j to O

history h, t

48

return topological sort(O)

89
90

39
40

86

regop rhs r, ht , t
87
88

91

precedence C
49



92
93

return vector {q | (q, , , ) in C}



O : empty list of operations
for each tag t ∈ T do
if lt = history(l, t) ̸= ϵ then
append Rf [t] ← regop rhs(r, lt , t) to O
else
append Rf [t] ← r[t] to O
return O

41
42

35
36
37
38



if t is a multi-tag then return r[t] · ht
else return the last element of ht


switch h do
case ϵ do return ϵ
case t · h′ do return p · history(h′ )
case −t · h′ do return n · history(h′ )
case
· h′ do return history(h′ )

Algorithm 3: Determinization of TNFA (Σ, T, Q, q0 , qf , ∆).
6



O : empty list of operations
for each (q, r, h, l) ∈ C do
for each tag t ∈ T do
if ht = history(h, t) ̸= ϵ and
(history(l, t) = ϵ or t is a multi-tag) then
v = regop rhs(r, ht , t)
i = V [t][v]
if i is undefined then
V [t][v] = i = max{R} + 1
R = R ∪ {i}
if operation i ← v is not in O then
append operation i ← v to O
r[t] = i
return O

M, M ′ : empty maps from register to register
for each pair (q, r, l) ∈ X and (q, r′ , l) ∈ X ′ do
for each t ∈ T do
if history(l, t) = ϵ or t is a multi-tag then
i = r[t], j = r′ [t]
if both M [i], M ′ [j] are undefined then
M [i] = j, M ′ [j] = i
else if M [i] ̸= j or M ′ [j] ̸= i then
return false

34



transition regops C, R, V

else
add s to S
if ∃(q, r, l) ∈ X such that q = qf then
add s to Sf
φ(s) = f inal regops(Rf , r, l)
return s

map (X, P ), (X ′ , P ′ ), O



50
51
52
53

epsilon closure B



18
19

24
25
26
27
28

step on symbol (X, P ), a

<!-- page 7 -->

epsilon closure performs depth-first search following transitions from left to right. POSIX policy is more complex; we do not include pseudocode for it here, but it is extensively covered in [6].
Algorithm 3 works as follows. The main function determinization starts by allocating initial registers r0
from 1 to |T | and final registers Rf from |T | + 1 to 2|T |. It constructs initial TDFA state s0 as the ϵ-closure
of the initial configuration (q0 , r0 , ϵ, ϵ). The initial state s0 is added to the set of states S and the algorithm
loops over states in S, possibly adding new states on each iteration. For each state s the algorithm explores
outgoing transitions on all alphabet symbols. Function step on symbol follows transitions marked with a given
symbol, and function epsilon closure constructs ϵ-closure C, recording tag sequences along each fragment of
TNFA path. The set of configurations in the ϵ-closure forms a new TDFA state s′ . Function transition regops
uses the h-components of configurations in C to construct register operations on transition from s to s′ . The
same register is allocated for all outgoing transitions with identical operation right-hand-sides, but different tags
do not share registers, and vacant registers from other TDFA states are not reused (these rules ensure that there
are no artificial dependencies between registers, which makes optimizations easier without the need to construct
SSA). The new state s′ is inserted into the set of states S: function add state first tries to find an identical state
in S; if that fails, it looks for a state that can be mapped to s′ ; if that also fails, s′ is added to S. If the new state
contains the final TNFA state, it is added to Sf , and the f inal regops function constructs register operations
for the final quasi-transition which does not consume input characters and gets executed only once at the end of
match.
TDFA states are considered identical if both components (configuration set and precedence) coincide. States
that are not identical but differ only in registers can be made identical (mapped), provided that there is a bijection
between registers. Function map attempts to construct such a bijection M : for every tag, and for each pair of
configurations it adds the corresponding pair of registers to M . If either of the two registers is already mapped to
some other register, bijection cannot be constructed. For single-valued tags mapping ignores configurations that
have the tag in the lookahead sequence — every transition out of TDFA state overwrites tag value with a set
operation, making the current register values obsolete. For multi-valued tags this optimization is not possible,
because append operations do not overwrite previous values. If the mapping has been constructed successfully,
map updates register operations: for each pair of registers in M it adds a copy operation, unless the left-hand-side
is already updated by a set or append operation, in which case it replaces left-hand-side with the register it is
mapped to. The operations are topologically sorted (topological sort is defined on page 12); in the presence of
copy and append operations this is necessary to ensure that old register values are used before they are updated.
Topological sort ignores trivial cycles such as append operation i ← i · h, but if there are nontrivial cycles the
mapping is rejected (handling such cycles requires a temporary register, which makes control flow more complex
and inhibits optimizations).
After determinization is done, the information in TDFA states is erased — it is no longer needed for TDFA
execution. States are just atomic values with no internal structure. Disambiguation decisions are embedded in
TDFA; there is no disambiguation at runtime. The only runtime overhead compared to an ordinary DFA is
the execution of register operations on transitions. A TDFA may have more states than a DFA for the same
RE with all tags removed, because states that can be mapped in a DFA cannot always be mapped in a TDFA.
Minimization can reduce the number of states, especially if it is applied after register optimizations that can get
rid of many operations and make more states compatible. We focus on optimizations in section 3.
Figure 1 shows an example of TDFA construction:
➔ The RE is (1a2)∗ 3(a|4b)5b∗ . It defines language {an bm | n + m > 0} and has five tags t1 , t2 , t3 , t4 , t5 .

➔ TNFA has three kinds of transitions: bold transitions on alphabet symbols (four of them for each symbol
in the RE), thin ϵ-transitions with priority and dashed ϵ-transitions with priority and tag. Tags t1 , t2 are
under repetition, so the zero-repetition path 0 → 5 → 6 → 7 contains transitions with negative tags −t1 , −t2 .
Likewise tag t4 is in alternative, so path 8 → 9 → 10 → 13 contains transition with negative tag −t4 . Tags
t3 , t5 are in top-level concatenation and do not need negative tags.
➔ TNFA simulation on string aab consists of four steps. The first step starts with state 0. Every other step
starts with states from the previous step and follows transitions labeled with the current symbol. Each step
constructs ϵ-closure by following ϵ-paths and collecting tag sequence along the way. The value of positive
tags in the corresponding row of the closure matrix is set to the number of characters consumed so far. The
value of negative tags is set to nil n. The value of tags not in the sequence is inherited from the previous
closure. Simulation ends when all characters have been consumed. Since the last closure contains a row
with the final state 17, it is a match and the final tag values are 1, 2, 2, 2, 3.
7

<!-- page 8 -->

1/t1
1/

2

1/t2

3

1/

1
2/

0

a/

5

1/−t1

0

1/−t2

2
9
12

7

1/

a/

9

10

8

1/t3

1/t4

11

12

0 − − − −
n n 0 − −
n n 0 0 −

2
9
12
15
17

a/t2 t3 t4
a/−t4 t5
a/−t4 t5

16

2/

1/

14

1/t5

17

2/

b/

closure 1

closure 2

state t1 t2 t3 t4 t5

a/t2 t1
a/t2 t3

15

1/

1/−t4
13

2/

state t1 t2 t3 t4 t5

−t1−t2 t3
−t1−t2 t3 t4

2/

6

closure 0
t1

b/

4

a/t2 t1
a/t2 t3
a/t2 t3 t4

1 1 − − −
0 1 1 − −
0 1 1 1 −
n n 0 n 1
n n 0 n 1

a/−t4 t5
a/−t4 t5

state t1 t2 t3 t4 t5

2
9
12
15
17

2
1
1
0
0

closure 3

2 − − −
2 2 − −
2 2 2 −
1 1 n 2
1 1 n 2

state t1 t2 t3 t4 t5

b/t5

15
17

b/t5

1 2 2 2 3
1 2 2 2 3

r12 ← r11
r11 ← p
r13 ← p
r14 ← p
TDFA state 1
state t1

2
9
12
15
17

a/r11 ← p
TDFA state 0
state t1 t2 t3 t4 t5

2
9
12

t3

t4 t5

r11 r2 r3 r4 r5 t2 t1
r11 r2 r3 r4 r5 t2 t3
r11 r2 r3 r4 r5 t2 t3 t4
r12 r13 r14 r4 r5 −t4 t5
r12 r13 r14 r4 r5 −t4 t5

TDFA state 2
state t1

15
17

t5

la

r12 r13 r14 r15 r5
r12 r13 r14 r15 r5

t5
t5

t2

t3

t4

b/r20 ← p
r6 ← r12
r7 ← r13
r8 ← r14
r9 ← r15
r10 ← p

state t1

2
9
12
15
17

a/r17 ← p
r18 ← p

b/r17 ← p
r18 ← p
r19 ← p

r6 ← r12
r7 ← r13
r8 ← r14
r9 ← n
r10 ← p
r12 ← r11
r13 ← p
r14 ← p
r15 ← p

mapped to state 1

a/r16 ← p
r17 ← p

la

a/r12 ← n
r13 ← n
r14 ← p

la

r1 r2 r3 r4 r5 t1
r1 r2 r3 r4 r5 −t1 −t2 t3
r1 r2 r3 r4 r5 −t1 −t2 t3 t4

b/r12 ← n
r13 ← n
r14 ← p
r15 ← p

t2

t2

t3

la

t4 t5

r16 r17 r3 r4 r5 t2 t1
r16 r17 r3 r4 r5 t2 t3
r16 r17 r3 r4 r5 t2 t3 t4
r11 r17 r18 r4 r5 −t4 t5
r11 r17 r18 r4 r5 −t4 t5
mapped to state 2

state t1

15
17

t5

la

r11 r17 r18 r19 r5
r11 r17 r18 r19 r5

t5
t5

t2

t3

t4

TDFA state 3
state t1

15
17

t2

t3

t4

t5 la

r12 r13 r14 r15 r20
r12 r13 r14 r15 r20

r6 ← r12
r7 ← r13
r8 ← r14
r9 ← r15
r10 ← r20

b

mapped to state 3
a/r12 ← r11
r11 ← p
r13 ← p
r14 ← p
a/r11 ← p
r12 ← n
r13 ← n
r14 ← p

0

1
r6 ← r12
r7 ← r13
r8 ← r14
r9 ← n
r10 ← p

state t1

15
17

t2

t3

t4

t5 la

r12 r13 r14 r15 r20
r12 r13 r14 r15 r20

b/r12 ← r11
r13 ← p
r14 ← p
r15 ← p
b
2

b/r12 ← n
r13 ← n
r14 ← p
r15 ← p

r6 ← r12
r7 ← r13
r8 ← r14
r9 ← r15
r10 ← p

b/r20 ← p

3
r6 ← r12
r7 ← r13
r8 ← r14
r9 ← r15
r10 ← r20

Figure 1: Example for a RE (1a2)∗ 3(a|4b)5b∗ : TNFA, simulation on string aab, determinization, TDFA.

8

<!-- page 9 -->

➔ The match is ambiguous: it is possible to match aab by following path 0 ⇝ 2 ⇝ 2 ⇝ 12 ⇝ 17 (let the greedy
repetition consume aa) but it is also possible to follow path 0 ⇝ 2 ⇝ 9 ⇝ 15 ⇝ 17 (let the greedy repetition
consume only the first a). The first match is preferable by both POSIX and leftmost-greedy policies.
➔ Determinization is similar to simulation, but TDFA states store registers instead of offsets. This solves the
problem of mapping states that differ only in tag values: for example, closures 1 and 2 cannot be mapped,
although they have identical states and tag sequences, but TDFA state corresponding to closure 2 is mapped
to state 1. This is possible due to the register operations on the dashed backward transition. Note that
there is one copy operation r12 ← r11 , but other copy operations for r11 , r13 , r14 are combined with set
operations, e.g. r11 ← p is the combination of r16 ← p and r11 ← r16 (see lines 43 – 45 of algorithm 3).
➔ Unlike simulation, determinization does not immediately apply tag sequences to registers. Instead, it
stores them as part of TDFA state (in the lookahead column, shortened as la). Compare tag sequences on
transitions to closures 0, 1, 2 to that in states 0, 1, 2 respectively — these are the same tags. Lookahead
tags form register operations on the outgoing transitions: e.g. lookahead tag t1 in the first row of TDFA
state 0 (corresponding to tagged TNFA transition 1 → 2) forms operation r11 ← p.
➔ For every distinct set or append operation transition regops allocates a new register and stores the updated
tag value in it. Note that it would be impossible to reuse the same register (e.g. to have r1 ← p instead
of r11 ← p on transition from state 0 to 1) because there may be conflicting operations (e.g. r12 ← n
for lookahead tag −t1 ). Therefore tag t1 in TDFA state 1 is represented with two registers r11 and r12 ,
reflecting the fact that state 1 may be reached by different TNFA paths with conflicting submatch values.
➔ Final TDFA states are all states containing TNFA state 17 (i.e. states 1, 2 and 3). In addition to normal
transitions final TDFA states have quasi-transitions that set final registers r6 – r10 . These quasi-transitions
do not consume any symbol, and the operations on them are executed once at the end of match.
➔ In the resulting TDFA all internal structure in the states is erased, leaving atomic states with transitions
and register operations. Registers can be renamed to occupy consecutive numbers, and the number of
registers and operations can be reduced (see section 3).

3

Implementation

In this section we describe optimizations and practical details that should be taken into account when implementing TDFA. None of the optimizations is particularly complex or vital for TDFA operation, but applied
together and in the correct order they can make TDFA considerably faster and smaller.

3.1

Multi-valued tags

The most straightforward representation of multi-valued tags is a vector of offsets. It is very inefficient
because copy and append operations need to copy entire vectors (which could grow arbitrarily long). A more
efficient representation is a prefix tree. It is possible because tag sequences in the operations map on the path
tree constructed by the ϵ-closure. The tree can be stored as an array of nodes (pred, offs) where pred is the index
of a predecessor node, and offs is a positive or negative tag value. Individual sequences in the tree are addressed
by integer indices of tree nodes (zero index corresponds to the empty sequence). This representation is space
efficient (common prefixes are shared), but most importantly it makes copy operations as simple as copying scalar
values (tree indices). Append operations are more difficult, as they require a new slot (or a couple of slots) in the
prefix tree. However, if the backing array is allocated in large chunks of memory, then the amortized complexity
of each operation is constant. This representation was used by multiple researches, e.g. Karper describes it as
the flyweight pattern [12].

3.2

Fallback operations

In practice it is often necessary to match the longest possible prefix of a string rather than the whole string.
After matching a short prefix, TDFA may attempt to match a longer prefix. If that fails, it must fallback to the
previous final state and restore the input position accordingly. A final state is also a fallback state if there are
non-accepting paths out of it, and a path is non-accepting if does not go through another final state (which may
happen either because the input characters do not match or due to a premature end of input).

9

<!-- page 10 -->

For an ordinary DFA the only information that should be saved in a fallback state is the input position. For
TDFA it is also necessary to backup registers that may be clobbered on the non-accepting paths from the fallback
state. Backup operations should be added on transitions out of the fallback state, and restore operations should
be added on the fallback quasi-transition, which replaces the final quasi-transition for fallback paths. Final
registers can be reused for backups, as by TDFA construction they are used only on the final quasi-transitions.
Backup registers are only needed for copy and append operations (set operations do not depend on registers).
fallback regops



1

ψ : undefined fallback function

2
3

for each fallback state s ∈ S do
O : empty list of register operations
for each operation on quasi-transition φ(s) do
if append i ← j · h and j is clobbered then
backup regops(s, i, j)
append operation i ← i · h to O
else if copy i ← j and j is clobbered then
backup regops(s, i, j)
else
append a copy of this operation to O

4
5
6
7
8
9
10
11

14
15
16
17



for each alphabet symbol a ∈ Σ do
(s′ , O) = δ(s, a)
if exist non-accepting paths from s′ then
append copy operation i ← j to O

ψ(s) = O

12
13

backup regops s, i, j

return ψ

Algorithm 4: Adding fallback operations to TDFA (Σ, T, S, Sf , s0 , R, Rf , δ, φ).
Algorithm 4 shows how to add such operations. It assumes that fallback states and clobbered registers for
each fallback state have already been identified. This can be done as follows. First, augment TDFA with a
default state that makes transition function δ total (if a premature end of input is possible, add a quasi-transition
from non-final states to the default state). Then compute reachability of the default state by doing backward
propagation from states that have transitions to it. If the default state is reachable from a final state, then it is a
fallback state. Clobbered registers can be found by doing depth-first search from a fallback state, visiting states
from which the default state is reachable, and accumulating left-hand-sides of register operations.

3.3

Register optimizations

TDFA induces a control flow graph (CFG) with three kinds of nodes:
• basic blocks for register operations on symbolic transitions
• final blocks for final register operations
• fallback blocks for fallback register operations
There is an arc between two blocks in CFG if one is reachable from another in TDFA without passing through
other register operations. Additionally, fallback blocks have arcs to all blocks reachable by TDFA paths that
may fall through to these blocks. Figure 2 shows CFG for the example from section 2.
CFG represents a program on registers, so the usual compiler optimizations can be applied to it, resulting in
significant reduction of registers and operations. RE2C uses the following optimization passes for the number of
repetitions N = 2 (pseudocode is given by the algorithms 5 and 6):
1. Compaction
2. Repeat N times:
a. Liveness analysis
b. Dead code elimination
c. Interference analysis
d. Register allocation with copy coalescing
e. Local normalization
Compaction pass is applied only once immediately after determinization. It renames registers so that they
occupy contiguous range of numbers with no “holes”. This is needed primarily to allow other optimization passes
use registers as indices in liveness and interference matrices.
10

<!-- page 11 -->


optimizations G
1
2
3
4
5
6
7
8
9

V = compaction(G)
G = renaming(G, V )
for i = 1, 2 do
L = liveness analysis(G)
dead code elimination(G, L)
I = interference analysis(G, L)
V = register allocation(G, I)
renaming(G, V )
normalization(G)

renaming G, V
10
11
12
13
14
15


compaction G



for each block b in G do
for each operation in b do
if set operation i ← v then
rename i to V [i]
if copy or append operation i ← j... then
rename i to V [i] and j to V [j]

L : boolean matrix indexed by blocks and registers

17
18
19

for each block b in G do
for each register i in G do
L[b][i] = false

20
21
22

for each final block b in G do
for each final register i in G do
L[b][i] = true

23
24
25
26
27
28
29
30
31
32
33
34
35

while true do
f ix = true
for each basic block b in G in post-order do
Lb = copy of row L[b]
for each successor s of block b do
Ls = copy of row L[s]
for each operation in s in post-order do
if set operation i ← v then
Ls [i] = false
if copy operation i ← j then
if Ls [i] then
Ls [i] = false
Ls [j] = true

36
37

for each register i in G do
Lb [i] = Lb [i] ∨ Ls [i]

38
39
40
41
42
43
44

56
57
58
59
60
61
62
63
64
65
66

for each register i in G do
U [i] = false
for each block b in G do
for each operation in b do
if set operation i ← v then
U [i] = true
if copy or append operation i ← j... then
U [i] = U [j] = true
n=0
for registers i in G such that U [i] do
n = n + 1, V [i] = n

67

return V

68
69
70
71
72
73
74
75
76
77

I : boolean matrix indexed by registers
V : vector of histories indexed by registers

80
81
82

for each register i in G do
for each register j in G do
I[i][j] = I[j][i] = false

83
84
85

for each block b in G do
for each copy or append operation i ← j... in b do
V [j] = j

94
95
96
97

for operations k ←
Ib [k] = false

98
99

for registers k in G such that Ib [k] do
I[i][k] = I[k][i] = true

100
101
102

for registers i in G not used in append operations do
for registers j in G used in append operations do
I[i][j] = I[j][i] = true

103

return I

91
92
93

Lb = copy of row L[b]
for each operation i ← in b do
Lb [i] = false
for each copy or append operation
Lb [j] = true

50
51
52

for each block s in G that may fall through to b do
for each register i in G do
L[s][i] = L[s][i] or Lb [i]

← j... in b do

for each operation in b do
Ib = copy of row L[b]
if set operation i ← v then
V [i] = v
Ib [i] = false
else if copy operation i ← j then
V [i] = V [j]
Ib [i] = Ib [j] = false
else if append operation i ← j · h then
V [i] = V [j] · h

88
89
90

for each fallback block b in G do
for each final register i in G do
L[b][i] = true

return L

Algorithm 5: Register optimizations (part 1).
11



78
79

86
87

if L[b] ̸= Lb then
L[b] = Lb
f ix = false
if f ix then break



for each basic block b in G do
Lb = copy of row L[b]
for each operation i ← in b in post-order do
if Lb [i] then
if set operation i ← v then
Lb [i] = false
if copy operation i ← j then
Lb [i] = false
Lb [j] = true
else remove dead operation

interference analysis G, L

45
46
47
48
49

53

U : boolean vector indexed by registers
V : integer vector indexed by registers

dead code elimination G, L


liveness analysis G
16

54
55

in b with V [k] = V [i] do

<!-- page 12 -->

register allocation G, I




normalization G

1
2
3

V : vector of registers indexed by registers
B : vector of registers indexed by registers
S : vector of register sets indexed by registers

4
5
6

for each register i in G do
B[i] = −1
S[i] = ∅

7
8
9
10
11
12
13

18
19
20
21

for each block b in G do
for each operation in b do
if copy or append i ← j... and i ̸= j then
x = B[i], y = B[j]
if x = −1 and y = −1 and ¬I[i][j] then
B[i] = B[j] = i
S[i] = {i, j}
else if x ̸= −1 and y = −1 then
if ∀k ∈ S[x] : ¬I[k][j] then
B[j] = x
S[x] = S[x] ∪ {j}
else if x = −1 and y ̸= −1 then
if ∀k ∈ S[y] : ¬I[k][i] then
B[i] = y
S[y] = S[y] ∪ {i}

22
23
24
25
26
27

for registers i in G such that B[i] = i do
for registers j in G such that B[j] = j and j > i do
if ∀i ∈ S[x], j ∈ S[y] : ¬I[i][j] then
B[y] = x
S[x] = S[x] ∪ S[y]
S[y] = ∅

28
29
30
31
32
33
34

for registers i in G such that B[i] = −1 do
if ∃j in G : B[j] = j and ∀k ∈ S[j] : ¬I[i][k] then
B[i] = j
S[j] = S[j] ∪ {i}
else
B[i] = i
S[i] = {i}

35
36
37
38
39

n=0
for registers i in G such that B[i] = i do
n=n+1
for registers j ∈ S[i] do
V [j] = n

40

return V

14
15
16
17

41
42
43
44
45
46
47
48
49

for each block b in G do
for each contiguous set operation range O do
remove duplicates(O)
sort(O)
for each contiguous copy operation range O do
remove duplicates(O)
topological sort(O)
for each contiguous append operation range O do
remove duplicates(O)

topological sort O



50
51
52
53
54

I : vector of in-degree indexed by registers
for each copy or append operation i ← j... in O do
I[i] = I[j] = 0
for each copy or append operation ← j... in O do
I[j] = I[j] + 1

55
56

O′ : empty list of operations
nontrivial cycle = false

66
67

while O is not empty do
for each operation i ← in O do
if I[i] = 0 then
remove operation from O, append to O′
if this is a copy/append operation i ← j... then
I[j] = I[j] − 1
if nothing added to O′ but O is not empty then
if ∃ operation i ← j... in O with i ̸= j then
nontrivial cycle = true
append O to O′
break (only cycles left)

68
69

O = O′
return ̸ nontrivial cycle

57
58
59
60
61
62
63
64
65

remove duplicates O
70
71
72



for each operation o in O do
for each subsequent operation o′ = o in O do
remove duplicate operation o′

Algorithm 6: Register optimizations (part 2).

Liveness analysis builds a boolean 2-dimensional matrix indexed by CFG blocks and registers. A cell L[b][i]
is true iff register i is alive in block b (meaning that its value is used). Function liveness analysis uses iterative
data-flow approach. Initially only the final registers in the final blocks are alive. The algorithm iterates over
CFG blocks in post-order, expanding the live set, until it reaches a fix point. Lastly it marks backup registers as
alive in all blocks reachable from fallback blocks by non-accepting paths.
Dead code elimination removes operations whose left-hand-side register is not alive.
Interference analysis builds a boolean square matrix indexed by registers. A cell I[i][j] is true iff registers i
and j interfere with each other (their lifetimes overlap, so they cannot be represented with one register). Initially
none of the registers interfere. Function interference analysis considers each CFG block b and inspects each
register j used on the right-hand-side of an operation: all registers alive in block b interfere with j, except for
registers that have the same value (tracked by the vector V ). Finally registers for multi-valued and single-valued
tags are marked as interfering with each other.
Register allocation partitions registers into equivalence classes. Registers inside of one class do not interfere
with each other, so they can all be replaced with a single representative. Initially none of the registers belongs
12

<!-- page 13 -->

to any class. Function register allocation loops over copy operations and tries to put source and destination
into one class (so that the copy can be removed). Vector B maps registers to their representative, and vector S
maps representatives to their class. The algorithm tries to merge non-interfering equivalence classes, and then
puts the remaining registers into an non-interfering class (allocating a new class if necessary). Finally it maps
representatives to consecutive numbers and stores them in the V vector. The constructed partitioning is not
minimal, but it’s a good approximation, since finding the minimal clique cover of a graph is NP-complete.
Local normalization reconciles operations after previous passes. The normalization function removes duplicate operations that might appear after different registers are collapsed into one. It also sorts operations, so that
operation sequences could be compared easily (which is used in further optimizations like in minimization). Each
continuous range of set, copy or append operations is handled separately, because operations of different kinds
should not be reordered (that could change the end result).
Figure 2 is a continuation of example on figure 1:
➔ The CFG contains 9 blocks: basic blocks 0 – 5 (one for each tagged transition on symbol in TDFA, plus
the start block 0) and final blocks 6 – 8 (one per each final quasi-transition). There are no fallback blocks
in this example, because there are no fallback states in TDFA: every transition out of a final state goes to
another final state, so the attempt to match a longer string will either fail immediately (before leaving the
final state), or it will succeed immediately.
➔ The second CFG is after compaction and the first pass of liveness and interference analysis. Compaction
renames registers r6 – r15 and r20 to r1 – r11 , reducing the size of the register range from 20 to 11 and
the size of the liveness and interference matrices almost 2x and 4x respectively. Liveness information is
shown at the top of each block. Interference matrix uses asterisk for interfering register pairs and dot
for non-interfering ones. It can be seen that there are many dots in the table, which means optimization
opportunities. The interference matrix is symmetrical, as the interference relation is commutative.
➔ The third CFG is after the second pass of liveness and interference analysis. The number of registers is
reduced from 11 to 5. Many operations have been eliminated, for example the copy operation r1 ← r7 in the
final block 8 of the second CFG was removed by copy coalescing, because registers 1 and 7 did not interfere
and register allocation put them in one equivalence class (and likewise for the other copy operations in
block 8). In basic blocks 1 and 3 set operations r6 ← p and r9 ← p were collapsed into r3 ← p after
non-interfering registers r6 and r9 had been renamed to r3 . Interference matrix has dots only on the main
diagonal (a register does not interfere with itself), which leaves no room for further optimization.
➔ The resulting optimized TDFA is at the bottom of figure 2. The final registers are now r1 – r5 .

3.4

Minimization

Minimization can be applied to TDFA in the same way as to an ordinary DFA (e.g. the Moore’s algorithm),
except that transitions on the same alphabet symbol but with different register operations should be treated as
different transitions, so their destination states cannot be merged. To get optimal performance minimization
algorithm should be able to compare operations on transitions in constant time. This is possible if operation
sequences are inserted into a hash map and represented with unique numeric identifiers. Such a comparison may
have false negatives, as non-identical operations lists may be equivalent (e.g. r1 ← p, r2 ← n is not identical, but
equivalent to r2 ← n, r1 ← p). False negatives do not affect minimization correctness, but the end result may
be suboptimal. To avoid that, minimization should be applied after register optimizations (which may remove
some register operations) and most importantly after normalization (defined on page 12).

3.5

Fixed tags

Fixed tags is a very important optimization that happens at the RE level. In cases with high tag density (such
as POSIX REs with nested submatch groups) this optimization alone may be more effective than all register
optimizations combined. The key observation is, if a pair of tags is within fixed distance from each other, there
is no need to track both of them: the value of one tag can be computed from the value of the other tag one by
adding a fixed offset. This optimization is fast (linear in the size of a RE) and has the potential to reduce both
TDFA construction time and matching time.
Algorithm 7 finds fixed tags by performing top-down structural recursion on a RE. It has four parameters: e
is the current sub-RE, t is the current base tag, d is the distance to base tag, and k is the distance to the start
13

<!-- page 14 -->

basic block 2
r12 ← n
r13 ← n
r14 ← p
r15 ← p

basic block 0

basic block 1
r11 ← p
r12 ← n
r13 ← n
r14 ← p

basic block 3
r12 ← r11
r11 ← p
r13 ← p
r14 ← p

final block 6
r6 ← r12
r7 ← r13
r8 ← r14
r9 ← n
r10 ← p
basic block 4
r12 ← r11
r13 ← p
r14 ← p
r15 ← p

live : r7 r8 r9 r10
r7 ← n
r8 ← n
r9 ← p
r10 ← p

live : ∅

live : r6 r7 r8 r9
r6 ← p
r7 ← n
r8 ← n
r9 ← p

live : r6 r7 r8 r9
r7 ← r6
r6 ← p
r8 ← p
r9 ← p

live : r1 r2 r3 r4 r5
r1 ← r7
r2 ← r8
r3 ← r9
r4 ← n
r5 ← p
live : r7 r8 r9 r10
r7 ← r6
r8 ← p
r9 ← p
r10 ← p

live : r1 r2 r3 r4
r1 ← n
r2 ← n
r3 ← p
r4 ← p

live : ∅

live : r1 r2 r3
r1 ← n
r2 ← n
r3 ← p

live : r1 r2 r3
r 1 ← r3
r2 ← p
r3 ← p

basic block 5
r20 ← p

final block 8
r6 ← r12
r7 ← r13
r8 ← r14
r9 ← r15
r10 ← r20

final block 7
r6 ← r12
r7 ← r13
r8 ← r14
r9 ← r15
r10 ← p

live : r7 r8 r9 r10 r11
r11 ← p

live : r1 r2 r3 r4 r5
r1 ← r7
r2 ← r8
r3 ← r9
r4 ← r10
r5 ← p

live : r1 r2 r3 r4 r5
r5 ← p

live : r1 r2 r3 r4 r5
r1 ← r7
r2 ← r8
r3 ← r9
r4 ← r10
r5 ← r11
Interference
r1 r2 r3 r4 r5 r6 r7 r8 r9 r10r11
r1 . ∗ ∗ ∗ ∗ . . ∗ ∗ ∗ ∗
r2 ∗ . ∗ ∗ ∗ . . . ∗ ∗ ∗
r3 ∗ ∗ . ∗ ∗ . . . . ∗ ∗
r4 ∗ ∗ ∗ . ∗ . . . . . ∗
r5 ∗ ∗ ∗ ∗ . . . . . . .
r6 . . . . . . ∗ ∗ . . .
r7 . . . . . ∗ . ∗ ∗ ∗ ∗
r8 ∗ . . . . ∗ ∗ . ∗ ∗ ∗
r9 ∗ ∗ . . . . ∗ ∗ . . ∗
r10 ∗ ∗ ∗ . . . ∗ ∗ . . ∗
r11 ∗ ∗ ∗ ∗ . . ∗ ∗ ∗ ∗ .

live : r1 r2 r3 r4 r5

live : r1 r2 r3 r4 r5
r4 ← n
r5 ← p
live : r1 r2 r3 r4
r1 ← r3
r2 ← p
r3 ← p
r4 ← p

Interference
r 1 r2 r3 r4 r5
r1 . ∗ ∗ ∗ ∗
r2 ∗ . ∗ ∗ ∗
r3 ∗ ∗ . ∗ ∗
r4 ∗ ∗ ∗ . ∗
r5 ∗ ∗ ∗ ∗ .

live : r1 r2 r3 r4 r5
r5 ← p

a/r1 ← r3
r2 ← p
r3 ← p
a/r1 ← n
r2 ← n
r3 ← p

1
r4 ← n
r5 ← p

0

b/r1 ← r3
r2 ← p
r3 ← p
r4 ← p
2

b/r1 ← n
r2 ← n
r3 ← p
r4 ← p

b
b/r5 ← p

3

r5 ← p

Figure 2: Register optimizations for TDFA on figure 1.
Top to bottom: initial CFG, CFG after compaction with per-block liveness information and interference table,
CFG on the second round of optimizations, optimized TDFA with final registers r1 to r5 .
14

<!-- page 15 -->

of the current level. Levels are parts of a RE where any two points either both match or both do not match. A
level increases on recursive descent into alternative or repetition subexpressions, but not concatenation. Tags on
different levels should not be fixed on each other, even if they are within fixed distance on any path that goes
through both of them, because there are paths that go through only one tag (so the other one is nil). Tag value
−1 denotes the absence of base tag: when descending to the next level initially there is no base tag, and the first
tag on the current level becomes the base. One exception is the top level, where the initial base tag should be a
special value denoting the rightmost position (which is always known at the end of the match). The algorithm
recursively returns the new base tag, the updated distance to base tag, and the updated level distance. Special
distance value NaN (not-a-number) is understood to be a fixed point in arithmetic expressions: any expression
involving NaN amounts to NaN .
f ixed tags e, t, d, k
1
2

if e = ϵ then
return t, d, k

3
4

else if e = a ∈ Σ then
return t, d + 1, k + 1

5
6
7
8
9
10

else if e = e1 |e2 then
, , k1 = f ixed tags(e1 , −1, NaN, 0)
, , k2 = f ixed tags(e2 , −1, NaN, 0)
if k1 = k2 then
return t, d + k1 , k + k1
return t, NaN, NaN

11
12
13
14

else if e = e1 e2 then
t2 , d2 , k2 = f ixed tags(e2 , t, d, k)
t1 , d1 , k1 = f ixed tags(e1 , t2 , d2 , k2 )
return t1 , d1 , k1

15
16
17
18
19

else if e = en,m
then
1
, , k1 = f ixed tags(e1 , −1, NaN, 0)
if n = m then
return t, d + n ∗ k1 , k + n ∗ k1
return t, NaN, NaN

20
21
22
23
24

a/r1 ← p



1
a/r1 ← n

r2 ← n
r3 ← p

0

b/r1 ← p
r2 ← p
2

b/r1 ← n
r2 ← p

b
b/r3 ← p

3

r3 ← p

Figure 3: Optimized TDFA with fixed tags
t1 ← (n if t2 = n else t2 − 1) and t3 ← (t5 − 1).
Tags t2 , t4 , t5 correspond to final registers r1 , r2 , r3 .

else if e = t1 ∈ T then
if t ̸= −1 and d ̸= NaN then
mark t1 as fixed on t with distance d
return t, d, k
return t1 , 0, k

Algorithm 7: Fixed tags optimization.
Figure 3 shows the effect of fixed tags in addition to other optimizations on figure 2:
➔ In the example RE (1a2)∗ 3(a|4b)5b∗ tags t1 and t2 are within one symbol from each other, so the value of
t1 can be computed as nil if t2 is nil, or t2 − 1 otherwise. Likewise t3 can be computed as t5 − 1 (although
there are multiple different paths through (a|4b), they all have the same length).
➔ Fixed tags are identified at the RE level and excluded from TNFA construction and determinization. There
are no registers and register operations associated with t1 and t3 , except for computing their values from
the base tags t2 and t5 at the end of match.

4

Multi-pass TDFA

TDFA with registers described in section 2 are well suited for ahead-of-time determinization (e.g. in lexer
generators) when one can spend considerable time on optimizations in order to emit a more efficient TDFA.
However, in RE libraries the overhead on determinization and optimizations is included in the run time, therefore
it is desirable to reduce TDFA construction time (although the overhead may be amortized if a RE is compiled
once and used to match many strings).
Another concern is the density of submatch information in a RE. TDFA with registers are perfect for sparse
submatch extraction, when the number of tags is small compared to the size of RE and the runtime performance is
15

<!-- page 16 -->

expected be close to an ordinary DFA. However, if a RE contains many tags (in the extreme, if every subexpression
is tagged) then transitions get cluttered with operations, making TDFA execution very slow. Moreover, the
optimizations described in section 3 become problematic due to the size of liveness and interference tables.
Multi-pass TDFA address these issues: they reduce TDFA construction time and they are better suited to
dense submatch extraction. The main difference with canonical TDFA is that multi-pass TDFA have no register
operations. Instead, as the name suggests, they have multiple passes: a forward pass that matches the input
string and records a sequence of TDFA states, and one or more backward passes that iterate through the recorded
states and collect submatch information (one backward pass is sufficient, but an extra pass may be used e.g. to
estimate and preallocate memory for the results). The representation of submatch results may vary and affects
only the backward pass(es); forward pass is the same for all representations.
Multi-pass TDFA construction differs from algorithm 3 in section 2 in a few ways. Recall that a closure
C corresponds to a TDFA transition between states s and s′ . For multi-pass TDFA closure configurations are
extended to five components (q, o, r, h, l) where the new component o is the origin TNFA state in s that leads
to state q in s′ , and the remaining components are as in algorithm 3. Origins are needed to trace back the
matching TNFA path from a sequence of TDFA states. Path fragments corresponding to closure configurations
are represented with backlinks, and every TDFA transition is associated with a backlink array. A backlink is a
pair (i, h) where i is an index in backlink arrays on preceding transitions, and h is a tag sequence corresponding
to the h-component of a configuration. Backlinks on transitions do not map one-to-one to configurations, because
in TDFA(1) contrary to TDFA(0) configurations with identical origins have identical h-components (inherited
from the lookahead tags), resulting in identical backlinks. To deduplicate such backlinks, unique origins in
unique origins C

construct backlinks C, U, U ′



1
2

U : mapping from TNFA states in C to integers
i=0

3
4
5
6

for each unique origin o in C do
for each (q, o′ , , , ) in C such that o′ = o do
U [q] = i
i=i+1

7

return U

match F , a1 . . . an
8
9
10
11
12
13

28

B : backlink array of size |range(U ′ )|

29
30
31
32

for each (q, o, , h, ) in C do
i = U ′ [q]
if B[i] is undefined then
B[i] = (U [o], h)

33

return B

extract tstring F, a1 . . . an , s0 . . . sn



V = {s0 }
for k = 1, n do
if s = δ(s, ak ) is defined then
append s to V
else return ∅
return V

extract offsets F, a1 . . . an , s0 . . . sn





|T |

34
35
36
37
38
39
40



(i, h) = φ(sn )
x=h
for k = n, 1 do
( , B) = δ(sk−1 , ak )
(i, h) = B[i]
x = h · ak · x
return x

extract offset lists F, a1 . . . an , s0 . . . sn



|T |

14
15
16

E = {∅}i=1 (no value for each tag)
(i, h) = φ(sn )
k=n

41
42
43

E = {{}}i=1 (empty list for each tag)
(i, h) = φ(sn )
k=n

17
18
19
20
21
22

44
45
46
47
48
49

23
24
25
26

while true do
for tag t in h in reverse order do
if t > 0 and E[t] = ∅ then
E[t] = k
else if E[−t] = ∅ then
E[−t] = −1
if k = 0 then break
( , B) = δ(sk−1 , ak )
(i, h) = B[i]
k =k−1

50
51
52
53

while true do
for tag t in h in reverse order do
if t > 0 then
prepend k to E[t]
else
prepend −1 to E[−t]
if k = 0 then break
( , B) = δ(sk−1 , ak )
(i, h) = B[i]
k =k−1

27

return E

54

return E

Algorithm 8: Backlink construction and matching with multi-pass TDFA F = (Σ, T, S, Sf , s0 , δ, φ).
16

<!-- page 17 -->

algorithm 8 creates a per-state mapping from origin state to a unique origin index. Transition function is defined
as δ(s, a) = (s′ , B) where B = construct backlinks(C, U, U ′ ) and U , U ′ are the unique origin mappings for s
and s′ respectively. Final TDFA states are associated with a single backlink, and the final function is defined
as φ(s) = (i, l) where i is the unique origin index of the final state qf in C and l is the lookahead tag sequence
(l-component of the final configuration). The resulting TDFA has no registers or register operations; the R and
Rf components are removed from TDFA, and functions transition regops, f inal regops and their dependencies
in algorithm 3 are not needed.
Algorithm 8 shows matching with a multi-pass TDFA. The forward pass is defined by the function match,
which executes TDFA on a string a1 . . . an and returns the matching sequence of TDFA states s0 . . . sn (or ∅ on
failure). Backward pass depends on the representation of submatch results; we provide three variants for offsets,
offset lists and tagged strings. In each case the backward pass follows a sequence of backlinks from the final state
to the initial state. Function extract offsets extracts the last offset for each tag and avoids overwriting it by
initializing all offsets to ∅ and checking each offset before writing. Function extract offset lists is similar, but it
collects offsets into lists. Function extract tstring concatenates the h-components of backlinks interleaved with
input symbols (this representation can be used to reconstruct a full parse tree, see [6] section 6).
In practice we found that the following details affect performance of algorithm 8. The h-components of
backlinks should be stored as arrays which allow fast access to individual tags, rather than linked lists packed
in a prefix tree (the latter representation was used for multi-valued tags in section 3). The forward pass should
record references to backlink arrays instead of TDFA states in order to reduce the number of indirections and
lookups. For tagged strings a separate backward pass may be used to estimate the amount of space for the
resulting string and preallocate it. If tags in a RE have nested structure (e.g. in the case of POSIX capturing
groups) then negative transition should be added only for the topmost tag of a subexpression (as described in
[6] section 9) rather than for all nested tags (as described in algorithm 2). The mapping from a tag to its nested
tags should be stored separately and used during matching (as in [6] section 6).
Figure 4 shows multi-pass TDFA for the running example (compare it with figure 1):
➔ TDFA transition 0 → 1 has two backlinks because the five configurations in state 1 originate in two TNFA
states 2 and 9. Likewise transition 1 → 1 has two backlinks corresponding to origins 2 and 9, transitions
0 → 2 and 1 → 2 have one backlink corresponding to origin 12, and transitions 2 → 3 and 3 → 3 have one
backlink corresponding to origin 15.
➔ Final states 1, 2 and 3 have a backlink corresponding to the final configuration with TNFA state 17.
➔ The i-component of each backlink equals to the unique origin index of the configuration o-component.
For example, both backlinks on transition 1 → 1 have i = 0 because their configurations (in the shadow
TDFA state mapped to state 1 on figure 1) have origins 2 and 9 in TDFA state 1, which have the same
unique origin index 0 (because they both have origin 2 in TDFA state 0). Consequently, both backlinks on
transition 1 → 1 connect to the first backlink on transitions 0 → 1 and 1 → 1. On the other hand, the final
backlink in state 1 connects to the second one.

a/{(0, t2 t1 ),
(0, t2 t3 )}

1 : (1, −t4 t5 )

a/{(0, t1 ),
(0, −t1−t2 t3 )}

b/{(0, t2 t3 t4 )}

0:∅

b/{(0, −t1−t2 t3 t4 )}

0
(0, t1 )

a

a

1
(0, t2 t1 )

2 : (0, t5 )

a

1
(0, t2 t3 t4 )

a

b/{(0, )}
b/{(0, t5 )}

b
b

3 : (0, )

2
(0, t5 )

Figure 4: Multi-pass TDFA for RE (1a2)∗ 3(a|4b)5b∗ matching string aab.
17

<!-- page 18 -->

➔ The sequence of TDFA states matching aab is 0 → 1 → 1 → 2, and backlinks can be traced back from the
final backlink in state 2 using i-component as index in backlink arrays.
➔ Submatch results for string aab are as follows. Single offsets: t1 = 1, t2 = 2, t3 = 2, t4 = 2, t5 = 3. Offset lists:
t1 = {0, 1}, t2 = {1, 2}, t3 = {2}, t4 = {2}, t5 = {3}. Tagged string: 1 a 2 1 a 2 3 4 b 5.

5

Evaluation

In this section we evaluate TDFA performance in practice and compare it to other algorithms. We present
three groups of benchmarks that cover different settings and show different aspects of the algorithm:
1. AOT determinization (figures 5, 6, 7, 8). We compare three lexer generators: RE2C [4], Ragel [14] and
Kleenex [13] that are based on different types of deterministic automata. All of them generate optimized C
code, which is further compiled to binary by GCC and Clang. The generated programs do string rewriting:
they read 100MB of input text and insert markers at submatch extraction points. These benchmarks use
leftmost-greedy disambiguation. The following automata are compared:
• TDFA(1), the algorithm described by Trafimovich in [3] and presented in this paper. It is implemented in RE2C with the optimizations described in section 3.
• TDFA(0), the original algorithm described by Laurikari in [1]. Contrary to TDFA(1) that use
one-symbol lookahead, TDFA(0) do not use lookahead: the two types of automata are called so by
analogy with LR(1) and LR(0). TDFA(0) apply register operations to the incoming transition, while
TDFA(1) split them on the lookahead symbol and apply them to the outgoing transitions, which
reduces the number of tag conflicts. As a consequence, TDFA(0) typically require more registers and
copy operations, which makes them slower than TDFA(1); see [3] for a detailed comparison. TDFA(0)
algorithm is also implemented in RE2C and benefits from the same optimizations as TDFA(1).
• StaDFA, the algorithm described by Chowdhury in [11], with a few modifications of our own that
were necessary for correctness. It is very similar to TDFA, but the automata have register operations
in states rather than on transitions (which implies that staDFA do not use lookahead). The algorithm
is implemented in RE2C and uses the same optimizations as TDFA(1) and TDFA(0).
• DSSTs, the algorithm described by Grathwohl in [8]. DSSTs stands for Deterministic Streaming
String Transducers; these are more distant relatives to TDFA, better suited to string rewriting and
full parsing. DSST states contain path trees constructed by the ϵ-closure, while TDFA states contain
similar information decomposed into register tables and lookahead tags. DSST registers contain fragments of strings over the output alphabet (the analogue of our tagged strings). Register operations
on transitions concatenate and move string fragments. DSSTs are implemented in Kleenex.
• Ordinary DFA with ad-hoc user-defined actions and manual conflict resolution via precedence operators, implemented in Ragel. This approach is fast, but it has correctness issues: in some cases it is
impossible to resolve the conflicts between actions by preferring one action over the other; instead, it
is necessary to keep both actions until more input is consumed and non-determinism is resolved. But
this is also impossible, as the actions modify the same shared state (e.g. set the same local variables).
An action may conflict with itself on different transitions due to non-determinism.
2. JIT determinization, C++ (figure 9). These benchmarks compare TDFA(1) and multi-pass TDFA(1)
presented in section 4 in the case of single offsets and offset lists. Both algorithms are implemented in
a C++ library based on RE2C. These benchmarks use POSIX disambiguation.
3. JIT determinization, Java (figures 10, 11). We compare two independent implementations: one in pure
Java (by Borsotti), and one in C++ via JNI, based on RE2C (by Trafimovich), both published as part of
the RE2C repository [5]. These benchmarks compare TDFA(1) and multi-pass TDFA(1) in the case of
single offsets, offset lists and tagged strings, and with different submatch density: sparse tags and
full parsing (where every subexpression in a RE is tagged). They use POSIX disambiguation.
Hardware specifications: Intel Core i7-8750H CPU with 32 KiB L1 data cache, 32 KiB L1 instruction cache,
256 KiB L2 cache, 9216 KiB L3 cache, 32 GiB RAM. Software versions: RE2C 3.0, Ragel 7.0.4, Kleenex built
from Git at commit d474c60, GCC 11.2.0, Clang 13.0.0, OpenJDK 17.0.1.

18

<!-- page 19 -->

gcc (time)
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

gcc (size)

196

clang (time)

82

598

Ü 1 393

299
197

Ü 786
Ü 582

134
134

clang (size)

217

664
381

210

74
Ü 1 700

Ü 518
Ü 814

134
142

http-rfc7230
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

157

22

511
506

227
174

30
22
22

192

70

26

578
525

286
199

86
66

38
42

http-simple
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

169

102

744

228
276
196

Ü 342
Ü 262

82
82

175

106

903

180
235
154

186

78
78

Ü 302

uri-rfc3986
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

239

30
38
30
22
22

505

197
230
194

205

34
34
58
34
38

631

160
186
195

uri-simple
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

88

18
30
18
18
18

467

151
186
187

264
107
103
110

18
26
22
22
22

481

apache-log
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

250

865

221
238
228

153

14
38
14
14
14

Ü 967

151
140
138

14
34
14
14
14

datetime
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

141
108
100
112

425

130

14
26
14
14
14

542

134
139
129

14
26
18
18
18

email
ragel
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

126
141
142
140

337

143

14
22
14
14
14

154
142
154

355

14
22
14
14
14

ipv4

Figure 5: Benchmarks for AOT determinization, real-world REs.
Figure 5 shows benchmark results for AOT determinization in the case of real-world REs that are likely to be
used in practice: HTTP message headers, URI, Apache logs, date, email addresses and IP addresses. REs vary
from very large and complex to small and simple; the number of tags in REs varies accordingly.
The main conclusions are:
• TDFA(1) and ordinary DFA are close in size and speed (the result often depends on GCC/Clang).

• In simple cases staDFA and TDFA(0) are on par with TDFA(1), but in complex cases they are considerably
slower and larger, and staDFA can get extremely large.
• DSSTs are generally slower and larger in most of the cases.
19

<!-- page 20 -->

kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

gcc (time)

gcc (size)

clang (time)

359
146
190
146

18
14
14
14

65
74
66

359
145
185
146

18
14
14
14

76
75
68

354
153
318
154

22
18
18
18

78
77
68

426
142
220
174

22
14
14
14

85
91
89

429
151
334
182

26
18
18
18

clang (size)

374

18
14
14
14

371

18
14
14
14

358

22
14
14
18

alt1-2
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1
alt1-4
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1
alt1-8
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

568

18
14
14
14

504

22
18
18
18

505

30
34
26
26

alt2-2
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

92

276
100

alt2-4
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

30
38
30
30

431
242
560
205

206
101

486

alt2-8
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

30
18
18
18

783

319
235
167

26
18
18
18

754

356
222
115

alt4-2
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

863
187

554

Ü 1 970

34
38
30

786

66
125

525

Ü 1 682

30
30

58
70

alt4-4
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

727
264

1 000

Ü 5 673

50
98
106

729

Ü 550
188

888

Ü 6 107

74

Ü 150
Ü 566
110

alt4-8

Figure 6: Benchmarks for AOT determinization, artificial REs with alternative.
Figure 6 shows benchmark results for AOT determinization in the case of artificial REs with emphasis on
alternative, in series of increasing size, complexity and the number of tags. Ordinary DFA are excluded because
Ragel’s ad-hoc disambiguation operators do not allow to implement all cases correctly. Conclusions:
• TDFA(1) perform better than other algorithms.
• TDFA(0) are generally slower than TDFA(1); the difference grows with RE size.
• StaDFA are close to TDFA(1) on small REs, but they degrade on large REs in both size and speed.
• DSSTs are generally slower and almost always larger than TDFA(1).
20

<!-- page 21 -->

gcc (time)
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

gcc (size)

353
137
164
153

18
14
14
14

467
290
194
184

18
14
14

367
240
128
132

18
18
18

clang (time)

clang (size)
18
14
14
14

364

78
94
90

cat2-0
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

50
96
110

466
338

18
14
14

78

cat2-4
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

90
126
111

359
467

22
18
18

74

cat2-8
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

150
164
161

382

22
18
14
14

142
170
122

381

22
18
18
18

cat4-0
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

194
163

494

764

26
22
22

54

712
684

181
144

30
22
18

106

cat4-2
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

462
333
146
150

Ü 286

38
26
26

137
115

481
551

Ü 290

42
26
22

cat4-4
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

162
215
180

26
18
18
18

465

30
22
22
22

427
148
286
162

cat8-0
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

385
282

Ü 2 794
1 328

66
82
122

154

432
304

Ü 2 158
Ü 2 073

170

98
118

170

cat8-1
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

265
227

1 285
Ü 1 681

170
154
Ü 230
194

311
247

1 155
Ü 2 091

158

Ü 442
Ü 254
Ü 202

cat8-2

Figure 7: Benchmarks for AOT determinization, artificial REs with concatenation.
Figure 7 shows benchmark results for AOT determinization in the case of artificial REs with emphasis on
concatenation, in series of increasing size, complexity and the number of tags. Ordinary DFA are excluded
because Ragel’s ad-hoc disambiguation operators do not allow to implement all cases correctly. Conclusions:
• TDFA(1) perform better than other algorithms.
• TDFA(0) are slower than TDFA(1), but the difference is not radical.
• StaDFA are slower than TDFA(1) on small REs, and the difference gets radical with RE size.
• DSSTs are generally slower and almost always larger than TDFA(1).

21

<!-- page 22 -->

gcc (time)
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

gcc (size)

clang (time)

291
273
273
277

18
14
14
14

284
174
176
178

18
18
18
18

92
94
94

284
157
157
159

18
18
18
18

clang (size)

336
212
209
203

18
14
14
14

282

18
18
18
18

269
168
75
75

18
22
22
22

rep-cat-5-3-2
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1
rep-cat-13-11-7
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1
rep-cat-23-19-17
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

334
463
392
292

14
14
14

30

375
309
364
261

50

14
14
14

rep-alt-5-3-2
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

354
294

661

22
18
22

363
300

550

18
18
18

rep-alt-13-11-7
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

313
268

745

30
30

42

852

308
333

26
22

42

rep-alt-23-19-17
kleenex
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

270
354

615

810

18
18
18

316
342

42

327

54

14
14
14

548

rep-5-rep-3-rep-2
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

416
313

1 171

34
38

58

533
386

1 161

70

26
26

rep-13-rep-11-rep-7
re2c-stadfa
re2c-tdfa0
re2c-tdfa1

388
317

Ü 1 546

78

Ü 286
90

458
378

Ü 1 984

42
42

Ü 334

rep-23-rep-19-rep-17

Figure 8: Benchmarks for AOT determinization, artificial REs with repetition.
Figure 8 shows benchmark results for AOT determinization in the case of artificial REs with emphasis on
repetition, in series of increasing size, complexity and the number of tags. Ordinary DFA are excluded because
Ragel’s ad-hoc disambiguation operators do not allow to implement all cases correctly, and DSSTs are excluded
in cases where they get too large to be compiled. Conclusions:
• TDFA(1) perform better than other algorithms.
• TDFA(0) are slower than TDFA(1), but the difference is not radical.
• StaDFA are slower and larger than TDFA(1) on small REs, and the difference gets radical with RE size.
• DSSTs are generally larger than TDFA(1), and the difference gets extreme with RE size.

22

<!-- page 23 -->

regcomp (time)
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

regexec (time)

1

0.0093

1

1.6

1.1

0.0092

3.8

http-rfc7230
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1

0.077

1
1.8

0.077

1.6
1.4

2.2

http-simple
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1

0.015

1

1.1

0.015

1.3

2.7
2.7

uri-rfc3986
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1

0.15

1

1.7

1.5

0.15

3.4
3.5

uri-simple
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1

0.051

1
0.99

1.3

0.051

2

2.4

ipv6
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

0.6
0.6

1

1

1.1

1.5

1.8

3

ipv4
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1
0.63
1.1
0.63

1

1

1

1.4
1.6

2.7

date
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

0.24

1.5

0.24

1.5

2

2.6

package
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

0.48

1

0.47

0.41

1.7

1
3

0.87

(a)∗ a10
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1
0.71
0.71

0.034

1.8

1
2.3

0.073

(a)∗ a100
last-offset
multipass-last-offset
all-offsets
multipass-all-offsets

1
0.82
1.2
0.82

0.0043
0.0084

1
2.3

(a)∗ a1000

Figure 9: Benchmarks for JIT determinization, C++ (regcomp/regexec time, relative).
23

5.2

<!-- page 24 -->

220 000

last-offset-java
last-offset-multipass-java
last-offset-re2c
last-offset-multipass-re2c
all-offsets-java
all-offsets-multipass-java
all-offsets-re2c
all-offsets-multipass-re2c
tagged-string-multipass-java
tagged-string-multipass-re2c

200 000

parse speed (characters per second)

180 000
160 000
140 000
120 000
100 000
80 000
60 000
40 000
20 000
0

20

30

40

50

60

70

80

regular expression size (characters)

Figure 10: Benchmarks for JIT determinization, Java, sparse tags (regexec speed).

200 000

last-offset-java
last-offset-multipass-java
last-offset-re2c
last-offset-multipass-re2c
all-offsets-java
all-offsets-multipass-java
all-offsets-re2c
all-offsets-multipass-re2c
tagged-string-multipass-java
tagged-string-multipass-re2c

180 000

parse speed (characters per second)

160 000
140 000
120 000
100 000
80 000
60 000
40 000
20 000
0

20

30

40

50

60

70

80

regular expression size (characters)

Figure 11: Benchmarks for JIT determinization, Java, full parsing (regexec speed).

24

<!-- page 25 -->

Figure 9 shows benchmark results for JIT determinization, C++. Time is shown relative to the first row.
k
There are two groups of benchmarks: real-world REs and artificial REs (a)∗ a10 for k ∈ {1, 2, 3}. Conclusions:

• Compilation is predictably slower for TDFA than for multi-pass TDFA for both groups, as multi-pass TDFA
do not need register actions and subsequent register optimizations.
• Execution time differs for the two groups: for real-world benchmarks TDFA are generally faster than
multi-pass TDFA, while for artificial benchmarks TDFA are much slower than multi-pass TDFA, and the
difference grows with the size of RE. In fact artificial REs demonstrate a pathological case for TDFA with
register actions: increasing k results in increased degree of nondeterminism, which requires more registers
and copy operations in order to track all nondeterministic values. High degree of nondeterminism is specific
to some REs with counted repetition, as demonstrated in [3] (page 21).
• The results are similar for single-offset and offset-list cases, although the latter is predictably slower.

Figures 10 and 11 show benchmark results for JIT determinization, Java, in the case of sparse tags and full
parsing respectively. The plots show the dependence of matching speed on RE size. Conclusions:
• Remarkably, the case of tagged strings with multi-pass TDFA is the only one that shows almost no degradation with RE size (the lines are almost horizontal). This holds for both implementations.
• TDFA with register actions (the RE2C implementation) is clearly the fastest algorithm in the case of sparse
tags. However, in the case of full parsing it either degrades faster than multi-pass TDFA (in the last-offset
case), or it is generally slower (in the offset-list case). For pure-Java implementation multi-pass TDFA is
almost always faster than TDFA with register actions.

6

Conclusions
TDFA(1) are generally faster and smaller than other automata capable of submatch extraction.

Optimizations play a very important part in any performance-sensitive TDFA implementation (compare the
unoptimized TDFA on figure 1 with the final optimized TDFA on figure 3).
The overhead on submatch extraction depends on tag density and degree of nondeterminism in a RE. In the
case of sparse tags with low nondeterminism TDFA with register actions are by far the fastest and have negligible
difference compared to ordinary DFA. In the case of high tag density (in the extreme, full parsing) or in the case
of highly nondeterministic REs multi-pass TDFA are more efficient.
The overhead on submatch extraction depends on the representation of submatch results. Tagged string
extraction with multi-pass TDFA is the only algorithm that shows almost no degradation with RE size. Extracting
only the last offset is predictably faster than extracting all offsets (fortunately, the choice is individual for each
tag, so all offsets can be extracted only for a selected subset of tags).
Multi-pass TDFA are better suited to JIT determinization than TDFA with register actions.

7

Future work

One very useful direction of future work is to find deterministic points in a RE. Often shifting a tag by a fixed
number of characters in a concatenation subexpression can reduce its degree of nondeterminism (the maximum
number of registers in a single TDFA state needed to track all parallel versions of the same tag). As a consequence,
this means fewer registers and register operations. For example, tag t1 in a∗ 1ak a∗ has nondeterminism degree
k and requires 2 ∗ k register operations, while tag t2 in a∗ ak 2a∗ has degree is 1 and only 1 operation. But tags
t1 and t2 are within fixed distance of k characters, so t1 can be the computed as t2 − k. In other words, t2 is a
deterministic point for t1 . Identifying such points in a RE would be a useful optimization.

Acknowledgments
I want to thank my parents Vladimir and Elina, my dearest friend and open source programmer Sergei, my
teachers Tatyana Leonidovna and Demian Vladimirovich and the whole open source community. And, of course,
my coauthor Angelo who was the greatest inspiration and help in this work!
Ulya Trafimovich
25

<!-- page 26 -->

References
[1] Ville Laurikari, NFAs with tagged transitions, their conversion to deterministic automata and application
to regular expressions, Proceedings Seventh International Symposium on String Processing and Information
Retrieval. SPIRE 2000, pp. 181-187, http://laurikari.net/ville/spire2000-tnfa.pdf, 2000.
[2] Chris Kuklewicz, Regular expressions/bounded space proposal, http://wiki.haskell.org/index.php?
title=Regular_expressions/Bounded_space_proposal&oldid=11475, 2007.
[3] Ulya Trafimovich, Tagged Deterministic Finite Automata with Lookahead, arXiv:1907.08837 [cs.FL], 2017.
[4] RE2C, a lexer generator for C, C++, Go and Rust. Source code: https://github.com/skvadrik/re2c.
Official website: https://re2c.org.
[5] Experimental Java library for TDFA. Source code: https://github.com/skvadrik/re2c/tree/master/
benchmarks/submatch_java.
[6] Angelo Borsotti, Ulya Trafimovich, Efficient POSIX submatch extraction on nondeterministic finite automata, Software: Practice and Experience 51, no. 2, pp. 159–192, DOI: https://doi.org/10.1002/spe.
2881, preprint: https://www.researchgate.net/publication/344781678_Efficient_POSIX_submatch_
extraction_on_nondeterministic_finite_automata, 2019.
[7] Ulya Trafimovich, RE2C: A lexer generator based on lookahead-TDFA, Software Impacts, 6, 100027, DOI:
https://doi.org/10.1016/j.simpa.2020.100027, 2020.
[8] Niels Bjørn Bugge Grathwohl, Parsing with Regular Expressions & Extensions to Kleene Algebra, DIKU,
University of Copenhagen, 2015.
[9] Stephen Cole Kleene, Representation of events in nerve nets and finite automata, RAND Project US Air
Force, 1951.
[10] Dexter Kozen, A completeness theorem for Kleene algebras and the algebra of regular events, Elsevier,
Information and computation, vol. 110 (2) pp. 366–390, 1994.
[11] Mohammad Imran Chowdhury, staDFA: An Efficient Subexpression Matching Method, Master thesis, Florida
State University, 2018.
[12] Aaron Karper, Efficient regular expressions that produce parse trees (thesis), University of Bern, 2014.
[13] Kleenex language, DIKU, University of Copenhagen. Official website: https://kleenexlang.org. Source
code: https://github.com/diku-kmc/kleenexlang,
[14] Ragel State Machine Compiler. Official website: https://www.colm.net/open-source/ragel.

26
