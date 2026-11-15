---
file: ../pdfs/2022-popl-chen-string-constraints-regex-transducers.pdf
citation: "Solving String Constraints with Regex-Dependent Functions through Transducers with Priorities and Variables; Chen, Flores-Lamas, Hague, Han, Hu, Kan, Lin, Rümmer, Wu; POPL 2022 (PACMPL 6)"
doi: 
source: "https://www.cs.rhul.ac.uk/home/uxac009/files/papers/popl2022-long.pdf"
topics: [semantics_theory_access]
pages: 45
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Solving String Constraints with Regex-Dependent Functions
through Transducers with Priorities and Variables
TAOLUE CHEN, Birkbeck, University of London, United Kingdom
ALEJANDRO FLORES-LAMAS, Royal Holloway, University of London, United Kingdom
MATTHEW HAGUE, Royal Holloway, University of London, United Kingdom
ZHILEI HAN, Tsinghua University, China
DENGHANG HU, Institute of Software, Chinese Academy of Sciences & University of Chinese
Academy of Sciences, China
SHUANGLONG KAN, University of Kaiserslautern, Germany
ANTHONY W. LIN, University of Kaiserslautern & Max-Planck Institute, Germany
PHILIPP RÜMMER, Uppsala University, Sweden
ZHILIN WU, Institute of Software, Chinese Academy of Sciences & University of Chinese Academy
of Sciences, China
Regular expressions are a classical concept in formal language theory. Regular expressions in
programming languages (RegEx) such as JavaScript, feature non-standard semantics of operators
(e.g. greedy/lazy Kleene star), as well as additional features such as capturing groups and references.
While symbolic execution of programs containing RegExes appeals to string solvers natively
supporting important features of RegEx, such a string solver is hitherto missing. In this paper,
we propose the first string theory and string solver that natively provides such support. The key
idea of our string solver is to introduce a new automata model, called prioritized streaming string
transducers (PSST), to formalize the semantics of RegEx-dependent string functions. PSSTs combine
priorities, which have previously been introduced in prioritized finite-state automata to capture
greedy/lazy semantics, with string variables as in streaming string transducers to model capturing
groups. We validate the consistency of the formal semantics with the actual JavaScript semantics by
extensive experiments. Furthermore, to solve the string constraints, we show that PSSTs enjoy nice
closure and algorithmic properties, in particular, the regularity-preserving property (i.e., pre-images
of regular constraints under PSSTs are regular), and introduce a sound sequent calculus that exploits
these properties and performs propagation of regular constraints by means of taking post-images or
pre-images. Although the satisfiability of the string constraint language is generally undecidable,
we show that our approach is complete for the so-called straight-line fragment. We evaluate the
Authors’ addresses: Taolue Chen, Department of Computer Science, Birkbeck, University of London, Malet
Street, London, United Kingdom, t.chen@bbk.ac.uk; Alejandro Flores-Lamas, Department of Computer
Science, Royal Holloway, University of London, Egham Hill, Egham, Surrey, TW20 0EX, United Kingdom,
Alejandro.Flores-Lamas@rhul.ac.uk; Matthew Hague, Department of Computer Science, Royal Holloway,
University of London, Egham Hill, Egham, Surrey, TW20 0EX, United Kingdom, matthew.hague@rhul.ac.
uk; Zhilei Han, School of Software, Tsinghua University, China, hzl21@mails.tsinghua.edu.cn; Denghang
Hu, State Key Laboratory of Computer Science, Institute of Software, Chinese Academy of Sciences &
University of Chinese Academy of Sciences, China, hudenghang15@mails.ucas.ac.cn; Shuanglong Kan,
University of Kaiserslautern, Kaiserslautern, Germany, shuanglong@cs.uni-kl.de; Anthony W. Lin, University
of Kaiserslautern & Max-Planck Institute, Kaiserslautern, Germany, lin@cs.uni-kl.de; Philipp Rümmer,
Department of Information Technology, Uppsala University, Box 337, Uppsala, SE-751 05, Sweden, philipp.
ruemmer@it.uu.se; Zhilin Wu, State Key Laboratory of Computer Science, Institute of Software, Chinese
Academy of Sciences & University of Chinese Academy of Sciences, China, wuzl@ios.ac.cn.

© 2022
2475-1421/2022/1-ART45
https://doi.org/10.1145/3498707
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

45

<!-- page 2 -->

45:2

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

performance of our string solver on over 195 000 string constraints generated from an open-source
RegEx library. The experimental results show the efficacy of our approach, drastically improving
the existing methods (via symbolic execution) in both precision and efficiency.
CCS Concepts: • Theory of computation → Automated reasoning; Program verification;
Regular languages; Logic and verification; Complexity classes.
Additional Key Words and Phrases: String Constraint Solving, Regular Expressions, Transducers,
Symbolic Execution
ACM Reference Format:
Taolue Chen, Alejandro Flores-Lamas, Matthew Hague, Zhilei Han, Denghang Hu, Shuanglong
Kan, Anthony W. Lin, Philipp Rümmer, and Zhilin Wu. 2022. Solving String Constraints with
Regex-Dependent Functions through Transducers with Priorities and Variables. Proc. ACM Program.
Lang. 6, POPL, Article 45 (January 2022), 45 pages. https://doi.org/10.1145/3498707

1

INTRODUCTION

In modern programming languages—such as JavaScript, Python, Java, and PHP—the string
data type plays a crucial role. A quick look at the string libraries for these languages is
enough to convince oneself how well supported string manipulations are in these languages,
in that a wealth of string operations and functions are readily available for the programmers.
Such operations include usual operators like concatenation, length, substring, but also
complex functions such as match, replace, split, and parseInt. Unfortunately, it is well-known
that string manipulations are error-prone and could even give rise to security vulnerabilities
(e.g. cross-site scripting, a.k.a. XSS). One powerful method for identifying such bugs in
programs is symbolic execution (possibly in combination with dynamic analysis), which
analyses symbolic paths in a program by viewing them as constraints whose feasibility is
checked by constraint solvers. Together with the challenging problem of string analysis,
this interplay between program analysis and constraint solvers has motivated the highly
active research area of string solving, resulting in the development of numerous string solvers
in the last decade or so including Z3 [de Moura and Bjørner 2008], CVC4 [Liang et al.
2014], Z3-str/2/3/4 [Berzish et al. 2017; Berzish, Murphy 2021; Zheng et al. 2015, 2013],
ABC [Bultan and contributors 2015], Norn [Abdulla et al. 2014], Trau [Abdulla et al. 2017,
2018; Bui and contributors 2019], OSTRICH [Chen et al. 2019], S2S [Le and He 2018],
Qzy [Cox and Leasure 2017], Stranger [Yu et al. 2010], Sloth [Abdulla et al. 2019; Holı́k
et al. 2018], Slog [Wang et al. 2016], Slent [Wang et al. 2018], Gecode+S [Scott et al. 2017],
G-Strings [Amadini et al. 2017], HAMPI [Kiezun et al. 2012], among many others.
One challenging problem in the development of string solvers is the need to support
an increasing number of real-world string functions, especially because the initial stage of
the development of string solvers typically assumed only simple functions (in particular,
concatenation, regular constraints, and sometimes also length constraints). For example, the
importance of supporting functions like the replaceAll function (i.e. replace with global flag)
in a string solver was elaborated in [Chen et al. 2018]; ever since, quite a number of string
solvers support this operator. Unfortunately, the gap between the string functions that are
supported by current string solvers and those supported by modern programming languages
is still too big. As convincingly argued in [Loring et al. 2019] in the context of constraint
solving, the widely used Regular Expressions in modern programming languages (among
others, JavaScript, Python, etc.)—which we call RegEx in the sequel—are one important
and frequently occurring feature in programs that are difficult for existing SMT theories over
strings to model and solve, especially because their syntaxes and semantics substantially
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 3 -->

Solving String Constraints with Regex-Dependent Functions

45:3

differ from the notion of regular expressions in formal language theory [Hopcroft and Ullman
1979]. Indeed, many important string functions in programming languages—such as exec,
test, search, match, replace, and split in JavaScript, as well as match, findall, search, sub,
and split in Python—can and often do exploit RegEx, giving rise to path constraints that
are difficult (if not impossible) to precisely capture in existing string solving frameworks.
We illustrate these difficulties in the following two examples.
Example 1.1. We briefly mention the challenges posed by the replace function in JavaScript;
a slightly different but more detailed example can be found in Section 2. Consider the
Javascript code snippet
var namesReg = /([A-Za-z]+) ([A-Za-z]+)/g;
var newAuthorList = authorList.replace(nameReg, "$2, $1");
Assuming authorList is given as a list of ;-separated author names — first name, followed
by a last name — the above program would convert this to last name, followed by first name
format. For instance, "Don Knuth; Alan Turing" would be converted to "Knuth, Don;
Turing, Alan". A natural post condition for this code snippet one would like to check is
the existence of at least one “,” between two occurrences of “;”.
Example 1.2. We consider the match function in JavaScript, in combination with replace.
Consider the code snippet in Figure 1. The function normalize removes leading and trailing
zeros from a decimal string with the input decimal. For instance, normalize("0.250")
== "0.25", normalize("02.50") == "2.5", normalize("025.0") == "25", and finally
normalize("0250") == "250". As the reader might have guessed, the function match
actually returns an array of strings, corresponding to those that are matched in the capturing
groups (two in our example) in the RegEx using the greedy semantics of the Kleene star/plus
operator. One might be interested in checking, for instance, that there is a way to generate
a the string "0.0007", but not the string "00.007".
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

function normalize(decimal) {
const decimalReg = /^(\d+)\.?(\d*)$/;
var
decomp
= decimal.match(decimalReg);
var
result
= "";
if (decomp) {
var integer
= decomp[1].replace(/^0+/, "");
var fractional = decomp[2].replace(/0+$/, "");
if (integer
!== "") result = integer; else result = "0";
if (fractional !== "") result = result + "." + fractional;
}
return result;
}
Fig. 1. Normalize a decimal by removing the leading and trailing zeros

The above examples epitomize the difficulties that have arisen from the interaction between
RegEx and string functions in programs. Firstly, RegEx uses deterministic semantics for
pattern matching (like greedy semantics in the above example, but the so-called lazy
matching is also possible), and allows features that do not exist in regular expressions in
formal language theory, e.g., capturing groups (those in brackets) in the above example.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 4 -->

45:4

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Secondly, string functions in programs can exploit RegEx in an intricate manner, e.g., by
means of references $1 and $2 in Example 1.1. Hitherto, no existing string solvers can support
any of these features. This is despite the fact that idealized versions of regular constraints
and the replace functions are allowed in modern string solvers (e.g. see [Abdulla et al. 2018;
Chen et al. 2019; Holı́k et al. 2018; Liang et al. 2014; Trinh et al. 2016; Yu et al. 2014]),
i.e., features that can be found in the above examples like capturing groups, greedy/lazy
matching, and references are not supported. This limitation of existing string solvers was
already mentioned in the recent paper [Loring et al. 2019].
In view of the aforementioned limitation of string solvers, what solutions are possible? One
recently proposed solution is to map the path constraints generated by string-manipulating
programs that exploit RegEx into constraints in the SMT theories supported by existing
string solvers. In fact, this was done in recent papers [Loring et al. 2019], where the path
constraints are mapped to constraints in the theory of strings with concatenation and
regular constraints in Z3 [de Moura and Bjørner 2008]. Unfortunately, this mapping is an
approximation, since such complex string manipulations are generally inexpressible in any
string theories supported by existing string solvers. To leverage this, CEGAR (counterexample guided abstraction and refinement) is used in [Loring et al. 2019], while ensuring
that an under-approximation is preserved. This results in a rather severe price in both
precision and performance: the refinement process may not terminate even for extremely
simple programs (e.g. the above examples).
Therefore, the current state-of-affairs is unsatisfactory because even the introduction
of very simple RegEx expressions in programs (e.g. the above examples) results in path
constraints that can not be solved by existing symbolic executions in combination with
string solvers. In this paper, we would like to firstly advocate that string solvers should
natively support important features of RegEx in their SMT theories. Existing work (e.g. the
reduction to Z3 provided by [Loring et al. 2019]) shows that this is a monumental theoretical
and programming task, not to mention the loss in precision and the performance penalty.
Secondly, we present the first string theory and string solver that natively provide such a
support.
Contributions. In this paper, we provide the first string theory and string solver that
natively support RegEx. Not only can our theory/solver easily express and solve Example 1.1
and Example 1.2 — which hitherto no existing string solvers and string analysis can handle
— our experiments using a library of 98,117 real-world regular expressions indicate that our
solver substantially outperforms the existing method [Loring et al. 2019] in terms of the
number of solved problems and runtime. We provide more details of our contributions below.
Our string theory provides for the first time a native support of the match and the replace
functions, which use JavaScript1 RegEx in the input arguments. Here is a quick summary of
our string constraint language (see Section 3 for more details):
𝜙

def

=

𝑥 = 𝑦 | 𝑧 = 𝑥 · 𝑦 | 𝑦 = extract𝑖,𝑒 (𝑥) | 𝑦 = replacepat,rep (𝑥) |
𝑦 = replaceAllpat,rep (𝑥) | 𝑥 ∈ 𝑒 | 𝜙 ∧ 𝜙 | 𝜙 ∨ 𝜙 | ¬𝜙

where 𝑒, pat are RegExes, 𝑖 ∈ N, 𝑥, 𝑦, 𝑧 are variables, and rep is called the replacement string
and might refer to strings matched in capturing groups, as in Example 1.1. Apart from the
standard concatenation operator ·, we support extract, which extracts the string matched by
1 JavaScript was chosen because it is relevant to string solving [Hooimeijer et al. 2011; Saxena et al. 2010],

due to vulnerabilities in JavaScripts caused by string manipulations. Our method can be easily adapted to
RegEx semantics in other languages.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 5 -->

Solving String Constraints with Regex-Dependent Functions

45:5

the 𝑖th capturing group in the RegEx 𝑒 (note that match can be simulated by several calls
to extract). We also support replace (resp. replaceAll), which replaces the first occurrence
(resp. all occurrences) of substrings in 𝑥 matched by pat by rep. Our solver/theory also
covers the most important features of RegEx (including greedy/lazy matching, capturing
groups, among others) that make up 74.97% of the RegEx expressions of [Loring et al. 2019]
across 415,487 NPM packages.
A crucial step in the development of our string solver is a formalization of the semantics
of the extract, replace, and replaceAll functions in an automata-theoretic model that is
amenable to analysis (among others, closure properties; see below). To this end, we introduce
a new transducer model called Prioritized Streaming String Transducers (PSSTs), which is
inspired by two automata/transducer models: prioritized finite-state automata [Berglund
and van der Merwe 2017a] and streaming string transducers [Alur and Cerný 2010; Alur and
Deshmukh 2011]. PSSTs allow us to precisely capture the non-standard semantics of RegEx
operators (e.g. greedy/lazy Kleene star) by priorities and deal with capturing groups by
string variables. We show that extract, replace, and replaceAll can all be expressed as PSSTs.
More importantly, we have performed an extensive experiment validating our formalization
against JavaScript semantics.
Next, by means of a sound sequent calculus, our string solver (implemented in the
standard DPLL(T) setting of SMT solvers [Nieuwenhuis et al. 2006]) will exploit crucial
closure and algorithmic properties satisfied by PSSTs. In particular, the solver attempts to
(1) propagate regular constraints (i.e. the constraints 𝑥 ∈ 𝑒) in the formula around by means
of the string functions ·, replace, replaceAll, and extract, and (2) either detect conflicting
regular constraints, or find a satisfiable assignment. A single step of the regular-constraint
propagation computes either the 𝑝𝑜𝑠𝑡-image or the 𝑝𝑟𝑒-image of the above functions. In
particular, it is crucial that each step of our constraint propagation preserves regularity of the
constraints. Since the 𝑝𝑜𝑠𝑡-image does not always preserve regularity, we only propagate by
taking 𝑝𝑜𝑠𝑡-image when regularity is preserved. On the other hand, one of our crucial results
is that taking 𝑝𝑟𝑒-image always preserves regularity: regular constraints are effectively closed
under taking 𝑝𝑟𝑒-image of functions captured in PSSTs. Finally, despite the fact that our
above string theory is undecidable (which follows from [Lin and Barceló 2016]), we show
that our string solving algorithm is guaranteed to terminate (and therefore is also complete)
under the assumption that the input formula syntactically satisfies the so-called straight-line
restriction.
We implement our decision procedure on top of the open-source solver OSTRICH [Chen
et al. 2019], and carry out extensive experiments to evaluate the performance. For the
benchmarks, we generate two collections of JavaScript programs (with 98,117 programs
in each collection), from a library of real-world regular expressions [Davis et al. 2019], by
using two simple JavaScript program templates containing match and replace functions,
respectively. Then we generate all the four (resp. three) path constraints for each match
(resp. replace) JavaScript program and put them into one SMT-LIB script. OSTRICH is
able to answer all four (resp. three) queries in 97% (resp. 91.5%) of the match (resp. replace)
scripts, with the average time 1.57s (resp. 6.62s) per file. Running ExpoSE [Loring et al.
2019] with the same time budget on the same benchmarks, we show that OSTRICH offers
a 8x–18x speedup in comparison to ExpoSE, while being able to cover substantially more
paths (9.6% more for match, 49.9% more for replace), making OSTRICH the first string
solver that is able to handle RegExes precisely and efficiently.

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 6 -->

45:6

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Organization. In Section 2, more details of Example 1.1 are worked out to illustrate our
approach. The string constraint language supporting RegExes is presented in Section 3.
The semantics of the RegEx-dependent string functions are formally defined via PSSTs in
Section 4. The sequent calculus for solving the string constraints is introduced in Section 5.
The implementation of the string solver and experiments are described in Section 6. The
related work is given in Section 7. Finally, Section 8 concludes this paper.
2

A DETAILED EXAMPLE

In this section, we provide a detailed example to illustrate our string solving method.
Consider the JavaScript program in Fig. 2; this example is similar to Example 1.1 from the
Introduction. The function “authorNameDBLPtoACM” in Figure 2 transforms an author
list in the DBLP BibTeX style to the one in the ACM BibTeX style. For instance, if a paper
is authored by Alice M. Brown and John Smith, then the author list in the DBLP BibTeX
style is “Alice M. Brown and John Smith”, while it is “Brown, Alice M. and Smith, John”
in the ACM BibTeX style.
The input of the function “authorNameDBLPtoACM” is authorList, which is expected
to follow the pattern specified by the regular expression autListReg. Intuitively, autListReg
stipulates that authorList joins the strings of full names as a concatenation of a given name,
middle names, and a family name, separated by the blank symbol (denoted by ∖s). Each of
the given, middle, family names is a concatenation of a capital alphabetic letter (denoted
by [A-Z]) followed by a sequence of letters (denoted by ∖w) or a dot symbol (denoted by .).
Between names, the word “and” is used as the separator. The symbols ˆ and $ denote the
beginning and the end of a string input respectively.
function authorNameDBLPtoACM(authorList)
{
var autListReg
= /^[A-Z](\w*|.)(\s[A-Z](\w*|.))*(\sand\s[A-Z](\w*|.)(\s[A-Z](\w*|.))*)*$/;
if (autListReg.test(authorList)) {
var nameReg = /([A-Z](?:\w*|.)(?:\s[A-Z](?:\w*|.))*)(\s[A-Z](?:\w*|.))/g;
return authorList.replace(nameReg, "$2, $1");
}
else return authorList;
}
Fig. 2. Change the author list from the DBLP format to the ACM format

The DBLP name format of each author is specified by the regular expression nameReg in
Fig. 2, which describes the format of a full name.
∙ There are two capturing groups in nameReg, one for recording the concatenation of the
given name and middle names, and the other for recording the family name. Note that
the symbols ?: in (?:∖s[A-Z](?:∖w*—.)) denote the non-capturing groups, i.e. matching
the subexpression, but not remembering the match.
∙ The greedy semantics of the Kleene star * is utilized here to guarantee that the
subexpression (?:∖s[A-Z](?:∖w*—∖.))* matches all the middle names (since there may
exist multiple middle names) and thus nameReg matches the full name. For instance,
the first match of nameReg in “Alice M. Brown and John Smith” should be “Alice M.
Brown”, instead of “Alice M.”. In comparison, if the semantics of * is assumed to be
non-greedy, then (?:∖s[A-Z](?:∖w*—∖.))* can be matched to the empty string, thus
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 7 -->

Solving String Constraints with Regex-Dependent Functions

45:7

nameReg is matched to “Alice M.”, which is not what we want. Therefore, the greedy
semantics of * is essential for the correctness of “authorNameDBLPtoACM”.
∙ The global flag “g” is used in nameReg so that the name format of each author is
transformed.
The name format transformation is via the replace function, i.e. authorList.replace(nameReg,
“$2, $1”), where $1 and $2 refer to the match of the first and second capturing group
respectively.
A natural post-condition of authorNameDBLPtoACM is that there exists at least one
occurrence of the comma symbol between every two occurrences of “and”. This post-condition
has to be established by the function on every execution path. As an example, consider the
path shown in Fig. 3, in which the branches taken in the program are represented as assume
statements. The negated post-condition is enforced by the regular expression in the last
assume. For this path, the post-condition can be proved by showing that the program in
Fig. 3 is infeasible: there does not exist an initial value authorList so that no assumption
fails and the program executes to the end.
1
2
3
4
5
6

var autListReg =
/^[A-Z](\w*|.)(\s[A-Z](\w*|.))*(\sand\s[A-Z](\w*|.)(\s[A-Z](\w*|.))*)*$/;
assume(autListReg.test(authorList));
var nameReg = /([A-Z](?:\w*|.)(?:\s[A-Z](?:\w*|.))*)(\s[A-Z](?:\w*|.))/g;
var result = authorList.replace(nameReg, "$2, $1");
assume(/\sand[^,]*\sand/.test(result));
Fig. 3. Symbolic execution of a path of the JavaScript program in Fig. 2

To enable symbolic execution of the JavaScript programs like in Fig. 3, one needs to model
both the greedy semantics of the Kleene star and store the matches of capturing groups. For
this purpose, we introduce prioritized streaming string transducers (PSST, cf. Section 4) by
which replace(nameReg, “$2, $1”) is represented as a PSST 𝒯 , where the priorities are used
to model the greedy semantics of * and the string variables are used to record the matches
of the capturing groups as well as the return value. Then the symbolic execution of the
program in Fig. 3 can be equivalently turned into the satisfiability of the following string
constraint,
authorList ∈ autListReg ∧ result = 𝒯 (authorList) ∧ result ∈ postConReg,

(1)

where postConReg = /^.*\sand[^,]*\sand.*$/, and autListReg is as in Fig. 2.
Our solver is able to show that (1) is unsatisfiable. On the calculus level (introduced in more
details in Section 5), the main inference step applied for this purpose is the computation
of the pre-image of postConReg under the function 𝒯 ; in other words, we compute the
language of all strings that are mapped to incorrect strings (containing two “and”s without
a comma in between) by 𝒯 . This inference step relies on the fact that the pre-images of
regular languages under PSSTs are regular (see Lemma 5.5). Denoting the pre-image of
postConReg by ℬ, formula (1) is therefore equivalent to
authorList ∈ ℬ ∧ authorList ∈ autListReg ∧ result = 𝒯 (authorList) ∧ result ∈ postConReg. (2)
To show that this formula (and thus (1)) is unsatisfiable, it is now enough to prove that
the languages defined by ℬ and autListReg are disjoint.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 8 -->

45:8

3

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

A STRING CONSTRAINT LANGUAGE NATIVELY SUPPORTING REGEX

In this section, we define a string constraint language natively supporting RegEx. Throughout
the paper, Z+ denotes the set of positive integers, and N denotes the set of natural numbers.
Furthermore, for 𝑛 ∈ Z+ , let [𝑛] := {1, . . . , 𝑛}. We use Σ to denote a finite set of letters,
called alphabet. A string over Σ is a finite sequence of letters from Σ. We use Σ* to denote
the set of strings over Σ, 𝜀 to denote the empty string, and Σ𝜀 to denote Σ ∪ {𝜀}. A string
𝑤′ is called a prefix (resp. suffix ) of 𝑤 if 𝑤 = 𝑤′ 𝑤′′ (resp. 𝑤 = 𝑤′′ 𝑤′ ) for some string 𝑤′′ .
We start with the syntax of RegEx which is essentially that used in JavaScript. (We do
not include backreferences though.)
Definition 3.1 (Regular expressions, RegEx).
def

𝑒 = ∅ | 𝜀 | 𝑎 | (𝑒) | [𝑒 + 𝑒] | [𝑒 · 𝑒] | [𝑒? ] | [𝑒?? ] |

[𝑒* ] | [𝑒*? ] | [𝑒+ ] | [𝑒+? ] | [𝑒{𝑚1 ,𝑚2 } ] | [𝑒{𝑚1 ,𝑚2 }? ]

where 𝑎 ∈ Σ, 𝑛 ∈ Z+ , 𝑚1 , 𝑚2 ∈ N with 𝑚1 ≤ 𝑚2 .

For Γ = {𝑎1 , . . . , 𝑎𝑘 } ⊆ Σ, we write Γ for [[· · · [𝑎1 + 𝑎2 ] + · · · ] + 𝑎𝑘 ] and thus [Γ* ] ≡
[[[· · · [𝑎1 + 𝑎2 ] + · · · ] + 𝑎𝑘 ]* ]. Similarly for [Γ*? ], [Γ+ ], and [Γ+? ]. We write |𝑒| for the length
of 𝑒, i.e., the number of symbols occurring in 𝑒. Note that square brackets [] are used for the
operator precedence and the parentheses () are used for capturing groups.
The operator [𝑒* ] is the greedy Kleene star, meaning that 𝑒 should be matched as many
times as possible. In contrast, the operator [𝑒*? ] is the lazy Kleene star, meaning 𝑒 should
be matched as few times as possible. The Kleene plus operators [𝑒+ ] and [𝑒+? ] are similar to
[𝑒* ] and [𝑒*? ] but 𝑒 should be matched at least once. Moreover, as expected, the repetition
operators [𝑒{𝑚1 ,𝑚2 } ] require the number of times that 𝑒 is matched is between 𝑚1 and 𝑚2
and [𝑒{𝑚1 ,𝑚2 }? ] is the lazy variant. Likewise, the optional operator has greedy and lazy
variants [𝑒? ] and [𝑒?? ], respectively.
For two RegEx 𝑒 and 𝑒′ , we say that 𝑒′ is a subexpression of 𝑒, if one of the following
conditions holds: 1) 𝑒′ = 𝑒, 2) 𝑒 = [𝑒1 · 𝑒2 ] or [𝑒1 + 𝑒2 ], and 𝑒′ is a subexpression of 𝑒1 or 𝑒2 , 3)
{𝑚1 ,𝑚2 }
{𝑚 ,𝑚 }?
+
+?
*
*?
𝑒 = [𝑒?1 ], [𝑒??
], [𝑒1 1 2 ] or (𝑒1 ), and 𝑒′ is a subexpression
1 ], [𝑒1 ], [𝑒1 ], [𝑒1 ], [𝑒1 ], [𝑒1
of 𝑒1 . We use 𝑆(𝑒) to denote the set of subexpressions of 𝑒.
We shall formalize the semantics of RegEx, in particular, for a given regular expression
and an input string, how the string is matched against the regular expression, in Section 4.2.
In the rest of this section, we define the string constraint language STR.
The syntax of STR is defined by the following rules.
𝜙

def

=

𝑥 = 𝑦 | 𝑧 = 𝑥 · 𝑦 | 𝑦 = extract𝑖,𝑒 (𝑥) | 𝑦 = replacepat,rep (𝑥) |
𝑦 = replaceAllpat,rep (𝑥) | 𝑥 ∈ 𝑒 | 𝜙 ∧ 𝜙 | 𝜙 ∨ 𝜙 | ¬𝜙

where
∙ · is the string concatenation operation which concatenates two strings,
∙ 𝑒 ∈RegEx and pat ∈RegEx,
∙ for the extract function, 𝑖 ∈ N,
∙ for the replace and replaceAll operation, rep ∈ REP, where REP is defined as a concatenation of letters from Σ, the references $𝑖 (𝑖 ∈ N), as well as $← and $→ . (Intuitively, $0
denotes the matching of pat, $𝑖 with 𝑖 > 0 denotes the matching of the 𝑖-th capturing
group, $← and $→ denote the prefix before resp. suffix after the matching of pat.)
The extract𝑖,𝑒 (𝑥) function extracts the match of the 𝑖-th capturing group in the successful
match of 𝑒 to 𝑥 for 𝑥 ∈ L (𝑒) (otherwise, the return value of the function is undefined).
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 9 -->

Solving String Constraints with Regex-Dependent Functions

45:9

Note that extract𝑖,𝑒 (𝑥) returns 𝑥 if 𝑖 = 0. Moreover, if the 𝑖-th capturing group of 𝑒 is
not matched, even if 𝑥 ∈ L (𝑒), then extract𝑖,𝑒 (𝑥) returns a special symbol null, denoting
the fact that its value is undefined. For instance, when [[𝑎+ ] + ([𝑎* ])] is matched to the
string 𝑎𝑎, [𝑎+ ], instead of ([𝑎* ]), will be matched, since [𝑎+ ] precedes ([𝑎* ]). Therefore,
extract1,[[𝑎+ ]+([𝑎* ])] (𝑎𝑎) = null.
Remark 1. The match function in programming languages, e.g. str.match(reg) in JavaScript,
finds the first match of reg in str, assuming that reg does not contain the global flag. We can
use extract to express the first match of reg in str by adding [Σ*? ] and [Σ* ] before and after
reg respectively. More generally, the value of the 𝑖-th capturing group in the first match of
a 𝑅𝑒𝑔𝐸𝑥 reg in str can be specified as extract𝑖+1,reg′ (str), where reg′ = [[[Σ*? ] · (reg)] · [Σ* ]].
The other string functions involving regular expressions, e.g. exec and test, without global
flags, are similar to match, thus can be encoded by extract as well.
The function replaceAllpat,rep (𝑥) is parameterized by the pattern pat ∈ 𝑅𝑒𝑔𝐸𝑥 and the
replacement string rep ∈ REP. For an input string 𝑥, it identifies all matches of pat in 𝑥 and
replaces them with strings specified by rep. More specifically, replaceAllpat,rep (𝑥) finds the
first match of pat in 𝑥 and replaces the match with rep, let 𝑥′ be the suffix of 𝑥 after the
first match of pat, then it finds the first match of pat in 𝑥′ and replace the match with rep,
and so on. A reference $𝑖 where 𝑖 > 0 is instantiated by the matching of the 𝑖-th capturing
group. There are three special references2 $0, $← , and $→ . These are instantiated by the
matched text, the text occurring before the match, and the text occurring after the match
respectively. In particular, if the input word is 𝑢𝑣𝑤 where 𝑣 has been matched and will be
replaced, then $0 takes the value 𝑣, $← takes the value 𝑢, and $→ takes the value 𝑤. When
there are multiple matches in a replaceAll, the values of $← and $→ are always with respect
to the original input string 𝑥.
The replacepat,rep (𝑥) function is similar to replaceAllpat,rep (𝑥), except that it replaces only
the first (leftmost) match of pat.
A STR formula 𝜙 is said to be straight-line, if 1) it contains neither negation nor disjunction,
2) the equations in 𝜙 can be ordered into a sequence, say 𝑥1 = 𝑡1 , . . . , 𝑥𝑛 = 𝑡𝑛 , such that
𝑥1 , . . . , 𝑥𝑛 are mutually distinct, moreover, for each 𝑖 ∈ [𝑛], 𝑥𝑖 does not occur in 𝑡1 , . . . , 𝑡𝑖−1 .
Let STRSL denote the set of straight-line STR formulas.
As a crucial step for solving the string constraints in STR, we shall define the formal
semantics of the extract, replace, and replaceAll functions in the next section.
4

SEMANTICS OF STRING FUNCTIONS VIA PSST

Our goal in this section is to define the formal semantics of the string functions involving
RegEx used in STR, that is, extract, replace and replaceAll. To this end, we need to first
define the semantics of RegEx-string matching. One of the key novelties here is to utilize
an extension of finite-state automata with transition priorities and string variables, called
prioritized streaming string transducers (abbreviated as PSST). It turns out that PSST
provides a convenient means to capture the non-standard semantics of RegEx operators
and to store the matches of capturing groups in RegEx, which paves the way to define the
semantics of string functions (and the string constraint language).

2 The corresponding syntax for $0, $← and $→ in JavaScript are $&, $‘, and $′ .

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 10 -->

45:10

4.1

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Prioritized streaming string transducers (PSST)

PSSTs can be seen as an extension of finite-state automata with transition priorities and
string variables. We first recall the definition of classic finite-state automata.
Definition 4.1 (Finite-state Automata). A (nondeterministic) finite-state automaton (FA)
over a finite alphabet Σ is a tuple 𝒜 = (Σ, 𝑄, 𝑞0 , 𝐹, 𝛿) where 𝑄 is a finite set of states, 𝑞0 ∈ 𝑄
is the initial state, 𝐹 ⊆ 𝑄 is a set of final states, and 𝛿 ⊆ 𝑄 × Σ𝜀 × 𝑄 is the transition
relation.
For an input string 𝑤, a run of 𝒜 on 𝑤 is a sequence 𝑞0 𝑎1 𝑞1 . . . 𝑎𝑛 𝑞𝑛 such that 𝑤 = 𝑎1 · · · 𝑎𝑛
and (𝑞𝑗−1 , 𝑎𝑗 , 𝑞𝑗 ) ∈ 𝛿 for every 𝑗 ∈ [𝑛]. The run is said to be accepting if 𝑞𝑛 ∈ 𝐹 . A string 𝑤
is accepted by 𝒜 if there is an accepting run of 𝒜 on 𝑤. The set of strings accepted by 𝒜,
i.e., the language recognized by 𝒜, is denoted by L (𝒜). The size |𝒜| of 𝒜 is the cardinality
of 𝛿, the set of transitions. ⋃︀
For a finite set 𝑄, let 𝑄 = 𝑛∈N {(𝑞1 , . . . , 𝑞𝑛 ) | ∀𝑖 ∈ [𝑛], 𝑞𝑖 ∈ 𝑄∧∀𝑖, 𝑗 ∈ [𝑛], 𝑖 ̸= 𝑗 → 𝑞𝑖 ̸= 𝑞𝑗 }.
Intuitively, 𝑄 is the set of sequences of non-repetitive elements from 𝑄. In particular, the
empty sequence () ∈ 𝑄. Note that the length of each sequence from 𝑄 is bounded by |𝑄|.
For a sequence 𝑃 = (𝑞1 , . . . , 𝑞𝑛 ) ∈ 𝑄 and 𝑞 ∈ 𝑄, we write 𝑞 ∈ 𝑃 if 𝑞 = 𝑞𝑖 for some 𝑖 ∈ [𝑛].
Moreover, for 𝑃1 = (𝑞1 , . . . , 𝑞𝑚 ) ∈ 𝑄 and 𝑃2 = (𝑞1′ , . . . , 𝑞𝑛′ ) ∈ 𝑄, we say 𝑃1 ∩ 𝑃2 = ∅ if
{𝑞1 , . . . , 𝑞𝑚 } ∩ {𝑞1′ , . . . , 𝑞𝑛′ } = ∅.
Definition 4.2 (Prioritized Streaming String Transducers). A prioritized streaming string
transducer (PSST) is a tuple 𝒯 = (𝑄, Σ, 𝑋, 𝛿, 𝜏, 𝐸, 𝑞0 , 𝐹 ), where
∙ 𝑄 is a finite set of states,
∙ Σ is the input and output alphabet,
∙ 𝑋 is a finite set of string variables,
∙ 𝛿 ∈ 𝑄 × Σ → 𝑄 defines the non-𝜀 transitions as well as their priorities (from highest
to lowest),
∙ 𝜏 ∈ 𝑄 → 𝑄 × 𝑄 such that for every 𝑞 ∈ 𝑄, if 𝜏 (𝑞) = (𝑃1 ; 𝑃2 ), then 𝑃1 ∩ 𝑃2 = ∅,
(Intuitively, 𝜏 (𝑞) = (𝑃1 ; 𝑃2 ) specifies the 𝜀-transitions at 𝑞, with the intuition that the
𝜀-transitions to the states in 𝑃1 (resp. 𝑃2 ) have higher (resp. lower) priorities than the
non-𝜀-transitions out of 𝑞.)
∙ 𝐸 associates with each transition a string-variable assignment function, i.e., 𝐸 is partial
function from 𝑄 × Σ𝜀 × 𝑄 to 𝑋 → (𝑋 ∪ Σ)* such that its domain is the set of tuples
(𝑞, 𝑎, 𝑞 ′ ) satisfying that either 𝑎 ∈ Σ and 𝑞 ′ ∈ 𝛿(𝑞, 𝑎) or 𝑎 = 𝜀 and 𝑞 ′ ∈ 𝜏 (𝑞),
∙ 𝑞0 ∈ 𝑄 is the initial state, and
∙ 𝐹 is the output function, which is a partial function from 𝑄 to (𝑋 ∪ Σ)* .
For 𝜏 (𝑞) = (𝑃1 ; 𝑃2 ), we will use 𝜋1 (𝜏 (𝑞)) and 𝜋2 (𝜏∑︀
(𝑞)) to denote
𝑃1 and 𝑃2 respec∑︀
tively. The size of 𝒯 , denoted by |𝒯 |, is defined as
|𝐸((𝑞, 𝑎, 𝑞 ′ ))(𝑥)|, where
(𝑞,𝑎,𝑞 ′ )∈dom(𝐸) 𝑥∈𝑋

|𝐸((𝑞, 𝑎, 𝑞 ′ ))(𝑥)| is the length of 𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥), i.e., the number of symbols from 𝑋 ∪ Σ in
it. A PSST 𝒯 is said to be copyless if for each transition (𝑞, 𝑎, 𝑞 ′ ) in 𝒯 and each 𝑥 ∈ 𝑋,
𝑥 occurs in (𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥′ ))𝑥′ ∈𝑋 at most once. A PSST 𝒯 is said to be copyful if it is not
copyless. For instance, if 𝑋 = {𝑥1 , 𝑥2 } and 𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥1 ) = 𝑥1 and 𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥2 ) = 𝑥1 𝑎 for
some transition (𝑞, 𝑎, 𝑞 ′ ), then 𝑥1 occurs twice in (𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥′ ))𝑥′ ∈𝑋 , thus 𝒯 is copyful.
A run of 𝒯 on a string 𝑤 is a sequence 𝑞0 𝑎1 𝑠1 𝑞1 . . . 𝑎𝑚 𝑠𝑚 𝑞𝑚 such that
∙ for each 𝑖 ∈ [𝑚],
– either 𝑎𝑖 ∈ Σ, 𝑞𝑖 ∈ 𝛿(𝑞𝑖−1 , 𝑎𝑖 ), and 𝑠𝑖 = 𝐸(𝑞𝑖−1 , 𝑎𝑖 , 𝑞𝑖 ),
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 11 -->

Solving String Constraints with Regex-Dependent Functions

45:11

– or 𝑎𝑖 = 𝜀, 𝑞𝑖 ∈ 𝜏 (𝑞𝑖−1 ) and 𝑠𝑖 = 𝐸(𝑞𝑖−1 , 𝜀, 𝑞𝑖 ),
∙ for every subsequence 𝑞𝑖 𝑎𝑖+1 𝑠𝑖+1 𝑞𝑖+1 . . . 𝑎𝑗 𝑠𝑗 𝑞𝑗 such that 𝑖 < 𝑗 and 𝑎𝑖+1 = · · · = 𝑎𝑗 = 𝜀,
it holds that each 𝜀-transition occurs at most once in it, namely, for every 𝑘, 𝑙 : 𝑖 ≤
𝑘 < 𝑙 < 𝑗, (𝑞𝑘 , 𝑞𝑘+1 ) ̸= (𝑞𝑙 , 𝑞𝑙+1 ).
Note that it is possible that 𝛿(𝑞, 𝑎) = (), that is, there is no 𝑎-transition out of 𝑞. From the
assumption that each 𝜀-transition occurs at most once in a sequence of 𝜀-transitions, we
deduce that given a string 𝑤, the length of a run of 𝒯 on 𝑤, i.e. the number of transitions
in it, is 𝑂(|𝑤||𝒯 |).
For any pair of runs 𝑅 = 𝑞0 𝑎1 𝑠1 . . . 𝑎𝑚 𝑠𝑚 𝑞𝑚 and 𝑅′ = 𝑞0 𝑎′1 𝑠′1 . . . 𝑎′𝑛 𝑠′𝑛 𝑞𝑛′ such that
𝑎1 . . . 𝑎𝑚 = 𝑎′1 . . . 𝑎′𝑛 , we say that 𝑅 is of a higher priority over 𝑅′ if
∙ either 𝑅′ is a prefix of 𝑅 (in this case, the transitions of 𝑅 after 𝑅′ are all 𝜀-transitions),
∙ or there is an index 𝑗 satisfying one of the following constraints:
′
– 𝑞0 𝑎1 𝑞1 . . . 𝑞𝑗−1 𝑎𝑗 = 𝑞0 𝑎′1 𝑞1′ . . . 𝑞𝑗−1
𝑎′𝑗 , 𝑞𝑗 ̸= 𝑞𝑗′ , 𝑎𝑗 ∈ Σ, and we have that 𝛿(𝑞𝑗−1 , 𝑎𝑗 ) =
(. . . , 𝑞𝑗 , . . . , 𝑞𝑗′ , . . .),
′
– 𝑞0 𝑎1 𝑞1 . . . 𝑞𝑗−1 𝑎𝑗 = 𝑞0 𝑎′1 𝑞1′ . . . 𝑞𝑗−1
𝑎′𝑗 , 𝑞𝑗 ̸= 𝑞𝑗′ , 𝑎𝑗 = 𝜀, and one of the following holds:
′
(i) 𝜋1 (𝜏 (𝑞𝑗−1 )) = (. . . , 𝑞𝑗 , . . . , 𝑞𝑗 , . . .), (ii) 𝜋2 (𝜏 (𝑞𝑗−1 )) = (. . . , 𝑞𝑗 , . . . , 𝑞𝑗′ , . . .), or (iii)
𝑞𝑗 ∈ 𝜋1 (𝜏 (𝑞𝑗−1 )) and 𝑞𝑗′ ∈ 𝜋2 (𝜏 (𝑞𝑗−1 )),
′
– 𝑞0 𝑎1 𝑞1 . . . 𝑞𝑗−1 = 𝑞0 𝑎′1 𝑞1′ . . . 𝑞𝑗−1
, 𝑎𝑗 = 𝜀, 𝑎′𝑗 ∈ Σ, 𝑞𝑗 ∈ 𝜋1 (𝜏 (𝑞𝑗−1 )), and 𝑞𝑗′ ∈
′
𝛿(𝑞𝑗−1 , 𝑎𝑗 ),
′
– 𝑞0 𝑎1 𝑞1 . . . 𝑞𝑗−1 = 𝑞0 𝑎′1 𝑞1′ . . . 𝑞𝑗−1
, 𝑎𝑗 ∈ Σ, 𝑎′𝑗 = 𝜀, 𝑞𝑗 ∈ 𝛿(𝑞𝑗−1 , 𝑎𝑗 ), and 𝑞𝑗′ ∈
𝜋2 (𝜏 (𝑞𝑗−1 )).
An accepting run of 𝒯 on 𝑤 is a run of 𝒯 on 𝑤, say 𝑅 = 𝑞0 𝑎1 𝑠1 . . . 𝑎𝑚 𝑠𝑚 𝑞𝑚 , such that 1)
𝐹 (𝑞𝑚 ) is defined, 2) 𝑅 is of the highest priority among those runs satisfying 1). The output
of 𝒯 on 𝑤, denoted by 𝒯 (𝑤), is defined as 𝜂𝑚 (𝐹 (𝑞𝑚 )), where 𝜂0 (𝑥) = 𝜀 for each 𝑥 ∈ 𝑋, and
𝜂𝑖 (𝑥) = 𝜂𝑖−1 (𝑠𝑖 (𝑥)) for every 1 ≤ 𝑖 ≤ 𝑚 and 𝑥 ∈ 𝑋. Note that here we abuse the notation
𝜂𝑚 (𝐹 (𝑞𝑚 )) and 𝜂𝑖−1 (𝑠𝑖 (𝑥)) by taking a function 𝜂 from 𝑋 to Σ* as a function from (𝑋 ∪ Σ)*
to Σ* , which maps each 𝑥 ∈ 𝑋 to 𝜂(𝑥) and each 𝑎 ∈ Σ to 𝑎. If there is no accepting run
of 𝒯 on 𝑤, then 𝒯 (𝑤) = ⊥, that is, the output of 𝒯 on 𝑤 is undefined. The string relation
defined by 𝒯 , denoted by ℛ𝒯 , is {(𝑤, 𝒯 (𝑤)) | 𝑤 ∈ Σ* , 𝒯 (𝑤) ̸= ⊥}.
Example 4.3. The PSST 𝒯 = (𝑄, Σ, 𝑋, 𝛿, 𝜏, 𝐸, 𝑞0 , 𝐹 ) to extract the match of the first
capturing group for the regular expression (\d+)(\d*) is illustrated in Fig. 4, where 𝑥1
and 𝑥2 store the matches of the two capturing groups. More specifically, in 𝒯 we have
Σ = {0, · · · , 9}, 𝑋 = {𝑥1 , 𝑥2 }, 𝐹 (𝑞4 ) = 𝑥1 denotes the final output, and 𝛿, 𝜏, 𝐸 are illustrated
in Fig. 4, where the dashed edges denote the 𝜀-transitions of lower priorities than the non𝜀-transitions and the symbol ℓ denotes the currently scanned input letter. For instance,
for the state 𝑞2 , 𝛿(𝑞2 , ℓ) = (𝑞2 ) for ℓ ∈ {0, . . . , 9}, 𝜏 (𝑞2 ) = ((); (𝑞3 )), 𝐸(𝑞2 , ℓ, 𝑞2 )(𝑥1 ) = 𝑥1 ℓ,
𝐸(𝑞2 , ℓ, 𝑞2 )(𝑥2 ) = 𝑥2 , 𝐸(𝑞2 , 𝜀, 𝑞3 )(𝑥1 ) = 𝑥1 , and 𝐸(𝑞2 , 𝜀, 𝑞3 )(𝑥2 ) = 𝜀. Note that the identity
assignments, e.g. 𝐸(𝑞2 , 𝜀, 𝑞3 )(𝑥1 ) = 𝑥1 , are omitted in Fig. 4 for readability. For the input
string 𝑤=“2050”, the accepting run of 𝒯 on 𝑤 is
𝜀

2

0

5

0

𝜀

𝑥1 :=𝜀

𝑥1 :=𝑥1 2

𝑥1 :=𝑥1 0

𝑥1 :=𝑥1 5

𝑥1 :=𝑥1 0

𝑥2 :=𝜀

𝜀

𝑞0 −−−−→ 𝑞1 −−−−−→ 𝑞2 −−−−−→ 𝑞2 −−−−−→ 𝑞2 −−−−−→ 𝑞2 −−−−→ 𝑞3 −
→ 𝑞4 ,
where the value of 𝑥1 and 𝑥2 when reaching the state 𝑞4 are “2050” and 𝜀 respectively.
4.2

Semantics of RegEx-String Matching

We now define the formal semantics of RegEx. Traditionally they are interpreted as a regular
language which can be defined inductively. In our case, where RegEx are mainly used in
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 12 -->

45:12

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu
` ∈ {0, · · · , 9}
x1 := x1 `

` ∈ {0, · · · , 9}
x2 := x2 `
F (q4 ) = x1

q0

ε
x1 := ε

` ∈ {0, · · · , 9}
q1
x1 := x1 `

q2

ε
x2 := ε

q3

ε

q4

Fig. 4. The PSST 𝒯 : Extract the matching of the first capturing group in (∖d+)(∖d*)

string functions, what matters is the intermediate result when parsing a string against
the given RegEx. As a result, we shall present an operational (as opposed to traditional
denotational) account of the RegEx-string matching by constructing PSSTs out of regular
expressions.
Note that in [Berglund et al. 2014; Berglund and van der Merwe 2017a], a construction
from RegEx to prioritized finite transducers (PFT) was given. The construction therein is a
variant of the classical Thompson construction from regular expressions to nondeterministic
finite automata [Thompson 1968]. In particular, the size of the constructed PFT is linear in
the size of the given RegEx. One may be tempted to think that the construction in [Berglund
et al. 2014; Berglund and van der Merwe 2017a] can be easily adapted to construct PSSTs
out of regular expressions. Nevertheless, the construction in [Berglund et al. 2014; Berglund
and van der Merwe 2017a] does not work for so called problematic regular expressions, i.e.,
those regular expressions that contain the subexpressions 𝑒* or 𝑒*? with 𝜀 ∈ L (𝑒). Moreover,
{𝑚 ,𝑚 }
{𝑚 ,𝑚 }?
the construction therein did not consider the repetition operators [𝑒1 1 2 ] or [𝑒1 1 2 ].
Our construction, which is considerably different from that in [Berglund et al. 2014; Berglund
and van der Merwe 2017a], works for arbitrary regular expressions. In particular, the size
of the constructed PSST can be exponential in the size of the given regular expression in
the worst case. Moreover, we validate by extensive experiments that our construction is
consistent with the actual RegEx-string matching in JavaScript.
For technical convenience, we assume that 𝐹 in a PSST is a set of final states, instead
of an output function, in the sequel. The main idea of the construction is to split the
set of final states, 𝐹 , into two disjoint subsets 𝐹1 and 𝐹2 , with the intention that 𝐹1 and
𝐹2 are responsible for accepting the empty string resp. non-empty strings. Therefore, the
PSSTs constructed below are of the form (𝑄, Σ, 𝑋, 𝛿, 𝜏, 𝐸, 𝑞0 , (𝐹1 , 𝐹2 )). The necessity of this
splitting will be illustrated in Example 4.6.
Furthermore, to deal with the situation that some capturing group may not be matched
to any string and its value is undefined, we introduce a special symbol null and assume that
the initial values of all the string variables are null. For simplicity, in the definition of a
PSST, if 𝛿(𝑞, 𝑎, 𝑞 ′ ) = () or 𝜏 (𝑞, 𝜀, 𝑞 ′ ) = ((); ()), they will not be stated explicitly. Moreover,
we will omit all the assignments 𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥) such that 𝐸(𝑞, 𝑎, 𝑞 ′ )(𝑥) = 𝑥.
For PSSTs of the form (𝑄, Σ, 𝑋, 𝛿, 𝜏, 𝐸, 𝑞0 , (𝐹1 , 𝐹2 )), we introduce a notation to be used
in the construction, namely, the concatenation of two PSSTs.
Definition 4.4 (Concatenation of two PSSTs). For 𝑖 ∈ {1, 2}, let 𝒯𝑖 be a PSST such that
𝒯𝑖 = (𝑄𝑖 , Σ, 𝑋𝑖 , 𝛿𝑖 , 𝜏𝑖 , 𝐸𝑖 , 𝑞𝑖,0 , (𝐹𝑖,1 , 𝐹𝑖,2 )). Then the concatenation of 𝒯1 and 𝒯2 , denoted by
′
′
′
𝒯1 · 𝒯2 , is defined as follows (see Fig. 5): Let 𝒯2′ = (𝑄′2 , Σ, 𝑋2 , 𝛿2′ , 𝜏2′ , 𝐸2′ , 𝑞2,0
, (𝐹2,1
, 𝐹2,2
)) be
a fresh copy of 𝒯2 , but with the string variables of 𝒯2 kept unchanged. Then
′
′
𝒯 = (𝑄1 ∪ 𝑄2 ∪ 𝑄′2 , Σ, 𝑋1 ∪ 𝑋2 , 𝛿, 𝜏, 𝑞1,0 , (𝐹2,1 , 𝐹2,2 ∪ 𝐹2,1
∪ 𝐹2,2
))

where
∙ 𝛿 comprises the transitions in 𝛿1 , 𝛿2 , and 𝛿2′ ,

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 13 -->

Solving String Constraints with Regex-Dependent Functions

45:13

∙ 𝜏 comprises the transitions in 𝜏1 , 𝜏2 , 𝜏2′ , and the following transitions,
– for every 𝑓1,1 ∈ 𝐹1,1 , 𝜏 (𝑓1,1 ) = ((𝑞2,0 ); ()),
′
– for every 𝑓1,2 ∈ 𝐹1,2 , 𝜏 (𝑓1,2 ) = ((𝑞2,0
), ()),
∙ 𝐸 inherits all the assignments in 𝐸1 , 𝐸2 , and 𝐸2′ , and includes the following assignments: for every 𝑓1,1 ∈ 𝐹1,1 , 𝑓1,2 ∈ 𝐹1,2 , and 𝑥′ ∈ 𝑋2 , 𝐸(𝑓1,1 , 𝜀, 𝑞2,0 )(𝑥′ ) =
′
𝐸(𝑓1,2 , 𝜀, 𝑞2,0
)(𝑥′ ) = null. (Intuitively, the values of all the variables in 𝑋2 are reset
when entering 𝒯2 and 𝒯2′ .)
T1 · T 2
q1,0
···
f1,1 ∈ F1,1
ε

∀x0 ∈ X2 ,
x0 := null

f1,2 ∈ F1,2

ε

q2,0

0
q2,0

···
f2,1 ∈ F2,1

F1

∀x0 ∈ X2 ,
x0 := null

f2,2 ∈ F2,2

···
0
0
f2,1
∈ F2,1

0
0
f2,2
∈ F2,2

F2

Fig. 5. 𝒯1 · 𝒯2 : Concatenation of 𝒯1 and 𝒯2

Note that in the above definition, it is possible that 𝑋1 ∩ 𝑋2 ̸= ∅. We remark that
if 𝐹1,1 = ∅ or 𝐹2,1 = ∅, then one copy of 𝒯2 , instead of two copies, is sufficient for the
concatenation.
We shall recursively construct a PSST 𝒯𝑒 for each RegEx 𝑒, such that the initial state has
no incoming transitions and each of its final states has no outgoing transitions. Moreover,
all the transitions out of the initial state are 𝜀-transitions. We assume that in 𝒯𝑒 , a string
variable 𝑥𝑒′ is introduced for each subexpression 𝑒′ of 𝑒.
The construction is technical and below we only select to present some representative
cases. The other cases are given in the long version of this paper [Chen et al. 2021].
Case 𝑒 = (𝑒1 ). 𝒯𝑒 is adapted from 𝒯𝑒1 = (𝑄𝑒1 , Σ, 𝑋𝑒1 , 𝛿𝑒1 , 𝜏𝑒1 , 𝐸𝑒1 , 𝑞𝑒1 ,0 , (𝐹𝑒1 ,1 , 𝐹𝑒1 ,2 )) by
adding the string variable 𝑥𝑒 and the assignments for 𝑥𝑒 , that is, 𝑋𝑒 = 𝑋𝑒1 ∪ {𝑥𝑒 } and for
each transition (𝑞, 𝑎, 𝑞 ′ ) in 𝒯𝑒1 with 𝑎 ∈ Σ𝜀 , we have 𝐸𝑒 (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒 ) = 𝐸𝑒1 (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒1 ).
Case 𝑒 = [𝑒1 +𝑒2 ] (see Fig. 6). For 𝑖 ∈ {1, 2}, let 𝒯𝑒𝑖 = (𝑄𝑒𝑖 , Σ, 𝑋𝑒𝑖 , 𝛿𝑒𝑖 , 𝜏𝑒𝑖 , 𝐸𝑒𝑖 , 𝑞𝑒𝑖 ,0 , (𝐹𝑒𝑖 ,1 , 𝐹𝑒𝑖 ,2 )).
Moreover, assume 𝑋𝑒1 ∩ 𝑋𝑒2 = ∅. Then
𝒯𝑒 = (𝑄𝑒1 ∪ 𝑄𝑒2 ∪ {𝑞𝑒,0 }, Σ, 𝑋𝑒1 ∪ 𝑋𝑒2 ∪ {𝑥𝑒 }, 𝛿𝑒 , 𝜏𝑒 , 𝐸𝑒 , 𝑞𝑒,0 , (𝐹𝑒1 ,1 ∪ 𝐹𝑒2 ,1 , 𝐹𝑒1 ,2 ∪ 𝐹𝑒2 ,2 ))

where
∙ 𝛿𝑒 comprises the transitions in 𝛿𝑒1 and 𝛿𝑒2 ,
∙ 𝜏𝑒 comprises the transitions in 𝜏𝑒1 and 𝜏𝑒2 , as well as the transition 𝜏𝑒 (𝑞𝑒,0 ) =
((𝑞𝑒1 ,0 ); (𝑞𝑒2 ,0 )),
∙ 𝐸𝑒 inherits 𝐸𝑒1 , 𝐸𝑒2 , plus the assignments 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑞𝑒1 ,0 )(𝑥𝑒 ) = 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑞𝑒2 ,0 )(𝑥𝑒 ) =
𝜀, as well as 𝐸𝑒 (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒 ) = 𝑥𝑒 𝑎 for every transition (𝑞, 𝑎, 𝑞 ′ ) in 𝒯𝑒1 and 𝒯𝑒2 (where
𝑎 ∈ Σ𝜀 ).
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 14 -->

45:14

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

T[e1 +e2 ]
qe,0
ε

ε
xe := ε

xe := ε

qe1 ,0

qe2 ,0

···

···

fe1 ,1 ∈ Fe1 ,1

fe1 ,2 ∈ Fe1 ,2

fe2 ,1 ∈ Fe2 ,1

Fe,1

Fe,2

fe2 ,2 ∈ Fe2 ,2

Fig. 6. The PSST 𝒯[𝑒1 +𝑒2 ]

Case 𝑒 = [𝑒1 · 𝑒2 ]. For 𝑖 ∈ {1, 2}, let 𝒯𝑒𝑖 = (𝑄𝑒𝑖 , Σ, 𝑋𝑒𝑖 , 𝛿𝑒𝑖 , 𝜏𝑒𝑖 , 𝐸𝑒𝑖 , 𝑞𝑒𝑖 ,0 , (𝐹𝑒𝑖 ,1 , 𝐹𝑒𝑖 ,2 )).
Moreover, let us assume that 𝑋𝑒1 ∩ 𝑋𝑒2 = ∅. Then 𝒯𝑒 is obtained from 𝒯𝑒1 · 𝒯𝑒2 (the concatenation of 𝒯𝑒1 and 𝒯𝑒2 , see Fig. 5) by adding a string variable 𝑥𝑒 , a fresh state 𝑞𝑒,0 as the initial
state, the 𝜀-transition 𝜏𝑒 (𝑞𝑒,0 ) = ((𝑞𝑒1 ,0 ); ()), and the assignments 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑞𝑒1 ,0 )(𝑥𝑒 ) = 𝜀,
𝐸𝑒 (𝑝, 𝑎, 𝑞)(𝑥𝑒 ) = 𝑥𝑒 𝑎 for every transition (𝑝, 𝑎, 𝑞) in 𝒯𝑒1 , 𝒯𝑒2 , and 𝒯𝑒′2 (where 𝑎 ∈ Σ𝜀 ).
Case 𝑒 = [𝑒?1 ] (see Fig. 7). Let 𝒯𝑒1 = (𝑄𝑒1 , Σ, 𝑋𝑒1 , 𝛿𝑒1 , 𝜏𝑒1 , 𝐸𝑒1 , 𝑞𝑒1 ,0 , (𝐹𝑒1 ,1 , 𝐹𝑒1 ,2 )). Then
𝒯𝑒 = (𝑄𝑒1 ∪ {𝑞𝑒,0 , 𝑓𝜀 }, Σ, 𝑋𝑒1 ∪ {𝑥𝑒 }, 𝛿𝑒 , 𝜏𝑒 , 𝐸𝑒 , 𝑞𝑒,0 , ({𝑓𝜀 }, 𝐹𝑒1 ,2 ))
where
∙ 𝛿𝑒 is exactly 𝛿𝑒1 ,
∙ 𝜏𝑒 comprises the transitions in 𝜏𝑒1 , as well as the transition 𝜏𝑒 (𝑞𝑒,0 ) = ((𝑞𝑒1 ,0 , 𝑓𝜀 ); ()),
∙ 𝐸𝑒 inherits 𝐸𝑒1 and includes the assignments 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑞𝑒1 ,0 )(𝑥𝑒 ) = 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑓𝜀 )(𝑥𝑒 ) =
𝜀, as well as 𝐸𝑒 (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒 ) = 𝑥𝑒 𝑎 for every transition (𝑞, 𝑎, 𝑞 ′ ) in 𝒯𝑒1 (where 𝑎 ∈ Σ𝜀 ).

Note that 𝐹𝑒1 ,1 is not included into 𝐹𝑒,1 here.

T[e?? ]

T[e? ]
qe,0
ε

T e1
fe1 ,1 ∈ Fe1 ,1

xe := ε

qe,0
ε

xe := ε

ε

ε

fε

qe1 ,0

xe := ε

xe := ε

T e1

···

fε

qe1 ,0
···

Fe,1

fe1 ,2 ∈ Fe1 ,2

fe1 ,2 ∈ Fe1 ,2

fe1 ,1 ∈ Fe1 ,1

Fe,2

Fe,1

Fe,2

Fig. 7. The PSST 𝒯[𝑒? ] and 𝒯[𝑒?? ]
1

1

Case 𝑒 = [𝑒??
is almost the same as 𝒯[𝑒?1 ] . The only
1 ] (see Fig. 7). In this case, 𝒯[𝑒??
1 ]
difference is that the priorities of the two 𝜀-transitions out of 𝑞𝑒,0 are swapped, namely,
𝜏𝑒 (𝑞𝑒,0 ) = ((𝑓𝜀 , 𝑞𝑒1 ,0 ); ()) here.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 15 -->

Solving String Constraints with Regex-Dependent Functions

45:15

Case 𝑒 = [𝑒*1 ] (see Fig. 8). Let 𝒯𝑒1 = (𝑄𝑒1 , Σ, 𝑋𝑒1 , 𝛿𝑒1 , 𝜏𝑒1 , 𝐸𝑒1 , 𝑞𝑒1 ,0 , (𝐹𝑒1 ,1 , 𝐹𝑒1 ,2 )). Then
𝒯𝑒 = (𝑄𝑒1 ∪ {𝑞𝑒,0 , 𝑓𝑒,1 , 𝑓𝑒,2 }, Σ, 𝑋𝑒 , 𝛿𝑒 , 𝐸𝑒 , 𝜏𝑒 , 𝑞𝑒,0 , ({𝑓𝑒,1 }, {𝑓𝑒,2 }))
where
∙ 𝛿𝑒 is exactly 𝛿𝑒1 ,
∙ 𝜏𝑒 comprises the transitions in 𝜏𝑒1 , as well as the transitions 𝜏𝑒 (𝑞𝑒,0 ) = ((𝑞𝑒1 ,0 , 𝑓𝑒,1 ); ()),
𝜏𝑒 (𝑓𝑒1 ,1 ) = ((𝑞𝑒1 ,0 ); ()) for every 𝑓𝑒1 ,1 ∈ 𝐹𝑒1 ,1 , and 𝜏𝑒 (𝑓𝑒1 ,2 ) = ((𝑞𝑒1 ,0 , 𝑓𝑒,2 ); ()) for every
𝑓𝑒1 ,2 ∈ 𝐹𝑒1 ,2 ,
∙ 𝐸𝑒 inherits 𝐸𝑒1 , and includes the assignments 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑓𝑒,1 )(𝑥𝑒 ) = 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑞𝑒1 ,0 )(𝑥𝑒 ) =
𝜀, 𝐸𝑒 (𝑓𝑒1 ,1 , 𝜀, 𝑞𝑒1 ,0 )(𝑥) = 𝐸𝑒 (𝑓𝑒1 ,2 , 𝜀, 𝑞𝑒1 ,0 )(𝑥) = null for every 𝑓𝑒1 ,1 ∈ 𝐹𝑒1 ,1 , 𝑓𝑒1 ,2 ∈
𝐹𝑒1 ,2 , and 𝑥 ∈ 𝑋𝑒1 , as well as 𝐸𝑒 (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒 ) = 𝑥𝑒 𝑎 for every transition (𝑞, 𝑎, 𝑞 ′ ) in 𝒯𝑒1
with 𝑎 ∈ Σ𝜀 . (Intuitively, the values of all the string variables in 𝑋𝑒1 are reset when
starting a new iteration of 𝑒1 .)
T[e∗?
1 ]

T[e∗1 ]
qe,0
ε

qe,0
ε

ε xe := ε

xe := ε

ε xe := ε

xe := ε

fe,1

fe,1
ε

qe1 ,0
∀x ∈ Xe1 ,
x := null

fe1 ,1 ∈ Fe1 ,1

ε

∀x ∈ Xe1 ,
x := null

ε

···
fe1 ,2 ∈ Fe1 ,2

xe := xe a

qe1 ,0
∀x ∈ Xe1 ,
x := null

fe1 ,1 ∈ Fe1 ,1

xe := xe a

ε
fe,2

∀x ∈ X ,

e1
ε
x := null
···
fe1 ,2 ∈ Fe1 ,2

ε
fe,2

Fig. 8. The PSST 𝒯[𝑒*1 ] and 𝒯[𝑒*? ]
1

*
Case 𝑒 = [𝑒*?
1 ] (see Fig. 8). The construction is almost the same as 𝑒 = [𝑒1 ]. The only
difference is that the priorities of the 𝜀-transitions out of 𝑞𝑒,0 resp. 𝑓𝑒1 ,2 ∈ 𝐹𝑒1 ,2 are swapped.

−
−
Case 𝑒 = [𝑒+
by
1 ]. We first construct 𝒯𝑒1 and 𝒯[𝑒* ] , where 𝒯[𝑒* ] is obtained from 𝒯[𝑒*
1]
1

1

dropping the string variable 𝑥[𝑒*1 ] . Therefore, 𝒯𝑒1 and 𝒯[𝑒−* ] have the same set of string variables,
1

𝑋𝑒1 . Then we construct 𝒯𝑒 by adding into 𝒯𝑒1 · 𝒯[𝑒−* ] a fresh state 𝑞𝑒,0 as the initial state, and
1
the transitions 𝜏𝑒 (𝑞𝑒,0 ) = ((𝑞𝑒1 ,0 ); ()), as well as the assignments 𝐸𝑒 (𝑞𝑒,0 , 𝜀, 𝑞𝑒1 ,0 )(𝑥𝑒 ) = 𝜀,
𝐸𝑒 (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒 ) = 𝑥𝑒 𝑎 for every transition (𝑞, 𝑎, 𝑞 ′ ) in 𝒯𝑒1 · 𝒯[𝑒−* ] .
1

{𝑚 ,𝑚 }

{𝑚 }

Case 𝑒 = [𝑒1 1 2 ] for 1 ≤ 𝑚1 < 𝑚2 (see Fig. 9). We first construct 𝒯𝑒1 1 as the
concatenation of 𝑚1 copies of 𝒯𝑒1 (Recall Definition 4.4 for the concatenation of PSSTs).
{𝑚 }
𝑚1
Note that 𝒯𝑒1 1 is different from 𝒯𝑒𝑚
, the concatenation
1 , the PSST constructed from 𝑒1
1
{𝑚 }

of the expression 𝑒1 for 𝑚1 times. In particular, the set of string variables in 𝒯𝑒1 1 is 𝑋𝑒1 ,
which is different from that of 𝒯𝑒𝑚
1.
1
{1,𝑚 −𝑚 }

Then we construct the PSST 𝒯𝑒1 2 1 (see Fig. 9), which consists of 𝑚2 − 𝑚1 copies of
(𝑖)
(1)
𝒯𝑒1 , denoted by (𝒯𝑒1 )𝑖∈[𝑚2 −𝑚1 ] , as well as the 𝜀-transition from 𝑞𝑒1 ,0 to a fresh state 𝑓0′ (of
(𝑖)

(𝑖)

the lowest priority), and the 𝜀-transitions from each 𝑓𝑒1 ,2 ∈ 𝐹𝑒1 ,2 with 1 ≤ 𝑖 < 𝑚2 − 𝑚1 to
(𝑖+1)

𝑞𝑒1 ,0 (of the highest priority) and a fresh state 𝑓1′ (of the lowest priority). The final states
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 16 -->

45:16

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu
{1,𝑚 −𝑚 }

(𝑖)

of 𝒯𝑒1 2 1 are ({𝑓0′ }, {𝑓1′ }). (Intuitively, each 𝒯𝑒1 accepts only nonempty strings, thus
(𝑖)
(𝑖)
{1,𝑚 −𝑚 }
𝑓𝑒1 ,1 ∈ 𝐹𝑒1 ,1 contains no outgoing transitions in 𝒯𝑒1 2 1 . ) Note that the set of string
{1,𝑚2 −𝑚1 }

variables in 𝒯𝑒1

is still 𝑋𝑒1 .

m2 − m1 copies of Te1
ε

ε

ε

(1)

qe1 ,0

(1)

T e1
(1)

(2)

···
(1)

fe1 ,1 ∈ Fe1 ,1

(1)

T e1

(1)

fe1 ,2 ∈ Fe1 ,2

(2)

ε

(2)
qe1 ,0

(m −m1 )

T e1 2

···
(2)

fe1 ,1 ∈ Fe1 ,1

(2)

...

(2)

fe1 ,2 ∈ Fe1 ,2

ε

(m −m1 )

fe1 ,12

ε

f00

(m −m1 )

qe1 ,02

···
(m −m1 )

∈ Fe1 ,12

(m −m1 )

fe1 ,22

(m −m1 )

∈ Fe1 ,22

ε

f10

{1,𝑚2 −𝑚1 }

Fig. 9. The PSST 𝒯𝑒1

{𝑚 }
{1,𝑚 −𝑚 }
{𝑚 }
Finally, we construct 𝒯𝑒 from 𝒯𝑒1 1 · 𝒯𝑒1 2 1 , the concatenation of 𝒯𝑒1 1 and
{1,𝑚2 −𝑚1 }
𝒯 𝑒1
, by adding a fresh state 𝑞𝑒,0 , a string variable 𝑥𝑒 , the 𝜀-transition 𝜏𝑒 (𝑞𝑒,0 ) =
{𝑚 }
((𝑞𝑒1 ,0 ); ()) (assuming that 𝑞𝑒1 ,0 is the initial state of 𝒯𝑒1 1 ), and also the assignments
′
𝐸𝑒 (𝑞𝑒0 , 𝜀, 𝑞𝑒1 ,0 )(𝑥𝑒 ) = 𝜀, as well as 𝐸𝑒 (𝑞, 𝑎, 𝑞 )(𝑥𝑒 ) = 𝑥𝑒 𝑎 for each transition (𝑞, 𝑎, 𝑞 ′ ) in
{𝑚 }
{1,𝑚 −𝑚 }
𝒯 𝑒1 1 · 𝒯 𝑒1 2 1 .

Example 4.5. Consider RegEx 𝑒 = [𝑎+ ]. We first construct 𝒯𝑎 and 𝒯𝑎−* (recall that 𝒯𝑎−* is
obtained from 𝒯𝑎* by removing the string variable 𝑥[𝑎* ] , see Fig. 10). Then we construct
𝒯𝑒 from 𝒯𝑎 · 𝒯𝑎−* by adding the initial state 𝑞[𝑎+ ],0 , the string variable 𝑥[𝑎+ ] , as well as the
assignments for 𝑥[𝑎+ ] (see Fig. 10). Note here only one copy of 𝒯𝑎−* is used in 𝒯𝑎 · 𝒯𝑎−* , since
𝜀 is not accepted by 𝒯𝑎 .
The following example illustrates the necessity of splitting final states into two disjoint
subsets.
Example 4.6. Consider RegEx 𝑒 = [([𝑎*? ])* ]. If we execute “𝑎𝑎𝑎”.match(/(𝑎*?)*/) in
node.js, then the result is the array [“𝑎𝑎𝑎”, “𝑎”], which means (𝑎*?)* is matched to “𝑎𝑎𝑎”
and (𝑎*?) is matched to 𝑎. If we did not split the set of final states into two disjoint subsets,
we would have obtained a PSST 𝒯𝑒′ as illustrated in Fig. 11, to simulate the matching of 𝑒
against words. The accepting run of 𝒯𝑒′ on 𝑤 = 𝑎𝑎𝑎 is
𝜀

𝜀

𝜀

𝜀

𝜀

𝑎

𝜀

𝜀

𝑞[([𝑎*? ])* ] −
→ 𝑞([𝑎*? ]),0 −
→ 𝑓([𝑎*? ]) −
→ 𝑞([𝑎*? ]),0 −
→ 𝑞𝑎,0 −
→ 𝑞𝑎,1 −
→ 𝑓𝑎 −
→ 𝑓([𝑎*? ]) −
→
𝜀
𝜀
𝑎
𝜀
𝜀
𝜀
𝜀
𝑎
𝜀
𝜀
𝑞([𝑎*? ]),0 −
→ 𝑞𝑎,0 −
→ 𝑞𝑎,1 −
→ 𝑓𝑎 −
→ 𝑓([𝑎*? ]) −
→ 𝑞([𝑎*? ]),0 −
→ 𝑞𝑎,0 −
→ 𝑞𝑎,1 −
→ 𝑓𝑎 −
→ 𝑓([𝑎*? ]) −
→
𝜀
𝜀
𝑞([𝑎*? ]),0 −
→ 𝑓([𝑎*? ]) −
→ 𝑓[([𝑎*? ])* ] ,
where 𝑥𝑒 = 𝑎𝑎𝑎 and 𝑥([𝑎*? ]) = 𝜀, namely, 𝑒 is matched to “𝑎𝑎𝑎” and ([𝑎*? ]) is matched to 𝜀.
Therefore, the semantics of 𝑒 defined by 𝒯𝑒′ is inconsistent with semantics of /(𝑎*?)*/ in
node.js. Intuitively, the semantics of /(𝑎*?)*/ in node.js requires that either it is matched to 𝜀
in whole and the subexpression 𝑎*? is not matched at all, or it is matched to a concatenation
of non-empty strings each of which matches 𝑎*?. This semantics can be captured by (adapted)
PSSTs where the set of final states is split into two disjoint subsets.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 17 -->

Solving String Constraints with Regex-Dependent Functions
Ta

T[a−∗ ]

qa,0

q[a∗ ],0
ε

ε xa := ε
f

qa,1
a xa := xa a

ε
qa,0

[a∗ ],1

ε xa := ε

xa := null

qa,1

ε

fa,0

45:17
T[a+ ]

ε
x[a+ ] := ε
ε xa := ε

q[a+ ],0

qa,0

qa,1
a xa := xa a
x[a+ ] := x[a+ ] a
ε

fa,0

xa := null

a xa := xa a

q[a∗ ],0
ε

ε
qa,0

f[a∗ ],1

fa,0
ε

ε xa := ε

xa := null

qa,1
xa := xa a
x[a+ ] := x[a+ ] a

ε

f[a∗ ],2

a

fa,0
ε
f[a∗ ],2

Fig. 10. The PSST 𝒯𝑒 for 𝑒 = [𝑎+ ]
q([a∗? ]),0

qa,0
ε xa := ε

ε
x([a∗? ]) := ε

qa,1
a xa := xa a
fa

0
T[([a
∗? ])∗ ]

0
T([a
∗? ])

Ta0

xa := null
ε

ε x([a∗? ]) := ε
qa,0
x := ε
ε a
qa,1
a xa := xa a
x([a∗? ]) := x([a∗? ]) a
fa
ε
f([a∗? ])

q[([a∗? ])∗ ],0
x[([a∗? ])∗ ] := ε

ε

ε

x[([a∗? ])∗ ] := ε

q([a∗? ]),0
ε
x([a∗? ]) := ε

xa := null
ε

ε x([a∗? ]) := ε

x([a∗? ]) := null

qa,0
x := ε
ε a

ε

qa,1
xa := xa a
a x ∗? := x ∗? a
([a ])
([a ])
x[([a∗? ])∗ ] := x[([a∗? ])∗ ] a
fa
ε
f([a∗? ])
ε

f[([a∗? ])∗ ]

Fig. 11. The PSST 𝒯𝑒′ for 𝑒 = [([𝑎*? ])* ] with a single set of final states

Validation experiments for the formal semantics. We have defined RegEx-string matching
by constructing PSSTs. In the sequel, we conduct experiments to validate the formal
semantics against the actual JavaScript RegEx-string matching.
Let O denote the set of RegEx operators: alternation +, concatenation ·, optional ?,
lazy optional ??, Kleene star *, lazy Kleene star *?, Kleene plus +, lazy Kleene plus +?,
repetition {𝑚1 , 𝑚2 }, and lazy repetition {𝑚1 , 𝑚2 }?. Moreover, let O 2 (resp. O 3 ) denote
the set of pairs (resp. triples) of operators from O. Aiming at a good coverage of different
syntactical ingredients of RegEx, we generate regular expressions for every element of
O ≤3 = O ∪ O 2 ∪ O 3 . As arguments of these operators, we consider the following character
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 18 -->

45:18

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

sets: S = {a, . . ., z}, C = {A, . . ., Z}, D = {0, . . . , 9}, and O, the set of ASCII symbols not
belonging to S ∪ C ∪ D. Intuitively, these character sets correspond to JavaScript character
classes [a-z], [A-Z], [0-9], and [ˆa-zA-Z0-9] (where ˆ denotes complement). Moreover, for the
regular expression generated for each element of O ≤3 , we set the subexpression corresponding
to its first component as the capturing group. For instance, for the pair (*?, *), we generate
the RegEx [([S*? ])* ]. In the end, we generate 10 + 10 * 10 + 10 * 10 * 10 = 1110 RegExes.
For each generated RegEx 𝑒, we construct a PSST 𝒯𝑒 , whose output corresponds to the
matching of the first capturing group in 𝑒. Moreover, we generate from 𝒯𝑒 an input string
𝑤 as well as the corresponding output 𝑤′ . We require that the length of 𝑤 is no less than
some threshold (e.g., 10), in order to avoid the empty string and facilitate a meaningful
comparison with the actual semantics of JavaScript regular-expression matching. Let reg
be the JavaScript regular expression corresponding to 𝑒. Then we execute the following
JavaScript program 𝒫𝑒,𝑤 ,
var x = w; console.log(x.match(reg)[1]);

and confirm that its output is equal to 𝑤′ , thus validating that the formal semantics of
RegEx-string matching defined by PSSTs is consistent with the actual semantics of JavaScript
match function. For instance, for the RegEx expression [([S*? ])* ], we generate from the 𝒯𝑒 the
input string 𝑤 = 𝑎𝑎𝑎𝑎𝑎𝑎𝑎𝑎𝑎𝑎, together with the output 𝑎. Then we generate the JavaScript
program from reg and 𝑤, execute it, and obtain the same output 𝑎.
In all the generated RegExs, we confirm the consistency of the formal semantics of RegExstring matching defined by PSSTs with the actual JavaScript semantics, namely, for each
RegEx 𝑒, the output of the PSST 𝒯𝑒 on 𝑤 is equal to the output of the JavaScript program
𝒫𝑒,𝑤 .
4.3

Modeling string functions by PSSTs

The extract, replace and replaceAll functions can be accurately modeled using PSSTs. That
is, we can reduce satisfiability of our string logic to satisfiability of a logic containing only
concatenation, PSST transductions, and membership of regular languages.
Lemma 4.7. The satisfiability of STR reduces to the satisfiability of boolean combinations
of formulas of the form 𝑧 = 𝑥 · 𝑦, 𝑦 = 𝒯 (𝑥), and 𝑥 ∈ 𝒜, where 𝒯 is a PSST and 𝒜 is an FA.
First, observe that regular constraints (aka membership queries) 𝑥 ∈ 𝑒 can be reduced
to FA membership queries 𝑥 ∈ 𝒜 using standard techniques. Features such a greediness
and capture groups do not affect whether a word matches a RegEx, they only affect how a
string matches it. Thus, for regular constraints, these features can be ignored and a standard
translation from regular expressions to finite automata can be used.
The extract𝑖,𝑒 function can be defined by a PSST 𝒯𝑖,𝑒 obtained from the PSST 𝒯𝑒 (see
Section 4.2) by removing all string variables, except 𝑥𝑒′ , where 𝑒′ is the subexpression of
𝑒 corresponding to the 𝑖th capturing group, and setting the output expression of the final
states as 𝑥𝑒′ .
We give a sketch of the encoding of replaceAll here. Full formal details are given in the
long version of the paper [Chen et al. 2021]. The encoding of replace is almost identical to
that of replaceAll.
A call replaceAllpat,rep (𝑥) replaces every match of pat by a value determined by the
replacement string rep. Recall, rep may contain references $𝑖, $← , or $→ . The first step in
our reduction to PSSTs is to eliminate the special references $0, $← , and $→ . In essence,
this simplification uses PSST transductions to insert the contextual information needed by
$← and $→ alongside each substring that will be replaced. Then, the call to replaceAll can
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 19 -->

Solving String Constraints with Regex-Dependent Functions

45:19

be rewritten to include this information in the match, and use standard references ($𝑖) in
the replacement string. The reference $0 can be eliminated by wrapping each pattern with
an explicit capturing group.
We show informally how to construct the PSST for replaceAllpat,rep where all the references
in rep are of the form $𝑖 with 𝑖 > 0. The full reduction is given in the long version of the
paper [Chen et al. 2021].
Let rep = 𝑤1 $𝑖1 𝑤2 · · · 𝑤𝑘 $𝑖𝑘 𝑤𝑘+1 . For each 𝑗 ∈ [𝑘], we introduce a fresh string variable
𝑦𝑗 . Let us use rep[(𝑦1 , · · · , 𝑦𝑘 )/($𝑖1 , · · · , $𝑖𝑘 )] to denote the sequence 𝑤1 𝑦1 𝑤2 · · · 𝑤𝑘 𝑦𝑘 𝑤𝑘+1 .
For instance, if rep = 𝑎$1𝑎$2𝑎$1𝑎, then rep[(𝑦1 , 𝑦2 , 𝑦3 )/(1, 2, 1)] = 𝑎𝑦1 𝑎𝑦2 𝑎𝑦3 𝑎. Moreover,
let 𝑒′𝑖1 , . . . , 𝑒′𝑖𝑘 be the subexpressions of pat corresponding to the 𝑖1 th, . . ., 𝑖𝑘 th capturing
groups. Note here we use mutually distinct fresh variables 𝑦1 , · · · , 𝑦𝑘 for $𝑖1 , · · · , $𝑖𝑘 , even if
𝑖𝑗 and 𝑖𝑗 ′ may be equal for 𝑗 ̸= 𝑗 ′ . We make this choice for the purpose of satisfying the
copyless property [Alur and Cerný 2010] of PSSTs, which leads to improved complexity
results in some cases (discussed in the sequel). If we tried to use the same variable for the
different occurrences of the same reference – then the resulting transition in the encoding
below would not be copyless. Moreover, the construction below guarantees that the values of
different variables for the multiple occurrences of the same reference are actually the same.
Suppose 𝒯pat = (𝑄pat , Σ, 𝑋pat , 𝛿pat , 𝜏pat , 𝐸pat , 𝑞pat,0 , (𝐹pat,1 , 𝐹pat,2 )). Then 𝒯replaceAllpat,rep is
obtained from 𝒯pat by adding the fresh string variables 𝑦1 , · · · , 𝑦𝑘 and a fresh state 𝑞0′ such
that (see Fig. 12)
∙ 𝒯replaceAllpat,rep goes from 𝑞0′ to 𝑞pat,0 via an 𝜀-transition of higher priority than the
non-𝜀-transitions, in order to search the first match of pat starting from the current
position,
∙ when 𝒯replaceAllpat,rep stays at 𝑞0′ , it keeps appending the current letter to the end of 𝑥0 ,
which stores the output of 𝒯replaceAllpat,rep ,
∙ starting from 𝑞pat,0 , 𝒯replaceAllpat,rep simulates 𝒯pat and stores the matches of capturing
groups of pat into the string variables (in particular, the matches of the 𝑖1 th, . . ., 𝑖𝑘 th
capturing groups into the string variables 𝑥𝑒′𝑖 , · · · , 𝑥𝑒′𝑖 respectively), moreover, for
1
𝑘
each 𝑗 ∈ [𝑘], 𝑦𝑗 is updated in the same way as 𝑥𝑒′𝑖 (in particular, for each transition
𝑗

(𝑞, 𝑎, 𝑞 ′ ) in 𝒯pat such that 𝐸pat (𝑞, 𝑎, 𝑞 ′ )(𝑥𝑒′𝑖 ) = 𝑥𝑒′𝑖 𝑎, we have 𝐸pat (𝑞, 𝑎, 𝑞 ′ )(𝑦𝑗 ) = 𝑦𝑗 𝑎),
𝑗
𝑗
∙ when the first match of pat is found, 𝒯replaceAllpat,rep goes from 𝑓pat,1 ∈ 𝐹pat,1 or 𝑓pat,2 ∈
𝐹pat,2 to 𝑞0′ via an 𝜀-transition, it then appends rep[(𝑦1 , · · · , 𝑦𝑘 )/($𝑖1 , · · · , $𝑖𝑘 )] (which
is the replacement string) to the end of 𝑥0 , resets the values of all the string variables,
except 𝑥0 , to null, and keeps searching for the next match of pat.

It may be observed that the PSST will be copyless. That is, the value of a variable is
not copied to two or more variables during a transition. In all but the last case, variables
are only copied to themselves, via assignments of the form 𝑥𝑒′ := 𝑥𝑒′ 𝑎, 𝑥𝑒′ := 𝑥𝑒′ , 𝑥𝑒′ := 𝜀,
or 𝑥𝑒′ := null. In the final case, when a replacement is made, the assignments are 𝑥0 :=
𝑥0 𝑤1 𝑦1 𝑤2 · · · 𝑤𝑘 𝑦𝑘 𝑤𝑘+1 and 𝑥′ := null for all the variables 𝑥′ ∈ 𝑋pat ∪ {𝑦1 , · · · , 𝑦𝑘 }. Again,
only one copy of the value of each variable is retained.
Copyful PSSTs are only needed when removing $← and $→ from the replacement strings.
To see this, consider the prefix preceding the first replacement in a string. If $← appears in
the replacement string, this prefix will be copied an unbounded number of times (once for
each matched and replaced substring). Conversely, references of the form $𝑖 are “local” to
a single match. By having a separate variable for each occurence of $𝑖 in the replacement
string, we can avoid having to make copies of the values of the variables.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 20 -->

45:20

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu
x0 := x0 rep[(y1 , · · · , yk )/($i1 , · · · , $ik )]

∀x0 ∈ Xpat ∪ {y1 , · · · , yk }, x0 := null

x0 := x0 a
a

Tpat

ε

xe0 := xe0 a
ij

fpat,1 ∈ Fpat,1

ij

yj := yj a
ε

q00

qpat,0

···

F (q00 ) = x0
ε

fpat,2 ∈ Fpat,2

x0 := x0 rep[(y1 , · · · , yk )/($i1 , · · · , $ik )]

∀x0 ∈ Xpat ∪ {y1 , · · · , yk }, x0 := null

Fig. 12. The PSST 𝒯replaceAllpat,rep
Table 1. Rules of the one-sided sequent calculus. A term 𝑒𝑐 denotes the complement of a regular
expression 𝑒, i.e., ℒ(𝑒𝑐 ) = Σ* ∖ ℒ(𝑒).
∧

Γ, 𝜙, 𝜓
Γ, 𝜙 ∧ 𝜓

¬∨

̸∈

Γ, 𝑥 ∈ 𝑒𝑐
Γ, 𝑥 ̸∈ 𝑒

̸=

=-Prop

Γ, ¬𝜙, ¬𝜓
Γ, ¬(𝜙 ∨ 𝜓)

Γ, 𝑥 ∈ 𝑒, 𝑦 ∈ 𝑒
Γ, 𝑥 ∈ 𝑒, 𝑥 = 𝑦
Close

Subsume

¬∧

Γ, ¬𝜙
Γ, ¬𝜓
Γ, ¬(𝜙 ∧ 𝜓)

where 𝑦 is fresh

̸=-Subsume
if |ℒ(𝑒)| = 1

Cut

Γ, 𝑥 ∈ 𝑒

Γ, 𝑥 ∈ 𝑒1 , 𝑦 ∈ 𝑒2
Γ, 𝑥 ∈ 𝑒1 , 𝑥 ̸= 𝑦, 𝑦 ∈ 𝑒2
̸=-Prop-Elim

Γ, 𝑥 ∈ 𝑒1 , . . . , 𝑥 ∈ 𝑒𝑛

Γ, 𝑥 ∈ 𝑒1 , . . . , 𝑥 ∈ 𝑒𝑛
Γ, 𝑥 ∈ 𝑒, 𝑥 ∈ 𝑒1 , . . . , 𝑥 ∈ 𝑒𝑛

Intersect

Fwd-Prop

Γ, 𝜙
Γ, 𝜓
Γ, 𝜙 ∨ 𝜓

Γ, 𝑥 ̸= 𝑦, 𝑦 = 𝑓 (𝑥1 , . . . , 𝑥𝑛 )
Γ, 𝑥 ̸= 𝑓 (𝑥1 , . . . , 𝑥𝑛 )

Γ, 𝑥 ∈ 𝑒, 𝑥 = 𝑦, 𝑦 ∈ 𝑒
Γ, 𝑥 ∈ 𝑒, 𝑥 = 𝑦

=-Prop-Elim

∨

Γ, 𝑥 ∈ 𝑒
Γ, 𝑥 ∈ 𝑒1 , . . . , 𝑥 ∈ 𝑒𝑛

Γ, 𝑥 ∈ 𝑒, 𝑥 = 𝑓 (𝑥1 , . . . , 𝑥𝑛 ), 𝑥1 ∈ 𝑒1 , . . . , 𝑥𝑛 ∈ 𝑒𝑛
Γ, 𝑥 = 𝑓 (𝑥1 , . . . , 𝑥𝑛 ), 𝑥1 ∈ 𝑒1 , . . . , 𝑥𝑛 ∈ 𝑒𝑛

¬¬
Γ

Γ, 𝜙
Γ, ¬¬𝜙
Γ, 𝑥 ∈ 𝑒𝑐

if ℒ(𝑒1 ) ∩ ℒ(𝑒2 ) = ∅

Γ, 𝑥 ∈ 𝑒, 𝑦 ∈ 𝑒𝑐
Γ, 𝑥 ∈ 𝑒, 𝑥 ̸= 𝑦

if |ℒ(𝑒)| = 1

if ℒ(𝑒1 ) ∩ · · · ∩ ℒ(𝑒𝑛 ) = ∅
if ℒ(𝑒1 ) ∩ · · · ∩ ℒ(𝑒𝑛 ) ⊆ ℒ(𝑒)
if

𝑛 > 1 and
ℒ(𝑒1 ) ∩ · · · ∩ ℒ(𝑒𝑛 ) = ℒ(𝑒)

if ℒ(𝑒) = 𝑓 (ℒ(𝑒1 ), . . . , ℒ(𝑒𝑛 ))

Γ, 𝑥 ∈ 𝑒, 𝑥1 ∈ 𝑒1 , . . . , 𝑥𝑛 ∈ 𝑒𝑛
ℒ(𝑒) = 𝑓 (ℒ(𝑒1 ), . . . , ℒ(𝑒𝑛 ))
if
and |ℒ(𝑒)| = 1
Γ, 𝑥 = 𝑓 (𝑥1 , . . . , 𝑥𝑛 ), 𝑥1 ∈ 𝑒1 , . . . , 𝑥𝑛 ∈ 𝑒𝑛
{︀
}︀
𝑖
𝑖 𝑘
𝑓 −1 (ℒ(𝑒)) =
Γ, 𝑥 ∈ 𝑒, 𝑥 = 𝑓 (𝑥1 , . . . , 𝑥𝑛 ), 𝑥1 ∈ 𝑒1 , . . . , 𝑥𝑛 ∈ 𝑒𝑛 𝑖=1
(︀ 𝑖 )︀)︀
if ⋃︀𝑘 (︀ (︀ 𝑖 )︀
Bwd-Prop
𝑖=1 ℒ 𝑒1 × · · · × ℒ 𝑒𝑛
Γ, 𝑥 ∈ 𝑒, 𝑥 = 𝑓 (𝑥1 , . . . , 𝑥𝑛 )
Fwd-Prop-Elim

5

A PROPAGATION-BASED CALCULUS FOR STRING CONSTRAINTS

We now introduce our calculus for solving string constraints in STR (see Table 1), state its
correctness, and observe that it gives rise to a decision procedure for the fragment STRSL of
straightline formulas. The calculus is based on the principle of propagating regular language
constraints by computing images and pre-images of string functions. We deliberately keep
the calculus minimalist and focus on the main proof rules; for an implementation, the
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 21 -->

Solving String Constraints with Regex-Dependent Functions

Close
Fwd-Prop

45:21

𝑥 ∈ 𝑎+ Σ* , 𝑥 = 𝑦 · 𝑧, 𝑦 ∈ 𝑎+ , 𝑧 ∈ Σ* , 𝑥 ∈ 𝑏+ (𝑎𝑐 )* , 𝑥 = replaceAll𝑎,𝑏 (𝑥)

Fwd-Prop

𝑥 ∈ 𝑎+ Σ* , 𝑥 = 𝑦 · 𝑧, 𝑦 ∈ 𝑎+ , 𝑧 ∈ Σ* , 𝑥 = replaceAll𝑎,𝑏 (𝑥)
*

∧

𝑥 = 𝑦 · 𝑧, 𝑦 ∈ 𝑎+ , 𝑧 ∈ Σ* , 𝑥 = replaceAll𝑎,𝑏 (𝑥)

𝑥 = 𝑦 · 𝑧 ∧ 𝑦 ∈ 𝑎+ ∧ 𝑧 ∈ Σ* ∧ 𝑥 = replaceAll𝑎,𝑏 (𝑥)

Fig. 13. Proof of unsatisfiability for (3) in Example 5.1

𝑥 ∈ 𝑎, 𝑧 ∈ 𝑎, 𝑦 ∈ 𝜖, 𝑟 ∈ 𝑏
Subsume*
𝑥 ∈ 𝑎, 𝑧 ∈ 𝑎, 𝑦 ∈ 𝜖, 𝑟 ∈ 𝑏, . . .
FPE
..
𝑧 ∈ 𝑎, 𝑦 ∈ 𝜖, 𝑥 ∈ 𝑎, 𝑟 = replaceAll𝑎,𝑏 (𝑥), . . .
.
..
FPE
𝑧 ∈ 𝑎, 𝑦 ∈ 𝜖, 𝑥 = 𝑦 · 𝑧, . . .
𝑧 ∈ 𝑎𝑐 , . . .
.
Cut
𝑦 ∈ 𝜖, 𝑧 ∈ 𝑎+ , 𝑥 = 𝑦 · 𝑧, 𝑥 ∈ 𝑎+ , . . .
𝑦 ∈ 𝑎+ , 𝑧 ∈ 𝑎* , . . .
Bwd-Prop
𝑥 = 𝑦 · 𝑧, 𝑥 ∈ 𝑎+ , 𝑟 = replaceAll𝑎,𝑏 (𝑥)
∧*
𝑥 = 𝑦 · 𝑧 ∧ 𝑥 ∈ 𝑎+ ∧ 𝑟 = replaceAll𝑎,𝑏 (𝑥)
Fig. 14. Proof of satisfiability for (4) in Example 5.2. FPE stands for Fwd-Prop-Elim

calculus has to be complemented with a suitable strategy for applying the rules, as well as
standard SMT optimizations such as non-chronological back-tracking and conflict-driven
learning. An implementation also has to choose a suitable effective representation of RegEx
membership constraints, for instance using finite-state automata.3 In particular, we use the
fact that—for membership—RegEx can be complemented. We denote the complement of 𝑒 in
a membership constraint by 𝑒𝑐 . Our calculus is parameterized in the set of considered string
functions; in this paper, we work with the set {·, extract, replace, replaceAll} consisting of
concatenation, extraction, and replacement, but this set can be extended by other functions
for which images and/or pre-images can be computed (see Section 5.2).
5.1

Sequents and Examples

The calculus operates on one-sided sequents, and can be interpreted as a sequent calculus in
the sense of Gentzen [Gentzen 1935] in which all formulas are located in the antecedent (to
the left of the turnstile ⊢). A one-sided sequent is a finite set Γ ⊆ STR of string constraints.
For sake of presentation, we write sequents as lists of formulas separated by comma, and
⋀︀
Γ, 𝜙1 , . . . , 𝜙𝑛 for the union Γ ∪ {𝜙1 , . . . , 𝜙𝑛 }. We say that a sequent Γ is unsatisfiable if Γ
is unsatisfiable. Our calculus is refutational and has the purpose of either showing that
some initial sequent Γ is unsatisfiable, or that it is satisfiable by constructing a solution for
it. A solution is a sequent 𝑥1 ∈ 𝑤1 , 𝑥2 ∈ 𝑤2 , . . . , 𝑥𝑛 ∈ 𝑤𝑛 that defines the values of string
variables using RegExes that only consist of single words.
Example 5.1. We first illustrate the calculus by showing unsatisfiability of the constraint4 :
𝑥 = 𝑦 · 𝑧 ∧ 𝑦 ∈ 𝑎+ ∧ 𝑧 ∈ Σ* ∧ 𝑥 = replaceAll𝑎,𝑏 (𝑥)

(3)

3 Recall features such as greediness do not need to be modeled for simple membership queries as they do not

change the accepted language.
4 Note here for convenience, in the regular constraints 𝑥 ∈ 𝑒, we write 𝑒 as in classical regular expressions

and do not strictly follow the syntax of STR, since in this case, only the language defined by 𝑒 matters.

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 22 -->

45:22

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

To this end, we construct a proof tree that has (3) as its root, by applying proof rules
until all proof goals have been closed (Fig. 13). The proof is growing upward, and is built
by first eliminating the conjunctions ∧, resulting in a list of formulas. Next, we apply the
rule Fwd-Prop for forward-propagation of a regular expression constraint. Given that
𝑦 ∈ 𝑎+ , 𝑧 ∈ Σ* , from the equation 𝑥 = 𝑦 · 𝑧 we can conclude that 𝑥 ∈ 𝑎+ Σ* . From 𝑥 ∈ 𝑎+ Σ*
and 𝑥 = replaceAll𝑎,𝑏 (𝑥), we can next conclude that 𝑥 ∈ 𝑏+ (𝑎𝑐 )* , i.e., 𝑥 starts with 𝑏 and
cannot contain the letter 𝑎. Finally, the proof can be closed because the languages 𝑎+ Σ*
and 𝑏+ (𝑎𝑐 )* are disjoint.
Example 5.2. We next consider the case of a satisfiable formula in STRSL :
𝑥 = 𝑦 · 𝑧 ∧ 𝑥 ∈ 𝑎+ ∧ 𝑟 = replaceAll𝑎,𝑏 (𝑥)

(4)

Fig. 14 shows how a solution can be constructed for this formula. The strategy is to first
derive constraints for the variables 𝑦, 𝑧 whose value is not determined by any equation.
Given that 𝑥 ∈ 𝑎+ , from the equation 𝑥 = 𝑦 · 𝑧 we can derive that either 𝑦 ∈ 𝜖, 𝑧 ∈ 𝑎+ or
𝑦 ∈ 𝑎+ , 𝑧 ∈ 𝑎* , using rule Bwd-Prop. We focus on the left branch 𝑦 ∈ 𝜖, 𝑧 ∈ 𝑎+ . Since
propagation is not able to derive further information for 𝑦, 𝑧, and no contradiction was
detected, at this point we can conclude satisfiability of (4). To construct a solution, we pick an
arbitrary value for 𝑧 satisfying the constraint 𝑧 ∈ 𝑎+ , and use Cut to add the formula 𝑧 ∈ 𝑎
to the branch. Again following the left branch, we can then use Fwd-Prop-Elim to evaluate
𝑥 = 𝑦 · 𝑧 and add the formula 𝑥 ∈ 𝑎, and after that 𝑟 ∈ 𝑏 due to 𝑟 = replaceAll𝑎,𝑏 (𝑥). Finally,
Subsume is used to remove redundant RegEx constraints from the proof goal. The resulting
sequent (top-most sequent on the left-most branch) is a witness for satisfiability of (4).
5.2 Proofs and Proof Rules
More formally, proof rules are relations between a finite list of sequents (the premises), and
a single sequent (the conclusion). Proofs are finite trees growing upward, in which each node
is labeled with a sequent, and each non-leaf node is related to the node(s) directly above it
through an instance of a proof rule. A proof branch is a path from the proof root to a leaf.
A branch is closed if a closure rule (a rule without premises) has been applied to its leaf,
and open otherwise. A proof is closed if all of its branches are closed.
The proof rules of the calculus are shown in Table 1. The first row shows standard
proof rules to handle Boolean operators; see, e.g., [Harrison 2009]. Rule ̸∈ turns negated
membership predicates into positive ones through complementation, and rule ̸= negative
function applications into positive ones. As a result, only disequalities between string variables
remain. The rule Cut can be used to introduce case splits, and is mainly needed to extract
solutions once propagation has converged (as shown in Example 5.2).
The next four rules handle equations between string variables. Rule =-Prop propagates
RegEx constraints from the left-hand side to the right-hand side of an equation; =-PropElim in addition removes the equation in the case where the propagated constraint has a
unique solution. The rule ̸=-Prop-Elim similarly turns a singleton RegEx for the left-hand
side of a disequality into a RegEx constraint on the right-hand side. As a convention, we allow
application of =-Prop, =-Prop-Elim, and ̸=-Prop-Elim in both directions, left-to-right
and right-to-left of equalities/disequalities. Finally, ̸=-Subsume eliminates disequalities that
are implied by the RegEx constraints of a proof goal.
The rule Close closes proof branches that contain contradictory RegEx constraints, and
is the only closure rule needed in our calculus. Subsume removes RegEx constraints that
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 23 -->

Solving String Constraints with Regex-Dependent Functions

45:23

are implied by other constraints in a sequent, and Intersect replaces multiple RegExes
with a single constraint.
The last three rules handle applications of functions 𝑓 ∈ {·, extract, replace, replaceAll}
through propagation. Rule Fwd-Prop defines forward propagation, and adds a RegEx
constraint 𝑥 ∈ 𝑒 for the value of a function by propagating constraints about the arguments.
The RegEx 𝑒 encodes the image of the argument RegExes under 𝑓 :
Definition 5.3 (Image). For an 𝑛-ary string function 𝑓 : Σ* × · · · × Σ* → Σ* and
languages 𝐿1 , . . . , 𝐿𝑛 ⊆ Σ* , we define the image of 𝐿1 , . . . , 𝐿𝑛 under 𝑓 as 𝑓 (𝐿1 , . . . , 𝐿𝑛 ) =
{𝑓 (𝑤1 , . . . , 𝑤𝑛 ) ∈ Σ* | 𝑤1 ∈ 𝐿1 , . . . , 𝑤𝑛 ∈ 𝐿𝑛 }.
Forward propagation is often useful to prune proof branches. It is easy to see, however, that
the images of regular languages under the functions considered in this paper are not always
regular; for instance, replacepat,$0$0 can map regular languages to context-sensitive languages.
In such cases, the side condition of Fwd-Prop cannot be satisfied by any RegEx 𝑒, and the
rule is not applicable.
Rule Fwd-Prop-Elim handles the special case of forward propagation producing a
singleton language. In this case, the function application is not needed for further reasoning
and can be eliminated. This rule is mainly used during the extraction of solutions (as shown
in Example 5.2).
Rule Bwd-Prop defines the dual case of backward propagation, and derives RegEx
constraints for function arguments from a constraint about the function value. The argument
constraints encode the pre-image of the propagated language:
Definition 5.4 (Pre-image). For an 𝑛-ary string function 𝑓 : Σ* × · · · × Σ* → Σ* and
a language 𝐿 ⊆ Σ* , we define the pre-image of 𝐿 under 𝑓 as the relation 𝑓 −1 (𝐿) =
{(𝑤1 , . . . , 𝑤𝑛 ) ∈ (Σ* )𝑛 | 𝑓 (𝑤1 , . . . , 𝑤𝑛 ) ∈ 𝐿}.
A key result of the paper is that pre-images of regular languages under
(︀ )︀ the functions
(︀ )︀
⋃︀𝑘
considered in the paper can always be represented in the form 𝑖=1 (ℒ 𝑒𝑖1 × · · · × ℒ 𝑒𝑖𝑛 ),
i.e., they are recognizable languages [Carton et al. 2006]. This implies that Bwd-Prop
is applicable whenever a RegEx constraint for the result of a function application exists,
and prepares the ground for the decidability result in the next section. For concatenation,
recognizability was shown in [Abdulla et al. 2014; Chen et al. 2019]. This paper contributes
the corresponding result for all functions defined by PSSTs:
Lemma 5.5 (Pre-image of regular languages under PSSTs). Given a PSST
𝒯 = (𝑄𝑇 , Σ, 𝑋, 𝛿𝑇 , 𝜏𝑇 , 𝐸𝑇 , 𝑞0,𝑇 , 𝐹𝑇 ) and an FA 𝒜 = (𝑄𝐴 , Σ, 𝛿𝐴 , 𝑞0,𝐴 , 𝐹𝐴 ), we can compute
an FA ℬ = (𝑄𝐵 , Σ, 𝛿𝐵 , 𝑞0,𝐵 , 𝐹𝐵 ) in exponential time such that L (ℬ) = ℛ−1
𝒯 (L (𝒜)).
The proof of Lemma 5.5 is given in the long version of the paper [Chen et al. 2021].
Moreover, we have already shown in Lemma 4.7 that extract, replace, and replaceAll can be
reduced to PSSTs. We can finally observe that the calculus is sound:
Lemma 5.6 (Soundness). The sequent calculus defined by Table 1 is sound: (i) the root
of a closed proof is an unsatisfiable sequent; and (ii) if a proof has an open branch that
ends with a solution 𝑥1 ∈ 𝑤1 , 𝑥2 ∈ 𝑤2 , . . . , 𝑥𝑛 ∈ 𝑤𝑛 , then the assignment {𝑥1 ↦→ 𝑤1 , 𝑥2 ↦→
𝑤2 , . . . , 𝑥𝑛 ↦→ 𝑤𝑛 } is a satisfying assignment of the root sequent.
Proof. By showing that each of the proof rules in Table 1 is an equivalence transformation:
the conclusion of a proof rule is equivalent to the disjunction of the premises.
□
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 24 -->

45:24

5.3

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Decision Procedure for STRSL

One of the main results of this paper is the decidability of the STRSL fragment of straightline
formulas including concatenation, extract, replace, and replaceAll:
Theorem 5.7. Satisfiability of STRSL formulas is decidable.
Proof. We define a terminating strategy to apply the rules in Table 1 to formulas in the
STRSL fragment. The resulting proofs will either be closed, proving unsatisfiability, or have
at least one satisfiable goal containing a solution:
∙ Phase 1: apply the Boolean rules (first row of Table 1) to eliminate Boolean operators.
∙ Phase 2: apply rule Bwd-Prop to all regex constraints and all function applications on
all proof branches. Whenever contradictory regex constraints occur in a proof goal, use
Close to close the branch. Also apply =-Prop to systematically propagate constraints
across equations. This phase terminates because STRSL formulas are acyclic.
If all branches are closed as a result of Phase 2, the considered formula is unsatisfiable;
otherwise, we can conclude satisfiability, and Phase 3 will extract a solution.
∙ Phase 3: select an open branch of the proof. On this branch, determine the set 𝐼 of input
variables, which are the string variables that do not occur as left-hand side of equations
or function applications. For every 𝑥 ∈ 𝐼, use rule Cut to introduce an assignment 𝑥 ∈ 𝑤
that is consistent with the regex constraints on 𝑥. Then systematically apply Subsume,
=-Prop, Fwd-Prop-Elim to evaluate remaining formulas and produce a solution.
□
Complexity analysis. Because the pre-image computation for each PSST incurs an exponential blow-up in the size of the input automaton 𝒜, the aforementioned decision procedure
has a non-elementary complexity in the worst-case. In fact, this is optimal and a matching
lower-bound is given in the long version of the paper [Chen et al. 2021].
When $← and $→ are not used, the PSSTs in the reduction are copyless, and the exponential
blow-up in the size of the input FA 𝒜 can be avoided. That is, the pre-image automaton ℬ
such that L (ℬ) = ℛ−1
𝒯 (L (𝒜)) is exponential only in the size of 𝒯 and not the size of 𝒜.
Hence, the exponentials do not stack on top of each other during the backwards analysis and
the non-elementary blow-up is not necessary. Since the PSST 𝒯 may be exponential in the
size of the underlying regular expression, we may compute automata that are up to double
exponential in size. The states of these automata can be stored in exponential space and the
transition relation can be computed on the fly, giving an exponential space algorithm. More
details are given in the long version of this paper [Chen et al. 2021].
Moreover, as the number of PSSTs is usually small in the path constraints of stringmanipulating programs, the performance of the decision procedure is actually good on the
benchmarks we tested, with the average running time per query a few seconds (see Section 6).
6

IMPLEMENTATION AND EXPERIMENTS

We extend the open-source solver OSTRICH [Chen et al. 2019] to support for STR based on
the calculus. In particular, it can decide the satisfiability of STRSL formulas. The extension
can handle most of the other operations of the SMT-LIB theory of Unicode strings.5
5 http://smtlib.cs.uiowa.edu/theories-UnicodeStrings.shtml

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 25 -->

Solving String Constraints with Regex-Dependent Functions

6.1

45:25

Implementation

Our solver extends classical regular expressions in SMT-LIB with indexed re.capture and
re.reference operators, which denote capturing groups and references to them. We also add
re.*?, re.+?, re.opt? and re.loop? as the lazy counterparts of Kleene star, plus operator,
optional operator and loop operator.
Three new string operators are introduced to make use of these extended regular expressions: str.replace cg, str.replace cg all, and str.extract. The operators str.replace cg and
str.replace cg all are the counterparts of the standard str.replace re and replace re all operators,
and allow capturing groups in the match pattern and references in the replacement pattern.
E.g., the following constraint swaps the first name and the last name, as in Example 1.1:
(= w (str.replace_cg_all v
(re.++ ((_ re.capture 1)
(re.+ (re.union (re.range "A" "Z") (re.range "a" "z"))))
(str.to.re " ")
((_ re.capture 2)
(re.+ (re.union (re.range "A" "Z") (re.range "a" "z")))))
(re.++ (_ re.reference 2) (_ re.reference 1))))

The replacement string is written as a regular expression only containing the operators
re.++, str.to re, and re.reference. The use of string variables in the replacement parameter is
not allowed, since the resulting transformation could not be mapped to a PSST.
The indexed operator str.extract implements extract𝑖,𝑒 in STR. For instance,
((_ str.extract 1)
(re.++ (re.*? re.allchar)((_ re.capture 1) (re.+ (re.range "a" "z")) re.all))
x)

extracts the left-most, longest sub-string of lower-case characters from a string 𝑥.
Our implementation is able to handle anchors as well, although for reasons of presentation
we did not introduce them as part of our formalism. Anchors match certain positions of a
string without consuming any input characters. In most programming languages, it is common
to use ^ and $ in regular expressions to signify the start and end of a string, respectively.
We add re.begin-anchor and re.end-anchor for them. Our implementation correctly models
the semantics of anchors and is able to solve constraints containing these operators.
OSTRICH implements the procedure in Theorem 5.7, and focuses on SL formulas. The
three string operators mentioned above will be converted into an equivalent PSST (see
the full version of the paper [Chen et al. 2021]). OSTRICH then iteratively applies the
propagation rules from Section 5 to derive further RegEx constraints, and eventually either
detect a contradiction, or converge and find a fixed-point. For straight-line formulas, the
existence of a fixed-point implies satisfiability, and a solution can then be constructed
as described in Section 5. In addition, similar to other SMT solvers, OSTRICH applies
simplification rules (e.g., Fwd-Prop-Elim, =-Prop, Subsume, Close, etc in Table 1) to formulas
before invoking the SL procedure. This enables OSTRICH to solve some formulas outside of
the SL fragment, but is not a complete procedure for non-SL formulas.
6.2

Experimental evaluation

Our experiments have the purpose of answering the following main questions:
R1: How does OSTRICH compare to other solvers that can handle real-world regular
expressions, including greedy/lazy quantifiers and capturing groups?
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 26 -->

45:26

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

R2: How does OSTRICH perform in the context of symbolic execution, the primary
application of string constraint solving?
For R1: There are no standard string benchmarks involving RegExes, and we are not
aware of other constraint solvers supporting capturing groups, neither among the SMT nor
the CP solvers. The closest related work is the algorithm implemented in ExpoSE, which
applies Z3 [de Moura and Bjørner 2008] for solving string constraints, but augments it with
a refinement loop to approximate the RegEx semantics.6 For R1, we compared OSTRICH
with ExpoSE+Z3 on 98,117 RegExes taken from [Davis et al. 2019].
(declare-fun x () String)
function fun(x) {
(define-fun y () String (str.replace_cg_all x <re1> <repl>)) if(/<re1>/.test(x)) {
(push 1)
var y = x.replace(/<re1>/g, <repl>);
(assert (str.in.re x (re.++ re.all <re1> re.all)))
if(/<re2>/.test(y))
(assert (str.in.re y (re.++ re.all <re2> re.all)))
console.log("1");
(check-sat) (get-model)
else
(pop 1) (push 1)
console.log("2");
(assert (str.in.re x (re.++ re.all <re1> re.all)))
}
(assert (not (str.in.re y
else
(re.++ re.all <re2> re.all))))
console.log("3");
(check-sat) (get-model)
}
(pop 1) (push 1)
(assert (not (str.in.re x (re.++ re.all <re1> re.all))))
var S$ = require("S$");
(check-sat) (get-model)
var x = S$.symbol("x", "");
(pop 1)
fun(x);

Fig. 15. Harnesses with replace-all: SMT-LIB for OSTRICH (left), and JavaScript for ExpoSE (right).

For each regular expression, we created four harnesses: two in SMT-LIB, as inputs for
OSTRICH, and two in JavaScript, as inputs for ExpoSE+Z3. The two harnesses shown in
Fig. 15 use one of the regular expressions from [Davis et al. 2019] (<re1>) in combination
with the replace-all function to simulate typical string processing; <re2> is the fixed pattern
[a-z]+, and <repl> the replacement string "$1". The three paths of the JavaScript function fun correspond to the three queries in the SMT-LIB script, so that a direct comparison
can be made between the results of the SMT-LIB queries and the set of paths covered by
ExpoSE+Z3. The other two harnesses are similar to the ones in Fig. 15, but use the match
function instead of replace-all, and contain four queries and four paths, respectively.
The results of this experiment are shown in Table 2. OSTRICH is able to answer all four
queries in 95,175 of the match benchmarks (97%), and all three queries in 89,794 of the
replace-all benchmarks (91.5%). The errors in 1,134 cases (resp., 1,135 cases) are mainly
due to back-references in <re1>, which are not handled by OSTRICH. ExpoSE+Z3 can
cover 228,888 paths of the match problems in total (91.2% of the number of sat results of
OSTRICH), although the runtime of ExpoSE+Z3 is on average 18x higher than that of
OSTRICH. For replace, ExpoSE+Z3 can cover 173,007 paths (66.7%), showing that this
class of constraints is harder; the runtime of ExpoSE+Z3 is on average 8x higher than that
of OSTRICH. Overall, even taking into account that ExpoSE+Z3 has to analyze JavaScript
code, as opposed to the SMT-LIB given to OSTRICH, the experiments show that OSTRICH
is a highly competitive solver for RegExes.
For R2: For this experiment, we integrated OSTRICH into Aratha [Amadini et al. 2019], a
symbolic execution engine for Javascript. We compare Aratha+OSTRICH with ExpoSE+Z3
6 We considered replacing Z3 with OSTRICH in ExpoSE for the experiments. However, ExpoSE integrates

Z3 using its C API, and changing to OSTRICH, with native support for capturing groups, would have
required the rewrite of substantial parts of ExpoSE.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 27 -->

Solving String Constraints with Regex-Dependent Functions

OSTRICH
# queries solved within 60s
0
1
2
3
4
#Err
Match
422
249
751
386
95,175 1,134
(98,117
Average time: 1.57s
benchm.)
Total #sat: 250,947, #unsat: 132,662
Replace 4,170 2,463 555 89,794
—
1,135
(98,117
Average time: 6.62s
bench.)
Total #sat: 259,354, #unsat: 13,601

45:27

ExpoSE+Z3
# paths covered within 60s
0
1
2
3
4
3,333 9,274 36,916 48,594
0
Average time: 28.0s
Total #paths covered: 228,888
5,281 18,221 69,059 5,556 —
Average time: 55.0s
Total #paths covered: 173,007

Table 2. The number of queries answered by OSTRICH, and number of paths covered by ExpoSE+Z3,
in R1. Experiments were done on an AMD Opteron 2220 SE machine, running 64-bit Linux and Java 1.8.
Runtime per benchmark was limited to 60s wall-clock time, 2GB memory, and the number of tests
executed concurrently by ExpoSE+Z3 to 1. Average time is wall-clock time per benchmark, timeouts
count as 60s.

ExpoSE
(49 programs)
Match
(28 programs)
Replace
(38 programs)

Aratha+OSTRICH
# paths covered within 120s
0
1
2
3 ≥4 #Err
14 9
9
2
15
2
Average time: 4.66s
Total #paths covered:124
3
7 12 6
0
0
Average time: 5.19s
Total #paths covered: 49
12 20 6
0
0
0
Average time: 4.14s
Total #paths covered: 32

ExpoSE+Z3
# paths covered within 120s
0
1
2
3 ≥4 #T.O.
14 9
9
2
15
6
Average time: 57.44s
Total #paths covered:121
3
8 12 5
0
6
Average time: 60.26s
Total #paths covered: 47
15 21 2
0
0
23
Average time: 95.34s
Total #paths covered: 25

Table 3. Results of Expose+Z3 and Aratha+OSTRICH on Javascript programs for R2. Experiments
were done on an Intel(R)-Core(TM)-i5-8265U-CPU-@1.60GHz cpu, running 64-bit Linux and Java 1.8.
Runtime was limited to 120s wall-clock time. Average time is wall-clock time needed per benchmark,
and counts timeouts as 120s. #Err is the number of non-straight-line path constraints that OSTRICH
fails to deal with and #T.O is the number of timeouts. Note that some paths may have already been
covered before T.O.

on the regression test suite of ExpoSE [Loring et al. 2017], as well as a collection of other
JavaScript programs containing match or replace functions extracted from Github. In
Table 3, we can see that Aratha+OSTRICH can within 120s cover slightly more paths
than ExpoSE+Z3. Aratha+OSTRICH can discover feasible paths much more quickly than
ExpoSE+Z3, however: on all three families of benchmarks, Aratha+OSTRICH terminates
on average in less than 10s, and it discovers all paths within 20s. ExpoSE+Z3 needs the full
120s for 35 of the programs (“T.O.” in the table), and it finds new paths until the end of
the 120s. Since ExpoSE+Z3 handles the replace-all operation by unrolling, it is not able to
prove infeasibility of paths involving such operations, and will therefore not terminate on
some programs. Overall, the experiments indicate that OSTRICH is more efficient than the
CEGAR-augmented symbolic execution for dealing with RegExes.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 28 -->

45:28

7

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

RELATED WORK

Modelling and Reasoning about RegEx. Variants and extensions of regular expressions
to capture their usage in programming languages have received attention in both theory
and practice. In formal language theory, regular expressions with capturing groups and
backreferences were considered in [Câmpeanu et al. 2003; Carle and Narendran 2009] and also
more recently in [Berglund and van der Merwe 2017b; Freydenberger 2013; Freydenberger
and Schmid 2019; Schmid 2016], where the expressibility issues and decision problems were
investigated. Nevertheless, some basic features of these regular expression, namely, the noncommutative union and the greedy/lazy semantics of Kleene star/plus, were not addressed
therein. In the software engineering community, some empirical studies were recently reported
for these regular expressions, including portability across different programing languages
[Davis et al. 2019] and DDos attacks [Staicu and Pradel 2018], as well as how programmers
write them in practice [Michael et al. 2019].
Prioritized finite-state automata and transducers were proposed in [Berglund and van der
Merwe 2017a]. Prioritized finite-state transducers add indexed brackets to the input string
in order to identify the matches of capturing groups. It is hard—if not impossible—to use
prioritized finite-state transducers to model replace(all) function, e.g., swapping the first
and last name as in Example 1.1. In contrast, PSSTs store the matches in string variables,
which can then be referred to, allowing us to conveniently model the match and replace(all)
function. Streaming string transducers were used in [Zhu et al. 2019] to solve the straight-line
string constraints with concatenation, finite-state transducers, and regular constraints.
String Constraint Solving. As we discussed Section 1, there has been much research
focussing on string constraint solving algorithms, especially in the past ten years. Solvers
typically use a combination of techniques to check the satisfiability of string constraints,
including word-based methods, automata-based methods, and unfolding-based methods like
the translation to bit-vector constraints. We mention among others the following string
solvers: Z3 [de Moura and Bjørner 2008], CVC4 [Liang et al. 2014], Z3-str/2/3/4 [Berzish
et al. 2017; Berzish, Murphy 2021; Zheng et al. 2015, 2013], ABC [Bultan and contributors
2015], Norn [Abdulla et al. 2014], Trau [Abdulla et al. 2017, 2018; Bui and contributors
2019], OSTRICH [Chen et al. 2019], S2S [Le and He 2018], Qzy [Cox and Leasure 2017],
Stranger [Yu et al. 2010], Sloth [Abdulla et al. 2019; Holı́k et al. 2018], Slog [Wang et al.
2016], Slent [Wang et al. 2018], Gecode+S [Scott et al. 2017], G-Strings [Amadini et al.
2017], HAMPI [Kiezun et al. 2012], and S3 [Trinh et al. 2014]. Most modern string solvers
provide support of concatenation and regular constraints. The push (e.g. see [Ganesh and
Berzish 2016; Ganesh et al. 2012; Kiezun et al. 2012; Lin and Barceló 2016; Saxena et al.
2010; Trinh et al. 2014]) towards incorporating other functions—e.g. length, string-number
conversions, replace, replaceAll—in a string theory is an important theme in the area, owing
to the desire to be able to reason about complex real-world string-manipulating programs.
These functions, among others, are now part of the SMT-LIB Unicode Strings standard.7
To the best of our knowledge, there is currently no solver that supports RegEx features
like greedy/lazing matching or capturing groups (apart from our own solver OSTRICH).
This was remarked in [Loring et al. 2019], where the authors try to amend the situation by
developing ExpoSE — a dynamic symbolic execution engine — that maps path constraints
in JavaScript to Z3. The strength of ExpoSE is in a thorough modelling of RegEx features,
some of which (including backreferences) we do not cover in our string constraint language
and string solver OSTRICH. However, the features that we do not cover are also rare in
7 See http://smtlib.cs.uiowa.edu/theories-UnicodeStrings.shtml

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 29 -->

Solving String Constraints with Regex-Dependent Functions

45:29

practice, according to [Loring et al. 2019] — in fact, around 75% of all the RegEx expressions
found in their benchmarks across 415,487 NPM packages can be covered in our fragment.
The strength of OSTRICH against ExpoSE is in a substantial improvement in performance
(by 30–50 fold) and precision. ExpoSE does not terminate even for simple examples (e.g. for
Example 1.1 and Example 1.2), which can be solved by our solver within a few seconds.
For string constraint solving in general, we refer the readers to the recent survey [Amadini
2020]. In this work, we consider a string constraint language which is undecidable in general,
and propose a propagation-based calculus to solve the constraints. However, we also identified
a straight-line fragment including concatenation, extract, replace(All) which turns to be
decidable. Our decision procedure extends the backward-reasoning approach in [Chen et al.
2019], where only standard one-way and two-way finite-state transducers were considered.
8

CONCLUSION

The challenge of reasoning about string constraints with regular expressions stems from
functions like match and replace that exploit features like capturing groups, not to mention
the subtle deterministic (greedy/lazy) matching. Our results provide the first string solving
method that natively supports and effectively handles RegEx, which is a large order of
magnitude faster than the symbolic execution engine ExpoSE [Loring et al. 2019] tailored
to constraints with regular expressions, which is at the moment the only available method
for reasoning about string constraints with regular expressions. Our solver OSTRICH relies
on two ingredients: (i) Prioritized Streaming String Transducers (used to capture subtle
non-standard semantics of RegEx, while being amenable to analysis), and (ii) a sequent
calculus that exploits nice closure and algorithmic properties of PSST, and performs a kind
of propagation of regular constraints by means of taking post-images or pre-images. We have
also carried out thorough empirical studies to validate our formalization of RegEx as PSST
with respect to JavaScript semantics, as well as to measure the performance of our solver.
Finally, although the satisfiability of the constraint language is undecidable, we have also
shown that our solver terminates (and therefore is complete) for the straight-line fragment.
Several avenues for future work are obvious. Firstly, it would be interesting to see how
ExpoSE could be used in combination with our solver OSTRICH. This would essentially lift
OSTRICH to a symbolic execution engine (i.e. working at the level of programs).
Secondly, we could incorporate other features of RegEx that are not in our framework, e.g.,
lookahead and backreferences. To handle lookahead, we may consider alternating variants of
PSSTs. Alternating automata [Chandra et al. 1981] are effectively able to branch and run
parallel checks on the input. We will need to model the subtle interplay between lookahead and
references. Backreferences could be handled by allowing some inspection of variable contents
during transducer runs. There is some precedent for this in higher-order automata [Engelfriet
1991; Masilov 1976], whose stacks non-trivially store and use data. However, the pre-image
of string functions supporting RegEx with backreferences will not be regular in general, and
emptiness of intersection of RegEx with backreferences is undecidable [Carle and Narendran
2009]. Decidability can be recovered in some cases [Freydenberger and Schmid 2019]. We
may study these cases or look for incomplete algorithms.
Finally, since strings do not live in isolation in a real-world program, there is a real need
to also extend our work with other data types, in particular the integer data type.
ACKNOWLEDGMENTS
We thank Johannes Kinder and anonymous referees for their helpful feedback. T. Chen
is supported by Birkbeck BEI School under Grant No. ARTEFACT, National Natural
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 30 -->

45:30

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Science Foundation of China under Grant No. 62072309, The State Key Laboratory of
Novel Software Technology, Nanjing University under Grant No. KFKT2018A16. M. Hague
and A. Flores-Lamas are supported by the Engineering and Physical Sciences Research
Council under Grant No. EP/T00021X/1 S. Kan and A. Lin is supported by the European
Research Council (ERC) under the European Union’s Horizon 2020 research and innovation
programme (grant agreement no 759969). P. Rümmer is supported by the Swedish Research
Council (VR) under grant 2018-04727, by the Swedish Foundation for Strategic Research
(SSF) under the project WebSec (Ref. RIT17-0011), by the Wallenberg project UPDATE,
and by grants from Microsoft and Amazon Web Services. Z. Wu is supported by the National
Natural Science Foundation of China under Grant No. 61872340.
REFERENCES
Parosh Aziz Abdulla, Mohamed Faouzi Atig, Yu-Fang Chen, Bui Phi Diep, Lukás Holı́k, Ahmed Rezine, and
Philipp Rümmer. 2017. Flatten and conquer: a framework for efficient analysis of string constraints. In Proceedings of the 38th ACM SIGPLAN Conference on Programming Language Design and Implementation,
PLDI 2017, Barcelona, Spain, June 18-23, 2017. 602–617. https://doi.org/10.1145/3062341.3062384
Parosh Aziz Abdulla, Mohamed Faouzi Atig, Yu-Fang Chen, Bui Phi Diep, Lukás Holı́k, Ahmed Rezine, and
Philipp Rümmer. 2018. Trau: SMT solver for string constraints. In 2018 Formal Methods in Computer
Aided Design, FMCAD 2018, Austin, TX, USA, October 30 - November 2, 2018, Nikolaj Bjørner and
Arie Gurfinkel (Eds.). IEEE, 1–5. https://doi.org/10.23919/FMCAD.2018.8602997
Parosh Aziz Abdulla, Mohamed Faouzi Atig, Yu-Fang Chen, Lukás Holı́k, Ahmed Rezine, Philipp Rümmer,
and Jari Stenman. 2014. String Constraints for Verification. In CAV. 150–166.
Parosh Aziz Abdulla, Mohamed Faouzi Atig, Bui Phi Diep, Lukás Holı́k, and Petr Janku. 2019. Chain-Free
String Constraints. In Automated Technology for Verification and Analysis - 17th International Symposium,
ATVA 2019, Taipei, Taiwan, October 28-31, 2019, Proceedings. 277–293. https://doi.org/10.1007/978-3030-31784-3 16
Rajeev Alur and Pavol Cerný. 2010. Expressiveness of streaming string transducers. In IARCS Annual
Conference on Foundations of Software Technology and Theoretical Computer Science, FSTTCS 2010,
December 15-18, 2010, Chennai, India. 1–12.
Rajeev Alur and Jyotirmoy V. Deshmukh. 2011. Nondeterministic Streaming String Transducers. In Automata,
Languages and Programming - 38th International Colloquium, ICALP 2011, Zurich, Switzerland, July
4-8, 2011, Proceedings, Part II (Lecture Notes in Computer Science, Vol. 6756), Luca Aceto, Monika
Henzinger, and Jirı́ Sgall (Eds.). Springer, 1–20.
Roberto Amadini. 2020. A Survey on String Constraint Solving. CoRR abs/2002.02376 (2020).
arXiv:2002.02376 https://arxiv.org/abs/2002.02376
Roberto Amadini, Mak Andrlon, Graeme Gange, Peter Schachte, Harald Søndergaard, and Peter J. Stuckey.
2019. Constraint Programming for Dynamic Symbolic Execution of JavaScript. In Integration of Constraint
Programming, Artificial Intelligence, and Operations Research - 16th International Conference, CPAIOR
2019, Thessaloniki, Greece, June 4-7, 2019, Proceedings (Lecture Notes in Computer Science, Vol. 11494),
Louis-Martin Rousseau and Kostas Stergiou (Eds.). Springer, 1–19. https://doi.org/10.1007/978-3-03019212-9 1
Roberto Amadini, Graeme Gange, Peter J. Stuckey, and Guido Tack. 2017. A Novel Approach to String
Constraint Solving. In Principles and Practice of Constraint Programming - 23rd International Conference,
CP 2017, Melbourne, VIC, Australia, August 28 - September 1, 2017, Proceedings (Lecture Notes in
Computer Science, Vol. 10416), J. Christopher Beck (Ed.). Springer, 3–20. https://doi.org/10.1007/9783-319-66158-2 1
Martin Berglund, Frank Drewes, and Brink van der Merwe. 2014. Analyzing Catastrophic Backtracking
Behavior in Practical Regular Expression Matching. In Proceedings 14th International Conference on
Automata and Formal Languages, AFL 2014, Szeged, Hungary, May 27-29, 2014 (EPTCS, Vol. 151),
Zoltán Ésik and Zoltán Fülöp (Eds.). 109–123. https://doi.org/10.4204/EPTCS.151.7
Martin Berglund and Brink van der Merwe. 2017a. On the semantics of regular expression parsing in the
wild. Theoretical Computer Science 679 (2017), 69 – 82.
Martin Berglund and Brink van der Merwe. 2017b. Regular Expressions with Backreferences Re-examined.
In Proceedings of the Prague Stringology Conference 2017, Prague, Czech Republic, August 28-30, 2017,
Jan Holub and Jan Zdárek (Eds.). Department of Theoretical Computer Science, Faculty of Information
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 31 -->

Solving String Constraints with Regex-Dependent Functions

45:31

Technology, Czech Technical University in Prague, 30–41. http://www.stringology.org/event/2017/p04.
html
Murphy Berzish, Vijay Ganesh, and Yunhui Zheng. 2017. Z3str3: A string solver with theory-aware heuristics.
In 2017 Formal Methods in Computer Aided Design, FMCAD 2017, Vienna, Austria, October 2-6, 2017.
55–59.
Berzish, Murphy. 2021. Z3str4: A Solver for Theories over Strings. Ph.D. Dissertation. http://hdl.handle.
net/10012/17102
Diep Bui and contributors. 2019. Z3-Trau. https://github.com/diepbp/z3-trau.
Tevfik Bultan and contributors. 2015. ABC string solver. https://github.com/vlab-cs-ucsb/ABC.
Cezar C^
ampeanu, Kai Salomaa, and Sheng Yu. 2003. A Formal Study Of Practical Regular Expressions.
Int. J. Found. Comput. Sci. 14, 6 (2003), 1007–1018.
Benjamin Carle and Paliath Narendran. 2009. On Extended Regular Expressions. In Language and Automata
Theory and Applications, Third International Conference, LATA 2009, Tarragona, Spain, April 2-8,
2009. Proceedings (Lecture Notes in Computer Science, Vol. 5457), Adrian-Horia Dediu, Armand-Mihai
Ionescu, and Carlos Martı́n-Vide (Eds.). Springer, 279–289.
Olivier Carton, Christian Choffrut, and Serge Grigorieff. 2006. Decision problems among the main subfamilies
of rational relations. ITA 40, 2 (2006), 255–275. https://doi.org/10.1051/ita:2006005
Ashok K. Chandra, Dexter Kozen, and Larry J. Stockmeyer. 1981. Alternation. J. ACM 28, 1 (1981),
114–133. https://doi.org/10.1145/322234.322243
Taolue Chen, Yan Chen, Matthew Hague, Anthony W. Lin, and Zhilin Wu. 2018. What is decidable about
string constraints with the ReplaceAll function. PACMPL 2, POPL (2018), 3:1–3:29.
Taolue Chen, Matthew Hague, Zhilei Han, Denghang Hu, Alejandro Flores-Lamas, Anthony Lin, Shanglong
Kan, Philipp Ruemmer, and Zhilin Wu. 2021. Solving String Constraints With Regex-Dependent Functions
Through Transducers With Priorities And Variables. CoRR (2021).
Taolue Chen, Matthew Hague, Anthony W. Lin, Philipp Rümmer, and Zhilin Wu. 2019. Decision Procedures
for Path Feasibility of String-Manipulating Programs with Complex Operations. PACMPL 3, POPL,
Article 49 (Jan. 2019), 30 pages. https://doi.org/10.1145/3290362
Arlen Cox and Jason Leasure. 2017. Model Checking Regular Language Constraints. arXiv:1708.09073 [cs.LO]
James C. Davis, Louis G. Michael IV, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. 2019.
Why Aren’t Regular Expressions a Lingua Franca? An Empirical Study on the Re-Use and Portability
of Regular Expressions. In Proceedings of the 2019 27th ACM Joint Meeting on European Software
Engineering Conference and Symposium on the Foundations of Software Engineering (ESEC/FSE 2019).
Association for Computing Machinery, New York, NY, USA, 443–454.
Leonardo Mendonça de Moura and Nikolaj Bjørner. 2008. Z3: An Efficient SMT Solver. In Tools and
Algorithms for the Construction and Analysis of Systems, 14th International Conference, TACAS 2008,
Held as Part of the Joint European Conferences on Theory and Practice of Software, ETAPS 2008,
Budapest, Hungary, March 29-April 6, 2008. Proceedings. 337–340. https://doi.org/10.1007/978-3-54078800-3 24
Joost Engelfriet. 1991. Iterated Stack Automata and Complexity Classes. Inf. Comput. 95, 1 (1991), 21–75.
https://doi.org/10.1016/0890-5401(91)90015-T
Dominik D. Freydenberger. 2013. Extended Regular Expressions: Succinctness and Decidability. Theory
Comput. Syst. 53, 2 (2013), 159–193.
Dominik D. Freydenberger and Markus L. Schmid. 2019. Deterministic regular expressions with backreferences. J. Comput. Syst. Sci. 105 (2019), 1–39.
Vijay Ganesh and Murphy Berzish. 2016. Undecidability of a Theory of Strings, Linear Arithmetic over
Length, and String-Number Conversion. CoRR abs/1605.09442 (2016). http://arxiv.org/abs/1605.09442
Vijay Ganesh, Mia Minnes, Armando Solar-Lezama, and Martin C. Rinard. 2012. Word Equations with
Length Constraints: What’s Decidable?. In Hardware and Software: Verification and Testing - 8th
International Haifa Verification Conference, HVC 2012, Haifa, Israel, November 6-8, 2012. Revised
Selected Papers. 209–226. https://doi.org/10.1007/978-3-642-39611-3 21
Gerhard Gentzen. 1935. Untersuchungen über das Logische Schliessen. Mathematische Zeitschrift 39 (1935),
176–210, 405–431. English translation, “Investigations into Logical Deduction,” in [Szabo 1969].
John Harrison. 2009. Handbook of Practical Logic and Automated Reasoning. Cambridge University Press.
I–XIX, 1–681 pages.
Lukás Holı́k, Petr Janku, Anthony W. Lin, Philipp Rümmer, and Tomás Vojnar. 2018. String constraints
with concatenation and transducers solved efficiently. PACMPL 2, POPL (2018), 4:1–4:32. https:
//doi.org/10.1145/3158092
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 32 -->

45:32

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Pieter Hooimeijer, Benjamin Livshits, David Molnar, Prateek Saxena, and Margus Veanes. 2011. Fast and
Precise Sanitizer Analysis with BEK. In USENIX Security Symposium.
John E. Hopcroft and Jeffrey D. Ullman. 1979. Introduction to Automata Theory, Languages and Computation. Addison-Wesley.
Adam Kiezun, Vijay Ganesh, Shay Artzi, Philip J. Guo, Pieter Hooimeijer, and Michael D. Ernst. 2012.
HAMPI: A solver for word equations over strings, regular expressions, and context-free grammars. ACM
Trans. Softw. Eng. Methodol. 21, 4 (2012), 25:1–25:28.
Quang Loc Le and Mengda He. 2018. A Decision Procedure for String Logic with Quadratic Equations,
Regular Expressions and Length Constraints. In Programming Languages and Systems - 16th Asian
Symposium, APLAS 2018, Wellington, New Zealand, December 2-6, 2018, Proceedings (Lecture Notes in
Computer Science, Vol. 11275), Sukyoung Ryu (Ed.). Springer, 350–372. https://doi.org/10.1007/978-3030-02768-1 19
Tianyi Liang, Andrew Reynolds, Cesare Tinelli, Clark Barrett, and Morgan Deters. 2014. A DPLL(T)
Theory Solver for a Theory of Strings and Regular Expressions. In CAV. 646–662.
Anthony W. Lin and Pablo Barceló. 2016. String Solving with Word Equations and Transducers: Towards a
Logic for Analysing Mutation XSS (POPL ’16). ACM, 123–136.
Blake Loring, Duncan Mitchell, and Johannes Kinder. 2017. ExpoSE: practical symbolic execution of
standalone JavaScript. In Proceedings of the 24th ACM SIGSOFT International SPIN Symposium on
Model Checking of Software, Santa Barbara, CA, USA, July 10-14, 2017, Hakan Erdogmus and Klaus
Havelund (Eds.). ACM, 196–199. https://doi.org/10.1145/3092282.3092295
Blake Loring, Duncan Mitchell, and Johannes Kinder. 2019. Sound regular expression semantics for dynamic
symbolic execution of JavaScript. In Proceedings of the 40th ACM SIGPLAN Conference on Programming
Language Design and Implementation, PLDI 2019, Phoenix, AZ, USA, June 22-26, 2019. ACM, 425–438.
A. N. Masilov. 1976. Multilevel magazine automata. Probl. Peredachi Inf. 12, 1 (1976), 55–62.
Louis G. Michael, James Donohue, James C. Davis, Dongyoon Lee, and Francisco Servant. 2019. Regexes
Are Hard: Decision-Making, Difficulties, and Risks in Programming Regular Expressions. In Proceedings
of the 34th IEEE/ACM International Conference on Automated Software Engineering (ASE ’19). IEEE
Press, 415–426. https://doi.org/10.1109/ASE.2019.00047
Robert Nieuwenhuis, Albert Oliveras, and Cesare Tinelli. 2006. Solving SAT and SAT Modulo Theories:
From an abstract Davis-Putnam-Logemann-Loveland procedure to DPLL(T). J. ACM 53, 6 (2006),
937–977.
Prateek Saxena, Devdatta Akhawe, Steve Hanna, Feng Mao, Stephen McCamant, and Dawn Song. 2010. A
Symbolic Execution Framework for JavaScript. In 31st IEEE Symposium on Security and Privacy, S&P
2010, 16-19 May 2010, Berleley/Oakland, California, USA. 513–528. https://doi.org/10.1109/SP.2010.38
Markus L. Schmid. 2016. Characterising REGEX languages by regular languages equipped with factorreferencing. Inf. Comput. 249 (2016), 1–17.
Joseph D. Scott, Pierre Flener, Justin Pearson, and Christian Schulte. 2017. Design and Implementation of
Bounded-Length Sequence Variables. In Integration of AI and OR Techniques in Constraint Programming
- 14th International Conference, CPAIOR 2017, Padua, Italy, June 5-8, 2017, Proceedings (Lecture
Notes in Computer Science, Vol. 10335), Domenico Salvagnin and Michele Lombardi (Eds.). Springer,
51–67. https://doi.org/10.1007/978-3-319-59776-8 5
Cristian-Alexandru Staicu and Michael Pradel. 2018. Freezing the Web: A Study of ReDoS Vulnerabilities
in Javascript-Based Web Servers. In Proceedings of the 27th USENIX Conference on Security Symposium
(Baltimore, MD, USA) (SEC’18). USENIX Association, USA, 361–376.
M. E. Szabo (Ed.). 1969. The Collected Papers of Gerhard Gentzen. North-Holland, Amsterdam.
Ken Thompson. 1968. Regular Expression Search Algorithm. Commun. ACM 11, 6 (1968), 419–422.
https://doi.org/10.1145/363347.363387
Minh-Thai Trinh, Duc-Hiep Chu, and Joxan Jaffar. 2014. S3: A Symbolic String Solver for Vulnerability
Detection in Web Applications. In CCS. 1232–1243.
Minh-Thai Trinh, Duc-Hiep Chu, and Joxan Jaffar. 2016. Progressive Reasoning over Recursively-Defined
Strings. In Computer Aided Verification - 28th International Conference, CAV 2016, Toronto, ON,
Canada, July 17-23, 2016, Proceedings, Part I. Springer, 218–240.
Hung-En Wang, Tzung-Lin Tsai, Chun-Han Lin, Fang Yu, and Jie-Hong R. Jiang. 2016. String Analysis
via Automata Manipulation with Logic Circuit Representation. In Computer Aided Verification - 28th
International Conference, CAV 2016, Toronto, ON, Canada, July 17-23, 2016, Proceedings, Part I
(Lecture Notes in Computer Science, Vol. 9779). Springer, 241–260. https://doi.org/10.1007/978-3-31941528-4
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 33 -->

Solving String Constraints with Regex-Dependent Functions

45:33

Hung-En Wang, Shih-Yu Chen, Fang Yu, and Jie-Hong R. Jiang. 2018. A Symbolic Model Checking
Approach to the Analysis of String and Length Constraints. In Proceedings of the 33rd ACM/IEEE
International Conference on Automated Software Engineering (ASE 2018). ACM, 623–633. https:
//doi.org/10.1145/3238147.3238189
Fang Yu, Muath Alkhalaf, and Tevfik Bultan. 2010. Stranger: An Automata-Based String Analysis Tool for
PHP. In TACAS. 154–157. Benchmark can be found at http://www.cs.ucsb.edu/∼vlab/stranger/.
Fang Yu, Muath Alkhalaf, Tevfik Bultan, and Oscar H. Ibarra. 2014. Automata-based Symbolic String
Analysis for Vulnerability Detection. Form. Methods Syst. Des. 44, 1 (2014), 44–70.
Yunhui Zheng, Vijay Ganesh, Sanu Subramanian, Omer Tripp, Julian Dolby, and Xiangyu Zhang. 2015. Effective Search-Space Pruning for Solvers of String Equations, Regular Expressions and Length Constraints.
In Computer Aided Verification - 27th International Conference, CAV 2015, San Francisco, CA, USA,
July 18-24, 2015, Proceedings, Part I. Springer, 235–254. https://doi.org/10.1007/978-3-319-21690-4 14
Yunhui Zheng, Xiangyu Zhang, and Vijay Ganesh. 2013. Z3-str: a Z3-based string solver for web application
analysis. In ESEC/SIGSOFT FSE. 114–124.
Qizhen Zhu, Hitoshi Akama, and Yasuhiko Minamide. 2019. Solving String Constraints with Streaming String
Transducers. Journal of Information Processing 27 (2019), 810–821. https://doi.org/10.2197/ipsjjip.27.810

A
A.1

APPENDIX
Construction of PSST from RegEx

Case 𝑒 = ∅ (see Figure 16). 𝒯∅ = ({𝑞∅,0 }, Σ, {𝑥∅ }, 𝛿∅ , 𝜏∅ , 𝐸∅ , 𝑞∅,0 , (∅, ∅)), where there are
no transitions out of 𝑞∅,0 , namely, 𝛿∅ (𝑞∅,0 , 𝑎) = () for every 𝑎 ∈ Σ, 𝜏∅ (𝑞∅,0 ) = ((); ()), and 𝐸∅
is vacuous here.
Case 𝑒 = 𝜀 (see Figure 16). 𝒯𝜀 = ({𝑞𝜀,0 , 𝑓𝜀,0 }, Σ, {𝑥𝜀 }, 𝛿𝜀 , 𝜏𝜀 , 𝐸𝜀 , 𝑞𝜀,0 , ({𝑓𝜀,0 }, ∅)), where
𝜏𝜀 (𝑞𝜀,0 ) = ((𝑓𝜀,0 ); ()), and 𝐸𝜀 (𝑞𝜀,0 , 𝜀, 𝑓𝜀,0 )(𝑥) = 𝜀. Note 𝐹2 = ∅ here.
Case 𝑒 = 𝑎 (see Figure 16). 𝒯𝑎 = ({𝑞𝑎,0 , 𝑞𝑎,1 , 𝑓𝑎,0 }, Σ, {𝑥𝑎 }, 𝛿𝑎 , 𝜏𝑎 , 𝐸𝑎 , 𝑞𝑎,0 , (∅, {𝑓𝑎,0 })),
where 𝜏𝑎 (𝑞𝑎,0 ) = ((𝑞𝑎,1 ); ()), 𝛿𝑎 (𝑞𝑎,1 , 𝑎) = (𝑓𝑎,0 ), 𝐸𝑎 (𝑞𝑎,0 , 𝜀, 𝑞𝑎,1 )(𝑥𝑎 ) = 𝜀, and 𝐸𝑎 (𝑞𝑎,1 , 𝑎, 𝑓𝑎,0 )(𝑥𝑎 ) =
𝑥𝑎 𝑎. Note 𝐹1 = ∅ here.
T∅

Tε

Ta

q∅,0

qε,0

qa,0

ε xε := ε

ε xa := ε

fε,0

qa,1
a xa := xa a
fa,0

Fig. 16. The PSST 𝒯∅ , 𝒯𝜀 , and 𝒯𝑎
−
Case 𝑒 = [𝑒+?
1 ]. Then 𝒯𝑒 is constructed from 𝒯𝑒1 and 𝒯[𝑒*? ] , similarly to the aforementioned
1
construction of 𝒯[𝑒+ ] .
1

{𝑚 ,𝑚 }?
Case 𝑒 = [𝑒1 1 2 ] for 1 ≤ 𝑚1 < 𝑚2 (see Figure 17). Then 𝒯𝑒 is constructed as the
{𝑚 }
{1,𝑚 −𝑚 }?
{1,𝑚 −𝑚 }?
concatenation of 𝒯𝑒1 1 and 𝒯𝑒1 2 1 , where 𝒯𝑒1 2 1 is illustrated in Figure 17,
{1,𝑚2 −𝑚1 }
which is the same as 𝒯𝑒1
in Figure 9, except that the priorities of the 𝜀-transition
(1)
′
from 𝑞𝑒1 ,0 to 𝑓0 has the highest priority and the priorities of the 𝜀-transitions out of each
(𝑖)
(𝑖)
𝑓𝑒1 ,2 ∈ 𝐹𝑒1 ,2 to 𝑓1′ are swapped.

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 34 -->

45:34

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu
m2 − m1 copies of Te1
ε

ε

ε

(1)

qe1 ,0

(1)

T e1
(1)

(1)

fe1 ,1 ∈ Fe1 ,1

ε

(2)

qe1 ,0

(2)

···
(1)

T e1

(1)

fe1 ,2 ∈ Fe1 ,2

(2)

(m −m1 )

T e1 2

···
(2)

fe1 ,1 ∈ Fe1 ,1

(2)

ε

f00

...

(2)

fe1 ,2 ∈ Fe1 ,2

(m −m1 )

fe1 ,12

ε

(m −m1 )

qe1 ,02

···
(m −m1 )

∈ Fe1 ,12

(m −m1 )

fe1 ,22

(m −m1 )

∈ Fe1 ,22

ε

f10

{1,𝑚2 −𝑚1 }?

Fig. 17. The PSST 𝒯𝑒1

A.2

From extract, replace and replaceAll to PSSTs

Lemma 4.7. The satisfiability of STR reduces to the satisfiability of boolean combinations
of formulas of the form 𝑧 = 𝑥 · 𝑦, 𝑦 = 𝒯 (𝑥), and 𝑥 ∈ 𝒜, where 𝒯 is a PSST and 𝒜 is an FA.
The proof is in two steps: first we remove $0, $← , and $→ , then we encode the remaining
string functions with PSSTs.
A.2.1 Removing Special References. The first step in our proof is to remove the special
references $0, $← , and $→ from the replacement strings. These can be replaced in a series of
steps, leaving only PSST transductions and replacement strings with only simple references
($𝑖). We will just consider replaceAll as replace is almost identical.
First, to remove $0, suppose we have a statement 𝑦 := replaceAllpat,rep (𝑥) with $0
in rep. We simply substitute 𝑦 := replaceAllpat′ ,rep′ (𝑥) where pat′ = (pat), and rep′ =
rep[$1/$0, $2/$1, . . . , $(𝑘 + 1)/$𝑘]. That is, we make the complete match an explicit (first)
capture, which shifts the indexes of the remaining capturing groups by 1.
Now suppose we have a statement 𝑦 := replaceAllpat,rep (𝑥) with $← or $→ in rep. We replace
it with the following statements, explained below, where 𝑦1 , . . . , 𝑦5 are fresh variables.
𝑦1 := replaceAll(pat),⟨$1⟩ (𝑥);
𝑦2 := 𝒯⟨ (𝑦1 );
𝑦3 := 𝒯rev (𝑦2 );
𝑦4 := 𝒯⟩ (𝑦3 );
𝑦5 := 𝒯rev (𝑦4 );
𝑦 := replaceAllpat′ ,rep′ (𝑦5 )
The first step is to mark the matched parts of the string with ⟨ and ⟩ brackets (where ⟨
and ⟩ are not part of the main alphabet). This is achieved by the first replaceAll.
Next, we use a PSST 𝒯⟨ that passes over the marked word. This is a copyful PSST that
simply stores the word read so far into a variable 𝑋, except for the ⟨ and ⟩ characters. It also
has an output variable 𝑂, to which it also copies each character directly, except ⟨. When
it encounters ⟨ it appends to 𝑂 the string ⟨𝑋⟨. That is, it puts the entire string preceding
each ⟨ into the output, surrounded by ⟨ at the start and end. This is copyful since 𝑋 will be
copied to both 𝑋 and 𝑂 in this step. For example, suppose the input string were 𝑎𝑏⟨𝑐⟩𝑑⟨𝑒⟩𝑓 ,
the output of 𝒯⟨ would be 𝑎𝑏⟨𝑎𝑏⟨𝑐⟩𝑑⟨𝑎𝑏𝑐𝑑⟨𝑒⟩𝑓 . We have underlined the strings inserted for
readability.
The next step is to do the same for ⟩. To achieve this we first reverse the string so that a
PSST can read the end of the string first. A similar transduction to 𝒯⟨ is performed before
the string is reversed again. In our example the resulting string is 𝑎𝑏⟨𝑎𝑏⟨𝑐⟩𝑑𝑒𝑓 ⟩𝑑⟨𝑎𝑏𝑐𝑑⟨𝑒⟩𝑓 ⟩𝑓 .
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 35 -->

Solving String Constraints with Regex-Dependent Functions

45:35

Finally, we have 𝑦 := replaceAllpat′ ,rep′ (𝑦5 ) where pat′ = ⟨(Σ*? )⟨pat⟩(Σ*? )⟩, and
rep′ = rep[$1/$← , $2/$1, . . . $(𝑘 + 1)/$𝑘, $(𝑘 + 2)/$→ ] .

That is, by inserting the preceding and succeeding text directly next to each match, we can
use simple references $𝑖 instead of $← and $→ .
A.2.2 Encoding string functions as PSSTs. Once $0, $← , and $→ are removed from the
replacement strings, then the string functions can be replaced by PSSTs.
Lemma A.1. For each string function 𝑓 = extract𝑖,𝑒 , replacepat,rep , or replaceAllpat,rep
without $← or $→ in the replacements strings, a PSST 𝒯𝑓 can be constructed such that
ℛ𝑓 = {(𝑤, 𝑤′ ) | 𝑤′ = 𝑓 (𝑤)}.

Proof. The extract𝑖,𝑒 can function be defined by a PSST 𝒯𝑖,𝑒 obtained from the PSST 𝒯𝑒
(see Section 4.2) by removing all the string variables, except the string variable 𝑥𝑒′ , where 𝑒′
is the subexpression of 𝑒 corresponding to the 𝑖th capturing group, and setting the output
expression of the final states as 𝑥𝑒′ .
Next, we give the construction of the PSST for replaceAllpat,rep where all the references in
rep are of the form $𝑖.
Recall rep = 𝑤1 $𝑖1 𝑤2 · · · 𝑤𝑘 $𝑖𝑘 𝑤𝑘+1 . Let 𝑒′𝑖1 , . . . , 𝑒′𝑖𝑘 denote the subexpressions of pat
corresponding to the 𝑖1 th, . . ., 𝑖𝑘 th capturing groups of pat. Then 𝒯replaceAllpat,rep = (𝑄pat ∪{𝑞0′ },
Σ, 𝑋 ′ , 𝛿 ′ , 𝜏 ′ , 𝐸 ′ , 𝑞0′ , 𝐹 ′ ) where
∙ 𝑞0′ ̸∈ 𝑄pat ,
∙ 𝑋 ′ = {𝑥0 } ∪ 𝑋pat ,
∙ 𝐹 ′ (𝑞0′ ) = 𝑥0 , and 𝐹 ′ (𝑞 ′ ) is undefined for every 𝑞 ′ ∈ 𝑄pat ,
∙ 𝛿 ′ comprises the transitions in 𝛿pat , and the transition 𝛿 ′ (𝑞0′ , 𝑎) = (𝑞0′ ) for 𝑎 ∈ Σ,
∙ 𝜏 ′ comprises the transitions in 𝜏pat , the transitions 𝜏 ′ (𝑞0′ ) = ((𝑞pat,0 ); ()), 𝜏 ′ (𝑓pat,1 ) =
((𝑞0′ ); ()) and 𝜏 ′ (𝑓pat,2 ) = ((𝑞0′ ); ()) for 𝑓pat,1 ∈ 𝐹pat,1 and 𝑓pat,2 ∈ 𝐹pat,2 ,
∙ 𝐸 ′ inherits 𝐸pat , and includes the assignments 𝐸 ′ (𝑞0′ , 𝑎, 𝑞0′ )(𝑥0 ) = 𝑥0 𝑎 for 𝑎 ∈ Σ,
𝐸 ′ (𝑓, 𝜀, 𝑞0′ )(𝑥0 ) = 𝑥0 rep[𝑥𝑒′𝑖 /$𝑖1 , . . . , 𝑥𝑒′𝑖 /𝑖𝑘 ] and 𝐸 ′ (𝑓, 𝜀, 𝑞0′ )(𝑥) = null for every 𝑓 ∈
1
𝑘
𝐹pat,1 ∪ 𝐹pat,2 and 𝑥 ∈ 𝑋pat .
□
The construction of the PSST for replacepat,rep is similar and illustrated in Fig. 18. The
details are omitted.
x0 := x0 a
a

Tpat

x0 := x0 a
a

fpat,1 ∈ Fpat,1
ε

q00

qpat,0

···

x0 := x0 rep[xe0 /$i1 , · · · , xe0 /$ik ]
i1

ik

ε

f00 F (f00 ) = x0

ε

fpat,2 ∈ Fpat,2

x0 := x0 rep[xe0 /$i1 , · · · , xe0 /$ik ]
i1

ik

ε

Fig. 18. The PSST 𝒯replacepat,rep

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 36 -->

45:36

A.3

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Proof of Lemma 5.5

Lemma 5.5. Given a PSST 𝒯 = (𝑄𝑇 , Σ, 𝑋, 𝛿𝑇 , 𝜏𝑇 , 𝐸𝑇 , 𝑞0,𝑇 , 𝐹𝑇 ) and an FA 𝒜 =
(𝑄𝐴 , Σ, 𝛿𝐴 , 𝑞0,𝐴 , 𝐹𝐴 ), we can compute an FA ℬ = (𝑄𝐵 , Σ, 𝛿𝐵 , 𝑞0,𝐵 , 𝐹𝐵 ) in exponential time
such that L (ℬ) = ℛ−1
𝒯 (L (𝒜)).
We prove Lemma 5.5 in the sequel.
Let 𝒯 = (𝑄𝑇 , Σ, 𝑋, 𝛿𝑇 , 𝜏𝑇 , 𝐸𝑇 , 𝑞0,𝑇 , 𝐹𝑇 ) be a PSST and 𝒜 = (𝑄𝐴 , Σ, 𝛿𝐴 , 𝑞0,𝐴 , 𝐹𝐴 ) be an
FA. Without loss of generality, we assume that 𝒜 contains no 𝜀-transitions. For convenience,
(𝑎)
we use ℰ(𝜏𝑇 ) to denote {(𝑞, 𝑞 ′ ) | 𝑞 ′ ∈ 𝜏𝑇 (𝑞)}. For convenience, for 𝑎 ∈ Σ, we use 𝛿𝐴 to
′
′
denote the relation {(𝑞, 𝑞 ) | (𝑞, 𝑎, 𝑞 ) ∈ 𝛿𝐴 }.
To illustrate the intuition of the proof of Lemma 5.5, let us start with the following natural
idea of firstly constructing a PFA ℬ for the pre-image: ℬ simulates a run of 𝒯 on 𝑤, and, for
each 𝑥 ∈ 𝑋, records an 𝒜-abstraction of the string stored in 𝑥, that is, the set of state pairs
(𝑝, 𝑞) ∈ 𝑄𝐴 × 𝑄𝐴 such that starting from 𝑝, 𝒜 can reach 𝑞 after reading the string stored in
𝑥. Specifically, the states of ℬ are of the form (𝑞, 𝜌) with 𝑞 ∈ 𝑄 and 𝜌 ∈ (𝒫(𝑄𝐴 × 𝑄𝐴 ))𝑋 .
Moreover, the priorities of ℬ inherit those of 𝒯 . The PFA ℬ is then transformed to an
equivalent FA by simply dropping all priorities. We refer to this FA as ℬ ′ .
Nevertheless, it turns out that this construction is flawed: A string 𝑤 is in ℛ−1
𝒯 (L (𝒜)) iff
the (unique) accepting run of 𝒯 on 𝑤 produces an output 𝑤′ that is accepted by 𝒜. However,
a string 𝑤 is accepted by ℬ ′ iff there is a run of 𝒯 on 𝑤, not necessarily of the highest
priority, producing an output 𝑤′ that is accepted by 𝒜.
While the aforementioned natural idea does not work, we choose to construct an FA ℬ
that simulates the accepting run of 𝒯 on 𝑤, and, for each 𝑥 ∈ 𝑋, records an 𝒜-abstraction
of the string stored in 𝑥, that is, the set of state pairs (𝑝, 𝑞) ∈ 𝑄𝐴 × 𝑄𝐴 such that starting
from 𝑝, 𝒜 can reach 𝑞 after reading the string stored in 𝑥. To simulate the accepting run of
𝒯 , it is necessary to record all the states accessible through the runs of higher priorities to
ensure the current run is indeed the accepting run of 𝒯 (of highest priority). Moreover, ℬ
also remembers the set of 𝜀-transitions of 𝒯 after the latest non-𝜀-transition to ensure that
no transition occurs twice in a sequence of 𝜀-transitions of 𝒯 .
Specifically, each state of ℬ is of the form (𝑞, 𝜌, Λ, 𝑆), where 𝑞 ∈ 𝑄𝑇 , 𝜌 ∈ (𝒫(𝑄𝐴 × 𝑄𝐴 ))𝑋 ,
Λ ⊆ ℰ(𝜏𝑇 ), and 𝑆 ⊆ 𝑄𝑇 . For a state (𝑞, 𝜌, Λ, 𝑆), our intention for 𝑆 is that the states in
it are those that can be reached in the runs of higher priorities than the current run, by
reading the same sequence of letters and applying the 𝜀-transitions as many as possible. Note
that when recording in 𝑆 all the states accessible through the runs of higher priorities, we
do not take the non-repetition of 𝜀-transitions into consideration since if a state is reachable
by a sequence of 𝜀-transitions where some 𝜀-transitions are repeated, then there exists also
a sequence of non-repeated 𝜀-transitions reaching the state. Moreover, when simulating an
𝑎-transition of 𝒯 (where 𝑎 ∈ Σ) at a state (𝑞, 𝜌, Λ, 𝑆), suppose 𝛿𝑇 (𝑞, 𝑎) = (𝑞1 , · · · , 𝑞𝑚 ) and
𝜏𝑇 (𝑞) = (𝑃1 , 𝑃2 ), then ℬ nondeterministically chooses 𝑞𝑖 and goes to the state (𝑞𝑖 , 𝜌′ , ∅, 𝑆 ′ ),
where
∙ 𝜌′ is obtained from 𝜌 and 𝐸𝑇 (𝑞, 𝜎, 𝑞𝑖 ),
∙ Λ is reset to ∅,
∙ all the states obtained from 𝑆 by applying an 𝑎 transition should be saturated by
𝜀-transitions and put into 𝑆 ′ , more precisely, all the states reachable from 𝑆 by first
applying an 𝑎-transition, then a sequence of 𝜀-transitions, should be put into 𝑆 ′ ,
∙ moreover, all the states obtained from 𝑞1 , · · · , 𝑞𝑖−1 (which are of higher priorities than
𝑞𝑖 ) by saturating with 𝜀-transitions should be put into 𝑆 ′ ,
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 37 -->

Solving String Constraints with Regex-Dependent Functions

45:37

∙ finally, all the states obtained from those in 𝑃1′ = {𝑞 ′ ∈ 𝑃1 | (𝑞, 𝑞 ′ ) ̸∈ Λ} (which
are of higher priorities than 𝑞𝑖 ) by saturating with non-Λ 𝜀-transitions first (i.e. the
𝜀-transitions that do not belong to Λ), and applying an 𝑎-transition next, finally
saturating with 𝜀-transitions again, should be put into 𝑆 ′ , (note that according to the
semantics of PSST, the 𝜀-transitions in Λ should be avoided when defining 𝑃1′ and
saturating the states in 𝑃1′ with 𝜀-transitions).
The above construction does not utilize the so-called copyless property (i.e. for each
transition 𝑡 and each variable 𝑥, 𝑥 appears at most once on the right-hand side of the
assignment for 𝑡) [Alur and Cerný 2010; Alur and Deshmukh 2011], thus it works for
general, or copyful, PSSTs [?]. It can be noted that the powerset in 𝜌 ∈ (𝒫(𝑄𝐴 × 𝑄𝐴 ))𝑋 is
required to handle copyful transductions as the contents of a variable may be used in many
different situations, each requiring a different abstraction. If the PSST is copyless, we can
instead use 𝜌 ∈ (𝑄𝐴 × 𝑄𝐴 )𝑋 . That is, each variable is used only once, and hence only one
abstraction pair is needed. The powerset construction in the transitions can be replaced by
a non-deterministic choice of the particular pair of states from 𝑄𝐴 that should be kept. This
avoids the construction being exponential in the size of 𝐴, which in turn avoids the tower of
exponential blow-up in the backwards reasoning.
For the formal construction of ℬ, we need some additional notations.
(𝑖𝑝)

∙ For 𝑆 ⊆ 𝑄𝑇 , 𝛿𝑇 (𝑆, 𝑎) = {𝑞1′ | ∃𝑞1 ∈ 𝑆, 𝑞1′ ∈ 𝛿𝑇 (𝑞1 , 𝑎)}.
(𝑖𝑝)
∙ For 𝑞 ∈ 𝑄𝑇 , if 𝜏𝑇 (𝑞) = (𝑃1 , 𝑃2 ), then 𝜏𝑇 ({𝑞}) = 𝑆 such that 𝑆 = 𝑃1 ∪ 𝑃2 . Moreover,
(︀ (𝑖𝑝) )︀*
⋃︀
(𝑖𝑝)
(𝑖𝑝)
for 𝑆 ⊆ 𝑄𝑇 , we define 𝜏𝑇 (𝑆) =
𝜏𝑇 ({𝑞}). We also use 𝜏𝑇
to denote the
𝑞∈𝑆
(︀ (𝑖𝑝) )︀*
(︀ (𝑖𝑝) )︀0
⋃︀ (︀ (𝑖𝑝) )︀𝑛
𝜀-closure of 𝒯 , namely, 𝜏𝑇
(𝑆) =
𝜏𝑇
(𝑆), where 𝜏𝑇
(𝑆) = 𝑆, and for
𝑛∈N
)︀
)︀
(︀ (𝑖𝑝) )︀𝑛+1
(︀(︀
(𝑖𝑝)
(𝑖𝑝) 𝑛
𝑛 ∈ N, 𝜏𝑇
(𝑆) = 𝜏𝑇
𝜏𝑇
(𝑆) .
(︀ (𝑖𝑝) )︀*
∙ For 𝑆 ⊆ 𝑄𝑇 and Λ ⊆ ℰ(𝜏𝑇 ), we use 𝜏𝑇 ∖Λ (𝑆) to denote the set of states reachable
from 𝑆 by sequences of 𝜀-transitions where no transitions (𝑞, 𝜀, 𝑞 ′ ) such that (𝑞, 𝑞 ′ ) ∈ Λ
are used.
∙ For 𝜌 ∈ (𝒫(𝑄𝐴 × 𝑄𝐴 ))𝑋 and 𝑠 ∈ 𝑋 → (𝑋 ∪ Σ)* , we use 𝑠(𝜌) to denote 𝜌′ that is
obtained from 𝜌 as follows: For each 𝑥 ∈ 𝑋, if 𝑠(𝑥) = 𝜀, then 𝜌′ (𝑥) = {(𝑝, 𝑝) | 𝑝 ∈ 𝑄𝐴 },
otherwise, let 𝑠(𝑥) = 𝑏1 · · · 𝑏ℓ with 𝑏𝑖 ∈ Σ ∪ 𝑋 for each 𝑖 ∈ [ℓ], then 𝜌′ (𝑥) = 𝜃1 ∘ · · · ∘ 𝜃ℓ ,
(𝑏 )
where 𝜃𝑖 = 𝛿𝐴 𝑖 if 𝑏𝑖 ∈ Σ, and 𝜃𝑖 = 𝜌(𝑏𝑖 ) otherwise, and ∘ represents the composition
of binary relations.

We are ready to present the formal construction of ℬ = (𝑄𝐵 , Σ, 𝛿𝐵 , 𝑞0,𝐵 , 𝐹𝐵 ).

∙ 𝑄𝐵 = 𝑄𝑇 × (𝒫(𝑄𝐴 × 𝑄𝐴 ))𝑋 × 𝒫(ℰ(𝜏𝑇 )) × 𝒫(𝑄𝑇 ),
∙ 𝑞0,𝐵 = (𝑞0,𝑇 , 𝜌𝜀 , ∅, ∅) where 𝜌𝜀 (𝑥) = {(𝑞, 𝑞) | 𝑞 ∈ 𝑄} for each 𝑥 ∈ 𝑋,
∙ 𝛿𝐵 comprises
– the tuples ((𝑞, 𝜌, Λ, 𝑆), 𝑎, (𝑞𝑖 , 𝜌′ , Λ′ , 𝑆 ′ )) such that
* 𝑎 ∈ Σ,
* 𝛿𝑇 (𝑞, 𝑎) = (𝑞1 , . . . , 𝑞𝑖 , . . . , 𝑞𝑚 ),
* 𝑠 = 𝐸((𝑞, 𝑎, 𝑞𝑖 )),
* 𝜌′ = 𝑠(𝜌),
* Λ′ = ∅, (Intuitively, Λ is reset.)
(︀ (𝑖𝑝) )︀* (︀
(︀ (𝑖𝑝) )︀*
)︀)︀
(𝑖𝑝) (︀
* let 𝜏𝑇 (𝑞) = (𝑃1 , 𝑃2 ), then 𝑆 ′ = 𝜏𝑇
{𝑞1 , . . ., 𝑞𝑖−1 }∪𝛿𝑇 𝑆∪ 𝜏𝑇 ∖Λ (𝑃1′ ), 𝑎 ,
where 𝑃1′ = {𝑞 ′ ∈ 𝑃1 | (𝑞, 𝑞 ′ ) ̸∈ Λ};
– the tuples ((𝑞, 𝜌, Λ, 𝑆), 𝜀, (𝑞𝑖 , 𝜌′ , Λ′ , 𝑆 ′ )) such that
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 38 -->

45:38

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

* 𝜏𝑇 (𝑞) = ((𝑞1 , . . . , 𝑞𝑖 , . . . , 𝑞𝑚 ); · · · ),
* (𝑞, 𝑞𝑖 ) ̸∈ Λ,
* 𝑠 = 𝐸(𝑞, 𝜀, 𝑞𝑖 ),
* 𝜌′ = 𝑠(𝜌),
* Λ′ = Λ ∪ {(𝑞, 𝑞𝑖 )},
(︀ (𝑖𝑝) )︀*
* 𝑆 ′ = 𝑆 ∪ 𝜏𝑇 ∖Λ ({𝑞𝑗 | 𝑗 ∈ [𝑖 − 1], (𝑞, 𝑞𝑗 ) ̸∈ Λ});
– the tuples ((𝑞, 𝜌, Λ, 𝑆), 𝜀, (𝑞𝑖 , 𝜌′ , Λ′ , 𝑆 ′ )) such that
* 𝜏𝑇 (𝑞) = ((𝑞1′ , . . . , 𝑞𝑛′ ); (𝑞1 , . . . , 𝑞𝑖 , . . . , 𝑞𝑚 )),
* (𝑞, 𝑞𝑖 ) ̸∈ Λ,
* 𝑠 = 𝐸(𝑞, 𝜀, 𝑞𝑖 ),
* 𝜌′ = 𝑠(𝜌),
* Λ′ = Λ ∪ {(𝑞, 𝑞𝑖 )},
(︀ (𝑖𝑝) )︀* (︀{︀ ′
}︀ {︀
}︀)︀
𝑞𝑗 | 𝑗 ∈ [𝑛], (𝑞, 𝑞𝑗′ ) ̸∈ Λ ∪ 𝑞𝑗 | 𝑗 ∈ [𝑖 − 1], (𝑞, 𝑞𝑗 ) ̸∈ Λ .
* 𝑆 ′ = 𝑆 ∪ {𝑞} ∪ 𝜏𝑇 ∖Λ
(Note that here we include 𝑞 into 𝑆 ′ , since the non-𝜀-transitions out of 𝑞 have
higher priorities than the transition (𝑞, 𝜀, 𝑞𝑖 ).)
∙ Moreover, 𝐹𝐵 is the set of states (𝑞, 𝜌, Λ, 𝑆) ∈ 𝑄𝐵 such that
(1) 𝐹𝑇 (𝑞) is defined,
(2) for every 𝑞 ′ ∈ 𝑆, 𝐹𝑇 (𝑞 ′ ) is not defined,
(3) if 𝐹𝑇 (𝑞) = 𝜀, then 𝑞0,𝐴 ∈ 𝐹𝐴 , otherwise, let 𝐹𝑇 (𝑞) = 𝑏1 · · · 𝑏ℓ with 𝑏𝑖 ∈ Σ ∪ 𝑋 for
each 𝑖 ∈ [ℓ], then (𝜃1 ∘ · · · ∘ 𝜃ℓ ) ∩ ({𝑞0,𝐴 } × 𝐹𝐴 ) ̸= ∅, where for each 𝑖 ∈ [ℓ], if 𝑏𝑖 ∈ Σ,
(𝑏 )
then 𝜃𝑖 = 𝛿𝐴 𝑖 , otherwise, 𝜃𝑖 = 𝜌(𝑏𝑖 ).
A.4 Tower-Hardness of String Constraints with Streaming String Transductions
We show that the satisfiability problem for STRSL is Tower-hard.
Theorem A.2. The satisfiability problem for STRSL is Tower-hard.
Our proof will use tiling problems over extremely wide corridors. We first introduce these
tiling problems, then how we will encode potential solutions as words. Finally, we will show
how STRSL can verify solutions.
(︀
)︀
A.4.1 Tiling Problems. A *tiling problem* is a tuple Θ, 𝐻, 𝑉, 𝑡0 , 𝑓 where Θ is a finite set
of tiles, 𝐻 ⊆ Θ × Θ is a horizontal matching relation, 𝑉 ⊆ Θ × Θ is a vertical matching
relation, and 𝑡0 , 𝑓 ∈ Θ are initial and final tiles respectively.
A solution to a tiling problem over a 𝑛-width corridor is a sequence
𝑡11 . . . 𝑡1𝑛
𝑡21 . . . 𝑡2𝑛
...
𝑡ℎ1 . . . 𝑡ℎ𝑛
(︁
)︁
where 𝑡11 = 𝑡0 , 𝑡ℎ𝑛 = 𝑓 , and for all 1 ≤ 𝑖 < 𝑛 and 1 ≤ 𝑗 ≤ ℎ we have 𝑡𝑗𝑖 , 𝑡𝑗𝑖+1 ∈ 𝐻 and for
(︁
)︁
all 1 ≤ 𝑖 ≤ 𝑛 and 1 ≤ 𝑗 < ℎ we have 𝑡𝑗𝑖 , 𝑡𝑗+1
∈ 𝑉 . Note, we will assume that 𝑡0 and 𝑓 can
𝑖
only appear at the beginning and end of the tiling respectively.
Tiling problems characterise many complexity classes [?]. In particular, we will use the
following facts.
∙ For any 𝑛-space Turing machine, there exists a tiling problem of size polynomial in
the size of the Turing machine, over a corridor of width 𝑛, that has a solution iff the
𝑛-space Turing machine has a terminating computation.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 39 -->

Solving String Constraints with Regex-Dependent Functions

45:39

(︀
)︀
∙ There is a fixed Θ, 𝐻, 𝑉, 𝑡0 , 𝑓 such that for any width 𝑛 there is a unique solution
𝑡11 . . . 𝑡1𝑛
𝑡21 . . . 𝑡2𝑛
...
𝑡ℎ1 . . . 𝑡ℎ𝑛
and moreover ℎ is exponential in 𝑛. One such example is a Turing machine where
the tape contents represent a binary number. The Turing machine starts from a
tape containing only 0s and finishes with a tape containing only 1s by repeatedly
incrementing the binary encoding on the tape. This Turing machine can be encoded
as the required tiling problem.
A.4.2 Large Numbers. The crux of the proof is encoding large numbers that can take values
between 1 and 𝑚-fold exponential.
A linear-length binary number could be encoded simply as a sequence of bits
𝑏0 . . . 𝑏𝑛 ∈ {0, 1}𝑛 .
(︀
)︀
To aid with later constructions we will take a more oblique approach. Let Θ1 , 𝐻1 , 𝑉1 , 𝑡01 , 𝑓1
be a copy of the fixed tiling problem from the previous section for which there is a unique
solution, whose length must be exponential in the width. In the future, we will need several
copies of this problem, hence the indexing here. Note, we assume each copy has disjoint tile
sets. Fix a width 𝑛 and let 𝑁1 be the corresponding corridor length. A level-1 number can
encode values from 1 to 𝑁1 . In particular, for 1 ≤ 𝑖 ≤ 𝑁1 we define
[𝑖]1 = 𝑡𝑖1 . . . 𝑡𝑖𝑛
where 𝑡𝑖1 . . . 𝑡𝑖𝑛 is the tiling of the 𝑖th row of the unique solution to the tiling problem.
A level-2 number will be derived from tiling a corridor of width 𝑁1 , (︀and thus the number
)︀
of rows will be doubly-exponential. For this, we require another copy Θ2 , 𝐻2 , 𝑉2 , 𝑡02 , 𝑓2 of
the above tiling problem. Moreover, let 𝑁2 be the length of the solution for a corridor of
width 𝑁1 . Then for any 1 ≤ 𝑖 ≤ 𝑁2 we define
[𝑖]2 = [1]1 𝑡𝑖1 [2]1 𝑡𝑖2 . . . [𝑁1 ]1 𝑡𝑖𝑁1
where 𝑡𝑖1 . . . 𝑡𝑖𝑁1 is the tiling of the 𝑖th row of the unique solution to the tiling problem. That
is, the encoding indexes each tile with it’s column number, where the column number is
represented as a level-1 number.
In general, a *level-𝑚* number is of length (𝑚(︀ − 1)-fold exponential
)︀ and can encode
numbers 𝑚-fold exponential in size. We use a copy Θ𝑚 , 𝐻𝑚 , 𝑉𝑚 , 𝑡0𝑚 , 𝑓𝑚 of the above tiling
problem and use a corridor of width 𝑁𝑚−1 . We define 𝑁𝑚 as the length of the unique
solution to this problem. Then, for any 1 ≤ 𝑖 ≤ 𝑁𝑚 we have
[𝑖]𝑚 = [1]𝑚−1 𝑡𝑖1 [2]𝑚−1 𝑡𝑖2 . . . [𝑁𝑚−1 ]𝑚−1 𝑡𝑖𝑁𝑚−1
where 𝑡𝑖1 . . . 𝑡𝑖𝑁𝑚−1 is the tiling of the 𝑖th row of the unique solution to the tiling problem.
Note that we can define regular languages to check that a string is a large number. In
particular
{︃
𝑛
[Θ1 ]
𝑚=1
𝑛
]︀*
𝑅𝑚 = [︀ 𝑛
𝑚>1.
𝑅𝑚−1 Θ𝑚
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 40 -->

45:40

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

A.4.3 Hardness Proof. We show that the satisfiability problem for STRSL is Tower-hard.
We first introduce the basic framework of solving a hard tiling problem. Then we discuss
the two phases of transductions required by the reduction. These are constructing a large
boolean formula, and then evaluating the formula. This two phases are described in separate
sections.
The Framework. The proof is by reduction from a tiling problem over an 𝑚-fold exponential
width corridor. In general, solving such problems is hard for 𝑚-ExpSpace.
Let 𝑁𝑚 be the width of the corridor. Fix a tiling problem
(︀
)︀
Θ, 𝐻, 𝑉, 𝑡0 , 𝑓 .
We will compose an STRSL formula 𝑆 with a free variable 𝑥. If 𝑆 is satisfiable, 𝑥 will
contain a string encoding a solution to the tiling problem. In particular, the value of 𝑥 will
be of the form
[1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚 #
[1]𝑚 𝑡21 . . . [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 #
...
ℎ
[1]𝑚 𝑡1 . . . [𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 # .
That is, each row of the solution is separated by the # symbol. Between each tile of a row is
it’s index, encoded using the large number encoding described in the previous section.
The formula 𝑆 will use a series of replacements and assertions to verify that the tiling
encoded by 𝑥 is a valid solution to the tiling problem. We will give the formula in three
steps.
We will define the alphabet to be
Σ=Θ∪Θ
where Θ is the set of tiles, and Θ is the set of characters required to encode large numbers,
plus #.
The first part is
(︁
[︀ 𝑛 * ]︀* )︁
assert 𝑥 ∈ [𝑅𝑚
Θ] #
;
(︀
)︀
𝑛 0
assert 𝑥 ∈ 𝑅𝑚
𝑡 ;
assert (𝑥 ∈ Σ* 𝑓 #) ;
(︃
[︃[︃
]︃*
]︃* )︃
∑︀
?
𝑛
𝑛
𝑛
;
assert 𝑥 ∈
𝑅𝑚 𝑡1 𝑅𝑚 𝑡2 [𝑅𝑚 Θ] #
(𝑡1 ,𝑡2 )∈𝐻
(︃
[︃
[︃
]︃*
]︃* )︃
∑︀
?
𝑛
𝑛
𝑛
𝑛
assert 𝑥 ∈ [𝑅𝑚 Θ]
𝑅𝑚 𝑡1 𝑅𝑚 𝑡2 [𝑅𝑚 Θ] #
;
(𝑡1 ,𝑡2 )∈𝐻

The first asserts simply verify the format of the value of 𝑥 is as expected and moreover,
the first appearing element of Θ in the string is 𝑡0 , and the last element is 𝑓 .
The final two assertions check the horizontal tiling relation. In particular, the first checks
that even pairs of tiles are in 𝐻, while the second checks odd pairs are in 𝐻.
The main challenge is checking the vertical tiling relation. This is done by a series of
transductions operating in two main phases. The first phase rewrites the encoding into a
kind of large Boolean formula, which is then evaluated in the second phase.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 41 -->

Solving String Constraints with Regex-Dependent Functions

45:41

Constructing the Large Boolean Formula. The next phase of the formula is shown below and
explained afterwards. For convenience, we will describe the construction using transductions.
After the explanation, we will describe how to achieve these transductions using replaceAll.
𝑥1𝑚 = 𝒯𝑚1 (︀(𝑥); )︀
𝑥2𝑚 = 𝒯𝑚2 (︀𝑥1𝑚 )︀;
𝑥3𝑚 = 𝒯𝑚3 𝑥2𝑚 (︀; )︀
1
3
𝑥1𝑚−1 = 𝒯𝑚−1
(︀𝑥1𝑚 ; )︀
2
2
𝑥𝑚−1 = 𝒯𝑚−1 (︀𝑥𝑚−1 )︀;
3
𝑥3𝑚−1 = 𝒯𝑚−1
𝑥2𝑚−1 ;
...
(︀ )︀
𝑥11 = 𝒯11 (︀𝑥32 )︀;
𝑥21 = 𝒯12 (︀𝑥11 )︀;
𝑥31 = 𝒯13(︀ 𝑥21)︀ ;
𝑥0 = 𝒯0 𝑥31 .
The Boolean formula is constructed by rewriting the encoding stored in 𝑥. We need to
check the vertical tiling relation by comparing 𝑡𝑖𝑗 with 𝑡𝑖+1
𝑗 . However, these are separated by
a huge number of other tiles, which also need to be checked against their counterpart in the
next row.
The goal of the transductions is to ”rotate” the encoding so that instead of each tile being
directly next to its horizontal counterpart, it is directly next to its vertical counterpart. Our
transductions do not quite achieve this goal, but instead place the tiles in each row next to
potential vertical counterparts. The Boolean formula contains large disjunctions over these
possibilities and use the indexing by large numbers to pick out the correct pairs.
The idea is best illustrated by showing the first three transductions, 𝒯𝑚1 , 𝒯𝑚2 , and 𝒯𝑚3 . We
start with
[1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚 #
[1]𝑚 𝑡21 . . . [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 #
...
[1]𝑚 𝑡ℎ1 . . . [𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 # .
The transducer 𝒯𝑚1 saves the row it is currently reading. Then, when reading the next row, it
outputs each index and tile of the current row followed by a copy of the last row. The output
is shown below. We use a disjunction symbol to indicate that, after the transduction, the tile
should match one of the tiles copied after it. Between each pair of a tile and a copied row,
we use the conjunction symbol to indicate that every disjunction should have one match.
The result is shown below. To aid readability, we underline the copied rows. The parentheses
⟨⟩ are also inserted to aid future parsing.
⟨
⟩
⟨
⟩
[1]𝑚 𝑡21 ∨ [1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚 ∧ . . . ∧ [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 ∨ [1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚
∧ . . .⟨∧
⟩
⟩
⟨
ℎ−1
. . . [𝑁𝑚 ]𝑚 𝑡ℎ−1
∧ . . . ∧ [𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 ∨ [1]𝑚 𝑡1ℎ−1 . . . [𝑁𝑚 ]𝑚 𝑡𝑁
[1]𝑚 𝑡ℎ1 ∨ [1]𝑚 𝑡ℎ−1
1
𝑁𝑚
𝑚

.

After this transduction, we apply 𝒯𝑚2 . This transduction forms pairs of a tile, with all
tiles following it from the previous row (up to the next ∧ symbol). This leaves us with a
conjunction of disjunctions of pairs. Inside each disjunct, we need to verify that one pair
has matching indices and tiles that satisfy the vertical tiling relation 𝑉 . The result of the
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 42 -->

45:42

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

second transduction is shown below.
⟨︀
⟩︀
⟨︀
⟩︀
[1]𝑚 𝑡21 [1]𝑚 𝑡11 ∨ . . . ∨ [1]𝑚 𝑡21 [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚 ∧ . . . ∧ [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 [1]𝑚 𝑡11 ∨ . . . ∨ [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚
∧ ⟨︀. . . ∧
⟩︀
⟩︀
⟨︀
ℎ−1
ℎ
∧
.
.
.
∧
[𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 [1]𝑚 𝑡1ℎ−1 ∨ . . . ∨ [𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 [𝑁𝑚 ]𝑚 𝑡ℎ−1
[1]𝑚 𝑡ℎ1 [1]𝑚 𝑡ℎ−1
∨
.
.
.
∨
[1]
𝑡
[𝑁
]
𝑡
𝑚
𝑚
𝑚
1
1
𝑁𝑚
𝑁𝑚

Notice that we now have each tile in a pair with its vertical neighbour, but also in a pair
with every other tile in the row beneath. The indices can be used to pick out the right pairs,
but we will need further transductions to analyse the encoding of large numbers.
To simplify matters, we apply 𝒯𝑚3 . This transduction removes the tiles from the string,
retaining each pair of indices where the tiles satisfy the vertical tiling relation. When the
tiling relation is not satisfied, we insert ⊥𝑚 . We use ⟨, #, and ⟩ to delimit the indices. We
are left with a string of the form
⋀︁ ⋁︁
⟨[𝑖]𝑚 #[𝑗]𝑚 ⟩ ∨ ⊥𝑚 ∨ · · · ∨ ⊥𝑚 .
We will often elide the ⊥𝑚 disjuncts for clarity. They will remain untouched until the formula
is evaluated in the next section.
We consider a pair ⟨[𝑖]𝑚 #[𝑗]𝑚 ⟩ to evaluate to true whenever 𝑖 = 𝑗. The truth of the formula
can be computed accordingly. However, it’s not straightforward to check whether 𝑖 = 𝑗 as
they are large numbers. The key observation is that they are encoded as solutions to indexed
tiling problems, which means we can go through a similar process to the transductions
above.
First, recall that [𝑖]𝑚 is of the form
[1]𝑚−1 𝑑𝑖1 [2]𝑚−1 𝑑𝑖2 . . . [𝑁𝑚−1 ]𝑚−1 𝑑𝑖𝑁𝑚−1
where we use 𝑑 to indicate tiles instead of 𝑡.
1
2
3
We apply three transductions 𝒯𝑚−1
, 𝒯𝑚−1
, and 𝒯𝑚−1
. The first copies the first index of
each pair directly after the tiles of the second index. That is, each pair
is rewritten to

⟨[𝑖]𝑚 #[𝑗]𝑚 ⟩
⟨
⟩
⟨
⟩
[1]𝑚−1 𝑑𝑗1 ∨ [𝑖]𝑚 ∧ . . . ∧ [𝑁𝑚−1 ]𝑚−1 𝑑𝑗𝑁𝑚−1 ∨ [𝑖]𝑚 .

Then we apply a similar second transduction: each disjunction is expanded into pairs of
indices and tiles. The result is
⟨
⟩
[1]𝑚−1 𝑑𝑗1 [1]𝑚−1 𝑑𝑖1 ∨ . . . ∨ [1]𝑚−1 𝑑𝑗1 [𝑁𝑚−1 ]𝑚−1 𝑑𝑖𝑁𝑚−1
∧...∧
⟨
⟩
[𝑁𝑚−1 ]𝑚−1 𝑑𝑗𝑁𝑚−1 [1]𝑚−1 𝑑𝑖1 ∨ . . . ∨ [𝑁𝑚−1 ]𝑚−1 𝑑𝑗𝑁𝑚−1 [𝑁𝑚−1 ]𝑚−1 𝑑𝑖𝑁𝑚−1

.

The third transduction replaces with ⊥𝑚−1 all pairs where we don’t have 𝑑𝑗𝑘 = 𝑑𝑖𝑘′ (recall,
we need to check that 𝑖 = 𝑗 so the tiles at each position should be the same). As before, for
a single pair, this leaves us with a string formula of the form
⋀︁ ⋁︁
⟨[𝑖′ ]𝑚−1 #[𝑗 ′ ]𝑚−1 ⟩ ∨ ⊥𝑚−1 ∨ · · · ∨ ⊥𝑚−1 .
Again, we will elide the ⊥𝑚−1 disjuncts for clarity as they will be untouched until the
formula is evaluated. Recalling that there are many pairs in the input string, the output of
this series of transductions is a string formula of the form
⋀︁ ⋁︁ ⋀︁ ⋁︁
⟨[𝑖′ ]𝑚−1 #[𝑗 ′ ]𝑚−1 ⟩ .
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 43 -->

Solving String Constraints with Regex-Dependent Functions

45:43

1
2
3
We repeat these steps using 𝒯𝑚−2
, 𝒯𝑚−2
, 𝒯𝑚−2
all the way down to 𝒯11 , 𝒯12 , 𝒯13 . We are
left with a string formula of the form
⋀︁ ⋁︁ ⋀︁ ⋁︁
···
⟨[𝑖′ ]1 #[𝑗 ′ ]1 ⟩ .

Recall each [𝑖′ ]1 is of the form
′

′

𝑑𝑖1 . . . 𝑑𝑖𝑛 .
The final step interleaves the tiles of the two numbers. The result is a string formula of the
form
⋀︁ ⋁︁ ⋀︁ ⋁︁ ⋀︁
···
𝑑𝑑′ .

This is the formula that is evaluated in the next phase.
To complete this section we need to implement the above transductions using replaceAll.
First, consider 𝒯𝑚1 . We start with
[1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚 #
[1]𝑚 𝑡21 . . . [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 #
...
[1]𝑚 𝑡ℎ1 . . . [𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 # .

We are aiming for
⟨︀
⟩︀
⟨︀
⟩︀
[1]𝑚 𝑡21 ∨ [1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚 ∧ . . . ∧ [𝑁𝑚 ]𝑚 𝑡2𝑁𝑚 ∨ [1]𝑚 𝑡11 . . . [𝑁𝑚 ]𝑚 𝑡1𝑁𝑚
∧ . . .⟨︀∧
⟨︀
⟩︀
⟩︀
ℎ−1
.
.
.
[𝑁
]
𝑡
∧
.
.
. ∧ [𝑁𝑚 ]𝑚 𝑡ℎ𝑁𝑚 ∨ [1]𝑚 𝑡ℎ−1
. . . [𝑁𝑚 ]𝑚 𝑡ℎ−1
.
[1]𝑚 𝑡ℎ1 ∨ [1]𝑚 𝑡ℎ−1
𝑚 𝑚 𝑁𝑚
1
1
𝑁𝑚

We use two replaceAlls. The first uses $← to do the main work of copying the previous row
into the current row a huge number of times. In fact, $← will copy too much, as it will
copy everything that came before, not just the last row. The second replaceAll will cut down
the contents of $← to only the last row. That is, we first apply replaceAllpat1 ,rep1 and then
replaceAllpat2 ,rep2 where
pat1
rep1

=
=

(𝑡)
$1 ▷ $← ◁

and ▷ and ◁ are two characters not in Σ, and, letting Σ# = Σ ∖ {#},
pat2
rep2

=
=

▷Σ*# #(Σ*# )#Σ*# ◁
∨$1 .

That is, the first replace adds after each tile the entire preceding string, delimited by ▷ and
◁. The second replace picks out the final row of each string between ▷ and ◁ and adds the ∨.
Notice that the second replace does not match anything between ▷ and ◁ on the first row.
In fact, we need another replaceAll to delete the first row. That is replaceAllpat3 ,rep3 where
pat3
rep3

=
=

*

[Σ ∪ {▷, ◁}] ◁ Σ*# #
𝜀.

Notice, the pattern above matches any row containing at least one ◁. This means only the
first row will be deleted as delimiters have already been removed from the other rows. To
complete the step, we replace all # with ∧ and insert the parenthesis ⟨⟩ using another
replaceAll (and a concatenation at the beginning and the end of the string).
The transduction 𝒯𝑚2 uses similar techniques to the above and we leave the details to the
reader. The same is true of the other similar transductions 𝒯𝑖1 and 𝒯𝑖2 .
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 44 -->

45:44

T. Chen, A. Flores-Lamas, M. Hague, Z. Han, D. Hu, S. Kan, A. W. Lin, P. Rümmer, and Z. Wu

Transduction 𝒯𝑚3 (and similarly the other 𝒯𝑖2 ) replaces all pairs
[𝑖]𝑚 𝑡2𝑖 [𝑖 + 1]𝑚 𝑡1𝑖+1

that do not satisfy the vertical tiling relation with ⊥𝑚 , and rewrites them to
⟨[𝑖′ ]1 #[𝑗 ′ ]1 ⟩

if the vertical tiling relation is matched. This can be done in two steps: first replace the
non-matches, then replace the matches. To replace the non-matches we use replaceAllpat1 ,rep1
where
∑︀
𝑛
𝑛
pat1 =
𝑅𝑚
𝑡1 𝑅𝑚
𝑡2
(𝑡1 ,𝑡2 ) /
∈𝑉

rep1 = ⊥𝑚 .
For the matches we use replaceAllpat2 ,rep2 where
∑︀
𝑛
𝑛
pat2 =
(𝑅𝑚
)𝑡1 (𝑅𝑚
)𝑡2
(𝑡1 ,𝑡2 )∈𝑉

rep1

= ⟨$1#$2⟩ .

The final transduction takes a string of the form
⋀︁ ⋁︁ ⋀︁ ⋁︁
···
⟨[𝑖′ ]1 #[𝑗 ′ ]1 ⟩
where each [𝑖′ ]1 is of the form

′

′

𝑑𝑖1 . . . 𝑑𝑖𝑛 .
We need to interleave the tiles of the two numbers, giving a string of the form
⋀︁ ⋁︁ ⋀︁ ⋁︁ ⋀︁
···
𝑑𝑑′ .
This can be done with a single replaceAllpat,rep where
pat = ⟨(Θ1 ) . . . (Θ1 )#(Θ1 ) . . . (Θ1 )⟩
rep = ⟨$1$(𝑛 + 1) ∧ · · · ∧ $𝑛$(2𝑛)⟩ .
Evaluating the Large Boolean Formula. The final phase of 𝑆 evaluates the Boolean formula
and is shown below. Again we write the formula using transductions and explain how they
can be done with replaceAll.
𝑥∧
0 = 𝒯0 (𝑥0 );
∧ ∧
𝑥∨
1 = 𝒯0 (𝑥0 );
∧
∨ ∨
𝑥1 = 𝒯1 (𝑥1 );
∧ ∧
𝑥∨
2 = 𝒯1 (𝑥1 );
∧
∨ ∨
𝑥2 = 𝒯2 (𝑥2 );
∧ ∧
𝑥∧
3 = 𝒯2 (𝑥2 );
. . . (︀
)︀
∧
∧
𝑥∨
𝑚 = 𝒯𝑚−1 𝑥𝑚−1 ;
∨ ∨
𝑥∧
𝑚 = 𝒯𝑚 (𝑥𝑚 );
∧ ∨
𝑥𝑓 =(︀ 𝒯𝑚 (𝑥𝑚 ); )︀
assert 𝑥𝑓 ∈ pat𝑓
The first transducer 𝒯1 reads the string formula
⋀︁ ⋁︁ ⋀︁ ⋁︁ ⋀︁
···
𝑑𝑑′ .
copies it to its output, except replacing each pair 𝑑𝑑′ with ⊤1 if 𝑑 = 𝑑′ and with ⊥1 otherwise.
This is requires two simple replaceAll calls.
Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.

<!-- page 45 -->

Solving String Constraints with Regex-Dependent Functions

45:45

The remaining transductions evaluate the innermost disjunction or conjunction as ⋁︀
appropriate (the parenthesis ⟨⟩ are helpful here). For example 𝒯1∨ replaces the innermost 𝑣
with ⊤1 if ⊤1 appears somewhere in the disjunction and ⊥1 otherwise. This can be done by
greedily matching any sequence of characters from {⊤1 , ⊥1 , ∨} that contains at least one
⊤1 and replacing the sequence with ⊤1 , then greedily matching any remaining sequence
of {⊥1 , ∨} and replacing it with ⊥1 . The evaluation of conjunctions works similarly, but
inserts ⊤2 and ⊥2 in the move to the next level of evaluation.
The final assert checks that 𝑥𝑓 contains only the character ⊤𝑚+1 and fails otherwise.
This completes the reduction.
A.5

Exponential Space Copyless Algorithm

We argue that, when the PSSTs are copyless, satisfiability for STRSL can be decided in
exponential space.
The algorithm in the proof of Theorem 5.7 for STRSL applies a subset of the proof rules
in Table 1. These proof rules branch on disjunctions and backward propagation through
transductions and concatenations. We can explore all branches through the proof tree,
storing the sequence of branches chosen in polynomial space. Thus, we can consider each
branch independently.
A branch consists of a backwards propagation through PSSTs and concatenations. We
will argue that each state of the automata constructed can be stored in exponential space.
Since these states are combinations of states of PSSTs and finite automata constructed by
earlier stages of the algorithm, it is possible to calculate the next states from the current
state on the fly. Thus, if the states can be stored in exponential space, the full algorithm
will only require exponential space.
We first consider the case where we have PSSTs and FAs rather than string functions
using RegEx. Let 𝑛 be the size of the largest PSST or FA. Let 𝑥 be the maximum number
of variables in the PSST. Finally, let ℓ be the length of the longest branch of transductions
and concatenations in the proof tree (which is linear in the size of the constraint).
The FA in the pre-image of a concatenation all have the same size as the output automaton
𝒜. The FA ℬ in the pre-image of a PSST 𝒯 with output automaton 𝒜 is an automaton
such that L (ℬ) = ℛ𝒯−1 (L (𝒜)). It has states of the form (𝑞, 𝜌, Λ, 𝑆), where 𝑞 is a state of
𝒯 , 𝜌 is a function from variables of 𝒯 to pairs of states of 𝒜, Λ ⊆ ℰ(𝜏𝑇 ), and 𝑆 ⊆ 𝑄𝑇 .
Note, because we assume 𝒯 is copyless, 𝜌 is a function to pairs of states, not to sets of pairs
of states. The space needed to store a state of of ℬ is hence 𝒪(𝑠 + 2𝑥𝑠 + 2𝑛) where 𝑠 is
the space required to store a state of 𝒜. Consequently, after ℓ backwards propagations, we
can store the states of the automaton in space 𝒪(2ℓ 𝑥ℓ 𝑛). That is, exponential space. This
remains true when the PSST and FA may be exponential in the size of the 𝑅𝑒𝑔𝐸𝑥.

Proc. ACM Program. Lang., Vol. 6, No. POPL, Article 45. Publication date: January 2022.
