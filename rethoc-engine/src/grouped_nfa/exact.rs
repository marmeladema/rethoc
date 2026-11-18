//! Exact grouped transfers and the streaming runner (plan §4–5).
//!
//! # Active rows
//!
//! A frontier (the stream's raw rows, or the closure arena of the scratch)
//! comes with a bitmap of its nonempty rows, one bit per node.  Invariant:
//! a row whose bit is clear is all zero, and a set bit means a nonempty
//! row.  The scratch is all zero between calls.
//!
//! A byte is closed from the raw rows into the scratch; the closure holds
//! a copy of every raw row, so consumption clears the raw rows and writes
//! the successors straight into them.
//!
//! Each byte runs one of two equivalent passes, chosen by [`Sweep`]:
//!
//! - **Active rows** touch only the active rows.  The closure takes the
//!   lowest pending node each time; a transfer only activates higher nodes
//!   (edges of the zero-width graph go up the topological order), so nodes
//!   are processed in increasing order and each after all its
//!   predecessors, exactly like the full sweep.  It costs the active rows
//!   plus a scan of the bitmaps (one word per 64 nodes).
//! - **Full sweep** walks every node in order and copies whole arenas.
//!   With most rows active it is cheaper than the bookkeeping above.
//!
//! The automatic choice takes the full sweep for plans of at most 32 rows
//! and when at least an eighth of the raw rows are active (measured with
//! the counting probe: below that, on wide plans, the active rows win by
//! up to four times; above it, the full sweep wins by up to two).  While
//! no row is active, the stream skips bytes with the prefilter when the
//! plan allows it.

use std::fmt;

use super::{GroupedNfaError, GroupedNfaPlan, Increment, NO_NODE, NodeIdx, Op, Row};
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
// Rows and active bitmaps
// ---------------------------------------------------------------------------

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
    if src.words == 1 {
        to[dst.off as usize] |= from[src.off as usize];
        return;
    }
    for (d, s) in to[dst.range()].iter_mut().zip(&from[src.range()]) {
        *d |= *s;
    }
}

/// `rows[r] = 0`.  Most rows are one word: a store, not a `memset` call.
#[inline]
fn zero_row(rows: &mut [u64], r: Row) {
    if r.words == 1 {
        rows[r.off as usize] = 0;
    } else {
        rows[r.range()].fill(0);
    }
}

/// `to[r] = from[r]`, a store for one-word rows.
#[inline]
fn copy_row(from: &[u64], to: &mut [u64], r: Row) {
    if r.words == 1 {
        to[r.off as usize] = from[r.off as usize];
    } else {
        to[r.range()].copy_from_slice(&from[r.range()]);
    }
}

#[inline]
fn activate(active: &mut [u64], n: NodeIdx) {
    active[n as usize / 64] |= 1 << (n % 64);
}

/// Call `f` for every active node, in increasing order.
#[inline]
fn for_each_active(active: &[u64], mut f: impl FnMut(NodeIdx)) {
    for (w, &word) in active.iter().enumerate() {
        let mut bits = word;
        while bits != 0 {
            f((w * 64) as NodeIdx + bits.trailing_zeros());
            bits &= bits - 1;
        }
    }
}

/// Zero the active rows and the bitmap.
pub(crate) fn clear(plan: &GroupedNfaPlan, rows: &mut [u64], active: &mut [u64]) {
    for_each_active(active, |n| zero_row(rows, plan.row(n)));
    active.fill(0);
}

#[inline]
fn is_empty_row(rows: &[u64], r: Row) -> bool {
    if r.words == 1 {
        rows[r.off as usize] == 0
    } else {
        rows[r.range()].iter().all(|&w| w == 0)
    }
}

#[inline]
fn is_clean(rows: &[u64], active: &[u64]) -> bool {
    rows.iter().chain(active).all(|&w| w == 0)
}

/// The bits of word `i` (counts `64 i ..= 64 i + 63`) inside `lo..=hi`.
#[inline]
fn word_mask(i: usize, lo: usize, hi: usize) -> u64 {
    let base = i * 64;
    if hi < lo || hi < base || lo > base + 63 {
        return 0;
    }
    let a = lo.saturating_sub(base);
    let b = (hi - base).min(63);
    (u64::MAX >> (63 - b)) & (u64::MAX << a)
}

/// Is count `v` in the row at `off`?
#[inline]
fn has(rows: &[u64], off: usize, v: usize) -> bool {
    rows[off + v / 64] >> (v % 64) & 1 != 0
}

/// Increment the count set `S = rows[src]` pointwise by `counter_step`:
/// OR the incremented counts that continue into `rows[out]`, and return
/// whether any continued and whether any breaks.  `src` and `out` are
/// distinct rows of one counter.
///
/// - Finite `{min,max}`: `T = S << 1` with carries between words.  `S`
///   lies below `max`, since only the continue edge returns into the body,
///   so `T` lies in `1..=max`: continue with `T ∩ 0..max`, break iff
///   `T ∩ min..=max` is nonempty.
/// - Saturating `{min,}`: `T = (S << 1) ∩ 0..=min`, plus `min` itself if
///   `min ∈ S` (it stays at `min`).  All of `T` continues; break iff
///   `min ∈ T`.  `min = 0` works: `T = {0}` whenever `S` is nonempty.
#[inline]
pub(crate) fn increment(rows: &mut [u64], src: Row, out: Row, inc: Increment) -> (bool, bool) {
    debug_assert_eq!(src.words, out.words, "increment across scopes");
    debug_assert_ne!(src.off, out.off, "increment into its own row");
    let (s0, o0, w) = (src.off as usize, out.off as usize, src.words as usize);
    if w == 1 {
        return increment_word(rows, s0, o0, inc);
    }
    let mut carry = 0u64;
    match inc {
        Increment::Finite { min, max } => {
            let (min, max) = (min as usize, max as usize);
            assert!(
                rows[s0 + max / 64] >> (max % 64) == 0,
                "a count reached its maximum before the increment"
            );
            let (mut cont, mut brk) = (false, false);
            for i in 0..w {
                let s = rows[s0 + i];
                let t = (s << 1) | carry;
                carry = s >> 63;
                brk |= t & word_mask(i, min, max) != 0;
                let c = t & word_mask(i, 0, max - 1);
                cont |= c != 0;
                rows[o0 + i] |= c;
            }
            debug_assert_eq!(carry, 0, "counts below max fit the row");
            (cont, brk)
        }
        Increment::Saturating { min } => {
            let min = min as usize;
            let stays = has(rows, s0, min);
            let mut cont = false;
            for i in 0..w {
                let s = rows[s0 + i];
                let t = (s << 1) | carry;
                carry = s >> 63;
                let c = t & word_mask(i, 0, min);
                cont |= c != 0;
                rows[o0 + i] |= c;
            }
            let reaches = min > 0 && has(rows, s0, min - 1);
            if stays || reaches {
                rows[o0 + min / 64] |= 1 << (min % 64);
            }
            (cont || stays || reaches, stays || reaches)
        }
    }
}

/// [`increment`] of a one-word row (domain top at most 63).
#[inline]
fn increment_word(rows: &mut [u64], s0: usize, o0: usize, inc: Increment) -> (bool, bool) {
    let s = rows[s0];
    match inc {
        Increment::Finite { min, max } => {
            assert!(
                s >> max == 0,
                "a count reached its maximum before the increment"
            );
            let t = s << 1;
            let cont = t & !(u64::MAX << max); // 0..max
            rows[o0] |= cont;
            (cont != 0, t >> min != 0) // T lies in 1..=max
        }
        Increment::Saturating { min } => {
            let mut t = (s << 1) & (u64::MAX >> (63 - min)); // 0..=min
            if s >> min & 1 != 0 {
                t |= 1 << min;
            }
            rows[o0] |= t;
            (t != 0, t >> min & 1 != 0)
        }
    }
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

/// Close the raw frontier into `closure`/`active` (clean on entry): copy
/// the active raw rows, insert Outside at the start (search reseeding),
/// then apply the zero-width transfers of the active nodes in increasing
/// order.  Returns whether `Match` was reached.  The caller cleans the
/// closure afterwards ([`clear`]).
///
/// All raw rows are seeded before the pass, so a node's incoming
/// contributions are complete when it is processed.
pub(crate) fn close(
    plan: &GroupedNfaPlan,
    raw: &[u64],
    raw_active: &[u64],
    closure: &mut [u64],
    active: &mut [u64],
    ctx: BoundaryCtx,
) -> bool {
    debug_assert!(is_clean(closure, active), "closure scratch is not clean");
    for_each_active(raw_active, |n| copy_row(raw, closure, plan.row(n)));
    active.copy_from_slice(raw_active);
    closure[plan.row(plan.start).off as usize] |= 1;
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
            let n = (w * 64) as NodeIdx + bit.trailing_zeros();
            let node = &plan.nodes[n as usize];
            matched |= transfer(plan, closure, node, ctx, |out| activate(active, out));
        }
    }
    matched
}

/// [`close`] by a full sweep: copy the whole raw arena and visit every
/// node in order, skipping empty rows.  Leaves the closure bitmap alone
/// (clean); the caller cleans the closure arena with one fill.
pub(crate) fn close_full(
    plan: &GroupedNfaPlan,
    raw: &[u64],
    closure: &mut [u64],
    ctx: BoundaryCtx,
) -> bool {
    closure.copy_from_slice(raw);
    closure[plan.row(plan.start).off as usize] |= 1;
    let mut matched = false;
    for &n in plan.zero_width.iter() {
        let node = &plan.nodes[n as usize];
        if is_empty_row(closure, node.row) {
            continue;
        }
        matched |= transfer(plan, closure, node, ctx, |_| {});
    }
    matched
}

/// Apply the zero-width transfer of `node` (whose row is nonempty) within
/// `closure`, calling `on` for every successor that received counts.
/// Returns whether `node` is a reached `Match`.
#[inline(always)]
fn transfer(
    plan: &GroupedNfaPlan,
    closure: &mut [u64],
    node: &super::Node,
    ctx: BoundaryCtx,
    mut on: impl FnMut(NodeIdx),
) -> bool {
    let src = node.row;
    match node.op {
        Op::Split { out, out1 } => {
            union_within(closure, src, plan.row(out));
            on(out);
            union_within(closure, src, plan.row(out1));
            on(out1);
        }
        Op::CounterInstance { out } => {
            closure[plan.row(out).off as usize] |= 1;
            on(out);
        }
        Op::CounterIncrement { out, out1, inc } => {
            let (cont, brk) = increment(closure, src, plan.row(out), inc);
            if cont {
                on(out);
            }
            if brk {
                closure[plan.row(out1).off as usize] |= 1;
                on(out1);
            }
        }
        Op::Assert { kind, out } => {
            if ctx.eval(kind) == AssertEval::Pass {
                union_within(closure, src, plan.row(out));
                on(out);
            }
        }
        Op::Match => return true,
        Op::Edge { .. } | Op::Table(_) => {}
    }
    false
}

/// Consume `b` from the closed rows into `next`/`next_active` (clean on
/// entry; the stream passes its raw rows, cleared): every active consuming
/// node tests its predicate once and moves its whole row.  Cleans the
/// closure on the way (each row once read).  Returns the number of
/// predicate tests.
pub(crate) fn consume(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &mut [u64],
    active: &mut [u64],
    next: &mut [u64],
    next_active: &mut [u64],
    b: u8,
) -> usize {
    debug_assert!(is_clean(next, next_active), "next scratch is not clean");
    let mut tests = 0;
    for_each_active(active, |n| {
        let node = &plan.nodes[n as usize];
        tests += usize::from(consume_node(
            plan,
            regex,
            closure,
            node,
            next,
            next_active,
            b,
        ));
        zero_row(closure, node.row);
    });
    active.fill(0);
    tests
}

/// [`consume`] by a full sweep over the consuming nodes; cleans the
/// closure arena with one fill.
pub(crate) fn consume_full(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &mut [u64],
    next: &mut [u64],
    next_active: &mut [u64],
    b: u8,
) -> usize {
    debug_assert!(is_clean(next, next_active), "next scratch is not clean");
    let mut tests = 0;
    for &n in plan.consumers.iter() {
        let node = &plan.nodes[n as usize];
        if !is_empty_row(closure, node.row) {
            tests += usize::from(consume_node(
                plan,
                regex,
                closure,
                node,
                next,
                next_active,
                b,
            ));
        }
    }
    closure.fill(0);
    tests
}

/// Consume `b` at `node` (nonempty row) if it is a consuming node: move its
/// row to the successors that accept `b`.  Returns whether it tested.
#[inline(always)]
fn consume_node(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &[u64],
    node: &super::Node,
    next: &mut [u64],
    next_active: &mut [u64],
    b: u8,
) -> bool {
    match node.op {
        Op::Edge { out, out_exit } => {
            if accepts(regex, &regex.states[node.state], b) {
                union_into(closure, node.row, next, plan.row(out));
                activate(next_active, out);
                if out_exit != NO_NODE {
                    union_into(closure, node.row, next, plan.row(out_exit));
                    activate(next_active, out_exit);
                }
            }
            true
        }
        Op::Table(table) => {
            let target = regex.byte_tables[table][b];
            if target != StateIdx::NONE {
                let out = plan.node_of[target.idx()];
                union_into(closure, node.row, next, plan.row(out));
                activate(next_active, out);
            }
            true
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Worker scratch
// ---------------------------------------------------------------------------

/// Per-worker scratch: the closure arena and its active bitmap.  Borrowed
/// only during a call, so streams of different regexes can share one, one
/// call at a time.  Capacity grows to the largest plan seen and is kept;
/// the arena is all zero between calls.
#[derive(Debug, Default)]
pub struct GroupedScratch {
    closure: Vec<u64>,
    closure_active: Vec<u64>,
    sweep: Sweep,
    /// Bytes stepped by the full sweep and by the active rows (for tests).
    #[cfg(test)]
    pub(crate) passes: [usize; 2],
}

/// Plans of at most this many rows always take the full sweep: it costs
/// little more than the active-row bookkeeping would.
const SMALL_PLAN_ROWS: usize = 32;

/// Which pass a byte runs (see the module documentation).  Both give the
/// same rows; tests and the counting probe force each.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Sweep {
    /// Full sweep for small plans and when at least an eighth of the raw
    /// rows are active; active rows otherwise.
    #[default]
    Auto,
    Full,
    Active,
}

impl GroupedScratch {
    /// Empty scratch; it grows on first use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Make room for `plan`, before any scanning.
    fn prepare(&mut self, plan: &GroupedNfaPlan) {
        fn grow(v: &mut Vec<u64>, len: usize) {
            if v.len() < len {
                v.reserve_exact(len - v.len());
                v.resize(len, 0);
            }
        }
        grow(&mut self.closure, plan.words());
        grow(&mut self.closure_active, plan.active_words());
    }

    /// Bytes held by the arena and bitmap (their high-water capacity).
    pub fn memory_bytes(&self) -> usize {
        (self.closure.capacity() + self.closure_active.capacity()) * size_of::<u64>()
    }

    /// Force a pass, for tests and measurements; [`Sweep::Auto`] is the
    /// default.
    #[doc(hidden)]
    pub fn set_sweep(&mut self, sweep: Sweep) {
        self.sweep = sweep;
    }

    /// Whether every arena and bitmap is all zero (for tests).
    #[cfg(test)]
    pub(crate) fn is_clean(&self) -> bool {
        is_clean(&self.closure, &self.closure_active)
    }
}

// ---------------------------------------------------------------------------
// Stream
// ---------------------------------------------------------------------------

/// One input stream on the grouped NFA of one regex.
///
/// Holds the raw rows and their active bitmap (fixed size: exactly the
/// plan's words, whatever the scratch's capacity), the previous byte, the
/// at-start flag and the sticky match flag.  Every call takes the regex it
/// was created for and a scratch.
///
/// While no row is active, bytes that the regex's prefilter rules out are
/// skipped when the plan allows it (`skip_when_idle`).
///
/// Lifecycle: [`new`](Self::new), any number of [`chunk`](Self::chunk)
/// calls, [`finish`](Self::finish); [`reset`](Self::reset) starts over.
/// Feeding or finishing a finished stream panics, as does passing a
/// different regex.
#[derive(Debug)]
pub struct GroupedStream {
    regex_id: u64,
    raw: Box<[u64]>,
    raw_active: Box<[u64]>,
    /// Active raw rows: none means idle.
    live: u32,
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
            raw: vec![0; plan.words()].into_boxed_slice(),
            raw_active: vec![0; plan.active_words()].into_boxed_slice(),
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
    pub fn reset(&mut self, regex: &Regex, scratch: &mut GroupedScratch) {
        let plan = self.plan(regex);
        clear(plan, &mut self.raw, &mut self.raw_active);
        self.live = 0;
        self.prev = None;
        self.at_start = true;
        self.matched = false;
        self.finished = false;
        scratch.prepare(plan);
        self.provisional(plan, scratch, Boundary::UnknownNext);
    }

    /// Feed the next bytes of the input.
    pub fn chunk(&mut self, regex: &Regex, scratch: &mut GroupedScratch, input: &[u8]) {
        let plan = self.plan(regex);
        assert!(!self.finished, "chunk on a finished stream: reset it first");
        if self.matched || input.is_empty() {
            return;
        }
        scratch.prepare(plan);
        let prefilter = &regex.prefilter;
        let skip = plan.skip_when_idle && !matches!(prefilter, crate::Prefilter::None);
        let mut i = 0;
        while i < input.len() {
            if skip && self.live == 0 {
                // The closure of an empty frontier is the start closure,
                // the same at every boundary, without a match; a byte the
                // prefilter rules out leaves the frontier empty.
                match prefilter.find(&input[i..]) {
                    Some(0) => {}
                    Some(off) => {
                        i += off;
                        self.prev = Some(input[i - 1]);
                        self.at_start = false;
                    }
                    None => {
                        self.prev = input.last().copied();
                        self.at_start = false;
                        return;
                    }
                }
            }
            if self.step(plan, regex, scratch, input[i]) {
                return;
            }
            i += 1;
        }
        self.provisional(plan, scratch, Boundary::UnknownNext);
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
            scratch.prepare(plan);
            self.provisional(plan, scratch, Boundary::EndOfInput);
        }
        self.matched
    }

    /// Whether a match has been found so far.
    pub fn ismatch(&self) -> bool {
        self.matched
    }

    /// Bytes owned by this stream ([`GroupedNfaPlan::stream_bytes`]).
    pub fn memory_bytes(&self) -> usize {
        size_of::<Self>() + (self.raw.len() + self.raw_active.len()) * size_of::<u64>()
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

    fn ctx(&self, boundary: Boundary) -> BoundaryCtx {
        BoundaryCtx {
            at_start: self.at_start,
            prev: self.prev,
            boundary,
        }
    }

    /// A closure that only looks for a match: the raw rows stay as they
    /// are, and the scratch is cleaned.
    fn provisional(
        &mut self,
        plan: &GroupedNfaPlan,
        scratch: &mut GroupedScratch,
        boundary: Boundary,
    ) {
        let (w, a) = (plan.words(), plan.active_words());
        let ctx = self.ctx(boundary);
        let full = self.full_sweep(plan, scratch.sweep);
        let (closure, active) = (&mut scratch.closure[..w], &mut scratch.closure_active[..a]);
        if full {
            self.matched |= close_full(plan, &self.raw, closure, ctx);
            closure.fill(0);
        } else {
            self.matched |= close(plan, &self.raw, &self.raw_active, closure, active, ctx);
            clear(plan, closure, active);
        }
    }

    /// Whether this byte runs the full sweep.
    #[inline]
    fn full_sweep(&self, plan: &GroupedNfaPlan, sweep: Sweep) -> bool {
        match sweep {
            Sweep::Full => true,
            Sweep::Active => false,
            Sweep::Auto => plan.rows() <= SMALL_PLAN_ROWS || self.live as usize * 8 >= plan.rows(),
        }
    }

    /// Close before `b`, then consume it: the raw rows are replaced by the
    /// consuming successors (the closure holds their old contents).
    /// Returns whether a match was found first.
    fn step(
        &mut self,
        plan: &GroupedNfaPlan,
        regex: &Regex,
        scratch: &mut GroupedScratch,
        b: u8,
    ) -> bool {
        let (w, a) = (plan.words(), plan.active_words());
        let ctx = self.ctx(Boundary::BeforeByte(b));
        let full = self.full_sweep(plan, scratch.sweep);
        #[cfg(test)]
        {
            scratch.passes[usize::from(!full)] += 1;
        }
        let (closure, active) = (&mut scratch.closure[..w], &mut scratch.closure_active[..a]);
        let (raw, raw_active) = (&mut *self.raw, &mut *self.raw_active);
        if full {
            if close_full(plan, raw, closure, ctx) {
                closure.fill(0);
                self.matched = true;
                return true;
            }
            raw.fill(0);
            raw_active.fill(0);
            consume_full(plan, regex, closure, raw, raw_active, b);
        } else {
            if close(plan, raw, raw_active, closure, active, ctx) {
                clear(plan, closure, active);
                self.matched = true;
                return true;
            }
            clear(plan, raw, raw_active);
            consume(plan, regex, closure, active, raw, raw_active, b);
        }
        self.live = raw_active.iter().map(|w| w.count_ones()).sum();
        self.prev = Some(b);
        self.at_start = false;
        false
    }

    /// The raw rows (for tests).
    #[cfg(test)]
    pub(crate) fn raw(&self) -> &[u64] {
        &self.raw
    }

    /// The active bitmap of the raw rows (for tests).
    #[cfg(test)]
    pub(crate) fn raw_active(&self) -> &[u64] {
        &self.raw_active
    }

    /// The boundary metadata: previous byte and at-start flag (for tests).
    #[cfg(test)]
    pub(crate) fn boundary_state(&self) -> (Option<u8>, bool) {
        (self.prev, self.at_start)
    }
}

impl GroupedNfaPlan {
    /// Bytes of one stream of this plan ([`GroupedStream::memory_bytes`]).
    pub(crate) fn stream_bytes(&self) -> usize {
        size_of::<GroupedStream>() + (self.words() + self.active_words()) * size_of::<u64>()
    }

    /// Bytes of a scratch prepared for this plan alone
    /// ([`GroupedScratch::memory_bytes`]).
    pub(crate) fn scratch_bytes(&self) -> usize {
        (self.words() + self.active_words()) * size_of::<u64>()
    }
}

struct DisplayStream<'a> {
    stream: &'a GroupedStream,
    plan: &'a GroupedNfaPlan,
}

impl fmt::Display for DisplayStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.stream;
        write!(
            f,
            "grouped words={} matched={}",
            self.plan.words(),
            s.matched
        )?;
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
        for_each_active(&s.raw_active, |n| {
            let node = &self.plan.nodes[n as usize];
            let row = node.row;
            parts.push(match node.scope {
                super::Scope::Outside => node.state.to_string(),
                super::Scope::Counter(c) => {
                    let counts: Vec<String> = (0..row.words as usize * 64)
                        .filter(|&v| s.raw[row.off as usize + v / 64] >> (v % 64) & 1 != 0)
                        .map(|v| v.to_string())
                        .collect();
                    format!("{}:c{c}={{{}}}", node.state, counts.join(","))
                }
            });
        });
        write!(f, " raw={{{}}}", parts.join(", "))
    }
}
