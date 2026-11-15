---
file: ../pdfs/2026-arxiv-bille-regex-simple-backreferences.pdf
citation: "Improved Regular Expression Matching with Simple Backreferences; Bille, Gørtz, Jessen; arXiv:2609.16914; 2026"
doi: 
source: "https://arxiv.org/pdf/2609.16914"
topics: [semantics_theory_access]
pages: 20
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Improved Regular Expression Matching with Simple Backreferences
Philip Bille
phbi@dtu.dk

Inge Li Gørtz
inge@dtu.dk

Rikke Schjeldrup Jessen
rscje@dtu.dk

arXiv:2609.16914v1 [cs.DS] 15 Sep 2026

Abstract
A regular expression with backreferences (rewb) specifies a set of strings formed by characters combined with concatenation, union, star operators, and backreferences. A backreference
consists of a capturing group (·)i and a reference \i. The substring matched by the reference
must match the substring matched by the corresponding capturing group. Given a rewb R
and a string Q, the rewb matching problem is to decide whether Q is one of the strings specified by R. Rewb matching is a basic tool in computer science for searching and processing
text, is supported in most modern programming languages, and is used across a wide range of
applications.
In full generality, rewb matching is NP-complete, but efficient solutions exist for various
subclasses. In the paper, we focus on rewb containing a single capturing group and k references.
For this class, Uezato [CPM 2026] gave an O((kn2 m2 ) time and O(n2 m2 ) space algorithm,
where m is the length of the regular expression R and n is the length of the string Q. For the
special case of k = 1, Nogami and Terauchi [MFCS 2025] gave an O(n2 m2 ) time and O(n + m2 )
space algorithm. On the other hand, Nogami, Nakamura, and Terauchi [arXiv 2026] gave a
conditional lower bound, showing that we cannot solve the problem in O(n2−ϵ poly(m)) for any
ϵ > 0 assuming the orthogonal vector hypothesis. Our main result is a new algorithm that runs
in O(n2 m) time and uses O(nm) space. This improves the above results (by a factor of km and
m, respectively) and the former’s space bound (by a factor of nm). We also show how to extend
our algorithm to handle a slightly more general class of ordered and single-nested rewbs.
To achieve our results, we combine suffix-tree enumeration of candidate strings for the capturing groups with a new interval-tree data structure. The interval tree stores state-sets of
Thompson’s finite automaton for dyadic substrings of the input and lets us efficiently propagate
feasible match endpoints across multiple references and capturing groups.

1

Introduction

A regular expression with backreferences (rewb) specifies a set of strings formed by characters
combined with concatenation (⊙), union (|), star (∗ ) operator, and backreferences. A backreference consists of a numbered capturing group (e)i and a corresponding numbered reference \i.
The substring matched by the reference has to be identical to the substring matched by the corresponding capturing group. For example, the rewb c(a∗ )1 (b|c)a\1 (by convention, we often omit
the ⊙ symbol when writing expressions) matches the string caabaaa, with the capturing group and
reference matching the substring aa. Given a rewb R and a string Q, the regular expression with
backreferences matching problem is to decide whether Q is one of the strings specified by R.
Rewb matching is a basic tool in computer science for searching and processing text and is
supported in most modern programming languages. Rewb matching appears across a wide range

1

<!-- page 2 -->

of applications such as network intrusion detection [18], graph-database querying [22], software
testing and symbolic execution [15], and string-constraint solving [8].
In full generality, rewb matching is NP-complete [1] and upper and lower bounds have been
studied for various subclasses of rewb [10, 12, 13, 14, 19, 20, 24]. In this paper, we focus on rewb
matching for rewbs that contain a single capturing group and one or more references. This class
of expressions captures many practical uses of rewb [20]. To state the previous results, let R
be a rewb of length m with a single capturing group and k ≥ 1 references (R is of the form
e0 (e)1 e1 \1 e2 \1 e3 . . . ek \1 ek+1 ) and let Q be a string of length n. If k = 1, (i.e., R = e0 (e)1 e1 \1e2 ),
Nogami and Terauchi [20] gave an O(n2 m2 ) time and O(n + m2 ) space algorithm. For general k,
Uezato [24] gave an O((kn2 m2 ) time and O(n2 m2 ) space algorithm. On the other hand, Nogami,
Nakamura, and Terauchi [19] gave a conditional lower bound, showing that we cannot solve the
problem in O(n2−ϵ poly(m)) time for any ϵ > 0, assuming the orthogonal vectors hypothesis. Thus,
the exponent on n is conditionally optimal for a polynomial dependency on m. A recent result by
Kumabe and Uezato [13] shows that it is possible to obtain a subquadratic time dependency on n
at the cost of a multiplicative exponential dependency on m.
Results

Our main result is the following.

Theorem 1. Given a regular expression of length m with a single capturing group and k references
and a string of length n, we can solve the regular expression matching with backreferences problem
in O(n2 m) time and O(nm) space.
Theorem 1 improves the previous results for the polynomial dependency on m regime. Compared to Uezato [24], we improve the time by a factor km and the space by a factor nm. Compared
to the Nogami and Terauchi [20] result for k = 1, we improve the time bound by a factor m.
We also show how to generalize our results to a slightly larger subclass of rewbs. We say that
a rewb R is ordered if all references to the ith capturing group appear before the i + 1st capturing
group and after the i − 1st capturing group. Furthermore, we define the nested level of R [14]
recursively as follows: a regular expression (with no backreferences) has level 0. The nested level
of (e)i and the reference \i is one more than the nested level of e. The nested level of R is the
maximal nested level of its subexpressions. For example, the expression (a∗ b)1 c(\1|ab)2 \2 is nested
with level 2, since (a∗ b)1 and \1 is nested with level 1. If R is nested with level 1, we say that R is
single-nested. Note that the above setting for Theorem 1 is a special case of ordered, single-nested
rewb. We show how to extend Theorem 1 to this more general setting.
Theorem 2. Given an ordered, single-nested regular expression of length m and a string of length
n, we can solve the regular expression matching with backreferences problem in O(n2 m) time and
O(nm) space.
Technical overview Our main tool is an interval tree, a complete binary tree T whose nodes
represent dyadic intervals of Q. For a regular expression e (without backreferences), each node v
in T stores, for every position on either side of its midpoint, a bit vector representing the states in
a finite automaton (Thompson’s NFA construction [23]) that are reachable by the corresponding
left or right substring. Any pair of endpoints is separated at a unique lowest node of the tree, and
the substring between them matches e precisely when the corresponding left and right state-sets
intersect. Consequently, a tree traversal can process many candidate endpoints simultaneously by
taking unions of left state-sets and intersecting them with right state-sets.
2

<!-- page 3 -->

For an expression R = e0 (e)1 e1 \1e2 , we use a suffix tree together with Thompson’s NFA for e to
enumerate every distinct substring q of Q that matches e, together with all occurrences of q. From
these occurrences and the matches of e0 and e2 , we obtain the possible boundaries surrounding e1 .
Then, traversing the interval tree for e1 we determine whether some choice gives a match of R.
We show that an amortized analysis over all q and all tree nodes gives O(n2 m) time and O(nm)
space. For several references to the same capturing group, we traverse one interval tree for each
intervening regular expression and propagate the feasible prefix endpoints from one traversal to
the next. Finally, an ordered, single-nested rewb decomposes into a sequence of such single-group
blocks, which we process successively while carrying forward the feasible endpoints.
Outline In Section 2 we review notation for strings, automata and suffix trees. Section 3
introduces the interval tree data structure. In Section 4, we present an algorithm to match the
simpler class of rewbs with only one capturing group and one reference. In Section 5, we generalize
the result to multiple references, and in Appendix 6 we generalize further to ordered and singlenested rewbs.

2

Preliminaries

Strings A string Q is a sequence of characters Q[0]Q[1] . . . Q[n − 1], such that Q[i] is the i’th
character of Q, and the substring Q[i, j] is the concatenation of characters Q[i]Q[i + 1] . . . Q[j]. We
also say that Q[i, i − 1] is the empty string, which we denote ε.
Finite automata A finite automaton is a tuple A = (V, E, Σ, Θ, Φ), where V is a set of nodes
called states, E ⊆ (V × V × Σ ∪ {ϵ}) is a set of directed edges between states called transitions
each labeled by a character from Σ ∪ {ϵ}, Θ ⊆ V is a set of start states, and Φ ⊆ V is a set of
accepting states. In short, A is an edge-labeled directed graph with designated subsets of start
and accepting nodes. A is a deterministic finite automaton (DFA) if A does not contain any ϵtransitions, all outgoing transitions of any state have different labels, and there is exactly one start
state. Otherwise, A is a nondeterministic finite automaton (NFA). For an automaton A, we also
←
−
define the reverse automaton A , which is the automaton with the same states as A, but with the
transition (u, v, a) for every transition (v, u, a) in A.
Given a string Q and a path p in A, we say that p and Q match if the concatenation of the
labels on the transitions in p is Q. Given a state s in A and a character α we define the state-set
transition δA (s, α) to be the set of states reachable from s through paths matching α S
(note that the
paths may include transitions labeled ϵ). For a set of states S we define δA (S, α) = s∈S δA (s, α).
We say that A accepts a string Q if there is a path from a state in Θ to a state in Φ that matches
Q. Otherwise, A rejects Q. We can use a sequence of state-set transitions to test acceptance of a
string Q of length n by computing a sequence of state-sets S0 , . . . , Sn , given by S0 = δA (Θ, ϵ) and
Si = δA (Si−1 , Q[i]), i = 1, . . . , n. We have that Φ ∩ Sn ̸= ∅ iff A accepts Q.
Thompson automata Given a regular expression R, we can construct an NFA accepting
precisely the strings in L(R) by several classical methods [11, 16, 23]. In particular, Thompson [23]
gave the simple textbook construction shown in Figure 1. We will call an NFA constructed with
these rules a Thompson NFA (TNFA). A TNFA N (R) for R has at most 2m states, at most 4m
transitions, and can be computed in O(m) time. Furthermore, a TNFA has only one start state, θ,
3

<!-- page 4 -->

(a)

ϵ

α

N (S)

(b)

ϵ

ϵ

(c)
ϵ

N (T )

N (S)

(d)

ϵ

ϵ

N (S)

ϵ

N (T )
ϵ

Figure 1: Thompson’s recursive NFA construction. The regular expression α ∈ Σ ∪ {ϵ} corresponds
to NFA (a). If S and T are regular expressions then N (ST ), N (S|T ), and N (S ∗ ) correspond to
NFAs (b), (c), and (d), respectively. In each of these figures, the leftmost node θ and rightmost
node ϕ are the start and the accept nodes, respectively. For the top recursive calls, these are the
start and accept nodes of the overall automaton. In the recursions indicated, e.g., for N (ST ) in
(b), we take the start node of the subautomaton N (S) and identify with the state immediately to
the left of N (S) in (b). Similarly the accept node of N (S) is identified with the state immediately
to the right of N (S) in (b).
and one accepting state, ϕ. We use subscripts θA and ϕA to specify the start and accepting states
of automaton A when needed. We can compute a state-set transition on a single character in O(m)
time with a breadth-first search. Thus, we can solve regular expression matching on Q in O(nm)
time and O(m) space using the state-set transition algorithm above. This bound can be slightly
improved by polylogarithmic factors, but by conditional lower bounds we should not hope to solve
regular expression matching in O((nm)1−ϵ ) time for ϵ > 0, see e.g., [2, 3, 4, 5, 6, 7, 17, 21].
Suffix trees The suffix tree ST of a string Q over an alphabet Σ of length n is the compacted
trie over the set of all suffixes of Q. Each leaf is represented by the starting index in Q of the suffix
it represents. For a node v, we define str(v) to be the string consisting of the concatenated labels
on the path from the root to v. The string depth of v is the length of str(v). The locus of a string
q in ST is the node v in ST with the smallest string depth such that q is a prefix of str(v). We
can construct the suffix tree in sort(n, |Σ|) time, where sort(n, u) is the time to sort n integers of
an alphabet of size u [9] and represent it in O(n) space. Given the locus of a string q in ST , we
can output all starting indices of occurrences of q in Q by traversing the subtree below the locus
of P in O(|occ q |) time, where |occ q | is the number of occurrences of q in Q.

3

Interval Trees

In this section, we introduce a data structure which we use to match the patterns throughout this
paper.
Let e be a regular expression of length m with corresponding TNFA A and let Q be a string of
length n (we assume without loss of generality that n is a power of 2). The interval tree T of Q
for e is a complete binary tree over Q such that the nodes of T correspond to the dyadic intervals
of Q. The leaves in T correspond to intervals of length 1 in Q and an internal node v of height k
corresponds to an interval of length 2k . See Figure 3(a) for an example.
4

<!-- page 5 -->

<latexit sha1_base64="gBd+oeQAzTe6ydFI84S/5pp57As=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FXQvQY8OIxonlAsoTZySQZMju7zPQKYcknePGgiFe/yJt/4yTZgyYWNBRV3XR3BbEUBl3328ltbG5t7+R3C3v7B4dHxeOTlokSzXiTRTLSnYAaLoXiTRQoeSfWnIaB5O1gcjv3209cGxGpR5zG3A/pSImhYBSt9FAOLvvFkltxFyDrxMtICTI0+sWv3iBiScgVMkmN6XpujH5KNQom+azQSwyPKZvQEe9aqmjIjZ8uTp2RC6sMyDDSthSShfp7IqWhMdMwsJ0hxbFZ9ebif143weGNnwoVJ8gVWy4aJpJgROZ/k4HQnKGcWkKZFvZWwsZUU4Y2nYINwVt9eZ20riperVK7r5bq1SyOPJzBOZTBg2uowx00oAkMRvAMr/DmSOfFeXc+lq05J5s5hT9wPn8AiYyNSA==</latexit>

(b) leftmatchstates(v, i)
<latexit sha1_base64="PkjDEP7GirnQCEdVQVyXFc7O1MM=">AAACB3icbZDLSsNAFIYnXmu9RV0KEixCBSmJSHVZcOOygr1AG8pketIOnVyYOSmU0J0bX8WNC0Xc+grufBsnbRba+sPAx3/OYc75vVhwhbb9baysrq1vbBa2its7u3v75sFhU0WJZNBgkYhk26MKBA+hgRwFtGMJNPAEtLzRbVZvjUEqHoUPOInBDegg5D5nFLXVM0+6AcWh8lMBPmpkQ4UUQU3L4wt+XuyZJbtiz2Qtg5NDieSq98yvbj9iSQAhMkGV6jh2jG5KJXImYFrsJgpiykZ0AB2NIQ1Auensjql1pp2+5UdSvxCtmft7IqWBUpPA052zrRdrmflfrZOgf+OmPIwThJDNP/ITYWFkZaFYfS6BoZhooExyvavFhlRShjq6LARn8eRlaF5WnGqlen9VqlXyOArkmJySMnHINamRO1InDcLII3kmr+TNeDJejHfjY966YuQzR+SPjM8fCU6ZVw==</latexit>

A1
<latexit sha1_base64="ADVtdyO6jXWrvZhxoGqrsaoIm2A=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseqF48V7Qe0oWy2k3bpZhN2N0IJ/QlePCji1V/kzX/jts1Bqw8GHu/NMDMvSATXxnW/nMLK6tr6RnGztLW9s7tX3j9o6ThVDJssFrHqBFSj4BKbhhuBnUQhjQKB7WB8M/Pbj6g0j+WDmSToR3QoecgZNVa6v+p7/XLFrbpzkL/Ey0kFcjT65c/eIGZphNIwQbXuem5i/Iwqw5nAaamXakwoG9Mhdi2VNELtZ/NTp+TEKgMSxsqWNGSu/pzIaKT1JApsZ0TNSC97M/E/r5ua8NLPuExSg5ItFoWpICYms7/JgCtkRkwsoUxxeythI6ooMzadkg3BW375L2mdVb1atXZ3Xqlf53EU4QiO4RQ8uIA63EIDmsBgCE/wAq+OcJ6dN+d90Vpw8plD+AXn4xu7aY10</latexit>

<latexit sha1_base64="vA2oAzeUy/yybn06paxvOiXpoWQ=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FXQvQY8OIxonlAsoTZSScZMju7zMwKYcknePGgiFe/yJt/4yTZgyYWNBRV3XR3BbHg2rjut5Pb2Nza3snvFvb2Dw6PiscnLR0limGTRSJSnYBqFFxi03AjsBMrpGEgsB1Mbud++wmV5pF8NNMY/ZCOJB9yRo2VHsr0sl8suRV3AbJOvIyUIEOjX/zqDSKWhCgNE1TrrufGxk+pMpwJnBV6icaYsgkdYddSSUPUfro4dUYurDIgw0jZkoYs1N8TKQ21noaB7QypGetVby7+53UTM7zxUy7jxKBky0XDRBATkfnfZMAVMiOmllCmuL2VsDFVlBmbTsGG4K2+vE5aVxWvVqndV0v1ahZHHs7gHMrgwTXU4Q4a0AQGI3iGV3hzhPPivDsfy9ack82cwh84nz+IB41H</latexit>

(a)
<latexit sha1_base64="iX/q7mz8zuTzK8Y26A4wWQuQRtw=">AAAB83icbVBNS8NAEJ3Ur1q/qh69LBahooREpHosePHYgv2ANJTNdtMu3WzC7qZQSv+GFw+KePXPePPfuGlz0NYHA4/3ZpiZFyScKe0431ZhY3Nre6e4W9rbPzg8Kh+ftFWcSkJbJOax7AZYUc4EbWmmOe0mkuIo4LQTjB8yvzOhUrFYPOlpQv0IDwULGcHaSL2mx67ca1KdXPqlfrni2M4CaJ24OalAjka//NUbxCSNqNCEY6U810m0P8NSM8LpvNRLFU0wGeMh9QwVOKLKny1unqMLowxQGEtTQqOF+ntihiOlplFgOiOsR2rVy8T/PC/V4b0/YyJJNRVkuShMOdIxygJAAyYp0XxqCCaSmVsRGWGJiTYxZSG4qy+vk/aN7dbsWvO2UrfzOIpwBudQBRfuoA6P0IAWEEjgGV7hzUqtF+vd+li2Fqx85hT+wPr8AalEkBc=</latexit>

Q[i + 1, c(v)]
ω
<latexit sha1_base64="/CLBYOuRKGJEcXnAEPu7JgR+BjM=">AAAB7HicbVBNS8NAEJ2tX7V+VT16WSyCp5CIVI8FLx4rmLbQhrLZbtqlm03Y3Qgl9Dd48aCIV3+QN/+NmzYHbX0w8Hhvhpl5YSq4Nq77jSobm1vbO9Xd2t7+weFR/fiko5NMUebTRCSqFxLNBJfMN9wI1ksVI3EoWDec3hV+94kpzRP5aGYpC2IyljzilBgr+YN0wmvDesN13AXwOvFK0oAS7WH9azBKaBYzaaggWvc9NzVBTpThVLB5bZBplhI6JWPWt1SSmOkgXxw7xxdWGeEoUbakwQv190ROYq1ncWg7Y2ImetUrxP+8fmai2yDnMs0Mk3S5KMoENgkuPscjrhg1YmYJoYrbWzGdEEWosfkUIXirL6+TzpXjNZ3mw3Wj5ZRxVOEMzuESPLiBFtxDG3ygwOEZXuENSfSC3tHHsrWCyplT+AP0+QNGxY5K</latexit>

<latexit sha1_base64="j2pjInKa82ZWirDw6V465jgtueM=">AAAB7nicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy2m3bpZhN2J0Ip/RFePCji1d/jzX/jps1BWx8MPN6bYWZemEph0PO+ndLG5tb2Tnm3srd/cHhUPT5pmyTTjLdYIhPdDanhUijeQoGSd1PNaRxK3gknd7nfeeLaiEQ94jTlQUxHSkSCUbRSp49jjrQyqNY811uArBO/IDUo0BxUv/rDhGUxV8gkNabneykGM6pRMMnnlX5meErZhI54z1JFY26C2eLcObmwypBEibalkCzU3xMzGhszjUPbGVMcm1UvF//zehlGt8FMqDRDrthyUZRJggnJfydDoTlDObWEMi3srYSNqaYMbUJ5CP7qy+ukfeX6dbf+cF1ruEUcZTiDc7gEH26gAffQhBYwmMAzvMKbkzovzrvzsWwtOcXMKfyB8/kD2A2PMw==</latexit>

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

<latexit sha1_base64="hkWheshLgzpdKLr+1wri01A8e88=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5xGvTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wdwDY3M</latexit>

l(v)

<latexit sha1_base64="CtEGMfRC/HVlqXvlzCqR3zid6XA=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5RGrTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wdiTo3D</latexit>

i
<latexit sha1_base64="r/ihzd0O5x9LBh+ZxNjcjw2xGyo=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF49V7Ae0oWy2m3bpZhN2J0Ip/QdePCji1X/kzX/jps1Bqw8GHu/NMDMvTKUw6HlfTmltfWNzq7xd2dnd2z+oHh61TZJpxlsskYnuhtRwKRRvoUDJu6nmNA4l74STm9zvPHJtRKIecJryIKYjJSLBKFrpXlQG1ZrneguQv8QvSA0KNAfVz/4wYVnMFTJJjen5XorBjGoUTPJ5pZ8ZnlI2oSPes1TRmJtgtrh0Ts6sMiRRom0pJAv158SMxsZM49B2xhTHZtXLxf+8XobRdTATKs2QK7ZcFGWSYELyt8lQaM5QTi2hTAt7K2FjqilDG04egr/68l/SvnD9ulu/u6w13CKOMpzAKZyDD1fQgFtoQgsYRPAEL/DqTJxn5815X7aWnGLmGH7B+fgGAY+M+A==</latexit>

<latexit sha1_base64="F7MShkZwzQaV3wW7H7B/OPRA0J4=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5pGrTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wd5N43S</latexit>

c(v)

<latexit sha1_base64="eUkQfcAyAzdEzbJEiZvHDTx2POA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF49V7Ae0oWy2m3btZjfsboQS+g+8eFDEq//Im//GTZuDtj4YeLw3w8y8MOFMG8/7dkpr6xubW+Xtys7u3v5B9fCorWWqCG0RyaXqhlhTzgRtGWY47SaK4jjktBNObnK/80SVZlI8mGlCgxiPBIsYwcZK94+VQbXmud4caJX4BalBgeag+tUfSpLGVBjCsdY930tMkGFlGOF0VumnmiaYTPCI9iwVOKY6yOaXztCZVYYoksqWMGiu/p7IcKz1NA5tZ4zNWC97ufif10tNdB1kTCSpoYIsFkUpR0ai/G00ZIoSw6eWYKKYvRWRMVaYGBtOHoK//PIqaV+4ft2t313WGm4RRxlO4BTOwYcraMAtNKEFBCJ4hld4cybOi/PufCxaS04xcwx/4Hz+AAMUjPk=</latexit>

j

r(v)

...

ω

<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

<latexit sha1_base64="iX/q7mz8zuTzK8Y26A4wWQuQRtw=">AAAB83icbVBNS8NAEJ3Ur1q/qh69LBahooREpHosePHYgv2ANJTNdtMu3WzC7qZQSv+GFw+KePXPePPfuGlz0NYHA4/3ZpiZFyScKe0431ZhY3Nre6e4W9rbPzg8Kh+ftFWcSkJbJOax7AZYUc4EbWmmOe0mkuIo4LQTjB8yvzOhUrFYPOlpQv0IDwULGcHaSL2mx67ca1KdXPqlfrni2M4CaJ24OalAjka//NUbxCSNqNCEY6U810m0P8NSM8LpvNRLFU0wGeMh9QwVOKLKny1unqMLowxQGEtTQqOF+ntihiOlplFgOiOsR2rVy8T/PC/V4b0/YyJJNRVkuShMOdIxygJAAyYp0XxqCCaSmVsRGWGJiTYxZSG4qy+vk/aN7dbsWvO2UrfzOIpwBudQBRfuoA6P0IAWEEjgGV7hzUqtF+vd+li2Fqx85hT+wPr8AalEkBc=</latexit>

Q[i + 1, c(v)]

Q[c(v) + 1, j → 1]
<latexit sha1_base64="bWU+IFrU9Rk2VKwYSSsYP1NnCmM=">AAAB9XicbVDLSgNBEOyNrxhfUY9eBoMQUZddkegx4MVjAuYByRpmJ7PJmNkHM7ORsOQ/vHhQxKv/4s2/cTbZgyYWNBRV3XR3uRFnUlnWt5FbWV1b38hvFra2d3b3ivsHTRnGgtAGCXko2i6WlLOANhRTnLYjQbHvctpyR7ep3xpTIVkY3KtJRB0fDwLmMYKVlh7qHVIen57Z548XtlPoFUuWac2AlomdkRJkqPWKX91+SGKfBopwLGXHtiLlJFgoRjidFrqxpBEmIzygHU0D7FPpJLOrp+hEK33khUJXoNBM/T2RYF/Kie/qTh+roVz0UvE/rxMr78ZJWBDFigZkvsiLOVIhSiNAfSYoUXyiCSaC6VsRGWKBidJBpSHYiy8vk+alaVfMSv2qVDWzOPJwBMdQBhuuoQp3UIMGEBDwDK/wZjwZL8a78TFvzRnZzCH8gfH5A4nxkIo=</latexit>

<latexit

sha1_base64="bWU+IFrU9Rk2VKwYSSsYP1NnCmM=">AAAB9XicbVDLSgNBEOyNrxhfUY9eBoMQUZddkegx4MVjAuYByRpmJ7PJmNkHM7ORsOQ/vHhQxKv/4s2/cTbZgyYWNBRV3XR3uRFnUlnWt5FbWV1b38hvFra2d3b3ivsHTRnGgtAGCXko2i6WlLOANhRTnLYjQbHvctpyR7ep3xpTIVkY3KtJRB0fDwLmMYKVlh7qHVIen57Z548XtlPoFUuWac2AlomdkRJkqPWKX91+SGKfBopwLGXHtiLlJFgoRjidFrqxpBEmIzygHU0D7FPpJLOrp+hEK33khUJXoNBM/T2RYF/Kie/qTh+roVz0UvE/rxMr78ZJWBDFigZkvsiLOVIhSiNAfSYoUXyiCSaC6VsRGWKBidJBpSHYiy8vk+alaVfMSv2qVDWzOPJwBMdQBhuuoQp3UIMGEBDwDK/wZjwZL8a78TFvzRnZzCH8gfH5A4nxkIo=</latexit>

Q[c(v )

+ 1, j →

1]

<latexit sha1_base64="XUUCbJWA7SdkkjKlIFeo6DX50TU=">AAACCHicbZDLSsNAFIYnXmu9RV26MFiEChISkeqy4MZlBXuBNpTJdNIMnUzCzEmhhC7d+CpuXCji1kdw59s4abPQ1h8GPv5zDnPO7yecKXCcb2NldW19Y7O0Vd7e2d3bNw8OWypOJaFNEvNYdnysKGeCNoEBp51EUhz5nLb90W1eb4+pVCwWDzBJqBfhoWABIxi01TdPehGGUAWZZMMQNJNQAQaqptXxBTsv982KYzszWcvgFlBBhRp986s3iEkaUQGEY6W6rpOAl2EJjHA6LfdSRRNMRnhIuxoFjqjystkhU+tMOwMriKV+AqyZ+3siw5FSk8jXnbO1F2u5+V+tm0Jw42VMJClQQeYfBSm3ILbyVKwBk5QAn2jARDK9q0VCLDEBnV0egrt48jK0Lm23Ztfuryp1u4ijhI7RKaoiF12jOrpDDdREBD2iZ/SK3own48V4Nz7mrStGMXOE/sj4/AHqDJnU</latexit>

rightmatchstates(v, i)

Figure 2: (a) The substrings of a dyadic interval that are matched in the automaton A1 to find
leftmatchstates(v, i) and rightmatchstates(v, j) for some choice of i and j. (b) The left match states
of v from i are the states in A1 that can be reached from θ by a path that matches Q[i, c(v)]. The
right match states of v from i are the states from which ϕ can be reached by a path matching
Q[c(v) + 1, j].
Let v be a node in T . For an integer i ∈ [l(v), c(v)] and an integer j ∈ [c(v) + 1, r(v)] we define
the sets of left match states and right match states
leftmatchstates(v, i) = {s ∈ A | there is a path from θ to s matching Q[i + 1, c(v)]}
rightmatchstates(v, j) = {s ∈ A | there is a path from s to ϕ matching Q[c(v) + 1, j − 1]}
Note that if i = c(v) then leftmatchstates(v, i) is the set of states s such that there is a path from
θ to s matching the empty string, and similarly for rightmatchstates(v, c(v) + 1).
Each node v in the interval tree stores the following information:
• The index of the left and right ends of the interval of v, denoted l(v) and r(v), respectively,
and the middle of the interval rounded down, denoted c(v), where c(v) = ⌊(l(v) + r(v))/2⌋.
• The left match states of v for each i ∈ [l(v), c(v)], and the right match states of v for each
j ∈ [c(v) + 1, r(v)]. The sets are stored as bit vectors with an entry for each state in A, which
has a 1 if the state is in the set, and a 0 otherwise.
Lemma 3. Given string Q of length n and regular expression e of length m, the interval tree T of
Q for e can be computed in O(n2 m) time and stored in O(nm) space.
Proof. We first construct the binary tree together with the information r(v), l(v) and c(v) for each
node v. To compute the left and right match sets for each node we first construct the TNFA A and
←
−
its reverse A .
An ancestor u of a node v is a left ancestor of v if v is in the right subtree of u. A right ancestor
is defined symmetrically. For each index i in Q, we compute leftmatchstates(v, i) for all nodes v that
are right ancestors of Q[i] using state-set transitions on A. To do so, we start matching Q[i, n − 1]
in A and whenever we have matched Q[c(v)] for a right ancestor v of Q[i], we store the current
state-set of A as leftmatchstates(v, i). The sets rightmatchstates(v, j) can be computed similarly
←
−
using state-set transitions on A and the reverse string QR .
We first analyze the time. The binary tree with r(v), l(v) and c(v) for each node can be
←
−
computed in O(n) time. Constructing A and A takes O(m) time. Using the state-set algorithm

5

<!-- page 6 -->

<latexit sha1_base64="Q2WJRKWL8I52pbiFoMoP1ATUqkk=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FXJIqngBePEc0DkiXMTnqTIbOzy8ysEEI+wYsHRbz6Rd78GyfJHjSxoKGo6qa7K0gE18Z1v53c2vrG5lZ+u7Czu7d/UDw8auo4VQwbLBaxagdUo+ASG4Ybge1EIY0Cga1gdDvzW0+oNI/loxkn6Ed0IHnIGTVWeigH571iya24c5BV4mWkBBnqveJXtx+zNEJpmKBadzw3Mf6EKsOZwGmhm2pMKBvRAXYslTRC7U/mp07JmVX6JIyVLWnIXP09MaGR1uMosJ0RNUO97M3E/7xOasJrf8JlkhqUbLEoTAUxMZn9TfpcITNibAllittbCRtSRZmx6RRsCN7yy6ukeVHxqpXq/WWpdpPFkYcTOIUyeHAFNbiDOjSAwQCe4RXeHOG8OO/Ox6I152Qzx/AHzucPi1qNTg==</latexit>

(b)

<latexit sha1_base64="ywRbGZsPqnBGmrboLJ1epPNCZVM=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FXJIqngBePEc0DkiXMTnqTIbOzy8ysEEI+wYsHRbz6Rd78GyfJHjSxoKGo6qa7K0gE18Z1v53c2vrG5lZ+u7Czu7d/UDw8auo4VQwbLBaxagdUo+ASG4Ybge1EIY0Cga1gdDvzW0+oNI/loxkn6Ed0IHnIGTVWeijT816x5FbcOcgq8TJSggz1XvGr249ZGqE0TFCtO56bGH9CleFM4LTQTTUmlI3oADuWShqh9ifzU6fkzCp9EsbKljRkrv6emNBI63EU2M6ImqFe9mbif14nNeG1P+EySQ1KtlgUpoKYmMz+Jn2ukBkxtoQyxe2thA2poszYdAo2BG/55VXSvKh41Ur1/rJUu8niyMMJnEIZPLiCGtxBHRrAYADP8ApvjnBenHfnY9Gac7KZY/gD5/MHidWNTQ==</latexit>

(a)

root
<latexit sha1_base64="A5TKUM0c6d7JNuHkjtyqEJx9mb8=">AAAB7HicbVBNS8NAEJ3Ur1q/qh69BIvgqSQiVTwVvHisYNpCG8pmu2mXbnbD7kQopb/BiwdFvPqDvPlv3LQ5aPXBwOO9GWbmRangBj3vyymtrW9sbpW3Kzu7e/sH1cOjtlGZpiygSijdjYhhgksWIEfBuqlmJIkE60ST29zvPDJtuJIPOE1ZmJCR5DGnBK0UaKWwMqjWvLq3gPuX+AWpQYHWoPrZHyqaJUwiFcSYnu+lGM6IRk4Fm1f6mWEpoRMyYj1LJUmYCWeLY+fumVWGbqy0LYnuQv05MSOJMdMksp0JwbFZ9XLxP6+XYXwdzrhMM2SSLhfFmXBRufnn7pBrRlFMLSFUc3urS8dEE4o2nzwEf/Xlv6R9Ufcb9cb9Za15U8RRhhM4hXPw4QqacActCIAChyd4gVdHOs/Om/O+bC05xcwx/ILz8Q2F6459</latexit>

ω
<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

u

a
<latexit sha1_base64="AWwWPMOu9bjMxMVLoIEfbDJjcNY=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1rqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Afj0jPw=</latexit>

<latexit sha1_base64="KQLoQLjzd8rygJrVK6NY+DbftLY=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYxX5AG8pmu2mXbjZhdyKU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYkUBl332ymsrW9sbhW3Szu7e/sH5cOjlolTzXiTxTLWnYAaLoXiTRQoeSfRnEaB5O1gfJv57SeujYjVI04S7kd0qEQoGEUrPaSlfrniVt05yCrxclKBHI1++as3iFkacYVMUmO6npugP6UaBZN8VuqlhieUjemQdy1VNOLGn84vnZEzqwxIGGtbCslc/T0xpZExkyiwnRHFkVn2MvE/r5tieO1PhUpS5IotFoWpJBiT7G0yEJozlBNLKNPC3krYiGrK0IaTheAtv7xKWhdVr1at3V9W6jd5HEU4gVM4Bw+uoA530IAmMAjhGV7hzRk7L86787FoLTj5zDH8gfP5AxdnjRA=</latexit>

ω
a
<latexit sha1_base64="AWwWPMOu9bjMxMVLoIEfbDJjcNY=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1rqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Afj0jPw=</latexit>

ω
<latexit sha1_base64="ws8kkb9T/i5LPoLpnQmUDZKxjLw=">AAAB7nicbVBNS8NAEN3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZhdyKU0B/hxYMiXv093vw3btoctPpg4PHeDDPzgkQKg6775ZTW1jc2t8rblZ3dvf2D6uFRx8Sp5tDmsYx1L2AGpFDQRoESeokGFgUSusH0Nve7j6CNiNUDzhLwIzZWIhScoZW6A5wAssqwWnPr7gL0L/EKUiMFWsPq52AU8zQChVwyY/qem6CfMY2CS5hXBqmBhPEpG0PfUsUiMH62OHdOz6wyomGsbSmkC/XnRMYiY2ZRYDsjhhOz6uXif14/xfDaz4RKUgTFl4vCVFKMaf47HQkNHOXMEsa1sLdSPmGacbQJ5SF4qy//JZ2LuteoN+4va82bIo4yOSGn5Jx45Io0yR1pkTbhZEqeyAt5dRLn2Xlz3petJaeYOSa/4Hx8A9upjz8=</latexit>

ω
<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

s1
<latexit sha1_base64="X6B9jVpTexGOAMXxJjucPI+KAQE=">AAAB7HicbVBNS8NAEJ2tXzV+VT16WSyCp5KIVPFU8OKxgmkLbSib7aZdutmE3Y1QQn+DFw+KePUHefPfuGlz0OqDgcd7M8zMC1PBtXHdL1RZW9/Y3KpuOzu7e/sHtcOjjk4yRZlPE5GoXkg0E1wy33AjWC9VjMShYN1welv43UemNE/kg5mlLIjJWPKIU2Ks5Ouh5zjDWt1tuAvgv8QrSR1KtIe1z8EooVnMpKGCaN333NQEOVGGU8HmziDTLCV0Ssasb6kkMdNBvjh2js+sMsJRomxJgxfqz4mcxFrP4tB2xsRM9KpXiP95/cxE10HOZZoZJulyUZQJbBJcfI5HXDFqxMwSQhW3t2I6IYpQY/MpQvBWX/5LOhcNr9lo3l/WWzdlHFU4gVM4Bw+uoAV30AYfKHB4ghd4RRI9ozf0vmytoHLmGH4BfXwDb26Nxg==</latexit>

s2

ω
s3
<latexit sha1_base64="8KrxpkSf/CC+Kyjk8vbeGUEJTig=">AAAB7HicbVBNS8NAEJ3Urxq/qh69LBbBU0lUqngqePFYwbSFNpTNdtMu3WzC7kYoob/BiwdFvPqDvPlv3KQ5aOuDgcd7M8zMCxLOlHacb6uytr6xuVXdtnd29/YPaodHHRWnklCPxDyWvQArypmgnmaa014iKY4CTrvB9C73u09UKhaLRz1LqB/hsWAhI1gbyVPDS9se1upOwymAVolbkjqUaA9rX4NRTNKICk04VqrvOon2Myw1I5zO7UGqaILJFI9p31CBI6r8rDh2js6MMkJhLE0JjQr190SGI6VmUWA6I6wnatnLxf+8fqrDGz9jIkk1FWSxKEw50jHKP0cjJinRfGYIJpKZWxGZYImJNvnkIbjLL6+SzkXDbTaaD1f11m0ZRxVO4BTOwYVraME9tMEDAgye4RXeLGG9WO/Wx6K1YpUzx/AH1ucPcnqNyA==</latexit>

c
<latexit sha1_base64="Q8+PsEdC5G0waUhEsipQCo5E2dw=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolIFU8FLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlBuuXym7FnYOsEi8nZchR75e+eoOYpRFKwwTVuuu5ifEzqgxnAqfFXqoxoWxMh9i1VNIItZ/ND52Sc6sMSBgrW9KQufp7IqOR1pMosJ0RNSO97M3E/7xuasIbP+MySQ1KtlgUpoKYmMy+JgOukBkxsYQyxe2thI2ooszYbIo2BG/55VXSuqx41Uq1cVWu3eZxFOAUzuACPLiGGtxDHZrAAOEZXuHNeXRenHfnY9G65uQzJ/AHzucPx4+M6g==</latexit>

ω
<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

b
<latexit sha1_base64="K2XpTScscLVACkIdcirXXQsiVG0=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYxX5AG8pmu2mXbjZhdyKU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYkUBl332ymsrW9sbhW3Szu7e/sH5cOjlolTzXiTxTLWnYAaLoXiTRQoeSfRnEaB5O1gfJv57SeujYjVI04S7kd0qEQoGEUrPQSlfrniVt05yCrxclKBHI1++as3iFkacYVMUmO6npugP6UaBZN8VuqlhieUjemQdy1VNOLGn84vnZEzqwxIGGtbCslc/T0xpZExkyiwnRHFkVn2MvE/r5tieO1PhUpS5IotFoWpJBiT7G0yEJozlBNLKNPC3krYiGrK0IaTheAtv7xKWhdVr1at3V9W6jd5HEU4gVM4Bw+uoA530IAmMAjhGV7hzRk7L86787FoLTj5zDH8gfP5A/p5jP0=</latexit>

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

<latexit sha1_base64="OmBp/3Myla63c8HCn4Mk1Fb80Hw=">AAAB7HicbVBNS8NAEJ3Urxq/oh69LBbBU0mKVPFU8OKxgmkLbSib7aZdutmE3Y1QQn+DFw+KePUHefPfuGlz0NYHA4/3ZpiZF6acKe2631ZlY3Nre6e6a+/tHxweOccnHZVkklCfJDyRvRArypmgvmaa014qKY5DTrvh9K7wu09UKpaIRz1LaRDjsWARI1gbyVfDhm0PnZpbdxdA68QrSQ1KtIfO12CUkCymQhOOlep7bqqDHEvNCKdze5ApmmIyxWPaN1TgmKogXxw7RxdGGaEokaaERgv190SOY6VmcWg6Y6wnatUrxP+8fqajmyBnIs00FWS5KMo40gkqPkcjJinRfGYIJpKZWxGZYImJNvkUIXirL6+TTqPuNevNh6ta67aMowpncA6X4ME1tOAe2uADAQbP8ApvlrBerHfrY9lascqZU/gD6/MHcPSNxw==</latexit>

s7
<latexit sha1_base64="HkxXCs5xrAiUlnPfSxS8zUTA16M=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEWvFU8OKxgv2ANpTNdtou3d2E3Y1QQv+CFw+KePUPefPfmLQ5aOuDgcd7M8zMCyLBjXXdb6ewsbm1vVPcLe3tHxwelY9P2iaMNcMWC0WouwE1KLjCluVWYDfSSGUgsBNM7zK/84Ta8FA92lmEvqRjxUecUZtJZlAvDcoVt+ouQNaJl5MK5GgOyl/9YchiicoyQY3peW5k/YRqy5nAeakfG4wom9Ix9lKqqETjJ4tb5+QiVYZkFOq0lCUL9fdEQqUxMxmknZLaiVn1MvE/rxfb0Y2fcBXFFhVbLhrFgtiQZI+TIdfIrJilhDLN01sJm1BNmU3jyULwVl9eJ+2rqler1h6uK43bPI4inME5XIIHdWjAPTShBQwm8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwBDbY24</latexit>

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

s4
<latexit sha1_base64="HWNs7MLrNuHEX/JCJre+bFsyLBM=">AAAB7HicbVBNS8NAEJ3Urxq/oh69LBbBU0mkVPFU8OKxgmkLbSib7aZdutmE3Y1QQn+DFw+KePUHefPfuGlz0NYHA4/3ZpiZF6acKe2631ZlY3Nre6e6a+/tHxweOccnHZVkklCfJDyRvRArypmgvmaa014qKY5DTrvh9K7wu09UKpaIRz1LaRDjsWARI1gbyVfDhm0PnZpbdxdA68QrSQ1KtIfO12CUkCymQhOOlep7bqqDHEvNCKdze5ApmmIyxWPaN1TgmKogXxw7RxdGGaEokaaERgv190SOY6VmcWg6Y6wnatUrxP+8fqajmyBnIs00FWS5KMo40gkqPkcjJinRfGYIJpKZWxGZYImJNvkUIXirL6+TzlXda9abD41a67aMowpncA6X4ME1tOAe2uADAQbP8ApvlrBerHfrY9lascqZU/gD6/MHdACNyQ==</latexit>

ω

s8
<latexit sha1_base64="3lCBvLTmV8MWuGEE9etUfGONRH8=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEavFU8OKxgv2ANpTNdtou3d2E3Y1QQv+CFw+KePUPefPfmLQ5aOuDgcd7M8zMCyLBjXXdb6ewsbm1vVPcLe3tHxwelY9P2iaMNcMWC0WouwE1KLjCluVWYDfSSGUgsBNM7zK/84Ta8FA92lmEvqRjxUecUZtJZlAvDcoVt+ouQNaJl5MK5GgOyl/9YchiicoyQY3peW5k/YRqy5nAeakfG4wom9Ix9lKqqETjJ4tb5+QiVYZkFOq0lCUL9fdEQqUxMxmknZLaiVn1MvE/rxfbUd1PuIpii4otF41iQWxIssfJkGtkVsxSQpnm6a2ETaimzKbxZCF4qy+vk/ZV1atVaw/XlcZtHkcRzuAcLsGDG2jAPTShBQwm8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwBE8o25</latexit>

ω
<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

s5
<latexit sha1_base64="MBgsBpdSwoPu9MRqFypG6FeZuVU=">AAAB7HicbVBNS8NAEJ3Urxq/qh69LBbBU0lEq3gqePFYwbSFNpTNdtMu3WzC7kYoob/BiwdFvPqDvPlv3KQ5aOuDgcd7M8zMCxLOlHacb6uytr6xuVXdtnd29/YPaodHHRWnklCPxDyWvQArypmgnmaa014iKY4CTrvB9C73u09UKhaLRz1LqB/hsWAhI1gbyVPDK9se1upOwymAVolbkjqUaA9rX4NRTNKICk04VqrvOon2Myw1I5zO7UGqaILJFI9p31CBI6r8rDh2js6MMkJhLE0JjQr190SGI6VmUWA6I6wnatnLxf+8fqrDGz9jIkk1FWSxKEw50jHKP0cjJinRfGYIJpKZWxGZYImJNvnkIbjLL6+SzkXDbTaaD5f11m0ZRxVO4BTOwYVraME9tMEDAgye4RXeLGG9WO/Wx6K1YpUzx/AH1ucPdYaNyg==</latexit>

s11 ω
<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

s6
<latexit sha1_base64="ePPj7+XtnUYsHpvzkAfDSRqNJwo=">AAAB7HicbVBNS8NAEJ2tXzV+VT16WSyCp5KIVPFU8OKxgmkLbSib7aZdutmE3Y1QQn+DFw+KePUHefPfuGlz0OqDgcd7M8zMC1PBtXHdL1RZW9/Y3KpuOzu7e/sHtcOjjk4yRZlPE5GoXkg0E1wy33AjWC9VjMShYN1welv43UemNE/kg5mlLIjJWPKIU2Ks5Oth03GGtbrbcBfAf4lXkjqUaA9rn4NRQrOYSUMF0brvuakJcqIMp4LNnUGmWUrolIxZ31JJYqaDfHHsHJ9ZZYSjRNmSBi/UnxM5ibWexaHtjImZ6FWvEP/z+pmJroOcyzQzTNLloigT2CS4+ByPuGLUiJklhCpub8V0QhShxuZThOCtvvyXdC4aXrPRvL+st27KOKpwAqdwDh5cQQvuoA0+UODwBC/wiiR6Rm/ofdlaQeXMMfwC+vgGdwyNyw==</latexit>

<latexit sha1_base64="SZDnNSOws0J4jJ+bkEP8QZHnB44=">AAAB7nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmu2mXbjZhdyKU0B/hxYMiXv093vw3btoctPpg4PHeDDPzgkQKg6775ZTW1jc2t8rblZ3dvf2D6uFRx8SpZrzNYhnrXkANl0LxNgqUvJdoTqNA8m4wvc397iPXRsTqAWcJ9yM6ViIUjKKVumaYed68MqzW3Lq7APlLvILUoEBrWP0cjGKWRlwhk9SYvucm6GdUo2CSzyuD1PCEsikd876likbc+Nni3Dk5s8qIhLG2pZAs1J8TGY2MmUWB7YwoTsyql4v/ef0Uw2s/EypJkSu2XBSmkmBM8t/JSGjOUM4soUwLeythE6opQ5tQHoK3+vJf0rmoe4164/6y1rwp4ijDCZzCOXhwBU24gxa0gcEUnuAFXp3EeXbenPdla8kpZo7hF5yPb3D4jvk=</latexit>

ω
<latexit sha1_base64="3SSJOTybfTtDwh3YySbChNlxgrc=">AAAB63icbVDLSgNBEOyNrxhfUY9eBoPgKeyKxMcp4MVjBPOAZAmzk95kyMzsMjMrhJBf8OJBEa/+kDf/xt1kD5pY0FBUddPdFcSCG+u6305hbX1jc6u4XdrZ3ds/KB8etUyUaIZNFolIdwJqUHCFTcutwE6skcpAYDsY32V++wm14ZF6tJMYfUmHioecUZtJpn9T6pcrbtWdg6wSLycVyNHol796g4glEpVlghrT9dzY+lOqLWcCZ6VeYjCmbEyH2E2pohKNP53fOiNnqTIgYaTTUpbM1d8TUyqNmcgg7ZTUjsyyl4n/ed3Ehtf+lKs4sajYYlGYCGIjkj1OBlwjs2KSEso0T28lbEQ1ZTaNJwvBW355lbQuql6tWnu4rNRv8ziKcAKncA4eXEEd7qEBTWAwgmd4hTdHOi/Ou/OxaC04+cwx/IHz+QNGd426</latexit>

c

ω

<latexit sha1_base64="SKu5/BFEibyNf8You4bKpoIY8Kw=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEWvFU8OKxgv2ANpTNdtMs3d2E3Y1QQv+CFw+KePUPefPfuGlz0NYHA4/3ZpiZFyScaeO6305pY3Nre6e8W9nbPzg8qh6fdHWcKkI7JOax6gdYU84k7RhmOO0nimIRcNoLpne533uiSrNYPppZQn2BJ5KFjGCTS8MkYqNqza27C6B14hWkBgXao+rXcByTVFBpCMdaDzw3MX6GlWGE03llmGqaYDLFEzqwVGJBtZ8tbp2jC6uMURgrW9Kghfp7IsNC65kIbKfAJtKrXi7+5w1SE974GZNJaqgky0VhypGJUf44GjNFieEzSzBRzN6KSIQVJsbGU7EheKsvr5PuVd1r1BsP17XWbRFHGc7gHC7Bgya04B7a0AECETzDK7w5wnlx3p2PZWvJKWZO4Q+czx8Uso5C</latexit>

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

<latexit sha1_base64="Q8+PsEdC5G0waUhEsipQCo5E2dw=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolIFU8FLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlBuuXym7FnYOsEi8nZchR75e+eoOYpRFKwwTVuuu5ifEzqgxnAqfFXqoxoWxMh9i1VNIItZ/ND52Sc6sMSBgrW9KQufp7IqOR1pMosJ0RNSO97M3E/7xuasIbP+MySQ1KtlgUpoKYmMy+JgOukBkxsYQyxe2thI2ooszYbIo2BG/55VXSuqx41Uq1cVWu3eZxFOAUzuACPLiGGtxDHZrAAOEZXuHNeXRenHfnY9G65uQzJ/AHzucPx4+M6g==</latexit>

s9

ω

ω

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

s10
<latexit sha1_base64="2P1gNR6azetuzFM1OQ9bwL6Z4lA=">AAAB7nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZhdyOU0B/hxYMiXv093vw3btoctPpg4PHeDDPzgkRwbVz3yymtrW9sbpW3Kzu7e/sH1cOjjo5TxbDNYhGrXkA1Ci6xbbgR2EsU0igQ2A2mt7nffUSleSwfzCxBP6JjyUPOqLFSVw8zz51XhtWaW3cXIH+JV5AaFGgNq5+DUczSCKVhgmrd99zE+BlVhjOB88og1ZhQNqVj7FsqaYTazxbnzsmZVUYkjJUtachC/TmR0UjrWRTYzoiaiV71cvE/r5+a8NrPuExSg5ItF4WpICYm+e9kxBUyI2aWUKa4vZWwCVWUGZtQHoK3+vJf0rmoe4164/6y1rwp4ijDCZzCOXhwBU24gxa0gcEUnuAFXp3EeXbenPdla8kpZo7hF5yPb29yjvg=</latexit>

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

a b c b c b a c b c c b b a b c
<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

ω

<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

<latexit sha1_base64="eV6So4eaFH64d5dlkyoZvDaCodY=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlRtAvld2KOwdZJV5OypCj3i999QYxSyOUhgmqdddzE+NnVBnOBE6LvVRjQtmYDrFrqaQRaj+bHzol51YZkDBWtqQhc/X3REYjrSdRYDsjakZ62ZuJ/3nd1IQ3fsZlkhqUbLEoTAUxMZl9TQZcITNiYgllittbCRtRRZmx2RRtCN7yy6ukdVnxqpVq46pcu83jKMApnMEFeHANNbiHOjSBAcIzvMKb8+i8OO/Ox6J1zclnTuAPnM8fxz+M7Q==</latexit>

<latexit sha1_base64="wWUA6WYXv8205leegQComW6xQgc=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlBu2Xym7FnYOsEi8nZchR75e+eoOYpRFKwwTVuuu5ifEzqgxnAqfFXqoxoWxMh9i1VNIItZ/ND52Sc6sMSBgrW9KQufp7IqOR1pMosJ0RNSO97M3E/7xuasIbP+MySQ1KtlgUpoKYmMy+JgOukBkxsYQyxe2thI2ooszYbIo2BG/55VXSuqx41Uq1cVWu3eZxFOAUzuACPLiGGtxDHZrAAOEZXuHNeXRenHfnY9G65uQzJ/AHzucPxbuM7A==</latexit>

<latexit sha1_base64="n8jLvSu1XmO2P3s3VxHAWx05qSs=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqicpePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1bqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Af0yjQI=</latexit>

<latexit sha1_base64="uGH2gGosOXh7LW4EbvEqKly0tiQ=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqngqePFYwX5AG8pmO2mXbjZxdyOU0D/hxYMiXv073vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKnR4mmotY9ssVt+rOQVaJl5MK5Gj0y1+9QczSCKVhgmrd9dzE+BlVhjOB01Iv1ZhQNqZD7FoqaYTaz+b3TsmZVQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14bWfcZmkBiVbLApTQUxMZs+TAVfIjJhYQpni9lbCRlRRZmxEJRuCt/zyKmldVL1atXZ/Wanf5HEU4QRO4Rw8uII63EEDmsBAwDO8wpvz6Lw4787HorXg5DPH8AfO5w9N+5Aj</latexit>

<latexit sha1_base64="wWUA6WYXv8205leegQComW6xQgc=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlBu2Xym7FnYOsEi8nZchR75e+eoOYpRFKwwTVuuu5ifEzqgxnAqfFXqoxoWxMh9i1VNIItZ/ND52Sc6sMSBgrW9KQufp7IqOR1pMosJ0RNSO97M3E/7xuasIbP+MySQ1KtlgUpoKYmMy+JgOukBkxsYQyxe2thI2ooszYbIo2BG/55VXSuqx41Uq1cVWu3eZxFOAUzuACPLiGGtxDHZrAAOEZXuHNeXRenHfnY9G65uQzJ/AHzucPxbuM7A==</latexit>

<latexit sha1_base64="n8jLvSu1XmO2P3s3VxHAWx05qSs=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqicpePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1bqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Af0yjQI=</latexit>

<latexit sha1_base64="n8jLvSu1XmO2P3s3VxHAWx05qSs=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqicpePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1bqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Af0yjQI=</latexit>

<latexit sha1_base64="wWUA6WYXv8205leegQComW6xQgc=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolI9SQFLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777aytb2xubRd2irt7+weHpaPjlo5TxbDJYhGrTkA1Ci6xabgR2EkU0igQ2A7GdzO//YRK81g+mEmCfkSHkoecUWOlBu2Xym7FnYOsEi8nZchR75e+eoOYpRFKwwTVuuu5ifEzqgxnAqfFXqoxoWxMh9i1VNIItZ/ND52Sc6sMSBgrW9KQufp7IqOR1pMosJ0RNSO97M3E/7xuasIbP+MySQ1KtlgUpoKYmMy+JgOukBkxsYQyxe2thI2ooszYbIo2BG/55VXSuqx41Uq1cVWu3eZxFOAUzuACPLiGGtxDHZrAAOEZXuHNeXRenHfnY9G65uQzJ/AHzucPxbuM7A==</latexit>

<latexit sha1_base64="n8jLvSu1XmO2P3s3VxHAWx05qSs=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqicpePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1bqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Af0yjQI=</latexit>

<latexit sha1_base64="n8jLvSu1XmO2P3s3VxHAWx05qSs=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqicpePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1bqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Af0yjQI=</latexit>

<latexit sha1_base64="bvTxXJRdz4vci7DBfXvdup+sIa4=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2k3bpZhN2N0IJ/QdePCji1X/kzX/jps1Bqw8GHu/NMDMvSATXxnW/nNLa+sbmVnm7srO7t39QPTzq6DhVDNssFrHqBVSj4BLbhhuBvUQhjQKB3WB6k/vdR1Sax/LBzBL0IzqWPOSMGivdu5VhtebW3QXIX+IVpAYFWsPq52AUszRCaZigWvc9NzF+RpXhTOC8Mkg1JpRN6Rj7lkoaofazxaVzcmaVEQljZUsaslB/TmQ00noWBbYzomaiV71c/M/rpya89jMuk9SgZMtFYSqIiUn+NhlxhcyImSWUKW5vJWxCFWXGhpOH4K2+/Jd0Lupeo964u6w13SKOMpzAKZyDB1fQhFtoQRsYhPAEL/DqTJ1n5815X7aWnGLmGH7B+fgGq32MwQ==</latexit>

0

1
<latexit sha1_base64="we9w2M/P7Gx+67UUwa/wxaw6+0g=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1Fip6Q3KFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jppX1W9WrXWvK7U3TyOIpzBOVyCBzdQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AeMWMrg==</latexit>

<latexit sha1_base64="ocNNXV1D3xuZN0nRpZdR9iCnS88=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx6r2A9oQ9lsN+3SzSbsToRS+g+8eFDEq//Im//GTZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbjO/88S1EbF6xGnC/YiOlAgFo2ilh1ppUK64VXcBsk68nFQgR3NQ/uoPY5ZGXCGT1Jie5yboz6hGwSSfl/qp4QllEzriPUsVjbjxZ4tL5+TCKkMSxtqWQrJQf0/MaGTMNApsZ0RxbFa9TPzP66UY3vgzoZIUuWLLRWEqCcYke5sMheYM5dQSyrSwtxI2ppoytOFkIXirL6+Tdq3q1av1+6tKw83jKMIZnMMleHANDbiDJrSAQQjP8ApvzsR5cd6dj2VrwclnTuEPnM8froeMww==</latexit>

2

<latexit sha1_base64="pIvXK2GHHxtp+uOWg6RGJ9owjOE=">AAAB6HicbVBNS8NAEJ34WetX1aOXxSJ4KolK9Vjw4rEF+wFtKJvtpF272YTdjVBCf4EXD4p49Sd589+4bXPQ1gcDj/dmmJkXJIJr47rfztr6xubWdmGnuLu3f3BYOjpu6ThVDJssFrHqBFSj4BKbhhuBnUQhjQKB7WB8N/PbT6g0j+WDmSToR3QoecgZNVZqXPVLZbfizkFWiZeTMuSo90tfvUHM0gilYYJq3fXcxPgZVYYzgdNiL9WYUDamQ+xaKmmE2s/mh07JuVUGJIyVLWnIXP09kdFI60kU2M6ImpFe9mbif143NeGtn3GZpAYlWywKU0FMTGZfkwFXyIyYWEKZ4vZWwkZUUWZsNkUbgrf88ippXVa8aqXauC7X3DyOApzCGVyABzdQg3uoQxMYIDzDK7w5j86L8+58LFrXnHzmBP7A+fwBe82MsA==</latexit>

3

<latexit sha1_base64="ZjvSgrQ5BSoQSS3+c01JvYz1HKk=">AAAB5HicbVBNS8NAEJ3Urxq/qlcvi0XwVBKR1mPBi8cK9gPaUDbbSbt2swm7G6GE/gIvHhSv/iZv/hu3bQ7a+mDg8d4MM/PCVHBtPO/bKW1t7+zulffdg8Oj45OKe9rRSaYYtlkiEtULqUbBJbYNNwJ7qUIahwK74fRu4XefUWmeyEczSzGI6VjyiDNqrPRwM6xUvZq3BNkkfkGqUKA1rHwNRgnLYpSGCap13/dSE+RUGc4Ezt1BpjGlbErH2LdU0hh1kC8PnZNLq4xIlChb0pCl+nsip7HWszi0nTE1E73uLcT/vH5motsg5zLNDEq2WhRlgpiELL4mI66QGTGzhDLF7a2ETaiizNhsXBuCv/7yJulc1/x6rV5tekUYZTiHC7gCHxrQhHtoQRsYILzAG7w7T86r87FqLDnFxBn8gfP5AxLzi4c=</latexit>

4

<latexit sha1_base64="/+ICqTa0/VQmJNcdpjEDDA2CFMk=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEq8eCF49V7Ae0oWy2m3bpZhN2J0Ip/QdePCji1X/kzX/jps1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7Gt5nffuLaiFg94iThfkSHSoSCUbTSw1WpX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboT6lGwSSflXqp4QllYzrkXUsVjbjxp/NLZ+TMKgMSxtqWQjJXf09MaWTMJApsZ0RxZJa9TPzP66YY3vhToZIUuWKLRWEqCcYke5sMhOYM5cQSyrSwtxI2opoytOFkIXjLL6+S1kXVq1Vr95eVupvHUYQTOIVz8OAa6nAHDWgCgxCe4RXenLHz4rw7H4vWgpPPHMMfOJ8/sxaMxg==</latexit>

5

<latexit sha1_base64="9gtWz5aeDYWq7E8CPni878nK31k=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2m3bpZhN2J0IJ/QdePCji1X/kzX/jps1Bqw8GHu/NMDMvSKQw6LpfTmltfWNzq7xd2dnd2z+oHh51TJxqxtsslrHuBdRwKRRvo0DJe4nmNAok7wbTm9zvPnJtRKwecJZwP6JjJULBKFrpvlEZVmtu3V2A/CVeQWpQoDWsfg5GMUsjrpBJakzfcxP0M6pRMMnnlUFqeELZlI5531JFI278bHHpnJxZZUTCWNtSSBbqz4mMRsbMosB2RhQnZtXLxf+8forhtZ8JlaTIFVsuClNJMCb522QkNGcoZ5ZQpoW9lbAJ1ZShDScPwVt9+S/pXNS9Rr1xd1lrukUcZTiBUzgHD66gCbfQgjYwCOEJXuDVmTrPzpvzvmwtOcXMMfyC8/ENtJuMxw==</latexit>

6

<latexit sha1_base64="FkvdHsGpV65TNO0tZFyWj4zoZ0Y=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEWo8FLx6r2A9oQ9lsN+3SzSbsToQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbud+54lrI2L1iNOE+xEdKREKRtFKD/XSoFxxq+4CZJ14OalAjuag/NUfxiyNuEImqTE9z03Qz6hGwSSflfqp4QllEzriPUsVjbjxs8WlM3JhlSEJY21LIVmovycyGhkzjQLbGVEcm1VvLv7n9VIMb/xMqCRFrthyUZhKgjGZv02GQnOGcmoJZVrYWwkbU00Z2nDmIXirL6+T9lXVq1Vr99eVhpvHUYQzOIdL8KAODbiDJrSAQQjP8ApvzsR5cd6dj2VrwclnTuEPnM8ftiCMyA==</latexit>

7

<latexit sha1_base64="PXkB78kQCHYpf1zmA2dlaCKJ8S8=">AAAB63icbVBNS8NAEJ3Urxq/qh69LBbBU0lEao8FLx4r2FpoQ9lst+3S3U3YnQgl9C948aCIV/+QN/+NSZuDtj4YeLw3w8y8MJbCoud9O6WNza3tnfKuu7d/cHhUOT7p2CgxjLdZJCPTDanlUmjeRoGSd2PDqQolfwynt7n/+MSNFZF+wFnMA0XHWowEo5hLDdd1B5WqV/MWIOvEL0gVCrQGla/+MGKJ4hqZpNb2fC/GIKUGBZN87vYTy2PKpnTMexnVVHEbpItb5+QiU4ZkFJmsNJKF+nsipcramQqzTkVxYle9XPzP6yU4agSp0HGCXLPlolEiCUYkf5wMheEM5SwjlBmR3UrYhBrKMIsnD8FffXmddK5qfr1Wv7+uNr0ijjKcwTlcgg830IQ7aEEbGEzgGV7hzVHOi/PufCxbS04xcwp/4Hz+ACBgjPE=</latexit>

8

<latexit sha1_base64="4UtmbgnGxZv3W3NftH6v3ZCRxh8=">AAAB6nicbVBNS8NAEJ3Urxq/qh69LBbBU0lEqt4KXjxWtB/QhrLZbtqlu5uwuxFK6E/w4kERr/4ib/4bN20O2vpg4PHeDDPzwoQzbTzv2ymtrW9sbpW33Z3dvf2DyuFRW8epIrRFYh6rbog15UzSlmGG026iKBYhp51wcpv7nSeqNIvlo5kmNBB4JFnECDZWerhx3UGl6tW8OdAq8QtShQLNQeWrP4xJKqg0hGOte76XmCDDyjDC6cztp5ommEzwiPYslVhQHWTzU2fozCpDFMXKljRorv6eyLDQeipC2ymwGetlLxf/83qpia6DjMkkNVSSxaIo5cjEKP8bDZmixPCpJZgoZm9FZIwVJsamk4fgL7+8StoXNb9eq99fVhteEUcZTuAUzsGHK2jAHTShBQRG8Ayv8OZw58V5dz4WrSWnmDmGP3A+fwDte4ze</latexit>

9

<latexit sha1_base64="n8jLvSu1XmO2P3s3VxHAWx05qSs=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqicpePFYxX5AG8pmO2mXbjZhdyOU0n/gxYMiXv1H3vw3btoctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2MbzO//YRK81g+mkmCfkSHkoecUWOlB1bqlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxp1QZzgTOSr1UY0LZmA6xa6mkEWp/Or90Rs6sMiBhrGxJQ+bq74kpjbSeRIHtjKgZ6WUvE//zuqkJr/0pl0lqULLFojAVxMQke5sMuEJmxMQSyhS3txI2oooyY8PJQvCWX14lrYuqV6vW7i8r9Zs8jiKcwCmcgwdXUIc7aEATGITwDK/w5oydF+fd+Vi0Fpx85hj+wPn8Af0yjQI=</latexit>

10 11
<latexit sha1_base64="qUgfWjtgUmIWidmDetn7mN6g/6E=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEWo8FLx6r2A9oQ9lsJ+3SzSbsboQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpwXMH5YpbdRcg68TLSQVyNAflr/4wZmmE0jBBte55bmL8jCrDmcBZqZ9qTCib0BH2LJU0Qu1ni0tn5MIqQxLGypY0ZKH+nshopPU0CmxnRM1Yr3pz8T+vl5rwxs+4TFKDki0XhakgJibzt8mQK2RGTC2hTHF7K2FjqigzNpySDcFbfXmdtK+qXq1au7+uNNw8jiKcwTlcggd1aMAdNKEFDEJ4hld4cybOi/PufCxbC04+cwp/4Hz+AOaajOg=</latexit>

<latexit sha1_base64="TnqfOUJ8kLpeyInP2cVAXXLzHJc=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEWo8FLx6r2A9oQ9lsJ+3SzSbsboQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpwfMG5YpbdRcg68TLSQVyNAflr/4wZmmE0jBBte55bmL8jCrDmcBZqZ9qTCib0BH2LJU0Qu1ni0tn5MIqQxLGypY0ZKH+nshopPU0CmxnRM1Yr3pz8T+vl5rwxs+4TFKDki0XhakgJibzt8mQK2RGTC2hTHF7K2FjqigzNpySDcFbfXmdtK+qXq1au7+uNNw8jiKcwTlcggd1aMAdNKEFDEJ4hld4cybOi/PufCxbC04+cwp/4Hz+AOgejOk=</latexit>

12 13
<latexit sha1_base64="I8gUlFIGXfwaosYwqv97zcF+E4E=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx6r2A9oQ9lsN+3SzSbsToQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbud+54lrI2L1iNOE+xEdKREKRtFKD15tUK64VXcBsk68nFQgR3NQ/uoPY5ZGXCGT1Jie5yboZ1SjYJLPSv3U8ISyCR3xnqWKRtz42eLSGbmwypCEsbalkCzU3xMZjYyZRoHtjCiOzao3F//zeimGN34mVJIiV2y5KEwlwZjM3yZDoTlDObWEMi3srYSNqaYMbTglG4K3+vI6adeqXr1av7+qNNw8jiKcwTlcggfX0IA7aEILGITwDK/w5kycF+fd+Vi2Fpx85hT+wPn8AemijOo=</latexit>

<latexit sha1_base64="PdjQ2POQolA0WsSu+gOqu6NxjZg=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lUqseCF49V7Ae0oWy2m3bpZhN2J0IJ/QdePCji1X/kzX/jts1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GtzO//cS1EbF6xEnC/YgOlQgFo2ilB++yX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboZ1SjYJJPS73U8ISyMR3yrqWKRtz42fzSKTmzyoCEsbalkMzV3xMZjYyZRIHtjCiOzLI3E//zuimGN34mVJIiV2yxKEwlwZjM3iYDoTlDObGEMi3srYSNqKYMbTglG4K3/PIqaV1UvVq1dn9Vqbt5HEU4gVM4Bw+uoQ530IAmMAjhGV7hzRk7L86787FoLTj5zDH8gfP5A+smjOs=</latexit>

14
<latexit sha1_base64="WE1T4JjP/lx+1mTLxeXzPc9yDXU=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mkVI8FLx6r2A9oQ9lsN+3SzSbsToQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbud+54lrI2L1iNOE+xEdKREKRtFKD15tUK64VXcBsk68nFQgR3NQ/uoPY5ZGXCGT1Jie5yboZ1SjYJLPSv3U8ISyCR3xnqWKRtz42eLSGbmwypCEsbalkCzU3xMZjYyZRoHtjCiOzao3F//zeimGN34mVJIiV2y5KEwlwZjM3yZDoTlDObWEMi3srYSNqaYMbTglG4K3+vI6aV9VvXq1fl+rNNw8jiKcwTlcggfX0IA7aEILGITwDK/w5kycF+fd+Vi2Fpx85hT+wPn8AeyqjOw=</latexit>

15
<latexit sha1_base64="yJ2c3md5kUWlPyQLzoz1doVEVwI=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEq8eCF49V7Ae0oWy2m3bpZhN2J0IJ/QdePCji1X/kzX/jts1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GtzO//cS1EbF6xEnC/YgOlQgFo2ilB++qX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboZ1SjYJJPS73U8ISyMR3yrqWKRtz42fzSKTmzyoCEsbalkMzV3xMZjYyZRIHtjCiOzLI3E//zuimGN34mVJIiV2yxKEwlwZjM3iYDoTlDObGEMi3srYSNqKYMbTglG4K3/PIqaV1UvVq1dn9Zqbt5HEU4gVM4Bw+uoQ530IAmMAjhGV7hzRk7L86787FoLTj5zDH8gfP5A+4ujO0=</latexit>

Figure 3: (a) The interval tree of the string Q = abcbcbacbccbcabc over the alphabet Σ = {a, b, c}.
Here l(u) = 8, r(u) = 15 and c(u) = 11. (b) The TNFA for the regular expression e1 = a(c∗ )b(a|c)∗ .
For the interval tree in (a), leftmatchstates(root, 5) = {s2 , s3 , s4 } and rightmatchstates(root, 11) =
{s1 , s3 , s4 }
from every index in Q takes O(nm) time for each index. Thus the total time to compute all left
and right match sets is O(n2 m). In total the time to construct T is O(n2 m).
We now analyze the space. Each node v in the tree stores l(v), r(v), and c(v) in constant space
per node. Let |A| be the number of states in A. A node v also stores the set leftmatchstates(v, i) for
each i where l(v) ≤ i ≤ c(v) and the set rightmatchstates(v, j) for each j where c(v) + 1 ≤ j ≤ r(v).
The bit vectors for the left and right match sets each use O(|A|) bits, and as |A| = O(m) such a
bit vector uses O(m/w) space. Multiplying by the size of the interval, the bit vectors for a node v
use O(m/w · (r(v) − l(v))) space. On each level of the tree this sums to O(nm/w). Over all levels
of the tree, this sums to O( nm
w log n) = O(nm).

4

Regular Expressions with a Single Reference

In this section, we consider expressions of the form R = e0 (e)1 e1 \1 e2 with the language L(R) =
{w0 qw1 qw2 |wi ∈ L(ei ), q ∈ L(e)}, and present an algorithm to determine whether such an expression R of length m matches a string Q of length n using O(n2 m) time and O(nm) space.

4.1

Interval Tree Traversal

Before describing the algorithm, we first introduce a procedure to traverse the interval tree for e1 .
This procedure determines, for a substring q of Q, whether Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 .
First, we introduce some concepts relating to a substring q of Q and the nodes of the interval
tree of Q for e1 . The prefix match indices for q of a node v, which we will denote prefixmatchq (v)
are the indices i in the range of v, l(v) ≤ i ≤ r(v), such that the prefix Q[0, i] of Q matches e0 ⊙ q.
Likewise, we define the suffix match indices for q of a node v, called suffixmatchq (v) which are the
indices j in the range of v such that the suffix Q[j, n − 1] of Q matches q ⊙ e2 .
We also introduce prefixmatchq,l (v) and prefixmatchq,r (v), which are simply the prefix match
indices of v partitioned such that prefixmatchq,l (v) contains the indices of prefixmatchq (v) that are
in the left half of the interval, that is, i ∈ prefixmatchq (v) where i ≤ c(v). Likewise prefixmatchq,r (v)
contains the indices i ∈ prefixmatchq (v) where i > c(v). In the same way, we can partition
6

<!-- page 7 -->

<latexit sha1_base64="vw26snnuDv48+md3eTZrMA/yfoc=">AAACBHicbVC7TsMwFHXKq5RXgLGLRYVUlihBqDBWYmEsEn1IbRQ5rtNadZxgOxVVlIGFX2FhACFWPoKNv8FNM0DLkSwdn3uP7r3HjxmVyra/jdLa+sbmVnm7srO7t39gHh51ZJQITNo4YpHo+UgSRjlpK6oY6cWCoNBnpOtPruf17pQISSN+p2YxcUM04jSgGCkteWZ1ECI1lkGqXQF90B88zrz0PqtPzzyzZlt2DrhKnILUQIGWZ34NhhFOQsIVZkjKvmPHyk2RUBQzklUGiSQxwhM0In1NOQqJdNP8iAyeamUIg0joxxXM1d+OFIVSzkJfd+YrL9fm4n+1fqKCKzelPE4U4XgxKEgYVBGcJwKHVBCs2EwThAXVu0I8RgJhpXOr6BCc5ZNXSefcchpW4/ai1rSKOMqgCk5AHTjgEjTBDWiBNsDgETyDV/BmPBkvxrvxsWgtGYXnGPyB8fkDqqOYsQ==</latexit>

prefixmatchq (v)
<latexit sha1_base64="/66u5tVU6U25BxDvwTkrRMmG97M=">AAACBnicbVDLSsNAFJ34rPUVdSnCYBEqSEhEqsuCG5cV7APaECbTSTt0Mokzk2IJWbnxV9y4UMSt3+DOv3GaZqGtBwbOnHsP997jx4xKZdvfxtLyyuraemmjvLm1vbNr7u23ZJQITJo4YpHo+EgSRjlpKqoY6cSCoNBnpO2Prqf19pgISSN+pyYxcUM04DSgGCkteeZRL0RqKINUuwL6oD94mHnp/RnLquNTz6zYlp0DLhKnIBVQoOGZX71+hJOQcIUZkrLr2LFyUyQUxYxk5V4iSYzwCA1IV1OOQiLdND8jgyda6cMgEvpxBXP1tyNFoZST0Ned+dLztan4X62bqODKTSmPE0U4ng0KEgZVBKeZwD4VBCs20QRhQfWuEA+RQFjp5Mo6BGf+5EXSOrecmlW7vajUrSKOEjgEx6AKHHAJ6uAGNEATYPAInsEreDOejBfj3fiYtS4ZhecA/IHx+QPxiJld</latexit>

<latexit sha1_base64="EfCM/yDD+WWz4lk9vP8YQSp4wEE=">AAACBnicbVDLSsNAFJ34rPUVdSnCYBEqSEhEqsuCG5cV7APaECbTSTt0Mokzk2IJWbnxV9y4UMSt3+DOv3GaZqGtBwbOnHsP997jx4xKZdvfxtLyyuraemmjvLm1vbNr7u23ZJQITJo4YpHo+EgSRjlpKqoY6cSCoNBnpO2Prqf19pgISSN+pyYxcUM04DSgGCkteeZRL0RqKINUuwL6oD94mHnp/ZnIquNTz6zYlp0DLhKnIBVQoOGZX71+hJOQcIUZkrLr2LFyUyQUxYxk5V4iSYzwCA1IV1OOQiLdND8jgyda6cMgEvpxBXP1tyNFoZST0Ned+dLztan4X62bqODKTSmPE0U4ng0KEgZVBKeZwD4VBCs20QRhQfWuEA+RQFjp5Mo6BGf+5EXSOrecmlW7vajUrSKOEjgEx6AKHHAJ6uAGNEATYPAInsEreDOejBfj3fiYtS4ZhecA/IHx+QP6uJlj</latexit>

prefixmatchq,l (v)

prefixmatchq,r (v)

e0 q
<latexit sha1_base64="Ii6RyZFzRNxcG0/wVTQ/mH7Yi84=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx4r2A9oQ9lsJ+3SzSbuboQS+he8eFDEq3/Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJYPZpqgH9GR5CFn1MwlHLiPg3LFrboLkHXi5aQCOZqD8ld/GLM0QmmYoFr3PDcxfkaV4UzgrNRPNSaUTegIe5ZKGqH2s8WtM3JhlSEJY2VLGrJQf09kNNJ6GgW2M6JmrFe9ufif10tNeONnXCapQcmWi8JUEBOT+eNkyBUyI6aWUKa4vZWwMVWUGRtPyYbgrb68Ttq1qlev1u+vKo1aHkcRzuAcLsGDa2jAHTShBQzG8Ayv8OZEzovz7nwsWwtOPnMKf+B8/gC9HI4C</latexit>

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

<latexit sha1_base64="hkWheshLgzpdKLr+1wri01A8e88=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5xGvTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wdwDY3M</latexit>

<latexit sha1_base64="CtEGMfRC/HVlqXvlzCqR3zid6XA=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5RGrTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wdiTo3D</latexit>

l(v)

<latexit sha1_base64="F7MShkZwzQaV3wW7H7B/OPRA0J4=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5pGrTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wd5N43S</latexit>

c(v)

r(v)

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

qe2
<latexit sha1_base64="aN117f3sGSK9Hu9hIWUYRFwsU2E=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx4r2A9oQ9lsJ+3S3U3c3Qil9C948aCIV/+QN/+NSZuDtj4YeLw3w8y8IBbcWNf9dgobm1vbO8Xd0t7+weFR+fikbaJEM2yxSES6G1CDgitsWW4FdmONVAYCO8HkNvM7T6gNj9SDncboSzpSPOSM2kx6xEFtUK64VXcBsk68nFQgR3NQ/uoPI5ZIVJYJakzPc2Prz6i2nAmcl/qJwZiyCR1hL6WKSjT+bHHrnFykypCEkU5LWbJQf0/MqDRmKoO0U1I7NqteJv7n9RIb3vgzruLEomLLRWEiiI1I9jgZco3MimlKKNM8vZWwMdWU2TSeUhqCt/ryOmnXql69Wr+/qjRqeRxFOINzuAQPrqEBd9CEFjAYwzO8wpsjnRfn3flYthacfOYU/sD5/AHAg44E</latexit>

<latexit sha1_base64="5ipO2X4AfZ4nEoL9L5BeFQlrtHs=">AAACBnicbVDLSsNAFL3xWesr6lKEYBEqSElEqsuCG5cV7APaECbTSTt0Mokzk2IJXbnxV9y4UMSt3+DOv3GaZqGtBwbOnHMv997jx4xKZdvfxtLyyuraemGjuLm1vbNr7u03ZZQITBo4YpFo+0gSRjlpKKoYaceCoNBnpOUPr6d+a0SEpBG/U+OYuCHqcxpQjJSWPPOoGyI1kEEqkyCgD/qDBxMvvT9jk/Lo1DNLdsXOYC0SJyclyFH3zK9uL8JJSLjCDEnZcexYuSkSimJGJsVuIkmM8BD1SUdTjkIi3TQ7Y2KdaKVnBZHQjysrU393pCiUchz6ujJbet6biv95nUQFV25KeZwowvFsUJAwS0XWNBOrRwXBio01QVhQvauFB0ggrHRyRR2CM3/yImmeV5xqpXp7UarZeRwFOIRjKIMDl1CDG6hDAzA8wjO8wpvxZLwY78bHrHTJyHsO4A+Mzx/9RZlm</latexit>

<latexit sha1_base64="dpVRecmGb2D5LhsESAa2K4fsoto=">AAACBnicbVDLSsNAFL3xWesr6lKEYBEqSElEqsuCG5cV7APaECbTSTt0Mokzk2IJXbnxV9y4UMSt3+DOv3GaZqGtBwbOnHMv997jx4xKZdvfxtLyyuraemGjuLm1vbNr7u03ZZQITBo4YpFo+0gSRjlpKKoYaceCoNBnpOUPr6d+a0SEpBG/U+OYuCHqcxpQjJSWPPOoGyI1kEEqkyCgD/qDBxMvvT8Tk/Lo1DNLdsXOYC0SJyclyFH3zK9uL8JJSLjCDEnZcexYuSkSimJGJsVuIkmM8BD1SUdTjkIi3TQ7Y2KdaKVnBZHQjysrU393pCiUchz6ujJbet6biv95nUQFV25KeZwowvFsUJAwS0XWNBOrRwXBio01QVhQvauFB0ggrHRyRR2CM3/yImmeV5xqpXp7UarZeRwFOIRjKIMDl1CDG6hDAzA8wjO8wpvxZLwY78bHrHTJyHsO4A+Mzx8GhJls</latexit>

su!xmatchq,l (v)

su!xmatchq,r (v)

<latexit sha1_base64="FEv3uQYSwKkUye81HiaZ5lLWEBU=">AAACBHicbVDLSsNAFL2pr1pfUZfdDBahbkoiUl0W3LisYB/QljCZTtqhk4czk2IJWbjxV9y4UMStH+HOv3GaZqGtBwbOnHMv997jRpxJZVnfRmFtfWNzq7hd2tnd2z8wD4/aMowFoS0S8lB0XSwpZwFtKaY47UaCYt/ltONOrud+Z0qFZGFwp2YRHfh4FDCPEay05Jjlvo/VWHqJjD2PPegPGadOcp9Wp2eOWbFqVga0SuycVCBH0zG/+sOQxD4NFOFYyp5tRWqQYKEY4TQt9WNJI0wmeER7mgbYp3KQZEek6FQrQ+SFQr9AoUz93ZFgX8qZ7+rKbOVlby7+5/Vi5V0NEhZEsaIBWQzyYo5UiOaJoCETlCg+0wQTwfSuiIyxwETp3Eo6BHv55FXSPq/Z9Vr99qLSsPI4ilCGE6iCDZfQgBtoQgsIPMIzvMKb8WS8GO/Gx6K0YOQ9x/AHxucPtlKYug==</latexit>

su!xmatchq (v)

Figure 4: The indices that are in prefixmatchq (v) and suffixmatchq (v) respectively. The indices are
the endpoints of intervals matching e0 q and qe2 respectively. The split into prefixmatchq,l (v) and
prefixmatchq,r (v) as well as suffixmatchq,l (v) and suffixmatchq,r (v) is also shown.
suffixmatchq (v) into suffixmatchq,l (v) and suffixmatchq,r (v). See Figure 4 for an illustration of the
different sets.
Another key element is the sets of left and right match states for q of a node v, which we call
leftmatchstatesq (v) and rightmatchstatesq (v) define as follows. The set leftmatchstatesq (v) is the
union of leftmatchstates(v, i) for all i in prefixmatchq,l . That is, a state s is in leftmatchstatesq (v) if
and only if there is an index i in prefixmatchq,l (v) such that there exists a path from θ to s in A1
that matches Q[i + 1, c(v)]. Similarly, rightmatchstatesq (v) is the union of rightmatchstates(v, i) for
all i in suffixmatchq,r .
Given a substring q, the two sets leftmatchstatesq (v) and rightmatchstatesq (v) can be used to
check if Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 . If leftmatchstatesq (v) and rightmatchstatesq (v) overlap, then
there exist indices i in prefixmatchq (v) and j in suffixmatchq (v) such that there is a path in A1 from
θ to ϕ that matches Q[i + 1, j − 1]. Since Q[0, i] matches e0 ⊙ q, Q[i + 1, j − 1] matches e1 and
Q[j, n − 1] matches q ⊙ e2 , then Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 . See Figure 5 for an illustration.
Algorithm With these definitions in place, we describe the traversal algorithm IntTraversal.
Let T be the interval tree for e1 . Given the prefix and suffix match indices of the root for a substring
q of Q, we traverse T in the following way:
IntTraversal(T, prefixmatchq (root), suffixmatchq (root))
node v of T , we perform the following four steps:

We traverse T starting in the root. In a

Step 1. We partition prefixmatchq (v) into prefixmatchq,l (v) and prefixmatchq,r (v), as well as suffixmatchq (v)
into suffixmatchq,l (v) and suffixmatchq,r (v).

7

<!-- page 8 -->

Step 2. We compute
[

leftmatchstatesq (v) =

leftmatchstates(v, i)

i∈prefixmatchq,l (v)

[

rightmatchstatesq (v) =

rightmatchstates(v, i)

i∈suffixmatchq,r (v)

by OR-ing the bitvectors.
Step 3. We compute the intersection of leftmatchstatesq (v) and rightmatchstatesq (v) by AND-ing
the bitvectors and check if it is empty. If the intersection is non-empty, then Q matches the
expression R and we stop and return true.
Step 4. We continue the traversal of T . Let u and w be the left and right child of v, respectively.
If neither suffixmatchq,l (v) nor prefixmatchq,l (v) is empty, recurse on u with suffixmatchq (u) =
suffixmatchq,l (v) and prefixmatchq (u) = prefixmatchq,l (v). Likewise if neither suffixmatchq,r (v)
nor prefixmatchq,r (v) is empty, recurse on w with suffixmatchq (w) = suffixmatchq,r (v) and
prefixmatchq (w) = prefixmatchq,r (v).

4.2

Algorithm

The algorithm consists of three parts, preprocessing, finding substrings matching e, and processing
matching substrings. In the preprocessing we construct the interval tree of Q for e1 and compute
some additional information about Q. We then find the substrings q of Q that match e and for each
such q, we construct the list of occurrences of q in Q. We find these substrings one at a time. Once
we have found a matching substring q and the list of occurrences occ q we process q by traversing
the interval tree as described in Section 4.1 to determine whether Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2
in O(n2 m) time in total for all substrings. We then find and process the next matching substring
and list of occurrences, and repeat until we have processed all matching substrings of e.
We first define the set of indices prefixset, which contains the indices i of Q such that the prefix
Q[0, i] matches e0 . Likewise, the set suffixset contains the the indices j of Q such that the suffix
Q[j, n − 1] matches e2 .
Example 1. Consider the string from Figure 3 and the regular expression e0 = (a|b|c)∗ (b|c) describing the set of strings that end with b or c, as well as e2 = (b|c)∗ a(b|c)∗ describing the set
of strings that contain exactly one a. Then prefixset = {1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 14, 15} and
suffixset = {7, 8, 9, 10, 11, 12, 13}.
Preprocessing We first construct the TNFAs Ae , A0 , and A2 for the regular expressions e,
←
−
e0 , and e2 , respectively, and the reverse automaton A2 . We also construct the interval tree T of
Q for e1 and the suffix tree ST of the string Q. We then compute the sets of indices prefixset and
←
−
suffixset using the TNFAs A0 and A2 .
We check if the empty string matches e using Ae , and if so, we construct the TNFA for e0 e1 e2 ,
and use the state-set transition algorithm to check if Q matches R when the capturing group
matches ε. If it does, we stop and return true.

8

<!-- page 9 -->

<latexit sha1_base64="gBd+oeQAzTe6ydFI84S/5pp57As=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FXQvQY8OIxonlAsoTZySQZMju7zPQKYcknePGgiFe/yJt/4yTZgyYWNBRV3XR3BbEUBl3328ltbG5t7+R3C3v7B4dHxeOTlokSzXiTRTLSnYAaLoXiTRQoeSfWnIaB5O1gcjv3209cGxGpR5zG3A/pSImhYBSt9FAOLvvFkltxFyDrxMtICTI0+sWv3iBiScgVMkmN6XpujH5KNQom+azQSwyPKZvQEe9aqmjIjZ8uTp2RC6sMyDDSthSShfp7IqWhMdMwsJ0hxbFZ9ebif143weGNnwoVJ8gVWy4aJpJgROZ/k4HQnKGcWkKZFvZWwsZUU4Y2nYINwVt9eZ20riperVK7r5bq1SyOPJzBOZTBg2uowx00oAkMRvAMr/DmSOfFeXc+lq05J5s5hT9wPn8AiYyNSA==</latexit>

(b)

A1
<latexit sha1_base64="ADVtdyO6jXWrvZhxoGqrsaoIm2A=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseqF48V7Qe0oWy2k3bpZhN2N0IJ/QlePCji1V/kzX/jts1Bqw8GHu/NMDMvSATXxnW/nMLK6tr6RnGztLW9s7tX3j9o6ThVDJssFrHqBFSj4BKbhhuBnUQhjQKB7WB8M/Pbj6g0j+WDmSToR3QoecgZNVa6v+p7/XLFrbpzkL/Ey0kFcjT65c/eIGZphNIwQbXuem5i/Iwqw5nAaamXakwoG9Mhdi2VNELtZ/NTp+TEKgMSxsqWNGSu/pzIaKT1JApsZ0TNSC97M/E/r5ua8NLPuExSg5ItFoWpICYms7/JgCtkRkwsoUxxeythI6ooMzadkg3BW375L2mdVb1atXZ3Xqlf53EU4QiO4RQ8uIA63EIDmsBgCE/wAq+OcJ6dN+d90Vpw8plD+AXn4xu7aY10</latexit>

s

<latexit sha1_base64="vA2oAzeUy/yybn06paxvOiXpoWQ=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FXQvQY8OIxonlAsoTZSScZMju7zMwKYcknePGgiFe/yJt/4yTZgyYWNBRV3XR3BbHg2rjut5Pb2Nza3snvFvb2Dw6PiscnLR0limGTRSJSnYBqFFxi03AjsBMrpGEgsB1Mbud++wmV5pF8NNMY/ZCOJB9yRo2VHsr0sl8suRV3AbJOvIyUIEOjX/zqDSKWhCgNE1TrrufGxk+pMpwJnBV6icaYsgkdYddSSUPUfro4dUYurDIgw0jZkoYs1N8TKQ21noaB7QypGetVby7+53UTM7zxUy7jxKBky0XDRBATkfnfZMAVMiOmllCmuL2VsDFVlBmbTsGG4K2+vE5aVxWvVqndV0v1ahZHHs7gHMrgwTXU4Q4a0AQGI3iGV3hzhPPivDsfy9ack82cwh84nz+IB41H</latexit>

(a)

<latexit sha1_base64="RkJPXzh1NmIIAbkXV6wiGNaIQ7Q=">AAAB6nicbVBNS8NAEJ2tXzV+VT16WSyCp5CIVI8FLx4r2g9oQ9lsN+3SzSbsboQS+hO8eFDEq7/Im//GTZuDVh8MPN6bYWZemAqujed9ocra+sbmVnXb2dnd2z+oHR51dJIpyto0EYnqhUQzwSVrG24E66WKkTgUrBtObwq/+8iU5ol8MLOUBTEZSx5xSoyV7rXjDGt1z/UWwH+JX5I6lGgNa5+DUUKzmElDBdG673upCXKiDKeCzZ1BpllK6JSMWd9SSWKmg3xx6hyfWWWEo0TZkgYv1J8TOYm1nsWh7YyJmehVrxD/8/qZia6DnMs0M0zS5aIoE9gkuPgbj7hi1IiZJYQqbm/FdEIUocamU4Tgr778l3QuXL/hNu4u6023jKMKJ3AK5+DDFTThFlrQBgpjeIIXeEUCPaM39L5sraBy5hh+AX18A0VMjRY=</latexit>

<latexit sha1_base64="WtpXcHnARjpMRLeWuF3R59SJ+nQ=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Qe0oWy2k3bpZhN2N0IJ/QlePCji1V/kzX/jts1Bqw8GHu/NMDMvTAXXxvO+nNLa+sbmVnm7srO7t39QPTxq6yRTDFssEYnqhlSj4BJbhhuB3VQhjUOBnXByM/c7j6g0T+SDmaYYxHQkecQZNVa6x4E/qNY811uA/CV+QWpQoDmofvaHCctilIYJqnXP91IT5FQZzgTOKv1MY0rZhI6wZ6mkMeogX5w6I2dWGZIoUbakIQv150ROY62ncWg7Y2rGetWbi/95vcxE10HOZZoZlGy5KMoEMQmZ/02GXCEzYmoJZYrbWwkbU0WZselUbAj+6st/SfvC9etu/e6y1nCLOMpwAqdwDj5cQQNuoQktYDCCJ3iBV0c4z86b875sLTnFzDH8gvPxDew9jYQ=</latexit>

e0 q
<latexit sha1_base64="Ii6RyZFzRNxcG0/wVTQ/mH7Yi84=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx4r2A9oQ9lsJ+3SzSbuboQS+he8eFDEq3/Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJYPZpqgH9GR5CFn1MwlHLiPg3LFrboLkHXi5aQCOZqD8ld/GLM0QmmYoFr3PDcxfkaV4UzgrNRPNSaUTegIe5ZKGqH2s8WtM3JhlSEJY2VLGrJQf09kNNJ6GgW2M6JmrFe9ufif10tNeONnXCapQcmWi8JUEBOT+eNkyBUyI6aWUKa4vZWwMVWUGRtPyYbgrb68Ttq1qlev1u+vKo1aHkcRzuAcLsGDa2jAHTShBQzG8Ayv8OZEzovz7nwsWwtOPnMKf+B8/gC9HI4C</latexit>

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

e1

<latexit sha1_base64="Z6z8uxk6rsOks13oMuvlGU/iEMY=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0iKVI8FLx4r2A9oQ9lsN+3S3U3c3Qgl9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737ZQ2Nre2d8q7lb39g8Oj6vFJR8epIrRNYh6rXog15UzStmGG016iKBYhp91wepv73SeqNIvlg5klNBB4LFnECDa59EiH9WG15rneAmid+AWpQYHWsPo1GMUkFVQawrHWfd9LTJBhZRjhdF4ZpJommEzxmPYtlVhQHWSLW+fowiojFMXKljRoof6eyLDQeiZC2ymwmehVLxf/8/qpiW6CjMkkNVSS5aIo5cjEKH8cjZiixPCZJZgoZm9FZIIVJsbGU7Eh+Ksvr5NO3fUbbuP+qtZ0izjKcAbncAk+XEMT7qAFbSAwgWd4hTdHOC/Ou/OxbC05xcwp/IHz+QO/T44A</latexit>

<latexit sha1_base64="CtEGMfRC/HVlqXvlzCqR3zid6XA=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5RGrTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wdiTo3D</latexit>

<latexit sha1_base64="F7MShkZwzQaV3wW7H7B/OPRA0J4=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5pGrTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wd5N43S</latexit>

c(v)

<latexit sha1_base64="hkWheshLgzpdKLr+1wri01A8e88=">AAAB63icbVBNS8NAEJ34WetX1aOXxSLUS0hEqseCF48V7Ae0oWy2m3bp7ibsbgol9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737Wxsbm3v7Jb2yvsHh0fHlZPTto5TRWiLxDxW3RBrypmkLcMMp91EUSxCTjvh5D73O1OqNIvlk5klNBB4JFnECDa5xGvTq0Gl6rneAmid+AWpQoHmoPLVH8YkFVQawrHWPd9LTJBhZRjhdF7up5ommEzwiPYslVhQHWSLW+fo0ipDFMXKljRoof6eyLDQeiZC2ymwGetVLxf/83qpie6CjMkkNVSS5aIo5cjEKH8cDZmixPCZJZgoZm9FZIwVJsbGU7Yh+Ksvr5P2tevX3frjTbXhFnGU4BwuoAY+3EIDHqAJLSAwhmd4hTdHOC/Ou/OxbN1wipkz+APn8wdwDY3M</latexit>

l(v)

i
<latexit sha1_base64="r/ihzd0O5x9LBh+ZxNjcjw2xGyo=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF49V7Ae0oWy2m3bpZhN2J0Ip/QdePCji1X/kzX/jps1Bqw8GHu/NMDMvTKUw6HlfTmltfWNzq7xd2dnd2z+oHh61TZJpxlsskYnuhtRwKRRvoUDJu6nmNA4l74STm9zvPHJtRKIecJryIKYjJSLBKFrpXlQG1ZrneguQv8QvSA0KNAfVz/4wYVnMFTJJjen5XorBjGoUTPJ5pZ8ZnlI2oSPes1TRmJtgtrh0Ts6sMiRRom0pJAv158SMxsZM49B2xhTHZtXLxf+8XobRdTATKs2QK7ZcFGWSYELyt8lQaM5QTi2hTAt7K2FjqilDG04egr/68l/SvnD9ulu/u6w13CKOMpzAKZyDD1fQgFtoQgsYRPAEL/DqTJxn5815X7aWnGLmGH7B+fgGAY+M+A==</latexit>

<latexit sha1_base64="eUkQfcAyAzdEzbJEiZvHDTx2POA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF49V7Ae0oWy2m3btZjfsboQS+g+8eFDEq//Im//GTZuDtj4YeLw3w8y8MOFMG8/7dkpr6xubW+Xtys7u3v5B9fCorWWqCG0RyaXqhlhTzgRtGWY47SaK4jjktBNObnK/80SVZlI8mGlCgxiPBIsYwcZK94+VQbXmud4caJX4BalBgeag+tUfSpLGVBjCsdY930tMkGFlGOF0VumnmiaYTPCI9iwVOKY6yOaXztCZVYYoksqWMGiu/p7IcKz1NA5tZ4zNWC97ufif10tNdB1kTCSpoYIsFkUpR0ai/G00ZIoSw6eWYKKYvRWRMVaYGBtOHoK//PIqaV+4ft2t313WGm4RRxlO4BTOwYcraMAtNKEFBCJ4hld4cybOi/PufCxaS04xcwx/4Hz+AAMUjPk=</latexit>

j

r(v)

qe2

<latexit sha1_base64="j2pjInKa82ZWirDw6V465jgtueM=">AAAB7nicbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy2m3bpZhN2J0Ip/RFePCji1d/jzX/jps1BWx8MPN6bYWZemEph0PO+ndLG5tb2Tnm3srd/cHhUPT5pmyTTjLdYIhPdDanhUijeQoGSd1PNaRxK3gknd7nfeeLaiEQ94jTlQUxHSkSCUbRSp49jjrQyqNY811uArBO/IDUo0BxUv/rDhGUxV8gkNabneykGM6pRMMnnlX5meErZhI54z1JFY26C2eLcObmwypBEibalkCzU3xMzGhszjUPbGVMcm1UvF//zehlGt8FMqDRDrthyUZRJggnJfydDoTlDObWEMi3srYSNqaYMbUJ5CP7qy+ukfeX6dbf+cF1ruEUcZTiDc7gEH26gAffQhBYwmMAzvMKbkzovzrvzsWwtOcXMKfyB8/kD2A2PMw==</latexit>

...

p1

<latexit sha1_base64="1V5pO4HXz90fS+mPG4qbmfkq1jE=">AAAB7HicbVBNS8NAEJ3Urxq/qh69LBbBU0iKtB4LXjxWMG2hDWWz3bRLN5uwuxFK6G/w4kERr/4gb/4bN20O2vpg4PHeDDPzwpQzpV3326psbe/s7lX37YPDo+OT2ulZVyWZJNQnCU9kP8SKciaor5nmtJ9KiuOQ0144uyv83hOViiXiUc9TGsR4IljECNZG8tNRw7ZHtbrruEugTeKVpA4lOqPa13CckCymQhOOlRp4bqqDHEvNCKcLe5gpmmIywxM6MFTgmKogXx67QFdGGaMokaaERkv190SOY6XmcWg6Y6ynat0rxP+8Qaaj2yBnIs00FWS1KMo40gkqPkdjJinRfG4IJpKZWxGZYomJNvkUIXjrL2+SbsPxmk7z4abedso4qnABl3ANHrSgDffQAR8IMHiGV3izhPVivVsfq9aKVc6cwx9Ynz9owI24</latexit>

p2
ω

ω

<latexit sha1_base64="/CLBYOuRKGJEcXnAEPu7JgR+BjM=">AAAB7HicbVBNS8NAEJ2tX7V+VT16WSyCp5CIVI8FLx4rmLbQhrLZbtqlm03Y3Qgl9Dd48aCIV3+QN/+NmzYHbX0w8Hhvhpl5YSq4Nq77jSobm1vbO9Xd2t7+weFR/fiko5NMUebTRCSqFxLNBJfMN9wI1ksVI3EoWDec3hV+94kpzRP5aGYpC2IyljzilBgr+YN0wmvDesN13AXwOvFK0oAS7WH9azBKaBYzaaggWvc9NzVBTpThVLB5bZBplhI6JWPWt1SSmOkgXxw7xxdWGeEoUbakwQv190ROYq1ncWg7Y2ImetUrxP+8fmai2yDnMs0Mk3S5KMoENgkuPscjrhg1YmYJoYrbWzGdEEWosfkUIXirL6+TzpXjNZ3mw3Wj5ZRxVOEMzuESPLiBFtxDG3ygwOEZXuENSfSC3tHHsrWCyplT+AP0+QNGxY5K</latexit>

<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

<latexit sha1_base64="oXbDseSeiS9PTclG0F7kOwbEpQ4=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF48V7Ae0oWy2k3bp7ibsboQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeEAturOt+O6WNza3tnfJuZW//4PCoenzSMVGiGbZZJCLdC6hBwRW2LbcCe7FGKgOB3WB6l/vdJ9SGR+rRzmL0JR0rHnJGbS7FQ68yrNbcursAWSdeQWpQoDWsfg1GEUskKssENabvubH1U6otZwLnlUFiMKZsSsfYz6iiEo2fLm6dk4tMGZEw0lkpSxbq74mUSmNmMsg6JbUTs+rl4n9eP7HhrZ9yFScWFVsuChNBbETyx8mIa2RWzDJCmebZrYRNqKbMZvHkIXirL6+TzlXda9QbD9e1Zr2IowxncA6X4MENNOEeWtAGBhN4hld4c6Tz4rw7H8vWklPMnMIfOJ8/Mh6Now==</latexit>

<latexit sha1_base64="oXbDseSeiS9PTclG0F7kOwbEpQ4=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF48V7Ae0oWy2k3bp7ibsboQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeEAturOt+O6WNza3tnfJuZW//4PCoenzSMVGiGbZZJCLdC6hBwRW2LbcCe7FGKgOB3WB6l/vdJ9SGR+rRzmL0JR0rHnJGbS7FQ68yrNbcursAWSdeQWpQoDWsfg1GEUskKssENabvubH1U6otZwLnlUFiMKZsSsfYz6iiEo2fLm6dk4tMGZEw0lkpSxbq74mUSmNmMsg6JbUTs+rl4n9eP7HhrZ9yFScWFVsuChNBbETyx8mIa2RWzDJCmebZrYRNqKbMZvHkIXirL6+TzlXda9QbD9e1Zr2IowxncA6X4MENNOEeWtAGBhN4hld4c6Tz4rw7H8vWklPMnMIfOJ8/Mh6Now==</latexit>

<latexit sha1_base64="1V5pO4HXz90fS+mPG4qbmfkq1jE=">AAAB7HicbVBNS8NAEJ3Urxq/qh69LBbBU0iKtB4LXjxWMG2hDWWz3bRLN5uwuxFK6G/w4kERr/4gb/4bN20O2vpg4PHeDDPzwpQzpV3326psbe/s7lX37YPDo+OT2ulZVyWZJNQnCU9kP8SKciaor5nmtJ9KiuOQ0144uyv83hOViiXiUc9TGsR4IljECNZG8tNRw7ZHtbrruEugTeKVpA4lOqPa13CckCymQhOOlRp4bqqDHEvNCKcLe5gpmmIywxM6MFTgmKogXx67QFdGGaMokaaERkv190SOY6XmcWg6Y6ynat0rxP+8Qaaj2yBnIs00FWS1KMo40gkqPkdjJinRfG4IJpKZWxGZYomJNvkUIXjrL2+SbsPxmk7z4abedso4qnABl3ANHrSgDffQAR8IMHiGV3izhPVivVsfq9aKVc6cwx9Ynz9owI24</latexit>

p1

p2

Figure 5: Given that there is overlap of leftmatchstatesq (v) and rightmatchstatesq (v) for a node v
in the interval tree, the string matches regular expressions as shown in (a). (b) Let s be a node in
this overlap. Then the paths p1 and p2 exist, going from θ to s and from s to ϕ respectively, and
they match substrings of Q as shown in (a).
Finding matching substrings In this part, we use the suffix tree to find the substrings q
of Q that match e. We do this in the following way:
For a node v in ST , let S(v) be the set of states in the state-set simulation after matching
str(v) in Ae . For the root r of the suffix tree, S(r) is the ε-closure of θAe . We now traverse the
suffix tree depth-first, starting in the root. Given S(v) for a node v, we process each child u of v in
the following way: Let Q[i, j] be the edge label of the edge (v, u). Starting with the state-set S(v),
match Q[i, j] in Ae using state-set transitions. For every k where i ≤ k ≤ j we check if ϕAe is in the
state-set after matching Q[i, k]. If so, then q = str(v) ⊙ Q[i, k] matches e. In this case, we construct
the list of occurrences of q in Q from the suffix tree, and process q as described below. Otherwise,
we continue. Once we have matched the entire label Q[i, j], we save the current state-set as S(u),
and continue the traversal of the suffix tree.
Example 2. Consider again the string from Figure 3 and e = ac|b Then the only two matching
substrings are ac and b, and occ ac = [6] and occ b = [1, 3, 5, 8, 11, 12, 14].
Processing a substring We compute the prefix and suffix match indices of the root, root, of
the interval tree by iterating through the indices i in occ q , and letting i + |q| ∈ prefixmatchq (root)
if and only if i ∈ prefixset. Likewise, i ∈ suffixmatchq (root) if and only if i + |q| ∈ suffixset. We then
traverse the interval tree using the procedure IntTraversal as described in Section 4.1. If there is
overlap between leftmatchstatesq (v) and rightmatchstatesq (v) for any v and q, then Q matches R.

Example 3. Consider the string and interval tree from Figure 3, as well as the sets prefixset
and suffixset from Example 1 and occ b from Example 2. Then, prefixmatchb (root) = [3, 5, 8, 11, 12]
and since c(root) = 7, prefixmatchb,l (root) = [3, 5]. Likewise, suffixmatchb (root) = [8, 11, 12] =
suffixmatchb,r (root), since all indices in suffixmatchb (root) are larger than c(v). For the root node root
in the interval tree, and string b, leftmatchstatesb (root) = {s2 , s3 , s4 } since leftmatchstates(root, 3) =
∅ and leftmatchstates(root, 5) = {s2 , s3 , s4 }. Likewise, rightmatchstatesb (root) = {s1 , s3 , s4 , s5 , s8 , s8 , s10 s11 , ϕ}
since rightmatchstates(r, 8) = {s5 , s8 , s10 , s11 , ϕ}, rightmatchstates(r, 11) = {s1 , s3 , s4 } and rightmatchstates(root, 12)
∅. Then the intersection leftmatchstatesb (root) ∩ rightmatchstatesb (root) = {s3 , s4 } is not empty. In
this case Q[6, 11] matches e1 , 5 ∈ prefixmatchb (root) and 12 ∈ suffixmatchb (root).

4.3

Analysis

We will first analyze the algorithm with respect to time. Let mi be the length of the regular
expression ei and me be the length of e, such that m = m0 + me + m1 + 1 + m2 .
9

<!-- page 10 -->

←
−
Preprocessing The TNFAs Ae , A0 , A2 and A2 can be built in O(me + m0 + m2 ) time. The
sets prefixset and suffixset are computed in O(n(m0 + m2 )) time. The suffix tree of Q can be
built in O(n log n) time. By Lemma 3 we can build the interval tree in O(n2 m1 ) time. Checking
if the empty string matches e takes O(me ) time, constructing the automaton for e0 e1 e2 takes
O(m0 + m1 + m2 ) time, and checking if Q matches e0 e1 e2 takes O(n(m0 + m1 + m2 )). Thus we use
O(n2 m1 + n(m0 + m1 + m2 ) + me ) time in total on preprocessing.
Finding matching substrings When traversing and matching in the suffix tree, each label
of the suffix tree is read once and the characters are matched in the TNFA for e, which takes
O(me ) per character. The combined length of the labels in the suffix tree is O(n2 ), so reading and
matching all labels takes O(n2 me ) time.
The combined length of occ q over all substrings q of Q that match e is O(n2 ) since there are
O(n2 ) substrings of Q. Since we have the position of the locus of q in the suffix tree the list occ q
can be computed in O(|occ q |). Thus the combined time to output occ q for all q is O(n2 ). In total
we use O(n2 me ) time on this part.
Processing substrings First, we compute the lists prefixmatchq (root) and suffixmatchq (root)
from occ q and prefixset and suffixset. This is only done in the root. The total length of the lists
occ q for all substrings q of Q is O(n2 ) so the combined time for computing prefixmatchq (root) and
suffixmatchq (root) for all q is O(n2 ) time.
We now look at the computations that are done in each node. For this, we will use the following
lemma:
Lemma 4. Let occ q (a, b) be the list containing the indices i from occ q where a ≤ i ≤ b. For a node
v in the interval tree,
X
X
|suffixmatchq (v)| ≤
|occ q (l(v), r(v))| ≤ (r(v) − l(v) + 1) · n
q∈Q

q∈Q

and
X
q∈Q

|prefixmatchq (v)| ≤

X

|occ q (l(v), r(v))| ≤ (r(v) − l(v) + 1) · n

q∈Q

Proof. We know that |suffixmatchq (v)| ≤ |occ q (l(v), r(v))| since every suffix Q[i, n − 1] where i ∈
suffixmatchq starts with an occurrence of q. Likewise, |prefixmatchq (v)| ≤ |occ q (l(v) − |q|, r(v) − |q|)|
since occ q (l(v) − |q|, r(v) − |q|) contains the indices of occurrences of q that end in the interval
P of v. Lastly, we note that at most n different substrings can start on the same index, so
q∈Q |occ q (a, b)| ≤ (b − a + 1) · n, which leads to the lemma.
In each node of the tree, we partition prefixmatchq (v) and suffixmatchq (v) according to c(v) by
comparing each index to c(v), which takes time O(|prefixmatchq (v)| + |suffixmatchq (v)|). We then
compute leftmatchstatesq (v) as a union of the state-sets leftmatchstates(v, i) for i ∈ prefixmatchq (v).
As the state-sets are represented by bit vectors of size O(m1 /w), each union takes O(m1 /w) time
using an OR operation. Similarly, for rightmatchstatesq (v). Computing these sets thus takes
O((|prefixmatchq (v)| + |suffixmatchq (v)|) · m1 /w) time.
The intersection between leftmatchstatesq (v) and rightmatchstatesq (v) is only computed in the
nodes of the interval tree that are visited in the traversal for the given q. A node is only visited if
10

<!-- page 11 -->

prefixmatchq (v) and suffixmatchq (v) are not empty, in which case we spend O(m1 /w) to compute
the intersection. Otherwise, we do not compute the intersection at node v. Thus, the bound
O((|prefixmatchq (v)| + |suffixmatchq (v)|) · m1 /w) also holds for the time spent on intersections.
The combined time spent in node v for a substring
P q of Q is O((|prefixmatchq (v)|+|suffixmatchq (v)|)·
m1 /w), so for all of the substrings of Q we use O( q∈Q (|prefixmatchq (v)|+|suffixmatchq (v)|)·m1 /w),
which by Lemma 4 is O((r(v) − l(v) + 1)n · m1 /w). Since the combined size of the intervals on each
level of the tree is n, the time spent on each level is O(n2 m1 /w). Over all levels of the tree, we
2
spend O( n wm1 log n) = O(n2 m1 ) time.
In total, we use O(n2 (m1 + me ) + n(m0 + m2 )) = O(n2 m) time. We now analyze the algorithm’s
space usage.
←
−
Preprocessing The TNFAs Ae , A0 , A2 , and A2 use O(me + m0 + m2 ) space, the sets prefixset
and suffixset use O(n) space, and the interval tree uses O(nm1 ) space by Lemma 3. The suffix tree
uses O(n) space, and the automaton for e0 e1 e2 uses O(m0 + m1 + m2 ) space. This gives us
O(nm1 + me + m0 + m2 ) space.
Finding matching substrings The sets S(v) use O(me ) space each, and we store one such
set per node in the suffix tree, so O(nme ) space in total. We only need occ q for one substring at a
time, and the size of each occ q is O(n). The space complexity of this part then becomes O(nme ).
Processing substrings The suffix and prefix match indices of q for the root use O(n) space,
and once we have partitioned the lists, we no longer need the originals. The sets of left and right
match states for q of each node use O(m1 /w) space, but are not needed after we finish processing
the corresponding node, so we only need O(m1 /w) space for this. Thus, processing substrings uses
O(n + m1 /w) space in addition to the precomputed info and data structures.
In total, the algorithm uses O(n(m1 + me ) + m0 + m2 ) = O(nm) space.

4.4

Correctness

To prove the algorithm’s correctness, we prove the following two lemmas.
Lemma 5. The procedure IntTraversal computes the intersection of the sets leftmatchstatesq (v) and
rightmatchstatesq (v) for every node v in the interval tree T where the intersection is not empty.
Proof. During the traversal, we compute the intersection of the sets leftmatchstatesq (v) and rightmatchstatesq (v)
for every v in T where neither suffixmatchq (v) nor prefixmatchq (v) s empty. If prefixmatchq (v) is
empty, then so is leftmatchstatesq (v). Likewise, if suffixmatchq (v) is empty, then so is rightmatchstatesq (v),
so in these cases, we know the intersection to be empty as well.
Lemma 6. Given a non-empty substring q of Q, the sets leftmatchstatesq (v) and rightmatchstatesq (v)
intersect for some v in T if and only if Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 .
Proof. We first show that if Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 then the sets leftmatchstatesq (v) and
rightmatchstatesq (v) overlap for some v in T .
If Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 , then there is a decomposition of Q into w0 qw1 qw2 where wi
matches ei . Let i = |w0 | + |q| − 1 be the position right before w1 in Q and j = n − |q| − |w2 | be the
position just after w1 in Q (See Figure 5(a)) and let v be the node in the interval tree corresponding
11

<!-- page 12 -->

to the smallest interval containing both i and j, that is l(v) ≤ i < j ≤ r(v). Note that since v
represents the smallest such interval, this implies that i ≤ c(v) < j. Since Q[0, i] = w0 q and i ≤ c(v),
i ∈ prefixmatchq,l (v). Likewise, since Q[j, n − 1] = qw2 and j > c(v), then j ∈ suffixmatchq,r (v).
Since Q[i + 1, j − 1] = w1 and w1 matches e1 , there is a path p from θA1 to ϕA1 in A1 matching
Q[i + 1, j − 1]. As the labels on the edges of A1 have length at most 1, we can split p into p1 and
p2 , such that p1 is a subpath of p that starts in θA1 , matches Q[i + 1, c(v)], and ends in some state
s. Then p2 is the subpath of p from s to ϕA1 which matches Q[c(v), j − 1]. Note that there could
be several valid choices for s due to ϵ-transitions.
Because of the existence of p1 we have that s is in leftmatchstates(v, i), and together with the
fact that i ∈ prefixmatchq,l (v) this implies that s ∈ leftmatchstatesq (v). Likewise, because of the
existence of p2 , then s is in rightmatchstates(v, i) and together with the fact that i ∈ suffixmatchq,r (v)
this implies that s ∈ rightmatchstatesq (v). Thus, s ∈ leftmatchstatesq (v) ∩ rightmatchstatesq (v).
We now show that if the sets leftmatchstatesq (v) and rightmatchstatesq (v) overlap for some v in
T then Q matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ e2 .
Assume that leftmatchstatesq (v) ∩ rightmatchstatesq (v) ̸= ∅ for some v in T , and consider a state
s ∈ leftmatchstatesq (v) ∩ rightmatchstatesq (v).
Since s ∈ leftmatchstatesq (v), there is an index i such that s ∈ leftmatchstates(v, i), and i ∈
prefixmatchq,l (v). Then, there exists a path p1 from θA1 to s matching Q[i + 1, c(v)]. Likewise,
there is an index j ∈ suffixmatchq,r (v) for which there exists a path p2 from s to ϕA1 matching
Q[c(v) + 1, j − 1]. Then p = p1 ⊙ p2 is a path from θA1 to ϕA1 matching Q[i + 1, j − 1]. Thus
w1 = Q[i + 1, j − 1] matches e1 .
Since i ∈ prefixmatchq,l (v) we know that Q[0, i] = w0 q, where w0 matches e0 and q matches e.
In the same way, Since j ∈ suffixmatchq,r (v) we know that Q[j, n − 1] = qw2 , where w2 matches e2
and q again matches e. Put together, we have that Q = w0 qw1 qw2 .
We traverse the interval tree for every non-empty substring q of Q that matches e, and explicitly
check the case where the capturing group matches the empty string, so the correctness of the
algorithm follows from Lemma 5 and Lemma 6. In summary, we have shown the following theorem.
Theorem 7. Given a regular expression of length m with a single capturing group and a single
reference, we can solve the regular expression matching with backreferences problem in O(n2 m) time
and O(nm) space.

5

Regular Expressions with Multiple References

In this section we consider more general patterns of the form R = e0 (e)1 e1 \1 e2 \1 e3 . . . ek \1 ek+1 .
We extend our techniques from Section 4 to this case while maintaining O(n2 m) time and O(nm)
space.
We introduce a new procedure, which traverses k interval trees T1 . . . Tk for the regular expressions e1 . . . ek . A key difference between this sequential interval tree traversal and the one in
Section 4 is that for the traversal of each Th , the new procedure keeps track of the positions i
where Q[0, i] matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ . . . ⊙ q ⊙ eh ⊙ q. These positions are stored in the set
nextprefixmatchq,h , and used as the prefix match indices of the root of Th+1 for the traversal of the
next interval tree. See Figure 6.

12

<!-- page 13 -->

5.1

Sequential Interval Tree Traversal

We will generalize the definition of prefixmatchq (v), relating it to the interval tree of v. For a node v
in Th , prefixmatchq (v) contains the indices i such that Q[0, i] matches e0 ⊙q⊙e1 ⊙q⊙. . .⊙q⊙eh−1 ⊙q.
Note that for h = 1, the prefix match indices of the root of Th are the indices i such that Q[0, i]
matches e0 ⊙ q, which fits with the definition from Section 4. For an illustration, see Figure 6.
We use the same notation as in Lemma 4, where occ q (i, j) are the occurrences of q that start
between index i and j.
Given a substring q of Q that matches e, as well as the list of occurrences occ q and the prefix
match indices of the root of T1 , we traverse the k interval trees in k rounds as follows:
Sequential interval tree traversal For h = 1 to k do the following. Let root h be the root
of Th . If h > 1 we start by setting prefixmatchq (root h ) = nextprefixmatchq,h . We then initialize an
empty set nextprefixmatchq,h and start the traversal of Th . In node v of Th we do the following four
steps:
Step 1. We partition prefixmatchq (v) into prefixmatchq,l (v) and prefixmatchq,r (v), as well as occ q (l(v), r(v))
into occ q (l(v), c(v)) and occ q (c(v) + 1, r(v)).
Step 2. We compute leftmatchstatesq (v) as the union of the sets leftmatchstates(v, i) for all indices
i in prefixmatchq,l (v).
Step 3. For every j ∈ occ q (c(v) + 1, r(v)) we compute the intersection of leftmatchstatesq (v)
and rightmatchstates(v, j). If the intersection is non-empty, then we add j − 1 + |q| to
nextprefixmatchq,h .
Step 4. We continue the traversal of Th . Let u and w be the left and right child of v, respectively.
If neither occ q (l(v), c(v)) nor prefixmatchq,l (v) is empty, recurse on u with occ q (l(u), r(u)) =
occ q (l(v), c(v)) and prefixmatchq (u) = prefixmatchq,l (v). Likewise if neither occ q (c(v)+ 1, r(v))
nor prefixmatchq,r (v) is empty, recurse on w with occ q (l(w), r(w)) = occ q (c(v) + 1, r(v)) and
prefixmatchq (w) = prefixmatchq,r (v).
Note that we here use occ q in much the same way as we used suffixmatchq (root) in Section 4,
since we are now interested in every position where an occurrence of q starts regardless of what
comes after it.

5.2

Algorithm

The algorithm is similar to the one from Section 4, with some key differences.
The main difference is that we use the sequential interval tree traversal defined above. Another
difference from the algorithm in Section 4 is that while finding matching substrings, we keep a set
endpoints. Every time we finish processing a substring q, we add the indices i where Q[0, i] matches
e0 ⊙ q ⊙ e1 ⊙ q ⊙ . . . ⊙ q ⊙ ek ⊙ q to endpoints. These indices are exactly the ones computed in
nextprefixmatchq,k .
In postprocessing, we check if there is an index i in endpoints such that i + 1 is in suffixset. If
so, R matches Q.

13

<!-- page 14 -->

eh
<latexit sha1_base64="wAOEUE4iJvgG+3pX+rFtumHdKwg=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mkVI8FLx4r2g9oQ9lsJ+3SzSbsboQS+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpAQfjQbniVt0FyDrxclKBHM1B+as/jFkaoTRMUK17npsYP6PKcCZwVuqnGhPKJnSEPUsljVD72eLUGbmwypCEsbIlDVmovycyGmk9jQLbGVEz1qveXPzP66UmvPEzLpPUoGTLRWEqiInJ/G8y5AqZEVNLKFPc3krYmCrKjE2nZEPwVl9eJ+2rqlev1u9rlUYtj6MIZ3AOl+DBNTTgDprQAgYjeIZXeHOE8+K8Ox/L1oKTz5zCHzifP0F2jcE=</latexit>

<latexit sha1_base64="nF1rYbP2Yy+O9MeGJdK2imhPe90=">AAAB+3icbVDLSsNAFJ3UV62vWJduBovgxpBIqS4LblxWsA9oQ5hMb9qhk0dnJmIJ+RU3LhRx64+482+ctllo64ELh3Pu5d57/IQzqWz72yhtbG5t75R3K3v7B4dH5nG1I+NUUGjTmMei5xMJnEXQVkxx6CUCSOhz6PqT27nffQQhWRw9qFkCbkhGEQsYJUpLnlkFz56C51iWBV42vnTyqWfWbMteAK8TpyA1VKDlmV+DYUzTECJFOZGy79iJcjMiFKMc8soglZAQOiEj6GsakRCkmy1uz/G5VoY4iIWuSOGF+nsiI6GUs9DXnSFRY7nqzcX/vH6qghs3Y1GSKojoclGQcqxiPA8CD5kAqvhME0IF07diOiaCUKXjqugQnNWX10nnynIaVuO+XmvWizjK6BSdoQvkoGvURHeohdqIoif0jF7Rm5EbL8a78bFsLRnFzAn6A+PzBycikzM=</latexit>

e0 qe1 ...eh→1 q

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

i
<latexit sha1_base64="GjSt58somH+uKTfwgBcMHhK5AFE=">AAAB6nicbVBNS8NAEJ2tXzV+VT16WSyCp5JIqR4LXjxWtB/QhrLZbtqlm03Y3Qgl9Cd48aCIV3+RN/+NmzYHbX0w8Hhvhpl5QSK4Nq77jUobm1vbO+VdZ2//4PCocnzS0XGqKGvTWMSqFxDNBJesbbgRrJcoRqJAsG4wvc397hNTmsfy0cwS5kdkLHnIKTFWeuCOM6xU3Zq7AF4nXkGqUKA1rHwNRjFNIyYNFUTrvucmxs+IMpwKNncGqWYJoVMyZn1LJYmY9rPFqXN8YZURDmNlSxq8UH9PZCTSehYFtjMiZqJXvVz8z+unJrzxMy6T1DBJl4vCVGAT4/xvPOKKUSNmlhCquL0V0wlRhBqbTh6Ct/ryOulc1bxGrXFfrzbrRRxlOINzuAQPrqEJd9CCNlAYwzO8whsS6AW9o49lawkVM6fwB+jzBzfejRI=</latexit>

q
<latexit sha1_base64="MXg03aTYy6+mdXpCGWNhGwTEncA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2m3bpZhN3J0Ip/QdePCji1X/kzX/jps1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GN5nffuLaiFg94CThfkSHSoSCUbTS/WOpX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboT6lGwSSflXqp4QllYzrkXUsVjbjxp/NLZ+TMKgMSxtqWQjJXf09MaWTMJApsZ0RxZJa9TPzP66YYXvtToZIUuWKLRWEqCcYke5sMhOYM5cQSyrSwtxI2opoytOFkIXjLL6+S1kXVq1Vrd5eV+mUeRxFO4BTOwYMrqMMtNKAJDEJ4hld4c8bOi/PufCxaC04+cwx/4Hz+AA+FjQY=</latexit>

<latexit sha1_base64="GU0SfVS+MGmvsQn0OPjhkDieuAY=">AAAB6nicbVBNS8NAEJ2tXzV+VT16WSyCp5JIqR4LXjxWtLXQhrLZbtq1m03Y3Qgl9Cd48aCIV3+RN/+NmzYHbX0w8Hhvhpl5QSK4Nq77jUpr6xubW+VtZ2d3b/+gcnjU0XGqKGvTWMSqGxDNBJesbbgRrJsoRqJAsIdgcp37D09MaR7LezNNmB+RkeQhp8RY6e7RcQaVqltz58CrxCtIFQq0BpWv/jCmacSkoYJo3fPcxPgZUYZTwWZOP9UsIXRCRqxnqSQR0342P3WGz6wyxGGsbEmD5+rviYxEWk+jwHZGxIz1speL/3m91IRXfsZlkhom6WJRmApsYpz/jYdcMWrE1BJCFbe3YjomilBj08lD8JZfXiWdi5rXqDVu69VmvYijDCdwCufgwSU04QZa0AYKI3iGV3hDAr2gd/SxaC2hYuYY/gB9/gA5ZI0T</latexit>

i→

j

<latexit sha1_base64="heb5UAhOvML8AY5anV5kKRyI/jQ=">AAAB63icbVBNS8NAEJ3Urxq/qh69LBbRU0mkVI8FLx4r2A9oQ9lsN+3S3U3Y3Qgl9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737ZQ2Nre2d8q77t7+weFR5fiko+NUEdomMY9VL8SaciZp2zDDaS9RFIuQ0244vcv97hNVmsXy0cwSGgg8lixiBJtcYpeuO6xUvZq3AFonfkGqUKA1rHwNRjFJBZWGcKx13/cSE2RYGUY4nbuDVNMEkyke076lEguqg2xx6xxdWGWEoljZkgYt1N8TGRZaz0RoOwU2E73q5eJ/Xj810W2QMZmkhkqyXBSlHJkY5Y+jEVOUGD6zBBPF7K2ITLDCxNh48hD81ZfXSee65jdqjYd6tVkv4ijDGZzDFfhwA024hxa0gcAEnuEV3hzhvDjvzseyteQUM6fwB87nD5iZjUM=</latexit>

Figure 6: For an index i′ to be in nextprefixmatchq,h , there must be an i in prefixmatchq (root h ) such
that Q[i + 1, j − 1] matches eh and the string Q[j, i′ ] for j = i′ − |q| + 1 is the string q (such that j
is in occ q ).
Preprocessing Let prefixset be the indices i such that Q[0, i] matches e0 , and suffixset be the
←−−−
indices j such that Q[j, n − 1] matches ek+1 . We build the TNFAs Ae , A0 and Ak+1 and use A0
←−−−
and Ak+1 to compute prefixset and suffixset. Then we construct the interval trees T1 . . . Tk .
We check if the empty string matches e using Ae , and if so, we construct the TNFA for
e0 e1 e2 . . . ek+1 , and use the state-set transition algorithm to check if Q matches R when the capturing group matches ε. If it does, we stop and return true.
Finding matching substrings Finding matching substrings is done just as in Section 4, and
we process each matching substring q as described below. Furthermore, we initialize a set endpoints
to which we will add the indices in nextprefixmatchq,k after we finish processing a substring q.
Processing matching substrings We compute prefixmatchq (root 1 ) as before by iterating
through the indices i in occ q , and letting i + |q| ∈ prefixmatchq (root h ) if and only if i ∈ prefixset.
We then traverse the k interval trees as described in Section 5.1. After traversing T1 . . . Tk for
a substring q, we add the indices of nextprefixmatchq,k to the set endpoints.
Postprocessing After processing all matching substrings, we check for every index i in
endpoints whether i + 1 is in suffixset. If such an i exists, R matches Q. Otherwise, R does
not match Q.

5.3

Analysis

Again, let mi be the length of the regular expression ei and me be the length of e, such that
m = m0 + me + m1 + 1 + m2 + 1 + . . . + 1 + mk+1 . We first analyze the algorithm’s running time.
←−−−
Preprocessing As in Section 4.3, we can build Ae , A0 , Ak+1 and Ak+1 in O(me + m0 + mk+1 )
time and compute prefixset and suffixset in O(n(m0 + mk+1 )) time. Building the interval tree
of Q P
for eh takes O(n2 mh ) according to Lemma 3, so building the interval trees T1 . . . Tk takes
k
2
O(n
empty string matches e takes O(me ) time, and checking if Q
h=1 mh ). Checking if the P
Pk
2
matches e0 e1 e2 . . . ek+1 takes O(n( k+1
h=0 mh )). This gives us O(n ( h=1 mh ) + n(m0 + mk+1 ))
Finding matching substrings The only thing in this part that has been changed from the
simpler algorithm in Section 4 is the addition of the set endpoints. We analyze only this, as the
rest is unchanged, and thus takes O(n2 me ).
The total number of times an index is added to the set endpoints is the combined length of
nextprefixmatch
substrings q. Since |nextprefixmatchq,k | ≤ |occ q |, the time spent
q,k over all matching P
P
is O( q∈Q nextprefixmatchq,k ) ≤ O( q∈Q |occ q |) = O(n2 ).
14

<!-- page 15 -->

Sequential interval tree traversal We will consider the time spent on the traversal of one
interval tree Th . For h = 1, we compute prefixmatchq (root 1 ) from occ q and prefixset. This takes
O(|occ q |) for each substring q, so O(n2 ) time in total for all substrings. In the rest of the interval
trees, we simply use the indices of nextprefixmatchq,h−1 , which has size smaller than |occ q |, so O(n2 )
time for all substrings.
We now consider the time spent in a node v in an interval tree Th . Partitioning prefixmatchq (v)
and occ q (l(v), r(v)) takes O(|prefixmatchq (v)| + |occ q (l(v), r(v))|) since we compare every index to
c(v). The set leftmatchstatesq (v) is computed in O(|prefixmatchq (v)| · mh /w) time by the same
argument as in Section 4.3. We then compute O(|occ q (l(v), r(v))|) intersections of state-sets, each
of which takes O(mh /w) time, so we spend O(|occ q (l(v), r(v))| · mh /w) time on these intersections.
In total, we spend O((|prefixmatch
q (v)| + |occ q (l(v), r(v))|) · mh /w) n a node for a substring q. Over
P
all substrings, we spend O( q∈Q (|prefixmatchq (v)| + |occ q (l(v), r(v))|) · mh /w) which by Lemma 4
is O((r(v) − l(v))nmh /w). For a level of the tree, we spend O(n2 mh /w), and for traversing all levels
P
2
of Th we spend O( n wmh log n) = O(n2 mh ). Traversing all k interval trees takes O(n2 kh=1 mh ). In
P
total the algorithm uses O(n2 (me + kh=1 mh ) + n(m0 + mk+1 )) = O(n2 m) time.
P
We now consider the space. We use O(n( kh=1 mh )) space for the interval trees and O(me +
Pk+1
h=0 mh ) space for the automata, while still using O(nme ) for finding matching substrings of e.
Prefix match indices use O(n) space and are only needed for one interval tree at a time, and occq
uses O(n) space and is only needed for one substring at a time. The sets of leftmatchstatesq (v)
use (mh /w),
P for v in Th , and are only needed for one node v at a time. Thus, in total we use
O(n(me + kh=1 mh ) + m0 + mk+1 ) = O(nm) space.

5.4

Correctness

We first show the following lemma.
Lemma 8. Let root h be the root of Th . Given that prefixmatchq (root h ) contains i if and only if
Q[0, i] matches a rewb E, then nextprefixmatchq,h contains i′ if and only if Q[0, i′ ] matches E⊙eh ⊙q.
Proof. We assume that prefixmatchq (root h ) contains i if and only if Q[0, i] matches E, and show that
nextprefixmatchq,h contains index i′ if Q[0, i′ ] matches E ⊙ eh ⊙ q. If Q[0, i′ ] matches E ⊙ eh ⊙ q, then
there is a decomposition of Q[0, i′ ] into W wh q such that the string W matches E and wh matches
eh . By the assumption and by this decomposition, we know that there is an i in prefixmatchq (root h )
such that Q[0, i] matches E. Let j = i′ − |q| + 1. Then j is the start of an occurrence of q, so j is
in occ q . Let v be the node in Th representing the smallest interval containing both i and j. Then
i ≤ c(v) < j, and since wh matches eh , there exists a path p in Ah matching Q[i + 1, j − 1]. We
split p into p1 and p2 , such that p1 is a subpath of p that starts in θAh , matches Q[i + 1, c(v)],
and ends in some state s. Then p2 is the subpath of p from s to ϕAh and matches Q[c(v), j − 1].
By the existence of p1 we know that s ∈ leftmatchstates(v, i), and by the existence of p2 we
know that s ∈ rightmatchstates(v, j). As i is in prefixmatchq,l (v), and s is in leftmatchstates(v, i),
then s is in leftmatchstatesq (v). Since j is in occ q (l(v), r(v)), we will compute the intersection
leftmatchstatesq (v) ∩ rightmatchstates(v, j), and as s is in rightmatchstates(v, j), then i′ = j + |q| − 1
is in nextprefixmatchq,h .
We now show that Q[0, i′ ] matches E⊙eh ⊙q if nextprefixmatchq,h contains index i′ . Again, let j =
i′ −|q|+1 If i′ is in nextprefixmatchq,h then there is some node v in Th for which j is in occ q (l(v), r(v))
and leftmatchstatesq (v)∩rightmatchstates(v, j) ̸= ∅. Consider s ∈ leftmatchstatesq (v)∩rightmatchstates(v, j).
15

<!-- page 16 -->

Eω
<latexit sha1_base64="nUhcCG+QwTa9M8dUyIZZDt/ta3c=">AAAB73icbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCCB4r2A9oQ9lsJ+3SzSbuboQS+ie8eFDEq3/Hm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCopeNUMWyyWMSqE1CNgktsGm4EdhKFNAoEtoPxzcxvP6HSPJYPZpKgH9Gh5CFn1Fipc9vPeijEtF+uuFV3DrJKvJxUIEejX/7qDWKWRigNE1Trrucmxs+oMpwJnJZ6qcaEsjEdYtdSSSPUfja/d0rOrDIgYaxsSUPm6u+JjEZaT6LAdkbUjPSyNxP/87qpCa/9jMskNSjZYlGYCmJiMnueDLhCZsTEEsoUt7cSNqKKMmMjKtkQvOWXV0nrourVqrX7y0rdzeMowgmcwjl4cAV1uIMGNIGBgGd4hTfn0Xlx3p2PRWvByWeO4Q+czx8X/I/4</latexit>

<latexit sha1_base64="wFMocP/Fp1XELAmR69IRK6G3ys0=">AAAB/XicbVDLSsNAFJ34rPUVHzs3g0VwY0mkVJcFKbisYB/QhDCZTNqhk5kwMxFqKP6KGxeKuPU/3Pk3TtsstPXAhcM593LvPWHKqNKO822trK6tb2yWtsrbO7t7+/bBYUeJTGLSxoIJ2QuRIoxy0tZUM9JLJUFJyEg3HN1M/e4DkYoKfq/HKfETNOA0phhpIwX2cTNwPBYJrWAzyD3C2IU7CeyKU3VmgMvELUgFFGgF9pcXCZwlhGvMkFJ910m1nyOpKWZkUvYyRVKER2hA+oZylBDl57PrJ/DMKBGMhTTFNZypvydylCg1TkLTmSA9VIveVPzP62c6vvZzytNME47ni+KMQS3gNAoYUUmwZmNDEJbU3ArxEEmEtQmsbEJwF19eJp3Lqluv1u9qlUatiKMETsApOAcuuAINcAtaoA0weATP4BW8WU/Wi/VufcxbV6xi5gj8gfX5A1LslHk=</latexit>

E0 . . . Eω→1

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

q
<latexit sha1_base64="MXg03aTYy6+mdXpCGWNhGwTEncA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2m3bpZhN3J0Ip/QdePCji1X/kzX/jps1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GN5nffuLaiFg94CThfkSHSoSCUbTS/WOpX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboT6lGwSSflXqp4QllYzrkXUsVjbjxp/NLZ+TMKgMSxtqWQjJXf09MaWTMJApsZ0RxZJa9TPzP66YYXvtToZIUuWKLRWEqCcYke5sMhOYM5cQSyrSwtxI2opoytOFkIXjLL6+S1kXVq1Vrd5eV+mUeRxFO4BTOwYMrqMMtNKAJDEJ4hld4c8bOi/PufCxaC04+cwx/4Hz+AA+FjQY=</latexit>

<latexit sha1_base64="WadJpYEY2LSYAaPcUjCEEtELz2U=">AAAB8XicbVBNS8NAEJ3Ur1q/qh69LBbBg5REpHosePFYwX5gG8pmO2mXbjZhdyOU0H/hxYMiXv033vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKj9jPeijEhTvtlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxM6oMZwKnpV6qMaFsTIfYtVTSCLWfzS+ekjOrDEgYK1vSkLn6eyKjkdaTKLCdETUjvezNxP+8bmrCGz/jMkkNSrZYFKaCmJjM3icDrpAZMbGEMsXtrYSNqKLM2JBKNgRv+eVV0rqserVq7f6qUnfzOIpwAqdwDh5cQx3uoAFNYCDhGV7hzdHOi/PufCxaC04+cwx/4Hz+ACT5kIg=</latexit>

eω,0

q
<latexit sha1_base64="MXg03aTYy6+mdXpCGWNhGwTEncA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2m3bpZhN3J0Ip/QdePCji1X/kzX/jps1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GN5nffuLaiFg94CThfkSHSoSCUbTS/WOpX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboT6lGwSSflXqp4QllYzrkXUsVjbjxp/NLZ+TMKgMSxtqWQjJXf09MaWTMJApsZ0RxZJa9TPzP66YYXvtToZIUuWKLRWEqCcYke5sMhOYM5cQSyrSwtxI2opoytOFkIXjLL6+S1kXVq1Vrd5eV+mUeRxFO4BTOwYMrqMMtNKAJDEJ4hld4c8bOi/PufCxaC04+cwx/4Hz+AA+FjQY=</latexit>

<latexit sha1_base64="iW6XjabZHCT4RLWJ7cVz6p2qggc=">AAAB8XicbVBNS8NAEJ3Ur1q/qh69LBbBg5REpHosePFYwX5gG8pmO2mXbjZhdyOU0H/hxYMiXv033vw3btsctPXBwOO9GWbmBYng2rjut1NYW9/Y3Cpul3Z29/YPyodHLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb2d++wmV5rF8MJME/YgOJQ85o8ZKj9jPeijEhTftlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxM6oMZwKnpV6qMaFsTIfYtVTSCLWfzS+ekjOrDEgYK1vSkLn6eyKjkdaTKLCdETUjvezNxP+8bmrCGz/jMkkNSrZYFKaCmJjM3icDrpAZMbGEMsXtrYSNqKLM2JBKNgRv+eVV0rqserVq7f6qUnfzOIpwAqdwDh5cQx3uoAFNYCDhGV7hzdHOi/PufCxaC04+cwx/4Hz+ACZ+kIk=</latexit>

eω,1

... q
<latexit sha1_base64="MXg03aTYy6+mdXpCGWNhGwTEncA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2m3bpZhN3J0Ip/QdePCji1X/kzX/jps1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GN5nffuLaiFg94CThfkSHSoSCUbTS/WOpX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboT6lGwSSflXqp4QllYzrkXUsVjbjxp/NLZ+TMKgMSxtqWQjJXf09MaWTMJApsZ0RxZJa9TPzP66YYXvtToZIUuWKLRWEqCcYke5sMhOYM5cQSyrSwtxI2opoytOFkIXjLL6+S1kXVq1Vrd5eV+mUeRxFO4BTOwYMrqMMtNKAJDEJ4hld4c8bOi/PufCxaC04+cwx/4Hz+AA+FjQY=</latexit>

<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

eω,kω q
<latexit sha1_base64="MXg03aTYy6+mdXpCGWNhGwTEncA=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEqseCF49V7Ae0oWy2m3bpZhN3J0Ip/QdePCji1X/kzX/jps1BWx8MPN6bYWZekEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmTjVjDdZLGPdCajhUijeRIGSdxLNaRRI3g7GN5nffuLaiFg94CThfkSHSoSCUbTS/WOpX664VXcOskq8nFQgR6Nf/uoNYpZGXCGT1Jiu5yboT6lGwSSflXqp4QllYzrkXUsVjbjxp/NLZ+TMKgMSxtqWQjJXf09MaWTMJApsZ0RxZJa9TPzP66YYXvtToZIUuWKLRWEqCcYke5sMhOYM5cQSyrSwtxI2opoytOFkIXjLL6+S1kXVq1Vrd5eV+mUeRxFO4BTOwYMrqMMtNKAJDEJ4hld4c8bOi/PufCxaC04+cwx/4Hz+AA+FjQY=</latexit>

<latexit sha1_base64="Q3gJ5hUYrj42ERDbvKcwVvBl25E=">AAAB+HicbVBNS8NAEJ3Ur1o/WvXoJVgED1ISkeqx4MVjBVsLbQib7aRdutmE3Y1QQ3+JFw+KePWnePPfuGlz0NYHwzzem2FnX5BwprTjfFultfWNza3ydmVnd2+/Wjs47Ko4lRQ7NOax7AVEIWcCO5ppjr1EIokCjg/B5Cb3Hx5RKhaLez1N0IvISLCQUaKN5Neq6GcD5Px84udt5tfqTsOZw14lbkHqUKDt174Gw5imEQpNOVGq7zqJ9jIiNaMcZ5VBqjAhdEJG2DdUkAiVl80Pn9mnRhnaYSxNCW3P1d8bGYmUmkaBmYyIHqtlLxf/8/qpDq+9jIkk1Sjo4qEw5baO7TwFe8gkUs2nhhAqmbnVpmMiCdUmq4oJwV3+8irpXjTcZqN5d1lvOUUcZTiGEzgDF66gBbfQhg5QSOEZXuHNerJerHfrYzFasoqdI/gD6/MHwjyTHg==</latexit>

i→
<latexit sha1_base64="heb5UAhOvML8AY5anV5kKRyI/jQ=">AAAB63icbVBNS8NAEJ3Urxq/qh69LBbRU0mkVI8FLx4r2A9oQ9lsN+3S3U3Y3Qgl9C948aCIV/+QN/+NmzYHbX0w8Hhvhpl5YcKZNp737ZQ2Nre2d8q77t7+weFR5fiko+NUEdomMY9VL8SaciZp2zDDaS9RFIuQ0244vcv97hNVmsXy0cwSGgg8lixiBJtcYpeuO6xUvZq3AFonfkGqUKA1rHwNRjFJBZWGcKx13/cSE2RYGUY4nbuDVNMEkyke076lEguqg2xx6xxdWGWEoljZkgYt1N8TGRZaz0RoOwU2E73q5eJ/Xj810W2QMZmkhkqyXBSlHJkY5Y+jEVOUGD6zBBPF7K2ITLDCxNh48hD81ZfXSee65jdqjYd6tVkv4ijDGZzDFfhwA024hxa0gcAEnuEV3hzhvDjvzseyteQUM6fwB87nD5iZjUM=</latexit>

i
<latexit sha1_base64="GjSt58somH+uKTfwgBcMHhK5AFE=">AAAB6nicbVBNS8NAEJ2tXzV+VT16WSyCp5JIqR4LXjxWtB/QhrLZbtqlm03Y3Qgl9Cd48aCIV3+RN/+NmzYHbX0w8Hhvhpl5QSK4Nq77jUobm1vbO+VdZ2//4PCocnzS0XGqKGvTWMSqFxDNBJesbbgRrJcoRqJAsG4wvc397hNTmsfy0cwS5kdkLHnIKTFWeuCOM6xU3Zq7AF4nXkGqUKA1rHwNRjFNIyYNFUTrvucmxs+IMpwKNncGqWYJoVMyZn1LJYmY9rPFqXN8YZURDmNlSxq8UH9PZCTSehYFtjMiZqJXvVz8z+unJrzxMy6T1DBJl4vCVGAT4/xvPOKKUSNmlhCquL0V0wlRhBqbTh6Ct/ryOulc1bxGrXFfrzbrRRxlOINzuAQPrqEJd9CCNlAYwzO8whsS6AW9o49lawkVM6fwB+jzBzfejRI=</latexit>

...
<latexit sha1_base64="1QJpGxWsPw6rO46U/BQR6jvQV5Y=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0hEqseCF48V7Ae0oWy223bp7ibsToQS+he8eFDEq3/Im//GpM1BWx8MPN6bYWZeGEth0fO+ndLG5tb2Tnm3srd/cHhUPT5p2ygxjLdYJCPTDanlUmjeQoGSd2PDqQol74TTu9zvPHFjRaQfcRbzQNGxFiPBKOaS67qVQbXmud4CZJ34BalBgeag+tUfRixRXCOT1Nqe78UYpNSgYJLPK/3E8piyKR3zXkY1VdwG6eLWObnIlCEZRSYrjWSh/p5IqbJ2psKsU1Gc2FUvF//zegmOboNU6DhBrtly0SiRBCOSP06GwnCGcpYRyozIbiVsQg1lmMWTh+CvvrxO2leuX3frD9e1hlvEUYYzOIdL8OEGGnAPTWgBgwk8wyu8Ocp5cd6dj2VrySlmTuEPnM8ffgyNLQ==</latexit>

Figure 7: For an index i′ to be in endpointsℓ , there is an index i in endpointsℓ−1 and a substring q
that matches dℓ such that Q[i + 1, i′ ] matches eℓ,0 ⊙ q ⊙ eℓ,1 ⊙ . . . ⊙ q ⊙ eℓ,kℓ ⊙ q.
Since s is in leftmatchstatesq (v), there is an index i such that i is in prefixmatchq,l (v) and and s
is in leftmatchstates(v, i). Then there is a path p1 from θAh to s matching Q[i + 1, c(v)]. Since
s ∈ rightmatchstates(v, j), there is a path p2 from s to ϕAh matching Q[c(v) + 1, j − 1]. Then p1 ⊙ p2
is a path from θAh to ϕAh matching Q[i + 1, j − 1], so Q[i + 1, j − 1] matches eh . Since j ∈ occ q ,
then Q[j, j + |q| − 1] = Q[j, i′ ] is an occurrence of q, so Q[0, i′ ] matches E ⊙ eh ⊙ q.
Due to the correctness of the state-set transition algorithm, prefixmatchq (root 1 ) contains i if
and only if Q[0, i] matches e0 ⊙ q. Then, since prefixmatchq (root h ) = nextprefixmatchq,h−1 , it
follows from Lemma 8 that nextprefixmatchq,k contains the indices i′ such that Q[0, i′ ] matches
e0 ⊙ q ⊙ e1 ⊙ q ⊙ . . . ⊙ q ⊙ ek ⊙ q. Therefore also endpoints contains all indices i′ such that Q[0, i′ ]
matches e0 ⊙ q ⊙ e1 ⊙ q ⊙ . . . ⊙ q ⊙ ek ⊙ q for all substrings q that match e. Thus endpoints contains
all indices i′ such that Q[0, i′ ] matches e0 (e)1 e1 \1 e2 \1 e3 . . . ek \1. Now, if there is an index i′ in
endpoints such that i′ + 1 is in suffixset, then R = e0 (e)1 e1 \1 e2 \1 e3 . . . ek \1 ek+1 matches Q.
In summary, we have shown Theorem 1.

6

Ordered and Single-Nested Regular Expressions

In this section we extend our results to solve the matching problem for ordered, single-nested rewbs
in O(n2 m) time and O(nm) space. These rewbs are of the form R = E0 E1 . . . Et et+1,0 , where Eℓ is
a subexpression of the form Eℓ = eℓ,0 (dℓ )ℓ eℓ,1 \ℓ eℓ,2 \ℓ . . . \ℓ eℓ,kℓ \ℓ with eℓ,j and dℓ being regular
expressions.
We say that a string Q matches R if there exists a decomposition of Q into W0 W1 . . . Wt wt+1,0
such that Wℓ = wℓ,0 qℓ wℓ,1 qℓ . . . qℓ wℓ,kℓ qℓ where qℓ matches dℓ and wℓ,j matches eℓ,j .

6.1

Algorithm

This type of rewb is essentially a sequence of rewbs of the type we consider in Section 5. For each
Eℓ , we find matching substrings of dℓ as in Section 5, and for each matching substring we traverse
the kℓ + 1 corresponding interval trees, to find all indices i such that Q[0, i] matches E0 . . . Eℓ .
These indices are then used as prefix match indices for Eℓ+1
Preprocessing Let prefixset be the indices i such that Q[0, i] matches e0,0 , and suffixset be
←−−−−
the indices j such that Q[j, n − 1] matches et+1,0 . We build the TNFAs A0,0 and At+1,0 for e0,0 and
et+1,0 and use them to compute prefixset and suffixset.

16

<!-- page 17 -->

Processing Eℓ for ℓ = 0 . . . t Starting with E0 , we process each subexpression sequentially
as follows: First, we build the interval trees Tℓ,0 . . . Tℓ,kℓ of Q for eℓ,0 . . . eℓ,kℓ (except for T0,0 which
we will not need). We then construct Adℓ . Using the state-set transition algorithm on Adℓ , we
check whether dℓ matches the empty string ε. If so, we construct the TNFA for eℓ,0 eℓ,1 eℓ,2 . . . eℓ,kℓ .
For ℓ = 0 we use the state-set transition algorithm on Adℓ from the beginning of Q, and for ℓ > 0,
we use the state-set transition algorithm on Adℓ from i + 1 for every index i in endpointsℓ−1 . This
way, we find all indices matching E0 . . . Eℓ when the capturing group matches ε. We add these
indices to endpointsℓ . We then find all matching substrings of dℓ one at a time by using the same
method as in Section 5. Given a substring q that matches dℓ , we traverse the kℓ + 1 interval trees
for eℓ,0 . . . eℓ,kℓ in kℓ + 1 rounds. Let root ℓ,h be the root of the interval tree Tℓ,h for eℓ,h . In round
h we do as follows: First, we compute prefixmatchq (root ℓ,h ). We do this according to the following
4 cases:
Case 1: ℓ = 0 and h = 0. We do not compute prefixmatchq (root ℓ,h ) or traverse an interval tree in
this case.
Case 2: ℓ = 0 and h = 1. We compute prefixmatchq (root ℓ,h ) from prefixset and occ q by iterating
through the indices i in occ q , and letting i+|q| ∈ prefixmatchq (root h ) if and only if i ∈ prefixset.
Case 3: ℓ > 0 and h = 0. We use endpointsℓ−1 as prefixmatchq (root ℓ,h )
Case 4: Otherwise. We use nextprefixmatchq,h−1 as prefixmatchq (root ℓ,h )
We then initialize an empty set nextprefixmatchq,h and start the traversal of Tℓ,h . In node v of
Tℓ,h we do the following four steps (These are identical to Section 5):
Step 1. We partition prefixmatchq (v) into prefixmatchq,l (v) and prefixmatchq,r (v), as well as occ q (l(v), r(v))
into occ q (l(v), c(v)) and occ q (c(v) + 1, r(v)).
Step 2. We compute leftmatchstatesq (v) as the union of the sets leftmatchstates(v, i) for all indices
i in prefixmatchq,l (v).
Step 3. We compute the intersection of leftmatchstatesq (v) and rightmatchstates(v, j) for every j ∈
occ q (c(v)+1, r(v)). If the intersection is not empty, then we add j−1+|q| to nextprefixmatchq,h .
Step 4. We continue the traversal of Tℓ,h . Let u and w be the left and right child of v, respectively.
If neither occ q (l(v), c(v)) nor prefixmatchq,l (v) is empty, recurse on u with occ q (l(u), r(u)) =
occ q (l(v), c(v)) and prefixmatchq (u) = prefixmatchq,l (v). Likewise if neither occ q (c(v)+ 1, r(v))
nor prefixmatchq,r (v) is empty, recurse on w with occ q (l(w), r(w)) = occ q (c(v) + 1, r(v)) and
prefixmatchq (w) = prefixmatchq,r (v).
After traversing Tℓ,0 . . . Tℓ,kℓ for a substring q, we add the indices of nextprefixmatchq,kℓ to the
set endpointsℓ , and find the next matching substring. Once we have traversed Tℓ,0 . . . Tℓ,kℓ for all
substrings q that match dℓ , endpointsℓ contains all indices i for which Q[0, i] matches E0 . . . Eℓ .
Postprocessing After processing Et , and computing endpointst , we check for every index i
in endpointst whether i + 1 is in suffixset. If such an i exists, R matches Q. Otherwise, R does not
match Q.
17

<!-- page 18 -->

6.2

Analysis

Let mℓ,h be the length of the regular expression eℓ,h , and mdℓ be the length of the regular expression
dℓ such that
m=

kℓ
t
t
X
X
X

(mℓ,0 + mdℓ + mℓ,1 + 1 + . . . + 1 + mℓ,kℓ ) + mt+1,0 =
mdℓ + O(kℓ ) +
mℓ,h + mt+1,0
ℓ=0

ℓ=0

h=0

←−−−−
We first analyze the time. Building the TNFAs A0 and At+1,0 takes O(m0,0 + mt+1,0 ) time, and
using them to compute prefixset and suffixset takes O(n(m0,0 + mt+1,0 )). We build interval trees
Tℓ,h for eℓ,h for ℓ = 0 . . . t and h = 0 . . . kℓ except for e0,0 . By Lemma 3, it takes O(n2 mℓ,h ) to
P
P ℓ
mℓ,h ) − m0,0 )) time. Constructing
build Tℓ,h , so for all interval trees, we spend O(n2 ( tℓ=0 ( kh=0
Adℓ and checking if the empty string matches dℓ takes O(mdℓ ) time. Constructing the TNFA for
P ℓ
mℓ,h ) time, and using the state-set transition algorithm on it from
eℓ,0 eℓ,1 eℓ,2 . . . eℓ,kℓ takes O( kh=0
P ℓ
mℓ,h )).
every index in endpointsℓ−1 takes O(n2 ( kh=0
2
Finding
Pt matching substrings for each dℓ takes O(n (mdℓ ) by the same argument as in Section 5,
2
so O(n
ℓ=0 mdℓ ) in total. Computing the prefix match indices is done as part of the interval tree
traversals, except for the case where ℓ = 0 and h = 1. In this case, we compute it in O(|occ q |) time
for each substring q that matches d0 , so it takes O(n2 ) in total. The traversal of each interval tree
is identical to that of Section 5, and takes O(n2 mℓ,h ) for Tℓ,h . Recall,
Ptwe do not traverse an
Pt that
2
interval tree for e0,0 . Traversing all interval trees then takes O(n ( ℓ=0 ( h=0 mℓ,h ) − m0,0 )). In
total, the algorithm uses
2

O(n

t
X

kℓ
t X
X
mℓ,h ) − m0,0 ) + n(m0,0 + mt+1,0 ))
mdℓ + n ( (
2

ℓ=0

=O(n2

t
X

mdℓ +

ℓ=0

ℓ=0 h=0
k
ℓ
X

mℓ,h



+ n(m0,0 + mt+1,0 ))

h=0

=O(n2 m)
time.
←−−−−
We now analyze the space. The A0 and At+1,0 take O(m0,0 + mt+1,0 ) space, and the interval
trees Tℓ,h for eℓ,h for ℓ = 0 . . . t and h = 0 . . . kℓ except for e0,0 use O(nmℓ,h ) by Lemma 3 for Tℓ,h so
P
P ℓ
for all interval trees, we use O(n( tℓ=0 ( kh=0
m ) − m0,0 )) space. The TNFA Adℓ uses O(mdℓ ),
Pkℓ,h
ℓ
and the TNFA for eℓ,0 eℓ,1 eℓ,2 . . . eℓ,kℓ uses O( h=0
mℓ,h ) space. Finding matching substrings for
each dℓ takes O(n(mdℓ ) by the same argument as in Section 5. The total space for the algorithm
is then
O(n

t
X

kℓ
t X
X
mdℓ + n( (
mℓ,h ) − m0,0 ) + m0,0 + mt+1,0 )

ℓ=0

=O(n

t
X
ℓ=0

mdℓ +

ℓ=0 h=0
kℓ
X

mℓ,h



+ (m0,0 + mt+1,0 ))

h=0

=O(nm).

18

<!-- page 19 -->

6.3

Correctness

Lemma 9. Given that endpointsℓ−1 contains index i′ if and only if Q[0, i′ ] matches E0 . . . Eℓ−1 ,
then endpointsℓ contains an index i′ if and only if Q[0, i′ ] matches E0 . . . Eℓ .
Proof. We have assumed, that endpointsℓ−1 contains index i′ if and only if Q[0, i′ ] matches E0 . . . Eℓ−1 .
Since the traversal of each interval tree is the same as in Section 5, then we can apply Lemma 8
repeatedly for a substring q that matches dℓ to show, that nextprefixmatchq,kℓ contains the indices
i′ which match E0 . . . Eℓ−1 eℓ,0 ⊙ q ⊙ eℓ,1 ⊙ q ⊙ . . . ⊙ q ⊙ eℓ,kℓ ⊙ q. Doing this for all substrings q of
Q that match dℓ , we show that endpointsℓ contains i′ if and only if Q[0, i′ ] matches E0 . . . Eℓ .
Since the algorithm for ℓ = 0 is the algorithm from Section 5, the argument from Section 5.4
holds, and endpoints0 contains i′ if and only if Q[0, i′ ] matches E0 . Then, we apply Lemma 9,
to show that endpointst contains i′ if and only if Q[0, i′ ] matches Et . Lastly, if there exists an
i′ ∈ endpointst such that i′ + 1 is in suffixset, then Q matches R. In summary, we have shown
Theorem 2.

References
[1] Alfred V. Aho. Algorithms for finding patterns in strings. In Algorithms and Complexity,
Handbook of Theoretical Computer Science, pages 255–300. Elsevier, Amsterdam, 1990. doi:
10.1016/B978-0-444-88071-0.50010-2.
[2] Arturs Backurs and Piotr Indyk. Which regular expression patterns are hard to match? In
Proc. 57th FOCS, pages 457–466, 2016.
[3] Philip Bille. New algorithms for regular expression matching. In Proc. 33rd ICALP, pages
643–654, 2006.
[4] Philip Bille and Martin Farach-Colton. Fast and compact regular expression matching. Theoret. Comput. Sci., 409:486 – 496, 2008.
[5] Philip Bille and Mikkel Thorup. Faster regular expression matching. In Proc. 36th ICALP,
pages 171–182, 2009.
[6] Philip Bille and Mikkel Thorup. Regular expression matching with multi-strings and intervals.
In Proc. 21st SODA, 2010.
[7] Karl Bringmann, Allan Grønlund, and Kasper Green Larsen. A dichotomy for regular expression membership testing. In Proc. 58th FOCS, pages 307–318, 2017.
[8] Taolue Chen, Alejandro Flores-Lamas, Matthew Hague, Zhilei Han, Denghang Hu, Shuanglong
Kan, Anthony W Lin, Philipp Rümmer, and Zhilin Wu. Solving string constraints with regexdependent functions through transducers with priorities and variables. In Proc. 49th POPL,
2022.
[9] Martin Farach-Colton, Paolo Ferragina, and Shanmugavelayutham Muthukrishnan. On the
sorting-complexity of suffix tree construction. J. ACM, 47(6):987–1011, 2000.

19

<!-- page 20 -->

[10] Dominik D Freydenberger and Markus L Schmid. Deterministic regular expressions with backreferences. J. Comput. Syst. Sci., 105:1–39, 2019.
[11] Victor M. Glushkov. The abstract theory of automata. Russian Math. Surveys, 16(5):1–53,
1961.
[12] Vojtěch Havlena, Lukáš Holı́k, Ondřej Lengál, Jan Vašák, and Sabı́na Gulčı́ková. Towards
efficient matching of regexes with backreferences using register set automata. In Proc. 47th
PLDI, pages 855–880, 2026.
[13] Soh Kumabe and Yuya Uezato. On the Complexity of the Matching Problem of Regular
Expressions with Backreferences. In Proc. 53rd ICALP, pages 135:1–135:25, 2026.
[14] Kim S. Larsen. Regular expressions with nested levels of back referencing form a hierarchy.
Inform. Process Lett., 65(4):169–172, 1998.
[15] Blake Loring, Duncan Mitchell, and Johannes Kinder. Sound regular expression semantics for
dynamic symbolic execution of javascript. In Proc. 40th PLDI, pages 425–438, 2019.
[16] R. McNaughton and H. Yamada. Regular expressions and state graphs for automata. IRE
Trans. on Electronic Computers, 9(1):39–47, 1960.
[17] E. W. Myers. A four-russian algorithm for regular expression pattern matching. J. ACM,
39(2):430–448, 1992.
[18] Kedar Namjoshi and Girija Narlikar. Robust and fast pattern matching for intrusion detection.
In Proc. 20th INFOCOM, pages 1–9, 2010.
[19] Taisei Nogami, Yoshiki Nakamura, and Tachio Terauchi. Hardness of regular expression matching with extensions. arXiv preprint arXiv:2601.03020, 2026.
[20] Taisei Nogami and Tachio Terauchi. Efficient Matching of Some Fundamental Regular Expressions with Backreferences. In Proc. 50th MFCS 2025, pages 81:1–81:19, 2025.
[21] Philipp Schepper. Fine-grained complexity of regular expression pattern matching and membership. In Proc. 28th ESA, 2020.
[22] Markus L Schmid. Conjunctive regular path queries with string variables. In Proc. 39th PODS,
pages 361–374, 2020.
[23] K. Thompson. Regular expression search algorithm. Commun. ACM, 11:419–422, 1968.
[24] Yuya Uezato. Matching Regular-Typed Pattern Languages: Quadratic-Time Algorithms. In
Proc. 37th CPM 2026, pages 11:1–11:20, 2026.

20
