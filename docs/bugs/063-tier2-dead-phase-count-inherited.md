# Bug 63: The count of a dead thread is inherited by the next thread at its offset

## Bug summary

- **Pattern**: `(?:a.){40,60}c` with the default settings;
  `(?:a[ab]){2,3}c`, `(?:[ab]a){2,3}c`, `(?:ab|ba){2,3}` with unrolling off
- **Input**: 78 × `a` then `baaac`; `aabaaac`; `aaabaac`; `abaaab`
- **Expected**: `false`; **actual**: `true` (Tier 2 false positive)
- **Affected tier**: Tier 2.  It is the tier selected by default for the
  first pattern.
- **Shape**: a counted body of more than one byte, unanchored, and an
  input in which attempts overlap at different byte offsets.  No
  assertion is involved.
- **Found by**: reading the rules that decide when Tier 2 clears a
  counter, right after Bug 62, and building an input for the gap.  An
  ad-hoc sweep found the `(?:ab|ba)` form independently.  Not hit by fuzz
  seeds 0–8999 or by either `rethoc sweep` family (neither has multi-byte
  bodies).

## Root cause

For a body of `L` bytes, Tier 2 keeps `L` *phases* per counter.  A thread
that entered the body at byte position `p` belongs to phase `p mod L`.
All threads of one phase stand on the same NFA states, so they are stored
as one differential counter.  On every byte exactly one phase is
*active*: its threads stand on the last body byte and cross the increment
on this byte — or die.

Nothing recorded the "or die".  Instances were only cleared by
`counter_reset`, which clears a whole counter, and only when the
successor DFA state holds no body-interior state:

- on a transition that crosses the counter's increment, the counter is
  skipped altogether;
- on any other transition, one thread anywhere inside the body is enough
  to keep every phase.

So when the threads of one phase died while a thread of another phase
was still inside the body, their instances stayed, with their counts.
New threads are seeded into the phase that was just active.  The next
thread to enter at the dead threads' offset therefore shared a phase with
them, and its first increment raised the stale counts as well.

`(?:a[ab]){2,3}c` on `aabaaac`:

```
pos 0-1  "aa"   thread A (even phase): one iteration, count 1
pos 2    "b"    A needs `a` and dies.  Thread B (odd phase, entered at 1)
                completes "ab": a counting transition, nothing is cleared.
pos 3-4  "aa"   B: count 2.  Thread D enters at 4, into the even phase,
                next to A's stale count 1.
pos 5    "a"    D completes "aa".  The even phase is incremented:
                D = 1, A = 2 >= min.  Break.
pos 6    "c"    match
```

In `(?:[ab]a){2,3}c` on `aaabaac` the dying threads fail on the last byte
of their iteration instead of the first; the rest is the same.

## Investigation narrative

1. Bug 62 was a rule about clearing counters whose condition was too
   wide.  Listing the other rules showed that `compute_counter_reset()`
   treats a counter as one unit although it holds `L` independent phases.
2. Worked out by hand an input where one phase dies and the other
   survives, for a two-byte body: `aabaaac`.  It failed on the first try,
   and so did the variant where the last body byte fails.
3. The `--debug` trace prints only the phase that is active next, so
   the stale count is not visible when it appears.  It shows up after
   position 4: three instances with `oldest=1`, where the fixed code has
   one instance with `oldest=0`.
4. A default-settings form followed by scaling the same input:
   78 × `a`, `b`, `aaa`, `c`.

## Fix

A new per-transition mask, `phase_reset`.  A counter's bit is set when
its body is longer than one byte and the transition does not cross its
increment (and `counter_reset` does not clear it anyway).  `step_inner()`
then clears that counter's active phase, before the phase clock advances
and before new instances are seeded.

Why this is enough.  Instances join a phase only when its threads are at
the start of the body.  From there all threads of the phase move
together, and die together.  If they die on the way, no thread of that
phase stands on the last byte when the phase next becomes active, the
transition does not cross the increment for it, and the phase is cleared
— before anything is seeded into it and without ever having been
incremented in between.  A stale instance can sit in a non-active phase
for up to `L - 1` bytes; nothing reads it there except `has_live()`,
which only delays the idle shortcut.

One-byte bodies have a single phase and keep using `counter_reset`.

Cost: one mask test per byte, and one loop iteration per multi-byte
counter that is not counting.  Callgrind gate: 19 cases, none regressed,
instruction deltas between −1.9 % and +1.2 % (the largest increase on
`control_tier2_live_counter`).

## What was hard

- Every step is locally right: increments, breaks and seeds all do what
  they should.  What is missing is a death.
- The stale count is harmless until a thread enters at exactly the same
  offset modulo `L`, so wrong step and wrong answer are several bytes
  apart, and most inputs never show it.
- The reset rule reads as conservative ("keep the instances while the
  body is in progress"), and for one phase it is.

## Tooling ideas

- Sweep families with multi-byte bodies.  Both existing families use
  one-byte bodies or Tier 3 shapes.
- A debug cross-check of Tier 2's counters against the NFA: after each
  byte, the set of counts per phase should equal the counts of the NFA
  threads standing in the body at that offset.  It would report this bug
  at position 2, four bytes before the wrong answer.
- `--debug` could print every phase of a counter; today it prints only
  the phase that is active next.
