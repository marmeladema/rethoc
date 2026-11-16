# Bug 55: Resolved seeds left at a stale origin, or kept alive after their byte

## Bug summary

- **Patterns / inputs** (Tier 3 vs NFA):
  - `^\b(1c?e){2,40}$` on `1e1e` — expected `true`, Tier 3 `false`
  - ` \b(b+ad?a{3,14}1?){9,43}d$` on a space, nine `baaaa`, `d` — expected
    `true`, Tier 3 `false`
  - `^(\b(f?y*1c?e{6,12}){7,40}(e\b.?[aa]?a?)?)?$` on seven `1eeeeee` —
    expected `true`, Tier 3 `false`
  - `^( ?\B((f|y+|.)){5,6}a?(a?a?)?)?$` on ` ffyffd` (unrolling off) —
    expected `false`, Tier 3 `true`
- **Affected tier**: Tier 3, both instance storages.
- **Found by**: differential run over `fuzz_gen` seeds 733, 2720 and 6111.

## Root cause

A deferred assertion in front of a counter (`\b(…){n,m}`) is parked in the
DFA state and resolved in `populate()` once the next byte is known.  A
thread that passes it enters the counter body *before* that byte is
consumed, so its instance belongs at the body's first consuming state with
value 0, and it has to take the step for that byte.

`populate()` did not model it that way.  It produced the seed for after
the step and patched its origin case by case:

- single-byte bodies (the first state's target is an `Increment`) became
  `pre_seeds`, seeded before the per-instance loop — the correct mechanism;
- other seeds were merged into the post-step `seeds`, with the origin
  remapped to the consumed byte's target (Bug 16);
- unless that target was not a consuming state, in which case the original
  origin was kept (Bug 50).

Two consequences:

1. **Stale origin (false negatives).**  When the first body byte's target
   is a split to states that do not include the origin itself (`1c?e`:
   `1` leads to `c | e`), the Bug 50 exception left the instance at `1`
   after `1` had been consumed.  The next byte could not be consumed from
   there and the instance died.  Bug 50's own pattern (`0*a{2,8}`) only
   worked because its split leads back to the origin.
2. **Undead seeds (false positives).**  A resolved seed whose first state
   cannot consume the byte at all was also kept at its origin.  It then
   consumed the *next* byte, as if the assertion had held one position
   later.

## Investigation narrative

1. The fuzz driver printed inputs truncated to 120 bytes, which made seed
   6111 look as if the NFA were wrong too.  Printing the full input and
   shrinking it by byte deletion gave seven repetitions of `1eeeeee`: a
   miss at exactly the minimum count.
2. Reducing the pattern by hand while re-checking the mismatch showed that
   `\b` and the optional `c?` after the first byte were both needed.  A
   brute-force run over all inputs up to nine bytes then gave the minimal
   case `^\b(1c?e){2,40}$` on `1e1e`.
3. `match --debug --tier 3` showed `c0: 1 inst [2@0]` after the first `1`
   while the DFA state was `{3,5}`: the instance was still at `Byte('1')`.
   The passing variant `^\b(1e?){2,40}$` showed `[3@0, 2@1]` — its first
   byte completes the body, so it took the `pre_seeds` route.
4. Reading the resolved-seed code in `populate()` then explained seed 2720
   as well: its three first-byte alternatives (`f`, `y`, `.`) are all
   seeded, only `.` consumes the space, and the other two survive.

## Fix

Every resolved seed whose origin is `reachable_without_break` becomes a
`pre_seed`, on counting and non-counting transitions alike.  The step's
per-instance loop then does what it does for any instance: advance to all
new origins, increment, or drop the instance when its origin has no target
for the byte.  The remap, the non-consuming-target exception and the
value bump for non-counting transitions are gone.

Break-gated origins are still left to the per-instance break effects
(Bug 40), now also on non-counting transitions.

## What was hard

- Three special cases (Bugs 14, 16, 50) each fixed one shape and together
  hid the fact that one of them — seed first, then step — was the general
  answer.
- The trace prints instances as `origin@value` but not whether they were
  seeded before or after the step, so a stale origin looks like a normal
  instance until it is compared with the DFA state by hand.
- The false positive and the false negatives looked unrelated until the
  code was read.

## Tooling ideas

- A debug assertion after each step that every instance origin is in the
  current DFA state's NFA set would have flagged the stale instance at
  once.
- Keep a brute-force checker in the repo: all inputs up to length N over a
  small alphabet, one tier against the NFA.  It turned a 49-byte
  reproduction into a 4-byte one in a second.
- `fuzz_gen` inputs should never be printed truncated in mismatch reports.
