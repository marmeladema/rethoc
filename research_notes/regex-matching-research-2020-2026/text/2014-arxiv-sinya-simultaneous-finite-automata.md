---
file: ../pdfs/2014-arxiv-sinya-simultaneous-finite-automata.pdf
citation: "Simultaneous Finite Automata: An Efficient Data-Parallel Model for Regular Expression Matching. Sin'ya, Matsuzaki, Sassa. arXiv 1405.0562, 2014"
doi: 
source: "https://arxiv.org/pdf/1405.0562"
topics: [performance_hardware]
pages: 10
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

arXiv:1405.0562v1 [cs.FL] 3 May 2014

Simultaneous Finite Automata:
An Efficient Data-Parallel Model
for Regular Expression Matching
Ryoma Sin’ya

Kiminori Matsuzaki

Masataka Sassa

Department of Mathematical
and Computing Sciences,
Tokyo Institute of Technology
Email: shinya.r.aa@m.titech.ac.jp

School of Information,
Kochi University of Technology
Email: matsuzaki.kiminori@kochi-tech.ac.jp

Department of Mathematical
and Computing Sciences,
Tokyo Institute of Technology
Email: sassa@is.titech.ac.jp

Abstract—Automata play important roles in wide area of
computing and the growth of multicores calls for their efficient
parallel implementation. Though it is known in theory that we
can perform the computation of a finite automaton in parallel by
simulating transitions, its implementation has a large overhead
due to the simulation. In this paper we propose a new automaton
called simultaneous finite automaton (SFA) for efficient parallel
computation of an automaton. The key idea is to extend an
automaton so that it involves the simulation of transitions. Since
an SFA itself has a good property of parallelism, we can develop
easily a parallel implementation without overheads. We have
implemented a regular expression matcher based on SFA, and
it has achieved over 10-times speedups on an environment with
dual hexa-core CPUs in a typical case.
This paper has been accepted at the following conference:
2013 International Conference on Parallel Processing (ICPP2013), October 1-4, 2013 Ecole Normale Suprieure de Lyon, Lyon,
France.

I.

I NTRODUCTION

Automata play important roles in theory and practice in
a wide area of computing. For example the use of nondeterministic or deterministic automata is crucial in regular expression matching. Under the growth of multicores, parallelism
becomes more and more important. In previous studies [1],
[2], computations of automata are naively executed in parallel
when both/either of queries and/or data are multiple, while a
single computation of an automaton is executed in sequential.
To extract more parallelism, parallelizing an automaton itself
would be important. It has been known for a long time
in theory that we can perform the computation of a finitestate automaton in parallel [3], [4]. The basic idea of the
parallelization is to simulate all the transitions from all the
possible states speculatively. However, as reported in previous
studies [5], [6], [7], [8], such a parallel implementation has a
large overhead due to the speculative simulation.
In this paper, we propose a novel approach for parallelizing
the computation of automata. The key idea is to extend
automata so that they involve the speculative simulation from
all the states. We develop new automata named simultaneous
finite automata (SFA in short) as extensions of finite-state
automata where the states in SFA are given as mappings from
states to states of the original automata. The key property of

the SFA is that they essentially involve parallelism and thus
we can straightforwardly implement the computation of SFA
in parallel. Though such an extension may increase the size
of automata, we can remove the runtime overhead. It is worth
noting that usually automata are considerably smaller than data
and the runtime speedup outstrips the enlargement of automata.
We can systematically construct an SFA from either an
NFA or a DFA by a technique similar to the so-called subset
construction technique. In general, such a construction may
increase the number of states exponentially. However, for
widely-used regular expressions, the number of states in SFA
is no more than the square of that in the original automata. We
show the effectiveness of SFA with the experiment results of
the SFA-based parallel regular expression matching. Our SFAbased implementation has almost no overhead and achieved
over 10-times speedups on an environment with dual hexa-core
CPUs with respect to the DFA-based sequential implementation in a typical case.
The contributions of this paper are summarized as follows.
•

We proposed a new automaton, simultaneous finite
automaton, for parallel regular expression matching
(Sect. IV). By using SFA, we can compute regular
expression matching simply in parallel without overheads (Algorithm 5). This SFA-based parallel regularexpression matcher is available online [9].

•

We developed an algorithm for constructing SFA from
NFA or DFA (Algorithm 4). Since the algorithm is a
natural extension of the subset construction algorithm,
we can apply known implementation techniques for it.

•

The only concern of SFA is the size explosion with
respect to DFA or NFA. We show that almost all the
SFA are small enough for practical regular expressions
in the SNORT rulesets. We also discuss the cases that
SFA have as many states as the upper bound.

The rest of the paper is organized as follows. We introduce
the basic idea of automata in Sect. II, and we review the
parallelization method based on the speculative simulation
in Sect. III. In Sect. IV, we define the simultaneous finite
automata and discuss their properties. In Sect. V, we develop
the implementation of SFA: a construction method and an

<!-- page 2 -->

application to parallel regular expression matching. In Sect.
VI, we show the experimental results on SFA’s size, scalability, and overheads. In Sect. VII, we discuss the algebraic
characterization of SFA for the theoretical upper bound of the
number of states. Finally, we conclude the paper in Sect. VIII.
Remarks on automata theory. Automata theory has been
deeply studied for a long time and there exist many extended
models of automata in terms of parallelism. Some examples are
parallel finite automata [10], concurrent finite automata [11],
and alternating finite automata [12]. These models are extension for dealing with parallel/concurrent events, and they are
not for implementing parallel matching of an automaton. The
SFA in this paper is a new automata for discussing data-parallel
regular expression matching.
II.
A. Notation
In this paper, we describe definitions and algorithms with
symbols in the basic set theory. Some things to note: |A|
denotes the size of set A (number of their elements). P(A)
is the power set of A and |P(A)| = 2|A| holds. F(A, B)
denotes all the mappings from A to B (f : A → B)
and |F(A, B)| = |B||A| holds. In particular, F(A, A) is
called a transformation of A, and F(A, P(A)) is called a
correspondence of A. We define function composition ◦ on
transformations and correspondences as follows:
(f ◦ g)(a)

f, g ∈ F(A, P(A)), ∀a ∈ A

(f ◦ g)(a)

:= f (g(a)),
[
f (b).
:=
b∈g(a)

We also define reverse composition • as f • g := g ◦ f . Here,
note that function composition and reverse composition are
always associative.
B. Finite Automata
We briefly introduce some basics of automata theory according to [13]. First we give the definition of nondeterministic
and deterministic finite automata.
Definition 1 A nondeterministic finite automaton (NFA) N is
a quintuple N = (Q, Σ, δ, I, F ), where Q is a finite set of
states, Σ is a set of input symbols, δ is a transition function
of type Q × Σ → P(Q), I ⊆ Q is a set of initial states, and
F ⊆ Q is a set of final states.
⋄
Definition 2 A deterministic finite automaton (DFA) D is a
special case of NFA, where I and every image of δ are
singletons:
|I| = 1

∧

∀q ∈ Q, ∀σ ∈ Σ [|δ(q, σ)| = 1] .

⋄

We may say the number of states in an automaton as the
size of the automaton, and we denote the size of automaton
A as |A|. We introduce δb for an extended transition function
over input texts:
[
b ′ , w),
b σw) :=
δ(q
δ(q,
q′ ∈δ(q,σ)

b ǫ) := {q} .
δ(q,

δ σ (q)
δbw (q)

:=
:=

δ(q, σ),
b w) .
δ(q,

b w) is a transition of automaton A, w is said
If p ∈ δ(q,
w
to be the label of the transition and we will write q −→ p (or
A

w

simply q −
→ p if it is unambiguous).
Definition 3 A computation c in A is a sequence of transitions, which can be written as follows:
σ

σ

σ

σ

n
3
2
1
qn .
· · · −−→
q2 −→
q1 −→
c := q0 −→

P RELIMINARIES

f, g ∈ F(A, A), ∀a ∈ A

The symbol ǫ denotes empty word and transition over ǫ does
nothing. We also introduce bound transition function δ σ , δbw :
Q → P(Q) defined by follows:

A word in Σ∗ is accepted by A if it is the label of a
computation that begins at an initial state and ends at a final
state in A.
⋄
Definition 4 L(A) denotes the set of all the words accepted
by A:
n
h
io
w
L(A) = w | ∃q ∈ I, ∃p ∈ F q −→ p .
⋄
A

We say two automata A and A′ are equivalent if L(A) =
L(A′ ) holds. The following theorem shows that there exists
an equivalent DFA to every automaton.
Theorem 1 (Rabin and Scott[14]) Every automaton A is
equivalent to a DFA D. If A is finite with n states, D can
be constructed with at most 2n states.
Proof: Let A = (Q, Σ, δ, I, F ) be an automaton. We
consider an automaton D = (Qd , Σ, δd , Id , Fd ): Qd is P(Q);
δd is the additive extension of δ
[
S ∈ P(Q), σ ∈ Σ
δd (S, σ) :=
δ(q, σ) ;
q∈S

Id is a singleton of set {I}; final states are given by Fd =
{S ∈ P(Q)|S ∩ F 6= ∅}. The automaton D is deterministic.
Furthermore, it is equivalent to A since we have the following
series of equivalences:
h
i
b w) ∩ F 6= ∅
w ∈ L(A) ⇔ ∃q ∈ I δ(q,
⇔ δbd ({I}, w) ∩ F 6= ∅
⇔ δbd (Id , w) ∈ Fd ⇔ w ∈ L(D).

C. Subset Construction and Sequential Computation in DFA
It is often faster to perform the computation with DFA than
to do with NFA. Given an NFA, we can determinize it by the
subset construction technique shown in Algorithm 1. Starting
from the set of initial states, we compute the accessible subset
of DFA step by step considering only those states obtained by
applying the transition function to the states already calculated.
Sequential implementation of the computation in DFA is
straightforward. Algorithm 2 shows the sequential program for
the computation in DFA, in which we use a table δd [q, σ] for

<!-- page 3 -->

Algorithm 1 Subset construction
Require: Automata A = (Q, Σ, δ, I, F )
Ensure: DFA D = (Qd , Σ, δd , Id , Fd ) is equivalent to A
1: Qd ← ∅, Qtmp ← {I}
2: while Qtmp 6= ∅ do
3:
choose and remove a set S from Qtmp
4:
Qd ← Qd ∪ {S}
5:
for all σ ∈ S
Σ do
6:
Snext ← q∈S δ(q, σ)
7:
δd [S, σ] ← Snext
8:
if Snext ∈
/ Qd then Qtmp ← Qtmp ∪ {Snext }
9:
end for
10: end while
11: Id ← {I}
12: Fd ← {S ∈ Qd |S ∩ F 6= ∅}
Algorithm 2 Sequential computation of DFA
Require: DFA D = (Qd , Σ, δd , {q0 }, Fd ), and
word w = σ1 σ2 · · · σn
w
Ensure: qf inal is the destination such that q0 −→ qf inal
D
1: q ← q0
2: for i = 1 → n do
3:
q ← δd [q, σi ]
4: qf inal ← q

the transition function. Note that we store only a single state
and reuse it during the computation.
Let D be the DFA, Σ be the set of input symbols, and n
be the size of input word. Then, the sequential computation
in DFA takes O(n) time, and the number of elements in the
table of the transition function is O(|D||Σ|).
III.

P RIOR W ORKS : PARALLEL C OMPUTATION IN DFA
WITH S PECULATIVE S IMULATION

It has been known for a long time that the computation
in DFA can be performed in parallel on parallel random
access machines (PRAMs) [3], [4]. The fundamental idea is the
speculative simulation of transitions in which we consider all
the states as initial states. Such simulation of transitions forms
a finite-sized mapping (between sets of states) and composition
of finite-sized mappings is associative. This associativity in
the composition of mappings enables us to perform parallel
reduction for the computation of DFA.
Algorithm 3 shows a parallel implementation of the computation of DFA based on speculative simulation [5], [6], [7], [8].
The following two points are important in this algorithm. First,
the mappings Ti [ ] are computed on subwords independently
in parallel and they contain transitions from all the states.
Secondly, we can reduce the subresults either in parallel with
associative binary operator • or in sequential.
Let D be the DFA, n be the size of input word, p be the
number of processors. The time complexities of Algorithm 3
are O(|D|n/p + |D| log p) when parallel reduction is used
or O(|D|n/p + p) when sequential reduction is used [5].
The coefficient |D| comes from the speculative simulation of
transitions, and it means that the parallel implementation no

Algorithm 3 Parallel computation of DFA
Require: DFA D = (Qd , Σ, δd , {q0 }, Fd ), number of threads
p,
word w = σ11 · · · σ1m1 σ21 · · · σ2m2 · · · σp1 · · · σpmp
w
Ensure: qf inal is destination such that q0 −→ qf inal
D
1: for all i ∈ [1, 2, . . . , p] parallel do
2:
for all q ∈ Qd do
3:
Ti [q] ← q
4:
for j = 1 → mi do
5:
for all q ∈ Qd do
6:
Ti [q] ← δ(Ti [q], σij )
7: end for
8: // parallel reduction
// sequential reduction
9: T ← T1 • T2 • . . . • Tp
qf inal ← q0
10: qf inal ← T [q0 ]
for i = 1 → p do
11:
qf inal ← Ti [qf inal ]

longer runs faster than the sequential implementation when the
size of the DFA is large.
IV.

S IMULTANEOUS F INITE AUTOMATA

The simulation-based parallel computation of DFA has a
large overhead linear to the size of DFA. In this section, we
propose a new model of automata that involve the simulation
of transitions in the definition. The key idea is that we can
evaluate the simulation in advance in the same way as we
evaluate the set of transitions during the construction of DFA
from NFA. The proposed model have a good property for data
parallel computation.
A. Formal Definition
We call the automaton simultaneous finite automaton (SFA,
in short). A state in SFA corresponds to a mapping from states
to sets of states in the normal finite automata.
Definition 5 Let A = (Q, Σ, δ, I, F ) be an automaton. A
simultaneous finite automaton (SFA) constructed from A is
a quintuple (Qs , Σ, δs , Is , Fs ):
•

Qs ⊆ F(Q, P(Q)) is a set of mappings;

•

Σ is the same set of symbols as A;

•

δs is the additive extension of δ in A that is defined
as f ∈ Qs , σ ∈ Σ, δs (f, σ) := {f • δ σ };

•

Is ⊆ Qs is a singleton of identity mapping {fI } that
satisfies fI (q) = {q} for any q ∈ Q;

•

Fs ⊆ Qs is defined as Fs = {f ∈ Qs | ∃q ∈ I
[f (q) ∩ F 6= ∅]}.
⋄

By definition, SFA are entirely deterministic. As described
later, SFA can be regarded as DFA with simultaneity.
Theorem 2 Every automaton A is equivalent to an SFA S. If
2
A is finite with n states, S can be constructed with at most 2n
states. In particular, if A is deterministic, S can be constructed
with at most nn states.

<!-- page 4 -->

a

b

state 0 is an accepted state in D1 , f4 is also an accepted state
in S1 .
⋄

2
b

a

a

0

1

C. Data-Parallel Property of SFA

b
Fig. 1.

We finally show an important property of SFA: the dataparallel nature in SFA. For any input text, we can divide it at
any points and apply the computation of SFA in parallel.

D1 : L(D1 ) = L((ab)∗)

b

f1
a

a

f0

b

w w

a

b

f2

Lemma 1 Let S be an SFA, f be a state in S, fw1 and fw2
w2
w1
fIw2 . Then the
fw1 and fI −−→
be the states satisfying f −−→
S
S
following equation holds:

b

f3

a
b

f4

a

a

2
fw1 w2 ⇔ fw1 • fIw2 = fw1 w2 .
f −−1−→

S

f5

b
Fig. 2.

Proof: By definition, we have
w w

2
fw1 w2 ⇔ δbs (fw1 , w2 ) = {fw1 w2 }
f −−1−→

S1 : L(S1 ) = L(D1 ) = L((ab)∗)

S

TABLE I.

T HE STATE MAPPINGS OF F IG .2

(1)

where δbs (f, w1 ) = {fw1 } .

f0
f1
f2
f3
f4
f5
0 7→ {0} 0 7→ {1} 0 7→ {2} 0 7→ {2} 0 7→ {0} 0 7→ {2}
1 7→ {1} 1 7→ {2} 1 7→ {0} 1 7→ {2} 1 7→ {2} 1 7→ {1}
2 7→ {2} 2 7→ {2} 2 7→ {2} 2 7→ {2} 2 7→ {2} 2 7→ {2}

We can transform the left-hand side as follows by applying
the definition of SFA.

Proof: Let the original automaton be A = (Q, Σ, δ, I, F ),
and the SFA constructed from A be S = (Qs , Σ, δs , {fI }, Fs ).

We used the fact that fI is an identity function and the equation
δs (fI , w2 ) = {fI • δbsw2 } = {fIw2 }. The lemma follows from
Equations (1) and (2).

In addition to the fact that S is deterministic, S is equivalent to A since we have the following series of equivalences:
h
i
b w) ∩ F 6= ∅
w ∈ L(A) ⇔ ∃q ∈ I δ(q,
h
i
⇔ ∃q ∈ I δbs (fI , w)(q) ∩ F 6= ∅
⇔ δbs (fI , w) ∈ Fs ⇔ w ∈ L(S).

{fw1 • δbsw2 } = {fw1 • (fI • δbsw2 )}
(2)
{fw1 • fIw2 }.

δbs (fw1 , w2 ) =
=

This lemma enables us to introduce the following important
theorem about the data-parallelism of the SFA.
w

→ f can be derived
Theorem 3 The computation in SFA fI −
S
by any division of label w = w1 w2 . . . wn .
w=w w ···w

The size of the set of mappings is bounded as |Qs | ≤
2
|F(Q, P(Q))| = 2|Q| . If A is deterministic, transition function
is one-to-one correspondence and |Qs | ≤ |F(Q, Q)| = |Q||Q| .

1 2
n
−−−−→
f can be decomProof: Computation fI −−−−−
S
posed into the following equation by Lemma 1:

f = fw1 •fw2 •· · ·•fwn

where

w

i
fwi
fI −→

S

(i = 1, . . . , n) .

w

B. Example
Here we give an example of an SFA, which corresponds to
a DFA. Notice that, though the states in SFA have meanings
of mappings from states to sets of states in corresponding
automaton, we need not to mind it when we compute the
transitions in SFA. In other words, we can compute all the
transitions in a finite automaton simultaneously by simply
computing the transitions in SFA.

i
fwi has no dependency on the other
Each computation fI −→
S
computations and these composition is associative. Hence,
computation in SFA can be performed in a data-parallel
manner. We call this method parallel computation in SFA.

In the following of the paper, we may classify the SFA in
terms of the original automaton. We call the SFA constructed
from NFA as N-SFA, and that from DFA as D-SFA.
V.

Example 1 Figure 1 shows DFA D1 that accepts L((ab)*).
Figure 2 shows SFA S1 equivalent to D1 where the states in S1
imply the mappings listed in Table I. Final states are denoted
with doubled circles in these figures.
Consider the computation of S1 over abab. By following
a
b
a
the states in Fig. 2, we have transitions f0 −→ f1 −→ f4 −→
b

S1
abab

S1

D1

S1

S1

f1 −→ f4 . Here, f4 (0) = {0} implies 0 −−−→ 0. Since the

I MPLEMENTING SFA

A. Construction of SFA from Finite Automaton
Algorithm 4 shows how we can construct an SFA from a
finite automaton. We name the algorithm correspondence construction after the subset construction algorithm (Algorithm 1)
that constructs a DFA from an NFA. The correspondence construction algorithm is very similar to the subset construction
algorithm, and the main difference in line 6 of Algorithm 4: we
compute a mapping fnext (q) for all the states in the original

<!-- page 5 -->

Algorithm 4 Correspondence construction
Require: Automaton A = (Q, Σ, δ, I, F )
Ensure: SFA S = (Qs , Σ, δs , Is , Fs ) is equivalent to an
automaton A
1: Qs ← ∅, Qtmp ← {fI }
2: while Qtmp 6= ∅ do
3:
choose and remove a mapping f from Qtmp
4:
Qs ← Qs ∪ {f }
5:
for all σ ∈ Σ do
S
6:
q ∈ Q fnext (q) := q′ ∈f (q) δ(q ′ , σ)
7:
δs [f, σ] ← fnext
8:
if fnext ∈
/ Qs then Qtmp ← Qtmp ∪ {fnext }
9:
end for
10: end while
11: Is ← {fI }
12: Fs ← {f ∈ Qs |∃q ∈ I|f (q) ∩ F 6= ∅}

Algorithm 5 Parallel computation of SFA
Require: SFA S = (Qs , Σ, δs , {fI }, Fs ) which is constructed
from automaton A = (Q, Σ, δ, I, F ), number of threads p,
word w = σ11 · · · σ1m1 σ21 · · · σ2m2 · · · σp1 · · · σpmp
Ensure: Sf in ish a set i of destinations such that ∀p ∈
w
Sf in , ∃q ∈ I q −→ p

automaton. If the original automaton is deterministic, then the
image of the transition function is a singleton and we can
simplify the line 6 as follows.

4, and the input word w be ababababababab that is split as
w = w1 w2 w3 w4 such that w1 = aba, w2 = baba, w3 = bab,
and w4 = abab. In the following, step 1 corresponds to lines
1–5 in Algorithm 5 and step 2 corresponds to lines 6–9.

q∈Q

fnext (q) := δ(q ′ , σ) where {q ′ } = f (q).

A

1: for all i ∈ [1, 2, . . . , p] parallel do
2:
fi ← fI
3:
for j = 1 → mi do
4:
fi ← δ[fi , σij ]
5: end for
6: // parallel reduction
// sequential reduction
7: ff in ← fS
Sf in ← I
1 • . . . • fp
for i = 1 → p S
do
8: Sf in ← q∈I ff in (q)
9:
Sf in ← p∈S fi (p)
f in

step 1
As is the case of the subset construction, the number of
the states in the constructed SFA may increase exponentially
compared with that in the original automaton. As we have
stated in Theorem 2, in the worst case, from an NFA with
n
2
states the number of the states in an N-SFA becomes 2n , and
from a DFA with n states the number of the states in a DSFA becomes nn . You might consider that these numbers of
states dismiss the practical use, but it is not true. From DFA
that correspond to typical regular expressions, fortunately, the
number of states in the constructed D-SFA is no more than the
square of that in DFA (we will show this fact in Sect. VI-A).
The on-the-fly construction is a well known technique [15]
in the implementation of an advanced DFA-based matcher.
The idea of the on-the-fly construction is to construct DFA
during the matching only for the required states, instead of
constructing full DFA before the matching. Since on-the-fly
construction generates states one by one after reading symbols,
it generates at most n states for input text of length n even
if the number of states in DFA explodes. We can easily
apply on-the-fly construction to an SFA-based matcher because
the correspondence construction is a natural extension of the
subset construction.
B. Parallel Computation in SFA
As we can see from Definition 5, SFA is deterministic
in the sense that the image of the transition function is a
singleton. Therefore, we can simply and efficiently implement
the computation of SFA by the table-look-up technique. In
addition, from Lemma 1, we can split the input word at any
point and perform the computation of SFA independently in
parallel. After local computation over subtexts, we reduce the
results either in parallel with associative binary operator • or in
sequential. Algorithm 5 shows the pseudo code of the parallel
computation of SFA.
Example 2 We show how Algorithm 5 runs using the SFA
S1 given in Example 1. Let the number of processors p be

step 2

For each subword wi , we compute transitions by S1
independently in parallel. For example, on the first
a
b
a
→ f1 . In the same
→ f4 −
→ f1 −
processor, we get f0 −
w2 =baba
w =bab
manner, we get f0 −−−−−→ f5 , f0 −−3−−−→ f2 , and
w4 =abab
f0 −−−−−→ f4 .
We calculate the reduction in parallel on the results
of step 1, that is, we calculate (f1 • f5 ) • (f2 • f4 ).
Here, we can compute the function composition with
the mappings in Table I. For example, we get (f1 •
f5 )(0) = (f5 ◦ f1 )(0) = f5 (1) = {1}, and similarly,
(f1 • f5 )(1) = {2} and (f1 • f5 )(2) = {2}; as a
consequence we get f1 • f5 = f1 from these results.
Evaluating the other • operators, we get (f1 • f5 ) •
(f2 • f4 ) = f1 • f2 = f4 as desired.
⋄

It is worth remarking that in Algorithm 5 each thread
only deals with a single state in SFA and just looks up the
transition table once for each character. In Algorithm 5, we
have therefore no overhead linear to the number of states
in DFA, which is the defect of Algorithm 3. The possible
overhead is unfortunate cache misses due to the enlargement of
the transition table, but the overhead is quite small for practical
regular expressions is discussed later.
We can also compute the reduction sequentially: starting
from the initial state in the original automaton, we simply
compute the states by picking up the states from the mappings
obtained in step 1. In the case of Example 2, we have (f4 ◦f2 ◦
f5 ◦ f1 )(0) = (f4 ◦ f2 ◦ f5 )(1) = · · · = {0}. We can compute
this sequential reduction in O(p) time, which is independent
from the number of states in SFA.
Table II lists the maximum number of states and the
execution time. The last four lines in the table differ in terms
of the cost of the reduction. In parallel reduction for N-SFA,
the computation of • operator corresponds to the logical matrix
multiplication (O(|N |3 )). In sequential reduction for N-SFA,
we evaluate the function one by one, which corresponds to
sequential computation of NFA (O(|N |)). In parallel reduction

<!-- page 6 -->

TABLE II.

Model
NFA N
DFA D

C OMPARISON OF COMPLEXITY

State complexity
|N | = O(m)
|D| = O(2|N | )

Computation time complexity
O(|N |n)
([16] p.165)
O(n)
(Algorithm 2)
O(|D|n/p + |D| log p)
(Algorithm 3)
O(|D|n/p + p)
(sequential reduction)
2
N-SFA Sn |Sn | = O(2|N | ) O(n/p + |N |3 log p)
O(n/p + |N |p)
(sequential reduction)
D-SFA Sd |Sd | = O(|D||D| ) O(n/p + |D| log p)
O(n/p + p)
(sequential reduction)
m is length of regular expression, n is length of input word, p is number of threads

for D-SFA, we need to simulate the transitions for all the
states in DFA, and it means we need O(|D|) time for each
computation of •. The sequential reduction for D-SFA is the
same as the transition of DFA (O(1)).
E XPERIMENTAL R ESULTS

We have implemented an SFA-based parallel regular expression matcher [9]. It runs in the following four steps: first
it converts a regular expression into an NFA by McNaughton
and Yamada’s algorithm [17]; secondly into a DFA by the
subset construction (Algorithm 1); thirdly into a SFA by the
correspondence construction (Algorithm 4); finally it executes
Algorithm 5 (with the sequential reduction) specialized to the
constructed SFA.
In the following, we show experiment results conducted
to confirm the good scalability and small overhead of parallel
computation of SFA. The experiment environment is a PC with
two Intel Xeon E5645 CPUs (2.40 GHz, 6 physical cores,
SpeedStep/TurboBoost off) and 12 GB DDR3-SDRAM (1333
MHz). We used CentOS release 5.5 for OS and pthread for
the thread library. In the following results, the throughput and
the execution time are of computation of DFA or SFA, and
exclude construction of automata.

1e+06
D-SFA’s size

VI.

1e+07

100000

x^1
x^2
x^3
x^4

10000
1000
100
10
10

100
DFA’s size

1000

Fig. 3. The distribution of the size of the minimal DFA and D-SFA on
SNORT rulesets.

A. The size of SFA

We would like to discuss the number of states in D-SFA
from two viewpoints: absolute size of D-SFA and relative
size of D-SFA compared with DFA. Firstly, only 102 (0.5%)
regular expressions lead to D-SFA that have more than 10000
states. As we discuss later, current CPUs efficiently compute
automata with 10000 states. Therefore, for almost all the
practical regular expressions, we can use D-SFA for efficient
parallel matching.

The first question that may concern the reader the most
would be “How large SFA are compared with original DFA
for practical regular expressions?” To answer this question,
we have constructed SFA and DFA for over 20000 regular expressions included in the rulesets of SNORT network intrusion
prevention and detection system1 [18], and compared the sizes
of automata.

Secondly, for almost all the regular expressions, the number
of states in the D-SFA is not more than the square of the
number of states in the minimal DFA. Only 279 (1.4%) regular
expressions lead to a D-SFA of over-square size (|Sd | > |D|2 ),
and just 6 regular expressions lead to a D-SFA of over-cubed
size (|Sd | > |D|3 ). These 6 regular expressions have a pattern
similar to:

The details of the experiments are as follows. The version
of the rulesets we used was “snortrules-snapshot-2940 (03
Feb, 2013)”. We extracted about 24000 regular expressions
from the rulesets, and used 20312 regular expressions for the
experiments. (We did not used too large expressions for which
DFA has more than 1000 states, nor extended expressions
that include back references etc.) For each regular expression,
we constructed a minimized DFA and then a D-SFA by
Algorithm 4. Figure 3 plots the sizes of D-SFA to the sizes of
minimized DFA.
1 http://snort.org/

. ∗ (T. ∗ T. ∗ Y. ∗ P. ∗ P. ∗ R. ∗ O. ∗ M. ∗ P. ∗ T.∗)
in which several .* appear in sequence. For the above regular
expression, the size of the minimal DFA is 10 but the size of
D-SFA is 3739. It is worth noting that no regular expressions in
the rulesets lead to a D-SFA of over-quadruplicate size (|Sd | >
|D|4 ).
In theory, the size of a D-SFA |Sd | is bounded by |D||D|
where |D| is the size of the DFA from which the D-SFA is constructed (Table II). From the experiment results, however, we
conclude that the size of D-SFA never grows up exponentially
for practical regular expressions. Of course, in a theoretical

<!-- page 7 -->

14
Throughput [GB/sec]

12

Fig. 4. The DFA of the regular expression r2 = ([0-4]{2}[5-9]{2})*

10
8
6
4
2
0
2

Fig. 6.

Throughput [GB/sec]

B. Scalability
Second question is “Does the SFA-based parallel matching
scale?” We confirmed the scalability of the parallel computation of SFA with regular expressions in the following form:

Figures 6 to 9 show the throughput of the DFA or D-SFA.
Note that the results with one thread were of DFA (and not
D-SFA). The input texts were 1GB string accepted by those
automata, and every character was read exactly once. The input
texts were stored on the memory before the execution.
As seen in Figures 6 and 7, the SFA-based parallel
matching scales well up to 12 threads (with respect to the
sequential DFA-base matching). However, in Fig. 8, the SFAbased parallel matching ran slower (even with 12 threads) than
sequential DFA-based matching. The difference between them
was the size of SFA (and DFA). For r = 50, the number of
states in SFA was 10099 and parallel matching performed well
for this size. For r = 500, the number of states in SFA was
1000999 while the number of states in DFA was 1000. In our
implementation, the transition table occupied 1KB for each
state (256 symbols times 4 bytes). For r = 500 the transition
table for SFA was 1GB and thus it overflowed the CPU cache
(The L3 cache of the CPU was 12MB).
It is worth noting that the large size of SFA does not
always mean the poor performance. It is often the case
that transitions are done among small number of states,
and then we can avoid cache misses fortunately. Figure 9
shows the experiment results for the regular expression

12

5
4.5
4
3.5
3
2.5
2
1.5
1
0.5
2

Fig. 7.

4

6
8
Number of threads

10

12

r50 =([0-4]{50}[5-9]{50})*, |D| = 100, |Sd | = 10099
0.35
0.3
Throughput [GB/sec]

for n = 5, 50, and 500. It is worth noting that the sizes of
D-SFA for these expressions are almost the square of those of
DFA. For better understanding, we illustrate the minimal DFA
in Fig. 4 and the corresponding D-SFA in Fig. 5 for the case
n = 2. The DFA has 2n states in a single loop, but the D-SFA
has 2n loops to distinguish from which state (in DFA) we start.
This is a typical case when we have square-sized D-SFA.

10

0

0.25
0.2
0.15
0.1
0.05
0
2

4

6
8
Number of threads

10

12

Fig. 8.
r500 =([0-4]{500}[5-9]{500})*, |D| = 1000, |Sd | =
1000999
14
12
Throughput [GB/sec]

rn = ([0 − 4]{n}[5 − 9]{n})∗

6
8
Number of threads

r5 =([0-4]{5}[5-9]{5})*, |D| = 10, |Sd | = 109

Fig. 5. The D-SFA of the regular expression r2 = ([0-4]{2}[5-9]{2})*

perspective, there exist regular expressions that lead to N-SFA
or D-SFA of near upper-bound sizes. We will discuss them in
Sect. VII-B.

4

10
8
6
4
2
0
2

4

6
8
Number of threads

10

12

Fig. 9. ra =([0-4]{500}[5-9]{500})*|a*, |D| = 1002, |Sd | =
1001000, input text is the repetition of “a” (1GB)

<!-- page 8 -->

Clock cycles (M cycles)

2.5

DFA
SFA (2 threads)

2
1.5

Fig. 11. The NFA Nex3 of the regular expression e = [ap] ∗ [al][alp]{n −
2}

1
0.5
0
200

400

600

800

1000

Input size (KB)

Fig. 10.

Execution times on small inputs

TABLE III.

T IMES ( IN SEC ) FOR CONSTRUCTING DFA AND D-SFA
FOR rn = ([0 − 4]{n}[5 − 9]{n})∗

DFA D
|D|
D-SFA Sd
|Sd |

r5
0.0003
10
0.0020
109

r50
0.0019
100
0.2020
10099

r500
0.0187
1000
23.937
1000999

([0-4]{500}[5-9]{500})*|a* and input text being a
repetition of “a”. Although the number of states in SFA was
the biggest (1001000), it achieved the best throughput. In this
case, the transitions were done in a single state and cache
misses were avoided.
C. Overheads
We conducted another set of experiments using a smaller
input to evaluate the overhead. Figure 10 shows the execution times of the sequential computation of DFA and the
parallel computation of SFA with two threads. The execution
times of the parallel computation includes the creation of
threads and the reduction. Here we used regular expression
(([02468][13579]){5})* (the size of DFA is 10, and
the size of SFA is 21). Though the execution time of the parallel computation swings caused by interfere between threads,
but the parallel computation runs faster in average over 600KB,
and completely over 800KB.
Finally we briefly remark on the cost of constructing SFA.
Table III shows the time required to constructing DFA and SFA
for the regular expressions rn = ([0 − 4]{n}[5 − 9]{n})∗.
Though the correspondence construction of D-SFA from DFA
is slower than construction of DFA because we need to
calculate the mapping between states, it is fast enough to
generate about 50000 states per second. As we have seen in
Fig. 3, D-SFA for almost all the practical regular expressions
are smaller than 10000 states, and thus we can construct them
in less than 0.2 seconds.
VII.

D ISCUSSION

A. Syntactic monoid
In this paper, we proposed simultaneous finite automaton
(SFA) as a data-parallel model of regular expression matching.

Fig. 12.
The minimal DFA Dex4 of the regular expression e =
(m|(t|c([mt] ∗ c){n − 2})[cmt])∗

SFA are natural extensions of finite automata on the automata
theory. In addition, SFA can be regarded as special cases of
DFA that include the structure of a syntactic monoid [19],
[13], which is an algebraic characterization of the regular
language. We would like to emphasize that SFA will bridge
the gap between the practice of automata and abstract theory
of syntactic monoid.
The size of a syntactic monoid for a regular language is
called syntactic complexity. Indeed, syntactic complexity of a
regular language is also the size of a minimal SFA of the
identical language. So far, syntactic complexity has received
less attention than state complexity that is the size of a minimal
DFA [20].
As we have shown in this paper, SFA provide a dataparallel model of regular expression matching, and thus we
can say that syntactic complexity is also parallel complexity
of regular expressions. We expect that syntactic complexity
gets more attentions for establishing the theory over automata
and their parallelization.
B. The state explosion problem: an algebraic approach
Here we discuss the theoretical upper bound of the number
of states in SFA. First, we see an example in which we
construct a DFA from an NFA followed by a D-SFA from
the DFA.
Example 3 Consider Σ = {a, l, p} and the regular expression
e = [ap] ∗ [al][alp]{n − 2}. Figure 11 shows the NFA Nex3
of the regular expression e.
Let us represent the set of states in NFA by a bit-sequence
of length n. Then, the initial set of states in Fig. 11 is 1 00
· · · 0}.
| {z
n−1

The symbols a and l make the following transitions from the
initial set of states:
a

→ 11 |00 {z
. . . 0} , and
1 00
. . . 0} −
| {z
n−1

n−2

l

→ 01 |00 {z
. . . 0} ,
1 00
. . . 0} −
| {z
n−1

n−2

<!-- page 9 -->

The following fact follows from Devadze’s theory.

and the symbol p makes the following transitions:
p

→ 101 00
. . . 0} , and
11 00
. . . 0} −
| {z
| {z
n−2

n−3

p

→ 001 00
. . . 0} .
01 00
. . . 0} −
| {z
| {z
n−2

n−3

Notice that the symbols a and l correspond to arithmetic shift
and logical shift and the symbol p corresponds to partial shift
applied to bit-sequences from second bit. With these three shift
operations, we can generate all the bit-sequences of length n
from the initial sequence. Hence the minimal DFA Dex3 of
|N
|
Nex3 satisfies |Dex3 | = 2 ex3 .
⋄
By Example 3, we obtain the following fact.
Fact 1 If |Σ| ≥ 3, then there exists a regular expression e over
Σ whose NFA N and minimal DFA N satisfies |D| = 2|N | .⋄
Based on a similar idea, we can find a regular expression
for which a D-SFA has as many states as the theoretical upper
bound from the size of DFA.
Example 4 Consider Σ = {c, m, t} and the regular expression
e = (m|(t|c([mt] ∗ c){n − 2})[cmt])∗. Figure 12 shows the
minimal DFA Dex4 of the regular expression e. The minimal
|D
|
D-SFA Dex4 of Sex4 satisfies |Sex4 | = |Dex4 | ex4 .
⋄
By Example 4, we obtain the following fact.
Fact 2 If |Σ| ≥ 3, then there exists a regular expression e over
Σ whose minimal DFA D and minimal D-SFA Sd satisfies
|Sd | = |D||D| .
⋄
Facts 1 and 2 mean the existence of regular expressions
with three symbols that lead to state explosion in the construction of DFA or D-SFA. Here, we have another question: Is
there a regular expression with a constant number of symbols
that lead to state explosion in the construction of N-SFA from
NFA? The following fact on the semigroup theory gives a
negative answer to this question.
Fact 3 (Devadze [21], [22]) The size of a minimal generating set of the semigroup of n × n boolean matrices grows
exponentially with n.
⋄
This fact was first presented by Devadze in 1968, and he
described minimal sets of generators of the semigroup of n×n
boolean matrices without a proof. Its was proved very recently
by Konieczny in 2011 [22].
We stated in the previous section that the states in SFA
correspond to elements in syntactic monoid. Since the syntactic
monoid can be represented with boolean matrices and their
multiplication 2 , the theorem also applies to the syntactic
monoid.

Corollary 3.1 To denote a regular expression that leads to an
2
N-SFA Sn with |Sn | = 2k states, we require an exponential
number of states with respect to k.
⋄
Corollary 3.1 means that it is unrealistic to find a large regular
expression that leads to state explosion in the construction of
N-SFA.
VIII.

We have defined a novel class of automata called simultaneous finite automata, and developed an implementation of
them for efficient data-parallel regular expression matching.
The parallel computation of SFA runs in O(n/p + p) time or
in O(n/p + |D| log p) time where |D| is the number of states
in DFA, n is the length of input word and p is the number of
threads.
We tackled SFA’s size issue in Sect. VI-A, made experiments in real world regular expressions (SNORT rulesets),
and show that SFA’s size is fully practical in typical case. We
also made experiments with the SFA-based regular expression
matcher, and confirmed good scalability by a factor of over
10 on an environments with dual hexa-core CPUs and small
overhead such that execution with two threads outperforms for
input data over 600KB.
Our implementation of the SFA-based parallel regular
expression matcher is available as an open-source software [9],
hence anyone can verify the experimental results in Sect. VI.
Acknowledgments
We would like to thank Kazuhiro Inaba for the helpful
discussion with him on the prior works described in Sect. III
and Sect. VII-B.
This work was partly supported by the joint project between ANR (France) and JST (Japan) (project PaPDAS ANR2010-INTB-0205-02 and JST 10102704).
R EFERENCES
[1]

[2]

[3]
[4]

[5]

[6]
2 See [23], [19] for the relation between the syntactic monoid and boolean

matrices. Theorem 3 in [23] is a proof for Fact 2. In the semigroup theory, the
problem corresponding to Fact 2 is one of basic propositions ([24], Exercise
6).

C ONCLUSION

S. Kumar, S. Dharmapurikar, F. Yu, P. Crowley, and J. Turner,
“Algorithms to accelerate multiple regular expressions matching
for deep packet inspection,” SIGCOMM Comput. Commun. Rev.,
vol. 36, no. 4, pp. 339–350, Aug. 2006. [Online]. Available:
http://doi.acm.org/10.1145/1151659.1159952
B. C. Brodie, D. E. Taylor, and R. K. Cytron, “A scalable architecture
for high-throughput regular-expression pattern matching,” SIGARCH
Comput. Archit. News, vol. 34, no. 2, pp. 191–202, May 2006.
[Online]. Available: http://doi.acm.org/10.1145/1150019.1136500
R. E. Ladner and M. J. Fischer, “Parallel prefix computation,” Journal
of the ACM, vol. 27, no. 4, pp. 831–838, 1980.
W. D. Hillis and G. L. Steele, Jr., “Data parallel algorithms,” Commun.
ACM, vol. 29, no. 12, pp. 1170–1183, Dec. 1986. [Online]. Available:
http://doi.acm.org/10.1145/7902.7903
J. Holub and S. Štekr, “On parallel implementations of deterministic
finite automata,” in Implementation and Application of Automata, 14th
International Conference, CIAA 2009, Proceedings, ser. Lecture Notes
in Computer Science, vol. 5642. Springer, 2009, pp. 54–64.
D. Luchaup, R. Smith, C. Estan, and S. Jha, “Multi-byte regular
expression matching with speculation,” in Proceedings of the 12th
International Symposium on Recent Advances in Intrusion Detection,
ser. RAID ’09. Berlin, Heidelberg: Springer-Verlag, 2009, pp. 284–303.
[Online]. Available: http://dx.doi.org/10.1007/978-3-642-04342-0 15

<!-- page 10 -->

[7]

[8]

[9]
[10]

[11]

[12]

[13]
[14]

[15]
[16]
[17]

[18]

——, “Speculative parallel pattern matching,” Trans. Info. For.
Sec., vol. 6, no. 2, pp. 438–451, Jun. 2011. [Online]. Available:
http://dx.doi.org/10.1109/TIFS.2011.2112647
Y. Ko, M. Jung, Y.-S. Han, and B. Burgstaller, “A speculative parallel
DFA membership test for multicore, SIMD and cloud computing
environments,” CoRR, vol. abs/1210.5093, 2012.
R. Sin’ya, “Regen: regular expression generator, engine, JIT-compiler.”
[Online]. Available: http://sinya8282.github.com/Regen/
P. D. Stotts and W. Pugh, “Parallel finite automata for modeling
concurrent software systems,” Journal of Systems and Software, vol. 27,
pp. 27–43, 1994.
M. Jantzen, M. Kudlek, and G. Zetzsche, “Concurrent finite automata,”
in Tagungsband 17. Theorietag Automaten und Formale Sprachen,
M. Droste and M. Lohrey, Eds., 2007, pp. 84–88.
A. K. Chandra, D. C. Kozen, and L. J. Stockmeyer, “Alternation,”
J. ACM, vol. 28, no. 1, pp. 114–133, Jan. 1981. [Online]. Available:
http://doi.acm.org/10.1145/322234.322243
J. Sakarovitch, Elements of Automata Theory. Cambridge University
Press, 2009.
M. O. Rabin and D. Scott, “Finite automata and their decision problems,” IBM Journal of Research and Development, vol. 3, no. 2, pp.
114–125, 1959.
R. Cox, “Regular expression matching can be simple and fast,” 2009.
[Online]. Available: http://swtch.com/∼ rsc/regexp/regexp1.html
A. V. Aho, R. Sethi, and J. D. Ullman, Compilers: Principles, Techniques, and Tools, second edition ed. Prentice Hall, 2006.
R. McNaughton and H. Yamada, “Regular expressions and state graphs
for automata,” IRE Transactions on Electronic Computers, vol. EC-9,
no. 1, pp. 39–47, 1960.
M. Roesch, “Snort: Lightweight intrusion detection for networks,” in
Proceedings of the 13th Conference on Systems Administration (LISA99). USENIX, 1999, pp. 229–238.

[19]

[20]

[21]
[22]

[23]

[24]

J.-E. Ṕin, Syntactic Semigroups. Springer-Verlag, 1997, vol. 1, ch. 10,
pp. 679–746, g. Rozenberg and A. Salomaa (eds). Handbook of Formal
Languages.
J. A. Brzozowski, B. Li, and Y. Ye, “Syntactic complexity of prefix-,
suffix-, and bifix-free regular languages,” in Descriptional Complexity
of Formal Systems — 13th International Workshop, DCFS 2011, Proceedings, ser. Lecture Notes in Computer Science, vol. 6808, 2011, pp.
93–106.
H. M. Devadze, “Generating sets of the semigroup of all binary relations
on a finite set.” 1968, pp. 765–768, russian.
J. Konieczny, “A proof of Devadze’s theorem on generators of the
semigroup of boolean matrices.” Semigroup Forum, vol. 83, no. 2, pp.
281–288, 2011.
M. Holzer and B. König, “On deterministic finite automata and syntactic
monoid size,” Theoretical Computer Science, vol. 327, no. 3, pp. 319–
347, 2004.
J. M. Howie, Fundamentals of Semigroup Theory (London
Mathematical Society Monographs New Series). Oxford University
Press, USA, Feb. 1996.
