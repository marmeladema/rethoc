# Bug 66: Compiling a pattern that tells all 256 bytes apart panics

## Bug summary

- **Pattern**: `(?-u)\x00yz|\x01yz|…|\xffyz` — 256 alternatives, one per
  byte value (1.8 KB of pattern text).  Default settings.
- **Expected**: compiles; **actual**: panic in `compute_byte_classes()`,
  "more than 256 byte classes"
- **Affected**: compilation of any pattern with at least 128 NFA states
  whose consuming states (and, since Bug 65, assertions) put every byte
  in a class of its own.  No tier is involved; the pattern never gets to
  match.
- **Found by**: review of the Bug 65 fix.  Present since byte classes
  were added.

## Root cause

`compute_byte_classes()` numbers the classes as it meets new signatures:

```rust
let mut next_class: u8 = 0;
...
let class = next_class;
class_map.insert(sig, class);
mapping[b as usize] = class;
// Safety: we cannot have more than 256 distinct classes.
next_class = next_class
    .checked_add(1)
    .expect("more than 256 byte classes");
```

The comment is right: there are at most 256 classes, numbered 0 to 255.
But `next_class` is the *count* of classes handed out so far, and after
class 255 the count is 256, which a `u8` cannot hold.  The increment
after the last possible class is the one that panics.

Compilation must not panic on any pattern (invariant 2 in AGENTS.md).

## Investigation narrative

1. The reviewer gave the pattern.  `rethoc info` on it panics, on this
   branch and on `main`.
2. With fewer than 256 distinct classes the last increment never happens,
   which is why 255-way patterns and everything in the suite pass.

## Fix

The class index is taken from the number of signatures seen so far
(`class_map.len()`, a `usize`) before the new one is inserted, converted
to `u8` — which cannot fail, since at most 256 signatures exist — and the
function returns `class_map.len()` as the count.  There is no counter to
overflow.

`test_byte_classes_all_256_distinct` compiles the pattern, checks that
every byte has its own class and matches a few inputs.

## What was hard

- Nothing, once seen.  The guard was written for an impossible case
  (a 257th class) and fired on a legal one (the 256th).
- Reaching it needs a pattern that is both large and distinguishes every
  byte.  The Bug 65 fix makes that easier, since assertions now split
  classes too.

## Tooling ideas

- `fuzz_compile` generates patterns from a grammar with a small alphabet;
  it never produces 256-way byte distinctions.  A few hand-written
  extreme patterns per limit (every byte, longest literal, deepest
  nesting) in the compile-robustness tests would cover what the grammar
  does not.
- An index and a count of the same things differ by one at the top of
  the range.  Deriving the count from the container, as here, leaves no
  second variable to get wrong.
