---
file: ../pdfs/2023-pldi-moseley-derivative-nonbacktracking-matching.pdf
citation: "Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics — Moseley, Nishio, Perez Rodriguez, Saarikivi, Toub, Veanes, Wan, Xu — PLDI (PACMPL 7) 2023"
doi: 
source: "https://www.microsoft.com/en-us/research/uploads/prod/2023/04/pldi23main-p249-final.pdf"
topics: [counting_and_derivatives, redos_backrefs_verification, semantics_theory_access]
pages: 24
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Derivative Based Nonbacktracking Real-World Regex
Matching with Backtracking Semantics
DAN MOSELEY, Microsoft, USA
MARIO NISHIO, Microsoft, USA
JOSE PEREZ RODRIGUEZ, Microsoft, USA
OLLI SAARIKIVI, Microsoft, USA
STEPHEN TOUB, Microsoft, USA
MARGUS VEANES, Microsoft, USA
TIKI WAN, Microsoft, USA
ERIC XU, Microsoft, USA
We develop a new derivative based theory and algorithm for nonbacktracking regex matching that supports
anchors and counting, preserves backtracking semantics, and can be extended with lookarounds. The algorithm
has been implemented as a new regex backend in .NET and was extensively tested as part of the formal release
process of .NET7. We present a formal proof of the correctness of the algorithm, which we believe to be the
first of its kind concerning industrial implementations of regex matchers. The paper describes the complete
foundation, the matching algorithm, and key aspects of the implementation involving a regex rewrite system,
as well as a comprehensive evaluation over industrial case studies and other regex engines.
CCS Concepts: • Theory of computation → Regular languages; • Computing methodologies →
Symbolic and algebraic algorithms.
Additional Key Words and Phrases: regex, derivative, symbolic, matching, automata, PCRE
ACM Reference Format:
Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub, Margus Veanes, Tiki Wan,
and Eric Xu. 2023. Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics.
Proc. ACM Program. Lang. 7, PLDI, Article 148 (June 2023), 24 pages. https://doi.org/10.1145/3591262

1 INTRODUCTION
Regular expressions play a central role in many software applications and are supported by the
standard libraries of most popular programming languages. Several studies have shown that serious
problems can be triggered in many matching engines through regular expression denial of service
(ReDoS) attacks [OWASP 2020] as a direct result of excessive backtracking [Davis 2019; Davis
et al. 2018]. Matching engines commonly use backtracking [Spencer 1994] to support non-regular
features – such as backreferences and balancing groups – which make the language Turing complete
in general. These engines may then exhibit behavior that is quadratic or exponential in the length
of the input even for regular expressions without non-regular features, because of the generality of
Authors’ addresses: Dan Moseley, Microsoft, USA, danmose@microsoft.com; Mario Nishio, Microsoft, USA, mario.nishio@
microsoft.com; Jose Perez Rodriguez, Microsoft, USA, joperezr@microsoft.com; Olli Saarikivi, Microsoft, USA, olsaarik@
microsoft.com; Stephen Toub, Microsoft, USA, stoub@microsoft.com; Margus Veanes, Microsoft, USA, margus@microsoft.
com; Tiki Wan, Microsoft, USA, tikiwan@microsoft.com; Eric Xu, Microsoft, USA, ericxu@microsoft.com.

This work is licensed under a Creative Commons Attribution 4.0 International License.
© 2023 Copyright held by the owner/author(s).
2475-1421/2023/6-ART148
https://doi.org/10.1145/3591262
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

148

<!-- page 2 -->

148:2

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

the runtime. When exposed as part of a web service these performance issues can be exploited to
cause outages [Baldwin 2016; Graham-Cumming 2019; Stack Exchange 2016].
We have developed a new algebraic framework for regular expression matching based on derivatives that offers input-linear performance, has clear foundations, supports a large set of industrially
relevant features, and is compatible with backtracking semantics. While derivatives have been well
studied in the past [Ausaf et al. 2016; Fischer et al. 2010; Owens et al. 2009; Sulzmann and Lu
2012], how to unify them with anchors, counters, and backtracking (PCRE) semantics, has not.
What makes this combination especially challenging is that certain classical properties of regular
languages such as 𝐿(𝑅·𝑆) = 𝐿(𝑅)·𝐿(𝑆) no longer hold when anchors are present. Non-regular
features are excluded to enable finite-state based techniques. The work presented here extends
the open source SRM library [Saarikivi et al. 2019] and is integrated as a new backend in .NET’s
System.Text.RegularExpressions library. It is extensively tested, open-source, and ships with
.NET7.1 Since .NET is one of the main developer platforms worldwide2 we expect this new backend
to benefit many applications where predictable performance is critical.
SRM does not support anchors and, more importantly, lacks the foundations needed to do so;
does not support eager or lazy interpretation of loops or order of alternations, and is therefore not
compatible with the match results that in .NET otherwise rely on backtracking; lacks a general rewrite
system used to optimize regexes in a way that preserves their backtracking semantics.
These were serious hurdles to overcome. Our first attempt to treat anchors was as new imaginary
(0-width) symbols. This approach is also discussed in [Wingbrant 2019]. While the idea seems
promising, it does not work in .NET: e.g., the regexes \A\z and \z\A are equivalent, but a string
where an imaginary start-of-input is followed by an end-of-input is only accepted by the first regex,
or similarly with the regex \b$ that is equivalent to $\b because the order of a sequence of anchors
is immaterial in general. A key observation for supporting anchors is that matching is context
dependent. A stark example of this is that a pattern consisting of just the word-boundary anchor
\b has four empty matches at locations 0, 5, 6 and 11 in "Hello World", as shown by bold borders
in H e l l o W o r l d where exactly one character in the immediate vicinity of that location
must be a word-letter, e.g., location 11 is surrounded by "d" and "".3 The word-border anchor \b
occurs in almost all patterns in our case study of word phrase pattern matching.
Regarding backtracking semantics, consider the regex a+?|a* and the input "aa". If laziness of
loops and order of alternatives is ignored then the regex is classically equivalent to a* in which
case the earliest match end in "aa" is at location 0 and the latest match end is at location 2, while the
backtracking match end of a+?|a* in "aa" is at location 1. This illustrates that prioritized alternatives
cannot in general be reduced to earliest or latest match semantics in classical regular expressions
and is thus an orthogonal feature of regexes. The classically valid rewrite rule 𝑅{0, 𝑚}|𝑅{0, 𝑛} →
𝑅{0, max(𝑚, 𝑛)} from SRM does not preserve backtracking semantics because the loops are eager
(e.g., the match end of a{0,1}|a{0,2} in "aa" is at location 1 while the match end of a{0,2} in
"aa" is at location 2). Preserving the same semantics for all regular expression backends in .NET is
critical not only for consistent user experience but also for implementation transparency: runtime
optimizations such as substituting a backtracking search engine by a nonbacktracking search engine
in the absence of non-regular features could otherwise not be applied.4
Other popular nonbacktracking matching engines, such as RE2 [Google 2021] and grep [GNU
2020], can at a high level be considered efficient implementations of [Thompson 1968] enhanced
1 .NET7 shipped in November 2022.
2 For example consult https://enlyft.com/tech/products/microsoft-net for recent market analysis.
3 A location in a string is intuitively the position of a border between two characters or the position of the two outer borders.
4 Such substitutions are currently not being applied in the .NET7 release due to other concerns but remain possible.

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 3 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:3

with on-the-fly determinization and memoization, or in the case of Hyperscan [Intel Co 2021] as a
different variant of NFA simulation via [Glushkov 1961]. Such translation results in an NFA (either
via Thompson’s or Glushkov’s construction) where the states meaning as “regular expressions”
themselves has been lost in translation. Moreover, these engines do not have first-class support for
counting. Instead, counters are unwound up-front by a pre-processor, prior to the NFA translation.
Use of large counters in general is an Achilles heel of all state-of-the-art nonbacktracking regex
matchers as a recent study demonstrates [Turoňová et al. 2022]. Derivatives decrease counters by
one, and thus loops such as a{0,𝑛} end up being expanded fully only when the upper bound 𝑛
has been reached in the given input. Furthermore, in those engines both anchors and backtracking
semantics have been treated as implementation concerns. In contrast, we formally prove that our
derivative based matcher gives backtracking semantics in the presence of anchors.
Perhaps the most contrasting aspect of our algorithm is that the relationship between states and
regexes they denote is not lost in translation. We lift character based derivatives [Brzozowski 1964]
to location based derivatives, which allow for a natural treatment of anchors. For example, if the
regex for the current state 𝑞 is ^.*?(she|he) and the current input location 𝑥 has s as its current
character and \n as its previous character, then the 𝑥-derivative of 𝑞 is the regex he|.*?(she|he)
– the start-of-line anchor ^ was nullable (non-blocking) due to change of line and an alternation
has been created for the two ways the h may be consumed. The key benefit of maintaining the
link between states and regexes is used in the rewriting system we develop to minimize regexes
and the state machine their derivatives induce. In the example, if the next character is h then the
derivative would evolve to e|e|.*?(she|he), which rewriting would simplify to e|.*?(she|he).
Notably, we develop subsumption based rules for eliminating unnecessary alternation which we
found critical for acceptable real-world performance on some classes of patterns.
Finally, as far as we know, our algorithm is the first industrial implementation of input-linear regex
matching that has a formal proof of correctness. Moreover, it is continuously tested on the extensive
regex test suite in .NET to semantically match the other backends on all the platforms that .NET
supports. There is thus also compelling experimental evidence of mutual consistency of all the
backends relative to the fragment RE of regexes (defined in Section 3.1) and their backtracking
semantics (defined in Section 4.1). For detailed proofs of theorems see [Moseley et al. 2023a].
Summary of contributions:
New derivative based framework for regular expression matching with fully developed theory based
on location derivatives with Theorem 3.3 as its main characterization, and the reversal Theorem 3.8
as a key corollary. We also extend the theory with lookarounds. (Section 3)
Derivative based backtracking simulation building on a conservative extension of the core framework through a tail-recursive formulation of backtracking through derivatives. This results in a
new matching algorithm with proof of correctness in Theorem 4.5. (Section 4)
Industrial scale implementation involving a regex rewrite system with several key ideas of how
the framework is being utilized for advanced optimizations. (Section 5)
Comprehensive evaluation at industrial scale, validating the efficacy of this work. (Section 7)
2 PRELIMINARIES
Here we introduce background material used in the paper. As a general meta-notation throughout
the paper we write lhs def
= rhs to let lhs be equal by definition to rhs. Let B = {false, true} stand for
basic Boolean values. Let Σ be a universe or domain of characters. We denote pairs of elements by
⟨𝑥, 𝑦⟩ and let 𝜋1 (⟨𝑥, 𝑦⟩) def
= 𝑥 and 𝜋 2 (⟨𝑥, 𝑦⟩) def
= 𝑦.
Strings. Let 𝜖 or "" denote the empty string and let Σ∗ denote the set of all strings over Σ. Let
𝑠 ∈ Σ∗ . The length of 𝑠 is denoted by |𝑠 |. We do not distinguish between individual characters and
strings of length 1. Let 𝑖 and 𝑙 be nonnegative integers such that 𝑖 + 𝑙 ≤ |𝑠 |. Then 𝑠𝑖,𝑙 denotes the
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 4 -->

148:4

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

substring of 𝑠 that starts from index 𝑖 and has length 𝑙, where the first character has index 0. In
= 𝑠𝑖,1 . Let also 𝑠 −1 = 𝑠 |𝑠 | def
= 𝜖. E.g., "abcde"1,3 = "bcd" and
particular 𝑠𝑖,0 = 𝜖. For 0 ≤ 𝑖 < |𝑠 | let 𝑠𝑖 def
"abcde"5,0 = 𝜖. We denote the reverse of 𝑠 by 𝑠 𝑟, so that 𝑠 𝑟𝑖 = 𝑠 |𝑠 | −1−𝑖 for 0 ≤ 𝑖 < |𝑠 |.
Locations. Let 𝑠 be a string. A location in 𝑠 is a pair ⟨𝑠, 𝑖⟩, where −1 ≤ 𝑖 ≤ |𝑠 |. We use 𝑠 ⟨𝑖⟩ def
= ⟨𝑠, 𝑖⟩
as a dedicated notation for locations, where 𝑠 is the string and 𝑖 the position of the location. Since
𝑠 ⟨𝑖⟩ is a pair, note also that 𝜋 1 (𝑠 ⟨𝑖⟩) = 𝑠 and 𝜋2 (𝑠 ⟨𝑖⟩) = 𝑖. If 𝑥 and 𝑦 are locations, then 𝑥 < 𝑦 iff
𝜋2 (𝑥) < 𝜋2 (𝑦). A location 𝑠 ⟨𝑖⟩ is valid if 0 ≤ 𝑖 ≤ |𝑠 |. A location 𝑠 ⟨𝑖⟩ is called final if 𝑖 = |𝑠 | and
initial if 𝑖 = 0. Let Final(𝑠 ⟨𝑖⟩) def
= 𝑖 = |𝑠 | and Initial(𝑠 ⟨𝑖⟩) def
= 𝑖 = 0. We let def
= 𝜖 ⟨−1⟩ that is going to
be used to represent match failure and in general 𝑠 ⟨−1⟩ is used as a pre-initial location. The reverse
𝑠 ⟨𝑖⟩𝑟 of a valid location 𝑠 ⟨𝑖⟩ in 𝑠 is the valid location 𝑠 𝑟⟨|𝑠 |−𝑖⟩ in 𝑠 𝑟. For example, the reverse of the
final location in 𝑠 is the initial location in 𝑠 𝑟. When working with sets 𝑆 of locations over the same
string we let max(𝑆) (min(𝑆)) denote the maximum (minimum) location in the set according to the
location order above. In this context we also let max(∅) = min(∅) def
= and 𝑟 def
= . Valid locations
in a string 𝑠 are illustrated by
𝑠0 𝑠1
···
𝑠 |𝑠 |−1
↑
𝑠 ⟨0⟩

↑
𝑠 ⟨1⟩

↑
𝑠 ⟨2⟩

↑
↑
𝑠 ⟨|𝑠 |−1⟩ 𝑠 ⟨|𝑠 |⟩

and should be viewed as border positions rather than character positions.5 For example, 𝜖 has only
one valid location 𝜖 ⟨0⟩ that is both initial and final.
Boolean Algebras as Alphabet Theories. The tuple A = (Σ, Ψ, [[_]], ⊥, ⊤, ∨, ∧, ¬) is called a
Boolean algebra over Σ where Ψ is a set of predicates that is closed under the Boolean connectives;
[[_]] : Ψ → 2Σ is a denotation function; ⊥, ⊤ ∈ Ψ; [[⊥]] = ∅, [[⊤]] = Σ, and for all 𝜑,𝜓 ∈ Ψ,
[[𝜑 ∨ 𝜓 ]] = [[𝜑]] ∪ [[𝜓 ]] , [[𝜑 ∧ 𝜓 ]] = [[𝜑]] ∩ [[𝜓 ]] , and [[¬𝜑]] = Σ \ [[𝜑]] . Two predicates 𝜙 and 𝜓 are
equivalent when [[𝜙]] = [[𝜓 ]] , denoted by 𝜙 ≡ 𝜓 . If 𝜑 . ⊥ then 𝜑 is satisfiable or SAT(𝜑).
Character Classes. In all the examples below we let Σ stand for the standard 16-bit character
set of Unicode6 and use the .NET syntax [Microsoft 2021c] of regular expression character classes.
For example, [A-Z] stands for all the Latin capital letters, [0-9] for all the Latin numerals, \d for
all the decimal digits, \w for all the word-letters and dot (.) for all characters besides the newline
character \n. (It is a standard convention that, by default, dot does not match \n.)
When we need to distinguish the concrete representation of character classes from the corresponding abstract representation of predicates in A we map each character class 𝐶 to the corresponding
predicate 𝜓𝐶 in Ψ. For example 𝜓 [^0-9] ≡ ¬𝜓 [0-9] , 𝜓 [\w-[\d]] ≡ 𝜓 \w ∧ ¬𝜓 \d , and 𝜓 \W ≡ ¬𝜓 \w . Observe
also that [[𝜓 [0-9] ]] ⊊ [[𝜓 \d ]] ⊊ [[𝜓 \w ]] and [[𝜓 \n ]] = {\n} and \n ∉ [[𝜓 \w ]] because \n is not a
word-letter. Regarding ⊥ and ⊤ it holds e.g., that ⊥ ≡ 𝜓 [0-[0]] and ⊤ ≡ 𝜓 [\0-\uFFFF] .
3 REGEXES AND LOCATION DERIVATIVES
Here we formally define regular expressions with anchors and loops supporting finite and infinite
bounds as well as lazy and eager interpretations. Regexes are defined modulo a character theory
A = (Σ, Ψ, [[_]], ⊥, ⊤, ∨, ∧, ¬) that we illustrate with standard (.NET Regex) character classes in
examples, but it is important to keep in mind that A itself is abstract and later, in Section 5, used in
two distinct forms, both of which are independent of the concrete syntax of character classes.
After the definition of regexes, we formally develop a framework of derivatives that leads to the
key notion of derivation relation between locations that is instrumental in reasoning and proving
properties in this framework. We also define reversal of regexes and prove the main reversal
theorem that is later used in Section 4 to prove correctness of the complete matching algorithm.
5 This intuition fits well with the semantics of anchors and matching, and is also helpful in maintaining symmetry between

locations and reversed locations.
6 Also known as Plane 0 or the Basic Multilingual Plane of Unicode.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 5 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:5

The framework developed in this section does not depend on laziness of loops or the order of
alternatives in an alternation, but is extended conservatively to take order in account in Section 4
where backtracking semantics is formally defined. We also make a remark about how the framework
can be extended with lookarounds, demonstrating its flexibility and generality.
3.1

Regexes

The class RE of regular expressions or regexes as used in this paper is defined by the following
abstract grammar. Let 𝜓 ∈ Ψ, and 0 ≤ 𝑚 ≤ 𝑛 ≠ 0, or 𝑛 = ∞, and 𝑏 ∈ B:
RE ::=

ï

| 𝜓

::=

\A

|

ï

\z

|
|

()

|

RE·RE

|

RE|RE

|

RE{𝑚, 𝑛, 𝑏}

^

|

$

|

\Z

|

|

\b

\a

|

\B

Elements of ï are called anchors and have the names: start (\A), end (\z), start-of-line (^), end-of-line
($), final-end-of-line (\Z), initial-start-of-line (\a), word-border (\b), non-word-border (\B).
The regex denoting nothing (the empty language) has a simple representation in RE as just the
predicate ⊥, so no dedicated syntax is needed.
Concatenation operator · is often implicit by using juxtaposition, and the empty sequence ()
is a unit element of concatenation, so that ()·𝑅 = 𝑅·() = 𝑅. As is common, concatenation binds
stronger than alternation. Both concatenation and alternation are right associative operators.
A loop 𝑅{𝑚, 𝑛, 𝑏} has body 𝑅 and is called lazy if 𝑏 = true else eager. Its lower bound is 𝑚 and
upper bound is 𝑛. The loop is infinite when 𝑛 = ∞, finite otherwise. While the upper bound in a
finite loop must be nonzero, we let 𝑅{0, 0, _} def
= () for convenience in recursive definitions.
We abbreviate an eager loop by 𝑅{𝑚, 𝑛} and a lazy loop by 𝑅{𝑚, 𝑛}? and we also use the standard
shorthands 𝑅* for 𝑅{0, ∞} and 𝑅+ for 𝑅{1, ∞}, with 𝑅*? and 𝑅+? as their lazy versions. A loop
𝑅{𝑚, 𝑚, _} is also denoted by 𝑅{𝑚}. A finite eager loop 𝑅{𝑚, 𝑛} can be written equivalently as
𝑅{𝑚}·(𝑅|()){𝑛 − 𝑚}, a finite lazy loop 𝑅{𝑚, 𝑛}? is equivalent to 𝑅{𝑚}·(()|𝑅){𝑛 − 𝑚}, an infinite
eager loop 𝑅{𝑚, ∞} is equivalent to 𝑅{𝑚}·𝑅*, and an infinite lazy loop 𝑅{𝑚, ∞}? is equivalent to
𝑅{𝑚}·𝑅*?. These are useful simplifying normal forms when reasoning about properties of loops.
3.2 Nullability and Anchor-Contexts
The language semantics of regexes is in general context dependent. An important factor in defining
the semantics is played by the immediately surrounding symbols of a matching substring of an input
𝑠 being searched. Let 𝑥 be a valid location. A regex being nullable in 𝑥 means that it matches the
empty string, that in general is context dependent. The anchor-context of 𝑥 is 𝑥b def
= ⟨𝜿 (𝑥−1), 𝜿 (𝑥)⟩
where 𝜿 (𝑦) ∈ KIND def
= {𝜺, n, N, o, w}. Intuitively KIND is an enum with 𝜺 = EOF, n = EOL, N =
last-EOL, w = word-letter, o = other-character, and 𝜿 (𝑦) is the kind of location 𝑦.

𝜺 if 𝑖 = −1 or 𝑖 = |𝑠 |

𝑥b = ⟨𝜿 (𝑥 −1),𝜿 (𝑥 ) ⟩



N else if 𝑠𝑖 = \n and (𝑖 = 0 or 𝑖 = |𝑠 | − 1)

© z }| { ª


®
n else if 𝑠𝑖 = \n
𝜿 (𝑠 ⟨𝑖⟩) def
=
®
­ 𝑠𝑖 −1 𝑠𝑖

®

↑
w else if 𝑠𝑖 ∈ [[𝜓 \w ]]


𝑥

¬
«
 o otherwise.

Intuitively 𝑥b describes the kinds of the immediately surrounding symbols of 𝑥. Nullability of an
anchor is now defined relative to 𝑥b. Let Null𝑥 (𝐴) def
= Null𝑥b (𝐴) for 𝐴 ∈ ï.
Null ⟨𝜅1,𝜅2 ⟩ (\A)
Null ⟨𝜅1,𝜅2 ⟩ (\a)
Null ⟨𝜅1,𝜅2 ⟩ (^)
Null ⟨𝜅1,𝜅2 ⟩ (\b)

def

=
=
def
=
def
=

def

𝜅1 = 𝜺
𝜅 1 ∈ {𝜺, N}
𝜅 1 ∈ {𝜺, N, n}
𝜅1 = w ⇔ 𝜅2 ≠ w

Null ⟨𝜅1,𝜅2 ⟩ (\z)
Null ⟨𝜅1,𝜅2 ⟩ (\Z)
Null ⟨𝜅1,𝜅2 ⟩ ($)
Null ⟨𝜅1,𝜅2 ⟩ (\B)

def

=
=
def
=
def
=

def

𝜅2 = 𝜺
𝜅 2 ∈ {𝜺, N}
𝜅 2 ∈ {𝜺, N, n}
𝜅1 = w ⇔ 𝜅2 = w

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 6 -->

148:6

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

All the remaining cases extend the classical notion of nullability conservatively:
Null𝑥 (𝜓 )
Null𝑥 (𝑅 | 𝑆)
Null𝑥 (𝑅{𝑚, 𝑛, _})

def

= false
= Null𝑥 (𝑅) or Null𝑥 (𝑆)
def
= 𝑚 = 0 or Null𝑥 (𝑅)
def

Null𝑥 (())
Null𝑥 (𝑅 · 𝑆)

def

=
=

def

true
Null𝑥 (𝑅) and Null𝑥 (𝑆)

Let Null ∀ (𝑅) def
= ∀𝑥 (Null𝑥 (𝑅) = true) and Null  (𝑅) def
= 𝑥 (Null𝑥 (𝑅) = true). For example, the loop
.* is always nullable, i.e., Null ∀ (.*) = true while the concatenation .*b is never nullable, i.e.,
Null  (.*b) = true. Both properties are statically determined from the regex, essentially based on
that nullability does not depend on any anchor. We consider some cases in Example 3.1.
Example 3.1. Consider the regex ^$ (that matches an empty line) and the string 𝑠 = "IT\n\nIS"
and find some valid location 𝑥 in 𝑠 s.t. Null𝑥 (^$) = true. Then such a location is 𝑠 ⟨3⟩ as marked
bold in I T \n \n I S – no other location both ends and starts a line. Consider now the regex \b
and the same question. Then all such locations in 𝑠 would be where exactly one of the surrounding
character kinds is w, as marked bold in I T \n \n I S (locations 𝑠 ⟨0⟩, 𝑠 ⟨2⟩, 𝑠 ⟨4⟩, and 𝑠 ⟨6⟩).
⊠
3.3

Derivatives and Match End Location

In contrast to a classical definition of a complete string being accepted or matched by a regex, here
the matching of a substring 𝑠𝑖,𝑛 of 𝑠 depends on its surrounding locations 𝑠 ⟨𝑖⟩ and 𝑠 ⟨𝑖 +𝑛⟩ in 𝑠. We lift
the classical definition of the derivative [Brzozowski 1964] 𝐷𝑎 (𝑅) for a character 𝑎 to the derivative
Der𝑥 (𝑅) for a valid nonfinal location 𝑥. This extension is conservative so that when 𝑅 does not use
anchors then the acceptance condition of a string 𝑠 is classically preserved (see Theorem 3.4).
In the following definition let 𝑥 = 𝑠 ⟨𝑖⟩ be a valid nonfinal location. Note in particular that 𝑠𝑖 ∈ Σ.
For example, if 𝑠 = "abc" then such locations in 𝑠 are 𝑠 ⟨0⟩, 𝑠 ⟨1⟩, and 𝑠 ⟨2⟩.
Der𝑥 (𝑅)
Der𝑠 ⟨𝑖 ⟩ (𝜓 )
Der𝑥 (𝑅|𝑆)
Der𝑥 (𝑅·𝑆)
Der𝑥 (𝑅{𝑚, 𝑛, 𝑙})

def

= ⊥ if 𝑅 ∈ ï or 𝑅 = ()
= if 𝑠𝑖 ∈ [[𝜓 ]] then () else ⊥
def
= Der𝑥 (𝑅) | Der𝑥 (𝑆)
def
= if
Der𝑥 (𝑅)·𝑆|Der𝑥 (𝑆) else Der𝑥 (𝑅)·𝑆
 Null𝑥 (𝑅) then


Der𝑥 (𝑅)·𝑅{𝑚 −1,
𝑛 −1,
𝑙}, if 𝑚=0 or Null ∀ (𝑅)=true or Null𝑥 (𝑅)=false;
def
=


Der𝑥 (𝑅·𝑅{𝑚 −1, 𝑛 −1, 𝑙}), otherwise.

def

def
def
def



where ∞−1
= ∞, 0−1
= 0, 𝑘 −1
= 𝑘 − 1 for 𝑘 > 0. Note the special case for standard infinite loops:
Der𝑥 (𝑅*) = Der𝑥 (𝑅)·𝑅*. Recall also that 𝑅{0, 0, _} def
= () and 𝑅·() = 𝑅, and so Der𝑥 (𝑅{0, 1, _}) =


Der𝑥 (𝑅{1, 1, _}) = Der𝑥 (𝑅). We let also 𝑅{𝑚, 𝑛, 𝑙} − 1 def
= 𝑅{𝑚 −1,
𝑛 −1,
𝑙}.
Consider 𝑅 = .*b and 𝑠 = "abba". Then Der𝑠 ⟨1⟩ (𝑅) = Der𝑠 ⟨1⟩ (.*)b|Der𝑠 ⟨1⟩ (b) = R|() by using
the rules for concatenations, predicates (where 𝑠 1 ∈ [[𝜓 b ]] = {b} and 𝑠 1 ∈ [[𝜓 . ]] ), and loops.
We are now ready to define what it means to find a match end location from a valid start location
𝑥 in a string 𝑠 by a regex 𝑅 ∈ RE. MatchEnd(𝑥, 𝑅) returns the latest match end location from a valid
𝑥 or if none exists. Note that max(𝑥, ) = max( , 𝑥) = 𝑥.

Null𝑥 (𝑅)
MatchEnd(𝑥, 𝑅)
IsMatch(𝑥, 𝑅)

def

= if Null𝑥 (𝑅) then 𝑥 else
= if Final(𝑥) then Null𝑥 (𝑅) else max(Null𝑥 (𝑅), MatchEnd(𝑥+1, Der𝑥 (𝑅)))
def
= MatchEnd(𝑥, 𝑅) ≠
def

The definition of MatchEnd(𝑥, 𝑅) with 𝑥 = 𝑠 ⟨𝑖⟩ computes the transition from the source state
(regex) 𝑅 to the target state 𝑆 = Der𝑥 (𝑅) for the character 𝑠𝑖 and then continues matching from
location 𝑥 + 1 and state 𝑆. The existence of a match, i.e., IsMatch(𝑥, 𝑅), is independent of backtracking
semantics. The additional notions required for MatchEnd(𝑥, 𝑅) to respect backtracking semantics
are discussed in Section 4 where pruning of regexes is introduced that primarily affects the definition
of Der𝑥 (𝑅·𝑆), while the top-level definition of MatchEnd(𝑥, 𝑅) as stated above remains unchanged.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 7 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:7

In the case IsMatch(𝑥, 𝑅) = true, 𝑥 need not be the final location in 𝑠. If 𝑥 is nonfinal the recursive
call continues from the next location with the 𝑥-derivative. The definition accurately reflects the
semantics of the actual implementation.7 We illustrate IsMatch in Example 3.2.
Example 3.2. Consider the regex \A.+$ that matches the first line of the input, and that line must
be nonempty, where dot denotes Σ \ {\n}. Let 𝑠 = "I\nAm". Then Null𝑠 ⟨0⟩ (\A) = true. So
IsMatch(𝑠 ⟨0⟩, \A.+$) ⇔ IsMatch(𝑠 ⟨1⟩, Der𝑠 ⟨0⟩ (\A.+$))
⇔ IsMatch(𝑠 ⟨1⟩, Der𝑠 ⟨0⟩ (.+$)|Der𝑠 ⟨0⟩ (\A). + $)
⇔ IsMatch(𝑠 ⟨1⟩, Der𝑠 ⟨0⟩ (.+$)) ⇔ IsMatch(𝑠 ⟨1⟩, Der𝑠 ⟨0⟩ (.).*$) ⇔ IsMatch(𝑠 ⟨1⟩, .*$)
where in the last step Der𝑠 ⟨0⟩ (.) = () because 𝑠 0 ≠ \n. Since Null𝑠 ⟨1⟩ (.*$) = true it follows that
IsMatch(𝑠 ⟨1⟩, .*$) = true. Note also that ⊥·𝑅 → ⊥, 𝑅|⊥ → 𝑅 and ()·𝑅 → 𝑅 are always applied as
immediate simplifications (rewrites) when regexes are constructed.
⊠
3.4

Properties of Derivatives

Here we introduce some fundamental properties of derivatives that are later needed in proving
𝑅 𝑦 between
correctness theorems of matching. The key concept is the derivation relation 𝑥 −→
locations, also denoted by ⟨𝑥, 𝑦⟩ |= 𝑅, that is instrumental in reasoning about properties. Let the
universe of all matches be U def
= {⟨𝑠 ⟨𝑖⟩, 𝑠 ⟨𝑗⟩⟩ | 𝑠 ∈ Σ∗, 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑠 |}. Then
Null𝑥∅ (𝑅)
AllMatchEnds(𝑥, 𝑅)
𝑅 𝑦
⟨𝑥, 𝑦⟩ |= 𝑅 def
= 𝑥 −→
M (𝑅)

def

= if Null𝑥 (𝑅) then {𝑥 } else ∅
= Null𝑥∅ (𝑅) ∪ if Final(𝑥) then ∅ else AllMatchEnds(𝑥+1, Der𝑥 (𝑅))
def
= 𝑦 ∈ AllMatchEnds(𝑥, 𝑅)
def
= {𝑀 ∈ U | 𝑀 |= 𝑅}

def

We say that 𝑀 ∈ U is a match of 𝑅 if 𝑀 |= 𝑅, and M (𝑅) is called the match language of 𝑅. For
example, M (⊥) = ∅ and M (⊤*) = U. Two regexes 𝑅 and 𝑆 are equivalent, denoted by 𝑅 ≡ 𝑆, if
M (𝑅) = M (𝑆). Note that MatchEnd(𝑥, 𝑅) = max(AllMatchEnds(𝑥, 𝑅)) follows directly.
Observe the invariant that if 𝑥 −𝑅→ 𝑦 then 𝜋 1 (𝑥) = 𝜋1 (𝑦), and 𝜋 2 (𝑥) ≤ 𝜋2 (𝑦), i.e., the string of the
locations remains fixed and only the distance 𝜋2 (𝑦) − 𝜋 2 (𝑥) between them can increase. Note also
that if 𝑥 = 𝑠 ⟨𝑖⟩ is valid and nonfinal then 𝑥 + 1 def
= 𝑠 ⟨𝑖 + 1⟩ is valid.
The following is the main derivation theorem. The proofs of (3) and (4) are by induction over
𝜋2 (𝑦) − 𝜋2 (𝑥), (5) uses (3,4), and (6) uses (3–5).
Theorem 3.3 (Derivation). For all regexes and valid locations, let 𝐴 ∈ ï ∪ {()} and 𝜓 ∈ Ψ:
(1) 𝑥 −𝐴→ 𝑦 ⇔ Null𝑥 (𝐴) and 𝑥 = 𝑦;
(2) 𝑠 ⟨𝑖⟩ −𝜓→ 𝑦 ⇔ 𝑠𝑖 ∈ [[𝜓 ]] and 𝑦 = 𝑠 ⟨𝑖 + 1⟩;
𝑆 𝑦 ⇔ (𝑥 𝑅 𝑦 or 𝑥 𝑆 𝑦);
(3) 𝑥 −𝑅−−|→
−→
−
→
𝑅·𝑆
𝑅 𝑧 𝑆 𝑦);
(4) 𝑥 −−−→ 𝑦 ⇔ ∃𝑧 (𝑥 −→
−
→
(5) ∀𝑚 > 0 : 𝑅{𝑚, 𝑛} ≡ 𝑅·𝑅{𝑚 − 1, 𝑛 − 1} ≡ 𝑅{𝑚 − 1, 𝑛 − 1}·𝑅;
(6) 𝑅{0, 𝑛} ≡ 𝑅{1, 𝑛}|();
A notable special case is ⊥* ≡ () because ⊥* ≡⊥·⊥*|() ≡ () where 𝑧 (𝑥 −⊥→ 𝑧) since [[⊥]] = ∅.
3.5

Relation to Classical Regular Expressions and Derivatives

Recall the classical definition of the language L (𝑅) ⊆ Σ∗ of 𝑅 without anchors: L (()) = {𝜖},
L (𝜓 ) = [[𝜓 ]] , L (𝐿·𝑅) = L (𝐿)·L (𝑅), L (𝐿|𝑅) = L (𝐿)∪L (𝑅), and L (𝑅*) = L (𝑅) ∗ that has the
generalization L (𝑅{𝑚}) = L (𝑅)·L (𝑅{𝑚 − 1}) to finite loops.
7 Many important optimizations are omitted here, such as MatchEnd(_, ⊥)
def

def

=

, MatchEnd(𝑠 ⟨_⟩, ⊤*)

def

= 𝑠 ⟨ |𝑠 | ⟩,

def

MatchEnd(𝑥, ()) = 𝑥, and MatchEnd(𝑥, 𝐴) = Null𝑥 (𝐴) for 𝐴 ∈ ï.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 8 -->

148:8

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

Theorem 3.4. If 𝑅 ∈ RE contains no anchors then 𝑠 ∈ L (𝑅) ⇔ ⟨𝑠 ⟨0⟩, 𝑠 ⟨|𝑠 |⟩⟩ ∈ M (𝑅).
Proof. If 𝑅 is classical then Der𝑠 ⟨𝑖 ⟩ (𝑅) = 𝐷𝑠𝑖 (𝑅) where 𝐷𝑎 (𝑅) is essentially the Brzozowski
derivative of 𝑅 for 𝑎 ∈ Σ. The statement then follows from [Brzozowski 1964] (provided that each
predicate 𝜓 in 𝑅 is viewed equivalently as an alternation over all characters in [[𝜓 ]] ).
□
We are more focused here on the derivation relation rather than languages. Note that most
classical properties fail for languages of 𝑅 ∈ RE when anchors are present. For example, if 𝑅 = a\b
then M (𝑅) = {⟨𝑠 ⟨𝑖⟩, 𝑠 ⟨𝑖+1⟩⟩ | 𝑠𝑖 = a, 𝑠𝑖+1 ∉ [[𝜓 \w ]] } but M (𝑅·𝑅) = ∅ because \b is infeasible
between two word-letters. While M (𝑅) is regular for all 𝑅 ∈ RE, because the derivation relation
induces a finite set of derivatives (see Section 5), even for IsMatch it would be very complicated
and for MatchEnd not possible, to convert 𝑅 ∈ RE into an equivalent regex without anchors.
Note on Loops. There is another crucial difference to the classical case of derivatives of loops.
Let 𝐿 = 𝑅{𝑚, 𝑛}. When 𝑅 contains no anchors, it holds that Der𝑥 (𝐿) = Der𝑥 (𝑅)·(𝐿 − 1) because
then either Null ∀ (𝑅) = true or else Null  (𝑅) = true – there is no “middle ground”. In contrast,
it would be incorrect to define Der𝑥 (𝐿) as Der𝑥 (𝑅)·(𝐿 − 1) when Null𝑥 (𝑅) = true and nullability
depends on anchors, as Example 3.5 illustrates, while definition of Der𝑥 (𝐿) as Der𝑥 (𝑅·(𝐿 − 1)) when
Null𝑥 (𝑅) = true and both 𝑚 = 0 and 𝑛 = ∞, would be circular through the rule for concatenation
and thus not well-defined. So the two cases of loop derivatives are crucial for correctness.
Example 3.5. Let 𝑅 = (a|\b) and 𝐿 = 𝑅{2} and 𝑥 = "abc"⟨0⟩. It is clear that 𝐿 is meant to be
equivalent to 𝑅·𝑅. Note that Der𝑥 (𝑅)·𝑅 = 𝑅, while Der𝑥 (𝑅·𝑅) = Der𝑥 (𝑅)·𝑅|Der𝑥 (𝑅) = 𝑅|() because
Null𝑥 (𝑅) = true. Therefore Null𝑥+1 (Der𝑥 (𝑅)·𝑅) = Null𝑥+1 (𝑅) = false because \b is not nullable in
𝑥 + 1 while Null𝑥+1 (Der𝑥 (𝑅·𝑅)) = Null𝑥+1 (𝑅|()) = true due to (). So Der𝑥 (𝑅)·𝑅 . Der𝑥 (𝑅·𝑅). ⊠
3.6

Reversal

Reversal of regexes is used in the complete matching algorithm in order to locate the beginning of a
match, where the original search pattern is used backwards from a previously found ending location.
Details of the complete matching procedure are in Section 4.5. Here our focus is on reversal itself.
The reverse 𝑅𝑟 of 𝑅 ∈ RE is defined as follows.
= $ \a𝑟 def
= \Z \b𝑟 def
= \b \z𝑟 def
= \A $𝑟 def
= ^ \Z𝑟 def
= \a \B𝑟 def
= \B
\A𝑟 def
= \z ^𝑟 def
def 𝑟 𝑟
def 𝑟
def
def 𝑟
𝑟 def
𝑟
𝑟
𝑟
𝑟
𝑟
𝜓 = 𝜓 () = () (𝑅|𝑆) = 𝑅 |𝑆 (𝑅·𝑆) = 𝑆 ·𝑅 𝑅{𝑚, 𝑛, 𝑏} = 𝑅 {𝑚, 𝑛, 𝑏}
Lemma 3.6 is proved by induction over RE. Example 3.7 illustrates an instance of Lemma 3.6.
Theorem 3.8 is proved by induction over location distances and uses Theorem 3.3 and Lemma 3.6.
Lemma 3.6. For all 𝑅 ∈ RE and valid locations 𝑥: Null𝑥 (𝑅) ⇔ Null𝑥 𝑟 (𝑅𝑟).
Example 3.7. Nullable anchor locations in the string "1-23\n" and its reverse:
1
2
3
\n
\n
3
2
1
reverse
↑
↑
↑
↑
↑
↑
↑
↑
↑
↑
↑
↑
⇐⇒

\b^\a\A

\b

\b

\B

\b$\Z \B$\Z\z

\B^\a\A \b^\a

\B

\b

\b

⊠

\b$\Z\z
𝑟

𝑅 𝑦 ⇔ 𝑦𝑟 𝑅 𝑥 𝑟.
Theorem 3.8 (Reversal). For all 𝑅 ∈ RE and valid locations 𝑥 and 𝑦: 𝑥 −→
−−→

3.7

Lookarounds

Lookarounds are expressions in the form of lookaheads (?=𝑅) and (?!𝑅), and lookbacks (?<=𝑅) and
(?<!𝑅), where 𝑅 is a regex. Lookarounds are currently not implemented in the nonbacktracking
engine but can very transparently be supported as follows. Let 𝑥 be a valid location.
Null𝑥 ((?=𝑅)) def
= IsMatch(𝑥, 𝑅)
Null𝑥 ((?!𝑅)) def
= not IsMatch(𝑥, 𝑅)
def
Null𝑥 ((?<=𝑅)) = IsMatch(𝑥 𝑟, 𝑅𝑟)
Null𝑥 ((?<!𝑅)) def
= not IsMatch(𝑥 𝑟, 𝑅𝑟)
(?=𝑅)𝑟 def
= (?<=𝑅𝑟) (?<=𝑅)𝑟 def
= (?=𝑅𝑟) (?!𝑅)𝑟 def
= (?<!𝑅𝑟) (?<!𝑅)𝑟 def
= (?!𝑅𝑟).
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 9 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:9

If ℓ is a lookaround and 𝑥 a valid nonfinal location then Der𝑥 (ℓ) def
= ⊥. In other words, lookarounds
are like anchors but use the full context of 𝑥.8 Observe that lookarounds are a proper generalization
of anchors, we could have started with RE including lookarounds and omitted anchors, in which
case all of the anchors can be defined in terms of lookarounds. E.g., observe that \A ≡ (?<!⊤):
Null𝑥 ((?<!⊤)) ⇔ not IsMatch(𝑥 𝑟, ⊤) ⇔ (Final(𝑥 𝑟) or not IsMatch(𝑥 𝑟+1, Der𝑥 𝑟 (⊤)))
⇔ (Final(𝑥 𝑟) or not IsMatch(𝑥 𝑟+1, ())) ⇔ (Final(𝑥 𝑟) or not true) ⇔ Initial(𝑥)
Where 𝑥 𝑟 is final iff 𝑥 is initial. Similarly, \z ≡ (?!⊤). The word-border anchor \b has also an
elegant equivalent definition by using lookarounds \b ≡ (?<=𝜓 \w )·(?!𝜓 \w )|(?<!𝜓 \w )·(?=𝜓 \w ).
Similarly for \B. All the other anchors can be defined similarly, even using nested lookarounds.
For example (?=\n|\z) ≡ $ and (?<=\A|\A\n) ≡ \a. The downside of allowing unrestricted use
of lookarounds would be that input-linear complexity of matching (Theorem 5.3) would not hold
because nullability tests would no longer be independent of the length of the input.
4 MATCHING WITH BACKTRACKING SEMANTICS WITHOUT BACKTRACKING
Here we introduce the top-level matching algorithm that preserves backtracking semantics. First
we formally describe what we mean by backtracking based regex matching. We then introduce
two key techniques: high-nullability and pruning that are fundamental in testing and restricting
derivatives in such a way that backtracking semantics is preserved. We prove that the tail-recursive
procedure MatchEnd works dually by simulating backtracking. Finally, we present the top-level
algorithm Match that uses MatchEnd in two phases: a forward phase that simulates backtracking by
pruning of derivatives to find an end location for a match, and a backwards phase that reverses the
given search pattern to find the start location backwards from the previously found end location
in nonbacktracking mode. All the theory presented in Section 3 is utilized here fully and formal
correctness theorems are stated and proved for all the key statements.
4.1

Backtracking Based Regex Match End Search Semantics

We describe an abstract recursive backtracking based match end search procedure semantics ⌊𝑅⌋ 𝑥
for a regex 𝑅 from a start location 𝑥 in a string 𝑠 by a recursive search through ⌈𝑅⌉ 𝑥 that returns the
list of all end locations in order of backtracking priority, where ⊕ appends lists. Let first ([]) def
=
and let first (ℓ) denote the first element of a nonempty list ℓ. In the following let 𝐴 ∈ ï, 𝜓 ∈ Ψ, 𝑠 ∈ Σ∗ ,
𝑥 = 𝑠 ⟨𝑖⟩ be a nonfinal location in 𝑠 (where 0 ≤ 𝑖 < |𝑠 |), and 𝑦 = 𝑠 ⟨|𝑠 |⟩ be the final location in 𝑠.
⌊𝑅⌋ 𝑥
⌈()⌉ 𝑥
⌈(𝑋 |𝑌 )·𝑍 ⌉ 𝑥

def

= first (⌈𝑅⌉ 𝑥 )
= [𝑥]
def
= ⌈𝑋 ·𝑍 ⌉ 𝑥 ⊕ ⌈𝑌 ·𝑍 ⌉ 𝑥

def

⌈𝐴·𝑍 ⌉ 𝑥
⌈𝜓 ·𝑍 ⌉ 𝑥
⌈𝑋 {𝑚}·𝑍 ⌉ 𝑥
⌈𝑅⌉ 𝑦

def

= if Null𝑥 (𝐴) then ⌈𝑍 ⌉ 𝑥 else []
= if 𝑠𝑖 ∈ [[𝜓 ]] then ⌈𝑍 ⌉ 𝑥+1 else []
def
= ⌈𝑋 ·𝑋 {𝑚−1}·𝑍 ⌉ 𝑥
def
= if Null 𝑦 (𝑅) then [𝑦] else []
def

All concatenations are in right associative form and where 𝑍 may be () for the case when the
regex is not a concatenation. We focus on finite loops only by interpreting ∞ as |𝑠 |+1 here. This
assumption preserves the semantics but avoids infinite recursion of ⌈𝑅*⌉ 𝑥 . We can therefore use the
normal form of loops with a single counter (recall Section 3.1).9 Below we extend the definition of
derivatives accordingly to capture the semantics of backtracking. Intuitively, derivatives store the
backtracking choice points in the regex itself by utilizing the order of alternatives where priority is
always given to the first alternative of an alternation.
8 Observe that, even if 𝑅 itself would contain lookarounds then all the involved definitions would still remain mutually
well-defined (by induction) because 𝑅 is a smaller expression than the lookaround containing it.
9We are not concerned here with how backtracking is implemented, where highly optimized data structures are being used.

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 10 -->

148:10

4.2

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

Prioritized Nullability

The central case is dealing with a concatenation 𝐿·𝑅 when 𝐿 is nullable, that typically results in an
alternation being created. In this case, if 𝐿 is high-priority nullable or high-nullable then skipping 𝐿
and continuing to match 𝑅 takes priority. Formally, for 𝑅 ∈ RE and valid locations 𝑥:
Null!𝑥 (𝑅)
Null!𝑥 (𝐿 | 𝑅)
Null!𝑥 (𝐿 · 𝑅)
Null!𝑥 (𝑅{𝑚, 𝑛, lazy})

def

= Null𝑥 (𝑅) if 𝑅 ∈ ï or 𝑅 ∈ Ψ or 𝑅 = ()
= Null!𝑥 (𝐿)
def
= Null!𝑥 (𝐿) and Null!𝑥 (𝑅)
def
= (lazy and 𝑚 = 0) or Null!𝑥 (𝑅)
def

The intuition behind high-nullability of loops is that in a lazy loop the intent is to exit the loop
as early as possible, while in an eager loop the intent is to exit the loop as late as possible. A

lazy loop 𝑅{0, 𝑛}? is equivalent to ()|𝑅·𝑅{0, 𝑛 −1}?
while an eager loop 𝑅{0, 𝑛} is equivalent to

𝑅·𝑅{0, 𝑛 −1}|(). The rule for loops now follows from that of alternation and concatenation.10
Example 4.1. The regex (abc|()) is nullable but not high-nullable because () comes second,
while (()|abc) is nullable as well as high-nullable because () comes first. The regex $|.* is nullable
but not high-nullable in "a\na"⟨0⟩ but it is high-nullable in "a\na"⟨1⟩.
⊠
The computation of derivatives is now updated as follows, by taking high-nullability into account.

if not Null𝑥 (𝐿);

 Der𝑥 (𝐿)·𝑅,

def
Der𝑥 (𝑅) | Der𝑥 (𝐿)·𝑅, else if Null!𝑥 (𝐿);
Der𝑥 (𝐿·𝑅) =

 Der𝑥 (𝐿)·𝑅 | Der𝑥 (𝑅), otherwise;


For example, Der"abba"⟨1⟩ (.*b) = .*b|() but Der"abba"⟨1⟩ (.*?b) = ()|.*?b.
4.3

Pruning

Pruning of a regex 𝑅 in a valid location 𝑥, denoted by Prune𝑥 (𝑅), removes those branches of 𝑅 that
are not used in backtracking in order to preserve backtracking semantics of the resulting derivatives.
Intuitively, pruning mimics how backtracking chooses a path.

, if Null𝑥 (𝑅) = false;

𝑅

def
()
, else if Null!𝑥 (𝑅) = true;
Prune𝑥 (𝑅) =

 Prune𝑥 (𝑍 ), else if 𝑅 = 𝑋 ·𝑍 and Null!𝑥 (𝑋 ) = true;


otherwise Null𝑥 (𝑅) = true and Null!𝑥 (𝑅) = false and if 𝑅 = 𝑋 ·𝑍 then Null!𝑥 (𝑋 ) = false – observe
that if 𝑋 is a loop with body 𝐵 then Null!𝑥 (𝐵) = false. We proceed by case analysis over 𝑅. We
focus on the key case of the normal form of loops using a single counter. Observe that the case
𝑅 = 𝑋 *?·𝑍 is not possible below because 𝑋 *? is high-nullable.
Prune𝑥 ((𝑋 ·𝑌 )·𝑍 )

def

=

Prune𝑥 (𝑋 ·(𝑌 ·𝑍 ))

Prune𝑥 (𝑋 |𝑌 )
Prune𝑥 ((𝑋 |𝑌 )·𝑍 )

def

=
=

if Null𝑥 (𝑋 ) then Prune𝑥 (𝑋 ) else 𝑋 | Prune𝑥 (𝑌 )
Prune𝑥 (𝑋 ·𝑍 | 𝑌 ·𝑍 )

Prune𝑥 (𝑋 {𝑚}·𝑍 )

def

=

Prune𝑥 (𝑋 ·𝑋 {𝑚−1}·𝑍 )

Prune𝑥 (𝑋 *·𝑍 )

def

Prune𝑥 (𝑋 )·𝑋 *·𝑍 | Prune𝑥 (𝑍 )

def

=

The main case is Prune𝑥 (𝑋 |𝑌 ) that prioritizes 𝑋 if a solution exists (𝑌 is forgotten). Concatenation
(𝑋 |𝑌 )𝑍 needs a special case of being treated as 𝑋𝑍 |𝑌 𝑍 in order to preserve 𝑋𝑍 as the first
alternative in case 𝑋𝑍 is not nullable and 𝑌 𝑍 ends up being pruned, essentially the alternation
must be propagated to the top level prior to pruning. Certain optimizations (shortcuts) are also
10 Typically the body of a loop is not nullable, in which case the loop is high-nullable iff its lower bound is 0 and it is lazy.

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 11 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:11

omitted for the case of Null ∀ (𝑅) = true in which case the counter need not be unfolded or when
the loop body of an eager loop is never nullable. For example, a{0,5}?c* is pruned to cc*|() that
is kept as c* because c is never nullable. A related optimization is that Prune𝑥 (𝑋 {0, 𝑛}) def
= 𝑋 {0, 𝑛}
if 𝑋 is never nullable.
As Example 4.1 illustrated, 𝑅 may be nullable in one location while not high-nullable in that
location, but high-nullable in another location. So pruning is in general location dependent because
of anchors. Pruning preserves backtracking semantics. It cuts off all alternatives that are not going
to be used in backtracking. We will use the following lemma that relates backtracking to derivatives,
and is proved by induction over 𝑅, using the definitions of pruning and high-nullability.
def

Lemma 4.2. For all valid locations 𝑥 and 𝑅 ∈ RE, let ⌊𝑅⌋ 𝑥+1 = if 𝑥 is final,
(1) not Null𝑥 (𝑅) ⇒ (⌊𝑅⌋ 𝑥 = ⌊Der𝑥 (𝑅)⌋ 𝑥+1 and (⌊𝑅⌋ 𝑥 = ⇒ AllMatchEnds(𝑥, 𝑅) = ∅)).
(2) Null𝑥 (𝑅) ⇒ ⌊𝑅⌋ 𝑥 = max(𝑥, ⌊Der𝑥 (Prune𝑥 (𝑅))⌋ 𝑥+1 ) ∈ AllMatchEnds(𝑥, 𝑅).
Lemma 4.2(1) states that search may equivalently continue using the derivative from the next
location when the current one is not a solution. Lemma 4.2(2) states that pruning eliminates exactly
those alternatives of 𝑅 that correspond to the choices that backtracking would never make, and
the current location is the solution unless a later one exists. Lemma 4.2 paves a way to implement
backtracking in a tail-recursive manner by using derivatives, as formalized next.
4.4 Finding Match End Locations with Backtracking Semantics
We extend the abstract syntax of regexes with an internal top-level marker indicating that, during
derivation, the regex is to be interpreted in a mode that simulates backtracking: REbt ::= RE | BT(RE).
Derivatives and nullability are extended to REbt where 𝑥 is valid. We let BT(⊥) def
= ⊥. The marker
is retained in derivatives in order to maintain the backtracking simulation mode, where 𝑅 is now
pruned before taking its derivative. (Let 𝑥 be nonfinal in Der𝑥 (𝑅).)
Der𝑥 (BT(𝑅)) def
= BT(Der𝑥 (Prune𝑥 (𝑅)))

Null𝑥 (BT(𝑅)) def
= Null𝑥 (𝑅)

Observe that MatchEnd operates in backtracking simulation mode if the marker is present else
in nonbacktracking mode where regexes are not pruned. Note that catastrophic backtracking is
not possible here because pruning simulates backtracking semantics without actual backtracking.
Lemma 4.3 establishes key properties used later, where 4.3(3) is proved by induction over locations
distances using Lemma 4.2.
Lemma 4.3. For 𝑅 ∈ RE and valid locations 𝑥 and 𝑦:
𝑅 𝑦;
(1) MatchEnd(𝑥, BT(𝑅)) = 𝑦 ≠ ⇒ 𝑥 −→
(2) AllMatchEnds(𝑥, 𝑅) = ∅ ⇒ MatchEnd(𝑥, BT(𝑅)) = ;
(3) MatchEnd(𝑥, BT(𝑅)) = ⌊𝑅⌋ 𝑥 .
Example 4.4. Consider the regex BT(.*?b) that finds the location immediately after the first
occurrence of b in an input. Let 𝑠 = "abba". We show main steps. Let F stand for MatchEnd.
F(𝑠 ⟨0⟩, BT(.*?b)) = F(𝑠 ⟨1⟩, BT(.*?b)) = F(𝑠 ⟨2⟩, BT(Der𝑠 ⟨1⟩ (.*?b)))
= F(𝑠 ⟨2⟩, BT(()|.*?b)) = max(𝑠 ⟨2⟩, F(𝑠 ⟨3⟩, BT(Der𝑠 ⟨2⟩ (Prune(()|.*?b)))))
= max(𝑠 ⟨2⟩, F(𝑠 ⟨3⟩, BT(Der𝑠 ⟨2⟩ (())))) = max(𝑠 ⟨2⟩, F(𝑠 ⟨3⟩, ⊥)) = max(𝑠 ⟨2⟩, ) = 𝑠 ⟨2⟩
The 𝑠 ⟨1⟩-derivative of the concatenation .*?b prioritizes skipping the lazy loop as opposed to
staying in the loop. So pruning eliminates the second alternative of ()|.*?b. Now consider the
regex BT(.*b) that finds the location after the last occurrence of b in 𝑠. We get that
F(𝑠, BT(.*b)) = F(𝑠 ⟨2⟩, BT(.*b|())) = max(𝑠 ⟨2⟩, F(𝑠 ⟨3⟩, BT(.*b|())))
= max(𝑠 ⟨2⟩, max(𝑠 ⟨3⟩, F(𝑠 ⟨4⟩, BT(.*b)))) = max(𝑠 ⟨2⟩, max(𝑠 ⟨3⟩, )) = 𝑠 ⟨3⟩
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 12 -->

148:12

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

The derivative of the concatenation .*b prioritizes staying in the eager loop. Consequently, pruning
keeps both alternatives of .*b|(). Note that 𝑠 ⟨4⟩ is final and .*b is not nullable.
⊠
4.5

Complete Matching

We are now ready to present the complete matching algorithm that given a string 𝑠 and a regex
𝑅 𝑦 and 𝑦 is the
𝑅 ∈ RE finds the earliest start location 𝑥 in 𝑠 and the end location 𝑦 in 𝑠 such that 𝑥 −→
backtracking end location. A successful search result is the pair ⟨𝑥, 𝑦⟩ and represents failed search.
Match(𝑠, 𝑅) def
= if 𝑦 = MatchEnd(𝑠 ⟨0⟩, BT(⊤*?·𝑅)) ≠

then ⟨MatchEnd(𝑦𝑟, 𝑅𝑟)𝑟, 𝑦⟩ else

In the correctness proof we use symmetry of reversal that implies, by Theorem 3.8, that for any
𝑅 𝑦} = (max{𝑧 | 𝑦𝑟 𝑅𝑟 𝑧})𝑟. In other words, 𝑧 is the earliest
valid location 𝑦 in a string 𝑠, min{𝑧 | 𝑧 −→
−−→
𝑟
𝑟
location in 𝑠 such that 𝑧 −𝑅→ 𝑦 iff 𝑧 is the latest location in 𝑠 𝑟 such that 𝑦𝑟 −𝑅−→ 𝑧𝑟.
Theorem 4.5 (Correctness of Match). For all 𝑠 ∈ Σ∗ and 𝑅 ∈ RE:
𝑅 𝑠 ⟨𝑗⟩).
(1) Match(𝑠, 𝑅) = ⇒  𝑖 𝑗 (𝑠 ⟨𝑖⟩ −→
(2) Match(𝑠, 𝑅) = ⟨𝑠 ⟨𝑖⟩, 𝑠 ⟨𝑗⟩⟩ |= 𝑅 such that
𝑅 𝑧)}
(a) 𝑖 = min{𝑖 | ∃𝑧 (𝑠 ⟨𝑖⟩ −→
(b) 𝑠 ⟨𝑗⟩ = ⌊⊤*?·𝑅⌋ 𝑠 ⟨0⟩
Proof. Let 𝑦 = MatchEnd(𝑠 ⟨0⟩, BT(⊤*?·𝑅)).
*? ·𝑅 𝑤). By
No match exists: Assume 𝑦 = . By using Lemma 4.3(2) it follows that 𝑤 (𝑠 ⟨0⟩ −⊤−−−−
→
𝑅 𝑤)). But at the same time 𝑠 ⟨0⟩ ⊤*? 𝑧 for all valid
Theorem 3.3(4), we get that 𝑤 (∃𝑧 (𝑠 ⟨0⟩ −⊤−−*?→ 𝑧 −→
−−−→
𝑅 𝑦.
locations 𝑧 in 𝑠. It follows that there exist no locations 𝑥 and 𝑦 in 𝑠 such that 𝑥 −→
*? ·𝑅 𝑦
Match exists: Assume 𝑦 ≠ . Let F stand for MatchEnd. Then, by Lemma 4.3(1), 𝑠 ⟨0⟩ −⊤−−−−
→
⊤
*?
𝑅
𝑅
and by Theorem 3.3(4) there exists 𝑧 s.t. 𝑠 ⟨0⟩ −−−→ 𝑧 −→ 𝑦. Let 𝑧 min = min{𝑧 | 𝑧 −→ 𝑦}. So
𝑟
F(𝑦𝑟, 𝑅𝑟) = max{𝑧 | 𝑦𝑟 −𝑅−→ 𝑧} and by Theorem 3.8, 𝑧 min = F(𝑦𝑟, 𝑅𝑟)𝑟. So 4.5(1) and 4.5(2a) follow, and
4.5(2b) follows from Lemma 4.3(3).
□
Example 4.6. Let 𝑅 = he|the|cat and 𝑠 = "I see the cat". Let F = MatchEnd. Then11
F(𝑠 ⟨0⟩, BT(⊤*?·𝑅)) = F(𝑠 ⟨6⟩, BT(⊤*?·𝑅)) = F(𝑠 ⟨7⟩, BT(he|⊤*?·𝑅))
= F(𝑠 ⟨8⟩, BT(e|e|⊤*?·𝑅)) = F(𝑠 ⟨9⟩, BT(()|()|⊤*?·𝑅)) = max(𝑠 ⟨9⟩, F(𝑠 ⟨10⟩, ⊥)) = 𝑠 ⟨9⟩
where pruning removes all the other alternatives besides the first (), once the nullable location
𝑠 ⟨9⟩ has been found, and where ⊥ = Der𝑠 ⟨9⟩ (()). Now 𝑅𝑟 = eh|eht|tac and 𝑠 𝑟 = "tac eht ees I"
and 𝑠 ⟨9⟩𝑟 = 𝑠 𝑟⟨|𝑠 | − 9⟩ = 𝑠 𝑟⟨4⟩ where (𝑠 𝑟)4 = e. Then
F(𝑠 𝑟⟨4⟩, 𝑅𝑟) = F(𝑠 𝑟⟨5⟩, h|ht) = F(𝑠 𝑟⟨6⟩, ()|t) = max(𝑠 𝑟⟨6⟩, F(𝑠 𝑟⟨7⟩, ())) = max(𝑠 𝑟⟨6⟩, max(𝑠 𝑟⟨7⟩, ))
So the result is ⟨𝑠 ⟨6⟩, 𝑠 ⟨9⟩⟩, where 𝑠 ⟨6⟩ = 𝑠 𝑟⟨7⟩𝑟, with 𝑠 6,9−6 = "the" as the matched substring. ⊠
Observe that using BT(𝑅𝑟) instead of 𝑅𝑟 in the second pass would not work in general. In the
above example the search would stop too early due to pruning in location 𝑠 𝑟⟨6⟩ in 𝑠 𝑟 and therefore
not reach the earliest location 𝑠 ⟨6⟩ in 𝑠.
5 IMPLEMENTATION
Here we give a high level overview of how MatchEnd can be materialized into a practical implementation. The main concern is the cost of the calls to Der and Null. We first describe how alphabet
compression can be used to reduce the large alphabet size of Unicode. We then show how an effective
caching scheme can be produced using the compressed alphabet and the property that Null only
depends on anchor-contexts.
11 Duplicate later alternatives in an alternation are always removed from the alternation, here we include them for clarity.

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 13 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:13

Finally, we will address the most central concern for avoiding unnecessary cache misses: syntactically different regexes that are semantically equivalent. We describe a rewrite system that is used
to simplify regexes that arise during derivation. Crucially, the rewrite system guarantees finiteness
of the space of derivatives. In the following sections we consider a fixed search pattern 𝑅0 ∈ RE.
5.1

Alphabet Compression

The following steps are taken to construct a compressed alphabet algebra A that is tailor-made for
𝑅0 . Initially, 𝑅0 is traversed to extract the set Γ of all those predicates that 𝑅0 depends on, using a
binary decision diagram (BDD) based algebra B for Unicode. Predicates in Γ are BDDs.
Ó
Ô
Minterm Computation. A minterm of Γ is a predicate ( 𝑆) ∧ ¬ (Γ\𝑆) for some 𝑆 ⊆ Γ.
Minterms(Γ) denotes the set of all minterms of Γ, we use the algorithm from [D’Antoni and
Veanes 2014]. All minterms of Γ are satisfiable and mutually disjoint, and each satisfiable predicate
𝑘 −1
in Γ is equivalent to a disjunction of some of its minterms. Let Minterms(Γ) = 𝛽¯ = (𝛽𝑖 )𝑖=0
so that min([[𝛽𝑖 ]] ) < min([[𝛽𝑖+1 ]] ) by using the underlying fixed order of the characters (by
their numeric code points) in Σ. This provides a fixed choice of the minterm order which does
not matter for correctness but is important to avoid nondeterminism in this step. The Unicode
code point order is fixed across all platforms and runtimes and independent of runtime culture
(System.Globalization.CultureInfo) or using the RegexOptions.CultureInvariant option.
Bitvector Algebra Computation. Next, we construct a bitvector algebra A with 𝑘-bit bitvectors.
The Boolean operations of A are bit-wise arithmetic operations with nonzero test being satisfiability.
def
B
A as the
𝑘 −1 and [[𝛼 ]]
A has minterms 𝛼¯ = (2𝑖 )𝑖=0
𝑖 A = [[𝛽𝑖 ]] B . Each BDD 𝜓 in Γ is translated into 𝜓
B
B
A
bitwise-OR of all 𝛼𝑖 such that SAT B (𝛽𝑖 ∧ 𝜓 ), so [[𝜓 ]] B = [[𝜓 ]] A . The rest of the engine now
works solely with 𝑅0 modulo A.
We also precompute a minterm lookup dictionary 𝜇 that maps each 𝑎 ∈ Σ to the unique minterm
𝜇 (𝑎) such that 𝑎 ∈ [[𝜇 (𝑎)]] and subsequently we use this dictionary and satisfiability in A to
implement membership 𝑠𝑖 ∈ [[𝜓 ]] in Der𝑠 ⟨𝑖 ⟩ (𝜓 ) by SAT(𝜇 (𝑠𝑖 ) ∧ 𝜓 ). If 𝑎 ∈ Σ is an ASCII character
then 𝜇 (𝑎) uses an array and else a multi-terminal BDD to perform the lookup.
B
B
B
B
B
B
B
} and 𝛽¯ = (𝜓 [\W-[\n]]
,𝜓 [\w-[\d]]
)
For example, let 𝑅0 = ^\w+\d then Γ = {𝜓 \n
,𝜓 \d
,𝜓 \w
,𝜓 \n
,𝜓 \d
that gives us 𝛼¯ = (00012, 00102, 01002, 10002 ) in A. So for example 𝜓 \w = 𝛼 2 ∨ 𝛼 3 = 11002 in A.
5.2

Caching Der and Null

The calls to Der𝑠 ⟨𝑖 ⟩ (𝑅) made in MatchEnd are not trivially cacheable because 𝑖 changes for every
⟨𝑖⟩ indirectly through Null.
call. However, Der only accesses 𝑠𝑖 directly and the anchor-context 𝑠d
This observation leads to the following caching scheme.
The cache for Der is a map eDer : KIND × RE × 𝛼¯ → RE. Each call to Der𝑠 ⟨𝑖 ⟩ (𝑅) first checks
if eDer [𝜿 (𝑠 ⟨𝑖−1⟩), 𝑅, 𝜇 (𝑠𝑖 )] is defined, and if so, immediately returns it. Otherwise, Der𝑠 ⟨𝑖 ⟩ (𝑅) is
computed normally and eDer [𝜿 (𝑠 ⟨𝑖−1⟩), 𝑅, 𝜇 (𝑠𝑖 )] is updated to the result.
The cache for Null is a map eNull : KIND × RE × KIND → B. Calls to Null𝑥 (𝑅) will first check
the cache at eNull [𝜿 (𝑥 − 1), 𝑅, 𝜿 (𝑥)] and update it as necessary.
The consistency of eDer and eNull follow from: 1) if 𝑏 ∈ [[𝜇 (𝑎)]] then 𝑎 and 𝑏 are indistinguishable
in 𝑅0 and in any derivative derived from it; 2) definition of Null𝑥 (𝑅) depends only on ⟨𝜿 (𝑥−1), 𝜿 (𝑥)⟩.
Recursive calls to Der and Null also use the caches, which is important for achieving a complexity
over all of the calls to Der and Null that is linear in the length of a pattern where any loops have
been unrolled. See Example 5.4 for a demonstration.
Example 5.1. Consider 𝑅0 = ⊤*?\w+\b. The two minterms in A are (1, 2) = (012, 102 ) where
[[1]] = Σ\[[𝜓 \w ]] and [[2]] = [[𝜓 \w ]] , so \w is represented by 𝜓 \w = 2. Let 𝑠 = "–ABC· · · Z-" and
|𝑠 | = 1000. Note that 𝜇 (-) = 1 and 𝜇 (𝑎) = 2 if 𝑎 is a word-letter. We show how eDer evolves during
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 14 -->

148:14

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

the run of MatchEnd(𝑠, 𝑅0 ).
𝑥

𝑅

𝑠𝑖

𝜇 (𝑠𝑖 )

𝑠 ⟨0⟩
𝑠 ⟨1⟩
𝑠 ⟨2⟩
𝑠 ⟨3⟩
...
𝑠 ⟨998⟩
𝑠 ⟨999⟩
𝑠 ⟨1000⟩

𝑅0
𝑅0
𝑅1
𝑅1

A
B
C

1
2
2
2

𝑅1
𝑅1
𝑅0

Z
2
1
N/A

𝑅 ′ in MatchEnd(𝑥 + 1, 𝑅 ′ )

Cache update

𝑅0 = Der𝑠 ⟨0⟩ (𝑅0 )
𝑅1 = Der𝑠 ⟨1⟩ (𝑅0 ) = \w*\b|𝑅0
𝑅1 = Der𝑠 ⟨2⟩ (𝑅1 ) = \w*\b|\w*\b|𝑅0 = \w*\b|𝑅0
𝑅1 = eDer [w, 𝑅2, 2]

eDer [𝜺, 𝑅0, 1] ≔ 𝑅0
eDer [o, 𝑅0, 2] ≔ 𝑅1
eDer [w, 𝑅2, 2] ≔ 𝑅1

𝑅1 = eDer [w, 𝑅2, 2]
𝑅0 = Der𝑠 ⟨999⟩ (𝑅1 ) = ⊥|𝑅0 = 𝑅0

eDer [w, 𝑅1, 1] ≔ 𝑅0

Observe that the caches populate quickly, after which and the hot loop amounts to just reading
eDer . In the end MatchEnd(𝑠 ⟨0⟩, 𝑅0 ) returns 𝑠 ⟨999⟩ because through the run only Null𝑠 ⟨999⟩ (𝑅1 ) is
true. Although not shown, the behavior for eNull would be similar to that of eDer .
⊠
Example 5.1 implicitly used the rewrite rules that we will discuss next. The elimination of duplicate
alternatives at 𝑠 ⟨2⟩ exhibits the most critical rule for guaranteeing that |eDer | eventually converges.
5.3

Rewrite Rules

Our system implements the following rewrite rules:
⊥𝑅 → ⊥ 𝑅⊥ → ⊥ ⊥|𝑅 → 𝑅

𝑅|⊥ → 𝑅

⊤*|𝑅 → ⊤* 𝑅() → 𝑅

()𝑅 → 𝑅

As well as the following rules that we give names to for clarity:
OptOpt: 𝑅{0, 1, 𝑏 2 }{0, 1, 𝑏 1 } → 𝑅{0, 1, 𝑏 1 ∨ 𝑏 2 }

AltAssoc: (𝑅 | 𝑆) | 𝑇 → 𝑅 | (𝑆 | 𝑇 )

AltUni: 𝑅1 | · · · | 𝑅𝑛 → 𝑆 1 | · · · | 𝑆𝑛 where 𝑆𝑖 = (if ∀𝑗 <𝑖 (𝑅 𝑗 ≠ 𝑅𝑖 ) then 𝑅𝑖 else ⊥)
AltSub⪅ : 𝑅 | 𝑆 → 𝑇 if 𝑅 ⪅ 𝑆 and 𝑇 = FoldAlt(𝑅, 𝑆) ≠ ⊥

AltSub⪆ : 𝑅 | 𝑆 → 𝑅 if 𝑆 ⪅ 𝑅

These rules are applied in the constructors of the RE datatype, which ensures that regexes that
are non-canonical under them cannot be constructed. The AltSub rules rely on a more involved
notion of subsumption and will be discussed in Section 5.4. In addition to the rules above, we
augment the AltSub rules to also handle cases of the form 𝑅 | (𝑆 | 𝑇 ) as follows: if a rule would
rewrite 𝑅 | 𝑆 → 𝑈 then we will rewrite 𝑅 | (𝑆 | 𝑇 ) → 𝑈 | 𝑇 . The rule AltUni played a key role in
Example 5.1 to remove the duplicate alternative from \w*\b|\w*\b|𝑅0 .
Example 5.2. To illustrate how the rewrite rules interact with derivation, consider the regex
𝑅0 = ⊤*?(she|he) with an input 𝑠 = "she". The first two derivatives required are expanded below
with the rewrites applied shown on the right.
𝑅1 = Der𝑠 ⟨0⟩ (𝑅0 ) = Der𝑠 ⟨0⟩ (she|he)|Der𝑠 ⟨0⟩ (⊤*?)(she|he)
= Der𝑠 ⟨0⟩ (s)he|Der𝑠 ⟨0⟩ (h)e|⊤*?(she|he)
= ()he|⊥e|⊤*?(she|he) = he|⊤*?(she|he)
()he −
→ he, ⊥e −
→ ⊥, he|⊥ −
→ he
𝑅2 = Der𝑠 ⟨1⟩ (𝑅1 ) = Der𝑠 ⟨1⟩ (he)|Der𝑠 ⟨0⟩ (she|he)|⊤*?he
= e|⊥|e|⊤*?(she|he) = e|⊤*?(she|he)

e|⊥|e −AltUni
−−−−−→ e|⊥|⊥ −
→e

⊠

These rewrite rules ensure that from any 𝑅0 a finite set of derivatives are reachable [Brzozowski 1964]. However, the problem of recognizing the equivalence of two regexes is still PSPACEhard [Stockmeyer and Meyer 1973], which means that this rewrite system need and should not
be complete. Rather, the rules should target the shapes of regexes that arise from derivation and
recursive application of the rewrite rules. We view the task of selecting rewrite rules as a design
problem that must balance the cost and power of rewriting.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 15 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:15

Theorem 5.3. The implementation of Match(𝑠, 𝑅) has 𝑂 (|𝑠 |) complexity for all 𝑠 ∈ Σ∗ and 𝑅 ∈ RE.
Proof. MatchEnd runs at most twice over 𝑠. The sizes of eDer and eNull do not depend on |𝑠 |.

□

In effect, eDer and eNull form an automaton whose states are conditionally accepting based on
the next location kind. Section 5.5 describes a state caching technique that further exploits this.
5.4 Subsumption for Reducing Alternations
The AltSub rules capture how subsumption can be used to eliminate alternatives. We write 𝑆 ≦ 𝑅
when a regex 𝑅 subsumes 𝑆 (or 𝑆 is subsumed by 𝑅), or formally M (𝑆) ⊆ M (𝑅). Observe that
𝑅 ≡ 𝑆 ⇔ 𝑅 ≦ 𝑆 ≦ 𝑅. Let 𝑅 ≡bt 𝑆 stand for 𝑅 ≡ 𝑆 and BT(𝑅) ≡ BT(𝑆) as a strong equivalence that
preserves both backtracking as well as nonbacktracking semantics. Our system includes a sound
but incomplete test ⪅ for subsumption implemented with the following inference rules. In the
following let 𝑋, 𝑌 , 𝑍 ∈ RE. An important property of the rules is that they maintain ≡bt in AltSub
rules, which is the only context where they are being used. The standard notation for a regex 𝑅
being eagerly optional is 𝑅? def
= 𝑅|() and being lazily optional is 𝑅?? def
= ()|𝑅.
⊥⪅𝑋

𝑋 ⪅𝑋

SubLazy1 :

𝑋 ⪅𝑌
𝑍𝑋 ⪅ 𝑍 ??𝑌

Null ∀ (𝑍 ) 𝑋 ⪅ 𝑌
SubLazy2 :
𝑍 ??𝑋 ⪅ 𝑍𝑌

Null ∀ (𝑍 ) 𝑋 ⪅ 𝑌
SubNull:
𝑋 ⪅ 𝑍𝑌

As an example of how subsumption enables the AltSub rewrites, by SubLazy1 it holds that
⪆ 𝑍 ??𝑌 . To see that the rewrite is correct,
𝑍𝑌 ⪅ 𝑍 ??𝑌 , which enables the rewrite 𝑍 ??𝑌 |𝑍𝑌 −AltSub
−−−−−−→
observe that 𝑍 {0, 1}? ≡bt ()|𝑍 and thus 𝑍 ??𝑌 ≡bt (𝑌 |𝑍𝑌 ). The rewrite can be thought of as an
implicit application of AltUni onto 𝑌 | 𝑍𝑌 | 𝑍𝑌 .
The SubNull rule is the workhorse for detecting nullable prefixes and would, for example,
establish that a ⪅ a?a and thus a?a|a is rewritten to just a?a via AltSub⪆ . It would not, however
be valid to rewrite a|a?a into a?a because the preference for match end locations would be altered.
BT(a|a?a) "aa"⟨1⟩ while "aa"⟨0⟩ BT(a?a|a) "aa"⟨2⟩.
For example, "aa"⟨0⟩ −−−−−−−−
−−−−−−−−→
→
The correct rewrite in this case is a|a?a −
→ a??a where the right side’s eager option a? has been
“folded” into the left alternative as a lazy option. The AltSub⪅ rule handles such rewrites with the
FoldAlt function, defined as follows:
FoldAlt(𝑅, 𝑆) def
= if (𝑅 ⪅ 𝑆 and 𝑃 = SubsPrefix(𝑅, 𝑆) ≠ ⊥) then 𝑃??𝑅 else ⊥
SubsPrefix(𝑅, 𝑆) def
= if 𝑆 ⪅ 𝑅 then () else (if 𝑆 = 𝑃𝑇 and 𝑅 ⪅ 𝑇 then 𝑃 ·SubsPrefix(𝑅,𝑇 ) else ⊥)
FoldAlt implements a rule that if 𝑆 subsumes 𝑅 due to there being a nullable prefix 𝑃 such that
𝑆 = 𝑃𝑇 then 𝑅|𝑆 can be rewritten to 𝑃??𝑅. The SubLazy rules are designed to prove subsumption
for the shapes of regexes resulting from FoldAlt.
These subsumption based rewrites proved critical for ensuring acceptable performance for
patterns that include long concatenations of nullable regexes. The danger with such patterns is
that if a pattern like a?a?· · · a? is allowed to evolve into an alternation of linear size, it becomes
difficult to avoid quadratic behavior over multiple derivations. The following examples show how
the AltSub rules work to avoid such blow-up.
Example 5.4. Consider the regex 𝑅 = a?a?a? and input 𝑠 = "aaa". For 𝑥 = 𝑠 ⟨0⟩ it holds that:
Der𝑥 (𝑅) = Der𝑥 (a?)a?a?|Der𝑥 (a?a?)
= Der𝑥 (a?)a?a?|Der𝑥 (a?)a?|Der𝑥 (a?)
AltSub⪆ a?a?
⪆ a?, a?a?|a? −
= a?a?|a?|() = a?a?|a? = a?a?
a?|() −AltSub
−−−−−−→
−−−−−−→
The AltSub⪆ rule helped avoid alternations in the result. For longer concatenations of nullable
regexes, this would avoid an 𝑂 (𝑛 2 ) blow-up in the size of the result. Now consider the regex
𝑆 = a??a??a?? instead. For 𝑥 = 𝑠 ⟨0⟩ it holds that:
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 16 -->

148:16

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

Der𝑥 (𝑆) = Der𝑥 (a??)|Der𝑥 (a??)a??|Der𝑥 (a??)a??a??
OptOpt
= ()|a??|a??a??
()|a?? −AltSub
−−−−−−⪅
→ (a??)?? −−−−−−→ a??
AltSub
,OptOpt
⪅
a??|a??a?? −−−−−−−−−−−−−−→ a??a??
= a??|a??a?? = a??a??
Similar quadratic blow-up is avoided here as above.

⊠

Depth Limit. The inference rules for subsumption are realized by implementing ⪅ as a function
that tries to apply each rule in turn by checking their conditions with recursive calls to itself when
required. This, however, may lead to deep recursions that are ultimately unproductive. For example,
consider the regexes 𝑅 = a?· · · a? and 𝑆 = a?𝑅. For a query 𝑆 ⪅ 𝑅 the SubNull rule would be tried
until a query 𝑆 ⪅ a? is made, at which point SubNull can no longer apply due to the left side not
being a concatenation. To limit this kind of unproductive work we introduce a recursion depth limit
for any subsumption query triggered by a rewrite rule. If the limit is reached ⪅ returns with failure.
In our implementation this depth limit is set to 50, which we found to be sufficient to cover realistic
patterns while limiting the impact for malicious patterns.
Subsumption Hinting. Note that in the example of the paragraph above, 𝑅 ⪅ 𝑆 does hold and,
furthermore, between the AltSub⪆ and AltSub⪅ rules both directions of subsumption will be
checked if the alternation 𝑅|𝑆 is constructed. This is particularly relevant for Der𝑥 (𝐿 · 𝑅) when 𝐿
is nullable (see Section 4.2). The order of the alternation between Der𝑥 (𝑅) and Der𝑥 (𝐿) · 𝑅 varies
depending on the high-nullability of 𝐿, such that AltSub⪅ is more likely to apply when 𝐿 is highnullable and AltSub⪆ when 𝐿 is not. To take advantage of this our implementation passes a hint
from Der to the alternation constructor to indicate which rule should be tried first.
Note on Extensibility. We have found ⪅ and FoldAlt to serve as useful points of extensibility for the rewriting system. Though the rules presented above mainly deal with subsumption due to nullable prefixes, we have on-going work on enabling regex subcapture matching
that introduces new types into RE. The ⪅ and FoldAlt functions can be extended to handle
these new types in a modular way. Loop subsumption optimizations in the style of [Saarikivi
et al. 2019] but corrected for backtracking semantics are another viable extension, e.g., the rule
𝑅{0, 𝑚}?|𝑅{0, 𝑛}? → 𝑅{0, max(𝑚, 𝑛)}?.

5.5

Other Implementation Considerations

Equality Checks. The caching and rewrite rules described in the previous sections rely on knowing
when regexes are structurally equal. To make this cheap, our implementation interns all regexes
such that pointer equality coincides with structural equality.
State Caching. As shown is Example 5.1, for sufficiently long inputs the vast majority of
calls to Der and Null will immediately hit the eDer and eNull caches. We implement a state graph
construction that further optimizes these top-level cache lookups.
A set 𝑄 of states seen so far is maintained, where each state is a pair ⟨𝜅, 𝑅⟩ ∈ KIND × RE of
the kind 𝜅 of the previous character that transitioned into this state and its derivative 𝑅. The
parameter 𝑅 of MatchEnd is replaced by a state in 𝑄 with ⟨𝜺, 𝑅0 ⟩ as the initial state. Top-level
lookups into eDer resolve instead through a lookup table £Der : 𝑄 × 𝛼¯ → 𝑄. For each entry
eDer [𝜅, 𝑅, 𝜇 (𝑎)] = 𝑅 ′ resulting from a top-level call to Der, the lookup table will have an entry
£Der [⟨𝜅, 𝑅⟩, 𝜇 (𝑎)] = ⟨𝜿 (𝑎), 𝑅 ′ ⟩. A similar lookup table £Null : 𝑄 × KIND → B is introduced for Null.
By associating each state with an index, these lookup tables can be implemented as flat arrays
that are grown on demand as new states are encountered. Furthermore, because £Der no longer
depends on the kind of the previous character, MatchEnd(𝑠 ⟨𝑖⟩, 𝑅) performs just one read into 𝑠𝑖 per
iteration. Altogether, £Der and £Null greatly improve the performance of the matching loop.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 17 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:17

NFA Mode. When the size of 𝑄 reaches a certain threshold12 the search engine switches into NFA
mode and the current state 𝑠 ∈ 𝑄 is converted into an ordered set 𝑆 such that ∀𝑖 (𝜋 1 (𝑠) = 𝜋 1 (𝑆𝑖 ))
|𝑆 |
and 𝜋 2 (𝑠) ≡ |𝑖=1
𝜋 2 (𝑆𝑖 ). Essentially, alternation inside the state is broken up into separate states.
Derivatives are computed for each 𝑆𝑖 separately, broken up again into ordered sets and inserted
into the successor 𝑆 ′ in order. One subtlety that arises is that the NFA mode must reimplement part
of Prune. When computing derivatives for 𝑆, if 𝜋2 (𝑄 𝑆𝑖 ) = BT(𝑇 ) for some 𝑖 and 𝑇 is nullable in the
current context, then all 𝑆 𝑗 for 𝑗 > 𝑖 are ignored. This mirrors how alternations are pruned.
Observe that the NFA mode is a space-time tradeoff, since iterating 𝑆 takes more time but
uses less space since only the components of 𝑆 are cached. The classical analogue here is that
each 𝑆𝑖 corresponds to a partial derivative [Antimirov 1995] in contrast to a (full) derivative
|𝑆 |
|𝑖=1
𝜋2 (𝑆𝑖 ) [Brzozowski 1964]. To minimize the performance impact, we implement 𝑆 as the sparse
ordered set data structure for small integers described in [Briggs and Torczon 1993], which allows
for 𝑂 (1) ordered set insertion and ordered iteration over contiguous memory. The NFA mode also
∗
uses a separate lookup table £NFA
Der : 𝑄 × 𝛼¯ → 𝑄 that directly caches the ordered sets of target states.
Fixed Length Matches. The second call to MatchEnd in Match can be avoided when the match is
found in a fixed length fragment of 𝑅0 . With 𝑅0 = ab|a+b and 𝑠 = "abc" the derivative at 𝑠 ⟨2⟩ would
be () | 𝑅0 , which has lost track of the fact that the match happened through ab. To overcome this
we tag 𝑅0 with fixed-length markers ()𝑛 that act like epsilon in Der and Null, e.g., 𝑅0 = ab()2 | a+b.
With this the derivative at 𝑠 ⟨2⟩ would instead be ()2 | () | 𝑅0 , which lets us resolve the whole
match as ⟨𝑠 ⟨0⟩, 𝑠 ⟨2⟩⟩ without having to call MatchEnd(𝑠 ⟨2⟩𝑟, 𝑅𝑟0)𝑟. This optimization roughly doubles
asymptotic throughput for a common class of patterns that are alternations of fixed strings.
Subcaptures. The matching algorithm uses tagged derivatives with tags recording start and end
locations of sub-patterns, during a third phase, run only when grouping constructs [Microsoft 2021c]
are present, and results in incrementally creating a variant of a tagged NFA [Laurikari 2000].
6 .NET INTEGRATION
Our implementation is integrated as a new backend for the System.Text.RegularExpressions
library in the .NET runtime and may be triggered with the RegexOptions.NonBacktracking flag.
The feature is available in .NET7 – released in November 2022.
The implementation is pure C#, but integrates with existing prefix optimizations for other .NET
regex engines. These optimizations target patterns like abc\w+, where the fixed string "abc" can
be efficiently searched for with vectorized string search procedures. We also employ a number of
low-level optimization tricks to ensure that the hot matching loop is as fast as possible, such as using
local variables to avoid read/writes to ref/out values, and having MatchEnd be a generic function
parameterized by a DFA or NFA state handler constrained to a struct with a static interface, which
allows inlining of state handling logic for both modes. We encourage interested readers to study
the open-source implementation in [Microsoft 2022].
The integration of NonBacktracking into .NET runtime went through extensive testing,
involving thousands of regexes covering multiple standard test suites, with tests that verify expected and mutually equal match results for all the .NET regex engines (NonBacktracking,
Compiled, and None) including all supported RegexOptions and covering all platforms supported
by .NET. The tests helped uncover multiple bugs in both implementation and theory. An early
bug in NonBacktracking was related to case-insensitivity in combination with complement in
character classes where case-insensitivity was applied after complement (as in SRM), while the
Unicode standard is to apply case-insensitivity before complement. E.g., (?i:[^B]) ≡ [^Bb] and
12 This threshold is 10000 by default in .NET7.

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 18 -->

148:18

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

Time (ms)

103
102
101
100

C#

l
a
2
e
c
o
a
3
3
2
P
g
d st
y rl st
n
Nim PH ckin Juli pile Ru alpha CRE PyPy Rub Pe Boo thon rysta Kotli Jav D ld Non G
#
C
- C P on
+ Py
C
om
ra
0
+
.
t
C
C
k
th
2.0
C#
ac
Py
M
nB
SR

No

Fig. 1. Various matchers on 15 patterns for the works of Mark Twain, ordered by average matching time.

(?i:[\0-AC-\uFFFF]) ≡ [\0-\uFFFF]13 although [^B] ≡ [\0-AC-\uFFFF]. Certain rewrites by
the regex parser, e.g., rewrites to atomic regexes (?>. . .), are not supported in NonBacktracking and had to be omitted. We found several bugs in early versions of pruning rules, such as
Prune𝑥 (𝑅{𝑚}) → Prune𝑥 (𝑅){𝑚} being incorrect when 𝑅 contains anchors, and in rewrite rules
adopted from SRM that violated backtracking semantics, such as the loop rewrite rule mentioned
in Section 1. These findings motivate further rigor: we intend to formalize our theory using a proof
assistant to provide even more confidence for the current implementation and also to serve as a
platform for verifying future optimizations.
7 EXPERIMENTS
In this section we evaluate the new engine against the other regular expression matchers, including
.NET’s other main backends and the SRM library [Saarikivi et al. 2019]. We refer to the .NET
engines by the name of the RegexOptions enum member that triggers them, i.e., None, Compiled
and NonBacktracking. All experiments below use .NET 7 Release Candidate 2.
When matching “well behaved” patterns (i.e. no catastrophic backtracking or state space explosion), performance is dominated by 1) the pre/post/infix search optimizations available and 2)
the byte-to-byte transition logic in the innermost matching loops. The search optimizations are
largely orthogonal to the matching approach, while both derivative- and automata-based engines
will have very similar innermost loops implementing transitions in a cached/lazily constructed
DFA. Therefore, the most interesting aspect to compare across regex engines is their performance
with potential outliers and this is what our evaluation focuses on.
7.1

Comparison with Standard Library Matchers

First we compare NonBacktracking with standard matchers for 16 different programming languages. We took a popular cross language benchmark [Juárez 2020] and to add potential outliers
we replaced its dataset14 with the “Twain” benchmark [Herczeg 2015], consisting of 15 patterns
for the collected works of Mark Twain. We excluded several redundant matchers as well as ones
13 The concrete syntax (?i:𝑅) means that 𝑅 is evaluated locally with RegexOptions.IgnoreCase, thus treating all character

classes that occur in 𝑅 as case-insensitive. Case-insensitivity of some Unicode characters, such as the Turkish ¤I, moreover
depend on the runtime culture (System.Globalization.CultureInfo), that was another source of subtle bugs.
14 The original dataset is three patterns for parsing emails, URIs and IPs, which are well behaved in all engines.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 19 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:19

7.2 Case Study: Word Phrase Matching
We extracted a dataset of 184 patterns from an industrial
word phrase matching use case. They use the \b anchor
extensively and include large alternations. The data is
5 MB of English dialog extracted from the MultiWOZ
dataset [Budzianowski et al. 2018].
We measured matching time for each pattern using
the .NET Performance tool [Microsoft 2021b], which we
modified to use the new patterns and data. The experiment ran on an Intel Core i7-1185G7 machine with 32 GB
of memory running Windows 11. Figure 2 presents the
results as a scatter plot.
The results highlight how in NonBacktracking any
number of alternations can be handled with little extra
cost, while backtracking engines match each option separately. The geometric mean speedup is 4.7× and the difference in total matching time is even larger at 24×. For
applications that depend on regular expression matching,
this kind of performance differential can be critical. Furthermore, the high variability in performance may lead to

Compiled (ms)

missing support for case-insensitive groups and added Compiled, NonBacktracking and SRM
version 2.0.0-alpha2. We verified that all of the .NET engines produce the same number of matches
for each pattern. The measurements were performed on an Azure Dsv4-series VM running an
Ubuntu 18.04 Docker image with 2 cores of an Intel Xeon 8272CL processor and 8 GB of memory.
Figure 1 presents the results.
C#’s None and Compiled have similar performance profiles due to using the same matching
logic, but the code generation helps speed up all patterns and already places C# near the top
of the pack. NonBacktracking further improves the performance and places it 3rd in average
matching time among the included matchers. Compared to both Compiled and SRM 2.0.0-alpha2
the improvement is mainly from avoiding outliers and having a more consistent performance
profile. The average/best cases seem largely equal between NonBacktracking and Compiled.
General search optimizations are shared between all the engines. In the hot (DFA mode) matching
loop, NonBacktracking uses additional optimizations that are specific to the algorithms discussed
in Section 5, such as dead-end state detection for early search failure.
Incidentally, the result for Rust further highlights the importance of avoiding outliers, as it would
be a clear winner if not for the pattern [a-q][ˆu-z]{13}x. The crux of the pattern is that [a-q]
denotes a subset of [ˆu-z] much like in the classical example a.{13}. This pattern is challenging
for DFA based engines, as the loop can be entered multiple times, leading to a 213 factor in number
of states. While lazy exploration of the state space with derivatives helps, this pattern is still the
slowest one for SRM 2.0.0-alpha2. Optimizations in NonBacktracking have made this pattern no
longer visible as an outlier. SRM cached states in a fixed-size array and a dictionary for overflow.
NonBacktracking instead grows the array on-demand, avoiding expensive dictionary lookups
and improving performance for patterns that create many states.
Even though the Go matcher in Figure 1 supports the RE2 regex dialect it is a reimplementation
in Go. We view the Rust engine as a better version of RE2, which implements the same techniques
but with many more micro-optimizations.

103
102
101
100
100 101 102 103
NonBacktracking (ms)

Fig. 2. Log-scale matching time of 184 patterns from an industrial word phrase matching dataset on 5 MB of English dialog.
Points above the line are faster with NonBacktracking.

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 20 -->

148:20

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

inconsistent user experiences: for NonBacktracking the slowest pattern takes only 0.26 seconds
longer than the fastest one, while for Compiled the difference is 13.6 seconds.

Compiled (s)

Case Study: Credential Scanning
102

101

101
102
NonBacktracking (s)

102
None (s)

7.3

CredScan [Microsoft 2021a] is a tool that scans source
code and other cloud datasets for leaked credentials and
other sensitive content. It uses a large set of regular expressions to identify potential leaks. We modified the tool
to use NonBacktracking and compare end-to-end scanning time against None and Compiled in .NET 7. Figure 3
presents the results from scanning 188 production repositories comprising 15 GB of data. Each point in the two
scatter plots represents a single repository and the time it
took to scan it with the baseline and NonBacktracking.
NonBacktracking gives overall speedups of 26% and
9% against None and Compiled, respectively. This is despite a lack of severe outliers for the backtracking engines,
which were the source of the massive speedups in Section 7.2: the best speedups for any single repository are
2.5× and 1.5× against None and Compiled, respectively.
The speedups are still driven by NonBacktracking’s
more consistent performance profile, which is visible as
lower scanning times on larger repositories. For small
repositories None, especially, is faster than NonBacktracking. This is due to higher initialization overhead for
NonBacktracking of the hundreds of patterns CredScan
includes. We believe the alphabet compression step described in Section 5.1 to be the main contributor. Against
Compiled the difference for the small repositories is
smaller, as the compilation it performs results in similar per-pattern overheads. We are investigating ways to
reduce the initialization overhead of NonBacktracking
through techniques such as code generation.

101

100
100

101
102
NonBacktracking (s)

Fig. 3. Log-scale end-to-end scanning time
of CredScan with different regex engines applied to 188 source code repositories. Points
above the line are faster with NonBacktracking.

8 RELATED WORK
The discussion here is limited to regular features of regexes and to derivative and automata based
techniques. The complete regex language of ECMAScript, covering non-regular features such as
backreferences and balancing groups (see [Loring et al. 2019]) is out of scope of the work here.
Derivative Foundations. Derivatives were introduced in [Brzozowski 1964] for DFAs and
reformulated in [Antimirov 1995] as partial derivatives for NFAs. Derivatives for symbolic regexes
were studied in [Keil and Thiemann 2014]. These works do not study anchors or lookarounds.
Alphabet Compression in Regex Matchers. Use of minterms in our case is key to fast and
straightforward state transition memoization while incrementally maintaining a state transition
graph both for DFAs and NFAs by keeping the size of the out-degree of transitions as small as
possible, in a regex dependent manner. Use of minterms as an alphabet compression technique for
regexes was first observed in [Hooimeijer and Veanes 2011]. Other tools like RE2 apply different
techniques where the input is converted to UTF8, independent of regex, and a specialized automaton
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 21 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:21

is used to recognize UTF8 encoding during reading of the input [Cox 2010]. In .NET this would not
be possible in general due to, e.g., UTF16 surrogates being valid characters with no UTF8 encoding.
Derivative Based Matching. Derivatives were originally studied in [Fischer et al. 2010; Owens
et al. 2009] for IsMatch. The work in [Sulzmann and Lu 2012] studies MatchEnd using Antimirov
derivatives and POSIX (leftmost longest) semantics. Subsequently in [Ausaf et al. 2016] that work is
improved and elegantly formalized in Isabelle/HOL and also extended for Brzozowski derivatives.
This work is very inspiring for us as we also intend to formalize our algorithm using a proof
assistant. The key similarity to our work is that ordering of alternatives arising from derivatives of
concatenations 𝑅·𝑆 is based on a variant of high-nullability of 𝑅 that determines the order in which
the resulting derivatives are then processed to maintain POSIX semantics. The key difference to our
work, is that nullability of regexes in these works is not context dependent and anchors and counting
are not considered. For regexes with anchors the classical language properties 𝐿(𝑅·𝑆) = 𝐿(𝑅)·𝐿(𝑆)
and 𝐿(𝑅 ∗ ) = 𝐿(𝑅) ∗ (axioms (4) and (6) in [Ausaf et al. 2016]) do not hold.
Anchors have so far been treated in ad-hoc fashion. In match generation anchors are a key
ingredient defining the boundaries of a match and therefore cannot be eliminated by preprocessing.
To this end, and to the best of our knowledge, the theory presented here is novel. In [Wingbrant
2019] anchors are treated as imaginary characters in the input using classical derivatives. This
approach does not preserve backtracking semantics, which we learned the hard way – this was
also our initial approach. Moreover, there are critical semantic differences compared to the classical
treatment of loops in the foundations of derivatives (recall Section 3.5) when anchors are used.
While some aspects of NonBacktracking build on SRM [Saarikivi et al. 2019], the current
work is based on a fundamental redesign of the foundations to support anchors and backtracking
semantics. Moreover, the top-level matcher of SRM is less efficient because it needs three passes
over the input to locate a match, instead of two.
Automata Based Matching. Modern automata based regular expression matchers such as
RE2 [Cox 2010] and grep [GNU 2020] also use state graph memoization similarly to NonBacktracking. One can roughly classify these matchers as highly optimized variants of [Thompson 1968]
enhanced with DFA state caching, or in the case of Hyperscan [Intel Co 2021] based on [Glushkov
1961]. As far as we know, there are no analogues of our Correctness theorem for these engines, that
would be difficult to state since anchors and backtracking are outside classical automata theory.
A key contribution of our work is the set of rewrite rules in Section 5.3, which gives powers that
we believe are unavailable to automata-caching engines that maintain a lazy DFA construction even
if the minimal DFA might be small, since the upfront cost of DFA construction and minimization is
undesirable for the target use case. Derivatives allow DFA-minimizing optimizations to be applied
on-the-fly. Relating exactly which rewrite rules go beyond what engines like RE2/Rust do, is difficult,
but in [Owens et al. 2009, Table 1] it is shown that derivatives with good rewrite rules often provide
minimal DFAs. Similar conclusions are drawn in [Sulzmann and Lu 2012, Section 5.4] relating NFA
sizes arising from Thompson’s and Glushkov’s, versus Antimirov’s constructions. The subsumptionbased rules in Section 5.3 are certainly beyond what is easily possible in automata-caching, because
in automata the subsumption checks would require global analysis.
The backtracking semantics and anchors support cover necessary modern features, while the
alphabet compression and caching scheme are necessary for good performance. Greedy matching
algorithm for backtracking (PCRE) semantics was originally introduced in [Frisch and Cardelli
2004], based on 𝜖-NFAs, while maintaining matches for eager loops. We do not believe an extension
to anchors is straightforward in this work, because anchors are context conditions with no direct
semantics in classical automata. In particular, [Frisch and Cardelli 2004, Proposition 2] assumes the
axiom 𝐿(𝑅·𝑆) = 𝐿(𝑅)·𝐿(𝑆) that fails with anchors. Derivatives also avoid the issue of 𝜖-transitions
in [Frisch and Cardelli 2004] that do not arise with derivatives, that is also true in [Ausaf et al. 2016;
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 22 -->

148:22

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

Sulzmann and Lu 2012]. Even if successfully extended with modern features the constructions
in [Cox 2010; Frisch and Cardelli 2004], result in an NFA that loses the local meaning of states,
which derivatives do maintain and thus enable further rewriting-based optimizations.
Our proof of correctness helped us find bugs, and gives confidence for future extensions. Our
work to modernize derivatives will enable features out of reach for engines like RE2, such as support
for intersection and complement, which SRM has but we cut for now due to lack of support in
.NET7’s regex dialect. Further minimizations with rewrite rule-based optimizations, such as, a
backtracking compatible version of SRM’s loop-subsumption rule, is ongoing work.
The two phases of the top-level matching algorithm in .NET NonBacktracking – forward phase
to find the match end location and backward phase to find the match start location – correspond
to those in RE2 [Cox 2010]. The preliminary third phase for sub-capture support, which is also
derivative based, is different. In [Cox 2010] the third phase will sometimes fall back to a backtracking
NFA execution, losing input-linearity. In our case, the third phase is also a linear pass over the
matched substring that uses tagged derivatives with tags recording start and end locations of
sub-patterns to be captured, resulting in a variant of a tagged NFA [Laurikari 2000].
When switching to NFA mode (recall Section 5.5) the overall effect is similar to [Cox 2010], where
mapping from DFA states to sets of NFA states has to be maintained, but in our case Brzozowski-style
derivatives can switch to Antimirov-style derivatives without any prior bookkeeping.
Derivative Based Analysis. SMT solvers that support regexes via derivatives are Z3 [de Moura
and Bjørner 2008] that now uses a generalization of derivatives called transition regexes [Stanford
et al. 2021], and CVC4 [CVC4 2020; Liang et al. 2015] that uses Antimirov derivatives. In a recent
study [Turoňová et al. 2020], derivatives were used to extend NFAs with counting arising from
Antimirov-style exploration of standard regexes supporting finite loops. Kleene algebras with
tests [Kozen 1997] have also been used to work with derivatives of symbolic classical regexes where
predicates are encoded by BDDs [Pous 2015]. An interesting direction for future work would be to
extend location based derivatives with intersection and complement and to also support lookarounds
and MatchEnd in SMT solvers, in addition to IsMatch over extended regular expressions.
Match Generation Semantics. The two most well-known standards for matching are PCRE
and POSIX with a formalization of POSIX based on tagged automata [Laurikari 2000]. There is also
a Boost variant of POSIX [Berglund et al. 2021]. PCRE semantics of NonBacktracking is needed
in .NET for compatibility with the other regex backends Compiled and None. PCRE is also the
more widely adopted semantics. POSIX finds the longest leftmost match, while PCRE stops on the
first match (according to backtracking). E.g., consider the regex (a|ab)* and the input string "abab",
where the PCRE match is the prefix "a" of the input while the POSIX match is the whole input.
9 CONCLUSIONS
The new nonbacktracking regex backend for .NET delivers significant speedups for real-world use
cases: 4.7× speedup on an internal word phrase matching task and 9% better end-to-end throughput
on a benchmark of CredScan [Microsoft 2021a]. We believe it can both enable new use cases and
significantly reduce resource requirements for users of .NET regular expressions. The correctness
of the matching algorithm has not only allowed us to integrate our work into .NET with confidence
but also experimentally confirmed mutual semantic consistency among all the backends. The
framework itself is open source and available for use as a research platform to explore new ideas.
ACKNOWLEDGMENTS
We thank all our reviewers and shepherd Konstantinos Mamouras for valuable feedback during the
review process. We particularly appreciated the suggestions that lead to a more detailed comparison
with related work, and a clearer positioning of the evaluation results.
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 23 -->

Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics

148:23

DATA AVAILABILITY STATEMENT
The implementation is available in .NET Runtime 7 under the RegexOptions.NonBacktracking
flag. Source code is available in the .NET Runtime repository [Microsoft 2022]. Additionally, an
artifact with instructions for reproducing the results in Section 7.1 is available [Moseley et al. 2023b].
The artifact does not cover Sections 7.2 and 7.3, as both of them concern proprietary datasets that
could not be made available.
REFERENCES
Valentin Antimirov. 1995. Partial Derivatives of Regular Expressions and Finite Automata Constructions. Theoretical
Computer Science 155 (1995), 291–319. https://doi.org/10.1007/3-540-59042-0_96
Fahad Ausaf, Roy Dyckhoff, and Christian Urban. 2016. POSIX Lexing with Derivatives of Regular Expressions (Proof Pearl).
In Interactive Theorem Proving (LNCS, Vol. 9807), Jasmin Christian Blanchette and Stephan Merz (Eds.). Springer, 69–86.
https://doi.org/10.1007/978-3-319-43144-4_5
Adam Baldwin. 2016. Regular Expression Denial of Service affecting Express.js. http://web.archive.org/web/20170116160113/
https://medium.com/node-security/regular-expression-denial-of-service-affecting-express-js-9c397c164c43.
Martin Berglund, Willem Bester, and Brink van der Merwe. 2021. Formalising and implementing Boost POSIX regular
expression matching. Theoretical Computer Science 857 (2021), 147–165. https://doi.org/10.1016/j.tcs.2021.01.010
Preston Briggs and Linda Torczon. 1993. An Efficient Representation for Sparse Sets. ACM Lett. Program. Lang. Syst. 2, 1–4
(mar 1993), 59–69. https://doi.org/10.1145/176454.176484
Janusz A. Brzozowski. 1964. Derivatives of regular expressions. JACM 11 (1964), 481–494. https://doi.org/10.1145/321239.
321249
Paweł Budzianowski, Tsung-Hsien Wen, Bo-Hsiang Tseng, Iñigo Casanueva, Ultes Stefan, Ramadan Osman, and Milica
Gašić. 2018. MultiWOZ - A Large-Scale Multi-Domain Wizard-of-Oz Dataset for Task-Oriented Dialogue Modelling. In
Proceedings of the 2018 Conference on Empirical Methods in Natural Language Processing (EMNLP). https://doi.org/10.
18653/v1/D18-1547
Russ Cox. 2010. Regular Expression Matching in the Wild. https://swtch.com/~rsc/regexp/regexp3.html
CVC4. 2020. https://github.com/CVC4/CVC4.
Loris D’Antoni and Margus Veanes. 2014. Minimization of Symbolic Automata. ACM SIGPLAN Notices – POPL’14 49, 1
(2014), 541–553. https://doi.org/10.1145/2535838.2535849
James C. Davis. 2019. Rethinking Regex Engines to Address ReDoS. In Proceedings of ESEC/FSE’19 (Tallinn, Estonia) (ESEC/FSE
2019). ACM, New York, NY, USA, 1256–1258. https://doi.org/10.1145/3338906.3342509
James C. Davis, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. 2018. The Impact of Regular Expression Denial
of Service (ReDoS) in Practice: An Empirical Study at the Ecosystem Scale. In Proceedings of ESEC/FSE’18 (Lake Buena
Vista, FL, USA) (ESEC/FSE 2018). ACM, New York, NY, USA, 246–256. https://doi.org/10.1145/3236024.3236027
Leonardo de Moura and Nikolaj Bjørner. 2008. Z3: An Efficient SMT Solver. In TACAS’08 (LNCS). Springer, 337–340.
https://doi.org/10.1007/978-3-540-78800-3_24
Sebastian Fischer, Frank Huch, and Thomas Wilke. 2010. A Play on Regular Expressions: Functional Pearl. SIGPLAN Not. 45,
9 (2010), 357–368. https://doi.org/10.1145/1863543.1863594
Alain Frisch and Luca Cardelli. 2004. Greedy Regular Expression Matching. In Automata, Languages and Programming
(ICALP’04) (LNCS, Vol. 3142), Josep Díaz, Juhani Karhumäki, Arto Lepistö, and Donald Sannella (Eds.). Springer, 618–629.
https://doi.org/10.1007/978-3-540-27836-8_53
V. M. Glushkov. 1961. The abstract theory of automata. Russian Math. Surveys 16 (1961), 1–53. https://doi.org/10.1070/
RM1961v016n05ABEH004112
GNU. 2020. grep. https://www.gnu.org/software/grep/.
Google. 2021. RE2. https://github.com/google/re2.
John Graham-Cumming. 2019. Details of the Cloudflare outage on July 2, 2019. https://blog.cloudflare.com/details-of-thecloudflare-outage-on-july-2-2019/.
Zoltan Herczeg. 2015. Performance comparison of regular expression engines. https://zherczeg.github.io/sljit/regex_perf.
html.
Pieter Hooimeijer and Margus Veanes. 2011. An Evaluation of Automata Algorithms for String Analysis. In VMCAI’11
(LNCS, Vol. 6538). Springer, 248–262. https://doi.org/10.1007/978-3-642-18275-4_18
Intel Co. 2021. Hyperscan. https://github.com/intel/
Mario Juárez. 2020. Languages Regex Benchmark. https://github.com/mariomka/regex-benchmark.
Matthias Keil and Peter Thiemann. 2014. Symbolic Solving of Extended Regular Expression Inequalities. In FSTTCS’14
(LIPIcs). 175–186. https://doi.org/10.4230/LIPIcs.FSTTCS.2014.175
Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.

<!-- page 24 -->

148:24

D. Moseley, M. Nishio, J. Perez Rodriguez, O. Saarikivi, S. Toub, M. Veanes, T. Wan, E. Xu

Dexter Kozen. 1997. Kleene algebra with tests. TOPLAS 19, 3 (1997), 427–443. https://doi.org/10.1145/256167.256195
V. Laurikari. 2000. NFAs with tagged transitions, their conversion to deterministic automata and application to regular
expressions. In 7th International Symposium on String Processing and Information Retrieval. 181–187. https://doi.org/10.
1109/SPIRE.2000.878194
Tianyi Liang, Nestan Tsiskaridze, Andrew Reynolds, Cesare Tinelli, and Clark Barrett. 2015. A Decision Procedure for
Regular Membership and Length Constraints over Unbounded Strings?. In Frontiers of Combining Systems, FroCoS 2015
(LNCS, Vol. 9322). Springer, 135–150. https://doi.org/10.1007/978-3-319-24246-0_9
Blake Loring, Duncan Mitchell, and Johannes Kinder. 2019. Sound Regular Expression Semantics for Dynamic Symbolic
Execution of JavaScript. In PLDI’19. ACM, 425–438. https://doi.org/10.1145/3314221.3314645
Microsoft. 2021a. CredScan. https://secdevtools.azurewebsites.net/helpcredscan.html.
Microsoft. 2021b. .NET Performance. https://github.com/dotnet/performance.
Microsoft. 2021c. Regular Expression Language - Quick Reference. https://docs.microsoft.com/en-us/dotnet/standard/basetypes/regular-expression-language-quick-reference.
Microsoft. 2022. .NET Regular Expressions. https://github.com/dotnet/runtime/tree/main/src/libraries/System.Text.
RegularExpressions.
Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu. 2023a.
Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics. Technical Report MSR-TR2023-15. Microsoft Research. https://www.microsoft.com/en-us/research/publication/derivative-based-nonbacktrackingreal-world-regex-matching-with-backtracking-semantics/
Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu.
2023b. Artifact for "Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics".
https://doi.org/10.5281/zenodo.7709500.
OWASP. 2020. Regular expression Denial of Service - ReDoS. https://owasp.org/www-community/attacks/Regular_
expression_Denial_of_Service_-_ReDoS
Scott Owens, John Reppy, and Aaron Turon. 2009. Regular-expression Derivatives Re-examined. J. Funct. Program. 19, 2
(2009), 173–190. https://doi.org/10.1017/S0956796808007090
Damien Pous. 2015. Symbolic Algorithms for Language Equivalence and Kleene Algebra with Tests. ACM SIGPLAN Notices
– POPL’15 50, 1 (2015), 357–368. https://doi.org/10.1145/2775051.2677007
Olli Saarikivi, Margus Veanes, Tiki Wan, and Eric Xu. 2019. Symbolic Regex Matcher. In Tools and Algorithms for the
Construction and Analysis of Systems (LNCS, Vol. 11427), Tomáš Vojnar and Lijun Zhang (Eds.). Springer, 372–378.
https://doi.org/10.1007/978-3-030-17462-0_24
Henry Spencer. 1994. Software Solutions in C. Academic Press Professional, Inc., San Diego, CA, USA, Chapter A Regularexpression Matcher, 35–71. http://dl.acm.org/citation.cfm?id=156626.184689
Stack Exchange. 2016. Outage Postmortem. http://stackstatus.net/post/147710624694/outage-postmortem-july-20-2016.
Caleb Stanford, Margus Veanes, and Nikolaj Bjørner. 2021. Symbolic Boolean Derivatives for Efficiently Solving Extended
Regular Expression Constraints. In PLDI’21. ACM, 620–635. https://doi.org/10.1145/3453483.3454066
L. J. Stockmeyer and A. R. Meyer. 1973. Word Problems Requiring Exponential Time(Preliminary Report). In Fifth Annual
ACM Symposium on Theory of Computing, STOC’73. ACM, 1–9. https://doi.org/10.1145/800125.804029
Martin Sulzmann and Kenny Zhuo Ming Lu. 2012. Regular Expression Sub-Matching Using Partial Derivatives. In Proceedings
of the 14th Symposium on Principles and Practice of Declarative Programming (PPDP’12). ACM, New York, NY, USA, 79–90.
https://doi.org/10.1145/2370776.2370788
K. Thompson. 1968. Programming techniques: Regular expression search algorithm. Commun. ACM 11, 6 (1968), 419–422.
https://doi.org/10.1145/363347.363387
Lenka Turoňová, Lukáš Holík, Ivan Homoliak, Ondřej Lengál, Margus Veanes, and Tomáš Vojnar. 2022. Counting in
Regexes Considered Harmful: Exposing ReDoS Vulnerability of Nonbacktracking Matchers. In 31st USENIX Security
Symposium (USENIX Security 22). USENIX Association, Boston, MA, 4165–4182. https://www.usenix.org/conference/
usenixsecurity22/presentation/turonova
Lenka Turoňová, Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Margus Veanes, and Tomáš Vojnar. 2020. Regex Matching with
Counting-Set Automata. Proc. ACM Program. Lang. 4, OOPSLA, Article 218 (Nov. 2020). https://doi.org/10.1145/3428286
Ola Wingbrant. 2019. Regular languages, derivatives and finite automata. https://doi.org/10.48550/ARXIV.1907.13577

Received 2022-11-10; accepted 2023-03-31

Proc. ACM Program. Lang., Vol. 7, No. PLDI, Article 148. Publication date: June 2023.
