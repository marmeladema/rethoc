# Bug 74: Tier 2 loses the position of a parked assertion

Tier 2 rejected `aa` for `\b^a{2,40}` under the default settings. The NFA
and the regex oracle accept it. With `(?m:^)` in place of `^`, it also
rejected a match immediately after a newline. Whole and bytewise feeds
give the same results. Tier 3 still fails these cases.

The initial DFA closure parks the word boundary until the first byte
arrives. Tier 2 then evaluated its continuation with `at_start=false`.
It also kept only the wordness of the previous byte, which lost the
distinction between space and LF. Evaluating each assertion independently
does not exercise this defect because the word boundary itself needs
only wordness.

A comparison with the assertions from Bug 70 exposed the discrepancy.
Tier 2 computes EOF acceptance for each parked assertion at the physical
boundary. It then interns only the previous-byte and input-start facts the
pending continuations can read with a known next byte. The summary follows
counter entries and stops at increments, whose break decision belongs to
the matcher. Tier 1 uses the same assertion analysis. The four nonword
predecessors space, LF, CR and hyphen thus share a state for a lone word
boundary; a line-start continuation still distinguishes LF and input start.

The regression cases check input-start, LF, space and CR. They include
a negative mid-input case to distinguish a genuine initial boundary from
a cached state encountered again. Showing the retained context in the
bytewise trace makes the original loss visible before the missing match.
