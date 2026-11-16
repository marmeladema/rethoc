# Bug 60: Prefilter skips the bytes after a counter whose body can match empty

## Bug summary

- **Pattern**: `(e?){40}d` with the default settings; `(e?){3}d`,
  `(e*){1,3}d`, `(e|){2,3}d` with unrolling off
- **Input**: `d`
- **Expected**: `true`; **actual**: `false` (false negative)
- **Affected tier**: the NFA (Tier 0).  A pattern with a counter body that
  can match empty is not eligible for any DFA tier, so the NFA is what
  runs by default.
- **Found by**: differential fuzzing (seeds 633 and 724:
  `((e*|[y])){9,41}d{4,47}` on `dddd`, `a*{10,10}{1,47}.+ *` on `b`).

## Root cause

`compute_prefilter()` walks the epsilon closure of the start state and
collects the bytes of the consuming states it reaches.  It stopped at
`CounterIncrement`:

```rust
// CounterIncrement and Match: stop this branch.
State::CounterIncrement { .. } | State::Match => {}
```

That is right when the body consumes: the increment is then only reached
after a byte, and what follows it is not a start byte.  When the body can
match empty, the increment is reached from the start state without
consuming anything, and the match can go on past it — round the loop
again, or out through the break once enough empty iterations are done.

For `(e?){3}d` the walk found only `e` and the prefilter became
`memchr1('e')`.  On the input `d` the matcher starts in its start
configuration, asks the prefilter for the next `e`, finds none and
returns.  The byte `d` is never stepped.

`(e?){0,3}d` was not affected: `{0,n}` is lowered to `(body{1,n})?`, and
the `?` gives the walk a path around the counter.

## Investigation narrative

1. The fuzz mismatches were NFA-versus-oracle, on patterns with a counter
   around a body that can match empty.  Minimised to `(e?){3}d` on `d`
   with unrolling off.
2. First suspect was the `(state, context)` dedup in the epsilon closure.
   The debug trace cleared it: after the chunk the thread list was
   `{0,0,0,4}` — `e` at counts 0, 1, 2 and `d` — which is the correct start
   closure, still flagged `at_start`.
3. Correct threads and no step meant the byte was skipped before the
   simulation.  `rethoc info` showed `Prefilter: memchr1('e')`.

## Fix

The walk follows both edges of a `CounterIncrement`.  If the increment is
reachable from the start without consuming, its body can match empty, so
`min` empty iterations are possible and the break edge is reachable too;
and a superset of the start bytes is always sound for a prefilter.
Patterns whose counter bodies consume never reach an increment in this
walk, so their prefilters are unchanged.

## What was hard

- The symptom pointed at the simulation (counters, dedup), not at the
  prefilter.  The closure code is correct for these patterns.
- The default unroll limit hides the bug for small counts: `(e?){3}d` is
  unrolled and has no counter.  It needs a count large enough to keep the
  counter (`{40}`) or unrolling off.

## Tooling ideas

- A debug-build check that the prefilter never rejects a byte the start
  closure can consume: compare against the NFA on the first byte.
- Trace output for prefilter skips (`skipped N bytes at offset M`), so a
  byte that is never stepped shows up in `--debug`.
- The brute-force checker over small alphabets (see Bug 55), run on the
  NFA against the oracle and not only tier against NFA.
