//! Experimental exact grouped NFA (`plans/grouped-nfa.md`).
//!
//! The Thompson NFA keeps one thread per `(state, counter context)`; a
//! state reached with many counts is visited, hashed and stepped once per
//! count.  For flat counters the context of a state is either empty or one
//! count of one known counter, so the set of contexts at a state fits a
//! bitset row: `state -> set of counts`.  This backend keeps one row per
//! reachable state and applies each NFA operation to whole rows:
//!
//! - a byte predicate is tested once per active state, not once per count;
//! - zero-width transfers run once per state, in a topological order of
//!   the zero-width graph computed at compile time, so no per-count visited
//!   set or worklist is needed;
//! - a `CounterIncrement` shifts its row and masks it into the continue
//!   and break successors.
//!
//! Nothing is approximated: expanding the rows gives exactly the NFA's
//! configurations.
//!
//! # Status
//!
//! Milestones P0 (analysis, opt-in plan) and P1 (one-word, assertion-free
//! runtime).  The backend is selected only explicitly: compile with
//! [`RegexConfig::grouped_nfa`](crate::RegexConfig::grouped_nfa) and run a
//! [`GroupedStream`].  Default dispatch and the tiers are unchanged.
//!
//! Admitted graphs ([`analysis`]): every reachable state has one incoming
//! scope (outside, or inside exactly one counter), the zero-width graph is
//! acyclic, and, for this milestone, there is no assertion, no unbounded
//! counter and no counter maximum above 63.
//!
//! # Streaming
//!
//! A stream persists the *raw* rows: the configurations right after a byte
//! was consumed, before any zero-width step.  Every call closes a copy of
//! them in worker scratch, so the stream itself never holds a partly closed
//! frontier (plan §5).  Without assertions the closure does not depend on
//! the boundary, but the boundary is passed through already so that
//! milestone P3 changes no call sequence.

mod analysis;
mod exact;

use std::fmt;

use crate::{AssertKind, ByteTableIdx, Regex, StateIdx};

pub use analysis::Rejection;
pub(crate) use analysis::{Limits, Scope, analyze};
pub use exact::{GroupedScratch, GroupedStream};

// ---------------------------------------------------------------------------
// Plan
// ---------------------------------------------------------------------------

/// A row: `words` consecutive `u64` in the frontier arena, from `off`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Row {
    off: u32,
    words: u32,
}

impl Row {
    /// No row (unreachable state, absent `out_exit`).
    const NONE: Row = Row {
        off: u32::MAX,
        words: 0,
    };

    #[inline]
    fn range(self) -> std::ops::Range<usize> {
        self.off as usize..(self.off + self.words) as usize
    }
}

/// Masks of a finite one-word `CounterIncrement` with bounds `{min,max}`.
///
/// Its incoming counts are below `max` (continuing excludes `max`), so the
/// incremented set `T = S << 1` lies in `1..=max` and fits the word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CincMasks {
    /// Counts that continue: `0..max`.
    cont: u64,
    /// Counts that break: `min..=max`.
    brk: u64,
}

impl CincMasks {
    /// Masks for `{min,max}`, `1 <= max <= 63`, `min <= max`.
    fn finite(min: usize, max: usize) -> Self {
        assert!((1..64).contains(&max) && min <= max, "one-word bounds");
        let upto_max = u64::MAX >> (63 - max);
        Self {
            cont: upto_max >> 1,
            brk: upto_max & !((1u64 << min) - 1),
        }
    }
}

/// A zero-width (or `Match`) state's transfer, in closure order.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ZeroOp {
    /// Union into both successors.
    Split { src: Row, out: Row, out1: Row },
    /// Outside present: insert count zero at `out`.
    CounterInstance { src: Row, out: Row },
    /// `T = {v + 1}`; `T ∩ 0..max` continues at `out`; Outside at `out1`
    /// iff `T ∩ min..=max` is nonempty.
    CounterIncrement {
        src: Row,
        out: Row,
        out1: Row,
        masks: CincMasks,
    },
    /// Union into `out` on `Pass` (not admitted before milestone P3).
    Assert {
        kind: AssertKind,
        src: Row,
        out: Row,
    },
    /// Nonempty (Outside present): a match.
    Match { src: Row },
}

/// How a consuming state moves its row on a byte it accepts.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ConsumerKind {
    /// A one-byte predicate with successors `out` and, unless
    /// [`Row::NONE`], the fused `out_exit`.
    Edge { out: Row, out_exit: Row },
    /// A byte dispatch table, read from [`Regex::byte_tables`].
    Table(ByteTableIdx),
}

/// A consuming state.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Consumer {
    state: StateIdx,
    src: Row,
    kind: ConsumerKind,
}

/// The immutable compiled plan of an admitted graph.
#[derive(Debug)]
pub(crate) struct GroupedNfaPlan {
    /// Zero-width and `Match` transfers in topological order.
    ops: Box<[ZeroOp]>,
    /// Consuming states.
    consumers: Box<[Consumer]>,
    /// Row of every state ([`Row::NONE`] if unreachable); read for
    /// `ByteTable` targets.
    row_of: Box<[Row]>,
    /// Scope of every row, in arena order (for display and tests).
    scopes: Box<[(StateIdx, Scope)]>,
    /// Row of the start state, reseeded at every boundary.
    start: Row,
    /// Words in one frontier (`W`).
    words: usize,
}

impl GroupedNfaPlan {
    /// Heap bytes of a plan with these array lengths.
    fn heap_bytes_for(states: usize, rows: usize, ops: usize, consumers: usize) -> usize {
        states * size_of::<Row>()
            + rows * size_of::<(StateIdx, Scope)>()
            + ops * size_of::<ZeroOp>()
            + consumers * size_of::<Consumer>()
    }

    /// Heap bytes of this plan's arrays.
    pub(crate) fn heap_bytes(&self) -> usize {
        Self::heap_bytes_for(
            self.row_of.len(),
            self.scopes.len(),
            self.ops.len(),
            self.consumers.len(),
        )
    }

    /// Words in one frontier.
    pub(crate) fn words(&self) -> usize {
        self.words
    }

    /// Rows: one per reachable state.
    pub(crate) fn rows(&self) -> usize {
        self.scopes.len()
    }
}

impl fmt::Display for GroupedNfaPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  rows: {} ({} words), start row {}",
            self.scopes.len(),
            self.words,
            self.start.off
        )?;
        for &(state, scope) in self.scopes.iter() {
            let row = self.row_of[state.idx()];
            writeln!(
                f,
                "    state {state}: {scope}, words {}..{}",
                row.off,
                row.off + row.words
            )?;
        }
        let state_of = |row: Row| -> String {
            self.scopes
                .iter()
                .find(|(s, _)| self.row_of[s.idx()] == row)
                .map_or_else(|| "-".to_string(), |(s, _)| s.to_string())
        };
        writeln!(f, "  closure order:")?;
        for op in self.ops.iter() {
            match *op {
                ZeroOp::Split { src, out, out1 } => writeln!(
                    f,
                    "    {} split -> {}, {}",
                    state_of(src),
                    state_of(out),
                    state_of(out1)
                )?,
                ZeroOp::CounterInstance { src, out } => {
                    writeln!(f, "    {} CI -> {}", state_of(src), state_of(out))?
                }
                ZeroOp::CounterIncrement {
                    src,
                    out,
                    out1,
                    masks,
                } => writeln!(
                    f,
                    "    {} CInc cont={:#x} -> {}, brk={:#x} -> {}",
                    state_of(src),
                    masks.cont,
                    state_of(out),
                    masks.brk,
                    state_of(out1)
                )?,
                ZeroOp::Assert { kind, src, out } => writeln!(
                    f,
                    "    {} {} -> {}",
                    state_of(src),
                    kind.label(),
                    state_of(out)
                )?,
                ZeroOp::Match { src } => writeln!(f, "    {} match", state_of(src))?,
            }
        }
        writeln!(f, "  consumers: {}", self.consumers.len())?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Result stored on Regex
// ---------------------------------------------------------------------------

/// The outcome of the grouped analysis of a regex compiled with
/// [`RegexConfig::grouped_nfa`](crate::RegexConfig::grouped_nfa).
#[derive(Debug)]
pub(crate) enum GroupedNfa {
    Admitted(GroupedNfaPlan),
    Rejected(Rejection),
}

impl GroupedNfa {
    /// Heap bytes beyond the boxed enum itself.
    pub(crate) fn heap_bytes(&self) -> usize {
        match self {
            GroupedNfa::Admitted(plan) => plan.heap_bytes(),
            GroupedNfa::Rejected(_) => 0,
        }
    }
}

/// Why a regex cannot run on the grouped backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupedNfaError {
    /// The regex was compiled without
    /// [`RegexConfig::grouped_nfa`](crate::RegexConfig::grouped_nfa).
    NotRequested,
    /// The analysis rejected the graph.
    Rejected(Rejection),
}

impl fmt::Display for GroupedNfaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRequested => {
                write!(f, "the regex was compiled without the grouped NFA option")
            }
            Self::Rejected(r) => write!(f, "not admitted by the grouped NFA: {r}"),
        }
    }
}

impl std::error::Error for GroupedNfaError {}

impl Regex {
    /// The grouped plan of this regex, or why there is none.
    pub(crate) fn grouped_plan(&self) -> Result<&GroupedNfaPlan, GroupedNfaError> {
        match self.grouped_nfa.as_deref() {
            None => Err(GroupedNfaError::NotRequested),
            Some(GroupedNfa::Rejected(r)) => Err(GroupedNfaError::Rejected(*r)),
            Some(GroupedNfa::Admitted(plan)) => Ok(plan),
        }
    }

    /// Whether this regex can run on the experimental grouped NFA, and
    /// why not.
    #[doc(hidden)]
    pub fn grouped_nfa_admission(&self) -> Result<(), GroupedNfaError> {
        self.grouped_plan().map(|_| ())
    }
}

#[cfg(test)]
pub(crate) mod tests;
