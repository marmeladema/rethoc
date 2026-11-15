---
file: ../pdfs/2024-arxiv-zhong-xav-regex-matching-engine.pdf
citation: "XAV: A High-Performance Regular Expression Matching Engine for Packet Processing. Zhong, Chen, Yu. arXiv 2403.16533, 2024"
doi: 
source: "https://arxiv.org/pdf/2403.16533"
topics: [performance_hardware]
pages: 15
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

1

XAV: A High-Performance Regular Expression
Matching Engine for Packet Processing

arXiv:2403.16533v1 [cs.NI] 25 Mar 2024

Jincheng Zhong, Shuhui Chen, Chuan Yu

Abstract—Regular expression matching is the core function of
various network security applications such as network intrusion
detection systems. With the network bandwidth increases, it is
a great challenge to implement regular expression matching for
line rate packet processing. To this end, a novel scheme named
XAV targeting high-performance regular expression matching is
proposed in this paper. XAV first employs anchor DFA to tackle
the state explosion problem of DFA. Then based on anchor DFA,
two techniques including pre-filtering and regex decomposition
are utilized to improve the average time complexity. Through
implementing XAV with an FPGA-CPU architecture, comprehensive experiments show that a high matching throughput of
up to 75 Gbps can be achieved for the large and complex Snort
rule-set. Compared to state-of-the-art software schemes, XAV
achieves two orders of magnitude of performance improvement.
While compared to state-of-the-art FPGA-based schemes, XAV
achieves more than 2.5x performance improvement with the same
hardware resource consumption.
Index Terms—Regular expression matching, finite state automaton, network intrusion detection, packet processing.

I. I NTRODUCTION
A regular expression (regex) comprises a sequence of
characters, which describe a search pattern that can match
a set of strings. For example, regex “[a-zA-Z0-9 -]+@[azA-Z0-9 -]+(\ .[a-zA-Z0-9 -]+)+$” describes a search pattern
matching any email address. For their powerful expressiveness
in defining search patterns, regexes are widely used in defining
malicious features within many network security applications
such as network intrusion detection systems (NIDS) and
web application firewalls (WAF). Usually, regular expression
matching (REM) is performed for every incoming packet in
these network security applications. If one packet is found
to match some regex, it indicates that the packet possibly
contains some malicious pattern and further instructions will
be performed accordingly.
In order not to affect business applications, network security
applications must perform packet checking at line rate [1]–[3].
However, with the rapid growth of the Internet, the network
bandwidth of data centers has rapidly increased to 40 Gbps or
even 100 Gbps. To the best of our knowledge, very few REM
schemes can achieve a matching throughput of tens of Gbps
for large-scale regex rule-sets.
Targeting high matching throughput of tens of Gbps, this
paper proposes a novel REM scheme – XAV. XAV is named
by its three matching stages: Xor filter, Anchor DFA, and
Verification. Finite state automaton is the basic way to implement regular expression matching, and nondeterministic finite
This paper was produced by National University of Defense Technology.
They are in Changssha, CN.

automata (NFA) and deterministic finite automata (DFA) are
two common forms of finite state automata. However, NFA
and DFA both confront severe problems in implementing highperformance REM for large-scale rule-sets. The time complexity of the former is too high, while the space complexity of
the latter is too large.
This paper first puts forward the concept of anchor DFA,
which is essentially a DFA compiled from regexes all starting
with anchor “ˆ”. The application of anchor DFA greatly simplifies the regex semantics, thus avoiding the state explosion
problem of DFA. Although anchor DFA has much lower
space complexity than DFA, anchor DFA also introduces one
big problem it needs to start one matching thread at every
position of input to keep the semantics of original regexes. In
order to make the average time complexity of anchor DFA
close to that of DFA, two optimizations are proposed for
anchor DFA. First, the xor filter, which is responsible for
the matching of string parts in regexes, is used to reduce
the number of anchor DFA matching threads. Second, some
regexes are split into multiple parts to prevent a matching
thread of the anchor DFA from matching too many input bytes.
To cooperate with regex splitting, an extra verification stage is
then introduced to perform the regex semantic verification for
original regexes that have been split. Consequently, combining
the two optimizations for anchor DFA, the three matching
stages of our proposed REM scheme are xor filter, anchor
DFA, and verification.
We observe that the logic of the first two matching stages
of XAV is simple and suitable for hardware implementation to
achieve high matching performance. However, the logic of the
last matching stage (verification) is complex and only suitable
for software implementation. Therefore, an FPGA-CPU architecture is proposed to implement XAV, with implementing xor
filter and anchor DFA in FPGA and implementing verification
in CPU.
The main contributions of this paper are as follows:
1) This paper first proposes to employ anchor DFA to solve
the state explosion problem of DFA, thus achieving highperformance regex matching for large-scale rule-sets.
2) Xor filter and an extra verification stage are introduced
to greatly reduce the time complexity of anchor DFA.
Combining the two stages with anchor DFA, a novel
REM scheme XAV is proposed.
3) A FPGA-CPU architecture is employed to implement
XAV and detailed implementations are given in this
paper. The compiler source code of XAV will be opensourced once the paper is accepted.

<!-- page 2 -->

2

Category
Software-based

Scheme
HFA [4]/XFA [5]

Constributions
Proposes novel automata
to avoid the defects
of NFA and DFA

Limitations
Only achieves a
tradeoff between
NFA and DFA
Performance seriously
degrades for
complex regexes

Hyperscan [6]

Proposes regex decomposition
and prefiltering techniques

Multi-stride NFA [7]

Implements multi-stride
NFA using FPGAs

Resource consuming

Multi-stride DFA [8]

Implements multi-stride
DFA based on FPGA

Suffers from DFA
explosion problem

FBDFA [9]

Greatly alleviates the state
explosion problem of DFA

Resource consuming

Pigasus [10]/Fidas [11]

Offloads network intrusion
detection to FPGA from
the system view

Only supports simple
regexes based on
string prefiltering

TCAM-based

Chain-based DFA [12]

Employs TCAMs to perform
compressed-DFA matching

Suffers from DFA
explosion problem

Memory-centric
architecture

Impala [13]

Designs automaton processing
architectures from the
transistor level

Requires dedicated
hardware far
from the market

FPGA-based

Performance
Hundreds of Mbps for mediumscale rule-sets with
hundreds of regexes
About 1 Gbps on large
Snort rule-set with
thousands of regexes
About 10 Gbps for mediumscale rule-sets with
hundreds of regexes
>100 Gbps for small
rule-sets with only
tens of regexes
About 6 Gbps on large
Snort rule-set with
thousands of regexes
80 Gbps for the Snort ruleset with regexes containing
no string fragments
Several Gbps for mediumscale rule-sets with
tens of regexes
80 Gbps for large Snort
rule-set with thousands
of complex regexes

There have been a large number of prior works concentrating on how to design an efficient REM engine over the past
two decades. This paper divides the related works into two
big categories: software REM schemes and hardware REM
schemes. Table II provides a brief overview of related works.

defects of NFA and DFA. Despite alleviating the high space
complexity of DFA and the bad time complexity of NFA,
these novel automata are essentially a tradeoff between NFA
and DFA. These novel automata can only achieve medium
matching performance for medium-scale rule-sets. With the
regex rule-set size rising, the matching performance of these
novel automata will severely drop.
In recent years, regex decomposition methods [6], [21], [22]
become popular for achieving fast software REM. The key
idea of regex decomposition methods is to extract string parts
from original regexes first. Then taking fast string matching as
the entry to regex matching, these REM schemes try to avoid
expensive automata processing. Since string matching can be
two magnitudes faster than regex matching [6] and string parts
in regexes are rarely matched to trigger further regex matching,
regex decomposition methods have achieved great matching
performance. Hyperscan [6], the most famous software REM
scheme, claims it can achieve 1 Gbps throughput on Snort
rule-set with one CPU core.

A. Software REM schemes

B. Hardware REM schemes

Since DFA has the fantastical time complexity of O(1), a
variety of studies [14]–[20] concentrate on trying to reduce
the memory usage of DFA to support large-scale regex rulesets. These studies have found that the state transition table
of DFA is sparse and propose various compression methods
accordingly. Although the proposed DFA compression techniques can cut down the memory usage of DFA by more
than 95%, linear memory compression cannot tackle the state
explosion problem of DFA, which increases the memory
footprint exponentially. Consequently, these REM schemes
based on only DFA compression techniques can still only
support small-scale rule-sets.
Another variety of studies realized that both NFA and DFA
suffer from inherent defects. To achieve high-performance
regex matching for large-scale regex rule-sets, novel automata
such as HFA [4] and XFA [5] are designed to avoid the

1) FPGA-based schemes: A large number of studies utilize
FPGA to accelerate regex matching [7], [23]–[26]. Because of
the inherent massive parallel logic, FPGA is an ideal platform
to perform regex matching based on NFA processing. NFA
can contain multiple active states at a time, and multiple state
transitions need to be performed to consume one byte of input.
Therefore, the software is slow for NFA processing. However,
FPGA can perform the multiple state transitions of NFA in
parallel. As a consequence, the time complexity of NFA in
FPGA becomes O(1), and in each cycle, one byte of input
can be handled. To achieve higher matching throughput, [7],
[27], [28] further propose multi-stride NFA to handle multiple
characters each cycle on FPGA. Benefiting from the multistride technique, the FPGA NFA scheme [7] even achieves
a high matching throughput of 10 Gbps for Snort rule-set.
Although FPGA NFA schemes can achieve high matching

4) Comprehensive experiments are conducted to evaluate
XAV. Experimental results show that XAV has great
spatial and temporal performance and has achieved 75
Gbps throughput for the large Snort rule-set.
The rest of this paper is organized as follows. Section II
introduces the related works and inspirations of the XAV
scheme are elaborated in Section III. Next, the compilation
procedure of XAV is given in Section IV, and the FPGACPU implementation of XAV is depicted in Section V. Finally,
Section VI comprehensively evaluates the proposed scheme,
and a conclusion is given in Section VII.
II. R ELATED WORKS

<!-- page 3 -->

3

performance for complex rule-sets, rule-set updates in FPGA
NFA schemes are extremely tricky because FPGA circuit
synthesis is required to map NFA state to FPGA circuits.
Some FPGA REM schemes [9], [29]–[31] also employ
DFA to perform regex matching. The biggest advantage of
employing DFA instead of NFA in FPGA is that the ruleset update for FPGA DFA schemes only needs to rewrite
the FPGA memories, which is a fast procedure. However,
DFA confronts the state explosion problem. Although in [9],
a regex decomposition method is used to tackle the state
explosion problem, the space complexity of DFA is still too
high for FPGA implementation. In the FPGA DFA scheme
proposed in this paper, the space complexity is further reduced
significantly. Consequently, our proposed novel scheme can
achieve much better matching performance with close hardware resources.
In recent years, there have been researches [10], [11] into
accelerating network intrusion detection based on FPGAs from
the system view. Zhao et al. [10] design the Pigasus IDS
employing an FPGA-CPU architecture. Pigasus has implemented the majority of Snort functionalities (including TCP
reassembly, fast string pattern matching, and packet header
matching) on the FPGA. Packets that pass the fast string
pattern matching and header matching are sent to the CPU for
regex matching. Pigasus claims that it can achieve 100 Gbps
packet processing throughput with one server and an FPGA
board. However, Pigasus focuses on accelerating Snort NIDS
from a system view and only employs some other available
regex matching engine.
Alibaba Group proposes Fidas [11], which is an FPGAbased intrusion detection offload system and has been deployed in their production data center. Unlike Pigasus, Fidas
fully offloads the primary NIC, rule pattern matching, and
traffic rate classification with no CPU overhead. Fidas uses a
multi-level filter-based approach for efficient regex processing.
It is reported that Fidas can achieve about 80 Gbps packet processing throughput in the paper. Although these system works
like Pigasus and Fidas remarkably reduce CPU overhead by
offloading packet processing to FPGA, they only support
simple regex rules with many string fragments. As network
threats become more complex and diverse, network intrusion
detection gradually needs to support more and more complex
regex rules. This paper proposes XAV trying to achieve highperformance regex matching for large-scale and complex rulesets.
2) TCAM-based schemes: Some REM schemes [12], [32],
[33] employ TCAMs to perform regex matching. TCAMs are
very effective for storing the state transition table of DFA.
Employing TCAM to perform DFA matching only needs
one TCAM query for handling each byte of input. However,
TCAM-based REM schemes confront a similar problem with
DFA compression methods: TCAMs can not help DFA tackle
the state explosion problem. What is worse, although TCAMbased REM schemes can easily achieve a high matching
throughput, they are very power-hungry for each TCAM query
is a brute-force search of every TCAM memory entry.
3) Memory-centric architecture schemes: In recent years,
several memory-centric architecture schemes [13], [34], [35]

are proposed to improve the performance of automata processing. Different from employing existing hardware like
FPGA and TCAM, these memory-centric architecture schemes
redesign hardware to fit regex matching. The Micron Automata
Processor (AP) [34] provides a DRAM-based dedicated automata processing chip, while CA [35] and Impala [13] modify
the structure of the last-level cache SRAM for automata
processing. Despite up to 80 Gbps throughput being claimed
by Impala, the dedicated hardware design is far from the
market.
III. M OTIVATIONS FOR XAV
A. Anchor DFA
As a popular way to perform regex matching, DFA suffers
from the state explosion problem 1 and requires impossibly
large memory for supporting large-scale rule-sets. However,
it is found that when a regex set is compiled into an anchor
DFA (see definition 1), the state explosion tends not to happen,
and the size of the anchor DFA is small. It is reasonable
that the semantics of one regex starting with anchor ”ˆ” is
much simpler than the same regex starting with ”.*” 2 . The
simpler the semantics of a regex set, the smaller the number of
DFA states required to represent the semantics. Therefore, the
anchor DFA compiled from all regexes starting with anchor
”ˆ” is much smaller than the DFA compiled from all the same
regexes starting with ”.*”.
Definition 1 (anchor DFA). An anchor DFA refers to a DFA
compiled from regexes all starting with anchor ”ˆ”. To compile
a set of regexes into an anchor DFA, each regex is added with
one anchor ”ˆ” to the head first.
Experiments are carried out to show that anchor DFA can
significantly reduce the number of states. Four rule-sets from
Regex [36] benchmark suit are used for the experiment, and
each of them is compiled into a DFA and an anchor DFA,
respectively. The size comparison of the DFA and the anchor
DFA on each rule-set is shown in Table I. For each of the four
rule-sets, the anchor DFA size is much smaller than the DFA
size. For each of the last three rule-sets, when all regexes in the
rule-set are compiled into a DFA, state explosion occurs and
the DFA construction is far from completion even when the
state number exceeds one million. However, while compiling
all regexes in each of the three rule-sets into an anchor
DFA, DFA construction is finished when the size reaches only
tens of thousands. From the above experiments, it is obvious
that anchor DFA can effectively alleviate the state explosion
problem of DFA.
Although anchor DFA can greatly reduce the number of
DFA states, the semantics of each regex has changed in anchor
DFA for that an anchor ”ˆ” is added to the head. To maintain
the semantics of original regexes starting with ”.*”, anchor
DFA should start a matching thread (see definition 2) at each
position of the input. For example, if RE1 =”abc” is compiled
1 The famous state explosion problem of DFA is introduced in Appendix

A.
2 Note that the starting ”.*” of one regex is usually omitted in practical
rule-sets.

<!-- page 4 -->

4

TABLE I
C OMPARISON OF THE DFA SIZE AND THE ANCHOR DFA SIZE ON
DIFFERENT RULE - SETS
Rule-set
bro217.re
ranges1.conf 300-0.re
ranges05.conf 300-0.re
dotstar0.3.conf 300-0.re

DFA
6533
>1000000
>1000000
>1000000

Anchor DFA
2155
11509
11639
43679

a b a b c
matching thread 1
matching thread 2
matching thread 3
matching thread 4
matching thread 5
Fig. 1. Example of Anchor DFA matching procedure.

into an anchor DFA A1 , now A1 can only match ”ˆabc”. For
the input text T1 =”ababc”, to find the occurrences of RE1 in
T1 , A1 should start a new matching thread at each position
of T1 as shown in Fig.1. And can only matching thread 3
starting at position 3 find the occurrence of RE1 . If only one
matching thread (i.e., matching thread 1 in the figure) is started
at position 1, which is the same as the DFA matching scheme,
then no occurrence of RE1 will be found.
Definition 2 (matching thread). For an anchor DFA, a matching thread refers to the entire DFA matching process, which
starts from the initial state and ends when arriving at the
dead/trap state or the end of the input.
Because each matching thread can require a number of
state transitions, starting a new matching thread at each
position of input indicates that anchor DFA has extremely high
time complexity. Therefore, directly applying anchor DFA to
regex matching is not an advisable option. However, some
optimizations can greatly reduce the time complexity of anchor
DFA.
B. Optimizations to Anchor DFA
In this section, two optimizations are proposed to reduce the
time complexity of anchor DFA. The first is to employ one
pre-filter to prevent anchor DFA from starting new matching
threads in most positions. The second is to limit the access
depth (see definition 3) of each matching thread through regex
decomposition.
Definition 3 (access depth). The access depth of one matching
thread refers to the total number of state transitions needing
to be performed by the matching thread.
1) Pre-filter: In most practical DPI-based network security
applications, input traffic rarely matches any rule in the ruleset [37]–[39]. Further, it has been observed that one pre-filter
built from string fragments of each rule can be very efficient
[6], [21], [40], [41]. Therefore, using the string fragments of
each regex, one pre-filter can be constructed to prune most
anchor DFA matching threads.

Taking RE2 =”ab.*cd” as an example, while RE2 is compiled into anchor DFA A2 , the string fragment S=”ab” can be
used to construct a pre-filter of A2 . Assume that the input text
is T2 =”aabcd”. As S is only matched by the pre-filter starting
at position 2 of T2 , only one matching thread of A2 needs to
be started at position 2 in the whole matching process. For
all other positions that are not matched by the pre-filter, no
matching thread of A2 is required.
A number of algorithms for multi-string pattern matching,
such as [42]–[46], can be selected as the pre-filter. In this
paper, the xor filter newly proposed in [47] is used for that
the xor filter is fast and small. What is more, the xor filter
is friendly for hardware implementation. Therefore, implementing xor filters in FPGAs can help XAV achieve a high
matching throughput.
2) regex decomposition: Although the introduction of a
pre-filter can significantly reduce the number of anchor DFA
matching threads, the access depth of some unpruned matching
threads may be very large, thereby severely downgrading the
matching performance.
Taking the example above, when S is matched by the prefilter, as ”.*” (i.e., dot-star) in RE2 means to match any
character any time, the activated matching thread of A2 will
run all the time until one match occurs or the input ends.
Besides the dot-star components in one regex, all kinds of
long regex components (see definition 4) can possibly lead to
a large access depth for one anchor DFA matching thread.
Definition 4 (long regex component). One long regex component refers to a regex component that can cause a large
expected access depth for one matching thread. Note that the
minimum expected access depth for a long regex component
can be flexibly defined in the specific implementation. Generally, long regex components include dot-star, almost dotstar such as ”[ˆ\n]+”, and big character classes with a large
counting constraint such as ”[ˆ\n]{500}”.
To limit the expected access depth of each activated matching thread, each regex can be decomposed into lsREs (see
definition 5) and lusREs (see definition 6). Only the lsREs of
each regex are employed to construct the anchor DFA, while
the matching of lusREs is left to a further verification stage.
In this way, the access depth of each anchor DFA matching
thread is limited to a small value, thereby reducing the time
complexity of anchor DFA.
Definition 5 (lsRE). One lsRE 3 refers to a regex fragment
with no long regex component. For example, ”abc” and
”user=[0-9]{32}” are both lsREs.
Definition 6 (lusRE). One lusRE 4 refers to a regex fragment
that contains at least one long regex component. For example,
”ab.*cd” and ”ab[ˆ\n]{500}” are both lusREs.
For example, RE2 =”ab.*cd” can be split into two lsREs,
lsRE1 =”ab” and lsRE2 =”cd”, and one lusRE, lusRE1 =”.*”.
3 LsRE is an abbreviation of Length reStricted REgex, which means that
the expected length of each matching text of lsRE is small.
4 LusRE is an abbreviation of Length UnreStricted REgex, which means
that the expected length of each matching text of lusRE can be very large.

<!-- page 5 -->

5

0UW\[
Z[YLHT

?VYMPS[LY

4H[JOPUN
WVZP[PVUZ

(UJOVY+-(

SZ9,
MYHNTLU[
TH[JOLZ

=LYPMPJH[PVU

9LNL_
TH[JOLZ

Fig. 2. The architecture of XAV.

We complile lsRE1 and lsRE2 into one anchor DFA A3 .
Now, the maximum access depth of each matching thread of
A3 is limited to 3, which is the length of lsRE1 or lsRE2
plus one. As for the matching of lusRE1 , only when lsRE1
and lsRE2 are sequentially matched by A3 , will the further
verification stage perform it.
The details of how to achieve the regex decomposition and
further verification will be described in Section IV.

C. XAV
Combining the above two optimizations with anchor DFA,
this paper proposes a novel REM scheme named XAV, which
represents the three matching stages of the scheme, Xor filter,
Anchor DFA and Verification. Anchor DFA tackles the state
explosion problem to support large-scale rule-sets, while the
xor filter and the verification stage reduce the time complexity
to achieve high matching performance. The architecture of
XAV is as shown in Fig.2.
Firstly, the input is matched by the xor filter, where matches
occur only at a small proportion of input positions. Then, these
suspicious positions are transmitted to the anchor DFA engine,
which will start a new matching thread at each of them. The
anchor DFA will find all occurrences of each lsRE fragment
and report them to the further verification engine. In the
verification engine, the matching results of each lsRE fragment
are recorded and verified if any original regex matches the
input.
To clarify the overall matching procedure of XAV, a simple
example is given below. For the rule-set with one regex,
RE2 =”ab.*cd”, and the input T2 =”aabcd”, the xor filter will
report that the input matches at positions 2 and 4. Through
two matching threads starting at positions 2 and 4, the anchor
DFA can find that lsRE1 =”ab” matches at position 2 and
lsRE2 =”cd” matches at position 4. When the verification
engine receives that lsRE2 =”cd” matches at position 4, since
it knows that lsRE1 has been matched at position 2, it can
determine that RE2 matches the input.
For the use of anchor DFA, the state explosion problem can
be avoided when compiling large-scale rule-sets. Therefore,
XAV has a low space complexity and can support the parallel
matching of hundreds to thousands of regexes. Besides, benefitted from the two optimizations to anchor DFA, the number
of activated anchor DFA matching threads is small, and the
expected access depth of each matching thread is limited.
Consequently, XAV has a low matching time complexity and
can deliver a high matching performance. The detailed implementation of the XAV scheme is described in the following
two sections.

9,!\ZLY$BHM Dbd
9,!(<;/CZBECUDbd
9,!7(9;0(3)6+@
9,!CKbdC_TPJC_J
¯
YLNL_ZL[

SZ9,!\ZLY$BHM Dbd
SZ9,!(<;/CZ
SZ9,!7(9;0(3
SZ9,!)6+@
SZ9,!CKbdC_TPJC_J
¯
SZ9,MYHNTLU[ZL[

?VYMPS[LY

(UJOVY+-(

9LNL_
KLJVTWVZP[PVU
S\Z9,!BECUDbd
S\Z9,!
¯

=LYPMPJH[PVU
LUNPUL

S\Z9,MYHNTLU[ZL[

Fig. 3. The compilation procedure of XAV.

IV. C OMPILATION OF XAV
This section elaborates on the compilation procedure of
XAV, which mainly includes regex decomposition, xor filter
building, anchor DFA construction, and the verification engine
building.
The overall compilation procedure is shown in Fig.3. Firstly,
each regex is split into lsRE fragments and lusRE fragments
through the regex decomposition. Then the xor filter and
anchor DFA are constructed from the lsRE fragment set. While
the lusRE fragment set is employed to build the verification
engine.
A. Regex decomposition
To implement regex decomposition, each regex is first
parsed to a component tree, which will be introduced below.
By identifying long regex components in the component
tree, the regex is then split into lsRE fragments and lusRE
fragments at the position of each long regex component.
1) Regex parsing: According to regex semantics, four component classes are defined in this paper, which are character class components, concatenation components, alternation
components, and repetition components, respectively. Given
regex components R and S, the definitions of the above four
component classes are listed below:
• character class component: A character class component refers to the most basic regex component that
matches a set of literal characters. Character class components include single literal characters (such as ”a”) and
character classes (such as ”[0-9]”).
• concatenation component (RS): A concatenation component refers to a set of regex components that are
concatenated one by one. To successfully match the
concatenation component, one text needs to match the set
of regex components sequentially. For the concatenation
component of R and S, each accepted text of (RS) should
match R and S in order.
• alternation component (R|S): An alternation component refers to the set union of a regex component set.
If one text matches any one regex component in the
regex component set, the text matches the alternation
component. For the alternation component of R and S,
each accepted text of (R|S) matches either R or S.
• repetition component (R{m, n}): A repetition component refers to a component with counting constraints. For

<!-- page 6 -->

6

(ab|cd)e[^\n]{100}

(ab|cd)

e

[^\n]{100}

character class component
concatenation component

ab

cd

[^\n]
alternation component

a

b

c

d

repetition component

Fig. 4. The component tree for RE3 .

the repetition component of R, with counting constraints
{m, n} (0 ≤ m ≤ n and n can be infinite), each accepted
text of R{m, n} should match R consecutively for m to
n times.
Based on the concepts of the four regex components,
each regex can be parsed into a component tree. For
example, the regex component tree parsed from regex
RE3 =”(ab|cd)e[ˆ\n]{100}” is as shown in Fig.4.
2) Regex splitting: Based on the component tree of one
regex, it is easy to identify where one long regex component
is and therefore to split the regex. The definition of long
regex components has been given in definition 4. In our actual
implementation, one character class component matching more
than 128 out of 256 characters with a counting constraint larger
than 50 is regarded as a long regex component.
To split one regex into lsRE fragments and lusRE fragments,
the corresponding component tree is first traversed to find
all the positions of long regex components. Then, the regex
is divided into lsRE fragments and lusRE fragments at the
position of each long regex component. Finally, the unfriendly
lsRE fragments will be merged into neighbor lusRE fragments.
Unfriendly lsRE fragments refer to those lsRE fragments that
can not adapt to the xor filter, which will be discussed in the
remainder of this paper.
Taking RE3 =”(ab|cd)e[ˆ\n]{100}” as an example, the repetition component ”[ˆ\n]{100}” will be identified as a long
regex component. Then, according to the position of the long
regex component, RE3 is split into one lsRE fragment (i.e.,
”(ab|cd)e”) and one lusRE fragment (i.e., ”[ˆ\n]{100}”).

B. Xor filter construction
After decomposing each regex in the rule-set, one lsRE
fragment set and one lusRE fragment set are generated. The
lsRE fragment set is employed to construct the xor filter and
the anchor DFA, while the lusRE fragment set is used for the
building of the verification engine.
As xor filter only supports the matching of ldREs (see definition 7). For building the xor filter, given a length limitation 5 ,
the ldRE fragment with the highest pre-filtering effectiveness
is first extracted from each lsRE fragment.
5 The length limitation of each ldRE, L , is set to 8 in detailed implemenT
tation.

Definition 7 (ldRE). One ldRE 6 refers to a regex fragment
that comprises only character class components. Note that
some repetition components, of which the sub-component is
a character class component and the counting constraint is
a fixed number, can be regarded as multiple character class
components. For instance, ”abc”, ”ab[a-z]”, ”ab[a-z]{3}” are
all ldREs.
Taking the lsRE fragment set in Fig.3 as an example, ldREs ”user=”, ”AUTH\s”, ”PARTIAL”, ”BODY”, and
”\x00mic\x7c” will be extracted from lsRE1−1 , lsRE2−1 ,
lsRE3−1 , lsRE3−3 and lsRE4−1 in the compilation procedure, respectively.
Note that every lsRE fragment is ensured in the regex
decomposition procedure to have at least one ldRE fragment
with high pre-filtering effectiveness 7 . Otherwise, the lsRE
fragment is denoted as an unfriendly lsRE fragment, which
is merged into neighboring lusRE fragments as described in
Section IV-A2.
After extracting one proper ldRE from each lsRE fragment,
all the ldREs can then be employed to construct the xor filter
as depicted in [47]. Since each xor filter unit can only support
the matching of patterns with the same length, multiple xor
filter units are used to support different lengths as shown in
Fig.5. The detailed implementation of the xor filter will be
further discussed in Section V.
C. Anchor DFA construction
According to the ldRE position of each lsRE, we divide
each lsRE fragment into two parts, the front part, and the
back part. The front part for one lsRE starts from the head
and ends at the end of the ldRE, while the back part is the
remaining part of the lsRE. For lsRE1−1 =”user=[a-f0-9]{32}”
in Fig.3, the front part is ”user=” and the back part is ”[a-f09]{32}”. While for lsRE4−1 =”\d{1,6}\x00mic\x7c”, since
the extracted ldRE (i.e., ”\x00mic\x7c”) ends at the end of
lsRE4−1 , it has only the front part, which is lsRE4−1 itself.
After dividing each lsRE fragment, all front parts of the
lsRE fragment set are compiled into one anchor DFA. While
the back part of each lsRE fragment is compiled into one
anchor DFA. Note that when the xor filter reports a match
at some position pos, to perform the matching of all front
parts for the lsRE set, the anchor DFA should match the input
from pos reversely. Therefore, the front parts of all lsREs
are reversed8 first and then compiled into one anchor DFA,
which is denoted as reverse DFA in the remaining text. As
for the matching of the back part for each lsRE, since the
corresponding anchor DFA only needs to perform it forward
from pos + 1 as typical DFAs, the anchor DFA for each back
part is denoted as one forward DFA in this paper.
6 LdRE is an abbreviation of Length Determined REgex, which means that
each accepted text of one ldRE has the same length.
7 Assuming random traffic, if the matching probability of one ldRE is lower
than PT , the ldRE is thought with high pre-filtering effectiveness. In our actual
implementation, PT is set to 0.0001.
8 A reversed version of one regex matches the reverse of original accepted
texts. For example, the reversed version of lsRE1−1 =”user=[a-f0-9]{32}” is
”[a-f0-9]{32}=resu”.

<!-- page 7 -->

7

Consequently, the anchor DFA engine in XAV includes
one reverse DFA and multiple forward DFAs. The matching
procedure of the anchor DFA engine is as below. When the xor
filter reports some match at a position pos, a matching thread
of the reverse DFA is activated at pos and matches the input
reversely. The reverse DFA can determine whether the input
matches some front part of one lsRE fragment. If the reverse
DFA is matched successfully by the input, the anchor DFA
engine will find if there is one back part corresponding to the
matched front part. If the back part exists, the corresponding
forward DFA for the back part is then used to match the input
from pos+1 forward. If there is no back part to be matched or
the back part is successfully matched, the anchor DFA engine
will report the lsRE matching result to the further verification
engine.
D. Verification engine construction
The verification engine is for the semantic verification of
original regexes, which needs to record lsRE fragment matches
and perform the matching of the lusRE fragment set.
Any regex RE can be regarded as the form of
R1 S1 R2 S2 ...Sn Rn+1 (for 1 ≤ i ≤ n, Ri represents a lusRE
fragment and Si represents a lsRE fragment), where R1 and
Rn+1 may be empty. To match RE, one text should match
R1 , S1 , ..., Sn and Rn+1 in order. As described earlier, the
matching of each lsRE fragment (i.e., Si in RE) is performed
by the xor filter and the anchor DFA engine. When some lsRE
fragment Sk is reported to match the input text T starting at
position pos sk and ending at position pos ek , the verification
engine should perform the semantic verification of RE as
follows.
1) If k =1, the verification engine will check if R1 matches
T ending at pos sk − 1. If R1 is matched, then the
matching result of Sk will be recorded.
2) If k ≥2, the verification engine will first check whether
there is one matching record of Sk−1 . If not, the verification engine will directly ignore the matching result of
Sk . Otherwise, the verification engine will check if Rk
can be matched starting from pos ek−1 +1 and ending at
pos sk − 1. If Rk is successfully matched, the matching
result of Sk will be recorded by the verification engine.
3) If k = n, since Sn is the last lsRE fragment of RE,
the verification engine will further check if Rn+1 is
matched by T starting at pos en +1. If Rn+1 is matched
successfully, it indicates that all fragments of RE have
been matched by T continuously. In this case, RE will
be reported to match the input text T .
As for how to perform the matching of lusRE fragments,
three matching approaches are employed for three kinds of
lusREs, which are listed below.
•

dot-MN (”.{m,n}”): The first kind of lusREs are denoted
as dot-MN, which means that it matches any character for
m to n times (0 ≤ m ≤ n and n can be infinity). To check
the semantics of one dot-MN lusRE, the verification
engine only needs to judge whether the interval of two
lsRE fragment matching results is in (m, n).

Input
stream

Input
scheduler

FPGA

Xor filter N
...
DFU
Xor
filter
(length
=
2) 2
DFU

Xor
filter
(length
= 2) 1
DFU
XFU
(length
= 2)
(length
=
4)
DFU
XFU
(length
= 2)
(length
= 4)
XFU
XFU
(length
= 4)
(length
=
8)
XFU
XFU
(length
= 4)
(length
= 8)
XFU
(length
= 8)
XFU
(length = 8)

Anchor DFA N
...
DFU
Anchor
DFA 2
(length
= 2)
DFU
Matching
positions

Anchor
DFA 1
(length
= 2)
DFU
XFU
(length
= 2)
(length
= 4)
XFU
reverse DFA
(length
= 4)
XFU
XFU
(length
= 4)
lsRE
(length
= 8)
XFU
fragment
(length
= 8)
idXFU

(length = 8)
forward DFA

lsRE
fragment
matches

Verification
engine

CPU

Fig. 5. FPGA-CPU architecture for XAV implementation.

CC-MN (”[c1 c2 ...ck ]{m,n}”): The second kind of lusREs are denoted as CC-MN, which is a character class
with counting constraints. The semantics of one CC-MN
is to match a text with m to n characters in [c1 c2 ...ck ],
where ci (1 ≤ i ≤ k) is a literal character and [c1 c2 ...ck ]
is a character set representing the character class. To
check the semantics of one CC-MN, the verification
engine first judges whether the interval of two lsRE fragment matching results is in (m, n). Then, all characters
between the two lsRE fragment matching results should
be checked if in [c1 c2 ...ck ].
• complex lusRE: The third kind of lusREs are denoted as
complex lusREs, which include all other lusREs except
the first two kinds of lusREs. Since the semantics of one
complex lusRE is much more complex, each complex
lusRE is compiled into a DFA, which is denoted as
lusDFA in this paper, to perform the matching.

•

V. I MPLEMENTATION VIA AN FPGA-CPU ARCHITECTURE
Since the matching logic of the xor filter and the anchor
DFA engine is simple and suitable for hardware implementation. To achieve high matching performance, we use FPGA to
implement the xor filter and the anchor DFA engine. While the
logic of the verification engine is much more complex, which
is suitable for software implementation in the CPU. Therefore,
an FPGA-CPU architecture is designed to implement XAV in
this paper. Through exploiting the massively parallel logic of
FPGA, a high concurrent matching throughput is expected.
A. Architecture overview
The proposed FPGA-CPU architecture for XAV implementation is outlined in Fig.5. Firstly, the input packet stream is
distributed to multiple matching units, where each matching
unit comprises an xor filter and an anchor DFA engine. Each
packet to be searched is then scanned by the xor filter in each
matching unit. When some match occurs in the xor filter, the
match position is reported to the anchor DFA in the same
matching unit. The anchor DFA will start a matching thread
at that position. If one lsRE match is found by the anchor
DFA, the lsRE matching result is then reported to the CPU
for further verification.
Note that when the CPU performs the verification of the
semantics of CC-MN and complex lusREs, it needs to scan
the packet content. Therefore, for those lsRE fragments that

<!-- page 8 -->

8

anchor DFA 1
match
positions

reverse DFA

lsRE
fragment id

forward DFA

lsRE fragment
matches

state transition query
state transition query

anchor DFA 2
match
positions

reverse DFA

lsRE
fragment id

state transition table
(STT)
STT copy 1

forward DFA

lsRE fragment
matches

state transition query
state transition query

STT copy 2

...
anchor DFA N
match
positions

reverse DFA

lsRE
fragment id

...
forward DFA

lsRE fragment
matches

STT copy M
state transition query
state transition query

Fig. 6. All anchor DFAs share M state transition tables.

will cause CC-MN or complex lusRE semantic verification,
once a match occurs, the packet needs to be transmitted to
the CPU. When a large proportion of traffic is required to
be processed in the CPU, the CPU overhead will be too
heavy to make the overall REM scheme feasible. However,
for practical applications, the practical traffic rarely matches
one rule. Besides, when performing regex decomposition, the
lsREs with low pre-filtering effectiveness have been merged
into neighboring lusREs. Therefore, only a small proportion
of traffic needs to be sent to the CPU in our scheme, and the
CPU overhead is small, which will be shown in the evaluation
section.
B. Xor filter implementation
In our detailed implementation, each xor filter comprises
one direct filter unit (DFU) and two xor filter units (XFU) as
shown in Fig.5. The direct filter unit is essentially a bitmap of
size 216 , so it can be applied for matching ldREs with a length
of 2. The two xor filter units are used for matching ldREs of
length 4 and length 8 respectively. For ldREs with a length
other than 2, 4, or 8, they will be cut to their nearest length
in {2, 4, 8} in the compilation procedure.
It is noted that the xor filter does not support the matching
for ldREs of length 1. Since the matching probability for one
1
ldRE of length 1 is at least 256
, which is too large to avoid
the frequent activation of the anchor DFA engine.
C. Anchor DFA implementation
In each matching unit, the anchor DFA engine comprises a
reverse DFA and a forward DFA. It is noticed that multiple
forward DFAs generated in the compilation stage can be
regarded as a forward DFA by merging the state transition
tables when performing matching. Our implementation of the
anchor DFA engine is as shown in Fig.6.
For a large-scale rule-set containing thousands of regexes,
the state transition table (STT) of the anchor DFA can consume
a lot of memory. However, the memory resource is limited,

and allocating one copy of STT for the anchor DFA in each
matching unit is infeasible. Fortunately, the anchor DFA in
each matching unit is active only for a small part of the time.
Hence, the N anchor DFAs in N matching units can share a
total of M copies of STT (M ≤ N ) 9 as shown in Fig.6. Each
state transition query emitted from all anchor DFAs (including
the forward DFAs and the reverse DFAs in all matching units)
is transmitted to a query scheduler, which will finish the query
with an idle STT copy and return the query result to the
corresponding anchor DFA.
Through state transition table sharing, the number of STT
copies can be greatly reduced. However, for rule-sets containing thousands of regexes, if no compression techniques are
employed, a few STT copies will still occupy up to hundreds
of megabytes of memory, which far exceeds the internal
storage of current FPGAs or the storage of fast external
SRAM. Therefore, compressing the STT of the anchor DFA is
necessary. In our implementation, a hybrid STT compression
scheme, which combines the perfect hashing method [48] and
the bitmap encoding method [49], is utilized. Experiments
show that the hybrid compression scheme can effectively
reduce the memory cost of anchor DFA by up to 98%.
VI. E VALUATION
In this section, the proposed REM scheme, XAV, is comprehensively evaluated with both real and synthetic traffic and
rule-sets.
A. Experimental settings
1) Platform: The experimental environment that evaluates
the performance of XAV is shown in Fig.7. The compiler and
the verification engine of XAV are implemented with C++
11. Experiments are carried out using an x86 machine with
128 GB DRAM. The CPU of the machine is a 2.5 GHz Intel
Xeon E5-2678 and the operating system is CentOS 7.5. The
xor filter and anchor DFA matching are performed with Intel
Stratix 10 SX850 FPGA. In our implementation, the FPGA
frequency is 200 MHz. As for the FPGA resource utilization,
96% (807K/841K) logic elements and 75% (6519KB/8692KB)
memory bits are utilized.
2) Rule-sets: Nine regex rule-sets from Regex [50] and
ANMLZoo [51] benchmark suits are used in our evaluation.
The nine regex rule-sets show different characteristics, which
are shown in Table II. The scale and complexity of each ruleset are represented by the number of compiled NFA states and
NFA transitions. From the table, the scale and complexity of
rule-sets increase from top to bottom. Bro217 rule-set is the
smallest and simplest one, while Snort rule-set is the largest
and most complex one.
3) Traffic: Two types of network traffic are used in evaluating the performance of XAV. The first type of traffic is 13.7
MB packets randomly generated and labeled as Random. The
second type of traffic is realistic packets captured in a campus
network 10 . Realistic traffic is divided into three parts and they
9 In our detailed implementation, N is 64 and M is set to 6.
10 The realistic network traffic was collected from volunteers, and the use
of it has been approved by them.

<!-- page 9 -->

9

x86 machine
(Intel Xeon E5-2678)
packet
emitter

verification
engine
pcie queues

......
64 matching units

FPGA
(Intel Stratix 10 SX850)
Fig. 7. The experimental environment which evaluates the performance of
XAV.
TABLE II
T HE CHARACTERISTICS OF TEST RULE - SETS .
Rule-set
Bro217
Ranges1
Ranges05
Dotstar03
Dotstar06
Dotstar09
PowerEN
ClamAV
Snort

#Rules
217
299
300
300
300
299
2860
515
3170

Avg. length
12.33
41.99
42.19
40.29
41.91
40.98
17.70
96.07
62.24

#NFA states
2864
13338
13261
13049
13721
13580
47492
52040
212731

#NFA transitions
6583
19474
18080
41977
65447
88103
239344
1435257
1586731

are labeled as Reality-1, Reality-2, and Reality-3, respectively.
The sizes of the three parts of realistic traffic are 160.5 MB,
70.2 MB, and 54.7 MB, respectively.
B. Compilation time
Table III shows the compilation time of XAV on test
rule-sets. The compilation procedure of XAV comprises four
stages: regex decomposition, xor filter compilation, anchor
DFA compilation, and verification engine compilation. As
shown in the table, XAV compiles rapidly on test rule-sets.
For the first six small rule-sets, their total compilation time is
almost less than 1 second. For the two medium-scale rule-sets,
PowerEN and ClamAV, the compilation of XAV costs a total
of only several seconds. For the largest rule-set, Snort, XAV
can also finish the entire compilation in about 30 seconds.
Among the four XAV compilation stages, anchor DFA compilation costs most of the overall compilation time. The reason
is that anchor DFA compilation involves the DFA construction
process [52], which runs relatively slowly. Although verification engine compilation can also involve DFA construction
for generating lusDFAs (DFAs for complex lusREs), it usually
takes much less time for that the overall size of all lusDFAs
is much smaller than the size of anchor DFA.
C. Spatial performance
Table IV lists the spatial data of XAV on test rule-sets.
The second column of the table shows the xor filter memory
consumption of XAV. Since each entry in the xor filter consumes an average of only a few bits, the memory consumption

of the xor filter on each test rule-set is small and it varies from
a few kilobytes to a dozen kilobytes.
The spatial data related to anchor DFA are listed in the third
to fifth columns of Table IV. As anchor DFA has simplified
regex semantics, it can avoid the DFA state explosion problem
as stated in Section III-A. Consequently, the anchor DFA state
number of each rule-set is close to the NFA state number
listed in Table II. After state table compression, the anchor
DFA memory consumption for the first six small rule-sets is
dozens of kilobytes. While for the two medium-scale ruleset, PowerEN and ClamAV, the anchor DFA occupies a few
hundred kilobytes of memory. For Snort, the largest test ruleset, the anchor DFA memory consumption can reach 1600
kilobytes. The compression ratio of the anchor DFA state table
is from 0.011 to 0.019 for each rule-set, which means that state
table compression helps reduce the memory consumption of
anchor DFA by 98% to 99%.
Note that as the anchor DFA in each matching unit is active
for only a small part of the time, multiple matching units can
share one copy of anchor DFA STT (state transition table) as
explained in Section V-C. In the implementation, an average
of about every ten matching units is designed to share one
anchor DFA STT. Therefore, the average anchor DFA memory
1
that is listed in
consumption per matching unit is about 10
Table IV.
The spatial data related to the verification engine are listed
in the sixth and seventh columns of Table IV. For all small
test rule-sets and PowerEN rule-set, as no complex lusREs are
generated in the regex decomposition stage, the state number
of lusDFAs is zero and the memory consumption of the
verification engine is small. When no lusDFAs are generated,
the verification engine only mainly involves one mapping table
recording the connection relations of each lsRE. Consequently,
the memory consumption of the verification engine is small
and only a few kilobytes of memory are required. For ClamAV
and Snort rule-sets, as complex lusREs result in the generation
of lusDFAs, the verification engine consumes much more
memory. In order to perform fast verification within the
CPU, no compression techniques are applied for lusDFA STT
and each lusDFA state requires one kilobyte of memory. As
Snort rule-set generates tens of thousands of lusDFA states,
it requires up to tens of MB of memory for the verification
engine. Compared with the memory consumption of at least
hundreds of MB in most existing DFA-based REM schemes,
XAV is believed to have shown excellent spatial performance.
D. Temporal performance
1) Effectiveness of xor filter: The xor filter hit ratio for each
rule-set and each type of traffic is shown in Figure 8.
It can be seen from the figure that the xor filter has high
prefiltering efficiency. The xor filter hit ratio is lower than
0.005 on all test rule-sets and all test traffic, which means that
the xor filter helps reduce the number of anchor DFA matching
threads by more than 99.5%. The figure also demonstrates that
the xor filter hit ratio rises with the increase of rule-set scale
and complexity. It is reasonable for that complex and largescale rule-sets generate more lsRE fragments and therefore

<!-- page 10 -->

10

TABLE III
T HE COMPILATION TIME ( SECONDS ) OF XAV ON TEST RULE - SETS .
Rule-set
Bro217
Ranges1
Ranges05
Dotstar03
Dotstar06
Dotstar09
PowerEN
ClamAV
Snort

Regex
decomposition
0.006
0.024
0.025
0.024
0.025
0.026
0.143
0.102
1.293

Xor filter
compilation
0.003
0.001
0.001
0.002
0.002
0.002
0.019
0.004
0.050

Anchor DFA
compilation
0.159
0.957
0.976
0.940
0.929
0.936
4.750
4.879
27.104

Verification engine
compilation
7.13x10−5
1.09x10−4
9.75x10−5
1.57x10−4
2.12x10−4
3.51x10−4
0.003
0.189
3.652

Total
0.168
0.982
1.002
0.965
0.956
0.964
4.914
5.174
32.100

TABLE IV
T HE SPATIAL PERFORMANCE OF XAV ON TEST RULE - SETS .

Xor filter hit ratio

10 2
10 3

state
number
2302
10759
10724
9804
9692
8900
38698
38078
135296

Anchor DFA
memory
consumption (KB)
24.775
95.850
88.240
88.220
87.261
81.614
299.060
431.008
1601.953

Random
Reality-1
Reality-2
Reality-3

10 4

Bro
21
Ra 7
ng
e
Ra s1
ng
es
Do 05
tst
a
Do r03
tst
a
Do r06
tst
ar
Po 09
we
rE
Cla N
mA
V
Sn
ort

10 5
Rule-set

Fig. 8. Xor filter hit ratio for each rule-set and each type of traffic.

more entries for the xor filter to match. It is also noted that
the filtering effectiveness of the xor filter on random traffic is
obviously higher than that on reality traffic.
2) Anchor DFA matching overhead: The anchor DFA
matching overhead in XAV is represented by the ratio of the
matching bytes of the anchor DFA to the total traffic. As shown
in Figure 9, the ratio of anchor DFA matched bytes on each
rule-set and each type of traffic is lower than 0.05, which is a
quite small value and indicates that every 20 bytes of traffic
can trigger at most one state transition for the anchor DFA.
Consequently, as mentioned in Section V-C, about every ten
matching units can share one copy of the anchor DFA STT,
which is enough to make the anchor DFA not the performance
bottleneck of XAV.

compression
ratio
0.016
0.014
0.013
0.014
0.014
0.014
0.011
0.018
0.019

10 1
10 2

Verification engine
lusDFA
memory
state number consumption (KB)
0
1.680
0
2.570
0
2.578
0
3.273
0
3.969
0
4.656
0
42.508
761
768.555
35306
35447.742

Random
Reality-1
Reality-2
Reality-3

10 3
10 4
10 5

Bro
21
Ra 7
ng
e
Ra s1
ng
es
Do 05
tst
a
Do r03
tst
a
Do r06
tst
ar
Po 09
we
rE
Cla N
mA
V
Sn
ort

Bro217
Ranges1
Ranges05
Dotstar03
Dotstar06
Dotstar09
PowerEN
ClamAV
Snort

Xor filter
memory
consumption (KB)
18.850
8.405
8.404
8.506
8.586
8.647
14.435
9.194
12.646

Ratio of anchor DFA matched bytes

Rule-set

Rule-set

Fig. 9. Ratio of anchor DFA matched bytes for each rule-set and each type
of traffic.

3) Verification engine overhead: The burden of the verification engine on the CPU comes from two aspects: obtaining
partial traffic packets from FPGA and matching partial traffic
bytes. Figure 10 shows the ratio of packets needing to be
transferred from FPGA to CPU on each rule-set and each type
of traffic, and Figure 11 shows the ratio of traffic bytes needing
to be matched by the verification engine.
From the two figures, for rule-sets Bro217, Ranges1,
Ranges05, and PowerEN, the ratios of packets transferred
and bytes matched by the verification engine are both zero,
which indicates there is no verification overhead. The zero
verification overhead for the four rule-sets is because each
regex in these rule-sets is a lsRE itself and does not need

<!-- page 11 -->

11

80

10 2
10 3

60
40
20

0.97

0.36

Rule-set

Fig. 10. Ratio of packets transferred from FPGA to CPU for each rule-set
and each type of traffic.

10 1

Random
Reality-1
Reality-2
Reality-3

10 2
10 3

Bro

21
7

Ra
ng
e
Ra s1
ng
es
Do 05
tst
a
Do r03
tst
a
Do r06
tst
ar
Po 09
we
rE
Cla N
mA
V
Sn
ort

10 4
Rule-set

Fig. 11. Ratio of bytes needing to be matched by the verification engine for
each rule-set and each type of traffic.

to be decomposed. Therefore, the anchor DFA in XAV can
perform the entire matching of each regex in these rule-sets
and no verification is required.
For the other five rule-sets, the ratio of bytes transferred
from FPGA to CPU and the ratio of bytes matched by the
verification engine are all small. Even for the largest Snort
rule-set and realistic traffic, the ratio of bytes transferred and
the ratio of bytes matched by verification are lower than 0.04
and 0.01 respectively. To make the verification engine not the
performance bottleneck of XAV, a few CPU cores are enough
for the small verification overhead. In the evaluation of XAV,
one to six CPU cores are sufficient to reach the maximum
matching throughput for all rule-sets.
E. Comparison with existing REM schemes
1) Comparison with commodity schemes: In our evaluation,
the regex matching performance of XAV is compared with that
of two commodity schemes, Bluefield-2 [53] and Hyperscan
[6]. Bluefield-2 is a data processing unit (DPU) launched by
Nvidia, which can provide a variety of acceleration functions,

Ra

Ra

Bro

es
Do 05
tst
a
Do r03
tst
a
Do r06
tst
ar
Po 09
we
rE
Cla N
mA
V
Sn
ort

es1

ng

Ra

ng

Ra

Bro

21

7

21
7

ng
es1
ng
e
Do s05
tst
a
Do r03
tst
a
Do r06
tst
ar
Po 09
we
rE
Cla N
mA
V
Sn
ort

0

10 4

Ratio of bytes verificated

7
6
5
4
3
2
1

XAV
Bluefield-2
Hyperscan

#CPU cores used in XAV

Random
Reality-1
Reality-2
Reality-3

Throughput (Gbps)

Ratio of packets transferred

10 1

Rule-set

Fig. 12. Throughput comparison with commodity REM schemes on realistic
traffic.

including accelerated regular expression matching. Hyperscan
is the current most efficient software REM solution provided
by Intel. The throughput comparison is shown in Figure 12.
In our implementation, with 64 matching units, XAV can
achieve a maximum throughput of about 75 Gbps11 on all rulesets. In comparison, Bluefield-2 achieves 42 Gbps throughput
for most rule-sets, but suffers performance degradation for
Snort rule-set, on which the throughput is 26 Gbps. What is
worse, hundreds of rules in Snort rule-set are not supported
by the compiler of Bluefield-2. Although XAV has to extract
string fragments from each regex rule to build the xor filter,
only tens of Snort rules containing no string fragment are
not supported by XAV. Therefore, it can be drawn that
XAV achieves better matching throughput and supports more
complex regex rules than Bluefield-2.
For small rule-sets, Hyperscan can achieve a matching
throughput of several to even more than ten Gbps with a
CPU core, which is an excellent performance for software
REM schemes. However, with the rule-set complexity and
scale increase, the matching throughput of Hyperscan drops
significantly and is only 0.97 Gbps on ClamAV rule-set and
0.36 Gbps on Snort rule-set. As a software solution, the
biggest advantage of Hyperscan is that it can support nearly
all Snort rules for software-only agility. It is noted that in the
matching throughput comparison on Snort rule-set, Hyperscan
only compiles the Snort rules supported by XAV for fairness. If
Hyperscan compiles all Snort rules, the matching performance
of Hyperscan will downgrade remarkably to only tens of
Mbps. Although XAV is less flexible than Hyperscan according to supporting as many complex regex rules as possible,
XAV achieves much better regex matching performance than
Hyperscan and therefore can help save a lot of CPU resources
for kinds of network security applications. For small rule-sets,
XAV can reduce the requirement of more than 80% CPU cores
compared to Hyperscan. While for large rule-sets like ClamAV
and Snort, XAV helps to offload more than 95% computation
burden into FPGA.
11 The performance bottleneck lies in the transmission of packets through
PCIE channels.

<!-- page 12 -->

12

TABLE V
C OMPARISON WITH FPGA- BASED SCHEMES ON Snort RULE - SET.
Scheme

Platform

Xie et al. [23], [24]
Yant et al. [7]
Yant et al. [8]
Zhong et al. [9]
XAV

Xilinx Kintex
Xilinx Virtex-5
Xilinx Virtex-7
Intel Arria 10
Intel Stratix 10

Rule-set
size
3379
565
34
3420
3170

Compilation
time
∼10 hours
hours scale
N/A
∼7 minutes
32 seconds

2) Comparison with FPGA-based schemes: Table V compares XAV to other FPGA-based schemes. The data for other
FPGA-based schemes are gained from their papers. From the
table, it shows that XAV has a significantly shorter compilation
time than other FPGA-based schemes. The throughput efficiency metric shows that XAV provides the highest matching
performance under the same hardware resource consumption. Compared to other FPGA-based schemes, XAV achieves
throughput efficiency ranging from 2.5 to 19 times higher,
according to the matching performance achieved per unit of
logic/storage resources.
The first two FPGA schemes in Table V are based on NFA.
Since NFA needs to be mapped into specific FPGA logic for
regex matching and the synthesis of FPGA circuits is quite
time-consuming, the compilation time of the two FPGA NFA
schemes is lengthy and can take several hours. The next two
FPGA schemes and XAV are all based on DFA. As DFA
can be updated by rewriting the internal memory in FPGA,
the compilation of the latter three schemes can avoid the
time-consuming FPGA circuit resynthesis. Consequently, the
compilation time for rule-set updates of the latter three FPGA
DFA schemes is significantly shorter than the first two FPGA
NFA schemes. The third FPGA scheme [8] does not solve
the state explosion problem of DFA, resulting in that it only
supports small rule-sets of tens of regexes. Compared to the
fourth FPGA scheme, because XAV better avoids the state
explosion problem of DFA by utilizing anchor DFA, XAV
generates much smaller DFA than the scheme proposed in [9].
As DFA compilation is also a time-consuming task [52], XAV
takes much less time to complete the construction of smaller
DFA.
Compared to other FPGA schemes, the much higher matching throughput efficiency achieved by XAV is mainly due to
the use of the xor filter. Because each xor filter unit occupies
very little memory and can prevent the majority of traffic from
being matched by the latter stages, tens of xor filter units
can be implemented in FPGA. Consequently, high matching
throughput efficiency can be achieved in XAV.
VII. C ONCLUSION
Regular expression matching plays a crucial role in various network security applications. High-performance regex
matching has always been desired to help implement realtime packet checking. This paper proposes to use anchor
DFA to tackle the state explosion problem of DFA, thereby
supporting large-scale regex rule-sets. To achieve high matching performance, the xor filter and an extra verification stage
are introduced to cooperate with anchor DFA. Consequently,

Throughput
(Gbps)
4.16
10.30
141
6.33
75

Tput efficiency
( Gbps∗states
)
#LU T
∼4.16
10.1
∼0.43
2.97
56.58

Tput efficiency
( Gbps∗states
)
bytes
∼0.21
>0.77
∼0.34
0.23
1.89

the proposed scheme is named XAV according to its three
matching stages: xor filter, anchor DFA, and verification.
Since the xor filter and anchor DFA have simple logic and
are suitable for hardware implementation, an FPGA-CPU
architecture is employed to implement XAV for high matching
throughput. Our evaluation shows that XAV has fantastical
space and temporal complexity. Compared to Bluefield-2 and
Hyperscan, the FPGA-CPU implementation of XAV shows a
much higher matching throughput of up to 75 Gbps for the
large Snort rule-set. Compared to other FPGA-based schemes,
XAV not only performs better in matching throughput but also
consumes fewer hardware resources.
R EFERENCES
[1] Marco Spaziani Brunella, Giacomo Belocchi, Marco Bonola, Salvatore Pontarelli, Giuseppe Siracusano, Giuseppe Bianchi, Aniello Cammarano, Alessandro Palumbo, Luca Petrucci, and Roberto Bifulco. hxdp:
Efficient software packet processing on fpga nics. Communications of
the ACM, 65(8):92–100, 2022.
[2] Tao Wang, Xiangrui Yang, Gianni Antichi, Anirudh Sivaraman, and
Aurojit Panda.
Isolation mechanisms for {High-Speed}{PacketProcessing} pipelines. In 19th USENIX Symposium on Networked
Systems Design and Implementation (NSDI 22), pages 1289–1305, 2022.
[3] C. Xu, S. Chen, J. Su, S. M. Yiu, and Lck Hui. A survey on
regular expression matching for deep packet inspection: Applications,
algorithms, and hardware platforms. IEEE Communications Surveys &
Tutorials, 18(4):2991–3029, 2016.
[4] S. Kumar, B. Chandrasekaran, J. Turner, and G. Varghese. Curing
regular expressions matching algorithms from insomnia, amnesia, and
acalculia. In Acm/ieee Symposium on Architecture for Networking &
Communications Systems, 2007.
[5] R. Smith, C. Estan, S. Jha, and S. Kong. Deflating the big bang: Fast
and scalable deep packet inspection with extended finite automata. In
ACM, pages 207–218, 2008.
[6] Xiang Wang, Yang Hong, Harry Chang, KyoungSoo Park, Geoff Langdale, Jiayu Hu, and Heqing Zhu. Hyperscan: a fast multi-pattern regex
matcher for modern cpus. In 16th {USENIX} Symposium on Networked
Systems Design and Implementation ({NSDI} 19), pages 631–648, 2019.
[7] Y. H. Yang and V. K. Prasanna. High-performance and compact
architecture for regular expression matching on fpga. IEEE Transactions
on Computers, 61(7):1013–1025, 2012.
[8] Jiajia Yang, Lei Jiang, Xu Bai, Huailiang Peng, and Qiong Dai. A highperformance round-robin regular expression matching architecture based
on fpga. In 2018 IEEE Symposium on Computers and Communications
(ISCC), pages 1–7. IEEE, 2018.
[9] Jincheng Zhong, Shuhui Chen, and Biao Han. FPGA-CPU Architecture
Accelerated Regular Expression Matching With Fast Preprocessing. The
Computer Journal, 10 2022. bxac138.
[10] Zhipeng Zhao, Hugo Sadok, Nirav Atre, James C Hoe, Vyas Sekar,
and Justine Sherry. Achieving 100gbps intrusion prevention on a single
server. In 14th USENIX Symposium on Operating Systems Design and
Implementation (OSDI 20), pages 1083–1100, 2020.
[11] Jian Chen, Xiaoyu Zhang, Tao Wang, Ying Zhang, Tao Chen, Jiajun
Chen, Mingxu Xie, and Qiang Liu. Fidas: Fortifying the cloud via
comprehensive fpga-based offloading for intrusion detection: Industrial
product. In Proceedings of the 49th Annual International Symposium
on Computer Architecture, ISCA ’22, page 1029–1041, New York, NY,
USA, 2022. Association for Computing Machinery.

<!-- page 13 -->

13

[12] K. Peng, S. Tang, C. Min, and Q. Dong. Chain-based dfa deflation for
fast and scalable regular expression matching using tcam. In Acm/ieee
Seventh Symposium on Architectures for Networking & Communications
Systems, 2011.
[13] E. Sadredini, R. Rahimi, M. Lenjani, M. Stan, and K. Skadron. Impala:
Algorithm/architecture co-design for in-memory multi-stride pattern
matching. In 2020 IEEE International Symposium on High Performance
Computer Architecture (HPCA), 2020.
[14] K Vinoth Kumar, S Prithi, and K Vinodha. Intelligent systems in latest
dfa compression methods for dpc. In Handbook of Research on Evolving
Designs and Innovation in ICT and Intelligent Systems for Real-World
Applications, pages 129–146. IGI Global, 2022.
[15] K Kumar et al. Network intrusion detection system in latest dfa compression methods for deep packet scruting. In Design, Applications, and
Maintenance of Cyber-Physical Systems, pages 219–243. IGI Global,
2021.
[16] Lei Gong, Chao Wang, Haojun Xia, Xianglan Chen, Xi Li, and Xuehai
Zhou. Enabling fast and memory-efficient acceleration for pattern
matching workloads: The lightweight automata processing engine. IEEE
Transactions on Computers, 72(4):1011–1025, 2022.
[17] S. Kumar, S Dharmapurikar, F Yu, P Crowley, and J Turner. Algorithms
to accelerate multiple regular expressions matching for deep packet
inspection. In ACM, pages 339–350, 2006.
[18] M. Becchi and P. Crowley. A-dfa: A time- and space-efficient dfa
compression algorithm for fast regular expression evaluation. ACM
Transactions on Architecture and Code Optimization, 10(1):1–26, 2013.
[19] R. Antonello, S. Fernandes, D. Sadok, J. Kelner, and Geza Szabo.
Design and optimizations for efficient regular expression matching in
dpi systems. Computer Communications, 61(may 1):103–120, 2015.
[20] D. Ficara, S. Giordano, and G. Procissi. An improved dfa for fast regular
expression matching. Acm Sigcomm Computer Communication Review,
38(5):29–40, 2008.
[21] Mohammad Hashem Haghighat, Zhe Fu, and Jun Li. Hes: Highly efficient and scalable technique for matching regex patterns. In Proceedings
of the 2018 2nd High Performance Computing and Cluster Technologies
Conference, pages 69–78, 2018.
[22] A. X. Liu and E. Norige. A de-compositional approach to regular
expression matching for network security. IEEE/ACM Transactions on
Networking, PP(99):1–13, 2019.
[23] Chunkun Bo, Vinh Dang, Ted Xie, Jack Wadden, Mircea Stan, and
Kevin Skadron. Automata processing in reconfigurable architectures:
In-the-cloud deployment, cross-platform evaluation, and fast symbolonly reconfiguration. ACM Trans. Reconfigurable Technol. Syst., 12(2),
may 2019.
[24] T. Xie, V. Dang, J. Wadden, K. Skadron, and M. Stan. Reapr: Reconfigurable engine for automata processing. In 2017 27th International
Conference on Field Programmable Logic and Applications (FPL),
2017.
[25] C. H. Lin, C. T. Huang, C. P. Jiang, and S. C. Chang. Optimization of
pattern matching circuits for regular expression on fpga. IEEE Transactions on Very Large Scale Integration (VLSI) Systems, 15(12):1303–
1310, 2007.
[26] Yhe Yang, W. Jiang, and V. K. Prasanna. Compact architecture for
high-throughput regular expression matching on fpga. In Proceedings
of the 2008 ACM/IEEE Symposium on Architecture for Networking
and Communications Systems, ANCS 2008, San Jose, California, USA,
November 6-7, 2008, 2008.
[27] H. Nakahara, T. Sasao, and M. Matsuura. A regular expression matching
circuit based on a modular non-deterministic finite automaton with
multi-character transition. Ieice Technical Report, 2010.
[28] N. Yamagaki, R. Sidhu, and S. Kamiya. High-speed regular expression
matching engine using multi-character nfa. In International Conference
on Field Programmable Logic & Applications, 2008.
[29] T. T. Hieu and N. T. Tran. A memory efficient fpga-based pattern
matching engine for stateful nids. In Fifth International Conference on
Ubiquitous & Future Networks, 2013.
[30] Shuhui Chen A and Rongxing Lu B. A regular expression matching
engine with hybrid memories. Computer Standards & Interfaces,
36(5):880–888, 2014.
[31] J. Yang, J. Lei, B. Xu, H. Peng, and Q. Dai. A high-performance roundrobin regular expression matching architecture based on fpga. In 2018
IEEE Symposium on Computers and Communications (ISCC), 2018.
[32] A. Bremler-Barr, D. Hay, and Y. Koral. Compactdfa: Generic state
machine compression for scalable pattern matching. In Infocom, IEEE,
2010.

[33] C. R. Meiners, J. Patel, E. Norige, E. Torng, and A. X. Liu. Fast regular
expression matching using small tcams for network intrusion detection
and prevention systems. In Usenix Conference on Security, 2010.
[34] P Dlugosch, D Brown, P Glendenning, M Leventhal, and H Noyes. An
efficient and scalable semiconductor architecture for parallel automata
processing. IEEE Transactions on Parallel & Distributed Systems,
25(12):3088–3098, 2014.
[35] A. Subramaniyan, J. Wang, Erm Balasubramanian, D. Blaauw, and
R. Das. Cache automaton. In the 50th Annual IEEE/ACM International
Symposium, 2017.
[36] Michela Becchi, Mark Franklin, and Patrick Crowley. A workload
for evaluating deep packet inspection architectures. In 2008 IEEE
International Symposium on Workload Characterization, pages 79–89.
IEEE, 2008.
[37] A. X. Liu and E. Norige. A de-compositional approach to regular
expression matching for network security. IEEE/ACM Transactions on
Networking, PP(99):1–13, 2019.
[38] Michela Becchi and Patrick Crowley. A hybrid finite automaton for
practical deep packet inspection. In Proceedings of the 2007 ACM
CoNEXT conference, pages 1–12, 2007.
[39] Chengcheng Xu, Shuhui Chen, Jinshu Su, Siu-Ming Yiu, and Lucas CK
Hui. A survey on regular expression matching for deep packet
inspection: Applications, algorithms, and hardware platforms. IEEE
Communications Surveys & Tutorials, 18(4):2991–3029, 2016.
[40] Mohammad Hashem Haghighat and Jun Li. Toward fast regex pattern
matching using simple patterns. In 2018 IEEE 24th International
Conference on Parallel and Distributed Systems (ICPADS), pages 662–
670. IEEE, 2018.
[41] Byungkwon Choi, Jongwook Chae, Muhammad Jamshed, Kyoungsoo
Park, and Dongsu Han. {DFC}: Accelerating string pattern matching
for network applications. In 13th {USENIX} Symposium on Networked
Systems Design and Implementation ({NSDI} 16), pages 551–565, 2016.
[42] Alfred V Aho and Margaret J Corasick. Efficient string matching: an aid
to bibliographic search. Communications of the ACM, 18(6):333–340,
1975.
[43] Marc Norton. Optimizing pattern matching for intrusion detection.
Sourcefire, Inc., Columbia, MD, 2004.
[44] Sun Wu and Udi Manber. Fast text searching: allowing errors. Communications of the ACM, 35(10):83–91, 1992.
[45] Iulian Moraru and David G Andersen. Exact pattern matching with
feed-forward bloom filters. Journal of Experimental Algorithmics (JEA),
17:3–1, 2012.
[46] Charalampos Stylianopoulos, Magnus Almgren, Olaf Landsiedel, and
Marina Papatriantafilou. Multiple pattern matching for network security
applications: Acceleration through vectorization. Journal of Parallel and
Distributed Computing, 137:34–52, 2020.
[47] T. M. Graf and D. Lemire. Xor filters: Faster and smaller than bloom
and cuckoo filters. Journal of Experimental Algorithmics, 25(1):1–16,
2020.
[48] C. H. Lin, J. C. Li, C. H. Liu, and S. C. Chang. Perfect hashing based
parallel algorithms for multiple string matching on graphic processing
units. IEEE Transactions on Parallel & Distributed Systems, PP(9):1–1,
2017.
[49] N. Tuck. Deterministic memory-efficient string matching algorithms for
intrusion detection. In Proceedings of the IEEE Infocom Conference,
2004, 2004.
[50] M. Becchi, M. Franklin, and P. Crowley. A workload for evaluating
deep packet inspection architectures. In IEEE International Symposium
on Workload Characterization, 2010.
[51] J. Wadden, V. Dang, N. Brunelle, T. T. Ii, D. Guo, E. Sadredini, K. Wang,
C. Bo, G. Robins, and M. Stan. Anmlzoo: a benchmark suite for
exploring bottlenecks in automata processing engines and architectures.
In IEEE International Symposium on Workload Characterization, pages
1–12, 2016.
[52] Jincheng Zhong and Shuhui Chen. Accelerating dfa construction based
on hierarchical merging. In 2019 IEEE 5th International Conference on
Computer and Communications (ICCC), pages 1360–1365. IEEE, 2019.
[53] Idan Burstein. Nvidia data center processing unit (dpu) architecture.
2021 IEEE Hot Chips 33 Symposium (HCS), pages 1–20, 2021.
[54] John E. Hopcroft, Rajeev Motwani, and Jeffrey D. Ullman. Introduction to Automata Theory, Languages, and Computation, 3rd Edition.
Addison-Wesley, 1979.
[55] Ken Thompson. Programming techniques: Regular expression search
algorithm. Communications of the ACM, 11(6):419–422, 1968.
[56] John Hopcroft. An n log n algorithm for minimizing states in a finite
automaton. Theory of machines and computations, 1971.

<!-- page 14 -->

14

TABLE VI
T HE COMMON METACHARACTERS IN REGEXES

Metacharacter
.
[]
[ˆ]
*
+
{m,}
{m,n}
|
ˆ
$

Semantics
any single character
any single character in square brackets
any single character not in square brackets
match the previous character any number of times
match the previous character once or more times
the previous element is repeated at least m times
the previous element is repeated m to n times
match the preceding element or the following element
match the start position of one string
match the end position of one string

A PPENDIX
P RELIMINARIES
In this section, the background knowledge about regular
expressions and finite state automata is introduced.
A. Regular expressions
A regex is a sequence of characters that define a search
pattern, namely languages or a string set. Each character in
a regex is either a general character or a metacharacter. Each
metacharacter has a special meaning. For example, metacharacter “.” means to match any character, and metacharacter
“*” means to match a character any number of times. Table
VI lists some common metacharacters used in regexes.
B. Finite state automata
Regexes are usually first compiled into a finite state automaton for matching. It is theoretically proved that regexes
and finite state automata are mathematically equivalent [54].
For any regex r, there is always a finite automaton A whose
semantics are the same as r; on the contrary, for any finite
automaton A, there is always a regex r whose semantics are
the same as A.
Nondeterministic Finite Automaton (NFA) and Deterministic Finite Automaton (DFA) are the two most common
automata applied for regex matching. Using the classical
Thompson algorithm [55], a set of regexes can be compiled
into an NFA. Furthermore, through the subset construction
algorithm [54], an NFA can be converted to a DFA. To
eliminate redundant states, one DFA is usually minimized
using the minimization algorithm [56].
NFA and DFA can both be represented by a five-tuple (Q,
Σ, δ, q0 , F), which is interpreted as follows:
• Q: A finite set of states representing the state space of
the automaton.
• Σ : A finite set of input symbols (usually 256 ASCII
characters).
• δ : The state transition function( Q × Σ → Q), indicating
how the state transitions are performed for each input
character.
• q0 : An initial state representing the state in which the
automaton starts to match. (q0 ∈ Q)
• F : The set of matching/accepting states. It indicates that
rule matching occurs when jumping to these states in the
automaton. (F ⊆ Q)

Example
“a.b” matches “aab”, “abb” and “acb”, etc
“[ab]” matches “a” or “b”
“[ˆab]” matches any character other than “a” and “b”
“a*b” matches “b”, “ab”, and “aab”, etc
“a+b” matches “ab”, “aab”, etc
“a{2,}” matches “aa”, “aaa”, and “aaaa”, etc
“a{2,4}” matches “aa”, “aaa”, and “aaaa”
“a|b” matches “a” or “b”
“ˆa” matches the strings start with “a”
“a$” matches the strings end with “a”

TABLE VII
C OMPARISON BETWEEN NFA AND DFA
Automaton
NFA
DFA

Time complexity
O(mn2 )
O(1)

Space complexity
O(mn)
O(2mn )

Taking one DFA as an example to clarify the matching
process of one automaton. Let the input text to be matched
be c0 c1 c2 ...cp . To begin with, the DFA starts from the initial
state q0 , which is called the current active state. For the first
character c0 , the state transition function δ is invoked to get the
next active state, which is supposed as q1 = δ(q0 , c0 ). Then,
the DFA reads the next character c1 , and consults the next
active state, q2 = δ(q1 , c0 ). The process is repeated until the
end of the input text. Assume that the DFA visits a sequence
of states, q0 q1 ...qp during the whole matching process. If qp is
in the accepting state set F, it indicates that the DFA accepts
the input text. Otherwise, the input text is rejected by the DFA.
The matching procedure of NFA is similar to DFA. The
only difference lies in that in the matching process of one
NFA, there can be multiple current active states, and each
state can transit to multiple states through the state transition
function δ.
C. NFA vs DFA
Table VII lists the comparison between NFA and DFA
according to the worst-case time complexity and space complexity. In general, employing either NFA or DFA is difficult
to realize efficient regex matching for large-scale and complex
rule-sets.
NFA usually has a simpler structure than DFA. Theoretical
analysis has shown that for a regex with length n, NFA
has a linear space complexity of O(n). However, multiple
states may be activated simultaneously in NFA. In the worst
case, all states in NFA are active, and each state may jump
to all other states. Therefore, for a regex of length n, the
worst-case time complexity for matching each input character
is O(n2 ). Compiling m regexes into an NFA, the space
complexity increases linearly to O(mn), and the worst-case
time complexity rises up to O(mn2 ), which indicates a low
matching performance.
In contrast, since there is always one active state in DFA
and only one state transition is required per input character, DFA has a low time complexity of is O(1). The low

<!-- page 15 -->

15

time complexity indicates that DFA has a huge advantage
in matching performance compared to NFA. However, the
excellent temporal performance of DFA comes at the cost of
huge space consumption. The subset construction algorithm
actually traverses all possible state combinations in NFA and
transforms each state combination into a new state in DFA.
Therefore, for a regex of length n, the worst-case space
complexity of DFA is as high as O(2n ). For m regexes, the
worst-case space complexity of DFA increases dramatically
to O(2mn ). When compiling a large-scale and complex ruleset into a DFA, the required space usually exceeds the upper
limit of the compiler’s physical memory. In this case, the state
explosion of DFA occurs.
