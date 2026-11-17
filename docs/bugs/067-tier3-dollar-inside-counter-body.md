# Bug 67: Tier 3 lets `$` inside a counted body pass in mid-input

## Bug summary

- **Pattern**: `(?:a$|bb){2,3}` (unrolling off); the same shape with a
  count too large to unroll, `(?:a$|bb){40,60}`, runs on Tier 3 with the
  default settings
- **Input**: `aa`
- **Expected**: `false`; **actual**: `true` (Tier 3 false positive).
  `a$` can only end an iteration at the end of the input, so `aa` is at
  most one iteration.
- **Affected tier**: Tier 3.  Tier 4 and the NFA are correct.
- **Found by**: checking where the patterns of Bug 68 would run once
  Tier 2 no longer took them.  A body with `$` and a one-byte body went
  to Tier 2 before, which hid this for the audit's patterns; with a body
  of variable length it has been on Tier 3 all along.

## Root cause

Tier 3 was only kept away from bodies containing a *deferred* assertion
— one that can ask to wait for the next byte: the word-boundary family
and `(?m:$)`.  `$` is not one of those: it never waits, it only needs to
know whether the input has ended.  So a body with `$` passed the check
(`body_has_deferred()`, a hand-written list of kinds).

Tier 3's analysis, however, has no place for an assertion inside a body.
What follows a consumed byte is computed once per NFA state, with `$`
handled as "match at end of input" for the paths that lead to `Match`.
A `$` in front of the body's increment is not such a path, and the
iteration was counted as if the `$` were not there.

## Investigation narrative

1. With the Tier 2 rule of Bug 68 in place, `(?:a$|b){40,60}` moved from
   Tier 2 to Tier 3.  Brute force over `ab` up to length 8 on the shapes
   with `$` found mismatches on six of seven, all on Tier 3.
2. `(?:a$|bb){2,3}` on `aa` gave the same result on `main`, where the
   pattern is on Tier 3 by default.
3. The check that should have excluded it lists the kinds it looks for;
   `End` is not in the list.

## Fix

The body check (`BodyAssertions::lookahead`, computed by
`body_assertions()`) asks the assertion itself: `AssertKind::looks_ahead()` is true for every kind whose outcome
can depend on what follows the position — the next byte, or whether the
input ends there.  That is every kind except `^` and `(?m:^)`.  The
match is exhaustive, and a test checks it against `eval()` for every
kind and every pair of bytes.

Bodies with `$` now run on Tier 4, which evaluates `$` on its own path
and is correct on every shape tried (the `assert-in-body` sweep).  On
1 MB of input `(?:a$|bb){40,60}` takes 0.23 s there against 0.08 s on
Tier 3.

The same check feeds two of Tier 2's requirements (no such assertion in
a body longer than one byte, or with several counters), so `$` counts
there too.

## What was hard

- A one-byte body with `$` went to Tier 2, which was wrong in another
  way (Bug 68).  Only a body of variable length showed the Tier 3 bug by
  default, and no test, sweep or fuzz seed had one.
- The list of kinds was correct for the name of the function
  ("deferred").  The question it was used to answer — can Tier 3 take
  this body — needed a different property.

## Tooling ideas

- `rethoc sweep assert-in-body`: assertions of every kind inside a
  one-byte body, in front of and behind the byte.  On the engine before
  this change 585 of its 4,560 runs fail (390 on Tier 2, 195 on Tier 3).
- When a pattern is taken away from a tier, check the tier that gets it.
  This bug and Bug 69 were found that way; `brute` on a handful of
  shapes was enough.
