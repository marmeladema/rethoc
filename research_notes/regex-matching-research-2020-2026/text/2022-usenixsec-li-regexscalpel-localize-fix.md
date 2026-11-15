---
file: ../pdfs/2022-usenixsec-li-regexscalpel-localize-fix.pdf
citation: "RegexScalpel: ReDoS Defense by Localize-and-Fix. Li et al. USENIX Security 2022"
doi: 
source: "https://www.usenix.org/system/files/sec22-li-yeting.pdf"
topics: [redos_backrefs_verification]
pages: 19
extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)
---
<!-- page 1 -->

RegexScalpel: Regular Expression Denial of
Service (ReDoS) Defense by Localize-and-Fix
Yeting Li, CAS-KLONAT, Institute of Information Engineering, Chinese Academy of Sciences;
University of Chinese Academy of Sciences; SKLCS, Institute of Software, Chinese Academy of
Sciences; Yecheng Sun, SKLCS, Institute of Software, Chinese Academy of Sciences; University of
Chinese Academy of Sciences; Zhiwu Xu, College of Computer Science and Software Engineering,
Shenzhen University; Jialun Cao, The Hong Kong University of Science and Technology; Yuekang
Li, School of Computer Science and Engineering, Nanyang Technological University; Rongchen
Li, SKLCS, Institute of Software, Chinese Academy of Sciences; University of Chinese Academy of
Sciences; Haiming Chen, SKLCS, Institute of Software, Chinese Academy of Sciences; CAS-KLONAT,
Institute of Information Engineering, Chinese Academy of Sciences; Shing-Chi Cheung, The Hong
Kong University of Science and Technology; Yang Liu, School of Computer Science and Engineering,
Nanyang Technological University; Yang Xiao, CAS-KLONAT, Institute of Information Engineering,
Chinese Academy of Sciences; University of Chinese Academy of Sciences
https://www.usenix.org/conference/usenixsecurity22/presentation/li-yeting

This paper is included in the Proceedings of the
31st USENIX Security Symposium.
August 10–12, 2022 • Boston, MA, USA
978-1-939133-31-1

Open access to the Proceedings of the
31st USENIX Security Symposium is
sponsored by USENIX.

<!-- page 2 -->

RegexScalpel: Regular Expression Denial of Service (ReDoS) Defense by
Localize-and-Fix
Yeting Li†§¶ Yecheng Sun¶§ Zhiwu Xu♮ Jialun Cao‡ Yuekang Li♯
Rongchen Li¶§ Haiming Chen¶†
Shing-Chi Cheung‡ Yang Liu♯ Yang Xiao†§
† CAS-KLONAT, Institute of Information Engineering, Chinese Academy of Sciences, Beijing, China
§ University of Chinese Academy of Sciences, Beijing, China
¶ SKLCS, Institute of Software, Chinese Academy of Sciences, Beijing, China
♮ College of Computer Science and Software Engineering, Shenzhen University, Shenzhen, China
‡ The Hong Kong University of Science and Technology, Hong Kong, China
♯ School of Computer Science and Engineering, Nanyang Technological University, Singapore
Abstract
The Regular expression Denial of Service (ReDoS) is a
class of denial of service attacks that exploit vulnerable regular expressions (regexes) whose execution time can be superlinearly related to input sizes. A common approach of defending ReDoS attacks is to repair the vulnerable regexes. Techniques have been recently proposed to synthesize repaired
regexes using program-by-example (PBE) techniques. However, these existing techniques may generate regexes, which
are not semantically equivalent or similar to the original ones,
or are still vulnerable to ReDoS attacks.
To address the challenges, we propose RegexScalpel, an
automatic regex repair framework that adopts a localize-andfix strategy. RegexScalpel first localizes the vulnerabilities
by leveraging fine-grained vulnerability patterns proposed by
us to analyze their vulnerable patterns, the source (i.e., the
pathological sub-regexes), and the root causes (e.g., the overlapping sub-regexes). Then, RegexScalpel targets to fix the
pathological sub-regexes according to our predefined repair
patterns and the localized vulnerability information. Furthermore, our repair patterns ensure that the repair regexes are
semantically either equivalent to or similar to the original
ones. Our iterative repair method also keeps out vulnerabilities of the repaired regexes. With an experiment on a total number of 448 vulnerable regexes, we demonstrate that
RegexScalpel can outperform all existing automatic regexes
fixing techniques by fixing 348 more regexes than the best
existing work. Also, we adopted RegexScalpel to detect ten
popular projects including Python and NLTK, and revealed 16
vulnerable regexes. We then applied RegexScalpel to successfully repair all of them, and these repairs were merged into
the later release by the maintainers, resulting in 8 confirmed
CVEs.

1

Introduction

Regular expressions (regexes) are widely used in many computer science fields such as programming languages, string

USENIX Association

processing, database query, etc [1,6,11,12,18,33]. Attacks targeting regexes at online services are common [11, 18, 24, 34].
These attacks are often launched by submitting a malicious
input string to an online server and triggering the corresponding regex that requires a matching time of super-linear (i.e.,
polynomial or exponential) worst-case with respect to the input length, leading to retarding responses from the server. For
example, millions of users were affected when Cloudflare [3]
went down for 27 minutes in 2019. Such attacks are known
as Regular expression Denial of Service (ReDoS) attacks. According to recent statistics [11, 18], more than 10% of regexes
used in software projects are vulnerable to ReDoS attacks.
There are three necessary conditions for successful ReDoS
attacks: (i) a slow regex engine, (ii) an attacker-controlled
input string and (iii) a vulnerable regex [11]. Practitioners
have deployed different defense strategies to break one or
more necessary conditions. To break the first condition, one
strategy is to substitute a faster regex engine for a slower one
(i.e., regex engine substitution). This helps to improve the
worst-case complexity of regexes by using alternative matching algorithms. However, such improvement is derived at the
cost of omitting extended features (e.g., backreferences and
lookarounds) [16] or space [13]. Besides, engine substitution
can bring semantic differences [12] or incompatibilities [13].
A strategy to break the second condition is to limit the length
of the input string (i.e., input length restriction). But this strategy faces a dilemma known as “Goldilocks problem” [14],
i.e., either the limited length is too short to accept all valid
inputs, or the length is too long to reject malicious inputs. Furthermore, according to our experiment (see Sec. 5), the above
defense methods ignore an important and universal vulnerable
pattern, known as SLQ (See Sec. 4.1.4 or prior work [19]),
which appears in 75.89% (340 / 448) of the benchmarks. As
a result, these defense methods can only fix 13.82% (47 /
340) of the benchmarks, leaving 86.18% of them vulnerable
to ReDoS attacks.
On searching for better defense strategies, practitioners
find that repairing vulnerable regexes (i.e., regex repair) can
greatly mitigate their vulnerabilities by breaking the third

31st USENIX Security Symposium

4183

<!-- page 3 -->

condition. Our statistics in Table 13 show that regex repair
is the most common defense strategy adopted by developers or maintainers, accounting for as high as 92.19%. However, regex repair is challenging. It is non-trivial to propose a
semantic-equivalent or semantic-similar regex to substitute
the vulnerable one, even for human experts. In addition, the
proposed regexes can vary significantly across experts and
newbies. In other words, manual regex repair is neither practical nor scalable. Moreover, there could be multiple vulnerabilities in a vulnerable regex. According to our investigation,
44.20% (198 / 448) vulnerable regexes have more than one
vulnerability. Similar statistics were also observed by a recent
work [19] published in 2021. The existence of multiple vulnerabilities in a vulnerable regex exacerbates the difficulty of
automatic regex repair.
Current state-of-the-art techniques [7, 22] adopt
programming-by-example (PBE) approaches to automatically repair ReDoS-vulnerable regexes. A vulnerable
regex is repaired by synthesizing another regex that accepts
the given positive examples (i.e., valid input strings) and
rejects the given negative examples (e.g., ReDoS attacks).
However, there are two drawbacks of the PBE approaches.
First, a synthesized regex may not be semantically
equivalent or similar to its original regex with respect to
all valid input strings. The effectiveness of synthesis is
limited by the insufficiency of high-quality examples [20, 25].
Insufficient or biased examples may result in the over-fitting
or under-fitting of the synthesized regexes, leading to
incorrect repairs. Second, the repaired regex may be still
vulnerable to ReDoS attacks. These techniques ignore
the SLQ vulnerable pattern, which leads to the incapability of repairing such vulnerable regexes. For example,
Chida and Terauchi [7] return a repaired regex <span
([^".1-8B-Y\[\\\]^b-dfh-y]*)font-style:italic
for the
vulnerable
regex
<span
([^>]*)>
[^>]*font-style:italic[^>]*>.
The
sub-regex
([^".1-8B-Y\[\\\]^b-dfh-y]*) in the repaired regex
is semantically very different from the pathological subregex [^>]* in the original one. As a result, the string
'<span class="replace-with italic">' is matched
by the original regex but not by the repaired one. In addition,
the repaired regex contains an SLQ pattern, which can
be attacked by some malicious attack strings, such as
'<spannfont-style:italic'.repeat(10000).
To overcome these challenges, we propose RegexScalpel,
a regex ReDoS vulnerability analysis and repair framework
based on localize-and-fix. RegexScalpel first leverages the
fine-grained vulnerability patterns proposed in this paper,
which enables analyzing the information necessary for the
repair, to localize the vulnerabilities, including their vulnerable patterns, the source (i.e., the pathological sub-regexes),
and the root causes (e.g., the overlapping sub-regexes). Then,
RegexScalpel aims at fixing the pathological sub-regexes according to the predefined repair patterns as well as the local-

4184

31st USENIX Security Symposium

ized vulnerability information. By removing the overlapping
paths or reducing the maximum times of backtracking using
micro-manipulations (e.g., adding a lookaround, deleting a
quantifier or sub-regex, modifying a quantifier or sub-regex),
the localized pathological sub-regexes are fixed and the ReDoS attack is thus defended. Furthermore, our repair patterns
ensure that the repair regexes are semantically either equivalent to or similar as the original ones, and support comprehensive types of vulnerable causes including SLQ . Our iterative
repair method also keeps out vulnerabilities of the repaired
regexes.
Our experiment shows the remarkable effectiveness of
RegexScalpel. Among the 448 vulnerable regexes collected
from practice, RegexScalpel can repair 443 (98.88%) of them.
In contrast, the top performer of existing methods can only
synthesize ReDoS-safe regexes for 95 (21.20%) of them.
The experiment also reveals the limitation of manual repair – the maintainers may ignore repairing some vulnerabilities if there are many in one vulnerable regex. As a
comparison, RegexScalpel repaired all vulnerabilities without
manual effort and domain knowledge. Finally, we also applied RegexScalpel to real-world popular projects (including
Python, NLTK, etc.) that are under repair. RegexScalpel successfully repaired 16 vulnerabilities, and the repaired regexes
were confirmed by the developers and merged into subsequent
releases.
In summary, this paper makes four major contributions:
• Novelty. We develop RegexScalpel, which is a ReDoS
vulnerability analysis and repair framework. To the best
of our knowledge, RegexScalpel is the first approach to
localize and fix the vulnerable regexes considering comprehensive and fine-grained types of vulnerable causes.
• Semantics Preservation. We design a repair algorithm
to preserve the semantics of the original regex. In particular, the language of the repaired regex is a subset of the
language of the original one.
• Effectiveness. The evaluation exhibits the remarkable
effectiveness of RegexScalpel. It achieves 98.88% successful repair ratio, compared with 21.20% achieved by
the best existing work.
• Usefulness. We also employed RegexScalpel to help repair 16 vulnerable regexes across ten projects including
Python and NLTK. The repaired results were merged
into their later release.

2

Background

Before presenting RegexScalpel, we introduce the background of Regexes and ReDoS. Let Σ be a finite alphabet.
The set of all words over Σ is denoted by Σ∗ . The empty word
/ respectively. Let
and the empty set are denoted by ε and 0,
N = {0, 1, 2, . . .}.

USENIX Association

<!-- page 4 -->

2.1

Regular Expressions (Regexes)

A regex r over Σ is a well-formed parenthesized formula using
capturing group, non-capturing group, named capturing group,
backreference, greedy quantifier, lazy quantifier, lookarounds,
and anchors, etc. Here, the regexes r?, r*, r+ and r{i} where
i ∈ N are abbreviations of r{0, 1}, r{0, ∞}, r{1, ∞} and r{i, i},
respectively. Besides, the regex r{m, ∞} is often simplified as
r{m, }.
Next, we explain the semantics of the regex constructs
informally with the aid of JavaScript examples. A capturing group (r) allows getting a part of the match as a separate item. For example, "aab!".match(/a+(b+)/) returns
["aab", "b"], where "aab" is the match of the whole regex
and "b" is the match of the capturing group (b+)1 . If we
do not want a group to capture its match, we can optimize
this group into a non-capturing group (?:r). For example,
the matching result of /a+(?:b+)/ is the same to /a+b+/.
Moreover, we can access the result array by index. For example, "aab!".match(/a+(b+)/)[1] returns "b". Some regex
engines (e.g., ES2018) also support a more convenient alternative to access capturing groups by names (i.e., named capturing groups). The named capturing groups are available in the
property groups (e.g., groups.day, as seen in the following
example).
let regex = /(?<month>[0-9]{2})-(?<day>[0-9]{2})/;
let str = "10-16";
let groups = str.match(regex).groups;
console.log(groups.day); // 16

A greedy quantifier is repeated as many times as possible
while a lazy quantifier is repeated as few times as possible.
For example, "123.456".match(/\d+\.\d+?/) will return
the array ["123.4"].
A backreference \i means “to match the same text as in the
i-th capturing group”. Similarly, to reference a named capturing group (?<name>r), we can use backreference \k<name>.
An example is shown in the following listing.
let str = "'foo'";
let r1 = /(['"])(.*?)\1/;
console.log(str.match(r1)); // ["'foo'", "'", "foo"]
let r2 = /(?<quote>['"])(.*?)\k<quote>/;
console.log(str.match(r2)); // ["'foo'", "'", "foo"]

Lookarounds are zero-length assertions, and search for
strings that satisfy certain contexts. Specifically, it includes
lookahead and lookbehind, specifying the context after and before the searching strings, respectively. A positive lookahead
r1 (?=r2 ) looks for a string that matches r1 only when the
1 In JavaScript, without the global search flag “g”, the method match()
returns the first match as an array, wherein the first element is the match of
the whole regex (i.e., the implicit capturing group 0) and the remaining are
the matches of the capturing groups (if any exist) in order.

USENIX Association

string is followed by a string that matches r2 . Similarly, a negative lookahead r1 (?!r2 ) searches for a string matching r1
only when the string is not followed by a string that matches
r2 . On the other hand, a positive lookbehind (?<=r2 )r1 looks
for a string which matches r1 only when there is a string
which matches r2 before it. Similarly, a negative lookbehind
(?<!r2 )r1 looks for a string which matches r1 only when the
string before it does not match r2 . An example is shown in
the following listing.
let str = "123foo456foz";
console.log(str.match(/[0-9]{3}(?=foo)/)); // ["123"]
console.log(str.match(/[0-9]{3}(?!foo)/)); // ["456"]
console.log(str.match(/(?<=foo)[0-9]{3}/)); // ["456"]
console.log(str.match(/(?<!foo)[0-9]{3}/)); // ["123"]

Anchors specify the non-character context. Specifically, a
start-of-line anchor ^ denotes the start of a line, while an endof-line anchor $ denotes the end. A word boundary anchor
\b matches the position where one side is a word and the
other side is not a word. A non-word boundary anchor \B,
the negation of \b, matches at any position between two word
characters as well as at any position between two non-word
characters. An example is given in the following listing.
let str = "123 4567";
console.log(str.match(/^[0-9]{3}/)); // ["123"]
console.log(str.match(/[0-9]{3}$/)); // ["567"]
console.log(str.match(/[0-9]{3}\b/)); // ["123"]
console.log(str.match(/[0-9]{3}\B/)); // ["456"]

Alphabet of a Regex. The alphabet Σr of a regex r is the set
of all symbols that appear in r. For example, for the regex
r1 = (?=a)[ab]{1,3}, the alphabet Σr1 is {a, b}.
Language of a Regex. The language L (r) of a regex r is the
set of all words accepted by r. For the above regex r1 , the
language L (r1 ) is {a, aa, ab, aaa, aab, aba, abb}. Note that
regexes with backreferences are not context-free [4]. We take
inspiration from Li et al. [19] and use an over-approximate solution – calculate the language of i-th capturing group instead
of calculating the language of backreference \i.

2.2

Regex Denial of Service (ReDoS)

The ReDoS is a type of algorithmic complexity attack. In a ReDoS attack, an attacker exploits a combination of a problematically ambiguous regex and a malicious input string to trigger
the worst-case complexity of the regex engine, and thereby
prevents legitimate users from accessing online services. An
example is given below. Let us consider an ambiguous regex
r2 = /(a|a)+b/. An attacker can craft the malicious input
string s = a| .{z
. . a}, which is not matched by the regex r2 , to
k times

make a backtracking-based regex engine explore all 2k failing
paths.

31st USENIX Security Symposium

4185

<!-- page 5 -->

In this paper, we exploit the regex repair strategy to fix
ReDoS vulnerabilities.
Regex Repair. Given a ReDoS-vulnerable regex r and a set
of test cases S , we aim to repair the regex r to form a new
regex r′ such that (i) r′ passes all the given test cases S ; and
(ii) r′ is invulnerable to ReDoS attacks.
Semantics preservation. Semantics preservation after repair
in this paper means that the language of the repaired regex is
a subset of the language of the original one.
First Set. To fix ReDoS vulnerabilities, we introduce the
first(r) set of a regex r:

2

https://github.com/daaku/nodejs-tmpl

3 The capturing group and the lazy quantifier are ignored.

4186

31st USENIX Security Symposium

③ Repaired Regex

The first step of our approach is to leverage a vulnerability
analyzer to diagnose the vulnerability source of the above
ReDoS-vulnerable regex {([\s\S]+?)}. As shown in the
phase ❶ of Figure 2, the analyzer identifies that the pathological sub-regex r ={[\s\S]+ 3 is a (possible) vulnerability, due
to its two components r1 = { and r2 = [\s\S]+ overlap (i.e.,
L (r1 ) ∩ L (r2 ) = {“{”} ̸= /0), and returns a triple (SLQ 3 , r,
[r1, r2]), where SLQ 3 is a vulnerability pattern introduced in
§4.1.4. In order to facilitate fixing the vulnerability, we also
use r p and rq to represent the sub-regex of r1 and r2 without
the outermost quantifier, that is, { and [\s\S], respectively.

eg
ex
ne
R
ro
Do
Sp
Re
①

s
x
ple
ge
Re
oS
eD

Example 1. Take the ReDoS-vulnerable regex
{([\s\S]+?)} as an example. The regex aims to match the
simple string formatting using {}, as mentioned in the NPM
package nodejs-tmpl2 (6,858,130 weekly downloads).

am
Ex

In this section, we illustrate how our ReDoS-vulnerable regex
repair framework RegexScalpel works using a motivating
example.
The workflow consists of three key components, as shown
in Figure 1. The first component Analyzer (§4.1) takes a given
ReDoS-vulnerable regex as input, diagnoses the vulnerability
source, then returns the vulnerability information including
pattern (i.e., the vulnerability type), source (i.e., the pathological sub-regex), and cause (e.g., the overlapping sub-regexes).
The second component Repairer (§4.2) takes the vulnerability
information as input and then repairs the ReDoS-vulnerable
regex. The last component Verifier (§4.3) determines whether
the repaired regexes are ReDoS-invulnerable and whether it
can pass all the given test cases.

④

Overview

User

R
tiAn

3

Vulnerability
Repairer

⑤

Intuitively, first(r) represents the set of all symbols that may
appear in the first position of the strings accepted by the
regex r. Taking the above regex r1 = (?=a)[ab]{1,3} as an
example, first(r1 ) = {a}.

② Pattern, Source, Cause

first(r) = {a|au ∈ L (r), a ∈ Σ, u ∈ Σ∗ }

Vulnerability
Analyzer

Vulnerability
Verifier

Figure 1: An Overview of RegexScalpel for ReDoS Repair.
A Proof of Concept (PoC) example of this vulnerability is
shown in the following listing.
var tmpl = require("tmpl")
tmpl("{".repeat(50000)+"answer", { answer: "zxc" });

The second step of our approach is to repair the ReDoSvulnerable regex, according to the vulnerability information
obtained from the first step. The repaired regexes are also
called patches. The key idea for repairing is to avoid catastrophic backtracking during matching. Considering the above
vulnerability, we propose four possible repair patterns (see
§4.2.4 for more detail) to fix it, as given in the phase ❷ of Figure 2. Specifically, we can add a start-of-line anchor ^before
{ (i.e., the first repair pattern t1 in Figure 2), yielding a patch
γ1 = ^{([\s\S]+?)}. The start-of-line anchor ^ specifies
that the following pattern r (i.e., {([\s\S]+?)} ) must begin
at the first position of the string, which can effectively avoid
catastrophic backtracking (i.e., ReDoS). Similarly, we can
also generate a patch γ2 = {([\s\S]{1,500}?)} which substitutes the plus quantifier with a small quantifier nµ = 500
(i.e., the second pattern t2 in Figure 2). Obviously, the small
quantifier nµ = 500 can cut the number of backtracking.
Eliminating the overlaps is a natural solution to avoid
catastrophic backtracking. Therefore, we propose the third
repair pattern t3 with a negative lookahead (?!) in Figure 2
to make rq should not be matched on the strings matched

USENIX Association

<!-- page 6 -->

{ ( [\s\S]+ ?)}

ReDoS-vulnerable Regex

sub-regex r1

Repair Pattern

t1 :

r =⇒ ˆr

Repair Process

{ =⇒ ˆ{

Repaired Regex

γ1 =ˆ{([\s\S]+?)}

t2 :

rp : {
rq : [\s\S]

sub-regex r2

nq =⇒ nµ

t3 : r =⇒ (?! Φ(r ))r
q
p
q

t4 :

phase ❶

scs(rq ) = true, Σrq \ first(r p ) ̸= /0
rq =⇒ Θ(Σrq \ first(r p ))

[\s\S] =⇒ (?! Φ(r p ))[\s\S]

[\s\S] =⇒ [ˆ{]

γ2 ={([\s\S]{1,500}?)} γ3 ={((?:(?!{)[\s\S])+?)}

γ4 ={([ˆ{]+?)}

+ =⇒ {1,500}

phase ❷
Verification

%

!

!

!

phase ❸

Figure 2: An Example for Repairing the ReDoS-vulnerable Regex {([\s\S]+?)}.
by r p , where Φ(r p ) returns a regex obtained by transforming all the (named) capturing groups in the regex r p into
the non-capturing ones. By applying the pattern t3 , we can
generate a patch γ3 = {((?:(?!{)[\s\S])+?)}4 . The subregex (?! Φ(r p ))rq (i.e., (?!{)[\s\S]) ensures that rq (i.e.,
[\s\S]) will not match on { that can be matched by r p (i.e.,
{), that is, the two sub-regexes r p and (?! Φ(r p ))rq will not
overlap. So the patch γ3 can throw away the ReDoS vulnerability. Moreover, we can further focus on the first position of the strings matched by r p . And if rq is a character
class, we can eliminate this overlap by utilizing a negative
set, that is, the fourth pattern t4 in Figure 2, yielding a patch
γ4 = {([^{]+?)} with a character class not overlapping with
r p , where Θ(D ) transforms the set D into an (equivalent)
character class, and scs(rq ) = true denotes regex rq is a character class and scs(rq ) = f alse otherwise. Indeed, patches γ3
and γ4 are equivalent.
The final step of our approach is to determine whether these
patches are successful. As demonstrated in the phase ❸ of
Figure 2, the patches γ2 , γ3 and γ4 are invulnerable to ReDoS
attacks and pass the given test cases, so they are successful.
While for the patch γ1 , it is easy to verify that γ1 is invulnerable to ReDoS attacks, but γ1 fails at the maintainer-provided
test case below, because it does not match the non-leading
strings formatting using {}.
exports['basic name substitution'] = function() {
assert.equal(
tmpl('the answer is {answer}', { answer: 42 }),
'the answer is 42')
}

We randomly selected the patch γ4 to report to the maintain4 Here, we add a non-capturing group (? :) to ensure that the capturing

groups of the patch γ3 and the initial regex are consistent.

USENIX Association

ers, and the maintainers confirmed it and released nodejs-tmpl
1.0.5 with the fix.

4

The Algorithms

4.1

Vulnerability Analysis

Inspirited by Li et al. [19] and Liu et al. [24], we diagnose the
vulnerability source via localizing the pathological sub-regex,
according to the vulnerable patterns.
4.1.1

Nested Quantifiers (N Q )

The first vulnerable pattern is a regex with nested quantifiers,
which is called Nested Quantifiers (N Q ) pattern [11, 19].
A N Q pattern has worst-case exponential behavior when a
pump string can be consumed by either an inner quantifier
or an outer one. In order to facilitate fixing the pathological
regex, we subdivide N Q pattern into three sub-patterns (i.e.,
N Q 1 , N Q 2 , and N Q 3 ), as given in Table 1.
In sub-pattern N Q 1 , the entire inner sub-regex is quantified as well. An example of sub-pattern N Q 1 is the key
portion (\d+)* in the real-world regex δ1 (Table 1) from
CVE-2015-9239. In sub-pattern N Q 2 , the inner sub-regex
is a form of r0 r1 r2 , where r0 and r2 are nullable, and r1 is
a quantified regex. An example of sub-pattern N Q 2 is the
key portion ([^\s\/?\.#]+\.?)+ in the real-world regex δ2
(Table 1) from CVE-2021-26272. The sub-pattern N Q 3 is
similar to N Q 2 , but differs in that r1 is a disjunction containing a quantified sub-regex as a disjunct. An example of
sub-pattern N Q 3 is the key portion (\[[^\[\]]*\]|\s+)+
in the real-world regex δ3 (Table 1) from the NPM package
moment5 .
5

https://github.com/moment/moment

31st USENIX Security Symposium

4187

<!-- page 7 -->

Table 1: The Sub-pattern, Vulnerability Description, Example Regex, and Results from AnaNQ of the Pattern N Q .
No.

Sub-pattern

#1

Vulnerability Description

Example Regex

Returned Triple

NQ1

r = r1 {m, n}, where r1 = r p {m p , n p }, n p > 1,
and n > 1

δ1 = \[(\d+;)?(\d+)*m (CVE-2015-9239)

(N Q 1 , (\d+)*, [+, *])

#2

NQ2

r = (r0 r1 r2 ){m, n}, where r0 and r2 are nullable,
r1 = r p {m p , n p }, n p > 1, and n > 1

δ2 = ^(https?|ftp):\/\/(-\.)?([^\s\/?\.#]+
\.?)+ (\/[^\s]*)?[^\s\.,]$ (CVE-2021-26272)

(N Q 2 ,

#3

NQ3

r = (r0 r1 r2 ){m, n}, where r0 and r2 are nullable,
r1 = (. . . |r p {m p , n p }| . . .), n p > 1, and n > 1

δ3 = D[oD]?(\[[^\[\]]*\]|\s+)+MMMM? (moment)

(N Q 3 , (\[[^\[\]]*\]|\s+)+, [+, +])

([^\s\/?\.#]+\.?)+, [+, +])

Table 2: The Sub-pattern, Vulnerability Description, Example Regex, and Results from AnaQOD of the Pattern Q OD .
No.

Sub-pattern

#1

Vulnerability Description

Example Regex

Q OD 1

r = α{m, n}, α = (r1 | . . . |rk ), and ∃ L (α1 ) ∩ L (α2 ) ̸= /0,
where α1 = r p1 r p2 . . . r pt , α2 = rq1 rq2 . . . rqs , 1 ≤ t, s ≤ k,
1 ≤ i ≤ t, 1 ≤ j ≤ s, 1 ≤ pi , q j ≤ k, and p1 ̸= q1

δ4 = ^(\d\.|\.\d|\d)+$

#2

Q OD 2

r = α{m, n}, α = (r1 | . . . |rk ), and ∃ L (r p ) ∩ L (α1 ) ̸= /0,
L (rl ) ∩ L (r p ) = /0, where α1 = rq1 rq2 . . . rqs , 1 ≤ i ≤ k,
ri = rl r p , rl is not nullable, 1 ≤ s ≤ k, 1 ≤ j ≤ s, 1 ≤ q j ≤ k

δ5 =(?:[\w-]|\$[-\w]+|#\{\$[-\w]+
\})+(?=\s*:) (CVE-2021-23341)

4.1.2

Quantified Overlapping Disjunction (Q OD )

The second vulnerable pattern is a quantified disjunction
whose multiple inner sub-regexes overlap, which is called
Quantified Overlapping Disjunction (Q OD ) pattern [19].
When matching a pump string, there are multiple choices
among the overlapping sub-regex, with worst-case exponential behavior on a mismatch. As shown in Table 2, we
subdivide Q OD pattern into two sub-patterns (i.e., Q OD 1 ,
Q OD 2 ) for the convenience of fixing the pathological regex.
In sub-pattern Q OD 1 , a t-iteration (i.e., α1 ) of the inner
sub-regexes overlaps with another s-iteration (i.e., α2 ), starting with different heads (i.e., r p1 and rq1 ). An example of subpattern Q OD 1 is the key portion (\d\.|\.\d|\d)+ in the
regex δ4 (Table 2) from [14], where α1 = \d\.\d and α2 =
\d\.\d. In sub-pattern Q OD 2 , a strict tail (i.e., r p ) of an inter
sub-regex (i.e., ri ) overlaps with an s-iteration (i.e., α1 ) starting with rq1 (at least one iteration is not ri ). An example of subpattern Q OD 2 is the key portion (?:[\w-]|\$[-\w]+|. . .)+
in the real-world regex δ5 (Table 2) from CVE-2021-23341,
where r p = [-\w]+ and α1 = [\w-].
4.1.3

Quantified Overlapping Adjacent (Q OA )

The third pattern is a quantified regex containing two adjacent
overlapping sub-regexes, which is called Quantified Overlapping Adjacent (Q OA ) pattern [19]. In the pattern Q OA , a
pump string can be consumed by either of the overlapping
adjacencies. To facilitate fixing the pathological regex, we
subdivide the pattern Q OA into five sub-patterns, as shown
in Table 3. The detailed descriptions and real-world examples are also given in Table 3. Note that, Q OA 1 contains
two directly adjacent overlapping sub-regexes, while Q OA 2 ,

4188

31st USENIX Security Symposium

Returned Triple

( [14])

(Q OD 1 , (\d\.|\.\d|\d)+, [\d\.\d,
\d\.\d])
(Q OD 2 , (?:[\w-]|\$[-\w]+|#\{\$
[-\w]+\})+, [[\w-], \$[-\w]+])

Q OA 3 , Q OA 4 , and Q OA 5 contain two overlapping subregexes that one can reach the other by crossing the quantifier
and/or skipping the optional sub-regex. In addition, assuming
the outer quantifier is {m, n}, we also call Q OA as EOA if
n > 1 and as P OA otherwise (i.e., n ≤ 1).
4.1.4

Starting with Large Quantifier (SLQ )

The fourth pattern is a regex starting with a sub-regex with a
large quantifier, which is called Starting with Large Quantifier
(SLQ ) pattern [19]. The SLQ pattern may cause the regex
engine to keep moving the matching regex across the malicious string that does not have a match for the regex, leading
to worst-case polynomial behavior.
As shown in Table 4, we subdivide SLQ pattern into five
sub-patterns (i.e., SLQ 1 , SLQ 2 , SLQ 3 , SLQ 4 , SLQ 5 ) for
fixing the pathological regex. Sub-pattern SLQ 1 starts with
a sub-regex with a large quantifier, and sub-pattern SLQ 2
can behave the same by skipping the beginning nullable subregex. While sub-patterns SLQ 3 , SLQ 4 and SLQ 5 start
with the sub-regex r1 r2 , where r1 is not nullable, r2 contains
a large outer quantifier, and r1 overlaps with the prefix, infix
and suffix of r2 , respectively. The examples for these five
sub-patterns are also given in Table 4.
We propose algorithms AnaNQ, AnaQOD, AnaQOA, and
AnaSLQ to check whether the given regex contains any of the
N Q , Q OD , Q OA , and SLQ sub-patterns. These algorithms
will return the vulnerability information as a triple, which
consists of a pattern (i.e., the vulnerability sub-pattern), source
(i.e., the pathological sub-regex), and cause (i.e., the main
components that may cause catastrophic backtracking during
matching and would be modified during repairing, such as
the overlapping sub-regexes), if any of these sub-patterns was

USENIX Association

<!-- page 8 -->

Table 3: The Sub-pattern, Vulnerability Description, Example Regex, and Results from AnaQOA of the Pattern Q OA .
No.

Sub-pattern

#1

Q OA 1

r = (. . . r1 r2 . . .){m, n}, where
r2 = rq {mq , nq }, L (r1 ) ∩ L (r2 ) ̸= 0/

#2

Q OA 2

#3

Vulnerability Description

Example Regex

Returned Triple

δ6 = ^__[^\W_]+\w+__$ (pylint)

(Q OA 1 ,

[^\W_]+\w+,

[[^\W_]+, \w+])

r = (. . . r1 r2 r3 . . .){m, n}, where r2 = rt {mt , nt } is nullable,
r1 = r p {m p , n p }, r3 = rq {mq , nq }, L (r1 ) ∩ L (r3 ) ̸= 0/

δ7 =
^(>=?|<=?)\s*(\d*\.?\d+)%$
(CVE-2021-23364)

(Q OA 2 ,

\d*\.?\d+,

[\d*, \d+])

Q OA 3

r = (r1 . . . r2 ){m, n}, where
r1 = r p {m p , n p },
r2 = rq {mq , nq }, n > 1, L (r1 ) ∩ L (r2 ) ̸= 0/

δ8 = @([\w\-]+\.[\w\-:]+)+[:/]

(Q OA 3 , ([\w\-]+\.[\w\-:]+)+, [[\w\-]+,
[\w\-:]+])

#4

Q OA 4

r = (r1 . . . r2 r3 ){m, n}, where r3 = rt {mt , nt } is nullable,
r1 = r p {m p , n p }, r2 = rq {mq , nq }, n > 1, L (r1 ) ∩ L (r2 ) ̸= 0/

δ9 = ^([ab]+d[ac]+e?)+$

(Q OA 4 ,

#5

Q OA 5

r = (r1 r2 . . . r3 ){m, n}, where r1 = rt {mt , nt } is nullable,
r2 = r p {m p , n p }, r3 = rq {mq , nq }, n > 1, L (r2 ) ∩ L (r3 ) ̸= 0/

δ10 = ^ (;(([ \t]*[0-9a-zA-Z]+=
[\x21-\x7E]*)*)[ \t]*)?$

(Q OA 5 , ([ \t]*[0-9a-zA-Z]+=[\x21-\x7E]*)*,
[[0-9a-zA-Z]+, [\x21-\x7E]*])

r1 = r p {m p , n p },

([ab]+d[ac]+e?)+, [[ab]+, [ac]+])

Table 4: The Sub-pattern, Vulnerability Description, Example Regex, and Results from AnaSLQ of the Pattern SLQ .
No.

Sub-pattern

#1

Vulnerability Description

Example Regex

SLQ 1

Starting with r = r1 , where r1 = rq {mq , nq }, and nq > 1

δ11 = ([A-Z]+)([A-Z][a-z])

#2

SLQ 2

Starting with r = r1 r2 , where r1 = r p {m p , n p } is nullable,
r2 = rq {mq , nq }, and nq > 1

δ12 = \$?[A-Z]+

#3

SLQ 3

Starting with r = r1 r2 , where r1 = r p {m p , n p } is not nullable, r2 = rq {mq , nq }, and L (r1 ) ∩ L (r2 ) ̸= /0

δ13 = {([\s\S]+?)}

#4

SLQ 4

Starting with r = r1 r2 , where r1 = r p {m p , n p } is not
nullable, r2 = rq {mq , nq }, rq = rq1 rq2 , rq1 is not nullable,
rq2 = rt {mt , nt }, and L (rq2 ) ∩ L (r1 ) ̸= /0

δ14 = [ab](ca+)+d

(SLQ 4 , [ab](ca+)+, [[ab], a+])

#5

SLQ 5

Starting with r = r1 r2 , where r1 = r p {m p , n p } is not
nullable, r2 = rq {mq , nq }, rq = rq1 rq2 rq3 , rq1 and rq3 are
not nullable, rq2 = rt {mt , nt }, and L (rq2 ) ∩ L (r1 ) ̸= 0/

δ15 = [ab](ca{1,2}da)+e

(SLQ 5 , [ab](ca{1,2}da)+, [[ab], a{1,2}])

Vulnerability Repair

After vulnerability analysis, we can get the vulnerability information (i.e., pattern, source, and cause). In this section, we
will introduce how to revise the regex, using our proposed
repair patterns targeting at the vulnerability information.
Table 5: The Repair Patterns for Nested Quantifiers (N Q ).
No.

Repair Pattern

τ1

L (r) = L (r p {m p × m, n p × n})
(N Q 1 )
Xn}
r =⇒ (r p {m p × m, n p × n})X
{m,
X
L (r) = L ((r0 r pX
{mX
}r2 ){m, n})
XpX
p, n

τ2

r =⇒ (r0 r pX
{mX
n pX
}r2 ){m, n}
p ,X

(N Q 2 )

L (r) = L ((r0 (. . . |r pX
{mX
}| . . .)r2 ){m, n})
XpX
p, n
τ3

r1 =⇒ (. . . |r pX
{mX
n pX
}| . . .)
p ,X

USENIX Association

(N Q 3 )

(CVE-2021-3820)

(PrismJS)
(CVE-2021-3777)

4.2.1

met.

4.2

Returned Triple
(SLQ 1 , [A-Z]+, [A-Z]+)
(SLQ 2 , \$?[A-Z]+, [A-Z]+)
(SLQ 3 , {([\s\S]+?), [{, [\s\S]+])

Nested Quantifiers (N Q )

After calling AnaNQ, we can get the triple (pattern, source,
cause), where pattern belongs to N Q 1 , N Q 2 , or N Q 3 ,
source is the pathological regex r, and cause contains the
nested quantifiers as shown in Table 1. The pathological regex
r has, by their nature, a redundant quantifier. So to fix r, we
can remove the redundant quantifier.
The repair patterns targeting at N Q are given in Table 5. Specifically, the repair pattern τ1 aims to repair the
sub-pattern N Q 1 by replacing the inner quantifier {m p , n p }
with the merged quantifier {m p × m, n p × n} and deleting the
XX
outer quantifier X
{m,
n}, provided that the condition L (r) =
L (r p {m p × m, n p × n}) is satisfied. Let us continue to consider the pathological regex (\d+)* from CVE-2015-9239
given in Table 1. Applying the repair pattern τ1 , we can fix
it into a safe regex (\d*). The repair patterns τ2 and τ3 are
proposed to repair the sub-patterns N Q 2 and N Q 3 , respectively. Both of them remove the inner quantifier {m p , n p }
directly, if the language of the repaired regex is equivalent
to the one of the original regex. Taking the pathological
regexes ([^\s\/?\.#]+\.?)+ and (\[[^\[\]]*\]|\s+)+
in Table 1 for example, we can respectively fix them into the
safe ones ([^\s\/?\.#]\.?)+ and (\[[^\[\]]*\]|\s)+
using the repair patterns τ2 and τ3 .

31st USENIX Security Symposium

4189

<!-- page 9 -->

Table 6: The Repair Patterns for Quantified Overlapping Disjunction (Q OD ).
No.

Repair Pattern

L (r) = L ((r1 | . . . |@
r p | . . . |rk ){m, n})
τ4

α =⇒ (r1 | . . . |@
r p | . . . |rk )

(Q OD 1 )

No.

Repair Pattern

No.

Repair Pattern

τ5

t >1
(Q OD 1 )
r p1 =⇒ r p1 (?! Φ(r p2 ))

τ6

s>1
(Q OD {1,2} )
rq1 =⇒ rq1 (?! Φ(rq2 ))

τ8

rq1 =⇒ (?! Φ(α1 ))rq1

τ9

scs(α1 ) = true, Σα1 \ first(rq1 ) ̸= 0/
(Q OD 1 )
α1 =⇒ Θ(Σα1 \ first(rq1 ))

τ12

rq1 =⇒ (?! Φ(r p ))rq1

(Q OD 1 )

(Q OD 1 )

τ7

r p1 =⇒ (?! Φ(α2 ))r p1

τ10

scs(α2 ) = true, Σα2 \ first(r p1 ) ̸= 0/
(Q OD 1 )
α2 =⇒ Θ(Σα2 \ first(r p1 ))

τ11

r p =⇒ r p (? <! Φ(α1 ))

τ13

scs(α1 ) = true, Σα1 \ first(r p ) ̸= 0/
(Q OD 2 )
α1 =⇒ Θ(Σrq1 \ first(r p ))

τ14

r p = ru {mu , nu }, scs(ru ) = true, Σru \ first(rq1 ) ̸= 0/
(Q OD 2 )
r p =⇒ Θ(Σru \ first(rq1 ))

4.2.2

Quantified Overlapping Disjunction (Q OD )

The cause returned by AnaQOD is the corresponding two
overlapping disjunctions in the pathological regex r (i.e., the
source), as illustrated in Table 2. Intuitively, a vulnerable pattern Q OD may have multiple matching paths across the overlapping disjunctions for a string. To repair Q OD , we need to
ensure that there is a unique matching path for each string.
For that, we propose three strategies, namely, deleting one
overlapping disjunction, adding a lookaround constraint to
one overlapping disjunction, and modifying one overlapping
disjunction by subtracting the first set of the other one. Table 6 gives our repair patterns for Q OD , where Φ(r) returns
a regex obtained by transforming all the (named) capturing
groups in the regex r into the non-capturing ones and Θ(D )
transforms the set D into an (equivalent) character class.
To fix the sub-pattern Q OD 1 , the repair pattern τ4 removes
the overlapping disjunction directly, if it does not alter the
language. For example, for the regex ^(ab|a|b)+$, the overlapping disjunctions (i.e., cause) returned by the algorithm
AnaQOD are α1 = ab and α2 = ab. As L ((ab|a|b)+) =
L ((Z
aZ
b|a|b)+), we can use the repair pattern τ4 to fix
(ab|a|b)+ and get (a|b)+. The repair patterns τ5 -τ8 add a
negative lookaround (i.e., negative lookahead or negative lookbehind) to specify what does not come before or after a match,
and thus ensure that each string has a unique matching path.
Consider the above regex (ab|a|b)+ again. With the repair
pattern τ5 , τ7 , or τ8 , we can also fix it into (ab|a(?!b)|b)+,
(ab|(?!ab)a|b)+, or ((?!a)ab|a|b)+, respectively. The
repair patterns τ9 and τ10 handle the case wherein one overlapping disjunction is a character class. Specifically, τ9 and
τ10 remove the common characters from the character class
to ensure there is no overlap. For example, we can fix the
pathological regex ([ab]|[ac])+ to ^([b]|[ac])+$ and
^([ab]|[c])+$, according to the repair patterns τ9 and τ10 ,
respectively. The sub-pattern Q OD 2 can be fixed similarly,
as shown in Table 6.

4190

31st USENIX Security Symposium

(Q OD 2 )

4.2.3

(Q OD 2 )

Quantified Overlapping Adjacent (Q OA )

Similar to Q OD , the returned cause for pattern Q OA contains the corresponding two overlapping adjacencies. Likewise, to repair Q OA , we need to ensure that only one overlapping adjacency would be selected when matching on a
string, and propose three repair strategies, that is, merging the
overlapping adjacencies, adding a lookaround constraint to
one overlapping adjacency, and modifying one overlapping
adjacency.
The repair patterns for the sub-pattern Q OA 1 are listed
in Table 7. First, if the languages of these two overlapping adjacencies are equivalent, the repair pattern τ15 simply merges them. For example, for the regex ^a+a+b$, we
can fix it to ^a{2,}b$ using repair pattern τ15 . Similar
to Q OD , the repair patterns τ16 and τ17 leverage a negative lookaround to resolve the overlap of the adjacencies,
thus making the matching path unique for each string. Consider the example regex δ6 = ^__[^\W_]+\w+__$ mentioned in Table 3, we can utilize repair pattern τ17 to fix
δ6 and get ^__[^\W_]+(?![^\W_])\w+__$. If a sub-pattern
Q OA 1 is a P OA , (i.e., the outer quantifier n ≤ 1 ), the
repair pattern τ18 substitutes both the quantifiers of these
two adjacencies with a small one nµ = 500, which can cut
the number of backtracking and thus avoid ReDoS attacks.
For example, for the above regex δ6 , we can also fix it to
^__[^\W_]{1,500}\w{1,500}__$ via the repair pattern τ18 .
The repair patterns τ19 and τ20 handle the case wherein one
overlapping adjacency is a character class. Consider the regex
δ6 again, we leverage repair pattern τ20 to fix δ6 and get
^__[^\W_]+_+__$.
Likewise, the other sub-patterns (i.e., Q OA 2 , Q OA 3 ,
Q OA 4 , and Q OA 5 ) can be fixed, and the repair patterns are
illustrated in Table 7.
4.2.4

Starting with Large Quantifier (SLQ )

The cause returned by AnaSLQ is either the sub-regex starting
with a large quantifier (for SLQ 1 and SLQ 2 ) or the overlap-

USENIX Association

<!-- page 10 -->

Table 7: The Repair Patterns for Quantified Overlapping Adjacency (Q OA ).
No.

Repair Pattern

No.

Repair Pattern

τ15

L (r p ) = L (rq )
(Q OA 1 )
r1 r2 =⇒ r p {m p +mq , n p +nq }

τ16

r1 =⇒ r1 (? <! Φ(rq ))

τ18

n≤1
n p =⇒ nµ , nq =⇒ nµ (Q OA {1,2} )

τ19

scs(r p ) = true, Σr p \ first(rq ) ̸= /0, m p ≥ 1
τ21

r1 =⇒ r p {m p −1, n p −1}Θ(Σr p \ first(rq ))

τ24

r2 =⇒ r2 (? <! Φ(r p ))

τ27

mt = 0
(Q OA 2 )
r =⇒ r1 r3 |r1 rt {1, nt }r3

r p =⇒ Θ(Σr p \ first(rq ))

r2 =⇒ Θ(Σrq \ first(r p ))rq {mq −1, nq −1}
scs(r p ) = true, Σr p \ first(rq ) ̸= /0, m p ≥ 1

(Q OA 3 )
τ25

r1 =⇒ Θ(Σr p \ first(rq ))r p {m p −1, n p −1}

τ28

mt = 0
(Q OA 4 )
r2 r3 =⇒ r2 |r2 rt {1, nt }

ping sub-regexes (for SLQ 3 , SLQ 4 , and SLQ 5 ), as illustrated in Table 4. Intuitively, a vulnerable pattern SLQ makes
the regex engine slide continuously to determine the starting
position of the match. To fix SLQ , we need to eliminate or alleviate continuous sliding. For that, we propose four strategies,
namely, adding a start-of-line anchor ^ to the pathological
regex r, replacing the large quantifier in the pathological regex
r with a small one, adding a lookaround constraint to one overlapping sub-regex, and modifying one overlapping sub-regex
by subtracting the first set of the other one. The repair patterns for SLQ are listed in Table 8. Example 1 mentioned in
§3 demonstrates the process of using these repair patterns to
repair SLQ .
Finally, we prove our repair patterns preserve the semantics,
that is, the language of the repaired regex is a subset of the
language of the original one (see Appendix A.1 for the proof).

4.3

Vulnerability Verification

After vulnerability repair, we may get one or more repaired
regexes, because there may be more than one repair pattern
suitable for the pathological regex. Then for every repaired
regex, we check whether it is free from ReDoS attacks (i.e.,
vulnerability-free) 6 and consistent with the given examples.
If so, the repaired regex is called a successful one. While if
the repaired regex still suffered from ReDoS attacks, then we
will continue the vulnerability analysis and repair it. This is
because a ReDoS-vulnerable regex may contain more than
one vulnerability. We argue that a vulnerability pattern would
not always appear in the repaired regexes again after repair
(see Appendix A.2 for the proof). Finally, we will return a
repaired regex randomly chosen from the successful ones.
We adopt a simple metric for checking ReDoS that is
widely adopted by existing work [11, 12, 26]: a regex is said
to be vulnerable to ReDoS attacks if an attack string of fewer
than 1 million characters could cause the regex to take 10
6 In this paper, a regex is vulnerability-free means that it is free of four

vulnerable patterns (i.e., N Q , Q OD , Q OA , and SLQ ) defined in this paper.

USENIX Association

τ17

r2 =⇒ (?! Φ(r p ))r2

(Q OA 1 )

scs(rq ) = true, Σrq \ first(r p ) ̸= 0/

(Q OA {1,3} )

scs(rq ) = true, Σrq \ first(r p ) ̸= /0, mq ≥ 1
τ22

Repair Pattern

(Q OA 1 )

scs(r p ) = true, Σr p \ first(rq ) ̸= /0

(Q OA 1 )

No.

τ20

(Q OA 1 )

τ23

rq =⇒ Θ(Σrq \ first(r p ))

r1 =⇒ (?! Φ(rq ))r1

(Q OA {1,3} )

(Q OA 3 )

scs(rq ) = true, Σrq \ first(r p ) ̸= /0, mq ≥ 1

(Q OA 3 )

τ26

r2 =⇒ rq {mq −1, nq −1}Θ(Σrq \ first(r p ))

τ29

mt = 0
(Q OA 5 )
r1 r2 =⇒ r2 |rt {1, nt }r2

(Q OA 3 )

seconds or more. In addition, for multiple vulnerabilities in
one vulnerable regex, we repair only one vulnerability at a
time, and then run the above steps multiple times until there
are no vulnerabilities found in the regex.

5

Evaluation

In this section, we evaluate RegexScalpel on a wide range
of ReDoS-vulnerable regexes collected from the SOLA-DA
benchmark [34] and real-world CVEs [9]. We implemented
RegexScalpel in Java, and conducted experiments on a machine with 16 cores Intel Xeon CPU E5620 @ 2.40GHz with
12MB Cache, 24GB RAM, running Windows 10. For all the
experiments described below, we set the repair time budget
for each vulnerability to 5 minutes and the small quantifier
nµ to 500. Our evaluation aims to answer the following three
research questions (RQs):
• RQ1. Can RegexScalpel outperform state-of-the-art
regex defense techniques? We evaluate the effectiveness of RegexScalpel on ReDoS-vulnerable regexes
from the SOLA-DA benchmark [34] and ReDoS-related
CVEs [9], compared with five state-of-the-art ReDoS
defense techniques. These techniques vary from regex
engine substitution, input length limit and regex repair.
• RQ2. Can RegexScalpel outperform handcrafted defense actions? We compare the repaired regexes synthesized by RegexScalpel with the defense actions handcrafted by project maintainers.
• RQ3. Can RegexScalpel detect new ReDoS vulnerabilities and synthesize repairs of usefulness to maintainers? We explore the usefulness of RegexScalpel on
popular real-world projects in the detection of new ReDoS vulnerabilities and the synthesis of valid repairs
useful to the project maintainers.
• RQ4. Can RegexScalpelsynthesize repaired regexes
preserving the semantics of the original ones? We

31st USENIX Security Symposium

4191

<!-- page 11 -->

Table 8: The Repair Patterns for Starting with Large Quantifier (SLQ ).
No.

Repair Pattern

No.

τ30

r =⇒ ˆr

τ33

r p =⇒ (?! Φ(rq ))r p

(SLQ {1,2,3,4,5} )
τ31
(SLQ 3 )

scs(rq ) = true, Σrq \ first(r p ) ̸= 0/
rq =⇒ Θ(Σrq \ first(r p ))

τ36

scs(r p ) = true, Σr p \ first(rt ) ̸= /0
r p =⇒ Θ(Σr p \ first(rt ))

τ39

(SLQ 3 )

(SLQ {4,5} )

Repair Pattern
nq =⇒ nµ (SLQ {1,2,3,4,5} )

No.

Repair Pattern

τ32

mp = 0
(SLQ 2 )
r =⇒ r2 |r p {1, n p }r2
scs(r p ) = true, Σr p \ first(rq ) ̸= /0

(SLQ 3 )

τ34

rq =⇒ (?! Φ(r p ))rq

τ37

r p =⇒ r p (? <! Φ(rt ))

τ40

scs(rt ) = true, Σrt \ first(r p ) ̸= 0/
(SLQ {4,5} )
rt =⇒ Θ(Σrt \ first(r p ))

τ35
(SLQ {4,5} )
τ48

r p =⇒ Θ(Σr p \ first(rq ))

rt =⇒ (?! Φ(r p ))rt

(SLQ 3 )

(SLQ {4,5} )

Table 9: The ReDoS-vulnerable Regex Sets for Evaluation.

5.1.2

Benchmark

To answer RQ1, we selected three state-of-the-art tools belonging to three paradigms, i.e., regex engine substitution
(RE2 [16]), input length restriction (LLI7 ), and regex repair
(FlashRegex [22]). Since RE2 and LLI do not defend ReDoS
vulnerabilities by regex repair, we use the term “defense” instead of “repair” when referring to the evaluation results in
RQ1. To answer RQ2, we compared RegexScalpel with the
project maintainers.

#Regex

SOLA-DA [34]

34

CVE [9]

414

Total

448

Description
ReDoS-vulnerable regexes in NPM
modules found by the Software Lab at
TU Darmstadt
ReDoS-vulnerable regexes extracted
from 70 ReDoS-related CVEs in recent
three years

5.1.3
verify that the repaired regexes are subsets of the original
ones, and analyze the semantic similarities between the
repaired regexes and the original ones.

5.1

Evaluation Setup

5.1.1

Evaluation Datasets

4192

31st USENIX Security Symposium

Evaluation Metrics

A defense is considered successful if it (i) passes all the given
test cases, and (ii) is free from ReDoS attack. The success
defense rate is calculated by dividing the number of successful
defenses by the total number of vulnerable regexes under
defense.

5.2
Our evaluation was conducted on the ReDoS-vulnerable
regexes collected from two widely-used sources: (i) the
SOLA-DA benchmark [34] and (ii) real-world CVEs [9]. The
statistics are listed in Table 9. The first benchmark SOLA-DA
was constructed by Staicu and Pradel [34]. It consists of 34
ReDoS-vulnerable regexes, and is often used for research on
finding, fixing, and mitigating ReDoS vulnerabilities. For the
second benchmark, we collected ReDoS-vulnerabilities in the
last three years from widely-used libraries with Common Vulnerabilities and Exposures (CVE) [9] identifiers. Collected
vulnerabilities without clear descriptions, live links, or test
cases were discarded, resulting in 70 CVEs in total. Moreover,
one CVE may contain more than one vulnerable regex. Finally, we extracted 414 ReDoS-vulnerable regexes from these
70 CVEs. In total, there are 448 (34 + 414) ReDoS-vulnerable
regexes in our evaluation dataset. The details of these regexes
together with their test cases can be found online [21].

Evaluation Approaches

RQ1: Comparing State-of-the-art

Overall Results. We evaluated the effectiveness of
RegexScalpel on 448 ReDoS-vulnerable regexes comparing
with different defense tools, i.e., RE2, LLI with different input
length limits, and FlashRegex. Table 108 shows the number
(ratio) of regexes that have been defended successfully. We
can see that across the two benchmarks, the effectiveness
of RegexScalpel outperforms existing works, defending over
98% vulnerable regexes, over four times of the best results
(FlashRegex with 21.20%) achieved by baselines.
In the following, we further investigated the reason why
existing works performed unsatisfactorily, and thus revealed
the advantage of RegexScalpel from two perspectives.
Defense Capabilities on Vulnerable Pattern SLQ . We analyzed the distribution of four vulnerable patterns over all
7 We implemented three variants of input length restriction to limit the
length of the input to 100, 500, and 5000, which are denoted as LLI(100),
LLI(500), and LLI(5000)), respectively.
8 Table 10 shows overall results. The detailed results can be found online [21].

USENIX Association

<!-- page 12 -->

Table 10: Success Defense Rate Across Automated Tools.
Tool

SOLA-DA

CVE

Total

RE2

18 (52.94%)

35 (8.45%)

53 (11.83%)

LLI(100)

22 (64.71%)

45 (10.87%)

67 (14.96%)

LLI(500)

26 (76.47%)

18 (4.35%)

44 (9.82%)

LLI(5000)

26 (76.47%)

19 (4.59%)

45 (10.04%)

FlashRegex

4 (11.76%)

91 (21.98%)

95 (21.20%)

RegexScalpel

33 (97.06%)

410 (99.03%)

443 (98.88%)

34

414

448

#Regex

Table 12: Success Defense Rate for Regexes with
Lookarounds or Backreferences.
Tool
RE2
LLI(100)
LLI(500)
LLI(5000)
FlashRegex
RegexScalpel
#Regex

SOLA-DA
0 (0%)
0 (0%)
5 (71.43%)
5 (71.43%)
0 (0%)
6 (85.71%)
7

CVE
0 (0%)
2 (6.25%)
2 (6.25%)
2 (6.25%)
0 (0%)
30 (93.75%)
32

Total
0 (0%)
2 (5.13%)
7 (17.95%)
7 (17.95%)
0 (0%)
36 (92.31%)
39

Table 11: Success Defense Rate for SLQ Regexes.
Tool
RE2
LLI(100)
LLI(500)
LLI(5000)
FlashRegex
RegexScalpel
#Regex

SOLA-DA
16 (55.17%)
21 (72.41%)
26 (89.66%)
26 (89.66%)
0 (0%)
29 (100%)
29

CVE
21 (6.75%)
26 (8.36%)
18 (5.79%)
18 (5.79%)
0 (0%)
309 (99.36%)
311

Total
37 (10.88%)
47 (13.82%)
44 (12.94%)
44 (12.94%)
0 (0%)
338 (99.41%)
340

vulnerable regexes, and observed that the pattern SLQ is the
major cause of the vulnerability, accounting for 85.29% (29
/ 34) and 75.12% (311 / 414) ReDoS vulnerabilities in the
two benchmarks, respectively. We further analyzed the results
of the tools for their defending vulnerabilities with respect
to SLQ pattern. The statistics are shown in Table 11. We
note that the replacement of regex engines (i.e., RE2) only
reaches 10.88% success defense rate, which indicates that it
cannot effectively address vulnerable regexes with SLQ pattern in both benchmarks. By limiting input length (i.e., LLI),
the success defense rate ranges from 5.79% to 89.66%, which
is still not satisfactory. In addition, FlashRegex, the state-ofthe-art regex defense tool, cannot defend regexes with SLQ
vulnerable pattern, resulting in 0% success defense rate in
both benchmarks. In contrast, RegexScalpel can successfully
defend 99.41% (338 over 340) regexes with SLQ pattern,
making them free from ReDoS attacks.
Defense Capabilities on Lookarounds and Backreferences.
The versatility of the rich extended features is also a reason
that sets existing tools back. Among the extended features,
we observed two features (i.e., lookarounds and backreferences) are more dominant than other features. So we investigated the effectiveness of regexes with these two features,
as shown in Table 12. There are 7 and 32 regexes using
lookarounds or backreferences in the two benchmarks, respectively. The design of RE2 and FlashRegex do not consider
lookarounds and backreferences, leaving them incapable of
defending such vulnerable regexes. The state-of-the-art defense tools (i.e., LLI(500) and LLI(5000)) can only defend

USENIX Association

at most 7 (17.95%) of them, while our RegexScalpel can successfully defend 92.31% (36 over 39). The table shows the
existing tools rarely consider handling vulnerable regexes
using lookarounds or backreferences, resulting in the unsatisfactory defense capabilities for these sorts of vulnerabilities.
Finally, we explain the reason why there are five (5/448,
1.1%) regexes which cannot be fixed by RegexScalpel. A fix
is considered to be successful if: (i) no vulnerabilities can be
found, and (ii) all the test cases pass. In our evaluation, the
unsuccessful fixes are all caused by the presence of failing test
cases. That is, some test cases would not be accepted/matched
by the repaired regex.
Summary to RQ1: RegexScalpel can effectively defend 98.88% of vulnerable regexes, compared with
21.20% achieved by the best work. The high defense
capability of RegexScalpel benefits from the comprehensive consideration of all four vulnerable patterns.
In addition, the design of RegexScalpelconsiders the
extended features such as lookarounds and backreferences, enabling RegexScalpel to be applicable for
regexes with such extended features.

5.3

RQ2: Comparing Maintainers’ Repairs

Overall Results. Apart from the automatic defense tools
shown above, we compared the repairs made by RegexScalpel
with the defending actions taken by the project maintainers.
Defending actions can be regex engine substitution, input
length limit, regex repair, or code logic modification. We
showed the statistics of maintainers’ defense strategies in Table 13. Among four strategies, regex repair is mostly used by
maintainers, accounting for the highest proportion, as high
as 92.19%, followed by code logic modification (5.13%).
Also note that there are 8 outstanding vulnerable regexes
(1.79%) whose defense actions have not been determined. To
answer RQ2, we evaluated RegexScalpel on the 413 vulnerable regexes that were fixed by the maintainers using regex
repair.

31st USENIX Security Symposium

4193

<!-- page 13 -->

Table 13: The proportion of Maintainers’ Defense Actions.
Defense Strategy

SOLA-DA

CVE

Total

Regex Engine Substitution

0 (0%)

1 (0.24%)

1 (0.22%)

Input Length Limit

1 (2.94%)

2 (0.48%)

3 (0.67%)

Code Logic Modification

6 (17.65%)

17 (4.11%)

23 (5.13%)

Regex Repair

21 (61.76%)

392 (94.69%)

413 (92.19%)

No Fix

6 (17.65%)

2 (0.48%)

8 (1.79%)

#Regex

34

414

448

SOLA-DA
14 (66.67%)
21 (100%)
21

CVE
305 (77.81%)
388 (98.98%)
392

Table 15: Success Defense Rate for Multiple Vulnerabilities
Regexes of the Repairs by Maintainers and RegexScalpel.
SOLA-DA

CVE

Total

Manual Repair

1 (14.29%)

113 (59.16%)

114 (57.58%)

RegexScalpel

7 (100%)

187 (97.91%)

194 (97.98%)

7

191

198

4194

31st USENIX Security Symposium

CVE

Total

Manual Repair

13 (72.22%)

229 (76.33%)

242 (76.10%)

RegexScalpel

18 (100%)

298 (99.33%)

316 (99.37%)

18

300

318

can successfully repair 72.22% and 76.33% of the regexes
with SLQ in the two benchmarks, respectively. In comparison, RegexScalpel can achieve 100% and 99.33% in the
two benchmarks, respectively. The results reveal that fixing
regexes with SLQ pattern manually is challenging.

Total
319 (77.23%)
409 (99.03%)
413

Table 14 shows that the repairs handcrafted by project maintainers can successfully defend 77.23% (319 / 413) vulnerable
regexes as compared with 99.03% (409 / 413) by the ones generated using RegexScalpel. In the following, we discuss our
observations from two perspectives, multiple vulnerabilities
in one regex and SLQ -pattern regexes.
Multiple Vulnerabilities in One Regex. On examining the
vulnerable regexes in the benchmarks, we observed there are
vulnerable regexes that contain more than one vulnerability. We conducted further analysis on the number of such
regexes that can be successfully repaired by the maintainers
and by RegexScalpel, respectively. As shown in Table 15,
there are 7 and 191 vulnerable regexes containing more than
one vulnerability in the two benchmarks. Maintainers can
only repair 57.58% of such regexes, with 14.29% and 59.16%
success rates for the benchmarks, respectively. In comparison, RegexScalpel can successfully repair 97.98% of these
vulnerable regexes automatically.
ReDoS-vulnerable Regexes with Pattern SLQ . Among
the 413 vulnerable regexes that were repaired by maintainers, we made a comparison over the ones with SLQ pattern. We found 318 such vulnerable regexes from both benchmarks. Table 16 shows the comparison results. Maintainers

#Regex

SOLA-DA

#Regex

Table 14: Success Defense Rate of the Repairs by Maintainers
and RegexScalpel.

Manual Repair
RegexScalpel
#Regex

Table 16: Success Defense Rate for SLQ Regexes of the
Repairs by Maintainers and RegexScalpel.

Summary to RQ2: Among the 413 repaired vulnerable regexes handcrafted by the maintainers, only
319 (77.23%) are ReDoS free. In RegexScalpel
outperforms manual fixing, successfully repairs
409 (99.03%) of the 413 regexes. In particular,
RegexScalpel excels at repairing regexes with multiple vulnerabilities and regexes with SLQ vulnerable
pattern.

5.4

RQ3: Usefulness to Maintainers

We explored whether RegexScalpel can detect new ReDoS
vulnerabilities in popular real-world projects and synthesize repairs of usefulness to project maintainers. We applied
RegexScalpel to ten popular projects including Python and
NLTK. The results are shown in Table 17. Although our
methodology in this paper focuses on regex repair, the modules in RegexScalpel can also be applied to detecting ReDoS
vulnerabilities. In total, RegexScalpel reported 16 new ReDoS regexes from ten projects. We applied RegexScalpel to
repair these vulnerable regexes, and reported the repairs to
the concerned project maintainers. All the 16 repairs were accepted by the maintainers and merged into subsequent project
releases. At the time of submission, 8 CVEs concerning 8
projects and 13 of the 16 vulnerable regexes were assigned.
The results demonstrate the usefulness of RegexScalpel to
project maintainers in defending against ReDoS attacks.
We take case #1 (i.e., the Python project) in Table 17 as an
example to demonstrate the process of applying RegexScalpel
in practice. The case concerns a vulnerable regex
(?:.*,)*[ \t]*([^ \t]+)[ \t]+realm=(["\']?)([^"
\']*)\2 used by Python built-in module urllib. The
regex contains six vulnerabilities. Processing of
the regex can be computationally expensive and is
therefore vulnerable to ReDoS attacks. The Python
maintainers
replaced the
pathological sub-regex
(?:.*,)* with the safe one (?:^|,) to get the patch

USENIX Association

<!-- page 14 -->

Table 17: Demographics of New Vulnerabilities Repaired by
RegexScalpel.
Project

Disclosure Date

CVE ID

#Vuln. Regex

#1

Python

Jan 30th, 2021

CVE-2021-3733

1

#2

NLTK

Sep 5th, 2020

–

1

#3

pylint

Sep 3rd, 2020

–

2

#4

mpmath

Oct 8th, 2021

CVE-2021-29063

1

#5

browserslist

Apr 28th, 2021

CVE-2021-23364

6

#6

code-server

Sep 17th, 2021

CVE-2021-3810

1

#7

ansi-regex

Sep 12th, 2021

CVE-2021-3807

1

#8

nth-check

Sep 17th, 2021

CVE-2021-3803

1

#9

nodejs-tmpl

Sep 15, 2021

CVE-2021-3777

1

#10

jspdf

Feb 12th, 2021

CVE-2021-23353

1

Total

16

❶

Q OA 1
Q OA 2
SLQ 1
SLQ 1
SLQ 2
SLQ 2

❷
❸
❹
❺
❻

Vuln. Source

Vuln. Cause

(?:.*,)*

[.*, ,]

(?:.*,)*[ \t]*([^ \t]+)

[(?:.*,)*, ([^ \t]+)]

(?:.*,)*

(?:.*,)*

.*

.*

(?:.*,)*[ \t]*

[ \t]*

(?:.*,)*[ \t]*([^ \t]+)

[^ \t]+

Summary to RQ3: RegexScalpel is useful to synthesize non-trivial and effective repairs for new vulnerable regexes found in popular real-world projects.

USENIX Association

|L (r1 ) ∩ L (r2 )|
|L (r1 ) ∪ L (r2 )|

(1)

where L (r1 ) and L (r2 ) are languages of regexes r1 and r2 ,
respectively. Yet, the languages can be infinite, so we adopted
the following equation used in previous work [5]:

Table 18: The Six ReDoS Vulnerabilities in Python built-in
module urllib.
Sub-pattern

Finally, we showed the empirical evidence by verifying that
the repaired regexes synthesized by RegexScalpel are subsets
of the original ones. The experiments indicated that all repaired regexes synthesized by RegexScalpel are subsets of
the corresponding original ones.
Furthermore, we calculated the semantic similarities between the repaired regexes and the original ones. To measure
the semantic similarity, we used the equation as follows:
Sim(r1 , r2 ) =

(?:^|,)[ \t]*([^ \t]+)[ \t]+realm=(["\']?)([^"\'
]*)\2. The patch fixed the first four vulnerabilities related
to the pathological sub-regex (?:.*,)* in Table 18, and
prevented the pathological sub-regex [ \t]* from being
used as a starting sub-regex, which in turn fixed the fifth
vulnerability. However, it cannot fix the sixth vulnerability,
which is related to the pathological sub-regex [^ \t]+.
RegexScalpel analyzed the vulnerability information of the
patch (SLQ 3 , (?:^|,)[ \t]*([^ \t]+), [,, ([^ \t]+)]).
It leveraged the pattern τ36 in Table 8 to synthesize a
repaired regex (?:^|,)[ \t]*([^ \t,]+)[ \t]+realm=
(["\']?)([^"\' ]*)\2 based on the patch provided
by the maintainers. In addition, RegexScalpel leveraged
the repair patterns τ19 in Table 7 and τ30 in Table 8
to repair the initial regex and synthesize another patch
^(?:[^,]*,)*[ \t]*([^ \t]+)[ \t]+realm=(["\']?)
([^"\']*)\2. The two synthesized patches RegexScalpel
passed all test cases, and the first one is committed to the
maintainers. Finally, the maintainers confirmed and released
it in Python 3.6-3.10, and appreciated our help.

No.

RQ4: Semantics Preservation

|L (r1 )≤λ ∩ L (r2 )≤λ |
λ→+∞ |L (r1 )≤λ ∪ L (r2 )≤λ |

Sim(r1 , r2 ) = lim

(2)

where λ ∈ N. Formally, for a language L (r), let |L (r)≤λ |
denote the number of words in L (r) of length at most λ. The
calculation proceeds iteratively until the solution converges
to 0.001 as set in [5].

300

Regexes count

No.

5.5

150
0
150
300

92%

94%

96%
Similarity

98%

100%

Figure 3: Semantic similarities between the vulnerable
regexes and the repaired ones.
Figure 3 visualizes the semantic similarity between the
vulnerable regex and the repaired one. We can see that most
similarities go beyond 98%. On average, the semantic similarity is 99.57%, meaning that the semantics of regexes are
well-preserved after the repair.
Summary to RQ4: RegexScalpel can synthesize repaired regexes preserving the semantics of the original
ones and keeping the semantics as close as possible
to the original ones.

31st USENIX Security Symposium

4195

<!-- page 15 -->

6

Related Work

ReDoS Defense. Various techniques [2, 7, 8, 10, 13, 15–17, 22,
23, 27–30, 37–39, 42] proposed to defense or mitigate ReDoS
vulnerabilities by modifying the structure of regexes (i.e.,
regex repair) or optimizing regex matching (i.e., regex engine
optimization).
Regex Repair. Li et al. [22] proposed the first programmingby-example (PBE) framework, using determinism to integrate
regex synthesis and repair with respect to ReDoS vulnerabilities. But it can not synthesize and repair regexes with extended
features, such as lookarounds and backreferences. Chida and
Terauchi [7] proposed another PBE repair ReDoS framework,
which uses strong determinism constraints and supports some
extended features (e.g., lookarounds and backreferences). But
strong determinism constraints are not necessary. For example, the regex a(b*|b*)c is not strongly deterministic but
invulnerable. Also, as discussed in Sec. 1, there exist several
common problems in [22] and [7]. First, the repair quality
depends on the quality of the examples provided by the users.
It is difficult for users to provide sufficient (characteristic)
examples [20, 25], which makes it difficult for these tools to
get a repaired regex that is semantically equivalent or similar to the original one. Second, these tools ignore the SLQ
vulnerable pattern, which leads to the incapability of repairing SLQ regexes. For example, they can not repair the regex
a+b. Van der Merwe et al. [39] proposed several DFA-based
optimizing techniques transforming vulnerable pure regular
expressions into safe ones while focusing only on exponential
vulnerabilities, thus this work does not support extended functionalities and is unable to fix polynomial vulnerabilities (e.g.,
SLQ and P OA patterns). Cody-Kenny et al. [8] presented
a genetic-programming based tool to get alternative regexes
with better running time performance, which may synthesize
ReDoS-vulnerable regexes and does not support extended
features.
Regex Engine Optimization. For eliminating or alleviating ReDoS, many works are dedicated to optimizing the
regex engine, e.g., by parallel algorithms [23], GPU-based
algorithms [42], state-merging algorithms [2], Thompson’s
Non-deterministic Finite Automaton algorithm [10,37], counting automata matching algorithm [38], Parsing Expression
Grammars (PEGs) [15, 17, 27], memoization-based optimization [13] and recursion-limit/backtracking-limit/timelimit [28–30]. Whereas significantly improving performance
and thus alleviating ReDoS attacks, these techniques also
have some obvious flaws, such as not supporting extended
features or sacrificing memory.
ReDoS Detection. Previous works [19,24,26,31–33,35,36,
40, 41] have studied ReDoS detection, which can be mainly
classified as static analysis [31, 32, 40, 41], dynamic analysis [26, 33, 35, 36], and hybrid approaches combining both
static and dynamic analysis [19, 24]. It is worth mentioning
that there are two detectors (i.e., [19] and [24]) to detect

4196

31st USENIX Security Symposium

ReDoS vulnerabilities by formally and comprehensively modeling vulnerable patterns. In particular, pattern SLQ proposed
by [19] is a new ReDoS pattern that has not been recognized
by previous detection work. Although the vulnerable patterns
proposed by [19] and [24] are comprehensive, they are not
fine-grained enough for us to directly apply to Sec. 4.1. We
took inspiration from Li et al. [19] and Liu et al. [24] and
presented fine-grained vulnerable sub-patterns based on [19].

7

Conclusion

In this paper, we propose RegexScalpel, which can defend ReDoS attacks by automatically localizing and repairing ReDoSvulnerable regexes. RegexScalpel is the first approach to localize and fix the vulnerable regexes considering comprehensive
and fine-grained types of vulnerable causes. The experiment
reveals the high effectiveness of RegexScalpel by successfully
fixing 98.88% vulnerable regexes, compared with 21.20%
achieved by the best existing work. Further, it outperformed
the manual repair, achieving 99.03% success repair ratio compared with only 77.23% for manual repair. It also helped to
repair 16 ReDoS vulnerabilities in the ten real-world projects
and got confirmed by the maintainers, resulting in 8 confirmed
CVEs.

Acknowledgments
The authors would like to thank the reviewers and
our shepherd, Engin Kirda, for their helpful feedback
on an earlier version of this paper. This work was
supported in part by National Natural Science Foundation of China (Grants #61872339, #61932021, #61972260,
#61772347, #61836005), National Key Research and Development Program of China under Grant #2019YFE0198100,
Guangdong Basic and Applied Basic Research Foundation under Grant #2019A1515011577, Shenzhen Fundamental Research Program (Grants #20200810150421002,
#JCYJ20200813091134001), the Hong Kong RGC/GRF
(Grant No. 16207120), Hong Kong ITF (Grant No.
MHP/055/19), Huawei PhD Fellowship, MSRA Collaborative Research Grant, the National Research Foundation, Prime
Ministers Office, Singapore under its National Cybersecurity
R&D Program (Award No. NRF2018NCR-NCR005-0001),
NRF Investigatorship NRFI06-2020-0022-0001, the National
Research Foundation through its National Satellite of Excellence in Trustworthy Software Systems (NSOE-TSS) project
under the National Cybersecurity R&D (NCR) Grant award
No. NRF2018NCR-NSOE003-0001 and the Ministry of Education, Singapore under its Academic Research Fund Tier 3
(MOET32020-0004). Any opinions, findings and conclusions
or recommendations expressed in this material are those of
the author(s) and do not reflect the views of the Ministry of
Education, Singapore.

USENIX Association

<!-- page 16 -->

References
[1] Alberto Bartoli, Andrea De Lorenzo, Eric Medvet, and
Fabiano Tarlao. Inference of regular expressions for text
extraction from examples. IEEE Trans. Knowl. Data
Eng., 28(5):1217–1230, 2016.
[2] Michela Becchi and Srihari Cadambi. Memory-efficient
regular expression search using state merging. In INFOCOM 2007. 26th IEEE International Conference
on Computer Communications, Joint Conference of the
IEEE Computer and Communications Societies, 6-12
May 2007, Anchorage, Alaska, USA, pages 1064–1072.
IEEE, 2007.
Details of the Cloud[3] The Cloudflare Blog.
flare outage on July 2, 2019, 2020.
https:
//blog.cloudflare.com/details-of-the-cloud
flare-outage-on-july-2-2019/.
[4] Cezar Câmpeanu, Kai Salomaa, and Sheng Yu. A Formal Study Of Practical Regular Expressions. Int. J.
Found. Comput. Sci., 14(6):1007–1018, 2003.
[5] Jialun Cao, Meiziniu Li, Yeting Li, Ming Wen, ShingChi Cheung, and Haiming Chen. SemMT: A Semanticbased Testing Approach for Machine Translation Systems. ACM Trans. Softw. Eng. Methodol., 31(2):34e:1–
34e:36, 2022.
[6] Carl Chapman, Peipei Wang, and Kathryn T. Stolee. Exploring regular expression comprehension. In Proceedings of the 32nd IEEE/ACM International Conference
on Automated Software Engineering, ASE 2017, Urbana,
IL, USA, October 30 - November 03, 2017, pages 405–
416, 2017.
[7] Nariyoshi Chida and Tachio Terauchi. Repairing
dos vulnerability of real-world regexes.
CoRR,
abs/2010.12450, 2020.
[8] Brendan Cody-Kenny, Michael Fenton, Adrian Ronayne, Eoghan Considine, Thomas McGuire, and Michael
O’Neill. A search for improved performance in regular
expressions. In Proceedings of the Genetic and Evolutionary Computation Conference, GECCO 2017, Berlin,
Germany, July 15-19, 2017, pages 1280–1287, 2017.
[9] The MITRE Corporation. Common Vulnerabilities and
Exposures (CVE), 2020. https://cve.mitre.org/
index.html.
[10] Russ Cox. Regular Expression Matching Can Be Simple
And Fast (But Is Slow In Java, Perl, PHP, Python, Ruby,
...), 2007. https://swtch.com/~rsc/regexp/rege
xp1.html.

USENIX Association

[11] James C. Davis, Christy A. Coghlan, Francisco Servant,
and Dongyoon Lee. The impact of regular expression
denial of service (redos) in practice: an empirical study
at the ecosystem scale. In Proceedings of the 2018 ACM
Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of Software
Engineering, ESEC/SIGSOFT FSE 2018, Lake Buena
Vista, FL, USA, November 04-09, 2018, pages 246–256,
2018.
[12] James C. Davis, Louis G. Michael IV, Christy A. Coghlan, Francisco Servant, and Dongyoon Lee. Why aren’t
regular expressions a lingua franca? an empirical study
on the re-use and portability of regular expressions. In
Proceedings of the ACM Joint Meeting on European
Software Engineering Conference and Symposium on
the Foundations of Software Engineering, ESEC/SIGSOFT FSE 2019, Tallinn, Estonia, August 26-30, 2019,
pages 443–454, 2019.
[13] James C. Davis, Francisco Servant, and Dongyoon Lee.
Using selective memoization to defeat regular expression denial of service (redos). In 42nd IEEE Symposium
on Security and Privacy, SP 2021, San Francisco, CA,
USA, 24-27 May 2021, pages 1–17. IEEE, 2021.
[14] James Collins Davis. On the Impact and Defeat of Regular Expression Denial of Service. PhD thesis, Virginia
Tech, 2020.
[15] Bryan Ford.
Parsing expression grammars: a
recognition-based syntactic foundation. In Proceedings
of the 31st ACM SIGPLAN-SIGACT Symposium on Principles of Programming Languages, POPL 2004, Venice,
Italy, January 14-16, 2004, pages 111–122, 2004.
[16] Google. RE2, 2020. https://github.com/google/
re2.
[17] IBM. Rosie Pattern Language (RPL), 2020.
//rosie-lang.org/.

https:

[18] Louis G. Michael IV, James Donohue, James C. Davis,
Dongyoon Lee, and Francisco Servant. Regexes are
hard: Decision-making, difficulties, and risks in programming regular expressions. In 34th IEEE/ACM International Conference on Automated Software Engineering, ASE 2019, San Diego, CA, USA, November
11-15, 2019, pages 415–426, 2019.
[19] Yeting Li, Zixuan Chen, Jialun Cao, Zhiwu Xu,
Qiancheng Peng, Haiming Chen, Liyuan Chen, and
Shing-Chi Cheung. ReDoSHunter: A Combined Static
and Dynamic Approach for Regular Expression DoS Detection. In 30th USENIX Security Symposium, USENIX
Security 2021, August 11-13, 2021, pages 3847–3864.
USENIX Association, 2021.

31st USENIX Security Symposium

4197

<!-- page 17 -->

[20] Yeting Li, Shuaimin Li, Zhiwu Xu, Jialun Cao, Zixuan
Chen, Yun Hu, Haiming Chen, and Shing-Chi Cheung.
TRANSREGEX: Multi-modal Regular Expression Synthesis by Generate-and-Repair. In 43rd IEEE/ACM International Conference on Software Engineering, ICSE
2021, Madrid, Spain, 22-30 May 2021, pages 1210–
1222. IEEE, 2021.
[21] Yeting Li, Yecheng Sun, Zhiwu Xu, Jialun Cao, Yuekang
Li, Rongchen Li, Haiming Chen, Shing-Chi Cheung,
Yang Liu, and Yang Xiao. RegexScalpel, 2022. https:
//sites.google.com/view/regexscalpel/.
[22] Yeting Li, Zhiwu Xu, Jialun Cao, Haiming Chen,
Tingjian Ge, Shing-Chi Cheung, and Haoren Zhao.
Flashregex: Deducing anti-redos regexes from examples. In 35th IEEE/ACM International Conference on
Automated Software Engineering, ASE 2020, Melbourne,
Australia, September 21-25, 2020, pages 659–671, 2020.
[23] Cheng-Hung Lin, Chen-Hsiung Liu, and Shih-Chieh
Chang. Accelerating regular expression matching using
hierarchical parallel machines on GPU. In Proceedings
of the Global Communications Conference, GLOBECOM 2011, 5-9 December 2011, Houston, Texas, USA,
pages 1–5. IEEE, 2011.
[24] Yinxi Liu, Mingxue Zhang, and Wei Meng. Revealer:
Detecting and Exploiting Regular Expression Denial-ofService Vulnerabilities. In 42nd IEEE Symposium on
Security and Privacy, SP 2021, San Francisco, CA, USA,
24-27 May 2021, pages 1468–1484. IEEE, 2021.
[25] Mehdi Hafezi Manshadi, Daniel Gildea, and James F.
Allen. Integrating Programming by Example and Natural Language Programming. In Proceedings of the
Twenty-Seventh AAAI Conference on Artificial Intelligence, July 14-18, 2013, Bellevue, Washington, USA,
2013.
[26] Robert McLaughlin, Fabio Pagani, Noah Spahn, Christopher Kruegel, and Giovanni Vigna. Regulator: Dynamic
Analysis to Detect ReDoS. In 31th USENIX Security
Symposium, USENIX Security 2022, August 10–12, 2022.
USENIX Association, 2022.
[27] Sérgio Medeiros, Fabio Mascarenhas, and Roberto
Ierusalimschy. From regexes to parsing expression
grammars. Sci. Comput. Program., 93:3–18, 2014.
[28] Microsoft. Regex class - C#, 2020. https://docs.m
icrosoft.com/en-us/dotnet/api/system.text
.regularexpressions.regex?view=net-5.0.
[29] PCRE. PCRE - Perl Compatible Regular Expressions,
2020. https://pcre.org/.

4198

31st USENIX Security Symposium

[30] PHP. PHP: preg_match - Manual, 2020. https://www.
php.net/manual/en/function.preg-match.php.
[31] Asiri Rathnayake. Semantics, Analysis And Security Of
Backtracking Regular Expression Matchers. PhD thesis,
University of Birmingham, UK, 2015.
[32] Asiri Rathnayake and Hayo Thielecke. Static analysis
for regular expression exponential runtime via substructural logics. CoRR, abs/1405.7058, 2014.
[33] Yuju Shen, Yanyan Jiang, Chang Xu, Ping Yu, Xiaoxing
Ma, and Jian Lu. Rescue: crafting regular expression
dos attacks. In Proceedings of the 33rd ACM/IEEE
International Conference on Automated Software Engineering, ASE 2018, Montpellier, France, September 3-7,
2018, pages 225–235, 2018.
[34] Cristian-Alexandru Staicu and Michael Pradel. Freezing
the web: A study of redos vulnerabilities in javascriptbased web servers. In 27th USENIX Security Symposium,
USENIX Security 2018, Baltimore, MD, USA, August
15-17, 2018, pages 361–376, 2018.
[35] Bryan Sullivan. New Tool: SDL Regex Fuzzer, 2010.
http://cloudblogs.microsoft.com/microsoftse
cure/2010/10/12/new-tool-sdl-regex-fuzzer.
[36] Bryan Sullivan. Regular Expression Denial of Service
Attacks and Defenses, 2010. https://docs.micro
soft.com/en-us/archive/msdn-magazine/2010/
may/security-briefs-regular-expression-den
ial-of-service-attacks-and-defenses.
[37] Ken Thompson. Regular expression search algorithm.
Commun. ACM, 11(6):419–422, 1968.
[38] Lenka Turonová, Lukás Holík, Ondrej Lengál, Olli
Saarikivi, Margus Veanes, and Tomás Vojnar. Regex
matching with counting-set automata. Proc. ACM Program. Lang., 4(OOPSLA):218:1–218:30, 2020.
[39] Brink van der Merwe, Nicolaas Weideman, and Martin
Berglund. Turning evil regexes harmless. In Proceedings of the South African Institute of Computer Scientists
and Information Technologists, SAICSIT 2017, Thaba
Nchu, South Africa, September 26-28, 2017, pages 38:1–
38:10, 2017.
[40] Nicolaas Weideman, Brink van der Merwe, Martin
Berglund, and Bruce W. Watson. Analyzing matching time behavior of backtracking regular expression
matchers by using ambiguity of NFA. In Implementation and Application of Automata - 21st International
Conference, CIAA 2016, Seoul, South Korea, July 19-22,
2016, Proceedings, pages 322–334, 2016.

USENIX Association

<!-- page 18 -->

[41] Valentin Wüstholz, Oswaldo Olivo, Marijn J. H. Heule,
and Isil Dillig. Static detection of dos vulnerabilities
in programs that use regular expressions. In Tools and
Algorithms for the Construction and Analysis of Systems
- 23rd International Conference, TACAS 2017, Held as
Part of the European Joint Conferences on Theory and
Practice of Software, ETAPS 2017, Uppsala, Sweden,
April 22-29, 2017, Proceedings, Part II, pages 3–20,
2017.
[42] Xiaodong Yu and Michela Becchi. GPU acceleration of
regular expression matching for large datasets: Exploring the implementation space. In Computing Frontiers
Conference, CF’13, Ischia, Italy, May 14 - 16, 2013,
pages 18:1–18:10. ACM, 2013.

A

Appendix

A.1 Proof of Semantics Preservation After Repair
Lemma 1. Given two regexes r1 = αγβ and r2 = αδβ, if
L (γ) ⊆ L (δ), then L (r1 ) ⊆ L (r2 ).
Proof. Since L (γ) ⊆ L (δ), L (r1 ) = L (α)L (γ)L (β) and
L (r2 ) = L (α)L (δ)L (β), then we have L (r1 ) ⊆ L (r2 ).
Theorem 1. Given a ReDoS-vulnerable regex r,
RegexScalpel returns the repaired regex r′ satisfying
that L (r′ ) ⊆ L (r).
Proof. For repairing vulnerable regexes, RegexScalpel uses
three kinds of strategies (i.e., adding a lookaround/start-ofline anchor, deleting a quantifier/sub-regex and modifying a
quantifier/sub-regex).
According to the repair patterns we proposed, τ1 − τ3 are
based on deleting a quantifier. We can see that these patterns
can only be used when the two languages before and after
deleting the quantifier are equivalent. According to Lemma 1,
we can prove L (r′ ) ⊆ L (r). τ4 is based on deleting a subregex, which can only be used when the two languages before
and after deleting are equivalent. Similarly, we can prove
L (r′ ) ⊆ L (r). τ18 and τ31 are based on modifying quantifiers.
For a sub-regex with a large quantifier, if we replace the quantifier with a smaller one (i.e., 500), it’s obvious that the language of the sub-regex replaced with a small quantifier is the
subset of the language of the sub-regex with a large quantifier.
Therefore, we can prove L (r′ ) ⊆ L (r) according to Lemma 1.
τ9 − τ10 , τ13 − τ15 , τ19 − τ22 , τ25 − τ29 , τ32 , τ35 − τ36 , τ39 − τ40
are based on modifying sub-regex. Among these repair patterns, τ15 can only be used when the two languages before and
after modifying are equivalent. τ27 − τ29 , τ32 rewrite the regex
by removing its nullable sub-regex, meaning that the regexes
after modifying are subsets of the original regexes. Other
patterns given above are based on modifying a sub-regex by

USENIX Association

removing some elements from a character class. Accordingly,
for all of these patterns, L (r′ ) ⊆ L (r) can be proved based
on Lemma 1. τ5 − τ8 , τ11 − τ12 , τ16 − τ17 , τ23 − τ24 , τ33 − τ34 ,
τ37 − τ38 are based on adding a lookaround. According to the
semantics of lookaround, the language of the sub-regex after
adding a lookaround is a subset of the original ones. Therefore, we can prove L (r′ ) ⊆ L (r) according to Lemma 1. τ30
is based on adding a start-of-line anchor ^. Because ^ is equivalent to lookaround (?!.*), it is obvious that the language of
the sub-regex adding a start-of-line anchor is a subset of the
language of the original sub-regex. Therefore, we can prove
L (r′ ) ⊆ L (r) too.

A.2

Proof of Vulnerability-free After Repair

Lemma 2. Given a ReDoS-vulnerable regex r = αγβ where
γ is the pathological sub-regex, RegexScalpel returns the repaired regex r′ = αδβ where δ is a safe regex obtained by
repairing γ with our repair patterns and L (δ) ⊆ L (γ). If δ
contains a construct overlapping with another construct in
sub-regex α or β, then γ contains this type of construct too,
which overlaps with the same construct in α or β.
Proof. For repairing vulnerable regexes, RegexScalpel uses
three kinds of strategies (i.e., adding a lookaround/start-ofline anchor, deleting a quantifier/sub-regex and modifying a
quantifier/sub-regex).
According to the repair patterns we proposed, τ1 − τ3 are
based on deleting a nested quantifier, and τ4 is based on deleting a sub-regex (i.e., an inner sub-regex in a disjunction).
Thus, deleting a quantifier/sub-regex ensures that γ contains
the same type of construct as δ which overlaps with another
construct in sub-regex α or β, and the construct in γ overlaps
with the same construct in α or β too. τ18 and τ31 are based
on modifying quantifiers which replace a large quantifier with
a smaller one (i.e., 500), it is obvious that γ contains the same
type of construct as δ which overlaps with another construct
in sub-regex α or β, and the construct in γ overlaps with the
same construct in α or β too. τ9 − τ10 , τ13 − τ15 , τ19 − τ22 ,
τ25 − τ29 , τ32 , τ35 − τ36 , τ39 − τ40 are based on modifying
sub-regex. Among these repair patterns, τ15 merges the directly adjacent regexes with a quantified sub-regex (i.e. r p
illustrated in τ15 ). τ27 − τ29 , τ32 rewrite the regex by removing its nullable sub-regex. Other patterns given above are
based on modifying a sub-regex by removing some elements
from a character class which will not change the type of regex
construct. Accordingly, for all of these patterns based on modifying sub-regex, we can conclude that if δ contains a construct
overlapping with another construct in sub-regex α or β, then
γ contains this type of construct too, which overlaps with the
same construct in α or β. τ5 −τ8 , τ11 −τ12 , τ16 −τ17 , τ23 −τ24 ,
τ33 −τ34 , τ37 −τ38 are based on adding a lookaround. Because
lookarounds are zero-length assertions which don’t consume
characters, adding a lookaround according to the above repair
patterns will not change the regex construct. τ30 is based on

31st USENIX Security Symposium

4199

<!-- page 19 -->

adding a start-of-line anchor ^. As the proof of Theorem 1, ^
is equivalent to lookaround (?!.*). Accordingly, we can conclude that adding a lookaround/start-of-line anchor ensures
that γ contains the same type of construct as δ which overlaps
with another construct in sub-regex α or β, and the construct
in γ overlaps with the same construct in α or β too.
Therefore, if δ contains a construct overlapping with another construct in sub-regex α or β, then γ contains this type
of construct too, which overlaps with the same construct in α
or β.
Lemma 3. Given a ReDoS-vulnerable regex r = αγβ where
γ is the pathological sub-regex, RegexScalpel returns the repaired regex r′ = αδβ where δ is a safe regex obtained by
repairing γ with our repair patterns and L (δ) ⊆ L (γ). r′ satisfies that no vulnerabilities in the form of our vulnerable
patterns will be newly introduced.
Proof. Suppose the contrary, that is, a new vulnerability in
the form of our vulnerable patterns will be introduced in r′ ,
which means that there are new nested quantifiers or overlapping constructs (i.e., overlapping disjunctions or adjacencies)
in r′ . Apparently, there are no nested quantifiers in the safe
regex δ, so no nested quantifiers will be newly introduced
in r′ . Therefore, there are new overlapping constructs in r′ .
It’s obvious that δ contains the construct which overlaps with
another construct in sub-regex α or β. According to Lemma 2,
we can conclude that γ contains the same construct as δ which
overlaps with another construct in sub-regex α or β. Accordingly, this vulnerability must exist in r too, meaning that the
vulnerability is not newly introduced, which leads to a contradiction. Hence we can conclude that no vulnerabilities in the
form of our vulnerable patterns will be newly introduced in
r′ .
Theorem 2. Given a ReDoS-vulnerable regex r,
RegexScalpel returns the repaired regex r′ satisfying
that there are no vulnerabilities in the form of our vulnerable
patterns.
Proof. By Lemma 3, it can be proved that no vulnerabilities will be newly introduced in r′ . Then the iterative repair
method can ensure that the final repaired regex does not contain any vulnerable patterns.

4200

31st USENIX Security Symposium

USENIX Association
