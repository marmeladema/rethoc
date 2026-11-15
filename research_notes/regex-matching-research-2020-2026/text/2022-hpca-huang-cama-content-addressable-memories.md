---
file: ../pdfs/2022-hpca-huang-cama-content-addressable-memories.pdf
citation: "CAMA: Energy and Memory Efficient Automata Processing in Content-Addressable Memories. Huang, Chen, Li, Yang. HPCA 2022"
doi: 
source: "https://arxiv.org/pdf/2112.00267"
topics: [performance_hardware]
pages: 13
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

CAMA: Energy and Memory Efficient Automata
Processing in Content-Addressable Memories
Yi Huang∗ , Zhiyu Chen∗ , Dai Li and Kaiyuan Yang

arXiv:2112.00267v1 [cs.AR] 1 Dec 2021

Department of Electrical and Computer Engineering, Rice University, Houston TX

of NFA that can be stored in a given amount of memory or
chip footprint) are always highly desired in both cloud and
embedded applications.
It is well known that finite automata processing, like many
other data- and memory-intensive computing problems, requires
frequent, yet irregular and unpredictable memory access on
general-purpose processors, leading to limited throughput and
high power consumption on modern CPU and GPGPU architectures [15], [17], [34]. FPGA solutions offer much higher speed,
but are often bottlenecked by the severe routing congestions
when dealing with a large number of complex patterns [22],
[37]. Even in application-specific digital accelerators, the
memory access challenges limit the total number of states
and transitions that can be processed in parallel, ultimately
limiting their throughput and efficiency [18], [30].
To circumvent these challenges, in-memory architectures are
increasingly being explored. Directly processing the NFA states
and transitions inside memories enjoys huge memory access
and computing parallelism. It also lowers energy by combining
memory access with logic operations. For instance, the seminal
Automata Processor (AP) design [10], [35] outperform x86
CPUs by 256×, GPGPUs by 32×, and accelerator XeonPhi by
62× in the ANMLZoo benchmark suite [29], [34], while Cache
Automaton (CA) [29] achieves 3.9× speedup over HARE [13]
and 3× speedup over UAP [11].
Existing in-memory automata processors all employ the bit
vector representation of the labels of states, i.e. the symbol
classes accepted by each state, enabling state matching in
standard RAMs. However, across real-world and synthesized
benchmarks [4], [34], it is found that only 3% of total
I. I NTRODUCTION
processing resources are utilized in 86% of the time on
Real-time stream processing plays an increasingly important average [24]. Thus, in most scenarios, the 256-bit vector
role in the era of big data and the internet of things. Non- representation for state matching is inefficient in memory usage,
deterministic Finite Automaton (NFA) is a popular computation which also leads to performance and energy penalties due to the
model to perform complex pattern matching described by spec- larger physical dimension of memories. E. Sadredini et al. [24]
ification languages like regular expressions. NFA has shown proposed a high-throughput SRAM-based design, Impala, to
promising applications in network security [16], [20], [31], tackle this particular issue by using two 16×256 6T SRAM
[39], data mining [28], [36], natural language processing [25], banks to function like one 256×256 6T SRAM in CA. However,
bioinformatics [7], [23], finance [1], machine learning [8], this scheme consumes more energy because of the doubled
and runtime verification of cyber-physical and embedded memory peripheral, which is the dominant source of energy.
In this work, we take an orthogonal approach and propose the
systems [3]. These applications demand high-speed processing
of large and complex NFAs on real-time data collected from first Content Addressable Memory (CAM) enabled Automata
sensors, networks, and various system traces. In addition, high accelerator, namely CAMA, whose memory and energy usage
energy efficiency and memory efficiency (defined as the size scale down with the average symbol class size of a given
NFA, without any performance overhead. CAM is naturally
* These authors contributed equally to this work
suitable for fixed-width bit string matching [19], which has

Abstract—Accelerating finite automata processing is critical
for advancing real-time analytic in pattern matching, data
mining, bioinformatics, intrusion detection, and machine learning.
Recent in-memory automata accelerators leveraging SRAMs and
DRAMs have shown exciting improvements over conventional
digital designs. However, the bit-vector representation of state
transitions used by all state-of-the-art (SOTA) designs is only
optimal in processing worst-case completely random patterns,
while a significant amount of memory and energy is wasted in
running most real-world benchmarks.
We present CAMA, a Content-Addressable Memory (CAM)
enabled Automata accelerator for processing homogeneous nondeterministic finite automata (NFA). A radically different state
representation scheme, along with co-designed novel circuits and
data encoding schemes, greatly reduces energy, memory, and chip
area for most realistic NFAs. CAMA is holistically optimized
with the following major contributions: (1) a 16×256 8-transistor
(8T) CAM array for state matching, replacing the 256×256 6T
SRAM array or two 16×256 6T SRAM banks in state-of-theart (SOTA) designs; (2) a novel encoding scheme that enables
content searching within 8T SRAMs and adapts to different
applications; (3) a reconfigurable and scalable architecture that
improves efficiency on all tested benchmarks, without losing
support for any NFA that’s compatible with SOTA designs; (4) an
optimization framework that automates the choice of encoding
schemes and maps a given NFA to the proposed hardware.
Two versions of CAMA, one optimized for energy (CAMA-E)
and the other for throughput (CAMA-T), are comprehensively
evaluated in a 28nm CMOS process, and across 21 real-world
and synthetic benchmarks. CAMA-E achieves 2.1×, 2.8×, and
2.04× lower energy than CA, 2-stride Impala, and eAP. CAMA-T
shows 2.68×, 3.87× and 2.62× higher average compute density
than 2-stride Impala, CA, and eAP. Both versions reduce the
chip area required for the largest tested benchmark by 2.48×
over CA, 1.91× over 2-stride Impala, and 1.78× over eAP.

1

<!-- page 2 -->

been extensively used in networking for IP lookup and packet
grammable symbol classes at greatly reduced area and
classification. Because CAMs are built purely with CMOS
energy costs. This is made possible by the proposed
devices, it also enjoys perfect scalability to cutting-edge
data encoding and symbol class compression methods.
CMOS process nodes [38]. However, existing CAM-based
All CAM entries will be encoded when storing into the
hardware for complex pattern matching mostly implement
memory, while the encoding of inputs will be performed
Deterministic Finite Automaton (DFAs) [21] constructed from
online with a 256×32 SRAM-based encoder. The encoder
either AC algorithm or regular expressions [2], [9], [40].
only occupies less than 0.11% energy on average.
They cannot handle many complex applications beyond packet
• We develop a reconfigurable local crossbar switch with
inspection, because of the well-known state explosion issues
8T SRAMs for state transition in CAMA, which features
when converting an expressive NFA to a DFA [10], [35]. To
a simpler and smaller physical implementation than prior
the best of our knowledge, CAMA is the first CAM-based
arts. It can be configured as an RCB for most applications
hardware targeting generic NFA processing.
or a full crossbar (FCB) for dense NFAs, in order to
In CAMA, CAMs perform state matching by directly
maintain generalizability to any NFA.
comparing the input symbol with the symbol classes of all
• We create an optimization framework that automatically
enabled states, instead of converting the input symbol to a bit
analyzes the homogenous NFA in an MNRL/ANML file,
vector representation for comparison like in RAMs with singleand chooses the optimal encoding scheme, the code length,
row access. This change apparently results in a much narrower
and the CAMA operation mode. The toolchain also maps
memory bit-width, but considering CAM’s larger footprint and
the optimized NFA to the hardware.
access energy than SRAMs, the remaining design challenges
• Two versions of CAMA designs in the 28nm CMOS
are to reduce the average number of CAM entries for each state
process, one optimized for energy (CAMA-E) and the
and the overall hardware footprint and energy. Thus, we propose
other for throughput (CAMA-T), are rigorously evaluated
tightly coupled optimizations across circuits, architecture, and
across 21 common real-world and synthetic benchmarks.
algorithms to (1) re-purpose standard 8-transistor (8T) SRAMs
Compared to SOTA designs, CAMA-E achieves > 2×
for state matching, which is made possible by a specially
lower energy, while CAMA-T shows > 2.6× higher
crafted data encoding scheme, (2) adapt the CAM bit-width to
compute density (i.e. throughput per unit area). Both
NFAs with different sparsity, and (3) compresses multiple
CAMA versions reduce the area required for the largest
symbols into a single CAM entry. With these co-designs,
benchmark by 1.91×, 1.78× and 2.48× over 2-stride
CAMA increases the total number of CAM entries by merely
Impala, eAP, and CA.
13% but reduces the overall state matching memory area by
II. BACKGROUND
3.6× over CA, a SOTA in-SRAM automata accelerator.
A.
Non-Deterministic
Finite Automata (NFA)
On the other hand, state transition is another critical step to
optimize. A. Subramaniyan et al. [29] proposed the use of 8T
A Non-deterministic Finite Automaton (NFA) is defined as a
SRAMs as fully connected crossbar interconnects, overcoming 5-tuple (Q,Σ,δ ,q0 ,F), in which the Q represents the set of states,
the limited flexibility of hardwired interconnect in AP. 8T Σ is a set of symbols, also called the input symbol alphabet, q0
SRAMs support logic OR operations among multiple activated means the set of start states, F represents the set of reporting
rows, without destructing stored data. Later, E. Sadredini et states and δ is defined as the transition function. The transition
al. [27] observed that most applications have sparse state function δ (Q, α) gives out the set of states which are the next
transitions in the shape of a diagonal matrix, and propose states starting from Q on input α. Compared to deterministic
the Reduced CrossBar (RCB) to save area. Inspired by RCB, FA (DFA), the NFA holds multiple active states at the same
CAMA employs a 128×128 compact local switch compatible time and supports multiple transitions on a single input.
with the CAM-based state matching. The proposed local switch
We adopt the ANML-NFA [12], [34], or homogenous NFA,
has a simpler and smaller physical implementation than that as our automaton model. A homogenous NFA is hardwarein eAP [27], and supports a novel reconfiguration scheme to friendly and used by all SOTA designs [10], [24], [27], [29].
be converted into a full crossbar (FCB) mode, which provides Algorithms to convert a classical NFA into a hardware-friendly
sufficient routing resources for any NFA.
ANML-NFA have been well studied. Figure 1(a) shows the
In summary, this paper makes the following contributions: classical NFA and the ANML-NFA for regular expression
• We propose CAMA, the first CAM-enabled, low-area, (a|b)e∗ cd + . The states in the ANML-NFA are often called
and energy-efficient automata processor for homogenous state transition elements (STEs). Here the NFA alphabet is
NFA in streaming processing, which replaces the bit {a, b, c, d, e} and the states or STEs with a triangle on them
vector representation in SOTA RAM-based designs by are start states. Any state can be converted to a start state by
direct symbol matching in CAMs. CAMA is holistically enabling it on every input. States with double circles denote
optimized across circuits, architecture, and algorithms, in reporting/closing states in NFAs.
order to improve energy, area, and compute density while
B. In-Memory Automata Accelerator
maintaining similar programmability as prior arts.
• We re-purpose 8T SRAM arrays to behave like 16T
In-memory automata accelerators are promising hardware
Ternary CAMs and perform state matching on pro- solutions for general-purpose high-speed and high-efficiency

2

<!-- page 3 -->

e
a, b

d

c

d

STE0

STE3

STE1

C. Content-Addressable Memories (CAM)

STE2

Binary CAM (BCAM) [19]: Figure 2(a) shows the design
of
a standard 10-transistor (10T) NOR-type BCAM. The two
(a)
ANML NFA
pull-down paths from the match line (ML) to ground, formed
STE0 STE1 STE2 STE3
1 1
EN[0:3]
by M1 through M4, implement the logic XNOR between the
0
0
0
0 1
0
0
0
pre-charge
0 0
input search data on the differential search lines (SL1/SL2)
0
1
0
8b input
256
ML[3] ML[2] ML[1] ML[0]
0 0
0
0
1
rows
0 0
and the differential stored data (D1/D2) in the 6T SRAM
1
0
0
16b
encoded
cell. Notice that bitlines (BLs), wordlines (WLs), and two
8b
input
0 0
input
16b
0
0
0
SLs
access transistors for normal SRAM read/write are omitted
Match Vector
EN[0:3] EN[0] EN[1] EN[2] EN[3]
for simplicity. A mismatch between D1/D2 and SL1/SL2 will
pull down the precharged ML through one of the NMOS pairs,
STE0 STE1 STE2 STE3
STE3 STE2 STE1 STE0
STE0
STE0
while
a match will disconnect both pull-down paths. When
STE1
STE1
STE2
multiple cells are connected to a single ML to form a CAM
STE2
STE3
STE3
word, all pull-down paths effectively perform the logic NOR
(c)
(b)
operation. ML keeps the precharged state (i.e. the matching
Fig. 1. (a) Classical NFA and ANML-NFA; and principles of (b) CA, a
representative in-memory automata accelerator, and (c) the proposed CAMA condition) only if all cells are in the matching state.
Ternary CAM (TCAM) [19]: Unlike the BCAMs that only
using Two-Zeros prefix encoding.
store and search logic ‘1’ and ‘0’, TCAM cells support a
SL1
SL2
SL1
SL2
ML
ML
new logic state ‘X’ (i.e. don’t care) which matches both logic
‘1’
and logic ‘0’, and enables the so-called wildcard search.
D1
D2
D1 D2
M1
M2
M2
M1
Figure 2(b) shows the implementation of the standard 16T
M3
M4
M4
M3
NOR-type TCAM cell. Two SRAM bit cells are required to
encode the 3 logic states, where ‘01’ represents logic ‘0’, ‘10’
(a)
(b)
represents logic ‘1’, and ‘11’ is logic ‘X’. As a result, ‘X’
Fig. 2. Circuit design for conventional (a)BCAMs and (b)TCAMs.
will always disconnect the two pull-down paths and indicate a
match regardless of the input data, while searching operations
automata processing, by circumventing the “memory walls”. on ‘0’ and ‘1’ are identical to that of BCAM. In addition to
Figure 1(b) shows the working principle of CA [29], a storing ‘X’, TCAM also allows searching ‘X’ by setting both
representative design in SRAM.
SL1 and SL2 to ‘0’. Similarly, this will result in a “match”
All existing in-memory automata processors execute an output due to the disconnected pull-down NMOS (M3/M4).
automaton in two steps: state matching and state transition.
III. P RINCIPLES OF CAM-BASED AUTOMATA (CAMA)
In state matching, the static/dynamic RAM stores STEs in
columns, with bit vector representations of each STE’s symbol
In this section, we first discuss the limitation of prior works
class. Every streaming input symbol is decoded to a one-hot and then summarize how CAMA tackles the challenges.
address and activates one row in the state matching table
(the RAM). Then the columns (STEs) comprising the input A. Limitations of prior works
symbol, i.e. those storing ‘1’s in the corresponding rows, will
Limitations in State Matching. Existing in-memory auread out ‘1’s, indicating potentially active STEs in the Match tomata accelerators are all based on bit vector (one-hot
Vector. Next, in state transition step, Match Vector is first encoding) representation for state matching, which has a
logically ANDed with last cycle’s transition list (Next Vector), constant cost for each state regardless of the actual transition
to obtain the current active states. If any active state is also a symbols. This leads to inefficient memory usage and high
reporting state, the result will be returned. In the meantime, an energy in most applications/NFAs, where each state only
updated Next Vector is generated by passing the active state matches a small number of symbols. For instance, more than
vectors through a pre-programmed routing network. Figure 1(b) 86% of the states are matched against at most eight symbols
illustrates how the regex (a|b)e∗ cd + is mapped to CA [29]. across 21 benchmarks in [24], which means only 3% (≈ 8/256)
Since this regex requires four STEs, only four memory columns of the cells in memory columns are utilized. Besides, the
are utilized in the 256×256 state matching SRAM bank. A small size of the alphabet in some applications also causes
pre-programmed 4×4 routing network (connections indicated underutilization of the state matching resources in CA, e.g.,
by dots in Figure 1b), which is a small subset of the actual ExactMath [4] has only 114 symbols so that more than half
256×256 local switches implemented in 8T SRAM, updates of the SRAM rows remain unused. Impala [24] made the first
the transition list (Next Vector) based on the current active attempt to tackle this issue by splitting 8-bit symbols to 4STEs. As an example, assume STE0 is a current active STE bit symbols so that 256-bit one-hot state matching SRAMs
that has a connection with STE1 and STE2. The next input can be replaced by two 16×256 ones. However, because
symbol is ‘e’ that activates Row5. The ANDed result on STE0’s SRAM periphery occupies ≥80% of SRAM access energy,
transition (EN[3:0]) and the Match Vector determines STE1 as reading two 16×256 SRAM banks consumes more energy
a current active STE for the next cycle.
than reading a single 256×256 bank. As a result, the energy
s1

s2

s3

s4

a, b

e

c

d

Encoder

0101111111 111100

0101111111 110111

0011111111 111101

0011111111 111110

Next Vector

Next Vector

...

...

...

...

...

Decoder

Original NFA

3

<!-- page 4 -->

efficiency of Impala turns out to be lower than CA [29] that multiple symbols into a single entry. However, a conventional
uses conventional 256-bit one-hot encoding.
16-transistor TCAM cell is about three times larger than 6T
Limitations in State Transition. Conventionally, in- SRAM cells and consumes more energy, limiting the area and
memory automata architectures, such as CA and Impala, energy benefits of the proposed architecture. To alleviate these
implements state transition by a full crossbar (FCB) local limitations and boost the CAMA efficiency, we propose holistic
switch built with 8T SRAMs. The local switches connect optimizations in both circuit and algorithm levels which will
each STE to any other STEs in the same state matching bank be discussed in §IV and §V.
including itself and thus the area overhead is proportional to
N × N, where N is the STE numbers in each state matching C. State Transition in CAMA
We propose a reconfigurable reduced crossbar (RRCB)
bank (N = 256 in CA [29] and Impala [24]). However, in most
real-world applications, the NFA transitions can be clustered compatible with the CAM-based state matching, to achieve
into connected components (CCs) with no transitions between a dense switch for state transition, inspired by the RCB
different CCs. Under the breadth-first search (BFS) mapping in approach in eAP [27]. The RCB reduces the crossbar size
eAP [27], the state transition has a diagonal connectivity pattern from 256 × 256 in CA [29] to 96 × 96, by remapping the
in most applications, which means most valid transitions are sparsely and diagonally located transitions in FCB into a
located near the diagonal of the local switches. Therefore, only denser storage in RCB. To support NFAs where the transitions
a small fraction of the SRAM cells are utilized. [27] shows are too dense to fit into a single RCB (e.g., transitions in
that the average local switch utilization is merely 0.48% across EntityResolution [34]), it reuses state-matching SRAMs for
FCB state transitions. However, in the proposed CAM-based
19 benchmarks in ANMLZoo [34] and Regex [4].
architecture, CAM banks are too small to function as FCBs.
B. State Matching in CAMA
Additionally, the physical implementation of RCB in eAP
Since the memory under-utilization comes from the overlong requires three bit line (BL) segments in each column and
one-hot address, CAM is a promising alternative to reduce three word lines (WLs) in each row, leading to congested
the code length of symbols, without doubling the periphery routing in practical SRAM layout with potentially much larger
energy like in Impala [24]. In a CAM-based state matching memory footprint and larger energy. In the proposed 128×128
design, the ANML-NFA states are no longer stored in the RRCB for CAMA, transitions in the 8T SRAM crossbar are
SRAM columns and input symbols will not be decoded into evenly allocated for most applications with a different mapping
one-hot addresses to access SRAM rows. Instead, NFA states strategy than RCB (details in §IV). The RRCB requires only
are encoded based on their symbol classes and mapped into two segments of WLs and BLs, which can be implemented
CAM entries (CAM states). Here, the input symbols are directly within standard 8T SRAMs without any area overhead. It can
sent to SLs in the CAM without the need of address decoders. further be reconfigured into a 128×128 FCB by altering the
Ideally, this modification reduces the bit-width of matching connection of WLs and BLs for dense transitions. With the
tables from A to log2 A bits (A is the size of the alphabet) new reconfiguration scheme, reusing the state matching CAM
while only requires one readout periphery. Furthermore, as as FCB is no longer needed.
Figure 1(c) shows, the AND operation in the conventional
D. Working Example of CAMA
architecture can be replaced by only precharging the columns
Figure 1(c) shows an working example of the CAMA on
enabled by transitions in the CAM through controllable column
a
regular
expression (a|b)e∗ cd + . In this example, the ASCII
prechargers. This selective enabling scheme will greatly reduce
CAM energy and is adopted in the proposed energy-optimized alphabet can be encoded to 16 bits and fit into 4 columns in
CAMA’s 16×256 state matching bank with no state increase
CAMA-E.
A naı̈ve implementation of this scheme is to use Binary over bit vector representation. Unlike SOTA designs that encode
CAM (BCAM) and the ASCII encoding, which only supports 8-bit inputs into 256-bit row addresses in Figure 1(b), CAMA’s
exact string match. It requires the number of CAM entries the 8-bit input will be encoded into 16-bit input and directly sent
same as the total symbol class size for all states. For example, to the search line. Ideally, only 8-bit input code is required,
RandomForest in ANMLZoo has a larger than 100 average but we utilize 16-bit encoding to balance the CAM entries and
symbol class size. Implementing it with a BCAM implemen- the code length (which are described in greater detail in §IV
tation will need > 100 entries for each state, diminishing all and §V). As for the state transition, this example is using an
the expected benefits from using CAM. Considering the cell- FCB for simplicity, but the actual implementation of CAMA
level larger energy and area of CAMs over SRAMs, this naı̈ve is based on the proposed 128×128 RRCB design as discussed
design will only be advantageous when the symbol class size in §IV.
is very small (e.g. ≤ 3).
IV. C IRCUIT D ESIGN AND O PTIMIZATION
Thus, we will need CAMs capable of storing and matching
several symbols within a single entry, which is called com- A. State-Matching CAM Optimization
pressed symbol class. To perform such symbol searches, the
In reducing the area and energy overhead of 16T (transistor)
state matching CAM needs to operate like a Ternary CAM TCAM cells, [14] showed that two standard 6T SRAMs can be
(TCAM), where the don’t care state ‘X’ allows compression of configured for TCAM operations. We made a further innovation

4

<!-- page 5 -->

Mismatch

‘0’

‘1’

ML[0]

‘0’ 6T
Cell

‘0’ 6T
Cell

‘1’ 6T
Cell

‘0’ 6T
Cell

‘0’ 6T
Cell

‘1’ 6T
Cell

SL[0]
CAM Matching Table

SL[1]

SL[2]

‘0’ Match

Match

‘1’Mismatch

Match

STORE

Match
‘1’

(a)

‘1’
‘0’
SEARCH
42

ML[1]

43

43

42

Fig. 3. Illustration of the single port search mode in an 8T CAM.

43

TABLE I
S YMBOL C LASS AND A LPHABET S IZES WITH AND WITHOUT NEGATION
OPTIMIZATION (NO)

43

Symbol Symbol Class Alphabet # CAM Entries with # CAM Entries
Class Size Size with NO
Size
Raw Symbol Class
with NO
Brill
1
1
256
42658
42658
ClamAV
1.006
1.006
256
49593
49593
Dotstar
1.56
1.56
256
103280
103280
Fermi
7.18
4
256
53769
61066
TCP
9.26
1.28
256
32883
20156
Protomata
4.41
2.65
256
162443
69715
Snort
4.41
2.02
256
90718
72884
Hamming
1
1
256
11346
11346
PowerEN
1.95
1.09
256
48016
41080
Levenshtein
1
1
256
2784
2784
RandomForest
179.05
51.55
256
80515
75936
EntityResolution
38.14
1.41
256
111996
95550
Bro217
1.55
1.55
256
2352
2352
Dotstar03
1.92
1.3
256
14245
12445
Dotstar06
2.48
1.28
256
16536
13116
Dotstar09
3.1
1.29
256
17834
12723
Ranges1
1.29
1.29
115
12947
12947
Ranges05
1.21
1.21
107
12990
12990
SPM
89.4
1.5
256
135675
100500
BlockRings
1
1
2
44352
44352
ExactMath
1.002
1.002
114
12439
12439

22 22

20

42

43

43

43

43

42

43

42

(b)

Fig. 4. Sketch of the remapping (a) from 18×18 FCB to 9×9 RRCB and (b)
from 256×256 FCB to 128×128 RRCB.
128

Benchmark

Readout Top

Row86

128

WL Driver

RBL127

WL Driver

Bank1

RBL87
RBL86

RBL85

WL Driver

RBL44
RBL43

Bank0

RBL42

RBL1
RBL0

...

Row43

Bank2

Readout Bottom

8T cell with separated RBL

Conventional 8T cell

Cell0

Cell0

Cell1

Cell1

Diffusion
Poly
Metal1
Metal2
Metal3

RBL
RBL
that a single search line (SL) is sufficient for the search
Fig. 5. Implementation and layout of 128×128 RCB.
operations if data and input symbols are encoded to have a
fixed number of ‘1’s. Therefore, standard push-rule 8T SRAM ‘A’(01000001) will wrongly match the ASCII ‘C’(01000011).
cells can be repurposed as 8T TCAM cells, but with only
Besides, some ANML-NFAs have symbol classes defined
around 1/3 of the area [16]. As illustrated in Figure 3, the read by negation b , which can be implemented by matching the
WL (RWL) and the read BL (RBL) in the 8T SRAM cell are excluded symbols and inverting the output if this reduces the
repurposed as the SL and match line (ML) of CAM. Only when total number of CAM entries and CAM area. An inverter is
the cell stores a ‘1’ and the search line is pulled up by input added to each row of the state-matching CAM, which will
‘1’, a discharging path will be formed and report a mismatch. be enabled when a symbol class defined by negation is met.
According to pigeonhole principle, when a fixed number of ‘1’ For example, if one state’s symbol class is [ b abcd], it will
is enforced in both the stored and input symbols, any mismatch accept all the other 252 symbols except a, b, c and d and its
between the two symbols will lead to at least one discharging symbol class size is 252 but the size of its negation symbol
path and the result of mismatch. Any other types of symbols class [abcd] is 4. This is named Negation Optimization (NO).
with varied number of ‘1’s will result in reduced coding As Table I shows, NO reduces the number of CAM states in
space and more complicated code compression [16]. However, nearly all applications, especially in cases where the original
TCAM mismatch should actually happen only when storing ‘1’ symbol class size is far larger than the optimized symbol class
and searching ‘0’. This can be achieved straightforwardly by size after NO.
adding inverters to all search line inputs, but a simpler method
The column precharger at each CAM entry is implemented
in CAMA is to directly embed the inversion into SRAM- to selectively enable entries to save energy consumption. It
based input encoder without any overhead, as required by the functions as the AND gates between the transition results and
CAMA encoding scheme (see Section §V). Since the search match results during state transition. Meanwhile, the CAM
operation requires activating multiple SLs simultaneously, the bank contains a mask so that it can turn off certain search bits
proposed 8T cell with decoupled read port ensures reliable to save energy when the bit width of search data is smaller
non-destructive research while the 6T cell based CAM [14] than the CAM’s word length, i.e. the number of rows.
faces severe read disturbance issue, especially in large arrays
and advanced technology nodes. It is worth mentioning that B. Switch Network Optimization
other coding schemes like ASCII do not work on the 8T CAM
The proposed 128×128 RRCB remaps the diagonal transibecause of the special encoding match condition, e.g. the ASCII tions in the 256×256 FCB to achieve a dense data storage.

5

<!-- page 6 -->

One-Zero [10][29]

NFA States
Alphabet: a~f

111101

f

b

e

c

d

101111

110
011

111110

110
110

101
011

Two-Zeros Prefix
101
101

001
01

001
10

Prefix
110
101
010
101
110
Suffix
01
Store ab (111100)
Match a,b
Store ab (110010)
Match a,b
Store abcd (001100) Match a,b,c,d Store abc (110000) Match a,b,c Store ab (00100)
Flip for wildcard search
Better
Compression Rate
Worse
Code Length
101111

a

111011

One-Zero Prefix

011111

100
01

010
10

Multi-Zeros [4]
100
10
Prefix
Suffix

Match a,b

1010

0011

1001

0110

0101

1100

Store ab (1000)

Match a,b,c
Error!
Worse
Better

Fig. 6. Illustrations of the encoding schemes.

Figure 4(a) illustrates a toy example of the remapping where
the diagonal transitions are divided into groups in FCB. The
neighboring groups are remapped to stacking into the same
columns in RRCB to save area. To fit into the 9 × 9 RRCB,
the diagonal width kdia that each group occupies is chosen
as 3. In reality, see Figure 4(b) we choose kdia = 43 to fit
into the 128×128 RRCB and ensure only two groups are
stacked in each column. Meanwhile, the WLs are divided into
3 segments with each one receiving different inputs. As shown
in Figure 5, it requires a split RBL in each column and two
read peripheries on the top and bottom. Compared to the layout
of conventional 8T cell (see Figure 5), the proposed 8T cell
with split RBL simply cuts off the shared drain of NMOS
and will not introduce area overhead. As a comparison, eAP
chooses kdia = 21 for the 96×96 RCB, leading to 3 segments
of BLs in each column which is impractical for the physical
implementation of the readout peripheries.
For dense transitions that RCB cannot handle, RRCB can
be reconfigured into a 128×128 FCB by combining the input
and output data. The two split RBLs can be considered as one
once the readout data from the top and bottom are combined
together using OR gates. Similarly, feeding three replicas of
inputs to each WL segment is equivalent to connecting the WLs
in the same row. As a result, there is no need to reuse the statematching memory like eAP, relaxing the storage requirement
of CAM.
There exist some situations that the CCs are too large to
fit into one CAM bank. Thus, global switches are used to
provide connections between different CAM banks. We adopt
the 256×256 8T SRAM bank similar to eAP [27]. Each local
switch allows 16 STEs sending to the global switch and 16
STEs receiving from the global switch.

‘ab’ is encoded as ‘0011’ to match both symbols. Meanwhile,
the number of zeros should be fixed for all symbols with
class size equal to 1, as mentioned in §IV.A. As a result, the
unique matching condition and the encoding constraints require
specialized encoding strategies.
We observe that the most important design choice of the
encoding is the number of zeros in the codes because it
determines both the code length and the compression rate.
The One-Zero encoding [10], [29], which has fixed one ‘0’
in the codes, achieves the largest compression rate because
any combinations of the symbols can be compressed into one
code by flipping ‘1’s at any positions. As Figure 6 shows,
states ‘a’/‘b’/‘c’/‘d’ can be simply compressed into one code.
However, its code length, which is equal to the alphabet size,
is overly large and thus causes low memory utilization.
On the contrary, an equal number of ‘1’s and ‘0’s leads to
the shortest code length but the worst compression rate. This
is termed as Multi-Zeros encoding [16]. As shown in Figure 6,
a wildcard search for ‘ab’ by flipping the second ‘1’ will cause
a wrong matching of state ‘c’ due to the poor compression
ability. Quantitatively, the numbers of symbols (NS ) that can
be compressed into one
 code must follow the combinatorial
number rule: NS = mn , where m is the possible number of ‘0’s
in the code after compression while n is the number of ‘0’s
in the original code. In this example, m = 3 or 4 and n = 2,
and thus NS = 3 or 6. Therefore, the 2 (6= NS ) states ‘a’ and
‘b’ cannot be compressed into one code. Generally, the more
‘0’s one code has, the more discrete the combinatorial numbers
will be, leading to increased difficulty of compression and high
state numbers.
To address the trade-off, we combine these two encoding
schemes above and divide the code into two parts: the prefix
and the suffix (see Figure 6). The suffix follows the One-Zero
V. DATA E NCODING S CHEMES AND O PTIMIZATION
encoding style to achieve the maximum compression capability.
A. Encoding Scheme
On the other hand, the overall code length is shortened by
The goal of optimizing the encoding scheme is to achieve an adjusting the number of ‘0’s in the prefix at the cost of less
optimal trade-off between the code length and the compression compression space.
rate. Since NFA state’s symbol class size will sometimes be
The first scheme, namely Two-Zeros Prefix encoding, has
larger than 1, we need to compress multiple symbols into fixed two ‘0’s in its prefix for each symbol (see Figure 6). By
one code while maintaining moderate coding complexity. The flipping the ‘1’s in the suffix, symbols with the same prefix
matching condition of the proposed 8T CAM, as discussed can be compressed into one code. This is termed as suffix
in Section §IV.A, is as follows: while ‘1’ in the cell only compression. For example, if state ‘a’ is ‘001 01’ and ‘b’ is
matches ‘1’ inputs, ‘0’ represents a don’t care that matches ‘001 10’, state ‘ab’ will be encoded to ‘001 00’ (see Figure 6).
both ‘0’ and ‘1’ inputs. Therefore, compression is achieved Similarly, a prefix compression is also proposed to flip ‘1’s
by flipping certain ‘1’s into ‘0’s in the codes. E.g. states ‘a’ in the prefix and compress the codes with the same suffix
and ‘b’ are encoded as ‘0111’ and ‘1011’, respectively, so (not shown in the figure). As mentioned above, the prefix

6

<!-- page 7 -->

compression has more restricted compression space since two
‘0’s exist in the prefix but achieves smaller code length.
The second scheme is the One-Zero Prefix encoding where
the prefix has fixed one ‘0’. Compared to the previous scheme,
its prefix compression has better compression rate: any number
of codes within the same suffix can be compressed to one code
because there is only one ‘0’ in the prefix. For instance, if
states ‘a’/‘b’/‘c’ are encoded as ‘110 011’/‘110 110’/‘110 101’,
state ‘abc’ can be compressed into ‘110 000’ (see Figure 6).
Despite the improved compression capability, the code length
becomes larger and thus it is specialized for those applications
with large average symbol class sizes.

TABLE II
C OMPARISON OF CAMA ENCODING SCHEME WITH 256- BIT AND FIXED
32- BIT O NE -Z ERO PREFIX E NCODING WITHOUT CLUSTERING
OPTIMIZATION (M EMORY U SAGE = C ODE L ENGTH × # S TATES )
Benchmark
Brill
ClamAV
Dotstar
Fermi
TCP
Protomata
Snort
Hamming
PowerEN
Levenshtein
RandomForest
EntityResolution
Bro217
Dotstar03
Dotstar06
Dotstar09
Ranges1
Ranges05
SPM
BlockRings
ExactMath

B. Encoding Selection and Symbol Clustering
We observe that using a single encoding scheme for all
benchmarks is inefficient. E.g., using a 32-bit One-Zero prefix
encoding scheme to encode the BlockRings [34] is a waste, as
32-bit codes can express 256 symbols but BlockRings only has
2 symbols. Thus, an encoding selection algorithm is proposed
to automatically select a proper encoding scheme by aiming
to find an encoding with minimum code length and adequate
compression space to balance the number of CAM states and
the code length.
First, we calculate the alphabet size (A) of ANML/MNRL
dataset and the average symbol class size with NO (S). If S
is equal to 1, the Multi-Zeros code is directly selected as it
requires no compression. Its code length is calculated by

(1)
L = minimize{L : L
L ≥ A}

256-bit OneZero States
42658
49538
96438
40783
19704
42011
69029
11346
40513
2784
33220
95136
2312
12144
12640
12431
12464
12439
100500
44352
12439

Fixed 32-bit One-Zero
Prefix Encoding States
42658
49616
99254
43972
20200
78078
88857
11346
41511
2784
128451
139994
2312
12325
12874
13000
12645
12801
130650
44352
12451

Proposed
Code
Encoding
Length
States
11
42658
16
49593
16
103280
16
61066
16
20156
16
69715
16
72884
11
11346
16
41080
11
2784
32
75936
16
95550
16
2352
16
12445
16
13116
16
12723
13
12947
12
12990
16
100500
2
44352
16
12439

decreased demands for prefix compression. At the beginning
of the algorithm, we calculate the frequency for each symbol.
For each unfilled cluster, we estimate the possibility that the
undistributed symbols and the symbols in the cluster appear in
the same symbol class, the P(XC) (C: symbols in chosen cluster,
X: the undistributed symbol). The symbol with the highest
probability will be packed into that cluster. The clustering
operation repeats until all clusters are full.
C. Encoding Results Analysis

2

Otherwise we need choose from Two-Zeros prefix and OneZero prefix encoding schemes based on the code length. For
Two-Zeros prefix encoding, the length of the suffix is set to
be larger than the symbol class size so that more symbols can
be compressed into one √
CAM entry. By sweeping the length
of the suffix from S to A, the minimum code length Lmin
can be calculated based on A:

Lmin = min(Lmin , ls + minimize{l p : l2p × ls ≥ A})
(2)

Table II shows the proposed encoding scheme with optimization only increases the average number of states by 13%
compared to one-hot encoding. Without code selection and
clustering optimization, the 32-bit One-Zero prefix encoding
not only needs longer code length but also increases the states
by 25%. The relatively large state increasing (about 1.23×)
in RandomForest is because its symbol class size with NO is
more than 50 (Table I). Overall, the proposed encoding scheme
and optimization algorithm successfully map all benchmarks
where ls and l p are the length of suffix and prefix, respectively. into CAMA without significance state increases, leading to the
For example, given S = 5 and A = 256, the length of Two-Zeros area and energy efficiency boosts.
prefix encoding is 16b because when the suffix length ls is set
The worst case of state increasing
will only occur in One√
from 5 to 16, the minimal L that satisfies Cl2p × ls ≥ 256 is 16. Zero prefix encoding with 2 A code length because the worst
√
For One-Zero prefix encoding, the shortest code length is 2 A case is supposed to have the maximum number of CAM entries
according
to Cauchy’s inequality. Therefore, by comparing with large symbol class size. Theoretically, the upper bound of
√
2 A and Lmin , the encoding with smaller code length will One-Zero prefix for one state is 2A memory bits (=number of
be selected. Specially, if the alphabet is small enough, which entries×number of bits in one state) in state matching which
means all states can be directly stored into one CAM entry, only occurs when any two symbols in a symbol class have
the One-Zero encoding will be adopted.
different suffixes and prefixes. However, having a few such
After the encoding scheme is determined, the symbol symbol classes will trigger re-clustering of symbols, unless in
clustering algorithm groups the symbols that have a high the rare case where the distribution of symbols and symbol
probability to occur in the same symbol class into one cluster classes is truly random. Overall, it is unlikely to reach the
with the same prefix (frequency-first), leading to the reduced worst case that requires 2A memory bits. For example, the
number of CAM states. Meanwhile, this algorithm ensures most maximum average number of state matching memory bits for
of the symbols can be compressed with suffix compression, one state is only 73 among our benchmarks, which happens
which simplifies the compression procedure because of the in RandomForest with dense transitions.

7

<!-- page 8 -->

In the 16-bit FCB mode, the system utilizes an encoding
scheme whose code length is less than 16-bit but the local
Local
switches in RCB mode is not able to store all the transitions.
Switch
16x256 CAM
128x128 Local Switch
Each tile turns off one of the CAM sub-arrays by power-gating
Local
Switch
and connects the two local switches to the other sub-array
Next V.
as shown in the Mode 2 in Figure 7. The working CAM is
Mode 2: 16bits FCB
divided into two 16×128 parts conceptually, and each local
Next V.
switch is operating as a normal 128×128 FCB SRAM bank to
Local
Switch
Global
Global
provide a full 128×128 interconnection for the two parts. As
...
Switch
Switch
Local
shown by the dash lines in Figure 8, 256-bit ANDed results
Switch
from the two CAM sub-arrays (one of sub-arrays is turned off
Mode 3: 32bits
but its results are reset to ‘1’s) are separated into two 128-bit
Next V.
parts and connected with 384-bit input through the MUX after
Local
Switch
replication (the replica is fixed) while the readout results of
Array 1
Array 16
RRCB are combined (OR) to 128-bit data after the deMUX.
Local
Switch
Finally, the combined outputs of two RRCBs are merged to the
256-bit output bus and sent back to the Next Vector register.
Fig. 7. General overview of bank abstraction and three different modes.
The system is configured to 32-bit mode when the 16-bit
VI. S YSTEM I MPLEMENTATION
encoding cannot cover large symbol classes of applications.
A. Overall Design
As described by Mode 3 in Figure 7, match results from the
The system diagram of a CAMA bank is shown in Figure 7, two sub-arrays are combined by an AND gate and then sent
which can store up to 65536 states. Each bank consists to the two local switches. Its circuit level working principle
of 16 arrays with 8 tiles in each array sharing one global is just like the 16-bit FCB mode except the code length and
switch. Inside the tile, two 128×128 local switches are stacked the number of working CAMs. Although this mode has the
vertically to match the length of the two horizontally placed max compression for symbol classes, this 32-bit mode has the
16×256 CAM sub-arrays, leading to a compact physical layout. worst area utilization and consumes the highest power.
The number of CAM entries is chosen based on the encoding
results in Table II where 16 is the maximum value in all B. Pipeline, Input and Output
There are two versions of CAMA, namely CAMA-E and
benchmarks except RandomForest. All the encoded inputs are
stored in a 256×32 SRAM (not drawn in the figure) which CAMA-T, targeting high energy efficiency and high throughput,
respectively. CAMA-E saves energy by only precharging the
only consumes only 0.1% of total energy on average.
According to the circuit and algorithm design, there exists enabled STEs during state transition (Figure 8(a)), where
some cases where we have to use 32-bit one-0 prefix encoding the transition results are directly sent back to CAM and the
for larger compression space or store the dense transitions in column precharger performs the AND operation. However,
local switches that are reconfigured to FCB. Therefore, three CAMA-E cannot pipeline state matching and state transition
processing modes are designed to handle different situations: because the next symbol must obtain the transition results from
16-bit RCB mode, 16-bit FCB mode, and 32-bit mode (see the last symbol but the direct feedback causes conflicts. As
Figure 7). Those modes cover nearly all applications with the shown in Figure 8(a), the non-pipelined architecture leads to
alphabet size no larger than 256 (the largest alphabet size is relatively low clock frequency. On the other hand, CAMA-T,
256 in ANMLZoo and Regex) since the longest code length a configuration optimized for throughput, connects transition
results to AND gates instead of feeding back them to the
of CAMA is 32 bit using One-Zero prefix encoding.
The 16-bit RCB mode is activated when the application can CAM to realize the pipeline as shown in the Figure 8(b) and
be expressed by codes less than 16-bit and the interconnection Figure 9(b). In the state matching phase, the results are stored
can be stored into the 128×128 RCB-mode local switch. In in the Match Vector register which acts as the pipeline register.
this mode, both CAM sub-arrays in the tile are connected to The rest of the operations are similar to CA. Since the trade-off
the local switches (see Mode 1 in Figure 7). Figure 8 illustrates between the energy and throughput is application-dependent, we
the detailed implementation of each tile, where the connection evaluate both non-pipelined (CAMA-E) and pipelined (CAMAof 16-bit mode is drawn with solid lines. As mentioned in T) in the experiments.
As for the data storage, CAMA uses two buffers to store the
§IV.B, the 128×128 RRCB has 3 WLs in each row and 2
BLs in each column, resulting in the 384-bit input and 256-bit input symbols and output report vectors. The input symbols are
output. After the state matching, the MUX before RRCB’s stored in the 128-entry input buffer similar to CA. Every cycle
input will choose the 256-bit CAM output and replicate some one symbol is deleted and distributed to all arrays through the
of them to match the 384-bit input bus (The replica connection bus. Every time the input buffer is empty, the buffer will cause
is fixed). The deMUX after RRCB’s readout routes 256-bit an interruption and get a set of new symbols.
output to the Next Vector register. This mode has the lowest
A 64-entry output buffer is designed to keep a similar
energy consumption and the highest area utilization.
interruption frequency between input and output. Similar to
Mode 1：16bits RCB

Next V.

16x256 CAM

Closed

16x256 CAM

16x256 CAM

16x256 CAM

Input/Output Buffer

16x256 CAM

8

<!-- page 9 -->

From
16b Global
256b

[240:255]

||

NNext V.t

NNext V.t

256b

From
Global

16b
R/F

256b

[0:239]

report

[240:255]

||
[0:239]

report

R/F

R/F
256b

256b

384b

report

Combined
128b

16b
To
Global

R/F

[0:127]
[128:255]

||

256b

Local 256b
Switch

To
Global
Combined
128b

Combined
128b
To
Global

R/F

R/F

report

16b

384b

256b

256b

||

[0:127]
[128:255]
Match V.

16b

384b

16b

384b

report

256b
16x256 CAM

16x256 CAM

R/F
report

Local 256b
Switch

256b

Match V.

16x256 CAM

Local 256b
Switch

16x256 CAM

256b

Local 256b
Switch

To
Global
Combined
128b

256b
R/F

R/F
256b

[0:239]

[0:239]

(a)

||

R/F

[240:255]

From
16b Global

256b

Next V.

[240:255]

Next V.

||

256b
256b

R/F
16b

(b)

From
Global

Fig. 8. Detailed implementation of one tile in CAMA: (a)CAMA-E with selective row enabling and (b) CAMA-T with pipelined matching and transition for
higher throughput.
TABLE III
State
G Switch
Input1 Matching
C IRCUIT MODELS IN 28 NM
L Switch
Type
Size
Energy (pJ) Delay (ps) Area (µm2) Leakage (µA)
State
G Switch
256×256
19.45
416
14877
532
Input2
Matching
L Switch
6T SRAM
16×256
15.3
317
3659
247
Cycle1
Cycle2
128×128
8.67
292
5655
243
(a)
8T SRAM
256×256
17.9
394
18153
584
State
G Switch
Input1 Matching
8T CAM 16×256
16.78
325
3919
299
L Switch
State
Matching

Input2
Cycle1

Cycle2

G Switch
L Switch

including CA [29], Impala [24], AP [10], FlexAmata [26], and
Grapefruit [22], all adopt ANMLZoo for evaluation. 10MB
inputs are used for all evaluations.
Experiment Setup: The open-source virtual automata simulator VASim [34] is used to evaluate CAMA and prior designs.
VASim is modified to support the proposed encoding scheme.
The simulator takes in one ANML or MNRL file, and calculates
matches and transitions of each input symbol cycle by cycle and
provides cycle-accurate energy estimation of the archtectures.
Table III lists memory parameters in our evaluations, including access energy, delay and area. These numbers come from
SPICE simulations on custom designed SRAM and CAM arrays
in TSMC 28nm CMOS. The 6T SRAM parameters at different
sizes and configurations are very close to 6T SRAMs from
TSMC compilers. We cannot entirely rely on the compiler
because it does not offer some of the memory types and
configurations in our evaluations. For fair comparison, we
re-simulate SOTA architectures with the same circuit models.

(b)

Fig. 9. Timing diagram of (a) non-pipeline and (b) pipeline implementations.

input interruption, every time the output buffer is full, an
interruption is sent to the CPU, and CPU will get the reports
and clear all entries. [32] characterizes the reporting behavior
on ANMLZoo, showing that 10 out of 12 benchmarks have less
than 0.5 reports per cycle on average on 1MB input. Therefore,
this ratio inspires us to adopt the 64-entry output buffer to hide
the output interruption behind input interruption. Each entry
stores the matching results and the corresponding reporting
state ID. The mapping of the report states is realized by setting
up a report mask for each bank. Once a report is detected, the
bank will create an output entry and store the active state ID,
partition ID, input symbol, and the reporting cycle.
VII. E VALUATION M ETHODOLOGY
NFA workloads: We evaluate the proposed architecture and
algorithmic optimization by using ANMLZoo [34] and Regex
[4] benchmarks. These benchmarks cover a wide range of realworld applications, including network intrusion detection [4],
machine learning, gene sequence matching [23] and data mining
[36]. While a more recent benchmark suite AutomataZoo [33]
extends ANMLZoo with larger benchmarks. ANMLZoo is
selected in our evaluation because (1) the AP chip with up to
48k states only supports ANMLZoo, (2) evaluating an automata
accelerator on large benchmarks in AutomataZoo is excessively
time consuming, and (3) prior studies on automata acceleration,

VIII. R ESULTS
In this section, we first compare the throughput of existing
in-memory automata processors, followed by analysis of area
efficiency, compute density and energy/power consumption.
A. Overall Performance
The overall performance of the spatial automata processing
architectures is defined as the clock period or throughput
(frequency×bits/cycle). As Table IV shows, CAMA and 2stride Impala’s state matching has less delay than CA and eAP.

9

<!-- page 10 -->

TABLE IV
D ELAYS AND FREQUENCY IN 28 NM
Design
CAMA-E
CAMA-T
2-stride Impala [24]
eAP [27]
CA [29]
AP [10]

State Match
325ps
325ps
317ps
394ps
416ps
N/A

L-switch
292ps
292ps
394ps
394ps
394ps
N/A

G-switch
420.1ps
420.1ps
442.69ps
515ps
493ps
N/A

Freq.Max
1.34GHz
2.38GHz
2.26GHz
1.94GHz
2.03GHz
0.133GHz

TABLE V
S WITCH MAPPING RESULTS FOR CA AND CAMA
Freq.Operated
1.21GHz
2.14GHz
2.03GHz
1.75GHz
1.82GHz
0.133GHz

Baseline
Local
169
199
381
160
78
166
277
47
162
12
139
500
10
49
51
50
50
51
419
192
50

Benchmark
Brill
ClamAV
Dotstar
Fermi
TCP
Protomata
Snort
Hamming
PowerEN
Levenshtein
RandomForest
EntityResolution
Bro217
Dotstar03
Dotstar06
Dotstar09
Ranges1
Ranges05
SPM
BlockRings
ExactMath

Baseline
Global
0
3
0
0
1
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0

Proposed
RCB mode
169
199
408
245
76
274
284
47
162
12
0
0
10
50
53
51
52
53
419
192
50

Proposed
Global
0
0
0
0
1
1
1
0
0
0
40
0
0
0
0
0
0
0
0
0
0

Proposed
FCB mode
0
3
0
0
8
5
27
0
0
0
662
1000
0
0
0
0
0
0
0
0
0

i

5

st

e

s0

re

ag

er

tm

fo

09

ar

st

ac

s1

do

m

an

ge

ge

at

an

R

an

R

R

E x

yR

tit

E n

10

av

06

ar

ot

D

D

h

03

ar

st

ot

n

P

st

ut

io

ot

D

ol

ei

in

ht

oc

B l

es

kr

in

m

Le

H

ve

am

ns

M

rm

E N

S P

F e

T C

n

g

gs

t

a

er

at

or

ot

P r

C

P o

w

S n

om

A V

ar

st

la

m

17

ot

B r

o2

B r

D

ill

2 8 n m

a re a (m m

2

)

This is because both designs have a denser state matching
array and reduce the length of BLs from 256 to 16. Shorter
BLs result in less RC delay. Benefiting from the compact local
switch design, CAMA and eAP will have less delay in local
switch. Because of the long-distance routing in the global
switch, the delay consists of two parts: the wire delay and the
memory access delay. Based on CA’s model, the wire delay
on global routing is 99ps. As a result, the global delay for
CA is 493ps with 394ps read delay and 99ps wire delay in
28nm. Since 2-stride Impala has 2× less state matching area
2 0
overhead than CA and CAMA is 4× less than CA (area listed
1 8
C A M A
2 - s t r id e I m p a la
1 6
in Table III), the wire delay of 2-stride Impala and CAMA
e A P
1 4
is estimated as 48.69ps and 26.1ps, repectively. Due to the
C A
1 2
1 0
large area of 8T SRAM in state matching, eAP has the largest
8
wire delay of 121ps. It is worth mentioning that the original
6
4
three-stage pipeline used in CA will cause a data-hazard issue
2
[27]. Thus, we adjust the original CA pipeline and make the
0
local switch and global switch process simultaneously, similar
to the settings in [27].
The largest stage latency determines the frequency of the
Fig. 10. Area comparison.
pipeline. Designs of 2-stride Impala, CA and CAMA-T all
have the slowest delay on global switches. The frequency of even if CAMA-E has smaller throughput, it shows an average
CAMA-E (the non-pipeline design) is bottlenecked by the state 1.51×, 2.19× and 1.48× higher compute density than 2-stride
matching delay plus the global switch delay while the local Impala and CA, eAP thanks to the reduced memory area. Only
switch delay is hidden behind the global switch delay because a few applications, e.g., EntityResoluton and RandomForest
they are working in parallel. We choose a 10% lower frequency which cannot fit into RCB-mode switch, show lower compute
than the max frequency to provide margin for estimation errors. density as it works in the 16-bit FCB mode where only one
Overall, CAMA-T has higher throughput than CA, 2-stride CAM sub-array in the tile is operating.
Impala, eAP and CAMA-E by 1.18×, 1.05×, 1.22× and 1.76×
in 28nm, respectively. This is mainly because CAMA has C. Energy/Power Consumption
To calculate the energy consumption in each cycle, we
smaller state matching array leading to smaller global wire
delay which is the bottleneck for all designs. CAMA-T and need to know the number of active partitions, the number
CAMA-E provide a 16.1× and 9.1× speedup than AP(50nm). of enabled partitions which is the key factor in CAMA-E (can
be overlooked by other pipelined designs), the number of active
B. Area Overhead and Compute Density
rows in both state matching and state transition phases, and
CAMA-T and CAMA-E benefit from the area reduction the number of dynamic transitions between partitions which
in both state matching phase and state transition phase while result in increased global switch accesses and wire energy. All
Impala only optimizes the state matching and eAP only reduces these factors are influenced by the mapping algorithm. We
the memory area of state transition. CA has the largest area use a greedy mapping algorithm similar to CA to pack the
overhead since neither of the phases are optimized. Table V connected components into hardware, and thus the number of
shows the mapping result in CAMA, where CAMA-T and active partitions are reduced.
CAMA-E have the same mapping. Overall, CAMA has lower
As mentioned in eAP and Impala, it is impossible to powerarea overhead than 2-stride Impala, eAP and CA by 1.91×, gate state matching arrays on a cycle-by-cycle bias in pipeline
1.78× and 2.48×, respectively.
design as we need to know the potential next states in advance.
Due to the faster clock frequency and area optimization in They are calculated in the transition phase in the last cycle but
both phases, CAMA features the highest compute density which overlapped with the state matching phase in one cycle in the
is defined as throughput per unit area (TOPS/mm2 ). CAMA-T pipeline design. Meanwhile, only active STEs will access local
provides an average 2.68×, 3.87× and 2.62× higher compute switch and the more active rows a switch has, the more energy
density than 2-stride Impala, CA and eAP in 28nm. Meanwhile, it will consume. This is not included in Impala and eAP’s

<!-- page 11 -->

2.5
2.0

6.0 101 .1 2.5 5.1

3.9 6.2

3.4

Energy/byte (Normalized to CAMA-E)

3.0

1.5

4.1 21.5 84.3 5.3 1.0 11.8 20.2 19.1 19.8 20.2 19.4 19.1

2.4

18.0

1.5
1.0

0.20

0.21 0.21

1.14
0.30

0.06
0.58
3.26

1.85

3.12

4.25
8.19

4.68

2.43

0.92 0.24 3.49 10.51
5.48

B
Br rill
o2
17
ot
s
C tar
la
m
AV
Pr Sno
ot
om rt
Po at
w a
er
EN
SP
M
F
H erm
am i
Le
m
v e in
ns g
En Blo hte
tit ck in
yR ri
es ngs
ol
ut
io
n
D TC
ot
P
st
D ar0
ot
st 3
D ar0
ot
s 6
Ex tar
ac 09
tm
R ath
an
R ge
R an s1
an ge
do s0
m 5
fo
r
av est
er
ag
e
0.25

2.42

0.26 0.26

0.31
1.38

0.36

0.08

0.70

8
2.24

3.95

6
3.78

5.1
9.92

4

5.67

2.94

1.11 0.30 4.22 12.72

0

la

C

6.63

D. Multi-stride Comparison
CAMA is compatible with multi-stride operation for higher
throughput. Here, we compare the 2-stride CAMA with 4-stride
Impala using the striding algorithm in [5], [6]. To support the
2-stride design, CAMA’s state matching array must be 64×256
and the local switch should be expanded to 256×256 to support
a larger number of transitions and alphabet sizes. Figure 13
shows that on average 4-stride Impala consumes 2.18× and
3.77× more energy than 2-stride CAMA-T and 2-stride CAMAE because 4-stride Impala needs four 16×256 SRAM banks
compared to CAMA’s one 64×256 bank (61.2pJ vs 22pJ).

2
0

D

B
Br rill
o2
17
ot
s
C tar
la
m
AV
Pr Sno
ot
om rt
Po at
w a
er
EN
SP
M
F
H erm
am i
Le
m
ve in
ns g
En Blo hte
tit ck in
yR ri
es ngs
ol
ut
io
n
D TC
ot
P
st
D ar0
ot
st 3
D ar0
ot
s 6
Ex tar
ac 09
tm
R ath
an
R ge
R an s1
an ge
do s0
m 5
fo
r
av est
er
ag
e

Power (Normalized to CAMA-E)

0.23

10

Fig. 11. Comparison of the (a) compute density (b) energy per symbol, and
(c) power in 28nm. All bars are normalized to the results of CAMA-E for
clarity and the absolute values (in units of Gbps/mm2 , nJ/byte, and Watt) of
CAMA-E are shown by red texts.
e n e rg y b re a k d o w n o f C A M A (% )

2

entries. CAMA-T’s energy stays at 16.78pJ, which is slightly
lower than CA and eAP’s 19.45pJ because of the reduced CAM
rows. The 2-stride Impala requires the larger energy (30.6pJ)
to access two 16x256 SRAMs due to the doubled readout
periphery. Particularly, both CAMA designs have higher energy
on RandomForest as it works on the 32-bit mode. We show
the energy breakdown of CAMA in Figure 12. On average, the
state matching occupies 27% and 64.6%, the interconnect takes
72.89% and 35.35% and the encoder only accounts for 0.11%
and 0.05% of the total energy in CAMA-E and CAMA-T.
The power consumption (see Figure 11(c)) further verifies
the high efficiency of CAMA-E. On average, CA, 2-stride
Impala, eAP and CAMA-T have 3.15×, 4.71×, 2.94× and
3.63× more power than CAMA-E in 28nm, respectively.

0.25

2.00

12

(c)

4

Fig. 13. Energy for 2-stride CAMA and 4-stride Impala in 28nm.

0.19

(b)

14

6

Br

D

B
Br rill
o2
17
ot
s
C t ar
la
m
AV
Pr Sno
ot
om rt
Po at
w a
er
EN
SP
M
F
H erm
am i
Le
m
ve in
ns g
En Blo hte
tit ck in
yR ri
es ngs
ol
ut
io
n
D TC
ot
P
st
D ar0
ot
st 3
D ar0
ot
s 6
Ex tar
ac 09
tm
R ath
an
R ge
R an s1
an ge
do s0
m 5
fo
r
av est
er
ag
e

(a)

16

8

Br

0.0

9.0
8.5
8.0
7.5
7.0
6.5
6.0
5.5
5.0
4.5
4.0
3.5
3.0
2.5
2.0
1.5
1.0
0.5
0.0

2-stride CAMA-E
2-stride CAMA-T
4-stride Impala

10

ill
o2
17
m
AV
S
Pr no
ot
r
om t
Po ata
w
er
EN
SP
M
F
H erm
am i
Le
m
v e in
ns g
h
En Blo tei
tit ck n
yR rin
es gs
ol
ut
io
n
D TCP
ot
st
D ar0
ot
3
st
D ar0
ot
6
st
Ex ar0
ac 9
tm
R at h
an
R ges
R ang 1
an
do es0
m 5
fo
re
av st
er
ag
e

0.5

D

Energy/cycle (Normalized to CAMA-E) Compute Density (Normalized to CAMA-E)

CAMA-E
CAMA-T
2-stride Impala
eAP
CA

3.5

C A M A -T e n c o d e r
s w itc h a n d w ir e
s ta te m a tc h

C A M A -E e n c o d e r
s w itc h a n d w ir e
s ta te m a tc h

1 0 0
8 0
6 0

IX. C ONCLUSION
This paper presents CAMA, a CAM-based in-memory archi0
tecture for efficient automata processing. CAMA co-designs
hardware architecture and encoding algorithms to achieve both
high compute dense and energy efficiency. It exploits an 8T
Fig. 12. Energy breakdown of CAMA in different benchmarks.
16×256 CAM bank to reduce the state matching area and
energy model as they only consider the worst case in which all energy, and a 128×128 RRCB as compact state transition
switches are accessed and the number of active rows in switch switches. The proposed encoding optimization algorithms
is not took into account. We update the energy/power results selects the optimal encoding to balance the compression rate
for CA and 2-stride Impala based on the two observations. and the code length for a given application, and cluster symbols
As Figure 11(b) shows, on average, CAMA-E achieves 2.1×, into the encoding clusters for optimal CAMA performance and
2.8×, 2.04× and 2.04× lower energy than CA, 2-stride Impala, efficiency. CAMA can be designed in non-pipelined version
eAP and CAMA-T in 28nm, respectively. This is as expected (CAMA-E) to save energy and pipelined mode (CAMA-T)
because the access energy of the state-matching CAM varies for faster speed. On average, the proposed CAMA-T shows
from 2.67pJ to 16.78pJ depending on the number of enabled 2.68×, 3.87× and 2.62× higher compute density than 2-stride
entries which is fundamentally smaller than that of accessing Impala, CA and eAP in 28nm while CAMA-E shows 2.1×,
all rows (16.78 pJ). In the worst benchmark (Fermi), CAME-E 2.8×, 2.04× and 2.04× lower energy than CA, 2-stride Impala,
only consumes 7.8pJ on average due to the selectively enbled eAP and CAMA-T in 28nm.
4 0

e

5

st

s0

re

ag

fo

er

at

ge

an

do

m

tm

an

E n

R

an

R

R

E x

av

09

ar

st

ac

h

06

ar

s1

03

ar

st

ot

ot

D

D

ge

P

st

ot

ut

tit

D

in

kr

oc

yR

es

ol

io

T C

n

g

in

ei

ht

ve

am

Le

H

B l

ns

M

rm

E N

m

F e

S P

n

gs

t

a

er

at

or

ot

P r

P o

w

S n

om

ar

A V

m

17

st

ot

la

D

C

B r

B r

o2

ill

i

2 0

11

<!-- page 12 -->

R EFERENCES
[1] A. V. Aho and M. J. Corasick, “Efficient string matching: An aid to
bibliographic search,” Commun. ACM, vol. 18, no. 6, p. 333–340, Jun.
1975. [Online]. Available: https://doi.org/10.1145/360825.360855
[2] M. Alicherry, M. Muthuprasanna, and V. Kumar, “High Speed Pattern
Matching for Network IDS/IPS,” in Proceedings of the 2006 IEEE
International Conference on Network Protocols, Nov. 2006, pp. 187–
196.
[3] R. Alur and M. Yannakakis, “Model checking of hierarchical state
machines,” ACM Trans. Program. Lang. Syst., vol. 23, no. 3, p. 273–303,
May 2001. [Online]. Available: https://doi.org/10.1145/503502.503503
[4] M. Becchi, M. Franklin, and P. Crowley, “A workload for evaluating deep
packet inspection architectures,” in 2008 IEEE International Symposium
on Workload Characterization, Sep. 2008, pp. 79–89.
[5] M. Becchi and P. Crowley, “An improved algorithm to accelerate
regular expression evaluation,” in Proceedings of the 3rd ACM/IEEE
Symposium on Architecture for Networking and Communications
Systems, ser. ANCS ’07. New York, NY, USA: Association
for Computing Machinery, 2007, p. 145–154. [Online]. Available:
https://doi.org/10.1145/1323548.1323573
[6] M. Becchi and P. Crowley, “Efficient regular expression evaluation:
Theory to practice,” in Proceedings of the 4th ACM/IEEE Symposium on
Architectures for Networking and Communications Systems, ser. ANCS
’08. New York, NY, USA: Association for Computing Machinery, 2008,
p. 50–59. [Online]. Available: https://doi.org/10.1145/1477942.1477950
[7] C. Bo, V. Dang, E. Sadredini, and K. Skadron, “Searching for potential
grna off-target sites for CRISPR/Cas9 using automata processing across
different platforms,” in 2018 IEEE International Symposium on High
Performance Computer Architecture (HPCA), 2018, pp. 737–748.
[8] C. Bo, K. Wang, J. J. Fox, and K. Skadron, “Entity resolution acceleration
using the automata processor,” in 2016 IEEE International Conference
on Big Data (Big Data), 2016, pp. 311–318.
[9] A. Bremler-Barr, D. Hay, and Y. Koral, “CompactDFA: Scalable
Pattern Matching Using Longest Prefix Match Solutions,” IEEE/ACM
Transactions on Networking, vol. 22, no. 2, pp. 415–428, Apr. 2014.
[10] P. Dlugosch, D. Brown, P. Glendenning, M. Leventhal, and H. Noyes, “An
Efficient and Scalable Semiconductor Architecture for Parallel Automata
Processing,” IEEE Transactions on Parallel and Distributed Systems,
vol. 25, no. 12, pp. 3088–3098, Dec. 2014.
[11] Y. Fang, T. T. Hoang, M. Becchi, and A. A. Chien, “Fast support
for unstructured data processing: The unified automata processor,” in
Proceedings of the 48th International Symposium on Microarchitecture.
Waikiki Hawaii: ACM, Dec. 2015, pp. 533–545.
[12] V. M. Glushkov, “THE ABSTRACT THEORY OF AUTOMATA,”
vol. 16, no. 5, pp. 1–53, publisher: IOP Publishing. [Online]. Available:
https://doi.org/10.1070/rm1961v016n05abeh004112
[13] V. Gogte, A. Kolli, M. J. Cafarella, L. D’Antoni, and T. F. Wenisch,
“HARE: Hardware accelerator for regular expressions,” in 2016 49th Annual IEEE/ACM International Symposium on Microarchitecture (MICRO).
Taipei, Taiwan: IEEE, Oct. 2016, pp. 1–12.
[14] S. Jeloka, N. B. Akesh, D. Sylvester, and D. Blaauw, “A 28 nm
Configurable Memory (TCAM/BCAM/SRAM) Using Push-Rule 6T Bit
Cell Enabling Logic-in-Memory,” IEEE Journal of Solid-State Circuits,
vol. 51, no. 4, pp. 1009–1021, Apr. 2016.
[15] M. Lenjani and M. R. Hashemi, “Tree-based scheme for reducing shared
cache miss rate leveraging regional, statistical and temporal similarities,”
IET Computers & Digital Techniques, vol. 8, no. 1, pp. 30–48, 2014.
[Online]. Available: https://ietresearch.onlinelibrary.wiley.com/doi/abs/10.
1049/iet-cdt.2011.0066
[16] D. Li and K. Yang, “A dual-port 8-t cam-based network intrusion detection
engine for iot,” IEEE Solid-State Circuits Letters, vol. 3, pp. 358–361,
2020.
[17] T. Liu, Y. Yang, Y. Liu, Y. Sun, and Li Guo, “An efficient regular
expressions compression algorithm from a new perspective,” in 2011
Proceedings IEEE INFOCOM, Apr. 2011, pp. 2129–2137.
[18] J. V. Lunteren, C. Hagleitner, T. Heil, G. Biran, U. Shvadron, and
K. Atasu, “Designing a Programmable Wire-Speed Regular-Expression
Matching Accelerator,” in 2012 45th Annual IEEE/ACM International
Symposium on Microarchitecture, Dec. 2012, pp. 461–472.
[19] K. Pagiamtzis and A. Sheikholeslami, “Content-addressable memory
(CAM) circuits and architectures: A tutorial and survey,” IEEE Journal
of Solid-State Circuits, vol. 41, no. 3, pp. 712–727, Mar. 2006.

12

[20] D. Pao, X. Wang, X. Wang, C. Cao, and Y. Zhu, “String Searching
Engine for Virus Scanning,” IEEE Transactions on Computers, vol. 60,
no. 11, pp. 1596–1609, Nov. 2011.
[21] K. Peng, S. Tang, M. Chen, and Q. Dong, “Chain-Based DFA Deflation
for Fast and Scalable Regular Expression Matching Using TCAM,” in
2011 ACM/IEEE Seventh Symposium on Architectures for Networking
and Communications Systems, Oct. 2011, pp. 24–35.
[22] R. Rahimi, E. Sadredini, M. Stan, and K. Skadron, “Grapefruit: An
Open-Source, Full-Stack, and Customizable Automata Processing on
FPGAs,” in 2020 IEEE 28th Annual International Symposium on FieldProgrammable Custom Computing Machines (FCCM). Fayetteville,
AR, USA: IEEE, May 2020, pp. 138–147.
[23] I. Roy and S. Aluru, “Discovering Motifs in Biological Sequences
Using the Micron Automata Processor,” IEEE/ACM Transactions on
Computational Biology and Bioinformatics, vol. 13, no. 1, pp. 99–111,
Jan. 2016.
[24] E. Sadredini, R. Rahimi, M. Lenjani, M. Stan, and K. Skadron, “Impala:
Algorithm/Architecture Co-Design for In-Memory Multi-Stride Pattern
Matching,” in 2020 IEEE International Symposium on High Performance
Computer Architecture (HPCA), Feb. 2020, pp. 86–98.
[25] E. Sadredini, D. Guo, C. Bo, R. Rahimi, K. Skadron, and H. Wang,
“A Scalable Solution for Rule-Based Part-of-Speech Tagging on Novel
Hardware Accelerators,” in Proceedings of the 24th ACM SIGKDD
International Conference on Knowledge Discovery & Data Mining.
London United Kingdom: ACM, Jul. 2018, pp. 665–674.
[26] E. Sadredini, R. Rahimi, M. Lenjani, M. Stan, and K. Skadron,
“FlexAmata: A Universal and Efficient Adaption of Applications to Spatial
Automata Processing Accelerators,” in Proceedings of the Twenty-Fifth
International Conference on Architectural Support for Programming
Languages and Operating Systems. Lausanne Switzerland: ACM, Mar.
2020, pp. 219–234.
[27] E. Sadredini, R. Rahimi, V. Verma, M. Stan, and K. Skadron,
“eAP: A scalable and efficient in-memory accelerator for automata
processing,” in Proceedings of the 52nd Annual IEEE/ACM International
Symposium on Microarchitecture, ser. MICRO ’52. New York, NY,
USA: Association for Computing Machinery, 2019, p. 87–99. [Online].
Available: https://doi.org/10.1145/3352460.3358324
[28] E. Sadredini, R. Rahimi, K. Wang, and K. Skadron, “Frequent subtree
mining on the automata processor: Challenges and opportunities,” in
Proceedings of the International Conference on Supercomputing, ser.
ICS ’17. New York, NY, USA: Association for Computing Machinery,
2017. [Online]. Available: https://doi.org/10.1145/3079079.3079084
[29] A. Subramaniyan, J. Wang, E. R. M. Balasubramanian, D. Blaauw,
D. Sylvester, and R. Das, “Cache automaton,” in Proceedings of the
50th Annual IEEE/ACM International Symposium on Microarchitecture
- MICRO-50 ’17. Cambridge, Massachusetts: ACM Press, 2017, pp.
259–272.
[30] P. Tandon, F. M. Sleiman, M. J. Cafarella, and T. F. Wenisch, “Hawk:
Hardware support for unstructured log processing,” in 2016 IEEE 32nd
International Conference on Data Engineering (ICDE), 2016, pp. 469–
480.
[31] Tran Trung Hieu and N. T. Tran, “A memory efficient FPGA-based
pattern matching engine for stateful NIDS,” in 2013 Fifth International
Conference on Ubiquitous and Future Networks (ICUFN). DA NANG,
Vietnam: IEEE, Jul. 2013, pp. 252–257.
[32] J. Wadden, K. Angstadt, and K. Skadron, “Characterizing and Mitigating
Output Reporting Bottlenecks in Spatial Automata Processing Architectures,” in 2018 IEEE International Symposium on High Performance
Computer Architecture (HPCA), Feb. 2018, pp. 749–761.
[33] J. Wadden, T. Tracy, E. Sadredini, L. Wu, C. Bo, J. Du, Y. Wei,
J. Udall, M. Wallace, M. Stan, and K. Skadron, “AutomataZoo: A Modern
Automata Processing Benchmark Suite,” in 2018 IEEE International
Symposium on Workload Characterization (IISWC), Sep. 2018, pp. 13–24.
[34] J. Wadden, V. Dang, N. Brunelle, T. T. Ii, D. Guo, E. Sadredini, K. Wang,
C. Bo, G. Robins, M. Stan, and K. Skadron, “ANMLzoo: A benchmark
suite for exploring bottlenecks in automata processing engines and
architectures,” in 2016 IEEE International Symposium on Workload
Characterization (IISWC). Providence, RI, USA: IEEE, Sep. 2016, pp.
1–12.
[35] K. Wang, K. Angstadt, C. Bo, N. Brunelle, E. Sadredini, T. Tracy,
J. Wadden, M. Stan, and K. Skadron, “An overview of micron’s
automata processor,” in Proceedings of the Eleventh IEEE/ACM/IFIP
International Conference on Hardware/Software Codesign and System
Synthesis, ser. CODES ’16. New York, NY, USA: Association for

<!-- page 13 -->

Computing Machinery, 2016. [Online]. Available: https://doi.org/10.
1145/2968456.2976763
[36] K. Wang, E. Sadredini, and K. Skadron, “Sequential pattern mining
with the micron automata processor,” in Proceedings of the ACM
International Conference on Computing Frontiers, ser. CF ’16. New
York, NY, USA: Association for Computing Machinery, 2016, p.
135–144. [Online]. Available: https://doi.org/10.1145/2903150.2903172
[37] T. Xie, V. Dang, J. Wadden, K. Skadron, and M. Stan, “Reapr: Reconfigurable engine for automata processing,” in 2017 27th International
Conference on Field Programmable Logic and Applications (FPL), 2017,
pp. 1–8.

13

[38] M. Yabuuchi, M. Morimoto, K. Nii, and S. Tanaka, “12-NM Fin-FET
3.0G-Search/s 80-Bit × 128-Entry Dual-Port Ternary CAM,” in 2018
IEEE Symposium on VLSI Circuits, Jun. 2018, pp. 19–20.
[39] F. Yu, Z. Chen, Y. Diao, T. V. Lakshman, and R. H. Katz, “Fast and
memory-efficient regular expression matching for deep packet inspection,”
in 2006 Symposium on Architecture For Networking And Communications
Systems, 2006, pp. 93–102.
[40] S. Yun, “An Efficient TCAM-Based Implementation of Multipattern
Matching Using Covered State Encoding,” IEEE Transactions on
Computers, vol. 61, no. 2, pp. 213–221, Feb. 2012.
