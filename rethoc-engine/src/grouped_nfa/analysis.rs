//! Admission analysis: scope proof, zero-width schedule and row layout.
//!
//! Runs once in [`RegexBuilder::build`](crate::RegexBuilder::build), on the
//! final (byte-table optimised) state array, and either produces an
//! immutable [`GroupedNfaPlan`] or a [`Rejection`] naming the first reason
//! the graph is outside the admitted class (`plans/grouped-nfa.md`, §2–3).
//!
//! Every budget is checked before the array it bounds is allocated.

use std::fmt;

use super::{Consumer, ConsumerKind, GroupedNfaPlan, Increment, Row, ZeroOp};
use crate::{AssertKind, ByteMap, CounterIdx, CounterInfo, State, StateIdx};

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

/// True for states that consume a byte.
pub(crate) fn is_consuming(state: &State) -> bool {
    matches!(
        state,
        State::Byte { .. }
            | State::ByteCI { .. }
            | State::Wildcard { .. }
            | State::ByteClassStatic { .. }
            | State::ByteClassCustom { .. }
            | State::ByteTable { .. }
    )
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
        let state = reachable
            .iter()
            .copied()
            .filter(|s| indegree[s.idx()] > 0)
            .min()
            .expect("an unsorted state remains");
        return Err(Rejection::ZeroWidthCycle { state: state.0 });
    }
    Ok(order)
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
/// labels and indegrees indexed by state, the reachable list, the work
/// stacks and the order (each at most one entry per state), and the bounds
/// per counter.
pub(crate) fn analysis_bytes(states: usize, num_counters: usize) -> usize {
    states * (size_of::<Option<Scope>>() + size_of::<u32>() + 4 * size_of::<StateIdx>())
        + num_counters * size_of::<Option<(usize, usize)>>()
}

/// Analyse the final state array and build a plan, or say why not.
pub(crate) fn analyze(
    states: &[State],
    byte_tables: &[ByteMap],
    start: StateIdx,
    num_counters: usize,
    counter_info: &[CounterInfo],
    limits: &Limits,
) -> Result<GroupedNfaPlan, Rejection> {
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

    // Row layout, in topological order so a closure pass walks the arena
    // roughly front to back.
    let mut words = 0usize;
    for &idx in &order {
        let scope = label[idx.idx()].expect("reachable");
        words = words.saturating_add(row_words(scope, &bounds));
    }
    if words > limits.max_row_words {
        return Err(Rejection::TooManyRowWords {
            words,
            limit: limits.max_row_words,
        });
    }

    let n_ops = order.iter().filter(|&&s| !is_consuming(&states[s])).count();
    let n_consumers = order.len() - n_ops;
    let plan_bytes = GroupedNfaPlan::heap_bytes_for(states.len(), order.len(), n_ops, n_consumers);
    if plan_bytes > limits.max_plan_bytes {
        return Err(Rejection::PlanTooLarge {
            bytes: plan_bytes,
            limit: limits.max_plan_bytes,
        });
    }

    let mut row_of = vec![Row::NONE; states.len()];
    let mut scopes = Vec::with_capacity(order.len());
    let mut off = 0u32;
    for &idx in &order {
        let scope = label[idx.idx()].expect("reachable");
        let w = row_words(scope, &bounds) as u32;
        row_of[idx.idx()] = Row { off, words: w };
        scopes.push((idx, scope));
        off += w;
    }

    let mut ops = Vec::with_capacity(n_ops);
    let mut consumers = Vec::with_capacity(n_consumers);
    let row = |s: StateIdx| -> Row {
        let r = row_of[s.idx()];
        debug_assert!(
            r != Row::NONE,
            "successor of a reachable state is reachable"
        );
        r
    };
    for &idx in &order {
        let src = row(idx);
        match states[idx] {
            State::Split { out, out1 } => ops.push(ZeroOp::Split {
                src,
                out: row(out),
                out1: row(out1),
            }),
            State::CounterInstance { out, .. } => {
                ops.push(ZeroOp::CounterInstance { src, out: row(out) })
            }
            State::CounterIncrement {
                out,
                out1,
                min,
                max,
                ..
            } => ops.push(ZeroOp::CounterIncrement {
                src,
                out: row(out),
                out1: row(out1),
                // The word budget passed, so the domain fits `u32`.
                inc: Increment::new(min, max),
            }),
            State::Assert { kind, out } => ops.push(ZeroOp::Assert {
                kind,
                src,
                out: row(out),
            }),
            State::Match => ops.push(ZeroOp::Match { src }),
            State::Byte { out, out_exit, .. }
            | State::ByteCI { out, out_exit, .. }
            | State::Wildcard { out, out_exit }
            | State::ByteClassStatic { out, out_exit, .. }
            | State::ByteClassCustom { out, out_exit, .. } => consumers.push(Consumer {
                state: idx,
                src,
                kind: ConsumerKind::Edge {
                    out: row(out),
                    out_exit: if out_exit == StateIdx::NONE {
                        Row::NONE
                    } else {
                        row(out_exit)
                    },
                },
            }),
            State::ByteTable { table } => consumers.push(Consumer {
                state: idx,
                src,
                kind: ConsumerKind::Table(table),
            }),
        }
    }

    let start = row(start);
    let plan = GroupedNfaPlan {
        ops: ops.into_boxed_slice(),
        consumers: consumers.into_boxed_slice(),
        row_of: row_of.into_boxed_slice(),
        scopes: scopes.into_boxed_slice(),
        start,
        words: off as usize,
    };
    debug_assert_eq!(plan.heap_bytes(), plan_bytes);
    Ok(plan)
}
