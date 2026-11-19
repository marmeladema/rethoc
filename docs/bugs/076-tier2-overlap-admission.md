# Bug 76: Tier 2 never consults its overlap proof

With unrolling disabled, `^([ab]{5,26}|[bc]{5,26})$` passed the unit
test for the overlap proof but selected the grouped NFA. Forced Tier 2
construction failed. The builder allocated Tier 2 analysis only for
patterns already admitted, while the overlap path required that
analysis and a rejection.

## A second failure behind the builder condition

Enabling the old proof in an isolated checkout exposed a wrong answer.
With unrolling disabled, `^([ab]{4}$|[bc]{2}$x)` on `bb` should fail.
The NFA and the regex crate rejected it. Tier 2 matched with whole and
bytewise feeding. At two bytes the second counter can break, but its
end assertion still leads to a consuming `x`. The proof simulated both
end assertions as immediate EOF acceptance. Consequently it allowed
the second counter's break to use the first counter's accepting exit.
The simulator also ignored the second successor of consuming states.

## Fix

Admission uses a sufficient condition on the compiled graph. All counter
breaks must reach the very same terminal state, either Match or End
immediately followed by Match. Every nonempty combination of breaks
therefore contributes the same action. Exits add no consuming body
states, resets or seeds. Bodies must be free of assertions because
a statically reachable increment could otherwise be gated while a
different counter increments on the same byte. Ordinary Tier 2
conditions and the existing identical-byte-set exclusion still apply.
The work is a linear scan using the precomputed counter-body membership
table. The builder can decide eligibility before allocating analysis.

## Discovery and coverage

Review of the builder revealed the contradictory conditions. The earlier
proof tests computed their own analysis; the differential tests skipped
their comparison when forced matcher construction failed. This made the
builder defect invisible to both. The regression checks the normal
builder and requires forced construction for admitted patterns. The
match table checks positive and negative answers with whole and bytewise
feeding, default and small caches, and forced byte classes.
