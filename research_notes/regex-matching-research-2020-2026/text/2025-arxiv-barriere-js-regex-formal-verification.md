---
file: ../pdfs/2025-arxiv-barriere-js-regex-formal-verification.pdf
citation: "Formal Verification for JavaScript Regular Expressions: a Proven Semantics and its Applications (extended version) — Barrière, Deng, Pit-Claudel — arXiv 2507.13091, 2025"
doi: 
source: "https://arxiv.org/pdf/2507.13091"
topics: [counting_and_derivatives, redos_backrefs_verification, semantics_theory_access]
pages: 36
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Formal Verification for JavaScript Regular Expressions:
A Proven Mechanized Semantics and Its Applications
(Extended Version)

arXiv:2507.13091v3 [cs.PL] 7 Apr 2026

AURÈLE BARRIÈRE, EPFL, Switzerland
VICTOR DENG, EPFL, Switzerland and DI ENS, École Normale Supérieure, PSL, CNRS, France
CLÉMENT PIT-CLAUDEL, EPFL, Switzerland
We present the first mechanized, succinct, practical, complete, and proven-faithful semantics for a modern
regular expression language with backtracking semantics. We ensure its faithfulness by proving it equivalent to
a preexisting line-by-line embedding of the official ECMAScript specification of JavaScript regular expressions.
We demonstrate its practicality by presenting two real-world applications. First, a new notion of contextual
equivalence for modern regular expressions, which we use to prove or disprove rewrites drawn from previous
work. Second, the first formal proof of the PikeVM algorithm used in many real-world engines. In contrast
with the specification and other formalization work, our semantics captures not only the top-priority match,
but a full backtracking tree recording all possible matches and their respective priority. All our definitions and
results have been mechanized in the Rocq proof assistant.
CCS Concepts: • Software and its engineering → Semantics; • Theory of computation → Program
verification.
Additional Key Words and Phrases: Regex, Semantics, Rocq, JavaScript, Formal Verification
ACM Reference Format:
Aurèle Barrière, Victor Deng, and Clément Pit-Claudel. 2026. Formal Verification for JavaScript Regular
Expressions: A Proven Mechanized Semantics and Its Applications (Extended Version). 1, 1 (April 2026),
36 pages. https://doi.org/10.1145/nnnnnnn.nnnnnnn

1

Introduction

Despite sharing a name and some features, the modern regular expressions found in many programming languages and libraries are fundamentally different from traditional regular expressions.
For one, with the addition of new features such as backreferences, modern regular expressions
(which we call regexes in the rest of this paper) no longer correspond to regular languages, nor
even to context-free languages [Nogami and Terauchi 2023]. More importantly, with features
such as capture groups, the matching problem changes. It no longer amounts to a recognition
problem (checking that a string belongs to the language of a regular expression), but instead to
a segmentation problem (extracting parts of a string that match individual subexpressions of a
regex). For instance, matching the regex a(b) on string "abc" in JavaScript returns that the whole
regex matched substring "ab", and that the subexpression in parentheses matched substring "b".
From these features arises a new problem: when there are several ways to match a regex, which
substring should be returned? One possible answer is given by the POSIX standard, favoring
the leftmost-longest match of the regex. Another more popular answer is to prioritize the match
that would be found by a backtracking algorithm. Many modern regex languages, such as the
Authors’ Contact Information: Aurèle Barrière, aurele.barriere@epfl.ch, EPFL, Lausanne, Switzerland; Victor Deng, victor.
deng@epfl.ch, EPFL, Lausanne, Switzerland and DI ENS, École Normale Supérieure, PSL, CNRS, Paris, France; Clément
Pit-Claudel, clement.pit-claudel@epfl.ch, EPFL, Lausanne, Switzerland.

© 2026 Copyright held by the owner/author(s). Publication rights licensed to ACM.
ACM XXXX-XXXX/2026/4-ART
https://doi.org/10.1145/nnnnnnn.nnnnnnn

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 2 -->

2

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

ones of Perl, JavaScript, Python, Go, Rust, Java, or .NET follow that disambiguation policy, called
backtracking semantics (sometimes also called leftmost-greedy, or PCRE semantics [Hazel 1997]).
Between the new features and this disambiguation policy, the semantics of these modern regexes
has become surprisingly large and complex. ECMAScript [ECMA 2025], the official JavaScript
standard, dedicates more than 30 pages to the semantics of JavaScript regexes. This specification
is so complex that semantics bugs are still regularly found in regex engines [V8 2023, 2025]; that
the JavaScript implementation used in Firefox gave up on writing a regex engine and uses Google
Chrome’s instead [Ireland 2020]; and in this work we exhibit semantics errors in the optimizer of a
real-world JavaScript regex processor, regexp-tree [Soshnikov 2025].
In this work, we present a new formal semantics for JavaScript regexes, that is mechanized,
succinct, complete, practical, and proven to be correct. Despite decades of modern regexes being
in wide use (including in more than 30% of JavaScript and Python packages [Davis et al. 2018]),
and multiple previous formalization efforts [Chattopadhyay et al. 2025; Chen et al. 2022; Chida
and Terauchi 2023, 2024; De Santo et al. 2024; Loring et al. 2019; Zhuchko et al. 2024], this is the
first time that a semantics enables practical mechanized verification for a real-world modern regex
language. Our semantics is mechanized in the Rocq proof assistant (formerly known as Coq). It is
proven to be faithful to the JavaScript semantics with a verified equivalence to Warblre [De Santo
et al. 2024], an existing line-by-line embedding of the ECMAScript regex specification into Rocq
that is easily auditable but verbose and impractical. It is succinct: the core rules of matching fit
in a single page. It is complete: it supports all regex features of the 14th edition of ECMAScript.
Finally, it is practical for formal verification. We demonstrate this with two applications. First,
we present a new formalized definition of contextual equivalence for regexes with backtracking
semantics. Second, we verify the PikeVM algorithm, which simulates a NFA (non-deterministic finite
automaton). This is the first time that this widely used matching algorithm is formally verified in
full, i.e. proven to return the top priority match according to backtracking semantics. Interestingly,
beyond the usual considerations of mechanized semantics design (convenient induction principles,
strict positive occurrences etc.), we find that a key ingredient for a practical regex backtracking
semantics is to not follow a backtracking algorithm. Instead, we formalize not only the one match
returned by a backtracking algorithm, but instead all possible matches, ordered by priority. We
claim the following novel contributions:
• A new succinct, complete, practical formal semantics, mechanized in Rocq.
• Evidence that this semantics is faithful to the ECMAScript regex standard, by proving it
equivalent to the Warblre embedding [De Santo et al. 2024].
• A new formal definition of equivalence for JavaScript regexes. We use it to prove the
correctness of regex equations from the literature, as well as prove and disprove optimizations
used in a real-world JavaScript regex processor.
• The first formal verification of the PikeVM algorithm, a linear-time regex matching
algorithm used in many real-world regex engines. This is the first time that NFA simulation
with capture groups and backtracking semantics comes with a formal proof.
The theorem relating our semantics model to the official ECMAScript standard makes this the first
formal verification work directly connected to a real-world regex specification.
Section 2 informally introduces JavaScript regexes. Section 3 presents our new inductive semantics and its properties. Section 4 shows its faithfulness by proving it equivalent to the Warblre
embedding. We then present two formally verified applications. First, Section 5 defines our new
regex equivalence and our case studies of correct and incorrect regex rewrites. Second, Section 6
presents our formal verification of the PikeVM algorithm. Finally, Section 7 discusses adapting and
extending our work to other languages and applications, as well as related and future work.
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 3 -->

Formal Verification for JavaScript Regular Expressions

Regular Expressions:
r ::= 𝜀
| cd
| r1 |r2
| r 1 r2
| (g r)
| ⟨r⟩
| a
| \g
| r{min, ∆, ⊤}
| r{min, ∆, ⊥}
| (?lk r)

Empty
Character
Disjunction
Sequence
Capture group
Non-capturing group
Anchor
Backreference
Greedy Quantifier
Lazy Quantifier
Lookaround

3

Character descriptors:
cd ::= c | · | [c0 c1 −c2 ] | [ˆc0 c1 −c2 ] |
\w | \W | \d | \D | \s | \S | . . .
Anchors:
a ::= ^
| \b
Lookarounds:
lk ::= =
| !
| <=
| <!

|
|

$
\B

Input boundary
Word boundary

Positive lookahead
Negative lookahead
Positive lookbehind
Negative lookbehind

Fig. 1. Abstract JavaScript regex syntax

Our Rocq development, consisting of over 15k lines of definitions and proofs, is available as
supplementary material. Appendix A links each individual paper definition to its Rocq counterpart.
Some definitions have been simplified in the paper for presentation purposes.
2
2.1

Background on JavaScript Regexes
Syntax

The abstract syntax of JavaScript regexes we use in this work is summarized on Figure 1, with
some modifications from concrete JavaScript syntax for presentation and convenience purposes.
These differences are as follows. We use 𝜀 instead of no string at all for the empty regex. In concrete
JavaScript syntax, capture groups are pairs of parentheses that are implicitly associated with an
index depending on their position in the regex. Here, we directly annotate each group with its
index, g. For instance, (a(b)) in JavaScript syntax corresponds to (1 a(2 b)). JavaScript also uses
syntactic sugar (?<name>r) (named group) and \k<name> (named backreference) to refer to groups
using names instead of indices. Group names can refer to one group at most, so there is a direct
correspondence from names to indices. To group subexpressions together without defining a
capture group, JavaScript uses non-capturing groups, denoted as (?:r) and for which we use the
notation ⟨r⟩. These non-capturing groups are here to help parsing, and are not reflected in the AST.
Disjunction is right-associative and has the lowest parsing precedence, meaning for instance that
a|ab|abc is exactly the same regex as a|⟨⟨ab⟩|⟨abc⟩⟩.
Quantifiers in concrete JavaScript syntax are written r{min,max} where max is either a natural
number or infinity. This represents any number of repetitions of r between min and max. Since
every valid regex quantifier in JavaScript has min ≤ max, we instead represent quantifier upper
bounds with a Δ representing max-min, simplifying the semantics. Quantifiers can either be greedy
or lazy (the latter by appending a ? in concrete syntax); we represent this with a ⊤ for greedy or
a ⊥ for lazy. JavaScript also defines useful syntactic sugar for some common quantifiers: r ∗ for
r{0, ∞, ⊤}, r ∗? for r{0, ∞, ⊥}, r + for r{1, ∞, ⊤}, r +? for r{1, ∞, ⊥}, r ? for r{0, 1, ⊤}, and r ?? for
r{0, 1, ⊥}.
This syntax covers all regexes from the 14th edition of ECMAScript [ECMA 2023].
2.2

Informal Semantics

We summarize here how JavaScript regexes differ from traditional regular expressions.
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 4 -->

4

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Character descriptors. There are many ways to describe a set of characters in JavaScript regexes:
the following list is not exhaustive. One can write the character directly: any character c matches
itself. Then, · matches all characters (except line terminator characters depending on the s flag,
described below). Character classes match several ranges of characters. For instance [af−z] matches
either a, or any letter in between f and z. They can also be negated, for instance [ˆaf−z] matches
any character that cannot be matched by the previous example. Finally, character class escapes
match common sets of characters, for instance \d matches numerical digits.
Capture groups. Capture groups record the last match of each subexpression inside parentheses.
For instance, matching a(1 ·∗)c on the string "abcd" returns a match on the substring "abc", and
captures the substring "b" between indices 1 and 2 in group 1.
Backreferences. A backreference \g matches the content of capture group g again. For instance,
the regex (1 \d)\1 matches any digit repeated twice, like "33".
Anchors. Anchors enforce a local condition on the current input position, for instance checking
for being at the end of the input (with $), or being at a word boundary (with \b). For instance, a$
does not match anything in "ab" but matches the substring "a" in "ba".
Lookarounds. Lookarounds describe a zero-width condition described as a regex. For instance,
the regex a(?= b) matches any "a" only if it is followed by a "b", but the "b" itself is not part of the
returned substring. Lookarounds can be either positive or negative; negative lookarounds require
that there is no match for the subexpression at the current input position. Positive lookarounds can
define capture groups. For instance, matching a(?= (1 b|c)) on string "ab" matches the substring
"a" and sets capture group 1 to "b". Lookbehinds allow to match the condition backwards. For
instance, matching (?<= Year:)\d+ on string "Year:2026" matches the substring "2026".
Backtracking semantics. As ECMAScript uses a backtracking algorithm to describe their semantics,
JavaScript regexes follow backtracking semantics. As a result, when there are several ways to match
a regex, the leftmost match has priority (for instance, matching a|b on "dba" returns "b"). Then,
the left branch of each disjunction has priority over the right one (for instance, matching a|ab on
"ab" returns "a"). Finally, greedy quantifiers give priority to a match that has iterated as many
times as possible, while lazy ones try to iterate as few times as possible (within what is allowed by
the min and Δ parameters).
Unanchored and anchored matching. The kind of regex matching we have discussed so far,
where a match can start at any position in the input string (but priority is given to the leftmoststarting match), is also known as unanchored matching. However, the problem of unanchored
matching is typically reduced to the problem of anchored matching, where given a regex, an
input string and an input position, one needs to find the top-priority match of the regex that
starts precisely at this position (but may end at any position). In the ECMAScript specification,
unanchored matching is performed by repeatedly trying an anchored matcher at every possible
starting position. In implementations, the top-priority unanchored match of a regex 𝑟 is sometimes
obtained by computing the top-priority anchored match of ⊙ ∗?(0 r), where ⊙ is a regex which
matches any character [Cox 2009]. The laziness of the star ensures we find the leftmost-starting
match, and the capture group with index 0 extracts the start and end positions of that match. As
a result, our semantics focuses on anchored matches; in the rest of this paper, matching refers to
anchored matching.
Top-level API. JavaScript provides several top-level functions to use regexes: we can find an
unanchored match with match, find all matches with matchAll, replace the result of matching in a
string with another substring with replace etc. All of these functions can be implemented using
an anchored matcher and common string manipulation operations.

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 5 -->

Formal Verification for JavaScript Regular Expressions

5

Regex flags. Regex flags can be used to change the behavior of matching. The Unicode flag
u changes the underlying alphabet of characters. Then, the three flags i (case-insensitivity), m
(multiline mode for anchors) and s (make · match line terminators) affect the way some characters
descriptors and anchors are matched. Finally, the other flags affect the behavior of top-level functions
and are not directly relevant to our semantics for anchored matching. The g flag enables global
matching (finding all matches of a regex in a string). With the sticky flag y, matching a regex on a
string starts from the position where a match of that same regex was previously found. When the
flag d is active, an engine should not only return the substring matched by each capture group, but
also the corresponding string indices.
JavaScript semantic peculiarities. While the rules above hold for any regex language with backtracking semantics, JavaScript semantics also differs from other such languages in two ways. First,
to prevent infinite repetitions in regexes like 𝜀 ∗, the Nullable Quantifier property of JavaScript
means that, when min = 0, iterations of a quantifier cannot match the empty string. Other languages
use different criteria to prevent infinite repetitions. This unique property requires adapting standard
matching algorithms (see section 6).
Second, the Capture Reset property means that at each quantifier iteration, the values of capture
groups inside that quantifier are reset. For instance, matching ⟨(1 a)|b⟩ ∗ on the string "ab" matches
the whole string, but the value of capture group 1 is set to None after the match: the regex is
iterated twice, the first iteration defines group 1, but the second iteration resets it.
3
3.1

Tree Semantics
An Inductive Relation for a Backtracking Execution Trace

A key insight of our semantics is to depart from the backtracking algorithm used to specify
JavaScript regex semantics in ECMAScript. In contrast, our semantics is an inductive relation that
connects a regex and a string to an execution trace of a backtracking algorithm that would not stop
at the first accepting result. This execution trace is represented with a tree, where each backtracking
decision (for disjunctions and quantifiers) is represented as a branching node, and other nodes
correspond to some operations of the algorithm, such as reading a character or a backreference,
opening or closing a capture group, or checking the validity of an anchor or lookaround.
Figure 2 showcases the backtracking tree of the regex ⟨a|a(1 b)|a⟩bc on the input "abbc". The
tree starts by branching on two possibilities: either matching the first or the second branch of the
disjunction. On the left, we see the operations corresponding to exploring the first alternative. First,
an "a" is read, then the disjunction has finished matching. We resume by matching a "b", but then
matching fails because in this string it is not possible to read a "c" at the current position. The
other two branches correspond to the deepest disjunction, and the middle branch does find a match
in the string, opening and closing a capture group along the way. Observe that the backtracking
tree includes the full third branch, even though a backtracking algorithm would stop after reaching
the match in the second branch.
The syntax of these backtracking trees is defined in Figure 3. A Mismatch node means that the
algorithm has failed to find a match, for instance because the next character in the input does
not correspond to the character descriptor found in the regex, or because some anchor was not
satisfied, or because a progress check failed, etc. For Choice nodes, the subtrees are ordered by
priority, meaning that the result of a backtracking algorithm corresponds to the leftmost Match
leaf of the corresponding tree. Finally, nodes LK lk tlook t and LKMismatch lk tlook include the entire
tree tlook corresponding to matching the lookaround lk at the current input position.
Advantages of our semantics. The core of our semantics comprises only 21 rules shown in Figure 4,
achieving a convenient succinctness compared to the ECMAScript specification, while providing
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 6 -->

6

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Choice
Read a

Choice

Read b

Read a

Read a

Mismatch

Open 1

Read b

Read b

Mismatch

Close 1
Read b
Read c
Match

Tree:
t ::=
|
|
|
|
|
|
|
|
|
|
|

Fig. 2. Backtracking tree t of the regex
⟨ a |⟨ a(1 b) | a ⟩⟩ bc on input "abbc"

Match
Mismatch
Choice t1 t2
Read c t
BackrefPass s t
Progress t
AnchorPass a t
Open g t
Close g t
Reset gl t
LK lk tlook t
LKMismatch lk tlook

Successful match
Match failure
Branching
Character read success
Backreference success
Progress check success
Anchor success
Group opening
Group closing
Group resetting
Lookaround success
Lookaround failure

Fig. 3. Backtracking Trees

a complete formalization of JavaScript regexes. To match individual characters, we reuse some
definitions from the Warblre mechanization, but reasoning on the core matching algorithm can
be done without unfolding them. The rules are straightforward to mechanize, in particular we
materialize the full tree of lookarounds and scan it for a match, thereby avoiding workarounds for
non-strict positivity that are needed in other work (see Section 7). Our semantics also supports
the relevant regex flags: although we omit them from the paper definitions, our Rocq definitions
are parameterized by the value of the flags i, m and s. To support the Unicode mode (flag u),
our semantics is parameterized by an alphabet of characters, like Warblre. Although we focus
on anchored matching, our semantics could also support the flags used for top-level functions.
Essentially, the flags g and y require changing the inital input position, and we already compute
the group indices required by the d flag.
More importantly, our semantics also formalizes all possible matches of a regex on an input,
not just the top-priority one. Later, we show that this is not only helpful to verify algorithms that
explore more paths than a backtracking algorithm (Section 6), but also to reason about contextual
equivalence in backtracking semantics (Section 5).
Semantics statement. Figure 4 presents the rules of our semantics. This relation naturally provides
an induction principle that we use extensively in our mechanized proofs. We define the big-step
relation (l, i, gm, d) ⇓ t, to mean that t is the backtracking tree for the list of actions l, the input i,
the group map gm and the reading direction d.
With lookbehinds, it is possible to read the input in reverse. As a result, there are two possible
reading directions d: forward (→) and backward (←). To support bidirectional reading, the input
string i is represented with a zipper (a pair of the list of next characters to consume, and the reversed
list of characters already consumed). The relation i2 <d i1 on inputs means that input i1 is the same
as i2 after having read one or more characters following the direction d. We define idx(i) to be the
number of characters already read in i, and next(i)d corresponds to i after reading one character.
The list l represents a stack of remaining actions, each of which can take one of three shapes.
First, regexes are actions: initially for a regex r, the list of actions is simply [r]. Second, Aclose g is
the action that closes the group g. For instance, in Figure 2, after reading the first character 𝑏 in
the middle branch, one needs to first close group 1 before matching the rest of the regex. Third,
Acheck i means that we need to check that the current input has made some progress compared
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 7 -->

Formal Verification for JavaScript Regular Expressions

([], i, gm, d) ⇓ Match

7

(l, i, GMclose (gm, g, idx(i)), d) ⇓ t
Close
(Aclose g :: l, i, gm, d) ⇓ Close g t

Match

icheck <d i
(l, i, gm, d) ⇓ t
Check
(Acheck icheck :: l, i, gm, d) ⇓ Progress t

¬(icheck <d i)
CheckFail
(Acheck icheck :: l, i, gm, d) ⇓ Mismatch

advance(cd, i, d) = Some (c, i′ )
(l, i′, gm, d) ⇓ t

advance(cd, i, d) = None
ReadFail
(cd :: l, i, gm, d) ⇓ Mismatch

Read

(cd :: l, i, gm, d) ⇓ Read c t

(r1 :: l, i, gm, d) ⇓ t1
(r2 :: l, i, gm, d) ⇓ t2
Disj
((r1 |r2 ) :: l, i, gm, d) ⇓ Choice t1 t2
(r1 :: r2 :: l, i, gm, →) ⇓ t
SeqForward
(r1 r2 :: l, i, gm, →) ⇓ t

(r2 :: r1 :: l, i, gm, ←) ⇓ t
SeqBackward
(r1 r2 :: l, i, gm, ←) ⇓ t
(r :: Aclose g :: l, i, GMopen (gm, g, idx(i)), d) ⇓ t
Group
((g r) :: l, i, gm, d) ⇓ Open g t

(l, i, gm, d) ⇓ t
Epsilon
(𝜀 :: l, i, gm, d) ⇓ t
check_anchor(a, i) = ⊤
(l, i, gm, d) ⇓ t
(a :: l, i, gm, d) ⇓ AnchorPass a t

check_anchor(a, i) = ⊥
AnchorFail
(a :: l, i, gm, d) ⇓ Mismatch

Anchor

advance_bkrf (gm, g, i, d) = Some (s, i′ )
(l, i′, gm, d) ⇓ t
(\g :: l, i, gm, d) ⇓ BackrefPass s t

Backref

advance_bkrf (gm, g, i, d) = None
BackrefFail
(\g :: l, i, gm, d) ⇓ Mismatch

(r :: r{min, ∆, p} :: l, i, GMreset (gm, G(r)), d) ⇓ t
Forced
(r{min + 1, ∆, p} :: l, i, gm, d) ⇓ Reset G(r) t

(l, i, gm, d) ⇓ t
Done
(r{0, 0, p} :: l, i, gm, d) ⇓ t

(l, i, gm, d) ⇓ t𝑠𝑘𝑖𝑝
(r :: Acheck i :: r{0, ∆, ⊤} :: l, i, GMreset (gm, G(r)), d) ⇓ titer
Greedy
(r{0, ∆ + 1, ⊤} :: l, i, gm, d) ⇓ Choice (Reset G(r) titer ) t𝑠𝑘𝑖𝑝
(l, i, gm, d) ⇓ t𝑠𝑘𝑖𝑝
(r :: Acheck i :: r{0, ∆, ⊥} :: l, i, GMreset (gm, G(r)), d) ⇓ titer
Lazy
(r{0, ∆ + 1, ⊥} :: l, i, gm, d) ⇓ Choice t𝑠𝑘𝑖𝑝 (Reset G(r) titer )
dir(lk) = d ′
lk_result(lk, tlook , gm, idx(i)) = Some gm′
( [r], i, gm, d ′ ) ⇓ tlook
(l, i, gm′, d) ⇓ t
((?lk r) :: l, i, gm, d) ⇓ LK lk tlook t
dir(lk) = d ′

Lookaround

lk_result(lk, tlook , gm, idx(i)) = None
( [r], i, gm, d ′ ) ⇓ tlook

((?lk r) :: l, i, gm, d) ⇓ LKMismatch lk tlook

LookaroundFail

Fig. 4. Inductive tree semantics
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 8 -->

8

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

to the input i; this is used in quantifier semantics to check that each optional iteration has not
matched the empty string.
The values of all capture groups are stored in a group map, gm, associating to each capture group
index g the optional range that the group last matched. In the initial group map GM ∅ , none of
the groups are defined. The function GMopen (gm, g, 𝑛) updates the group map gm to record that
capture group g is open at position 𝑛. Then, GMclose (gm, g, 𝑛) and GMreset (gm, gl) respectively
close the group g and reset the value of each group in the list gl.
Explanations of rules. We discuss here the most interesting rules of the semantics. To pass a
progress check (Check), the semantics ensures that some progress has been made in the current
input i compared to the input icheck at which the progress check was generated. For character
descriptors (Read), the semantics checks with the function advance(cd, i, d) that the next character
c (for direction d) in the input i can be matched by the character descriptor cd, and returns the
next input zipper i′ . In contrast with traditional regular expressions, matching the sequence r1 r2
does not mean that we can split the input into disjoint parts (one that matches r1 and the other
that matches r2 ): a lookahead in r1 might explore the same part of the input that is matched by
r2 . In SeqForward, we instead match r1 at the current input, and add r2 to the list of actions to
match it afterwards. Handling the sequence also depends on the current direction of matching
(SeqBackward). For anchors (Anchor), check_anchor(a, i) returns ⊤ when the input i satisfies the
anchor a (for instance, when the input is in the final position and the anchor is $), and ⊥ otherwise.
For backreferences (Backref), the function advance_bkrf (gm, g, i, d) looks up the value of group g
in gm and checks that the next characters of i match. When the group g is not defined, matching
the backreference corresponds to reading the empty substring.
The rules for quantifiers come in three shapes. When a quantifier has forced repetitions (min > 0,
Forced), the corresponding tree consists in resetting the groups inside the subregex, then matching
the regex followed by the quantifier where the min has decreased. The function G(r) returns the
list of groups defined in r. When both the min and the Δ of a quantifier are 0, the quantifier is forced
to be skipped (Done). Finally, when min = 0 and Δ > 0, there is a Choice in the tree, between
doing one more iteration or skipping the quantifier. For a greedy quantifier (Greedy), the extra
iteration has higher priority, while for a lazy quantifier (Lazy), skipping has higher priority.
For lookarounds (Lookaround) the semantics considers a tree tlook of the regex inside the
lookaround from the current input position and the direction of lk (→ for lookaheads, ← for
lookbehinds). Then, the function lk_result(lk, tlook , gm, 𝑖) checks that this tree tlook contains an
accepting branch if the lookaround is positive, in which case it returns an updated group map
with the groups defined in the first accepting branch of that tree. If the lookaround is negative, the
function checks that the tree has no accepting branch, and then returns the group map unchanged.

Semantics result. We call accepting branches the branches of a tree that end with a Match. The
leaf of an accepting branch (a final input position and a final group map), is obtained by replaying
the group operations in the branch. The result of matching a regex on a string corresponds to the
leaf of the first accepting branch in the corresponding tree. When there is no such accepting branch,
there is no match for the regex on the string. We define the function L0 (t, i) to return either the
first leaf of t for input i, or None.
In Figure 2, L0 (t, Input[a; b; b; c] []) = Some (Input[] [c; b; b; a], GMclose (GMopen (GM ∅ , 1, 1), 1, 2)),
where Input 𝑙 1 𝑙 2 denotes an input zipper, with 𝑙 1 being the list of next characters and 𝑙 2 the list of
previously seen characters.

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 9 -->

Formal Verification for JavaScript Regular Expressions

3.2

9

Semantics Properties

In this section, we show that for any list of actions, input, group map and direction, there exists a
unique backtracking tree. We first prove that our semantics is deterministic.
Theorem 1 (Determinism). ∀ l, i, gm, d, t1, t2 . (l, i, gm, d) ⇓ t1 ∧ (l, i, gm, d) ⇓ t2 =⇒ t1 = t2 .
Proof. By induction over a derivation of (l, i, gm, d) ⇓ t1 .

□

We then prove its productivity. To do so, we define a function that computes a backtracking
tree, and show that this tree respects the relation defined in Figure 4. However, the termination
of this function is not straightforward: the termination of quantifiers relies on the fact that each
non-forced iteration must consume at least one character, and that the input itself is finite. This is
made even harder with lookarounds allowing to change the direction of matching.
We first define a function T (l, i, gm, d, 𝑛) with a fuel 𝑛, allowing only 𝑛 recursive calls to ensure
termination, but which may return None when not provided with enough fuel. Next, we define a
function ||l|| id which computes an upper bound on the minimal amount of recursive calls needed
for the function to terminate. The fuel of a list of actions l and a regex r, respectively ||l|| id (left)
and ||r || id (right), is defined as follows:
Δ

Δ

||𝜀 || id = ||cd|| id = 1

Δ

||a|| id = ||\g|| id = 1

|| [] || id = 1
||r :: l|| id = ||r || id + ||l|| id
Δ

||Aclose g :: l|| id = 1 + ||l|| id
Δ

||Acheck icheck :: l|| id = 0
(when icheck is at the end for d)
Δ

Δ
Δ

||r1 |r2 || id = ||r1 r2 || id = 1 + ||r1 || id + ||r2 || id
Δ

||(g r)|| id = 2 + ||r || id
Δ

||(?lk r)|| id = 2 + ||r || worst(lk,i)
dir(lk)

Δ
check ) d
||Acheck icheck :: l|| id = 1 + ||l|| next(i
||r{min, ∆, p}|| id = (2 + ||r || id ) × (1 + min + |i| d )
d
(otherwise)
There are three interesting cases. For quantifiers, we know that each non-forced repetition has to
make progress. As a result, there can be at most 1 + 𝑚𝑖𝑛 + |i| d iterations of the quantifiers, where
|i| d is the number of characters left to be read in the string for direction d. For lookarounds, we
cannot know ahead-of-time where they will be matched from, so we define worst(lk, i) to be the
worst possible input for the lookaround direction (at the beginning for a lookahead, at the very
end for a lookbehind). For Acheck icheck actions, we know that if the input being compared to is at
the end position (for instance, when the current direction is → and the input zipper is of the form
Input [] 𝑙 2 ), then the progress check cannot succeed. And otherwise, we know that in the worst
case, the input after passing the check will be next(icheck )d , after having read exactly one character.
With these definitions, we can prove that this fuel computation is indeed an upper bound on the
number of recursive calls sufficient to finish the computation of the tree.

Theorem 2 (Termination). ∀ l, i, gm, d, 𝑛. 𝑛 > ||l|| id =⇒ T (l, i, gm, d, 𝑛) ≠ None.
Proof. By induction over 𝑛. In the inductive case, we prove that each recursive call strictly
decreases the fuel computation.
□
From here on we omit the fuel argument. Finally, we prove that the tree we compute is a correct
backtracking tree for its arguments. Combined with the determinism of Theorem 1, this shows that
for any list of actions, input, and direction, there exists a unique backtracking tree.
Theorem 3 (Functional Semantics Correctness). ∀ l, i, gm, d. (l, i, gm, d) ⇓ T (l, i, gm, d).
Proof. By induction over the number of recursive calls.

□

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 10 -->

10

4

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Semantics Faithfulness: Equivalence to Warblre

In this section, we prove that our tree semantics is faithful to the 14th edition of the ECMAScript specification [ECMA 2023]. To do so, we provide a Rocq proof that it is equivalent to Warblre [De Santo
et al. 2024], a faithful, manually audited, line-by-line translation of the ECMAScript regex chapter
into Rocq. The ECMAScript regex chapter consists of more than 30 pages of a pseudocode algorithm.
The main function of that algorithm, CompilePattern (that we name compile(r) for short), compiles
a regex into a pseudocode matcher function that implements a backtracking search. This matcher
function takes as input a string and a starting index position, and returns a match result, which
either indicates that no match was found at that starting position, or otherwise returns the final
string position and the set of capture group values. We prove that the first accepting branch of our
backtracking tree semantics corresponds to the result of the Warblre matcher function.
Regex equivalence. First, to facilitate proofs and provide concise rules, the regex type we use in
our semantics differs slightly from the AST used in ECMAScript and Warblre. While our quantifiers
use convenient min and Δ parameters, in Warblre each quantifier has a min and a max parameter,
which corresponds to min + Δ. At first glance, our type appears less expressive, but ECMAScript
starts compilation by checking that the regex is well-formed, which includes that in all quantifiers,
max ≥ min. We denote by W the set of well-formed Warblre regexes (those that pass Early Errors
in ECMAScript parlance). We define a function, ⇃r𝑤⇂ translating a well-formed regex r𝑤 ∈ W
from the Warblre AST into ours. Another interesting difference is the indexing of capture groups.
In Warblre, capture groups have no indices in the AST. Instead, at each manipulation of a group,
its index is recomputed by counting the number of open parentheses that occurred before in the
original regex. This process requires carrying around a cumbersome compilation context in Warblre
definitions. To facilitate definitions and proofs instead, our ⇃r𝑤⇂ function annotates in the AST the
index of each group. Similarly, it translates every named group or named backreference into the
corresponding indexed group and indexed backreference. Finally, there are other minor differences
in the way characters are represented. For instance, to facilitate formal proofs, we use the same
type to represent both single characters and Unicode or identity escapes.
Finally, we can prove the equivalence below (Theorem 4). The Warblre definitions use strings
(list of characters) and natural numbers to encode position. Our zipper inputs are more convenient
to express the tree semantics, and we note str(i) the original string of the zipper input i. Finally, as
our group map type slightly differs from the Warblre definition, we define a function ↿res↾ that
transforms our result into a Warblre one.
Theorem 4 (Faithfulness). ∀ i, r𝑤 ∈ W,
compile(r𝑤 ) (str(i), idx(i)) = ↿L0 (T ( [⇃rw⇂], i, GM ∅ , →), i)↾
Proof. We summarize a few key insights of this considerable proof (over 6000 lines of Rocq). In
order to define the function compile(r), ECMAScript generates matcher continuations, functions
that can match a regex in a particular direction, open and close a capture group, or check for
progress in the string. Our proof starts by defining an equivalence relation, parameterized by a
direction, between these continuations and our lists of actions. We prove an intermediate theorem:
the matcher continuation of a regex r𝑤 and a direction d is equivalent, for d, to the list of actions
[⇃r𝑤⇂]. Theorem 4 then follows as a corollary.
The proof of that intermediate theorem proceeds by induction. For quantifiers, we then proceed
by induction on their maximum number of iterations (the sum of min and the size of the remaining
string for the current direction). Groups are also handled differently in the two semantics. In our
semantics, we generate an Open node before matching the subregex, but in ECMAScript, the
corresponding matcher continuation remembers the initial input position, matches the regex, then
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 11 -->

Formal Verification for JavaScript Regular Expressions

11

opens and closes the group at the same time. As a result, we prove as an invariant that whenever our
group maps have partially defined groups (opened but not yet closed), the corresponding matcher
continuations in Warblre will later open the same groups at the same index.
□
5

Formally Verified Regex Equivalence

To demonstrate the practicality of our semantics, we use it to define a new notion of contextual
regex equivalence for backtracking semantics. We use it to prove and disprove some seemingly
intuitive equivalences, and even highlight bugs in a real-world JavaScript regex processor.
Reasoning about modern regex equivalence has many real-world uses. Derivative-based engines
compute regex equivalence classes among derivatives during matching [Moseley et al. 2023; Owens
et al. 2009; Varatalu et al. 2025]. Regex-optimization libraries (like regexp-tree [Soshnikov 2025]
for JavaScript regexes) rewrite regexes into equivalent, optimized regexes that are expected to be
faster to match. The topic of traditional regular expression equivalence is well studied, even in
proof assistants [Asperti 2012; Coquand and Siles 2011; Krauss and Nipkow 2012; Moreira et al.
2012; Nipkow and Traytel 2014], but these definitions are not applicable to modern regexes with
backtracking semantics and nontraditional features. For these, the literature is much more sparse.
Freydenberger [2013] has shown that backreferences make regex equivalence undecidable. Recent
work has also established a few results for regex languages with restricted feature sets. For instance,
Zhuchko et al. [2024] show that some anchors can be expressed in terms of lookarounds and
Mamouras and Chattopadhyay [2024] establish regex equivalence formulae about lookarounds.
However, even these new equivalences may not hold for a full real-world language with backtracking semantics. For instance, we found that some lookaround equivalences from Mamouras
and Chattopadhyay [2024], while correct in a language without backreferences, do not hold in
JavaScript. This is because lookarounds do not commute: the regex (?= (1 a)) (?= a\1)a does not
match the string "a", as the first lookaround sets group 1 to "a", but then the second one fails
to read "a" twice. In contrast, (?= a\1) (?= (1 a))a does match the string "a": since group 1 is not
defined yet, the backreference matches the empty string.
We present a new way to formally verify regex equivalence using our backtracking tree semantics.
To facilitate comparison to previous work, we restrict this discussion to the default JavaScript
semantics, leaving out the i (ignoreCase), m (multiline), and s (dotAll) flags.
5.1

A New Definition of Regex Equivalence

Observational equivalence and its limitations. The most natural way to define regex equivalence
is to say that r1 and r2 are equivalent when for any input i, matching r1 on i returns the same
result as matching r2 on i. We refer to this as observational equivalence, and denote it as r1 ≈ r2 .
While observational equivalence is enough to replace the matching of a regex with another, it is
not strong enough to be used locally, to replace a subregex within a bigger context.
For instance, consider the regexes 𝜀 |a and 𝜀 |b. These two regexes are observationally equivalent:
on any input, they will both match the empty substring since this corresponds to the match with
the highest priority. However, we cannot replace 𝜀 |a with 𝜀 |b in a bigger context and preserve
equivalence. For instance, c⟨𝜀 |a⟩c matches the input "cac", but c⟨𝜀 |b⟩c does not.
Directional contextual equivalence. Instead, we need a notion of equivalence that allows local
rewriting of regexes. We refer to this as contextual equivalence, and use the notation r1 ∼ r2 . We
would like this definition to imply observational equivalence in a bigger context: r1 ∼ r2 =⇒
∀ C. C [r1 ] ≈ C [r2 ], where C is a regex context (a regex with a hole).
But in fact, we show that we can define a more precise equivalence with the following novel
observation: some regex equivalences only hold in a given matching direction. As a result, in the
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 12 -->

12

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

rest of this section we define two notions of equivalence, r1 ∼→ r2 and r1 ∼← r2 , for equivalences
that can be rewritten inside different contexts.
→
−
We distinguish three types of contexts. Forward contexts, denoted as C , have their hole inside a
lookahead, meaning that we know the regex will be matched in the forward direction. Backwards
←
−
contexts, denoted as C , have their hole inside a lookbehind. In cases where lookarounds are nested,
the deepest lookaround sets the direction of the context. Finally, bidirectional contexts, denoted as
C =, are contexts in which the hole does not appear in a lookaround.
Using leaves to define contextual equivalence. We now show that leaves of backtracking trees of
regexes are sufficient to define this directional contextual equivalence. Crucially, the top priority
match of c⟨𝜀 |a⟩c on "cac" (our previous example) uses the second priority match of subexpression
𝜀 |a. To define a contextual equivalence, we must reason on several possible matches of a regex,
which our tree semantics was designed to formalize. We first define L (t, i, d) to return the orderpreserving list of leaves (pairs of a final input and a final group map at a Match node) of the
tree t for input i and direction d. We say that two lists of leaves are equivalent, noted leaves1 ≡
leaves2 , when they are equal after removing lower-priority duplicates in each list. For instance,
[(i1, gm1 ); (i2, gm2 ); (i1, gm1 )] ≡ [(i1, gm1 ); (i2, gm2 )]. Finally, we define directional contextual
equivalence as follows:
Δ
r1 ∼d r2 = G(r1 ) = G(r2 ) ∧ ∀ i, gm. L (T ( [r1 ], i, gm, d), i, d) ≡ L (T ( [r2 ], i, gm, d), i, d)
We require the two regexes to define exactly the same groups, so that replacing r1 with r2 will
not turn a well-formed regex into an ill-formed one with duplicated groups. We use notation
r1 ∼↔ r2 for equivalence in both directions. Removing duplicates in the list of leaves allows to
prove equivalences like c|c ∼↔ c : even though the first regex has more leaves, any result that can
be obtained from the second branch could also be obtained from the first one.
Using these definitions, we were able to prove the following theorems:
Theorem 5 (Bidirectional Eqivalence). ∀ r1, r2, d. r1 ∼d r2 =⇒ ∀C = . C = [r1 ] ∼d C = [r2 ]
→
−
→
−
→
−
Theorem 6 (Forward Eqivalence). ∀ r1, r2 . r1 ∼→ r2 =⇒ ∀ C , d. C [r1 ] ∼d C [r2 ]
←
−
←
−
←
−
Theorem 7 (Backward Eqivalence). ∀ r1, r2 . r1 ∼← r2 =⇒ ∀ C , d. C [r1 ] ∼d C [r2 ]
Theorem 8 (Contextual to Observational Eqivalence). ∀ r1, r2 . r1 ∼→ r2 =⇒ r1 ≈ r2
Proof. Most proofs proceed by induction over the context. In the case of quantifiers for the
bidirectional equivalence, we proceed by induction over the length of the input remaining to match
(giving an upper-bound for the number of remaining iterations).
□
5.2

Formally Verified and Invalid Equivalences

Using these theorems, we can now locally rewrite subregexes, using the forward equivalence
when inside a lookahead or outside of lookarounds, and the backward equivalence when inside
a lookbehind. As a case study, we have proved or disproved rewrites from the literature. We
consider three sets of regex rewrites: rewriting anchors as lookarounds, quantifier merging, and
the traditional associativity and distributivity of sequence and disjunction.
Associativity and distributivity. The regex equivalences we have proved correct are shown on
Figure 5. The missing directions for distributivity are not shown because they are not correct: we
provide counter-examples in Appendix B and Figure 6. We prove associativity for both the sequence and the disjunction. These proofs illustrate that our semantics eliminates some unnecessary
complexity from the Warblre definitions. Consider for instance the associativity of disjunction. In
Warblre, we would need to prove that even though r2 is compiled in different contexts, these two
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 13 -->

Formal Verification for JavaScript Regular Expressions

13

Associativity

Anchors as lookarounds

r1 |⟨r2 |r3 ⟩ ∼↔ ⟨r1 |r2 ⟩|r3
r1 ⟨r2 r3 ⟩ ∼↔ ⟨r1 r2 ⟩r3

^ ∼↔ (?<! ⊙)
$ ∼↔ (?! ⊙)
\b ∼↔ (?<! \w) (?= \w)|(?<= \w) (?! \w)
\B ∼↔ ⟨(?<= \w)|(?! \w)⟩⟨(?<! \w)|(?= \w)⟩

Distributivity (when r1 has no group)
r1 ⟨r2 |r3 ⟩ ∼← ⟨r1 r2 ⟩|⟨r1 r3 ⟩
⟨r2 |r3 ⟩r1 ∼→ ⟨r2 r1 ⟩|⟨r3 r1 ⟩

Fig. 5. Correct regex equivalences — associativity, distributivity and anchors

Choice
Read a
Read a
Choice
Mismatch Read b
Match

Read b
Choice
Read c Mismatch
Match

Choice
Choice Choice
Read a Read a
MismatchRead b
Read c

Read a Read a
Read b Read b
Match Mismatch

Match

Fig. 6. Backtracking trees of ⟨ a|ab ⟩⟨ c | b ⟩ and ⟨ a|ab ⟩ c |⟨ a|ab ⟩ b on string "abc"

contexts are equivalent in the sense that they do not change the ordering of capture groups, requiring a new general property for matcher functions compiled in equivalent contexts. In contrast, with
our equivalence definition, this associativity boils down to the associativity of the concatenation of
lists of leaves, and the proof concludes in five lines. We also find that contrary to traditional regular
expressions, concatenation and disjunction can only distribute under certain conditions. First, the
distributed regex must not define any groups, to preserve the property that each capture group is
defined at most once in JavaScript regexes. Second, depending on the context direction, distributing
can change the top priority result. For instance, in Figure 6, we can see that distributing a|ab over
c|b changes which disjunction is considered first in their respective backtracking trees, as a result
the two middle branches are inverted and the regexes are not equivalent.
Anchors as lookarounds. We also proved that anchors can be rewritten as lookarounds, where
⊙ is a regex that matches all characters and \w matches all word characters. These equivalences
had been formulated by Varatalu et al. [2023] and the first two had been mechanized in Lean
by Zhuchko et al. [2024], but for a regex language without backtracking semantics. Now, using our
semantics and equivalence definitions, we can prove that these rewrites hold for JavaScript regexes
as well.
Quantifier merging. Finally, the JavaScript regex processor library regexp-tree [Soshnikov 2025]
optimizes repeated quantifiers in a regex. It replaces every instance of r{min1, ∆1, p}r{min2, ∆2, p}
Δ
Δ
Δ
with r{min1 + min2, ∆1 + ∆2, p}, where + is defined such that 𝑛 + ∞ = ∞ + 𝑛 = ∞ + ∞ = ∞. While
trying to prove the correctness of this equivalence, we have found counter-examples. But we also
found that, when r defines no group, depending on the min and Δ parameters, their order, and
the direction of the context, it can be correct to perform the optimization. First, we prove the
following equivalence, allowing to change the greediness of a quantifier with only forced iterations:
r{min, 0, ⊤} ∼↔ r{min, 0, ⊥}. Then, we prove the eight correct equivalences shown on Figure 7.
We give counter-examples for incorrect ones in Appendix B. The ✓ and ✗ labels indicate correct
and incorrect rewrite directions, and n/a indicates that merging is not applicable since it mixes
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 14 -->

14

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

non-forced repetitions of different priorities. Since regexp-tree performed all rewrites in both
directions, all ✗ marks are regexp-tree bugs. We have reported the issue to the regexp-tree
maintainers.1
2nd quantifier
1st quantifier

r{min2, 0, p}

r{0, ∆2, ⊤}

r{0, ∆2, ⊥}

↔✓
←✓ →✗
←✓ →✗

→✓ ←✗
↔✓
n/a

→✓ ←✗
n/a
↔✗

r{min1, 0, p}
r{0, ∆1, ⊤}
r{0, ∆1, ⊥}

Fig. 7. Correct and incorrect regex quantifier merging equivalences

To provide an insight into these equivalence proofs, consider for instance proving one direction
of the middle case of Figure 7: r{0, ∆1, ⊤}r{0, ∆2, ⊤} ∼→ r{0, ∆1 + ∆2, ⊤}. In the case where both
Δ1 and Δ2 are natural numbers, this is proved by induction on Δ1 . The simplified trees of each
regex are shown on Figure 8. By induction hypothesis, the two framed subtrees have equivalent
leaves. However, when Δ2 > 0, the tree on the left has an extra subtree when the first quantifier is
skipped, but the second one iterates. Since Δ2 − 1 ≤ Δ1 + Δ2 , we can prove that each leaf of this
extra subtree is a leaf of the subtree of r{0, ∆1 + ∆2, ⊤}, and then by induction hypothesis that this
is a leaf of r{0, ∆1, ⊤}r{0, ∆2, ⊤} as well. As a result, all leaves of this extra subtree are duplicates
of the leaves of the leftmost subtree, and the leaf equivalence between the two regexes hold.
r{0, ∆1 + 1, ⊤}r{0, ∆2, ⊤}
Choice skip

r{0, ∆1 + ∆2 + 1, ⊤}
Choice skip

iterate

r{0, ∆1, ⊤}r{0, ∆2, ⊤}
iterate
extra subtree:

r{0, ∆2, ⊤}
Choice

r{0, ∆2 − 1, ⊤}

iterate

∼→

r{0, ∆1 + ∆2, ⊤}

Match

skip

Match

Fig. 8. Inductive case when proving r{0, ∆1, ⊤}r{0, ∆2, ⊤} ∼→ r{0, ∆1 + ∆2, ⊤}

These rewrites can be combined. For instance, the following chain of forward equivalences holds:
r{min0, 0, ⊥}r{min1, ∆1, ⊤}r{0, ∆2, ⊤} ∼→ r{min0, 0, ⊤}r{min1, 0, ⊤}r{0, ∆1, ⊤}r{0, ∆2, ⊤}
∼→ r{min0 + min1, 0, ⊤}r{0, ∆1 + ∆2, ⊤}
∼→ r{min0 + min1, ∆1 + ∆2, ⊤}
For each invalid rewrite, we conjecture that restrictions either on the subregex r or on the
contexts could restore correctness. For traditional regular expressions, previous work explored
transformations valid within an optional repetition or a quantifier [Kahrs and Runciman 2022].
6

Formal Verification of the PikeVM Matching Algorithm

To implement modern backtracking semantics, most engines use a backtracking algorithm. For
instance, both V8 and SpiderMonkey (the JavaScript implementations of Google Chrome and
Firefox, respectively) use Irregexp, a backtracking regex engine. However, bactracking algorithms
suffer from string-size exponential complexity, leading to the REgex Denial Of Service (ReDoS)
vulnerability, a complexity attack harming many real-world programs [Staicu and Pradel 2018].
To sidestep this complexity vulnerability, some modern engines instead use a more restrictive
flavor of regexes and implement linear-time matching algorithms. For instance, by removing
1 https://github.com/DmitrySoshnikov/regexp-tree/issues/267

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 15 -->

Formal Verification for JavaScript Regular Expressions

15

instr
r

::=
|
|
|
|
|
|

𝜀
cd
r1 |r2
r 1 r2
(g r)
r∗
r ∗?

Empty
Character
Disjunction
Sequence
Capture group
Greedy star
Lazy star

Fig. 9. The subset of regexes, P,
supported by the PikeVM algorithm

::=
|
|
|
|
|
|
|
|

Accept
Consume cd
Jump l
Fork l1 l2
RegOpen g
RegClose g
ResetRegs gl
BeginLoop
EndLoop l

Fig. 10. NFA bytecode instructions

backreferences (which make the matching problem NP-hard [Dominus 2000]) and lookarounds,
libraries like RE2 [Google 2010], HyperScan [Wang et al. 2019], the Rust Regex crate [Gallant
2014] or the Go regexp package [Go 2025] all achieve linear-time guarantees. For languages with
non-linear features like .NET [Moseley et al. 2023] or JavaScript [V8 2021], implementers sometimes
provide both a complete backtracking engine and a restricted linear engine for a subset of regexes.
The PikeVM algorithm [Cox 2009] is a popular linear-time matching algorithm for modern
regexes with backtracking semantics. This algorithm is a descendant of NFA simulation [Thompson
1968], where the NFA is extended to encode priority and is represented as interpreted bytecode.
With its encoding of priority, the PikeVM algorithm supports capture groups, and greedy and lazy
quantifiers, and returns the same result as a backtracking algorithm but without the exponential
complexity. In the examples above, all the linear engines supporting capture groups implement a
PikeVM (among other linear algorithms): the RE2 library2 , the Rust Regex crate3 , the Go engine4
and the linear engine of V8 for JavaScript regexes.5
However, the execution of the PikeVM algorithm is vastly different from the execution of a
backtracking algorithm. It explores several paths in parallel, and discards some paths to avoid
the exponential complexity. As a result, a PikeVM can be challenging to implement. Worse: small
variations in semantics can affect the correctness of the base algorithm, and Barrière and Pit-Claudel
[2024] have shown that the PikeVM implementation deployed in V8 used to contain a semantic bug.
While traditional NFA simulation itself has been formalized and verified many times (see Section 7),
handling priority requires new formal arguments, and to the best of our knowledge the PikeVM
has never been formally verified despite its widespread use in modern engines.
In this section, we provide the first formal verification of the PikeVM algorithm. The proof has
been fully mechanized in the Rocq proof assistant. Using an inductive backtracking tree semantics
greatly facilitates the proof, as several crucial properties can be conveniently expressed on trees.
We prove that the algorithm returns the result specified by the ECMAScript standard, making it the
first time a modern regex matching algorithm is formally verified against a real-world specification.
6.1 The PikeVM Algorithm
In this section, we present a formal model of the PikeVM algorithm, first introduced by Rob Pike in
the text editor sam [Pike 1987]. We formalize and prove a version of the PikeVM that slightly differs
from the original algorithm to account for JavaScript-specific semantics: we generate instructions
2 https://github.com/google/re2/blob/main/re2/nfa.cc
3 https://github.com/rust-lang/regex/blob/master/regex-automata/src/nfa/thompson/pikevm.rs
4 https://cs.opensource.google/go/go/+/master:src/regexp/exec.go
5 https://github.com/v8/v8/tree/main/src/regexp/experimental

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 16 -->

16

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

for capture reset, and we use the extension described in Barrière and Pit-Claudel [2024] to support
JavaScript nullable quantifiers. Both of these extensions have been used in the PikeVM engine of
V8 (we discuss adapting these definitions for other languages in Section 7). This algorithm supports
a subset of JavaScript regexes, P, depicted on Figure 9. This corresponds to the traditional features
of regular expressions, extended with capture groups, and with the star being either greedy or lazy.
We extend this subset to lists of actions (all Acheck and Aclose actions belong to the subset). We
also extend this subset to trees (rejecting trees containing nodes corresponding to features that are
not in P, like BackrefPass or LK).
Regex r
𝜀
cd
r1 r2

Label

r1 |r2
l1 :
l2 :
l3 :
(g r)

r∗

r ∗?

l𝑠 :
l𝑖𝑛 :

l𝑜𝑢𝑡 :
l𝑠 :
l𝑖𝑛 :

NFA(r)
no instruction
Consume cd
NFA(r1 )
NFA(r2 )
Fork l1 l2
NFA(r1 )
Jump l3
NFA(r2 )
...
RegOpen g
NFA(r)
RegClose g
Fork l𝑖𝑛 l𝑜𝑢𝑡
BeginLoop
ResetRegs G(r)
NFA(r)
EndLoop l𝑠
...
Fork l𝑜𝑢𝑡 l𝑖𝑛
BeginLoop
ResetRegs G(r)
NFA(r)
EndLoop l𝑠
...

Compilation. The first step of the PikeVM algorithm consists in computing a bytecode representing the NFA of the
regex. An NFA is encoded as a list of bytecode instructions.
Each bytecode instruction corresponds to a state of the
NFA; these instructions are represented on Figure 10. All
instructions are labeled with a natural number l indicating
their position in the list. Edges of the NFA are represented
in two ways: either the instructions directly contain the
labels of their successor states, or there is an implicit edge
between each instruction without a label and the following instruction in the list. The NFA bytecode compilation
function, shown on Figure 11, is an extension of the traditional Thompson NFA construction [Thompson 1968]. The
recursive compilation function, NFA(r), transforms a regex
into a list of bytecode instructions. The dots “. . . ” indicate
a fresh label, to be used for the next instruction. At the end
of compilation, an Accept instruction (the accepting state
of the NFA) is appended to the end of the list of instructions. To encode priority, the labels in the Fork instruction
are ordered: the first label corresponds to the top priority
branch to explore. As a result, the compilation of r ∗ and
r ∗? differ in their first instruction, the greedy star giving
more priority to doing one more iteration, and the lazy one
giving more priority to exiting.

Execution. After the regex is compiled to bytecode,
PikeVM interprets this bytecode on a string of characters.
The algorithm reads each character of the string one at a
Fig. 11. Compiling a regex to its
time. In between each character, it builds a list of possible
bytecode extended NFA
incomplete paths (ordered by priority) in the bytecode NFA.
To explore different parts of the bytecode simultaneously, PikeVM keeps track of several threads. In a thread
(pc, gm, b), pc is the current label, gm is a group map, and b is a Boolean indicating whether the
thread has consumed a character since entering its last quantifier. In this section, we describe
the execution algorithm with small-step semantics, shown on Figure 12.6 States of the PikeVM
algorithm are either of the form VMend (best) or tuples (i, best, A, B, S). In these states, i is the
current input (although PikeVM explores several threads in parallel, they are all synchronized
at the same position in the original string). best is the top priority result found so far if any (a
l𝑜𝑢𝑡 :

6 For a more traditional imperative description of the algorithm, we refer the reader to Cox [2009].

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 17 -->

Formal Verification for JavaScript Regular Expressions

(i, best, [], [], S) →code VMend (best)

Final

17

next(i) = Some i′
B ≠ []
NextChar
′
(i, best, [], B, S) →code (i , best, B, [], ∅)

(pc, b) ∈ S
Skip
(i, best, (pc, gm, b) :: A, B, S) →code (i, best, A, B, S)
code # pc = Accept
(pc, b) ∉ S
S ′ = S ∪ {(pc, b)}
Match
(i, best, (pc, gm, b) :: A, B, S) →code (i, Some (i, gm), [], B, S ′ )
code # pc = Consume cd
(pc, b) ∉ S

advance(cd, i, →) ≠ None
S ′ = S ∪ {(pc, b)}

(i, best, (pc, gm, b) :: A, B, S) →code (i, best, A, B ++ [(pc + 1, gm, ⊤)], S ′ )
code # pc = Consume cd
(pc, b) ∉ S

Block

advance(cd, i, →) = None
S ′ = S ∪ {(pc, b)}

(i, best, (pc, gm, b) :: A, B, S) →code (i, best, A, B, S ′ )

FailBlock

code # pc = Jump pc ′
(pc, b) ∉ S
S ′ = S ∪ {(pc, b)}
Jump
(i, best, (pc, gm, b) :: A, B, S) →code (i, best, (pc ′, gm, b) :: A, B, S ′ )
code # pc = Fork pc 1 pc 2
(pc, b) ∉ S
S ′ = S ∪ {(pc, b)}
Fork
(i, best, (pc, gm, b) :: A, B, S) →code (i, best, (pc 1, gm, b) :: (pc 2, gm, b) :: A, B, S ′ )
code # pc = RegOpen g
(pc, b) ∉ S

GMopen (gm, g, idx(i)) = gm′
S ′ = S ∪ {(pc, b)}

(i, best, (pc, gm, b) :: A, B, S) →code (i, best, (pc + 1, gm′, b) :: A, B, S ′ )
code # pc = RegClose g
(pc, b) ∉ S

GMclose (gm, g, idx(i)) = gm′
S ′ = S ∪ {(pc, b)}

(i, best, (pc, gm, b) :: A, B, S) →code (i, best, (pc + 1, gm′, b) :: A, B, S ′ )
code # pc = ResetRegs gl
(pc, b) ∉ S

Open

Close

GMreset (gm, gl) = gm′
S ′ = S ∪ {(pc, b)}

(i, best, (pc, gm, b) :: A, B, S) →code (i, best, (pc + 1, gm′, b) :: A, B, S ′ )

Reset

code # pc = BeginLoop
(pc, b) ∉ S
S ′ = S ∪ {(pc, b)}
Begin
(i, best, (pc, gm, b) :: A, B, S) →code (i, best, (pc + 1, gm, ⊥) :: A, B, S ′ )
code # pc = EndLoop pc ′
(pc, ⊤) ∉ S
S ′ = S ∪ {(pc, ⊤)}
End
(i, best, (pc, gm, ⊤) :: A, B, S) →code (i, best, (pc ′, gm, ⊤) :: A, B, S ′ )
code # pc = EndLoop pc ′
(pc, ⊥) ∉ S
S ′ = S ∪ {(pc, ⊥)}
EndStuck
(i, best, (pc, gm, ⊥) :: A, B, S) →code (i, best, A, B, S ′ )
Fig. 12. PikeVM small-step semantics

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 18 -->

18

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

match with even higher priority may be found later when not in a VMend () state). A is a list of
active threads to explore, ordered by priority. B is an ordered list of blocked threads that reached
a Consume instruction for the current input, which will become the list of active threads when
the algorithm advances to the next input. Finally, S is a set of pairs (pc, b) that have already been
visited in the execution for the current input. The initial state of the algorithm is defined to be
Δ
VMinit (i) = (i, None, [(0, GM ∅ , ⊤)], [], ∅).
Small-step rules. A final state is reached when there are no more active or blocked threads (Final).
When a thread reaches an already explored state (where (pc, b) ∈ S), it is discarded entirely (Skip).
This mechanism is what makes the algorithm linear: each NFA state is explored at most once per
input character (the S set is reset each time PikeVM moves to the next input (NextChar)). Since
threads are always ordered by priority, when two threads eventually reach the same configuration
(pc, b), the lower priority one is discarded. The other rules explain how to handle each instruction.
When reaching an Accept instruction (Match), a new top-priority match is found and stored in
best. In that case, the algorithm discards the remaining, lower-priority active threads, but keeps the
blocked threads that could lead to a higher-priority match. When reaching a Consume instruction,
the current active thread is added to the bottom of the blocked list (Block), and the Boolean is set to
⊤ to indicate that a character has been read since entering the last star. Conversely, when reaching
a BeginLoop instruction, the Boolean is set to ⊥. Finally, when reaching an EndLoop instruction,
the thread is either kept or discarded depending on its Boolean (End, EndStuck).
An example of PikeVM bytecode for the regex (1 a∗|a)b is shown on Figure 15a. An example
execution for the input "ab" is shown on the right column of Figure 15c, showing the evolution
of the lists A and B where each thread is represented only with its pc. The algorithm uses an
unconventional exploring order of branches: it alternates between a breadth-first search (computing
all possible reachable states for a given input position), and a depth-first search within a given
input position. In that example, we can also see PikeVM skipping a branch: it visits pc 9 twice for
the same input position (once per branch of the disjunction) and discards the second visit.
Correctness. To formally verify the correctness of PikeVM algorithm, one needs to prove that if a
final state can be reached using the small-step semantics of Figure 12, then this result corresponds
to the ECMAScript specification. As PikeVM is vastly different from a backtracking algorithm, the
proof needs to address the following verification challenges:
• Different progress checks: Instead of comparing inputs, PikeVM uses a Boolean to encode
progress in star iterations, as suggested in [Barrière and Pit-Claudel 2024]. One needs to
verify that this Boolean correctly mirrors the outcomes of each progress check.
• Different exploration schemes: PikeVM explores several possible paths in parallel, while
the backtracking semantics only explores one at a time in a depth-first search. One needs to
prove that these two exploration schemes return the same result.
• Skipping branches for linearity: To ensure linearity, PikeVM skips entire branches when
they correspond to previously visited states (pc, b) of the extended NFA. One needs to prove
that skipping these paths does not change the result.
• Compilation correctness: The PikeVM compiles the regex to a bytecode. This compilation
needs to be formally verified.
We have designed a proof methodology in three steps, addressing most of these challenges
separately. In all steps, the backtracking tree semantics allows to write elegant invariants, and
offers a convenient induction principle. In a first step, in Section 6.2, we handle the first verification
challenge by presenting an alternative tree semantics, resembling the rules of Figure 4, but encoding
progress with a Boolean instead of comparing the current input with the input in an Acheck action.
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 19 -->

Formal Verification for JavaScript Regular Expressions

19

(l, i, ⊤) ⇓ t
Check

(Acheck icheck :: l, i, ⊤) ⇓ Progress t

CheckFail

(Acheck icheck :: l, i, ⊥) ⇓ Mismatch

advance(cd, i, d) = Some (c, i′ )

(l, i′, ⊤) ⇓ t
Read

(cd :: l, i, b) ⇓ Read c t
(l, i, b) ⇓ t𝑠𝑘𝑖𝑝

(r :: Acheck i :: r{0, ∆, ⊤} :: l, i, ⊥) ⇓ titer
Greedy

(r{0, ∆ + 1, ⊤} :: l, i, b) ⇓ Choice (Reset G(r) titer ) t𝑠𝑘𝑖𝑝
Fig. 13. Boolean tree semantics — selected rules

([], i) ⊢ b

(l, i) ⊢ b
(r :: l, i) ⊢ b

(l, i) ⊢ b
(Aclose g :: l, i) ⊢ b

(l, i) ⊢ ⊤
icheck <→ i
(Acheck icheck :: l, i) ⊢ ⊤

(l, i) ⊢ b
(Acheck i :: l, i) ⊢ ⊥

Fig. 14. Encoding actions with a Boolean

Then, to handle the two next challenges, we present an intermediate algorithm in Section 6.3, which
follows the path exploration order and the skipping of PikeVM, but without compiling to bytecode.
Finally, in Section 6.4, we tackle the final challenge, and show that the PikeVM algorithm working
on the compiled bytecode returns the same result as the intermediate algorithm.
6.2

Correctness of Encoding Progress

First, we prove that encoding progress with a Boolean correctly allows PikeVM to predict the
behavior of the progress checks. This property can be expressed purely using trees, without even
considering the PikeVM compilation process.
We present a new semantics, the Boolean tree semantics, of which a few selected rules are
depicted on Figure 13. This semantics is identical to the previous tree semantics of Section 3, with
two exceptions. First, we add a Boolean parameter indicating whether a check would succeed.
Rules Check and CheckFail use this parameter to decide whether to fail or not, independently
from the input inside the Acheck action. The Boolean is updated in the premises of the rules that
read a character (Read) or enter a non-forced iteration of a quantifier (Greedy). Second, since
this semantics is used to verify the correctness of PikeVM, we make simplifications to reflect the
restricted subset of supported regexes P. We remove irrelevant parameters and rules, for instance
the direction (no lookarounds) and the group map (no backreferences).
Linear algorithms rely on the uniform-futures property [Barrière and Pit-Claudel 2024], that
states that while matching a regex, the future of a path is independent from the current values of
each group. With the removal of the group map parameter, the Boolean tree semantics of Figure 13
has this property by construction.
To relate the two semantics, we define an invariant, (l, i) ⊢ b on Figure 14. To understand how
it can be used to prove the correctness of the Boolean encoding, consider a backward proof of
(l, i) ⊢ 𝑏. After using the fourth rule of Figure 14, the hypothesis requires the Boolean to be ⊤.
Consequently, the rest of the proof cannot use the fifth rule anymore. In other words, (l, i) ⊢ 𝑏
implies that l can be split into two lists: first, a list where each Acheck icheck action has its input
icheck equal to the current input i (corresponding to stars that have just been entered), followed by a
list where each Acheck icheck action has its input icheck strictly smaller than i (for stars in which we
have made progress already). When 𝑏 = ⊤, it means that the first list contains no Acheck action: all
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 20 -->

20

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

stars have progressed, and all progress checks will succeed. When 𝑏 = ⊥, there is at least one check
for which progress has not been made, and the next check will fail unless we advance in the input.
Finally, we can prove that for the same regex and the same input, the two semantics build the
same tree. With this proof, we have showed that the progress strategy used by PikeVM (encoding
with a Boolean the validity of future checks), matches the specification.
Theorem 9 (Correctness of the Boolean semantics). ∀ l ∈ P, i, gm, t, b.
(l, i) ⊢ b ∧ (l, i, gm, →) ⇓ t =⇒ (l, i, b) ⇓ t.
Theorem 10. ∀ r ∈ P, i, t. ( [r], i, GM ∅ , →) ⇓ t ⇐⇒ ( [r], i, ⊤) ⇓ t.
Proof. Theorem 9 is proved by induction on a derivation of (l, i, gm, →) ⇓ t. The left to right
direction of Theorem 10 is a direct consequence of Theorem 9. The right to left direction is a
consequence of the productivity of the tree semantics (Theorem 3), the determinism of the Boolean
tree semantics (similar to Theorem 1) and Theorem 9.
□
6.3

Correctness of the PikeVM Exploration Scheme

As the second step of our proof, we present an intermediate algorithm, resembling PikeVM but
without bytecode compilation. This algorithm, which we name PikeTree, is given a backtracking
tree, and finds its top priority accepting branch. Instead of exploring the tree in depth-first order like
a backtracking algorithm, the algorithm emulates the mixed breadth-first and depth-first exploration
order of PikeVM: it explores several paths in parallel, all synchronized on the same input position.
Just like PikeVM avoids exploring the same NFA state twice, this algorithm also skips some subtrees
that have already been explored. Our strategy consists in first showing that this algorithm always
computes the first accepting branch of a given tree, then showing that a PikeVM execution of a
regex and a string corresponds to a PikeTree execution of the corresponding tree (Section 6.4). The
key advantage of this strategy is that we can prove two crucial properties without even considering
compilation to a bytecode: the correctness of the exploration order, and the possibility of skipping
entire branches. Here we also make use of a key property of our backtracking trees: since the
PikeVM algorithm explores paths that are not explored by a backtracking algorithm that stops at
the first priority match, it is crucial for our semantics to materialize these extra paths.
An example execution of the PikeTree algorithm is shown on Figure 15. Figure 15b shows the
backtracking tree of (1 a∗|a)b on input "ab", which is used to initialize the algorithm. Instead of
maintaining lists of threads like PikeVM, PikeTree maintains lists of trees to explore. It can also
skip trees already explored for the current input position. For instance, the subtree 𝑡 9 is equal to
the subtree 𝑡 15 (the two rectangles in Figure 15b), which allows the PikeTree algorithm to skip 𝑡 15
in its execution on Figure 15c. On this diagram, we can see how the PikeTree algorithm is designed
to match the behavior of PikeVM: each active tree corresponds to an active thread (but note that
PikeTree sometimes takes fewer steps than PikeVM, see Section 6.4).
Selected rules of the PikeTree small-step semantics are presented in Figure 16. The full semantics
is available in Appendix C. States of the PikeTree algorithm are either of the form PTend (best) when
the algorithm terminates, or (i, best, A, B, S) otherwise. Given a tree t and an input i, the initial
Δ
state of the PikeTree algorithm is defined to be PTinit (t, i) = (i, None, [(t, GM ∅ )], [], ∅). These states
closely resemble those of the PikeVM semantics of Figure 12, with the following differences. First,
A and B are now lists of pairs (t, gm) of a subtree to explore and a group map. Second, the S set
now contains subtrees that have already been explored.
More importantly, the way in which the PikeTree algorithm skips branches is different from
PikeVM. In PikeVM, different threads, at different program counters, can correspond to the execution
of the same subtree. For instance, when compiling the regex 𝑎𝑏 | 𝑎𝑏, the PikeVM algorithm will
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 21 -->

Formal Verification for JavaScript Regular Expressions

NFA((1 a∗|a)b)
0 RegOpen 1
1 Fork 2 8
2 Fork 3 7
3 BeginLoop
4 ResetRegs []
5 Consume a
6 EndLoop 2
7 Jump 9
8 Consume a
9 RegClose 1
10 Consume b
11 Accept

PikeTree
A
B
[𝑡 0 ]
[]
[𝑡 1 ]
[]
[𝑡 2 ; 𝑡 14 ]
[]
[𝑡 3 ; 𝑡 12 ; 𝑡 14 ] []

(a) Bytecode NFA of (1 a∗|a)b

𝑡 0 : Open 1
𝑡 1 : Choice
𝑡 2 : Choice

𝑡 14 : Read a

𝑡 3 : Reset []

𝑡 12 : Close 1

𝑡 15 : Close 1

𝑡 4 : Read a

𝑡 13 : Mismatch

𝑡 16 : Read b

𝑡 5 : Progress

𝑡 17 : Match

𝑡 6 : Choice
𝑡 7 : Reset []

𝑡 9 : Close 1

𝑡 8 : Mismatch

𝑡 10 : Read b

21

Equal subtrees

𝑡 11 : Match
(b) Backtracking tree of (1 a∗|a)b and "ab"

PikeVM
A
B
[0]
[]
[1]
[]
[2; 8]
[]
[3; 7; 8] []
[4; 7; 8] []
[𝑡 4 ; 𝑡 12 ; 𝑡 14 ] []
[5; 7; 8] []
[𝑡 12 ; 𝑡 14 ]
[𝑡 5 ]
[7; 8]
[6]
[9; 8]
[6]
[𝑡 13 ; 𝑡 14 ]
[𝑡 5 ]
[10; 8]
[6]
[8]
[6]
[𝑡 14 ]
[𝑡 5 ]
[]
[𝑡 5 ; 𝑡 15 ] []
[6; 9]
Advance after reading "a"
[𝑡 5 ; 𝑡 15 ]
[]
[6; 9]
[]
[2; 9]
[]
[𝑡 6 ; 𝑡 15 ]
[]
[𝑡 7 ; 𝑡 9 ; 𝑡 15 ] []
[3; 7; 9] []
[4; 7; 9] []
[𝑡 8 ; 𝑡 9 ; 𝑡 15 ] []
[5; 7; 9] []
[7; 9]
[]
[𝑡 9 ; 𝑡 15 ]
[]
[9; 9]
[]
[𝑡 10 ; 𝑡 15 ]
[]
[10; 9]
[]
[9]
[11]
[𝑡 15 ]
[𝑡 11 ]
Skip 𝑡 15 =𝑡 9
Skip 9
[]
[𝑡 11 ]
[]
[11]
Advance after reading "b"
[𝑡 11 ]
[]
[11]
[]
Found result
Found result
(c) Execution of PikeTree and PikeVM

Fig. 15. Example execution of the PikeTree and PikeVM algorithms for regex (1 a∗|a)b and input "ab"

compile 𝑎𝑏 twice in different locations. Two threads of PikeVM executing these two pieces of
bytecode will always correspond to the same tree, but since they are located at different places in
the bytecode, PikeVM will not skip either of them. As our goal is to relate a PikeVM execution to a
PikeTree execution, we make the PikeTree algorithm non-deterministic: when it sees a tree that is
already in the S set, it can either skip it (Skip), or not skip it and process it anyways (for instance
Blocked does not require that Read c t ∉ S). While the PikeTree algorithm is non-deterministic,
we prove that each possible execution produces the correct result, and then show that the PikeVM
execution corresponds to one specific PikeTree execution.
To prove the correctness of our new algorithm we define a relation, pts ↓↓ res, between a state of
the PikeTree semantics (pts) and an optional match result (res), then show that this relation is an
invariant of the execution. This invariant is a combination of several other relations.
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 22 -->

22

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

(i, best, [], [], S) → PTend (best)

Final

t∈S
Skip
(i, best, (t, gm) :: A, B, S) → (i, best, A, B, S)

S ′ = S ∪ {Read c t}
Blocked
(i, best, (Read c t, gm) :: A, B, S) → (i, best, A, B ++ [(t, gm)], S ′ )
Fig. 16. PikeTree small-step semantics — selected rules (full version in Appendix C)

First, we define a non-deterministic relation (t, gm) ↓iS res, relating a tree t to an optional result
res. Informally, (t, gm) ↓iS res holds when res is a leftmost accepting leaf of t after removing any
number of subtrees found in S, for input i and group map gm . This means that executing the
PikeTree algorithm on tree t with the set S can produce result res (recall that PikeTree may, but
does not have to, skip trees in S). For instance, if res1 is the first accepting branch of t1 and res2 of
t2 , then both Choice t1 t2 ↓i{t1 } res1 and Choice t1 t2 ↓i{t1 } res2 hold.
We extend this definition to ordered lists of trees: A ↓iS res means that res is one possible
first result of the list A after removing any number of subtrees in S. We can further extend this
definition to PikeTree semantics states (where 𝑟 1 ◦ 𝑟 2 is equal to 𝑟 1 when it is different from None,
and 𝑟 2 otherwise):
Δ

(i, best, A, B, S) ↓ res = ∃res𝑎 , res𝑏 . B ↓next(i)
res𝑏 ∧ A ↓iS res𝑎 ∧ res = res𝑏 ◦ res𝑎 ◦ best
∅
While this relation is non-deterministic (because we can choose whether to skip subtrees in S),
it turns out that the PikeTree algorithm can only evaluate to a single result. Our final invariant,
pts ↓↓ res, captures that fact. We define the invariant below, then prove that the invariant is correctly
initialized (Theorem 11) and preserved (Theorem 12).
∀𝑟 . (i, best, A, B, S) ↓ 𝑟 =⇒ 𝑟 = res
PTend (best) ↓↓ best
(i, best, A, B, S) ↓↓ res
Theorem 11 (Invariant initialization). ∀ i, t ∈ P. PTinit (t, i) ↓↓ L0 (t, i).
Theorem 12 (Preservation). ∀ pts1, pts2, res. pts1 ↓↓ res ∧ pts1 → pts2 =⇒ pts2 ↓↓ res.
Proof. For Theorem 11, the initial state PTinit (t, i), has an empty S set. As a result, the only
possible result is the first accepting branch of 𝑡. The proof for Theorem 12 proceeds by case analysis
over pts1 → pts2 . Informally, when the algorithm skips a previously seen tree t (Skip), we know
that all results of the new state are results of the previous state where t was skipped. In other cases,
when visiting a new tree t, PikeTree not only adds t to the S set, but also adds the children of
t either in A or in B. Because of the addition to the S set, PikeTree might skip more branches
(corresponding to t) in the future. However, we prove that skipping these branches will not change
the result of the algorithm, since any result in these branches is also a result of the children of t. □
6.4

Correctness of the PikeVM Compiler

We now prove that each execution of the PikeVM algorithm itself, as described on Figure 12,
corresponds to an execution of the PikeTree algorithm. Informally, at all times during the execution,
each thread of the active or blocked list in the PikeVM state corresponds to a tree in the active
or blocked list of the PikeTree algorithm (see Figure 15c). We then need to define an equivalence
relation between threads and trees. Whenever PikeVM skips a thread (pc, gm, b) because it has seen
its program counter pc and Boolean b already, it means that PikeTree has seen the corresponding
tree already and thus is allowed to skip it as well. Most steps of the PikeVM algorithm directly
correspond to a similar step in the PikeTree algorithm, with a few exceptions explained below.
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 23 -->

Formal Verification for JavaScript Regular Expressions

(l, i, b) ⇓ t

23

(t, gm) ∼icode (pc + 1, gm, ⊥)

code # pc = BeginLoop

repcode l pc

(t, gm) ∼icode (pc, gm, b)

(t, gm) ∼icode (pc, gm, b)
GMreset (gm, gl) = gm′

code # pc = ResetRegs gl

(t, gm′ ) ∼icode (pc + 1, gm′, b)

(Reset gl t, gm) ∼icode (pc, gm, b)
Fig. 17. Equivalence between trees and PikeVM threads

A ∼icode A𝑉 𝑀
PTend (res) ∼code VMend (res)

B ∼next(i)
B𝑉 𝑀
code

S𝑉 𝑀 ⊆ A S

(i, best, A, B, S) ∼code (i, best, A𝑉 𝑀 , B𝑉 𝑀 , S𝑉 𝑀 )

Fig. 18. Equivalence between PikeTree semantic states and PikeVM semantic states

First, we formalize this notion of equivalence between threads and trees on Figure 17. The first
rule corresponds to the more common case. A thread (pc, gm, b) is equivalent to a tree t when
there exists a list of actions l, such that t is the tree of l for the current input i and Boolean b, and
this list of actions is represented in the code at label pc. This representation predicate repcode l pc
(omitted here for brevity) is inductively defined to state that the bytecode corresponding to each
action in l starts at label pc in code, and then points to an Accept instruction. The representation
of an Aclose g action is a RegClose g instruction, and the representation of an Acheck icheck action
is an EndLoop instruction. In-between the bytecode representation of each action, there may be a
Jump instruction, in case we are representing the first branch of a disjunction (see Figure 11).
The invariant also contains two more rules to cover cases where the thread does not directly
correspond to the tree of a list of actions. This happens in two cases, when the thread is at a
ResetRegs or a BeginLoop instruction. Finally, we naturally extend this equivalence to lists of
threads and trees, and we write A ∼icode A𝑉 𝑀 when all elements of the lists are pairwise equivalent.
The executions of the PikeVM and the PikeTree algorithms are quite similar. As one adds
seen threads to its S set, the other adds equivalent trees. There is however one exception: the
PikeVM algorithm sometimes performs more steps than the PikeTree algorithm. This happens when
executing bytecode instructions that do not correspond to operations recorded in the backtracking
tree: Jump and BeginLoop. We refer to these instructions as stuttering instructions. In Figure 15c,
when executing labels 3 and 7, PikeVM takes one more step than PikeTree. As we execute these
instructions, some threads are added to the S set of PikeVM before the corresponding tree is added
on the PikeTree side. As a result, an invariant of the executions is that each thread (pc, gm, b) of the
S𝑉 𝑀 set of PikeVM is either equivalent to a tree in the S set of the PikeTree algorithm, or code # pc
is a stuttering instruction and (pc, gm, b) is equivalent to the current active tree of the PikeTree
algorithm (the head of the A list). We write S𝑉 𝑀 ⊆ A S to express this property. Finally, we can
define on Figure 18 the invariant relating the execution of PikeVM to one execution of the PikeTree
algorithm, and show that the invariant is initialized (Theorem 13) and preserved (Theorem 14).
Theorem 13 (PikeVM invariant initialization). ∀ r ∈ P, t, code, i.
NFA(r) = code ∧ ( [r], i, ⊤) ⇓ t =⇒ PTinit (t, i) ∼code VMinit (i).
Theorem 14 (PikeVM invariant preservation). ∀ pts1, pvs1, pvs2, code.
pts1 ∼code pvs1 ∧ pvs1 →code pvs2 =⇒ (∃pts2 . pts1 → pts2 ∧ pts2 ∼code pvs2 ) ∨ (pts1 ∼code pvs2 ).
Proof. For Theorem 13, we show that the list of actions [r] is represented at the label 0 of its
compiled code code by induction over r. Then, the proof of Theorem 14 proceeds by case analysis
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 24 -->

24

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

over pvs1 →code pvs2 . Most cases are proved by induction over the (l, i, b) ⇓ t predicate relating the
current active tree t being executed by PikeTree and the actions l represented at the current active
thread executed by PikeVM. An induction is required because this list of actions could start with
an arbitrary sequence of 𝜀 actions before the action responsible for the current operation in the
tree and current instruction in the bytecode. By construction of the Boolean semantics (Figure 13),
we deduce that progress checks on the PikeTree side match the checks on the PikeVM side. When
PikeVM decides to skip a thread, it means this thread was in its S set. From the inclusion between
S sets, we know that there exists an equivalent tree that can be skipped on the PikeTree side. □
Finally, we can prove that PikeVM returns the same result as the ECMAScript specification
(Theorem 16) by first proving that PikeVM returns a result of PikeTree (Theorem 15), where →∗
represents the transitive reflexive closure of a small-step relation.
Theorem 15 (PikeVM to PikeTree execution). ∀ 𝑟 ∈ P, i, t, res.
([r], i, ⊤) ⇓ t ∧ VMinit (i) →∗NFA(r ) VMend (res) =⇒ PTinit (t, i) →∗ PTend (res)
Theorem 16 (PikeVM correctness theorem). ∀ r𝑤 ∈ W, i, res.
⇃r𝑤⇂ ∈ P∧ VMinit (i) →∗NFA(⇃r𝑤⇂) VMend (res) =⇒ compile(r𝑤 ) (str(i), idx(i)) = ↿res↾.
Proof. Theorem 15 is proved by induction over the derivation of VMinit (i) →∗NFA(r ) VMend (res),
applying Theorem 14 at each step. Our final Theorem 16 then follows from previous results. From
Theorem 15, we know that each execution of the PikeVM algorithm on a regex and an input
corresponds to an execution of the PikeTree algorithm on the corresponding Boolean tree. From
Theorem 9, we know that this corresponding Boolean tree is the backtracking tree of that regex
and that input. From Theorems 12 and 11, we know that each result of the PikeTree algorithm
corresponds to the first accepting branch of that tree. Finally, from Theorem 4, we know that this
first accepting branch is precisely the result defined by the ECMAScript standard.
□
Despite the PikeVM algorithm having been used widely for decades, this is the first time that
it has been formally proved to return the top priority result according to backtracking semantics.
Reasoning on backtracking trees greatly facilitates the proof as it enables simple ways to express
crucial properties. For instance, proving the correctness of skipping branches would be much harder
to prove directly on the PikeVM algorithm: two executions of the same bytecode instruction might
yield completely different results when the S set is different.
7

Discussion and Related Work

Adapting our work to other languages. While our semantics matches that of the JavaScript regexes,
it could be straightforwardly modified for other languages with backtracking semantics. We chose
JavaScript for its wide use and because it is the only modern backtracking semantics regex language
to come with a full mechanized specification that we can formally compare ourselves to.
For the PikeVM, we could expect the following changes. In languages without capture reset, we
could simply remove the Reset node from the tree type, and the PikeVM would not generate any
ResetRegs instruction. In languages without the special nullable quantifier semantics of JavaScript,
we would remove the Acheck action, and make all nodes of the tree check for progress. The
PikeVM would become simpler: we could remove the Boolean from threads entirely, and remove
instructions BeginLoop and EndLoop. In the proof, we would remove the Boolean semantics of
Section 6.2 entirely. In both cases, there would be one less case in Figure 17.
For contextual equivalence, most of our proofs of Section 5 do not depend on JavaScript specificities (with the exception of anchor rewriting equivalences). We expect them to hold in any
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 25 -->

Formal Verification for JavaScript Regular Expressions

25

backtracking semantics language. Counter-examples are easier to generalize as they can be confirmed with tests, we discuss them in Appendix B.

7.1

Related Work

With the exception of Warblre, mechanized semantics and verification work typically exclude
backtracking semantics and capture groups. This is known to be difficult: according to Zhuchko
et al. [2024], “Formalizing even the core aspect of the capture semantics and algorithms [. . . ] is therefore
a major undertaking with many challenges”. Our semantics and proofs provide a solution.
Warblre. Warblre [De Santo et al. 2024] was the first mechanization, in an interactive theorem
prover, of a modern, general-purpose regex language (ECMAScript 2023 [ECMA 2023, §22.2]). By
closely following the paper specification, it traded succintness and ease-of-use for high auditability
and faithfulness; accordingly, its authors described it as “a foundation for researchers to restate
the semantics in a way that better suits their field”. Our Section 3 does that, and provides a new,
complete formalization of ECMAScript 2023 §22.2. It preserves the desirable properties of Warblre
(in particular, Theorem 4 shows faithfulness), but without its shortcomings: our tree semantics
are more succinct, and much easier to reason about (we provide an induction principle, and we
eliminate the burden of reasoning about the error monad).
JavaScript regex formalizations. Several previous efforts have developed unmechanized formal
semantics for JavaScript regexes. Loring et al. [2019] presented a formalized semantics for ECMAScript 2015 regexes and used it for symbolic execution of programs with regexes. Similarly,
Eriksson et al. [2023] augmented the Ostrich constraint solver with a model of ECMAScript regexes,
using two-way alternating automata. Recently, Chida and Terauchi [2023, 2024] presented a formal semantics for JavaScript regexes used for regex repair. Their semantic statement includes a
matching direction and a group map just like ours, and their continuation regex resembles our list
of actions, but they only formalize the top-priority match. Finally, Chen et al. [2022] formalized a
large subset of JavaScript regex semantics using prioritized streaming string transducers. While
there are mechanizations of the JavaScript language [Bodin et al. 2014; Park et al. 2020], none
include the regular expression chapter of ECMAScript. In fact, besides Warblre, our semantics is the
only mechanized one for JavaScript regexes, and also the only complete one: other formalizations
exclude some JavaScript features such as lookbehinds, regex flags, or backreferences. Manually
defining formal semantics for a modern regex language is a complex and error-prone task. For
instance, careful inspection of the semantics proposed by Eriksson et al. [2023, Extended version,
Table 5] reveals some mistakes: capture reset is not performed, and the semantics of lookarounds is
incorrect. JavaScript lookarounds are supposed to be atomic, meaning that once a match is found
for a lookaround, an engine cannot backtrack and match it in a different way [ECMA 2025, Section
22.2.2.4, Note 3]. Such subtle differences are easy to miss (this behavior of lookarounds can only be
observed with a backreference to a capture group defined inside a positive lookaround); by contrast,
our proof of Theorem 4 ensures the correctness of the semantics. Still, mechanization also entails
specific challenges: for instance, some paper formalizations use non-strict positive occurrences by
referring to the negation of matching within the definition of the matching relation itself (for
instance for negative lookarounds, or for the second branch of the disjunction). Such definitions
would have to be adjusted to be accepted by Rocq [Zhuchko et al. 2024].
For a different language, the original PCRE documentation [Hazel 2012] notes that “The set of
strings that are matched by a regular expression can be represented as a tree structure [...] of infinite
size” and mentions both depth-first and breadth-first traversal of the tree. We have found that
specializing the tree to a particular input provides a finite tree that is more practical for formal

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 26 -->

26

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

verification than a coinductive one, and that the PikeVM algorithm is more than a breadth-first
traversal: it alternates between depth-first and breadth-first order to maintain priority.
Formalizations of modern features. Other work has also verified or formalized individual modern
features, but in simplified languages. For instance, matching algorithms supporting lookarounds
have recently been mechanized in Lean [Zhuchko et al. 2024] and Rocq [Chattopadhyay et al.
2025]. However, these works include neither capture groups nor backtracking semantics. Other
unmechanized work specify capture groups using prioritized transducers [Berglund and van der
Merwe 2017] or encode lookarounds as alternating automata [Berglund et al. 2021].
PikeVM verification. Recently, the lean-regex regex matcher has also been implemented and
verified in Lean [Pan 2025]. This project also includes a PikeVM implementation, along with a
proof. However, the proof differs from our Section 6 in some ways. First, its theorem states that the
algorithm returns one of the possible matches of the input regex on the input string. In contrast, in our
work we consider that the correctness of the PikeVM requires a stronger property: not only that it
returns a result when it should, but also that it returns the top-priority one according to backtracking
semantics. Second, our theorem also relates the result of the PikeVM to a mechanized translation
of a real-world regex semantics, Warblre. On the other hand, the lean-regex formalization can
be executed and uses efficient data-structure implementations and optimizations, while our work
stops at algorithm verification.
Traditional regular expressions. Traditional regular expressions and finite state automata, including NFA simulation, have been verified and mechanized many times [Braibant and Pous 2010;
Cardoso et al. 2023; Coquand and Siles 2011; Doczkal et al. 2013; Doczkal and Smolka 2018; Firsov
and Uustalu 2013; Jourdan et al. 2012; Krauss and Nipkow 2010; Owens and Slind 2008]. Regular
expression matching algorithms have been formally verified for lexers [Chassot and Kunčak 2024;
Egolf et al. 2021, 2022; Ouedraogo et al. 2024] and in particular for leftmost-longest semantics [Ausaf
et al. 2016; Urban 2023], but not for backtracking semantics.
7.2

Future Work

Our semantics is verified to be equivalent to the latest version of Warblre, based on the 2023 edition
of ECMAScript [ECMA 2023]. As of 2025, two new editions have been released [ECMA 2024, 2025],
introducing new regex features: the v Unicode flag, modifiers, and duplicate named groups. If
Warblre were to be updated, we could adapt accordingly. First, the new v flag adds new features
in character descriptors that we could support by adapting the advance(cd, i, d) function. Second,
new modifiers allow flags i, m and s to be set locally in a subregex. Currently, we pass the record
containing the value of each flag as parameter of the (l, i, gm, d) ⇓ t predicate. To support local
modifiers, we would pass this record as an index instead, and add a corresponding type of node in
backtracking trees. Finally, duplicate named groups allow some capture group identifiers to appear
several times in distinct disjunction branches. If Warblre’s W set were updated, we expect that
we could accordingly relax our definition of contextual equivalence to allow equivalent regexes to
have duplicated named groups.
While we have proved the correctness of the PikeVM algorithm, we have not verified an implementation, let alone an efficient one. A realistic engine would require efficient data structures
(e.g., for capture groups and group maps), matching optimizations (e.g., prefix acceleration), and a
specification and implementation of top-level APIs (e.g., matchAll). Other bugs in implementations
can lie in their complexity. Future work could prove that the PikeVM semantics terminates in a
number of steps linear in the size of the input.
It would also be interesting to extend the base PikeVM algorithm with additional features, to
obtain a formally verified linear-time algorithm supporting all JavaScript regex features except
, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 27 -->

Formal Verification for JavaScript Regular Expressions

27

backreferences. We expect that adapting our proofs would be straightforward for most missing
features. To support anchors, existing PikeVM implementations compile them to a new bytecode
instruction that checks the surroundings of the current string position. Supporting the question
mark quantifiers can be done with only Fork, BeginLoop and EndLoop instructions. Then, generic
quantifiers r{min, ∆, p} can be handled by duplicating the bytecode of r (for instance, compiling
r{1, 1, ⊤} concatenates the bytecodes of r and r?). A more challenging extension would consist in
supporting lookaorunds, following the algorithm of Barrière and Pit-Claudel [2024].
Finally, other work has also explored other extended features that are not part of JavaScript, such
as atomic groups [Fujinami and Hasuo 2024], the shuffle operator [Thiemann 2016], or verifying in
Idris 2 [Kammar and Marek 2023] the type-safety of an engine supporting typed regexes [Radanne
2019]. Other work goes beyond capture groups and instead returns parse trees for standard regular
expressions, indicating the full list of substrings a subexpression has matched [Dubé and Feeley
2000; Frisch and Cardelli 2004; Nielsen and Henglein 2011; Ribeiro and Bois 2017]. An interesting
future direction would be to extend our work to support these extensions and alternative semantics.
8

Conclusion

We have presented a new semantics for JavaScript regexes, that is mechanized in Rocq, complete yet
succinct, proven to be faithful to the ECMAScript standard, and practical for formal verification. We
have showed that formalizing not only the first match of a regex, but also lower-priority matches,
makes the semantics practical. We have used it to provide novel proofs of real-world applications:
the first verification of the PikeVM linear matching algorithm, used in many deployed engines;
and a new formal notion of contextual equivalence that allowed us to prove and disprove regex
equivalences from the literature and from the optimizer of a popular JavaScript regex manipulation
library. Thanks to the verified connection with an audited formalization, formal proofs conducted
with our semantics can be trusted to correspond exactly to the behavior specified by ECMAScript.
Our work lays the foundation for the development of verified and realistic modern regex engines.
Acknowledgments
We thank Yann Herklotz, Martin Odersky, and Marcin Wojnarowski for feedback on this paper,
and Eugène Flesselle for preliminary exploration of regex equivalence. This research was funded in
whole or in part by the Swiss National Science Foundation (SNSF), grant number 10003649.
Artifact Availability
Our development is free software and can be accessed online: https://github.com/epfl-systemf/
Linden. We have also packaged all the definitions and proofs presented in this paper in an artifact [Barrière et al. 2025]. The artifact offers both the standalone Rocq files, and a virtual machine
image (along with a script to create this image from scratch).
References
Andrea Asperti. 2012. A Compact Proof of Decidability for Regular Expression Equivalence. In Interactive Theorem Proving Third International Conference, ITP 2012, Princeton, NJ, USA, August 13-15, 2012. Proceedings (Lecture Notes in Computer
Science, Vol. 7406), Lennart Beringer and Amy P. Felty (Eds.). Springer, 283–298. doi:10.1007/978-3-642-32347-8_19
Fahad Ausaf, Roy Dyckhoff, and Christian Urban. 2016. POSIX Lexing with Derivatives of Regular Expressions (Proof Pearl).
In Interactive Theorem Proving - 7th International Conference, ITP 2016, Nancy, France, August 22-25, 2016, Proceedings
(Lecture Notes in Computer Science, Vol. 9807), Jasmin Christian Blanchette and Stephan Merz (Eds.). Springer, 69–86.
doi:10.1007/978-3-319-43144-4_5
Aurèle Barrière and Clément Pit-Claudel. 2024. Linear Matching of JavaScript Regular Expressions. Proc. ACM Program.
Lang. 8, PLDI (2024), 1336–1360. doi:10.1145/3656431

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 28 -->

28

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel. 2025. Artifact for "Formal Verification for JavaScript Regular
Expressions: a Proven Mechanized Semantics and its Applications", at POPL 2026. doi:10.5281/zenodo.17305393
Martin Berglund and Brink van der Merwe. 2017. On the semantics of regular expression parsing in the wild. Theor. Comput.
Sci. 679 (2017), 69–82. doi:10.1016/J.TCS.2016.09.006
Martin Berglund, Brink van der Merwe, and Steyn van Litsenborgh. 2021. Regular Expressions with Lookahead. J. Univers.
Comput. Sci. 27, 4 (2021), 324–340. doi:10.3897/JUCS.66330
Martin Bodin, Arthur Charguéraud, Daniele Filaretti, Philippa Gardner, Sergio Maffeis, Daiva Naudziuniene, Alan Schmitt,
and Gareth Smith. 2014. A Trusted Mechanised JavaScript Specification. In The 41st Annual ACM SIGPLAN-SIGACT Symposium on Principles of Programming Languages, POPL ’14, San Diego, CA, USA, January 20-21, 2014, Suresh Jagannathan
and Peter Sewell (Eds.). ACM, 87–100. doi:10.1145/2535838.2535876
Thomas Braibant and Damien Pous. 2010. An Efficient Coq Tactic for Deciding Kleene Algebras. In Interactive Theorem
Proving, First International Conference, ITP 2010, Edinburgh, UK, July 11-14, 2010. Proceedings (Lecture Notes in Computer
Science, Vol. 6172), Matt Kaufmann and Lawrence C. Paulson (Eds.). Springer, 163–178. doi:10.1007/978-3-642-14052-5_13
Elton Maximo Cardoso, Leonardo Vieira dos Santos Reis, and Rodrigo Geraldo Ribeiro. 2023. A Verified Operational
Semantics for Regular Expression Parsing. In Proceedings of the XXVII Brazilian Symposium on Programming Languages,
SBLP 2023, Campo Grande, MS, Brazil, September 25-29, 2023. ACM, 82–90. doi:10.1145/3624309.3624317
Samuel Chassot and Viktor Kunčak. 2024.
Verified invertible lexer using regular expressions and DFAs.
arXiv:2412.13581 [cs.PL] https://arxiv.org/abs/2412.13581
Agnishom Chattopadhyay, Wu Angela Li, and Konstantinos Mamouras. 2025. Verified and Efficient Matching of Regular
Expressions with Lookaround. In Proceedings of the 14th ACM SIGPLAN International Conference on Certified Programs
and Proofs, CPP 2025, Denver, CO, USA, January 20-21, 2025, Kathrin Stark, Amin Timany, Sandrine Blazy, and Nicolas
Tabareau (Eds.). ACM, 198–213. doi:10.1145/3703595.3705884
Taolue Chen, Alejandro Flores-Lamas, Matthew Hague, Zhilei Han, Denghang Hu, Shuanglong Kan, Anthony W. Lin, Philipp
Rümmer, and Zhilin Wu. 2022. Solving string constraints with Regex-dependent functions through transducers with
priorities and variables. Proceedings of the ACM on Programming Languages 6, POPL (2022), 1–31. doi:10.1145/3498707
Nariyoshi Chida and Tachio Terauchi. 2023. Repairing Regular Expressions for Extraction. Proceedings of the ACM on
Programming Languages 7, PLDI (2023), 1633–1656. doi:10.1145/3591287
Nariyoshi Chida and Tachio Terauchi. 2024. Repairing Regex-Dependent String Functions. In Proceedings of the 39th
IEEE/ACM International Conference on Automated Software Engineering, ASE 2024, Sacramento, CA, USA, October 27 November 1, 2024, Vladimir Filkov, Baishakhi Ray, and Minghui Zhou (Eds.). ACM, 294–305. doi:10.1145/3691620.3695005
Thierry Coquand and Vincent Siles. 2011. A Decision Procedure for Regular Expression Equivalence in Type Theory. In
Certified Programs and Proofs - First International Conference, CPP 2011. doi:10.1007/978-3-642-25379-9_11
Russ Cox. 2009. Regular Expression Matching: the Virtual Machine Approach. https://swtch.com/~rsc/regexp/regexp2.html.
James C. Davis, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. 2018. The impact of regular expression denial of
service (ReDoS) in practice: an empirical study at the ecosystem scale. In Proceedings of the 2018 ACM Joint Meeting on
European Software Engineering Conference and Symposium on the Foundations of Software Engineering, ESEC/SIGSOFT FSE
2018. ACM, 246–256. doi:10.1145/3236024.3236027
Noé De Santo, Aurèle Barrière, and Clément Pit-Claudel. 2024. A Coq Mechanization of JavaScript Regular Expression
Semantics. Proc. ACM Program. Lang. 8, ICFP (2024), 1003–1031. doi:10.1145/3674666
Christian Doczkal, Jan-Oliver Kaiser, and Gert Smolka. 2013. A Constructive Theory of Regular Languages in Coq. In
Certified Programs and Proofs - Third International Conference, CPP 2013, Melbourne, VIC, Australia, December 11-13, 2013,
Proceedings (Lecture Notes in Computer Science, Vol. 8307), Georges Gonthier and Michael Norrish (Eds.). Springer, 82–97.
doi:10.1007/978-3-319-03545-1_6
Christian Doczkal and Gert Smolka. 2018. Regular Language Representations in the Constructive Type Theory of Coq. J.
Autom. Reason. 61, 1-4 (2018), 521–553. doi:10.1007/S10817-018-9460-X
Mark Jason Dominus. 2000. Perl Regular Expression Matching is NP-Hard. https://perl.plover.com/NPC/NPC-3SAT.html.
Danny Dubé and Marc Feeley. 2000. Efficiently building a parse tree from a regular expression. Acta Informatica 37, 2 (2000),
121–144. doi:10.1007/S002360000037
ECMA. 2023. ECMA-262, 14th edition: ECMAScript® 2023 Language Specification. https://262.ecma-international.org/14.0/
ECMA. 2024. ECMA-262, 15th edition: ECMAScript® 2024 Language Specification. https://262.ecma-international.org/15.0/
ECMA. 2025. ECMA-262, 16th edition: ECMAScript® 2025 Language Specification. https://262.ecma-international.org/16.0/
Derek Egolf, Sam Lasser, and Kathleen Fisher. 2021. Verbatim: A Verified Lexer Generator. In IEEE Security and Privacy
Workshops, SP Workshops 2021, San Francisco, CA, USA, May 27, 2021. IEEE, 92–100. doi:10.1109/SPW53761.2021.00022
Derek Egolf, Sam Lasser, and Kathleen Fisher. 2022. Verbatim++: verified, optimized, and semantically rich lexing with
derivatives. In CPP ’22: 11th ACM SIGPLAN International Conference on Certified Programs and Proofs, Philadelphia, PA,
USA, January 17 - 18, 2022, Andrei Popescu and Steve Zdancewic (Eds.). ACM, 27–39. doi:10.1145/3497775.3503694

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 29 -->

Formal Verification for JavaScript Regular Expressions

29

Benjamin Eriksson, Amanda Stjerna, Riccardo De Masellis, Philipp Rümmer, and Andrei Sabelfeld. 2023. Black Ostrich:
Web Application Scanning with String Solvers. In Proceedings of the 2023 ACM SIGSAC Conference on Computer and
Communications Security, CCS 2023, Copenhagen, Denmark, November 26-30, 2023, Weizhi Meng, Christian Damsgaard
Jensen, Cas Cremers, and Engin Kirda (Eds.). ACM, 549–563. doi:10.1145/3576915.3616582
Denis Firsov and Tarmo Uustalu. 2013. Certified Parsing of Regular Languages. In Certified Programs and Proofs - Third
International Conference, CPP 2013, Melbourne, VIC, Australia, December 11-13, 2013, Proceedings (Lecture Notes in Computer
Science, Vol. 8307), Georges Gonthier and Michael Norrish (Eds.). Springer, 98–113. doi:10.1007/978-3-319-03545-1_7
Dominik D. Freydenberger. 2013. Extended Regular Expressions: Succinctness and Decidability. Theory Comput. Syst. 53, 2
(2013), 159–193. doi:10.1007/S00224-012-9389-0
Alain Frisch and Luca Cardelli. 2004. Greedy Regular Expression Matching. In Automata, Languages and Programming: 31st
International Colloquium, ICALP 2004, Turku, Finland, July 12-16, 2004. Proceedings (Lecture Notes in Computer Science,
Vol. 3142), Josep Díaz, Juhani Karhumäki, Arto Lepistö, and Donald Sannella (Eds.). Springer, 618–629. doi:10.1007/978-3540-27836-8_53
Hiroya Fujinami and Ichiro Hasuo. 2024. Efficient Matching with Memoization for Regexes with Look-around and Atomic
Grouping. In Programming Languages and Systems - 33rd European Symposium on Programming, ESOP 2024, Held as Part
of the European Joint Conferences on Theory and Practice of Software, ETAPS 2024, Luxembourg City, Luxembourg, April
6-11, 2024, Proceedings, Part II (Lecture Notes in Computer Science, Vol. 14577), Stephanie Weirich (Ed.). Springer, 90–118.
doi:10.1007/978-3-031-57267-8_4
Andrew Gallant. 2014. Crate regex: An implementation of regular expressions for Rust. https://docs.rs/regex/latest/regex/.
Go. 2025. GO Regexp package. https://pkg.go.dev/regexp.
Google. 2010. RE2: A fast, safe, thread-friendly alternative to backtracking regular expression engines like those used in
PCRE, Perl, and Python. https://github.com/google/re2.
Philip Hazel. 1997. PCRE - Perl Compatible Regular Expressions. https://www.pcre.org/.
Philip Hazel. 2012. PCRE Library Functions Manual. https://pcre.org/pcre.txt.
Iain Ireland. 2020. A New RegExp Engine in SpiderMonkey. https://hacks.mozilla.org/2020/06/a-new-regexp-engine-inspidermonkey/.
Jacques-Henri Jourdan, François Pottier, and Xavier Leroy. 2012. Validating LR(1) Parsers. In Programming Languages and
Systems - 21st European Symposium on Programming, ESOP 2012, Held as Part of the European Joint Conferences on Theory
and Practice of Software, ETAPS 2012, Tallinn, Estonia, March 24 - April 1, 2012. Proceedings (Lecture Notes in Computer
Science, Vol. 7211), Helmut Seidl (Ed.). Springer, 397–416. doi:10.1007/978-3-642-28869-2_20
Stefan Kahrs and Colin Runciman. 2022. Simplifying regular expressions further. J. Symb. Comput. 109 (2022), 124–143.
doi:10.1016/J.JSC.2021.08.003
Ohad Kammar and Katarzyna Marek. 2023. Idris TyRE: a dependently typed regex parser. CoRR abs/2305.04480 (2023).
doi:10.48550/ARXIV.2305.04480 arXiv:2305.04480
Alexander Krauss and Tobias Nipkow. 2010. Regular Sets and Expressions. Archive of Formal Proofs (May 2010). https://isaafp.org/entries/Regular-Sets.html, Formal proof development.
Alexander Krauss and Tobias Nipkow. 2012. Proof Pearl: Regular Expression Equivalence and Relation Algebra. J. Autom.
Reason. 49, 1 (2012), 95–106. doi:10.1007/S10817-011-9223-4
Blake Loring, Duncan Mitchell, and Johannes Kinder. 2019. Sound Regular Expression Semantics for Dynamic Symbolic
Execution of JavaScript. In Proceedings of the 40th ACM SIGPLAN Conference on Programming Language Design and
Implementation, PLDI 2019, Phoenix, AZ, USA, June 22-26, 2019, Kathryn S. McKinley and Kathleen Fisher (Eds.). ACM,
425–438. doi:10.1145/3314221.3314645
Konstantinos Mamouras and Agnishom Chattopadhyay. 2024. Efficient Matching of Regular Expressions with Lookaround
Assertions. Proc. ACM Program. Lang. 8, POPL (2024), 2761–2791. doi:10.1145/3632934
Nelma Moreira, David Pereira, and Simão Melo de Sousa. 2012. Deciding Regular Expressions (In-)Equivalence in Coq.
In Relational and Algebraic Methods in Computer Science - 13th International Conference, RAMiCS 2012, Cambridge, UK,
September 17-20, 2012. Proceedings (Lecture Notes in Computer Science, Vol. 7560), Wolfram Kahl and Timothy G. Griffin
(Eds.). Springer, 98–113. doi:10.1007/978-3-642-33314-9_7
Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu. 2023.
Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics. Proc. ACM Program. Lang.
7, PLDI (2023), 1026–1049. doi:10.1145/3591262
Lasse Nielsen and Fritz Henglein. 2011. Bit-coded Regular Expression Parsing. In Language and Automata Theory and
Applications - 5th International Conference, LATA 2011, Tarragona, Spain, May 26-31, 2011. Proceedings (Lecture Notes in
Computer Science, Vol. 6638), Adrian-Horia Dediu, Shunsuke Inenaga, and Carlos Martín-Vide (Eds.). Springer, 402–413.
doi:10.1007/978-3-642-21254-3_32
Tobias Nipkow and Dmitriy Traytel. 2014. Unified Decision Procedures for Regular Expression Equivalence. In Interactive
Theorem Proving - 5th International Conference, ITP 2014, Held as Part of the Vienna Summer of Logic, VSL 2014, Vienna,

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 30 -->

30

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Austria, July 14-17, 2014. Proceedings (Lecture Notes in Computer Science, Vol. 8558), Gerwin Klein and Ruben Gamboa
(Eds.). Springer, 450–466. doi:10.1007/978-3-319-08970-6_29
Taisei Nogami and Tachio Terauchi. 2023. On the Expressive Power of Regular Expressions with Backreferences. In
48th International Symposium on Mathematical Foundations of Computer Science, MFCS 2023, August 28 to September 1,
2023, Bordeaux, France (LIPIcs, Vol. 272), Jérôme Leroux, Sylvain Lombardy, and David Peleg (Eds.). Schloss Dagstuhl Leibniz-Zentrum für Informatik, 71:1–71:15. doi:10.4230/LIPICS.MFCS.2023.71
Wendlasida Ouedraogo, Gabriel Scherer, and Lutz Straßburger. 2024. Coqlex: Generating Formally Verified Lexers. Art Sci.
Eng. Program. 8, 1 (2024). doi:10.22152/PROGRAMMING-JOURNAL.ORG/2024/8/3
Scott Owens, John Reppy, and Aaron Turon. 2009. Regular-expression derivatives re-examined. Journal of Functional
Programming 19, 2 (2009), 173–190.
Scott Owens and Konrad Slind. 2008. Adapting functional programs to higher order logic. High. Order Symb. Comput. 21, 4
(2008), 377–409. doi:10.1007/S10990-008-9038-0
Yulu Pan. 2025. Lean-regex. https://github.com/pandaman64/lean-regex.
Jihyeok Park, Jihee Park, Seungmin An, and Sukyoung Ryu. 2020. JISET: JavaScript IR-based Semantics Extraction Toolchain.
In 35th IEEE/ACM International Conference on Automated Software Engineering, ASE 2020, Melbourne, Australia, September
21-25, 2020. IEEE, 647–658. doi:10.1145/3324884.3416632
Rob Pike. 1987. The Text Editor sam. Softw. Pract. Exp. 17, 11 (1987), 813–845. doi:10.1002/SPE.4380171105
Gabriel Radanne. 2019. Typed parsing and unparsing for untyped regular expression engines. In Proceedings of the 2019
ACM SIGPLAN Workshop on Partial Evaluation and Program Manipulation, PEPM@POPL 2019, Cascais, Portugal, January
14-15, 2019, Manuel V. Hermenegildo and Atsushi Igarashi (Eds.). ACM, 35–46. doi:10.1145/3294032.3294082
Rodrigo Geraldo Ribeiro and André Rauber Du Bois. 2017. Certified Bit-Coded Regular Expression Parsing. In Proceedings of
the 21st Brazilian Symposium on Programming Languages, SBLP 2017, Fortaleza, CE, Brazil, September 21-22, 2017, Fabio
Mascarenhas (Ed.). ACM, 4:1–4:8. doi:10.1145/3125374.3125381
Dmitry Soshnikov. 2025. regexp-tree. https://github.com/DmitrySoshnikov/regexp-tree.
Cristian-Alexandru Staicu and Michael Pradel. 2018. Freezing the Web: A Study of ReDoS Vulnerabilities in JavaScriptbased Web Servers. In 27th USENIX Security Symposium, USENIX Security 2018. USENIX Association, 361–376. https:
//www.usenix.org/conference/usenixsecurity18/presentation/staicu
Peter Thiemann. 2016. Derivatives for Enhanced Regular Expressions. In Implementation and Application of Automata - 21st
International Conference, CIAA 2016, Seoul, South Korea, July 19-22, 2016, Proceedings (Lecture Notes in Computer Science,
Vol. 9705), Yo-Sub Han and Kai Salomaa (Eds.). Springer, 285–297. doi:10.1007/978-3-319-40946-7_24
Ken Thompson. 1968. Regular Expression Search Algorithm. Commun. ACM 11, 6 (1968), 419–422. doi:10.1145/363347.363387
Christian Urban. 2023. POSIX Lexing with Derivatives of Regular Expressions. J. Autom. Reason. 67, 3 (2023), 24. doi:10.
1007/S10817-023-09667-1
V8. 2021. An Additional Non-backtracking RegExp Engine. https://v8.dev/blog/non-backtracking-regexp.
V8. 2023. Mismatch between the Experimental engine and the backtracking engine on empty repetitions. https://issues.
chromium.org/issues/42204037.
V8. 2025. RegExp: Inconsistent branch priority in look-behind assertions. https://issues.chromium.org/issues/388290816.
Ian Erik Varatalu, Margus Veanes, and Juhan-Peep Ernits. 2023. Derivative Based Extended Regular Expression Matching
Supporting Intersection, Complement and Lookarounds. CoRR abs/2309.14401 (2023). doi:10.48550/ARXIV.2309.14401
arXiv:2309.14401
Ian Erik Varatalu, Margus Veanes, and Juhan P. Ernits. 2025. RE#: High Performance Derivative-Based Regex Matching
with Intersection, Complement, and Restricted Lookarounds. Proc. ACM Program. Lang. 9, POPL (2025), 1–32. doi:10.
1145/3704837
Xiang Wang, Yang Hong, Harry Chang, KyoungSoo Park, Geoff Langdale, Jiayu Hu, and Heqing Zhu. 2019. Hyperscan:
A Fast Multi-pattern Regex Matcher for Modern CPUs. In 16th USENIX Symposium on Networked Systems Design and
Implementation, NSDI 2019, Boston, MA, February 26-28, 2019, Jay R. Lorch and Minlan Yu (Eds.). USENIX Association,
631–648. https://www.usenix.org/conference/nsdi19/presentation/wang-xiang
Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner. 2024. Lean Formalization of Extended Regular Expression Matching
with Lookarounds. In Proceedings of the 13th ACM SIGPLAN International Conference on Certified Programs and Proofs,
CPP 2024, London, UK, January 15-16, 2024, Amin Timany, Dmitriy Traytel, Brigitte Pientka, and Sandrine Blazy (Eds.).
ACM, 118–131. doi:10.1145/3636501.3636959

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 31 -->

Formal Verification for JavaScript Regular Expressions

A

31

Correspondence between the paper and the mechanization

The following table presents a one-to-one correspondence between the paper and the code included
in the supplementary material.
Paper Definition
Section 2
Figure 1
Figure 1
Section 3
Figure 2
Figure 3
Figure 4, (l, i, gm, d) ⇓ t
List of actions l
Input i (zipper)
i2 < d i1
idx(i)
next(icheck )d
Group map gm
GM ∅
GMopen (gm, g, 𝑛)
GMclose (gm, g, 𝑛)
GMreset (gm, gl)
advance(cd, i, d)
check_anchor(a, i)
advance_bkrf (gm, g, i, d)
G(𝑟 )
lk_result(lk, tlook , gm, 𝑖)
L0 (t, i)
Theorem 1
T (l, i, gm, d, 𝑛)
||l|| id
||𝑟 || id
|i| d
worst(lk, i)
dir(lk)
Theorem 2
T (l, i, gm, d)
Theorem 3
Section 4
⇃r𝑤⇂
↿res↾
Theorem 4
Equivalence relation
between continuations
and lists of actions
Section 5
r1 ≈ r2

File

Rocq Name

Semantics/Regex.v
Semantics/Chars.v

regex, anchor, lookaround
char_descr

Semantics/Examples.v
Semantics/Tree.v
Semantics/Semantics.v
Semantics/Semantics.v
Semantics/Chars.v
Semantics/StrictSuffix.v
Semantics/Chars.v
Semantics/Chars.v
Semantics/Groups.v
Semantics/Groups.v
Semantics/Groups.v
Semantics/Groups.v
Semantics/Groups.v
Semantics/Chars.v
Semantics/Semantics.v
Semantics/Semantics.v
Semantics/Regex.v
Semantics/Semantics.v
Semantics/Tree.v
Semantics/Semantics.v
Semantics/FunctionalSemantics.v
Semantics/FunctionalSemantics.v
Semantics/FunctionalSemantics.v
Semantics/FunctionalSemantics.v
Semantics/FunctionalSemantics.v
Semantics/Regex.v
Semantics/FunctionalSemantics.v
Semantics/FunctionalUtils.v
Semantics/ComputeIsTree.v

fig2_tree
tree
is_tree l i gm d t
actions
input
strict_suffix i1 i2 d
idx i
advance_input icheck d
GroupMap.t
GroupMap.empty
GroupMap.open 𝑛 g gm
GroupMap.close 𝑛 g gm
GroupMap.reset gl gm
read_char cd i d
anchor_satisfied a i
read_backref gm g i d
def_groups 𝑟
lk_group_map lk tlook gm 𝑖
first_leaf t i
is_tree_determ
compute_tree l i gm d 𝑛
actions_fuel l i d
regex_fuel 𝑟 i d
max_iter i d
worst_input i d
lk_dir lk
functional_terminates
compute_tr l i gm d
compute_is_tree

WarblreEquiv/RegexpTranslation.v warblre_to_linden
WarblreEquiv/ResultTranslation.v to_MatchState
WarblreEquiv/EquivMain.v
equiv_main_reconstruct
WarblreEquiv/EquivDef.v
equiv_cont

Rewriting/Equivalence.v

observ_equiv

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 32 -->

32

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Regex contexts C
Rewriting/Equivalence.v
C [r]
Rewriting/Equivalence.v
Type of context C
Rewriting/Equivalence.v
L (t, i, d)
Semantics/Tree.v
ℓ1 ≡ ℓ2
Rewriting/LeavesEquivalence.v
r1 ∼d r2
Rewriting/Equivalence.v
r1 ∼↔ r2
Rewriting/Equivalence.v
Theorem 5
Rewriting/Equivalence.v
Theorem 6
Rewriting/Equivalence.v
Theorem 7
Rewriting/Equivalence.v
Theorem 8
Rewriting/Equivalence.v
Figure 6
Semantics/Example.v
Figure 5:
r1 |⟨r2 |r3 ⟩ ∼↔ ⟨r1 |r2 ⟩|r3
Rewriting/Associativity.v
r1 ⟨r2 r3 ⟩ ∼↔ ⟨r1 r2 ⟩r3
Rewriting/Associativity.v
⟨r2 |r3 ⟩r1 ∼→ ⟨r2 r1 ⟩|⟨r3 r1 ⟩
Rewriting/Distributivity.v
when r1 has no group
r1 ⟨r2 |r3 ⟩ ∼← ⟨r1 r2 ⟩|⟨r1 r3 ⟩
Rewriting/Distributivity.v
when r1 has no group
Anchors as lookarounds
Rewriting/Anchors.v
⊙
Semantics/Chars.v
r{min, 0, ⊤} ∼↔ r{min, 0, ⊥} Rewriting/ForcedQuant.v
Figure 7:
r{min1, 0, p}r{min2, 0, p} ∼↔ Rewriting/RegexpTree.v
r{min1 + min2, 0, p}
r{min1, 0, p}r{0, ∆2, ⊤} ∼→
Rewriting/RegexpTree.v
r{min1, ∆2, ⊤}
r{min1, 0, p}r{0, ∆2, ⊥} ∼→
Rewriting/RegexpTree.v
r{min1, ∆2, ⊥}
r{0, ∆1, ⊤}r{min2, 0, p} ∼←
Rewriting/RegexpTree.v
r{min2, ∆1, ⊤}
r{0, ∆1, ⊥}r{min2, 0, p} ∼←
Rewriting/RegexpTree.v
r{min2, ∆1, ⊥}
r{0, ∆1, ⊤}r{0, ∆2, ⊤} ∼↔
Rewriting/RegexpTree.v
r{0, ∆1 + ∆2, ⊤}
Chain of forward equivalences Rewriting/Chain.v
Section 6
Figure 9
Engine/PikeSubset.v
Subset of actions
Engine/PikeSubset.v
Subset of trees
Engine/PikeSubset.v
Figure 10
Engine/NFA.v
Figure 11
Engine/NFA.v
Label l
Engine/NFA.v
Accept instruction appended Engine/NFA.v
Thread (pc, gm, b)
Engine/PikeVM.v
Figure 12
Engine/PikeVM.v
States of PikeVM
Engine/PikeVM.v

, Vol. 1, No. 1, Article . Publication date: April 2026.

regex_ctx
plug_ctx C r
ctx_dir C
tree_leaves t GM ∅ i d
leaves_equiv [] ℓ1 ℓ2
tree_equiv_dir
tree_equiv
regex_equiv_ctx_samedir
regex_equiv_ctx_forward
regex_equiv_ctx_backward
observe_equivalence
different_results
disj_assoc
seq_assoc
factored_expanded_
right_equiv
factored_expanded_
left_equiv
desugar_anchor_correct
CdAll
forced_equiv
bounded_bounded_equiv
bounded_atmost_equiv
bounded_atmost_lazy_equiv
atmost_bounded_equiv
atmost_bounded_lazy_equiv
atmost_atmost_equiv
equivalence_chain
pike_regex
pike_action
pike_subtree
bytecode, code
compile
label
compilation
thread
pike_vm_step
pike_vm_state

<!-- page 33 -->

Formal Verification for JavaScript Regular Expressions

VMinit (i)
Figure 15
Figure 15a
Figure 15b
Figure 13
Figure 14, (l, i) ⊢ b
Theorem 9
Theorem 10
Figure 16
States of PikeTree
PTinit (t, i)
pts ↓↓ res
(t, gm) ↓iS res
A ↓iS res
(i, best, A, B, S) ↓ res
Theorem 11
Theorem 12
Figure 17
repcode l pc
A ∼icode A ′
S𝑉 𝑀 ⊆ A S
Figure 18
Theorem 13
Theorem 14
Theorem 15
Theorem 16

33

Engine/PikeVM.v
Engine/FunctionalPikeVM.v
Engine/FunctionalPikeVM.v
Engine/FunctionalPikeVM.v
Engine/BooleanSemantics.v
Engine/BooleanSemantics.v
Engine/BooleanSemantics.v
Engine/BooleanSemantics.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeTree.v
Engine/PikeEquiv.v
Engine/NFA.v
Engine/PikeEquiv.v
Engine/PikeEquiv.v
Engine/PikeEquiv.v
Engine/PikeEquiv.v
Engine/PikeEquiv.v
Engine/Correctness.v
Engine/Correctness.v

pike_vm_initial_state
paper_regex
paper_bytecode
paper_tree
bool_tree
bool_encoding b i l
encode_equal
booltree_istree_equiv
pike_tree_step
pike_tree_state
pike_tree_initial_state
piketreeinv pts res
tree_nd t gm i S res
list_nd A i S res
state_nd i A best B S res
init_piketree_inv
pts_preservation
tree_thread
actions_rep
list_tree_thread
seen_inclusion
pike_inv
initial_pike_inv
invariant_preservation
pike_vm_to_pike_tree
pikevm_warblre

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 34 -->

34

B

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Counterexamples for incorrect equivalences

In this section, we provide counter-examples for every distributivity or quantifier merging equivalence we have not proved in Section 5. We use the character □ to denote the hole in the context.
For counter examples of backward equivalences, we present a backward context where the □ is
inside a lookbehind. We also provide equivalent JavaScript syntax that can be copy-pasted directly
into any JavaScript engine to check that the two regexes return different results.
B.1

Distributivity

Even when r1 does not contain capture groups, there are two incorrect distributivity equivalences.
Counter-example of r1 ⟨r2 |r3 ⟩ ∼→ ⟨r1 r2 ⟩|⟨r1 r3 ⟩:
r1 = ⟨a|⟨ab⟩⟩
r2 = c
r3 = b
C =□
Input: "abc"
JavaScript syntax:
"abc".match(/(?:a|(?:ab))(?:c|b)/);
"abc".match(/(?:(?:a|(?:ab))c)(?:(?:a|(?:ab))b)/);

Counter-example of ⟨r2 |r3 ⟩r1 ∼← ⟨r2 r1 ⟩|⟨r3 r1 ⟩ :
r1 = ⟨c|⟨bc⟩⟩
r2 = a
r3 = b
C = abc(?<= (1 □))\1
JavaScript syntax:

Input: "abcabc"

"abcabc".match(/abc(?<=((?:a|b)(?:c|(?:bc))))\1/);
"abcabc".match(/abc(?<=((?:a(?:c|(?:bc))|(?:b(?:c|(?:bc))))))\1/);

B.2

Quantifier Merging

Counter-example of r{0, ∆1, ⊤}r{min2, 0, p} ∼→ r{min2, ∆1, ⊤}:
r = ⟨a|⟨ab⟩⟩
Δ1 = 1
min2 = 1
C =□
JavaScript syntax:

Input: "aba"

"aba".match(/(?:a|(?:ab)){0,1}(?:a|(?:ab)){1}/);
"aba".match(/(?:a|(?:ab)){1,2}/);

Counter-example of r{0, ∆1, ⊥}r{min2, 0, p} ∼→ r{min2, ∆1, ⊥}:
r = ⟨⟨ab⟩|⟨aba⟩⟩
Δ1 = 1
min2 = 1
C = □⟨⟨bcd⟩|c⟩
JavaScript syntax:

Input: "ababcd"

"ababcd".match(/(?:(?:ab)|(?:aba)){0,1}?(?:(?:ab)|(?:aba)){1}?(?:(?:bcd)|c)/);
"ababcd".match(/(?:(?:ab)|(?:aba)){1,2}?(?:(?:bcd)|c)/);

Counter-example of r{min1, 0, p}r{0, ∆2, ⊤} ∼← r{min1, ∆2, ⊤}:
r = ⟨a|⟨ba⟩⟩
min1 = 1
Δ2 = 1
C = caba(?<= ⟨⟨cab⟩|c⟩(1 □))\1
JavaScript syntax:

Input: "cabaa"

"cabaa".match(/caba(?<=(?:(?:cab)|c)((?:a|(?:ba)){1}(?:a|(?:ba)){0,1}))\1/);
"cabaa".match(/caba(?<=(?:(?:cab)|c)((?:a|(?:ba)){1,2}))\1/);

Counter-example of r{min1, 0, p}r{0, ∆2, ⊥} ∼← r{min1, ∆2, ⊥}:
r = ⟨⟨ba⟩|⟨aba⟩⟩ min1 = 1 Δ2 = 1 C = cbaba(?<= ⟨c|⟨cb⟩⟩(1 □))\1
JavaScript syntax:

Input: "cbabababa"

"cbabababa".match(/cbaba(?<=(?:c|(?:cb))((?:(?:ba)|(?:aba)){1}(?:(?:ba)|(?:aba)){0,1}?))\1/);
"cbabababa".match(/cbaba(?<=(?:c|(?:cb))((?:(?:ba)|(?:aba)){1,2}?))\1/);

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 35 -->

Formal Verification for JavaScript Regular Expressions

35

Counter-example of r{0, ∆1, ⊥}r{0, ∆2, ⊥} ∼→ r{0, ∆1 + ∆2, ⊥}:
r = ⟨⟨ab⟩|⟨aba⟩⟩
Δ1 = 1
Δ2 = 1
C = □⟨b|c⟩
JavaScript syntax:

Input: "ababc"

"ababc".match(/(?:(?:ab)|(?:aba)){0,1}?(?:(?:ab)|(?:aba)){0,1}?(?:b|c)/);
"ababc".match(/(?:(?:ab)|(?:aba)){0,2}?(?:b|c)/);

Counter-example of r{0, ∆1, ⊥}r{0, ∆2, ⊥} ∼← r{0, ∆1 + ∆2, ⊥}:
r = ⟨⟨ba⟩|⟨aba⟩⟩
Δ1 = 1
Δ2 = 1
C = cbaba(?<= ⟨c|⟨cb⟩⟩(1 □))\1
JavaScript syntax:

Input: "cbabababa"

"cbabababa".match(/cbaba(?<=(?:c|(?:cb))((?:(?:ba)|(?:aba)){0,1}?(?:(?:ba)|(?:aba)){0,1}?))\1/);
"cbabababa".match(/cbaba(?<=(?:c|(?:cb))((?:(?:ba)|(?:aba)){1,2}?))\1/);

B.3

Checking these counter-examples in other languages

The distributivity and quantifier merging counter-examples above should hold for any regex
language following backtracking semantics, when their feature set supports it. We have considered
the following popular regex languages: .NET, PCRE2, PCRE, Python, Go, Rust, and Java.
Forward counter-examples. All of these regex languages support all features used in our forward
counter-examples. We checked that all our forward counter-examples are indeed counter-examples
for all these languages.
Backward counter-examples. Our backward counter-examples use features that are not supported
in all languages. First, Go and Rust support neither backreferences nor lookarounds. Then, PCRE2,
PCRE and Python disallow lookbehinds (?<= r) when the length of all strings matched by r is not
known to be constant (which is not the case in our counter-examples). As a result, our backward
counter-examples are not applicable in Go, Rust, PCRE2, PCRE, and Python.
.NET supports all of these features, and we checked that all our backward counter-examples are
indeed counter-examples for .NET.
Finally, while Java regexes support lookbehinds, they do not follow backtracking semantics when
inside lookbehinds (but they do inside lookaheads). Instead of matching in reverse like JavaScript or
.NET, when matching a lookbehind (?<= r) from a position 𝑝 in a string, the matching algorithm
implemented in OpenJDK first tries to match r from position 𝑝, then from position 𝑝 − 1, then
𝑝 − 2. . . As a result, the shortest match in a lookbehind has priority in Java regexes, and our
backward counter-examples are not applicable. To illustrate these differences in semantics, consider
matching the regex abc(?<= (1 abc|c|bc))x on string "abcx" in Java: it returns that the capture
group 1 matched the substring "c" (the middle branch of the disjunction).

, Vol. 1, No. 1, Article . Publication date: April 2026.

<!-- page 36 -->

36

C

Aurèle Barrière, Victor Deng, and Clément Pit-Claudel

Small-step semantics of the PikeTree algorithm

Figure 19 describes the full small-step semantics rules of the PikeTree algorithm (of which a few
selected rules were depicted on Figure 16).
(i, best, [], [], S) → PTend (best)

Final

next(i) = Some i′
B ≠ []
NextChar
′
(i, best, [], B, S) → (i , best, B, [], ∅)
t∈S
Skip
(i, best, (t, gm) :: A, B, S) → (i, best, A, B, S)
S ′ = S ∪ {Match}
Match
(i, best, (Match, gm) :: A, B, S) → (i, Some (i, gm), [], B, S ′ )
S ′ = S ∪ {Read c t}
Blocked
(i, best, (Read c t, gm) :: A, B, S) → (i, best, A, B ++ [(t, gm)], S ′ )
S ′ = S ∪ {Choice t1 t2 }
Choice
(i, best, (Choice t1 t2, gm) :: A, B, S) → (i, best, (t1, gm) :: (t2, gm) :: A, B, S ′ )
S ′ = S ∪ {Progress t}
Progress
(i, best, (Progress t, gm) :: A, B, S) → (i, best, (t, gm) :: A, B, S ′ )
S ′ = S ∪ {Open g t}
GMopen (gm, g, idx(i)) = gm′
Open
(i, best, (Open g t, gm) :: A, B, S) → (i, best, (t, gm′ ) :: A, B, S ′ )
S ′ = S ∪ {Close g t}
GMclose (gm, g, idx(i)) = gm′
Close
(i, best, (Close g t, gm) :: A, B, S) → (i, best, (t, gm′ ) :: A, B, S ′ )
S ′ = S ∪ {Reset gl t}
GMreset (gm, gl) = gm′
Reset
(i, best, (Reset gl t, gm) :: A, B, S) → (i, best, (t, gm′ ) :: A, B, S ′ )
Fig. 19. PikeTree small-step semantics

, Vol. 1, No. 1, Article . Publication date: April 2026.
