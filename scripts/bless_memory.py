#!/usr/bin/env python3
"""Bless memory sizes in match_tests! entries.

Scans ``rethoc-engine/src/lib.rs`` for ``match_tests!`` entries, compiles each pattern
with ``rethoc info --format json``, and updates the expected memory size
in-place if it has changed.

Only blesses ``memory:`` -- does NOT touch ``min_tier:`` (to avoid
masking tier regressions).

Usage::

    python3 scripts/bless_memory.py          # update rethoc-engine/src/lib.rs in-place
    python3 scripts/bless_memory.py --check  # exit 1 if any are stale
    python3 scripts/bless_memory.py --dry-run # show what would change
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Optional

REPO_ROOT = Path(__file__).resolve().parent.parent
LIB_RS = REPO_ROOT / "rethoc-engine" / "src" / "lib.rs"
RETHOC = REPO_ROOT / "target" / "release" / "rethoc"

# ---------------------------------------------------------------------------
# Pattern extraction
# ---------------------------------------------------------------------------

# Matches a test entry header inside match_tests!{ ... }.
# Groups: 1=test_name, pat=pattern literal, mem=memory value,
#         unroll=optional unroll_limit, states=optional max_states.
ENTRY_RE = re.compile(
    r"""
    (\w+)                                      # test name
    \s*\{
    \s*pattern:\s*(?P<pat>.+?),                # pattern literal (any form)
    \s*memory:\s*(?P<mem>\d+),                 # memory value
    \s*min_tier:\s*\d+,                        # min_tier (not touched)
    (?:\s*unroll_limit:\s*(?P<unroll>\d+),)?    # optional unroll_limit
    (?:\s*max_states:\s*(?P<states>[\d_]+),)?    # optional max_states
    """,
    re.VERBOSE,
)

# Matches the ``memory: <digits>,`` fragment inside a matched entry span.
MEMORY_FIELD_RE = re.compile(r"memory:\s*\d+,")


def parse_rust_string(literal: str) -> str:
    """Convert a Rust string literal to a Python string.

    Handles ``"..."``, ``r"..."``, and ``r#"..."#`` (any number of ``#``).
    """
    literal = literal.strip()

    # r#"..."#  (with any number of #s)
    m = re.match(r'^r(#+)"(.*?)"\1$', literal, re.DOTALL)
    if m:
        return m.group(2)

    # r"..."
    m = re.match(r'^r"(.*?)"$', literal, re.DOTALL)
    if m:
        return m.group(1)

    # Regular "..." -- handle standard Rust escape sequences
    m = re.match(r'^"(.*)"$', literal, re.DOTALL)
    if m:
        s = m.group(1)
        result: list[str] = []
        i = 0
        while i < len(s):
            if s[i] == "\\" and i + 1 < len(s):
                c = s[i + 1]
                if c == "n":
                    result.append("\n")
                elif c == "r":
                    result.append("\r")
                elif c == "t":
                    result.append("\t")
                elif c == "\\":
                    result.append("\\")
                elif c == '"':
                    result.append('"')
                elif c == "0":
                    result.append("\0")
                elif c == "x":
                    hex_str = s[i + 2 : i + 4]
                    result.append(chr(int(hex_str, 16)))
                    i += 4
                    continue
                else:
                    # Unknown escape -- keep as-is
                    result.append(s[i])
                    result.append(c)
                i += 2
            else:
                result.append(s[i])
                i += 1
        return "".join(result)

    raise ValueError(f"Cannot parse Rust string literal: {literal!r}")


def get_memory_size(
    pattern: str, unroll_limit: Optional[int], max_states: Optional[int]
) -> int:
    """Run ``rethoc info --format json`` and return ``memory.total``.

    Returns -1 on failure.
    """
    # Compile like the test does: the default limits unless the entry
    # overrides them.
    cmd = [str(RETHOC), "info", "--format", "json"]
    if unroll_limit is not None:
        cmd.extend(["--unroll-limit", str(unroll_limit)])
    if max_states is not None:
        cmd.extend(["--max-states", str(max_states)])
    cmd.extend(["--", pattern])

    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
    if result.returncode != 0:
        print(
            f"  ERROR running rethoc for pattern {pattern!r}:",
            file=sys.stderr,
        )
        print(f"  {result.stderr.strip()}", file=sys.stderr)
        return -1

    data = json.loads(result.stdout)
    return data["memory"]["total"]


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description="Bless memory sizes in match_tests!")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Check mode: exit 1 if any memory values are stale",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would change without modifying the file",
    )
    args = parser.parse_args()

    # Always (re)build: a stale binary would report stale sizes.  This is a
    # no-op when the release build is up to date.
    subprocess.run(
        ["cargo", "build", "--release", "--quiet", "-p", "rethoc-cli"],
        cwd=REPO_ROOT,
        check=True,
    )

    content = LIB_RS.read_text()

    # Collect all entries and compute actual memory sizes.
    entries = list(ENTRY_RE.finditer(content))
    # Each replacement is (abs_start, abs_end, new_fragment, test_name, old, new).
    replacements: list[tuple[int, int, str, str, int, int]] = []
    errors = 0

    for m in entries:
        test_name = m.group(1)
        pat_literal = m.group("pat")
        old_memory = int(m.group("mem"))
        unroll_str = m.group("unroll")
        unroll_limit = int(unroll_str) if unroll_str else None
        states_str = m.group("states")
        max_states = int(states_str.replace("_", "")) if states_str else None

        try:
            pattern = parse_rust_string(pat_literal)
        except ValueError as e:
            print(f"  SKIP {test_name}: {e}", file=sys.stderr)
            continue

        actual = get_memory_size(pattern, unroll_limit, max_states)
        if actual < 0:
            errors += 1
            continue

        if actual != old_memory:
            # Locate the ``memory: <N>,`` fragment within this match span.
            mem_m = MEMORY_FIELD_RE.search(content, m.start(), m.end())
            if mem_m is None:
                print(
                    f"  WARNING: could not find memory field for {test_name}",
                    file=sys.stderr,
                )
                continue
            replacements.append(
                (
                    mem_m.start(),
                    mem_m.end(),
                    f"memory: {actual},",
                    test_name,
                    old_memory,
                    actual,
                )
            )

    # Report
    if not replacements and errors == 0:
        print(f"All {len(entries)} memory values are up to date.")
        return 0

    if errors > 0:
        print(
            f"\n{errors} pattern(s) failed to compile -- see errors above.",
            file=sys.stderr,
        )

    if replacements:
        print(f"\n{len(replacements)} memory value(s) to update:\n")
        for _, _, _, name, old, new in replacements:
            print(f"  {name}: {old} -> {new}")

    if args.check:
        if replacements:
            print(
                f"\nFAILED: {len(replacements)} stale memory value(s). "
                "Run `python3 scripts/bless_memory.py` to fix.",
                file=sys.stderr,
            )
            return 1
        return 1 if errors else 0

    if args.dry_run:
        return 0

    # Apply replacements in reverse offset order so positions stay valid.
    if replacements:
        replacements.sort(key=lambda r: r[0], reverse=True)
        for start, end, new_text, _name, _old, _new in replacements:
            content = content[:start] + new_text + content[end:]

        LIB_RS.write_text(content)
        print(f"\nUpdated {len(replacements)} memory value(s) in {LIB_RS}.")

    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
