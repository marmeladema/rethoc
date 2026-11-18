//! Sparse counting sets on the grouped NFA (`plans/sparse-counting-sets.md`):
//! an experimental variant that keeps, for every state inside a counted
//! scope, a short sorted list of representative counts instead of a bitset
//! of all of them.  It is never selected automatically.
//!
//! # Counts
//!
//! A row inside the scope of a finite counter `{l,h}` holds *active
//! iteration numbers* `u` in `1..=h`: the iteration being matched, one
//! more than the completed iterations `c` the exact grouped rows hold
//! (`u = c + 1`).  A `CounterInstance` enters with `{1}`.  A
//! `CounterIncrement` reads its row once: the exit fires iff some
//! `u >= l`, and `{u + 1 : u < h}` continues into the body.  The exit is
//! tested on the row *before* it is incremented.  Zero is never a count,
//! and an empty row is not a row holding zero.
//!
//! # Representatives
//!
//! A row is a strictly ascending list of the counts it stands for, without
//! those that no future can tell apart from the rest.  With
//! `k = h - l + 1`, a value `b` between `a < b < c` is dropped when
//! `c - a <= k` ([`push`]): after `t` more iterations a count `u` can exit
//! iff `max(0, l - u) <= t <= h - u`, and the intervals of `a` and `c`
//! then cover that of `b`.  The smallest count is always kept, so the row
//! also dies (every count past `h`) exactly when the full set would.  What
//! is kept is a subset of the exact set with the same answers to "can
//! this exit after `t` more iterations" and "is anything left after `t`
//! more" for every `t`: not the exact set, and not a canonical form of it.
//!
//! A reduced list has `v[i + 2] - v[i] > k`, hence at most
//! `D = min(h, 2 * ceil(h / (k + 1)))` values ([`representatives`]).  Every
//! counted row reserves `D` slots; nothing is ever truncated to fit.
//!
//! No transfer may look at counts in any other way: a guard that kept,
//! say, the counts below some value would tell a reduced row from the set
//! it stands for.
//!
//! # Assertions
//!
//! An assertion passes or blocks its whole row, whatever the counts: it
//! reads only the boundary (stream start, previous byte, next byte or end
//! of input), through the evaluator the exact grouped NFA uses, for the
//! kinds reviewed there ([`reviewed`](super::analysis::reviewed)).  The
//! stream keeps the frontier as it was right after the last byte and
//! closes a copy of it at every boundary, so an assertion that failed or
//! could not be decided blocks one edge of one closure and nothing else:
//! the next closure, with the next byte or at end of input, evaluates it
//! afresh from the same rows, and once a byte is consumed the blocked path
//! is gone.  Rows merge at a state only after each incoming path has
//! passed its own guards.

use std::fmt;

use super::analysis::{Rejection, Scope, Structure, analysis_bytes};
use super::exact::{Boundary, BoundaryCtx, Sweep, accepts};
use super::stats::stat;
use super::{NO_NODE, NodeIdx, SparseCountingError};
use crate::{AssertEval, AssertKind, ByteClassBits, ByteMap, ByteTableIdx, Regex, State, StateIdx};

// ---------------------------------------------------------------------------
// Limits and rejections
// ---------------------------------------------------------------------------

/// Admission limits of the experiment (plan §3).  Engineering caps, not
/// measured crossovers.  The limits on states, edges and the structural
/// analysis are those of the shared [`Structure`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SparseLimits {
    /// Most representatives of one counted row (`D`).
    pub(crate) max_reps: usize,
    /// Most reserved representative slots in one frontier.
    pub(crate) max_slots: usize,
    /// Most bytes of retained plan arrays.
    pub(crate) max_plan_bytes: usize,
    /// Most bytes of transient analysis arrays, those of the shared
    /// structure included.
    pub(crate) max_analysis_bytes: usize,
}

impl SparseLimits {
    pub(crate) const DEFAULT: SparseLimits = SparseLimits {
        max_reps: 32,
        max_slots: 16_384,
        max_plan_bytes: 1 << 20,
        max_analysis_bytes: 2 << 20,
    };
}

/// Why a graph is not admitted by the sparse variant.  Normal compilation,
/// dispatch and the exact grouped NFA are unaffected by any of these.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SparseRejection {
    /// The structure shared with the exact grouped NFA was not proven.
    Structure(Rejection),
    /// A state is inside the scope of a counter without a reachable
    /// increment, whose bounds are therefore unknown.
    ScopeWithoutIncrement { counter: u8 },
    /// The counter is unbounded (`{min,}`): its saturated counts need a
    /// mapping of their own.
    UnboundedCounter { counter: u8 },
    /// Bounds whose counts do not fit the `u32` representatives.
    BoundsTooLarge { counter: u8, max: usize },
    /// The counter needs more representatives per row than the limit.
    TooManyRepresentatives {
        counter: u8,
        needed: usize,
        limit: usize,
    },
    /// The frontier would reserve more representative slots than the
    /// limit.
    TooManySlots { slots: usize, limit: usize },
    /// The transient analysis arrays would exceed their budget.
    AnalysisTooLarge { bytes: usize, limit: usize },
    /// The retained plan arrays would exceed their budget.
    PlanTooLarge { bytes: usize, limit: usize },
}

impl fmt::Display for SparseRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Structure(r) => write!(f, "{r}"),
            Self::ScopeWithoutIncrement { counter } => {
                write!(
                    f,
                    "the scope of counter {counter} has no reachable increment"
                )
            }
            Self::UnboundedCounter { counter } => write!(
                f,
                "counter {counter} is unbounded: the sparse variant admits finite counters only"
            ),
            Self::BoundsTooLarge { counter, max } => {
                write!(
                    f,
                    "counter {counter}: a maximum of {max} does not fit a representative"
                )
            }
            Self::TooManyRepresentatives {
                counter,
                needed,
                limit,
            } => write!(
                f,
                "counter {counter} needs {needed} representatives per row, over the limit of {limit}"
            ),
            Self::TooManySlots { slots, limit } => write!(
                f,
                "{slots} representative slots per frontier exceed the limit of {limit}"
            ),
            Self::AnalysisTooLarge { bytes, limit } => write!(
                f,
                "analysis arrays of {bytes} bytes exceed the limit of {limit}"
            ),
            Self::PlanTooLarge { bytes, limit } => {
                write!(
                    f,
                    "plan arrays of {bytes} bytes exceed the limit of {limit}"
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The representative bound
// ---------------------------------------------------------------------------

/// The spacing `k = h - l + 1` and the row capacity
/// `D = min(h, 2 * ceil(h / (k + 1)))` of a finite counter `{min,max}`, or
/// `None` when the bounds are not ones the lists are defined for:
/// `max == 0`, `min > max`, or a maximum at or above `u32::MAX` (every
/// count and `k`, at most `max + 1`, must fit a `u32`).
pub(crate) fn representatives(min: usize, max: usize) -> Option<(u32, usize)> {
    if max == 0 || min > max || max >= u32::MAX as usize {
        return None;
    }
    let (l, h) = (min as u64, max as u64);
    let k = h - l + 1;
    let d = (2 * h.div_ceil(k + 1)).min(h);
    Some((k as u32, d as usize))
}

// ---------------------------------------------------------------------------
// List primitives
// ---------------------------------------------------------------------------

/// Push `v`, greater than every value of the reduced list `list[..*n]`,
/// and drop the values it makes redundant: while the last three values
/// `a < b < c` have `c - a <= k`, remove `b`.  Leaves a reduced list.
///
/// `list` needs room for one value more than the row capacity: the push
/// happens before the removal.
#[inline]
pub(crate) fn push(list: &mut [u32], n: &mut usize, v: u32, k: u32) {
    debug_assert!(*n == 0 || list[*n - 1] < v, "values are pushed in order");
    list[*n] = v;
    *n += 1;
    while *n >= 3 && list[*n - 1] - list[*n - 3] <= k {
        list[*n - 2] = list[*n - 1];
        *n -= 1;
    }
}

/// Reduce the strictly ascending `list` in place, by one left-to-right
/// scan; returns the new length.
#[cfg(test)]
pub(crate) fn reduce(list: &mut [u32], k: u32) -> usize {
    let mut n = 0;
    for i in 0..list.len() {
        let v = list[i];
        push(list, &mut n, v, k);
    }
    n
}

/// The reduced union of the reduced lists `a` and `{v + shift : v in b,
/// v + shift <= max}`, written to `out`; returns its length.  `shift` is 0
/// for a plain union (`max` then at least every value) and 1 for the
/// continuation of a `CounterIncrement`, which drops the counts that
/// reached `max`.
///
/// `out` must not overlap the inputs and needs room for one value more
/// than the row capacity.  The inputs are only read: the caller writes the
/// result back, so a source row stays available for its other successors.
#[inline]
pub(crate) fn union(a: &[u32], b: &[u32], shift: u32, max: u32, k: u32, out: &mut [u32]) -> usize {
    debug_assert!(shift <= 1);
    // A value of `b` is at most `max`, so only its last one can pass
    // `max` when shifted.
    let b = match b.split_last() {
        Some((&last, rest)) if last > max - shift => rest,
        _ => b,
    };
    let (mut i, mut j, mut n) = (0, 0, 0);
    while i < a.len() && j < b.len() {
        let (x, y) = (a[i], b[j] + shift);
        let v = x.min(y);
        i += usize::from(x == v);
        j += usize::from(y == v);
        push(out, &mut n, v, k);
    }
    for &x in &a[i..] {
        push(out, &mut n, x, k);
    }
    for &y in &b[j..] {
        push(out, &mut n, y + shift, k);
    }
    n
}

/// Whether a `CounterIncrement` with minimum `min` exits from the
/// nonempty row `list`: some count has reached `min`, so the greatest
/// has.
#[inline]
pub(crate) fn exits(list: &[u32], min: u32) -> bool {
    list.last().is_some_and(|&u| u >= min)
}

// ---------------------------------------------------------------------------
// Plan
// ---------------------------------------------------------------------------

/// A successor: its node and the first slot of its row, so that a transfer
/// needs no lookup.  Its capacity and spacing are the source's (the scope
/// proof), except Outside after the exit of a `CounterIncrement` and the
/// counter's after a `CounterInstance`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Succ {
    pub(crate) node: NodeIdx,
    pub(crate) off: u32,
}

impl Succ {
    /// No successor (an absent `out_exit`).
    pub(crate) const NONE: Succ = Succ {
        node: NO_NODE,
        off: u32::MAX,
    };
}

/// What a node does with its row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SparseOp {
    /// Union into both successors.
    Split { out: Succ, out1: Succ },
    /// Outside active: insert count one at `out`, a row of the counter
    /// with spacing `k` and capacity `cap`.
    Instance { out: Succ, k: u32, cap: u32 },
    /// Outside at `out1` iff some count is at least `min`; the counts
    /// below `max`, plus one, continue to `out`.
    Increment {
        out: Succ,
        out1: NodeIdx,
        min: u32,
        max: u32,
    },
    /// Union the whole row into `out` iff the shared evaluator says
    /// `Pass` at the current boundary.
    Assert { kind: AssertKind, out: Succ },
    /// Active: a match.
    Match,
    /// Consumes with the one-byte predicate of its state; successors `out`
    /// and, unless [`Succ::NONE`], the fused `out_exit`.
    Edge { out: Succ, out_exit: Succ },
    /// Consumes through a byte dispatch table.
    Table(ByteTableIdx),
}

/// A reachable state and its row.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SparseNode {
    /// Its own index: its position in the topological order.
    pub(crate) node: NodeIdx,
    pub(crate) state: StateIdx,
    pub(crate) scope: Scope,
    /// First slot of the row in the representative arena.
    pub(crate) off: u32,
    /// Reserved slots: `D` of the counter, 0 for an Outside row, which is
    /// only active or not.
    pub(crate) cap: u32,
    /// The spacing `k` of the counter (0 Outside).
    pub(crate) k: u32,
    pub(crate) op: SparseOp,
}

/// The immutable compiled plan of an admitted graph.
#[derive(Debug)]
pub(crate) struct SparsePlan {
    /// Reachable states in topological order of the zero-width graph;
    /// rows are laid out in the same order.
    pub(crate) nodes: Box<[SparseNode]>,
    /// Node of every state ([`NO_NODE`] if unreachable); read for
    /// `ByteTable` targets.
    pub(crate) node_of: Box<[NodeIdx]>,
    /// Copies of the zero-width and `Match` nodes, in order: the full
    /// sweep's closure walks these contiguously.
    zero_width: Box<[SparseNode]>,
    /// Copies of the consuming nodes, in order: the full sweep's
    /// consumption.
    consumers: Box<[SparseNode]>,
    /// Node of the start state, reseeded at every boundary.
    pub(crate) start: NodeIdx,
    /// Representative slots of one frontier.
    pub(crate) slots: usize,
    /// The largest row capacity (0 without a counter).
    pub(crate) max_cap: usize,
    /// Words of one active-row bitmap: one bit per node.
    active_words: usize,
    /// When the zero-width closure of the start reaches neither an
    /// assertion nor `Match`: the bytes some consuming state of that
    /// closure accepts.  The closure of an empty frontier is then the same
    /// at every boundary and never matches, and any other byte leaves the
    /// frontier empty: an idle stream skips to the next such byte.
    pub(crate) idle: Option<Box<[bool; 256]>>,
}

impl SparsePlan {
    /// Heap bytes of a plan with these array lengths, with or without the
    /// idle table.
    fn heap_bytes_for(states: usize, nodes: usize, idle: bool) -> usize {
        // `node_of`, the nodes, and their copies split in two lists.
        states * size_of::<NodeIdx>()
            + 2 * nodes * size_of::<SparseNode>()
            + if idle { size_of::<[bool; 256]>() } else { 0 }
    }

    /// Heap bytes of this plan's arrays.
    pub(crate) fn heap_bytes(&self) -> usize {
        debug_assert_eq!(
            self.zero_width.len() + self.consumers.len(),
            self.nodes.len()
        );
        Self::heap_bytes_for(self.node_of.len(), self.nodes.len(), self.idle.is_some())
    }

    /// Rows: one per reachable state.
    pub(crate) fn rows(&self) -> usize {
        self.nodes.len()
    }

    /// Rows inside a counted scope.
    pub(crate) fn counted_rows(&self) -> usize {
        self.nodes.iter().filter(|n| n.cap > 0).count()
    }

    /// The counted scopes, by counter: bounds and row capacity.
    pub(crate) fn scopes(&self) -> Vec<crate::SparseScopeInfo> {
        let mut scopes: Vec<crate::SparseScopeInfo> = Vec::new();
        for node in self.nodes.iter() {
            if let (Scope::Counter(c), SparseOp::Increment { min, max, .. }) = (node.scope, node.op)
                && scopes.iter().all(|s| s.counter != c.idx())
            {
                scopes.push(crate::SparseScopeInfo {
                    counter: c.idx(),
                    min: min as usize,
                    max: max as usize,
                    capacity: node.cap as usize,
                });
            }
        }
        scopes.sort_by_key(|s| s.counter);
        scopes
    }
}

/// The plan, for `rethoc dump`.  A counted row lists its slots: the count
/// values it keeps are active iteration numbers `u` (one more than the
/// completed iterations), and only as many of them as the futures of the
/// row can tell apart.
impl fmt::Display for SparsePlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  rows: {} ({} counted, {} count slots), start state {}, skip when idle: {}",
            self.rows(),
            self.counted_rows(),
            self.slots,
            self.nodes[self.start as usize].state,
            self.idle.is_some()
        )?;
        for s in self.scopes() {
            writeln!(
                f,
                "  counter {}: {{{},{}}}, counts u in 1..={}, at most {} values per row",
                s.counter, s.min, s.max, s.max, s.capacity
            )?;
        }
        let st = |n: NodeIdx| -> String {
            if n == NO_NODE {
                "-".into()
            } else {
                self.nodes[n as usize].state.to_string()
            }
        };
        writeln!(f, "  nodes, in closure order:")?;
        for node in self.nodes.iter() {
            write!(f, "    {} ({}", node.state, node.scope)?;
            if node.cap > 0 {
                write!(f, ", slots {}..{}", node.off, node.off + node.cap)?;
            }
            write!(f, "): ")?;
            match node.op {
                SparseOp::Split { out, out1 } => {
                    writeln!(f, "split -> {}, {}", st(out.node), st(out1.node))?
                }
                SparseOp::Instance { out, .. } => writeln!(f, "CI: u = 1 -> {}", st(out.node))?,
                SparseOp::Increment {
                    out,
                    out1,
                    min,
                    max,
                } => writeln!(
                    f,
                    "CInc: exit if u >= {min} -> {}, then u + 1 if u < {max} -> {}",
                    st(out1),
                    st(out.node)
                )?,
                SparseOp::Assert { kind, out } => {
                    writeln!(f, "{} -> {}", kind.label(), st(out.node))?
                }
                SparseOp::Match => writeln!(f, "match")?,
                SparseOp::Edge { out, out_exit } => {
                    write!(f, "consume -> {}", st(out.node))?;
                    if out_exit != Succ::NONE {
                        write!(f, ", {}", st(out_exit.node))?;
                    }
                    writeln!(f)?
                }
                SparseOp::Table(t) => writeln!(f, "consume via table {}", t.idx())?,
            }
        }
        Ok(())
    }
}

/// Upper bound of the transient arrays of [`analyze`] and of the
/// [`Structure`] it reads: on top of the structural analysis, the spacing
/// and capacity per counter, the row offsets, and the flags and stack of
/// the walk over the start closure.
pub(crate) fn sparse_analysis_bytes(states: usize, num_counters: usize) -> usize {
    analysis_bytes(states, num_counters)
        + num_counters * size_of::<Option<(u32, u32)>>()
        + states * (size_of::<u32>() + size_of::<bool>() + size_of::<StateIdx>())
}

/// The bytes that some consuming state of the zero-width closure of
/// `start` accepts, or `None` when that closure reaches an assertion or
/// `Match` (it then depends on the boundary, or matches).
fn start_bytes(
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
    start: StateIdx,
) -> Option<Box<[bool; 256]>> {
    let mut starts = Box::new([false; 256]);
    let mut seen = vec![false; states.len()];
    let mut stack = vec![start];
    while let Some(idx) = stack.pop() {
        if std::mem::replace(&mut seen[idx.idx()], true) {
            continue;
        }
        match states[idx] {
            State::Assert { .. } | State::Match => return None,
            State::Split { out, out1 } => stack.extend([out, out1]),
            State::CounterInstance { out, .. } => stack.push(out),
            // Not reachable without a consumed byte in an admitted graph
            // (it would close a zero-width cycle); were it, the closure
            // would not be the fixed set this table stands for.
            State::CounterIncrement { .. } => return None,
            State::ByteTable { table } => {
                for b in 0..=255u8 {
                    starts[b as usize] |= byte_tables[table][b] != StateIdx::NONE;
                }
            }
            ref state => {
                for b in 0..=255u8 {
                    starts[b as usize] |= accepts(classes, state, b);
                }
            }
        }
    }
    Some(starts)
}

/// Lay out the sparse rows of a proven [`Structure`] and build the plan,
/// or say why not.  `structure` is the outcome of
/// [`analysis::structure`](super::analysis::structure) on `states`.
pub(crate) fn analyze(
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
    start: StateIdx,
    structure: Result<&Structure, Rejection>,
    limits: &SparseLimits,
) -> Result<SparsePlan, SparseRejection> {
    let structure = structure.map_err(SparseRejection::Structure)?;
    let Structure {
        label,
        order,
        bounds,
    } = structure;
    let analysis = sparse_analysis_bytes(states.len(), bounds.len());
    if analysis > limits.max_analysis_bytes {
        return Err(SparseRejection::AnalysisTooLarge {
            bytes: analysis,
            limit: limits.max_analysis_bytes,
        });
    }

    // Spacing and capacity of every counter with a reachable increment.
    let mut shape: Vec<Option<(u32, u32)>> = vec![None; bounds.len()];
    for (c, b) in bounds.iter().enumerate() {
        let Some((min, max)) = *b else { continue };
        let counter = c as u8;
        if max == usize::MAX {
            return Err(SparseRejection::UnboundedCounter { counter });
        }
        let Some((k, d)) = representatives(min, max) else {
            return Err(SparseRejection::BoundsTooLarge { counter, max });
        };
        // A row length is a `u16`.
        if d > limits.max_reps.min(u16::MAX as usize) {
            return Err(SparseRejection::TooManyRepresentatives {
                counter,
                needed: d,
                limit: limits.max_reps,
            });
        }
        shape[c] = Some((k, d as u32));
    }
    let shape_of = |scope: Scope| -> Result<(u32, u32), SparseRejection> {
        match scope {
            Scope::Outside => Ok((0, 0)),
            Scope::Counter(c) => {
                shape[c.idx()].ok_or(SparseRejection::ScopeWithoutIncrement { counter: c.0 })
            }
        }
    };

    // Slots, before anything is allocated for them.
    let mut slots = 0usize;
    for &idx in order {
        let (_, cap) = shape_of(label[idx.idx()].expect("reachable"))?;
        slots = slots.saturating_add(cap as usize);
    }
    if slots > limits.max_slots {
        return Err(SparseRejection::TooManySlots {
            slots,
            limit: limits.max_slots,
        });
    }
    // The idle table is optional: a plan that fits without it is admitted
    // without it.
    let mut idle = start_bytes(states, classes, byte_tables, start);
    let mut plan_bytes = SparsePlan::heap_bytes_for(states.len(), order.len(), idle.is_some());
    if plan_bytes > limits.max_plan_bytes && idle.is_some() {
        idle = None;
        plan_bytes = SparsePlan::heap_bytes_for(states.len(), order.len(), false);
    }
    if plan_bytes > limits.max_plan_bytes {
        return Err(SparseRejection::PlanTooLarge {
            bytes: plan_bytes,
            limit: limits.max_plan_bytes,
        });
    }

    let mut node_of = vec![NO_NODE; states.len()];
    for (i, &idx) in order.iter().enumerate() {
        node_of[idx.idx()] = i as NodeIdx;
    }
    // Row offsets in node order.
    let mut offs = Vec::with_capacity(order.len());
    let mut off = 0u32;
    for &idx in order {
        offs.push(off);
        off += shape_of(label[idx.idx()].expect("reachable"))?.1;
    }
    debug_assert_eq!(off as usize, slots);
    let succ = |s: StateIdx| -> Succ {
        let node = node_of[s.idx()];
        debug_assert!(
            node != NO_NODE,
            "successor of a reachable state is reachable"
        );
        Succ {
            node,
            off: offs[node as usize],
        }
    };
    let mut nodes = Vec::with_capacity(order.len());
    let mut max_cap = 0usize;
    for (i, &idx) in order.iter().enumerate() {
        let scope = label[idx.idx()].expect("reachable");
        let (k, cap) = shape_of(scope)?;
        let op = match states[idx] {
            State::Split { out, out1 } => SparseOp::Split {
                out: succ(out),
                out1: succ(out1),
            },
            State::CounterInstance { counter, out } => {
                let (k, cap) = shape_of(Scope::Counter(counter))?;
                SparseOp::Instance {
                    out: succ(out),
                    k,
                    cap,
                }
            }
            State::CounterIncrement {
                out,
                out1,
                min,
                max,
                ..
            } => SparseOp::Increment {
                out: succ(out),
                out1: succ(out1).node,
                // `representatives` accepted these bounds: they fit.
                min: min as u32,
                max: max as u32,
            },
            State::Assert { kind, out } => SparseOp::Assert {
                kind,
                out: succ(out),
            },
            State::Match => SparseOp::Match,
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => SparseOp::Edge {
                out: succ(out),
                out_exit: if out_exit == StateIdx::NONE {
                    Succ::NONE
                } else {
                    succ(out_exit)
                },
            },
            State::ByteTable { table } => SparseOp::Table(table),
        };
        nodes.push(SparseNode {
            node: i as NodeIdx,
            state: idx,
            scope,
            off: offs[i],
            cap,
            k,
            op,
        });
        max_cap = max_cap.max(cap as usize);
    }
    let (consumers, zero_width): (Vec<SparseNode>, Vec<SparseNode>) = nodes
        .iter()
        .partition(|n| matches!(n.op, SparseOp::Edge { .. } | SparseOp::Table(_)));
    let plan = SparsePlan {
        start: node_of[start.idx()],
        active_words: nodes.len().div_ceil(64),
        zero_width: zero_width.into_boxed_slice(),
        consumers: consumers.into_boxed_slice(),
        nodes: nodes.into_boxed_slice(),
        node_of: node_of.into_boxed_slice(),
        slots,
        max_cap,
        idle,
    };
    debug_assert_eq!(plan.heap_bytes(), plan_bytes);
    Ok(plan)
}

// ---------------------------------------------------------------------------
// Frontiers
// ---------------------------------------------------------------------------
//
// A frontier is a length per node, the representative arena and a bitmap
// of the nonempty rows.  The length of an Outside row is 1 when it is
// active and 0 otherwise; that of a counted row is its number of
// representatives, 0 when it is empty.  Slots past the length hold stale
// values and are never read.  The bitmap of the stream's raw frontier is
// always exact; that of the closure in the scratch is kept by the
// active-row pass only.  Between calls the scratch's lengths and bitmap
// are all zero.
//
// Each byte runs one of two equivalent passes, chosen by [`Sweep`] like
// the exact grouped NFA's:
//
// - **Full sweep** walks every node in order, skipping empty rows.
// - **Active rows** takes the lowest pending node of the bitmap each time;
//   a transfer only activates higher nodes (edges of the zero-width graph
//   go up the topological order), so nodes are processed in increasing
//   order and each after all its predecessors, exactly like the full
//   sweep.
//
// Under the automatic choice an idle stream (no active row) also skips the
// bytes that cannot start anything ([`SparsePlan::idle`]).

/// Plans of at most this many rows always take the full sweep; larger ones
/// take it when at least an eighth of their rows are active.
const SMALL_PLAN_ROWS: usize = 32;

#[inline(always)]
fn activate(active: &mut [u64], n: NodeIdx) {
    active[n as usize / 64] |= 1 << (n % 64);
}

/// Call `f` for every active node, in increasing order.
#[inline(always)]
fn for_each_active(active: &[u64], mut f: impl FnMut(NodeIdx)) {
    for (w, &word) in active.iter().enumerate() {
        let mut bits = word;
        while bits != 0 {
            f((w * 64) as NodeIdx + bits.trailing_zeros());
            bits &= bits - 1;
        }
    }
}

/// `to[dst..dst + count] = from[src..src + count]`.  Rows are a few values
/// long: a loop, not a `memcpy` call.
#[inline(always)]
#[allow(clippy::manual_memcpy)]
fn copy_values(from: &[u32], src: usize, to: &mut [u32], dst: usize, count: usize) {
    let (from, to) = (&from[src..src + count], &mut to[dst..dst + count]);
    for i in 0..count {
        to[i] = from[i];
    }
}

/// [`copy_values`] within one arena, between rows that do not overlap.
#[inline(always)]
fn copy_values_within(vals: &mut [u32], src: usize, dst: usize, count: usize) {
    debug_assert!(src + count <= dst || dst + count <= src, "rows overlap");
    assert!(src + count <= vals.len() && dst + count <= vals.len());
    for i in 0..count {
        vals[dst + i] = vals[src + i];
    }
}

/// A capacity violation: a reduced list always fits its row (see the
/// module documentation), so this is a bug in the transfers, never a
/// reason to truncate.
#[cold]
#[inline(never)]
fn over_capacity(count: usize, cap: u32) -> ! {
    panic!("a reduced row of {count} values exceeds its capacity of {cap}")
}

/// Union the `count` values at `from[src..]` into the counted row `dst` of
/// the frontier `(len, vals)`, of spacing `k` and capacity `cap`.  Returns
/// whether the row was empty.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn union_row(
    from: &[u32],
    src: usize,
    count: usize,
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    dst: Succ,
    k: u32,
    cap: u32,
) -> bool {
    stat!(words_unioned += count);
    stat!(fanout += 1);
    let (d, off) = (dst.node as usize, dst.off as usize);
    let held = len[d] as usize;
    let source = &from[src..src + count];
    if held == 0 {
        // The source is reduced already.
        copy_values(from, src, vals, off, count);
        len[d] = count as u16;
        return true;
    }
    let n = union(&vals[off..off + held], source, 0, u32::MAX, k, merge);
    if n > cap as usize {
        over_capacity(n, cap);
    }
    copy_values(merge, 0, vals, off, n);
    len[d] = n as u16;
    false
}

/// [`union_row`] within one arena: the row at `src` into the row `dst`.
/// Rows of distinct nodes never overlap.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn union_within(
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    src: usize,
    count: usize,
    dst: Succ,
    k: u32,
    cap: u32,
) -> bool {
    stat!(words_unioned += count);
    stat!(fanout += 1);
    let (d, off) = (dst.node as usize, dst.off as usize);
    debug_assert_ne!(src, off, "union of a row into itself");
    let held = len[d] as usize;
    if held == 0 {
        copy_values_within(vals, src, off, count);
        len[d] = count as u16;
        return true;
    }
    let n = union(
        &vals[off..off + held],
        &vals[src..src + count],
        0,
        u32::MAX,
        k,
        merge,
    );
    if n > cap as usize {
        over_capacity(n, cap);
    }
    copy_values(merge, 0, vals, off, n);
    len[d] = n as u16;
    false
}

/// Move the whole row of `node` (`count` values) to the successor `out`
/// in the same scope, within the closure frontier.
#[inline(always)]
fn pass_within(
    node: &SparseNode,
    count: usize,
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    out: Succ,
) {
    if node.cap == 0 {
        stat!(fanout += 1);
        len[out.node as usize] = 1;
    } else {
        union_within(
            len,
            vals,
            merge,
            node.off as usize,
            count,
            out,
            node.k,
            node.cap,
        );
    }
}

/// Apply the zero-width transfer of `node` (whose row is nonempty) within
/// the closure frontier, calling `on` for every successor that received
/// something.  Returns whether `node` is a reached `Match`.
///
/// A source row is not changed by its own transfer, so each of its
/// successors reads the same counts.
#[inline(always)]
fn transfer(
    node: &SparseNode,
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    ctx: BoundaryCtx,
    mut on: impl FnMut(NodeIdx),
) -> bool {
    let (src, count) = (node.off as usize, len[node.node as usize] as usize);
    match node.op {
        SparseOp::Split { out, out1 } => {
            stat!(rows_closed += 1);
            pass_within(node, count, len, vals, merge, out);
            on(out.node);
            pass_within(node, count, len, vals, merge, out1);
            on(out1.node);
        }
        SparseOp::Instance { out, k, cap } => {
            stat!(rows_closed += 1);
            stat!(fanout += 1);
            let (d, off) = (out.node as usize, out.off as usize);
            let held = len[d] as usize;
            if held == 0 {
                vals[off] = 1;
                len[d] = 1;
            } else if vals[off] != 1 {
                // One is below every count the row holds.
                let n = union(&[1], &vals[off..off + held], 0, u32::MAX, k, merge);
                if n > cap as usize {
                    over_capacity(n, cap);
                }
                copy_values(merge, 0, vals, off, n);
                len[d] = n as u16;
            }
            on(out.node);
        }
        SparseOp::Increment {
            out,
            out1,
            min,
            max,
        } => {
            stat!(rows_closed += 1);
            stat!(words_incremented += count);
            // The exit first, on the counts as they are: some count has
            // reached `min` iff the greatest has.
            if exits(&vals[src..src + count], min) {
                stat!(fanout += 1);
                len[out1 as usize] = 1;
                on(out1);
            }
            // Then the continuation, which leaves this row as it is: only
            // its greatest count can have reached `max`.
            let (d, off) = (out.node as usize, out.off as usize);
            let held = len[d] as usize;
            let n = if held == 0 {
                let kept = count - usize::from(vals[src + count - 1] == max);
                for i in 0..kept {
                    vals[off + i] = vals[src + i] + 1;
                }
                kept
            } else {
                let n = union(
                    &vals[off..off + held],
                    &vals[src..src + count],
                    1,
                    max,
                    node.k,
                    merge,
                );
                if n > node.cap as usize {
                    over_capacity(n, node.cap);
                }
                copy_values(merge, 0, vals, off, n);
                n
            };
            if n > 0 {
                stat!(fanout += 1);
                len[d] = n as u16;
                on(out.node);
            }
        }
        SparseOp::Assert { kind, out } => {
            stat!(rows_closed += 1);
            if ctx.eval(kind) == AssertEval::Pass {
                pass_within(node, count, len, vals, merge, out);
                on(out.node);
            }
        }
        SparseOp::Match => return true,
        SparseOp::Edge { .. } | SparseOp::Table(_) => {}
    }
    false
}

/// Copy the active raw rows into the closure frontier (whose lengths are
/// all zero) and activate the start (search reseeding).
#[inline(always)]
fn seed(
    plan: &SparsePlan,
    raw_len: &[u16],
    raw_vals: &[u32],
    raw_active: &[u64],
    len: &mut [u16],
    vals: &mut [u32],
) {
    for_each_active(raw_active, |n| {
        let node = &plan.nodes[n as usize];
        let count = raw_len[n as usize] as usize;
        stat!(words_copied += count.max(1));
        len[n as usize] = count as u16;
        if node.cap > 0 {
            let off = node.off as usize;
            copy_values(raw_vals, off, vals, off, count);
        }
    });
    debug_assert_eq!(
        plan.nodes[plan.start as usize].cap, 0,
        "the start is Outside"
    );
    len[plan.start as usize] = 1;
}

/// Close the raw frontier into the closure frontier by a full sweep:
/// apply the zero-width transfer of every nonempty row in node order,
/// which is a topological order of the zero-width graph, so that a row is
/// read after all its zero-width predecessors have written to it.  Returns
/// whether `Match` was reached.  The raw frontier is only read; the
/// closure bitmap is left alone.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn close_full(
    plan: &SparsePlan,
    raw_len: &[u16],
    raw_vals: &[u32],
    raw_active: &[u64],
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    ctx: BoundaryCtx,
) -> bool {
    seed(plan, raw_len, raw_vals, raw_active, len, vals);
    let mut matched = false;
    for node in plan.zero_width.iter() {
        if len[node.node as usize] != 0 {
            matched |= transfer(node, len, vals, merge, ctx, |_| {});
        }
    }
    matched
}

/// [`close_full`] over the active rows only, keeping the closure bitmap
/// `active` (all zero on entry).
#[allow(clippy::too_many_arguments)]
fn close_active(
    plan: &SparsePlan,
    raw_len: &[u16],
    raw_vals: &[u32],
    raw_active: &[u64],
    len: &mut [u16],
    vals: &mut [u32],
    active: &mut [u64],
    merge: &mut [u32],
    ctx: BoundaryCtx,
) -> bool {
    seed(plan, raw_len, raw_vals, raw_active, len, vals);
    active.copy_from_slice(raw_active);
    activate(active, plan.start);
    let mut matched = false;
    for w in 0..active.len() {
        let mut done = 0u64;
        loop {
            let pending = active[w] & !done;
            if pending == 0 {
                break;
            }
            let bit = pending.isolate_lowest_one();
            done |= bit;
            let node = &plan.nodes[w * 64 + bit.trailing_zeros() as usize];
            matched |= transfer(node, len, vals, merge, ctx, |out| activate(active, out));
        }
    }
    matched
}

/// The raw frontier being written by a consumption pass.
struct Next<'a> {
    len: &'a mut [u16],
    vals: &'a mut [u32],
    active: &'a mut [u64],
    /// Rows written so far.
    live: u32,
}

/// Move the whole closed row of `node` (`count` values) to the successor
/// `out` of the raw frontier being written.
#[inline(always)]
fn pass_across(
    node: &SparseNode,
    count: usize,
    vals: &[u32],
    next: &mut Next<'_>,
    merge: &mut [u32],
    out: Succ,
) {
    let fresh = if node.cap == 0 {
        stat!(fanout += 1);
        let fresh = next.len[out.node as usize] == 0;
        next.len[out.node as usize] = 1;
        fresh
    } else {
        union_row(
            vals,
            node.off as usize,
            count,
            next.len,
            next.vals,
            merge,
            out,
            node.k,
            node.cap,
        )
    };
    if fresh {
        next.live += 1;
        activate(next.active, out.node);
    }
}

/// Consume `b` at the consuming `node`, whose closed row is nonempty: test
/// its predicate once and move the whole row to its successors.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn consume_node(
    plan: &SparsePlan,
    regex: &Regex,
    node: &SparseNode,
    len: &[u16],
    vals: &[u32],
    next: &mut Next<'_>,
    merge: &mut [u32],
    b: u8,
) {
    let count = len[node.node as usize] as usize;
    match node.op {
        SparseOp::Edge { out, out_exit } => {
            stat!(tests += 1);
            if accepts(&regex.classes, &regex.states[node.state], b) {
                pass_across(node, count, vals, next, merge, out);
                if out_exit != Succ::NONE {
                    pass_across(node, count, vals, next, merge, out_exit);
                }
            }
        }
        SparseOp::Table(table) => {
            stat!(tests += 1);
            let target = regex.byte_tables[table][b];
            if target != StateIdx::NONE {
                let out = plan.node_of[target.idx()];
                let out = Succ {
                    node: out,
                    off: plan.nodes[out as usize].off,
                };
                pass_across(node, count, vals, next, merge, out);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Worker scratch
// ---------------------------------------------------------------------------

/// Per-worker scratch of the sparse variant: the closure frontier and the
/// merge workspace.  Borrowed only during a call, so streams of different
/// regexes can share one, one call at a time.  Capacity grows to the
/// largest plan seen and is kept; the lengths and the bitmap are all zero
/// between calls.
#[derive(Debug, Default)]
pub struct SparseScratch {
    len: Vec<u16>,
    vals: Vec<u32>,
    active: Vec<u64>,
    /// Room for a merge before it is stored: twice the largest row.
    merge: Vec<u32>,
    sweep: Sweep,
    /// Bytes stepped by the full sweep and by the active rows, and bytes
    /// skipped while idle (for tests).
    #[cfg(test)]
    pub(crate) passes: [usize; 3],
}

impl SparseScratch {
    /// Empty scratch; it grows on first use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Make room for `plan`, before any scanning.
    fn prepare(&mut self, plan: &SparsePlan) {
        fn grow<T: Clone + Default>(v: &mut Vec<T>, len: usize) {
            if v.len() < len {
                v.reserve_exact(len - v.len());
                v.resize(len, T::default());
            }
        }
        grow(&mut self.len, plan.rows());
        grow(&mut self.vals, plan.slots);
        grow(&mut self.active, plan.active_words);
        grow(&mut self.merge, plan.merge_slots());
    }

    /// Bytes held (the high-water capacity).
    pub fn memory_bytes(&self) -> usize {
        self.len.capacity() * size_of::<u16>()
            + (self.vals.capacity() + self.merge.capacity()) * size_of::<u32>()
            + self.active.capacity() * size_of::<u64>()
    }

    /// Force a pass, for tests and measurements; [`Sweep::Auto`] is the
    /// default.  Forcing a pass also turns off the skipping of an idle
    /// stream, so that the general step can be tested and measured against
    /// it.
    #[doc(hidden)]
    pub fn set_sweep(&mut self, sweep: Sweep) {
        self.sweep = sweep;
    }

    /// Whether the lengths and the bitmap are all zero (for tests).
    #[cfg(test)]
    pub(crate) fn is_clean(&self) -> bool {
        self.len.iter().all(|&l| l == 0) && self.active.iter().all(|&w| w == 0)
    }
}

impl SparsePlan {
    /// Values of the merge workspace: a merge holds at most one value more
    /// than the largest row; twice the row is reserved.
    fn merge_slots(&self) -> usize {
        2 * self.max_cap
    }

    /// Bytes of one stream of this plan ([`SparseStream::memory_bytes`]).
    pub(crate) fn stream_bytes(&self) -> usize {
        size_of::<SparseStream>()
            + self.rows() * size_of::<u16>()
            + self.slots * size_of::<u32>()
            + self.active_words * size_of::<u64>()
    }

    /// Bytes of a scratch prepared for this plan alone
    /// ([`SparseScratch::memory_bytes`]).
    pub(crate) fn scratch_bytes(&self) -> usize {
        self.rows() * size_of::<u16>()
            + (self.slots + self.merge_slots()) * size_of::<u32>()
            + self.active_words * size_of::<u64>()
    }
}

// ---------------------------------------------------------------------------
// Stream
// ---------------------------------------------------------------------------

/// One input stream on the sparse counting variant of one regex.
///
/// Holds the *raw* frontier (right after a byte was consumed, before any
/// zero-width step), the previous byte, the at-start flag and the sticky
/// match flag.  Every call closes a copy of the raw frontier in the
/// scratch, with what is known of the boundary: the next byte before
/// consuming it, nothing at the end of a chunk, end of input at `finish`.
/// A closure is never stored back.
///
/// While no row is active, bytes that cannot start anything are skipped
/// when the plan allows it (its start closure reaches neither an assertion
/// nor a match).
///
/// Lifecycle: [`new`](Self::new), any number of [`chunk`](Self::chunk)
/// calls, [`finish`](Self::finish); [`reset`](Self::reset) starts over.
/// Feeding or finishing a finished stream panics, as does passing a
/// different regex.
#[derive(Debug)]
pub struct SparseStream {
    regex_id: u64,
    /// One length per node of the plan.
    len: Vec<u16>,
    /// The plan's slots.
    vals: Vec<u32>,
    /// The nonempty rows, one bit per node.
    active: Vec<u64>,
    /// Their number: none means idle.
    live: u32,
    prev: Option<u8>,
    at_start: bool,
    matched: bool,
    finished: bool,
}

impl SparseStream {
    /// A fresh stream for `regex`, which must have been compiled with
    /// [`RegexConfig::sparse_counting`](crate::RegexConfig::sparse_counting)
    /// and admitted.
    pub fn new(regex: &Regex, scratch: &mut SparseScratch) -> Result<Self, SparseCountingError> {
        let plan = regex.sparse_plan()?;
        let mut stream = Self {
            regex_id: regex.id,
            len: vec![0; plan.rows()],
            vals: vec![0; plan.slots],
            active: vec![0; plan.active_words],
            live: 0,
            prev: None,
            at_start: true,
            matched: false,
            finished: false,
        };
        scratch.prepare(plan);
        stream.provisional(plan, scratch, Boundary::UnknownNext);
        Ok(stream)
    }

    /// Return to the state of a fresh stream, keeping the allocation.
    pub fn reset(&mut self, regex: &Regex, scratch: &mut SparseScratch) {
        let plan = self.plan(regex);
        self.len.fill(0);
        self.active.fill(0);
        self.live = 0;
        self.prev = None;
        self.at_start = true;
        self.matched = false;
        self.finished = false;
        scratch.prepare(plan);
        self.provisional(plan, scratch, Boundary::UnknownNext);
    }

    /// Make this a fresh stream for `regex`, which may be another regex
    /// than the one it was made for, reusing the allocation when its
    /// capacity suffices.  On an error the stream is left as it was.
    pub fn rebind(
        &mut self,
        regex: &Regex,
        scratch: &mut SparseScratch,
    ) -> Result<(), SparseCountingError> {
        let plan = regex.sparse_plan()?;
        self.regex_id = regex.id;
        // Exactly the plan's size when an allocation has to grow.
        fn fit<T: Clone + Default>(v: &mut Vec<T>, len: usize) {
            v.clear();
            v.reserve_exact(len);
            v.resize(len, T::default());
        }
        fit(&mut self.len, plan.rows());
        fit(&mut self.vals, plan.slots);
        fit(&mut self.active, plan.active_words);
        self.reset(regex, scratch);
        Ok(())
    }

    /// Feed the next bytes of the input.
    pub fn chunk(&mut self, regex: &Regex, scratch: &mut SparseScratch, input: &[u8]) {
        let plan = self.plan(regex);
        assert!(!self.finished, "chunk on a finished stream: reset it first");
        if self.matched || input.is_empty() {
            return;
        }
        scratch.prepare(plan);
        // Skipping runs under the automatic choice only: forcing a pass
        // runs the general step on every byte, which the tests compare
        // with it.
        let starts = plan
            .idle
            .as_deref()
            .filter(|_| scratch.sweep == Sweep::Auto);
        let prefilter = &regex.prefilter;
        let has_prefilter = !matches!(prefilter, crate::Prefilter::None);
        let mut i = 0;
        while i < input.len() {
            if self.live == 0
                && let Some(starts) = starts
            {
                // The closure of an empty frontier is the start closure,
                // the same at every boundary, without a match: find the
                // next byte that one of its consuming states accepts.  The
                // others leave the frontier empty.
                let from = i;
                let found = loop {
                    let rest = &input[i..];
                    let off = if has_prefilter {
                        prefilter.find(rest)
                    } else {
                        rest.iter().position(|&b| starts[b as usize])
                    };
                    match off {
                        Some(off) if starts[rest[off] as usize] => break Some(i + off),
                        Some(off) => i += off + 1,
                        None => break None,
                    }
                    if i >= input.len() {
                        break None;
                    }
                };
                let next = found.unwrap_or(input.len());
                stat!(bytes_skipped += next - from);
                #[cfg(test)]
                {
                    scratch.passes[2] += next - from;
                }
                if next > from {
                    self.prev = Some(input[next - 1]);
                    self.at_start = false;
                }
                if found.is_none() {
                    return;
                }
                i = next;
            }
            if self.step(plan, regex, scratch, input[i]) {
                return;
            }
            i += 1;
        }
        self.provisional(plan, scratch, Boundary::UnknownNext);
    }

    /// Signal end of input and return the final result.
    pub fn finish(&mut self, regex: &Regex, scratch: &mut SparseScratch) -> bool {
        let plan = self.plan(regex);
        assert!(
            !self.finished,
            "finish on a finished stream: reset it first"
        );
        self.finished = true;
        if !self.matched {
            scratch.prepare(plan);
            self.provisional(plan, scratch, Boundary::EndOfInput);
        }
        self.matched
    }

    /// Whether a match has been found so far.
    pub fn ismatch(&self) -> bool {
        self.matched
    }

    /// Bytes owned by this stream: [`SparsePlan::stream_bytes`] for a new
    /// stream, more after [`rebind`](Self::rebind) to a smaller plan.
    pub fn memory_bytes(&self) -> usize {
        size_of::<Self>()
            + self.len.capacity() * size_of::<u16>()
            + self.vals.capacity() * size_of::<u32>()
            + self.active.capacity() * size_of::<u64>()
    }

    /// A displayable view of the raw frontier.
    pub fn display<'a>(&'a self, regex: &'a Regex) -> impl fmt::Display + 'a {
        DisplayStream {
            stream: self,
            plan: self.plan(regex),
        }
    }

    /// The plan, checking that `regex` is the one this stream was made for.
    fn plan<'r>(&self, regex: &'r Regex) -> &'r SparsePlan {
        assert_eq!(
            regex.id, self.regex_id,
            "sparse stream used with a different regex"
        );
        regex
            .sparse_plan()
            .expect("the regex of a stream has a plan")
    }

    fn ctx(&self, boundary: Boundary) -> BoundaryCtx {
        BoundaryCtx {
            at_start: self.at_start,
            prev: self.prev,
            boundary,
        }
    }

    /// Whether this byte runs the full sweep.
    #[inline]
    fn full_sweep(&self, plan: &SparsePlan, sweep: Sweep) -> bool {
        match sweep {
            Sweep::Full => true,
            Sweep::Active => false,
            Sweep::Auto => plan.rows() <= SMALL_PLAN_ROWS || self.live as usize * 8 >= plan.rows(),
        }
    }

    /// A closure that only looks for a match: the raw frontier stays as
    /// it is, and the scratch is cleaned.
    fn provisional(&mut self, plan: &SparsePlan, scratch: &mut SparseScratch, boundary: Boundary) {
        stat!(provisional += 1);
        let ctx = self.ctx(boundary);
        let full = self.full_sweep(plan, scratch.sweep);
        let (len, vals, active) = (
            &mut scratch.len[..plan.rows()],
            &mut scratch.vals[..plan.slots],
            &mut scratch.active[..plan.active_words],
        );
        let (raw_len, raw_vals, raw_active) = (&self.len, &self.vals, &self.active);
        if full {
            self.matched |= close_full(
                plan,
                raw_len,
                raw_vals,
                raw_active,
                len,
                vals,
                &mut scratch.merge,
                ctx,
            );
            len.fill(0);
        } else {
            self.matched |= close_active(
                plan,
                raw_len,
                raw_vals,
                raw_active,
                len,
                vals,
                active,
                &mut scratch.merge,
                ctx,
            );
            for_each_active(active, |n| len[n as usize] = 0);
            active.fill(0);
        }
    }

    /// Close before `b`, then consume it: the raw frontier is replaced by
    /// the consuming successors (the closure holds its old contents).
    /// Returns whether a match was found first.
    fn step(
        &mut self,
        plan: &SparsePlan,
        regex: &Regex,
        scratch: &mut SparseScratch,
        b: u8,
    ) -> bool {
        let ctx = self.ctx(Boundary::BeforeByte(b));
        let full = self.full_sweep(plan, scratch.sweep);
        #[cfg(test)]
        {
            scratch.passes[usize::from(!full)] += 1;
        }
        let (len, vals, active) = (
            &mut scratch.len[..plan.rows()],
            &mut scratch.vals[..plan.slots],
            &mut scratch.active[..plan.active_words],
        );
        let merge = &mut scratch.merge[..];
        let (raw_len, raw_vals, raw_active) = (&mut *self.len, &mut *self.vals, &mut *self.active);
        if full {
            stat!(steps_full += 1);
            if close_full(plan, raw_len, raw_vals, raw_active, len, vals, merge, ctx) {
                len.fill(0);
                self.matched = true;
                return true;
            }
            stat!(words_cleared += raw_len.len() + len.len());
            raw_len.fill(0);
            raw_active.fill(0);
            let mut next = Next {
                len: raw_len,
                vals: raw_vals,
                active: raw_active,
                live: 0,
            };
            for node in plan.consumers.iter() {
                if len[node.node as usize] != 0 {
                    consume_node(plan, regex, node, len, vals, &mut next, merge, b);
                }
            }
            self.live = next.live;
            len.fill(0);
        } else {
            stat!(steps_active += 1);
            if close_active(
                plan, raw_len, raw_vals, raw_active, len, vals, active, merge, ctx,
            ) {
                for_each_active(active, |n| len[n as usize] = 0);
                active.fill(0);
                self.matched = true;
                return true;
            }
            for_each_active(raw_active, |n| raw_len[n as usize] = 0);
            raw_active.fill(0);
            let mut next = Next {
                len: raw_len,
                vals: raw_vals,
                active: raw_active,
                live: 0,
            };
            // Each closed row is read once, then cleaned.
            for_each_active(active, |n| {
                let node = &plan.nodes[n as usize];
                consume_node(plan, regex, node, len, vals, &mut next, merge, b);
                stat!(words_cleared += 1);
                len[n as usize] = 0;
            });
            self.live = next.live;
            active.fill(0);
        }
        #[cfg(feature = "op-stats")]
        count_frontier(plan, &self.len);
        self.prev = Some(b);
        self.at_start = false;
        false
    }

    /// The representatives of the raw row of `state` (for tests); `None`
    /// for an unreachable state or an Outside row, which holds no counts.
    #[cfg(test)]
    pub(crate) fn raw_counts(&self, regex: &Regex, state: StateIdx) -> Option<&[u32]> {
        let plan = self.plan(regex);
        let n = plan.node_of[state.idx()];
        (n != NO_NODE && plan.nodes[n as usize].cap > 0).then(|| {
            let off = plan.nodes[n as usize].off as usize;
            &self.vals[off..off + self.len[n as usize] as usize]
        })
    }

    /// The boundary metadata: previous byte and at-start flag (for tests).
    #[cfg(test)]
    pub(crate) fn boundary_state(&self) -> (Option<u8>, bool) {
        (self.prev, self.at_start)
    }

    /// Whether the raw row of `state` is active (for tests).
    #[cfg(test)]
    pub(crate) fn raw_active(&self, regex: &Regex, state: StateIdx) -> bool {
        let n = self.plan(regex).node_of[state.idx()];
        n != NO_NODE && self.len[n as usize] != 0
    }

    /// Check the invariants of the raw frontier (for tests): the bitmap
    /// and the count of active rows are those of the lengths, and every
    /// row is a reduced list within its capacity.
    #[cfg(test)]
    pub(crate) fn check_invariants(&self, regex: &Regex, label: &str) {
        let plan = self.plan(regex);
        let mut live = 0;
        for (n, node) in plan.nodes.iter().enumerate() {
            let count = self.len[n] as usize;
            let bit = self.active[n / 64] >> (n % 64) & 1 != 0;
            assert_eq!(bit, count != 0, "{label}: bitmap of node {n}");
            live += u32::from(count != 0);
            if node.cap == 0 {
                assert!(count <= 1, "{label}: Outside row of node {n}");
                continue;
            }
            assert!(count <= node.cap as usize, "{label}: capacity of node {n}");
            let row = &self.vals[node.off as usize..][..count];
            assert!(
                row.windows(2).all(|w| w[0] < w[1])
                    && row.windows(3).all(|w| w[2] - w[0] > node.k)
                    && row.first().is_none_or(|&u| u >= 1),
                "{label}: node {n} holds {row:?}, not a reduced list"
            );
        }
        assert_eq!(live, self.live, "{label}: active rows");
    }
}

/// Add the active rows of a raw frontier, and the representatives of its
/// counted rows, to the operation counters.
#[cfg(feature = "op-stats")]
fn count_frontier(plan: &SparsePlan, len: &[u16]) {
    for (node, &count) in plan.nodes.iter().zip(len) {
        if count == 0 {
            continue;
        }
        stat!(live_rows += 1);
        if node.cap > 0 {
            stat!(counted_rows += 1);
            stat!(counted_words += node.cap);
            stat!(count_values += count);
            stat!(max_row_values max= count);
        }
    }
}

/// A sparse stream with the regex and the scratch it runs with: what
/// [`AnyMatcher`](crate::AnyMatcher) holds for the sparse variant.  Made by
/// [`MatcherMemory::matcher_sparse`](crate::MatcherMemory::matcher_sparse),
/// which keeps the stream storage and scratch.
pub struct SparseMatcher<'a> {
    pub(crate) regex: &'a Regex,
    pub(crate) stream: &'a mut SparseStream,
    pub(crate) scratch: &'a mut SparseScratch,
}

impl SparseMatcher<'_> {
    /// Feed the next bytes of the input.
    #[inline]
    pub fn chunk(&mut self, input: &[u8]) {
        self.stream.chunk(self.regex, self.scratch, input);
    }

    /// Signal end of input and return the final result.
    pub fn finish(self) -> bool {
        self.stream.finish(self.regex, self.scratch)
    }

    /// Whether a match has been found so far.
    pub fn ismatch(&self) -> bool {
        self.stream.ismatch()
    }
}

impl fmt::Display for SparseMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stream.display(self.regex))
    }
}

impl fmt::Debug for SparseMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SparseMatcher({})", self.stream.display(self.regex))
    }
}

struct DisplayStream<'a> {
    stream: &'a SparseStream,
    plan: &'a SparsePlan,
}

/// The raw rows: `state` for an active Outside row, and
/// `state:cN u~{..}` for a counted one, where the listed values are
/// representatives of the active iteration numbers `u` (one more than the
/// completed iterations), not all of them.
impl fmt::Display for DisplayStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.stream;
        write!(f, "sparse slots={} matched={}", self.plan.slots, s.matched)?;
        if s.at_start {
            write!(f, " at_start")?;
        }
        if let Some(b) = s.prev {
            write!(f, " prev={b:#04x}")?;
        }
        if s.finished {
            write!(f, " finished")?;
        }
        let mut parts = Vec::new();
        for (n, node) in self.plan.nodes.iter().enumerate() {
            if s.len[n] == 0 {
                continue;
            }
            parts.push(match node.scope {
                Scope::Outside => node.state.to_string(),
                Scope::Counter(c) => {
                    let counts: Vec<String> = s.vals[node.off as usize..][..s.len[n] as usize]
                        .iter()
                        .map(u32::to_string)
                        .collect();
                    format!("{}:c{c} u~{{{}}}", node.state, counts.join(","))
                }
            });
        }
        write!(f, " raw={{{}}}", parts.join(", "))
    }
}

impl Regex {
    /// Experimental: feed `input` whole to a fresh sparse counting stream
    /// and finish it; `None` if this regex cannot run on it.  For the
    /// differential harnesses.
    #[doc(hidden)]
    pub fn sparse_is_match(&self, scratch: &mut SparseScratch, input: &[u8]) -> Option<bool> {
        let mut stream = SparseStream::new(self, scratch).ok()?;
        stream.chunk(self, scratch, input);
        Some(stream.finish(self, scratch))
    }
}

#[cfg(test)]
#[path = "sparse_tests.rs"]
pub(crate) mod tests;

#[cfg(test)]
#[path = "sparse_assertion_tests.rs"]
mod assertion_tests;
