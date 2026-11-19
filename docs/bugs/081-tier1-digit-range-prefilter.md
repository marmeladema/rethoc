# Bug 81: Repeated short digit-range searches in the Tier 1 control

On PR #27 the x86-64 Callgrind control named
`control_tier1_no_prefilter_text` executed 5,023,556 instructions against
4,675,544 on main, exceeding the 5% gate. Its conditional branch counts
differed by three. The pattern matches an address and port; despite the
benchmark name, its compiled prefilter searches for ASCII digits. The
benchmark returned the expected non-match.

On this stream the matcher repeatedly returns to its start state after a
short failed candidate. In the fixture's 256 KiB of deterministic text,
over 90% of start probes encounter a digit in the first 16 bytes.
The x86 range finder checked for AVX2 and dispatched to a scan kernel
even when the first vector answered the whole query. This work was also
present on main; the CI counters alone do not explain the difference
between the two builds.

Check one unaligned SSE2 vector before the long-scan dispatch. The first
set bit gives the first matching byte. On a miss, search only the
remaining slice and translate its offset back to the original input.
The scalar, vector-boundary, alignment and unsigned-range tests exercise
the first-hit contract. AArch64 uses a separate NEON implementation, so
the x86 CI run is necessary to evaluate the instruction-count change.
