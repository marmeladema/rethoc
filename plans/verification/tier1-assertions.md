# Tier 1 assertion plan audit

Design: [complete Tier 1 assertion support](../tier1-assertions.md).
Rust baseline: `97adc64e7e3f6ce720541498a79e4b8d476e5cc9`.
No engine implementation was changed for this audit.

## Reproduce

From the repository root, without Python optimization flags (the checks use
`assert`):

```sh
scripts/capped --time 30 -- python3 plans/verification/check_assertion_eval.py
scripts/capped --time 60 -- python3 plans/verification/check_tier1_assertion_model.py
```

The first script requires rustc and checks the actual Rust evaluator extracted
from the current source. The second requires only Python and imports the
transcribed assertion predicate from `check_grouped_assertion_model.py` in this
directory. It disables bytecode output and does not modify the engine.

## Predicate checks

The existing Rust evaluator audit passed:

```text
kinds=10 complete_boundary_checks=660490 provisional_contexts=2570 stable_pass_extensions=1285
```

This compares all ten assertion kinds against separate complete-boundary rules
for every previous byte or input-start, and every next byte or EOF. It also
checks that provisional Pass remains Pass when a byte or EOF is revealed at
the same boundary. It does not check the Rust Tier 1 scheduler.

## Proposed scheduler and cache model

`check_tier1_assertion_model.py` compares two graph interpreters:

- An uncached oracle retains raw graph roots, then evaluates each boundary
  with its real previous byte, next byte, and start/end flags.
- A proposed lazy matcher precomputes five-context EOF masks and epsilon-tail
  dependencies, stores consuming and pending frontiers with canonical context,
  resolves pending assertions with a known next byte, and caches transitions.
  It captures exact EOF acceptance from raw roots before discarding context.

The model derives minimal sufficient previous-context dependencies from the
predicate over the five context classes. The planned Rust implementation uses
an exhaustive dependency declaration checked against `eval`; the model does
not validate that future declaration or its integration into byte classes.

Graphs include all 100 ordered pairs of assertion kinds in four families:
prefix, suffix, alternatives, and an epsilon cycle with a consuming exit.
Nine targeted graphs cover CRLF-middle exclusions and the existing chain/EOF
failures; a larger literal-prefix graph forces cache eviction. Consumers are
literal bytes or wildcard. Each graph checks all inputs of lengths 0–3 over
`a`, space, CR, LF, and `0xff`, every chunk partition, and inserted empty chunks.

One cache per graph and cap is reused across inputs and partitions. The two
cache configurations are unlimited and eight states. The small cache retains
the start and current states and clears transitions on eviction; state keys
are immutable tuples, so this **does not test production integer-ID remapping**.
Prefix checks ask what the answer would be if the stream finished there;
the model's observation is non-destructive, unlike the Rust consuming
`finish(self)` API. Empty chunks are also checked for state stability.

Recorded result:

```text
graphs=410
alphabet_bytes=5
max_input_length=3
cache_caps=[unlimited, 8]
executions=455920
prefix_checks=1342340
positive_final_outcomes=91482
cache_hits=1261358
cache_misses=10802
cache_evictions=30
```

Executions count graph/input/partition/cache combinations, not unique patterns
or inputs. Positive outcomes repeat across partitions and cache settings.
All comparisons passed. This is bounded experimental evidence for the chosen
algorithm, not a proof for arbitrary graphs.

## Fault sensitivity

All four deliberately unsafe model variants disagreed with the oracle:

| Mutation | Graph equivalent | Input | Expected | Mutated |
|---|---|---|---|---|
| Drop CR context | `\r(?Rm:^)a` | CR, `a` | true | false |
| Drop LF context behind a parked assertion | `\n\b(?m:^)a` | LF, `a` | true | false |
| Drop input-start context | `\b^a` | `a` | true | false |
| Treat every assertion as passable in EOF reachability | `a$\B` | `a` | false | true |

These are model graphs, not evidence that the compiler preserves each textual
shape. The actual CLI failures recorded in the plan establish production
examples independently.

## Remaining verification obligations

Both interpreters share a transcribed assertion predicate. The separate Rust
audit reduces that risk but does not directly compare the Python predicate
to Rust or substitute for the `regex` crate oracle. The model omits production
compilation/optimizations, ByteCI and byte classes/tables, fused consuming
exits, DEAD-state optimization, prefilter skipping, integer cache IDs, cache
reuse across different regex objects, counters, memory layout, allocation and
performance. It also does not test the proposed shared-state compatibility
wrappers for Tiers 2–4.

The implementation gates in the plan therefore still require real-engine
regressions, differential generation, all chunk partitions for small cases,
forced classes and eviction, shared-memory reuse, full tests and clippy, and
baseline/candidate performance measurements. The audit is not permission to
skip any of them.
