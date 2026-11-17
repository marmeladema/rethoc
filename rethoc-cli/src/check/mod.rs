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
use std::sync::OnceLock;

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
    s.push_str(&regime_note());
    s
}

/// Environment variable that sets the DFA state cap of every matcher the
/// CLI creates.  An environment variable and not a flag so that the child
/// processes of `fuzz --batch` and `sweep` inherit it.
pub(crate) const DFA_STATES_ENV: &str = "RETHOC_DFA_STATES";

/// Environment variable that sets the NFA state count from which the DFA
/// tiers use byte equivalence classes (`0`: for every pattern).
pub(crate) const BYTE_CLASS_ENV: &str = "RETHOC_BYTE_CLASS_MIN_STATES";

/// Engine thresholds overridden through the environment.  Behaviour that
/// switches at a threshold is only exercised on one side of it by small
/// patterns and short inputs; these move the thresholds so that the
/// checking commands reach the other side.  No setting may change a
/// result.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Regime {
    /// [`DFA_STATES_ENV`]: the DFA state cap.  At 8, the minimum, the DFA
    /// caches are emptied every few bytes.
    pub(crate) dfa_states: Option<usize>,
    /// [`BYTE_CLASS_ENV`]: see [`RegexConfig::byte_class_min_states`].
    pub(crate) byte_class_min_states: Option<usize>,
}

/// The environment's [`Regime`], read once per process.
static REGIME: OnceLock<Regime> = OnceLock::new();

/// Read a variable holding a number: `None` when unset, an error message
/// when it is not a number.
fn read_number(name: &str) -> Result<Option<usize>, String> {
    match std::env::var(name) {
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(value)) => {
            Err(format!("{name}={value:?} is not a number"))
        }
        Ok(value) => value
            .parse()
            .map(Some)
            .map_err(|_| format!("{name}={value:?} is not a number")),
    }
}

fn read_regime() -> Result<Regime, String> {
    Ok(Regime {
        dfa_states: read_number(DFA_STATES_ENV)?,
        byte_class_min_states: read_number(BYTE_CLASS_ENV)?,
    })
}

/// Read and check the regime variables before anything else runs.  A bad
/// value is one usage error in the process the user started, not one
/// failure per child process.
pub(crate) fn init_regime() -> Result<(), String> {
    let regime = read_regime()?;
    // Already set only if something asked for it first.
    let _ = REGIME.set(regime);
    Ok(())
}

/// The thresholds requested through the environment.
pub(crate) fn regime() -> Regime {
    *REGIME.get_or_init(|| read_regime().unwrap_or_default())
}

/// A fresh [`MatcherMemory`], with the DFA state cap of the [`Regime`].
pub(crate) fn new_memory() -> MatcherMemory {
    let mut memory = MatcherMemory::default();
    memory.set_dfa_state_cap(regime().dfa_states);
    memory
}

/// The regime as it appears in summary and result lines, e.g.
/// ` dfa-states=8 byte-class-min=0`; empty at the defaults.  It goes into
/// every such line, so that a run in another regime cannot be mistaken
/// for one at the defaults.
pub(crate) fn regime_note() -> String {
    let regime = regime();
    let mut note = String::new();
    if let Some(n) = regime.dfa_states {
        note.push_str(&format!(" dfa-states={n}"));
    }
    if let Some(n) = regime.byte_class_min_states {
        note.push_str(&format!(" byte-class-min={n}"));
    }
    note
}

/// What to put in front of a reproduction command so that it runs in
/// the same regime (empty at the defaults).
pub(crate) fn repro_env() -> String {
    let regime = regime();
    let mut env = String::new();
    if let Some(n) = regime.dfa_states {
        env.push_str(&format!("{DFA_STATES_ENV}={n} "));
    }
    if let Some(n) = regime.byte_class_min_states {
        env.push_str(&format!("{BYTE_CLASS_ENV}={n} "));
    }
    env
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
    format!("{}{}", repro_env(), shell_join(&args))
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
