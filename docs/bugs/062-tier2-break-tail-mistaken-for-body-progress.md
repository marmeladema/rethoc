# Bug 62: A byte taken after the counter is mistaken for the counter's first byte

## Bug summary

- **Pattern**: `\b\d{40,60}\b USD` with the default settings;
  `\ba{2,3}\b c` with unrolling off
- **Input**: forty `7`s, then ` 7 USD`; `aa a c`
- **Expected**: `false`; **actual**: `true` (Tier 2 false positive)
- **Affected tier**: Tier 2.  It is the tier selected by default for the
  first pattern.
- **Shape**: a word-boundary assertion in front of the counter, another
  one right after it, then at least two more bytes.
- **Found by**: `rethoc sweep tier3-assert` (8 of its patterns, all of
  the form `\b(x{2,3}\b c)+…`).  Present since before the sweep existed;
  no fuzz seed in 0–8999 hits it.

## Root cause

A deferred assertion (`\b`) is parked in the DFA state and resolved on the
next transition, once the next byte is known ("Phase 1" of
`Tier2DfaCache::populate()`).  If the resolved path enters a counter and
the counter's first body byte takes the current byte, the transition gets
a *pre-seed* for that counter and the counter is exempted from
`counter_reset`.  If the body does not take the byte the seed is
speculative and must be dropped (Bug 14).

The test for "the body took the byte" was one flag for the whole
transition:

```rust
for &idx in cr.nfa_states.iter() {
    if let Some((t, te)) = consume_byte(idx, byte, regex) {
        targets.push(t);
        ...
        resolved_body_consumed = true;
    }
}
```

`cr.nfa_states` holds every consuming state reached from *any* assertion
resolved on this transition.  The flag says that one of them consumed the
byte, not which.

In `\ba{2,3}\b c` after `aa`, the DFA state parks two assertions: the
leading `\b` (a new attempt could start here) and the `\b` behind the
counter's break.  On the following space both pass:

- the leading one enters the counter, whose body `a` does not take the
  space;
- the trailing one reaches the state for ` `, which does.

The flag was set by the second thread and applied to the first.  The
transition therefore pre-seeded the counter and, worse, did not reset it.
The finished instance of the previous run stayed alive with its count of
2.  The next `a` raised it to 3, the counter "broke" after a single `a`,
and ` c` completed the match.

```
after "aa"   c0: 1 inst oldest=2
after " "    c0: 2 inst oldest=2 youngest=0     <- should be empty
after "a"    break taken (oldest reached 3)     <- one `a` only
after " c"   match
```

## Investigation narrative

1. Minimised with `rethoc brute --tier 2`: both assertions are needed
   (`\ba{2,3} c` and `a{2,3}\b c` are correct), and so are two bytes after
   the counter (`\ba{2,3}\b ` is correct).  The `+` and the `$` of the
   sweep's patterns are not.
2. Bisected over the last eight commits with the engine sources swapped in
   place: present in all of them, so not caused by the prefilter
   re-engagement work.
3. The `--debug` trace showed two instances after the space, one of them
   with the old count, where the NFA has no thread in the body.
4. Followed the transition for the space through `populate()` by hand:
   non-counting, so `counter_reset` would clear the counter, but
   `resolved_body_mask` took it out of the reset again.

## Fix

The flag becomes a per-counter mask, `resolved_body_mask`.  A counter's
bit is set only if one of that counter's own first body states, reached
in the resolved closure, consumes the byte:

```rust
for &(counter, first) in cr.seed_instances.iter() {
    if cr.nfa_states.contains(&first) && consume_byte(first, byte, regex).is_some() {
        resolved_body_mask |= 1u64 << counter.idx();
    }
}
```

Pre-seeds and the exemption from `counter_reset` are both derived from
the mask.  A side effect: a counter whose body starts with several
alternative states used to get one pre-seed per state; it now gets one.
The extra instances had the same value and never changed a result.

## What was hard

- The stale instance does nothing until a later byte re-enters the body,
  so the wrong step and the wrong answer are three bytes apart.
- The flag was introduced to *remove* false positives (Bug 14) and reads
  as correct: it is set inside a loop over resolved states, next to the
  code that consumes them.  The mistake is only visible with two resolved
  assertions that lead to different places.
- Small counts are unrolled, so with the default settings the pattern
  needs a count above the unroll limit.

## Tooling ideas

- A debug check in Tier 2 after every step: a counter may only hold
  instances if the current DFA state lists one of its body states, or the
  transition just seeded it.  After the space the state lists no body
  state, and the instance with count 2 was not seeded there.
- A sweep family aimed at Tier 2: fixed-length bodies with assertions in
  front, behind and both, followed by literals and second counters.  The
  ad-hoc version used to check this fix ran 11,520 patterns over `ab c`
  with inputs up to length 6: 72 runs failed before the fix and 12 after,
  none of them new.  The remaining 12 all have the body `(?:ab|ba)` and
  no assertion; they are a different bug.
- Input length matters: a first version of that sweep with inputs up to
  length 5 found nothing on the unfixed code, because the shortest
  failing input has 6 bytes.  A sweep should be shown to fail on a known
  bug before its silence is trusted.
- `--debug` could print, per transition, the pre-seeds and the reset mask
  that were applied.
