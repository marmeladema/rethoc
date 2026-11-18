//! Exact grouped NFA (`plans/grouped-nfa.md`).
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
//! Milestones P0–P5 (`plans/grouped-nfa.md`).  Automatic selection
//! ([`Regex::engine`](crate::Regex::engine)) runs the grouped NFA when
//! neither Tier 1 nor Tier 2 applies and it admits the pattern; see
//! [`GroupedNfaMode`](crate::GroupedNfaMode) to disable it or to analyse it
//! for every regex.  It can also be forced
//! ([`MatcherMemory::matcher_grouped`](crate::MatcherMemory::matcher_grouped))
//! or run on a [`GroupedStream`] with a separately owned
//! [`GroupedScratch`].
//!
//! [`sparse`] is an experimental variant on the same structural analysis:
//! its rows hold a few representative counts instead of all of them.  It
//! is only ever forced ([`SparseStream`]).
//!
//! Admitted graphs ([`analysis`]): every reachable state has one incoming
//! scope (outside, or inside exactly one counter), the zero-width graph is
//! acyclic, the frontier fits the word budget, and every assertion is of a
//! kind reviewed for raw replay (all current kinds).  A finite counter's rows hold `0..=max`; an
//! unbounded one saturates at `min` (`counter_step`), so its rows hold
//! `0..=min`.
//!
//! # Streaming
//!
//! A stream persists the *raw* rows: the configurations right after a byte
//! was consumed, before any zero-width step.  Every call closes a copy of
//! them in worker scratch, so the stream itself never holds a partly closed
//! frontier (plan §5):
//!
//! - before consuming byte `b`, the closure sees `b` as the next byte;
//! - at the end of a chunk it sees an unknown next byte, which can only
//!   discover definite matches (a `Pass` with the next byte unknown stays
//!   a `Pass` whatever comes);
//! - at `finish` it sees end of input.
//!
//! Every assertion of one closure sees the same boundary.  An assertion
//! that fails or defers simply blocks its path in that closure; the raw
//! rows are untouched, so the next closure (with the next byte, or at end
//! of input) evaluates it afresh.  Consuming a byte replaces the raw rows
//! by the consuming successors, so a blocked path never reaches a later
//! boundary.

mod analysis;
mod bitmap;
mod exact;
pub(crate) mod sparse;
pub mod stats;

use std::fmt;

use crate::{AssertKind, ByteTableIdx, Regex, StateIdx};

pub use analysis::Rejection;
#[cfg(test)]
pub(crate) use analysis::analyze;
pub(crate) use analysis::{Limits, Scope, layout, structure};
pub use exact::{GroupedMatcher, GroupedScratch, GroupedStream, Sweep};
pub use sparse::{SparseMatcher, SparseRejection, SparseScratch, SparseStream};

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
    #[inline]
    fn range(self) -> std::ops::Range<usize> {
        self.off as usize..(self.off + self.words) as usize
    }
}

/// The bounds of a `CounterIncrement`, as the row transfer needs them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Increment {
    /// `{min,max}`: domain `0..=max`.  Incoming counts are below `max`
    /// (continuing excludes `max`).
    Finite { min: u32, max: u32 },
    /// `{min,}`: counts saturate at `min` (`counter_step`), domain
    /// `0..=min`.  Every incremented count continues.
    Saturating { min: u32 },
}

impl Increment {
    /// From `CounterIncrement` bounds (`max == usize::MAX`: unbounded).
    /// The analysis has checked that the domain fits the word budget.
    fn new(min: usize, max: usize) -> Self {
        let narrow = |v: usize| u32::try_from(v).expect("domain within the word budget");
        if max == usize::MAX {
            Increment::Saturating { min: narrow(min) }
        } else {
            assert!(max >= 1 && min <= max, "admitted bounds");
            Increment::Finite {
                min: narrow(min),
                max: narrow(max),
            }
        }
    }

    /// Words of a row of this counter.
    #[cfg(test)]
    fn words(self) -> usize {
        match self {
            Increment::Finite { max, .. } => max as usize / 64 + 1,
            Increment::Saturating { min } => min as usize / 64 + 1,
        }
    }
}

impl fmt::Display for Increment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Increment::Finite { min, max } => write!(f, "{{{min},{max}}}"),
            Increment::Saturating { min } => write!(f, "{{{min},}}"),
        }
    }
}

/// Index of a node: a reachable state's position in the topological order
/// of the zero-width graph.  Rows and active bits are indexed by it, and
/// every zero-width edge goes from a lower to a higher node.
pub(crate) type NodeIdx = u32;

/// No node (an absent `out_exit`, an unreachable state).
pub(crate) const NO_NODE: NodeIdx = u32::MAX;

/// A successor: its node and the first word of its row, so that a
/// transfer needs no lookup.  Its width follows from the scope proof: the
/// source's, except one word for Outside after a `CounterIncrement` break
/// and the counter's after a `CounterInstance`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Succ {
    node: NodeIdx,
    off: u32,
}

impl Succ {
    /// No successor (an absent `out_exit`).
    const NONE: Succ = Succ {
        node: NO_NODE,
        off: u32::MAX,
    };

    /// The successor's row, of width `words`.
    #[inline]
    fn row(self, words: u32) -> Row {
        Row {
            off: self.off,
            words,
        }
    }
}

/// What a node does with its row.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Op {
    /// Union into both successors.
    Split { out: Succ, out1: Succ },
    /// Outside present: insert count zero at `out`.
    CounterInstance { out: Succ },
    /// `T = {counter_step(v)}`; the counts of `T` that continue go to
    /// `out`; Outside at `out1` iff some count of `T` breaks.
    CounterIncrement {
        out: Succ,
        out1: Succ,
        inc: Increment,
    },
    /// Union into `out` iff the shared evaluator says `Pass` at the
    /// current boundary.
    Assert { kind: AssertKind, out: Succ },
    /// Nonempty (Outside present): a match.
    Match,
    /// Consumes with the one-byte predicate of its state; successors `out`
    /// and, unless [`Succ::NONE`], the fused `out_exit`.
    Edge { out: Succ, out_exit: Succ },
    /// Consumes through a byte dispatch table of [`Regex::byte_tables`].
    Table(ByteTableIdx),
}

/// A reachable state.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Node {
    state: StateIdx,
    scope: Scope,
    row: Row,
    op: Op,
}

/// The immutable compiled plan of an admitted graph.
#[derive(Debug)]
pub(crate) struct GroupedNfaPlan {
    /// Reachable states in topological order of the zero-width graph;
    /// rows are laid out in the same order.
    nodes: Box<[Node]>,
    /// Node of every state ([`NO_NODE`] if unreachable); read for
    /// `ByteTable` targets.
    node_of: Box<[NodeIdx]>,
    /// Copies of the zero-width and `Match` nodes, in order: the full
    /// sweep's closure walks these contiguously, without indexing `nodes`.
    zero_width: Box<[Node]>,
    /// Copies of the consuming nodes, in order: the full sweep's
    /// consumption.
    consumers: Box<[Node]>,
    /// Node of the start state, reseeded at every boundary.
    start: NodeIdx,
    /// Words in one frontier (`W`).
    words: usize,
    /// Words of one active-row bitmap: one bit per node.
    active_words: usize,
    /// Whether the start state's zero-width closure has no assertion and
    /// no `Match`, so that the closure of an empty frontier is the same at
    /// every boundary and never matches: bytes the regex's prefilter rules
    /// out can then be skipped while the frontier is empty.
    skip_when_idle: bool,
    /// Compile-time tables for count-free frontiers, when the start node
    /// is count-free and the bitmaps are at most 8 words (512 rows).
    pub(crate) count_free: Option<Box<CountFreeTables>>,
}

/// Compile-time tables for frontiers where no counter has counted yet.
///
/// A raw frontier is *unit* when every active row holds exactly bit 0
/// (Outside, or count 0).  A node is *count-free* when its zero-width
/// closure reaches no assertion and no `CounterIncrement`: from a unit row
/// that closure is a fixed set of nodes, each holding bit 0 (`Split`
/// copies, `CounterInstance` turns Outside into count 0), whatever the
/// boundary.  So when the raw frontier is unit and all its nodes are
/// count-free, the closure is the OR of their closure bitmaps and the
/// start's, the consumers that fire are that bitmap AND the byte's
/// acceptance bitmap, and their successors, each holding bit 0, are the
/// next raw frontier: a step by bitmaps, without the closure pass.
#[derive(Debug)]
pub(crate) struct CountFreeTables {
    /// Words per bitmap (`active_words`).
    pub(crate) a: usize,
    /// Closure bitmap of each count-free node (`a` words per node; zero
    /// for the others).
    pub(crate) closure: Box<[u64]>,
    /// Consuming nodes that accept each byte (`a` words per byte).
    pub(crate) accept: Box<[u64]>,
    /// The count-free nodes.
    pub(crate) statics: Box<[u64]>,
    /// The `Match` nodes.
    pub(crate) matches: Box<[u64]>,
    /// Closure bitmap of the start node.
    pub(crate) start_closure: Box<[u64]>,
    /// Whether some consumer of the start closure accepts each byte: from
    /// an empty frontier, the other bytes leave it empty.
    pub(crate) starts: Box<[bool; 256]>,
}

impl CountFreeTables {
    /// Heap bytes of the tables of a plan of `nodes` nodes and `a` bitmap
    /// words, the boxed struct included.
    pub(crate) fn bytes_for(nodes: usize, a: usize) -> usize {
        size_of::<Self>() + (nodes + 256 + 3) * a * size_of::<u64>() + size_of::<[bool; 256]>()
    }
}

impl GroupedNfaPlan {
    /// Heap bytes of a plan with these array lengths, and count-free
    /// tables of `count_free` bitmap words if any.
    fn heap_bytes_for(states: usize, nodes: usize, count_free: Option<usize>) -> usize {
        // `node_of`, the nodes, and their copies split in two lists.
        states * size_of::<NodeIdx>()
            + 2 * nodes * size_of::<Node>()
            + count_free.map_or(0, |a| CountFreeTables::bytes_for(nodes, a))
    }

    /// Heap bytes of this plan's arrays.
    pub(crate) fn heap_bytes(&self) -> usize {
        debug_assert_eq!(
            self.zero_width.len() + self.consumers.len(),
            self.nodes.len()
        );
        Self::heap_bytes_for(
            self.node_of.len(),
            self.nodes.len(),
            self.count_free.as_ref().map(|t| t.a),
        )
    }

    /// Words in one frontier.
    pub(crate) fn words(&self) -> usize {
        self.words
    }

    /// Words of one active-row bitmap.
    pub(crate) fn active_words(&self) -> usize {
        self.active_words
    }

    /// Rows: one per reachable state.
    pub(crate) fn rows(&self) -> usize {
        self.nodes.len()
    }

    /// The row of a node.
    #[inline]
    fn row(&self, node: NodeIdx) -> Row {
        self.nodes[node as usize].row
    }

    /// Reachable states with their scope and row, in node order.
    #[cfg(test)]
    pub(crate) fn rows_iter(&self) -> impl Iterator<Item = (StateIdx, Scope, Row)> + '_ {
        self.nodes.iter().map(|n| (n.state, n.scope, n.row))
    }

    /// The row of a state, if it is reachable.
    #[cfg(test)]
    pub(crate) fn row_of_state(&self, state: StateIdx) -> Option<Row> {
        let n = self.node_of[state.idx()];
        (n != NO_NODE).then(|| self.row(n))
    }
}

impl fmt::Display for GroupedNfaPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  rows: {} ({} words), start state {}, skip when idle: {}, count-free tables: {}",
            self.nodes.len(),
            self.words,
            self.nodes[self.start as usize].state,
            self.skip_when_idle,
            self.count_free.is_some()
        )?;
        let st = |s: Succ| -> String {
            if s.node == NO_NODE {
                "-".into()
            } else {
                self.nodes[s.node as usize].state.to_string()
            }
        };
        writeln!(f, "  nodes, in closure order:")?;
        for node in self.nodes.iter() {
            let (s, row) = (node.state, node.row);
            write!(
                f,
                "    {s} ({}, words {}..{}): ",
                node.scope,
                row.off,
                row.off + row.words
            )?;
            match node.op {
                Op::Split { out, out1 } => writeln!(f, "split -> {}, {}", st(out), st(out1))?,
                Op::CounterInstance { out } => writeln!(f, "CI -> {}", st(out))?,
                Op::CounterIncrement { out, out1, inc } => {
                    writeln!(f, "CInc{inc} cont -> {}, break -> {}", st(out), st(out1))?
                }
                Op::Assert { kind, out } => writeln!(f, "{} -> {}", kind.label(), st(out))?,
                Op::Match => writeln!(f, "match")?,
                Op::Edge { out, out_exit } => {
                    write!(f, "consume -> {}", st(out))?;
                    if out_exit != Succ::NONE {
                        write!(f, ", {}", st(out_exit))?;
                    }
                    writeln!(f)?
                }
                Op::Table(t) => writeln!(f, "consume via table {}", t.idx())?,
            }
        }
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

/// The outcome of the sparse counting analysis of a regex compiled with
/// [`RegexConfig::sparse_counting`](crate::RegexConfig::sparse_counting).
#[derive(Debug)]
pub(crate) enum SparseCounting {
    Admitted(sparse::SparsePlan),
    Rejected(SparseRejection),
}

impl SparseCounting {
    /// Heap bytes beyond the boxed enum itself.
    pub(crate) fn heap_bytes(&self) -> usize {
        match self {
            SparseCounting::Admitted(plan) => plan.heap_bytes(),
            SparseCounting::Rejected(_) => 0,
        }
    }
}

/// Why a regex cannot run on the sparse counting variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SparseCountingError {
    /// The regex was compiled without
    /// [`RegexConfig::sparse_counting`](crate::RegexConfig::sparse_counting).
    NotRequested,
    /// The analysis rejected the graph.
    Rejected(SparseRejection),
}

impl fmt::Display for SparseCountingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRequested => write!(
                f,
                "sparse counting was not analysed for this regex (compile with \
                 RegexConfig::sparse_counting)"
            ),
            Self::Rejected(r) => write!(f, "not admitted by sparse counting: {r}"),
        }
    }
}

impl std::error::Error for SparseCountingError {}

/// Why a regex cannot run on the grouped backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupedNfaError {
    /// The grouped NFA was not analysed for this regex: disabled
    /// ([`GroupedNfaMode::Disabled`](crate::GroupedNfaMode::Disabled)), or
    /// not needed by automatic selection because Tier 1 or 2 runs it
    /// (compile with [`GroupedNfaMode::Always`](crate::GroupedNfaMode::Always)
    /// to force it).
    NotRequested,
    /// The analysis rejected the graph.
    Rejected(Rejection),
}

impl fmt::Display for GroupedNfaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRequested => write!(
                f,
                "the grouped NFA was not analysed for this regex (disabled, or Tier 1/2 runs it; \
                 compile with GroupedNfaMode::Always to force it)"
            ),
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

    /// The sparse counting plan of this regex, or why there is none.
    pub(crate) fn sparse_plan(&self) -> Result<&sparse::SparsePlan, SparseCountingError> {
        match self.sparse_counting.as_deref() {
            None => Err(SparseCountingError::NotRequested),
            Some(SparseCounting::Rejected(r)) => Err(SparseCountingError::Rejected(*r)),
            Some(SparseCounting::Admitted(plan)) => Ok(plan),
        }
    }

    /// Whether this regex can run on the sparse counting variant, and why
    /// not.
    #[doc(hidden)]
    pub fn sparse_counting_admission(&self) -> Result<(), SparseCountingError> {
        self.sparse_plan().map(|_| ())
    }

    /// Whether this regex can run on the grouped NFA, and
    /// why not.
    #[doc(hidden)]
    pub fn grouped_nfa_admission(&self) -> Result<(), GroupedNfaError> {
        self.grouped_plan().map(|_| ())
    }

    /// Experimental: feed `input` whole to a fresh grouped NFA stream and
    /// finish it; `None` if this regex cannot run on the grouped NFA.  For
    /// the differential harnesses.
    #[doc(hidden)]
    pub fn grouped_is_match(&self, scratch: &mut GroupedScratch, input: &[u8]) -> Option<bool> {
        let mut stream = GroupedStream::new(self, scratch).ok()?;
        stream.chunk(self, scratch, input);
        Some(stream.finish(self, scratch))
    }
}

#[cfg(test)]
pub(crate) mod assertion_tests;
#[cfg(test)]
mod selection_tests;
#[cfg(test)]
pub(crate) mod tests;
