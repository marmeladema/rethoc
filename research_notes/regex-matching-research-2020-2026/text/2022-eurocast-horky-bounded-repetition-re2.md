---
file: ../pdfs/2022-eurocast-horky-bounded-repetition-re2.pdf
citation: "Automata with Bounded Repetition in RE2. Horký, Síč, Turoňová. EUROCAST 2022, LNCS 13789 (2023), pp. 232–239"
doi: 10.1007/978-3-031-25312-6_27
source: "https://doi.org/10.1007/978-3-031-25312-6_27 (extracted from the proceedings volume, added manually)"
topics: [performance_hardware]
pages: 8
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Automata with Bounded Repetition
in RE2
Michal Horký, Juraj Sı́č, and Lenka Turoňová(B)
Faculty of Information Technology, Brno University of Technology,
Brno, Czech Republic
{xhorky23,sicjuraj,ituronova}@fit.vutbr.cz
Abstract. Regular expression (regex ) matching has an irreplaceable
role in software development. It is a computationally intensive process
often applied on large texts. Predictability of its efficiency has a significant impact on the overall usability of software applications in practice.
A problem is that standard approaches for regex matching suffer from
high worst case complexity. An unlucky combination of a regex and text
may increase the matching time by orders of magnitude. This can be
a doorway for the so-called Regular Expression Denial of Service (ReDoS)
attack in which the attacker causes a denial of service by providing a specially crafted regex or text. We focus on one of the sources of these
attacks, which are regex with bounded repetition (e.g., ‘(ab)100’). Succinct representation and fast matching of such regexes can be archived
by using a novel counting-set automaton. We present a C++ implementation of a matching algorithm based on the counting-set automaton.
The implementation is done within RE2, which is a fast state-of-the-art
regex matcher. We perform experiments on real-life regexes. The experiments show that implementation within the RE2 is faster than the original
C# implementation.

1

Introduction

Regular expression (regex ) matching plays a significant role in software development. They are powerful means of automatically manipulating text, e.g., searching, finding and replacing, data validation, or syntax highlighting [18]. As stated
in [2–4], about 30 − 40% of the Python, Java, and JavaScript software uses regex
matching.
Regexes are only a small part of big projects so they often lack testing and
analysis. However, a single poorly-written regex may lead to catastrophic consequences, such as failed input validation, leaky firewalls and even events such as
the recent catastrophic outage of Cloudflare services [8] or Stack Overflow [1].
The problems are caused by the so-called Regex Denial of Service (ReDoS),
a denial of service attack based on a high-complexity evaluation of matching
regex against a malign text, where an attacker provides a crafted text that
forces the worst-case time complexity of the matching engine.
This work has been supported by the FIT BUT internal project FIT-S-20-6427 and
the Czech Science Foundation (project No. 19-24397S).
c
 The Author(s), under exclusive license to Springer Nature Switzerland AG 2022
R. Moreno-Dı́az et al. (Eds.): EUROCAST 2022, LNCS 13789, pp. 232–239, 2022.
https://doi.org/10.1007/978-3-031-25312-6_27

<!-- page 2 -->

Automata with Bounded Repetition in RE2

233

This may severely impact the performance especially of the backtrackingbased regular matchers (such as regex matching engines of wide-spread programming languages .NET, Java, JavaScript, Perl, Python, etc.) since the evaluation
of vulnerable regexes against the malign input takes polynomial or exponential
time in the length of the input. A backtracking regex engine constructs a nondeterministic finite automaton (NFA) from the regex and then simulates the NFA
on the input text [4].
An alternative approach is to use so-called offline DFA simulation [15] that
uses a precomputed deterministic finite automaton (DFA) with much lower
worst-case complexity (wrt the length of input text). The major drawback of
offline DFA simulation is the state explosion of the DFA construction, which can
cause significant performance issues when using this method in practice [18].
To prevent the state space explosion, some matching engines (such as RE2 [7],
grep [9], or Rust [6]) use a variant of Thompson’s algorithm [17] (an on-the-fly
subset construction). However, they work with highly non-deterministic automata
which can cause a slowdown due to the computation of large DFA states. This problem can be partially solved by caching already visited parts of the DFA (a technique used by modern matchers). A step within a cached part is then a constant
time operation, the same as for the offline DFA simulation. However, regexes that
cause exploding determinization are problematic for all variants, explicit determinization, as well as cached or non-cached NFA simulation [18].
A frequent cause of the DFA explosion are regexes with the bounded repetition. Repeated patterns, such as ‘(ab){1,100}’, can be succinctly expressed by
a novel succinct and fast deterministic machine called the counting-set automaton (CsA) proposed in a recent paper by Turoňová et al. [18]. This machine is
an automaton with so-called counting sets, a special type of registers that can
hold bounded integer values. It also supports a limited selection of simple set
operations that can be implemented in constant time.
The main contribution of this paper is an efficient implementation of regexes
matching using this novel CsA within the state-of-the-art matcher RE2 and its
evaluation against the original CsA-based matcher of [18] implemented in C# and
other state-of-the-art matchers. We show that RE2 extended with CsA is faster
compared to the original C# matcher from [18] and for regexes with bounded
repetition it can beat existing state-of-the-art matchers.

2

Preliminaries

We consider a fixed finite alphabet of characters/symbols Σ (presumably a large
one such as Unicode). Words are sequences of characters from Σ, with the empty
sequence denoted by ε. Languages are sets of words. The operators of concatenation · and iteration ∗ applied on words or languages have the usual meaning.
We consider the usual basic syntax of regexes generated by the grammar
R ::= α | (R) | R R | R|R | R* | R{n,m}
where n, m ∈ N, 0 ≤ n, 0 < m, n ≤ m, and α is a character class, i.e.,
a set of characters from Σ. A character class is most often of the form a, .,

<!-- page 3 -->

234

M. Horký et al.
. ∧ c < 100/c := c + 1
c := 0

q
.

a

[^a]
c := {0}

s
{c ≥ 100}

(a) CA for ‘.*a.{100}’

a

{q}

a/c := {0} ∪ c + 1
{q, s} {Max(c) ≥ 100}

[^a] ∧ Min(c) ≥ 100/
c := {0}

[^a] ∧ Min(c) < 100/
c := c + 1

(b) CsA from determinization of (a)

Fig. 1. CA and CsA. The transitions are labeled by their guard, which specifies
the input character class and possibly restricts counter (or counting set) values, separated by ‘/’ from the counter update. In (b), the notation {0}∪c+1 stands for the set of
values obtained by incrementing each value in c, adding 0, and removing values larger
than the upper bound of the counter, 100 for (b). The edges denoting initial states are
labelled with initial values of the counters. Final states are in (a) and (b) labelled with
an acceptance condition on counters, e.g. {c ≥ 100} in (a).

[a1 -b1 a2 -b2 ...an -bn ], or [^a1 -b1 a2 -b2 ...an -bn ], denoting a singleton containing the character a ∈ Σ, the entire set Σ, a union of n intervals of characters,
or the complement of the same, respectively.
The language of a regex R, denoted L(R), is constructed inductively to
the structure of R, from its atomic sub-expressions, character classes, using
the language operations denoted by the regex combinators. They are understood
as usual: two regexes in a sequence stand for the concatenation of their languages,
‘|’ is the choice or union, ‘*’ is the iteration, and ‘{n,m}’, is the bounded iteration,
equivalent to the union of i-fold concatenations of its operand for n ≤ i ≤ m.
Automata-Based Regex Matching. Matching engines that are based on automata
theory usually work by constructing a NFA for a given regex and then simulating it on a given text by using on-the-fly subset construction, i.e. creating parts
of the DFA needed for matching. This method can be very efficient for regexes
without bounded iteration, as there are constructions such that the size of created NFA is linear to the size of the regex. However, to transform bounded
iteration R{n,m} to NFA, we have to unfold it, i.e. concatenate m copies of
the NFA for R. The constructed automata would be then too big for larger values of m. Therefore, we use approach introduced in [18] where such regexes are
transformed into succinct counting automata and their simulation is achieved by
using deterministic counting-set automata.
Counting Automata. Regexes with bounded iteration can be succinctly represented by using counting automata (CA) [18]. As an example, in Fig. 1a there
is a CA for the regex ‘.*a.{100}’. As seen in the figure, a transition of the CA
can reset a counter to 0, keep it unchanged, increment it, and test whether its
value belongs to a specified constant interval. The values of every counter c
can only reach values in between 0 and some maxc ∈ N (the maximum number which c is compared against). A run of a CA over a word goes through
a sequence of configurations, pairs of the form (q, ν) where q is a control state

<!-- page 4 -->

Automata with Bounded Repetition in RE2

235

and ν is a counter valuation, a mapping of counters to their integer values. For
instance, one of the CA runs from Fig. 1a on the word a100 generates configurations (q, c = 0), (s, c = 0), (s, c = 1), . . . , (s, c = 99), but the CA can postpone
the transition into s arbitrarily, leading to different values of c. It is easy to see
that one can construct an NFA whose set of states is the set of reachable configurations of a CA; the runs of such an NFA would go precisely through the same
configurations as the runs of the CA over the same word.
Counting-Set Automata. Matching with CA can be accomplished by determinizing them to DFA, however, that would still result in the blow-up that
we are trying to evade. We therefore determinize CA into the so-called countingset automata (CsAs) [18]. The CsA is a deterministic machine that simulates
the DFA but achieves succinctness by computing the counter values only at runtime, as values of a certain kind of registers. Since a single DFA state contains
many counter values paired with CA control states, these registers must be capable of holding a set of integer values. We call these registers, which store sets of
integers, counting sets. A transition may then update a counting set c by incrementing all its elements, resetting it to the singleton {0}, adding the element 0
or 1 to it, and test whether the minimal or the maximal value in the set belongs
to some constant interval.1 A counting set for a counter c is also restricted to
only contain values between 0 and maxc (the set-increment operation removes
values greater than maxc ). An example of a CsA is the automaton obtained by
determinizing the CA from Fig. 1a, shown in Fig. 1b. The authors of [18] show
how a CA can be determinised into a CsA of a size independent of the counter
bounds (unlike DFA, which may be exponentially large).

3

Implementation

We have implemented2 CA and CsA-based matching in RE2 [7]. RE2 is an
automata-based C++ regex matching library that implements Thompson’s algorithm. It starts by simplifying the input regex and then compiling it into NFA.
The compilation compiles multi-byte UTF-8 characters down to an automaton
that reads the input one byte at a time, so the UTF-8 decoding is built into
the automaton [14]. To keep the amount of generated transitions small, it does
not generate a new transition for each character (byte), but it labels transitions
with character (byte) classes for which the automaton always behaves the same.
The matching itself then uses an optimized version of Thompson’s on-the-fly
determinization algorithm. RE2 treats the states of the DFA as a cache. When
the cache fills, it frees all the states and starts over again, thus it is able to work
in a fixed amount of memory [14].
1

These operations can be implemented to work in constant time (see [18]), hence
simulation of CsA gives a fast matching algorithm for bounded repetition.
2
The implementation can be found at https://gitlab.com/MichalHorky/DP-re2repository.

<!-- page 5 -->

236

M. Horký et al.

Table 1. Experimental evaluation of CsA-based RE2 and CA (times are given in milliseconds). CA denotes the time from the loading of input regex to the counting automaton
being created. CsA denotes the time of the determinization of the counting automaton.
CsA without timeouts denotes the time of the determinization, which was run only on
regexes on which CA implementation did not suffer from timeout. The timeout was set
to 1–800 s.
CA
CA

CsA

RE2 (CsA)
CA CsA

Mean

10 966 73 433 8

Median

254 11 338 4

Timeouts

0

14 0

CsA (no CA timeouts)

62 709 11 136
1 990

1 050

5

0

For bounded iteration operator R{n,m}, RE2 uses unfolding during parsing which can cause a significant blow-up during determinization, especially for
higher bounds. In fact, RE2 does not support bounds larger than 1000. We therefore implemented support for CsA-based matching of these regexes where we can
divide implementation into three main steps:
1. parsing the regex using the original RE2 implementation,
2. translation of the regex to CA by using generalization (as defined in [18])
of the Antimirov’s partial derivative construction (keeping the byte classes
approach of original RE2), and
3. the CsA-based matching itself using CA obtained in the previous step.
There are two kinds of matching implemented. The first is the so-called full
matching, which matches only the whole input string, and the partial matching, which matches the string anywhere on the line. Similarly to original RE2,
the matching is done on the fly, i.e. we create only those parts of CsA which are
needed for given input text.

4

Experiments

We experimentally evaluated our implementation by
– comparing the efficiency of creating CA from regexes and determinizing them
into CsA with the original C# implementation called CA, and
– comparing the regex matching of our solution with the matcher used in grep,
original RE2 and the matcher CA.
We used the benchmark from [18] which contains regexes with non-trivial
bounded repetition selected (1) from the database coming from an Internetwide analysis of regexes collected from over 190,000 software projects [5]; (2)
from databases of regexes used by network intrusion detection systems (NIDSes),
in particular, Snort [11], Bro [13], Sagan [16], and the academic papers [19,20];
(4) the RegExLib database of regexes [12]; and (5) industrial regexes from [10],
used for security purposes.

<!-- page 6 -->

Automata with Bounded Repetition in RE2
100

100

100

10

10

10

1

1

1

0.1

0.1

0.1

0.01

0.01

0.01
0.01

0.1

1

10

(a) RE2 (CsA) vs. CA

100

237

0.01

0.1

1

10

100

(b) RE2 (CsA) vs. RE2

0.01

0.1

1

10

100

(c) RE2 (CsA) vs. grep

Fig. 2. Comparison of CsA-based matching implemented in RE2 with CA, original RE2,
and grep.

4.1

Comparison with the Matcher CA

We took 37 regexes which were difficult for the original CA-based C# matcher CA
and compared the runtime of creating CA from the regexes and determinizing
them into CsA.
The results in Table 1 show that CsA-based RE2 is faster for both translations
of the regex into CA and their determinization. The determinization part of
the RE2 implementation takes a long time on some regexes on which CA suffers
from timeout, therefore we also give the results of the RE2 implementation for
those regexes on which CA does not suffer from timeout to show that CsA-based
RE2 is much faster.
4.2

Regex Matching Comparison

We compared our CsA-based regex matching implemented in RE2 with original
RE2 and the matchers CA and grep. We used 2320 regexes from the benchmark
set of [18] and we run them within the benchmark tool of [18]. The actual comparison of matching times was based on 2148 benchmarks, where CsA-based RE2
algorithm found the correct number of matches. The results of the original RE2
implementation were considered as a reference for the number of matches. Also,
each of the individual comparisons was made based only on those regexes, for
which both of the compared tools get the correct number of matches. Note that
the comparison with grep and also the overall statistics are done on a smaller
set of regexes. It is because grep uses different semantics of regexes, causing
it to quit after reading only several characters and reporting zero matches on
such regexes. Therefore, regexes for which the time and the number of matches
of grep were zero, and the other tools find a match or their matching time
was larger than zero (i.e. grep quit after reading just several characters while
the other tools try to find a match in the whole input text), were not considered.

<!-- page 7 -->

238

M. Horký et al.

Table 2. The benchmarks statistic of the individual tools. Times are given in seconds,
and the timeout was set to 60 s. The supplied times are based on 1625 benchmarks on
which none of the tools suffers from error or timeout, and the regex suits the grep’s
semantics. For the timeout statistics, the regexes for which the tools suffer from timeout
were also added, resulting in a set of 1640 regexes being used for the timeout statistics.

RE2

grep CA

RE2 (CsA)

Mean

0.153 0.537 0.480 0.399

Median

0

0.06

0.43

0.28

Std. deviation 1.614 3.175 0.207 1.739
Timeouts

1

10

2

2

The result of the comparison between CsA-based RE2 implementation and
other tools in the form of scatter plots is in Fig. 2. Comparing the C# and C++
implementations of CsA-based matching, the C++ implementation was faster for
1908 out of 2145 benchmarks. For original RE2, CsA-based RE2 was faster for
139 benchmarks, while for grep, CsA-based RE2 was faster for 269 benchmarks.
For grep, the total number of benchmarks was 1629 caused by the different
semantics of regexes. Even though original RE2 and grep win more often, there
are clearly regexes, for which CsA-based RE2 implementation is faster than these
two. Those are often the regexes with the counting loops with larger bounds.
Table 2 contains the statistic of matching times of individual tools. Comparing the two CsA-based algorithms, the one implemented within RE2 is faster.
Even though grep wins most of the time compared with CsA-based RE2 (as
shown in Fig. 2c), there are regexes on which CsA-based RE2 is significantly
faster, making the overall mean better for CsA-based RE2. grep also suffers
from the most timeouts compared with other tools.
Original RE2 is the fastest among all the tools. It also suffers from timeout
the least. Considering the plot in Fig. 2b, the speed of original RE2 still drops
on regexes with counting loops with higher bounds on which CsA-based RE2 is
faster. The usage of the CsA-based algorithm in RE2 is determined by a constructor parameter. This allows the developer to choose the CsA-based algorithms
or the original algorithms. If the developer has control over the regexes (i.e.,
the regexes are not supplied by the user), the developer can use the CsA-based
algorithm for the counting-heavy regexes and the original algorithms for the rest
of the regexes, where the numerous RE2 optimizations are faster. This could be
also done automatically, where CsA-based matching would be chosen for example
by some limit on the counter bounds.

References
1. Outage
postmortem.
https://stackstatus.net/post/147710624694/outagepostmortem-july-20-2016, Accessed 14 Mar 2022

<!-- page 8 -->

Automata with Bounded Repetition in RE2

239

2. Chapman, C., Stolee, K.T.: Exploring regular expression usage and context in
python. In: Proceedings of ISSTA 2016. Association for Computing Machinery
(2016)
3. Davis, J.C.: Rethinking regex engines to address redos. In: Proceedings of
ESEC/FSE 2019. Association for Computing Machinery (2019)
4. Davis, J.C., Coghlan, C.A., Servant, F., Lee, D.: The impact of regular expression
denial of service (redos) in practice: an empirical study at the ecosystem scale. In:
Proceedings of ESEC/FSE 2018. Association for Computing Machinery (2018)
5. Davis, J.C., Michael IV, L.G., Coghlan, C.A., Servant, F., Lee, D.: Why aren’t
regular expressions a lingua franca? an empirical study on the re-use and portability of regular expressions. In: Proceedings of ESEC/FSE 2019. Association for
Computing Machinery (2019)
6. docs.rs: regex - rust. https://docs.rs/regex/1.5.4/regex/
7. Google: Re2. https://github.com/google/re2
8. Graham-Cumming, J.: Details of the cloudflare outage on july 2, 2019. https://
blog.cloudflare.com/details-of-the-cloudflare-outage-on-july-2-2019, Accessed 14
Mar 2022
9. Haertel, M., et al.: GNU grep. https://www.gnu.org/software/grep/
10. Holı́k, L., Lengál, O., Saarikivi, O., Turoňová, L., Veanes, M., Vojnar, T.: Succinct
determinisation of counting automata via sphere construction. In: Lin, A.W. (ed.)
APLAS 2019. LNCS, vol. 11893, pp. 468–489. Springer, Cham (2019). https://doi.
org/10.1007/978-3-030-34175-6 24
11. Roesch, M., et al.: Snort: a network intrusion detection and prevention system.
http://www.snort.org
12. RegExLib.com: The Internet’s first Regular Expression Library. http://regexlib.
com/
13. Sommer, R., et al.: The Bro Network Security Monitor. http://www.bro.org
14. Russ, C.: Regular expression matching in the wild (2010). https://swtch.com/rsc/
regexp/regexp3.html, Accessed 18 May 2021
15. Sipser, M.: Introduction to the theory of computation. SIGACT News 27(1), 27–29
(1996)
16. The Sagan team: The Sagan Log Analysis Engine. https://quadrantsec.com/sagan
log analysis engine/
17. Thompson, K.: Programming techniques: regular expression search algorithm.
Commun. ACM 11(6), 419–422 (1968)
18. Turoňová, L., Holı́k, L., Lengál, O., Saarikivi, O., Veanes, M., Vojnar, T.: Regex
matching with counting-set automata. Proc. ACM Program. Lang. 4(OOPSLA)
(2020)
19. Češka, M., Havlena, V., Holı́k, L., Lengál, O., Vojnar, T.: Approximate reduction of
finite automata for high-speed network intrusion detection. In: Beyer, D., Huisman,
M. (eds.) TACAS 2018. LNCS, vol. 10806, pp. 155–175. Springer, Cham (2018).
https://doi.org/10.1007/978-3-319-89963-3 9
20. Yang, L., Karim, R., Ganapathy, V., Smith, R.: Improving NFA-based signature matching using ordered binary decision diagrams. In: Jha, S., Sommer, R.,
Kreibich, C. (eds.) RAID 2010. LNCS, vol. 6307, pp. 58–78. Springer, Heidelberg
(2010). https://doi.org/10.1007/978-3-642-15512-3 4
