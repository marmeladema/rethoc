//! Prefilter selection and byte-scanning dispatch.
//!
//! When the DFA is in the DEAD state (no active threads), a prefilter
//! skips over large runs of irrelevant input at SIMD speed by scanning
//! for bytes that can possibly begin a match.
//!
//! The prefilter is computed at compile time from the NFA start state's
//! epsilon closure, collecting the set of bytes accepted by consuming
//! states reachable without passing through assertions.

use std::fmt;

use crate::memclass::MemclassTable;
use crate::{ByteMap, State, StateIdx};

/// A prefilter for skipping non-matching bytes in the input.
///
/// Derived from the NFA start state: the set of consuming states
/// reachable via epsilon transitions determines which input bytes can
/// possibly begin a match.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Prefilter {
    /// No prefilter — all bytes are potentially interesting (e.g. dot-star,
    /// empty pattern, or too many distinct start bytes).
    #[default]
    None,
    /// Exactly one byte can start a match.
    Memchr1(u8),
    /// Exactly two distinct bytes can start a match.
    Memchr2(u8, u8),
    /// Exactly three distinct bytes can start a match.
    Memchr3(u8, u8, u8),
    /// Four to eight distinct start bytes, searched via SIMD shuffle
    /// lookup tables.
    Memclass(MemclassTable),
    /// All start bytes fall in a contiguous range `[lo, hi]` (inclusive).
    /// Used when there are more than 8 distinct start bytes but they fit
    /// within a range of at most 16 values.
    Range(u8, u8),
}

impl fmt::Display for Prefilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefilter::None => write!(f, "none"),
            Prefilter::Memchr1(b) => write!(f, "memchr1({:?})", *b as char),
            Prefilter::Memchr2(a, b) => {
                write!(f, "memchr2({:?}, {:?})", *a as char, *b as char)
            }
            Prefilter::Memchr3(a, b, c) => {
                write!(
                    f,
                    "memchr3({:?}, {:?}, {:?})",
                    *a as char, *b as char, *c as char
                )
            }
            Prefilter::Memclass(t) => {
                let bytes: Vec<String> = (0..=255u8)
                    .filter(|&b| t.contains(b))
                    .map(|b| {
                        if b.is_ascii_graphic() {
                            format!("{:?}", b as char)
                        } else {
                            format!("0x{:02X}", b)
                        }
                    })
                    .collect();
                write!(f, "memclass({})", bytes.join(", "))
            }
            Prefilter::Range(lo, hi) => write!(f, "range(0x{:02X}..=0x{:02X})", lo, hi),
        }
    }
}

/// Compute a prefilter from the NFA start state.
///
/// Walks epsilon transitions from `start` to collect all bytes accepted
/// by reachable consuming states.  Follows `Split`, `CounterInstance` and
/// `CounterIncrement` (all epsilon states; the last is only reachable
/// when a counter body can match empty).  Bails on `Assert` (needs
/// runtime context — left for a future improvement).
///
/// Returns `Prefilter::None` when no consuming states are reachable
/// (e.g. assertions block all paths) or when the byte set is too large
/// for any prefilter variant.  The caller is responsible for checking
/// whether the pattern can match empty (in which case every byte is
/// interesting and no prefilter helps).
pub(crate) fn compute_prefilter(
    start: StateIdx,
    states: &[State],
    classes: &[crate::ByteClassBits],
    byte_tables: &[ByteMap],
) -> Prefilter {
    let mut start_bytes = [false; 256];
    let mut stack = vec![start];
    let mut visited = vec![false; states.len()];
    let mut found_any = false;

    while let Some(idx) = stack.pop() {
        let i = idx.idx();
        if i >= states.len() || visited[i] {
            continue;
        }
        visited[i] = true;
        match states[idx] {
            State::Split { out, out1 } => {
                stack.push(out);
                stack.push(out1);
            }
            // CounterInstance is an epsilon state — follow through to
            // find consuming bytes in the counter body.
            State::CounterInstance { out, .. } => {
                stack.push(out);
            }
            State::Byte { byte, .. } => {
                start_bytes[byte as usize] = true;
                found_any = true;
            }
            State::ByteCI { byte, .. } => {
                start_bytes[byte as usize] = true;
                start_bytes[(byte ^ 0x20) as usize] = true;
                found_any = true;
            }
            State::Wildcard { .. } => {
                for b in 0..=255u8 {
                    start_bytes[b as usize] = true;
                }
                found_any = true;
            }
            State::ByteClassStatic { table, .. } => {
                for b in 0..=255u8 {
                    if table[b as usize] {
                        start_bytes[b as usize] = true;
                    }
                }
                found_any = true;
            }
            State::ByteClassCustom { class, .. } => {
                for b in 0..=255u8 {
                    if classes[class.idx()].contains(b) {
                        start_bytes[b as usize] = true;
                    }
                }
                found_any = true;
            }
            State::ByteTable { table, .. } => {
                for b in 0..=255u8 {
                    if byte_tables[table.idx()].0[b as usize] != StateIdx::NONE {
                        start_bytes[b as usize] = true;
                    }
                }
                found_any = true;
            }
            // Assert states need runtime context.  Bail entirely —
            // including bytes behind assertions would produce a
            // prefilter that finds candidates the DFA re-seeding
            // can't process (start_closure is empty when asserts
            // are present).
            State::Assert { .. } => {
                return Prefilter::None;
            }
            // A CounterIncrement is only reachable here, without consuming
            // a byte, when its body can match empty.  The match can then go
            // on past it: around the loop again, or out through the break
            // once enough (possibly empty) iterations are done.  Follow
            // both, so that the bytes after the repetition are candidates
            // too.  A superset of the true start bytes is always sound.
            State::CounterIncrement { out, out1, .. } => {
                stack.push(out);
                stack.push(out1);
            }
            State::Match => {}
        }
    }

    if !found_any {
        return Prefilter::None;
    }

    let bytes: Vec<u8> = (0..=255u8).filter(|&b| start_bytes[b as usize]).collect();
    match bytes.len() {
        0 => Prefilter::None,
        1 => Prefilter::Memchr1(bytes[0]),
        2 => Prefilter::Memchr2(bytes[0], bytes[1]),
        3 => Prefilter::Memchr3(bytes[0], bytes[1], bytes[2]),
        4..=8 => Prefilter::Memclass(MemclassTable::new(&bytes)),
        _ => {
            let lo = bytes[0];
            let hi = bytes[bytes.len() - 1];
            if hi - lo < 16 {
                Prefilter::Range(lo, hi)
            } else {
                Prefilter::None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Regex, RegexConfig};

    /// Helper: compile a pattern and return its prefilter.
    fn prefilter_for(pattern: &str) -> Prefilter {
        let re = Regex::new(pattern).unwrap();
        re.prefilter
    }

    /// Helper: compile with unrolling disabled (forces counters).
    fn prefilter_for_no_unroll(pattern: &str) -> Prefilter {
        let re = Regex::with_config(
            pattern,
            RegexConfig {
                max_unroll_states: 0,
                ..Default::default()
            },
        )
        .unwrap();
        re.prefilter
    }

    // ── Basic variant selection ──

    #[test]
    fn test_single_literal() {
        assert!(matches!(prefilter_for("abc"), Prefilter::Memchr1(b'a')));
    }

    #[test]
    fn test_two_start_bytes() {
        // a|b merges to [a-b] class in HIR → 2 bytes.
        // But [ac] stays as class with 2 bytes.
        match prefilter_for("[ac]x") {
            Prefilter::Memchr2(a, b) => {
                assert_eq!(a, b'a');
                assert_eq!(b, b'c');
            }
            other => panic!("expected Memchr2, got {:?}", other),
        }
    }

    #[test]
    fn test_three_start_bytes() {
        match prefilter_for("[ace]x") {
            Prefilter::Memchr3(a, b, c) => {
                assert_eq!(a, b'a');
                assert_eq!(b, b'c');
                assert_eq!(c, b'e');
            }
            other => panic!("expected Memchr3, got {:?}", other),
        }
    }

    #[test]
    fn test_case_insensitive_two_start_bytes() {
        // (?i)a has 2 start bytes: a, A
        match prefilter_for("(?i)abc") {
            Prefilter::Memchr2(a, b) => {
                assert_eq!(a, b'A');
                assert_eq!(b, b'a');
            }
            other => panic!("expected Memchr2, got {:?}", other),
        }
    }

    #[test]
    fn test_memclass_six_bytes() {
        // (?i)[abc]x → 6 start bytes
        match prefilter_for("(?i)[abc]x") {
            Prefilter::Memclass(t) => {
                for &b in b"aAbBcC" {
                    assert!(t.contains(b), "should contain {}", b as char);
                }
                assert!(!t.contains(b'd'));
            }
            other => panic!("expected Memclass, got {:?}", other),
        }
    }

    #[test]
    fn test_memclass_eight_bytes() {
        match prefilter_for("(?i)[abcd]x") {
            Prefilter::Memclass(t) => {
                for &b in b"aAbBcCdD" {
                    assert!(t.contains(b));
                }
            }
            other => panic!("expected Memclass, got {:?}", other),
        }
    }

    #[test]
    fn test_dot_star_no_prefilter() {
        // .* matches everything — no useful prefilter.
        assert!(matches!(prefilter_for(".*"), Prefilter::None));
    }

    // ── CounterInstance follow-through ──

    #[test]
    fn test_counter_does_not_defeat_prefilter() {
        // Pattern with a counter: (?:<...){100,} — the counter body
        // starts with '<', which should be found through the CI state.
        match prefilter_for("(?:<x){100,}") {
            Prefilter::Memchr1(b'<') => {}
            other => panic!("expected Memchr1('<'), got {:?}", other),
        }
    }

    #[test]
    fn test_alternation_with_counter_branch() {
        // <a...|(?:<...){100,} — both branches start with '<'.
        // The counter branch should not defeat the prefilter.
        match prefilter_for("<ax|(?:<bx){100,}") {
            Prefilter::Memchr1(b'<') => {}
            other => panic!("expected Memchr1('<'), got {:?}", other),
        }
    }

    #[test]
    fn test_counter_min_zero_still_gets_prefilter() {
        // {0,100} is lowered as (body{1,100})? — the ? creates a Split
        // where one branch has the CI, the other skips to continuation.
        // The CI branch contributes '<' to the prefilter.
        let pf = prefilter_for("(?:<x){0,100}y");
        assert!(
            !matches!(pf, Prefilter::None),
            "expected a prefilter, got None"
        );
    }

    // ── Assert bail-out ──

    #[test]
    fn test_assert_bails_prefilter() {
        // ^abc — the ^ assertion causes prefilter bail.
        assert!(matches!(prefilter_for("^abc"), Prefilter::None));
    }

    #[test]
    fn test_assert_in_alternation_bails() {
        // (?:^|/)abc — the ^ in one branch bails the entire prefilter.
        assert!(matches!(prefilter_for("(?:^|/)abc"), Prefilter::None));
    }

    #[test]
    fn test_word_boundary_bails() {
        assert!(matches!(prefilter_for(r"\bfoo"), Prefilter::None));
    }

    // ── Quantifiers ──

    #[test]
    fn test_plus_single_byte() {
        // a+ → self-loop Byte('a'), start closure has 'a'.
        match prefilter_for("a+x") {
            Prefilter::Memchr1(b'a') => {}
            other => panic!("expected Memchr1('a'), got {:?}", other),
        }
    }

    #[test]
    fn test_star_single_byte() {
        // a*x → Split(Byte('a'), next), start closure has 'a' and 'x'.
        match prefilter_for("a*x") {
            Prefilter::Memchr2(a, b) => {
                assert_eq!(a, b'a');
                assert_eq!(b, b'x');
            }
            other => panic!("expected Memchr2('a','x'), got {:?}", other),
        }
    }

    #[test]
    fn test_optional_single_byte() {
        // a?x → Split(Byte('a'), Byte('x')), start closure has 'a' and 'x'.
        match prefilter_for("a?x") {
            Prefilter::Memchr2(a, b) => {
                assert_eq!(a, b'a');
                assert_eq!(b, b'x');
            }
            other => panic!("expected Memchr2('a','x'), got {:?}", other),
        }
    }

    #[test]
    fn test_plus_case_insensitive() {
        // (?i)a+x → self-loop ByteCI('a'), start has 'A' and 'a'.
        match prefilter_for("(?i)a+x") {
            Prefilter::Memchr2(a, b) => {
                assert_eq!(a, b'A');
                assert_eq!(b, b'a');
            }
            other => panic!("expected Memchr2('A','a'), got {:?}", other),
        }
    }

    #[test]
    fn test_star_class() {
        // [ab]*x → start closure has 'a', 'b', 'x'.
        match prefilter_for("[ab]*x") {
            Prefilter::Memchr3(a, b, c) => {
                assert_eq!(a, b'a');
                assert_eq!(b, b'b');
                assert_eq!(c, b'x');
            }
            other => panic!("expected Memchr3, got {:?}", other),
        }
    }

    // ── Unrolling disabled (forces counters) ──

    #[test]
    fn test_no_unroll_single_counter() {
        // a{1,5}x with unrolling → tier 1, no counter.
        // Without unrolling → counter, CI in start path.
        // Both should get memchr1('a').
        match prefilter_for("a{1,5}x") {
            Prefilter::Memchr1(b'a') => {}
            other => panic!("expected Memchr1('a') with unroll, got {:?}", other),
        }
        match prefilter_for_no_unroll("a{1,5}x") {
            Prefilter::Memchr1(b'a') => {}
            other => panic!("expected Memchr1('a') without unroll, got {:?}", other),
        }
    }

    #[test]
    fn test_no_unroll_case_insensitive_counter() {
        // (?i)a{1,5}x → with unrolling: 2 start bytes (a, A).
        // Without unrolling: counter, CI leads to ByteCI('a').
        let pf = prefilter_for_no_unroll("(?i)a{1,5}x");
        match pf {
            Prefilter::Memchr2(a, b) => {
                assert_eq!(a, b'A');
                assert_eq!(b, b'a');
            }
            other => panic!("expected Memchr2('A','a'), got {:?}", other),
        }
    }

    #[test]
    fn test_no_unroll_alternation_with_counter() {
        // <a|(?:<b){3,10} — both start with '<'.
        // Without unrolling, the second branch uses a counter.
        match prefilter_for_no_unroll("<a|(?:<b){3,10}") {
            Prefilter::Memchr1(b'<') => {}
            other => panic!("expected Memchr1('<'), got {:?}", other),
        }
    }

    #[test]
    fn test_no_unroll_class_counter() {
        // [abc]{2,10}x — without unrolling, counter with ByteClass body.
        let pf = prefilter_for_no_unroll("[abc]{2,10}x");
        match pf {
            Prefilter::Memchr3(a, b, c) => {
                assert_eq!(a, b'a');
                assert_eq!(b, b'b');
                assert_eq!(c, b'c');
            }
            other => panic!("expected Memchr3('a','b','c'), got {:?}", other),
        }
    }

    // ── Counter bodies that can match empty (Bug 60) ──

    #[test]
    fn test_no_unroll_nullable_counter_body_includes_following_byte() {
        // Every iteration of (e?){3} can be empty, so a match can start
        // with the `d` after the repetition.
        for pattern in ["(e?){3}d", "(e*){1,3}d", "(e|){2,3}d", "(e*){2,}d"] {
            match prefilter_for_no_unroll(pattern) {
                Prefilter::Memchr2(b'd', b'e') => {}
                other => panic!("{pattern}: expected Memchr2('d','e'), got {:?}", other),
            }
        }
    }

    #[test]
    fn test_nullable_counter_body_default_settings() {
        // Too large to unroll with the default limit: the counter is kept.
        match prefilter_for("(e?){40}d") {
            Prefilter::Memchr2(b'd', b'e') => {}
            other => panic!("expected Memchr2('d','e'), got {:?}", other),
        }
    }

    #[test]
    fn test_no_unroll_nested_nullable_counter_bodies() {
        // Both loops can be skipped without consuming anything.
        match prefilter_for_no_unroll("((e?){2}f?){2}d") {
            Prefilter::Memchr3(b'd', b'e', b'f') => {}
            other => panic!("expected Memchr3('d','e','f'), got {:?}", other),
        }
    }

    #[test]
    fn test_no_unroll_nullable_inner_counter_stops_at_consuming_byte() {
        // The inner loop can be empty but the `x` after it cannot, so
        // nothing past `x` is a start byte.
        match prefilter_for_no_unroll("((e?){2}x){2}d") {
            Prefilter::Memchr2(b'e', b'x') => {}
            other => panic!("expected Memchr2('e','x'), got {:?}", other),
        }
    }

    // ── Display ──

    #[test]
    fn test_display_none() {
        assert_eq!(Prefilter::None.to_string(), "none");
    }

    #[test]
    fn test_display_memchr1() {
        assert_eq!(Prefilter::Memchr1(b'<').to_string(), "memchr1('<')");
    }

    #[test]
    fn test_display_memchr2() {
        assert_eq!(
            Prefilter::Memchr2(b'a', b'b').to_string(),
            "memchr2('a', 'b')"
        );
    }

    #[test]
    fn test_display_range() {
        assert_eq!(
            Prefilter::Range(0x41, 0x5A).to_string(),
            "range(0x41..=0x5A)"
        );
    }
}
