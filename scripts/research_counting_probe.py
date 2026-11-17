#!/usr/bin/env python3
"""Run isolated counting probes, preserving timeout results beside timings."""
import subprocess

print("family,bound,bytes,tier,chunk,config,min_tier,compiled_bytes,compile_us,cold_us,runs,warm_ns_per_byte,warm_us", flush=True)
cases = []
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
        print(result.stderr, end="", file=__import__("sys").stderr)
    else:
        print(result.stdout, end="", flush=True)
