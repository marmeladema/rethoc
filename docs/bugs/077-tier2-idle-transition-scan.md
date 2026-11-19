# Bug 77: Tier 2 repeatedly checks counter metadata on idle transitions

The Criterion case `reengage/tier2_word_boundary_gap_once` scans 1 MiB
with `\bfoo.{0,100}bar`. At 7c40d78 it took 3.29 ms, compared with
2.26 ms on 8d0380a, on the AArch64 test machine. Repeating on another
CPU reproduced the slowdown. The match answer was correct.

After the initial gap expires, there are no live counters. The word
boundary prevents a literal prefilter, so the matcher traverses cached
DFA transitions for the rest of the input. On each byte it read the
increment mask and the lengths of both seed lists to identify an
ordinary DFA move. All three values are fixed when the transition is
cached.

The cache now records whether the transition counts or seeds. An idle
matcher checks one flag. A matcher with live instances still performs
resets and phase advancement even on a transition with no seeds or
increments.

Wall-clock comparisons exposed the regression. Callgrind instruction
counts for the original fast path barely changed across the regressing
revisions, and the available measurements did not establish a particular
hardware cause. Measuring the warm scan alongside the instruction gate
lets the two measurements catch different changes.

Tested separately, caching the flag brought this case to 1.39 ms in a
paired Criterion run against 2.27 ms on main. The existing live-counter
and prefilter cases also passed the Callgrind gate.
