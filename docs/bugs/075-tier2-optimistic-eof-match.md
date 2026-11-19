# Bug 75: Tier 2 accepts a guarded path after end-of-input

Tier 2 matched `aa` against `a{2,40}$\B`. At EOF the preceding `a` is a
word character and `\B` fails. The NFA and regex oracle both reject the
input. The one-byte body meets the Tier 2 eligibility conditions; the
counter correctly allows a break after the second byte. The failure
affects default settings, whole inputs and bytewise feeding. Tier 3
still exhibits it.

The counting DFA's epsilon closure flagged a possible EOF match when the
state after End had a zero-width path to Match. That reachability table
treats assertions as passable. Tier 2's finish method then accepted
immediately. Patterns ending directly at End behaved correctly, which
obscured the difference between reaching End and passing every assertion
after it.

Comparing the Tier 2 finish path with Bug 71 exposed the shortcut.
Tier 2 now looks up the exact EOF answer for End in the current physical
boundary context. Each cached successor records its own EOF result, so a
break can contribute only when the counter allows it. This also handles
unconditional paths through splits, as in `a{2,4}$(?:x)?`. End never
needs to be parked, and a following byte replaces the previous EOF flag.

A paired test exercises both `$\B` and `$\b` after the same counter. It
also finishes below the counter minimum and feeds another byte after the
candidate EOF position. Generating assertion pairs after counter exits
would have exercised this shortcut alongside the counter-free EOF checks.
