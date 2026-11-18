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

use std::fmt;

use super::analysis::{Rejection, Scope, Structure, analysis_bytes};
use super::exact::{Boundary, BoundaryCtx, accepts};
use super::stats::stat;
use super::{NO_NODE, NodeIdx, SparseCountingError};
use crate::{AssertEval, AssertKind, ByteTableIdx, Regex, State, StateIdx};

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
    /// An assertion is reachable; assertions are not supported yet.
    Assertion { state: u32 },
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
            Self::Assertion { state } => write!(
                f,
                "assertion at state {state}: the sparse variant does not support assertions yet"
            ),
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

/// What a node does with its row.  Successors are nodes; [`NO_NODE`] is an
/// absent `out_exit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SparseOp {
    /// Union into both successors.
    Split { out: NodeIdx, out1: NodeIdx },
    /// Outside active: insert count one at `out`.
    Instance { out: NodeIdx },
    /// Outside at `out1` iff some count is at least `min`; the counts
    /// below `max`, plus one, continue to `out`.
    Increment {
        out: NodeIdx,
        out1: NodeIdx,
        min: u32,
        max: u32,
    },
    /// Union into `out` iff the shared evaluator says `Pass` at the
    /// current boundary.  Not admitted yet.
    Assert { kind: AssertKind, out: NodeIdx },
    /// Active: a match.
    Match,
    /// Consumes with the one-byte predicate of its state.
    Edge { out: NodeIdx, out_exit: NodeIdx },
    /// Consumes through a byte dispatch table.
    Table(ByteTableIdx),
}

/// A reachable state and its row.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SparseNode {
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
    /// Node of the start state, reseeded at every boundary.
    pub(crate) start: NodeIdx,
    /// Representative slots of one frontier.
    pub(crate) slots: usize,
    /// The largest row capacity (0 without a counter).
    pub(crate) max_cap: usize,
}

impl SparsePlan {
    /// Heap bytes of a plan with these array lengths.
    fn heap_bytes_for(states: usize, nodes: usize) -> usize {
        states * size_of::<NodeIdx>() + nodes * size_of::<SparseNode>()
    }

    /// Heap bytes of this plan's arrays.
    pub(crate) fn heap_bytes(&self) -> usize {
        Self::heap_bytes_for(self.node_of.len(), self.nodes.len())
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
            "  rows: {} ({} counted, {} count slots), start state {}",
            self.rows(),
            self.counted_rows(),
            self.slots,
            self.nodes[self.start as usize].state
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
                SparseOp::Split { out, out1 } => writeln!(f, "split -> {}, {}", st(out), st(out1))?,
                SparseOp::Instance { out } => writeln!(f, "CI: u = 1 -> {}", st(out))?,
                SparseOp::Increment {
                    out,
                    out1,
                    min,
                    max,
                } => writeln!(
                    f,
                    "CInc: exit if u >= {min} -> {}, then u + 1 if u < {max} -> {}",
                    st(out1),
                    st(out)
                )?,
                SparseOp::Assert { kind, out } => writeln!(f, "{} -> {}", kind.label(), st(out))?,
                SparseOp::Match => writeln!(f, "match")?,
                SparseOp::Edge { out, out_exit } => {
                    write!(f, "consume -> {}", st(out))?;
                    if out_exit != NO_NODE {
                        write!(f, ", {}", st(out_exit))?;
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
/// [`Structure`] it reads: the spacing and capacity per counter on top of
/// the structural analysis.
pub(crate) fn sparse_analysis_bytes(states: usize, num_counters: usize) -> usize {
    analysis_bytes(states, num_counters) + num_counters * size_of::<Option<(u32, u32)>>()
}

/// Lay out the sparse rows of a proven [`Structure`] and build the plan,
/// or say why not.  `structure` is the outcome of
/// [`analysis::structure`](super::analysis::structure) on `states`.
pub(crate) fn analyze(
    states: &[State],
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
    if let Some(&idx) = order
        .iter()
        .find(|&&s| matches!(states[s], State::Assert { .. }))
    {
        return Err(SparseRejection::Assertion { state: idx.0 });
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
    let plan_bytes = SparsePlan::heap_bytes_for(states.len(), order.len());
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
    let node = |s: StateIdx| -> NodeIdx {
        let n = node_of[s.idx()];
        debug_assert!(n != NO_NODE, "successor of a reachable state is reachable");
        n
    };
    let mut nodes = Vec::with_capacity(order.len());
    let mut off = 0u32;
    let mut max_cap = 0usize;
    for &idx in order {
        let scope = label[idx.idx()].expect("reachable");
        let (k, cap) = shape_of(scope)?;
        let op = match states[idx] {
            State::Split { out, out1 } => SparseOp::Split {
                out: node(out),
                out1: node(out1),
            },
            State::CounterInstance { out, .. } => SparseOp::Instance { out: node(out) },
            State::CounterIncrement {
                out,
                out1,
                min,
                max,
                ..
            } => SparseOp::Increment {
                out: node(out),
                out1: node(out1),
                // `representatives` accepted these bounds: they fit.
                min: min as u32,
                max: max as u32,
            },
            State::Assert { kind, out } => SparseOp::Assert {
                kind,
                out: node(out),
            },
            State::Match => SparseOp::Match,
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => SparseOp::Edge {
                out: node(out),
                out_exit: if out_exit == StateIdx::NONE {
                    NO_NODE
                } else {
                    node(out_exit)
                },
            },
            State::ByteTable { table } => SparseOp::Table(table),
        };
        nodes.push(SparseNode {
            state: idx,
            scope,
            off,
            cap,
            k,
            op,
        });
        off += cap;
        max_cap = max_cap.max(cap as usize);
    }
    debug_assert_eq!(off as usize, slots);
    let plan = SparsePlan {
        start: node(start),
        nodes: nodes.into_boxed_slice(),
        node_of: node_of.into_boxed_slice(),
        slots,
        max_cap,
    };
    debug_assert_eq!(plan.heap_bytes(), plan_bytes);
    Ok(plan)
}

// ---------------------------------------------------------------------------
// Frontiers
// ---------------------------------------------------------------------------
//
// A frontier is a length per node and the representative arena.  The
// length of an Outside row is 1 when it is active and 0 otherwise; that of
// a counted row is its number of representatives, 0 when it is empty.
// Slots past the length hold stale values and are never read.

/// The representatives of `node` in a frontier.
#[inline]
fn row<'a>(node: &SparseNode, len: &[u16], vals: &'a [u32], n: NodeIdx) -> &'a [u32] {
    &vals[node.off as usize..][..len[n as usize] as usize]
}

/// Store the `count` values of `merge` as the row of `node`.  A reduced
/// list fits the capacity (see the module documentation): anything else is
/// a bug in the transfers, never a reason to truncate.
#[inline]
fn store(node: &SparseNode, len: &mut [u16], vals: &mut [u32], n: NodeIdx, merge: &[u32]) {
    assert!(
        merge.len() <= node.cap as usize,
        "a reduced row of {} values exceeds its capacity of {}",
        merge.len(),
        node.cap
    );
    vals[node.off as usize..][..merge.len()].copy_from_slice(merge);
    len[n as usize] = merge.len() as u16;
}

/// Union the row of `src` into that of `dst`, in one frontier.  Both are
/// in the same scope (the scope proof), so both are Outside or both hold
/// counts of the same counter.
#[inline]
fn union_within(
    plan: &SparsePlan,
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    src: NodeIdx,
    dst: NodeIdx,
) {
    let (s, d) = (&plan.nodes[src as usize], &plan.nodes[dst as usize]);
    debug_assert_eq!(s.cap, d.cap, "union of rows of different scopes");
    debug_assert_ne!(src, dst, "union of a row into itself");
    stat!(fanout += 1);
    if s.cap == 0 {
        len[dst as usize] = 1;
        return;
    }
    let count = len[src as usize] as usize;
    stat!(words_unioned += count);
    if len[dst as usize] == 0 {
        // The source is reduced already.
        vals.copy_within(s.off as usize..s.off as usize + count, d.off as usize);
        len[dst as usize] = count as u16;
        return;
    }
    let n = union(
        row(d, len, vals, dst),
        row(s, len, vals, src),
        0,
        u32::MAX,
        d.k,
        merge,
    );
    store(d, len, vals, dst, &merge[..n]);
}

/// Union the row of `src` in the frontier `from` into that of `dst` in
/// the frontier `to`.
#[inline]
#[allow(clippy::too_many_arguments)]
fn union_across(
    plan: &SparsePlan,
    from_len: &[u16],
    from_vals: &[u32],
    to_len: &mut [u16],
    to_vals: &mut [u32],
    merge: &mut [u32],
    src: NodeIdx,
    dst: NodeIdx,
) {
    let (s, d) = (&plan.nodes[src as usize], &plan.nodes[dst as usize]);
    debug_assert_eq!(s.cap, d.cap, "union of rows of different scopes");
    stat!(fanout += 1);
    if s.cap == 0 {
        to_len[dst as usize] = 1;
        return;
    }
    let source = row(s, from_len, from_vals, src);
    stat!(words_unioned += source.len());
    if to_len[dst as usize] == 0 {
        to_vals[d.off as usize..][..source.len()].copy_from_slice(source);
        to_len[dst as usize] = source.len() as u16;
        return;
    }
    let n = union(
        row(d, to_len, to_vals, dst),
        source,
        0,
        u32::MAX,
        d.k,
        merge,
    );
    store(d, to_len, to_vals, dst, &merge[..n]);
}

/// Close the raw frontier into the closure frontier: copy it, activate
/// the start (search reseeding), then apply the zero-width transfer of
/// every nonempty row in node order, which is a topological order of the
/// zero-width graph: a row is read after all its zero-width predecessors
/// have written to it.  Returns whether `Match` was reached.
///
/// The raw frontier is only read.  A source row is not changed by its own
/// transfer, so each of its successors reads the same counts.
fn close(
    plan: &SparsePlan,
    raw_len: &[u16],
    raw_vals: &[u32],
    len: &mut [u16],
    vals: &mut [u32],
    merge: &mut [u32],
    ctx: BoundaryCtx,
) -> bool {
    stat!(words_copied += raw_len.len() + raw_vals.len());
    len.copy_from_slice(raw_len);
    vals.copy_from_slice(raw_vals);
    debug_assert_eq!(
        plan.nodes[plan.start as usize].cap, 0,
        "the start is Outside"
    );
    len[plan.start as usize] = 1;

    let mut matched = false;
    for (n, node) in plan.nodes.iter().enumerate() {
        let n = n as NodeIdx;
        if len[n as usize] == 0 {
            continue;
        }
        match node.op {
            SparseOp::Split { out, out1 } => {
                stat!(rows_closed += 1);
                union_within(plan, len, vals, merge, n, out);
                union_within(plan, len, vals, merge, n, out1);
            }
            SparseOp::Instance { out } => {
                stat!(rows_closed += 1);
                stat!(fanout += 1);
                let d = &plan.nodes[out as usize];
                let count = union(row(d, len, vals, out), &[1], 0, u32::MAX, d.k, merge);
                store(d, len, vals, out, &merge[..count]);
            }
            SparseOp::Increment {
                out,
                out1,
                min,
                max,
            } => {
                stat!(rows_closed += 1);
                stat!(words_incremented += len[n as usize]);
                // The exit first, on the counts as they are; then the
                // continuation, which leaves this row as it is.
                if exits(row(node, len, vals, n), min) {
                    stat!(fanout += 1);
                    len[out1 as usize] = 1;
                }
                let d = &plan.nodes[out as usize];
                let count = union(
                    row(d, len, vals, out),
                    row(node, len, vals, n),
                    1,
                    max,
                    d.k,
                    merge,
                );
                if count > 0 {
                    stat!(fanout += 1);
                    store(d, len, vals, out, &merge[..count]);
                }
            }
            SparseOp::Assert { kind, out } => {
                stat!(rows_closed += 1);
                if ctx.eval(kind) == AssertEval::Pass {
                    union_within(plan, len, vals, merge, n, out);
                }
            }
            SparseOp::Match => matched = true,
            SparseOp::Edge { .. } | SparseOp::Table(_) => {}
        }
    }
    matched
}

/// Consume `b` from the closed frontier into the raw one, which it
/// replaces: every nonempty consuming row tests its predicate once and
/// moves whole.
#[allow(clippy::too_many_arguments)]
fn consume(
    plan: &SparsePlan,
    regex: &Regex,
    len: &[u16],
    vals: &[u32],
    raw_len: &mut [u16],
    raw_vals: &mut [u32],
    merge: &mut [u32],
    b: u8,
) {
    stat!(words_cleared += raw_len.len());
    raw_len.fill(0);
    for (n, node) in plan.nodes.iter().enumerate() {
        let n = n as NodeIdx;
        if len[n as usize] == 0 {
            continue;
        }
        match node.op {
            SparseOp::Edge { out, out_exit } => {
                stat!(tests += 1);
                if accepts(&regex.classes, &regex.states[node.state], b) {
                    union_across(plan, len, vals, raw_len, raw_vals, merge, n, out);
                    if out_exit != NO_NODE {
                        union_across(plan, len, vals, raw_len, raw_vals, merge, n, out_exit);
                    }
                }
            }
            SparseOp::Table(table) => {
                stat!(tests += 1);
                let target = regex.byte_tables[table][b];
                if target != StateIdx::NONE {
                    let out = plan.node_of[target.idx()];
                    union_across(plan, len, vals, raw_len, raw_vals, merge, n, out);
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Worker scratch
// ---------------------------------------------------------------------------

/// Per-worker scratch of the sparse variant: the closure frontier and the
/// merge workspace.  Borrowed only during a call, so streams of different
/// regexes can share one, one call at a time.  Capacity grows to the
/// largest plan seen and is kept; nothing in it outlives a call.
#[derive(Debug, Default)]
pub struct SparseScratch {
    len: Vec<u16>,
    vals: Vec<u32>,
    /// Room for a merge before it is stored: twice the largest row.
    merge: Vec<u32>,
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
        grow(&mut self.merge, plan.merge_slots());
    }

    /// Bytes held (the high-water capacity).
    pub fn memory_bytes(&self) -> usize {
        self.len.capacity() * size_of::<u16>()
            + (self.vals.capacity() + self.merge.capacity()) * size_of::<u32>()
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
        size_of::<SparseStream>() + self.rows() * size_of::<u16>() + self.slots * size_of::<u32>()
    }

    /// Bytes of a scratch prepared for this plan alone
    /// ([`SparseScratch::memory_bytes`]).
    pub(crate) fn scratch_bytes(&self) -> usize {
        self.rows() * size_of::<u16>() + (self.slots + self.merge_slots()) * size_of::<u32>()
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
        // Exactly the plan's size when the allocation has to grow.
        self.len.clear();
        self.len.reserve_exact(plan.rows());
        self.len.resize(plan.rows(), 0);
        self.vals.clear();
        self.vals.reserve_exact(plan.slots);
        self.vals.resize(plan.slots, 0);
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
        for &b in input {
            if self.step(plan, regex, scratch, b) {
                return;
            }
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

    /// A closure that only looks for a match: the raw frontier stays as
    /// it is.
    fn provisional(&mut self, plan: &SparsePlan, scratch: &mut SparseScratch, boundary: Boundary) {
        stat!(provisional += 1);
        let ctx = self.ctx(boundary);
        self.matched |= close(
            plan,
            &self.len,
            &self.vals,
            &mut scratch.len[..plan.rows()],
            &mut scratch.vals[..plan.slots],
            &mut scratch.merge,
            ctx,
        );
    }

    /// Close before `b`, then consume it: the raw frontier is replaced by
    /// the consuming successors.  Returns whether a match was found first.
    fn step(
        &mut self,
        plan: &SparsePlan,
        regex: &Regex,
        scratch: &mut SparseScratch,
        b: u8,
    ) -> bool {
        stat!(steps_full += 1);
        let ctx = self.ctx(Boundary::BeforeByte(b));
        let (len, vals) = (
            &mut scratch.len[..plan.rows()],
            &mut scratch.vals[..plan.slots],
        );
        if close(
            plan,
            &self.len,
            &self.vals,
            len,
            vals,
            &mut scratch.merge,
            ctx,
        ) {
            self.matched = true;
            return true;
        }
        consume(
            plan,
            regex,
            len,
            vals,
            &mut self.len,
            &mut self.vals,
            &mut scratch.merge,
            b,
        );
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
        (n != NO_NODE && plan.nodes[n as usize].cap > 0)
            .then(|| row(&plan.nodes[n as usize], &self.len, &self.vals, n))
    }

    /// Whether the raw row of `state` is active (for tests).
    #[cfg(test)]
    pub(crate) fn raw_active(&self, regex: &Regex, state: StateIdx) -> bool {
        let n = self.plan(regex).node_of[state.idx()];
        n != NO_NODE && self.len[n as usize] != 0
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
                    let counts: Vec<String> = row(node, &s.len, &s.vals, n as NodeIdx)
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
