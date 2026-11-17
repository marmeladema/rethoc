# Grouped NFA assertion audit

Design under review: [grouped NFA plan](../grouped-nfa.md).
Engine baseline: `b0bb981e8c6cba7b23df176d79f741c3b61089a7`.
No engine implementation was changed for this audit.

Three independent reviews examined assertion semantics, graph/scope structure,
and an executable comparison of two scheduling models. No counterexample was
found for the admitted exact, acyclic, flat-counter design. This is bounded
experimental evidence and source-based reasoning, not a formal verification or
a test of an implemented Rust grouped backend.

## Reproduce

From the repository root:

```sh
python3 plans/verification/check_assertion_eval.py
scripts/capped --time 120 -- python3 plans/verification/check_grouped_assertion_model.py
```

The first script needs rustc. It extracts the current assertion enum, evaluator,
and word predicate into a temporary Rust program, checks them, and removes the
temporary directory. It never edits the engine. The second needs only Python;
`--suite short`, `--suite long`, and `--suite mutations` run its parts separately.
Do not use Python optimization flags that disable assertions.

## Actual evaluator checks

`check_assertion_eval.py` compares complete-boundary evaluation to a separate
byte-relation specification, over all ten assertion kinds, every possible previous
byte or stream start, and every possible next byte or EOI. It also requires
provisional Pass to remain Pass after any of those boundary completions.

Recorded result:

```
kinds=10 complete_boundary_checks=660490 provisional_contexts=2570 stable_pass_extensions=1285
```

All complete boundaries resolved to Pass or Fail, never Defer. Stable-pass
extensions include both next-byte revelation and EOI. The separate exhaustive
reference match causes a new assertion kind to require an explicit audit update.
The extraction helper is deliberately small and must be revisited if the Rust
source declaration structure changes; it is not a general Rust parser.

## Independent scheduling models

`check_grouped_assertion_model.py` compares:

- A scalar stack-based interpreter that preserves consuming states and parked
  assertions, then resumes assertion terminals when more boundary context arrives.
- A grouped set-row interpreter that recomputes topological closure from raw
  post-consumption roots.

They share an independently transcribed assertion predicate, but do not share
their closure scheduler or persistent frontier representation. The actual Rust
predicate is checked separately by the first script.

| Suite | Generated graphs | Inputs and chunking | Runs | Prefix checks |
|---|---:|---|---:|---:|
| Short | 300 | All inputs length 0–3 over a/space/CR/LF/0xff, every partition, injected empty chunks | 166,800 | 972,900 |
| Longer | 60 | Selected graphs, all inputs length 0–4, every partition, empty chunks | 333,360 | 2,444,580 |

The short suite includes all 100 ordered pairs of assertion kinds in graph
families with counter entry/body/exit placement, chains, alternatives, and
bypasses. The longer suite exercises more successful paths: it recorded 100,229
positive final outcomes. Both check initialization, prefix match flags, and
final results. No mismatch occurred.

The suites overlap intentionally. Their total is **500,160 executions and
3,417,480 prefix comparisons**, not that many distinct graph/input pairs.
The model also checks provisional-pass stability for all previous/next bytes
and EOI; its printed count of 1,280 counts byte extensions only, while the
additional five EOI stability checks are uncounted in that field.

## Fault sensitivity

All five targeted mutations disagreed with the scalar reference:

| Injected fault | Counterexample shape | Wrong result |
|---|---|---|
| Write updated counts at original CInc roots | Anchored exact-two wildcard count, one byte | False positive |
| Carry old raw roots after consumption | Anchored one wildcard then End, two bytes | False positive |
| Omit EOI replay | End assertion, empty input | False negative |
| Treat lookahead as prev before evaluating closure | Word boundary, wildcard, End on a | False negative |
| Retain only consuming rows, dropping blocked roots | Anchored wildcard then End on a | False negative |

A separate attempted mutation—retaining a full exact closure at its correct
state positions—did not produce a counterexample. Such a representation is not
inherently a double-increment bug: closure can be idempotent if roots, blocked
assertions, and state identities are preserved. The plan was corrected to
distinguish this alternative from the concrete unsafe transformations above.
The selected design still persists raw roots to keep its invariant simple.

## What remains unverified

The abstract graph families have one finite counter, maximum two, and wildcard
consuming predicates. They do not validate the production compiler, scope
admission, ByteTable/out_exit, multiple counter scopes, saturation, machine-word
bitset carries, allocation/reuse, or the performance of a Rust backend. They
also do not constitute an external regex-semantics oracle.

The plan's P3 gate therefore requires actual grouped/NFA/regex differential
tests, every assertion kind and placement, multi-counter handoffs, raw bytes,
all chunk partitions for small inputs, and scratch interleaving. The six fault
classes in the plan include an EOI-as-consuming-sentinel mutation, which is a
future implementation/model test; the current script injects the five listed
above. Future optimizations must retain these semantic and fault-sensitivity
checks rather than treating this audit as permanent certification.
