---
file: ../pdfs/2024-cpp-zhuchko-lean-extended-regex-lookarounds.pdf
citation: "Lean Formalization of Extended Regular Expression Matching with Lookarounds — Zhuchko, Veanes, Ebner — CPP 2024"
doi: 
source: "https://www.microsoft.com/en-us/research/wp-content/uploads/2025/01/cpp24-final.pdf"
topics: [counting_and_derivatives, redos_backrefs_verification]
pages: 14
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Lean Formalization of Extended Regular Expression
Matching with Lookarounds
Ekaterina Zhuchko∗

Tallinn University of Technology
Estonia
ekzhuc@taltech.ee

Margus Veanes

Microsoft Research
USA
margus@microsoft.com

Abstract

Gabriel Ebner

Microsoft Research
USA
gabrielebner@microsoft.com

When regular expressions are deployed in this manner,
essentially as an executable specification language, the trust
in correctness of their supporting theory and of the underlying algorithms becomes critical, perhaps even more so than
perfomance. Formal correctness and analysis of derivative
based theory and algorithms of regular expressions, using
proof assistants, is a fairly recent and active research area including work in analysis of lexing algorithms [3, 8, 24, 26].
One explanation is that proof assistants have matured and
become easier to work with, and derivatives admit an algebraic and inductive view of regular expression semantics,
that lends itself to formal analysis in a way that direct handling of finite automata is less suited for.
A new location based theory of derivatives of regular expressions has recenetly been developed and implemented
as a new backend in the .NET regular expression framework [17], where also anchors are supported and backtracking (PCRE) semantics is maintained. This work has subsequently been extended in [27] to also support lookarounds
as well as intersection and complement while switching the
semantics from PCRE to POSIX because the current understanding of what intersection and complement would mean
in PCRE is unspecified. Since both the theory as well as the
matching algorithm are new, having formal confidence by
a proof assistant is very valuable for any further adaptation
or development of the theory and optimizations in related
algoritms.
In this paper we target the work in [27] and formalize the
core aspects of it in Lean. This work also provides a platform for analysis of [17] because the different variations of
matching semantics can also be represented faithfully with
derivatives and encoded as different rules in Lean, in particular, by omitting commutativity of alternation and selecting
the standard subset of regular expressions with lookarounds.
This work has important practical applications as it builds
confidence in an existing industrial engine in .NET with a
variant that extends it with Boolean operators so far not
available in other engines, with much increased expressivity, allowing to use regular expressions as a full-blown logic,
which is particularly relevant for applications such as CredScan [13].
The main lessons learnt from the formalization are that
choosing the right representation and definitions can give
a lot of insight into the problem that we are defining. For

We present a formalization of a matching algorithm for extended regular expression matching based on locations and
symbolic derivatives which supports intersection, complement and lookarounds and whose implementation mirrors
an extension of the recent .NET NonBacktracking regular
expression engine. The formalization of the algorithm and
its semantics uses the Lean 4 proof assistant. The proof of its
correctness is with respect to standard matching semantics.
CCS Concepts: • Theory of computation → Regular languages; • Computing methodologies → Boolean algebra algorithms.
Keywords: regex, derivative, POSIX
ACM Reference Format:
Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner. 2024. Lean
Formalization of Extended Regular Expression Matching with Lookarounds. In Proceedings of the 13th ACM SIGPLAN International
Conference on Certified Programs and Proofs (CPP ’24), January 15–
16, 2024, London, UK. ACM, New York, NY, USA, 14 pages. https:
//doi.org/10.1145/3636501.3636959

1 Introduction

Regular expressions play a central role in many practical security critical contexts often as a core subset of a domain
specific language for detection, search, filtering, and lexing.
There are many such applications, including in data security for credential scanning, in network intrusion detection,
in web security to prevent cross-site scripting attacks as
rules of traffic filters in most firewalls, in malware scanning,
in email spam filters, etc. Regular expressions are also supported in some SMT solvers as an integrated part of their sequence theory algorithms, and SMT solvers are widely used
by other verification tools.
∗ Work done during Microsoft Research internship.

This work is licensed under a Creative Commons Attribution 4.0 International License.
CPP ’24, January 15–16, 2024, London, UK
© 2024 Copyright held by the owner/author(s).
ACM ISBN 979-8-4007-0488-8/24/01
https://doi.org/10.1145/3636501.3636959

118

<!-- page 2 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

example, defining and working with locations as zippers
and spans allowed a direct and intuitive definition of the
universe of the match semantics. Moreover, we present the
first formalization of an Effective Boolean Algebra which
can represent an interface to SMT solvers. Another insight
is the choice of termination metrics needed in proofs, which
filled some of the gaps in induction arguments in [27]. The
complete Lean formalization of the work presented here is
available in [28].

of 𝑤 from position 𝑖 where 0 ≤ 𝑖 ≤ |𝑤 | of length 𝑛 where
𝑖 + 𝑛 ≤ |𝑤 | is denoted by 𝑤𝑖,𝑛 , e.g., 𝜖0,0 = 𝜖.
Locations. A location describes a position in a string. We
represent a location as pair of words in Loc def
= 𝜎 ∗ × 𝜎 ∗. A
location h𝜖, 𝑢i is initial and a location h𝑢, 𝜖i is final. The re= h𝑣, 𝑢i. Let
verse of a location h𝑢, 𝑣i is the location h𝑢, 𝑣i r def
h𝑢, 𝑎 :: 𝑣i + 1 def
= h𝑎 :: 𝑢, 𝑣i define the next location from any
given nonfinal location and let h𝑢, 𝜖i + 1 def
= h𝑢, 𝜖i. Let also
𝑥 +𝑛 be defined analogously for all 𝑥 ∈ Loc and 𝑛 ≥ 0 where
we let 𝑥 + 0 def
= 𝑥.
Let 𝑤 ∈ 𝜎 ∗ . A location in 𝑤 is a location h𝑢 r, 𝑣i such that
𝑤 = 𝑢 ++ 𝑣. Informally, we write 𝑤 [𝑖] for the location with
|𝑢 | = 𝑖, where 𝑖, 0 ≤ 𝑖 ≤ |𝑤 |, is called the index or position
of the location in 𝑤.
A location is a variant of a list zipper [11] for 𝑤 ∈ 𝜎 ∗ that
facilitates traversing 𝑤 forwards and backwards relative to
an index which is why the first segment 𝑢 r above is in reverse.The representation is also convenient in Lean where
the underlying type of words is List 𝜎 and this representation aligns well with standard list operations of directly
accessing the head and the tail of a list and simplifying the
main operations over locations (and the reversed locations).
In Lean, we represent locations by the following product
type and refer to the left projection as Loc.left and the
right projection as Loc.right.

Outline of the paper. We first introduce basic terminology and notation in Section 2 where we also point out the
close relation between locations and list zippers [11]. In the
following sections we develop the formal theory while also
providing the “informal” definitions in parallel with the formal ones in Lean. In Section 3 we formally define the class
RE that includes lookarounds and all Boolean operators and
we define their semantics in terms of spans that are at some
level related to the value type System.Span<T> for representing contiguous memory regions in .NET [25]. In Sections 4 and 5 we formally develop the theory of derivatives,
prove their corretness with respect to their match semantics,
and prove the correctness of the match algorithm llmatch
introduced in [27] that respects POSIX semantics. In Section 6 we discuss additional rewrites that can be used to
simplify derivatives and we also prove a theorem that shows
that negative lookarounds can be eliminated, which has potential practical applications for further optimizations that
we briefly mention. Section 7 is about related work and Section 8 mentions future work and open problems. Section 9
concludes the paper.

def Loc : List 𝜎 × List 𝜎

Informally, we consider the set of all nonfinal locations
= 𝜎 ∗ × 𝜎 + . A nonfinal location h𝑢 r, 𝑎::𝑣i in 𝑤 = 𝑢𝑎𝑣
Loc+ def
corresponds to the zipper h𝑢 r, 𝑎, 𝑣i, but a location in 𝑤 can
also be final h𝑤 r, 𝜖i. Locations in a word 𝑤 of length 𝑛 = |𝑤 |
are illustrated in Figure 1. For example, 𝜖 has only one loca-

2 Preliminaries

The formalization makes use of concepts like zippers that
are not traditionally used in formal languages. To make it
easier to follow along, we will provide an informal explanation of the arguments involved. In this section, we give
an overview of the basic concepts and how they are represented in Lean. We write lhs def
= rhs to let lhs be equal by definition to rhs. Let Bool = {false, true} denote Boolean values.
We write h𝑥 1, . . . , 𝑥𝑛 i for tuples of elements for 𝑛 ≥ 2 and
let h𝑥 1, . . . , 𝑥𝑛 i.𝑖 def
= 𝑥𝑖 for 1 ≤ 𝑖 ≤ 𝑛.

𝑤0
↑
𝑤 [0]

···

𝑤1
↑
𝑤 [1]

↑
𝑤 [2]

𝑤𝑛−1
↑
𝑤 [𝑛−1]

↑
𝑤 [𝑛]

Figure 1. Locations in a word 𝑤 of length 𝑛.
tion 𝜖 [0] = h𝜖, 𝜖i that is both initial and final. Observe that
the reverse of a location 𝑤 [𝑖] in 𝑤 is the location 𝑤 r[|𝑤 |−𝑖]
in 𝑤 r. For example, the reverse of the final location in 𝑤 is
the initial location in 𝑤 r.
For finite nonempty sets 𝑋 of locations in some word 𝑤,
we informally let max(𝑋 ) (min(𝑋 )) denote the maximum
(minimum) location in the set according to the location order defined by 𝑤 [𝑖] ≤ 𝑤 [ 𝑗] def
= 𝑖 ≤ 𝑗.

Words. Let 𝜎 be an alphabet type, 𝜎 may denote an infinite set. Words or strings over 𝜎 are represented by lists
of elements of type 𝜎, that is denoted by 𝜎 ∗ and in Lean
represented by the type List 𝜎. We let 𝜖 or [] denote the
empty word. The length of a word 𝑤 is denoted by |𝑤 |. For
0 ≤ 𝑖 < |𝑤 | let 𝑤𝑖 ∈ 𝜎 denote the 𝑖’th element of 𝑤. We
denote the reverse of 𝑤 by 𝑤 r, so that 𝑤 r𝑖 = 𝑤 |𝑤 | −1−𝑖 for
0 ≤ 𝑖 < |𝑤 |. Concatenation of 𝑢 ∈ 𝜎 ∗ with 𝑣 ∈ 𝜎 ∗ is formally denoted by 𝑢 ++ 𝑣. We also write 𝑎 :: 𝑣 for prepending
𝑎 ∈ 𝜎 to 𝑣 ∈ 𝜎 ∗ (the cons operation on lists). Informally, we
write the juxtaposition 𝑢𝑣 and 𝑎𝑣 in both cases. The subword

Effective Boolean Algebras. An Effective Boolean Algebra (EBA) over 𝜎 is a tuple A = (𝜎, 𝛼, J_K, ⊥, ⊤, ⊔, ⊓,c ) where
𝛼 is a set of predicates such that ⊥, ⊤ ∈ 𝛼 and 𝛼 is closed under the Boolean connectives [5]. The function J_K : 𝛼 → 2𝜎
is the denotation satisfying J⊥K = ∅, J⊤K = 𝜎, and for all
𝜑,𝜓 ∈ 𝛼, J𝜑 ⊔ 𝜓 K = J𝜑K ∪ J𝜓 K, J𝜑 ⊓ 𝜓 K = J𝜑K ∩ J𝜓 K, and

119

<!-- page 3 -->

Lean Formalization of Extended Regular Expression Matching with Lookarounds

J𝜑 c K = 𝜎 \ J𝜑K. In Lean, we model EBAs as a type class on
the type 𝛼 . The element type 𝜎 is marked as an out-param,
which means that 𝜎 is automatically inferred from 𝛼 .

CPP ’24, January 15–16, 2024, London, UK

def String.characterClass (s : String) : BA Char :=
s.toList |>.map .atom |>.foldr .or .bot

For example the string “0123456789” is converted into a
predicate that is equivalent to 𝜓 [0-9] . Here the type BA Char
corresponds to 𝛼 and Char corresponds to 𝜎 above.

class Denotation (𝛼 : Type u) (𝜎 : outParam (Type v)) where
denote : 𝛼 → 𝜎 → Bool

3 Regexes with Lookarounds

class EffectiveBooleanAlgebra (𝛼 : Type u)
(𝜎 : outParam (Type v)) extends Denotation 𝛼 𝜎,
Bot 𝛼, Top 𝛼, Inf 𝛼, Sup 𝛼, HasCompl 𝛼 where
denote_bot : denote ⊥ c = false
denote_top : denote ⊤ c = true
denote_compl : denote 𝜙 𝑐 c = !denote 𝜙 c
denote_inf : denote (𝜙⊓𝜓 ) c = (denote 𝜙 c && denote 𝜓 c)
denote_sup : denote (𝜙⊔𝜓 ) c = (denote 𝜙 c || denote 𝜓 c)

Here we formally define extended regular expressions with
lookarounds modulo an EBA A = (𝜎, 𝛼, J_K, ⊥, ⊤, ⊔, ⊓,c ). In
examples we use standard (.NET Regex) character classes,
while the actual representation of predicates is irrelevant
and 𝜎 may even be infinite.

For 𝜑 ∈ 𝛼 and 𝑐 ∈ 𝜎 the membership relation (denote 𝜑 𝑐)
returns true iff 𝑐 ∈ J𝜑K and is therefore decidable. We have
not formalized satisfiability of 𝜑 ∈ 𝛼 in Lean here, i.e., deciding J𝜑K ≠ ∅, because we do not use it here. Additionally, for
every type 𝛽 with a denotation function J_K : 𝛽 → 2𝜎 we
define a type BA 𝛽 of Boolean combinations over 𝛽, which
is naturally an EBA.
The EBA abstraction implies that the formalization presented here is generic with respect to any EBA A, although
examples will use A to represent character classes over the
Plane 0 subset of Unicode, which is also of primary interest
here because the character type char in .NET is based on
Uint16. More generally, A can represent an interface to an
SMT solver where 𝜎 is the infinite universe of all models
that the solver can produce, 𝛼 is the set of all well-formed
formulas of the solver, and J𝜑K is an effective enumeration
of all models that satisfy the formula 𝜑. The only operation
needed here is, given 𝑐 ∈ 𝜎 and 𝜑 ∈ 𝛼, to decide if 𝑐 ∈ J𝜑K,
that is typically also denoted by 𝑐 ⊨ 𝜑 in A.

3.1 Regexes
The class RE of regular expressions, or regexes for short, is
here defined by the following abstract grammar. Let 𝜓 ∈ 𝛼
and 𝑅, 𝑅 ′ range over RE.
𝑅

::= 𝜓 | 𝜀 | 𝑅 ⋓ 𝑅 ′ | 𝑅 ⋒ 𝑅 ′ | 𝑅 · 𝑅 ′ | 𝑅* | ~𝑅 | ℓ

ℓ

::= (?=𝑅) | (?<=𝑅) | (?!𝑅) | (?<!𝑅)

where the operators in the first row appear in order of precedence, with alternation ⋓ having lowest and complement ~
having highest precedence. The remaining operators in the
first row are the empty-word regex 𝜀 also denoted by (), intersection ⋒, concatenation ·, and Kleene star *. We use the
standard abbreviation 𝑅+ for 𝑅·𝑅*. Concatenation is often
implicit by using juxtaposition when this is unambiguous.
The regex matching nothing is ⊥ ∈ 𝛼.
The expressions ℓ are called lookarounds: (?=𝑅) is lookahead, (?<=𝑅) is lookbehind, (?!𝑅) is negative lookahead, and
(?<!𝑅) is negative lookbehind, where 𝑅 is the body of the
lookaround, also denoted by body(ℓ). We let LA denote the
set of all lookarounds.
While anchors are not explicitly included in the core definition of RE, they can be defined in terms of lookarounds
as illustrated in Section 6.2 for the start anchor and end anchor. More generally, all standard anchors can be defined
using lookarounds, see [27, Table 1].
In Lean, we represent the syntax of regular expressions as
the following inductive datatype RE. The parameter 𝛼 here
refers to the EBA, theorems about regular expression then
take an [EffectiveBooleanAlgebra 𝛼 𝜎] typeclass argument.

Character classes. In all the examples below we let 𝜎
stand for the standard 16-bit character set of Unicode1 and
use the .NET syntax [14] of regular expression character
classes. For example, [A-Z] stands for all the Latin capital
letters, [0-9] for all the Latin numerals, \d for all the decimal digits, \w for all the word-letters and . for all characters
besides the newline character \n. Any individual character
(as a character class) denotes only that character.
In regular expression examples we use character classes
directly as predicates. For example, 0 ∈ J\dK and 0 ∈ J\wK
and so 0 ∉ J\w ⊓ \dc K. More accurately, a predicate corresponding to a character class 𝐶 is represented by some
predicate 𝜓𝐶 in 𝛼. So, in Lean (denote𝜓 [A-Z] 𝑐) returns true
iff 𝑐 is a Latin capital letter, and 𝜓 [\w-[\d]] is equivalent to
c , i.e., both predicates denote the same elements.
𝜓 \w ⊓ 𝜓 \d
In Lean, we can construct a predicate from a string (denoting the union of all the characters in the string)2 with
the following function:

inductive RE (𝛼 : Type) : Type where
| 𝜀
: RE 𝛼
| Pred (e : 𝛼) : RE 𝛼
| Alternation
: RE → RE → RE
| Intersection
: RE → RE → RE
| Concatenation
: RE → RE → RE
| Star
: RE → RE
| Negation
: RE → RE
| Lookahead
: RE → RE
| Lookbehind
: RE → RE
| NegLookahead
: RE → RE
| NegLookbehind
: RE → RE

1 Also known as Plane 0 or the Basic Multilingual Plane of Unicode.

2 The string is not being “parsed” in any way as a character class of .NET.

120

<!-- page 4 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

In order to present the semantics of the Kleene star operator in Section 3.2, we define the 𝑛-times repetition of 𝑅 for
= 𝑅·𝑅 (𝑛) . In Lean, we de= 𝜀 and 𝑅 (𝑛+1) def
𝑛 ≥ 0, 𝑅 (𝑛) , as 𝑅 (0) def
fine the operation inductively on the number of repetitions:

For a span sp := (s, u, v), we refer to s as sp.left, u as
sp.match and v as sp.right. We also define the operation
to convert a span into a location view.
def Span.loc (sp : Span 𝜎) : List 𝜎 × List 𝜎 :=
hsp.left, sp.match ++ sp.righti

def repeat_cat (R : RE 𝛼) (n : N) : RE 𝛼 :=
match n with
| 0
=> 𝜀
| Nat.succ n => R · (repeat_cat R n)
notation f " ( " n ") " => repeat_cat f n

= hsp.3, sp.2r, sp.1i.
Let sp ∈ Span. The reverse of sp is spr def
r
Thus |sp.2| = |sp .2|. The start location loc, the end location,
the substring match, and the string of sp are:

The empty-word regex is used as base case and the inductive case essentially represents the expansion law for the
Kleene star operator, with concatenation on the left. The
notation command allows us to define arbitrary mixfix syntax r ( n ) .
It is crucial in the formalization to ensure that all procedures terminate, which can be done in Lean using the
termination_by pragma. For this reason, when the inductive step is not trivially terminating, we need to provide for
each theorem a custom metric that can be used to ensure
that the recursion is indeed well-founded.
The size of a regex 𝑅, |𝑅|, is defined recursively as the
number of operators of 𝑅, where |𝜓 | = |𝜀 | def
= 0 and for all
unary operators ♦ and binary operators ⋄, | ♦𝑅| def
= 1 + |𝑅| and
|𝐿 ⋄ 𝑅| def
= 1 + |𝐿| + |𝑅|. For example, |(?=~(𝜀·⊥))| = 3.
Other crucial examples of metrics used in the theorems
are the star height and lookaround height of 𝑅, denoted by
starHeight (𝑅) and lookHeight (𝐻 ), respectively. Each metric
captures the nesting depth of the star and lookarounds of
the regex, and takes the maximum of the nesting depths in
the case of binary operators. For example,
starHeight (𝑅*) def
= starHeight (𝑅) + 1,
starHeight (𝐿 ⋄ 𝑅) def
= max(starHeight (𝐿), starHeight (𝑅)),
starHeight (ℓ) def
= starHeight (body(ℓ)),
and lookaround height is defined analogously by increasing
the nesting only in lookHeight (ℓ) def
= lookHeight (body(ℓ)) +
1, for example, lookHeight ((?=~(𝜀·⊤*)·(?!⊤))) = 2.
In Lean, we let lookaround_height and star_height be
defined analogously, both of which will be used as termination metrics later in the development.

loc(sp)
end (sp)
match(sp)
string(sp)

def

=

hsp.1, sp.2 ++ sp.3i

def

hsp.2r ++ sp.1, sp.3i

=

def

=

sp.2

def

sp.1r ++ sp.2 ++ sp.3

=

Note that end (sp) = loc(spr) r. For all 𝑅 ∈ RE and sp ∈ Span,
the match semantics sp |= 𝑅 is defined by induction over
the regexes, as follows. Note in particular that spans and
locations are used to capture the context conditions of the
semantics of lookarounds.
def
sp |= 𝜀
= |sp.2| = 0
def
sp |= 𝜙
= |sp.2| = 1 ∧ (sp.2)0 ∈ J𝜙K
def
sp |= 𝐿 · 𝑅
= ∃ 𝑠 1, 𝑠 2 : loc(sp) = loc(𝑠 1 ) ∧ 𝑠 1 |= 𝐿 ∧
end (sp) = end (𝑠 2 ) ∧ 𝑠 2 |= 𝑅
sp |= 𝐿 ⋓ 𝑅 def
= sp |= 𝐿 ∨ sp |= 𝑅
sp |= 𝐿 ⋒ 𝑅 def
= sp |= 𝐿 ∧ sp |= 𝑅
def
sp |= ~𝑅
= sp 6 |= 𝑅
def
sp |= 𝑅*
= ∃ 𝑛 : sp |= 𝑅 (𝑛)
def
sp |= (?=𝑅) = |sp.2|=0 ∧ ∃ 𝑠 : loc(𝑠)=loc(sp) ∧ 𝑠 |= 𝑅
sp |= (?!𝑅) def
= |sp.2|=0 ∧  𝑠 : loc(𝑠)=loc(sp) ∧ 𝑠 |= 𝑅
sp |= (?<=𝑅) def
= |sp.2|=0 ∧ ∃ 𝑠 : end (𝑠)=end (sp) ∧ 𝑠 |= 𝑅
sp |= (?<!𝑅) def
= |sp.2|=0 ∧  𝑠 : end (𝑠)=end (sp) ∧ 𝑠 |= 𝑅
Any span sp = h𝑠 r, 𝑢, 𝑣i in 𝑤 also has the more intuitive informal representation 𝑤 [𝑖,𝑛] def
= 𝑤 [𝑖:𝑗] def
= sp, as illustrated
in Figure 2, where 𝑖 = |𝑠 |, 𝑛 = |𝑢 | and 𝑗 = 𝑖 + 𝑛.
𝑠 = 𝑤 0,𝑖
↑
𝑤 [0]

3.2 Match Semantics

𝑢 = 𝑤𝑖,𝑛
↑
𝑤 [𝑖 ]

𝑣 = 𝑤 𝑗,|𝑣 |
↑
𝑤[𝑗 ]

↑
𝑤 [ |𝑤 | ]

Figure 2. Span 𝑤 [𝑖:𝑗] in a word 𝑤.

In this section we introduce the match semantics of regexes,
which will serve as the specification to prove the correctness of our derivative-based approach. The match semantics
corresponds essentially to the classical semantics of regexes
based on languages, but using spans and locations in order
to reason about the matching sections of a word. A span is
defined as Span def
= 𝜎 ∗ ×𝜎 ∗ ×𝜎 ∗ , i.e., a span is a triple of words
that we denote by h𝑠, 𝑢, 𝑣i where 𝑠, 𝑢, 𝑣 ∈ 𝜎 ∗ . Let 𝑤 ∈ 𝜎 ∗ , a
span in 𝑤 is a span sp = h𝑠 r, 𝑢, 𝑣i such that 𝑤 = 𝑠𝑢𝑣, where,
similarly to locations, sp.1 is the reverse of 𝑠.
In Lean, we represent spans by the nested pair of the following type:

Example 3.1. Consider a regex 𝑅 = 𝑅1 ⋒ 𝑅2 ⋒ 𝑅3 ⋒ 𝑅4 with
𝑅1 = (.*[A-Z].*), 𝑅2 = (.*\d.*\d.*), 𝑅3 = (.*[a-z].*),
and 𝑅4 = ((?<=\W).*(?=\W)). 𝑅 is intuitively a specialized “password search pattern” matching a substring containing at least one Latin capital letter, at least two digits, at
least one Latin noncapital letter, and is surrounded by nonword-letters. Let 𝑤 = “0B:1aD2;e” be the input string. Then
𝑤 [3,4] |= 𝑅. Note that, 𝑤 [3,4] = h“:B0”, “1aD2”, “;e”i where
match(𝑤 [3,4]) = 𝑤 3,4 = “1aD2”.
⊠
In Lean, we mirror the match semantics by defining the
match semantics function models, which we represent also

def Span : List 𝜎 × (List 𝜎 × List 𝜎)

121

<!-- page 5 -->

Lean Formalization of Extended Regular Expression Matching with Lookarounds

CPP ’24, January 15–16, 2024, London, UK

In order to prove termination of the models function, we
need a custom metric star_size_metric which is effectively defined as the lexicographic order by jointly using
the star height and size of a regex metrics. The star height
is only needed to make the star constructor case terminate,
while the other metric is used in all other cases. In particular, it is crucial for our definition that starHeight (𝑅 (𝑛) ) <
starHeight (𝑅*).
Note that semantics of lookbehind can be stated either in
terms of regex reversal or span reversal; for example this
is a valid condition for the positive lookbehind case (where
regex reversal is defined below):

def models (sp : Span 𝜎) (R : RE 𝛼) : Prop :=
match R with
| 𝜀
=> sp.match.length = 0
| Pred 𝜙 => sp.match.length = 1
∧ sp.match_head?.any (denote 𝜙)
| l · r =>
∃ u1 u2 , models hsp.left, u1 , u2 ++ sp.righti l
∧ models hu1 𝑟 ++ sp.left, u2 , sp.righti r
∧ u1 ++ u2 = sp.match
| l ⋓ r => models sp l ∨ models sp r
| l ⋒ r => models sp l ∧ models sp r
| ~ r
=> ¬ models sp r
| r *
=> ∃ (m : N), models sp r ( m )
| ?= r
=>
sp.match.length = 0
∧ ∃ (s : Span 𝜎), models s r ∧ s.loc = sp.loc
| ?! r
=>
sp.match.length = 0
∧ ¬ (∃ (s : Span 𝜎), models s r ∧ s.loc = sp.loc)
| ?<= r =>
sp.match.length = 0
∧ ∃ (s : Span 𝜎), models s𝑟 r ∧ s.loc = sp𝑟 .loc
| ?<! r =>
sp.match.length = 0
∧ ¬ (∃ (s : Span 𝜎), models s𝑟 r ∧ s.loc = sp𝑟 .loc)
termination_by models sp R => star_metric R
infix:30 " ⊨ " => models

∃ (s : Span 𝜎), models s (r𝑟 ) ∧ s.loc = sp𝑟 .loc

However, we decided to use the reversal on spans instead,
as using the reversal operation would implicitly rely on the
correctness of the more complicated regex reversal. Use of
span reversal, as a one-line definition, moreover reflects accurately of what lookbehind means intuitively.
3.3 Reversal
Reversal of regexes plays a key role in derivatives of lookbehinds as well as in the top-level match algorithm. The reverse
𝑅 r of 𝑅 ∈ RE is defined as follows:

Figure 3. Formal definition of match semantics in Lean.

𝜓r
𝜀r
(𝐿 · 𝑅) r
(𝑅*) r
(𝐿 ⋓ 𝑅) r
(𝐿 ⋒ 𝑅) r
(~𝑅) r

by the infix operator ⊨ as above. See Figure 3. Note that
end (sp) r = loc(spr), so in the Lean formalization explicit definition of end (sp) is not needed.
Note that, although defined as function, this effectively
represents a matching relation. The main reason why the
models relation ⊨ is not defined as an inductively-defined
predicate is due to the negative occurrence of the relation
that would arise when defining the ~ r constructor. Another possible strategy would have been to define the relation using a positive normal form i.e. for each constructor,
we state what it means to be a match and what it means not
to be a match. However, it would be particularly cumbersome to have to introduce all dual definitions and would result in a considerable number of (mainly artificial) operators.
At the end, we decided to simply adopt a function constructing a Prop in order to adhere to the positivity requirement
in Lean. Inductively-defined predicates have no termination
requirement with respect to the regular expression, and the
star case could have been effectively represented with its
expansion law 𝑅∗ = 𝜀 ⋓ 𝑅 · 𝑅∗. However, due to our definition as a function, we had to come up with an alternative
encoding to represent the star case in order to accomodate
the termination requirement. We instead define the semantics of the star case with the repeat_cat function by stating
that there exists a (finite) number of repetitions of the regex
which matches.

def

=
=
def
=
def
=
def
=
def
=
def
=

def

𝜓
𝜀
𝑅r · 𝐿r
(𝑅 r)*
𝐿r ⋓ 𝑅r
𝐿r ⋒ 𝑅r
~(𝑅 r)

(?=𝑅)r
(?<=𝑅)r
(?!𝑅)r
(?<!𝑅)r

def

= (?<=𝑅 r)
=
(?=𝑅 r)
def
= (?<!𝑅 r)
def
=
(?!𝑅 r)

def

Note how in the lookarounds cases we replace each constructor with its dual in the opposite direction. It follows
by induction over 𝑅 that |𝑅 r| = |𝑅| and that (𝑅 r) r = 𝑅. Theorem 1 is instrumental in linking the derivative based definition (which uses reversal based definition of lookbehinds)
with the formal match semantics.
Theorem 1. ∀𝑅 ∈ RE, sp ∈ Span : sp |= 𝑅 ⇔ spr |= 𝑅 r
Note that, by the reversal theorem and the fact that reversal of regexes is an involutive operation, we can a posteriori
restate the matching semantics of lookbehinds in terms of
regex reversal rather than span reversal.
In order to prove Theorem 1 in Lean it was occasionaly
useful to reason about the equivalence of regular expressions in an equational way using a setoid reasoning-style approach, as it can be idiomatically done in Agda and Idris and
that Lean also supports using the calc abstraction. In order
to achieve this, we define the relation models_equivalence
and show that it is indeed an equivalence relation and a congruence with respect to concatenation.
def models_equivalence : Prop := sp ⊨ r ↔ sp ⊨ q
infixr:30 " ↔𝑟 " => models_equivalence

122

<!-- page 6 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

location 𝑥, der𝑛𝑥 (𝑅), as follows:

theorem equiv_trans (rq : r ↔𝑟 q) (qp : q ↔𝑟 p) : r ↔𝑟 p
theorem equiv_sym (rq : r ↔𝑟 q) : q ↔𝑟 r
theorem equiv_refl : r ↔𝑟 r
theorem equiv_cat_cong (rr : r ↔𝑟 r') (qq : q ↔𝑟 q') :
r · q ↔𝑟 r' · q'

der𝑥0 (𝑅) def
= 𝑅,

def
𝑛
der𝑛+1
𝑥 (𝑅) = der𝑥+1 (der(𝑅, 𝑥))

In the case of a lookahead, nullability null((?=𝑅), 𝑥) tests
existence of a match of 𝑅 starting from location 𝑥, meaning
that there exists 𝑛 ≥ 0 such that der𝑛𝑥 (𝑅) is nullable in the
location 𝑥 +𝑛. For lookbehind, null((?<=𝑅), 𝑥) uses reversal
to test null((?=𝑅 r), 𝑥 r).
Consider (?<=\n) as an example and let 𝑥 = 𝑤 [3] with
𝑤 as above. In this case null((?<=\n), 𝑥) tests if there exists
a derivation of \n starting from 𝑥 r = “edcb\n\na”[4]. The
derivative der(\n, 𝑥 r) = 𝜀 because 𝑤 r4 = \n and 𝜀 is nullable
in all locations, so null((?<=\n), 𝑥) = true.
The main intuition behind derivative based matching of
a regex (?<=𝐿)·𝑅 from a location 𝑥 = 𝑤 [𝑖] is illustrated
in Figure 4 as a state machine with conditional branching,
whose states are regexes with (?<=𝐿)·𝑅 as the initial state
and the state der𝑛𝑥 (𝑅) is conditionally accepting in location
𝑥 + 𝑛 iff it is nullable in 𝑥 + 𝑛.

We make use of the ↔𝑟 relation in the theorems below
which allows us to prove associativity of concatenation, as
well as the left and right identity laws. Moreover, we prove
two useful properties of the repeat_cat function which
will be needed for the star case in the proof of reversal. In
particular, we use this reasoning to prove intermediate lemmas about repeat_cat.
theorem equiv_cat_assoc : ((r · q) · w) ↔𝑟 (r · (q · w))
theorem equiv_eps_cat : 𝜀 · r ↔𝑟 r
theorem equiv_cat_eps : r · 𝜀 ↔𝑟 r
theorem equiv_repeat_cat_cat : r ( m ) · r ↔𝑟 r · r ( m )
theorem equiv_reverse_regexp_repeat_cat {r : RE 𝛼} {m : N}:
(r ( m ) ) 𝑟 ↔𝑟 (r 𝑟 ) ( m )

The following theorem witnesses the correctness of the
reversal operation on regexes, as done in Theorem 1.
theorem models_reversal : sp ⊨ R ↔ sp𝑟 ⊨ R𝑟

(?<=𝐿)·𝑅

The proof follows by induction on R. We found proving this
theorem using the semantic definition more intuitive than
using the derivative-based matching.

𝑥

null((?=𝐿 r), 𝑥 r)

false

⊥

true

4 Location Based Derivatives

der𝑥1 (𝑅)

Here we formally define the framework of location based
derivatives and introduce its Lean representation. The definitions are based on [27] that builds on [17] and is a generalization of [4].
We start with some intuition behind the notion of location based derivatives. Classically, a derivative of a regex 𝑅
is taken for a given character and nullability of 𝑅 is a static
property of 𝑅 that holds iff 𝑅 matches the empty word.
The derivative of a regex 𝑅 ∈ RE for a location 𝑥, denoted
by der(𝑅, 𝑥) ∈ RE, mimics the classical definition, except
that if 𝑅 is a concatenation 𝑅1 ·𝑅2 then nullability of 𝑅1 depends on 𝑥 and all lookarounds ℓ are treated the same as 𝜀
(der(ℓ, 𝑥) = ⊥). In all other aspects, der(𝑅, 𝑥) computes a
derivative in the style of [4] that is then used to continue
matching from the next location.
When lookarounds are used, the notion of nullability becomes location dependent. Nullability of 𝑅 in a location 𝑤 [𝑖],
denoted by null(𝑅, 𝑤 [𝑖]), means in terms of the formal semantics that 𝑤 [𝑖,0] |= 𝑅, i.e., 𝑅 matches the empty word
𝑤𝑖,0 in position 𝑖, which is always true when 𝑅 is either
𝜀 or a Kleene star loop, but may in general depend on 𝑖.
For example, (?=\n) matches an internal end of line, e.g.,
if 𝑤 = “a\n\nbcde” then 𝑤 [𝑖,0] |= (?=\n) only for 𝑖 = 1, 2.
In order to capture the semantics of lookarounds, nullability test of a lookaround recursively invokes existence
of a match that also uses derivatives. To explain this more
clearly, we define, for 𝑛 ≥ 0, the 𝑛’th derivative of 𝑅 from

𝑥+1

der𝑥2 (𝑅)

der𝑛𝑥 (𝑅)

Figure 4. Derivative based matching of (?<=𝐿)·𝑅.
The relationship to the formal semantics follows from Section 5, namely that 𝑤 [𝑖,𝑛] |= (?<=𝐿)·𝑅. The figure is simplified by treating ⊥ as the zero element of · and as the unit
element of ⋓, and where der((?<=𝐿), 𝑥) = ⊥.
4.1 Main Derivative Function
The location derivative der(𝑅, 𝑥) of 𝑅 ∈ RE for a nonfinal
location 𝑥 ∈ Loc+ is defined as follows. Let ℓ ∈ LA, 𝜙 ∈ 𝛼:
der(𝜀, 𝑥) def
= ⊥
der(ℓ, 𝑥) def
= ⊥
{
𝜀, if (𝑥 .2)0 ∈ J𝜙K;
der(𝜙, 𝑥) def
=
{⊥, otherwise.
def der(𝐿, 𝑥)·𝑅 ⋓ der(𝑅, 𝑥), if null(𝐿, 𝑥);
der(𝐿·𝑅, 𝑥) =
der(𝐿, 𝑥)·𝑅,
otherwise.
der(𝐿 ⋓ 𝑅, 𝑥) def
= der(𝐿, 𝑥) ⋓ der(𝑅, 𝑥)
der(𝐿 ⋒ 𝑅, 𝑥) def
= der(𝐿, 𝑥) ⋒ der(𝑅, 𝑥)
der(𝑅*, 𝑥) def
= der(𝑅, 𝑥) · 𝑅*
der(~𝑅, 𝑥) def
= ~der(𝑅, 𝑥)
In Lean, we totalize the location derivative so that it is defined for final locations as well. The only case we need to
adapt is the one for predicates: we set der(𝜙, h𝑢, 𝜖i) def
= ⊥.
The Boolean function null(𝑅, 𝑥) checks if 𝑅 ∈ RE is nullable

123

<!-- page 7 -->

Lean Formalization of Extended Regular Expression Matching with Lookarounds

at the location 𝑥 ∈ Loc, is defined by induction over RE:
null(𝜀, 𝑥)
null(𝜙, 𝑥)
null(𝑅*, 𝑥)
null(𝐿 ⋓ 𝑅, 𝑥)
null(𝐿 ⋒ 𝑅, 𝑥)
null(𝐿 · 𝑅, 𝑥)
null(~𝑅, 𝑥)
null((?=𝑅), 𝑥)
null((?<=𝑅), 𝑥)
null((?!𝑅), 𝑥)
null((?<!𝑅), 𝑥)

def

=
=
def
=
def
=
def
=
def
=
def
=
def
=
def
=
def
=
def
=
def

mutual
def der (R : RE 𝛼) (x : Loc 𝜎) : RE 𝛼 :=
match R with
| 𝜀
=> Pred ⊥
| ?= R
=> Pred ⊥
=> Pred ⊥
| ?<= R
| ?! R
=> Pred ⊥
=> Pred ⊥
| ?<! R
| Pred 𝜙
=>
match x with
| (_ , []) => Pred ⊥
| (_ , c::_) => if denote 𝜙 c then 𝜀 else Pred ⊥
| L · R
=> if null L x then
der L x · R ⋓ der R x
else
der L x · R
| L ⋓ R
=> der L x ⋓ der R x
| L ⋒ R
=> der L x ⋒ der R x
| R *
=> der R x · R *
=> ~ der R x
| ~ R

true
false
true
null(𝐿, 𝑥) ∨ null(𝑅, 𝑥)
null(𝐿, 𝑥) ∧ null(𝑅, 𝑥)
null(𝐿, 𝑥) ∧ null(𝑅, 𝑥)
¬ null(𝑅, 𝑥)
existsMatch(𝑅, 𝑥)
existsMatch(𝑅 r, 𝑥 r)
¬ existsMatch(𝑅, 𝑥)
¬ existsMatch(𝑅 r, 𝑥 r)

Existence of a match in 𝑅 ∈ RE from location 𝑥 ∈ Loc is then
defined as follows:
existsMatch(𝑅, 𝑥) def
=
null(𝑅, 𝑥) ∨ (𝑥 .2 ≠ 𝜖 ∧ existsMatch(der(𝑅, 𝑥), 𝑥 + 1))

def null (R : RE 𝛼) (x : Loc 𝜎) : Bool :=
match R with
| 𝜀
=> true
| Pred 𝜙 => false
| L · R => null L x && null R x
| L ⋓ R => null L x || null R x
| L ⋒ R => null L x && null R x
| R *
=> true
| ~ R
=> ¬ null R x
| ?= R => existsMatch R x
| ?<= R => existsMatch R𝑟 x𝑟
| ?! R => ¬ existsMatch R x
| ?<! R => ¬ existsMatch R𝑟 x𝑟

These are defined in Lean by a mutually recursive definition of null, existsMatch and der. See Figure 5. The mutual dependence arises from the presence of lookarounds.
More specifically, we have to check if there exists a valid
match when performing nullability checks for lookarounds.
Given the interdependence of the definitions due to lookarounds, these mutually inductive definitions need a specific metric to show that the recursion is indeed terminating. The metric is defined as follows, by carefully combining
lexicographically lookaround_height, size of the regex, remaining length of the string to be read, and a flag to prioritize the checking of existsMatch with respect to the other
functions.

def existsMatch (R : RE 𝛼) (x : Loc 𝜎) : Bool :=
match x with
| (u, [])
=>
null R (u, [])
| (u, a::v) =>
null R (u, a::v)
|| existsMatch (der R (u, a::v)) (a::u, v)
end
termination_by
=> der_termination_metric R x 0
null R x
existsMatch R x => der_termination_metric R x 1
der R x
=> der_termination_metric R x 0

def der_termination_metric (r : RE 𝛼) (x : Loc 𝜎) (n : N) :
N × N × N × N :=
(lookaround_height r, sizeOf x.right, sizeOf_RE r, n)

Another crucial aspect for termination that has been omitted from the definition above is the output type of the der
function, which is given as the following subtype:
def der (R : RE 𝛼) (x : Loc 𝜎) :
{r : RE 𝛼 // lookaround_height r ≤ lookaround_height R}

The main idea is to encapsulate the output regex with a
proof that it is structurally smaller than the input one with
respect to one of the metrics used. This trick is used to simultaneously provide a definition for der and show that the
mutual calls are indeed terminating; the property is specifically necessary in the der call in existsMatch to show that
the whole induction terminates.

Figure 5. Formal definition of derivatives in Lean.
Lemma 1 is an adjustment of [27, Theorem 1] to the formalism and notations used here and also treats loops slightly
differently (but equivalently). More concretely, [27] uses finite loops (counters) for the semantics of Kleene star while
we use repetitions here (which simulate counters by iterated concatenation) to simplify the RE type. Note how each
lemma effectively mirrors the matching semantics given in
Section 3.2.

4.2 Derivation Relation
𝑅 𝑦 for 𝑅 ∈ RE and span(𝑥, 𝑦) ∈
The derivation relation 𝑥 −→
Span defines reachability from location 𝑥 to location 𝑦 via
derivatives of 𝑅, where span(𝑥, 𝑦) denotes a span sp such
that 𝑥 = loc(sp) and 𝑦 = end (sp):
𝑅 𝑦
𝑥 −→

def

=

(null(𝑅, 𝑥) ∧ 𝑥 = 𝑦)

∨

𝑥)
(𝑥 .2 ≠ 𝜖 ∧ 𝑥+1 −der(𝑅,
−−−−−−−→ 𝑦)

CPP ’24, January 15–16, 2024, London, UK

Lemma 1. ∀𝐿, 𝑅 ∈ RE,𝜓 ∈ 𝛼, span(𝑥, 𝑦) ∈ Span :
𝜀 𝑦 ⇔𝑥 =𝑦
1. 𝑥 −→
2. 𝑥 −𝜓→ 𝑦 ⇔ (𝑥 .2)0 ∈ J𝜓 K ∧ 𝑦 = 𝑥 + 1
𝑅)
𝑅
3. a. 𝑥 −(?=
−−−−→ 𝑦 ⇔ 𝑥 = 𝑦 ∧ ∃𝑧 : 𝑥 −→ 𝑧

124

<!-- page 8 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

5 Equivalence

𝑅)
𝑅
b. 𝑥 −(?!
−−−−→ 𝑦 ⇔ 𝑥 = 𝑦 ∧ 𝑧 : 𝑥 −→ 𝑧
r
r
𝑅
(?<=
)
4. a. 𝑥 −−−−−−→ 𝑦 ⇔ 𝑥 = 𝑦 ∧ ∃𝑧 : 𝑥 −𝑅−→ 𝑧
r
r
𝑅
(?<!
𝑅
)
b. 𝑥 −−−−−−→ 𝑦 ⇔ 𝑥 = 𝑦 ∧ 𝑧 : 𝑥 −−→ 𝑧
𝑅
𝐿
5. 𝑥 −𝐿⋓𝑅
→ 𝑦 ∨ 𝑥 −→ 𝑦
−−−→ 𝑦 ⇔ 𝑥 −
𝐿 𝑦 ∧𝑥 𝑅 𝑦
6. 𝑥 −𝐿⋒𝑅
𝑦
⇔
𝑥
−−−→
−
→
−→
𝑅 𝑦 ⇔ ¬(𝑥 𝑅 𝑦)
7. 𝑥 −~−→
−→
𝐿
𝑅
8. 𝑥 −𝐿·𝑅
−−→ 𝑦 ⇔ ∃𝑧 : 𝑥 −
→ 𝑧 −→ 𝑦
(𝑛)
𝑅
*
𝑅
9. 𝑥 −−→ 𝑦 ⇔ ∃𝑛 : 𝑥 −−−−→ 𝑦

Our main correctness theorem shows the equivalence between the derivative relation and the match semantics rela𝑅 end (sp).
tion. Let sp ⊢ 𝑅 def
= loc(sp) −→
Theorem 2. ∀𝑅 ∈ RE, sp ∈ Span : sp ⊢ 𝑅 ⇔ sp |= 𝑅
In Lean, we state the theorem accordingly:
theorem correctness : sp ⊢ R ↔ sp ⊨ R

There are two general approaches to proving this theorem.
The first approach is to prove the main correctness by induction on the regular expression R and show that, for each
regex case, the definition of ⊨ induces a property that is also
satisfied by the derivation relation with ⊢. For example, the
theorem statement of derives_Cat looks identical to the
concatenation case in models, but using derives instead of
the matching relation.

In Lean, the derivation relation is encoded by the function derives sp R for which we introduce the infix notation sp ⊢ R. If sp.match is empty, we simply check that R is
also nullable at the current span sp. Otherwise, we continue
moving forward in the string and check that the derivative
of R (with respect to sp.loc) matches.
def derives (sp : Span 𝜎) (R : RE 𝛼) : Bool :=
match sp with
| hs, [], vi
=> null R sp.loc
| hs, c::u, vi => derives hc::s, u, vi (der R sp.loc)
infix:40 " ⊢ " => derives

theorem derives_Cat :
sp ⊢ (l · r) ↔
∃ u1 u2 ,
hsp.left, u1 , u2 ++ sp.righti ⊢ l
∧ hu1 𝑟 ++ sp.left, u2 , sp.righti ⊢ r
∧ u1 ++ u2 = sp.match

Note that the derives function has the advantage of effectively being a decidable relation, whereas the models relation only defines a specification in Prop. Thanks to the
equivalence theorem that we will present in Section 5, this
effectively also provides a decision procedure for the classical match semantics for any given span and regex.
Notationally, given a span sp we use the following functions to refer to the position (i.e., the length of the left part)
of the matching start location loc(sp) and matching end location end (sp):

This is the approach we follow in the formalization, as
it is more modular and allows the main properties of the
matching relation (e.g. derives on a disjunction is indeed
equivalent to the disjunction of the individual derives calls)
to be directly applied to other theorems, which mainly use
the derivation relation.
One alternative way of proving correctness could have
been by induction on the match length, and then show that
both in the nullable and the inductive case the null and der
functions are indeed the correct definitions with respect to
the match semantics relation. The main correctness theorem
then follows directly by induction on the match length and
unfolding the definition of derives. In other words, the correctness would rely on these lemmas:

def Span.i (sp : Span 𝜎) : N := sp.left.length
def Span.j (sp : Span 𝜎) : N := sp.i + sp.match.length

Here, sp.i is the start position and sp.j is the end position of sp.match.
Our implementation choice for span is to use a zipperlike data-stucture. The derives function could have been
alternatively defined on a string with two indices or on two
locations, with one encoding the start of the match and the
second encoding the end of the match. We believe that we
chose the more natural span representation because it intrinsically preserves both the invariant that sp.i ≤ sp.j (i.e.,
the two locations cannot be arbitrarily swapped) and the
fact that the two locations indeed refer to the same string
(i.e., the concatenation of the left and right components of
the two locations coincide). Moreover, the span representation directly captures the matching string and the match
length (sp.j - sp.i), which allows us to apply induction
on the length of the match. In practice this is reflected in the
test for nullability, when sp.i = sp.j and there is a match
at the current position, or alternatively advance sp.i to the
next position and take the derivative of the regex (match
later).

theorem models_to_derives_null :
hs, [], vi ⊨ R ↔ null R hs, vi
theorem models_to_derives_der :
hs, c::u, vi ⊨ R ↔ hc::s, u, vi ⊨ (der R hs, (c::u) ++ vi)

We simply prove these properties using the correctness
obtained with the first approach. Moreover, the following
properties follow from Theorem 2, and Theorem 1 in 5.1(3).
Corollary 5.1. For all 𝑅 ∈ RE and span(𝑥, 𝑦) ∈ Span:
1. null(𝑅, 𝑥) ⇔ span(𝑥, 𝑥) |= 𝑅
2. existsMatch(𝑅, 𝑥) ⇔ ∃𝑦 : span(𝑥, 𝑦) |= 𝑅
𝑅 𝑦 ⇔ 𝑦 r 𝑅r 𝑥 r
3. 𝑥 −→
−−→
5.1 Match Algorithm
After having proven the correctness of the derives relation with respect to the classical match semantics models,
we show how a concrete match finding algorithm based on
derivatives can be extracted and proven correct with respect

125

<!-- page 9 -->

Lean Formalization of Extended Regular Expression Matching with Lookarounds

to the relations previously introduced. One of the main advantage of the algorithm with respect to other approaches
for matching is that it is linear with respect to the string (assuming bounded length lookarounds). The top-level match
algorithm llmatch(𝑅, 𝑤) takes a word 𝑤 ∈ 𝜎 ∗ and a regex
𝑅 ∈ RE, and either returns none if there exists no match for
𝑅 or a concrete span 𝑤 [𝑖:𝑗] |= 𝑅 such that 𝑖 is minimal and 𝑗
is maximal for the given 𝑖, i.e., the match returned is at the
leftmost location with the longest length possible.
We first introduce some auxiliary definitions before presenting the matching algorithm directly in Lean. The function null? tests nullability of a given location with respect
to a regex, either returning the current location (as a span),
or failing otherwise.

CPP ’24, January 15–16, 2024, London, UK

Dually, we also provide a definition of minMatchStart
which, given an end location, finds the minimum starting
location that matches the regex. This is simply obtained by
reversing the inputs (both spans and regexes) and the output
of maxMatchEnd.
def minMatchStart (r : RE 𝛼) (x : Loc 𝜎) : Option(Span 𝜎):=
Option.map Span.reverse (maxMatchEnd r𝑟 x𝑟 )

Despite it being defined in terms of maxMatchEnd, we abstract away the implementation details of minMatchStart
by proving its properties as if it was a separate function, and
we will use it in the top-level algorithm llmatch to find the
leftmost location.
We identify the following correctness and completeness
properties with respect to the maxMatchEnd, which can be
proven by induction on the structure of the input location.
The correctness of minMatchStart can be defined similarly
by dualizing appropriately.

def null? (r : RE 𝛼) (x : Loc 𝜎) : Option (Span 𝜎) :=
if null r x then
some x.as_span
else
none

Theorem 3. For all 𝑅 ∈ RE and 𝑥 ∈ Loc:
{
𝑅 𝑦
none, if 𝑦 : 𝑥 −→
maxMatchEnd(𝑥, 𝑅) =
𝑅 𝑦}, otherwise.
some max{𝑦 | 𝑥 −→

Moreover, the function increase_match_left takes a span
sp and increases the sp.match portion of the span to include the previous character before the start of the match.
This function is used in inductive cases to increase the match
given by a recursive call, and, thanks to the zipper-like definition of a span, it is a constant-time operation.

Note that although we stated correctness in terms of locations, our implementation returns a span, thus encoding
both a start and final location. This is justified by a crucial invariant of the two algorithms, which is that the initial (resp., final) matching location of the match returned by
maxMatchEnd (resp., minMatchEnd) is at the same location of
the one provided as input. This invariant is crucial to prove
the correctness of the top-level algorithm later introduced,
and we state it as follows by first introducing the intuitive
notion of a location loc being the starting match location
of a span sp:

def Span.increase_match_left (sp : Span 𝜎) : Span 𝜎 :=
match sp with
| h[], u, vi => h[], u, vi
| hc::s, u, vi => hs, c::u, vi

The central part of the matching algorithm is the function
maxMatchEnd(𝑥, 𝑅), which finds the maximal (rightmost)
match end location relative to a given start location 𝑥. We
define it as follows, using the Option type in Lean to represent the case in which there is no match at the given start
location.

def derivesStartLocation (loc : Loc 𝜎) (sp2 : Span 𝜎):Prop
loc.pos = sp2.i ∧ loc.string = sp2.string
theorem maxMatchEnd_derivesStartLocation
(matching : maxMatchEnd r x = some sp_out) :
derivesStartLocation x sp_out

def maxMatchEnd (r : RE 𝛼) (x : Loc 𝜎) : Option (Span 𝜎):=
match x with
| h_,[]i => null? r x
| hu,c::vi =>
match maxMatchEnd (der r x) hc::u,vi with
| none => null? r x
| some res => some res.increase_match_left
termination_by
maxMatchEnd r x => x.right

The correctness and completeness can then be defined
with the following statements, with similarly dualized definitions in the case of minMatchStart. The fact that this theorem follows directly by induction on the match length is
what justifies the introduction of the derivation relation and
its equivalence with the match semantics.

The reason why we alternate between locations and spans
in the definition of maxMatchEnd is because the correctness
of maxMatchEnd in Theorem 3 and the top-level algorithm is
stated with respected to derives, which is defined on spans.
Observe that the structure of maxMatchEnd (which produces a concrete match with maximal ending location) is
almost identical to that of existsMatch (which simply says
whether a match exists with the given starting location) and
derives (which only tests whether a given span is indeed a
match).

theorem maxMatchEnd_max
(matching : maxMatchEnd r x = some sp_out) :
(∀ sp, derivesStartLocation x sp
→ sp ⊢ r
→ sp.j ≤ sp_out.j)
theorem maxMatchEnd_correct
(matching : maxMatchEnd r x = some sp_out) : sp_out ⊢ r
theorem maxMatchEnd_no_match
(matching : maxMatchEnd r x = none) :
(∀ sp, derivesStartLocation x sp → ¬(sp ⊢ r))

126

<!-- page 10 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

Proof. Theorem 2 is used implicitly. We have that for all
𝑧 ∈ Loc and all 𝐿 ∈ RE, maxMatchEnd(𝑧, 𝐿) = max{𝑦 |
𝐿 𝑦}. Thus, if 𝑥 = max{𝑦 | 𝑤 r[0] ⊤* ·𝑅 r 𝑦} ≠ none
𝑧 −→
−−−−−→
*
then 𝑥 r = min{𝑦 | 𝑦 −𝑅·⊤
−−−→ 𝑤 [|𝑤 |]} by using basic properties of reversal and Theorem 1. So 𝑥 r = 𝑤 [𝑖] is the leftmost
(minimal) location in 𝑤 because ⊤* matches all spans. Then
maxMatchEnd(𝑤 [𝑖], 𝑅) = 𝑤 [ 𝑗] is such that 𝑗−𝑖 is longest
(maximal). Statement (2) follows.
Statement (1) follows if 𝑥 = none because then no match
end location for ⊤*·𝑅 r exists in 𝑤 r since ⊤* matches all prefixes of 𝑤 r and thus no start location for 𝑅·⊤* exists in 𝑤 by
using Theorem 1, and thus no match for 𝑅 exists in 𝑤. □

5.2 Top-Level Matching Algorithm
Finally, the top-level matching algorithm llmatch takes a
regex R and a word w, and returns the leftmost longest match
in w, that is Lean is defined as follows:
minMatchStart

The algorithm is written in monadic style using do-notation
in the Option monad, which effectively means that the algorithm returns none when either the calls to maxMatchEnd
or the call to minMatchStart fails. Informally the algorithm
can equivalently be described as follows:
llmatch(𝑅,
𝑤) def
= let 𝑥 = maxMatchEnd(𝑤 r[0], ⊤*·𝑅 r) in
{
none,
if 𝑥 = none;
some span(𝑥 r, maxMatchEnd(𝑥 r, 𝑅)), otherwise.

We now show the main theorems on the top-level algorithm as they are proven in Lean. First, we prove that the
match returned by the algorithm is indeed a match with respect to the derivation relation. This follows directly from
the correctness of maxMatchEnd. Note that for simplicity,
we use the derivation relation to prove the properties about
llmatch even though it is equivalent to the match semantics.

We now describe the general idea of the algorithm more in
detail and outline the idea behind its correctness, which we
state in Theorem 4. The idea is to call the auxiliary function
maxMatchEnd twice. First, we determine the leftmost location by calling minMatchStart (and thus, indirectly via reversal, maxMatchEnd). Then, we supply the leftmost location
we found as a start location to maxMatchEnd, which provides
the longest match found from that location.
More in detail, in the first call to minMatchStart we supply as end location the end location of the entire string. In
practice, this is simply obtained by reversing the string w,
and then having no characters to read on the right:

theorem llmatch_matches
(matching : llmatch r x = some sp_out) : sp_out ⊢ r

When a match is found, we show that it is indeed the one
with minimum left position with respect to any other span
which recognizes the same string w.
theorem llmatch_leftmost
(m : llmatch r w = some sp_out) :
(∀ sp, sp.string = w
→ sp ⊢ r
→ sp_out.i ≤ sp.i)

def List.as_end_location (w : List 𝜎) : Loc 𝜎 := hw𝑟 , []i

The use of R · (Pred ⊤)* is then crucial to ensure that
the match length does not play a role in finding the leftmost location, while at the same time ensuring the that the
left position matches the regex R. Indeed, using the boundary invariants previously mentioned, the span returned by
minMatchStart is such that the end match position is always going to be the input one, i.e., the end of the string.
Thus, it is effectively ignored, and this is reflected by the
fact that we simply supply sp.loc to the second call.
The role of the second call is conceptually simpler, and
is used to find the end of the longest match from the leftmost location. This follows directly by the correctness of the
maxMatchEnd stated in Section 5.1. Using again the boundary invariant, the left location of the span returned is still
going to be the leftmost location.
We formally state the correctness and completeness of
the algorithm and subsequently show the theorems as we
formalized them in Lean. The following is the correctness
theorem of llmatch.

Similarly, the match given as output is the longest with
respect to all those spans which recognize the same string
and that start at the same leftmost location.
theorem llmatch_longest
(m : llmatch r w = some sp_out) :
(∀ sp, sp.string = w
→ sp.i = sp_out.i
→ sp ⊢ r
→ sp_out.match.length ≥ sp.match.length)

Finally, we have the completeness property, ensuring that
a match is found whenever there is one:
theorem llmatch_no_match
(m : llmatch r w = none) :
(∀ sp, sp.string = w
→ ¬(sp ⊢ r))

5.3 Executing Matching in Lean
All the definitions in our Lean formalization are computable,
and, except for the match semantics, the predicates take values in Bool instead of Prop. These definitions can therefore
be directly evaluated in Lean [6] as executable code. We take
Example 3.1 and show a test run for it. The character classes
for upper and lower case letters, and digits are approximated
as uc, lc and d, respectively.

Theorem 4. ∀𝑤 ∈ 𝜎 ∗, 𝑅 ∈ RE:
1. llmatch(𝑅, 𝑤) = none ⇔ 𝑖, 𝑗 : 𝑤 [𝑖:𝑗] |= 𝑅;
2. llmatch(𝑅, 𝑤) = 𝑤 [𝑖:𝑗] ⇒
𝑖 = min{𝚤 | ∃𝚥 : 𝑤 [𝚤:𝚥] |= 𝑅} ∧
𝑗 = max{𝚥 | 𝑤 [𝑖:𝚥] |= 𝑅}.

127

<!-- page 11 -->

Lean Formalization of Extended Regular Expression Matching with Lookarounds

CPP ’24, January 15–16, 2024, London, UK

6 Rewrites

def uc := "ABCDEFGHIJKLMOPQRSTUVWXYZ".characterClass
def lc := "abcdefghijklmopqrstuvwxyz".characterClass
def d := "0123456789".characterClass
def us := "_".characterClass
def w := uc ⊔ lc ⊔ d ⊔ us

There are several kinds of optimizations that can be applied
to derivatives. Rewrites can be applied to regexes while preserving their correctness according to Theorem 2. A further
property that we prove below is that negative lookarounds
can be rewritten to positive lookarounds through anchoring.

def Ts [EffectiveBooleanAlgebra 𝛼 𝜎] : RE 𝛼 := (Pred ⊤) *
def W : RE (BA Char) := Pred w𝑐
def R1 : RE (BA Char) := Ts · Pred uc · Ts
def R2 : RE (BA Char) := Ts · Pred d · Ts · Pred d · Ts
def R3 : RE (BA Char) := Ts · Pred lc · Ts
def R4 : RE (BA Char) := (?<= W) · Ts · (?= W)
def R : RE (BA Char) := R1 ⋒ R2 ⋒ R3 ⋒ R4

6.1 Inlined Simplifications
Our current formalization of derivatives in Lean does not
inline rewrites at the level of regular expressions. However,
such rules, analogous to the ones in [19], by building on
Theorem 2, can be added as a separate optimization layer,
in particular for eliminating trivial cases involving 𝜀 as the
unit of · and ⊥ as the zero of · and the unit of ⋓. For example,
if ℓ ∈ LA then the following simplified version of the derivation rule for concatenation can be inlined directly into the
definition, which was used earlier implicitly in Figure 4:

#eval llmatch R "0B:1aD2;e".toList
-- some ([':','B','0'], ['1','a','D','2'], [';','e'])

Note that in the example above, we defined W as a singleton regex matching any non-word-letter. In order to illustrate the difference between c and ~, we define the W', R4 '
variants which use the regex complement ~ rather than c of
the EBA.

der(ℓ·𝑅, 𝑥) = if null(ℓ, 𝑥) then der(𝑅, 𝑥) else ⊥

def W' : RE (BA Char) := ~ Pred w
def R4 ' : RE (BA Char) := (?<= W') · Ts · (?= W')
def R' : RE (BA Char) := R1 ⋒ R2 ⋒ R3 ⋒ R4 '

However, such inlining of simplifications in the general definition of derivatives needs care, because we also intend
to support variants of ⋓ (and ⋒) that are potentially noncommutative (see Section 8). For example, in the case of
backtracking semantics, (𝐿 ⋓ 𝑅) · 𝑆 is backtrack-equivalent
to 𝐿 · 𝑆 ⋓ 𝑅 · 𝑆 but 𝑆 · (𝐿 ⋓ 𝑅) is not backtrack-equivalent
𝑆 · 𝐿 ⋓ 𝑆 · 𝑅, i.e., the distributivity law of concatenation over
alternation applies from right to left but does not apply from
left to right in order to preserve backtracking semantics.

The effect of this change becomes apparent when we execute llmatch with the modified R' and the same input
string, which matches the same string and, somewhat counterintuitively, has different semantics.
#eval llmatch R' "0B:1aD2;e".toList
-- some ([], ['0','B',':','1','a','D','2',';','e'], [])

The key observation is that the negation inside W is still an
atomic predicate, which in particular means that the matching length is always going to be exactly one character. However, complementing a singleton regex allows matches of
any length different from one, hence the different semantics displayed in the two cases. In particular, we have, for
example, that ⊤ matches any singelton string, so the empty
string, as well as any string of two or more characters, is a
match for ~⊤, i.e., ~⊤ is equivalent to 𝜀 ⋓ ⊤·⊤+.
The true power of ~ comes into play when we want to
express a property such as “the match must not contain two
or more digits in a row”, e.g., using the regex S:

6.2 Elimination of General Negative Lookarounds
We define the following regexes, where \A is called the start
anchor and \z is called the end anchor:
\A def
= (?<!⊤)

\z def
= (?!⊤)

In the case of \A there cannot exist a location 𝑤 [𝑖−1] immediately before 𝑤 [𝑖] (or else 𝑤𝑖 −1 ∈ J⊤K = 𝜎), and in the case
of \z there cannot exist a location 𝑤 [𝑖+1] immediately after
𝑤 [𝑖] (or else 𝑤𝑖 ∈ J⊤K = 𝜎). It follows that 𝑤 [𝑖:𝑗] |= \A iff
𝑖 = 𝑗 = 0, and 𝑤 [𝑖:𝑗] |= \z iff 𝑖 = 𝑗 = |𝑤 |. The following basic
property follows from the definition of match semantics.

def S : RE (BA Char) := ~ (Ts · Pred d · Pred d · Ts)

Then, for example,
#eval llmatch R ⋒ S "0B:1aD23;e".toList
-- none

while the original input string still contains a match

Theorem 5. ∀𝑅 ∈ RE :

#eval llmatch R ⋒ S "0B:1aD2;e".toList
-- some ([':','B','0'], ['1','a','D','2'], [';','e'])

1. (?=~(𝑅·⊤*)·\z) ≡ (?!𝑅)
2. (?<=\A·~(⊤*·𝑅)) ≡ (?<!𝑅).

and the modified input string contains a match for R alone

Proof. We prove (1). Let 𝑅 ∈ RE and span(𝑥, 𝑦) ∈ Span. The
proof steps are easier to view by using Theorem 2 although

#eval llmatch R "0B:1aD23;e".toList
-- some ([':','B','0'], ['1','a','D','2','3'], [';','e'])

128

<!-- page 12 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

it follows from the match semantics directly.

The recent work in [27] builds on [17] by extending the
location based derivative theory with both lookarounds as
well as intersection and complement, as denoted here by
the class RE. This work validates experimentally that RE
remains feasible to implement and offers succinctness that
is also theoretically proved by [10], and practical expressivity, that is currently not supported in any state-of-theart regular expression engine but could potentially be supported in future versions of such. The work in [27] abandons
backtracking semantics in favor of leftmost-longest semantics that allows commutativity of ⋓ and ⋒, and implies that
RE becomes an effective Boolean algebra. While the formalization of [27] was the primary focus here, our work here
also provides a platform for the formalization of [17] (see
Section 8).
The original work by [22, 23], introduced a match search
algorithm for lexing via partial derivatives [1] by using the
POSIX [20] semantics. This algorithm was subsequently generalized to Brzozowski derivatives, improved upon by [2, 3]
and formalized in Isabelle/HOL. The work was recently expanded by [26] to allow bounded (finite) loops as well as
character-sets, and uses an alternative definition of POSIX
values together with an equivalence proof with the definition by [18]. The formalization in [3, 26] elegantly demonstrates the power of proof assistants by discovering several
nontrivial gaps in the original correctness argument of [22].
Recently, [24] presented a functional recursive variant of
the [22] algorithm formalized in Isabelle/HOL. A recent derivative based lexing algorithm called Verbatim (Verbatim++)
[7, 8] has been formalized in Coq. Verbatim is related to but
differs from [26] in the way POSIX tokens are processed and
how simplifications are applied to derivatives.
Recall that the leftmost-longest match semantics guaranteed by llmatch(𝑅, 𝑤), as it is formulated in Theorem 4, is
the following. This is our formal interpretation of POSIX semantics in terms of spans, when applied to the class RE:
{
𝑖 = min{𝚤 | ∃𝚥 : 𝑤 [𝚤:𝚥] |= 𝑅} ∧
llmatch(𝑅, 𝑤) = 𝑤 [𝑖:𝑗] ⇒
𝑗 = max{𝚥 | 𝑤 [𝑖:𝚥] |= 𝑅}

(?=~ (𝑅·⊤* ) · \z) 𝑦 ⇔ 𝑥=𝑦 ∧ ∃𝑧 : 𝑥 ~ (𝑅·⊤* ) · \z 𝑧
𝑥 −−−−−−−−−−−−−
→
−−−−−−−−−−→

(𝑅·⊤* ) 𝑧 ′ \z 𝑧
⇔ 𝑥=𝑦 ∧ ∃𝑧, 𝑧 ′ : 𝑥 −~−−−−−−
→
−−→
(𝑅·⊤* ) 𝑤 [|𝑤 |]
⇔ 𝑥=𝑦 ∧ 𝑥 −~−−−−−−
→
*
⇔ 𝑥=𝑦 ∧ ¬(𝑥 −𝑅·⊤
−−−→ 𝑤 [|𝑤 |])

𝑅 𝑧 ⊤* 𝑤 [|𝑤 |]
⇔ 𝑥=𝑦 ∧ 𝑧 : 𝑥 −→
−−→
𝑅 𝑧
⇔ 𝑥=𝑦 ∧ 𝑧 : 𝑥 −→
𝑅)
⇔ 𝑥 −(?!
−−−−→ 𝑦
* 𝑤 [|𝑤 |] is always true because
where we used that 𝑧 −⊤−→
∀sp ∈ Span : sp |= ⊤*. Statement (2) follows by applying
□
Theorem 1 to (1).

Theorem 5 implies that if we add \A and \z as primitive
regexes with the semantics that
h𝑢, 𝑣, 𝑠i |= \A ⇔ 𝑢 = 𝑣 = 𝜖

h𝑢, 𝑣, 𝑠i |= \z ⇔ 𝑣 = 𝑠 = 𝜖

then negative lookarounds are not needed, but can be represented with ~ and anchors using positive lookarounds. We
then also define \Ar def
= \z and \zr def
= \A and since anchors
are primitive, their lookaround height is zero.
There is a practical application of Theorem 5 that already
applies to standard regexes with lookarounds. For example,
a concatenation of any two consecutive lookaheads as in
(?=𝐿)·(?=𝑅) can be combined into an equivalent one lookahead (?=𝐿·⊤*⋒𝑅·⊤*), which means that (?=𝐿)·(?!𝑅) can
also be combined into one lookahead by using Theorem 5.
A single lookahead evaluation is more economical than two
separate evaluations, and the regex 𝐿·⊤* ⋒ 𝑅·⊤* can potentially be further simplified, depending on 𝐿 and 𝑅.

7 Related Work

Brzozowski derivatives were originally studied for derives
by [19] where it is shown experimentally (in Table 1) that,
for the standard fragment of regexes, good rewrite rules
often provide minimal DFAs. An intuitive introduction to
derivatives with a matching algorithm developed in Haskell
also appears in [9].
The first industrial use of a derivative based regex matcher,
as far as we know, is SRM [21], that has been deployed
in the Microsoft credential scanning tool [13]. The work on
SRM has subsequently been extended and modified to support anchors and integrated into .NET under the NonBacKtRacKing flag [17] of RegexOptions, where, unlike SRM,
the matching semantics is compatible with PCRE, i.e., compatible with the other backtracking based backends Compiled and None. The theory in [17] introduces location based
derivatives for supporting anchors, and lookarounds are introduced informally as a generalization of anchors. Lookaheads have also been formalized by [16] with derivatives,
but using a different underlying semantic domain based on
commutative concatenation.

The fundamental difference between llmatch(𝑅, 𝑤) and the
lexing algorithms mentioned above is that in the first pass
llmatch(𝑅, 𝑤) traverses 𝑤 backwards and only in the second
pass forwards. This is not desired for lexing or even applicable to streaming input. Although the work in [27] discusses
informally some implementation techniques used to swap
the passes, those aspects remain unclear to us and fall outside our current formalization.
When comparing the formalization of llmatch(𝑅, 𝑤) with
the formalization in [26], what stands out is that the ordering semantics of POSIX match values is much more intricate
in [26, Fig.2]. We believe that one reason behind this is that
detection of match begin location is more difficult when the
first pass goes forward and the second pass goes backward
from a nullable position as is the case in [26, Fig. 1] – where

129

<!-- page 13 -->

Lean Formalization of Extended Regular Expression Matching with Lookarounds

reversal is not used, and is not directly applicable in the second pass according to [26, p.22]. Due to the extended class
RE, and use of spans, a deeper analysis requires more research.
In prior works, to the best of our knowledge, regex matching with anchors and/or lookarounds have not been formalized in proof assistants. Intersection and complement have
also been of moderate interest because matching engines at
large have so far not supported them, despite the fact that
their derivatives are a very natural extension of standard
regexes [4]. Brzozowski derivatives of regular expressions
extended with intersection and complement have recently
also been formalized in Coq, where handling of complement
required some additional encoding effort, increasing specification verbosity for defining negative propositions dually
to the corresponding positive ones.3

CPP ’24, January 15–16, 2024, London, UK

An outstanding semantic challenge is to combine
these rules in a meaningful way with intersection
and complement in the context of the full RE class
with non-commutative alternation.
We are also working on a finiteness proof in Lean of the
state space 𝑄 /≃ under a weak equivalence relation ≃, with 𝑄
as the set of all reachable regexes from a given initial regex
𝑅 ∈ 𝑄 as the initial state, such that, for all 𝑞 ∈ 𝑄 and 𝑥 ∈ Loc,
der(𝑞, 𝑥) ∈ 𝑄. We believe that the only necessary laws for
≃ are associativity, idempotence, and deduplication of ⋓, the
latter is in [17, Sec.5.3] implicitly present in the key rewrite
rule AltUni. In other words, alternations are maintained
in a canonical right-associative form and without any duplicate elements. This would formally validate finiteness of
the state space required in the input-linear complexity result [17, Theorem 5.3], which does currently not follow directly from [4] due to alternation being non-commutative.
A potential follow-up of finiteness of the state space would
be to prove decidability of nonemptiness of 𝑅 ∈ RE modulo
A, i.e., decidability of ∃ sp ∈ Span : sp |= 𝑅. Observe that,
for all 𝜓 ∈ 𝛼, ∃ sp ∈ Span : sp |= 𝜓 ⇔ ∃ 𝑐 ∈ 𝜎 : 𝑐 ∈ J𝜓 K ⇔
J𝜓 K ≠ ∅ so decidablility of A is a prerequisite in this case,
while not needed in the formalization here, where membership 𝑐 ∈ J𝜓 K is a much simpler decision problem.
Support for captures in the .NET nonbacktracking engine
is perhaps the most difficult and complex aspect of the whole
engine. This part is also not explained or formalized in any
way in [17], other than providing a hint at a relationship to
tagged automata [12]. Formalizing even the core aspect of
the capture semantics and algorithms of the nonbacktracking engine in Lean, based on its open-source implementation [15], is therefore a major undertaking with many challenges.

8 Future Work

We discuss some ongoing and future work items where the
current Lean formalization is used as the starting platform.
A direct application of the Lean formalization is to validate the correctness of the implementation in [27] through
fuzzing, by using the fact that the definition of llmatch in
Lean is executable, which enables comparing match results
for a variety of different classes of regexes and inputs generated for those regexes.
The formalization of derivatives in Lean can also be extended to support backtracking semantics, where a core aspect is that alternation is non-commutative. The following
brief summary is based on [17, Sec.4.2]. The derivative of a
concatenation is there defined as follows

if ¬null(𝐿, 𝑥);

 der(𝐿, 𝑥)·𝑅,

= der(𝑅, 𝑥) ⋓ der(𝐿, 𝑥)·𝑅, else if null!(𝐿, 𝑥);
der(𝐿·𝑅, 𝑥) def

 der(𝐿, 𝑥)·𝑅 ⋓ der(𝑅, 𝑥), otherwise.


9 Conclusions

where 𝑅 has higher priority than 𝐿 in the second case and
where null! prioritizes nullability of the left alternative over
the right one. E.g., while both 𝜀 ⋓ 𝜓 and 𝜓 ⋓ 𝜀 are always
nullable, null!(𝜀 ⋓ 𝜓, 𝑥) = true but null!(𝜓 ⋓ 𝜀, 𝑥) = false.
Intuitively, null!(𝐿, 𝑥) means that, in the location 𝑥, 𝐿 is
equivalent with (𝜀 ⋓ 𝐿) which implies, by left-distributivity
of concatenation over alternation in PCRE, that (𝜀 ⋓ 𝐿)·𝑅
is equivalent to 𝑅 ⋓ 𝐿·𝑅, from which the second case of the
definition above follows.
Formally null!(𝑙 ⋓ 𝑟, 𝑥) def
= null!(𝑙, 𝑥) and null!(𝜀, 𝑥) def
=
true and, for ℓ ∈ A, null!(ℓ, 𝑥) def
= null(ℓ, 𝑥). Observe also
that this extension has no semantic impact when ⋓ is commutative.
Related pruning rules also need to be included into derivative processing that mimic how backtracking chooses a path
by eliminating alternatives that backtracking would forget.

We have formalized the core of a state-of-the-art industrialstrength regular expression engine supporting intersection,
showing that the derivative-based implementation captures
precisely the natural semantics of the extended regular expressions. The formalization was completed by an inexperienced Lean user in two months, resulting in around 1kLOC
of Lean code. This shows that a formalization of the full engine including backtracking semantics is realistically within
reach. The full formalization will not just increase our confidence in the correctness of the existing engine implementation. It will also enable ambitious optimizations that are
too risky to deploy in a production engine without formal
guarantees.

References
[1] Valentin Antimirov. 1996. Partial Derivatives of Regular Expressions
and Finite Automata Constructions. Theoretical Computer Science 155
(1996), 291–319. https://doi.org/10.1007/3-540-59042-0_96

3 Personal communication with Pit-Claudel Clément.

130

<!-- page 14 -->

CPP ’24, January 15–16, 2024, London, UK

Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner

[2] Fahad Ausaf, Roy Dyckhoff, and Christian Urban. 2016. POSIX Lexing
with Derivatives of Regular Expressions. In Archive of Formal Proofs.
http://www.isa-afp.org/entries/Posix-Lexing.shtml
[3] Fahad Ausaf, Roy Dyckhoff, and Christian Urban. 2016. POSIX Lexing
with Derivatives of Regular Expressions (Proof Pearl). In Interactive
Theorem Proving (LNCS, Vol. 9807), Jasmin Christian Blanchette and
Stephan Merz (Eds.). Springer, 69–86. https://doi.org/10.1007/978-3319-43144-4_5
[4] Janusz A. Brzozowski. 1964. Derivatives of regular expressions. JACM
11 (1964), 481–494. https://doi.org/10.1145/321239.321249
[5] Loris D’Antoni and Margus Veanes. 2021. Automata Modulo Theories. Commun. ACM 64, 5 (May 2021), 86–95. https://doi.org/10.1145/
3419404
[6] Gabriel Ebner, Sebastian Ullrich, Jared Roesch, Jeremy Avigad, and
Leonardo de Moura. 2017. A metaprogramming framework for formal verification. Proc. ACM Program. Lang. 1, ICFP (2017), 34:1–34:29.
https://doi.org/10.1145/3110278
[7] Derek Egolf, Sam Lasser, and Kathleen Fisher. 2021. Verbatim: A Verified Lexer Generator. In 2021 IEEE Security and Privacy Workshops
(SPW). 92–100. https://doi.org/10.1109/SPW53761.2021.00022
[8] Derek Egolf, Sam Lasser, and Kathleen Fisher. 2022. Verbatim++:
Verified, Optimized, and Semantically Rich Lexing with Derivatives.
In Proceedings of the 11th ACM SIGPLAN International Conference
on Certified Programs and Proofs (CPP 2022). ACM, 27–39. https:
//doi.org/10.1145/3497775.3503694
[9] Sebastian Fischer, Frank Huch, and Thomas Wilke. 2010. A Play on
Regular Expressions: Functional Pearl. SIGPLAN Not. 45, 9 (2010), 357–
368. https://doi.org/10.1145/1863543.1863594
[10] Wouter Gelade and Frank Neven. 2012. Succinctness of the Complement and Intersection of Regular Expressions. ACM Trans. Comput.
Logic 13, 1, Article 4 (jan 2012), 19 pages. https://doi.org/10.1145/
2071368.2071372
[11] Gérard Huet. 1997. The Zipper. Journal of Functional Programming 7,
5 (1997), 549–554. https://doi.org/10.1017/S0956796897002864
[12] V. Laurikari. 2000. NFAs with tagged transitions, their conversion
to deterministic automata and application to regular expressions. In
7th International Symposium on String Processing and Information Retrieval. 181–187. https://doi.org/10.1109/SPIRE.2000.878194
[13] Microsoft. 2021. CredScan. https://secdevtools.azurewebsites.net/
helpcredscan.html.
[14] Microsoft. 2021.
Regular Expression Language - Quick Referhttps://docs.microsoft.com/en-us/dotnet/standard/baseence.
types/regular-expression-language-quick-reference.
[15] Microsoft. 2022.
.NET Regular Expressions.
https:
//github.com/dotnet/runtime/tree/main/src/libraries/System.Text.
RegularExpressions.
[16] Takayuki Miyazaki and Yasuhiko Minamide. 2019. Derivatives of Regular Expressions with Lookahead. J. Inf. Process. 27 (2019), 422–430.
https://doi.org/10.2197/ipsjjip.27.422

[17] Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi,
Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu. 2023. Derivative Based Nonbacktracking Real-World Regex Matching with
Backtracking Semantics. In PLDI ’23: 44th ACM SIGPLAN International Conference on Programming Language Design and Implementation, Florida, USA, June 17-21, 2023, Nate Foster et al. (Ed.). ACM, 1–2.
https://doi.org/10.1145/3591262
[18] Satoshi Okui and Taro Suzuki. 2010. Disambiguation in Regular Expression Matching via Position Automata with Augmented
Transitions. In Implementation and Application of Automata (LNCS,
Vol. 6482), Michael Domaratzki and Kai Salomaa (Eds.). Springer, 231–
240. https://doi.org/10.1007/978-3-642-18098-9_25
[19] Scott Owens, John Reppy, and Aaron Turon. 2009. Regular-expression
Derivatives Re-examined. J. Funct. Program. 19, 2 (2009), 173–190.
https://doi.org/10.1017/S0956796808007090
[20] POSIX. 2008. IEEE Standard for Information Technology - Portable
Operating System Interface (POSIX(R)). IEEE Std 1003.1-2008 (Revision of IEEE Std 1003.1-2004) (2008), 1–3874. https://doi.org/10.1109/
IEEESTD.2008.4694976
[21] Olli Saarikivi, Margus Veanes, Tiki Wan, and Eric Xu. 2019. Symbolic Regex Matcher. In Tools and Algorithms for the Construction and
Analysis of Systems (LNCS, Vol. 11427), Tomáš Vojnar and Lijun Zhang
(Eds.). Springer, 372–378. https://doi.org/10.1007/978-3-030-174620_24
[22] Martin Sulzmann and Kenny Zhuo Ming Lu. 2012. Regular Expression Sub-Matching Using Partial Derivatives. In Proceedings of the
14th Symposium on Principles and Practice of Declarative Programming
(PPDP’12). ACM, New York, NY, USA, 79–90. https://doi.org/10.1145/
2370776.2370788
[23] Martin Sulzmann and Kenny Zhuo Ming Lu. 2014. POSIX regular
expression parsing with derivatives. In FLOPS 2014 (LNCS, Vol. 8475),
M. Codish and E. Sumii (Eds.). Springer, 203–220. https://doi.org/10.
1007/978-3-319-43144-4_5
[24] Chengsong Tan and Christian Urban. 2023. POSIX Lexing with Bitcoded Derivatives. In 14th International Conference on Interactive Theorem Proving (LIPICS, 26), A. Naumowicz and R. Thiemann (Eds.).
Dagstuhl Publishing, 26:1–26:18. https://doi.org/10.4230/LIPIcs.ITP.
2023.27
[25] Stephen Toub. 2018. All About Span: Exploring a New .NET Mainstay. MSDN Magazine 33, 1 (2018). https://learn.microsoft.com/enus/archive/msdn-magazine/2018/january/csharp-all-about-spanexploring-a-new-net-mainstay
[26] Christian Urban. 2023. POSIX Lexing with Derivatives of Regular
Expressions. Journal of Automated Reasoning 67 (July 2023), 1–24.
https://doi.org/10.1007/s10817-023-09667-1
[27] Ian Erik Varatalu, Margus Veanes, and Juhan Ernits. 2023. Derivative Based Extended Regular Expression Matching Supporting Intersection, Complement and Lookarounds. In arXiv. https://doi.org/10.
48550/arXiv.2309.14401
[28] Ekaterina Zhuchko. 2023. Lean sources for this paper. https://github.
com/ezhuchko/extended-regexes

Received 2023-09-19; accepted 2023-11-25

131
