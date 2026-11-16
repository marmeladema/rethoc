# Bug 59: Break-tail analysis does not terminate on an epsilon cycle

## Bug summary

- **Pattern**: `^y{4,30}((x| *| +))*$` with unrolling disabled
- **Symptom**: compilation allocates without bound (over 1 GB in 0.6 s)
  and aborts.  No input is involved.
- **Affected code**: `break_consuming_tails()` in the Tier 3 analysis, so
  every pattern with non-nested counters, whichever tier it ends up on.
- **Found by**: differential run, seeds 385, 695, 1317, 5756, 5853, 5867
  and 7097; also by the `test_fuzz` proptests, which re-run every pattern
  with unrolling off and can exhaust the machine's memory this way.

This breaks the guarantee that compiling an attacker-written pattern is
bounded in time and memory.

## Root cause

`break_consuming_tails()` walks the epsilon closure of a counter's break
path and records, for each consuming state, the deferred assertions on the
path to it.  A state has to be revisitable, because a second path may
carry a different set of assertions.  Since Bug 49 the rule was: skip a
state only if it was already reached with *strictly fewer* assertions.

Reaching a state again with the same number of assertions was therefore a
new visit.  On an epsilon cycle — a nullable loop such as `( *| +)*` —
the walk comes back to the same state with the same (empty) set forever,
and the work stack grows until allocation fails.

The same rule has a second problem: two paths with different assertions of
equal count are both explored, so `(\b|\B)(\b|\B)…` doubles the work with
every group.  A path with more assertions than another, but not a superset
of them, was discarded although it is a genuine alternative.

## Investigation narrative

1. The first report blamed the wrong seed: the watchdog in the fuzz driver
   printed the seed it had last seen start, and a slow but harmless
   pattern (`b{7,36}{0,36}$`, 15 s) happened to run just before.  Tracing
   each phase put the blow-up at seed 385, in `Regex::with_config` with
   unrolling off.
2. `rethoc info --unroll-limit 0` reproduced it on the original commit.
3. Bug 57's fix needed the same kind of walk for `Advance` targets.  The
   first version copied the length-based pruning and about twenty more
   seeds started to exhaust memory, which pointed at the rule itself.
4. A `gdb` backtrace of the original blow-up ended in
   `break_consuming_tails` at the `stack.push` of the `Split` arm.

## Fix

One walker, `consuming_by_assertion_set()`, now serves both the break-tail
analysis and the `Advance` split of Bug 57.  A state is revisited only
with a set of assertions that none of its earlier sets is contained in.
Coming back with the same set is not a new visit, which ends the walk on
any cycle, and an alternative with more but different assertions is kept.

The number of incomparable sets per state is capped
(`MAX_ASSERT_SETS_PER_STATE`).  Beyond the cap the walk gives up, the
analysis is marked `effects_exact = false`, and the pattern is not Tier 3
eligible.

## What was hard

- Nothing pointed at the analysis: the process simply died, and by default
  an allocation failure gives no backtrace.
- The driver's misattributed seed sent the first round of the
  investigation after the wrong pattern.
- The pruning rule had been introduced as a fix (Bug 49) and read as a
  careful optimisation.

## Tooling ideas

- `fuzz_compile` should run every pattern with unrolling off and with
  merging off as well; it would have found this at once.
- Run the proptest fuzzers under a memory cap so that a compile blow-up
  fails one case instead of the machine.
- A debug-build step counter in every analysis walk, asserting a bound
  that is polynomial in the NFA size.
