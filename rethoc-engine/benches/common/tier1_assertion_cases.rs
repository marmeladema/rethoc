//! Existing-correct assertion workloads shared by timing and instruction gates.
//!
//! All fixtures are checked against the bytes-regex oracle before measurement.
//! `short` measures preparation/EOF costs; `classes` forces compressed tables;
//! `tiny` uses eight cached states. The literal-prefix churn case exercises that
//! cap. Cold/warm measurements differ only in whether setup primes the cache.

use rethoc_engine::{MatcherMemory, Regex, RegexConfig};

/// One immutable workload and its forced engine.
pub struct Case {
    pub name: &'static str,
    pub pattern: &'static str,
    pub repeat: &'static [u8],
    pub tail: &'static [u8],
    pub tier: u8,
}

/// Nonmatching controls and matching-at-the-end assertion cases.
pub const CASES: &[Case] = &[
    Case {
        name: "literal_sparse",
        pattern: "foo",
        repeat: b"abcdeghijklmnopqrstuvwxyz",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "literal_dense",
        pattern: "foo",
        repeat: b"fxfxfxfx",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "no_prefilter",
        pattern: "[ab]+z",
        repeat: b"abba",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "word",
        pattern: r"\bfoo\b",
        repeat: b" fooX !fooX ",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "word_late",
        pattern: r"\bfoo\b",
        repeat: b" fooX !fooX ",
        tail: b" foo ",
        tier: 1,
    },
    Case {
        name: "nonword",
        pattern: r"\Bfoo\B",
        repeat: b" foo ! foo ",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "directional",
        pattern: r"\b{start}foo\b{end}",
        repeat: b" fooX !fooX ",
        tail: b" foo ",
        tier: 1,
    },
    Case {
        name: "lf",
        pattern: r"(?m)^foo$",
        repeat: b"fooX\nfooX\r\n",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "lf_late",
        pattern: r"(?m)^foo$",
        repeat: b"fooX\nfooX\r\n",
        tail: b"\nfoo",
        tier: 1,
    },
    Case {
        name: "endlf",
        pattern: r"foo(?m:$)",
        repeat: b"fooX\nfooX\r\n",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "end",
        pattern: r"foo$",
        repeat: b"fooX",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "end_late",
        pattern: r"foo$",
        repeat: b"fooX",
        tail: b"foo",
        tier: 1,
    },
    Case {
        name: "anchored",
        pattern: r"^foo$",
        repeat: b"fooX",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "mixed",
        pattern: r"\bfoo\b(?m:$)",
        repeat: b" fooX\nfooX\r\n",
        tail: b"\nfoo",
        tier: 1,
    },
    Case {
        name: "churn",
        pattern: r"[ab]*a[ab]{5}z",
        repeat: b"aaaaaabaaaabbaababababbbabbbbbb",
        tail: b"",
        tier: 1,
    },
    Case {
        name: "tier2",
        pattern: r"foo.{0,100}bar",
        repeat: b"fxabcdefghijklmnop",
        tail: b"",
        tier: 2,
    },
    Case {
        name: "tier3",
        pattern: r"foo.{0,100}bar.{0,100}baz",
        repeat: b"fxabcdefghijklmnop",
        tail: b"",
        tier: 3,
    },
    Case {
        name: "tier4",
        pattern: r"(a{2,3}b){2,3}z",
        repeat: b"aabx",
        tail: b"",
        tier: 4,
    },
];

impl Case {
    /// Resolve a benchmark id without silently changing its workload.
    pub fn named(name: &str) -> &'static Self {
        CASES
            .iter()
            .find(|case| case.name == name)
            .expect("known assertion workload")
    }

    /// Compile without relying on automatic engine selection for controls.
    pub fn compile(&self, classes: bool) -> Regex {
        Regex::with_config(
            self.pattern,
            RegexConfig {
                byte_class_min_states: if classes { 0 } else { usize::MAX },
                max_unroll_states: if self.tier == 1 { 32 } else { 0 },
                ..RegexConfig::default()
            },
        )
        .expect("valid assertion benchmark")
    }
}

/// Prepared input; memory is kept across streams to distinguish cold and warm.
pub struct Prepared {
    pub regex: Regex,
    pub input: Vec<u8>,
    pub memory: MatcherMemory,
    pub chunk: usize,
    pub tier: u8,
}

impl Prepared {
    /// Build an oracle-checked fixture. Modes are explicit in benchmark ids.
    pub fn new(name: &str, mode: &str, warm: bool, length: usize) -> Self {
        let case = Case::named(name);
        let length = if mode == "short" { 32 } else { length };
        let mut input: Vec<_> = case.repeat.iter().copied().cycle().take(length).collect();
        input.extend_from_slice(case.tail);
        let regex = case.compile(matches!(mode, "classes" | "tiny"));
        let mut memory = MatcherMemory::default();
        if mode == "tiny" {
            memory.set_dfa_state_cap(Some(8));
        }
        let mut fixture = Self {
            regex,
            input,
            memory,
            chunk: if mode == "bytewise" { 1 } else { 16 * 1024 },
            tier: case.tier,
        };
        let oracle = regex::bytes::Regex::new(&format!("(?s-u){}", case.pattern))
            .expect("oracle accepts fixture");
        let expected = oracle.is_match(&fixture.input);
        // Validate fresh memory separately so cold measurement remains cold.
        let mut check = MatcherMemory::default();
        check.set_dfa_state_cap(if mode == "tiny" { Some(8) } else { None });
        assert_eq!(
            scan(
                &mut check,
                &fixture.regex,
                &fixture.input,
                fixture.chunk,
                fixture.tier
            ),
            expected,
            "{name}/{mode}"
        );
        if warm {
            assert_eq!(fixture.run(), expected, "warm {name}/{mode}");
        }
        fixture
    }

    /// Feed one complete stream through the requested engine.
    pub fn run(&mut self) -> bool {
        scan(
            &mut self.memory,
            &self.regex,
            &self.input,
            self.chunk,
            self.tier,
        )
    }
}

/// Scan through a forced engine, including its EOF handling.
pub fn scan(
    memory: &mut MatcherMemory,
    regex: &Regex,
    input: &[u8],
    chunk: usize,
    tier: u8,
) -> bool {
    let mut matcher = memory
        .matcher_for_tier(regex, tier)
        .expect("fixture tier remains eligible");
    for bytes in input.chunks(chunk) {
        matcher.chunk(bytes);
    }
    matcher.finish()
}
