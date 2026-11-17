#!/usr/bin/env python3
"""Check actual Rust assertion predicates against grouped-replay prerequisites.

Requires rustc. Extracts source to a temporary directory, without editing the
engine. This verifies predicates, not a complete grouped matcher.
"""

from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def rust_item(source, marker):
    start = source.index(marker)
    brace = source.index("{", start)
    depth = 1
    end = brace + 1
    while depth:
        depth += (source[end] == "{") - (source[end] == "}")
        end += 1
    return source[start:end]


def main():
    source = (ROOT / "rethoc-engine/src/lib.rs").read_text()
    declaration = rust_item(source, "assert_kinds! {")
    kinds = re.findall(r"^    ([A-Z]\w*),$", declaration, re.MULTILINE)
    assert kinds, "could not extract assertion kinds"
    evaluator = rust_item(source, "fn eval(self, at_start:")
    word_helper = rust_item(source, "fn is_word_byte(b:")
    program = (
        "#[derive(Clone, Copy, Debug)] enum AssertKind {"
        + ",".join(kinds)
        + "}\n"
        + "#[derive(Clone, Copy, Debug, PartialEq, Eq)] enum AssertEval {Pass, Fail, Defer}\n"
        + word_helper
        + "\nimpl AssertKind {"
        + evaluator
        + "}\nconst KINDS: &[AssertKind] = &["
        + ",".join("AssertKind::" + kind for kind in kinds)
        + "];\n"
        + r"""
// Independent specification of complete boundaries. The exhaustive match
// deliberately fails compilation if a new assertion kind is introduced.
fn reference(k: AssertKind, start: bool, end: bool,
             prev: Option<u8>, next: Option<u8>) -> bool {
    let word = |b: Option<u8>| b.is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_');
    match k {
        AssertKind::Start => start,
        AssertKind::End => end,
        AssertKind::StartLF => start || prev == Some(b'\n'),
        AssertKind::EndLF => end || next == Some(b'\n'),
        AssertKind::StartCRLF => start || prev == Some(b'\n')
            || (prev == Some(b'\r') && next != Some(b'\n')),
        AssertKind::EndCRLF => end || next == Some(b'\r')
            || (next == Some(b'\n') && prev != Some(b'\r')),
        AssertKind::WordAscii => word(prev) != word(next),
        AssertKind::WordAsciiNegate => word(prev) == word(next),
        AssertKind::WordStartAscii => !word(prev) && word(next),
        AssertKind::WordEndAscii => word(prev) && !word(next),
    }
}

fn main() {
    let mut complete = 0_u64;
    let mut stable = 0_u64;
    let mut provisional = 0_u64;
    for &kind in KINDS {
        // Physical boundaries: no previous byte iff at stream start.
        for prev in std::iter::once(None).chain((0..=255u8).map(Some)) {
            let start = prev.is_none();
            let unknown = kind.eval(start, false, prev, None);
            provisional += 1;
            // Reveal a byte OR finish at this SAME boundary.
            for next in (0..=255u8).map(Some).chain(std::iter::once(None)) {
                let end = next.is_none();
                let result = kind.eval(start, end, prev, next);
                let expected = if reference(kind, start, end, prev, next) {
                    AssertEval::Pass
                } else { AssertEval::Fail };
                assert_eq!(result, expected, "{kind:?} prev={prev:?} next={next:?}");
                complete += 1;
                if unknown == AssertEval::Pass {
                    assert_eq!(result, AssertEval::Pass,
                        "unstable Pass: {kind:?} prev={prev:?} next={next:?}");
                    stable += 1;
                }
            }
        }
    }
    println!("kinds={} complete_boundary_checks={complete} provisional_contexts={provisional} stable_pass_extensions={stable}", KINDS.len());
}
"""
    )
    with tempfile.TemporaryDirectory(prefix="grouped-assert-eval-") as directory:
        path = Path(directory)
        rust = path / "audit.rs"
        binary = path / "audit"
        rust.write_text(program)
        subprocess.run(
            ["rustc", "--edition=2024", "-O", str(rust), "-o", str(binary)],
            check=True,
        )
        subprocess.run(
            [str(ROOT / "scripts/capped"), "--time", "10", "--", str(binary)],
            check=True,
        )


if __name__ == "__main__":
    main()
