# Bug 64: A full DFA cache turns live threads into "no match"

## Bug summary

- **Pattern**: `a[ab]{12}c` (Tier 1), `a[ab]{10}c[0-9]{40,60}` (Tier 2),
  `a[ab]{10}c(?:xy|z){40,60}q` (Tier 3).  Default settings.
- **Input**: every 12-letter word over `a`/`b` in order (49 KB), followed
  by a text the pattern matches, e.g. `abbbbbbbbbbbbc`
- **Expected**: `true`; **actual**: `false` (false negative)
- **Second symptom**: after that input, the same `MatcherMemory` answers
  `false` for `abbbbbbbbbbbbc` alone — a 14-byte input that matches when
  run first.  One stream corrupts the answers for later streams.
- **Affected tiers**: 1, 2 and 3 (they share `DfaCache`).
- **Found by**: an audit of Tier 2 by a sub-agent, reading
  `unwrap_or(DfaStateId::DEAD)` and building an input that visits more
  than 2,048 DFA states.  No test, fuzz seed or sweep reached the cap.

## Root cause

A lazy-DFA matcher records where it is as one `DfaStateId`.  The states
live in a table capped at 2,048 entries (`DFA_MAX_STATES`).  When a
transition led to a state that was not in the table and the table was
full, `intern_state()` returned `None`, and every caller did

```rust
self.intern_state(...).unwrap_or(DfaStateId::DEAD)
```

`DEAD` is the id of the empty thread set.  So "this state cannot be
stored" was reported as "no thread is alive": every partial match in
progress was dropped, and the byte was not even offered to a fresh
attempt.  The result was then written into the transition table, which is
kept for the next matcher on the same regex, so the loss repeated for
every later stream that took that transition.

The cap arrived in March with the commit that replaced an LRU transition
cache by a flat table (`ac8c94d`).  Its message says "new states are not
interned and transitions return DEAD"; the doc comment it added on the
cache says the transition "falls back to recomputing without caching".
The comment describes the right behaviour and the code never did it.
Before that commit transitions were evicted but states were never
removed, so the table had no bound at all.

## Investigation narrative

1. The sub-agent's report gave the pattern, the 49 KB filler and the
   `unwrap_or` lines.  Reproduced on Tier 1 with the default settings.
2. Running filler and tail as two inputs of one `rethoc match` showed the
   second symptom: the tail alone is answered wrongly once the cache is
   full.
3. Shrinking `DFA_MAX_STATES` to 6 in the source and running the existing
   suite: 166 of 776 tests fail.  The bug was always within reach of the
   tests; the constant kept them away from it.

## Fix

The table is treated as what it is, a cache of a function that can
always be recomputed.  When it is full it is emptied and refilled on
demand.  Answers no longer depend on what is cached.

- `DfaCache::evict()` drops every state except the start state and the
  states the matcher stands on, and rewrites those ids in place.  The
  kept states are moved, not copied, and the table keeps its capacity.
  The start state keeps its id (it is always state 0): matchers copy
  `start_id` once per chunk and compare against it.
- Protocol, the same in all three tiers: on a missing transition, before
  computing it, the matcher checks that the states the computation can
  add still fit (`has_room`); if not, it evicts and looks its slot up
  again.  Interning then cannot fail, and the `unwrap_or(DEAD)` calls
  are gone: `DEAD` again means only "no thread".
- Tier 3 computes up to two transitions per byte (the main one and the
  clean no-break chain's).  A missing main transition makes room for
  both when the clean chain will take one, so a byte empties the cache
  at most once.  Only when the main transition was cached and the clean
  one is not can the cache be emptied between the two; the main
  transition is then computed again.
- Each tier clears its own tables indexed by state id: transitions;
  Tier 2's settled states and dead-state transitions; Tier 3's idle
  classification.  Tier 3's matcher holds three ids and one slot
  (`current`, `no_break_current`, `clean_nb`, `clean_nb_trans_slot`),
  all refreshed in `evict_cache()`.
- Tier 2 keeps a stream's counters inside its cache object.  They are
  the stream's state, not cached data, and eviction leaves them alone.
- The cap is now a field (`max_states`, minimum `DFA_MIN_STATES` = 8),
  settable through `MatcherMemory::set_dfa_state_cap()`.  The test
  helpers for Tiers 1–3 run every `match_tests!` entry at the default
  cap and at the minimum, where the cache is emptied every few bytes.

Cost.  Ordinary input uses few states and never evicts: 49 KB of
`abab…` leave 14 states in the cache for `a[ab]{12}c`.  The 49 KB filler
empties the cache 19 times, once every 2,600 bytes or so.  What it pays
for is not the emptying but the misses: about five bytes in six lead to
a state that has to be built again.  1 MB of it takes 0.32 s on Tier 1
against 0.15 s on the NFA.  The work per byte is bounded by the pattern,
so matching stays linear.

Callgrind gate: 19 cases, none regressed, −2.7 % to +2.0 % instructions.
Getting there took four layouts of the same logic.  The miss path of
Tiers 1 and 2 is a cold out-of-line function (left inline, two Tier 2
cases were 8 % slower).  Tier 3's scanning loop reacts the other way:
moving its transition computation out of line cost 7 % on two cases, and
room checks written inline cost up to 4.7 %.  It keeps the computation
in the loop and calls cold helpers for the room checks.

## What was hard

- Nothing fails until 2,048 distinct states exist, and no test input was
  long or varied enough.  The suite ran entirely on one side of a
  threshold.
- The wrong value was a legitimate one.  `DEAD` is what a transition
  returns thousands of times in normal operation, so nothing downstream
  could tell the difference.
- A comment said the right thing.  Anyone checking the design against
  the documentation found it sound.
- Tier 3 holds several ids at once and computes two transitions per
  byte; an eviction during the second invalidates the first.

## Tooling ideas

- Thresholds as test parameters.  Byte-class compression (128 NFA
  states) and unrolling (32 states) switch behaviour the same way; the
  suite and the fuzzers should run at their extremes too.  Done here for
  the state cap; `RETHOC_DFA_STATES=8` does it for the CLI tools.
- A mutation pass for code like this: the first version of the tests let
  four deliberate breakages through (stale Tier 2 dead-state
  transitions, stale Tier 3 idle table, two Tier 3 matcher fields).
  They are now caught by tests that check the tables directly.
- If hostile input turns out to matter for throughput: count evictions
  per stream and hand the stream to the NFA when they come too close
  together.  Not done; the cost is bounded without it.
- Tier 4 has its own state table with no cap at all.  It needs the same
  treatment; it is a memory problem there, not a wrong answer.
