//! Admission analysis: scope proof, zero-width schedule and row layout.
//!
//! Runs once in [`RegexBuilder::build`](crate::RegexBuilder::build), on the
//! final (byte-table optimised) state array, and either produces an
//! immutable [`GroupedNfaPlan`] or a [`Rejection`] naming the first reason
//! the graph is outside the admitted class (`plans/grouped-nfa.md`, §2–3).
//!
//! It has two parts.  [`structure`] proves what does not depend on how
//! count sets are stored: one scope per reachable state, an acyclic
//! zero-width graph, reviewed assertions and consistent counter bounds
//! ([`Structure`]).  [`layout`] then sizes the exact rows and builds the
//! plan.  The sparse variant ([`super::sparse`]) lays out the same
//! [`Structure`] its own way, so a graph rejected here only for its row
//! words can still be admitted there.
//!
//! Every budget is checked before the array it bounds is allocated.

use std::fmt;

use super::CountFreeTables;
use super::{GroupedNfaPlan, Increment, NO_NODE, Node, NodeIdx, Op, Row, Succ};
use crate::{AssertKind, ByteClassBits, ByteMap, CounterIdx, CounterInfo, State, StateIdx};

// ---------------------------------------------------------------------------
// Limits and rejections
// ---------------------------------------------------------------------------

/// Experimental admission limits (plan §3, "Experimental admission
/// limits").  They bound the prototype; they are not production targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Limits {
    /// Most states in the whole optimised state array, checked before any
    /// `StateIdx`-indexed analysis array is allocated.
    pub(crate) max_states: usize,
    /// Most edge entries examined, a `ByteTable` counting all 256 entries.
    pub(crate) max_edges: usize,
    /// Most `u64` words in one frontier (`W`).
    pub(crate) max_row_words: usize,
    /// Most bytes of retained plan arrays.
    pub(crate) max_plan_bytes: usize,
    /// Most bytes of transient analysis arrays.
    pub(crate) max_analysis_bytes: usize,
}

impl Limits {
    /// The experimental limits.  Counted domains have no limit of their
    /// own: a counter is admitted when its rows fit `max_row_words`.
    pub(crate) const DEFAULT: Limits = Limits {
        max_states: 4096,
        max_edges: 1 << 20,
        max_row_words: 8192,
        max_plan_bytes: 1 << 20,
        max_analysis_bytes: 2 << 20,
    };
}

/// Why a graph is not admitted.  Normal compilation and dispatch are
/// unaffected by any of these.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rejection {
    /// The state array is larger than the analysis budget.
    TooManyStates { states: usize, limit: usize },
    /// More edge entries than the analysis budget.
    TooManyEdges { limit: usize },
    /// The transient analysis arrays would exceed their budget.
    AnalysisTooLarge { bytes: usize, limit: usize },
    /// A state is reachable both outside a counted scope and inside one,
    /// or inside two different ones.
    ConflictingScope { state: u32 },
    /// A `CounterInstance` is reachable inside a counted scope: nesting.
    NestedCounter { state: u32 },
    /// A `CounterIncrement` is reachable outside its own counter's scope.
    IncrementScopeMismatch { state: u32 },
    /// `Match` is reachable inside a counted scope.
    MatchInScope { state: u32 },
    /// Two reachable increments of one counter disagree on its bounds, or
    /// the counter metadata disagrees with them.
    CounterBoundsMismatch { counter: u8 },
    /// Bounds the transfers are not defined for (`max == 0`, `min > max`).
    UnsupportedBounds { counter: u8, min: usize, max: usize },
    /// The zero-width graph has a cycle through this state (for example a
    /// nullable counted body).
    ZeroWidthCycle { state: u32 },
    /// An assertion of a kind not reviewed for raw replay (see
    /// [`reviewed`]).
    UnreviewedAssertion { state: u32 },
    /// The frontier would need more words than the budget.
    TooManyRowWords { words: usize, limit: usize },
    /// The retained plan arrays would exceed their budget.
    PlanTooLarge { bytes: usize, limit: usize },
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TooManyStates { states, limit } => {
                write!(f, "{states} NFA states exceed the limit of {limit}")
            }
            Self::TooManyEdges { limit } => write!(f, "more than {limit} edge entries"),
            Self::AnalysisTooLarge { bytes, limit } => {
                write!(f, "analysis needs {bytes} bytes, limit {limit}")
            }
            Self::ConflictingScope { state } => {
                write!(f, "state {state} is reachable in two different scopes")
            }
            Self::NestedCounter { state } => {
                write!(
                    f,
                    "counter instance {state} is inside a counted scope (nested)"
                )
            }
            Self::IncrementScopeMismatch { state } => {
                write!(
                    f,
                    "counter increment {state} is outside its counter's scope"
                )
            }
            Self::MatchInScope { state } => {
                write!(f, "match state {state} is inside a counted scope")
            }
            Self::CounterBoundsMismatch { counter } => {
                write!(f, "counter c{counter} has inconsistent bounds")
            }
            Self::UnsupportedBounds { counter, min, max } => {
                write!(
                    f,
                    "counter c{counter} has unsupported bounds {{{min},{max}}}"
                )
            }
            Self::ZeroWidthCycle { state } => {
                write!(f, "zero-width cycle through state {state}")
            }
            Self::UnreviewedAssertion { state } => {
                write!(f, "assertion at state {state} is of an unreviewed kind")
            }
            Self::TooManyRowWords { words, limit } => {
                write!(f, "frontier needs {words} words, limit {limit}")
            }
            Self::PlanTooLarge { bytes, limit } => {
                write!(f, "plan needs {bytes} bytes, limit {limit}")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Scope proof
// ---------------------------------------------------------------------------

/// The proven incoming scope of a reachable state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Scope {
    /// No counter is active: a row is one word, bit 0 = reachable.
    Outside,
    /// Exactly this counter is active: a row holds its reachable counts.
    Counter(CounterIdx),
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Outside => write!(f, "out"),
            Scope::Counter(c) => write!(f, "c{c}"),
        }
    }
}

/// Call `edge(target)` for every successor of `state`, over all edges
/// (consuming and zero-width).
fn for_each_successor(state: &State, byte_tables: &[ByteMap], mut edge: impl FnMut(StateIdx)) {
    match *state {
        State::Split { out, out1 } => {
            edge(out);
            edge(out1);
        }
        State::CounterInstance { out, .. } | State::Assert { out, .. } => edge(out),
        State::CounterIncrement { out, out1, .. } => {
            edge(out);
            edge(out1);
        }
        State::Byte { out, out_exit, .. }
        | State::ByteCI { out, out_exit, .. }
        | State::Wildcard { out, out_exit }
        | State::ByteClassStatic { out, out_exit, .. }
        | State::ByteClassCustom { out, out_exit, .. } => {
            edge(out);
            if out_exit != StateIdx::NONE {
                edge(out_exit);
            }
        }
        State::ByteTable { table } => {
            for &target in byte_tables[table].0.iter() {
                if target != StateIdx::NONE {
                    edge(target);
                }
            }
        }
        State::Match => {}
    }
}

/// Number of edge entries [`for_each_successor`] examines for `state`.
fn edge_entries(state: &State) -> usize {
    match state {
        State::ByteTable { .. } => 256,
        State::Match => 0,
        _ => 2,
    }
}

/// Propagate scope labels from `start` (Outside) over every edge.
///
/// Returns the label of each state (`None` = unreachable) and the reachable
/// states in discovery order.
fn prove_scopes(
    states: &[State],
    byte_tables: &[ByteMap],
    start: StateIdx,
    limits: &Limits,
) -> Result<(Vec<Option<Scope>>, Vec<StateIdx>), Rejection> {
    let mut label: Vec<Option<Scope>> = vec![None; states.len()];
    let mut reachable = Vec::new();
    let mut stack = vec![start];
    label[start.idx()] = Some(Scope::Outside);
    let mut edges = 0usize;

    // Assign `scope` to `target`, rejecting a different second label.
    let visit = |target: StateIdx,
                 scope: Scope,
                 label: &mut [Option<Scope>],
                 stack: &mut Vec<StateIdx>|
     -> Result<(), Rejection> {
        match label[target.idx()] {
            None => {
                label[target.idx()] = Some(scope);
                stack.push(target);
                Ok(())
            }
            Some(s) if s == scope => Ok(()),
            Some(_) => Err(Rejection::ConflictingScope { state: target.0 }),
        }
    };

    while let Some(idx) = stack.pop() {
        reachable.push(idx);
        let state = &states[idx];
        edges += edge_entries(state);
        if edges > limits.max_edges {
            return Err(Rejection::TooManyEdges {
                limit: limits.max_edges,
            });
        }
        let scope = label[idx.idx()].expect("a stacked state is labelled");
        match *state {
            State::CounterInstance { counter, out } => {
                if scope != Scope::Outside {
                    return Err(Rejection::NestedCounter { state: idx.0 });
                }
                visit(out, Scope::Counter(counter), &mut label, &mut stack)?;
            }
            State::CounterIncrement {
                counter, out, out1, ..
            } => {
                if scope != Scope::Counter(counter) {
                    return Err(Rejection::IncrementScopeMismatch { state: idx.0 });
                }
                visit(out, Scope::Counter(counter), &mut label, &mut stack)?;
                visit(out1, Scope::Outside, &mut label, &mut stack)?;
            }
            State::Match => {
                if scope != Scope::Outside {
                    return Err(Rejection::MatchInScope { state: idx.0 });
                }
            }
            _ => {
                let mut result = Ok(());
                for_each_successor(state, byte_tables, |target| {
                    if result.is_ok() {
                        result = visit(target, scope, &mut label, &mut stack);
                    }
                });
                result?;
            }
        }
    }
    Ok((label, reachable))
}

// ---------------------------------------------------------------------------
// Counter domains
// ---------------------------------------------------------------------------

/// Bounds of each counter, from its reachable increments.  An unbounded
/// counter has `max == usize::MAX`.
fn counter_bounds(
    states: &[State],
    reachable: &[StateIdx],
    counter_info: &[CounterInfo],
    num_counters: usize,
) -> Result<Vec<Option<(usize, usize)>>, Rejection> {
    let mut bounds: Vec<Option<(usize, usize)>> = vec![None; num_counters];
    for &idx in reachable {
        if let State::CounterIncrement {
            counter, min, max, ..
        } = states[idx]
        {
            let c = counter.0;
            match bounds[counter.idx()] {
                None => bounds[counter.idx()] = Some((min, max)),
                Some(b) if b == (min, max) => {}
                Some(_) => return Err(Rejection::CounterBoundsMismatch { counter: c }),
            }
            if let Some(info) = counter_info.iter().find(|i| i.index == counter.idx())
                && (info.min, info.max) != (min, max)
            {
                return Err(Rejection::CounterBoundsMismatch { counter: c });
            }
            if max != usize::MAX && (max == 0 || min > max) {
                return Err(Rejection::UnsupportedBounds {
                    counter: c,
                    min,
                    max,
                });
            }
        }
    }
    Ok(bounds)
}

// ---------------------------------------------------------------------------
// Zero-width topological order
// ---------------------------------------------------------------------------

/// Successors of `state` in the zero-width graph: edges leave only
/// `Split`, `Assert`, `CounterInstance` and `CounterIncrement`.
fn for_each_zero_width_successor(state: &State, mut edge: impl FnMut(StateIdx)) {
    match *state {
        State::Split { out, out1 } | State::CounterIncrement { out, out1, .. } => {
            edge(out);
            edge(out1);
        }
        State::Assert { out, .. } | State::CounterInstance { out, .. } => edge(out),
        _ => {}
    }
}

/// Topologically sort the reachable states over zero-width edges (Kahn).
/// Duplicate edges are counted in indegrees and released once each.
fn zero_width_order(states: &[State], reachable: &[StateIdx]) -> Result<Vec<StateIdx>, Rejection> {
    let mut indegree = vec![0u32; states.len()];
    for &idx in reachable {
        for_each_zero_width_successor(&states[idx], |t| indegree[t.idx()] += 1);
    }
    let mut ready: Vec<StateIdx> = reachable
        .iter()
        .copied()
        .filter(|s| indegree[s.idx()] == 0)
        .collect();
    let mut order = Vec::with_capacity(reachable.len());
    while let Some(idx) = ready.pop() {
        order.push(idx);
        for_each_zero_width_successor(&states[idx], |t| {
            indegree[t.idx()] -= 1;
            if indegree[t.idx()] == 0 {
                ready.push(t);
            }
        });
    }
    if order.len() != reachable.len() {
        let state = state_on_cycle(states, reachable, &indegree);
        return Err(Rejection::ZeroWidthCycle { state: state.0 });
    }
    Ok(order)
}

/// A state on a zero-width cycle, after Kahn's sort left the states with a
/// nonzero `indegree`.  Those include states downstream of a cycle, but
/// each has a predecessor among them, so walking predecessors from any of
/// them ends on a cycle.  Returns the lowest-numbered state of that cycle.
fn state_on_cycle(states: &[State], reachable: &[StateIdx], indegree: &[u32]) -> StateIdx {
    let left = |s: StateIdx| indegree[s.idx()] > 0;
    let mut pred = vec![StateIdx::NONE; states.len()];
    for &s in reachable.iter().filter(|&&s| left(s)) {
        for_each_zero_width_successor(&states[s], |t| {
            if left(t) {
                pred[t.idx()] = s;
            }
        });
    }
    let mut seen = vec![false; states.len()];
    let mut s = reachable
        .iter()
        .copied()
        .find(|&s| left(s))
        .expect("an unsorted state remains");
    while !std::mem::replace(&mut seen[s.idx()], true) {
        s = pred[s.idx()];
        debug_assert!(
            s != StateIdx::NONE,
            "a remaining state has a remaining predecessor"
        );
    }
    // `s` is on the cycle: walk it once more for its lowest state.
    let (start, mut lowest, mut t) = (s, s, pred[s.idx()]);
    while t != start {
        lowest = lowest.min(t);
        t = pred[t.idx()];
    }
    lowest
}

// ---------------------------------------------------------------------------
// Plan construction
// ---------------------------------------------------------------------------

/// Words of a row in `scope`.  A finite counter's domain is `0..=max`;
/// an unbounded one saturates at `min`, so its domain is `0..=min` (never
/// sized from the `usize::MAX` sentinel).
fn row_words(scope: Scope, bounds: &[Option<(usize, usize)>]) -> usize {
    match scope {
        Scope::Outside => 1,
        Scope::Counter(c) => {
            let (min, max) = bounds[c.idx()].expect("a counted scope has an increment");
            let top = if max == usize::MAX { min } else { max };
            top / 64 + 1
        }
    }
}

/// Whether raw replay (plan §5) has been reviewed for this assertion kind:
/// it reads only the shared boundary context (stream start, previous byte,
/// next byte or end of input); with a known next byte or at end of input
/// it never defers; and a pass before the next byte is known stays a pass
/// for every next byte and at end of input.  The tests check the last two
/// properties on `AssertKind::eval` for every kind.
///
/// The match is exhaustive on purpose: a new kind does not compile until
/// it is listed here, and it should be listed as `false` until it has been
/// reviewed.
pub(crate) const fn reviewed(kind: AssertKind) -> bool {
    match kind {
        AssertKind::Start
        | AssertKind::End
        | AssertKind::StartLF
        | AssertKind::EndLF
        | AssertKind::StartCRLF
        | AssertKind::EndCRLF
        | AssertKind::WordAscii
        | AssertKind::WordAsciiNegate
        | AssertKind::WordStartAscii
        | AssertKind::WordEndAscii => true,
    }
}

/// Upper bound of the transient analysis arrays for `states` states:
/// labels, indegrees, predecessors and three sets of flags indexed by
/// state, the reachable list, the work stacks and the order (each at most
/// one entry per state), and the bounds per counter.
pub(crate) fn analysis_bytes(states: usize, num_counters: usize) -> usize {
    states
        * (size_of::<Option<Scope>>()
            + size_of::<u32>()
            + 3 * size_of::<bool>()
            + 5 * size_of::<StateIdx>())
        + num_counters * size_of::<Option<(usize, usize)>>()
}

/// What [`structure`] proves of a graph, whatever the representation of
/// its count sets.
#[derive(Debug)]
pub(crate) struct Structure {
    /// The scope of each state; `None` if unreachable.
    pub(crate) label: Vec<Option<Scope>>,
    /// The reachable states in a topological order of the zero-width
    /// graph.
    pub(crate) order: Vec<StateIdx>,
    /// The bounds of each counter with a reachable increment; unbounded is
    /// `max == usize::MAX`.  Finite bounds satisfy `1 <= max` and
    /// `min <= max`.
    pub(crate) bounds: Vec<Option<(usize, usize)>>,
}

/// Analyse the final state array and build a plan, or say why not:
/// [`structure`], then [`layout`].
#[cfg(test)]
pub(crate) fn analyze(
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
    start: StateIdx,
    num_counters: usize,
    counter_info: &[CounterInfo],
    limits: &Limits,
) -> Result<GroupedNfaPlan, Rejection> {
    let structure = structure(
        states,
        byte_tables,
        start,
        num_counters,
        counter_info,
        limits,
    )?;
    layout(states, classes, byte_tables, start, &structure, limits)
}

/// Prove the structure of the final state array: the part of the
/// admission that does not depend on the representation of count sets.
/// Reads `max_states`, `max_edges` and `max_analysis_bytes` of `limits`.
pub(crate) fn structure(
    states: &[State],
    byte_tables: &[ByteMap],
    start: StateIdx,
    num_counters: usize,
    counter_info: &[CounterInfo],
    limits: &Limits,
) -> Result<Structure, Rejection> {
    if states.len() > limits.max_states {
        return Err(Rejection::TooManyStates {
            states: states.len(),
            limit: limits.max_states,
        });
    }
    let analysis_bytes = analysis_bytes(states.len(), num_counters);
    if analysis_bytes > limits.max_analysis_bytes {
        return Err(Rejection::AnalysisTooLarge {
            bytes: analysis_bytes,
            limit: limits.max_analysis_bytes,
        });
    }

    let (label, reachable) = prove_scopes(states, byte_tables, start, limits)?;
    let order = zero_width_order(states, &reachable)?;
    if let Some(&idx) = order
        .iter()
        .find(|&&s| matches!(states[s], State::Assert { kind, .. } if !reviewed(kind)))
    {
        return Err(Rejection::UnreviewedAssertion { state: idx.0 });
    }
    let bounds = counter_bounds(states, &reachable, counter_info, num_counters)?;
    Ok(Structure {
        label,
        order,
        bounds,
    })
}

/// Lay out the exact rows of a proven [`Structure`] and build the plan.
/// Reads `max_row_words` and `max_plan_bytes` of `limits`.
pub(crate) fn layout(
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
    start: StateIdx,
    structure: &Structure,
    limits: &Limits,
) -> Result<GroupedNfaPlan, Rejection> {
    let Structure {
        label,
        order,
        bounds,
    } = structure;

    // Row layout, in topological order so a closure pass walks the arena
    // roughly front to back.
    let mut words = 0usize;
    for &idx in order {
        let scope = label[idx.idx()].expect("reachable");
        words = words.saturating_add(row_words(scope, bounds));
    }
    if words > limits.max_row_words {
        return Err(Rejection::TooManyRowWords {
            words,
            limit: limits.max_row_words,
        });
    }

    // Count-free tables: when the start node is count-free (it seeds every
    // closure) and the bitmaps are small enough.  They are optional: a plan
    // that fits without them is admitted without them.
    let a = order.len().div_ceil(64);
    let mut count_free =
        (a <= MAX_COUNT_FREE_WORDS && closure_is_count_free(states, start)).then_some(a);
    let mut plan_bytes = GroupedNfaPlan::heap_bytes_for(states.len(), order.len(), count_free);
    if plan_bytes > limits.max_plan_bytes && count_free.is_some() {
        count_free = None;
        plan_bytes = GroupedNfaPlan::heap_bytes_for(states.len(), order.len(), None);
    }
    if plan_bytes > limits.max_plan_bytes {
        return Err(Rejection::PlanTooLarge {
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
    // Rows in node order.
    let mut rows = Vec::with_capacity(order.len());
    let mut off = 0u32;
    for &idx in order {
        let words = row_words(label[idx.idx()].expect("reachable"), bounds) as u32;
        rows.push(Row { off, words });
        off += words;
    }
    let succ = |s: StateIdx| -> Succ {
        let node = node(s);
        Succ {
            node,
            off: rows[node as usize].off,
        }
    };
    let mut nodes = Vec::with_capacity(order.len());
    for (i, &idx) in order.iter().enumerate() {
        let scope = label[idx.idx()].expect("reachable");
        let op = match states[idx] {
            State::Split { out, out1 } => Op::Split {
                out: succ(out),
                out1: succ(out1),
            },
            State::CounterInstance { out, .. } => Op::CounterInstance { out: succ(out) },
            State::CounterIncrement {
                out,
                out1,
                min,
                max,
                ..
            } => Op::CounterIncrement {
                out: succ(out),
                out1: succ(out1),
                // The word budget passed, so the domain fits `u32`.
                inc: Increment::new(min, max),
            },
            State::Assert { kind, out } => Op::Assert {
                kind,
                out: succ(out),
            },
            State::Match => Op::Match,
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => Op::Edge {
                out: succ(out),
                out_exit: if out_exit == StateIdx::NONE {
                    Succ::NONE
                } else {
                    succ(out_exit)
                },
            },
            State::ByteTable { table } => Op::Table(table),
        };
        nodes.push(Node {
            state: idx,
            scope,
            row: rows[i],
            op,
        });
    }

    debug_check_successors(&nodes);
    let (consumers, zero_width): (Vec<Node>, Vec<Node>) = nodes
        .iter()
        .partition(|n| matches!(n.op, Op::Edge { .. } | Op::Table(_)));
    let count_free = count_free.map(|a| {
        Box::new(count_free_tables(
            nodes.len(),
            |n| match nodes[n].op {
                Op::Assert { .. } | Op::CounterIncrement { .. } => CountFreeNode::Blocked,
                Op::Split { out, out1 } => CountFreeNode::ZeroWidth([out.node, out1.node]),
                Op::CounterInstance { out } => CountFreeNode::ZeroWidth([out.node, NO_NODE]),
                Op::Match => CountFreeNode::Match,
                Op::Edge { .. } | Op::Table(_) => CountFreeNode::Consumer(nodes[n].state),
            },
            node(start),
            a,
            states,
            classes,
            byte_tables,
        ))
    });
    let plan = GroupedNfaPlan {
        count_free,
        zero_width: zero_width.into_boxed_slice(),
        consumers: consumers.into_boxed_slice(),
        start: node(start),
        skip_when_idle: start_closure_is_static(states, start),
        active_words: nodes.len().div_ceil(64),
        nodes: nodes.into_boxed_slice(),
        node_of: node_of.into_boxed_slice(),
        words: off as usize,
    };
    debug_assert_eq!(plan.heap_bytes(), plan_bytes);
    Ok(plan)
}

/// Most bitmap words for which count-free tables are built: 512 rows.
/// Their size grows with rows times words.
pub(crate) const MAX_COUNT_FREE_WORDS: usize = 8;

/// Walk the zero-width closure of `start`, calling `visit` once for every
/// state of it, until `visit` returns `false`.  Returns whether it never
/// did.  This is the one walk behind every question asked of a start
/// closure: its edges are those of [`for_each_zero_width_successor`], so
/// a new zero-width state kind is handled in one place.
fn walk_closure(states: &[State], start: StateIdx, mut visit: impl FnMut(&State) -> bool) -> bool {
    let mut seen = vec![false; states.len()];
    let mut stack = vec![start];
    while let Some(idx) = stack.pop() {
        if std::mem::replace(&mut seen[idx.idx()], true) {
            continue;
        }
        let state = &states[idx];
        if !visit(state) {
            return false;
        }
        for_each_zero_width_successor(state, |t| stack.push(t));
    }
    true
}

/// Whether the zero-width closure of `start` reaches neither an assertion
/// nor a `CounterIncrement`.
pub(crate) fn closure_is_count_free(states: &[State], start: StateIdx) -> bool {
    walk_closure(states, start, |s| {
        !matches!(s, State::Assert { .. } | State::CounterIncrement { .. })
    })
}

/// A state of a closure that depends on the boundary (an assertion) or
/// matches: with one in the start closure, an empty frontier cannot be
/// skipped over.
fn is_static(state: &State) -> bool {
    !matches!(state, State::Assert { .. } | State::Match)
}

/// Whether the consuming `state` accepts `b`.
pub(crate) fn consumer_accepts(
    state: &State,
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
    b: u8,
) -> bool {
    match *state {
        State::ByteTable { table } => byte_tables[table][b] != StateIdx::NONE,
        _ => super::exact::accepts(classes, state, b),
    }
}

/// When the zero-width closure of `start` is static
/// ([`start_closure_is_static`]): the bytes that some consuming state of
/// it accepts.  From an empty frontier, any other byte leaves the frontier
/// empty.
pub(crate) fn start_bytes(
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
    start: StateIdx,
) -> Option<Box<[bool; 256]>> {
    let mut starts = Box::new([false; 256]);
    walk_closure(states, start, |state| {
        let consumes = !matches!(
            state,
            State::Split { .. }
                | State::Assert { .. }
                | State::CounterInstance { .. }
                | State::CounterIncrement { .. }
                | State::Match
        );
        if consumes {
            for b in 0..=255u8 {
                starts[b as usize] |= consumer_accepts(state, classes, byte_tables, b);
            }
        }
        is_static(state)
    })
    .then_some(starts)
}

/// What the count-free tables need to know of a node, whatever the
/// representation of its row.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CountFreeNode {
    /// An assertion or a `CounterIncrement`: never count-free.
    Blocked,
    /// A `Split` or a `CounterInstance`, with its one or two successors
    /// (the second is [`NO_NODE`] when there is one).
    ZeroWidth([NodeIdx; 2]),
    Match,
    /// Consumes a byte, as its state says.
    Consumer(StateIdx),
}

/// Build the count-free tables (see [`CountFreeTables`]) of the `r` nodes
/// that `node` describes, in node order; the start node must be
/// count-free.  The exact rows and the sparse ones number their nodes
/// alike and share these tables' meaning: a unit row holds the count of a
/// counter that has not counted yet.
pub(crate) fn count_free_tables(
    r: usize,
    node: impl Fn(usize) -> CountFreeNode,
    start: NodeIdx,
    a: usize,
    states: &[State],
    classes: &[ByteClassBits],
    byte_tables: &[ByteMap],
) -> CountFreeTables {
    let bit = |n: usize| (n / 64, 1u64 << (n % 64));
    let mut count_free = vec![false; r];
    let mut closure = vec![0u64; r * a];
    // Successors come later in node order: fill backwards.
    for n in (0..r).rev() {
        let succs = match node(n) {
            CountFreeNode::Blocked => continue,
            CountFreeNode::ZeroWidth(succs) => succs,
            CountFreeNode::Match | CountFreeNode::Consumer(_) => [NO_NODE; 2],
        };
        let succs = succs.iter().filter(|&&s| s != NO_NODE);
        if !succs.clone().all(|&s| count_free[s as usize]) {
            continue;
        }
        count_free[n] = true;
        let (w, m) = bit(n);
        closure[n * a + w] |= m;
        for &s in succs {
            let src = s as usize * a;
            for i in 0..a {
                closure[n * a + i] |= closure[src + i];
            }
        }
    }
    let start = start as usize;
    assert!(count_free[start], "the start node is count-free");
    let mut accept = vec![0u64; 256 * a];
    let (mut statics, mut matches) = (vec![0u64; a], vec![0u64; a]);
    for (n, &free) in count_free.iter().enumerate() {
        let (w, m) = bit(n);
        if free {
            statics[w] |= m;
        }
        let kind = node(n);
        match kind {
            CountFreeNode::Match => matches[w] |= m,
            CountFreeNode::Consumer(state) => {
                for b in 0..=255u8 {
                    if consumer_accepts(&states[state.idx()], classes, byte_tables, b) {
                        accept[b as usize * a + w] |= m;
                    }
                }
            }
            _ => {}
        }
    }
    let start_closure: Box<[u64]> = closure[start * a..][..a].into();
    let mut starts = Box::new([false; 256]);
    for (b, s) in starts.iter_mut().enumerate() {
        *s = (0..a).any(|i| start_closure[i] & accept[b * a + i] != 0);
    }
    CountFreeTables {
        a,
        closure: closure.into_boxed_slice(),
        accept: accept.into_boxed_slice(),
        statics: statics.into_boxed_slice(),
        matches: matches.into_boxed_slice(),
        start_closure,
        starts,
    }
}

/// Check (in debug builds) what the transfers assume of a successor
/// without looking it up: its offset is its node's row, and its width is
/// the source's, one word (Outside) after a break, or the counter's after
/// an instance.
fn debug_check_successors(nodes: &[Node]) {
    if !cfg!(debug_assertions) {
        return;
    }
    let check = |src: &Node, s: Succ, same_width: bool| {
        if s == Succ::NONE {
            return;
        }
        let dst = &nodes[s.node as usize];
        assert_eq!(
            s.off, dst.row.off,
            "successor offset of state {}",
            src.state
        );
        if same_width {
            assert_eq!(
                dst.row.words, src.row.words,
                "successor width of state {}",
                src.state
            );
        }
    };
    for n in nodes {
        match n.op {
            Op::Split { out, out1 } => {
                check(n, out, true);
                check(n, out1, true);
            }
            Op::Assert { out, .. } => check(n, out, true),
            Op::Edge { out, out_exit } => {
                check(n, out, true);
                check(n, out_exit, true);
            }
            Op::CounterInstance { out } => {
                check(n, out, false);
                assert!(matches!(nodes[out.node as usize].scope, Scope::Counter(_)));
            }
            Op::CounterIncrement { out, out1, .. } => {
                check(n, out, true);
                check(n, out1, false);
                assert_eq!(nodes[out1.node as usize].scope, Scope::Outside);
            }
            Op::Match | Op::Table(_) => {}
        }
    }
}

/// Whether the zero-width closure of `start` reaches neither an assertion
/// nor `Match`: then it is the same at every boundary and never matches.
fn start_closure_is_static(states: &[State], start: StateIdx) -> bool {
    walk_closure(states, start, is_static)
}
