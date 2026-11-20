//! Prefilter selection and byte-scanning dispatch.
//!
//! When the DFA is in the DEAD state (no active threads), a prefilter
//! skips over large runs of irrelevant input at SIMD speed by scanning
//! for candidate match starts.
//!
//! The prefilter is computed at compile time from the NFA start state's
//! epsilon closure, collecting the set of bytes accepted by consuming
//! states reachable without passing through assertions. Narrow ranges may
//! also use the next consuming frontier to rule out a start using lookahead.

use std::fmt;

use crate::memclass::MemclassTable;
use crate::{ByteClassBits, ByteMap, State, StateIdx};

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
    /// Two consecutive inclusive ranges. A final byte in the first range
    /// is a candidate because the next byte may arrive in another chunk.
    RangePair(u8, u8, u8, u8),
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
            Prefilter::RangePair(lo, hi, next_lo, next_hi) => write!(
                f,
                "range_pair(0x{lo:02X}..=0x{hi:02X}, 0x{next_lo:02X}..=0x{next_hi:02X})"
            ),
        }
    }
}

/// Dispatch on a [`Prefilter`]: evaluate `$none` for [`Prefilter::None`],
/// otherwise `$body` with `$find` bound to a closure
/// `|hay: &[u8]| -> Option<usize>` that returns the offset of the first
/// candidate byte.  The one place that maps a variant to its search; each
/// variant gets its own closure type, so a generic `$body` is compiled
/// once per variant with the search inlined, and a hot loop pays no
/// per-call dispatch.
macro_rules! with_prefilter {
    ($prefilter:expr, $none:expr, |$find:ident| $body:expr) => {
        match $prefilter {
            $crate::Prefilter::None => $none,
            $crate::Prefilter::Memchr1(a) => {
                let $find = |hay: &[u8]| memchr::memchr(a, hay);
                $body
            }
            $crate::Prefilter::Memchr2(a, b) => {
                let $find = |hay: &[u8]| memchr::memchr2(a, b, hay);
                $body
            }
            $crate::Prefilter::Memchr3(a, b, c) => {
                let $find = |hay: &[u8]| memchr::memchr3(a, b, c, hay);
                $body
            }
            $crate::Prefilter::Memclass(t) => {
                let $find = |hay: &[u8]| $crate::memclass::memclass(&t, hay);
                $body
            }
            $crate::Prefilter::RangePair(lo, hi, next_lo, next_hi) => {
                let $find =
                    |hay: &[u8]| $crate::memrange::memrange_pair(lo, hi, next_lo, next_hi, hay);
                $body
            }
            $crate::Prefilter::Range(lo, hi) => {
                let $find = |hay: &[u8]| $crate::memrange::memrange(lo, hi, hay);
                $body
            }
        }
    };
}
pub(crate) use with_prefilter;

impl Prefilter {
    /// Whether this byte can start a match.
    #[inline]
    pub(crate) fn accepts(&self, byte: u8) -> bool {
        match self {
            Self::None => true,
            Self::Memchr1(a) => byte == *a,
            Self::Memchr2(a, b) => byte == *a || byte == *b,
            Self::Memchr3(a, b, c) => byte == *a || byte == *b || byte == *c,
            Self::Memclass(table) => table.contains(byte),
            Self::Range(lo, hi) | Self::RangePair(lo, hi, ..) => (*lo..=*hi).contains(&byte),
        }
    }

    /// Identify an immediate candidate before starting a bulk search.
    /// Single literals use a word-wide comparison. Other filters probe only
    /// position zero using first-byte membership. A miss leaves the full
    /// search to the caller.
    #[inline(always)]
    pub(crate) fn find_in_prefix(&self, hay: &[u8]) -> Option<usize> {
        if let Self::Memchr1(byte) = *self
            && let Some(word) = hay.first_chunk::<8>()
        {
            // The first zero byte is the first literal in input order.
            // A borrow can flag a later byte as well; only the first bit is used.
            let diff = u64::from_le_bytes(*word) ^ u64::from_le_bytes([byte; 8]);
            let zeros = diff.wrapping_sub(0x0101_0101_0101_0101) & !diff & 0x8080_8080_8080_8080;
            return (zeros != 0).then(|| zeros.trailing_zeros() as usize / 8);
        }
        hay.first().filter(|&&byte| self.accepts(byte)).map(|_| 0)
    }

    /// Offset of the first candidate start in `hay`, including an unconfirmed
    /// final byte for a pair filter, or `None`
    /// if there is none.  Every byte is a candidate for [`Prefilter::None`].
    pub(crate) fn find(&self, hay: &[u8]) -> Option<usize> {
        match self {
            // The class table by reference.  Through the macro it is bound
            // by value, and whether that 32-byte copy is elided is up to
            // the optimiser; for a search called once per short stretch
            // the copy is a measurable part of the cost.  (The macro's
            // other users copy it once per chunk.)
            Prefilter::Memclass(t) => crate::memclass::memclass(t, hay),
            _ => with_prefilter!(*self, (!hay.is_empty()).then_some(0), |find| find(hay)),
        }
    }
}

/// States reached at one byte offset, including zero-width transitions.
/// Retain the visited states so their consuming successors can seed the next offset.
struct Frontier {
    queue: Vec<StateIdx>,
    seen: Vec<bool>,
}

impl Frontier {
    fn new(num_states: usize) -> Self {
        Self {
            queue: Vec::new(),
            seen: vec![false; num_states],
        }
    }

    fn push(&mut self, idx: StateIdx) {
        if idx != StateIdx::NONE && !self.seen[idx.idx()] {
            self.seen[idx.idx()] = true;
            self.queue.push(idx);
        }
    }

    /// Follow splits and collect the consuming states' bytes. Counters and
    /// an early Match retain the single-byte approximation but rule out
    /// refinement. An assertion makes this offset unsuitable for scanning.
    fn collect(
        &mut self,
        states: &[State],
        classes: &[ByteClassBits],
        byte_tables: &[ByteMap],
    ) -> Option<([bool; 256], bool)> {
        let mut bytes = [false; 256];
        let mut can_refine = true;
        let mut cursor = 0;
        while let Some(&idx) = self.queue.get(cursor) {
            cursor += 1;
            match states[idx] {
                State::Split { out, out1 } => {
                    self.push(out);
                    self.push(out1);
                }
                State::CounterInstance { out, .. } => {
                    can_refine = false;
                    self.push(out);
                }
                State::CounterIncrement { out, out1, .. } => {
                    // An empty body may reach both the next iteration and the exit.
                    can_refine = false;
                    self.push(out);
                    self.push(out1);
                }
                State::Match => can_refine = false,
                State::Assert { .. } => return None,
                State::Byte { byte, .. } => bytes[usize::from(byte)] = true,
                State::ByteCI { byte, .. } => {
                    bytes[usize::from(byte)] = true;
                    bytes[usize::from(byte ^ 0x20)] = true;
                }
                State::Wildcard { .. } => bytes.fill(true),
                State::ByteClassStatic { table, .. } => {
                    for (slot, &accepted) in bytes.iter_mut().zip(table) {
                        *slot |= accepted;
                    }
                }
                State::ByteClassCustom { class, .. } => {
                    for b in 0..=255u8 {
                        bytes[usize::from(b)] |= classes[class.idx()].contains(b);
                    }
                }
                State::ByteTable { table } => {
                    for (slot, &target) in bytes.iter_mut().zip(&byte_tables[table.idx()].0) {
                        *slot |= target != StateIdx::NONE;
                    }
                }
            }
        }
        Some((bytes, can_refine))
    }

    /// Advance one consuming edge from each member. Both successors of
    /// a fused consume-and-split are reachable after the same input byte.
    fn successors(&self, states: &[State], byte_tables: &[ByteMap]) -> Self {
        let mut next = Self::new(states.len());
        for &idx in &self.queue {
            match states[idx] {
                State::Byte { out, out_exit, .. }
                | State::ByteCI { out, out_exit, .. }
                | State::Wildcard { out, out_exit }
                | State::ByteClassStatic { out, out_exit, .. }
                | State::ByteClassCustom { out, out_exit, .. } => {
                    next.push(out);
                    next.push(out_exit);
                }
                State::ByteTable { table } => {
                    for &target in &byte_tables[table.idx()].0 {
                        next.push(target);
                    }
                }
                State::Split { .. } => {}
                State::Assert { .. }
                | State::CounterInstance { .. }
                | State::CounterIncrement { .. }
                | State::Match => unreachable!("only refine an unconditional consuming frontier"),
            }
        }
        next
    }
}

/// Compute a start filter for a pattern that does not match the empty string.
/// A narrow first-byte range may also restrict the following byte when
/// every starting path consumes two bytes without passing a counter or assertion.
pub(crate) fn compute_prefilter(
    start: StateIdx,
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
) -> Prefilter {
    let mut first = Frontier::new(states.len());
    first.push(start);
    let Some((start_bytes, can_refine)) = first.collect(states, classes, byte_tables) else {
        return Prefilter::None;
    };

    let bytes: Vec<u8> = (0..=255u8)
        .filter(|&b| start_bytes[usize::from(b)])
        .collect();
    match bytes.len() {
        0 => Prefilter::None,
        1 => Prefilter::Memchr1(bytes[0]),
        2 => Prefilter::Memchr2(bytes[0], bytes[1]),
        3 => Prefilter::Memchr3(bytes[0], bytes[1], bytes[2]),
        4..=8 => Prefilter::Memclass(MemclassTable::new(&bytes)),
        _ => {
            let lo = bytes[0];
            let hi = bytes[bytes.len() - 1];
            if hi - lo >= 16 {
                return Prefilter::None;
            }
            if can_refine
                && let Some((next_bytes, true)) =
                    first
                        .successors(states, byte_tables)
                        .collect(states, classes, byte_tables)
            {
                let accepted = |b: &u8| next_bytes[usize::from(*b)];
                if let (Some(next_lo), Some(next_hi)) =
                    ((0..=255u8).find(accepted), (0..=255u8).rfind(accepted))
                    && next_hi - next_lo < 16
                {
                    return Prefilter::RangePair(lo, hi, next_lo, next_hi);
                }
            }
            Prefilter::Range(lo, hi)
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

    #[test]
    fn word_prefix_finds_the_first_literal_in_input_order() {
        for literal in 0..=255 {
            let filter = Prefilter::Memchr1(literal);
            for bits in 0_u16..256 {
                // XOR 1 in every other position exercises borrow propagation
                // beyond each true equality. Check every eight-byte hit mask.
                let word: [u8; 8] = std::array::from_fn(|i| {
                    if bits & (1 << i) != 0 {
                        literal
                    } else {
                        literal ^ 1
                    }
                });
                assert_eq!(
                    filter.find_in_prefix(&word),
                    word.iter().position(|&b| b == literal)
                );
            }
        }
    }

    #[test]
    fn singleton_membership_agrees_with_slice_search() {
        let filters = [
            Prefilter::None,
            Prefilter::Memchr1(255),
            Prefilter::Memchr2(0, 128),
            Prefilter::Memchr3(1, 127, 254),
            Prefilter::Memclass(MemclassTable::new(&[0, 15, 240, 255])),
            Prefilter::Range(176, 191),
            Prefilter::RangePair(120, 130, 250, 255),
        ];
        for filter in filters {
            for byte in 0..=255 {
                assert_eq!(filter.accepts(byte), filter.find(&[byte]).is_some());
            }
        }
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

    #[test]
    fn range_pair_date() {
        let re = Regex::new("([0-9][0-9]?)/([0-9][0-9]?)/([0-9][0-9]([0-9][0-9])?)")
            .expect("valid date pattern");
        assert!(matches!(
            re.prefilter,
            Prefilter::RangePair(b'0', b'9', b'/', b'9')
        ));
        assert_eq!(re.info().prefilter, "range_pair(0x30..=0x39, 0x2F..=0x39)");
        assert!(
            re.dump(false)
                .to_string()
                .contains("RangePair(48, 57, 47, 57)")
        );
        assert_eq!(re.prefilter.find_in_prefix(b"2/"), Some(0));
        assert_eq!(re.prefilter.find_in_prefix(b"2"), Some(0));
        assert_eq!(re.prefilter.find_in_prefix(b"2x3/"), Some(0));
        assert_eq!(re.prefilter.find(b"2x3/"), Some(2));
    }

    #[test]
    fn range_pair_includes_exit_from_first_consumer() {
        assert!(matches!(
            prefilter_for("[0-9]{1,2}/"),
            Prefilter::RangePair(b'0', b'9', b'/', b'9')
        ));
        assert!(matches!(
            prefilter_for("[0-9]{1,2}"),
            Prefilter::Range(b'0', b'9')
        ));
    }

    #[test]
    fn range_pair_byte_table_uses_all_destinations() {
        let re = Regex::new("0a|1b|2c|3d|4e|5f|6g|7h|8i|9j").expect("valid alternation");
        assert!(matches!(re.states[re.start], State::ByteTable { .. }));
        assert!(matches!(
            re.prefilter,
            Prefilter::RangePair(b'0', b'9', b'a', b'j')
        ));
        let mut memory = crate::MatcherMemory::default();
        for offset in 0..10 {
            let mut matcher = memory.matcher(&re);
            matcher.chunk(&[b'0' + offset, b'a' + offset]);
            assert!(matcher.finish());
        }
    }

    #[test]
    fn range_pair_refinement_guards() {
        for pattern in [r"[0-9]\b/", "[0-9][az]", "[0-9](?i:a)"] {
            assert!(
                matches!(prefilter_for(pattern), Prefilter::Range(b'0', b'9')),
                "{pattern}"
            );
        }
        for pattern in ["^[0-9]/", "[0-9]?"] {
            assert!(
                matches!(prefilter_for(pattern), Prefilter::None),
                "{pattern}"
            );
        }
        assert!(matches!(
            prefilter_for_no_unroll("[0-9]{2,5}/"),
            Prefilter::Range(b'0', b'9')
        ));
        assert!(matches!(
            prefilter_for("[0-9]/"),
            Prefilter::RangePair(b'0', b'9', b'/', b'/')
        ));
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
