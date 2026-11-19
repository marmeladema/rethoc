# Bug 78: Tier 2 pays bulk prefilter costs for one-byte chunks

The Tier 2 bytewise controls in the Criterion `tier1_assertions` bench
stream 256 KiB through `foo.{0,100}bar` one byte at a time. The match
correctly failed. On AArch64, 7c40d78 took 21% longer than 8d0380a with
fresh matcher memory and 26% longer with warm memory in paired runs.
The slowdown reproduced on a second CPU. Direct chunking on the same
fixture showed much smaller changes.

An idle matcher at the start state repeatedly entered the bulk scanning
loop to search a single byte for a literal candidate. Tier 2 now tests
that byte against the prefilter directly. A rejected byte returns with
the same stream state as the original unsuccessful search. A candidate,
a live counter, or a matcher away from the start state still follows
the normal matcher path.

The regression was found by measuring elapsed time. The scalar predicate
is checked against the slice search for every byte and every prefilter
variant. Both cold and warm bytewise fixtures matter because warming a
cache does not remove per-chunk dispatch.

With both Tier 2 scan fixes applied, the cold and warm cases took 0.68 ms
and 0.69 ms on a second CPU, versus 0.85 ms for each on main. These are
geometric means of two Criterion runs in opposite revision order.
