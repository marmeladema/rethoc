---
file: ../pdfs/2026-pods-gawrychowski-optimal-enumeration-regular-pattern.pdf
citation: "Optimal Enumeration of Regular Pattern Matches; Gawrychowski, Manea, Sarnighausen-Cahn, Siemer; PODS 2026 (DOI 10.1145/3801908)"
doi: 
source: "https://upenn.ferric.systems/dbscholar/pods_papers/2026/10.1145_3801908.pdf"
topics: [semantics_theory_access]
pages: 25
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Optimal Enumeration of Regular Pattern Matches

PAWEŁ GAWRYCHOWSKI∗, Institute of Computer Science, University of Wrocław, Poland
FLORIN MANEA∗, Department of Informatics, University of Göttingen, Germany
PAUL SARNIGHAUSEN-CAHN∗, Department of Informatics, University of Göttingen, Germany
STEFAN SIEMER∗, Department of Informatics, University of Göttingen, Germany

Enumerating all matches of a regular expression with capture variables (variable regex, for short) to a document
is a fundamental task in textual information extraction, e.g., in connection to document spanners. State-of-the-
art results [Amarilli et al., ACM TODS 2021] show how the matches of a variable regex, representable by a
sequential variable-set automaton, to a document can be enumerated with delay constant in the size of the
document and polynomial in the size of the query, after a preprocessing requiring time linear in the size of
the document and polynomial in the size of the query. However, the polynomial dependency on the size of the
query in both the delay and the preprocessing is actually quite high: at least quadratic.

We consider here the matching problem for regular expressions with capture variables which can be
represented as regular patterns: 𝑤0𝑥0𝑤1𝑥1 · · ·𝑤𝑘𝑥𝑘𝑤𝑘+1, where, for 𝑖∈{0, . . . ,𝑘+ 1}, 𝑤𝑖is a string of terminal
letters and, for 𝑖∈{0, . . . ,𝑘}, 𝑥𝑖is a variable. This problem naturally lies in the intersection of information
extraction and stringology, as it can also be seen as computing all the ways in which 𝑘+ 1 given strings
𝑤0, . . . ,𝑤𝑘+1 occur, in this order and without overlaps, in a given text 𝑇. We provide an optimal enumeration
algorithm for this problem. After a preprocessing requiring linear time in the size of the document 𝑇and
the size of the query pattern 𝛼, we can enumerate all matches with worst-case constant delay (in combined
complexity, i.e., both in the document- and the query-size).

CCS Concepts: • Theory of computation →Design and analysis of algorithms.

Additional Key Words and Phrases: Pattern matching; Enumeration algorithms; Variable regex

ACM Reference Format:
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, and Stefan Siemer. 2026. Optimal Enumeration
of Regular Pattern Matches. Proc. ACM Manag. Data 4, 2 (PODS), Article 112 (May 2026), 25 pages. https:
//doi.org/10.1145/3801908

1
Introduction
A pattern (with variables) is a string which consists of terminal letters (e. g., a, b, c), treated as
constants, and variables (e. g., 𝑥1,𝑥2); formally, a pattern is a string 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1 where, for
𝑖∈{0, . . . ,𝑘+ 1}, 𝑤𝑖is a string over the alphabet of terminal letters and, for 𝑖∈{0, . . . ,𝑘}, 𝑥𝑖is a
variable. A pattern is mapped to a string by substituting its variables by strings of terminals. For
example, 𝑥1b𝑥1bab𝑥2a𝑥2 can be mapped to aabaababbab by the substitution (𝑥1 →aa,𝑥2 →b). If
a pattern 𝛼can be mapped to a string of terminals 𝑇(also called text), we say that 𝛼matches 𝑇,
and a substitution that maps 𝛼to 𝑇is called valid for 𝛼and 𝑇. The problem of deciding whether

∗All authors contributed equally to this research.

Authors’ Contact Information: Paweł Gawrychowski, gawry@cs.uni.wroc.pl, Institute of Computer Science, University of
Wrocław, Wrocław, Poland; Florin Manea, florin.manea@cs.uni-goettingen.de, Department of Informatics, University of
Göttingen, Göttingen, Germany; Paul Sarnighausen-Cahn, paul.cahn@cs.uni-goettingen.de, Department of Informatics,
University of Göttingen, Göttingen, Germany; Stefan Siemer, stefan.siemer@cs.uni-goettingen.de, Department of Informatics,
University of Göttingen, Göttingen, Germany.

This work is licensed under a Creative Commons Attribution 4.0 International License.
© 2026 Copyright held by the owner/author(s).
ACM 2836-6573/2026/5-ART112
https://doi.org/10.1145/3801908

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 2 -->

112:2
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

there exists a substitution mapping a given pattern 𝛼to a given text 𝑇(i.e., a valid substitution for
𝛼and 𝑇) is called the matching problem.

Pattern Matching (Match)
Input: Pattern 𝛼and text 𝑇.
Output: Decide whether there is a valid substitution for 𝛼and 𝑇.

The matching problem Match for patterns with variables can be canonically seen as a task which
requires compiling structured information from an input text document, i.e., a textual information
extraction task. Indeed, a match between a pattern 𝛼(seen as query) and a text𝑇(seen as document)
maps every variable occurring in the pattern to a substring of the text, or, in other words, extracts
and stores in each variable a substring (also known as a span) of the text. Putting this in context,
a pattern can be seen as a regular expression with capture variables (variable regex), and the
matching problem becomes, as such, a particular case of the problem requiring computing a match
between such a variable regex and a document. This is a problem that was heavily investigated (see,
e.g., [2, 19, 22] and the references therein) in relation to the study of document spanners [13], an
information extraction framework whose main idea is (paraphrasing [19]) to use regular languages
to locate the data to be extracted, and variables to store this data. Even more generally, as already
pointed in [19], such matching problems can be seen as MSO queries on strings (where capture
variables are encoded by pairs of first-order variables); see also [6, 11, 27, 33].

While the variable regex matching problem as well as its particular case, the pattern matching
problem, are generally intractable [17, 18], restricted classes of variable regexes for which the
matching problem (or, alternatively seen, information extraction problem) becomes tractable were
considered in the literature (see, e.g., [17–19, 22, 23, 30] and the references therein). Of particular
interest for our work is a class of such variable regexes defined via variable-set automata (or VAs).
Intuitively, VAs are nondeterministic finite automata which have transitions labelled with (opening
and closing) variable-markers. These markers are used to extract, in a run of the VA on an input
text (also called an evaluation of the VA over the input text), substrings of the respective text, and
store them in the variables indicated by the markers. A VA is called sequential if each variable
marker (both opening and closing) occurs at most once on every accepting run on an input text,
and, for each variable, if an opening marker occurs at some point, then the corresponding closing
marker also occurs. For more details on such automata see, e.g., [2, 19, 22]. Checking whether a
string is accepted by a sequential VA (which can be canonically seen as the matching problem for
the variable regexes corresponding to that VA) is clearly tractable (via, e.g., state-set simulation),
the key observation being that a matching algorithm does not need to memorize the assignment of
each variable. Imposing this restriction on patterns for variables yields the well-studied class of
regular patterns, which is also in the main focus of this paper. These are patterns (or, alternatively,
variable regexes) of the form 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1 where, for 𝑖∈{0, . . . ,𝑘+ 1}, 𝑤𝑖is a string over
the alphabet of terminal letters, and, for 𝑖∈{0, . . . ,𝑘}, 𝑥𝑖is a variable, and 𝑥𝑖≠𝑥𝑗for all 𝑖≠𝑗; they
are called regular as the languages described by these regexes are clearly regular, which is not the
case for languages described by arbitrary patterns, which allow repeated variables.

The corresponding matching problem is the following:

Pattern Matching for Regular Patterns (MatchReg)
Input: Regular pattern 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1 and text 𝑇.
Output: Decide whether there is a valid substitution for 𝛼and 𝑇.

This matching problem can be solved in linear time O(|𝑇| + |𝛼|) by a greedy strategy [16].

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 3 -->

Optimal Enumeration of Regular Pattern Matches
112:3

A natural task in the context of textual information extraction, studied in [2, 19, 22] for sequential
VAs, is the computation of all the evaluations of the VA over the input text, and their efficient
enumeration. Initially, in [22] it is shown that one can enumerate the mappings of the variables with
delay linear in the input text and quadratic in the size of the VA. Further, in [19] it is shown that, if
the VA is fixed (or, in other words, if we consider the data complexity perspective), after a processing
linear in the size of the input document, one can enumerate the mappings of the variables with
constant delay. However, both the preprocessing time and the delay depend exponentially on the
size of the VA. Finally, in [2] it is shown that the respective enumeration problem can be solved
efficiently in combined complexity: after a processing linear in the size of the input document and
polynomial in the size of the VA, one can enumerate the mappings of the variables with delay
constant in the size of the input text and polynomial in the size of the VA. Worth noting, it was
already shown in [6, 11, 27, 33] that the enumeration of MSO Queries on strings (and as such also
the computation of all matches of a variable regex to an input text) can be done with constant delay
in data complexity, but such algorithms were not efficient in combined complexity; see the discussion
from [19]. We stress that, while the algorithm of [2] achieves delay independent on the size of
the input text, its dependency on the size of the VA is actually quite high: at least quadratic. We
comment that even checking if a word is accepted by a given nondeterministic automaton [5], or
whether it belongs to a language described by a regular expression (even if the regular expression
is somewhat restricted) [8], requires quadratic time. Thus, it is natural to seek a restricted (but still
nontrivial) class of patterns for which a better time bound can be attained.

As a teaser, we will show that enumerating all the mappings under which a given regular pattern
𝛼matches an input document 𝑇can be done with constant delay in combined complexity, i.e., w.r.t.
both |𝑇| (the size of 𝑇) and |𝛼| (the size of 𝛼), after a linear time preprocessing (again, both w.r.t. |𝑇|
and |𝛼|). Before going more into details, let us describe the setting in which we obtain these results.

Enumeration framework for MatchReg. A match (solution of the matching problem), for an in-
put pattern 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1 and a text 𝑇of length |𝑇| = 𝑛, is defined by a substitution of
the variables 𝑥𝑖by substrings 𝑇[𝑏𝑖: 𝑒𝑖] which is valid, in the sense that 𝑤0𝑇[𝑏0 : 𝑒0]𝑤1𝑇[𝑏1 :
𝑒1] · · ·𝑤𝑘𝑇[𝑏𝑘: 𝑒𝑘]𝑤𝑘+1 = 𝑇. Such a valid substitution (or match of 𝛼to 𝑇) is represented as a
𝑘-tuple (𝑗1, . . . , 𝑗𝑘), called valid embedding of 𝛼in 𝑇, if, for all 𝑖∈{0, . . . ,𝑘}, 𝑥𝑖is mapped to the
substring 𝑇[𝑗𝑖+ |𝑤𝑖| : 𝑗𝑖+1 −1], with 𝑗0 = 1 and 𝑗𝑘+1 = 𝑛−|𝑤𝑘+1| + 1; more precisely, 𝑗𝑖is the
position of 𝑇where the match of the string 𝑤𝑖(or, in other words, its embedding in 𝑇) starts in a
correct matching of 𝛼to 𝑇. Note that two valid substitutions (i.e., matches) are different, meaning
that at least one variable is mapped to two different substrings of 𝑤in the respective substitutions,
if and only if the corresponding embeddings are different.

Our goal is to design an algorithm which takes 𝛼and 𝑇as input, and reports all the matches of 𝛼
to𝑇, without repetitions. Each such match is given as a valid embedding, which in turn is described
by 𝑘numbers. Thus, each valid embedding would need O(𝑘) time to be explicitly written down. As
standard in many other papers in the area of enumeration algorithms (see the handbook [28], the
survey on combinatorial Gray codes [31], as well as any recent paper in this area, for example [3]),
we avoid explicitly writing down a full description of each solution (i.e., outputting the string to
which each variable is mapped). Instead, we maintain an array Sol in which the current valid
embedding is stored. Then, during the execution of the algorithm a procedure report is called,
signalling that the array Sol currently contains a valid embedding, which was not enumerated yet;
we say that, with this call of report, the match corresponding to the respective valid embedding is
reported.1

1Note the similarities with the usage of the Python function yield in the context of Generators.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 4 -->

112:4
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

The motivation behind such a setting is mainly laid out in [28]. From a theoretical perspective,
it allows separating the actual process of generating the embeddings, and, as such, the traversal
of the solution space of our problem, from their output. From a practical perspective, it allows
for pipelining the enumeration and some further processing: paraphrasing Knuth [28], we are
interested in having the solutions to our combinatorial problem momentarily present in a data
structure, allowing other programs to examine them; as another example, we might want to evaluate
each produced solution and report only the most desirable ones, and it seems plausible that the
score of the current solution can be maintained effectively after each change to the Sol array;
alternatively, as yet another example, we might want to only output the differences between two
consecutive solutions. All such cases can be easily formalized in this setting.

Enumeration of Regular Patterns Matches (EnumMatchReg)
Input: Regular pattern 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1 and text 𝑇.
Output: Report all valid substitutions (as valid embeddings) for 𝛼and 𝑇.

To analyse the complexity of such an algorithm, we split its computation in two phases. The first
phase, called preprocessing, consists in the computation done before the first call of report; the
preprocessing time is the time needed, in the worst case, by the respective computation, expressed
as a function in 𝑘and 𝑛. The rest of the computation is the actual enumeration. As defined in
[25, 26, 28], the delay is the time elapsed, in the worst case, between two calls of the report
function, again expressed as a function in 𝑘and 𝑛; note that, as already mentioned above, we focus
here on the time needed to compute the current embedding from the previous one (that is, the time
elapsed between two consecutive calls of report), and do not include the time needed to actually
output that embedding (or some representation of this embedding). The algorithm is said to have
output-linear delay if the delay is O(𝑘) and it is said to have constant delay if the delay is O(1).
Such a constant-delay algorithm has an interesting combinatorial interpretation: enumerating all
valid embeddings in such a way that two consecutive valid embeddings differ only on a constant
number of positions can be seen as a combinatorial Gray code for the class of embeddings of 𝛼in 𝑇:
this is a listing that contains each object from the class exactly once such, that any two consecutive
objects in the list differ only by a small change [31]. In this case, one could also explicitly output in
O(1) time, when report is called, an edit script which summarizes the changes done in the current
valid embedding w.r.t. the previous one.

Before moving on to describing our contribution, let us note that, although [2, 19, 22] (as well as
[6, 11, 27, 33]) do not explicitly consider this enumeration framework, the complexity aspects of
their algorithms hold as described above in this framework as well (most importantly, the delays of
the respective algorithms remain as stated, and do not decrease by the fact that the time needed to
output the solutions is not counted in the delay).

Our results. Note that in the results reported below, which concern enumeration algorithms, both
the preprocessing time and the delay are given in combined complexity, meaning that there is no
hidden dependency on the size of the input pattern (which is, thus, not considered to be constant).

Florenzano et al. [19] mention that an algorithm running in total time O(𝑓(𝑛) + |𝑆|) would be
an ideal solution to the problem of enumerating all matches of a variable regex to a text 𝑇, where
|𝑇| = 𝑛, 𝑓is a function not depending on the size of the output, and |𝑆| is the size of the output,
suggesting that the output is obtained such that the time between generating two of its consecutive
elements is O(1) on average. Thus, our first result, shows that all matches of pattern 𝛼to text 𝑇
can be reported in overall O(𝑛+ |𝑆|) time (we assume, w.l.o.g., that |𝛼| ∈O(𝑛)), where 𝑆is the set
of valid embeddings.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 5 -->

Optimal Enumeration of Regular Pattern Matches
112:5

Theorem 1.1. We can enumerate the set 𝑆of all valid embeddings of 𝛼in 𝑇with constant delay,
after a preprocessing running in O(𝑛+ |𝑆|) time. The used space is O(𝑛+ min{|𝑆|,𝑛𝑘}).

Further, we present an algorithm that still reports all matches of 𝛼to 𝑇in O(𝑛+ |𝑆|) time, but
has the additional properties that the preprocessing time is O(𝑛) and that for all 𝑠≤|𝑆|, the first 𝑠
embeddings are reported in O(𝑛+ 𝑠) time; in other words, this algorithm works with amortized
O(1) delay.

Theorem 1.2. We can enumerate the set 𝑆of all valid embeddings of 𝛼in𝑇with amortized constant
delay, after a preprocessing running in O(𝑛) time. The used space is O(𝑛+ min{|𝑆|,𝑛𝑘}).

Finally, we give an algorithm for the considered problem that works with preprocessing time
O(𝑛) and constant delay. It is worth noting that employing the algorithms from [2, 19] to the
restricted class of regular patterns that we consider would lead to enumeration algorithms with a
delay that depends (polynomially) on the length |𝛼| of the input pattern.

Theorem 1.3. We can enumerate the set 𝑆of all valid embeddings of 𝛼in𝑇with worst-case constant
delay, after a preprocessing running in O(𝑛) time. The used space is O(𝑛min{|𝑆|,𝑛𝑘}).

While Theorem 1.3 improves both previous results, we prefer stating our results in this incre-
mental fashion, which allows us to introduce our tools and their functionality gradually.

The algorithms presented here heavily take advantage, on the one hand, that the strings separating
the variables are fixed (not described by regular expressions), which allows us to use tools from
stringology and efficient data structures, and, on the other hand, by a deeper analysis and usage
of enumeration techniques. This allowed us to obtain (for the first time, as far as we are aware)
optimal algorithms in combined complexity for a non-trivial subcase of the problem of enumerating
all matches of a regular expression with capture variables. While our techniques are tailored
towards the special case of regular patterns and unlikely to be directly applicable to the more
general cases of [2, 19], we hope that they will spark further interest in transferring tools from
stringology to enumerating all matches of a regular expression, for more general classes of regular
expressions. Further, we believe that some of our insights, for example Theorem 3.6, might find
further applications.

As a first step towards future work, we consider a dynamic version of the problem, which requires
building an index for 𝑇, that allows us to enumerate matches of regular patterns. More precisely,
we are given a text 𝑇and then, one by one, regular patterns 𝛼1, 𝛼2, . . . and we want to report, with
constant delay, all matches of 𝛼𝑗to 𝑇, after 𝛼𝑗is read. Ideally, we would like to avoid a linear
time processing of 𝑇for each pattern 𝛼𝑗(as it would be the case if we would apply our previously
mentioned algorithms every time). We obtain the following result.

Theorem 1.4. We can preprocess the text 𝑇in O(𝑛log𝑛) time, so that when given a pattern 𝛼
with 𝑘variables, after an additional processing running in O(|𝛼| + 𝑘log𝑛) time, the set 𝑆of all valid
embeddings of 𝛼in 𝑇can be enumerated with constant delay.

Overview of our algorithms. Our four algorithms are based on a common idea. We can iden-
tify, greedily, a left-canonical valid embedding (ℓ1, . . . , ℓ𝑘) and a right-canonical valid embedding
(𝑟1, . . . ,𝑟𝑘), such that for every other valid embedding (𝑗1, . . . , 𝑗𝑘) we have that ℓ𝑖≤𝑗𝑖≤𝑟𝑖, for all
𝑖∈[𝑘]. Intuitively, to compute the left-canonical embedding, after checking that the prefix 𝑤0 of
the pattern coincides with a prefix of 𝑇, we let ℓ1 to be the leftmost position where 𝑤1 occurs in 𝑇
after the prefix of length |𝑤0|, ℓ2 to be the leftmost position where 𝑤2 occurs in 𝑇after position
ℓ1 + |𝑤1| −1, and, in general, ℓ𝑖+1 is the leftmost position where 𝑤𝑖+1 occurs in 𝑇after position
ℓ𝑖+ |𝑤𝑖| −1; we need to also check that the suffix 𝑤𝑘+1 of 𝛼to the suffix of 𝑇of length |𝑤𝑘| and
this suffix starts after ℓ𝑘+ |𝑤𝑘| −1. The left-canonical embedding is computed in linear time,

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 6 -->

112:6
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

by traversing the text 𝑇left to right, and employing standard string matching algorithms. The
right-canonical embedding can be computed similarly, but scanning 𝑇from right to left. The main
observation is now that, for 𝑖∈[𝑘], every position 𝑗𝑖, with ℓ𝑖≤𝑗𝑖≤𝑟𝑖, where 𝑤𝑖occurs is part of
at least one valid embedding. So, it makes sense to focus on the sets 𝑂𝑖storing all the positions
where 𝑤𝑖occurs in 𝑇, in the range 𝑇[ℓ𝑖: 𝑟𝑖+ |𝑤𝑖| −1], and a first important observation is that
�𝑘
𝑖=1 |𝑂𝑖| = O(𝑘+ |𝑆|), where 𝑆is the set of valid embeddings of 𝛼in 𝑇.

Now, in our first algorithm, from Theorem 1.1, we develop a method to enumerate the elements
of all the sets 𝑂𝑖with worst-case constant delay, after a linear time preprocessing (in which we
construct a series of indexing data structures for strings, allowing us to retrieve efficiently the
occurrences of the strings 𝑤1, . . . ,𝑤𝑘in 𝑇). Then we sort these sets (in linear time w.r.t. their
total size, using radix sort), and show that this is enough in order to enumerate the set of valid
embeddings of 𝛼in 𝑇with worst-case constant delay, using a recursive procedure. This leads to
an algorithm which enumerates the elements of 𝑆in O(𝑛+ |𝑆|) time, overall. However, before the
enumeration of 𝑆starts in this algorithm, we actually use O(𝑛+ �𝑘

𝑖=1 |𝑂𝑖|) = O(𝑛+ |𝑆|) time in
the preprocessing.

To reduce the preprocessing time, while keeping the enumeration efficient, in Theorem 1.2, we
use the first O(�𝑘

𝑖=1 |𝑂𝑖|) enumeration steps to report, with O(1) delay, embeddings of the form
(ℓ1, . . . , ℓ𝑖−1, 𝑗,𝑟𝑖+1, . . . ,𝑟𝑘)) (called simple) and also construct simultaneously the sets 𝑂𝑖. Then, we
can afford to sort the lists 𝑂𝑖again, in linear time O(𝑛+ �𝑘

𝑖=1 |𝑂𝑖|), as this amortizes with the
time spent to enumerate the simple embeddings. Then, we continue the algorithm as in the initial
approach, only taking care that we do not output the simple embeddings again. This algorithm does
not have constant worst-case delay, but it has the property that, after a preprocessing running in
O(𝑛) time, it enumerates the valid embedding with amortized constant delay: the first 𝑡embeddings
are reported in O(𝑡) time.

The next algorithm, from Theorem 1.3, is loosely based on the idea of the previous algorithm,
to retrieve the elements of 𝑂𝑖on the fly, during the enumeration, but avoids computing and
sorting these sets in order to keep the delay constant in the worst-case. Instead, it organizes very
carefully the calls to the enumeration procedure, making sure that in every call, when we set the
𝑖th component of an embedding in order to produce a new valid embedding, we have access to an
element of 𝑂𝑖that can be used to define the respective embedding-component.

Finally, the algorithm from Theorem 1.4, shows that a longer preprocessing time, of O(𝑛log𝑛),
allows us to implement the initial approach with worst-case constant delay, and, moreover, to also
be able to process and answer efficiently multiple queries without needing to reprocess the entire
text. In this case, the idea is to obtain a succinct representation of the sets 𝑂𝑖, allowing us to access
their elements in an order that can still be managed efficiently by our initial enumeration algorithm.

Connections to other areas. As mentioned, our results exhibit a combinatorial Gray code for the
class of embeddings of a pattern in a text. The study of combinatorial Gray codes is a well developed
and intensely studied area of combinatorial algorithms, rooted in the study of Gray codes and with
applications in various other areas of theory, see [31].

Matching a regular pattern to a text can be seen as a generalization of the problem of matching
a subsequence in a text (which is connected to complex event processing and recognition, see,
e.g., [24] and the references therein); indeed, testing whether 𝑢= 𝑢[1] · · ·𝑢[𝑚] is a subsequence
of 𝑇is equivalent to testing whether the pattern 𝑥1𝑢1 · · ·𝑥𝑘𝑢𝑘𝑥𝑘+1 matches 𝑇. Also, it is easy to
observe that the matching problem for a text 𝑇and a variable regex built using only concatenation
(over terminal letters and variables), where, additionally, each variable occurs only once and either
captures a substring of the input regex or is not constrained at all, can be reduced to matching a
regular pattern to the respective text 𝑇, by simply removing every capture variable in the matching

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 7 -->

Optimal Enumeration of Regular Pattern Matches
112:7

step, and then computing the substrings they store according to their scope and the values of the
unconstrained variables. Without going more into details (and using the formalism of [19]) we give
an example: the expression 𝑥{ab𝑧{𝑦bc}}ab𝑣{abab𝑢} can be reduced to the pattern ab𝑦bcababab𝑢,
and for each match of the pattern to the text 𝑇(which defines the substrings substituting the
unconstrained variables 𝑦and 𝑢) we can canonically compute the string extracted by every capture
variable 𝑥,𝑧, 𝑣.

While deciding whether a regular pattern 𝛼matches a text 𝑇is straightforward, enumerating
efficiently all variable-assignments under which 𝛼matches 𝑇can be interpreted as a natural, far
less obvious, stringology problem: enumerate all the ways in which the tuple of strings (𝑤0, . . . ,𝑤𝑘)
occur in the text 𝑇, in this order, without overlaps, such that 𝑤0 is a prefix and 𝑤𝑘a suffix of 𝑇.
Moreover, regular patterns are a well investigated class of pattern languages, especially in the
context of algorithmic learning theory, in particular related to the computation of descriptive
patterns describing a set of strings [4, 15, 34].

Computational model. We assume the standard unit-cost word RAM with logarithmic word-size 𝜔
(so each memory word consists of 𝜔≥log𝑛bits). Standard arithmetical and bitwise operations
(and indirect addressing) on such words are assumed to take O(1) time [20]. To avoid clutter, we
present our enumeration algorithms as recursive procedures, and we assume tail calls to a recursive
function are implemented without adding a new stack frame to the call stack. They can be rephrased
as iterative procedures with the same complexity.

2
Preliminaries
Notation. Let N denote the set of natural numbers. For 𝑚,𝑛∈N, we define the intervals [𝑚:
𝑛] = {𝑚,𝑚+ 1, . . . ,𝑛}, [𝑚: 𝑛) = {𝑚,𝑚+ 1, . . . ,𝑛−1} and [𝑛] = [1 : 𝑛]. Let Σ be a finite alphabet
of terminal symbols. Let Σ∗denote the set of all strings over Σ, including the empty string 𝜀, and
Σ+ := Σ∗\ {𝜀}. For strings 𝑤1,𝑤2, . . . ,𝑤𝑘∈Σ∗, their concatenation is denoted by �𝑘

𝑖=1 𝑤𝑖. For a
string 𝑇∈Σ∗, we write |𝑇| = 𝑛to denote its length, i.e., the number of symbols in 𝑇. The symbol
occurring on position 𝑖∈[𝑛] of 𝑇is denoted by 𝑇[𝑖]. For 1 ≤𝑖≤𝑗≤𝑛, we denote by 𝑇[𝑖: 𝑗] the
string 𝑇[𝑖]𝑇[𝑖+ 1] · · ·𝑇[𝑗]; we set 𝑇[𝑖: 𝑗] = 𝜀if 𝑖> 𝑗. For indices 𝑖, 𝑗∈[𝑛] we say 𝑇[1 : 𝑖] is a
prefix, 𝑇[𝑗: 𝑛] a suffix, and 𝑇[𝑖: 𝑗] a substring of 𝑇.

Recall that the problem discussed here is EnumMatchReg. Consider an input instance of this
problem: a pattern 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1 and a string 𝑇, over an alphabet of terminal letters Σ.
We note that we can assume without loss of generality that 𝑤0 = 𝑤𝑘+1 = 𝜀. Otherwise, we write
𝑇= 𝑤0𝑇′𝑤𝑘+1 (if this is not possible then there are no matches), and 𝑤= 𝑤0𝑤′𝑤𝑘+1. Then,

the matches of 𝛼′ = 𝑥0(�𝑘

𝑖=1 𝑤𝑖𝑥𝑖) to 𝑤′ correspond bijectively to the matches of 𝛼and 𝑤. We
also assume for the rest of this paper that |𝛼| = 𝑚and |𝑇| = 𝑛. Clearly, we can assume that
|𝑤1 · · ·𝑤𝑘| = �𝑘

𝑖=1 |𝑤𝑖| ≤𝑛because otherwise there is no solution. Let us also note that the
case when there exists some 𝑖∈[𝑘] such that 𝑤𝑖= 𝜀is superfluous, both for the MatchReg and
EnumMatchReg. In the case of MatchReg we could simply replace 𝑥𝑖−1𝑥𝑖by a new variable 𝑥′

𝑖and solve
the problem for the transformed pattern. A similar approach works for EnumMatchReg, with the
observation that for each valid embedding of the transformed pattern we have to also enumerate
the solutions considering all possibilities to split the string assigned to 𝑥′

𝑖into strings assigned to the
variables 𝑥𝑖−1 and 𝑥𝑖(or, in other words, to consider all possibilities to embed the empty string 𝑤𝑖
between the embedding of 𝑤𝑖−1 and 𝑤𝑖+1). This would only clutter our presentation, so we assume
that 𝑤𝑖≠𝜀for all 𝑖∈[𝑘]. Thus, we assume that the total size of the input is O(𝑛). Finally, as usual
in stringology, we assume that the input alphabet is polynomial, that is, Σ = {1, . . . , poly(𝑛)}.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 8 -->

112:8
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

Substring search. The basic problem considered in the area of string algorithm is locating an
occurrence of a pattern (without variables, i.e., a string) in a text. It is often called the pattern
matching problem, but, to avoid ambiguity, we call it the substring search problem.

Substring search (substr)
Input: String 𝑤with |𝑤| = 𝑚and text 𝑇with |𝑇| = 𝑛.
Output: Find the minimal index 𝑝∈[𝑛−𝑚+ 1] such that 𝑇[𝑝: 𝑝+ 𝑚−1] = 𝑤.

It is well known that substr can be solved efficiently:

Lemma 2.1 ([29]). Let 𝑤,𝑇∈Σ∗with |𝑤| = 𝑚and |𝑇| = 𝑛. After O(𝑚) time preprocessing, the
minimal 𝑖such that 𝑇[𝑖−𝑚+ 1 : 𝑖] = 𝑤, can be found in O(𝑖+ 𝑚) time. Further, all 𝑖such that
𝑇[𝑖−𝑚+ 1 : 𝑖] = 𝑤can be enumerated, in increasing order, in O(𝑛) time.

Radix sort. For a sequence of 𝑛keys (𝑥1, . . . ,𝑥𝑛), each key being a 𝑘-tuple over [1 : 𝑚], radix sort
is a non-comparison-based sorting algorithm that sorts the keys lexicographically by processing
their components from least significant to most significant. Each pass applies counting sort to the
corresponding component of the keys. It is known that radix sort can sort 𝑛keys of length 𝑘in
O((𝑛+ 𝑚)𝑘) time [1]. If 𝑚is polynomial 𝑛(i.e., upper bounded by 𝑛𝑂(1)), then radix sort can be
implemented in O(𝑛𝑘) time. For a sequence of 𝑛variable-length keys 𝑥1, . . . ,𝑥𝑛(i.e., each key is a
tuple, but not all keys have the same size), whose components are from [1 : 𝑚], radix sort works in
O(𝑚+ �𝑛

𝑖=1 |𝑥𝑖|) time [1]; again, if 𝑚is bounded by 𝑛𝑂(1), then radix sort works in O(𝑛+ �𝑛

𝑖=1 |𝑥𝑖|)
time.

Tries and compacted tries. A trie is a rooted tree in which the edges outgoing from a node are
ordered and labelled with distinct symbols from the alphabet. Each node naturally corresponds
to a string obtained by concatenating the labels of the edges on its path from the root. For a set
of strings 𝑆= {𝑤1, . . . ,𝑤𝑛}, with 𝑤𝑖∈Σ∗for 𝑖∈[𝑛], such that no string is a prefix of another
one, there exists a unique trie T such that there is a one-to-one correspondence between the
leaves of T and the strings. Then, a compacted trie T ′ is obtained from a trie T by replacing
every maximal path consisting of nodes 𝑣0, 𝑣1, . . . , 𝑣𝑘such that 𝑣𝑖+1 is the only child of 𝑣𝑖, for every
𝑖∈{0, . . . ,𝑘−1}, with a single edge labelled with the concatenations of the labels on the path. For
a trie T corresponding to a prefix-free set of strings 𝑆, its compacted trie T ′ consists of O(𝑛) edges,
and the label of each of its edges is a substring of some string 𝑤𝑖. Thus, we can store a description of
such a compacted trie in O(𝑛) space, by representing the label of each edge as a substring 𝑤𝑖[𝑏: 𝑒].
The nodes of T ′ are called explicit, while the nodes of T that have been dissolved when creating
T ′ are called implicit. The edges outgoing from each node of T ′ are arranged in the sorted order
of their first symbols, which also corresponds to the lexicographical order of their labels.

Suffix trees and arrays. For a string𝑇[1 : 𝑛], its suffix tree 𝑆𝑇is the compacted trie corresponding
to the non-empty suffixes of𝑇$, where $ is a special symbol not occurring in𝑇and lexicographically
smaller than all letters of Σ. Thus, 𝑆𝑇has 𝑛+ 1 leaves, each corresponding to a string of the form
𝑇[𝑖: 𝑛]$. It is known that, under the assumption of polynomial alphabet, the suffix tree of 𝑇[1 : 𝑛]

can be constructed in O(𝑛) time [14]. Next, the suffix array 𝑆𝐴is a permutation of [𝑛] corresponding
to the lexicographical ordering of the suffixes of 𝑇[1 : 𝑛]. This naturally corresponds to writing
down the suffixes corresponding to the leaves of 𝑆𝑇(except for the suffix $) arranged in the order
corresponding to the pre-order traversal of the whole 𝑆𝑇. More generally, for any node 𝑣of 𝑆𝑇,
the pre-order traversal of the subtree rooted at 𝑣results in visiting the leaves corresponding to a
range 𝑆𝐴[𝑏: 𝑒]. We can compute and store this range [𝑏: 𝑒] for every node 𝑣with a bottom-up
traversal in O(𝑛) time. Finally, if 𝑠is the label of the path from the root to 𝑣and 𝑆𝐴[𝑏: 𝑒] is its

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 9 -->

Optimal Enumeration of Regular Pattern Matches
112:9

corresponding range, then the suffixes of 𝑇[1 : 𝑛]$ that have 𝑠as a prefix are exactly the suffixes
𝑇[𝑖: 𝑛]$ such that 𝑖occurs in 𝑆𝐴[𝑏: 𝑒]. One can show the following lemma.

Lemma 2.2. Let 𝑇[1 : 𝑛] be a text and let 𝑤1, . . . ,𝑤𝑘be strings with �𝑘

𝑖=1 |𝑤𝑖| ≤𝑛. Then, in total
O(𝑛) time, we can compute, for every 𝑖∈[𝑘], the range 𝑆𝐴[𝑎𝑤𝑖: 𝑏𝑤𝑖] containing exactly the positions

𝑗such that 𝑇[𝑗: 𝑗+ |𝑤𝑖| −1] = 𝑤𝑖.

Proof. We build the suffix tree and preprocess its node to store the corresponding ranges of the
suffix array. Then, we lexicographically sort all strings 𝑤1, . . . ,𝑤𝑘with radix sort for variable length
keys in O(𝑛+ �𝑘

𝑖=1 |𝑤𝑖|) = O(𝑛) time. Next, for every 𝑖∈[𝑘], we want to find the node of the
suffix tree corresponding to 𝑤𝑖(the node might be explicit or implicit) and return its corresponding
range (if a node is implicit, we return the range of its nearest explicit descendant). To find the
node corresponding to 𝑤𝑖, we first go up from the already known node corresponding to 𝑤𝑖−1 to
reach the node corresponding to the longest common prefix of 𝑤𝑖−1 and 𝑤𝑖, and then we navigate
down with the remaining suffix of 𝑤𝑖. Whenever we are at a node 𝑣and need to find its outgoing
edge with label starting form 𝑐, we simply scan the outgoing edges arranged in the lexicographical
order. However, when we have to again find the outgoing edge from the same node 𝑣, we start the
scan from the last previously considered edge (instead of the very first one). Because the strings
are considered in the sorted order, this preserves the correctness. Further, each outgoing edge is
skipped over at most once, so the total time is as claimed.
□

Range minimum/maximum and reporting. For an array 𝐴[1 : 𝑛], a range minimum query (RMQ)
is defined as RMinQ𝐴(𝑎,𝑏) = arg min𝐴[𝑎: 𝑏] with 1 ≤𝑎≤𝑏≤𝑛. Similarly, a range maximum
query is defined as RMaxQ𝐴(𝑎,𝑏) = arg max𝐴[𝑎: 𝑏]. The following is known.

Lemma 2.3 ([7]). An array 𝐴[1 : 𝑛] can be preprocessed in O(𝑛) time to answer range mini-
mum/maximum queries in O(1) time.

Lemma 2.4. A permutation of [𝑛] stored in an array 𝐴[1 : 𝑛] can be preprocessed in O(𝑛) time
for the following queries: given integers 𝑎,𝑏,𝑥∈[𝑛] with 𝑎≤𝑏, enumerate all elements of the set
{𝑖∈[𝑎: 𝑏] | 𝐴[𝑖] ≥𝑥} with worst-case constant delay.

Proof. The preprocessing amounts to constructing the RMQ structure from Theorem 2.3 in
O(𝑛) time. After receiving the integers 𝑎,𝑏,𝑥∈[𝑛] with 𝑎≤𝑏, we proceed as follows. We maintain
a stack storing intervals [𝑎𝑖,𝑏𝑖] such that RMaxQ𝐴(𝑎𝑖,𝑏𝑖) ≥𝑥. We first check if RMaxQ𝐴(𝑎,𝑏) ≥𝑥
and if so push the interval [𝑎,𝑏] onto the stack. Then, as long as the stack is non-empty we pop
its topmost element [𝑎𝑡,𝑏𝑡]. We compute 𝑝:= RMaxQ𝐴(𝑎𝑡,𝑏𝑡) and report 𝑝as the next element of
the set. Then, if 𝑎𝑡< 𝑝and RMaxQ𝐴(𝑎𝑡, 𝑝−1) ≥𝑥we push [𝑎𝑡, 𝑝−1] onto the stack. Similarly, if
𝑝< 𝑏𝑡and RMaxQ𝐴(𝑝+ 1,𝑏𝑡) ≥𝑥we push [𝑝+ 1,𝑏𝑡] onto the stack. It is straightforward to verify
that each element of the set is pushed and popped exactly once, and the delay is O(1).
□

Observation 2.1. By combining Lemmas 2.2 and 2.4 we obtain the following primitive: Let𝑇[1 : 𝑛]
be a text and let 𝑤1, . . . ,𝑤𝑘be strings with �𝑘

𝑖=1 |𝑤𝑖| ≤𝑛. After O(𝑛) preprocessing, given an index
𝑖and a position 𝑥, we can enumerate all occurrences of 𝑤𝑖starting at position 𝑥or later with O(1)
worst-case delay.

Figure 1 illustrates Observation 2.1.

3
Warm-up: Computing All Valid Embeddings

We recall that a match of 𝛼to𝑇is represented as a 𝑘-tuple (𝑗1, . . . , 𝑗𝑘) called valid embedding, where

𝑗𝑖is the position in 𝑇where 𝑤𝑖is embedded.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 10 -->

112:10
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

𝐿
. . .
. . .
𝑀
. . .
𝑀
. . .
𝑀
. . .

𝑁

Fig. 1. All occurrences of 𝑤in 𝑇starting at position 𝑥or later.

Definition 3.1. A left-canonical embedding (ℓ1, . . . , ℓ𝑖, . . . , ℓ𝑘) is a valid embedding where each
ℓ𝑖is the minimal possible among all valid embeddings. Analogously, (𝑟1, . . . ,𝑟𝑖, . . . ,𝑟𝑘) is a right-
canonical embedding if each 𝑟𝑖is the maximal possible among all valid embeddings.

Lemma 3.2. We can compute the left-canonical and right-canonical embedding or decide that they
do not exist in O(𝑛) time.

Proof. We describe the case of computing a left-canonical embedding; the right-canonical case
can be solved analogously. We proceed greedily from left to right. Using Lemma 2.1, we first find
the leftmost occurrence of 𝑤1 in 𝑇, starting at position 1. Next, assume that ℓ1, . . . , ℓ𝑖is already
computed, we then search for the leftmost occurrence of 𝑤𝑖+1 in the remaining suffix𝑇[ℓ𝑖+ |𝑤𝑖| : 𝑛]
with the same algorithm. If at any step such an occurrence does not exist, we return that no valid
embedding exists. If all 𝑤1, . . . ,𝑤𝑘have been found, then (ℓ1, . . . , ℓ𝑘) is a left-canonical embedding.
The entire procedure runs in O(𝑛) time, since the 𝑖-th call to the algorithm from Lemma 2.1 takes

O(|𝑤𝑖| + ℓ𝑖−ℓ𝑖−1 + |𝑤𝑖|) time, which telescopes to O(𝑛). The right-canonical embedding can be
obtained analogously by performing the same greedy search on the reversals (and reversed order)
of all strings and the text and then translating the indices to the non-reversed text.
□

Note that the existence of a left-canonical embedding implies the existence of a right-canonical
embedding (they might be, however, identical). From now on, we assume that a valid embedding
exists, and so the left- and the right-canonical embeddings also exist. We state their simple properties.

Observation 3.1. ℓ𝑖+ |𝑤𝑖| ≤ℓ𝑖+1 and 𝑟𝑖+ |𝑤𝑖| ≤𝑟𝑖+1, for all 𝑖∈[𝑘−1], so, in particular,
ℓ1 < ℓ2 < . . . < ℓ𝑘and 𝑟1 < 𝑟2 < . . . < 𝑟𝑘. Further, ℓ𝑖≤𝑟𝑖for all 𝑖∈[𝑘].

Lemma 3.3. For all valid embeddings (𝑗1, . . . , 𝑗𝑘), we have that 𝑗𝑖∈[ℓ𝑖: 𝑟𝑖], for all𝑖∈[𝑘]. Moreover,
every occurrence of 𝑤𝑖starting in 𝑇[ℓ𝑖: 𝑟𝑖] belongs to a valid embedding.

Proof. Let us consider the contraposition. If 𝑗𝑖< ℓ𝑖, then 𝑗1, . . . , 𝑗𝑖can not be matched in the
prefix 𝑇[1 : ℓ𝑖−1]. Analogously, when 𝑗𝑖> 𝑟𝑖, then 𝑗𝑖, . . . , 𝑗𝑘can not be matched in the suffix
𝑇[𝑟𝑖+ 1 : 𝑛]. Therefore, it holds that 𝑗𝑖∈[ℓ𝑖: 𝑟𝑖].

For the second part of our claim, consider such an occurrence starting at position 𝑗𝑖. From the
first part of this lemma, we know that 𝑗𝑖∈[ℓ𝑖: 𝑟𝑖]. Therefore, (ℓ1, . . . , ℓ𝑖−1, 𝑗𝑖,𝑟𝑖+1, . . . ,𝑟𝑘) is a valid
embedding by Observation 3.1.
□

Let 𝑂𝑖:= {𝑝∈[ℓ𝑖: 𝑟𝑖] | 𝑇[𝑝: 𝑝+ |𝑤𝑖| −1] = 𝑤𝑖} denote the set of valid occurrences of 𝑤𝑖, i.e.,
starting positions of 𝑤𝑖in 𝑇, that belong to at least one valid embedding. Further, let 𝑆be the set
of all valid embeddings. Note that for 𝑗𝑖∈𝑂𝑖\ {ℓ𝑖} every embedding (ℓ1, . . . , ℓ𝑖−1, 𝑗𝑖,𝑟𝑖+1, . . . ,𝑟𝑘) is
valid, and every two such embeddings are distinct. Hence, the following result follows immediately.

Lemma 3.4. �𝑘

𝑖=1(|𝑂𝑖| −1) ≤min{𝑘𝑛, |𝑆|}.

Proof. It is immediate that �𝑘

𝑖=1(|𝑂𝑖| −1) ≤𝑘𝑛, as we trivially have that |𝑂𝑖| ≤𝑛. So, we only
show that �𝑘

𝑖=1(|𝑂𝑖| −1) ≤|𝑆|.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 11 -->

Optimal Enumeration of Regular Pattern Matches
112:11

For 𝑖∈[𝑘], we construct valid embeddings (𝑗1, . . . , 𝑗𝑖−1, 𝑗𝑖, 𝑗𝑖+1, . . . , 𝑗𝑘) as follows. We fix 𝑗1 =
ℓ1, ..., 𝑗𝑖−1 = ℓ𝑖−1 and 𝑗𝑖+1 = 𝑟𝑖+1, ..., 𝑗𝑘= 𝑟𝑘. Then, choosing any 𝑗𝑖∈𝑂𝑖\ {ℓ𝑖} results in a valid
embedding (by Theorem 3.3), and the valid embeddings obtained in this way, for some 𝑖, are pairwise
distinct because they differ on the 𝑖-th coordinate. Further, two embeddings constructed as above
for different values of 𝑖, 𝑖1 and 𝑖2, differ on position min{𝑖1,𝑖2}, so are also distinct. Thus, we have
constructed �𝑘

𝑖=1(|𝑂𝑖| −1) distinct valid embeddings. The result follows.
□

Next, we design an algorithm that outputs efficiently the set 𝑆of all valid embeddings. To set
a baseline, we first note that, given the sorted sets 𝑂𝑖, we can output all valid embeddings by a
straightforward recursive approach: after setting Sol[𝑖], we set Sol[𝑖+ 1] to a value from 𝑂𝑖+1,
greater or equal to Sol[𝑖], and recurse.

Lemma 3.5. Given sorted sets 𝑂1, . . . ,𝑂𝑘, all valid embeddings can be enumerated in O(|𝑆|𝑘) time.

Proof. We compute the set of valid embeddings recursively. We maintain an array Sol, such
that before a call to report we have Sol[𝑖] = 𝑗𝑖, for all 𝑖∈[𝑘], i.e., the array stores the current
embedding. Assume that 𝑗1, . . . , 𝑗𝑖−1 have already been fixed. For the current index 𝑖, we iterate in
decreasing order over the sorted set 𝑂𝑥

𝑖:= {𝑗𝑖| 𝑗𝑖∈𝑂𝑖, 𝑗𝑖≥𝑥} where 𝑥= 𝑗𝑖−1 + |𝑤𝑖−1| is passed as
an argument in the recursive call from 𝑗𝑖−1. For each choice of 𝑗𝑖, we set Sol[𝑖] = 𝑗𝑖and recurse.
Whenever 𝑖= 𝑘after updating Sol[𝑘], we report the current array Sol. This clearly computes all
valid embeddings one by one and takes O(𝑘) between two calls to report.
□

We will now strengthen the above lemma in two directions. First, we will replace the requirement
that the lists 𝑂1, . . . ,𝑂𝑘are given sorted by two requirements (which trivially hold for sorted lists).
Second, we will improve the delay between the enumeration of two embeddings to O(1), and
obtain a total O(|𝑆|) time to enumerate 𝑆.

Lemma 3.6. Provided data structures allowing us to:

• enumerate, for a given 𝑖and a number 𝑥, with constant delay, all elements of 𝑂𝑖greater or equal
to 𝑥, starting with 𝑟𝑖,
• retrieve the largest two elements in 𝑂𝑖in O(1) time,
all valid embeddings can be enumerated, after an additional preprocessing running in O(𝑘) time, with
constant delay. The space used, additionally to that allocated for the provided data structures, is O(𝑘).

Proof. We define a new recursive function enum where, as opposed to the previous approach,
each recursive call will report a valid embedding. The function has two parameters: an integer 𝑖
and a boolean 𝑏. When the function is called, the prefix Sol[1], . . . , Sol[i −1] has been already
fixed, and additionally Sol[i] = 𝑟𝑖, . . . , Sol[k] = 𝑟𝑘holds. The goal of the function is to report
all valid embeddings with the fixed prefix, except for the valid embedding currently stored in the
Sol array. We require that ℓ𝑖< 𝑟𝑖, so at least one valid embedding will be indeed reported in the
recursive call. Further, 𝑏stores the parity of the depth of the recursive call (this will be possibly
different than simply the parity of 𝑖, and will be explained later).

To be able to implement the recursive function efficiently, we first define an auxiliary function
jump : [𝑘] →[𝑘+ 1] which basically guides our recursive approach more efficiently. The function
is defined as jump(𝑘) = 𝑘+ 1 and for each 𝑖∈[𝑘−1]:

jump(𝑖) =

�
𝑖+ 1,
if |{𝑥∈𝑂𝑖+1 | 𝑥≥𝑟𝑖+ |𝑤𝑖|}| ≥2,
jump(𝑖+ 1),
otherwise.
This function computes, for some 𝑖, the smallest 𝑗> 𝑖such that there are two occurrences of 𝑤𝑗,
which can still lead to a valid embedding, i.e., to the right of 𝑟𝑗−1 + |𝑤𝑗−1| −1. In other words, it
allows us to jump, in our recursion, over the positions of that embedding that can only be assigned

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 12 -->

112:12
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

as in the right-canonical embedding. The function jump is computed in O(𝑘) time during the
preprocessing as follows. We proceed with 𝑖from 𝑘−1 down to 1, and when computing jump(𝑖),
we simply retrieve the largest two elements in 𝑂𝑖+1 and, if both are greater or equal to 𝑟𝑖+ |𝑤𝑖|, we
set jump(𝑖) = 𝑖+ 1; otherwise, we set jump(𝑖) = jump(𝑖+ 1).

After that, still in the preprocessing phase, we first set the elements of Sol to be (𝑟1, . . . ,𝑟𝑘),
defining a right-canonical embedding. We report the current embedding, and if jump(1) ≠𝑘+ 1
(otherwise we are already done) then we call enum(jump(1), false).

In the recursive function, we enumerate with constant delay (as allowed by this lemma’s prereq-
uisites), all elements of 𝑂𝑖greater or equal to 𝑗𝑖−1 + |𝑤𝑖−1| (for convenience, we assume 𝑗0 = 1 and
𝑤0 = 𝜀). We initially disregard the current element if it is equal to 𝑟𝑖, and note that this does not
increase the delay as it can happen only once and |𝑂𝑖| ≥2. Let the currently considered element of
𝑂𝑖be 𝑥≠𝑟𝑖. We set Sol[𝑖] = 𝑥and if 𝑏= false then we report the current embedding. Further,
we check if the largest two elements of 𝑂𝑖+1 are greater or equal to 𝑥+ |𝑤𝑖|, and if so we call
enum(𝑖+ 1, not 𝑏). Otherwise, if jump(𝑖+ 1) ≠𝑘+ 1 we call enum(jump(𝑖+ 1), not 𝑏). Finally,
if 𝑏= true then we report the current embedding. Once we have enumerated all elements of
𝑂𝑠\ {𝑟𝑠}, we set Sol[𝑖] = 𝑟𝑖, and if jump(𝑖) ≠𝑘+ 1 then we call enum(jump(𝑖), not 𝑏). This very
last recursive call is implemented with tail recursion, so that we don’t need to account for the time
to return from the recursive call. The pseudocode of this procedure is given below.

Algorithm 1: The main call and the recursive procedure enum.

1 for 𝑖∈[𝑘] do

2
Sol[𝑖] ←𝑟𝑖;

3 report;

4 if jump(1) ≠𝑘+ 1 then

5
enum(jump(1), false);

6 function enum(𝑖,𝑏):

7
for 𝑥∈𝑂𝑖\ {𝑟𝑖} s.t. 𝑥≥𝑗𝑖−1 + |𝑤𝑖−1| do

8
Sol[𝑖] ←𝑥;

9
if not 𝑏then

10
report;

11
if the largest two elements of 𝑂𝑖+1 are ≥𝑥+ |𝑤𝑖| then

12
enum(𝑖+ 1, not 𝑏);

13
else

14
if jump(𝑖) ≠𝑘+ 1 then

15
enum(jump(𝑖), not 𝑏);

16
if 𝑏then

17
report;

18
Sol[𝑖] ←𝑟𝑖;

19
if jump(𝑖) = 𝑘+ 1 then

20
return;

21
else

22
enum(jump(𝑖),𝑏);
⊲tail recursion

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 13 -->

Optimal Enumeration of Regular Pattern Matches
112:13

Let us first analyse the complexity of this algorithm. The preprocessing takes, clearly, O(𝑘) time.
Analysing the delay is more complicated.

We first observe that after entering a recursive call we will report a valid embedding after
O(1) time. This is because ℓ𝑖< 𝑟𝑖, so there will be at least one 𝑥considered in the for loop. If
𝑏= false then we immediately report the current embedding. If 𝑏= true then either we call

enum(𝑖+ 1, false) or enum(jump(𝑖), false), where we again consider at least one 𝑥in the for loop
and then immediately report the current embedding (as now 𝑏= true) there, or do not perform
any recursive call and report the current embedding before finishing the iteration of the for loop.

Now we can analyse the delay between two calls to report. Consider a call to report inside the
for loop. Then, if there is another iteration of the for loop, then either𝑏= false and we immediately
report the current embedding in its beginning, or 𝑏= true and we either perform a recursive
call, in which we report a valid embedding after O(1) time as argued above, or we report the
current embedding before finishing the iteration. The remaining problematic case is that of report
in the very last iteration of the for loop. If jump(𝑖) ≠𝑘+ 1 then we will immediately have another
recursive call (implemented with tail recursion, but this is irrelevant at this point), so as argued
above we will report a valid embedding after O(1) time. Otherwise, we return from the recursive
call. Due to tail recursion, we might actually immediately return to any of the predecessor recursive
calls enum(𝑖′,𝑏′), where 𝑏′ = not 𝑏(we note that this is why the very last call is enum(jump(𝑖),𝑏)
and not enum(jump(𝑖), not 𝑏)) and continue in line 16. If 𝑏′ = true then we immediately report
a valid embedding. Otherwise, if there is another iteration of the for loop we will report a valid
embedding after O(1) time, similarly if we call enum(jump(𝑖′),𝑏′). The remaining case is that
𝑏′ = false and we return from the recursive call to some predecessor recursive call enum(𝑖′′,𝑏′′),
where 𝑏′′ = not 𝑏′, so 𝑏′′ = true. But then repeating the argument one more time for 𝑖′′ and 𝑏′′

shows that we will report a valid embedding after O(1) time.

As far as the space complexity of this approach is concerned, on top of the space needed for
the provided data structures, we use O(𝑘) space to store the jump function, the Sol array, and
(implicitly) the recursion stack.

Now we will explain why the algorithm correctly enumerates all valid embeddings. The main
idea is that for a prefix (𝑗1, . . . , 𝑗𝑖−1), we consider one by one all choices of 𝑗𝑖and, for each of them,
every possible continuation: either we detect that only a right canonical embedding is possible for
the remaining positions, or we detect the next position where we choose something else than the
right canonical embedding and continue from there.

So, for each choice 𝑗𝑖= 𝑥, 𝑥≠𝑟𝑖, we report the embedding (𝑗1, . . . , 𝑗𝑖−1,𝑥,𝑟𝑖+1, . . . ,𝑟𝑘) in the
current iteration of the for loop. In the corresponding recursive call, we report other embeddings
that extend the prefix (𝑗1, . . . , 𝑗𝑖−1,𝑥). Once all these embeddings are enumerated (each exactly
once), due to the fact that our computation considers the elements of each set 𝑂𝑡, with 𝑡∈[𝑘], such
that 𝑟𝑡is always the last to be processed, we end up with the embedding (𝑗1, . . . , 𝑗𝑖−1,𝑥,𝑟𝑖+1, . . . ,𝑟𝑘)
again, but this is not reported in during the recursive calls (otherwise we would have repetitions in
our enumeration, which is not desirable). Then we proceed to the next 𝑥, if there is any.

Now, for the case when 𝑗𝑖= 𝑟𝑖, if jump(𝑠) = 𝑘+ 1, we are done: the current embedding is
(𝑗1, . . . , 𝑗𝑖−1,𝑟𝑖, . . . ,𝑟𝑘) has been already reported earlier in the recursion, and there are no other
embeddings with the prefix (𝑗1, . . . , 𝑗𝑖−1,𝑟𝑖). Otherwise, we proceed as before, and enumerate all
embeddings that extend (𝑗1, . . . , 𝑗𝑖−1,𝑟𝑖), other than the one where all coordinates to the right of 𝑠
are set to their value from the right-canonical embedding, i.e., (𝑗1, . . . , 𝑗𝑖−1,𝑟𝑖,𝑟𝑖+1, . . . ,𝑟𝑘). Once
we are done with all these embeddings, the current valid embedding is (𝑗1, . . . , 𝑗𝑖−1,𝑟𝑖,𝑟𝑖+1, . . . ,𝑟𝑘)
which is not reported (as explained above, it has been already reported). Instead, the last call
of enum made during the enumeration of these embeddings returns (due to the elimination of

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 14 -->

112:14
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

tail recursion) to a call of enum for some parameter 𝑖′ < 𝑖, such that the current embedding is
(𝑗1, . . . , 𝑗𝑖′,𝑟𝑖′+1, . . . ,𝑟𝑘) with 𝑗𝑖′ ≠𝑟𝑖′.

In conclusion, we report all embeddings extending (𝑗1, . . . , 𝑗𝑠−1,𝑥), for all 𝑥∈𝑂𝑠, and each is
reported exactly once.
□

For the rest of the section, we show that we can actually obtain the sorted sets 𝑂1,𝑂2, . . .𝑂𝑘,
which would allow us to apply the previous lemma. A simple (but slow) approach is as follows.
Assume that we have computed the left-canonical embedding (ℓ1, . . . , ℓ𝑘) and a right-canonical
embedding (𝑟1, . . . ,𝑟𝑘). We compute each 𝑂𝑖naively by repeatedly invoking Lemma 2.1 on the
relevant substring of the text 𝑇[ℓ𝑖: 𝑟𝑖+ |𝑤𝑖|) to extract the occurrences of 𝑤𝑖one by one in the
sorted order. This takes �𝑘

𝑖=0 O(𝑟𝑖−ℓ𝑖+ |𝑤𝑖|) = O(𝑛𝑘) time altogether for all sets. Combining this
with Theorem 3.5 yields total O(𝑘(𝑛+ |𝑆|)) time to output all valid embeddings. We will next show
how to obtain the sorted lists 𝑂1,𝑂2, . . .𝑂𝑘in O(|𝑆|) time with a more efficient approach.

Definition 3.7 (Epochs). Consider the sequence of intervals [ℓ1 : 𝑟1 + |𝑤1|), . . . , [ℓ𝑘: 𝑟𝑘+ |𝑤𝑘|)
defined by the left- and right-canonical embeddings. We define another sequence of intervals
𝑒1,𝑒2, . . . ,𝑒𝑣, called epochs, as follows:

• 𝑒0 = ∅. For 𝑔≥1, assume that [ℓ1 : 𝑟1 + |𝑤1|), . . . , [ℓ𝑎−1 : 𝑟𝑎−1 + |𝑤𝑎−1|) are contained in
epochs 𝑒1, . . . ,𝑒𝑔−1 and [ℓ𝑎: 𝑟𝑎+ |𝑤𝑎|) is not contained in any of these epochs. Then the next
epoch 𝑒𝑔is defined as the interval [ℓ𝑎: 𝑟𝑏+ |𝑤𝑏|), such that ℓ𝑏< 𝑟𝑎+ |𝑤𝑎| and ℓ𝑏+1 ≥𝑟𝑎+ |𝑤𝑎|
(or 𝑏= 𝑘). Thus, 𝑒𝑔contains [ℓ𝑎: 𝑟𝑎+ |𝑤𝑎|), . . . , [ℓ𝑏: 𝑟𝑏+ |𝑤𝑏|).
• For 𝑔≥1, let left𝑔= min𝑒𝑔and right𝑔= max𝑒𝑔, and set 𝑐𝑔= 𝑟𝑎+ |𝑤𝑎| to be the center of the
epoch 𝑒𝑔. We call, respectively, [left𝑔: 𝑐𝑔−1] the left and [𝑐𝑔+ 1 : right𝑔] the right part of 𝑒𝑔.

We note that 𝑣≤𝑘and, for every 𝑔∈[𝑣], the union of the epochs 𝑒1, . . . ,𝑒𝑔covers a prefix of
the sequence of intervals [ℓ1 : 𝑟1 + |𝑤1|), . . . , [ℓ𝑘: 𝑟𝑘+ |𝑤𝑘|), and, in fact, that every position of 𝑇is
covered by at most two epochs. The following lemma holds.

Lemma 3.8. �𝑣

𝑔=1(right𝑔−left𝑔) = O(𝑛).

Proof. For two consecutive epochs 𝑒𝑔and 𝑒𝑔+1, we have by, Theorem 3.7, that 𝑐𝑔< left𝑔+1
and right𝑔< 𝑐𝑔+1. Hence, [left𝑔: 𝑐𝑔] ∩[left𝑔+1 : right𝑔+1] = ∅and [𝑐𝑔+1 : right𝑔+1] ∩[left𝑔:
right𝑔] = ∅. Therefore, if [left𝑔: right𝑔] ∩[left𝑔+1 : right𝑔+1] ≠∅, then the common positions
must lie in [𝑐𝑔+ 1 : right𝑔] ∩[left𝑔+1 : 𝑐𝑔+1]. These parts of the epochs occur strictly one after

𝑅

First epoch [𝑄1 : 𝑁3 + |𝑆3|)

Second epoch [𝑄4 : 𝑁5 + |𝑆5|)

Center 𝑍1 = 𝑁1 + |𝑆1|
Center 𝑍2 = 𝑁4 + |𝑆4|
𝑄1
𝑄2 𝑄3
𝑄4
𝑁1
𝑁2
𝑁3
𝑁4
𝑄5
𝑁5

Fig. 2. Visualization of two epochs with labeled intervals [ℓ𝑖,𝑟𝑖], occurrences (black rectangles) of the strings
𝑤𝑖, with 𝑖∈[4], and epoch centers indicated in red.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 15 -->

Optimal Enumeration of Regular Pattern Matches
112:15

another for 𝑔from 1 to 𝑣. Consequently, the sum of all intersections of epochs can be bounded as
�𝑣−1
𝑔=1 |[𝑐𝑔+ 1 : right𝑔] ∩[left𝑔+1 : 𝑐𝑔+1]| ≤2𝑛. Thus, also the sum of the lengths of all epochs is
�𝑣
𝑔=1(right𝑔−left𝑔) = O(𝑛).
□

Definition 3.9. For an epoch 𝑒𝑔that includes [ℓ𝑖: 𝑟𝑖], the set of occurrences of 𝑤𝑖fully included
in left part of an epoch is denoted by 𝐿𝑖and the set of the occurrences fully in the right part by 𝑅𝑖.
The set of occurrences fully in 𝑇[𝑐𝑔−|𝑤𝑖| + 1 : 𝑐𝑔+ |𝑤𝑖|) is denoted by 𝑀𝑖,

As every occurrence of 𝑤𝑖from an epoch 𝑒𝑔is in exactly one of the sets 𝐿𝑖, 𝑀𝑖, or 𝑅𝑖, we have
𝑂𝑖= 𝐿𝑖∪𝑀𝑖∪𝑅𝑖. Further, in Theorem 3.10, we state that 𝐿𝑖, 𝑅𝑖, and 𝑀𝑖can be computed efficiently:
the part regarding 𝐿𝑖and 𝑅𝑖follows from applying Observation 2.1 for each left part (read left to
right) and, respectively, right part (read right to left) of an epoch, while the part on 𝑀𝑖follows from
Theorem 2.1.

Lemma 3.10. The epochs can be computed in O(𝑘) time and can be preprocessed in O(𝑛) time for
the following queries:

(1) given an index 𝑖∈[𝑘], enumerate the elements of 𝐿𝑖and 𝑅𝑖with constant delay.
(2) given an index 𝑖∈[𝑘], enumerate the elements of 𝑀𝑖with constant delay.

Proof. We first note that a procedure computing the epochs in linear time follows directly
from the definition of the epochs. If 𝑒𝑔contains [ℓ𝑎: 𝑟𝑎+ |𝑤𝑎|), . . . , [ℓ𝑏: 𝑟𝑏+ |𝑤𝑏|) (but not

[ℓ𝑎−1 : 𝑟𝑎−1 + |𝑤𝑎−1|) nor [ℓ𝑏: 𝑟𝑏+ |𝑤𝑏|)), we can store, for each 𝑖∈[𝑎: 𝑏], 𝑒𝑔as the epoch which
contains [ℓ𝑖: 𝑟𝑖+ |𝑤𝑖|).

For both queries, we need the following O(𝑛) time preprocessing. Recall that each interval
[ℓ𝑖: 𝑟𝑖+ |𝑤𝑖|) is contained in some epoch. We will separately preprocess each epoch 𝑒𝑔and all 𝑖
such that the corresponding interval is contained in 𝑒𝑔in O(|𝑒𝑔|) time. This will sum up to O(𝑛)
over all the epochs. When preprocessing a single epoch, we need that the alphabet of each relevant
𝑤𝑖is polynomial in |𝑒𝑔|, and not only polynomial in 𝑛(which might be much larger). This is done by
renaming the symbols in each fragment 𝑇[left𝑔: right𝑔) and each relevant 𝑤𝑖so that the alphabet
is [|𝑒𝑔| + 1] (recall that each 𝑤𝑖occurs in the fragment). To this end, for each position in either
𝑇[left𝑔: right𝑔) or any relevant 𝑤𝑖we add a pair consisting of the index 𝑔and the symbol at the

corresponding position to a list. The pair is decorated with the information about its source. Then
we use radix sort to sort the list of pairs in O(𝑛) time. This allows us to consistently rename the
symbols in each fragment as required by scanning the sorted list and identifying symbols that are
equal.

Let us now show to preprocess the epochs in O() time for the following queries: given an index
𝑖∈[𝑘], enumerate the elements of 𝐿𝑖and 𝑅𝑖with constant delay. We consider a single epoch 𝑒𝑔and
all 𝑖such that the corresponding interval is contained in 𝑒𝑔. We comment that the total length of the
corresponding patterns 𝑤𝑖is at most |𝑒𝑔|. After the renaming described above, we can assume that
the alphabet of𝑇[left𝑔: right𝑔) and each such 𝑤𝑖is [|𝑒𝑔| +1], allowing us to build a suffix tree/array
for𝑇[left𝑔: right𝑔) in linear time. To obtain 𝐿𝑖, we define the left part 𝐿𝑃𝑔= 𝑇[left𝑔: 𝑐𝑔−1]. Then,
we apply Observation 2.1 to the left part and each relevant 𝑤𝑖. The preprocessing takes O(|𝑒𝑔|)
time and allows us to enumerate all occurrences of 𝑤𝑖starting at position ℓ𝑖or later in the left
part with O(1) delay. To obtain 𝑅𝑖, we define the right part 𝑅𝑃𝑔= 𝑇[𝑐𝑔+ 1 : right𝑔]. We proceed
analogously but we reverse 𝑅𝑃𝑔and all the relevant 𝑤𝑖before applying Observation 2.1.

Now we move to preprocessing the epochs in O(𝑛) time for the following queries: given an index
𝑖∈[𝑘], enumerate the elements of 𝑀𝑖with constant delay. For each epoch 𝑒𝑔and every string 𝑤𝑖
contained in 𝑒𝑔, define the middle part 𝑀𝑃𝑖= 𝑇[𝑐𝑔−|𝑤𝑖| + 1 : 𝑐𝑔+ |𝑤𝑖|). By Theorem 2.1, we can
enumerate all occurrences of 𝑤𝑖in 𝑀𝑃𝑖from left to right in O(|𝑀𝑃𝑖|) = O(|𝑤𝑖|) time. We explicitly

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 16 -->

112:16
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

store these occurrences in a sorted list 𝑀𝑖. Since �𝑘

𝑖=1 2|𝑤𝑖| = O(𝑛), the whole preprocessing takes
O(𝑛) time. Then, the elements of any 𝑀𝑖can be enumerated by directly accessing the corresponding
sorted list.
□

We define, for 𝑖∈[𝑘], the list 𝑂𝑖as the union of 𝐿𝑖, 𝑅𝑖, and 𝑀𝑖, a The previous lemmas allow us
to obtain the following result.

Lemma 3.11. We can compute all sorted 𝑂1, . . . ,𝑂𝑘in O(|𝑆| + 𝑛) time and O(𝑛+ �𝑘

𝑖=1 |𝑂𝑖|) space.

Proof. We apply Theorem 3.10 to preprocess 𝑇in O(𝑛) time. Then we compute and store (as
the list 𝑂𝑖) the elements of 𝐿𝑖, 𝑅𝑖, and 𝑀𝑖, in O(|𝐿𝑖| + |𝑅𝑖| + |𝑀𝑖|) time, for all 𝑖∈[𝑘]. We can sort
the concatenation of all lists 𝑂𝑖in total O(|𝑆| + 𝑛) time using radix sort (while also keeping track
of the original list 𝑂𝑖from which each element in this concatenation came from), due to the fact
that all positions are integers bounded by |𝑇| = 𝑛and 𝑘≤𝑛. Then, using this sorted list of the
concatenation of the lists 𝑂𝑖, and based on the fact that we know the original list of each element
of this sorted list, we can obtain the sorted 𝑂1, . . . ,𝑂𝑘in O(�𝑘

𝑖=1 |𝑂𝑖|) ∈O(|𝑆|).
The space required for this computation is O(𝑛) for 𝑇and the additional data structures con-
structed for it and O(𝑛+ �𝑘

𝑖=1 |𝑂𝑖|) for computing, sorting, and storing the respective lists.
□

After using Theorem 3.11, we can apply Theorem 3.6 to the sorted lists 𝑂1, . . . ,𝑂𝑘(as the two
requirements of Theorem 3.6 trivially hold for the respective sorted lists) and directly obtain
Theorem 1.1.

Theorem 1.1. We can enumerate the set 𝑆of all valid embeddings of 𝛼in 𝑇with constant delay,
after a preprocessing running in O(𝑛+ |𝑆|) time. The used space is O(𝑛+ min{|𝑆|,𝑛𝑘}).

4
Amortized Constant Time Enumeration
Theorem 1.1 shows an enumeration algorithm with average constant delay, i.e., the total running
time of the algorithm is O(𝑛+ |𝑆|), where 𝑆is the set of valid embeddings of 𝛼in 𝑇. We show next
that we can actually achieve linear time preprocessing and amortized constant delay. To decrease
the preprocessing time to O(𝑛) we move some of the work done in the preprocessing to the the
actual enumeration phase, although this might affect the worst-case delay between two consecutive
reports. More precisely, we need to avoid precomputing the lists 𝑂𝑖in the preprocessing phase of
the algorithm, as their total length might significantly exceed O(𝑛).

Our algorithm maintains, in the array Sol, valid embeddings (𝑗1, . . . , 𝑗𝑘). We call such a valid
embedding (𝑗1, . . . , 𝑗𝑘) simple when, for some 𝑖∈[𝑘], we have (i) 𝑗1 = ℓ1, . . . , 𝑗𝑖−1 = ℓ𝑖−1, (ii)
ℓ𝑖< 𝑗𝑖≤𝑟𝑖, and (iii) 𝑗𝑖+1 = 𝑟𝑖+1, . . . , 𝑗𝑘= 𝑟𝑘. By Lemma 3.4, the number of simple embedding is
�𝑘
𝑖=1(|𝑂𝑖| −1).

The simple embeddings can be enumerated efficiently. In fact, it suffices to run the preprocessing
from Theorem 3.10, and then we can enumerate, for 𝑖from 1 to 𝑘, such that ℓ𝑖≠𝑟𝑖, the elements
of 𝑂𝑖= 𝐿𝑖∪𝑅𝑖∪𝑀𝑖, other than ℓ𝑖. We start with the embedding (ℓ1, . . . , ℓ𝑘). Then, for some 𝑖, the
valid embedding maintained by our algorithm has the prefix (ℓ1, . . . , ℓ𝑖−1), the suffix (𝑟𝑖+1, . . . ,𝑟𝑘),
and we just keep changing the 𝑖th component of the embedding, according to the current element
of 𝑂𝑖\ {ℓ𝑖}, and reporting it. When moving to 𝑖+ 1, we just have to set the 𝑖th component to ℓ𝑖, and
start the enumeration for 𝑂𝑖+1 \ {ℓ𝑖}. The only subtlety is that, in order to keep the delay constant,
we only need to consider in our enumeration those 𝑖for which ℓ𝑖≠𝑟𝑖(as there are no simple
embeddings with prefix (ℓ1, . . . , ℓ𝑖−1) and suffix (𝑟𝑖+1, . . . ,𝑟𝑘) if ℓ𝑖= 𝑟𝑖). Moreover, we also compute,
this way, the lists 𝑂𝑖. This is formalized in the following lemma.

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 17 -->

Optimal Enumeration of Regular Pattern Matches
112:17

Lemma 4.1. After a preprocessing running in O(𝑛) time, all simple embeddings can be enumerated
with constant delay. After all simple embeddings have been enumerated, we have the unsorted list 𝑂𝑖,
for every 𝑖∈[𝑘].

Proof. We use the preprocessing from Theorem 3.10 in O(𝑛) time. Then, we construct a sorted
list 𝐼of all indices 𝑖such that ℓ𝑖< 𝑟𝑖, i.e there are at least two valid occurrences of 𝑤𝑖. We also
set 𝑗1 = 𝑟1, . . . , 𝑗𝑘= 𝑟𝑘. This takes O(𝑘) = O(𝑛) time, which is accounted for in the preprocessing.
Then, we iterate over all indices 𝑖∈𝐼, in increasing order. For each such 𝑖, we enumerate all
valid occurrences of 𝑤𝑖in 𝐿𝑖∪𝑅𝑖∪𝑀𝑖of 𝑤𝑖with constant delay, using Theorem 3.10. Let the
currently considered occurrence of 𝑤𝑖be at position 𝑗𝑖; note that ℓ𝑖≤𝑗𝑖≤𝑟𝑖. We first add 𝑗𝑖to the
unsorted list 𝑂𝑖and, if ℓ𝑖< 𝑗𝑖, then we report the current (simple) embedding (𝑗1, . . . , 𝑗𝑘). The
delay between reporting two simple embeddings is constant because, for every 𝑖, we consider at
least two positions 𝑗𝑖and, for every two consecutively considered positions 𝑗𝑖at least one results
in reporting a simple embedding. After we have considered all valid occurrences of 𝑤𝑖, starting on
positions between ℓ𝑖and 𝑟𝑖, we set 𝑗𝑖= ℓ𝑖, such that after processing 𝑖the current embedding is of
the form (ℓ1, . . . , ℓ𝑖,𝑟𝑖+1, . . .𝑟𝑘). We then proceed to the next 𝑖from the list 𝐼.
□

We now proceed as follows. In the preprocessing phase we run the algorithm from Lemma 4.1,
and then continue running it to enumerate up to 𝑛simple embeddings, however we don’t actually
report them. If in fact there are no more simple embeddings than we have already obtained, for
every 𝑖, the unsorted list 𝑂𝑖. We can sort all of those lists together with radix sort in additional O(𝑛)
time to obtain, for every 𝑖, the sorted list 𝑂𝑖. This concludes the preprocessing phase, and then we
can proceed as in the previous section. The remaining case is that there are more than 𝑛simple
embeddings. Then we again run the algorithm from Lemma 4.1, but now we do report the obtained
simple embeddings. After having reported all the Ω(𝑛) simple embeddings we have the unsorted
list 𝑂𝑖, and we can sort all of them together with radix sort in additional O(𝑛+ �

𝑖|𝑂𝑖|) time to
obtain, for every 𝑖, the sorted list 𝑂𝑖. We observe that O(𝑛+ �

𝑖|𝑂𝑖|) is only amortized constant
time per every valid embedding reported so far. Having the sorted lists 𝑂𝑖, we now proceed as in
the previous section, but we need to be careful as to not report the simple embeddings again. To
this end, we can keep track of whether the currently considered valid embedding is simple with
the following lemma.

Lemma 4.2. After O(𝑘) preprocessing, we can keep track of whether the currently considered valid
embedding is simple in O(1) time after changing any 𝑗𝑖.

Proof. We observe that the current valid embedding (𝑗1, . . . , 𝑗𝑘) is simple if and only if (i) if
𝑗𝑖= 𝑟𝑖then 𝑗𝑖+1 = 𝑟𝑖+1 holds for every 𝑖∈[𝑘−1], (ii) if 𝑗𝑖+1 = ℓ𝑖+1 then 𝑗𝑖= ℓ𝑖holds for every
𝑖∈[𝑘−1], and (ii) if ℓ𝑖< 𝑗𝑖< 𝑟𝑖then 𝑗𝑖−1 = ℓ𝑖−1 (or 𝑖= 1) and 𝑗𝑖+1 = 𝑟𝑖+1 (or 𝑖= 𝑘). We maintain
three counters: the first corresponds to the number of indices 𝑖such that 𝑗𝑖= 𝑟𝑖but 𝑗𝑖+1 ≠𝑟𝑖+1,
the second corresponds to the number of indices 𝑖such that 𝑗𝑖+1 = ℓ𝑖+1 but 𝑗𝑖≠ℓ𝑖, and the third
corresponds to the number of indices 𝑖such that ℓ𝑖< 𝑗𝑖< 𝑟𝑖but it does not hold that 𝑗𝑖−1 = ℓ𝑖−1 (or
𝑖= 1) and 𝑗𝑖+1 = 𝑟𝑖+1 (or 𝑖= 𝑘). The initial values of all counters can be computed in O(𝑘) time.
Then, the conditions for every 𝑖depend only on 𝑗𝑖−1, 𝑗𝑖and 𝑗𝑖+1, so after changing any 𝑗𝑖we only
need to re-evaluate the conditions for 𝑖−1,𝑖and 𝑖+ 1, which allows us to maintain every counter
in constant time. Then, the current valid embedding is simple if and only if all three counters are
zero.
□

By proceeding as described as above, and then running the enumeration algorithm from Theo-
rem 1.1 together with the procedure from Theorem 4.2 (with its preprocessing run in the beginning)

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 18 -->

112:18
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

to disregard the already reported simple embeddings we obtain Theorem 1.2. The space consump-
tion of our algorithm is just as in the case of Theorem 1.1, as no additional data structures are
required.

Theorem 1.2. We can enumerate the set 𝑆of all valid embeddings of 𝛼in𝑇with amortized constant
delay, after a preprocessing running in O(𝑛) time. The used space is O(𝑛+ min{|𝑆|,𝑛𝑘}).

5
Enumeration with Worst-Case Constant Delay
We now describe how to further improve the solution described in the previous section so that
every valid embedding is reported in O(1) worst-case time. Compared to the previous section, now
we cannot afford to “pause” reporting to sort the lists 𝑂𝑖, and further we are not necessarily able to
afford to consider some valid embedding twice (as we might have multiple such embeddings in a
row, which would lead to a non-constant delay). Instead, we avoid generating the sorted lists 𝑂𝑖
altogether.

Recall that each list 𝑂𝑖consists of the list 𝐿𝑖(the occurrences of 𝑤𝑖fully contained in the left part
of the epoch), 𝑀𝑖(the occurrences of 𝑤𝑖straddling both parts of the epoch), and 𝑅𝑖(the occurrences
of 𝑤𝑖fully contained in the right part of the epoch). We need to be able to enumerate all elements
of 𝑂𝑖greater or equal to 𝑥, starting from the largest, with constant delay, and retrieve the largest
two elements in 𝑂𝑖. It is enough to implement both operations separately for 𝐿𝑖, 𝑀𝑖, and 𝑅𝑖in such
complexities. Then, to enumerate the elements of 𝑂𝑖greater or equal to 𝑥we will first enumerate
such elements of 𝐿𝑖, then 𝑀𝑖, and finally 𝑅𝑖. To retrieve the largest two elements in 𝑂𝑖we will
retrieve such elements in 𝐿𝑖, 𝑀𝑖, and 𝑅𝑖(so at most 6 in total), and then return the two largest.

We observe that applying the second part of Theorem 3.10 actually results in constructing all
sorted lists 𝑀𝑖in O(𝑛) time. This clearly allows us to enumerate the elements of 𝑀𝑖greater or
equal to 𝑥, starting from the largest, with constant delay, and provides constant time access to the
largest two elements in 𝑀𝑖. Further, we recall that the approach used to obtain the first part of
Theorem 3.10 to enumerate the elements of 𝐿𝑖and 𝑅𝑖is based on applying Theorem 2.4. We need
to look more closely at the underlying procedure before describing how to provide the required
access to 𝐿𝑖and 𝑅𝑖.

Revisiting Theorem 2.4. The iterative procedure described in the proof can be seen as traversing
an implicitly defined heap. Each node of the heap corresponds to a range of positions [𝑎′ : 𝑏′]
such that 𝐴[𝑖] ≥𝑥, where 𝑖= RMaxQ𝐴(𝑎′,𝑏′), and stores the position 𝑖. The root corresponds to
the whole [𝑎: 𝑏]. Then, for a node 𝑣corresponding to [𝑎′ : 𝑏′], where 𝑖= RMaxQ𝐴(𝑎′,𝑏′), its left
child corresponds to [𝑎′ : 𝑖−1] (if the largest element in the range is at least 𝑥, otherwise there
is no left child), and symmetrically its right child corresponds to [𝑖+ 1 : 𝑏′]. Then, the procedure
traverses the nodes of the heap and enumerates their stored elements in such a way that at any
point in time if the element stored at a node 𝑢has been already enumerated then so has the element
stored at the parent of 𝑢(if any). We modify the procedure so that it not only enumerates the
elements, but actually maintains the already discovered nodes of the heap. Thus, after the procedure
terminates, the whole heap is available. Further, we modify the procedure so that as soon as a
node 𝑢is discovered we also have access to both of its children (by issuing the corresponding two
RMaxQ𝐴queries, which takes additional constant time).

Providing access to 𝐿𝑖. To enumerate the elements of 𝐿𝑖greater or equal to 𝑥with constant delay,
it is enough to directly apply Theorem 2.4 as in the original proof. To retrieve the largest two
elements in 𝐿𝑖we observe that the largest element corresponds to the root of the implicitly defined
heap. Then, the second largest element corresponds to one of the children of the root of the heap.
Thus, we can run the traversal procedure to obtain the first element, which is in fact the largest

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 19 -->

Optimal Enumeration of Regular Pattern Matches
112:19

element. Then, as explained above, as we have already discovered the root we can inspect its
children, and choose the second largest element in constant time.

Providing access to 𝑅𝑖. This is more complicated, because we need to enumerate the elements of
𝑅𝑖greater or equal to 𝑥, but Theorem 2.4 needs to be applied on the reversed fragment of the text,
and as such would only be able to enumerate the elements of 𝑅𝑖smaller or equal to 𝑥. To overcome
this difficulty, we will observe that there is a certain monotonicity regarding the parameter 𝑥in
subsequent calls to the enumeration procedure. In particular, we always first enumerate all elements
of 𝑅𝑖, i.e., with 𝑥being the center of the corresponding epoch, which can be done with Theorem 2.4.
Then, if we now need to enumerate elements of 𝑅𝑖greater or equal to 𝑥then earlier during the
execution of the algorithm we have enumerated elements of 𝑅𝑖greater or equal to some 𝑥′ < 𝑥.
This allows us to reuse the results of the previous enumeration.

In a recursive call enum(𝑖,𝑏), we want to enumerate the occurrences of 𝑤𝑖starting at positions in
[𝑗𝑖−1+|𝑤𝑖−1| : 𝑟𝑖]. For each such occurrence at position𝑥, we then call enum(𝑖, not 𝑏), where𝑖′ = 𝑖+1
or 𝑖′ = jump(𝑖+ 1), to enumerate the occurrences of 𝑤𝑖′ starting at positions [𝑗𝑖′−1 + |𝑤𝑖′−1| : 𝑟𝑖′],
and so on. In every such recursive call, we will first enumerate the occurrences in 𝐿𝑖(without any
assumptions about their order). Then, we will enumerate the precomputed occurrences in 𝑀𝑖(in
the sorted order). Finally, we will enumerate the occurrences in 𝑅𝑖by running Theorem 2.4. We
conceptually arrange all occurrences of 𝑤𝑖in 𝑀𝑖∪𝑅𝑖to form a heap 𝐻𝑖. More precisely, we first
construct a path formed by listing the elements of 𝑀𝑖in the sorted order. Then, we attach as the
only child of the last node of the path the root of the heap obtained by running Theorem 2.4 to
enumerate all elements of 𝑅𝑖. We stress that initially only the path is constructed, and the remaining
part of 𝐻𝑖is explored on demand. More precisely, if we have already reached a node of 𝐻𝑖then in
constant time we can reach its children. Further, for every position 𝑥in the middle part 𝑀𝑃𝑖we
store a pointer to the node of 𝐻𝑖that corresponds to the successor of 𝑥in 𝑀𝑖∪𝑅𝑖, which is always
a node of the path or the only child of its last node. The preprocessing takes only O(𝑛) time and
space, as the total length of all middle parts is O(𝑛).

We maintain the following invariant inside a recursive call enum(𝑖,𝑏): if the currently considered
occurrence of 𝑤𝑖is from 𝑀𝑖∪𝑅𝑖, then we already have the list of all relevant occurrences of 𝑤𝑖′ in
𝑀𝑟′ ∪𝑅𝑖′ (where 𝑖′ might be in a different epoch than 𝑖). We think that every node of 𝐻𝑖stores a
list of nodes of 𝐻𝑖′. However, these lists are generated during the recursion to make sure that if we
are considering an occurrence corresponding to some node of 𝐻𝑖then its list is already prepared.

The base case is that we need to prepare the list for the root of 𝐻𝑖. We observe that the very first
recursive call concerning some string 𝑤𝑖, that is not the last in its epoch, starts with an occurrence
in 𝐿𝑖. Thus, when considering an occurrence 𝑥in 𝐿𝑖we can prepare for the first occurrence in
𝑀𝑖∪𝑅𝑖. More precisely, the subsequent recursive call first enumerates the relevant occurrences
in 𝐿𝑖′ using Theorem 2.4, and then enumerates the relevant occurrences in 𝑀𝑖′ ∪𝑅𝑖′. The latter
is done by traversing 𝐻𝑖′ starting either at the root or (if 𝑥+ |𝑤𝑖| falls within 𝑀𝑃𝑖′) at the node
corresponding to the preprocessed successor of 𝑥+ |𝑤𝑖| −1 in 𝑀𝑖′ ∪𝑅𝑖′, which can be accessed in
constant time. During the traversal, we create a list of nodes that should be stored for the root of
𝐻𝑖. For a string 𝑤𝑖that is the last in its epoch, 𝐿𝑖might be empty, so we start with an occurrence in
𝑀𝑖. The subsequent recursive call enumerates all occurrences of 𝑤𝑖′ that is the first in its epoch
(observe that this is possibly after skipping multiple epochs that consist of single patterns, that is,
𝑖′ could be larger than 𝑖+ 1), and thus has an empty 𝑀𝑖′ ∪𝑅𝑖′, so there is nothing to store for the
root of 𝐻𝑖.

In the general case, when considering an occurrence of 𝑤𝑖at position 𝑥corresponding to a node
𝑢of 𝐻𝑖, we prepare for considering the at most two subsequent occurrences of 𝑤𝑖at positions 𝑥′ and
𝑥′′, with 𝑥< 𝑥′,𝑥′′, corresponding to the children of 𝑢in 𝐻𝑖. More precisely, when enumerating

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 20 -->

112:20
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

m1

m2

m3

x1

x2

x4

x8
x9

x3

x6
x7

Conceptual heap for Mi ∪Ri

Mi

Ri

r1

r2

r4

r8
r9

r10
r11

r3

r6

r12

r7

Conceptual heap for Ri′

Fig. 3. The heap 𝐻𝑖associated to 𝑀𝑖∪𝑅𝑖(left) and the part of the heap 𝐻𝑖′ associated to 𝑀𝑖′ ∪𝑅𝑖′ corresponding
to 𝑅𝑖′ (right). The nodes 𝑥𝑡correspond to the occurrences of 𝑤𝑖in 𝑀𝑖∪𝑅𝑖, arranged in a heap. The nodes 𝑟𝑡
correspond to the occurrences of 𝑤𝑖′ in 𝑅𝑖′. Every node 𝑥𝑡keeps a list of the occurrences of 𝑤𝑖′ relevant for
the occurrence of 𝑤𝑖corresponding to 𝑥𝑡. In this figure, for a coloured node 𝑥𝑡from the left heap, the box of
the same colour on the right side denotes the list of 𝑥𝑡. Note that any occurrence relevant for a child of 𝑥𝑡is
also relevant for 𝑥𝑡.

the relevant occurrences of 𝑤𝑖′ in 𝑀𝑖′ ∪𝑅𝑖′ we check if they will be relevant for 𝑥′ and 𝑥′′, and if so
add them to the corresponding list. This is illustrated in Figure 3.

Conclusion. By providing access to the elements of 𝑂𝑖as described above, we can run again our
enumeration procedure, and obtain Theorem 1.3. As far as the space consumption is concerned, we
note that, in this case, we will end up, as in the previous cases, storing, for every 𝑖∈[𝑘], the lists
𝑂𝑖completely and, additionally, for each element of each set 𝑀𝑖∪𝑅𝑖a pointer to a list of elements
(from the conceptual heap) of 𝑀𝑖′ ∪𝑅𝑖′ (for some 𝑖′ > 𝑖). Overall, adding this to the space used by
the data structures constructed in the preprocessing of all our algorithms, this leads to a total space
consumption of O(𝑛+ �𝑘

𝑖=1 |𝑂𝑖| + 𝑛(�𝑘

𝑖=1 |𝑂𝑖|)) = 𝑂(𝑛min{|𝑆|,𝑛𝑘}).

Theorem 1.3. We can enumerate the set 𝑆of all valid embeddings of 𝛼in𝑇with worst-case constant
delay, after a preprocessing running in O(𝑛) time. The used space is O(𝑛min{|𝑆|,𝑛𝑘}).

6
An Index for the Enumeration of Regular Pattern Matches

The problem we approach in this section is the following:

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 21 -->

Optimal Enumeration of Regular Pattern Matches
112:21

Index for the Enumeration of Regular Patterns Matches (EnumMatchReg)
Input: Text 𝑇[1 : 𝑛].
Task: Preprocess 𝑇for the following queries: given a regular pattern 𝛼= (�𝑘

𝑖=0 𝑤𝑖𝑥𝑖)𝑤𝑘+1,
report all valid substitutions (as valid embeddings) for 𝛼and 𝑇.

Preprocessing and data structures. As before, we construct for 𝑇its suffix tree 𝑆𝑇and suffix array
𝑆𝐴, in O(𝑛) time. Now, we build an additional data structure W which has the following properties:

• W is a balanced, binary, ordered tree. Each node 𝑣of this tree is associated with a range
[𝑎: 𝑏] of the suffix array 𝑆𝐴. The node 𝑣stores the associated range [𝑎: 𝑏] as well as an
(increasingly) ordered doubly-linked list containing the elements of 𝑆𝐴[𝑎: 𝑏].
• If 𝑣is a leaf of W, then its range is [𝑖: 𝑖], for some 𝑖∈[𝑛], corresponding to position 𝑖of 𝑆𝐴.
• If 𝑣is an internal node of W, associated to the range [𝑎: 𝑏], with children 𝑣1 and 𝑣2, then
𝑎< 𝑏and the range of 𝑣1 is [𝑎: 𝑐] and the range of 𝑣2 is [𝑐+ 1 : 𝑏], for some 𝑐∈[𝑎: 𝑏−1].
Moreover, for each element 𝑟= 𝑆[𝑖] occurring in the ordered list stored in 𝑣(i.e., 𝑎≤𝑖≤𝑏),
we store pointers to the predecessor and the successor of 𝑟in the ordered lists of 𝑣1 and 𝑣2.

The tree W is strongly related to both segment trees [12] and wavelet trees [32]. Its construction
can be performed in O(𝑛log𝑛) time and space, as explained in [9].

Lemma 6.1 ([9]). W can be constructed in O(𝑛log𝑛) time and space.

Proof. We briefly sketch the way W is constructed.
W is constructed in rounds, bottom-up. In each round, we have a list 𝐿= (𝑢1, . . . ,𝑢ℓ) of nodes,
sorted increasingly by the left border of the range they represent. The ranges of every two nodes
are disjoint, and their union is [1 : 𝑛]. Initially, we have the list of leaves (ordered by the position of
𝑆𝐴they represent). In each round, we define a new node 𝑣𝑖which has as children the nodes 𝑢2𝑖−1
and 𝑢2𝑖, on positions 2𝑖−1 and 2𝑖of the list 𝐿as children (for 𝑖from 1 to ⌊ℓ/2⌋). The range of 𝑣𝑖is
the union of the ranges of 𝑢2𝑖−1 and 𝑢2𝑖, and the ordered list stored in 𝑣𝑖is obtained by merging the
ordered lists of 𝑢2𝑖−1 and 𝑢2𝑖. Note that, during the merging process, we can trivially compute for
every element of the ordered list stored in 𝑣𝑖the predecessor and the successor of that element
in the ordered lists of 𝑢2𝑖−1 and 𝑢2𝑖. All the new nodes and, in the case when ℓwas odd, the last
node of 𝐿are considered as the input list for the next round. Each round can be implemented in
O(𝑛) time (proportional to the total size of the merged lists), and there are O(log𝑛) rounds until
we obtain a list with a single node. The conclusion follows.
□

For a range [𝑎: 𝑏], with 𝑎,𝑏∈[𝑛], we say that a node 𝑣of W is a canonical node for [𝑎: 𝑏] if 𝑣
is associated to a range [𝑐: 𝑑] with 𝑎≤𝑐≤𝑑≤𝑏(i.e., the range [𝑐: 𝑑] is included in the range

[𝑎: 𝑏]) and the range of the parent node of 𝑣is not included in [𝑎: 𝑏]. It can be shown that for each
range [𝑎: 𝑏] there are at most O(log𝑛) canonical nodes in W (see, e.g., [32] and the references
therein). One can show, in fact, the next lemmas:

Lemma 6.2. The following queries can be answered in O(log𝑛) time each using W:

• cNodes(𝑎,𝑏): returns the list of the canonical nodes of the range [𝑎: 𝑏].
• cNodesPred(𝑎,𝑏,𝑟): for a value 𝑟∈[𝑛], returns a list of pointers to the predecessors of 𝑟in the
ordered lists of the canonical nodes of [𝑎: 𝑏]; the returned list is decreasingly sorted w.r.t. the
values pointed by its elements (i.e., the predecessors of 𝑟).
• cNodesSucc(𝑎,𝑏, ℓ): for a value ℓ∈[𝑛], returns a list of pointers to the successors of ℓin the
ordered lists of the canonical nodes of [𝑎: 𝑏]; the returned list is increasingly sorted w.r.t. the
values pointed by its elements (i.e., the successors of ℓ).

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 22 -->

112:22
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

Proof. The answer to cNodes(𝑎,𝑏) is computed as follows: We traverse W recursively. Let 𝑣be
the node currently considered in the traversal. If the range [𝑐: 𝑑] associated to 𝑣is included in

[𝑎: 𝑏], then we output 𝑣and do not recurse in the subtree of 𝑣. If the intersection of the range [𝑐: 𝑑]
and [𝑎: 𝑏] is empty, we return without producing an output and do not recurse in the subtree of 𝑣.
Otherwise, we recursively consider the children of 𝑣. Given that the number of canonical nodes for

[𝑎: 𝑏] is O(log𝑛), the claim follows.

To answer cNodesPred(𝑎,𝑏,𝑟) we proceed similarly, with the single difference being that we
now also have the current predecessor of 𝑟as a parameter 𝑝. We also compute a list 𝐿. We start
our recursive traversal in the root of W with 𝑝= 𝑟. We traverse W recursively. Let 𝑣be the node
currently considered in the traversal, and 𝑝be the predecessor-parameter. If the range [𝑐: 𝑑]
associated to 𝑣is included in [𝑎: 𝑏], then we put 𝑝in the output list 𝐿and do not recurse in the
subtree of 𝑣. If the intersection of the range [𝑐: 𝑑] and [𝑎: 𝑏] is empty, we return without adding
anything to 𝐿and do not recurse in the subtree of 𝑣. Otherwise, we recursively consider the children
𝑣1 and 𝑣2 of 𝑣, with the predecessor-parameters being, respectively, 𝑝1 the predecessor of 𝑝in the
ordered list stored in 𝑣1 (corresponding to the range of the suffix array covered by 𝑣1), and 𝑝2, the
predecessor of 𝑝in the ordered list stored in 𝑣2. Note that these predecessors are precomputed.
Given that the number of canonical nodes for [𝑎: 𝑏] is O(log𝑛), this process ends in O(log𝑛) time
and produces an unsorted list. Now, we can sort this list in O(log𝑛) time, using an algorithm based
on atomic heaps, as described in [21].

The queries cNodesSucc(𝑎,𝑏, ℓ) are answered analogously.
□

Lemma 6.3. After running cNodesPred(𝑎,𝑏,𝑟), we can:

• for any given ℓ∈[𝑛], enumerate with constant delay the elements 𝑆[𝑖] of 𝑆[𝑎: 𝑏] such that
ℓ≤𝑆[𝑖] ≤𝑟, starting with the largest one.
• return the largest and second largest elements of 𝑆[𝑎: 𝑏], which are not larger than 𝑟.

Lemma 6.4 ([10]). Provided the suffix tree 𝑆𝑇(respectively, the suffix array 𝑆𝐴) of a text 𝑇, for a
given string 𝑤, of length 𝑚≤𝑛, we can find the range 𝑆𝐴[𝑎𝑤: 𝑏𝑤] containing exactly the positions 𝑗
such that 𝑇[𝑗: 𝑗+ |𝑤| −1] = 𝑤in O(𝑚+ log𝑛) time.

Processing a query. Assume that we are now given a query 𝛼= 𝑥0(�𝑘

𝑖=0 𝑤𝑖𝑥𝑖), and we want to
enumerate all the embeddings of 𝛼in 𝑇. We show how we can produce all the data structures
needed to perform the enumeration algorithms from the previous sections.

We proceed as follows, for each 𝑖∈[𝑘]. Firstly, we use Theorem 6.4 to compute for each 𝑤𝑖the
range 𝑆𝐴[𝑎𝑖: 𝑏𝑖] containing exactly the positions 𝑗such that 𝑇[𝑗: 𝑗+ |𝑤𝑖| −1] = 𝑤𝑖. This takes,
in total, O(|𝛼| + 𝑘log𝑛) time.

Now, we can find the right-canonical (respectively, left-canonical) embedding (𝑟1, . . . ,𝑟𝑘) (re-
spectively, (ℓ1, . . . , ℓ𝑘)) for 𝛼and 𝑇. We define 𝑟𝑘to be the largest element of 𝑆[𝑎𝑘: 𝑏𝑘] (can
be obtained in O(log𝑛), using Theorem 6.3). Then, for 𝑖from 𝑘−1 to 1, we execute the query
cNodesPred(𝑎𝑖,𝑏𝑖,𝑟𝑖+1) as in Theorem 6.2 and obtain the list I𝑖of pointers to the predecessors
of 𝑟𝑖+1 in the ordered lists of the canonical nodes of [𝑎𝑖: 𝑏𝑖]; the returned list is decreasingly
sorted w.r.t. the values pointed by its elements; we define 𝑟𝑘as being the element pointed by the
first element in the list returned by cNodesPred(𝑎𝑖,𝑏𝑖,𝑟𝑖+1). In total, computing 𝑟1, . . . ,𝑟𝑘takes
O(𝑘log𝑛) time. The left-canonical embedding (ℓ1, . . . , ℓ𝑘) is computed by a symmetrical procedure.

Further, we can also compute the largest two elements in 𝑆[𝑎𝑖: 𝑏𝑖] smaller or equal to 𝑟𝑖, for all
𝑖∈[𝑘], by Theorem 6.3. This takes, again, O(𝑘log𝑛) time.

At this point, recall Theorem 6.3 and note that the list I𝑖offers us a way to enumerate one by
one the elements of the set 𝑂𝑖(defined as in Section 3, for 𝛼and 𝑇), which are greater or equal
to a given number ℓ, with constant delay. We consider the current element of the list I𝑖(initially,

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 23 -->

Optimal Enumeration of Regular Pattern Matches
112:23

the current element is the first pointer of this list). This points to an element 𝑡in the ordered list
of some canonical node 𝑣. If 𝑡≥ℓ, then we return, one by one, with constant delay, by simply
traversing the ordered list of 𝑣, all elements smaller or equal to 𝑡and greater or equal to ℓ. Once
there are no more such elements in the list of 𝑣, we advance in the list I𝑖and repeat this procedure
for the next element of that list (unless it points to an element smaller than ℓ, when we simply
stop).

Enumeration. We can now directly use Theorem 3.6 to enumerate the valid embeddings of 𝛼in𝑇,
as its requirements hold. First, we are able to enumerate, one by one, in O(1) time per element, the
elements of 𝑂𝑖which are greater or equal to the (𝑖−1)th component of the embedding, starting
with 𝑟𝑖. This is possible, as described above, once the list I𝑖is computed. Second, we are able to
retrieve the largest two elements of the set 𝑂𝑖. As noted in Section 4, in Theorem 3.6, this leads to an
enumeration algorithm with constant delay for all the valid embeddings for 𝛼and 𝑇. In conclusion,
we obtain Theorem 1.4. The space needed is O(𝑛log𝑛) for the initial preprocessing, and, then,

O(|𝛼| + 𝑛𝑘) for the preprocessing of 𝛼and the enumeration of its valid embeddings.

Theorem 1.4. We can preprocess the text 𝑇in O(𝑛log𝑛) time, so that when given a pattern 𝛼
with 𝑘variables, after an additional processing running in O(|𝛼| + 𝑘log𝑛) time, the set 𝑆of all valid
embeddings of 𝛼in 𝑇can be enumerated with constant delay.

7
Conclusion
We have considered the matching problem for regular patterns. This problem naturally lies in
the intersection of information extraction and stringology. We have designed an enumeration
algorithm that preprocesses the document and the query pattern in optimal linear time, and then
enumerates the matches with worst-case constant delay. While our techniques are perhaps specific
to the considered class of regular expressions, we hope that our work will trigger further interest
in transferring methods from stringology and data structures to information extraction.

Acknowledgments
Paweł Gawrychowski is partially supported by the Polish National Science Centre grant number
2023/51/B/ST6/01505.
Florin Manea, Paul Sarnighausen-Cahn and Stefan Siemer are supported by the German Research
Foundation (Deutsche Forschungsgemeinschaft, DFG) through the Heisenberg project 466789228
and the research project 562495345.

References

[1] Alfred V. Aho, John E. Hopcroft, and Jeffrey D. Ullman. 1974. The Design and Analysis of Computer Algorithms.

Addison-Wesley.
[2] Antoine Amarilli, Pierre Bourhis, Stefan Mengel, and Matthias Niewerth. 2021. Constant-Delay Enumeration for

Nondeterministic Document Spanners. ACM Trans. Database Syst. 46, 1 (2021), 2:1–2:30. doi:10.1145/3436487
[3] Antoine Amarilli and Mikaël Monet. 2023. Enumerating Regular Languages with Bounded Delay. In 40th International

Symposium on Theoretical Aspects of Computer Science, STACS 2023, Hamburg, Germany, March 7-9, 2023 (LIPIcs,
Vol. 254), Petra Berenbrink, Patricia Bouyer, Anuj Dawar, and Mamadou Moustapha Kanté (Eds.). Schloss Dagstuhl -
Leibniz-Zentrum für Informatik, 8:1–8:18. doi:10.4230/LIPICS.STACS.2023.8
[4] Dana Angluin. 1980. Finding Patterns Common to a Set of Strings. J. Comput. Syst. Sci. 21, 1 (1980), 46–62. doi:10.1016/

0022-0000(80)90041-0
[5] Arturs Backurs and Piotr Indyk. 2016. Which Regular Expression Patterns Are Hard to Match?. In FOCS. IEEE Computer

Society, 457–466.
[6] Guillaume Bagan. 2006. MSO Queries on Tree Decomposable Structures Are Computable with Linear Delay. In

Computer Science Logic, 20th International Workshop, CSL 2006, 15th Annual Conference of the EACSL, Szeged, Hungary,

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 24 -->

112:24
Paweł Gawrychowski, Florin Manea, Paul Sarnighausen-Cahn, & Stefan Siemer.

September 25-29, 2006, Proceedings (Lecture Notes in Computer Science, Vol. 4207), Zoltán Ésik (Ed.). Springer, 167–181.
doi:10.1007/11874683_11
[7] Michael A. Bender and Martin Farach-Colton. 2000. The LCA Problem Revisited. In LATIN 2000: Theoretical Informatics,

4th Latin American Symposium, Punta del Este, Uruguay, April 10-14, 2000, Proceedings (Lecture Notes in Computer
Science, Vol. 1776), Gaston H. Gonnet, Daniel Panario, and Alfredo Viola (Eds.). Springer, 88–94. doi:10.1007/10719839_9
[8] Karl Bringmann, Allan Grønlund, Marvin Künnemann, and Kasper Green Larsen. 2024. The NFA Acceptance Hypothesis:

Non-Combinatorial and Dynamic Lower Bounds. TheoretiCS 3 (2024).
[9] Bernard Chazelle. 1988. A Functional Approach to Data Structures and Its Use in Multidimensional Searching. SIAM J.

Comput. 17, 3 (1988), 427–462. doi:10.1137/0217026
[10] Richard Cole and Moshe Lewenstein. 2003. Multidimensional matching and fast search in suffix trees. In Proceedings of

the Fourteenth Annual ACM-SIAM Symposium on Discrete Algorithms, January 12-14, 2003, Baltimore, Maryland, USA.
ACM/SIAM, 851–852. http://dl.acm.org/citation.cfm?id=644108.644251
[11] Bruno Courcelle. 2009. Linear delay enumeration and monadic second-order logic. Discret. Appl. Math. 157, 12 (2009),

2675–2700. doi:10.1016/J.DAM.2008.08.021
[12] Mark de Berg, Otfried Cheong, Marc J. van Kreveld, and Mark H. Overmars. 2008. Computational geometry: algorithms

and applications, 3rd Edition. Springer. doi:10.1007/978-3-540-77974-2
[13] Ronald Fagin, Benny Kimelfeld, Frederick Reiss, and Stijn Vansummeren. 2015. Document Spanners: A Formal

Approach to Information Extraction. J. ACM 62, 2 (2015), 12:1–12:51. doi:10.1145/2699442
[14] M. Farach. 1997. Optimal suffix tree construction with large alphabets. In Proceedings 38th Annual Symposium on

Foundations of Computer Science. 137–143. doi:10.1109/SFCS.1997.646102
[15] Henning Fernau, Florin Manea, Robert Mercas, and Markus L. Schmid. 2018. Revisiting Shinohara’s algorithm for

computing descriptive patterns. Theor. Comput. Sci. 733 (2018), 44–54. doi:10.1016/J.TCS.2018.04.035
[16] Henning Fernau, Florin Manea, Robert Mercas, and Markus L. Schmid. 2020. Pattern Matching with Variables: Efficient

Algorithms and Complexity Results. ACM Trans. Comput. Theory 12, 1 (2020), 6:1–6:37. doi:10.1145/3369935
[17] Henning Fernau and Markus L. Schmid. 2015. Pattern matching with variables: A multivariate complexity analysis.

Inf. Comput. 242 (2015), 287–305. doi:10.1016/J.IC.2015.03.006
[18] Henning Fernau, Markus L. Schmid, and Yngve Villanger. 2016. On the Parameterised Complexity of String Morphism

Problems. Theory Comput. Syst. 59, 1 (2016), 24–51. doi:10.1007/S00224-015-9635-3
[19] Fernando Florenzano, Cristian Riveros, Martín Ugarte, Stijn Vansummeren, and Domagoj Vrgoc. 2018. Constant Delay

Algorithms for Regular Document Spanners. In Proceedings of the 37th ACM SIGMOD-SIGACT-SIGAI Symposium on
Principles of Database Systems, Houston, TX, USA, June 10-15, 2018. ACM, 165–177. doi:10.1145/3196959.3196987
[20] Michael L. Fredman and Dan E. Willard. 1990. BLASTING through the Information Theoretic Barrier with FUSION-

TREES. In Proceedings of the 22nd Annual ACM Symposium on Theory of Computing, May 13-17, 1990, Baltimore,
Maryland, USA, Harriet Ortiz (Ed.). ACM, 1–7. doi:10.1145/100216.100217
[21] Michael L. Fredman and Dan E. Willard. 1994. Trans-Dichotomous Algorithms for Minimum Spanning Trees and

Shortest Paths. J. Comput. Syst. Sci. 48, 3 (1994), 533–551. doi:10.1016/S0022-0000(05)80064-9
[22] Dominik D. Freydenberger, Benny Kimelfeld, and Liat Peterfreund. 2018. Joining Extractions of Regular Expressions.

In Proceedings of the 37th ACM SIGMOD-SIGACT-SIGAI Symposium on Principles of Database Systems, Houston, TX,
USA, June 10-15, 2018. ACM, 137–149. doi:10.1145/3196959.3196967
[23] Dominik D. Freydenberger and Markus L. Schmid. 2017. Deterministic Regular Expressions with Back-References. In

34th Symposium on Theoretical Aspects of Computer Science, STACS 2017, March 8-11, 2017, Hannover, Germany (LIPIcs,
Vol. 66), Heribert Vollmer and Brigitte Vallée (Eds.). Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 33:1–33:14.
doi:10.4230/LIPICS.STACS.2017.33
[24] André Frochaux and Sarah Kleest-Meißner. 2023. Puzzling over Subsequence-Query Extensions: Disjunction and

Generalised Gaps. In Proceedings of the 15th Alberto Mendelzon International Workshop on Foundations of Data Manage-
ment (AMW 2023), Santiago de Chile, Chile, May 22-26, 2023 (CEUR Workshop Proceedings, Vol. 3409). CEUR-WS.org.
https://ceur-ws.org/Vol-3409/paper3.pdf
[25] Leslie Ann Goldberg. 1991. Efficient algorithms for listing combinatorial structures. Ph. D. Dissertation. University of

Edinburgh, UK. https://hdl.handle.net/1842/10917
[26] David S. Johnson, Christos H. Papadimitriou, and Mihalis Yannakakis. 1988. On Generating All Maximal Independent

Sets. Inf. Process. Lett. 27, 3 (1988), 119–123. doi:10.1016/0020-0190(88)90065-8
[27] Wojciech Kazana and Luc Segoufin. 2013. Enumeration of monadic second-order queries on trees. ACM Trans. Comput.

Log. 14, 4 (2013), 25:1–25:12. doi:10.1145/2528928
[28] Donald Ervin Knuth. 2014. The Art of Computer Programming, Volume 4A: Combinatorial Algorithms, Part 1. Addison-

Wesley.
[29] Donald E. Knuth, James H. Morris, Jr., and Vaughan R. Pratt. 1977. Fast Pattern Matching in Strings. SIAM J. Comput.

6, 2 (1977), 323–350. doi:10.1137/0206024

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.

<!-- page 25 -->

Optimal Enumeration of Regular Pattern Matches
112:25

[30] Florin Manea and Markus L. Schmid. 2019. Matching Patterns with Variables. In Combinatorics on Words - 12th

International Conference, WORDS 2019, Loughborough, UK, September 9-13, 2019, Proceedings (Lecture Notes in Computer
Science, Vol. 11682). Springer, 1–27. doi:10.1007/978-3-030-28796-2_1
[31] Torsten Mütze. 2022. Combinatorial Gray codes-an updated survey. CoRR abs/2202.01280 (2022). arXiv:2202.01280

https://arxiv.org/abs/2202.01280
[32] Gonzalo Navarro. 2012. Wavelet Trees for All. In Combinatorial Pattern Matching - 23rd Annual Symposium, CPM 2012,

Helsinki, Finland, July 3-5, 2012. Proceedings (Lecture Notes in Computer Science, Vol. 7354), Juha Kärkkäinen and Jens
Stoye (Eds.). Springer, 2–26. doi:10.1007/978-3-642-31265-6_2
[33] Matthias Niewerth and Luc Segoufin. 2018. Enumeration of MSO Queries on Strings with Constant Delay and

Logarithmic Updates. In Proceedings of the 37th ACM SIGMOD-SIGACT-SIGAI Symposium on Principles of Database
Systems, Houston, TX, USA, June 10-15, 2018, Jan Van den Bussche and Marcelo Arenas (Eds.). ACM, 179–191. doi:10.
1145/3196959.3196961
[34] Takeshi Shinohara. 1982. Polynomial Time Inference of Extended Regular Pattern Languages. In RIMS Symposium on

Software Science and Engineering, Kyoto, Japan, 1982, Proceedings (Lecture Notes in Computer Science, Vol. 147). Springer,
115–127. doi:10.1007/3-540-11980-9_19

Received December 2025; accepted February 2026

Proc. ACM Manag. Data, Vol. 4, No. 2 (PODS), Article 112. Publication date: May 2026.
