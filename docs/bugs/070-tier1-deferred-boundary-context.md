# Bug 70: Tier 1 loses context behind a deferred assertion

## Bug summary

| Pattern | Input | Expected | Tier 1 before the fix |
|---|---|---|---|
| `\b(?m:^)a` | `a` | `true` | `false` |
| `\B(?m:^)` | empty | `true` | `false` |

The first example needs the boundary context when the next byte arrives; the
second needs it at EOF. The NFA and `regex` oracle agree on both. The failures
were reproduced with whole-input and bytewise feeding under the default,
no-unroll, and no-unroll/no-merge compilation configurations.

Affected path: Tier 1's deferred assertion resolution. The fix is specific to
Tier 1; it does not establish correctness of assertion chains in counting tiers.

Reproduce on the old implementation:

```sh
cargo run --quiet -- brute --alphabet 'a\n\r' --max-len 1 '\b(?m:^)a'
cargo run --quiet -- brute --alphabet 'a' --max-len 0 '\B(?m:^)'
```

## Root cause

Tier 1 computes a provisional epsilon closure after consuming a byte. An
assertion needing the next byte is parked in the DFA state. That state kept
only whether its previous byte was a word character, then reconstructed either
`a` or space when the parked assertion was resolved. Both the next-byte
mini-closure and EOF assertion walk supplied `at_start=false`.

Wordness is sufficient for the word assertion itself, but not necessarily for
its zero-width continuation. In `\b(?m:^)a`, `\b` waits at the initial
boundary. On arrival of `a`, it passes, then the mini-closure reaches `(?m:^)`
at that same initial boundary. The supplied context says neither input-start
nor previous LF, so the line anchor fails incorrectly. The empty-input example
takes the corresponding EOF path: `\B` passes, but the following line anchor
loses input-start.

The same representation cannot retain an LF needed by a line anchor behind a
parked word assertion. It also cannot distinguish CR from other nonword bytes,
which is necessary for the planned CRLF anchors. Examining only the first
parked assertion's reads therefore cannot justify the cached context.

## Fix

Tier 1 keeps a compact boundary context containing only the facts its pending
continuations need: previous WORD/LF/CR properties and input-start. An exhaustive
assertion dependency declaration supplies side-specific reads; byte classes
continue to derive their union from that declaration.

Compile-time analysis propagates dependencies backward across the complete
zero-width continuation, including Split alternatives and assertion chains.
It stops at consuming states. For resolution with a known next byte, it also
stops at End, which cannot pass there. A finite-bit worklist handles cycles
without a separate graph traversal from every assertion.

Before interning a state, Tier 1 masks its actual boundary context by the union
of its pending assertions' requirements. Ordinary word-only continuations keep
only wordness; an EndLF-only continuation needs no previous-byte context. The
next-byte closure uses this representative context through the entire chain,
consumes the new byte once, then creates the successor boundary. Start is never
inferred from equality with the initial DFA state, which can recur midstream.

The exact EOF metadata introduced with [Bug 71](071-tier1-eof-assertion-chain.md)
handles the EOF half while the actual boundary context is still available.
Other tiers retain their existing word-only wrappers and assertion semantics.

The regression entries cover the initial-boundary examples and a line-boundary
chain, `\n\b(?m:^)a`. Analysis tests compare masked and full contexts for
ordered assertion pairs. The match-test macro repeats the concrete cases with
the minimum DFA cache cap and with byte classes forced on.

## How it was found

The CRLF-support investigation traced why the compiler explicitly excluded
CRLF anchors from Tier 1. Reading the previous-byte stand-in revealed that the
limitation also applied to assertions already admitted behind a deferred
assertion. Short CLI brute-force runs confirmed the initial-boundary failures.
The Tier 1 implementation plan recorded them as prerequisite fixes rather than
adding new eligibility exclusions.

## What made it hard

- Tests of each assertion in isolation do not exercise the context required by
  an assertion hidden behind another one.
- The first assertion can evaluate correctly while a later assertion sees the
  wrong boundary, making the error look like an epsilon-closure omission.
- Whole-input feeding still uses deferred resolution internally, so this was
  not only a chunk-boundary defect.
- A default-context representative is safe only after all future reads at the
  same boundary have been accounted for.

## Tooling that would have helped

- Ordered assertion-pair generation, including line anchors behind word
  assertions, at both input-start and EOF.
- Exhaustive comparison of each declared context abstraction with actual
  assertion evaluation, including the full continuation rather than only its
  parked root.
- Debug output showing the retained context alongside pending assertions.
- Mutation checks that deliberately remove start, LF, or CR information and
  require the relevant regression to fail.
