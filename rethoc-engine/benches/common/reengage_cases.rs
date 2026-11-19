//! Streams for the prefilter re-engagement benchmarks, shared by the
//! Criterion bench (`reengage.rs`, wall-clock throughput) and the Callgrind
//! bench (`reengage_profile.rs`, instruction counts).
//!
//! Each case scans the full stream. The overlap cases match at EOF; the
//! other streams have no match. The streams are the inputs on which the counting
//! tiers used to fall off the prefilter (or off the DFA fast path) for the
//! rest of the stream, the inputs on which re-arming the prefilter is pure
//! overhead, and a few controls on paths that must stay unaffected.

use rethoc_engine::{AnyMatcher, MatcherMemory, Regex, RegexConfig};

/// Chunk size used to feed the stream: the engine is a streaming matcher,
/// and re-engagement state has to survive chunk boundaries.
pub const CHUNK: usize = 16 * 1024;

/// One benchmark case.
pub struct Case {
    /// Benchmark id.
    pub name: &'static str,
    pub pattern: &'static str,
    /// Tier the pattern must compile to.  Checked in [`Case::compile`] so
    /// that a tier-selection change cannot silently change what is measured.
    pub tier: u8,
    /// Divides the haystack size.  Greater than 1 for cases that are slow
    /// by nature (live counter instances on every byte).
    pub shrink: usize,
    /// Builds the stream for a given length.
    pub haystack: fn(usize) -> Vec<u8>,
}

/// Single counted gap between two literals (Tier 2).
const ONE_GAP: &str = "foo.{0,100}bar";
/// Two counted gaps (Tier 3, per-instance).
const TWO_GAPS: &str = "foo.{0,100}bar.{0,100}baz";
/// Rule-like pattern: case-insensitive alternations around two gaps.
const RULE: &str = "(?i)(select|union|insert).{0,40}(from|into).{0,40}(users|passwd)";

/// Lower-case letters without `f`: no byte can start `foo`.
const NO_F: &[u8] = b"abcdeghijklmnopqrstuvwxyz";
/// Mixed text: every anchor byte occurs, about once per 70 bytes each.
const TEXT: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 /=&?.-_";

pub const CASES: &[Case] = &[
    // -- A counter ran once; afterwards nothing in the stream is a candidate.
    //    The matcher has to get back to the prefilter. --
    Case {
        name: "tier2_gap_once",
        pattern: ONE_GAP,
        tier: 2,
        shrink: 1,
        haystack: |n| once(b"foo", NO_F, n),
    },
    Case {
        name: "tier3_candidate_once",
        pattern: TWO_GAPS,
        tier: 3,
        shrink: 1,
        haystack: |n| once(b"fx", NO_F, n),
    },
    Case {
        name: "tier3_gap_once",
        pattern: TWO_GAPS,
        tier: 3,
        shrink: 1,
        haystack: |n| once(b"foo", NO_F, n),
    },
    Case {
        name: "tier3_two_gaps_once",
        pattern: TWO_GAPS,
        tier: 3,
        shrink: 1,
        haystack: |n| once(b"fooxbar", NO_F, n),
    },
    // -- Start states the prefilter cannot serve.  The matcher still has to
    //    leave the counting slow path once the gap has expired. --
    Case {
        name: "tier2_word_boundary_gap_once",
        pattern: "\\bfoo.{0,100}bar",
        tier: 2,
        shrink: 1,
        haystack: |n| once(b"foo", b"x", n),
    },
    Case {
        name: "tier2_anchored_gap_once",
        pattern: "^foo.{0,100}bar",
        tier: 2,
        shrink: 1,
        haystack: |n| once(b"foo", b"x", n),
    },
    Case {
        name: "tier2_anchored_dead",
        pattern: "^foo.{0,100}bar",
        tier: 2,
        shrink: 1,
        haystack: |n| once(b"fox", b"x", n),
    },
    Case {
        name: "tier3_no_prefilter_gap_once",
        pattern: "[a-z0-9]oo.{0,100}bar.{0,100}baz",
        tier: 3,
        shrink: 1,
        haystack: |n| once(b"aoo", b"!", n),
    },
    // -- Streams on which watching for the start state cannot pay off:
    //    candidates that fail at once, or no prefilter at all.  These must
    //    not be slower than plain DFA stepping. --
    Case {
        name: "tier3_dense_candidates",
        pattern: TWO_GAPS,
        tier: 3,
        shrink: 1,
        haystack: |n| random(b"fx", n),
    },
    Case {
        name: "tier3_candidate_run",
        pattern: TWO_GAPS,
        tier: 3,
        shrink: 1,
        haystack: |n| random(b"f", n),
    },
    Case {
        name: "tier3_no_prefilter_start",
        pattern: "[^q]oo.{0,100}bar.{0,100}baz",
        tier: 3,
        shrink: 1,
        haystack: |n| random(b"q", n),
    },
    Case {
        name: "tier2_dense_candidates",
        pattern: ONE_GAP,
        tier: 2,
        shrink: 1,
        haystack: |n| random(b"fx", n),
    },
    // -- Mixed text: anchors, failed candidates and expiring gaps all
    //    occur naturally. --
    Case {
        name: "tier2_gap_text",
        pattern: ONE_GAP,
        tier: 2,
        shrink: 1,
        haystack: |n| random(TEXT, n),
    },
    Case {
        name: "tier3_gaps_text",
        pattern: TWO_GAPS,
        tier: 3,
        shrink: 1,
        haystack: |n| random(TEXT, n),
    },
    Case {
        name: "tier3_rule_text",
        pattern: RULE,
        tier: 3,
        shrink: 1,
        haystack: |n| random(TEXT, n),
    },
    // -- Controls: paths the re-engagement logic must not slow down. --
    Case {
        name: "control_tier1_literal_text",
        pattern: "foobar",
        tier: 1,
        shrink: 1,
        haystack: |n| random(TEXT, n),
    },
    Case {
        name: "control_tier1_no_prefilter_text",
        pattern: "\\d+\\.\\d+\\.\\d+\\.\\d+:\\d+",
        tier: 1,
        shrink: 1,
        haystack: |n| random(TEXT, n),
    },
    Case {
        name: "control_tier2_live_counter",
        pattern: "a.{0,1000}c",
        tier: 2,
        shrink: 1,
        haystack: |n| random(b"ab", n),
    },
    Case {
        name: "control_tier3_live_instances",
        pattern: "(ab|c){5,300}x",
        tier: 3,
        shrink: 8,
        haystack: |n| random(b"abc", n),
    },
];

impl Case {
    /// Look a case up by its benchmark id.
    #[allow(dead_code)] // only the Callgrind bench selects cases by name
    pub fn named(name: &str) -> &'static Case {
        CASES
            .iter()
            .find(|c| c.name == name)
            .unwrap_or_else(|| panic!("unknown re-engagement case `{name}`"))
    }

    /// Compile the pattern and build a stream of about `size` bytes.
    ///
    /// Panics if the pattern does not compile to the expected tier or if
    /// the stream matches: either would make the numbers meaningless.
    pub fn compile(&self, size: usize) -> (Regex, Vec<u8>) {
        let re = Regex::new(self.pattern).expect("benchmark pattern compiles");
        assert_eq!(
            re.min_tier(),
            self.tier,
            "case `{}`: `{}` no longer compiles to tier {}",
            self.name,
            self.pattern,
            self.tier
        );
        let haystack = (self.haystack)(size / self.shrink);
        let mut memory = MatcherMemory::default();
        assert!(
            !scan(&mut memory, &re, &haystack),
            "case `{}`: the stream must not match",
            self.name
        );
        (re, haystack)
    }
}

/// Workloads for overlapping counters admitted by Tier 2.
#[derive(Clone, Copy)]
pub enum OverlapCase {
    Expiry,
    MixedLengths,
}

impl OverlapCase {
    /// Benchmark id shared by the two runners.
    pub fn name(self) -> &'static str {
        match self {
            Self::Expiry => "tier2_overlap_expiry",
            Self::MixedLengths => "tier2_overlap_mixed_lengths",
        }
    }

    /// Build a stream ending in a new candidate after earlier attempts fail.
    pub fn compile(self, size: usize) -> (Regex, Vec<u8>) {
        let (pattern, mut haystack, tail): (_, _, &[u8]) = match self {
            Self::Expiry => {
                // Both counters expire on the fifth b. Re-enter at the final x.
                let mut haystack = vec![b'b'; size - 3];
                haystack[0] = b'x';
                (r"x(?:[ab]{2,4}|[bc]{2,4})$", haystack, b"xbb")
            }
            Self::MixedLengths => {
                // b advances both branches; every second b crosses both increments.
                // a kills the two-byte body, c kills the one-byte body, ! kills both.
                // Only the two-byte body can accept the final bbbc.
                let block = b"xbbbba!xbbbbbc!";
                // Scale the live-counter stream to keep the Callgrind run short.
                let haystack = block.repeat(size / 8 / block.len());
                (r"x(?:[ab]{2,8}|(?:b[bc]){2,4})$", haystack, b"xbbbc")
            }
        };
        let regex = Regex::with_config(
            pattern,
            RegexConfig {
                max_unroll_states: 0,
                ..Default::default()
            },
        )
        .expect("valid overlap benchmark pattern");
        let mut memory = MatcherMemory::default();
        assert!(
            !scan_overlap(&mut memory, &regex, &haystack),
            "{}",
            self.name()
        );
        haystack.extend_from_slice(tail);
        assert!(
            scan_overlap(&mut memory, &regex, &haystack),
            "{}",
            self.name()
        );
        (regex, haystack)
    }
}

/// Scan an overlap workload through forced Tier 2.
pub fn scan_overlap(memory: &mut MatcherMemory, re: &Regex, haystack: &[u8]) -> bool {
    let matcher = memory
        .matcher_for_tier(re, 2)
        .expect("overlap must admit Tier 2");
    scan_chunks(matcher, haystack)
}

/// Feed `haystack` to a new matcher in [`CHUNK`]-sized pieces.
pub fn scan(memory: &mut MatcherMemory, re: &Regex, haystack: &[u8]) -> bool {
    scan_chunks(memory.matcher(re), haystack)
}

fn scan_chunks(mut matcher: AnyMatcher<'_>, haystack: &[u8]) -> bool {
    for chunk in haystack.chunks(CHUNK) {
        matcher.chunk(chunk);
    }
    matcher.finish()
}

/// `prefix`, then pseudo-random bytes drawn from `alphabet`.
fn once(prefix: &[u8], alphabet: &[u8], len: usize) -> Vec<u8> {
    let mut haystack = prefix.to_vec();
    haystack.extend(random(alphabet, len.saturating_sub(prefix.len())));
    haystack
}

/// `len` pseudo-random bytes drawn uniformly from `alphabet`
/// (xorshift64, fixed seed: every run sees the same stream).
fn random(alphabet: &[u8], len: usize) -> Vec<u8> {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            alphabet[(state >> 33) as usize % alphabet.len()]
        })
        .collect()
}
