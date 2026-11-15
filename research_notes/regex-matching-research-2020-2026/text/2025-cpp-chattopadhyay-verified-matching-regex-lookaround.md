---
file: ../pdfs/2025-cpp-chattopadhyay-verified-matching-regex-lookaround.pdf
citation: "Verified and Efficient Matching of Regular Expressions with Lookaround — Chattopadhyay, Li, Mamouras — CPP 2025"
doi: 10.1145/3703595.3705884
source: "https://doi.org/10.1145/3703595.3705884 (publisher version, added manually)"
topics: [counting_and_derivatives, redos_backrefs_verification, semantics_theory_access]
pages: 16
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Verified and Efficient Matching of Regular Expressions
with Lookaround
Agnishom Chattopadhyay

Angela W. Li

Konstantinos Mamouras

Rice University
Houston, USA
agnishom@rice.edu

Rice University
Houston, USA
awl@rice.edu

Rice University
Houston, USA
mamouras@rice.edu

Abstract

make it possible to match certain non-regular languages.
Certain constructs, such as capture groups or lookarounds,
do not add to the expressive power in terms of the overall
strings that can be matched, but are best described using a
different semantics (i.e., different from the simple Boolean
semantics of 𝑤 ∈ J𝑟 K and 𝑤 ⇑∈ J𝑟 K).
The focus of this paper is on expressions with lookarounds.
Lookahead assertions are used to assert that a certain pattern
is satisfied in the future of the current position in the string.
For example, the expression \d+\.\d\d(?=USD) could be used
to match a price of some item formatted as xxx.yy USD.
Since the pattern USD appears within a lookahead, this part
of the string is not returned, so that the programmer can
directly work with the numerical information. Similarly, if
the pattern USD was expected to appear before the price,
one could use the expression (?<=USD)\d+\.\d\d , where the
pattern (?<=USD) is called a lookbehind.
The majority of popular regex engines bundled with the
standard library of programming languages such as Java,
Javascript and Python are based on backtracking, which
may take exponential time in the worst-case scenario (called
catastrophic backtracking [6]). This behavior could be used
to conduct a denial-of-service attack [13]. Automata-based
tools such as Google’s RE2 [40] and Intel’s Hyperscan [23]
are guaranteed to finish execution in linear time. Despite this,
backtracking engines remain popular due to their flexibility
and support for non-classical features, including (but not
limited to) lookarounds.
The contributions of this paper are as follows: (1) We
formally verify in Coq an algorithm for matching regular
expressions with lookarounds, that runs in linear time in
terms of the size of the regular expression as well as the
length of the input string; (2) We present a purely functional
version of the Oracle NFA based algorithm in [31]; and (3)
We empirically demonstrate the linear time-complexity of
our algorithm.
Approaches for the linear-time matching of expressions
with lookarounds have been recently published in [31], [21]
and [5]. An approach that uses memoization and backtracking is described in [14]. In this paper, we follow the terminology and approach from [31]. The key idea in these
algorithms can be summarized as a layered dynamic programming algorithm. As input, we are given expressions
with lookarounds, which allow imposing conditions on the
part of the string which is “ahead” or “behind” the current

Regular expressions can be extended with lookarounds for
contextual matching. This paper discusses a Coq formalization of the theory of regular expressions with lookarounds.
We provide an efficient and purely functional algorithm for
matching expressions with lookarounds and verify its correctness. The algorithm runs in time linear in both the size
of the regular expression as well as the input string. Our
experimental results provide empirical support to our complexity analysis. To the best of our knowledge, this is the
first formalization of a linear-time matching algorithm for
regular expressions with lookarounds.
CCS Concepts: • Theory of computation → Regular languages; Automata extensions; Pattern matching; • Software
and its engineering → Software verification.
Keywords: lookbehind, lookahead, NFA, regex
ACM Reference Format:
Agnishom Chattopadhyay, Angela W. Li, and Konstantinos
Mamouras. 2025. Verified and Efficient Matching of Regular Expressions with Lookaround. In Proceedings of the 14th ACM SIGPLAN
International Conference on Certified Programs and Proofs (CPP ’25),
January 20–21, 2025, Denver, CO, USA. ACM, New York, NY, USA,
16 pages. https://doi.org/10.1145/3703595.3705884

1

Introduction

While textbook regular expressions involve only concatenation (𝑟 ⋅𝑠), alternation (𝑟 +𝑠) and Kleene iteration 𝑟 ∗ , modern
regex engines (such as PCRE [43]) support a multitude of additional features. Some of these features are purely syntactic
sugar (such as the use of character classes or the use of the ?
operator), some of these features (like counting) add exponential succinctness and some features (like backreferences)
Permission to make digital or hard copies of all or part of this work for
personal or classroom use is granted without fee provided that copies
are not made or distributed for profit or commercial advantage and that
copies bear this notice and the full citation on the first page. Copyrights
for components of this work owned by others than the author(s) must
be honored. Abstracting with credit is permitted. To copy otherwise, or
republish, to post on servers or to redistribute to lists, requires prior specific
permission and/or a fee. Request permissions from permissions@acm.org.
CPP ’25, January 20–21, 2025, Denver, CO, USA
© 2025 Copyright held by the owner/author(s). Publication rights licensed
to ACM.
ACM ISBN 979-8-4007-1347-7/25/01
https://doi.org/10.1145/3703595.3705884

198

<!-- page 2 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

position. We tackle these expressions by decomposing them
into multiple layers, with each level of nested lookaround
in a layer. At the bottom layer, we find ordinary regular expressions. When considering the subsequent upper layers,
the information relating to whether or not the expression
in the layer below was matched becomes necessary. Thus,
by proceeding in a layerwise fashion, the recomputation of
matching information in the lookarounds can be avoided.
In the intermediate stages of the algorithm, we use the notion of Oracle Regular Expressions to capture the concept of
expressions which rely on external oracle queries to determine their satisfaction. These expressions can be matched
in a manner similar to standard regular expressions (such as
by simulating an NFA, as seen in [31]) but with the added
complication of handling the oracle queries.
In this paper, we show how algorithms based on Marked
Regular Expressions [3, 19] can be extended to match Oracle
Expressions. Marked expressions allow simulating NFAs in a
manner that closely resembles the syntax of regular expressions, facilitating elegant verification without compromising
performance. Our algorithm is purely functional, by which
we mean that the execution of the algorithm does not require
destructive updates. In contrast, direct efficient implementations of NFA simulations are usually done using mutable
sets containing the “active states” of the NFA. Functional
Alternatives to such set-like data structures would usually
incur a logarithmic overhead in the time complexity.
Outline of the Paper. In Section 2, we discuss how the
formal semantics of lookarounds are encoded in Coq, and
how they can be used for equational reasoning. In Section
3, we define Oracle Regular Expressions and Oracle Strings,
the core abstraction used in our algorithm. The matching algorithm for oracle expressions is discussed in Section 4. The
main algorithm is presented in Section 5. Performance experiments using our extracted code are presented in Section
6. Finally, we discuss related work in Section 7.

2

𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝜀 ⇐⇒ 𝑖 = 𝑗
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝜎 ⇐⇒ 𝑗 = 𝑖 + 1 ∧ 𝜎(𝑤𝑖 ) = 1
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 1 + 𝑟 2 ⇐⇒ 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 1 ∨ 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 2
𝑤, (︀𝑖, 𝑘⌋︀ ⊧ 𝑟 1
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 1 ⋅ 𝑟 2 ⇐⇒ ∃𝑘 ∈ (︀𝑖, 𝑗⌋︀ s.t. (
)
∧𝑤, (︀𝑘, 𝑗⌋︀ ⊧ 𝑟 2
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 ∗ ⇐⇒ 𝑖 = 𝑗 ∨ ∃𝑘 ∈ (︀𝑖, 𝑗⌋︀ s.t.

⎛ 𝑤, (︀𝑖, 𝑘⌋︀ ⊧ 𝑟 ⎞
⎝∧𝑤, (︀𝑘, 𝑗⌋︀ ⊧ 𝑟 ∗ ⎠

𝑤, (︀𝑖, 𝑗⌋︀ ⊧ (?> 𝑟 ) ⇐⇒ 𝑖 = 𝑗 ∧ 𝑤, (︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ ⊧ 𝑟
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ (?⇑> 𝑟 ) ⇐⇒ 𝑖 = 𝑗 ∧ 𝑤, (︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ ⊧
⇑𝑟
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ (?< 𝑟 ) ⇐⇒ 𝑖 = 𝑗 ∧ 𝑤, (︀0, 𝑖⌋︀ ⊧ 𝑟
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ (?⇑< 𝑟 ) ⇐⇒ 𝑖 = 𝑗 ∧ 𝑤, (︀0, 𝑖⌋︀ ⊧
⇑𝑟

Figure 1. The satisfaction relation ⊧ relating a string 𝑤 ∈ Σ∗ ,
a window (︀𝑖, 𝑗⌋︀ with 0 ≤ 𝑖 ≤ 𝑗 ≤ ⋃︀𝑤 ⋃︀, and a regular expression
𝑟 ∈ LReg, possibly with lookarounds.
lookahead assertions (respectively, lookbehind assertions). Furthermore, (?> 𝑟 ) and (?< 𝑟 ) are called positive lookarounds,
while (?⇑> 𝑟 ) and (?⇑< 𝑟 ) are called negative lookarounds.
We write ⋃︀𝑤 ⋃︀ to denote the length of a string 𝑤. The empty
string (i.e., the string of length 0) is denoted by 𝜀. For a string
𝑤 ∈ Σ∗ , we will call a formal pair (︀𝑖, 𝑗⌋︀ with 0 ≤ 𝑖 ≤ 𝑗 ≤ ⋃︀𝑤 ⋃︀ a
window in 𝑤. We use some additional notation throughout
the paper. We write 𝑟 + to denote 𝑟 ⋅ 𝑟 ∗ , i.e., the repetition of
𝑟 one or more times.
In Coq, LReg is encoded as an inductive data type LRegex.
We use two inductively defined predicates match_regex, and
not_match_regex. The Proposition match_regex r w s d
and not_match_regex r w s d stand for the propositions
𝑤, (︀𝑠, 𝑠 + 𝑑⌋︀ ⊧ 𝑟 and its negation 𝑤, (︀𝑠, 𝑠 + 𝑑⌋︀ ⊧
⇑ 𝑟 , respectively.
These two predicates are defined using mutual recursion.
We define not_match_regex directly, instead of encoding
it as the negation of match_regex. This is because such an
encoding cannot be used in the arguments of the constructor
NegLookAhead or NegLookBehind. The encoding 𝜎 →  of
the negation ¬𝜎 as an argument of a constructor would
violate strict positivity. We prove the two following lemmas
which show that the two predicates behave in the manner
expected. Note that a different way of dealing with negation
would be to encode the ⊧ relation as a fixpoint returning a
Prop (as in [45], for example).

Lookaround Semantics

This section introduces the formal definitions used in the
paper.
Definition 1 (Regular Expressions with Lookaround).
Let Σ be an alphabet, and 𝒫 a set of decidable predicates over
Σ (i.e., functions of type Σ → {0, 1}). The set LReg of regular
expressions with lookaround is defined by the following
grammar:

Lemma match_not_match : forall r w start delta,
~ (match_regex r w start delta)
<-> (not_match_regex r w start delta).

𝑟, 𝑟 1, 𝑟 2 ∶∶= 𝜀 ⋃︀ 𝜎 ∈ 𝒫 ⋃︀ 𝑟 1 ⋅ 𝑟 2 ⋃︀ 𝑟 1 + 𝑟 2 ⋃︀ 𝑟 ∗ ⋃︀
(?> 𝑟 ) ⋃︀ (?⇑> 𝑟 ) ⋃︀ (?< 𝑟 ) ⋃︀ (?⇑< 𝑟 )

Lemma match_lem : forall r w start delta,
match_regex r w start delta
\/ ~ (match_regex r w start delta).

The expressions of the form (?> 𝑟 ), (?⇑> 𝑟 ), (?< 𝑟 ) and (?⇑< 𝑟 )
are called positive lookahead, negative lookahead, positive
lookbehind and negative lookbehind respectively. Collectively,
they are called lookaround assertions. Expressions of the form
(?> 𝑟 ) and (?⇑> 𝑟 ) (respectively, (?< 𝑟 ) and (?⇑< 𝑟 )) are called

In our Coq formalization, we choose to work with the representation 𝑤, (︀𝑠, 𝑠 + 𝑑⌋︀ ⊧ 𝑟 , rather than 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 . This

199

<!-- page 3 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

is so that we can avoid dealing with additional ordering
constraints 𝑖 ≤ 𝑗 while dealing with proofs.

relations can be expressed as Proper morphisms [41]. Declaring them in this manner allows standard rewrite tactics to
use ≡-based equations for rewriting inside proofs.

Example 2. Our use of predicates on the alphabet correspond to character classes in PCRE [43] terminology. For
instance, one could use the character class \d or [0-9] to
indicate the predicate that matches a digit. The slight difference in the semantics of lookarounds in our notation, in
contrast with PCRE, is that our lookaheads (respectively,
lookbehinds) match until the end (respectively, beginning)
of the string. The positive PCRE lookahead (?=r) is equivalent to our (?> 𝑟 Σ∗ ) and our lookahead (?> 𝑟 ) is equivalent
to the PCRE (?=r$) .
Consider the expression

Instance regex_eq_equiv : Equivalence regex_eq.
Instance union_proper : Proper
(regex_eq ==> regex_eq ==> regex_eq) Union.
Instance concat_proper : Proper
(regex_eq ==> regex_eq ==> regex_eq) Concat.
Instance star_proper : Proper
(regex_eq ==> regex_eq) Star.
...

An equivalent approach is to define 𝑟 ⊑ 𝑠 so that it holds
when for every 𝑤 ∈ Σ∗ and every window (︀𝑖, 𝑗⌋︀ of 𝑤, we
have 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 Ô⇒ 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑠. We prove that these two
definitions are equivalent.

𝑟 = (?<=c((?!\d\d).)+)(a+)b(?=.*d.*e)
in PCRE notation. Writing 𝛿 for the character-class \d , we
can translate this expression into our notation as

Lemma subset_leq : forall (r s : LRegex),
regex_leq r s <-> (forall (w : list A) (start d : nat),
match_regex r w start d
-> match_regex s w start d).

𝑟 = (?< Σ∗𝑐 ⋅ ((?⇑> 𝛿𝛿 Σ∗ )Σ)+ ) ⋅ 𝑎 +𝑏 ⋅ (?> Σ∗𝑑Σ∗𝑒Σ∗ ).
For the string 𝑤 shown below, there are four potential
matches for the subexpression 𝑎 +𝑏, as indicated below by
highlighting the substrings corresponding to their windows
at (︀2, 4⌋︀, (︀9, 12⌋︀, (︀10, 12⌋︀ and (︀14, 16⌋︀:

We have shown that ⊑ is a partial order and the operations of
union and concatenation are monotone with respect to this
order. These monotonicity theorems can also be expressed
using proper morphisms.

𝑐 𝑐 𝑎𝑏 𝑐 7 7𝑑 𝑐 𝑎 𝑎𝑏 7𝑑 𝑎𝑏 𝑒
The window (︀9, 12⌋︀ does not satisfy the lookbehind, since the
Kleene + requires at least one character after the occurrence
of 𝑐 before the match window. The window (︀2, 4⌋︀ does not
satisfy the lookbehind either. This is because even though
there is a 𝑐 at position 0, the substring 77 appearing at (︀5, 7⌋︀
in the future disqualifies it. The window (︀14, 16⌋︀ does not
satisfy the lookahead since there is no 𝑑 (or 𝑒) in the future.
It can be checked that only the window (︀10, 12⌋︀ is a match.

Instance regex_leq_partialorder :
PartialOrder regex_eq regex_leq.
Instance union_monotone : Proper
(regex_leq ==> regex_leq ==> regex_leq) Union.
Instance concat_monotone : Proper
(regex_leq ==> regex_leq ==> regex_leq) Concat.

The purpose of verifying these properties is to streamline
reasoning about regular expressions with lookarounds into
an equational manner when possible. In [31], the authors
have proven several identities resembling the properties of
Kleene Algebra [27] for the equivalence relation ≡, along
with several properties for lookarounds. We have verified
them in Coq. In future work, this style of reasoning could
serve as the basis for proving the correctness of simplification
algorithms for expressions with lookarounds.

Equational Reasoning. Using the semantics above, we
can set up a framework for equational reasoning on regular
expressions with lookarounds.
Definition 3 (Equivalence and Containment Relations on
Regular Expressions). Let 𝑟, 𝑠 ∈ LReg. We say that 𝑟 and 𝑠 are
equivalent, and we write 𝑟 ≡ 𝑠, if for every string 𝑤 ∈ Σ∗ and
every window (︀𝑖, 𝑗⌋︀ of 𝑤, we have 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 if and only if
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑠. We also define the containment relation 𝑟 ⊑ 𝑠 as
notational shorthand for 𝑟 + 𝑠 ≡ 𝑠.

3

Oracles for Lookaround Assertions

In this section, we will assume that relevant information
about lookaround assertions can be accessed using oracle
queries. In the algorithm we describe later in Section 5 which
is based on decomposing the given expression into layers,
this information for the inner layers will be computed in
a prior step. We provide an automata-theoretic framework
describing how such regular expressions with oracle queries
can be handled. To do this, we first make the notion of
strings augmented with oracle valuations explicit. An extension of regular expressions which can query these oracles is presented. The idea is to replace lookaround expressions with these oracle queries, so that information involving

We encode the ≡ and ⊑ relations in Coq as follows:
Definition regex_eq (r1 r2 : LRegex) : Prop :=
forall w s d,
match_regex r1 w s d <-> match_regex r2 w s d.
Definition regex_leq (r s : Regex) : Prop :=
regex_eq (Union r s) s.

We notice that this is an equivalence relation and a congruence with respect to the regular expression combinators such as concatenation, union, Kleene iteration and the
lookarounds. In the terminology of Coq, these congruence

200

<!-- page 4 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

Note that the slice (︀𝑖, 𝑗⌋︀ can be obtained by taking the slice
(︀0, 𝑗⌋︀ to obtain ∐︀𝑤 ′, 𝛽 ′ ̃︀ and then selecting the slice (︀𝑖, ⋃︀𝑤 ′ ⋃︀⌋︀.
This is how we encode the notion of slicing in our Coq
formalization. We define the functions oskipn and ofirstn
corresponding to the slices (︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ and (︀0, 𝑖⌋︀ respectively.
These mirror the definition of the functions skipn and firstn
in the Coq standard library, and satisfy analogous lemmas.
Note the slight asymmetry in the definitions below, resulting
from the mismatch in the lengths of the components of ostrings.

lookarounds can be answered using an oracle. These oraclebased expressions can be matched using a model that is an
extension of NFAs. We show that these NFAs can be simulated using a purely functional algorithm. Similar to a usual
NFA, this simulation is done in a single left-to-right pass,
consuming the input word (including the oracle valuations)
one character at a time, taking 𝑂(𝑚) time at each step, where
𝑚 is the size of the regular expression.
3.1

Oracle Strings and Oracle Regular Expressions

Suppose 𝑉 is a finite set of (Boolean) variables. A valuation
of 𝑉 is a truth assignment to the elements of 𝑉 . We will
denote the set of valuations of 𝑉 as 2𝑉 . An o-string is a pair
∐︀𝑤, 𝛽̃︀ where 𝑤 ∈ Σ∗ and 𝛽 ∈ (2𝑉 )∗ such that ⋃︀𝛽⋃︀ = ⋃︀𝑤 ⋃︀ + 1.
The set of o-strings over the alphabet Σ and variables 𝑉 is
denoted 𝒪(Σ,𝑉 ).
These represent strings together with additional information accessible via oracle queries. Suppose 𝑤 = 𝑎 0 ⋅ 𝑎 1 ⋯𝑎𝑛−1
and 𝛽 = 𝛽 0 ⋅ 𝛽 1 ⋅ 𝛽 2 ⋯𝛽𝑛 . Informally, we wish to associate each
character 𝑎𝑖 with the oracle valuation 𝛽𝑖 to its left and 𝛽𝑖+1
to its right. Thus, the o-string ∐︀𝑤, 𝛽̃︀ could be more visually
represented as

Definition ofirstn (n : nat) (s : ostring) : ostring :=
(firstn n (fst s), firstn (S n) (snd s)).
Definition oskipn (n : nat) (s : ostring) : ostring :=
(skipn n (fst s), skipn (min n (length (fst s))) (snd s)).

The concatenation operation on 𝒪(Σ,𝑉 ) needs to be defined carefully so that it matches the way we intend to use
these strings. The concatenation of two elements of 𝒪(Σ,𝑉 )
is only defined if the oracle-values agree. Formally, suppose ∐︀𝑤 1, 𝛽 ⋅ 𝑢̃︀, ∐︀𝑤 2, 𝑣 ⋅ 𝛾̃︀ ∈ 𝒪(Σ,𝑉 ) with 𝑢, 𝑣 ∈ 2𝑉 , then
∐︀𝑤 1, 𝛽 ⋅ 𝑢̃︀ ⋅ ∐︀𝑤 2, 𝑣 ⋅ 𝛾̃︀ is defined iff 𝑢 = 𝑣. The concatenation
∐︀𝑤 1, 𝛽 ⋅ 𝑢̃︀ ⋅ ∐︀𝑤 2,𝑢 ⋅ 𝛾̃︀ is defined to be ∐︀𝑤 1𝑤 2, 𝛽 ⋅ 𝑢 ⋅ 𝛾̃︀. We
extend this definition in a natural way to concatenation of
sets of o-strings. Kleene-iteration of an o-string (or a set of
o-strings) is also defined in an analogous manner, respecting
the agreement of oracle valuations at concatenation boundaries.

𝛽 0 𝑎 0 𝛽 1 𝑎 1 𝛽 2 𝑎 2 𝛽 3 ⋯𝛽𝑛−1 𝑎𝑛−1 𝛽𝑛 .

This explains the constraints on the length of the components
of o-strings. The length of the o-string ∐︀𝑤, 𝛽̃︀ written ⋃︀∐︀𝑤, 𝛽̃︀⋃︀
is defined to be ⋃︀𝑤 ⋃︀. Note that this means that there may be
more than one o-string of length 0, since such an o-string
consists of a single oracle valuation.
In Coq, we define valuations as list of Booleans, and ostrings simply as pairs of lists. Instead of working with dependent types, we enforce the constraints on the lengths
(i.e, that ⋃︀𝛽⋃︀ = ⋃︀𝑤 ⋃︀ + 1, and that each valuation has the same
number of Booleans) using a separate predicate.

Definition 4 (Oracle Regular Expressions). The set OReg
of oracle regular expressions is defined with the following
grammar:
𝑅, 𝑆 ∶∶= 𝜀 ⋃︀ 𝜎 ∈ 𝒫 ⋃︀ Q+ (𝑣 ∈ 𝑉 ) ⋃︀ Q- (𝑣 ∈ 𝑉 ) ⋃︀ 𝑅 ⋅ 𝑆 ⋃︀ 𝑅 + 𝑆 ⋃︀ 𝑅 ∗ .
Instead of lookaheads as in LReg, oracle regular expressions have positive queries of the form Q+ (𝑣) or negative
queries of the form Q- (𝑣), where 𝑣 ∈ 𝑉 . These queries express assertions on the accompanying oracle valuations. The
semantics of a OReg expression 𝑅 is given in terms of J𝑅K,
a subset of 𝒪(Σ,𝑉 ). This language is defined inductively as
follows:

Definition valuation : Type := list bool.
Definition ostring : Type := (list A) * (list valuation).
Definition outer_length_wf (s : ostring) : Prop :=
length (fst s) + 1 = length (snd s).
Definition inner_length_wf (s : ostring) : Prop :=
forall u v : valuation,
In u (snd s) -> In v (snd s) -> length u = length v.
Definition ostring_wf (s : ostring) : Prop :=
outer_length_wf s /\ inner_length_wf s.

J𝜀K = {∐︀𝜀, 𝛽 0 ̃︀ ⋃︀ 𝛽 0 ∈ 2𝑉 }

J𝜎K = {∐︀𝑎, 𝛽 0 ⋅ 𝛽 1 ̃︀ ⋃︀ 𝑎 ∈ Σ, 𝛽 0, 𝛽 1 ∈ 2𝑉 , 𝜎(𝑎) = 1}

JQ+ (𝑣)K = {∐︀𝜀, 𝛽 0 ̃︀ ⋃︀ 𝛽 0 ∈ 2𝑉 , 𝛽 0 (︀𝑣⌋︀ = 1}

Definition olength (s : ostring) : nat := length (fst s).

JQ- (𝑣)K = {∐︀𝜀, 𝛽 0 ̃︀ ⋃︀ 𝛽 0 ∈ 2𝑉 , 𝛽 0 (︀𝑣⌋︀ = 0}

When slicing o-strings, we must take into account the
oracle valuations at the boundaries. Suppose we have an
∐︀𝑤, 𝛽̃︀ ∈ 𝒪(Σ,𝑉 ) with 𝑤 = 𝑎 0 ⋯𝑎𝑛−1 and 𝛽 = 𝛽 0 ⋅ 𝛽 1 ⋯𝛽𝑛 .
For 0 ≤ 𝑖 ≤ 𝑗 ≤ 𝑛, we define the slice of ∐︀𝑤, 𝛽̃︀ at window
(︀𝑖, 𝑗⌋︀, denoted by ∐︀𝑤, 𝛽̃︀(︀𝑖, 𝑗⌋︀, as the o-string (𝑤, 𝛽 ′ ) where
𝛽 ′ = 𝛽𝑖 ⋅ 𝛽𝑖+1 ⋯𝛽 𝑗−1 ⋅ 𝛽 𝑗 and 𝑤 = 𝑎𝑖 ⋅ 𝑎𝑖+1 ⋯𝑎 𝑗−1 . In particular,
when 𝑖 = 𝑗, 𝑤 is the empty string 𝜀 and 𝛽 ′ is the singleton
string 𝛽𝑖 .

J𝑅 + 𝑆K = J𝑅K ∪ J𝑆K

J𝑅 ⋅ 𝑆K = J𝑅K ⋅ J𝑆K

J𝑅 ∗ K = J𝑅K∗

Example 5. Consider the oregex 𝑟 1 = Q+ (𝑣 0 ) ⋅ 𝑎 +𝑏 ⋅ Q+ (𝑣 1 ).
Here, the queries express the constraint that the valuation
corresponding to the variable 𝑣 0 at the beginning of the
string, and the valuation corresponding to the variable 𝑣 1
at the end of the string are both true. Concretely, we have
∐︀𝑤, 𝛽̃︀ ∈ J𝑟 K iff 𝑤 ∈ J𝑎 +𝑏K and 𝛽 0 (︀𝑣 0 ⌋︀ = 𝛽 ⋃︀𝑤⋃︀ (︀𝑣 1 ⌋︀ = 1.

201

<!-- page 5 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

Consider the oregex 𝑟 2 = 𝑐 ⋅(Q- (𝑣 2 )Σ)+ . Here, the negative
query Q- (𝑣 2 ) asserts that the valuation corresponding to the
variable 𝑣 2 be false before each character that matches the
subexpression Σ. More concretely, suppose ∐︀𝑤, 𝛽̃︀ ∈ J𝑟 2 K.
Then, we have that 𝑤 0 = 𝑐 and ⋃︀𝑤 ⋃︀ > 1 (enforced by the
Kleene plus). For 1 ≤ 𝑖 ≤ ⋃︀𝑤 ⋃︀ − 1, we must have 𝛽𝑖 (︀𝑣 2 ⌋︀ = 0.
Note that no constraints are placed on 𝛽 0 or 𝛽 ⋃︀𝑤⋃︀ .

If (◁, 𝑠) or (▷, 𝑠) appears in maxLk(𝑟 ), then we say that
𝑠 is a maximal lookaround of 𝑟 . The arity of an expression
𝑟 is the number of its maximal lookarounds, i.e., arity(𝑟 ) =
⋃︀maxLk(𝑟 )⋃︀.
Note that the directions associated with the definitions
above may be counter-intuitive. As we will see in subsection
5.1, lookbehinds need the scanning of the string from left to
right, while lookaheads need the scanning from right to left.

In our Coq formalization, we represent objects of type
OReg as the inductive type ORegular. In order to represent
membership in the set J𝑅K, we work with a satisfaction relation match_oregex. This is similar to our handling of LReg,
except that the complication involving handling negation
does not appear here. The case for concatenation and Kleene
iteration, however, is interesting. This is because concatenation on o-strings is defined only when the oracle valuations
agree. We resolve this issue by phrasing the concatenation
in terms of the slicing operation. In particular, we say that
∐︀𝑤, 𝛽̃︀ ∈ J𝑅 ⋅ 𝑆K iff there exists an 𝑖 such that ∐︀𝑤, 𝛽̃︀(︀0, 𝑖⌋︀ ∈ J𝑅K
and ∐︀𝑤, 𝛽̃︀(︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ ∈ J𝑆K. The Kleene iteration is handled in a
similar manner. These two cases are shown below.

Definition 7 (Abstraction). The abstraction abstract(𝑟 )
of a regular expression 𝑟 is obtained by replacing each maximal lookaround with Q+ (𝑣) or Q- (𝑣). The variables here
are chosen from the set 𝑉 = {𝑣𝑖 ⋃︀ 𝑖 < ⋃︀arity(𝑟 )⋃︀}. Formally,
abstract(𝑟 ) is defined to be abstract0 (𝑟 ), and given 𝑖 ∈ N,
we define abstract𝑖 (𝑟 ) as follows:
abstract𝑖 (𝑟 ) = 𝑟 , if 𝑟 ∈ {𝜀, (𝜎 ∈ 𝒫)}
abstract𝑖 (𝑟 1 ⋅ 𝑟 2 ) = abstract𝑖 (𝑟 1 ) ⋅ abstract𝑖+arity(𝑟 1 ) (𝑟 2 )
abstract𝑖 (𝑟 1 + 𝑟 2 ) = abstract𝑖 (𝑟 1 ) + abstract𝑖+arity(𝑟 1 ) (𝑟 2 )
abstract𝑖 (𝑟 ∗ ) = abstract𝑖 (𝑟 )∗
abstract𝑖 (𝑟 ) = Q+ (𝑣𝑖 ), if 𝑟 ∈ {(?> 𝑠), (?< 𝑠)}

...
| omatch_concat :
forall (r1 r2 : ORegex) (os : ostring) (n : nat),
match_oregex r1 (ofirstn n os)
-> match_oregex r2 (oskipn n os)
-> match_oregex (OConcat r1 r2) os
...
| omatch_star_cons :
forall (r : ORegex) (os : ostring) (n : nat),
match_oregex r (ofirstn n os)
-> match_oregex (OStar r) (oskipn n os)
-> match_oregex (OStar r) os
...

abstract𝑖 (𝑟 ) = Q- (𝑣𝑖 ), if 𝑟 ∈ {(?⇑> 𝑠), (?⇑< 𝑠)}
Example 8. Consider again the expression
𝑟 = (?< Σ∗𝑐 ⋅ ((?⇑> 𝛿𝛿 Σ∗ )Σ)+ ) ⋅ 𝑎 +𝑏 ⋅ (?> Σ∗𝑑Σ∗𝑒Σ∗ )
from Example 2. We have that maxLk(𝑟 ) = (︀𝑚 0,𝑚 1 ⌋︀ where
𝑚 0 = (▷, Σ∗𝑐 ⋅ ((?⇑> 𝛿𝛿 Σ∗ )Σ)+ ) and 𝑚 1 = (◁, Σ∗𝑑Σ∗𝑒Σ∗ ).
Therefore, we have arity(𝑟 ) = 2. Note that even though
(?⇑> 𝛿𝛿 Σ∗ ) is a lookahead that appears as a subexpression, it
is not considered maximal.
The abstraction of 𝑟 is given by
abstract(𝑟 ) = abstract0 (𝑟 ) = Q+ (𝑣 0 ) ⋅ 𝑎 +𝑏 ⋅ Q+ (𝑣 1 )

3.2

. If we consider the subexpression 𝑠 = Σ∗𝑐 ⋅ ((?⇑> 𝛿𝛿 Σ∗ )Σ)+
of 𝑟 , then we have abstract(𝑠) = Σ∗𝑐 ⋅ (Q- (𝑣 0 ) ⋅ Σ)+ .

Choosing Appropriate Oracle Valuations

In this section, we will make concrete the connection between LReg and OReg. In particular, we will show that if
the lookaround assertions are pre-evaluated, then they could
be used as valuations for o-strings which could be supplied
to an oracle regular expression obtained by replacing the
lookarounds with queries.

Next, we define the notion of tapes for expressions and
lookarounds, which are sequences of truth values obtained
by incrementally scanning the string.
Definition 9 (Tapes, Oracle Valuations). Let 𝑟 ∈ LReg
be an expression and 𝑤 ∈ Σ∗ be a string such that ⋃︀𝑤 ⋃︀ = 𝑛.
We define the left-to-right tape tape(▷, 𝑟, 𝑤) and the rightto-left tape tape(◁, 𝑟, 𝑤) as sequences of length 𝑛 + 1 over
{0, 1} such that for each 0 ≤ 𝑖 ≤ 𝑛, the 𝑖-th entry of the tape
satisfies the following:
)︀
⌉︀
⌉︀1 if 𝑤, (︀0, 𝑖⌋︀ ⊧ 𝑟
tape(▷, 𝑟, 𝑤)(︀𝑖⌋︀ = ⌋︀
⌉︀
0 if 𝑤, (︀0, 𝑖⌋︀ ⇑
⊧𝑟
⌉︀
]︀
)︀
⌉︀
⌉︀1 if 𝑤, (︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ ⊧ 𝑟
tape(◁, 𝑟, 𝑤)(︀𝑖⌋︀ = ⌋︀
⌉︀
0 if 𝑤, (︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ ⇑
⊧𝑟
⌉︀
]︀
When there is no confusion, we will write tape to mean
tape(▷).

Definition 6 (Maximal Lookarounds, Arity). Let 𝑟 ∈
LReg be an expression. The maximal lookarounds of 𝑟 , written maxLk(𝑟 ) is a list of tuples of type {▷, ◁} × LReg. It is
formally defined as follows:
maxLk((?> 𝑟 )) = (︀(◁, 𝑟 )⌋︀

maxLk((?< 𝑟 )) = (︀(▷, 𝑟 )⌋︀

maxLk((?⇑> 𝑟 )) = (︀(◁, 𝑟 )⌋︀

maxLk((?⇑< 𝑟 )) = (︀(▷, 𝑟 )⌋︀

maxLk(𝑟 ) = (︀⌋︀, if 𝑟 ∈ {𝜀, (𝜎 ∈ 𝒫)}
maxLk(𝑟 1 ○ 𝑟 2 ) = maxLk(𝑟 1 ) ++ maxLk(𝑟 2 ), if ○ ∈ {⋅, +}
maxLk(𝑟 ∗ ) = maxLk(𝑟 )
where the notation ++ refers to list concatenation.

202

<!-- page 6 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

We also extend the notion of tapes to OReg and maximal
lookarounds. For 𝑠 ∈ OReg with ∐︀𝑤, 𝛽̃︀ ∈ 𝒪(Σ,𝑉 ), we define tape(𝑠, ∐︀𝑤, 𝛽̃︀) in a similar manner as above, indicating
the truth values of the slices ∐︀𝑤, 𝛽̃︀(︀0, 𝑖⌋︀ for tape(▷) and
∐︀𝑤, 𝛽̃︀(︀𝑖, ⋃︀𝑤 ⋃︀⌋︀ for tape(◁). Given a maximal lookaround of
the form 𝑚 = (𝑑, 𝑟 ), where 𝑑 ∈ {▷, ◁} and 𝑟 ∈ LReg, we
define tape((𝑑, 𝑟 )) in a natural way as tape((▷, 𝑟 ), 𝑤) =
tape(▷, 𝑟, 𝑤) and tape((◁, 𝑟 ), 𝑤) = tape(◁, 𝑟, 𝑤).
Let 𝑟 ∈ LReg be of arity 𝑘. Let 𝑇 be a collection of 𝑘 tapes
s.t.for each 0 ≤ 𝑖 < 𝑘, tape 𝑇 (︀𝑖⌋︀ = tape(maxLk(𝑟 )(︀𝑖⌋︀, 𝑤). Let
𝑉 = 𝑣 0, 𝑣 1, . . . 𝑣𝑘−1 . The oracle valuations for 𝑟 on 𝑤, written
oval(𝑟, 𝑤) is the sequence of length ⋃︀𝑤 ⋃︀ + 1 over 2𝑉 obtained
by transposing 𝑇 . In other words,

The following lemma establishes the connection between
LReg and OReg by choosing the appropriate oracle valuations.
Lemma 11. Let 𝑟 ∈ LReg and 𝑤 ∈ Σ∗ . Let 𝛽 = oval(𝑟, 𝑤).
Then, the following holds.
𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 ⇐⇒ ∐︀𝑤, 𝛽̃︀(︀𝑖, 𝑗⌋︀ ∈ Jabstract(𝑟 )K

Thus, in order to match an expression 𝑟 with lookarounds
on 𝑤, we can first evaluate the lookarounds to obtain the
oracle valuations 𝛽 = oval(𝑟, 𝑤), and then match the oracle
regular expression abstract(𝑟 ) on ∐︀𝑤, 𝛽̃︀. This formalizes the
key idea behind our approach.
In Coq, this lemma is stated in terms of the ofirstn and
oskipn functions. Recall that the index 𝑠 is used to keep
track of the subscript of abstract𝑠 .

oval(𝑟, 𝑤)(︀𝑖⌋︀(︀𝑣 𝑗 ⌋︀ = 𝑇 (︀𝑗⌋︀(︀𝑖⌋︀, for 0 ≤ 𝑖 ≤ ⋃︀𝑤 ⋃︀, 0 ≤ 𝑗 < 𝑘
Example 10. Consider 𝑟 and 𝑤 from Example 2, whose
abstraction and maximal lookarounds were shown in Example 8. The maximal lookaround 𝑚 0 searches for the character
𝑐 in the past followed by a non-empty sequence of characters
satisfying certain conditions, while the maximal lookaround
𝑚 1 searches for an occurrence of the character 𝑑 followed
by the character 𝑒 in the future. The columns below show
the word 𝑤 and the tapes for tape(𝑚 0, 𝑤) and tape(𝑚 1, 𝑤),
in that order. The oracle valuations oval(𝑟, 𝑤) for 𝑟 on 𝑤 can
be obtained by reading the table below in a column-wise
manner.
c
0
1

c
0
1

a
0
1

b
0
1

c
0
1

7
0
1

7
0
1

d
0
1

c
0
1

a
0
1

a
1
1

b
1
1

7
1
1

d
1
1

a
1
0

b
1
0

Lemma oracle_compose_aux (r : @LRegex A) (w : list A)
(s : nat) (vs : list valuation) :
is_oval_aux r w s vs
-> forall start delta, start + delta <= length w
-> match_regex r w start delta
<-> match_oregex (abstractAux s r)
(ofirstn delta (oskipn start (w, vs))).

4

In this section, we discuss how the oracle regular expressions
can be matched on oracle strings, i.e., the computation of
tapes (from Definition 9 for the case of OReg. This is very
similar to the matching algorithm for ordinary regular expressions (without queries or lookarounds), except that the
queries need to be checked against the oracle valuations.
The main difference between handling characterpredicates and queries can be understood by analyzing the
case of concatenation. For example, consider the regular
expression 𝜎1 ⋅ 𝜎2 obtained by concatenating two predicates
𝜎1 and 𝜎2 . This matches a length 2 string where the first
character satisfies 𝜎1 and the second character satisfies 𝜎2 .
In contrast, the oracle regular expression Q+ (𝑣 1 ) ⋅ Q+ (𝑣 2 )
matches an o-string ∐︀𝜀, 𝛽 0 ̃︀, where 𝛽 0 (︀𝑣 1 ⌋︀ = 𝛽 1 (︀𝑣 2 ⌋︀ = 1. Thus,
the concatenated queries match the same oracle valuation. In
[31], this is done using the notion of Oracle NFAs in which
the transitions are guarded with queries.
We use a purely functional approach based on marked regular expressions. The notion of marked regular expressions
for classical regular expressions is discussed in [19] and [3].
In [36], the authors formalize this approach in Isabelle, and
utilize them in order to decide equivalence. Here, we extend
this approach to oracle regular expressions, and formalize it
in Coq.
The marked regular expression approach directly deals
with the syntax tree of regular expressions instead of building an explicit NFA. This is particularly pleasant in functional programming languages where we can represent the

e
1
0

Purely Functional Matching of Oracle
Expressions

1
0

In our formalization, we define is_lookaround_tape to
encode the notion of tape for maximal lookarounds, (i.e.,
pairs {▷, ◁} × LReg). The definition of oval is instrumented
using an additional index 𝑠 which corresponds to the subscript abstract𝑠 . This is necessary in order to formulate a
stronger inductive hypothesis we can induct on. Note also
that we phrase this definition as a relation rather than a
function.
Definition oval_tapes_aux (e : @LRegex A) (w : list A)
(s : nat) (ts : list tape) : Prop :=
length ts >= s + arity e
/\ (forall t, In t ts -> length t = length w + 1)
/\ forall i t r, i < arity e
-> nth_error ts (s + i) = Some t
-> nth_error (maximal_lookarounds e) i = Some r
-> is_lookaround_tape r w t.
Definition is_oval_aux (r : @LRegex A) (w : list A)
(s : nat) (vs : list valuation) : Prop :=
exists ts, oval_tapes_aux r w s ts
/\ vs = transpose (length w + 1) ts.
Definition is_oval (r : @LRegex A) (w : list A)
(vs : list valuation) : Prop :=
is_oval_aux r w 0 vs.

203

<!-- page 7 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

expressions as inductive types and use pattern matching to
access the subexpressions. The marks placed on the character classes of the regular expression are used to keep track
of the active states of the corresponding NFA.

2 manipulate the marks without changing the underlying
regex.
Let us start by characterizing the functions nullable and
final. In the following, we call a marked predicate 𝜎 spurious
if the predicate 𝜎 is unsatisfiable. If 𝑚 has no such predicates,
we say 𝑚 has no spurious marks.

Definition 12 (Marked Oracle Regular Expressions).
The set MReg of marked oracle regular expressions is defined
with the following grammar:

Lemma 14. Let 𝑚 ∈ MReg. Then, the following holds.

𝑀, 𝑁 ∶∶= 𝜀 ⋃︀ 𝜎 ⋃︀ 𝜎 ⋃︀ Q+ (𝑣) ⋃︀ Q- (𝑣) ⋃︀ 𝑀 ⋅ 𝑁 ⋃︀ 𝑀 + 𝑁 ⋃︀ 𝑀 ∗

1. For 𝛽 0 ∈ 2𝑉 , nullable(𝛽 0,𝑚) = 1 iff ∐︀𝜀, 𝛽 0 ̃︀ ∈ Jstrip(𝑚)K
2. Suppose 𝑚 has no spurious marks and 𝛽 1 ∈ 2𝑉 . If
final(𝛽 1,𝑚) = 1, then there exists 𝑎 ∈ Σ such that
for all 𝛽 0 ∈ 2𝑉 , ∐︀𝑎, 𝛽 0 𝛽 1 ̃︀ ∈ ⎷𝑚⌄
3. Let 𝑎 ∈ Σ and 𝛽 0, 𝛽 1 ∈ 2𝑉 . If ∐︀𝑎, 𝛽 0 𝛽 1 ̃︀ ∈ ⎷𝑚⌄, then
final(𝛽 1,𝑚) = 1

where 𝜎 ∈ 𝒫 and 𝑣 ∈ 𝑉 . We call 𝜎 (respectively, 𝜎) a marked
(respectively, unmarked) predicate.
We extend the terminology above to say that an expression
is unmarked if it contains no marked predicates. Given 𝑟 ∈
OReg, we can view it as an element of MReg in which none
of the predicates are marked. This expression is written as
toMarked(𝑟 ). Conversely, given 𝑚 ∈ MReg, we can remove
all the marks from it to obtain the expression strip(𝑚) ∈
OReg. For each 𝑚 ∈ MReg, we define a subset ⎷𝑟 ⌄ of 𝒪(Σ,𝑉 )
as follows:
⎷𝑟 ⌄ = ∅ if 𝑟 ∈ {𝜀, 𝜎, Q+ (𝑣), Q- (𝑣)}

Example 15. Consider the expressions 𝑚 6 = (𝑎 + 𝜀)Q+ (𝑣),
and 𝑚 7 = (𝑎 + 𝜀)Q+ (𝑣). Since strip(𝑚 6 ) = strip(𝑚 7 ), we can
see that nullable(𝛽 0,𝑚 6 ) = nullable(𝛽 0,𝑚 7 ) for any choice
of 𝛽 0 . In particular, nullable(𝛽 0,𝑚 6 ) = 1 iff 𝛽 0 (︀𝑣⌋︀ = 1. Here,
the function nullable tests whether the o-string ∐︀𝜀, 𝛽 0 ̃︀ is in
Jstrip(𝑚 6 )K.
Informally, the function final checks if there is a mark
on a predicate that is in the final position, and whether the
subsequent queries can be satisfied by the valuation provided.
Thus, if 𝛽 1 (︀𝑣⌋︀ = 1, we do have final(𝛽 1,𝑚 7 ) = 1. For 𝑚 6 , we
have final(𝛽 0,𝑚 6 ) = 0 for any 𝛽 0 since there are no marks in
𝑚 6 . Similarly, final(𝛽 0, (𝑎𝑏𝑐 +𝜀)Q+ (𝑣)) = 0, since the marked
character 𝑎 is not in a final position. Also, final(𝛽 0, (𝑎+𝜀)) = 1
for any 𝛽 0 since there are no queries imposing constraints
on the valuations.

⎷𝜎⌄ = J𝜎K

⎷𝑒 1 + 𝑒 2 ⌄ = ⎷𝑒 1 ⌄ ∪ ⎷𝑒 2 ⌄
⎷𝑒 1 ⋅ 𝑒 2 ⌄ = ⎷𝑒 1 ⌄ ⋅ Jstrip(𝑒 2 )K ∪ ⎷𝑒 2 ⌄
⎷𝑒 ∗ ⌄ = ⎷𝑒⌄ ⋅ Jstrip(𝑒)K∗

Example 13. The intuition for the language of ⎷𝑟 ⌄ is that
it represents the set of o-strings that could be matched by
starting from the current mark. For example, consider the
expressions 𝑚 1 = 𝑎𝑏𝑐, 𝑚 2 = 𝑎𝑏𝑐 and 𝑚 3 = 𝑎𝑏𝑐. Let ∐︀𝑤, 𝛽̃︀
be an o-string. We have ∐︀𝑤, 𝛽̃︀ ∈ ⎷𝑚 1 ⌄ iff 𝑤 = 𝑎𝑏𝑐, ∐︀𝑤, 𝛽̃︀ ∈
⎷𝑚 2 ⌄ iff 𝑤 = 𝑏𝑐 and ∐︀𝑤, 𝛽̃︀ ∈ ⎷𝑚 3 ⌄ iff 𝑤 = 𝑎𝑏𝑐 or 𝑤 = 𝑐. On
the other hand, if 𝑚 4 = 𝑎𝑏𝑐 with no marks, then ⎷𝑚 4 ⌄ = ∅.
This can be understood by realizing that this represents a
scenario where there are no active states in the simulated
NFA. In general, we can observe that for unmarked 𝑚, the
language ⎷𝑚⌄ is ∅.
The queries in expressions in MReg do not carry any
marks, but they can still express constraints on valuations if
they occur after a marked predicate. For instance, consider
𝑚 5 = 𝑎Q+ (𝑣)𝑏𝑐 and 𝑚 6 = Q+ (𝑣)𝑎𝑏𝑐. Given ∐︀𝑤, 𝛽̃︀ ∈ ⎷𝑚 5 ⌄,
we must have 𝑤 = 𝑎𝑏𝑐 and 𝛽 1 (︀𝑣⌋︀ = 1. However, for ∐︀𝑤, 𝛽̃︀ ∈
⎷𝑚 6 ⌄, we have no constraints on 𝛽.

Next, we characterize the function read. The function
read(𝑎, ⋅) has the effect of removing the marks from marked
predicates 𝜎 which do not satisfy 𝜎(𝑎) = 1. Roughly, this has
the effect of removing from ⎷𝑚⌄ the o-strings which do not
start with the character 𝑎.
Lemma 16. Let 𝑚 ∈ MReg and 𝑎 ∈ Σ. Then, the following
holds.
1. ⎷read(𝑎,𝑚)⌄ ⊆ ⎷𝑚⌄
2. Suppose ∐︀𝑎 ⋅ 𝑤, 𝛽̃︀ ∈ ⎷𝑚⌄. Then, ∐︀𝑎 ⋅ 𝑤, 𝛽̃︀ ∈
⎷read(𝑎,𝑚)⌄
3. Suppose 𝑎 ′ ∈ Σ and ∐︀𝑎 ′ ⋅ 𝑤, 𝛽̃︀ ∈ ⎷read(𝑎,𝑚)⌄. Then,
∐︀𝑎 ⋅ 𝑤, 𝛽̃︀ ∈ ⎷𝑚⌄
4. The expression read(𝑎,𝑚) has no spurious marks

In Coq, we define an inductive type MRegex to represent
the set MReg. Similar to the case of LReg and OReg, we represent the notion of ⎷⋅⌄ using a satisfaction relation encoded
as the inductive type match_mregex.
4.1

Example 17. Let us use the predicate 𝛿 to denote the set of
digits. Consider the expression 𝑚 = 𝛿𝑎𝑏𝑐. We observe that
read(7,𝑚) = 𝑚 since the digit 7 satisfies the predicate 𝛿. On
the other hand, read(𝑎,𝑚) = 𝛿𝑎𝑏𝑐 – the mark is lost since 𝑎
does not satisfy the predicate.

Operations on Marked Expressions

Our matching algorithm operates by transforming marked
regular expressions. The functions nullable and final are used
to extract information about the state machine that is being
simulated. The functions follow and read presented in Figure

In order to prove the properties of the function follow, it
is helpful to define two other functions init and shift (also
shown in Figure 2). These two functions are related in the

204

<!-- page 8 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

𝑉

𝑉

final ∶ 2 × MReg → B

nullable ∶ 2 × MReg → B
∗

nullable(𝛽, 𝜀) = 1

nullable(𝛽, 𝑚 ) = 1

final(𝛽, 𝜀) = 0

nullable(𝛽, 𝜎) = 0

nullable(𝛽, 𝜎) = 0

final(𝛽, 𝜎) = 0

+

-

nullable(𝛽, Q (𝑣)) = 𝛽(︀𝑣⌋︀

+

final(𝛽, Q (𝑣)) = 0

nullable(𝛽, Q (𝑣)) = ¬𝛽(︀𝑣⌋︀

∗

final(𝛽, 𝑚 ) = final(𝛽, 𝑚)
final(𝛽, 𝜎) = 1
-

final(𝛽, Q (𝑣)) = 0

nullable(𝛽, 𝑚 1 + 𝑚 2 ) = nullable(𝛽, 𝑚 1 ) ∨ nullable(𝛽, 𝑚 2 )

final(𝛽, 𝑚 1 + 𝑚 2 ) = final(𝛽, 𝑚 1 ) ∨ final(𝛽, 𝑚 2 )

nullable(𝛽, 𝑚 1 ⋅ 𝑚 2 ) = nullable(𝛽, 𝑚 1 ) ∧ nullable(𝛽, 𝑚 2 )

final(𝛽, 𝑚 1 ⋅ 𝑚 2 ) = (final(𝛽, 𝑚 1 ) ∧ nullable(𝛽, 𝑚 2 )) ∨ final(𝛽, 𝑚 2 )

𝑉

follow ∶ B × 2 × MReg → MReg
follow(𝛽, 𝜀) = 𝜀
+

+

follow(𝛽, Q (𝑣)) = Q (𝑣)

read ∶ Σ × MReg → MReg
-

-

follow(𝛽, Q (𝑣)) = Q (𝑣)

read(𝑎, 𝜀) = 𝜀
+

if 𝑏 = 1
otherwise

𝜎
follow(𝑏, 𝛽, 𝜎) = follow(𝑏, 𝛽, 𝜎) = {
𝜎

′

follow(𝑏, 𝛽, 𝑚 1 ⋅ 𝑚 2 ) = follow(𝑏, 𝛽, 𝑚 1 ) ⋅ follow(𝑏 , 𝛽, 𝑚 2 )

-

if 𝜎(𝑎) = 1
otherwise

read(𝑎, 𝑚 1 + 𝑚 2 ) = read(𝑎, 𝑚 1 ) + read(𝑎, 𝑚 2 )

′

where 𝑏 = final(𝛽, 𝑚 1 ) ∨ (𝑏 ∧ nullable(𝛽, 𝑚 1 ))
∗

-

read(𝑎, Q (𝑣)) = Q (𝑣)

𝜎
read(𝑎, 𝜎) = {
𝜎

read(𝑎, 𝜎) = 𝜎

follow(𝑏, 𝛽, 𝑚 1 + 𝑚 2 ) = follow(𝑏, 𝛽, 𝑚 1 ) + follow(𝑏, 𝛽, 𝑚 2 )

follow(𝑏, 𝛽, 𝑚 ) = (follow(𝑏 ∨ final(𝛽, 𝑚), 𝛽, 𝑚))

+

read(𝑎, Q (𝑣)) = Q (𝑣)

read(𝑎, 𝑚 1 ⋅ 𝑚 2 ) = read(𝑎, 𝑚 1 ) ⋅ read(𝑎, 𝑚 2 )

∗

∗

read(𝑎, 𝑚 ) = read(𝑎, 𝑚)

∗

𝑉

shift ∶ 2 × MReg → MReg
+

-

shift(𝛽, 𝑟 ) = 𝑟 , if 𝑟 ∈ {𝜀, Q (𝑣), Q (𝑣)}
𝑉

init ∶ 2 × MReg → MReg

shift(𝛽, 𝜎) = shift(𝛽, 𝜎) = 𝜎

+

init(𝛽, 𝜎) = init(𝛽, 𝜎) = 𝜎

′′

shift(𝛽, 𝑚 1 ⋅ 𝑚 2 ) = shift(𝛽, 𝑚 1 ) ⋅ 𝑚 2 , where

init(𝛽, 𝑚 1 + 𝑚 2 ) = init(𝛽, 𝑚 1 ) + init(𝛽, 𝑚 2 )

init(𝛽, shift(𝛽, 𝑚 2 )) if final(𝛽, 𝑚 1 ) = 1
′′
𝑚2 = {
shift(𝛽, 𝑚 2 )
otherwise
∗

′

init(𝛽, 𝑚 1 ⋅ 𝑚 2 ) = init(𝛽, 𝑚 1 ) ⋅ 𝑚 2 ,where

′′ ∗

init(𝛽, 𝑚 2 ) if nullable(𝛽, 𝑚 1 ) = 1
′
𝑚2 = {
𝑚2
otherwise

shift(𝛽, 𝑚 ) = (𝑚 ) , where
init(𝛽, shift(𝛽, 𝑚))
𝑚 ={
shift(𝛽, 𝑚)
′′

-

init(𝛽, 𝑟 ) = 𝑟 , if 𝑟 ∈ {𝜀, Q (𝑣), Q (𝑣)}

shift(𝛽, 𝑚 1 + 𝑚 2 ) = shift(𝛽, 𝑚 1 ) + shift(𝛽, 𝑚 2 )

if final(𝛽, 𝑚) = 1
otherwise

∗

init(𝛽, 𝑚 ) = init(𝛽, 𝑚)

∗

Figure 2. Operations on MReg expressions
2. Suppose ∐︀𝑤, 𝛽 ∗ ⋅ 𝛽̃︀ ∈ ⎷shift(𝛽 1,𝑚)⌄. Then, there exists
some 𝑎 0 ∈ Σ such that forall 𝛽 0 ∈ 2𝑉 , ∐︀𝑎 0 ⋅ 𝑤, 𝛽 0 𝛽 1 ⋅ 𝛽̃︀ ∈
⎷𝑚⌄.

following way:
follow(0, 𝛽 0,𝑚) = shift(𝛽 0,𝑚)
follow(1, 𝛽 0,𝑚) = init(𝛽 0, shift(𝛽 0,𝑚))

Example 19. We have init(𝛽 0, 𝑎𝑏𝑐 + 𝑑𝑒) = 𝑎𝑏𝑐 + 𝑑𝑒 for all
𝛽 0 . Shifting would move each mark to the next position,
i.e., shift(𝛽 0, 𝑎𝑏𝑐 + 𝑑𝑒) = 𝑎𝑏𝑐 + 𝑑𝑒. When there are queries
guarding the expression, the supplied valuation must satisfy the query in order to place the mark. For example,
init(𝛽 0, Q+ (𝑣)𝑎𝑏𝑐) = Q+ (𝑣)𝑎𝑏𝑐 only 𝛽 0 (︀𝑣⌋︀ = 1. Similarly,
shift(𝛽 0, 𝑎Q+ (𝑣)𝑏𝑐) = 𝑎Q+ (𝑣)𝑏𝑐 only if 𝛽 0 (︀𝑣⌋︀ = 1.

These equivalences can be proven using a straightforward
induction once the idempotence of the function init is shown.
Informally, the function init is used to place marks at initial
positions in the expression, and the function shift is used
to move the marks to the next positions. This intuition is
formalized in the following lemmas.
Lemma 18. Let 𝑚 ∈ MReg and 𝛽 0, 𝛽 ∗ ∈ 2𝑉 , 𝛽 ∈ (2𝑉 )∗ and
𝑤 ∈ Σ∗ . Then, the function init satisfies the following properties.
1. ⎷𝑚⌄ ⊆ ⎷init(𝛽 0,𝑚)⌄
2. Suppose 𝑤 ≠ 𝜀. Suppose ∐︀𝑤, 𝛽 0 ⋅ 𝛽̃︀ ∈ Jstrip(𝑚)K. Then,
∐︀𝑤, 𝛽 0 ⋅ 𝛽̃︀ ∈ ⎷init(𝛽 0,𝑚)⌄.
3. Suppose ∐︀𝑤, 𝛽 ∗ ⋅𝛽̃︀ ∈ ⎷init(𝛽 0,𝑚)⌄. Then either ∐︀𝑤, 𝛽 ∗ ⋅
𝛽̃︀ ∈ ⎷𝑚⌄ or ∐︀𝑤, 𝛽 0 ⋅ 𝛽̃︀ ∈ Jstrip(𝑚)K.
Next, let 𝛽 1 ∈ 2𝑉 and 𝑎 0 ∈ Σ. Then, the function shift satisfies
the following properties.
1. Suppose 𝑤 ≠ 𝜀 and ∐︀𝑎 0 ⋅ 𝑤, 𝛽 0 𝛽 1 ⋅ 𝛽̃︀ ∈ ⎷𝑚⌄. Then,
∐︀𝑤, 𝛽 1 ⋅ 𝛽̃︀ ∈ ⎷shift(𝛽 1,𝑚)⌄.

4.2

Caching final and nullable for Marked
Expressions

We wish to compute follow(𝑚) in 𝑂(⋃︀𝑚 ⋃︀) time. However,
translating the definitions from Figure 2 will result in an
algorithm that would take 𝑂(⋃︀𝑚 ⋃︀2 ) time in the worst case.
This is because the functions final and nullable are called repeatedly on the same subexpressions. This can be avoided by
augmenting the type of expressions to store these additional
results. We call these expressions marked expressions with
caching, denoted by CMReg. In Coq, we define the mutually
inductive types CMRegex and CMRe as follows. Note that with

205

<!-- page 9 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

this definition, the functions cFinal and cNullable are simply field accessors which run in 𝑂(1) time, which store the
results of final and nullable respectively.

sync ∶ 2𝑉 × CMReg → CMReg
sync(𝛽, 𝜀) = mkEps
sync(𝛽, 𝜎) = mkCharClass(0, 𝜎)
sync(𝛽, 𝜎) = mkCharClass(1, 𝜎)

Inductive CMRegex : Type :=
| MkCMRegex : bool -> bool -> CMRe -> CMRegex
with CMRe : Type :=
| CMEpsilon : CMRe
...
| CMStar : CMRegex -> CMRe.

sync(𝛽, Q+ (𝑣)) = mkQueryPos(𝛽(︀𝑣⌋︀, 𝑣)
sync(𝛽, Q- (𝑣)) = mkQueryNeg(𝛽(︀𝑣⌋︀, 𝑣)
sync(𝛽, 𝑚 1 + 𝑚 2 ) = mkUnion(sync(𝛽, 𝑚 1 ), sync(𝛽, 𝑚 2 ))
sync(𝛽, 𝑚 1 ⋅ 𝑚 2 ) = mkConcat(sync(𝛽, 𝑚 1 ), sync(𝛽, 𝑚 2 ))
sync(𝛽, 𝑚 ∗ ) = mkStar(sync(𝛽, 𝑚))

Definition cNullable (r : CMRegex) : bool :=
match r with | MkCMRegex b _ _ => b end.
Definition cFinal (r : CMRegex) : bool :=
match r with | MkCMRegex _ b _ => b end.
Definition cRe (r : CMRegex) : CMRe :=
match r with | MkCMRegex _ _ re => re end.

cFollow ∶ B × CMReg → CMReg
cFollow(𝑏, 𝜀) = mkEps
cFollow(𝑏, 𝜎) = cFollow(𝑏, 𝜎) = mkCharClass(𝑏, 𝜎)
cFollow(𝑏, Q+ (𝑣)) = mkQueryPos(cNullable(Q+ (𝑣)), 𝑣)
cFollow(𝑏, Q- (𝑣)) = mkQueryNeg(cNullable(Q- (𝑣)), 𝑣)

An interesting consideration here is the induction principle that allows us to prove properties involving CMRegex
and CMRe which are mutually recursive. The auto-generated
induction principle is insufficient since the subexpressions of
CMRe (respectively, CMRegex) are nested inside an additional
layer of CMRegex (respectively, CMRe). We write our custom
induction principle in the term language, which is later used
to prove properties.
Given 𝑐 ∈ CMReg, we write unCache(𝑐) ∈ MReg to denote the underlying marked regular expression. We say that
𝑐 is synced to a valuation 𝛽 0 ∈ 𝑉 if cNullable(𝑐) and cFinal(𝑐)
are nullable(𝛽 0, unCache(𝑐)) and final(𝛽 0, unCache(𝑐)) respectively. In order to streamline our functions and proofs,
we define smart constructors mkEps, mkCharClass, etc
which propagate the caching information. These have the
property that if the supplied arguments are synced with
respect to some valuation, then so is the resulting expression. The explicit definition and property for mkConcat are
shown. Compare this with the cases of concatenation in the
definition of nullable and final in Figure 2.

cFollow(𝑏, 𝑚 1 + 𝑚 2 ) = mkUnion(cFollow(𝑏, 𝑚 1 ), cFollow(𝑏, 𝑚 2 ))
cFollow(𝑏, 𝑚 1 ⋅ 𝑚 2 ) = mkConcat(cFollow(𝑏, 𝑚 1 ), cFollow(𝑏 ′ , 𝑚 2 ))
where 𝑏 ′ = cFinal(𝑚 1 ) ∨ (𝑏 ∧ cNullable(𝑚 1 ))
cFollow(𝑏, 𝑚 ∗ ) = mkStar(cFollow(𝑏 ∨ cFinal(𝑚), 𝑚))
cRead ∶ Σ × CMReg → CMReg
cRead(𝑎, 𝜀) = mkEps
cRead(𝑎, 𝜎) = mkCharClass(0, 𝜎)
)︀
⌉︀
⌉︀mkCharClass(1, 𝜎) if 𝜎(𝑎) = 1
cRead(𝑎, 𝜎) = ⌋︀
⌉︀
⌉︀
]︀mkCharClass(0, 𝜎) otherwise
+
cRead(𝑎, Q (𝑣)) = mkQueryPos(cNullable(Q+ (𝑣)), 𝑣)
cRead(𝑎, Q- (𝑣)) = mkQueryNeg(cNullable(Q- (𝑣)), 𝑣)
cRead(𝑎, 𝑚 1 + 𝑚 2 ) = mkUnion(cRead(𝑎, 𝑚 1 ), cRead(𝑎, 𝑚 2 ))
cRead(𝑎, 𝑚 1 ⋅ 𝑚 2 ) = mkConcat(cRead(𝑎, 𝑚 1 ), cRead(𝑎, 𝑚 2 ))
cRead(𝑎, 𝑚 ∗ ) = mkStar(cRead(𝑎, 𝑚))

Figure 3. Operations on CMReg expressions
1. The expression sync(𝛽 0,𝑚) is synced with respect to
𝛽 0 and it preserves the underlying marked expression,
i.e., unCache(sync(𝛽 0,𝑚)) = 𝑚.
2. If 𝑚 is synced w.r.t. 𝛽 0 , then so is cRead(𝛽 0,𝑚).
3. The function cRead simulates read on the underlying MReg expression, i.e., unCache(cRead(𝑎,𝑚)) =
read(𝑎, unCache(𝑚))
4. When 𝑚 is synced to 𝛽 0 , unCache(cFollow(𝑏,𝑚)) =
follow(𝑏, 𝛽 0, unCache(𝑚)). In other words, the function cFollow simulates follow on the underlying MReg
expression.

Definition mkConcat (r1 r2 : CMRegex) : CMRegex :=
MkCMRegex
(cNullable r1 && cNullable r2)
((cFinal r1 && cNullable r2) || cFinal r2)
(CMConcat r1 r2).

We manipulate the cached expressions using the functions sync, cFollow and cRead as defined in Figure 3. The
operations cFollow and cRead are the versions of follow and
read on MReg expressions, respectively, that work on the
cached version. The operation sync is defined in a manner
that updates the cNullable and cFinal fields of the expression using a given oracle valuation. All of these functions are
defined using the ‘smart constructors’ mkXXX. Their formal
relationship with the corresponding functions for MReg is
summarized in the following lemmas.

These lemmas establish the correspondence between the
operations on MReg and their cached version CMReg.
4.3

Consuming Oracle Strings

We define the function consume, whose effect is to simulate
moving tokens in the corresponding NFA whose paths are
labelled by the supplied string. We write toCached ∶ MReg →

Lemma 20. Let 𝑚 ∈ MReg and 𝛽 0 ∈ 2𝑉 . Then, the following
holds.

206

<!-- page 10 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

5.1

CMReg to denote the function which initializes the caching
information to 0 for a given expression in MReg.

In Lemma 11, we see that the appropriate oracle valuations
for the oracle regular expression abstract(𝑟 ) is given in terms
of oval(𝑟, 𝑤), which is defined in terms of tapes of the maximal lookarounds. Here, we show how these can be computed
using other existing concepts. One important detail here is
that we need to scan the strings in reverse order in order to
compute oracle valuations for lookahead.
When 𝑟 ∈ LReg or 𝑟 ∈ OReg, we write rev(𝑟 ) to denote its
reversal, which is defined in the usual way. Notably, rev on
LReg interchanges lookaheads and lookbehinds but rev on
OReg fixes oracle queries. The reversals satisfy the following
useful properties.

Definition 21 (Consuming OStrings). Let 𝑟 ∈ OReg, and
𝑤 = 𝑎 0 ⋅ 𝑎 1 ⋅ . . . ⋅ 𝑎𝑛−1 ∈ Σ∗ , and 𝛽 = 𝛽 0 ⋅ 𝛽 1 ⋅ . . . ⋅ 𝛽𝑛 ∈ (2𝑉 )∗ . We
define the series of expressions 𝑚 0,𝑚 1 . . . 𝑚𝑛 ∈ CMReg and
associated Boolean values 𝑏 0, 𝑏 1 . . . 𝑏𝑛 as follows.
𝑚 0 = sync(𝛽 0, toCached(toMarked(𝑟 )))
𝑚 1 = sync(𝛽 1, cRead(𝑎 0, cFollow(1,𝑚 0 )))
𝑚𝑖+1 = sync(𝛽𝑖+1, cRead(𝑎𝑖 , cFollow(0,𝑚𝑖 )))
𝑏 0 = cNullable(𝑚 0 )
𝑏𝑖+1 = cNullable(𝑚𝑖+1 )
We define oMatch(𝑟, ∐︀𝑤, 𝛽̃︀) = (︀𝑏 0, 𝑏 1, . . . , 𝑏𝑛 ⌋︀.

Lemma 24. Let 𝑟 ∈ LReg, 𝑠 ∈ OReg, 𝑟ˆ = abstract(𝑟 ), 𝑤 ∈ Σ∗ ,
𝛽 = oval(𝑟, 𝑤) and 𝛽 ′ ∈ (2𝑉 )⋃︀𝑤⋃︀+1 . Then, the following hold:

Theorem 22 (Correctness of oMatch). Let 𝑟 ∈ OReg, and
∐︀𝑤, 𝛽̃︀ ∈ 𝒪(Σ,𝑉 ). The list oMatch(∐︀𝑤, 𝛽̃︀) records whether
∐︀𝑤, 𝛽̃︀(︀0, 𝑖⌋︀ ∈ J𝑟 K, i.e., oMatch(𝑟, ∐︀𝑤, 𝛽̃︀) = tape(𝑟, ∐︀𝑤, 𝛽̃︀).

1. 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 ⇐⇒ rev(𝑤), (︀⋃︀𝑤 ⋃︀ − 𝑗, ⋃︀𝑤 ⋃︀ − 𝑖⌋︀ ⊧ rev(𝑟 )
2. ∐︀𝑤, 𝛽 ′ ̃︀, (︀𝑖, 𝑗⌋︀ ⊧ 𝑠 ⇐⇒ ∐︀rev(𝑤), rev(𝛽 ′ )̃︀, (︀⋃︀𝑤 ⋃︀ − 𝑗, ⋃︀𝑤 ⋃︀ −
𝑖⌋︀ ⊧ rev(𝑠)
3. tape(◁, 𝑟, 𝑤) = rev(tape(▷, rev(𝑟 ), rev(𝑤)))
4. tape(rev(𝑟 ), rev(𝑤))
=
tape(rev(ˆ𝑟 ), ∐︀rev(𝑤), rev(𝛽)̃︀)

Proof. (Sketch) The key facts needed are the following: (1)
∐︀𝜀, 𝛽 0 ̃︀ ∈ J𝑟 K iff cNullable(𝑚 0 ) = 1, and (2) For 0 < 𝑖 ≤ ⋃︀𝑤 ⋃︀,
then ∐︀𝑤, 𝛽̃︀(︀0, 𝑖⌋︀ ∈ J𝑟 K iff cFinal(𝑚𝑖 ) = 1. These would follow
from the properties of nullable and final (Lemma 14), and the
fact that cFollow and cRead simulate the functions cFollow
and cRead from subsection 4.1 (Lemma 20).
□

Part (1) and (2) of the lemma are characteristic properties
of reversals of LReg and OReg expressions respectively. Part
(3) gives us a way to express tape(◁) in terms of tape(▷).
It is a direct consequence of (1). Part (4) follows from Part
(1) and Lemma 11. This tells us that we can work with the
reversal of abstract(𝑟 ) instead of 𝑟 itself, and saves us from
repeatedly reversing subexpressions, which is crucial for the
linear time complexity of the algorithm.

Theorem 23 (Resource Usage of oMatch). Suppose 𝑟 ∈
OReg with ⋃︀𝑟 ⋃︀ = 𝑚 and ∐︀𝑤, 𝛽̃︀ ∈ 𝒪(Σ,𝑉 ) with ⋃︀𝑤 ⋃︀ = 𝑛. Then,
one can compute oMatch(𝑟, ∐︀𝑤, 𝛽̃︀) in 𝑂(𝑚 ⋅ 𝑛) time in a
streaming manner that requires an additional 𝑂(𝑚) state
space.
Proof. With the caching trick explained in subsection 4.2, we
know that reading off the cFinal and cNullable fields can
be done in 𝑂(1) time. Similarly, applications of the smart
constructors mkEps, mkCharClass, etc also take 𝑂(1) time.
Using this information, we can show that the functions sync,
cFollow and cRead in Figure 4 can be computed in 𝑂(𝑚)
time, since they access each of their subexpressions exactly
once. The function applies cRead using each character in 𝑤,
sync using each valuation in 𝛽, and uses cFollow exactly ⋃︀𝑤 ⋃︀
times. Thus, the functions of complexity 𝑂(𝑚) are iterated
𝑂(𝑛) times, giving us a total time complexity of 𝑂(𝑚 ⋅ 𝑛).
Each bit of oMatch can be computed in a streaming manner: we only need to store the additional state represented as
𝑚𝑖 as defined in Definition 21. This requires 𝑂(𝑚) space. □

5

Computing Tapes

5.2

Matching Algorithm

In this section, we will describe the function eval which given
𝑟 ∈ LReg and 𝑤 ∈ Σ∗ computes abstract(𝑟 ) and oval(𝑟, 𝑤)
together using oMatch from Definition 21.
The definition of the function eval shown in Figure 4, is
given in terms of the auxiliary function evalAux. The function evalAux is given five arguments: 𝑤, 𝑤, 𝑟 , 𝑖 and 𝑇 . The
first two arguments are the string and its reversal. We compute the reversal in advance in the function eval so that
we do not repeatedly reverse the string in recursive calls
of evalAux. The third argument is the expression 𝑟 that we
want to abstract. The fourth argument is the index 𝑖 used
for computing indices of the queries. The fifth argument is a
list of tapes, which have already been computed. The output
has three components, 𝑠, 𝑛 and 𝑇 . The first component 𝑠
is intended to be abstract(𝑟 ). The second component 𝑛 is
arity(𝑟 ). And the last component 𝑇 consists of tape(𝑚𝑖 , 𝑤)
for each maximal lookarounds 𝑚𝑖 in 𝑟 . However, these tapes
appear in reverse order. This is because we add each new
tape to the front of the list 𝑇 for the sake of efficiency. The
formal connection is stated in the following lemma.

Efficient Layerwise Matching

In Section 3, we have shown that oracle regular expressions
can be used to evaluate regular expressions with lookahead
if the truth values of the lookaheads are supplied as oracle
valuations. In Section 4, we have demonstrated how oracle
regular expressions can be matched on oracle strings. Now,
we will show how to combine these two results to match
regular expressions with lookahead in an efficient manner.

207

<!-- page 11 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

// Computes the leftmost longest match
Definition llMatch (𝑟 ∶ LReg) (𝑤 ∶ list Σ) ∶ Option(N × N) :=
let (ˆ𝑟, 𝛽) = eval(𝑟, 𝑤) in
let 𝑡 1 = oMatch(Σ∗ ⋅ rev(ˆ𝑟 ), ∐︀rev(𝑤), rev(𝛽)̃︀) in
let 𝑖 ′ be the largest index of 𝑡 1 which is 1 in
match i’ with
None Ô⇒ None
Some(𝑖 ′ ) Ô⇒
let 𝑖 = ⋃︀𝑤⋃︀ − 𝑖 ′ in
let 𝑡 2 = oMatch(ˆ𝑟, ∐︀𝑤, 𝛽̃︀(︀𝑖, ⋃︀𝑤⋃︀⌋︀) in
let 𝑑 be the largest index of 𝑡 2 which is 1 in
Some((︀𝑖, 𝑖 + 𝑑⌋︀) // 𝑑 must exist
// Computes whether 𝑤, (︀𝑖, 𝑖 + 𝑘⌋︀ ⊧ 𝑟 for all 0 ≤ 𝑘 ≤ 𝑗 − 𝑖
Definition match (𝑟 ∶ LReg) (𝑤 ∶ list Σ) (𝑖, 𝑗 ∶ N) ∶ list B :=
let (ˆ𝑟, 𝛽) = eval(𝑟, 𝑤(︀𝑖, 𝑗⌋︀) in
oMatch(ˆ𝑟, ∐︀𝑤, 𝛽̃︀(︀𝑖, 𝑗⌋︀)
// Computes ∐︀abstract(𝑟 ), oval(𝑟, 𝑤)̃︀
Definition eval (𝑟 ∶ LReg) (𝑤 ∶ list Σ) ∶ OReg × list B :=
let (ˆ𝑟, _,𝑇 ) = evalAux(𝑤, rev(𝑤), 𝑟, 0, (︀⌋︀) in
let 𝛽 = transpose(rev(𝑇 )) in
(ˆ𝑟, 𝛽)
// Ensure 𝑤 = rev(𝑤)
// Appends the tapes for the maximal lookarounds of 𝑟 to 𝑇
Fixpoint evalAux (𝑤, 𝑤 ∶ list Σ) (𝑟 ∶ LReg)
(𝑖 ∶ N) (𝑇 ∶ list list B) ∶ OReg × N × list list B :=
match r with
𝜀 Ô⇒ (𝜀, 0,𝑇 )
𝜎 Ô⇒ (𝜎, 0,𝑇 )
𝑟 1 ○ 𝑟 2 where ○ ∈ {⋅, +} Ô⇒
let (𝑠 1 , 𝑛 1 ,𝑇 ′ ) = evalAux(𝑤, 𝑤, 𝑟 1 , 𝑖,𝑇 ) in
let (𝑠 2 , 𝑛 2 ,𝑇 ′′ ) = evalAux(𝑤, 𝑤, 𝑟 2 , 𝑖 + 𝑛 1 ,𝑇 ′ ) in
(𝑠 1 ○ 𝑠 2 , 𝑛 1 + 𝑛 2 ,𝑇 ′′ )
𝑟 ∗ Ô⇒
let (𝑠, 𝑛,𝑇 ′ ) = evalAux(𝑤, 𝑤, 𝑟, 𝑖,𝑇 ) in
(𝑠 ∗ , 𝑛,𝑇 ′ )
(?< 𝑟 ) Ô⇒
let (𝑠, 𝛽) = eval(𝑟, 𝑤) in
let tape = oMatch(𝑠, ∐︀𝑤, 𝛽̃︀) in
let 𝑞 = Q+ (𝑣𝑖 ) in
(𝑞, 1, (︀tape⌋︀ ++ 𝑇 )
(?⇑
< 𝑟 ) Ô⇒
let (𝑠, 𝛽) = eval(𝑟, 𝑤) in
let tape = rev(oMatch(𝑠, ∐︀𝑤, 𝛽̃︀)) in
let 𝑞 = Q+ (𝑣𝑖 ) in
(𝑞, 1, (︀tape⌋︀ ++ 𝑇 )
(?> 𝑟 ) Ô⇒
let (𝑠, 𝛽) = eval(𝑟, 𝑤) in
let tape = rev(oMatch(rev(𝑠), ∐︀𝑤, rev(𝛽)̃︀)) in
let 𝑞 = Q+ (𝑣𝑖 ) in
(𝑞, 1, (︀tape⌋︀ ++ 𝑇 )
(?⇑
> 𝑟 ) Ô⇒
let (𝑠, 𝛽) = eval(𝑟, 𝑤) in
let tape = oMatch(𝑠, ∐︀𝑤, 𝛽̃︀) in
let 𝑞 = Q+ (𝑣𝑖 ) in
(𝑞, 1, (︀tape⌋︀ ++ 𝑇 )

2. 𝑛 = arity(𝑟 ), the number of maximal lookarounds in 𝑟 .
3. 𝑇 ′ = 𝑇 ′′ ++ 𝑇 , where 𝑇 ′′ is a list of ⋃︀𝑤 ⋃︀ + 1 length sequences added to 𝑇 . There are 𝑛 = arity(𝑟 ) sequences
in 𝑇 ′′ . Furthermore, 𝑇 ′′ consists of the tape of the
maximal lookarounds stacked in reverse order, i.e.,
𝑇 ′′ (︀𝑛 − 𝑗 + 1⌋︀ = tape(maxLk(𝑟 )(︀𝑗⌋︀, 𝑤) for all 0 ≤ 𝑗 < 𝑛.
As a result, if (ˆ𝑟, 𝛽) = eval(𝑟, 𝑤), then, 𝑟ˆ = abstract(𝑟 ) and
𝛽 = oval(𝑟, 𝑤).
The proof of this theorem makes use of the correctness of
oMatch, (i.e., Theorem 22) along with Lemma 11 (about the
connection of oval and abstract) and Lemma 24 (about the
connection of tape(▷) and tape(◁)).
A match for 𝑟 on 𝑤 is a window (︀𝑖, 𝑗⌋︀ such that 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 .
Call a match (︀𝑖, 𝑗⌋︀ maximal if it is not subsumed by any other
match (︀𝑖 ′, 𝑗 ′ ⌋︀ where 𝑖 ′ ≤ 𝑖 ≤ 𝑗 ≤ 𝑗 ′ with either 𝑖 ′ < 𝑖 or 𝑗 < 𝑗 ′ .
The leftmost longest match is a maximal match (︀𝑖, 𝑗⌋︀ where 𝑖
has the least possible value. For a given regular expression 𝑟
and a word 𝑤, this match is uniquely defined. The problem
of extracting the leftmost-longest match has been discussed
before (see, e.g., the notes by Russ Cox [12]) and is considered
the POSIX standard.
The function match passes the results of eval(𝑟, 𝑤) to
oMatch to check if we have a match. The computation of
the function llMatch also uses the results of eval(𝑟, 𝑤), and
it makes two passes to identify the two end points. The following theorem follows from Lemma 25 (establishing the
correctness of eval), Theorem 22 (establishing the correctness of oMatch) and Lemma 11 (establishing the connection
between abstract(𝑟 ) and oval(𝑟, 𝑤)).
Theorem 26 (Correctness of match, llMatch). Let 𝑟 ∈
LReg, 𝑤 ∈ Σ∗ and (︀𝑖, 𝑗⌋︀ a window in 𝑤. Then,
)︀
⌉︀
⌉︀1 if 𝑤, (︀𝑖, 𝑖 + 𝑘⌋︀ ⊧ 𝑟
match(𝑟, 𝑤, 𝑖, 𝑗)(︀𝑘⌋︀ = ⌋︀
⌉︀
0 if 𝑤, (︀𝑖, 𝑖 + 𝑘⌋︀ ⇑
⊧𝑟
⌉︀
]︀
for all 0 ≤ 𝑘 ≤ 𝑗 − 𝑖.
If, llMatch(𝑟, 𝑤) = Some((︀𝑖, 𝑗⌋︀), then 𝑤, (︀𝑖, 𝑗⌋︀ ⊧ 𝑟 and
(︀𝑖, 𝑗⌋︀ is the leftmost maximal match for 𝑟 on 𝑤. Otherwise if llMatch(𝑟, 𝑤) = None, for any window (︀𝑖, 𝑗⌋︀ in 𝑤,
𝑤, (︀𝑖, 𝑗⌋︀ ⊧
⇑ 𝑟.
In [31], the authors show that the function match can be
computed in linear time and requires a linear amount
of space. This continues to hold for our implementation as
well, where the main difference is that we have implemented
oNFA execution using marked regular expressions. However,
the resource consumption of matching using marked regular
expressions is exactly the same as shown in Theorem 23.
We note some important implementation details for the
function eval. Consider a regular expression 𝑟 of size 𝑚,
which has 𝑂(𝑚) layers of lookaheads. In the computation of
eval, we would need to reverse the given string 𝑂(𝑚) times.
However, we have avoided this by reversing the string in
advance before passing it to evalAux. Similarly, we may have

Figure 4. The Matching Algorithm
Lemma 25 (Behavior of evalAux and eval). Suppose 𝑤 ∈
Σ∗ , 𝑤 = rev(𝑤), 𝑟 ∈ LReg, 𝑖 ∈ N , and 𝑇 is a list of sequences
each of length ⋃︀𝑤 ⋃︀+1. Define (𝑠, 𝑛,𝑇 ′ ) = evalAux(𝑤, 𝑤, 𝑟, 𝑖,𝑇 ).
Then, the following holds:
1. 𝑠 = abstract𝑖 (𝑟 ) (see Definition 7).

208

<!-- page 12 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

to reverse the 𝑟 itself 𝑂(𝑚) times. However, after processing
each lookaround 𝑟𝑖 of 𝑟 , we work instead with an abstracted
version of 𝑟 in which 𝑟𝑖 has been replaced by a query. If
⋃︀𝑟𝑖 ⋃︀ = 𝑚𝑖 then, the abstracted version of 𝑟 has size 𝑚 − 𝑚𝑖 ,
lowering the cost of subsequent reversals. These implementation details keep the time complexity of eval linear, instead
of quadratic.

6

version 20.0.2. Our extracted Haskell code is compiled with
ghc version 9.4.8.

7

Related Work

The widely used regex engines which do support
lookarounds are all based on backtracking, and can thus take
exponential time to match in the worst-case scenario (even in
the absence of lookarounds). Some of them, including PCRE
and Python, do not allow unbounded lookbehinds. A notable
exception is the integration of the RegElk matcher [5] with
the Javascript engine V8, which is part of the Chromium
browser engine. The linear time algorithm for matching regular expressions with lookarounds we have presented in
this paper is based on the techniques from [31]. The key
difference in our implementation is the purely functional approach, in particular the use of Marked Regular Expressions
instead of explicit NFAs. The approach of using marked regular expressions to simulate NFAs was discussed in [4, 19].
Formalized versions of Marked Expressions in Isabelle/HOL
are seen in [36] and [3]. A discussion on two variants for the
marking of regular expressions can be found in [36]. While
marked regexes have been discussed and formalized before,
our paper introduces their generalization to expressions with
oracle queries. This addition presents a substantial challenge
in the formalization, since the change of the language ⎷𝑟 ⌄
produced when operations are applied on the marked regex
𝑟 (i.e, the lemmas in Section 4.1) are much less elegant.
What we have described as o-strings in this paper (following the terminology of [31]) are widely known in the
literature as guarded strings. They were introduced by [25]
as an abstraction for program semantics. Note that such
strings also arise as traces over labelled transition systems.
A closely notion is that of Kleene Algebra with Tests (KAT)
[28]. KAT Expressions form a Kleene Algebra with a Boolean
subalgebra known as the algebra of tests. In particular, the
concatenation operator behaves as conjunction on the tests,
and additionally they come equipped with the notion of negation. Other than languages over guarded strings, KAT expressions can be interpreted over binary relations or semantics
of programs. In fact, KAT expressions subsume propositional
Hoare logic. Our Oracle Regular Expressions are a special
case of KAT expressions, where the oracle queries are the
tests, and the proposed syntax expresses them in negation
normal form. Automata on Guarded strings, similar to the
notion of Oracle-NFAs in [31], have been studied in [29].
These can be viewed as a generalization of 𝜀-NFAs: the 𝜀
transition on NFAs are akin to a trivial test, while transitions
with tests may perform multiple tests in succession without
consuming additional characters. Pous [39] has formalized
the theory of KAT expressions in Coq (including their completeness theorem, decision procedures for equivalence and
models including guarded strings) for the purpose of reasoning about while-programs. Similar work for Schematic

Experiments

We extracted our Coq scripts into Haskell code and conducted experiments on to empirically confirm the claim that
our verified matching algorithm takes linear time both in
terms of the size of the string and that of the expresion.
We have chosen three families of synthetic regexes (shown
in PCRE notation, in Table 1) which we have called DNLA,
NX and ND. The family DNLA consists of increasingly many
disjuncts of negative lookaheads, while NX and ND consist
of increasingly nested lookaheads. Observe that the size of
the regular expressions in each family increases linearly
with its parameter. For the families NX and ND, we use input
strings of the form 𝑎𝑛 ; and for the family DNLA, we use input
strings of the form 𝑒 𝑛 ⋅ abcd. These expressions and inputs
are deliberately chosen so that they would have been computationally challenging for a naive implementation (such
as backtracking) of lookahead matching.
In Figure 5, the running time of the algorithm is plotted
against the length of the input string. In Figure 6, the running
time of the algorithm is plotted against the size of the regular
expression, while keeping the input string fixed. The points
shown in the figures are the average of 10 trials. In both
cases, we can observe linear trends, validating our claim of
linear time complexity.
Our experiments in Figure 7 demonstrate that regular expressions chosen in Table 1 were chosen so that they would
be computationally challenging for naive implementations of
lookaround matching. They also serve as a preliminary comparison between the performance of our extracted Haskell
code, and other available tools for matching expressions with
lookarounds. We have measured the performances of the
expressions in Table 1 on PCRE [43], java.util.regex of
the Java standard library [37] and the tool developed and
verified in Lean presented in [45]. The Lean tool is based
on Brzozowski derivatives [9], and the two industrial tools
are backtracking-based. The results are shown in Figure 7.
We can observe that the performance of PCRE, Java and the
Lean tool are not linear in the size of the input string, in
general.
Experimental Setup. The experiments were conducted
on a MacBook Air M2 (8 cores) running Ventura 13.6.1 with
8GB of RAM. We used PCRE version 10.42 and Apple clang
version 14.0.3 for compilation. The Lean code and proofscripts are taken from [45] and built with Lean 4 (version
4.5.0-rc1). The runtime environment used for Java is Java SE

209

<!-- page 13 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

Table 1. Families of Regular Expressions in Experiments
Family
DNLA
NX
ND

𝑁 =1

𝑁 =2

𝑁 =3

(?!.*a.*).*
a(?=.*c)
a(?=.*b)

((?!.*a.*)|(?!.*b.*)).*
a(?=.*(?=.*c))
a((?=.*c)|.*a(?=.*b))

(((?!.*a.*)|(?!.*b.*))|(?!.*c.*)).*
a(?=.*(?=.*(?=.*c)))
a((?=.*d)|(.*a((?=.*c)|.*a(?=.*b))))

Total Time (ms)

DNLA

ND

⋯
⋯
⋯
⋯

NX

Parameter
1e+04
1
2
5e+03

3
4

0e+00
1e+05

2e+05

3e+05

1e+05

2e+05

3e+05

1e+05

2e+05

3e+05

Length of Input String

Figure 5. Performance of the matching algorithm on strings of increasing length
DNLA

ND

NX

Total Time (ms)

2.0e+04

Input Length
1.5e+04

5.0e+04
1.0e+05

1.0e+04

1.5e+05
5.0e+03

2.0e+05

0.0e+00
2

4

6

8

10

2

4

6

8

10

2

4

6

8

10

Regex Parameter

Figure 6. Performance of the matching algorithm on regular expressions of increasing size
KAT, a specific extension of KAT has been formalized in
Isabelle/HOL [2].
Techniques for the linear time matching of lookaround expressions have also been published in [5] and [21]. Barriere et
al [5] represent the NFAs using Virtual Machine instructions,
a technique introduced by Rob Pike [38] and popularized by
Russ Cox [12]. Fujinami and Hasuo [21] use a memoization
approach to filling in the oracle valuations, and use a formalism based on ‘NFA with sub-automata’. The backtrackingbased greedy strategy of matching regexes prefers the first
option 𝑟 1 in a union 𝑟 1 +𝑟 2 and prefers to lengthen the number
of blocks matching 𝑟 in a Kleene iteration 𝑟 ∗ . Compatibility with the greedy strategy is a consideration in [21] and
[5]. Berglund et al [7] define the semantics of lookaheads
using alternating finite automata (AFA). A consequence of
Berglund’s approach is that the string could be matched in
𝑂(𝑚 ⋅ 𝑛) time by running the AFA on the string in reverse,
but this approach does not directly support lookbehinds.

Zhuchko et al [45] have verified an algorithm for matching
regular expressions with lookarounds in Lean based on the
idea of location-based (i.e., context-dependent) derivatives
from [34]. We have found that this algorithm does not run
in linear time because of the unchecked growth of the size
of the derivatives. Derivatives are popular for functional and
verified implementations of regex matching. Coquand and
Siles [11] have formalized the notion of Brzozowski derivatives [9] using Coq. Antimirov’s [1] partial derivatives which
correspond to states of an NFA have been verified by [26]
and [33]. Stanford et al [42] have defined symbolic derivatives to deal with Boolean combinations of derivatives. This
work has been extended in [34, 44] and used by Moseley et
al [34] to develop a matching library in the .NET framework.
An extensive theory of regular languages, involving regular expressions, DFAs and NFAs were formalized in Coq by
Doczkal et al [16, 17]. A matrix-based NFA formalization in
Agda is seen in [18]. Kammar and Marek [24] have formalized a parser based on typed regular expressions in Idris. A

210

<!-- page 14 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

Regex Parameter
DNLA

2

3

4

NX

ND

4e+03

2e+03

2e+03

2e+03

1e+03

1e+05

2e+05

3e+05

0e+00

0e+00

1e+05

3e+05

1e+04

2e+05

1e+03

1e+05

1e+02

1e+05

2e+05

3e+05

25000

50000

75000

100000

250

500

750

1000

250

500

750

1000

100

200

300

400

1e+05
1e+04
lean

4e+05

extracted

4e+03

0e+00

Time (ms)

1

1e+03
1e+02
1e+01

0e+00
100

200

300

25

50

75

100

1.0e+04

1e+06

1e+05
1e+04

5.0e+03

1e+03

1e+04

1e+02

2.5e+03

pcre

7.5e+03

1e+02

1e+01
0.0e+00
1e+07

2e+07

3e+07

100

200

300

1.2e+05
1e+05
10000

9.0e+04

1e+04
1e+03

100

3.0e+04

1e+02

10

0.0e+00
1e+05

2e+05

3e+05

java

1000

6.0e+04

1e+01
100

200

300

Length of Input String

Figure 7. Performance of our extracted Haskell code (extracted), the Lean tool in [45] (lean), PCRE (pcre), and Java (java)
on the three different regex families DNLA, NX, and ND
Coq formalization of finite state Automata is also discussed
in [8] which they have verified in the context of Kleene Algebras. Recently, the semantics of Javascript regular expressions (including lookarounds) have been comprehensively
mechanized in Coq [15].

8

As future work, we plan to leverage our equational reasoning framework to verify the correctness of rewritingbased optimizations, such as the removal of lookarounds of
bounded length. A more thorough evaluation of the performance of our extracted code would involve measurement
of space usage, and comparison with additional tools such
as the one developed in [5]. It would also be interesting to
extend our mechanization to include algorithms for bounded
repetition (e.g., [30]) and for match extraction with greedy
or POSIX disambiguation (see, e.g., [20, 22, 32, 35]).

Conclusion and Future Work

In this paper, we have presented our formalization of a linear time algorithm for matching regular expressions with
lookarounds. We have also empirically verified our claims
of time complexity. Our formalization is around 10k lines of
Coq code (1.5k lines of definitions and 8.5k lines of proofs).
The code compiles, is axiom-free, and is available at [10].

Acknowledgments
This research was supported in part by the US National
Science Foundation award 2319572.

211

<!-- page 15 -->

Verified and Efficient Matching of Regular Expressions with Lookaround

CPP ’25, January 20–21, 2025, Denver, CO, USA

References

[17] Christian Doczkal and Gert Smolka. 2016. Two-Way Automata in
Coq. In Interactive Theorem Proving, Jasmin Christian Blanchette and
Stephan Merz (Eds.). Springer International Publishing, Cham, 151–
166.
[18] Denis Firsov and Tarmo Uustalu. 2013. Certified Parsing of Regular
Languages. In Certified Programs and Proofs, Georges Gonthier and
Michael Norrish (Eds.). Springer International Publishing, Cham, 98–
113.
[19] Sebastian Fischer, Frank Huch, and Thomas Wilke. 2010. A Play
on Regular Expressions: Functional Pearl. In Proceedings of the 15th
ACM SIGPLAN International Conference on Functional Programming.
Association for Computing Machinery, New York, NY, USA, 357–368.
https://doi.org/10.1145/1863543.1863594
[20] Alain Frisch and Luca Cardelli. 2004. Greedy Regular Expression Matching. In ICALP 2004 (LNCS, Vol. 3142), Josep Díaz, Juhani
Karhumäki, Arto Lepistö, and Donald Sannella (Eds.). Springer, Berlin,
Heidelberg, 618–629. https://doi.org/10.1007/978-3-540-27836-8_53
[21] Hiroya Fujinami and Ichiro Hasuo. 2024. Efficient Matching with
Memoization for Regexes with Look-around and Atomic Grouping. In
Programming Languages and Systems: 33rd European Symposium on
Programming, ESOP 2024, Held as Part of the European Joint Conferences
on Theory and Practice of Software, ETAPS 2024, Luxembourg City,
Luxembourg, April 6–11, 2024, Proceedings, Part II (Luxembourg City,
Luxembourg). Springer-Verlag, Berlin, Heidelberg, 90–118. https:
//doi.org/10.1007/978-3-031-57267-8_4
[22] Bjørn Bugge Grathwohl, Fritz Henglein, Ulrik Terp Rasmussen, Kristoffer Aalund Søholm, and Sebastian Paaske Tørholm. 2016. Kleenex:
Compiling Nondeterministic Transducers to Deterministic Streaming
Transducers. In Proceedings of the 43rd Annual ACM SIGPLAN-SIGACT
Symposium on Principles of Programming Languages (POPL ’16). ACM,
New York, NY, USA, 284–297. https://doi.org/10.1145/2837614.2837647
[23] Hyperscan 2023. Intel’s Hyperscan: A high-performance multiple
regex matching library. https://github.com/intel/hyperscan.
[24] Ohad Kammar and Katarzyna Marek. 2023. Idris TyRE: a dependently
typed regex parser. arXiv:2305.04480 [cs.PL]
[25] Donald M. Kaplan. 1969. Regular expressions and the equivalence
of programs. J. Comput. Syst. Sci. 3, 4 (Nov. 1969), 361–386. https:
//doi.org/10.1016/S0022-0000(69)80027-9
[26] Vladimir Komendantsky. 2012. Reflexive Toolbox for Regular Expression Matching: Verification of Functional Programs in Coq+Ssreflect.
In Proceedings of the Sixth Workshop on Programming Languages Meets
Program Verification (PLPV ’12). ACM, New York, NY, USA, 61–70.
https://doi.org/10.1145/2103776.2103784
[27] Dexter Kozen. 1994. A Completeness Theorem for Kleene Algebras
and the Algebra of Regular Events. Information and Computation 110,
2 (1994), 366–390. https://doi.org/10.1006/inco.1994.1037
[28] Dexter Kozen. 1997. Kleene Algebra with Tests. ACM Transactions
on Programming Languages and Systems 19, 3 (1997), 427–443. https:
//doi.org/10.1145/256167.256195
[29] Dexter Kozen. 2001. Automata on Guarded Strings and Applications.
Technical Report. USA.
[30] Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras. 2023.
Regular Expression Matching Using Bit Vector Automata. Proceedings
of the ACM on Programming Languages 7, OOPSLA1, Article 92 (2023),
30 pages. https://doi.org/10.1145/3586044
[31] Konstantinos Mamouras and Agnishom Chattopadhyay. 2024. Efficient Matching of Regular Expressions with Lookaround Assertions.
Proceedings of the ACM on Programming Languages 8, POPL, Article
92 (2024), 31 pages. https://doi.org/10.1145/3632934
[32] Konstantinos Mamouras, Alexis Le Glaunec, Wu Angela Li, and Agnishom Chattopadhyay. 2024. Static Analysis for Checking the Disambiguation Robustness of Regular Expressions. Proceedings of the
ACM on Programming Languages 8, PLDI, Article 231 (2024), 25 pages.
https://doi.org/10.1145/3656461

[1] Valentin Antimirov. 1996. Partial Derivatives of Regular Expressions
and Finite Automaton Constructions. Theoretical Computer Science
155, 2 (1996), 291–319. https://doi.org/10.1016/0304-3975(95)00182-4
[2] Alasdair Armstrong, Georg Struth, and Tjark Weber. 2013. Program
Analysis and Verification Based on Kleene Algebra in Isabelle/HOL. In
Interactive Theorem Proving, Sandrine Blazy, Christine Paulin-Mohring,
and David Pichardie (Eds.). Springer Berlin Heidelberg, Berlin, Heidelberg, 197–212.
[3] Andrea Asperti. 2012. A Compact Proof of Decidability for Regular Expression Equivalence. In Interactive Theorem Proving, Lennart Beringer
and Amy Felty (Eds.). Springer Berlin Heidelberg, Berlin, Heidelberg,
283–298.
[4] Andrea Asperti, Claudio Sacerdoti Coen, and Enrico Tassi. 2010. Regular Expressions, au point. arXiv:1010.2604 [cs.FL]
[5] Aurèle Barrière and Clément Pit-Claudel. 2024. Linear Matching
of JavaScript Regular Expressions. Proceedings of the ACM on Programming Languages 8, PLDI, Article 201 (June 2024), 25 pages.
https://doi.org/10.1145/3656431
[6] Martin Berglund, Frank Drewes, and Brink van der Merwe. 2014. Analyzing Catastrophic Backtracking Behavior in Practical Regular Expression Matching. In Automata and Formal Languages 2014 (AFL
2014) (Electronic Proceedings in Theoretical Computer Science (EPTCS),
Vol. 151), Zoltán Ésik and Zoltán Fülöp (Eds.). Open Publishing Association, 109–123. https://doi.org/10.4204/eptcs.151.7
[7] Martin Berglund, Brink van der Merwe, and Steyn van Litsenborgh.
2021. Regular Expressions with Lookahead. JUCS - Journal of Universal
Computer Science 27, 4 (2021), 324–340. https://doi.org/10.3897/jucs.
66330
[8] Thomas Braibant and Damien Pous. 2010. Deciding Kleene Algebras
in Coq. In ITP (LNCS, Vol. 6172). Springer, Edinburgh, United Kingdom,
163–178. https://doi.org/10.1007/978-3-642-14052-5_13
[9] Janusz A. Brzozowski. 1964. Derivatives of Regular Expressions. J.
ACM 11, 4 (oct 1964), 481–494. https://doi.org/10.1145/321239.321249
[10] Agnishom Chattopadhyay. 2024. Efficient Matching of Regular Expressions with Lookaround - Coq Source Code. https://github.com/
Agnishom/lregex/.
[11] Thierry Coquand and Vincent Siles. 2011. A Decision Procedure for
Regular Expression Equivalence in Type Theory. In CPP 2011 (LNCS,
Vol. 7086), Jean-Pierre Jouannaud and Zhong Shao (Eds.). Springer,
Berlin, Heidelberg, 119–134. https://doi.org/10.1007/978-3-642-253799_11
[12] Russ Cox. 2010. Regular Expression Matching in the Wild. https:
//swtch.com/~rsc/regexp/regexp3.html.
[13] James C. Davis, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. 2018. The Impact of Regular Expression Denial of Service
(ReDoS) in Practice: An Empirical Study at the Ecosystem Scale. In
Proceedings of the 2018 26th ACM Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of
Software Engineering (Lake Buena Vista, FL, USA) (ESEC/FSE 2018).
Association for Computing Machinery, New York, NY, USA, 246–256.
https://doi.org/10.1145/3236024.3236027
[14] James C. Davis, Francisco Servant, and Dongyoon Lee. 2021. Using
Selective Memoization to Defeat Regular Expression Denial of Service
(ReDoS). In 2021 IEEE Symposium on Security and Privacy (SP). IEEE,
USA, 1–17. https://doi.org/10.1109/SP40001.2021.00032
[15] Noé De Santo, Aurèle Barrière, and Clément Pit-Claudel. 2024. A Coq
Mechanization of JavaScript Regular Expression Semantics. Proceedings of the ACM on Programming Languages ICFP (Sept. 2024), 30 pages.
https://doi.org/10.48550/arXiv.2403.11919
[16] Christian Doczkal, Jan-Oliver Kaiser, and Gert Smolka. 2013. A Constructive Theory of Regular Languages in Coq. In Certified Programs
and Proofs, Georges Gonthier and Michael Norrish (Eds.). Springer
International Publishing, Cham, 82–97.

212

<!-- page 16 -->

CPP ’25, January 20–21, 2025, Denver, CO, USA

Agnishom Chattopadhyay, Angela W. Li, and Konstantinos Mamouras

[33] Nelma Moreira, David Pereira, and Simão Melo de Sousa. 2012. Deciding Regular Expressions (In-)Equivalence in Coq. In RAMiCS 2012
(LNCS, Vol. 7560), Wolfram Kahl and Timothy G. Griffin (Eds.). Springer,
Berlin, Heidelberg, 98–113. https://doi.org/10.1007/978-3-642-333149_7
[34] Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi,
Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu. 2023. Derivative
Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics. Proc. ACM Program. Lang. 7, PLDI, Article 148 (jun
2023), 24 pages. https://doi.org/10.1145/3591262
[35] Lasse Nielsen and Fritz Henglein. 2011. Bit-coded Regular Expression
Parsing. In LATA 2011 (LNCS, Vol. 6638), Adrian-Horia Dediu, Shunsuke
Inenaga, and Carlos Martín-Vide (Eds.). Springer, Berlin, Heidelberg,
402–413. https://doi.org/10.1007/978-3-642-21254-3_32
[36] Tobias Nipkow and Dmitriy Traytel. 2014. Unified Decision Procedures
for Regular Expression Equivalence. In Interactive Theorem Proving,
Gerwin Klein and Ruben Gamboa (Eds.). Springer International Publishing, Cham, 450–466.
[37] Oracle. [n. d.]. Java Regex Matching. https://docs.oracle.com/javase/8/
docs/api/java/util/regex/Pattern.html. [Online; accessed Feb 28, 2024].
[38] Rob Pike. 1987. The text editor sam. Software: Practice and Experience
17, 11 (1987), 813–845.
[39] Damien Pous. 2013. Kleene Algebra with Tests and Coq Tools for while
Programs. In Interactive Theorem Proving, Sandrine Blazy, Christine
Paulin-Mohring, and David Pichardie (Eds.). Springer Berlin Heidelberg, Berlin, Heidelberg, 180–196.

[40] RE2 2023. RE2: Google’s regular expression library. https://github.
com/google/re2.
[41] Matthieu Sozeau. 2023. Generalized Rewriting. https://coq.inria.fr/
doc/V8.19.2/refman/addendum/generalized-rewriting.html. [Online;
accessed Aug 7, 2024].
[42] Caleb Stanford, Margus Veanes, and Nikolaj Bjørner. 2021. Symbolic
Boolean derivatives for efficiently solving extended regular expression
constraints. In Proceedings of the 42nd ACM SIGPLAN International
Conference on Programming Language Design and Implementation (Virtual, Canada) (PLDI 2021). Association for Computing Machinery, New
York, NY, USA, 620–635. https://doi.org/10.1145/3453483.3454066
[43] The PCRE2 Developers. 2022. Perl-compatible Regular Expressions
(revised API: PCRE2). https://pcre2project.github.io/pcre2/doc/html/
index.html. [Online; accessed Feb 26, 2023].
[44] Ian Erik Varatalu, Margus Veanes, and Juhan-Peep Ernits. 2023. Derivative Based Extended Regular Expression Matching Supporting
Intersection, Complement and Lookarounds. arXiv:2309.14401 [cs.FL]
[45] Ekaterina Zhuchko, Margus Veanes, and Gabriel Ebner. 2024. Lean
Formalization of Extended Regular Expression Matching with
Lookarounds. In Proceedings of the 13th ACM SIGPLAN International
Conference on Certified Programs and Proofs (London, UK) (CPP 2024).
Association for Computing Machinery, New York, NY, USA, 118–131.
https://doi.org/10.1145/3636501.3636959

Received 2024-09-17; accepted 2024-11-19

213
