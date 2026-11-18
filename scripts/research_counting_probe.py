#!/usr/bin/env python3
"""Run isolated counting probes, preserving timeout results beside timings.

`--grouped` runs the grouped NFA comparison instead of the default set.
`--sparse` runs the families of plans/sparse-counting-sets.md, section 10;
with `--stats` it prints what one scan does (admission, sizes, allocations
and operation counters) instead of timings, which needs a probe built with
`--features op-stats`.
"""
import subprocess
import sys

stats = "--stats" in sys.argv
if not stats:
    print("family,bound,bytes,tier,chunk,config,min_tier,compiled_bytes,compile_us,cold_us,runs,warm_ns_per_byte,warm_us,engine", flush=True)

# `--grouped`: the grouped NFA comparison (plans/grouped-nfa.md, §8): every
# family and bound under forced counters and default compilation, on every
# path that accepts it, including the grouped NFA.
cases = []
if "--grouped" in sys.argv:
    # `grouped-full` / `grouped-active` force one of its two passes.
    # `auto`: automatic selection.
    paths = ["auto", "0", "1", "2", "3", "4", "grouped", "grouped-full", "grouped-active"]
    for family in ["prefix", "variable", "exact", "flat", "phase", "mixed", "sparse", "wide"]:
        for bound in [8, 64, 256, 1000]:
            for config in ["counters", "default"]:
                for path in paths:
                    cases.append((family, bound, 4096, path, 4096, config))
    for path in paths:
        cases.append(("control", 0, 4096, path, 4096, "default"))
        cases.append(("prefix", 256, 4096, path, 1, "counters"))
    for bound in [4, 8, 16]:
        for path in ["4", "grouped"]:
            cases.append(("nested", bound, 1024, path, 1024, "counters"))
elif "--sparse" in sys.argv:
    # Forced counters only: under default compilation small bounds unroll
    # and the comparison is no longer one of representations.  With
    # `--stats` only the grouped NFA has counters to report.
    paths = ["grouped"] if stats else ["auto", "0", "2", "4", "grouped", "grouped-full", "grouped-active"]
    for family in ["range", "rangemid", "rangenarrow", "rangeexact", "sustained", "sustainedx",
                   "mixed", "seq", "phase", "unbounded"]:
        for bound in [8, 64, 256, 1000]:
            for path in paths:
                cases.append((family, bound, 65536, path, 4096, "counters"))
    for family in ["holey", "loop", "control", "nullable"]:
        for path in paths:
            cases.append((family, 8, 65536, path, 4096, "counters"))
    for path in paths:
        cases.append(("nested", 8, 65536, path, 4096, "counters"))
else:
    for family in ["flat", "phase", "variable", "prefix"]:
        for bound in [8, 64, 256]:
            for tier in [2, 3, 4]:
                cases.append((family, bound, 4096, tier, 4096, "counters"))
    for family in ["flat", "variable"]:
        for bound in [8, 64, 256]:
            for tier in [1, 2, 3]:
                cases.append((family, bound, 4096, tier, 4096, "default"))
    for family in ["flat", "variable", "prefix"]:
        for size in [1024, 16384]:
            for tier in [2, 3]:
                cases.append((family, 64, size, tier, size, "counters"))
        for tier in [2, 3]:
            cases.append((family, 64, 4096, tier, 1, "counters"))
    for bound in [4, 8, 16]:
        for size in [256, 1024]:
            cases.append(("nested", bound, size, 4, size, "counters"))
header_printed = not stats
for case in cases:
    args = list(map(str, case)) + (["stats"] if stats else [])
    result = subprocess.run(
        ["scripts/capped", "--mem", "1000000", "--time", "30" if "--sparse" in sys.argv else "5", "--",
         "target/release/examples/research_counting_probe", *args],
        text=True, capture_output=True,
    )
    if result.returncode:
        print(",".join(args) + f",exit_{result.returncode}", flush=True)
        print(result.stderr, end="", file=sys.stderr)
        continue
    lines = result.stdout.splitlines(keepends=True)
    if stats:
        # Every stats run prints the header: keep the first one.
        if not header_printed:
            print(lines[0], end="")
            header_printed = True
        lines = lines[1:]
    print("".join(lines), end="", flush=True)
