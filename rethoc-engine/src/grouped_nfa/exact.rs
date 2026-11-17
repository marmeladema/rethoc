//! Exact grouped transfers and the streaming runner (plan §4–5).

use std::fmt;

use super::{CincMasks, ConsumerKind, GroupedNfaError, GroupedNfaPlan, Row, ZeroOp};
use crate::{AssertEval, Regex, State, StateIdx, byte_match_ci};

// ---------------------------------------------------------------------------
// Boundary context
// ---------------------------------------------------------------------------

/// What is known about the input at the current byte boundary.  Every
/// assertion in one closure sees the same boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Boundary {
    /// The current chunk has ended; future input is unknown.
    UnknownNext,
    /// `b` is the next byte and is still unconsumed.
    BeforeByte(u8),
    /// The caller has finished the stream.
    EndOfInput,
}

/// The boundary context an assertion is evaluated in.
#[derive(Clone, Copy, Debug)]
pub(crate) struct BoundaryCtx {
    pub(crate) at_start: bool,
    pub(crate) prev: Option<u8>,
    pub(crate) boundary: Boundary,
}

impl BoundaryCtx {
    fn eval(self, kind: crate::AssertKind) -> AssertEval {
        let (at_end, next) = match self.boundary {
            Boundary::UnknownNext => (false, None),
            Boundary::BeforeByte(b) => (false, Some(b)),
            Boundary::EndOfInput => (true, None),
        };
        kind.eval(self.at_start, at_end, self.prev, next)
    }
}

// ---------------------------------------------------------------------------
// Row primitives
// ---------------------------------------------------------------------------

#[inline]
fn is_empty(rows: &[u64], row: Row) -> bool {
    rows[row.range()].iter().all(|&w| w == 0)
}

/// `rows[dst] |= rows[src]` within one arena.  Rows of distinct states
/// never overlap, and the scope proof gives both the same width.
#[inline]
fn union_within(rows: &mut [u64], src: Row, dst: Row) {
    debug_assert_eq!(src.words, dst.words, "union of rows of different scopes");
    debug_assert_ne!(src.off, dst.off, "union of a row into itself");
    for i in 0..src.words as usize {
        rows[dst.off as usize + i] |= rows[src.off as usize + i];
    }
}

/// `to[dst] |= from[src]` across arenas.
#[inline]
fn union_into(from: &[u64], src: Row, to: &mut [u64], dst: Row) {
    debug_assert_eq!(src.words, dst.words, "union of rows of different scopes");
    for (d, s) in to[dst.range()].iter_mut().zip(&from[src.range()]) {
        *d |= *s;
    }
}

/// A finite one-word increment of the count set `s`: the incremented
/// counts that continue, and whether any of them breaks.
///
/// `T = {v + 1 : v in S}`; continue with `T ∩ 0..max`, break iff
/// `T ∩ min..=max` is nonempty.  `S` lies below `max`, since only the
/// continue edge returns into the body.
#[inline]
pub(crate) fn increment(s: u64, masks: CincMasks) -> (u64, bool) {
    assert_eq!(
        s & !masks.cont,
        0,
        "a count reached its maximum before the increment"
    );
    let t = s << 1;
    (t & masks.cont, t & masks.brk != 0)
}

/// Does the single-byte predicate of `state` accept `b`?
#[inline]
fn accepts(regex: &Regex, state: &State, b: u8) -> bool {
    match *state {
        State::Byte { byte, .. } => b == byte,
        State::ByteCI { byte, .. } => byte_match_ci(b, byte),
        State::Wildcard { .. } => true,
        State::ByteClassStatic { table, .. } => table[b as usize],
        State::ByteClassCustom { class, .. } => regex.classes[class.idx()].contains(b),
        _ => unreachable!("not a single-byte consuming state: {state:?}"),
    }
}

// ---------------------------------------------------------------------------
// Closure and consumption
// ---------------------------------------------------------------------------

/// Close `raw` into `closure`: copy every raw row, insert Outside at the
/// start (search reseeding), then apply the zero-width transfers once each
/// in topological order.  Returns whether `Match` was reached.
///
/// All rows are seeded before the pass, so a state's incoming
/// contributions are complete when it is processed.
pub(crate) fn close(
    plan: &GroupedNfaPlan,
    raw: &[u64],
    closure: &mut [u64],
    ctx: BoundaryCtx,
) -> bool {
    closure.copy_from_slice(raw);
    closure[plan.start.off as usize] |= 1;
    let mut matched = false;
    for op in plan.ops.iter() {
        match *op {
            ZeroOp::Split { src, out, out1 } => {
                if !is_empty(closure, src) {
                    union_within(closure, src, out);
                    union_within(closure, src, out1);
                }
            }
            ZeroOp::CounterInstance { src, out } => {
                if closure[src.off as usize] & 1 != 0 {
                    closure[out.off as usize] |= 1;
                }
            }
            ZeroOp::CounterIncrement {
                src,
                out,
                out1,
                masks,
            } => {
                debug_assert_eq!(src.words, 1, "one-word counters only before P2");
                let s = closure[src.off as usize];
                if s == 0 {
                    continue;
                }
                let (cont, brk) = increment(s, masks);
                closure[out.off as usize] |= cont;
                if brk {
                    closure[out1.off as usize] |= 1;
                }
            }
            ZeroOp::Assert { kind, src, out } => {
                if !is_empty(closure, src) && ctx.eval(kind) == AssertEval::Pass {
                    union_within(closure, src, out);
                }
            }
            ZeroOp::Match { src } => {
                if closure[src.off as usize] != 0 {
                    matched = true;
                }
            }
        }
    }
    matched
}

/// Consume `b` from the closed rows into `next`, which is cleared first:
/// every active consuming state tests its predicate once and moves its
/// whole row.  Returns the number of predicate tests.
pub(crate) fn consume(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &[u64],
    next: &mut [u64],
    b: u8,
) -> usize {
    next.fill(0);
    let mut tests = 0;
    for c in plan.consumers.iter() {
        if is_empty(closure, c.src) {
            continue;
        }
        tests += 1;
        match c.kind {
            ConsumerKind::Edge { out, out_exit } => {
                if accepts(regex, &regex.states[c.state], b) {
                    union_into(closure, c.src, next, out);
                    if out_exit != Row::NONE {
                        union_into(closure, c.src, next, out_exit);
                    }
                }
            }
            ConsumerKind::Table(table) => {
                let target = regex.byte_tables[table][b];
                if target != StateIdx::NONE {
                    union_into(closure, c.src, next, plan.row_of[target.idx()]);
                }
            }
        }
    }
    tests
}

// ---------------------------------------------------------------------------
// Worker scratch
// ---------------------------------------------------------------------------

/// Per-worker scratch: the closure and next-root arenas.  Borrowed only
/// during a call, so streams of different regexes can share one, one call
/// at a time.  Capacity grows to the largest plan seen and is kept.
#[derive(Debug, Default)]
pub struct GroupedScratch {
    closure: Vec<u64>,
    next: Vec<u64>,
}

impl GroupedScratch {
    /// Empty scratch; it grows on first use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Make room for a frontier of `words`, before any scanning.
    fn prepare(&mut self, words: usize) {
        if self.closure.len() < words {
            self.closure.resize(words, 0);
            self.next.resize(words, 0);
        }
    }

    /// Bytes held by the two arenas (their high-water capacity).
    pub fn memory_bytes(&self) -> usize {
        (self.closure.capacity() + self.next.capacity()) * size_of::<u64>()
    }
}

// ---------------------------------------------------------------------------
// Stream
// ---------------------------------------------------------------------------

/// One input stream on the grouped NFA of one regex.
///
/// Holds the raw rows (fixed size: exactly the plan's `W` words, whatever
/// the scratch's capacity), the previous byte, the at-start flag and the
/// sticky match flag.  Every call takes the regex it was created for and a
/// scratch.
///
/// Lifecycle: [`new`](Self::new), any number of [`chunk`](Self::chunk)
/// calls, [`finish`](Self::finish); [`reset`](Self::reset) starts over.
/// Feeding or finishing a finished stream panics, as does passing a
/// different regex.
#[derive(Debug)]
pub struct GroupedStream {
    regex_id: u64,
    raw: Box<[u64]>,
    prev: Option<u8>,
    at_start: bool,
    matched: bool,
    finished: bool,
}

impl GroupedStream {
    /// A fresh stream for `regex`, which must have been compiled with
    /// [`RegexConfig::grouped_nfa`](crate::RegexConfig::grouped_nfa) and
    /// admitted.
    pub fn new(regex: &Regex, scratch: &mut GroupedScratch) -> Result<Self, GroupedNfaError> {
        let plan = regex.grouped_plan()?;
        let mut stream = Self {
            regex_id: regex.id,
            raw: vec![0; plan.words].into_boxed_slice(),
            prev: None,
            at_start: true,
            matched: false,
            finished: false,
        };
        stream.start(plan, scratch);
        Ok(stream)
    }

    /// Return to the state of a fresh stream, keeping the allocation.
    pub fn reset(&mut self, regex: &Regex, scratch: &mut GroupedScratch) {
        let plan = self.plan(regex);
        self.raw.fill(0);
        self.prev = None;
        self.at_start = true;
        self.matched = false;
        self.finished = false;
        self.start(plan, scratch);
    }

    /// Feed the next bytes of the input.
    pub fn chunk(&mut self, regex: &Regex, scratch: &mut GroupedScratch, input: &[u8]) {
        let plan = self.plan(regex);
        assert!(!self.finished, "chunk on a finished stream: reset it first");
        if self.matched || input.is_empty() {
            return;
        }
        scratch.prepare(plan.words);
        let w = plan.words;
        for &b in input {
            if self.close(plan, scratch, Boundary::BeforeByte(b)) {
                return;
            }
            consume(
                plan,
                regex,
                &scratch.closure[..w],
                &mut scratch.next[..w],
                b,
            );
            self.raw.copy_from_slice(&scratch.next[..w]);
            self.prev = Some(b);
            self.at_start = false;
        }
        self.close(plan, scratch, Boundary::UnknownNext);
    }

    /// Signal end of input and return the final result.
    pub fn finish(&mut self, regex: &Regex, scratch: &mut GroupedScratch) -> bool {
        let plan = self.plan(regex);
        assert!(
            !self.finished,
            "finish on a finished stream: reset it first"
        );
        self.finished = true;
        if !self.matched {
            scratch.prepare(plan.words);
            self.close(plan, scratch, Boundary::EndOfInput);
        }
        self.matched
    }

    /// Whether a match has been found so far.
    pub fn ismatch(&self) -> bool {
        self.matched
    }

    /// Bytes owned by this stream.
    pub fn memory_bytes(&self) -> usize {
        size_of::<Self>() + self.raw.len() * size_of::<u64>()
    }

    /// A displayable view of the raw rows.
    pub fn display<'a>(&'a self, regex: &'a Regex) -> impl fmt::Display + 'a {
        DisplayStream {
            stream: self,
            plan: self.plan(regex),
        }
    }

    /// The plan, checking that `regex` is the one this stream was made for.
    fn plan<'r>(&self, regex: &'r Regex) -> &'r GroupedNfaPlan {
        assert_eq!(
            regex.id, self.regex_id,
            "grouped stream used with a different regex"
        );
        regex
            .grouped_plan()
            .expect("the regex of a stream has a plan")
    }

    /// The closure at stream start: makes `ismatch` right before any input.
    fn start(&mut self, plan: &GroupedNfaPlan, scratch: &mut GroupedScratch) {
        scratch.prepare(plan.words);
        self.close(plan, scratch, Boundary::UnknownNext);
    }

    /// Close the raw rows into scratch; record a match.  The raw rows are
    /// not modified.
    fn close(
        &mut self,
        plan: &GroupedNfaPlan,
        scratch: &mut GroupedScratch,
        boundary: Boundary,
    ) -> bool {
        let ctx = BoundaryCtx {
            at_start: self.at_start,
            prev: self.prev,
            boundary,
        };
        let w = plan.words;
        self.matched |= close(plan, &self.raw, &mut scratch.closure[..w], ctx);
        self.matched
    }

    /// The raw rows (for tests).
    #[cfg(test)]
    pub(crate) fn raw(&self) -> &[u64] {
        &self.raw
    }
}

struct DisplayStream<'a> {
    stream: &'a GroupedStream,
    plan: &'a GroupedNfaPlan,
}

impl fmt::Display for DisplayStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.stream;
        write!(f, "grouped words={} matched={}", self.plan.words, s.matched)?;
        if s.at_start {
            write!(f, " at_start")?;
        }
        if let Some(b) = s.prev {
            write!(f, " prev={b:#04x}")?;
        }
        if s.finished {
            write!(f, " finished")?;
        }
        write!(f, " raw={{")?;
        let mut first = true;
        for &(state, scope) in self.plan.scopes.iter() {
            let row = self.plan.row_of[state.idx()];
            if is_empty(&s.raw, row) {
                continue;
            }
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            match scope {
                super::Scope::Outside => write!(f, "{state}")?,
                super::Scope::Counter(c) => {
                    let counts: Vec<String> = (0..row.words as usize * 64)
                        .filter(|&v| s.raw[row.off as usize + v / 64] >> (v % 64) & 1 != 0)
                        .map(|v| v.to_string())
                        .collect();
                    write!(f, "{state}:c{c}={{{}}}", counts.join(","))?;
                }
            }
        }
        write!(f, "}}")
    }
}
