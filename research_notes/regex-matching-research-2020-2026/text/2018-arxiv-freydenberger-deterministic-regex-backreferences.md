---
file: ../pdfs/2018-arxiv-freydenberger-deterministic-regex-backreferences.pdf
citation: "Deterministic Regular Expressions With Back-References. Freydenberger, Schmid. arXiv 2018 (STACS 2017 / JCSS)"
doi: 
source: "https://arxiv.org/pdf/1802.01508"
topics: [redos_backrefs_verification]
pages: 44
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Deterministic Regular Expressions With Back-References∗
Dominik D. Freydenberger1 and Markus L. Schmid2
1

Loughborough University, United Kingdom, ddfy@ddfy.de
2
University of Trier, Germany, MLSchmid@MLSchmid.de

arXiv:1802.01508v1 [cs.FL] 5 Feb 2018

February 6, 2018

Abstract
Most modern libraries for regular expression matching allow back-references (i. e., repetition operators) that substantially increase expressive power, but also lead to intractability. In order to find a better balance between expressiveness and tractability, we combine
these with the notion of determinism for regular expressions used in XML DTDs and XML
Schema. This includes the definition of a suitable automaton model, and a generalization
of the Glushkov construction. We demonstrate that, compared to their non-deterministic
superclass, these deterministic regular expressions with back-references have desirable algorithmic properties (i. e., efficiently solvable membership problem and some decidable problems in static analysis), while, at the same time, their expressive power exceeds that of
deterministic regular expressions without back-references.

1

Introduction

Regular expressions were introduced in 1956 by Kleene [34] and quickly found wide use in
both theoretical and applied computer science, including applications in bioinformatics [41],
programming languages [49], model checking [48], and XML schema languages [47]. While the
theoretical interpretation of regular expressions remains mostly unchanged (as expressions that
describe exactly the class of regular languages), modern applications use variants that vary
greatly in expressive power and algorithmic properties. This paper tries to find common ground
between two of these variants with opposing approaches to the balance between expressive power
and tractability.
REGEX
The first variant that we consider are regex, regular expressions that are extended with a backreference operator. This operator is used in almost all modern programming languages (like e. g.
Java, PERL, and .NET). For example, the regex hx : (a ∨ b)∗ i · &x defines {ww | w ∈ {a, b}∗},
as (a ∨ b)∗ can create a w ∈ {a, b}∗ , which is then stored in the variable x and repeated with the
reference &x. Hence, back-references allow to define non-regular languages; but with the side
effect that the membership problem is NP-complete (cf. Aho [2]).
Regex were first examined from a theoretical point of view by Aho [2], but without fully
defining the semantics. There were various proposals for semantics, of which we mention the
first by Câmpeanu, Salomaa, Yu [10], and the recent one by Schmid [46], which is the basis for this
paper. Apart from defining the semantics, there was work on the expressive power [10, 11, 25],
the static analysis [11, 23, 24], and the tractability of the membership problem (investigated in
terms of a strongly restricted subclass of regex) [21, 22]. They have also been compared to related
models in database theory, e. g. graph databases [4, 26] and information extraction [20, 24].
∗ This
work represents an extended version
pressions
with
Back-References”
presented
at
(http://dx.doi.org/10.4230/LIPIcs.STACS.2017.33 ).

1

of the
STACS

paper “Deterministic
2017
and
published

Regular Exin
LIPICS

<!-- page 2 -->

Deterministic Regular Expressions
The second variant, deterministic regular expressions (also known as 1-unambiguous regular
expressions), uses an opposite approach, and achieves a more efficient membership problem
than regular expressions by defining only a strict subclass of the regular languages.
Intuitively, a regular expression is deterministic if, when matching a word from left to right
with no lookahead, it is always clear where in the expression the next symbol must be matched.
This property has a characterization via the Glushkov construction that converts every regular
expression α into a (potentially non-deterministic) finite automaton M(α), by treating each terminal position in α as a state. Then α is deterministic if M(α) is deterministic. As a consequence,
the membership problem for deterministic regular expressions can be solved more efficiently than
for regular expressions in general (more details can be found in [31]). Hence, in spite of their
limited expressive power, deterministic regular expressions are used in actual applications: Originally defined for the ISO standard for SGML (see Brüggemann-Klein and Wood [9]), they are a
central part of the W3C recommendations on XML DTDs [7] and XML Schema [27] (see Murata
et al. [42]). Following the original paper [9], deterministic regular expressions have been studied
extensively. Aspects include computing the Glushkov automaton and deciding the membership
problem (e. g. [8, 31, 44]), static analysis (cf. [40]), deciding whether a regular language is deterministic (e. g. [16, 31, 39]), closure properties and descriptional complexity [37], and learning
(e. g. [5]). One noteworthy extension are counter operators (e. g. [29, 31, 36]), which we briefly
address in Section 8.
Deterministic REGEX
The goal of this paper is finding common ground between these two variants, by combining the
capability of backreferences with the concept of determinism in regular expressions. Generally,
our definition of determinism for regex mimics that for classical regular expressions, i. e., we
define a Glushkov-like conversion from regex into suitable automata and then say that a regex
is deterministic if and only if its Glushkov-automaton is. The thus defined class of deterministic
regex is denoted by DRX, and L(DRX) refers to the corresponding language class.
The underlying automaton model for this approach is a slight modification of the memory
automata (MFA) proposed by Schmid [46] as a characterisation for the class of regex-languages.
More precisely, we introduce memory automata with trap-state (TMFA), for which the deterministic variant, the DTMFA, is better suited for complementation than the deterministic MFA.
As indicated by the title of this subsection, it is our hope to preserve, on the one hand, the
increased expressive power provided by backreferences, and, on the other hand, the tractability
that usually comes with determinism. While it is not surprising that DRX do not achieve these
goals to the full extent, a comprehensive study reveals that their expressive power clearly exceeds
that of (deterministic) regular expressions, while, at the same time, being much more tractable
than the full class of regex.
We shall now outline our main results according to these two aspects, and we start with the
algorithmic features of DRX:
1. We can decide in time O(|Σ||α|k), whether a regex α with k variables and over alphabet
Σ is deterministic (if so, we can construct its Glushkov-automaton in the same time).
2. We can decide in time O(|Σ||α|n + k|w|), whether w ∈ Σ∗ can be generated by an α ∈ DRX
with k variables and n occurrences of terminal symbols or variable references.
3. The intersection-emptiness problem for DRX is undecidable, but in PSPACE for variablestar-free1 DRX (as well as the inclusion and equivalence problem).
Results 1 and 2 are a consequence of the Glushkov-construction for regex. In view of the
NP-hardness of the membership problem for the full class of regex, result 2 demonstrates that
the membership problem for DRX can be solved almost as efficiently as for deterministic regular
expression (which is possible in time O(|Σ||α| + |w|) [8, 44] or O(|α| + |w| · log log |α|) [31]). The
positive results of 3 are based on encoding the intersection-emptiness problem in the existential
1 A regex is variable-star-free if each of its sub-regexes under a Kleene-star contains no variable operations (see

Section 6).

2

<!-- page 3 -->

theory of concatenation with regular constraints. With respect to 3, we observe that it is in
fact the determinism, which makes the inclusion and equivalence problem for variable-star-free
DRX decidable, since these problems are known to be undecidable for non-deterministic variablestar-free regex (see [23]). Moreover, result 3 also yields a PSPACE minimization algorithm for
variable-star-free DRX (enumerate all smaller candidates and check equivalence).
Throughout the paper, there are numerous examples that demonstrate the expressive power
of DRX. We also provide a tool for proving non-expressibility, which does not use a pumping
argument. In fact, it can be shown that, despite the automata-theoretic characterisation of
deterministic regex, L(DRX) contains infinite languages that cannot be pumped (in the sense as
regular languages are “pumpable”). In addition, we show the following results with respect to
DRX’s expressiveness:
1. There are regular languages that are not in L(DRX).
2. L(DRX) contains regular languages that are not deterministic regular.
3. L(DRX) contains all unary regular languages.
4. L(DRX) is not closed under union, concatenation, reversal, complement, homomorphism,
inverse homomorphism, intersection, and intersection with deterministic regular languages.
While result 1 fits to the situation that there are also regular languages that are not deterministic regular languages (and, thus, points out that our definition of determinism restricts
regex in a similar way as classical regular expressions), result 2 points out that in the case of
regex this restriction is not as strong. With respect to result 3, note that not all unary regular languages are deterministic regular (see [37]). From a technical point of view, it is worth
mentioning that in some of our proofs, we use subtleties of the back-reference operator in novel
ways. Intuitively speaking, defining and referencing variables under a Kleene-star allows for
shifting around factors between different variables (even arbitrarily often in loops), which makes
it possible to abuse variables for generating seemingly non-deterministic regular structures in a
deterministic way, instead of generating non-regular structures.
As a last strong point in favour of our definition of determinism for regex, we examine a
natural relation of the definition of determinism (i. e., requiring determinism only with respect
to a constant look-ahead). We prove that checking whether a regex is deterministic under this
more general notion is intractable (even for the class of variable-star-free regex).
Summing up, from the perspective of deterministic regular expressions, we propose a natural extension that significantly increases the expressive power, while still having a tractable
membership problem. From a regex point of view, we restrict regex to their deterministic core,
thus obtaining a tractable subclass. Hence, the authors intend this paper as a starting point for
further work, as it opens a new direction on research into making regex tractable.
Structure of the paper
In Section 2, we define some basic concepts and the syntax and semantics of regex; in addition,
due to the fact that this aspect is often neglected in the existing literature, which caused misunderstandings, we also provide a thorough discussion of existing variants of regex in theory
and practice. Section 3 is devoted to the definition of memory automata with trap state and
their deterministic subclass. In addition, we provide an extensive automata-theoretic toolbox in
this section that, besides showing interesting facts about TMFA, shall play an important role for
our further results. Next, in Section 4, we define deterministic regex and provide the respective
Glushkov-construction. The expressive power of DRX (and of related classes resulting from different variants of TMFA) is investigated in Section 5 and the decidability and hardness results of
the static analysis of DRX are provided in Section 6. Finally, in Section 7, we discuss the above
mentioned relaxation of determinism, and we conclude the paper by giving some conclusions in
Section 8.

3

<!-- page 4 -->

2

Preliminaries

We use ε to denote the empty word. The subset and proper subset relation are denoted by ⊆ and
⊂, respectively. Let Σ be a finite terminal alphabet (unless otherwise noted, we assume |Σ| ≥ 2)
and let Ξ be an infinite variable alphabet with Ξ ∩ Σ = ∅. For a word w ∈ Σ∗ and for every i,
1 ≤ i ≤ |w|, w[i] denotes the symbol at position i of w. We define w0 : = ε and wi+1 : = wi · w
i
for all i ≥ 0, and, for w = a1 · · · an with ai ∈ Σ, let wm+ n = wm · a1 · · · ai for all m ≥ 0 and all
i with 0 ≤ i ≤ n. A v ∈ Σ∗ is a factor of w if there exist u1 , u2 ∈ Σ∗ with w = u1 vu2 . If u2 = ε,
v is also a prefix of w.
We use the notions of deterministic and non-deterministic finite automata (DFA and NFA)
like [32]. If an NFA can have ε-transitions, we call it an ε-NFA. Given a class C of language
description mechanisms (e. g., a class of automata or regular expressions), we use L(C) to denote
the class of all languages L(C) with C ∈ C. The membership problem for C is defined as follows:
Given a C ∈ C and a w ∈ Σ∗ , is w ∈ L(C)?
Next, we define the syntax and semantics of regular expressions with backreferences.
Definition 1 (Syntax of regex). The set RX of regex over Σ and Ξ is recursively defined as
follows:
Terminals and ε: a ∈ RX and var(a) = ∅ for every a ∈ (Σ ∪ {ε}).
Variable reference: &x ∈ RX and var(&x) = {x} for every x ∈ Ξ.
Concatenation: (α · β) ∈ RX and var(α · β) = var(α) ∪ var(β) if α, β ∈ RX.
Disjunction: (α ∨ β) ∈ RX and var(α ∨ β) = var(α) ∪ var(β) if α, β ∈ RX.
Kleene plus: (α+ ) ∈ RX and var(α+ ) = var(α) if α ∈ RX.
Variable binding:hx : αi ∈ RX and var(hx : αi) = var(α) ∪ {x} if α ∈ RX with x ∈ Ξ \ var(α).
In addition, we allow ∅ as a regex (with var(∅) = ∅), but we do not allow ∅ to occur in any other
regex. An α ∈ RX with var(α) = ∅ is called a proper regular expression, or just regular expression. We use REG to denote the set of all regular expressions. We add and omit parentheses
freely, as long as the meaning
remains clear. We use the Kleene star α∗ as shorthand for ε ∨ α+ ,
W
and A as shorthand for a∈A a for non-empty A ⊆ Σ.

We define the semantics of regex using the ref-words (short for reference words) by Schmid [46].
A ref-word is a word over (Σ ∪ Ξ ∪ Γ), where Γ : ={[x , ]x , | x ∈ Ξ}. Intuitively, the symbols [x
and ]x mark the beginning and the end of the match that is stored in the variable x, while an
occurrence of x represents a reference to that variable. Instead of defining the language of a
regex α directly, we first treat α as a generator of ref-words by defining its ref-language R(α) as
follows.
• For every α ∈ Σ ∪ {ε}, R(α) : ={α}.
• For every x ∈ Ξ, α ∈ RX,
– R(&x) : ={x},
– R(hx : αi) : =([x R(α)]x ).
• For every α, β ∈ RX,
– R(α · β) : = R(α) · R(β),
– R(α ∨ β) : = R(α) ∪ R(β), and
– R(α+ ) : = R(α)+ .

In particular, if α is a regular expression, then L(α) = R(α). An alternative definition of the
ref-language would be R(α) : = L(αR ), where αR is the proper regular expression obtained from
α by replacing each sub-regex hx : βi by [x βR ]x , and each &x by x.
Intuitively speaking, every occurrence of a variable x in some r ∈ R(α) functions as a pointer
to the next factor [x v]x to the left of this occurrence (or to ε if no such factor exists). In this
way, a ref-word r compresses a word over Σ, the so-called dereference D(r) of r, which can be
obtained by replacing every variable occurrence x by the corresponding factor v (note that v
might again contain variable occurrences, which need to be replaced as well), and removing all
4

<!-- page 5 -->

symbols [x , ]x ∈ Γ afterwards. See [46] for a more detailed definition, or the following Example 1
for an illustration. Finally, the language of a regex α is defined by L(α) : ={D(r) | r ∈ R(α)}.
+
Example 1. Let α : = hx : (a ∨ b)+ i&x . Then
R(α) = {[x w1 ]x · x · · · [x wn ]x · x | n ≥ 1, wi ∈ {a, b}+} ,

or, equivalently, L(α) = (Lcopy )+ , with Lcopy : ={ww | w ∈ {a, b}+}.
∗
An interesting example is the regex αsq : = hx : &yihy : &x · ai with ref-language R(αsq ) =
i
{ri | i ≥ 0}, where ri : = [x y]x · [y x · a]y . For example, for
r3 = [x y]x · [y x · a]y · [x y]x · [y x · a]y · [x y]x · [y x · a]y ,

2

we have r3 ∈ R(αsq ) with D(r3 ) = a9 . Using induction, we can verify that D(ri ) = ai . Thus,
2
L(αsq ) = {an | n ≥ 0}.
Hence, unlike regular expressions, regex can define non-regular languages. The expressive
power comes at a price: their membership problem is NP-complete (follows from Angluin [3]), and
various other problems are undecidable (Freydenberger [23]). Starting with Aho [2], there have
been various approaches to specifying syntax and semantics of regex. While [2] only sketched
the intuition behind the semantics, the first formal definition (using parse trees) was proposed
by Câmpeanu, Salomaa, Yu [10], followed by the ref-words of Schmid [46]. In the following, we
provide a more detailed discussion of the different approaches and actual implementations of
regex.

2.1

Regex in Theory and Practice

In this section, we motivate the choice of the formalization of regex syntax and semantics that
are used in the current paper, in particular in comparison to [10], and then connect these to
the use of back-references in actual implementations. Note that in order to explain how our
results and concepts can be adapted to various alternative definitions of syntax and semantics,
we anticipate some of the technical content of our paper.
2.1.1

Choices behind the definition

We begin with a discussion of semantics of back-references, which most actual implementations
define in terms of the used matching algorithm2 . For a theoretical analysis, this approach is not
satisfactory.
To the authors’ knowledge, the first theoretical analysis of regular expressions is due to
Aho [2], who defined the semantics informally. Câmpeanu, Salomaa, Yu [10] then proposed a
definition using parse trees, which was precise, but rather technical and unwieldy. Schmid [46]
then introduced the definition with ref-words that we use in the current paper. The two definitions differ only in some semantical particularities, which we discuss further down.
The most obvious difference in approaches to syntax is that some formalizations, like [10], do
not use variables, but numbered back-references. For example, hx : a∗ ib · &x would be written
as (a∗ )b\1, where \1 refers to the content of the first pair of parentheses (called the first group).
After working with this definition for some time, the authors of the present paper came to
the conclusion that using numbered back-references instead of named variables is inconvenient
(both when reading and writing regex). The developers of actual implementations seem to agree
with this sentiment: While using numbered back-references was well-motivated when considering
PERL at the time [10] was published, most current regex dialects allow the use of named groups,
which basically act like our variables (depending on the actual dialect, see below). The choice
between variables and numbered groups is independent of the choice of semantics, as parse trees
can also be used with variables, see [23]. Hence, using variables instead of numbers is a natural
choice.
2 From a theory point of view, this might be considered a rather generous use of the term “define”.

5

<!-- page 6 -->

Building on this, the next question is whether the same variable can be bound at different
places in the regex (which is automatically excluded by the use of numbered groups as in [10]),
i. e., whether one allows expressions like


hx : a+ ihy : b+ i ∨ hy : b+ ihx : a+ i c · &x · &y.

While some implementations that have developed from back-references forbid these constructions to certain degrees (see Section 2.1.2 below), there seems to be no particular reason for this
decision when approaching this question without this historical baggage. In fact, one can argue
from a point of applications that expressions like the following make sense (abstracting away
some details that would be needed in actual use):



Σ∗ Name:hx : Σ+ i Title:hy : Σ+ i ∨ Title:hy : Σ+ i Name:hx : Σ+ i Σ∗

In fact, these constructions are explicitly allowed in the regex formulas of Fagin et al. [20], that
are closely related to regex. In particular, both the semantic definitions (ref-words and parse
trees) allow this choice. Thus, there seems to be no particular practical reason to disallow these
constructions when considering only the model (instead of its algorithmic properties).
In addition to disallowing the repeated binding of the same variable described above, the
regex definition in [10] also includes a syntactic restriction that changes the expressive power
considerably: It requires that a backreference \n can only appear in a regex if it occurs to the
right of corresponding group number n. In [10], otherwise, the expression is called a “semi∗
regex”. Consider αsq = hx : &yihy : &x · ai from Example 1. In the numbered notation of [10],
∗
this would be expressed as β : = (2 \3)2 (3 \2 · a)3 , when adding group numbers to the groups
to increase readability. But using definitions from [10], β is only a semi-regex, as the reference
\3 occurs to the left of group 3.
The motivation behind this restriction is not explained in [10]. While one might argue that
this was chosen to avoid referencing unused groups, the definition of semantics in [10] still needs
to deal with this problem in regexes like ((2 a)2 ∨(3 a)3 ) · \2 · \3, and handles them by assigning ε
(like the definition from [46], which we use as well). Hence, even on “semi-regex”, the parse tree
semantics behave like the ref-word semantics.
Arguably, the restriction has an advantage from a theoretical point of view, as it allows
Câmpeanu, Salomaa, Yu [10] and Carle and Narendran [11] to define pumping lemmas for
this class. Using these, it is possible to show that languages like L(αsq ) from Example 1 or
the language from Lemma 3 cannot be expressed with the regex model from [10]. But in other
areas, there seems to be no advantage in this choice: Even under this restriction, the membership
problem is NP-complete (since it is still possible to describe Angluin’s pattern languages [3]),
the undecidability results from [23] on various problems of static analysis are unaffected by this
choice, and even the proof of Theorem 9 (the undecidability of the disjointness problem for
deterministic RX) directly works on this subclass. In summary, the authors of the current paper
see no reason to adapt this restriction.
For full disclosure, the second author points out that in his own articles [45, 46], using [10]
as reference for a full definition of regex is not entirely correct, since the restrictions of [10]
discussed above are not used in these papers; instead they talk about the language class L(RX)
of the current paper.
The last choice in the definition that we need to address is how we deal with referencing
undefined variables. Both [10] and [46] default those references to ε (as do others, like [23]); but
there is also literature, like [11], that uses ∅ as default value (under these semantics, a ref-word
that contains a variable that de-references to ∅ cannot generate any terminal words; the same
holds for a parse tree that contains such a reference). This choice can easily be implemented
in both semantics by discarding a ref-word or parse tree that contains such a reference; and a
TMFA (see Section 3) can reject if a run encounters a reference to such an undefined memory.
While these “∅-semantics” are also used in some actual implementations, the authors of the
current paper are against this approach. One of the reasons is that using ∅ as default allows the
use of curious synchronization effects that distract from the main message of this paper. For

6

<!-- page 7 -->

example, let Σ = {a1 , . . . , an } for some n ≥ 1, and define
αn : =

n
_

n
(ai · hxi : εi) · &x1 · · · &xn .

i=1

If unbound variables default to ∅, this regex generates the language

aπ(1) · · · aπ(n) | π is a permutation of {1, . . . , n} ,

as every variable xi needs to be assigned ε exactly once (otherwise, a reference would return ∅
and block). Hence, using this semantics, even variables that are bound only to ε can be used
for synchronization effects. While this can lead to interesting constructions, the authors think
that it provides more insight to study the effects of back-references on lower bounds without
relying on these additional features. This way, there is no question whether the hardness of the
examined problems is due to the effects of the ∅-semantics.
Furthermore, all examples in the present paper can be adapted from the used ε-semantics to
∅-semantics: Given an α ∈ RX with the variables x1 , . . . , xk , define α′ : =hx1 : εi · · · hxk : εi · α.
First, we observe that the language that is defined by α′ under ∅-semantics is the same language
that α defines under ε-semantics. Furthermore, note that if α is deterministic (in the sense as
shall be defined in Section 4), α′ is also deterministic. The analogous construction can be used
for TMFA (and DTMFA).
While it is possible to adapt most of the results in the current paper directly to this alternative
semantics, the authors chose to keep the paper focused on ε-semantics.
2.1.2

Actual implementations

We now give a brief overview of how back-references are used in some actual implementations. For
a good introduction on various dialects, the authors recommend [30], in particular the section on
back-references and named groups. As this behavior is often under-defined, badly documented,
and implementation dependent, this can only be a very short and superficial summary of some
behavior.
Before we go into details, we address why back-references are used, in spite of the resulting NP-hard membership problem: Most regex libraries use a backtracking algorithm that can
have exponential running time, even on many proper regular expressions (see Cox [15]). From
this point of view, back-references can be added with little implementation effort and without
changing the efficiency of the program.
Most modern dialects of regex not only support numbered back-references as used by [10],
but also named capture groups, which work like our variables. In some dialects like e. g. Python,
PERL, and PCRE, these act as aliases for back-references with numbers; hence, hx : a∗ ib&x
would be interpreted as (a∗ )b\1. As a consequence, each name resolves to a well-defined number.
As some of these dialects assign the empty set as default value of unbound back-references (or
group names), the resulting behavior is similar implicitly requiring the restriction from [10].
This implementation of named capture groups seems to be mostly for historical reasons (as
back-references were introduced earlier).
In contrast to this, there are other dialects that use numbered back-references and explicitly
allow references to access groups that occur to their right in the expression. For example,
the W3C recommendation for XPath and XQuery functions and operators [33] defines regular
expressions with back-references for the use in fn:matches. There, it is possible to refer to
capture groups that occur to the right of the reference (although only for the capture groups
1 to 9, but not for 10 to 99, which might be considered a peculiar decision). As this dialect
defaults unbound references to ε, it is possible to directly express αsq by renaming the variable
references to back-references.
Furthermore, .NET allows the same name to be used for different groups, for example
((hx : a+ i ∨hx : b+ ic)&x)∗ . While .NET defaults unset variables to ∅, it is possible to express
L(αsq ), by using an expression like hx : εi · αsq . In the same way, every regex in the sense of our
paper can be converted into an equivalent .NET regex.
Finally, in 2007 (just four years after the publication of [10]), PERL 5.10 introduced branch
reset groups (which were also adapted in PCRE). These reset the numbering inside disjunctions,
7

<!-- page 8 -->

and allow expressions that behave like the expression ((hx : a+ i ∨hx : b+ ic)&x)∗ . This allows
PERL regex to replicate a large part of the behavior of .NET regex.
In conclusion, it seems that almost every formalization of regex syntax and semantics can be
justified by finding the right dialect; but every restriction might be superseded by the continual
evolution of regex dialects. Hence, the current paper attempts to avoid restrictions; and when in
doubt, we choose natural definitions over trying to directly emulate a single dialect. Therefore,
we use variables instead of numbered back-references, and allow multiple uses of the same variable
name.
The authors acknowledge that most actual implementations of “regular expressions” allow
additional operators. Common features are counters, which allow constructions like e. g. a2,5
that define the language {ai | i ∈ {2, . . . , 5}}, character classes and ranges, which are shortcuts
for sets like “all alphanumeric symbols” or “all letters from b to y”, and look ahead and look
behind, which can be understood as allowing the expression to call another expressions as a kind
of subroutine.
While these operators are outside of the scope of the current paper, we briefly address the issue
of counters. These are used in XML DTDs and XML Schema, and were studied in connection
to determinism. In particular, Gelade, Gyssens, Martens [29] described how counters can be
added to finite automata and proposed an appropriate extension of determinism and Glushkov
construction to this model. Although the current paper does not address this matter (in order
to keep the paper focussed), the TMFA that we introduce in Section 3 can also be extended with
counters (like the extension to NFA in [29]). Likewise, the Glushkov constructions of [29] and the
current paper can be combined, as can the notions of determinism. The membership problem
for the resulting class of deterministic regex with counters can then be solved as efficiently as
for deterministic regex (see Theorem 5).

3

Memory Automata with Trap State

In this section, we define memory automata with trap-state, the deterministic variant of which
will be the algorithmic foundation for deterministic regex. Before moving on to the actual
definition of deterministic regex in Section 4 and the applications of memory automata with
trap-state, we subject this automaton model to a thorough theoretical analysis. Most of the
thus obtained insights will have immediate consequences and applications for proving the main
results regarding deterministic regex, while others have the mere purpose of supporting our
understanding of memory automata (and therefore the important class of regex languages).
Memory automata [46] are a simple automaton model that characterizes L(RX). Intuitively
speaking, these are classical finite automata that can record consumed factors in memories,
which can be recalled later on in order to consume the same factor again. However, for our
applications, we need to slightly adapt this model to memory automata with trap-state.
Definition 2. For every k ∈ N, a k-memory automaton with trap-state, denoted by TMFA(k),
is a tuple M = (Q, Σ, δ, q0 , F ), where Q is a finite set of states that contains the trap-state
[trap], Σ is a finite alphabet, q0 ∈ Q is the initial state, F ⊆ Q is the set of final states and
δ : Q × (Σ ∪ {ε} ∪ {1, 2, . . . , k}) → P(Q × {o, c, r, ⋄}k ) is the transition function (where P(A)
denotes the power set of a set A), which satisfies δ([trap], b) = {([trap], ⋄, ⋄, . . . , ⋄)}, for every
b ∈ Σ ∪ {ε}, and δ([trap], i) = ∅, for every i, 1 ≤ i ≤ k. The elements o, c, r and ⋄ are called
memory instructions (they stand for opening, closing and reseting a memory, respectively, and
⋄ leaves the memory unchanged).
A configuration of M is a tuple (q, w, (u1 , r1 ), . . . , (uk , rk )), where q ∈ Q is the current
state, w is the remaining input and, for every i, 1 ≤ i ≤ k, (ui , ri ) is the configuration of
memory i, where ui ∈ Σ∗ is the content of memory i and ri ∈ {O, C} is the status of memory
i (i. e., ri = O means that memory i is open and ri = C means that it is closed). The initial
configuration of M ( on input w) is the configuration (q0 , w, (ε, C), . . . , (ε, C)), a configuration
(q, w, (u1 , r1 ), . . . , (uk , rk )) is an accepting configuration if w = ε and q ∈ F .
M can change from a configuration c = (q, vw, (u1 , r1 ), . . . , (uk , rk )) to a configuration c′ =
(p, w, (u′1 , r1′ ), . . . , (u′k , rk′ )), denoted by c ⊢M c′ , if there exists a transition δ(q, b) ∋ (p, s1 , . . . , sk )
with either (b ∈ (Σ ∪ {ε}) and v = b) or (b ∈ {1, 2, . . . , k}, sb = c and v = ub ), and, for every i,
1 ≤ i ≤ k,
8

<!-- page 9 -->

• (si = ⋄) ∧ (ri = O) ⇒ (u′i , ri′ ) = (ui v, ri ),
• (si = ⋄) ∧ (ri = C) ⇒ (u′i , ri′ ) = (ui , ri ),
• si = o ⇒ (u′i , ri′ ) = (v, O),
• si = c ⇒ (u′i , ri′ ) = (ui , C),
• si = r ⇒ (u′i , ri′ ) = (ε, C).
Furthermore, M can change from a configuration (q, vw, (u1 , r1 ), . . . , (uk , rk )) to the configuration ([trap], w, (u1 , r1 ), . . . , (uk , rk )), if δ(q, b) ∋ (p, s1 , . . . , sk ) for some p ∈ Q, b ∈ {1, 2, . . . , k}
and sb = c, such that ub = vv ′ with v ′ 6= ε and v ′ [1] 6= w[1].
A transition δ(q, b) ∋ (p, s1 , s2 , . . . , sk ) is an ε-transition if b = ε and is called consuming,
otherwise (if all transitions are consuming, then M is called ε-free). If b ∈ {1, 2, . . . , k}, it is
called a memory recall transition and the situation that a memory recall transition leads to the
state [trap], is called a memory recall failure.
The symbol ⊢∗M denotes the reflexive and transitive closure of ⊢M . A w ∈ Σ∗ is accepted
by M if cinit ⊢∗M cf , where cinit is the initial configuration of M on w and cf is an accepting
configuration. The set of words accepted by M is denoted by L(M ).
Note that executing the open action o on a memory that already contains some word discards
the previous contents of that memory. A crucial part of TMFA is the trap-state [trap], in which
computations terminate, if a memory recall failure happens. If [trap] is not accepting, then TMFA
are (apart from negligible formal differences) identical to the memory automata introduced in
[46], which characterize the class of regex language. If, on the other hand, [trap] is accepting, then
every computation with a memory recall failure is accepting (independent from the remaining
input). While it seems counter-intuitive to define the words of a language via “failed” backreferences, the possibility of having an accepting trap-state yields closure under complement for
deterministic TMFA (see Theorem 3). It will be convenient to consider the partition of TMFA
into TMFArej and TMFAacc (having a rejecting and an accepting trap-state, respectively).
Next, we illustrate the concept of memory automata with trap state by some examples3
(further illustrations can be found in [46]).
Intuitively speaking, in a single step of a computation of a TMFA, we first change the memory
statuses according to the memory instructions si , 1 ≤ i ≤ k, and then a (possibly empty) prefix
v of the remaining input (v is either from Σ ∪ {ε} or it equals the content of some memory that,
according to the definition, has been closed by the same transition) is consumed and appended
to the content of every memory that is currently open (note that here the new statuses after
applying the memory instructions count). The changes of memory configurations caused by a
transition are illustrated in Figure 1.
Example 2. Consider the following TMFArej M with two memories over Σ = {a, b}:
a, ⋄, ⋄
a, ⋄, ⋄
b, ⋄, ⋄
b, ⋄, ⋄
a, o, ⋄
a, c, o
2, ⋄, c
1, c, ⋄
q0
q1
q2
q3
q4
b, o, ⋄

b, c, o

ε, ⋄, ⋄

This TMFA works as follows. First, in state q1 , we record a non-empty word over {a, b} in the
first memory, then, in state q2 , a non-empty word over {a, b} in the second memory, and then,
by moving through states q3 and q4 , these words are repeated in reverse order by first recalling
the second and then the first memory (note that in the transition from q3 to q4 , an already closed
memory is closed again, since according to Definition 2, every memory that is recalled must be
closed in the same transition). Due to the ε-transition from q4 to q0 , M describes the Kleene-plus
3 For the sake of convenience, we present TMFA in the form of the usual automata diagrams (initial states
are marked by an unlabeled incoming arc, accepting states by an additional circle and arcs are labelled with the
transition tuples).

9

<!-- page 10 -->

c, r
(ε, O)

o

c, r

(ε, C)
o

o

r

r

o
c

(ε, O)

(ε, C)

c

Figure 1: Possible configuration changes of a fixed memory. Note that by ε and ε, we denote an
empty or non-empty memory content, respectively; the instruction ⋄ is omitted. Moreover, the
diagram only shows configuration changes caused by memory instructions (in particular, ε can
only change into ε by consuming transitions).
of such words, i. e., L(M ) = L(α), where α = (hx : (a ∨ b)+ ihy : (a ∨ b)+ i · &y · &x)+ = ({uvvu |
u, v ∈ {a, b}+})+ .
Note that each of the two memory recall transitions closes the respective memory. This is
required by definition, as a transition can only recall a memory if it ensures that it is closed.
Example 3. Consider the following TMFArej M with two memories over Σ = {a, b, d}:
a, ⋄, ⋄
q0

ε, o, ⋄

q1

b, ⋄, ⋄

ε, ⋄, o

q2

d, ⋄, ⋄

ε, c, ⋄

q3

ε, ⋄, c

q4

1, c, ⋄

q5

2, ⋄, c

q6

The behavior of M can be described as follows: First, M opens memory 1 and reads ai , i ≥ 0.
After that, M opens the second memory, reads bj (which is stored in both memories), j ≥ 0,
closes the first memory, reads dk , k ≥ 0, and closes the second memory. Hence, after reading
ai bj dk , the first memory contains ai bj , and the second bj dk . Finally, M recalls memory 1 and
then 2. Hence, L(M ) = {ai bj dk ai b2j dk | i, j, k ≥ 0}.
Now, note that in each input word w, memory 2 is opened and closed after memory 1. Hence,
if j > 0, the areas in w where the two memories are open overlap, instead of being nested. This
cannot happen in a regex, as it is ensured from the syntax of variable bindings that these “areas”
in the word are properly nested. For this reason, it seems impossible to express L(M ) with a
regex with only two variables. But this does not mean that L(M ) is not a regex language, as
L(M ) = L(α) for α = hx : a∗ ihy : b∗ ihz : d∗ i · &x&y · &y&z. In other words, the key idea is
expressing each memory with two variables (one for the overlapping parts of the memories, and
one for each rest).4
Next, we shall see that every TMFAacc can be transformed into an equivalent TMFArej , which
implies L(TMFA) = L(TMFArej ); thus, it follows from [46] that TMFA characterize L(RX). The
idea of this construction is as follows. Every memory i is simulated by two memories (i, 1) and
(i, 2), which store a (nondeterministically guessed) factorisation of the content of memory i. This
allows us to guess and verify if a memory recall failure occurs, i. e., (i, 1) stores the longest prefix
that can be matched and (i, 2) starts with the first mismatch. For correctness, it is crucial that
every possible factorisation of the content of a memory i can be guessed.
We first need the following definition. An M ∈ TMFA is in normal form if no empty
memory is recalled, no open memory is opened, no memory is reset, and, for every transition
δ(q, b) ∋ (p, s1 , . . . , sk ),
• if b 6= ε, then si = ⋄, 1 ≤ i ≤ k,
• if si 6= ⋄, for some i, 1 ≤ i ≤ k, then b = ε and sj = ⋄, for every j, 1 ≤ j ≤ k, i 6= j.
4 Proving that these overlaps can always be resolved is the main step in showing the equivalence of L(RX) and

L(TMFA), which is provided in [46] (see also the discussion at the end of the proof of Theorem 1).

10

<!-- page 11 -->

Proposition 1. Any TMFA can be transformed into an equivalent TMFA in normal form.
Proof. An arbitrary TMFA can be changed into an equivalent one in normal form as follows. By
introducing ε-transitions, we can make sure that every transition is of the form stated in the
proposition. Furthermore, by adding states, we can keep track of the memory configurations
(i. e., their status and whether or not they are empty; this simple technique is also explained in
more detail in the proof of Theorem 3). This allows us to replace transitions that are recalling
an empty memory by ε-transitions. Furthermore, transitions that open an open memory i are
replaced by transitions applying the memory instructions c and o in this order to memory i, and
transitions that reset a memory i are replaced by transitions applying the memory instructions
c, o and c in this order to memory i (the correctness of this can be easily checked with the help
of Figure 1). The TMFA is then in normal form and, by definition, these modifications do not
change the accepted language.
Now, we can formally prove the claimed characterisation.
Theorem 1. L(TMFA) = L(TMFArej ) = L(RX).
Proof. We first note that L(TMFArej ) = L(RX) follows from [46] (we briefly discuss this at the
end of the proof). Since L(TMFArej ) ⊆ L(TMFA) and TMFA = TMFArej ∪ TMFAacc , it only
remains to prove L(TMFAacc ) ⊆ L(TMFArej ). To this end, let M be a TMFAacc in normal form.
First, we replace every memory i, 1 ≤ i ≤ k, by two memories (i, 1) and (i, 2) and we implement
in the finite state control a list (x1 , x2 , . . . , xk ) with entries from Σ ∪ {ε}, which initially satisfies
xi = ε, 1 ≤ i ≤ k. Then, we change the transitions of M such that the new memories (i, 1) and
(i, 2) simulate the old memory i, i. e., memory i stores some word u if and only if memories (i, 1)
and (i, 2) store u1 and u2 , respectively, with u = u1 u2 . Moreover, the element xi always equals
the first symbol of the content of memory (i, 2). More precisely, this can be done as follows. Let
δ(q, b) ∋ (p, s1 , . . . , sk ) be an original transition of M .
• If si = o or si = c, for some i, 1 ≤ i ≤ k, then instead we open memory (i, 1) or close
memory (i, 2), respectively.
• If b ∈ Σ, then, for every open memory (i, 1), we nondeterministically choose to close it and
open memory (i, 2) instead and set xi = b. Then we read b from the input and change to
state p.
• If b ∈ {1, 2, . . . , k}, then we first recall memory (b, 1) and then, for every open memory
(i, 1), we nondeterministically choose to close it and open memory (i, 2) instead and set
xi = xb . Then, we recall memory (b, 2) and change to state p.
All these modifications can be done by introducing intermediate states and using ε-transitions
and the accepted language of M does not change.
The automaton M now stores some content u of an original memory i factorised into two
factors u1 and u2 in the memories (i, 1) and (i, 2), respectively. For the sake of convenience, we
simply say that u is stored in (i, 1) · (i, 2) in order to describe this situation. Next, we show that
if u is stored in (i, 1) · (i, 2), then any way of how u is factorised into the content of (i, 1) and
(i, 2) is possible. More precisely, we show that, for every w, u, u1 , u2 ∈ Σ∗ with u1 u2 = u, M can
reach state p by consuming w with u stored in (i, 1) · (i, 2) if and only if M can reach state p by
consuming w with u1 and u2 stored in (i, 1) and (i, 2), respectively .
The if part of this statement is trivial. We now assume that M can reach state p by
consuming w with u stored in (i, 1) · (i, 2). This implies that we reach the situation that (i, 1) is
open, currently stores u′1 and the next consuming transition consumes u′′1 u′2 , where u1 = u′1 u′′1
and u2 = u′2 u′′2 with u′2 6= ε. If u′′1 = ε, then M can choose to close (i, 1) and then open (i, 2),
which results in u1 and u2 being stored in (i, 1) and (i, 2), respectively. If, on the other hand,
u′′1 6= ε, then the next transition recalls memories (j, 1) and (j, 2) such that u′′1 u′2 is stored in
(j, 1) · (j, 2). If u′′1 and u′2 are stored in (j, 1) and (j, 2), respectively, then M first recalls (j, 1),
chooses to close (i, 1) and open (i, 2), and then recalls (j, 2), which results in u1 and u2 being
stored in (i, 1) and (i, 2). Consequently, we have to repeat this argument for memories (j, 1) and
(j, 2), i. e., we have to show that it is possible that u′′1 u′2 is stored in (j, 1) · (j, 2) in such a way
that u′′1 is stored in (j, 1) and u′2 is stored in (j, 2). Repeating this argument, we will eventually
11

<!-- page 12 -->

arrive at a memory that is not filled by any memory recalls; thus, we necessarily have the case
u′′1 = ε.
Now, we turn M into a TMFArej M ′ , i. e., the state [trap] becomes non-accepting, and, in
addition, we add a new accepting state qt (simulating the old accepting [trap]) with δ(qt , x) =
{(qt , ⋄, . . . , ⋄)}, x ∈ Σ, and we change all ordinary transitions (i. e., transitions that are not recall
failure transitions) of the former M that lead to [trap] such that they now lead to qt . Furthermore,
we change this M ′ such that for every memory recall, there is also the nondeterministic choice
to only recall (i, 1), then check whether xi does not equal the next symbol on the input and, if
this is the case, enter state qt . Obviously, this simulates the memory recall failure of M .
Every word accepted by M without memory recall failures can be accepted by M ′ in the same
way, every word accepted by M due to a recall failure can be accepted by M ′ by guessing and
simulating this memory recall failure. On the other hand, if M ′ accepts a word with a simulated
memory recall failure, then M will accept this word by a proper memory recall failure, and if M ′
accepts a word without a simulated memory recall failure, then, since M ′ ∈ TMFArej , there is no
memory recall failure in the computation and M can accept the word by the same computation.
This completes the proof of L(TMFAacc ) ⊆ L(TMFArej ).
We shall conclude this proof by briefly sketching why L(TMFArej ) = L(RX) holds. For an
α ∈ RX, it is straightforward to obtain an equivalent TMFArej : Transform α into a proper regular
expression αR with L(αR ) = R(α) (by just renaming variable bindings and references), then
transform αR into an equivalent NFA M , and finally interpret M as a TMFArej by interpreting
transition labels [x , ]x as memory instructions and transition labels x as memory recalls. The
other direction relies on first resolving overlaps of memories (i. e., the case that two memories
store factors that overlap in the input word, see also Example 3) and then transforming the
TMFArej M into a proper regular expression for a ref-language that dereferences to L(M ), which
can then directly be interpreted as a regex (due to the non-overlapping property of memories,
which translates into a well-formed nesting of the parentheses [x , ]x ). This works in the same
way as for the case of memory automata without trap-states (see [46] for details).
A consequence of the proof is that TMFA inherits the NP-hardness of the membership problem
from RX. We do not devote more attention to this, as we focus on deterministic TMFA.

3.1

Deterministic TMFA

A TMFA is deterministic (or a DTMFA, for short) if δ satisfies |δ(q, b)| ≤ 1, for every q ∈ Q and
b ∈ Σ ∪ {ε} ∪ {1, 2, . . . , k} (for the sake of convenience, we then interpret δ as a partial function
with range Q × {o, c, r, ⋄}k ), and, furthermore, for every q ∈ Q, if δ(q, x) is defined for some
x ∈ {1, 2, . . . , k} ∪ {ε}, then, for every y ∈ (Σ ∪ {ε} ∪ {1, 2, . . . , k}) \ {x}, δ(q, y) is undefined.5
Analogously to TMFA, we partition DTMFA into DTMFAacc and DTMFArej .
Clearly, the TMFA of Examples 2 and 3 are not deterministic (in Example 2, there are different
transitions for the same state that consume the same symbol and in Example 3, there are states
for which ε-transitions exists in addition to other transitions). By minor changes of the TMFA of
Example 2, a DTMFA can be easily constructed for the language ({udvdvu | u, v ∈ {a, b}+ })+ ,
the details are left to the reader.
The algorithmically most important feature of DTMFA is that their membership can be
solved efficiently by running the automaton on the input word. However, for each processed
input symbol, there might be a delay of at most |Q| steps, due to ε-transitions and recalls of
empty memories, which leads to O(|Q||w|). Removing such non-consuming transitions first, is
possible, but problematic. In particular, recalls of empty memories depend on the specific input
word and could only be determined beforehand by storing for each memory whether it is empty,
which is too expensive. However, by an O(|Q|2 ) preprocessing, we can compute the information
that is needed in order to determine in O(k) where to jump if certain memories are empty, and
which memories are currently empty can be determined on-the-fly while processing the input.
This leads to a delay of only k, the number of memories:
Theorem 2. Given M ∈ DTMFA with n states and k memories, and w ∈ Σ∗ , we can decide
whether or not w ∈ L(M )
5 Note that in [46] deterministic memory automata without trap-state are considered.

12

<!-- page 13 -->

• in time O(n|w|) without preprocessing, or
• in time O(k|w|) after an O(n2 ) preprocessing.
Proof. We first modify M with respect to its ε-transitions as follows. Let p ∈ Q be a state with
an ε-transition that is followed by another ε-transition. If p is contained in a cycle q1 , q2 , . . . , qn
of ε-transitions, we simply replace this cycle by a single state q ′ (i. e., all incoming edges of
any qi , 1 ≤ i ≤ n, then point to q ′ ) that is accepting if and only if some qi , 1 ≤ i ≤ n, is
(note that, since M is deterministic, no qi has any other transition). Otherwise, there are states
q1 , q2 , . . . , qn , p = q1 , with transitions δ(qi , ε) = (qi+1 , si,1 , . . . , si,k ), 1 ≤ i ≤ n − 1, such that qn
has no ε-transition. We can now remove the transition δ(q1 , ε) = (q2 , s1,1 , . . . , s1,k ) and add a
transition δ(q1 , ε) = (qn , t1 , . . . , tk ), where, for every j, 1 ≤ j ≤ k, tj is a memory instruction
that has the same effect as applying instructions s1,j , s2,j , . . . , sn−1,j in this order. Moreover, if,
for some i, 2 ≤ i ≤ n − 1, qi ∈ F , then we define q1 as accepting. By applying this modification
for every ε-transition that is followed by another ε-transition, we can modify M such that no
ε-transition is followed by another ε-transition. Hence, since M is deterministic, there are at
most |Q| ε-transitions and for each, we have to determine the states q1 , q2 , . . . , qn and perform
the modifications described above, which can be done in time O(|Q|), as well. Consequently, the
whole procedure can be carried out in O(|Q|2 ).
Next, we consider states with a memory recall transition. Similar as for states with εtransition, such states are followed by a (possibly empty) sequence of consecutive memory recall
or ε-transitions that either ends in a state with neither memory recall nor ε-transition or eventually forms a loop. We first consider the case, where this sequence does not contain any
ε-transitions and does not form a loop. Let q1 be the state with memory recall transition and
let (q1 , ℓ1 ), (q2 , ℓ2 ), . . . , (qn , ℓn ), qn+1 be the sequence of the following states with consecutive
memory recall transitions along with the memory that is recalled. More precisely, the transition
from qi to qi+1 , 1 ≤ i ≤ n, recalls ℓi and the last element qn+1 is the first state without memory
recall transition (and, by assumption, also without ε-transition). We now contract this list by
the following algorithm. Initially, let A = ∅. Then we move through the list from left to right
and for every element (qi , ℓi ) (except for qn+1 ), we proceed as follows. If ℓi ∈ A, then we remove
(qi , ℓi ) and if ℓi ∈
/ A, then we keep (qi , ℓi ) and add ℓi to A. Obviously, this results in a list
(p1 , r1 ), . . . , (pn′ , rn′ ), qn+1 with n′ ≤ k. The idea is that if we move from left to right through
this new list, it tells us which state to enter if the memory of the current memory recall is empty,
i. e., if memory r1 is non-empty, we recall it in state p1 , if memory r1 is empty, we can directly
jump to state p2 and either recall r2 , if it is non-empty, or jump to p3 otherwise, and so on. If
all memories (that occur somewhere in the list) are empty, we end up in state qn+1 .
In the presence of ε-transitions, we simply ignore these and always only consider the next
transition that recalls a memory, i. e., it is possible that for elements (qi , ℓi ) and (qi+1 , ℓi+1 )
of the non-contracted list, there is an intermediate state p with recall transition from qi to p
and ε-transitions from p to qi+1 (note that due to the construction from above, there are no
consecutive ε-transitions), but the contraction works in the same way. Moreover, if (qi , ℓi ) and
(qj , ℓj ) (or qn+1 , the last element) with i < j are consecutive elements of the contracted list
(i. e., all elements (qr , ℓr ), i + 1 ≤ r ≤ j − 1, have been deleted by the algorithm), then we
replace (qi , ℓi ) by (qiacc , ℓi ) (where the marker acc means that we can accept), if for some r,
i + 1 ≤ r ≤ j, qr ∈ F . Note that this is analogous to the modification from above, where
we define states as accepting, if they are connected to an accepting state by a sequence of εtransitions, but here we cannot change acceptance of the actual states, since it depends on the
current contents of memories, whether we can reach an accepting state by only recalls of empty
memories or ε-transitions.
If the sequence of memory recall transitions enters a loop, we construct the list only up to
the first time a state is repeated, say p, and have (p, loop) as the last element of the list. Then
we apply the contraction in the same way as before, where (p, loop) plays the role of qn+1 .
Similarly as before, we mark elements (qi , ℓi ) as accepting if a pair was removed that contained
an accepting state.
In addition to the states, we also store in the list the memory instructions that have to be
applied in order to jump to the next state (this can be done similiar as for contracting the
ε-transitions above). We construct such a list for every state with a memory recall transition.
Every single list can be constructed in time O(|Q|), so we need time O(|Q|2 ) in total.
13

<!-- page 14 -->

Now we check whether or not w ∈ L(M ) by running (the modified) M on input w in a special
way. We first initialise a list (1, C, ε), (2, C, ε), . . . , (k, C, ε) indicating that every memory is closed
and empty. Then we simulate M on input w as follows. Every transition that consumes a single
symbol as well as every ε-transition is just carried out. Whenever a memory status is changed,
we store this in the list and we also store whether a memory is currently empty or not (note
that we have to know the current statuses in order to do this). When a memory is recalled
in state q, then we move through the list stored for state q until we find a recall of a memory
that is currently non-empty, jump in the automaton to the corresponding state and apply the
memory instructions. Whenever we reach an element (qiacc , ℓi ) in the list, then we check whether
the input has been fully consumed and if yes, we conclude w ∈ L(M ). If we reach in a list an
element (p, loop), then we conclude w ∈ L(M ), if p ∈ F and w ∈
/ L(M ) otherwise. If in the
computation the input has been completely consumed, then we conclude w ∈ L(M ) if and only
if M is in an accepting state.
Since there are no consecutive ε-transitions, every consumption of a single symbol from the
input by a transition is done in constant time. Every consumption by a memory recall transition
requires time O(k), since we have to move through a list of size O(k). Consequently, the total
running time is O(|Q|2 + k|w|).
Note that the preprocessing in the proof of Theorem 2 is only required once, which implies
the following corollary.
Corollary 1. Given M ∈ DTMFA with n states and k memories, and words P
wi ∈ Σ∗ , 1 ≤ i ≤ ℓ,
2
we can decide whether or not wi ∈ L(M ), 1 ≤ i ≤ ℓ, in total time O(n + k |wi |).

Moreover, if it is guaranteed that no empty memories are recalled, then membership can be
solved in O(n + |w|) (where O(n) is needed in order to remove ε-transitions).
Similar to DFA, it is possible to complement DTMFA by toggling the acceptance of states.
However, for DTMFA, we have to remove ε-transitions and recalls of empty memories. In particular, our construction uses the finite control to store whether memories are empty or not, which
causes a blow-up that is exponential in the number of memories.
We first extend the notion of completeness from DFA to DTMFA, by saying that a DTMFA
is complete if, for every q ∈ Q, either δ(q, x) is defined, for every x ∈ Σ, or δ(q, i) is defined,
for some i, 1 ≤ i ≤ k, or δ(q, ε) is defined. This means that a complete DTMFA has, for every
state, either exactly |Σ| transitions (which are all consuming transitions, but not memory recall
transitions), exactly one memory recall transition, or exactly one ε-transition.
For deterministic automata, it is usually possible to apply the state complementation technique (i. e., toggling acceptance of states) in order to show closure under complement. However,
we also need completeness and ε-freeness, since otherwise it may happen that a word is not
accepted because its computation gets stuck or enters an infinite ε-loop and therefore is not
entirely processed, which leads to a word which is accepted neither by the original nor by the
complement automaton. The requirement of completeness and ε-freeness is not a restriction for
DTMFA, since these properties can be achieved by classical techniques. However, recalling empty
memories, which are special cases of ε-transition, can cause the same problems and therefore we
have to get rid of them as well. This can be done by storing in the finite-state control whether
the memories are currently empty or non-empty and then treating recalls of empty memories as
ε-transitions and remove them along with the other ε-transition in the classical way (note that
the trick of handling empty memories that has been used in the context of Theorem 2 cannot
applied here, since the automaton needs to store the information for all possible runs on input
words).
We need a few more definitions: Let Γ = {o, c, r, ⋄} and let ⊚ be a binary operator on Γ
defined by x ⊚ y = y, if y 6= ⋄ and x ⊚ y = x, if y = ⋄. Furthermore, we extend ⊚ to Γk by
(x1 , . . . , xk ) ⊚ (y1 , . . . , yk ) = (x1 ⊚ y1 , . . . , xk ⊚ yk ). We note that ⊚ is associative and some
memory instructions s1 , s2 , . . . , sn ∈ Γ applied to some memory in this order have the same
result as the memory instruction s1 ⊚ s2 ⊚ . . . ⊚ sn (this can be easily verified with the help of
Figure 1).
Next, we prove a sequence of propositions (that are all proved in a straightforward way by
applying classical automata constructions):

14

<!-- page 15 -->

Proposition 2. Let M ∈ DTMFA. For every w ∈ Σ∗ and every configuration c for M , there
exists at most one configuration c′ with c ⊢M c′ .
Proof. Let c = (q, v, (u1 , r1 ), . . . , (uk , rk )). If no δ(q, i), 1 ≤ i ≤ k, is defined, then there is
obviously at most one c′ with c ⊢m c′ . If δ(q, i) = (p, s1 , . . . , sk ), for some i, 1 ≤ i ≤ k,
then either v = ui v ′ , which implies that c ⊢M (p, v ′ , (u′1 , r1′ ), . . . , (u′k , rk′ )), where the (u′j , rj′ ),
1 ≤ j ≤ k, are uniquely determined by ui and the sj , 1 ≤ j ≤ k, or ui is not a prefix of v, which
implies that c ⊢M ([trap], v ′′ , (u1 , r1 ), . . . , (uk , rk )), where v = v ′ v ′′ and v ′ is the largest common
prefix of v and ui . In both cases, there is at most one configuration c′ with c ⊢M c′ .
Proposition 3. For every M ∈ DTMFA there exists an ε-free M ′ ∈ DTMFA with L(M ) =
L(M ′ ).
Proof. Let M = (Q, Σ, δ, q0 , F ). For every p ∈ Q, if, for some q ∈ Q, δ(p, ε) = (q, s1 , . . . , sk ),
then we define Sε,1 (p) = q and M(p, q) = (s1 , . . . , sk ). For every p ∈ Q and every i, 2 ≤ i ≤
|Q| − 1, we define Sε,i (p) = Sε,1 (Sε,i−1 (p)) and, if Sε,i (p) is defined, we define (or redefine)
M(p, Sε,i (p)) = M(p, Sε,i−1 (p)) ⊚ (s1 , . . . , sk ), where δ(Sε,i−1 (p), ε) = (Sε,i (p), s1 , . . . , sk ).
For every p ∈ Q with δ(p, ε) defined, we now remove the ε-transitions as follows. Let
i, 1 ≤ i ≤ |Q| − 1, be such that Sε,i (p) = q and Sε,i+1 (p) is undefined. Furthermore, let
δ(q, xj ) = (tj , sj,1 , . . . , sj,k ), 1 ≤ j ≤ ℓ, for some ℓ with 0 ≤ ℓ ≤ |Σ|, be all the transitions from
q (note that ℓ = 1 and x1 ∈ {1, 2, . . . , k} covers the case of a single memory recall transition
and, furthermore, xj = ε is by definition not possible). We now add new transitions δ(p, xj ) =
(tj , s′1 ⊚ sj,1 , . . . , s′k ⊚ sj,k ), where M(p, q) = (s′1 , . . . , s′k ). Then, we simply delete all ε-transitions
(note that this may produce states that are not reachable anymore, which are deleted as well).
It can be easily verified that this results in an M ′ ∈ DTMFA with L(M ) = L(M ′ ).
Proposition 4. For every M ∈ DTMFA there exists a complete M ′ ∈ DTMFA with L(M ) =
L(M ′ ).
Proof. Let M = (Q, Σ, δ, q0 , F ). We transform M into M ′ by adding a new non-accepting state
t with δ(t, x) = (t, ⋄, . . . , ⋄), for every x ∈ Σ, and we add transitions for every state q ∈ Q as
follows. If δ(q, i) is undefined, for every i, 1 ≤ i ≤ k, and δ(q, ε) is undefined, then, for every
x ∈ Σ with δ(q, x) undefined, we set δ(q, x) = (t, ⋄, . . . , ⋄). On the other hand, if δ(q, i) is defined,
for some i, 1 ≤ i ≤ k, or δ(q, ε) is defined, then we do not add any transition. By definition, M ′
is complete and, since t is non-accepting, L(M ) = L(M ′ ).
Remark 1. We note that the construction of the proof of Proposition 3 preserves completeness, i. e., if M is a complete DTMFA, then we obtain an equivalent complete DTMFA without
ε-transitions. Moreover, the construction of the proof of Proposition 4 does not introduce εtransitions; thus, it turns an ε-free DTMFA into an equivalent complete DTMFA that is still
ε-free.
We are now ready to show closure of L(DTMFA) under complementation.
Theorem 3. L(DTMFA) is closed under complement.
Proof. Let M = (Q, Σ, δ, q0 , F ) ∈ DTMFA. By Proposition 4, we can assume that M is complete.
Due to Proposition 2, for any input w, there is a unique computation of M on w. Hence, the
idea is now to toggle the acceptance of all the states of M in order to obtain a DTMFA that
accepts L(M ). However, this only works if M is ε-free, since otherwise it is possible that some
word w ∈ Σ∗ cannot be fully consumed by M (for example, if it leads into a loop in which all
transitions are ε-transitions and no state is accepting); thus, w is neither accepted by M nor by
the DTMFA obtained by toggling the acceptance of states. While we can remove ε-transitions
due to Proposition 3, we encounter the problem that a memory recall transition with respect
to an empty memory behaves just like an ε-transition and, thus, can cause the same problems.
Hence, we first have to transform such memory recall transition into ordinary ε-transitions,
which can then be removed according to Proposition 3.
To this end, we modify M such that the finite state control stores, for every i, 1 ≤ i ≤ k,
whether or not memory i is open and whether or not memory i stores the empty word. More
precisely, we obtain an M1 ∈ DTMFA by modifying M as follows. Every state q is replaced by
15

<!-- page 16 -->

22k new states [q, (r1 , c1 ), . . . , (rk , ck )], where ri ∈ {C, O}, ci ∈ {ε, ε}, 1 ≤ i ≤ k, and we change
the transitions such that if M1 reaches a configuration with state [q, (r1 , c1 ), . . . , (rk , ck )], then,
in the current configuration, for every i, 1 ≤ i ≤ k, ri is the status of memory i and memory
i is empty if and only if ci = ε. For example, if M1 is in state [p, (r1 , c1 ), . . . , (rk , ck )] with
(ri , ci ) = (C, ε) and δ(p, x) = (q, s1 , . . . , sk ) with x ∈ Σ and si = o, then, if x is the next symbol
of the input, M1 changes to a state [q, (r1′ , c′1 ), . . . , (rk′ , c′k )] with (ri′ , c′i ) = (O, ε). We note that
M1 is still complete and deterministic.
Next, we change M1 into M2 by replacing, for every i, 1 ≤ i ≤ k, every transition of the form
δ([p, (r1 , c1 ), . . . , (rk , ck )], i) = ([q, (r1′ , c′1 ), . . . , (rk′ , c′k )], s1 , . . . , sk )
with ci = ε by an ε-transition
δ([p, (r1 , c1 ), . . . , (rk , ck )], ε) = ([q, (r1′ , c′1 ), . . . , (rk′ , c′k )], s1 , . . . , sk ) .
We note that L(M1 ) = L(M2 ) and, since M1 is deterministic, this only introduces ε-transitions,
such that if δ(p, ε) is defined then, for every y ∈ (Σ ∪ {1, 2, . . . , k}), δ(q, y) is undefined. Consequently, M2 is still deterministic and it never happens that an empty memory is recalled. Next,
by Proposition 3, we can transform M2 into a complete M3 ∈ DTMFA without ε-transitions (see
Remark 1) that still has the property that no empty memories are recalled.
Let M ∈ DTMFA be obtained from M3 by toggling the acceptance of the states, i. e., if Q3
and F3 are the sets of states and accepting states, respectively, of M3 , then M is obtained from
M3 by replacing F3 by Q3 \ F3 . Obviously, for every w ∈ Σ∗ , both M3 and M , on input w, reach
the same state and completely consume the input. This directly implies L(M ) = L(M3 ).
We next discuss expressive power: If there is a constant upper bound on the lengths of
contents of memories that are recalled in accepting computations of an M ∈ DTMFA, then
memories can be simulated by the finite state control; thus, L(M ) ∈ L(REG). Consequently,
if L(M ) ∈
/ L(REG), there is a word uvw that is accepted by recalling some memory with an
arbitrarily large content v. Moreover, if [trap] is non-accepting, then no word can be accepted
that contains u as a prefix, but not uv, since this will cause a memory recall failure. Intuitively
speaking, a DTMFArej for a non-regular language makes arbitrarily large “jumps”:
Lemma 1 (Jumping Lemma). Let L ∈ L(DTMFArej ). Then either L is regular, or for every
m ≥ 0, there exist n ≥ m and pn , vn ∈ Σ+ such that
1. |vn | = n,
2. vn is a factor of pn ,
3. pn vn is a prefix of a word from L,
4. for all u ∈ Σ+ , pn u ∈ L only if vn is a prefix of u.
Proof. As L ∈ L(DTMFArej ), there exists an M ∈ DTMFArej with L(M ) = L. If there is an
m ≥ 0 such that in every accepting run of M , each memory stores only a word of length at most
m, then L is regular (as we can rewrite M into a DFA that stores the contents of the memories
in its states). Likewise, if memories can store words of unbounded length, but are then never
recalled, these memories can be eliminated, which also allows us to turn M into a DFA for L.
Hence, if L is not regular, M has at least one memory x such that for every m ≥ 0, there
is an accepting run of M on a word w during which x stores a word of length n ≥ m, and
this memory is recalled with this content. Let pn be the part of the accepting run that M has
processed up to a state q where it recalls x at a point where this memory contains a word vn of
length n. As vn must have been consumed while processing pn , |pn | ≥ n holds, and vn must be
a factor of pn .
If M succeeds at recalling x at this point (i. e., it consumes vn ), it can continue to accept
w, which means that pn vn is a prefix of w ∈ L. On the other hand, on an input pn u for some
u ∈ Σ+ such that vn is not a prefix of u, M encounters a memory recall failure and rejects. As
M is deterministic, the recall transition for x must be the only transition that leaves the state
q. Hence, pn u ∈
/ L for u that do not have vn as prefix.
16

<!-- page 17 -->

The Jumping Lemma is a convenient tool for proving that languages cannot be accepted by
a DTMFArej , which shall be illustrated by some examples.
Example 4. Let L : ={ww | w ∈ Σ∗ } with |Σ| ≥ 2, which is well-known to be not regular.
Assume L ∈ L(DTMFArej ) and choose m : = 1. Then there exist n ≥ 1 and pn , vn ∈ Σ∗ that
satisfy the conditions of Lemma 1. Choose a ∈ Σ that is not the first letter of vn , and define
u : = apn a. Then vn is not a prefix of u, but pn u = (pn a)2 ∈ L, which is a contradiction.
Example 5. Let L : ={ai baj | i > j ≥ 0}. Using textbook methods, it is easily shown that L
is not regular. Now, assuming that L ∈ L(DTMFArej ), choose m : = 4. Then there exist n ≥ 4
and pn , vn ∈ Σ+ that satisfy the conditions of Lemma 1. As pn vn is a prefix of a word in L,
either pn = ai or pn = ai baj with i, j ≥ 0 (and i ≥ 4 or i + j ≥ 3). In the first case, consider
u : = ba. Then pn u = ai ba with i ≥ 4; hence, pn u ∈ L. But u starts with b, and vn is a factor
of pn = ai , which leads to a contradiction, as vn cannot be a prefix of u. For the second case, let
u : = a. As pn vn is a prefix of a word in L, and as |vn | = n, i > j + n ≥ j + 4 must hold. Hence,
pn u = ai baj+1 , and pn u ∈ L, which, as vn is not a prefix of u, leads again to a contradiction.
For unary languages, there is an alternative to Lemma 1 that is easier to apply and that
characterizes unary DTMFArej -languages. It is built on the following definition: A language
L ⊆ {a}∗ is an infinite arithmetic progression if L = {abi+c | i ≥ 0} for some b ≥ 1, c ≥ 0.
Lemma 2. Let L ∈ L(DTMFArej ) be an infinite language with L ⊆ {a}∗ . The following conditions are equivalent:
1. L is regular.
2. L contains an infinite arithmetic progression.
3. There is b ≥ 1 such that, for every n ≥ 0, there exists some cn ≥ 0 with abi+cn ∈ L for all
0 ≤ i ≤ n.
Proof. We show that 1 implies 2, which implies 3, which implies 1. The first two of these steps
are simple: Assume that L is regular. Every regular language over a single letter alphabet
can be expressed as a finite union of arithmetic progressions (cf., e. g., Chrobak [13, 14]). As
L is infinite, it must contain an infinite arithmetic progression. But if L contains an infinite
arithmetic progression aib+c , then the third condition is satisfied by definition.
The step from 3 to 1 is more involved. Before we prove this, note that there are unary
languages (which are not DTMFA-languages), for which condition 3 does not imply the existence
of an infinite arithmetic progression, see Example 6 below.
Assume that L ⊆ {a}∗ is infinite, L ∈ L(DTMFArej ), and condition 3 is met for some b ≥ 1.
By definition, there is an M ∈ DTMFArej with L(M ) = L. As M is deterministic, each of
its states can have at most one outgoing transition; and as L is infinite, each state must have
exactly one outgoing transition. Hence, like a DFA for a unary language (see e. g. the proof of
Theorem 6, in particular Figure 2), M consists of a chain and a cycle. Let m be the number
of accepting states on the cycle, and let k be the number of memories that are accessed in the
cycle (by recalling them, or by performing memory instructions).
Now consider an n > m(b + 1)k such that there exists some cn with wi : = abi+cn ∈ L for all
0 ≤ i ≤ n, and reading w0 = acn takes M into the cycle (as condition 3 holds for all n, such a cn
exists for every n that is sufficiently large). Let q be the accepting state that is reached by w0 .
In the following, by an iteration of the cycle, we mean the situation that M is in state q
and then consumes input symbols until it reaches q for the next time. The iteration of the
cycle that starts after having fully consumed w0 is called iteration 1. Now, for every j ≥ 1, we
define a function ~vj : {1, . . . , k} → N that describes the content of each memory after completing
iteration j.
In the remainder of the proof, we show that there is a constant upper bound for the values
~vj (x), 1 ≤ x ≤ k, j ≥ 1. Note that if the length of the content of each memory is bounded, then
M can be rewritten into an equivalent DFA that simulates all memories in its states. Hence, L
must be a regular language, which shows that condition 3 implies condition 1.
As M has to accept all words wi with 0 ≤ i ≤ n, and as each iteration of the cycle can accept
n
only m words, we know that M has to perform at least I : = m
> (b + 1)k iterations of the cycle
17

<!-- page 18 -->

in order to accept wn . During these iterations, M cannot consume more than ab between each
pair of accepting states – otherwise, M would skip at least one of the wi (as M is deterministic,
the run for wn must be an extension of each run for a wi with i < n). In particular, this means
that each memory that is recalled during these iterations cannot contain more than ab ; thus,
there are only b + 1 possible contents for each memory. Furthermore, as M is deterministic, we
know that each memory that is not recalled during these iterations will not be recalled during
any later iterations of the cycle, which means that it can be removed from the cycle (and, as
the chain is of finite length, it can also be removed from the chain). Hence, without loss of
generality, we can assume that ~vj (x) ≤ b, 1 ≤ x ≤ k ′ , 1 ≤ j ≤ I, where the cycle contains
exactly the memories 1, . . . , k.
As I > (b + 1)k and as there are only (b + 1)k possible choices of ~vj , there exist j, j ′ with
0 ≤ j ′ < j ′ ≤ I and ~vj = ~vj ′ . As M is deterministic, this allows us to conclude ~vj+l = ~vj ′ +l
for all l ≥ 0. In other words, the sequence of transitions from iteration j to iteration j ′ will be
repeated forever, using exactly the same memory contents, which means that ~vl (x) ≤ b for all
l ≥ 0 and all 1 ≤ x ≤ k. As explained above, this concludes the proof.
Just like the Jumping Lemma, Lemma 2 can be used to prove DTMFArej -inexpressibility for
unary languages. See the following examples.
Example 6. We define a Lex ⊂ {a}∗ together with its complement Lex in the following way:
First, add a to Lex , then add the two words a2 and a3 to Lex , and the three words a4 to a6
to Lex , and so on. In other words, in each step i, we add the next i words of {a}∗ to one
of the languages; namely Lex if i is odd, and Lex if i is even. Then Lex satisfies condition 3 of
Lemma 2, but it does not contain any infinite arithmetic progression. Hence, Lex ∈
/ L(DTMFArej );
rej
/ L(DTMFA ) follows analogously.
and Lex ∈
Example 7. Let α : =hx : aa+ i(&x)+ (this regex is also known as “Abigail’s expression” [1] in
the PERL community). Then L(α) = {amn | m, n ≥ 2}. In other words, α generates the
language of all ai such that i is a composite number (i. e., not a prime number). As L(α) is not
regular and contains the arithmetic progression 2i + 4, Lemma 2 yields L(α) ∈
/ L(DTMFArej ).
The following result is a curious consequence of Lemma 2:
Proposition 5. Over unary alphabets, we have
L(DTMFArej ) ∩ L(DTMFAacc ) = L(REG).
Proof. We first observe that L(REG) ⊆ L(DTMFArej ) ∩ L(DTMFAacc ) holds by definition. Next,
we assume that there is a non-regular language L ∈ (L(DTMFArej )∩L(DTMFAacc )) over {a}∗. In
particular, this implies that both L and its complement L : ={a}∗ \L are infinite and, furthermore,
by Theorem 3, L ∈ L(DTMFAacc ) implies L ∈ L(DTMFArej ). Since L ∈ L(DTMFArej ) is a nonregular DTMFArej language, Lemma 1 allows us to conclude that for every m ≥ 0, there exist an
n ≥ m and a pn ≥ n such that ai ∈ L for all pn ≤ i < pn + n. Hence, L contains finite arithmetic
progressions of unbounded length; and as L is infinite, Lemma 2 states that L is regular, which
is a contradiction.

4

Deterministic Regex

In order to define deterministic regex as an extension of deterministic regular expressions, we first
extend the notion of a marked alphabet that is commonly used for the latter: For every alphabet
A, let Ã : ={a(n) | a ∈ A, n ≥ 1}. For every α ∈ RX, we define α̃ as a regex that is obtained
by taking αR (the proper regular expression over Σ ∪ Ξ ∪ Γ that generates the ref-language
R(α)), and marking each occurrence of χ ∈ (Σ ∪ Ξ ∪ Γ) by a unique number (to make this
well-defined, we assume that the markings start at 1 and are increased stepwise). For example,
if α : =hy : (a ∨ &x)∗ · (ε ∨ b · a)i · &y, then α̃ = [y (1) (a(2) ∨ x(3) )∗ · (ε ∨ b(4) · a(5) )]y (6) · y(7) . We
also use these markings in the ref-words: For example, [y (1) a(2) a(2) x(3) a(2) ]y (6) y(7) ∈ R(α̃).
Before we explain this definition and use it to define deterministic regex, we first discuss the
special case of deterministic regular expressions: A proper regular expression α is not deterministic if there exist words u, v1 , v2 ∈ Σ̃∗ , a terminal a ∈ Σ and positions i 6= j such that ua(i) v1
18

<!-- page 19 -->

and ua(j) v2 are elements of L(α̃) (see e. g. [9, 31]). Otherwise, it is a deterministic proper regular
expression (or, for short, just deterministic regular expression).
The intuition behind this definition is based on the Glushkov construction for the conversion
of regular expressions into finite automata, as a regular expression α is deterministic if and only
if its Glushkov automaton M(α) is deterministic. Given a regular expression α, we define M(α)
in the following way: First, we use the marked regular expression α̃ to construct its occurrence
graph Gα̃ , a directed graph that has a source node src, a sink node snk, and one node for each
a(i) in α̃.6 The edges are constructed in the following way: Each node a(i) has an incoming edge
from src if a(i) can be the first letter of a word in L(α̃), and an outgoing edge to snk if it can
be the last letter of such a word. Furthermore, for each factor a(i) b(j) that occurs in a word of
L(α̃), there is an edge from a(i) to b(j) . As a consequence, there is a one-to-one-correspondence
between marked words in L(α̃) and paths from src to snk in Gα̃ . To obtain M(α), we directly
interpret Gα̃ as NFA over Σ: The source src is the starting state, each node a(i) is a state qi ,
and an edge from a(i) to b(j) corresponds to a transition from qi to qj when reading b. The
sink snk does not become a state; instead, each node with an edge to snk is a final state (hence,
M(α) contains the source state, and one state for every terminal in α). This interpretation
allows us to treat occurrence graphs as an alternative notation for a subclass of NFA (namely
those where the starting state is not reachable from other states, and for each state q, there is
a characteristic terminal aq such that all transitions to q read aq ). When doing so, we usually
omit the occurrence markings on the nodes in graphical representations.
Intuitively, M(α) treats each terminal of α as a state. Recall that α is not deterministic if
there exists words ua(i) v1 and ua(j) v2 in L(α̃) with i 6= j. This corresponds to the situation
where, after reading u, M(α) has to decide between states a(i) and a(j) for the input letter a.
Example 8. Let α : =(ε ∨((a ∨ b)+ a)). Then α̃ = (ε ∨((a1 ∨ b2 )+ a3 )), and the Glushkov automaton M(α) of α is defined as follows:
a
a(1)

a

a(3)

b a
b(2)

1

a

3

b a
b
2

To the left, M(α) is represented as an occurrence graph, to the right in standard NFA notation.
Then M(α) and α are both not deterministic: For M(α), consider state 1; for α, consider
u = a(1) , v1 = a(3) , v2 = ε, and the words ua(1) v1 and ua(3) v2 .
As shown in [9], L(DREG) ⊂ L(REG) (also see [16, 39], or Lemma 5 below). Like for
determinism of regular expressions, the key idea behind our definition of deterministic regex is
that a matcher for the expression treats terminals (and variable references) as states. Then an
expression is deterministic if the current symbol of the input word always uniquely determines
the next state and all necessary variable actions. For regular expressions, non-determinism can
only occur when the matcher has to decide between two occurrences of the same terminal symbol;
but as regex also need to account for non-determinism that is caused by variable operations or
references, their definition of non-determinism is more complicated.
Definition 3. An α ∈ RX is not deterministic if there exist ρ1 , ρ2 ∈ R(α̃) such that any of the
following conditions is met for some r, s1 , s2 ∈ (Σ̃ ∪ Ξ̃ ∪ Γ̃)∗ and γ1 , γ2 ∈ Γ̃∗ :
1. ρ1 = r · γ1 · a(i) · s1 and ρ2 = r · γ2 · a(j) · s2 with a ∈ Σ and i 6= j,
2. ρ1 = r · γ1 · x(i) · s1 and ρ2 = r · γ2 · χ(j) · s2 with x ∈ Ξ, χ ∈ (Σ ∪ Ξ) and i 6= j,
3. ρ1 = r · γ1 · χ(i) · s1 and ρ2 = r · γ2 · χ(i) · s2 with χ ∈ (Σ ∪ Ξ) and γ1 6= γ2 ,
4. ρ1 = r · γ1 and ρ2 = r · γ2 with γ1 6= γ2 .
Otherwise, α is deterministic. We use DRX to denote the set of all deterministic regex, and
define DREG : = DRX ∩ REG as the set of deterministic regular expressions (as discussed below,
Condition 1 of Definition 3 covers the case of deterministic proper regular expression).
6 Most literature, like [9], defines the occurrence graph only implicitly by using sets first, last, and follow, which
correspond to the edge from src, the edges to snk, or to the other edges of the graph, respectively. The explicit
use of a graph is taken from the k-occurrence automata by Bex et al. [5]. We shall see that an advantage of
graphs is that they can be easily extended by adding memory actions to the edges.

19

<!-- page 20 -->

Example 9. We define α1 : =(hx : ai ∨ a), α2 : =(a ∨ &x), α3 : =(hx : εi ∨ ε)a and α4 : =(hx : εi ∨ ε).
None of these regex are deterministic, as each αi meets the i-th condition of Definition 3. We
discuss this for α1 : Observe α̃1 = ([x (1) a(2) ]x (3) ) ∨ a(4) . Then choosing ρ1 = [x (1) a(2) ]x (3) and
ρ2 = a(4) , with r = ε, γ1 = [x (1) , s1 = ]x (3) , and γ2 = s2 = ε shows the condition is met.
∗
Let β1 : =hx : (a ∨ b)∗ ic · &x and β2 : = hx : &yihy : &x · ai . Both regex are deterministic,
2
with L(β1 ) : ={wcw | w ∈ {a, b}∗ } and L(β2 ) = {an | n ≥ 0} (see Example 1).
Condition 1 of Definition 3 describes cases where non-determinism is caused by two occurrences of the same terminal (γ1 and γ2 are included for cases like α1 in Example 9). If restricted
to regular expressions, it is equivalent to the usual definition of deterministic regular expressions. Condition 2 expresses that the matcher has to decide between a variable reference and
any other symbol (this may be a terminal, a different variable or the same variable, but with
a different index); while in condition 3, the symbol is unique, but there is a non-deterministic
choice between variable operations. Finally, condition 4 describes cases where the behavior of
variables is non-deterministic after the end of the word (while one could consider this edge case
deterministic, this choice simplifies recursive definitions). In conditions 3 and 4, the definition
not only requires that it is clear which variables are reset, but also that it is clear which part of
the regex acts on the variables. Hence, (hx : εi ∨hx : εi) is also not deterministic. This is similar
to the notion of strong determinism for regular expressions, see [29]. As one might expect, some
non-deterministic regexes define DRX-languages:
Example 10. Let Σ = {0, 1} and α : = 1+ hx : 0∗ i(1+ &x)∗ 1+ . This regex was introduced by
Fagin et al. [20], who call its language the “uniform-0-chunk language”. Obviously, α is not
deterministic (in fact, it satisfies conditions 1, 2, and 3 of Definition 3). Nonetheless,
it is

possible to express L(α) with the deterministic regex 1 1+ ∨ 0hx : 0∗ i1+ (0 · &x · 1+ )∗ .

We now discuss the conversion from DRX to DTMFArej , which generalizes the Glushkov
construction of M(α) for regular expressions. The core idea is extending the occurrence graph
to a memory occurrence graph Gα̃ , which has two crucial differences: First, instead of only
considering terminals, each terminal and each variable reference of a regex α becomes a node.
Second, each edge is labeled with a ref-word from Γ̃∗ that describes the memory actions (hence,
there can be multiple edges from one node to another). In analogy to the occurrence graph, each
memory occurrence graph can be directly interpreted as an ε-free TMFArej .
Theorem 4. Let α ∈ RX, and let n denote the number of occurrences of terminals and variable
references in α. We can construct an n + 2 state TMFArej M(α) with L(M(α)) = L(α) that
is deterministic if and only if α is deterministic. In time O(|Σ||α|n), the algorithm either
1. computes M(α) if α is deterministic, or 2. detects that α is not deterministic.
Proof. We construct M(α) by first constructing a graph Gα̃ from the marked regex α̃. As
Gα̃ is a generalization of the occurrence graphs for proper regular expressions, we call this the
memory occurrence graph. Analogously to proper regular expressions, this graph can be directly
interpreted as an M(α) ∈ TMFArej that is deterministic if and only if α is deterministic.
Memory occurrence graph Gα̃ : Given a marked regex α̃, we define a memory occurrence
graph Gα̃ : =(Vα̃ , Eα̃ ) with a source node src, a sink node snk, and one node for each marked
variable reference or terminal. The labeled edges are of the form (u, ν, v), where u, v ∈ Vα̃ ,
and each label ν is a marked ref-word ν ∈ Γ̃∗ . We use marked ref-words instead of unmarked
ref-words to fulfill the promise that M(α) is deterministic if and only if α is deterministic. If α
has n occurrences of variable references and terminals, M(α) has n + 2 states: the initial state,
the state [trap] for memory recall failures, and one state for each of the n occurrences in α.
If we only want to construct an algorithm that turns a deterministic regex into a DTMFArej
and rejects non-deterministic regexes, we can use unmarked edge labels instead (see the section
at the end of this proof).
When interpreting Gα̃ as a TMFA M(α), we first remove the markings from the edge labels,
and interpret these as memory actions of a TMFA, e. g., [x corresponds to opening the memory
for x. In order to simplify the construction, we take into account that different ref-words over Γ
can have the same net effect on variables, and can be represented by the same single transition
20

<!-- page 21 -->

in a TMFA. For example, ]x [x ]x [x and ]x [x and [x all have the same effect as performing o
on the memory for x. Following this intuition, given a ref-word ν ∈ Γ∗ , we define the net
variable action of ν as a function netν : Ξ → {o, c, r, ⋄}, where for each x ∈ Ξ, netν (x) : = ⋄ if no
element of Γx : ={[x , ]x } occurs in ν, and netν (x) : = o if the rightmost occurrence of an element
of Γx is a [x . Furthermore, if the rightmost occurrence of an element of Γx is ]x , we define
netν (x) : = r if ν contains [x , and netν (x) : = c otherwise. In the construction further down, we
also consider concatenations of labels. We observe the following for all ν, ν1 , ν2 ∈ Γ∗ and all
x ∈ Ξ: If netν (x) = ⋄, then netν1 ·ν (x) = netν1 (x) and netν·ν2 = netν2 (x). If netν2 (x) ∈ {o, r},
then netν1 ·ν2 (x) = netν2 (x).
We also use the following notion of minimal representations: For all ν ∈ Γ∗ and x ∈ Ξ, we
define minx (ν) ∈ Γ∗ by minx (ν) : = [x if netν (x) = o, minx (ν) : = ]x if netν (x) = c, minx (ν) : = [x ]x
if netν (x) = r, and minx (ν) : = ε if netν (x) = ⋄. For any ν ∈ Γ∗ , its minimal representation
min(ν) is defined as any concatenation of all minx (ν) for all x ∈ Ξ (as netν 6= ⋄ holds only for
finitely many x ∈ Ξ, this is not problematic). In other words, for each ν ∈ Γ∗ , min(ν) is one of
the shortest words in Γ∗ that satisfies netmin(ν) = netν . By unmark : (Σ̃ ∪ Ξ̃ ∪ Γ̃) → (Σ ∪ Ξ ∪ Γ)∗ ,
we denote the morphism that removes the markings from marked letters.
By using net, we can directly interpret a memory occurrence graph Gα̃ as a TMFA M(α) : =(Q,
Σ, δ, src, F ), analogously to the occurrence graph for proper regular expressions. The components of M(α) are obtained as follows: First, we rename the variables such that Gα̃ contains
exactly the variables {1, . . . , k} for some k ≥ 0 (hence, for each 1 ≤ i ≤ k, there is a variable
xi ∈ var(α) such that xi is represented by i). We then define
Q : =(Vα̃ \ {snk}) ∪ {[trap]},
F : ={u ∈ Q | (u, ν, snk) ∈ Eα̃ for some ν}.
In other words, all nodes except snk are states, and all nodes that have an edge to snk are final
states (as in the occurrence graph). Following this intuition, each edge (u, ν, v) with v 6= snk
corresponds to a transition from state u to state v, while performing the memory actions of
netν (x) on each x ∈ var(α). In order to allow recursive applications of the construction, each
edge (u, ν, snk) not only marks that u is an accepting state, but also that the memory actions of
ν need to be performed before accepting. Formally, we define δ to include exactly the following
transitions:
1. If (u, ν, a(i) ) ∈ Eα̃ with a ∈ Σ, then (a(i) , s1 , . . . , sk ) ∈ δ(u, a).
2. If (u, ν, x(i) ) ∈ Eα̃ with x ∈ var(α), then (x(i) , s1 , . . . , sk ) ∈ δ(u, x),
where for each 1 ≤ i ≤ k, si : = netunmark(ν) (xi ) (unless the transition recalls memory i; then we
choose si : = c as required by Definition 2). As we shall see, in order to satisfy the condition that
M(α) is deterministic only if α is deterministic, we need to slightly adapt this definition.
Following this interpretation, we say that a memory occurrence graph Gα̃ is not deterministic
if there exists a u ∈ Vα̃ such that any of the following conditions is met:
1. Eα̃ contains edges (u, ν1 , a(i) ) and (u, ν2 , a(j) ) with i 6= j and a ∈ Σ.
2. Eα̃ contains edges (u, ν1 , x(i) ) and (u, ν2 , χ(j) ) with i 6= j, x ∈ Ξ, χ ∈ (Ξ ∪ Σ),
3. Eα̃ contains edges (u, ν1 , χ(i) ) and (u, ν2 , χ(i) ) with ν1 6= ν2 and χ ∈ (Ξ ∪ Σ),
4. Eα̃ contains edges (u, ν1 , snk) and (u, ν2 , snk) with ν1 6= ν2 .
Otherwise, we call Gα̃ deterministic. It is easily seen that if Gα̃ is deterministic, M(α) is
also deterministic. For the other direction, we need to account for two problems: First, it is
possible that two labeled ref-words ν1 and ν2 map to the same memory action netunmark(ν1 ) =
netunmark(ν2 ) , e. g., ν1 = [x (1) ]x (2) and ν2 = [x (3) ]x (4) [x (5) ]x (6) , which can occur in regex like

α1 : = hx : εi ∨(hx : εihx : εi) a. Second, as M(α) has no ε-transitions, it does not model the difference between distinct edges to snk, as they appear when converting regex like α2 : =(ε ∨hx : εi).
As DTMFA cannot detect explicitly that the end of the input has been reached, they cannot
simulate the memory actions of edges to snk, which means that the construction ignores this.
21

<!-- page 22 -->

In both cases, the accepted language is correct; but this has the side effect that the resulting
M(α) is deterministic, although α and Gα̃ are not. Hence, to ensure that M(α) is deterministic
only if α is deterministic, we proceed as follows: If Gα̃ contains any of these edges, we pick any
transition δ(q, b) ∋ (p, s1 , . . . , sk ) with b ∈ Σ ∪ {1, 2, . . . , k}. We then add a new state pndet , a
transition δ(q, b) ∋ (pndet , s1 , . . . , sk ), and pndet has the same outgoing transitions as p. If we
want to construct an algorithm that rejects non-deterministic regex, we can simply omit this
technical crutch, and detect these cases in the construction of Gα̃ as discussed below.
Constructing Gα̃ : We now define Gα̃ = (Vα̃ , Eα̃ ) recursively.
1. Empty word: If α̃ = ε, we define
Vα̃ : ={src, snk},
Eα̃ : ={(src, ε, snk)}.
This case is completely straightforward: An edge from src to snk is how occurrence graphs
model ε, and the marking ε means that this transition performs no memory actions.
2. Terminals and variable references: If α̃ = χ(i) with χ ∈ (Σ ∪ Ξ), we define
Vα̃ : ={src, χ(i) , snk},
Eα̃ : ={(src, ε, χ(i) ), (χ(i) , ε, snk)}.
Similar to the case for ε, this models that the terminal is read, or that a variable reference
is processed, by recalling the appropriate memory.
3. Variable bindings: If α̃ = ([x (i) β̃]x (j) ) with x ∈ Ξ, we define Vα̃ : = Vβ̃ and
Eα̃ : = {(src, [x (i) · νin , v) | (src, νin , v) ∈ Eβ̃ , v 6= snk}
∪ {(u, ν, v) | (u, ν, v) ∈ Eβ̃ , u 6= src, v 6= snk}
∪ {(u, νout · ]x (j) , snk) | (u, νout , snk) ∈ Eβ̃ , u 6= src}
∪ {(src, [x (i) · νε · ]x (j) , snk) | (src, νε , snk) ∈ Eβ̃ },
Less formally, we take the memory occurrence graph for β and add opening (and closing) of
x to all edges from src (and to snk, respectively); while all other edges remain unchanged.
Note that for edges from src to snk, we could also use νε · [x ]x or [x ]x · νε , as by Definition 1,
hx : βi is only a regex if x ∈
/ var(β), which implies that no marked [x or ]x occurs in νε .
4. Disjunction: If α̃ = (β̃ ∨ γ̃), we define Vα̃ : = Vβ̃ ∪ Vγ̃ and Eα̃ : = Eβ̃ ∪ Eγ̃ .
As the markings define a one to one correspondence between the nodes in Vα̃ and the
terminals and the variable references in α, we know that Vβ̃ ∩ Vγ̃ = {src, snk}. Therefore,
the resulting memory occurrence graph Gα̃ computes the union of Gβ̃ and Gγ̃ .
5. Concatenation: If α̃ = (β̃ · γ̃), we define
Vα̃ : = Vβ̃ ∪ Vγ̃ ,
Eα̃ : ={(u, ν, v) | (u, ν, v) ∈ Eβ̃ , v 6= snk}
∪ {(u, ν, v) | (u, ν, v) ∈ Eγ̃ , u 6= src}
∪ {(u, (ν1 · ν2 ), v) | (u, ν1 , snk) ∈ Eβ̃ , (src, ν2 , v) ∈ Eγ̃ },
Again, we use the fact that Vβ̃ ∩ Vγ̃ = {src, snk}. The memory occurrence graph Gα̃
first simulates Gβ̃ , until the latter would accept by processing an edge (u, ν1 , snk) ∈ Eβ̃ .
Instead of following this edge to snk, Gα̃ then starts its simulation of Gγ̃ , by picking any
edge (src, ν2 , v) ∈ Eγ̃ , which is merged with (u, ν1 , snk) into a single edge from u to v, and
its label is (ν1 · ν2 ). Hence, it is easy to see that Gα̃ computes the concatenation of Gβ̃
and Gγ̃ .
22

<!-- page 23 -->

6. Kleene plus: Assume α̃ = β̃ + . This case requires some additional definitions. Let Nε
denote the set of all ν with (src, ν, snk) ∈ Eβ̃ , and let N (∗) : ={min(ν) | ν ∈ Nε∗ }, where
we assume that the elements of N (∗) have some arbitrary markings (as we shall see, this
definition matters only for non-deterministic regex, which means that we do not need
markings to detect non-determinism).
Note that, as Nε is finite, there are only finitely many x ∈ Ξ such that netν (x) 6= ⋄ for
a ν ∈ Nε , which implies that N (∗) is finite. In order to avoid hiding non-determinism in
some very special cases, we assume that N (∗) always contains at least two elements (this
is possible without loss of generality, as we can always add some ν 2 for a ν ∈ N (∗) without
changing the behavior). We now define Vα̃ : = Vβ̃ , as well as
Eα̃ : = Eβ̃ ∪ {(src, ν̂ · νin , v) | (src, νin , v) ∈ Eβ̃ , ν̂ ∈ N (∗) }
∪ {(u, νout · ν̂, snk) | (u, νout , snk) ∈ Eβ̃ , ν̂ ∈ N (∗) }
∪ {(u, νout · ν̂ · νin , v) | (u, νout , snk) ∈ Eβ̃ , (src, νin , v) ∈ Eβ̃ , ν̂ ∈ N (∗) }.
Similar to the construction for concatenation, the idea is that Gα̃ simulates Gβ̃ ; and
whenever the latter could accept by taking an edge to snk, the former can loop back to the
beginning. The only difficult part is when Gβ̃ contains edges from src to snk with memory
actions. As the Kleene plus allows us to use an arbitrary amount of these edges before
taking an edge from src or to snk, we need to include Nε∗ in the functions. This set is
generally infinite; but it can be compacted to the finite set N (∗) .
For deterministic regex, this construction collapses to a far simpler case that does not use
N (∗) : First, note that if α is deterministic and contains β + , (src, ν, snk) ∈ Eβ̃ implies ν = ε,
as otherwise, β would satisfy condition 4 of Definition 3, and α would satisfy condition 3
or 4. Hence, if α is deterministic, we can assume that Nε = {ε} or Nε = ∅; both cases lead
to N (∗) = {ε}. This allows us to use the following simplified definition:
Eα̃ : = Eβ̃ ∪ {(u, νout · νin , v) | (u, νout , snk) ∈ Eβ̃ , (src, νin , v) ∈ Eβ̃ }.
Hence, when constructing Gα̃ inductively, we first check if Eβ̃ contains an edge (u, ν, v)
with ν 6= ε. If this is the case, we can reject α as not deterministic. Otherwise, we use this
simplified definition.
Correctness and determinism: The correctness of the construction is easily seen by
a lengthy but straightforward induction, using the explanations provided with the definitions
above. In particular, note that if α does not contain a Kleene plus (or contains a Kleene plus and
is deterministic), each path from src to snk through Gα̃ corresponds to a marked ref-word from
R(α), and vice versa. If α is not deterministic and contains a Kleene plus, the correspondence
is a little bit less strict, as ref-words γ ∈ Γ+ are compressed to the equivalent min(γ).
To see that M(α) is deterministic if and only if α is deterministic, recall that we established
above that M(α) is deterministic if and only if Gα̃ is deterministic. Hence, it suffices to show
that determinism in Gα̃ is equivalent to determinism in α. But this follows immediately from
our observation that there is a one-to-one correspondence between paths in Gα̃ and the marked
ref-words in R(α), and the fact that each node χ(i) ∈ Vα̃ \ {src, snk} corresponds to the same
χ(i) in α̃. Thus, if Gα̃ satisfies a condition i for non-determinism, α satisfies the same condition
i in Definition 3, and vice versa.
Complexity: Given a regex α, let n denote the number of occurrences of terminals and
variable references in α. We examine two steps of the computation: Computing Gα̃ , and converting it to M(α).
For the first step, observe that Gα̃ has n + 2 nodes, and if α is deterministic, each node has at
most min(n, |Σ|) outgoing edges, which means that we can bound this number with |Σ|. Hence,
if α is deterministic, Gα̃ can be computed in time O(|Σ||α|n) by directly following the recursive
definition of Gα̃ : If α is represented as a tree, it has at most |α| nodes, which means that the
recursive rules have to be applied O(|α|) times. Each rule application requires the creation of at
most O(|Σ|n) edges, each of which uses a concatenation.
23

<!-- page 24 -->

For the conversion, we need to process each edge (u, ν, v) ∈ Eα̃ , and compute its function
netunmark(ν) . From the recursive definition, we can immediately conclude that |ν| ∈ O(|α|) (as α
is deterministic, we do not even need to take into account that the definition for Kleene plus uses
min). Hence, each edge can be turned into a transition in time O(|α|). As α is deterministic,
there are O(|Σ|n) edges, which gives us a total time of O(|Σ||α|n) for this step.
As we have the same estimation for both steps, we conclude that the total running time is
O(|Σ||α|n).
If α is not deterministic, this can be discovered during the construction, as soon as the
recursive definition computes a non-deterministic memory occurrence graph Gβ̃ for a nondeterministic subexpression β of α, or if hidden non-determinism is detected.
Unmarked edge labels: As mentioned above, if the goal is not to construct an M(α) that
is deterministic if and only if α is deterministic, but to turn every deterministic α in a deterministic M(α) and to reject non-deterministic α, we can construct Gα̃ by using unmarked ref-words
on the labels. The only cases where using unmarked ref-words can hide non-determinism (in the
sense that Gα̃ is deterministic, although α is not) is in the rule for union. For example, consider
α : =hx : εi ∨hx : εi, which satisfies condition 4 of Definition 3, as L(α̃) contains [x (1) ]x (2) and
[x (3) ]x (4) , due to α̃ = ([x (1) ]x (2) ) ∨([x (3) ]x (4) ). If we use unmarked ref-words, Gα̃ consists only
of a single edge from src to snk with label [x ]x , which is clearly deterministic. Nonetheless, we
can detect this hidden non-determinism when recursively constructing Gα̃ , by checking whether
there exist edges (src, ν1 , snk) ∈ Eβ̃ and (src, ν2 , snk) ∈ Eγ̃ with ν1 6= ε or ν2 6= ε. Hence, if the
conversion algorithm encounters this case, it can reject the regex as non-deterministic.
Note that concatenation cannot hide non-determinism: For u ∈ Vβ̃ and v ∈ Vγ̃ , define
Nu : ={ν | (u, ν, snk) ∈ Eβ̃ } and Nv : ={ν | (src, ν, v) ∈ Eγ̃ }. Assume that at least one of the two
sets Nu and Nv contains more than one element. Then Eα̃ contains at least two edges from u
to v, which means that Gα̃ is not deterministic. Finally, Kleene star is also unaffected by this
change, as the presence of any edge (src, ν, snk) with ν 6= ε causes non-determinism regardless of
whether ν is marked or not.
Furthermore, note that an implementation of this construction can also represent each label
ν in reduced form as netmin(ν) , if it ensures that no hidden non-determinism is present.
Let us illustrate the construction of the proof of Theorem 4 by an example.
Example 11. Consider the deterministic regex α : =hx : (a ∨ b)+ i·d ·&x. Applying the markings
yields α̃ : = [x (1) (a(2) ∨ b(3) )+ ]x (4) · d(5) · x(6) , and M(α) is the following automaton:
a, ⋄
[x (1)

a(2)

]x (4)

a, o
d(5)

[x (1)

b(3)

x(6)

2

a, ⋄

]x (4)

b, o

d, c
b, ⋄

3

5

1, c

6

d, c
b, ⋄

To the left, M(α) is represented as the memory occurrence graph Gα̃ , to the right as the DTMFA
that can be directly derived from this graph (which uses memory 1 for x).
The construction from the proof of Theorem 4 behaves like the Glushkov construction for regular expressions, with one important difference: On regex that are not deterministic, its running
time may be exponential in the number of variables; as there are non-deterministic regex where
conversion into a TMFA without ε-transitions requires an exponential amount
∗ For
W of transitions.
example, for k ≥ 1, let α : = a · (ε ∨hx1 : εi) · · · (ε ∨hxk : εi) · b and β : = a 1≤i≤k hxi : εi b. An
automaton that is derived with a Glushkov style conversion then contains states q1 and q2 that
correspond to the terminals; and between these two states, there must be 2k different transitions
to account for all possible combinations of actions on the variables. This suggests that converting
a regex into a TMFA without ε-edges is only efficient for deterministic regex; while in general, it
is probably advisable to use a construction with ε-edges.
By combining Theorems 4 and 2, due to n ≤ |α|, we immediately obtain the following:
24

<!-- page 25 -->

Theorem 5. Given α ∈ DRX with n occurrences of terminal symbols or variable references and
k variables, and w ∈ Σ∗ , we can decide in time O(|Σ||α|n + k|w|), whether w ∈ L(α).
If we ensure that recalled variables never contain ε (or that only a bounded number of
variables references are possible in a row), we can even drop the factor k. For comparison, the
membership problem for DREG can be decided in time O(|Σ||α| + |w|) when using optimized
versions of the Glushkov construction (see [8, 44]), and in O(|α| + |w| · log log |α|) with the
algorithm by Groz and Maneth [31] that does not compute an automaton.

5

Expressive Power

Although Câmpeanu, Salomaa, Yu [10] as well as Carle and Narendran [11] state pumping
lemmas for a class of regex, these do not apply to regex as defined in this paper (see Section 2.1.1).
However, Lemmas 1 and 2, introduced in Section 3, shall be helpful for proving inexpressibility.
A consequence of Lemma 2 is that there are infinite unary DTMFArej -languages that are not
pumpable (in the sense that certain factors can be repeated arbitrarily often), as this would
always lead to an arithmetic progression. It is also possible to demonstrate this phenomenon on
larger alphabets, without relying on a trivial modification of the unary case.
The Fibonacci word Fω is the infinite word that is the limit of the sequence of words F0 : = b,
F1 : = a, and Fn+2 : = Fn+1 ·Fn for all n ≥ 0. The Fibonacci word has a number of curious properties. In particular, it is cube-free, which means that it does not contain cubes (i. e., factors www,
with w 6= ε). This and various other properties are explained throughout Lothaire [38]. In
the following, we demonstrate that an infinite subset L of {Fi | i ≥ 0} can be generated by a
deterministic regex. Since the cube-freeness of Fω particularly implies that all Fi , i ≥ 0, are
cube-free, this demonstrates that L is a DRX-language that cannot be pumped by repeating factors of sufficiently large words arbitrarily often. This is a rather counter-intuitive phenomenon
with respect to a class of formal languages that is characterised by a natural extension of classical
finite automata.
Lemma 3. Let L : ={F4i+3 | i ≥ 0}. Then L = L(β) holds for the deterministic regex
β : = ahx0 : bihx1 : ai · (βshift )∗ ,
βshift : =hx2 : &x1 &x0 ihx3 : &x1 &x0 &x1 ihx0 : &x3 &x2 ihx1 : &x3 &x2 &x3 i .
Proof. First, we observe that R(β) = {ri | i ≥ 0}, where the ref-words ri are defined by
r0 : = a[x0 b]x0 [x1 a]x1 and ri+1 : = ri · r̂ for all i ≥ 0, where
r̂ : = [x2 x1 x0 ]x2 [x3 x1 x0 x1 ]x3 [x0 x3 x2 ]x0 [x1 x3 x2 x3 ]x1 .
We now prove by induction that, for each i ≥ 0, D(ri ) = F4i+3 , and the rightmost values that
are assigned to x0 and x1 are F4i and F4i+1 , respectively. For i = 0, this is obviously true:
D(r0 ) = aba = F3 , x0 is assigned b = F0 , and x1 is assigned a = F1 .
Now assume that the claim holds for some i ≥ 0, and consider ri . Then we can observe that
D(ri+1 ) = D(ri · r̂) = D(ri ) · D(s), where the ref-word s is obtained from r̂ by replacing x0 and
x1 with their respective values F4i and F4i+1 . Hence,
s = [x2 F4i+1 · F4i ]x2 [x3 F4i+1 · F4i · F4i+1 ]x3 [x0 x3 x2 ]x0 [x1 x3 x2 x3 ]x1
= [x2 F4i+2 ]x2 [x3 F4i+3 ]x3 [x0 x3 x2 ]x0 [x1 x3 x2 x3 ]x1 .
The second part of this equation uses that Fn+3 = Fn+2 · Fn+1 = Fn+1 · Fn · Fn+1 holds for
all n ≥ 0. We now construct a ref-word t by replacing the variables x2 and x3 in s with their
respective values. Hence,
t = [x2 F4i+2 ]x2 [x3 F4i+3 ]x3 [x0 F4i+3 · F4i+2 ]x0 [x1 F4i+3 · F4i+2 · F4i+3 ]x1
= [x2 F4i+2 ]x2 [x3 F4i+3 ]x3 [x0 F4i+4 ]x0 [x1 F4i+5 ]x1 .
Then D(ri+1 ) = D(ri · t) holds, and ri+1 assigns x0 and x1 as t does. Hence, x0 is assigned
F4(i+1) , and x1 is assigned F4(i+1)+1 , as required by the claim. To see that D(ri+1 ) = F4(i+1)+3 ,
25

<!-- page 26 -->

we observe that
D(ri+1 ) = D(ri · t)
= F4i+3 · D([x2 F4i+2 ]x2 [x3 F4i+3 ]x3 [x0 F4i+4 ]x0 [x1 F4i+5 ]x1 )
= F4i+3 · F4i+2 · F4i+3 · F4i+4 · F4i+5
= F4i+5 · F4i+4 · F4i+5
= F4i+7 = F4(i+1)+3 .
This concludes the proof.
Also note that we can construct an M ∈ DTMFArej with L(M ) = LF : ={Fn | n ≥ 1} from
M(β). More specifically, this can be achieved by making the states that have an outgoing
transition that opens a variable accepting. In terms of regex, this corresponds to being able to
accept before each variable binding (the correctness of this construction follows from the proof
of Lemma 3). Note that the cycle in M has multiple accepting states. The authors conjecture
that there is no DTMFA for LF that has a cycle with exactly one accepting state, and that LF
is not a DRX-language.
For further separations, we use the following language:
∗
i
Example 12. Let α : = a2 · hx : a2 i · hy : &x · &xi · hx : &y · &yi . Then L(α) = {a4 | i ≥ 1}.
This can be proven with a straightforward induction that is left to the reader.
From this, we define an L ∈ L(TMFA) with neither L ∈ L(DTMFArej ), nor L ∈ L(DTMFAacc ):
i

Lemma 4. Let L : ={a4i+1 | i ≥ 0} ∪ {a4 | i ≥ 1}. Then L ∈ L(TMFA) \ L(DTMFA).
Proof. To show that L ∈ L(TMFA), we construct a regex α for L, by α : = α1 ∨ α2 , where
i
α1 : = a(a4 )∗ , and α2 is the deterministic regex with L(α2 ) = {a4 | i ≥ 1} from Example 12.
Next, observe that L contains the arithmetic progression {a4i+1 | i ≥ 0}, and L : ={a}∗ \ L
contains the arithmetic progression {a4i+2 | i ≥ 0}. Assume that L is a DTMFA-language. Then
there is an A ∈ DTMFArej that accepts L or L. Moreover, by Lemma 2, L is regular (note that in
case L(A) = L, we also use that the class of regular languages is closed under complementation).
This leads to a contradiction, since, as we shall see next, L is not regular. To show that L is
not regular, first assume the contrary. Then L(α2 ) = L ∩ {a4 }∗ would be regular, as the class of
regular languages is closed under intersection. But L(α2 ) is not regular, as for every pair i 6= j,
i
j
a4 and a4 are not Nerode-equivalent.
While inexpressibility through DTMFArej provides us with a powerful sufficient criterion for
DRX-inexpressibility, it is not powerful enough to cover all cases of DRX-inexpressibility. In
particular, there are even regular languages that are no DRX-languages:

1
Lemma 5. Let L : = L (ab)∗ (a ∨ ε) = {(ab) 2 i | i ≥ 0}. Then L ∈ L(REG) \ L(DRX).

Proof. Before we assume the existence of an α ∈ DRX with L(α) = L (and use this to obtain
a contradiction), we first examine the structure of any M ∈ DTMFArej with L(M ) = L. We
observe that, with the exemption of states that are unreachable or cannot reach an accepting
state, M must consist of a chain (which might be empty) that is followed by a cycle that contains
at least one final state (like a DFA for a unary language, see the proof of Theorem 6, in particular
Figure 2).
This is for the following reason: First, like for every DTMFA, each state of M that has an
outgoing memory recall transition cannot have any other outgoing transitions. The same holds
for ε-transitions. Furthermore, due to the structure of L, in M no state can have an outgoing
transition that consumes a and an outgoing transition that consumes b at the same time. For
DTMFA, this is not problematic. In fact, as L is regular, we can interpret any DFA for L as
a DTMFA for L. For example, consider the following minimal incomplete DFA for L, and its
corresponding notation as an occurrence graph (without markings):

26

<!-- page 27 -->

a

a

b

b

If we consider the occurrence graph notation, we see that this DFA cannot be obtained from
a deterministic regular expression (at least not using the Glushkov construction, which – in the
absence of variables – is identical to the construction from the proof of Theorem 4). Note that
the states for a and b belong to the same strongly connected component. Hence, if there is an
α ∈ DREG such that this automaton is M(α), then α must contain a subexpression β + with the
occurrences a(i) and b(j) that correspond to these states. Then Gβ̃ must contain edges from a(i)
and b(j) to snk, and from src to a(i) . Using the rule for Kleene plus from the proof of Theorem 4,
we see that Gα̃ must contain an edge from a(i) to itself. This is a contradiction. Of course, this
argument only shows that this DFA cannot be obtained from a deterministic regular expression;
but it can be generalized to show that there is no α ∈ DREG with L(α) = L (see e. g. [9], and in
particular [12], which explains how to apply the technique from [9] on this language).
We shall now use a similar line of reasoning to obtain a contradiction from the assumption
that there is an α ∈ DRX with L(α) = L. As explained above, the DTMFA M(α) must consist
of a chain and a cycle (by definition, each state of M(α) is reachable; and from each state, we
can reach an accepting state). This means that Gα̃ contains a cycle v1 , . . . , vn for some n ≥ 1
and vi ∈ (Σ̃ ∪ Ξ̃) such that there is an edge from vi to vi+1 for 1 ≤ i < n and from vn to v1 .
Hence, each vi has at most two outgoing edges: One to the next node in the cycle, and (if it is
an accepting state) one to the sink node snk. Furthermore, exactly one vi has an incoming edge
from outside the cycle, let this be v1 .
From the construction of Gα̃ , this cycle must have been generated from a Kleene plus in α.
But this allows us to conclude that only exactly one vi can have an edge to snk; and furthermore,
that this must be vn . This can be concluded from the following reasoning: If there existed nodes
vi , vj with i 6= j, and both have an edge to snk, then the construction for Kleene plus would
require edges from both vi and vj to v1 , which would break the cycle structure. Likewise, if
i 6= n, then there must be an edge from vi to v1 , and from vi to vi+1 , which is a contradiction
to our previous observations.
Hence, each iteration of the cycle must consume exactly one terminal letter (otherwise, we
would skip over words of L), alternating between a and b. Thus, vi ∈ Ξ̃ must hold for all
1 ≤ i ≤ n (as vi = a(j) with a(j) ∈ Σ̃ would consume a in every iteration, which would
contradict the fact that the iterations alternate between consuming a and b).
Now assume that we enter an iteration that consumes a (the same reasoning shall hold for b).
Then no variable that is recalled can contain b, and no variable can be bound to b, as otherwise,
the iteration would consume more than a. But in the next iteration, the same variables are
recalled, and as neither of them contains b, the iteration cannot consume b. Therefore, we arrive
at a contradiction, and conclude that there is no α ∈ DRX with L(α) = L.
The language L from Lemma 5 is also known to be a non-deterministic regular language (see
e. g. [9]). Our proof can be seen as taking the idea behind the characterization of deterministic
regular languages from [9], applying it to the specific language L, and also taking variables into
account. While this accomplishes the task of proving that deterministic regex share some of the
limitations of deterministic regular expressions, the approach does not generalize (at least not
in a straightforward manner). In particular, deterministic regex can express regular languages
that are not deterministic regular, and are also quite similar to L:
3

Example 13. Let L : ={(ab) 2 i | i ≥ 0}. Then L is generated by the non-deterministic regular
expression (ababab)∗ (ε ∨(aba)), and one can show that L is not a deterministic regular language
by using the BKW-algorithm [9] (also [16, 39]) on the minimal DFA M for L. But for
∗
α : = ahy : bihx : ai hz : &yihy : &xihx : &zi ,

we have α ∈ DRX and L(α) = L (to see this, note that in the iterations of the Kleene-star, the
variable contents alternate between z = b, y = a, x = b and z = a, y = b, x = a).

27

<!-- page 28 -->

q1

a
p0

a

p1

a

···

a

a

q2

..
.

pm
a

a

qn−1

a

qn−2

a

Figure 2: Illustration of the unary DFA in the proof of Theorem 6. Note that here, we do not
distinguish between accepting and non-accepting states
The “shifting gadget” that is used in Example 13 can be extended to show a far more
general result for unary languages. Considering that L(DREG) ⊂ L(REG) holds even over unary
alphabets (cf. Losemann et al. [37]), the following result might seem surprising:
Theorem 6. For every regular language L over a unary alphabet, L ∈ L(DRX).
Proof. Assume that L ∈ L(REG) with L ⊆ {a}∗ . Our goal is to construct an α ∈ DRX with
L(α) = L. For technical reasons, we assume that ε ∈
/ L (this is no problem, as for any α ∈ DRX
with ε ∈
/ L(α), (α ∨ ε) ∈ DRX). If L is finite, L ∈ L(DREG), and hence L ∈ L(DRX). (As we
shall see, our construction can also be used for finite languages, by replacing αcycle in αchain
below
k
with ε. But to streamline the argument, we only consider infinite L.)
Let M be a DFA with L(M ) = L. Assume that all states of M are reachable, and that from
each state, an accepting state can be reached. Then M has the form as shown in Figure 2. We
refer to the states p0 to pm as the chain, and to the states q1 , . . . , qn−1 and pm as the cycle.
Without loss of generality, we can assume that pm is accepting (the cycle contains at least an
accepting state; and as the automaton does not have to be minimal, we can extend the chain of
pi by unrolling the cycle until it starts with an accepting state).
Now there exists a number k ≥ 1 and c1 , . . . , ck ≥ 1 such that the words ac1 , ac1 +c2 , . . . , ac1 +···+ck
are exactly the words that are accepted in the chain (recall that we assume that pm is accepting,
and that p0 is not accepting, as ε ∈
/ L). Furthermore, there exists an ℓ ≥ 1 and b1 , . . . , bℓ ≥ 1
such that the words ab1 , ab1 +b2 , . . . , ab1 +···+bℓ are exactly the words that
P advance the cycle
P from
pm to each of the accepting states. These conditions also imply m = ki=1 ci and n = ℓi=1 bi .
As an additional restriction, we assume that b1 ≥ 2. This is possible for the following reasons:
If bi = 1 for all i, we can replace the cycle with the deterministic regular expression a∗ and are
done. Furthermore, if b1 = 1, but there is a bi ≥ 2, we can unroll the cycle into the chain until
b1 ≥ 2 (for the “new” b1 ).
We define α : = αchain
, where, for 1 ≤ i < k,
1
αchain
: = aci (ε ∨ αchain
i+1 ),
i
αchain
: =hxℓ : aiack −1 (ε ∨ αcycle ).
k
Before we define αcycle , note that if we disregard the words that can be generated by the subexpression αcycle , L(α) contains exactly the words that are accepted by the chain. Furthermore, if
we assume that αcycle is a deterministic regex and that its language does not contain ε, we can
conclude α ∈ DRX. Finally, note that if we first enter αcycle , the variable xℓ contains a.
The central part of the construction is defining αcycle in such a way that it is deterministic
and it simulates the cycle of the DFA. We define
αcycle : = αshift · αcont

+

,

αshift : =hx0 : &xℓ ihxℓ : &xℓ−1 i · · · hx2 : &x1 ihx1 : &x0 i,
αcont : = &x1b1 −2 · &x2b2 −1 · · · · · &xbℓℓ −1 .
The idea behind this definition is as follows. The expression αcycle should be able to generate
Pℓ′
the words that are generated in the cycle of the DFA, i. e., all the words ai , where i = r j=1 bj
28

<!-- page 29 -->

L(TMFA) = L(RX)
L(DTMFA)
L(DTMFArej )
L(DRX)

L(DTMFAacc )

L(REG)

L(DREG)

Figure 3: The proper inclusions from Theorem 7. Arrows point from sub- to superset.
for some r ≥ 1 and ℓ′ with 1 ≤ ℓ′ ≤ ℓ (more precisely, r is the iteration of the cycle and in the
current iteration, the word terminates in the (ℓ′ )th accepting state). This is done by producing,
in every iteration of the expression (αshift · αcont )+ , another factor abj as follows: Before the first
iteration, xℓ contains a, and all other variables default to ε. Now, note that passing through
αshift , the a from xℓ is shifted to x1 , or from xi to xi+1 for 1 ≤ i < ℓ. This means that in every
application of αcont , exactly one of the variables x1 , x2 , . . . , xℓ contains a, which is responsible
for producing the next abj factor, while all the other variables are empty. However, this shifting
mechanism needs an additional variable, namely x0 , which, in each iteration that charges x1 with
a, produces an additional occurrence of a (this explains the assumption b1 ≥ 2 above and that
&x1 is repeated for only b1 − 2 times in αcont instead of bi − 1, as the other variable references).
In summary, in the i-th iteration of the plus, the variable xj with j : =((i − 1) mod ℓ) + 1 is set
to a, and if j = 1, then x0 is also set to a. All other variables are set to ε. This means that αshift
produces a2 in the i-th iteration if (i mod ℓ) = 1, and a in all other iterations. If j = 1, then
the &x1b1 −2 in αcont produces ab1 −2 , which means that in this iteration, αshift · αcont produces
b −1
aa · ab1 −2 = ab1 (recall that all other variables are set to ε). If j > 1, then αcont used &xjj to
produce abj −1 , which means that αshift · αcont produces a · abj −1 = abj .
In conclusion, the i-th iteration of the Kleene plus in αcycle adds the word abj for j : =((i −
1) mod ℓ) + 1; which means that αcycle simulates the cycle. Hence, L(α) = L. As αcycle contains
no disjunctions or Kleene plus (except for the surrounding plus), it is deterministic. As remarked
above, this allows us to conclude α ∈ DRX.
As, in the proof of Theorem 6, a DFA with n states is converted into a deterministic regex
of length O(n), this construction is even efficient. We summarize our observations (also see
Figure 3):
Theorem 7.
L(DREG) ⊂ L(DRX) ⊂ L(DTMFArej ) ⊂ L(DTMFA) ⊂ L(TMFA) = L(RX)
The following pairs of classes are incomparable: L(DRX) and L(REG), L(DRX) and L(DTMFAacc ),
as well as L(DTMFArej ) and L(DTMFAacc ).
Proof. This follows from our previous observations as follows:
1. L(DREG) ⊂ L(DRX): The inclusion follows from the fact that our definition of determinism
for regex is an extension of the notion of determinism for proper regular expressions. To
see that the inclusion is proper, we recall any of the non-regular DRX-languages that we
2
have seen, for example {wcw | w ∈ {a, b}∗ } and {an | n ≥ 0} from Example 9.
2. L(DRX) ⊂ L(DTMFArej ): The inclusion follows from Theorem 4, it is proper due to the
1
language {(ab) 2 i | i ≥ 0}, see Lemma 5.

29

<!-- page 30 -->

3. L(DTMFArej ) ⊂ L(DTMFA): The inclusion holds by definition. It is proper as the inclusion L(DTMFAacc ) ⊂ L(DTMFA) also holds by definition, and as L(DTMFArej ) and
L(DTMFAacc ) are incomparable (see below).
4. L(DTMFA) ⊂ L(TMFA): Again, the inclusion holds by definition. Languages that separate
the two classes are for example the language of all ww (where w is from a non-unary
i
alphabet, see Example 4), and the language {a4i+1 | i ≥ 0} ∪ {a4 | i ≥ 1} from Lemma 4.
5. L(TMFA) = L(RX) is the statement of Theorem 1.
1

6. L(DRX) and L(REG) are incomparable: Again, we can use {(ab) 2 i | i ≥ 0} from Lemma 5,
and a non-regular DRX-language, like {wcw | w ∈ {a, b}∗}.
7. L(DTMFArej ) and L(DTMFAacc ) are incomparable: Due to Proposition 5, over a unary
alphabet, for every non-regular language L ∈ L(DTMFArej ), we have L ∈ L(DTMFArej ) \
2
L(DTMFAacc ), and L ∈ L(DTMFAacc )\L(DTMFArej ). Hence, we can choose e. g. L = {an |
n ≥ 0} (which, as shown in Example 9, is in L(DRX) ⊂ L(DTMFArej )) and its complement
to show the two classes to be incomparable.
8. L(DRX) and L(DTMFAacc ) are incomparable: Since L(REG) ⊆ L(DTMFAacc ), the language
1
{(ab) 2 i | i ≥ 0} is in the class L(DTMFAacc ), but, due to Lemma 5, not in L(DRX).
Moreover,
2
L = {an | n ≥ 0} ∈ L(DRX),
see Example 9, but if L ∈ L(DTMFAacc ), then, due to Theorem 4, also L ∈ L(DTMFArej ) ∩
L(DTMFAacc ), which, by Proposition 5, leads to the contradiction L ∈ L(REG).
This concludes the proof.
We can also use the examples from this section to show that the classes L(DRX) and
L(DTMFArej ) are not closed under most of the commonly studied operations on languages:
Theorem 8. L(DRX) and L(DTMFArej ) are not closed under the following operations: union,
concatenation, reversal, complement, homomorphism, and inverse homomorphism. L(DRX) is
also not closed under intersection, and intersection with DREG-languages.
Proof. The proofs for the operations that apply to both classes follow the same basic scheme:
We start with one (or more) DRX-language(s), and show that applying the operation yields a
language that is not a DTMFArej -language:
i

Union: We use L1 : ={a4i+1 | i ≥ 0} and L2 : ={a4 | i ≥ 1}, which are defined by the
∗ deterministic regex α1 : = a(a4 )∗ , and α2 : = a2 · hx : a2 i · hy : &x · &xi · hx : &y · &yi (see
Example 12). Then L1 ∪ L2 ∈
/ L(DTMFArej ), as shown in Lemma 4.
Concatenation: Define the deterministic regexes α3 : = a+ and α4 : =hx : a∗ i · b · &x. Then
L(α4 ) = {ai bai | i ≥ 0}, and L(α3 ) · L(α4 ) = {ai baj | i > j ≥ 0}. As shown in Example 5,
L(α3 ) · L(α4 ) is not a DTMFArej -language.
Reversal: Let α5 : =hx : a∗ i · b · &x · a+ . Then α5 ∈ DRX, and L(α5 ) = {aj bai | i > j ≥ 0}.
Reversing L(α5 ) again gives us the language from Example 5.
2

Complement: This follows directly from Proposition 5. Consider e. g. {an | n ≥ 0}.
Homomorphism: Let α6 : =((c · α1 ) ∨(d · α2 )). Then α6 ∈ DRX, but h(L(α6 )) = L1 ∪ L2 for
the morphism h that is defined by h(x) : = x if x ∈ {a, b} and h(x) : = ε if x ∈ {c, d}.
Inverse homorphism: Define a morphism g by g(a) : = g(b) : = a, and g(c) : = b. Then let
L7 : = g −1 (L(α4 )) = {u · c · v | u, v ∈ {a, b}∗, |u| = |v|}. We use Lemma 1 to show that
L7 ∈
/ L(DTMFArej ). Assume to the contrary that it is, and choose m ≥ 1. Then there
exist n ≥ m and words pn , vn that satisfy the conditions of Lemma 1. We now distinguish
the following cases: First, assume that pn does not contain the letter c. Then choose a
d ∈ {a, b} that is not the first letter of vn , and define u : = d · c · a|pn |+1 . Then pn u ∈ L7 ;
30

<!-- page 31 -->

but as vn is not a prefix of u, this contradicts Lemma 1. Now assume that pn contains c.
Then pn = w1 cw2 with w1 , w2 ∈ {a, b}∗, and |w1 | ≥ |w2 | + n ≥ |w2 | + m. Again, choose
d ∈ {a, b} such that d is not the first letter of vn , and define u : = d · a|w1 |−|w2 |−1 . Then
pn u = w1 cw3 for w3 = w2 d · a|w1 |−|w2 |−1 , and |w3 | = |w1 |. Hence, pn u ∈ L7 , but as vn is
not a prefix of u, this contradicts Lemma 1.
Intersection: In order to show both claims on the intersection of L(DRX), it suffices to
show that we can obtain a language that is not a DRX-language by intersecting two deterministic
regular languages. Accordingly,
we define deterministic regular expressions β1 : =(a(b ∨ ε))∗ and

β2 : = ε ∨ a(b(a ∨ ε))∗ (these expressions have been obtained by very minor modifications to
the expressions that Caron, Han, Mignot [12] use to show that L(DREG) is not closed under
intersection).
1
Let L8 = {(ab) 2 i | i ≥ 0}. To show that L8 = L(β1 ) ∩ L(β2 ), we follow the approach
from [12], and first consider M(β1 ) and the corresponding minimal incomplete DFA:
a
a

a

b

b

Likewise, we consider M(β2 ) and the corresponding minimal incomplete DFA (which merges the
two states for a):
b
a

b

b

a

a

a

Now it is easily seen that L8 = L(β1 ) ∩ L(β2 ). From Lemma 5, we know that L8 ∈
/ L(DRX).
Hence, the class of deterministic regex languages is not closed under intersection with deterministic regular languages, which also implies that it is not closed under intersection.
We leave open whether L(DTMFArej ) is closed under intersection (with itself or with L(DREG)),
but we conjecture that this is not the case. In this regard, note that while L(TMFA) is closed
under intersection with L(REG) (as show in [46]), it is open whether L(TMFA) closed under
intersection with itself.
We also leave open whether L(DRX) and L(DTMFArej ) are closed under Kleene plus or star.

6

Static Analysis

In this section, we examine a restriction DRX and DTMFA, which we motivate with the following
observation: As shown by Carle and Narendran [11], the intersection emptiness problem for
regex is undecidable7 . For DRX, that proof cannot be used, but the result still holds (and by
Theorem 4, this extends to DTMFA):
Theorem 9. Given α, β ∈ DRX, it is undecidable whether L(α) ∩ L(β) = ∅.
Proof. We show this with a reduction from Post’s Correspondence Problem (PCP, for short). Let
(u1 , v1 ), . . . , (uk , vk ) ∈ Σ∗ × Σ∗ , k ≥ 1, be a PCP instance. Our goal is to construct α, β ∈ DRX
such that L(α) ∩ L(β) 6= ∅ if and only if there exists a sequence i1 , . . . , in , n ≥ 1 and 1 ≤ ij ≤ k,
such that ui1 · · · uin = vi1 · · · vin . To do so, we first introduce an alphabet A : ={a1 . . . , ak }
7 Although that proof refers to a subclass of our definition of regex, see Section 2.1.1, it directly translates to

RX

31

<!-- page 32 -->

such that A, Σ, and {#, $, ¢} are pairwise disjoint (at the end of the proof, we discuss how this
construction can be adapted to binary terminal alphabets). We then define
k
∗
_
ai # ui hx : Σ∗ i # vi hy : Σ∗ i $ &x # &y ¢ ,
α:=
i=1


∗
β : = A #hz : Σ+ i # &z $ hx : Σ+ i #hy : Σ+ i ¢ A # &x # &y $ # ¢ .

To see that α is deterministic, note that the disjunction ranges over the letters from A. For β,
we observe that after each iteration of the starred subexpression, we read either a letter from Σ,
and start a new iteration, or #, which means that this was the last iteration.
We now claim that w ∈ L(α) ∩ L(β) if and only if there exist an n ≥ 1 and i1 , . . . , in with
1 ≤ ij ≤ k such that ui1 · · · uin = vi1 · · · vin and w = w1 ¢ w2 ¢ · · · wn ¢, where
wj = aij # uij · · · uin # vij · · · vin $ uij+1 · · · uin # vij+1 · · · vin .
Take note that wn always ends on $ #. Informally explained, w encodes how a solution of
the PCP instance is constructed, where the finished solution is in w1 , and the start of the
construction is at wn . Starting at w1 , the sequence of wj can be understood as splitting off pairs
of prefixes (uj , vj ) from the solution, where each word wj also encodes which tuple (uj , vj ) is
processed (by using the preceding symbol aj as a marker), and the words before and after the
pair is split off (to the left and right of $, respectively).
Here, α ensures that in each wj , uj and vj are split off correctly, while β ensures that the
“after” words of wj are the “before” words of wj+1 . Hence, such a w exists if and only if
the instance of the PCP has a solution. As the existence of the latter is undecidable (see e. g.
Hopcroft and Ullman [32]), deciding L(α) ∩ L(β) 6= ∅ is also undecidable.
To adapt the construction to a binary alphabet (say, {a, b}), we use a morphism h : (A ∪
Σ ∪ {#, $, ¢})∗ → {a, b}∗ that is defined as follows (we assume an arbitrary ordering on Σ with
Σ = {b1 , . . . , b|Σ| }):
• h(ai ) : = abi a for all ai ∈ A,
• h(bi ) : = abi a,
• h(#) : = bab, h($) : = ba2 b, and h(¢) : = ba3 b.
If we apply h to α and β by applying h to each terminal, we obtain regex h(α) and h(β) such that
L(h(α)) ∩ L(h(β)) 6= ∅ if and only if the instance of the PCP has a solution. The only problem
is that these regex are not deterministic, as there are disjunctions that start with the same
terminal letter. But each of these disjunctions can be rewritten into a deterministic disjunction
by nesting the branches. For example, consider the disjunction (a1 ∨ a2 ∨ a3 ). Using h, this
becomes (aba ∨ ab2 a ∨ ab3 a), which is not deterministic, but can be rewritten to the equivalent
(ab(a ∨(b(a ∨ ba)))).
Now, note that if we apply this rewriting to the disjunctions to h(α) and h(β) (including the
disjunctions that are hidden in shorthand notations A and Σ), we obtain deterministic regex. In
particular, note that Kleene plus and Kleene star are only used on elements of A ∪ Σ, and are
always followed by either # or ¢. As the encodings of the former start with a, while the encodings
of the latter start with b, rewriting the disjunctions is enough to ensure determinism.
As a consequence, DTMFA intersection emptiness problem is also undecidable. Theorem 9
applies even to very restricted DRX, as no variable binding contains a reference to another
variable, | var(α)| = 2, and | var(β)| = 3. Hence, bounding the number of variables does not
make the problem decidable. Instead, the key part seems to be that the variables occur under Kleene stars, which means that they can be reassigned an unbounded amount of times.
Following similar observations, Freydenberger and Holldack [25] introduced the following concept: A regex is variable-star-free (vstar-free) if each of its plussed sub-regexes contains neither
variable references, nor variable bindings. Analogously, we call a TMFA memory-cycle-free if it
contains no cycle with a memory transition (a transition in a TMFA that is a memory recall,
or that contains memory actions other than ⋄). Let RXvsf be the set of all vstar-free regex,
32

<!-- page 33 -->

and DRXvsf = RXvsf ∩ DRX. Let TMFAmcf be the set of all memory-cycle-free TMFA, and define DTMFAmcf , TMFArej
mcf ,. . . analogously. The proof of Theorem 4 allows us to conclude that
M(α) ∈ DTMFAmcf holds for every α ∈ DRXvsf . Likewise, we can use the proof of Theorem 1
to conclude L(TMFAmcf ) = L(RXvsf ). Note that for ε-free DTMFAmcf , the membership problem
can be decided in time O(|Q| + |w|), as the preprocessing step of Theorem 2 is not necessary (as
only a bounded number of variable references is possible in each run). Likewise, we can drop
the factor k from Theorem 5 when restricted to DRXvsf .
T
As shown by Freydenberger [24], it is decidable in PSPACE whether ni=1 L(αi ) = ∅ for
α1 , . . . , αn ∈ RXvsf . By combining the proof for this with some ideas from another construction
from [24], we encode the intersection emptiness problem for TMFAmcf in the existential theory
of concatenation with regular constraints (a PSPACE-decidable, positive logic on words, see
Diekert [17], Diekert, Jeż, Plandowski [19]). This yields the following:
Tn
Theorem 10. Given M1 , . . . , Mn ∈ TMFAmcf , we can decide i=1 L(Mi ) = ∅ in PSPACE. The
problem is PSPACE-hard, even if restricted to L(α) ∩ L(β), α ∈ DRXvsf and β ∈ DREG (if the
size of Σ is not bounded), or to L(α) ∩ L(M ), α ∈ DRXvsf and M ∈ DFA.
Proof. We begin with the first lower bound: As shown by Martens, Neven, and Schwentick [40]
(Theorem 3.10), the intersection emptiness problem for deterministic regular expressions is
PSPACE-complete (if |Σ| is not bounded; the paper does not discuss the unbounded case, and the
proof cannot be adapted
directly). This problem is defined as follows: Given β1 , . . . , βn ∈ DREG
Tn
for some n ≥ 2, is i=1 L(βi ) = ∅? We use a new terminal letter # ∈
/ Σ, and define

n−1
α : =hx : Σ∗ i # &x #
β : = β1 # β2 # · · · βn # .
First, observe
/ Σ. Now w ∈ L(α) holds if and only if
n that α and β are deterministic, as # ∈
w = ŵ # for some ŵ ∈ Σ∗ , and w ∈ L(β) if and only there exist w1 , . . . ,Twn ∈ Σ∗ with
wi ∈ L(βi ) and w = w1 # w2 # · · · wn #. Hence, (L(α) ∩ L(β)) 6= ∅ if and only if ni=1 L(βi ) 6= ∅.
?

As this problem is PSPACE-complete, deciding (L(α) ∩ L(β)) = ∅ is PSPACE-hard.
The second lower bound is a reduction from the intersection emptiness problem for DFA,
which is defined as follows: Given M1 , . . . , Mn ∈ DFA with n ≥ 2, is there a w ∈ Σ∗ with
w ∈ L(Mi ) for all 1 ≤ i ≤ n? This problem is PSPACE-complete (cf. Kozen [35]). We take a
new terminal symbol # ∈
/ Σ, define α as above, and choose M to be the DFA for the language
L(M1 ) # L(M2 ) # · · · # L(Mn ) (as # does not occur in the languages of the DFA, this is trivially
possible). The reasoning continues as above; but as the DFA can be defined on a binary alphabet,
this proof does not require an unbounded alphabet.
The upper bound takes more work, including further definitions. Our goal is to encode the
intersection emptiness problem for TMFAmcf in ECreg , the existential theory of concatenation
with regular constraints, which we now introduce (for a more detailed definition and examples
on ECreg , see for example Freydenberger [24]).
One of the basic elements of ECreg -formulas are word equations: A pattern is a word α ∈
(Σ∪Ξ)∗ , and a word equation is a pair of patterns (ηL , ηR ), which can also be written as ηL = ηR
(hence the name equation). A pattern substitution is a homomorphism σ : (Ξ ∪ Σ)∗ → Σ∗ with
σ(a) = a for all a ∈ Σ. It is a solution of a word equation (ηL , ηR ) if σ(ηL ) = σ(ηR ), and
we write this as σ |= (ηL , ηR ). Less formally, a pattern substitution replaces all variables with
terminal words (where multiple occurrences of the variable have to be substituted in the same
way), and it is a solution of an equation if both sides have the same terminal word as a result.
The other basic building block are constraint symbols: For every ε-NFA A and every x ∈ Ξ,
we can use a constraint symbol CA (x). A pattern substitution σ satisfies CA (x) if σ(x) ∈ L(A).
We write this as σ |= CA (x).
The existential theory of concatenation with regular constraints ECreg is obtained by combining word equations and constraint symbols using ∧, ∨ and existential quantification over
variables. Semantics are defined canonically: We have σ |= (ϕ1 ∧ ϕ2 ) if σ |= (ϕ1 ) and σ |= ϕ2 ;
and σ |= (ϕ1 ∨ ϕ2 ) if σ |= (ϕ1 ) or σ |= ϕ2 . Finally, σ |= (∃x : ϕ) if there exists a w ∈ Σ∗
such that σ[x→w] |= ϕ, where the pattern substitution σ[x→w] is defined by σ[x→w] (x) : = w, and
σ[x→w] (y) : = σ(y) if y 6= x. In slight abuse of notation, we also write w |= ϕ(x) if σ |= ϕ(x)
holds for the pattern substitution σ(x) : = w.
33

<!-- page 34 -->


For example, let ϕ(x) : = ∃y : (x = yby) ∧ CA (y) , where A is an NFA with L(A) = {a∗ }.
Then w |= ϕ(x) if and only if w = an ban for some n ≥ 0.
Given an ECreg -formula ϕ, deciding the existence of a pattern substitution σ with σ |= ϕ is
PSPACE-complete, cf. Diekert[17].
We first prove the claim only for automata with rejecting trap states (as we shall see further
down, the case for accepting trap states requires only a small modification). Before we proceed
to the main idea of the construction, we first take a closer look at the accepting runs of memorycycle-free TMFA.
Let M ∈ TMFArej
mcf with M = (Q, Σ, δ, q0 , F ) and memories {1, . . . , k}, and consider any
accepting run of M . As M is memory cycle free, whenever it takes a memory transition from
a state p to a state q, we know that p cannot occur anywhere else in the run. Otherwise, it
would be possible to repeat the memory transition from p to q arbitrarily often, which would
contradict the assumption that M is memory cycle free. Hence, we know that every accepting
run of M can use at most |Q| − 1 memory transitions.
This allows us to condense any accepting run of M by considering only its memory transitions.
Formally, for some 0 ≤ ℓ < |Q|, we define a condensed run κ = (~q, ~p, τ~ ) of length ℓ as follows:
1. q~ = (q0 , . . . , qℓ ) is a sequence of states, where q0 is the starting state of M ,
2. ~p = (p0 , . . . , pℓ ) is a sequence of states, with pℓ ∈ F ,
3. τ~ = (τ1 , . . . , τℓ ) is a sequence of memory transitions, where for each 1 ≤ i ≤ ℓ, either
τi = (pi , xi , qi+1 , si,1 , . . . , si,k ) with xi ∈ {1, . . . , k}, or
τi = (pi , bi , qi+1 , si,1 , . . . , si,k ) for some bi ∈ (Σ ∪ {ε})
and si,j ∈ {o, c, r, ⋄}, 1 ≤ j ≤ k. In the second case, we also require that there is at least
one j with si,j 6= ⋄.
4. pi is reachable from qi without using memory transitions for all 0 ≤ i ≤ ℓ.
The intuition behind this construction is that we condense the run to a sequence of states
(q0 , p0 , q1 , p1 , . . . , qℓ , pℓ ) that only contains the starting state q0 , a final state pℓ , and the states
before and after each memory transition (as M runs from qi to pi only without memory transitions, and each memory transition τi takes the automaton from pi to qi+1 ).
As we shall see, each condensed run κ can be converted in polynomial time into an ECreg
formula ϕκ (w) that defines exactly the language of all w ∈ L(M ) for which there is an accepting
run of M that can be condensed to κ. In particular, the parts of the run between each pair of
states qi and pi (which involve no memory transitions) shall be handled by appropriate regular
constraints.
Tn
Now, given M1 , . . . , Mn ∈ TMFArej
i=1 L(Mi ) = ∅.
mcf , we proceed as follows to decide whether
First, we guess a condensed run κi for each Mi (as the length of each sequence is bounded by
the number of states of Mi and as PSPACE = NPSPACE, this is allowed). Next, we convert each
κi into an ECreg -formula ϕi (w) (as we shallVsee, this is possible in polynomial time). Finally,
n
we combine these into the formula ϕ(w) : = i=1 ϕi (w), and decide whether ϕ is satisfiable (as
mentioned above, this is possible in PSPACE, see Diekert [17]). As ϕ is satisfiable if and only if
Tn
there exists a w ∈ i=1 L(Mi ), this proves the claim for TMFArej
mcf (as mentioned above, we shall
discuss the case of accepting failure states at the end of the proof).
We now discuss how to construct ϕκ from κ. Consider a word w ∈ L(M ), and the condensed
run κ of length ℓ for any accepting run of M on w. Then w can be decomposed into w =
u0 v1 u1 · · · vℓ uℓ with ui , vi ∈ Σ∗ such that ui is the word that M consumes when processing from
qi to pi (without using memory tranisitons), and vi is the word that is consumed when processing
from pi−1 to qi (using the memory transition τi ). This is illustrated by the following picture:
q0

u0

p0

v1
τ1

q1

u1

p1

v2
τ2

34

···

uℓ−1

pℓ−1

vℓ
τℓ

qℓ

uℓ

pℓ

<!-- page 35 -->

Following this intuition, we define
ϕκ (w) : = ∃u0 , . . . , uℓ , v1 , . . . , vℓ :
(w = u0 v1 u1 · · · vℓ uℓ ) ∧

ℓ
^

CMqi ,pi (ui ) ∧

i=0

^

i∈T

(vi = bi ) ∧

^

(vi = ηi ),

i∈R

where the following holds:
1. for p, q ∈ Q, Mq,p is the ε-NFA that is obtained from M by removing all memory transitions,
using q as starting and p as only finite state,
2. T ⊆ {1, . . . , ℓ} is the set of all i such that τi is not a memory recall (i. e., τi consumes a
terminal symbol bi ),
3. R ⊆ {1, . . . , ℓ} is the set of all i such that τi is a memory recall (i. e., τi recalls memory
xi ),
4. for each i ∈ R, we define ηi to describe the current content for xi (as the memory actions
are completely determined by the τj , this is directly possible by checking the τj with j ≤ i).
There are three possible cases:
(a) if xi was never changed in a memory transition τj with j < i, then its value defaults
to ε, and we define ηi : = ε,
(b) if xi was reset in a memory transition τj with j < i, and not changed between τj and
τi , we define ηi : = ε,
(c) otherwise, there exist well-defined 1 ≤ j < j ′ ≤ i such that xi was opened in transition τj and closed in τj ′ , and not changed between τj and τi . Hence, we define
ηi : = vj uj · · · uj ′ .
The constraints CMqi ,pi (ui ) check that each ui conforms to a sequence of transitions that takes M
from qi to pi , without using memory transitions. The conjunction over the i ∈ T ensures that the
memory actions that are not memory recalls consume terminals correctly, and the conjunction
over the i ∈ R ensures that each memory recall refers to the right part of the consumed input.
Hence, for all w ∈ Σ∗ , w |= ϕκ if and only if w ∈ L(M ), and there is an accepting run of M on
w that can be condensed to κ. It is easily seen that ϕκ can be constructed in polynomial time
(and, hence, its size is polynomial in |Q|): We need to construct ℓ + 1 ≤ |Q| automata Mqi ,pi ,
each of which has at most |Q| states and at most |Q|2 transitions. Determining each ηi is also
possible in time O(|Q|), be checking the previous transitions τj .
This concludes the proof for the case of Mi ∈ TMFArej
mcf . For the case where the failure state
is accepting, we need to add a small extension: Instead of only considering condensed runs for
runs that reach a final state, we also need to consider the runs that end in a memory recall that
fails. Hence, we consider a condensed run κ of length ℓ < |Q|, such that τℓ is a memory recall
transition for a variable x ∈ {1, . . . , k}. Then we construct ϕκ almost as explained above. The
only difference is that we replace the equation (vℓ = ηℓ ) in the conjunction over the elements of
R with the following formula:



∃z : (ηℓ = vℓ z) ∧ CA (vℓ ) ∧ CA (z)

∨

_

_

a∈Σ b∈Σ\{a}


∃y, z1 , z2 : (vℓ = yaz1 ) ∧ (ηℓ = ybz2 ) ,

where A is the minimal DFA for L(A) = Σ+ . The left part of this disjunctions describes all
cases where vℓ is non-empty and a proper prefix of the content of x; the right part describes all
cases where vℓ and the content of x differ at at least one position (recall that the content of x
at this point of the run is represented by ηℓ ). Hence, this formula describes all cases where this
condensed run ends in a memory recall failure.
This shows that the approach also works for M ∈ TMFAacc
mcf . Hence, given M1 , . . . , Mk ∈
TMFAmcf , we can decide whether the intersection of all L(Mi ) is empty by guessing a condensed
35

<!-- page 36 -->

run κi for each Mi . If Mi ∈ TMFArej
mcf , we only need to consider runs that end in final states; if
Mi ∈ TMFAacc
,
we
also
need
to
consider
runs that end in memory recall failures. Either way,
mcf
the length of κi is bounded by the number of states in Mi . We then
Vn transform in polynomial
time each κi into an ECreg -formula ϕi , and combine these into ϕ : = i=1 ϕi . Then w |= ϕ if and
only if w ∈ L(Mi ) for all i; and as satisfiability of ECreg -formulas can be decided in PSPACE,
intersection emptiness is also decidable in PSPACE. As we already showed hardness at the very
beginning of this proof, we conclude that the problem is PSPACE-complete.
The unbounded size of Σ comes from the PSPACE-hardness of the intersection emptiness
problem for DRX by Martens et al. [40], which has the same requirement. Using the existential
theory of concatenation for the upper bound might seem conceptually excessive, considering
how complicated even the satisfiability problem for word equations is (see Diekert [17, 18] for a
detailed and a recent survey). But even intersection emptiness for DRXvsf is at least as hard as
the satisfiability problem for word equations:
Proposition 6. Given a word equation η over Σ, we can construct in linear time αL , αR ∈
DRXvsf over Σ ∪ {#} such that L(αL ) ∩ L(αR ) 6= ∅ holds if and only if η has a solution.
Proof. Let η = (ηL , ηR ), and assume that x1 , . . . , xk are the variables that occur in η. Let # be
a new terminal letter, # ∈
/ Σ, and define
αL : =hx1 : Σ∗ i #hx2 : Σ∗ i # · · · hxk : Σ∗ i # βL ,
αR : =hx1 : Σ∗ i #hx2 : Σ∗ i # · · · hxk : Σ∗ i # βR ,
where βL and βR are obtained from ηL and ηR (respectively) by replacing each occurrence of
a variable xi with the reference &xi . As # ∈
/ Σ, and as βL and βR consist only of a chain of
terminals and variable references, αL , αR ∈ DRX. Furthermore, w ∈ L(αL ) ∩ L(αR ) holds if and
only if there is a homomorphism σ : (Σ ∪ Ξ)∗ → Σ∗ with σ(a) = a for all a ∈ Σ such that
w = σ(x1 ) # σ(x2 ) # · · · σ(xk ) # σ(ηL )
= σ(x1 ) # σ(x2 ) # · · · σ(xk ) # σ(ηR ),
which holds if and only if there is a solution σ of η.
We now combine the proofs of Theorems 3 and 10, and observe:
Theorem 11. Given M1 , M2 ∈ DTMFAmcf , L(M1 ) ⊆ L(M2 ) can be decided in PSPACE.
Proof. We use the simple fact that L1 ⊆ L2 holds if and only if L1 ∩ (Σ∗ \ L2 ) = ∅. Due to
Proposition 4, we can assume that M2 is complete (see Section 3.1).
While we could use Theorem 10 together with Theorem 3 to show that the inclusion problem
is decidable, the proof of Theorem 3 uses a construction that can lead to an exponential blowup
in the number of states. The reason for this is that even in a complete DTMFA, we cannot simply
toggle the acceptance behaviour of states, as the automaton might continue its computation by
recalling memories that contain ε.
But as we shall see, it is possible to adapt the proof of Theorem 10 to handle this as well.
First, note that we do not need to consider how to handle memory recall failures, as this is
already part of the proof (we can simply add or remove the modifications that we discussed
for DTMFAacc ). The first modification is that the algorithm now guesses a condensed run κ
that ends in an state pℓ that is not accepting. But to ensure that we can treat this state as an
accepting state, we need to ensure that no accepting state can be reached from it. Instead of
putting this into the formula, we make an additional guess: For each variable x ∈ {1, . . . , k},
we also guess a language Lx such that Lx = {ε} or Lx = Σ+ . Formally, in addition to κ and ℓ,
the algorithm guesses a function f : {1, . . . , k} → {Mε , MΣ+ }, where Mε and MΣ+ are NFA with
L(Mε ) = {ε} and L(MΣ+ ) = Σ+ .
It then checks whether it is possible to reach an accepting state from q, using only ε-transitions
and memory recalls for variables x with f (x) = Mε . If that is the case, the algorithm rejects

36

<!-- page 37 -->

the guess. Otherwise, it constructs a formula ϕκ,f , which is obtained from ϕκ by adding the
following formula to the conjunction:
^

(yx = η̂x ) ∧ Cf (x) (yi ) ,
∃y1 , . . . , yk :
x∈{1,...,k}

where each η̂x is chosen to represent the content of the variable x in pℓ , like the ηj in the proof
of Theorem 10. Hence, this formula checks whether the contents of the variables when reaching
the state pℓ conform to the guessed function f .
Hence, for every word that would be rejected by M2 , we can guess appropriate ℓ, κ, and f ,
which allows us to decide the intersection emptiness of L(M1 ) and (Σ∗ \ L(M2 )) in PSPACE as
in the proof of Theorem 10. Hence, inclusion is decidable in PSPACE.
Obviously, this implies that equivalence for DTMFAmcf is decidable in PSPACE, and, furthermore, this also holds for DRXvsf , which is an interesting contrast to non-deterministic RXvsf : As
shown by Freydenberger [23], equivalence (and, hence, inclusion and minimization) are undecidable for RXvsf (while [23] does not explicitly mention the concept, the regex in that proof are
vstar-free, as discussed in [25]). Hence, Theorem 11 also yields a minimization algorithm for
DRXvsf and DTMFAmcf that works in PSPACE (enumerate all smaller candidates and check equivalence). We leave open whether this is optimal, but observe that even for DREG, minimization
is NP-complete, see Niewerth [43].

7

A Relaxation of Determinism

One could argue that Definition 3 is overly restrictive; e. g., consider α : =hx : a+ ihy : b+ ic(&x ∨ &y).
Then α is not deterministic; but as the contents of x and y always start with a or b (respectively), deterministic choices between &x and &y are possible by looking at the current letter
of the input word. Analogous observations can be made for TMFA. These observations lead to
the following definition of ℓ-determinism.
Let ℓ ≥ 1 and let u, v ∈ Σ∗ . The words u and v are ℓ-prefix equivalent, denoted by u ≡ℓ v, if
u is a prefix of v, v is a prefix of u or their longest common prefix has a size of at least ℓ. By
u 6≡ℓ v, we denote that u and v are not ℓ-prefix equivalent. Note that in order to check for two
words u and v whether or not u ≡k v, it is sufficient to compare the first min{k, |u|, |v|} symbols
of u and v.
Based on this, we define the notion of ℓ-deterministic TMFA (for short: ℓ- DTMFA) as a
relaxation of the criteria of DTMFA: In contrast to the latter, an ℓ-deterministic M ∈ TMFA(k)
can have states q with multiple outgoing memory recall-transitions, as long as
1. these recall distinct memories, and
2. for every reachable configuration (q, v, (u1 , r1 ), . . . , (uk , rk )) of M , ui 6≡ℓ uj holds for all
i 6= j that appear on the recall transitions of q.
For technical reasons, we define 0-DTMFA to coincide with DTMFA.
Next, we note that this relaxation from determinism to ℓ-determinism does not increase the
expressive power of DTMFA.
S
Proposition 7. DTMFA = ℓ≥0 ℓ- DTMFA.
S
Proof. The inclusion DTMFA ⊆ ℓ≥0 ℓ- DTMFA holds by definition. In order to show the converse inclusion, let M ∈ ℓ- DTMFA(k), for some ℓ ≥ 1. We transform M into an equivalent
DTMFA M ′ as follows. We implement k auxiliary memories (called state-memories in the
following) in the finite state control, which can store words of length at most ℓ, i. e., we replace every state q by states [q, m1 , m2 , . . . , mk ], where mi ∈ Σ∗ , |mi | ≤ ℓ, 1 ≤ i ≤ k. The
general idea is that M ′ simulates M in such a way that whenever M reaches a configuration
(q, v, (u1 , r1 ), . . . , (uk , rk )), then M ′ reaches the configuration with state [q, m1 , m2 , . . . , mk ] and
memory configurations (u′i , ri ), 1 ≤ i ≤ k, such that, for every i, 1 ≤ i ≤ k, ui = mi u′i and if
u′i 6= ε, then |mi | = k. This can be achieved as follows.
37

<!-- page 38 -->

Initially, all memories and state-memories are empty and closed (to this end, the finite state
control contains a flag for each state-memory, indicating whether or not it is open). If M recalls
memory i, then M ′ consumes the content of the state-memory i from the input, symbol by
symbol, and then applies a memory recall instruction on memory i (note that memory i might
be empty). If the consumption of the content of state-memory i fails, i. e., it is not a prefix of
the remaining input, then we move to the state [trap].
Whenever M opens memory i, M ′ empties the state-memory i and marks it as open, but
does not yet open memory i. The scanned input is now stored as follows. If a single symbol is
read and the state-memory currently stores a word of length at most ℓ − 1, then this symbol
is appended to the state-memory (furthermore, if the new symbol exhausts the state-memory’s
capacity, then memory i is opened), and if the state-memory already stores a word of length ℓ,
then the symbol is automatically stored in the open memory i.
On the other hand, if M consumes a prefix u of the input by a memory recall instruction
for some memory j, i. e., in M ′ , the state-memory j stores some u′ and memory j stores some
u′′ with u = u′ u′′ , then this is simulated by M ′ as follows. We start consuming u′ symbol by
symbol and store every symbol in state-memory i. If this is possible without exhausting the
capacity of state-memory i (i. e., state-memory i now stores a word of length at most ℓ − 1),
then |u′ | < ℓ, which implies u′′ = ε and we are done. On the other hand, if the consumption of
u′ exhausts the state-memories capacity, i. e., u′ = v ′ v ′′ , where v ′ is the largest prefix that fits
in state-memory i (note that v ′ = u is possible), then we open memory i and fill it with v ′′ u′′
by first consuming v ′′ symbol by symbol and then consulting memory j.
We implement the modifications from above in such a way that whenever in M there is a
nondeterministic choice of the form that, for some state q and several i1 , i2 , . . . , is , 1 ≤ ij ≤ k,
1 ≤ j ≤ s, each δ(q, ij ), 1 ≤ j ≤ s, is defined (note that, since M is ℓ-deterministic, these are the
only possible non-deterministic choices), then this is implemented in M ′ by s many ε-transitions
from the states [q, m1 , m2 , . . . , mk ]. Since the modifications from above do not require any
nondeterminism, there is a one-to-one correspondence between the nondeterministic choices of
M and M ′ . We further note that, for every j, 1 ≤ j ≤ s, the ε-transition for consulting memory
ij is followed by a path of states, in which the content of state-memory ij is consumed symbol
by symbol, followed by a recall of memory ij (and, simultaneously, for every open memory i, the
state-memory is filled with the consumed symbols until it is full and then memory i is opened).
The memory recall performed by this path of states either fails, which can happen in the phase
where the content of the state-memory is matched with the input or in the actual recall of
the memory, or it successfully simulates the memory recall. We shall now describe how the
nondeterministic choices of M ′ can be removed.
Instead of nondeterministically choosing one of these paths, we carry them out in parallel as
follows. We start consuming a prefix of the remaining input and compare it, symbol by symbol,
with the contents of the state-memories ij , 1 ≤ j ≤ s. Whenever the next input symbol does
not match the next symbol of a state-memory ij , we mark this memory as inactive and ignore
it from now on. If all memories are inactive, we change to state [trap] and if there is exactly one
active memory ij left, we conclude the consultation of this memory (i. e., we match the remaining
part of the state-memory ij with the input and then consult memory ij ). In particular, we note
that if a state-memory has been completely and successfully matched with a prefix of the input
and there is a another memory still active, then the contents of these memories are ℓ-prefix
equivalent, which is a contradiction to the ℓ-determinism of M . Consequently, we encounter
the situation that either all memories are inactive or that exactly one active one is left, before
a state-memory is completely matched with a prefix of the input. Obviously, this procedure is
completely deterministic and it results in an equivalent automaton.
Next, we consider the problem to decide whether a given TMFA is ℓ-deterministic for some
given ℓ ≥ 1, and we shall see that this is a hard problem (even for TMFAmcf ). Moreover,
extending the notion of ℓ-determinism to RX by defining α ∈ RX to be ℓ-deterministic if and
only if M(α) is, we shall see that deciding ℓ-determinism is also hard for RX (and even RXvsf ).
Proposition 8. For every ℓ ≥ 1, deciding whether a TMFA is ℓ-deterministic is PSPACEcomplete. The problem is coNP-complete if the input is restricted to TMFAmcf . These lower
bounds hold even if we restrict the input to RX and RXvsf , respectively.

38

<!-- page 39 -->

Proof. Before we proceed to the actual proof, we briefly discuss why it is possible to treat nondeterministic regex as an input for the problem, considering that the number of transitions in
M(α) can be exponential (in the number of variables of α). While this is true in general, the
non-deterministic regex that have these blowups are also not ℓ-deterministic: As soon as an
M ∈ TMFA has more than one transition from one state to another, it is not ℓ-deterministic.
Hence, we can use an algorithm that decides ℓ-determinism for TMFA to decide ℓ-determinism
for RX by converting every input α ∈ RX into M(α) according to the proof of Theorem 4, but
aborting if Gα̃ contains nodes u and v with at least two edges from u and v (if these occur, α
can be rejected as not ℓ-deterministic, regardless which ℓ was chosen).
Upper bounds: In order to prove the upper bounds, let M ∈ TMFA and ℓ ≥ 1. Assume
that M is not deterministic, but only violates the criteria by having states with multiple outgoing
memory recall transitions for different variables (if any other violation of the criteria occurs, M
cannot be ℓ-deterministic). Now, M is not ℓ-deterministic if and only if there exists a state
q in M that has outgoing memory recall transitions for two different variables x and y, and
there is a run of M that reaches q while x and y contain words wx and wy (respectively) such
that wx ≡ℓ wy . We show this property can be decided in PSPACE in general, and in NP if M is
memory-cycle-free. The claim of the Proposition follows then directly if M is memory-cycle-free,
and from the closure of PSPACE under complementation in the general case.
We first consider the general case: The PSPACE algorithm guesses a state q that has two
outgoing memory recall transitions for variables x and y. It then guesses its way from q0 through
the automaton, while storing for each variable z of M (not just x and y) the first ℓ letters of the
stored word wz (in order to determine these for a memory z, it suffices to know all terminal edges
that are traversed while z is open, and at most ℓ letters of each memory z ′ that is referenced
while z is open). If the algorithm reaches q while wx ≡ℓ wy , the algorithm correctly identifies M
as not ℓ-deterministic. Hence, this can be decided in PSPACE.
For the memory-cycle-free case, we combine this with the condensed runs from the proof
of Theorem 10. The NP-algorithm first guesses a condensed run κ of M that ends at q with
outgoing memory recall transitions for x and y. In order to determine the first ℓ letters of each
variable, it then guesses a prefix ui of length at most ℓ for each transition from a qi to a pi , and
checks whether there is a word in L(Mqi ,pi ) that has ui as a prefix (where the ε-NFA Mqi ,pi is
obtained as in the proof of Theorem 10: Remove all memory transitions from M , and take qi
as starting and pi as only accepting state). It then computes the first ℓ letters of wx and wy ,
which suffice to determine whether wx ≡ℓ wy . If wx ≡ℓ wy , the algorithm correctly identifies M
as not ℓ-deterministic. Hence, for memory-cycle-free TMFA, the absence of ℓ-determinism can
be decided in NP, which means that ℓ-determinism can be decided in coNP.
Lower bound for TMFAmcf and RXvsf : We prove this claim with a reduction from the
3-satisfiability problem, which is well-known to be NP complete (cf. Garey and Johnson [28]).
Let ϕVbe a formula in 3-conjunctive normal form, with variables V = {v1 , . . . , vk }, k ≥ 1, where
n
ϕ : = i=1 ϕi with ϕi = (λi,1 ∨ λi,2 ∨ λi,3 ), and λi,j ∈ {v, ¬v | v ∈ V } for all 1 ≤ i ≤ n and
1 ≤ j ≤ 3.
Our goal is to construct a β ∈ RXvsf that is not ℓ-deterministic if and only if there is an
assignment to the variables in V that satisfies ϕ. As the latter problem is NP-complete, deciding
whether a vsf-regex is ℓ-deterministic is coNP-hard. To this end, we first construct an α ∈ DRXvsf
that has a variable z that can only contain ε if ϕ has a satisfying assignment, and that otherwise
contains a word from {a}+.
We then define β : = α · hz1 : bℓ ihz2 : &z bℓ i(&z1 ∨ &z2 ). Note that β is not ℓ-deterministic if
and only if ε can be assigned to z; as otherwise, z always contains some word from a+ b, which
means that z1 and z2 already differ on the first letter.
We implement this by modeling each variable vi ∈ V of ϕ with two variables xi and x̂i in α,
where an assignment of 1 to vi is modeled by setting xi to ε and x̂i to a, while assigning 0 is
modeled by setting xi to a and x̂i to ε. Keeping this in mind, we define α : = αinit · αsat , where
αinit : = α1init · · · αkinit ,

αiinit : = (ahxi : εihx̂i : ai) ∨(bhxi : aihx̂i : εi)
39

<!-- page 40 -->

for 1 ≤ i ≤ k, as well as
αsat : = α1sat · · · αnsat · hz : &y1 · · · &yn i,


i,2
i,3 
,
αisat : =(a · αi,1
)
∨
b
(a
·
α
)
∨(b
·
α
)
lit
lit
lit
(
hyi : &xl i if λi,j = vl ,
αi,j
lit : =
hyi : &x̂l i if λi,j = ¬vl
for 1 ≤ i ≤ n, and 1 ≤ j ≤ 3.
Now, observe that α is deterministic, as each part of a disjunction starts with a unique first
letter (a or b); and α is obviously vstar-free. To see that α can assign ε to z if and only if ϕ
has a satisfying assignment, we read α from left to right: First, αinit ensures that for each pair
of variables xi and x̂i , exactly one is bound to ε, and the other to a (recall that setting xi to ε
corresponds to assigning 1 to vi ). Next, for each clause ϕi , αisat stores the value of one of the
literals λi,j ∈ {vl , ¬vl } under the chosen assignment yi , by recalling the appropriate xl or x̂l .
Thus, yi can only contain ε if the assignment satisfies ϕi . Finally, all yi are concatenated, and
the result is stored in z. Hence, z can only contain ε if all clauses ϕi are satisfied, which means
that ϕ is satisfied. Likewise, each satisfying assignment can be used to make the appropriate
choices in the αiinit and αjsat such that z contains ε.
Hence, as explained above, β is ℓ-deterministic if and only if ϕ has no satisfying assignment,
which means that deciding whether a vstar-free regex is ℓ-deterministic is coNP-hard. As we
already showed the matching upper bound, the problem is coNP-complete.
Lower bound for TMFA and RX: We show this with a reduction from the intersection
emptiness problem for DFA, which is defined as follows: Given M1 , . . . , Mn ∈ DFA for some
n ≥ 2, is there a w ∈ Σ∗ with w ∈ L(Mi ) for all 1 ≤ i ≤ n? This problem is PSPACE-complete
(cf. Kozen [35]).
As in the case for vstar-free regex, we first construct an α ∈ DRX that has a variable z such
that it is possible to assign ε to z if and only if the intersection of the L(Mi ) is not empty (and
which is set to a word from {a}+ otherwise), and then define
β : = α · hz1 : bℓ ihz2 : &z bℓ i(&z1 ∨ &z2 ).
Again, β is not ℓ-deterministic if and only if ε can be assigned to z; as otherwise, z always
contains some word from a+ b.
Consider M1 , . . . , Mn ∈ DFA with Mi = (Σ, Qi , qi,0 , δi , Fi ). In order to simplify the construction, we assume Qi = {qi,0 , . . . , qi,m } for some m ≥ 1, and Σ ⊇ {a, b, c0 , . . . , cmax(m,n) }. We
shall discuss in the proof how the construction can be adapted to a binary alphabet, but using
an unbounded alphabet is simpler.
The main idea of the construction is that each state qi,j is represented by a variable xi,j ,
which can take either a or ε as values. The regex α contains a subexpression αiter which uses
a Kleene star to simulate all Mi in parallel on the same input. In particular, it ensures that
xi,j can be set to ε if and only if Mi can enter state qi,j at the current point of the parallel
simulation. Using this, we shall see that it is possible to set z to ε if and only if all Mi can reach
an accepting state at the same time. We define
∗
α : = αinit · a · αiter · b · αacc
Before we define the subexpressions of α, we observe that the use of the Kleene star does not
affect determinism, as the terminals a and b signal whether there should be another iteration of
the star or not (respectively). The subexpressions of α are defined as follows:
αinit : = α1init · · · αninit ,
αiinit : =hxi,0 : εihxi,1 : ai · · · hxi,m : ai ,

40

<!-- page 41 -->

for all 1 ≤ i ≤ n. This represents that each automaton Mi is in its starting state q0,i . Furthermore, to simulate the behaviour of the automata Mi , we define

αiter : = (a · αastep ) ∨(b · αbstep ) · αswitch ,
d,n
αdstep : = αd,1
step · · · αstep ,

_
αd,i
cj ·
step : =
0≤j≤m

_

0≤l≤m,
δi (ql,i ,d)=qj,i



cl · hx̂i,j : &xi,l i · αi,j
dump ,

αi,j
dump : =hx̂i,0 : ai · · · hx̂i,j−1 : aihx̂i,j+1 : ai · · · hx̂i,m : ai,
αswitch : = α1switch · · · αnswitch ,
αiswitch : =hxi,0 : &x̂i,0 i · · · hxi,m : &x̂i,m i
for d ∈ {a, b}, 1 ≤ i ≤ n, and 0 ≤ j ≤ m. Each αd,i
step picks a pair of states qi,j and qi,l of
Mi , such that δ(qi,l , d) = qi,j . Less formally, qi,j is the successor state of qi,l on input d. The
temporary variable x̂i,j is then set to the content of xi,l , while all other temporary variables x̂i,j ′
with j ′ 6= j are set to a, using αi,j
dump .
d
Hence, each iteration of αstep can set x̂i,j to ε if and only if qi,j is the successor state on
input d for a state qi,l such that xi,l contains ε. In other words, each iteration of αiter uses a
subexpression αdstep to simulates all Mi in parallel on the input letter d ∈ {a, b}, and αswitch sets
each xi,j to the same content as its corresponding temporary variable x̂i,j .
As an aside, note that it is possible to adapt the construction to a binary terminal alphabet.
To do so, one replaces the disjunctions over the terminals cj and cl with nested disjunctions
over a and b, as in the expressions αisat in the proof for the lower bound for RXvsf above.
Regardless of the number of terminal letters, we define the remaining subexpressions as
follows:
αacc : = α1acc · · · αnacc · hz : &y1 · · · &yn i,
_
αiacc : =
cj · hyi : &xi,j i
0≤j≤m,
qi,j ∈Fi

for all 1 ≤ i ≤ n. Again, this disjunction can be adapted to a binary terminal alphabet, as
described above.
It is possible to set z to ε if and only if every yi can be set to ε. In turn, this is possible if and
only if for every Mi , there is an accepting state qj,i such that xj,i can be set to ε. As established
above, αiter ensures that this is only possible if these states can be reached by simulating all Mi
in parallel on the same input. Hence, z can be set to ε if and only if the intersection of all L(Mi )
is not empty.
As discussed above, α is deterministic (we discussed the use of the Kleene star above, and
all branches disjunctions start with characteristic letters). Hence, β is ℓ-deterministic if and
only if the intersection of the L(Mi ) is empty. As β can obviously be constructed in polynomial
time, this shows that deciding whether a regex is ℓ-deterministic is PSPACE-hard. As we already
established the matching upper bound, this concludes the whole proof.
Furthermore, while this definition of ℓ-determinism is only concerned with choices between
different variables, it is also possible to adapt the notion of 1-determinism to include the distinction between a variable and a terminal. For example, the expression hx : a+ ib(b ∨ &x)∗ is
not deterministic; but as the content of x always starts with a, such cases could be considered 1-deterministic. Propositions 7 and 8 can be directly adapted to this extended notion of
1-determinism.

8

Conclusions and Further Directions

Based on TMFA, an automaton model for regex, we extended the notion of determinism from
regular expressions to regex. Although the resulting language class cannot express all regular
41

<!-- page 42 -->

languages, it is still rich; and by using a generalization of the Glushkov construction, deterministic regex can be converted into a DTMFA, and the membership problem can then be solved
quite efficiently. Although we did not discuss this, the construction is also compatible with the
Glushkov construction with counters by Gelade, Gyssens, Martens [29]. Hence, one can add
counters to DRX and DTMFA without affecting the complexity of membership.
Many challenging questions remain open, for example: Can the more advanced results for
DREG be adapted to DRX, i. e., can M(α) be computed more efficiently (as in [8, 44]), or is
it even possible, like in [31], to avoid computing M(α)? Is effective minimization possible for
DTMFA or DRX? Is it decidable whether a DTMFA defines a DRX-language? Are inclusion and
equivalence decidable for DRX or DTMFA? Can determinism be generalized to larger classes of
regex without making the membership problem intractable?

Acknowledgements
The authors thank Wim Martens for helpful feedback, Matthias Niewerth, for pointing out
that vn must be a factor of pn in the jumping lemma, and Martin Braun, for creating a library
and tool for DRX and DTMFA (available at [6]).

References
[1] Abigail. Re: Random number in perl. Posting in the newsgroup comp.lang.perl.misc,
October 1997. Message-ID slrn64sudh.qp.abigail@betelgeuse.wayne.fnx.com.
[2] Alfred V. Aho. Algorithms for finding patterns in strings. In Jan van Leeuwen, editor,
Handbook of Theoretical Computer Science, volume A, chapter 5, pages 255–300. Elsevier,
Amsterdam, 1990.
[3] Dana Angluin. Finding patterns common to a set of strings. J. Comput. Syst. Sci., 21:46–62,
1980.
[4] Pablo Barceló, Carlos A. Hurtado, Leonid Libkin, and Peter T. Wood. Expressive languages
for path queries over graph-structured data. In Proc. PODS 2010, 2010.
[5] Geert Jan Bex, Wouter Gelade, Frank Neven, and Stijn Vansummeren. Learning deterministic regular expressions for the inference of schemas from XML data. ACM Trans. Web,
4(4):14, 2010.
[6] Martin Braun. moar – Deterministic Regular Expressions with Backreferences, 2016. Accessed February 2018.
[7] Tim Bray, Jean Paoli, C. M. Sperberg-McQueen, Eve Maler, and François Yergeau. Extensible markup language XML 1.0 (fifth edition). W3C recommendation. Technical Report
https://www.w3.org/TR/2008/REC-xml-20081126/, W3C, November 2008.
[8] Anne Brüggemann-Klein. Regular expressions into finite automata. Theor. Comput. Sci.,
120(2):197–213, 1993.
[9] Anne Brüggemann-Klein and Derick Wood. One-unambiguous regular languages. Inf.
Comput., 142(2):182–206, 1998.
[10] Cezar Câmpeanu, Kai Salomaa, and Sheng Yu. A formal study of practical regular expressions. Int. J. Found. Comput. Sci., 14:1007–1018, 2003.
[11] Benjamin Carle and Paliath Narendran. On extended regular expressions. In Proc. LATA
2009, 2009.
[12] Pascal Caron, Yo-Sub Han, and Ludovic Mignot. Generalized one-unambiguity. In Proc.
DLT 2011, pages 129–140, 2011.

42

<!-- page 43 -->

[13] Marek Chrobak. Finite automata and unary languages. Theor. Comput. Sci., 47(3):149–158,
1986.
[14] Marek Chrobak. Errata to: “Finite automata and unary languages”. Theor. Comput. Sci.,
302(1):497–498, 2003.
[15] Russ Cox. Regular expression matching can be simple and fast (but is slow in Java, Perl,
PHP, Python, Ruby, . . . ). https://swtch.com/~rsc/regexp/regexp1.html, 2007. Accessed February 2018.
[16] Wojciech Czerwinski, Claire David, Katja Losemann, and Wim Martens. Deciding definability by deterministic regular expressions. In Proc. FOSSACS 2013, pages 289–304, 2013.
[17] Volker Diekert. Makanin’s Algorithm. In Algebraic Combinatorics on Words [38], chapter 12.
[18] Volker Diekert. More than 1700 years of word equations. In Proc. CAI 2015, 2015.
[19] Volker Diekert, Artur Jeż, and Wojciech Plandowski. Finding all solutions of equations in
free groups and monoids with involution. Inf. Comput., 251:263–286, 2016.
[20] Ronald Fagin, Benny Kimelfeld, Frederick Reiss, and Stijn Vansummeren. Document spanners: A formal approach to information extraction. J. ACM, 62(2):12, 2015.
[21] Henning Fernau and Markus L. Schmid. Pattern matching with variables: A multivariate
complexity analysis. Inform. Comput., 242:287–305, 2015.
[22] Henning Fernau, Markus L. Schmid, and Yngve Villanger. On the parameterised complexity
of string morphism problems. Theory Comput. Syst., 59(1):24–51, 2016.
[23] Dominik D. Freydenberger. Extended regular expressions: Succinctness and decidability.
Theory Comput. Sys., 53(2):159–193, 2013.
[24] Dominik D. Freydenberger. A logic for document spanners. In Proc. ICDT 2017, 2017.
[25] Dominik D. Freydenberger and Mario Holldack. Document spanners: From expressive
power to decision problems. Theory of Computing Systems, 2017.
[26] Dominik D. Freydenberger and Nicole Schweikardt. Expressiveness and static analysis of
extended conjunctive regular path queries. J. Comput. Syst. Sci., 79(6):892 – 909, 2013.
[27] Shudi (Sandy) Gao, C. M. Sperberg-McQueen, and Henry S. Thompson.
W3C
XML schema definition language (XSD) 1.1 part 1: Structures. Technical Report
https://www.w3.org/TR/2012/REC-xmlschema11-1-20120405/, W3C, April 2012.
[28] Michael R. Garey and David S. Johnson. Computers and Intractability. W. H. Freeman
and Company, 1979.
[29] Wouter Gelade, Marc Gyssens, and Wim Martens. Regular expressions with counting:
Weak versus strong determinism. SIAM J. Comput., 41(1):160–190, 2012.
[30] Jan Goyvaerts. Regular expressions tutorial. http://www.regular-expressions.info/tutorial.html,
2016. Accessed February 2018.
[31] Benoı̂t Groz and Sebastian Maneth. Efficient testing and matching of deterministic regular
expressions. J. Comput. Syst. Sci., 89:372 –399, 2017.
[32] John E. Hopcroft and Jeffrey D. Ullman. Introduction to Automata Theory, Languages,
and Computation. Addison-Wesley, 1979.
[33] Michael Kay. XPath and XQuery Functions and Operators 3.0 W3C Recommendation.
Technical Report https://www.w3.org/TR/2014/REC-xpath-functions-30-20140408/,
W3C, April 2014.

43

<!-- page 44 -->

[34] S. C. Kleene. Representation of events in nerve nets and finite automata. In C. E. Shannon,
J. McCarthy, and W. R. Ashby, editors, Automata Studies, pages 3–42. Princeton University
Press, Princeton, NJ, 1956.
[35] Dexter Kozen. Lower bounds for natural proof systems. In Proc. FOCS 1977, 1977.
[36] Markus Latte and Matthias Niewerth. Definability by weakly deterministic regular expressions with counters is decidable. In Proc. MFCS 2015, 2015.
[37] Katja Losemann, Wim Martens, and Matthias Niewerth. Closure properties and descriptional complexity of deterministic regular expressions. Theor. Comput. Sci., 627:54–70,
2016.
[38] M. Lothaire. Algebraic Combinatorics on Words, volume 90 of Encyclopedia of mathematics
and its applications. Cambridge University Press, 2002.
[39] Ping Lu, Joachim Bremer, and Haiming Chen. Deciding determinism of regular languages.
Theory Comput. Syst., 57(1):97–139, 2015.
[40] Wim Martens, Frank Neven, and Thomas Schwentick. Complexity of decision problems for
XML schemas and chain regular expressions. SIAM J. Comput., 39(4):1486–1530, 2009.
[41] D. W. Mount. Bioinformatics: Sequence and Genome Analysis. Cold Spring Harbor Laboratory Press, Woodbury, NY, 2004.
[42] Makoto Murata, Dongwon Lee, Murali Mani, and Kohsuke Kawaguchi. Taxonomy of XML
schema languages using formal language theory. ACM TOIT, 5(4):660–704, 2005.
[43] Matthias Niewerth. Data Definition Languages for XML Repository Management Systems.
PhD thesis, TU Dortmund, 2015.
[44] Jean-Luc Ponty, Djelloul Ziadi, and Jean-Marc Champarnaud. A new quadratic algorithm
to convert a regular expression into an automaton. In Proc. WIA ’96, 1996.
[45] Markus L. Schmid. Inside the class of regex languages. Int. J. Found. Comput. Sci.,
24(7):1117–1134, 2013.
[46] Markus L. Schmid. Characterising REGEX languages by regular languages equipped with
factor-referencing. Inform. Comput., 249:1–17, 2016.
[47] C. M. Sperberg-McQueen and H. Thompson. XML schema. http://www.w3.org/XML,
2005.
[48] M. Y. Vardi. From monadic logic to PSL. Pillars of Computer Science. Springer, Berlin,
2008.
[49] L. Wall, T. Christiansen, and J. Orwant. Programming Perl. O’Reilly Media, Sebastopol,
CA, 2000.

44
