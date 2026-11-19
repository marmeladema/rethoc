# Tier 2 overlap admission

Each counter has its own differential counts and phases. When several
increments fire on the same byte the matcher can choose two successors.
The first follows every continue edge. The second follows those same
edges and every break edge. The matcher selects the second if any live
instance can break.

Counters with pairwise disjoint byte sets use the ordinary eligibility
rule. For overlapping byte sets, the compiler checks the following
sufficient conditions on the compiled graph, after the ordinary
flatness and fixed-length requirements:

- Every counter breaks to the same state, either `Match` or an absolute
  end assertion with a direct edge to `Match`. Consequently every
  nonempty subset of breaks adds exactly the same match condition.
  No exit can seed more instances or contribute body-interior states
  that would suppress a counter reset.
- No body contains an assertion. Per-byte increment detection walks
  epsilon edges structurally after the last consumed byte. With
  several counters firing, an assertion on this path could suppress
  one increment even when another counter increments on the same byte.
- The existing restriction on counters with identical body byte sets
  still applies.

For example, `^([ab]{5,26}|[bc]{5,26})$` qualifies when compiled with
counters. On the common byte `b` both branches may count; either one
reaching its minimum enables the same end assertion. By contrast,
`^([ab]{4}$|[bc]{2}$x)` has two different exits. At the end of `bb`,
only the second counter may break and its tail still requires `x`.

The check uses the compiled graph and its counter-body membership
table. It needs no reachable-state or counter-subset enumeration.
