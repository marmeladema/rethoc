# Bug 61: The value of an unbounded counter grows without limit

## Bug summary

A **matching** bug.  Compilation was never affected: both patterns below
compile in under 10 ms, to about 1 KB each.  The cost is paid by the
matcher.

Two symptoms, one cause.  Both with the default settings.

- **Unbounded memory in the matcher**
  - **Pattern**: `(e*){40,}d` (also `(e*){2,}d` with unrolling off)
  - **Input**: any — the empty input and `x` fail the same way as `d`
  - **When**: as soon as a matcher is created.  The start state's epsilon
    closure is computed before the first byte is read, and that closure is
    what does not end.  It is only precomputed at compile time when it
    goes through `Split` states alone (`compute_start_closure()`); with a
    counter or an assertion in the way the matcher computes it itself, at
    creation and again at every re-seed.
  - **Actual**: the matcher allocates until the process is killed
    (625 MB in under a second).
  - **Affected tier**: the NFA.
- **Quadratic matching time**
  - **Pattern**: `(?:a{40,}b){2,5}c`
  - **Input**: a run of `a`
  - **When**: while consuming input; the cost per byte grows with the
    length of the run so far.
  - **Actual**: 32 s for 20 KB; over a minute for 100 KB.
  - **Affected tiers**: Tier 4 (selected by default for this pattern) and
    the NFA (`a{40,}b` forced to Tier 0: 30 s for 20 KB).
- **Found by**: probing the patterns of Bug 60 with larger counts; the
  Tier 4 case by timing unbounded counters on every tier.

## Root cause

A counter increment computed `new_value = value + 1` and then

```rust
let take_continue = new_value < max;
let take_break = new_value >= min;
```

For `{min,}` the maximum is `usize::MAX`.  The continue test never fails,
so the value has no upper limit.  Every value at or above `min` behaves
the same — continue and break are both taken — but the values are
distinct, and threads are deduplicated on `(state, context)`.

- **Body that can match empty**: the increment's continue edge leads back
  to the increment without consuming.  Each time round produces a new
  value, hence a new `(state, context)` pair, hence no dedup hit.  The
  epsilon closure never ends.
- **Body that consumes**: on a long run a thread is started at every
  position, and each has a different count.  None are merged, so there are
  as many threads as bytes in the run, and each byte costs time
  proportional to that.

The compile-time size estimate already assumes the saturated form: it
counts `X{min,}` as `min - 1` copies followed by `X+`.

Tier 2 and Tier 3 keep counts in their own structures and were not
affected.  Bounded counters are not affected either: their values stop at
`max`, and `max_estimated_states` limits the product of nested maxima.

## Investigation narrative

1. With Bug 60 understood, `(e*){40,}d` still exhausted a 1 GB cap on any
   input.  The NFA dump shows the loop `CInc → Split → CInc` with no
   consuming state on it.
2. The dedup is exact on the counter value, so the loop only stops if the
   value stops changing.  For a bounded counter it does, at `max`.
3. The same reasoning says an ordinary unbounded counter keeps one thread
   per start position.  `a{40,}b` on the NFA confirmed it: 1.7 s for 5 KB,
   7.1 s for 10 KB, 29.7 s for 20 KB.
4. Timing the same input on every tier showed Tier 4 with the same curve,
   and `(?:a{40,}b){2,5}c` selects Tier 4 by default.  `tier4.rs` has two
   copies of the increment.

## Fix

A shared helper, `counter_step(value, min, max)`, saturates the value at
`min` when `max == usize::MAX`.  It replaces `value + 1` in the NFA
closure and in both Tier 4 increment paths.

Values at or above `min` are indistinguishable for an unbounded counter,
so the result is unchanged; the number of distinct contexts per state is
now at most `min + 1`.

Measured after the fix: `a{40,}b` on the NFA, 20 KB in 0.08 s;
`(?:a{40,}b){2,5}c` by default, 20 KB in 0.03 s; `(e*){40,}d` on `d`
matches, with the CLI process peaking at 3.7 MB.  The extra test costs under 0.5% instructions on bounded-counter
NFA workloads (Callgrind), nothing measurable on Tier 4.

## What was hard

- Nothing was wrong with any single step.  Each increment is correct; the
  problem is that equivalent states were not recognised as the same.
- The quadratic case gives right answers, so no differential test sees
  it.  It only shows in timing or in the number of live threads.
- The regression tests for the memory symptom cannot fail politely: with
  the fix reverted the test process allocates until it is stopped.  Run
  them under `ulimit -v` when touching this code.

## Tooling ideas

- A memory budget on the counter pool, returning an error instead of
  growing, so that a closure that does not end fails a test rather than
  the machine.
- Thread-count assertions in the test runner: for each `match_tests!`
  input, compare the largest number of live threads against a bound
  derived from the pattern.
- A benchmark of unbounded counters on long runs, per tier, alongside the
  re-engagement gate.
