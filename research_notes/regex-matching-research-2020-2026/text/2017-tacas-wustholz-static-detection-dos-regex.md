---
file: ../pdfs/2017-tacas-wustholz-static-detection-dos-regex.pdf
citation: "Static Detection of DoS Vulnerabilities in Programs that use Regular Expressions (Rexploiter). Wüstholz, Olivo, Heule, Dillig. TACAS 2017"
doi: 
source: "https://arxiv.org/pdf/1701.04045"
topics: [redos_backrefs_verification]
pages: 26
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Static Detection of DoS Vulnerabilities in
Programs that use Regular Expressions
(Extended Version)

arXiv:1701.04045v1 [cs.CR] 15 Jan 2017

Valentin Wüstholz, Oswaldo Olivo, Marijn J. H. Heule, and Isil Dillig
The University of Texas at Austin
{valentin, olivo, marijn, isil}@cs.utexas.edu
Abstract. In an algorithmic complexity attack, a malicious party takes
advantage of the worst-case behavior of an algorithm to cause denial-ofservice. A prominent algorithmic complexity attack is regular expression
denial-of-service (ReDoS ), in which the attacker exploits a vulnerable
regular expression by providing a carefully-crafted input string that triggers worst-case behavior of the matching algorithm. This paper proposes
a technique for automatically finding ReDoS vulnerabilities in programs.
Specifically, our approach automatically identifies vulnerable regular expressions in the program and determines whether an “evil" input string
can be matched against a vulnerable regular expression. We have implemented our proposed approach in a tool called Rexploiter and found
41 exploitable security vulnerabilities in Java web applications.

1

Introduction

Regular expressions provide a versatile mechanism for parsing and validating
input data. Due to their flexibility, many developers use regular expressions to
validate passwords or to extract substrings that match a given pattern. Hence,
many languages provide extensive support for regular expression matching.
While there are several algorithms for determining membership in a regular
language, a common technique is to construct a non-deterministic finite automaton (NFA) and perform backtracking search over all possible runs of this NFA.
Although simple and flexible, this strategy has super-linear (in fact, exponential)
complexity and is prone to a class of algorithmic complexity attacks [14]. For some
regular expressions (e.g., (a|b)*(a|c)*), it is possible to craft input strings that
could cause the matching algorithm to take quadratic time (or worse) in the size
of the input. For some regular expressions (e.g, (a+)+), one can even generate
input strings that could cause the matching algorithm to take exponential time.
Hence, attackers exploit the presence of vulnerable regular expressions to launch
so-called regular expression denial-of-service (ReDoS) attacks.
ReDoS attacks have been shown to severely impact the responsiveness and
availability of applications. For example, the .NET framework was shown to be
vulnerable to a ReDoS attack that paralyzed applications using .NET’s default
validation mechanism [2]. Furthermore, unlike other DoS attacks that require
thousands of machines to bring down critical infrastructure, ReDoS attacks can
1

<!-- page 2 -->

be triggered by a single malicious user input. Consequently, developers are responsible for protecting their code against such attacks, either by avoiding the
use of vulnerable regular expressions or by sanitizing user input.
Unfortunately, protecting an application against ReDoS attacks can be nontrivial in practice. Often, developers do not know which regular expressions are
vulnerable or how to rewrite them in a way that avoids super-linear complexity.
In addition, it is difficult to implement a suitable sanitizer without understanding
the class of input strings that trigger worst-case behavior. Even though some
libraries (e.g., the .Net framework) allow developers to set a time limit for
regular expression matching, existing solutions do not address the root cause of
the problem. As a result, ReDoS vulnerabilities are still being uncovered in many
important applications. For instance, according to the National Vulnerability
Database (NVD), there are over 150 acknowledged ReDoS vulnerabilities, some
of which are caused by exponential matching complexity (e.g., [2,3]) and some
of which are characterized by super-linear behavior (e.g., [1,4,5]).
In this paper, we propose a static technique for automatically uncovering
DoS vulnerabilities in programs that use regular expressions. There are two
main technical challenges that make this problem difficult: First, given a regular expression E, we need to statically determine the worst-case complexity of
matching E against an arbitrary input string. Second, given an application A
that contains a vulnerable regular expression E, we must statically determine
whether there can exist an execution of A in which E can be matched against
an input string that could cause super-linear behavior.
We solve these challenges by developing a two-tier algorithm that combines
(a) static analysis of regular expressions with (b) sanitization-aware taint analysis at the source code level. Our technique can identify both vulnerable regular
expressions that have super-linear complexity (quadratic or worse), as well as
hyper-vulnerable ones that have exponential complexity. In addition and, most
importantly, our technique can also construct an attack automaton that captures all possible attack strings. The construction of attack automata is crucial
for reasoning about input sanitization at the source-code level.
To summarize, this paper makes the following contributions:

– We present algorithms for reasoning about worst-case complexity of NFAs.
Given an NFA A, our algorithm can identify whether A has linear, superlinear, or exponential time complexity and can construct an attack automaton that accepts input strings that could cause worst-case behavior for A.
– We describe a program analysis to automatically identify ReDoS vulnerabilities. Our technique uses the results of the regular expression analysis to
identify sinks and reason about input sanitization using attack automata.
– We use these ideas to build an end-to-end tool called Rexploiter for finding
vulnerabilities in Java. In our evaluation, we find 41 security vulnerabilities
in 150 Java programs collected from Github with a 11% false positive rate.
2

<!-- page 3 -->

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

public class RegExV alidat or {
boolean validEmail ( String t ) { return t.matches ( " . + @. +\\ . [a - z ]+ " ); }
boolean validComment ( String t ) {
return ! t.matches ( " (\\ p { Blank }*(\\ r ?\\ n )\\ p { Blank }*)+ " ); }
boolean safeComment ( String t ) { return t.matches ( " ([^\/ < >])+ " ); }
boolean validUrl ( String t ) {
return t.matches ( " www \\ .shoppers \\ .com / . +/ . +/ . +/ . +/ " ); }
}
public class C o m m e n t F o r m V a l i d a t o r implements Validator {
private Admin admin ;
public void validate ( CommentForm form , Errors errors ) {
String senderEmail = f o r m . g e t S e n d e r E m a i l ();
String productUrl = f o r m . g e t P r o d u c t U r l ();
String comment = f orm .g et C om me n t ();
if (! R e g E x V a l i d a t o r . v a l i d E m a i l ( ad min.ge tEmail ())) return ;
if ( s e n d e r E m a i l . l e n g t h () <= 254) {
if ( R e g E x V a l i d a t o r . v a l i d E m a i l ( senderEmail )) ... }
if ( p r o d u c t U r l . s p l i t ( " / " ) .length == 5) {
if ( R e g E x V a l i d a t o r . v a l i d U r l ( productUrl )) ... }
if ( R e g E x V a l i d a t o r . s a f e C o m m e n t ( comment )) {
if ( R e g E x V a l i d a t o r . v a l i d C o m m e n t ( comment )) ... }
}

Fig. 1: Motivating example containing ReDoS vulnerabilities

2

Overview

We illustrate our technique using the code snippet shown in Fig. 1, which shows
two relevant classes, namely RegExValidator, that is used to validate that certain strings match a given regular expression, and CommentFormValidator, that
checks the validity of a comment form filled out by a user. In particular, the
comment form submitted by the user includes the user’s email address, the URL
of the product about which the user wishes to submit a comment1 , and the text
containing the comment itself. We now explain how our technique can determine
whether this program contains a denial-of-service vulnerability.
Regular expression analysis. For each regular expression in the program, we
construct its corresponding NFA and statically analyze it to determine whether
its worst-case complexity is linear, super-linear, or exponential. For our running
example, the NFA complexity analysis finds instances of each category. In particular, the regular expression used at line 5 has linear matching complexity,
while the one from line 4 has exponential complexity. The regular expressions
from lines 2 and 7 have super-linear (but not exponential) complexity. Fig. 2
plots input size against running time for the regular expressions from lines 2
and 4 respectively. For the super-linear and exponential regular expressions, our
technique also constructs an attack automaton that recognizes all strings that
cause worst-case behavior. In addition, for each regular expression, we determine
a lower bound on the length of any possible attack string using dynamic analysis.
Program analysis. The presence of a vulnerable regular expression does not
necessarily mean that the program itself is vulnerable. For instance, the vulnerable regular expression may not be matched against an attacker-controlled string,
1

Due to the store’s organization, the URL is expected to be of the form
www.shoppers.com/Dept/Category/Subcategory/product-id/

3

<!-- page 4 -->

Fig. 2: Matching time against malicious string size for vulnerable (left) and hypervulnerable (right) regular expressions from Fig. 1.

or the program may take measures to prevent the user from supplying a string
that is an instance of the attack pattern. Hence, we also perform static analysis
at the source code level to determine if the program is actually vulnerable.
Going back to our example, the validate procedure (lines 11–22) calls
validEmail to check whether the website administrator’s email address is valid.
Even though validEmail contains a super-linear regular expression, line 15 does
not contain a vulnerability because the administrator’s email is not supplied by
the user. Since our analysis tracks taint information, it does not report line 15 as
being vulnerable. Now, consider the second call to validEmail at line 17, which
matches the vulnerable regular expression against user input. However, since the
program bounds the size of the input string to be at most 254 (which is smaller
than the lower bound identified by our analysis), line 17 is also not vulnerable.
Next, consider the call to validUrl at line 19, where productUrl is a user
input. At first glance, this appears to be a vulnerability because the matching
time of the regular expression from line 4 against a malicious input string grows
quite rapidly with input size (see Fig. 2). However, the check at line 18 actually prevents calling validUrl with an attack string: Specifically, our analysis
determines that attack strings must be of the form www.shoppers.com·/b ·/+ ·x,
where x denotes any character and b is a constant inferred by our analysis (in
this case, much greater than 5). Since our program analysis also reasons about
input sanitization, it can establish that line 19 is safe.
Finally, consider the call to validComment at line 21, where comment is again
a user input and is matched against a regular expression with exponential complexity. Now, the question is whether the condition at line 20 prevents comment
from conforming to the attack pattern \n\t\n\t( \t\n\t)k a. Since this is not
the case, line 21 actually contains a serious DoS vulnerability.
Summary of challenges. This example illustrates several challenges we must
address: First, given a regular expression E, we must reason about the worstcase time complexity of its corresponding NFA. Second, given vulnerable regular
expression E, we must determine whether the program allows E to be matched
against a string that is (a) controlled by the user, (b) is an instance of the attack
pattern for regular expression E, and (c) is large enough to cause the matching
algorithm to take significant time.
4

<!-- page 5 -->

Our approach solves these challenges by combining complexity analysis of
NFAs with sanitization-aware taint analysis. The key idea that makes this combination possible is to produce an attack automaton for each vulnerable NFA.
Without such an attack automaton, the program analyzer cannot effectively
determine whether an input string can correspond to an attack string.
As shown in Fig. 3, the Rexploiter toolchain incorporates
Regex
Static regex
both static and dynamic reguextraction
analysis
lar expression analysis. The static
analysis creates attack patterns
Dynamic regex
s0 · sk · s1 and dynamic analyanalysis
sis infers a lower bound b on the Program
Vulnerabilities
number of occurrences of s in orStatic program
der to exceed a minimum runtime
analysis
threshold. The program analysis
uses both the attack automaton
Fig. 3: Overview of our approach
and the lower bound b to reason
about input sanitization.

3

Preliminaries

This section presents some useful background and terminology.
Definition 1. (NFA) An NFA A is a 5-tuple (Q, Σ, ∆, q0 , F ) where Q is a
finite set of states, Σ is a finite alphabet of symbols, and ∆ : Q × Σ → 2Q is the
transition function. Here, q0 ∈ Q is the initial state, and F ⊆ Q is the set of
accepting states. We say that (q, l, q 0 ) is a transition via label l if q 0 ∈ ∆(q, l).
An NFA A accepts a string s = a0 a1 . . . an iff there exists a sequence of
states q0 , q1 , ..., qn such that qn ∈ F and qi+1 ∈ ∆(qi , ai ). The language of A,
denoted L(A), is the set of all strings that are accepted by A. Conversion from
a regular expression to an NFA is sometimes referred to as compilation and can
be achieved using well-known techniques, such as Thompson’s algorithm [25].
In this paper, we assume that membership in a regular language L(E) is
decided through a worst-case exponential algorithm that performs backtracking
search over possible runs of the NFA representing E. While there exist lineartime matching algorithms (e.g., based on DFAs), many real-world libraries employ backtracking search for two key reasons: First, the compilation of a regular
expression is much faster using NFAs and uses much less memory (DFA’s can be
exponentially larger). Second, the backtracking search approach can handle regular expressions containing extra features like backreferences and lookarounds.
Thus, many widely-used libraries (e.g., java.util.regex, Python’s standard
library) employ backtracking search for regular expression matching.
In the remainder of this paper, we will use the notation A∗ and A∅ to denote
the NFA that accepts Σ ∗ and the empty language respectively. Given two NFAs
5

<!-- page 6 -->

A1 and A2 , we write A1 ∩ A2 , A1 ∪ A2 , and A1 · A2 to denote automata intersection, union, and concatenation. Finally, given an automaton A, we write A
to represent its complement, and we use the notation A+ to represent the NFA
that recognizes exactly the language {sk | k ≥ 1 ∧ s ∈ L(A)}.
Definition 2. (Path) Given an NFA A = (Q, Σ, ∆, q0 , F ), a path π of A is a
sequence of transitions (q1 , `1 , q2 ), . . . , (qm−1 , `m−1 , qm ) where qi ∈ Q, `i ∈ Σ,
and qi+1 ∈ ∆(qi , `i ). We say that π starts in qi and ends at qm , and we write
labels(π) to denote the sequence of labels (`1 , . . . , `m−1 ).

4

Detecting Hyper-Vulnerable NFAs

In this section, we explain our technique for determining if an NFA is hypervulnerable and show how to generate an attack automaton that recognizes exactly
the set of attack strings.
Definition 3. (Hyper-Vulnerable NFA) An NFA A = (Q, Σ, ∆, q0 , F ) is
hyper-vulnerable iff there exists a backtracking search algorithm Match over the
paths of A such that the worst-case complexity of Match is exponential in the
length of the input string.
We will demonstrate that an NFA A is hyper-vulnerable by showing that
there exists a string s such that the number of distinct matching paths πi from
state q0 to a rejecting state qr with labels(πi ) = s is exponential in the length of
s. Clearly, if s is rejected by A, then Match will need to explore each of these
exponentially many paths. Furthermore, even if s is accepted by A, there exists a
backtracking search algorithm (namely, the one that explores all rejecting paths
first) that results in exponential worst-case behavior.
Theorem 1. An NFA A = (Q, Σ, ∆, q0 , F ) is hyper-vulnerable iff there exists
a pivot state q ∈ Q and two distinct paths π1 , π2 such that (i) both π1 , π2 start
and end at q, (ii) labels(π1 ) = labels(π2 ), and (iii) there is a path πp from initial
state q0 to q, and (iv) there is a path πs from q to a state qr 6∈ F .
Proof. The sufficiency argument is laid out below, and the necessity argument
can be found in the appendix.
To gain intuition about hyperlabels(π1 )=labels(π2 )
π1
vulnerable NFAs, consider Fig. 4 illusprefix
suffix
trating the conditions of Theorem 1.
q0
q
qr
First, a hyper-vulnerable NFA must
πp
πs
pivot
contain a pivot state q, such that,
starting at q, there are two different
π2
ways (namely, π1 , π2 ) of getting back Fig. 4: Hyper-vulnerable NFA pattern
to q on the same input string s (i.e.,
labels(π1 )). Second, the pivot state q should be reachable from the initial state
q0 , and there must be a way of reaching a rejecting state qr from q.
6

<!-- page 7 -->

a

q0

a
a

q

b

q2
qr

a

q0

b

a

a

q1

b

q3

Fig. 5: A hyper-vulnerable NFA (left) and an attack automaton (right).
To understand why these conditions cause exponential behavior, consider a
string of the form s0 ·sk ·s1 , where s0 is the attack prefix given by labels(πp ), s1 is
the attack suffix given by labels(πs ), and s is the attack core given by labels(π1 ).
Clearly, there is an execution path of A in which the string s0 · sk · s1 will be
rejected. For example, πp · π1k · πs is exactly such a path.
Now, consider a string s0 ·
Algorithm 1 Hyper-vulnerable NFA
sk+1 · s1 that has an additional
1: function AttackAutomaton(A)
instance of the attack core s in
2:
assume A = (Q, Σ, ∆, q0 , F )
the middle, and suppose that
È
∅
3:
A ←A
there are n possible executions
4:
for qi ∈ Q do
of A on the prefix s0 · sk that
È
5:
Ai ← AttackForPivot(A, qi )
end in q. Now, for each of
6:
AÈ ← AÈ ∪ AÈi
these n executions, there are
È
7:
return A
two ways to get back to q after
8: function AttackForPivot(A, q)
reading s: one that takes path
9:
assume A = (Q, Σ, ∆, q0 , F )
È
∅
π1 and another that takes path
10:
A ←A
π
11:
for (q, l, q1 ), (q, l, q2 ) ∈ ∆ ∧ q1 6= q2 do
2 . Therefore, there are 2n pos12:
A1 ← LoopBack(A, q, l, q1 )
sible executions of A that end
13:
A2 ← LoopBack(A, q, l, q2 )
in q. Furthermore, the match14:
Ap ← (Q, Σ, ∆, q0 , {q})
ing algorithm will (in the worst
15:
As ← (Q, Σ, ∆, q, F )
case) end up exploring all of
16:
AÈ ← AÈ ∪ (Ap · (A1 ∩ A2 )+ · As )
these 2n executions since there
17:
return AÈ
is a way to reach the rejecting
18: function LoopBack(A, q, l, q 0 )
state qr . Hence, we end up dou19:
assume A = (Q, Σ, ∆, q0 , F )
bling
the running time of the al20:
q ? ← NewState(Q)
gorithm
every time we add an
21:
Q0 ← Q ∪ q ? ; ∆0 ← ∆ ∪ (q ? , l, q 0 )
0
0
?
instance
of the attack core s to
22:
return (Q , Σ, ∆ , q , {q})
the middle of the input string.
Example 1. The NFA in Fig. 5 (left) is hyper-vulnerable because there exist two
different paths π1 = (q, a, q), (q, a, q) and π2 = (q, a, q0 ), (q0 , a, q) that contain
the same labels and that start and end in q. Also, q is reachable from q0 , and
the rejecting state qr is reachable from q. Attack strings for this NFA are of the
form a · (a · a)k · b, and the attack automaton is shown in Fig. 5 (right).
We now use Theorem 1 to devise Algorithm 1 for constructing the attack
automaton AÈ for a given NFA. The key idea of our algorithm is to search for
all possible pivot states qi and construct the attack automaton AÈi for state qi .
The full attack automaton is then obtained as the union of all AÈi . Note that
7

<!-- page 8 -->

Algorithm 1 can be used to determine if automaton A is vulnerable: A exhibits
worst-case exponential behavior iff the language accepted by AÈ is non-empty.
In Algorithm 1, most of the real work is done by the AttackForPivot
procedure, which constructs the attack automaton for a specific state q: Given
a pivot state q, we want to find two different paths π1 , π2 that loop back to q
and that have the same set of labels. Towards this goal, line 11 of Algorithm 1
considers all pairs of transitions from q that have the same label (since we must
have labels(π1 ) = labels(π2 )).
Now, let us consider a pair of transitions τ1 = (q, l, q1 ) and τ2 = (q, l, q2 ). For
each qi (i ∈ {1, 2}), we want to find all strings that start in q, take transition
τi , and then loop back to q. In order to find all such strings S, Algorithm 1
invokes the LoopBack function (lines 18–22), which constructs an automaton
A0 that recognizes exactly S. Specifically, the final state of A0 is q because we
want to loop back to state q. Furthermore, A0 contains a new initial state q ∗
(where q ∗ 6∈ Q) and a single outgoing transition (q ∗ , l, qi ) out of q ∗ because we
only want to consider paths that take the transition to qi first. Hence, each Ai
in lines 12–13 of the AttackForPivot procedure corresponds to a set of paths
that loop back to q through state qi . Observe that, if a string s is accepted by
A1 ∩ A2 , then s is an attack core for pivot state q.
We now turn to the problem of computing the set of all attack prefixes and
suffixes for pivot state q: In line 14 of Algorithm 1, Ap is the same as the original
NFA A except that its only accepting state is q. Hence, Ap accepts all attack
prefixes for pivot q. Similarly, As is the same as A except that its initial state is
q instead of q0 ; thus, As accepts all attack suffixes for q.
Finally, let us consider how to construct the full attack automaton AÈ for q.
As explained earlier, all attack strings are of the form s1 · sk · s2 where s1 is the
attack prefix, s is the attack core, and s2 is the attack suffix. Since Ap , A1 ∩ A2 ,
and As recognize attack prefixes, cores, and suffixes respectively, any string that
is accepted by Ap · (A1 ∩ A2 )+ · As is an attack string for the original NFA A.
Theorem 2. (Correctness of Algorithm 1)2 Let AÈ be the result of calling
AttackAutomaton(A) for NFA A = (Q, Σ, ∆, q0 , F ). For every s ∈ L(AÈ ),
there exists a rejecting state qr ∈ Q \ F s.t. the number of distinct paths πi from
q0 to qr with labels(πi ) = s is exponential in the number of repetitions of the
attack core in s.

5

Detecting Vulnerable NFAs

So far, we only considered the problem of identifying NFAs whose worst-case
running time is exponential. However, in practice, even NFAs with super-linear
complexity can cause catastrophic backtracking. In fact, many acknowledged
ReDoS vulnerabilities (e.g., [1,4,5]) involve regular expressions whose matching
complexity is “only" quadratic. Based on this observation, we extend the techniques from the previous section to statically detect NFAs with super-linear time
complexity. Our solution builds on insights from Section 4 to construct an attack
automaton for this larger class of vulnerable regular expressions.
2

The proofs of Theorems 2 and 4 are given in the appendix.

8

<!-- page 9 -->

π1

q0

prefix
πp

q

π3
π2

q0

labels(π1 ) = labels(π2 )
= labels(π3 )
suffix
qr
πs

pivot

Fig. 6: General pattern characterizing vulnerable NFAs
5.1

Understanding Super-Linear NFAs

Before we present the algorithm for detecting super-linear NFAs, we provide a
theorem that explains the correctness of our solution.
Definition 4. (Vulnerable NFA) An NFA A = (Q, Σ, ∆, q0 , F ) is vulnerable
iff there exists a backtracking search algorithm Match over the paths of A such
that the worst-case complexity of Match is at least quadratic in the length of
the input string.
Theorem 3. An NFA A = (Q, Σ, ∆, q0 , F ) is vulnerable iff there exist two
states q ∈ Q ( the pivot), q 0 ∈ Q, and three paths π1 , π2 , and π3 (where π1 6= π2 )
such that (i) π1 starts and ends at q, (ii) π2 starts at q and ends at q 0 , (iii) π3
starts and ends at q 0 , (iv) labels(π1 ) = labels(π2 ) = labels(π3 ), and (v) there is
a path πp from q0 to q, (vi) there is a path πs from q 0 to a state qr 6∈ F .
Proof. The necessity argument can be found in the appendix. The sufficiency
argument is in the following text.
Fig. 6 illustrates the intuition behind the conditions above. The distinguishing characteristic of a super-linear NFA is that it contains two states q, q 0 such
that q 0 is reachable from q on input string s, and it is possible to loop back from
q and q 0 to the same state on string s. In addition, just like in Theorem 1, the
pivot state q needs to be reachable from the initial state, and a rejecting state qr
must be reachable from q 0 . Observe that any automaton that is hyper-vulnerable
according to Theorem 1 is also vulnerable according to Theorem 3. Specifically,
consider an automaton A with two distinct paths π1 , π2 that loop around q. In
this case, if we take q 0 to be q and π3 to be π1 , we immediately see that A also
satisfies the conditions of Theorem 3.
To understand why the conditions of Theorem 3 imply super-linear time
complexity, let us consider a string of the form s0 · sk · s1 where s0 is the attack
prefix given by labels(πp ), s1 is the attack suffix given by labels(πs ), and s is
the attack core given by labels(π1 ). Just like in the previous section, the path
πp π1k πs describes an execution for rejecting the string s0 · sk · s1 in automaton
A. Now, let Tq (k) represent the running time of rejecting the string sk s1 starting
from q, and suppose that it takes 1 unit of time to read string s. We can write
the following recurrence relation for Tq (k):
Tq (k) = (1 + Tq (k − 1)) + (1 + Tq0 (k − 1))
To understand where this recurrence is coming from, observe that there are two
ways to process the first occurence of s:
9

<!-- page 10 -->

q0

q1
q0

c

a

a

b

q

a

b
b

q0

q2

c

q1

q2
a

Fig. 7: A vulnerable NFA (left) and its attack automaton (right).

– Take path π1 and come back to q, consuming 1 unit of time to process string
s. Since we are back at q, we still have Tq (k − 1) units of work to perform.
– Take path π2 and proceed to q 0 , also consuming 1 unit of time to process
string s. Since we are now at q 0 , we have Tq0 (k − 1) units of work to perform.
Now, observe that a lower bound on Tq0 (k) is k since one way to reach qr is
π3k πs , which requires us to read the entire input string. This observation allows
us to obtain the following recurrence relation:
Tq (k) ≥ Tq (k − 1) + k + 1
Thus, the running time of A on the input string s0 · sk · s1 is at least k 2 .
Example 2. The NFA shown in Fig. 7 (left) exhibits super-linear complexity
because we can get from q to q 0 on input string ab, and for both q and q 0 , we
loop back to the same state when reading input string ab. Specifically, we have:
π1 : (q, a, q1 ), (q1 , b, q)

π2 : (q, a, q2 ), (q2 , b, q 0 )

π3 : (q 0 , a, q2 ), (q2 , b, q 0 )

Furthermore, q is reachable from q0 , and there exists a rejecting state, namely
q 0 itself, that is reachable from q 0 . The attack strings are of the form c(ab)k , and
Fig. 7 (right) shows the attack automaton.
5.2

Algorithm for Detecting Vulnerable NFAs

Based on the observations from the previous subsection, we can now formulate
an algorithm that constructs an attack automaton AÈ for a given automaton A.
Just like in Algorithm 1, we construct an attack automaton AÈi for each state in
A by invoking the AttackForPivot procedure. We then take the union of all
such AÈi ’s to obtain an automaton AÈ whose language consists of strings that
cause super-linear running time for A.
Algorithm 2 describes the AttackForPivot procedure for the super-linear
case. Just like in Algorithm 1, we consider all pairs of transitions from q with the
same label (line 11). Furthermore, as in Algorithm 1, we construct an automaton
Ap that recognizes attack prefixes for q (line 13) as well as an automaton A1
that recognizes non-empty strings that start and end at q (line 12).
The key difference of Algorithm 2 is that we also need to consider all states
that could be instantiated as q 0 from Fig. 6 (lines 15–19). For each of these
candidate q 0 ’s, we construct automata A2 , A3 that correspond to paths π2 , π3
10

<!-- page 11 -->

Algorithm 2 Construct super-linear attack automaton AÈ for A and pivot q
1: function AnyLoopBack(A, q 0 )
2:
assume A = (Q, Σ, ∆, q0 , F )
3:
q ? ← NewState(Q); Q0 ← Q ∪ q ? ; ∆0 ← ∆
4:
for (q 0 , l, qi ) ∈ ∆ do
5:
∆0 ← ∆0 ∪ (q ? , l, qi )
0
6:
A ← (Q0 , Σ, ∆0 , q ? , {q 0 })
7:
return A0
8: function AttackForPivot(A, q)
9:
assume A = (Q, Σ, ∆, q0 , F )
10:
AÈ ← A ∅
11:
for (q, l, q1 ) ∈ ∆ ∧ (q, l, q2 ) ∈ ∆ ∧ q1 6= q2 do
12:
A1 ← LoopBack(A, q, l, q1 )
13:
Ap ← (Q, Σ, ∆, q0 , {q})
14:
for q 0 ∈ Q do
15:
qi ← NewState(Q)
16:
A2 ← (Q ∪ {qi }, Σ, ∆ ∪ {(qi , l, q2 )}, qi , {q 0 })
17:
A3 ← AnyLoopBack(A, q 0 )
18:
As ← (Q, Σ, ∆, q 0 , F )
19:
AÈ ← AÈ ∪ (Ap · (A1 ∩ A2 ∩ A3 )+ · As )
20:
return AÈ

from Fig. 6 (lines 16–17). Specifically, we construct A2 by introducing a new
initial state qi with transition (qi , l, q2 ) and making its accepting state q 0 . Hence,
A2 accepts strings that start in q, transition to q2 , and end in q 0 .
The construction of automaton A3 , which should accept all non-empty words
that start and end in q 0 , is described in the AnyLoopBack procedure. First,
since we do not want A3 to accept empty strings, we introduce a new initial
state q ? and add a transition from q ? to all successor states qi of q 0 . Second, the
final state of A0 is q 0 since we want to consider paths that loop back to q 0 .
The final missing piece of the algorithm is the construction of As (line 19),
whose complement accepts all attack suffixes for state q 0 . As expected, As is
the same as the original automaton A, except that its initial state is q 0 . Finally,
similar to Algorithm 1, the attack automaton for states q, q 0 is obtained as Ap ·
(A1 ∩ A2 ∩ A3 )+ · As .
Theorem 4. (Correctness of Algorithm 2) Let NFA A = (Q, Σ, ∆, q0 , F )
and AÈ be the result of calling AttackAutomaton(A). For every s ∈ L(AÈ ),
there exists a rejecting state qr ∈ Q \ F s.t. the number of distinct paths πi from
q0 to qr with labels(πi ) = s is super-linear in the number of repetitions of the
attack core in s.

6

Dynamic Regular Expression Analysis

Algorithms 1 and 2 allow us to determine whether a given NFA is vulnerable.
Even though our static analyses are sound and complete at the NFA level, differ11

<!-- page 12 -->

ent regular expression matching algorithms construct NFAs in different ways and
use different backtracking search algorithms. Furthermore, some matching algorithms may determinize the NFA (either lazily or eagerly) in order to guarantee
linear complexity. Since our analysis does not perform such partial determinization of the NFA for a given regular expression, it can, in practice, generate false
positives. In addition, even if a regular expression is indeed vulnerable, the input
string must still exceed a certain minimum size to cause denial-of-service.
In order to overcome these challenges in practice, we also perform dynamic
analysis to (a) confirm that a regular expression E is indeed vulnerable for Java’s
matching algorithm, and (b) infer a minimum bound on the size of the input
string. Given the original regular expression E, a user-provided time limit t, and
the attack automaton AÈ (computed by static regular expression analysis), our
dynamic analysis produces a refined attack automaton as well as a number b
such that there exists an input string of length greater than b for which Java’s
matching algorithm takes more than t seconds. Note that, as usual, this dynamic
analysis trades soundness for completeness to avoid too many false positives.
In more detail, given an attack automaton AÈ of the form Ap · A+
c · As , the
dynamic analysis finds the smallest k where the shortest string s ∈ L(Ap ·Akc ·As )
exceeds the time limit t. In practice, this process does not require more than a
few iterations because we use the complexity of the NFA to predict the number
of repetitions that should be necessary based on previous runs. The minimum
required input length b is determined based on the length of the found string s.
In addition, the value k is used to refine the attack automaton: in particular,
given the original attack automaton Ap · A+
c · As , the dynamic analysis refines
it to be Ap · Akc · A∗c · As .

7

Static Program Analysis

As explained in Section 2, the presence of a vulnerable regular expression does
not necessarily mean that the program is vulnerable. In particular, there are
three necessary conditions for the program to contain a ReDoS vulnerability:
First, a variable x that stores user input must be matched against a vulnerable
regular expression E. Second, it must be possible for x to store an attack string
that triggers worst-case behavior for E; and, third, the length of the string stored
in x must exceed the minimum threshold determined using dynamic analysis.
To determine if the program actually contains a ReDoS vulnerability, our
approach also performs static analysis of source code. Specifically, our program
analysis employs the Cartesian product [7] of the following abstract domains:
– The taint abstract domain [6,26] tracks taint information for each variable.
In particular, a variable is considered tainted if it may store user input.
– The automaton abstract domain [33,32,12] overapproximates the contents of
string variables using finite automata. In particular, if string s is in the language of automaton A representing x’s contents, then x may store string s.
– The interval domain [13] is used to reason about string lengths. Specifically,
we introduce a ghost variable lx representing the length of string x and use
the interval abstract domain to infer upper and lower bounds for each lx .
12

<!-- page 13 -->

Since these abstract domains are fairly standard, we only explain how to use
this information to detect ReDoS vulnerabilities. Consider a statement match(x, E)
that checks if string variable x matches regular expression E, and suppose that
the attack automaton for E is AÈ . Now, our program analysis considers the
statement match(x, E) to be vulnerable if the following three conditions hold:
1. E is vulnerable and variable x is tainted;
2. The intersection of AÈ and the automaton abstraction of x is non-empty;
3. The upper bound on ghost variable lx representing x’s length exceeds the
minimum bound b computed using dynamic analysis for AÈ and a userprovided time limit t.
Appendix D offers a more rigorous formalization of the analysis.

8

Experimental Evaluation

To assess the usefulness of the techniques presented in this paper, we performed
an evaluation in which our goal is to answer the following questions:
Q1: Do real-world Java web applications use vulnerable regular expressions?
Q2: Can Rexploiter detect ReDoS vulnerabilities in web applications and
how serious are these vulnerabilities?
Results for Q1. In order to assess if real-world Java programs contain vulnerabilities, we scraped the top 150 Java web applications (by number of stars) that
contain at least one regular expression from GitHub repositories (all projects
have between 10 and 2, 000 stars and at least 50 commits) and collected a total
of 2, 864 regular expressions. In this pool of regular expressions, Rexploiter
found 37 that have worst-case exponential complexity and 522 that have superlinear (but not exponential) complexity. Thus, we observe that approximately
20% of the regular expressions in the analyzed programs are vulnerable. We believe this statistic highlights the need for more tools like Rexploiter that can
help programmers reason about the complexity of regular expression matching.
Results for Q2. To evaluate the effectiveness of Rexploiter in finding ReDoS
vulnerabilities, we used Rexploiter to statically analyze all Java applications
that contain at least one vulnerable regular expression. These programs include
both web applications and frameworks, and cover a broad range of application
domains. The average running time of Rexploiter is approximately 14 minutes
per program, including the time to dynamically analyze regular expressions. The
average size of analyzed programs is about 58, 000 lines of code.
Our main result is that Rexploiter found exploitable vulnerabilities in 27
applications (including from popular projects, such as the Google Web Toolkit
and Apache Wicket) and reported a total of 46 warnings. We manually inspected
each warning and confirmed that 41 out of the 46 vulnerabilities are exploitable,
with 5 of the exploitable vulnerabilities involving hyper-vulnerable regular expressions and the rest being super-linear ones. Furthermore, for each of these 41
13

<!-- page 14 -->

Fig. 8: Running times for exponential vulnerabilities (left) and super-linear vulnerabilities (right) for different input sizes.
vulnerabilities (including super-linear ones), we were able to come up with a full,
end-to-end exploit that causes the server to hang for more than 10 minutes.
In Fig. 8, we explore a subset of the vulnerabilities uncovered by Rexploiter
in more detail. Specifically, Fig. 8 (left) plots input size against running time for
the exponential vulnerabilities, and Fig. 8 (right) shows the same information
for a subset of the super-linear vulnerabilities.
Possible fixes. We now briefly discuss some possible ways to fix the vulnerabilities uncovered by Rexploiter. The most direct fix is to rewrite the regular
expression so that it no longer exhibits super-linear complexity. Alternatively,
the problem can also be fixed by ensuring that the user input cannot contain
instances of the attack core. Since our technique provides the full attack automaton, we believe Rexploiter can be helpful for implementing suitable sanitizers.
Another possible fix (which typically only works for super-linear regular expressions) is to bound input size. However, for most vulnerabilities found by Rexploiter, the input string can legitimately be very large (e.g., review). Hence,
there may not be an obvious upper bound, or the bound may still be too large to
prevent a ReDoS attack. For example, Amazon imposes an upper bound of 5000
words (∼25,000 characters) on product reviews, but matching a super-linear
regular expression against a string of that size may still take significant time.

9

Related Work

To the best of our knowledge, we are the first to present an end-to-end solution
for detecting ReDoS vulnerabilities by combining regular expression and program
analysis. However, there is prior work on static analysis of regular expressions
and, separately, on program analysis for finding security vulnerabilities.
Static analysis of regular expressions. Since vulnerable regular expressions
are known to be a significant problem, previous work has studied static analysis techniques for identifying regular expressions with worst-case exponential
complexity [9,18,22,24]. Recent work by Weideman et al. [30] has also proposed
an analysis for identifying super-linear regular expressions. However, no previous
technique can construct attack automata that capture all malicious strings. Since
14

<!-- page 15 -->

attack automata are crucial for reasoning about sanitization, the algorithms we
propose in this paper are necessary for performing sanitization-aware program
analysis. Furthermore, we believe that the attack automata produced by our
tool can help programmers write suitable sanitizers (especially in cases where
the regular expression is difficult to rewrite).
Program analysis for vulnerability detection. There is a large body of
work on statically detecting security vulnerabilities in programs. Many of these
techniques focus on detecting cross-site scripting (XSS) or code injection vulnerabilities [8,11,12,15,17,19,20,23,27,28,29,31,32,33,34]. There has also been recent
work on static detection of specific classes of denial-of-service vulnerabilities.
For instance, Chang et al. [10] and Huang et al. [16] statically detect attackercontrolled loop bounds, and Olivo et al. [21] detect so-called second-order DoS
vulnerabilities, in which the size of a database query result is controlled by the
attacker. However, as far as we know, there is no prior work that uses program
analysis for detecting DoS vulnerabilities due to regular expression matching.
Time-outs to prevent ReDoS. As mentioned earlier, some libraries (e.g.,
the .Net framework) allow developers to set a time-limit for regular expression
matching. While such libraries may help mitigate the problem through a bandaid solution, they do not address the root cause of the problem. For instance,
they neither prevent against stack overflows nor do they prevent DoS attacks in
which the attacker triggers the regular expression matcher many times.

10

Conclusions and Future Work

We have presented an end-to-end solution for statically detecting regular expression denial-of-service vulnerabilities in programs. Our key idea is to combine complexity analysis of regular expressions with safety analysis of programs.
Specifically, our regular expression analysis constructs an attack automaton that
recognizes all strings that trigger worst-case super-linear or exponential behavior.
The program analysis component takes this information as input and performs
a combination of taint and string analysis to determine whether an attack string
could be matched against a vulnerable regular expression.
We have used our tool to analyze thousands of regular expressions in the
wild and we show that 20% of regular expressions in the analyzed programs are
actually vulnerable. We also use Rexploiter to analyze Java web applications
collected from Github repositories and find 41 exploitable security vulnerabilities
in 27 applications. Each of these vulnerabilities can be exploited to make the
web server unresponsive for more than 10 minutes.
There are two main directions that we would like to explore in future work:
First, we are interested in the problem of automatically repairing vulnerable
regular expressions. Since it is often difficult for humans to reason about the
complexity of regular expression matching, we believe there is a real need for
techniques that can automatically synthesize equivalent regular expressions with
linear complexity. Second, we also plan to investigate the problem of automat15

<!-- page 16 -->

ically generating sanitizers from the attack automata produced by our regular
expression analysis.

References
1. CVE-2013-2009. cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2013-2099
2. CVE-2015-2525. cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2015-2526
3. CVE-2015-2525. cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2009-3275
4. CVE-2016-2515. cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-2515
5. CVE-2016-2537. cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2016-2537
6. Arzt, S., Rasthofer, S., Fritz, C., Bodden, E., Bartel, A., Klein, J., Traon, Y.L.,
Octeau, D., McDaniel, P.: Flowdroid: precise context, flow, field, object-sensitive
and lifecycle-aware taint analysis for android apps. In: PLDI. pp. 259–269. ACM
(2014)
7. Ball, T., Podelski, A., Rajamani, S.K.: Boolean and cartesian abstraction for model
checking c programs. In: International Conference on Tools and Algorithms for the
Construction and Analysis of Systems. pp. 268–283. Springer (2001)
8. Bandhakavi, S., Tiku, N., Pittman, W., King, S.T., Madhusudan, P., Winslett, M.:
Vetting browser extensions for security vulnerabilities with VEX. Commun. ACM
54(9), 91–99 (2011)
9. Berglund, M., Drewes, F., van der Merwe, B.: Analyzing catastrophic backtracking
behavior in practical regular expression matching. In: AFL. EPTCS, vol. 151, pp.
109–123 (2014)
10. Chang, R.M., Jiang, G., Ivancic, F., Sankaranarayanan, S., Shmatikov, V.: Inputs
of coma: Static detection of denial-of-service vulnerabilities. In: CSF. pp. 186–199.
IEEE Computer Society (2009)
11. Chaudhuri, A., Foster, J.S.: Symbolic security analysis of ruby-on-rails web applications. In: CCS. pp. 585–594. ACM (2010)
12. Christensen, A.S., Møller, A., Schwartzbach, M.I.: Precise analysis of string expressions. In: SAS. LNCS, vol. 2694, pp. 1–18. Springer (2003)
13. Cousot, P., Cousot, R.: Abstract interpretation: a unified lattice model for static
analysis of programs by construction or approximation of fixpoints. In: POPL. pp.
238–252. ACM (1977)
14. Crosby, S.A., Wallach, D.S.: Denial of service via algorithmic complexity attacks.
In: USENIX Security Symposium. USENIX Association (2003)
15. Dahse, J., Holz, T.: Static detection of second-order vulnerabilities in web applications. In: USENIX Security Symposium. pp. 989–1003. USENIX Association
(2014)
16. Huang, H., Zhu, S., Chen, K., Liu, P.: From system services freezing to system
server shutdown in android: All you need is a loop in an app. In: CCS. pp. 1236–
1247. ACM (2015)
17. Kiezun, A., Guo, P.J., Jayaraman, K., Ernst, M.D.: Automatic creation of SQL
injection and cross-site scripting attacks. In: ICSE. pp. 199–209. IEEE (2009)
18. Kirrage, J., Rathnayake, A., Thielecke, H.: Static analysis for regular expression
denial-of-service attacks. In: NSS. LNCS, vol. 7873, pp. 135–148. Springer (2013)
19. Livshits, V.B., Lam, M.S.: Finding security vulnerabilities in java applications with
static analysis. In: USENIX Security Symposium. USENIX Association (2005)
20. Martin, M.C., Livshits, V.B., Lam, M.S.: Finding application errors and security
flaws using PQL: a program query language. In: OOPSLA. pp. 365–383. ACM
(2005)

16

<!-- page 17 -->

21. Olivo, O., Dillig, I., Lin, C.: Detecting and exploiting second order denial-of-service
vulnerabilities in web applications. In: CCS. pp. 616–628. ACM (2015)
22. Rathnayake, A., Thielecke, H.: Static analysis for regular expression exponential
runtime via substructural logics. CoRR abs/1405.7058 (2014)
23. Su, Z., Wassermann, G.: The essence of command injection attacks in web applications. In: POPL. pp. 372–382. ACM (2006)
24. Sugiyama, S., Minamide, Y.: Checking time linearity of regular expression matching
based on backtracking. IPSJ Online Transactions 7, 82–92 (2014)
25. Thompson, K.: Programming techniques: Regular expression search algorithm.
Communications of the ACM 11(6), 419–422 (1968)
26. Tripp, O., Pistoia, M., Fink, S.J., Sridharan, M., Weisman, O.: TAJ: effective taint
analysis of web applications. In: PLDI. pp. 87–97. ACM (2009)
27. Wassermann, G., Su, Z.: Sound and precise analysis of web applications for injection vulnerabilities. In: PLDI. pp. 32–41. ACM (2007)
28. Wassermann, G., Su, Z.: Static detection of cross-site scripting vulnerabilities. In:
ICSE. pp. 171–180. ACM (2008)
29. Wassermann, G., Yu, D., Chander, A., Dhurjati, D., Inamura, H., Su, Z.: Dynamic
test input generation for web applications. In: ISSTA. pp. 249–260. ACM (2008)
30. Weideman, N., van Der Merwe, B., Berglund, M., Watson, B.: Analyzing matching
time behavior of backtracking regular expression matchers by using ambiguity of
NFA. In: CIAA (2016), to appear
31. Xie, Y., Aiken, A.: Static detection of security vulnerabilities in scripting languages.
In: USENIX Security Symposium. USENIX Association (2006)
32. Yu, F., Alkhalaf, M., Bultan, T.: Stranger: An automata-based string analysis tool
for PHP. In: TACAS. LNCS, vol. 6015, pp. 154–157. Springer (2010)
33. Yu, F., Alkhalaf, M., Bultan, T., Ibarra, O.H.: Automata-based symbolic string
analysis for vulnerability detection. FMSD 44(1), 44–70 (2014)
34. Yu, F., Bultan, T., Hardekopf, B.: String abstractions for string verification. In:
SPIN. LNCS, vol. 6823, pp. 20–37. Springer (2011)

Appendix A: Proof about Algorithm 1
Proof. (sketch) We first show that any attack string s is accepted by AÈ . Based
on Section 4, we know that attack strings that cause exponential behavior are
of the form s0 · skc · s1 where s0 = labels(πp ), sc = labels(π1 ) = labels(π2 ),
s1 = labels(πs ) for some pivot state q. Now, we argue that s will be accepted by
the attack automaton AÈq for q, which implies that s is also accepted by AÈ since
AÈq ⊆ AÈ . Since AttackForPivot is invoked for each state q, we will consider
the two distinct transitions (q, l, q1 ) and (q, l, q2 ) that start paths π1 and π2 .
Furthermore, by the construction in the LoopBack procedure, labels(π1 ) and
labels(π2 ) will be accepted by A1 and A2 . Thus, string sc will be accepted by
(A1 ∩ A2 ). Similarly, by the construction at lines 14–15, Ap and As will accept
s0 and s1 respectively. Hence, the attack string s = s0 skc s1 will be recognized by
Ap · (A1 ∩ A2 )+ · As .
For the other direction, we show that if a string s is accepted by AÈ , then it is
indeed an attack string. The attack automaton constructed by the algorithm is a
union of automata AÈq of the form Ap · (A1 ∩ A2 )+ · As , one of which must accept
17

<!-- page 18 -->

the string s. As a consequence, there must exist strings s0 ∈ Ap , skc ∈ (A1 ∩A2 )+ ,
and s1 ∈ As such that s = s0 · skc · s1 . Due to the construction of AÈq in function
AttackForPivot, there must exist corresponding paths π1 , π2 (distinct from
π1 ), πp , and πs such that s0 = labels(πp ), sc = labels(π1 ) = labels(π2 ), and
s1 = labels(πs ). Based on Section 4, any such string s constitutes an attack
string.

Appendix B: Proof about Algorithm 2
Proof. (sketch) We first show that any attack string s is accepted by AÈ . Based
on Section 5.1, we know that attack strings that cause super-linear behavior are
of the form s0 · skc · s1 where s0 = labels(πp ), sc = labels(π1 ) = labels(π2 ) =
labels(π3 ), s1 = labels(πs ) for some pivot state q and a state q 0 . Now, we argue
that s will be accepted by the attack automaton AÈq for q, which implies that s is
also accepted by AÈ since AÈq ⊆ AÈ . Since AttackForPivot is invoked for each
state q, we will consider the two distinct transitions (q, l, q1 ) and (q, l, q2 ) that
start paths π1 and π2 and any state q 0 . Furthermore, by the construction in the
LoopBack procedure, labels(π1 ) will be accepted by A1 . By the construction
on lines 15–16, labels(π2 ) will be accepted by A2 . By the construction in the
AnyLoopBack procedure, labels(π3 ) will be accepted by A3 . Thus, string sc
will be accepted by (A1 ∩ A2 ∩ A3 ). Similarly, by the construction at lines 13
and 18, Ap and As will accept s0 and s1 respectively. Hence, the attack string
s = s0 skc s1 will be recognized by Ap · (A1 ∩ A2 ∩ A3 )+ · As .
For the other direction, we show that if a string s is accepted by AÈ , then it
is indeed an attack string. The attack automaton constructed by the algorithm
is a union of automata AÈq of the form Ap · (A1 ∩ A2 ∩ A3 )+ · As , one of which
must accept the string s. As a consequence, there must exist strings s0 ∈ Ap ,
skc ∈ (A1 ∩A2 ∩A3 )+ , and s1 ∈ As such that s = s0 ·skc ·s1 . Due to the construction
of AÈq in function AttackForPivot, there must exist corresponding paths π1 ,
π2 (distinct from π1 ), π3 , πp , and πs such that s0 = labels(πp ), sc = labels(π1 ) =
labels(π2 ) = labels(π3 ), and s1 = labels(πs ). Based on Section 5.1 any such string
s constitutes an attack string.

Appendix C: Necessity Proofs (Theorems 1 and 3)
This section shows that the conditions in Theorems 1 and 3 are not only sufficient, but also necessary for the NFA to exhibit exponential and super-linear
complexity respectively. In the rest of this section, we use the term vulnerable
NFA to mean an NFA that satisfies the conditions of Theorem 1 and hypervulnerable NFA to mean an NFA satisfying conditions of Theorem 3.
Our proof uses the concept of strongly-connected component pair path (SPP).
Given an NFA with |Q| states, there are at most O(|Q||Q| ) such SPPs. Given
a string s, if the NFA is not hyper-vulnerable, then there are at most |s||Q|
possible matchings per SPP. The complexity of a worst-case backtracking search
algorithm is thus polynomial in length of s: O(|Q||Q| |s||Q| ).
18

<!-- page 19 -->

If the NFA is not vulnerable, then there are at most |Q||Q| possible matchings
per SPP. A backtracking search algorithm could match all possible substrings
of s adding a of factor |s| and a factor of |Σ||Q| , resulting in the complexity:
O(|Q||Q| |Σ||Q| |s|).
Definition 5. (Strongly-Connected Component) Given an NFA
A = (Q, Σ, ∆, q0 , F ). Two states q, q 0 ∈ Q with q 6= q 0 are in the same stronglyconnected component if and only if there exist a path from q to q 0 and from q 0 to
q. A partition of Q in strongly connected components is unique.
Lemma 1. Given a non-hyper-vulnerable NFA A = (Q, Σ, ∆, q0 , F ), a string s,
and two states q, q 0 ∈ Q occurring in the same strongly-connected component.
The path π from q to q 0 such that labels(π) = s is unique.
Proof. Assume that there are two paths π1 and π2 from q to q 0 such that π1 6= π2
and labels(π1 ) = labels(π2 ) = s. Since q and q 0 occur in the same stronglyconnected component, there must be a path π3 from q 0 to q, because in a stronglyconnected component there exists a path from every state to every state. Now we
have two cycles from q to q, π1 π3 and π2 π3 such that labels(π1 π3 ) = labels(π2 π3 ).
This violates the assumption that A is not hyper-vulnerable.
Notice that Lemma 1 also holds for non-vulnerable NFAs as each non-vulnerable
NFA is also non-hyper-vulnerable.
Definition 6. (Strongly-Connected Component Pair Path) Given an NFA
A = (Q, Σ, ∆, q0 , F ). A strongly-connected component pair path (SPP) of A
is a sequence of state pairs (qin , qout ), with qin and qout occurring in the same
strongly-connected component. Moreover, for any two consecutive (i.e., occurring
in different, but connected strongly-connected components) state pairs (qin , qout )
0
0
0
) there must exist a transition (qout , l, qin
) ∈ ∆ for some label l.
and (qin
, qout
Lemma 2. Given an NFA A = (Q, Σ, ∆, q0 , F ). There are at most O(|Q||Q| )
different strongly-connected component pair paths.
Proof. A has at most |Q| strongly-connected components. Consequently, a SPP
consists of at most |Q| pairs. Each strongly-connected component consists of at
most |Q| states. Hence there are at most |Q|2 different state pairs per stronglyconnected components. The number of SPPs for A is thus at most |Q|2|Q| or
O(|Q||Q| ).
Definition 7. (Path Partition) Given an NFA A = (Q, Σ, ∆, q0 , F ), a string
s and a strongly-connected component pair path Π of A. A path partition of s
and Π is a partition of s into 2|Π| − 1 substrings si with i ∈ {0, . . . , 2|Π| − 2} in
such a way that each si with odd i consists of exactly one symbol. The substrings
si with i even can be arbitrarily long (or short, even empty). Notice that the
choice of the si with odd i define the si with the even i: Let i be odd, si+1 are
all the symbols in s that occur between si and si+2 .
19

<!-- page 20 -->

Lemma 3. Given an NFA A = (Q, Σ, ∆, q0 , F ), a string s and a stronglyconnected component pair path Π of A. There exist at most |s||Q| different path
partitions of s and Π.

|s|
Proof. There exists at most |Π|−1
different path partitions of s and Π, i.e., all
possible |Π| − 1 choices of si ’s with odd i. Notice that |Π| − 1 < |Q|, because a
SPP has at most length |Q|. Therefore the number of different path partitions
is less than |s||Q| .
Theorem 5. Let A be the NFA (Q, Σ, ∆, q0 , F ), which is not hyper-vulnerable.
The runtime to determine if a string s is accepted by A is at most O(|Q||Q| |s||Q| ).
Proof. Below we assume that s consists of at least |Q| symbols. In case |s| < |Q|,
then the number of steps is limited by |Q||s| and thus |Q||Q| even for hypervulnerable NFAs: From each state we could potentially go to each other state
and repeat that |s| times.
From Lemma 1 we know that there is a path from state q to q 0 with q and
0
q occurring in the same strongly connected component is unique. Given path
partition P of s and Π, we can now deduce that the path of s from q0 to the
last qout in Π is unique: each si with even i uniquely force the path within a
strongly-connected component, while each si with odd i uniquely define the path
in between strongly-connected components.
Furthermore, from Lemma 3, we know that the number of part partitions is
at most |s||Q| .
Consequently, a backtracking search algorithm for s will require at most
O(|Q||Q| |s||Q| ) steps. Notice that we did not discuss that a backtrack search
algorithm also matches strings that are the first n symbols of s. This adds another
factor of |s|, which can be ignored given the above complexity result.
Definition 8. (Labelled Strongly-Connected Component Pair Path)
Given an NFA A = (Q, Σ, ∆, q0 , F ). A labelled strongly-connected component
pair path (LSPP) of A is a strongly-connected component pair path with has a
0
0
)
specific label l in between two consecutive state pairs (qin , qout ) and (qin
, qout
0
that describes the transition from qout to qin .
Lemma 4. Given an NFA A = (Q, Σ, ∆, q0 , F ). There are at most O(|Q||Q| |Σ||Q| )
different labelled strongly-connected component pair paths.
Proof. Let Π be a strongly-connected component pair path. There are at most
O(|Q||Q| ) strongly-connected component pair paths (Lemma 2). There are |Π|−1
consecutive state pairs in Π. For each of them a label l ∈ Σ can be selected.
Hence there are |Σ||Π|−1 different labelled strongly-connected component pair
path that have the same state pairs as Π. This results in O(|Q||Q| |Σ||Q| ) different
LSSPs.
A path partition P of string s and strongly-connected component pair path
Π is called valid, if there exists a path π that follows the states described in Π
such that labels(π) = s. Lemma 3 states that for a non-hyper-vulnerable NFA
20

<!-- page 21 -->

there are at most |s||Q| path partitions. All of them could be valid. However,
below we will show that for a non-vulnerable NFA that the number of valid
path partitions is at most |Q||Q| , thereby removing the factor |s||Q| from the
complexity.
Lemma 5. Given a non-vulnerable NFA A = (Q, Σ, ∆, q0 , F ), a string s, and a
0
0
labelled strongly-connected component pair path Π. Let (qin , qout ) and (qin
, qout
)
be two consecutive state pairs of Π and let l be the label in between the state
0
pairs: (qout , l, qin
) ∈ ∆. There are at most |Q| possible choices of l in s, such that
there exists a valid path partition P of s and Π.
Proof. (sketch) Let qend be the last state in Π, i.e., the second state in the last
state pair. Let Q0 be all the states for which a path exists to qout including qout
0
0
and let Q1 be all states for which a path exists starting from qin
including qin
.
Notice that the intersection of Q0 and Q1 is empty.
Consider |Q0 | different choices of l in s such that exists a path from q0 to
0
qout following the states described in Π and a path from qin
to qend following the
states described in Π. On any such path that can be at most |Q0 | − 1 different
l transitions. Hence there must exists a chosen transition (q, l, q 0 ) such that the
path from q0 to qout uses that transition at least twice. For this q, there exists
a cycle π1 from q to q and a path π2 starting at q that includes the transition
0
(qout , l, qin
) such that labels(π1 ) = labels(π2 ).
We can apply the same reason for Q1 : Consider |Q1 | different picks of l in s
such that exists a path from q0 to qout following the states described in Π and
0
a path from qin
to qend following the states described in Π. On any such path
that can be at most |Q1 | − 1 different l transitions. Hence there must exists a
0
transition (q 00 , l, q 000 ) such that the path from qin
to qend uses that transition at
00
least twice. For this q , there exists a cycle π3 from q 00 to q 00 and a path π4 ending
0
at q 00 that includes the transition (qout , l, qin
) such that labels(π3 ) = labels(π4 ).
00
Let π5 be a path from q to q . Now we can change the cycles π1 and π3
to π10 and π30 by extending them with loops such that labels(π10 ) = labels(π30 ) =
labels(π5 ). The existence of such paths is in conflict with the assumption that A
is not vulnerable. Consequently, there must be less than |Q0 | + |Q1 | choices for
l. Since the intersection of Q0 and Q1 is empty, |Q0 | + |Q1 | ≤ |Q|.
Theorem 6. Let A be the NFA (Q, Σ, ∆, q0 , F ), which is not vulnerable. The
runtime to determine if a string s is accepted by A is at most O(|Q||Q| |Σ||Q| |s|).
Proof. From Lemma 4 we know that there are at most O(|Q||Q| |Σ||Q| ) labelled
strongly-connected component pair path of s. Let Π be one of these LSPPs.
There are |Π| − 1 labels between consecutive state pairs. For each of them there
are at lost |Q| choices from s, such that the path partition is valid (Lemma 5).
Consequently, there are at most |Q||Π|−1 valid path partitions for Π. Since |Π|−
1 < |Q|, the number of valid path partitions is less than |Q||Q| . Hence we can
ignore the number of valid path partitions in the complexity, because it does not
alter O(|Q||Q| |Σ||Q| ).
21

<!-- page 22 -->

Given a valid path partition, a backtrack search algorithm may take |s| steps
from q0 to the last state. This adds a factor of |s| to the complexity resulting in
O(|Q||Q| |Σ||Q| |s|).

Appendix D: Formal Analysis

Statement S

:= x := e | getInput(x)
| match(x, E) | S1 ; S2
| assume(x ∈ R)
| assume(len(x) ≤ ν)
| if(?) then S1 else S2
| while(?) do S

String exp e
:= x | ?
Int exp ν
:= int | len(x) | ν1 + ν2 | ν1 − ν2
Pure regex E
:= a ∈ Σ | E ∗ | E1 + E2 | E1 · E2
Impure regex R := E | x | R∗ | R1 + R2 | R1 · R2

Fig. 9: The Strimp intermediate language
Intermediate language. We formalize our program analysis using the intermediate language shown in Fig. 9. This language, which we refer to as Strimp,
is suitable for describing our analysis because it models the effects of different
string manipulation functions in a uniform way using assume statements.
In Strimp, all variables have type string. In addition to the standard assignment, sequence, conditional, and loop constructs, Strimp contains a function
getInput(x), which binds variable x to a string supplied by the user. Another
function, match(x, E), models matching string x against regular expression E.
The Strimp language contains two kinds of assume statements that allow us to model the effect of string manipulation procedures (e.g., provided
by java.lang.String). First, the statement assume(x ∈ R) states that x belongs to the language given by impure regular expression R. Here, we refer to
R as impure because the regular expression can refer to program variables. For
example, the statement assume(x ∈ y · a) models that the value stored in x is
the value stored in y concatanated with the character a. Thus, if y can be any
string, then this annotation expresses that x is a string ending in a. The use of
such impure regular expressions in Strimp allows us to model string operations
in a uniform way.
The second form of annotation in Strimp is of the form assume(len(x) ≤ ν)
and allows us to express constraints on the size of strings. Here, the integer
expression ν can refer to the length of other strings and can contain arithmetic
operators (+, −).
One final point worth noting is that string expressions include a special symbol ?, which represents an unknown string constant. Hence, an assignment of
22

<!-- page 23 -->

JEKΛ = A(E)
JxKΛ = snd(Λ(x))
JR∗ KΛ = (JRKΛ )∗
JR1 R2 KΛ = JR1 KΛ · JR2 KΛ
JR1 + R2 KΛ = JR1 KΛ + JR2 KΛ

Fig. 10: Helper rules for evaluating impure regular expressions. We use A(E) to
denote an NFA that accepts the same language as regular expression E.
JintKΛ = hint, inti
Jlen(x)KΛ = fst(Λ(x))
Jν1 + ν2 KΛ = Jν1 KΛ ⊕ Jν2 KΛ
Jν1 − ν2 KΛ = Jν1 KΛ Jν2 KΛ

Fig. 11: Helper rules for evaluating arithmetic expressions
the form x = "abc" is easily expressible in our language using the code snippet:
x :=?; assume(x ∈ abc)

Program abstraction. As mentioned earlier, our program analysis needs to track
taint information as well as information about string lengths and contents. Towards this goal, our analysis employs three kinds of program abstractions:
– The taint abstraction Φ is a set of variables such that x ∈ Φ indicates that
x may be tainted.
– The string abstraction Λ is a mapping from each program variable x to a
pair (I, A), where I is an interval hl, ui such that l ≤ len(s) ≤ u and A is an
NFA representing x’s contents. In particular, if a string s is in the language
of A, this indicates that x can store string s.
– The attack abstraction Ψ maps each regular expression E in the program
to a pair (b, AÈ ). Here, b is a minimum bound on the length of the input
string s such that, if len(s) < b, matching s against E takes negligible time 3 .
The NFA AÈ is the attack automaton for regular expression E and is precomputed using the analyses from Sections 4 and 5.
Since our program abstractions involve pairs (e.g., (I, A)) , we use the notation fst(p) and snd(p) to retrieve the first and second components of pair p
respectively.
Analysis rules. We describe our static analysis using judgments of the form
Ψ, Φ, Λ ` S : Φ0 , Λ0 which state that, if we execute S in a state that satisfies
program abstractions Ψ, Φ, Λ, we obtain a new taint abstraction Φ0 and new
string abstraction Λ0 . The inference rules describing our analysis are shown in
Fig. 13.
3

Here, what constitutes negligible time is an input parameter of our analysis and can
be customized by the user.

23

<!-- page 24 -->

hl1 , u1 i ⊕ hl2 , u2 i = hl1 + l2 , u1 + u2 i
hl1 , u1 i hl2 , u2 i = hl1 − l2 , u1 − u2 i
hl1 , u1 i t hl2 , u2 i = hmin(l1 , l2 ), max(u1 , u2 )i
(Λ1 t Λ2 )(x) = (fst(Λ1 (x)) t fst(Λ2 (x)),
snd(Λ1 (x)) ∪ snd(Λ2 (x)))

Fig. 12: Operations on abstract domains
In this figure, rule (1) describes the analysis of sources (i.e., getInput(x)).
Since variable x is now tainted, we add it to our taint abstraction Φ. Furthermore,
since the user is free to supply any string, Λ0 abstracts the length of x using the
interval h0, ∞i and its contents using the automaton A∗ , which accepts any
string.
Rule (2) for processing assignments x1 := x2 is straightforward: In particular,
x1 becomes tainted iff x2 is tainted, and the string abstraction of x1 is the same
as variable x2 . For assignments of the form x :=? (rule 3), we untaint variable
x by removing it from Φ since ? denotes string constants in Strimp. However,
since ? represents unknown strings, Λ0 maps x to (h0, ∞i, A∗ ).
Rule (4) describes the analysis of assumptions of the form assume(x ∈ R).
Because R can refer to program variables, we must first figure out the regular
expressions that are represented by R. For this purpose, Fig. 10 describes the
evaluation of impure regular expression R under string abstraction Λ, denoted
as JRKΛ . Since the assumption states that the value stored in x must be in
the language JRKΛ , the new string abstraction Λ0 maps x to the automaton
snd(Λ(x)) ∩ JRKΛ . 4
Rule (5), which is quite similar to rule (4), allows us to handle assumptions
of the form assume(len(x) ≤ ν). Since integer expression ν can refer to terms
of the form len(y), we must evaluate ν under string abstraction Λ. For this
purpose, Fig. 11 shows the evaluation of ν under Λ, denoted as JνKΛ . Now, going
back to rule (5) of Fig. 13, suppose that JνKΛ yields the interval hl2 , u2 i, and
suppose that Λ maps x to the length abstraction hl1 , u1 i. Clearly, the assumption
assume(len(x) ≤ ν) does not change the lower bound on len(x); hence the lower
bound remains l1 . However, if u2 is less than the previous upper bound u1 , we
now have a more precise upper-bound u1 . Hence, the new string abstraction Λ0
maps the length component of x to the interval hl1 , min(u1 , u2 )i.
Rule (6) for match statements allows us to detect if the program contains a
vulnerability. In particular, the premise of this rule states that either (1) x is not
tainted (x 6∈ Φ ) or (2) the automaton representing x’s contents does not contain
any string in the attack automaton for E (i.e., A = A∅ ), or (3) the length of x
cannot exceed the minimum bound given by Ψ (i.e., fst(Ψ (E)) 6∈ fst(Λ(x))). If
4

Observe that Rule (4) does not modify the length abstraction component of Λ. This
is clearly sound, but potentially imprecise. However, since we model Java string
operations by adding a pair of assumptions, one concerning length and the other
concerning content, our analysis does not lead to a loss of precision because of the
way assumptions are introduced.

24

<!-- page 25 -->

Φ0 = Φ ∪ {x}
Λ = Λ[x 7→ (h0, ∞i, A∗ )]
Ψ, Φ, Λ ` getInput(x) : Φ0 , Λ0
0

(1)

(2)

(3)

(4)

(5)

Λ0= Λ[x1 7→ Λ(x2 )]
Φ ∪ {x1 } if x2 ∈ Φ
Φ0 =
Φ
if x2 6∈ Φ
Ψ, Φ, Λ ` x1 := x2 : Φ0 , Λ0
Λ0 = Λ[x 7→ (h0, ∞i, A∗ )]
Ψ, Φ, Λ ` x := ? : Φ \ {x}, Λ0
A = snd(Λ(x)) ∩ JRKΛ
Λ0 = Λ[x 7→ (fst(Λ(x)), A)]
Ψ, Φ, Λ ` assume(x ∈ R) : Φ, Λ0
hl1 , u1 i = fst(Λ(x))
hl2 , u2 i = JνKΛ
I = hl1 , min(u1 , u2 )i
Λ0 = Λ[x 7→ (I, snd(Λ(x)))]
Ψ, Φ, Λ ` assume(len(x) ≤ ν) : Φ, Λ0

A = snd(Ψ (E)) ∩ snd(Λ(x))
x 6∈ Φ ∨ A = A∅ ∨ fst(Ψ (E)) 6∈ fst(Λ(x))
(6)
Ψ, Φ, Λ ` match(x, E) : Φ, Λ

(7)

Ψ, Φ, Λ ` S1 : Φ1 , Λ1
Ψ, Φ1 , Λ1 ` S2 : Φ2 , Λ2
Ψ, Φ, Λ ` S1 ; S2 : Φ2 , Λ2

Ψ, Φ, Λ ` S1 : Φ1 , Λ1
Ψ, Φ, Λ ` S2 : Φ2 , Λ2
Φ0 = Φ1 ∪ Φ2 , Λ0 = Λ1 t Λ2
(8)
Ψ, Φ, Λ ` if(?) then S1 else S2 : Φ0 , Λ0

(9)

Φ∗ ⊇ Φ, Λ∗ w Λ
Ψ, Φ∗ , Λ∗ ` S : Φ∗ , Λ∗
Ψ, Φ, Λ ` while(?) do S : Φ∗ , Λ∗

Fig. 13: Inference rules describing static analysis

25

<!-- page 26 -->

Java statement or predicate
Strimp translation
x.contains(s)
assume(x ∈ (Σ ∗ · s · Σ ∗ )); assume(len(s) ≤ len(x))
y = x.replaceAll(a, b)
(y := x ⊗ y :=?); assume(y ∈ (!a)∗ ); assume(len(y) ≤ len(x))
y = x.substring(c1, c2)
(y := x ⊗ y :=?); assume(len(y) ≤ c2 − c1 )
x.length() <= c
assume(len(x) ≤ c)
x.split(a).length() == c
assume(x ∈ ((!a)∗ · a · (!a)∗ )c )
x.indexOf(s) != -1
assume(x ∈ (Σ ∗ · s · Σ ∗ )); assume(len(s) ≤ len(x))
x.endsWith(y)
assume(x ∈ (Σ ∗ · y)); assume(len(y) ≤ len(x))
x.equals(y)
assume(x ∈ y); assume(len(x) ≤ len(y)); assume(len(y) ≤ len(x))
x.matches(E)
assume(x ∈ E);
x.startsWith(y)
assume(x ∈ (y · Σ ∗ )); assume(len(y) ≤ len(x))

Table 1: Examples illustrating translation from Java string operations to Strimp
constructs. Here x, y denote variables, s denotes string constants, a, b represent distinct characters, and c, c1 , c2 represent integer constants. The notation !a means any character other than a, and S1 ⊗ S2 is syntactic sugar for
if(?) then S1 else S2 . Observe that the statement y := x ⊗ y :=? has the effect
of tainting y if x is tainted but does not introduce any assumptions about the
content or size of string y.

these conditions in the premise of the match rule are not met, then the program
may contain a vulnerability.
The next rules for sequencing (7) and conditionals (8) are fairly standard.
Since we take the union of the taint abstractions in rule (7), a variable x becomes
tainted if it was tainted in either branch of the conditional. Also, note that the
join operator on string abstractions is defined in Fig. 12. Finally, the last rule
describes the analysis of loops. In particular, rule (9) states that the abstractions
Φ∗ and Λ∗ overapproximate the behavior of the loop because (a) they subsume
the initial abstractions Φ, Λ (first premise), and (b) they are inductive (second
premise). While this rule does not describe how to compute Φ∗ and Λ∗ in an
algorithmic way, our implementation performs standard fixed point computation
(using widening) to find these loop invariants.

26
