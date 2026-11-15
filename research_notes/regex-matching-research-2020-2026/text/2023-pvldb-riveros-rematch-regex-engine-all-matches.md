---
file: ../pdfs/2023-pvldb-riveros-rematch-regex-engine-all-matches.pdf
citation: "REmatch: a novel regex engine for finding all matches; Riveros, Van Sint Jan, Vrgoč; PVLDB 16(11); 2023"
doi: 
source: "https://www.vldb.org/pvldb/vol16/p2792-vrgoc.pdf"
topics: [semantics_theory_access]
pages: 13
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

REmatch: a novel regex engine for finding all matches
Cristian Riveros

Nicolás Van Sint Jan

Domagoj Vrgoč

PUC Chile & IMFD Chile
cristian.riveros@uc.cl

PUC Chile & IMFD Chile
nicovsj@uc.cl

PUC Chile & IMFD Chile
vrdomagoj@uc.cl

ABSTRACT

The leftmost-longest semantics is standard for RegEx engines, as
it captures the majority of meaningful matches, although not all of
them. However, in some scenarios adopting an “all-match semantics”
is a valuable and desirable feature for the users. For instance, in
DNA analysis we will often need to match patterns (called motifs)
onto a DNA sequence, and these can overlap. The question of
finding overlapping matches with RegEx is also recurrent in user
discussions [21, 22, 42]. For information extraction, the all-match
semantics leaves freedom to the user to extract all positions, called
spans, where there is relevant information in a document. Therefore
the all-match semantics is a desirable feature for RegEx engines
that, to the best of our knowledge, no engine supports natively.
To overcome the problem of finding all-matches, RegEx engines
offer look-around operators, namely, operators that allows checking if a subexpression can be matched forward or backward from
the current position, without advancing from the current position.
For instance, by using look-around, we can modify the expression
aa to (?=(aa)) and find the missing match 𝑎 1𝑎 2 over the above
document. Despite this example, look-around operators cannot discover all matches for every RegEx expression. For instance, consider
the document 𝑎𝑏𝑐𝑑 and the RegEx [abcd]+. This expression asks
to find a match of one or more letters, which is satisfied by the
substrings 𝑎, 𝑎𝑏, 𝑎𝑏𝑐, 𝑎𝑏𝑐𝑑, 𝑏, 𝑏𝑐, etc. A typical RegEx engine will
only output 𝑎𝑏𝑐𝑑. Instead, this time look-ahead will not help us.
The most obvious way would be to use an expression of the form
(?=([abcd]+)); however, the engine will only output the matches
𝑎𝑏𝑐𝑑, 𝑏𝑐𝑑, 𝑐𝑑, and 𝑑, and 𝑎 or 𝑎𝑏 will be omitted, since they both
start at the same position 0 like 𝑎𝑏𝑐𝑑 (see also Example 2.4).
In terms of implementation, RegEx engines are usually divided
into three categories: DFA-based, NFA-based, and recursive NFAbased [18]. DFA is generally the fastest evaluation strategy, followed
by NFA. In contrast, recursive NFA-based engines use backtracking,
which is susceptible to well-documented performance issues, like
regular expression denial of service attacks (ReDos) [18], where
the engine can exhibit exponential time performance [10]. From
the positive side, recursive NFA-based engines have the advantage
of keeping track of the evaluation, which allows implementing
operators like look-around and back-references. In summary, until
now, the only way of finding all matches (in some cases) is by
using look-around operators implemented by recursive NFA-based
engines, which suffer from unfortunate performance issues.

In this paper, we present the REmatch system for information
extraction. REmatch is based on a recently proposed enumeration
algorithm for evaluating regular expressions with capture variables
supporting the all-match semantics. It tells a story of what it takes
to make a theoretically optimal algorithm work in practice. As we
show here, a naive implementation of the original algorithm would
have a hard time dealing with realistic workloads. We thus develop
a new algorithm and a series of optimizations that make REmatch
as fast or faster than many popular RegEx engines while at the
same time being able to return all the outputs: a task that most
other engines tend to struggle with.
PVLDB Reference Format:
Cristian Riveros, Nicolás Van Sint Jan, and Domagoj Vrgoč. REmatch: a
novel regex engine for finding all matches. PVLDB, 16(11): 2792 - 2804,
2023.
doi:10.14778/3611479.3611488
PVLDB Artifact Availability:
The source code, data, and/or other artifacts have been made available at
https://github.com/REmatchChile/REmatch-paper.

1

INTRODUCTION

Regular expressions, or RegEx, are one of the most used technologies for managing text data. The development of RegEx engines
started in the early 70s [26, 44], and they are now a common part of
many complex information systems such as compilers, databases,
or search engines. Moreover, modern RegEx engines are highlyoptimized systems that are crucial for finding patterns in diverse
areas like biology [33], literature [14], or medicine [17].
Given a regular expression and a document, the task of a RegEx
engine is to find all occurrences, or matches, of the pattern in the
document. For this, RegEx engines deploy the so-called leftmostlongest paradigm [25], meaning that they find the match which
is the leftmost one, and from there they find the longest possible
match. The process is then repeated starting from the rightmost
position of the previous match. For example, if we want to evaluate
the RegEx aa over the document 𝑎 0𝑎 1𝑎 2𝑎 3 (here the subindices
are for referencing positions; the document consists of the letter 𝑎
repeated four times), a typical RegEx engine will output the matches
𝑎 0𝑎 1 and 𝑎 2𝑎 3 . In particular, RegEx engines will not output 𝑎 1𝑎 2
since the first leftmost-longest match ends with 𝑎 1 .

Contribution. To overcome these issues, this paper presents REmatch, a RegEx engine supporting the all-match semantics, and
its accompanying regular expression language REQL. Contrary to
the status quo of RegEx evaluation, REmatch is based on a new
evaluation strategy, inspired by the theory of enumeration algorithms [40], that allows finding all the matches, and avoids the
exponential behavior of recursive NFA evaluation. Moreover, REmatch performance is comparable to popular RegEx engines, while

This work is licensed under the Creative Commons BY-NC-ND 4.0 International
License. Visit https://creativecommons.org/licenses/by-nc-nd/4.0/ to view a copy of
this license. For any use beyond those covered by this license, obtain permission by
emailing info@vldb.org. Copyright is held by the owner/author(s). Publication rights
licensed to the VLDB Endowment.
Proceedings of the VLDB Endowment, Vol. 16, No. 11 ISSN 2150-8097.
doi:10.14778/3611479.3611488

2792

<!-- page 2 -->

at the same time finding all the matches, thus obtaining the best of
both worlds. Specific contributions of the paper are as follows:

adopts the classical RegEx syntax (e.g., POSIX Basic Regular Expressions), which is familiar to most users. However, the REQL
semantics for capture variables differs from classical RegEx: it finds
all appearances of a pattern in the document. This “all-match” semantics allows for defining meaningful queries that classical RegEx
engines cannot express (see examples below). Therefore, this new
semantics motivates the need for a new query language, REQL.

(1) We introduce the REQL query language, which extends
classical RegEx with variables and the all-match semantics.
(2) We present REmatch, a RegEx system that allows evaluating
REQL using output-linear delay. For this, we develop a new
evaluation method extending the algorithm of [16], and
allowing REmatch to compete with modern RegEx engines.
(3) We develop a set of experiments to evaluate the effect of
different optimizations on REmatch performance, and compare it to existing RegEx engines.

Documents and spans. We follow the theoretical framework of
documents and spans introduced in [15]. For us, a document 𝑑 is
simply a string over some finite alphabet (e.g. the ASCII charset,
UTF-8, or a similar encoding scheme)1 . We write 𝑑 = 𝑎 0𝑎 1 . . . 𝑎𝑛−1
to denote a document of length |𝑑 | = 𝑛 where 𝑎𝑖 is the 𝑖-symbol
and the first symbol starts from 02 . An example of a document is:

Related work. Regular expressions are a classical formalism in
computer science, introduced by Kleene [28]. The first appearance
of regular expressions in program form (RegEx) is credited to Thomson [44] and was initially conceived for pattern matching and lexical
analysis. RegEx has been implemented by different tools (e.g., AWK,
grep, PCRE) and widely included in programming languages (e.g.,
PHP, Python, Java), with the leftmost longest semantics being the
de-facto standard [18].
RegEx engines are highly optimized these days [11], with advancements in algorithms [7], hardware [41], or filtering [48, 49]
techniques being developed to speed-up their performance. All
these optimizations usually apply for pattern matching of RegEx,
and it is unclear how to apply them to finding all matches of a
RegEx in a text document. For instance, in [48, 49] all substrings
satisfying a RegEx are detected in an efficient manner. However, our
approach goes a step further and allows to nest capture variables
inside patterns, thus effectively requiring to detect all matches both
for the pattern and for some of its sub-patterns. Combining these
approaches seems a very promising line of future work.
The Document Spanners framework was first introduced by
Fagin et al. [15] as a formal approach for information extraction,
attracting a lot of research in the last few years [12]. We base
REmatch’s model and query language (REQL) on the Document
Spanners framework; however, there are several differences in its
syntax (e.g., REQL is a user-oriented language) and semantics (e.g.,
REQL is unanchored and disallows capturing 𝜖 substrings). A theoretical algorithm for non-deterministic variable-set automata was
also proposed and implemented in [4]. This implementation is experimental and preliminary tests with REmatch show that it runs
at least one order of magnitude slower over realistic query workloads. To the best of our knowledge, REmatch is the first practical
implementation based on the Document Spanners framework with
a performance comparable to popular RegEx engines.

𝑑 1 := t h a t h a t h a t
0 1 2 3 4 5 6 7 8 9

A span of a document 𝑑 (also called a match) is a pair 𝑠 = [𝑖, 𝑗⟩ of
natural numbers 𝑖 and 𝑗 with 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑑 |. In that case, 𝑠 is
associated with the continuous region of the document 𝑑 whose
content is the substring of 𝑑 from position 𝑖 to position 𝑗 − 1. We
denote this substring by 𝑑 (𝑠) or 𝑑 (𝑖, 𝑗). For instance, 𝑑 1 ( [0, 4⟩) =
that, since this is the content of the string 𝑑 in positions 0 through 3.
Notice that if 𝑖 = 𝑗, then 𝑑 (𝑠) = 𝑑 (𝑖, 𝑗) = 𝜀, the empty string. Given
two spans 𝑠 1 = [𝑖 1, 𝑗1 ⟩ and 𝑠 2 = [𝑖 2, 𝑗2 ⟩ such that 𝑗1 = 𝑖 2 , we define
their concatenation as 𝑠 1 · 𝑠 2 = [𝑖 1, 𝑗2 ⟩. The set of all spans of 𝑑 is
denoted by span(𝑑).
Syntax. Syntactically, REQL is similar to standard regular expressions, apart from a special construct !x{e}, which states that
a substring matching e should be stored into the variable name x.
Formally, the syntax of REQL queries can be defined as follows:
e

≔

a | . | [w] | [^w] | !x{e} |
ee | e|e | e* | e+ | e? | e{n,m}

Here, a is a character (e.g., ASCII charset or UTF-8), the dot symbol
is a wildcard for any character, and [w] or [^w] are a char class
or the negation of a char class, respectively, where w declares a
set of characters. We use the standard notation of ranges of ASCII
characters found in POSIX for declaring char classes (e.g. [a-z],
[A-Z0-9apt], etc) and write set(w) to denote the set of characters
represented by w (e.g. set(a-z) = {𝑎, 𝑏, ..., 𝑧}). Moreover, x is a variable name where the character ! is used to differentiate a variable
name from a letter or string of the alphabet. This, along with the
use of { and } for delimiting the captured subregex is the only
special notation where we differ from POSIX. Finally, n and m are
numbers such that 0 ≤ n ≤ m. In the REmatch system, REQL also
allow the usual regex abbreviations for character classes (e.g. \d
for a digit, or \w for a word, etc), however, we do not include them
in the formal definition in order to keep the presentation concise3 .

Outline. In Section 2 we introduce REQL. We then explain each
module of the REmatch architecture. Section 3 presents the rewriting module, Section 4 the filtering module, and Section 5 the output
module. Section 6 explains the evaluation algorithm of REmatch.
Section 7 puts all components together and displays the experimental comparison with other engines. We conclude in Section 8.

Example 2.1. To give a preliminary example of how REQL works,
assume that we would like to extract all the occurrences of the word
1 Note that a multi-line document is simply a single string.

2

REQL: A REGEX QUERY LANGUAGE FOR IE

2 In [15], the first position is 1. We use 0 to be compliant with programming languages

This section introduces REQL, a RegEx Query Language for information extraction, that we implement in REmatch. The language

and RegEx engines which use 0 as the start position.
3We remark that the start-of-file symbol (^) and end-of-file symbol ($) are currently
not supported in REmatch. However, adding them is a straightforward exercise.

2793

<!-- page 3 -->

Table 1: The inductive semantics of REQL queries.

“that" from a text document. This can be done in REQL as follows:
e0 := !x{that}

JeK𝑑 = {𝜇 | (𝑠, 𝜇) ∈ VeU𝑑 }

Intuitively, the query captures the positions of a substring that into
the variable 𝑥. This query also illustrates a key feature of our semantics
(defined below): there can be overlapping matches. To make this more
clear, consider again the document 𝑑 1 . The query above will result in
precisely three matches for the variable x, corresponding to the three
occurrences of the substring that in the document we are processing.
The first match will be in positions [0, 4⟩, the second in [3, 7⟩, and the
last match in [6, 10⟩. We notice that the middle match [3, 7⟩ will not
be captured by most regular expression tools, unless some sort of a
look-around operator is used.

VaU𝑑 = {(𝑠, ∅) | 𝑠 ∈ span(𝑑) and 𝑑 (𝑠) = a}
V.U𝑑 = {([𝑖, 𝑖 + 1⟩, ∅) | 0 ≤ 𝑖 < |𝑑 |}

V[w]U𝑑 = {(𝑠, ∅) | 𝑠 ∈ span(𝑑) and 𝑑 (𝑠) ∈ set(w)}

V[^w]U𝑑 = {(𝑠, ∅) | 𝑠 ∈ span(𝑑) and 𝑑 (𝑠) ∉ set(w)}

V!x{e}U𝑑 = {(𝑠, 𝜇) | ∃(𝑠, 𝜇 ′ ) ∈ VeU𝑑 : 𝑑 (𝑠) ≠ 𝜖,

x ∉ dom(𝜇 ′ ) and 𝜇 = 𝜇 ′ ∪ [𝑥 → 𝑠] }

Ve1 e2U𝑑 = {(𝑠, 𝜇) | ∃(𝑠 1, 𝜇1 ) ∈ Ve1U𝑑 . ∃(𝑠 2, 𝜇2 ) ∈ Ve2U𝑑 :
𝑠 = 𝑠 1 · 𝑠 2 and 𝜇 = 𝜇 1 ∪ 𝜇2 }

The reader could notice that the above syntax is so general that
one can define the capture of the same variable multiple times. For
instance, a query like !x{a!x{b}} defines the capture of x twice.
For this reason, REQL has some simple syntactic restrictions to
use variables correctly. Let var(e) be the set of all variables names
used in e. We say that a REQL query is well-designed 4 if every
subquery e satisfies the following four conditions: (1) if e = !x{e1},
then x ∉ var(e1), (2) if e = e1 e2, then var(e1) ∩ var(e2) = ∅; (3) if
e = e1|e2, then var(e1) = var(e2); and (4) if e is equal to e1*,
e1+, e1? or e1{n,m}, then var(e1) = ∅. One can easily check that
queries !x{a!x{b}}, !x{a}!x{b}, a|!x{b}, or (!x{a}b)* are not
well-designed. Instead, queries like !x{a}!y{b}, !x{a}|!x{b}, or
!x{a}(b)* do satisfy all conditions and then are well-designed.
Note that, as shown in [15], the well-designed condition does not
diminish the query language’s expressive power. Then from now
on, we will consider all the queries we evaluate to be well-designed.

Ve1|e2U𝑑 = Ve1U𝑑 ∪ Ve2U𝑑

Ve*U𝑑 = V𝜀U𝑑 ∪ VeU𝑑 ∪ Ve eU𝑑 ∪ Ve e eU𝑑 ∪ · · ·
Ve+U𝑑 = Ve (e*)U𝑑

Ve?U𝑑 = VeU𝑑 ∪ {([𝑖, 𝑖⟩, ∅) | 0 ≤ 𝑖 ≤ |𝑑 |}

Ve{n,m}U𝑑 = Ve n-times
. . . e (e?) (m−n)-times
...
(e?)U𝑑

Notice that these mappings will not share any variables, given that
the expression is assumed to be well-formed. The second layer, JeK𝑑 ,
then simply gives us the mappings that e defines when matching
the entire document. Note that when e is an ordinary regular expression (i.e., no variables), then the empty mapping is output if the
entire document matches e, and no mapping is output otherwise.
In the following, we provide several examples from English text
analysis to grasp the power of REQL for information extraction and
to see its differences concerning classical RegEx. The reader can
test these examples and other REQL queries in our REmatch beta
demo available on www.rematch.cl.

Semantics. We define the matches extracted by REQL in terms
of mappings. Formally, a mapping for a document 𝑑 is a (partial)
function 𝜇 from variables to spans of 𝑑. Intuitively, a mapping
represents a single match that a REQL query makes on a document
𝑑. For instance, in our previous example, the query e0 will produce
three mappings as its output: 𝜇 1 , with 𝜇 1 (𝑥) = [0, 4⟩, 𝜇 2 , with
𝜇 2 (𝑥) = [3, 7⟩, and 𝜇 3 , with 𝜇3 (𝑥) = [6, 10⟩. We write dom(𝜇)
to denote the domain of 𝜇 and 𝜇1 ∪ 𝜇 2 for the disjoint union of
mappings whenever dom(𝜇1 ) ∩ dom(𝜇 2 ) = ∅. We also use the
notation [𝑥 → 𝑠] to define the singleton mapping that only maps
𝑥 to the span 𝑠 (e.g., 𝜇1 = [𝑥 → [0, 4⟩]), and use ∅ for the trivial
empty mapping (where the domain is the empty set).
With the formalism of mappings, we can give a concise declarative semantics for REQL, similarly as in [31]. This is done in Table 1.
The semantics is defined by structural induction on e and has two
layers. The first layer, VeU𝑑 , defines the set of all pairs (𝑠, 𝜇) with
𝑠 ∈ span(𝑑) and 𝜇 a mapping such that (1) e successfully matches
the substring 𝑑 (𝑠) and (2) 𝜇 results as a consequence of this successful match. For example, the REQL query 𝑎 matches all substrings
of input document 𝑑 equal to 𝑎, but results in only the empty mapping. On the other hand, !x{e} matches all substrings that are
matched by e, but assigns to 𝑥 the non-empty span 𝑠 that delimits
the substring being matched, while preserving the previous variable
assignments. Similarly, in the case of concatenation e1 e2 we join
the mapping defined on the left with the one defined on the right.

Example 2.2. A typical task in language analysis is detecting
words with particular roots, or more precisely lexemes, which are
basic units of meaning. For example, one could be interested in words
in the English language that start with the prefix ‘a’. To extract all such
words from a text, we can simply use the following REQL expression:
e1 = !word{[Aa]\w+}[ .]
where denotes a single white space, and \w denotes the char class
of words characters, as commonly used is Perl-compatible regular
expressions. Note that in [ .] the . denotes the dot symbol and not
a wildcard. This is consistent with the classic RegEx syntax, since a
wildcard symbol is useless for defining a char class.
In e1 we are looking for a word staring with the letter ’a’. To assure
we will capture the entire word, we preceded it by a space, and we
require that after reading it we see either a space or a dot symbol5 . If
we evaluate e1 over document:
𝑑 2 := T h e

ant is an amazing architect.

0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31

which is a sentence from the book “What is a man?” by Mark Twain,
we will get four mappings assigning the variable word to the spans
5 This example is for illustration purposes. The actual expression would allow arbitrary

4 In [15], expressions satisfying these conditions are called functional.

spacing and sentence punctuation, and allow matching the first word in the sentence.

2794

<!-- page 4 -->

[4, 7⟩, [11, 13⟩, [14, 21⟩, and [22, 31⟩, representing the words “ant”,
“an”, “amazing”, and “architect”, respectively.

Why do we need a new query language? The motivation for introducing REQL in presence of existing RegEx frameworks is twofold.
The most apparent reason for having a new language is that the
REQL’s semantics differ from RegEx. Indeed, REQL allows capturing all matches, not only the leftmost-longest matches. This fact
implies that REQL does not need look-around operators, given
that these features are naturally incorporated in its semantics, as
Example 2.2 and 2.3 shows. Moreover, REQL queries can be significantly different than RegEx expressions for the same task, as in
Example 2.2, which is one reason to introduce a new language.
The second and less obvious motivation is that RegEx expressions were originally designed for pattern matching, namely, for
checking the existence of a pattern in a string. Later, people extended RegEx with captures to support the extraction of some
substrings. Instead, we ground REQL on the theoretical framework
of Document Spanners [15], designed for information extraction.
Although RegEx can extract spans, they are somewhat limited in
this regard. For instance, Example 2.4 shows a task that REQL can
naturally do, and that RegEx cannot express.

In classical RegEx, round parentheses denote a capture group for
extracting a substring. That is, (R) will extract what is matched by
the RegEx R. We could therefore try to express e1 from Example 2.2
by the expression: ([Aa]\w+)[ .] which replaces REQL’s capture
variables with a capture group. However, when evaluated over
the document 𝑑 2 , one fewer output will be produced; namely: the
span [14, 21⟩ corresponding to the word “amazing” will be missing.
This is due to leftmost-longest match semantics deployed by classic
RegEx engines, which will consume the white space following
the word “an”, therefore preventing the expression from matching
“amazing”. A typical workaround for this problem is the use of lookahead operators, which allow to check whether a string is present
starting from some position. A RegEx expression equivalent to e1
would then be ([Aa]\w+)(?=[ .]) which upon matching a word
will look-ahead for a space or a dot, without advancing with the
current match. In general, using look-ahead operators is somewhat
cumbersome, and, as we show below, is not sufficient to capture all
the matches in some cases.

3

Example 2.3. Suppose that the user wants to process the English
text into 𝑘-grams (i.e., 𝑘 consecutive words in a text) that satisfy some
particular pattern. Specifically, suppose this user wants to extract all
2-grams where each word begins with the letter ‘a’. We can extract
them by running the following REQL query:
e2 := !w1{[Aa]\w+} !w2{[Aa]\w+}[ .]
Note that e2 is the extension of e1 where now we use two variable
names, called w1 and w2, for obtaining the substrings of the first and
second words, respectively. For instance, if we run e2 over document 𝑑 2
we will get mappings:
[w1 ↦→ [11, 13⟩, w2 ↦→ [14, 21⟩]

REWRITING MODULE

The first step for the evaluation of a REQL query is the compilation
and rewriting into a logical plan, called a logical VA. This plan is
essentially an automaton with variables that is equally expressive
as a REQL query. Furthermore, logical VA is suitable for rewriting.
Specifically, we perform an offset transformation over the logical VA
that keeps the semantics of the query but improves the performance
of the evaluation algorithm. Next we explain these two components.
Logical VA. A logical variable-set automata (logical VA) is a finite
state automaton extended with captures variables. Formally, a logical VA A is a tuple (𝑄, 𝛿, 𝑞 0, 𝑞 𝑓 ), where 𝑄 is a finite set of states, 𝑞 0
and 𝑞 𝑓 are the initial and the final state, and 𝛿 is a transition relation
consisting of letter transitions (𝑞, 𝐶, 𝑞 ′ ), and variable transitions
(𝑞, [𝑥, 𝑞 ′ ) or (𝑞, 𝑥⟩ , 𝑞 ′ ), where 𝑞, 𝑞 ′ ∈ 𝑄, 𝐶 is a char class (e.g, a
letter a, [w] or [^w]) and 𝑥 is a variable. The [𝑥 and 𝑥⟩ are special
symbols to denote the opening or closing of a variable 𝑥. In the
following, we refer to [𝑥 and 𝑥⟩ collectively as variable markers.
A configuration of a logical VA over a document 𝑑 is a tuple
(𝑞, 𝑖) where 𝑞 ∈ 𝑄 is the current state and 𝑖 ∈ [0, |𝑑 |] is the current
position in 𝑑. A run 𝜌 over 𝑑 = 𝑎 0𝑎 1 · · · 𝑎𝑛−1 is a sequence:

[w1 ↦→ [14, 21⟩, w2 ↦→ [22, 31⟩]

representing the 2-grams “an amazing” and “amazing architect”.
Note that the previous query cannot be obtained by any RegEx engine
without “look-arounds”, given that 2-grams can overlap.
Example 2.4. We end by showing another capacity of REQL for
extracting contextual information, another feature not supported by
RegEx. Suppose that, in addition to the 2-grams, the user wants to
extract the sentence where the match happens. This additional information could be useful for understanding the context where these
2-grams are used. For this, we can modify our query e2 as follows:

𝑜𝑚
𝑜1
𝑜2
𝜌 = (𝑞 0, 𝑖 0 ) −→
(𝑞 1, 𝑖 1 ) −→
··· −
→ (𝑞𝑚 , 𝑖𝑚 )

where (𝑞 𝑗 , 𝑜 𝑗+1, 𝑞 𝑗+1 ) ∈ 𝛿 and 𝑖 0, . . . , 𝑖𝑚 is an increasing sequence,
and 𝑖 𝑗+1 = 𝑖 𝑗 + 1 if 𝑜 𝑗+1 is a char class such that 𝑎𝑖 𝑗 ∈ set(𝑜 𝑗+1 )
(i.e. the automata moves one position in the document only when
reading a letter) and 𝑖 𝑗+1 = 𝑖 𝑗 otherwise. Furthermore, 𝜌 must
satisfy that variables are opened and closed in a correct manner,
namely, each 𝑥 is closed at most once and only if it is opened
previously. We say that 𝜌 is accepting if 𝑞𝑚 = 𝑞 𝑓 in which case we
define the mapping 𝜇 𝜌 that maps 𝑥 to [𝑖 𝑗 , 𝑖𝑘 ⟩ if, and only if, 𝑜𝑖 𝑗 = [𝑥
and 𝑜𝑖𝑘 = 𝑥⟩ in 𝜌. Notice that we do not require that 𝑖 0 = 0, nor
𝑖𝑚 = 𝑛; namely, an accepting run can start or end at any position in
the document 𝑑, as long as it consumes a contiguous substring of 𝑑.
Finally, the semantics of A over 𝑑, denoted by JAK𝑑 is defined as
the set of all 𝜇 𝜌 where 𝜌 is an accepting run of A over 𝑑.

e3 := \.!sent{ [^.]*
!w1{[Aa]\w+} !w2{[Aa]\w+}
( [^.]*)?\. }
Here, the new variable sent will store the information containing the
sentence where the 2-gram occurs. The reader can check that if we
evaluate e3 over 𝑑 2 , then we will obtain the mappings of Example 2.3
where each mapping will have in addition the variable sent maps
to [0, 31⟩, which represents the whole sentence. Interestingly, this
semantics context of a match cannot be extracted by RegEx, even if
we use look-ahead operators. The main issue for look-ahead operators
is that due to the leftmost-longest semantics, no two matches starting
at the same position can be returned, which is an issue in our case.

2795

<!-- page 5 -->

Algorithm 1 [Light Search] The segmentation algorithm for a
logical VA A = (𝑄, 𝛿, 𝑞 0, 𝑞 𝑓 ) over a document 𝑑 = 𝑎 0 . . . 𝑎𝑛−1 .

Example 3.1. Consider the REQL query e0 of Example 2.1. The
following is a logica VA representing e0:
[𝑥
0

𝑡
1

ℎ

𝑎
3

2

1: procedure Filtering(A, 𝑎 0 . . . 𝑎𝑛−1 )

𝑥⟩

𝑡
5

4

6
2:

In this figure, the states are {0, . . . , 5}, where 0 and 5 are the initial
and final state, respectively. The edges between states are transitions,
where the first and last edges are variable transitions, i.e., they open
and close 𝑥 with the variable markers [𝑥 and 𝑥⟩, respectively, and the
middle edges are letter transitions.

3:
4:
5:
6:
7:
8:

Example 3.1 shows how to compile a REQL query into a logical
VA. Using a Thomson-liked construction [20], we can covert every
REQL query into a logical VA, giving us a logical plan for the query.

9:
10:
11:

Proposition 3.2. For every REQL query e, one can build in linear
time a logical VA A such that JeK𝑑 = JAK𝑑 for every document 𝑑.

12:
13:

Note that logical VA is an extension of variable-set automata
(VA) from [15]. The main difference between the two models is
that logical VA uses char classes in its letter transitions whereas VA
uses individual letters. Moreover, a logical VA can start a run at any
position, whereas VA starts from the beginning of the document.
Although both models are equally expressive, we use logical VAs
as logical plans for compiling REQL formulas in practice.

shifted by ℓ, namely, for every variable 𝑥, 𝜇 +ℓ (𝑥) = [𝑖 +ℓ, 𝑗 +ℓ⟩ where
𝜇 (𝑥) = [𝑖, 𝑗⟩. A segmentation of 𝑑 is a sequence [𝑖 1, 𝑗1 ⟩, . . . , [𝑖𝑘 , 𝑗𝑘 ⟩
of spans of 𝑑 such that 𝑗ℎ < 𝑖ℎ+1 for every ℎ < 𝑘. We say that the
segmentation is valid for A over 𝑑 iff
𝑘
JAK𝑑 = ∪ℎ=1
{𝜇+𝑖ℎ | 𝜇 ∈ JAK(𝑑 [𝑖ℎ , 𝑗ℎ ⟩)}

Offsets (Optimization). In some cases, opening a variable can be
postponed in order not to store the information about runs that
will not result in an output. To illustrate this, consider again the
expression e0 and its logical VA of Example 3.1. Intuitively, our
algorithm needs to store the position information for the opening
of a variable 𝑥 every time a t would be read. If the document we
are reading has the text thasty, this run would then be extended
for two more steps, although it will eventually be abandoned, and
not result in any outputs. In cases such as these, we can actually
postpone (offset) opening of the variable 𝑥 by transforming the
logical VA as follows: (i) first read the word that; (ii) now open a
variable marker [𝑥, but remember that it was actually opened four
symbols before (i.e. it has an offset 4); (iii) proceed with the current
run. When reconstructing the output, we will start reading four
symbols before the position that is stored for [𝑥. The transformation
of the logical VA from Example 3.1 would look as follows:
𝑡
0

ℎ
1

𝑎
2

[𝑥 −4

𝑡
3

4

namely, if we can evaluate A over 𝑑 by considering the segments
𝑑 [𝑖ℎ , 𝑗ℎ ⟩ of 𝑑 and shifting the results. The task of the filtering module is to search for a good segmentation that is valid for A over 𝑑,
and which can be computed quickly. The segmentation [0, |𝑑 |⟩ is
always valid, and we wish to refine it into smaller segments.
Filtering the document into disjoint segments is not new when
evaluating regular expressions. For example, RE2 [10] runs a deterministic automaton back and forth to find the starting and ending
positions for the leftmost-longest match. For simulating this behavior in REmatch we use the split-correctness framework of [13]. The
main goal of split-correctness is to find a REQL query e A with a
single variable such that Je A K𝑑 is a valid segmentation of A over 𝑑.
The filtering module in REmatch is inspired by split-correctness,
but we improve the approach in two ways. First, REmatch does not
restrict the filtering module to using single-variable expressions.
Second, we look for a “cheap” segmentation algorithm, i.e., a process that runs faster than the evaluation algorithm. Indeed, using
regex for filtering does not payoff if computing the segmentation
takes longer than evaluating the target query itself.

𝑥⟩
5

6

The notation [𝑥 −4 is used in order to signal that the variable 𝑥 was

Light search (Optimization). In REmatch, the filtering module
finds a segmentation by simulating the logical VA over the document, but only storing the starting and ending position where there
is at least one output. For this we need the following extension
of the transition relation. Let A = (𝑄, 𝛿, 𝑞 0, 𝑞 𝑓 ) be a logical VA.
We extend 𝛿 to a function 𝛿 ∗ that given a set 𝑆 ⊆ 𝑄 and a letter
𝑎, outputs all states that can be reached from 𝑆 by using zero or
more variable transitions and then a letter transition that satisfies 𝑎,
namely, 𝑝 ∈ 𝛿 ∗ (𝑆, 𝑎) if there exists a sequence of transitions in 𝛿 of
𝑣𝑚
𝑣1
𝑣2
𝐶
the form 𝑞 −→
𝑞 1 −→
··· −
→ 𝑞𝑚 −→ 𝑝 such that 𝑞 ∈ 𝑆, 𝑎 ∈ set(𝐶),
and 𝑣𝑖 is a variable marker (e.g., [𝑥 or 𝑥⟩) for every 𝑖 ≤ 𝑚. We
also define 𝛿 ∗ (𝑆, 𝜖) such that 𝑝 ∈ 𝛿 ∗ (𝑆, 𝜖) if, and only if, 𝑝 can be
reached from a state in 𝑆, by only using variable transitions.

actually opened four positions before it was recorded in the run.
Offsets are implemented in the rewriting module of REmatch
after constructing the first logical VA from a REQL query. When
the query contains quantifiers or alternations special care must be
taken for offsetting the variables. More details are provided at [2].

4

𝑆←∅
𝑖 ← 0, 𝑗 ← 0
for ℓ = 0 to 𝑛 do
(𝑆, output, ends) ← next𝛿 (𝑆, 𝑎 ℓ )
if output then
𝑗 ←ℓ +1
else if ends then
if 𝑖 < 𝑗 then
Enumerate [𝑖, 𝑗⟩
𝑖 ←ℓ +1
if 𝑖 < 𝑗 then
Enumerate [𝑖, 𝑗⟩

FILTERING MODULE

In this section, we present the module in charge of filtering the input
document and reducing the load of the main algorithm. The plan is
to run a light process that scans the input and quickly finds sections
of the document where there is at least one output. Formally, let
A be the logical VA constructed from a REQL query, and let 𝑑 be a
document. For a mapping 𝜇 and a number ℓ, let 𝜇 +ℓ be a mapping

2796

<!-- page 6 -->

Algorithm 1, also called Light Search, presents the filtering
procedure for finding a segmentation of A over 𝑑. The algorithm
simulates A over 𝑑 by keeping a set of states 𝑆 ⊆ 𝑄 of active runs,
namely, each 𝑞 ∈ 𝑆 represents a run of A over a prefix of 𝑑 that could
produce an output. The workhorse of Algorithm 1 is the function
next𝛿 (see line 5). Given a set 𝑆 ⊆ 𝑄 and a letter 𝑎, the function
next𝛿 (𝑆, 𝑎) returns a triple (𝑆 ′, output, ends) where 𝑆 ′ ⊆ 𝑄 and
output, ends are boolean values. The first component 𝑆 ′ is equal to
𝛿 ∗ (𝑆, 𝑎) ∪ 𝛿 ∗ ({𝑞 0 }, 𝑎). Intuitively, 𝛿 ∗ (𝑆, 𝑎) are all states that one can
reach from 𝑆 by using some variable transitions and reading 𝑎. On
the other hand, 𝛿 ∗ ({𝑞 0 }, 𝑎) are the new states that one can reach by
starting from 𝑞 0 and by reading 𝑎. Recall that a match can be made
from any position in the document, so we start a fresh run by using
𝛿 ∗ ({𝑞 0 }, 𝑎). The second component output is true iff 𝑞 𝑓 ∈ 𝛿 ∗ (𝑆 ′, 𝜖),
namely, output is true when there is a run that reaches the final
state. Finally, ends is true iff 𝛿 ∗ (𝑆, 𝑎) = ∅, which tells whether the
runs in 𝑆 ends with the new letter. When implementing Algorithm 1
in REmatch, we cache the output next𝛿 (𝑆, 𝑎), in order to compute
it at most once for every pair (𝑆, 𝑎).
We have all the ingredients to explain how Algorithm 1 works.
The algorithm keeps a set of active states 𝑆, and two pointers 𝑖 and 𝑗.
As we already mentioned, 𝑆 contains all active states when reading
the document. Instead, the pair (𝑖, 𝑗) stores the last span [𝑖, 𝑗⟩ (called
active span) where there is an output, namely, JAK(𝑑 [𝑖, 𝑗⟩) ≠ ∅.
We assume here that, if 𝑗 ≤ 𝑖, then the algorithm has not found
a segment yet (i.e., from the previous segment that was output).
In lines 2-3, we start by setting 𝑆 = ∅ (i.e., no active runs) and
(𝑖, 𝑗) = (0, 0) (i.e., no active span). Then we iterate sequentially over
each letter 𝑎 ℓ . For each letter, we compute next𝛿 (𝑆, 𝑎 ℓ ) returning
as output the triple (𝑆 ′, output, ends) where 𝑆 ′ is the new set of
active states (line 5). If output is true, an active state can reach
𝑞 𝑓 and the segment [𝑖, ℓ + 1⟩ contains an output. Then by setting
𝑗 = ℓ + 1 we update the new active span (lines 6-7). Instead, if there
is no output and ends is true, then all active states of the previous
iteration end with the new letter 𝑎 ℓ , and we can start a new active
span by setting 𝑖 = ℓ (line 11). However, if 𝑖 < 𝑗, then we cannot
extend more the active span represented by (𝑖, 𝑗), we can safely
return the active span [𝑖, 𝑗⟩ and continue (lines 9-10). Finally, after
the document ends (lines 12-13) we check if there is an active span
that was not output (i.e., 𝑖 < 𝑗), and return it if this is the case.

Algorithm 1 maintains the invariant that, after reading 𝑎 ℓ , 𝑖 is a
position before any of the current active runs started and, if 𝑖 < 𝑗,
then 𝑗 is the latest position such that JAK(𝑑 [𝑖, 𝑗⟩) ≠ ∅. Indeed,
these invariant is enough to prove that the algorithm is correct.
Theorem 4.2. Given a logical VA A and a document 𝑑, Algorithm 1 outputs a sequence of spans [𝑖 1, 𝑗1 ⟩, . . . , [𝑖𝑘 , 𝑗𝑘 ⟩ that is a valid
segmentation for A over 𝑑.
Note that the load of computing Algorithm 1 is low: we need
one pass over the document and for each letter we need to perform
a small number of simple operations (i.e., apply the function next𝛿
and check at most two if-statements). Given that we can cache the
output next𝛿 (𝑆, 𝑎) for each new pair (𝑆, 𝑎), the cost per new letter
is low when the caching of the function next𝛿 stabilizes. This is
probably the main reason why filtering runs faster than performing
the main evaluation algorithm (see Section 7 for further discussion).

5

The output of a REQL query e over a document 𝑑 can be prohibitively large, namely, of size O (|𝑑 | 2|𝑒 | ), since for the all-match
semantics we could even ask for all substrings being matched to all
the variables. Since such queries are, at least in principle, expressible in REQL, we need to be able to handle them in REmatch. For
this, we deploy the notion of enumeration algorithms and output
delay, which measure the efficiency of an algorithm with respect
to both its input and its output.
More formally, we use the framework of enumeration algorithms,
which received a lot of attention in the database community [4, 6, 16,
23, 24, 29, 39, 40, 46]. In enumeration algorithms, we are required
to produce the (finite) output set 𝑂 = {𝑜 1, . . . , 𝑜𝑘 }, in any order,
and without repetitions. Such algorithms operate in two phases:
(1) The pre-processing phase builds a data structure which
allows enumerating the results;
(2) The enumeration phase retrieves the outputs from the data
structure.
In the case of REQL queries, the desired result is the set of all the
output mappings. We will say that an enumeration algorithm works
with output-linear delay, if the time to print out the 𝑖-th output 𝑜𝑖 ,
measured as the time needed from printing the (𝑖 − 1)-st output
𝑜𝑖 −1 to finishing the output 𝑜𝑖 , is proportional to the length of 𝑜𝑖 ,
and independent of the size of the document, the query, or the size
of the output set 𝑂. The algorithm is also required to terminate
immediately after outputting the final output. If these conditions are
met, the time needed to enumerate 𝑂 is O (|𝑂 |), hence output-linear.
Next we describe the REmatch module for managing the system
memory and the data structure supporting output-linear delay.

Example 4.1. In the figure below, we display the execution of
Algorithm 1 for the logical VA of query e0 (see Example 3.1) over the
document thathatsthat. For each letter, we show below the value
of variables ℓ, 𝑆, output, ends, 𝑖, and 𝑗 after finishing each iteration.

t h a t h a t s t h a t
ℓ=

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

10

OUTPUT MODULE

11

The data structure. In general, the pre-processing phase builds a
data structure encoding all the mappings that are to be enumerated.
We next explain which operations this structure needs to support
in order to encode outputs of a REQL query. For this, consider again
the document 𝑑 1 from Section 2 and the REQL query:

𝑆 = ∅ {2} {3} {4} {2,5} {3} {4} {2, 5} ∅ {2} {3} {4} {2, 5}
output =
F
F
F
T
F
F
T
F
F
F
F
T
ends =
T
F
F
F
F
F
F
T
T
F
F
F
𝑖= 0
7
0
0
0
0
0
0
0
8
8
8
8
𝑗= 0
7
7
7
7
7 12
0
0
0
4
4
4

e4 := !x{th}.*!y{hat}
By following the run of the algorithm, we can check that it outputs
the segmentation [0, 7⟩ and [8, 12⟩ corresponding to the substrings
thathat and that, respectively.

that extracts the substring th in the variable 𝑥 followed by the
substring hat in the variable 𝑦. One output here is 𝜇 1 , with 𝜇 1 (𝑥) =
[0, 2⟩, and 𝜇 1 (𝑦) = [4, 7⟩. Notice that for each output mapping, we

2797

<!-- page 7 -->

need to define when a variable is opened, and when it is closed, in
order to define the span it captures. In REmatch we will represent
a mapping as an output sequence of pairs (𝑆, 𝑖), where 𝑆 is a set of
variable markers (e.g., 𝑆 = {𝑥⟩, [𝑦}), denoting when a span associated with the variable 𝑥 starts, or finishes, respectively. Therefore,
the mapping 𝜇 1 is represented by the output sequence:

[𝑥, 0

[𝑦, 4

𝑥⟩, 2

𝑦⟩, 7

⊥

∪
[𝑥, 3

𝑥⟩, 5

∪

[𝑦, 7

𝑦⟩, 10

Figure 1: The ECS representing three outputs.

( [𝑥, 0), (𝑥⟩, 2), ([𝑦, 4), (𝑦⟩, 7).6
In essence, REmatch will create a data structure encoding this
information for each mapping. Since many mappings will share
information (for instance, 𝜇 2 , with 𝜇 2 (𝑥) = [0, 2⟩, and 𝜇 2 (𝑦) = [7, 9⟩
has the 𝑥-part identical to 𝜇 1 ), we can exploit this fact to create a
succinct representation of all the outputs.
The data structure we will use to represent the set of all output
sequences in JeK𝑑 , for a REQL query e and a document 𝑑 is called
enumerable compact set, or ECS for short, and was first introduced
in [5, 32]. The ECS data structure can be thought as a directed
acyclic graph (DAG) with three types of nodes n:
(i) a (unique) terminal node (denoted emptyNode or ⊥ for
short), with no children that signals the end of output;
(ii) content nodes, which store a pair (𝑆, 𝑖) from an output
sequence and have a single child n′ ; and
(iii) union nodes, which have two children n1 and n2 .
More importantly, every node n defines a set of mappings JnK, represented as a set of output sequences. Specifically, the emptyNode
represents the set JemptyNodeK = {𝜖}, the output node n with the
child n′ represents the set Jn′ K · {(𝑆, 𝑖)}, and the union node with
children n1 and n2 the set Jn1 K ∪ Jn2 K.

anymore by the evaluation algorithm, detecting that it will not be
reached from any newly added node, thus allowing to delete the
node and other nodes whose pointer count hits zero by this removal.
As stated previously, the memory locations of deleted nodes are not
liberated but are instead overwritten by newly created nodes and
returned to the memory pool. We use the command NM.discard(n)
when we wish to signal that a node is to be discarded.
The node manager NM is also in charge of implementing the
following three operations over nodes:
• NM.emptyNode, which creates the terminal node;
• NM.extend(n, (𝑆, 𝑖)), which creates a content node which
contains (𝑆, 𝑖), and links this newly created node to n; and
• NM.union(n1, n2 ), which takes nodes n1 and n2 , and creates
a new union node, representing the union of both outputs.
NM implements these three operations, taking constant time for
each operation. Moreover, NM module supports enumerating outputs
from any given node n, which we denote by NM.enumerate(n). More
importantly, this enumeration takes output-linear delay, and one
can do it at any point without further preprocessing. This last fact is
the guarantee for REmatch to retrieve all outputs JeK𝑑 with outputlinear delay. We refer to [32] for the implementation details of such
operations and enumeration procedure.

Example 5.1. Consider again the query e4, and document 𝑑 1 from
Section 2. The output sequences of mappings in Je4K𝑑 are then:
𝜇 1 : ([𝑥, 0), (𝑥⟩, 2), ([𝑦, 4), (𝑦⟩, 7)

Early output (Optimization). Note again that NM allows enumerating the outputs from a node at any point without further preprocessing. This fact is crucial for the optimization that we call early
output enumeration. Specifically, when evaluating a logical VA over
a document, we can often detect whether we reach a final state
before the entire document is read completely (for details, see Section 6). In essence, this means that we can provide certain outputs
to the user at this point before continuing to construct the ECS in
its entirety. The benefits are twofold: (i) the outputs are delivered
to the user as soon as possible, similarly as pipelined evaluation
is done in databases; and (ii) once we enumerate these outputs,
we can delete the unused nodes, thus saving storage space. This
optimization is highly effective in decreasing the memory usage
and decreasing the time to deliver the first output (see Section 7).

𝜇 2 : ([𝑥, 0), (𝑥⟩, 2), ([𝑦, 7), (𝑦⟩, 10)
𝜇 3 : ([𝑥, 3), (𝑥⟩, 5), ([𝑦, 7), (𝑦⟩, 10)
The ECS for this set of output sequences is given in Figure 1. The
rightmost union node represents 𝜇 1 , 𝜇 2 , and 𝜇3 . Following the paths
from this node to the ⊥ node represent the three output sequences. The
shared output for the variable 𝑦 in 𝜇2 and 𝜇 3 is represented only once.
Node manager (Optimization). The key step when evaluating a
REQL query over a document is building the ECS which encodes
all the outputs. Naively building the ECS would require allocating
the memory for each node being added, which is highly inefficient.
Therefore, REmatch includes a node manager module, denoted NM,
which allocates memory in bulk, and acts as a garbage collector
for the ECS. The NM module essentially creates a memory pool for
storing the nodes in the ECS. Each time NM fills the pool, it allocates
the memory for another pool double the original size. This strategy
allows memory allocation to occur infrequently and in big chunks,
thus preventing fragmentation and multiple pointer dereferencing. In addition, the NM module acts as a lazy garbage collector by
keeping a pointer count for each node in the memory pool. Once
the pointer counter hits zero, it moves the node to a pool of nodes
that can be deleted. This will happen when a node is not required

6

EVALUATION MODULE

To evaluate REQL, we compile logical VA into so-called extended
VA, a “physical” automata model closer to the evaluation algorithm
than to the query language. We use this model for the REQL evaluation algorithm used in REmatch, which makes one pass over
the document and produces the resulting mappings with outputlinear delay. This evaluation algorithm refines and improves the
theoretical algorithm presented in [16] for evaluating extended VA
by taking care of the memory usage, simplifying the algorithm, and
applying various optimizations presented in the previous sections.

6 Strictly speaking, we should write ( { [𝑥 }, 0), ( {𝑥 ⟩ }, 2), . . .. For the sake of simplifi-

cation, we omit the set brackets whenever it is possible.

2798

<!-- page 8 -->

Extended VA. An extended variable-set automaton (eVA) [15] is
a finite-state automaton extended with capture variables in a way
analogous to logical VA. The difference is that reading and outputting variable markers can be done in a single transition7 Formally, an eVA E is a tuple (𝑄, 𝑞 0, 𝐹, 𝛿), where 𝑄 is a finite set of
states; 𝑞 0 ∈ 𝑄 is the initial state; 𝐹 ⊆ 𝑄 is the set of final states;
and 𝛿 is a transition relation consisting of transitions of the form
(𝑞, 𝑎, 𝑆, 𝑞 ′ ) where 𝑞, 𝑞 ′ ∈ 𝑄, 𝑎 ∈ Σ ∪ {■}, and 𝑆 is a set of variable
markers (e.g., 𝑆 = {[𝑥, 𝑦⟩}). The meaning behind the transition
(𝑞, 𝑎, 𝑆, 𝑞 ′ ) is that when the automaton is in the state 𝑞 and reads
the 𝑖-th letter 𝑎𝑖 = 𝑎, then it switches to state 𝑞 ′ and outputs (𝑆, 𝑖),
where 𝑆 is the set of variables that are opened and closed before
reading the 𝑖-th letter. In particular, when 𝑆 = ∅, the automaton
changes the state and nothing is output.
The fact that a transition can open or close multiple variables at
the same time allows us to handle nested variables (e.g. !x{!y{a}})
in a single automaton transition. Furthermore, the ■ symbol is used
as a special input to denote the end of a document (i.e., an End Of
File, or EOF). This will be useful in the run of an eVA for outputting
spans [𝑖, 𝑗⟩ over a document of size 𝑛, where 𝑖 and 𝑗 range from 0
to 𝑛 (i.e., we need 𝑛 + 1 possible positions).
Next we define how an eVA produces outputs. A run 𝜌 of E over
a document 𝑑 = 𝑎 0 · · · 𝑎𝑛−1 is a sequence of the form:
/𝑆𝑛
0 /𝑆 0
1 /𝑆 1
2 /𝑆 2
𝜌 = 𝑞 0 𝑏−−→
𝑞 1 𝑏−−→
𝑞 2 𝑏−−→
. . . 𝑞𝑛 𝑏−𝑛−→
𝑞𝑛+1

needs to consume the entire document, we add self-loops in the initial
𝑡 /𝑥 ⟩
and the final state; for instance 4−
−→5. Also note that the transition
∗/∅
0−−→0 closes the variable x before the character (EOF or any other
character) is consumed.
Determinization. There is a critical issue with using eVA as our
guide for evaluating REQL: several runs can produce the same
output sequence. To illustrate this issue in its simplest form, assume
the following extended VA E0′ , which is a mild modification of E0 :
𝑎/∅
𝑡/{[𝑥 }
0

(1)

ℎ/∅

𝑎/∅
2

∗/{𝑥⟩}

𝑡/∅
3

4

4
3′

5

𝑡/∅

One can check that E and E det are equivalent (i.e., they define the
same output for every document), and that E det is deterministic.
An inconvenience of E det is that its size is exponential in |E |.
Fortunately, for the evaluation algorithm it is not necessary to
construct entire E det . Instead, we can start from the initial set {𝑞 0 }
and traverse only the transitions and states needed by the next
letter. Each time that we need a new state or transitions of E det , we
use E to build them and cache it in main memory for future access.
This is the purpose of the determinization module of REmatch,
called DET. Specifically, DET has a method next such that, given
a state 𝑋 ∈ 𝑄 det and a letter 𝑎, DET.next(𝑋, 𝑎) computes a list ℓ
with all pairs (𝑆, 𝑋 ′ ) such that (𝑋, 𝑎, 𝑆, 𝑋 ′ ) ∈ 𝛿 det . Instead, if ℓ was
already computed before (and cached), DET.next(𝑋, 𝑎) outputs ℓ
immediately. Then by using DET we only need to compute each
state and transition once. Moreover, the number of states accessed
by DET depends on E and the input document 𝑑. In practice, this
size is small and at most three or four times the size of E.

∗/∅
1

∗/{𝑥⟩}

2

𝛿 det = {(𝑋, 𝑎, 𝑆, 𝑋 ′ ) | ∀𝑞 ′ ∈ 𝑋 ′ . ∃𝑞 ∈ 𝑋 . (𝑞, 𝑎, 𝑆, 𝑞 ′ ) ∈ 𝛿 }.

Example 6.2. To illustrate Proposition 6.1, recall the logical VA
A0 from Example 3.1. The following eVA E0 is equivalent to A0 :

𝑡/{[𝑥 }

3

𝑡/∅

One can check that E0′ is equivalent to E0 , namely, JE0′ Kseq (𝑑) =
JE0 Kseq (𝑑) for every document 𝑑. However, for every output sequence of JE0′ Kseq (𝑑), two different runs are witnessing it: one
crossing the state 3 and another crossing the state 3′ . Then, if we
guide an evaluation algorithm of eVA with runs, for E0′ we will
enumerate each output sequence twice, although the user expects
to extract each output without duplicates.
To remove duplicate runs from eVA, we use a subclass called deterministic eVA. We say that an eVA E is deterministic if its transition
relation 𝛿 satisfies that for every two transitions (𝑞, 𝑎 1, 𝑆 1, 𝑞 1′ ) ∈ 𝛿
and (𝑞, 𝑎 2, 𝑆 2, 𝑞 2′ ) ∈ 𝛿, if (𝑎 1, 𝑆 1 ) = (𝑎 2, 𝑆 2 ), then 𝑞 1′ = 𝑞 2′ . In other
words, given a state 𝑞, the next state is determined by the pair (𝑎, 𝑆).
The reader can check that E0 is deterministic but E0′ is not.
A deterministic eVA ensures that for every document and output
sequence, there is at most one accepting run. This correspondence
is crucial for our evaluation algorithm, given that we can simulate
runs and construct the output, without worrying about duplicates.
Fortunately, we can always “determinize” a non-deterministic eVA
E = (𝑄, 𝑞 0, 𝐹, 𝛿) by using a subset construction, similar to the
standard determinization procedure for NFAs. More precisely, we
define E det = (𝑄 det, 𝑞 0det, 𝐹 det, 𝛿 det ) such that: 𝑄 det = 2𝑄 , 𝑞 0det =
{𝑞 0 }, 𝐹 det = {𝑋 | 𝑋 ∩ 𝐹 ≠ ∅}, and:

Proposition 6.1. For every logical VA A compiled from a REQL
query e, one can build, in linear time, an eVA E such that E evaluated over a document 𝑑 produces precisely the output sequences
representing the mappings in JAK𝑑 = JeK𝑑 .

0

ℎ/∅
1

𝑎/∅

where 𝑏 0𝑏 1 . . . 𝑏𝑛 = 𝑎 0 . . . 𝑎𝑛−1 ■, 𝑆 𝑗 is a set of variable markers, and
(𝑞 𝑗 , 𝑏 𝑗 , 𝑆 𝑗 , 𝑞 𝑗+1 ) ∈ 𝛿, for 0 ≤ 𝑗 ≤ 𝑛. Also, we say that 𝜌 is accepting
if 𝑞𝑛+1 ∈ 𝐹 . A run 𝜌 like (1) naturally defines an output sequence as
out(𝜌) = out(𝑆 0, 0) · . . . · out(𝑆𝑛 , 𝑛) such that out(𝑆𝑖 , 𝑖) = (𝑆𝑖 , 𝑖) if
𝑆𝑖 ≠ ∅, and 𝜖, otherwise. Finally, the semantics of E over 𝑑, denoted
seq
by JEK𝑑 , is defined as the set of all output sequences out(𝜌) where
𝜌 is an accepting run of E over 𝑑.
eVA has several differences compared to logical VA: (1) an eVA
starts from the beginning of the document, (2) produces output
sequences instead of mappings, and (3) the transitions are fired
by symbols (i.e., not by char classes). These relaxations plus readcaptures transitions will considerably simplify the evaluation algorithm. Indeed, we can show that if we start from a logical VA A,
then we can construct an equivalent eVA E in linear time.

∗/∅

∗/∅

∗/∅

5

Here we draw an edge 𝑞 −−→ 𝑞 ′ to denote a transition from 𝑞 to 𝑞 ′
that output 𝑆 when reading a letter 𝑎, and use ∗ to denote any letter.
Intuitively, we have moved transitions with variable markers “forward”
[𝑥 𝑡
if we compare E0 with A0 . For instance, A0 had transitions 0−→
1−→2,
𝑡 /[𝑥
but instead E0 has a direct transition 0−−→1. Since an extended VA
𝑎/𝑆

7 The name eVA is also used by [16] for an automaton model outputting sets of markers.

2799

<!-- page 9 -->

Algorithm 2 Evaluation of an extended variable-set automaton E = (𝑄, 𝛿, 𝑞 0, 𝐹 ) over the document 𝑏 0 . . . 𝑏𝑛 where 𝑏 0 . . . 𝑏𝑛−1 is the original
document and 𝑏𝑛 = ■ is an EOF symbol. DET and NM are the determinization module and node manager, respectively.
1: procedure Evaluate(E, 𝑏 0 . . . 𝑏𝑛 )
2:
3:
4:
5:
6:
7:
8:
9:
10:
11:
12:
13:

DET.initialize(E)
InitializeLists
for 𝑖 = 0 to 𝑛 do
for all 𝑋 ∈ setslist do
ℓ ← DET.next(𝑋, 𝑏𝑖 )
if ℓ ≠ empty then
UpdateSets(𝑋, ℓ, 𝑖)
else
NM.garbage(𝑋 .n)
setslist.swap(setslist′ )
setslist′ .clear
Enumerate

14: procedure InitializeLists
15:
16:
17:
18:
19:
20:

setslist.clear
setslist′ .clear
𝑋 0 ← DET.initialStateSet
𝑋 0 .phase ← −1
𝑋 0 .n ← NM.emptyNode
setslist.add(𝑋 0 )

22: procedure Enumerate
24:
25:

28:
29:
30:
31:
32:
33:

21:

23:

26: procedure UpdateSets(𝑋, ℓ, 𝑖)
27:
for all (𝑆, 𝑋 ′ ) ∈ ℓ do

for all 𝑋 ∈ setslist do
if 𝑋 .isFinal then
NM.enumerate(𝑋 .n)

34:
35:
36:

n′ ← 𝑋 .n
if 𝑆 ≠ ∅ then
n′ ← NM.extend(n′, (𝑆, 𝑖))
if 𝑋 ′ .phase < 𝑖 then
𝑋 ′ .phase ← 𝑖
setslist′ .add(𝑋 ′ )
𝑋 ′ .n ← n′
else
𝑋 ′ .n ← NM.union(𝑋 ′ .n, n′ )

provides a method DET.initialStateSet that outputs 𝑋 0 = {𝑞 0 }
(line 17). Then we initiate 𝑋 0 .phase with −1 (line 18) and fill 𝑋 0 .n
with the empty output node (line 19). Intuitively, no iterations have
accessed 𝑋 0 yet, and the E det -run at 𝑋 0 only has the empty output.
The evaluation algorithm processes the document 𝑏 0 . . . 𝑏𝑛 symbol by symbol, from 𝑖 = 0 to 𝑛 (line 4). During the 𝑖-th iteration, the
setslist keeps all state-sets 𝑋 that can be reached by runs reading
𝑏 0 . . . 𝑏𝑖 −1 , and 𝑋 .n a node storing all outputs of these runs. Instead,
setslist′ will contain the new state-sets after reading the letter 𝑏𝑖 .
Intuitively, to build setslist′ from setslist we fire all state-sets 𝑋 in
setslist one-by-one (line 5) by calling the method DET.next(𝑋, 𝑏𝑖 )
with symbol 𝑏𝑖 (line 6). This method gives a list ℓ of pairs (𝑆, 𝑋 ′ ),
where there is a transition (𝑋, 𝑏𝑖 , 𝑆, 𝑋 ′ ) ∈ 𝛿 det . Then, if (𝑆, 𝑋 ′ ) ∈ ℓ,
we must extend the output sequences in 𝑋 .n with (𝑆, 𝑖), and store
them in 𝑋 ′ . We do this updating by calling UpdateSets(𝑋, ℓ, 𝑖)
(line 8), discussed below. If the list ℓ is empty (e.g., DET detects that
by reading 𝑏𝑖 from 𝑋 , there is no way to continue), then we call the
node manager NM and mark 𝑋 .n for the garbage collection (line 10).
Finally, when we end firing all state-sets in setslist, we swap the
two lists and clean setslist′ to start the iteration again (lines 11-12).

Algorithm’s variables. In Algorithm 2 we present the main algorithm for REmatch. This algorithm evaluates an eVA E over the
input document 𝑑, enumerating all output sequences JEKseq (𝑑).
Two main components used by the algorithm are the node manager
NM, introduced in Section 5, and the determinization module DET,
introduced above. In addition, we use states-sets constructed by DET,
set-lists that store states-sets, and nodes created and operated by NM.
We introduced nodes n in Section 5.
The states-sets, denoted by 𝑋 in the algorithm, are built and
cached by the determinization module for representing a set 𝑋 ⊆ 𝑄.
Each states-set 𝑋 has two variables: 𝑋 .n and 𝑋 .phase. The former
can store a node that represents the current outputs of runs that
reached 𝑋 . Instead, the latter is an integer that encodes the current
phase number: if 𝑋 .phase = 𝑖 then the 𝑖-th iteration was the last
one that reached 𝑋 . In practice, phase will help to know whether it
is the first time we reached 𝑋 during some iteration. Further, each
states-set has a method 𝑋 .isFinal that outputs TRUE if, and only
if, 𝑋 is a final set (i.e., 𝑋 ∩ 𝐹 ≠ ∅). In the implementation, this is a
flag that the determinization module sets when creating 𝑋 .
We will also use set-lists, denoted by setslist in the algorithm,
which are linked-list of states-sets. For this data structure, we assume a method setslist.clear to empty the list, setslist.add(𝑋 ) to
add 𝑋 at the end, and setslist.swap(setslist′ ) to swap the content
between two lists. To iterate over each element 𝑋 in the list, we
conveniently write “for all 𝑋 ∈ setslist”. We assume any straightforward implementation of set-list that takes constant time for each
call to these methods. Finally, during the algorithm, we use two
set-lists variables, setslist, and setslist′ . We assume that setslist,
setslist′ , NM, and DET can be globally accessed by all methods.

Update method. The workhorse of the evaluation algorithm is
UpdateSets(𝑋, ℓ, 𝑖), which is in charge of updating each state-set
𝑋 ′ by extending the outputs in 𝑋 .n with (𝑆, 𝑖), for each (𝑆, 𝑋 ′ ) ∈ ℓ.
For this purpose, we iterate over each (𝑆, 𝑋 ′ ) ∈ ℓ, create a copy n′
of 𝑋 .n, and extend all output sequences in n′ with (𝑆, 𝑖) if 𝑆 ≠ ∅
(lines 28-30). Next, we need to update 𝑋 ′ .n depending on whether
it is the first time or not that we reach 𝑋 ′ . For this, we use variable
𝑋 ′ .phase: if 𝑋 ′ .phase < 𝑖, then we update 𝑋 ′ .phase to 𝑖, add 𝑋 ′ to
setslist′ , and set 𝑋 ′ .n equal to n′ (lines 31-34). Otherwise, it is not
the first time that we reach 𝑋 ′ , and we need to union the output
sequences in 𝑋 ′ .n with the ones in n′ (lines 35-36).
For the correctness of UpdateSets, we cannot reach any stateset 𝑋 in two consecutive iterations (i.e., both in the (𝑖 − 1)- and 𝑖-th
iteration for some 𝑖). Otherwise, we could use 𝑋 .n by reading and
updating it simultaneously, possibly erasing its content. For this
reason, the rewriting module duplicates the logical VA, alternating
between even and odd positions. This construction ensures that we
cannot reach any state-set 𝑋 in two consecutive iterations.

Main algorithm. The main method of Algorithm 2 is Evaluate,
which receives as input an eVA E = (𝑄, 𝛿, 𝑞 0, 𝐹 ) and the document
𝑏 0 . . . 𝑏𝑛 . Recall that 𝑏 0 . . . 𝑏𝑛−1 is the original document and 𝑏𝑛 = ■
is the EOF symbol. The algorithm starts by initializing the determinization module DET with E at line 2. The initialize method
is for storing E inside DET and using it later during the determinization. Then we do the initialization of setslist and setslist′ by calling
InitializeLists (line 3). This method clears both lists (lines 15-16)
and adds the state-set 𝑋 0 to setslist (line 20). This state-set represents {𝑞 0 }, namely, the determinization initial set. For this, DET

2800

<!-- page 10 -->

t h a t h a t h a t ■

𝑛8 :

𝑛4 :

[𝑥,9
𝑛0 :

[𝑥,6
𝑛2 :

⊥

[𝑥,3

𝑛7 :

𝑥⟩,10
𝑛5 :

𝑛9 :

∪

𝑥⟩,7
𝑛6 :
𝑛3 :

𝑛 1 : [𝑥,0

𝑥⟩,4

∪

*Offsets
Rewriting
Module

lVA

*Next index
DET
Module

lVA

0

Enumerable compact set

0

REQL

setslists

0𝑛 1𝑛 2𝑛 3𝑛 4𝑛 5𝑛 5𝑛 5𝑛 5𝑛 5𝑛 5𝑛 5𝑛
3
1
1
0
3
3
3
6
6
6
9
1
0𝑛 0𝑛 0𝑛 1𝑛 2𝑛 3𝑛 4𝑛 2𝑛 3𝑛 4𝑛 0𝑛
5
7
2
2
4
0
0
0
0
2
4
0𝑛 0𝑛 0𝑛 1𝑛 0𝑛 0𝑛 1𝑛
8
4
0
0
0
0
0
0𝑛
0𝑛

doc

eVA

*Node manager
Filtering segm Evaluation nodes Output
Module
Module
Module
*Light search
*Early output
Output (mappings)

Figure 3: REmatch’s architecture. Optimizations are in grey.

Figure 2: An example of how Algorithm 2 works.

an early output enumeration whenever we reach a final state at the
𝑖-th iteration. For ease of presentation, we code Algorithm 2 with
the enumeration procedure after reading the whole document.

UpdateSets passes and updates the outputs from the (𝑖 − 1)- to
the 𝑖-th layer, which is the heaviest part of the algorithm. Therefore,
it is crucial to perform UpdateSets as efficiently as possible. For
this, we use linked-lists and phase variables, which allows us to
check in a single instruction whether 𝑋 ′ is already in setslist′ or not.
Our approach here is similar as in [10], but computes all matches.

7

EXPERIMENTAL EVALUATION

In this section we provide an experimental evaluation showing
the viability of REmatch in practice. For this we designed our
experiments around several real-world text corpora, and:
(1) Set internal baselines by showing how various optimizations described throughout this paper affect the performance of REmatch (Subsection 7.2);
(2) We compare the performance of REmatch against established RegEx engines (Subsection 7.3).

Example 6.3. Figure 2 displays a graphic of how Algorithm 2
would run with eVA A0 from Example 6.2 and document 𝑑 0 . Below
document 𝑑 0 , we draw as columns the setslists that are computed after
reading the corresponding symbol. For this example, each state-set
is a single state, and then each number in a setslist corresponds to a
state in A0 . Below each state-set 𝑋 , we draw in grey the node 𝑋 .n
from the ECS that the algorithm construct.

Data, queries and the REmatch source code, can be obtained at [2].

7.1

Next index (Optimization). A significant step for the evaluation
algorithm is the call to the DET.next function (line 6). Given a
state-set 𝑋 and a symbol 𝑏𝑖 , the first time that the algorithm calls
DET.next(𝑋, 𝑏𝑖 ) the DET module must compute a list with all pairs
(𝑆, 𝑋 ′ ) such that (𝑋, 𝑎, 𝑆, 𝑋 ′ ) ∈ 𝛿 det , and save it in its cache. Then,
for later calls to DET.next(𝑋, 𝑏𝑖 ), the DET module must quickly find
this list in the cache. The evaluation uses the DET’s cache multiple
times, making it one of the heaviest parts of the computation. To
decrease the load of the algorithm, we add an index to each state
set 𝑋 , which quickly allows finding the next state-set given a 𝑏𝑖 .
Currently, REmatch only supports ASCII documents; therefore, we
implement this index as an array with 128 entries. This array on
each state-set allows us to quickly find the next state-set, considerably improving the performance of the evaluation algorithm. Here,
for the next index, we are sacrificing space versus time. Of course,
a more compact next index (e.g., for non-ASCII documents) could
save space during the evaluation. We leave this for future work.

Experimental setup

The implementation. REmatch was implemented in C++, and
includes all the components presented in the paper. An overview of
the REmatch architecture can be found in Figure 3. Each module was
described in a section of the paper. The Rewriting module (Section 3)
takes a REQL expression and converts it to a logical VA (lVA), which
is used both by the DET module (Section 6), and by the Filtering
module (Section 4). The latter processes the input document by
splitting it into segments containing outputs, which are then passed
to the Evaluation module (Section 6). The Evaluation module runs
Algorithm 2 by communicating with the DET module to obtain
the next state of the eVA, and with the Output module (Section 5),
which creates the nodes of the data structure encoding the output
mappings. Specific optimizations were highlighted in each section.
The datasets. We use the following three real-world text corpora:
(1) Literature. This is a combined corpus of collected works
by English literature greats: Mark Twain, William Shakespeare, and Charles Dickens. We used texts provided by the
Project Gutenberg [1], and concatenated them into a single
document of size 50.7 MB and around 50 million characters.
(2) DNA. This dataset consists of DNA sequences. In particular,
we used the list of proteomes of the zebrafish organism, as
provided by the BLAST initiative [8]. The combined size of
this dataset is 38.5 MB and 38.5 million characters.

Enumeration. When we get to the end of the document, setslist
contains all state-set 𝑋 that can be reached by runs of E det when
reading 𝑏 0 . . . 𝑏𝑛 . In particular, 𝑋 .n has the node representing all
outputs of these runs. Then we call Enumerate (line 13), which
iterates over all 𝑋 ∈ setslist that are final, and enumerates the output sequences in 𝑋 .n by calling the enumerate method of the node
manager (lines 23-25). As described in Section 5, we can perform

2801

<!-- page 11 -->

(3) SPARQL. Our final dataset consists of public query logs of
the (now defunct) British Museum SPARQL endpoint, as
collected by the Linked SPARQL Queries Dataset team [43].
We merged all the logs into a single document weighing
71.1 MB, and consisting of roughly 76 million characters.

(1) Naive
(2) Node Manager
(3) Next Index
DNA

Literature

8
7
6

(1) Literature. Here we take a list of common English language
morphemes8 , as provided by [30]. We then specify queries
that look for 2-grams [30], that is, two consecutive words
each containing a morpheme from our list (e.g. the first
word ends in -ing, and the second one in -er).
(2) DNA. Motif detection is a key task in DNA analysis [3].
We select 100 DNA motifs from the Prosite database [19]
that commonly occur in our dataset. Our queries take any
such pair of motifs, and look for their occurrences in the
proteomic sequence separated by at most 20 characters.
(3) SPARQL. Our logs have one query per line. For our expressions we fix two sets of up to three SPARQL keywords [47]
(e.g., WHERE or OPTIONAL), and extract two consecutive queries where the first one contains the keywords from the
first set, and the second one from the second set.

5

Time (s)

The queries. Our queries for each dataset are designed as follows:

17.5
15.0

4

12.5

SPARQL

10.0

3

4

(6) Light Search
(7) All optimizations

5

7.5

3

2

5.0

2
1

1
0

2.5
0.0

0
1 2 3 4 5 6 7

1 2 3 4 5 6 7

1 2 3 4 5 6 7

Figure 4: Performance gains of REmatch optimizations
Table 2: Average memory usage of different versions (MB).

Naive
Node Manager
Next Index
Offset
Early Output
Light Search
REmatch

For each dataset roughly 10,000 queries were generated. We then
sample 150 queries from each set and use these for our experiments.
The queries were designed to cover real world-scenarios where
overlapping matches occur naturally. For instance, in the DNA
dataset, a starting motif might be paired with multiple occurrences
of an end motif, which requires the use of the all-match semantics.
How we ran the queries. All the experiments were run on a Apple
M1 Pro 10 cores/10 threads machine with clock speed 2064 – 3220
MHz and 16GB RAM. The operating system used was MacOS 13.1.
Queries were run in succession, and each query was executed 5
times with the average of runtime/memory being reported.

7.2

(4) Offset
(5) Early Output

DNA
1202.1
3.19
1333.6
7.8
1268.2
11.2
13.4

Literature
435.1
2.1
517.3
271
453
1.96
2.1

SPARQL
1418.4
7.98
1502.5
9.86
1457.3
739.8
3.6

• Early Output, which outputs results as soon as they are
available (Section 5).
• Light Search, which finds a valid segmentation of the
document that are guaranteed to produce the output and
runs the full algorithm over these segments (Section 4).
• REmatch, which is the full version of the system.

What do our optimizations do?

Throughout the paper we described a series of optimizations which
define the REmatch system architecture, as illustrated in Figure 3.
Of course, a natural question to ask is what is the effect of each
one of these optimizations, and whether implementing the basic
algorithm for computing all matches of a REQL expression would
be competitive enough already? In this subsection, we test that
hypothesis, and show that a naive implementation of Algorithm 2
runs several orders of magnitude slower that the full REmatch
stack. Additionally, we test how each single optimization in isolation affects the performance of REmatch. For this, we run our
experiments with the following versions of REmatch:

Here we test the effect of each optimization in isolation. For
instance, the Offset version only has the offset optimization on top
of the naive algorithm. The exception is the full REmatch system,
which includes all of the mentioned optimizations. We consider two
metrics: runtime (Figure 4) and memory consumption (Table 2).
Discussion. As we can observe (Figure 4), each of our optimizations does drop the total runtime of the workload. The most consistent improvements come from Next index and Offset optimizations. The Light Search version offers significant improvements,
particularly in the median, but bad cases can hamper its performance, as witnessed over the SPARQL dataset. Over all the dataset
the full version of REmatch runs the fastest, as expected. Concerning memory consumption (Table 2), Node Manager drastically
reduces memory usage. Offset and Light Search can also reduce
memory usage, but their performance varies significantly based on
the query load. Over all the tested dimensions, the full version of
REmatch shows superior performance to any a single optimization
on its own. Slight hit in memory usage is noticeable in some cases
due to the interaction of different optimization methods in the full
version, but overall memory usage is still very low.

• Naive, which is just the implementation of Algorithm 2.
• Node Manager, which adds the NM module and will discard
unusable nodes as soon as possible (Section 5).
• Next Index, which uses a bit array for ASCII characters in
a transition for quick access (Section 6).
• Offset, which postpones storing the (potential) output
information as much as possible (Section 3).
8 A morpheme is the smallest meaningful constituent of a linguistic expression.

2802

<!-- page 12 -->

7.3

Comparison with other engines

(1) REmatch
(2) RE2

The setup. Here we do a thorough analysis of how REmatch compares to classic RegEx processing libraries. For a fair comparison,
we considered a representative set of RegEx engines implemented
in C++ that can approximate the all-match semantics using lookaround operators. In addition, we also considered two engines that
do not support look-around operators; and will thus not output the
same matches. The engines used for comparison are:

(3) PCRE
(4) PCRE2

DNA

(5) Boost
(6) Oniguruma

Literature
5

3.0
1.5
Time (s)

SPARQL

3.5

2.0

• PCRE [35] version: 8.45;
• PCRE2 [36] version 10.40 (using JPCRE2 C++ wrapper [27]);
• pcregrep [37] version 8.45;
• Boost.Regex [9] version 1.81.0;
• Oniguruma [34] version 6.9.7.1;
• RE2 [38] version 2021–11–01; and
• TRE [45] version 0.8.0.

(7) TRE
(8) pcregrep

2.5

4

2.0

3

1.0
1.5

2

1.0

0.5

1

0.5
0.0

0

0.0
1 2 3 4 5 6 7

12345678

1 2 3 4 5 6 8

Figure 5: Runtime metrics of our experiments

For engines that support look-around operators (PCRE, PCRE2,
Boost and Oniguruma) we rewrite the experiments from Subsection 7.1 so that they retrieve all the matches. For RE2 and TRE, which
do not support look-around operators, we rewrite the queries using capture groups such they resemble as closely as possible the
original experiments, although they do not perform the same task.
We also include grep and use its PCRE syntax to have a standard
command line tool in our comparison. Since standard grep does
not allow extracting substrings an extended version is used.
The results are presented in Figure 5 and Table 3. Here we compare only in terms of time. The memory consumption was relatively
stable along all the engines, with REmatch generally using slightly
more memory, which is justified by the extra bookkeeping needed
to retrieve all the matches. REmatch had some spikes in memory
usage for the DNA dataset, and PCRE2 in the SPARQL dataset, but
the document size still dominated memory usage significantly.
In this discussion, it is essential to recall that REmatch is incomparable with standard RegEx engines, given that it always finds all
matches. Therefore, it is not our purpose to prove that REmatch
is “faster” than other RegEx engines. Instead, we want to show
that the performance of REmatch is comparable to standard RegEx
engines, despite running a more heavy processing load.

Table 3: Average number of outputs (highest in bold).

REmatch
RE2
PCRE
PCRE2
pcregrep
Boost
Oniguruma
TRE

DNA
16,187.4
10,556.9
13,130.4
13,130.4
N/A
13,130.4
13,130.4
10,556.9

Literature
706.6
704.9
705.1
705.1
701.3
642.6
705.5
704.2

SPARQL
29,424.2
12,287.8
29,424.2
29,424.2
29,424.2
29,424.2
29,424.2
N/A

as in the other two datasets. The bad cases for REmatch come
from the extra bookkeeping needed to assure that all matches will
be captured. Overall, the experiments illustrate that the task of
encountering all the outputs can be done with minimal overhead
when comparing with classical RegEx matching.

8

CONCLUSIONS

This paper presents REmatch, a novel RegEx engine allowing to
find all matches. The reader can test the all-match semantics at
our beta demo available on www.rematch.cl. Such a semantics is
relevant in areas like literature or DNA analysis, where overlapping matches do matter. We experimentally prove that although
REmatch does a more demanding job, it does so with almost no additional cost when compared to classical RegEx engines, making it a
good candidate for use cases where all matches are needed. Finally,
this paper presents the architecture, algorithms, and optimizations
for capturing all matches. However, we see plenty of room for new
optimizations like state reductions, filtering, or indices. A promising first step would be to integrate the approach of [48, 49] into the
filtering module of REmatch, allowing to speed up our light search
procedure. Additionally, it would be interesting to see whether [48]
can be extended to also capture sub-patterns of the main search
pattern, and thus incorporated into the main REmatch algorithm.

Discussion. As we can see, REmatch shows good performance
as compared to other engines. On the Literature dataset, REmatch
is bested only by RE2, which does not look for all the outputs.
On the DNA dataset REmatch is a close third, bested only by RE2
and PCRE2 by a tiny margin. When it comes to SPARQL the story
is similar, now with pcregrep also being fairly competitive. We
remark that on the SPARQL dataset TRE throws an error on every
query, while over the DNA dataset pcregrep runs out of buffer,
since the document is one very long line. Comparing the number
of outputs, in Table 3 we can observe that REmatch generally does
more work compared to other engines. This is particularly evident
when comparing to engines not using look-around operators (RE2
and TRE). Even for the engines with look-around supported, we
sometimes cannot capture all the outputs (for instance when two
nested matches start at the same position), as witnessed in Table 3.
Interestingly, on the SPARQL set of experiments look-ahead allows
capturing all the outputs. While in this case PCRE2 does outperform
REmatch in general, the median result for REmatch is better, same

ACKNOWLEDGMENTS
Work supported by ANID – Millennium Science Initiative Program
– Code ICN17_002 and ANID Fondecyt Regular project 1221799.

2803

<!-- page 13 -->

REFERENCES

joins under updates. The VLDB Journal 29, 2-3 (2020), 619–653.
[25] IEEE and The Open Group. 2018. https://pubs.opengroup.org/onlinepubs/
9699919799/basedefs/V1_chap09.html. Accessed on 2023-07-21.
[26] Walter L. Johnson, James H. Porter, Stephanie I. Ackley, and Douglas T. Ross.
1968. Automatic Generation of Efficient Lexical Processors Using Finite State
Techniques. Commun. ACM 11, 12 (1968), 805–813.
[27] JPCRE2 C++ wrapper for PCRE2 library 2022. https://github.com/jpcre2/jpcre2.
Accessed on 2023-07-21.
[28] Stephen C Kleene et al. 1956. Representation of events in nerve nets and finite
automata. Automata studies 34 (1956), 3–41.
[29] Katja Losemann and Wim Martens. 2014. MSO queries on trees: enumerating
answers under updates. In CSL-LICS. 67:1–67:10.
[30] Andrea D. Sims Martin Haspelmath. 2010. Understanding Morphology. Hodder
Education.
[31] Francisco Maturana, Cristian Riveros, and Domagoj Vrgoc. 2018. Document
Spanners for Extracting Incomplete Information: Expressiveness and Complexity.
In PODS. 125–136.
[32] Martin Muñoz and Cristian Riveros. 2022. Streaming Enumeration on Nested
Documents. In ICDT. 19:1–19:18.
[33] Gonzalo Navarro and Mathieu Raffinot. 2005. New Techniques for Regular
Expression Searching. Algorithmica 41, 2 (2005), 89–116.
[34] Oniguruma – a modern and flexible regular expressions library 2022. https:
//github.com/kkos/oniguruma. Accessed on 2023-07-21.
[35] PCRE – Perl Compatible Regular Expressions 2022. https://www.pcre.org/. Accessed on 2023-07-21.
[36] PCRE2 – Perl-Compatible Regular Expressions 2022. https://github.com/
PCRE2Project/pcre2. Accessed on 2023-07-21.
[37] PCREgrep – A grep program that uses the PCRE regular expression library 2014.
https://github.com/vmg/pcre/blob/master/pcregrep.c/. Accessed on 2023-07-21.
[38] RE2 regular expression library 2022. https://github.com/google/re2. Accessed
on 2023-07-21.
[39] Nicole Schweikardt, Luc Segoufin, and Alexandre Vigny. 2022. Enumeration
for FO Queries over Nowhere Dense Graphs. Journal of the ACM (JACM) 69, 3
(2022), 22:1–22:37.
[40] Luc Segoufin. 2013. Enumerating with constant delay the answers to a query. In
ICDT. 10–20.
[41] Reetinder Sidhu and Viktor K Prasanna. 2001. Fast regular expression matching
using FPGAs. In FCCM. IEEE, 227–238.
[42] Anubhava Srivastava. 2017. Java 9 Regular Expressions. Packt Publishing.
[43] The LSQ team. 2015. The Linked SPARQL Queries Dataset. http://aksw.github.
io/LSQ/. Accessed on 2023-07-21.
[44] Ken Thompson. 1968. Programming techniques: Regular expression search
algorithm. Commun. ACM 11, 6 (1968), 419–422.
[45] TRE – a lightweight, robust, and efficient POSIX compliant regexp matching
library 2021. https://github.com/laurikari/tre. Accessed on 2023-07-21.
[46] Nikolaos Tziavelis, Deepak Ajwani, Wolfgang Gatterbauer, Mirek Riedewald, and
Xiaofeng Yang. 2020. Optimal Algorithms for Ranked Enumeration of Answers
to Full Conjunctive Queries. VLDB 13, 9 (2020), 1582–1597.
[47] W3C Sparql 2013. SPARQL 1.1 Query Language. https://www.w3.org/TR/
sparql11-query/. Accessed on 2023-07-21.
[48] Xiaochun Yang, Tao Qiu, Bin Wang, Baihua Zheng, Yaoshu Wang, and Chen Li.
2016. Negative factor: Improving regular-expression matching in strings. ACM
Transactions on Database Systems (TODS) 40, 4 (2016), 1–46.
[49] Xiaochun Yang, Bin Wang, Tao Qiu, Yaoshu Wang, and Chen Li. 2013. Improving
regular-expression matching on strings using negative factors. In SIGMOD. 361–
372.

[1] [n.d.]. Project Gutenberg. https://www.gutenberg.org/. Accessed on 2023-07-21.
[2] [n.d.]. REmatch Website. https://github.com/REmatchChile/REmatch-paper.
Accessed on 2023-07-21.
[3] [n.d.]. Sequence motif. https://en.wikipedia.org/wiki/Sequence_motif. Accessed
on 2023-07-21.
[4] Antoine Amarilli, Pierre Bourhis, Stefan Mengel, and Matthias Niewerth. 2021.
Constant-Delay Enumeration for Nondeterministic Document Spanners. ACM
Transactions on Database Systems (TODS) 46, 1 (2021), 2:1–2:30.
[5] Antoine Amarilli, Louis Jachiet, Martin Muñoz, and Cristian Riveros. 2022. Efficient Enumeration for Annotated Grammars. In PODS. 291–300.
[6] Christoph Berkholz, Fabian Gerhardt, and Nicole Schweikardt. 2020. Constant
delay enumeration for conjunctive queries: a tutorial. ACM SIGLOG News 7, 1
(2020), 4–33.
[7] Philip Bille and Mikkel Thorup. 2009. Faster Regular Expression Matching. In
ICALP. 171–182.
[8] BLAST: Basic Local Alignment Search Tool 2022. https://blast.ncbi.nlm.nih.gov/
doc/blast-help/downloadblastdata.html. Accessed on 2023-07-21.
[9] Boost Regex Library 2022. https://github.com/boostorg/regex. Accessed on
2023-07-21.
[10] Russ Cox. 2007. Regular expression matching can be simple and fast (but is slow
in java, perl, php, python, ruby,...). https://swtch.com/~rsc/regexp/regexp1.html.
Accessed on 2023-07-21.
[11] Russ Cox. 2010. Regular expression matching in the wild. https://swtch.com/
~rsc/regexp/regexp3.html. Accessed on 2023-07-21.
[12] Johannes Doleschal, Benny Kimelfeld, and Wim Martens. 2021. Database principles and challenges in text analysis. ACM SIGMOD Record 50, 2 (2021), 6–17.
[13] Johannes Doleschal, Benny Kimelfeld, Wim Martens, Yoav Nahshon, and Frank
Neven. 2019. Split-Correctness in Information Extraction. In PODS. 149–163.
[14] Krzysztof Dorosz and Anna Szczerbinska. 2009. Enhancing Regular Expressions
for Polish Text Processing. Computer Science 10 (2009), 19–36.
[15] Ronald Fagin, Benny Kimelfeld, Frederick Reiss, and Stijn Vansummeren. 2015.
Document Spanners: A Formal Approach to Information Extraction. Journal of
the ACM (JACM) 62, 2 (2015), 12:1–12:51.
[16] Fernando Florenzano, Cristian Riveros, Martín Ugarte, Stijn Vansummeren, and
Domagoj Vrgoc. 2020. Efficient Enumeration Algorithms for Regular Document
Spanners. ACM Transactions on Database Systems (TODS) 45, 1 (2020), 3:1–3:42.
[17] Christopher A. Flores, Rosa L. Figueroa, and Jorge E. Pezoa. 2021. Active Learning
for Biomedical Text Classification Based on Automatically Generated Regular
Expressions. IEEE Access 9 (2021), 38767–38777.
[18] Jeffrey E.F. Friedl. 2006. Mastering regular expressions. O’Reilly.
[19] The Swiss-Prot group. 2022. The PROSITE database. https://ftp.expasy.org/
databases/prosite/prosite.dat. World Wide Web Consortium.
[20] John E. Hopcroft and Jeffrey D. Ullman. 1979. Introduction to Automata Theory,
Languages and Computation. Addison-Wesley.
[21] How can I match overlapping strings with regex? 2014.
https:
//stackoverflow.com/questions/20833295/how-can-i-match-overlappingstrings-with-regex/33903830. Accessed on 2023-07-21.
[22] How to find overlapping matches with a regexp? 2013. https://stackoverflow.
com/questions/11430863/how-to-find-overlapping-matches-with-a-regexp. Accessed on 2023-07-21.
[23] Muhammad Idris, Martín Ugarte, and Stijn Vansummeren. 2017. The Dynamic
Yannakakis Algorithm: Compact and Efficient Query Processing Under Updates.
In SIGMOD. 1259–1274.
[24] Muhammad Idris, Martín Ugarte, Stijn Vansummeren, Hannes Voigt, and Wolfgang Lehner. 2020. General dynamic Yannakakis: conjunctive queries with theta

2804
