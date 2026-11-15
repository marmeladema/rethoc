---
file: ../pdfs/2017-psc-berglund-backreferences-reexamined.pdf
citation: "Regular Expressions with Backreferences Re-examined (conference version of the TCS paper). Berglund, van der Merwe. Prague Stringology Conference 2017"
doi: 
source: "http://www.stringology.org/cgi-bin/getfile.cgi?t=pdf&c=-&y=2017&n=04"
topics: [redos_backrefs_verification]
pages: 12
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Regular Expressions with Backreferences
Re-examined
Martin Berglund1,2 and Brink van der Merwe3
1

Department of Information Science, Stellenbosch University, South Africa
2
Center for AI Research, CSIR, Stellenbosch University, South Africa
3
Department of Computer Science, Stellenbosch University, South Africa

Abstract. Most modern regular expression matching libraries (one of the rare exceptions being Google’s RE2) allow backreferences, operations which bind a substring to a
variable allowing it to be matched again verbatim. However, different implementations
not only vary in the syntax permitted when using backreferences, but both implementations and definitions in the literature offer up a number of different variants on how
backreferences match. Our aim is to compare the various flavors by considering the formal languages that each can describe, resulting in the establishment of a hierarchy of
language classes. Beyond the hierarchy itself, some complexity results are given, and as
part of the effort on comparing language classes new pumping lemmas are established,
and old ones extended to new classes.

1

Introduction

Regular expressions as used and implemented in practice are vastly different from
their traditional theoretic counterpart, both in semantics (driven by the features
offered), and expectations of performance. Even when not using the more complex
features the performance profile of practical regular expression matching is a fairly
deep subject matter, which has seen theoretical study only fairly recently, such as
in [2] and [7]. In this paper we focus on regular expressions with backreferences
(rewbr for short), an advanced feature which is available in most regular expression
matching libraries. This subject matter has seen some study in the literature, we will
refer frequently to [1], [4], and [5], but each paper has its own definition of a rewbr
and its semantics ([4] has two), and many implementations disagree with all of them
(the definition given by Aho in [1] is common however), and with each other.
A backreference is placed in a regular expression to indicate that the substring
matched by some specified capturing group (where capturing group is synonymous
with parenthesized subexpression), should be matched again at the position (or positions) where the backreference is placed. In the Java programming language we
denote by \i that the substring most recently matched by the ith capturing group
should be matched by the backreference again, where capturing groups are numbered
from 1 onwards, based on the relative position of their left parenthesis when reading
the regular expression from left to right. For example, [0-9]+\.\d*(\d+)\1+ can be
used to match recurring decimal numbers, such as 0.33, 0.818181 and 0.04555, since
the subexpression (\d+) captures some sequence of digits in the input string and the
backreference \1+ instructs the matcher to match this sequence again, one or more
times. Similarly, the regular expression (.+)\1 matches strings of the form ww, i.e.
producing the non-context-free reduplication property.
Long and complicated regular expressions may be hard to read and maintain as
adding or removing capturing groups changes the numbers of all groups following the
Martin Berglund, Brink van der Merwe: Regular Expressions with Backreferences Re-examined, pp. 30–41.
Proceedings of PSC 2017, Jan Holub and Jan Žd’árek (Eds.), ISBN 978-80-01-06193-0 c Czech Technical University in Prague, Czech Republic

<!-- page 2 -->

M. Berglund, B. van der Merwe: Regular Expressions with Backreferences Re-examined

31

modification. Python’s re module was the first to offer a solution in terms named
capturing groups and backreferences — (?P<name>group) captures the match of
the subexpression group into name, whereas a backreference to the contents of this
capturing group is done with (?P=name). In some implementations it is then possible
to reuse the same label for different capturing groups (e.g. Python and .NET both
allow naming of groups, but .NET allows reusing names where Python does not),
which opens possibilities obviously not available when simply numbering capturing
groups from left to right. Also, regular expression matchers use different conventions
in terms of how matching is defined when encountering a backreference without having
captured a substring with a label corresponding to the backreference. These subtle
differences in syntax and semantics allowed in rewbr influence the classes of languages
described, as well as the relative succinctness of the rewbr variants. It is thus clear
that a thorough comparison of rewbr variants is needed if further study is to be
possible, which forms a big part of our contribution.
This paper uses as starting point the definitions and results, on rewbr, from [1],
[4], [6] and [5]. In particular, the structure of the definition of matching semantics of
rewbr is taken from [6], and the pumping lemma from [4] (for rewbr) provides the
intuition for our own more general pumping lemmas.
The outline of the paper is as follows. After providing the necessary notation and
definitions in the next section, we first give some improvements on past complexity
results (demonstrating some differences between the classes), we then develop various
pumping lemmas and then describe the relationships between the language classes
obtained when considering the variants of rewbr as found in theory and practice.

2

Notation and Definitions

We use Σ and Φ as finite input and backreference alphabets respectively, with these
(possibly empty) alphabets being disjoint. Also, ∅ and ε denote the empty set and
word respectively, and N the set of natural numbers including 0. To improve readability, we sometimes denote v1 = w1 , . . . , vn = wn as (v1 , . . . , vn ) = (w1 , . . . , wn ). For
a string w over Σ (or any other alphabet), we denote by |w| the length of w, i.e. the
number of occurrences of symbols from Σ in w, and more generally, if Σ ′ ⊆ Σ, then
|w|Σ ′ is the number of occurrences of symbols from Σ ′ in w. For sets A and B of
strings, the concatenations A · B is defined as usual as {w1 w2 | w1 ∈ A, w2 ∈ B} and
i
i
the Kleene closure of A, denoted as A∗ , as {ε} ∪ (∪∞
i=1 A ), where A is the concatenation of A with itself using the concatenation operator (i − 1) times.
Definition 1. A regular expression with backreferences (rewbr) over input alphabet
Σ and backreference alphabet Φ is defined inductively as:
(1) ∅, or an element of Σ ∪ {ε}, or
(2) an expression of the form (E1 | E2 ), (E1 · E2 ), or, (E1 ∗ ), the Kleene closure, for
any rewbr E1 and E2 , or,
(3) for φ ∈ Φ, the expression [φ E]φ , i.e. a capturing group labeled by φ, or ↑φ , a
backreference to a (possibly non-existing) capturing group labeled by φ.
Let rewbrΣ,Φ denote all rewbr over input alphabet Σ and backreference alphabet Φ.
The subset of rewbr obtained by using only (1) and (2) above (i.e. regular expressions
over Σ without backreferences), is denoted by RΣ .

<!-- page 3 -->

32

Proceedings of the Prague Stringology Conference 2017

We use Σ and Φ to indicate a generic input and backreference alphabet respectively, without stating it explicitly.
As usual, parenthesis may be elided from rewbr by using the rule that Kleene
closure ‘ ∗ ’ takes precedence over concatenation ‘ · ’, which takes precedence over union
‘ | ’. In addition, outermost parenthesis may be dropped and E1 · E2 abbreviated as
E1 E2 . The brackets which denote a capturing group may not be elided (except if no
corresponding backreference appears in the rewbr). Also, for E ∈ rewbrΣ,Φ , we use
E + as abbreviation for E · E ∗ .
When E ∈ RΣ , the language described by E, denoted as L(E), is defined inductively as usual, i.e. L(∅) = ∅, L(a) = {a} for a ∈ Σ ∪ {ε}, L(E1 | E2 ) = L(E1 ) ∪ L(E2 ),
L(E1 · E2 ) = L(E1 ) · L(E2 ) and L(E1 ∗ ) = L(E1 )∗ .
Following [6] and [5], we use ref-words (short for reference words), to define the
matching semantics of rewbr, instead of using the approach of Câmpeanu et al. in [4].
Câmpeanu and his co-authors used parse trees as mechanism to describe the way in
which a string is matched in terms of which substring of the input string is captured
by which subexpression of the rewbr. A backreference then matches a substring that
is equal to the closest matched substring w′ to its left in the input string, where w′
was matched/captured by a subexpression labeled by the same symbol as used by
the backreference. The ref-words and parse tree approaches of arriving at matching
semantics are indeed equivalent. We use the ref-words approach, explained next, since
it allows us to show that the various pumping lemmas for rewbr is a direct consequence
of the regular language counterpart.
Let ΣΦ = Σ ∪ {φ, (φ , )φ | φ ∈ Φ} and w ∈ (ΣΦ )∗ . Then if φ appears in w and no
subword of the form (φ u)φ appears to the left of φ (in w), we say that the φ is unbound.
We define a function Dε : (ΣΦ )∗ → Σ ∗ by using the following steps to obtain Dε (w)
from w. First replace all instances of φ by ε if φ is unbound. Next replace iteratively,
φ by u, if (φ u)φ is the closest subword to the left of φ in w starting and ending with
‘(φ ’ and ‘)φ ’ respectively, and u ∈ Σ ∗ . Finally, delete all symbols from {(φ , )φ | φ ∈ Φ}.
The order in which the replacements are made in the second step, has no effect on
the final word obtained, and thus we may assume it is made from left to right.
On occasion we are interested in the image of a specific substring in an input
string under Dε , which obviously depends on the prefix to the left of the substring
of interest. We denote by Dε,w (w′ ) the string obtained by removing the prefix Dε (w)
from Dε (ww′ ).
Similarly to Dε , we let D∅ : (ΣΦ )∗ → Σ ∗ be the partial function with D∅ (w)
undefined if w contains an unbound reference, and being equal to Dε (w) otherwise.
The partial function D∅,w (w′ ) is defined in the same way as Dε,w (w′ ).
We denote by B(ΣΦ ) the subset of (ΣΦ )∗ of strings with well-balanced parenthesis
from {(φ , )φ | φ ∈ Φ}. In our exposition we only use the case where the function
and partial function Dε and D∅ are applied on strings which are prefixes of words in
B(ΣΦ ), and in fact, Dε and D∅ are mostly applied on strings from B(ΣΦ ).
For E ∈ rewbrΣ,Φ , we denote by ref-regex(E) the regular expression E ′ ∈ RΣΦ
obtained by replacing ↑φ , [φ , ]φ with φ, (φ , )φ respectively.

Example 2. Let Σ = {a}, Φ = {0, 1}, and E = ([0 a↑1 ]0 [1 ↑0 ↑0 ]1 )∗ . Then E ′ =
ref-regex(E) = ((0 a1)0 (1 00)1 )∗ and for w′ = (0 a1)0 (1 00)1 (0 a1)0 (1 00)1 ∈ L(E ′ ) we

<!-- page 4 -->

M. Berglund, B. van der Merwe: Regular Expressions with Backreferences Re-examined

33

have that Dε (w′ ) = a12 , which is obtained by rewriting w′ as follows:
w′ → (0 a)0 (1 02 )1 (0 a1)0 (1 02 )1 → (0 a)0 (1 a2 )1 (0 a1)0 (1 02 )1
→ (0 a)0 (1 a2 )1 (0 a3 )0 (1 02 )1 → (0 a)0 (1 a2 )1 (0 a3 )0 (1 a6 )1
→ a12
The partial function D∅ is undefined on all of L(E ′ ), with the exception of ε,
since all non-empty strings in L(E ′ ) has (0 a1)0 (1 00)1 as prefix, in which the 1 in the
substring (0 a1)0 is unbound.
Remark 3. Note that |Dε (w)| ≤ 2|w| (and similarly for D∅ (w)), since each time we
substitute a backreference φ with a substring w′ (where w′ ∈ Σ ∗ ), we at most double
the length of the string. More generally, we have |Dε,w1 (w2 )| ≤ max(1, |Dε (w1 )|)2|w2 | ≤
2|w1 |+|w2 | , where the first inequality follows from the fact that in computing Dε,w1 (w2 )
from w2 by substitution, we may immediately use captured substrings of w1 (if w1 6= ε)
for substitution as we process w2 from left to right.
Definition 4. For E ∈ rewbrΣ,Φ , we define the language described by E based on εsemantics and ∅-semantics, and denoted by Lε (E) and L∅ (E) respectively, as follows:
– Lε (E) = {Dε (w) | w ∈ L(ref-regex(E))}
– L∅ (E) = {D∅ (w) | w ∈ L(ref-regex(E)) and D∅ (w) is defined }
Definition 5. We obtain variants of rewbr by using Lε (r) or L∅ (r) or syntactically
restricting the rewbr we consider in rewbrΣ,Φ by not allowing more than one occurrence
of [φ for each φ (i.e. capturing labels may not be repeated) in the rewbr we consider.
The four variants are then:
No label repetitions
May repeat labels
ε-semantics Câmpeanu-Salomaa-Yu Freydenberger-Schmid [5]
semi-regex [4]
∅-semantics
Java, Python
Aho [1], Boost,
PCRE, .NET
A fifth variant is the extended (non-semi) regexes of [4], which additionally require
that ↑φ only occur to the right of the occurrence of ]φ in the rewbr.
When we distinguish between these variants we call them (going left-to-right topto-bottom) semi-CSY-, FS-, Java-, and Aho-style, with the addition of CSY-style to
refer to the full (non-semi) regexes of [4]. We denote by Lx the class of languages
matched by an x-style variant rewbr.
Example 6. The expression ↑1 [1 Σ ∗ ]1 can be interpreted as a semi-CSY-, FS-, Java-,
or Aho-style rewbr, but not a CSY-style one (as ↑1 occurs before ]1 ). However, the
semi-CSY- and FS-style rewbr described by that expression matches Σ ∗ , whereas the
Java- and Aho-style ones match ∅, by the difference between ε- and ∅-semantics set
out in Definition 4. Meanwhile the expression ([1 a∗ ]1 |[1 (b | c)]1 )↑1 can only describe
either an FS- or Aho-style rewbr (as it repeats labels), but in this instance they match
the same language, as the final ↑1 always refers to something bound, eliminating the
distinction between Dε and D∅ .
Remark 7. The (CSY- and Java-style) restriction of not allowing repeated labels,
leads to unnatural closure properties of the respective classes of rewbr, since if E =
F (G | H) is of CSY- or Java-style, and F contains a capturing group, then in E ′ =
(F G | F G) a label is repeated, and E ′ is thus not CSY- or Java-style.

<!-- page 5 -->

34

Proceedings of the Prague Stringology Conference 2017

Remark 8. The additional restriction used to obtain CSY-style rewbr can be used in
conjunction with the other four variants to obtain eight variants of rewbr in total, but
by doing so, we end up with an additional three variants which appear to not have
been considered before in literature, nor been used in practical matching software,
making them of little interest to us.
Remark 9. In [5], Freydenberger and Schmid disallowed rewbr with subexpressions of
the form [φ · · · ↑φ · · · ]φ (i.e. backreferences within a capturing group using the same
label), since their memory automaton model, which provides a state machine equivalent formalism for the class of languages equivalent to FS-style rewbr (with this
additionally stated constraint), has a memory location for each capture symbol, but
it is not possible to update a memory location (of a memory automaton) and use its
previous content at the same time. We, however, do not consider this restriction.
Remark 10. Notice that rewbr (independent of the choice of variant from Definition 5) are exponentially more succinct than regular expressions for some languages,
for example the family
En = [0 a]0 [1 ↑0 ↑0 ]1 · · · [n ↑n−1 ↑n−1 ]n
n+1

has L(En ) = {a2 −1 }, which is exponential in the length of the expression itself.
By contrast, a regular expression is always at least as long as the shortest string it
matches.
Next we define a generalization of the syntactic constraint that was used to define
the CSY subclass of semi-CSY-style rewbr.
Definition 11. For E ∈ rewbrΣ,Φ , we define the relation ∼E on Φ as φ ∼E φ′ for
φ, φ′ ∈ Φ, if E contains a subexpression of the form [φ · · · ↑φ′ · · · ]φ . Let ≈E be the
transitive closure of ∼E . Then E is non-circular if it is not the case that φ ≈E φ for
any φ ∈ Φ.
In a similar way as in the definition above, we define when strings in B(ΣΦ ) are
non-circular, and note that if w ∈ L(ref-regex(E)), with E non-circular, then w is also
non-circular. Note that the rewbr in Example 2 is circular, while En in Remark 10 is
non-circular.
Remark 12. The class of CSY-style rewbr has the unnatural closure (or more precisely, non-closure) property that if we start with E of CSY-style and replace in E a
subexpression of the form (F1 |F2 ) by (F2 |F1 ) to obtain E ′ , then E ′ might no longer be
of CSY-style (but of course still semi-CSY-style). This makes it clear that non-circular
rewbr (or non-circular semi-CSY) is a more natural subclass of rewbr to consider.

3

The Complexity of Backreference Matching

It is shown already in [1] that matching a rewbr to a string is NP-complete in general.
In that proof a reduction from Vertex Cover is performed, with a large alphabet.
As usual the alphabet can be reduced to a binary one by straightforward encoding
of symbols, but we take one step further and prove that the matching problem for
rewbr is NP-complete even for a unary alphabet.

<!-- page 6 -->

M. Berglund, B. van der Merwe: Regular Expressions with Backreferences Re-examined

35

Theorem 13. Uniform membership testing a rewbr (independent of the choice of
semantics from Definition 5) over alphabet Σ is NP-complete even for |Σ| = 1.
Proof. We demonstrate this by a reduction from Satisfiability (deciding satisfiability of propositional formulas on conjunctive normal form). For any instance of such
a formula, c1 ∧ · · · ∧ cn over the variables x1 , . . . , xm , first, for each clause ci construct
the rewbr ri as the union of backreferences for every literal in the disjunction. That
is, if ci = x3 ∨ x7 ∨ x9 (where x represents the literal negating the variable x, here
viewed as a single symbol) then ri = ↑x3 | ↑x7 | ↑x9 . Then construct the rewbr:
R = ([x1 a]x1 |[x1 a]x1 ) · · · ([xm a]xm |[xm a]xm )r1 · · · rn
We then argue that am+n ∈ L(R) if and only if the formula is satisfiable. This is
straightforward: clearly at most m + n symbols can be read (as the expression is
a concatenation of m + n unions). The initial sequence of unions corresponding to
variables will read m symbols, in the process defining a capture either xi or xi for
each i. The only way to read another n symbols is if every union contains at least
one backreference to a literal which was chosen in the first phase. This corresponds
precisely to assigning truth values to the variables, and requiring that each disjunction
in the original formula contains at least one true literal.
The problem is in NP for all alphabet sized by a straightforward search argument over the expression. Starting at the left hand side of the expression nondeterministically search for a path through the expression in the obvious way, consuming symbols from the string as needed. If the right of the expression is reached
with the entire string consumed the search accepts. This search can be restricted
to polynomially many steps by rejecting whenever it would visit a position in the
expression twice without either consuming a symbol or passing through a previously
unvisited capturing group (i.e. defining ↑φ for some φ where it was previously undefined) in an intervening step. The latter case is necessary for Java- and Aho-style
semantics when matching e.g. ([1 a∗ ]1 |[2 b∗ ]2 )∗ ↑1 ↑2 c to the string c, having to repeat
the first Kleene closure twice to get ↑1 and ↑2 initialized. This easily gives a bound
of |E|2 |w| (heavily overestimating), as there are |E| positions, and no more than |E|
capturing groups which may get defined.
⊓
⊔
Using the above result we can further demonstrate that some of the rewbr semantics we consider also give rise to a difficult emptiness problem.
Theorem 14. For Java- and Aho-style semantics uniform membership testing and
emptiness checking is NP-complete, even for Σ = ∅.
Proof. For the empty alphabet emptiness checking and membership testing is equivalent, as the only string that can be in the language matched is ε. Use the same
reduction that was shown in Theorem 13, but remove all as from the rewbr R. Now
ε ∈ L∅ (R) if and only if the formula is satisfiable.
This is easy to see, as Java- and Aho-style rewbr have the ∅-semantics defined by
the D∅ function, which does not permit ↑xi to match anything if it is unbound (i.e. if
the capturing group labeled xi has not been matched to something). Therefore each
clause must contain some literal chosen to match in the first part of the expression
(despite the captures simply being the empty string), which again simulates assigning
truth values to the variables.
Membership is in NP for all alphabet sizes by the argument in Theorem 13.
Emptiness is also in NP by a similar search argument, simply ignoring what symbols

<!-- page 7 -->

36

Proceedings of the Prague Stringology Conference 2017

are being consumed. As some expressions may contain only long strings (see e.g.
Remark 10) a witness string must not be explicitly constructed, but it is sufficient for
the search to track which capturing groups have been visited, capturing some string,
not caring which.
⊓
⊔
Remark 15. It should be clear that (semi-)CSY-/FS-style semantics have linear-time
emptiness-checking (and therefore membership testing with |Σ| = 0), as an expression
is only empty if it is a concatenation with one empty sub-expression, or is a union with
both sub-expressions empty, or it equals ∅. In practical implementations ∅ is seldom
even available, as it has very limited usefulness, making practical emptiness-checking
constant time, since then no CSY-/FS-style expression is empty).
It is reasonably obvious, by practical use if nothing else, that the difficulty of rewbr
matching is not insurmountable. If used with care, capturing in contexts where the
ambiguity is low (i.e. the number of options for capturing is limited) the performance
impact can be minimized. Practical regular expression libraries (all mentioned here)
often have operators specifically aimed at managing such ambiguity, see for example [3]. A deeper study of the fixed parameter complexity of matching will, however,
be left as future work in this paper.

4

Pumping Lemmas with Backreference Matching

The pumping lemma given in [4] is a useful tool for finding languages that cannot
be matched by CSY-style rewbr. It is used in the next section to show that LCSY (
Lsemi-CSY . First we recall the definition of the pumping lemma, which we will then
consider in the context of the additional semantics treated here, to then introduce a
more generalized pumping lemma.
Lemma 16 (from [4]). For every L ∈ LCSY (i.e. any language matched by some
CSY-style rewbr) there exists a constant k such that if w ∈ L with |w| > k, then there
is a decomposition w = x0 vx1 vx2 · · · vxn , for some n ≥ 1, such that:
– |x0 v| < k;
– |v| ≥ 1; and,
– x0 v i x1 v i x2 · · · v i xn ∈ L(E) for all i ≥ 1.
First we note that this pumping lemma does not apply to most of the other styles
considered here. We satisfy ourselves with proving that it does not hold for semi-CSYstyle, extending the proof to FS- and Aho-style is straightforward, but Lemma 22 will
later on achieve the same result by demonstrating that languages matched by semiCSY-style forms a subclass of FS- and Aho-style.
Lemma 17. The pumping lemma of [4] does not hold for semi-CSY-style rewbr.
Proof. This follows from there being exponentially growing languages matched by
n
semi-CSY-style rewbr. Let E = ([α ↑β ↑β ]α [β aα]β )∗ . Then L = Lε (E) = {a2 −1 | n ≥ 1}
and L ∈ Lsemi-CSY . The result now follows by observing that the pumping lemma
recalled in Lemma 16 does not hold for L, as it implies there would exist some k, n
and v such that ak+i·n|v| ∈ L for i ≥ 1, which precludes strict exponential growth. ⊓
⊔
However, this pumping lemma does hold for Java-style rewbr.
Lemma 18. The pumping lemma of [4] also holds for Java-style rewbr.

<!-- page 8 -->

M. Berglund, B. van der Merwe: Regular Expressions with Backreferences Re-examined

37

Proof. The intuition for why Java-style rewbr differs from semi-CSY-style in this
regard is that, while there may be circular capturing groups in Java, the first capture
in the cycle must be possible to perform without using any of the other backreferences
in the cycle (as they will be unbound). Since the capturing labels cannot repeat, the
option of not using any backreference in the capturing sub-expression will then remain
on every subsequent repetition of the cycle, making it possible to “restart” the cycle
at will. A formal argument follows.
Let E be a Java-style rewbr, set k = 2|E| , to match a string w with |w| > k
some Kleene closure must be repeated at least once (matching a backreference may
at most double the length of the string matched, see e.g. Remark 3, and, obviously, E
contains at most linearly many backreferences). Fix one particular match for w, and
take the Kleene closure which first repeats a full match of its enclosed subexpression,
x0 vx1 vx2 · · · vxn , where the first v is the substring matched by the the first repetition
of the F subgroup, and each following v is produced by a backreference to that initial
matching of the subgroup (obviously n may be one if the capture is never referred to).
Then we argue that in Java-style semantics x0 v i x1 v i x2 · · · v i xn ∈ L(E) for all i ≥ 1.
This is the case as, by assumption, the match of v was the first entry into F ,
and as unbound backreferences do not match in Java-style, and capture group labels
may not repeat, this means the match of v used a path through F on which no
backreference is used which is subsequently assigned by a capture inside F (as such a
backreference would have had to be undefined). This means that if F is repeated, it
will be able to match v again, any number of times, without changing any capturing
group contents (performing the same captures as it did the first time), using the same
path through F in each instance. The remaining vs, down the line, are produced by
backreferences and need no special argument.
⊓
⊔
In the next three lemmas we develop a more general pumping lemma for LCSY .
Lemma 19. For L ∈ LCSY there exists a constant kL such that if w0 w ∈ L with
|w| ≥ kL max(1, |w0 |), then we have strings u, x, y, z such that uxyz, y ∈ B(ΣΦ ), with:
– (Dε (u), Dε,u (xyz)) = (w0 , w) and thus Dε (uxyz) = w0 w;
– |Dε,u (xy)| ≤ kL max(1, |w0 |);
– Dε,uxyi (y) = Dε,ux (y) 6= ε for all i ≥ 0; and
– Dε (uxy ∗ z) ⊆ L.
Proof. Assume L = Lε (E), E ′ = ref-regex(E), p > |E ′ | (i.e. p is a pumping constant for the regular language L(E ′ )) and kL = 2p . The result now follows from
the relationship between regular languages and LCSY via the function Dε , Remark 3
and the pumping lemma applied to the regular language L(E ′ ). Next the details.
We have u, u′ with (Dε (u), Dε,u (u′ )) = (w0 , w) and uu′ = L(E ′ ). From Remark 3,
|u′ | ≥ p. The pumping lemma for L(E ′ ) implies we have x, y, z with u′ = xyz,
uxy ∗ z ⊆ L(E ′ ) ⊆ B(ΣΦ ) and thus Dε (uxy ∗ z) ⊆ L. We may assume y is matched
by F , with F ∗ a subexpression of E ′ and F not containing Kleene stars. To get
|D∅,ux (y)| ≥ 1, we consider all possible non-empty substrings y of u′ matched by an
F with F ∗ a subexpression of E ′ , and if for all of them |D∅,ux (y)| = 0, we would get
a contradiction to |w| ≥ kL max(1, |w0 |). By picking the first possible y (to the left)
with |D∅,ux (y)| ≥ 1, we also ensure |Dε,u (xy)| ≤ kL max(1, |w0 |), by using Remark 3.
Finally, to obtain Dε,uxyi (y) = Dε,ux (y) for all i ≥ 0, we need the CSY assumption
that backreferences do not appear before corresponding capturing subexpressions

<!-- page 9 -->

38

Proceedings of the Prague Stringology Conference 2017

in E, which implies that if y = y1 φy2 , then y1 contains a substring of the form (φ y ′ )φ .
⊓
⊔
This is enough to ensure Dε,uxyi (y) = Dε,ux (y).
Lemma 20. Assume x, y, z are strings with xyz, y ∈ B(ΣΦ ) and xyz non-circular
with (Dε (x), Dε,x (y)) = (x0 , v). Also assume if y = y1 φy2 , then y1 contains a substring
of the form (φ y ′ )φ . Then for i ≥ 1, Dε (xy i z) = x0 v i x1 v i x2 · · · v i xn , for some strings
x1 , . . . , xn where n ≥ 1.
Proof. If xyz = w0 (φ w1 yw2 )φ w3 , where w3 has (n − 1) occurrences of the symbol φ
before any substring of the form (φ w)φ (which includes the case of w3 not having a
substring of the form (φ w)φ ), then Dε (xy i z) is as specified. Otherwise, if y is not
properly contained in a substring of the form (φ w)φ , we have n = 1.
⊓
⊔
To see that the non-circular requirement is necessary in Lemma 20, take x =
z = ε and y = (0 a1)0 (1 00)1 . Then Dε (y) = a3 , Dε (y 2 ) = a12 , Dε (y 3 ) = a33 , and in
general, |Dε (y i )| ≥ 3i . Also, if (x, y, z) = ((0 a)0 , 0(0 b)0 , ε), then Dε (xyz) = a2 b, while
Dε (xy i+1 z)) = a2 bb2i , and thus the reason for assuming if y = y1 φy2 , then y1 contains
a substring of the form (φ y ′ )φ .
Lemma 21. For L ∈ LCSY there is a constant kL such that if w0 w ∈ L with |w| ≥
kL max(1, |w0 |), then there are strings x0 , . . . , xn , v, for some n ≥ 1, with |v| ≥ 1, so
that we have:
– w = x0 vx1 vx2 · · · vxn ; and
– w0 x0 v i x1 v i x2 · · · v i xn ∈ L for all i ≥ 1.
Proof. From Lemma 19 we have u, x, y, z with (Dε (u), Dε,u (xyz)) = (w0 , w) (and
Dε (uxyz) = ww0 ), Dε,uxyi (y) = v 6= ε for i ≥ 0, with Dε (uxy ∗ z) ⊆ L. L ∈ LCSY
implies we can use Lemma 20 and conclude that there is some n ≥ 1 so that
Dε (uxy i z) = w0 x0 v i x1 v i x2 · · · v i xn ∈ L, for i ≥ 1.
⊓
⊔
Beyond its general usefulness, Lemma 21 will be used to distinguish between the
language classes matched by CSY- and Java-style rewbr in Lemma 25.

5

Language Hierarchies

In the previous section several containment relationships between the language classes
which can be matched by the different styles of rewbr were established, in this section
we refine this further. Let us begin by combining and summarizing a few straightforward relations with what was already established in previous sections.
Lemma 22. The following inclusions hold: LCSY ( Lsemi-CSY ⊆ LFS ⊆ LAho , and in
addition, Lsemi-CSY 6⊆ LJava ⊆ LAho .
Proof. LCSY ⊆ Lsemi-CSY ⊆ LFS and LJava ⊆ LAho follow directly by Definition 5, by
explicit restrictions placed on the styles. Lsemi-CSY 6⊆ LCSY and Lsemi-CSY 6⊆ LJava is
shown in Lemma 17. Finally, LFS ⊆ LAho since Aho-style can simulate FS-style, i.e.
LFS ⊆ LAho , since if E ∈ rewbrΣ,Φ and E ′ = [φ1 ]φ1 · · · [φn ]φn E, where ↑φ1 , . . . , ↑φn are
all of the distinct backreference symbols in E, then Lε (E) = L∅ (E ′ ).
⊓
⊔
As a first additional piece of the puzzle we demonstrate that the two most powerful
formalisms in the hierarchy are actually equivalent.

<!-- page 10 -->

M. Berglund, B. van der Merwe: Regular Expressions with Backreferences Re-examined

39

Lemma 23. LFS = LAho .
Proof. Lemma 22 already demonstrates that LFS ⊆ LAho , so all that is still needed
is to establish that LAho ⊆ LFS . Let A be Aho-style rewbr E with the property that
if w ∈ L(ref-regex(E)), then w has no unbound reference. Thus L∅ (E) = Lε (E) for
E ∈ A. Let F be a rewbr of Aho-style. We show that there exists F ′ ∈ A with
L∅ (F ) = L∅ (F ′ ) = Lε (F ′ ) and thus LAho ⊆ LFS . Let [φ1 F1 ]φ1 , . . . , [φk Fk ]φk be all
capturing subexpressions in F . We replace subexpressions in F as follows:
– subexpressions of the form F̄ ∗ are replaced by (ε | F̄ + ); and
– (F̄ | Ḡ)∗ is replaced by F̄ + Ḡ(F̄ | Ḡ)∗ | Ḡ+ F̄ (F̄ | Ḡ)∗ ).
After these replacements we use the fact that concatenation distribute over union
to obtain F ′ = (H1 | . . . | Hl ), with L∅ (F ) = L∅ (F ′ ). Each Hi is such that if ↑φk is a
subexpression of Hi , then Hi is a concatenation of subexpressions, one of which is H ′ =
[φk Fk ]φk , and this subexpression H ′ appears in the concatenation of subexpressions
forming Hi , before the subexpression containing ↑φ . Thus during a match with Hi ,
a subexpression of the form [φk Fk ]φk must be used to match a substring of the input
string, before encountering ↑φk . This property of the Hi ’s ensures that F ′ ∈ A.
⊓
⊔
Example 24. Here we illustrate part of the construction used in the proof of Lemma 23
to turn an Aho-style rewbr into a language equivalent FS-style rewbr. Let Πn be the
set of n! permutations on {1, . . . , n}, and Pn = {aπ(1) · · · aπ(n) | π ∈ Πn }. Then
L∅ (En ) = Pn , where En is the Aho-style (in fact Java-style) rewbr given by:
(a1 (1 ε)1 | . . . | an (n ε)n )n ↑φ1 . . . ↑φn
Since En contains no Kleene star operators, when we use the procedure described in
the proof of the previous theorem, we simply have to distribute concatenation over
union to obtain an FS-style (more precisely, semi-CSY-style) rewbr Fn , with L∅ (En ) =
Lε (Fn ). When we do this, we obtain that Fn is the union of n! subexpressions of the
following form, for all π ∈ Πn :
aπ(1) (π(1) ε)π(1) · · · aπ(n) (π(1) ε)π(n) ↑φ1 · · · ↑φn
Note that when distributing concatenation over union, we get many more subexpressions which are all of form b1 · · · bn ↑φ1 · · · ↑φn , with bi ∈ {a1 , . . . , an }, for 1 ≤ i ≤ n.
But when some of the bi ’s are equal (i.e when we have backreferences to non-existing
capturing groups), the languages represented by these subexpressions are empty in
Aho-style, and they are thus not used in Fn . The semi-CSY-style rewbr Fn is of
course more complicated than necessary to describe Pn , but it remains open if a more
succinct rewbr of semi-CSY-style or even FS-style exists for the language Pn , than
simply taking the union of all n! subexpressions of the form aπ(1) · · · aπ(n) .
Further, while Java does fulfill the original pumping lemma recalled in Lemma 16,
it does in fact not fulfill Lemma 21, the generalized pumping lemma for LCSY .
Lemma 25. LJava 6⊆ LCSY .
Proof. Take the Java-style rewbr E = ([0 a | ↑0 a]0 b)∗ , for which we have L(E) =
i
0
({(ab)i | i ≥ 0} ∪ {a2 b · · · a2 b | i ≥ 0})∗ . We argue that Lemma 21 does not hold
1
j
for L(E), by assuming the contrary and taking w0 = ab and w = a2 b · · · a2 b, with
|w| ≥ 2kL(E) . Let u be the non-empty pumping substring. Clearly any u consisting

<!-- page 11 -->

40

Proceedings of the Prague Stringology Conference 2017

of only as does not work, but choosing a string containing a b will under pumping
also give rise to a substring of the form · · · al bal b · · · , which is not in L(E) for any l
except for l = 1, however, that corresponds to u = ab, and the only place where that
substring could be pumped in this particular string would be in the initial prefix, but
as that prefix is taken by w0 that is not available as a choice. We thus conclude that
LJava 6⊆ LCSY .
⊓
⊔
We are with these results in hand ready to summarize the containment results for
the classes of languages matched by the various variants of rewbr.
Theorem 26. The following inclusions hold.

(

Lsemi-CSY (
6⊆

6⊆

LCSY

LJava (

LFS = LAho

Proof. Combine Lemmas 22 (in turn using Lemma 17), 23, and 25.

6

Conclusions and Future Work

These initial definitions, pumping lemmas, and inclusion proofs create a solid foundation, there are still numerous avenues for further investigation available:
– The inclusions of Theorem 26 paint a fairly clear picture, but it does remain to
show whether the non-inclusions are one side of the classes being incomparable,
or, seemingly more likely, whether they can be expanded into containments. That
is, it seems a reasonable conjecture that a completed result should read
LCSY ( LJava ( Lsemi-CSY ( LFS = LAho ,
but the actual inclusions of LCSY in LJava and LJava in Lsemi-CSY remain to be
demonstrated.
– Pumping lemmas which generalize to semi-CSY-, FS- and Aho-style rewbr should
also be found, it may be that Lemma 19 can be adapted to these cases with some
minor restatements and additional argument.
– While differences between the language classes matched seems the most important
point from a theoretical perspective, it may for practical purposes be almost more
important to determine the relative succinctness of the rewbr variants. The exponential growth exhibited by some of the classes, demonstrating the differences, is
likely not languages of very great interest for practical matching. However, how
compactly some of the languages within the intersection of the language classes
can be described could inform choices for future implementations (e.g. if Ahostyle is not more succinct on interesting cases it may be inadvisable to accept the
additional power offered by the variant).
– Finally, the most important practical questions is no doubt matching time complexity. While we give a refinement on the hardness of matching with rewbr in
Section 3 there is much more that can be done attacking this problem. Applying
parameterized complexity theory to study which aspects of the problem cause the
seeming high complexity seems a promising avenue, as practical use suggests that
suitably limited use of backreferences make for matching performance which is in
fact entirely tractable.

<!-- page 12 -->

M. Berglund, B. van der Merwe: Regular Expressions with Backreferences Re-examined

41

More broadly the area of practical regular expressions remains teeming with poorly
understood extensions and common use cases which require study to form a solid
theoretical foundation for practical string matching.

References
1. A. V. Aho: in Handbook of Theoretical Computer Science (Vol. A), J. van Leeuwen, ed., MIT
Press, Cambridge, MA, USA, 1990, ch. Algorithms for Finding Patterns in Strings, pp. 255–300.
2. M. Berglund, F. Drewes, and B. van der Merwe: Analyzing catastrophic backtracking
behavior in practical regular expression matching, in Automata and Formal Languages, Z. Ésik
and Z. Fülöp, eds., vol. 151 of Electronic Proceedings in Theoretical Computer Science, 2014,
pp. 109–123.
3. M. Berglund, B. van der Merwe, B. Watson, and N. Weideman: On the semantics of
atomic subgroups in practical regular expressions, in Implementation and Application of Automata, A. Carayol and C. Nicaud, eds., vol. 10329 of Lecture Notes in Computer Science, Springer,
2017, pp. 14–26.
4. C. Câmpeanu, K. Salomaa, and S. Yu: A formal study of practical regular expressions.
International Journal of Foundations of Computer Science, 14(6) 2003, pp. 1007–1018.
5. D. D. Freydenberger and M. L. Schmid: Deterministic regular expressions with backreferences, in 34th Symposium on Theoretical Aspects of Computer Science (STACS 2017),
H. Vollmer and B. Vallée, eds., vol. 66 of Leibniz International Proceedings in Informatics
(LIPIcs), Dagstuhl, Germany, 2017, Schloss Dagstuhl – Leibniz-Zentrum für Informatik, pp. 33:1–
33:14.
6. M. L. Schmid: Characterising REGEX languages by regular languages equipped with factorreferencing, in Developments in Language Theory - 18th International Conference, DLT 2014,
Ekaterinburg, Russia, August 26-29, 2014. Proceedings, A. M. Shur and M. V. Volkov, eds.,
vol. 8633 of Lecture Notes in Computer Science, Springer, 2014, pp. 142–153.
7. N. Weideman, B. van der Merwe, M. Berglund, and B. Watson: Analyzing matching
time behavior of backtracking regular expression matchers by using ambiguity of NFA, in Implementation and Application of Automata, Y. Han and K. Salomaa, eds., vol. 9705 of Lecture Notes
in Computer Science, Springer, 2016, pp. 322–334.
