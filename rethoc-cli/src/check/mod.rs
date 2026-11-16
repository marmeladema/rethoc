//! Differential checking: `brute`, `fuzz`, `shrink` and `sweep`.
//!
//! Every command judges a pattern the same way ([`Subject::check`]): the
//! `regex` crate (compiled with `(?s-u)`, as in the test suite) is the
//! reference, and the NFA and every eligible DFA tier, each fed the whole
//! input and fed one byte at a time, must agree with it.
//!
//! Inputs are read and printed with Rust byte escapes ([`escape`],
//! [`unescape`]), so any input a command prints can be pasted back into
//! `shrink` or `brute`.

pub(crate) mod brute;
pub(crate) mod cap;
pub(crate) mod fuzz;
pub(crate) mod shrink;
pub(crate) mod sweep;

use std::fmt;

use rethoc_engine::{MatcherMemory, Regex, RegexConfig};

use crate::CompileArgs;

/// No mismatch.
pub(crate) const EXIT_OK: i32 = 0;
/// At least one mismatch.
pub(crate) const EXIT_MISMATCH: i32 = 1;
/// Bad command line.
pub(crate) const EXIT_USAGE: i32 = 2;
/// Aborted: watchdog, timeout, out of memory or crash.
pub(crate) const EXIT_ABORT: i32 = 3;
/// The pattern was rejected (by rethoc in every configuration, or by the
/// reference), so nothing was checked.
pub(crate) const EXIT_REJECTED: i32 = 4;

// ---------------------------------------------------------------------------
// Reference and subject
// ---------------------------------------------------------------------------

/// The `regex` crate, configured like the test suite's oracle.
pub(crate) struct Reference(regex::bytes::Regex);

impl Reference {
    /// Compile `pattern` as `(?s-u)pattern`.
    pub(crate) fn new(pattern: &str) -> Result<Self, regex::Error> {
        regex::bytes::Regex::new(&format!("(?s-u){pattern}")).map(Self)
    }

    /// Whether `pattern` matches anywhere in `input`.
    pub(crate) fn is_match(&self, input: &[u8]) -> bool {
        self.0.is_match(input)
    }
}

/// How an input is fed to a matcher.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Feed {
    /// One `chunk()` call with the whole input.
    Whole,
    /// One `chunk()` call per byte.
    Bytewise,
}

impl fmt::Display for Feed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whole => "whole",
            Self::Bytewise => "bytewise",
        })
    }
}

/// One disagreement with the reference.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Failure {
    /// Tier that disagreed (0 = NFA).
    pub(crate) tier: u8,
    /// How the input was fed.
    pub(crate) feed: Feed,
    /// What the tier answered; the reference said the opposite.
    pub(crate) got: bool,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tier{}/{}={}", self.tier, self.feed, self.got)
    }
}

/// Why a pattern could not be checked in a configuration.
pub(crate) enum Rejection {
    Compile(rethoc_engine::Error),
    NotEligible { tier: u8, min_tier: u8 },
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile(e) => write!(f, "compile error: {e}"),
            Self::NotEligible { tier, min_tier } => {
                write!(f, "not eligible for tier {tier} (min_tier={min_tier})")
            }
        }
    }
}

/// A pattern compiled in one configuration, with the tiers to check.
pub(crate) struct Subject {
    /// Configuration the pattern was compiled with.
    pub(crate) config: RegexConfig,
    /// The compiled pattern.
    pub(crate) regex: Regex,
    /// Tier 0 first, then the eligible DFA tiers (or only the requested one).
    pub(crate) tiers: Vec<u8>,
}

impl Subject {
    /// Compile `pattern` and find the tiers to check.  With `only_tier`,
    /// check the NFA and that tier; otherwise every eligible tier.
    pub(crate) fn compile(
        pattern: &str,
        config: &RegexConfig,
        only_tier: Option<u8>,
        memory: &mut MatcherMemory,
    ) -> Result<Self, Rejection> {
        let regex = Regex::with_config(pattern, config.clone()).map_err(Rejection::Compile)?;
        let candidates: Vec<u8> = match only_tier {
            None => (0..=4).collect(),
            Some(0) => vec![0],
            Some(t) => vec![0, t],
        };
        let tiers: Vec<u8> = candidates
            .into_iter()
            .filter(|&t| memory.matcher_for_tier(&regex, t).is_ok())
            .collect();
        if let Some(t) = only_tier
            && !tiers.contains(&t)
        {
            return Err(Rejection::NotEligible {
                tier: t,
                min_tier: regex.min_tier(),
            });
        }
        Ok(Self {
            config: config.clone(),
            regex,
            tiers,
        })
    }

    /// Run `input` on every tier and feed; push each disagreement with
    /// `expected` into `failures` (which is cleared first).
    pub(crate) fn check(
        &self,
        memory: &mut MatcherMemory,
        input: &[u8],
        expected: bool,
        failures: &mut Vec<Failure>,
    ) {
        failures.clear();
        for &tier in &self.tiers {
            for feed in [Feed::Whole, Feed::Bytewise] {
                let got = self.run(memory, tier, feed, input);
                if got != expected {
                    failures.push(Failure { tier, feed, got });
                }
            }
        }
    }

    /// Run `input` on one tier.
    pub(crate) fn run(
        &self,
        memory: &mut MatcherMemory,
        tier: u8,
        feed: Feed,
        input: &[u8],
    ) -> bool {
        let mut m = memory
            .matcher_for_tier(&self.regex, tier)
            .expect("tier was checked for eligibility");
        match feed {
            Feed::Whole => m.chunk(input),
            Feed::Bytewise => {
                for b in input {
                    m.chunk(std::slice::from_ref(b));
                }
            }
        }
        m.finish()
    }
}

// ---------------------------------------------------------------------------
// Configurations
// ---------------------------------------------------------------------------

/// `unroll=0 merge=false`, plus `max-states=N` when not the default.
pub(crate) fn describe(config: &RegexConfig) -> String {
    let mut s = format!(
        "unroll={} merge={}",
        config.max_unroll_states, config.merge_repetitions
    );
    if config.max_estimated_states != RegexConfig::default().max_estimated_states {
        s.push_str(&format!(" max-states={}", config.max_estimated_states));
    }
    s
}

/// Command-line flags that select exactly `config`.
pub(crate) fn config_args(config: &RegexConfig) -> Vec<String> {
    CompileArgs::exact(config).to_args()
}

/// A `rethoc shrink` command line reproducing a mismatch.
pub(crate) fn shrink_command(pattern: &str, config: &RegexConfig, input: &[u8]) -> String {
    let mut args = vec!["rethoc".to_string(), "shrink".into()];
    args.extend(config_args(config));
    let escaped = escape(input);
    if pattern.starts_with('-') || escaped.starts_with('-') {
        args.push("--".into());
    }
    args.push(pattern.into());
    args.push(escaped);
    shell_join(&args)
}

// ---------------------------------------------------------------------------
// Escapes and quoting
// ---------------------------------------------------------------------------

/// Render bytes with Rust byte escapes (`[u8]::escape_ascii`).
pub(crate) fn escape(input: &[u8]) -> String {
    input.escape_ascii().to_string()
}

/// Parse the escapes [`escape`] produces: `\n`, `\r`, `\t`, `\0`, `\\`,
/// `\'`, `\"` and `\xNN`.
pub(crate) fn unescape(s: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        if b != b'\\' {
            out.push(b);
            continue;
        }
        let e = bytes
            .next()
            .ok_or_else(|| format!("trailing backslash in {s:?}"))?;
        out.push(match e {
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'0' => 0,
            b'\\' | b'\'' | b'"' => e,
            b'x' => {
                let hex: Vec<u8> = bytes.by_ref().take(2).collect();
                let hex = std::str::from_utf8(&hex).unwrap_or("");
                u8::from_str_radix(hex, 16)
                    .ok()
                    .filter(|_| hex.len() == 2)
                    .ok_or_else(|| format!("bad \\x escape in {s:?}"))?
            }
            other => return Err(format!("unknown escape \\{} in {s:?}", other as char)),
        });
    }
    Ok(out)
}

/// Quote `s` for a POSIX shell if needed.
pub(crate) fn shell_quote(s: &str) -> String {
    let safe = !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_=./,:@+%".contains(&b));
    if safe {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\'', r"'\''"))
    }
}

/// Join arguments into a shell command line.
pub(crate) fn shell_join(args: &[String]) -> String {
    args.iter()
        .map(|a| shell_quote(a))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_round_trip() {
        let all: Vec<u8> = (0..=255).collect();
        assert_eq!(unescape(&escape(&all)).expect("valid escapes"), all);
    }

    #[test]
    fn unescape_rejects_bad_escapes() {
        assert!(unescape(r"a\").is_err());
        assert!(unescape(r"\q").is_err());
        assert!(unescape(r"\x4").is_err());
        assert!(unescape(r"\xzz").is_err());
    }

    #[test]
    fn shell_quote_quotes_when_needed() {
        assert_eq!(shell_quote("abc"), "abc");
        assert_eq!(shell_quote("a b"), "'a b'");
        assert_eq!(shell_quote("it's"), r"'it'\''s'");
        assert_eq!(shell_quote(""), "''");
    }
}
