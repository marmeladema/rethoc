# Bug 71: Tier 1 treats optimistic reachability as exact EOF acceptance

## Bug summary

| Pattern | Input | Expected | Tier 1 before the fix |
|---|---|---|---|
| `a$\B` | `a` | `false` | `true` |
| `$\b` | empty | `false` | `true` |
| `$^` | `a` | `false` | `true` |

Affected path: Tier 1's EOF match flag. The NFA and `regex` oracle agree on
these inputs. The failures were reproduced with whole-input and bytewise
feeding under all three compilation configurations. Counting-tier behavior is
outside this fix's scope.

These are valid patterns. Assertions do not consume input, so assertions after
`$` test the same boundary. After the word character `a`, EOF is a word
boundary and `\B` must fail. At the boundary of empty input, `\b` must fail.
`$^` can match empty input, but cannot match a nonempty input.

Reproduce on the old implementation:

```sh
cargo run --quiet -- brute --alphabet 'a' --max-len 1 'a$\B'
cargo run --quiet -- brute --alphabet 'a' --max-len 0 '$\b'
cargo run --quiet -- brute --alphabet 'a' --max-len 1 '$^'
```

## Root cause

Before EOF, End returns Fail rather than Defer. The shared DFA epsilon closure
therefore treated End specially: if its successor had
`state_can_reach_match=true`, it set `is_match_at_end` on the resulting state.
Tier 1 `finish()` returned true immediately for that flag.

The precomputed reachability table intentionally follows assertion edges
without evaluating them. It answers whether some zero-width graph path leads
to Match, not whether every assertion on a path passes at the actual boundary.
Turning that optimistic answer into an EOF match bypassed assertions after
End. For `a$\B`, the path from `\B` to Match exists, but its gate is false
when the previous byte is `a` and the next boundary is EOF.

## Fix

Compile exact EOF acceptance for every state of a counter-free assertion
graph. There are five physical previous contexts: input-start, WORD, LF, CR,
and other. For each context, reverse reachability starts at Match and crosses
only Split edges and assertions whose actual evaluator passes at EOF. It
stops at every consuming state. The least fixed point handles cycles without
treating an unproductive cycle as accepting.

One byte per state stores the five acceptance bits, propagated together by the
general worklist. The common case without Splits or assertion chains computes
the same result directly, without reverse adjacency or worklist scratch. A
second fast path uses a reverse index scan when every epsilon edge is verified
to point forward. Tests compare these paths with the general fixed point and
a separate forward EOF walk.

Tier 1's provisional closure obtains EOF acceptance from its raw roots using
the actual context, before discarding any context for the cached state. It
stores that exact answer in `is_match_at_end`. The ordinary closure still
does not cross End until EOF. A known-next-byte closure makes no EOF hypothesis
at its old boundary; consuming that byte creates a new boundary with its own
EOF answer. This prevents an old `$` candidate from surviving an extra byte.

`finish()` now reads the exact flag without replaying a graph or allocating
EOF scratch. Assertion-free patterns use ordinary closure Match as their EOF
answer and need no assertion metadata. Counter-free patterns no longer build
the optimistic reachability table or its per-counter break summary. Patterns
with retained counters still build both tables, preserving their meaning and
counting-tier behavior.

Regression entries cover the reported failures and the old-position case
`a$` on `ab`. Structural tests compare exact EOF metadata with forward closure
for assertion pairs and every possible previous byte, including input-start.

## How it was found

While turning the CRLF-support investigation into an implementation plan, a
review followed the EOF path independently of next-byte assertion resolution.
The static reachability shortcut contradicted the requirement that the whole
assertion chain be evaluated at one boundary. Small CLI brute-force examples
made the false positives explicit. The plan's bounded scheduler model then
included exact EOF masks and a mutation that restores optimistic reachability.

## What made it hard

- `$` does not return Defer, so merely widening deferred assertion context
  cannot fix this path.
- A graph reachability optimization has a plausible name and works when
  nothing conditional follows End; simple `$` tests therefore pass.
- An impossible assertion chain is legal syntax and still needs an exact
  false result. Rejecting it, or assuming nobody writes it, would hide the
  evaluator defect.
- Next-byte resolution and EOF resolution refer to different boundary facts.
  Carrying an EOF hypothesis across consumption would introduce another
  false-positive family even with an exact table.

## Tooling that would have helped

- Assertion-pair sweeps with End both before and after another assertion,
  including empty input and impossible combinations.
- An independent complete-context closure oracle for table construction.
- Debug output distinguishing a proven EOF answer from optimistic graph
  reachability, together with the boundary context used to compute it.
- Fault injection that lets every assertion pass during EOF precomputation,
  and tests that fail when an old EOF answer is carried across a consumed byte.
