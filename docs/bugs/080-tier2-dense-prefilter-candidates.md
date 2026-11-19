# Bug 80: Tier 2 repeatedly searches a suffix that starts with a candidate

For `foo.{0,100}bar` on pseudorandom `f` and `x` bytes, many positions
start a candidate, but none complete `foo`. The 64 KiB fixture fed in
16 KiB chunks took 7–10% more time on 99f950e than on 8d0380a in the
landed-change review. The review's 1 MiB comparison was essentially flat.
Both revisions returned the correct answer.

Each failed prefix returns the DFA to its start state. The bulk scanner
then searches for the next candidate. If the current byte already passes
the prefilter, the search can only return offset zero. Test that byte
directly and leave its consumption to the ordinary DFA transition.
A miss still goes through the bulk finder, which can skip a long run.

The original instruction-count gate improved on the dense fixture, so it
did not reveal the elapsed-time regression. The local benchmark now uses
the same deterministic dense stream at both sizes. The sparse gap and
live-counter controls also matter: delaying the finder indiscriminately
can make those scans do extra work.

On two pinned CPUs the same 64 KiB review fixture took 3.20–3.23 ns per
byte with the fix, versus 4.46–4.54 ns on the merged revision. At 1 MiB
the respective ranges were 4.22–4.25 and 4.48–4.51 ns per byte. The
singleton path remains separate from the specialized bulk prefilter loop.

On the x86-64 CI dense-candidate fixture, checking just the first byte
cut instructions by 14% but raised simulated conditional mispredictions
from 66,087 to 98,911. For a single literal the scanner now checks the
first eight bytes together when available. It finds the first equal byte
with a word-wide zero-byte test, then falls back to the bulk finder on a
miss. Short suffixes and other prefilters retain the scalar check.
