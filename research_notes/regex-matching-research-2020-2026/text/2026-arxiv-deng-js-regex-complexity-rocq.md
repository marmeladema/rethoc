---
file: ../pdfs/2026-arxiv-deng-js-regex-complexity-rocq.pdf
citation: "On the Computational Complexity of JavaScript Regex Matching, in Rocq — Deng, Barrière, Pit-Claudel — arXiv 2603.26139, 2026"
doi: 
source: "https://arxiv.org/pdf/2603.26139"
topics: [counting_and_derivatives, redos_backrefs_verification, semantics_theory_access]
pages: 37
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

On the computational complexity of JavaScript
regex matching, in Rocq
Victor Deng #
EPFL, Switzerland

Aurèle Barrière #
CNRS, ENS de Lyon, Inria, Université Claude Bernard Lyon 1, LIP, France

Clément Pit-Claudel #
EPFL, Switzerland

arXiv:2603.26139v2 [cs.PL] 22 Sep 2026

Abstract
Despite widespread use, the complexity of the matching problem for modern regular expressions
languages remains unclear. Previous work proved that an idealized regular expression language
with backreferences and lookarounds had PSPACE-complete matching. We extend this work to a
real-world regex language by proving that JavaScript regex matching with expanded lower-bounded
quantifiers is PSPACE-complete. We then generalize the result: we show that PSPACE-hardness
survives the removal of negative lookarounds, and that removing all lookarounds leads to an
OptP-complete parsing problem. Our core arguments are formalized in Rocq.
2012 ACM Subject Classification Theory of computation → Complexity classes; Theory of computation → Logic and verification
Keywords and phrases Regex, Complexity, JavaScript, Rocq, Formal Verification
Acknowledgements The authors would like to thank Ola Svensson, Mika Göös and A. R. Balasubramanian for helpful discussions about this paper. This research was funded in whole or in part by
the Swiss National Science Foundation (SNSF), grant number 10003649.

<!-- page 2 -->

2

On the computational complexity of JavaScript regex matching, in Rocq

1

Introduction

Modern regular expressions, or regexes, enjoy widespread use, such as in over 30% of Python
and JavaScript packages [6]. Some languages and libraries (Rust-lang/Regex, Go, RE2) adopt
a regex language that permits polynomial-time matching. Others include more advanced
features, such as backreferences and atomic groups. For the different combinations of features
used in these languages, the computational complexity of the regex-matching problem is still
unknown: while results exist for idealized regex languages, no work has characterized the
complexity of matching real-world regexes. To make things worse, real-world regex languages
often disagree on the semantics of common features [7].
Two facts are known with respect to real-world regex matching complexity. First,
backreferences make the problem NP-hard (by reduction from 3-SAT). While traditional
regular expressions extended with backreferences alone are also NP-complete [1], online
sources often incorrectly generalize this NP-completeness claim for languages that contain
more features [9]. Second, JavaScript regex matching without backreferences nor bounded
quantifiers1 is in P, even with lookarounds [3].
More definitive results are also known for theoretical regex languages, without being
directly applicable to real-world ones. For instance, an extension of traditional regular expressions with backreferences and lookarounds was recently shown to be PSPACE-complete [31].
This result is proved for a non-backtracking semantics that allows duplicate groups and
does not have atomic lookarounds, whereas JavaScript regex matching is specified by a
backtracking algorithm, has constrained duplicate capture groups, and atomic lookarounds.
In this paper, we focus on JavaScript regex matching, because JavaScript regexes are
popular and have a well-defined specification, in the form of a dedicated chapter in the
ECMAScript specification of JavaScript [10, 11, 12]. We clarify the computational complexity of JavaScript regex matching and present new results, with mechanized proofs. Our
contributions are the following:
We adapt the result of Uezato [31] about PSPACE-hardness of regex matching with
lookarounds and backreferences to the real-world JavaScript regexes: we prove a reduction
of a variant of the PSPACE-complete problem QBF to JavaScript regex matching.
We show that negative lookarounds, which are constructs that allow negating the results
of regex matching, are in fact not essential to PSPACE-hardness of JavaScript regex
matching: we prove a new reduction of the same variant of QBF to JavaScript regex
matching without negative lookarounds. To do so, we show that in JavaScript (and in
most modern regex languages), positive lookarounds are unexpectedly powerful, and are
in fact sufficient to encode a limited form of negation.
We show that JavaScript regex matching with expanded lower-bounded quantifiers is in
PSPACE.
We show that JavaScript regex parsing without lookarounds (and with expanded lowerbounded quantifiers) is OptP-complete. In the problem of regex parsing, the output is
how the regex matched the string.
We mechanize, in the Rocq proof assistant, the core arguments of all of these results.
Our Rocq proofs about JavaScript regexes are conducted on a preexisting mechanized
semantics of JavaScript regular expressions [2]. In Appendix G, we show how our proofs
have also been ported to an equivalent preexisting Rocq semantics, that is a direct manual

1

Bounded quantifiers are quantifiers that specify their minimum and maximum number of iterations.

<!-- page 3 -->

V. Deng, A. Barrière and C. Pit-Claudel

3

line-by-line translation of the ECMAScript regex chapter [8]. Accordingly, our Rocq proofs
ensure that our results are directly applicable to a real-world regex language, despite its
complex backtracking semantics. To our knowledge, this is the first time that complexity
class results are proven about a real-world regex specification and that such proofs are
mechanized in a proof assistant.
The rest of this paper is organized as follows. Section 2 provides background about JavaScript regexes and the semantics we use. In the next two sections, we give key intuitions and
examples for our results, and refer to the corresponding appendices and the Rocq development, available at https://github.com/LindenRegex/js-regex-matching-complexity,
for details. Section 3 shows that JavaScript regex matching is PSPACE-complete. Section
4 shows that JavaScript regex parsing without lookarounds is OptP-complete. Section 5
discusses related work, Section 6 discusses our findings, its limitations and future work, and
Section 7 concludes the paper.

2

Background

2.1

JavaScript regular expressions

Modern regular expressions, such as those found in JavaScript, include many features: besides
the traditional characters, concatenation, disjunction, Kleene star and Kleene plus, one can
find anchors, capture groups, lookarounds and backreferences, among others. We focus here
on JavaScript regular expressions, whose features (as of the 2023 edition of the ECMAScript
specification [10]) are listed in Figure 1.
r

::=
|
|
|
|
|
|
|
|

ε
c
r1 |r2
r 1 r2
(g r)
⟨r⟩
\g | \k⟨g⟩
r∗
(?! r)

Empty
Character
Disjunction
Sequence
Capture group
Non-capturing group
Backreference
Star
Negative lookahead

|
|
|
|
|
|
|
|

(?= r)
^ | $ | \b | \B
[c0 c1 − c2 ]
[ˆc0 c1 − c2 ]
\w | \W | ...
(?<= r) | (?<! r)
r{min, ∆, ⊤}
r{min, ∆, ⊥}

Positive lookahead
Anchors
Character descriptors
Lookbehinds
Greedy quantifiers
Lazy quantifiers

Figure 1 JavaScript regex syntax (adapted for presentation purposes). Left: subset of constructs
used in our proof of PSPACE-hardness. Right: other regex constructs. Figure adapted from [2].

The syntax of JavaScript regexes described in Figure 1 is adapted for presentation
purposes as in [2]. Notably:
We annotate all capture groups with a group index or group name instead of relying on
the implicit numbering from left to right of their opening parentheses. For instance, the
original JavaScript regex (a)(b)cd corresponds to the regex (1 a)(2 b)cd in our syntax.
We use a different syntax for quantified regexes. In r{min, ∆, p}, min is the minimum
number of repetitions, ∆ (possibly infinite) corresponds to the difference between the
maximum and the minimum number of repetitions, and p = ⊤ (resp. p = ⊥) denotes a
greedy (resp. lazy) quantifier.
The JavaScript regex features of interest that are not traditional regex features are as
follows. A capture group records the part of the input string that was last matched by a
subregex. For instance, matching the regex a(1 b∗)c on string "abbbc" returns the entire

<!-- page 4 -->

4

On the computational complexity of JavaScript regex matching, in Rocq

string as well as the substring "bbb" corresponding to capture group 1. If a capture group
has not been matched, an engine should return an undefined value: Undef . Capture groups
can optionally be named. A lookaround is an assertion that checks whether a regex matches
the next (lookahead) or previous (lookbehind) part of the string; as an assertion, it does
not consume characters in the string during matching. For instance, matching the regex
a(?= bc)b on the string "abc" returns the string "ab": indeed, the lookahead assertion
(?= bc) succeeds when matched at position 1 (between the a and the b), since it is followed
by the string "bc", but the matching process then continues by matching the remaining
regex b starting from position 1. Lookarounds can be negative, in which case they succeed
if and only if the underlying regex does not match the string at the current position. For
instance, matching the regex a(?! bc)b on "abc" does not succeed. Finally, a backreference
matches a substring that was previously recorded by a capture group. For instance, the regex
(1 ab∗c)\1 matches any string of the form "ab...bc" repeated twice; it matches the string
"abbcabbc" (with capture group 1 set to "abbc") but not the string "abbcabc". When a
backreference refers to a group that is undefined, it matches the empty string.

Regex size in the presence of quantifiers
When measuring the size of a regular expression, we count lower-bounded quantifiers as
if they had been expanded away: for example, we treat r + as r r ∗ (this is equivalent to
restricting ourselves to regexes without lower-bounded quantifiers). More precisely, we define
the size function as follows:
|ε| = |cd| = 1
|a| = |\g| = 1
|⟨r1 |r2 ⟩| = |r1 r2 | = 1 + |r1 | + |r2 |

|r{min, ∆, p}| = (1 + min) × (3 + |r|)
|(?lk r)| = 1 + |r|
|(g r)| = 2 + |r|

Figure 2 Definition of the regex expanded size. Nonlinearity in terms of AST size highlighted.

Semantics
JavaScript regexes follow backtracking semantics (also called leftmost-greedy), meaning that
the result is specified by a backtracking algorithm. This backtracking algorithm tries matching
the regex from each starting position from left to right, and within a starting position, it
prioritizes the left branch of each disjunction, and only tries the right branch if the left
branch fails to find a match.

2.2

The Backtracking Tree Semantics for JavaScript regular expressions

In this work, we build on the inductive big-step semantics of JavaScript regexes presented
in [2]. Intuitively, it relates a regex and a string to a backtracking tree, which describes the
steps and choices performed by a backtracking algorithm, but without stopping at the first
match as prescribed by the backtracking semantics. The branches of that tree are ordered:
the leftmost branch corresponds to the path that would be explored first by the backtracking
algorithm. More precisely, the inductive statement of the semantics has the following form:
(l, i, gm, d) ⇓ t where l is a list of actions left to be performed to finish matching, i is the
current input (an input string and a position within the string), gm is a group map that stores
the optional values of each capture group in the regex, d is a matching direction (forward
or backward) and t is a backtracking tree. There are three types of actions: matching a

<!-- page 5 -->

V. Deng, A. Barrière and C. Pit-Claudel

5

regex r, which is simply denoted by r, closing a capture group g, denoted by Aclose g, and
checking for progress with respect to a certain input i , denoted by Acheck i (to implement
the JavaScript nullable quantifier semantics, which forbids optional iterations of a quantifier
to match the empty string). Initially, the list of actions contains only the input regex.
([], i, gm, d) ⇓ Match

Match

(l, i, GMclose (gm, g, idx(i)), d) ⇓ t
Close
(Aclose g :: l, i, gm, d) ⇓ Close g t

Open 1
Choice

(r1 :: l, i, gm, d) ⇓ t1
(r2 :: l, i, gm, d) ⇓ t2
Disj
((r1 |r2 ) :: l, i, gm, d) ⇓ Choice t1 t2
(r1 :: r2 :: l, i, gm, →) ⇓ t
SeqForward
(r1 r2 :: l, i, gm, →) ⇓ t

Read a

Read a

Close 1

Read b

Mismatch

Close 1
Read c

(l, i, gm, d) ⇓ tskip
(r :: Acheck i :: r ∗ :: l, i, GMreset (gm, G(r)), d) ⇓ titer
(r ∗ :: l, i, gm, d) ⇓ Choice (Reset G(r) titer ) tskip

Match
GreedyStar

Figure 3 Left: examples of inductive rules of the semantics defined in [2]. Right: backtracking
tree of regex (1 a|ab)c on string "abc".

Figure 3 (left) presents a few inductive rules of the semantics, where GMclose and GMreset
manipulate the group map. For instance, rule Match states that the backtracking tree
corresponding to no action at all is simply a Match node. Rule Close states that the
backtracking tree for the list of actions Aclose g :: l is a Close g t node, whose subtree t is
the backtracking tree corresponding to some list of actions l. Finally, the simplified rule
GreedyStar creates two branches in the tree, under a Choice node. The left branch titer
corresponds to iterating the greedy star, while the right one tskip simply ignores the star and
matches the remaining actions l. Inside the actions of the left branch, an action Acheck i is
inserted to check that in that iteration, progress is made with respect to the original input i.
We refer the reader to [2] for the full, precise semantics.
For instance, the backtracking tree of the regex (1 a|ab)c and the string "abc" is shown
on Figure 3 (right), where the Choice node is created by the disjunction.
This backtracking tree semantics allows to recover the result of JavaScript regex matching,
by considering the branches of the tree that end on a Match node. For each of these branches,
one can compute a match result, called a leaf [2]: a pair of an input and a group map. For
instance, in Figure 3 (right), there is only one such branch. In its associated leaf, the input is
at position 3 (there are 3 Read nodes on that branch), and the group map contains a group 1,
that has been opened at position 0, and closed at position 2. The first leaf of a backtracking
tree is the leaf associated with the leftmost branch that ends on a Match node: it is the
match of highest priority and corresponds to the result of JavaScript regex matching.

3

JavaScript regex matching is PSPACE-complete

In this section, we show that the decision version of JavaScript regex matching is PSPACEcomplete. We call this decision problem JSRegMatch:
Input: A regex r and a string s.
Problem: Does r have an anchored match on s?

<!-- page 6 -->

6

On the computational complexity of JavaScript regex matching, in Rocq

An anchored match of r on s is a match of r on s that starts at the beginning of s.
In JavaScript, one can request an anchored match by setting the sticky (y) flag to true.
We prove that JSRegMatch is PSPACE-hard (Section 3.1), and that it is in PSPACE
(Section 3.3). We discuss how to extend our result to unanchored matching in Appendix H.

3.1

JavaScript regex matching is PSPACE-hard

In this section, we adapt the result of Uezato [31] about PSPACE-hardness of regex matching
with lookarounds and backreferences to JavaScript regex matching. As in Uezato’s work, we
prove PSPACE-hardness of JSRegMatch by reducing a variant of the PSPACE-complete
problem QBF (Quantified Boolean Formula) to it.

QBF and variants
QBF is the canonical PSPACE-complete problem [17]. The input to this problem is a
quantified boolean formula, which is a formula that comprises variables x, y, logical connectors
¬, ∧ and ∨ and quantifiers ∀ and ∃, and where all variables are bound by a quantifier. Its
output is whether the quantified boolean formula is true or false. For instance, ∀x.∃y.x ∨ y
is a true QBF (take for instance y = ⊤), while ∀x.∃y.x is a false QBF (take x = ⊥).
It is well-known that the PCNF-QBF variant is also PSPACE-complete. In this variant,
the quantified boolean formula is in prenex form: all quantifiers are at the beginning of the
formula, and its underlying propositional formula is in conjunctive normal form.
Finally, as in Uezato’s work [31], we consider another variant QBF∃ of the problem
PCNF-QBF, which is also PSPACE-complete. In this variant, the only quantifiers are ∃ and
¬∃, the underlying propositional formula is in conjunctive normal form (like in PCNF-QBF)
and, unlike in PCNF-QBF, the underlying propositional formula can be negated.
QBF∃ is in PSPACE because QBF is, and it is PSPACE-hard because one can easily
reduce PCNF-QBF to it in linear time, by replacing ∀x. quantifiers by ¬∃x.¬ and eliminating
double negations. We prove this reduction from PCNF-QBF to QBF∃ correct in Rocq:
▶ Theorem 1. Let q be a PCNF-QBF formula and q ′ its translation into a QBF∃ formula.
Then q ′ and q have the same size, and q is true if and only if q ′ is true.

Our reduction from QBF∃ to JSRegMatch
In this section, we explain our reduction from QBF∃ to JSRegMatch using an example.
The example is as follows: q is a QBF∃ formula, and we build the regex r and the string s.
q = ∃x1 .¬∃x2 .(x1 ∨ ¬x2 ) ∧ x2

r = ⟨(1 a)|a⟩;(?! ⟨(2 a)|a⟩;⟨\1|\2a⟩;\2;)

s = "a;a;a;a;z"

With our reduction, the backtracking regex matching process simulates a naive QBF∃
checking algorithm. As the regex is being matched, the group map models a valuation
v : {x1 , x2 } → {⊤, ⊥} being tested, by mapping each variable xi , i ∈ {1, 2} to either the
string "a" (for ⊤) or Undef , that a backreference will treat as the empty string "" (for ⊥).
The assignment fragments of the regex, ⟨(i a)|a⟩; (for i = 1, 2) force the backtracking semantics
to consider all the assignments of the capture groups 1 and 2 to "a" or "", effectively testing
all possible valuations. The core fragment, ⟨\1|\2a⟩;\2;, when matched against the substring
"a;a;", checks whether the current valuation makes the CNF (x1 ∨ ¬x2 ) ∧ x2 true. To do so,
each subfragment ⟨\1|\2a⟩; and \2; must match the substring "a;". In these subfragments,
each disjunct corresponds to one literal, with a backreference \i matching the string "a" if
and only if variable xi is true and a backreference \ia matching the string "a" if and only if

<!-- page 7 -->

V. Deng, A. Barrière and C. Pit-Claudel

variable xi is false. Finally, the negative lookahead encodes the negated existential quantifier.
After the first assignment fragment, the regex (?! ⟨(2 a)|a⟩;⟨\1|\2a⟩;\2;) matches "a;a;a;" if
and only if the current valuation makes the partial QBF∃ ¬∃x2 .(x1 ∨ ¬x2 ) ∧ x2 true.
The final z in the string is not used in the reduction, it is only here to ease proof sharing
with the next subsection. It has no incidence on matching, since a regex does not need to
match the entire string to have an anchored match on that string.
We prove in Rocq the following theorem, which proves the correctness of the reduction
from QBF∃ to JavaScript regex matching:
▶ Theorem 2. Let q be a QBF∃ formula, and (r, s) the regex and the string that result from
the translation of q into an instance of regex matching. Then r has an anchored match on s
if and only if q is true.
We refer the reader to Appendix A.1 for the formal construction.

3.2

JSRegMatch is PSPACE-hard, even without negative lookarounds

We found that negative lookarounds, while the most natural way of modeling negation, were
not essential to do so. Instead, we leverage the atomicity of lookarounds in JavaScript (and
other regex languages) to provide a reduction from QBF∃ to JavaScript regex matching
without negative lookarounds. This atomicity means that lookarounds act as a backtracking
frontier: when matching a regex (?= A)B, if the backtracking semantics finds some match
for the regex A but then fails to find a match for the regex B, it will not backtrack inside the
lookahead to try another match for A. For instance, there is no match of (?= (1 a|ab))\1c on
the string "abc": even though taking the second branch of the lookahead would set group 1
to "ab" and allow \1c to match, this branch is never visited.
We combine this atomicity with the fact that with backtracking semantics, when matching
a regex A|B, if we know that B matched, then we also know that A did not match. As
a consequence, one can test whether a regex A does not match a string s by matching a
regex of the form (?= A|B)C , where the regex C checks whether the regex B was matched
in the lookahead. If B was matched, it means that A did not match, since otherwise the
backtracking semantics would not consider matching B. Conversely, if B did not match in
the lookahead, it means that A did match.
We explain our new negative-lookbehind-free reduction using the following example:
q = ∃x1 .¬∃x2 .x1 ∨ x2

r = ⟨(1 a)|a⟩;(?= ⟨(2 a)|a⟩;⟨\1|\2⟩;|⟨a; ⟩∗(z z))⟨a; ⟩∗\k⟨z⟩$
s = "a;a;a;z"

The role of the assignment fragments ⟨(i a)|a⟩; remains the same as in Section 3.1: the
backtracking semantics has to consider all valuations of xi . The core fragment ⟨\1|\2⟩; keeps
its purpose as well: it matches the string "a;" if and only if the current valuation makes the
CNF x1 ∨ x2 true. What changes is the way negation is modeled. Consider a group map
gm that has group 1 set to either "a" or Undef and all other groups undefined, and the
corresponding agreeing valuation v. Assume that the first assignment fragment ⟨(1 a)|a⟩; has
been matched against the string "a;", and we have a current valuation v for the variable x1 .
Assume that v makes the partial QBF∃ ∃x2 .x1 ∨x2 true. Then the sub-regex ⟨(2 a)|a⟩;⟨\1|\2⟩;
matches the string "a;a;" and the positive lookahead (?= ⟨(2 a)|a⟩;⟨\1|\2⟩;|⟨a; ⟩∗(z z))
matches (the empty string) leaving group z undefined; consequently, the fragment
⟨a; ⟩∗\k⟨z⟩$ does not have an anchored match in the string "a;a;z". Because of the

7

<!-- page 8 -->

8

On the computational complexity of JavaScript regex matching, in Rocq

atomicity of lookaheads, the backtracking semantics will not backtrack inside the lookahead and match the other branch ⟨a; ⟩∗(z z) against the string "a;a;z". The matching
process therefore fails to find a match.
Assume conversely that v makes the partial QBF∃ ∃x2 .x1 ∨ x2 false. Then the sub-regex
⟨(2 a)|a⟩;⟨\1|\2⟩; does not have a match on the string "a;a;z", and the positive lookahead
(?= ⟨(2 a)|a⟩;⟨\1|\2⟩;|⟨a; ⟩∗(z z)) matches with group z set to the string "z"; consequently,
the fragment ⟨a; ⟩∗\k⟨z⟩$ matches the string "a;a;z". The matching process therefore
succeeds in finding a match.
Finally, the matching process finds a match if and only if v makes the partial QBF∃ ∃x2 .x1 ∨x2
false, effectively negating the output of matching for the sub-regex ⟨(2 a)|a⟩;⟨\1|\2⟩;.
We prove in Rocq the following theorem, that proves the correctness of the reduction
from QBF∃ to JavaScript regex matching:
▶ Theorem 3. Let q be a QBF∃ formula. Let (r, s) be the regex and string that result from
the translation of q into a regex without negative lookarounds and a string. Then r has an
anchored match on s if and only if q is true.
We refer the reader to Appendix A.2 for the full formal construction.

3.3

JavaScript regex matching is in PSPACE

To prove PSPACE membership of JSRegMatch, we present a backtracking algorithm. This
algorithm relies on a partial function, compute_result (l, i, gm, d, fuel), that computes the
first leaf of the tree t such that (l, i, gm, d) ⇓ t. The function is presented in its entirety in
Appendix B. The function is partial: the fuel argument strictly decreases at each recursive
call, and the function only returns a result when the recursion depth does not exceed the
initial fuel. This function is directly inspired by the functional semantics of Barrière et al. [2],
but returning the first leaf instead of the full tree.
An example of the recursive definition is shown below:
compute_result ((r1 |r2 ) :: l) i gm d (fuel + 1) =
match compute_result (r1 :: l) i gm d fuel with
| Out_of_fuel ⇒ Out_of_fuel
| Success lf ⇒ Success lf
| NoMatch ⇒ compute_result (r2 :: l) i gm d fuel
end

To show that we can solve JSRegMatch using polynomial space, we find and prove
polynomial bounds (in the sizes of the regex and the input string) both for the size of
the arguments of compute_result, and also for an amount of initial fuel that is sufficient
for the partial function to always return a result. We present these bounds in Appendix
B. These bounds are sufficient to show that the algorithm uses polynomial space: the
function execution only stores the current arguments and a call stack. The arguments require
polynomial space, and the call stack contains at most a polynomial number of stack frames,
each using polynomial space to store function arguments.
The key difficulty resides in polynomially bounding the recursion depth of compute_result.
Barrière et al. [2] previously presented a recursion depth bound for their functional semantics,
but their bound is exponential in the size of the AST of the regex. Intuitively, they defined a
fuel function ||r||i , over-approximating the depth of the backtracking tree of regex r with
input string i. For the greedy star r ∗, they relied on the following observation: since each
iteration of the star either reads a character or ends up being discarded by a failed progress

<!-- page 9 -->

V. Deng, A. Barrière and C. Pit-Claudel

check, then there can be at most |i| + 1 iterations of r. They consequently defined ||r ∗||i as
(||r||i + 2) × (|i| + 1). This formula can be generalized to quantifiers with forced iterations
(r{min, ∆, b} when min > 0) by adding min to the number of iterations. The issue with this
multiplication happens when stars are nested: ||r ∗∗||i is quadratic, and the fuel of n nested
stars is Ω(|i|n ).
To avoid this exponential complexity, we define a new, more complex fuel formula that
leverages the following new observation: every valid non-forced iteration of a nested quantifier
consumes a character, and therefore decreases the number of possible non-forced iterations of
outer quantifiers. In other words, when matching r ∗∗ on input i, the subregex r can only be
matched |s| + 1 times. The formulae for the bounds and a proof of adequacy of the bounds
can be found in Appendix C. We prove in Rocq the following theorems:
▶ Theorem 4. Let r be a regex and s be an input string. Let ||r||s = 1 + (1 + |s|)(|r| + |r|2 ),
where |r| is the expanded size of r (Figure 2). Then running the backtracking algorithm with
r, s and ||r||s succeeds, yielding a boolean result that matches the result of the instance (r, s)
of JSRegMatch.
▶ Theorem 5. Let r be a regex. Let l, i, d be a list of actions, input and matching direction
that can result from matching r on a string. Consider matching the list of actions l on the
input i in direction d using the backtracking algorithm. Let sf = (1 + |i|)|l| be an upper bound
on the memory usage of a stack frame of the backtracking algorithm, and P = (1+|i|)(|r|+|r|2 )
be a polynomial in the regex and input sizes. Then ||r||i × sf ≤ P (P + 1).

4

JavaScript regex parsing without lookarounds is OptP-complete

In this section, we prove that JavaScript regex parsing without lookarounds is OptP-complete.
OptP [22] is the following class of optimization problems:
▶ Definition 6. Let Σ be an alphabet with {0, 1} ⊆ Σ. A function f : Σ∗ → N is in OptP if
and only if there exists a nondeterministic Turing machine M that runs in polynomial time,
on each branch writes a binary number and accepts, and such that for all x ∈ Σ∗ , f (x) is the
maximum (for a maximization problem) number written by a branch of M on input x.
JavaScript regex parsing can be defined as the following problem:
Input: A regex r and a string s.
Output: a binary number with nb (r, s) + 1 bits, where nb (r, s) = (1 + |s|) × |r| + 1. If
there is an anchored match of r on s, then the list of binary backtracking choices made
while matching the regex against the string, preceded by a 1, padded to the right using ones;
otherwise, 0 followed by nb (r, s) ones. A 1 means taking the higher-priority branch, a 0
means taking the lower-priority branch. Most significant bit first.
This can therefore be viewed as a function from the set of pairs of a regex and a string
to the set of natural numbers. The output binary number encodes how the regex matched
the string, and in particular which branch of each disjunction was taken and the iterating
decisions made for each quantifier. Therefore, the range matched by each subregex can be
recovered from this number, the regex and the string.

4.1

JavaScript regex parsing without lookarounds is OptP-hard

In this section, we prove that JavaScript regex parsing without lookarounds is OptP-hard
by metrically reducing a variant of the OptP-complete problem Lexicographic SAT (called

9

<!-- page 10 -->

10

On the computational complexity of JavaScript regex matching, in Rocq

Maximum Satisfying Assignment in [22]) to JavaScript regex parsing. Lexicographic SAT
is the following optimization problem:
Input: A propositional formula φ. It must use exactly all variables from x1 to xn for
some n ∈ N.
Output: The lexicographically maximum assignment of variables from x1 to xn that
makes φ true, or ⊥ if there is no such assignment.
Without loss of generality, we force the propositional formula to not contain fictive
variables, i.e. variables that would be present in assignments but absent from the formula
[26]. Indeed, in the presence of such variables, one can use an increasing renaming ρ of
the non-fictive variables to some contiguous range {1, ..., n′ }, solve the new instance of
Lexicographic SAT obtained, then rename the variables of the result using ρ−1 and (if
desired) set the fictive variables to 1 in the result.
In the variant CNF Lexicographic SAT of Lexicographic SAT that we consider,
the formula is in conjunctive normal form. This variant is also OptP-hard [26, Thm. 2.1].
A metric reduction is defined by Krentel [22] as follows:
▶ Definition 7. Let f, g : Σ∗ → N. A metric reduction from f to g is a pair of polynomialtime computable functions T1 : Σ∗ → Σ∗ and T2 : Σ∗ × N → N such that for all x ∈ Σ∗ ,
f (x) = T2 (x, g(T1 (x))).
Essentially, to reduce a function f to a function g, one must be able for each instance x ∈ Σ∗
to compute in polynomial time an instance T1 (x) such that one can retrieve in polynomial
time f (x) from g(T1 (x)) and x.
The reduction from CNF Lexicographic SAT to JavaScript regex parsing is essentially
the same as in Section 3.1, in the special case where all the quantifiers are ∃ (no ¬∃ quantifier).
Below is an example:
φ = (x1 ∨ ¬x2 ) ∧ x2

r = ⟨(1 a)|a⟩;⟨(2 a)|a⟩;⟨\1|\2a⟩;\2;

s = "a;a;a;a;z"

JavaScript’s backtracking semantics makes the backtracking engine consider the assignments of capture groups 1 and 2 to "a" or Undef in reverse lexicographic order. Indeed,
the left branch of each disjunction ⟨(i a)|a⟩; is tried first, and that branch corresponds to
assigning variable xi to the string "a". After the groups have been set up, as shown in
Section 3.1, the core of the regex ⟨\1|\2a⟩;\2; has an anchored match on the string "a;a;z"
if and only if the agreeing valuation makes the propositional formula φ true. Therefore, the
matching process stops at the lexicographically maximum assignment that makes φ true and
returns the corresponding backtracking choices, from which the values of the capture groups
and hence the corresponding agreeing valuation can be recovered in polynomial time.
We prove in Rocq the following theorem:
▶ Theorem 8. Let φ be a propositional formula in CNF without fictive variables. Let (r, s)
be the regex and string that result from translating φ into an instance of regex matching.
Then:
|r| ≤ 12|φ| and |s| ≤ 2|φ|, where |φ| = 1 + m + ℓ, m is the number of clauses in φ and ℓ
is the number of literal occurrences in φ,
r has neither lookarounds nor lower-bounded quantifiers,
if there is no match of r on s, then the Lexicographic SAT instance φ has no solution,
otherwise, the capture groups of the highest priority match, which can be recovered from
the result of the instance (r, s) of regex parsing, encode the solution to the original
Lexicographic SAT instance.
The formal construction can be found in Appendix D.

<!-- page 11 -->

V. Deng, A. Barrière and C. Pit-Claudel

4.2

JavaScript regex parsing without lookarounds is in OptP

In this section, we show that JavaScript regex parsing without lookarounds is in OptP. We
show this by directly using the definition of OptP. We modify the algorithm for PSPACE
membership shown in Section 3.3 and make it take a list of choices with a fixed polynomial
length (depending on the regex and the input string) which is large enough so that the
algorithm never runs out of choices to make. This algorithm follows the list of choices to
make its branching decisions, returns the list of choices preceded by a 1 if it leads to a match
and returns the list of choices preceded by a 0 if it leads to a mismatch. One bit is consumed
for each recursive call, even if there is no choice to make at that recursive call.
The full algorithm pseudocode can be found in Appendix E.
This algorithm models a nondeterministic Turing machine that guesses the backtracking
choices and returns the list of choices preceded by a 1 if they lead to a match or by a 0 if
they lead to a mismatch. We show that the algorithm takes a polynomial amount of time to
run, and that the lexicographically highest list of choices that is returned by the algorithm
over all possible lists of choices is indeed the desired list of choices.
More formally, we prove in Rocq the following theorem:
▶ Theorem 9. Let r be a regex without lookarounds and s an input string. Let n = nb (r, s) =
(1 + |s|) × |r| + 1. Then there exists a maximal result b of the above algorithm when run with
lists of choices of length n. This result b has length n + 1 and corresponds to the result of
matching r on s.

5

Related work

A body of work is interested in the complexity of modern regex matching. Uezato [31] shows
that regex matching with lookarounds and backreferences is PSPACE-complete for a natural,
non-backtracking semantics. We take inspiration from their work in this paper, extending
it to a real-world backtracking semantics and providing a mechanized proof of the core
arguments. More recently, Nogami and Terauchi [24] show that under the Orthogonal Vectors
Conjecture, regex matching with backreferences, intersection or complement cannot be solved
in time O(|s|2−ε poly(|r|)) for any ε > 0, where |s| is the size of the string and |r| is the size
of the AST of the regular expression. The authors did not consider lookarounds, because
it was shown [3] that regex matching with lookarounds (but without backreferences nor
bounded quantifiers) could be done in O(|r| × |s|). However, our results and that of Uezato
show that surprisingly, while regex matching with backreferences is NP-complete (in its
decision version), regex matching with lookarounds and backreferences is PSPACE-complete.
This contrasts with the lack of increase in computational complexity of the regex matching
problem when only adding lookarounds without backreferences. Finally, the construction
that we formalize for OptP-hardness is directly inspired by [9].
The complexity of other regex-related problems has also been studied. First, the problem
of succinctness of regular expressions is concerned with determining the size of the smallest
regular expression (traditional or not) that can represent some language. For instance,
Ehrenfeucht and Zeiger [13] consider (among others) the language defined by the set of all
paths between two nodes on the arc-labelled complete graph on n nodes, and show that
any traditional regular expresssion representing this language has at least 2n−1 alphabetical
symbols (among other results). More recently, Freydenberger [15] shows that regular expressions with backreferences cannot be minimized effectively with respect to length or number
of variables. Gelade and Neven [19] show that one cannot avoid an exponential or double

11

<!-- page 12 -->

12

On the computational complexity of JavaScript regex matching, in Rocq

exponential increase in size when finding a traditional regular expression corresponding to
the complement of another one, or to the intersection of a fixed or arbitrary number of other
traditional regular expressions. Gelade [18] shows that one cannot avoid a double exponential
increase in size when translating a regular expresssion with interleaving into a standard one.
Second, the problem of regex equivalence is concerned with determining whether two
regexes represent the same language. Stockmeyer and Meyer [28] show that traditional regular
expression equivalence is PSPACE-complete. Together with our results, this means that one
can encode an instance of traditional regex equivalence into an instance of modern regex
matching. Slightly more recently, Stearns and Hunt III [27] show that traditional regular
expression equivalence when restricted to unambiguous regular expressions is decidable in
polynomial time, and generalized this result to regular expressions whose degree of ambiguity
is bounded by a fixed quantity.
In this paper, we relied on the mechanized semantics of JavaScript regexes presented
by Barrière et al. [2]. Other mechanized semantics for modern regexes exist: for instance,
Zhuchko et al. [32] formalized in Lean a model of regular expressions with lookarounds,
intersection and complement, Perry [25] formalized in Lean two disambiguation policies for
regular expressions with capture groups, and Chattopadhyay et al. [5] formalized in Rocq a
model of regular expressions with lookarounds. However, these models are either incomplete
(missing some features) or not explicitly related to a real-world regex language. This is
important to note, as there are semantic differences between different regex languages that
make many regexes non-portable between languages [7]. The semantics that we use in this
work, on the other hand, has been proved equivalent to a direct and complete transcription
of the ECMAScript 2023 semantics of JavaScript regular expressions [8] (see Appendix G).
Finally, complexity results can be established without reasoning with Turing machines,
as we do in this paper. Forster et al. [14] show that the weak call-by-value λ-calculus is a
reasonable model of computation for both time and space, so that complexity results can
be derived by studying the natural time (number of β-reduction steps) and space (size of
largest term in computation) measures of a computation. Gäher and Kunze [16] leverage
this observation to mechanize the Cook-Levin theorem in Rocq.

6

Limitations, future work and discussion

Regex size
The membership results of Sections 3.3 and 4.2 rely on a definition of the size of a regex that
expands away lower-bounded quantifiers. This choice matches usual practice in linear-time
regex matchers, which expand both lower and upper-bounded quantifiers away and measure
complexity relative to this expanded size (of course, this expansion can lead to an exponential
growth of the regex compared to its textual representation, and indeed backtracking matchers
recurse once per regex iteration, leading to worst-case exponential stack usage). In our
case, because optional iterations must consume a character, we only need to expand lowerbounded quantifiers, not upper-bounded ones. Bounded repetitions are an active area of
research [30, 23, 21]. We leave the complexity of matching unexpanded counted repetitions
to future work.

PSPACE-hardness without negation
The result of Section 3.2, the PSPACE-hardness of JavaScript regex matching without
negative lookarounds, still relies on the ability to encode some form of negation of regex

<!-- page 13 -->

V. Deng, A. Barrière and C. Pit-Claudel

matching, thanks to the atomicity of lookarounds. To our knowledge, it is still unknown
whether regex matching with non-atomic positive lookarounds and backreferences is still
PSPACE-hard. Studying the computational complexity of such regexes is left to future work.

Other regex languages
Our complexity results apply to JavaScript regexes as specified in the 2023 edition of the
ECMAScript specification [10], because the preexisting semantics we build upon [2] formalizes
that version. Therefore, two questions naturally arise: whether our results still apply to
current JavaScript regular expressions (e.g. as specified in the 2025 edition of the ECMAScript
specification [12]) and whether our results extend to other regex languages such as Perl, Java,
PCRE2, etc. For the first question, we believe that our results still apply to 2025 JavaScript
regexes, as the new features neither change the semantics of the ones we use, nor seemingly
increase matching complexity. To be confident in this result, one would need to update
the backtracking tree semantics, as well as the Warblre semantics [8] (see Appendix G) to
ECMAScript 2025, and finally adapt our proofs to the new semantics. This is left to future
work.
For the second question, there is a wide range of regex languages, each supporting
different features with different semantics. The hardness results presented in this work can
be expected to apply to any real-world regex language that includes both backreferences
and lookarounds. In regex languages like the ones of PCRE2, Python, Java and .NET,
backreferences fail when referencing a group that has not been defined (instead of matching
the empty string, like we rely on in our reductions) [4]. For these, we suggest changing the
assignment fragments in our reductions, from ⟨(i a)|a⟩; to ⟨(i a|)a?⟩;. As far as we know, all
real-world languages that include lookarounds have atomic lookarounds, that can encode
negation like in Section 3.2. PCRE2 contains both atomic and non-atomic lookarounds,
with dedicated syntax. The membership problems would however require more care, as
this work does not consider features such as balancing groups, control verbs, or recursion,
that can be found in some languages. Also, some languages like POSIX regexes follow
longest-match semantics instead of backtracking semantics, changing the optimization version
of the matching problem. Unfortunately, these other real-world languages do not benefit
from an existing complete mechanization in a proof assistant, and porting our work to other
languages is left as future work.

Relation between exec and parsing
In Section 4, we considered the problem of regex parsing, where, given a regex and a string,
the output describes how the regex matches the string. This contrasts with the problem
solved by the commonly-used JavaScript regex function exec, that only returns the capture
ranges last matched by each capture group in the regex; we call this problem JSRegExec. It
turns out that this problem is not clearly in OptP. Indeed, there is no ordering on the capture
groups alone that can distinguish higher-priority from lower-priority matches. Consider the
following example:
r1 = ^.∗(1 .).∗$ r2 = ^.∗?(1 .).∗$ s = "ab"
Both regexes r1 and r2 have two matches on the string s, where the capture group is mapped
to either "a" or "b", but "b" has higher priority than "a" for regex r1 while "a" has higher
priority than "b" for regex r2 .
However, JSRegExec is FPNP -complete. Indeed, JavaScript regex parsing is in FPNP :
JavaScript regex parsing is in OptP, and as Krentel [22] points out, all problems in OptP are

13

<!-- page 14 -->

14

On the computational complexity of JavaScript regex matching, in Rocq

in FPNP . And since one can recover group boundaries from the result of JavaScript regex
parsing in linear time, JSRegExec is also in FPNP [22, Thm. 3.2(i)]. JSRegExec is also
FPNP -hard: the metric reduction from Lexicographic SAT to JavaScript regex parsing
shown in Section 4.1 is essentially a reduction to JSRegExec, and Lexicographic SAT
is FPNP -hard.

Mechanization
Our Rocq development covers the following: For the hardness proofs, we mechanize the
correctness of the two PSPACE-hardness reductions and of the OptP-hardness reduction; for
membership, we mechanize the monotonicity and the bound on the termination measure,
bounds on the size of each state, and the correctness of the simulation algorithm. We also
prove that these results extend to Warblre’s regexes and semantics by composing Linden
proofs with the pre-existing Warblre-Linden equivalence. In contrast, we do not formalize
the following: the definitions of PSPACE and OptP in terms of Turing machines (we do not
translate our algorithms to Turing machine programs), the space cost of algorithms in terms
of concrete memory (we bound the size of each action and the number of actions in Rocq,
but we do not have a machine model to relate them to), and the complexity of reductions
(we prove reductions correct and bound their output sizes in Rocq, but we rely on manual
observation to guarantee their complexity).
Our code can be found at https://github.com/LindenRegex/js-regex-matching-complexity.

7

Conclusion

Modern regex matching is very powerful: we define the problems of matching and parsing regular expressions with expanded lower-bounded quantifiers, and show that they are
PSPACE- and Opt-complete, respectively. Our main results are mechanized in Rocq against
a faithful model of ECMAScript’s regular expression matching specification, marking the
first time mechanized complexity results have been given for a real-world regex language.

A

Formal constructions for PSPACE-hardness

A.1

With negative lookarounds

Uezato [31] proves that regex matching with backreferences and lookarounds is PSPACE-hard
by reducing QBF∃ to it. However, the reduction Uezato describes makes use of duplicate
capture groups, and the semantics of regular expressions in [31] is not the same as in
JavaScript; for instance, the semantics in [31] is not a backtracking one, and it allows all
iterations of quantifiers to match the empty string. Therefore, Uezato’s result is not directly
applicable to JavaScript. In this section, we adapt this reduction to JavaScript regular
expressions and prove its correctness in Rocq. The reduction we present here is also partly
inspired from [9], where a reduction from 3-SAT to Perl regex matching is presented.
We polynomially reduce QBF∃ to JSRegMatch; this reduction transforms in polynomial
time each QBF∃ input formula to a regex and a string, such that the regex finds a match
on the string if and only if the original formula is true. As in [31], the idea is to let the
backtracking regex matching algorithm simulate the naïve algorithm for checking the validity
of a QBF∃ formula. In this process, the group map, which remembers the values of each
capture group, simulates the environment that maps each variable to ⊤ or ⊥.

<!-- page 15 -->

V. Deng, A. Barrière and C. Pit-Claudel

We start with an example. Let qa = ¬∃x1 .∃x2 .¬∃x3 .(x1 ∨ x2 ) ∧ (¬x2 ∨ x3 ) be a QBF
in the input format of QBF∃ . We want to set up a regex and string of size polynomial in
|qa | such that qa is true if and only if the regex matches the string from the beginning. Our
reduction produces the following regex ra and the following string sa :
ra = (?! E(x1 )E(x2 )(?! E(x3 )⟨\1|\2⟩;⟨\2a|\3⟩;))
sa = "a;a;a;a;a;z"
where E(xi ) = ⟨(i a)|a⟩;
Here is a breakdown of the regex. First, the core, ⟨\1|\2⟩;⟨\2a|\3⟩;. We assume first that the
group map maps each group from 1 to 3 to either "a" or Undef . Under this assumption,
the sequence ⟨\1|\2⟩; ⟨\2a|\3⟩; matches the string "a;a;" if and only if both subregexes
match the string "a;". The components of the sequence match if and only if the groups are
correctly set up. For example, the fragment ⟨\2a|\3⟩; matches the string "a;" if and only
if group 2 is set to Undef (false) or group 3 is set to "a" (true), hence if and only if the
formula ¬x2 ∨ x3 is true given values for each group. Thus, the whole core fragment tests
one assignment: it matches the string "a;a;" if and only if the underlying propositional
formula of the QBF qa is true given values for each variable.
Then, the assignment fragment, i.e. the part with the E(xi ) and the lookarounds,
forces the backtracking engine to consider all possible assignments. From now on, we say
that a group map gm agrees with an assignment e of variables x1 , x2 , ..., xn to ⊥ or ⊤
(an environment) when for each xi , gm maps group i to "a" if and only if e maps xi to
⊤, and gm has no mapping for group i if and only if e maps xi to ⊥. We say that a
group map gm agrees with an environment e on variables in X if and only if the above
condition holds for each xi ∈ X. E(x3 ) = ⟨(3 a)|a⟩; always matches the string "a;", but
has two ways of doing so, either by capturing the a, hence setting group 3 to "a", or by
not capturing the a, leaving group 3 undefined. Therefore, the backtracking engine tries
matching E(x3 ) followed by the inner fragment on the string "a;a;a;" by first setting
group 3 to "a" (true), then trying to match the inner fragment on the string "a;a;". If
this does not succeed, it backtracks, leaving group 3 set to the Undef (false), and tries
matching the inner fragment again on the string "a;a;". Therefore, given a group map
and an environment that agree on variables x1 and x2 and do not define variable x3 ,
the regex E(x3 )⟨\1|\2⟩; ⟨\2a|\3⟩; matches the string "a;a;a;" if and only if the partially
quantified boolean formula ∃x3 .(x1 ∨ x2 ) ∧ (¬x2 ∨ x3 ) is true. Going further outside, the
negative lookahead (?! E(x3 )⟨\1|\2⟩; ⟨\2a|\3⟩; ) matches the string "a;a;a;" if and only if
the lookahead regex E(x3 )⟨\1|\2⟩; ⟨\2a|\3⟩; does not match "a;a;a;", i.e. if and only if the
partially quantified boolean formula ¬∃x3 .(x1 ∨x2 )∧(¬x2 ∨x3 ) is true (note the negation before
the existential quantifier). Then similarly, given a group map and environment that agree on
variable x1 and do not define x2 and x3 , the regex E(x2 )(?! E(x3 )⟨\1|\2⟩; ⟨\2a|\3⟩; ) matches
the string "a;a;a;a;" if and only if the partially quantified boolean formula ∃x2 .¬∃x3 .(x1 ∨
x2 ) ∧ (¬x2 ∨ x3 ) is true. Then, the regex E(x1 )E(x2 )(?! E(x3 )⟨\1|\2⟩; ⟨\2a|\3⟩; ) matches the
string "a;a;a;a;a;" with an empty group map if and only if the quantified boolean formula
∃x1 .∃x2 .¬∃x3 .(x1 ∨ x2 ) ∧ (¬x2 ∨ x3 ) is true. Finally, with the outer negative lookahead, the
regex ra matches the string "a;a;a;a;a;" with an empty group map if and only if the QBF
qa is true. Since the final z is not used anywhere during the matching process, and a regex
does not need to match the entire string to match the said string, it turns out that the regex
ra matches the string sa if and only if the QBF qa is true.
More generally, let q = Q1 x1 .Q2 x2 ....Qn xn .φ(x1 , x2 , ..., xn ) be a QBF that respects
the input format of QBF∃ , where for all i, Qi ∈ {∃, ¬∃} and φ is a possibly negated

15

<!-- page 16 -->

16

On the computational complexity of JavaScript regex matching, in Rocq

propositional formula in CNF. Let φ(x1 , x2 , ..., xn ) = C1 ∧ C2 ∧ ... ∧ Cm or φ(x1 , x2 , ..., xn ) =
¬(C1 ∧ C2 ∧ ... ∧ Cm ), where each of the Cj is a clause (disjunction of literals). Let for all j,
Cj = lj,1 ∨ lj,2 ∨ ... ∨ lj,vj where each lj,k is a literal xi or ¬xi . We define:
Rlit (xi ) = \i

for all i ∈ {1, ..., n}

Rlit (¬xi ) = \ia

for all i ∈ {1, ..., n}

This way, given a group map and environment that agree on variable xi , Rlit (xi ) matches
the string "a" with the group map if and only if the group map maps i to the string "a", i.e.
if and only if variable xi is true in the environment. Conversely, Rlit (¬xi ) matches the string
"a" if and only if group i is set to Undef , i.e. if and only if the literal ¬xi is true under the
environment.
We then define:
Rcl (Cj ) = ⟨Rlit (lj,1 )|Rlit (lj,2 )|...|Rlit (lj,vj )⟩;

for all j ∈ {1, ..., m}

This way, given a group map and an environment that agree, Rcl (Cj ) matches the string
"a;" with the group map if and only if either of the Rlit (lj,k ) matches the string "a" with
that group map, i.e. if and only if either of the literals lj,k is true, i.e. if and only if clause
Cj is true under the environment.
We then define:
Rconj (φ) = Rcl (C1 ) Rcl (C2 ) ... Rcl (Cm )
This way, given a group map and environment that agree, Rconj (φ) matches the string
"a;a;...;a;" ("a;" m times) with the group map if and only if each of the Rcl (Cj ) matches
the string "a;" with the group map, i.e. if and only if all of the clauses Cj are true, i.e. if
and only if the conjunction C1 ∧ C2 ∧ ... ∧ Cj is true under the environment.
We then define:
(
Rconj (φ) if φ(x1 , ..., xn ) = C1 ∧ C2 ∧ ... ∧ Cm
Rn+1 = Rprop (φ) =
(?! Rconj ) if φ(x1 , ..., xn ) = ¬(C1 ∧ C2 ∧ ... ∧ Cm )
This way, given a group map and environment that agree:
if φ is not negated, then Rprop (φ) matches the string "a;a;...;a;" ("a;" m times) with
the group map if and only if Rconj (φ) matches the same string with the group map, i.e.
if and only if φ = C1 ∧ C2 ∧ ... ∧ Cm is true under the environment, according to the
previous explanation,
if φ is negated, then Rprop (φ) matches the string "a;a;...;a;" ("a;" m times) with
the group map if and only if Rconj (φ) does not match the same string, i.e. if and only if
C1 ∧ C2 ∧ ... ∧ Cm is false under the environment (according to the previous explanation),
i.e. if and only if φ = ¬(C1 ∧ C2 ∧ ... ∧ Cm ) is true under the environment.
Then, we define:
E(xi ) = ⟨(i a)|a⟩; for all i ∈ {1, ..., n}
(
E(xi )Ri+1
if Qi = ∃
Ri =
(?! E(xi )Ri+1 ) if Qi = ¬∃
Let qi = Qi xi .Qi+1 xi+1 ....Qn xn .φ(x1 , ..., xn ) and consider a group map and environment
that agree on variables x1 to xi−1 and leave variables xi to xn undefined. Then, Ri matches
the string "a;a;...;a;" ("a;" m + (n + 1 − i) times) with the group map if and only

<!-- page 17 -->

V. Deng, A. Barrière and C. Pit-Claudel

if the partially quantified boolean formula qi is true under the environment. To see this,
proceed by descending induction on i ∈ {1, ..., n + 1}. The case i = n + 1 corresponds to the
characterization of Rprop (φ) given at the previous step. Then, given i ∈ {1, ..., n}, assuming
the property to be true for i + 1, consider a group map and environment that agree on
variables x1 to xi−1 and leave variables xi to xn undefined:
if Qi = ∃, then Ri = E(xi )Ri+1 = ⟨(i a)|a⟩;Ri+1 matches the string "a;a;...;a;" ("a;"
m + (n + 1 − i) times) under the group map if and only if Ri+1 matches the string
"a;...;a;" ("a;" m + (n + 1 − (i + 1)) times) under the group map with group i set to
either "a" or Undef . This is equivalent by the induction hypothesis to qi+1 being true
under the environment with either xi+1 = ⊤ or xi+1 = ⊥, i.e. to qi = ∃xi .qi+1 being true
under the environment;
if Qi = ¬∃, then Ri = (?! E(xi )Ri+1 ) matches the string "a;a;...;a;" ("a;" m + (n +
1 − i) times) with the group map if and only if E(xi )Ri+1 does not match the string
"a;a;...;a;" ("a;" m + (n + 1 − i) times) with the group map. By the previous point,
this is equivalent to ∃xi .qi+1 being false, i.e. to qi = ¬∃xi .qi+1 being true under the
environment.
Finally, we define:
r = R1
s = "a;a;...;a;z"

("a;" n + m times, followed by "z")

We reduce the problem of checking whether q is true to the problem of checking whether r
matches s from the beginning of the string.
The reduction is done in polynomial time. Indeed, an amount of work proportional to
the size of the underlying variable is done per quantifier to map each Qi xi to either E(xi )
or (?! E(xi )...). Then for each clause, the reduction performs a constant amount of work
(appending a ‘;’ to the clause-checking regex) plus an amount of work proportional to the size
of the variable for each variable (adding a disjunction plus either \i or \ia for variable xi ).
The proof that the above reduction from QBF∃ to JSRegMatch is correct, meaning
that a QBF is true if and only if the corresponding regex matches the corresponding string,
was mechanized in the Rocq proof assistant. It relies on six theorems, each specifying a part
of the reduction: one about the Rlit (lj,k ), one about the Rcl (Cj ), one about Rconj (φ), one
about Rprop (φ), one about E(xi ), and one about the Ri . The final theorem about r is as
follows:
▶ Theorem 10. The regex r corresponding to a QBF q matches the string s corresponding
to the QBF q if and only if q is true.

A.2
A.2.1

Without negative lookarounds
Encoding negation without negative lookarounds

Previous work [31] and our reduction in the previous subsection relies on negative lookarounds
to negate regex matching and thus encode negation. Indeed, it seems at first that negative
lookarounds are the only feature of JavaScript regexes that allow such a negation of regex
matching. However, we find that the use of negative lookarounds is in fact not essential to
encode negated existential quantifiers, and that positive lookarounds are enough to encode
a limited form of negation; this reduces the subset of JavaScript regex features that make
JavaScript regex matching PSPACE-hard.

17

<!-- page 18 -->

18

On the computational complexity of JavaScript regex matching, in Rocq

Indeed, in JavaScript and other languages such as Perl [29] and PCRE2 [20], (positive)
lookarounds are atomic, which means that once a match for the lookaround sub-regex is
found, if no match is found for the rest of the regex, no further backtracking is performed
inside the lookaround. Therefore, if we know that a branch of the lookaround matched, then
we know that there was no higher priority branch of the lookaround that could have matched.
In concrete terms, to check whether a regex r does not match a string s = "a;...;a;z",
we check whether the regex
(?= r|Rcap (gz ))Rchk (gz )
matches the string s, where for all g, Rcap (g) = ⟨a;⟩∗(g z) and Rchk (g) = ⟨a;⟩∗\g$ and gz is
a fresh group name. If r indeed does not match s, then the right branch of the disjunction
Rcap (gz ) = ⟨a;⟩∗(gz z) matches "a;...;a;z", capturing the final z in group gz , the lookaround
succeeds and the remaining regex Rchk (gz ) = ⟨a;⟩∗\gz $ matches the string s = "a;...;a;z".
Conversely, if r does match s (it does not capture group gz , since gz is fresh), then the
lookahead succeeds without capturing group gz , and the remaining regex Rchk (gz ) does not
match the string s. Crucially, since lookarounds are atomic in JavaScript, the matching
process does not attempt to backtrack in the lookaround to try to match the right branch of
the disjunction, which would succeed while making group gz capture the final z and make
the entire regex match the string s. Therefore, in that case, the entire regex does not match
s. All in all, we have that the regex (?= r|Rcap (gz ))Rchk (gz ) matches the string s if and only
if the regex r does not match the string s.

A.2.2

Reducing QBF∃ to JavaScript regex matching without negative
lookarounds

To see how we leverage this principle to reduce QBF∃ to JavaScript regex matching without
negative lookarounds, we start with an example. Consider the QBF qb = ∃x1 .¬∃x2 .x1 ∨ x2 .
We want to build a regex without negative lookarounds and a string such that the regex
matches the string from the beginning of the string if and only if qb is true. Our new
reduction produces the following regex rb and string sb :
rb = E(x1 )(?= ⟨E(x2 )⟨\1|\2⟩;⟩|Rcap (z2 ))Rchk (z2 )
sb = "a;a;a;z"
where E(xi ) is defined as in the previous subsection and Rcap (g) and Rchk (g) are defined as
in Appendix A.2.1 for all g.
Here is a breakdown of the regex. As in Appendix A.1, the core of the regex, ⟨\1|\2⟩;,
matches the string "a;z" given values of groups 1 and 2 if and only if the propositional
formula x1 ∨ x2 underlying the QBF qb is true given the corresponding values of the
variables x1 and x2 . Going further outside, as in Appendix A.1, given a group map and
environment that agree on variable x1 and leave x2 undefined, the regex E(x2 )⟨\1|\2⟩;
matches the string "a;a;z" with the group map if and only if the partially quantified boolean
formula ∃x2 .x1 ∨ x2 is true under the environment. Then, as explained above, the regex
(?= ⟨E(x2 )⟨\1|\2⟩;⟩|Rcap (z2 ))Rchk (z2 ) matches the string "a;a;z" with the group map if
and only if the inner regex E(x2 )⟨\1|\2⟩; does not match the string "a;a;z" with the group
map, hence if and only if the partially quantified boolean formula ∃x2 .x1 ∨ x2 is false,
i.e. the partially quantified boolean formula ¬∃x2 .x1 ∨ x2 is true under the environment.
Finally, as in Appendix A.1, the regex E(x1 )(?= ⟨E(x2 )⟨\1|\2⟩;⟩|Rcap (z2 ))Rchk (z2 ) matches
the string "a;a;a;z" if and only if there exists a value of group 1, either "a" or Undef ,
that makes the regex (?= ⟨E(x2 )⟨\1|\2⟩;⟩|Rcap (z2 ))Rchk (z2 ) match the string "a;a;z", i.e.

<!-- page 19 -->

V. Deng, A. Barrière and C. Pit-Claudel

19

if and only if there exists a value of variable x1 making the partially quantified boolean
formula ¬∃x2 .x1 ∨ x2 true, i.e. if and only if the QBF qb = ∃x1 .¬∃x2 .x1 ∨ x2 is true.
More generally, let q = Q1 x1 .Q2 x2 ....Qn xn .φ(x1 , x2 , ..., xn ) be a QBF that respects
the input format of QBF∃ , as in Appendix A.1, where for all i, Qi ∈ {∃, ¬∃} and φ is a
possibly negated propositional formula in CNF. Let φ(x1 , x2 , ..., xn ) = C1 ∧ C2 ∧ ... ∧ Cm or
φ(x1 , x2 , ..., xn ) = ¬(C1 ∧ C2 ∧ ... ∧ Cm ), where each of the Cj is a clause. We define:
Rconj (φ) = Rcl (C1 ) Rcl (C2 ) ... Rcl (Cm ) as before
(
Rconj (φ)
′
′
Rn+1
= Rprop
(φ) =
(?= Rconj (φ)|Rcap (zn+1 ))Rchk (zn+1 )

if φ(x1 , ..., xn ) = C1 ∧ C2 ∧ ... ∧ Cm
if φ(x1 , ..., xn ) = ¬(C1 ∧ C2 ∧ ... ∧ Cm )

This way, consider a group map and an environment that agree on variables x1 , ..., xn such
that the group map does not define group zn+1 ; as in Appendix A.1, Rconj (φ) matches the
string "a;...;a;z" ("a;" m times followed by a z) with the group map if and only if the
conjunction C1 ∧ C2 ∧ ... ∧ Cm is true under the environment. Then:
′
If φ is not negated, then Rprop
(φ) = Rconj (φ) matches the string "a;...;a;z" with the
group map if and only if φ(x1 , ..., xn ) is true under the environment.
If φ is negated, then as explained in Appendix A.2.1, the regex
′
Rprop
(φ) = (?= Rconj (φ)|Rcap (zn+1 ))Rchk (zn+1 )

matches the string "a;...;a;z" with the group map if and only the inner regex, Rconj (φ),
does not match the string "a;...;a;z" with the group map, hence if and only if
C1 ∧ C2 ∧ ... ∧ Cm is false, i.e. if and only if φ(x1 , ..., xn ) = ¬(C1 ∧ C2 ∧ ... ∧ Cm ) is true
under the environment.
We then define:
Rcap (g) = ⟨a;⟩∗(g z)
Rchk (g) = ⟨a;⟩∗\g$
(
′
E(xi )Ri+1
if Qi = ∃
′
Ri =
′
(?= E(xi )Ri+1
|Rcap (zi ))Rchk (zi ) if Qi = ¬∃
Let qi = Qi xi .Qi+1 xi+1 ....Qn xn .φ(x1 , ..., xn ) for all i ∈ {1, ..., n + 1}. Then, given a group
map and an environment that agree on variables x1 to xi−1 such that the group map does
not define any other groups, Ri′ matches the string "a;a;...;a;z" ("a;" m + (n + 1 − i)
times followed by a z) with the group map if and only if qi is true under the environment.
To see this, we proceed by descending induction on i ∈ {1, ..., n + 1}. The case i = n + 1
′
corresponds to the above characterization of Rprop
(φ). Then, let i ∈ {1, ..., n}, assume the
property to be true for i + 1, and consider a group map and environment that agree on
variables x1 to xi−1 such that the group map does not define any other groups:
′
If Qi = ∃, then similarly to Appendix A.1, Ri′ = E(xi )Ri+1
matches the string
′
"a;a;...;a;z" if and only if Ri+1 matches the string "a;...;a;z" with the group
map with group i set to either "a" or Undef , i.e. if and only if qi+1 is true under the
environment with some value of the variable xi , i.e. if and only if qi = ∃xi .qi+1 is true
under the environment.
′
If Qi = ¬∃, then by the previous point, E(xi )Ri+1
matches the string "a;a;...;a;z"
with the group map if and only if ∃xi .qi+1 is true under the environment. Then, as
explained in Appendix A.2.1, the regex
′
Ri′ = (?= E(xi )Ri+1
|Rcap (zi ))Rchk (zi )

<!-- page 20 -->

20

On the computational complexity of JavaScript regex matching, in Rocq

matches the string "a;a;...;a;z" with the group map if and only if the inner regex
′
E(xi )Ri+1
does not match the string "a;a;...;a;z" with the group map, i.e. if and
only if ∃xi .qi+1 is false, i.e. if and only if qi = ¬∃xi .qi+1 is true under the environment.
We finally define:
r′ = R1′
s = "a;a;...;a;z"

("a;" n + m times, followed by "z")

We reduce the problem of determining whether q is true to the problem of determining
whether r′ matches s.
The correctness of the reduction was proved in Rocq, with significant proof sharing with
the reduction that uses negative lookarounds. Three theorems (plus auxiliary lemmas) were
needed: one specifying the behavior of the Rcap (zi ), one specifying the behavior of the
Rchk (zi ), and one specifying the behavior of the Ri′ . The final theorem follows:
▶ Theorem 11. The regex r′ corresponding to a QBF q matches the string s corresponding
to the QBF q if and only if q is true.

B

Pseudo-code of matching algorithm for PSPACE membership

Below is the description of a recursive function compute_result that computes the result
(not a backtracking tree) of JavaScript regex matching given a list of actions, input, group
map, matching direction, and a fuel argument bounding the recursion depth. Below, we
define a function pspace_algo that uses compute_result to check whether a given regex
matches a given input string.
compute_result act inp gm dir 0 = Out_of_fuel
compute_result [] inp gm dir (S fuel) = Success (inp, gm)
compute_result (Acheck strcheck :: cont) inp gm dir (S fuel) =
if (is_strict_suffix inp strcheck dir) then
compute_result cont inp gm dir fuel
else NoMatch
compute_result (Aclose gid :: cont) inp gm dir (S fuel) =
compute_result cont inp (GroupMap.close (idx inp) gid gm) dir fuel
compute_result (Epsilon :: cont) inp gm dir (S fuel) =
compute_result cont inp gm dir fuel
compute_result (Regex.Character cd :: cont) inp gm dir (S fuel) =
match read_char rer cd inp dir with
| Some (c, nextinp) ⇒
compute_result cont nextinp gm dir fuel
| None ⇒ NoMatch
end
compute_result (Disjunction r1 r2 :: cont) inp gm dir (S fuel) =
match compute_result (r1 :: cont) inp gm dir fuel with
| Out_of_fuel ⇒ Out_of_fuel
| Success lf ⇒ Success lf
| NoMatch ⇒ compute_result (r2 :: cont) inp gm dir fuel
end
compute_result (Sequence r1 r2 :: cont) inp gm dir (S fuel) =
compute_result (seq_list r1 r2 dir ++cont) inp gm dir fuel
compute_result (Quantified greedy (S min) delta r1 :: cont) inp gm dir (S fuel) =
let gidl := def_groups r1 in
compute_result (r1 :: Quantified greedy min delta r1 :: cont) inp

<!-- page 21 -->

V. Deng, A. Barrière and C. Pit-Claudel

(GroupMap.reset gidl gm) dir fuel
compute_result (Quantified greedy 0 0 r1 :: cont) inp gm dir (S fuel) =
compute_result cont inp gm dir fuel
compute_result (Quantified true 0 delta r1 :: cont) inp gm dir (S fuel) =
let gidl := def_groups r1 in
match
compute_result
(r1 :: Acheck inp ::
Quantified true 0 (delta−1) r1 :: cont)
inp (GroupMap.reset gidl gm) dir fuel
with
| Out_of_fuel ⇒ Out_of_fuel
| Success lf ⇒ Success lf
| NoMatch ⇒ compute_result cont inp gm dir fuel
end
compute_result (Quantified false 0 delta r1 :: cont) inp gm dir (S fuel) =
let gidl := def_groups r1 in
match (compute_result cont inp gm dir fuel) with
| Out_of_fuel ⇒ Out_of_fuel
| Success lf ⇒ Success lf
| NoMatch ⇒
compute_result
(r1 :: Acheck inp :: Quantified false 0 (delta−1) r1 :: cont)
inp (GroupMap.reset gidl gm) dir fuel
end
compute_result (Group gid r1 :: cont) inp gm dir (S fuel) =
compute_result (r1 :: Aclose gid :: cont) inp (GroupMap.open (idx inp) gid gm) dir fuel
compute_result (Lookaround lk r1 :: cont) inp gm dir (S fuel) =
match (compute_result [r1] inp gm (lk_dir lk) fuel) with
| Out_of_fuel ⇒ Out_of_fuel
| Success (_,gmlk) ⇒
match (positivity lk) with
| false ⇒ NoMatch
| true ⇒ compute_result cont inp gmlk dir fuel
end
| NoMatch ⇒
match (positivity lk) with
| true ⇒ NoMatch
| false ⇒ compute_result cont inp gm dir fuel
end
end
compute_result (Anchor a :: cont) inp gm dir (S fuel) =
if anchor_satisfied rer a inp then
compute_result cont inp gm dir fuel
else NoMatch
compute_result (Backreference gid :: cont) inp gm dir (S fuel) =
match read_backref rer gm gid inp dir with
| Some (br_str, nextinp) ⇒
compute_result cont nextinp gm dir fuel
| None ⇒ NoMatch
end
pspace_algo (r:regex) (s:string) =
let init_fuel := fuel r input(s) in
match compute_result [r] (input(s)) GroupMap.empty forward init_fuel with

21

<!-- page 22 -->

22

On the computational complexity of JavaScript regex matching, in Rocq

| Success _ ⇒ Some true
| NoMatch ⇒ Some false
| OutOfFuel ⇒ None (* we proved this does not happen *)
end

The fuel function is described in Section C.1.3. The compute_result function is very
similar to the functional semantics of [2], except that instead of computing the backtracking
tree, it explores branches of the backtracking tree in a depth-first search order (starting with
the highest priority branches).

C

Full description of PSPACE-membership metric

The algorithm described in Appendix B is similar to the functional semantics of [2]. Like
for the functional semantics of [2], the termination of compute_result is not clear a priori,
since the list of actions grows and shrinks as the function recurses. Both therefore take a fuel
argument, which decreases by one for each recursive call. This fuel argument acts as a bound
on the number of nested recursions that the function can use at any given time. Barrière et
al. [2] show that there exists an amount of fuel for any list of actions, input, group map and
matching direction (a semantic state) such that the functional semantics called with these
arguments and this fuel terminates without running out of fuel. This fuel would (very likely)
also ensure the termination of our algorithm without running out of fuel. However, this fuel
can be exponential in the size of the AST of the regex in the presence of nested quantifiers,
such as with the sequence of regexes ε∗, ε∗∗, ε∗∗∗, etc.
In Section C.1, we present a more complex, but much more tight fuel function fuel that
ensures the termination of compute_result without running out of fuel. This new fuel is
polynomial in the regex and input sizes. In Section C.2, we additionally prove that all the
semantic states involved in the computation of the matching result have a size bounded by a
polynomial in the regex and input sizes.

C.1

Showing termination of the functional semantics of [2] with a
polynomial fuel

To prove termination of our algorithm with our new polynomial fuel, we prove that at each
step of our algorithm, when there is a recursive call, the fuel associated with the semantic
state of the next call is strictly smaller than the one associated to the original state. Namely,
we prove:
▶ Theorem 12. For all semantic states s1 and s2 , if s2 is one of the next semantic states of
s1 , then fuel(s2 ) < fuel(s1 ).
This is the key result of our membership proof. This theorem is proved in the next section.
From this theorem, we deduce:
▶ Theorem 13. For all l, i, gm, d, f such that f > fuel(l, i, gm, d), we have
compute_result l i gm d f <> OutOfFuel.
Proof. This is a direct consequence of Theorem 12. A proof of this theorem that includes
the arguments of Theorem 12 is mechanized.
◀
Finally, we show that for any regex and string, the fuel of the initial state is polynomial
in the sizes of the regex and string. Using GM∅ to denote the initial group map where every
group is set to Undef, and I(s, i) to denote the input s at position i:

<!-- page 23 -->

V. Deng, A. Barrière and C. Pit-Claudel

23

▶ Theorem 14. For all regex r and string s,
fuel([r], I(s, 0), GM∅ , →) ≤ (1 + |s|) × (|r| + |r|2 ).
Proof. This theorem follows from the definition of fuel in Sections C.1.1 and C.1.3. This
proof is mechanized.
◀
We will now define the fuel function and prove Theorem 12. We first provide a formula for
the fuel function that assumes that there are no lookarounds, then extend it in Section C.1.3.

C.1.1

Fuel definition

Our fuel definition does not need the group map. To define our termination measure, we
first divide the list of actions into chunks, sublists of actions separated by the Acheck actions.
In the semantics in [2], chunks are created when unrolling non-lower-bounded quantifiers,
and erased when successfully passing a progress check.
We define three functions, fuel_first, fuel_middle and fuel_last for different chunks. We
define fuel(l, i) as follows: given l = c1 ++ Acheck i1 :: c2 ++ ... ++ Acheck in−1 :: cn ,
fuel(l, i) = fuel_first(c1 , prog(l, i)) +

n−1
X

(fuel_middle(ck ) + 1) + fuel_last(cn , i, prog(l, i))

k=2

where prog(l, i) =

(
1

if ∀i ′ , Acheck i ′ ∈ l ⇒ i < i ′

0

otherwise

1+
Pn

Pn

Given l = [a1 ; ...; an ],
(
fuel_first(l, p) =

fuel_middle(l) = 1 +

fuel_last(l, i, p) =

k=2 |ak |

k=1 |ak |

n
X

k=2
n
X

if a1 is a non-lower-bounded quantifier and p = 1
otherwise

|ak |
!

|ak |

× (|i| + p)

k=1

and |Aclose g| = 1 for all g.
We say that a check input i is smaller than a check input i ′ , denoted i < i ′ , when i
represents the same string as i ′ but has a strictly further position in the string than i ′ .

C.1.2

A strictly decreasing fuel (Theorem 12)

In this section, we focus on explaining why this function is a strictly decreasing termination
measure (Theorem 12). We focus on high-level arguments. For any details, the proof has
been fully mechanized.

Sums of sizes
At the heart of our fuel formula, we sum the sizes of actions in multiple places. The main
idea is that, for any regex feature except non-lower-bounded quantifiers, the sum of the sizes
of actions in the action list provides a polynomial, strictly decreasing termination measure.
Indeed, while the list of actions both grows (e.g. when unrolling a sequence) and shrinks
(e.g. when consuming a character) as we explore the tree, at most steps, the sum of the sizes

<!-- page 24 -->

24

On the computational complexity of JavaScript regex matching, in Rocq

of all actions in the list strictly decreases. For instance, in the case of the sequence, the list
grows but the sum of sizes decreases by one.
While this simple observation fails in the presence of quantifiers, e.g. when unfolding one
iteration of a star, it still explains why a large part of our definitions consists in summing
sizes of actions. Everything that is not a sum of sizes is to deal with non-lower-bounded
quantifiers.

Multiplication by |i| + 1 for the last chunk
In the fuel_last function, we multiply the sum of the sizes of each action by the length of
the remaining input plus one (we ignore cases where prog(l, i) is zero for now; this will be
explained later).
One key observation is that in JavaScript, each optional iteration has to consume at least
one character in order for the subsequent progress check to pass. A consequence is that
non-lower-bounded quantifiers can only do as many iterations as the remaining length of the
string plus one. Barrière et al. [2] used that argument, to define a fuel function where we
sum the sizes of each action except for quantifiers, for which the size is multiplied by the
remaining length of input. This fuel function is proved to be strictly decreasing, but it is not
polynomial in general. Consider for instance the case of the list of actions [r ∗∗] for a regex
r. For a given input i, [2] gives it fuel ∥r ∗∗∥i = ∥r ∗∥i × |i| = ∥r∥i × |i| × |i|. Each level of
nested star multiplies the entire fuel by the length of the input, which can make the fuel
grow exponentially with the size of input.
In order to be more precise in the case of nested stars, we present a new key observation:
Nested Iterations Observation: each subregex can be matched at most |i| + 1 times,
regardless of how many nested quantifiers it is inside of. For instance, when matching r ∗∗,
it is possible for both the inner and the outer star to be iterated |i| + 1 times. However, it is
not possible to have |i| + 1 iterations of the outer star, each of which doing |i| + 1 iterations
of the inner star. If one of the outer star iterations iterates the inner star |i| + 1 times, then
we consumed at least |i| characters in the string, and the outer star cannot iterate anymore.
That is why in the definition of fuel_last, in order to be polynomial, we multiply the
whole sum of sizes by |i| + 1 at most. If a regex with nested stars ends up in that chunk, the
whole size will be multiplied only once by the input length.
While this is more precise, this also makes the formula more complex: we have to treat
other chunks than the last one in a different way.

Handling other chunks
When we unroll a quantifier, for instance when the list of actions [r ∗∗] becomes [r ∗ ::
Acheck i :: r ∗∗], we end up with multiple chunks, increasing the size of our list. Our formula
cannot measure quantifiers in these new chunks the same way that it measured quantifiers in
the last chunk, otherwise the fuel would increase.
We again leverage the nested iterations observation. In this example, r ∗ can be iterated
n1 times and r ∗∗ can be iterated n2 times, but we should ensure that n1 + n2 ≤ |i| + 1 so
that the common subregex r is iterated at most |i| + 1 times.
We are trying to compute a bound on the maximum number of steps that executing this
list of actions could take. In our example, since r ∗ has a smaller size than r ∗∗, the worst
case happens when iterating ∗ only once (n1 = 1), and iterating r ∗∗ |i| times (n2 = |i|). In
that example for the worst case, we need n1 to be at least 1, otherwise the Acheck action
will not succeed.

<!-- page 25 -->

V. Deng, A. Barrière and C. Pit-Claudel

To generalize this idea, we prove the following invariants:
▶ Invariant 1. In any reachable list of actions, each Acheck action is followed by a nonlower-bounded quantifier.
▶ Invariant 2. Except for the last chunk, the size of any chunk is smaller than the size of
the next quantified regex (just after the check at the end of the chunk).
With these invariants, we know that every chunk has a size smaller than the quantifier that
follows it, and that the last chunk has a bigger size than any other chunk. By multiplying
the last chunk by |i| + 1 and not the others, we ensure:
1. that we enforce the nested iterations observation, not giving too much fuel to nested
stars, and
2. that by multiplying by |i| + 1 the size of the biggest chunk (and not the others), we
have enough fuel to handle the worst-case scenario where the bigger regex (r ∗∗ in our
example) is iterated more times.

Measuring quantifiers outside of the last chunk
Using that same principle (iterating the last chunk is more costly than iterating other part
of the list of actions), when a non-lower-bounded quantifier immediately follows an Acheck
action, the worst-case scenario consists in skipping that quantifier, allowing more iterations
of the last chunk. This is why the first action of each middle chunk is measured as 1.
Similarly, when the first action of the first chunk is a quantifier, we may skip it and
measure it as 1, but only in the case where skipping the quantifier does not prevent us
from passing the subsequent checks (i.e., only when all checks in the list pass). In cases like
[r ∗ :: Acheck i :: r ∗∗] at input i, in the worst case, r ∗ is iterated once in order to pass the
Acheck i action, and then be able to iterate the last chunk as many times as possible.

Using progress
In our formula, we multiply the last chunk by either |i| or |i| + 1 depending on the value
of prog(l, i), which encodes whether or not we have made enough progress in the string to
pass all checks in the list of actions. When at least one check does not pass (prog(l, i) = 0),
it means that we are currently iterating a quantifier, but we can’t iterate it further unless
we read one character during this current iteration and pass the Acheck action successfully.
This means that the quantifier after the check can be iterated at most |i| times, not |i| + 1,
since we only execute them after one character has been read. On the contrary, if all checks
pass (prog(l, i) = 1), then it is possible to pass all Acheck actions without consuming any
more characters, leading to at most |i| + 1 iterations of the last chunk.
To mechanize this argument, we had to prove the following invariant:
▶ Invariant 3. All check inputs in a reachable list of actions are ordered from more restrictive
to less restrictive.

Example
To see how this formula is strictly decreasing even when dealing with quantifiers, consider
matching the regex a∗ on the string "aa".
At some point, we reach the list of actions l1 = [a :: Acheck "aa" :: a∗] with input
i1 = "aa". The fuel corresponding to this list of actions and input is 1 + 1 + (4 × 2) = 10 (since

25

<!-- page 26 -->

26

On the computational complexity of JavaScript regex matching, in Rocq

prog(l1 , i1 ) = 0). As we read the first letter, we move on to the lists l2 = [Acheck "aa" :: a∗],
then l3 = [a∗], at input i2 = i3 = "a". Their fuel is respectively 1 + (4 × (1 + 1)) = 9 and
4 × (1 + 1) = 8 (prog(l2 , i2 ) = prog(l3 , i3 ) = 1).
On the longest branch of the tree, the one which iterates the star again, we move on
to the list of actions l4 = [a :: Acheck "a" :: a∗], with input i4 = "a". This pair has fuel
1 + 1 + (4 × 1) = 6 (because prog(l4 , i4 ) = 0).
We can see that at each step, the fuel function has strictly decreased.

C.1.3

Extension of the fuel function to lookarounds

In order to take lookarounds into account, we add enough fuel to match the lookarounds from
any position in the string, and adapt our fuel definition to support both matching directions.
We define the function reg_lk_fuel as follows:
reg_lk_fuel(s, ε) = reg_lk_fuel(s, cd) = 0
reg_lk_fuel(s, a) = reg_lk_fuel(s, \g) = 0
reg_lk_fuel(s, (g r)) = reg_lk_fuel(s, r{min, ∆, p}) = reg_lk_fuel(s, r)
reg_lk_fuel(s, r1 |r2 ) = reg_lk_fuel(s, r1 r2 ) = max(reg_lk_fuel(s, r1 ), reg_lk_fuel(s, r2 ))
reg_lk_fuel(s, (?lk r)) = (1 + |s|) × |r| + reg_lk_fuel(s, r)
We lift this definition to actions by defining the function act_lk_fuel as
act_lk_fuel(s, l) = max reg_lk_fuel(s, r)
r∈l

We finally (re)define fuel(l, i, d) as follows: given l = c1 ++ Acheck i1 :: c2 ++ ... ++
Acheck in−1 :: cn ,
fuel(l, i, d) = fuel_first(c1 , prog(l, i, d)) +

n−1
X

(fuel_middle(ck ) + 1)

k=2

+ fuel_last(cn , i, d, prog(l, i, d)) + act_lk_fuel(str(i), l)
(
1 if ∀i ′ , Acheck i ′ ∈ l ⇒ i <d i ′
where prog(l, i, d) =
0 otherwise
Given l = [a1 ; ...; an ],
(
fuel_first(l, p) =

1+
Pn

fuel_middle(l) = 1 +

fuel_last(l, i, d, p) =

Pn

k=2 |ak |

k=1 |ak |
n
X

if a1 is a non-lower-bounded quantifier and p = 1
otherwise

|ak |

k=2
n
X

!

|ak |

× (|i|d + p)

k=1

We write |i|d for the remaining length of input i in direction d. We say that a check input i
is smaller than a check input i ′ in direction d, denoted i <d i ′ , where i represents the same
string as i ′ but has a position in the string that is strictly further in direction d than i ′ .
As in [2], we consider the worst case for the strings of each lookaround, which means
taking as remaining length of the input the length of the entire input string.

<!-- page 27 -->

V. Deng, A. Barrière and C. Pit-Claudel

C.2

Bounding the size of the semantic states used in recursive calls

We now polynomially bound the memory footprint of the semantic states used in recursive
calls by a polynomial in the size of the input string s and regex r.
Such a semantic state contains a list of actions, an input, a group map and a matching
direction.
The memory footprint of the input is bounded by |s|. The group map contains at most
as many pairs of indices into the string as there are groups in the regex, and there are at
most |r| such groups. Therefore, the memory footprint of the group map is O(|r| × log |s|).
The matching direction has a constant memory footprint.
We now polynomially bound the sizes of the lists of actions reachable from r and s. The
key idea to bound these sizes is to use invariant 2 about the lists of actions, which states
that each chunk has a size less than that of the quantified regex following the next check
action. This entails that each chunk has a size less than the next one, which allows bounding
the sum of the sizes of the chunks by m(m + 1)/2 where m is the size of the last chunk.
Finally, the size of the last chunk is less than or equal to the size of the original regex. This
is intuitively because the last chunk is the result of unrolling the original regex without
considering duplicates incurred by unrolling a quantified regex, and that cannot increase the
size of the last chunk.
We end up with the following theorem, which was proven in Rocq:
▶ Theorem 15. For any list of actions l reachable from a regex r,
|l| ≤ |r| + |r|(|r| + 1)/2
where |l| is the sum of the sizes of each action of l and |Acheck i| = 1.
However, the size of the list of actions may be an underestimate of its memory footprint,
mainly due to us supporting quantified regexes with an unbounded maximum number of
repetitions. Indeed, the size we define for the regex r{0, max, p} is 3 + |r|, discarding entirely
the memory footprint of max. max may not even appear in the maximum numbers of
repetitions encountered in the original regex. To solve this problem, we notice the following:
the bound above remains a bound on the number of actions in the list of actions, and we
can easily bound the memory footprint of each action by either the memory footprint of
the original regex (for any action other than an Acheck action) or by O(log |s|) (for Acheck
actions, we remember the index into the input string).
Therefore, the memory footprint of any list of actions reachable from a regex r is
O((|r| + log |s|) × [|r| + |r|(|r| + 1)/2]).
This yields a bound on the memory footprint of the semantic states of the functional
semantics which is polynomial in the sizes of the initial regex and input string.

C.3

Putting everything together

Putting everything together, we obtain the following theorem, which is not mechanized:
▶ Theorem 16. For any regex r and string s, executing pspace_algo(r, s) requires polynomial
space (in the size of s and the expanded size of r).
Proof. We know that during the execution, we never use more stack frames than the initial
fuel. From Theorem 14, we know that this number is polynomial in the size of s and the
expanded size of r. From Section C.2, we know that each of these stack frames themselves
require a polynomial amount of space.
◀

27

<!-- page 28 -->

28

On the computational complexity of JavaScript regex matching, in Rocq

Correctness of the algorithm
We prove:
▶ Theorem 17. Let r be a regex and s be a string. Let t be the backtracking tree such that
([r], I(s, 0), GM∅ , →) ⇓ t. Then pspace_algo(r, s) returns the first leaf of t.
Proof. We prove the following generalized result: Let l be a list of actions, i an input, gm a
group map, d a matching direction and f ∈ N. Assume that compute_result l i gm d f
terminates without running out of fuel. Let t be the backtracking tree such that (l, i, gm, d)⇓t.
Then compute_result l i gm d f returns the first leaf of t.
This is proven by induction on f .
◀
This entails:
▶ Theorem 18. For any regex r and string s, pspace_algo(r, s) returns the boolean result of
the instance of JSRegMatch with r and s, according to the ECMAScript 2023 specification.
Proof. Compose Theorem 17 with the proof of faithfulness of the semantics in [2] (Section
4).
◀
We conclude from Theorems 16 and 18:
▶ Theorem 19. JavaScript regex matching with expanded lower-bounded quantifiers is in
PSPACE.

D

Formal construction for OptP-hardness

D.1

Principle of reduction from CNF Lexicographic SAT

In the absence of negated existential quantifiers, the two reductions presented in Appendices A.1 and A.2 are identical. This reduction is essentially also a reduction from CNF
Lexicographic SAT to JavaScript regex parsing without lookarounds. Indeed, consider
for instance the following propositional formula in CNF with three variables x1 , x2 and x3 :
φc = (x1 ∨ x2 ) ∧ (¬x2 ∨ x3 )
The corresponding QBF qc = ∃x1 .∃x2 .∃x3 .φc is encoded by the reductions of Appendix A
into the problem of deciding whether the following regex rc matches the following string sc :
rc = E(x1 )E(x2 )E(x3 )Rprop (φc )
sc = "a;a;a;a;a;z"
where Rprop (φc ) = ⟨\1|\2⟩;⟨\2a|\3⟩;. As explained earlier, each regex E(xi ) either sets
the capture group i to "a" (representing ⊤) or leaves it undefined (representing ⊥), and
after all capture groups are set up, Rprop (φc ) checks whether the resulting environment
satisfies the propositional formula φc . What also makes this reduction a reduction from
CNF Lexicographic SAT to JavaScript regex parsing without lookarounds is the following
observations:
The backtracking semantics of JavaScript regexes gives priority to first setting group 1
to "a" (left branch of the disjunction) before leaving it undefined (right branch of the
disjunction). Within a setup of group 1, the backtracking semantics gives priority to

<!-- page 29 -->

V. Deng, A. Barrière and C. Pit-Claudel

29

first setting group 2 to "a" before leaving it undefined, and similarly for group 3. As a
consequence, the backtracking semantics of JavaScript specifies that environments are
considered with an order of priority corresponding to descending lexicographic order.
When the first match of a regex on a string is found, the parsing process returns the list
of backtracking choices made, from which the values of the capture groups (the group
map) can be deduced. Returning the group map is equivalent in our case to returning
the highest priority environment that makes the propositional formula true.
In the absence of negated existential quantifiers in the QBF, the reduction does not
involve negative or positive lookarounds in the regex.
Therefore, to solve our instance of CNF Lexicographic SAT, one can simply try matching
rc on sc from the beginning of the string and straightforwardly convert the resulting group
map into an environment mapping variables to ⊤ or ⊥ (or return 0 if rc does not match sc ).
More generally, let φ(x1 , x2 , ..., xn ) = C1 ∧ C2 ∧ ... ∧ Cm be a propositional formula
in CNF, where each of the Cj is a clause. We reduce the resulting instance of CNF
Lexicographic SAT to the problem of matching the following regex r on the following
string s:
Rprop (φ) = Rcl (C1 ) Rcl (C2 ) ... Rcl (Cm )

as before

r = E(x1 ) E(x2 ) ... E(xn )Rprop
s = "a;a;...;a;z"

D.2

("a;" n + m times, followed by "z")

Proof of validity of reduction

We now prove that the above reduction from CNF Lexicographic SAT to JavaScript
regex parsing without lookarounds is valid, which means that given an instance of CNF
Lexicographic SAT, one can recover the result of this instance from the result of the
corresponding instance of regex parsing. This proof is mechanized in the Rocq proof assistant.
We set Rconj (φ) = Rn+1 = Rprop (φ) and Ri = E(xi ) E(xi+1 ) ... E(xn )Rprop (φ) for all
i ∈ {1, ..., n + 1}; these notations match the reduction presented in Appendix A.1. We say
that a group map gm is well-formed when for all i ∈ {1, ..., n}, either gm(i) is undefined or
gm(i) = (2(i − 1), 2(i − 1) + 1) (corresponding to the i-th a in the input string). We then
define, for all well-formed group maps gm and gm ′ :
gm ′ >lex gm ⇔ ∃i ∈ {1, ..., n},
gm ′ (i) = (2(i − 1), 2(i − 1) + 1) ∧ gm(i) = ⊥ ∧ ∀j ∈ {1, ..., i − 1}, gm ′ (j) = gm(j)
where gm(i) = ⊥ means that gm(i) is undefined; this is a strict partial order on the set
of well-formed group maps. Finally, we define the following function G that computes the
group map of the first leaf corresponding to a list of actions, input, group map and matching
direction:
▶ Definition 20. Let l be a list of actions, inp be an input, gm be a group map and d be a
matching direction. We define G(l, inp, gm, d) as follows. Let t be the backtracking tree such
that (l, inp, gm, d) ⇓ t. Then:
if t has a leaf, and its first leaf is (inp′ , gm ′ ), we define G(l, inp, gm, d) = gm ′ ,
if t has no leaf, G(l, inp, gm, d) is undefined (denoted G(l, inp, gm, d) = ⊥).
This function is similar to the function L0 in [2] that computes the first leaf of a backtracking
tree for an input.
We now prove the following invariant:

<!-- page 30 -->

30

On the computational complexity of JavaScript regex matching, in Rocq

▶ Theorem 21. Let i ∈ {1, ..., n + 1}. Let inp = I(s, 2(i − 1)) be the input in string s at
position 2(i − 1) (right before the i-th a). Let gm 0 be a well-formed group map such that
gm 0 (i), gm 0 (i + 1), ..., gm 0 (n) are undefined.
1. Assume that G([Ri ], inp, gm 0 , →) = gm for some gm. Then:
a. gm satisfies φ,
b. gm and gm 0 coincide on indices 1, 2, ..., i − 1,
c. for any well-formed gm ′ such that gm ′ >lex gm and gm ′ and gm coincide on indices
1, 2, ..., i − 1, gm ′ does not satisfy φ.
2. Assume that G([Ri ], inp, gm 0 , →) is undefined. Then for any well-formed gm that coincides
with gm 0 on indices 1, 2, ..., i − 1, gm does not satisfy φ.
Proof (sketch). We proceed by descending induction on i ∈ {1, ..., n + 1}.
First, let i = n + 1. Let inp = I(s, 2n) and gm 0 be a well-formed group map.
Assume that G([Rn+1 ], inp, gm 0 , →) = gm for some gm. Then statements 1a and 1b are
consequences of the theorem about Rconj (φ) in Appendix A.1, while statement 1c has an
impossible condition (a group map gm ′ cannot both coincide with gm on all indices and
be lexicographically greater than gm).
Statement 2 follows from the fact that the statement “gm satisfies φ” does not depend on
indices other than 1, 2, ..., n: essentially, the only gm that coincides with gm 0 on indices
1, 2, ..., n is gm 0 itself.
Now, let i ∈ {1, ..., n} and assume that the invariant holds for i+1. Let inpi = I(s, 2(i−1))
and gm 0 be a well-formed group map such that gm 0 (i), gm 0 (i + 1), ..., gm 0 (n) are undefined.
To prove statement 1, assume that G([Ri ], inpi , gm 0 , →) = gm for some gm. Then, letting
inpi+1 = I(s, 2i), one the following is true:
G([Ri+1 ], inpi+1 , gm 0 [i 7→ (2(i − 1), 2(i − 1) + 1)], →) = gm.
G([Ri+1 ], inpi+1 , gm 0 [i 7→ (2(i − 1), 2(i − 1) + 1)], →) = ⊥ and G([Ri+1 ], inpi+1 , gm 0 , →
) = gm.
We then reason by case analysis:
In the first case, the induction hypothesis directly implies statement 1a, and it almost
directly implies statement 1b. For statement 1c, notice that the condition implies
gm ′ >lex gm and that gm ′ and gm coincide on indices 1, 2, ..., i = i + 1 − 1: the induction
hypothesis applies.
In the second case, the induction hypothesis directly implies statement 1a, and it almost
directly implies statement 1b. For statement 1c, the condition states that gm ′ >lex gm
and that gm ′ and gm coincide on indices 1, 2, ..., i − 1. Perform case analysis on whether
gm ′ (i) = gm(i); notice that gm(i) = ⊥ by statement 1b of the induction hypothesis.
If gm ′ (i) = ⊥, then gm ′ and gm coincide on indices 1, 2, ..., i = i + 1 − 1 and statement
1c of the induction hypothesis applies.
If gm ′ (i) = (2(i − 1), 2(i − 1) + 1), then statement 2 of the induction hypothesis applies,
because G([Ri+1 ], inpi+1 , gm 0 [i 7→ (2(i − 1), 2(i − 1) + 1)], →) = ⊥.
To prove statement 2, assume that G([Ri ], inpi , gm 0 , →) = ⊥. Let inpi+1 = I(s, 2i). Then
we have G([Ri+1 ], inpi+1 , gm 0 [i 7→ (2(i−1), 2(i−1)+1)], →) = G([Ri+1 ], inpi+1 , gm 0 , →) = ⊥.
For any well-formed group map gm that coincides with gm 0 on indices 1, 2, ..., i − 1:
if gm(i) = ⊥, then the induction hypothesis applies with group map gm 0 ,
if gm(i) = (2(i − 1), 2(i − 1) + 1), then the induction hypothesis applies with group map
gm 0 [i 7→ (2(i − 1), 2(i − 1) + 1)].
◀

<!-- page 31 -->

V. Deng, A. Barrière and C. Pit-Claudel

This invariant implies the following theorem:
▶ Theorem 22.
Assume that G([r = R1 ], I(s, 0), GM∅ , →) = gm for some gm. Then gm satisfies φ, and
for any well-formed group map gm ′ such that gm ′ >lex gm, gm ′ does not satisfy φ.
Assume that G([r = R1 ], I(s, 0), GM∅ , →) = ⊥. Then for any well-formed group map gm,
gm does not satisfy φ.
This theorem in turn proves the validity of the reduction:
▶ Theorem 23. The result of the instance φ of CNF Lexicographic SAT is equal to the
transformation of the result of matching the regex r′ on the string s into an assignment of
propositional variables to ⊤ or ⊥.
The transformation from a group map gm consists in the assignment that maps each variable
xi to either ⊤ if gm(i) is defined or ⊥ otherwise.

E

Pseudo-code of OptP algorithm

We provide an algorithm optp_output that, given a regex without lookarounds and a string,
as well as a list of choices (left or right), tries matching the regex on the string by following
the choices in the list of choices instead of performing the full backtracking. It then returns
the list of choices preceded with a 1 (true) if matching succeeds or a 0 (false) if matching
fails. This algorithm is similar to the PSPACE algorithm presented in Appendix B and to
the functional semantics of [2], and is presented below.
Record config := Cfg { cfg_act: actions; cfg_inp: input; cfg_gm: group_map }.
Inductive step := SLeaf (lf: leaf) | SFail | SGo (c: config) | SBranch (hi lo: config).
optp_step dir (Cfg [] inp gm) = SLeaf (inp, gm)
optp_step dir (Cfg (Acheck strcheck :: cont) inp gm) =
if is_strict_suffix inp strcheck dir then SGo (Cfg cont inp gm) else SFail
optp_step dir (Cfg (Aclose gid :: cont) inp gm) =
SGo (Cfg cont inp (GroupMap.close (idx inp) gid gm))
optp_step dir (Cfg (Epsilon :: cont) inp gm) = SGo (Cfg cont inp gm)
optp_step dir (Cfg (Regex.Character cd :: cont) inp gm) =
match read_char rer cd inp dir with
| Some (_, nextinp) ⇒ SGo (Cfg cont nextinp gm)
| None ⇒ SFail
end
optp_step dir (Cfg (Disjunction r1 r2 :: cont) inp gm) =
SBranch (Cfg (r1 :: cont) inp gm) (Cfg (r2 :: cont) inp gm)
optp_step dir (Cfg (Sequence r1 r2 :: cont) inp gm) =
SGo (Cfg (seq_list r1 r2 dir ++cont) inp gm)
optp_step dir (Cfg (Quantified greedy (S min) delta r1 :: cont) inp gm) =
SGo (Cfg (r1 :: Quantified greedy min delta r1 :: cont) inp
(GroupMap.reset (def_groups r1) gm))
optp_step dir (Cfg (Quantified greedy 0 0 r1 :: cont) inp gm) =
SGo (Cfg cont inp gm)
optp_step dir (Cfg (Quantified greedy 0 (delta + 1) r1 :: cont) inp gm) =
let iter := Cfg (r1 :: Acheck inp :: Quantified greedy 0 delta r1 :: cont) in
let skip := Cfg cont inp gm in
if greedy then SBranch iter skip else SBranch skip iter

31

<!-- page 32 -->

32

On the computational complexity of JavaScript regex matching, in Rocq

optp_step dir (Cfg (Group gid r1 :: cont) inp gm) =
SGo (Cfg (r1 :: Aclose gid :: cont) inp (GroupMap.open (idx inp) gid gm))
optp_step dir (Cfg (Anchor a :: cont) inp gm) =
if anchor_satisfied rer a inp then SGo (Cfg cont inp gm) else SFail
optp_step dir (Cfg (Backreference gid :: cont) inp gm) =
match read_backref rer gm gid inp dir with
| Some (_, nextinp) ⇒ SGo (Cfg cont nextinp gm)
| None ⇒ SFail
end
optp_algo dir (c: config) ([]: list bool) = Out_of_fuel
optp_algo dir (c: config) ((b :: cs): list bool) =
match optp_step dir c with
| SLeaf lf ⇒ Success lf
| SFail ⇒ NoMatch
| SGo c’ ⇒ optp_algo dir c’ cs
| SBranch hi lo ⇒ optp_algo dir (if b then hi else lo) cs
end
optp_run r inp (cs: list bool) = optp_algo forward (Cfg [r] inp GroupMap.empty) cs
matched (Success _) = true
matched _ = false
optp_output r inp (cs: list bool) = matched (optp_run r inp cs) :: cs

The config record above stores a semantic state, which is the combination of a list of
actions, an input and a group map. In the absence of lookarounds, the direction remains
constant throughout matching, and hence does not need to be included in the semantic state.
The optp_step function above computes the next step(s) corresponding to a semantic state.
There are four types of next steps, modeled by the step inductive: reaching a leaf, failing
to find a match, going to a next configuration without branching, or branching to one of
two next configurations. Given a configuration c and a list of choices cs, the optp_algo
function above steps from the configuration c by following the list of choices cs, then returns
the corresponding matching result. Given a regex r, an input inp and a list of choices
cs, the optp_run function above computes the result of matching r on inp while following
the choices in cs. The matched function above computes the bit that allows making all
unsuccessful lists of choices precede all successful lists of choices in the ordering of the parsing
results.
The optp_output function above is meant to be called with a list of choices whose length
is equal to nb (r, inp) = (1 + |inp|)|r| + 1, where |inp| is the number of remaining characters
in the input inp. This ensures that optp_algo does not run out of choices to make, and
that all considered lists of choices have equal length.
When there is no choice to be made, the choice at the head of the list of choices is ignored
and popped. When there is a choice to be made, namely when matching a disjunction or a
non-lower-bounded quantifier, instead of trying both choices, the algorithm uses the choice
at the head of the list of choices to decide which choice to make.

<!-- page 33 -->

V. Deng, A. Barrière and C. Pit-Claudel

F

Proof of OptP membership of JavaScript regex parsing without
lookarounds

We now show why this algorithm shows that JavaScript regex parsing without lookarounds
is in OptP. We need to prove two things: that this algorithm runs in polynomial time given
a list of choices, and that the minimum result of all the branches corresponds to the result of
JavaScript regex parsing.
▶ Theorem 24. Given a regex r without lookarounds, a string s and a list of choices of
length nb (r, s), optp_algo runs in polynomial time in the size of s and the expanded size of
r.
Proof. Given a list of choices of length nb (r, s), this algorithm never branches, and it
performs a polynomial amount of work per recursive call. Moreover, there are no more than
fuel r I(s, 0) recursive calls made, and fuel r I(s, 0) is polynomial in r and s by Theorem
14. Therefore, the algorithm runs in polynomial time given a list of choices.
◀
▶ Theorem 25. Given a regex r without lookarounds and a string s, the maximum result
of all calls to optp_algo with a list of choices of length nb (r, s) corresponds to the result of
JavaScript regex matching as defined in the ECMAScript 2023 specification.
Proof. The proof of this theorem was mechanized in the Rocq proof assistant.
The correctness of this algorithm relies on the following two points:
No matter the list of choices given to optp_algo, as long as its length is equal to nb (r, s),
the result is never OutOfFuel. This means that we explore all the branches when ranging
over all possible lists of choices.
We order the results by lexicographic order. This lexicographic order corresponds to the
lexicographic order on the pair of a bit that indicates whether the match succeeded and
the list of choices. As such:
if there is no match of r on s, no result will start with a 1, hence the lexicographically
lowest result of the algorithm is 0 followed by as many ones as needed, which is what
is required by the parsing problem,
if there is a match of r on s, the lexicographically highest result of the algorithm is 1
followed by the lexicographically highest list of choices that leads to a match of r on s,
which is what is required by the parsing problem.
◀
Combining Theorems 24 and 25, we obtain:
▶ Theorem 26. JavaScript regex parsing without lookarounds is in OptP.

G

Connecting our proofs to the specification text of ECMAScript

There exists two mechanized semantics of JavaScript regular expressions in Rocq: the Warblre
mechanization of [8], and the backtracking tree semantics of [2]. The first one is focused on
faithfulness and auditability: it is the result of manually translating line-by-line each line
of pseudocode from the ECMAScript specification into its Rocq equivalent. On the other
hand, the backtracking tree semantics is designed for succinctness and practicality: it is
much easier to conduct proofs using the inductive principle that arises from backtracking
tree semantics, than with the matcher functions of Warblre.

33

<!-- page 34 -->

34

On the computational complexity of JavaScript regex matching, in Rocq

Previously, the two semantics had been proved to be equivalent [2], meaning that the
leftmost branch of a backtracking tree is always equal to the result specified by Warblre. There
is however one difference: the backtracking tree semantics uses a datatype to represent regexes
that is slightly more expressive than the one used by both Warblre and the ECMAScript
specification. For instance, it is possible to define backtracking tree semantics for regexes
that contain badly indexed capture groups, unconstrained duplicated capture groups, or
double negations in character classes. Consequently, for each of our hardness results, we
must also prove that the regexes we generate fall within the subset of regexes that can also
be expressed as well-formed ECMAScript regexes.
We have formally verified that property for each of our hardness reductions, by redefining
the functions so that they target Warblre regexes, proving that these regexes are well-formed
(according to the EarlyErrors check of ECMAScript), and proving that they are the direct
equivalent of the regexes for which we proved the reduction correct. We compose all of these
results, and obtain theorems that are stated directly using Warblre regexes and Warblre
semantics.

H

On unanchored vs. anchored matching

All of our results are proved for anchored matching, where given a regex r and a string s,
we want to find a match of r in s that starts at the beginning of s. This is the behavior of
JavaScript regex matching with the sticky (y) flag, and contrasts with the default behavior
of JavaScript regex matching, where we want to find a match of r that starts anywhere in
s, with priority given to earlier starting positions in s. We made the choice of dealing with
anchored matching, because the core of both the Linden [2] and Warblre [8] specifications
deal with anchored matching.
To extend our results to unanchored matching, we observe the following. For hardness
results, we notice that our reductions from PCNF-QBF and Lexicographic SAT can be
modified straightforwardly to target unanchored matching: indeed, in the absence of newline
characters in the string (which is the case in our reductions), finding an anchored match
of a regex r on a string s is equivalent to finding an unanchored match of ^r on s, where
^ is the beginning-of-string (or beginning-of-line, if the multiline flag m is on) anchor. For
membership results, we notice that unanchored matching reduces to anchored matching:
finding an unanchored match of a regex r on a string s reduces to finding an anchored match
of the regex [ˆ]∗?(0 r), where [ˆ] is a regex that matches all characters, including newline
characters. Indeed, the lazy star gives priority to exiting the quantifier instead of iterating it
again, so backtracking semantics will first try matching r from position 0 in the string, then
from position 1, then 2, etc. The capture group 0 allows to retrieve the range effectively
matched by r.

I

AI Disclosure

In deciding whether and where to apply Generative AI tools, we distinguished core theorems,
where the key arguments are novel observations (the PSPACE hardness and membership
proofs against the inductive backtracking tree semantics [2]) from sanity-checking theorems,
where the proof is routine and the value of mechanization is only to provide stronger
confidence (connecting QBF∃ to QBF, reusing the PSPACE-hardness construction and
PSPACE-membership proof arguments for OptP, and extending arguments to cover the
Warblre interpreted semantics as explained in Appendix G). For core theorems, we completed

<!-- page 35 -->

V. Deng, A. Barrière and C. Pit-Claudel

reductions and proofs on paper first, then mechanized them in Rocq by hand, with no AI
support. For sanity-checking theorems, we used Generative AI to generate Rocq proofs
matching paper proof sketches, then manually reviewed the proofs and documented top-level
statements. Generative AI was not used in the authoring of this paper. To facilitate auditing,
our code supplement includes a patch file ai.patch summarizing all modifications that were
made with the assistance of generative AI tools.
References
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
11
12
13

Alfred V. Aho. Algorithms for finding patterns in strings, page 255–300. MIT Press, Cambridge,
MA, USA, 1991.
Aurèle Barrière, Victor Deng, and Clément Pit-Claudel. Formal verification for javascript
regular expressions: A proven mechanized semantics and its applications. Proc. ACM Program.
Lang., 10(POPL), January 2026. doi:10.1145/3776710.
Aurèle Barrière and Clément Pit-Claudel. Linear matching of javascript regular expressions.
Proc. ACM Program. Lang., 8(PLDI), June 2024. doi:10.1145/3656431.
Martin Berglund and Brink van der Merwe. Regular expressions with backreferences reexamined. In Jan Holub and Jan Zdárek, editors, Proceedings of the Prague Stringology
Conference 2017, Prague, Czech Republic, August 28-30, 2017, pages 30–41. Department of
Theoretical Computer Science, Faculty of Information Technology, Czech Technical University
in Prague, 2017. URL: http://www.stringology.org/event/2017/p04.html.
Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras. Verified and efficient
matching of regular expressions with lookaround. In Proceedings of the 14th ACM SIGPLAN
International Conference on Certified Programs and Proofs, CPP ’25, page 198–213, New York,
NY, USA, 2025. Association for Computing Machinery. doi:10.1145/3703595.3705884.
James C. Davis, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. The impact of
regular expression denial of service (redos) in practice: an empirical study at the ecosystem
scale. In Proceedings of the 2018 ACM Joint Meeting on European Software Engineering
Conference and Symposium on the Foundations of Software Engineering, ESEC/SIGSOFT
FSE 2018, pages 246–256. ACM, 2018. doi:10.1145/3236024.3236027.
James C. Davis, Louis G. Michael IV, Christy A. Coghlan, Francisco Servant, and Dongyoon
Lee. Why aren’t regular expressions a lingua franca? an empirical study on the re-use and
portability of regular expressions. In Proceedings of the 2019 27th ACM Joint Meeting on
European Software Engineering Conference and Symposium on the Foundations of Software
Engineering, ESEC/FSE 2019, page 443–454, New York, NY, USA, 2019. Association for
Computing Machinery. doi:10.1145/3338906.3338909.
Noé De Santo, Aurèle Barrière, and Clément Pit-Claudel. A coq mechanization of javascript
regular expression semantics. Proc. ACM Program. Lang., 8(ICFP):1003–1031, 2024. doi:
10.1145/3674666.
Mark Jason Dominus. Perl regular expression matching is NP-complete. https://perl.
plover.com/NPC/NPC-3SAT.html, 2000.
ECMA. Ecma-262, 14th edition: Ecmascript® 2023 language specification, June 2023. URL:
https://262.ecma-international.org/14.0/.
ECMA. Ecma-262, 15th edition: Ecmascript® 2024 language specification, June 2024. URL:
https://262.ecma-international.org/15.0/.
ECMA. Ecma-262, 16th edition: Ecmascript® 2025 language specification, June 2025. URL:
https://262.ecma-international.org/16.0/.
Andrzej Ehrenfeucht and Paul Zeiger. Complexity measures for regular expressions. In
Proceedings of the Sixth Annual ACM Symposium on Theory of Computing, STOC ’74, page
75–79, New York, NY, USA, 1974. Association for Computing Machinery. doi:10.1145/
800119.803886.

35

<!-- page 36 -->

36

On the computational complexity of JavaScript regex matching, in Rocq

14

15
16

17
18

19
20
21

22

23

24
25
26

27

28

29
30

Yannick Forster, Fabian Kunze, and Marc Roth. The weak call-by-value λ-calculus is reasonable
for both time and space. Proc. ACM Program. Lang., 4(POPL), December 2019. doi:
10.1145/3371095.
Dominik D. Freydenberger. Extended regular expressions: Succinctness and decidability.
Theory of Computing Systems, 53(2):159–193, Aug 2013. doi:10.1007/s00224-012-9389-0.
Lennard Gäher and Fabian Kunze. Mechanising Complexity Theory: The Cook-Levin Theorem
in Coq. In Liron Cohen and Cezary Kaliszyk, editors, 12th International Conference on
Interactive Theorem Proving (ITP 2021), volume 193 of Leibniz International Proceedings in
Informatics (LIPIcs), pages 20:1–20:18, Dagstuhl, Germany, 2021. Schloss Dagstuhl – LeibnizZentrum für Informatik. URL: https://drops.dagstuhl.de/entities/document/10.4230/
LIPIcs.ITP.2021.20, doi:10.4230/LIPIcs.ITP.2021.20.
Michael R. Garey and David S. Johnson. Computers and Intractability; A Guide to the Theory
of NP-Completeness. W. H. Freeman & Co., USA, 1990.
Wouter Gelade. Succinctness of regular expressions with interleaving, intersection and counting.
Theoretical Computer Science, 411(31):2987–2998, 2010. URL: https://www.sciencedirect.
com/science/article/pii/S0304397510002537, doi:10.1016/j.tcs.2010.04.036.
Wouter Gelade and Frank Neven. Succinctness of the complement and intersection of regular
expressions. ACM Trans. Comput. Logic, 13(1), January 2012. doi:10.1145/2071368.2071372.
Philip Hazel. pcre2pattern specification. www.pcre.org. URL: https://www.pcre.org/
current/doc/html/pcre2pattern.html.
Michal Horký, Juraj Síč, and Lenka Turoňová. Automata with bounded repetition in re2.
In Roberto Moreno-Díaz, Franz Pichler, and Alexis Quesada-Arencibia, editors, Computer
Aided Systems Theory – EUROCAST 2022, pages 232–239, Cham, 2022. Springer Nature
Switzerland.
Mark W. Krentel. The complexity of optimization problems. Journal of Computer and System
Sciences, 36(3):490–509, 1988. URL: https://www.sciencedirect.com/science/article/
pii/0022000088900396, doi:10.1016/0022-0000(88)90039-6.
Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras. Regular expression matching
using bit vector automata. Proc. ACM Program. Lang., 7(OOPSLA1), April 2023. doi:
10.1145/3586044.
Taisei Nogami and Tachio Terauchi. Hardness of regular expression matching with extensions,
2026. URL: https://arxiv.org/abs/2601.03020, arXiv:2601.03020.
Keegan Kyle Perry. Verified derivative-based regular expression matching. Master’s thesis,
Stellenbosch University, 2025.
Steffen Reith and Heribert Vollmer.
Optimal satisfiability for propositional calculi and constraint satisfaction problems. Information and Computation, 186(1):1–19,
2003. URL: https://www.sciencedirect.com/science/article/pii/S0890540103000920,
doi:10.1016/S0890-5401(03)00092-0.
R. E. Stearns and H. B. Hunt III. On the equivalence and containment problems for unambiguous regular expressions, regular grammars and finite automata. SIAM Journal on Computing,
14(3):598–611, 1985. arXiv:https://doi.org/10.1137/0214044, doi:10.1137/0214044.
L. J. Stockmeyer and A. R. Meyer. Word problems requiring exponential time(preliminary
report). In Proceedings of the Fifth Annual ACM Symposium on Theory of Computing,
STOC ’73, page 1–9, New York, NY, USA, 1973. Association for Computing Machinery.
doi:10.1145/800125.804029.
The Perl 5 Porters. perlre - Perl regular expressions - Perldoc Browser. perldoc.perl.org. URL:
https://perldoc.perl.org/perlre.
Lenka Turoňová, Lukáš Holík, Ivan Homoliak, Ondřej Lengál, Margus Veanes, and Tomáš
Vojnar. Counting in regexes considered harmful: Exposing ReDoS vulnerability of nonbacktracking matchers. In 31st USENIX Security Symposium (USENIX Security 22), pages
4165–4182, Boston, MA, August 2022. USENIX Association. URL: https://www.usenix.org/
conference/usenixsecurity22/presentation/turonova.

<!-- page 37 -->

V. Deng, A. Barrière and C. Pit-Claudel

31

32

Yuya Uezato. Regular Expressions with Backreferences and Lookaheads Capture NLOG.
In Karl Bringmann, Martin Grohe, Gabriele Puppis, and Ola Svensson, editors, 51st
International Colloquium on Automata, Languages, and Programming (ICALP 2024),
volume 297 of Leibniz International Proceedings in Informatics (LIPIcs), pages 155:1–
155:20, Dagstuhl, Germany, 2024. Schloss Dagstuhl – Leibniz-Zentrum für Informatik.
URL: https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2024.155,
doi:10.4230/LIPIcs.ICALP.2024.155.
Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner. Lean formalization of extended
regular expression matching with lookarounds. In Proceedings of the 13th ACM SIGPLAN
International Conference on Certified Programs and Proofs, CPP 2024, page 118–131, New
York, NY, USA, 2024. Association for Computing Machinery. doi:10.1145/3636501.3636959.

37
