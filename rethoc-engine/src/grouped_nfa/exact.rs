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
//! Under the automatic choice, a frontier where no counter has counted yet
//! takes the count-free step instead, by compile-time bitmaps
//! ([`CountFreeTables`](super::CountFreeTables)).
//!
//! The automatic choice takes the full sweep for plans of at most 32 rows
//! and when at least an eighth of the raw rows are active (measured with
//! the counting probe: below that, on wide plans, the active rows win by
//! up to four times; above it, the full sweep wins by up to two).  While
//! no row is active, the stream skips bytes with the prefilter when the
//! plan allows it.

use std::fmt;

use super::bitmap::{self, activate, for_each_active};
use super::stats::stat;
use super::{GroupedNfaError, GroupedNfaPlan, Increment, NodeIdx, Op, Row, Succ};
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
    pub(crate) fn eval(self, kind: crate::AssertKind) -> AssertEval {
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
    stat!(words_unioned += src.words);
    stat!(fanout += 1);
    for i in 0..src.words as usize {
        rows[dst.off as usize + i] |= rows[src.off as usize + i];
    }
}

/// `to[dst] |= from[src]` across arenas.
#[inline]
fn union_into(from: &[u64], src: Row, to: &mut [u64], dst: Row) {
    debug_assert_eq!(src.words, dst.words, "union of rows of different scopes");
    stat!(words_unioned += src.words);
    stat!(fanout += 1);
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
    stat!(words_cleared += r.words);
    if r.words == 1 {
        rows[r.off as usize] = 0;
    } else {
        rows[r.range()].fill(0);
    }
}

/// `to[r] = from[r]`, a store for one-word rows.
#[inline]
fn copy_row(from: &[u64], to: &mut [u64], r: Row) {
    stat!(words_copied += r.words);
    if r.words == 1 {
        to[r.off as usize] = from[r.off as usize];
    } else {
        to[r.range()].copy_from_slice(&from[r.range()]);
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
#[inline(always)]
pub(crate) fn increment(rows: &mut [u64], src: Row, out: Row, inc: Increment) -> (bool, bool) {
    debug_assert_eq!(src.words, out.words, "increment across scopes");
    debug_assert_ne!(src.off, out.off, "increment into its own row");
    let (s0, o0, w) = (src.off as usize, out.off as usize, src.words as usize);
    stat!(words_incremented += w);
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
pub(crate) fn accepts(classes: &[crate::ByteClassBits], state: &State, b: u8) -> bool {
    match *state {
        State::Byte { byte, .. } => b == byte,
        State::ByteCI { byte, .. } => byte_match_ci(b, byte),
        State::Wildcard { .. } => true,
        State::ByteClassStatic { table, .. } => table[b as usize],
        State::ByteClassCustom { class, .. } => classes[class.idx()].contains(b),
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
            matched |= transfer(closure, node, ctx, |out| activate(active, out));
        }
    }
    matched
}

/// [`close`] by a full sweep: copy the whole raw arena and visit every
/// node in order, skipping empty rows.  Leaves the closure bitmap alone
/// (clean); the caller cleans the closure arena with one fill.
#[inline(always)]
pub(crate) fn close_full(
    plan: &GroupedNfaPlan,
    raw: &[u64],
    closure: &mut [u64],
    ctx: BoundaryCtx,
) -> bool {
    stat!(words_copied += raw.len());
    closure.copy_from_slice(raw);
    closure[plan.row(plan.start).off as usize] |= 1;
    let mut matched = false;
    for node in plan.zero_width.iter() {
        if is_empty_row(closure, node.row) {
            continue;
        }
        matched |= transfer(closure, node, ctx, |_| {});
    }
    matched
}

/// Apply the zero-width transfer of `node` (whose row is nonempty) within
/// `closure`, calling `on` for every successor that received counts.
/// Returns whether `node` is a reached `Match`.
#[inline(always)]
fn transfer(
    closure: &mut [u64],
    node: &super::Node,
    ctx: BoundaryCtx,
    mut on: impl FnMut(NodeIdx),
) -> bool {
    let src = node.row;
    stat!(rows_closed += 1);
    match node.op {
        Op::Split { out, out1 } => {
            union_within(closure, src, out.row(src.words));
            on(out.node);
            union_within(closure, src, out1.row(src.words));
            on(out1.node);
        }
        Op::CounterInstance { out } => {
            stat!(fanout += 1);
            closure[out.off as usize] |= 1;
            on(out.node);
        }
        Op::CounterIncrement { out, out1, inc } => {
            let (cont, brk) = increment(closure, src, out.row(src.words), inc);
            if cont {
                stat!(fanout += 1);
                on(out.node);
            }
            if brk {
                stat!(fanout += 1);
                closure[out1.off as usize] |= 1;
                on(out1.node);
            }
        }
        Op::Assert { kind, out } => {
            if ctx.eval(kind) == AssertEval::Pass {
                union_within(closure, src, out.row(src.words));
                on(out.node);
            }
        }
        Op::Match => return true,
        Op::Edge { .. } | Op::Table(_) => {}
    }
    false
}

/// What a consumption pass did.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Consumed {
    /// Predicate tests.
    pub(crate) tests: usize,
    /// Whether every next row is exactly bit 0 (a unit frontier): every
    /// consumer that moved a row moved exactly bit 0.
    pub(crate) unit: bool,
}

/// Consume `b` from the closed rows into `next`/`next_active` (clean on
/// entry; the stream passes its raw rows, cleared): every active consuming
/// node tests its predicate once and moves its whole row.  Cleans the
/// closure on the way (each row once read).
pub(crate) fn consume(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &mut [u64],
    active: &mut [u64],
    next: &mut [u64],
    next_active: &mut [u64],
    b: u8,
) -> Consumed {
    debug_assert!(is_clean(next, next_active), "next scratch is not clean");
    let mut done = Consumed {
        tests: 0,
        unit: true,
    };
    for_each_active(active, |n| {
        let node = &plan.nodes[n as usize];
        consume_node(plan, regex, closure, node, next, next_active, b, &mut done);
        zero_row(closure, node.row);
    });
    active.fill(0);
    stat!(tests += done.tests);
    done
}

/// [`consume`] by a full sweep over the consuming nodes; cleans the
/// closure arena with one fill.
#[inline(always)]
pub(crate) fn consume_full(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &mut [u64],
    next: &mut [u64],
    next_active: &mut [u64],
    b: u8,
) -> Consumed {
    debug_assert!(is_clean(next, next_active), "next scratch is not clean");
    let mut done = Consumed {
        tests: 0,
        unit: true,
    };
    for node in plan.consumers.iter() {
        if !is_empty_row(closure, node.row) {
            consume_node(plan, regex, closure, node, next, next_active, b, &mut done);
        }
    }
    stat!(words_cleared += closure.len());
    closure.fill(0);
    stat!(tests += done.tests);
    done
}

/// Consume `b` at `node` if it is a consuming node: move its row to the
/// successors that accept `b`, counting the test and noting whether the
/// moved row is exactly bit 0.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn consume_node(
    plan: &GroupedNfaPlan,
    regex: &Regex,
    closure: &[u64],
    node: &super::Node,
    next: &mut [u64],
    next_active: &mut [u64],
    b: u8,
    done: &mut Consumed,
) {
    match node.op {
        Op::Edge { out, out_exit } => {
            done.tests += 1;
            if accepts(&regex.classes, &regex.states[node.state], b) {
                done.unit = done.unit && is_unit_row(closure, node.row);
                let w = node.row.words;
                union_into(closure, node.row, next, out.row(w));
                activate(next_active, out.node);
                if out_exit != Succ::NONE {
                    union_into(closure, node.row, next, out_exit.row(w));
                    activate(next_active, out_exit.node);
                }
            }
        }
        Op::Table(table) => {
            done.tests += 1;
            let target = regex.byte_tables[table][b];
            if target != StateIdx::NONE {
                done.unit = done.unit && is_unit_row(closure, node.row);
                let out = plan.node_of[target.idx()];
                union_into(closure, node.row, next, plan.row(out));
                activate(next_active, out);
            }
        }
        _ => {}
    }
}

/// Add the active rows of a raw frontier, and the counts of its counted
/// rows, to the operation counters.
#[cfg(feature = "op-stats")]
fn count_frontier(plan: &GroupedNfaPlan, raw: &[u64], raw_active: &[u64]) {
    for_each_active(raw_active, |n| {
        let node = &plan.nodes[n as usize];
        stat!(live_rows += 1);
        if node.scope == super::Scope::Outside {
            return;
        }
        let (mut values, mut runs, mut carry) = (0u32, 0u32, 0u64);
        for &word in &raw[node.row.range()] {
            values += word.count_ones();
            // A run starts at a count whose predecessor is absent.
            runs += (word & !(word << 1 | carry)).count_ones();
            carry = word >> 63;
        }
        stat!(counted_rows += 1);
        stat!(counted_words += node.row.words);
        stat!(count_values += values);
        stat!(count_runs += runs);
        stat!(max_row_values max= values);
    });
}

/// Whether row `r` holds exactly bit 0.
#[inline(always)]
fn is_unit_row(rows: &[u64], r: Row) -> bool {
    rows[r.off as usize] == 1
        && is_empty_row(
            rows,
            Row {
                off: r.off + 1,
                words: r.words - 1,
            },
        )
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
    /// Bytes stepped by the full sweep, by the active rows and by the
    /// count-free step (for tests).
    #[cfg(test)]
    pub(crate) passes: [usize; 3],
}

/// Run `$body` with the const `$n` bound to the count-free bitmap width
/// `$a` (1 to [`MAX_COUNT_FREE_WORDS`](super::analysis) words), so that
/// each width gets its own code with the bitmaps in registers.
macro_rules! by_width {
    ($a:expr, |$n:ident| $body:expr) => {
        match $a {
            1 => {
                const $n: usize = 1;
                $body
            }
            2 => {
                const $n: usize = 2;
                $body
            }
            3 => {
                const $n: usize = 3;
                $body
            }
            4 => {
                const $n: usize = 4;
                $body
            }
            5 => {
                const $n: usize = 5;
                $body
            }
            6 => {
                const $n: usize = 6;
                $body
            }
            7 => {
                const $n: usize = 7;
                $body
            }
            8 => {
                const $n: usize = 8;
                $body
            }
            a => unreachable!("count-free bitmaps of {a} words"),
        }
    };
}

pub(crate) use by_width;

/// Which pass a byte runs (see the module documentation).  Both give the
/// same rows; tests and the counting probe force each.  Forcing a pass also
/// turns off the count-free step, so that the general step can be tested
/// and measured against it.
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
    /// Exactly the plan's words long.  A `Vec` so that
    /// [`rebind`](Self::rebind) can reuse the allocation for another regex.
    raw: Vec<u64>,
    raw_active: Vec<u64>,
    /// Active raw rows: none means idle.
    live: u32,
    /// Every active raw row holds exactly bit 0 (kept exactly by every
    /// step).
    unit: bool,
    prev: Option<u8>,
    at_start: bool,
    matched: bool,
    finished: bool,
}

impl GroupedStream {
    /// A fresh stream for `regex`, whose grouped NFA must have been analysed
    /// and admitted.  Automatic selection analyses it only for regexes that
    /// Tier 1 and Tier 2 cannot run; compile with
    /// [`GroupedNfaMode::Always`](crate::GroupedNfaMode::Always) to have it
    /// for every regex.  Otherwise this returns
    /// [`GroupedNfaError::NotRequested`] (or `Rejected`).
    pub fn new(regex: &Regex, scratch: &mut GroupedScratch) -> Result<Self, GroupedNfaError> {
        let plan = regex.grouped_plan()?;
        let mut stream = Self {
            regex_id: regex.id,
            raw: vec![0; plan.words()],
            raw_active: vec![0; plan.active_words()],
            live: 0,
            unit: true,
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
        self.unit = true;
        self.prev = None;
        self.at_start = true;
        self.matched = false;
        self.finished = false;
        scratch.prepare(plan);
        self.provisional(plan, scratch, Boundary::UnknownNext);
    }

    /// Make this a fresh stream for `regex`, which may be another regex
    /// than the one it was made for, reusing the allocation when its
    /// capacity suffices.
    pub fn rebind(
        &mut self,
        regex: &Regex,
        scratch: &mut GroupedScratch,
    ) -> Result<(), GroupedNfaError> {
        let plan = regex.grouped_plan()?;
        if regex.id == self.regex_id {
            // Clears only the active rows.
            self.reset(regex, scratch);
            return Ok(());
        }
        self.regex_id = regex.id;
        self.raw.clear();
        self.raw.resize(plan.words(), 0);
        self.raw_active.clear();
        self.raw_active.resize(plan.active_words(), 0);
        self.live = 0;
        self.unit = true;
        self.prev = None;
        self.at_start = true;
        self.matched = false;
        self.finished = false;
        scratch.prepare(plan);
        self.provisional(plan, scratch, Boundary::UnknownNext);
        Ok(())
    }

    /// Feed the next bytes of the input.
    ///
    /// A small function: a stream that is idle for the whole chunk, the
    /// common case for many streams of many patterns, only skips here.
    /// The stepping loop is a function of its own, with the frame and the
    /// registers its steps need.
    pub fn chunk(&mut self, regex: &Regex, scratch: &mut GroupedScratch, input: &[u8]) {
        let plan = self.plan(regex);
        assert!(!self.finished, "chunk on a finished stream: reset it first");
        if self.matched || input.is_empty() {
            return;
        }
        // The count-free tables, which come with the skip, under the
        // automatic choice only: forcing a pass runs the general step on
        // every byte, which the tests compare with them.
        let tables = plan
            .count_free
            .as_deref()
            .filter(|_| scratch.sweep == Sweep::Auto);
        // Nothing is kept across these calls, so this function needs no
        // saved registers: a counting stream fed a byte at a time pays for
        // little more than a call here.
        match tables {
            Some(tables) if self.live == 0 => self.chunk_idle(regex, scratch, input, tables),
            _ => self.step_from(plan, regex, scratch, input, 0, tables),
        }
    }

    /// [`chunk`](Self::chunk) on an idle stream: skip to the first byte
    /// that starts something, and step from there if there is one.
    #[inline(never)]
    fn chunk_idle(
        &mut self,
        regex: &Regex,
        scratch: &mut GroupedScratch,
        input: &[u8],
        tables: &super::CountFreeTables,
    ) {
        let Some(next) = self.skip_idle(&regex.prefilter, &tables.starts, input, 0) else {
            return;
        };
        let plan = self.plan(regex);
        self.step_from(plan, regex, scratch, input, next, Some(tables));
    }

    /// The stepping loop of [`chunk`](Self::chunk), from `input[i]`.
    #[inline(never)]
    fn step_from(
        &mut self,
        plan: &GroupedNfaPlan,
        regex: &Regex,
        scratch: &mut GroupedScratch,
        input: &[u8],
        mut i: usize,
        tables: Option<&super::CountFreeTables>,
    ) {
        scratch.prepare(plan);
        let prefilter = &regex.prefilter;
        let has_prefilter = !matches!(prefilter, crate::Prefilter::None);
        // Without the count-free tables, the prefilter skips an idle
        // stream ahead of the step.
        let skip = tables.is_none() && plan.skip_when_idle && has_prefilter;
        // One call of `step` in this loop, so that it is inlined: called
        // from two places it was not, and a general step then paid for a
        // call and reloaded its state (a fifth more instructions).  A skip
        // leads straight into the count-free step, as it always has: any
        // test between them follows the search's data-dependent loop, and
        // costs branch mispredictions out of proportion to its size.
        while i < input.len() {
            if let Some(tables) = tables {
                if self.live == 0 {
                    // Returns at once on the first iteration: `chunk` has
                    // skipped to `i`.
                    let Some(next) = self.skip_idle(prefilter, &tables.starts, input, i) else {
                        return;
                    };
                    i = next;
                }
                // The count-free step applies to a unit frontier of
                // count-free nodes.  An empty one is both.
                if self.live == 0 || self.unit && bitmap::all_count_free(&self.raw_active, tables) {
                    let (next, found) = by_width!(tables.a, |A| self
                        .count_free_run::<A>(plan, tables, regex, input, i));
                    #[cfg(test)]
                    {
                        scratch.passes[2] += next - i + usize::from(found);
                    }
                    stat!(steps_count_free += next - i);
                    if found {
                        return;
                    }
                    i = next;
                    continue;
                }
            } else if skip && self.live == 0 {
                // The closure of an empty frontier is the start closure,
                // the same at every boundary, without a match; a byte the
                // prefilter rules out leaves the frontier empty.
                match prefilter.find(&input[i..]) {
                    Some(0) => {}
                    Some(off) => {
                        stat!(bytes_skipped += off);
                        i += off;
                        self.prev = Some(input[i - 1]);
                        self.at_start = false;
                    }
                    None => {
                        stat!(bytes_skipped += input.len() - i);
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
        if let Some(tables) = tables
            && self.unit
            && bitmap::all_count_free(&self.raw_active, tables)
        {
            self.matched |= by_width!(tables.a, |A| bitmap::matches::<A>(&self.raw_active, tables));
            return;
        }
        self.provisional(plan, scratch, Boundary::UnknownNext);
    }

    /// With an empty frontier: skip to the next byte at or after `i` that
    /// starts something (the others leave an empty frontier empty) and
    /// return its index, or `None` when the chunk has none.  Keeps the
    /// boundary: the previous byte is the last one skipped.
    #[inline(always)]
    fn skip_idle(
        &mut self,
        prefilter: &crate::Prefilter,
        starts: &[bool; 256],
        input: &[u8],
        i: usize,
    ) -> Option<usize> {
        let Some(next) = bitmap::next_start(prefilter, starts, input, i) else {
            stat!(bytes_skipped += input.len() - i);
            self.prev = input.last().copied();
            self.at_start = false;
            return None;
        };
        stat!(bytes_skipped += next - i);
        // The byte before `next` is not recorded: the step or the run that
        // follows starts from an empty frontier, whose closure reads no
        // boundary (the tables exist only when the start closure has no
        // assertion), and records its own byte.  Recording it here costs a
        // branch that the input decides.
        Some(next)
    }

    /// [`step`](Self::step) on a unit, count-free frontier, by bitmaps of
    /// `A` words ([`bitmap::run`]), from `input[i]` while the frontier
    /// stays nonempty and count-free (it stays unit).  The raw rows are
    /// written once, when the run ends.  Returns the next byte's index and
    /// whether a match was found (before the byte at that index).
    #[inline(always)]
    fn count_free_run<const A: usize>(
        &mut self,
        plan: &GroupedNfaPlan,
        fast: &super::CountFreeTables,
        regex: &Regex,
        input: &[u8],
        i: usize,
    ) -> (usize, bool) {
        debug_assert!(self.unit && bitmap::all_count_free(&self.raw_active, fast));
        let nodes: &[super::Node] = &plan.nodes;
        let node_of: &[NodeIdx] = &plan.node_of;
        let byte_tables = &regex.byte_tables;
        let run = bitmap::run::<A>(fast, &self.raw_active, input, i, |c, b, next| {
            match nodes[c].op {
                Op::Edge { out, out_exit } => {
                    bitmap::set(next, out.node);
                    if out_exit != Succ::NONE {
                        bitmap::set(next, out_exit.node);
                    }
                }
                Op::Table(t) => bitmap::set(next, node_of[byte_tables[t][b].idx()]),
                _ => unreachable!("only consumers accept bytes"),
            }
        });
        // A row of a unit frontier is its first word, 1 or 0, so only the
        // rows that changed are stored.
        let raw = &mut *self.raw;
        run.for_each_change(|n, on| raw[nodes[n].row.off as usize] = u64::from(on));
        self.raw_active[..A].copy_from_slice(&run.active);
        self.live = run.live();
        self.unit = true;
        if run.next > i {
            self.prev = Some(input[run.next - 1]);
            self.at_start = false;
        }
        if run.found {
            self.matched = true;
        }
        (run.next, run.found)
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

    /// Bytes owned by this stream: [`GroupedNfaPlan::stream_bytes`] for a
    /// new stream, more after [`rebind`](Self::rebind) to a smaller plan.
    pub fn memory_bytes(&self) -> usize {
        size_of::<Self>() + (self.raw.capacity() + self.raw_active.capacity()) * size_of::<u64>()
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
        stat!(provisional += 1);
        let (closure, active) = (&mut scratch.closure[..w], &mut scratch.closure_active[..a]);
        if full {
            self.matched |= close_full(plan, &self.raw, closure, ctx);
            stat!(words_cleared += closure.len());
            closure.fill(0);
        } else {
            self.matched |= close(plan, &self.raw, &self.raw_active, closure, active, ctx);
            clear(plan, closure, active);
        }
    }

    /// Whether this byte runs the full sweep.
    #[inline]
    fn full_sweep(&self, plan: &GroupedNfaPlan, sweep: Sweep) -> bool {
        bitmap::full_sweep(sweep, plan.rows(), self.live)
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
            stat!(steps_full += 1);
            if close_full(plan, raw, closure, ctx) {
                closure.fill(0);
                self.matched = true;
                return true;
            }
            stat!(words_cleared += raw.len());
            raw.fill(0);
            raw_active.fill(0);
            self.unit = consume_full(plan, regex, closure, raw, raw_active, b).unit;
        } else {
            stat!(steps_active += 1);
            if close(plan, raw, raw_active, closure, active, ctx) {
                clear(plan, closure, active);
                self.matched = true;
                return true;
            }
            clear(plan, raw, raw_active);
            self.unit = consume(plan, regex, closure, active, raw, raw_active, b).unit;
        }
        self.live = raw_active.iter().map(|w| w.count_ones()).sum();
        #[cfg(feature = "op-stats")]
        count_frontier(plan, raw, raw_active);
        self.prev = Some(b);
        self.at_start = false;
        false
    }

    /// The raw rows (for tests).
    #[cfg(test)]
    pub(crate) fn raw(&self) -> &[u64] {
        &self.raw
    }

    /// Whether every active raw row holds exactly bit 0 (for tests).
    #[cfg(test)]
    pub(crate) fn unit(&self) -> bool {
        self.unit
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

/// A grouped NFA stream with the regex and the scratch it runs with: what
/// [`AnyMatcher`](crate::AnyMatcher) holds for the grouped NFA.  Made by
/// [`MatcherMemory::matcher`](crate::MatcherMemory::matcher) and
/// [`matcher_grouped`](crate::MatcherMemory::matcher_grouped), which keep
/// the stream storage and scratch.
pub struct GroupedMatcher<'a> {
    pub(crate) regex: &'a Regex,
    pub(crate) stream: &'a mut GroupedStream,
    pub(crate) scratch: &'a mut GroupedScratch,
}

impl GroupedMatcher<'_> {
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

impl fmt::Display for GroupedMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stream.display(self.regex))
    }
}

impl fmt::Debug for GroupedMatcher<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GroupedMatcher({})", self.stream.display(self.regex))
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
