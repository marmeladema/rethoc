---
file: ../pdfs/2024-soda-bille-sparse-regex-matching.pdf
citation: "Sparse Regular Expression Matching; Bille, Gørtz; SODA 2024"
doi: 
source: "https://arxiv.org/pdf/1907.04752"
topics: [semantics_theory_access]
pages: 29
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

Sparse Regular Expression Matching
Philip Bille
phbi@dtu.dk

Inge Li Gørtz
inge@dtu.dk

arXiv:1907.04752v7 [cs.DS] 6 Nov 2023

Abstract
A regular expression specifies a set of strings formed by single characters combined with
concatenation, union, and Kleene star operators. Given a regular expression R and a string
Q, the regular expression matching problem is to decide if Q matches any of the strings
specified by R. Regular expressions are a fundamental concept in formal languages and regular
expression matching is a basic primitive for searching and processing data. A standard textbook
solution [Thompson, CACM 1968] constructs and simulates a nondeterministic finite automaton,
leading to an O(nm) time algorithm, where n is the length of Q and m is the length of R.
Despite considerable research efforts only polylogarithmic improvements of this bound are known.
Recently, conditional lower bounds provided evidence for this lack of progress when Backurs
and Indyk [FOCS 2016] proved that, assuming the strong exponential time hypothesis (SETH),
regular expression matching cannot be solved in O((nm)1−ϵ ), for any constant ϵ > 0. Hence, the
complexity of regular expression matching is essentially settled in terms of n and m.
In this paper, we take a new approach and introduce a density parameter, ∆, that captures
the amount of nondeterminism in the NFA simulation on Q. The density is at most nm + 1 but
can be significantly smaller. Our main result is a new algorithm that solves regular expression
matching in


nm
O ∆ log log
+n+m
∆
time.
This essentially replaces nm with ∆ in the complexity of regular expression matching. We
complement our upper bound by a matching conditional lower bound that proves that we cannot
solve regular expression matching in time O(∆1−ϵ ) for any constant ϵ > 0 assuming SETH.
The key technical contribution in the result is a new linear space representation of the classic
position automaton that supports fast state-set transition computation in near-linear time in
the size of the input and output state sets. To achieve this, we develop several new insights
and techniques of independent interest, including new structural properties of the parse trees
of regular expressions, a decomposition of state-set transitions based on parse trees, and a fast
batched predecessor data structure.

<!-- page 2 -->

1

Introduction

A regular expression R specifies a set of strings formed by characters from an alphabet Σ combined
with concatenation (⊙), union (|), and Kleene star (∗ ) operators. For instance, (a|(b ⊙ a))∗ describes
the set of strings of as and bs such that every b is followed by an a. Given a regular expression R and
string Q, the regular expression matching is to decide if Q matches any of the strings specified by R.
Regular expressions are a fundamental concept in formal language theory introduced by Kleene in
the 1950’ties [48] and regular expression matching is a basic tool in computer science for searching
and processing text. Standard tools such as grep and sed provide direct support for regular
expression matching in files, and the scripting language perl [72] is a full programming language
designed to support regular expression matching easily. Regular expression matching appears in
many large-scale data processing applications such as internet traffic analysis [44, 49, 78], data
mining [31], data bases [52, 60], computational biology [63], and human-computer interaction [47].
A classic textbook algorithm for regular expression matching, due to Thompson [70] from 1968,
constructs and simulates a nondeterministic finite automaton (NFA) A in O(nm) time, where n is
the length of Q and m is the number of character symbols in R. The simulation processes Q from
left to right and computes a sequence of sets of states S0 , . . . , Sn such that Si is the set of states in A
to which there is a path from the initial state that matches Q[1..i]. In 1985 Galil [29] asked if a faster
algorithm could be obtained. A sequence of results [10, 11, 12, 62] improved the O(nm) bound using
log n
tabulation or word-level parallelism leading to solutions using either O(nm log
+ n + m) [12] or
log1.5 n
O(nm logww + n + m log m) time [10] time, where w is the word length. Finally, Bille and Thorup [13]
gave an algorithm using O(nk logww + n + m log k) time, where k ≤ m is the number of strings
appearing in the regular expression (see also [8, 26]).
The above solutions are based on the classic NFA simulation algorithm from Thompson’s original
algorithm [70] and thus achieve the same O(nm) time with some polylogarithmic factors shaved.
Recently, conditional lower bounds have provided evidence for the lack of more significant progress.
First, Backurs and Indyk [7] showed in 2016 that we cannot solve regular expression matching in
time O((nm)1−ϵ ), for ϵ > 0, assuming the strong exponential time hypothesis (SETH) [43]. Then, in
2018 Abboud and Bringmann [1] showed that we cannot solve the problem in time O(nm/ log7+ϵ n),
for ϵ > 0, assuming the Formula SAT hypothesis [1]. These results, together with Bringmann,
Larsen, and Grønlund [15] and Schepper [67], also studied subclasses of regular expression matching
depending on the structure of the operators in the expression, leading to a classification of the
complexity of each such subclass. In summary, the complexity of regular expression matching is
essentially settled in terms of n and m.
In this paper, we take a new approach and introduce a density parameter, ∆, that captures the
amount of nondeterminism in the NFA simulation on Q. The density is at most nm + 1 but can be
significantly smaller. Our main result is a new algorithm that solves regular expression matching in


nm
O ∆ log log
+n+m
∆
time. This essentially replaces nm with ∆ in the complexity of regular expression matching. We
complement our upper bound by a conditional lower bound that proves that we cannot solve regular
expression matching in time O(∆1−ϵ ) for any constant ϵ > 0 assuming SETH.

1.1

Sparse Regular Expression Matching

Recall that the NFA simulation algorithm constructs a sequence S0 , . . . , Sn of state sets such that
Si is the set of states in the automaton to which there is a path from the initial state that matches
1

<!-- page 3 -->

a
<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

c
b
<latexit sha1_base64="reJF7fIRK1L1tE7SseTH9oIFa+s=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipyQblilt1FyDrxMtJBXI0BuWv/jBmaYTSMEG17nluYvyMKsOZwFmpn2pMKJvQEfYslTRC7WeLQ2fkwipDEsbKljRkof6eyGik9TQKbGdEzVivenPxP6+XmvDWz7hMUoOSLReFqSAmJvOvyZArZEZMLaFMcXsrYWOqKDM2m5INwVt9eZ20a1Xvquo2ryv1Wh5HEc7gHC7Bgxuowz00oAUMEJ7hFd6cR+fFeXc+lq0FJ585hT9wPn8Awj+M2Q==</latexit>

a
a

a
a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a
<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="+j8R4BI5yr9BO5sktBhQivJBXR4=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipGQzKFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16reVdVtXlfqtTyOIpzBOVyCBzdQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AwLuM2A==</latexit>

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a

a

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

b

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="+j8R4BI5yr9BO5sktBhQivJBXR4=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipGQzKFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16reVdVtXlfqtTyOIpzBOVyCBzdQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AwLuM2A==</latexit>

a

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

a
<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

b
<latexit sha1_base64="+j8R4BI5yr9BO5sktBhQivJBXR4=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipGQzKFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16reVdVtXlfqtTyOIpzBOVyCBzdQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AwLuM2A==</latexit>

c

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="reJF7fIRK1L1tE7SseTH9oIFa+s=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipyQblilt1FyDrxMtJBXI0BuWv/jBmaYTSMEG17nluYvyMKsOZwFmpn2pMKJvQEfYslTRC7WeLQ2fkwipDEsbKljRkof6eyGik9TQKbGdEzVivenPxP6+XmvDWz7hMUoOSLReFqSAmJvOvyZArZEZMLaFMcXsrYWOqKDM2m5INwVt9eZ20a1Xvquo2ryv1Wh5HEc7gHC7Bgxuowz00oAUMEJ7hFd6cR+fFeXc+lq0FJ585hT9wPn8Awj+M2Q==</latexit>

a

b

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="+j8R4BI5yr9BO5sktBhQivJBXR4=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipGQzKFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16reVdVtXlfqtTyOIpzBOVyCBzdQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AwLuM2A==</latexit>

(a)

a

(b)

<latexit sha1_base64="xCYow5BVFq3CiJo01XnPJUWogoA=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FSSItRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0KlVPcutm0qjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w++j4zV</latexit>

<latexit sha1_base64="ViTER4vnQkS23GscKnbkckEBaz0=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBahXkpSBT0WvHisaGuhDWWznbRLN5uwuxFK6E/w4kERr/4ib/4bt20O2vpg4PHeDDPzgkRwbVz32ymsrW9sbhW3Szu7e/sH5cOjto5TxbDFYhGrTkA1Ci6xZbgR2EkU0igQ+BiMb2b+4xMqzWP5YCYJ+hEdSh5yRo2V7qv0vF+uuDV3DrJKvJxUIEezX/7qDWKWRigNE1Trrucmxs+oMpwJnJZ6qcaEsjEdYtdSSSPUfjY/dUrOrDIgYaxsSUPm6u+JjEZaT6LAdkbUjPSyNxP/87qpCa/9jMskNSjZYlGYCmJiMvubDLhCZsTEEsoUt7cSNqKKMmPTKdkQvOWXV0m7XvMuau5dvdK4zOMowgmcQhU8uIIG3EITWsBgCM/wCm+OcF6cd+dj0Vpw8plj+APn8weEf408</latexit>

<latexit sha1_base64="/GfBiAAtkl9YEn59iIdV/ofF0JE=">AAAB6nicbVBNS8NAEJ34WetX1aOXxSLUS0m0oMeCF48V7Qe0oWy2k3bpZhN2N0IJ/QlePCji1V/kzX/jts1BWx8MPN6bYWZekAiujet+O2vrG5tb24Wd4u7e/sFh6ei4peNUMWyyWMSqE1CNgktsGm4EdhKFNAoEtoPx7cxvP6HSPJaPZpKgH9Gh5CFn1FjpoRJc9Etlt+rOQVaJl5My5Gj0S1+9QczSCKVhgmrd9dzE+BlVhjOB02Iv1ZhQNqZD7FoqaYTaz+anTsm5VQYkjJUtachc/T2R0UjrSRTYzoiakV72ZuJ/Xjc14Y2fcZmkBiVbLApTQUxMZn+TAVfIjJhYQpni9lbCRlRRZmw6RRuCt/zyKmldVr2rqntfK9dreRwFOIUzqIAH11CHO2hAExgM4Rle4c0Rzovz7nwsWtecfOYE/sD5/AGGpI0/</latexit>

c
<latexit sha1_base64="reJF7fIRK1L1tE7SseTH9oIFa+s=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipyQblilt1FyDrxMtJBXI0BuWv/jBmaYTSMEG17nluYvyMKsOZwFmpn2pMKJvQEfYslTRC7WeLQ2fkwipDEsbKljRkof6eyGik9TQKbGdEzVivenPxP6+XmvDWz7hMUoOSLReFqSAmJvOvyZArZEZMLaFMcXsrYWOqKDM2m5INwVt9eZ20a1Xvquo2ryv1Wh5HEc7gHC7Bgxuowz00oAUMEJ7hFd6cR+fFeXc+lq0FJ585hT9wPn8Awj+M2Q==</latexit>

Figure 1: (a) The position automaton for the regular expression a∗ a∗ a∗ a∗ . (b) The position
automaton for a(a∗ )(aba)∗ (b|c).
Q[1..i]. The goal of this paper is to explore the complexity of regular expression matching if these
sets are sparse. More precisely, let (R, Q) be an instance of the regular expression matching and
let A be a finite automaton that accepts the set of strings defined by R, and let S0 , . . . , Sn be the
sequence of sets of states in the simulation of A on Q. We define the density of (R, Q) wrt. A to be
∆A
R,Q =

n
X
i=0

|Si |,

i.e., the density is the total size of the state sets in the simulation of A on Q. We will focus on
density wrt. to the classic position automaton (also known as Glushkov’s automaton), denoted APos ,
proposed by Glushkov in 1960 [34, 35] and independently by McNaughton and Yamada [57]. For
most NFA constructions [4,17,33,70] (including several textbook constructions [2,3,24,51,56,69,77]),
the density wrt. to APos is a lower bound on the density wrt. to the other NFA construction. The
key observation is that the set of states in APos naturally corresponds to a subset of the set of
states in the other constructions. For instance, we can convert Thompson’s NFA, AT , into the
corresponding position automaton, APos , by carefully contracting ϵ-transitions [4, 17, 33]. This
AT
APos
Pos
implies ∆A
R,Q ≤ ∆R,Q and thus if an algorithm is efficient in terms of ∆R,Q the same algorithm
T
is also efficient in terms of ∆A
R,Q . Hence, for the rest of the paper, we define the density, denoted

Pos
∆R,Q , to be ∆A
R,Q , and when R and Q are clear from the context we simply write ∆.
Intuitively, the density captures the amount of nondeterminism in the simulation of APos . At
one extreme ∆ = n + 1 when all of the n + 1 state sets are singletons (assuming R matches Q)
and at the other extreme ∆ = nm + 1 when all of the state sets, except the special S0 , consists of
all states. The density can be significantly smaller than nm in important practical scenarios. For
instance, in internet traffic analysis a stream is matched against a large set of rules specified as
a regular expression. Typically, most of these packets will only match a small subset of the rules
implying a small density of the problem instance.
A related concept is deterministic regular expressions (also known as 1-unambiguous regular
expressions). These are defined as regular expressions for which APos is deterministic, that is, all
state-set transitions on any singleton state set result in a singleton state set. Deterministic regular
expressions are widely used in schema languages [14, 19, 30, 61] and have been extensively studied in
complexity and automata theory [18, 19, 23, 36, 54, 55, 65]. Groz and Maneth [36] showed how to
solve the deterministic regular expression matching problem in O(n log log m + m) time. Note that
if the regular expression is deterministic we always have that ∆ ≤ n + 1.

2

<!-- page 4 -->

1.2

Sparse State-Set Transitions

Given a set of S of states and a character α a state-set transition, denoted δ(S, α), is the set of
states reachable from S via paths of transitions in the NFA that match α (for ϵ-free NFAs the paths
are always single transitions). We can implement the NFA simulation using n state-set transitions
by setting S0 to be the initial state, and computing Si = δ(Si−1 , Q[i]) for i = 1, . . . , n. In our
scenario, we are interested in a compact representation of APos that supports fast sparse state-set
transitions, i.e., a state-set computation that is efficient in terms of the sizes of the input set |S|
and the output set |δ(S, α)|. Since ∆ is the total size of state sets in the simulation this implies an
efficient algorithm for sparse regular expression matching.
Surprisingly, few results are known for this problem. If we store APos explicitly we can compute
δ(S, α) by computing the union of the endpoints of transitions out of states in S labeled α. This
leads to a data structure that uses O(m2 ) space and supports state-set transitions in O(|S||δ(S, α)|)
time. Note that since endpoints of the transition may overlap (see Figure 1(a)) we may need to
explore Ω(|S||δ(S, α)|) transitions in general. A similar worst-case trade-off also holds for the many
variants of the position automaton, see e.g. [5, 20, 42, 59]. While ϵ-free NFAs with fewer transitions
are known [32,37,40,68] these do not appear to translate to simulations for APos nor do they improve
the above time bound.
Alternatively, we can store Thompson’s automaton, AT , and use the mapping of states mentioned
above to convert state-set transitions on AT to state-set transitions on APos . Since AT is not an
ϵ-free automaton we can compute a state-set transition using a breadth-first search to explore all
paths from S that match α. This uses O(m) space and O(m) time. However, it is easy to see that
with this approach we may need to traverse large subgraphs of Ω(m) transitions labeled ϵ even if
the sets |S| or δ(S, α) are sparse. Indeed, the efficient solutions in terms of n and m are based on
improving state-set transitions in AT for the dense case by polylogarithmic factors.

1.3

Results

Our main result is an efficient algorithm for sparse regular expression matching.
Theorem 1. Given a regular expression R with m positions and a string Q of length n, we can
solve the regular expression matching problem in space O(m) and time


nm
O ∆ log log
+n+m .
∆
Since the density ∆ is at most nm + 1, this essentially replaces nm with ∆ in the complexity
of regular expression matching. As an immediate Corollary of Theorem 1 we obtain a solution
to deterministic regular expression matching using O(n log log m + m) time and O(m) space, thus
matching the best known bound of Groz and Maneth [36]. We complement Theorem 1 with an
essentially matching conditional lower bound.
Theorem 2. For any ∆ = n1+γ , for any constant 0 < γ ≤ 1, there exists no O(∆1−ϵ ) time
algorithm for regular expression matching for any constant ϵ > 0 assuming SETH.
Theorem 1 is based on a compact representation of the position automaton that supports efficient
sparse state-set transitions.
Theorem 3. Given a regular expression R with m positions, we can represent the position automaton
in O(m) space and preprocessing time, such that given any set of states S in sorted order and a

3

<!-- page 5 -->

character α, we can compute the state-set transition δ(S, α) in time


m
O |S| log log
+ |δ(S, α)| .
|S|
The output of the state-set transition is also reported in sorted order.
The sorted order of S and δ(S, α) in Theorem 3 refers to the ordering of the corresponding
positions in R from left to right (without this condition the log log(m/|S|) factor becomes log log |S|)).
Theorem 3 significantly improves the previous O(|S||δ(S, α)|) and O(m) time bounds. Since any
solution must use at least Ω(|S| + |δ(S, α)|) to read the input and write the output the bound is
almost optimal.

1.4

Techniques

We develop several new insights and techniques of independent interest, including new structural
properties of the parse trees of regular expressions, a novel decomposition of state-set transitions
based on parse trees, and a fast batched predecessor data structure.
We show how to decompose any state-set transition δ(S, α) into a set of internal transitions on
a set of O(|S| + |δ(S, α)|) transition nodes of the parse tree of R. We have two types of internal
transitions: one for ⊙ and one for ∗. Intuitively, if R(v) = R(u)⊙R(w) then the internal ⊙-transition
of v wrt. α are all the states/positions in R(w) reachable from a state/position in R(u) using a
transition labeled α. The internal ∗-transitions are more complicated to describe, but both types of
internal transitions are independent of the state set S. We show how to represent R in linear space
to efficiently compute internal transitions for any node v and character α.
We identify the set of transition nodes for δ(S, α) by first computing a compact representation
of a transition tree, which encodes all paths in R containing transition nodes in O(|S|) space. Then,
we find the set of transition nodes using this tree. The key challenge is that even though the
representation of the transition tree is small the tree itself can be significantly larger and contain
many nodes that are irrelevant for the character α and/or irrelevant for the state set S. Using the
structural properties of the parse tree we show how to overcome the challenges and efficiently find
the set of transition nodes for any S and α. Computing the internal transitions of all transition
nodes could take too long, as the output of these overlap and we could end up using Ω(|S||δ(S, α)|)
time. However, we prove that these output sets form a laminar family and show how to divide the
computations of the internal transitions into computations on a bounded number of non-overlapping
intervals.
In combination, the above techniques lead to an O(m) space representation of R that supports
state-set transitions in O(|S| log log m + |δ(S, α)|) time. The bottleneck here is computing a O(|S|)
predecessor queries in O(log log m) time. We present a simple two-level data structure that solves
m
this batched predecessor problem in O(|S| log log |S|
) time while maintaining linear space leading to
our final structure. Using this solution for sparse-set transitions to implement the NFA simulation
implies our main result for sparse regular expression matching of Theorem 1.
The lower bound follows from a reduction from the orthogonal vectors problem (OVP). We
prove that given ∆ = n1+γ , for any constant 0 < γ ≤ 1, we can construct an instance of regular
expression matching such that the existence an O(∆1−ϵ ) algorithm for regular expression matching
violates SETH. The reduction is based on the reduction by Backurs and Indyk [7] and is a fairly
straightforward generalization of their lower bound.

4

<!-- page 6 -->

1.5

Related Work

Another NFA construction, by Chang and Paige [21], considered compact representations of APos that
support efficiently implementing NFA to DFA conversion by subset construction. They presented a
linear space representation that supports efficiently computing the set of states S ′ reachable via any
character from a state-set S in time O(|S| + |S ′ |). Since S ′ can be much larger than δ(S, α) this
does not imply an efficient sparse state-set transition.
Some measures of nondeterminism of NFAs have been studied in automata theory, e.g., width,
ambiguity, string tree width, string path width, and cycle height [39, 45, 46, 50, 53]. These focus on
the complexity of computing measures of the nondeterminism of a given NFA. In contrast, we study
the complexity of regular expression matching in terms of the nondeterminism of a simulation on a
given NFA and input string.
As mentioned, Bille and Thorup [13] considered the number of strings k ≤ m in the regular
expression as a parameter for regular expression matching. They gave an algorithm using O(nk logww +
n+m log k) = O(nk +m log k) time. It is straightforward to construct instances of regular expression
matching (for a matching regular expression) such that either nk = Θ(nm) and ∆ = Θ(n) or
nk = Θ(n) and ∆ = Θ(nm) hence this result is incomparable to ours. Cotumaccio, D’Agostino,
Policriti, and Prezza [22] studied the indexing version of regular expressions, where the goal
is to preprocess a regular expression in order to allow for fast matching given a query string.
They considered the co-lexicographic width of an automaton. Applying their construction in the
matching setting gives an algorithm that runs in O(m2 + np2 log(p · σ)), where p is the width of the
co-lexicographic order and m2 comes from the preprocessing of the automaton.
Several papers have studied the related problem of string matching in labeled graphs. For
example, Rizzo, Tomescu, and Policriti [66] studied the problem of matching a pattern on a
labeled graph parameterized wrt the size of the labeled direct product graph, and Nellore, Nguyen,
and Thompson [64] studied string matching in graphs parameterized by the size of the powerset
automaton.
Finally, we note that sparsity is a well-studied phenomenon in a wide range of areas in computer
science. In particular, sparsity has been extensively studied for other classic pattern matching
problems, see, e.g., [6, 27, 28, 41, 73, 74].

1.6

Outline

We review regular expressions and automata in Section 2 and the parse tree view of regular
expressions in Section 3. We introduce internal transitions, state-set decompositions, and transition
trees in Section 4 and present our main algorithm for sparse state-set transitions in Section 5. In
Section 6, we present the improved batched predecessor data structure. We use this to obtain the
final result for sparse state-set transitions of Theorem 3 which we then use to obtain Theorem 1.
Finally, we show the lower bound of Theorem 2 in Section 7.

2

Regular Expressions and Automata

We briefly review the classical concepts used in the paper. For more details, see, e.g., Aho et al. [2].
Regular Expressions We consider the set of non-empty regular expressions over an alphabet
Σ, defined recursively as follows. If α ∈ Σ ∪ {ϵ} then α is a regular expression, and if S and T are
regular expressions then so is the concatenation, (S) ⊙ (T ), the union, (S)|(T ), and the star, (S)∗ .
We often omit the concatenation ⊙ when writing regular expressions. The language L(R) generated
5

<!-- page 7 -->

by a regular expression R is defined as follows. If α ∈ Σ ∪ {ϵ}, then L(α) is the set containing the
single string α. If S and T are regular expressions, then L(S ⊙ T ) = L(S) ⊙ L(T ), that is, any string
formed by theSconcatenation of a string in L(S) with a string in L(T ), L(S)|L(T ) = L(S) ∪ L(T ),
and L(S ∗ ) = i≥0 L(S)i , where L(S)0 = {ϵ} and L(S)i = L(S)i−1 ⊙ L(S), for i > 0.
Finite Automata A finite automaton is a tuple A = (V, E, Σ, Θ, Φ), where V is a set of nodes
called states, E ⊆ (V × V × Σ ∪ {ϵ}) is a set of directed edges between states called transitions each
labeled by a character from Σ ∪ {ϵ}, Θ ⊆ V is a set of start states, and Φ ⊆ V is a set accepting
states. In short, A is an edge-labeled directed graph with designated subsets of start and accepting
nodes. A is a deterministic finite automaton (DFA) if A does not contain any ϵ-transitions, all
outgoing transitions of any state have different labels, and there is exactly one start state. Otherwise,
A is a nondeterministic finite automaton (NFA).
Given a string Q and a path p in A we say that p and Q match if the concatenation of the
labels on the transitions in p is Q. Given a state s in A and a character α we define the state-set
transition δA (s, α) to be the set of states reachable from s through paths matching α (note
that the
S
paths may include transitions labeled ϵ). For a set of states S we define δA (S, α) = s∈S δA (s, α).
We say that A accepts a string Q if there is a path from a state in Θ to a state in Φ that matches
Q. Otherwise, A rejects Q. We can use a sequence of state-set transitions to test acceptance of a
string Q of length n by computing a sequence of state-sets S0 , . . . , Sn , given by S0 = δA (Θ, ϵ) and
Si = δA (Si−1 , Q[i]), i = 1, . . . , n. We have that Φ ∩ Sn ̸= ∅ iff A accepts Q.
The Position Automaton Given a regular expression R, we can construct an NFA accepting
precisely the strings in L(R) by several classic methods [34, 57, 70]. In particular, Glushkov gave
an important construction called the position automaton or Glushkov automaton. The position
automaton is an ϵ-free NFA consisting of only m + 1 states and O(m2 ) transitions (See Figure 1).
Each state except the start state corresponds to a position. Intuitively, each state-set in a state-set
simulation is the set of positions in R that correspond to a match of a prefix of Q.
We review the details of the position automaton in the following. Let R be a regular expression
with m character symbols from an alphabet Σ. The position of a character in R is the index of the
character in the left-to-right order among the characters in R. The set of positions in R, denoted
Pos(R), is the set {1, . . . , m}. The label of a position p, denoted label(p), is the character at position
p. The subset of positions labeled α is denoted Posα (R). When R is clear from the context we
abbreviate Pos(R) to phb Pos.
The marked regular expression of R, denoted R, is obtained from R by subscripting each character
in R with its position. Similarly, the marked alphabet, denoted Σ, is obtained from Σ by adding
subscripts. The marked regular expression R defines the language L(R) over the marked alphabet
Σ. Note that Pos(R) = Pos(R). Given a position p we define label(p) to be the label of p in R. The
first and last set of R represent the positions that match the first and last character, respectively, in
some string in L(R). Given a position p, the follow set of R and p is the set of positions that can
follow a position p in L(R). More precisely,
∗

first(R) = {p ∈ Pos(R) | ∃s ∈ Σ , label(p) ⊙ s ∈ L(R))}
∗

last(R) = {p ∈ Pos(R) | ∃s ∈ Σ , s ⊙ label(p) ∈ L(R))}
∗

follow(R, p) = {q ∈ Pos(R) | ∃s, t ∈ Σ , s ⊙ label(p) ⊙ label(q) ⊙ t ∈ L(R))}

6

<!-- page 8 -->

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v1

|
v3

<latexit sha1_base64="4jJRhQri4Qs0CWbrp7OwtGEEc8s=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHivYD2lA22027dLMJu5NCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNECoOu++0UtrZ3dveK+6WDw6Pjk/LpWdvEqWa8xWIZ625ADZdC8RYKlLybaE6jQPJOMLlb1DtTro2I1RPOEu5HdKREKBhFaz1OB96gXHGr7lJkE7wcKpCrOSh/9YcxSyOukElqTM9zE/QzqlEwyeelfmp4QtmEjnjPoqIRN362XHVOrqwzJGGs7VNIlu7viYxGxsyiwHZGFMdmvbYw/6v1Ugxv/UyoJEWu2OqjMJUEY7K4mwyF5gzlzAJlWthdCRtTTRnadEo2BG/95E1o31Q9yw+1SqOWx1GEC7iEa/CgDg24hya0gMEInuEV3hzpvDjvzseqteDkM+fwR87nDwRwjZA=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="YL4rWaB3wCpeqPEXmefuNPpoPjU=">AAAB63icbZDLSgMxFIZP6q3WW9Wlm2ARXJUZEXRZcOOygr1AO5RMJtOGJpkhyQhl6Cu4caGIW1/InW9jpp2Ftv4Q+PjPOeScP0wFN9bzvlFlY3Nre6e6W9vbPzg8qh+fdE2Saco6NBGJ7ofEMMEV61huBeunmhEZCtYLp3dFvffEtOGJerSzlAWSjBWPOSW2sIaSR6N6w2t6C+F18EtoQKn2qP41jBKaSaYsFcSYge+lNsiJtpwKNq8NM8NSQqdkzAYOFZHMBPli1zm+cE6E40S7pyxeuL8nciKNmcnQdUpiJ2a1Vpj/1QaZjW+DnKs0s0zR5UdxJrBNcHE4jrhm1IqZA0I1d7tiOiGaUOviqbkQ/NWT16F71fQdP1w3WtdlHFU4g3O4BB9uoAX30IYOUJjAM7zCG5LoBb2jj2VrBZUzp/BH6PMHBLOOKg==</latexit>

v2
<latexit sha1_base64="Y8iuL0s/S6j+GFGp7O/VRtSriAQ=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FSSUtBjwYvHivYD2lA220m7dLMJu5tCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs7aOU8WwxWIRq25ANQousWW4EdhNFNIoENgJJneLemeKSvNYPplZgn5ER5KHnFFjrcfpoDYoV9yquxTZBC+HCuRqDspf/WHM0gilYYJq3fPcxPgZVYYzgfNSP9WYUDahI+xZlDRC7WfLVefkyjpDEsbKPmnI0v09kdFI61kU2M6ImrFery3M/2q91IS3fsZlkhqUbPVRmApiYrK4mwy5QmbEzAJlittdCRtTRZmx6ZRsCN76yZvQrlU9yw/1SqOex1GEC7iEa/DgBhpwD01oAYMRPMMrvDnCeXHenY9Va8HJZ87hj5zPHwX0jZE=</latexit>

<latexit sha1_base64="NgrFx/JBikjR0a7O+X3fJ8RkpEU=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSLeix4MVjRfsBbSib7aRdutmE3U2hhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEiuDau++0UNja3tneKu6W9/YPDo/LxSUvHqWLYZLGIVSegGgWX2DTcCOwkCmkUCGwH47t5vT1BpXksn8w0QT+iQ8lDzqix1uOkf90vV9yquxBZBy+HCuRq9MtfvUHM0gilYYJq3fXcxPgZVYYzgbNSL9WYUDamQ+xalDRC7WeLVWfkwjoDEsbKPmnIwv09kdFI62kU2M6ImpFerc3N/2rd1IS3fsZlkhqUbPlRmApiYjK/mwy4QmbE1AJlittdCRtRRZmx6ZRsCN7qyevQuqp6lh9qlXotj6MIZ3AOl+DBDdThHhrQBAZDeIZXeHOE8+K8Ox/L1oKTz5zCHzmfPwd4jZI=</latexit>

a
p1
<latexit sha1_base64="BqxKyohfuZK4UkYEG9pGCIF/PpM=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0LmpepZbtUqjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w+/KYzX</latexit>

<latexit sha1_base64="zCLnLU08esy4y2JDR3R4aUla7s4=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKeix4MVjRdMW2lA22027dLMJuxOhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvGEqhUHX/XZKG5tb2zvl3cre/sHhUfX4pG2STDPus0QmuhtSw6VQ3EeBkndTzWkcSt4JJ7fzeueJayMS9YjTlAcxHSkRCUbRWg/pwBtUa27dXYisg1dADQq1BtWv/jBhWcwVMkmN6XluikFONQom+azSzwxPKZvQEe9ZVDTmJsgXq87IhXWGJEq0fQrJwv09kdPYmGkc2s6Y4tis1ubmf7VehtFNkAuVZsgVW34UZZJgQuZ3k6HQnKGcWqBMC7srYWOqKUObTsWG4K2evA7tq7pn+b5RazaKOMpwBudwCR5cQxPuoAU+MBjBM7zCmyOdF+fd+Vi2lpxi5hT+yPn8Afs9jYo=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v4
⇤
<latexit sha1_base64="/SQRcrQmWtX4evaiuYQBxbwShyI=">AAAB63icbZBNSwMxEIZn61etX1WPXoJF8FR2RdBjwYvHCvYD2qVk02wbmmSXZFYopX/BiwdFvPqHvPlvzLZ70NYXAg/vzJCZN0qlsOj7315pY3Nre6e8W9nbPzg8qh6ftG2SGcZbLJGJ6UbUcik0b6FAybup4VRFkneiyV1e7zxxY0WiH3Ga8lDRkRaxYBRzq08tDqo1v+4vRNYhKKAGhZqD6ld/mLBMcY1MUmt7gZ9iOKMGBZN8XulnlqeUTeiI9xxqqrgNZ4td5+TCOUMSJ8Y9jWTh/p6YUWXtVEWuU1Ec29Vabv5X62UY34YzodMMuWbLj+JMEkxIfjgZCsMZyqkDyoxwuxI2poYydPFUXAjB6snr0L6qB44frmuN6yKOMpzBOVxCADfQgHtoQgsYjOEZXuHNU96L9+59LFtLXjFzCn/kff4AGd2OOA==</latexit>

b
p6

<latexit sha1_base64="77NFmdMQ8Sc114H97fDGaKxLv/I=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdVjwYvHivYD2lA22027dLMJuxOhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEihUHX/XYKG5tb2zvF3dLe/sHhUfn4pG3iVDPeYrGMdTeghkuheAsFSt5NNKdRIHknmNzO650nro2I1SNOE+5HdKREKBhFaz0kg/qgXHGr7kJkHbwcKpCrOSh/9YcxSyOukElqTM9zE/QzqlEwyWelfmp4QtmEjnjPoqIRN362WHVGLqwzJGGs7VNIFu7viYxGxkyjwHZGFMdmtTY3/6v1Ugxv/EyoJEWu2PKjMJUEYzK/mwyF5gzl1AJlWthdCRtTTRnadEo2BG/15HVoX1U9y/e1SqOWx1GEMziHS/DgGhpwB01oAYMRPMMrvDnSeXHenY9la8HJZ07hj5zPHwLgjY8=</latexit>

⇤

v5

v6

<latexit sha1_base64="/SQRcrQmWtX4evaiuYQBxbwShyI=">AAAB63icbZBNSwMxEIZn61etX1WPXoJF8FR2RdBjwYvHCvYD2qVk02wbmmSXZFYopX/BiwdFvPqHvPlvzLZ70NYXAg/vzJCZN0qlsOj7315pY3Nre6e8W9nbPzg8qh6ftG2SGcZbLJGJ6UbUcik0b6FAybup4VRFkneiyV1e7zxxY0WiH3Ga8lDRkRaxYBRzq08tDqo1v+4vRNYhKKAGhZqD6ld/mLBMcY1MUmt7gZ9iOKMGBZN8XulnlqeUTeiI9xxqqrgNZ4td5+TCOUMSJ8Y9jWTh/p6YUWXtVEWuU1Ec29Vabv5X62UY34YzodMMuWbLj+JMEkxIfjgZCsMZyqkDyoxwuxI2poYydPFUXAjB6snr0L6qB44frmuN6yKOMpzBOVxCADfQgHtoQgsYjOEZXuHNU96L9+59LFtLXjFzCn/kff4AGd2OOA==</latexit>

a
p2
<latexit sha1_base64="BqxKyohfuZK4UkYEG9pGCIF/PpM=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0LmpepZbtUqjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w+/KYzX</latexit>

<latexit sha1_base64="yeKJFrsXU80HnFxHI2dtmAju3PM=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FSSUqjHghePFe0HtKFstpt26WYTdidCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNECoOu++0UtrZ3dveK+6WDw6Pjk/LpWcfEqWa8zWIZ615ADZdC8TYKlLyXaE6jQPJuML1d1LtPXBsRq0ecJdyP6FiJUDCK1npIhrVhueJW3aXIJng5VCBXa1j+GoxilkZcIZPUmL7nJuhnVKNgks9Lg9TwhLIpHfO+RUUjbvxsueqcXFlnRMJY26eQLN3fExmNjJlFge2MKE7Mem1h/lfrpxje+JlQSYpcsdVHYSoJxmRxNxkJzRnKmQXKtLC7EjahmjK06ZRsCN76yZvQqVU9y/f1SrOex1GEC7iEa/CgAU24gxa0gcEYnuEV3hzpvDjvzseqteDkM+fwR87nD/zBjYs=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v7
a
p3
<latexit sha1_base64="BqxKyohfuZK4UkYEG9pGCIF/PpM=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0LmpepZbtUqjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w+/KYzX</latexit>

<latexit sha1_base64="nvPCcyEqGvc65FiwQU/FSYSOLws=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSLdRjwYvHivYD2lA22027dLMJuxOhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEihUHX/XYKG5tb2zvF3dLe/sHhUfn4pG3iVDPeYrGMdTeghkuheAsFSt5NNKdRIHknmNzO650nro2I1SNOE+5HdKREKBhFaz0kg+tBueJW3YXIOng5VCBXc1D+6g9jlkZcIZPUmJ7nJuhnVKNgks9K/dTwhLIJHfGeRUUjbvxsseqMXFhnSMJY26eQLNzfExmNjJlGge2MKI7Nam1u/lfrpRje+JlQSYpcseVHYSoJxmR+NxkKzRnKqQXKtLC7EjammjK06ZRsCN7qyevQvqp6lu9rlUYtj6MIZ3AOl+BBHRpwB01oAYMRPMMrvDnSeXHenY9la8HJZ07hj5zPH/5FjYw=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v8
b
p4
<latexit sha1_base64="AeMJS4Zve980dZjymK/2xpflTW4=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWq1gWK64VXcpsgleDhXI1RyWvwajmKURSsME1brvuYnxM6oMZwLnpUGqMaFsSsfYtyhphNrPlovOyZV1RiSMlX3SkKX7eyKjkdazKLCdETUTvV5bmP/V+qkJb/2MyyQ1KNnqozAVxMRkcTUZcYXMiJkFyhS3uxI2oYoyY7Mp2RC89ZM3oXNT9Sy3apVGLY+jCBdwCdfgQR0acA9NaAMDhGd4hTfn0Xlx3p2PVWvByWfO4Y+czx/ArYzY</latexit>

<latexit sha1_base64="16vruzNKyUMJ7jbY1mPotqiFSYA=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKeix4MVjRdMW2lA220m7dLMJuxuhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvGEquDau++2UNja3tnfKu5W9/YPDo+rxSVsnmWLos0QkqhtSjYJL9A03ArupQhqHAjvh5HZe7zyh0jyRj2aaYhDTkeQRZ9RY6yEdNAbVmlt3FyLr4BVQg0KtQfWrP0xYFqM0TFCte56bmiCnynAmcFbpZxpTyiZ0hD2Lksaog3yx6oxcWGdIokTZJw1ZuL8nchprPY1D2xlTM9artbn5X62XmegmyLlMM4OSLT+KMkFMQuZ3kyFXyIyYWqBMcbsrYWOqKDM2nYoNwVs9eR3aV3XP8n2j1mwUcZThDM7hEjy4hibcQQt8YDCCZ3iFN0c4L86787FsLTnFzCn8kfP5A//JjY0=</latexit>

c
p7
<latexit sha1_base64="r3ejAKtl8dLj9kbqri6Oudn5p3g=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi02LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0LmpepZbtUqjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w/CMYzZ</latexit>

<latexit sha1_base64="AeMJS4Zve980dZjymK/2xpflTW4=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWq1gWK64VXcpsgleDhXI1RyWvwajmKURSsME1brvuYnxM6oMZwLnpUGqMaFsSsfYtyhphNrPlovOyZV1RiSMlX3SkKX7eyKjkdazKLCdETUTvV5bmP/V+qkJb/2MyyQ1KNnqozAVxMRkcTUZcYXMiJkFyhS3uxI2oYoyY7Mp2RC89ZM3oXNT9Sy3apVGLY+jCBdwCdfgQR0acA9NaAMDhGd4hTfn0Xlx3p2PVWvByWfO4Y+czx/ArYzY</latexit>

<latexit sha1_base64="P3RPka0mlcU2JXTpcEUtPVD7o44=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKbTHghePFe0HtKFstpN26WYTdjdCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpreLevcJleaxfDSzBP2IjiUPOaPGWg/JsD4sV9yquxTZBC+HCuRqDctfg1HM0gilYYJq3ffcxPgZVYYzgfPSINWYUDalY+xblDRC7WfLVefkyjojEsbKPmnI0v09kdFI61kU2M6Imolery3M/2r91IQNP+MySQ1KtvooTAUxMVncTUZcITNiZoEyxe2uhE2ooszYdEo2BG/95E3o3FQ9y/e1SrOWx1GEC7iEa/CgDk24gxa0gcEYnuEV3hzhvDjvzseqteDkM+fwR87nDwRkjZA=</latexit>

v
v1
v2
v3
v4
v5
v6
v7
v8

fi s (v)
{p1 }
{p1 }
{p6 p7 }
{p2 p3 }
{p2 }
{p3 }
{p3 }
{p4 }

as (v)
{p6 p7 }
{p1 p2 p5 }
{p6 p7 }
{p2 p5 }
{p2 }
{p5 }
{p5 }
{p5 }

p
p1
p2
p3
p4
p5
p6
p7

o ow(v1 p)
{p2 p3 p6 p7 }
{p2 p3 p6 p7 }
{p4 }
{p5 }
{p3 p6 p7 }
✏
✏

o ow(v2 p)
{p2 p3 }
{p2 p3 }
{p4 }
{p5 }
{p3 }

a
p5
<latexit sha1_base64="BqxKyohfuZK4UkYEG9pGCIF/PpM=">AAAB6HicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHFuwHtKFstpN27WYTdjdCCf0FXjwo4tWf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs46OU8WwzWIRq15ANQousW24EdhLFNIoENgNpneLevcJleaxfDCzBP2IjiUPOaPGWi06LFfcqrsU2QQvhwrkag7LX4NRzNIIpWGCat333MT4GVWGM4Hz0iDVmFA2pWPsW5Q0Qu1ny0Xn5Mo6IxLGyj5pyNL9PZHRSOtZFNjOiJqJXq8tzP9q/dSEt37GZZIalGz1UZgKYmKyuJqMuEJmxMwCZYrbXQmbUEWZsdmUbAje+smb0LmpepZbtUqjlsdRhAu4hGvwoA4NuIcmtIEBwjO8wpvz6Lw4787HqrXg5DPn8EfO5w+/KYzX</latexit>

<latexit sha1_base64="+6UVKniJdWB72V1Ex8gU8/fejUM=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSadFjwYvHivYD2lA22027dLMJuxOhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEihUHX/XYKG5tb2zvF3dLe/sHhUfn4pG3iVDPeYrGMdTeghkuheAsFSt5NNKdRIHknmNzO650nro2I1SNOE+5HdKREKBhFaz0kg/qgXHGr7kJkHbwcKpCrOSh/9YcxSyOukElqTM9zE/QzqlEwyWelfmp4QtmEjnjPoqIRN362WHVGLqwzJGGs7VNIFu7viYxGxkyjwHZGFMdmtTY3/6v1Ugxv/EyoJEWu2PKjMJUEYzK/mwyF5gzl1AJlWthdCRtTTRnadEo2BG/15HVoX1U9y/e1SqOWx1GEMziHS/DgGhpwB01oAYMRPMMrvDnSeXHenY9la8HJZ07hj5zPHwFcjY4=</latexit>

Figure 2: The parse tree for the expression a(a∗ )(aba)∗ (b|c), the corresponding first and ast sets,
and the fo ow sets for v1 and v2 .
We then define the position automaton for R as the NFA A = (V, E, {0}, F ), where
V = Pos(R) ∪ {0},
E = {(0, q, abe (q)) | q ∈ first(R)} ∪

[
p∈Pos

{(p, q, abe (q)) | q ∈ fo ow(R, p)}

(
{0} ∪ ast(R) if ϵ ∈ L(R),
F =
ast(R)
otherwise

3

Regular Expressions as Trees

Throughout the rest of the paper, let R be a regular expression with m positions and let δ denote
the state-set trans t on funct on of the pos t on automaton for R For s mp c ty n the presentat on
we w focus on mp ement ng δ on the pos t ons of R and gnore the extra start state of the pos t on
automaton The extra start state s stra ghtforward to represent w th add t ona near space and s
only needed in the initial step of state-set simulations.
We identify regular expressions by their parse trees (see Figure 2). Note that the leaves in
left-to-right order are the positions. We call the three types of internal nodes ⊙-nodes, ∗-nodes,
and |-nodes. For a |-node or ⊙-node v the left and right child are denoted eft(v) and r ght(v),
respectively, and for a ∗-node the single child is denoted eft(v). The depth of a node v in R is
the number of edges on the path from the root of R to v. We denote the subtree (equivalently
subexpression) rooted at a node v by R(v). If u ∈ R(v) then v is an ancestor of u, denoted v ⪯ u,
and if u ∈ R(v) and u = v then v is a proper ancestor of u, denoted v ≺ u. If v is a (proper)
ancestor of u then u s a (proper) descendant of v A node w s a common ancestor of u and v f t
is an ancestor of both u and v. The owest common ancestor of u and v, ca(u, v), is the common
ancestor of u and v of greatest depth. The owest star ancestor of a node v, denoted parent∗ (v), is
the lowest ancestor of v that is a ∗-node.
We extend the definition of labels to internal nodes. For each internal node v, the label of v,
denoted abe (v) s a set of characters such that α ∈ abe (v) ff v = ca(p, q) for some pos t ons p and
q both abe ed α In F gure 2 abe (v2 ) = {a} s nce v2 = ca(p1 , p2 ) and abe (p1 ) = abe (p2 ) = {a}
Since the total number of internal nodes containing label α is |Posα | − 1 the total size of all labels
is O(m). For a node v we extend our notation to define Pos(v), first(v), ast(v), and fo ow(v, p) to
denote the sets on the subexpression R(v) (see Figure 2).
7

<!-- page 9 -->

In our setting, we will often view the first and last sets from the perspective of a single
position p and consider the nodes for which p appears in the corresponding first and last sets,
respectively. Specifically, we define the first extent and last extent of a position p, respectively,
to be the set of nodes in R given by firstextent(p) = {v | p ∈ first(v)} and lastextent(p)
= {v |
S
p ∈ last(v)}. Furthermore,
for a set of positions P , we write firstextent(P ) = p∈P firstextent(p)
S
and lastextent(P ) = p∈P lastextent(p). For instance, in Figure 2 we have firstextent({p3 , p6 }) =
{p3 , v7 , v6 , v4 , p6 , v3 }. We define the first extent and last extent of an internal node v, to be the
sets firstextent(v) = {u | u ⪯ v and u ∈ firstextent(Pos(v))} and lastextent(v) = {u | u ⪯ v and u ∈
lastextent(Pos(v))}, respectively.
The first sets and the last sets, respectively, form a laminar family. That is, for any two nodes
in the parse tree, their first sets, respectively, last sets, are either disjoint or one is contained in the
other. That implies that the set of nodes in firstextent(p), respectively, lastextent(p), forms a path
from position p to an ancestor of p.
Lemma 4. Let p be a position in a regular expression R and let v and u be nodes in R such
that u ⪯ v ⪯ p. If u ∈ firstextent(p), then v ∈ firstextent(p) and if u ∈ lastextent(p), then
v ∈ lastextent(p).
Proof. We have that R(v) is a subexpression of R(u) and p is a position in R(v). Then, if p ∈ first(u)
then p ∈ first(v). Similarly, if p ∈ last(u) then p ∈ last(v).

4

Internal Transitions, State-Set Decompositions, and Transition
Trees

We now introduce the main structural properties of state-set transitions that we need for our fast
sparse state-set transition algorithm in Section 5. We first characterize state-set transitions in the
position automaton in terms of firstextent and lastextent using the following important property.
Lemma 5. Let p ∈ Pos and q ∈ Posα and v = lca(p, q). Then, q ∈ δ(p, α) iff either
(i) v is a ⊙-node, left(v) ∈ lastextent(p), and right(v) ∈ firstextent(q), or
(ii) parent∗ (v) ∈ lastextent(p) ∩ firstextent(q).
Lemma 5 has appeared in various forms in earlier work [21, 36, 65]. For our purposes, we state
it in terms of lowest common ancestors and first extents and last extents. Lemma 5 states that a
position q can only appear in δ(p, α) through a ⊙-node or a ∗-node. We write q ∈ δ ⊙ (p, α) if (i) is
satisfied and q ∈ δ ∗ (p, α) if (ii) is satisfied.

4.1

Internal Transitions

Given an internal node v, an internal transition on v and a character α will correspond to the
conditions on q and v in Lemma 5 while ignoring the condition on p. In general, we will also specify
a range of positions we are interested in. Formally, given an internal node v, a character α, and
positions l and r, define the internal ⊙-transition and internal ∗-transition, denoted δ ⊙ (v, α) and
δ ∗ (v, α), respectively, as follows.
⊙
δ[l,r]
(v, α) = {q ∈ Posα | right(v) ∈ firstextent(q) and q ∈ [l, r]}

if v is a ⊙-node

∗
δ[l,r]
(v, α) = {q ∈ Posα ∩ Pos(v) | parent∗ (v) ∈ firstextent(q) and q ∈ [l, r]} if v is a ∗-node

8

<!-- page 10 -->

⊙
When the range includes all positions we drop the subscript, that is, δ ⊙ (v, α) = δ[1,m]
(v, α) and
⊙
∗
∗
⊙
δ (v, α) = δ[1,m] (v, α). For instance, in Figure 2 we have δ (v2 , a) = {p2 , p3 }, δ[3,5] (v2 , a) = {p3 },
δ ⊙ (v1 , c) = {p7 } and δ ∗ (v7 , a) = {p3 }.

4.2

Transition Nodes

Given a state-set transition δ(P, α), the transitions nodes are a set of nodes N such that if we
compute the union of internal transitions on N we obtain δ(P, α). Formally, we define the ⊙transition nodes and ∗-transition nodes of δ(P, α), denoted N ⊙ (P, α) and N ∗ (P, α), respectively,
as
N ⊙ (P, α) = {v | v is a ⊙-node and left(v) ∈ lastextent(P ) and right(v) ∈ firstextent(Posα )}
N ∗ (P, α) = {v | there exists q ∈ Posα and p ∈ P such that v = lca(p, q)

and parent∗ (v) ∈ lastextent(p) ∩ firstextent(q)}

In combination, the set of transition nodes is the union of the ⊙-transition nodes and the ∗-transition
nodes.
Lemma 6. For any set of positions P and a character α,
[
δ(P, α) =
δ ⊙ (v, α)
∪
v∈N ⊙ (P,α)

[

δ ∗ (v, α) .

(1)

v∈N ∗ (P,α)

Proof. Let RH denote the right handside of (1).
We first show that δ(P, α) ⊆ RH. Let p ∈ P and q ∈ Pos be positions with v = lca(p, q)
such that q ∈ δ(p, α). Then, q ∈ Posα and hence p, q, and v satisfies either case (i) or (ii)
in Lemma 5. If (i) is satisfied, v is a ⊙-node and left(v) ∈ lastextent(p) ⊆ lastextent(P ) and
right(v) ∈ firstextent(q) ⊆ lastextent(Posα ). By definition, v ∈ N ⊙ (P, α), and thus q ∈ δ ⊙ (v, α) ⊆
∪v∈N ⊙ (P,α) δ ⊙ (v, α). Similarly, if (ii) is satisfied, then parent∗ (v) ∈ lastextent(p) ∩ firstextent(q), and
it follows v ∈ N ∗ (P, α). This implies that q ∈ δ ∗ (v, α) ⊆ ∪v∈N ∗ (P,α) δ ∗ (v, α).
To show RH ⊆ δ(P, α) first suppose q ∈ ∪v∈N ⊙ (P,α) δ ⊙ (v, α). Then, q ∈ Posα and there is
a ⊙-node v such that right(v) ∈ firstextent(q) and left(v) ∈ lastextent(P ), which implies that
v = lca(p, q) for some p ∈ P . By Lemma 5(i) q ∈ δ(P, α). If q ∈ ∪v∈N ∗ (P,α) δ ∗ (v, α) then there
exists a v ∈ N ∗ (P, α) such that q ∈ Pos(v) and parent∗ (v) ∈ firstextent(q). It follows from the
definition of N ∗ (P, α) and By Lemma 5(ii) that there exists a p ∈ P and a q ′ ∈ P osα such that
v = lca(p, q ′ ) and parent∗ ∈ lastextent(p) ∩ firstextent(q ′ ). Since both p and q are descendants of v we
have that u = lca(p, q) is a (not necessarily proper) descendant of v. It follows from Lemma 4 that
u ∈ lastextent(p) and parent∗ (u) ∈ firstextent(q). Thus by Lemma 5(ii) we have q ∈ δ(P, α).
We show that the total size of the two sets N ⊙ (P, α) and N ∗ (P, α) is O(|P | + |δ(P, α)|).
Lemma 7. We have |N ⊙ (P, α)| ≤ |P | + |δ ⊙ (P, α)| − 1 and |N ∗ (P, α)| ≤ |P | + |δ ∗ (P, α)| − 1.
Proof. By definition every node in N ⊙ (P, α) is the lowest common ancestor of some position p ∈ P
and some position q ∈ δ ⊙ (P, α). The number of distinct pairwise lowest common ancestors of a
subset of ℓ leaves in a tree cannot exceed ℓ − 1. Therefore, the number of lowest common ancestors
between positions in P and positions in δ ⊙ (P, α) can never be larger than |P | + |δ ⊙ (P, α)| − 1. The
same argument holds for the number of nodes in |N ∗ (P, α)|.

9

<!-- page 11 -->

⇤
<latexit sha1_base64="/SQRcrQmWtX4evaiuYQBxbwShyI=">AAAB63icbZBNSwMxEIZn61etX1WPXoJF8FR2RdBjwYvHCvYD2qVk02wbmmSXZFYopX/BiwdFvPqHvPlvzLZ70NYXAg/vzJCZN0qlsOj7315pY3Nre6e8W9nbPzg8qh6ftG2SGcZbLJGJ6UbUcik0b6FAybup4VRFkneiyV1e7zxxY0WiH3Ga8lDRkRaxYBRzq08tDqo1v+4vRNYhKKAGhZqD6ld/mLBMcY1MUmt7gZ9iOKMGBZN8XulnlqeUTeiI9xxqqrgNZ4td5+TCOUMSJ8Y9jWTh/p6YUWXtVEWuU1Ec29Vabv5X62UY34YzodMMuWbLj+JMEkxIfjgZCsMZyqkDyoxwuxI2poYydPFUXAjB6snr0L6qB44frmuN6yKOMpzBOVxCADfQgHtoQgsYjOEZXuHNU96L9+59LFtLXjFzCn/kff4AGd2OOA==</latexit>

v0
<latexit sha1_base64="jqfwRaSv+eh3nTL20vqMX5MxO+o=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2lpoQ9lsN+3SzSbsTgol9Cd48aCIV3+RN/+N2zYHbX0w8Hhvhpl5QSKFQdf9dgobm1vbO8Xd0t7+weFR+fikbeJUM95isYx1J6CGS6F4CwVK3kk0p1Eg+VMwvp37TxOujYjVI04T7kd0qEQoGEUrPUz6br9ccavuAmSdeDmpQI5mv/zVG8QsjbhCJqkxXc9N0M+oRsEkn5V6qeEJZWM65F1LFY248bPFqTNyYZUBCWNtSyFZqL8nMhoZM40C2xlRHJlVby7+53VTDG/8TKgkRa7YclGYSoIxmf9NBkJzhnJqCWVa2FsJG1FNGdp0SjYEb/XlddKuVb2rav2+XmnU8jiKcAbncAkeXEMD7qAJLWAwhGd4hTdHOi/Ou/OxbC04+cwp/IHz+QMFPo2W</latexit>

u0
<latexit sha1_base64="BHSH6IoW0ELbx8FS97g0PMEETjY=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx4r2g9oQ9lsJ+3SzSbsboQS+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpIR24g3LFrboLkHXi5aQCOZqD8ld/GLM0QmmYoFr3PDcxfkaV4UzgrNRPNSaUTegIe5ZKGqH2s8WpM3JhlSEJY2VLGrJQf09kNNJ6GgW2M6JmrFe9ufif10tNeONnXCapQcmWi8JUEBOT+d9kyBUyI6aWUKa4vZWwMVWUGZtOyYbgrb68Ttq1qlevevdXlUYtj6MIZ3AOl+DBNTTgDprQAgYjeIZXeHOE8+K8Ox/L1oKTz5zCHzifPwLCjZI=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v0

⇤

<latexit sha1_base64="jqfwRaSv+eh3nTL20vqMX5MxO+o=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2lpoQ9lsN+3SzSbsTgol9Cd48aCIV3+RN/+N2zYHbX0w8Hhvhpl5QSKFQdf9dgobm1vbO8Xd0t7+weFR+fikbeJUM95isYx1J6CGS6F4CwVK3kk0p1Eg+VMwvp37TxOujYjVI04T7kd0qEQoGEUrPUz6br9ccavuAmSdeDmpQI5mv/zVG8QsjbhCJqkxXc9N0M+oRsEkn5V6qeEJZWM65F1LFY248bPFqTNyYZUBCWNtSyFZqL8nMhoZM40C2xlRHJlVby7+53VTDG/8TKgkRa7YclGYSoIxmf9NBkJzhnJqCWVa2FsJG1FNGdp0SjYEb/XlddKuVb2rav2+XmnU8jiKcAbncAkeXEMD7qAJLWAwhGd4hTdHOi/Ou/OxbC04+cwp/IHz+QMFPo2W</latexit>

<latexit sha1_base64="/SQRcrQmWtX4evaiuYQBxbwShyI=">AAAB63icbZBNSwMxEIZn61etX1WPXoJF8FR2RdBjwYvHCvYD2qVk02wbmmSXZFYopX/BiwdFvPqHvPlvzLZ70NYXAg/vzJCZN0qlsOj7315pY3Nre6e8W9nbPzg8qh6ftG2SGcZbLJGJ6UbUcik0b6FAybup4VRFkneiyV1e7zxxY0WiH3Ga8lDRkRaxYBRzq08tDqo1v+4vRNYhKKAGhZqD6ld/mLBMcY1MUmt7gZ9iOKMGBZN8XulnlqeUTeiI9xxqqrgNZ4td5+TCOUMSJ8Y9jWTh/p6YUWXtVEWuU1Ec29Vabv5X62UY34YzodMMuWbLj+JMEkxIfjgZCsMZyqkDyoxwuxI2poYydPFUXAjB6snr0L6qB44frmuN6yKOMpzBOVxCADfQgHtoQgsYjOEZXuHNU96L9+59LFtLXjFzCn/kff4AGd2OOA==</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

u1
<latexit sha1_base64="tDbhV3Fyy96Kc4x2J6knXv8JUZ0=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx4r2g9oQ9lsJ+3SzSbsboQS+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpIR14g3LFrboLkHXi5aQCOZqD8ld/GLM0QmmYoFr3PDcxfkaV4UzgrNRPNSaUTegIe5ZKGqH2s8WpM3JhlSEJY2VLGrJQf09kNNJ6GgW2M6JmrFe9ufif10tNeONnXCapQcmWi8JUEBOT+d9kyBUyI6aWUKa4vZWwMVWUGZtOyYbgrb68Ttq1qlevevdXlUYtj6MIZ3AOl+DBNTTgDprQAgYjeIZXeHOE8+K8Ox/L1oKTz5zCHzifPwRGjZM=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v1
<latexit sha1_base64="L8ixP38Ki8r76rgO5OrnmpWGUaI=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2lpoQ9lsN+3SzSbsTgol9Cd48aCIV3+RN/+N2zYHbX0w8Hhvhpl5QSKFQdf9dgobm1vbO8Xd0t7+weFR+fikbeJUM95isYx1J6CGS6F4CwVK3kk0p1Eg+VMwvp37TxOujYjVI04T7kd0qEQoGEUrPUz6Xr9ccavuAmSdeDmpQI5mv/zVG8QsjbhCJqkxXc9N0M+oRsEkn5V6qeEJZWM65F1LFY248bPFqTNyYZUBCWNtSyFZqL8nMhoZM40C2xlRHJlVby7+53VTDG/8TKgkRa7YclGYSoIxmf9NBkJzhnJqCWVa2FsJG1FNGdp0SjYEb/XlddKuVb2rav2+XmnU8jiKcAbncAkeXEMD7qAJLWAwhGd4hTdHOi/Ou/OxbC04+cwp/IHz+QMGwo2X</latexit>

v1
<latexit sha1_base64="L8ixP38Ki8r76rgO5OrnmpWGUaI=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2lpoQ9lsN+3SzSbsTgol9Cd48aCIV3+RN/+N2zYHbX0w8Hhvhpl5QSKFQdf9dgobm1vbO8Xd0t7+weFR+fikbeJUM95isYx1J6CGS6F4CwVK3kk0p1Eg+VMwvp37TxOujYjVI04T7kd0qEQoGEUrPUz6Xr9ccavuAmSdeDmpQI5mv/zVG8QsjbhCJqkxXc9N0M+oRsEkn5V6qeEJZWM65F1LFY248bPFqTNyYZUBCWNtSyFZqL8nMhoZM40C2xlRHJlVby7+53VTDG/8TKgkRa7YclGYSoIxmf9NBkJzhnJqCWVa2FsJG1FNGdp0SjYEb/XlddKuVb2rav2+XmnU8jiKcAbncAkeXEMD7qAJLWAwhGd4hTdHOi/Ou/OxbC04+cwp/IHz+QMGwo2X</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

u2
<latexit sha1_base64="ZI2tTMIsWvX+/fH4G/DAyQoXX68=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKVI8FLx4r2g9oQ9lsN+3SzSbsToQS+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbud+54lrI2L1iNOE+xEdKREKRtFKD+mgNihX3Kq7AFknXk4qkKM5KH/1hzFLI66QSWpMz3MT9DOqUTDJZ6V+anhC2YSOeM9SRSNu/Gxx6oxcWGVIwljbUkgW6u+JjEbGTKPAdkYUx2bVm4v/eb0Uwxs/EypJkSu2XBSmkmBM5n+TodCcoZxaQpkW9lbCxlRThjadkg3BW315nbRrVa9e9e6vKo1aHkcRzuAcLsGDa2jAHTShBQxG8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwAFyo2U</latexit>

<latexit sha1_base64="hp81Dr1Gw44DD0zPmEoJQSX8PGQ=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBahXkpSRD0WvHisaGuhDWWy3bRLN5uwuxFK6E/w4kERr/4ib/4bt20O2vpg4PHeDDPzgkRwbVz32ymsrW9sbhW3Szu7e/sH5cOjto5TRVmLxiJWnQA1E1yyluFGsE6iGEaBYI/B+GbmPz4xpXksH8wkYX6EQ8lDTtFY6b6K5/1yxa25c5BV4uWkAjma/fJXbxDTNGLSUIFadz03MX6GynAq2LTUSzVLkI5xyLqWSoyY9rP5qVNyZpUBCWNlSxoyV39PZBhpPYkC2xmhGellbyb+53VTE177GZdJapiki0VhKoiJyexvMuCKUSMmliBV3N5K6AgVUmPTKdkQvOWXV0m7XvMua97dRaVRz+MowgmcQhU8uIIG3EITWkBhCM/wCm+OcF6cd+dj0Vpw8plj+APn8weF041A</latexit>

(a)

v2

u5

⇤

<latexit sha1_base64="S3bKOATsaH5CNT+JhAXFYVcZvMM=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoPgKewGX8eAF48RzQOSJcxOepMhs7PLzKwQlnyCFw+KePWLvPk3TpI9aGJBQ1HVTXdXkAiujet+O4W19Y3NreJ2aWd3b/+gfHjU0nGqGDZZLGLVCahGwSU2DTcCO4lCGgUC28H4dua3n1BpHstHM0nQj+hQ8pAzaqz0kPYv++WKW3XnIKvEy0kFcjT65a/eIGZphNIwQbXuem5i/Iwqw5nAaamXakwoG9Mhdi2VNELtZ/NTp+TMKgMSxsqWNGSu/p7IaKT1JApsZ0TNSC97M/E/r5ua8MbPuExSg5ItFoWpICYms7/JgCtkRkwsoUxxeythI6ooMzadkg3BW355lbRqVe+q6t1fVOq1PI4inMApnIMH11CHO2hAExgM4Rle4c0Rzovz7nwsWgtOPnMMf+B8/gAKVo2X</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="Kf2rhJHUKT7GuN9n536PfMBQBD4=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKqMeCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0m/1i9X3Kq7AFknXk4qkKPRL3/1BjFLI66QSWpM13MT9DOqUTDJZ6VeanhC2ZgOeddSRSNu/Gxx6oxcWGVAwljbUkgW6u+JjEbGTKPAdkYUR2bVm4v/ed0Uw1s/EypJkSu2XBSmkmBM5n+TgdCcoZxaQpkW9lbCRlRThjadkg3BW315nbRqVe+66j1cVeq1PI4inME5XIIHN1CHe2hAExgM4Rle4c2Rzovz7nwsWwtOPnMKf+B8/gAHUI2V</latexit>

<latexit sha1_base64="/SQRcrQmWtX4evaiuYQBxbwShyI=">AAAB63icbZBNSwMxEIZn61etX1WPXoJF8FR2RdBjwYvHCvYD2qVk02wbmmSXZFYopX/BiwdFvPqHvPlvzLZ70NYXAg/vzJCZN0qlsOj7315pY3Nre6e8W9nbPzg8qh6ftG2SGcZbLJGJ6UbUcik0b6FAybup4VRFkneiyV1e7zxxY0WiH3Ga8lDRkRaxYBRzq08tDqo1v+4vRNYhKKAGhZqD6ld/mLBMcY1MUmt7gZ9iOKMGBZN8XulnlqeUTeiI9xxqqrgNZ4td5+TCOUMSJ8Y9jWTh/p6YUWXtVEWuU1Ec29Vabv5X62UY34YzodMMuWbLj+JMEkxIfjgZCsMZyqkDyoxwuxI2poYydPFUXAjB6snr0L6qB44frmuN6yKOMpzBOVxCADfQgHtoQgsYjOEZXuHNU96L9+59LFtLXjFzCn/kff4AGd2OOA==</latexit>

<latexit sha1_base64="k7YPU4NzUHwHdX+lfYLdeOTc0P4=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBahXkpSRD0WvHisaGuhDWWznbRLN5uwuxFK6E/w4kERr/4ib/4bt20O2vpg4PHeDDPzgkRwbVz32ymsrW9sbhW3Szu7e/sH5cOjto5TxbDFYhGrTkA1Ci6xZbgR2EkU0igQ+BiMb2b+4xMqzWP5YCYJ+hEdSh5yRo2V7qvBeb9ccWvuHGSVeDmpQI5mv/zVG8QsjVAaJqjWXc9NjJ9RZTgTOC31Uo0JZWM6xK6lkkao/Wx+6pScWWVAwljZkobM1d8TGY20nkSB7YyoGellbyb+53VTE177GZdJalCyxaIwFcTEZPY3GXCFzIiJJZQpbm8lbEQVZcamU7IheMsvr5J2veZd1ry7i0qjnsdRhBM4hSp4cAUNuIUmtIDBEJ7hFd4c4bw4787HorXg5DPH8AfO5w+HWI1B</latexit>

(b)

v2
<latexit sha1_base64="Kf2rhJHUKT7GuN9n536PfMBQBD4=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKqMeCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0m/1i9X3Kq7AFknXk4qkKPRL3/1BjFLI66QSWpM13MT9DOqUTDJZ6VeanhC2ZgOeddSRSNu/Gxx6oxcWGVAwljbUkgW6u+JjEbGTKPAdkYUR2bVm4v/ed0Uw1s/EypJkSu2XBSmkmBM5n+TgdCcoZxaQpkW9lbCRlRThjadkg3BW315nbRqVe+66j1cVeq1PI4inME5XIIHN1CHe2hAExgM4Rle4c2Rzovz7nwsWwtOPnMKf+B8/gAHUI2V</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

u3
<latexit sha1_base64="OUlPtABAaD+7HegWqF/6xMthcA8=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqqMeCF48V7Qe0oWy2k3bpZhN2N0IJ/QlePCji1V/kzX/jts1BWx8MPN6bYWZekAiujet+O4W19Y3NreJ2aWd3b/+gfHjU0nGqGDZZLGLVCahGwSU2DTcCO4lCGgUC28H4dua3n1BpHstHM0nQj+hQ8pAzaqz0kPYv+uWKW3XnIKvEy0kFcjT65a/eIGZphNIwQbXuem5i/Iwqw5nAaamXakwoG9Mhdi2VNELtZ/NTp+TMKgMSxsqWNGSu/p7IaKT1JApsZ0TNSC97M/E/r5ua8MbPuExSg5ItFoWpICYms7/JgCtkRkwsoUxxeythI6ooMzadkg3BW355lbRqVe+q6t1fVuq1PI4inMApnIMH11CHO2hAExgM4Rle4c0Rzovz7nwsWgtOPnMMf+B8/gAHTo2V</latexit>

u4
<latexit sha1_base64="ynTUYP1UQgDUsbQo5ezFWRE6vGo=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2g9oQ9lsN+3SzSbsToQS+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbud+54lrI2L1iNOE+xEdKREKRtFKD+mgPihX3Kq7AFknXk4qkKM5KH/1hzFLI66QSWpMz3MT9DOqUTDJZ6V+anhC2YSOeM9SRSNu/Gxx6oxcWGVIwljbUkgW6u+JjEbGTKPAdkYUx2bVm4v/eb0Uwxs/EypJkSu2XBSmkmBM5n+TodCcoZxaQpkW9lbCxlRThjadkg3BW315nbRrVe+q6t3XK41aHkcRzuAcLsGDa2jAHTShBQxG8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwAI0o2W</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v3
<latexit sha1_base64="BYBOu8tG50DF/FG12e+p/q4rNYQ=">AAAB6nicbVBNS8NAEJ34WetX1aOXxSJ4KkkV9Vjw4rGi/YA2lM120i7dbMLuplBCf4IXD4p49Rd589+4bXPQ1gcDj/dmmJkXJIJr47rfztr6xubWdmGnuLu3f3BYOjpu6jhVDBssFrFqB1Sj4BIbhhuB7UQhjQKBrWB0N/NbY1Sax/LJTBL0IzqQPOSMGis9jnuXvVLZrbhzkFXi5aQMOeq90le3H7M0QmmYoFp3PDcxfkaV4UzgtNhNNSaUjegAO5ZKGqH2s/mpU3JulT4JY2VLGjJXf09kNNJ6EgW2M6JmqJe9mfif10lNeOtnXCapQckWi8JUEBOT2d+kzxUyIyaWUKa4vZWwIVWUGZtO0YbgLb+8SprVindd8R6uyrVqHkcBTuEMLsCDG6jBPdShAQwG8Ayv8OYI58V5dz4WrWtOPnMCf+B8/gAI1I2W</latexit>

⇤
<latexit sha1_base64="/SQRcrQmWtX4evaiuYQBxbwShyI=">AAAB63icbZBNSwMxEIZn61etX1WPXoJF8FR2RdBjwYvHCvYD2qVk02wbmmSXZFYopX/BiwdFvPqHvPlvzLZ70NYXAg/vzJCZN0qlsOj7315pY3Nre6e8W9nbPzg8qh6ftG2SGcZbLJGJ6UbUcik0b6FAybup4VRFkneiyV1e7zxxY0WiH3Ga8lDRkRaxYBRzq08tDqo1v+4vRNYhKKAGhZqD6ld/mLBMcY1MUmt7gZ9iOKMGBZN8XulnlqeUTeiI9xxqqrgNZ4td5+TCOUMSJ8Y9jWTh/p6YUWXtVEWuU1Ec29Vabv5X62UY34YzodMMuWbLj+JMEkxIfjgZCsMZyqkDyoxwuxI2poYydPFUXAjB6snr0L6qB44frmuN6yKOMpzBOVxCADfQgHtoQgsYjOEZXuHNU96L9+59LFtLXjFzCn/kff4AGd2OOA==</latexit>

v4
<latexit sha1_base64="eDDFvQZK4pqVd67GDn2xr8XnO38=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2lpoQ9lsN+3SzSbsTgol9Cd48aCIV3+RN/+N2zYHbX0w8Hhvhpl5QSKFQdf9dgobm1vbO8Xd0t7+weFR+fikbeJUM95isYx1J6CGS6F4CwVK3kk0p1Eg+VMwvp37TxOujYjVI04T7kd0qEQoGEUrPUz69X654lbdBcg68XJSgRzNfvmrN4hZGnGFTFJjup6boJ9RjYJJPiv1UsMTysZ0yLuWKhpx42eLU2fkwioDEsbalkKyUH9PZDQyZhoFtjOiODKr3lz8z+umGN74mVBJilyx5aIwlQRjMv+bDITmDOXUEsq0sLcSNqKaMrTplGwI3urL66Rdq3pXVe++XmnU8jiKcAbncAkeXEMD7qAJLWAwhGd4hTdHOi/Ou/OxbC04+cwp/IHz+QMKWI2X</latexit>

Figure 3: The dotted lines edges indicate the firstextent paths for the positions in Posα . (a) We
have δ ⊙ (v2 , α) = δ ⊙ (v1 , α) and they are contained in but not equal to δ ⊙ (v0 , α). Both δ ⊙ (u3 , α)
and δ ⊙ (u5 , α) = δ ⊙ (u1 , α) are contained in δ ⊙ (u0 , α). Whereas, δ ⊙ (u2 , α) ̸⊆ δ ⊙ (u0 , α). Note, that
we cannot dismiss e.g. δ ⊙ (u3 , α) at preprocessing time, since left(u0 ) might not be in lastextent(P ).
(b) The grey nodes are lcas of a position in P and a position in Posα . The set δ ∗ (v4 , α) is not
included in any of the others. Nodes v2 and v1 has the same ∗-parent and thus δ ∗ (v3 , α) ⊆ δ ∗ (v2 , α).
Since Posα ∩ Pos(v2 ) = Posα ∩ Pos(v3 ) in the example then δ ∗ (v3 , α) = δ ∗ (v2 , α). We also have
δ ∗ (v2 , α) ⊂ δ ∗ (v1 , α) = δ ∗ (v0 , α). We cannot dismiss any of the nodes at preprocessing time as might
be the case that only a subset (or none) of them is in lastextent(P ).
The internal transitions on the set of transition nodes are not disjoint and hence we cannot afford
to compute internal transitions on each of the transition nodes explicitly. Fortunately, by Lemma 4,
the internal ⊙-transitions (resp. ∗-transitions) of the nodes from N ⊙ (P, α) (resp. N ∗ (P, α)) form a
laminar family. We use this to divide the computations of the internal transitions into computations
on a bounded number of non-overlapping intervals. We implement this idea by compactly encoding
all relevant transition nodes in the transition tree defined in the following.

4.3

Transition Trees

Let P be a set of positions. Given a state-set transition δ(P, α), we define the transition tree T as
the subtree of R induced by all nodes in P and their ancestors (see Figure 4(a)). A segment in T is
a path from a leaf or a branching node to (but not including) the nearest branching node above it
(or to the root if no such branching node exists). The root node is its own segment (see Figure 4(b)).
The bottom node of a segment s is denoted bot(s). Note that bot(s) is always a branching node, a
leaf, or the root. Any branching node in T is the lowest common ancestor of two nodes in P and
vice versa. Hence, we can compactly store T in O(|P |) space by storing P and the branching nodes
with pointers into R.
The following observation follows immediately from the fact that all nodes in lastextent(P ) and
their ancestors are contained in T .
Observation 8. Let T be the transition tree for P in R. If v is a transition node for δ(P, α) then
v is a node on a segment in T .
Thus it is enough to consider the nodes in the transition tree when computing the set of transition
nodes.
Observation 9. Any ⊙-node u that is an internal node on a segment and where left(u) ∈
lastextent(P ) must have its left child on the same segment.
10

<!-- page 12 -->

(a)

(b)

<latexit sha1_base64="RaKubww8bHA7bv0Y71M1/0w+EPc=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tFqJeSFEGPBS8eK9oPaEPZbCft0s0m7G6EEvoTvHhQxKu/yJv/xm2bg7a+sPDwzgw78waJ4Nq47rdT2Njc2t4p7pb29g8Oj8rHJ20dp4phi8UiVt2AahRcYstwI7CbKKRRILATTG7n9c4TKs1j+WimCfoRHUkeckaNtR6q9HJQrrg1dyGyDl4OFcjVHJS/+sOYpRFKwwTVuue5ifEzqgxnAmelfqoxoWxCR9izKGmE2s8Wq87IhXWGJIyVfdKQhft7IqOR1tMosJ0RNWO9Wpub/9V6qQlv/IzLJDUo2fKjMBXExGR+NxlyhcyIqQXKFLe7EjamijJj0ynZELzVk9ehXa95lu/rlcZVHkcRzuAcquDBNTTgDprQAgYjeIZXeHOE8+K8Ox/L1oKTz5zCHzmfP4PXjTo=</latexit>

<latexit sha1_base64="/IZxkdWWXhp/Wt2hgTFqXWy1ERc=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tFqJeSFEGPBS8eK9oPaEPZbCft0s0m7G6EEvoTvHhQxKu/yJv/xm2bg7a+sPDwzgw78waJ4Nq47rdT2Njc2t4p7pb29g8Oj8rHJ20dp4phi8UiVt2AahRcYstwI7CbKKRRILATTG7n9c4TKs1j+WimCfoRHUkeckaNtR6qweWgXHFr7kJkHbwcKpCrOSh/9YcxSyOUhgmqdc9zE+NnVBnOBM5K/VRjQtmEjrBnUdIItZ8tVp2RC+sMSRgr+6QhC/f3REYjradRYDsjasZ6tTY3/6v1UhPe+BmXSWpQsuVHYSqIicn8bjLkCpkRUwuUKW53JWxMFWXGplOyIXirJ69Du17zLN/XK42rPI4inME5VMGDa2jAHTShBQxG8Ayv8OYI58V5dz6WrQUnnzmFP3I+fwCFXI07</latexit>

Figure 4: (a) The parse tree R. The blue leaves are the positions in P . (b) The transition tree T of
P . Branching nodes are grey and internal nodes on segments are white.
To see why, observe that since left(u) ∈ lastextent(P ), then left(u) is an ancestor of a node in P ,
and thus by definition of T , left(u) belongs to T . Since u is an internal node on a segment s, only
one of its children belongs to T . Therefore, left(u) must also belong to segment s.
We will show in Section 5.2 that the transition tree T and the key information we need in our
algorithm can be computed in O(|P |) time.

5

Sparse State-Set Transitions

We now present our O(m) space data structure that supports computing a state-set transition δ(P, α)
in O(|P | log log m + |δ(|P |, α)|) time. We describe the data structure and analyze the space and
preprocessing time in Section 5.1. The high-level idea of our sparse state-set algorithm is to identify
the transition nodes for δ ⊙ (P, α) and δ ∗ (P, α) using the transition tree. The state-set transitions for
each set of nodes induce a partition of Posα into nested intervals. We partition these intervals into
non-overlapping intervals and then compute the internal transitions for each new interval. We then
compute the union of these, which by the decomposition in Lemma 6, is precisely the set δ(P, α).
We describe how to construct the transition tree in Section 5.2. In Section 5.3 we describe how to
find the set transition nodes and how to construct the intervals. In Section 5.4 we we show how to
compute the internal transitions efficiently. Finally, in Section 5.5 we put everything together to get
an algorithm for computing a state-set transition δ(P, α) in O(|P | log log m + |δ(P, α)|) time.

5.1

Data Structure

We store the regular expression with labels on leaves and internal nodes together with the following
components.
• For each node v in R, we store the range of positions that are descendants of v, the depth of
v, the highest node in lastextent(v), and the highest node in firstextent(v). We also store a
pointer parent⊙ (v) to the lowest ancestor u of v such that u is an ⊙-node and v ∈ T (left(u)),
and a pointer parent∗ (v) to its lowest star ancestor.
• At each branching node v we store the position of the rightmost leaf in left(v) and the position
of the leftmost leaf in right(v).
• Data structures for R that supports lowest common ancestor queries and first label queries.
Given a node v and a character α, a first label query, denoted firstlabel(v, α), returns the
lowest ancestor of v whose label contains α.
Furthermore, we store the following information for each α ∈ Σ.

11

<!-- page 13 -->

• Arrays Aα and Dα , where Aα [i] is the ith position labeled α in the left-to-right ordering of
Posα , and Dα [i] is the depth of the highest node in firstextent(Aα [i]).
• A data structure supporting predecessor and successor queries on the positions in Posα . That
is, given any position p in R the predecessor (successor) query returns the position in Aα
of the nearest position labeled α to the left (right) of p including p itself. For a branching
node v ∈ R, we define the successor of v in Posα as the successor in Posα of the leftmost leaf
in right(v). Note that this corresponds to the first position labeled α after v in the order
obtained by an inorder traversal of the nodes in R. Similarly, we define the predecessor of v
in Posα as the predecessor in Posα of the rightmost leaf in left(v).
• A data structure on Dα that supports range minimum queries. Given any pair of indices l
and r, the range minimum query on Dα returns a minimum value in the subarray Dα [l, r].
• For each node v containing label α:
– A pointer next⊙ (v, α) to the lowest proper ancestor u of v such that v ∈ T (left(u)),
α ∈ label(u), and δ ⊙ (u, α) is non-empty. If no such u exists we store a null pointer to
indicate this.
– A pointer next∗ (v, α) to the lowest proper ancestor u of v labeled α such that there exists
a q ∈ δ ∗ (u, α) where q ∈ T (right(u)) if v ∈ T (left(u)) and q ∈ T (left(u)) if v ∈ T (right(u)).
If no such u exists we store a null pointer to indicate this.
– The range of positions in Aα that are descendants of v, of left(v), and of right(v),
respectively.
The idea of the next⊙ -pointers is that they form a chain of prospective nodes for N ⊙ with label
α. Any node u from N ⊙ (P, α) with label α on a segment s has its left child on the path and
δ ⊙ (u, α) ̸= ∅, so it is included in this chain. We show that at most one node can be from N ⊙ (P, α)
on each segment that does not have label α. Furthermore, given a segment s, the nodes from the
chain on s that belong to N ⊙ (P, α) form a subchain starting from the lowest node of the chain that
is on s to (not including) the first node in the chain that is either not in lastextent(P ) or not on s.
Similarly, the next∗ -pointers form a chain of prospective nodes for N ∗ with label α.
Space The regular expression and the labels use O(m) space. The arrays Aα and Dα , α ∈ Σ,
use O(m) space in total. We use linear space and linear preprocessing time data structures to
support lowest common ancestors in constant time [9,38], first label queries in O(log log m) time [25],
predecessor queries in O(log log m) time [58, 75], and range minimum queries in constant time [9, 38].
For each alphabet character, the total size of these data structures is linear in the number of leaves
labeled with that character. Thus in total the space for these data structures is linear in m. The
cited data structures for predecessor queries both use randomization, but since we only need a static
structure it is straightforward and well-known how to obtain the same bound deterministically by
combining deterministic dictionaries [37] with a simple two-level approach (see, e.g., Thorup [71]).
We store at most two next pointers for each label in R and a single pointer for each position using
O(m) space. The remaining information uses O(m) space.
Preprocessing We compute the range of positions that are descendants of v, the depth of v, the
parent⊙ (v) and parent∗ (v) pointers, and the positions of left(v) and right(v) using tree traversal in
linear time. To compute the highest node for each node in R, we first compute for each node u ∈ R
if ϵ ∈ R(u) using a linear time bottom-up tree traversal. In top-down traversal we then compute the
12

<!-- page 14 -->

highest node HL (v) in lastextent(v) for each node v ∈ R using the following rules: If v is the left
child of an ⊙-node and ϵ ̸∈ R(right(v)) then HL (v) = v. Otherwise, HL (v) = HL (parent(v)). We
compute the highest node in firstextent(v) similarly. We construct the arrays Aα and Dα , for all
α ∈ Σ in a single tree traversal.
To compute the remaining information we do the following for each α ∈ Σ. Construct a tree Rα
containing all nodes with label α. To do this, we use lca queries on each consecutive pair of leaves
in Aα from left to right. By keeping track of the depths of the nodes and checking if the newest
node is an ancestor of the previous node it is straightforward to implement this in linear time. To
compute the next-pointers we do a top-down traversal of Rα . In each node u we check if δ ⊙ (u, α)
and δ ∗ (u, α) are empty using range minimum queries on Dα . To check if δ ⊙ (u, α) we do the range
minimum query on the range of positions in Posα in right(v). If the depth returned is less than or
equal to depth(right(v)) then δ ⊙ (u, α) is non-empty. For δ ∗ (u, α) we do the query separately on the
intervals corresponding to the left and the right child and we compare with the depth of parent∗ (v).
With this information, we can compute the next-pointers during the traversal of Rα in constant time
per node. The total size of all the Rα trees O(m), since each tree has size 2|Posα | − 1. Thus the
total time used for each α is O(|Posα |). Hence, it follows that the total preprocessing time is O(m).

5.2

Constructing the Transition Tree

We say that a node v is a ⊙-live node if left(v) ∈ lastextent(P ) and v is a ⊙-node. Note that any
node in N ⊙ (P, α) is a ⊙-live node and a node in the transition tree T . It follows from Observation 9
that any ⊙-live node that is an internal node on a segment has its left child on the segment. A
segment s in T is called a ∗-segment if bot(s) is not the root and parent∗ (bot(s)) ∈ lastextent(P ).
We compute the compact representation of the transition tree as follows. Let P be the set of
leaves and repeatedly take the lca of adjacent nodes to form the internal nodes and the segments of
T . Using a tree traversal on the compact transition tree we also compute for all branching nodes v
in T the depth of the highest node in lastextent(P ∩ Pos(v)), all ⊙-live branching nodes in T , and
all ∗-segments of T . Hence, we have the following result.
Lemma 10. In O(|P |) time we can compute the transition tree T of P , for all branching nodes v
in T the depth of the highest node in lastextent(P ∩ Pos(v)), all ⊙-live branching nodes in T , and
all ∗-segments of T .
Observation 11. We can check in constant time if a node v ∈ T is in lastextent(P ) given the
segment it is on.
If v is a leaf or a branching node, we already computed the information. Otherwise, v is an
internal node on a segment s. Then we compare the depth of v with the depth d of the highest node
in lastextent(P ∩ Pos(bot(s))). Now v is in lastextent(P ) if and only if the depth of v is at least d.

5.3

Computing Transitions Nodes and Intervals

We construct two sets of nodes M ⊙ and M ∗ that consists of all nodes in N ⊙ (P, α) and N ∗ (P, α),
respectively, together with a constant number of other nodes per segment. We compute these sets
for each segment using a depth-first traversal of the transition tree. We also construct sets L⊙ and
L∗ , that partition Posα into intervals in order to avoid recomputing overlapping internal transitions.
We associate each interval with the lowest node from M ⊙ (respectively M ∗ ) that can contain the
positions in its internal transition (see Figure 5).

13

<!-- page 15 -->

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v1

v4

<latexit sha1_base64="kYtegLoOv6kyn8BzHHxCf5HoLCk=">AAAB7XicbVBNSwMxEJ2tX7V+VT16CRbBg5RdaVFvBS8eK9gP2C4lm2bb2GyyJFmhLP0PXjwo4tX/481/Y9ruQVsfDDzem2FmXphwpo3rfjuFtfWNza3idmlnd2//oHx41NYyVYS2iORSdUOsKWeCtgwznHYTRXEcctoJx7czv/NElWZSPJhJQoMYDwWLGMHGSm2/fuHdBP1yxa26c6BV4uWkAjma/fJXbyBJGlNhCMda+56bmCDDyjDC6bTUSzVNMBnjIfUtFTimOsjm107RmVUGKJLKljBorv6eyHCs9SQObWeMzUgvezPxP89PTXQdZEwkqaGCLBZFKUdGotnraMAUJYZPLMFEMXsrIiOsMDE2oJINwVt+eZW0L6tevere1yoNN4+jCCdwCufgwRU04A6a0AICj/AMr/DmSOfFeXc+Fq0FJ585hj9wPn8AMzyOKw==</latexit>

[5, 19]

<latexit sha1_base64="4jJRhQri4Qs0CWbrp7OwtGEEc8s=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHivYD2lA22027dLMJu5NCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNECoOu++0UtrZ3dveK+6WDw6Pjk/LpWdvEqWa8xWIZ625ADZdC8RYKlLybaE6jQPJOMLlb1DtTro2I1RPOEu5HdKREKBhFaz1OB96gXHGr7lJkE7wcKpCrOSh/9YcxSyOukElqTM9zE/QzqlEwyeelfmp4QtmEjnjPoqIRN362XHVOrqwzJGGs7VNIlu7viYxGxsyiwHZGFMdmvbYw/6v1Ugxv/UyoJEWu2OqjMJUEY7K4mwyF5gzlzAJlWthdCRtTTRnadEo2BG/95E1o31Q9yw+1SqOWx1GEC7iEa/CgDg24hya0gMEInuEV3hzpvDjvzseqteDkM+fwR87nDwRwjZA=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v2

<latexit sha1_base64="fVSNK6X7bbZozfYXSG7NmvInUr4=">AAAB7nicbVBNS8NAEJ34WetX1aOXxSJ4kJKUij0WvHisYD+gDWWznbRLN5uwuxFK6I/w4kERr/4eb/4bt20O2vpg4PHeDDPzgkRwbVz329nY3Nre2S3sFfcPDo+OSyenbR2nimGLxSJW3YBqFFxiy3AjsJsopFEgsBNM7uZ+5wmV5rF8NNME/YiOJA85o8ZKnV61fl2r+4NS2a24C5B14uWkDDmag9JXfxizNEJpmKBa9zw3MX5GleFM4KzYTzUmlE3oCHuWShqh9rPFuTNyaZUhCWNlSxqyUH9PZDTSehoFtjOiZqxXvbn4n9dLTVj3My6T1KBky0VhKoiJyfx3MuQKmRFTSyhT3N5K2JgqyoxNqGhD8FZfXiftasW7qbgPtXLDzeMowDlcwBV4cAsNuIcmtIDBBJ7hFd6cxHlx3p2PZeuGk8+cwR84nz+s7I5s</latexit>

[28, 48]

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="E5MD5XnKqUKA3DFEtVnKahAFYqU=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKdRjwYvHivYD2lA220m7dLMJu5tCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs7aOU8WwxWIRq25ANQousWW4EdhNFNIoENgJJneLemeKSvNYPplZgn5ER5KHnFFjrcfpoDYoV9yquxTZBC+HCuRqDspf/WHM0gilYYJq3fPcxPgZVYYzgfNSP9WYUDahI+xZlDRC7WfLVefkyjpDEsbKPmnI0v09kdFI61kU2M6ImrFery3M/2q91IS3fsZlkhqUbPVRmApiYrK4mwy5QmbEzAJlittdCRtTRZmx6ZRsCN76yZvQvql6lh9qlUYtj6MIF3AJ1+BBHRpwD01oAYMRPMMrvDnCeXHenY9Va8HJZ87hj5zPHwj8jZM=</latexit>

<latexit sha1_base64="Cl48njpcJudtOcbhaeAcyFdknZ8=">AAAB7nicbVBNS8NAEJ3Ur1q/qh69LBbBg5RELOqt4MVjBfsBbSib7aZdutmE3YlQQn+EFw+KePX3ePPfuG1z0NYHA4/3ZpiZFyRSGHTdb6ewtr6xuVXcLu3s7u0flA+PWiZONeNNFstYdwJquBSKN1Gg5J1EcxoFkreD8d3Mbz9xbUSsHnGScD+iQyVCwShaqd31ahferd8vV9yqOwdZJV5OKpCj0S9/9QYxSyOukElqTNdzE/QzqlEwyaelXmp4QtmYDnnXUkUjbvxsfu6UnFllQMJY21JI5urviYxGxkyiwHZGFEdm2ZuJ/3ndFMMbPxMqSZErtlgUppJgTGa/k4HQnKGcWEKZFvZWwkZUU4Y2oZINwVt+eZW0Lqtereo+XFXqbh5HEU7gFM7Bg2uowz00oAkMxvAMr/DmJM6L8+58LFoLTj5zDH/gfP4Ao76OZg==</latexit>

[15, 19]

<latexit sha1_base64="Y8iuL0s/S6j+GFGp7O/VRtSriAQ=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FSSUtBjwYvHivYD2lA220m7dLMJu5tCCf0JXjwo4tVf5M1/47bNQVtfWHh4Z4adeYNEcG1c99spbG3v7O4V90sHh0fHJ+XTs7aOU8WwxWIRq25ANQousWW4EdhNFNIoENgJJneLemeKSvNYPplZgn5ER5KHnFFjrcfpoDYoV9yquxTZBC+HCuRqDspf/WHM0gilYYJq3fPcxPgZVYYzgfNSP9WYUDahI+xZlDRC7WfLVefkyjpDEsbKPmnI0v09kdFI61kU2M6ImrFery3M/2q91IS3fsZlkhqUbPVRmApiYrK4mwy5QmbEzAJlittdCRtTRZmx6ZRsCN76yZvQrlU9yw/1SqOex1GEC7iEa/DgBhpwD01oAYMRPMMrvDnCeXHenY9Va8HJZ87hj5zPHwX0jZE=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v5

[32, 36] v6
<latexit sha1_base64="p1hvN2ompkbu69q3n7HnuXkGtwo=">AAAB7nicbVDLSgNBEOyNrxhfUY9eBoPgQcJu4usY8OIxgnnAZgmzk9lkyOzsMDMrhCUf4cWDIl79Hm/+jZNkD5pY0FBUddPdFUrOtHHdb6ewtr6xuVXcLu3s7u0flA+P2jpJFaEtkvBEdUOsKWeCtgwznHalojgOOe2E47uZ33miSrNEPJqJpEGMh4JFjGBjpY5fr13Ur4N+ueJW3TnQKvFyUoEczX75qzdISBpTYQjHWvueK02QYWUY4XRa6qWaSkzGeEh9SwWOqQ6y+blTdGaVAYoSZUsYNFd/T2Q41noSh7Yzxmakl72Z+J/npya6DTImZGqoIItFUcqRSdDsdzRgihLDJ5Zgopi9FZERVpgYm1DJhuAtv7xK2rWqd1V1Hy4rDTePowgncArn4MENNOAemtACAmN4hld4c6Tz4rw7H4vWgpPPHMMfOJ8/oLWOZA==</latexit>

<latexit sha1_base64="VQxAmSG0x76mrnVl0gLd2vw7WU4=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSqeix4MVjRfsBbSib7aZdutmE3UmhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEihUHX/XYKG5tb2zvF3dLe/sHhUfn4pGXiVDPeZLGMdSeghkuheBMFSt5JNKdRIHk7GN/N6+0J10bE6gmnCfcjOlQiFIyitR4n/et+ueJW3YXIOng5VCBXo1/+6g1ilkZcIZPUmK7nJuhnVKNgks9KvdTwhLIxHfKuRUUjbvxsseqMXFhnQMJY26eQLNzfExmNjJlGge2MKI7Mam1u/lfrphje+plQSYpcseVHYSoJxmR+NxkIzRnKqQXKtLC7EjaimjK06ZRsCN7qyevQuqp6lh9qlXotj6MIZ3AOl+DBDdThHhrQBAZDeIZXeHOk8+K8Ox/L1oKTz5zCHzmfPwqAjZQ=</latexit>

v3

<latexit sha1_base64="eR90vyN/L9w6g//pq+Gb3FxF888=">AAAB7XicbVBNSwMxEJ2tX7V+VT16CRbBg5SsKPZY8OKxgv2AdinZNNvGZpMlyQpl6X/w4kERr/4fb/4b03YP2vpg4PHeDDPzwkRwYzH+9gpr6xubW8Xt0s7u3v5B+fCoZVSqKWtSJZTuhMQwwSVrWm4F6ySakTgUrB2Ob2d++4lpw5V8sJOEBTEZSh5xSqyTWt3ahY+DfrmCq3gOtEr8nFQgR6Nf/uoNFE1jJi0VxJiujxMbZERbTgWblnqpYQmhYzJkXUcliZkJsvm1U3TmlAGKlHYlLZqrvycyEhsziUPXGRM7MsveTPzP66Y2qgUZl0lqmaSLRVEqkFVo9joacM2oFRNHCNXc3YroiGhCrQuo5ELwl19eJa3Lqn9dxfdXlTrO4yjCCZzCOfhwA3W4gwY0gcIjPMMrvHnKe/HevY9Fa8HLZ47hD7zPHyonjiU=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

v1 v3
<latexit sha1_base64="fJgyKYHYkEUVE/cOyzVHt8IVH4I=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0nEoseCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0nf65crbtVdgKwTLycVyNHol796g5ilEVfIJDWm67kJ+hnVKJjks1IvNTyhbEyHvGupohE3frY4dUYurDIgYaxtKSQL9fdERiNjplFgOyOKI7PqzcX/vG6K4a2fCZWkyBVbLgpTSTAm87/JQGjOUE4toUwLeythI6opQ5tOyYbgrb68TlpXVa9WdR+uK3U3j6MIZ3AOl+DBDdThHhrQBAZDeIZXeHOk8+K8Ox/L1oKTz5zCHzifPwSMjZA=</latexit>

<latexit sha1_base64="/OLkXieDCvHsLVK3PwJnIpvYLuk=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBiyURRY8FLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777RTW1jc2t4rbpZ3dvf2D8uFRS8epYthksYhVJ6AaBZfYNNwI7CQKaRQIbAfju5nffkKleSwfzCRBP6JDyUPOqLFS46JfrrhVdw6ySrycVCBHvV/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/YzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJ67LqXVfdxlWl5uZxFOEETuEcPLiBGtxDHZrAAOEZXuHNeXRenHfnY9FacPKZY/gD5/MHcHWMow==</latexit>

0

<latexit sha1_base64="ifS8Ptc8pK8xTihOhc/J0SzvBlk=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0nEoseCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipWRuUK27VXYCsEy8nFcjRGJS/+sOYpRFKwwTVuue5ifEzqgxnAmelfqoxoWxCR9izVNIItZ8tDp2RC6sMSRgrW9KQhfp7IqOR1tMosJ0RNWO96s3F/7xeasJbP+MySQ1KtlwUpoKYmMy/JkOukBkxtYQyxe2thI2poszYbEo2BG/15XXSvqp6tarbvK7U3TyOIpzBOVyCBzdQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AfJWMqw==</latexit>

5

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

[8, 10]

<latexit sha1_base64="NgrFx/JBikjR0a7O+X3fJ8RkpEU=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSLeix4MVjRfsBbSib7aRdutmE3U2hhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEiuDau++0UNja3tneKu6W9/YPDo/LxSUvHqWLYZLGIVSegGgWX2DTcCOwkCmkUCGwH47t5vT1BpXksn8w0QT+iQ8lDzqix1uOkf90vV9yquxBZBy+HCuRq9MtfvUHM0gilYYJq3fXcxPgZVYYzgbNSL9WYUDamQ+xalDRC7WeLVWfkwjoDEsbKPmnIwv09kdFI62kU2M6ImpFerc3N/2rd1IS3fsZlkhqUbPlRmApiYjK/mwy4QmbE1AJlittdCRtRRZmx6ZRsCN7qyevQuqp6lh9qlXotj6MIZ3AOl+DBDdThHhrQBAZDeIZXeHOE8+K8Ox/L1oKTz5zCHzmfPwd4jZI=</latexit>

<latexit sha1_base64="6E7OG4+BkgXIUg42LIRDEoeKcYE=">AAAB6HicbVDLSgNBEOz1GeMr6tHLYBA8hVlR9Bjw4jEB84BkCbOT3mTM7OwyMyuEkC/w4kERr36SN//GSbIHTSxoKKq66e4KUymMpfTbW1vf2NzaLuwUd/f2Dw5LR8dNk2SaY4MnMtHtkBmUQmHDCiuxnWpkcSixFY7uZn7rCbURiXqw4xSDmA2UiARn1kl12iuVaYXOQVaJn5My5Kj1Sl/dfsKzGJXlkhnT8WlqgwnTVnCJ02I3M5gyPmID7DiqWIwmmMwPnZJzp/RJlGhXypK5+ntiwmJjxnHoOmNmh2bZm4n/eZ3MRrfBRKg0s6j4YlGUSWITMvua9IVGbuXYEca1cLcSPmSaceuyKboQ/OWXV0nzsuJfV2j9qlyleRwFOIUzuAAfbqAK91CDBnBAeIZXePMevRfv3ftYtK55+cwJ/IH3+QN1AYym</latexit>

<latexit sha1_base64="rw/RaFMSTlHn1uKns/Ez6JitbcQ=">AAAB7nicbVBNS8NAEJ3Ur1q/qh69LBbBg5REKvZY8OKxgv2ANJTNdtsu3WzC7kQooT/CiwdFvPp7vPlv3LY5aOuDgcd7M8zMCxMpDLrut1PY2Nza3inulvb2Dw6PyscnbROnmvEWi2WsuyE1XArFWyhQ8m6iOY1CyTvh5G7ud564NiJWjzhNeBDRkRJDwShaqePXvKtaPeiXK27VXYCsEy8nFcjR7Je/eoOYpRFXyCQ1xvfcBIOMahRM8lmplxqeUDahI+5bqmjETZAtzp2RC6sMyDDWthSShfp7IqORMdMotJ0RxbFZ9ebif56f4rAeZEIlKXLFlouGqSQYk/nvZCA0ZyinllCmhb2VsDHVlKFNqGRD8FZfXift66p3U3UfapWGm8dRhDM4h0vw4BYacA9NaAGDCTzDK7w5ifPivDsfy9aCk8+cwh84nz+lRo5n</latexit>

[41, 48]

<latexit sha1_base64="9lRhXHumU8cd64o+SoUeHPVUhAA=">AAAB6nicbZBNS8NAEIYn9avWr6pHL4tF8FQSKeqx4MVjRfsBbSib7aZdutmE3UmhhP4ELx4U8eov8ua/cdvmoK0vLDy8M8POvEEihUHX/XYKG5tb2zvF3dLe/sHhUfn4pGXiVDPeZLGMdSeghkuheBMFSt5JNKdRIHk7GN/N6+0J10bE6gmnCfcjOlQiFIyitR4n/et+ueJW3YXIOng5VCBXo1/+6g1ilkZcIZPUmK7nJuhnVKNgks9KvdTwhLIxHfKuRUUjbvxsseqMXFhnQMJY26eQLNzfExmNjJlGge2MKI7Mam1u/lfrphje+plQSYpcseVHYSoJxmR+NxkIzRnKqQXKtLC7EjaimjK06ZRsCN7qyevQuqp6lh9qlXotj6MIZ3AOl+DBDdThHhrQBAZDeIZXeHOk8+K8Ox/L1oKTz5zCHzmfPwwEjZU=</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="0l1n5VrVk61VuMAe/V80nJ1HBTc=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoPgKez6QI8BLx4jmgckS5idzCZDZmeXmd5AWPIJXjwo4tUv8ubfOEn2oIkFDUVVN91dQSKFQdf9dgpr6xubW8Xt0s7u3v5B+fCoaeJUM95gsYx1O6CGS6F4AwVK3k40p1EgeSsY3c381phrI2L1hJOE+xEdKBEKRtFKj+PeZa9ccavuHGSVeDmpQI56r/zV7ccsjbhCJqkxHc9N0M+oRsEkn5a6qeEJZSM64B1LFY248bP5qVNyZpU+CWNtSyGZq78nMhoZM4kC2xlRHJplbyb+53VSDG/9TKgkRa7YYlGYSoIxmf1N+kJzhnJiCWVa2FsJG1JNGdp0SjYEb/nlVdK8qHrXVffhqlJz8ziKcAKncA4e3EAN7qEODWAwgGd4hTdHOi/Ou/OxaC04+cwx/IHz+QMHlI2S</latexit>

<latexit sha1_base64="HC6/M8Gv8YJ4xegsSB75y1mnlRI=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0lEsceCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipWRuUK27VXYCsEy8nFcjRGJS/+sOYpRFKwwTVuue5ifEzqgxnAmelfqoxoWxCR9izVNIItZ8tDp2RC6sMSRgrW9KQhfp7IqOR1tMosJ0RNWO96s3F/7xeasKan3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jppX1W9m6rbvK7U3TyOIpzBOVyCB7dQh3toQAsYIDzDK7w5j86L8+58LFsLTj5zCn/gfP4AgSGMrg==</latexit>

v1

v2

<latexit sha1_base64="fJgyKYHYkEUVE/cOyzVHt8IVH4I=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0nEoseCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0nf65crbtVdgKwTLycVyNHol796g5ilEVfIJDWm67kJ+hnVKJjks1IvNTyhbEyHvGupohE3frY4dUYurDIgYaxtKSQL9fdERiNjplFgOyOKI7PqzcX/vG6K4a2fCZWkyBVbLgpTSTAm87/JQGjOUE4toUwLeythI6opQ5tOyYbgrb68TlpXVa9WdR+uK3U3j6MIZ3AOl+DBDdThHhrQBAZDeIZXeHOk8+K8Ox/L1oKTz5zCHzifPwSMjZA=</latexit>

8 10
<latexit sha1_base64="4qg7aWXFEbKcp5w3L1y/jj/emk8=">AAAB6XicbVDLSgNBEOyNrxhfUY9eBoPgKcyKoseAF49RzAOSJcxOepMhs7PLzKwQQv7AiwdFvPpH3vwbJ8keNLGgoajqprsrTKUwltJvr7C2vrG5Vdwu7ezu7R+UD4+aJsk0xwZPZKLbITMohcKGFVZiO9XI4lBiKxzdzvzWE2ojEvVoxykGMRsoEQnOrJMefNorV2iVzkFWiZ+TCuSo98pf3X7CsxiV5ZIZ0/FpaoMJ01ZwidNSNzOYMj5iA+w4qliMJpjML52SM6f0SZRoV8qSufp7YsJiY8Zx6DpjZodm2ZuJ/3mdzEY3wUSoNLOo+GJRlEliEzJ7m/SFRm7l2BHGtXC3Ej5kmnHrwim5EPzll1dJ86LqX1Xp/WWlRvM4inACp3AOPlxDDe6gDg3gEMEzvMKbN/JevHfvY9Fa8PKZY/gD7/MH5FqM4Q==</latexit>

15

v5

v4

<latexit sha1_base64="bkPNraSgOq4AixIFPjtYSiasjaM=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKoseCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0m/1i9X3Kq7AFknXk4qkKPRL3/1BjFLI66QSWpM13MT9DOqUTDJZ6VeanhC2ZgOeddSRSNu/Gxx6oxcWGVAwljbUkgW6u+JjEbGTKPAdkYUR2bVm4v/ed0Uw1s/EypJkSu2XBSmkmBM5n+TgdCcoZxaQpkW9lbCRlRThjadkg3BW315nbRqVe+66j5cVepuHkcRzuAcLsGDG6jDPTSgCQyG8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwAGEI2R</latexit>

<latexit sha1_base64="y8IYq3CzHa9f2pIuJTJGlNNPs7I=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0nEUo8FLx6r2A9oQ9lsJ+3SzSbsboQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpwasNyhW36i5A1omXkwrkaA7KX/1hzNIIpWGCat3z3MT4GVWGM4GzUj/VmFA2oSPsWSpphNrPFpfOyIVVhiSMlS1pyEL9PZHRSOtpFNjOiJqxXvXm4n9eLzXhjZ9xmaQGJVsuClNBTEzmb5MhV8iMmFpCmeL2VsLGVFFmbDglG4K3+vI6aV9VvVrVvb+uNNw8jiKcwTlcggd1aMAdNKEFDEJ4hld4cybOi/PufCxbC04+cwp/4Hz+AOvujOY=</latexit>

<latexit sha1_base64="oab+wWB+yH0NnADlzCIUaP/3n5g=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mkoseCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0m/1i9X3Kq7AFknXk4qkKPRL3/1BjFLI66QSWpM13MT9DOqUTDJZ6VeanhC2ZgOeddSRSNu/Gxx6oxcWGVAwljbUkgW6u+JjEbGTKPAdkYUR2bVm4v/ed0Uw1s/EypJkSu2XBSmkmBM5n+TgdCcoZxaQpkW9lbCRlRThjadkg3BW315nbSuqt511X2oVepuHkcRzuAcLsGDG6jDPTSgCQyG8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwAJGI2T</latexit>

<latexit sha1_base64="/OLkXieDCvHsLVK3PwJnIpvYLuk=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBiyURRY8FLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777RTW1jc2t4rbpZ3dvf2D8uFRS8epYthksYhVJ6AaBZfYNNwI7CQKaRQIbAfju5nffkKleSwfzCRBP6JDyUPOqLFS46JfrrhVdw6ySrycVCBHvV/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/YzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJ67LqXVfdxlWl5uZxFOEETuEcPLiBGtxDHZrAAOEZXuHNeXRenHfnY9FacPKZY/gD5/MHcHWMow==</latexit>

19

28

<latexit sha1_base64="ODKckztvqzcEnxFqWHBmZk2ioEQ=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0nEot4KXjxWsR/QhrLZbtqlm03YnQgl9B948aCIV/+RN/+N2zYHbX0w8Hhvhpl5QSKFQdf9dgpr6xubW8Xt0s7u3v5B+fCoZeJUM95ksYx1J6CGS6F4EwVK3kk0p1EgeTsY38789hPXRsTqEScJ9yM6VCIUjKKVHrybfrniVt05yCrxclKBHI1++as3iFkacYVMUmO6npugn1GNgkk+LfVSwxPKxnTIu5YqGnHjZ/NLp+TMKgMSxtqWQjJXf09kNDJmEgW2M6I4MsveTPzP66YYXvuZUEmKXLHFojCVBGMye5sMhOYM5cQSyrSwtxI2opoytOGUbAje8surpHVR9WpV9/6yUnfzOIpwAqdwDh5cQR3uoAFNYBDCM7zCmzN2Xpx352PRWnDymWP4A+fzB/H+jOo=</latexit>

<latexit sha1_base64="9Sfh0XWDmzkaE0f9OsoMolRyIYI=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKYo8FLx6r2A9oQ9lsJ+3SzSbsboQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpoVYflCtu1V2ArBMvJxXI0RyUv/rDmKURSsME1brnuYnxM6oMZwJnpX6qMaFsQkfYs1TSCLWfLS6dkQurDEkYK1vSkIX6eyKjkdbTKLCdETVjverNxf+8XmrCup9xmaQGJVsuClNBTEzmb5MhV8iMmFpCmeL2VsLGVFFmbDglG4K3+vI6adeq3nXVvb+qNNw8jiKcwTlcggc30IA7aEILGITwDK/w5kycF+fd+Vi2Fpx85hT+wPn8AfH/jOo=</latexit>

32

v6

v4

<latexit sha1_base64="KhzAUr+nU3Bzf9rBbcqe/n2D/TM=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0nEoseCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0m/1i9X3Kq7AFknXk4qkKPRL3/1BjFLI66QSWpM13MT9DOqUTDJZ6VeanhC2ZgOeddSRSNu/Gxx6oxcWGVAwljbUkgW6u+JjEbGTKPAdkYUR2bVm4v/ed0Uw1s/EypJkSu2XBSmkmBM5n+TgdCcoZxaQpkW9lbCRlRThjadkg3BW315nbSuql6t6j5cV+puHkcRzuAcLsGDG6jDPTSgCQyG8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwAKnI2U</latexit>

<latexit sha1_base64="YgTOWuiwh5P0BEOvcER0+0obgco=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mqoseCF49V7Ae0oWy2k3bpZhN2N0IJ/QdePCji1X/kzX/jts1BWx8MPN6bYWZekAiujet+O4W19Y3NreJ2aWd3b/+gfHjU0nGqGDZZLGLVCahGwSU2DTcCO4lCGgUC28H4dua3n1BpHstHM0nQj+hQ8pAzaqz0cFHrlytu1Z2DrBIvJxXI0eiXv3qDmKURSsME1brruYnxM6oMZwKnpV6qMaFsTIfYtVTSCLWfzS+dkjOrDEgYK1vSkLn6eyKjkdaTKLCdETUjvezNxP+8bmrCGz/jMkkNSrZYFKaCmJjM3iYDrpAZMbGEMsXtrYSNqKLM2HBKNgRv+eVV0qpVvauqe39Zqbt5HEU4gVM4Bw+uoQ530IAmMAjhGV7hzRk7L86787FoLTj5zDH8gfP5A+psjOU=</latexit>

<latexit sha1_base64="oab+wWB+yH0NnADlzCIUaP/3n5g=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBbBU0mkoseCF48V7Qe0oWy2m3bpZhN2J4US+hO8eFDEq7/Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSMnGqGW+yWMa6E1DDpVC8iQIl7ySa0yiQvB2M7+Z+e8K1EbF6wmnC/YgOlQgFo2ilx0m/1i9X3Kq7AFknXk4qkKPRL3/1BjFLI66QSWpM13MT9DOqUTDJZ6VeanhC2ZgOeddSRSNu/Gxx6oxcWGVAwljbUkgW6u+JjEbGTKPAdkYUR2bVm4v/ed0Uw1s/EypJkSu2XBSmkmBM5n+TgdCcoZxaQpkW9lbCRlRThjadkg3BW315nbSuqt511X2oVepuHkcRzuAcLsGDG6jDPTSgCQyG8Ayv8OZI58V5dz6WrQUnnzmFP3A+fwAJGI2T</latexit>

36
<latexit sha1_base64="RdFlxE7mxIou44pmjh3nVjI+9Cs=">AAAB6XicbVDLSgNBEOyNrxhfUY9eBoPgKez6Pga8eIxiHpAsYXbSmwyZnV1mZoWw5A+8eFDEq3/kzb9xkuxBEwsaiqpuuruCRHBtXPfbKaysrq1vFDdLW9s7u3vl/YOmjlPFsMFiEat2QDUKLrFhuBHYThTSKBDYCka3U7/1hErzWD6acYJ+RAeSh5xRY6WH86teueJW3RnIMvFyUoEc9V75q9uPWRqhNExQrTuemxg/o8pwJnBS6qYaE8pGdIAdSyWNUPvZ7NIJObFKn4SxsiUNmam/JzIaaT2OAtsZUTPUi95U/M/rpCa88TMuk9SgZPNFYSqIicn0bdLnCpkRY0soU9zeStiQKsqMDadkQ/AWX14mzbOqd1l17y8qNTePowhHcAyn4ME11OAO6tAABiE8wyu8OSPnxXl3PuatBSefOYQ/cD5/APB8jOk=</latexit>

<latexit sha1_base64="bCNT2eilmeDfYUSsQ2YAbCd+scM=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoPgKeyKr2PAi8eI5gHJEmYns8mQ2dllpjcQlnyCFw+KePWLvPk3TpI9aGJBQ1HVTXdXkEhh0HW/ncLa+sbmVnG7tLO7t39QPjxqmjjVjDdYLGPdDqjhUijeQIGStxPNaRRI3gpGdzO/NebaiFg94SThfkQHSoSCUbTS47h33StX3Ko7B1klXk4qkKPeK391+zFLI66QSWpMx3MT9DOqUTDJp6VuanhC2YgOeMdSRSNu/Gx+6pScWaVPwljbUkjm6u+JjEbGTKLAdkYUh2bZm4n/eZ0Uw1s/EypJkSu2WBSmkmBMZn+TvtCcoZxYQpkW9lbChlRThjadkg3BW355lTQvqt5V1X24rNTcPI4inMApnIMHN1CDe6hDAxgM4Ble4c2Rzovz7nwsWgtOPnMMf+B8/gAMII2V</latexit>

41
<latexit sha1_base64="pyECjY5t+aCWsaB6lV5omyqm/ps=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mkRY8FLx6r2A9oQ9lsN+3SzSbsToQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IJHCoOt+O4WNza3tneJuaW//4PCofHzSNnGqGW+xWMa6G1DDpVC8hQIl7yaa0yiQvBNMbud+54lrI2L1iNOE+xEdKREKRtFKDzVvUK64VXcBsk68nFQgR3NQ/uoPY5ZGXCGT1Jie5yboZ1SjYJLPSv3U8ISyCR3xnqWKRtz42eLSGbmwypCEsbalkCzU3xMZjYyZRoHtjCiOzao3F//zeimGN34mVJIiV2y5KEwlwZjM3yZDoTlDObWEMi3srYSNqaYMbTglG4K3+vI6aV9VvXrVva9VGm4eRxHO4BwuwYNraMAdNKEFDEJ4hld4cybOi/PufCxbC04+cwp/4Hz+AOptjOU=</latexit>

<latexit sha1_base64="/OLkXieDCvHsLVK3PwJnIpvYLuk=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBiyURRY8FLx5bsB/QhrLZTtq1m03Y3Qgl9Bd48aCIV3+SN/+N2zYHbX0w8Hhvhpl5QSK4Nq777RTW1jc2t4rbpZ3dvf2D8uFRS8epYthksYhVJ6AaBZfYNNwI7CQKaRQIbAfju5nffkKleSwfzCRBP6JDyUPOqLFS46JfrrhVdw6ySrycVCBHvV/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/YzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJ67LqXVfdxlWl5uZxFOEETuEcPLiBGtxDHZrAAOEZXuHNeXRenHfnY9FacPKZY/gD5/MHcHWMow==</latexit>

48 50
<latexit sha1_base64="ge8ZeDEWgfrDALKaGPQG436b+Zc=">AAAB6XicbVBNS8NAEJ3Ur1q/qh69LBbBU0mkYo8FLx6r2A9oQ9lsJ+3SzSbsboQS+g+8eFDEq//Im//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ7dzvPKHSPJaPZpqgH9GR5CFn1FjpoVYflCtu1V2ArBMvJxXI0RyUv/rDmKURSsME1brnuYnxM6oMZwJnpX6qMaFsQkfYs1TSCLWfLS6dkQurDEkYK1vSkIX6eyKjkdbTKLCdETVjverNxf+8XmrCup9xmaQGJVsuClNBTEzmb5MhV8iMmFpCmeL2VsLGVFFmbDglG4K3+vI6aV9Vveuqe1+rNNw8jiKcwTlcggc30IA7aEILGITwDK/w5kycF+fd+Vi2Fpx85hT+wPn8AfUJjOw=</latexit>

<latexit sha1_base64="Vha4Q/+MYS2LlJ7Njn6YaRWY7os=">AAAB6XicbVDLSgNBEOyNrxhfUY9eBoPgKcyKQY8BLx6jmAckS5id9CZDZmeXmVkhhPyBFw+KePWPvPk3TpI9aLSgoajqprsrTKUwltIvr7C2vrG5Vdwu7ezu7R+UD49aJsk0xyZPZKI7ITMohcKmFVZiJ9XI4lBiOxzfzP32I2ojEvVgJykGMRsqEQnOrJPua7RfrtAqXYD8JX5OKpCj0S9/9gYJz2JUlktmTNenqQ2mTFvBJc5KvcxgyviYDbHrqGIxmmC6uHRGzpwyIFGiXSlLFurPiSmLjZnEoeuMmR2ZVW8u/ud1MxtdB1Oh0syi4stFUSaJTcj8bTIQGrmVE0cY18LdSviIacatC6fkQvBXX/5LWhdVv1ald5eVOs3jKMIJnMI5+HAFdbiFBjSBQwRP8AKv3th79t6892VrwctnjuEXvI9v6m6M5Q==</latexit>

Figure 5: The nodes in M ⊙ are colored grey. The interval [lv , rv ] covered by right(v)
is written next to the nodes. The list L⊙ = {(5, 7, v1 ), (8, 10, v3 ), (11, 14, v1 ), (15, 19, v2 ),
(28, 31, v4 ), (32, 36, v5 ), (37, 49, v4 ), (41, 48, v6 )}. In the final algorithm, we compute and return
⊙
⊙
⊙
⊙
the union of δ[5,7]
(v1 , α), δ[8,10]
(v3 , α), δ[11,14]
(v1 , α), . . . , δ[41,48]
(v6 , α).
5.3.1

Computing Transitions Nodes

We first explain how to compute the transition nodes using the next-pointers and the transition tree.
Computing M ⊙

We find for each segment s all ⊙-transition nodes for δ(P, α) as follows.

1. Find first relevant ⊙-node on s:
⊙-live node stop.

Set x = parent⊙ (bot(s)). If x is not on s or x is not an

2. Find first ⊙-transition node on s: Compute the successor q of x in Posα , i.e., q is the successor
in Posα of the leftmost leaf in R(right(x)). If no such q exists stop, otherwise set v = lca(x, q).
If v is not on s stop. If v is a ⊙-live node not labeled α we add v to M ⊙ . Now compute
x = firstlabel(v, α). If x is not a ⊙-live node set x = next⊙ (x, α).
3. Repeatedly find next transition node by following next⊙ -pointers: We find all ⊙-transition
nodes by repeatedly following next⊙ -pointers from x and adding the visited nodes to M ⊙
as follows. As long as left(x) ∈ lastextent(P ) and x is on s we add x to M ⊙ and set
x = next⊙ (x, α).
Computing M ∗

We find for each segment s = (v, w) all ∗-transition nodes for δ(P, α).

1. If s is not a ∗-segment we stop.
2. Find first ∗-transition node on s: Compute the predecessor q − and successor q + of bot(s) in
Posα and let v be the lowest of lca(bot(s), q − ) and lca(bot(s), q + ). If v is not on s stop. If
α ̸∈ label(v) and parent∗ (v) ∈ lastextent(P ) we add v to M ∗ .
3. Repeatedly find next transition node by following next∗ -pointers: We first compute x =
firstlabel(v, α). We find all ∗-transition nodes by repeatedly following next∗ -pointers from x
and adding the visited nodes to M ∗ as follows. As long as parent∗ (x) ∈ lastextent(P ) and x is
on s we add x to M ∗ and set x = next∗ (v, α).

14

<!-- page 16 -->

Complexity We first analyze the time used to find M ⊙ . We use O(|M ⊙ |) time to follow pointers.
Additionally, we use O(|T | log log m) = O(|P | log log m) time to compute the first label queries, as
we do one first label query on each of the |T | segments. The time to check if a node is ⊙-live is
constant. For all the bottom nodes of the segments the information is stored in the tree and for
all the other nodes v we can check in constant time if left(v) ∈ lastextent(P ) as described in the
end of Section 5.2 as left(v) will always be on the current segment. Thus the total time used is
O(|M ⊙ | + |P | log log m). Similarly, we use time O(|M ∗ | + |P | log log m) to compute M ∗ . Next we
analyze the size of M ⊙ and M ∗ .
Lemma 12. |M ⊙ | = O(|δ ⊙ (P, α)| + |P |).

Proof. We will prove that at most one node from M ⊙ from each segment is not in N ⊙ (P, α).
Recall that N ⊙ (P, α) consists of all the ⊙-nodes v that have left(v) ∈ lastextent(P ) and right(v) ∈
firstextent(Posα ).
Any node u ∈ s added to M ⊙ in step 3 except the first one has δ(u, α) ̸= ∅, since they were
found using next⊙ -pointers. Thus, right(u) ∈ firstextent(Posα ). A node is only added if it is ⊙-live,
i.e., left(u) ∈ lastextent(P ). Thus u ∈ N ⊙ (P, α). Therefore, only the first node found on each
segment—the node from step 2—might not be in N ⊙ (P, α). Since there are O(|P |) segments in T
we have |M ⊙ | = O(|N ⊙ (P, α)| + |P |). By Lemma 7 we have |N ⊙ (P, α)| ≤ |δ ⊙ (P, α)| + |P | and thus
|M ⊙ | = O(|δ ⊙ (P, α)| + |P |).
The argument for the size of M ∗ is similar, but here we show that the number of nodes in M ∗
that are not in N ∗ (P, α) is at most 2|P |.

Lemma 13. |M ∗ | = O(|δ ∗ (P, α)| + |P |).

Proof. We will prove that at most two nodes from M ∗ from each segment is not in N ∗ (P, α). Recall
that N ∗ (P, α) consists of all the nodes v such that there exists a q ∈ Posα and a p ∈ P such that
v = lca(p, q) and parent∗ (v) ∈ lastextent(p) ∩ firstextent(q).
Any node u added to M ∗ by following next∗ -pointers has δ ∗ (u, α) ̸= ∅ and parent∗ (u) ∈
lastextent(P ). Furthermore, if left(u) is on the segment s, then there exists a position q ∈
right(u) ∩ δ ∗ (u, α) and a position p ∈ P ∩ left(u). This implies that u is in N ∗ (P, α). The argument for the case where right(u) is on the segment s is symmetric.
At most two other nodes are added to M ∗ for each segment (the first two nodes are added to
M ∗ on each segment). It follows that the total number of nodes in M ∗ is at most |δ ∗ (P, α)| + 2|P |.
By Lemma 7 we have |N ∗ (P, α)| ≤ |δ ∗ (P, α)| + |P | and thus |M ∗ | = O(|δ ∗ (P, α)| + |P |).
Combining Lemmas 12 and 13 and the above discussion, the total time to compute M ⊙ and M ∗
is O(|δ ⊙ (P, α)| + |P | log log m) and O(|δ ∗ (P, α)| + |P | log log m), respectively.
Correctness We argue that the sets M ⊙ and M ∗ include all transition nodes for δ(P, α). We
need the following lemma, which follows from the path structure of the last extent sets.
Lemma 14. Let u be an ⊙-live node and let s be the segment in T containing u. All ⊙-nodes below
u on s with left(u) on s are also ⊙-live.
Proof. If left(u) is not on s then u is a branching node in T by construction of T . Thus u = bot(s)
and it is trivially true since u has no descendants on s. We will prove the case left(u) on s by
contradiction. Since u is ⊙-live there exists a node p ∈ left(u) ∩ P such that u ∈ lastextent(p).
Furthermore, bot(s) is an ancestor of some node p′ in P . Let w = lca(bot(s), p). If p′ = p then
w = bot(s). If p′ ̸= p then w is a branching node in T . Since there are no branching nodes internally
on a segment it follows that w = bot(s). By Lemma 4 all nodes on s are in lastextent(p).
15

<!-- page 17 -->

⇤
<latexit sha1_base64="eR3rXfUZKEjdTewmcytIoZeAYvs=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2FZoQ9lsN+3S3STsToQS+he8eFDEq3/Im//GTZuDtj4YeLw3w8y8IJHCoOt+O6WNza3tnfJuZW//4PCoenzSNXGqGe+wWMb6MaCGSxHxDgqU/DHRnKpA8l4wvc393hPXRsTRA84S7is6jkQoGMVcGlCDw2rNrbsLkHXiFaQGBdrD6tdgFLNU8QiZpMb0PTdBP6MaBZN8XhmkhieUTemY9y2NqOLGzxa3zsmFVUYkjLWtCMlC/T2RUWXMTAW2U1GcmFUvF//z+imGN34moiRFHrHlojCVBGOSP05GQnOGcmYJZVrYWwmbUE0Z2ngqNgRv9eV10m3Uvat6875ZazWKOMpwBudwCR5cQwvuoA0dYDCBZ3iFN0c5L86787FsLTnFzCn8gfP5Axwvjj8=</latexit>

u
<latexit sha1_base64="0wecHVVeuwuNqCAQlEJBFS0+0vw=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKqMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipmQ7KFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16redbXWvKrUa3kcRTiDc7gED26gDvfQgBYwQHiGV3hzHp0X5935WLYWnHzmFP7A+fwB3yeM8A==</latexit>

u

u

<latexit sha1_base64="0wecHVVeuwuNqCAQlEJBFS0+0vw=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKqMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipmQ7KFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16redbXWvKrUa3kcRTiDc7gED26gDvfQgBYwQHiGV3hzHp0X5935WLYWnHzmFP7A+fwB3yeM8A==</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

x

<latexit sha1_base64="hp81Dr1Gw44DD0zPmEoJQSX8PGQ=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBahXkpSRD0WvHisaGuhDWWy3bRLN5uwuxFK6E/w4kERr/4ib/4bt20O2vpg4PHeDDPzgkRwbVz32ymsrW9sbhW3Szu7e/sH5cOjto5TRVmLxiJWnQA1E1yyluFGsE6iGEaBYI/B+GbmPz4xpXksH8wkYX6EQ8lDTtFY6b6K5/1yxa25c5BV4uWkAjma/fJXbxDTNGLSUIFadz03MX6GynAq2LTUSzVLkI5xyLqWSoyY9rP5qVNyZpUBCWNlSxoyV39PZBhpPYkC2xmhGellbyb+53VTE177GZdJapiki0VhKoiJyexvMuCKUSMmliBV3N5K6AgVUmPTKdkQvOWXV0m7XvMua97dRaVRz+MowgmcQhU8uIIG3EITWkBhCM/wCm+OcF6cd+dj0Vpw8plj+APn8weF041A</latexit>

(a)

<latexit sha1_base64="0wecHVVeuwuNqCAQlEJBFS0+0vw=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKqMeCF48t2A9oQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgobm1vbO8Xd0t7+weFR+fikreNUMWyxWMSqG1CNgktsGW4EdhOFNAoEdoLJ3dzvPKHSPJYPZpqgH9GR5CFn1FipmQ7KFbfqLkDWiZeTCuRoDMpf/WHM0gilYYJq3fPcxPgZVYYzgbNSP9WYUDahI+xZKmmE2s8Wh87IhVWGJIyVLWnIQv09kdFI62kU2M6ImrFe9ebif14vNeGtn3GZpAYlWy4KU0FMTOZfkyFXyIyYWkKZ4vZWwsZUUWZsNiUbgrf68jpp16redbXWvKrUa3kcRTiDc7gED26gDvfQgBYwQHiGV3hzHp0X5935WLYWnHzmFP7A+fwB3yeM8A==</latexit>

<latexit sha1_base64="hf4FI+Q3xw5AN9LocCmV/nHsnu8=">AAAB7HicbZBNS8NAEIYn9avWr6pHL8EieCqJCHosePFYwbSFNpTNZtMu3eyG3YlQQn+DFw+KePUHefPfuG1z0NYXFh7emWFn3igT3KDnfTuVjc2t7Z3qbm1v/+DwqH580jEq15QFVAmlexExTHDJAuQoWC/TjKSRYN1ocjevd5+YNlzJR5xmLEzJSPKEU4LWCgYqVjisN7ymt5C7Dn4JDSjVHta/BrGiecokUkGM6ftehmFBNHIq2Kw2yA3LCJ2QEetblCRlJiwWy87cC+vEbqK0fRLdhft7oiCpMdM0sp0pwbFZrc3N/2r9HJPbsOAyy5FJuvwoyYWLyp1f7sZcM4piaoFQze2uLh0TTSjafGo2BH/15HXoXDV9yw/XjdZ1GUcVzuAcLsGHG2jBPbQhAAocnuEV3hzpvDjvzseyteKUM6fwR87nD+cqjrA=</latexit>

<latexit sha1_base64="wkKfwlFB9/ey+ROXQFF6Ib71vvw=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2E3BPUY8OIxonlAsoTZSW8yZHZ2mZkVQsgnePGgiFe/yJt/4yTZgyYWNBRV3XR3BYng2rjut5Pb2Nza3snvFvb2Dw6PiscnLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb+d++wmV5rF8NJME/YgOJQ85o8ZKD+Xgsl8suRV3AbJOvIyUIEOjX/zqDWKWRigNE1Trrucmxp9SZTgTOCv0Uo0JZWM6xK6lkkao/eni1Bm5sMqAhLGyJQ1ZqL8npjTSehIFtjOiZqRXvbn4n9dNTXjjT7lMUoOSLReFqSAmJvO/yYArZEZMLKFMcXsrYSOqKDM2nYINwVt9eZ20qhXvqlK7r5Xq1SyOPJzBOZTBg2uowx00oAkMhvAMr/DmCOfFeXc+lq05J5s5hT9wPn8AiE6NRA==</latexit>

<latexit sha1_base64="eR3rXfUZKEjdTewmcytIoZeAYvs=">AAAB63icbVBNS8NAEJ3Ur1q/qh69LBbBU0lKUY8FLx4r2FZoQ9lsN+3S3STsToQS+he8eFDEq3/Im//GTZuDtj4YeLw3w8y8IJHCoOt+O6WNza3tnfJuZW//4PCoenzSNXGqGe+wWMb6MaCGSxHxDgqU/DHRnKpA8l4wvc393hPXRsTRA84S7is6jkQoGMVcGlCDw2rNrbsLkHXiFaQGBdrD6tdgFLNU8QiZpMb0PTdBP6MaBZN8XhmkhieUTemY9y2NqOLGzxa3zsmFVUYkjLWtCMlC/T2RUWXMTAW2U1GcmFUvF//z+imGN34moiRFHrHlojCVBGOSP05GQnOGcmYJZVrYWwmbUE0Z2ngqNgRv9eV10m3Uvat6875ZazWKOMpwBudwCR5cQwvuoA0dYDCBZ3iFN0c5L86787FsLTnFzCn8gfP5Axwvjj8=</latexit>

<latexit sha1_base64="b7fI0ASaJK2/xBrf9yCBeHMZ0P0=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2E3BPUY8OIxonlAsoTZSW8yZHZ2mZkVQsgnePGgiFe/yJt/4yTZgyYWNBRV3XR3BYng2rjut5Pb2Nza3snvFvb2Dw6PiscnLR2nimGTxSJWnYBqFFxi03AjsJMopFEgsB2Mb+d++wmV5rF8NJME/YgOJQ85o8ZKD2V22S+W3Iq7AFknXkZKkKHRL371BjFLI5SGCap113MT40+pMpwJnBV6qcaEsjEdYtdSSSPU/nRx6oxcWGVAwljZkoYs1N8TUxppPYkC2xlRM9Kr3lz8z+umJrzxp1wmqUHJlovCVBATk/nfZMAVMiMmllCmuL2VsBFVlBmbTsGG4K2+vE5a1Yp3Vand10r1ahZHHs7gHMrgwTXU4Q4a0AQGQ3iGV3hzhPPivDsfy9ack82cwh84nz+J041F</latexit>

(b)

<latexit sha1_base64="Tdo8RPsK395dTnon50Pq9f+UXtY=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHYJUY8kXjxCIo8ENmR2aGBkdnYzM2skG77AiweN8eonefNvHGAPClbSSaWqO91dQSy4Nq777eQ2Nre2d/K7hb39g8Oj4vFJS0eJYthkkYhUJ6AaBZfYNNwI7MQKaRgIbAeT27nffkSleSTvzTRGP6QjyYecUWOlxlO/WHLL7gJknXgZKUGGer/41RtELAlRGiao1l3PjY2fUmU4Ezgr9BKNMWUTOsKupZKGqP10ceiMXFhlQIaRsiUNWai/J1Iaaj0NA9sZUjPWq95c/M/rJmZ446dcxolByZaLhokgJiLzr8mAK2RGTC2hTHF7K2FjqigzNpuCDcFbfXmdtCpl76pcbVRLtUoWRx7O4BwuwYNrqMEd1KEJDBCe4RXenAfnxXl3PpatOSebOYU/cD5/AORXjPU=</latexit>

⇤
(c)

x
<latexit sha1_base64="Tdo8RPsK395dTnon50Pq9f+UXtY=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHYJUY8kXjxCIo8ENmR2aGBkdnYzM2skG77AiweN8eonefNvHGAPClbSSaWqO91dQSy4Nq777eQ2Nre2d/K7hb39g8Oj4vFJS0eJYthkkYhUJ6AaBZfYNNwI7MQKaRgIbAeT27nffkSleSTvzTRGP6QjyYecUWOlxlO/WHLL7gJknXgZKUGGer/41RtELAlRGiao1l3PjY2fUmU4Ezgr9BKNMWUTOsKupZKGqP10ceiMXFhlQIaRsiUNWai/J1Iaaj0NA9sZUjPWq95c/M/rJmZ446dcxolByZaLhokgJiLzr8mAK2RGTC2hTHF7K2FjqigzNpuCDcFbfXmdtCpl76pcbVRLtUoWRx7O4BwuwYNrqMEd1KEJDBCe4RXenAfnxXl3PpatOSebOYU/cD5/AORXjPU=</latexit>

q q0

q
<latexit sha1_base64="hep7KbXLnKjf7S5wE4cnXU4+omk=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHYJUY8kXjxCIo8ENmR2aGBkdnadmTUhG77AiweN8eonefNvHGAPClbSSaWqO91dQSy4Nq777eQ2Nre2d/K7hb39g8Oj4vFJS0eJYthkkYhUJ6AaBZfYNNwI7MQKaRgIbAeT27nffkKleSTvzTRGP6QjyYecUWOlxmO/WHLL7gJknXgZKUGGer/41RtELAlRGiao1l3PjY2fUmU4Ezgr9BKNMWUTOsKupZKGqP10ceiMXFhlQIaRsiUNWai/J1Iaaj0NA9sZUjPWq95c/M/rJmZ446dcxolByZaLhokgJiLzr8mAK2RGTC2hTHF7K2FjqigzNpuCDcFbfXmdtCpl76pcbVRLtUoWRx7O4BwuwYNrqMEd1KEJDBCe4RXenAfnxXl3PpatOSebOYU/cD5/ANm7jO4=</latexit>

q0

<latexit sha1_base64="8Exvn8rVjq2bBlEgsvbCmqU7HsY=">AAAB6XicbVDLTgJBEOzFF+IL9ehlIjF6IruEqEcSLx7RyCOBDZkdemHC7Ow6M2tCCH/gxYPGePWPvPk3DrAHBSvppFLVne6uIBFcG9f9dnJr6xubW/ntws7u3v5B8fCoqeNUMWywWMSqHVCNgktsGG4EthOFNAoEtoLRzcxvPaHSPJYPZpygH9GB5CFn1Fjp/vG8Vyy5ZXcOskq8jJQgQ71X/Or2Y5ZGKA0TVOuO5ybGn1BlOBM4LXRTjQllIzrAjqWSRqj9yfzSKTmzSp+EsbIlDZmrvycmNNJ6HAW2M6JmqJe9mfif10lNeO1PuExSg5ItFoWpICYms7dJnytkRowtoUxxeythQ6ooMzacgg3BW355lTQrZe+yXL2rlmqVLI48nMApXIAHV1CDW6hDAxiE8Ayv8OaMnBfn3flYtOacbOYY/sD5/AE6O40f</latexit>

<latexit sha1_base64="YnTDxIN4/ZYfWSUVt/wISquMdrc=">AAAB6XicbVBNS8NAEJ34WetX1aOXxSJ6KkkR9Vjw4rGK/YA2lM120i7dbOLuRiih/8CLB0W8+o+8+W/ctjlo64OBx3szzMwLEsG1cd1vZ2V1bX1js7BV3N7Z3dsvHRw2dZwqhg0Wi1i1A6pRcIkNw43AdqKQRoHAVjC6mfqtJ1Sax/LBjBP0IzqQPOSMGivdP571SmW34s5AlomXkzLkqPdKX91+zNIIpWGCat3x3MT4GVWGM4GTYjfVmFA2ogPsWCpphNrPZpdOyKlV+iSMlS1pyEz9PZHRSOtxFNjOiJqhXvSm4n9eJzXhtZ9xmaQGJZsvClNBTEymb5M+V8iMGFtCmeL2VsKGVFFmbDhFG4K3+PIyaVYr3mWlendRrlXzOApwDCdwDh5cQQ1uoQ4NYBDCM7zCmzNyXpx352PeuuLkM0fwB87nDzmXjR0=</latexit>

<latexit sha1_base64="hep7KbXLnKjf7S5wE4cnXU4+omk=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHYJUY8kXjxCIo8ENmR2aGBkdnadmTUhG77AiweN8eonefNvHGAPClbSSaWqO91dQSy4Nq777eQ2Nre2d/K7hb39g8Oj4vFJS0eJYthkkYhUJ6AaBZfYNNwI7MQKaRgIbAeT27nffkKleSTvzTRGP6QjyYecUWOlxmO/WHLL7gJknXgZKUGGer/41RtELAlRGiao1l3PjY2fUmU4Ezgr9BKNMWUTOsKupZKGqP10ceiMXFhlQIaRsiUNWai/J1Iaaj0NA9sZUjPWq95c/M/rJmZ446dcxolByZaLhokgJiLzr8mAK2RGTC2hTHF7K2FjqigzNpuCDcFbfXmdtCpl76pcbVRLtUoWRx7O4BwuwYNrqMEd1KEJDBCe4RXenAfnxXl3PpatOSebOYU/cD5/ANm7jO4=</latexit>

x
<latexit sha1_base64="Tdo8RPsK395dTnon50Pq9f+UXtY=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHYJUY8kXjxCIo8ENmR2aGBkdnYzM2skG77AiweN8eonefNvHGAPClbSSaWqO91dQSy4Nq777eQ2Nre2d/K7hb39g8Oj4vFJS0eJYthkkYhUJ6AaBZfYNNwI7MQKaRgIbAeT27nffkSleSTvzTRGP6QjyYecUWOlxlO/WHLL7gJknXgZKUGGer/41RtELAlRGiao1l3PjY2fUmU4Ezgr9BKNMWUTOsKupZKGqP10ceiMXFhlQIaRsiUNWai/J1Iaaj0NA9sZUjPWq95c/M/rJmZ446dcxolByZaLhokgJiLzr8mAK2RGTC2hTHF7K2FjqigzNpuCDcFbfXmdtCpl76pcbVRLtUoWRx7O4BwuwYNrqMEd1KEJDBCe4RXenAfnxXl3PpatOSebOYU/cD5/AORXjPU=</latexit>

q
<latexit sha1_base64="hep7KbXLnKjf7S5wE4cnXU4+omk=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHYJUY8kXjxCIo8ENmR2aGBkdnadmTUhG77AiweN8eonefNvHGAPClbSSaWqO91dQSy4Nq777eQ2Nre2d/K7hb39g8Oj4vFJS0eJYthkkYhUJ6AaBZfYNNwI7MQKaRgIbAeT27nffkKleSTvzTRGP6QjyYecUWOlxmO/WHLL7gJknXgZKUGGer/41RtELAlRGiao1l3PjY2fUmU4Ezgr9BKNMWUTOsKupZKGqP10ceiMXFhlQIaRsiUNWai/J1Iaaj0NA9sZUjPWq95c/M/rJmZ446dcxolByZaLhokgJiLzr8mAK2RGTC2hTHF7K2FjqigzNpuCDcFbfXmdtCpl76pcbVRLtUoWRx7O4BwuwYNrqMEd1KEJDBCe4RXenAfnxXl3PpatOSebOYU/cD5/ANm7jO4=</latexit>

<latexit sha1_base64="GKbbUbUnvdab1UrVHcMNt7JtPew=">AAAB6HicbZC7SgNBFIbPeo0xatRSkMEgWIXdFGoZsLFMwFwgWcLs5CQZMzu7zMwKYUlpZWOhiK1Pkeew8xl8CSeXQhN/GPj4/3OYc04QC66N6345a+sbm1vbmZ3sbm5v/yB/eFTXUaIY1lgkItUMqEbBJdYMNwKbsUIaBgIbwfBmmjceUGkeyTszitEPaV/yHmfUWKsad/IFt+jORFbBW0ChnJtUvx9PJ5VO/rPdjVgSojRMUK1bnhsbP6XKcCZwnG0nGmPKhrSPLYuShqj9dDbomJxbp0t6kbJPGjJzf3ekNNR6FAa2MqRmoJezqflf1kpM79pPuYwTg5LNP+olgpiITLcmXa6QGTGyQJnidlbCBlRRZuxtsvYI3vLKq1AvFb3LYqnqFcolmCsDJ3AGF+DBFZThFipQAwYIT/ACr8698+y8Oe/z0jVn0XMMf+R8/ACvMpCD</latexit>

<latexit sha1_base64="GKbbUbUnvdab1UrVHcMNt7JtPew=">AAAB6HicbZC7SgNBFIbPeo0xatRSkMEgWIXdFGoZsLFMwFwgWcLs5CQZMzu7zMwKYUlpZWOhiK1Pkeew8xl8CSeXQhN/GPj4/3OYc04QC66N6345a+sbm1vbmZ3sbm5v/yB/eFTXUaIY1lgkItUMqEbBJdYMNwKbsUIaBgIbwfBmmjceUGkeyTszitEPaV/yHmfUWKsad/IFt+jORFbBW0ChnJtUvx9PJ5VO/rPdjVgSojRMUK1bnhsbP6XKcCZwnG0nGmPKhrSPLYuShqj9dDbomJxbp0t6kbJPGjJzf3ekNNR6FAa2MqRmoJezqflf1kpM79pPuYwTg5LNP+olgpiITLcmXa6QGTGyQJnidlbCBlRRZuxtsvYI3vLKq1AvFb3LYqnqFcolmCsDJ3AGF+DBFZThFipQAwYIT/ACr8698+y8Oe/z0jVn0XMMf+R8/ACvMpCD</latexit>

p

<latexit sha1_base64="GKbbUbUnvdab1UrVHcMNt7JtPew=">AAAB6HicbZC7SgNBFIbPeo0xatRSkMEgWIXdFGoZsLFMwFwgWcLs5CQZMzu7zMwKYUlpZWOhiK1Pkeew8xl8CSeXQhN/GPj4/3OYc04QC66N6345a+sbm1vbmZ3sbm5v/yB/eFTXUaIY1lgkItUMqEbBJdYMNwKbsUIaBgIbwfBmmjceUGkeyTszitEPaV/yHmfUWKsad/IFt+jORFbBW0ChnJtUvx9PJ5VO/rPdjVgSojRMUK1bnhsbP6XKcCZwnG0nGmPKhrSPLYuShqj9dDbomJxbp0t6kbJPGjJzf3ekNNR6FAa2MqRmoJezqflf1kpM79pPuYwTg5LNP+olgpiITLcmXa6QGTGyQJnidlbCBlRRZuxtsvYI3vLKq1AvFb3LYqnqFcolmCsDJ3AGF+DBFZThFipQAwYIT/ACr8698+y8Oe/z0jVn0XMMf+R8/ACvMpCD</latexit>

p

p

Figure 6: The thick edges are the edges on the segment s and the black node is bot(s). In (a) both
q and q ′ are positions in the right subtree of u, and u = lca(x, q). In (b) the positions q and q ′ are
in different subtrees and node u has label α. In (c) the arrows indicates the next∗ -pointers.
We are now ready to prove that N ⊙ (P, α) is contained in M ⊙ . Let u be a node in N ⊙ (P, α).
There are two main cases in the proof depending on whether u labeled α or not. If not, then we
show that u = lca(x, q) in step 2. If u is labeled α, then we show, that either u is the first ⊙-node
on the segment, in which case it is added as the first node in step 3, or it is a node on the path
induced by the next⊙ -pointers. As we proved in Lemma 14 above, all the nodes on this path below
u are also ⊙-live and we show by induction that all the ⊙-live nodes from s on this path are added
to M ⊙ in step 3.
Lemma 15. N ⊙ (P, α) ⊆ M ⊙ .
Proof. Let u ∈ N ⊙ (P, α). By definition u is ⊙-live, right(v) ∈ firstextent(Posα ), and δ(u, α) ̸= ∅.
Let s be the segment in T containing u. We want to show that u is added to M ⊙ . Define
x = parent⊙ (bot(s)) and q as the successor of x in Posα as in step 1 and 2 of the algorithm. There
are two cases depending on whether u has label α or not.
Case 1: α ̸∈ label(u). We first show that u = lca(x, q). Since u is an ⊙-node we have that u is an
ancestor of x. If u is a proper ancestor of x then x ∈ R(left(u)). Since right(u) ∈ firstextent(Posα )
there is a position q ′ ∈ Posα such that q ′ ∈ Pos(right(u)). This implies that q ≤ q ′ . If q ∈
Pos(right(u)) then u = lca(x, q) and we are done (see Figure 6(a)). Assume q ̸∈ Pos(right(u))
and let w = lca(x, q) ̸= u. Since q < q ′ we have u ≺ w. But then q ∈ Pos(w) ⊂ Pos(left(u)).
Thus u = lca(q, q ′ ) and u has label α which is a contradiction (see Figure 6(b)). It follows that
u = lca(x, q). Thus, since u is ⊙-live and not labeled α, u is added to M ⊙ in step 2.
Case 2: α ∈ label(u). There are two subcases. In the first case u = parent⊙ (bot(s)). Then u = x
in step 1. Since right(u) ∈ firstextent(Posα ) we have u = v = lca(x, q) in step 2. Since u is labeled α
we have u = firstlabel(v, α) = firstlabel(u, α). Since left(u) ∈ lastextent(P ) we add u to M ⊙ in the
first iteration in step 3.
In the other case u ̸= parent⊙ (bot(s)). Let
Sα = {w ∈ s | w is a ⊙ -live node, left(w) ∈ s, α ∈ label(w) and δ ⊙ (w, α) ̸= ∅} .
Clearly, u ∈ Sα , and thus u ∈ M ⊙ follows from the following claim.

Claim 16. Let w be a node in Sα . Then w is added to M ⊙ in step 3.

16

<!-- page 18 -->

Proof: We prove the claim using induction on the height of w. For the base case let w be the lowest
node in Sα . Since w is ⊙-live it follows from Lemma 14 that parent⊙ (bot(s)) is also ⊙-live and
thus we continue from step 1 to step 2. Since right(w) ∈ firstextent(Posα ) we have w ⪯ v, where
v = lca(x, q) as computed in step 2. Now either w = firstlabel(v, α) and then w is added to M ⊙ in
the first iteration in step 3. Otherwise, w ≺ firstlabel(v, α). Since w is the lowest node in Sα then
x = firstlabel(v, α) has its next⊙ -pointer pointing to w. Now either x is an ⊙-live node, in which
case x is set to w in the end of the first iteration of step 3. Otherwise, x is set to w in the end of
step 2. In either case, w is added to M ⊙ in step 3.
Induction step: Let w ∈ Sα be a node that is not the lowest node in Sα . Let w′ ∈ Sα be the
highest node in Sα that is a proper descendant of w. Then w′ points to w. Let v = next⊙ (w′ , α).
By definition of the next⊙ -pointers v ∈ Sα and v ≺ w′ . Now, since w′ ∈ R(left(w)), α ∈ label(w)
and δ ⊙ (w, α) ̸= ∅, we have v = next⊙ (w′ , α) ⪰ w. If v ̸= w then w′ ≻ v ≻ w contradicting that w′
is the highest descendant of w in Sα . Thus v = w. By the induction hypothesis, w′ was added to
M ⊙ in step 3, whereafter we follow the next⊙ -pointer to w and add w in the next iteration. ■
We will now prove that N ∗ (P, α) ⊆ M ∗ . Here is an outline of the proof. Recall that for any
node u in N ∗ (P, α), there exists a position q in Posα and a position p in P , such that u is the lca of
q and p and parent∗ (u) is in both firstextent(q) and lastextent(p). We first prove that the segment
s containing u is a ∗-segment. Then it follows easily that if u = bot(s) then u is added to M ∗ in
step 2 or 3. If u is not the bottom node on s, then due to the properties of the transition tree the
child of u not on s contains q in its subtree. We then show that if u is not labeled α then it is added
to M ∗ in step 2. Otherwise, it is either added as the first node in step 3, or it is a node on the path
induced by the next∗ -pointers on s. By similar arguments as those in the previous proof all nodes
below u on this path has their parent∗ -node in lastextent(p), the first node on the path is x found
by a firstlabel query in step 3, and thus u is added to M ∗ in step 3. See Figure 6.
Lemma 17. N ∗ (P, α) ⊆ M ∗ .
Proof. Let u be a node in N ∗ (P, α). Then there exists a position p ∈ P and a position q ∈ Posα
such that u = lca(p, q) and parent∗ (u) ∈ lastextent(p) ∩ firstextent(q). Let s be the segment u is on.
We first prove that s is a ∗-segment: If u = bot(s) then parent∗ (bot(s)) = parent∗ (u) ∈ lastextent(P )
and thus s is a ∗-segment. Otherwise, u ≺ bot(s). By construction of T we have p ∈ Pos(bot(s))
and thus by Lemma 4 bot(s) ∈ lastextent(p), since parent∗ (u) is an ancestor of bot(s).
We now prove that u ∈ M ∗ . Let v be the lowest of lca(bot(s), q−) and lca(bot(s), q+) as
computed in step 2. If u = bot(s) then q ∈ Pos(bot(s)) and thus v = bot(s) = u. If α ̸∈ label(u)
then u is added to M ∗ in step 2. Otherwise, it is added in step 3.
If u ̸= bot(s), let us be the child of u that is on s and let uo be the child not on s. By construction
of T we have p ∈ Pos(us ) and p ∈ Pos(bot(s)), since all bottom nodes of segments in T have a
position from P in their subtree and any node that is the lca of two nodes in P is a branching node
in T . Thus q ∈ Pos(uo ). There are two cases depending on whether u is labeled α or not.
Case 1: α ̸∈ label(u). Since q ∈ Pos(uo ) we have Pos(us ) ∩ Posα = ∅. Thus, either q+ or q− is in
Pos(uo ) and then v = u. Therefore, u is added to M ∗ in step 2.
Case 2: α ∈ label(u). Then u is the lca of two positions in Posα , which implies that there exists a
position labeled α in Pos(us ). It follows that either q− or q+ is in Pos(us ) and thus u ⪯ v. If u = v
then u is added to M ∗ in the first iteration in step 3. Otherwise, u ≺ v. Let x = firstlabel(v, α). If
u = x then u is added to M ∗ in the first iteration in step 3.
If u ̸= x then u ≺ x. Since q ∈ Pos(uo ) and parent∗ (u) ∈ firstextent(q) we have q ∈ δ ∗ (u, α).
Therefore, the highest proper descendant of u labeled α has its next∗ -pointer pointing to u. It
17

<!-- page 19 -->

follows that there is a chain of next∗ -pointers from x to u. It remains to show that for all the nodes
in this chain x = x0 , . . . , xk = u we have parent∗ (xi ) ∈ lastextent(P ). If this is true we will reach
u in step 3. Since p ∈ Pos(bot(s)) then all nodes on s are ancestors of p. Since u ⪯ xi we have
parent∗ (u) ⪯ parent∗ (xi ). By Lemma 4 we have parent∗ (xi ) ∈ lastextent(p).
5.3.2

Computing the Intervals

We now compute the lists L⊙ and L∗ of intervals for the nodes in M ⊙ and M ∗ , respectively. We do
this by processing the nodes M ⊙ and M ∗ in inorder using a depth-first left-to-right inorder traversal
of T .
First, we compute for each node v in M ⊙ the range [lv , rv ] of positions labeled α that are
descendants of right(v). If v is labeled α the range [lv , rv ] is stored at v and otherwise we use the
predecessor data structure to compute it using the range stored at right(v). Similarly, we compute
for each node in M ∗ the range [lv , rv ] of positions labeled α that are descendants of v.
Computing L⊙ We compute the list of intervals L⊙ by a depth-first left-to-right inorder traversal
of T . We maintain a stack S keeping track of the deepest node not finished and a counter ℓ equal
to the left starting point of the currently open interval. If there is no open interval ℓ = −1. Initially,
S = ∅ and ℓ = −1.
For each node v ∈ M ⊙ in inorder do the following:
• When we meet v in the traversal after traversing the left subtree of v: If ℓ ̸= −1 append
(ℓ, lv − 1, top(S)) to L⊙ . Set ℓ = lv and push v onto the stack S.
• When we finish the traversal of the subtree containing v: Note that in this case top(S) = v. If
ℓ ≤ rv append (ℓ, rv , v) to L⊙ . Pop v from S. If the stack is now empty set ℓ = −1, otherwise
set ℓ = rv + 1.
Note that right(v) might not be in T , in which case the two steps for v follow immediately after
each other.
Computing L∗ We maintain a stack S and counter ℓ as before.
For each node v ∈ M ∗ in inorder do the following:
• First time we meet v in the traversal: If ℓ ̸= −1 append (ℓ, lv − 1, top(S)) to L∗ . Set ℓ = lv
and push v onto the stack S.
• Last time we meet v in the traversal: Note that in this case top(S) = v. If ℓ ≤ rv append
(ℓ, rv , v) to L∗ . Pop v from S. If the stack is now empty set ℓ = −1, otherwise set ℓ = rv + 1.
Complexity To compute the ranges use time O(|M ⊙ | + |P | log log m) as at most one node in M ⊙
on each segment is not labeled α. We will store the nodes in M ⊙ in increasing order of depth for
each segment. This is easy to maintain as we find them in order of decreasing depth. This way we
can do the depth-first left-to-right traversal on the nodes of M ⊙ in T in linear time in the size of
M ⊙ . Thus computing L⊙ takes time O(|M ⊙ |).
Similarly, we use time O(|M ∗ |) to compute L∗ . In summary, we have the following lemma.
Lemma 18. The sets M ⊙ and M ∗ , and the lists L⊙ and L∗ , can be computed in time O(|δ(P, α)| +
|P | log log m).
18

<!-- page 20 -->

5.4

Internal Transitions

We will compute and return the state-set transition by computing the internal transitions on the
nodes in L⊙ and L∗ . Next, we show how to compute internal transitions efficiently using 3-sided
range queries. We assume that the range [l, r] is given as indexes in Aα .
⊙
⊙
Computing δ[l,r]
(v, α). Given an ⊙-node v, a character α, and a range [l, r] we compute δ[l,r]
(v, α)
as follows. We perform a 3-sided range reporting query (l, r, depth(right(v)) on Dα . That is, we
return all positions in Dα [l, r] with a value less than or equal to depth(right(v)). This can be done
by a standard technique of recursively applying range minimum queries as follows. Let j be the
position returned by range minimum query on Dα [l, r]. If Dα [j] ≤ depth(right(v)) return Aα [j] and
recurse on the ranges [l, j − 1] and [j + 1, r]. We stop if this is not the case or if the range is empty.
⊙
For instance, suppose we compute δ[2,4]
(v2 , a) in our example in Figure 2. The range [2, 4] in
Aα corresponds to the positions p2 , p3 , and p5 . We find the highest first extent in to be f = v4
corresponding to both p2 and p3 . Suppose j = 3 corresponding to p3 . Then we compare f with
right(v2 ) = v4 and since f is an ancestor of v4 we report p3 and repeat on the subarrays [2, 2] and
[4, 4]. On [2, 2] we return p2 , while on [4, 4] we do not get a position since v8 is a proper descendant
of v4 .
Note, that we can get the output in sorted order if we first recurse on the range [l, j − 1], then
report Aα [j], and then recurse on the range [j + 1, r].
The algorithm returns all positions q with label α in [l, r] such that right(v) ∈ firstextent(q) and
⊙
is thus correct. Each recursive call uses constant time and we repeat at most 2|δ[l,r]
(v, α)| + 1 times.
⊙
Hence, in total we use O(1 + |δ[l,r] (v, α)|) time.
∗ (v, α). To compute an internal transition for the ∗-case we do a 3-sided range
Computing δ[l,r]
reporting query (l, r, depth(parent∗ (v)). Correctness follows as above and the time and space bounds
are the same.
In summary, we have the following result.

Lemma 19. Let R be a regular expression of size m. Given R we can build a data structure in
O(m) space and preprocessing time, such that given a node v in R, a character α ∈ Σ, and a range
⊙
⊙
∗ (v, α) in sorted
[l, r] we can compute δ[l,r]
(v, α) in sorted order in time O(1 + |δ[l,r]
(v, α)|) and δ[l,r]
∗ (v, α)|).
order in time O(1 + |δ[l,r]

5.5

Computing State-Set Transitions

Given a set of positions P and a character α, we compute the state-set transition δ(P, α) as follows.
For simplicity, we assume that the positions in P are sorted according to their left-to-right order
since otherwise we can sort them in additional O(|P | log log m) time using integer sorting. The final
algorithm for computing fast state-set transitions is as follows.
1. First, we construct the transition tree and all the information from Lemma 10 as in Subsection 5.2.
2. We then compute M ⊙ , M ∗ , L⊙ , and L∗ as in Subsection 5.3.
3. Finally, we compute and return the state-set transition by computing
[
[
⊙
∗
D⊙ =
δ[l,r]
(u, α)
and
D∗ =
δ[l,r]
(u, α).
(l,r,u)∈L∗

(l,r,u)∈L⊙

19

<!-- page 21 -->

by processing L⊙ and L∗ from left-to-right using the procedure from Subsection 5.4 that
⊙
∗ (u, α) in sorted order. Since both lists L⊙ and L∗ are sorted the
computes δ[l,r]
(u, α) and δ[l,r]
lists D⊙ and D∗ are also sorted. We merge these two lists to get the final output.
Analysis of the algorithm Step 1 uses O(|P |) time by Lemma 10, and Step 2 uses O(|δ(P, α)| +
|P | log log m) time by Lemma 18. By Lemma 19 the time to compute all internal transitions in
step 3 is


X
X
⊙
∗
O
(1 + |δ[l,r]
(u, α)|) +
(1 + |δ[l,r]
(u, α)|)
(2)
(l,r,u)∈L∗

(l,r,u)∈L⊙

The length of list L⊙ is O(|M ⊙ |) as each interval endpoint is due to meeting a node in M ⊙ for the
first or last time in the traversal. Similarly, |L∗ | = O(|M ∗ |). Hence, (2) is O(|M ⊙ |+|M ∗ |+|δ(P, α)|).
The time to merge the two lists is linear in the total size of the lists since the lists are sorted. Thus
the time for step 3 is O(|M ⊙ | + |M ∗ | + |δ(P, α)|). Plugging in the bounds from Lemma 12 and 13
we get a total running time of O(|δ(P, α)| + |P | log log m).
Correctness We have already proved that N ⊙ (P, α) ⊆ M ⊙ and N ∗ (P, α) ⊆ M ∗ . Thus, by
Lemma 6 the set of positions computed in step 4 is δ(P, α). There are at most |P | nodes in
M ⊙ \ N ⊙ (P, α). These nodes are all in lastextent(P ) and thus the set of internal transitions on
these will all be in δ(P, α). Similarly, for the nodes in M ∗ \ N ∗ (P, α).
to show that the partition into lists is correct. We show that a position q ∈
S It remains
⊙ (v, α) is covered by the lowest node in M ⊙ such that q ∈ δ ⊙ (u, α), i.e., where covered
δ
⊙
v∈M
⊙
means that q ∈ δ[l,r]
(u, α) for some l, r such that (l, r, u) ∈ L⊙ . This follows easily from the properties
of the inorder traversal. The arguments for q ∈ ∪v∈M ∗ δ ∗ (v, α) are similar. For completeness, the
full proof is shown below.
Lemma 20. We have
[
δ ⊙ (v, α) =
v∈M ⊙

[

⊙
δ[l,r]
(u, α)

[

and

δ ∗ (v, α)

v∈M ∗

(l,r,u)∈L⊙

=

[

∗
δ[l,r]
(u, α) .

(l,r,u)∈L∗

Proof. We split the proof into two cases.
S
S
⊙
Case 1: v∈M ⊙ δ ⊙ (v, α) = (l,r,u)∈L⊙ δ[l,r]
(u, α).
We are only adding subranges of the range
⊙
⊙
⊙
of right(v) for any node in v ∈ M to L . This immediately implies that ∪(l,r,u)∈L⊙ δ[l,r]
(u, α) ⊆
⊙
∪v∈M ⊙ δ (v, α).
For the other direction, let q be a position in ∪v∈M ⊙ δ ⊙ (v, α) and let u be the deepest node in
⊙
⊙
M such that q ∈ δ ⊙ (u, α). We will show that q ∈ δ[l,r]
(u, α) for some l, r such that (l, r, u) ∈ L⊙ .
Let pred(u) = arg maxx∈M ⊙ {rx < q} and succ(u) = arg minx∈M ⊙ {lx > q}. If neither pred(u)
nor succ(u) are in T (right(u)) then nothing happens with ℓ and the stack S after the step where
we add u to the top of the stack until we leave u the last time. At this point top(S) = v and
ℓ = lu and (ℓ, ru , u) = (lu , ru , u) is appended to L⊙ . If pred(u) ∈ T (right(u)) then when we leave
pred(u), node u will be on top of the stack and ℓ = rpred(u) + 1. If also succ(u) ∈ T (right(u)) then
succ(u) is the next node from M ⊙ we process in our inorder traversal. When we meet succ(u)
before traversing its right subtree we add (ℓ, lsucc(u) − 1, top(S)) = (rpred(u) + 1, lsucc(u) − 1, u) to L⊙ .
⊙
Since rpred(u) + 1 ≤ q ≤ lsucc(u) − 1 we have q ∈ δ[r
+1,l
+1] (u, α). If succ(u) ̸∈ T (right(u))
pred(u)

succ(u)

then the next change we perform is when leaving u. Here we add (ℓ, ru − 1, u) to L⊙ . If only
20

<!-- page 22 -->

succ(u) ∈ T (right(u)) then we have top(S) = u and ℓ = lu when we process succ(u) the first time
(after visiting its left subtree). Then we add (lu , lsucc(u) − 1, u) to L⊙ .
S
S
∗ (u, α).
Case 2: v∈M ∗ δ ∗ (v, α) = (l,r,u)∈L∗ δ[l,r]
The arguments are similar to case 1. We are
only adding subranges of the range of v for any node in v ∈ M ∗ to L∗ . This immediately implies
∗ (u, α) ⊆ ∪
∗
that ∪(l,r,u)∈L∗ δ[l,r]
v∈M ∗ δ (v, α).
S
For the other direction let q be a position in v∈M ∗ δ ∗ (v, α) and let u be the deepest node in
∗ (u, α) for some l, r such that (l, r, u) ∈ L∗ .
M ∗ such that q ∈ δ ∗ (u, α). We will show that q ∈ δ[l,r]
If there are no other nodes from M ∗ than u in T (u) then nothing happens between the first
and last time we meet u in the traversal and (lu , ru , u) is appended to L⊙ when we meet u the last
time. Similarly to case 1, let upred(u) = arg maxx∈M ∗ {rx < q} and succ(u) = arg minx∈M ∗ {lx > q}.
If pred(u) ∈ T (u) then when we leave pred(u), node u will be on top of the stack and ℓ = rpred(u) + 1.
If also succ(u) ∈ T (u) then succ(u) is the next node from M ∗ we process in our traversal. When we
meet succ(u) the first time we add (ℓ, lsucc(u) − 1, top(S)) = (rpred(u) + 1, lsucc(u) − 1, u) to L∗ . Since
∗
rpred(u) + 1 ≤ q ≤ lsucc(u) − 1 we have q ∈ δ[r
(u, α). If succ(u) ̸∈ T (u) then the next
pred(u) +1,lsucc(u) +1]
change we perform is when leaving u. Here we add (ℓ, ru − 1, u) to L∗ . If only succ(u) ∈ T (u) then
we have top(S) = u and ℓ = lu when we process succ(u) the first time (after visiting its left subtree).
Then we add (lu , lsucc(u) − 1, u) to L∗ .
In summary, we have the following result.
Lemma 21. Given a regular expression R of size m, we can build a data structure in O(m) space
and preprocessing time such that given any set of positions P in R and character α ∈ Σ, we can
compute δ(P, α) in O(|P | log log m + |δ(P, α)|) time.

6

Speeding Up State-Set Transitions

We now show how to improve the run time of computing a state-set transition δ(P, α) from
m
O(|P | log log m + |δ(P, α)|) to O(|P | log log |P
| + |δ(P, α)|) while still using linear space. Finally, we
show how to use this to obtain the main results of Theorems 1. We now require that the input
positions in P are sorted and the output positions in δ(P, α) are reported in sorted order.
First, observe that the log log m factor is from computing |P | predecessor queries and |P | first
label queries in steps 2 and 3 in the main algorithm in Section 5.5. The first label queries in turn
are reduced to computing O(|P |) predecessor queries on the Euler tour of R [25]. In both cases,
we need to answer a batch of b = Θ(|P |) predecessor queries on a set of size t = Θ(|Posα |) from a
universe of size u = Θ(m). The batch is provided in sorted order and the output should also be
sorted.
We use a simple two-level data structure as follows. We first partition the universe into t intervals
of size u/t (except possibly the last which may be smaller). For each interval, we store a predecessor
data structure over the subset of the elements in the interval using a reduced universe of size u/t.
Furthermore, we also store a pointer to the nearest non-empty smaller interval. Using the same
predecessor data structure as in Section 5.5 for each interval the total space is O(t). We answer a
batch of b predecessor queries according to the following two cases:
1. If b ≤ t we process each predecessor query in the batch by identifying the at most two intervals
containing the answer and then querying these predecessor data structures. In total, this uses
O(b log log(u/t)) = O(b log log(u/b)) time.
2. If b > t we simply merge the sorted batch of queries with the input set using O(b + t) = O(b)
time.
21

<!-- page 23 -->

Since the batch is sorted we can also return the output in sorted order in O(b) time. It follows that
the running time is bounded by O(b log log(u/b). Plugging into to the algorithm of Section 5.5, we
obtain a data structure that uses O(m) space and supports computing a state-set transition δ(P, α)
m
in time O(|P | log log |P
| + |δ(P, α)|). This completes the proof of Theorem 3.
Next consider Theorem 1. Let Q be a string of length n and let S0 , . . . , Sn be the state-sets in
the simulation of R on Q. We implement the state-set transitions using Theorem 3. Note that each
state-set transition produces the output in sorted order as required. Since logarithms are concave
we have that the total time for the state-set transitions is
!


n

X
m
∆
nm 
m
O
|Si | log log
= O (n + 1)
= O ∆ log log
log log
.
|Si |
n+1
∆/(n + 1)
∆
i=0

The algorithm uses O(m) space to store the representation of R and at most two state sets during
the simulation. This completes the proof of Theorem 1.

7

Conditional Lower Bound

We now prove the conditional lower bound of Theorem 2. Our lower bound follows the reduction of
Backurs and Indyk [7] from the orthogonal vectors problem (OVP) to regular expression matching.
The orthogonal vectors problems is defined as follows. Given two sets A, B ⊆ {0, 1}d such
that |A| = M , |B| = N , determine if there exists a ∈ A and b ∈ B such that a · b = 0. For any
M = Θ(N α ) for some α ∈ (0, 1] and any constant ϵ > 0, any algorithm for OVP with running time
O((M N )1−ϵ ) violates SETH for d = ω(log N ) [16, 76].
Backurs and Indyk [7] showed hardness of regular expression matching using a reduction from
OVP. Given an instance of OVP they show how to construct a regular expression R′ and a string
Q′ in O(N d) time such that Q′ matches R′ if and only if there exists a ∈ A and b ∈ B such that
a · b = 0. The reduction works in O(N d) time, the lengths of both R′ and Q′ is Θ(N d), and the
alphabet is {x, y}. The regular expression R′ has the form
 ′

 ′

|Q |
|Q |
K
K
R′ = 
(x∗ y ∗ ) · P · 
(x∗ y ∗ ) .
j=1

j=1

Here P is a regular expression of length O(M d) with the property that a substring of Q′ can be
derived from P if and only if there exists a ∈ A and b ∈ B such that a · b = 0. The precise definition
of R′ , Q′ , and P can be found in [7].
We claim that ∆R′ ,Q′ = Θ(|Q′ |2 ). To see this first note that ∆R′ ,Q′ is at most |R′ ||Q′ | + 1 =
O(|Q′ |2 ). For the lower bound, consider the sequence S0 , S1 , . . . , S|Q′ | of state sets in the NFA
simulation, and focus on the first 2|Q′ | positions in R′ , i.e., the positions corresponding to the
subexpression immediately before P . Since Q is a string of xs and ys, we have |S1 | = |Q′ | and
|Si | ≥ |Si−1 | − 1 . Thus, ∆R′ ,Q′ = Ω(|Q′ |2 ) and hence the claim follows. See Figure 7(a).
We can now prove the following theorem.
Theorem 22. Given A = {a1 , . . . , aN } ⊆ {0, 1}d and B = {b1 , . . . , bM } ⊆ {0, 1}d and a constant γ,
where 0 < γ ≤ 1, we can construct in O((N d)2/(1+γ) ) time a regular expression R and a string Q,
such that there exists a ∈ A and b ∈ B where a · b = 0 if and only if Q ∈ L(R). The size of R and
Q is Θ((N d)2/(1+γ) ) and ∆R,Q = Θ(|Q|1+γ ) = Θ(N 2 d2 ).

22

<!-- page 24 -->

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x
y
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
y

x

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

y
y

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

y

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
y

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

(a)

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

y

<latexit sha1_base64="ViTER4vnQkS23GscKnbkckEBaz0=">AAAB6nicbVBNS8NAEJ3Ur1q/qh69LBahXkpSBT0WvHisaGuhDWWznbRLN5uwuxFK6E/w4kERr/4ib/4bt20O2vpg4PHeDDPzgkRwbVz32ymsrW9sbhW3Szu7e/sH5cOjto5TxbDFYhGrTkA1Ci6xZbgR2EkU0igQ+BiMb2b+4xMqzWP5YCYJ+hEdSh5yRo2V7qv0vF+uuDV3DrJKvJxUIEezX/7qDWKWRigNE1Trrucmxs+oMpwJnJZ6qcaEsjEdYtdSSSPUfjY/dUrOrDIgYaxsSUPm6u+JjEZaT6LAdkbUjPSyNxP/87qpCa/9jMskNSjZYlGYCmJiMvubDLhCZsTEEsoUt7cSNqKKMmPTKdkQvOWXV0m7XvMuau5dvdK4zOMowgmcQhU8uIIG3EITWsBgCM/wCm+OcF6cd+dj0Vpw8plj+APn8weEf408</latexit>

y

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x
y
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

x

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

y
y

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

y

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

x
y

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

<latexit sha1_base64="4PISKDMmgcpXrypWRGfg8p7aH4k=">AAAB6nicbVDLSgNBEOyNrxhfUY9eBoMQL2FX4uMY8OIxonlAsoTZSW8yZHZ2mZkVQsgnePGgiFe/yJt/4yTZgyYWNBRV3XR3BYng2rjut5NbW9/Y3MpvF3Z29/YPiodHTR2nimGDxSJW7YBqFFxiw3AjsJ0opFEgsBWMbmd+6wmV5rF8NOME/YgOJA85o8ZKD+XgvFcsuRV3DrJKvIyUIEO9V/zq9mOWRigNE1Trjucmxp9QZTgTOC10U40JZSM6wI6lkkao/cn81Ck5s0qfhLGyJQ2Zq78nJjTSehwFtjOiZqiXvZn4n9dJTXjjT7hMUoOSLRaFqSAmJrO/SZ8rZEaMLaFMcXsrYUOqKDM2nYINwVt+eZU0LyreVeXyvlqqVbM48nACp1AGD66hBndQhwYwGMAzvMKbI5wX5935WLTmnGzmGP7A+fwBiTqNRw==</latexit>

(b)

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

x
<latexit sha1_base64="NjmD1SpQfZ1ao6v7lsZQG0qhV2Q=">AAAB6HicbVDLTgJBEOzFF+IL9ehlIjHxRHaJryOJF4+QyCOBDZkdemFkdnYzM2skhC/w4kFjvPpJ3vwbB9iDgpV0UqnqTndXkAiujet+O7m19Y3Nrfx2YWd3b/+geHjU1HGqGDZYLGLVDqhGwSU2DDcC24lCGgUCW8Hodua3HlFpHst7M07Qj+hA8pAzaqxUf+oVS27ZnYOsEi8jJchQ6xW/uv2YpRFKwwTVuuO5ifEnVBnOBE4L3VRjQtmIDrBjqaQRan8yP3RKzqzSJ2GsbElD5urviQmNtB5Hge2MqBnqZW8m/ud1UhPe+BMuk9SgZItFYSqIicnsa9LnCpkRY0soU9zeStiQKsqMzaZgQ/CWX14lzUrZuypf1i9K1UoWRx5O4BTOwYNrqMId1KABDBCe4RXenAfnxXl3PhatOSebOYY/cD5/AOSpjPY=</latexit>

Figure 7: (a) The position automaton for x∗ y ∗ x∗ y ∗ x∗ y ∗ .
x8 x∗ y ∗ x∗ y ∗ x∗ y ∗ .

y
<latexit sha1_base64="hM8m80kDov7ptxZemnnr1DabRZY=">AAAB6HicbVBNS8NAEJ3Ur1q/qh69LBbBU0mKX8eCF48t2FpoQ9lsJ+3azSbsboQS+gu8eFDEqz/Jm//GbZuDtj4YeLw3w8y8IBFcG9f9dgpr6xubW8Xt0s7u3v5B+fCoreNUMWyxWMSqE1CNgktsGW4EdhKFNAoEPgTj25n/8IRK81jem0mCfkSHkoecUWOl5qRfrrhVdw6ySrycVCBHo1/+6g1ilkYoDRNU667nJsbPqDKcCZyWeqnGhLIxHWLXUkkj1H42P3RKzqwyIGGsbElD5urviYxGWk+iwHZG1Iz0sjcT//O6qQlv/IzLJDUo2WJRmApiYjL7mgy4QmbExBLKFLe3EjaiijJjsynZELzll1dJu1b1rqqXzYtKvZbHUYQTOIVz8OAa6nAHDWgBA4RneIU359F5cd6dj0VrwclnjuEPnM8f5i2M9w==</latexit>

(b) The position automaton for

Proof. We construct our instance R, Q from R′ and Q′ as follows.
Q = x ℓ · Q′

and

R = xℓ · R ′

where ℓ = (N d)2/(1+γ) .
Clearly, Q matches R if and only if Q′ matches R′ . Furthermore, the NFA simulation on the
first ℓ characters must produce singleton state sets (see Figure 7(b)). Hence, we have that ∆R,Q =
ℓ + ∆R′ ,Q′ = Θ((N d)2 ). Since |Q| = ℓ + |Q′ | = Θ((N d)2/(1+γ) ) we have that ∆ = Θ(|Q|1+γ) ).
Theorem 2 follows directly from Theorem 22, since an O(∆1−ϵ ) = O((N d)2−2ϵ ) time algorithm
for regular expression matching would imply a O((N d)2−2ϵ + (N d)2/(1+γ) ) algorithm for OVP.

8

Acknowledgments

We thank the anonymous reviewers whose comments and suggestions significantly improved the
presentation of the paper.

References
[1] Amir Abboud and Karl Bringmann. Tighter connections between formula-sat and shaving logs.
In Proc. 45th ICALP, 2018.
[2] Alfred V. Aho, Ravi Sethi, and Jeffrey D. Ullman. Compilers: principles, techniques, and tools.
Addison-Wesley Longman Publishing Co., Inc., 1986.
[3] Alfred V Aho and Jeffrey D Ullman. The theory of parsing, translation, and compiling, volume 1.
Prentice-Hall Englewood Cliffs, 1973.
[4] Cyril Allauzen and Mehryar Mohri. A unified construction of the Glushkov, Follow, and
Antimirov automata. In Proc. 36th MFCS, pages 110–121, 2006.

23

<!-- page 25 -->

[5] Valentin M. Antimirov. Partial derivatives of regular expressions and finite automaton constructions. Theor. Comput. Sci., 155(2):291–319, 1996.
[6] Alberto Apostolico and Concettina Guerra. The longest common subsequence problem revisited.
Algorithmica, 2(1-4):315–336, 1987.
[7] Arturs Backurs and Piotr Indyk. Which regular expression patterns are hard to match? In
Proc. 57th FOCS, pages 457–466, 2016.
[8] Djamal Belazzougui and Mathieu Raffinot. Approximate regular expression matching with
multi-strings. In Proc. 18th SPIRE, pages 55–66, 2011.
[9] Michael A. Bender and Martin Farach-Colton. The LCA problem revisited. In Proc. 4th
LATIN, pages 88–94, 2000.
[10] Philip Bille. New algorithms for regular expression matching. In Proc. of the 33rd ICALP,
pages 643–654, 2006.
[11] Philip Bille and Martin Farach-Colton. Fast and compact regular expression matching. Theor.
Comput. Sci., 409(3):486–496, 2008.
[12] Philip Bille and Mikkel Thorup. Faster regular expression matching. In Proc. 36th ICALP,
pages 171–182, 2009.
[13] Philip Bille and Mikkel Thorup. Regular expression matching with multi-strings and intervals.
In Proc. 21st SODA, pages 1297–1308, 2010.
[14] T Bray, J Paoli, CM Sperberg-McQueen, Y Mailer, and F Yergeau. Extensible markup language
(XML) 1.0 5th edition. Technical report, W3C, 2008.
[15] Karl Bringmann, Allan Grønlund, and Kasper Green Larsen. A dichotomy for regular expression
membership testing. In Proc. 58th FOCS, pages 307–318, 2017.
[16] Karl Bringmann and Marvin Künnemann. Quadratic conditional lower bounds for string
problems and dynamic time warping. In 2015 IEEE 56th Annual Symposium on Foundations
of Computer Science, pages 79–97. IEEE, 2015.
[17] Sabine Broda, Markus Holzer, Eva Maia, Nelma Moreira, and Rogério Reis. A mesh of automata.
Inform. and Comput., 265:94–111, 2019.
[18] Anne Brüggemann-Klein. Regular expressions into finite automata. Theoret. Comput. Sci.,
120(2):197–213, 1993.
[19] Anne Brüggemann-Klein and Derick Wood. One-unambiguous regular languages. Inform. and
Comput., 140(2):229–253, 1998.
[20] Janusz A Brzozowski. Derivatives of regular expressions. J. ACM, 11(4):481–494, 1964.
[21] Chia-Hsiang Chang and Robert Paige. From regular expressions to DFA’s using compressed
NFA’s. Theoret. Comput. Sci., 178(1-2):1–36, 1997.
[22] Nicola Cotumaccio, Giovanna D’Agostino, Alberto Policriti, and Nicola Prezza. Colexicographically ordering automata and regular languages - part I. J. ACM, 70(4), 2023.

24

<!-- page 26 -->

[23] Wojciech Czerwiński, Claire David, Katja Losemann, and Wim Martens. Deciding definability
by deterministic regular expressions. J. Comput. System Sci., 88:75–89, 2017.
[24] Peter J Denning, Jack B Dennis, and Joseph E Qualitz. Machines, languages, and computation.
Prentice Hall, 1978.
[25] P. F. Dietz. Fully persistent arrays. In Proc. 1st WADS, pages 67–74, 1989.
[26] Bartlomiej Dudek, Pawel Gawrychowski, Garance Gourdel, and Tatiana Starikovskaya. Streaming regular expression membership and pattern matching. In Proc. 33rd SODA, pages 670–694,
2022.
[27] David Eppstein, Zvi Galil, Raffaele Giancarlo, and Giuseppe F. Italiano. Sparse dynamic
programming i: Linear cost functions. J. ACM, 39(3):519–545, 1992.
[28] David Eppstein, Zvi Galil, Raffaele Giancarlo, and Giuseppe F Italiano. Sparse dynamic
programming ii: convex and concave cost functions. J. ACM, 39(3):546–567, 1992.
[29] Z. Galil. Open problems in stringology. In A. Apostolico and Z. Galil, editors, Combinatorial
problems on words, NATO ASI Series, Vol. F12, pages 1–8. 1985.
[30] Shudi Sandy Gao, C Michael Sperberg-McQueen, and Henry Thompson. W3C XML schema
definition language (XSD) 1.1 part 1: Structures. Technical report, W3C, 2012.
[31] Minos N Garofalakis, Rajeev Rastogi, and Kyuseok Shim. SPIRIT: Sequential pattern mining
with regular expression constraints. In Proc. 25th VLDB, pages 223–234, 1999.
[32] Viliam Geffert. Translation of binary regular expressions into nondeterministic ε-free automata
with O(n log n) transitions. J. Comput. Syst. Sci., 66(3):451–472, 2003.
[33] Dora Giammarresi, Jean-Luc Ponty, and Derick Wood. Gluskov and Thompson constructions:
a synthesis. Technical report, 1998. http://www.cs.ust.hk/tcsc/RR/1998-11.ps.gz.
[34] Victor M. Glushkov. The abstract theory of automata. Russian Math. Surveys, 16(5):1–53,
1961.
[35] Viktor M Glushkov. On a synthesis algorithm for abstract automata. Ukr. Matem. Zhurnal,
12(2):147–156, 1960.
[36] Benoı̂t Groz and Sebastian Maneth. Efficient testing and matching of deterministic regular
expressions. J. Comput. Syst. Sci., 89:372–399, 2017.
[37] Torben Hagerup, Peter Bro Miltersen, and Rasmus Pagh. Deterministic dictionaries. J.
Algorithms, 41(1):69–85, 2001.
[38] D. Harel and R. E. Tarjan. Fast algorithms for finding nearest common ancestors. SIAM J.
Comput., 13(2):338–355, 1984.
[39] Juraj Hromkovič, Sebastian Seibert, Juhani Karhumäki, Hartmut Klauck, and Georg Schnitger. Communication complexity method for measuring nondeterminism in finite automata.
Information and Computation, 172(2):202–217, 2002.
[40] Juraj Hromkovič, Sebastian Seibert, and Thomas Wilke. Translating regular expressions into
small ε-free nondeterministic finite automata. J. Comput. Syst. Sci., 62(4):565–588, 2001.
25

<!-- page 27 -->

[41] James W Hunt and Thomas G Szymanski. A fast algorithm for computing longest common
subsequences. Commun. ACM, 20(5):350–353, 1977.
[42] Lucian Ilie and Sheng Yu. Follow automata. Inform. and Comput., 186(1):140 – 162, 2003.
[43] Russell Impagliazzo and Ramamohan Paturi. On the complexity of k-SAT. J. Comput. Syst.
Sci., 62(2):367–375, 2001.
[44] Theodore Johnson, S. Muthukrishnan, and Irina Rozenbaum. Monitoring regular expressions
on out-of-order streams. In Proc. 23nd ICDE, pages 1315–1319, 2007.
[45] Chris Keeler and Kai Salomaa. Branching measures and nearly acyclic nfas. International
Journal of Foundations of Computer Science, 30(06n07):1135–1155, 2019.
[46] Chris Keeler and Kai Salomaa. Structural properties of nfas and growth rates of nondeterminism
measures. Information and Computation, 284:104690, 2022.
[47] Kenrick Kin, Björn Hartmann, Tony DeRose, and Maneesh Agrawala. Proton: multitouch
gestures as regular expressions. In Proc. SIGCHI, pages 2885–2894, 2012.
[48] S. C. Kleene. Representation of events in nerve nets and finite automata. In C. E. Shannon
and J. McCarthy, editors, Automata Studies, Ann. Math. Stud. No. 34, pages 3–41. Princeton
U. Press, 1956.
[49] Sailesh Kumar, Sarang Dharmapurikar, Fang Yu, Patrick Crowley, and Jonathan Turner.
Algorithms to accelerate multiple regular expressions matching for deep packet inspection. In
Proc. SIGCOMM, pages 339–350, 2006.
[50] Denis Kuperberg and Anirban Majumdar. Computing the width of non-deterministic automata.
Logical Methods in Computer Science, 15, 2019.
[51] Harry R. Lewis and Christos H. Papadimitriou. Elements of the Theory of Computation.
Prentice-Hall, 1981.
[52] Quanzhong Li and Bongki Moon. Indexing and querying XML data for regular path expressions.
In Proc. 27th VLDB, pages 361–370, 2001.
[53] Christof Löding and Stefan Repke. Decidability results on the existence of lookahead delegators
for nfa. In IARCS Annual Conference on Foundations of Software Technology and Theoretical
Computer Science (FSTTCS 2013). Schloss Dagstuhl-Leibniz-Zentrum fuer Informatik, 2013.
[54] Ping Lu, Joachim Bremer, and Haiming Chen. Deciding determinism of regular languages.
Theory Comput. Syst., 57:97–139, 2015.
[55] Wim Martens, Frank Neven, and Thomas Schwentick. Complexity of decision problems for
xml schemas and chain regular expressions. SIAM J. Comput., 39(4):1486–1530, 2010.
[56] John C Martin. Introduction to Languages and the Theory of Computation. McGraw-Hill, 1991.
[57] R. McNaughton and H. Yamada. Regular expressions and state graphs for automata. IRE
Trans. on Electronic Computers, 9(1):39–47, 1960.
[58] K. Mehlhorn and S. Nähler. Bounded ordered dictionaries in O(log log N ) time and O(n) space.
Inform. Process. Lett., 35(4):183–189, 1990.
26

<!-- page 28 -->

[59] Boris G Mirkin. An algorithm for constructing a base in a language of regular expressions.
Engineering Cybernetics, 5:51–57, 1966.
[60] Makoto Murata. Extended path expressions of XML. In Proc. 20th PODS, pages 126–137,
2001.
[61] Makoto Murata, Dongwon Lee, Murali Mani, and Kohsuke Kawaguchi. Taxonomy of xml
schema languages using formal language theory. ACM Trans. Internet Tech., 5(4):660–704,
2005.
[62] E. W. Myers. A four-russian algorithm for regular expression pattern matching. J. ACM,
39(2):430–448, 1992.
[63] Gonzalo Navarro and Mathieu Raffinot. Fast and simple character classes and bounded gaps
pattern matching, with applications to protein searching. J. Comp. Biology, 10(6):903–923,
2003.
[64] Abhinav Nellore, Austin Nguyen, and Reid F. Thompson. An invertible transform for efficient
string matching in labeled digraphs. In Proc. 32nd CPM, volume 191 of LIPIcs, pages 20:1–20:14.
Schloss Dagstuhl - Leibniz-Zentrum für Informatik, 2021.
[65] J-L Ponty, Djelloul Ziadi, and J-M Champarnaud. A new quadratic algorithm to convert a
regular expression into an automaton. In Proc. 1st WIA, pages 109–119. Springer, 1996.
[66] Nicola Rizzo, Alexandru I. Tomescu, and Alberto Policriti. Solving string problems on graphs
using the labeled direct product. Algorithmica, 84(10):3008–3033, 2022.
[67] Philipp Schepper. Fine-grained complexity of regular expression pattern matching and membership. In Proc. 28th ESA, 2020.
[68] Georg Schnitger. Regular expressions and NFAs without ε-transitions. In Proc. 23rd STACS,
pages 432–443, 2006.
[69] Seppo Sippu and Eljas Soisalon-Soininen. Parsing Theory: Volume I Languages and Parsing.
Springer, 1988.
[70] K. Thompson. Regular expression search algorithm. Commun. ACM, 11:419–422, 1968.
[71] Mikkel Thorup. Space efficient dynamic stabbing with fast queries. In Proc. 33rd STOC, pages
649–658, 2003.
[72] Larry Wall. The Perl Programming Language. Prentice Hall Software Series, 1994.
[73] W John Wilbur and David J Lipman. Rapid similarity searches of nucleic acid and protein
data banks. Proc. Nat. Acad. Sci., 80(3):726–730, 1983.
[74] W John Wilbur and David J Lipman. The context dependent comparison of biological sequences.
SIAM J. Appl. Math., 44(3):557–567, 1984.
[75] Dan E Willard. Log-logarithmic worst-case range queries are possible in space Θ(N ). Inform.
Process. Lett., 17(2):81–84, 1983.
[76] Ryan Williams. A new algorithm for optimal 2-constraint satisfaction and its implications.
Theoretical Computer Science, 348(2-3):357–365, 2005.
27

<!-- page 29 -->

[77] Derrick Wood. Theory of Computation: A Primer. Addison-Wesley, 1987.
[78] Fang Yu, Zhifeng Chen, Yanlei Diao, T. V. Lakshman, and Randy H. Katz. Fast and memoryefficient regular expression matching for deep packet inspection. In Proc. ANCS, pages 93–102,
2006.

28
