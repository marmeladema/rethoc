# Bug 58: Resolved tail kept alive when it cannot consume the byte

## Bug summary

- **Pattern**: `^ {2,3}\b(a|bc){2,3}$` (unrolling off)
- **Input**: two spaces, then `abcaa` (four iterations)
- **Expected**: `false`; **actual**: `true` (Tier 3 false positive)
- **Affected tier**: Tier 3.
- **Found by**: brute-force check written to probe Bug 57; not hit by the
  fuzz seeds.

## Root cause

A guarded `AddTail` is resolved in `resolve_pending_effects()` once the
next byte `b` is known.  The assertion is evaluated against `b`, so the
tail's thread stands right before `b` and must consume it.

When the tail could not consume `b`, the code kept it: it was pushed to
`resolved_tails` and so into `post_break_tails` for the following step.
The thread then consumed a later byte instead, as if the assertion had
held one position further on.

In the example the counter `(a|bc){2,3}` is entered behind `\b`.  The
break path of the first counter yields a guarded seed for the body and a
guarded tail for `b` (the first byte of `bc`, whose target is not an
increment).  The first byte is `a`: the tail cannot take it and is kept.
It consumes the `b` that follows, then `c`, and is handed to the counter
with a count of one — one iteration short of the truth.

## Investigation narrative

1. While fixing Bug 57 a resolved tail landing on an `Increment` turned
   out to be double-counted.  Looking for a pre-existing shape that puts a
   guarded tail inside a counter body led to a two-byte alternative behind
   `\b`.
2. The brute-force run found four mismatches, the shortest seven bytes
   long, with the same result on `main`.
3. The trace showed `tails: [6]` surviving the `a`, then `tails: [7]`, then
   two extra instances `5@1, 6@1` after `c` next to the correct
   `5@2, 6@2`.
4. The culprit is one `else` branch commented "Tail can't consume this
   byte — keep as post_break_tail".

## Fix

The branch is removed: a resolved tail that cannot consume the byte it was
resolved against is dropped.

## What was hard

- The branch looked deliberate, and the comment at the call site described
  it as a feature.  Neither the Bug 44 post-mortem nor the code gives a
  case that needs it.  The suite passes without it, after correcting one
  unit test that had been written against the old behaviour.
- It only matters when the origin is still present in a later transition,
  which again means a counter-body state.

## Tooling ideas

- Trace lines for resolved effects: which guards passed, which tails
  consumed the byte, which were dropped.
- The brute-force checker over small alphabets (see Bug 55).
