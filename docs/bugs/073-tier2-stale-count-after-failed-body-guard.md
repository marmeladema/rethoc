# Bug 73: Tier 2 retains a count after a body guard fails

The default engine accepted `a a` for `\b(?:a|[ \n](?m:^)){2,40}`.
The NFA and the regex crate reject it. Both alternatives consume one byte.
After a space, the line-start assertion fails. Replacing the space with
a newline makes that assertion pass, so `a\n` is a match.

An audit of Tier 2's assertion handling found the example by comparing
whole and bytewise matching with the NFA. The discrepancy first becomes
visible after the space. The debug trace still shows the oldest count at
one, although the body has no surviving consuming state. The final
`a` takes that count to two.

At the boundary between the first `a` and the space, the parked word
boundary also starts a fresh counter instance. The space takes the
body's character-class edge. Tier 2 used that fact to exempt the whole
counter from reset. The subsequent assertion failed and the transition
never reached the increment, but the exemption kept the old count.

The successor closure already accounts for assertions after consumption.
For a one-byte body the transition must reach the increment to preserve
the counter. For a longer body an interior consuming state also records
progress. A deferred entry now uses the resulting reset mask to decide
whether to seed an instance. This applies the same survival test to
existing counts and to an entry resolved on the current byte.

The false positive requires both a deferred entry and a consuming edge
followed by a failing guard. Testing either in isolation misses the
interaction. A sweep combining entry assertions with guards on the last
byte of each alternative would exercise it directly. The regression
checks both outcomes of the guard. Tier 3 has its own documented defect
on this pattern (Bug 69).
