---
file: ../pdfs/2023-arxiv-varatalu-derivative-extended-regex-matching.pdf
citation: "Derivative Based Extended Regular Expression Matching Supporting Intersection, Complement and Lookarounds — Varatalu, Veanes, Ernits — arXiv 2309.14401, 2023"
doi: 
source: "https://arxiv.org/pdf/2309.14401"
topics: [counting_and_derivatives, redos_backrefs_verification, semantics_theory_access]
pages: 26
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

arXiv:2309.14401v1 [cs.FL] 25 Sep 2023

Derivative Based Extended Regular Expression Matching
Supporting Intersection, Complement and Lookarounds
IAN ERIK VARATALU, Tallinn University of Technology, Estonia
MARGUS VEANES, Microsoft, USA
JUHAN-PEEP ERNITS, Tallinn University of Technology, Estonia
Regular expressions are widely used in software. Various regular expression engines support different combinations of extensions to classical regular constructs such as Kleene star, concatenation, nondeterministic
choice (union in terms of match semantics). The extensions include e.g. anchors, lookarounds, counters,
backreferences. The properties of combinations of such extensions have been subject of active recent research.
In the current paper we present a symbolic derivatives based approach to finding matches to regular
expressions that, in addition to the classical regular constructs, also support complement, intersection and
lookarounds (both negative and positive lookaheads and lookbacks). The theory of computing symbolic
derivatives and determining nullability given an input string is presented that shows that such a combination
of extensions yields a match semantics that corresponds to an effective Boolean algebra, which in turn opens
up possibilities of applying various Boolean logic rewrite rules to optimize the search for matches.
In addition to the theoretical framework we present an implementation of the combination of extensions to
demonstrate the efficacy of the approach accompanied with practical examples.

1

INTRODUCTION

Regular expressions are supported by standard libraries of all major programming languages and
software development kits. They are essential in string manipulation, membership testing, text
extraction tasks on strings, etc. In recent years symbolic derivatives based regular expression matching has seen rapid development as being performance wise more predictable than backtracking
engines and modern optimized regular expression engines such as e.g. Hyperscan [Wang et al.
2019] and RE2 [Google 2023], that are all, in essence, relying on algorithms by [Glushkov 1961] or
[Thompson 1968].
The extended regular expressions supported by backtracking engines often involve support for
combinations of extensions, e.g. backreferences, lookarounds and balancing groups, yielding a
family of languages that has been shown by [Carle and Narendran 2009] not to be closed under
intersection. Recent work by [Chida and Terauchi 2023] has explored the expressive power of the
combination of regular expressions with backreferences and positive lookaheads and devised a
new flavor of memory automata to express their behavior.
We present a derivative-based symbolic extended regular expression engine, which supports
intersection (&), complement (∼) and lookaround ((?=), (?!), (?<=) and (?<!)) operations, i.e.
both negative and positive lookahead and lookback operations. To remind the reader, by example
of a positive lookahead, the regex a(?=c) will match a in the string “ac” but nothing in “ab”,
i.e. lookarounds do not match text but positions in strings in a more general way than anchors
(e.g. word boundary \b) [Friedl 2006]. We show that the extended engine can be used to solve
problems that are currently unfeasible to solve with existing regular expression engines. We also
show that all context dependent regex anchors can be expressed using lookarounds. We have
implemented the extended engine as a library for the .NET platform, which extends the existing
.NET nonbacktracking engine [Moseley et al. 2023], but without preserving backtracking semantics.

Authors’ addresses: Ian Erik Varatalu, Tallinn University of Technology, Estonia, ian.varatalu@taltech.ee; Margus Veanes,
Microsoft, USA, margus@microsoft.com; Juhan-Peep Ernits, Tallinn University of Technology, Estonia, juhan.ernits@
taltech.ee.

<!-- page 2 -->

We demonstrate the usefulness of the extended engine by examples of precise text extraction in
a way that is to our knowledge not possible with standard regexes or involves a factorial blowup of
the regex pattern. We make the case that intersection and complement are useful tools in concise
definition of regular expressions, showing examples of incremental specification of regexes by use
of complement, and exclusion of unwanted matches by use of negation.
Our work is motivated by the fact that the current state-of-the-art symbolic regex engine [Moseley
et al. 2023] does not support intersection because its semantics is difficult to combine with all the
other features in a meaningful way while maintaining backtracking semantics. It is also a problem
of backwards compatibility to, for example, support & as a new operator.
Contributions
This paper makes the following contributions:
• We build on the .NET 7 nonbacktracking regular expression engine [Moseley et al. 2023] to
develop a theory for symbolic derivatives based regular expression matching that, in addition
to Kleene star, bounded loops (counters), alternation (which in our case is synonymous to
union) and concatenation, supports intersection, complement and both positive and negative
lookaround operations in regexes. The result is a significant increment over [Moseley et al.
2023] in terms of Theorem 1, Theorem 2 and Theorem 3 accompanied by proofs.
• The theoretical results are accompanied by an implementation [Varatalu 2023] of an extended regular expression engine that supports all Boolean operations on regexes, including
intersection and complement using a natural syntax making it convenient to use.
• Due to the match semantics of such a regular expression language corresponding to an
effective Boolean algebra, it is now possible to define and apply numerous Boolean rewrites
and optimizations that yield better runtime performance. It is thus also possible to reason
about the validity of such optimizations in terms of Boolean algebra.
• We show that the extended engine can be used to solve problems that are currently either
impossible or infeasible to solve with standard regular expressions.
• We show that all regex anchors can be expressed using lookarounds as conjectured in
[Moseley et al. 2023]. We prove the conjecture together with providing an implementation
as mentioned above.
Before proceeding to introducing the theory, let us look at some practical examples that illustrate
how the proposed combination extensions can be used to establish the existence of matches but
also locate them in strings.
2

MOTIVATING EXAMPLES

In this section we explain the intuition behind the intersection (conjunction) and complement
(negation) operators in regexes and give some motivating examples of how they can be useful. The
examples are also available in the accompanying web application 1 . The following examples are
written in the syntax of the .NET regex engine, with the addition of &, ∼, and the symbol ⊤, to
denote a predicate that is true on any character. The syntax is explained in more detail in Section 3.
Example 2.1 (Password extraction). Consider the following regex
^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,}$

1 https://ieviev.github.io/sbre/

2

<!-- page 3 -->

that originates from a StackOverflow question 2 asking for a regex for validating a password.
The regex in principle is an intersection of three lookaheads, each of which checks for a certain
condition and a loop that checks for the length of the password.
• (?=.*[a-z]) checks for at least one lowercase letter
• (?=.*[A-Z]) checks for at least one uppercase letter
• (?=.*\d) checks for at least one digit
• [a-zA-Z\d]{8,} checks for at least 8 characters
The regex is a good example of emulating conjunctive conditions using lookaheads for checking
if there exists a match. However, the regex is not suitable for extracting a password from an
arbitrary string, even when removing the anchors. The reason is that the regex engine will try
to match the lookaheads independently, and then backtrack to the beginning of the match to try
the next lookahead. This is an example of a limitation to what can be expressed in traditional
regular expressions. And note that due to the lack of support for lookarounds in RE2 [Cox 2010],
Hyperscan [Wang et al. 2019] and the nonbacktracking engine in .NET to name a few optimized
open source regex libraries, it cannot be expressed concisely using an engine that does not support
lookaheads.
Now consider the following regular expression, composed of intersections which we denote by
the & symbol:
.*[a-z].*&.*[A-Z].*&.*\d.*&[a-zA-Z\d]{8,}
The regular expression has a number of differences from the previous one.
• The proposed regex engine will check for all four conditions at the same time, without any
backtracking.
• Upon meeting one of the conditions, the rest of the alternation is matched with one less
condition to check. For example, after matching the lowercase letter a at the beginning, the
set [a-z] is satisfied, and the regex engine will match the rest of the input with the regex
.*[A-Z].*&.*\d.*&[a-zA-Z\d]{7,}, by effectively removing the first condition from the
regex, as the remaining .* will be subsumed by the other intersections. In Sections 4.2 and
5.1 we show how it is achieved.
• Because all checks are performed together at the same position, intersections allow precise
text extraction in a single pass, which is also highly amenable to parallelization in the
form of vectorization. Such optimizations could be beneficial for tasks such as scanning for
potential credential leaks.
• The pattern does not currently check for the validity of the entire string, but it can be
done by appending the intersection ^.*$ to the regex, which uses lookarounds to constrain
the regex to match the entire line. Similarly, the regex could be constrained to match text
between word borders by appending the intersection \b\S*\b to the regex.
Now consider modifying the regular expression to find potential password substrings while
excluding certain ones, such as those having 2 consecutive digits or the word "password" in it.
Such conditions cannot be checked effectively with positive lookaheads but can be achieved with
negative ones. For example, the following regular expression uses negative lookaheads to match a
password that does not have 2 consecutive digits:
.*[a-z].*&.*[A-Z].*&.*\d.*&(?!\S*\d\d)[a-zA-Z\d]{8,}
An important detail to point out here, is that in such regular expressions, the ranges of the
lookaheads and the loop are not necessarily the same. For example, if there was something like
2 http://stackoverflow.com/questions/19605150/regex-for-password-must-contain-at-least-eight-characters-at-least-one-

number-a

3

<!-- page 4 -->

"@11" on the same line, after the occurrence of the potential match - it would be falsely considered
a non-match, as \S* travels further than [a-zA-Z\d]{8,}, while the loop would stop at @.
A better way of writing the pattern would be to convert the negative lookahead (?!\S*\d\d) to
(?![a-zA-Z\d]*\d\d), that is, adjust the loop in the negative lookahead to the entire following
regular expression body, to guarantee that the constraints apply to the same range. This is where
the use of complement (‘~‘) can help.
.*[a-z].*&.*[A-Z].*&.*\d.*&[a-zA-Z\d]{8,}&~(.*\d\d.*)
The regex above is equivalent to the previous one, but it is constrained to the exact range of the
potential match, as ∼(.*\d\d.*) remains nullable exactly until two sequential digits are found,
after which it turns the entire intersection of regexes into ⊥. The negation operator is explained in
detail in Section 4.2.
Example 2.2 (Incremental specification of regexes through composition). Consider the following
regex:
.*A.*&.*B.*&.*C.*
The regex matches any line that contains the substrings A, B and C in any order.
Now consider creating an equivalent regular expression without using intersections. The result
would be similar to the following:
.*A.*B.*C.*|.*A.*C.*B.*|.*B.*A.*C.*|.*B.*C.*A.*|.*C.*A.*B.*|.*C.*B.*A.*
The regex is equivalent to the previous one, but it is much more verbose, as it requires enumerating
all possible permutations of the substrings as separate alternatives. In such scenarios the use of
intersection (&) becomes very6 helpful. The number of alternations required is equal to the factorial
of the number of substrings.
Matching substrings A,B,C,D,E,F this way would require 720 alternations, while the equivalent
regex with intersections would only require 6 intersections. Note that positive lookaheads can be
used for such examples strictly as long as it is possible to define a clear lookup range for all the
substrings, which is not always the case.
Another important advantage of intersections is that they are not interlinked with each other,
and can be added or removed independently. For example, if we wanted to add a fourth substring D,
we would only need to add one more intersection, instead of adding 18 alternations and modifying
the existing 6. And if we wanted to remove the substring C, we would only need to remove one
intersection, instead of reducing and modifying all alternations down to 4.
Similarly, if we wanted exclude strings containing the substring D, we would only need append
the negated intersection ∼(.*D.*), instead of modifying all existing alternations to exclude the
substring D.
Such incremental aspect can be significant when dealing with large regexes, and can be useful for
tasks such as machine learning based text extraction with regular expressions, where the regex is
incrementally built by adding and removing positive or negative intersection terms. Such approach
also improves the readability of regular expressions, as the regexes are built in a modular way.
Example 2.3 (Matching paragraphs of text containing multiple substrings). Another practical use
case for the new features is matching paragraphs of text that meet certain conditions, as both
intersection and negation introduce new ways of matching ranges of text with complex conditions.
While it is possible to envisage sequential use of e.g. grep for positively and negatively matching
lines of text on the input string when each paragraph is on a separate line, we generalize the
example to paragraphs spanning over multiple lines and paragraphs being separated by double
newlines, as is often done in texts, e.g. in Project Gutenberg books in plain text format. The example
4

<!-- page 5 -->

generalizes to any substring start and end condition and provides a mechanism to also extract the
match in addition to checking for the existence of it.
Note that, for the sake of improved readability, we use the ⊤ predicate to denote the set of all
characters, which can be thought of as a canonical representation of [\s\S] or [\w\W].
For example, consider the following regular expression:
\n\n∼(⊤*\n\n⊤*),
which effectively splits the text into ranges not containing two sequential newlines, as it becomes
nullable with two newlines, and remains nullable until two newlines are found again, after which
it becomes ⊥. As negation does not match at the position of the final newline, an additional \n can
be appended to include that as well.
After defining the previous construct, we can extend it with intersections to match paragraphs
containing certain substrings. For example, the following regex matches paragraphs containing the
substrings "King", "Paris" and "English" in any order:
\n\n∼(⊤*\n\n⊤*)&⊤*King⊤*&⊤*Paris⊤*&⊤*English⊤*
A key advantage of the approach is that it is very easy to infer the starting predicate of such
a regex and skip to it, we have all the necessary contextual information to do so. A regex that
starts with ⊤* essentially means that you can skip taking derivatives until the starting predicate of
the tail of the regex matches. In the case of ⊤*King⊤*, the only valid predicate that changes the
state of this regex is 𝜓𝐾 , after which the regex becomes ing⊤*|⊤*King⊤*, and the next predicate
always changes state, as the alternation ing⊤* either continues matching in the case of 𝜓𝑖 , or turns
to ⊥, making it an "unskippable" state.
Using such information from all intersections, we can skip taking derivatives until we reach
a starting predicate that changes the state of the regex, which allows us to perform a vectorized
string search procedure on any skippable state of the regex. Such starting predicate lookup and
skipping is explained in more detail in Section 5.3.
Another feature, that is exclusively available in the proposed regex engine, is the ability to define
a matching range based on non-matching constraints. For example matching a range of text starting
with "King" and ending with "Paris", that must not contain two sequential digits between each
other can be concisely expressed with the following regex: King∼(⊤*\d\d⊤*)Paris which finds
a match in "The King in Paris", but not "The King 11 Paris". Using a negative lookahead for
regexes such as this, for example King(?!⊤*?\d\d)⊤*?Paris, is very difficult, as the ⊤* inside
the lookahead will leak far past the occurrence of Paris, making it incorrect if any two sequential
digits occur inside the text after the match.
Example 2.4 (Conditional matching). Extending the previous example, let us try to locate a
paragraph containing the word “charity” only in the case the paragraph does not contain the word
“honor”. Such properties can be represented in terms of Boolean combinations of the implication
operation, which we, for now, represent in terms of complement, union and intersection.
Let us first try to find all paragraphs that match the implication charity =⇒ honor. When
writing ~(⊤*\n\n⊤*)&(~(⊤*charity⊤*)|(⊤*honor⊤*)) the result is not yet precise enough,
because the term ~(⊤*\n\n⊤*) stipulates that the match should not contain two consecutive
newlines, but there is no condition that tells that the string has to have double newlines before
and after it. In the Example 2.3 it worked because we did not use negative conditions, i.e. where
some word should not be contained in the match. Here we need to extend the term for matching
paragraphs by appropriate lookarounds as follows:
(?<=\n\n|\A)~(⊤*\n\n⊤*)(?=\n\n|\Z)&(~(⊤*charity⊤*)|(⊤*honor⊤*))
5

<!-- page 6 -->

The result now matches all paragraphs where there is no word “charity” or there exists a word
“honor” (according to the definition of the standard Boolean implication operation). To locate the
paragraph where “charity” is present, but “honor” is not, we need to take the complement of the
implication and write
(?<=\n\n|\A)~(⊤*\n\n⊤*)(?=\n\n|\Z)&~(~(⊤*charity⊤*)|(⊤*honor⊤*))
which in turn can be rewritten by applying de Morgan’s rule into
(?<=\n\n|\A)~(⊤*\n\n⊤*)(?=\n\n|\Z)&(⊤*charity⊤*)&~(⊤*honor⊤*)
The resulting regular expression matches the paragraph that contains the word “charity” but does
not contain “honor”.
It is possible to utilize the If-Then-Else conditionals (?(?=𝑅1 )(𝑅2 )|(𝑅3 )) in traditional regex
engines supporting the construct. The explanation of If-Then-Else is nontrivial [Friedl 2006]: 𝑅1 acts
as a test and if the test yields true then 𝑅2 gets applied, otherwise 𝑅3 . 𝑅1 can either be a reference
to a capturing group and test if the group participated in a match (which is different from the actual
match) or be a lookaround and yield a Boolean result at a particular location. As mentioned before,
there are differences in the extent of the string in which lookarounds get evaluated and the extent
of loops, as was shown in the Example 2.1. As a result, writing a conditional regular expression in
terms of If-Then-Else corresponding to the above requirements is error prone and requires a great
deal of care. We argue that the use of complement and intersection (and consequently a whole
Boolean algebra) together with lookarounds simplifies the task significantly.
We now proceed to establishing the theory for regular expressions extended with lookarounds,
complement and intersection.
3

PRELIMINARIES

Here we introduce the notation and main concepts used in the paper. The general meta-notation,
notation for denoting strings and locations is based on the approach taken in [Moseley et al. 2023].
As a general meta-notation throughout the paper we write lhs def
= rhs to let lhs be equal by
definition to rhs. Let B = {false, true} denote Boolean values. Let Σ be a domain of characters. ⟨𝑥, 𝑦⟩
stands for pairs of elements and let 𝜋1 (⟨𝑥, 𝑦⟩) def
= 𝑥 and 𝜋2 (⟨𝑥, 𝑦⟩) def
= 𝑦.
Strings. Let 𝜖 or "" denote the empty string and let Σ∗ denote the set of all strings over Σ. Let 𝑠 ∈ Σ∗ .
The length of 𝑠 is denoted by |𝑠 |. Individual characters and strings of length 1 are not distinguished.
Let 𝑖 and 𝑙 be nonnegative integers such that 𝑖 + 𝑙 ≤ |𝑠 |. Then 𝑠𝑖,𝑙 denotes the substring of 𝑠 that
starts from index 𝑖 and has length 𝑙, where the first character has index 0. In particular 𝑠𝑖,0 = 𝜖.
For 0 ≤ 𝑖 < |𝑠 | let 𝑠𝑖 def
= 𝑠𝑖,1 . Let also 𝑠 −1 = 𝑠 |𝑠 | def
= 𝜖. E.g., "abcde"1,3 = "bcd" and "abcde"5,0 = 𝜖. 𝑠 𝑟
𝑟
denotes the reverse of 𝑠, so that 𝑠 𝑖 = 𝑠 |𝑠 | −1−𝑖 for 0 ≤ 𝑖 < |𝑠 |.
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
= .
Valid locations in a string 𝑠 can be illustrated by
6

<!-- page 7 -->

𝑠 0 𝑠 ⟨1⟩ 𝑠 1 𝑠 ⟨2⟩ · · · 𝑠 ⟨|𝑠 |−1⟩ 𝑠 |𝑠 |−1 𝑠 ⟨|𝑠 |⟩

𝑠 ⟨0⟩

and should be interpreted as border positions rather than character positions. For example, 𝜖 has
only one valid location 𝜖 ⟨0⟩ that is both initial and final.
Boolean algebras as alphabet theories. The tuple A = (Σ, Ψ, [[_]], ⊥, ⊤, ∨, ∧, ¬) is called an effective
Boolean algebra over Σ where Ψ is a set of predicates that is closed under the Boolean connectives;
[[_]] : Ψ → 2Σ is a denotation function; ⊥, ⊤ ∈ Ψ; [[⊥]] = ∅, [[⊤]] = Σ, and for all 𝜑,𝜓 ∈ Ψ,
[[𝜑 ∨ 𝜓 ]] = [[𝜑]] ∪ [[𝜓 ]] , [[𝜑 ∧ 𝜓 ]] = [[𝜑]] ∩ [[𝜓 ]] , and [[¬𝜑]] = Σ \ [[𝜑]] . Two predicates 𝜙 and 𝜓 are
equivalent when [[𝜙]] = [[𝜓 ]] , denoted by 𝜙 ≡ 𝜓 . If 𝜑 . ⊥ then 𝜑 is satisfiable or SAT(𝜑).
Character classes. In all the examples below we let Σ stand for the standard 16-bit character
set of Unicode3 and use the .NET syntax [Microsoft 2021] of regular expression character classes.
For example, [A-Z] stands for all the Latin capital letters, [0-9] for all the Latin numerals, \d for
all the decimal digits, \w for all the word-letters and dot (.) for all characters besides the newline
character \n. (It is a standard convention that, by default, dot does not match \n.). We also use the ⊤
predicate to denote the set of all characters, which can be thought of as a canonical representation
of [\s\S] (or [\w\W]).
When we need to distinguish the concrete representation of character classes from the corresponding abstract representation of predicates in A we map each character class 𝐶 to the corresponding
predicate 𝜓𝐶 in Ψ. For example 𝜓 [^0-9] ≡ ¬𝜓 [0-9] , 𝜓 [\w-[\d]] ≡ 𝜓 \w ∧ ¬𝜓 \d , and 𝜓 \W ≡ ¬𝜓 \w . Observe
also that [[𝜓 [0-9] ]] ⊊ [[𝜓 \d ]] ⊊ [[𝜓 \w ]] and [[𝜓 \n ]] = {\n} and \n ∉ [[𝜓 \w ]] because \n is not a
word-letter. All predicates in Ψ are represented in a canonical form in the implementation so that if
[[𝜑]] = [[𝜓 ]] then 𝜑 = 𝜓 by reusing the predicate algebra for Unicode Plane 0 that is built-in into
the .NET 7 runtime. In other words, character classes are only used as the concrete notation, while,
e.g., 𝜓 [\s\S] = 𝜓 [\w\W] = ⊤.
4

REGEXES WITH LOOKAROUNDS AND LOCATION DERIVATIVES

Here we formally define regular expressions with lookarounds and loops supporting finite and
infinite bounds. Regexes are defined modulo a character theory A = (Σ, Ψ, [[_]], ⊥, ⊤, ∨, ∧, ¬)
that we illustrate with standard (.NET Regex) character classes in examples while the actual
representation of character classes in Ψ is irrelevant for the purposes here where Σ may even
be infinite. After the definition of regexes, we formally define a framework of derivatives that
leads to the key notion of derivation relation between locations that is used to prove properties in
this framework. We build on and extend definitions from [Moseley et al. 2023] by incorporating
intersection, complement, loops, and lookarounds into a single unified theory of location derivatives
and matching.
4.1

Regexes

The class R of extended regular expressions with lookarounds, or regexes, is here defined by the
following abstract grammar. Let 𝜓 ∈ Ψ, and 0 ≤ 𝑚 ≤ 𝑛 ≠ 0, or 𝑛 = ∞, and let 𝑅 range over R.
𝑅

|

()

(?=𝑅)

|

::= 𝜓

|

𝑅 | 𝑅′

(?<=𝑅)

|
|

𝑅 & 𝑅′
(?!𝑅)

|
|

𝑅 · 𝑅′

|

𝑅{𝑚, 𝑛}

|

~𝑅

|

(?<!𝑅)

where the operators in the first row appear in order of precedence, with union (|) having lowest
and complement (~) having highest precedence. The remaining operators in the first row are the
empty-word regex (()), intersection (&), concatenation (·), and the loop expression 𝑅{𝑚, 𝑛} where
3 Also known as Plane 0 or the Basic Multilingual Plane of Unicode.

7

<!-- page 8 -->

𝑅 is the body, 𝑚 the lower bound, and 𝑛 the upper bound of the loop. If 𝑛 = ∞ then the loop is
infinite else finite. We let 𝑅{0, 0} def
= () for convenience in recursive definitions. We use the common
abbreviations 𝑅* for 𝑅{0, ∞}, 𝑅+ for 𝑅{1, ∞}.
The regex denoting nothing is just the predicate ⊥. Concatenation operator · is often implicit by
using juxtaposition. The expressions in the second row are called lookarounds: (?=𝑅) is lookahead,
(?<=𝑅) is lookback, (?!𝑅) is negative lookahead, and (?<!𝑅) is negative lookback.
The reverse 𝑅𝑟 of 𝑅 ∈ R is defined as follows:

𝜓𝑟
()𝑟
(𝑅 | 𝑆)𝑟
(𝑅 & 𝑆)𝑟
(𝑅 · 𝑆)𝑟
𝑅{𝑚, 𝑛}𝑟
(~𝑅)𝑟

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

(?=𝑅)𝑟
(?<=𝑅)𝑟
(?!𝑅)𝑟
(?<!𝑅)𝑟

𝜓
()
𝑅𝑟 | 𝑆 𝑟
𝑅𝑟 & 𝑆 𝑟
𝑆 𝑟 · 𝑅𝑟
𝑅𝑟{𝑚, 𝑛}
~(𝑅𝑟)

def

= (?<=𝑅𝑟)
= (?=𝑅𝑟)
def
= (?<!𝑅𝑟)
def
= (?!𝑅𝑟)
def

Note that the reverse 𝑅𝑟 of a regex 𝑅 has exactly the same size as 𝑅. The size of a regex, |𝑅|, is
defined recursively as the number of subexpressions, where each predicate 𝜓 ∈ Ψ is considered to
have size one, i.e., the actual representation size of predicates is irrelevant in this context. It also
follows by induction from the definition that (𝑅𝑟)𝑟 = 𝑅.
4.2

Derivatives

In the following definition let 𝑥 = 𝑠 ⟨𝑖⟩ be a valid nonfinal location. For example, if 𝑠 = "ab" then
the valid nonfinal locations in 𝑠 are 𝑠 ⟨0⟩ and 𝑠 ⟨1⟩. In the following let ℓ range over lookarounds.
def
def
def



and let ∞−1
= ∞, 0−1
= 0, and 𝑘 −1
= 𝑘 − 1 for 𝑘 > 0.

𝜕𝑥 (())
𝜕𝑥 (ℓ)

def

𝜕𝑠 ⟨𝑖 ⟩ (𝜓 )

def

𝜕𝑥 (𝑅 | 𝑆)
𝜕𝑥 (𝑅 & 𝑆)
𝜕𝑥 (~𝑅)

def

𝜕𝑥 (𝑅·𝑆)

def

𝜕𝑥 (𝑅{𝑚, 𝑛})

def

=
=

def

=

=
=
def
=
def

=
=

⊥
⊥

(), if 𝑠𝑖 ∈ [[𝜓 ]] ;
⊥, otherwise.
𝜕𝑥 (𝑅) | 𝜕𝑥 (𝑆)
𝜕𝑥 (𝑅) & 𝜕𝑥 (𝑆)
~𝜕
 𝑥 (𝑅)
𝜕𝑥 (𝑅)·𝑆|𝜕𝑥 (𝑆), if Null𝑥 (𝑅);
𝜕
otherwise.
 𝑥 (𝑅)·𝑆, 

𝜕𝑥 (𝑅)·𝑅{𝑚 −1, 𝑛 −1},
if 𝑚=0 or Null ∀ (𝑅)=true or Null𝑥 (𝑅)=false;


𝜕𝑥 (𝑅·𝑅{𝑚 −1, 𝑛 −1}), otherwise.

where Null ∀ (𝑅) def
= ∀𝑥 (Null𝑥 (𝑅) = true) intuitively means that 𝑅 is nullable and nullability does not
depend on lookarounds. Next we define nullability of locations and what it means to find a match
end location from a valid start location 𝑥 in a string 𝑠 by a regex 𝑅 ∈ R. MatchEnd(𝑥, 𝑅) returns the
8

<!-- page 9 -->

latest match end location from a valid 𝑥 or if none exists. Note that max(𝑥, ) = max( , 𝑥) = 𝑥.

𝑥, if Null𝑥 (𝑅);
Null𝑥 (𝑅) def
=
, otherwise.

Null𝑥 (𝑅),
if Final(𝑥);
def
MatchEnd(𝑥, 𝑅) =
max(Null𝑥 (𝑅), MatchEnd(𝑥+1, 𝜕𝑥 (𝑅))), otherwise.
IsMatch(𝑥, 𝑅) def
= MatchEnd(𝑥, 𝑅) ≠
Null𝑥 (()) def
= true
Null𝑥 (𝜓 ) def
= false
Null𝑥 (𝑅 | 𝑅 ′ ) def
= Null𝑥 (𝑅) or Null𝑥 (𝑅 ′ )
def
′
Null𝑥 (𝑅 & 𝑅 ) = Null𝑥 (𝑅) and Null𝑥 (𝑅 ′ )
Null𝑥 (𝑅 · 𝑅 ′ ) def
= Null𝑥 (𝑅) and Null𝑥 (𝑅 ′ )
def
Null𝑥 (𝑅{𝑚, 𝑛}) = 𝑚 = 0 or Null𝑥 (𝑅)
Null𝑥 (~𝑅) def
= not Null𝑥 (𝑅)
Null𝑥 ((?=𝑅)) def
= IsMatch(𝑥, 𝑅)
Null𝑥 ((?<=𝑅)) def
= IsMatch(𝑥 𝑟, 𝑅𝑟)
def
Null𝑥 ((?!𝑅)) = not IsMatch(𝑥, 𝑅)
Null𝑥 ((?<!𝑅)) def
= not IsMatch(𝑥 𝑟, 𝑅𝑟)
Observe that the definition of MatchEnd(𝑥, 𝑅) with 𝑥 = 𝑠 ⟨𝑖⟩ computes the transition from the
source state (regex) 𝑅 to the target state 𝑆 = 𝜕𝑥 (𝑅) for the character 𝑠𝑖 and then continues matching
from location 𝑥 + 1 and state 𝑆. The definition of derivatives and IsMatch are mutually recursive,
through nullability test of lookarounds. The definition of Null𝑥 (ℓ) of lookarounds ℓ is well-defined
because the regex 𝑅 in ℓ is a strictly smaller expression than ℓ and 𝑅𝑟 has the same size as 𝑅.
The definition of IsMatch(𝑥, 𝑅) terminates as soon as Null𝑥 (𝑅) = true. This is an obvious but
important optimization that is absent from the abstract definition above, along many other common
cases, to avoid redundant computation.
4.3

Derivation Relation

𝑅 𝑦 between locations, that is instrumental in reasoning
The key concept is the derivation relation 𝑥 −→
about properties of derivatives. Given a valid location 𝑥, the definition of all matches of 𝑅 from 𝑥,
𝑅 ), is as follows.
(𝑥 −→


∅
if Final(𝑥);
def Null𝑥 (𝑅),
def {𝑥 }, if Null 𝑥 (𝑅);
∅
𝑅
(𝑥 −→) =
where Null𝑥 (𝑅) =
(𝑅)
∅, otherwise.
Null𝑥∅ (𝑅) ∪ (𝑥+1−𝜕𝑥
−−−−→), otherwise.
def
𝑅 𝑦 = 𝑦 ∈ (𝑥 𝑅 )
𝑥 −→
−→
𝑅 𝑦 ⇔ 𝑥 𝑆 𝑦.
Two regexes 𝑅 and 𝑆 are equivalent, denoted by 𝑅 ≡ 𝑆, if, for all locations 𝑥 and 𝑦, 𝑥 −→
−
→
The following is the main derivation theorem that extends [Moseley et al. 2023, Theorem 3.3].
𝑅 ) def
When 𝑥 is a final location we let (𝑥+1−→
= ∅ for convenience in formal arguments. Note also that,
for all 𝑥, (𝑥 −⊥→) = ∅.

Theorem 1 (Derivation). For all regexes and valid locations, let ℓ be a lookaround and 𝜓 ∈ Ψ:
(1) 𝑥 −()
−→ 𝑦 ⇔ 𝑥 = 𝑦;
ℓ
(2) 𝑥 −
→ 𝑦 ⇔ Null 𝑥 (ℓ) and 𝑥 = 𝑦;
(3) 𝑠 ⟨𝑖⟩ −𝜓→ 𝑦 ⇔ 𝑠𝑖 ∈ [[𝜓 ]] and 𝑦 = 𝑠 ⟨𝑖 + 1⟩;
𝑆 𝑦 ⇔ (𝑥 𝑅 𝑦 or 𝑥 𝑆 𝑦);
(4) 𝑥 −𝑅−−|→
−→
−
→
𝑆 𝑦 ⇔ (𝑥 𝑅 𝑦 and 𝑥 𝑆 𝑦);
(5) 𝑥 −𝑅−−&→
−→
−
→
𝑅 𝑦 ⇔ not (𝑥 𝑅 𝑦);
(6) 𝑥 −~−→
−→
9

<!-- page 10 -->

𝑅
𝑆
(7) 𝑥 −𝑅·𝑆
−−→ 𝑦 ⇔ ∃𝑧 (𝑥 −→ 𝑧 −
→ 𝑦);
(8) ∀𝑚 > 0 : 𝑅{𝑚, 𝑛} ≡ 𝑅·𝑅{𝑚 − 1, 𝑛 − 1} ≡ 𝑅{𝑚 − 1, 𝑛 − 1}·𝑅;
(9) 𝑅{0, 𝑛} ≡ 𝑅{1, 𝑛}|();

Proof. 1(1) follows from Null𝑥 (()) = true, 𝜕𝑥 (()) = ⊥ and (𝑥+1−⊥→) = ∅.
∅
ℓ
ℓ
⊥
Proof of 1(2). We have that 𝑥 −
→ 𝑦 iff 𝑦 ∈ (𝑥 −
→) iff 𝑦 ∈ Null𝑥 (ℓ) because 𝜕𝑥 (ℓ) = ⊥ and (𝑥+1 −→) = ∅.
∅
∅
We also have that if Null𝑥 (ℓ) then Null𝑥 (ℓ) = {𝑥 } else Null𝑥 (ℓ) = ∅.
(𝜓 )
Proof of 1(3). Here 𝑥 = 𝑠 ⟨𝑖⟩ is nonfinal. It follows that 𝑠 ⟨𝑖⟩ −𝜓→ 𝑦 iff 𝑦 ∈ (𝑥+1−𝜕𝑥
−−−−→) iff 𝑠𝑖 ∈ [[𝜓 ]]
∅
and 𝑦 ∈ Null𝑥+1 (()) iff 𝑠𝑖 ∈ [[𝜓 ]] and 𝑦 = 𝑥+1 = 𝑠 ⟨𝑖+1⟩.

Proof of 1(4). By induction over 𝜋 2 (𝑦) − 𝜋2 (𝑥) with 𝑦 fixed. If 𝑥 = 𝑦 then
(𝑥=𝑦)

(𝑥=𝑦)

| 𝑆 𝑦 ⇔ Null (𝑅 | 𝑆) ⇔ Null (𝑅) or Null (𝑆) ⇔ 𝑥 𝑅 𝑦 or 𝑥 𝑆 𝑦
𝑥 −𝑅−−−
→
−→
−
→
𝑥
𝑥
𝑥

If 𝑥 < 𝑦 then
𝑆 𝑦 ⇔ 𝑥+1 𝜕𝑥 (𝑅 |𝑆 ) 𝑦
𝑥 −𝑅−−|→
−−−−−−−→

(𝑅) |𝜕𝑥 (𝑆 ) 𝑦
⇔ 𝑥+1 −𝜕𝑥
−−−−−−−−−−
→
ih

(𝑅)
𝜕 (𝑆 ) 𝑦 ⇔ 𝑥 𝑅 𝑦 or 𝑥 𝑆 𝑦
⇔ 𝑥+1 −𝜕𝑥
−−−−→ 𝑦 or 𝑥+1 − 𝑥
−−−−→
−→
−
→

Proof of 1(5). By induction over 𝜋2 (𝑦) − 𝜋2 (𝑥) with 𝑦 fixed. For the base case 𝑥 = 𝑦 we get that
(𝑥=𝑦)

(𝑥=𝑦)

& 𝑆 𝑦 ⇔ Null (𝑅 & 𝑆) ⇔ Null (𝑅) and Null (𝑆) ⇔ 𝑥 𝑅 𝑦 and 𝑥 𝑆 𝑦
𝑥 −𝑅−−−
−→
−
→
→
𝑥
𝑥
𝑥

For the induction case 𝑥 < 𝑦 we get that
(𝑥 <𝑦)

& 𝑆 𝑦 ⇔ 𝑥+1 𝜕𝑥 (𝑅 & 𝑆 ) 𝑦
𝑥 −𝑅−−−
−−−−−−−−→
→

) 𝑦
𝑥 (𝑅) & 𝜕𝑥 (𝑆→
⇔ 𝑥+1 −𝜕−−−−−−−−−−−
(𝑥 <𝑦)

(ih)

𝜕 (𝑆 ) 𝑦 ⇔ 𝑥 𝑅 𝑦 and 𝑥 𝑆 𝑦
(𝑅)
⇔ 𝑥+1 −𝜕𝑥
−→
−
→
−−−−→
−−−−→ 𝑦 and 𝑥+1 − 𝑥

Proof of 1(6). By induction over 𝜋2 (𝑦) − 𝜋2 (𝑥). For the base case 𝑥 = 𝑦 we get that
(𝑥=𝑦)

(𝑥=𝑦)

𝑅 𝑦 ⇔ Null (~𝑅) ⇔ not(Null (𝑅)) ⇔ not(𝑥 𝑅 𝑦)
𝑥 −~−→
−→
𝑥
𝑥

For the induction case 𝑥 < 𝑦 we get that
(𝑥 <𝑦)

(ih)

(𝑥 <𝑦)

𝑅 𝑦 ⇔ 𝑥 + 1 𝜕𝑥 ( ~𝑅) 𝑦 ⇔ 𝑥 + 1 ~𝜕𝑥 (𝑅) 𝑦 ⇔ not(𝑥 + 1 𝜕𝑥 (𝑅) 𝑦) ⇔ not(𝑥 𝑅 𝑦)
𝑥 −~−→
−−−−−−→
−−−−−−→
−−−−−→
−→

Proof of 1(7). by induction over 𝜋2 (𝑦) − 𝜋2 (𝑥) with 𝑦 fixed. If 𝑥 = 𝑦 then
∅
∅
∅
𝑅
𝑆
𝑅
𝑆
𝑥 −𝑅·𝑆
−−→ 𝑦 ⇔ 𝑦 ∈ Null𝑥 (𝑅·𝑆) ⇔ (𝑦 ∈ Null𝑥 (𝑅) and 𝑦 ∈ Null𝑥 (𝑆)) ⇔ (𝑥 −→ 𝑦 −
→ 𝑦) ⇔ ∃𝑧 (𝑥 −→ 𝑧 −
→ 𝑦)
𝑅 𝑧 𝑆 𝑥 implies that 𝑥 ≤ 𝑧 ≤ 𝑥. If 𝑥 < 𝑦 then,
Observe that 𝑧 in the last ‘⇔’ must be 𝑥 because 𝑥 −→
−
→
by case analysis over Null𝑥 (𝑅), if Null𝑥 (𝑅) = false then
ih

𝜕 (𝑅·𝑆 ) 𝑦 ⇔ 𝑥+1 𝜕𝑥 (𝑅) ·𝑆 𝑦 ⇔ ∃𝑧 (𝑥+1 𝜕𝑥 (𝑅) 𝑧 𝑆 𝑦) ⇔ ∃𝑧 (𝑥 𝑅 𝑧 𝑆 𝑦)
𝑥 −𝑅·𝑆
−−→ 𝑦 ⇔ 𝑥+1 − 𝑥
−−−−−−→
−−−−−−−→
−−−−−→ −
→
−→ −
→

In the last ‘⇔’ above Null𝑥 (𝑅) = false so 𝑥+1 ≤ 𝑧. If Null𝑥 (𝑅) = true then
𝑥 −𝑅·𝑆
−−→ 𝑦

⇔

(𝑅) ·𝑆 |𝜕𝑥 (𝑆 ) 𝑦
𝑥+1 −𝜕𝑥
−−−−−−−−−−−−
→

by 1(4)

⇔

𝜕𝑥 (𝑆 ) 𝑦
𝑥 (𝑅) ·𝑆
𝑥+1 −𝜕−−−−−−
→ 𝑦 or 𝑥+1 −−−−−
→

ih

𝑆
𝜕𝑥 (𝑆 ) 𝑦
𝑥 (𝑅)
∃𝑧 (𝑥+1 −𝜕−−−−
→𝑧 −
→ 𝑦) or 𝑥+1 −−−−−
→

⇔
(𝑥 <𝑦)

⇔
⇔
Null𝑥 (𝑅)=true

⇔

(𝑅)
𝑆
𝑆
∃𝑧 (𝑥+1 −𝜕𝑥
−−−−→ 𝑧 −
→ 𝑦) or 𝑥 −
→𝑦
∃𝑧>𝑥 (𝑥 −𝑅→ 𝑧 −𝑆→ 𝑦) or 𝑥 −𝑆→ 𝑦
𝑅 𝑧 𝑆 𝑦)
∃𝑧 (𝑥 −→
−
→

10

<!-- page 11 -->

Proof of 1(8). Let 𝐿 = 𝑅{𝑚, 𝑛} where 𝑚 > 0. We prove the statement by proving (for all 𝑚 and 𝑛)
𝐿 𝑦 ⇔ 𝑥 𝑅· (𝐿 − 1) 𝑦
𝑥 −→
−−−−−−−−→

The statement holds trivially when 𝑚 = 𝑛 = 1. Assume 𝑛 > 1.
We prove the statement by induction over 𝜋2 (𝑦) − 𝜋 2 (𝑥). In each induction step we also get, by
using the IH, that 𝑅(𝐿 − 1) ≡ 𝑅((𝐿 − 2)𝑅) ≡ (𝑅(𝐿 − 2))𝑅 ≡ (𝐿 − 1)𝑅, where (𝐿 − 2) is well-defined
because 𝑛 > 1.
The case Null ∀ (𝑅) = false but Null𝑥 (𝑅) = true then 𝜕𝑥 (𝐿) = 𝜕𝑥 (𝑅·(𝐿 − 1)) by definition. If
Null𝑥 (𝑅) = false then 𝜕𝑥 (𝐿) = 𝜕𝑥 (𝑅)·(𝐿 − 1) but then also 𝜕𝑥 (𝑅·(𝐿 − 1)) = 𝜕𝑥 (𝑅)·(𝐿 − 1). So, in
either case it follows that 𝐿 ≡ 𝑅·(𝐿 − 1) without induction.
The remaining case is Null ∀ (𝑅) = true. The base case of 𝑥 = 𝑦 follows directly because 𝑥 = 𝑦 iff
Null𝑥 (𝑅) = true iff Null𝑥 (𝐿) iff Null𝑥 (𝑅·(𝐿 − 1)).
For the induction case Let 𝑥 < 𝑦. If 𝑚 > 1 then let 𝐿 ′ = (𝐿 − 1) else if 𝑚 = 1, using that
𝑅{0, 𝑘} ≡ 𝑅{1, 𝑘} when Null ∀ (𝑅) = true, let 𝐿 ′ = 𝑅{1, 𝑛 − 1} in order to maintain that the lower
bound remains positive. Observe that the induction is over 𝜋2 (𝑦) − 𝜋2 (𝑥) and in the induction steps
′
there exists 𝑧 ≥ 𝑥 + 1 such that 𝑧 −𝐿−→ 𝑦 where 𝜋2 (𝑦) − 𝜋 2 (𝑧) < 𝜋2 (𝑦) − 𝜋2 (𝑥).
(𝐿 − 1)
𝑥 −𝑅·
−−−−−−−→ 𝑦

⇔
⇔
⇔

′

) 𝑦
𝑥 (𝑅·𝐿 →
𝑥+1 −𝜕−−−−−−−
𝜕𝑥 (𝑅) ·𝐿 ′ | 𝜕𝑥 (𝐿 ′ ) 𝑦
𝑥+1 −−−−−−−−−−−−−−−
→
′
𝜕𝑥 (𝐿 ′ ) 𝑦
𝑥 (𝑅) ·𝐿→ 𝑦 or 𝑥+1 −
𝑥+1 −𝜕−−−−−−−
−−−−−→

(ih)

⇔
⇔
Null ∀ (𝑅)

⇔
⇔

′

(𝑅) · (𝐿 − 2) ·𝑅
𝜕 (𝐿 ) 𝑦
𝑥+1 −𝜕𝑥
−−−−−−−−−−−−−→ 𝑦 or 𝑥+1 − 𝑥
−−−−−→
′
′
𝜕𝑥 (𝐿 ) 𝑦
𝑥 (𝐿 ) ·𝑅
𝑥+1 −𝜕−−−−−−−
𝑦
or
𝑥+1
→
−−−−−−→
′

𝑥 (𝐿 ) ·𝑅
𝑥+1 −𝜕−−−−−−−
→𝑦
𝜕
𝑥 (𝑅) · (𝐿 − 2) ·𝑅
𝑥+1 −−−−−−−−−−−−−−
→𝑦

(ih)

⇔
⇔

′

𝜕𝑥 (𝑅) · (𝐿 ) 𝑦
𝑥+1 −−−−−−−−−
→
𝐿
𝑥 −→ 𝑦

Proof of 1(9). for the case of Null ∀ (𝑅) = true is similar to the proof of 1(8) and observe that in
this case 𝑅 ≡ 𝑅|(). Assume Null ∀ (𝑅) = false. Then for Null𝑥 (𝑅) = true or Null𝑥 (𝑅) = false the
proof follows directly because in either case 𝜕𝑥 (𝑅{0, 𝑛}) = 𝜕𝑥 (𝑅·𝑅{0, 𝑛 − 1}):
{0, 𝑛 } 𝑦 ⇔ 𝑥 + 1 𝜕𝑥 (𝑅·𝑅 {0, 𝑛 − 1} ) 𝑦 or 𝑥 () 𝑦 ⇔ 𝑥 𝑅·𝑅 {0, 𝑛 − 1} | () 𝑦
𝑥 −𝑅−−−−−
→
−−−−−−−−−−−−−−−→
−−→
−−−−−−−−−−−−−−−→

where 𝑅·𝑅{0, 𝑛 − 1} ≡ 𝑅{1, 𝑛} by 1(8). The theorem follows by the induction principle over long
distances.
□
It is possible to prove Theorem 1 by induction over 𝑅 but the proof becomes much more involved,
while using induction over location distances takes full advantage of the definition of location
derivatives. We get the following key characterization of R as a corollary of Theorem 1. Let
U
⟨𝑥, 𝑦⟩ |= 𝑅
M (𝑅)

= {⟨𝑠 ⟨𝑖⟩, 𝑠 ⟨𝑗⟩⟩ | 𝑠 ∈ Σ∗, 0 ≤ 𝑖 ≤ 𝑗 ≤ |𝑠 |}
𝑅 𝑦 for ⟨𝑥, 𝑦⟩ ∈ U
= 𝑥 −→
def
= {𝑀 ∈ U | 𝑀 |= 𝑅}
def

def

We say that 𝑀 ∈ U is a match of 𝑅 if 𝑀 |= 𝑅, and M (𝑅) is called the match set or match sematics
of 𝑅. Observe that 𝑅 ≡ 𝑆 ⇔ M (𝑅) = M (𝑆).
Corollary 1. 𝔐 = (U, R, M, ⊥, ⊤*, |, &, ~) is an effective Boolean algebra over U.
11

<!-- page 12 -->

Proof. Recall that complement of 𝑋 ⊆ U in 𝔐 is ∁(𝑋 ) def
= U \ 𝑋 . It holds that M (⊥) = ∅ and
M (⊤*) = U. Theorem 1(4) implies that M (𝑅 | 𝑆) = M (𝑅) ∪ M (𝑆). Theorem 1(5) implies that
M (𝑅 & 𝑆) = M (𝑅) ∩ M (𝑆). Theorem 1(6) implies that M (~𝑅) = ∁(M (𝑅)).
□
Corollary 1 is a powerful tool that allows us to apply laws of distributivity and de Morgan’s
laws in order to rewrite regexes into various normal forms and to simplify regexes based on other
general laws of Boolean algebras as well as specific laws of 𝔐 in combination with the laws of
Kleene-star and concatenation. For example, it follows from Theorem 1 that ~⊥ ≡ ⊤*, ~() ≡ ⊤+,
⊤* & 𝑅 ≡ 𝑅, and thus for example that 𝜕𝑥 (~() & 𝑅) ≡ 𝜕𝑥 (⊤+) & 𝜕𝑥 (𝑅) = ⊤* & 𝜕𝑥 (𝑅) ≡ 𝜕𝑥 (𝑅).
4.4

Reversal Theorem

Reversal of regexes plays a key role in the top-level matching algorithm, where it is used to locate
the start location of a match backwards from the end location (provided that the end location exists).
Here our focus is on reversal itself and we extend [Moseley et al. 2023, Theorem 3.8]. We make use
of the following basic semantic fact of lookbacks.
𝑅 𝑥.
Lemma 1. Let 𝑅 ∈ R and 𝑥 be a valid location. Then Null𝑥 ((?<=𝑅)) ⇔ ∃𝑧 : 𝑧 −→

Theorem 2 (Reversal). For all 𝑅 ∈ R and valid locations 𝑥 and 𝑦:
(1) Null𝑥 (𝑅) ⇔ Null𝑥 𝑟 (𝑅𝑟)
𝑅 𝑦 ⇔ 𝑦𝑟 𝑅𝑟 𝑥 𝑟.
(2) 𝑥 −→
−−→
Proof. The proof uses Theorem 1 and is by induction over (|𝑅|, 𝜋2 (𝑦) − 𝜋2 (𝑥)) where (|𝑆 |, 𝑑) <
(|𝑅|, 𝑒) if either |𝑆 | < |𝑅| or |𝑆 | = |𝑅| and 𝑑 < 𝑒. For loops |𝑋 {𝑚, 𝑛}| < |𝑋 {𝑚 ′, 𝑛 ′ }| when either
𝑚 < 𝑚 ′ or else 𝑚 = 𝑚 ′ and 𝑛 < 𝑛 ′ . Induction uses 𝜋 2 (𝑦) − 𝜋2 (𝑥) only when 𝑅 = 𝑋 *. (Recall also
that 𝑅{0, 0} def
= ().) Let 𝑥 = 𝑠 ⟨𝑖⟩ be a valid location.
The proof of (1) follows from (2) in all induction cases except when 𝑅 is a lookaround where (1)
is proved by using the IH of (2).
Base case 𝑅 = (). Follows from ()𝑟 = ().
𝑟

Base case 𝑅 = 𝜓 . Here 𝑥 is nonfinal so 𝑥 + 1 is valid. We get that 𝑥 −𝜓→ 𝑦 ⇔ 𝑦𝑟 −𝜓−→ 𝑥 𝑟 by Thm 1(3)
because 𝑥 + 1 = 𝑦 iff 𝑦𝑟 + 1 = 𝑥 𝑟 and 𝜓 𝑟 = 𝜓 , where 𝑦𝑟 = 𝑠 ⟨𝑖 + 1⟩𝑟 = 𝑠 𝑟⟨|𝑠 | − 𝑖 − 1⟩ and 𝑠 𝑟|𝑠 | −𝑖 −1 = 𝑠𝑖 , so
𝑠𝑖 ∈ [[𝜓 ]] iff 𝑠 𝑟|𝑠 | −𝑖 −1 ∈ [[𝜓 𝑟]] . Note also that Null𝑥 (𝜓 ) = Null𝑥 𝑟 (𝜓 𝑟) = Null𝑥 𝑟 (𝜓 ) = false.
Induction case 𝑅 is a lookback. Let 𝑅 = (?<=𝑆).
(Lma 1)

(ih)
𝑟

𝑆 𝑥 ⇔ ∃𝑧 : 𝑥 𝑟 𝑆 𝑧𝑟 ⇔ IsMatch(𝑥 𝑟, 𝑆 𝑟) ⇔ Null 𝑟 ((?=𝑆 𝑟)) ⇔ Null 𝑟 (𝑅𝑟)
Null𝑥 (𝑅) ⇔ ∃𝑧 : 𝑧 −→
−−→
𝑥
𝑥
𝑟

𝑅 𝑦 ⇔ 𝑦𝑟 𝑅 𝑥 𝑟 because 𝑥 𝑅 𝑦 ⇔ (Null (𝑅) and 𝑥 = 𝑦). The proof of the
It follows also that 𝑥 −→
−−→
−→
𝑥
negative lookback is similar because Null𝑥 ((?<!𝑆)) ⇔ not Null𝑥 ((?<=𝑆)).

Induction case 𝑅 is a lookahead. Let 𝑅 = (?=𝑆).
(Lma 1)

(ih)
𝑟

𝑆 𝑧 ⇔ ∃𝑧 : 𝑧𝑟 𝑆 𝑥 𝑟 ⇔ Null 𝑟 ((?<=𝑆 𝑟)) ⇔ Null 𝑟 (𝑅𝑟)
Null𝑥 (𝑅) ⇔ IsMatch(𝑥, 𝑆) ⇔ ∃𝑧 : 𝑥 −→
−−→
𝑥
𝑥
𝑟

𝑅 𝑦 ⇔ 𝑦𝑟 𝑅 𝑥 𝑟. The proof of the negative lookahead is similar because
It follows also that 𝑥 −→
−−→
Null𝑥 ((?!𝑆)) ⇔ not Null𝑥 ((?=𝑆)).

Induction case 𝑅 = 𝑋|𝑌 . Then
Thm 1(4)
𝑅 𝑦
𝑥 −→

⇔

(ih)

Thm 1(4)
𝑟

𝑟

𝑥 −𝑋→ 𝑦 or 𝑥 −𝑌→ 𝑦 ⇔ 𝑦𝑟 −𝑋−→ 𝑥 𝑟 or 𝑦𝑟 −𝑌−→ 𝑥 𝑟
12

⇔

𝑟

𝑟

𝑟

|𝑌 𝑥 𝑟 ⇔ 𝑦 𝑟 𝑅 𝑥 𝑟
𝑦𝑟 −𝑋−−−−
→
−−→

<!-- page 13 -->

Table 1. Standard anchors in regexes and their definitions by lookarounds relative to a valid location 𝑠 ⟨𝑖⟩.
ï

Name
\A start
\z end
^ start-of-line
$
end-of-line
\Z end-or-last-$
\b word-border
\B non-word-border

Definition
(?<!⊤)
(?!⊤)
(\A|(?<=\n))
(\z|(?=\n))
(\z|(?=\n·\z))
(?<=\w)·(?!\w)|(?<!\w)·(?=\w)
(?<=\w)·(?=\w)|(?<!\w)·(?!\w)

Effective meaning
initial location of input (𝑖 = 0)
final location of input (𝑖 = |𝑠 |)
𝑖 = 0 or 𝑠𝑖 −1 = \n
𝑖 = |𝑠 | or 𝑠𝑖 = \n
𝑖 = |𝑠 | or 𝑖 = |𝑠 |−1 and 𝑠𝑖 = \n
𝑠𝑖 −1 ∈ [[𝜓 \w ]] ⇔ 𝑠𝑖 ∉ [[𝜓 \w ]]
𝑠𝑖 −1 ∈ [[𝜓 \w ]] ⇔ 𝑠𝑖 ∈ [[𝜓 \w ]]

Induction case 𝑅 = 𝑋 &𝑌 . Then
Thm 1(5)
𝑅 𝑦
𝑥 −→

(ih)

Thm 1(5)
𝑟

𝑟

𝑥 −𝑋→ 𝑦 and 𝑥 −𝑌→ 𝑦 ⇔ 𝑦𝑟 −𝑋−→ 𝑥 𝑟 and 𝑦𝑟 −𝑌−→ 𝑥 𝑟

⇔

𝑟

𝑟

𝑟

& 𝑌 𝑥 𝑟 ⇔ 𝑦𝑟 𝑅 𝑥 𝑟
𝑦𝑟 −𝑋−−−−−
→
−−→

⇔

Induction case 𝑅 = ~𝑋 . Then
Thm 1(6)

(ih)

⇔

not(𝑥 −𝑋→ 𝑦) ⇔ not(𝑦𝑟 −𝑋−→ 𝑥 𝑟)

𝑅 𝑦
𝑥 −→

Thm 1(6)
𝑟

⇔

(𝑋 ) 𝑥 𝑟 ⇔ 𝑦𝑟 𝑅 𝑥 𝑟
𝑦𝑟 −~−−−−
→
−−→
𝑟

𝑟

Induction case 𝑅 = 𝑋 ·𝑌 . Then
Thm 1(7)
𝑅 𝑦
𝑥 −→

⇔

(ih)

Thm 1(7)
𝑟

𝑟

∃𝑧 (𝑥 −𝑋→ 𝑧 −𝑌→ 𝑦) ⇔ ∃𝑧 (𝑦𝑟 −𝑌−→ 𝑧𝑟 −𝑋−→ 𝑥 𝑟)

⇔

·𝑋 𝑥 𝑟 ⇔ 𝑦𝑟 𝑅 𝑥 𝑟
𝑦𝑟 −𝑌−−−−
→
−−→
𝑟

𝑟

𝑟

Induction case 𝑅 = 𝑋 {𝑚, 𝑛} (𝑚 > 0). Then
Thm 1(7,8)
𝑅 𝑦
𝑥 −→

⇔

(ih)

Thm 1(7,8)

− 1 𝑦) ⇔ ∃𝑧 (𝑦𝑟 𝑅 − 1 𝑧𝑟 𝑋 𝑥 𝑟)
∃𝑧 (𝑥 −𝑋→ 𝑧 −𝑅−−−
→
−−−−−→
−−→
𝑟

𝑟

⇔

𝑟

𝑦𝑟 −𝑅−→ 𝑥 𝑟

Induction case 𝑅 = 𝑋 {0, 𝑛}. Then, by using Thm 1(8,9,7,2),
𝑅 𝑦
𝑥 −→

⇔

− 1 𝑦) or 𝑥 () 𝑦
∃𝑧 (𝑥 −𝑋→ 𝑧 −𝑅−−−
→
−−→

(ih)
− 1 𝑧𝑟 𝑋 𝑥 𝑟) or 𝑦𝑟 () 𝑥 𝑟
⇔ ∃𝑧 (𝑦𝑟 −𝑅−−−−
→
−−→
−−→
𝑟 − 1) ·𝑋 𝑟 | ()
𝑟
(𝑅
⇔ 𝑦 −−−−−−−−−−−−−→ 𝑥 𝑟
𝑟{1, 𝑛 } | ()
𝑟
⇔ 𝑦𝑟 −𝑋−−−−−−−−−−
→𝑥
𝑟
⇔ 𝑦𝑟 −𝑅−→ 𝑥 𝑟
𝑟

𝑟

where 𝜋 2 (𝑦) − 𝜋2 (𝑧) < 𝜋 2 (𝑦) − 𝜋2 (𝑥) when Null𝑥 (𝑋 ) = false and 𝑛 = ∞, so the IH applies. If 𝑅 = 𝑋 *
and Null𝑥 (𝑋 ) = true then the case 𝑥 = 𝑦 is immediate. Otherwise 𝑥 < 𝑦 and we may assume that
any “stuttering” steps 𝑥 −𝑋→ 𝑥 are omitted from the derivation because they do not contribute to
completing the derivation that is finite and bounded by 𝜋 2 (𝑦) − 𝜋 2 (𝑥). Then the IH applies because
then 𝑧 ≥ 𝑥 + 1.
□
4.5

Anchors as Lookarounds

All anchors can be defined by lookarounds as was conjectured in [Moseley et al. 2023, Section 3.7].
In our implementation all the standard anchors are defined by using lookarounds. Many other
custom anchors can be defined with lookarounds, e.g., to support different line separators besides
\n such as \r\n for different file formats, or to support anchors for detecting enclosing parentheses.
Table 1 gives a summary of all anchors typically supported by regular expression engines [Friedl
2006] represented in terms of lookarounds. Recall that 𝑠 −1 = 𝑠 |𝑠 | = 𝜖 and thus, for all 𝜓 ∈ Ψ,
𝑠 −1 = 𝑠 |𝑠 | ∉ [[𝜓 ]] . Therefore, e.g., the definition of \B as (?<=\w)(?=\w)|(?<=\W)(?=\W) would be
incorrect where [[𝜓 \W ]] = [[¬𝜓 \w ]] , because for example \B is nullable in all locations of "#" instead
of in no locations if one would use (?<=\w)(?=\w)|(?<=\W)(?=\W).
13

<!-- page 14 -->

In other words, (?!\w) . (?=\W). In general, negative lookarounds of predicates cannot be
converted into positive lookarounds by delegating negation to the underlying character algebra A
by using ¬ of A.
It is also important to note that complement (~) of positive lookahead does not correspond to a
negative lookahead (and vice versa). Below we show that ~\b . \B, i.e. they are not complements
of each other in match semantics. Thus, negations of combinations of lookarounds must be used
with caution as their semantics needs careful analysis. In fact, ~\b ≡ \B|⊤+, as shown below.
While it follows from definitions, for all valid locations 𝑥, that Null𝑥 ((?!𝑅)) ⇔ not Null𝑥 ((?=𝑅))
their derivatives are fundamentally different: they are complements of each other. In particular,
𝜕𝑥 ((?!𝑅))
𝜕𝑥 (~(?=𝑅))

= ⊥
= ~𝜕𝑥 ((?=𝑅)) = ~⊥ ≡ ⊤*

The same applies to lookbacks. The derivative of a lookaround is always ⊥ because lookarounds
are context conditions for locations with no forward progress on their own, and essentially operate as blocking conditions inside concatenations. It follows from the definition of derivatives of
concatenations that, if ℓ is a lookaround then

𝜕 (𝑅), if Null𝑥 (ℓ);
𝜕𝑥 (ℓ·𝑅) ≡ 𝑥
⊥,
otherwise.
which is how derivatives are implemented for this case, i.e., the simplification rewrite rules are
directly embedded into the abstract definition itself, rather than being applied afterwards in a
separate step. The same applies to all similar situations involving simplification rules for example
for (), ⊥, and ⊤*, in intersections, unions, and concatenations. Many of those rules are mandatory
in order to maintain finiteness of the state space of regexes that arise during matching, and all
rewrite rules are applied as early as possible.
In order to understand the interaction of anchors with newly added extensions better, let us
take the word-border anchor \b and derive what the complement of it (~\b) would look like.
Here we make use of de Morgan’s laws via Corollary 1 and the property that concatenation of
two lookarounds has the same semantics as the intersection (conjunction) of those lookarounds.
Moreover, to simplify, we use laws of distributivity and the properties that
~(?!𝑅) & ~(?=𝑅)
~(?<!𝑅) & ~(?<=𝑅)

≡
≡

⊤+
⊤+

where 𝜕𝑥 (~ℓ & ~ℓ ′ ) ≡ ⊤* and the combined nullability conditions become trivially false, e.g.,
Null𝑥 (~(?!𝑅) & ~(?=𝑅)) ⇔ Null𝑥 ((?=𝑅)) and not Null𝑥 ((?=𝑅))
Then
~\b ≡ ~((?<=\w)&(?!\w)|(?<!\w)&(?=\w))
≡ (~(?<=\w)|~(?!\w))&(~(?<!\w)|~(?=\w))
≡ ~(?<=\w)&~(?<!\w) | ~(?<=\w)&~(?=\w) | ~(?!\w)&~(?<!\w) | ~(?!\w)&~(?=\w)
≡ ~(?<=\w)&~(?=\w) | ~(?!\w)&~(?<!\w) | ⊤+
≡ \B | ⊤+
where the last equivalence holds because, for any valid location 𝑥,
Null𝑥 (~(?<=\w)&~(?=\w) | ~(?!\w)&~(?<!\w)) ⇔ Null𝑥 ((?<!\w)&(?!\w) | (?=\w)&(?<=\w))
that is equivalent to Null𝑥 (\B). One can also show that ~\B ≡ \b | ⊤+. Observe therefore that
~(\B | ⊤+) ≡ ~\B & ~(⊤+) ≡ (\b | ⊤+) & () ≡ \b & () ≡ \b
14

<!-- page 15 -->

A natural question that arises, that we are investigating, is if there exists a general way to encode
any negative lookaround (?!𝑅) (or (?<!𝑅)) by an equivalent positive lookaround (?=𝑅 ′ ) (or
(?<=𝑅 ′ )), by converting 𝑅 ∈ R into a regex 𝑅 ′ ∈ R by using ~ in some manner. A possible
algorithmic advantage is that this transformation may enable further transformations. For example,
(?!𝑅)·(?=𝑆) could then be transformed into (?=𝑅 ′ ·⊤* & 𝑆·⊤*).
The anchor denoting the end of previous match \G [Friedl 2006] is a non-traditional anchor as it
uses metadata instead of regular expressions. While it can be supported by adding implementation
details, we have decided to omit it as we have not observed it being pervasively used in real life
applications. The anchor \a in [Moseley et al. 2023] that is needed internally for reversal, can be
defined here as (\Z)𝑟; \a is not supported in the concrete syntax of .NET regular expressions.
4.6

Top-Level Match Algorithm

The top-level match algorithm llMatch(𝑠, 𝑅) takes a string 𝑠 ∈ Σ∗ and a regex 𝑅 ∈ R, and either
returns if M (𝑅) = ∅, or else returns a match ⟨𝑠 ⟨𝑖⟩, 𝑠 ⟨𝑗⟩⟩ ∈ M (𝑅) such that 𝑖 is minimal (leftmost)
and then 𝑗 is maximal for the given 𝑖 (in particular all loops are eager). Semantically the implementation corresponds to the following algorithm, which therefore provides the leftmost and longest
match, also known as POSIX semantics,

,
if 𝑥 = ;
llMatch(𝑠, 𝑅) def
= let 𝑥 = MatchEnd(𝑠 𝑟⟨0⟩, ⊤*·𝑅𝑟) in
⟨𝑥 𝑟, MatchEnd(𝑥 𝑟, 𝑅)⟩, otherwise.
In the actual implementation the first phase initially locates the end location 𝑠 ⟨𝑗⟩ by simulating a
PCRE “lazy” loop ⊤*? concatenated with 𝑅, and subsequently locates the start location backwards
from 𝑠 ⟨𝑗⟩ by using 𝑅𝑟. It is typically more economical to start the search from the start of the string
rather than backwards from the end of the string, although both variants are possible.
Example 4.1. Consider 𝑠 = "###abacarabacaraba##" and 𝑅 = abacaraba. So 𝑅𝑟 = abaracaba
and 𝑠 𝑟 = "##abaracabaracaba###". Then MatchEnd(𝑠 𝑟⟨0⟩, ⊤*·𝑅𝑟) = 𝑠 𝑟⟨17⟩ = 𝑠 ⟨|𝑠 | − 17⟩𝑟 = 𝑠 ⟨3⟩𝑟
and MatchEnd(𝑠 ⟨3⟩, 𝑅) = 𝑠 ⟨12⟩. So llMatch(𝑠, 𝑅) = ⟨𝑠 ⟨3⟩, 𝑠 ⟨12⟩⟩. Observe that MatchEnd(𝑠, ⊤*·𝑅) =
𝑠 ⟨18⟩ would be the end location of the second match. Therefore, when starting with the end location
search, the initial *-loop must be treated or simulated as a lazy loop.
If we instead let 𝑅 = abacaraba\b so that the match must end a word then in 𝑅𝑟 = \babaracaba
the match must start a word. In that case MatchEnd(𝑠 𝑟⟨0⟩, ⊤*·𝑅𝑟) = 𝑠 𝑟⟨11⟩ = 𝑠 ⟨9⟩𝑟 because # is not
a word-letter, and then llMatch(𝑠, 𝑅) = ⟨𝑠 ⟨9⟩, 𝑠 ⟨18⟩⟩.
The POSIX match for 𝑅 in 𝑠, if a match exists, is a pair ⟨𝑠 ⟨𝑖⟩, 𝑠 ⟨𝑗⟩⟩ where 𝑠 ⟨𝑖⟩ is the minimal
𝑅 𝑦 and 𝑠 ⟨𝑗⟩ is the maximal location in 𝑠 such that 𝑠 ⟨𝑖⟩ 𝑅 𝑠 ⟨𝑗⟩.
location in 𝑠 such that ∃𝑦 : 𝑠 ⟨𝑖⟩ −→
−→
The following is the correctness theorem of llMatch.
Theorem 3 (Match). For all 𝑠 ∈ Σ∗ and 𝑅 ∈ R the following statements hold:
𝑅 𝑠 ⟨𝑗⟩;
(1) llMatch(𝑠, 𝑅) = ⇔ 𝑖, 𝑗 : 𝑠 ⟨𝑖⟩ −→
(2) if llMatch(𝑠, 𝑅) ≠ then llMatch(𝑠, 𝑅) is the POSIX match for 𝑅 in 𝑠.
𝑆 ).
Proof. By using Theorems 1 and 2, and that for all 𝑥 and 𝑆 ∈ R, MatchEnd(𝑥, 𝑆) = max(𝑥 −→
𝑟
𝑟
𝑟
⊤
*
·𝑅
The proof also uses the fact that if 𝑠 ⟨𝑖⟩ = max(𝑠 ⟨0⟩ −−−−−→) then 𝑠 ⟨𝑖⟩ is the minimal location in 𝑠
*
such that 𝑠 ⟨𝑖⟩ −𝑅·⊤
□
−−−→ 𝑠 ⟨|𝑠 |⟩.

5

IMPLEMENTATION

Here we give a high level overview of how the engine was implemented, what parts are shared with
the rest of .NET and the nonbacktracking engine, and some key differences and implementation
details.
15

<!-- page 16 -->

The core parser was taken directly from the .NET runtime, but was modified to read the symbols
& and ∼ as intersection and negation respectively. Fortunately the escaped variants ‘\&’ and ‘\∼’
were not assigned to any regex construct, and all existing regex patterns can be used by escaping
these two characters.
Another important component we directly took from the .NET runtime is the alphabet compression of the nonbacktracking engine that builds the minimal number of predicates on the underlying
character set by applying appropriate Boolean combinations. For example, all input characters
in the the regex ab can be represented with just 3 predicates: 𝑎, 𝑏 and [ˆ𝑎𝑏]. It is an essential
preprocessing step for such a large alphabet (16-bit Unicode Basic Multilingual Plane in our case),
which allows us to represent the input predicates as bits, and then use bitwise operations to check
if a character is in the set of characters represented by a predicate.
The engine itself is implemented in mostly high-level F#, without caching any derivatives, which
does not give it the optimal performance characteristics as of at the time of writing. The aim
is rather to provide a working prototype that, despite having much higher number of memory
allocations than, e.g., the nonbacktracking or backtracking alternatives of .NET 7, is still capable of
solving problems that neither the backtracking nor nonbacktracking variant can currently solve.
To make the modifications possible, the System.Text.RegularExpressions library was copied
to another namespace and the visibility of the library was changed from internal to public. The
only other meaningful modification to the library is the extension of the parser to parse negation
and intersection symbols appropriately, as described above.
The concatenation and 𝜖-nodes were implemented as just a single-linked list of other regex
nodes, as it allows to traverse to next one using just a tail pointer, without needing any other
steps. There is never any need to traverse the concatenation backwards, thus allowing to take
advantage of what the concatenation fundamentally is – a single linked list. And () is just an empty
concatenation, i.e. empty list.
5.1

Rewrite Rules and Subsumption

Our system implements a number of regex rewrite rules, which are essential for the efficiency of
the implementation. Figure 1 illustrates the basic rewrite rules that are always applied when regular
expressions are constructed. Intersection and union are implemented as commutative, associative
and idempotent operators, so changing the order of their arguments does not change the result.
There are many further derived rules that can be beneficial in reducing the state space. Unions
and intersections are both implemented by sets. If a union contains a regex 𝑆, such as a predicate 𝜓 ,
that is trivially subsumed by another regex 𝑅, such as 𝜓 *, then 𝑆 is removed from the union. This
is an instance of the loop rule in Figure 1 that rewrites 𝜓 {0, ∞}|𝜓 {1, 1} to 𝜓 {0, ∞}.
A further simplification rule for unions is that if a union contains a regex 𝜓 * and all the other
alternatives only refer to elements from [[𝜓 ]] then the union reduces to 𝜓 *. This rule rewrites any
union such as (.*ab.* | .*) to just .* (recall that . ≡ [^\n]), which significantly reduces the number
of alternations in total. Such rewrite could potentially be extended even further, e.g., rewriting
(.*ab.* | .*b.*) to .*b.*, but the detection overhead is more complex for rewrites like this and has
not been evaluated yet.
5.2 POSIX Semantics
One key difference from the .NET 7 nonbacktracking engine, is that we treat both alternations
and intersections as sets, which makes the semantics different from the backtracking one. For
example, matching the regular expression (a|ab)* on the string “abab” with the current .NET
engines results in the prefix “a”, as the leftmost alternation finds a successful match. Our engine
16

<!-- page 17 -->

~(⊤*)

~⊥

~~𝑅

~()

~(⊤+)

⊥·𝑅

𝑅·⊥

()·𝑅

𝑅·()

⊥*

⊥

⊤*

𝑅

⊤+

()

⊥

⊥

𝑅

𝑅

()

⊤* | 𝑅

⊤* & 𝑅

⊥|𝑅

⊥&𝑅

𝑅{1, 1}

𝑅{0, 0}

⊤*

𝑅

𝑅

⊥

𝑅

()

() & 𝑅

() & 𝑅
(Null𝑥 (𝑅))

𝑅

⊥

𝑅{𝑙, 𝑚} | 𝑅{𝑘, 𝑛}
(not Null𝑥 (𝑅))

(𝑙 ≤ 𝑘 ≤ 𝑚)

𝑅{𝑙, max(𝑚, 𝑛)}
Fig. 1. Basic rewrite rules.

will match the whole string “abab”, as all the alternations are explored in parallel, and the longest
match is in the right alternation. The behavior of IsMatch(𝑥, 𝑅) is identical in both engines.
In the nonbacktracking engine, PCRE semantics is achieved by prepending ⊤*? to the regex
pattern, which is always nullable and has only two potential derivatives. For example, in the case
of the pattern ⊤*?ab, the derivative can be either b|⊤*?ab, when the character matches 𝜓 a (the
initial pattern creates a new alternation) , or ⊤*?ab, (i.e. just the initial pattern) when it does not.
In our case we meet POSIX semantics by keeping the initial pattern as is, but starting a potential
match on every input position, that has a valid starting predicate. Then we keep track of the
nullability of all alternations the initial pattern produced separately. This is done by having a set of
"top-level alternation" data structures, each of which consist of a regex and the maximum nullable
position of the alternation. Once an alternation turns into ⊥, we check if the alternation has a
nullable position, and if it does, we return the match. If the alternation does not have a nullable
position, we remove it from the set and continue matching the remaining alternations.
5.3

Conjunctive Startset Lookup and Skipping

One very important optimization that allows to avoid doing a lot of unnecessary work on average,
depending on the input string and regular expression, is the startset optimization, which enables
search for characters belonging to the set of initial characters of a regex denoted by the startset
predicate by using dedicated string traversal constructs which can be several orders of magnitude
faster than checking characters one-by-one.
The result of evaluating the predicate startset is captured in the state of the regex node. Startset
optimization is often comprised of checking if the first character of the regex pattern is present in
the input string. As mentioned in Section 2.3, we make use of the fact that determining the startset
of a regex with derivatives is a relatively cheap operation, and we can use it to incorporate efficient
vectorized string-search procedures into the matching algorithm.
A key difference from other regex engines is that we don’t just skip until the initial startset of
the regex, but perform intermediate startset computations and appropriate skipping in the inner
loop as well. To take advantage of the fast startset check, we first need to determine if a regex is
“skippable” or not, where we determine if taking the derivative changes the node on only a subset
of the input alphabet. Such regexes start with a *-loop.
A skippable regex is defined as a regex that starts with a *-loop, or one in which all child regexes
recursively start with a *-loop, such as an alternation, lookahead, intersection or negation of regexes
starting only with *-loops. Lookbacks are more complex and do not currently take advantage of
this optimization. The key insight of this skippable check is that *-loops keep producing the same
regex derivative until either the loop terminator or tail of one of the *-loops matches.
17

<!-- page 18 -->

For example, all regexes starting with ⊤* are skippable, as according to the derivation rules in
Section 4.2: the derivative of the regex does not change unless the tail of the concatenation produces
a non-⊥ derivative. The same is true for all regexes starting with .*, as the only two predicates
that produce a non-initial derivative are 𝜓 \n , that is the loop termination predicate for .*, and the
starting predicate of the concatenation tail. For example, the regex .*ab has two predicates that
change the state: 𝜓 \n , in which case the regex becomes ⊥, and 𝜓 a , in which case the regex becomes
b|.*ab. All other predicates produce the initial regex.
Such skipping becomes useful when we want to match over input text with multiple intersections
or alternations. For example, consider the regex (12.*)&(.*\d), which is strictly non-skippable,
as the transition from 12.* to ⊥ or to 2.* is an important part of the matching process. However,
the regex (.*12.*)&(.*\d) is skippable, allowing us to look up the occurrence of a character
satisfying the predicate 𝜓 [\d\n] (the disjunction of predicates 1, \d and the loop exit condition \n),
and continue matching directly at that position.
The in-match startset computation heuristic itself is very simple, and only considers the first
node of the regex, or the first node and the second node recursively in the case of a star. In the case
of a regex starting with a predicate, we take the predicate itself. In the case of a predicate star loop,
we take the negation of the loop body predicate, and union it with the concatenation tail predicate,
as done in the previous example. In the case of an intersection, alternation or complement, we take
the union of the startsets of the contained regex bodies. Notably, the startset of a complement is
not the complement of the startset of the contained regex, but the same startset, as the derivation
rules apply the same way inside negation.
After computing the joint startset of the regex, we use it in a vectorized index-of lookup in the
following input string. Currently, we perform this lookup in an overestimated manner, by taking
the union of all startsets, but this can be improved upon by taking the intersection in some cases.
For example, if we have an intersection with the startsets 𝜓 \d and 𝜓 1 strictly in the same position,
then we know ahead-of-time that the startset of the intersection is constrained to only 𝜓 1 - the
conjunction of the two startsets.
The startset optimization is very powerful as it allows us to skip over large parts of the input
string in a single step, and only perform the more expensive derivative computation on the
remaining positions. A future improvement would be to extend this to multi-character predicates
and intersections, which would allow us to skip over even more of the input string.
6

EVALUATION

The evaluation of the engine is still in its early stages, and we have not yet implemented all the
optimizations that are possible, such as caching transition regexes, or skipping over multi-character
predicates. However, despite being only a research prototype, we can already see that the engine is
capable of solving problems that neither of the available .NET regex engines can currently solve.
An industrial implementation of the engine should reach performance characteristics comparable
to [Moseley et al. 2023] over standard regexes, because the same .NET framework is used, with
potential marginal gains in space/time due to commutative |, as non-commutativity of | disallows some useful rewrites in [Moseley et al. 2023]. Moreover, all initial fixed-prefix/suffix-search
optimizations are shared across all backends. For a more comprehensive evaluation on standard
regexes outside .NET, see the evaluation of [Moseley et al. 2023] directly.
Note that we do not use the caching mechanism from the nonbacktracking engine in our
prototype, as it is not yet fully implemented, and would require a significant amount of work to
integrate in the presence of lookarounds. The lack of caching is the main reason why our engine is
not yet competitive with the nonbacktracking engine on the benchmarks.
18

<!-- page 19 -->

107

Time to find a match (µs)

106

Backtracking lookahead
Backtracking loop
NonBacktracking loop
SBRE

105
104
103
102
101
2

4

6
8
Number of substrings matched

10

12

Fig. 2. Time taken to find a matching paragraph containing all required substrings in any order in a 9kB
sample text.

We have evaluated the performance of our engine against the .NET default backtracking engine,
and the symbolic automata based nonbacktracking version. The benchmarks were run on a machine
with an AMD Ryzen 7 5800X 8-Core CPU, and 32 GB of RAM. The benchmarks were run on .NET
version 7.0.305.
We compare the performance of extracting paragraphs containing multiple substrings from the
collected works of Mark Twain [Herczeg 2015], first we compare the performance on a 9 kB string,
containing 34 paragraphs, extracted from the lines 188589 to 188771, and then on the entire 20 MB
file, containing 379897 lines in total.
The paragraphs are extracted with three kinds of patterns: one that uses a negative lookahead to
match the end of the paragraph, one that uses a line loop until the occurrence of two sequential
newlines, and one that uses negation to constrain the paragraph range and intersections to constrain
the paragraph contents.
Examples of the patterns used to match paragraphs containing the word "King":
(1) Negative lookahead: \n\n((?!\n\n)[\s\S])*?(King)((?!\n\n)[\s\S])*?\n\n
(2) Loop: \n\n((.+\n)+?(.*King.*\n)(.+\n)+?)\n
(3) Conjunction, negation: \n\n~([\s\S]*\n\n[\s\S]*)\n&[\s\S]*King[\s\S]*
Note that using a simpler pattern, such as \n\n[\s\S]*?King[\s\S]*?\n\n, would not work, as
any non-match inside the paragraph would cause [\s\S]*? to leak outside the current paragraph,
and match over multiple paragraphs.
The substrings we look for in the paragraph are "King", "Paris", "English", "would", "rise", "struck",
"council", "march", "war", "May", "Orleans" and "work", where we introduce the next substring, as
the number of substrings increases. In the case of 1 substring we use "King", for 2 substrings we
look for "King" and "Paris" in the same paragraph, etc.
The results of an experiment run on a 9 kB string where the first paragraph matching the number
of substrings indicated on the x axis is given in Fig. 2. As the y axis is logarithmic it can be seen how
19

<!-- page 20 -->

Table 2. Paragraph extraction pattern lengths in characters

n of substrings
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

Neg. lookahead
50
101
363
1869
11805
87885
740925
6854445
69310125
769305645

Loop
34
77
295
1513
9385
69145
579625
5322265
53343385
587865625

& and ∼
46
66
88
108
127
148
170
190
208
226

using the intersections produces more efficient match as there is no blowup in match orderings
required in conventional regular expressions.
In the 9 kB sample text, our engine, SBRE [Varatalu 2023] starts becoming faster than the
rest at 3 substrings, but due to lack of caching has 4 times higher memory allocations than the
nonbacktracking engine and about 9 times higher allocations than the backtracking implementation.
After a certain number of substrings, the order of the matchings (Table 2) becomes so large that
both the backtracking and nonbacktracking engines stop being able to find a match within 60
seconds, while our engine still manages to complete the match with minor slowdowns.
In the full 20 MB text, our research prototype (SBRE) takes significantly more unnecessary
derivatives than the nonbacktracking variant, as it suffers from the lack of caching, but after a
certain number of substrings, it is still the only engine that can complete the match within 60
seconds. The results are shown in Fig. 3.
A notable observation in the full text is that the lookahead pattern of the backtracking engine falls
off significantly earlier than in the 9 kB text, where it showed promising results up to 6 substrings.
In the full text, the nonbacktracking engine shows the best performance up to 6 substrings, but
then falls off completely, as the memory allocated blows up from 74.41 MB in 5 paragraphs to 3.39
GB in 6 paragraphs. The nonbacktracking engine was not able to produce a result with 7 substrings
within 10 minutes.
The results show that our engine is able to search for a large amount of substrings in a paragraph
efficiently, while the other engines suffer from the exponential blowup in match orderings. In the
full text, adding a substring to the SBRE pattern after 2 substrings currently causes a constant
memory allocation growth of 800 MB, which is a significant amount of memory, but still allows
for the match to complete, while the other engines are not able to complete the match at all. The
memory usage of the SBRE engine could be significantly reduced by caching the derivatives, which
would make the engine more competitive overall.
7

RELATED WORK

Regular expressions have in practice many extensions, such as backreferences and balancing groups,
that reach far beyond regular languages in their expressive power. Such extensions, see [Loring et al.
2019], fall outside the scope of this paper. The focus on related work here is solely on automata and
derivative based matching algorithms, tools, and techniques, that in some form or shape maintain,
at least in principle, a finite state based view corresponding to regular languages. In particular,
lookaheads maintain regularity [Morihata 2012] and regular expressions with lookaheads can
20

<!-- page 21 -->

Time to find a match (ms)

105

104

103

102
Backtracking lookahead
Backtracking loop
NonBacktracking loop
SBRE

101

100
2

3

4

5
6
7
Number of substrings matched

8

9

10

Fig. 3. Time taken to find a matching paragraph containing all required substrings in any order in the full
20MB sample text.

be converted to Boolean automata [Berglund et al. 2021b]. [Chida and Terauchi 2023] consider
extended regular expressions in the context of backreferences and lookaheads. They build on [Carle
and Narendran 2009] to show that extended regular expressions involving backreferences and both
positive and negative lookaheads leads to undecidable emptiness, but, when restricted to positive
lookaheads only is closed under complement and intersection. [Miyazaki and Minamide 2019]
present an approach to find match end with derivatives in regular expressions with lookaheads.
The semantics of derivatives in [Miyazaki and Minamide 2019] uses Kleene algebras with lookahead
as an extension of Kleene algebras with tests [Kozen 1997], and is fundamentally different from our
formulation in several aspects, where the underlying semantic concatenation is commutative and
idempotent and where the difference to derivatives of concatenations in relation to [Brzozowski
1964] is also pointed out, which also leaves unclear the question of supporting lookbacks and reverse.
Derivatives in combination of Kleene algebras are also studied in [Pous 2015]. Our approach is a
conservative extension of the theory in [Moseley et al. 2023] with intersection and complement, as
well as positive and negative lookbacks and lookaheads, that builds on [Brzozowski 1964], leading
to the core fundamental results of Theorem 1 and Theorem 2.
In functional programming derivatives were studied in [Fischer et al. 2010; Owens et al. 2009]
for IsMatch. [Ausaf et al. 2016; Sulzmann and Lu 2012] study MatchEnd with Antimirov derivatives
and POSIX semantics and also Brzozowski derivatives in [Ausaf et al. 2016] with a formalization
in Isabelle/HOL. The algorithm of [Sulzmann and Lu 2012] has been recently further studied
in [Tan and Urban 2023] as a recursive functional program and also formalized in Isabelle/HOL.
The fundamental difference to the theory here is that, because of lookarounds (that in particular
enable the definition anchors) imply that certain classical laws, such as several of the inhabitation
relation rules in [Tan and Urban 2023], become invalid. In [Wingbrant 2019] anchors are considered
as special symbols in an extended alphabet, using classical derivatives. This approach has the
drawback that anchors are intended as specialized lookarounds and treating them as special
symbols conflicts with match semantics in terms of locations. Some aspects of our work here, such
21

<!-- page 22 -->

as support for intersection, are related to SRM [Saarikivi et al. 2019] that is the predecessor of
the NonBacktracking regex backend of .NET [Moseley et al. 2023], but SRM lacks support for
lookarounds as well as anchors. The top-level matcher of SRM is also different and more costly
because it uses three passes over the input to locate a match, instead of two.
State-of-the-art nonbacktracking regular expression matchers based on automata such as RE2 [Cox
2010] and grep [GNU 2023] using variants of [Thompson 1968], and Hyperscan [Wang et al. 2019]
using a variant of [Glushkov 1961], as well as the derivative based NonBacktracking engine
in .NET make heavy use of state graph memoization. None of these engines currently support
lookarounds intersection or complement. An advantage of using derivatives is that they often
minimize the state graph (but do not guarantee minimization), as was already shown in [Owens
et al. 2009, Table 1] for DFAs. Further evidence of this is also provided in [Sulzmann and Lu 2012,
Section 5.4] where NFA sizes are compared for Thompson’s and Glushkov’s, versus Antimirov’s
constructions, showing that Antimirov’s construction consistently yields a smaller state graph. In
automata-based engines an upfront DFA minimization is undesirable because it is too costly, while
derivatives allow DFA-minimizing optimizations to be applied essentially on-the-fly. In general,
the rewrite rules applied in our framework are not feasible in automata-caching, because the
corresponding semantic language-level checks would require global analysis, because in traditional
automata based representations one has lost the relationship between states and regular expressions.
In our case we preserve the relationship between regular expressions
The two phases of the top-level matching algorithm: to find the match end location and to find
the match start location are similar in RE2 [Cox 2010] as well as in .NET NonBacktracking which
our implementation builds on top of. It therefore also benefits from switching to NFA mode when a
certain threshold of DFA states is reached, having an overall effect similar to that of RE2 [Cox 2010],
but the derivatives can switch to Antimirov-style derivatives without any prior bookkeeping. The
top-level loop is in some sense oblivious to the fact that intersection, complement, and lookarounds
are being used inside the regular expressions.
The two main standards for matching are PCRE (backtracking semantics) and POSIX [Berglund
et al. 2021a; Laurikari 2000]. Greedy matching algorithm for backtracking semantics was originally
introduced in [Frisch and Cardelli 2004], based on 𝜖-NFAs, while maintaining matches for eager
loops. We should point out that [Frisch and Cardelli 2004, Proposition 2] assumes the axiom
𝐿(𝑅·𝑆) = 𝐿(𝑅)·𝐿(𝑆) that fails with anchors or lookarounds. While backtracking semantics is needed
in .NET for compatibility across all the backends – including NonBacktracking [Moseley et al.
2023] – here we use POSIX semantics that allows us to treat alternations and intersections as
commutative operations. The rationale behind using POSIX is that it is semantically unclear as
to what de Morgan’s laws and laws of distributivity would mean in the context of backtracking
semantics if intersection would be treated as a noncommutative operation.
The results [Moseley et al. 2023, Theorem 3.3 and Theorem 3.8] form a strict subset of our
Theorem 1 and Theorem 2 whose proofs are novel and nonobvious because definitions of nullability
and derivatives are mutually recursive. It was far from obvious if regex complement and intersection
could even be combined in any meaningful way with lookarounds. It remains unclear to us, as to if
the main result [Moseley et al. 2023, Theorem 4.5] that builds on formally linking the semantics
of derivatives to backtracking (PCRE) semantics can be extended to cover the extended fragment
of regexes defined here by R due to noncommutativity of alternations in backtracking semantics,
the proof of [Moseley et al. 2023, Theorem 4.5] is much more complicated than that of Theorem 3.
Intersection was also included as an experimental feature in the initial version of SRM [Saarikivi et al.
2019] by building directly on derivatives in [Brzozowski 1964], and used an encoding via regular
expression conditionals (if-then-else) that unfortunately conflicts with the intended semantics of
conditionals and therefore has, to the best of our knowledge, never been used or evaluated.
22

<!-- page 23 -->

The conciseness of using intersection and complement in regular expressions is also demonstrated
in [Gelade and Neven 2012] where the authors show that using intersection and complement in
regular expressions can lead to a double exponentially more succinct representation of regular
expressions.
8

FUTURE WORK

The theory of derivatives based on locations that is developed here can be used to extend regular
expressions with lookarounds in SMT solvers that support derivative based lazy exploration of
regular expressions as part of the sequence theory, such solvers are CVC4 [CVC4 2020; Liang
et al. 2015] and Z3 [de Moura and Bjørner 2008; Stanford et al. 2021]. A further extension is to lift
the definition of location derivatives to a fully symbolic form as is done with transition regexes in
Z3 [Stanford et al. 2021]. [Chen et al. 2022] mention that the OSTRICH string constraint solver could
be extended with backreferences and lookaheads by some form of alternating variants of prioritized
streaming string transducers (PSSTs), but it has, to our knowledge, not been done. Such extensions
would widen the scope of analysis of string verification problems that arise from applications that
involve regexes using anchors and lookarounds. It would then also be beneficial to extend the
SMT-LIB [SMT-LIB 2021] format to support lookarounds.
Counters are a well-known Achilles heel of essentially all nonbacktracking state-of-the-art
regular expression matching engines as recently also demonstrated in [Turoňová et al. 2022], which
makes any algorithmic improvements of handling counters highly valuable. In [Turoňová et al.
2020], Antimirov-style derivatives [Antimirov 1996] are used to extend NFAs with counting to
provide a more succinct symbolic representation of states by grouping states that have similar
behavior for different stages of counter values together using a data-structure called a counting-set.
It is an intriguing open problem to investigate if this technique can be adapted to work with location
derivatives within our current framework. [Glaunec et al. 2023] point out that it is important to
optimize specific steps of regular expression matching to address particular performance bottlenecks.
The specific BVA-Scan algorithm is aimed at finding matches with regular expressions containing
counters more efficient. [Holík et al. 2023] report on a subset of regexes with counters called
synchronizing regexes that allow for fast matching.
Further work of improved rewrite rules is also needed in our current implementation to reduce
the number of states that arise in the matching engine, as well as enhancing caching techniques
that record transitions that have already been seen for lookarounds. Currently, no special handling
of transitions arising from lookarounds is taking place. And, as intersections allow precise text
extraction in a single pass, the approach can be optimized further by application of vectorization.
One of the main drawbacks of lookarounds is that they are not cacheable in the same way as
other regexes, as they depend on the position of the matcher in the string. This means that the
regex engine cannot statically determine if a lookaround will match at a given position and has to
try the lookaround at every position.
This is not a problem for the short context-lookups, like the anchor lookarounds in Table 1, as
they do not move around in the string, and retain the linear time complexity of the regex. However,
this is a problem for the other lookarounds, such as the lookback (?<=a.*)b, where the occurrence
of the character a has to be re-checked at every occurrence of b. This leads to a non-ideal quadratic
time complexity.
However, these kinds of lookarounds could be cached by storing the predicate 𝜓. = 𝜓 [^\n] and
passing it along with the derivative. This way, the regex engine will only invalidate the cache upon
finding an occurrence of a character that does not match the loop predicate, i.e., in the case of
(?<=a.*) this cached predicate will be invalidated by \n, which is the only character not in [[𝜓.]] .
23

<!-- page 24 -->

This caching technique could lead to a linear time complexity for many lookarounds but needs
more research.
One notable feature that is missing from our regexes is support for laziness. However, this
feature can often be emulated by using complement and negation of character classes. For example,
the regex a.*?b, which matches the shortest string in the range of a and b, can be emulated
by a[^b\n]*b, which has the exact same semantics, but without using laziness. Another way to
emulate laziness is by using intersection and complement, as in (a.*)&(~(.*b.*)b), which is also
equivalent a.*?b. Note that ~(.*b.*), that means “does not contain b”, matches before b where
the final match character must be b in (~(.*b.*)b).
9

CONCLUSION

We have presented both a theory and implementation for a combination of extensions to regular
expressions including complement, intersection and positive and negative lookarounds that have
not previously been explored in depth in such a combination. Prior work has analyzed different
other sets of extensions and their properties, but several such combinations veer out of the scope
of regular languages. The work combines and extends the symbolic derivatives based approaches
presented in [Saarikivi et al. 2019] and [Moseley et al. 2023] by showing how the carefully selected
set of extensions to regular expressions yield an effective Boolean algebra on the match set semantics,
producing a regular language with interesting new applications and opening up a principled way
to defining rewrite rules for optimizations that play an important role in practical applications.
In addition, we have provided a precisely defined approach to finding matches using symbolic
derivatives.
When considering context-sensitive anchors, we showed how anchors can be represented in
terms of lookarounds and are thus supported by our approach. Moreover, such generalization
provides possibilities for defining custom anchors.
The implementation reuses several components of the .NET 7 nonbacktracking regular expression
backend and adds support for newly introduced extensions. To demonstrate the efficacy of the
approach we showed that the task of locating a substring containing a set of matches in arbitrary
order can be solved by the proposed engine while neither the nonbacktracking nor backtracking
engines of .NET 7 scaled due to the factorial blowup of the possible orderings of the matches.
REFERENCES
Valentin Antimirov. 1996. Partial Derivatives of Regular Expressions and Finite Automata Constructions. Theoretical
Computer Science 155 (1996), 291–319. https://doi.org/10.1007/3-540-59042-0_96
Fahad Ausaf, Roy Dyckhoff, and Christian Urban. 2016. POSIX Lexing with Derivatives of Regular Expressions (Proof Pearl).
In Interactive Theorem Proving (LNCS, Vol. 9807), Jasmin Christian Blanchette and Stephan Merz (Eds.). Springer, 69–86.
https://doi.org/10.1007/978-3-319-43144-4_5
Martin Berglund, Willem Bester, and Brink van der Merwe. 2021a. Formalising and implementing Boost POSIX regular
expression matching. Theoretical Computer Science 857 (2021), 147–165. https://doi.org/10.1016/j.tcs.2021.01.010
Martin Berglund, Brink van der Merwe, and Steyn van Litsenborgh. 2021b. Regular Expressions with Lookahead. Journal of
Universal Computer Science 27, 4 (2021), 324–340. https://doi.org/10.3897/jucs.66330
Janusz A. Brzozowski. 1964. Derivatives of regular expressions. JACM 11 (1964), 481–494. https://doi.org/10.1145/321239.
321249
Benjamin Carle and Paliath Narendran. 2009. On Extended Regular Expressions. In Language and Automata Theory and
Applications, Third International Conference, LATA 2009, Tarragona, Spain, April 2-8, 2009. Proceedings (Lecture Notes
in Computer Science, Vol. 5457), Adrian-Horia Dediu, Armand-Mihai Ionescu, and Carlos Martín-Vide (Eds.). Springer,
279–289. https://doi.org/10.1007/978-3-642-00982-2_24
Taolue Chen, Alejandro Flores-Lamas, Matthew Hague, Zhilei Han, Denghang Hu, Shuanglong Kan, Anthony W. Lin, Philipp
Rümmer, and Zhilin Wu. 2022. Solving String Constraints with Regex-Dependent Functions through Transducers with
Priorities and Variables. Proc. ACM Program. Lang. 6, POPL, Article 45 (jan 2022), 31 pages. https://doi.org/10.1145/3498707

24

<!-- page 25 -->

Nariyoshi Chida and Tachio Terauchi. 2023. On Lookaheads in Regular Expressions with Backreferences. IEICE Trans. Inf.
Syst. 106, 5 (2023), 959–975. https://doi.org/10.1587/transinf.2022edp7098
Russ Cox. 2010. Regular Expression Matching in the Wild. https://swtch.com/~rsc/regexp/regexp3.html
CVC4. 2020. https://github.com/CVC4/CVC4.
Leonardo de Moura and Nikolaj Bjørner. 2008. Z3: An Efficient SMT Solver. In TACAS’08 (LNCS). Springer, 337–340.
https://doi.org/10.1007/978-3-540-78800-3_24
Sebastian Fischer, Frank Huch, and Thomas Wilke. 2010. A Play on Regular Expressions: Functional Pearl. SIGPLAN Not. 45,
9 (2010), 357–368. https://doi.org/10.1145/1863543.1863594
J.E.F. Friedl. 2006. Mastering Regular Expressions, 3rd Edition. O’Reilly Media, Incorporated. https://books.google.ee/books?
id=px6JzwEACAAJ
Alain Frisch and Luca Cardelli. 2004. Greedy Regular Expression Matching. In Automata, Languages and Programming
(ICALP’04) (LNCS, Vol. 3142), Josep Díaz, Juhani Karhumäki, Arto Lepistö, and Donald Sannella (Eds.). Springer, 618–629.
https://doi.org/10.1007/978-3-540-27836-8_53
Wouter Gelade and Frank Neven. 2012. Succinctness of the Complement and Intersection of Regular Expressions. ACM
Trans. Comput. Log. 13, 1 (2012), 4:1–4:19. https://doi.org/10.1145/2071368.2071372
Alexis Le Glaunec, Lingkun Kong, and Konstantinos Mamouras. 2023. Regular Expression Matching using Bit Vector
Automata. Proc. ACM Program. Lang. 7, OOPSLA1 (2023), 492–521. https://doi.org/10.1145/3586044
V. M. Glushkov. 1961. The abstract theory of automata. Russian Math. Surveys 16 (1961), 1–53. https://doi.org/10.1070/
RM1961v016n05ABEH004112
GNU. 2023. grep. https://www.gnu.org/software/grep/.
Google. 2023. RE2. https://github.com/google/re2.
Zoltan Herczeg. 2015. Performance comparison of regular expression engines. https://zherczeg.github.io/sljit/regex_perf.
html.
Lukás Holík, Juraj Síc, Lenka Turonová, and Tomás Vojnar. 2023. Fast Matching of Regular Patterns with Synchronizing
Counting. In Foundations of Software Science and Computation Structures - 26th International Conference, FoSSaCS 2023,
Held as Part of the European Joint Conferences on Theory and Practice of Software, ETAPS 2023, Paris, France, April 22-27,
2023, Proceedings (Lecture Notes in Computer Science, Vol. 13992), Orna Kupferman and Pawel Sobocinski (Eds.). Springer,
392–412. https://doi.org/10.1007/978-3-031-30829-1_19
Dexter Kozen. 1997. Kleene algebra with tests. TOPLAS 19, 3 (1997), 427–443. https://doi.org/10.1145/256167.256195
V. Laurikari. 2000. NFAs with tagged transitions, their conversion to deterministic automata and application to regular
expressions. In 7th International Symposium on String Processing and Information Retrieval. 181–187. https://doi.org/10.
1109/SPIRE.2000.878194
Tianyi Liang, Nestan Tsiskaridze, Andrew Reynolds, Cesare Tinelli, and Clark Barrett. 2015. A Decision Procedure for
Regular Membership and Length Constraints over Unbounded Strings?. In Frontiers of Combining Systems, FroCoS 2015
(LNCS, Vol. 9322). Springer, 135–150. https://doi.org/10.1007/978-3-319-24246-0_9
Blake Loring, Duncan Mitchell, and Johannes Kinder. 2019. Sound Regular Expression Semantics for Dynamic Symbolic
Execution of JavaScript. In PLDI’19. ACM, 425–438. https://doi.org/10.1145/3314221.3314645
Microsoft. 2021. Regular Expression Language - Quick Reference. https://docs.microsoft.com/en-us/dotnet/standard/basetypes/regular-expression-language-quick-reference.
Takayuki Miyazaki and Yasuhiko Minamide. 2019. Derivatives of Regular Expressions with Lookahead. J. Inf. Process. 27
(2019), 422–430. https://doi.org/10.2197/ipsjjip.27.422
Akimasa Morihata. 2012. Translation of Regular Expression with Lookahead into Finite State Automaton. Computer Software
29, 1 (2012), 147–158. https://doi.org/10.11309/jssst.29.1_147
Dan Moseley, Mario Nishio, Jose Perez Rodriguez, Olli Saarikivi, Stephen Toub, Margus Veanes, Tiki Wan, and Eric Xu. 2023.
Derivative Based Nonbacktracking Real-World Regex Matching with Backtracking Semantics. In PLDI ’23: 44th ACM
SIGPLAN International Conference on Programming Language Design and Implementation, Florida, USA, June 17-21, 2023,
Nate Foster et al. (Ed.). ACM, 1–2.
Scott Owens, John Reppy, and Aaron Turon. 2009. Regular-expression Derivatives Re-examined. J. Funct. Program. 19, 2
(2009), 173–190. https://doi.org/10.1017/S0956796808007090
Damien Pous. 2015. Symbolic Algorithms for Language Equivalence and Kleene Algebra with Tests. ACM SIGPLAN Notices
– POPL’15 50, 1 (2015), 357–368. https://doi.org/10.1145/2775051.2677007
Olli Saarikivi, Margus Veanes, Tiki Wan, and Eric Xu. 2019. Symbolic Regex Matcher. In Tools and Algorithms for the
Construction and Analysis of Systems (LNCS, Vol. 11427), Tomáš Vojnar and Lijun Zhang (Eds.). Springer, 372–378.
https://doi.org/10.1007/978-3-030-17462-0_24
SMT-LIB. 2021. The Satisfiability Modulo Theories Library. http://smtlib.cs.uiowa.edu/
Caleb Stanford, Margus Veanes, and Nikolaj Bjørner. 2021. Symbolic Boolean Derivatives for Efficiently Solving Extended
Regular Expression Constraints. In PLDI’21. ACM, 620–635. https://doi.org/10.1145/3453483.3454066

25

<!-- page 26 -->

Martin Sulzmann and Kenny Zhuo Ming Lu. 2012. Regular Expression Sub-Matching Using Partial Derivatives. In Proceedings
of the 14th Symposium on Principles and Practice of Declarative Programming (PPDP’12). ACM, New York, NY, USA, 79–90.
https://doi.org/10.1145/2370776.2370788
Chengsong Tan and Christian Urban. 2023. POSIX Lexing with Bitcoded Derivatives. In 14th International Conference on
Interactive Theorem Proving (LIPICS, 26), A. Naumowicz and R. Thiemann (Eds.). Dagstuhl Publishing, 26:1–26:18.
Ken Thompson. 1968. Programming Techniques: Regular Expression Search Algorithm. Commun. ACM 11, 6 (jun 1968),
419–422. https://doi.org/10.1145/363347.363387
Lenka Turoňová, Lukáš Holík, Ivan Homoliak, Ondřej Lengál, Margus Veanes, and Tomáš Vojnar. 2022. Counting in
Regexes Considered Harmful: Exposing ReDoS Vulnerability of Nonbacktracking Matchers. In 31st USENIX Security
Symposium (USENIX Security 22). USENIX Association, Boston, MA, 4165–4182. https://www.usenix.org/conference/
usenixsecurity22/presentation/turonova
Lenka Turoňová, Lukáš Holík, Ondřej Lengál, Olli Saarikivi, Margus Veanes, and Tomáš Vojnar. 2020. Regex Matching with
Counting-Set Automata. Proc. ACM Program. Lang. 4, OOPSLA, Article 218 (Nov. 2020). https://doi.org/10.1145/3428286
Ian Erik Varatalu. 2023. SBRE - Symbolic Boolean Regular Expressions. https://doi.org/10.5281/zenodo.8369590
Xiang Wang, Yang Hong, Harry Chang, KyoungSoo Park, Geoff Langdale, Jiayu Hu, and Heqing Zhu. 2019. Hyperscan:
A Fast Multi-pattern Regex Matcher for Modern CPUs. In 16th USENIX Symposium on Networked Systems Design and
Implementation (NSDI 19). USENIX Association, Boston, MA, 631–648. https://www.usenix.org/conference/nsdi19/
presentation/wang-xiang
Ola Wingbrant. 2019. Regular languages, derivatives and finite automata. https://doi.org/10.48550/ARXIV.1907.13577

26
