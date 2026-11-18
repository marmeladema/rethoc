# Finite sparse-counting experiment: verification record

Design: [sparse-counting experiment](../sparse-counting-sets.md).
Reviewed revision: `730eebd5abd4d176bf22a0a0eee491522e154264`.
The original audit used `c84282e`; the re-review confirmed the only upstream
change is CLI `grep` selection, with no engine/evaluator changes. Both checks
below were rerun after rebasing.
No matching or compilation implementation changed for this audit.

## Sources and independent review

The full [paper text](../../research_notes/regex-matching-research-2020-2026/text/2026-ncma-berglund-sparse-counting-sets.md)
was read, especially Definitions 8/11/16, Properties 9/10, Lemmas 17–20 and
Corollary 21. One review examined the paper and proof obligations; another
examined actual grouped transfers, structural admission, assertions and APIs;
a third implemented the independent finite model below. Both source reviews
identified the positive-count mapping `u=c+1` and the need to check sparse
exit before increment. They also identified separate gates for unbounded
saturation and the existing bit-zero count-free acceleration.

The plan's interval-profile argument is a proposed direct-Thompson adaptation,
not a theorem about assertions taken from the paper. Reviewing it and checking
finite instances does not formally verify the future Rust engine.

## Reproduce

From the repository root, without Python optimization flags:

```sh
scripts/capped --mem 1000000 --time 120 -- \
  python3 plans/verification/check_sparse_counting_model.py
scripts/capped --mem 1000000 --time 120 -- \
  python3 plans/verification/check_assertion_eval.py
```

The sparse checker needs only Python's standard library. Its bounds are fixed
in the source, with no random inputs. The evaluator checker extracts the current
Rust evaluator and needs rustc. During this run, its wrapper warned that virtual
memory limits were unsupported; do not interpret it as a verified memory-cap
test. The enumeration itself is finite and the invocation has a time limit.

## Finite algebra checks

The checker compares exact integer sets with a direct-positive-value sparse
reducer. Its implementation uses masks only as a small-domain reference model;
it is not the proposed production storage layout. It verifies:

- Subset relation, endpoint retention, spacing and reserved-capacity bound.
- Equality of all future exit and nonempty observations through exhaustion.
- Bounded increment, seed injection, check-triggered reset and whole-row gates.
- Exact completed-count to positive active-count transfer mapping.
- All pairs of independently reduced sets merged and reduced again.
- Two closures from identical roots under independently chosen Boolean gate
  outcomes in a fixed eight-node operation DAG. It includes copy, union,
  continuation, reset and guarded paths, checking each intermediate row.

Recorded successful run on 2026-11-18:

| Check | Enumerated cases |
|---|---:|
| Positive subsets, h=1..8 and l=0..h | 4,096 |
| Merge pairs | 757,304 |
| Increment cases | 4,096 |
| Seed/reset cases | 4,096 |
| Thompson mapping cases | 4,096 |
| Whole-row gates | 8,192 |
| Immutable-root replay pairs, h=1..4 | 25,472 |
| Intermediate DAG row comparisons | 407,552 |

Counts overlap: they are operation checks, not distinct regex/input tests.
No discrepancy was found. The DAG is algebraic, not extracted from Rust, and
its gates do not encode actual byte-boundary assertion semantics.

## Deliberately incorrect variants detected

| Mutation | Witness |
|---|---|
| Prune at k+1 instead of k | `[3,3]`, `{1,2,3}` loses the future observation supplied by 2 |
| Check exit after continuation | `[3,3]`, `{3}` can exit but its continuation is empty |
| Apply an arbitrary upper mask | `[5,10]`, `{4..10}` versus `{4,10}`, then retain `<=7` |
| Commit a speculative continuation as raw | `[2,2]`, `{1}` becomes `{2}`, then incorrectly expires on replay |

These witnesses show fault sensitivity for the listed algebra mistakes. They
do not establish that every possible scheduling or compiler fault is caught.

## Actual assertion evaluator

The separate existing checker was rerun on the reviewed revision:

```
kinds=10 complete_boundary_checks=660490 provisional_contexts=2570 stable_pass_extensions=1285
```

Complete contexts resolve; provisional Pass stays Pass under byte revelation
and EOI. See [the earlier assertion audit](README.md) for its independent
specification. This verifies evaluator properties, not a sparse matcher.

## Remaining obligations

When this audit was written no Rust sparse runtime existed. It now does
(`rethoc-engine/src/grouped_nfa/sparse.rs`), and its tests carry the
obligations this record left open; they are evidence by testing, not a proof.

- The primitive checks above are translated to Rust on the production list
  code (`sparse_tests.rs`): every subset and every pair for `h <= 8`, against
  `exact::increment`, with the capacity checked on a scratch of exactly the
  promised size. One correction to this record: reduction keeps the least
  count but, after an increment, not always the greatest (`{1,2,3}` in `{0,3}`
  reduces to `{1,3}`, then `{2}` stands for `{2,3}`); the observations are
  equal, which is what the argument needs.
- Graph-to-sparse lowering, layouts and limits: admission tests at each
  boundary, and the raw rows compared with the exact grouped rows after every
  byte (subset, least count, exit profile), also with assertions.
- Lifecycle, allocations and replay: stream tests, the `match_tests!` builds,
  every partition of short inputs over word, non-word, CR, LF, NUL and 0xFF
  bytes, and the differential tools (`--sparse-counting`). The probes report
  no allocation during a scan.
- The mutations listed above, and those of the plan's §9 that apply to the
  runtime, were applied to the Rust source by hand and each failed the tests;
  they are not kept as automated mutation tests.

Still open: unbounded saturation and a count-free step are not implemented
(the idle skip is); nothing here is a performance claim.
