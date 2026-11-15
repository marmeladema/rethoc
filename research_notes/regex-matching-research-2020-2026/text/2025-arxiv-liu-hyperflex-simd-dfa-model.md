---
file: ../pdfs/2025-arxiv-liu-hyperflex-simd-dfa-model.pdf
citation: "Hyperflex: A SIMD-based DFA Model for Deep Packet Inspection. Liu, Zhu, Chang, Hong, Langdale, Qiu, Zhao. arXiv 2512.07123, 2025"
doi: 
source: "https://arxiv.org/pdf/2512.07123"
topics: [performance_hardware]
pages: 14
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

1

Hyperflex: A SIMD-based DFA Model for Deep
Packet Inspection

arXiv:2512.07123v1 [cs.NI] 8 Dec 2025

Yang Liu, Wenjun Zhu, Member, IEEE, Harry Chang, Yang Hong, Geoff Langdale, Kun Qiu, Senior
Member, IEEE, and Jin Zhao, Senior Member, IEEE

Abstract—Deep Packet Inspection (DPI) has been extensively
employed for network security. It examines traffic payloads by
searching for regular expressions (regex) with the Deterministic
Finite Automaton (DFA) model. However, as the network bandwidth and ruleset size are increasing rapidly, the conventional
DFA model has emerged as a significant performance bottleneck
of DPI. Leveraging the Single-Instruction-Multiple-Data (SIMD)
instruction to perform state transitions can substantially boost
the efficiency of the DFA model. In this paper, we propose
Hyperflex, a novel SIMD-based DFA model designed for highperformance regex matching. Hyperflex incorporates a region
detection algorithm to identify regions suitable for acceleration
by SIMD instructions across the whole DFA graph. Also, we
design a hybrid state transition algorithm that enables state
transition in both SIMD-accelerated and normal regions, and
ensures seamless state transition across the two types of regions.
We have implemented Hyperflex on the commodity CPU and
evaluated it with real network traffic and DPI regexes. Our
evaluation results indicate that Hyperflex reaches a throughput of
8.89Gbit/s, representing an improvement of up to 2.27 times over
Mcclellan, the default DFA model of the prominent multi-pattern
regex matching engine Hyperscan. As a result, Hyperflex has
been successfully deployed in Hyperscan, significantly enhancing
its performance.
Index Terms—Deep Packet Inspection, Regular Expression,
Deterministic Finite Automata.

I. I NTRODUCTION

D

EEP Packet Inspection (DPI) [1]–[4] serves as a pivotal component in numerous network security frameworks, including Intrusion Detection/Prevention Systems
(IDS/IPS) [5]–[7], Web Application Firewalls (WAF) [8]–[10],
and application identification systems [11]. Unlike conventional packet inspection methods, which only focuses on the
fixed 5 tuples in packet headers, DPI delves into the entire
packet payloads [12]–[15]. It analyzes the packet payloads
by searching for specific rules (i.e., patterns or signatures),
which are usually regular expression (regex) rules because of
their enhanced expressiveness and ability to describe a wide
variety of payload signatures [16]–[19]. Consequently, DPI
necessitates extensive regex matching to fulfill its role.
Nowadays, the network bandwidth is dramatically increasing [20]–[22] and many DPI-based applications demand realtime stream processing, which takes the high demand of the
regex matching efficiency [4], [23], [24]. However, the conventional regex engine struggles to meet the high-performance
Yang Liu, Wenjun Zhu, Kun Qiu, and Jin Zhao are with the School of
Computer Science, Fudan University, Shanghai 200438, China
Harry Chang, Yang Hong and Geoff Langdale are with Intel Asia-Pacific
Research & Development Ltd., Shanghai 200240, China

requirements for extensive and complex networks. It has been
reported that RE2 [25], Google’s regex engine, takes 4696
seconds to perform 1GB traffic matching with Intel Xeon
Platinum 8180 CPU, while this traffic contains only 818,682
packets [26]. Another popular regex engine, PCRE [27], is
even slower than it. DFA (Deterministic Finite Automata) is
frequently utilized in regex matching due to its stable processing and no-backtracking characteristic, ensuring efficient and
consistent regex recognition [15], [28]–[31]. So it is an urge to
enhance the performance of the DFA model. There have been
some research efforts towards improved DFA models [26],
[32].
The well-known regex matching engine, Hyperscan [26],
supports parallel matching of regex rules by employing graph
decomposition techniques to transform regex matching into a
series of string matching and automaton matching tasks. It
strives to employ DFA model for subsequent state transition
processes to fully exploit the high-performance advantages of
DFA. Hyperscan boasts performance enhancements of up to
10.3x over RE2 and 2.3x improvement over PCRE2, positioning it as the fastest regex engine [26].The DFA model
of Hyperscan is called Mcclellan, which leverages efficient
state analysis algorithms to simplify state graphs to facilitate
efficient state transitions. However, Mcclellan still employs the
traditional DFA state transition algorithm, which is fundamentally a memory access procedure on a two-dimensional table.
This method inherently incurs significant time overhead due
to the necessity of searching the state transition table, where
each transition may be hindered by memory access latency.
As such, there exists considerable potential for enhancing the
performance of the traditional state transition algorithm.
The recently proposed high-performance DFA model Hyperverse [32] uses SIMD instructions to accelerate DFA state
transitions and incorporates a parallel state transition algorithm. Compared to the traditional DFA model, Mcclellan,
Hyperverse significantly improves performance. However, this
model relies on SIMD instructions, and most contemporary
CPUs are equipped with SIMD instruction sets that do not
surpass 512 bits in length, such as the AVX512 instruction
set, thereby limiting the maximum number of DFA states to
64. So Hyperverse is only applicable to DFAs with fewer than
64 states. Moreover, numerous real-world DPI regexes, when
transformed into DFAs, involve a state count significantly exceeding 64 states [16]. Thus Hyperverse cannot be effectively
applied to practical matching scenarios.
In this paper, we introduce Hyperflex, a SIMD-based DFA
model that enables all regexes to benefit from the high per-

<!-- page 2 -->

2

formance of SIMD instructions. Initially, Hyperflex identifies
the most suitable region from the whole regex to apply with
SIMD-based parrel state transition. The selection of the region
significantly impacts the efficiency of the model. Next, we
design a hybrid transition algorithm to deal with the state
transition of two kinds of regions and ensure seamless state
transitions across different regions. Notably, Hyperflex reaches
a throughput of 8.89 Gbit/s, up to 2.27x of Mcclellan. It has
been successfully deployed in Hyperscan. Briefly speaking,
this paper makes the following contributions:
• we propose a region detection algorithm, which detects
the most suitable region of the entire state graph for
employing SIMD-based state transition. When a state
enters a suitable region, it is more likely to circulate
within the region rather than exiting it. This can take
advantage of the high-performance of SIMD instruction
as soon as possible. The algorithm substantially enhances
the performance of our model. If the region is chosen
randomly, the performance of Hyperflex even decreases
by 1.66x.
• We design a hybrid transition algorithm to make states
from different regions run under different mechanisms,
while meticulously managing the transitions across these
regions. As part of this algorithm, we design the gutter
state transition table to address the potential mismatching
issues caused by region switching. Moreover, to expedite
the identification of the earliest escape state, we incorporate a SIMD-based escape state detection method into the
hybrid algorithm. These components operate in concert to
ensure seamless state transitions and enhance efficiency.
• We implement Hyperflex on a commodity CPU with
AVX512 SIMD support. We conduct experiments to
compare Hyperflex with Mcclellan and other mainstream
DFA models.
II. RELATED WORK
A. Deterministic Finite Automaton
A deterministic
finite automaton (DFA) is defined as a

5-tuple S, Σ, M, s0 , F , which embodies: a finite collection
of states S, an alphabet Σ constituting the set of input
symbols, a transition function M , a designated initial state s0 ,
and a collection of accept states F [33]–[35]. The matching
procedure is as follows. Consider c = {c0 , c1 , c2 , . . . , cn }
as a string formulated from the alphabet Σ. Typically, a
DFA initializes with the state s0 . Then the DFA progresses
through each character of string c, it transitions from state
si−1 to state si by the transition function M . If the final state
sn ∈ F , then the string is accepted by the DFA. Conversely,
the DFA rejects the string, culminating in a non-match [29],
[36], [37]. We conceptualize the operation of state transition
with T . The process of DFA can be expressed as Formula 1:
s = T [c][s]

(1)

Formula 1 shows that the performance of DFA is not only
affected by the number of states but also determined by the
performance of the state transition algorithm.

B. Hardware Acceleration Methods
To accelerate regex matching, there have been some solutions leveraging hardware accelerators like FPGA, GPU or
TACM [38]–[47].
For example, Lin et al. [45] devised an FPGA-based acceleration architecture, CPDFA, which employs a mixed storage
strategy to reduce DFA memory requirements: high-frequency
states are stored via indirect indexing, while low-frequency
states utilize direct indexing. Meiners et al. [47] introduced
a TCAM-based DFA optimization scheme using dimension
splitting. This method decomposes high-dimensional classifiers into multiple lower-dimensional classifiers and distributes
them across the TCAM pipeline, lowering memory consumption while boosting throughput.
Nonetheless, these methods exhibit three main drawbacks:
high capital costs, insufficient memory to hold a large number
of rules and limited flexibility, as any algorithmic update
necessitates a complete hardware redesign. Although performance does improve, these solutions are rarely deployed. It is
still the software regex matching algorithms running on CPUs
that are adopted in most real scenarios.
C. DFA Space Compression Algorithms
Kong et al. [48] observe the similarity in the transition
behaviors of different input characters in the DFA transition
table and partition the state set into multiple disjoint subsets. For each subset, they create an independent re-encoded
alphabet and map the corresponding transition table to this
new encoding through a mapping table, thereby reducing the
overhead of redundant storage.
Kumar et al. [29] propose the classic D2 FA transition edge
compression algorithm. The core idea behind this algorithm
is to share transitions among states with similar transition
tables and to establish default transitions between them, thus
reducing memory usage. However, default transition paths may
grow excessively long, which in turn raises memory access
overhead. To address this issue, Becchi et al. [49] introduce
the A-DFA method. By imposing a “state depth” constraint, ADFA restricts the number of default-transition hops, ensuring
that each character visits at most two states. Under identical
memory bandwidth constraints, A-DFA exhibits a significantly
higher compression rate than D2 FA.
DFA space compression algorithms can reduce DFA space
consumption but often incur substantial preprocessing time,
making them unsuitable for dynamic network scenarios with
large-scale rule sets and frequent rule updates. The practical
DFA performance is greatly affected of the state transition
algorithm.
D. The Traditional DFA State Transition Algorithm
The Traditional DFA State Transition Algorithm relies
on a two-dimensional transition table for navigating state
transitions. This table is meticulously structured, with rows
corresponding to states generated from the regex, and columns
indexed from 0 to 255, encompassing all the 256 ASCII characters. This structured approach ensures that the DFA model,

<!-- page 3 -->

3

TABLE I
T RADITIONAL S TATE T RANSITION TABLE FOR R EGEX “ MODE + L”

State

...

d

e

...

l

m

...

o

TABLE II
SIMD- BASED S TATE T RANSITION TABLE FOR R EGEX “ MODEL”

Character

...

0

...

0

0

...

0

1

...

0

...

1

...

0

0

...

0

0

...

2

...

2

...

3

0

...

0

0

...

0

...

3

...

0

4

...

0

0

...

0

...

4

...

0

4

...

5

0

...

0

...

5

...

0

0

...

0

1

...

0

...

from any specific state, can accurately ascertain the next state
by querying the table according to the input character. For
instance, considering the regex “model”, Fig. 1 visualizes its
state graph, while Table I details the transition table extracted
from this graph, where “5” indicates the accept state.
starting state

0

m

1

o

terminal state

2

d

3

e

4

l

5

e
Fig. 1. DFA representation for regex “mode+l”

The working process of the traditional state transition algorithm is essentially a sequence of queries to the state transition
table. The transition to the subsequent state is determined by
the current state and the character in the string. Consider the
example string “hmodel”, the matching progress is illustrated
in Fig. 2. This matching mechanism is straight, and the table’s
capacity to encompass all possible states enables the DFA
model to apply to all regexes. However, if the table consists of
500 states that fit into the L1 (the latency is one CPU cycle)
cache. Because the L1 cache size is typically under 64KB,
it cannot accommodate the extensive size of the transition
table, which may approximate 500 KB. Consequently, each
step in the matching process likely involves loading segments
of the table from main memory into the L1 cache, incurring
a latency of 4 to 5 CPU cycles. It also requires 2 cycles of
arithmetic computation to get the position within the transition
table. Therefore, the time spent on each matching process
amounts to 7 to 8 CPU cycles, presenting substantial potential
for optimization.

Vector (label 0–63)

...

...

d

{003000...0}

e
...

{000440...0}
...

l

{000050...0}

m
...

{100000...0}
...

o
...

{020000...0}
...

E. The SIMD-based DFA State Transition Algorithm
Hyperverse is a SIMD-based DFA model [32], which introduces a novel SIMD-based algorithm for state transitions. It
enables an integer state into a vector comprising 64 identical
state numbers and utilizes the SIMD instruction shuffle for
the transition process, eliminating the need to directly access
the transition table. Fig. 3 shows how two state vectors
perform a shuffle operation for state transition. The shuffle
operation rearranges 8-bit integers in Src across lanes using
the corresponding indices in Idx, and stores the result in Des.
The “S0 ” is a start vector of state, and the “m” is the state
vector of the ASCII ‘m’. Through the shuffle operation, the
model can take out the value in the input character ‘m’ vector
according to the value of the initial state to get the subsequent
state “S1 ”.
64 8-bit integers

S₀

m

S₁

0

0

1

1

1

0

0

1

2

0

0

1

3

0

0

1

4

0

0

1

5

0

0

1

...

...

...

...

0

0

1

Idx vector

Src vector

Des vector

index

63

Fig. 3. Shuffle operation for state transition

string: "hmodel"
0

T[0]['h']

0

T[1]['m']

1

T[1]['o']

2

T[2]['d']

3

T[3]['e']

Fig. 2. Traditional state transition algorithm working progress

4

T[4]['l']

5

We use the input string “model” and the regex “mode+l” as
an example to explain the whole matching progress in Fig. 4.
For each character in the input string, we retrieve a corresponding vector from Table II, derived from the state graph.
This table encompasses vector representations of all ASCII
characters, with each vector being 512 bits and comprising 64

<!-- page 4 -->

4

integer states. The size of the table is 16KB, which can be
easily accommodated by the L1 cache. So it saves the time
that loads the table from memory to the L1 cache. Then, we
operate a shuffle operation with the character vector and the
state vector to get the subsequent state vector, which consumes
3 CPU cycles. Finally, we also get the correct accept state.
The process leverages the shuffle instruction as an alternative
to traditional table queries, consuming merely 4 CPU cycles.
Hyperverse further exploits the parallelism of SIMD instructions. After introducing SIMD instructions, two characters’
corresponding SIMD vectors can take the VPERMB instruction to progress the state transition, which does not depend on
the previous state. Thus, the original serial operation can be
split into two parallel chains, thereby further accelerating the
state transition speed.
We also take the input string “model” as an example and
decompose its matching process into two chains, as shown in
Equation 2. “Sinit ” denotes the initial state vector, while “m”
and other letters represent the SIMD vectors corresponding
to the input characters. “S ⊗ m” denotes performing the
VPERMB operation on “S” and “m”.
Smodel = {Sinit ⊗ m ⊗ o} ⊗ {d ⊗ e ⊗ l}

: VPERMB instruction

S₀

m

S₂

d

S₄

l

0

0

1

2

0

4

0

1

0

0

2

0

4

0

2

0

0

2

3

4

0

3

0

0

2

0

4

0

4

0

0

2

0

4

5

index

5

0

0

2

0

4

0

...

...

...

...

...

...

...

63

0

0

2

0

4

0

S₁

o

S₃

e

S₅

0

1

0

3

0

5

1

1

2

3

0

5

2

1

0

3

0

5

3

1

0

3

4

5

4

1

0

3

4

5

5

1

0

3

0

5

...

...

...

...

...

...

63

1

0

3

0

5

index

(2)

The result of Chain 2, {d ⊗ e ⊗ l}, is independent of the
result of Chain 1, {Sinit ⊗ m ⊗ o}, and can be executed
in parallel. By dividing the entire state transition process
into two computational chains, the serial nature of the state
transitions is replaced with a parallel model, further improving
the speed of state transitions. This design further enhances
Hyperverse’s performance, making it the fastest DFA model
available today. However, it only applies to regex containing
less than 64 states, given that the AVX512 instruction spans
512 bits, accommodating a maximum of 64 states [50], [51].
So Hyperverse can not be applied in practical scenarios as
numerous regexes transformer to DFAs comprise states more
than 64.
III. HYPERFLEX ARCHITECTURE
A. Design Overview
As shown in Fig. 5, Hyperflex contains two main stages: the
compilation stage and the runtime stage. In the compilation
stage, the input regex is transformed into a DFA graph where
nodes represent states and edges represent state transitions.
Then, for applying the SIMD-based state transition algorithm,
a suitable region limited to 64 states is identified from the
whole graph. If the region is accepted via evaluation, the interregion transition table and the outer-region transition table are
constructed to facilitate the subsequent matching process. In
the runtime stage, the current state is examined to determine
which region it is in and decide to use which transition table
to process the state transition. A state transition is executed
based on the current state, the relevant transition table, and
the subsequent character in the input string. In the matching
process, it is crucial to ensure the continuity of state transitions
across regions. This system makes Hyperflex achieve better
state transition performance than traditional DFA model and
meanwhile solves the scale capability issue of the Hyperverse.

Fig. 4. SIMD-based state transition algorithm working progress

B. The Region Detection Algorithm
The SIMD-based state transition algorithm is more efficient
but only applies to DFA whose states are fewer than 64, owing
to the limitation of the longest SIMD vector in modern CPUs.
To employ the algorithm with any regex, it is imperative
to select a suitable state region from the entire DFA graph,
referred to as the hyper region. The rest of the DFA graph is
called outer region. Detecting the hyper region necessitates
a comprehensive analysis of the entire graph to determine the
starting state, followed by expansion to get the hyper region.
Subsequently, the quality of the hyper region is evaluated
to determine acceptance or rejection. It is crucial to pick a
suitable region in our design, which significantly impacts the
performance of Hyperflex. The detail of the region detection
algorithm is shown in Part IV.

C. The Hybrid Transition Algorithm
The hybrid transition algorithm makes states from different
regions run under different mechanisms. It examines the
current state’s location to determine the next transition process.
In the state transitions within the hyper region, to reduce the
frequency of checking the region to which a state belongs
and to leverage the parallelism of SIMD instructions, multiple
characters are processed in a batch for parallel state transitions.
Additionally, to address the issue of a state exiting the region
within the same batch, which could lead to subsequent matching errors, we propose a novel state transition table. This new
table works in conjunction with the original state transition
table to accurately detect instances where a state exits the
region and ensures proper handling of subsequent processes.

<!-- page 5 -->

5

Region Detection
Regex

DFA Graph

Region
Expansion

Starting Position
Detection

drop

Return to
Traditional DFA

keep

Build Hybrid
DFA Transition
Table

Region
Evaluation

Compilation
Runtime

ion

g
outer re

Traditional State Transition

a si

ngle

Check New State
hyper
r

Hybrid Transition

egion

Simd-based State Transition

cha
ract
er
aracte

ed ch
batch

String

rs

Fig. 5. Design overview for the Hyperflex DFA model

Detailed information about the hybrid transition algorithm is
provided in Part V.

outer region
hyper region edges
outer region edges

IV. T HE H YPER R EGION D ESIGN

cross region edges

hyper region

The core of the Hyperflex design is to find the hyper
region of the whole DFA graph to apply the SIMD-based state
transition algorithm. This is a new problem both in automata
theory and graph theory. We define the problem on our own
and provide an analytical modeling framework.
A. Problem Analysis
Hyperverse utilizes the SIMD instruction VPERMB to process the state transition, the SIMD vector of the shuffle needs
to contain all the states of the DFA. While the longest SIMD
vector of most modern CPUs has only 512 bits, Hyperverse
can only be applied for less than 64-state DFA(the SIMD
vector takes 8 bits to represent an integer). As shown in Fig. 6,
to make all regexes benefit from the high performance of
SIMD instruction, it is imperative to identify the hyper region
within the entire DFA graph. The detection of the hyper region
is far more difficult than randomly picking a region with less
than 64 states. The states in the hyper region must adhere to
a specific principle. When the state enters the hyper region,
it tends to circulate within the region rather than promptly
leaving it. This is because when a state turns to another
region, it cannot benefit from the high efficiency of the SIMD
instruction and requires additional operations to perform the
transition. A bad hyper region results in minimal matching
progress with the SIMD-based state transition algorithm and
requires numerous change operations, significantly impairing
the efficiency of the DFA model.
B. Good Hyper Region Properties
The SIMD-based state transition algorithm has better state
transition speed, so in Hyperflex, we hope once a state enters
the hyper region, it is better to stay in the region as much
as possible to make more use of its speed advantage, which

Fig. 6. A hyper region in the whole DFA graph

is the principle when detecting the hyper region. To achieve
that, besides containing states less than 64, a good hyper region
whose states adhere to the principle should have the following
three properties.
Property 1. States in the region better come from the same
strongly connected component.
Property 2. States in the region should have relatively large
state stickiness.
Property 3. States in the region are better located near the
DFA start state.
Next, we will introduce the three properties in detail.
1) Property 1: We hope the hyper region better does
state transition inside itself. In graph theory, there is one
concept that has similar characteristics: strongly connected
component(SCC). A graph is said to be strongly connected
if every vertex is reachable from every other vertex. The
strongly connected components of an arbitrary directed graph
form a partition into subgraphs that are themselves strongly
connected. As shown in Fig. 6, the states in the same strongly
connected component are more likely to transition in the interregion, and when it leaves the region, it will never come back.

<!-- page 6 -->

6

Thus, states in the hyper region are better to come from the
same strongly connected component as possible.
2) Property 2: To evaluate the different states from a
strongly connected component, we propose the concept of
“state stickiness”. State stickiness represents the number of
different characters on the in-edges of a state. The large
stickiness means a relatively large possibility of returning to
this state from adjacent states. For example, as depicted in
Fig. 7, state 0 has 4 different in-edges, thus its state stickiness
is 2. Meanwhile, state 1 has 8 in-edges, but its state stickiness
is 2 as it only has two types ‘a’ and ‘c’. State 0 has a relatively
larger possibility of back transition to itself. Since hyper region
needs a high probability of internal state transition, we think
states in the region are better to have higher state stickiness.
DFA state
state transition

a
b

0
c

c a
d

a
c

1

c
a
c

a

Fig. 7. Two states of different in-edges to show the state stickiness

3) Property 3: According to the characteristics of DFA
model matching, subsequent states are more likely to revert to
previous states than to progress forward. Thus, states closing
to DFA starting state are more likely to be active during DFA
runtime. Similarly, a hyper region closer to the DFA starting
state is more likely to be used, compared to other regions far
from the DFA graph entrance. Therefore, the search process
should start from the strongly connected component near the
DFA graph entrance.
C. Hyper Region Detection
The above three properties guide the search of the hyper
region. First, through a comprehensive analysis of the DFA
graph, the algorithm identifies all SCCs in the graph, and then
selects an appropriate starting search region from them. This
process leverages Property 1.
Next, the algorithm traverses each SCC in order of its distance from the DFA’s initial state, summing the state stickiness
values of all states within it. It then checks whether this sum
exceeds the experimentally determined threshold σ, the value
of the σ is detailed in Part VI. If an SCC’s total state stickiness
value falls below σ, the algorithm bypasses it and continues
searching for a suitable region. If no SCC meets the threshold,
the algorithm terminates and defaults to the traditional state
transition process. This process not only satisfies Property 3,
which prioritizes the SCC closer to the DFA’s initial state,
but also leverages Property 2, ensuring that states within the
region exhibit strong state stickiness, thereby increasing the
likelihood of efficient looping within the region.
Upon identifying the starting state, the hyper region is
expanded to encompass a maximum of 64 states. We choose

Board Forward Search(BFS) to expand the hyper region.
Because the BFS continuously involves nearby states as much
as possible, the states in the hyper region are more likely to
come from the same strongly connected component closer to
the DFA starting state, leveraging Property 1 and Property
3. After region expansion to a candidate hyper region, a
scoring model is executed to decide whether to accept the
hyper region. The detailed design of the evaluation model is
presented below.
D. Hyper Region Evaluation
The detection of the hyper region is a qualitative analysis
based on the three properties. To more accurately assess
the goodness of the candidate region, we have developed a
quantitative scoring model termed region leakiness.
Whenever the state transits from the hyper region into the
outer region, this is referred to as a leakiness transition. In
Fig. 8, the dashed edges represent leakiness transitions. To
calculate the probability of a leakiness transition, we need to
get the possibility of a state transition via a particular outedge first. As Fig. 8 shows, C1 , C2 and C3 are 3 mutually
exclusive character sets corresponding to 3 out-edges of state
1. |C1 |,|C2 | and |C3 | means the count of characters of each set,
the value is called the width of the edge, denoted by width(e).
In this model, the total number of characters in the ASCII set
is 256, and thus the sum of all outgoing edge widths is 256.
The formula of edge transition probability is as follows.
hyper region state

outer region state

Ci input char

candidate hyper region
...

0

outer region

C2

1

...
C1

...

2

...

3

...

N

C3

Fig. 8. Leakage probabilities of a candidate hyper region

width(e)
(3)
256
Formula 3 is the probability of a state transiting to the
subsequent state through a specified out-edge. This formula
serves as a cornerstone for the subsequent calculation of
hyper region leakiness. And the probability of the candidate
hyper region leakiness is utilized to assess the quality of the
region. Ideally, a high-quality hyper region should exhibit a
low leakage probability. Formula 4 specifically addresses the
leakiness probability of a given state.


100%,
u ∈ Outer Region

P
Pleak (u) =
Ptransit (e) × Pleak (v) u ∈ Hyper Region

 v=dest of e
Ptransit (e) =

e∈out-edge(u)

(4)
In accordance with Formula 4, the outer region state exhibits
a leakage probability of 100%. The leakage probability of a
hyper region state is calculated by the weighted average of the

<!-- page 7 -->

7

leakiness probabilities of all its subsequent states, where the
weight is the out-edge transition probability.
The leakiness evaluation model accesses the leakiness probability of the candidate hyber region. The larger the result is,
the more possible this candidate region has state transition
leakage into the outer region. We treat the leakiness result
of the starting state as the leakiness of the candidate region.
When the leakiness probability of a candidate hyper region
is below the region leakiness threshold, λ, the region is
considered a valid hyper region, and SIMD state transitions
are applied. Otherwise, the DFA will use the traditional state
transition algorithm. The leakiness threshold λ, is determined
experimentally, and the specific experimental details will be
discussed in Part VI.
E. Hybrid DFA State Finalization
Following the leakiness evaluation, a decision is made on
whether to accept the candidate region as a hyper region.
If not, all further matching processes are handled by the
traditional DFA model. Conversely, if the region is identified
as a hyper region, it becomes necessary to reconstruct the
state transition table. This involves dividing the entire graph
into two regions, which leads to the development of two
separate DFA transition tables: table Tt for outer region,
and table Ts for hyper region. Each transition table only
contains the states from their respective region. Table Ts is
a one-dimensional array containing each character’s transition
mask vector, and table Tt is a two-dimensional array like
the traditional transition table, containing all the state’s next
transition state through each character. The building progress
is as follows.
Initially, the state values within the entire DFA graph are
reconfigured. A hash table Tn is established to maintain the
relationship between each state and its newly assigned value.
The states in the hyper region are reassigned numbers ranging
from 0 to 63 as the sequence they are added to the hyper
region. Similarly, the states in the outer region are renumbered
starting from 64, corresponding to their original size sequence.
In the two newly created transition tables, these reassigned
numbers are utilized to represent the current state.
This design has two main advantages. Firstly, it maximizes
the utilization of the SIMD mask vector space in the table
Ts . The SIMD mask vector can only incorporate 64 states.
Without reassigning state values, the mask would be unable
to cover all states of the hyper region. Secondly, this strategy
significantly reduces the size of the table Tt . Consistent with
the principle of locality, reducing the array size contributes to
decreasing the frequency of cache and memory swaps, leading
to enhanced performance.
V. H YBRID S TATE T RANSITION A LGORITHM
After detecting the appropriate hyper region and completing
hybrid DFA state finalization, the hybrid transition algorithm
is responsible for doing each state transition process. As the
Fig. 9, this algorithm uses the 2 state transition tables: table
Tt for outer region and table Ts for hyper region. It keeps
checking the current state type to decide which table to use
for subsequent transition.

Traditional transition

n

egio
er r

out

es
ca

pe

single state transition
s₁=Tₜ[s₀][c]

hy

S

pe

rr

e

eg

io
n

Check State
hype

r reg

ion

SIMD-based transition

chain 1

S1=VPERMB(S0,Ts[c0])
S2=VPERMB(S1,Ts[c1])
S3=VPERMB(S2,Ts[c2])
S4=VPERMB(S3,Ts[c3])

chain 2

C4->5=VPERMB( Ts[c4],Ts[c5])
C4->6=VPERMB( C4->5,Ts[c6])
C4->7=VPERMB( C4->6,Ts[c7])
C4->8=VPERMB( C4->7,Ts[c8])

Region Escaping Check
find the earliest state escaping
hyper region Se in order to pass
to the outer region

in
stay

ion

r reg

hype

S9=VPERMB(S4, C4->8)

Fig. 9. The process of the hybrid transition algorithm

A. The Procedure of the Hybrid Transition
When the hyper region is detected and the state transition
table is reconstructed, the hybrid state transition algorithm
takes responsibility for performing state transitions in different
regions. By checking which region the current state belongs to,
the algorithm determines which state transition table and state
transition method to use, thereby ensuring both correctness
and efficiency in the transitions.
The specific state transition process is illustrated in Fig. 9.
In this figure and the subsequent description, VPERMB refers
to the SHUFFLE instruction in the AVX512 instruction set,
si represents a state integer, and Si denotes a SIMD state
vector consisting of 64 copies of si . Likewise, ci is an ASCII
character, Ci is the state vector fetched from Ts using ci , and
Ci→j signifies applying VPERMB successively to transform
Ci to Cj .
Firstly, the procedure commences with an analysis of the
current state. Each state in this framework correlates to a
unique value in the hash table Tn , with hyper region states
assigned lower IDs and outer region states assigned higher
IDs. The boundary between these regions is defined by a
Slimit , indicating the largest ID for a hyper region state.
States exceeding Slimit are processed using the table Tt of
outer region. In contrast, states within this threshold utilize
the table Ts of the hyper region. In the hyper region, a
SIMD-based parallel state transition algorithm is employed,
and the transition process is divided into two independent
chains, referred to here as Chain 1 and Chain 2. As there
are no interdependencies between these two chains, they
can be executed in parallel, thereby improving computational
efficiency. To further exploit the high-performance potential
of the SIMD-based parallel transition algorithm and reduce
the frequency of state region checks, a batch state transition
mechanism is introduced. In this mechanism, l input characters
are handled as a single batch. Details regarding the choice of
l are discussed in the following section, and for illustrative
purposes, a batch size of 9 is used as an example in the Fig. 9.
When performing batch state transitions in the hyper region,
it is necessary to ensure that none of the states in the batch
fall outside of the hyper region. However, in actual operation,
it is not always possible to keep all transitions strictly within

<!-- page 8 -->

8

the hyper region. In particular, there are two critical scenarios
that may occur.
• All results remain in hyper region: if all state transition results stay within the hyper region, the algorithm
proceeds by advancing S9 as the initial state for the
subsequent iteration. This scenario effectively exploits
the optimal performance of the SIMD-based parrel state
transition algorithm.
• One of the results transitions to outer region: if any
of the state jump out of hyper region, it is imperative
to identify the earliest escaped result, denoted as Sj .
Since all state transition results are obtained based on
the hyper region table, the escaped Sj means the outer
region table should be used instead, causing subsequent
results to become unreliable. We need to pass Sj instead
of S9 to the outer region transition.

chain 1
index
3
0

61

1

61

1

3

2

3

0

61

3

3

67

61

4

3

42

61

5
...

3

31

61

...

...

...

63

3

2

61

S0

C0

C1

chain 2

Slimit

5

C2

3

...

3

...

3

...

3

34

3

...

3

...

...

...

3

...

S4

C4->8

C3

5

7

17

12

3

1

10

18

45

27

0

61

4

19

41

67

23

13

43

34

4

2

54

3

10

3

31

71

21

36

...

...

...

...

...

2

1

31

47

61

C4

C5

C6

C7

C8

S9
S1
S2

67

S3

67
67

34

C4->5

34

31

34

10

34

S4

67

S1 > Slimit
detect the out-region
error

67
...
67

2

34

18

34

23

34

...
61

The final
state
transition
result is
incorrect.

: VPERMB instruction

C4->7

...

7

34

67

C4->6
C4->8

...

...

...

...

...

4

...

...

...

3

...

...

...

34

...

...

...

...

...

...

...

...

can not
detect
the outregion
error

...

B. Design of the New State Transition Table
1) Problem Analysis: Selecting the correct table and
method based on a state’s region is crucial in hybrid transition
algorithm. Each table contains only its regional states, thus
using a mismatched table inevitably leads to errors.
The SIMD-based parallel state transition algorithm performs
9 matches in a loop, each using the hyper region mask table.
This is necessary to perform the VPERMB operation and
achieve parallelism, but it assumes that no state will exceed the
hyper region during the process. In reality, a state may exceed
the hyper region midway through the matching processes,
resulting in subsequent matching errors. This type of error
is referred to as an “out-region error”.
Moreover, as shown in Fig. 9, the parallel state transition is
divided into two separate chains: Chain 1 and Chain 2. Chain 1
generates the state vectors S1 , S2 , S3 , and S4 . By contrast,
Chain 2 computes the possible transitions for each state in
response to specified input characters but discards intermediate
vectors S5 , S6 , S7 , and S8 . Consequently, any out-region
error in these intermediate steps cannot be detected, since the
algorithm does not preserve these vectors for inspection.
Fig. 10 illustrates how the existing algorithm handles the
out-region error in Chain 1 and Chain 2 respectively. If an
out-region error arises in Chain 1, it is immediately detected
and resolved by switching to the traditional transition algorithm. However, if the same situation occurs in Chain 2, the
VPERMB instruction simply applies a modulus-based hash to
the out-of-bounds index, thus masking the error. Because the
resulting state vector S9 may remain below the boundary limit,
it slips through subsequent boundary checks and ultimately
leads to an erroneous match result.
2) Proposed Design: In order to address the problem above,
it is imperative to detect the out-region error when it occurs.
The primary challenge lies in identifying the error with the
minimum possible number of comparisons. Remarkably, our
design accomplishes this objective by examining only one
state, thereby enhancing the efficiency of the DFA model.
To achieve this goal, we propose a new state transition table,
termed the gutter state transition table Tg , to be utilized in
conjunction with the existing transition table Ts . The primary

Fig. 10. Out-Region Error in Chain 1 and Chain 2

distinction between Tg and Ts lies in the addition of a specific
“out-of-bounds state”. This state indicates that a certain state
has exceeded the predefined boundary of the hyper region.
To ensure that Ts always has sufficient capacity to store this
out-of-bounds state, one regular state in the hyper region is
sacrificed, reducing the maximum number of hyper region
states from 64 to 63. Consequently, the value 63 is reserved
as the out-of-bounds state.
Perform out-region
detection on the result

Chain2 performs state transitions
according to Tg

establish gutter transition table
original state transition table Ts

5

7

17

12

3

1

10

18

45

27

0

61

4

19

41

67

23

13

43

34

4

2

54

34

10

3

31

71

21

36

...

...

...

...

...

2

1

31

47

61

C4

C5

C6

C7

C8

5

7

17

12

3

1

10

18

45

27

0

61

4

19

41

63

23

13

43

34

4

2

54

34

10

5

7

17

12

3

3

...

1

10

18

45

27

3

...

0

61

9

19

41

3

...

63

23

13

43

34

3

63

4

23

54

3

10

3

...

3

31

63

21

36

3

...

...

...

...

...

...

...

...

63

63

63

63

63

3

...

C4

C5

C6

C7

C8

S4

C4->8

S9
C4->5

3

31

63

21

36

...

...

...

...

...

63

63

63

63

63

31
10
7
63
18
23
...
63

gutter state transition table Tg

63

C4->6

...
...

...

...

...

63

...

...

63

...

...

...

...

63

...
63

VPERMB instruction

63

C4->7
C4->8

63

...

63

...

63

...

63

63

63

...

63

...

S9>Slimit
detect the out-region
error

...
63

Fig. 11. Design and usage of the gutter state transition table

Fig. 11 illustrates a concrete state transition process and
highlights the use of Tg . Orange squares represent state values
in Tg that differ from those in the original table Ts . When
an out-region error occurs, the state transitions to 63 and
remains there. The resulting state vector, composed entirely
of 63, will always exceed Slimit , allowing the algorithm to
accurately detect the out-region error and execute appropriate

<!-- page 9 -->

9

error-handling or adjustments.

step1：

64

56

48

40

32

24

16

8

0

S1 >> 56

0

0

0

0

0

0

0

s1

C. Earliest Escaping State Identification Algorithm

S2 >> 48

0

0

0

0

0

0

s2

s2

When an out-region error arises in Chain 1, it is necessary
to identify the earliest state escaping the hyper region, enabling the outer region to manage subsequent transitions. A
straightforward method compares each element of the state
vectors from Chain 1 with the boundary vector Slimit one
by one, which is intuitive but relies on conditional checks,
underutilizing hardware parallelism.
To address this, we proposes a SIMD-based algorithm that
identifies the earliest escaping state by grouping state vectors
and performing XOR operations in parallel. This not only
enhances detection speed but also supports scalable extensions
to handle additional state vectors as needed.
Fig. 12 illustrates the operational steps of the proposed
algorithm. Although the SIMD state vector has a length of 512
bits, the algorithm processes it in 64-bit groups for simplicity,
with identical operations applied to each group. Hence, only
the first 64 bits are shown. Each 64-bit segment Si consists
of eight-bit state integers si , with s denoting the maximum
state value in the hyper region. The notation s1:n represents
successive XOR operations on n values.
First, it extracts and aggregates all state values from the
Chain 1-generated vectors into a single vector. To achieve this,
each state vector is shifted by a distinct offset, and the results
are XORed to form an intermediate vector T. A subsequent
eight-bit right shift of T, followed by another XOR with T,
produces the vector Sall . In this manner, the four states from
Chain 1 are consolidated and aligned in their transition order.
Next, SIMD instructions pinpoint the earliest escaping state.
Suppose S2 is the first to exceed the boundary, with s2 >
s4 > s > s1 > s3 . The algorithm compares entries in Sall to
Slimit , takes their minimums, and subtracts them from Slimit .
Only states above the boundary value produce a zero in this
subtraction; states below the boundary yield negative results.
By extracting and inverting the sign bits, the algorithm derives
an eight-bit mask that flags escaping states with 1. Finally, a
CTZ operation locates the least significant set bit, indicating
the earliest escaping state. In this example, index 1 for S2 . This
outcome matches expectations, demonstrating the algorithm’s
ability to detect the earliest escaping state correctly.

S3 >> 40

0

0

0

0

0

s3

s3

s3

S4 >> 32

0

0

0

0

s4

s4

s4

s4

TABLE III
SIMD I NSTRUCTIONS U TILIZED IN THE E ARLIEST E SCAPING D ETECTION
A LGORITHM
Symbol

Operation

XOR

Bitwise XOR

SIMD Instruction
VPXORQ

MIN

Minimum Value

VPMINUB

SUB

Subtraction

VPSUBB

MASK

Extract Sign Bits

VPTESTMD

CTZ

Count Trailing Zeros

VPCTZ

T = XOR above
T

0

0

0

0

s4

s3:4

s2:4

s1:4

T >> 8

0

0

0

0

0

s4

s3:4

s2:4

Sall

0

0

0

0

s4

s3

s2

s1

Slimit

s

s

s

s

s

s

s

s

<0

0

<0

0

1

0

Sall = XOR above

Assume s2>s4>s>s1>s3
Stmp = MIN(Sall ,Slimit)SUBSlimit

step2：

Stmp

0

0

0

0

0

mask=!MASK（Stmp）
mask

1

1

1

1

1

res=CTZ(mask)=1

Fig. 12. Workflow of the SIMD-based Escaping State Identification Algorithm

VI. E VALUATION
A. Environment
We evaluate Hyperflex on a commodity CPU. We conduct
experiments on a Linux server (Ubuntu 20.04) with Intel Xeon
Platinum 8360Y (2.40GHz, 2 sockets, 36 cores per socket)
with 256GB DDR4 memory. The processor has the support
of AVX512. The hybrid DFA state transition algorithm is
implemented in C. We use GCC 9.4.0 to compile them without
any optimization.
B. Dataset
To evaluate the efficiency of Hyperflex, we conduct experiments on a variety of real-world DPI regex rules and network
traffic. Regex rules are collected from Snort Emerging Threats
Rules v2.9.0 [52], which is a network security rule set from
the well-known DPI application Snort [5]. The input strings
are derived from from IXIA HTTP packets, Alexa non-HTTP
packets, volunteer packets collected through the deployment
of routers , and randomly generated byte sequences, thereby
encompassing a broad spectrum of network traffic scenarios.
The specific sizes of these input strings are presented in Table
IV.
C. Performance Analysis
The operation of DFA consists of two main stages: the
compilation stage and the runtime stage. Accordingly, the
performance evaluation experiments are divided into two parts:

<!-- page 10 -->

10

TABLE IV
I NPUT S TRING DATA

Type

Size

IXIA HTTP Packets

122MB

Volunteer Packets

29MB

Alexa non-HTTP Packets

4MB

Random Byte Sequences

988KB

compilation time comparison and runtime throughput comparison.
We select two DFA models for comparison: Mcclellan [26],
the fastest traditional DFA model in Hyperscan, and PCRE2DFA [27], a widely used DFA model in DPI applications.
Both Mcclellan and Hyperflex are implemented within Hyperscan for experimentation. Hyperscan facilitates parallel regular
expression matching by partitioning a large DFA graph into
multiple subgraphs, which are independently handled by the
DFA model.
Furthermore, to assess the effectiveness of the proposed
region detection algorithm, we design a random region model
as a baseline for comparison with Hyperflex. In this model,
the start state of the hyper region is randomly selected from
states near the initial DFA state, and the region expansion
process does not incorporate the test of the state stickiness
and leakiness of Hyperflex. This model is referred to as
HyperRandom.
1) Compilation Time: During the compilation stage, DFA
processing involves only the compilation of regex rules, independent of the input string. To simulate real-world scenarios,
experiments are conducted on regex rule sets of varying sizes,
ranging from 10 to 1,000 rules. The regex rules used in the
experiments are sourced from the Snort rule set.
TABLE V
C OMPILATION T IME OF D IFFERENT M ODELS (U NIT: S ECONDS )

Rules Mcclellan Hyperflex

HyperRandom PCRE2-DFA

10

0.044

0.074

0.049

0.000059

50

0.211

0.264

0.224

0.000179

100

0.430

0.612

0.570

0.000376

200

0.810

1.43

1.25

0.000783

400

1.22

1.90

1.65

0.00144

600

2.21

2.51

2.35

0.00262

800

2.43

3.40

2.75

0.00326

1000

2.77

3.73

3.49

0.00403

The experimental results presented in Table V indicate that
the compilation time of Hyperflex is slightly higher than that of
Mcclellan and the randomly selected region model, HyperRandom. However, all three models exhibit significantly longer
compilation times compared to PCRE2-DFA. This is primarily
due to Hyperflex’s complex hyper region detection process
during compilation, including tests for state stickiness and

region leakiness. In contrast, Mcclellan does not incorporate
region detection, and HyperRandom employs a simpler region
detection algorithm, resulting in shorter compilation time.
Nevertheless, all three models perform state minimization,
whereas PCRE2-DFA adopts a more straightforward compilation process without extensive state compression or optimization, thereby substantially reducing its compilation time.
Consequently, it exhibits inferior performance in subsequent
throughput evaluations.
In practical DPI applications, the system primarily deals
with large-scale network traffic, and compilation occurs only
once. Consequently, the key factor determining overall system
performance is runtime throughput rather than compilation
time. As demonstrated in the subsequent throughput comparison, although Hyperflex incurs a slight increase in compilation
time, its runtime performance gains more than compensate
for this overhead. Its throughput optimization is particularly
valuable in large-scale network traffic detection scenarios.
2) Throughput: To comprehensively evaluate the runtime
performance of Hyperflex, four sets of experiments are conducted using four different types of input strings, as detailed
in Table IV. These experiments are performed alongside regex
rules sourced from the Snort rule set, with the number of rules
ranging from 10 to 1,000. The experimental results, presented
in Fig.s 13(a), 13(b), 13(c), and 13(d), consistently demonstrate that Hyperflex achieves higher throughput compared
to Mcclellan, PCRE2-DFA, and HyperRandom in almost all
cases.
Additionally, the match rate remains consistent across all
models. For the random byte sequences input, the match
rate is significantly lower. For the other three types of input
strings, the match rate increases as the number of rules grows,
aligning with expectations and further validating the accuracy
of Hyperflex in regex matching.
In Fig. 13(a), Hyperflex achieves 1.32x ∼ 2.13x performance of Mcclellan, 1.23x ∼ 1.66x performance of HyperRandom, and 3.39x ∼ 91.25x performance of PCRE2-DFA. In
Fig. 13(b), Hyperflex achieves 1.24x ∼ 1.89x performance of
Mcclellan, 1.34x ∼ 1.64x performance of HyperRandom, and
2.49x ∼ 111.81x performance of PCRE2-DFA. In Fig. 13(c),
Hyperflex achieves 1.17x ∼ 1.69x performance of Mcclellan,
1.07x ∼ 1.33x performance of HyperRandom, and 2.26x ∼
71.54x performance of PCRE2-DFA. In Fig. 13(d), Hyperflex
achieves 1.21x ∼ 2.27x performance of Mcclellan, 1.21x ∼
1.49x performance of HyperRandom, and 3.17x ∼ 86.31x
performance of PCRE2-DFA.
Hyperflex, Mcclellan, and HyperRandom all support parallel
regex matching, while PCRE2-DFA only supports serial rule
matching. Therefore, as the number of rules increases, the performance of PCRE2-DFA significantly lags behind the other
models. The experimental results demonstrate that Hyperflex
exhibits a clear performance advantage in regular expression
matching, making it suitable for high-throughput applications
such as DPI.
D. Parameter Selection
The selection of key parameters, including the state stickiness threshold σ, the region leakiness threshold λ, and the

<!-- page 11 -->

11

100

200

400

600

800

Rules

1000

80

3000

60

2000

40

20

1000

20

0

0
0

200

(a) IXIA HTTP Packets
180

HyperRandom
Match Rate

140
120

5000

100

4000

80

3000

60

2000

40

1000

20

0
0

200

400

600

800

Rules

800

Rules

1000

Hyperflex
Mcclellan
PCRE2-DFA

10000

160

Matches/Mbit

Throughput (Mbit/s)

6000

600

0

1000

8000

160

HyperRandom
Match Rate

140
120

Matches/Mbit

Hyperflex
Mcclellan
PCRE2-DFA

7000

400

(b) Alexa non-HTTP Packets

Throughput (Mbit/s)

8000

120

4000

40

0

6000

140
100

60

2000

7000

160

HyperRandom
Match Rate

5000

80

4000

Hyperflex
Mcclellan
PCRE2-DFA

Matches/Mbit

120

6000

0

8000

140

Matches/Mbit

Throughput (Mbit/s)

8000

160

HyperRandom
Match Rate

Throughput (Mbit/s)

Hyperflex
Mcclellan
PCRE2-DFA

100

6000

80

4000

60
40

2000

20

0

0

0

200

(c) Volunteer Packets

400

600

Rules

800

1000

0

(d) Random Byte Sequences

Fig. 13. The throughput performance of different models among 4 types of strings

3000

7000

2000

3000
2000
1000

0.1

9

8

0.0

7

0.0

0.0

6

5

0.0

0.0

4

0

0.0

1) State Stickiness Threshold σ: State stickiness measures
the propensity of states within a region to loop back to
themselves during transitions. In the hyper region detection
process, a strongly connected component is accepted only
if its stickiness exceeds a predefined threshold; otherwise,
the system checks other components. If no component meets
this threshold, no hyper region is formed. Although higher
thresholds make establishing hyper regions more difficult, once
established, they promote frequent internal loops that better

4000

3

Fig. 14. Throughput comparison of Hyperflex under different σ values

5000

2

60

55

State Stickiness Threshold( )

50

45

40

35

30

25

20

15

5

10

0

0

50 Rules
200 Rules
600 Rules

0.0

1000

6000

0.0

4000

1

5000

Throughput (Mbit/s)

Throughput (Mbit/s)

6000

0.0

50 Rules
200 Rules
600 Rules

7000

leverage SIMD-based parallel state transitions.
As shown in Fig. 14, when σ < 20, the accepted strongly
connected components exhibit low quality, resulting in suboptimal throughput. Conversely, when σ > 35, the threshold is
excessively high, preventing some regex rules from forming
hyper region and degrading performance. Throughput is optimal when σ ranges from 20 to 35. Specifically, while σ = 30
performs slightly worse than σ = 35 under 50 regex rules, it
achieves better results with larger sets of 200 and 600 rules.
Given that practical DPI scenarios often involve large-scale
rule sets, we set the threshold to 30 by default.

0.0

number of characters per batch l for parallel state transitions, is
based on experimentation. These parameters play a crucial role
in determining the throughput of Hyperflex. The parameter
tuning experiments use IXIA HTTP Packets as the input string
and three different sets of regex rule sets of varying sizes.

Fig. 15. Throughput comparison of Hyperflex under different λ values

2) Leakage Probability Threshold λ: The region leakiness
threshold λ is used to evaluate the quality of the candidate
hyper region. It determines whether a candidate hyper region

<!-- page 12 -->

12

is accepted and processed using the SIMD-based parallel state
transition algorithm or rejected in favor of the traditional
DFA state transition algorithm. When the threshold is set
too low, only a few regions can be accepted as accelerated
regions. Conversely, if the threshold is set too high, low-quality
regions may also be accepted. Therefore, it is necessary to
experimentally determine an appropriate threshold.
As shown in Fig. 15, Hyperflex performs poorly when
λ = 0, because no candidate regions pass the leakiness
evaluation, preventing any benefit from SIMD-based parallel
state transitions. As λ increases, more candidate accelerated regions meet the quality criteria, thereby improving throughput.
However, once λ exceeds 0.05, overall throughput declines due
to the acceptance of low-quality regions, whose frequent exits
from hyper region introduce time overhead that negatively
impacts overall performance. Experimental results indicate
that λ = 0.05 optimizes Hyperflex’s performance across three
different scales of regex rule sets.
50 Rules
200 Rules
600 Rules

8000
7000

Hyperflex-SIMD identification algorithm
Hyperflex-traditional identification algorithm

7000
6000

6000

Throughput(Mbit/s)

Throughput (Mbit/s)

The SIMD-based earliest boundary violation state identification algorithm proposed in Part V, referred to as
the“SIMD identification algorithm”, demonstrates significant
performance improvements over the “traditional identification
algorithm”. The latter sequentially compares multiple state
vectors with boundary vectors to determine the earliest escape
state.
To evaluate the contribution of this optimized algorithm
to the overall throughput of Hyperflex, experiments were
conducted using IXIA packets as the input string, with regex
rule sets of 50, 200, and 600 rules. The experimental results,
as shown in Fig. 17, indicate that Hyperflex using the SIMD
identification algorithm outperforms the version using the
traditional identification algorithm, with an average throughput
improvement of 7.5%. This result demonstrates that the SIMD
identification algorithm effectively reduces the time required
to identify the earliest state that escapes from the hyper region
and enhances the overall matching efficiency of the model.

5000
4000
3000
2000
1000

5000
4000
3000
2000
1000

15

14

13

12

11

l

10

9

8

7

6

5

4

3

0

Fig. 16. Throughput comparison of Hyperflex under different l values

3) Number of Characters per Batch l for Parallel State
Transitions: The value of l not only affects the number of
calls to the region identification algorithm but also the number
of state transitions that need to be re-executed when a state of
the batch escapes hyper region. As shown in Fig. 16,when
l increases, the number of characters processed per batch
increases, reducing the number of calls to the region identification algorithm, which is beneficial for overall performance.
However, as l continues to grow, the computational overhead
from re-executing state transitions after the earliest state escapes hyper region increases, leading to a more significant
negative impact on performance. For the three different scales
of regex rule sets, Hyperflex achieves optimal performance
when l = 9.
E. Performance Comparison of the Earliest Escaping State
Identification Algorithm
Based on theoretical analysis and experimental results,
the primary performance improvement of Hyperflex stems
from replacing traditional state transitions with the SIMDbased parallel state transition algorithm within accelerated
regions. Additionally, the careful design of SIMD instructions
to replace simple program statements has also contributed to
performance enhancements.

0

50

200

600

Fig. 17. Throughput comparison of Hyperflex using earliest escaping state
identification algorithm

VII. C ONCLUSION
In this paper, we propose Hyperflex, a highly-optimized,
SIMD-based DFA model, which achieves a significant improvement in processing speed compared to the traditional
DFA model. It employs a novel region detection algorithm
to identify the region that is most suitable for employing
the SIMD-based state transition algorithm of the whole DFA
graph. Furthermore, it introduces an efficient hybrid transition
algorithm to make different regions under different algorithms
and ensure seamless state transitions across different regex
regions. We implemented this model on commodity CPU and
conducted experiments against real-world network traffic and
DPI regex rules. Our comparative analysis with Mcclellan,
Hyperscan’s traditional DFA matching model, demonstrates
that Hyperflex achieves a throughput of 8.89Gbit/s, up to 2.27x
compared with the performance of Mcclellan. This model has
been successfully integrated into Hyperscan.
ACKNOWLEDGMENTS
This work was supported by National Key R&D Program
of China under Grant No. 2022YFB3102902.

<!-- page 13 -->

13

R EFERENCES
[1] T. Bujlow, V. Carela-Español, and P. Barlet-Ros, “Independent comparison of popular dpi tools for traffic classification,” Comput. Netw., vol. 76,
pp. 75–89, 2015.
[2] J. Sherry, C. Lan, R. A. Popa, and S. Ratnasamy, “Blindbox: Deep packet
inspection over encrypted traffic,” in Proc. ACM SIGCOMM, 2015, pp.
213–226.
[3] Z. Trabelsi, S. Zeidan, and M. M. Masud, “Network packet filtering and
deep packet inspection hybrid mechanism for ids early packet matching,”
in Proc. 30th IEEE Int. Conf. Adv. Inf. Netw. Appl. IEEE, 2016, pp.
808–815.
[4] M. Çelebi, A. Özbilen, and U. Yavanoğlu, “A comprehensive survey on
deep packet inspection for advanced network traffic analysis: issues and
challenges,” Niğde Ömer Halisdemir Üniv. Mühendislik Bilim. Derg.,
vol. 12, no. 1, pp. 1–29, 2023.
[5] Snort intrusion detection system. [Online]. Available: https://snort.org/
[6] Suricata: Open source ids. [Online]. Available: http://suricata-ids.org/
[7] M. A. Jamshed, J. Lee, S. Moon, I. Yun, D. Kim, S. Lee, Y. Yi, and
K. Park, “Kargus: a highly-scalable software-based intrusion detection
system,” in Proc. 2012 ACM Conf. Comput. Commun. Secur., 2012, pp.
317–328.
[8] Modsecurity: Open source web application firewall. [Online]. Available:
https://github.com/SpiderLabs/ModSecurity/
[9] Shorewall: The shoreline firewall. [Online]. Available: http://shorewall.
net/
[10] R. A. Muzaki, O. C. Briliyant, M. A. Hasditama, and H. Ritchi,
“Improving security of web-based application using modsecurity and
reverse proxy in web application firewall,” in Proc. 2020 Int. Workshop
Big Data Inf. Secur. IEEE, 2020, pp. 85–90.
[11] K. Qiu, H. Chang, Y. Wang, X. Yu, W. Zhu, Y. Liu, J. Ma, W. Li, X. Liu,
and S. Dai, “Traffic analytics development kits (tadk): Enable realtime ai inference in networking apps,” in 2022 Thirteenth International
Conference on Ubiquitous and Future Networks (ICUFN). IEEE, 2022,
pp. 392–398.
[12] C. Xu, S. Chen, J. Su, S. M. Yiu, and L. C. K. Hui, “A survey on
regular expression matching for deep packet inspection: Applications,
algorithms, and hardware platforms,” IEEE Commun. Surv. Tutor.,
vol. 18, no. 4, pp. 2991–3029, 2016.
[13] E. Sitaridi, O. Polychroniou, and K. A. Ross, “Simd-accelerated regular
expression matching,” in Proc. 12th Int. Workshop Data Manag. New
Hardware, 2016, pp. 1–7.
[14] T. Bujlow, V. Carela-Español, and P. Barlet-Ros, “Independent comparison of popular dpi tools for traffic classification,” Comput. Netw., vol. 76,
pp. 75–89, 2015.
[15] H. Xu, H. Chang, K. Qiu, Y. Hong, W. Zhu, X. Wang, B. Li, and
J. Zhao, “Accelerating deep packet inspection with simd-based multiliteral matching engine,” IEEE Trans. Netw. Serv. Manage., 2024.
[16] D. Ficara, S. Giordano, G. Procissi, F. Vitucci, G. Antichi, and
A. Di Pietro, “An improved dfa for fast regular expression matching,”
ACM SIGCOMM Comput. Commun. Rev., vol. 38, no. 5, pp. 29–40,
2008.
[17] R. Sommer and V. Paxson, “Enhancing byte-level network intrusion
detection signatures with context,” in Proc. 10th ACM Conf.
Comput. Commun. Secur., Oct 2003. [Online]. Available: http:
//dx.doi.org/10.1145/948109.948145
[18] K. Thompson, “Programming techniques: Regular expression search
algorithm,” Commun. ACM, vol. 11, no. 6, pp. 419–422, 1968.
[19] F. Yu, Z. Chen, Y. Diao, T. V. Lakshman, and R. H. Katz, “Fast
and memory-efficient regular expression matching for deep packet
inspection,” in Proc. ACM/IEEE Symp. Archit. Netw. Commun. Syst.,
2006, pp. 93–102.
[20] H. Xu, H. Chang, W. Zhu, Y. Hong, G. Langdale, K. Qiu, and J. Zhao,
“Harry: A scalable simd-based multi-literal pattern matching engine for
deep packet inspection,” in Proc. IEEE INT. Conf. Comput. Commun.
IEEE, 2023, pp. 1–10.
[21] Accton Technology Corporation. (2023) The new world of
400 gbps ethernet. [Online]. Available: https://www.accton.com/
Technology-Brief/the-new-world-of-400-gbps-ethernet/
[22] W.-E. W. Group et al. Ieee standard for ethernet—amendment 10: media
access control parameters, physical layers, and management parameters
for 200 gb/s and 400 gb/s operation.
[23] S. Antonatos, K. G. Anagnostakis, and E. P. Markatos, “Generating
realistic workloads for network intrusion detection systems,” in Proc.
4th Int. Workshop Softw. Perform., 2004, pp. 207–215.

[24] A. Bremler-Barr, Y. Harchol, D. Hay, and Y. Koral, “Deep packet
inspection as a service,” in Proc. 10th ACM Int. Conf. Emerg. Netw.
Exp. Technol., 2014, pp. 271–282.
[25] Google re2. [Online]. Available: https://github.com/google/re2/
[26] X. Wang, Y. Hong, H. Chang, K. Park, G. Langdale, J. Hu, and H. Zhu,
“Hyperscan: A fast multi-pattern regex matcher for modern {CPUs},”
in Proc. NSDI, 2019, pp. 631–648.
[27] Pcre:perl compatible regular expressions. [Online]. Available: https:
//www.pcre.org/.
[28] R. T. El-Maghraby, N. M. Abd Elazim, and A. M. Bahaa-Eldin, “A
survey on deep packet inspection,” in Proc. 12th Int. Conf. Comput.
Eng. Syst. IEEE, 2017, pp. 188–197.
[29] S. Kumar, J. Turner, and J. Williams, “Advanced algorithms for fast
and scalable deep packet inspection,” in Proc. ACM/IEEE Symp. Archit.
Netw. Commun. Syst., 2006, pp. 81–92.
[30] K. Qiu, H. Chang, Y. Hong, W. Zhu, X. Wang, and B. Li, “Teddy: An
efficient simd-based literal matching engine for scalable deep packet
inspection,” in Proc. 50th Int. Conf. Parallel Process., 2021, pp. 1–11.
[31] C. E. Graves, W. Ma, X. Sheng, B. Buchanan, L. Zheng, S.-T. Lam,
X. Li, S. R. Chalamalasetti, L. Kiyama, M. Foltin, J. P. Strachan,
and M. P. Hardy, “Regular expression matching with memristor tcams
for network security,” in Proc. 14th IEEE/ACM Int. Symp. Nanoscale
Archit., 2018, p. 65–71.
[32] W. Zhu, H. Chang, Y. Hong, X. Wang, G. Langdale, K. Qiu, and
M. Zhang, “Hyperverse: A high throughput pattern matching engine for
metaverse,” in Proc. IEEE Int. Conf. Distrib. Comput. Syst. Workshops.
IEEE, 2022, pp. 248–253.
[33] E. Gribkoff, “Applications of deterministic finite automata,” UC Davis,
pp. 1–9, 2013.
[34] K. B. Vayadande, P. Sheth, A. Shelke, V. Patil, S. Shevate, and
C. Sawakare, “Simulation and testing of deterministic finite automata
machine,” Int. J. Comput. Sci. Eng., vol. 10, no. 1, pp. 13–17, 2022.
[35] H. Fernau, P. Heggernes, and Y. Villanger, “A multi-parameter analysis
of hard problems on deterministic finite automata,” J. Comput. Syst. Sci.,
vol. 81, no. 4, pp. 747–765, 2015.
[36] D. Ron and R. Rubinfeld, “Learning fallible deterministic finite automata,” Mach. Learn., vol. 18, pp. 149–185, 1995.
[37] J. Holub and S. Štekr, “On parallel implementations of deterministic
finite automata,” in Proc. Int. Conf. Implement. Appl. Autom. Springer,
2009, pp. 54–64.
[38] D. Sidler, Z. István, M. Owaida, and G. Alonso, “Accelerating pattern
matching queries in hybrid cpu-fpga architectures,” in Proc. ACM Int.
Conf. Manag. Data (SIGMOD), 2017, pp. 403–415.
[39] Y.-H. Yang and V. Prasanna, “High-performance and compact architecture for regular expression matching on fpga,” IEEE Trans. Comput.,
vol. 61, no. 7, pp. 1013–1025, 2012.
[40] L. Vespa, N. Weng, and R. Ramaswamy, “Ms-dfa: Multiple-stride pattern
matching for scalable deep packet inspection,” Comput. J., vol. 54, no. 2,
pp. 285–303, 2011.
[41] E. Sadredini, R. Rahimi, M. Lenjani, M. Stan, and K. Skadron, “Impala:
Algorithm/architecture co-design for in-memory multi-stride pattern
matching,” in Proc. IEEE Int. Symp. High Perform. Comput. Archit.,
2020, pp. 86–98.
[42] E. Sadredini, D. Guo, C. Bo, R. Rahimi, K. Skadron, and H. Wang,
“A scalable solution for rule-based part-of-speech tagging on novel
hardware accelerators,” in Proc. ACM SIGKDD Int. Conf. Knowl. Discov.
Data Mining, 2018, p. 665–674.
[43] C. Clark and D. Schimmel, “Scalable pattern matching for high speed
networks,” in Proc. 12th IEEE Symp. Field-Programmable Custom
Comput. Mach., 2004, pp. 249–257.
[44] R. Sidhu and V. Prasanna, “Fast regular expression matching using
fpgas,” in 9th IEEE Symp. Field-Programmable Custom Comput. Mach.
(FCCM’01), 2001, pp. 227–238.
[45] W. Lin, Y. Tang, B. Liu, D. Pao, and X. Wang, “Compact dfa structure
for multiple regular expressions matching,” in Proc. 2009 IEEE Int.
Conf. Commun. IEEE, 2009, pp. 1–5.
[46] G. Vasiliadis, M. Polychronakis, S. Antonatos, E. P. Markatos, and
S. Ioannidis, “Regular expression matching on graphics hardware for
intrusion detection,” in Proc. 12th Int. Symp. Recent Adv. Intrusion
Detect. Springer, 2009, pp. 265–283.
[47] C. R. Meiners, A. X. Liu, E. Torng, and J. Patel, “Split: Optimizing
space, power, and throughput for tcam-based classification,” in Proc.
2011 ACM/IEEE 7th Symp. Archit. Netw. Commun. Syst. IEEE, 2011,
pp. 200–210.
[48] S. Kong, R. Smith, and C. Estan, “Efficient signature matching with
multiple alphabet compression tables,” in Proc. 4th Int. Conf. Secur.
Privacy Commun. Netw., 2008, pp. 1–10.

<!-- page 14 -->

14

[49] M. Becchi and P. Crowley, “A-dfa: A time-and space-efficient dfa
compression algorithm for fast regular expression evaluation,” ACM
Trans. Archit. Code Optim., vol. 10, no. 1, pp. 1–26, 2013.
[50] D. Kusswurm, “Simd fundamentals,” in Mod. Parallel Program. with
C++ and Assem. Lang.: X86 SIMD Dev. Using AVX, AVX2, and AVX512. Springer, 2022, pp. 1–16.
[51] J. Zhou and K. A. Ross, “Implementing database operations using simd
instructions,” in Proc. 2002 ACM SIGMOD Int. Conf. Manage. Data,
2002, pp. 145–156.
[52] Snort emerging threats rules v2.9.0. [Online]. Available: https:
//www.snort.org/downloads

Yang Hong received his B.S. Degree from Nanjing
University in 2014, and received the M.E. Degree
from Shanghai Jiao Tong University in 2017. He
now works for Intel as a software engineer. His work
interests include SIMD algorithm optimization and
pattern matching.

Yang Liu received his bachelor’s degree in mechanical engineering from Shanghai Jiao Tong University,
Shanghai, China, in 2021. He is currently working
toward the master’s degree with the the School
of Computer Science, Fudan University, Shanghai, China. His research interests include softwaredefined networking and deep packet inspection.
Geoff Langdale received his Ph.D. degree in Computer Science from Carnegie Mellon University. He
served as the chief architect of Hyperscan, a highperformance regular expression matching engine. He
is interested in writing code at the lowest architectural level and trying to pack every cycle with useful
work.

Wenjun Zhu received his master’s Degree from
Nanjing University of Posts and Telecommunications in 2020. He works for Intel as a software
engineer from 2020 to 2023. Now he joined Fudan
University in 2023 as a software engineer. His
research interests include computer networks and
computer architecture.

Harry Chang received his B.Eng. Degree from
Shanghai Jiao Tong University in 2006. He now
works for Intel as a senior software engineer. His
research/work interests include computer network
and performance optimization on Intel architecture.

Kun Qiu received his B.Sc. Degree from Fudan
University in 2013, and received his Ph.D. Degree
from Fudan University in 2019. He works for Intel
as a software engineer from 2019 to 2023. Now
he joined Fudan University in 2023 as an assistant
professor. His research interests include computer
networks and computer architecture. He is a senior
member of IEEE, CCF and member of ACM.

Jin Zhao received the B.Eng. degree in computer
communications from Nanjing University of Posts
and Telecommunications, China, in 2001, and the
Ph.D. degree in computer science from Nanjing University, China, in 2006. He joined Fudan University
in 2006. His research interests include software defined networking and distributed machine learning.
He is a senior member of IEEE.
