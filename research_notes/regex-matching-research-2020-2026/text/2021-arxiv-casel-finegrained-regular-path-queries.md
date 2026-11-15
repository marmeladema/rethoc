---
file: ../pdfs/2021-arxiv-casel-finegrained-regular-path-queries.pdf
citation: "Fine-Grained Complexity of Regular Path Queries; Casel, Schmid; arXiv:2101.01945 (published LMCS 19(4), 2023); 2021"
doi: 
source: "https://arxiv.org/pdf/2101.01945"
topics: [semantics_theory_access]
pages: 39
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Logical Methods in Computer Science
Volume 19, Issue 4, 2023, pp. 15:1–15:39
https://lmcs.episciences.org/

Submitted
Published

Oct. 29, 2021
Nov. 27, 2023

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES
KATRIN CASEL

a

AND MARKUS L. SCHMID

b

a

Hasso Plattner Institute, University of Potsdam, Potsdam, Germany
e-mail address: Katrin.Casel@hpi.de

b

Humboldt-Universität zu Berlin, Unter den Linden 6, D-10099, Berlin, Germany
e-mail address: MLSchmid@MLSchmid.de

Abstract. A regular path query (RPQ) is a regular expression q that returns all node pairs
(u, v) from a graph database that are connected by an arbitrary path labelled with a word
from L(q). The obvious algorithmic approach to RPQ-evaluation (called PG-approach),
i. e., constructing the product graph between an NFA for q and the graph database, is
appealing due to its simplicity and also leads to efficient algorithms. However, it is unclear
whether the PG-approach is optimal. We address this question by thoroughly investigating
which upper complexity bounds can be achieved by the PG-approach, and we complement
these with conditional lower bounds (in the sense of the fine-grained complexity framework).
A special focus is put on enumeration and delay bounds, as well as the data complexity
perspective. A main insight is that we can achieve optimal (or near optimal) algorithms with
the PG-approach, but the delay for enumeration is rather high (linear in the database). We
explore three successful approaches towards enumeration with sub-linear delay: super-linear
preprocessing, approximations of the solution sets, and restricted classes of RPQs.

1. Introduction
An essential component of graph query languages (to be found both in academical prototypes
as well as in industrial solutions) are regular path queries (RPQs). Abstractly speaking,
a regular expression q over some alphabet Σ is interpreted as query that returns from a
Σ-edge-labelled, directed graph D (i. e., a graph database) the set q(D) of all node pairs (u, v)
that are connected by a q-path, i. e., a path labelled with a word from q’s language (and
possibly also a witness path per node pair, or even all such paths). This simple, yet relevant
concept has heavily been studied in database theory (the following list is somewhat biased
towards recent work): results on RPQs [CMW87, BBG20, LM13, MNT20, MT19b, BT16],
Key words and phrases: Graph Databases, Regular Path Queries, Enumeration, Fine-Grained Complexity.
This is the full version of the article [CS21]. The first author has been funded by the Federal Ministry of
Education and Research of Germany (BMBF) in the KI-LAB-ITSE framework – project number 01IS19066.
The second author has been funded by the German Research Foundation (Deutsche Forschungsgemeinschaft,
DFG) – project number 416776735 (gefördert durch die Deutsche Forschungsgemeinschaft (DFG) – Projektnummer 416776735).
2012 ACM Subject Classification: Theory of computation → Regular languages; Theory of computation
→ Problems, reductions and completeness; Theory of computation → Database query languages (principles);
Theory of computation → Data structures and algorithms for data management.

l

LOGICAL METHODS
IN COMPUTER SCIENCE

DOI:10.46298/LMCS-19(4:15)2023

© K. Casel and M. L. Schmid
CC
⃝
Creative Commons

<!-- page 2 -->

15:2

K. Casel and M. L. Schmid

Vol. 19:4

conjunctive RPQs [BOS15, RRV17, BBMT17] and extensions thereof [LMV16, BLLW12,
LMV16, FS13], questions of static analysis [FGK+ 20, BFR19, Fig20, GMO19, RBV17],
experimental analyses [BMT17, BMT20, MT19a], and surveys of this research area [Bar13,
Woo12, AAB+ 17, CDLV03].
In the simplest setting, where we are only interested in the node pairs (but no paths)
connected by arbitrary q-paths (instead of, e. g., simple paths), evaluation can be done
efficiently. Deviating from this simple setting, however, leads to intractability: if we
ask for nodes connected by simple paths (no repeated nodes), or connected by trails (no
repeated arcs), then RPQ evaluation is NP-hard even in data-complexity (see [MW95,
BBG20, MT18] and [MNT20], respectively). Note that the simple path and trail semantics
are mostly motivated by the fact that under these semantics there is only a finite number of
q-paths per node pair. If we move to conjunctions of RPQs (CRPQs) or even more powerful
extensions motivated by practical requirements, then also with the arbitrary path semantics
evaluation becomes intractable in combined complexity (i. e., they inherit hardness from
relational conjunctive queries (CQs)).
In order to guide practical developments in the area of graph databases, the computational hard cases of RPQ (and CRPQ) evaluation have been thoroughly investigated in
database theory. However, with respect to arbitrary q-paths, research seems to have stopped
at the conclusion that efficient evaluation is possible by the following simple PG-approach:
given graph database D = (VD , ED ) and NFA Mq for q with state set Vq , construct the
product graph GD,q with nodes VD × Vq and an arc ((u, p), (v, p′ )) iff, for some a ∈ Σ, D has
an arc (u, a, v) and Mq has an arc (p, a, p′ ), and then use simple graph-searching techniques
on GD,q .
The PG-approach is explicitly defined in several papers, e. g., [MW95, Bar13, MT18], and
mainly used to prove a worst-case upper bound (actually O(|q|| D |) for Boolean evaluation;
in [MT18] it is used for enumerating q-paths between two given nodes. But it is also
very appealing from a practical point of view due to its simplicity: we are just coupling
well-understood algorithmic concepts like finite automata and graph reachability algorithms.
Arguably, implementing the PG-approach is an exercise suitable for a first year programming
course (making it feasible and cost-efficient for industrial systems). As it seems, putting
RPQ evaluation with arbitrary path semantics and the respective PG-approach into the
focus of a thorough theoretical study has not yet been done. This paper is devoted to this
task. In particular, we wish to investigate the following two (somewhat overlapping) aspects:
(1) Applicability of the PG-approach: the PG-approach is suited for solving simple evaluation
problems like checking q(D) = ∅ or (u, v) ∈ q(D) (for given u, v ∈ VD ), but is it also
appropriate for more relevant tasks like computing, counting or enumerating q(D)?
(2) Optimality of the PG-approach: Does the PG-approach lead to optimal algorithms, or
can it be beaten by conceptionally different techniques?
Answering these questions provides a better theoretical understanding of RPQ-evaluation
(which, as mentioned above, are at the heart of many graph query languages). But also
for the more powerful CRPQs and more practically motivated graph query languages, we
can derive valuable insights from our investigation. Let us mention two such examples
(a complete summary of our results follows further down). As noted in [Bar13], we can
reduce CRPQ evaluation to the evaluation of relational CQs by first constructing all tables
represented by the single RPQs and then evaluating a CQ over this database. To do this, we
first have to compute the results of all RPQs, so it seems helpful to know the best algorithms

<!-- page 3 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

Non-enum. Results
upper
bounds
lower
bounds

Boole,
Test,
Eval
Witness
O(| D ||q|)
O(|VD || D ||q|)
O((|VD ||q|)ω )

OV & com-BMM O((| D ||q|)1−ϵ )
OV
—
SBMM
—
—
com-BMM

—
—

15:3

Count
O(|VD || D ||q|)
O((|VD ||q|)ω )
—

Odc ((|VD || D |)1−ϵ )
Odc (|q(D)| + | D |) —
Odc ((|VD || D |)1−ϵ ) —

Table 1: All upper bounds can be achieved as running times of some algorithm, while
the lower bounds cannot be achieved as running time by any algorithm, unless
the displayed hypothesis fails. The exponent ω denotes the best known matrix
multiplication exponent.
for this intermediate task. Moreover, if we want to benefit from the existing CQ evaluation
techniques (e. g., exploiting acyclicity etc.) we are more or less forced to this two-step
approach. With respect to enumerations of CQs, it is known that linear preprocessing
and constant delay enumeration is possible provided that the CQs satisfy certain acyclicity
properties (see [BDG07, BKS17], or the surveys [BGS20, Seg15]). Unfortunately, these
techniques do not carry over to CRPQs since, as we show, linear preprocessing and constant
delay enumeration is not possible even for single RPQs (conditional to some complexity
assumptions).
Since the problem we investigate can be solved in polynomial time (also in combined
complexity), we cannot show lower bounds in terms of hardness results for complexity classes
like NP or PSPACE. Instead, we make use of the framework of fine-grained complexity, which
allows to prove lower bounds that are conditional on some algorithmic assumptions (see the
surveys [Wil15, Bri19, Wil18]). In particular, fine-grained complexity is a rather successful
toolbox for giving evidence that the obvious algorithmic approach to some basic problem, is
also the optimal one. This is exactly our setting here, with respect to RPQ-evaluation and
the PG-approach. To the knowledge of the authors, such conditional lower bounds are not
yet a well-established technique in database theory (however, see [BGS20, Section 6] for a
survey of conditional lower bounds in the context of CQ enumeration).
A main challenge is that fine-grained complexity is not exactly tailored to either the
data-complexity perspective or to enumeration problems. We will next outline our results.

1.1. Our Contribution. All investigated RPQ-evaluation problems are summarised on
page 8 (see especially Table 3). In the following, D = (VD , ED ) is the graph database, q is the
RPQ, and ϵ > 0. With the notation Odc (·), we hide factors f (|q|) for some function f (i. e.,
it is used for stating data-complexities). All lower bounds mentioned in the following are
conditional to some of the algorithmic assumptions summarised in Section 4 (we encourage
the reader less familiar with fine-grained complexity hypotheses to have a look at this
section first, which can be read independently). For presentational reasons, we do not always
explicitly mention this in the rest of the introduction and when we say that a certain running
time is “not possible”, this statement is always conditional in this sense (see Tables 1 and 2
for the actual hypotheses). As common in fine-grained complexity, we rule out true sub-linear

<!-- page 4 -->

15:4

K. Casel and M. L. Schmid

Vol. 19:4

(O(n1−ϵ )), sub-quadratic (O(n2−ϵ )), or sub-cubic (O(n3−ϵ )) running times, but not possible
n3
running time improvements by logarithmic factors, e. g., O( log(n)
).
1.1.1. Non-Enumeration Variants. The following results are summarised in Table 1. For
the simple problems Boole (checking q(D) = ∅), Test (checking (u, v) ∈ q(D)) and
Witness (computing some element from q(D)), the PG-approach yields an upper bound of
O(| D ||q|), which is optimal (since linear) in data complexity, and we can show lower bounds
demonstrating its optimality also in combined complexity. For Eval (computing the set
q(D)) the PG-approach yields a data complexity upper bound of Odc (|VD || D |), which cannot
be improved by combinatorial algorithms, although Odc (|VD |2.37 ) is possible by fast matrix
multiplication (see Section 4 for a discussion of the meaning of the term “combinatorial”).
In addition, we can show that linear time data complexity, i. e., Odc (|q(D)| + | D |), is not
possible even for non-combinatorial algorithms. For Count (computing |q(D)|), we get
Odc (|VD || D |) as upper and lower bound, not restricted to combinatorial algorithms.
1.1.2. Enumeration. Our results for RPQ-enumeration are summarised in Table 2. An entry
“O(delay)” in column “preprocessing” means that the preprocessing is bounded by the delay
(which means that no preprocessing is required). The column “sorted” indicates whether
the enumeration is produced lexicographically sorted.
In comparison to the non-enumeration problem variants, the picture is less clear and
deserves more explanation. The PG-approach yields a simple enumeration algorithm with
delay O(| D ||q|), that also trivially supports updates in constant time, since the preprocessing
fits into the delay bound. Our lower bounds for Boole also mean that this delay cannot
be improved in terms of combined complexity. While this lower bound was interesting for
problems like Boole etc., it now gives a correct answer to the wrong question. The main
goal now should be to find out whether we can remedy the linear dependency of the delay
on | D |, at the expense of spending more time in terms of |q|, or of losing the ability of
handling updates, or even of allowing a slightly super-linear preprocessing.
In this regard, the strongest result would be linear preprocessing O(| D |f (|q|)) and
constant delay O(f (|q|)). However, we can rule this out even for algorithms not capable of
handling updates. Then, the next question is which non-constant delays can be achieved
that are strictly better than linear. For example, none of our lower bounds for the nonenumeration variants suggest that linear preprocessing and a delay bounded by, e. g., |VD |
or the degree of D, should not be possible. We are not able to answer this question in
its general form (and believe it to be very challenging), but we are able to provide several
noteworthy insights.
For linear preprocessing, a delay of O(|VD |) (if possible at all) cannot be beaten by
combinatorial algorithms (even without updates). This can be strengthened considerably, if
we also require updates in some reasonable time: for general algorithms (i. e., not necessarily
combinatorial) delay and update time strictly better than Odc (|VD |) is not possible even with
arbitrary preprocessing, and for combinatorial algorithms with linear preprocessing even
delay and update time of O(| D |) cannot be beaten. This last result nicely complements the
upper bound at least for combinatorial algorithms and in the dynamic case.
In summary, for linear preprocessing, O(|VD |) is a lower bound for the delay and if we
can beat O(| D |), we should not be able to also support updates.

<!-- page 5 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

Enum. Results
upper
bounds
lower
bounds

OV & com-BMM
SBMM
com-BMM
OMv
com-BMM

Enum
preprocessing

delay

sorted updates

O(delay)

O(| D ||q|)



O(1)

O(delay)

O((| D ||q|)1−ϵ )
Odc (1)
Odc (|VD |1−ϵ )
Odc (|VD |1−ϵ )
Odc (|VD |2−ϵ )

×
×
×
×
×

×
×
×
Odc (|VD |1−ϵ )
Odc (|VD |2−ϵ )

Odc (| D |)
Odc (| D |)
arbitrary
Odc (| D |)

15:5

Table 2: All upper bounds can be achieved as running times of some algorithm, while
the lower bounds cannot be achieved as running time by any algorithm, unless
the displayed hypothesis fails. The exponent ω denotes the best known matrix
multiplication exponent.
1.1.3. Enumeration of Restricted Variants. Finally, we obtain restricted problem variants
that can be solved with delay strictly better than O(| D |) (in data complexity). We explore
three different approaches:
(1) by allowing super-linear preprocessing of Odc (∆(D) log(∆(D))| D |) (where ∆(D) is the
average degree of D), we can achieve a delay of O(|VD |);
(2) in linear preprocessing and constant delay, we can enumerate a representative subset of
q(D) instead of the whole set q(D);
(3) for a subclass of RPQs, we can solve RPQ-Enum with linear preprocessing and delay
O(∆(D)) (where ∆(D) is the maximum degree of D).

2. Main Definitions
Let N = {1, 2, 3, . . .} and [n] = {1, 2, . . . , n} for n ∈ N. For a finite alphabet A, A+ denotes
the set of non-empty words over A and A∗ = A+ ∪ {ε} (where ε is the empty word). For
a word w ∈ A∗ , |w| denotes its length; w1 = w and wk = wwk−1 for every k ≥ 2. For
L, K ⊆ A∗ , let LS· K = {w1 · w2 | w1 ∈ L, w2 ∈ K}, let L1 = L and Lk = L · Lk−1 for every
k ≥ 2, let L+ = k≥1 Lk and L∗ = L+ ∪ {ε}.
2.1. Σ-Graphs. We now define the central graph model that is used to represent graph
databases as well as finite automata. Let Σ be a finite alphabet of constant size. A Σ-graph
is a directed, edge labelled multigraph G = (V, E), where V is the set of vertices (or nodes)
and E ⊆ V × (Σ ∪ {ε}) × V is the set of edges (or arcs). For u ∈ V and x ∈ Σ ∪ {ε},
Ex (u) = {v | (u, x, v) ∈ E} is the set of x-successors of u. A path from w0 ∈ V to wk ∈ V
of length k ≥ 0 is a sequence p = (w0 , a1 , w1 , a2 , w2 . . . , wk−1 , ak , wk ) with (wi−1 , ai , wi ) ∈ E
for every i ∈ [k]. We say that p is labelled with the word a1 a2 . . . ak ∈ Σ∗ . According to this
definition, for every v ∈ V , (v) is a path from v to v of length 0 that is labelled by ε. Hence,
every node v of a Σ-graph has an ε-labelled path to itself, even though there might not be
an ε-arc from v to v. Moreover, due to ε as a possible edge-label, paths of length k may be
labelled with words w with |w| < k. The size of G = (V, E) is |G| = max{|V |, |E|}.

<!-- page 6 -->

15:6

K. Casel and M. L. Schmid

Vol. 19:4

For any Σ-graph G = (V, E), we call (V, {(u, v) | u ̸= v ∧ ∃x ∈ Σ ∪ {ε} : (u, x, v) ∈ E})
the underlying graph of G (note that the underlying graph is simple, non-labelled and has no
loops). In particular, by a slight abuse of notation, we denote by E ∗ the reflexive-transitive
closure of the underlying graph of G. Since we always assume |Σ| to be a constant, we
have that |G| = Θ(max{|V |, |{(u, v) | u ̸= v ∧ ∃x ∈ Σ ∪ {ε} : (u, x, v) ∈ E}|}) (i. e., |G| is
asymptotically
equal to the size of its underlying graph). For every u ∈ V , the degree of u is
S
∆(u) = | x∈Σ∪{ε} Ex (u)| (so ∆(u) is actually the out-degree), and the maximum degree of
P
G is ∆(G) = max{| ∆(u)| | u ∈ V }. The average degree of G is ∆(G) = |V1 | u∈V | ∆(u)|.
Obviously, ∆(G) ≤ ∆(G) ≤ |V |.
Since Σ-graphs are the central data structures for our algorithms, we have to discuss
implementational aspects of Σ-graphs in more detail. The set V of a Σ-graph G = (V, E)
is represented as a list, and, for every u ∈ V and for every x ∈ Σ ∪ {ε}, we store a list of
all x-successors of u, which is called the x-adjacency list for u. We assume that we can
check in constant time whether a list is empty and we can insert elements in constant time.
However, finding and deleting an element from a list requires linear time. Furthermore, we
assume that we always store together with a node a pointer to its adjacency list (thus, we
can always retrieve the x-adjacency list for a given node in constant time).
Remark 2.1. All lower bounds presented in this paper hold for any graph representation
that can be constructed in time linear in |G| = max{|V |, |E|}. For the upper bounds, we
chose the simple representation with adjacency lists as it emerged as the natural structure
for our enumeration approach; let us point out here that since we always store pointers
to the adjacency lists along with the nodes, we can perform a breadth-first search (BFS)
from any given start node u in time O(|G|). It is a plausible assumption that most specific
graph representations can be transformed into our list-based representation without much
effort. This ensures a certain generality of our upper complexity bounds in the sense that
the corresponding algorithms are, to a large extent, independent from implementational
details. Note also that the list-based structure only requires space linear in |G|.
In the adjacency list representation, we do not have random access to specific nodes in
the graph database, or to specific neighbours of a given node. Thus, we have to measure
a non-constant running-time for performing such operations. However, the algorithms for
our upper bounds are independent from this aspect, i. e., the total running times would not
change if we assume random access to nodes in constant time.
An exception to this is Theorem 7.1, for which we can obtain some small improvement
by applying the technique of lazy array initialization (see Remark 7.2).
For a Σ-graph G = (V, E), we denote by GR = (V, E R ) the Σ-graph obtained from G
by reversing all arcs, i. e., E R = {(v, x, u) | (u, x, v) ∈ E}.
Lemma 2.2. Let G = (V, E) be a Σ-graph. Then GR can be computed in time O(|G|).
Proof. Since GR = (V, E R ), it is sufficient to show how the adjacency lists can be computed
that represent E R . To avoid confusion with respect to whether we talk about the Σ-graph
G or the Σ-graph GR to be constructed, we denote the x-adjacency lists by x-G-adjacency
lists or x-GR -adjacency lists, respectively.
We first move through the list for V and, for every u ∈ V , we store this node in an
array along with an empty x-GR -adjacency list for every x ∈ Σ ∪ {ε}. This requires time
O(|V |). Then, for every u ∈ V and x ∈ Σ ∪ {ε}, we move through the x-G-adjacency list
for u, and for every element v that we encounter, we add u to the x-GR -adjacency list for

<!-- page 7 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:7

v. Since we can access all x-GR -adjacency lists in constant time, we only have to add as
many elements to some x-GR -adjacency as there are edges in E, i. e., the second step can be
done in time O(|E|). Consequently, we can construct all x-GR -adjacency lists in total time
O(|V | + |E|) = O(|G|).

2.2. Graph Databases and Regular Path Queries. A nondeterministic finite automaton
(NFA for short) is a tuple M = (G, S, T ), where G = (V, E) is a Σ-graph (the nodes q ∈ V
are also called states), S ⊆ V with S ̸= ∅ is the set of start states and T ⊆ V with T ̸= ∅
is the set of final states. The language L(M ) of an NFA M is the set of all labels of paths
from some start state to some final state. For a Σ-graph G = (V, E), any subsets S, T ⊆ V
with S ̸= ∅ ̸= T induce the NFA (G, S, T ). If S = {s} and T = {t} are singletons, then we
also write (G, s, t) instead of (G, {s}, {t}).
The set REΣ of regular expressions (over Σ) is recursively defined as follows: a ∈ REΣ for
every a ∈ Σ ∪ {ε}; (α · β) ∈ REΣ , (α ∨ β) ∈ REΣ , and (α)+ ∈ REΣ , for every α, β ∈ REΣ . For
any α ∈ REΣ , let L(α) be the regular language described by the regular expression α defined
as usual:1 for every a ∈ Σ∪{ε}, L(a) = {a}, and for every α, β ∈ REΣ , L(α ·β) = L(α)·L(β),
L(α ∨ β) = L(α) ∪ L(β) and L(α+ ) = L(α)+ . We also use α∗ as short hand form for α+ ∨ ε.
By |α|, we denote the length of α represented as a string.
Proposition 2.3. Every regular expression α can be transformed in time O(|α|) into an
equivalent NFA M = (G, p0 , pf ) with |G| = O(|α|).
Proof. Let α ∈ REΣ . We first construct the syntax tree Tα of α with node set VTα . Obviously,
Tα has size O(|α|) and we can obtain VTα from α in time O(|α|) (for example, we can transform
the expression α to prefix notation and then construct Tα while moving through the prefix
notation of α from left to right). We now construct an NFA Mα = (G = (V, E), {p0 }, {pf })
from Tα as follows (recall that G is a Σ-graph and therefore it should adhere to our
representations of Σ-graphs). We first construct an array of size 2|VTα | that contains the
nodes of V = {t1 , t2 | t ∈ VTα } and we initialise empty x-adjacency lists for all these nodes
and for every x ∈ Σ ∪ {ε}. Then we move through Tα top-down and if the current node t is
an inner node with two children r and s, then we do the following:
• If t corresponds to a concatenation ·, then we add r1 to the ε-adjacency list of t1 , we add
s1 to the ε-adjacency list of r2 , and we add t2 to the ε-adjacency list of s2 .
• If t corresponds to an alternation ∨, then we add r1 to the ε-adjacency list of t1 , we add
s1 to the ε-adjacency list of t1 , we add t2 to the ε-adjacency list of r2 , and we add t2 to
the ε-adjacency list of s2 .
If t is an inner node with one child r (which means it necessarily corresponds to a +), then
we add r1 to the ε-adjacency list of t1 , we add t2 to the ε-adjacency list of r2 , and we add t1
to the ε-adjacency list of t2 . If t is a leaf labelled with x ∈ Σ ∪ {ε}, then we add t2 to the
x-adjacency list of t1 . Finally, if t is the root of Tα , we relabel t1 by p0 and we relabel t2
by pf .
It can be easily verified that L(Mα ) = L(α). By construction |V | = O(|Tα |) = O(|α|)
and, since G has constant degree, we also have that |G| = O(|V |) = O(|α|). Moreover, in
the construction we spend constant time per arc that is added, so the whole construction of
Mα can be done in time O(|α|).
1As usual, we use the same notation L(·) both for regular expressions as well as finite automata.

<!-- page 8 -->

15:8

Name
RPQ-Boole
RPQ-Test
RPQ-Witness
RPQ-Eval
RPQ-Count
RPQ-Enum

K. Casel and M. L. Schmid

Input
D, q
D, q, u, v
D, q
D, q
D, q
D, q

Vol. 19:4

Task
Decide whether qB (D) = true.
Decide whether (u, v) ∈ q(D).
Compute a witness (u, v) ∈ q(D) or report that none exists.
Compute the whole set q(D)
Compute |q(D)|.
Enumerate the whole set q(D).

Table 3: The investigated RPQ-evaluation problems (D is a graph database, q an RPQ and
u, v two nodes from D).
In the following, when we speak about an automaton (or an NFA) for a regular expression
α, we always mean an NFA equivalent to α with the properties asserted by Proposition 2.3.
A Σ-graph without ε-arcs is also called a graph database (over Σ); in the following,
we denote graph databases by D = (VD , ED ). Since VD is represented as a list, any graph
database implicitly represents a linear order on VD (i. e., the order induced by the list that
represents VD ), which we denote by ⪯D , or simply ⪯ if D is clear from the context. A class
C of graph databases is called sparse if there is a constant c such that |ED | ≤ c|VD | for every
D ∈ C. Slightly abusing notation, we shall also call single graph databases sparse to denote
that we are dealing with a graph database from a sparse class of graph databases.
Regular expressions q (over alphabet Σ) are interpreted as regular path queries (RPQ) for
graph databases (over Σ). The result q(D) of an RPQ q on a graph database D = (VD , ED )
over Σ is the set q(D) = {(u, v) | u, v ∈ VD , L((D, u, v)) ∩ L(q) ̸= ∅}.
If we interpret q as a Boolean RPQ, then the result is qB (D) = true if q(D) ̸= ∅ and
qB (D) = false otherwise. We consider the RPQ-evaluation problems summarised in Table 3.
By sorted RPQ-Enum (or semi-sorted RPQ-Enum), we denote the variant of RPQ-Enum,
where the pairs of q(D) are to be enumerated in lexicographical order with respect to ⪯D
(or ordered only with respect to their left elements, while successive pairs with the same
right element can be ordered arbitrarily, respectively).
Remark 2.4. If an order ⪯′ on VD is explicitly given as a bijection π : VD → {1, . . . , n},
then we can modify D (in O(|VD |)) such that ⪯D = ⪯′ . In this regard, sorted RPQ-Enum
just models the case where we wish the enumeration to be sorted according to some order.
In particular, by assuming the order ⪯D to be implicitly represented by D, we do not hide
the complexity of sorting n elements.
A graph database D is well-formed if VD = [n] for some n ∈ N and ⪯D corresponds to
≤ on [n].
Lemma 2.5. Let D be a graph database with |VD | = n. Then we can construct in time
O(| D |) a well-formed graph database D′ and an isomorphism π : [n] → VD between D′
and D.
Proof. Let D be a graph database with |VD | = n. We initialise an array A of size n that
can be addressed with the elements of VD and the entries of which can store numbers of [n],
and an array B of size n that can be addressed with the elements of [n] and the entries of
which can store elements of VD . Furthermore, we initialise a counter c = 1. Then we move
through the list for VD from left to right and for each element u that we encounter, we set

<!-- page 9 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:9

A[u] = c, B[c] = u and increment c. Obviously, A describes an isomorphism π : VD → [n]
and B describes π −1 . Moreover, this can be done in time O(|VD |).
Next, we make a copy D′ of D but replace each u ∈ VD by the number A[u] (note that
D is just a collection of lists that store elements of VD along with pointers to lists). This
can be done in time O(| D |). Moreover, D′ is obviously isomorphic to D and B describes an
isomorphism [n] → VD .
Lemma 2.5 means that with an overhead of time O(| D |), we can always assume that
our input graph databases are well-formed; in particular, note that with the isomorphism
π ensured by the lemma, we can always translate elements from q(D′ ), where D′ is the
well-formed graph database isomorphic to D, back to the corresponding elements of q(D).
Thus, Lemma 2.5 justifies that whenever we spend at least O(| D |) in some preprocessing,
we can always assume that the input graph database is well-formed.
2.3. General Algorithmic Framework for RPQ-Evaluation. We assume the RAMmodel with logarithmic word-size as our computational model. Let us next discuss our
algorithmic framework for RPQ-evaluation. The input to our algorithms is a graph database
D = (VD , ED ) and an RPQ q (and, for solving the problem RPQ-Test, also a pair (u, v) ∈
VD ).
In the case of RPQ-Enum, the algorithms have routines preprocess and enum. Initially,
preprocess performs some preliminary computations on the input or constructs some auxiliary
data-structures; the performance of preprocess is measured in its running-time depending on
the input size as usual (i. e., we treat preprocess as an individual algorithm). Then enum will
produce an enumeration (u1 , v1 ), (u2 , v2 ), . . . , (uℓ , vℓ ) such that q(D) = {(ui , vi ) | 1 ≤ i ≤ ℓ},
no element occurs twice, and the algorithm reports when the enumeration is done. We
measure the performance of enum in terms of its delay, which describes the time that (in
the worst-case) elapses between enumerating two consecutive elements, between the start
of the enumeration and the first element, and between the last element and the end of the
enumeration (or between start and end in case that q(D) = ∅). We say that (variants of)
RPQ-Enum can be solved with preprocessing p and delay d, where p and d are functions
bounding the preprocessing running-time and the delay. In the case that p = O(d), the
preprocessing complexity is absorbed by the delay; in this case, we say that (variants of)
RPQ-Enum can be solved with delay d and do not mention any bound on the preprocessing.
We also consider RPQ-Enum in the dynamic setting, i. e., there is the possibility to
perform update operations on the input graph database D, which trigger a routine update.
After an update and termination of the update routine, invoking enum is supposed to
enumerate q(D′ ), where D′ is the updated graph database. The performance of an algorithm
for RPQ-Enum is then measured in the running-times of routines preprocess (to be initially
carried out only once) and update, as well as the delay. We only consider the following types
of individual updates: inserting a new arc between existing nodes, deleting an arc, adding a
new (isolated) node, deleting an (isolated) node. In particular, deleting or adding a single
non-isolated node u may require a non-constant number of updates.
3. The Product Graph Approach
The PG-approach has already been informally described in the introduction; for our finegrained perspective, we need to define it in detail. Let D = (VD , ED ) be a graph database over

<!-- page 10 -->

15:10

K. Casel and M. L. Schmid

Vol. 19:4

some alphabet Σ and let q be an RPQ over Σ. Furthermore, let (Gq , p0 , pf ) with Gq = (Vq , Eq )
be an automaton for q. Recall that, according to Proposition 2.3, Gq can be obtained in
time O(|q|) and it has O(|q|) states and O(|q|) arcs. The product graph of D and Gq is the
Σ-graph G⊠ (D, q) = (V⊠ (D, q), E⊠ (D, q)), where V⊠ (D, q) = {(u, p) | u ∈ VD , p ∈ Vq } and
E⊠ (D, q) = {((u, p), x, (v, p′ )) | (u, x, v) ∈ ED , (p, x, p′ ) ∈ Eq } ∪
{((u, p), ε, (u, p′ )) | u ∈ VD , (p, ε, p′ ) ∈ Eq } .
Remark 3.1. The arc labels in G⊠ (D, q) are superfluous in the sense that we only need the
underlying graph of G⊠ (D, q) (see Lemma 3.3). We define it nevertheless as Σ-graph, since
then all our definitions and terminology for Σ-graphs introduced above apply as well.
Lemma 3.2. |V⊠ (D, q)| = O(|VD ||q|), |E⊠ (D, q)| = O(| D ||q|) and G⊠ (D, q) can be computed
in time O(|G⊠ (D, q)|) = O(| D ||q|).
Proof. We first note that |V⊠ (D, q)| ≤ |VD ||q| directly follows from the definition. Moreover,
the following is also immediate by definition:
|{((u, p), ε, (u, p′ )) | u ∈ VD , (p, ε, p′ ) ∈ Eq }| = O(|VD ||Eq |) = O(| D ||q|) and
|{((u, p), x, (v, p′ )) | (u, x, v) ∈ ED , (p, x, p′ ) ∈ Eq }| = O(|ED ||Eq |) = O(| D ||q|) .
Consequently, |E⊠ (D, q)| = O(| D ||q|).
For the question how G⊠ (D, q) can be computed, we have to keep in mind our list-based
implementation of Σ-graphs (see Section 2). For every u ∈ VD and every p ∈ Vq , we add
(u, p) to the list that stores V⊠ (D, q). This can be done by moving |VD | times through the
list for Vq . Thus, time O(|VD ||Vq |) = O(| D ||q|) is sufficient.
In order to construct the adjacency lists, we proceed as follows. Let u ∈ VD , p ∈ Vq and
x ∈ Σ. Then we add all (v, p′ ) to the x-adjacency list of (u, p), where v is an element of the
x-adjacency list of u, and p′ is an element of the x-adjacency list of p. Moreover, we add
all (u, p′ ) to the ε-adjacency list of (u, p), where p′ is an element of the ε-adjacency list of
p. This can be done by moving once through the lists of adjacency lists for ED and, for
each encountered element, to move through the lists of adjacency lists for Eq . Since each
insertion to a list can be done in constant time, the whole procedure can be carried out in
time O(|Σ|| D ||q|) = O(| D ||q|).
The following lemma, which is an immediate consequence of the construction, shows
how G⊠ (D, q) can be used for solving RPQ-evaluation tasks (recall that E ∗ is the reflexivetransitive closure of the underlying unlabelled graph).
Lemma 3.3. For every u, v ∈ VD , (u, v) ∈ q(D) if and only if ((u, p0 ), (v, pf )) ∈ (E⊠ (D, q))∗ .
4. Fine-grained Complexity and Conditional Lower Bounds
We now state several computational problems along with hypotheses regarding their complexity, which are commonly used in the framework of fine-grained complexity to obtain
conditional lower bounds. We discuss some details and give background information later
on.
• Orthogonal Vectors (OV): Given sets A, B each containing n Boolean-vectors of dimension d, check whether there are vectors ⃗a ∈ A and ⃗b ∈ B that are orthogonal.
OV-Hypothesis: For every ϵ > 0 there is no algorithm solving OV in time O(n2−ϵ poly(d)).

<!-- page 11 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:11

• Boolean Matrix Multiplication (BMM): Given Boolean n×n matrices A, B, compute A×B.
com-BMM-Hypothesis: For every ϵ > 0 there is no combinatorial algorithm that solves
BMM in time O(n3−ϵ ).
• Sparse Boolean Matrix Multiplication (SBMM): Like BMM, but all matrices are represented
as sets {(i, j) | A[i, j] = 1} of 1-entries.
SBMM-Hypothesis: There is no algorithm that solves SBMM in time O(m) (where m is the
total number of 1-entries, i. e., m = |{(i, j) | A[i, j] = 1}| + |{(i, j) | B[i, j] = 1}| + |{(i, j) |
(A × B)[i, j] = 1}|).
• Online Matrix-Vector Multiplication (OMv): Given Boolean n × n-matrix M and a sequence
⃗v 1 , ⃗v 2 , . . . , ⃗v n of n-dimensional Boolean vectors, compute the sequence M⃗v 1 , M⃗v 2 , . . . , M⃗v n ,
where M⃗v i is produced as output before ⃗v i+1 is received as input.
OMv-Hypothesis: For every ϵ > 0 there is no algorithm that solves OMv in time O(n3−ϵ ).
We will reduce these problems to variants of RPQ evaluation problems in such a way that
algorithms with certain running-times for RPQ evaluation would break the corresponding
hypotheses mentioned above. Thus, we obtain lower bounds for RPQ evaluation that are
conditional to these hypotheses. In the following, we give a very brief overview of the
relevance of these problems and corresponding hypotheses in fine-grained complexity.
The problem OV can be solved by brute-force in time O(n2 d) and the hypothesis
that there is no subquadratic algorithm is well-established. It exists in slightly different
variants and has been formulated in several different places in the literature (e. g., [Bri14,
Bri19, Wil15]). The variant used here is sometimes referred to as moderate dimension
OV-hypothesis in contrast to low dimension variants, where d can be assumed to be rather
small in comparison to n. The relevance of the OV-hypothesis is due to the fact that it is
implied by the Strong Exponential Time Hypothesis (SETH) [Wil04, Wil05], and therefore it
is a convenient tool to prove SETH lower bounds that has been applied in various contexts.
One of the most famous computational problems is BMM, which, unfortunately, is
a much less suitable basis for conditional lower bounds. The straightforward algorithm
solves it in time O(n3 ), but there are fast matrix multiplication algorithms that run in time
O(n2.373 ) [Wil12b, Gal14]. It is unclear how much further this exponent can be decreased
and there is even belief that BMM can be solved in time n2+o(1) (see [Wil12a] and [BGS20,
Section 6]). However, these theoretically fast algorithms cannot be considered efficient in
a practical sense, which motivates the mathematically informal notion of “combinatorial ”
algorithms (see, e. g., [WW18]).2 So far, no truly subcubic combinatorial BMM-algorithm
exists and it has been shown in [WW18] that BMM is contained in a class of problems
(including other prominent examples like Triangle Finding (also mentioned below) and
Context-Free Grammar Parsing) which are all equivalent in the sense that if one such
problem is solvable in truly subcubic time by a combinatorial algorithm, then all of them
are. Consequently, it is often possible to argue that the existence of a certain combinatorial
algorithm for some problem would imply a major (and unlikely) algorithmic breakthrough
with respect to BMM, Parsing, Triangle Finding, etc. Despite the defect of relying on the
vague notion of combinatorial algorithms, this lower bound technique is a common approach
in fine-grained complexity (see, e. g., [WW18, HKNS15, AW14, AWY18, ABW18, HLNW17]).
Whenever we use the com-BMM-hypothesis, our reductions will always be combinatorial,
which is necessary; moreover, whenever we say that a certain running time cannot be achieved
2The term “combinatorial algorithm” is not well-defined, but intuitively such algorithms have running-times

with low constants in the O-notation, and are feasibly implementable.

<!-- page 12 -->

15:12

K. Casel and M. L. Schmid

Vol. 19:4

unless the com-BMM-hypothesis fails, we mean, of course, that it cannot be achieved by a
combinatorial algorithm.
In order to make BMM suitable as base problem for conditional lower bounds (that
does not rely on combinatorial algorithms) one can formulate the weaker (i. e., more
plausible) hypothesis that BMM cannot be solved in time linear in the number of 1-entries
of the matrices (therefore called sparse BMM since matrices are represented in a sparse
way); see [AP09, YZ05]. Another approach is to require the output matrix A × B to be
computed column by column, i. e., formulating it as the online-version OMv. For OMv,
subcubic algorithms are not known and would yield several major algorithmic breakthroughs
(see [HKNS15]).
A convenient tool to deal with BMM is the problem Triangle: check whether a given
undirected graph G has a triangle. This is due to the fact that these two problems
are subcubic equivalent with respect to combinatorial algorithms (see [WW18]), i. e., the
com-BMM-hypothesis fails if and only if Triangle can be solved by a combinatorial algorithm
in time O(n3−ϵ ) for some ϵ > 0. Thus, for lower bounds conditional to the com-BMMhypothesis, we can make use of both these problems. There is also a (non-combinatorial)
Triangle-hypothesis that states that Triangle cannot be solved in linear time in the number
of edges, but we were not able to apply it in the context of RPQ-evaluation (see [AW14] for
different variants of Triangle).
5. Bounds for the Non-Enumeration Problem Variants
We now investigate how well the PG-approach performs with respect to the non-enumeration
variants of RPQ-evaluation, and we give some evidence that, in most cases, it can be
considered optimal or almost optimal (subject to the algorithmic hypotheses of Section 4).
5.1. Boolean Evaluation, Testing and Computing a Witness. It is relatively straightforward to see that the problems RPQ-Test and RPQ-Boole are equivalent and can both
be reduced to RPQ-Witness. Hence, upper bounds for RPQ-Witness and lower bounds
for RPQ-Test or RPQ-Boole automatically apply to all three problem variants, which
simplifies the proofs for such bounds. We shall now formally prove this.
Lemma 5.1. Let (D, q) be an RPQ-Boole-instance. Then we can construct an equivalent
RPQ-Test-instance (D′ , q ′ , u, v) with | D′ | = O(| D |) and |q ′ | = O(|q|) in time O(| D | + |q|).
Proof. Let D and q be defined over Σ. We transform D into a graph database D′ over
Σ ∪ {#} (where # ∈
/ Σ) by adding new nodes u and v, and new arcs (u, #, x) and (x, #, v)
for every x ∈ VD . Moreover, we set q ′ = #q#. This construction can be carried out in time
O(| D | + |q|) and we can also note that | D′ | = O(| D |) and |q ′ | = O(|q|).
It remains to show that qB (D) = true if and only if (u, v) ∈ q ′ (D′ ). If qB (D) = true, then
there is some (u′ , v ′ ) ∈ q(D), which means that in D there is a path u′ , . . . , v ′ that is labelled
with a word w ∈ L(q). Since there are arcs (u, #, u′ ) and (v ′ , #, v) in D′ , there is a path
u, u′ , . . . , v ′ , v in D′ that is labelled with #w# ∈ L(q ′ ). Therefore, (u, v) ∈ q ′ (D′ ). On the
other hand, if (u, v) ∈ q ′ (D′ ), then there is some path u, u′ , . . . , v ′ , v in D′ that is labelled
with #w# ∈ L(q ′ ), so therefore also (u′ , v ′ ) ∈ q(D) and qB (D) = true (note that u′ = v ′ and
therefore w = ε is also possible).
Lemma 5.2. Let (D, q, u, v) be an RPQ-Test-instance. Then we can construct an equivalent RPQ-Boole-instance (D′ , q ′ ) with | D′ | = O(| D |) and |q ′ | = O(|q|) in time O(| D |+|q|).

<!-- page 13 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:13

Proof. Let D and q be defined over Σ. We transform D into a graph database D′ over
Σ ∪ {#} (where # ∈
/ Σ) by adding new nodes s and t with arcs (s, #, u) and (v, #, t), and
we define q ′ = #q#. This construction can be carried out in time O(| D | + |q|) and we can
also note that | D′ | = O(| D |) and |q ′ | = O(|q|).
It remains to show that (u, v) ∈ q(D) if and only if qB′ (D′ ) = true. If (u, v) ∈ q(D),
then there is a path from u to v in D that is labelled with a word from L(q). Since there
are arcs (s, #, u) and (v, #, t) in D′ , there is a path in D′ from s to t labelled with a word
from L(#q#); thus (s, t) ∈ q ′ (D′ ) and therefore qB′ (D′ ) = true. On the other hand, if
qB′ (D′ ) = true, then we can conclude that (s, t) ∈ q ′ (D′ ), which is due to the fact that
q ′ = #q#, q does not contain any occurrence of # and the only arcs labelled with # have
source s and target t. This implies that there is a path s, u, . . . , v, t labelled with a word
#w# with w ∈ L(q), which implies that there is a path from u to v labelled with a word
from L(q), which means that (u, v) ∈ q(D).
Theorem 5.3. Let f : N × N → N be some polynomial function with f (n1 , n2 ) = Ω(n1 + n2 ).
(1) RPQ-Test can be solved in time f (| D |, |q|) if and only if RPQ-Boole can be solved
in time f (| D |, |q|).
(2) If RPQ-Witness can be solved in time f (| D |, |q|), then RPQ-Boole and RPQ-Test
can be solved in time f (| D |, |q|).
Proof. Lemmas 5.1 and 5.2 directly imply Point 1. Moreover, an algorithm that solves
RPQ-Witness implicitly solves RPQ-Boole as well. Together with Point 1, this proves
Point 2.
We are now ready to prove that the PG-approach yields the following upper bound.
Theorem 5.4. The problems RPQ-Test, RPQ-Boole and RPQ-Witness can be solved
in time O(| D ||q|).
Proof. We only show the upper bound for RPQ-Witness (due to Theorem 5.3 it applies to
RPQ-Test and RPQ-Boole as well). To this end, let D = (VD , ED ) be a graph database
over Σ and let q be an RPQ over Σ. We construct G⊠ (D, q), which, according to Lemma 3.2,
can be done in time O(| D ||q|). In the following considerations, we interpret G⊠ (D, q) as its
underlying non-labelled graph.
We add to G⊠ (D, q) a node vsource with an arc to each (u, p0 ) with u ∈ VD . This can be
done in time O(|VD ||q|) as follows. First, we add vsource to the list that represents V⊠ (D, q),
which requires constant time. Then, we move through the list that represents V⊠ (D, q) and
every node (u, p0 ) that we encounter is added to the adjacency list for vsource .
Next, we perform a special kind of BFS in vsource . First, we construct an array S of
size |V⊠ (D, q)| the entries of which can store values from V⊠ (D, q) ∪ {0} and which can be
addressed by the nodes from V⊠ (D, q). Moreover, S is initialised with every entry storing 0.
Then, for every u ∈ VD , we set S[(u, p0 )] = u. This can be done in time O(|V⊠ (D, q)|) =
O(| D ||q|). We perform a BFS from vsource and whenever we traverse an arc ((u, p), (u′ , p′ )),
we set S[(u′ , p′ )] = S[(u, p)]. This can be done in time O(|G⊠ (D, q)|) = O(| D ||q|). Since
S[(u, p0 )] = u for every u ∈ VD , we can conclude by induction that, for every (v, p) ∈ V⊠ (D, q),
if (v, p) is reachable from some node (u, p0 ), then S[(v, p)] = u′ for some u′ ∈ VD such that
(v, p) is reachable from (u′ , p0 ), and if (v, p) is not reachable from any node (u, p0 ), then
S[(u, p)] = 0. Consequently, if there is some v ∈ VD with S[(v, pf )] = u ̸= 0, then there
is a path from (u, p0 ) to (v, pf ) in G⊠ (D, q), which, according to Lemma 3.3, means that
(u, v) ∈ q(D) and therefore, we can produce the output (u, v). On the other hand, if there

<!-- page 14 -->

15:14

K. Casel and M. L. Schmid

Vol. 19:4

is no v ∈ VD with S[(v, pf )] ̸= 0, then there are no u, v ∈ VD with a path from (u, p0 ) to
(v, pf ); thus, q(D) = ∅. Checking whether there is some v ∈ VD with S[(v, pf )] ̸= 0 can be
done in time O(|VD |).
More interestingly, we can complement this upper bound with lower bounds as follows.
Theorem 5.5. If any of the problems RPQ-Test, RPQ-Boole and RPQ-Witness can
be solved in time O(| D |2−ϵ +|q|2 ) or O(| D |2 +|q|2−ϵ ) for some ϵ > 0, then the OV-hypothesis
fails. This lower bound also holds for the restriction to sparse graph databases.
Proof. We prove the lower bound for RPQ-Test only, since by Theorem 5.3 it also applies to
RPQ-Witness and RPQ-Boole. We first devise a general reduction from the OV-problem
to RPQ-Test (which is similar to the reduction from [BI16] used for proving conditional
lower bounds of regular expression matching):
Let A = {⃗a1 , . . . , ⃗an } and B = {⃗b1 , . . . , ⃗bn } be an instance for the OV-problem. We
define an RPQ q and a graph database D over the alphabet Σ = {0, 1, #} as follows. For
every i ∈ [n], let wi = ⃗bi [1]⃗bi [2] . . . ⃗bi [d], and let q = #(w1 ∨ w2 ∨ . . . ∨ wn )#. Moreover, let
D = (VD , ED ), where VD contains nodes s and t and, for every i ∈ [n], nodes vi,0 , vi,1 , . . . , vi,d .
For every i ∈ [n] and j ∈ {0} ∪ [d − 1], there is an arc from vi,j to vi,j+1 labelled with
0 and, if ⃗ai [j + 1] = 0, also an arc from vi,j to vi,j+1 labelled with 1. Finally, for every
i ∈ [n], there are arcs labelled with # from s to vi,0 and from vi,d to t. It can be easily
verified that there are orthogonal ⃗a ∈ A and ⃗b ∈ B if and only if (s, t) ∈ q(D). Moreover,
| D | = O(|A|d) = O(nd) and |q| = O(|B|d) = O(nd), and, furthermore, D and q can also be
constructed in time O(nd), and, since |ED | = O(|VD |), D is a sparse graph database.
We now assume that RPQ-Test can be solved in time O(| D |2 + |q|2−ϵ ) for some ϵ > 0.
Let again A = {⃗a1 , . . . , ⃗an } and B = {⃗b1 , . . . , ⃗bn } be an instance for the OV-problem, and let ϵ′
′
be arbitrarily chosen with 0 < ϵ′ < ϵ. We divide A into A1 , A2 , . . . , A⌈nϵ′ ⌉ with |Ai | = ⌈n1−ϵ ⌉
S⌈nϵ′ ⌉
′
for every i ∈ [⌈nϵ ⌉] and such that i=1 Ai = A. Obviously, (A, B) is a positive OV-instance
if and only if at least one of (A1 , B), (A2 , B), . . . , (A⌈nϵ′ ⌉ , B) is a positive OV-instance. We
can now separately reduce each (Ai , B) to an RPQ-Test-instance (Di , q) as described above
′
and we note that | Di | = O(|Ai |d) = O(⌈n1−ϵ ⌉d) and |q| = O(|B|d) = O(nd). Then we solve
each instance in the assumed time bound O(| D |2 + |q|2−ϵ ), which requires total time of
′
′
′
′
′
O(⌈nϵ ⌉((⌈n1−ϵ ⌉d)2 + (nd)2−ϵ ) = O((n2−ϵ + n2−ϵ+ϵ ) poly(d)) = O(n2−(ϵ−ϵ ) ) poly(d)), where
(ϵ − ϵ′ ) > 0. This contradicts the OV-hypothesis.
The assumption that RPQ-Test can be solved in time O(| D |2−ϵ + |q|2 ) for some ϵ > 0
can be handled analogously. We divide again A into A1 , A2 , . . . , A⌈nϵ′ ⌉ as described above,
but then we reduce the OV-instances (B, A1 ), (B, A2 ), . . . , (B, A⌈nϵ′ ⌉ ) to the RPQ-Testinstances (D, q1 ), (D, q2 ), . . . , (D, q⌈nϵ′ ⌉ ). Note that | D | = O(nd) and |qi | = O(|Ai |d) =
′

O(⌈n1−ϵ ⌉d). By assumption, each of these instances can be solved in time O(| D |2−ϵ +|qi |2 ) =
′
′
O((nd)2−ϵ + (⌈n1−ϵ ⌉d)2 ), which again leads to a total running-time of O(⌈nϵ ⌉((nd)2−ϵ +
′
′
(⌈n1−ϵ ⌉d)2 )) = O(n2−(ϵ−ϵ ) ) poly(d)), where (ϵ − ϵ′ ) > 0. This contradicts the OV-hypothesis.
Since (| D ||q|)1−ϵ ≤ ((max{| D |, |q|})2 )1−ϵ = max{| D |2−2ϵ , |q|2−2ϵ } ≤ | D |2−ϵ + |q|2 ,
Theorem 5.5 also rules out running times of the form O((| D ||q|)1−ϵ ) and O(max{| D |, |q|}2−ϵ ),
but does not exclude a running time of O(| D |2−ϵ + f (|q|)) with f (|q|) = Ω(|q|2 ). However,
for ϵ < 1 this is super-linear in | D | (and therefore inferior to O(| D ||q|) under the assumption

<!-- page 15 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:15

|q| ≪ | D |, i. e., that the database is much larger than the query), and for ϵ = 1, we would
obtain O(| D | + f (|q|)), which, under the assumption |q| ≪ | D |, is a small and arguably
negligible asymptotic improvement over O(| D ||q|).
If the size of D is expressed in terms of |VD |, then Theorem 5.4 also gives an upper
bound of O(|VD |2 |q|). In this regard, we can show the following lower bound.
Theorem 5.6. If any of the problems RPQ-Test, RPQ-Boole and RPQ-Witness can
be solved in time O(|VD |3−ϵ + |q|3−ϵ ) for some ϵ > 0, then the com-BMM-hypothesis fails.
Proof. We prove the lower bound for RPQ-Boole only, since by Theorem 5.3 it also applies
to RPQ-Witness and RPQ-Boole.
We assume that RPQ-Boole can be solved by a combinatorial algorithm in time
O(|VD |3−ϵ + |q|3−ϵ ) for some ϵ > 0. We can then solve Triangle on some instance G = (V, E)
with V = {v1 , v2 , . . . , vn } as follows (the result will then follow from the combinatorial
subcubic equivalence of BMM and Triangle (see Section 4)). We construct the graph
database D over {a, #} with nodes
VD = {s′ , t′ } ∪ {sj , tj , | 1 ≤ j ≤ n} ∪ {(u, i) | 0 ≤ i ≤ 3, u ∈ V }
and arcs
ED ={((u, i), a, (u′ , i + 1)) | 0 ≤ i ≤ 2, (u, u′ ) ∈ E} ∪
{(s′ , #, s1 ), (si , a, si+1 ), (ti , a, ti+1 ), (tn , #, t′ ) | 1 ≤ i ≤ n − 1} ∪
{(si , a, (vi , 0)), ((vi , 3), a, ti ) | 1 ≤ i ≤ n} .
Furthermore, we define the RPQ q = #an+4 #.
By definition, q(D) = {(s′ , t′ )} or q(D) = ∅. We call any path p from s′ to t′ an i-j-path
for some i, j ∈ [n] if there are ℓ1 , ℓ2 ∈ [n] such that
p = (s′ , s1 , . . . , si , vi,0 , vℓ1 ,1 , vℓ2 ,2 , vj,3 , tj , . . . , tn , t′ ) .
It can be easily seen that any path from s′ to t′ is an i-j-path for some i, j ∈ [n], and that
any i-j-path is labelled with #ai a3 an−j+1 # = #ai+n−j+4 #. Hence, q(D) ̸= ∅ if and only if
there is an i-i-path for some i ∈ [n]. Since, for every i ∈ [n], there is an i-i-path if and only
if there is a path (vi,0 , vℓ1 ,1 , vℓ2 ,2 , vi,3 ), we see that, for every i ∈ [n], there is an i-i-path if
and only if G has triangle that contains vi . Consequently, q(D) ̸= ∅ if and only if G has a
triangle.
This means that by first constructing D and q in time O(|D| + |q|) = O(|G| + |V |)
and then solving RPQ-Boole for instance (D, q) in the assumed time O(|VD |3−ϵ + |q|3−ϵ )
for some ϵ > 0, means that we can solve Triangle by a combinatorial algorithm in time
O(|VD |3−ϵ + |q|3−ϵ ) = O(|V |3−ϵ ).
With Lemma 5.3, we conclude that the assumptions that RPQ-Test or RPQ-Witness
can be solved in time O(|VD |3−ϵ + |q|3−ϵ ) for some ϵ > 0, leads to a combinatorial O(|V |3−ϵ )
algorithm for Triangle as well. Thus, due to the combinatorial subcubic equivalence of
BMM and Triangle (see Section 4), there is a combinatorial O(n3−ϵ ) algorithm for BMM,
contradicting the com-BMM-hypothesis.
Since O((| D ||q|)1−ϵ ) ⊆ O((|VD |2 |q|)1−ϵ ) ⊆ O(|VD |3−ϵ + |q|3−ϵ ), such running-times are
also ruled out under the com-BMM-hypothesis. Especially, a combinatorial algorithm with
running-time O((| D ||q|)1−ϵ ) refutes both the OV- and the com-BMM-hypothesis; thus, such
an algorithm does not exist provided that at least one of these hypotheses is true (basing lower
bounds on several hypotheses is common in fine-grained complexity, see, e. g., [AWY18]).

<!-- page 16 -->

15:16

K. Casel and M. L. Schmid

Vol. 19:4

The lower bounds discussed above are only meaningful for combined complexity. However,
the upper bound of Theorem 5.4 already yields the optimum of linear data complexity.

5.2. Full Evaluation and Counting. The following upper bound is again a straightforward
application of the PG-approach.
Theorem 5.7. RPQ-Eval can be solved in time O(|VD || D ||q|).
Proof. Let D = (VD , ED ) be a graph database over Σ and let q be an RPQ over Σ. We
construct G⊠ (D, q), which, according to Lemma 3.2, can be done in time O(| D ||q|). We
interpret G⊠ (D, q) as its underlying non-labelled graph. According to Lemma 3.3, we can
now compute q(D) by performing a BFS from each node (u, p0 ) with u ∈ VD . Each such
BFS requires time O(|G⊠ (D, q)|) = O(| D ||q|), which means that the total running-time is
O(|VD ||G⊠ (D, q)|) = O(|VD || D ||q|).
Instead of using graph-searching techniques on G⊠ (D, q), we could also compute the
complete transitive closure of G⊠ (D, q) with fast matrix multiplication.
Theorem 5.8. If BMM can be solved in time O(nω ) with ω ≥ 2, then RPQ-Eval can be
solved in time O(|VD |ω |q|ω ).
Proof. We assume that BMM can be solved in time O(nω ). Let D = (VD , ED ) be a graph
database over Σ and let q be an RPQ over Σ. We construct G⊠ (D, q), which, according
to Lemma 3.2, can be done in time O(| D ||q|), and we interpret it as its underlying nonlabelled graph. Obviously, we can turn the adjacency list-based representation of G⊠ (D, q)
into an adjacency matrix-based representation in time O(|V⊠ (D, q)|2 ) = O((|VD ||q|)2 ) =
O((|VD ||q|)ω ). Then, we compute the transitive closure (E⊠ (D, q))∗ of G⊠ (D, q), which can
be done in time O(|V⊠ (D, q)|ω ) = O((|VD ||q|)ω ) (see [Mun71]). In order to obtain q(D), it is
sufficient to go through all elements ((u, p), (v, p′ )) ∈ (E⊠ (D, q))∗ and add (u, v) to a new
set if and only if p = p0 and p′ = pf (see Lemma 3.3). Since |(E⊠ (D, q))∗ | ≤ |V⊠ (D, q)|2 =
O(|V⊠ (D, q)|ω ) = O((|VD ||q|)ω ), this can be done in time O((|VD ||q|)ω ).
We mention this theoretical upper bound for completeness, but stress the fact that
our main interest lies in combinatorial algorithms. In addition to the limitation that
algorithms for fast matrix multiplication are not practical, we also observe that the approach
of Theorem 5.8 is only better if the graph database is not too sparse, i. e., only if |VD || D | =
Ω(|VD |ω ).
Next, we investigate the question whether O(|VD || D ||q|) is optimal for RPQ-Eval at
least with respect to combinatorial algorithms. Since for RPQ-Eval the PG-approach does
not yield an algorithm that is linear in data complexity (like it was the case with respect to
the problems of Section 5.1), the question arises whether the |VD || D | part can be improved
at the cost of spending more time in |q|. It seems necessary that respective data complexity
lower bounds need reductions that do not use q to represent a non-constant part of the
instance, as it was the case for both the OV and the Triangle reduction from Section 5.1.
It is not difficult to see that multiplying two n×n Boolean matrices reduces to RPQ-Eval
as formally stated by the next lemma.
Lemma 5.9. If RPQ-Eval can be solved in time O(|VD |ω f (|q|)) for some function f and
ω ≥ 2, then BMM can be solved in time O(nω ).

<!-- page 17 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:17

Proof. We assume that RPQ-Eval can be solved in time O(|VD |ω f (|q|)) for some function f .
Let A and B be n × n Boolean matrices. Then we construct the graph database DA,B over
{a} with VDA,B = {(i, 0), (i, 1), (i, 2) | i ∈ [n]} and EDA,B = {((i, 0), a, (j, 1)) | A[i, j] = 1} ∪
{((i, 1), a, (j, 2)) | B[i, j] = 1}, and the RPQ q = aa. Obviously, q(DA,B ) ⊆ {((i, 0), (j, 2)) |
i, j ∈ [n]} and ((i, 0), (j, 2)) ∈ q(DA,B ) if and only if (A × B)[i, j] = 1. Hence, we can
construct A × B from q(DA,B ) in time O(|q(DA,B )|) = O(n2 ). By assumption, the set q(D)
can be computed in time O(|VDA,B |ω f (|q|)) = O(nω f (2)) = O(nω ).
From this, we can conclude the following lower bound.
Theorem 5.10. If RPQ-Eval can be solved in time O((|VD || D |)1−ϵ f (|q|)) for some function f and some ϵ > 0, then the com-BMM-hypothesis fails.
Proof. If RPQ-Eval can be solved by a combinatorial algorithm with a running time
in O((|VD || D |)1−ϵ f (|q|)) for some function f and some ϵ > 0, then, since we have that
O((|VD || D |)1−ϵ f (|q|)) = O(|VD |3−ϵ f (|q|)), Lemma 5.9 implies that BMM can be solved in
time O(n3−ϵ ). Thus, the com-BMM-hypothesis fails.
If we drop the restriction to combinatorial algorithms, we can nevertheless show (with
more or less the same construction) that linear time in data complexity is impossible, unless
the SBMM-hypothesis fails. However, since the size of the output q(D) might be super-linear
in | D |, we should interpret linear as linear in | D | + |q(D)|.
Lemma 5.11. If RPQ-Eval can be solved in time O((|q(D)| + | D |)ω f (|q|)) for some
function f and ω ≥ 1, then SBMM can be solved in time O(mω ).
Proof. We assume that RPQ-Eval can be solved in time O((|q(D)| + | D |)ω f (|q|)) for some
function f . Let A and B be n × n Boolean matrices given as sets of their 1-entries and
let m be the total number of 1-entries in A, B and A × B. Then we can construct a
graph database DA,B over {a} as follows. For every i ∈ [n], the set VDA,B contains a node
(i, 0) if the ith row of A contains at least one 1-entry, a node (i, 1) if the ith column of A
contains at least one 1-entry or the ith row of B contains at least one 1-entry, and a node
(i, 2) if the ith column of B contains at least one 1-entry. The set of arcs is defined by
EDA,B = {((i, 0), a, (j, 1)) | A[i, j] = 1} ∪ {((i, 1), a, (j, 2)) | B[i, j] = 1}. We observe that
|EDA,B | = O(m) and, since every node in VDA,B has degree at least 1, |VDA,B | = O(|EDA,B |).
Consequently, | DA,B | = O(m) and DA,B can be constructed in time O(m). Further, we
define q = aa.
We can observe that q(DA,B ) ⊆ {((i, 0), (j, 2)) | i, j ∈ [n]} and, for every i, j ∈ [n],
((i, 0), (j, 2)) ∈ q(DA,B ) if and only if (A × B)[i, j] = 1. Thus, |q(DA,B )| = O(m) and we can
obtain a set of exactly the 1-entries of A × B from q(DA,B ) in time O(|q(DA,B )|) = O(m).
By assumption, we can compute q(DA,B ) in time O((|q(DA,B )| + | DA,B |)ω f (|q|)) for
some function f and ω ≥ 1. This implies that we can obtain a set of exactly the 1-entries of
A × B in time O((|q(DA,B )| + | DA,B |)ω f (|q|)) = O(mω f (2)) = O(mω ).
Lemma 5.11 directly implies the following lower bound.
Theorem 5.12. If RPQ-Eval can be solved in time O((|q(D)| + | D |)f (|q|)) for some
function f , then the SBMM-hypothesis fails.
Surprisingly, we can obtain a more complete picture for the problem RPQ-Count. First,
we observe that obviously all upper bounds carry over from RPQ-Eval to RPQ-Count.
On the other hand, a combinatorial O((|VD || D |)1−ϵ f (q)) algorithm or a general O((|q(D)| +

<!-- page 18 -->

15:18

K. Casel and M. L. Schmid

Vol. 19:4

| D |)f (|q|)) algorithm for RPQ-Count does not seem to help for solving Boolean matrix
multiplication (and therefore, the lower bounds do not carry over). Fortunately, it turns
out that OV is a suitable problem to reduce to RPQ-Count, although by a rather different
reduction compared to the one used for Theorem 5.5.
Theorem 5.13. If RPQ-Count can be solved in time O(| D |2−ϵ f (|q|)) for some function f
and ϵ > 0, then the OV-hypothesis fails.
Proof. We assume that RPQ-Count can be solved in time O(| D |2−ϵ f (|q|)) for some
function f and ϵ > 0. Let A = {⃗a1 , ⃗a2 , . . . , ⃗an } and B = {⃗b1 , ⃗b2 , . . . , ⃗bn } be an OV-instance,
i. e., for every i ∈ [n], ⃗ai and ⃗bi are d-dimensional Boolean vectors. Now let A′ be the
Boolean matrix having rows ⃗a1 , ⃗a2 , . . . , ⃗an and let B ′ be the Boolean matrix having columns
⃗b1 , ⃗b2 , . . . , ⃗bn . It can be easily seen that, for every i, j ∈ [n], (A′ × B ′ )[i, j] = 0 if and only if
⃗ai and ⃗bj are orthogonal. Moreover, A′ and B ′ can be constructed in time O(nd).
Next, we construct DA′ ,B ′ as in the proof of Lemma 5.9, and we note that VDA′ ,B′ =
O(n + d) and EDA′ ,B′ = O(nd); in particular, we construct DA′ ,B ′ in time O(nd). Since,
for every i, j ∈ [n], ((i, 0), (j, 2)) ∈ q(DA,B ) if and only if (A × B)[i, j] = 1, we know that,
for every i, j ∈ [n], ((i, 0), (j, 2)) ∈ q(DA,B ) if and only if ⃗ai and ⃗bj are not orthogonal.
Furthermore, since q(DA,B ) ⊆ {((i, 0), (j, 2)) | i, j ∈ [n]}, |q(DA,B )| = n2 if and only if there
are no ⃗a ∈ A and ⃗b ∈ B that are orthogonal. Consequently, we can check whether there are
no ⃗a ∈ A and ⃗b ∈ B that are orthogonal by computing |q(DA,B )|, which, by assumption, can
be done in time O(| DA,B |2−ϵ f (|q|)) = O((nd)2−ϵ f (2)) = O(n2−ϵ poly(d)).
Since Theorem 5.13 also excludes running time O((|VD || D |)1−ϵ f (|q|)) for any function
f and ϵ > 0 (without restriction to combinatorial algorithms), it also shows that, subject to
the OV-hypothesis, O(|VD || D |) is a tight bound for the data complexity of RPQ-Count.

6. Bounds for the Enumeration of RPQs
By using the PG-approach for enumeration, we can obtain the following upper bound.
Theorem 6.1. Sorted RPQ-Enum can be solved with O(| D ||q|) delay and O(1) updates.
Proof. Let D be a graph database over Σ, let q be an RPQ over Σ and let ⪯ be the order on
VD . We assume that VD = [n] with 1 ⪯ 2 ⪯ . . . ⪯ n (see Lemma 2.5).
Preprocessing:
(1) We compute G⊠ (D, q), which, according to Lemma 3.2, can be done in time O(| D ||q|),
and we interpret G⊠ (D, q) as its underlying non-labelled graph.
(2) We construct two arrays S and T of size |VD | such that, for every i ∈ [n], S[i] = (i, p0 )
and T [i] = (i, pf ). Note that this also means that pointers to the corresponding adjacency
lists are stored along with the nodes in S and T . Computing S and T can be done in
time O(|VD ||q|) as follows. We move through the list that represents V⊠ (D, q) and for
every node (i, p0 ), we set S[i] = (i, p0 ), and for every node (i, pf ), we set T [i] = (i, pf ).
(3) We modify G⊠ (D, q) by adding a new node vsink with an arc from each (i, pf ) with
i ∈ [n]. This can be done in time O(|VD |) as follows. We first add vsink to the list that
represents V⊠ (D, q), which requires constant time. Then, for every node (i, pf ) of T , we
add vsink to the adjacency list for (i, pf ). Again, this can be done in time O(|VD |).

<!-- page 19 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:19

(4) We obtain (G⊠ (D, q))R , which, by Lemma 2.2, can be done in time O(|G⊠ (D, q)|) =
O(| D ||q|). Then we initialise a Boolean array S ′ of size |VD | with entries 0 everywhere.
We then perform a BFS in (G⊠ (D, q))R starting in vsink and, for every visited node
(i, p0 ), we set S ′ [i] = 1. For every i ∈ VD , S ′ [i] = 1 if and only if vsink and therefore some
node from T can be reached from (i, p0 ). This step can be done in time O(| D ||q|).
(5) We initialise a Boolean array T ′ of size |VD | with entries 0 everywhere, which can be
done in time O(|VD |).
Enumeration: In the enumeration phase, we carry out the following procedure.
• For every i = 1, 2, . . . , n:
– If S ′ [i] = 1, then
∗ perform a BFS in (i, p0 ) and for every (j, pf ) that we visit, we set T ′ [j] = 1,
∗ for every j = 1, 2, . . . , n: produce (i, j) as output if T ′ [j] = 1,
∗ Set all entries of T ′ to 0.
Correctness: In the enumeration phase we perform a BFS from each (i, p0 ) that can reach
at least one node (j, pf ) (i. e., from each (i, p0 ) with S ′ [i] = 1) and after termination of
this BFS, we output exactly the pairs (i, j) for which (j, pf ) is visited in this BFS. This
directly shows that we correctly enumerate q(D) without repetitions. Furthermore, since
we consider the start vertices for the BFSs in increasing order with respect to ⪯, and since
after termination of a BFS started in (i, p0 ) we output the pairs (i, j) in increasing order by
the second element, the enumeration is sorted by lexicographic order.
Next, we estimate the delay of the enumeration. Each iteration of the main loop
with S ′ [i] = 1 requires time O(|G⊠ (D, q)|) for performing the BFS, O(|VD |) for producing
the output and O(|VD |) for resetting T ′ . Thus, the total running-time is O(|G⊠ (D, q)|) =
O(| D ||q|) in the case that S ′ [i] = 1. If, on the other hand, S ′ [i] = 0, then the iteration
terminates after constant time. Moreover, since S ′ [i] = 1 implies that i can reach vsink and
since vsink can only be reached via some (j, pf ), we produce at least one output in such an
iteration. Consequently, the delay between two outputs is bounded by the total running
time for one iteration of the main loop, which is O(| D ||q|).
Updates: If D is changed to D′ by an update, then we can again perform the whole
preprocessing (with respect to D′ ) followed by the enumeration procedure. Technically, the
preprocessing is done as a first step of the enumeration procedure (since our algorithmic
framework does not allow to re-run the preprocessing after an update), which is possible
since its running time of O(| D′ ||q|) is completely subsumed by the time available for the
first delay.
This enumeration algorithm is easy to implement and has some nice features like linear
preprocessing (in data complexity), sorted enumeration and constant updates. Unfortunately,
these features come more or less for free with the disappointing delay bound. Is the PGapproach therefore the wrong tool for RPQ-enumeration? Or can we give evidence that
linear delay is a barrier we cannot break? The rest of this work is devoted to this question.
Since running an algorithm for RPQ-Enum until we get the first element yields an
algorithm for RPQ-Boole (with preprocessing plus delay as running time), and since
running such an algorithm completely solves RPQ-Eval in time preprocessing plus |q(D)|
times delay, we can inherit several lower bounds directly from Section 5.
Theorem 6.2. If, for some function f and ϵ > 0, RPQ-Enum can be solved with
(1) delay O(| D |2−ϵ + |q|2 ) or O(| D |2 + |q|2−ϵ ), then the OV-hypothesis fails.

<!-- page 20 -->

15:20

K. Casel and M. L. Schmid

Vol. 19:4

(2) delay O(|VD |3−ϵ + |q|3−ϵ ), then the com-BMM-hypothesis fails.
(3) preprocessing O(| D |f (|q|)) and delay O(f (|q|)), then the SBMM-hypothesis fails.
(4) preprocessing O(|VD |3−ϵ f (|q|)) and delay O(|VD |1−ϵ f (|q|)), then the com-BMM-hypothesis
fails.
Proof. We prove the four lower bounds of Theorem 6.2 separately.
(1) Let us assume that RPQ-Enum can be solved with delay O(| D |2−ϵ + |q|2 ) (or O(| D |2 +
|q|2−ϵ )) for some ϵ > 0. Then we can solve RPQ-Boole on an instance (D, q) as
follows. We run the assumed enumeration algorithm for RPQ-Enum on (D, q), but
stop as soon as the first pair (if any) is produced. By assumption, this requires time
O(| D |2−ϵ + |q|2 ) (or O(| D |2 + |q|2−ϵ )). Moreover, (D, q) is a positive RPQ-Boole
instance if and only if the interrupted enumeration algorithm produces a pair. Thus, we
can solve RPQ-Boole in time O(| D |2−ϵ + |q|2 ) (or O(| D |2 + |q|2−ϵ )) for some ϵ > 0,
which, according to Theorem 5.5 means that the OV-hypothesis fails.
(2) Let us assume that RPQ-Enum can be solved with delay O(|VD |3−ϵ + |q|3−ϵ ) for some
ϵ > 0. Then we can solve RPQ-Boole on an instance (D, q) as follows. We run the
assumed enumeration algorithm for RPQ-Enum on (D, q), but stop as soon as the first
pair (if any) is produced. By assumption, this requires time O(|VD |3−ϵ +|q|3−ϵ ). Moreover,
(D, q) is a positive RPQ-Boole instance if and only if the interrupted enumeration
algorithm produces a pair. Thus, we can solve RPQ-Boole in time O(|VD |3−ϵ + |q|3−ϵ )
for some ϵ > 0, which, according to Theorem 5.6 means that the com-BMM-hypothesis
fails.
(3) We assume that RPQ-Enum can be solved with preprocessing O(| D |f (|q|)) and delay
O(f (|q|)) for some function f . By completely running the corresponding enumeration
algorithm, we can solve RPQ-Eval in time O(| D |f (|q|))+O(|q(D)|f (|q|)) = O((|q(D)|+
| D |)f (|q|)). According to Theorem 5.12 this means that SBMM-hypothesis fails.
(4) Assume that RPQ-Enum can be solved with preprocessing O(|VD |3−ϵ f (|q|)) and delay
O(|VD |1−ϵ f (|q|)) for some function f and ϵ > 0. By completely running the corresponding enumeration algorithm, we can solve RPQ-Eval in time O(|VD |3−ϵ f (|q|) +
|q(D)||VD |1−ϵ f (|q|)) = O(|VD |3−ϵ f (|q|)). According to Lemma 5.9, this means that BMM
can be solved in time O(n3−ϵ ).
The first two bounds only tell us that we might not be able to lower the delay O(| D ||q|)
in terms of combined complexity. While this point of view was justified for the problems
discussed in Section 5, it does not say anything regarding delays of the form O(| D |1−ϵ f (|q|)).
The third bound, conditional to the SBMM-hypothesis, is much more relevant, since it
suggests that the optimum of linear preprocessing and constant delay is not reachable. For
combinatorial algorithms, the fourth bound at least answers our main question with | D |
replaced by |VD |: with linear preprocessing, we cannot get below a delay of O(|VD |).
These lower bounds can be improved significantly, if we also want to handle updates
(within some reasonable time bounds).
Theorem 6.3. If RPQ-Enum can be solved with
(1) arbitrary preprocessing, O(|VD |1−ϵ f (|q|)) updates and O(|VD |1−ϵ f (|q|)) delay for some
function f and ϵ > 0, then the OMv-hypothesis fails.
(2) O(|VD |3−ϵ f (|q|)) preprocessing, O(|VD |2−ϵ f (|q|)) updates and O(|VD |2−ϵ f (|q|)) delay for
some function f and ϵ > 0, then the com-BMM-hypothesis fails.
Proof. We prove the two parts of Theorem 6.3 separately.

<!-- page 21 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:21

(1) Assume that there is an algorithm that solves RPQ-Enum with preprocessing tp , delay
td and update time tu . We can then solve OMv on some instance M and ⃗v 1 , ⃗v 2 , . . . , ⃗v n as
follows. We initialise the empty graph database (over alphabet {a}) and the RPQ q = aa,
and then we perform the preprocessing in constant time. Then, by performing O(n2 )
updates, we transform the current graph database into the graph database DM,⃗v1 over
{a} with nodes {ui , vi , w | 1 ≤ i ≤ n} and arcs {(ui , a, vj ) | M [i, j] = 1} ∪ {(vj , a, w) |
⃗v1 [j] = 1}, which requires time O(n2 tu ). Since q(DM,⃗v1 ) = {(ui , w) | (M⃗v 1 )[i] = 1},
we can now compute M⃗v 1 by enumerating q(DM,⃗v1 ) in time O(ntd ). In the same way,
for every i with 2 ≤ i ≤ n, we can obtain M⃗v i by enumerating q(DM,⃗vi ), where every
DM,⃗vi+1 can be obtained from DM,⃗vi by n updates in time O(ntu ). Consequently, the
whole procedure requires time O(n2 tu + ntd + (n − 1)(ntu + ntd )), and the number of
nodes always satisfies |VD | ∈ O(n). With the assumption that td = O(|VD |1−ϵ f (|q|))
and tu = O(|VD |1−ϵ f (|q|)) for some ϵ > 0 and some function f , this leads to a total
running-time of O(n2 |VD |1−ϵ f (|q|)) = O(n3−ϵ ).
(2) Assume that there is an algorithm that solves RPQ-Enum with preprocessing tp , delay
td and update time tu . We can then solve Triangle on some instance G = (V, E) with
V = {v1 , v2 , . . . , vn } as follows (the result will then follow from the combinatorial
subcubic equivalence of BMM and Triangle (see Section 4)). We construct the graph
database Dv1 over {a} with nodes VDv1 = {s, t, (u, i) | 0 ≤ i ≤ 3, u ∈ V } and arcs
EDv1 = {((u, i), a, (u′ , i + 1)) | 0 ≤ i ≤ 2, (u, u′ ) ∈ E} ∪ {(s, a, (v1 , 0)), ((v1 , 3), a, t)}.
Furthermore, we define the RPQ q = aaaaa. It can be easily seen that q(Dv1 ) = {(s, t)}
if there is a triangle that contains v1 , and q(Dv1 ) = ∅ otherwise. In particular, this
means that enumerating q(Dv1 ) can be done in time O(td ).
Analogously, by constructing Dv2 , . . . , Dvn and enumerating q(Dv2 ), . . . , q(Dvn ), we
can therefore check whether G contains a triangle. For every i with 2 ≤ i ≤ n, we
can obtain Dvi+1 from Dvi by performing a constant number of updates (we just have
to remove arcs (s, a, (vi , 0)) and ((vi , 3), a, t) and instead add arcs (s, a, (vi+1 , 0)) and
((vi+1 , 3), a, t)). Consequently, this whole procedure requires the preprocessing for a graph
database with O(|V |) nodes and O(|E|) arcs, |V | many delays and |V | many updates, i. e.,
the total running time of this procedure is O(tp + |V |(td + tu )). With the assumption that
tp = O(|VD |3−ϵ f (|q|)) and td = tu = O(|VD |2−ϵ f (|q|)) for some ϵ > 0 and some function f ,
this leads to a total running-time of O(|V |3−ϵ f (|q|) + |V ||VD |2−ϵ f (|q|)) = O(|V |3−ϵ ).
The first bound rules out that we can get below O(|VD |) for delay and update time,
regardless of the preprocessing; the second one analogously rules out anything below O(| D |)
(for combinatorial algorithms and linear preprocessing). While all these lower bounds suggest
that improving on the linear delay may be rather difficult, they leave the following case
open.
Question 6.4. Can RPQ-Enum be solved with O(| D |) preprocessing and O(|VD |) delay
in data complexity?
What is a reasonable conjecture with respect to this question? A combinatorial algorithm
that answers it in the positive does not seem to have any unlikely consequences. Indeed, it
would just entail an O(| D ||VD |) algorithm for RPQ-Eval (which exactly fits to Theorem 5.7),
an O(|V ||G|) algorithm for computing transitive closures and an O(n3 ) algorithm for
multiplying Boolean n × n matrices, which is the state of the art for combinatorial algorithms
(this is due to the obvious reductions from these problems to RPQ-Enum). However,

<!-- page 22 -->

15:22

K. Casel and M. L. Schmid

Vol. 19:4

since Question 6.4 is about enumeration, a positive answer also means that after O(|G|)
preprocessing, we can enumerate the transitive closure of a graph with delay O(|V |), and
that after O(n2 ) preprocessing, all 1-entries of the Boolean matrix multiplication can be
enumerated with delay O(n). Are such enumeration algorithms unlikely, so that we should
rather expect a negative answer to Question 6.4? In fact not, since for the simple RPQs
q = a∗ or q = aa, which are sufficient to encode transitive closures and Boolean matrix
multiplications, linear preprocessing and delay O(|VD |) is indeed possible, as we shall obtain
as byproducts of the results in the next section.
We close this section by the following remark that points out some similarity (and
differences) of reductions used in Sections 5 and 6.
Remark 6.5. As already mentioned in Section 5.1, the reduction used for the combined
complexity lower bounds of Theorems 5.5 (and therefore Point 1 of Theorem 6.2) is similar
to the reduction from [BI16] used for proving conditional lower bounds of regular expression
matching. Also note that [EMT21] improves the bound of [BI16] by showing that subquadratic algorithms for finding a path in a graph labelled by a given pattern string are
most likely impossible, even if we are allowed to build indexes in polynomial time. The
paper [EGMT19] strengthens the bound of [BI16] by restricting the structure of the graphs.
Moreover, the OV-lower bound for RPQ-Count of Theorem 5.13 is similiar to a lower bound
on counting the results of certain conjunctive queries from [BKS17], and the OMv-lower
bound from Point 1 of Theorem 6.3 is similiar to a lower bound on enumerating certain
conjunctive queries from [BKS17].
The quite simple observation that Boolean matrix multiplication can be expressed as
querying a bipartite graph (with conjunctive queries) has also been used in [BDG07] (see
also [BGS20, Section 6]) and is also the base for the OMv-lower bound of [BKS17]. In the
context of this paper, this connection has been used in the bounds of Theorems 5.10, 5.12
and Points 3 and 4 of 6.2.
The obvious connection between evaluating (non-acyclic) conjunctive queries and finding
triangles (or larger cliques) has already been observed in [Bra13] (see also [BGS20, Section 6]).
However, the Triangle-lower bounds of this paper are quite different, since RPQs cannot
explicitly express the structure of a triangle (or a larger clique, for that matter) by using
conjunction. Therefore, our respective lower bounds (Theorems 5.6, Point 2 of Theorem 6.2,
and Point 2 of Theorem 6.3) need to encode this aspect in a different way. With respect
to Theorems 5.6 and Point 2 of Theorem 6.2 this is done by using non-constant queries
(which explains why the lower bounds are not for data complexity, in contrast to the case of
conjunctive queries), and with respect to Point 2 of Theorem 6.3, which does work for data
complexity, it is done by using updates. Moreover, our Triangle-lower bounds do not seem to
extend to larger cliques like it is the case for conjunctive queries (see [BGS20, Section 6]).
Finally, we wish to point out that although some of the reductions used in this paper
are similar to reductions used in the context of conjunctive queries, due to the difference of
RPQs and CQs, none of the lower bounds directly carry over. Furthermore, note that the
lower bound reductions in [BDG07, BKS17] have been used for obtaining dichotomies and
therefore have been stated in a much more general way.

<!-- page 23 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:23

7. Enumeration with Sub-Linear Delay
We now explore three different approaches towards enumeration of q(D) with delay strictly
better than O(| D |) (in data complexity):
(1) allowing super-linear preprocessing,
(2) enumerating a representative subset of q(D), and
(3) restricting the RPQs.

7.1. Super-Linear Preprocessing. Regarding the first approach, we can improve the delay
from O(| D |) to O(|VD |) by increasing the linear preprocessing by a factor of ∆(D) log(∆(D))
(in data complexity). Recall that ∆(D) is the average degree of D.
Theorem 7.1. Sorted RPQ-Enum can be solved with O(|q|2 log(∆(D)|q|) ∆(D)| D |) preprocessing and O(|VD |) delay.
Proof. Let D be a graph database over Σ, let q be an RPQ over Σ and let ⪯ be the order
on VD . We assume that VD = [n] with 1 ⪯ 2 ⪯ . . . ⪯ n (see Lemma 2.5). Recall that ∆(D)
denotes the average degree of D, which in particular means that O(|ED |) = O(|VD | ∆(D)).
Preprocessing:
(1) Compute G⊠ (D, q) and interpret it as its underlying non-labelled graph. We also
compute the number ∆(D), which can be done by moving through the adjacency lists of
D and counting all edges.
(2) Let H = (VH , EH ) be the DAG of G⊠ (D, q)’s strongly connected components. Without
loss of generality, we assume that connected components are numbered with 1, 2, . . . , ℓ for
some ℓ ∈ N, i. e., VH = [ℓ], and we further assume that 1, 2, . . . , ℓ is a reverse topological
sorting of H. We can compute H as follows.
(a) By the algorithm of Tarjan [Tar72], we first compute the strongly connected
components K1 , K2 , . . . , Kℓ ⊆ V⊠ (D, q) of G⊠ (D, q). Since the finishing times of
the depth-first searches of this algorithm determine a reverse topological sorting of
H, we can assume that K1 , K2 , . . . , Kℓ describes a reverse topological sorting. We
represent the strongly connected components K1 , K2 , . . . , Kℓ by an array L of size
|VD ||q| addressable by the nodes of V⊠ (D, q), i. e., for every (i, p) ∈ V⊠ (D, q), we set
L[(i, p)] = j if and only if (i, p) ∈ Kj .
(b) In order to construct the actual graph H, we first initialise an empty adjacency
list for every node j ∈ VH = [ℓ] and also an additional empty list Pj . Then we
iterate through the list that represents V⊠ (D, q) and add every node (i, p) that we
encounter to the list PL[(i,p)] . For every j ∈ [ℓ], Pj now represents Kj as a list.
Next, we initialise an empty integer array W of size ℓ. Then, for each j ∈ [ℓ], we
iterate through the list Pj and for each node (i, p) that we encounter, we do the
following. For every (i′ , p′ ) in the adjacency list of (i, p), if W [L[i′ , p′ ]] ̸= j, then we
add L[(i′ , p′ )] to the adjacency list of j and set W [L[i′ , p′ ]] = j (the array W makes
sure that we do not add the same arc between strongly connected components
more than once, i. e., for each individual j ∈ [ℓ] considered in the above procedure,
W stores for each strongly connected component j ′ ∈ [ℓ] whether we have already
added the arc (j, j ′ )).
(3) Construct H R .

<!-- page 24 -->

15:24

K. Casel and M. L. Schmid

Vol. 19:4

(4) Initialise for each node j in H R an empty AVL tree Aj (or any search tree that allows
the operations insert, delete, lookup, delete-min and delete-max in worst-case time that
is logarithmic in its size), and store pointers to the roots of these trees and a counter for
their size in an array A of size ℓ, such that finding the tree associated to a specific node
can be done in constant time. For every i ∈ [n], insert i into the tree AL[(i,pf )] , and at
each point where the size of AL[(i,pf )] exceeds ∆(D)|q| during these insertions, delete the
node of largest index from AL[(i,pf )] .
(5) For j = 1, . . . , ℓ go through the adjacency list of node j in H R . For each arc (j, j ′ ) in
this list, insert one by one all entries from Aj into Aj ′ (first lookup to avoid duplicates).
Update also the size counter of the tree Aj ′ accordingly, and at each point where this
size exceeds ∆(D)|q| during these insertions, delete the node of largest index from Aj ′ .
This ensures that Aj ′ is kept at a size at most ∆(D)|q|.
(6) Initialise a Boolean array S of size ℓ with entries 0 everywhere. Create for each j ∈ [ℓ] a
sorted array Zj of all entries in Aj by successively extracting and deleting the smallest
entry. If Zj is not empty, set S[j] = 1.
(7) Initialise a Boolean array T of size |VD | with entries 0 everywhere.
Enumeration: In the enumeration phase, we carry out the following procedure.
• For every i = 1, 2, . . . , n:
– If S[L[(i, q0 )]] = 1, then
∗ If ZL[(i,q0 )] has size less than ∆(D)|q|, output all (i, j) for all j stored in Zi (in the
sorted order).
∗ If ZL[(i,q0 )] has size ∆(D)|q| perform a BFS in G⊠ (D, q) starting at node (i, p0 ) and
for every visited (i′ , pf ), set T [i′ ] = 1. Output during this BFS, all pairs (i, i′ ) for all i′
stored in ZL[(i,q0 )] (in the sorted order) to keep the desired delay, i.e., start with k = 1
and whenever the delay of |VD ||q| has expired output (i, Zi [k]) and set k = k + 1.
∗ for every i′ = ZL[(i,q0 )] [k], . . . , n: produce (i, i′ ) as output if T [i′ ] = 1,
∗ Set all entries of T to 0.
Running time: We first note that Steps 1 to 3 can be done in time O(|G⊠ (D, q)|) =
O(| D ||q|): Lemma 3.2 ensures that G⊠ (D, q) can be computed in O(| D ||q|), Lemma 2.2
ensures that H R can be computed in O(|H|) = O(| D ||q|), and it can be easily seen that in
order to construct H, we only have to move through G⊠ (D, q) a constant number of times;
thus, time O(|G⊠ (D, q)|) = O(| D ||q|) is sufficient.
Since the search trees never exceed a size of ∆(D)|q|, each operation supported by
the search trees can be carried out in time O(log(∆(D)|q|)). In Step 4, we first initialise
the search trees, which can be done in O(|VH |) = O(|V⊠ (D, q)|). Then, adding i to the
search tree AL[(i,pf )] for every i ∈ [n] can be done in time O(|VD | log(∆(D)|q|)). In Step 5,
we have to perform O(∆(D)|q|) search tree operations per arc of H, i. e., we need time
O(|EH | ∆(D)|q| log(∆(D)|q|)) = O(|G⊠ (D, q)| ∆(D)|q| log(∆(D)|q|)).
Step 6, we have to copy, for every j ∈ [ℓ], the elements from Aj to Zj , which requires
a total of ℓ ∆(D)|q| search tree operations, so time O(|G⊠ (D, q)| ∆(D)|q| log(∆(D)|q|)) is
sufficient (note that the time needed for initialising and filling S is clearly subsumed by this).
Finally, Step 7 can obviously be carried out in time O(|VD |). Hence, the total running time
for the preprocessing is O(|G⊠ (D, q)| ∆(D)|q| log(∆(D)|q|)) = O(| D ||q|2 ∆(D) log(∆(D)|q|)).

<!-- page 25 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:25

Correctness: We first show that after the preprocessing terminates, for every i ∈ [n],
ZL[(i,p0 )] contains the first ∆(D)|q| nodes i′ (“first” in the sense of smallest index) such that
the node (i′ , pf ) is reachable from (i, p0 ). In other words, ZL[(i,p0 )] contains the first ∆(D)|q|
elements of a lexicographically sorted enumeration of {(i, i′ ) | i′ ∈ [n], (i, i′ ) ∈ q(D)}. To this
end, we will show if i′ is among the ∆(D)|q| smallest reachable nodes from (i, p0 ), then i′ is
at some point inserted into AL[i,p0 ] , which means that i′ is among the nodes that are copied
to ZL[(i,p0 )] in Step 6 (recall that we only remove largest elements from some Aj if its size
exceeds ∆(D)|q|).
If (i, p0 ) and (i′ , pf ) are in the same strongly connected component in G⊠ (D, q), then
L[(i, p0 )] = L[(i′ , pf )] which means that in Step 4 the index i′ is inserted into AL[(i′ ,pf )] =
AL[(i,p0 )] . Otherwise there is a path in H from L[(i, p0 )] to L[(i′ , pf )], and reversing this
path yields a path from L[(i′ , pf )] to L[(i, p0 )] in H R . Denote by L[(i′ , pf )] = i1 , i2 , . . . , ix =
L[(i, p0 )] the vertices on the path from L[(i′ , pf )] to L[(i, p0 )] in H R . By the reverse topological
sorting on H, it follows that i1 < i2 < . . . < ix . In Step 4, i′ is put in Ai1 = AL[(i′ ,pf )] , and
iteratively in Step 5, i′ is copied from Aih to Aih+1 for 1 ≤ h < x, and therefore also into
Aix = AL[i,p0 ] .
In the enumeration procedure, we go through phases i = 1, 2, . . . , n and we will next
show that in phase i, we enumerate without duplicates exactly the pairs (i, i′ ) ∈ q(D) sorted
by their right element (this means that the enumeration procedure enumerates q(D) without
duplicates and lexicographically sorted).
If the size of ZL[(i,p0 )] is less than ∆(D)|q|, then, by the considerations above, we know
that ZL[(i,p0 )] contains all i′ with (i, i′ ) ∈ q(D). Thus, we enumerate all pairs (i, i′ ) with
i′ ∈ ZL[(i,p0 )] sorted by their right elements. On the other hand, if the size of ZL[(i,p0 )] is at
least ∆(D)|q|, we perform a BFS starting in (i, p0 ). After termination of this BFS, we output
exactly the pairs (i, i′ ) for which (i′ , pf ) is visited in this BFS and that are not already
contained in ZL[(i,p0 )] . This shows that we correctly enumerate q(D) without duplicates.
Furthermore, since we list again the pairs (i, i′ ) by increasing index i′ (the first ∆(D)|q|
by the sorting provided by ZL[(i,p0 )] , and the possibly further by the sorted array T ), the
enumeration is again sorted by the right elements of the pairs.
It only remains to estimate the delay of the enumeration procedure, and it is sufficient
to do this for each phase i separately. If S[L[(i, q0 )] = 0 then phase i terminates in constant
time, which also means that in case that S[L[(i, q0 )] = 0 for all phases i, we need at most
time in O(|VD |) to determine that the enumeration has terminated. If the size of ZL[(i,p0 )] is
less than ∆(D)|q|, then we enumerate all pairs with constant delay. If, on the other hand,
the size of ZL[(i,p0 )] is ∆(D)|q|, then this array contains enough solutions to pay for the BFS
we run for (i, p0 ), in the sense that producing the output form ZL[(i,p0 )] with delay |VD |
yields time |VD | ∆(D)|q| = |ED ||q| which is enough to completely run the BFS. The possibly
further solutions stored in T are then produced with delay at most |VD | (for running through
this list to find the non-zero entries).
Obviously, in the worst case we can have ∆(D) = Ω(|VD |) and then the preprocessing
of the algorithm of Theorem 7.1 is Ω(|VD || D |) (in data complexity) and therefore no
improvement over just computing the complete set q(D) in time O(|VD || D |). However, for
graph databases with low average degree, we can decrease the delay significantly from O(| D |)
to O(|VD |) at the cost of a slightly super-linear preprocessing time. As another remark

<!-- page 26 -->

15:26

K. Casel and M. L. Schmid

Vol. 19:4

about Theorem 7.1, we observe that the pre-computed information becomes worthless if D
is updated.
Finally, we discuss a minor modification of the algorithm of Theorem 7.1, which yields a
slightly different result that is interesting in its own right (we state this result as a corollary
after the remark).
Remark 7.2. At the cost of additional space (in O(|VD |2 )) and giving up on sorted
enumeration, we can use the lazy array initialisation technique (see, e. g., the textbook [MS91])
in order to reduce the preprocessing time in Theorem 7.1 by a factor log(∆(D)|q|) to
O(|q|2 ∆(D)| D |). This can be done by replacing each AVL-tree Aj in step 4 by an array
Aj (with lazy initialization) of size |VD |, and completely dropping step 6. Instead of the
bj of the elements stored
arrays Zj , we use in the enumeration phase the unsorted lists A
in the arrays Aj . In step 4 and 5, we simply stop filling the arrays Aj as soon as their
counter exceeds ∆(D)|q|. This reduces the running time for these steps to O(|VD |), and
O(|G⊠ (D, q)| ∆(D)|q|), respectively. Note that the lazy initialization allows us to avoid
duplicates in constant time (which before was achieved by looking up the elements in the
AVL-trees which required time O(log(∆(D)|q|))). In every phase i of the enumeration
procedure instead of a sorted list ZL[(i,p0 )] with the (at most) ∆(D)|q| smallest nodes i′
bL[(i,p )] of at most ∆(D)|q| nodes i′ (not
with (i, i′ ) ∈ q(D) we only have an unsorted list A
0
′
necessarily the smallest ones) with (i, i ) ∈ q(D). In the case where less than ∆(D)|q|
bL[(i,p )] , we can just produce all (i, i′ ) for all i′ stored in A
bL[(i,p )] .
elements are stored in A
0
0
bL[(i,p )] stores more elements, then again we use the elements stored
On the other hand, if A
0
bL[(i,p )] in order to pay for a BFS and store in the array T the new elements found by
in A
0
the BFS. However, since we do not produce the elements in a sorted way, we use T to also
track which elements have been produced already. Whenever we output (i, i′ ) with i′ from
bL[(i,p )] during the run of the BFS for i, we set T [i′ ] = 2 (and never overwrite a 2-entry
A
0
in T ). In order to list the pairs produced by the BFS after its termination, we pass once
through T and only produce (i, i′ ) if T [i′ ] = 1. As last technical detail, we change step 7 to
initialise T as integer array (so we can actually have 2-entries in T ).
Corollary 7.3. RPQ-Enum can be solved with O(|q|2 ∆(D)| D |) preprocessing and O(|VD |)
delay.

7.2. Enumeration of Representative Subsets. Evaluating an RPQ q on a graph database
D aims to find for each node u all its q-successors, i. e., nodes reachable by a q-path. It is
therefore a natural restriction to ask for only at least one (if any) such successor. Likewise,
we could also ask for at least one (if any) q-predecessor for every node. More precisely,
instead of the whole set q(D), the task is to enumerate a q(D)-approximation, which is a
set A ⊆ q(D) such that, for every u, v ∈ VD , if (u, v) ∈ q(D), then also (u, v ′ ), (u′ , v) ∈ A for
some u′ , v ′ ∈ VD . Such a set is representative for q(D), since it contains for every node the
information, whether it is involved as a source and whether it is involved as a target in some
reachable pair from q(D). The problem of enumerating any q(D)-approximation for given D
and q will be denoted by App-RPQ-Enum.
Lemma 7.4. Given a graph database D and an RPQ q, we can compute a q(D)-approximation
in time O(| D ||q|).

<!-- page 27 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:27

Proof. Let D be a graph database over Σ, let q be an RPQ over Σ and let ⪯ be the order on
VD . We assume that VD = [n] with 1 ⪯ 2 ⪯ . . . ⪯ n (see Lemma 2.5).
We compute G⊠ (D, q), which, according to Lemma 3.2, can be done in time O(| D ||q|),
and we interpret G⊠ (D, q) as its underlying non-labelled graph. We construct two arrays S
and T of size |VD | such that, for every i ∈ [n], S[i] = (i, p0 ) and T [i] = (i, pf ). Note that
this also means that pointers to the corresponding adjacency lists are stored along with the
nodes in S and T . Computing S and T can be done in time O(|VD ||q|) as follows. We move
through the list that represents V⊠ (D, q) and for every node (i, p0 ), we set S[i] = (i, p0 ), and
for every node (i, pf ), we set T [i] = (i, pf ).
We modify G⊠ (D, q) by adding a new node vsource with an arc to each (i, p0 ) with
i ∈ [n], and a new node vsink with an arc from each (i, pf ) with i ∈ [n]. This can be done in
time O(|VD |) as follows. We first add vsource and vsink to the list that represents V⊠ (D, q),
which requires constant time. Then we add all vertices from S to the adjacency list for
vsource , which requires time O(|VD |). Finally, for every node (i, pf ) in T , we add vsink to the
adjacency list for (i, pf ). Again, this can be done in time O(|VD |).
Let S ′ and T ′ be arrays of size |V⊠ (D, q)| the entries of which can store values from
[n] ∪ {0} and can be addressed by the nodes from V⊠ (D, q). We wish to fill these arrays such
that they satisfy the following properties. For every (u, p) ∈ V⊠ (D, q), if (u, p) is reachable
from some node from S, then S ′ [(u, p)] = i for some i ∈ [n] such that (u, p) is reachable from
(i, p0 ), and if (u, p) is not reachable from any node from S, then S ′ [(u, p)] = 0. Analogously,
for every (u, p) ∈ V⊠ (D, q), if (u, p) can reach some node from T , then T ′ [(u, p)] = i for some
i ∈ [n] such that (u, p) can reach (i, pf ), and if (u, p) cannot reach any node from T , then
T ′ [(u, p)] = 0.
This means that S ′ and T ′ contain all the information we need to construct a q(D)approximation. More precisely, let
A = {(i, T ′ [(i, p0 )]) | i ∈ [n], T ′ [(i, p0 )] ̸= 0} ∪ {(S ′ [(i, pf )], i) | i ∈ [n], S ′ [(i, pf )] ̸= 0} .
It can be easily seen that A ⊆ q(D) and, for every u, v ∈ VD ,
(u, v) ∈ q(D) =⇒ (∃v ′ ∈ VD : (u, v ′ ) ∈ A) ∧ (∃u′ ∈ VD : (u′ , v) ∈ A) .
Thus, A is a q(D)-approximation. Moreover, provided that we have the arrays S ′ and T ′ at
our disposal, we can compute A in time O(|VD |). Consequently, in order to conclude the
proof, it remains to explain how the arrays S ′ and T ′ can be computed in time O(| D ||q|).
We assume that S ′ and T ′ are initialised with every entry storing 0. For every i ∈ [n],
we set S ′ [(i, p0 )] = i and T ′ [(i, pf )] = i. This can be done in time O(|V⊠ (D, q)|) = O(|VD ||q|).
Next, we perform a BFS from vsource and whenever we traverse an arc ((u, p), (u′ , p′ )), we set
S ′ [(u′ , p′ )] = S ′ [(u, p)]. This can be done in time O(|G⊠ (D, q)|) = O(| D ||q|). Interpreting
the initialisation that ensures S ′ [(i, p0 )] = i, for every i ∈ [n], as the base of an induction and
the assignments S ′ [(u′ , p′ )] = S ′ [(u, p)] for traversed arcs ((u, p), (u′ , p′ )) as the step of the
induction, we can directly conclude by induction that after termination of this BFS, S ′ has
the desired property. In order to compute T ′ , we first have to construct (G⊠ (D, q))R , which,
according to Lemma 2.2, can be done in time O(|G⊠ (D, q)|) = O(| D ||q|). Then we perform a
BFS from vsink and whenever we traverse an arc ((u, p), (u′ , p′ )), we set T ′ [(u′ , p′ )] = T ′ [(u, p)].
Hence, computing S ′ and T ′ can be done in time O(|G⊠ (D, q)|) = O(| D ||q|).
Lemma 7.4 directly implies the following result.
Theorem 7.5. App-RPQ-Enum can be solved with O(| D ||q|) preprocessing and delay
O(1).

<!-- page 28 -->

15:28

K. Casel and M. L. Schmid

Vol. 19:4

Proof. Let D be a graph database over Σ and let q be an RPQ over Σ. In the preprocessing,
we can compute a q(D)-approximation A, which, according to Lemma 7.4, can be done in
time O(| D ||q|). In the enumeration, we simply enumerate the set A with constant delay.
Interestingly, it seems rather difficult to also support updates while keeping a low delay
(the following bounds are due to the same reductions used for Theorem 6.3, simply because
these reductions produce instances for which all q(D)-approximations are equal to q(D)).
Theorem 7.6. If App-RPQ-Enum can be solved with
(1) arbitrary preprocessing, O(|VD |1−ϵ f (|q|)) updates and O(|VD |1−ϵ f (|q|)) delay, then the
OMv-hypothesis fails.
(2) O(|VD |3−ϵ f (|q|)) preprocessing, O(|VD |2−ϵ f (|q|)) updates and O(|VD |2−ϵ f (|q|)) delay for
some function f and ϵ > 0, then the com-BMM-hypothesis fails.
Proof. We prove the two lower bounds of Theorem 7.6 separately.
(1) We observe that exactly the same reduction used in the proof of Point 1 of Theorem 6.3
works as well. Since we assume the same bound on the time required for updates,
we can construct the graph database DM,⃗v1 in the same way as in the proof of Point
1 of Theorem 6.3. Recall that q(DM,⃗v1 ) = {(ui , w) | (M⃗v 1 )[i] = 1}. However, since
for every i ∈ [n], either no node is reachable by a path labelled with 11 or only the
node w is reachable by a path labelled with 11, we know that A ⊆ q(D) is a q(DM,⃗v1 )approximation if and only if A = q(DM,⃗v1 ). Thus, we can compute M⃗v 1 by enumerating
a q(DM,⃗v1 )-approximation in time O(ntd ). Repeating this step in the same way as
done in the proof of Point 1 of Theorem 6.3 leads to an algorithm solving OMv in time
O(n3−ϵ ).
(2) We observe that exactly the same reduction used in the proof of Point 2 of Theorem 6.3
works as well. We construct the graph database Dv1 over {a} and the RPQ q = aaaaa
in the same way. Recall that q(Dv1 ) = {(s, t)} if there is a triangle that contains v1 ,
and q(Dv1 ) = ∅ otherwise. This also means that any q(Dv1 )-approximation A is equal
to {(s, t)} if there is a triangle that contains v1 , and A = ∅ otherwise. Thus, the
assumption that App-RPQ-Enum can be solved with preprocessing O(|VD |3−ϵ f (|q|)),
O(|VD |2−ϵ f (|q|)) updates and O(|VD |2−ϵ f (|q|)) delay for some function f and ϵ > 0,
leads to combinatorial subcubic algorithm for Triangle in the same way as in the proof
of Point 2 of Theorem 6.3.

7.3. Restricted RPQs. As our third approach, we show that for restricted RPQ, we can
solve RPQ-Enum with delay much smaller than O(| D |). We first need some definitions.
For any class Q ⊆ RPQ, we denote
W by Enum(Q) the problem RPQ-Enum where the
input RPQ is from Q. Moreover, (Q) is the set of all RPQs of the form (q1 ∨ . . . ∨ qm )
with qi ∈ Q for every i ∈ [m]. An RPQ q over Σ is a basic transitive RPQ (over Σ) if
q = (x1 ∨ . . . ∨ xk )∗ or q = (x1 ∨ . . . ∨ xk )+ , where x1 , . . . , xk ∈ Σ; and q is a short RPQ (over
Σ) if q = (x1 ∨ . . . ∨ xk ) or q = (x1 ∨ . . . ∨ xk )(y1 ∨ . . . ∨ yk′ ), where x1 , . . . , xk , y1 , . . . , yk′ ∈ Σ.
By BT-RPQ and S-RPQ, we denote the class of basic transitive RPQ and the class of short
RPQ, respectively.
W
We show that for the class (S-RPQ ∪ BT-RPQ) (which, e. g., contains RPQs of the form
q = (ab ∨ c∗ ∨ b(c ∨ d) ∨(a ∨ b ∨ d)+ )), semi-sorted RPQ-Enum can be solved with linear
preprocessing and delay O(∆(D)) in data complexity (recall semi-sorted from Section 2).

<!-- page 29 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:29

W
To this end, we first prove a general upper bound for semi-sorted Enum( (Q)) in terms of
the Enum(Q).
Lemma 7.7. Let Q ⊆ RPQ be such that semi-sorted Enum(Q) can
W be solved with preprocessing p(| D |, |q|) and delay d(| D |, |q|), then semi-sorted Enum( (Q)) can be solved with
preprocessing O((|q|p(| D |, |q|)) + |VD |) and delay O(|q|d(| D |, |q|)).
Proof. We assume that semi-sorted
W Enum(Q) can be solved with preprocessing p(| D |, |q|)
and delay d(| D |, |q|). Let q ∈ (Q), i. e., q = (q1 ∨ q2 ∨ . . . ∨ qm ) with qi ∈ Q for every
i ∈ [m]. By assumption, there are enumeration algorithms A1 , A2 , . . . , Am that, for any
input graph database D, perform some preprocessing in time p(| D |, |qi |) and then enumerate
the pairs qi (D) ordered by their first elements with delay d(| D |, |qi |). We describe now an
enumeration algorithm A that, for any input graph database D, performs some preprocessing
in time O(|q|p(| D |, |q|)) and then enumerates theSpairs in q(D) ordered by their first elements
with delay O(|q|d(| D |, |q|)). Note that q(D) = m
i=1 qi (D).
Let D be the input graph database over Σ. We assume that VD = [n] with 1 ⪯ 2 ⪯ . . . ⪯ n
(see Lemma 2.5).
Intuitive Explanation: The algorithm A uses size-m arrays L and R that store nodes
from VD = [n], and size-n Boolean arrays X and Y that represent subsets of [n]. We
construct these in the preprocessing, in addition to performing the preprocessing procedures
of the algorithms A1 , A2 , . . . , Am . Since the enumeration algorithms A1 , A2 , . . . , Am provide
a semi-ordered enumeration, each enumeration of qj (D) can be seen as a table with two
columns whose rows correspond to the pairs from qj (D), ordered by the first column. The
enumeration then produces the rows of this table from top to bottom. Therefore, the
enumeration proceeds in n phases, where the ith phase consists in producing all rows with
i in the left column (note that for some i such rows might not exist). Consequently, we
can also interpret phase i of the algorithms Aj as an enumeration of only single elements
from VD , since the left elements of the pairs are always i. The enumeration procedure
of A also proceeds in n phases, where the ith phase consists in having the enumeration
algorithms A1 , A2 , . . . , Am perform their ith phases in an interleaved manner, i. e., we let
each Ai produce the next element one after the other, and then we repeat this step until
all Aj have finished their ith phase. We maintain in array L the current phases (i. e., the
current left element of the produced pairs) of the algorithms Aj (this is necessary, since
only a subset may participate in phase i for a fixed i and some Aj may finish it earlier than
others) and we store in array R the elements produced most recently by the algorithms
Aj . Once all Aj (that are still in phase i) have produced one new element, we disregard
all those elements v among them such that A has already produced (i, v) in phase i (these
already produced elements will be stored in Y ), and we store all other elements in X. Then
A picks some element v ∈ X, produces (i, v) (and therefore “buys” another delay), marks v
as already produced by adding it to Y and removes it from X. Then, again each Aj will
produce the next element and this goes on until all Aj have finished phase i, and therefore
we can move on to the next phase.
Due to the sets X and Y , we do not produce duplicates. However, in order to bound
the delay, we have to produce at least one pair in each iteration of the main loop in each
phase, and therefore we have to show that it cannot happen that X is empty when A needs
to produce the next pair. We shall now define this algorithm more formally.

<!-- page 30 -->

15:30

K. Casel and M. L. Schmid

Vol. 19:4

Preprocessing: First, A performs all preprocessing procedures of the algorithms
A1 , A2 , . . . , Am ,
P
which can be done in time m
i=1 p(| D |, |qi |) = O(|q|p(| D |, |q|)). Then we construct arrays
L and R each of size m the entries of which can store values from [n] ∪ {0, n + 1}, and we
initialise all entries with 0. Furthermore, we construct Boolean arrays X and Y of size n
that initially store 0 in every entry (note that these arrays shall be used for storing subsets
of [n]). This can clearly be done in time O(|VD | + |q|). Consequently, the total preprocessing
time is O(|q|p(| D |, |q|) + |VD |).
Enumeration: Firstly, we set c = 0, where c will be a counter that indicates the current
phase. Then we iterate the following main loop until it terminates:
(1) For every i ∈ [m] with L[i] = c, we request the next element from the enumeration
procedure of Ai . If such an element (u, v) is returned, we set L[i] = u and R[i] = v, and
if no element is returned (i. e., the enumeration of Ai is done), we set L[i] = R[i] = n + 1.
(2) If c < min{L[i] | i ∈ [m]}, then
(a) for every v ∈ X, remove v from X and produce (c, v),
(b) set Y = ∅,
(c) set c = min{L[i] | i ∈ [m]}.
(3) If c = n + 1, interrupt.
(4) For every i ∈ [m], if L[i] = c and R[i] ∈
/ Y , add R[i] to X.
(5) Choose some element v ∈ X, produce (c, v) as output, remove v from X and add v to Y .
(6) Move to Step 1.
Correctness: We first note that in the first iteration of the main loop, we have L[i] = c = 0
for every i ∈ [m], so we request the first pairs from all Aj and store the respective left and right
elements in L and R, respectively (or store n+1 in both L and R, if the enumeration procedure
has already terminated without producing any pair). In any subsequent iteration, this only
happens with respect to those Aj that are in phase c. If condition c < min{L[i] | i ∈ [m]}
is satisfied in Step 2, then this means that by performing Step 1, we have reached a new
phase (note that also in the very first iteration, we reach a new phase, i. e., phase i for the
smallest left element i ∈ [n] in any pair produced by some Aj ) and therefore we produce
all elements of X as output (paired with c) and then empty the sets X and Y (which were
only storing elements that are relevant for phase c that just terminated). Moreover, we have
to determine the next phase, which is done by setting c = min{L[i] | i ∈ [m]}. In the very
first iteration, X is empty, so nothing happens in these steps except setting c to the current
phase (which must be strictly larger than 0, but not necessarily 1). Next, Step 3 would now
interrupt the whole procedure, if c = n + 1, which means that all enumeration procedures
of the algorithms Aj have already terminated. Note that if this happens for the first time,
i. e., the last still active enumeration procedures of some Aj terminate by requesting their
last elements in Step 1, then we necessarily also have c < min{L[i] | i ∈ [m]} in Step 2,
which means that we have produced all elements from X as output before the interruption
is invoked in Step 3. If, on the other hand, we reach Step 4, then we collect in the set X the
right elements from all new pairs produced by Aj that are in phase c, but only if these are
not already stored in Y , since then they would have already been produced as right element
in a pair with c as left element. Step 5 then chooses and actually produces (paired with c)
one of the elements from X (and removes it from X and adds it to Y to store that is has

<!-- page 31 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:31

already been produced). Finally, in Step 6, we move back to Step 1, which triggers a new
iteration.
These considerations show that the procedure from above will enumerate q(D). In
particular, note that if we enter Step 5 with X being empty, then we do not produce
an output at this point, but, due to Step 2, we will nevertheless completely enumerate
q(D). Since we synchronise the enumerations of the Aj with respect to their phases, the
enumeration of q(D) produces all pairs from q(D) with left element i, then all pairs with
left element i′ > i and so on; thus, the enumeration of q(D) is semi-ordered. Furthermore,
the book keeping done in sets X and Y guarantees that we do not produce duplicates. It
only remains to analyse the delay of this enumeration procedure.
We first observe that Step 2b, i. e., setting Y = ∅, is problematic since it requires time
O(n) (we have to set Y [i] = 0 for every i ∈ [n]). Therefore, we implement the array Y
as follows. Instead of letting it be Boolean, we assume that it can store elements from
[n] ∪ {0}. The idea is that Y [i] = 0 means that i ∈
/ Y (just as for the Boolean case), while
Y [i] = j with j ∈ [n] means i ∈ Y in the case that we are currently in phase j, i. e., c = j,
and i ∈
/ Y otherwise. With this interpretation, Step 2b is not necessary anymore, since
setting c = min{L[i] | i ∈ [m]} in Step 2c has the same effect as erasing all elements from Y .
Consequently, we can ignore Step 2b altogether (or rather interpret as a mere comment in
the pseudo code above to indicate what is happening at Step 2b). In particular, we note that
with this implementation of Y , we can still check for both X and Y whether they contain a
specific element, and we can add or erase specific elements in constant time (adding i to Y
in phase c just means to set Y [i] = c instead of Y [i] = 1).
However, in Steps 2a and 5, we also have to retrieve some element from X. In order
to do this efficiently (and not by moving through the array from left to right to find some
elements, which requires time O(n)), we also store the elements of X as an unsorted list
(which is initialised in the preprocessing). This means that we can always obtain some
element of X in constant time (by just retrieving the first list element). Keeping the array
and the list for X synchronised is no problem: Whenever we add some i to X (Step 4), we
set X[i] = 1 and add x at the end of the list for X; whenever we want to retrieve some
element from X (Steps 2a and 5), we retrieve and remove the first list element, say i, and
then we set X[i] = 0. Consequently, all operations with respect to the lists X and Y can be
performed in constant time.
We estimate
P the running-time for each of the separate steps of an iteration. Step 1 requires time O( m
j=1 d(| D |, |qj |)) = O(md(| D |, |q|)). The total running time of Steps 2 to 2c
is O(m + k), where k is the number of pairs (c, v) produced in Step 2a and O(m) is needed
to compute min{L[i] | i ∈ [m]}. Note that the pairs that are produced in Step 2 are output
with constant delay in Step 2a and pay for the running time dependence on k, hence in the
worst case, k = 0 where Step 2 requires time in O(m). Step 4 requires time O(m). All other
steps can be carried out in constant time. This means that if in each iteration at least one
pair is produced by Step 6, then the delay of the whole enumeration procedure of algorithm
A is O(md(| D |, |q|)). Obviously, if we can never reach the situation that X = ∅ in Step 5,
then in each iteration at least one pair is produced. Therefore, it is sufficient to prove this
property.
Let us assume that the enumeration procedure has just finished Step 3 and we are
in some iteration of phase i, i. e., c = i. Moreover, we assume that so far, we have not
encountered the situation that X = ∅ in Step 5. For every j ∈ [m], let aj = R[j] if L[j] = c
where aj = ⊥ indicates the situation that element aj does not exist. This means that all

<!-- page 32 -->

15:32

K. Casel and M. L. Schmid

Vol. 19:4

existing elements aj with j ∈ [m] are exactly those elements that have most recently been
produced in phases i of the enumeration procedures from the algorithms Aj (this can have
happened in Step 1 of the same iteration or, if this is the first iteration of phase i, also in
applications of Step 1 in previous iterations). In particular, the existing elements aj with
j ∈ [m] have not yet been handeled in the sense of Step 4, i. e., we have not yet checked
whether they have already been produced as output and, if not, have added them to X.
We can also note that there must be at least one j ∈ [m] with aj ̸= ⊥, since otherwise
c < min{L[i] | i ∈ [m]}.
In addition to these elements aj , for every j ∈ [m], let bj,1 , bj,2 , . . . , bj,ℓj (note that
ℓj = 0 is possible) be exactly the elements already produced in phase i of the enumeration
procedure of Aj in some previous iterations. In other words, for every j ∈ [m], we have
requested in applications of Step 1 exactly the elements bj,1 , bj,2 , . . . , bj,ℓj , aj from phase
i of the enumeration procedure of Aj (note that aj = ⊥ is possible, which means that
aj does not exist and therefore has not been requested). Moreover, this has happened in
ℓ = max{ℓj | j ∈ [m]} previous (i. e., not counting the current one) iterations of the main
loop of the enumeration procedure of algorithm A. In particular, this means that we have in
phase i so far only produced ℓ pairs with i as left element.
Let K = {bj,p | j ∈ [m], p ∈ [ℓj ]} and let M = {aj | j ∈ [m]}. Since ℓ = max{ℓj | j ∈
[m]}, there is at least one j ′ ∈ [m] with ℓj ′ = ℓ and therefore |{bj ′ ,1 , bj ′ ,2 , . . . , bj ′ ,ℓj ′ }| = ℓ.
This is true since the elements bj ′ ,1 , bj ′ ,2 , . . . , bj ′ ,ℓj ′ are part of the ith phase of the enumeration
of Aj ′ and therefore must be distinct. Thus, |K| ≥ ℓ. Furthermore, we can choose j ′ such
that aj ′ ̸= ⊥. This is the case since we have aj = ⊥ if and only if ℓj < ℓ (i. e., phase i of the
enumeration of Aj has already terminated) and, as observed above, there must be at least
one j ∈ [m] with aj ̸= ⊥.
For every b ∈ K, either (i, b) has been produced as output, or b ∈ X. Since so far we
have only produced ℓ pairs as output, this directly implies that if |K| > ℓ, then X ̸= ∅,
which means that we reach Step 5 with X ̸= ∅. If, on the other hand, |K| = ℓ, then
K = {bj ′ ,1 , bj ′ ,2 , . . . , bj ′ ,ℓj ′ }, which also means that aj ′ ∈
/ K since bj ′ ,1 , bj ′ ,2 , . . . , bj ′ ,ℓj ′ , aj ′ is
an enumeration of distinct elements provided by algorithm Aj ′ (recall that aj ′ ̸= ⊥, as
observed above). Consequently, we also have aj ′ ∈
/ Y and therefore aj ′ is added to X in
Step 4. Hence, we reach Step 5 with X ̸= ∅.
Now, we give upper bounds for Enum(BT-RPQ) and Enum(S-RPQ) separately.
Theorem 7.8. Semi-sorted Enum(BT-RPQ) can be solved with delay O(∆(D)) (and without
preprocessing).
Proof. Let D be a graph database, let q = (x1 ∨ x2 ∨ . . . ∨ xk )∗ and q ′ = (x1 ∨ x2 ∨ . . . ∨ xk )+ ,
where x1 , x2 , . . . , xk ∈ Σ. It can be easily seen that enumerating q(D) or q ′ (D) is the same as
enumerating the reflexive-transitive closure (ED′ )∗ or the transitive closure (ED′ )+ , where D′
is obtained from D by deleting all x-adjacency lists with x ∈
/ {x1 , x2 , . . . , xk }. In [CFNS20],
it is shown for general directed graphs G = (V, E) how to enumerate E ∗ and E + sorted by
first coordinate (denoted by “row-wise”) with delay O(∆(G)). This approach translates to a
semi-sorted enumeration and can be used on D in such a way that all x-adjacency lists with
x∈
/ {x1 , x2 , . . . , xk } are ignored (so without preprocessing). Thus, by using this procedure
on D, we can enumerate q(D) and q ′ (D) semi-sorted with delay O(∆(D)).
Theorem 7.9. Semi-sorted Enum(S-RPQ) can be solved with preprocessing O(| D |) and
delay O(∆(D)).

<!-- page 33 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:33

Proof. Let D be a graph database over Σ. We assume that VD = [n] with 1 ⪯ 2 ⪯ . . . ⪯ n
(see Lemma 2.5).
Let q = (x1 ∨ x2 ∨ . . . ∨ xk ) with x1 , x2 , . . . , xk ∈ Σ. Then q(D) = {(u, v) | (u, xi , v) ∈
ED , 1 ≤ i ≤ k}. Obviously, we can in time O(| D |) compute this whole set by moving trough
the list for VD and for every encountered node u and every i ∈ [k], we access the xi -adjacency
list for u and for all v it contains, we add (u, v) to a list. This list contains exactly the
elements from q(D) and is also semi-sorted. Thus, we can compute q(D) completely in the
preprocessing and therefore enumerate it semi-sorted with constant delay.
Next, we assume that
q = (x1 ∨ x2 ∨ . . . ∨ xk )(y1 ∨ y2 ∨ . . . ∨ yk′ )
with x1 , . . . , xk , y1 , . . . , yk′ ∈ Σ, for which the algorithm is more difficult.
Preprocessing: First, we construct G⊠ (D, q) in time O(| D ||q|) = O(| D |). Since Gq ,
the underlying Σ-graph of the NFA for q, can be assumed to have nodes {1, 2, 3}, where
1 is the initial and 3 the accepting state, and arcs {(1, xi , 2) | 1 ≤ i ≤ k} ∪ {(2, yi , 3) |
1 ≤ i ≤ k ′ }, we can assume that the underlying non-labelled graph of G⊠ (D, q) has the
following simple structure: V⊠ (D, q) = V1 ∪ V2 ∪ V3 with Vi = {(u, i) | u ∈ VD } for every
i ∈ [3], and E⊠ (D, q) = E1 ∪ E2 with E1 = {((u, 1), (v, 2)) | ∃i ∈ [k] : (u, xi , v) ∈ ED } and
E2 = {((u, 2), (v, 3)) | ∃i ∈ [k ′ ] : (u, yi , v) ∈ ED }. The task of enumerating q(D) reduces now
to the task of enumerating all reachable pairs ((u, 1), (v, 3)) or, equivalently, to enumerate
(E⊠ (D, q))∗ ∩(V1 ×V3 ). We further assume that the sets V1 , V2 and V3 are stored in individual
lists. This can be easily achieved during the construction of G⊠ (D, q): for every u ∈ VD we
add (u, 1) to a list that stores V1 , then we construct lists for V2 and V3 analogously.
Finally, we modify G⊠ (D, q) as follows. We construct a Boolean array Sin of size |VD |
(that can be addressed by the elements from VD ) initialised with 0 in all entries. Then,
we move through the list for V1 and for every (u, 1) we encounter, we move through its
adjacency list and for every (v, 2) we encounter in this adjacency list, we set Sin [v] = 1. This
procedure can be carried out in O(| D |) and after its termination, we have that Sin [v] = 0
if and only if the in-degree of (v, 2) is 0. Next, we construct a Boolean array Sout of size
|VD | (that can be addressed by the elements from VD ) initialised with 0 in all entries. Then,
we move through the list for V2 and for every (u, 2) we encounter that has a non-empty
adjacency list, we set Sout [u] = 1. This procedure can be carried out in O(|VD |) and after its
termination, we have that Sout [u] = 0 if and only if the out-degree of (u, 2) is 0. We move
again through the list for V2 and we remove every encountered node (v, 2) if Sin [v] = 0 or
Sout [v] = 0; moreover, we move through the list for V1 and for every (u, 1) we encounter, we
move through its adjacency list and we remove every (v, 2) that we encounter if Sin [v] = 0 or
Sout [v] = 0. These two steps can again be carried out in time O(| D |). We have now removed
all nodes from V2 with in-degree or out-degree 0, and all arcs adjacent to such nodes. Next,
we remove all isolated nodes from V1 and V3 . This can be done by first computing all nodes
from V1 with out-degree 0 and all nodes from V3 with in-degree 0, which can be done in the
same way as we did for nodes from V2 in time O(| D |).
Enumeration: Let Q be an empty queue. For every i = 1, 2, . . . , n, we start a BFS from
(i, 1). Such a BFS is carried out until all nodes from the neighbourhood of (i, 1) have
been visited; thus, the BFS proceeds by visiting all the neighbourhoods N ((j, 2)) for every
(j, 2) ∈ N ((i, 1)). From now on, we add (i, k) to Q for every node (k, 3) that we visit for the
first time. Whenever ∆(D) steps are made by the BFS after the last output while visiting

<!-- page 34 -->

15:34

K. Casel and M. L. Schmid

Vol. 19:4

a neighbourhood N ((j, 2)), we produce after visiting this whole neighbourhood the first
element from Q as output and remove this element from Q. In order to do this efficiently, we
store the elements of Q not just as a list but also maintain two lists of size [n] over {1, . . . , n}
storing the elements currently in and the elements already produced and deleted from Q,
respectively. When the BFS terminates, i. e., all neighbourhoods of the nodes from N ((i, 1))
have been visited, then we produce the remaining elements in Q (and thus emptying it) and
proceed with the BFS for the next node from V1 .
Correctness: The enumeration procedure goes through n phases, where in each such phase
we perform a BFS in some vertex u ∈ V1 and produce only pairs of the form (i, j), where
u = (i, 1). This means it is sufficient to show for each phase separately that we will produce
all pairs of the form (i, j) (where u = (i, 1) ∈ V1 is the BFS-start-node of this phase) without
duplicates and with the desired delay. Moreover, note that the enumeration is obviously
semi-sorted. Further, if Q is never empty when we request the next element, the delay is
obviously in O(∆(D)) since visiting one neighbourhood N ((j, 2)) requires at most ∆(D)
steps.
Let u ∈ V1 and consider the phase of the enumeration procedure that performs the BFS
in u. The BFS visits all nodes of the neighbourhood N (u) = {v1 , v2 , . . . , vh } and marks
them as visited, which can be done in time O(h) = O(∆(D)). Then it visits the complete
neighbourhood N (v1 ), then the complete neighbourhood N (v2 ) and so on until all nodes
Sh
Pi
i=1 N (vi ) are visited. For each i ∈ [h], the BFS performs ki =
j=1 |N (vj )| steps to
visit all neighbourhoods N (v1 ), . . . , N (vi ), while producing at least qi = max{|N (vj )| | 1 ≤
j ≤ i} different elements for Q. To show that Q is never empty, it suffices to show that
Pi
Pri
j=1 kj ≤ qi ∆(D) for all i; note that we request an element from Q exactly after
j=1 kj
Pri+1 +1
steps for some r1 , . . . , rx ∈ [h] where 1 = r1 < r2 < . . . , rx and j=ri kj ≥ ∆(D) for
each i ∈ [x]. Since the degree of u is at most ∆(D), we know that h ≤ ∆(D) which means
Pi
j=1 kj ≤ i max{|N (vj )| | 1 ≤ j ≤ i} ≤ ∆(D)qi .
Finally, by using Lemma 7.7, we can plug
W together Theorems 7.8 and 7.9 in order to
obtain the following upper bound for Enum( (S-RPQ ∪ BT-RPQ)).
W
Theorem 7.10. Semi-sorted Enum( (S-RPQ ∪ BT-RPQ)) can be solved with preprocessing
O(|q|2 | D |) and delay O(|q|2 ∆(D)).
Proof. Due to Theorems 7.8 and 7.9, both semi-sorted Enum(S-RPQ) and semi-sorted
Enum(BT-RPQ) can be solved with preprocessing O(|q|| D |) and delay O(∆(D)). This obviously also means that semi-sorted Enum(S-RPQ∪BT-RPQ) can be solved with preprocessing
W
O(|q|| D |) and delay O(∆(D)). Lemma 7.7 now implies that semi-sorted Enum( (S-RPQ ∪
BT-RPQ)) can be solved with preprocessing O(|q|2 | D |) and delay O(|q|2 ∆(D)).
Since q = aa ∈ S-RPQ is sufficient to express BMM as RPQ-evaluation, Theorem 7.10
implies that enumerating (the 1-entries of) Boolean matrix products can be solved with
linear preprocessing and O(n) delay, but, on the other hand, this also immediately implies a
matching data complexity lower bound for the upper bound of Theorem 7.10.
Theorem 7.11. If RPQ-Enum(S-RPQ) can be solved with prep. O(|VD |3−ϵ f (|q|)) and delay
O(| ∆(D)|1−ϵ f (|q|)) for some function f and ϵ > 0, then the com-BMM-hypothesis fails.
Proof. If RPQ-Enum(S-RPQ) can be solved with preprocessing O(|VD |3−ϵ f (|q|)) and with
delay O(| ∆(D)|1−ϵ f (|q|)) for some function f and ϵ > 0, then we can also compute q(D)

<!-- page 35 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:35

in total time O((|VD |3−ϵ f (|q|)) + (|q(D)|| ∆(D)|1−ϵ f (|q|))). For the reduction of Lemma 5.9
this implies that we can solve com-BMM in time O(n3−ϵ + (n2 n1−ϵ )) = O(n3−ϵ ).
W
Compared to the full class of RPQs, the class (S-RPQ ∪ BT-RPQ) is quite restricted.
However, comprehensive experimental analyses of query logs suggest that quite restricted
RPQs are still practically relevant: in the corpus of more than 50 million RPQs analysed
in [BMT19, Table 4], roughly 50% of the RPQs are from BT-RPQ and another 25% are of
the form q = x1 x2 . . . xk . Many of these RPQs of the form q = x1 x2 . . . xk also satisfy k ≤ 2,
which means that they are S-RPQs [Mar].
Conclusions
In this work, we thoroughly investigated the fine-grained complexity of evaluating regular
path queries. We focussed on what can be considered the most simple case of RPQs, i. e.,
the solution set contains only node pairs (and no witness paths) that are connected by
arbitrary paths (in contrast to simple path semantics, trail semantics etc.). As explained in
the introduction, more powerful RPQs become computationally intractable, so this simple
setting is suitable for a fine-grained complexity analysis. On the other hand, it can still
be considered as a core functionality to be found in typical graph query languages. We
considered the evaluation problems typically investigated in database theory (see Table 3):
the basic decision problems of Boolean evaluation and testing, and the function problems of
computing and counting the solution set, and finally the enumeration variant.
For the non-enumeration variants, we were able to complement the upper bounds
obtained by the product-graph approach by conditional lower bounds (see Table 1). Hence,
an algorithmic approach leading to strictly better upper complexity bounds than the productgraph approach seems to require unlikely improvements with respect to computing orthogonal
vectors and Boolean matrices.
Our picture is much less complete for the enumeration variant of RPQ-evaluation (see
Table 2). Although we produce many individual conditional lower bounds (which rule out
many algorithms), it is still open whether enumeration with O(| D |) preprocessing and
O(|VD |) delay (or any delay truly sublinear in the size of the graph database) is possible (in
data complexity); see also Question 6.4. Since finding an enumeration algorithm that achieves
a sublinear delay is definitely a worthwhile research task (or, conversely, an enumeration
algorithm with a delay bound as large as the whole database seems disappointing), we
investigated several approaches to find enumeration algorithms with a sublinear delay. Our
first such approach is to drop the restriction of linear preprocessing and it turns out that if
we allow an additional factor of ∆(D) in the preprocessing (or log(∆(D)) ∆(D) if we do not
want to use lazy-initialisation), then a delay of O(|VD |) is indeed possible (see Theorem 7.1
and Corollary 7.3).3 Our second approach shows that even linear preprocessing and constant
delay is possible, if we are satisfied with enumerating just a representative subset of the
solution set (see Theorem 7.5). Here, representative means that if u can reach any node
v with a correctly labelled path, then at least one such witness pair exists in the set (and
likewise if u can be reached by any node v with a correctly labelled path). While this variant
is quite restricted in comparison to the full evaluation task, it might be a worthwhile query
result to start with (e. g., before running a complete enumeration algorithm without constant
delay, we could first enumerate all these witnesses in constant delay and see whether this
3Recall that ∆(D) is the average degree of D.

<!-- page 36 -->

15:36

K. Casel and M. L. Schmid

Vol. 19:4

information is already enough). Finally, in our third approach, we identify a class of RPQs
that can be enumerated with linear preprocessing and delay O(∆(D)) (see Theorem 7.10).4
This result points out that the simplicity of the regular expression of the query might be
exploited to achieve a better delay. This is particularly interesting given the fact that very
simple RPQs are already sufficient for conditional lower bound reductions with respect to
data complexity. Moreover, empirical work has shown that in practical scenarios where
regular expressions are used as means of querying graph databases it is often the case that
the regular expressions are rather simple. In case that Question 6.4 can be answered in
the negative, and if enumeration algorithms with sublinear delay are of high relevance, we
should concentrate on algorithms that only work for a special and simple class of RPQs.
A possible future research task with respect to the topic of this paper is to answer
Question 6.4. We conjecture that an algorithm that answers the question in the affirmative
will be non-trivial and likely to yield more general algorithmic insights with respect to
querying graphs with regular expressions. If, by a conditional lower bound, it can be
shown that the answer to the question is negative, then the question arises for which RPQs
a sublinear delay is possible and for which it is (conditionally) not. Our Theorem 7.10
constitutes a partial result in this regard.
Acknowledgment
We wish to thank the anonymous reviewers of the conference version [CS21] of this work for
their valuable feedback. In particular, following the reviewer’s comments and suggestions,
we have included more background information and comprehensive explanations of certain
aspects, which substantially improved the overall exposition of this paper. We also thank
the reviewers of this journal version for their thorough reviewing.
References
[AAB+ 17] Renzo Angles, Marcelo Arenas, Pablo Barceló, Aidan Hogan, Juan L. Reutter, and Domagoj
Vrgoc. Foundations of modern query languages for graph databases. ACM Comput. Surv.,
50(5):68:1–68:40, 2017. doi:10.1145/3104031.
[ABW18] Amir Abboud, Arturs Backurs, and Virginia Vassilevska Williams. If the current clique algorithms
are optimal, so is valiant’s parser. SIAM J. Comput., 47(6):2527–2555, 2018. doi:10.1137/
16M1061771.
Rasmus Resen Amossen and Rasmus Pagh. Faster join-projects and sparse matrix multiplications.
[AP09]
In Database Theory - ICDT 2009, 12th International Conference, St. Petersburg, Russia, March
23-25, 2009, Proceedings, pages 121–126, 2009. doi:10.1145/1514894.1514909.
[AW14]
Amir Abboud and Virginia Vassilevska Williams. Popular conjectures imply strong lower bounds
for dynamic problems. In 55th IEEE Annual Symposium on Foundations of Computer Science,
FOCS 2014, Philadelphia, PA, USA, October 18-21, 2014, pages 434–443, 2014. doi:10.1109/
FOCS.2014.53.
[AWY18] Amir Abboud, Virginia Vassilevska Williams, and Huacheng Yu. Matching triangles and basing
hardness on an extremely popular conjecture. SIAM J. Comput., 47(3):1098–1122, 2018. doi:
10.1137/15M1050987.
[Bar13]
Pablo Barceló. Querying graph databases. In Proceedings of the 32nd ACM SIGMOD-SIGACTSIGART Symposium on Principles of Database Systems, PODS 2013, New York, NY, USA June 22 - 27, 2013, pages 175–188, 2013.
[BBG20] Guillaume Bagan, Angela Bonifati, and Benoı̂t Groz. A trichotomy for regular simple path queries
on graphs. J. Comput. Syst. Sci., 108:29–48, 2020. doi:10.1016/j.jcss.2019.08.006.
4Recall that ∆(D) is the degree of D.

<!-- page 37 -->

Vol. 19:4

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:37

[BBMT17] Jean-François Baget, Meghyn Bienvenu, Marie-Laure Mugnier, and Michaël Thomazo. Answering
conjunctive regular path queries over guarded existential rules. In Proceedings of the Twenty-Sixth
International Joint Conference on Artificial Intelligence, IJCAI 2017, Melbourne, Australia,
August 19-25, 2017, pages 793–799, 2017. doi:10.24963/ijcai.2017/110.
[BDG07] Guillaume Bagan, Arnaud Durand, and Etienne Grandjean. On acyclic conjunctive queries and
constant delay enumeration. In Computer Science Logic, 21st International Workshop, CSL
2007, 16th Annual Conference of the EACSL, Lausanne, Switzerland, September 11-15, 2007,
Proceedings, pages 208–222, 2007.
[BFR19]
Pablo Barceló, Diego Figueira, and Miguel Romero. Boundedness of conjunctive regular path
queries. In 46th International Colloquium on Automata, Languages, and Programming, ICALP
2019, July 9-12, 2019, Patras, Greece, pages 104:1–104:15, 2019. doi:10.4230/LIPIcs.ICALP.
2019.104.
[BGS20]
Christoph Berkholz, Fabian Gerhardt, and Nicole Schweikardt. Constant delay enumeration for
conjunctive queries: a tutorial. ACM SIGLOG News, 7(1):4–33, 2020. doi:10.1145/3385634.
3385636.
[BI16]
Arturs Backurs and Piotr Indyk. Which regular expression patterns are hard to match? In IEEE
57th Annual Symposium on Foundations of Computer Science, FOCS 2016, 9-11 October 2016,
Hyatt Regency, New Brunswick, New Jersey, USA, pages 457–466, 2016. doi:10.1109/FOCS.
2016.56.
Christoph Berkholz, Jens Keppeler, and Nicole Schweikardt. Answering conjunctive queries under
[BKS17]
updates. In Proceedings of the 36th ACM SIGMOD-SIGACT-SIGAI Symposium on Principles of
Database Systems, PODS 2017, Chicago, IL, USA, May 14-19, 2017, pages 303–318, 2017.
[BLLW12] Pablo Barceló, Leonid Libkin, Anthony Widjaja Lin, and Peter T. Wood. Expressive languages
for path queries over graph-structured data. ACM Transactions on Database Systems (TODS),
37(4):31:1–31:46, 2012.
[BMT17] Angela Bonifati, Wim Martens, and Thomas Timm. An analytical study of large SPARQL query
logs. PVLDB, 11(2):149–161, 2017.
[BMT19] Angela Bonifati, Wim Martens, and Thomas Timm. Navigating the maze of wikidata query logs.
In The World Wide Web Conference, WWW 2019, San Francisco, CA, USA, May 13-17, 2019,
pages 127–138, 2019. doi:10.1145/3308558.3313472.
[BMT20] Angela Bonifati, Wim Martens, and Thomas Timm. An analytical study of large SPARQL query
logs. VLDB J., 29(2-3):655–679, 2020. doi:10.1007/s00778-019-00558-9.
Meghyn Bienvenu, Magdalena Ortiz, and Mantas Simkus. Regular path queries in lightweight
[BOS15]
description logics: Complexity and algorithms. J. Artif. Intell. Res., 53:315–374, 2015. doi:
10.1613/jair.4577.
[Bra13]
Johann Brault-Baron. De la pertinence de l’énumération : complexité en logiques propositionnelle
et du premier ordre. (The relevance of the list: propositional logic and complexity of the first order).
PhD thesis, University of Caen Normandy, France, 2013. URL: https://tel.archives-ouvertes.
fr/tel-01081392.
[Bri14]
Karl Bringmann. Why walking the dog takes time: Frechet distance has no strongly subquadratic
algorithms unless SETH fails. In 55th IEEE Annual Symposium on Foundations of Computer
Science, FOCS 2014, Philadelphia, PA, USA, October 18-21, 2014, pages 661–670, 2014. doi:
10.1109/FOCS.2014.76.
Karl Bringmann. Fine-grained complexity theory (tutorial). In 36th International Symposium on
[Bri19]
Theoretical Aspects of Computer Science, STACS 2019, March 13-16, 2019, Berlin, Germany,
pages 4:1–4:7, 2019. doi:10.4230/LIPIcs.STACS.2019.4.
[BT16]
Meghyn Bienvenu and Michaël Thomazo. On the complexity of evaluating regular path queries
over linear existential rules. In Web Reasoning and Rule Systems - 10th International Conference,
RR 2016, Aberdeen, UK, September 9-11, 2016, Proceedings, pages 1–17, 2016. doi:10.1007/
978-3-319-45276-0\_1.
[CDLV03] Diego Calvanese, Giuseppe De Giacomo, Maurizio Lenzerini, and Moshe Y. Vardi. Reasoning on
regular path queries. SIGMOD Record, 32(4):83–92, 2003.
[CFNS20] Katrin Casel, Tobias Friedrich, Stefan Neubert, and Markus L. Schmid. Shortest distances as
enumeration problem. CoRR, abs/2005.06827, 2020. URL: https://arxiv.org/abs/2005.06827,
arXiv:2005.06827.

<!-- page 38 -->

15:38

K. Casel and M. L. Schmid

Vol. 19:4

Isabel F. Cruz, Alberto O. Mendelzon, and Peter T. Wood. A graphical query language supporting
recursion. In Proceedings of the Association for Computing Machinery Special Interest Group
on Management of Data 1987 Annual Conference, San Francisco, California, May 27-29, 1987,
pages 323–330, 1987.
[CS21]
Katrin Casel and Markus L. Schmid. Fine-grained complexity of regular path queries. In 24th
International Conference on Database Theory, ICDT 2021, March 23-26, 2021, Nicosia, Cyprus,
pages 19:1–19:20, 2021. doi:10.4230/LIPIcs.ICDT.2021.19.
[EGMT19] Massimo Equi, Roberto Grossi, Veli Mäkinen, and Alexandru I. Tomescu. On the complexity
of string matching for graphs. In 46th International Colloquium on Automata, Languages, and
Programming, ICALP 2019, July 9-12, 2019, Patras, Greece, pages 55:1–55:15, 2019. doi:
10.4230/LIPIcs.ICALP.2019.55.
[EMT21] Massimo Equi, Veli Mäkinen, and Alexandru I. Tomescu. Graphs cannot be indexed in polynomial
time for sub-quadratic time string matching, unless SETH fails. In SOFSEM 2021: Theory and
Practice of Computer Science - 47th International Conference on Current Trends in Theory
and Practice of Computer Science, SOFSEM 2021, Bolzano-Bozen, Italy, January 25-29, 2021,
Proceedings, pages 608–622, 2021. doi:10.1007/978-3-030-67731-2\_44.
[FGK+ 20] Diego Figueira, Adwait Godbole, Shankara Narayanan Krishna, Wim Martens, Matthias Niewerth,
and Tina Trautner. Containment of simple conjunctive regular path queries. In Proceedings of
the 17th International Conference on Principles of Knowledge Representation and Reasoning, KR
2020, Rhodes, Greece, September 12-18, 2020, pages 371–380, 2020. doi:10.24963/kr.2020/38.
[Fig20]
Diego Figueira. Containment of UC2RPQ: the hard and easy cases. In 23rd International
Conference on Database Theory, ICDT 2020, March 30-April 2, 2020, Copenhagen, Denmark,
pages 9:1–9:18, 2020. doi:10.4230/LIPIcs.ICDT.2020.9.
[FS13]
Dominik D. Freydenberger and Nicole Schweikardt. Expressiveness and static analysis of extended
conjunctive regular path queries. J. Comput. Syst. Sci., 79(6):892–909, 2013. doi:10.1016/j.
jcss.2013.01.008.
François Le Gall. Powers of tensors and fast matrix multiplication. In Katsusuke Nabeshima,
[Gal14]
Kosaku Nagasaka, Franz Winkler, and Ágnes Szántó, editors, International Symposium on
Symbolic and Algebraic Computation, ISSAC ’14, Kobe, Japan, July 23-25, 2014, pages 296–303.
ACM, 2014. doi:10.1145/2608628.2608664.
[GMO19] Grzegorz Gluch, Jerzy Marcinkowski, and Piotr Ostropolski-Nalewaja. The first order truth
behind undecidability of regular path queries determinacy. In 22nd International Conference on
Database Theory, ICDT 2019, March 26-28, 2019, Lisbon, Portugal, pages 15:1–15:18, 2019.
[HKNS15] Monika Henzinger, Sebastian Krinninger, Danupon Nanongkai, and Thatchaphol Saranurak.
Unifying and strengthening hardness for dynamic problems via the online matrix-vector multiplication conjecture. In Proceedings of the Forty-Seventh Annual ACM on Symposium on
Theory of Computing, STOC 2015, Portland, OR, USA, June 14-17, 2015, pages 21–30, 2015.
doi:10.1145/2746539.2746609.
[HLNW17] Monika Henzinger, Andrea Lincoln, Stefan Neumann, and Virginia Vassilevska Williams. Conditional hardness for sensitivity problems. In 8th Innovations in Theoretical Computer Science
Conference, ITCS 2017, January 9-11, 2017, Berkeley, CA, USA, pages 26:1–26:31, 2017.
doi:10.4230/LIPIcs.ITCS.2017.26.
[LM13]
Katja Losemann and Wim Martens. The complexity of regular expressions and property paths in
SPARQL. ACM Transactions on Database Systems (TODS), 38(4):24:1–24:39, 2013.
[LMV16] Leonid Libkin, Wim Martens, and Domagoj Vrgoc. Querying graphs with data. Journal of the
ACM, 63(2):14:1–14:53, 2016.
[Mar]
Wim Martens. Personal communication by email, October 28, 2019.
[MNT20] Wim Martens, Matthias Niewerth, and Tina Trautner. A trichotomy for regular trail queries. In
37th International Symposium on Theoretical Aspects of Computer Science, STACS 2020, March
10-13, 2020, Montpellier, France, pages 7:1–7:16, 2020. doi:10.4230/LIPIcs.STACS.2020.7.
[MS91]
Bernard M. E. Moret and Henry D. Shapiro. Algorithms from P to NP (Vol. 1): Design and
Efficiency. Benjamin-Cummings Publishing Co., Inc., USA, 1991.
[MT18]
Wim Martens and Tina Trautner. Evaluation and enumeration problems for regular path queries.
In 21st International Conference on Database Theory, ICDT 2018, March 26-29, 2018, Vienna,
Austria, pages 19:1–19:21, 2018.
[CMW87]

<!-- page 39 -->

Vol. 19:4

[MT19a]
[MT19b]
[Mun71]
[MW95]
[RBV17]

[RRV17]
[Seg15]
[Tar72]
[Wil04]

[Wil05]
[Wil12a]
[Wil12b]

[Wil15]

[Wil18]
[Woo12]
[WW18]
[YZ05]

FINE-GRAINED COMPLEXITY OF REGULAR PATH QUERIES

15:39

Wim Martens and Tina Trautner. Bridging theory and practice with query log analysis. SIGMOD
Rec., 48(1):6–13, 2019. doi:10.1145/3371316.3371319.
Wim Martens and Tina Trautner. Dichotomies for evaluating simple regular path queries. ACM
Trans. Database Syst., 44(4):16:1–16:46, 2019. doi:10.1145/3331446.
J. Ian Munro. Efficient determination of the transitive closure of a directed graph. Inf. Process.
Lett., 1(2):56–58, 1971. doi:10.1016/0020-0190(71)90006-8.
Alberto O. Mendelzon and Peter T. Wood. Finding regular simple paths in graph databases.
SIAM Journal on Computing (SICOMP), 24(6):1235–1258, 1995.
Miguel Romero, Pablo Barceló, and Moshe Y. Vardi. The homomorphism problem for regular
graph patterns. In 32nd Annual ACM/IEEE Symposium on Logic in Computer Science, LICS
2017, Reykjavik, Iceland, June 20-23, 2017, pages 1–12, 2017. doi:10.1109/LICS.2017.8005106.
Juan L. Reutter, Miguel Romero, and Moshe Y. Vardi. Regular queries on graph databases.
Theory of Computing Systems (ToCS), 61(1):31–83, 2017.
Luc Segoufin. Constant delay enumeration for conjunctive queries. SIGMOD Record, 44(1):10–17,
2015.
Robert Endre Tarjan. Depth-first search and linear graph algorithms. SIAM J. Comput., 1(2):146–
160, 1972. doi:10.1137/0201010.
Ryan Williams. A new algorithm for optimal constraint satisfaction and its implications. In
Automata, Languages and Programming: 31st International Colloquium, ICALP 2004, Turku, Finland, July 12-16, 2004. Proceedings, pages 1227–1237, 2004. doi:10.1007/978-3-540-27836-8\
_101.
Ryan Williams. A new algorithm for optimal 2-constraint satisfaction and its implications. Theor.
Comput. Sci., 348(2-3):357–365, 2005. doi:10.1016/j.tcs.2005.09.023.
Virginia Vassilevska Williams. Algorithms column: An overview of the recent progress on matrix
multiplication. SIGACT News, 43(4):57–59, 2012. doi:10.1145/2421119.2421134.
Virginia Vassilevska Williams. Multiplying matrices faster than coppersmith-winograd. In
Howard J. Karloff and Toniann Pitassi, editors, Proceedings of the 44th Symposium on Theory of
Computing Conference, STOC 2012, New York, NY, USA, May 19 - 22, 2012, pages 887–898.
ACM, 2012. doi:10.1145/2213977.2214056.
Virginia Vassilevska Williams. Hardness of easy problems: Basing hardness on popular conjectures
such as the strong exponential time hypothesis (invited talk). In 10th International Symposium
on Parameterized and Exact Computation, IPEC 2015, September 16-18, 2015, Patras, Greece,
pages 17–29, 2015. doi:10.4230/LIPIcs.IPEC.2015.17.
Virginia Vassilevska Williams. Some open problems in fine-grained complexity. SIGACT News,
49(4):29–35, 2018. doi:10.1145/3300150.3300158.
Peter T. Wood. Query languages for graph databases. SIGMOD Rec., 41(1):50–60, 2012. doi:
10.1145/2206869.2206879.
Virginia Vassilevska Williams and R. Ryan Williams. Subcubic equivalences between path, matrix,
and triangle problems. J. ACM, 65(5):27:1–27:38, 2018. doi:10.1145/3186893.
Raphael Yuster and Uri Zwick. Fast sparse matrix multiplication. ACM Trans. Algorithms,
1(1):2–13, 2005. doi:10.1145/1077464.1077466.

This work is licensed under the Creative Commons Attribution License. To view a copy of this
license, visit https://creativecommons.org/licenses/by/4.0/ or send a letter to Creative
Commons, 171 Second St, Suite 300, San Francisco, CA 94105, USA, or Eisenacher Strasse 2,
10777 Berlin, Germany
