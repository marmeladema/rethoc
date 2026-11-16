# Bug 57: Post-break tail crosses a deferred assertion into a counter body

## Bug summary

- **Patterns / inputs** (unrolling off, Tier 3 `true`, expected `false`):
  - `^(.{2,3}1\b)+$` on `111111`
  - `^.{2,4}1\b.{2,3}$` on `111 111`
  - `^(\b.{4,9}1\b)+$` on `cx c1adday1`
- **Affected tier**: Tier 3.  Tier 2 handles the first pattern correctly.
- **Found by**: differential run, seed 3736.

## Root cause

After a counter breaks, the consuming states on its break path are tracked
as post-break tails.  When a tail consumes a byte, the effect compiled for
the byte's target tells where the thread goes next.  For an `Advance`
target that was the list `new_origins`, computed by `analyze_target()`,
which walks through every assertion as if it held.

The runtime pushed all of `new_origins` as tails at once.  The assumption
was that an origin behind an assertion that later fails would not be in
the next transition's `origin_keys` — the DFA parks the deferred assertion
in its state and resolves it itself — so the tail would be dropped.

That holds only if nothing else keeps the origin in the DFA state.  A
counter-body state is kept there by the counter's own instances.  So a
thread that consumed `1` and then met `\b` in front of a counter (the loop
back to the same counter, or another counter) found its origin present,
consumed the next byte and was handed to the counter with a count of one
— whether or not `\b` held.

## Investigation narrative

1. Shrinking gave `cx c1adday1`: the group can only end at the `1` in the
   middle if `\b` holds between `1` and `a`.
2. The trace after that `1` showed `tails: [2,5]` — a tail on the body
   state `.` — next to the correctly guarded `Match` effect for the `$`
   branch.  One byte later the counter had a second instance, `2@1`.
3. `dump --dfa` showed `state 6: Advance → [2]` for the assertion state.
4. A two-counter variant passed the first brute-force run, which looked
   like a different mechanism.  Tracing it showed the same unconditional
   tail, dropped only because its origin was missing from the DFA state.
   Giving the first counter one more iteration (`{2,4}`) lets two break
   points coexist, keeps the origin alive, and reproduces the bug.

## Fix

`compile_all_target_effects()` now splits every `Advance`: origins reached
without crossing a deferrable assertion stay in `new_origins`; origins
reached only through such assertions become guarded effects, resolved at
the next byte like the guarded effects of a counter break:

- a guarded `AddSeed` with value 0 when the path enters a counter (the
  thread has consumed nothing inside the body yet);
- a guarded `AddTail` otherwise.

Both tail-processing sites enqueue these guarded effects.

The split enumerates the minimal sets of assertions in front of each
origin.  That number is bounded per NFA state; a pattern that exceeds the
bound gets `effects_exact = false` and is not Tier 3 eligible.

A first version emitted `AddTail` for origins inside a counter as well.
A resolved tail that lands on an `Increment` is seeded into the current
buffer with its count already raised, and the step then counts the same
byte again; `111 1` matched `^(.{2,3}1\b)+$`.  Entering a counter is
therefore always a seed.

## What was hard

- The unconditional advance is correct almost always, and when it is wrong
  it is wrong silently: the tail looks like any other.
- The two-counter variant needed a specific overlap of two break points,
  so "passes on all inputs up to nine bytes" was true and misleading.
- The first fix exposed a latent double count in the resolved-tail
  hand-off, which had to be understood before the fix could be trusted.

## Tooling ideas

- Show in the trace which tails were pushed unconditionally and which came
  from a resolved guard.
- A debug check that a tail on a counter-body state was produced by a
  hand-off, never by an `Advance`.
- Sweep small pattern templates (prefix, counted body, middle, second
  counter, suffix, optional `+`) through the brute-force checker; 63 of
  10,848 such patterns were wrong before Bugs 55 to 58 were fixed.
