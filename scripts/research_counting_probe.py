#!/usr/bin/env python3
"""Run isolated counting probes, preserving timeout results beside timings.

`--grouped` runs the grouped NFA comparison instead of the default set.
"""
import subprocess
import sys

print("family,bound,bytes,tier,chunk,config,min_tier,compiled_bytes,compile_us,cold_us,runs,warm_ns_per_byte,warm_us", flush=True)

# `--grouped`: the grouped NFA comparison (plans/grouped-nfa.md, §8): every
# family and bound under forced counters and default compilation, on every
# path that accepts it, including the grouped NFA.
cases = []
if "--grouped" in sys.argv:
    # `grouped-full` / `grouped-active` force one of its two passes.
    paths = ["0", "1", "2", "3", "4", "grouped", "grouped-full", "grouped-active"]
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
for case in cases:
    args = list(map(str, case))
    result = subprocess.run(
        ["scripts/capped", "--mem", "1000000", "--time", "5", "--",
         "target/release/examples/research_counting_probe", *args],
        text=True, capture_output=True,
    )
    if result.returncode:
        print(",".join(args) + f",exit_{result.returncode}", flush=True)
        print(result.stderr, end="", file=sys.stderr)
    else:
        print(result.stdout, end="", flush=True)
