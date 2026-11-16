# Bug 56: Unconditional seeds applied although the clean chain is dead

## Bug summary

- **Pattern**: `^(c?1{2,3}.{2,3}?0)+` (unrolling off)
- **Input**: `1110c11cc0`
- **Expected**: `false`; **actual**: `true` (Tier 3 false positive)
- **Affected tier**: Tier 3, patterns with at least two counters.
- **Found by**: differential run, seed 3271 (`^(c?1{9,44}.{10,27}?0)+`).

## Root cause

When some instance breaks, Tier 3 moves to the `with_break` DFA state,
whose closure follows the break path of *every* counter increment it
meets — also of counters that have not reached their minimum.  The state
is then "contaminated": it lists NFA states that no thread occupies.  For
patterns with several counters the matcher keeps a second, clean no-break
chain (`clean_nb`) to tell which states are genuinely counter-free, and it
filters the unconditional `seeds` and `pre_seeds` of a transition against
the clean chain's transition (Bugs 32 and 48).

The filter was only applied when `clean_nb_trans_slot` was `Some`.  That
slot is `None` in two situations: the state is not contaminated, or it is
contaminated and the clean chain is dead.  In the second situation there
is no counter-free thread at all, so every unconditional seed is spurious
— and that was exactly the case in which the filter was skipped.

In the example, `1{2,3}` breaks on the third `1` while `.{2,3}` increments
on the same byte, so the `with_break` state contains `Byte('0')`, the
break target of `.{2,3}`, although that counter is at 1.  The stray `0`
is consumed from there, `c` follows, and the transition on `c` seeds
`1{2,3}` again: a second round of the group starts in the middle of the
first.  The pattern is anchored, so the clean chain died after the first
byte.

## Investigation narrative

1. Shrinking the fuzz input left ten `1`, `0c`, twenty-six `1`, `0`: one
   short of what `.{10,27}` can cover.
2. The trace showed `c0: 1 inst [3@0]` after the `c` at offset 11 — a new
   instance of the first counter — with `tails: []` and `clean_nb=DEAD`.
3. The seed came from the transition's `seeds`.  `filter_seeds` was
   `t.is_counting && self.clean_nb_trans_slot.is_some()`.
4. Small bounds first seemed not to reproduce.  The contamination needs
   the first counter to break on the byte on which the second one
   increments, which takes three `1` for `{2,3}`; the shortest failing
   input is ten bytes long and the first brute-force runs stopped at nine.

## Fix

The filter is driven by the contamination of the state the step starts
from.  A dead clean chain leaves the set of clean seed origins empty, so
all `seeds` are suppressed; `pre_seeds` are skipped the same way.  Threads
that really re-enter the group are tracked as post-break tails and reach
the counter through the tail hand-off, as before.

## What was hard

- `None` meant two opposite things, and the code read naturally in the
  wrong way ("no clean transition to compare with, so nothing to filter").
- The false positive needs both counters to act on one byte, so inputs
  built from the pattern's obvious shape do not trigger it.

## Tooling ideas

- Print in the trace why a seed was applied (clean, filtered, break) and
  mark contaminated-with-dead-clean-chain states explicitly.
- Make the clean-chain lookup return a three-valued result
  (`NotContaminated`, `Clean(slot)`, `Dead`) instead of an `Option`.
