# Bug 69: Tier 3 and a line anchor behind a byte

## Bug summary

Three symptoms, two causes, one place in the analysis.  The first cause
is fixed.  The second is a design defect of Tier 3 and is recorded, not
fixed: see "What is left".

- **False negative** (release builds, default settings)
  - **Pattern**: `(?:ab|\n){20,30}(?m:^)`;
    `(?:a|\n){40,60}(?m:^)(?:c|bb){20,30}`; unrolling off,
    `[^a]{2,3}(?m:^)`
  - **Input**: 19 × `ab`, `\n`, `x`; 39 × `a`, `\n`, 20 × `c`; ` \n`
  - **Expected**: `true`; **actual**: `false`.  What follows a
    counter's break behind a line anchor — the match, a second
    counter — never happens.
  - On `main`; not hit by any test, fuzz seed or sweep.
- **False positive** (release and debug builds; **not fixed**)
  - **Pattern**: `(?:a(?m:^)|\n\n){2,3}c` (unrolling off); with a count
    too large to unroll, `(?:a(?m:^)|\n\n){20,30}c`, it ran on Tier 3
    with the default settings
  - **Input**: `a\n\nc`
  - **Expected**: `false`; **actual**: `true`.  `a(?m:^)` can never
    hold: the position after `a` is not the start of a line.
- **Panic at compile time** (debug builds only): "unexpected deferred
  assert kind StartLF in target deferred", for any Tier 3 pattern with
  `^` or `(?m:^)` behind a byte, inside a counted body or not:
  `(?:a|bb){20,30}\n(?m:^)c`, `\n(?m:^)(?:a|bb){20,30}c`,
  `(?:a|bb){20,30}a^c`.
- **Affected tier**: Tier 3.  Tier 2, Tier 4 and the NFA are correct.
- **Found by**: the debug-build test run of the Bug 68 change, whose new
  unit tests compile `(?:\n(?m:^)){2,3}c` (panic, then the false
  positive).  The first fix only covered bodies; a review pointed at the
  same panic outside them.  The false negative was found last, by a
  mutation check of the second fix: no test noticed when the guard was
  given word-ness only again, so there had to be a pattern where it
  matters, or the change was not needed.

## Root cause

When Tier 3 analyses what follows a consumed byte, it walks the epsilon
transitions behind it.  Two things happen on that walk.

**Guards.**  An assertion met on the way (other than `$`) becomes a
*guard* on the effects found behind it: seeds, break tails.  The guard
is evaluated one byte later, when the next byte is known.  The only
context kept for that evaluation was whether the previous byte was a
word byte, and three debug assertions said that the kind had to be one
of those the DFA tiers defer.  `^` and `(?m:^)` are not.  They need
nothing from the next byte, but they do need the previous one, and
"word byte or not" does not say whether it was `\n`.  In a release
build the guard was evaluated with a stand-in for the previous byte
that was never `\n`, so `(?m:^)` as a guard always failed.  Guards
in front of a counter do not notice: `\n(?m:^)(?:a|bb){20,30}c` is
answered correctly on that build, the threads behind the anchor being
in the DFA state anyway.  Guards behind a counter's break do: what a
break leads to exists only as a guarded effect, so with a body that can
end in `\n`, nothing behind `(?m:^)` was ever reached.

**Increments.**  A consumed byte is classified as *incrementing* when a
`CounterIncrement` can be reached behind it through epsilon transitions,
assertions included.  The count is applied on that byte.  If an
assertion sits in between and fails, the thread is gone from the DFA
state, but the count stays, in an instance that later threads share.
`(?:a(?m:^)|\n\n){2,3}c` on `a\n\nc`: `a` counts one although
`(?m:^)` fails behind it, `\n\n` makes two, break, match.  For
assertions that look ahead this shape was already excluded (Bug 67);
for `^` and `(?m:^)` it was not.

This is one of four paths of the same kind; see "What is left".

## Investigation narrative

1. The debug assertion named the kind and the place.
2. In a release build, brute force over `ac\n` found the false positive
   for bodies with `(?m:^)` behind a byte, on Tier 3 only, and none for
   bodies with it in front.  The first fix excluded every body with an
   assertion behind a byte.
3. The review compiled `(?:a|bb){2,3}a(?m:^)c` in a debug build: same
   panic, no body involved.  So the assertion was about the guard, not
   about bodies.
4. With the guard given the previous byte's full properties and the
   debug assertions gone, the eligibility rule was switched off and the
   shapes brute-forced again.  Outside bodies and between two body
   bytes: correct.  Between a byte and the increment: still wrong.
   That separated the two causes.
5. The mutation check of the new code: reducing the guard's context to
   word-ness again failed no test.  Brute force over shapes with a line
   anchor right after a counter's break then found the false negative,
   on `main` as well.
6. The control for that sweep, a build of the previous commit from a
   second worktree, agreed with the fixed build.  It was the fixed
   build: on this machine cargo shares one build directory between
   projects, and the binary had been linked against the other
   worktree's engine.  Its compiled-regex size gave it away.  With a
   build directory of its own the control shows 32 mismatches in
   `assert-edges`.

## Fix

- **Guards**: a pending effect carries the `ByteProps` of the previous
  byte (word, `\n`, `\r`) instead of one bit.  When the guard is
  evaluated, `ByteProps::stand_in()` gives a byte with those properties
  and `AssertKind::eval()` decides.  Every kind but `$` is a legal
  guard; the three debug assertions are removed.
- **Increments**: not fixed.  Two rules were tried, each excluding the
  bodies that had failed so far ("an assertion behind a byte", then "an
  assertion between a byte and the increment"), and each was too
  narrow.  Both are gone again; the failing patterns are test entries
  marked `tier3_broken`.

## What is left

The increment rule was derived twice from the inputs that failed, and
both times it was too narrow.  Reading the algorithm instead gives the
class, and shows that the second rule covered a quarter of it.

Tier 3 has two evaluators.  The DFA state is built by the same closure
as in Tiers 1 and 2, which evaluates every assertion exactly.  Counter
instances are moved by static tables, built by walks over the NFA that
step through every assertion inside a body without evaluating it.  The
only check on an instance is that its new state is in the DFA state.
That validates the state, not the count: when another instance
legitimately stands on the state behind an anchor, the instance that
should have died there lives on with its own count.

Inside a body there are four such paths:

| Path | Walk | Status |
|---|---|---|
| byte → increment | `analyze_target` | wrong: `(?:a(?m:^)\|\n\n){20,30}c` matches `a`, 19 × `\n\n`, `c` |
| byte → next byte | `analyze_target` (`advance_origins`) | wrong: `(?:(?:px(?m:^)\|x)a){20,30}c` matches 20 × `pxa`, `c` |
| increment → start of the next iteration | `consuming_states_from` (`continue_origins`) | wrong: `(?:(?m:^)[ab]+[b\n]){20,30}c` matches `\na`, 40 × `b`, `c` |
| counter entry → first byte | `consuming_states_from` (`ci_origins`) | same mechanism; no example built |

The examples run on Tier 3 with the default settings, on `main` as
well.  They are in `match_tests!` as `tier3_broken` entries: every
other engine is checked on them, and Tier 3 must still be wrong, so the
marker has to go when the defect does.  Two of them stand next to a
fixed-length twin that runs on Tier 2 and is right: Tier 2 has one
evaluator, and all instances of a phase stand on the same states.  The
`assert-in-body` sweep reaches the first path in 480 of its runs; CI
checks it without Tier 3 (blocking) and on Tier 3 alone (reported).

No rule is added for them.  The sound rule would be "no assertion in a
Tier 3 body", which also takes `(?:(?m:^)a+\n){20,30}c` off the tier;
the alternative is to evaluate `^` and `(?m:^)` on these paths, which
is possible (they read only the byte just consumed) and is a change to
the step loop.  Either belongs to a redesign of Tier 3, not to another
exclusion.

## What was hard

- The debug assertion had been there all along and nothing compiled a
  pattern that reaches it.  A check that never runs protects nothing.
- The first fix made the symptom go away for the patterns at hand and
  was written up as the cause.  It was a rule about bodies for a defect
  that had nothing to do with bodies, and it hid a second defect that
  did.
- The second rule came from an experiment: switch the first rule off
  and see what still fails.  It found the position that failed on the
  shapes at hand, and was written up as the position that conflicts
  with the tier.  All positions do; the sweeps had no body with a
  second way into the state behind the anchor.
- An exclusion that makes a sweep go quiet looks like a fix.  Tier 3
  has had several; they are why its defects were hard to see as one.

## Tooling ideas

- The fuzz generator emits `\b` and `\B` only.  Line anchors would
  have reached the debug assertion in the first `cargo test test_fuzz`
  run.
- The differential tools ran release binaries only.  Done here: every
  sweep runs once more in CI on short inputs with a debug build
  (`rethoc sweep --max-len 4`), `assert-in-body` has line anchors
  behind a byte before, inside and after the body, and `assert-edges`
  has `(?m:^)` behind the counter and a second counter as suffix.
- A control build has to be identified before it is believed: `rethoc
  info` could print the engine's commit.
- After a fix by exclusion, switch the exclusion off and look at what
  still fails.  That experiment told the two causes apart; it did not
  show where the second one ends.  A rule has to be derived from what
  the algorithm can represent, with the walks listed, before a sweep's
  silence means anything.
- A mutation that no test catches is a finding, not a formality: here
  it led to the only symptom that affects release builds on `main`
  with default settings.
