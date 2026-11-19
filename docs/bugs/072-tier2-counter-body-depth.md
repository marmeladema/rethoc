# Bug 72: Tier 2 accepts a variable-length counter body

On `(a|bb|c){2,5}`, Tier 2 rejected `abb` and accepted `bbb`.
The NFA and the regex oracle gave the opposite answers, both whole
and bytewise. This also happened with unrolling disabled.

## Root cause

The byte table at the counter entrance puts the increment on the worklist
at depth one for `a`. While that state is pending, the walk follows the
second `b` and reaches the same increment at depth two. Its pending flag
suppressed the depth comparison. The reported length of one let Tier 2
discard the count earned by `a` in the first input and combine overlapping
occurrences of `bb` in the second.

Record the depth on first discovery, including at the entry. Compare the
depth on every subsequent arrival, whether the state is pending or has
already been processed. Equal-depth arrivals need no more work. Any
conflict excludes the body from Tier 2.

## Investigation

The original brute-force failure used `(c|y{2,3}|f{2,4}){3,5}` on
`ccyy`. Disabling unrolling there leaves nested counters and selects
Tier 4, obscuring which counter had the wrong length. The NFA dump
showed that only the outer counter survived at defaults, with a reported
body length of one. A three-way alternation of literals isolated the
depth error without relying on inner counts. Generating alternatives
with known byte lengths and permuting their byte-table entries checks
the worklist-order dependency directly.
