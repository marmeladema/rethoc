# Bug 79: Tier 2 keeps looking for a restart after an anchored gap expires

The bytewise scan of `^foo.{0,100}bar` on `foo` followed by a long run
of `x` correctly returned false. On the reviewer's AArch64 benchmark,
99f950e spent about 13% more elapsed time per byte than 8d0380a. This
reproduced on both 64 KiB and 1 MiB streams.

The trace reaches DEAD after the bounded gap and all possible suffix
threads expire. Ordinarily Tier 2 must try to restart from DEAD: a line
anchor, for example, may pass again after LF. For this pattern every
restart is impossible. The matcher still visited its dead-transition
cache on the remaining bytes, including on every new one-byte chunk.

The compiler records whether the HIR requires absolute input start on
every path. When asked to advance from DEAD on that kind of stream, the
matcher records an exhausted search. A state with a pending EOF match
retains its own DFA state until the next byte. Advancing from DEAD
clears the previous EOF flag. Fresh streams
using the same cache start a new search.

The regression test checks that no restart transition is populated after
absolute-start exhaustion. A mixed absolute-start/line-start pattern
must still restart after LF. The local Criterion fixtures time both
input sizes with one-byte chunks; the existing anchored instruction
gate measures bulk feeding. The timings establish the slowdown and the
improvement; they do not isolate a hardware cause for the difference
between the two earlier binaries.

With the fix, the reviewer’s harness took 0.77–0.80 ns per input byte on
two pinned CPUs at both sizes, versus 4.20–4.26 ns on the merged revision.
These are medians over eleven shuffled process runs per combination.
