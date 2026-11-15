---
file: ../pdfs/2021-spe-borsotti-efficient-posix-submatch-nfa.pdf
citation: "Efficient POSIX Submatch Extraction on NFA; Borsotti, Trofimovich; Software: Practice and Experience 51(2); 2021"
doi: 
source: "https://re2c.org/2019_borsotti_trofimovich_efficient_posix_submatch_extraction_on_nfa.pdf"
topics: [semantics_theory_access]
pages: 33
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Received 24 July 2019

Revised 22 June 2020

Accepted 30 June 2020

DOI: 10.1002/spe.2881

RESEARCH ARTICLE

Efficient POSIX Submatch Extraction on NFA
Angelo Borsotti1 | Ulya Trofimovich2
1 Department of Electronics, Information,

and Bioengineering, Polytechnic University
of Milan, Milan, Italy.

2 Department of Discrete Mathematics and

Algorithmics, Belarusian State University,
Minsk, Belarus.

Correspondence
Angelo Borsotti
Email: angelo.borsotti@mail.polimi.it
Ulya Trofimovich
Email: skvadrik@gmail.com

Summary
In this paper we study the performance of NFA-based POSIX submatch extraction algorithms. We propose an algorithm that combines Laurikari tagged NFA and

extended Okui-Suzuki disambiguation. The algorithm works in worst-case 𝑂(𝑛 𝑚2 𝑡)

time and 𝑂(𝑚2 ) space (including preprocessing), where 𝑛 is the length of input, 𝑚

is the size of the regular expression with bounded repetition expanded and 𝑡 is the

number of capturing groups and subexpressions that contain them. On real-world
benchmarks our algorithm performs close to the 𝑂(𝑛 𝑚 𝑡) complexity of leftmost-

greedy matching, although on artificial benchmarks it can be significantly slower. We

propose a lazy version of the algorithm that runs much faster, but requires 𝑂(𝑛 𝑚2 )

space. We show that the Kuklewicz algorithm is slower in practice, and the backward
matching algorithm proposed by Cox is incorrect.

KEYWORDS:
Regular Expressions, Parsing, Submatch Extraction, Finite-State Automata, POSIX

1

INTRODUCTION

Regular expressions (RE) are a convenient notation for describing regular languages. RE syntax varies among different specifications and standards. Most of the variations allow a more succinct representation, but do not increase the expressive power
of RE. In this paper we do not consider such trivial extensions, with the exception of bounded repetition, which requires
non-trivial changes in the algorithms and proofs. We also do not consider extensions that bring RE beyond regular languages,
such as backreferences. The recognition problem for RE can be solved in 𝑂(𝑛 𝑚) time and 𝑂(𝑚) space, where 𝑛 is the size of
input and 𝑚 is the size of RE (e.g. by simulation of a Thompson’s NFA 1 2 ). The parsing problem is more difficult because it has
to deal with ambiguity. Ambiguity means that there are multiple ways to parse the input. The preferred way is usually defined
by a disambiguation policy. There are two widely used policies: the Perl leftmost-greedy policy and the POSIX longest-match
policy. The Perl policy is defined in terms of RE structure; it admits a simple and efficient implementation. The POSIX policy,
on the other hand, is defined in terms of the structure of parse results; it is difficult to implement compared to the Perl policy. In
this paper we focus on the POSIX policy. The problem of submatch extraction is a special case of parsing: it has the same worstcase complexity, but in practice specialized submatch extraction algorithms are faster than generic full parsing algorithms.
We study NFA-based approaches to the problem. Namely, we consider the algorithms proposed by Okui and Suzuki 3 , Kuklewicz 5 and Cox 6 . Our experiments show that in general Okui-Suzuki approach is the most efficient one. We combine it with
the insights and useful techniques from other approaches and suggest a few algorithmic and practical improvements of our own.
The proposed algorithm is thoroughly formalized. We are aware of an alternative approach based on Brzozowski derivatives
proposed by Sulzmann and Lu 7 , but in our experience this approach is slower in practice and has worse algorithmic complexity
0 Preprint, the original is Softw Pract Exper. 2020;1-36. https://doi.org/10.1002/spe.2881

<!-- page 2 -->

2

(discussed in more detail below). Both NFA-based and derivative-based approach can be used to construct DFA with POSIX
semantics 8 9 11 . The resulting DFA are very fast, because the disambiguation is done at determinization time and there is no
run-time overhead. However, determinization is not always viable due to its exponential worst-case complexity. The algorithm
we propose in this paper is based on NFA, but it can also be used as a basis for DFA construction.
We give an overview of existing algorithms, including some that are incorrect, but interesting from historical perspective. The
list is by no means exhaustive, but in our experience other approaches produce incorrect results or require memory proportional
to the length of input (e.g. the Glibc implementation 21 ).
Laurikari, 2001 (incorrect).
Laurikari described an algorithm based on TNFA — 𝜖-NFA with tagged transitions 4 . He
represented each submatch group with a pair of tags (opening and closing). Disambiguation is based on minimizing the value of
opening tags and maximizing the value of closing tags, where different tags have priority according to the POSIX subexpression
hierarchy. The algorithm gives incorrect results for REs with iteration subexpressions, such as (a|aa)* on a string aa (it
minimizes the value of the opening tag on the last iteration, and fails to take into account the preceding iterations). The reported
time complexity is 𝑂(𝑛 𝑚 𝑐 𝑡 log(𝑡)), where 𝑛 is the length of input, 𝑚 is TNFA size, 𝑐 is the time for comparing tag values and
𝑡 is the number of tags. Space complexity is 𝑂(𝑚 𝑡). Notably, Laurikari used the idea of topological order to avoid worst-case
exponential time of 𝜖-closure construction, although his closure algorithm is not optimal, as shown in 11 .
Kuklewicz, 2007.
Kuklewicz fixed Laurikari algorithm by introducing orbit tags for iteration subexpressions. He gave only
an informal description 5 , but the algorithm was later formalized in 11 . It works in the same way as the Laurikari algorithm,
except that the comparison of orbit tags takes into account the history of all iterations, not just the most recent one. The key
idea that allows the algorithm to execute in bounded memory is to compress orbit tag histories in a matrix of size 𝑡 × 𝑚, where
𝑚 is the size of TNFA and 𝑡 is the number of tags. 𝑡-Th row of the matrix represents the ordering of closure states with respect
to 𝑡-th tag. The matrix is updated at each step using the continuations of tag histories. The algorithm requires 𝑂(𝑚 𝑡) space and
𝑂(𝑛 𝑚 𝑡 (𝑚 + 𝑡 log(𝑚))) time, where 𝑛 is the length of input (we assume a worst-case optimal 𝑂(𝑚2 𝑡) algorithm for 𝜖-closure
construction, and an 𝑂(𝑚 log(𝑚) 𝑡2 ) matrix update, because for 𝑡 tags 𝑚 states are sorted with an 𝑂(𝑡) comparison function).
Kuklewicz disambiguation is combined with Laurikari determinization in construction of TDFA 11 .
Cox, 2009 (incorrect).
Cox came up with the idea of backward POSIX matching 6 , which is based on the observation that
reversing the longest-match rule simplifies the handling of iteration subexpressions: instead of maximizing submatch from the
first to the last iteration, one needs to maximize the iterations in reverse order. This means that the disambiguation is always
based on the most recent iteration, removing the need to remember all previous iterations (except for the backwards-first, i.e.
the last one, which contains submatch result). The algorithm tracks two pairs of offsets per each submatch group: the active
pair (used for disambiguation) and the result pair. It gives incorrect results under two conditions: (1) ambiguous matches have
equal offsets on some iteration, and (2) disambiguation happens too late, when the active offsets have already been updated
and the difference between ambiguous matches is erased. We found that such situations may occur for two reasons. First, the
𝜖-closure algorithm may compare ambiguous paths after their join point, when both paths have a common suffix with tagged
transitions. This is the case with the Cox prototype implementation 6 ; for example, it gives incorrect results for (aa|a)* and
string aaaaa. Most of such failures can be repaired by exploring states in topological order, but a topological order does not
exist in the presence of 𝜖-loops. The second reason is bounded repetition: ambiguous paths may not have an intermediate join
point at all. For example, in the case of (aaaa|aaa|a){3,4} and string aaaaaaaaaa we have matches (aaaa)(aaaa)(a)(a)
and (aaaa)(aaa)(aaa) with a different number of iterations. Assuming that the bounded repetition is unrolled by chaining
three sub-automata for (aaaa|aaa|a) and an optional fourth one, by the time ambiguous paths meet both have active offsets
(0,4). Despite the flaw, Cox algorithm is interesting: if somehow the delayed comparison problem was fixed, it would work.
The algorithm requires 𝑂(𝑚 𝑡) memory and 𝑂(𝑛 𝑚2 𝑡) time (assuming a worst-case optimal closure algorithm), where 𝑛 is the
length of input, 𝑚 it the size of RE and 𝑡 is the number of submatch groups and subexpressions that contain them.
Okui and Suzuki, 2013.
Okui and Suzuki view the disambiguation problem from the point of comparison of parse trees 3 .
Ambiguous trees have the same frontier of leaf symbols, but their branching structure is different. Each subtree corresponds
to a subexpression. The norm of a subtree is the number of alphabet symbols in it (a.k.a. submatch length). The longest match
corresponds to a tree in which the norm of each subtree in leftmost in-order traversal is maximized. The clever idea of Okui
and Suzuki is to relate the norm of subtrees to their height (the distance from the root). Namely, if we walk through the leaves
of two ambiguous trees, tracking the height of each complete subtree, then at some step the heights will diverge: subtree with

<!-- page 3 -->

3

a smaller norm will already be complete, but the one with a greater norm will not. The height of subtrees is easy to track by
attributing it to parentheses and encoding in the automaton transitions. Okui and Suzuki use PAT — 𝜖-free position automaton
with transitions labeled by sequences of parentheses. Disambiguation is based on comparing parentheses along ambiguous PAT
paths. Similar to Kuklewicz, Okui and Suzuki avoid recording full-length paths by pre-comparing them at each step and storing
the comparison results in a pair of matrices indexed by PAT states. The authors report complexity 𝑂(𝑛(𝑚2 + 𝑐)), where 𝑛 is
the input length, 𝑚 is the number of occurrences of the most frequent symbol in RE and 𝑐 is the number of submatch groups
and repetition operators. Memory requirement is 𝑂(𝑚2 ). However, these estimates leave out precomputation of the precedence
relation and the construction of PAT, which may grow exponential in the size of RE: the transformation from RE to PAT is
ambiguity-preserving, which means that there may be multiple transitions with different labels between a pair of PAT states.
For example, for RE ((a*)|(a*)){k} there are 2𝑘 different ways to match the empty string, and consequently 2𝑘 transitions
between the initial and the final states of PAT. Although such RE are unlikely in practice, RE engines should be able to handle
them in reasonable time. Okui-Suzuki disambiguation is combined with Berry-Sethi construction in parsing DFA 9 .
Sulzmann and Lu, 2013.
Sulzmann and Lu based their algorithm on Brzozowski derivatives 7 (correctness proof is given
by Ausaf, Dyckhoff and Urban 10 ). The algorithm unfolds a RE into a sequence of derivatives and then folds it back into a
parse tree. Each derivative is obtained from the previous one by consuming an input symbol in left-to-right order, and each tree
is built from the next tree by injecting a symbol in reversed right-to-left order. In practice, Sulzmann and Lu fuse backward
and forward passes, which allows to avoid potentially unbounded memory usage on keeping all intermediate derivatives. The
algorithm is elegant in that it does not require explicit disambiguation: by the definition of derivative, parse trees are ordered by
the longest-match criterion. Time and space complexity is not entirely clear. In 7 Sulzmann and Lu consider the size of RE as a
constant. In 8 they give more precise estimates: 𝑂(2𝑚 𝑡) space and 𝑂(𝑛 log(2𝑚 ) 2𝑚 𝑡2 ) time, where 𝑚 is the size of RE, 𝑛 is the
length of input and 𝑡 the number of submatch groups (the authors do not differentiate between 𝑚 and 𝑡). However, this estimate
assumes worst-case 𝑂(2𝑚 ) derivative size and on-the-fly DFA construction. The authors also mention a better 𝑂(𝑚2 ) theoretical
bound for derivative size. If we adopt this bound and exclude DFA construction, we get 𝑂(𝑚2 𝑡) memory requirement and
𝑂(𝑛 𝑚2 𝑡2 ) time, which seems reasonably close to (but worse than) NFA-based approaches.

Our contributions are the following:

• We extend Okui-Suzuki algorithm on partially ordered parse trees, which greatly reduces the disambiguation overhead.

• We extend Okui-Suzuki algorithm on the case of bounded repetition.

• We use Laurikari TNFA instead of Okui-Suzuki PAT, which avoids the potential blowout of PAT size.
• We introduce negative tags that allow us to handle no-match and match cases uniformly.

• We consider 𝜖-closure construction as a shortest-path problem and use the Goldberg-Radzik algorithm.

• We give a fast 𝑂(𝑚2 ) algorithm for updating the precedence matrix (where 𝑚 is the size of the 𝜖-closure).
• We provide a way to output either parse trees or POSIX-style offsets.

• We give a lazy version of the algorithm that is faster, but uses memory proportional to the size of input.
• We provide a C++ implementation of the described algorithms.

The rest of this paper is arranged as follows. In section 2 we present the main idea and the skeleton of our algorithm. In section
3 we provide theoretical foundations for the rest of the paper. After that, we go into specific details: section 4 is concerned with
𝜖-closure construction, section 5 discusses data structures used to represent TNFA paths, section 6 discusses possible output
formats (parse trees or POSIX-style offsets), section 7 gives the core disambiguation algorithms, section 8 presents a lazy version
of the algorithm, and section 9 gives specific TNFA construction. The remaining sections 10, 11 and 12 contain complexity
analysis, benchmarks, conclusions and directions for future work.

2

SKELETON OF THE ALGORITHM

Our algorithm is based on four cornerstone concepts: regular expressions, parse trees, parenthesized expressions and tagged
NFA. Following Okui and Suzuki 3 , we give the interpretation of regular expressions as sets of parse trees and define POSIX
disambiguation semantics in terms of order on parse trees. This definition reflects the POSIX standard, but it is too high-level

<!-- page 4 -->

4

to be used in practice. From parse trees we go to their linearized representation — parenthesized expressions. We define an
order on parenthesized expressions and show its equivalence to the order on parse trees. The latter definition of order is more
low-level and can be converted to an efficient disambiguation algorithm. Finally, we construct TNFA and map parenthesized
expressions to its paths, which allows us to compare ambiguous paths using the algorithm for parenthesized expressions. In this
section we give the four basic definitions and the skeleton of the algorithm. In the following sections we formalize the relation
between different representations and fill in all the details.

Definition 1. Regular expressions (RE) over finite alphabet Σ, denoted Σ :
1. Empty RE 𝜖 and unit RE 𝑎 (where 𝑎 ∈ Σ) are in Σ .
2. If 𝑒1 , 𝑒2 ∈ Σ , then alternative 𝑒1 |𝑒2 , concatenation 𝑒1 ⋅ 𝑒2 , repetition 𝑒𝑛,𝑚
(where 0 ≤ 𝑛 ≤ 𝑚 ≤ ∞), and submatch group
1
(𝑒1 ) are in Σ . (Convention: 𝑒1 ⋅ 𝑒2 may be shortened as 𝑒1 𝑒2 , and 𝑒𝑛,𝑛 may be shortened as 𝑒𝑛 .)

Definition 2. Parse trees (PT) over finite alphabet Σ, denoted Σ :
1. Nil tree ⊥𝑖 , empty tree 𝜖 𝑖 and unit tree 𝑎𝑖 are in Σ , where 𝑎 ∈ Σ and 𝑖 ∈ ℤ.
2. If 𝑡1 , … , 𝑡𝑛 ∈ Σ (where 𝑛 ≥ 1) and 𝑖 ∈ ℤ, then 𝑇 𝑖 (𝑡1 , … , 𝑡𝑛 ) is in Σ .

Definition 3. Parenthesized expressions (PE) over finite alphabet Σ, denoted Σ :
1. Nil expression ⟨⟩, empty expression 𝜖 and unit expression 𝑎 (where 𝑎 ∈ Σ) are in Σ .
2. If 𝛼, 𝛽 ∈ Σ , then 𝛼𝛽 and ⟨𝛼⟩ are in Σ .

Definition 4. Tagged Nondeterministic Finite Automaton (TNFA) is a structure (Σ, 𝑄, 𝑀, Δ, 𝑞0 , 𝑞𝑓 ), where:
Σ is a finite set of symbols (alphabet)
𝑄 is a finite set of states
𝑀 is a function that maps tags 𝑚 ∈ ℤ to tuples (𝑆, 𝑁),
where 𝑆 ⊂ ℤ is a set of submatch groups, and 𝑁 ⊂ ℤ is a set of nested tags
Δ = ΔΣ ⊔ Δ𝜖 is the transition relation, consisting of two parts:
ΔΣ contains untagged transitions on symbols of the form (𝑞1 , 𝑎, 𝜖, 𝑞2 )
Δ𝜖 contains optionally tagged 𝜖-transitions with priority of the form (𝑞1 , 𝑛, 𝑚, 𝑞2 )
where 𝑞1 , 𝑞2 ∈ 𝑄, 𝑎 ∈ Σ, 𝑛 ∈ ℤ is priority, 𝑚 ∈ ℤ ∪ {𝜖} is an optional tag
𝑞0 ∈ 𝑄 is the initial state
𝑞𝑓 ∈ 𝑄 is the final state

Our definition of RE is extended with submatch operator and generalized repetition. This is not just syntactic sugar: in POSIX
(a)(a) is semantically different from (a){2}, and (a) in not the same as a. Parse trees have a special nil-tree constructor and
an upper index, which allows us to distinguish between submatch and non-submatch subtrees. Mirroring parse trees, parenthesized expressions have a nil-parenthesis. TNFA is in essence a nondeterministic finite-state transducer which reads symbolic
strings and transduces them to sequences of tags — integer numbers that denote opening and closing parentheses of submatch
groups. Tags on transitions can be negative, which represents the absence of match and corresponds to the nil-parenthesis ⟨⟩
and the nil-tree ⊥. Transition priorities are used to impose specific order of TNFA traversal.
We use the following notation in the algorithms throughout the paper (with possible subscripts and diacritics):
• Integer numbers, indexes and tags are denoted with 𝑖, 𝑗, 𝑘, 𝑙, 𝑚, 𝑛.
• Strings over Σ are denoted with 𝑤 and 𝑎1 … 𝑎𝑛 .
• REs and their variations are denoted with 𝑒.

• PTs are denoted with 𝑡, 𝑠 and 𝑟.

• PEs and PE fragments are denoted with 𝛼, 𝛽, 𝛾, 𝛿.

• TNFA is denoted with 𝐹 , and its states are denoted with 𝑞.

• 𝑈 is the tag path tree — a data structure that stores tag sequences along TNFA paths. Individual paths are represented
with tree indices — integer numbers, denoted with 𝑢. Index zero is the empty path.
• 𝑃 is the precedence matrix — a square matrix indexed by TNFA states, with elements in the {−1, 0, 1} set, denoted with
𝑝. It is used for disambiguation and corresponds to the Okui-Suzuki 𝐷-matrix. The value of a matrix cell 𝑃 [𝑞1 ][𝑞2 ] equals
−1 if the path to 𝑞1 precedes the path to 𝑞2 , 0 if they are equal, or 1 otherwise.

<!-- page 5 -->

5

• 𝐻 is the height matrix — a square integer matrix indexed by TNFA states, with elements denoted with ℎ. It contains
auxiliary data used in the computation of 𝑃 -matrix, and corresponds to the Okui-Suzuki 𝐵-matrix.
• Partial match results (parse trees or POSIX-style offsets) are denoted with 𝑑.

• 𝐶 is a set of configurations (𝑞2 , 𝑞1 , 𝑢, 𝑑), where 𝑞2 is a unique target state, 𝑞1 is the origin state (an index in the 𝐻 and 𝑃
matrices), 𝑢 is the tagged path from 𝑞1 to 𝑞2 (an index in the 𝑈 -tree), and 𝑑 is the partial match result.
• We use either subscript (as in 𝑎𝑖 ) or square brackets (as in 𝑃 [𝑞1 ][𝑞2 ]) to denote the elements of sequences or matrices.

• We use the dot notation (as in 𝑈 [𝑛].𝑝𝑟𝑒𝑑) to access the elements of tuples and structures.
• We avoid global state and explicitly pass parameters to functions.

Below is the skeleton of the matching algorithm:
(
)
1 𝑚𝑎𝑡𝑐ℎ 𝐹 = (Σ, 𝑄, 𝑀, Δ, 𝑞0 , 𝑞𝑓 ), 𝑎1 … 𝑎𝑛
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

14

15

𝐻, 𝑃 ∶ square integer matrices of size |𝑄|
𝑈 ={
𝑒𝑚𝑝𝑡𝑦 𝑝𝑎𝑡ℎ 𝑡𝑟𝑒𝑒()
}
𝐶 = (𝑞0 , ⊥, 0, 𝑖𝑛𝑖𝑡𝑖𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡(𝑀))

for 𝑖 = 1, 𝑛 do
𝐶 = 𝑐𝑙𝑜𝑠𝑢𝑟𝑒(𝐹 , 𝐶, 𝑈 , 𝐻, 𝑃 )
𝐶 = 𝑢𝑝𝑑𝑎𝑡𝑒 𝑟𝑒𝑠𝑢𝑙𝑡(𝑇 , 𝐶, 𝑈 , 𝑖, 𝑎𝑖 )
(𝐻, 𝑃{) = 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒(𝐹 , 𝐶, 𝑈 , 𝐻, 𝑃 )
}
𝐶 = (𝑞2 , 𝑞1 , 0, 𝑑) ∣ (𝑞1 , , , 𝑑) ∈ 𝐶 ∧ (𝑞1 , 𝑎𝑖 , 𝜖, 𝑞2 ) ∈ Δ
if 𝐶 = ∅ then
return ⊥
𝐶 = 𝑐𝑙𝑜𝑠𝑢𝑟𝑒(𝐹 , 𝐶, 𝑈 , 𝐻, 𝑃 )
if ∃(𝑞, , 𝑢, 𝑑) ∈ 𝐶 ∣ 𝑞 = 𝑞𝑓 then
return 𝑓𝑖𝑛𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡(𝑀, 𝑈 , 𝑢, 𝑑, 𝑛)
return ⊥
ALGORITHM 1: TNFA simulation on a string.

The 𝑚𝑎𝑡𝑐ℎ algorithm takes a TNFA 𝐹 and a string 𝑎1 … 𝑎𝑛 as input. During the initialization step it creates an empty 𝑈 -tree
and allocates 𝐻 and 𝑃 matrices (initialization of their elements is not needed, as the initial values are not used). The initial
configuration set 𝐶 contains a single configuration that consists of the initial TNFA state 𝑞0 , undefined origin state ⊥, empty
tagged path and initial match result. The algorithm loops over the input characters 𝑎𝑖 until either all characters are matched,
or the configuration set 𝐶 becomes empty, indicating a match failure. At each step the algorithm constructs 𝜖-closure of the
current configuration set, extending the tagged paths in the 𝑈 -tree, updates the partial match result, re-computes the precedence
information in the 𝐻 and 𝑃 matrices, and steps on TNFA transitions labeled with the current input symbol. Finally, if all
input symbols have been matched and the final configuration set contains a configuration with the final state 𝑞𝑓 , the algorithm
terminates successfully and returns the final match result. Otherwise it returns a failure.
We intentionally leave some parts of the algorithm undefined in this section. They will be addressed in detail in subsequent
sections, after we present the formal foundations of our algorithm in section 3:
• Function 𝑐𝑙𝑜𝑠𝑢𝑟𝑒 constructs 𝜖-closure of a configuration set 𝐶 (section 4). Essentially, it builds shortest paths from the
states in the current set, following 𝜖-transitions in TNFA and comparing paths by the POSIX criterion.

• Functions 𝑒𝑚𝑝𝑡𝑦 𝑝𝑎𝑡ℎ 𝑡𝑟𝑒𝑒, 𝑒𝑥𝑡𝑒𝑛𝑑 𝑝𝑎𝑡ℎ and 𝑢𝑛𝑟𝑜𝑙𝑙 𝑝𝑎𝑡ℎ are used with the 𝑈 -tree (section 5).
• Functions 𝑖𝑛𝑖𝑡𝑖𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡, 𝑢𝑝𝑑𝑎𝑡𝑒 𝑟𝑒𝑠𝑢𝑙𝑡 and 𝑓𝑖𝑛𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡 update match results (section 6).

• Function 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒 computes the 𝐻 and 𝑃 matrices by performing a pairwise comparison of all configurations
in 𝐶 (section 7). It is the key part of the algorithm that allows to compress precedence information about arbitrary long
tagged paths in constant space. If the paths originating from the current configurations join at some future step, the
𝑐𝑙𝑜𝑠𝑢𝑟𝑒 function will compare them using the information in 𝐻 and 𝑃 . If, on the other hand, the paths do not join, then
the comparison performed by 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒 is redundant. Unfortunately, we cannot avoid such redundant comparisons, as we do not know in advance which configurations will spawn ambiguous paths. It is possible to use on-demand
comparison instead of 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒 (section 8), but that requires keeping arbitrary long tagged paths in memory.

<!-- page 6 -->

6

3

FORMALIZATION

In this section we establish the relation between all intermediate representations. For brevity all proofs have been moved to the
appendix. First of all, we rewrite REs in a form that separates submatch information from the RE structure: instead of using
parentheses to denote submatch groups, it stores submatch information for each subexpression in the form of an implicit submatch
index and a set of explicit submatch indices. Explicit indices enumerate submatch groups in RE: an empty set ∅ means that the
subexpression is not a submatch group, and a set with multiple indices means that the subexpression is enclosed in multiple
submatch groups, as in ((𝑒)). Implicit indices enumerate all subexpressions that affect disambiguation, in top-down and leftto-right order (this includes submatch groups and subexpressions that contain nested or sibling submatch groups). The above
representation reflects the POSIX standard, which states that submatch extraction applies only to parenthesized subexpressions,
but the disambiguation rules apply to all subexpressions regardless of parentheses.
Definition 5. Indexed regular expressions (IRE) over finite alphabet Σ, denoted Σ :
1. 𝐸𝑝𝑠(𝑖, 𝐽 ) and 𝑆𝑦𝑚(𝑖, 𝐽 , 𝛼) are in Σ , where 𝛼 ∈ Σ, 𝑖 ∈ ℤ and 𝐽 ⊆ ℤ.
2. If 𝑒1 , 𝑒2 ∈ Σ then alternative 𝐴𝑙𝑡(𝑖, 𝐽 , 𝑒1 , 𝑒2 ), concatenation 𝐶𝑎𝑡(𝑖, 𝐽 , 𝑒1 , 𝑒2 ) and repetition 𝑅𝑒𝑝(𝑖, 𝐽 , 𝑒1 , 𝑛, 𝑚) are in Σ ,
where 𝑖 ∈ ℤ, 𝐽 ⊆ ℤ and 𝑛, 𝑚 ∈ ℤ ∶ 0 ≤ 𝑛 ≤ 𝑚 ≤ ∞.

Here 𝑖 is the implicit submatch index and 𝐽 is the set of explicit submatch indices, denoted 𝑖𝑠𝑢𝑏(𝑒) and 𝑒𝑠𝑢𝑏(𝑒) respectively.
Function 𝐼𝑅𝐸 ∶ Σ → Σ transforms RE into IRE. It is defined via a composition of two functions. Function 𝑚𝑎𝑟𝑘 ∶ ℤ ×
Σ ←→ ℤ × Σ transforms RE into IRE with implicit indices in the boolean range (1 if a subexpression affects disambiguation,
otherwise 0) and computes explicit index sets. It tracks the maximal used value of the explicit index. Function 𝑒𝑛𝑢𝑚 ∶ ℤ×Σ ←→
ℤ × Σ substitutes boolean implicit indices with consecutive numbers. It tracks the maximal used value of the implicit index.
Auxiliary function 𝑓𝑖𝑥 ∶ ℤ × 2ℤ × Σ ←→ Σ applies its input submatch indices to the top-level indices of IRE.
𝑚𝑎𝑟𝑘(𝑗, 𝜖) = (𝑗, 𝐸𝑝𝑠(0, ∅))
𝑚𝑎𝑟𝑘(𝑗, 𝛼) = (𝑗, 𝑆𝑦𝑚(0, ∅, 𝛼))
𝑚𝑎𝑟𝑘(𝑗, 𝑒1 ∣ 𝑒2 ) = (𝑗 ′′ , 𝐴𝑙𝑡(𝑖, ∅, 𝑓𝑖𝑥(𝑖, ∅, 𝑒′1 ), 𝑓𝑖𝑥(𝑖, ∅, 𝑒′2 )))
where (𝑗 ′ , 𝑒′1 ) = 𝑚𝑎𝑟𝑘(𝑗, 𝑒1 )
(𝑗 ′′ , 𝑒′2 ) = 𝑚𝑎𝑟𝑘(𝑗 ′ , 𝑒2 )
𝑖 = 𝑖𝑠𝑢𝑏(𝑒′1 ) ∨ 𝑖𝑠𝑢𝑏(𝑒′2 )
𝑚𝑎𝑟𝑘(𝑗, 𝑒1 ⋅ 𝑒2 ) = (𝑗 ′′ , 𝐶𝑎𝑡(𝑖, ∅, 𝑓𝑖𝑥(𝑖, ∅, 𝑒′1 ), 𝑓𝑖𝑥(𝑖, ∅, 𝑒′2 )))
where (𝑗 ′ , 𝑒′1 ) = 𝑚𝑎𝑟𝑘(𝑗, 𝑒1 )
(𝑗 ′′ , 𝑒′2 ) = 𝑚𝑎𝑟𝑘(𝑗 ′ , 𝑒2 )
𝑖 = 𝑖𝑠𝑢𝑏(𝑒′1 ) ∨ 𝑖𝑠𝑢𝑏(𝑒′2 )
𝑛,𝑚
𝑚𝑎𝑟𝑘(𝑗, 𝑒 ) = (𝑗 ′ , 𝑅𝑒𝑝(𝑖𝑠𝑢𝑏(𝑒′ ), ∅, 𝑒′ , 𝑛, 𝑚))
where (𝑗 ′ , 𝑒′ ) = 𝑚𝑎𝑟𝑘(𝑗, 𝑒)
𝑚𝑎𝑟𝑘(𝑗, (𝑒)) = (𝑗 ′ , 𝑓𝑖𝑥(1, {𝑗}, 𝑒′ ))
where (𝑗 ′ , 𝑒′ ) = 𝑚𝑎𝑟𝑘(𝑗 + 1, 𝑒)

𝑒𝑛𝑢𝑚(𝑘, 𝐸𝑝𝑠(𝑖, 𝐽 )) = (𝑘 + 𝑖, 𝐸𝑝𝑠(𝑘 ⋅ 𝑖, 𝐽 ))
𝑒𝑛𝑢𝑚(𝑘, 𝑆𝑦𝑚(𝑖, 𝐽 , 𝛼)) = (𝑘 + 𝑖, 𝑆𝑦𝑚(𝑘 ⋅ 𝑖, 𝐽 , 𝛼))
𝑒𝑛𝑢𝑚(𝑘, 𝑋𝑜𝑝(𝑖, 𝐽 , 𝑒1 , 𝑒2 )) ∣𝑋𝑜𝑝∈{𝐴𝑙𝑡,𝐶𝑎𝑡} = (𝑘′′ , 𝑋𝑜𝑝(𝑘 ⋅ 𝑖, 𝐽 , 𝑒′1 , 𝑒′2 ))
where (𝑘′ , 𝑒′1 ) = 𝑒𝑛𝑢𝑚(𝑘 + 𝑖, 𝑒1 )
(𝑘′′ , 𝑒′2 ) = 𝑒𝑛𝑢𝑚(𝑘′ , 𝑒2 )
𝑒𝑛𝑢𝑚(𝑘, 𝑅𝑒𝑝(𝑖, 𝐽 , 𝑒, 𝑛, 𝑚)) = (𝑘′ , 𝑅𝑒𝑝(𝑘 ⋅ 𝑖, 𝐽 , 𝑒′ , 𝑛, 𝑚))
where (𝑘′ , 𝑒′ ) = 𝑒𝑛𝑢𝑚(𝑘 + 𝑖, 𝑒)
𝑓𝑖𝑥(𝑘, 𝐾, 𝐸𝑝𝑠(𝑖, 𝐽 )) = 𝐸𝑝𝑠(𝑖∨𝑘, 𝐽 ∪𝐾)
𝑓𝑖𝑥(𝑘, 𝐾, 𝑆𝑦𝑚(𝑖, 𝐽 , 𝛼)) = 𝑆𝑦𝑚(𝑖∨𝑘, 𝐽 ∪𝐾, 𝛼)
𝑓𝑖𝑥(𝑘, 𝐾, 𝑋𝑜𝑝(𝑖, 𝐽 , 𝑒1 , 𝑒2 )) ∣𝑋𝑜𝑝∈{𝐴𝑙𝑡,𝐶𝑎𝑡} = 𝑋𝑜𝑝(𝑖∨𝑘, 𝐽 ∪𝐾, 𝑒1 , 𝑒2 )
𝑓𝑖𝑥(𝑘, 𝐾, 𝑅𝑒𝑝(𝑖, 𝐽 , 𝑒, 𝑛, 𝑚)) = 𝑅𝑒𝑝(𝑖∨𝑘, 𝐽 ∪𝐾, 𝑒, 𝑛, 𝑚)
𝐼𝑅𝐸(𝑒) = 𝑒′′ where ( , 𝑒′ ) = 𝑚𝑎𝑟𝑘(1, 𝑒), ( , 𝑒′′ ) = 𝑒𝑛𝑢𝑚(1, 𝑒′ )

A step-by-step example of constructing an IRE from a RE is given on figure 1 .

The relation between REs and PTs is given by the operator 𝑃 𝑇 ∶ Σ → 2Σ . Each IRE denotes a set of PTs. We write 𝑠𝑡𝑟(𝑡)
to denote the string formed by concatenation of all alphabet symbols in the left-to-right traversal of 𝑡, and 𝑃 𝑇 (𝑒, 𝑤) denotes the
{
}
set 𝑡 ∈ 𝑃 𝑇 (𝑒) ∣ 𝑠𝑡𝑟(𝑡) = 𝑤 of all PTs for IRE 𝑒 and a string 𝑤.
𝑃 𝑇 (𝐸𝑝𝑠(𝑖, )) = {𝜖 𝑖 }
𝑃 𝑇 (𝑆𝑦𝑚(𝑖, , 𝛼)) = {𝛼 𝑖 }
𝑃 𝑇 (𝐴𝑙𝑡(𝑖, , 𝑒1 , 𝑒2 )) = {𝑇 𝑖 (𝑡, ⊥𝑖𝑠𝑢𝑏(𝑒2 ) ) ∣ 𝑡 ∈ 𝑃 𝑇 (𝑒1 )} ∪ {𝑇 𝑖 (⊥𝑖𝑠𝑢𝑏(𝑒1 ) , 𝑡) ∣ 𝑡 ∈ 𝑃 𝑇 (𝑒2 )}
𝑃 𝑇 (𝐶𝑎𝑡(𝑖, , 𝑒1 , 𝑒2 )) = {𝑇 𝑖 (𝑡1 , 𝑡2 ) ∣ 𝑡1 ∈ 𝑃 𝑇 (𝑒1 ), 𝑡2 ∈ 𝑃 𝑇 (𝑒2 )}
𝑃 𝑇 (𝑅𝑒𝑝(𝑖, , 𝑒, 𝑛, 𝑚)) ∣𝑛=0 = {𝑇 𝑖 (𝑡1 , … , 𝑡𝑚 ) ∣ 𝑡𝑘 ∈ 𝑃 𝑇 (𝑒), 1 ≤ 𝑘 ≤ 𝑚} ∪ {𝑇 𝑖 (⊥𝑖𝑠𝑢𝑏(𝑒) )}
𝑃 𝑇 (𝑅𝑒𝑝(𝑖, , 𝑒, 𝑛, 𝑚)) ∣𝑛>0 = {𝑇 𝑖 (𝑡𝑛 , … , 𝑡𝑚 ) ∣ 𝑡𝑘 ∈ 𝑃 𝑇 (𝑒), 𝑛 ≤ 𝑘 ≤ 𝑚}

<!-- page 7 -->

7

𝑚𝑎𝑟𝑘(1, (((𝜖|𝑎0,∞ ))(𝑎|𝜖)0,3 )) = [
𝐶𝑎𝑡(1, {1}, ∗, ∗)Λ
𝑚𝑎𝑟𝑘(2, ((𝜖|𝑎0,∞ ))(𝑎|𝜖)0,∞ ) = [
𝑚𝑎𝑟𝑘(2, ((𝜖|𝑎0,∞ ))) = [
𝐴𝑙𝑡(2, {2, 3}, ∗, ∗)1
𝑅𝑒𝑝(3, ∅, ∗, 0, 3)2
𝑚𝑎𝑟𝑘(3, (𝜖|𝑎0,∞ )) = [
𝑚𝑎𝑟𝑘(4, 𝜖|𝑎0,∞ ) = [
𝐴𝑙𝑡(4, {4}, ∗, ∗)2.1
𝐸𝑝𝑠(0, ∅)1.1 𝑅𝑒𝑝(0, ∅, ∗, 0, ∞)1.2
𝑚𝑎𝑟𝑘(4, 𝜖) = (4, 𝐸𝑝𝑠(0, ∅))
𝑚𝑎𝑟𝑘(4, 𝑎0,∞ ) = [
𝑆𝑦𝑚(0, ∅, 𝑎)1.2.1 𝑆𝑦𝑚(0, ∅, 𝑎)2.1.1 𝐸𝑝𝑠(0, ∅)2.1.2
𝑚𝑎𝑟𝑘(4, 𝑎) = (4, 𝑆𝑦𝑚(0, ∅, 𝑎))
] = (4, 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞))
] = (4, 𝐴𝑙𝑡(0, ∅, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)))
] = (4, 𝐴𝑙𝑡(1, {3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)))
𝑇Λ1 (1)
] = (4, 𝐴𝑙𝑡(1, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)))
𝑚𝑎𝑟𝑘(4, (𝑎|𝜖)0,3 ) = [
𝑇12 (1)
𝑇23 (0)
𝑚𝑎𝑟𝑘(4, (𝑎|𝜖)) = [
𝑚𝑎𝑟𝑘(5, 𝑎|𝜖) = [
0
⊥01.1 (∞) 𝑇1.2
(∞) ⊥42.1 (−1)
𝑚𝑎𝑟𝑘(5, 𝑎) = (5, 𝑆𝑦𝑚(0, ∅, 𝑎))
𝑚𝑎𝑟𝑘(5, 𝜖) = (5, 𝐸𝑝𝑠(0, ∅))
𝑎01.2.1 (∞)
] = (5, 𝐴𝑙𝑡(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝐸𝑝𝑠(0, ∅), 𝑎)))
] = (5, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)))
𝑠 = 𝑇 1 (𝑇 2 (⊥0 , 𝑇 0 (𝑎0 )), 𝑇 3 (⊥4 ))
] = (5, 𝑅𝑒𝑝(1, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3))
] = (5, 𝐶𝑎𝑡(1, ∅, 𝐴𝑙𝑡(1, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)),
𝑇Λ1 (1)
𝑅𝑒𝑝(1, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3)))
] = (5, 𝐶𝑎𝑡(1, {1}, 𝐴𝑙𝑡(1, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)),
𝑅𝑒𝑝(1, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3)))
𝑇12 (0)
𝑇23 (1)
𝑒𝑛𝑢𝑚(1, 𝐶𝑎𝑡(1, {1}, 𝐴𝑙𝑡(1, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)),
𝑅𝑒𝑝(1, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3))) = [
𝑒𝑛𝑢𝑚(2, 𝐴𝑙𝑡(1, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞))) = [
𝑒𝑛𝑢𝑚(3, 𝐸𝑝𝑠(0, ∅)) = (3, 𝐸𝑝𝑠(0, ∅))
𝑒𝑛𝑢𝑚(3, 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)) = [
𝑒𝑛𝑢𝑚(3, 𝑆𝑦𝑚(0, ∅, 𝑎)) = (3, 𝑆𝑦𝑚(0, ∅, 𝑎))
] = (3, 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞))
] = (3, 𝐴𝑙𝑡(2, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)))
𝑒𝑛𝑢𝑚(3, 𝑅𝑒𝑝(1, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3)) = [
𝑒𝑛𝑢𝑚(4, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅))) = [
𝑒𝑛𝑢𝑚(5, 𝑆𝑦𝑚(0, ∅, 𝑎)) = (5, 𝑆𝑦𝑚(0, ∅, 𝑎))
𝑒𝑛𝑢𝑚(5, 𝐸𝑝𝑠(0, ∅)) = (5, 𝐸𝑝𝑠(0, ∅))
] = (5, 𝐴𝑙𝑡(4, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)))
] = (5, 𝑅𝑒𝑝(3, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3))
] = (5, 𝐶𝑎𝑡(1, {1}, 𝐴𝑙𝑡(2, {2, 3}, 𝐸𝑝𝑠(0, ∅), 𝑅𝑒𝑝(0, ∅, 𝑆𝑦𝑚(0, ∅, 𝑎), 0, ∞)),
𝑅𝑒𝑝(3, ∅, 𝐴𝑙𝑡(1, {4}, 𝑆𝑦𝑚(0, ∅, 𝑎), 𝐸𝑝𝑠(0, ∅)), 0, 3)))

0
4
𝜖1.1
(∞) ⊥01.2 (∞) 𝑇2.1
(1)

4
𝑇2.2
(0)

0
𝑎02.1.1 (∞) ⊥02.1.2 (∞) ⊥02.2.1 (∞) 𝜖2.2.2
(∞)

𝑟 = 𝑇 1 (𝑇 2 (𝜖 0 , ⊥0 ), 𝑇 3 (𝑇 4 (𝑎0 , ⊥0 ), 𝑇 4 (⊥0 , 𝜖 0 )))
𝑇Λ1 (1)
𝑇12 (0)

𝑇23 (1)

0
⊥01.1 (∞) 𝑇1.2
(∞)

4
𝑇2.1
(1)

⊥01.2.1 (∞) 𝑎02.1.1 (∞) ⊥02.1.2 (∞)
𝑡 = 𝑇 1 (𝑇 2 (⊥0 , 𝑇 0 (⊥0 )), 𝑇 3 (𝑇 4 (𝑎0 , ⊥0 )))

FIGURE 1 IRE for RE (((𝜖|𝑎0,∞ ))(𝑎|𝜖)0,3 ) and examples of PTs for string 𝑎 with S-norm in parentheses.
Following Okui and Suzuki, we assign positions to the nodes of IRE and PT. The root position is Λ, and position of the 𝑖-th
subtree of a tree with position 𝑥 is 𝑥.𝑖 (we shorten ‖𝑡‖Λ as ‖𝑡‖). The length of position 𝑥, denoted |𝑥|, is defined as 0 for Λ and
|𝑥| + 1 for 𝑥.𝑖. The subtree of a tree 𝑡 at position 𝑥 is denoted 𝑡|𝑥 . Position 𝑥 is a prefix of position 𝑦 iff 𝑦 = 𝑥.𝑥′ for some 𝑥′ , and
a proper prefix if additionally 𝑥 ≠ 𝑦. Position 𝑥 is a sibling of position 𝑦 iff 𝑦 = 𝑧.𝑖, 𝑥 = 𝑧.𝑗 for some 𝑧 and 𝑖, 𝑗 ∈ ℕ. Positions
are ordered lexicographically. The set of all positions of a tree 𝑡 is denoted 𝑃 𝑜𝑠(𝑡). Additionally, we define a set of submatch
{
}
positions as 𝑆𝑢𝑏(𝑡) = 𝑥 ∈ 𝑃 𝑜𝑠(𝑡) ∣ ∃𝑖 ≠ 0, 𝑠 ∶ 𝑡|𝑥 = 𝑠𝑖 — a subset of 𝑃 𝑜𝑠(𝑡) that contains positions of subtrees with nonzero
implicit submatch index. Intuitively, this is the set of positions important for disambiguation: in the case of ambiguity we do not
need to consider the full trees, just the relevant parts of them. PTs have two definitions of norm, one for 𝑃 𝑜𝑠 and one for 𝑆𝑢𝑏,
which we call p-norm and s-norm respectively.

<!-- page 8 -->

8

Definition 6. The p-norm and s-norm of a PT 𝑡 at position 𝑥 are:
⎧−1

⎪
‖𝑡‖𝑝𝑜𝑠
= ⎨|𝑠𝑡𝑟(𝑡|𝑥 )|
𝑥
⎪∞
⎩

if 𝑥 ∈ 𝑃 𝑜𝑠(𝑡) and 𝑡|𝑥 = ⊥𝑖
if 𝑥 ∈ 𝑃 𝑜𝑠(𝑡) and 𝑡|𝑥 ≠ ⊥𝑖
if 𝑥 ∉ 𝑃 𝑜𝑠(𝑡)

⎧−1

⎪
‖𝑡‖𝑠𝑢𝑏
= ⎨|𝑠𝑡𝑟(𝑡|𝑥 )|
𝑥
⎪∞
⎩

if 𝑥 ∈ 𝑆𝑢𝑏(𝑡) and 𝑡|𝑥 = ⊥𝑖
if 𝑥 ∈ 𝑆𝑢𝑏(𝑡) and 𝑡|𝑥 ≠ ⊥𝑖
if 𝑥 ∉ 𝑆𝑢𝑏(𝑡)

Generally, the norm of a subtree means the number of alphabet symbols in its leaves, with two exceptions. First, for nil subtrees
the norm is −1: intuitively, they have the lowest “ranking” among all possible subtrees. Second, for nonexistent subtrees (those
with positions not in 𝑃 𝑜𝑠(𝑡)) the norm is infinite. This may seem counter-intuitive at first, but it makes sense in the presence of
REs with empty repetitions. According to POSIX, optional empty repetitions are not allowed, and our definition reflects this: if
a tree 𝑠 has a subtree 𝑠|𝑥 corresponding to an empty repetition, and another tree 𝑡 has no subtree at position 𝑥, then the infinite
norm ‖𝑡‖𝑥 “outranks” ‖𝑠‖𝑥 . We define two orders on PTs:

Definition 7 (P-order on PTs). Given parse trees 𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, we say that 𝑡 <𝑥 𝑠 w.r.t. decision
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
position 𝑥 iff ‖𝑡‖𝑝𝑜𝑠
𝑥 > ‖𝑠‖𝑥 and ‖𝑡‖𝑦 = ‖𝑠‖𝑦 ∀𝑦 < 𝑥. We say that 𝑡 < 𝑠 iff 𝑡 <𝑥 𝑠 for some 𝑥.

Definition 8 (S-order on PTs). Given parse trees 𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, we say that 𝑡 ≺𝑥 𝑠 w.r.t. decision
position 𝑥 iff ‖𝑡‖𝑠𝑢𝑏
> ‖𝑠‖𝑠𝑢𝑏
and ‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
∀𝑦 < 𝑥. We say that 𝑡 ≺ 𝑠 iff 𝑡 ≺𝑥 𝑠 for some 𝑥.
𝑥
𝑥
𝑦
𝑦

Definition 9. PTs 𝑡 and 𝑠 are incomparable, denoted 𝑡 ∼ 𝑠, iff neither 𝑡 ≺ 𝑠, nor 𝑠 ≺ 𝑡.
Theorem 1. P-order < is a strict total order on 𝑃 𝑇 (𝑒, 𝑤) for any IRE 𝑒 and string 𝑤.
Theorem 2. S-order ≺ is a strict weak order on 𝑃 𝑇 (𝑒, 𝑤) for any IRE 𝑒 and string 𝑤.

The following theorem 3 establishes an important relation between P-order and S-order. P-order is total, and there is a unique <minimal tree 𝑡𝑚𝑖𝑛 . S-order is partial, it partitions all trees into equivalence classes and there is a whole class of ≺-minimal trees
𝑇𝑚𝑖𝑛 (such trees coincide in submatch positions, but differ in some non-submatch positions). Theorem 3 shows that 𝑡𝑚𝑖𝑛 ∈ 𝑇𝑚𝑖𝑛 .
This means that P-order and S-order “agree” on the notion of minimal tree: we can continuously narrow down 𝑇𝑚𝑖𝑛 until we are
left with 𝑡𝑚𝑖𝑛 . Note that this doesn’t mean that P-order is an extension of S-order: the two orders may disagree. For example,
consider trees 𝑡 and 𝑟 on figure1 : on one hand 𝑡 ≺2.2 𝑟, because ‖𝑡‖𝑠𝑢𝑏
= ∞ > 0 = ‖𝑟‖𝑠𝑢𝑏
and s-norms at all preceding submatch
2.2
2.2
𝑝𝑜𝑠
𝑝𝑜𝑠
positions agree; on the other hand 𝑟 <1.1 𝑡, because ‖𝑡‖1.1 = −1 < 0 = ‖𝑟‖1.1 and p-norms at all preceding positions agree.
Theorem 3. Let 𝑡𝑚𝑖𝑛 be the <-minimal tree in 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, and let 𝑇𝑚𝑖𝑛 be the class of ≺-minimal
trees in 𝑃 𝑇 (𝑒, 𝑤). Then 𝑡𝑚𝑖𝑛 ∈ 𝑇𝑚𝑖𝑛 .
The following theorem 4 states that submatch refinement is a contiguous process: adding more parentheses in RE does not
cardinally change existing submatch results, it only adds new details.

Theorem 4. Let 𝑒, 𝑒′ be two REs, such that 𝑒′ is constructed from 𝑒 by adding a pair of parentheses around some subexpression,
′
and let 𝑒̄ = 𝐼𝑅𝐸(𝑒) and 𝑒̄′ = 𝐼𝑅𝐸(𝑒′ ). If 𝑇𝑚𝑖𝑛 , 𝑇𝑚𝑖𝑛
are the classes of ≺-minimal trees in 𝑃 𝑇 ( ̄𝑒, 𝑤) and 𝑃 𝑇 (𝑒̄′ , 𝑤) respectively
′
for some string 𝑤, then 𝑇𝑚𝑖𝑛 ⊆ 𝑇𝑚𝑖𝑛 .

Following the idea of Okui and Suzuki, we go from comparison of parse trees to comparison of their linearized representation
— parenthesized expressions. Parenthesis ⟨ is opening, and parenthesis ⟩ is closing; the nil-parenthesis ⟨⟩ is both opening and
closing. For convenience we sometimes annotate parentheses with height, which we define as the number of preceding opening
parentheses (including this one) minus the number of preceding closing parentheses (including this one). Explicit height annotations allow us to consider PE fragments in isolation without losing the context of the whole expression. However, height is not
a part of parenthesis itself, and it is not taken into account when comparing the elements of PEs. Function Φ ∶ ℤ × Σ → Σ
transforms PT at the given height into a PE:
⎧𝑠𝑡𝑟(𝑡 )
⎪⟨⟩ℎ
⎪
𝑖
Φℎ (𝑡 ) = ⎨⟨ℎ+1 ⟩ℎ
⎪⟨ℎ+1 𝑎⟩ℎ
⎪
⎩⟨ℎ+1 Φℎ+1 (𝑡1 ) … Φℎ+1 (𝑡𝑛 )⟩ℎ
𝑖

if 𝑖 = 0
if 𝑖 ≠ 0 ∧ 𝑡 = ⊥
if 𝑖 ≠ 0 ∧ 𝑡 = 𝜖
if 𝑖 ≠ 0 ∧ 𝑡 = 𝑎 ∈ Σ
if 𝑖 ≠ 0 ∧ 𝑡 = 𝑇 (𝑡1 , … , 𝑡𝑛 )

<!-- page 9 -->

9

{
}
For a given IRE 𝑒 and string 𝑤 the set of all PEs Φ0 (𝑡) ∣ 𝑡 ∈ 𝑃 𝑇 (𝑒, 𝑤) is denoted 𝑃𝐸(𝑒, 𝑤).

Definition 10 (Frame representation of PEs). A given PE 𝛽 can be represented as 𝛽0 𝑎1 𝛽1 … 𝑎𝑛 𝛽𝑛 , where 𝛽𝑖 is the 𝑖-th frame —
a possibly empty sequence of parentheses between subsequent alphabet symbols 𝑎𝑖 and 𝑎𝑖+1 , or the beginning and end of 𝛽.

Definition 11 (Comparable PE fragments). PE fragments 𝛼 and 𝛽 are comparable if they are prefixes of strings in 𝑃𝐸(𝑒, 𝑤) for
some IRE 𝑒 and string 𝑤 and have the same number of frames.
Definition 12 (Fork). The fork of comparable PE fragments 𝛼 and 𝛽 is the index of the first distinct pair of frames.

We use the following notation. For PE fragments 𝛼 and 𝛽, 𝛼 ⊓ 𝛽 denotes their longest common prefix, and 𝛼∖𝛽 denotes the
suffix of 𝛼 after removing 𝛼 ⊓ 𝛽. For a PE fragment 𝛼, 𝑓𝑖𝑟𝑠𝑡(𝛼) denotes the first parenthesis in 𝛼 (or ⊥ if 𝛼 is empty or begins
with an alphabet symbol), 𝑙𝑎𝑠𝑡ℎ(𝛼) denotes the height of the last parenthesis in 𝛼 (or ∞ if 𝛼 is empty or begins with an alphabet
symbol), and 𝑚𝑖𝑛ℎ(𝛼) denotes the minimal height of parenthesis in 𝛼 (or ∞ if 𝛼 is empty or begins with an alphabet symbol).

Definition 13 (Traces). Let 𝛼, 𝛽 be comparable PE prefixes, such that 𝛼 = 𝛼0 𝑎1 𝛼1 … 𝑎𝑛 𝛼𝑛 , 𝛽 = 𝛽0 𝑎1 𝛽1 … 𝑎𝑛 𝛽𝑛 and 𝑘 is the fork.
We define 𝑡𝑟𝑎𝑐𝑒(𝛼, 𝛽) as the sequence (ℎ0 , … , ℎ𝑛 ), where:
⎧−1
⎪
ℎ𝑖 = ⎨𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(𝛼𝑖 ⊓ 𝛽𝑖 ), 𝑚𝑖𝑛ℎ(𝛼𝑖 ∖𝛽𝑖 ))
⎪𝑚𝑖𝑛(ℎ , 𝑚𝑖𝑛ℎ(𝛼 ))
𝑖−1
𝑖
⎩
(
)
We write 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) to denote 𝑡𝑟𝑎𝑐𝑒(𝛼, 𝛽), 𝑡𝑟𝑎𝑐𝑒(𝛽, 𝛼) .

if 𝑖 < 𝑘
if 𝑖 = 𝑘
if 𝑖 > 𝑘

)
(
Definition 14. (Longest precedence.) Let 𝛼, 𝛽 be comparable PE prefixes and 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) = (ℎ0 , … , ℎ𝑛 ), (ℎ′0 , … , ℎ′𝑛 ) . Then
(
)
(
)
𝛼 ⊏ 𝛽 ⇔ ∃𝑖 ≤ 𝑛 ∶ ℎ𝑖 > ℎ′𝑖 ∧ ℎ𝑗 = ℎ′𝑗 ∀𝑗 > 𝑖 . If neither 𝛼 ⊏ 𝛽, nor 𝛽 ⊏ 𝛼, then 𝛼, 𝛽 are longest-equivalent: 𝛼 ∼ 𝛽 (note that
in this case ℎ𝑖 = ℎ′𝑖 for 1 ≤ 𝑖 ≤ 𝑛).

Definition 15. (Leftmost precedence.) Let 𝛼, 𝛽 be comparable PE prefixes, and let 𝑥 = 𝑓 𝑖𝑟𝑠𝑡(𝛼∖𝛽), 𝑦 = 𝑓 𝑖𝑟𝑠𝑡(𝛽∖𝛼). Then
𝛼 ⊂ 𝛽 ⇔ 𝑥 < 𝑦, where the set of possible values of 𝑥 and 𝑦 is ordered as follows: ⊥ <⟩ < ⟨< ⟨⟩.
(
) (
)
Definition 16. (Longest-leftmost precedence.) Let 𝛼, 𝛽 be comparable PE prefixes, then 𝛼 < 𝛽 ⇔ 𝛼 ⊏ 𝛽 ∨ 𝛼 ∼ 𝛽 ∧ 𝛼 ⊂ 𝛽 .
Theorem 5. If 𝑠, 𝑡 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, then 𝑠 ≺ 𝑡 ⇔ Φℎ (𝑠) < Φℎ (𝑡) ∀ℎ.

Examples of PE comparison are shown on figures 2 and 3 .
𝑡1

𝑎

𝑎
𝑡2

𝑡3

𝑡2

𝑎

𝑎
𝑡3

𝑎

𝑎
𝑡4

𝑎
𝑡5

𝑎

𝑡5

𝑡4

𝑎

𝑎
𝑡6

𝑎
𝑡7

𝑎

𝑎
𝑡8

𝑎

𝑡6

𝑎

𝑎

𝑡9

-1 -1 3 0 -1 3 3 0 -1 3 3 0 -1 -1 3 0 -1 3 3 0 -1 3 3 0 -1 3 3 0 -1 3 3 0
-1 -1 2 0 -1 2 2 0 -1 2 2 0 -1 -1 1 0 -1 1 1 0 -1 2 1 0 -1 1 1 0 -1 1 1 0
-1 3 2 0 -1 3 2 0 -1 -1 2 0 -1 3 2 0 -1 3 2 0 -1 3 2 0 -1 3 2 0
-1 2 2 0 -1 2 2 0 -1 -1 1 0 -1 1 1 0 -1 2 1 0 -1 1 1 0 -1 1 1 0
-1 -1 3 0 -1 2 2 0 -1 2 2 0 -1 -1 3 0 -1 2 2 0 -1 2 2 0
-1 -1 2 0 -1 3 1 0 -1 1 1 0 -1 -1 1 0 -1 1 1 0 -1 1 1 0
-1 2 2 0 -1 2 2 0 -1 -1 2 0 -1 2 2 0 -1 2 2 0
-1 3 1 0 -1 1 1 0 -1 -1 1 0 -1 1 1 0 -1 1 1 0
-1 3 1 0 -1 3 1 0 -1 3 1 0 -1 3 1 0
-1 1 1 0 -1 2 1 0 -1 1 1 0 -1 1 1 0
-1 1 1 0 -1 -1 3 0 -1 -1 3 0
-1 2 1 0 -1 -1 2 0 -1 -1 1 0
-1 2 1 0 -1 2 1 0
-1 1 1 0 -1 1 1 0
-1 -1 2 0
-1 -1 1 0

𝑡1
𝑡2
𝑡3
𝑡4
𝑡5
𝑡6
𝑡7
𝑡8

𝑡7

𝑎

𝑎

𝑎

𝑎

𝑡9

𝑡8

𝑎

𝑎

𝑎

𝑎

𝑎

𝑎

𝑎

⎴
⏞⏞⏞⏞⏞
⎴ ⎴ ⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡1 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟨4 𝑎 ⟩3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴ ⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡2 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟨4 𝑎 ⟩3 ⟩2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴ ⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡3 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟨3 ⟨4 𝑎 ⟩3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡4 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴ ⎴
⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡5 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
⎴ ⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡6 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡7 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡8 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
Φ0 (𝑡9 ) = ⟨1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟨2 ⟨3 ⟨4 𝑎 ⟩3 ⟩2 ⟩1 ⟩0
𝑡1 ≺ 𝑡 2 ≺ 𝑡 3 ≺ 𝑡 4 ≺ 𝑡 5 ≺ 𝑡 7 ≺ 𝑡 6 ≺ 𝑡 8 ≺ 𝑡 9

FIGURE 2 Example of pairwise frame-by-frame comparison of all PEs for RE (((𝑎)1,3 )1,3 )1,3 and string 𝑎𝑎𝑎.
Table entry (𝑡𝑖 , 𝑡𝑗 ) contains 𝑡𝑟𝑎𝑐𝑒𝑠(Φ0 (𝑡𝑖 ), Φ0 (𝑡𝑗 )).

<!-- page 10 -->

10

⟨1 𝑇 1 ⟩0
⟨2 𝑇 2 ⟩1
⊥0

⎴ ⎴ ⎴
𝛼 = Φ0 (𝑠) = ⟨1 ⟨2 𝑎
𝑎 ⟩1 ⟩0
⎴ ⎴ ⎴
𝛽 = Φ0 (𝑡) = ⟨1 ⟨2 𝑎 ⟩1 ⟨2 𝑎 ⟩1 ⟩0

⟨1 𝑇 1 ⟩0
⟨2 𝑇 2 ⟩1
𝑎0

𝑇0

𝑎0

⊥0
1

𝑎0
𝑎0
𝑠 = 𝑇 1 (𝑇 2 (⊥0 , 𝑇 0 (𝑎0 , 𝑎0 )))

⟨2 𝑇 2 ⟩1

2

0

0

2

⊥0
0

0

𝑡 = 𝑇 (𝑇 (𝑎 , ⊥ ), 𝑇 (𝑎 , ⊥ ))

‖𝑠‖𝑠𝑢𝑏
= 2 > 1 = ‖𝑡‖𝑠𝑢𝑏
∧ ‖𝑠‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑥 < 1 ⇒ 𝑠 ≺1 𝑡
1
1
𝑥
𝑥

𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) ∶
′
⎡ℎ0 = ℎ0 = −1
⎢ℎ1 = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟨1 ⟨2 ), 𝑚𝑖𝑛ℎ(𝜖)) = 𝑚𝑖𝑛(2, ∞) = 2
⎢ℎ′ = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟨1 ⟨2 ), 𝑚𝑖𝑛ℎ(⟩1 ⟨2 )) = 𝑚𝑖𝑛(2, 1) = 1
⎢ℎ1 = 𝑚𝑖𝑛(ℎ , 𝑚𝑖𝑛ℎ(⟩ ⟩ )) = 𝑚𝑖𝑛(2, 0) = 0
1
1 0
⎢ 2′
⎣ℎ2 = 𝑚𝑖𝑛(ℎ′1 , 𝑚𝑖𝑛ℎ(⟩1 ⟩0 )) = 𝑚𝑖𝑛(1, 0) = 0
ℎ1 > ℎ′1 ∧ ℎ2 = ℎ′2 ⇒ 𝛼 ⊏ 𝛽 ⇒ 𝛼 < 𝛽

(a) – Rule 1: longest precedence, RE (𝑎|𝑎𝑎)0,∞ and string 𝑎𝑎.
⟨1 𝑇 1 ⟩0
⟨2 𝑎2 ⟩1

⎴ ⎴
⎴
⏞⏞⏞
𝛼 = Φ0 (𝑠) = ⟨1 ⟨2 𝑎 ⟩1 ⟨⟩1 ⟩0
⎴
⎴ ⎴
⏞⏞⏞
𝛽 = Φ0 (𝑡) = ⟨1 ⟨⟩1 ⟨2 𝑎 ⟩1 ⟩0

⟨1 𝑇 1 ⟩0

⊥3 ⟨⟩1

⊥2 ⟨⟩1 ⟨2 𝑎3 ⟩1

𝑠 = 𝑇 1 (𝑎2 , ⊥3 )

𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) ∶
⎡ℎ0 = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟨1 ), 𝑚𝑖𝑛ℎ(⟨2 )) = 𝑚𝑖𝑛(1, 2) = 1
⎢ℎ′0 = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟨1 ), 𝑚𝑖𝑛ℎ(⟨⟩1 ⟨2 )) = 𝑚𝑖𝑛(1, 1) = 1
⎢ℎ = 𝑚𝑖𝑛(ℎ , 𝑚𝑖𝑛ℎ(⟩ ⟨⟩ ⟩ )) = 𝑚𝑖𝑛(1, 0) = 0
0
1 1 0
⎢ 1′
⎣ℎ1 = 𝑚𝑖𝑛(ℎ′0 , 𝑚𝑖𝑛ℎ(⟩1 ⟩0 )) = 𝑚𝑖𝑛(1, 0) = 0

𝑡 = 𝑇 1 (⊥2 , 𝑎3 )

ℎ𝑖 = ℎ′𝑖 ∀𝑖 ∧ 𝑓𝑖𝑟𝑠𝑡(𝛼∖𝛽) = ⟨< ⟨⟩ = 𝑓𝑖𝑟𝑠𝑡(𝛽∖𝛼) ⇒ 𝛼 ∼ 𝛽 ∧ 𝛼 ⊂ 𝛽 ⇒ 𝛼 < 𝛽

‖𝑠‖𝑠𝑢𝑏
= 1 > −1 = ‖𝑡‖𝑠𝑢𝑏
∧ ‖𝑠‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑥 < 1 ⇒ 𝑠 ≺1 𝑡
1
1
𝑥
𝑥

(b) – Rule 2: leftmost precedence, RE (𝑎)|(𝑎) and string 𝑎.

⟨1 𝑇 1 ⟩0
⟨2 𝑇 2 ⟩1
0

𝑎

⊥

⎴ ⎴
𝛼 = Φ0 (𝑠) = ⟨1 ⟨2 𝑎 ⟩1 ⟩0
⎴ ⎴
⏞⏞⏞⏞⏞
⎴
𝛽 = Φ0 (𝑡) = ⟨1 ⟨2 𝑎 ⟩1 ⟨2 ⟩1 ⟩0

⟨1 𝑇 1 ⟩0
⟨2 𝑇 2 ⟩1
0

𝑠 = 𝑇 1 (𝑇 2 (𝑎0 , ⊥0 ))

0

𝑎

⊥

⟨2 𝑇 2 ⟩1
0

⊥

0

𝜖

𝑡 = 𝑇 1 (𝑇 2 (𝑎0 , ⊥0 ), 𝑇 2 (⊥0 , 𝜖 0 ))

‖𝑠‖𝑠𝑢𝑏
= ∞ > 0 = ‖𝑡‖𝑠𝑢𝑏
∧ ‖𝑠‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑥 < 2 ⇒ 𝑠 ≺2 𝑡
2
2
𝑥
𝑥

0

𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) ∶
⎡ℎ0 = ℎ′0 = −1
⎢ℎ1 = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟩1 ), 𝑚𝑖𝑛ℎ(⟩0 )) = 𝑚𝑖𝑛(1, 0) = 0
⎢ℎ′ = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟩ ), 𝑚𝑖𝑛ℎ(⟨ ⟩ ⟩ )) = 𝑚𝑖𝑛(1, 0) = 0
⎣ 1
1
2 1 0

ℎ𝑖 = ℎ′𝑖 ∀𝑖 ∧ 𝑓𝑖𝑟𝑠𝑡(𝛼∖𝛽) =⟩ < ⟨= 𝑓𝑖𝑟𝑠𝑡(𝛽∖𝛼) ⇒ 𝛼 ∼ 𝛽 ∧ 𝛼 ⊂ 𝛽 ⇒ 𝛼 < 𝛽

(c) – Rule 3: no optional empty repetitions, RE (𝑎|𝜖)0,∞ and string 𝑎.

⟨1 𝑇 1 ⟩0

⟨1 𝑇 1 ⟩0

⊥2 ⟨⟩1

⟨2 𝑇 2 ⟩1

𝑠 = 𝑇 1 (⊥2 )

⊥

0

𝜖

⎴
⎴
⏞⏞⏞
𝛼 = Φ0 (𝑠) = ⟨1 ⟨⟩1 ⟩0
⎴
⏞⏞⏞⏞⏞
⎴
𝛽 = Φ0 (𝑡) = ⟨1 ⟨2 ⟩1 ⟩0
0

𝑡 = 𝑇 1 (𝑇 2 (𝜖 0 ))
‖𝑠‖𝑠𝑢𝑏
= 0 > −1 = ‖𝑠‖𝑠𝑢𝑏
∧ ‖𝑠‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑥 < 2 ⇒ 𝑡 ≺2 𝑠
2
2
𝑥
𝑥

𝑡𝑟𝑎𝑐𝑒𝑠(𝛼,
𝛽) ∶
[
ℎ0 = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟨1 ), 𝑚𝑖𝑛ℎ(⟨⟩1 ⟩0 )) = 𝑚𝑖𝑛(1, 0) = 0
ℎ′0 = 𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(⟨1 ), 𝑚𝑖𝑛ℎ(⟨2 ⟩1 ⟩0 )) = 𝑚𝑖𝑛(1, 0) = 0
ℎ𝑖 = ℎ′𝑖 ∀𝑖 ∧ 𝑓𝑖𝑟𝑠𝑡(𝛼∖𝛽) = ⟨⟩ > ⟨= 𝑓𝑖𝑟𝑠𝑡(𝛽∖𝛼) ⇒ 𝛼 ∼ 𝛽 ∧ 𝛽 ⊂ 𝛼 ⇒ 𝛽 < 𝛼

(d) – Rule 4: empty match is better than no match, RE (𝑎|𝜖)0,∞ and string 𝜖.

FIGURE 3 Examples: (a) – (d): four main rules of POSIX comparison.
Next, we go from comparison of PEs to comparison of TNFA paths. We extend the notion of order from PEs to paths.

Definition 17 (Path). A path in TNFA (Σ, 𝑄, 𝑇 , Δ, 𝑞0 , 𝑞𝑓 ) is a sequence of transitions {(𝑞𝑖 , 𝑎𝑖 , 𝑏𝑖 , 𝑞𝑖+1 )}𝑛−1
⊆ Δ, where 𝑛 ∈ ℕ.
𝑖=1
Every path induces a string of alphabet symbols and a mixed string of alphabet symbols and tags that corresponds to a PE
fragment: positive opening tags map to ⟨, positive closing tags map to ⟩, and negative tags map to ⟨⟩. We write 𝑞1 ←→ 𝑞𝑛 to
denote that a path from 𝑞1 to 𝑞𝑛 induces an alphabet string 𝑤 and a PE fragment 𝛼.
𝑤|𝛼

<!-- page 11 -->

11

Definition 18 (Path comparison). A path 𝜋1 = 𝑞1 ←→ 𝑞2 is less than a path 𝜋2 = 𝑞1 ←→ 𝑞3 , denoted 𝜋1 < 𝜋2 , if 𝛼 < 𝛽.
𝑤|𝛼

𝑤|𝛽

Definition 19 (Minimal path). A path in TNFA for IRE 𝑒 is minimal if it induces 𝛼 = 𝑃𝐸(𝑡) for some minimal tree 𝑡 ∈ 𝑃 𝑇 (𝑒).
𝑤′ |𝛽

Definition 20 (Ambiguous paths). Paths 𝑞1 ←→ 𝑞2 and 𝑞1′ ←→ 𝑞2′ are ambiguous if 𝑞1 = 𝑞1′ , 𝑞2 = 𝑞2′ and 𝑤 = 𝑤′ .
𝑤|𝛼

𝑤1 |𝛼

𝑤2 |𝛾

𝑤1 |𝛽

𝑤3 |𝛿

𝑤1 |𝛼

Definition 21 (Join point). For paths 𝜋1 = 𝑞1 ←→ 𝑞 ←→ 𝑞2 and 𝜋2 = 𝑞1 ←→ 𝑞 ←→ 𝑞3 that have ambiguous prefixes 𝑞1 ←→ 𝑞
𝑤1 |𝛽

and 𝑞1 ←→ 𝑞, state 𝑞 is a join point (a state where ambiguous prefixes meet).

In order to justify our TNFA simulation algorithm, we need to show that PEs induced by TNFA paths can be compared incrementally (otherwise we would have to keep full-length PEs, which requires the amount of memory proportional to the length
of input). Justification of incremental comparison consists of two parts: the following lemma 1 justifies comparison between
frames, and lemmas 2, 3, 4 in section 4 justify comparison at join points inside of one frame (this is necessary as the number of
paths in closure may be exponential in the number of states).
Lemma 1 (Frame-by-frame comparison of PEs). If 𝛼, 𝛽 are comparable PE prefixes, 𝑐 is an alphabet symbol and 𝛾 is a singleframe PE fragment, then 𝛼 < 𝛽 implies 𝛼𝑐𝛾 < 𝛽𝑐𝛾.

4

CLOSURE CONSTRUCTION

The problem of constructing 𝜖-closure with POSIX disambiguation can be formulated as a shortest path problem on a directed
graph with weighted arcs. In our case weight is not a number — it is the PE fragment induced by the path. We give two
alternative implementations of 𝑐𝑙𝑜𝑠𝑢𝑟𝑒(): 𝑐𝑙𝑜𝑠𝑢𝑟𝑒 𝑔𝑜𝑟1() and 𝑐𝑙𝑜𝑠𝑢𝑟𝑒 𝑔𝑡𝑜𝑝(). The first algorithm, GOR1, is named after the
well-known Goldberg-Radzik algorithm 15 . The second algorithm, GTOP, is an abbreviation of “global topological order”. Both
algorithms have the usual structure of shortest-path finding algorithms. Each algorithm starts with a set of initial configurations, an empty queue and an empty set of resulting configurations. Initial configurations are enqueued and the algorithm loops
until the queue becomes empty. At each iteration it dequeues a configuration (𝑞2 , 𝑞1 , 𝑢, 𝑑) and scans 𝜖-transitions from state 𝑞.
For a transition (𝑞2 , , 𝛼, 𝑞3 ) it constructs a new configuration (𝑞3 , 𝑞1 , 𝑣, 𝑑) that combines 𝑢 and 𝛼 in an extended path 𝑣. If the
resulting set already contains another configuration for state 𝑞3 , the algorithm chooses the configuration which has a better path
from POSIX perspective. Otherwise it adds the new configuration to the resulting set. If the resulting set has been changed, the
new configuration is enqueued for further scanning. Eventually all states in the 𝜖-closure are explored, no improvements can be
made, and the algorithm terminates.
The difference between GOR1 and GTOP is in the order they inspect configurations. Both algorithms are based on the idea
of topological ordering. Unlike other shortest-path algorithms, their queuing discipline is based on graph structure, not on
the distance estimates. This is crucial, because we do not have any distance estimates: paths can be compared, but there is
no absolute “POSIX-ness” value that we can attribute to each path. GOR1 is described in 15 . It uses two stacks and makes a
number of passes; each pass consists of a depth-first search on the admissible subgraph followed by a linear scan of states that
are topologically ordered by depth-first search. The algorithm is one of the most efficient shortest-path algorithms 16 . 𝑛-Pass
structure guarantees worst-case complexity 𝑂(𝑛 𝑚) of the Bellman-Ford algorithm, where 𝑛 is the number of states and 𝑚 is the
number of transitions in the 𝜖-closure (both can be approximated by TNFA size) 17 . GTOP is a simple algorithm that maintains
one global priority queue (e.g. a binary heap) ordered by the topological index of states (for graphs with cycles, we assume
reverse depth-first post-order). Since GTOP does not have the 𝑛-pass structure, its worst-case complexity is not clear. However,
it is much simpler to implement and in practice it performs almost identically to GOR1 on graphs induced by TNFA 𝜖-closures.
On acyclic graphs, both GOR1 and GTOP have linear 𝑂(𝑛 + 𝑚) complexity.
The general proof of correctness of shortest-path algorithms is out of the scope of this paper. However, we need to justify the
application of these algorithms to our problem. In order to do that, we recall the framework for solving shortest-path algorithms
based on closed semirings described in 12 (section 26.4) and show that our problem fits into this framework. A semiring is a
structure (𝕂, ⊕, ⊗, 0, 1), where 𝕂 is a set, ⊕∶𝕂 × 𝕂 → 𝕂 is an associative and commutative operation with identity element 0,
⊗∶𝕂 × 𝕂 → 𝕂 is an associative operation with identity element 1, ⊗ distributes over ⊕ and 0 is annihilator for ⊗. Additionally,
closed semiring requires that ⊕ is idempotent, any countable ⊕-sum of 𝕂 elements is in 𝕂, and associativity, commutativity,

<!-- page 12 -->

12

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
31
32
33
34
35

𝑐𝑙𝑜𝑠𝑢𝑟𝑒 𝑔𝑜𝑟1(𝐹 = (Σ, 𝑄, 𝑀, Δ, 𝑞0 , 𝑞𝑓 ), 𝐶, 𝑈 , 𝐻, 𝑃 )
𝑋 = (𝐹 , 𝑈 , 𝐻, 𝑃
, 𝑡𝑜𝑝𝑠𝑜𝑟𝑡, 𝑙𝑖𝑛𝑒𝑎𝑟 // stacks of states
, 𝑟𝑒𝑠𝑢𝑙𝑡 // configuration or ⊥ mapped to a state
, 𝑠𝑡𝑎𝑡𝑢𝑠 // status of a state (one of 𝑂𝐹𝐹 , 𝑇𝑂𝑃 , 𝐿𝐼𝑁)
, 𝑎𝑐𝑡𝑖𝑣𝑒 // a boolean indicating if a state needs rescan
, 𝑒𝑡𝑟𝑎𝑛𝑠 // 𝜖-transitions from a state, in priority order
, 𝑛𝑒𝑥𝑡 // index of the active transition for a state
) // context
𝑟𝑒𝑠𝑢𝑙𝑡(𝑞) ≡ ⊥
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) ≡ 𝑂𝐹𝐹
𝑎𝑐𝑡𝑖𝑣𝑒(𝑞) ≡ 𝑓𝑎𝑙𝑠𝑒
𝑛𝑒𝑥𝑡(𝑞) ≡ 1
for 𝑐 = ( , 𝑞, , ) ∈ 𝐶 sorted by inverted 𝑝𝑟𝑒𝑐() do
𝑟𝑒𝑠𝑢𝑙𝑡(𝑞) = 𝑐
𝑝𝑢𝑠ℎ(𝑡𝑜𝑝𝑠𝑜𝑟𝑡, 𝑞)
while 𝑡𝑜𝑝𝑠𝑜𝑟𝑡 is not empty do
while 𝑡𝑜𝑝𝑠𝑜𝑟𝑡 is not empty do
𝑞 = 𝑝𝑜𝑝(𝑡𝑜𝑝𝑠𝑜𝑟𝑡)
if 𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) ≠ 𝐿𝐼𝑁 then
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) = 𝑇𝑂𝑃
𝑝𝑢𝑠ℎ(𝑡𝑜𝑝𝑠𝑜𝑟𝑡, 𝑞)
if ¬𝑠𝑐𝑎𝑛(𝑋, 𝑞, 𝑓𝑎𝑙𝑠𝑒) then
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) = 𝐿𝐼𝑁
𝑝𝑜𝑝(𝑡𝑜𝑝𝑠𝑜𝑟𝑡)
𝑝𝑢𝑠ℎ(𝑙𝑖𝑛𝑒𝑎𝑟, 𝑞)
while 𝑙𝑖𝑛𝑒𝑎𝑟 is not empty do
𝑞 = 𝑝𝑜𝑝(𝑙𝑖𝑛𝑒𝑎𝑟)
if 𝑎𝑐𝑡𝑖𝑣𝑒(𝑞) then
𝑛𝑒𝑥𝑡(𝑞) = 1
𝑎𝑐𝑡𝑖𝑣𝑒(𝑞) = 𝑓𝑎𝑙𝑠𝑒
𝑠𝑐𝑎𝑛(𝑋, 𝑞, 𝑡𝑟𝑢𝑒)
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) = 𝑂𝐹𝐹
return 𝑝𝑟𝑢𝑛𝑒(𝑋)

𝑠𝑐𝑎𝑛(𝑋, 𝑞, 𝑎𝑙𝑙)
36
𝑎𝑛𝑦 = 𝑓𝑎𝑙𝑠𝑒
37
while 𝑛𝑒𝑥𝑡(𝑞) ≤ |𝑒𝑡𝑟𝑎𝑛𝑠(𝑞)| do
38
(𝑞, 𝜖, 𝛼, 𝑞 ′ ) = 𝑒𝑡𝑟𝑎𝑛𝑠(𝑞)[𝑛𝑒𝑥𝑡(𝑞)]
39
𝑛𝑒𝑥𝑡(𝑞) = 𝑛𝑒𝑥𝑡(𝑞) + 1
40
(𝑞 ′′ , 𝑞, 𝑢, 𝑑) = 𝑟𝑒𝑠𝑢𝑙𝑡(𝑞)
41
𝑐1 = 𝑟𝑒𝑠𝑢𝑙𝑡(𝑞 ′ )
42
𝑐2 = (𝑞 ′′ , 𝑞 ′ , 𝑒𝑥𝑡𝑒𝑛𝑑 𝑝𝑎𝑡ℎ(𝑈 , 𝑢, 𝛼), 𝑑)
43
if 𝑐1 = ⊥ ∨ 𝑖𝑛𝑑𝑒𝑔𝑟𝑒𝑒(𝑞 ′ ) < 2 ∨ 𝑙𝑒𝑠𝑠(𝑋, 𝑐2 , 𝑐1 ) then
44
𝑟𝑒𝑠𝑢𝑙𝑡(𝑞 ′ ) = 𝑐2
45
if 𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) = 𝑂𝐹𝐹 then
46
𝑎𝑛𝑦 = 𝑡𝑟𝑢𝑒
47
𝑛𝑒𝑥𝑡(𝑞 ′ ) = 1
48
𝑝𝑢𝑠ℎ(𝑡𝑜𝑝𝑠𝑜𝑟𝑡, 𝑞 ′ )
49
if ¬𝑎𝑙𝑙 then 𝑏𝑟𝑒𝑎𝑘
50
else 𝑎𝑐𝑡𝑖𝑣𝑒(𝑞 ′ ) = 1
51
return 𝑎𝑛𝑦

52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72

73

74

75

76

77

78

79
80
81
82
83
84
85

86
87
88
89
90
91
92

𝑐𝑙𝑜𝑠𝑢𝑟𝑒 𝑔𝑡𝑜𝑝(𝐹 = (Σ, 𝑄, 𝑀, Δ, 𝑞0 , 𝑞𝑓 ), 𝐶, 𝑈 , 𝐻, 𝑃 )
𝑋 = (𝐹 , 𝑈 , 𝐻, 𝑃
, 𝑞𝑢𝑒𝑢𝑒 // priority queue of states
, 𝑟𝑒𝑠𝑢𝑙𝑡 // configuration or ⊥ mapped to a state
, 𝑠𝑡𝑎𝑡𝑢𝑠 // status of a state (one of 𝐼𝑁, 𝑂𝑈 𝑇 )
, 𝑒𝑡𝑟𝑎𝑛𝑠 // 𝜖-transitions from a state, in priority order
) // context
𝑟𝑒𝑠𝑢𝑙𝑡(𝑞) ≡ ⊥
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) ≡ 𝑂𝑈 𝑇
for 𝑐1 = ( , 𝑞, , ) ∈ 𝐶 do
𝑐2 = 𝑟𝑒𝑠𝑢𝑙𝑡(𝑞)
if 𝑐2 = ⊥ ∨ 𝑙𝑒𝑠𝑠(𝑋, 𝑐1 , 𝑐2 ) then
𝑟𝑒𝑠𝑢𝑙𝑡(𝑞) = 𝑐1
if 𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) ≠ 𝐼𝑁 then
𝑘 = 𝑡𝑜𝑝𝑜𝑙𝑜𝑔𝑖𝑐𝑎𝑙 𝑖𝑛𝑑𝑒𝑥(𝑞)
𝑖𝑛𝑠𝑒𝑟𝑡 𝑤𝑖𝑡ℎ 𝑝𝑟𝑖𝑜𝑟𝑖𝑡𝑦(𝑞𝑢𝑒𝑢𝑒, 𝑞, 𝑘)
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) = 𝐼𝑁
while 𝑞𝑢𝑒𝑢𝑒 is not empty do
𝑞 = 𝑒𝑥𝑡𝑟𝑎𝑐𝑡 𝑚𝑖𝑛(𝑞𝑢𝑒𝑢𝑒)
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞) = 𝑂𝑈 𝑇
for (𝑞, 𝜖, 𝛼, 𝑞 ′ ) ∈ 𝑒𝑡𝑟𝑎𝑛𝑠(𝑞) do
(𝑞 ′′ , 𝑞, 𝑢, 𝑑) = 𝑟𝑒𝑠𝑢𝑙𝑡(𝑞)
𝑐1 = 𝑟𝑒𝑠𝑢𝑙𝑡(𝑞 ′ )
𝑐2 = (𝑞 ′′ , 𝑞 ′ , 𝑒𝑥𝑡𝑒𝑛𝑑 𝑝𝑎𝑡ℎ(𝑈 , 𝑢, 𝛼), 𝑑)
if 𝑐1 = ⊥ ∨ 𝑖𝑛𝑑𝑒𝑔𝑟𝑒𝑒(𝑞 ′ ) < 2 ∨ 𝑙𝑒𝑠𝑠(𝑋, 𝑐2 , 𝑐1 ) then
𝑟𝑒𝑠𝑢𝑙𝑡(𝑞 ′ ) = 𝑐2
if 𝑠𝑡𝑎𝑡𝑢𝑠(𝑞 ′ ) ≠ 𝐼𝑁 then
𝑘 = 𝑡𝑜𝑝𝑜𝑙𝑜𝑔𝑖𝑐𝑎𝑙 𝑖𝑛𝑑𝑒𝑥(𝑞 ′ )
𝑖𝑛𝑠𝑒𝑟𝑡 𝑤𝑖𝑡ℎ 𝑝𝑟𝑖𝑜𝑟𝑖𝑡𝑦(𝑞𝑢𝑒𝑢𝑒, 𝑞 ′ , 𝑘)
𝑠𝑡𝑎𝑡𝑢𝑠(𝑞 ′ ) = 𝐼𝑁
return 𝑝𝑟𝑢𝑛𝑒(𝑋)
𝑝𝑟𝑢𝑛𝑒(𝑋)
return {𝑐 ∣ 𝑐 = ( , 𝑞, , ) = 𝑟𝑒𝑠𝑢𝑙𝑡(𝑞), 𝑞 ∈ 𝑄
∧ (𝑞 = 𝑞𝑓 ∨ ∃(𝑞 ′ , 𝑎, , ) ∈ Δ ∣ 𝑞 = 𝑞 ′ ∧ 𝑎 ∈ Σ)}
𝑙𝑒𝑠𝑠(𝑋, 𝑐1 , 𝑐2 )
( , , 𝑙) = 𝑐𝑜𝑚𝑝𝑎𝑟𝑒(𝑐1 , 𝑐2 , 𝑈 , 𝐻, 𝑃 )
return 𝑙 < 0
𝑝𝑟𝑒𝑐(𝑋, 𝑐1 , 𝑐2 )
(𝑞1 , , , ) = 𝑐1
(𝑞2 , , , ) = 𝑐2
return 𝑃 [𝑞1 ][𝑞2 ] < 0

ALGORITHM 2: Closure algorithms GOR1 and GTOP. Context components are addressed without qualification in the
subroutines, e.g. 𝑈 rather than 𝑋.𝑈 . Functions 𝑐𝑜𝑚𝑝𝑎𝑟𝑒() and 𝑒𝑥𝑡𝑒𝑛𝑑 𝑝𝑎𝑡ℎ() are defined in sections 7 and 5. The definition
of 𝑝𝑢𝑠ℎ(), 𝑝𝑜𝑝(), 𝑖𝑛𝑠𝑒𝑟𝑡 𝑤𝑖𝑡ℎ 𝑝𝑟𝑖𝑜𝑟𝑖𝑡𝑦(), 𝑒𝑥𝑡𝑟𝑎𝑐𝑡 𝑚𝑖𝑛(), 𝑖𝑛𝑑𝑒𝑔𝑟𝑒𝑒() and 𝑡𝑜𝑝𝑜𝑙𝑜𝑔𝑖𝑐𝑎𝑙 𝑖𝑛𝑑𝑒𝑥() is omitted for brevity.

<!-- page 13 -->

13

distributivity and idempotence apply to countable ⊕-sums. Mohri generalizes this definition and notes that either left or right
distributivity is sufficient 13 . In our case 𝕂 is the set of closure paths without tagged 𝜖-loops: the following lemma 2 and 3
show that, on one hand, paths with tagged 𝜖-loops are not minimal, and on the other hand such paths are discarded by the
algorithm, so they can be removed from consideration. Consequently 𝕂 is finite. We have semiring (𝕂, 𝑚𝑖𝑛, ⋅, ⊥, 𝜖), where 𝑚𝑖𝑛
is POSIX comparison of ambiguous paths, ⋅ is concatenation of paths at the join points (subject to restriction that paths do
not contain tagged 𝜖-loops and remain within TNFA bounds — concatenation of arbitrary paths is not in 𝕂), ⊥ corresponds to
artificial infinitely long path, and 𝜖 is the empty path. It is easy to see that 𝑚𝑖𝑛 is commutative and associative, ⊥ is identity
for 𝑚𝑖𝑛 (𝑚𝑖𝑛(𝜋, ⊥) = 𝑚𝑖𝑛(⊥, 𝜋) = 𝜋), ⋅ is associative, 𝜖 is identity for ⋅ (𝜋 ⋅ 𝜖 = 𝜖 ⋅ 𝜋 = 𝜋), and ⊥ is an annihilator for ⋅
(𝜋 ⋅ ⊥ = ⊥ ⋅ 𝜋 = ⊥). Right distributivity of ⋅ over 𝑚𝑖𝑛 for paths with at most one 𝜖-loop is given by lemma 4. Idempotence holds
because 𝑚𝑖𝑛(𝜋, 𝜋) = 𝜋. Since 𝕂 is finite, the properties for ⊕-sums over countable subsets are satisfied.
Lemma 2. Minimal paths do not contain tagged 𝜖-loops.

Lemma 3. GOR1 and GTOP discard paths with tagged 𝜖-loops.
Lemma 4 (Right distributivity of comparison over concatenation for paths without tagged 𝜖-loops). Let 𝜋𝛼 = 𝑞0 ←→ 𝑞1 and
𝑢|𝛼

𝜋𝛽 = 𝑞0 ←→ 𝑞1 be ambiguous paths in TNFA 𝑓 for IRE 𝑒, and let 𝜋𝛾 = 𝑞1 ←→ 𝑞2 be their common 𝜖-suffix, such that 𝜋𝛼 𝜋𝛾 and
𝜋𝛽 𝜋𝛾 do not contain tagged 𝜖-loops. If 𝛼 < 𝛽 then 𝛼𝛾 < 𝛽𝛾.
𝑢|𝛽

5

𝜖|𝛾

TREE REPRESENTATION OF PATHS

In this section we specify the representation of path fragments in configurations and define tag path tree 𝑈 and the associated
functions. The 𝑈 -tree is a prefix tree of tags induced by the shortest path tree that is constructed by 𝑐𝑙𝑜𝑠𝑢𝑟𝑒(). Some care is
necessary with TNFA construction in order to ensure the prefix property (subautomata for alternatives should not start with
identical tagged transitions), but that is easy to accommodate and we give the details in section 9. The 𝑈 -tree can be stored as
an array of nodes (𝑝𝑟𝑒𝑑, 𝑠𝑢𝑐𝑐, 𝑡𝑎𝑔) where 𝑝𝑟𝑒𝑑 is the index of a predecessor node, 𝑠𝑢𝑐𝑐 is an ordered set of successor indices,
and 𝑡𝑎𝑔 is a positive or negative tag. Successor indices are only necessary if the advanced algorithm for 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒() is
used (section 7), otherwise the 𝑠𝑢𝑐𝑐 component can be omitted. Individual paths in the 𝑈 -tree are addressed by integer indices
of tree nodes (zero index corresponds to the empty path). It is important to use numeric indices rather than pointers to nodes
because it allows the use of the “two-fingers” algorithm that find the fork of two paths (section 7). The prefix tree representation
is space efficient (common path prefixes are not duplicated), and the operation of copying a path (e.g. from one configuration to
another) is as cheap as copying an integer index. This representation was used by multiple researches, e.g. Laurikari mentions
a functional data structure 4 and Karper describes it as the flyweight pattern 14 .
1

𝑒𝑚𝑝𝑡𝑦 𝑝𝑎𝑡ℎ 𝑡𝑟𝑒𝑒()
2
return an empty list of tuples (𝑝𝑟𝑒𝑑, 𝑠𝑢𝑐𝑐, 𝑡𝑎𝑔)
3 𝑒𝑥𝑡𝑒𝑛𝑑 𝑝𝑎𝑡ℎ(𝑈 , 𝑛, 𝑡)
4
if 𝑡 ≠ 𝜖 then
5
𝑚 = 𝑙𝑒𝑛𝑔𝑡ℎ(𝑈 ) + 1
6
𝑖𝑛𝑠𝑒𝑟𝑡(𝑈 [𝑛].𝑠𝑢𝑐𝑐, 𝑚)
7
𝑎𝑝𝑝𝑒𝑛𝑑(𝑈 , (𝑛, ∅, 𝑡))
8
return 𝑚
9
else return 𝑛

10
11
12
13
14
15

𝑢𝑛𝑟𝑜𝑙𝑙 𝑝𝑎𝑡ℎ(𝑈 , 𝑛)

𝑠=𝜖
while 𝑛 ≠ 0 do
𝑎𝑝𝑝𝑒𝑛𝑑(𝑠, 𝑈 [𝑛].𝑡𝑎𝑔)
𝑛 = 𝑈 [𝑛].𝑝𝑟𝑒𝑑
return 𝑟𝑒𝑣𝑒𝑟𝑠𝑒(𝑠)

ALGORITHM 3: Operations on tag path tree. The definition of 𝑖𝑛𝑠𝑒𝑟𝑡(), 𝑎𝑝𝑝𝑒𝑛𝑑() and 𝑟𝑒𝑣𝑒𝑟𝑠𝑒() is omitted for brevity.

6

REPRESENTATION OF MATCH RESULTS

In this section we show two ways to construct match results: POSIX offsets and a parse tree.
In the case of POSIX offsets, the 𝑑-component of a configuration is an array of offsets for each tag. Offsets are updated incrementally at each step by scanning the corresponding path fragment and setting negative tags to −1 and positive tags to the

<!-- page 14 -->

14

current step number. For negative tags, it is necessary to also set all nested tags to −1. Only the most recent value of each tag is
needed, therefore the offset is updated at most once. At the end of match, the offset array is converted to the representation specified by the POSIX standard: an array 𝑝𝑚𝑎𝑡𝑐ℎ of offset pairs (𝑟𝑚 𝑠𝑜, 𝑟𝑚 𝑒𝑜). Conversion takes into account the mapping of tags
to submatch groups. Section 9 shows the construction of the mapping 𝑀 from tags to their nested tags and submatch groups.
In the case of a parse tree, the 𝑑-component of a configuration is a tagged string that is accumulated at each step, and eventually
converted to a parse tree at the end of match. The resulting parse tree is only partially structured: leaves that correspond to
subexpressions with zero implicit submatch index contain “flattened” substring of alphabet symbols. It is possible to construct
parse trees incrementally as well, but this is more complex and the partial trees may require even more space than tagged strings.
1

𝑖𝑛𝑖𝑡𝑖𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡(𝑀)
2
𝑑: integer array of size |𝑑𝑜𝑚(𝑀)|
3
return 𝑑
𝑢𝑝𝑑𝑎𝑡𝑒 𝑟𝑒𝑠𝑢𝑙𝑡(𝑀, 𝐶, 𝑈 , 𝑘, )
{
5
return (𝑞2 , 𝑞1 , 𝑢, 𝑎𝑝𝑝𝑙𝑦 𝑡𝑎𝑔𝑠(𝑀,
} 𝑈 , 𝑢, 𝑑, 𝑘))
6
∣ (𝑞2 , 𝑞1 , 𝑢, 𝑑) ∈ 𝐶
4

7

𝑎𝑝𝑝𝑙𝑦 𝑡𝑎𝑔𝑠(𝑀, 𝑈 , 𝑢, 𝑑, 𝑘)
8
𝑑𝑜𝑛𝑒(𝑡) = 𝑓𝑎𝑙𝑠𝑒 ∀𝑡
9
while 𝑢 ≠ 0 do
10
𝑡 = 𝑈 [𝑢].𝑡𝑎𝑔
11
if 𝑡 > 0 then
12
if ¬𝑑𝑜𝑛𝑒(𝑡) then
13
𝑑𝑜𝑛𝑒(𝑡) = 𝑡𝑟𝑢𝑒
14
𝑑[𝑡] = 𝑘
15
else
16
for 𝑛 ∈ 𝑀(−𝑡).𝑁 do
17
if ¬𝑑𝑜𝑛𝑒(𝑛) then
18
𝑑𝑜𝑛𝑒(𝑛) = 𝑡𝑟𝑢𝑒
19
𝑑[𝑛] = −1
20
𝑢 = 𝑈 [𝑢].𝑝𝑟𝑒𝑑
21
return 𝑑
22

𝑓𝑖𝑛𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡(𝑀, 𝑈 , 𝑢, 𝑑, 𝑘)
𝑑 ′ = 𝑎𝑝𝑝𝑙𝑦 𝑡𝑎𝑔𝑠(𝑀, 𝑈 , 𝑢, 𝑑, 𝑘)
𝑝𝑚𝑎𝑡𝑐ℎ[0].𝑟𝑚 𝑠𝑜 = 0
𝑝𝑚𝑎𝑡𝑐ℎ[0].𝑟𝑚 𝑒𝑜 = 𝑘
26
for 𝑖 = 1, |𝑑𝑜𝑚(𝑀)|∕2 do
27
for 𝑠 ∈ 𝑀(2𝑖).𝑆 do
28
𝑝𝑚𝑎𝑡𝑐ℎ[𝑠].𝑟𝑚 𝑠𝑜 = 𝑑 ′ [2𝑖 − 1]
29
𝑝𝑚𝑎𝑡𝑐ℎ[𝑠].𝑟𝑚 𝑒𝑜 = 𝑑 ′ [2𝑖]
30
return 𝑝𝑚𝑎𝑡𝑐ℎ
23
24
25

31

𝑖𝑛𝑖𝑡𝑖𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡( )

32

return 𝜖

33

𝑢𝑝𝑑𝑎𝑡𝑒 𝑟𝑒𝑠𝑢𝑙𝑡( , 𝐶, 𝑈 , , 𝑎)
{
34
return (𝑞2 , 𝑞1 , 𝑢, 𝑑 ⋅ 𝑢𝑛𝑟𝑜𝑙𝑙 𝑝𝑎𝑡ℎ(𝑈
, 𝑢) ⋅ 𝑎)
}
35
∣ (𝑞2 , 𝑞1 , 𝑢, 𝑑) ∈ 𝐶
36
37

38
39
40
41

42
43
44
45
46
47
48
49

50
51
52
53

𝑝𝑎𝑟𝑠𝑒 𝑡𝑟𝑒𝑒(𝑢, 𝑖)
if 𝑢 = (2𝑖−1) ⋅ (2𝑖) then
return 𝑇 𝑖 (𝜖)
if 𝑢 = (1−2𝑖) ⋅ … then
return 𝑇 𝑖 (⊥)
if 𝑢 = (2𝑖−1) ⋅ 𝑎1 … 𝑎𝑛 ⋅ (2𝑖) ∧ 𝑎1 , … , 𝑎𝑛 ∈ Σ then
return 𝑇 𝑖 (𝑎1 , … , 𝑎𝑛 )
if 𝑢 = (2𝑖−1) ⋅ 𝛽1 … 𝛽𝑚 ⋅ (2𝑖) ∧ 𝛽1 = 2𝑗 −1 ∈ 𝑇 then
𝑛 = 0, 𝑘 = 1
while 𝑘 ≤ 𝑚 do
𝑙=𝑘
while |𝛽𝑘+1 | > 2𝑗 do 𝑘 = 𝑘 + 1
𝑛=𝑛+1
𝑡𝑛 = 𝑝𝑎𝑟𝑠𝑒 𝑡𝑟𝑒𝑒(𝛽𝑙 … 𝛽𝑘 , 𝑗)
return 𝑇 𝑖 (𝑡1 , … , 𝑡𝑛 )
return ⊥ // ill-formed PE
𝑓𝑖𝑛𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡( , 𝑈 , 𝑢, 𝑑, )
return 𝑝𝑎𝑟𝑠𝑒 𝑡𝑟𝑒𝑒(𝑑 ⋅ 𝑢𝑛𝑟𝑜𝑙𝑙 𝑝𝑎𝑡ℎ(𝑈 , 𝑢), 1)

ALGORITHM 4: Construction of match results: POSIX offsets (on the left) and parse tree (on the right).

7

DISAMBIGUATION PROCEDURES

In this section we define disambiguation procedures 𝑐𝑜𝑚𝑝𝑎𝑟𝑒() and 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒(). The pseudocode follows definition 16
closely and relies on the prefix tree representation of paths given in section 6. In order to find fork of two paths in 𝑐𝑜𝑚𝑝𝑎𝑟𝑒() we
used the so-called “two-fingers” algorithm, which is based on the observation that parent index is always less than child index.
Given two indices 𝑛1 and 𝑛2 , we continuously set the greater index to its parent until the indices become equal, at which point
we have either found fork or the root of 𝑈 -tree. We track minimal height of each path along the way and memorize the pair of
indices right after the fork — they are used to determine the leftmost path in case of equal heights.

<!-- page 15 -->

15
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

𝑐𝑜𝑚𝑝𝑎𝑟𝑒(𝑐1 , 𝑐2 , 𝑈 , 𝐻, 𝑃 )
( , 𝑞1 , 𝑢1 , ) = 𝑐1
( , 𝑞2 , 𝑢2 , ) = 𝑐2
if 𝑞1 = 𝑞2 ∧ 𝑢1 = 𝑢2 then return (∞, ∞, 0)
𝑓𝑜𝑟𝑘 = (𝑞1 = 𝑞2 )
if 𝑓𝑜𝑟𝑘 then ℎ1 = ℎ2 = ∞
else ℎ1 = 𝐻[𝑞1 ][𝑞2 ], ℎ2 = 𝐻[𝑞2 ][𝑞1 ]
𝑢′1 = 𝑢′2 = ⊥
while 𝑢1 ≠ 𝑢2 do
if 𝑢1 > 𝑢2 then
ℎ1 = 𝑚𝑖𝑛(ℎ1 , ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢1 ].𝑡𝑎𝑔))
𝑢′1 = 𝑢1 , 𝑢1 = 𝑈 [𝑢1 ].𝑝𝑟𝑒𝑑
else
ℎ2 = 𝑚𝑖𝑛(ℎ2 , ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢2 ].𝑡𝑎𝑔))
𝑢′2 = 𝑢2 , 𝑢2 = 𝑈 [𝑢2 ].𝑝𝑟𝑒𝑑
if 𝑢1 ≠ ⊥ then
ℎ = ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢1 ].𝑡𝑎𝑔)
ℎ1 = 𝑚𝑖𝑛(ℎ1 , ℎ), ℎ2 = 𝑚𝑖𝑛(ℎ2 , ℎ)
if ℎ1 > ℎ2 then 𝑙 = −1
else if ℎ1 < ℎ2 then 𝑙 = 1
else if ¬𝑓𝑜𝑟𝑘 then 𝑙 = 𝑃 [𝑞1 ][𝑞2 ]
else 𝑙 = 𝑙𝑒𝑓𝑡𝑝𝑟𝑒𝑐(𝑢′1 , 𝑢′2 , 𝑈 )
return (ℎ1 , ℎ2 , 𝑙)

𝑙𝑒𝑓𝑡𝑝𝑟𝑒𝑐(𝑢1 , 𝑢2 , 𝑈 )

33

if 𝑢1 = 𝑢2 then return 0
if 𝑢1 = ⊥ then return −1
if 𝑢2 = ⊥ then return 1
𝑡1 = 𝑈 [𝑢1 ].𝑡𝑎𝑔, 𝑡2 = 𝑈 [𝑢2 ].𝑡𝑎𝑔
if 𝑡1 < 0 then return 1
if 𝑡2 < 0 then return −1
if 𝑡1 𝑚𝑜𝑑 2 ≡ 0 then return −1
if 𝑡2 𝑚𝑜𝑑 2 ≡ 0 then return 1
return 0

34

𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒(𝐹 , 𝐶, 𝑈 , 𝐻, 𝑃 )

25
26
27
28
29
30
31
32

35
36
37
38

39
40

for 𝑐1 = (𝑞1 , , , ) ∈ 𝐶 do
for 𝑐2 = (𝑞2 , , , ) ∈ 𝐶 do
(ℎ1 , ℎ2 , 𝑙) = 𝑐𝑜𝑚𝑝𝑎𝑟𝑒(𝑐1 , 𝑐2 , 𝑈 , 𝐻, 𝑃 )
𝐻 ′ [𝑞1 ][𝑞2 ] = ℎ1 , 𝑃 ′ [𝑞1 ][𝑞2 ] = 𝑙
𝐻 ′ [𝑞2 ][𝑞1 ] = ℎ2 , 𝑃 ′ [𝑞2 ][𝑞1 ] = −𝑙
return (𝐻 ′ , 𝑃 ′ )

41
42

43
44
45
46
47
48
49
50
51
52
53
54
55
56

57
58

59

60
61
62
63
64
65
66
67
68
69
70

𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒(𝐹 , 𝐶, 𝑈 , 𝐻, 𝑃 )

𝑠𝑡𝑎𝑐𝑘 ∶ empty stack of 𝑈 -tree indices
𝑙𝑒𝑣𝑒𝑙 ∶ empty array of tuples (𝑞 ′ , 𝑞, 𝑢, ℎ)
𝑛𝑒𝑥𝑡 ∶ active successor for a given 𝑈 -tree index
𝑛𝑒𝑥𝑡(𝑢) ≡ 1
𝑖=0
𝑝𝑢𝑠ℎ(𝑠𝑡𝑎𝑐𝑘, 0)
while 𝑠𝑡𝑎𝑐𝑘 is not empty do
𝑢 = 𝑝𝑜𝑝(𝑠𝑡𝑎𝑐𝑘)
if 𝑛𝑒𝑥𝑡(𝑢) < 𝑘 then
𝑝𝑢𝑠ℎ(𝑠𝑡𝑎𝑐𝑘, 𝑢)
𝑝𝑢𝑠ℎ(𝑠𝑡𝑎𝑐𝑘, 𝑈 [𝑢].𝑠𝑢𝑐𝑐[𝑛𝑒𝑥𝑡(𝑢)])
𝑛𝑒𝑥𝑡(𝑢) = 𝑛𝑒𝑥𝑡(𝑢) + 1
𝑐𝑜𝑛𝑡𝑖𝑛𝑢𝑒
ℎ = ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢].𝑡𝑎𝑔), 𝑖1 = 𝑖
for (𝑞 ′ , 𝑞, 𝑢1 , ) ∈ 𝐶 ∣ 𝑢1 = 𝑢 do
𝑖 = 𝑖 + 1, 𝑙𝑒𝑣𝑒𝑙[𝑖] = (𝑞 ′ , 𝑞, ⊥, ℎ)
for 𝑗1 = 𝑖1 + 1, 𝑖 do
for 𝑗2 = 𝑗1 , 𝑖 do
(𝑞1′ , 𝑞1 , , ) = 𝑙𝑒𝑣𝑒𝑙[𝑗1 ]
(𝑞2′ , 𝑞2 , , ) = 𝑙𝑒𝑣𝑒𝑙[𝑗2 ]
if 𝑢 = 0 ∧ 𝑞1 ≠ 𝑞2 then
ℎ1 = 𝐻[𝑞1 ][𝑞2 ], ℎ2 = 𝐻[𝑞2 ][𝑞1 ]
𝑙 = 𝑃 [𝑞1 ][𝑞2 ]
else ℎ1 = ℎ2 = ℎ, 𝑙 = 0

𝐻 ′ [𝑞1′ ][𝑞2′ ] = ℎ1 , 𝑃 ′ [𝑞1′ ][𝑞2′ ] = 𝑙
𝐻 ′ [𝑞2′ ][𝑞1′ ] = ℎ2 , 𝑃 ′ [𝑞2′ ][𝑞1′ ] = −𝑙

for 𝑢′ ∈ 𝑈 [𝑢].𝑠𝑢𝑐𝑐 in reverse do
𝑖2 = 𝑖1
while 𝑖2 > 0 ∧ 𝑙𝑒𝑣𝑒𝑙[𝑖2 ].𝑢 = 𝑢′ do 𝑖2 = 𝑖2 − 1

71
72

for 𝑗1 = 𝑖2 , 𝑖1 do
𝑙𝑒𝑣𝑒𝑙[𝑗1 ].ℎ = 𝑚𝑖𝑛(𝑙𝑒𝑣𝑒𝑙[𝑗1 ].ℎ, ℎ);

73

for 𝑗2 = 𝑖1 , 𝑖 do
(𝑞1′ , 𝑞1 , 𝑢1 , ℎ1 ) = 𝑙𝑒𝑣𝑒𝑙[𝑗1 ]
(𝑞2′ , 𝑞2 , 𝑢2 , ℎ2 ) = 𝑙𝑒𝑣𝑒𝑙[𝑗2 ]

74

75
76
77
78
79
80
81
82
83
84

85
86
87

if 𝑢 = 0 ∧ 𝑞1 ≠ 𝑞2 then
ℎ1 = 𝑚𝑖𝑛(ℎ1 , 𝐻[𝑞1 ][𝑞2 ])
ℎ2 = 𝑚𝑖𝑛(ℎ2 , 𝐻[𝑞2 ][𝑞1 ])
if ℎ1 > ℎ2 then 𝑙 = −1
else if ℎ1 < ℎ2 then 𝑙 = 1
else if 𝑞1 ≠ 𝑞2 then 𝑙 = 𝑃 [𝑞1 ][𝑞2 ]
else 𝑙 = 𝑙𝑒𝑓𝑡𝑝𝑟𝑒𝑐(𝑢1 , 𝑢2 , 𝑈 )

𝐻 ′ [𝑞1′ ][𝑞2′ ] = ℎ1 , 𝑃 ′ [𝑞1′ ][𝑞2′ ] = 𝑙
𝐻 ′ [𝑞2′ ][𝑞1′ ] = ℎ2 , 𝑃 ′ [𝑞2′ ][𝑞1′ ] = −𝑙
𝑖1 = 𝑖2
for 𝑗 = 𝑖1 , 𝑖 do 𝑙𝑒𝑣𝑒𝑙[𝑗].𝑢 = 𝑢
return (𝐻 ′ , 𝑃 ′ )

ALGORITHM 5: Disambiguation procedures. Function ℎ𝑒𝑖𝑔ℎ𝑡() gives the height of a tag; its definition is omitted for
brevity (it can be computed in one top-down pass over IRE when constructing the TNFA, see section 9).

We give two alternative algorithms for 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒(): a simple one with 𝑂(𝑚2 𝑡) complexity (on the left) and a complex
one with 𝑂(𝑚2 ) complexity (on the right), where 𝑚 is the number of states in the 𝜖-closure and 𝑡 is the number of tags. The
worst case is demonstrated by RE ((𝑎|𝜖)0,𝑘 )0,∞ where 𝑘 ∈ ℕ, for which the simple algorithm takes 𝑂(𝑘3 ) time and the complex

<!-- page 16 -->

16

algorithm takes 𝑂(𝑘2 ) time. The idea of the complex algorithm is to avoid repeated re-scanning of path prefixes in the 𝑈 -tree. It
makes one pass over the tree, constructing an array 𝑙𝑒𝑣𝑒𝑙 of items (𝑞 ′ , 𝑞, 𝑢, ℎ), where 𝑞 ′ and 𝑞 are target and origin states (as in
configurations), 𝑢 is the current tree index, and ℎ is the current minimal height. One item is added per each closure configuration
(𝑞 ′ , 𝑞, 𝑢, 𝑑) when traversal reaches the tree node with index 𝑢. After a subtree has been traversed, the algorithm scans 𝑙𝑒𝑣𝑒𝑙
items added during traversal of this subtree (such items are distinguished by their 𝑢-component), sets their ℎ-component to the
minimum of ℎ and the height of tag at the current node, and computes the new value of 𝐻 and 𝑃 matrices for each pair of
target states in items from different branches. After that, 𝑢-component of all scanned items is downgraded to the tree index of
the current node (erasing the difference between items from different branches).

8

LAZY DISAMBIGUATION

Most of the disambiguation overhead in our algorithm comes from updating the 𝐻 and 𝑃 matrices at each step. It is all the more
unfortunate since many comparisons performed by 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒() are useless — the compared paths may never meet. In
fact, if the input is unambiguous, all comparisons are useless. A natural idea, therefore, is to compare paths only in case of a
real ambiguity (when they meet in closure) and avoid the computation of precedence matrices altogether. We can do it with a
few modifications to our original algorithm. First, we no longer need the 𝐻 and 𝑃 matrices and 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒() function.
Instead, we introduce a cache 𝑍 that maps a pair of 𝑈 -tree indices (𝑢1 , 𝑢2 ) to a triple of precedence values (ℎ1 , ℎ2 , 𝑙). The 𝑍cache stores the “useful” part of 𝐻 and 𝑃 matrices on multiple preceding steps. It is populated lazily during disambiguation
and allows us to avoid re-computing the same values multiple times. Second, we need to modify the 𝑈 -tree representation of
paths in the following way: successor indices are no longer needed (they are only used in the advanced 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒()
algorithm), and the 𝑈 -tree nodes must be augmented with the current step and the origin state: (𝑝𝑟𝑒𝑑, 𝑡𝑎𝑔, 𝑠𝑡𝑒𝑝, 𝑜𝑟𝑖𝑔). Third,
instead of setting the 𝑢-component of configurations to zero at each step of the 𝑚𝑎𝑡𝑐ℎ() algorithm in section 2, we need to set
it to the 𝑢-component of the parent configurations, so that paths are accumulated rather than reset at each step. Fourth, we no
longer need to call 𝑢𝑝𝑑𝑎𝑡𝑒 𝑟𝑒𝑠𝑢𝑙𝑡() at each step — this can be done once at the end of match. The only part of the algorithm
that requires non-trivial change is the 𝑐𝑜𝑚𝑝𝑎𝑟𝑒() function, which is defined below via co-recursive functions 𝑐𝑜𝑚𝑝𝑎𝑟𝑒1() that
populates the 𝑍-cache and 𝑐𝑜𝑚𝑝𝑎𝑟𝑒2() that recurses one frame back, or stops if the fork has been reached.
1
2
3
4
5
6
7
8

𝑐𝑜𝑚𝑝𝑎𝑟𝑒(𝑐1 , 𝑐2 , 𝑈 , 𝑍)
( , , 𝑢1 , ) = 𝑐1
( , , 𝑢2 , ) = 𝑐2
return 𝑐𝑜𝑚𝑝𝑎𝑟𝑒1(𝑢1 , 𝑢2 , 𝑈 , 𝑍)
𝑐𝑜𝑚𝑝𝑎𝑟𝑒1(𝑢1 , 𝑢2 , 𝑈 , 𝑍)
if 𝑍(𝑢1 , 𝑢2 ) = ⊥ then
𝑍(𝑢1 , 𝑢2 ) = 𝑐𝑜𝑚𝑝𝑎𝑟𝑒2(𝑢1 , 𝑢2 , 𝑈 , 𝑍)
return 𝑍(𝑢1 , 𝑢2 )

9
10
11
12
13
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
31
32

𝑐𝑜𝑚𝑝𝑎𝑟𝑒2(𝑢1 , 𝑢2 , 𝑈 , 𝑍)
if 𝑢1 = 𝑢2 then return (∞, ∞, 0)
ℎ1 = ℎ2 = ∞
𝑞1 = 𝑈 [𝑢1 ].𝑜𝑟𝑖𝑔, 𝑞2 = 𝑈 [𝑢2 ].𝑜𝑟𝑖𝑔
𝑘1 = 𝑈 [𝑢1 ].𝑠𝑡𝑒𝑝, 𝑘2 = 𝑈 [𝑢2 ].𝑠𝑡𝑒𝑝, 𝑘 = 𝑚𝑎𝑥(𝑘1 , 𝑘2 )
𝑓𝑜𝑟𝑘 = (𝑞1 = 𝑞2 ) ∧ (𝑘1 = 𝑘2 )

𝑢′1 = 𝑢′2 = ⊥
while 𝑢1 ≠ 𝑢2 ∧ (𝑘1 ≥ 𝑘 ∨ 𝑘2 ≥ 𝑘) do
if 𝑘1 ≥ 𝑘 ∧ (𝑢1 > 𝑢2 ∨ 𝑘2 < 𝑘) then
ℎ1 = 𝑚𝑖𝑛(ℎ1 , ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢1 ].𝑡𝑎𝑔))
𝑢′1 = 𝑢1 , 𝑢1 = 𝑈 [𝑢1 ].𝑝𝑟𝑒𝑑, 𝑘1 = 𝑈 [𝑢1 ].𝑠𝑡𝑒𝑝
else
ℎ2 = 𝑚𝑖𝑛(ℎ2 , ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢2 ].𝑡𝑎𝑔))
𝑢′2 = 𝑢2 , 𝑢2 = 𝑈 [𝑢2 ].𝑝𝑟𝑒𝑑, 𝑘2 = 𝑈 [𝑢2 ].𝑠𝑡𝑒𝑝
if ¬𝑓𝑜𝑟𝑘 then
(ℎ′1 , ℎ′2 , 𝑙) = 𝑐𝑜𝑚𝑝𝑎𝑟𝑒1(𝑢1 , 𝑢2 , 𝑈 , 𝑍)
ℎ1 = 𝑚𝑖𝑛(ℎ1 , ℎ′1 ), ℎ2 = 𝑚𝑖𝑛(ℎ2 , ℎ′2 )
else if 𝑢1 ≠ ⊥ then
ℎ = ℎ𝑒𝑖𝑔ℎ𝑡(𝑈 [𝑢1 ].𝑡𝑎𝑔)
ℎ1 = 𝑚𝑖𝑛(ℎ1 , ℎ), ℎ2 = 𝑚𝑖𝑛(ℎ2 , ℎ)
if ℎ1 > ℎ2 then 𝑙 = −1
else if ℎ1 < ℎ2 then 𝑙 = 1
else if 𝑓𝑜𝑟𝑘 then 𝑙 = 𝑙𝑒𝑓𝑡𝑝𝑟𝑒𝑐(𝑢′1 , 𝑢′2 , 𝑈 )
return (ℎ1 , ℎ2 , 𝑙)

ALGORITHM 6: Lazy disambiguation procedures (the 𝑍-cache is modified in-place).

<!-- page 17 -->

17

The problem with this approach is that we need to keep full-length history of each active path: at the point of ambiguity we may
need to look an arbitrary number of steps back in order to find the fork of ambiguous paths. This may be acceptable for small
inputs (and memory footprint may even be smaller due to the reduction of precedence matrices), but it is infeasible for long or
streaming inputs. A possible solution may be a hybrid approach that uses lazy disambiguation, but every 𝑘 steps fully calculates
the precedence matrices and “forgets” the path prefixes. Another possible solution is to keep both algorithms and switch between
them depending on the length of input.

9

TNFA CONSTRUCTION

TNFA construction is given by the function 𝑡𝑛𝑓𝑎() that accepts IRE 𝑒 and state 𝑞𝑓 and returns TNFA for 𝑒 with the final state 𝑞𝑓
(algorithm 7). This precise construction is not necessary for the algorithms to work, but it has a number of important properties.
• Non-essential 𝜖-transitions are removed, as they make closure algorithms GOR1 and GTOP slower.

• Bounded repetition 𝑒𝑛,𝑚 is unrolled in a way that duplicates 𝑒 exactly 𝑚 times and factors out common path prefixes:
subautomaton for (𝑘 + 1)-th iteration is only reachable from subautomaton for 𝑘-th iteration. For example, 𝑎2,5 is unrolled
as 𝑎𝑎(𝜖|𝑎(𝜖|𝑎(𝜖|𝑎))), not as 𝑎𝑎(𝜖|𝑎|𝑎𝑎|𝑎𝑎𝑎). This ensures that the tag tree build by 𝜖-closure is a prefix tree.

• Priorities are assigned so as to make it more likely that depth-first traversal of the 𝜖-closure finds short paths before long
paths. This is an optimization that makes GOR1 much faster in specific cases with many ambiguous paths that are longestequivalent and must be compared by the leftmost criterion. An example of such case is (((𝜖)0,𝑘 )0,𝑘 )0,𝑘 ) for some large 𝑘.
Because GOR1 has a depth-first component, it is sensitive to the order of transitions in TNFA. If it finds the shortest path
early, then all other paths are just canceled at the first join point with the shortest path (because there is no improvement
and further scanning is pointless). In the opposite case GOR1 finds long paths before short ones, and whenever it finds an
improved (shorter) path, it has to schedule configurations for re-scan on the next pass. This causes GOR1 to make more
passes and scan more configurations on each pass, which makes it significantly slower. Arguably this bias is a weakness
of GOR1 — GTOP is more robust in this respect.

• When adding negative tags, we add a single transition for the topmost closing tag (it corresponds to the nil-parenthesis,
which has the height of a closing parenthesis). Then we map this tag to the full range of its nested tags, including itself
and the pair opening tag. An alternative approach is to add all nested negative tags as TNFA transitions and get rid of the
mapping, but this may result in significant increase of TNFA size and major slowdown (we observed 2x slowdown on
large tests with hundreds of submatch groups).
• Although for simplicity we use sets to represent nested tags and submatch groups for each tag (the 𝑀 component of a
TNFA), these sets contain consecutive numbers and in practice can be represented more efficiently as ranges.

• Passing the final state 𝑞𝑓 in 𝑡𝑛𝑓𝑎() function allows to link subautomata in a simple and efficient way. It allows to avoid
tracking and patching of subautomaton transitions that go to the final state (when this final state needs to be changed).

See figure4 for an example of TNFA.

10

COMPLEXITY ANALYSIS

Our algorithm consists of three steps: conversion of RE to IRE, construction of TNFA from IRE and simulation of TNFA on
the input string. We discuss time and space complexity of each step in terms of the following parameters: 𝑛 — the length of
input, 𝑚 — the size of RE with counted repetition subexpressions expanded (each subexpression duplicated the number of
times equal to the repetition counter), and 𝑡 — the number of capturing groups and subexpressions that contain them.
The first step, conversion of RE to IRE, is given by the functions 𝑚𝑎𝑟𝑘() and 𝑒𝑛𝑢𝑚() from section 3. For each sub-RE, 𝑚𝑎𝑟𝑘()
constructs a corresponding sub-IRE, and 𝑒𝑛𝑢𝑚() performs a linear visit of the IRE (which doesn’t change its structure), therefore IRE size is 𝑂(𝑚). Each subexpression is processed twice (once by 𝑚𝑎𝑟𝑘() and once by 𝑒𝑛𝑢𝑚()) and processing takes 𝑂(1)
time, therefore total time is 𝑂(𝑚).

<!-- page 18 -->

18
1

𝑡𝑛𝑓𝑎(𝑒, 𝑞𝑓 )

2 𝑖 = 𝑖𝑠𝑢𝑏(𝑒), 𝐽 = 𝑒𝑠𝑢𝑏(𝑒) // submatch indices
3 if 𝑖 = 0 then
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
31
32

33

34
35

if 𝑒 = 𝐸𝑝𝑠(0, ∅) then
return (Σ, {𝑞𝑓 }, ∅, ∅, 𝑞𝑓 , 𝑞𝑓 )

else if 𝑒 = 𝑆𝑦𝑚(0, ∅, 𝑎) ∣𝑎∈Σ then
return (Σ, {𝑞0 , 𝑞𝑓 }, ∅, {(𝑞0 , 𝑎, 𝜖, 𝑞𝑓 )}, 𝑞0 , 𝑞𝑓 )

else if 𝑒 = 𝐶𝑎𝑡(0, ∅, 𝑒1 , 𝑒2 ) then
(Σ, 𝑄2 , 𝑀2 , Δ2 , 𝑞2 , 𝑞𝑓 ) = 𝑡𝑛𝑓𝑎(𝑒2 , 𝑞𝑓 )
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞1 , 𝑞2 ) = 𝑡𝑛𝑓𝑎(𝑒1 , 𝑞2 )
return (Σ, 𝑄1 ∪ 𝑄2 , 𝑀1 ⊔ 𝑀2 , Δ1 ∪ Δ2 , 𝑞1 , 𝑞𝑓 )

else if 𝑒 = 𝐴𝑙𝑡(0, ∅, 𝑒1 , 𝑒2 ) then
(Σ, 𝑄2 , 𝑀2 , Δ2 , 𝑞2 , 𝑞𝑓 ) = 𝑡𝑛𝑓𝑎(𝑒2 , 𝑞𝑓 )
(Σ, 𝑄′2 , 𝑀2 , Δ′2 , 𝑞2′ , 𝑞𝑓 ) = 𝑛𝑡𝑎𝑔(𝑀2 , 𝑞𝑓 )
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞1 , 𝑞2′ ) = 𝑡𝑛𝑓𝑎(𝑒2 , 𝑞2′ )
(Σ, 𝑄′1 , 𝑀1 , Δ′1 , 𝑞1′ , 𝑞2 ) = 𝑛𝑡𝑎𝑔(𝑀1 , 𝑞2 )
𝑄 = 𝑄1 ∪ 𝑄′1 ∪ 𝑄2 ∪ 𝑄′2 ∪ {𝑞0 }
Δ = Δ1 ∪ Δ′1 ∪ Δ2 ∪ Δ′2 ∪ {(𝑞0 , 1, 𝜖, 𝑞1 ), (𝑞0 , 2, 𝜖, 𝑞1′ )}
return (Σ, 𝑄, 𝑀1 ⊔ 𝑀2 , Δ, 𝑞0 , 𝑞𝑓 )

else if 𝑒 = 𝑅𝑒𝑝(0, ∅, 𝑒1 , 𝑛, 𝑚) ∣1<𝑛≤𝑚≤∞ then
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞2 , 𝑞𝑓 ) = 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0,∅,𝑒1 ,𝑛−1,𝑚−1),𝑞𝑓 )
(Σ, 𝑄2 , 𝑀2 , Δ2 , 𝑞1 , 𝑞2 ) = 𝑡𝑛𝑓𝑎(𝑒1 , 𝑞2 )
return (Σ, 𝑄1 ∪ 𝑄2 , 𝑀1 ⊔ 𝑀2 , Δ1 ∪ Δ2 , 𝑞1 , 𝑞𝑓 )

else if 𝑒 = 𝑅𝑒𝑝(0, ∅, 𝑒1 , 0, 𝑚) then
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞1 , 𝑞𝑓 ) = 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, 𝑚), 𝑞𝑓 )
(Σ, 𝑄′1 , 𝑀1 , Δ′1 , 𝑞1′ , 𝑞𝑓 ) = 𝑛𝑡𝑎𝑔(𝑀1 , 𝑞𝑓 )
𝑄 = 𝑄1 ∪ 𝑄′1 ∪ {𝑞0 }
Δ = Δ1 ∪ Δ′1 ∪ {(𝑞0 , 1, 𝜖, 𝑞1 ), (𝑞0 , 2, 𝜖, 𝑞1′ )}
return (Σ, 𝑄, 𝑀1 , Δ, 𝑞0 , 𝑞𝑓 )
else if 𝑒 = 𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, ∞) then
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞0 , 𝑞1 ) = 𝑡𝑛𝑓𝑎(𝑒1 , )
𝑄 = 𝑄1 ∪ {𝑞𝑓 }
Δ = Δ1 ∪ {(𝑞1 , 1, 𝜖, 𝑞0 ), (𝑞1 , 2, 𝜖, 𝑞𝑓 )}
return (Σ, 𝑄, 𝑀1 , Δ, 𝑞0 , 𝑞𝑓 )

else if 𝑒 = 𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, 1) then return 𝑡𝑛𝑓𝑎(𝑒1 , 𝑞𝑓 )

else if 𝑒 = 𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, 𝑚) ∣1<𝑚<∞ then
37
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞1 , 𝑞𝑓 ) = 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, 𝑚−1), 𝑞𝑓 )
38
(Σ, 𝑄2 , 𝑀2 , Δ2 , 𝑞0 , 𝑞2 ) = 𝑡𝑛𝑓𝑎(𝑒1 , 𝑞1 )
39
Δ = Δ1 ∪ Δ2 ∪ {(𝑞1 , 1, 𝜖, 𝑞𝑓 ), (𝑞1 , 2, 𝜖, 𝑞2 )}
40
return (Σ, 𝑄1 ∪ 𝑄2 , 𝑀1 ⊔ 𝑀2 , Δ, 𝑞0 , 𝑞𝑓 )
41 else
42
𝑒′ = 𝑓𝑖𝑥(0, ∅, 𝑒) // reset submatch indices
43
(Σ, 𝑄1 , 𝑀1 , Δ1 , 𝑞1 , 𝑞2 ) = 𝑡𝑛𝑓𝑎(𝑒′ , 𝑞2 )
44
𝑄 = 𝑄1 ∪ {𝑞0 , 𝑞𝑓 }
45
𝑁 = 𝑑𝑜𝑚(𝑀
{ 1 ) ⊔ {2𝑖−1,2𝑖} // nested tags
𝑀1 (𝑚) if 𝑚 ∈ 𝑑𝑜𝑚(𝑀1 )
46
𝑀(𝑚) =
(𝐽 , 𝑁) if 𝑚 ∈ {2𝑖−1, 2𝑖}
47
Δ = Δ1 ∪ {(𝑞0 , 1, 2𝑖−1, 𝑞1 ), (𝑞2 , 1, 2𝑖, 𝑞𝑓 )}
48
return (Σ, 𝑄, 𝑀, Δ, 𝑞0 , 𝑞𝑓 )
36

49
50
51

𝑛𝑡𝑎𝑔(𝑀, 𝑞𝑓 )

// smallest closing tag represents all tags in 𝑀
𝑛 = 𝑚𝑖𝑛{𝑚 ∈ 𝑑𝑜𝑚(𝑀) ∣ 𝑚 ≡ 0 𝑚𝑜𝑑 2}
return (Σ, {𝑞0 , 𝑞𝑓 }, 𝑀, {(𝑞0 , 1, −𝑛, 𝑞𝑓 )}, 𝑞0 , 𝑞𝑓 )

𝑞𝑓

(a) 𝑡𝑛𝑓𝑎(𝐸𝑝𝑠(0, ∅), 𝑞𝑓 )
𝑎∕𝜖

𝑞0

𝑞𝑓

(b) 𝑡𝑛𝑓𝑎(𝑆𝑦𝑚(0, ∅, 𝑎), 𝑞𝑓 )
𝑡𝑛𝑓𝑎(𝑒1 , 𝑞2 )

𝑞1

𝑡𝑛𝑓𝑎(𝑒2 , 𝑞𝑓 )

𝑞2

𝑞𝑓

(c) 𝑡𝑛𝑓𝑎(𝐶𝑎𝑡(0, ∅, 𝑒1 , 𝑒2 ), 𝑞𝑓 )
1∕𝜖

𝑡𝑛𝑓𝑎(𝑒1 , 𝑞2′ )

𝑞1

𝑞2′

𝑞0
𝑞1′

2∕𝜖

𝑛𝑡𝑎𝑔(𝑀1 , 𝑞2 )

𝑞2

𝑛𝑡𝑎𝑔

(𝑀 ,

2 𝑞𝑓 )

𝑒 , 𝑞𝑓
𝑡𝑛𝑓𝑎( 2

)

𝑞𝑓

(d) 𝑡𝑛𝑓𝑎(𝐴𝑙𝑡(0, ∅, 𝑒1 , 𝑒2 ), 𝑞𝑓 )
𝑡𝑛𝑓𝑎(𝑒1 , 𝑞2 )

𝑞1

𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 𝑛−1, 𝑚−1), 𝑞𝑓 )

𝑞2

𝑞𝑓

(e) 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 𝑛, 𝑚), 𝑞𝑓 ) ∣1 < 𝑛 ≤ 𝑚 ≤ ∞
1∕𝜖

𝑞1

𝑡𝑛𝑓𝑎(𝑅
𝑒𝑝(0, ∅
,𝑒

1 , 1, 𝑚),𝑞
𝑓)

𝑞0

, 𝑞𝑓 )
𝑛𝑡𝑎𝑔 (𝑀 1

𝑞1′

2∕𝜖

𝑞𝑓

(f) 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 0, 𝑚), 𝑞𝑓 )
1∕𝜖

𝑞0

𝑡𝑛𝑓𝑎(𝑒1 , 𝑞1 )

2∕𝜖

𝑞1

𝑞𝑓

(g) 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, ∞), 𝑞𝑓 )
1∕𝜖
𝑞0

𝑡𝑛𝑓𝑎(𝑒1 , 𝑞1 )

𝑞1

2∕𝜖

𝑞2

𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, 𝑚−1), 𝑞𝑓 )

(h) 𝑡𝑛𝑓𝑎(𝑅𝑒𝑝(0, ∅, 𝑒1 , 1, 𝑚), 𝑞𝑓 ) ∣1 < 𝑚 < ∞
𝑞0

1∕2𝑖−1

𝑞1

(i) 𝑡𝑛𝑓𝑎(𝑒, 𝑞𝑓 ) ∣𝑖 ≠ 0

𝑞0

1∕ − 𝑛

𝑡𝑛𝑓𝑎(𝑒′ , 𝑞2 )

𝑞2

1∕2𝑖

𝑞𝑓

(j) 𝑛𝑡𝑎𝑔(𝑀, 𝑞𝑓 )

ALGORITHM 7: TNFA construction. Notation 𝑀 = 𝑀1 ⊔ 𝑀2 for functions with disjoint domains means that
𝑀(𝑚) = 𝑀𝑖 (𝑚) ∀𝑚 ∈ 𝑑𝑜𝑚(𝑀𝑖 ), 1 ≤ 𝑖 ≤ 2. Functions 𝑓𝑖𝑥, 𝑖𝑠𝑢𝑏 and 𝑒𝑠𝑢𝑏 are defined on page 6.

𝑞𝑓

𝑞𝑓

<!-- page 19 -->

19

1∕𝜖
1∕𝜖
1∕3
1∕1
1

2

3

2∕𝜖

5

𝑎∕𝜖

1∕𝜖
6

2∕𝜖

4

2∕𝜖
7

1∕𝜖

1∕7
1∕4
8

9

1∕𝜖

10
2∕𝜖

1∕5

(

11

12
2∕𝜖

1∕𝜖

𝑎∕𝜖
13

1∕8
14

25

1∕−6

)
𝑀(1) = 𝑀(2) = {1}, {1, 2, 3, 4, 5, 6, 7, 8}
(
)
𝑀(3) = 𝑀(4) = {2, 3}, {3, 4}

1∕7
2∕𝜖

15

16

17
2∕𝜖

1∕𝜖

𝑎∕𝜖
18

1∕8
19

1∕𝜖

1∕7
2∕𝜖

21

20

1∕𝜖

22
2∕𝜖

𝑎∕𝜖
23

1∕8
24

1∕6
26

(

𝑀(5) = 𝑀(6) = ∅, {5, 6, 7, 8}
(
)
𝑀(7) = 𝑀(8) = {4}, {7, 8}

)

1∕2
27

FIGURE 4 Example of TNFA for RE (((𝜖|𝑎0,∞ ))(𝑎|𝜖)0,3 ).
The second step, TNFA construction, is given by the 𝑡𝑛𝑓𝑎() function (algorithm 7). At this step counted repetition is unrolled,
so TNFA may be much larger than IRE. For each subexpressions of the form 𝑅𝑒𝑝(𝑖, 𝐽 , 𝑒, 𝑘, 𝑙) automaton for 𝑒 is duplicated
exactly 𝑙 times if 𝑙 ≠ ∞, or 𝑚𝑎𝑥(1, 𝑘) times otherwise (at each step of recursion the counter is decremented and one copy
of automaton is constructed). Since repetition counters are constants, unrolling increases the size of sub-TNFA by a constant
factor. Other 𝑡𝑛𝑓𝑎() clauses are in one-to-one correspondence with sub-IRE. Each clause adds only a constant number of states
and transitions (including calls to 𝑛𝑡𝑎𝑔𝑠() and excluding recursive calls). Therefore TNFA contains 𝑂(𝑚) states and transitions.
The size of mapping 𝑀 is 𝑂(𝑡), which is 𝑂(𝑚). Therefore total TNFA size is 𝑂(𝑚). Time complexity is 𝑂(𝑚), because each
clause takes 𝑂(1) time (excluding recursive calls), and total 𝑂(𝑚) clauses are executed.
The third step, TNFA simulation, is given by algorithm 1. Initialization at lines 2-4 takes 𝑂(1) time. Main loop at lines 5-11 is
executed at most 𝑛 times. The size of closure is bounded by TNFA size, which is 𝑂(𝑚) (typically closure is much smaller than
that). Each iteration of the loop includes the following steps. At line 6 the call to 𝑐𝑙𝑜𝑠𝑢𝑟𝑒() takes 𝑂(𝑚2 𝑡) time if GOR1 from
section 4 is used, because GOR1 makes 𝑂(𝑚2 ) scans (closure contains 𝑂(𝑚) states and 𝑂(𝑚) transitions), and at each scan we
may need to compare the tag sequences using 𝑐𝑜𝑚𝑝𝑎𝑟𝑒() from section 7, which takes 𝑂(𝑡) time (there is 𝑂(𝑡) unique tags and we
consider paths with at most one 𝜖-loop, so the number of occurrences of each tag is bounded by the repetition counters, which
amounts to a constant factor). At line 7 the call to 𝑢𝑝𝑑𝑎𝑡𝑒 𝑟𝑒𝑠𝑢𝑙𝑡() takes 𝑂(𝑚 𝑡) time, because closure size is 𝑂(𝑚), and the
length of the tag sequence is 𝑂(𝑡) as argued above. At line 8 the call to 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒() takes 𝑂(𝑚2 ) time if the advanced
algorithm from section 7 is used. At line 9 scanning the closure for reachable states takes 𝑂(𝑚) time, because closure size is
𝑂(𝑚). The sum of the above steps is 𝑂(𝑚2 𝑡), so the total time of loop at lines 5-11 is 𝑂(𝑛 𝑚2 𝑡). The final call to 𝑐𝑙𝑜𝑠𝑢𝑟𝑒() at
line 12 takes 𝑂(𝑚2 𝑡), and 𝑓𝑖𝑛𝑎𝑙 𝑟𝑒𝑠𝑢𝑙𝑡() at line 14 takes 𝑂(𝑚 𝑡). The total time taken by 𝑚𝑎𝑡𝑐ℎ() is therefore 𝑂(𝑛 𝑚2 𝑡).
Space complexity of 𝑚𝑎𝑡𝑐ℎ() is as follows. The size of the closure is 𝑂(𝑚). Precedence matrices 𝐻 and 𝑃 take 𝑂(𝑚2 ) space.
Match results take 𝑂(𝑚 𝑡) space in case of POSIX-style offsets, because the number of parallel results is bounded by the closure
size, and each result takes 𝑂(𝑡) space. In case of parse trees, match results take 𝑂(𝑚 𝑛) space, because each result accumulates
all loop iterations. The size of the 𝑈 -tree is 𝑂(𝑚2 ) because GOR1 makes 𝑂(𝑚2 ) scans and thus adds no more than 𝑂(𝑚2 ) tags
in the tree. The total space taken by 𝑚𝑎𝑡𝑐ℎ() is therefore 𝑂(𝑚2 ) for POSIX-style offsets and 𝑂(𝑚(𝑚 + 𝑛)) for parse trees.
For leftmost-greedy submatch extraction, space complexity is 𝑂(𝑚 𝑡) and time complexity is 𝑂(𝑛 𝑚 𝑡), because the 𝜖-closure is
constructed with a DFS in 𝑂(𝑚) time and has 𝑂(𝑚) size, and each closure configuration contains 𝑂(𝑡) submatch results that
need to be updated.

11

BENCHMARKS

In this section 𝑛, 𝑚, 𝑡 have the same meaning as in section 10. We used two groups of benchmarks:

1. Real-world benchmarks (A1 – A12). These REs are designed to evaluate the algorithms in a natural setting. They describe
practical concepts, such as URI or HTTP message headers, and range from very large (containing thousands of characters
and hundreds of capturing groups) to small (containing under a hundred of characters and about five capturing groups).
The input consists of short strings conforming to the RE grammar.

<!-- page 20 -->

20

2. Artificial benchmarks (B1 – B12 and C1 – C12). These REs are designed to be highly ambiguous and reveal worst-case
behavior and pathological cases for different algorithms. All REs in this group have an alphabet consisting of a single
symbol a, and the input is a long string of as (16K). There are two subgroups in this group:
(a) REs with alternative. Benchmarks B1 – B6 contain REs of the form (𝑎𝑘1 |𝑎𝑘2 |𝑎𝑘3 )0,∞ , where 𝑘𝑖 are prime numbers.
Benchmarks B7 – B12 contain their variations with more capturing groups: (((𝑎)𝑘1 )|((𝑎)𝑘2 )|((𝑎)𝑘3 ))0,∞ . These REs
show the difficulty of POSIX longest-match semantics, because the choice of alternatives varies indefinitely with
the number of as in the input string. For example, consider RE (𝑎2 |𝑎3 |𝑎5 )0,∞ : given an input string a...a, submatch
on the last iteration depends on the length of input: it equals aaaaa for a 5𝑛-character string, aa for strings of length
5𝑛 − 3 and 5𝑛 − 1, and aaa for strings of length 5𝑛 − 2 and 5𝑛 + 1 (𝑛 ∈ ℕ). Variation continues indefinitely with a
period of five characters. The variation period is longer for higher counter values.
(b) REs with concatenation. These benchmarks contain REs of the form (𝑒0,𝑘 )0,∞ , where 𝑒 is (𝑎|𝜖) for C1 – C3, (𝑎0,∞ )
for C4 – C6, 𝑎 for C7 – C9 and (𝑎) for C10 – C11. These REs have large 𝜖-closure sizes.
We benchmarked the following algorithms:

• LG: the basic leftmost-greedy implementation. It updates offsets at each step of TNFA simulation and runs in 𝑂(𝑛 𝑚 𝑡)
time and 𝑂(𝑚 𝑡) space. This algorithm has no overhead on disambiguation and serves as a baseline for other algorithms.

• Lazy-LG: a “lazy” version of LG. It accumulates tag history of all simulation steps, updates offsets at the end of match
rather than at each step, and runs in 𝑂(𝑛 𝑚) time and 𝑂(𝑛 𝑚) space. This algorithm reduces the overhead on incremental
offset updates, but it requires memory proportional to the size of input.

• Re2-LG: a leftmost-greedy implementation by the Google RE2 library 20 . This algorithm is used to relate our implementations to the real world: both RE2C and RE2 are written in C++, so they are comparable. Internally RE2 uses a bunch
of different algorithms: simple cases are optimized, and the results vary considerably from one algorithm to another.

• OS: the main Okui-Suzuki implementation described in this paper. It uses GOR1 and the advanced 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒()
algorithm. Disambiguation and offset updates are performed eagerly at each step of TNFA simulation and the algorithm
runs in 𝑂(𝑛 𝑚2 𝑡) time and 𝑂(𝑚2 ) space.
• GTOP-OS: the same as OS, but uses GTOP instead of GOR1.

• Simple-OS: the same as OS, but uses the simple 𝑢𝑝𝑑𝑎𝑡𝑒 𝑝𝑟𝑒𝑐𝑒𝑑𝑒𝑛𝑐𝑒() algorithm.

• Lazy-OS: the “lazy” version of OS described in section 8. It accumulates tag history of all simulation steps and disambiguates on demand, memoizing the results in cache. Offsets are updated at the end of match. The algorithm has the
same 𝑂(𝑛 𝑚2 𝑡) time complexity as OS (because the 𝜖-closure construction takes 𝑂(𝑚2 𝑡) time), and its space complexity
is 𝑂(𝑛 𝑚2 ) due to the need to store 𝑂(𝑚2 ) tag path tree for each of the 𝑛 steps. This algorithm reduces the overhead on
incremental offset updates and precedence matrix computation, but it requires memory proportional to the size of input.
• Kuklewicz: the implementation of Kuklewicz algorithm 5 as described in 11 . The main difference with OS is that the
algorithm splits the common tag history into individual tag histories and considers them separately. The algorithm runs
in 𝑂(𝑛 𝑚 𝑡 (𝑚 + 𝑡 log(𝑚)) time and 𝑂(𝑚 𝑡) space.

• Backward: the backward-matching algorithm proposed by Cox 6 . We had to modify it substantially in order to support
bounded repetition and fix bugs (described in the introduction). We also use a fast forward pass to find the matching
string prefix before running the main backward pass. Naturally, these modifications incur some overhead. The resulting
algorithm is not fully correct, but the errors are rare compared to the original algorithm (about 2.5% of our tests), and the
test cases that trigger errors are more complex.

All algorithms except re2-LG are implemented in the open-source lexer generator RE2C 19 in the form of a regular expression
library libre2c, which provides the usual POSIX API of regcomp, regexec and regfree. Individual algorithms are selected
by passing non-standard flags to regcomp: for example, there is a flag that enables leftmost-greedy semantics instead of POSIX
semantics, a flag that enables GTOP instead of GOR1, a flag that enables “lazy” mode, etc. Most of the flags can be combined.
We selected a limited number of combinations for the benchmarks in order to show the impact of each individual feature. Correctness of our implementation has been tested on a subset of Glenn Fowler test suite 18 (we removed tests for backreferences
and start/end anchors), extended by Kuklewicz and further extended by ourselves to some 500 tests.
Benchmark results are shown on figures 5 , 6 and 7 .

<!-- page 21 -->

21

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz

1
0.77
3.25
4.01
3.62
3.35
1.95

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz

1
0.79
1.96
3.13
2.9
2.94
1.64

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz

1
0.51
1.1
3.87
4.14
4.24
1.21

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.77
0.46
3.28
2.88
2.69
1.61
4.77

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz

1
0.74
0.41
3.85
3.4
3.07
1.74

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.82
0.38
3.33
2.87
2.93
1.8
5.89

A1 RFC-7230 compliant HTTP parser (6204 symbols, 185 captures)

68

11.85

1
0.74
0.17
2.86
2.52
2.42
1.4

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.86
0.17
3.28
3.05
2.96
2.42
5.35

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.76
0.1
3.17
3.42
3.46
2.8

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.82
0.28
3.97
3.3
3.14
1.83
4.67

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.81
0.45
4.01
3.4
3.41
2.01
6.13

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.79
0.28
3.13
2.77
2.78
1.72
3.65
5.32

17.45

A2 simple HTTP parser (573 symbols, 33 captures)

33.9

A4 simple URI parser (234 symbols, 14 captures)

A3 RFC-3986 compliant URI parser (3149 symbols, 93 captures)

48.79

8.79

16.45

A6 simple IPv6 parser (93 symbols, 7 captures)

A5 RFC-7230 compliant IPv6 parser (2343 symbols, 62 captures)

24.63

A7 IPv4 parser (235 symbols, 6 captures)

19.75

A8 simple IPv4 parser (57 symbols, 5 captures)

11.12

32.1

A10 simple date parser (81 symbols, 7 captures)

A9 date parser (224 symbols, 14 captures)

A11 Gentoo package atom (117 symbols, 14 captures)

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz

69.6

A12 simple package atom (45 symbols, 5 captures)

FIGURE 5 Real-world benchmarks (A1 – A12).

<!-- page 22 -->

22

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.57
0.18
2.87
2.94
3.75
2.47
2.04
2.25

B1 (𝑎2 |𝑎3 |𝑎5 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

B7 (((𝑎)2 )|((𝑎)3 )|((𝑎)5 ))0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.47
0.92
5.61
5.73
13.97
1.72
2.54
2.1

B2 (𝑎7 |𝑎13 |𝑎19 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.44
0.89
12.7
13.43
1.3
4.73
2.03

1
0.42
0.92

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

41.28

25.52
25.81

1.32
5.13
2.03

1
0.46
0.93

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

85.78

49.63
49.05

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

170.54

1.27
5.54
2.19

1
0.47
1.1
1.16
5.44
2.22

B6 (𝑎199 |𝑎239 |𝑎271 )0,∞

1
0.78
0.75
14.55
15.72
10.83
3.81

40.17
42.22

B10 (((𝑎)67 )|((𝑎)83 )|((𝑎)103 ))0,∞

B5 (𝑎127 |𝑎151 |𝑎179 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.73
0.72
7.73
8.64
19.31
22.5
9.7
3.26

B9 (((𝑎)29 )|((𝑎)41 )|((𝑎)53 ))0,∞

B4 (𝑎67 |𝑎83 |𝑎103 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.85
0.75
4.21
4.66
8.16
11.98
9.6
3.57

B8 (((𝑎)7 )|((𝑎)13 )|((𝑎)19 ))0,∞

B3 (𝑎29 |𝑎41 |𝑎53 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.88
0.71
2.68
2.92
3.1
4.66
3.42
3.77

1
0.8
0.84

25.79
27.08

11.03
3.74

70.86
60.26

B11 (((𝑎)127 )|((𝑎)151 )|((𝑎)179 ))0,∞

84.31
78.26

260.5

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.78
0.84

11.32
4.36

40.13
42.13
72.39

B12 (((𝑎)199 )|((𝑎)239 )|((𝑎)271 ))0,∞

FIGURE 6 Artificial benchmarks (B1 — B12).
Highly ambiguous REs with alternative on 16K string of as.

108.65

<!-- page 23 -->

23

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.97
0.33
1.95
2.09
1.49
1
1.24
2.43

C1 ((𝑎|𝜖)0,1 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward
C2 ((𝑎|𝜖)
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward
C3 ((𝑎|𝜖)
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

C7 (𝑎0,1 )0,∞

1
0.11
0.09
3.5
5.39
5.43
0.37

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

108.91
27.46

0,256 0,∞

1
0.06
0.05
4.38
6.32
4.82
0.21

227.48
32.78

0,512 0,∞

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

111.69
58.81

0.39

C6 ((𝑎0,∞ )0,512 )0,∞

114.81
116.42
136.61

20.02
3.52
3.34

1
0.94
0.33
2.15
2.06
1.53
1.03
1.27
2.3

1
0.87
0.61
16.39
17.69
36.86
18.12
2.03
2.68

C11 ((𝑎)0,256 )0,∞

1
0.07
0.04
7.6
7.74
10.21

1
0.72
0.66

C10 ((𝑎)0,1 )0,∞

1
0.14
0.08
6.57
6.78
10.93

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.95
0.05
1.67
1.72
1.52
2.18
1.33
2.77

C5 ((𝑎0,∞ )0,256 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

15.91
2.86
3.33

41.88
43.34
59.69

C9 (𝑎0,512 )0,∞

)

0.67

1
0.68
0.65

C8 (𝑎0,256 )0,∞

)

C4 ((𝑎0,∞ )0,1 )0,∞
LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.86
0.36
1.98
1.86
1.47
0.9
1.23
2.6

237.48
53

LG
lazy-LG
re2-LG
OS
GTOP-OS
simple-OS
lazy-OS
Kuklewicz
backward

1
0.86
0.61

48.13
50.45
19.71

2.17
2.87

C12 ((𝑎)0,512 )0,∞

FIGURE 7 Artificial benchmarks (C1 — C12).
Highly ambiguous REs with concatenation on 16K string of as.

81.47

<!-- page 24 -->

24

In order to present benchmark results in a meaningful way, we show the time of each algorithm relative to LG. This allows us
to show the net overhead on POSIX disambiguation. We measured the speed of each algorithm in characters per second and
divided it by the speed of LG (therefore LG time is always equal to 1). We ran benchmarks on Intel(R) Core(TM) i7-8750H
CPU @ 2.20GHz machine with 12 cores (we ran in single-threaded mode), 32 GB of RAM, 32K L1d cache, 32K L1i cache,
256K L2 cache, 9216K L3 cache, running Gentoo Linux. We used RE2 version 2019-12-01 and RE2C from Git 19 at commit
662b45ef468713f59e667200aacd922f29580c4a.
Memory consumption.
We used Massif 22 to estimate memory usage. The algorithms can be partitioned in two groups:
those that execute in bounded memory (depending on the size of RE, but not on the length of input), and those that require
memory proportional to the length of input. The first group contains LG, re2-LG, OS, GTOP-OS, simple-OS, Kuklewicz and
backward: these algorithms stay under 50M for all benchmarks, and usually the memory is pre-allocated at the beginning (e.g.
for precedence matrices). The second group contains lazy-LG and lazy-OS: memory consumption of these algorithms grows
with the length of input and reaches peak values for C6 (over 700M for lazy-LG and over 4G for lazy-OS). Both algorithms keep
closure trees for all steps in memory, but the size of the trees is linear in the size of RE for lazy-LG (which uses a simple DFS),
and worst-case quadratic for lazy-OS (which uses GOR1). Lazy-OS also allocates a large disambiguation cache. Our cache
implementation is a red-black tree from the standard C++ library, which incurs a lot of small heap allocations and deallocations
and is not optimal. However, it suffices to show the general problem.
Run-time performance.
On real-world RE all algorithms except Kuklewicz and backward perform quite well. Lazy-OS is
the fastest among POSIX implementations, and only ~2x slower than LG. Other OS algorithms (OS, GTOP-OS and simple-OS)
are 3-4x slower than LG, with OS being generally a bit slower due to a more complex implementation. Kuklewicz algorithm is
noticeably slower on large RE, and the backward algorithm is so slow that it timed out on some of the benchmarks. On artificial
benchmarks the results are quite different. Of all POSIX implementations, only the backward algorithm performs reasonably
well. Other algorithms show performance degradation on some of the benchmarks, and simple-OS is particularly slow. In general,
we can observe the following tendencies:

• OS degrades with increased closure size, as shown by B1 – B6, B7 – B12, C7 – C9 and C10 – C12. This is caused by an
𝑂(𝑚2 ) precedence matrix update algorithm.

• Simple-OS degrades with increased closure size, and much faster than OS, as shown by C1 – C3 and C4 – C6. (it is ~50x
slower than OS on C3). This is caused by an 𝑂(𝑚2 𝑡) precedence matrix update algorithm.

• Kuklewicz algorithm degrades with increased closure size, although not as fast as OS (as shown by B7 – B12, C1 – C3
and C4 – C6) and with increased number of tags (compare B1 – B6 with fewer captures to B7 – B12 with more captures),
in particular orbit tags (compare C1 – C3 with an innermost alternative to C4 – C6 with an innermost repetition). On
REs with many capturing groups Kuklewicz algorithm can get very slow (A1, A5). This is caused by an 𝑂(𝑚 log(𝑚) 𝑡2 )
precedence matrix update algorithm.
• Backward algorithm degrades with increased number of tags (as shown by A1 – A12, and also observable on B7 – B12
compared to B1 – B6). This is caused by the necessity to copy offset arrays between closure configurations: instead of
using a prefix tree to store tag histories during closure construction, the algorithm stores an array of offsets (a pair of
offsets per tag) in each configuration. GOR1 is asymptotically faster than DFS with backtracking used by the original
implementation 6 , but it increases the amount of copying (DFS order of graph traversal allows to update and restore a
single offset array in-place). Even with DFS copying is a bottleneck for REs with many capturing groups.

• Lazy-OS degrades with increased input length, increased closure size and ambiguity in RE (it performs well on B1 – B6,
but much worse on B7 – B12 which differ only in capturing parentheses). This is caused by the need to keep all closure
trees in memory and allocate a large disambiguation cache. In highly ambiguous cases the lazy algorithm does all the work
of a non-lazy algorithm, but with the additional overhead on cache lookups/insertions and accumulation of data from the
previous steps.

• GOR1, despite having 𝑂(𝑚2 𝑡) worst-case complexity, in practice performs closer to 𝑂(𝑚 𝑡) and adds less overhead than
updating the precedence matrix (compare B1 – B6 for OS and lazy-OS, which also uses GOR1, but no precedence matrix).

• GOR1 and GTOP performance is similar. GTOP is usually slightly faster on real-world RE (probably because the closure
graphs are simple and GOR1 code is more complex). We failed to find a pathological case for GTOP; this may mean that
we didn’t look hard enough, or that TNFA closures have some special properties that make GTOP efficient.

<!-- page 25 -->

25

• LG and lazy-LG performance is close to re2-LG, faster on large RE, but slower on small RE. This is explained by the fact
that RE2 uses an optimized bitmap-based implementation for small RE.

12

CONCLUSIONS AND FUTURE WORK

As the benchmarks show, none of the POSIX matching algorithms we considered is perfect, but for each algorithm there are
use cases where it performs better than other algorithms. On the whole though, we can make the following observations. LazyOS is generally the fastest algorithm, but it cannot be used for long inputs due to its unbounded memory consumption. OS is
generally the most robust algorithm: it has the best worst-case time and space complexity, it is reasonably fast, and it can be
used for long inputs (as it executes in bounded memory). Therefore a practical implementation would use a use a combination
of lazy-OS and OS.
It is still an open question to us whether it is possible to combine the elegance of the derivative-based approach to POSIX
disambiguation with the efficiency of NFA-based algorithms. In the derivative-based approach, match results are constructed
in such order that the longest-leftmost result is always first. Perhaps a similar idea can be applied to NFA-based algorithms
in order to reduce the amount of disambiguation they perform. We experimented with recursive descent parsers based on the
ordering idea and constructed a prototype implementation (but it did not result in a faster algorithm).
It would be interesting to apply our approach to automata with counters instead of unrolling bounded repetition.

ACKNOWLEDGMENTS
I want to thank my husband and fellow programmer Sergei, my parents Vladimir and Elina, my teachers Tatyana Leonidovna
and Demian Vladimirovich, and the Belarusian State University. And cheers to the RE2C users!
Ulya Trofimovich

References
[1] Ken Thompson, Programming Techniques: Regular expression search algorithm, Communications of the ACM, vol. 11 (6), 1968.
[2] Alfred V. Aho, Monica S. Lam, Ravi Sethi, Jeffrey D. Ullman, Compilers: Principles, Techniques, & Tools (chapter 3.7.3: Efficiency of NFA Simulation),
2nd edition, Boston, MA, USA: Pearson Addison-Wesley, ISBN 0-321-48681-1, pp. 157-159.
[3] Satoshi Okui and Taro Suzuki, Disambiguation in Regular Expression Matching via Position Automata with Augmented Transitions, International
Conference on Implementation and Application of Automata, pp. 231-240, Springer, Berlin, Heidelberg, 2013.
[4] Ville Laurikari, Efficient submatch addressing for regular expressions, Helsinki University of Technology, 2001.
[5] Chris Kuklewicz, Regular expressions/bounded space proposal, http://wiki.haskell.org/index.php?title=Regular_expressions/Bounded_space_proposal&
oldid=11475, 2007.
[6] Russ Cox, backward POSIX matching algorithm (source code), https://swtch.com/~rsc/regexp/nfa-posix.y.txt, 2009.
[7] Martin Sulzmann, Kenny Zhuo Ming Lu, POSIX Regular Expression Parsing with Derivatives, International Symposium on Functional and Logic
Programming, pp. 203-220, Springer, Cham, 2014.
[8] Martin Sulzmann, Kenny Zhuo Ming Lu, Correct and Efficient POSIX Submatch Extraction with Regular Expression Derivatives, https://www.home.
hs-karlsruhe.de/~suma0002/publications/posix-derivatives.pdf, 2013.
[9] Angelo Borsotti1, Luca Breveglieri, Stefano Crespi Reghizzi, Angelo Morzenti, From Ambiguous Regular Expressions to Deterministic Parsing Automata,
International Conference on Implementation and Application of Automata. Springer, Cham, pp.35-48, 2015.
[10] Fahad Ausaf, Roy Dyckhoff, Christian Urban, POSIX Lexing with Derivatives of Regular Expressions, International Conference on Interactive Theorem
Proving. Springer, Cham, pp. 69-86, 2016.
[11] Ulya Trofimovich, Tagged Deterministic Finite Automata with Lookahead, arXiv:1907.08837 [cs.FL], 2017.
[12] Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, Introduction to algorithms, 1st edition, MIT Press and McGraw-Hill, ISBN 0-262-03141-8.

<!-- page 26 -->

26

[13] Mehryar Mohri, Semiring frameworks and algorithms for shortest-distance problems, Journal of Automata, Languages and Combinatorics 7 (2002) 3,
321–350, Otto-von-Guericke-Universitat, Magdeburg, 2002.
[14] Aaron Karper, Efficient regular expressions that produce parse trees, Master’s thesis, University of Bern, 2014.
[15] Andrew V. Goldberg, Tomasz Radzik, A heuristic improvement of the Bellman-Ford algorithm, Elsevier, Applied Mathematics Letters, vol. 6, no. 3, pp.
3-6, 1993.
[16] Boris V. Cherkassky, Andrew V. Goldberg, Tomasz Radzik, Shortest paths algorithms: Theory and experimental evaluation, Springer, Mathematical
programming, vol. 73, no. 2, pp. 129-174, 1996.
[17] Boris V. Cherkassky, Loukas Georgiadis, Andrew V. Goldberg, Robert E. Tarjan, and Renato F. Werneck. Shortest Path Feasibility Algorithms: An
Experimental Evaluation, Journal of Experimental Algorithmics (JEA), 14, 7, 2009.
[18] Glenn Fowler, An Interpretation of the POSIX Regex Standard, https://archive.org/details/glenn_fowler_an_interpretation_of_the_posix_regex_standard,
2003.
[19] RE2C, lexer generator for C/C++. Website: http://re2c.org, source code: http://github.com/skvadrik/re2c.
[20] RE2, regular expression library. Source code: http://github.com/google/re2.
[21] The GNU C library, https://www.gnu.org/software/libc/.
[22] Massif, a heap profiler, http://www.valgrind.org/docs/manual/ms-manual.html.

<!-- page 27 -->

27

APPENDIX
Proof of theorem 1
Lemma 5 (Unique position mapping from all PTs to IRE). If 𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒) for some IRE 𝑒 and there is a common position
𝑥 ∈ 𝑃 𝑜𝑠(𝑡) ∩ 𝑃 𝑜𝑠(𝑠), then 𝑥 corresponds to the same position 𝑥′ ∈ 𝑃 𝑜𝑠(𝑒) in 𝑒 for both 𝑡 and 𝑠.
Proof. The proof is by induction on the length of 𝑥. Induction basis: 𝑥 = 𝑥′ = Λ (the roots of 𝑡 and 𝑠 correspond to the root
of 𝑒). Induction step: suppose that for any position 𝑥 of length |𝑥| < ℎ the lemma is true. We will show that if exists a 𝑘 ∈ ℕ
such that 𝑥.𝑘 ∈ 𝑃 𝑜𝑠(𝑡) ∩ 𝑃 𝑜𝑠(𝑠), then 𝑥.𝑘 corresponds to the same position 𝑥′ .𝑘′ in 𝑒 for both 𝑡 and 𝑠 (for some 𝑘′ ∈ ℕ). If 𝑒|𝑥′
is an elementary IRE of the form 𝐸𝑝𝑠(𝑖, 𝑗) or 𝑆𝑦𝑚(𝑖, 𝑗, 𝑎), or if at least one of 𝑡|𝑥 and 𝑠|𝑥 is ⊥, then 𝑘 doesn’t exist. Otherwise
𝑒|𝑥′ is a compound IRE and both 𝑡|𝑥 and 𝑠|𝑥 are not ⊥. If 𝑒|𝑥′ is an alternative 𝐴𝑙𝑡(𝑖, 𝑗, 𝑒1 , 𝑒2 ) or a concatenation 𝐶𝑎𝑡(𝑖, 𝑗, 𝑒1 , 𝑒2 ),
then both 𝑡|𝑥 and 𝑠|𝑥 have exactly two subtrees, and positions 𝑥.1 and 𝑥.2 in 𝑡 and 𝑠 correspond to positions 𝑥′ .1 and 𝑥′ .2 in 𝑒.
Otherwise, 𝑒|𝑥′ is a repetition 𝑅𝑒𝑝(𝑖, 𝑗, 𝑒1 , 𝑛, 𝑚) and for any 𝑘 ≥ 1 position 𝑥.𝑘 in 𝑡 and 𝑠 corresponds to position 𝑥′ .1 in 𝑒.
Theorem 1. P-order < is a strict total order on 𝑃 𝑇 (𝑒, 𝑤) for any IRE 𝑒 and string 𝑤.

Proof. We need to show that < is transitive and trichotomous.

(1) Transitivity: we need to show that ∀𝑡, 𝑠, 𝑟 ∈ 𝑃 𝑇 (𝑒, 𝑤) ∶ (𝑡 < 𝑠 ∧ 𝑠 < 𝑟) ⇐⇒ 𝑡 < 𝑟.
Let 𝑡 <𝑥 𝑠 and 𝑠 <𝑦 𝑟 for some positions 𝑥, 𝑦, and let 𝑧 = 𝑚𝑖𝑛(𝑥, 𝑦).

𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
First, we show that ‖𝑡‖𝑝𝑜𝑠
𝑧 > ‖𝑟‖𝑧 . If 𝑥 ≤ 𝑦, we have ‖𝑡‖𝑥 > ‖𝑠‖𝑥 (implied by 𝑡 <𝑥 𝑠) and ‖𝑠‖𝑥 ≥ ‖𝑟‖𝑥 (implied
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
by 𝑠 <𝑦 𝑟 ∧ 𝑥 ≤ 𝑦), therefore ‖𝑡‖𝑥 > ‖𝑟‖𝑥 . Otherwise 𝑥 > 𝑦, we have ‖𝑡‖𝑦 > ‖𝑟‖𝑦 (implied by 𝑠 <𝑦 𝑟) and
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
‖𝑡‖𝑝𝑜𝑠
𝑦 = ‖𝑠‖𝑦 (implied by 𝑡 <𝑥 𝑠 ∧ 𝑦 < 𝑥), therefore ‖𝑡‖𝑦 > ‖𝑟‖𝑦 .

Second, we show that ∀𝑧′ < 𝑧 ∶ ‖𝑡‖𝑝𝑜𝑠
= ‖𝑟‖𝑝𝑜𝑠
. We have ‖𝑡‖𝑝𝑜𝑠
= ‖𝑠‖𝑝𝑜𝑠
(implied by 𝑡 <𝑥 𝑠∧𝑧′ < 𝑥) and ‖𝑠‖𝑝𝑜𝑠
= ‖𝑟‖𝑝𝑜𝑠
𝑧′
𝑧′
𝑧′
𝑧′
𝑧′
𝑧′
𝑝𝑜𝑠
𝑝𝑜𝑠
(implied by 𝑠 <𝑦 𝑟 ∧ 𝑧′ < 𝑦), therefore ‖𝑡‖𝑧′ = ‖𝑟‖𝑧′ .

(2) Trichotomy: we need to show that ∀𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) exactly one of 𝑡 < 𝑠, 𝑠 < 𝑡 or 𝑡 = 𝑠 holds. Consider the set of positions
𝑝𝑜𝑠
where norms of 𝑡 and 𝑠 disagree 𝑋 = {𝑥 ∈ 𝑃 𝑜𝑠(𝑡) ∪ 𝑃 𝑜𝑠(𝑠) ∶ ‖𝑡‖𝑝𝑜𝑠
𝑥 ≠ ‖𝑠‖𝑥 }.
(2.1) First case: 𝑋 ≠ ∅. We show that in this case exactly one of 𝑡 < 𝑠 or 𝑠 < 𝑡 is true (𝑡 ≠ 𝑠 is obvious).

First, we show that at least one of 𝑡 < 𝑠 or 𝑠 < 𝑡 is true. Let 𝑥 = 𝑚𝑖𝑛(𝑋); it is well-defined since 𝑋 is non𝑝𝑜𝑠
empty, finite and lexicographically ordered. For all 𝑦 < 𝑥 we have ‖𝑡‖𝑝𝑜𝑠
𝑦 = ‖𝑠‖𝑦 (by definition of 𝑥 and because
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
‖𝑡‖𝑦 = ∞ = ‖𝑠‖𝑦 if 𝑦 ∉ 𝑃 𝑜𝑠(𝑡) ∪ 𝑃 𝑜𝑠(𝑠)). Since ‖𝑡‖𝑥 ≠ ‖𝑠‖𝑥 , we have either 𝑡 <𝑥 𝑠 or 𝑡 <𝑥 𝑠.

Second, we show that at most one of 𝑡 < 𝑠 or 𝑠 < 𝑡 is true, i.e. < is asymmetric: ∀𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) ∶ 𝑡 < 𝑠 ⇐⇒ 𝑠 ≮ 𝑡.
Suppose, on the contrary, that 𝑡 <𝑥 𝑠 and 𝑠 <𝑦 𝑡 for some 𝑥, 𝑦. Without loss of generality let 𝑥 ≤ 𝑦. On one hand
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑝𝑜𝑠
𝑡 <𝑥 𝑠 implies ‖𝑡‖𝑝𝑜𝑠
𝑥 > ‖𝑠‖𝑥 . But on the other hand 𝑠 <𝑦 𝑡 ∧ 𝑥 ≤ 𝑦 implies ‖𝑡‖𝑥 ≤ ‖𝑠‖𝑥 . Contradiction.

(2.2) Second case: 𝑋 = ∅. We show that in this case 𝑡 = 𝑠.

We have 𝑃 𝑜𝑠(𝑡) = 𝑃 𝑜𝑠(𝑠) — otherwise there is a position with norm ∞ in only one of the trees. Therefore 𝑡 and 𝑠
have identical node structure. By lemma 5 any position in 𝑡 and 𝑠 corresponds to the same position in 𝑒. Since any
position in 𝑒 corresponds to a unique explicit submatch index, it must be that submatch indices of all nodes in 𝑡 and
𝑠 coincide. Consider some position 𝑥 ∈ 𝑃 𝑜𝑠(𝑡). If 𝑥 corresponds to an inner node, then both 𝑡|𝑥 and 𝑠|𝑥 are of the
form 𝑇 𝑖 (…). Otherwise, 𝑥 corresponds to a leaf node, which can be either ⊥ or 𝜖 or 𝛼. Since all three have different
𝑝𝑜𝑠
norms (−1, 0 and 1 respectively), and since ‖𝑡‖𝑝𝑜𝑠
𝑥 = ‖𝑠‖𝑥 , it must be that 𝑡|𝑥 and 𝑠|𝑥 are identical.

Proof of theorem 2
Lemma 6. If 𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, then 𝑡 ∼ 𝑠 ⇔ ∀𝑥 ∶ ‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
.
𝑥
𝑥

Proof. Forward implication: let 𝑡 ∼ 𝑠 and suppose, on the contrary, that ∃𝑥 = 𝑚𝑖𝑛{𝑦 ∣ ‖𝑡‖𝑠𝑢𝑏
≠ ‖𝑠‖𝑠𝑢𝑏
}, then either 𝑡 ≺𝑥 𝑠 (if
𝑦
𝑦
𝑠𝑢𝑏
𝑠𝑢𝑏
𝑠𝑢𝑏
𝑠𝑢𝑏
‖𝑡‖𝑥 > ‖𝑠‖𝑥 ) or 𝑠 ≺𝑥 𝑡 (if ‖𝑡‖𝑥 < ‖𝑠‖𝑥 ), both cases contradict 𝑡 ∼ 𝑠. Backward implication: ∀𝑥 ∶ ‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
implies
𝑥
𝑥
∄𝑥 ∶ 𝑡 ≺𝑥 𝑠 and ∄𝑦 ∶ 𝑠 ≺𝑦 𝑡, which implies 𝑡 ∼ 𝑠.

<!-- page 28 -->

28

Theorem 2. S-order ≺ is a strict weak order on 𝑃 𝑇 (𝑒, 𝑤) for any IRE 𝑒 and string 𝑤.
Proof. We need to show that ≺ is asymmetric and transitive, and incomparability relation ∼ is transitive.
(1) Asymmetry: we need to show that ∀𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) ∶ 𝑡 ≺ 𝑠 ⇐⇒ 𝑠 ⊀ 𝑡.

Suppose, on the contrary, that 𝑡 ≺𝑥 𝑠 and 𝑠 ≺𝑦 𝑡 for some 𝑥, 𝑦. Without loss of generality let 𝑥 ≤ 𝑦. On one hand 𝑡 ≺𝑥 𝑠
implies ‖𝑡‖𝑠𝑢𝑏
> ‖𝑠‖𝑠𝑢𝑏
. But on the other hand 𝑠 ≺𝑦 𝑡 ∧ 𝑥 ≤ 𝑦 implies ‖𝑡‖𝑠𝑢𝑏
≤ ‖𝑠‖𝑠𝑢𝑏
. Contradiction.
𝑥
𝑥
𝑥
𝑥
(2) Transitivity: we need to show that ∀𝑡, 𝑠, 𝑟 ∈ 𝑃 𝑇 (𝑒, 𝑤) ∶ (𝑡 ≺ 𝑠 ∧ 𝑠 ≺ 𝑟) ⇐⇒ 𝑡 ≺ 𝑟.
Let 𝑡 ≺𝑥 𝑠 and 𝑠 ≺𝑦 𝑟 for some positions 𝑥, 𝑦, and let 𝑧 = 𝑚𝑖𝑛(𝑥, 𝑦).

First, we show that ‖𝑡‖𝑠𝑢𝑏
> ‖𝑟‖𝑠𝑢𝑏
. If 𝑥 ≤ 𝑦, we have ‖𝑡‖𝑠𝑢𝑏
> ‖𝑠‖𝑠𝑢𝑏
(implied by 𝑡 ≺𝑥 𝑠) and ‖𝑠‖𝑠𝑢𝑏
≥ ‖𝑟‖𝑠𝑢𝑏
(implied
𝑧
𝑧
𝑥
𝑥
𝑥
𝑥
𝑠𝑢𝑏
𝑠𝑢𝑏
𝑠𝑢𝑏
𝑠𝑢𝑏
by 𝑠 ≺𝑦 𝑟 ∧ 𝑥 ≤ 𝑦), therefore ‖𝑡‖𝑥 > ‖𝑟‖𝑥 . Otherwise 𝑥 > 𝑦, we have ‖𝑡‖𝑦 > ‖𝑟‖𝑦 (implied by 𝑠 ≺𝑦 𝑟) and
‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
(implied by 𝑡 ≺𝑥 𝑠 ∧ 𝑦 < 𝑥), therefore ‖𝑡‖𝑠𝑢𝑏
> ‖𝑟‖𝑠𝑢𝑏
.
𝑦
𝑦
𝑦
𝑦

Second, we show that ∀𝑧′ < 𝑧 ∶ ‖𝑡‖𝑠𝑢𝑏
= ‖𝑟‖𝑠𝑢𝑏
. We have ‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
(implied by 𝑡 ≺𝑥 𝑠∧𝑧′ < 𝑥) and ‖𝑠‖𝑠𝑢𝑏
= ‖𝑟‖𝑠𝑢𝑏
𝑧′
𝑧′
𝑧′
𝑧′
𝑧′
𝑧′
𝑠𝑢𝑏
(implied by 𝑠 ≺𝑦 𝑟 ∧ 𝑧′ < 𝑦), therefore ‖𝑡‖𝑠𝑢𝑏
=
‖𝑟‖
.
′
′
𝑧
𝑧
(3) Transitivity of incomparability: we need to show that ∀𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤) ∶ (𝑡 ∼ 𝑠 ∧ 𝑠 ∼ 𝑟) ⇐⇒ 𝑡 ∼ 𝑟.

By forward implication of lemma 6 𝑡 ∼ 𝑠 ⇒ ∀𝑥 ∶ ‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
and 𝑠 ∼ 𝑟 ⇒ ∀𝑥 ∶ ‖𝑠‖𝑠𝑢𝑏
= ‖𝑟‖𝑠𝑢𝑏
, therefore
𝑥
𝑥
𝑥
𝑥
𝑠𝑢𝑏
𝑠𝑢𝑏
(𝑡 ∼ 𝑠 ∧ 𝑠 ∼ 𝑟) ⇒ ∀𝑥 ∶ ‖𝑡‖𝑥 = ‖𝑟‖𝑥 ⇒ 𝑡 ∼ 𝑢 by backward implication of lemma 6.

Proof of Theorem 3
Lemma 7 (Comparability of subtrees). For a given IRE 𝑒, string 𝑤 and position 𝑥, if 𝑡, 𝑠 ∈ 𝑃 𝑇 (𝑒, 𝑤), 𝑥 ∈ 𝑆𝑢𝑏(𝑡) ∪ 𝑆𝑢𝑏(𝑠) and
‖𝑡‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
∀𝑦 ≤ 𝑥, then ∃𝑒′ , 𝑤′ ∶ 𝑡|𝑥 , 𝑠|𝑥 ∈ 𝑃 𝑇 (𝑒′ , 𝑤′ ).
𝑦
𝑦

Proof. By induction on the length of 𝑥. Induction basis: 𝑥 = Λ, let 𝑒′ = 𝑒 and 𝑤′ = 𝑤. Induction step: suppose that the lemma
is true for any position 𝑥 of length |𝑥| < ℎ, we will show that it is true for any position 𝑥.𝑘 of length ℎ (𝑘 ∈ ℕ). Assume that
𝑥.𝑘 ∈ 𝑆𝑢𝑏(𝑡) ∩ 𝑆𝑢𝑏(𝑠) (otherwise either 𝑥.𝑘 ∉ 𝑆𝑢𝑏(𝑡) ∪ 𝑆𝑢𝑏(𝑠), or exactly one of ‖𝑡‖𝑠𝑢𝑏
, ‖𝑠‖𝑠𝑢𝑏
is ∞ — in both cases lemma
𝑥.𝑘
𝑥.𝑘
conditions are not satisfied). Then both 𝑡|𝑥 and 𝑠|𝑥 have at least one subtree: let 𝑡|𝑥 = 𝑇 (𝑡1 , … , 𝑡𝑛 ) and 𝑠|𝑥 = 𝑇 (𝑠1 , … , 𝑠𝑚 ),
where 𝑛, 𝑚 ≥ 𝑘. By induction hypothesis ∃𝑒′ , 𝑤′ ∶ 𝑡|𝑥 , 𝑠|𝑥 ∈ 𝑃 𝑇 (𝑒′ , 𝑤′ ). We have 𝑤′ = 𝑠𝑡𝑟(𝑡1 ) … 𝑠𝑡𝑟(𝑡𝑛 ) = 𝑠𝑡𝑟(𝑠1 ) … 𝑠𝑡𝑟(𝑠𝑚 ).
We show that 𝑠𝑡𝑟(𝑡𝑘 ) = 𝑠𝑡𝑟(𝑠𝑘 ). Consider positions 𝑥.𝑗 for 𝑗 ≤ 𝑘. By definition the set of submatch positions contains siblings,
𝑠𝑢𝑏
therefore 𝑥.𝑗 ∈ 𝑆𝑢𝑏(𝑡) ∩ 𝑆𝑢𝑏(𝑠). By lemma conditions ‖𝑡‖𝑠𝑢𝑏
𝑥.𝑗 = ‖𝑠‖𝑥.𝑗 (because 𝑥.𝑗 ≤ 𝑥.𝑘), therefore |𝑠𝑡𝑟(𝑡1 ) … 𝑠𝑡𝑟(𝑡𝑘−1 )|
∑
∑𝑘−1
𝑘−1
‖𝑠‖𝑠𝑢𝑏
= 𝑗=1 ‖𝑡‖𝑠𝑢𝑏
𝑗 = |𝑠𝑡𝑟(𝑠1 ) … 𝑠𝑡𝑟(𝑠𝑘−1 )| and |𝑠𝑡𝑟(𝑡𝑘 )| = |𝑠𝑡𝑟(𝑠𝑘 )|. Consequently, 𝑠𝑡𝑟(𝑡𝑘 ) and 𝑠𝑡𝑟(𝑠𝑘 ) start and end at
𝑗 =
𝑗=1
the same character in 𝑤′ and therefore are equal. Finally, have 𝑡|𝑥.𝑘 , 𝑠|𝑥.𝑘 ∈ 𝑃 𝑇 (𝑟|𝑥.𝑘 , 𝑠𝑡𝑟(𝑡𝑘 )) and induction step is complete.
Theorem 3. Let 𝑡𝑚𝑖𝑛 be the <-minimal tree in 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, and let 𝑇𝑚𝑖𝑛 be the class of ≺-minimal
trees in 𝑃 𝑇 (𝑒, 𝑤). Then 𝑡𝑚𝑖𝑛 ∈ 𝑇𝑚𝑖𝑛 .
Proof. Consider any 𝑡 ∈ 𝑇𝑚𝑖𝑛 . From 𝑡 we can construct another tree 𝑡′ in the following way. Consider all positions 𝑥 ∈ 𝑆𝑢𝑏(𝑡)
which are not proper prefixes of another position in 𝑆𝑢𝑏(𝑡). For each such position, 𝑡|𝑥 is itself a PT for some sub-IRE 𝑒′ and
substring 𝑤′ : 𝑡|𝑥 ∈ 𝑃 𝑇 (𝑒′ , 𝑤′ ). Let 𝑡′𝑚𝑖𝑛 be the <-minimal tree in 𝑃 𝑇 (𝑒′ , 𝑤′ ) and substitute 𝑡|𝑥 with 𝑡′𝑚𝑖𝑛 . Let 𝑡′ be the tree resulting
from all such substitutions (note that they are independent of the order in which we consider positions 𝑥). Since substitutions
preserve s-norm at submatch positions, we have 𝑡′ ∈ 𝑇𝑚𝑖𝑛 . We will show that 𝑡′ = 𝑡𝑚𝑖𝑛 .

Suppose, on the contrary, that 𝑡′ ≠ 𝑡𝑚𝑖𝑛 . Since 𝑡𝑚𝑖𝑛 is <-minimal, it must be that 𝑡𝑚𝑖𝑛 <𝑥 𝑡′ for some decision position 𝑥. By
𝑝𝑜𝑠
′ 𝑝𝑜𝑠
′ 𝑝𝑜𝑠
definition it means that ‖𝑡𝑚𝑖𝑛 ‖𝑝𝑜𝑠
𝑥 > ‖𝑡 ‖𝑥 and ‖𝑡𝑚𝑖𝑛 ‖𝑦 = ‖𝑡 ‖𝑦 ∀𝑦 < 𝑥. It follows that all positions 𝑦 ≤ 𝑥 that are in 𝑃 𝑜𝑠(𝑡𝑚𝑖𝑛 )
′
′ 𝑝𝑜𝑠
are also in 𝑃 𝑜𝑠(𝑡 ), otherwise we get a contradiction ‖𝑡 ‖𝑦 = ∞ > ‖𝑡𝑚𝑖𝑛 ‖𝑝𝑜𝑠
𝑦 . By lemma 5 position 𝑦 corresponds to the same
position in the IRE 𝑒 for both 𝑡𝑚𝑖𝑛 and 𝑡′ . Consequently, any 𝑦 ≤ 𝑥 is either a submatch position in both 𝑡𝑚𝑖𝑛 and 𝑡′ , or in neither
of them. Let 𝑥′ be the longest prefix of 𝑥 in 𝑆𝑢𝑏(𝑡′ ). Since 𝑥′ ≤ 𝑥, by the above reasoning 𝑥′ is also in 𝑆𝑢𝑏(𝑡𝑚𝑖𝑛 ). All prefixes of
a submatch position are also submatch positions, therefore for 𝑦 ≤ 𝑥′ s-norms coincide with p-norms: ‖𝑡𝑚𝑖𝑛 ‖𝑠𝑢𝑏
= ‖𝑡𝑚𝑖𝑛 ‖𝑝𝑜𝑠
𝑦 and
𝑦
′ 𝑠𝑢𝑏
′ 𝑝𝑜𝑠
′
′
′
′
‖𝑡 ‖𝑦 = ‖𝑡 ‖𝑦 . It follows that 𝑥 ≠ 𝑥, otherwise 𝑡𝑚𝑖𝑛 <𝑥 𝑡 implies 𝑡𝑚𝑖𝑛 ≺𝑥 𝑡 , which contradicts to 𝑡 ∈ 𝑇𝑚𝑖𝑛 . Therefore 𝑥′ < 𝑥,

<!-- page 29 -->

29

′ 𝑝𝑜𝑠
𝑠𝑢𝑏
′ 𝑠𝑢𝑏
′
and since ‖𝑡𝑚𝑖𝑛 ‖𝑝𝑜𝑠
𝑦 = ‖𝑡 ‖𝑦 ∀𝑦 < 𝑥 and s-norms coincide with p-norms, we have ‖𝑡𝑚𝑖𝑛 ‖𝑦 = ‖𝑡 ‖𝑦 ∀𝑦 ≤ 𝑥 . Then by lemma
′
′
′
′
′
′
′
7 subtrees 𝑡 |𝑥′ and 𝑡𝑚𝑖𝑛 |𝑥′ are comparable: ∃𝑒 , 𝑤 ∶ 𝑡 |𝑥′ , 𝑡𝑚𝑖𝑛 |𝑥′ ∈ 𝑃 𝑇 (𝑒 , 𝑤 ). By construction of 𝑡 , subtree 𝑡′ |𝑥′ is <-minimal
in 𝑃 𝑇 (𝑒′ , 𝑤′ ), but at the same time 𝑡𝑚𝑖𝑛 <𝑥′ .𝑥′′ 𝑡′ implies 𝑡𝑚𝑖𝑛 |𝑥′ <𝑥′′ 𝑡′ |𝑥′ . Contradiction.

Theorem 4. Let 𝑒, 𝑒′ be two REs, such that 𝑒′ is constructed from 𝑒 by adding a pair of parentheses around some subexpression,
′
and let 𝑒̄ = 𝐼𝑅𝐸(𝑒) and 𝑒̄′ = 𝐼𝑅𝐸(𝑒′ ). If 𝑇𝑚𝑖𝑛 , 𝑇𝑚𝑖𝑛
are the classes of ≺-minimal trees in 𝑃 𝑇 ( ̄𝑒, 𝑤) and 𝑃 𝑇 (𝑒̄′ , 𝑤) respectively
′
for some string 𝑤, then 𝑇𝑚𝑖𝑛 ⊆ 𝑇𝑚𝑖𝑛 .
Proof. First, note that adding parentheses in RE does not affect the set of IRE positions: we have 𝑃 𝑜𝑠( ̄𝑒) = 𝑃 𝑜𝑠(𝑒̄′ ). Consequently, p-order on 𝑃 𝑇 ( ̄𝑒, 𝑤) and 𝑃 𝑇 (𝑒̄′ , 𝑤) is identical, and the <-minimal tree 𝑡𝑚𝑖𝑛 is the same in both cases. By theorem 3 we
′
′
have 𝑡𝑚𝑖𝑛 ∈ 𝑇𝑚𝑖𝑛 and 𝑡𝑚𝑖𝑛 ∈ 𝑇𝑚𝑖𝑛
, therefore the classes of ≺-minimal trees have at least one common tree: 𝑡𝑚𝑖𝑛 ∈ 𝑇𝑚𝑖𝑛 ∩ 𝑇𝑚𝑖𝑛

Second, note that adding parentheses in RE may only increase the set of IRE submatch positions: we have 𝑆𝑢𝑏( ̄𝑒) ⊆ 𝑆𝑢𝑏(𝑒̄′ ).
Let 𝑆, 𝑆 ′ denote 𝑆𝑢𝑏(𝑡𝑚𝑖𝑛 ) for 𝑒̄ and 𝑒̄′ respectively, then we have 𝑆 ⊆ 𝑆 ′ .

′
Consider any tree 𝑡 ∈ 𝑇𝑚𝑖𝑛
. It is incomparable with 𝑡𝑚𝑖𝑛 in the s-order on 𝑃 𝑇 (𝑒̄′ , 𝑤), therefore by lemma 6 (forward implication) all s-norms at positions in 𝑆 ′ in 𝑡 and 𝑡𝑚𝑖𝑛 are equal: ‖𝑡‖𝑠𝑢𝑏
= ‖𝑡𝑚𝑖𝑛 ‖𝑠𝑢𝑏
∀𝑥 ∈ 𝑆 ′ . Since 𝑆 ⊆ 𝑆 ′ , it follows that
𝑥
𝑥
𝑠𝑢𝑏
𝑠𝑢𝑏
‖𝑡‖𝑥 = ‖𝑡𝑚𝑖𝑛 ‖𝑥 ∀𝑥 ∈ 𝑆. Therefore by lemma 6 (backward implication) 𝑡 is incomparable with 𝑡𝑚𝑖𝑛 in the s-order on 𝑃 𝑇 ( ̄𝑒, 𝑤),
′
′
and consequently 𝑡 ∈ 𝑇𝑚𝑖𝑛 . Since 𝑡 is an arbitrary tree in 𝑇𝑚𝑖𝑛
, it follows that 𝑇𝑚𝑖𝑛
⊆ 𝑇𝑚𝑖𝑛 .

Proof of Theorem 5
Lemma 8. Let 𝑠, 𝑡 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤. If 𝑠 ∼ 𝑡, then Φℎ (𝑠) = Φℎ (𝑡) ∀ℎ.

Proof. By induction on the height of 𝑒. Induction basis: for height 1 we have |𝑃 𝑇 (𝑒, 𝑤)| ≤ 1 ∀𝑤, therefore 𝑠 = 𝑡 and Φℎ (𝑠) =
Φℎ (𝑡). Induction step: height is greater than 1, therefore 𝑠 = 𝑇 𝑜 (𝑠1 , … , 𝑠𝑛 ) and 𝑡 = 𝑇 𝑜 (𝑡1 , … , 𝑡𝑚 ). If 𝑜 = 0, then Φℎ (𝑠) = 𝑠𝑡𝑟(𝑠) =
𝑤 = 𝑠𝑡𝑟(𝑡) = Φℎ (𝑡). Otherwise 𝑜 ≠ 0. By lemma 6 we have 𝑠 ∼ 𝑡 ⇒ ‖𝑠‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑥. This implies 𝑛 = 𝑚 (otherwise
𝑥
𝑥
the norm of subtree at position 𝑚𝑖𝑛(𝑛, 𝑚) + 1 is ∞ for only one of 𝑠, 𝑡). Therefore Φℎ (𝑠) = ⟨ℎ+1 Φℎ+1 (𝑠1 ), … , Φℎ+1 (𝑠𝑛 )⟩ℎ and
Φℎ (𝑡) = ⟨ℎ+1 Φℎ+1 (𝑡1 ), … , Φℎ+1 (𝑡𝑛 )⟩ℎ . It suffices to show that ∀𝑖 ≤ 𝑛 ∶ Φℎ+1 (𝑠𝑖 ) = Φℎ+1 (𝑡𝑖 ). We have ‖𝑠𝑖 ‖𝑠𝑢𝑏
= ‖𝑡𝑖 ‖𝑠𝑢𝑏
∀𝑥
𝑥
𝑥
𝑠𝑢𝑏
𝑠𝑢𝑏
′
′
′
′
(implied by ‖𝑠‖𝑥 = ‖𝑡‖𝑥 ∀𝑥), therefore by lemma 6 𝑠𝑖 ∼ 𝑡𝑖 , and by lemma 7 ∃𝑒 , 𝑤 ∶ 𝑠𝑖 , 𝑡𝑖 ∈ 𝑃 𝑇 (𝑒 , 𝑤 ), where the height
of 𝑒′ is less than the height of 𝑒. By induction hypothesis Φℎ+1 (𝑠𝑖 ) = Φℎ+1 (𝑡𝑖 ).
Lemma 9. Let 𝑠, 𝑡 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤. If 𝑠 ≺𝑥 𝑡 and |𝑥| = 1, then Φℎ (𝑠) < Φℎ (𝑡) ∀ℎ.
Proof. By lemma conditions |𝑥| = 1, therefore 𝑥 ∈ ℕ. At least one of 𝑠|𝑥 and 𝑡|𝑥 must exist (otherwise ‖𝑠‖𝑠𝑢𝑏
= ∞ = ‖𝑡‖𝑠𝑢𝑏
𝑥
𝑥
𝑜
which contradicts 𝑠 ≺𝑥 𝑡), therefore 𝑒 is a compound IRE and 𝑠, 𝑡 can be represented as 𝑠 = 𝑇 (𝑠1 , … , 𝑠𝑛 ) and 𝑡 = 𝑇 𝑜 (𝑡1 , … , 𝑡𝑚 )
where 𝑜 ≠ 0 because Λ is a prefix of decision position 𝑥. Let 𝑘 be the number of frames and let 𝑗 be the fork, then:
|
Φℎ (𝑠) = ⟨ℎ+1 Φℎ+1 (𝑠1 ) …Φℎ+1 (𝑠𝑛 )⟩ℎ = 𝛽0 𝑎1 … 𝑎𝑗 𝛽𝑗 | 𝛾𝑗 𝑎𝑗+1 … 𝑎𝑘 𝛾𝑘
||
Φℎ (𝑡) = ⟨ℎ+1 Φℎ+1 (𝑡1 ) …Φℎ+1 (𝑡𝑚 )⟩ℎ = 𝛽0 𝑎1 … 𝑎𝑗 𝛽𝑗 | 𝛿𝑗 𝑎𝑗+1 … 𝑎𝑘 𝛿𝑘
|
𝑠𝑢𝑏
Consider any 𝑖 < 𝑥 (𝑖 ∈ ℕ). By lemma conditions 𝑠 ≺𝑥 𝑡, therefore ‖𝑠‖𝑠𝑢𝑏
=
‖𝑡‖
∀𝑦 < 𝑥. In particular ‖𝑠𝑖 ‖𝑠𝑢𝑏
= ‖𝑡𝑖 ‖𝑠𝑢𝑏
∀𝑦,
𝑦
𝑦
𝑦
𝑦
(
)
′
′
therefore by lemma 6 𝑠𝑖 ∼ 𝑡𝑖 , therefore by lemma 8 Φℎ+1 (𝑠𝑖 ) = Φℎ+1 (𝑡𝑖 ). Let 𝑡𝑟𝑎𝑐𝑒𝑠(Φℎ (𝑠), Φℎ (𝑡)) = (ℎ0 , … , ℎ𝑘 ), (ℎ0 , … , ℎ𝑘 ) .

(1) Case ∞ = ‖𝑠‖𝑠𝑢𝑏
> ‖𝑡‖𝑠𝑢𝑏
. In this case 𝑠𝑥 does not exist and fork happens immediately after Φℎ+1 (𝑠𝑥−1 ), Φℎ+1 (𝑡𝑥−1 ):
𝑥
𝑥
|
Φℎ (𝑠) = ⟨ℎ+1 Φℎ+1 (𝑠1 ) …Φℎ+1 (𝑠𝑥−1 ) | ⟩ℎ
||
Φℎ (𝑡) = ⟨ℎ+1 Φℎ+1 (𝑡1 ) …Φℎ+1 (𝑡𝑥−1 ) | Φℎ+1 (𝑡𝑥 ) …Φℎ+1 (𝑡𝑚 )⟩ℎ
|
Fork frame is the last one, therefore both 𝛾𝑗 and 𝛿𝑗 contain the closing parenthesis ⟩ℎ and we have ℎ𝑗 = ℎ′𝑗 = ℎ. For all
𝑖 < 𝑗 we have ℎ𝑖 = ℎ′𝑖 = −1. Therefore ℎ𝑖 = ℎ′𝑖 ∀𝑖 and Φℎ (𝑠) ∼ Φℎ (𝑡). Since 𝑓 𝑖𝑟𝑠𝑡(𝛾𝑗 ) is ⟩ and 𝑓 𝑖𝑟𝑠𝑡(𝛿𝑗 ) is one of ⟨ and
⟨⟩, we have Φℎ (𝑠) ⊂ Φℎ (𝑡). Therefore Φℎ (𝑠) < Φℎ (𝑡).

<!-- page 30 -->

30

(2) Case ∞ > ‖𝑠‖𝑠𝑢𝑏
> ‖𝑡‖𝑠𝑢𝑏
= −1. In this case both 𝑠𝑥 and 𝑡𝑥 exist, 𝑠𝑥 is not ⊥ and 𝑡𝑥 is ⊥, and fork happens immediately
𝑥
𝑥
after Φℎ+1 (𝑠𝑥−1 ), Φℎ+1 (𝑡𝑥−1 ):
|
Φℎ (𝑠) = ⟨ℎ+1 Φℎ+1 (𝑠1 ) …Φℎ+1 (𝑠𝑥−1 ) | ⟨ℎ+2 𝛽 ′ ⟩ℎ+1 Φℎ+1 (𝑠𝑥+1 ) …Φℎ+1 (𝑠𝑛 )⟩ℎ
||
Φℎ (𝑡) = ⟨ℎ+1 Φℎ+1 (𝑡1 ) …Φℎ+1 (𝑡𝑥−1 ) | ⟨⟩ℎ+1
Φℎ+1 (𝑡𝑥+1 ) …Φℎ+1 (𝑡𝑚 )⟩ℎ
|
(2.1) If the fork frame is the last one, then both 𝛾𝑗 and 𝛿𝑗 contain the closing parenthesis ⟩ℎ and we have ℎ𝑗 = ℎ′𝑗 = ℎ.
(2.2) Otherwise the fork frame is not the last one. We have 𝑚𝑖𝑛ℎ(𝛾𝑗 ), 𝑚𝑖𝑛ℎ(𝛿𝑗 ) ≥ ℎ + 1 and 𝑙𝑎𝑠𝑡ℎ(𝛽𝑗 ) = ℎ + 1 (the last
parenthesis in 𝛽𝑗 is either ⟨ℎ+1 if 𝑥 = 1 and 𝑠𝑥−1 does not exist, or else one of ⟩ℎ+1 and ⟨⟩ℎ+1 ), therefore ℎ𝑗 = ℎ′𝑗 = ℎ+1.
For subsequent frames 𝑖 such that 𝑗 < 𝑖 < 𝑘 we have ℎ𝑖 = ℎ′𝑖 = ℎ + 1 (on one hand ℎ𝑖 , ℎ′𝑖 ≤ ℎ + 1 because
ℎ𝑗 = ℎ′𝑗 = ℎ + 1, but on the other hand 𝑚𝑖𝑛ℎ(𝛾𝑖 ), 𝑚𝑖𝑛ℎ(𝛿𝑖 ) ≥ ℎ + 1). For the last pair of frames we have ℎ𝑘 = ℎ′𝑘 = ℎ
(they both contain the closing parenthesis ⟩ℎ ).
In both cases ℎ𝑖 = ℎ′𝑖 ∀𝑖 ≥ 𝑗. Since ℎ𝑖 = ℎ′𝑖 = −1 ∀𝑖 < 𝑗, we have ℎ𝑖 = ℎ′𝑖 ∀𝑖 and therefore Φℎ (𝑠) ∼ Φℎ (𝑡). Since
𝑓 𝑖𝑟𝑠𝑡(𝛾𝑗 ) = ⟨< ⟨⟩ = 𝑓 𝑖𝑟𝑠𝑡(𝛿𝑗 ) we have Φℎ (𝑠) ⊂ Φℎ (𝑡). Therefore Φℎ (𝑠) < Φℎ (𝑡).
(3) Case ∞ > ‖𝑠‖𝑠𝑢𝑏
> ‖𝑡‖𝑠𝑢𝑏
≥ 0. In this case both 𝑠𝑥 and 𝑡𝑥 exist and none of them is ⊥, and fork happens somewhere after
𝑥
𝑥
the opening parenthesis ⟨ℎ+2 and before the closing parenthesis ⟩ℎ+1 in Φℎ (𝑠𝑥 ), Φℎ (𝑡𝑥 ):
|
Φℎ (𝑠) = ⟨ℎ+1 Φℎ+1 (𝑠1 ) …Φℎ+1 (𝑠𝑥−1 ) ⟨ℎ+2 𝛽 ′ | 𝛾 ′ ⟩ℎ+1 Φℎ+1 (𝑠𝑥+1 ) …Φℎ+1 (𝑠𝑛 )⟩ℎ
||
Φℎ (𝑡) = ⟨ℎ+1 Φℎ+1 (𝑡1 ) …Φℎ+1 (𝑡𝑥−1 ) ⟨ℎ+2 𝛽 ′ | 𝛿 ′ ⟩ℎ+1 Φℎ+1 (𝑡𝑥+1 ) …Φℎ+1 (𝑡𝑚 )⟩ℎ
|
𝑠𝑢𝑏
From ‖𝑠‖𝑠𝑢𝑏
>
‖𝑡‖
≥
0
it
follows
that
𝑠
contains
more
alphabet
symbols than 𝑡𝑥 . Consequently Φℎ+1 (𝑠𝑥 ) contains more
𝑥
𝑥
𝑥
alphabet symbols, and thus spans more frames than Φℎ+1 (𝑡𝑥 ). Let 𝑙 be the index of the frame 𝛿𝑙 that contains the closing
parenthesis ⟩ℎ+1 of Φℎ+1 (𝑡𝑥 ). By the above reasoning Φℎ+1 (𝑠𝑥 ) does not end in frame 𝛾𝑙 , therefore 𝛾𝑙 does not contain the
closing parenthesis ⟩ℎ+1 and we have 𝑚𝑖𝑛ℎ(𝛾𝑙 ) ≥ ℎ + 2 and 𝑚𝑖𝑛ℎ(𝛿𝑙 ) = ℎ + 1. Furthermore, note that 𝑚𝑖𝑛ℎ(𝛽 ′ ), 𝑚𝑖𝑛ℎ(𝛾 ′ ),
𝑚𝑖𝑛ℎ(𝛿 ′ ) ≥ ℎ + 2, therefore 𝑙𝑎𝑠𝑡ℎ(𝛽𝑗 ) ≥ ℎ + 2 (including the case when 𝛽 ′ is empty), and for all frames 𝑖 such that
𝑗 ≤ 𝑖 < 𝑙 (if any) we have ℎ𝑖 , ℎ′𝑖 ≥ ℎ + 2. Consequently, for 𝑙-th frame we have ℎ𝑙 ≥ ℎ + 2 and ℎ′𝑙 = ℎ + 1, thus ℎ𝑙 > ℎ′𝑙 .
For subsequent frames 𝑖 such that 𝑙 < 𝑖 < 𝑘 we have 𝑚𝑖𝑛ℎ(𝛾𝑖 ), 𝑚𝑖𝑛ℎ(𝛿𝑖 ) ≥ ℎ + 1, therefore ℎ𝑖 ≥ ℎ + 1 and ℎ′𝑖 = ℎ + 1,
thus ℎ𝑖 ≥ ℎ′𝑖 . For the last pair of frames we have ℎ𝑘 = ℎ′𝑘 = ℎ, as they both contain the closing parenthesis ⟩ℎ . Therefore
Φℎ (𝑠) ⊏ Φℎ (𝑡), which implies Φℎ (𝑠) < Φℎ (𝑡).
Lemma 10. Let 𝑠, 𝑡 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤. If 𝑠 ≺𝑥 𝑡, then Φℎ (𝑠) < Φℎ (𝑡) ∀ℎ.
Proof. The proof is by induction on the length of 𝑥. Induction basis for |𝑥| = 1 is given by lemma 9. Induction step: suppose
that the lemma is correct for all 𝑥 of length |𝑥| < ℎ and let |𝑥| = ℎ (ℎ ≥ 2). Let 𝑥 = 𝑥′ .𝑥′′ where 𝑥′ ∈ ℕ. At least one of 𝑠|𝑥 and
𝑡|𝑥 must exist (otherwise ‖𝑠‖𝑠𝑢𝑏
= ∞ = ‖𝑡‖𝑠𝑢𝑏
which contradicts 𝑠 ≺𝑥 𝑡), therefore both 𝑒 and 𝑒|𝑥′ are compound IREs and 𝑠, 𝑡
𝑥
𝑥
′
′
𝑜
can be represented as 𝑠 = 𝑇 (𝑠1 , … , 𝑠𝑛 ) and 𝑡 = 𝑇 𝑜 (𝑡1 , … , 𝑡𝑚 ) where 𝑠′ = 𝑠𝑥′ = 𝑇 𝑜 (𝑠′1 , … , 𝑠′𝑛′ ) and 𝑡′ = 𝑡𝑥′ = 𝑇 𝑜 (𝑡′1 , … , 𝑡′𝑚′ )
and both 𝑜, 𝑜′ ≠ 0 (because Λ and 𝑥′ are prefixes of decision position 𝑥). Therefore Φℎ (𝑠), Φℎ (𝑡) can be represented as follows:
Φℎ+1 (𝑠′ )

⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
Φℎ (𝑠) = ⟨ℎ+1 Φℎ+1 (𝑠1 ) … Φℎ+1 (𝑠𝑥′ −1 ) ⟨ℎ+2 Φℎ+2 (𝑠′1 ) … Φℎ+2 (𝑠′𝑛′ )⟩ℎ+1 Φℎ+1 (𝑠𝑥′ +1 )Φℎ+1 (𝑠𝑛 )⟩ℎ
Φℎ (𝑡) = ⟨ℎ+1 Φℎ+1 (𝑡1 ) … Φℎ+1 (𝑡𝑥′ −1 ) ⟨ℎ+2 Φℎ+2 (𝑡′1 ) … Φℎ+2 (𝑡′𝑚′ )⟩ℎ+1 Φℎ+1 (𝑡𝑥′ +1 )Φℎ+1 (𝑡𝑚 )⟩ℎ
⏟⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏟⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏟
Φℎ+1 (𝑡′ )

Consider any 𝑖 < 𝑥′ . By lemma conditions 𝑠 ≺𝑥 𝑡, therefore ‖𝑠‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑦 < 𝑥, and in particular ‖𝑠𝑖 ‖𝑠𝑢𝑏
= ‖𝑡𝑖 ‖𝑠𝑢𝑏
∀𝑦,
𝑦
𝑦
𝑦
𝑦
′
𝑠𝑢𝑏
𝑠𝑢𝑏
therefore by lemma 6 𝑠𝑖 ∼ 𝑡𝑖 , therefore by lemma 8 Φℎ+1 (𝑠𝑖 ) = Φℎ+1 (𝑡𝑖 ). Since 𝑥 < 𝑥 we have ‖𝑠‖𝑦 = ‖𝑡‖𝑦 ∀𝑦 ≤ 𝑥′ and by
lemma 7 ∃𝑒′ , 𝑤′ ∶ 𝑠′ , 𝑡′ ∈ 𝑃 𝑇 (𝑒′ , 𝑤′ ). Since ‖𝑠′ ‖𝑠𝑢𝑏
= ‖𝑠‖𝑠𝑢𝑏
∀𝑦 and ‖𝑡′ ‖𝑠𝑢𝑏
= ‖𝑡‖𝑠𝑢𝑏
∀𝑦, we have 𝑠′ ≺𝑥′′ 𝑡′ . Since |𝑥′′ | < |𝑥|
𝑦
𝑦
𝑥′ .𝑦
𝑥′ .𝑦

<!-- page 31 -->

31

by induction hypothesis we have Φℎ+1 (𝑠′ ) < Φℎ+1 (𝑡′ ). If 𝑗 is the fork and 𝑓 ≤ 𝑗 ≤ 𝑘, then Φℎ (𝑠), Φℎ (𝑡) can be represented as:
Φℎ+1 (𝑠′ )

⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞
|
1
Φℎ (𝑠) = 𝛽0 𝑎1 … 𝑎𝑓 𝛽𝑓 𝛽𝑓2 𝑎𝑓 +1 … 𝑎𝑗 𝛽𝑗 | 𝛾𝑗 𝑎𝑗+1 … 𝑎𝑘 𝛾𝑘1 𝛾𝑘2 𝑎𝑘+1 … 𝑎𝑙 𝛾𝑙
|
|
Φℎ (𝑡) = 𝛽0 𝑎1 … 𝑎𝑓 𝛽𝑓1 𝛽𝑓2 𝑎𝑓 +1 … 𝑎𝑗 𝛽𝑗 | 𝛿𝑗 𝑎𝑗+1 … 𝑎𝑘 𝛿𝑘1 𝛿𝑘2 𝑎𝑘+1 … 𝑎𝑙 𝛿𝑙
|
⏟⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏟⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏞⏟
(

Φℎ+1 (𝑡′ )

)
(
)
Let 𝑡𝑟𝑎𝑐𝑒𝑠(Φℎ (𝑠), Φℎ (𝑡)) = (ℎ0 , … , ℎ𝑙 ), (ℎ′0 , … , ℎ′𝑙 ) and 𝑡𝑟𝑎𝑐𝑒𝑠(Φℎ+1 (𝑠′ ), Φℎ+1 (𝑡′ )) = (𝜎ℎ , … , 𝜎𝑘 ), (𝜎ℎ′ , … , 𝜎𝑘′ ) . We show
that for frames 𝑖 such that 𝑗 ≤ 𝑖 < 𝑘 we have ℎ𝑖 = 𝜎𝑖 ∧ ℎ′𝑖 = 𝜎𝑖′ and for subsequent frames 𝑘 ≤ 𝑖 ≤ 𝑙 we have ℎ𝑖 = ℎ′𝑖 .

(1) Case 𝑖 = 𝑗 < 𝑘 ≤ 𝑙 (the fork frame). Since we have shown that Φℎ+1 (𝑠𝑖 ) = Φℎ+1 (𝑡𝑖 ) ∀𝑖 < 𝑥′ , and since Φℎ+1 (𝑠′ ) and
Φℎ+1 (𝑡′ ) have nonempty common prefix ⟨ℎ+2 , it follows that 𝑙𝑎𝑠𝑡ℎ(Φℎ (𝑠) ⊓ Φℎ (𝑡)) = 𝑙𝑎𝑠𝑡ℎ(Φℎ+1 (𝑠′ ) ⊓ Φℎ+1 (𝑡′ )). From
𝑗 < 𝑘 it follows that 𝛾𝑗 and 𝛿𝑗 end before 𝑎𝑘 and are not changed by appending 𝛾𝑘2 and 𝛿𝑘2 . Therefore ℎ𝑗 = 𝜎𝑗 ∧ ℎ′𝑗 = 𝜎𝑗′ .
(2) Case 𝑗 < 𝑖 < 𝑘 ≤ 𝑙. The computation of ℎ𝑖 , ℎ′𝑖 depends only on ℎ𝑗 , ℎ′𝑗 , or which we have shown ℎ𝑗 = 𝜎𝑗 ∧ ℎ′𝑗 = 𝜎𝑗′ in case
(1), and on Φℎ+1 (𝑠′ ), Φℎ+1 (𝑡′ ), which are not changed by appending 𝛾𝑘2 and 𝛿𝑘2 since 𝑖 < 𝑘. Therefore ℎ𝑖 = 𝜎𝑖 ∧ ℎ′𝑖 = 𝜎𝑖′ .
(3) Case 𝑗 ≤ 𝑖 = 𝑘 < 𝑙. We have 𝑚𝑖𝑛ℎ(𝛾𝑘1 ) = 𝑚𝑖𝑛ℎ(𝛿𝑘1 ) = ℎ + 1 and 𝑚𝑖𝑛ℎ(𝛾𝑘2 ) = 𝑚𝑖𝑛ℎ(𝛿𝑘2 ) ≥ ℎ + 1. None of the preceding
frames after the fork contain parentheses with height less than ℎ + 1, therefore ℎ𝑘 = ℎ′𝑘 = ℎ + 1.
(4) Case 𝑗 ≤ 𝑘 < 𝑖 < 𝑙. We have ℎ𝑖 = ℎ′𝑖 = ℎ + 1, because ℎ𝑘 = ℎ′𝑘 = ℎ + 1 and 𝑚𝑖𝑛ℎ(𝛾𝑖 ), 𝑚𝑖𝑛ℎ(𝛿𝑖 ) ≥ ℎ + 1.
(5) Case 𝑗 ≤ 𝑘 ≤ 𝑖 = 𝑙. We have ℎ𝑙 = ℎ′𝑙 = ℎ, because both 𝛾𝑙 and 𝛿𝑙 contain the closing parenthesis ⟩ℎ .

We have shown that ℎ𝑖 = 𝜎𝑖 ∧ ℎ′𝑖 = 𝜎𝑖′ ∀𝑖 ∶ 𝑗 ≤ 𝑖 < 𝑘 and ℎ𝑖 = ℎ′𝑖 ∀𝑖 ∶ 𝑘 ≤ 𝑖 ≤ 𝑙. It trivially follows that Φℎ+1 (𝑠′ ) ⊏ Φℎ+1 (𝑡′ )
⇒ Φℎ (𝑠) ⊏ Φℎ (𝑡) and Φℎ+1 (𝑠′ ) ∼ Φℎ+1 (𝑡′ ) ⇒ Φℎ (𝑠) ∼ Φℎ (𝑡). Because none of Φℎ+1 (𝑠′ ), Φℎ+1 (𝑡′ ) is a proper prefix of another,
Φℎ+1 (𝑠′ ) ⊂ Φℎ+1 (𝑡′ ) ⇒ Φℎ (𝑠) ⊂ Φℎ (𝑡). Therefore Φℎ+1 (𝑠′ ) < Φℎ+1 (𝑡′ ) ⇒ Φℎ (𝑠) < Φℎ (𝑡) (the premise has been shown).
Theorem 5. If 𝑠, 𝑡 ∈ 𝑃 𝑇 (𝑒, 𝑤) for some IRE 𝑒 and string 𝑤, then 𝑠 ≺ 𝑡 ⇔ Φℎ (𝑠) < Φℎ (𝑡) ∀ℎ.
Proof. Forward implication is given by lemma 10. Backward implication: suppose, on the contrary, that Φℎ (𝑠) < Φℎ (𝑡) ∀ℎ, but
𝑠 ⊀ 𝑡. Since ≺ is a strict weak order (by theorem 2), it must be that either 𝑠 ∼ 𝑡 (then Φℎ (𝑠) = Φℎ (𝑡) ∀ℎ by lemma 8), or 𝑡 ≺ 𝑠
(then Φℎ (𝑡) < Φℎ (𝑠) ∀ℎ by lemma 10). Both cases contradict Φℎ (𝑠) < Φℎ (𝑡) ∀ℎ, therefore assumption 𝑠 ⊀ 𝑡 is incorrect.

Correctness of incremental path comparison
Lemma 1 (Frame-by-frame comparison of PEs). If 𝛼, 𝛽 are comparable PE prefixes, 𝑐 is an alphabet symbol and 𝛾 is a singleframe PE fragment, then 𝛼 < 𝛽 implies 𝛼𝑐𝛾 < 𝛽𝑐𝛾.
)
(
Proof. Let (ℎ1 , … , ℎ𝑛 ), (ℎ′1 , … , ℎ′𝑛 ) = 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) where 𝑛 ≥ 1. Since 𝛼𝑐𝛾, 𝛽𝑐𝛾 have one more frame than 𝛼, 𝛽 and the first
(
)
𝑛 frames are identical to frames of 𝛼, 𝛽, we can represent 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼𝑐𝛾, 𝛽𝑐𝛾) as (ℎ1 , … , ℎ𝑛 , ℎ𝑛+1 ), (ℎ′1 , … , ℎ′𝑛 , ℎ′𝑛+1 ) .

(1) Case 𝛼 ∼ 𝛽 ∧𝛼 ⊂ 𝛽. In this case ℎ𝑖 = ℎ′𝑖 ∀𝑖 ≤ 𝑛, therefore ℎ𝑛+1 = 𝑚𝑖𝑛(ℎ𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = 𝑚𝑖𝑛(ℎ′𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = ℎ′𝑛+1 and 𝛼𝑐𝛾 ∼
𝛽𝑐𝛾. Furthermore, 𝑓 𝑖𝑟𝑠𝑡(𝛼𝑐𝛾∖𝛽𝑐𝛾) = 𝑓 𝑖𝑟𝑠𝑡(𝛼∖𝛽) and 𝑓 𝑖𝑟𝑠𝑡(𝛽𝑐𝛾∖𝛼𝑐𝛾) = 𝑓 𝑖𝑟𝑠𝑡(𝛽∖𝛼), therefore 𝛼 ⊂ 𝛽 ⇐⇒ 𝛼𝑐𝛾 ⊂ 𝛽𝑐𝛾.
(2) Case 𝛼 ⊏ 𝛽. In this case ∃𝑗 ≤ 𝑛 such that ℎ𝑗 > ℎ′𝑗 and ℎ𝑖 = ℎ′𝑖 ∀𝑗 < 𝑖 ≤ 𝑛. We show that ∃𝑙 ≤ 𝑛 + 1 such that ℎ𝑙 > ℎ′𝑙 and
ℎ𝑖 = ℎ′𝑖 ∀𝑙 < 𝑖 ≤ 𝑛 + 1, which by definition means that 𝛼𝑐𝛾 ⊏ 𝛽𝑐𝛾.
(2a) Case 𝑗 < 𝑛. In this case ℎ𝑛 = ℎ′𝑛 and ℎ𝑛+1 = 𝑚𝑖𝑛(ℎ𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = 𝑚𝑖𝑛(ℎ′𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = ℎ′𝑛+1 , therefore 𝑙 = 𝑗.
(2b) Case 𝑗 = 𝑛 and 𝑚𝑖𝑛ℎ(𝛾) > ℎ′𝑛 . In this case ℎ𝑛 > ℎ′𝑛 and we have ℎ𝑛+1 = 𝑚𝑖𝑛(ℎ𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) > ℎ′𝑛 and ℎ′𝑛+1 =
𝑚𝑖𝑛(ℎ′𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = ℎ′𝑛 , therefore ℎ𝑛+1 > ℎ′𝑛+1 and 𝑙 = 𝑛 + 1.
(2c) Case 𝑗 = 𝑛 and 𝑚𝑖𝑛ℎ(𝛾) ≤ ℎ′𝑛 . In this case ℎ𝑛 > ℎ′𝑛 and we have ℎ𝑛+1 = 𝑚𝑖𝑛(ℎ𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = 𝑚𝑖𝑛ℎ(𝛾) and
ℎ′𝑛+1 = 𝑚𝑖𝑛(ℎ′𝑛 , 𝑚𝑖𝑛ℎ(𝛾)) = 𝑚𝑖𝑛ℎ(𝛾), therefore ℎ𝑛+1 = ℎ′𝑛+1 and 𝑙 = 𝑛.

In both cases 𝛼𝑐𝛾 < 𝛽𝑐𝛾.

Lemma 2. Minimal paths do not contain tagged 𝜖-loops.

<!-- page 32 -->

32

Proof. Suppose, on the contrary, that 𝜋 is a minimal path in some TNFA and that 𝜋 contains at least one tagged 𝜖-loop. We
show that it is possible to construct another path 𝜋 ′ such that 𝜋 ′ < 𝜋. Path 𝜋 can be represented as 𝜋 = 𝜋1 𝜋2 𝜋3 , where
𝜋1 = 𝑞0 ←→ 𝑞, 𝜋2 = 𝑞 ←→ 𝑞 is the last tagged 𝜖-loop on 𝜋 and 𝜋3 = 𝑞 ←→ 𝑞𝑓 . Let 𝜋 ′ = 𝜋1 𝜋3 be the path that is obtained
from 𝜋 by removing the loop 𝜋2 . Paths 𝜋 and 𝜋 ′ consume the same input string 𝑢𝑣 and induce comparable PEs 𝛼𝛽𝛾 and 𝛼𝛾. Let
(
)
(ℎ1 , … , ℎ𝑛 ), (ℎ′1 , … , ℎ′𝑛 ) = 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼𝛽𝛾, 𝛼𝛾) and let 𝑘 be the index of the fork frame. By construction of TNFA the loop 𝜋2
must be contained in a sub-TNFA 𝑓 for sub-IRE of the form 𝑒 = 𝑅𝑒𝑝( , , 𝑒1 , 1, ∞), as this is the only looping TNFA construct
— see algorithm 7. Let 𝑓1 be the sub-TNFA for 𝑒1 . Path 𝜋 enters 𝑓 and iterates through 𝑓1 at least twice before leaving 𝑓 (single
iteration is not enough to create a loop by TNFA construction). Let 𝑗 be the total number of iterations through 𝑓1 , and let 𝑖 be
the index of the last 𝜖-loop iteration (note that not all iterations are necessarily 𝜖-loops). Consider two possible cases:
𝑢|𝛼

𝜖|𝛽

𝑣|𝛾

(1) Case 𝑖 = 𝑗. In this case fork of 𝛼𝛽𝛾 and 𝛼𝛾 happens immediately after (𝑖 − 1)-th iteration:
|
𝛼𝛽𝛾 = 𝑥0 ⟨ℎ−1 ⟨ℎ 𝑥1 ⟩ℎ … ⟨ℎ 𝑥𝑖−1 ⟩ℎ | ⟨ℎ 𝑥𝑖 ⟩ℎ ⟩ℎ−1 𝑥𝑗+1
||
𝛼𝛾 = 𝑥0 ⟨ℎ−1 ⟨ℎ 𝑥1 ⟩ℎ … ⟨ℎ 𝑥𝑖−1 ⟩ℎ |
⟩ℎ−1 𝑥𝑗+1
|
Since 𝑥𝑖 is an 𝜖-loop, it is contained in the fork frame of 𝛼𝛽𝛾. We have 𝑚𝑖𝑛ℎ(𝛽) = ℎ and 𝑚𝑖𝑛ℎ(𝛾) ≤ ℎ − 1, therefore
ℎ𝑘 = ℎ′𝑘 ≤ ℎ−1. Subsequent frames 𝑙 > 𝑘 (if any) are identical and thus ℎ𝑙 = ℎ′𝑙 . Furthermore, 𝑓 𝑖𝑟𝑠𝑡(𝛾) =⟩ < ⟨= 𝑓 𝑖𝑟𝑠𝑡(𝛽).
Therefore 𝛼𝛽𝛾 ∼ 𝛼𝛾 and 𝛼𝛾 ⊂ 𝛼𝛽𝛾.
(2) Case 𝑖 < 𝑗. In this case (𝑖 + 1)-th iteration cannot be an 𝜖-loop (because we assumed that 𝑖-th iteration is the last 𝜖-loop),
therefore the fork of 𝛼𝛽𝛾 and 𝛼𝛾 happens inside of 𝑖-th iteration of 𝛼𝛽𝛾 and (𝑖 + 1)-th iteration of 𝛼𝛾:
|
𝛼𝛽𝛾 = 𝑥0 ⟨ℎ−1 ⟨ℎ 𝑥1 ⟩ℎ … ⟨ℎ 𝑥𝑖−1 ⟩ℎ ⟨ℎ 𝑦1 | 𝑦2 ⟩ℎ ⟨ℎ 𝑥𝑖+1 ⟩ℎ ⟨ℎ 𝑥𝑖+2 ⟩ℎ … ⟨ℎ 𝑥𝑗 ⟩ℎ ⟩ℎ−1 𝑥𝑗+1
||
𝛼𝛾 = 𝑥0 ⟨ℎ−1 ⟨ℎ 𝑥1 ⟩ℎ … ⟨ℎ 𝑥𝑖−1 ⟩ℎ ⟨ℎ 𝑦1 | 𝑦3
⟩ℎ ⟨ℎ 𝑥𝑖+2 ⟩ℎ … ⟨ℎ 𝑥𝑗 ⟩ℎ ⟩ℎ−1 𝑥𝑗+1
|
Here 𝑦1 𝑦2 = 𝑥𝑖 and 𝑦1 𝑦3 = 𝑥𝑖+1 (𝑖-th iteration is missing from 𝛼𝛾 by construction of 𝜋 ′ ). Fragment 𝑦2 is part of the 𝜖loop, therefore fork frame of 𝛼𝛽𝛾 contains a parenthesis ⟩ℎ and we have ℎ𝑘 = ℎ. On the other hand, 𝑦3 contains alphabet
symbols, because 𝑥𝑖+1 is not an 𝜖-loop and 𝑦1 is a part of the 𝜖-loop. Therefore fork frame of 𝛼𝛾 ends in 𝑦3 and we have
ℎ′𝑘 > ℎ. All subsequent frames 𝑙 > 𝑘 are identical: if they contain parentheses of height less than ℎ, then ℎ𝑙 = ℎ′𝑙 < ℎ;
otherwise ℎ𝑙 ≤ ℎ and ℎ′𝑙 > ℎ. Therefore 𝛼𝛾 ⊏ 𝛼𝛽𝛾.

In both cases 𝛼𝛾 < 𝛼𝛽𝛾, which contradicts the fact that 𝜋 is a minimal path.
Lemma 3. GOR1 and GTOP discard paths with tagged 𝜖-loops.

Proof. Suppose that GOR1/GTOP finds path 𝜋1 𝜋2 where 𝜋1 = 𝑞0 ←→ 𝑞1 and 𝜋2 = 𝑞1 ←→ 𝑞1 is a tagged 𝜖-loop. Both algorithms
construct new paths by exploring transitions from the end state of existing paths, so they can only find 𝜋1 𝜋2 after they find
𝑠|𝛼

𝜖|𝛾

𝑠|𝛼 ′

𝜋1 . Therefore when GOR1/GTOP finds 𝜋1 𝜋2 , it already has some shortest-path candidate 𝜋1′ = 𝑞0 ←→ 𝑞1 and must compare
ambiguous paths 𝜋1 𝜋2 and 𝜋1′ . There are two possibilities: either 𝛼 ′ = 𝛼 or 𝛼 ′ < 𝛼 (the latter means that the algorithm has found
(
)
a shorter path to 𝑞1 in between finding 𝜋1 and 𝜋1 𝜋2 ). Let (ℎ1 , … , ℎ𝑘 ), (ℎ′1 , … , ℎ′𝑘 ) = 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼 ′ , 𝛼𝛾).
(1) Case 𝛼 ′ = 𝛼. Because 𝛼 is a proper prefix of 𝛼𝛾, fork happens at the last frame and we have ℎ𝑘 = 𝑙𝑎𝑠𝑡ℎ(𝛼) and ℎ′𝑘 =
𝑚𝑖𝑛(𝑙𝑎𝑠𝑡ℎ(𝛼), 𝑚𝑖𝑛ℎ(𝛾)). If 𝑙𝑎𝑠𝑡ℎ(𝛼) > 𝑚𝑖𝑛ℎ(𝛾), then ℎ𝑘 > ℎ′𝑘 and 𝛼 ⊏ 𝛼𝛾. Otherwise ℎ𝑘 = ℎ′𝑘 and 𝛼 ∼ 𝛼𝛾, and we have
𝑓 𝑖𝑟𝑠𝑡(𝛼∖𝛼𝛾) = ⊥ and 𝑓 𝑖𝑟𝑠𝑡(𝛼𝛾∖𝛼) ≠ ⊥, therefore 𝛼 ⊂ 𝛼𝛾. In both cases 𝛼 < 𝛼𝛾.
(
)
(2) Case 𝛼 ′ < 𝛼. Let (𝜎1 , … , 𝜎𝑘 ), (𝜎1′ , … , 𝜎𝑘′ ) = 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼 ′ , 𝛼). We have ℎ𝑘 = 𝜎𝑘 and ℎ′𝑘 = 𝑚𝑖𝑛(𝜎𝑘′ , 𝑚𝑖𝑛ℎ(𝛾)) ≤ 𝜎𝑘 . If
𝑚𝑖𝑛ℎ(𝛾) < 𝜎𝑘′ then ℎ𝑘 > ℎ′𝑘 and 𝛼 ′ ⊏ 𝛼𝛾. Otherwise ℎ′𝑘 = 𝜎𝑘′ . If 𝛼 ′ ⊏ 𝛼 then 𝛼 ′ ⊏ 𝛼𝛾. Otherwise 𝛼 ′ ∼ 𝛼 and 𝛼 ′ ⊂ 𝛼. None
of 𝛼 and 𝛼 ′ is a proper prefix of the other because otherwise the longer path has an 𝜖-loop through 𝑞1 , which contradicts
our assumption about 𝜋1 and 𝜋1′ . Therefore 𝑓 𝑖𝑟𝑠𝑡(𝛼 ′ ∖𝛼) = 𝑓 𝑖𝑟𝑠𝑡(𝛼 ′ ∖𝛼𝛾) and 𝑓 𝑖𝑟𝑠𝑡(𝛼∖𝛼 ′ ) = 𝑓 𝑖𝑟𝑠𝑡(𝛼𝛾∖𝛼 ′ ). Consequently
𝛼 ′ ⊂ 𝛼 ⇐⇒ 𝛼 ′ ⊂ 𝛼𝛾. Thus 𝛼 ′ < 𝛼𝛾.

In both cases 𝛼 ′ < 𝛼𝛾, therefore path 𝜋1 𝜋2 is discarded.

Lemma 4 (Right distributivity of comparison over concatenation for paths without tagged 𝜖-loops). Let 𝜋𝛼 = 𝑞0 ←→ 𝑞1 and
𝑢|𝛼

𝜋𝛽 = 𝑞0 ←→ 𝑞1 be ambiguous paths in TNFA 𝑓 for IRE 𝑒, and let 𝜋𝛾 = 𝑞1 ←→ 𝑞2 be their common 𝜖-suffix, such that 𝜋𝛼 𝜋𝛾 and
𝜋𝛽 𝜋𝛾 do not contain tagged 𝜖-loops. If 𝛼 < 𝛽 then 𝛼𝛾 < 𝛽𝛾.
𝑢|𝛽

𝜖|𝛾

<!-- page 33 -->

33

(
)
(
)
Proof. Let (ℎ1 , … , ℎ𝑘 ), (ℎ′1 , … , ℎ′𝑘 ) = 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼, 𝛽) and (𝜎1 , … , 𝜎𝑘 ), (𝜎1′ , … , 𝜎𝑘′ ) = 𝑡𝑟𝑎𝑐𝑒𝑠(𝛼𝛾, 𝛽𝛾). Appending 𝛾 to 𝛼 and
𝛽 changes only the last frame, therefore for frames 𝑖 < 𝑘 we have ℎ𝑖 = 𝜎𝑖 and ℎ′𝑖 = 𝜎𝑖′ . Consider two possible cases.

(1) Case 𝛼 ∼ 𝛽 ∧ 𝛼 ⊂ 𝛽. We show that 𝛼𝛾 ∼ 𝛽𝛾 ∧ 𝛼𝛾 ⊂ 𝛽𝛾. We have ℎ𝑖 = ℎ′𝑖 ∀𝑖 (implied by 𝛼 ∼ 𝛽), therefore 𝜎𝑖 = 𝜎𝑖′ ∀𝑖
and consequently 𝛼𝛾 ∼ 𝛽𝛾. Let 𝑥 = 𝑓 𝑖𝑟𝑠𝑡(𝛼∖𝛽), 𝑦 = 𝑓 𝑖𝑟𝑠𝑡(𝛽∖𝛼), 𝑥′ = 𝑓 𝑖𝑟𝑠𝑡(𝛼𝛾∖𝛽𝛾) and 𝑦′ = 𝑓 𝑖𝑟𝑠𝑡(𝛽𝛾∖𝛼𝛾). If one of
𝜋𝛼 and 𝜋𝛽 is a proper prefix of another, then the longer path contains tagged 𝜖-loop through 𝑞1 , which contradicts lemma
conditions (the suffix of the longer path must be an 𝜖-path, because 𝛼 and 𝛽 have the same number of frames and the suffix
is contained in the last frame). Therefore none of 𝜋𝛼 and 𝜋𝛽 is a proper prefix of another. Consequently 𝑥 = 𝑥′ and 𝑦 = 𝑦′ ,
and we have 𝛼 ⊂ 𝛽 ⇐⇒ 𝑥 < 𝑦 ⇐⇒ 𝑥′ < 𝑦′ ⇐⇒ 𝛼𝛾 ⊂ 𝛽𝛾.
(2) Case 𝛼 ⊏ 𝛽: by definition this means that ∃𝑗 ≤ 𝑘 such that ℎ𝑗 > ℎ′𝑗 and ℎ𝑖 = ℎ′𝑖 ∀𝑖 > 𝑗. We show that 𝛼𝛾 ⊏ 𝛽𝛾.
(2a) Case 𝑗 < 𝑘. In this case ℎ𝑘 = ℎ′𝑘 and appending 𝛾 does not change relation on the last frame: 𝜎𝑘 = 𝑚𝑖𝑛(ℎ𝑘 , 𝑚𝑖𝑛ℎ(𝛾)) =
𝑚𝑖𝑛(ℎ′𝑘 , 𝑚𝑖𝑛ℎ(𝛾)) = 𝜎𝑘′ . Since 𝜎𝑖 = ℎ𝑖 and 𝜎𝑖′ = ℎ′𝑖 for all preceding frames 𝑖 < 𝑘, we have 𝛼𝛾 ⊏ 𝛽𝛾.
(2b) Case 𝑗 = 𝑘 and 𝑚𝑖𝑛ℎ(𝛾) > ℎ′𝑘 . In this case ℎ𝑘 > ℎ′𝑘 and again appending 𝛾 does not change relation on the last
frame: 𝜎𝑘 = 𝑚𝑖𝑛(ℎ𝑘 , 𝑚𝑖𝑛ℎ(𝛾)) > ℎ′𝑘 and 𝜎𝑘′ = 𝑚𝑖𝑛(ℎ′𝑘 , 𝑚𝑖𝑛ℎ(𝛾)) = ℎ′𝑘 , therefore 𝜎𝑘 > 𝜎𝑘′ . Therefore 𝛼𝛾 ⊏ 𝛽𝛾.
(2c) Case 𝑗 = 𝑘 and 𝑚𝑖𝑛ℎ(𝛾) ≤ ℎ′𝑘 and ∃𝑙 < 𝑘 such that ℎ𝑙 > ℎ′𝑙 and ℎ𝑖 = ℎ′𝑖 for 𝑙 < 𝑖 < 𝑘. In this case 𝛾 contains
parentheses of low height and appending it makes height on the last frame equal: 𝜎𝑘 = 𝜎𝑘′ = 𝑚𝑖𝑛ℎ(𝛾). However, the
relation on the last preceding differing frame is the same: 𝜎𝑙 = ℎ𝑙 > ℎ′𝑙 = 𝜎𝑙′ . Therefore 𝛼𝛾 ⊏ 𝛽𝛾.
(2d) Case 𝑗 = 𝑘 and 𝑚𝑖𝑛ℎ(𝛾) ≤ ℎ′𝑘 and ∄𝑙 < 𝑘 such that ℎ𝑙 > ℎ′𝑙 and ℎ𝑖 = ℎ′𝑖 for 𝑙 < 𝑖 < 𝑘. In this case 𝛾 contains
parentheses of low height, appending it makes height on the last frame equal: 𝜎𝑘 = 𝜎𝑘′ = 𝑚𝑖𝑛ℎ(𝛾), and this may
change comparison result as the relation on the last preceding differing frame may be different. We show that in this
case the extended path 𝜋𝛽 𝜋𝛾 contains a tagged 𝜖-loop. Consider the fragments of paths 𝜋𝛼 and 𝜋𝛽 from fork to join,
including (if it exists) the common 𝜖-transition to the fork state: 𝜋𝛼′ and 𝜋𝛽′ . Minimal parenthesis height on 𝜋𝛼′ is ℎ𝑘 .
By TNFA construction this means that 𝜋𝛼′ is contained in a sub-TNFA 𝑓 ′ for 𝑒|𝑥 at some position 𝑥 with length
|𝑥| = ℎ𝑘 . As for 𝜋𝛽′ , its start state coincides with 𝜋𝛼′ and thus is in 𝑓 ′ . The minimal height of all but the last frames
of 𝜋𝛽′ is at least ℎ𝑘 : by conditions of (2d) either 𝑘 = 1 and there are no such frames, or ℎ′𝑘−1 ≥ ℎ𝑘−1 which implies
ℎ′𝑘−1 ≥ ℎ𝑘 (because by definition ℎ𝑘 = 𝑚𝑖𝑛(ℎ𝑘−1 , 𝑚𝑖𝑛ℎ(𝛼𝑘 )) ≤ ℎ𝑘−1 ). On the last frame of 𝜋𝛽′ minimal height is
ℎ′𝑘 < ℎ𝑘 . Therefore all but the last frames of 𝜋𝛽′ are contained in 𝑓 ′ , but the the last frame is not. Now consider 𝜋𝛾 :
by conditions of (2d) its minimal height is less than ℎ𝑘 , therefore it is not contained in 𝑓 ′ , but its start state is the
join point of 𝜋𝛼′ and 𝜋𝛽′ and thus in 𝑓 ′ . Taken together, above facts imply that the last frame of 𝜋𝛽 𝜋𝛾 starts in 𝑓 ′ , then
leaves 𝑓 ′ , then returns to 𝑓 ′ and joins with 𝜋𝛼 𝜋𝛾 , and then leaves 𝑓 ′ second time. Since the end state of 𝑓 ′ is unique
(by TNFA construction), 𝜋𝛽 𝜋𝛾 must contain a tagged 𝜖-loop through it, which contradicts lemma conditions.

(Note that in the presence of tagged 𝜖-loops right distributivity may not hold: we may have paths 𝜋1 , 𝜋2 and 𝜋3 such that 𝜋2 and
𝜋3 are two different 𝜖-loops through the same subautomaton and 𝜋1 𝜋2 < 𝜋1 𝜋3 , in which case 𝜋1 𝜋2 𝜋3 < 𝜋1 𝜋3 , but 𝜋1 < 𝜋1 𝜋2
because the first is a proper prefix of the second.)
