---
file: ../pdfs/2024-esop-fujinami-memoization-lookaround-atomic.pdf
citation: "Efficient Matching with Memoization for Regexes with Look-around and Atomic Grouping. Fujinami, Hasuo. ESOP 2024"
doi: 10.1007/978-3-031-57267-8_4
source: "https://doi.org/10.1007/978-3-031-57267-8_4 (open-access publisher version, extracted from ESOP 2024 volume, added manually)"
topics: [redos_backrefs_verification]
pages: 29
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Efficient Matching with Memoization for
Regexes with Look-around and Atomic
Grouping⋆
Hiroya Fujinami1,2(B)

and Ichiro Hasuo1,2

1

National Institute of Informatics, Tokyo, Japan
makenowjust@nii.ac.jp
2
SOKENDAI (The Graduate University for Advanced Studies), Hayama,
Japan

Abstract. Regular expression (regex) matching is fundamental in many
applications, especially in web services. However, matching by backtracking—preferred by most real-world implementations for its practical performance and backward compatibility—can suffer from so-called catastrophic backtracking, which makes the number of backtracking superlinear and leads to the well-known ReDoS vulnerability. Inspired by
a recent algorithm by Davis et al. that runs in linear time for (nonextended) regexes, we study efficient backtracking matching for regexes
with two common extensions, namely look-around and atomic grouping.
We present linear-time backtracking matching algorithms for these extended regexes. Their efficiency relies on memoization, much like the one
by Davis et al.; we also strive for smaller memoization tables by carefully trimming their range. Our experiments—we used some real-world
regexes with the aforementioned extensions—confirm the performance
advantage of our algorithms.
Keywords: regular expression · look-around · atomic grouping · pattern matching · ReDoS · memoization

1

Introduction

Regex Matching Regular expressions (regexes) are a fundamental formalism
for various pattern-matching tasks. Many regex matching implementations, however, suffer from occasional super-linear growth of their execution time. Such excessive execution time can be exploited for DoS attacks—this is a vulnerability
called regex denial of service (ReDoS). ReDoS is recognized as a significant security concern in many real-world systems, especially web services such as Stack
Overflow and Cloudflare (see §2.4 for more details).
Need for Efficient Backtracking Regex Matching The principal cause
of ReDoS is catastrophic backtracking, that is, the explosion of recursion in a
backtracking-based matching algorithm.
⋆

The authors are supported by CREST ZT-IoT Project (No. JPMJCR21M3), ERATO HASUO Metamathematics for Systems Design Project (No. JPMJER1603),
and ASPIRE Grant No. JPMJAP2301, JST.

c The Author(s) 2024
S. Weirich (Ed.): ESOP 2024, LNCS 14577, pp. 90–118, 2024.
https://doi.org/10.1007/978-3-031-57267-8_ 4

<!-- page 2 -->

Efficient Matching with Memoization for (la, at)-regexes

91

In regex matching, in general, a regex r is converted into a non-deterministic
finite automaton (NFA) A, and the latter is executed for an input string w. The
non-determinism of A can be resolved in either a depth-first or a breadth-first
manner. The former is called backtracking regex matching, and the latter is the
on-the-fly DFA construction.
Catastrophic backtracking and ReDoS are phenomena unique to the former
(i.e., backtracking)—as is well-known, the time complexity of the on-the-fly DFA
construction is linear (i.e., O(|w|)). Indeed, many modern regex implementations
are based on the on-the-fly DFA construction, including RE23 , Go’s regexp4 ,
and Rust’s regex5 .
It is practically essential, however, to make backtracking regex matching
more efficient. A principal reason is consistency. Most existing regex matching
implementations use backtracking, and they return only one matching position
out of many (see §2.3). While it is possible to replace them with on-the-fly
DFA matching, it is non-trivial to ensure consistency, that is, that the chosen
matching position is the same as the original backtracking matching implementation. .NET’s regex implementation has a linear-time complexity backend using
a derivative-based approach, which is compatible with a backtracking backend.
Still, it does not support look-around and atomic grouping [28]. Once the returned matching position changes, it can unexpectedly affect the behavior of all
the systems (e.g., web services) that use regex matching.
Another reason for improving backtracking regex matching is its extensibility.
There are many extensions of regexes widely used—such as the ones we study,
namely look-around and atomic grouping—and they are supported by few onthe-fly DFA matching implementations.
Existing Work: Linear-time Backtracking Matching with Memoization
Memoization is a well-known technique for speeding up recursive computations.
The recent work [10] shows that memoization can be applied to backtracking
regex matching with consistency in mind. Specifically, the work [10] presents
a backtracking matching algorithm that runs in O(|w|) time—thus, it is theoretically guaranteed to avoid catastrophic backtracking—for regexes without
extensions. (They also mention application to extended regexes in [10], but we
found issues in their discussion—see Remark 2).
Our Contribution: Linear-time Backtracking Matching for Some Extended Regexes In this paper, we present a linear-time backtracking matching algorithm for regexes with look-around and atomic grouping, two real-world
extensions of regexes. It uses memoization in order to achieve a linear-time complexity. We also prove that it is consistent (i.e., it chooses the same matching
position as the original algorithm without memoization).
The technical key to our algorithm is the design of suitable memoization tables. We follow the general idea in [10] of using memoization for backtracking
matching, but our examination of its issues with extended regexes (Remark 2)
https://github.com/google/re2
https://pkg.go.dev/regexp
5
https://docs.rs/regex/latest/regex/
3
4

<!-- page 3 -->

92

H. Fujinami and I. Hasuo

shows that the range—i.e., the set of possible entries—of memoization tables
should be suitably extended. Specifically, the range in [10] is {false}, recording only matching failures; it is extended in our algorithm to {Failure(j) | j ∈
{0, . . . , ν(A)}} ∪ {Success}. Here, ν(A) is the maximum nesting depth of atomic
grouping for the (extended) NFA A, defined in §5.
Our development is rigorous and systematic, based on the notion of NFA
whose labels can themselves be NFAs. This extended notion of NFA is suggested
in [10, Section IX.B]; in this paper, we formalize it and build its theory.
We experimentally evaluate our algorithm; the experiment results confirm its
performance advantages. Additionally, we survey the usage status of look-around
and atomic grouping—two regex extensions of our interest—in real-world regexes
and demonstrate their wide usage (§6).
Technical Contributions We summarize our technical contributions.
– We propose a backtracking matching algorithm for regexes with look-around,
proving its linear-time complexity (§4). This algorithm fixes the issues in the
algorithm in [10] (Remark 2) and restores correctness and linearity.
– We also propose a backtracking matching algorithm for regexes with atomic
grouping, proving its linear-time complexity (§4).
– We experimentally confirm the performance of our algorithms (§6).
– We investigate the usage status of look-around and atomic grouping in realworld regexes and confirm their wide usage (§6).
– We establish a rigorous theoretical basis for our algorithms for extended
regexes, namely NFAs with sub-automata (§2.6).
Organization We provide some preliminaries in §2, such as regex extensions
of our interest. Our formalization of NFAs with sub-automata is also presented
here. In §3, we discuss the work [10] that is closest to ours. We present our
matching algorithm for regex with look-around in §4 and the one for regex with
atomic grouping in §5. Then, we discuss our implementation and experimental
evaluation in §6. We conclude in §7.
Some additional proofs and other materials are deferred to the appendices in
the extended version [15].
Related Work Many related works are discussed elsewhere in suitable contexts.
Here, we discuss other related works.
There are many theoretical studies on look-around and atomic grouping. The
work [27] is a theoretical study of look-ahead operators; it shows how to convert
them to finite automata. Another conversion based on derivatives is introduced
in [26]. The work [3] conducts a fine-grained analysis of the size of DFAs obtained from converting regexes with look-ahead, improving the bounds given
in [26,27]. The work [5] discusses the relation between look-ahead operators and
back-references in regexes. A recent study [22] presents a linear-time matching
algorithm for regexes with look-around; it uses a memoization-like construct for
efficiency. However, the compatibility with backtracking is not a concern there,
unlike the current work. On atomic grouping, conversion to finite automata is
proposed [4], where atomic grouping is simulated by look-ahead.

<!-- page 4 -->

Efficient Matching with Memoization for (la, at)-regexes

93

Another common regex extension is back-reference. We do not deal with this
extension because 1) this extension is known to be non-regular (i.e., the language
class defined by back-reference is beyond regular), and 2) its matching problem
is known to be NP-complete [1] (thus the search for linear-time matching is
doomed). There are other extensions (absent operators, conditional branching,
etc.), but they are used less often (cf. §6).
ReDoS countermeasures are an active scientific topic. Besides efficient matching, there are two directions for them: ReDoS detection and ReDoS repair. ReDoS detection is a problem that determines whether a given regex can cause
catastrophic backtracking. This can be done by finding specific structures in a
transition diagram of an automaton [2, 18, 29, 34, 36, 37]. Besides, dynamic analysis, such as fuzzing [31], and combinations of static and dynamic analyses [19]
are studied. ReDoS repair is a problem of modifying a given regex so that it
does not cause ReDoS. Known solutions include exploring ReDoS-free regexes
using SMT solvers [6, 21] and rule-based rewriting of vulnerable regexes [20].
These ReDoS detection and repair measures are computationally demanding,
and their real-world deployment is limited.
There are other implementation-level studies on speeding up regex matching,
such as Just-in-Time (JIT) compilation [17] and FPGA [32]. However, these
studies are not intended to prevent catastrophic backtracking.

2

Preliminaries

We introduce preliminaries for this paper. Firstly, we present some basic concepts such as regexes, NFAs, conversion from regexes to NFAs, and backtracking
matching. We then discuss catastrophic backtracking and the ReDoS vulnerability that it can cause. Finally, we introduce look-around and atomic grouping as
practical regex extensions and NFAs with sub-automata for these extensions.
We fix a finite set Σ as an alphabet throughout this paper. We call sequences
of elements of Σ strings. The empty string is denoted by ε. For a string w =
σ0 σ1 . . . σn−1 , the length of w, denoted by |w|, is defined as |w| = n. We also
write w[i] = σi for i ∈ {0, . . . , n − 1}.
We use partial functions for memoization. For two sets A and B, a partial
function G from A to B, denoted by G : A ⇀ B, is defined as a function G : A →
B ∪ {⊥}. Here ⊥ is the element for “undefined,” and it is assumed that ⊥ ̸∈ B.
Let G : A ⇀ B be a partial function, a ∈ A, and b ∈ B. We let G(a) ← b
denote an updated partial function: it carries a to b, and any other x ∈ A to
G(x) (it is undefined if G(x) is initially undefined).
2.1

Regexes

Regular expressions (regexes) are defined by the following abstract grammar.
r ::= σ (a (literal) character, where σ ∈ Σ)
| r|r (an alternation)
| r∗ (a repetition)

| ε (the empty string)
| r·r (a concatenation)

<!-- page 5 -->

94

H. Fujinami and I. Hasuo
q0

Char(σ)

q1

q0

(a) σ ∈ Σ

Eps

q1

Eps

r1

(b) ε

r2

(c) r1 · r2
Eps

r

r1
q0

Branch

q1
r2

(d) r1 |r2

q0

Branch

q1

(e) r∗

Fig. 1: a conversion from regexes to NFAs
The concatenation operator · may be omitted when there is no ambiguity. The
precedence of operators is as follows: repetition, concatenation, and alternation.
For example, ab∗ |c means (a · (b∗ ))|c.
For a regex r, the size of r, denoted by |r|, is defined as follows: |σ| = |ε| = 1,
|(r1 |r2 )| = |r1 · r2 | = |r1 | + |r2 | + 1, and |r∗ | = |r| + 1.
2.2

NFAs

A non-deterministic finite state automaton (NFA) is a quadruple (Q, q0 , F, T ),
where Q is a finite set of states q0 ∈ Q is an initial state, F ⊆ Q is a set
of accepting states, and T is a transition function. For each q ∈ Q \ F , T (q)
can be one of the following: T (q) = Eps(q ′ ), T (q) = Branch(q ′ , q ′′ ), and T (q) =
Char(σ, q ′ ) where q ′ , q ′′ ∈ Q and σ ∈ Σ.
The above definition of a transition function T is tailored to our purpose of
backtracking. Compared to the common definition δ : Q × ({ε} ∪ Σ) → 2Q , it
expresses general branching as combinations of certain elementary branchings.
The latter is namely one transition by ε, two transitions by ε, and one transition by a certain character σ ∈ Σ. This makes the description of backtracking
matching easier. Note, in particular, that the successors q ′ , q ′′ in the branching Branch(q ′ , q ′′ ) are ordered. Here, q ′ and q ′′ are called the first and second
successors, respectively. This definition of transition functions is similar to the
op-codes of many real-world regex-matching implementations (cf. [8]).
We present a conversion from regexes to NFAs (see Figure 1); it is similar to
the Thompson–McNaughton–Yamada construction [23, 35]. For a regex r, A(r)
denotes the NFA A converted from r. In the figure, labels on arrows show kinds
of transitions. In a Branch transition, the top arrow points to the first successor,
and the bottom points to the second successor. Rectangles indicate that the
conversion is applied to sub-expressions inductively. Because each case of this
construction introduces at most two new states, for a regex r and the NFA
A(r) = (Q, q0 , F, T ), we have |Q| = O(|r|).

<!-- page 6 -->

Efficient Matching with Memoization for (la, at)-regexes

95

Algorithm 1 a partial backtracking matching algorithm for NFAs
1: function MatchA,w (q, i)
Parameters: an NFA A, and an input string w
Input: a current state q, and a current position i
Output: returns SuccessAt(i′ ) if the matching succeeds, or
returns Failure if the matching fails
2:
(Q, q0 , F, T ) = A
3:
if q ∈ F then return SuccessAt(i)
4:
else if T (q) = Eps(q ′ ) then return MatchA,w (q ′ , i)
5:
else if T (q) = Branch(q ′ , q ′′ ) then
6:
result ← MatchA,w (q ′ , i)
7:
if result = Failure then return MatchA,w (q ′′ , i)
8:
else return result
9:
else if T (q) = Char(σ, q ′ ) then
10:
if i < |w| and w[i] = σ then return MatchA,w (q ′ , i + 1)
11:
else return Failure

We collectively call Eps and Branch transitions ε-transitions. Later in this
paper, if there are consecutive ε-transitions, they may be shown as a single
transition in a figure. When a certain state returns to itself by ε-transitions,
such a sequence of ε-transitions is called an ε-loop. ε-loops are problematic in
matching because they cause infinite loops in matching.
An ε-loop can be detected during matching by recording a position on an
input string when a state is visited. When an ε-loop is detected, several solutions
exist to deal with it (see, e.g., [30]), such as treating an ε-loop as a failure (e.g.,
JavaScript and RE2) or treating it as a success but escaping it (e.g., Perl). These
solutions can be easily adapted to our algorithms; therefore, for the simplicity
of presentation, we introduce the following assumption.
Assumption 1 (no ε-loops). NFAs do not contain ε-loops.
2.3

Backtracking Matching

We present a basic backtracking matching algorithm for NFAs in Algorithm 1.
It serves as a basis for optimization by memoization, both in [10] and in the
current work.
The function MatchA,w is recursively called in this algorithm, but it must
terminate on Asm. 1. It takes two parameters: A is an NFA, and w is an
input string. It also takes two arguments: q ∈ Q is the current state, and
i ∈ {0, . . . , |w|} is the current position on w. MatchA,w (q0 , i) for an NFA A =
(Q, q0 , F, T ) returns SuccessAt(i′ ) with the matching position i′ ∈ {0, . . . , |w|} if
the matching with A succeeds from i to i′ on w, or returns Failure if the matching
fails.
The Match function implements partial matching: given the position i ∈
{0, . . . , |w|} of interest, one obtains, by running MatchA,w (q0 , i), one “matching
position” i′ (if it exists) such that w[i] w[i+1] . . . w[i′ ] is accepted by A. Note the

<!-- page 7 -->

96

H. Fujinami and I. Hasuo

Eps

q2
q1
q0

q3

Eps

Branch

q6

Branch

q4
q7

Char(a)

Char(b)

Char(a)

q5

Eps

q8

∗

Fig. 2: the NFA A((a|a) b)
difference from total matching: given A and w, it returns true if (the whole) w is
accepted by A and false otherwise. The practical relevance of partial matching
must be clear, as we can use it for text search and replacement.
Lines 5 to 8 in Algorithm 1 perform matching for Branch transitions. Here,
the algorithm first tries matching from the first successor q ′ , and if that fails,
it tries matching from the second successor q ′′ with the same position. This
behavior is called backtracking.
We define the regex partial matching problem using the function Match.
Problem 1 (regex partial matching).
Input: a regex r, an input string w, and a starting position i ∈ {0, . . . , |w|}
Output: returns MatchA(r),w (q0 , i) where A(r) = (Q, q0 , F, T ).
Remark 1. One can say that the problem formulation is a bit strange. It requires,
as output, a specific matching position chosen by a specific algorithm Match,
while a usual formulation would require an arbitrary matching position. We take
this formulation since we aim to show that our optimization by memoization not
only solves partial matching but also is consistent with an existing backtracking
matching algorithm, in the sense we discussed in §1. We formulate consistency
as correctness with respect to Prob. 1, that is, preserving the solution chosen by
the specific algorithm Match. We also note that the algorithm Match mirrors
many existing implementations of regex matching (cf. §2.2).
2.4

Catastrophic Backtracking and ReDoS

In the execution of the Match function (Algorithm 1), depending on an NFA
A and an input string w, the number of recursive calls for the Match function
may increase explosively, resulting in a very long matching time, as we will see
in Example 1. This explosive increase in matching time is called catastrophic
backtracking.
∗

Example 1 (catastrophic backtracking). Consider the NFA A = A((a|a) b) =
(Q, q0 , F, T ) shown in Figure 2, and let w = "an c" (the string repeating a of n
times and ending with c) be an input string. MatchA,w (q0 , 0) invokes recursive

<!-- page 8 -->

Efficient Matching with Memoization for (la, at)-regexes

97

calls O(2n ) times until returning Failure. The reason for this recursive call explosion is to try all combinations on q2 to q3 and q4 to q5 transitions for each a
in w during the matching.
Regexes denial of service (ReDoS) is a security vulnerability caused by catastrophic backtracking. In ReDoS, catastrophic backtracking causes a huge load
on servers, making them unable to respond in a timely manner. There are cases
of service outages due to ReDoS at Stack Overflow in 2016 [12] and at Cloudflare
in 2019 [16]. Additionally, a 2018 study [33] reported that over 300 web services
have potential ReDoS vulnerabilities. Thus, ReDoS is a widespread problem in
the real world, and there is a great need for countermeasures.
According to a 2019 study [25], only 38% of developers are aware of ReDoS. This study also found that many developers find it difficult not only to
read regexes but also to find and validate regexes to match their desires. It is
mentioned in [25] that developers use Internet resources such as Stack Overflow
to find regexes. In recent years, it has also become common to use generative
AIs such as ChatGPT for such a purpose. However, when the authors asked,
“Please suggest 10 regexes for validating email addresses” to ChatGPT,6 2 of
the 10 suggested regexes would cause ReDoS (see Table 1). Developers may unknowingly use such vulnerable regexes. For this reason, it is important to develop
ReDoS countermeasures that can be achieved without the developer being aware
of them.
Matching speed-up is a way to avoid causing ReDoS by ensuring that matching is linear in time to the length of an input string, freeing developers from worrying about ReDoS. A popular method for matching speed-up is using breadthfirst search for non-deterministic transition instead of backtracking (depth-first
search); it is called the on-the-fly DFA construction [7, 28]. However, since lookaround and atomic grouping are extensions based on backtracking (see §2.6), it
is not obvious that they can be supported by the on-the-fly DFA construction.
Memoization is another approach to ensuring linear-time backtracking matching; we pursue it in this paper.
2.5

Regex Extensions: Look-around and Atomic Grouping

Many real-world regexes come with various extensions for enhanced expressivity [13]. In this paper, we are interested in two classes of extensions, namely
look-around and atomic grouping.
Look-around Look-around is a regex extension that allows constraints on
strings around a certain position. It is also called zero-width assertion (e.g.,
in [10]) because it does not consume any characters. Look-around consists of
four types: positive or negative, and look-ahead or look-behind.
Positive look-ahead is typically represented by the syntax (?=r); its matching
succeeds when, reading ahead from the current position of the input string, the
6
7

We used ChatGPT 3.5 (September 25, 2023 version).
The second and third regexes are the same; they are the actual output of ChatGPT.

<!-- page 9 -->

98

H. Fujinami and I. Hasuo

Table 1: the regexes given by ChatGPT for the question “Please suggest 10
regexes for validating email addresses”.7
suggested regex (ChatGPT’s comment)

vulnerable?

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$
(Basic Email Validation)

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$
(Basic Email Validation with TLD)

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$
(Strict Email Validation)

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(?:\.[a-z
A-Z]{2,})?$
(Email Validation Allowing for Subdomains)

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{1,}$
(Email Validation Allowing Single-Character Domain Name)

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(?:\.[\p{
L}\p{N}]{2,})?$
(Email Validation Allowing Internationalized Domain Names (IDNs))

ˆ(?:"[\w\s]+")?([a-zA-Z0-9._%+-]+)@[a-zA-Z0-9.-]+\.[a-zA
-Z]{2,}$
(Email Validation with Optional Quoted Local Part)

ˆ(?:\([ˆ()]*\)|[\w\s]+)?([a-zA-Z0-9._%+-]+)@[a-zA-Z0-9.]+\.[a-zA-Z]{2,}$
vulnerable
(Email Validation with Optional Comments)

ˆ[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$i
(Email Validation Allowing for Case-Insensitive Domain)

ˆ(?:(?:[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})|(
[a-zA-Z0-9._%+-]+)\+[ˆ@]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$ vulnerable
(Email Validation with Support for Subaddressing)

matching of the inner regex r succeeds. Note that the position for the overall
matching does not change by the inner matching of r. For example, the regex
/(?=bc)/ matches the string "abc" from position 1 (i.e., after the first character
a) without consuming any characters.
The matching of a negative look-ahead (?!r) succeeds when the inner regex
r is not matched.
Positive or negative look-behind—denoted by (?<=r) or (?<!r), respectively—
is similar to the above, with the difference that the inner matching of r is performed backward, i.e., from right to left. For example, the regex /(?<=ab)/
matches the string "abc" from position 2 (i.e., before the last character c) without consuming any characters.
A typical use of look-around is to put a look-behind before (or a look-ahead
after) a regex r. This is useful when one wants to perform a search or replacement
of r for only those occurrences that are in a certain context. For example, the
regex /(?<=<p>)[ˆ<]*(?=<\/p>)/ matches only contents of the HTML <p> tag.
As another example, common assertions such as \A (this matches the beginning

<!-- page 10 -->

Efficient Matching with Memoization for (la, at)-regexes

99

of a string) and \z (this matches the end) can be expressed using look-around,
namely \A = (?<!.) and \z = (?!.).
Atomic Grouping Atomic grouping is a regex extension that controls backtracking behaviors. It is designed to manually avoid problems caused by backtracking, such as catastrophic backtracking (§2.4).
Atomic grouping is represented by the syntax (?>r); once the matching of
the inner regex r succeeds, the remaining branches in potential backtracking for
matching r are discarded. For example, the regex /(a|ab)c/ matches the string
"abc", but the regex /(?>(a|ab))c/ using atomic grouping does not match it.
This is because, once a in the atomic grouping matches the first character a of
"abc", the remaining branch ab (in a|ab) is discarded, and one is left with the
regex c and the string "bc".
Atomic grouping is often used for the purpose of preventing catastrophic
backtracking. In that case, it is used in combination with the repetition syntax,
e.g., (?>(r*)) (often abbreviated as r*+) and (?>(r+)) (abbrev. as r++). These
abbreviations are called possessive quantifiers. The former (namely (?>(r*)))
is intuitively understood as (?>(ε|r|rr|. . . )), with the difference that longer
matching is preferred (this is because the Eps loop is the first successor in Figure 1e). Once a longer match is found, the remaining branches (i.e., those for
shorter matches) get discarded, thus preventing catastrophic backtracking.
One might wonder if our (linear-time and thus ReDoS-free) matching algorithm should support atomic grouping—the principal use of atomic grouping
is to suppress backtracking and avoid ReDoS. We do need to support it since,
as we discussed in §1, ours is meant to be a drop-in replacement for matching
implementations that are currently used.
Our Target Extended Regexes Our target class, namely regexes with lookaround and atomic grouping, is defined by the following grammar.
r ::= . . .
| (?=r) | (?!r)

(the same as the regexes definition, §2.1)
(positive and negative look-ahead)

| (?<=r) | (?<!r)

(positive and negative look-behind)

| (?>r)

(atomic grouping)

For brevity, we sometimes refer to regexes with look-around and atomic grouping
as (la, at)-regexes. We also refer to regexes with look-around as la-regexes and
regexes with atomic grouping as at-regexes.
For a (la, at)-regex r, the size of r, denoted by |r|, is defined as the same as
the regex one except for |(?=r)| = |(?>r)| = |r| + 1.
Look-around is known to be regular: they can be converted to DFA, and the
language family of la-regexes is the same as the regular language. This fact is
mentioned in [3,26,27]. Atomic grouping is also known to be regular in the same
sense [4]. However, it is known that look-ahead and atomic grouping can make
the number of states of the corresponding DFA grow exponentially [3, 4, 26, 27].
In what follows, for simplicity, we only discuss positive look-ahead in discussions of look-around. Adaptation to other look-around operators, such as
negative look-behind, is straightforward.

<!-- page 11 -->

100

H. Fujinami and I. Hasuo

2.6

NFAs with Sub-automata

We introduce NFAs with sub-automata for backtracking matching algorithms for
(la, at)-regexes. This extended notion of NFAs is suggested in [10, Section IX.B],
but it seems ours is the first formal exposition.
Roughly speaking, an NFA with sub-automata is an NFA whose transitions
can be labeled with—in addition to a character σ ∈ Σ, as in usual NFAs—
another NFA with sub-automata. See Figure 3, where transitions from q0 to q1
are labeled with r , the NFA with sub-automata obtained by converting r. We
annotate these transitions further with a label (pla for positive look-ahead, at
for atomic grouping, etc.) that indicates which operator they arise from. Note
that NFAs with sub-automata can be nested—transitions in r in Figure 3 can
be labeled with NFAs with sub-automata, too.
Our precise definition is as follows. There, P is the set that collects all states
that occur in an NFA with sub-automata A, i.e., in 1) the top-level NFA, 2) its
label NFAs, 3) their label NFAs, and so on.
Definition 1 (NFAs with sub-automata). An NFA with sub-automata A
is a quintuple A = (P, Q, q0 , F, T ) where P is a finite set of states and Q ⊆ P is a
set of so-called top-level states. We require that the quadruple (Q, q0 , F, T ) is an
NFA, except that the value T (q) of the transition function T is either 1) Eps(q ′ ),
Branch(q ′ , q ′′ ), or Char(σ, q ′ ) (as in usual NFAs, §2.2), or 2) Sub(k, A′ , q ′ ), where
A′ is an NFA with sub-automata, q ′ is a successor state, and k is a kind label
where k ∈ {pla, nla, plb, nlb, at}.
We further impose the following requirements. Firstly, we require all NFAs
with sub-automata in A to have disjoint state spaces. That is, for any distinct top-level states q, q ′′ ∈ Q in A, if T (q) = Sub(k, A′ , q ′ ) and T (q ′′ ) =
Sub(k ′ , A′′ , q ′′′ ), then we must have P ′ ∩ P ′′ = ∅, Q ∩ P ′ = ∅ and Q ∩ P ′′ = ∅,
where A′ = (P ′ , . . . ) and A′′ = (P ′′ , . . . ). Secondly, we require that the set P
in A = (P,
S . . . ) is the (disjoint) union of all states that occur within A, that is,
P = Q ∪ q∈Q,T (q)=Sub(k,A′ ,q′ ),A′ =(P ′ ,... ) P ′ .
The kind label k in Sub(k, A′ , q ′′ ) indicates how the sub-automaton A′ should
be used (cf. Algorithm 2). If every kind label occurring in A (including its subautomata) is either pla, nla, plb, or nlb, then A is called a la-NFA. Similarly, if
every kind label is at, A is called an at-NFA. Following this convention, general
NFAs with sub-automata are called (la, at)-NFAs.
Note that the definition is recursive. Non-well-founded nesting is prohibited,
however, by the finiteness of P . By the definition, if P = Q, then A does not
contain any transitions labeled with sub-automata.
In addition to Eps and Branch transitions, we refer to Sub transitions with
a label k ∈ {pla, nla, plb, nlb} as ε-transitions too. We also assume the following,
similarly to Asm. 1.
Assumption 2. (la, at)-NFAs do not contain ε-loops.

<!-- page 12 -->

Efficient Matching with Memoization for (la, at)-regexes
Sub(pla, r )
q0

101

Sub(at, r )
q1

(a) (?=r) (positive look-ahead)

q0

q1

(b) (?>r) (atomic grouping)

Fig. 3: a conversion from (la, at)-regexes to (la, at)-NFAs. For negative lookahead, we use the corresponding kind label nla. For positive and negative lookbehind, besides using the kind labels plb and nlb, we suitably reverse r .
Algorithm 2 a partial backtracking matching algorithm for (la, at)-NFAs
1: function Match-(la, at)A,w (q, i)
Parameters: a (la, at)-NFA A, and an input string w
Input: a current state q, and a current position i
Output: returns SuccessAt(i′ ) if the matching succeeds, or
returns Failure if the matching fails
2:
(P, Q, q0 , F, T ) = A
3:
if q ∈ F then
..
.
▷ the same as Algorithm 1
12:
else if T (q) = Sub(pla, A′ , q ′ ) then
▷ positive look-ahead; other look-around ops. are similar
13:
(P ′ , Q′ , q0′ , F ′ , T ′ ) = A′
14:
result ← Match-(la, at)A′ ,w (q0′ , i)
15:
if result = SuccessAt(i′ ) then return Match-(la, at)A,w (q ′ , i)
16:
else return r
17:
else if T (q) = Sub(at, A′ , q ′ ) then
▷ atomic grouping
18:
(P ′ , Q′ , q0′ , F ′ , T ′ ) = A′
19:
result ← Match-(la, at)A′ ,w (q0′ , i)
20:
if result = SuccessAt(i′ ) then return Match-(la, at)A,w (q ′ , i′ )
21:
else return r

For (la, at)-regexes, their conversion to (la, at)-NFAs is described by the
constructions in Figure 3—using transitions labeled with sub-automata—in addition to the conversion for regexes in §2.2. Note that we have |P | = O(|r|) in
these constructions.
The backtracking matching algorithm in Algorithm 1 can be naturally extended to (la, at)-NFAs; it is shown in Algorithm 2. The clauses for positive
look-ahead (Lines 12 to 16) and atomic grouping (Lines 17 to 21) are similar to
each other, conducting matching for sub-automata. Note that their difference is
in the “return position” (i in Line 15; i′ in Line 20).
The clauses for other look-around operators are similar to the ones for positive look-around. For look-behind, we can suitably use an additional parameter
d ∈ {−1, +1} for indicating a matching direction.
Using the extended backtracking matching algorithm (Algorithm 2), we define the partial matching problem for (la, at)-regexes in the same way as for
regexes without extensions (Prob. 1).

<!-- page 13 -->

102

H. Fujinami and I. Hasuo

Problem 2 ((la, at)-regex partial matching).
Input: a (la, at)-regex r, an input string w, and a starting position i
Output: returns Match-(la, at)A(r),w (q0 , i) where A(r) = (P, Q, q0 , F, T ).

3

Previous Works on Regex Matching with Memoization

This section introduces an existing work [10] on regex matching with memoization, paving the way for our algorithms for (la, at)-regexes in Sections 4 and 5.
Memoization is a programming technique that makes recursive computations
more efficient by 1) recording arguments of a function and the corresponding
return values and 2) reusing them when the function is called with the recorded
arguments.
As we described in §2.3, regex matching is conducted by backtracking matching. It is implemented by recursive functions (see Algorithms 1 and 2); thus, it is
a natural idea to apply memoization. Since Java 14, Java’s regex implementation
has indeed used memoization for optimization. However, this optimization is not
enough to completely prevent ReDoS; see, e.g., [24].
The work that inspires the current work the most is [10], whose main novelty
is linear-time backtracking regex matching (much like the current work). Its
contributions are as follows.
1. Focusing on (non-extended) regexes (see §2.1), they introduce a backtracking
matching algorithm that uses memoization. It achieves a linear-time complexity: for an input string w, its runtime is O(|w|).
2. They introduce selective memoization, by which they reduce the domain of
the memoization table from Q × N to Qsel × N. Here Qsel is a subset of Q
that is often much smaller.
3. They introduce a memory-efficient compression method—based on run-length
encoding (RLE)—for memoization tables.
4. Finally, they discuss adaptations of the above method to extended regexes,
namely REWZWA (the extension by look-around; look-around is called zerowidth assertion in [10]) and REWBR (the extension by back-reference).
We will mainly discuss the above item 1; it serves as a basis for our algorithms
in Sections 4 and 5. The technique in the item 2 is potentially very relevant: we
expect that it can be combined with the current work; doing so is future work.
The content of the item 2 is reviewed in [15, Appendix A] for the record.
Remark 2. On the above item 4, the work [10] claims that the time complexity
of their algorithm is linear also for REWZWA (O(|w|) for an input string w).
However, we believe that this claim comes with the following problems.
– The description of an algorithm for REZWA in [10] is abstract and leaves
room for interpretation. The description is to “preserve the memo functions
of the sub-automata throughout the simulation of the top-level M-NFA, remembering the results from sub-simulations that begin at different indices i
of w” [10, Section IX-B]. For example, it is not explicit what the “results”
are—they can mean (complete) matching results or mere success/failure.

<!-- page 14 -->

Efficient Matching with Memoization for (la, at)-regexes

103

Algorithm 3 a total matching algorithm with memoization for NFAs without
ε-transitions [10, Listing 2].
1: function DavisSLM
A,w (q, i)
Parameters: an NFA A without ε-transitions, an input string w, and
a memoization table M : Q × N ⇀ {false}
Input: a current state q, and a current position i
Output: returns true if the matching succeeds, or
returns false if the matching fails
2:
(Q, q0 , F, δ) = A
3:
if i = |w| then return whether q ∈ F
4:
if M (q, i) ̸= ⊥ then return M (q, i)
5:
for q ′ ∈ δ(q, w[i]) do
′
6:
if DavisSLM
A,w (q , i + 1) then return true
7:
M (q, i) ← false
8:
return false

– Moreover, the part “that begin at different indices i of w” is problematic;
we believe that remembering these results does not lead to linear-time complexity. This point is discussed later in Remark 4.
– Besides, there is a gap between the algorithm described in the paper [10]
and its prototype implementation [11], even for (non-extended) regexes. See
Remark 3.
– Because of this gap, the implementation [11] works in linear time for all
regexes, including REZWA, but can lead to erroneous results for REZWA.
See Remark 4.
Our contribution includes a correct memoization algorithm for look-around (REZWA)
that resolves the above problems.
3.1

Linear-time Backtracking Matching with Memoization

We describe the first main contribution of the work [10] (the item 1 in the above
list), namely a backtracking algorithm that achieves a linear-time complexity
thanks to memoization. The algorithm [10, Listing 2] is presented in Algorithm 3.
In this algorithm DavisSLM
A,w , an NFA A is a quinq
tuple (Q, q0 , F, δ) where δ : Q × Σ → 2Q is an nona
a
deterministic transition function. An additional parameb
q
q
ter M : Q × N ⇀ {false} is a memoization table, which
a
a
is mathematically a mutable partial function. This algoq
rithm implements total matching (cf. §2.3). It is notable
that the memoization table records only matching failures:
a matching success does not have to be recorded since it Fig. 4: the NFA
∗
immediately propagates to the success of the whole prob- A((aa|aa) b),
after
removing
lem.
ε-transitions
1

0

2

3

<!-- page 15 -->

104

H. Fujinami and I. Hasuo

Algorithm 4 a variant of Algorithm 3 implemented in their prototype [11]
1: function DavisSLImplM
A,w (q, i)
2:
(Q, q0 , F, δ) = A
3:
if i = |w| then return whether q ∈ F
4:
if M (q, i) ̸= ⊥ then return M (q, i)
M (q, i) ← false
5:
for q ′ ∈ δ(q, w[i]) do
..
.
7:
M (q, i) ← false
..
.

▷ M (q, i) is speculatively set to false

▷ moved up

This algorithm achieves a linear-time matching. It thus prevents ReDoS. A
full proof of linear-time complexity is found in [10, Appendix C], but its essence
is the following (note the critical role of memoization here).
– For any call DavisSLM
A,w (q, i), if M (q, i) is defined, then the call does not
invoke any further recursive calls.
– When such a call returns false, the entry M (q, i) of the memoization gets
defined (Line 7).
– As a consequence, the number of recursive calls of DavisSLM
A,w is limited to
|Q| × |w|.
Example 2 (matching with memoization for NFAs without ε-transitions). Let us
consider the regex (aa|aa)∗ b and the corresponding NFA A((aa|aa)∗ b) defined
in §2.2. For the purpose of applying Algorithm 3, we manually remove its εtransitions, leading to the NFA in Figure 4. Let w = "a2n c" be an input string.
MatchA,w (q0 , 0) (without memoization) invokes recursive calls O(2n ) times for
0
the same reason as in Example 1, but DavisSLM
A,w (q0 , 0) (with memoization,
where M0 is the initial memoization table) invokes recursive calls O(n) times
because M (q0 , i) for each position i ∈ {0, 2, . . . , 2n} has been recorded after the
first visit.
Remark 3. Following the discussion in Remark 2, here we describe the gap between Algorithm 3—the algorithm described in the paper [10]—and its prototype
implementation [11]. The latter is shown in Algorithm 4.
The precise difference between the two algorithms is that Line 7 in Algorithm 3 is moved up to the moment just before the for-loop, in Algorithm 4.
It is not hard to see that this modification does not affect the correctness of
the algorithm: if the pair (q, i) is visited again in the future, it means that the
current matching from (q, i) did not succeed, and backtracking occurred. Note
that, in case the current matching is successful, the function call returns true
so the memoization content M (q, i) should not matter.
However, the above argument is true only when there is no look-around.
(A detailed discussion is in Example 3.) This point seems to be missed in the
implementation [11].

<!-- page 16 -->

Efficient Matching with Memoization for (la, at)-regexes

105

Algorithm 5 a partial matching algorithm with memoization. An adaptation
of Algorithm 3 from [10], and a basis for our algorithms (Algorithms 6 and 7)
1: function MemoM
A,w (q, i)
Parameters: an NFA A, an input string w, and
a memoization table M : Q × N ⇀ {Failure}
Input: a current state q, and a current position i
Output: returns SuccessAt(i′ ) if the matching succeeds, or
returns Failure if the matching fails
2:
(Q, q0 , F, T ) = A
3:
if M (q, i) ̸= ⊥ then return M (q, i)
4:
result ← ⊥
5:
if q ∈ F then result ← SuccessAt(i)
′
6:
else if T (q) = Eps(q ′ ) then result ← MemoM
A,w (q , i)
′
′′
7:
else if T (q) = Branch(q , q ) then
′
8:
result ← MemoM
A,w (q , i)
′′
9:
if result = Failure then result ← MemoM
A,w (q , i)
′
10:
else if T (q) = Char(σ, q ) then
′
11:
if i < |w| and w[i] = σ then result ← MemoM
A,w (q , i + 1)
12:
else result ← Failure
13:
if result = Failure then M (q, i) ← Failure
14:
return result
▷ result ̸= ⊥, as one can easily see

3.2

Matching with Memoization Adapted to the Current Formalism

In Algorithm 5, we present an adaptation of Algorithm 3 to our formalism,
especially our definition of NFA (§2.2) that offers fine-grained handling of nondeterminism. Algorithm 5 has been adapted also to solve partial matching (it
returns a matching position i′ ) rather than total matching as in Algorithm 3 (cf.
§2.3). Algorithm 5 serves as a basis towards our extensions to look-around and
atomic grouping in Sections 4 to 5.
The adaptation is straightforward: Line 5 ensures that the algorithm solves
partial matching; the rest is a natural adaptation of the for-loop of Algorithm 3
to our definition of NFA (§2.2). The algorithm terminates thanks to Asm. 1. We
note that the type of memoization tables does not have to be changed compared
to Algorithm 3.
Algorithm 5 exhibits the same desired properties as Algorithm 3, namely
correctness (with respect to Prob. 1) and linear-time complexity. We formally
state these properties for the record; here, M0 : Q × N ⇀ {Failure} is the initial
memoization table (its entry is anywhere ⊥).
Theorem 1 (linear-time complexity of Algorithm 5). For an NFA A =
0
(Q, q0 , F, T ), an input string w, and an position i ∈ {0, . . . , |w|}, MemoM
A,w (q0 , i)
terminates with O(|w|) recursive calls.
Theorem 2 (correctness of Algorithm 5). For an NFA A = (Q, q0 , F, T ),
an input string w, and an position i ∈ {0, . . . , |w|}, MatchA,w (q0 , i) =
0
MemoM
A,w (q0 , i).
The proofs can be found in [15, Appendix B.1]. Here is their outline.

<!-- page 17 -->

106

H. Fujinami and I. Hasuo
A′

Eps

Eps
q1
q0

Sub(pla, A′ )

q2

Char(a)

q3

q6

Branch

q5

Char(a)

q7

Branch

q4

q8

Fig. 5: the la-NFA A(((?=a∗ )a)∗ )
A′
Eps
q1
q0

Char(a)

Eps
q2

q8

Branch

q7
q3

Sub(at, A′ )

q4

Char(a)

q5

Char(b)

q6

Char(a)

q9

Branch

q10

Fig. 6: the at-NFA A(a∗ (?>a∗ )ab)
We first introduce the notion of run for Match and Memo; it records recursive calls of the function itself, as well as invocations of the memoization table,
together with their return values.
For linear time complexity (Thm. 1), we show that 1) a recursive call with
the same argument (q, i) appears at most once in a run, and that 2) the number
of invocations of the memoization table with the same key (q, i) is bounded by
the (graph-theoretic) in-degree. Linear-time complexity then follows easily.
For correctness (Thm. 2), we introduce a conversion from runs of Memo to
runs of Match. By showing that 1) the result is indeed a valid run of Match
and 2) the conversion preserves return values, we show the coincidence of the
return values of the two algorithms, i.e., correctness.

4

Memoization for Regexes with Look-around

We describe our first main technical contribution, namely a backtracking matching algorithm for la-NFAs with memoization (Algorithm 6). We prove that it is
correct (Thm. 4) and that its time complexity is linear (O(|w|), Thm. 3).
The key ingredient of our algorithm is the type of memoization tables, where
their range is extended from {Failure} to {Failure, Success}. We motivate this extension through two problematic algorithms MemoExit-la and MemoEnter-la;
MemoExit-la is obtained by naively extending Algorithm 5 (Memo) with adding
the processing of sub-automaton transitions with pla (positive look-ahead) done
in Algorithm 2 (Lines 12 to 16), and MemoEnter-la is similar to MemoExit-la,
but this records to the memoization table at the same timing as Algorithm 4
(DavisSLImpl). In particular, their memoization tables only record false.
The example below shows the problems with the two naive algorithms. Specifically, MemoExit-la is not linear and MemoEnter-la is not correct.
Example 3. Consider the la-NFA A = A(((?=a∗ )a)∗ ) = (P, Q, q0 , F, T ) shown in
Figure 5, and let w = "an " be an input string.

<!-- page 18 -->

Efficient Matching with Memoization for (la, at)-regexes

107

2
0
MemoExit-laM
A,w (q0 , 0) invokes recursive calls O(|w| ) times—in the same
′

way as Match-(la, at)—because there are no matching failures in A that contribute to memoization.
We also see MemoEnter-la is not correct: Match-(la, at)A,w (q0 , 0) re0
turns SuccessAt(n), but MemoEnter-laM
A,w (q0 , 0) returns SuccessAt(1) because
M (q5 , 1) = false is recorded during the first loop and interpreted as a matching
failure.
In Example 3, a natural solution to the non-linearity issues with MemoExit-la
is to enrich memoization so that it also records previous successes of look-around.
Furthermore, since matching positions do not matter in look-around, the type
of memoization tables should be M : P × N ⇀ {Failure, Success}.
Remark 4. The work [10, Section IX-B] proposes an adaptation of their memoization algorithm to REZWA. Its description in [10, Section IX-B] (to “preserve
the memo functions. . . ”; see Remark 2) consists of the following two points:
1. preserving the memoization tables of the sub-automata throughout the whole
matching, and
2. recording the results of sub-automata matching from different start positions
i of w.
The naive algorithm MemoExit-la we discussed above implements the first
point. We can further add the second point (that is essentially “memoization for
sub-automaton matching”) to MemoExit-la.
However, we find that this is not enough to ensure linear-time complexity.
The problem is that the “memoization for sub-automaton matching” is used too
infrequently. For example, in Example 3, the start positions of sub-automaton
matching are different each time; thus, the memoized results are never used.
Our algorithm (Algorithm 6) resolves this problem by letting the memoization tables (for sub-automaton matching) record results not only for starting
positions but also for non-starting positions.
We also note that there is a gap between the algorithm in the paper [10] and
its prototype implementation [11]; see Remark 3. The latter is linear time but
not always correct. For example, in Example 3, the correct result is SuccessAt(n),
but the prototype [11] returns SuccessAt(1), similarly to MemoEnter-la.
Algorithm 6 is the matching algorithm for la-NFAs that we propose. It adopts
the above extended type of M . In Line 18, Success is recorded in the memoization table when the matching succeeded. This function can return one of
SuccessAt(i′ ), Failure, and Success. We first prove the following lemma to see
that the algorithm indeed solves the partial matching problem (Prob. 2).
Lemma 1. For a la-NFA A = (P, Q, q0 , F, T ), an input string w, and a
′
0
position i ∈ {0, . . . , |w|}, Memo-laM
A,w (q0 , i) returns either SuccessAt(i ) for
i′ ∈ {0, . . . , |w|} or Failure (it does not return Success).
Proof. When we obtain Success as a return value, it must be via an entry M (q, i)
of the memoization table. However, due to Asm. 2, when M (q, i) is set to Success
for a state q of the top-level automaton of A, the matching is already finished
and returns SuccessAt(i′ ).
⊔
⊓

<!-- page 19 -->

108

H. Fujinami and I. Hasuo

Algorithm 6 our partial matching algorithm with memoization for la-NFAs
1: function Memo-laM
A,w (q, i)
Parameters: a la-NFA A, an input string w, and
a memoization table M : P × N ⇀ {Failure, Success}
Input: a current state q, and a current position i
Output: returns SuccessAt(i′ ) if the matching succeeds,
returns Success if a matching success is in M (cf. Lemma 1), or
returns Failure if the matching fails
2:
(P, Q, q0 , F, T ) = A
3:
if M (q, i) ̸= ⊥ then return M (q, i)
4:
result ← ⊥
5:
if q ∈ F then
▷ the same as Lines 5 to 12 of Algorithm 5
..
.
13:
else if T (q) = Sub(pla, A′ , q ′ ) then
14:
(P ′ , Q′ , q0 ′ , F ′ , T ′ ) = A′
′
15:
result ← Memo-laM
A′ ,w (q0 , i)
′
16:
if result = SuccessAt(i ) or Success then
′
17:
result ← Memo-laM
A,w (q , i)
′
18:
if result = SuccessAt(i ) or Success then M (q, i) ← Success
19:
else if result = Failure then M (q, i) ← Failure
20:
return result

As a consequence of the lemma, we can further shrink the memoization tables
in Algorithm 6 by not recording Success for M (q, i) where q is a state of the toplevel automaton.
Algorithm 6 exhibits the desired properties, namely correctness (with respect
to Prob. 2) and linear-time complexity.
Theorem 3 (linear-time complexity of Algorithm 6). For a la-NFA
A = (P, Q, q0 , F, T ), an input string w, and a position i ∈ {0, . . . , |w|},
0
Memo-laM
A,w (q0 , i) terminates with O(|w|) recursive calls.
Theorem 4 (correctness of Algorithm 6).
For a la-NFA A =
(P, Q, q0 , F, T ), an input string w, and a position i ∈ {0, . . . , |w|},
0
Match-(la, at)(q0 , i) = Memo-laM
A,w (q0 , i).
Thm. 3 and 4 can be shown similarly to Thm. 1 and 2; see [15, Appendix B.2].
The in-degree for sub-automata requires some additional care.

5

Memoization for Regexes with Atomic Grouping

We describe our second main technical contribution, namely a backtracking
matching algorithm for at-NFAs with memoization (Algorithm 7). We prove that
it is correct (Thm. 6) and that its time complexity is linear (O(|w|), Thm. 5).
The key ingredient of our algorithm is the type of memoization tables, where
their range is extended from {Failure} to {Failure(j) | j ∈ {0, . . . , ν(A0 )}}; the
latter records a depth j of atomic grouping in order to distinguish failures of different depths. We motivate this extension through two problematic algorithms

<!-- page 20 -->

Efficient Matching with Memoization for (la, at)-regexes

109

MemoExit-at and MemoEnter-at. Much like in §4, MemoExit-at naively
extends Algorithm 5 (Memo) by adding the processing of sub-automaton transitions with at done in Algorithm 2 (Lines 17 to 21), and MemoEnter-la is
similar to MemoExit-at, but records to the memoization table at the same
timing as Algorithm 4 (DavisSLImpl).
Firstly, we observe that MemoExit-at is not linear for a reason similar to
Example 3. (A concrete example is given by Example 4.) Therefore, we turn to
the other candidate, namely MemoEnter-at.
We find, however, that MemoEnter-at is also problematic. It is not correct.
Example 4. Consider the at-NFA A = A(a∗ (?>a∗ )ab) = (P, Q, q0 , F, T ) shown in
Figure 6, and let w = "an b" be an input string. Match-(la, at)A,w (q0 , 0) returns
Failure—the atomic grouping (?>a∗ ) consumes all a’s in w and no a is left for
0
the final ab pattern—but MemoEnter-atM
A,w (q0 , 0) returns SuccessAt(n + 1).
Thus MemoEnter-at is wrong.
For both algorithms, the state q7 in the at transition is first reached at position i = n, and then backtracking is conducted, leading to the state q7 again at
i = n − 1. The execution of MemoEnter-at proceeds as follows.
– The first execution path consumes all a’s in the loop from q0 to q2 , reaches
q7 with i = n, eventually leading to failure at q4 and thus to backtracking.
Speculative memoization (M (q, i) ← false in Algorithm 4) is conducted in
its course; in particular, M (q7 , n) = false is recorded.
– After backtracking, the second execution path reaches q7 with i = n − 1; it
then visits q8 once and reaches q7 with i = n. Now it uses the memoized
value M (q7 , n) = false (cf. Line 4 of Algorithm 4), leading to backtracking
to q7 with i = n − 1. It then takes the branch to q10 , and the matching for A′
succeeds. Therefore, the execution reaches q4 with i = n − 1, and the whole
matching succeeds.
The last example shows the challenge we are facing, namely the need of
distinguishing failures of different depths. Specifically, in the previous example,
the memoized value M (q7 , n) = false comes from the failure of matching for
ambient A; still, it is used to control backtracking in the sub-automaton A′ . This
fact is problematic in an atomic grouping where, roughly speaking, backtracking
in an ambient automaton should not cause backtracking in a sub-automaton.
Atomic grouping can be nested, so we must track at which depth failure has
happened.
Definition 2 (nesting depth of atomic grouping). For an at-NFA A =
(P, Q, q0 , F, T ) and a state q ∈ P , the nesting depth of atomic grouping for q,
denoted by νA (q), is

if q ∈ Q
0
νA (q) =

1 + νA′ (q)



where A′ = (P ′ , Q′ , q0′ , F ′ , T ′ )
s.t. T (q ′ ) = Sub(at, A′ , q ′′ ) and q ∈ P ′ .

<!-- page 21 -->

110

H. Fujinami and I. Hasuo

Algorithm 7 our partial matching algorithm with memoization for at-NFAs
1: function Memo-atM
A0 ,A,w (q, i)
Parameters: an at-NFA A0 , a sub-automaton A of A0 (it can be A0 itself),
an input string w, and a memoization table M : P × N ⇀
{Failure(j) | j ∈ {0, . . . , ν(A0 )}}
Input: a current state q, and a current position i
Output: returns SuccessAt(i′ , K) if the matching succeeds, or
returns Failure(j) if the matching fails
2:
(P, Q, q0 , F, T ) = A
3:
if M (q, i) ̸= ⊥ then return M (q, i)
4:
result ← ⊥
5:
if q ∈ F then result ← Success(i, ∅)
′
6:
else if T (q) = Eps(q ′ ) then result ← Memo-atM
A0 ,A,w (q , i)
7:
else if T (q) = Branch(q ′ , q ′′ ) then
′
8:
result ← Memo-atM
A0 ,A,w (q , i)
9:
if result = Failure(j) and j = νA0 (q) then
′′
10:
result ← Memo-atM
A0 ,A,w (q , i)
′
11:
if result = Failure(j ) then result ← Failure(min(j, j ′ ))
12:
else if T (q) = Char(σ, q ′ ) then
′
13:
if i < |w| and w[i] = σ then result ← Memo-atM
A0 ,A,w (q , i + 1)
14:
else result ← Failure(νA0 (q))
15:
else if T (q) = Sub(at, A′ , q ′ ) then
16:
(P ′ , Q′ , q0 ′ , F ′ , T ′ ) = A′
′
17:
result ← Memo-atM
A0 ,A′ ,w (q0 , i)
18:
if result = SuccessAt(i′ , K ′ ) then
′ ′
19:
result ← Memo-atM
A0 ,A,w (q , i )
′′
′′
20:
if result = SuccessAt(i , K ) then result ← SuccessAt(i′′ , K ′ ∪ K ′′ )
else if result = Failure(j) then
21:
22:
for k ∈ K ′ do M (k) ← Failure(j)
23:
else if result = Failure(j) and j > νA0 (q) then result ← Failure(νA0 (q))
24:
if result = SuccessAt(i′ , K) then result ← SuccessAt(i′ , K ∪ {(q, i)})
25:
else if result = Failure(j) then M (q, i) ← Failure(j)
26:
return result

We also define the maximum nesting depth of atomic grouping for A, denoted
by ν(A), as ν(A) = maxq∈P νA (q).
Algorithm 7 is our algorithm for at-NFAs; the type of its memoization tables
is M : P × N ⇀ {Failure(j) | j ∈ {0, . . . , ν(A)}}. Some remarks are in order.
Note first that the algorithm takes, as its parameters, the whole at-NFA A0
and its sub-automaton A as the algorithm’s current scope. The top-level call is
made with A0 = A (cf. Thm. 5 and 6); when an at transition is encountered,
the scope goes to the corresponding sub-automaton (A′ in Line 17).
In Line 9, the if condition checks that the nesting depth of Failure is the
depth of the current NFA, and backtracking is performed if and only if it is true.
This approach is crucial for avoiding the error in Example 4. The rest of the
cases for Eps, Branch, Char is similar to Algorithm 5.

<!-- page 22 -->

Efficient Matching with Memoization for (la, at)-regexes

111

The case for Sub (Lines 15–23) requires some explanation. It is an adaptation
of Lines 17–21 of Algorithm 5 with memoization. The apparent complication
comes from the set K in SuccessAt(i′ , K). The set K is a set of keys for a
memoization table M , that is, pairs (q, i) of a state and a position. The role of
K is to collect the set of keys of M for which, once failure happens, the entry
Failure(j) has to be recorded (this is done in a batch manner in Line 22). More
specifically, once failure happens in an outer automaton (i.e., at a smaller depth
j), this has to be recorded as Failure(j) for inner automata (at greater depths).
The set K collects those keys for which this has to be done, starting from inner
automata (A′ , Line 18) and going to outer ones (A, Lines 19–20).
A closer inspection reveals that Line 20 is vacuous in Algorithm 7; however,
it is needed when we combine it with look-around at the end of the section.
Algorithm 7 exhibits the desired properties, namely correctness (with respect
to Prob. 2) and linear-time complexity. In Thm. 6, f is a function that converts
results of Algorithm 7 to results of Algorithm 2; it is defined by f (Failure(j)) =
Failure and f (SuccessAt(i′ , K)) = SuccessAt(i′ ).
Theorem 5 (linear-time complexity of Algorithm 7). For an at-NFA
A = (P, Q, q0 , F, T ), an input string w, and an position i ∈ {0, . . . , |w|},
0
Memo-atM
A,A,w (q0 , i) terminates with O(|w|) recursive calls.
Theorem 6 (correctness of Algorithm 7).
For an at-NFA A =
(P, Q, q0 , F, T ), an input string w, and an position i ∈ {0, . . . , |w|},
0
Match-(la, at)(q0 , i) = f (Memo-atM
A,A,w (q0 , i)).
Thm. 5 and 6 are proved similarly to Thm. 1 and 2; see [15, Appendix B.3]. The
following points require some extra care.
Firstly, for linear-time complexity (Thm. 5), there is another recursive call
(Line 19) before the return value of a recursive call (Line 17) is memoized
(Line 22). If the second recursive call (Line 19) eventually leads to (the same
call as) the first call (Line 17) (let’s call this event (∗)), then this can nullify the
effect of memoization. We prove, as a lemma, that (∗) never happens.
Secondly, for correctness (Thm. 6), our conversion of runs should replace
an invocation of the memoization table—if it returns a failure with a shallower
depth—with not only the corresponding run (as before) but also the run of the
second recursive call (Line 19). See [15, Appendix B.3] for details.
Combination with Look-around It is also possible to combine with Algorithm 6 (for look-around) and Algorithm 7 (for atomic grouping). In this
case, the type of memoization tables becomes M : P × N ⇀ {Failure(j) | j ∈
{0, . . . , ν(A)}} ∪ {Success} and nesting depths of the atomic group are reset by
look-around operators. A complete algorithm can be found in [15, Appendix C];
it also exhibits the desired properties.

6

Experiments and Evaluation

Implementation We implemented the algorithm proposed in this paper for
evaluation. We call our implementation memo-regex. It is written in 1368 lines
of Scala.

<!-- page 23 -->

112

H. Fujinami and I. Hasuo

Table 2: our benchmark regexes and input strings
r1

/ˆ(?=ˆ.{1,254}$)(ˆ(?:(?!\.|-)([a-z0-9\-\*]{1,63}|([a-z0-9\-]{1,62}[az0-9]))\.)+(?:[a-z]{2,})$)$/s
input: w1 = "0." "0.0a."n "\u0000", complexity: O(2n )
https://regexlib.com/REDetails.aspx?regexp_id=3494

/(?=(?:[ˆ\']*\'[ˆ\']*\')*(?![ˆ\']*\'))/
r2

input: w2 = "x"n "′ ", complexity: O(n2 )
https://regexlib.com/REDetails.aspx?regexp_id=938

r3

/(?<=[\w\s](?:[\.\!\? ]+[\x20]*[\x22\xBB]*))(?:\s+(?![\x22\xBB](?!\w)
))/
input: w3 = "\”"n "

", complexity: O(n2 )

https://regexlib.com/REDetails.aspx?regexp_id=2355

r4

/(?:(<)\s*?(\w+)(\s*?(?>(?!=[\/\?]?>)(\w+)(?:\s*(=)\s*)((?:\'[ˆ\']*\'
|\"[ˆ\"]*\"|[ˆ >]+))))\s*?([\/\?]?>))/
input: w4 = "<" "aaa"n ">", complexity: O(n2 )
https://regexlib.com/REDetails.aspx?regexp_id=373

memo-regex supports both look-around (i.e., look-ahead and look-behind)
and atomic grouping. We implemented a regex parser ourselves. Backtracking is
implemented by managing a stack manually rather than using a recursive function to prevent stack overflow. In this case, the memoization keys are pushed
onto the stack. Recoding these keys in a memoization table is done during backtracking. We used the mutable HashMap from the Scala standard library as a
data structure for memoization tables.
memo-regex also supports capturing sub-matchings. However, this feature
cannot be used within atomic grouping and positive look-around because submatching information is lost for memoization.
The code of memo-regex, as well as all experiment scripts, is available [14].
Efficiency of Our Algorithm We conducted experiments to assess the performance of our memo-regex, in particular in comparison with other existing
implementations.
As target regexes, we looked for those with look-around and/or atomic grouping in the real-world regexes posted on regexlib.com. We then identified—by
manual inspection—four regexes r1 , . . . , r4 that are subject to potential catastrophic backtracking. These regexes are shown in Table 2. We then crafted input
strings w1 , . . . , w4 , respectively, so that they cause catastrophic backtracking.
Specifically, r1 contains positive look-ahead and negative look-ahead; this positive look-ahead is used for restricting the length of input strings. The regexes r2
and r3 are themselves positive look-ahead and look-behind, respectively; both
include negative look-ahead, too. The regex r4 includes atomic grouping and
negative look-ahead.
For these regexes, we measured matching time using memo-regex on OpenJDK 21.0.1. We compared it with the following implementations: Node.js 20.5.0,

<!-- page 24 -->

Efficient Matching with Memoization for (la, at)-regexes

(a) comparison to others

00

00

00
30

00

00

n

26

00

00
22

00
00
10

(b) memo-regex only

00

100

00
00

00
00

30

00
00

n

26

00
00

22

18

14

00
00
10

n

30

26

22

18

14

10

0

00
00

0.2

150

00

0.4

memo-regex

18

0.6

113

implementation

200

00

1

memo-regex

14

2

time (sec)

time (sec)

memo-regex
js
php
php-nojit
ruby

3

implementation

0.8

implementation

4

memory (MB)

5

(c) memoization table size

Fig. 7: result for r1
3

2
1

memo-regex
js

2
1

(a) r2

0
00

0
00

30

0
00

n

26

0
00

22

18

0

0

00
14

00
10

0
00

0
00

30

0
00

26

0
00

22

0

18

00
14

00
10

n

implementation

2

memo-regex
php
php-nojit

1
0

0
0

0

3

implementation

10
00
0
14
00
0
18
00
0
22
00
0
26
00
0
30
00
0

memo-regex
js
php
php-nojit
ruby

time (sec)

implementation

time (sec)

time (sec)

3

(b) r3

n

(c) r4

Fig. 8: matching time for r2 , r3 and r4
Ruby 3.1.4, and PCRE2 10.42 (used by PHP 8.3.1, w/ or w/o JIT). All of these
implementations use backtracking; Ruby and PCRE2 have restrictions on regexes
inside look-behind and Node.js does not support atomic grouping. The experiments were performed 10 times and the average was adopted. Furthermore,
for memo-regex, we measured the size of its memoization table by the memory
usage, using jamm.8 The experiments were conducted on MacBook Pro 2021
(Apple M1 Pro, RAM: 32 GB).
We show the results in Figures 7 and 8. Note that the values of n are different
depending on whether the matching time complexity is O(n2 ) or O(2n ). Results
for some implementations are absent for r3 and r4 because of the syntactic
restrictions discussed above.
In Figures 7 and 8, we observe clear performance advantages of memo-regex.
In particular, its linear-time complexity and linear memory consumption (memoization table size) are experimentally confirmed.
Real-world Usage of Look-around and Atomic Grouping We additionally surveyed the use of the regex extensions of our interest, in order to confirm
their practical relevance.
We used a regex dataset collected by a 2019 survey [9]. This dataset contains
537,806 regexes collected from the source code of real-world products.
8

https://github.com/jbellis/jamm

<!-- page 25 -->

114

H. Fujinami and I. Hasuo

We tallied the usage of each regex extension by parsing these regexes in the
dataset with our parser in memo-regex. 8,679 regexes could not be parsed or
compiled; this is due to back-reference for 4,360 regexes, unsupported syntax
(Unicode character class, conditional branching, etc.) for 4,134 regexes, and too
large or semantically invalid regexes for the other 184 regexes. We adopted the
remaining 529,127 regexes for tallying.
The result is shown in Table 3. Note that
Table 3: regex ext. usage
1) the numbers for look-ahead and look-behind do not include simple zero-width asser- feature
# of regexes
tions such as ˆ (line-begin) or $ (line-end), (total)
529,127
and 2) that of atomic grouping includes pos- positive look-ahead
7,476 (1.4%)
negative look-ahead
6,917 (1.3%)
sessive quantifiers such as *+ and ++.
1,746 (0.3%)
In Table 3, we observe that 17,167 regexes positive look-behind
negative look-behind
3,750 (0.7%)
(3.2%) in the dataset use at least one of the atomic grouping
1,113 (0.2%)
extensions we studied in this paper. While at least one of the above 17,167 (3.2%)
the ratio is not very large, the absolute number (17,167 regexes) is significant; this implies that there are a number of applications (such as web services) that rely on the regex extensions. Thereby we
confirm the practical relevance of these regex extensions.

7

Conclusions and Future Work

In this paper, we proposed a backtracking algorithm with memoization for
regexes with look-around and atomic grouping. It is the first linear-time backtracking matching algorithm for such regexes. It also fixs problems of the memoization matching algorithm in [10] for look-ahead. We implemented the algorithm; our experimental evaluation confirms its performance advantage.
One direction of future work is to support more extensions. Our implementation does not support a widely used regex extension, namely back-references.
In the recent work [10], back-reference was supported by additionally recording captured positions in memoization tables. We expect that a similar idea is
applicable to our algorithm.
Combination with selective memoization (used in [10]; see [15, Appendix A])
is another direction. We believe it is possible, but it will require a more detailed
discussion on how to handle sub-automata in the selective memoization schema.

Acknowledgments
Thanks are due to Konstantinos Mamouras for pointing to [22] after the dissemination of the preprint version of this paper.

Data-Availability Statement
The data that support the findings of this study are openly available in Zenodo
at 10.5281/zenodo.10458317, reference number [14].

<!-- page 26 -->

Efficient Matching with Memoization for (la, at)-regexes

115

References
1. Aho, A.V.: Algorithms for finding patterns in strings. In: van Leeuwen, J. (ed.)
Handbook of Theoretical Computer Science, Volume A: Algorithms and Complexity, pp. 255–300. Elsevier and MIT Press (1990)
2. Berglund, M., Drewes, F., van der Merwe, B.: Analyzing catastrophic backtracking
behavior in practical regular expression matching. In: Ésik, Z., Fülöp, Z. (eds.) Proceedings 14th International Conference on Automata and Formal Languages, AFL
2014, Szeged, Hungary, May 27-29, 2014. EPTCS, vol. 151, pp. 109–123 (2014).
https://doi.org/10.4204/EPTCS.151.7, https://doi.org/10.4204/EPTCS.151.7
3. Berglund, M., van der Merwe, B., van Litsenborgh, S.: Regular expressions with lookahead. J. Univers. Comput. Sci. 27(4), 324–340 (2021).
https://doi.org/10.3897/jucs.66330, https://doi.org/10.3897/jucs.66330
4. Berglund, M., van der Merwe, B., Watson, B.W., Weideman, N.: On the
semantics of atomic subgroups in practical regular expressions. In: Carayol,
A., Nicaud, C. (eds.) Implementation and Application of Automata - 22nd
International Conference, CIAA 2017, Marne-la-Vallée, France, June 27-30,
2017, Proceedings. Lecture Notes in Computer Science, vol. 10329, pp. 14–
26. Springer (2017). https://doi.org/10.1007/978-3-319-60134-2“˙2, https://doi.
org/10.1007/978-3-319-60134-2\_2
5. Chida, N., Terauchi, T.: On lookaheads in regular expressions with backreferences. In: Felty, A.P. (ed.) 7th International Conference on Formal Structures
for Computation and Deduction, FSCD 2022, August 2-5, 2022, Haifa, Israel.
LIPIcs, vol. 228, pp. 15:1–15:18. Schloss Dagstuhl - Leibniz-Zentrum für Informatik (2022). https://doi.org/10.4230/LIPIcs.FSCD.2022.15, https://doi.org/
10.4230/LIPIcs.FSCD.2022.15
6. Chida, N., Terauchi, T.: Repairing dos vulnerability of real-world regexes.
In: 43rd IEEE Symposium on Security and Privacy, SP 2022, San
Francisco, CA, USA, May 22-26, 2022. pp. 2060–2077. IEEE (2022).
https://doi.org/10.1109/SP46214.2022.9833597,
https://doi.org/10.1109/
SP46214.2022.9833597
7. Cox, R.: Regular expression matching can be simple and fast (but is slow in
java, perl, php, python, ruby,...) (2007), URL: http://swtch.com/rsc/regexp/
regexp1.html
8. Cox, R.: Regular expression matching: the virtual machine approach (2009), URL:
http://swtch.com/˜rsc/regexp/regexp2.html
9. Davis, J.C., IV, L.G.M., Coghlan, C.A., Servant, F., Lee, D.: Why aren’t regular expressions a lingua franca? an empirical study on the re-use and portability of regular expressions. In: Dumas, M., Pfahl, D., Apel, S., Russo, A.
(eds.) Proceedings of the ACM Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of Software Engineering,
ESEC/SIGSOFT FSE 2019, Tallinn, Estonia, August 26-30, 2019. pp. 443–
454. ACM (2019). https://doi.org/10.1145/3338906.3338909, https://doi.org/
10.1145/3338906.3338909
10. Davis, J.C., Servant, F., Lee, D.: Using selective memoization to defeat regular expression denial of service (redos). In: 42nd IEEE Symposium on Security
and Privacy, SP 2021, San Francisco, CA, USA, 24-27 May 2021. pp. 1–17. IEEE
(2021). https://doi.org/10.1109/SP40001.2021.00032, https://doi.org/10.1109/
SP40001.2021.00032

<!-- page 27 -->

116

H. Fujinami and I. Hasuo

11. Davis, J.: PurdueDualityLab/memoized-regex-engine: IEEE S&P2́1 artifact
(Apr 2021). https://doi.org/10.5281/zenodo.4718966, URL: https://zenodo.org/
record/4718966
12. Exchange, S.: Stack exchange network status — outage postmortem - july
20,
2016.
URL:
http://web.archive.org/web/20210308040819/https:
//stackstatus.net/post/147710624694/outage-postmortem-july-20-2016
(2016)
13. Friedl, J.E.F.: Mastering regular expressions - understand your data and be more
productive: for Perl, PHP, Java, .NET, Ruby, and more (3. ed.). O’Reilly (2006),
http://www.oreilly.de/catalog/regex3/index.html
14. Fujinami, H., Hasuo, I.: Artifact Archive for memo-regex: lineartime regex matching implementation with memoization (Jan 2024).
https://doi.org/10.5281/zenodo.10458317
15. Fujinami, H., Hasuo, I.: Efficient matching with memoization for regexes with lookaround and atomic grouping (extended version) (2024), http://arxiv.org/abs/
2401.12639
16. Graham-Cumming, J.: Details of the cloudflare outage on july 2, 2019
(2019), URL: https://web.archive.org/web/20190712160002/https://blog.
cloudflare.com/details-of-the-cloudflare-outage-on-july-2-2019/
17. Herczeg, Z.: Extending the PCRE library with static backtracking based justin-time compilation support. In: Kaeli, D.R., Moseley, T. (eds.) 12th Annual
IEEE/ACM International Symposium on Code Generation and Optimization,
CGO 2014, Orlando, FL, USA, February 15-19, 2014. p. 306. ACM (2014),
https://dl.acm.org/citation.cfm?id=2544146
18. Kirrage, J., Rathnayake, A., Thielecke, H.: Static analysis for regular expression
denial-of-service attacks. In: López, J., Huang, X., Sandhu, R.S. (eds.) Network
and System Security - 7th International Conference, NSS 2013, Madrid, Spain,
June 3-4, 2013. Proceedings. Lecture Notes in Computer Science, vol. 7873, pp.
135–148. Springer (2013). https://doi.org/10.1007/978-3-642-38631-2“˙11, https:
//doi.org/10.1007/978-3-642-38631-2\_11
19. Li, Y., Chen, Z., Cao, J., Xu, Z., Peng, Q., Chen, H., Chen, L., Cheung, S.:
ReDoSHunter: A combined static and dynamic approach for regular expression
dos detection. In: Bailey, M., Greenstadt, R. (eds.) 30th USENIX Security Symposium, USENIX Security 2021, August 11-13, 2021. pp. 3847–3864. USENIX
Association (2021), https://www.usenix.org/conference/usenixsecurity21/
presentation/li-yeting
20. Li, Y., Sun, Y., Xu, Z., Cao, J., Li, Y., Li, R., Chen, H., Cheung, S., Liu, Y.,
Xiao, Y.: RegexScalpel: Regular expression denial of service (redos) defense by
localize-and-fix. In: Butler, K.R.B., Thomas, K. (eds.) 31st USENIX Security
Symposium, USENIX Security 2022, Boston, MA, USA, August 10-12, 2022. pp.
4183–4200. USENIX Association (2022), https://www.usenix.org/conference/
usenixsecurity22/presentation/li-yeting
21. Li, Y., Xu, Z., Cao, J., Chen, H., Ge, T., Cheung, S., Zhao, H.:
FlashRegex: Deducing anti-redos regexes from examples. In: 35th IEEE/ACM
International Conference on Automated Software Engineering, ASE 2020,
Melbourne, Australia, September 21-25, 2020. pp. 659–671. IEEE (2020).
https://doi.org/10.1145/3324884.3416556, https://doi.org/10.1145/3324884.
3416556
22. Mamouras, K., Chattopadhyay, A.: Efficient matching of regular expressions with
lookaround assertions. Proc. ACM Program. Lang. 8(POPL), 2761–2791 (2024).
https://doi.org/10.1145/3632934, https://doi.org/10.1145/3632934

<!-- page 28 -->

Efficient Matching with Memoization for (la, at)-regexes

117

23. McNaughton, R., Yamada, H.: Regular expressions and state graphs
for automata. IRE Trans. Electron. Comput. 9(1), 39–47 (1960).
https://doi.org/10.1109/TEC.1960.5221603,
https://doi.org/10.1109/TEC.
1960.5221603
24. van der Merwe, B., Mouton, J., van Litsenborgh, S., Berglund, M.: Memoized
regular expressions. In: Maneth, S. (ed.) Implementation and Application of Automata - 25th International Conference, CIAA 2021, Virtual Event, July 1922, 2021, Proceedings. Lecture Notes in Computer Science, vol. 12803, pp. 39–
52. Springer (2021). https://doi.org/10.1007/978-3-030-79121-6“˙4, https://doi.
org/10.1007/978-3-030-79121-6\_4
25. Michael, IV, L.G., Donohue, J., Davis, J.C., Lee, D., Servant, F.: Regexes are
hard: Decision-making, difficulties, and risks in programming regular expressions. In: 34th IEEE/ACM International Conference on Automated Software Engineering, ASE 2019, San Diego, CA, USA, November 11-15, 2019. pp. 415–
426. IEEE (2019). https://doi.org/10.1109/ASE.2019.00047, https://doi.org/
10.1109/ASE.2019.00047
26. Miyazaki, T., Minamide, Y.: Derivatives of regular expressions with lookahead.
J. Inf. Process. 27, 422–430 (2019). https://doi.org/10.2197/ipsjjip.27.422, https:
//doi.org/10.2197/ipsjjip.27.422
27. Morihata, A.: Translation of regular expression with lookahead into finite state automaton. Computer Software 29(1), 1 147–1 158 (2012).
https://doi.org/10.11309/jssst.29.1˙147
28. Moseley, D., Nishio, M., Rodriguez, J.P., Saarikivi, O., Toub, S., Veanes, M.,
Wan, T., Xu, E.: Derivative based nonbacktracking real-world regex matching with
backtracking semantics. Proc. ACM Program. Lang. 7(PLDI), 1026–1049 (2023).
https://doi.org/10.1145/3591262, https://doi.org/10.1145/3591262
29. Rathnayake, A., Thielecke, H.: Static analysis for regular expression exponential
runtime via substructural logics. CoRR abs/1405.7058 (2014), http://arxiv.
org/abs/1405.7058
30. Sakuma, Y., Minamide, Y., Voronkov, A.: Translating regular expression matching into transducers. J. Appl. Log. 10(1), 32–51 (2012).
https://doi.org/10.1016/j.jal.2011.11.003,
https://doi.org/10.1016/j.jal.
2011.11.003
31. Shen, Y., Jiang, Y., Xu, C., Yu, P., Ma, X., Lu, J.: ReScue: crafting regular expression dos attacks. In: Huchard, M., Kästner, C., Fraser, G. (eds.) Proceedings of the 33rd ACM/IEEE International Conference on Automated Software Engineering, ASE 2018, Montpellier, France, September 3-7, 2018. pp. 225–
235. ACM (2018). https://doi.org/10.1145/3238147.3238159, https://doi.org/
10.1145/3238147.3238159
32. Sidhu, R.P.S., Prasanna, V.K.: Fast regular expression matching using fpgas. In:
The 9th Annual IEEE Symposium on Field-Programmable Custom Computing
Machines, FCCM 2001, Rohnert Park, California, USA, April 29 - May 2, 2001. pp.
227–238. IEEE Computer Society (2001). https://doi.org/10.1109/FCCM.2001.22,
https://doi.ieeecomputersociety.org/10.1109/FCCM.2001.22
33. Staicu, C., Pradel, M.: Freezing the web: A study of redos vulnerabilities in
javascript-based web servers. In: Enck, W., Felt, A.P. (eds.) 27th USENIX Security Symposium, USENIX Security 2018, Baltimore, MD, USA, August 15-17, 2018.
pp. 361–376. USENIX Association (2018), https://www.usenix.org/conference/
usenixsecurity18/presentation/staicu

<!-- page 29 -->

118

H. Fujinami and I. Hasuo

34. Sugiyama, S., Minamide, Y.: Checking time linearity of regular expression matching based on backtracking. Information and Media Technologies 9(3), 222–232
(2014). https://doi.org/10.11185/imt.9.222
35. Thompson, K.: Regular expression search algorithm. Commun. ACM 11(6), 419–
422 (1968). https://doi.org/10.1145/363347.363387, https://doi.org/10.1145/
363347.363387
36. Weideman, N., van der Merwe, B., Berglund, M., Watson, B.W.: Analyzing matching time behavior of backtracking regular expression matchers by using ambiguity of NFA. In: Han, Y., Salomaa, K. (eds.) Implementation and Application
of Automata - 21st International Conference, CIAA 2016, Seoul, South Korea,
July 19-22, 2016, Proceedings. Lecture Notes in Computer Science, vol. 9705, pp.
322–334. Springer (2016). https://doi.org/10.1007/978-3-319-40946-7“˙27, https:
//doi.org/10.1007/978-3-319-40946-7\_27
37. Wüstholz, V., Olivo, O., Heule, M.J.H., Dillig, I.: Static detection of dos vulnerabilities in programs that use regular expressions. In: Legay, A., Margaria,
T. (eds.) Tools and Algorithms for the Construction and Analysis of Systems 23rd International Conference, TACAS 2017, Held as Part of the European Joint
Conferences on Theory and Practice of Software, ETAPS 2017, Uppsala, Sweden, April 22-29, 2017, Proceedings, Part II. Lecture Notes in Computer Science, vol. 10206, pp. 3–20 (2017). https://doi.org/10.1007/978-3-662-54580-5“˙1,
https://doi.org/10.1007/978-3-662-54580-5\_1

Open Access This chapter is licensed under the terms of the Creative Commons
Attribution 4.0 International License (http://creativecommons.org/licenses/by/4.0/),
which permits use, sharing, adaptation, distribution and reproduction in any medium
or format, as long as you give appropriate credit to the original author(s) and the
source, provide a link to the Creative Commons license and indicate if changes were
made.
The images or other third party material in this chapter are included in the
chapter’s Creative Commons license, unless indicated otherwise in a credit line to the
material. If material is not included in the chapter’s Creative Commons license and
your intended use is not permitted by statutory regulation or exceeds the permitted
use, you will need to obtain permission directly from the copyright holder.
