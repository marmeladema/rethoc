# Bug 68: Tier 2 cannot count an iteration that ends in an assertion

## Bug summary

Four families of wrong answers, all with the default settings and all
with an assertion between the last byte of a counted body and the count:

| Pattern | Input | Expected | Tier 2 |
|---|---|---|---|
| `^(?:a\B\|b){40,60}c` | 39 × `b`, then `ac` | `true` | `false` |
| `^(?:a\B\|b){1,40}c` | 40 × `b`, then `ac` | `false` | `true` |
| `b(?:[ab]\B){40}c` | `bab`, 39 × `a`, `c` | `false` | `true` |
| `a(?:[ab]\B){1,40}c` | `aa`, 40 × `b`, `c` | `true` | `false` |
| `(?:a\B){1,40}\bc` | `ac` | `false` | `true` |
| `(?:a\b){1,40}(?m:$)` | `a\n` | `true` | `false` |
| `x(?:a$){1,40}` | `xa` | `true` | `false` |

- **Affected tier**: Tier 2, the default tier for these patterns.
- **Found by**: the Tier 2 audit sub-agent (families N1–N4 of its
  report), with sweeps over one-byte bodies containing assertions.

## Root cause

Tier 2 rests on one assumption: a counter is incremented on the
transition that consumes the last byte of its body — once per byte, and
for all threads of a phase together.

An assertion behind that byte breaks it.  In `(?:a\B){2,3}` the thread
has consumed `a` and now waits: whether the iteration counts is only
known when the next byte arrives.  On that next transition:

- the pending increment is applied, and the same transition may then
  consume a byte and reach the increment again (`(?:a\B|b)` on `ab`):
  two increments of one counter on one byte, with a possible break in
  between.  A transition has room for one;
- a thread that waits and a thread that enters the body on the same
  byte end up in the same phase, although one has an increment pending
  and the other does not;
- when the waiting thread dies, its count has to go; when the input
  ends, `$` and `(?m:$)` have to be evaluated on a path the transition
  table does not represent.

The module documentation lists "no deferred assertions inside counter
bodies" as a requirement.  The compiler made an exception for one-byte
bodies, and `populate()` grew special cases for it (resolved increments,
break targets consumed during resolution, a seed re-injected after a
reset, a second look at the counters at end of input).  The four families
are the places where those special cases do not add up.

An assertion *in front of* the body's byte is different: a thread
waiting there has not consumed anything, so there is no increment
pending.  `(?:\B[ab]){2,3}` works, and keeps working.

## Investigation narrative

1. The audit report gave the families with traces.  All reproduced.
2. The audit also showed the boundary: of its bodies, those with the
   assertion only in front were clean in every family; every failure had
   an assertion behind the byte, on at least one branch.
3. Stating what the failing bodies have in common in terms of the NFA:
   the increment is reachable from an assertion without consuming.

## Fix

A sixth eligibility requirement: every increment directly follows a
consumed byte.  `body_assertions()` walks each body once, collects its
assertions, and reports whether the increment can be reached through
epsilon transitions from one that looks ahead
(`AssertKind::looks_ahead()`): `BodyAssertions::increment_behind_lookahead`.
Such a pattern is not a Tier 2 pattern.

What stays on Tier 2, by test: assertions outside the counter
(`\b\d{40,60}\b USD`), in front of a one-byte body (`(?:\B[ab]){40,60}c`,
`(?:\Ba|b){40,60}c`), and `^` / `(?m:^)` anywhere, which never wait.

Where the others go: a word boundary in a body is taken by no counting
tier, so those run on the NFA; `$` runs on Tier 4.  On 1 MB of input,
`(?:[ab]\B){40,60}c` takes 0.95 s on the NFA against 0.02 s on Tier 2.
That is the price of the right answer for a rare shape: `(?:x\b){n}` can
only repeat when the body mixes word and non-word bytes.

The special cases in `populate()` and `finish()` became unreachable and
are removed, with the analysis tables only they read (the consuming
states and the match reachability of each break path): resolved
increments, break targets consumed during resolution, the seed
re-injected after a reset, the second look at the counters at end of
input.  A compiled regex is 48 bytes smaller.

What they assumed is now checked where it can be decided, at compile
time.  `compute_tier2_analysis()` panics if an increment is reachable,
without consuming, from an assertion that looks ahead.  It reads the
table the matcher uses (`cinc_reachable`), not the eligibility walk, so
the two cannot drift apart silently; a pattern on which they disagree
never reaches a matcher.

## What was hard

- Each family looked like a separate defect with its own trace.  What
  they share only shows when the body is described by where the
  assertion sits relative to the byte.
- The tempting fix is to repair the four mechanisms.  Each repair would
  be another special case in a tier whose transitions have no way to
  say "this counter moves twice".
- Rejecting too much is as easy as rejecting too little: a rule that
  excludes every assertion in a body would have sent the working
  front-of-byte shapes to the NFA as well.  Two entries pin those to
  Tier 2.

## Tooling ideas

- `rethoc sweep assert-in-body` (see Bug 67).
- A list, kept with each tier, of what its transitions can express,
  checked against the eligibility conditions.  "One increment per
  counter per byte" was true of the data structure and not of the
  patterns it was given.
