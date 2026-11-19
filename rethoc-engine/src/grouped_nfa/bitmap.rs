//! Bitmap routines shared by the exact grouped NFA ([`super::exact`]) and
//! its sparse counting variant ([`super::sparse`]): the active-row
//! bitmaps, the choice of a pass, the search for the next byte that starts
//! something, and the count-free step.
//!
//! None of them reads a row.  Both engines number their nodes alike and
//! build the same [`CountFreeTables`]; what differs is what a *unit* row
//! holds (exactly count zero, or exactly `{1}`), which only matters when a
//! count-free run ends and its rows are written back.  Keeping the logic
//! here keeps the two engines from drifting apart: the differential tests
//! compare each with the NFA, not with the other.

use super::exact::Sweep;
use super::{CountFreeTables, NodeIdx};
use crate::Prefilter;

/// Plans of at most this many rows always take the full sweep: it costs
/// little more than the active-row bookkeeping would.
pub(crate) const SMALL_PLAN_ROWS: usize = 32;

#[inline(always)]
pub(crate) fn activate(active: &mut [u64], n: NodeIdx) {
    active[n as usize / 64] |= 1 << (n % 64);
}

/// Call `f` for every active node, in increasing order.
#[inline(always)]
pub(crate) fn for_each_active(active: &[u64], mut f: impl FnMut(NodeIdx)) {
    for (w, &word) in active.iter().enumerate() {
        let mut bits = word;
        while bits != 0 {
            f((w * 64) as NodeIdx + bits.trailing_zeros());
            bits &= bits - 1;
        }
    }
}

/// Whether a byte runs the full sweep, for a plan of `rows` rows of which
/// `live` are active: always on a small plan, and when at least an eighth
/// of the rows are active.
#[inline]
pub(crate) fn full_sweep(sweep: Sweep, rows: usize, live: u32) -> bool {
    match sweep {
        Sweep::Full => true,
        Sweep::Active => false,
        Sweep::Auto => rows <= SMALL_PLAN_ROWS || live as usize * 8 >= rows,
    }
}

/// From an empty frontier whose start closure is the same at every
/// boundary: the index, at or after `from`, of the next byte of `input`
/// that one of the closure's consuming states accepts (`starts`), or
/// `None`.  The bytes before it leave the frontier empty.  The prefilter,
/// when there is one, finds the candidates.
#[inline(always)]
pub(crate) fn next_start(
    prefilter: &Prefilter,
    starts: &[bool; 256],
    input: &[u8],
    mut i: usize,
) -> Option<usize> {
    if matches!(prefilter, Prefilter::None) {
        return find_start(starts, &input[i..]).map(|off| i + off);
    }
    loop {
        // A candidate of the prefilter that starts nothing is skipped.
        let off = prefilter.find(&input[i..])?;
        if starts[input[i + off] as usize] {
            return Some(i + off);
        }
        i += off + 1;
        if i >= input.len() {
            return None;
        }
    }
}

/// The offset of the first byte of `hay` that `starts` accepts.
///
/// A function of its own, like the prefilter's search: it runs a byte at a
/// time over idle stretches, and its loop is six instructions only while
/// the input and the table stay in registers.  Inlined into a stream's
/// `chunk`, whose steps need many registers, it had the input pointer
/// spilled and reloaded on every byte (a sixth more instructions).  It is
/// called once per idle stretch.
#[inline(never)]
fn find_start(starts: &[bool; 256], hay: &[u8]) -> Option<usize> {
    hay.iter().position(|&b| starts[b as usize])
}

/// Whether every active node is count-free.
#[inline(always)]
pub(crate) fn all_count_free(active: &[u64], tables: &CountFreeTables) -> bool {
    active
        .iter()
        .zip(tables.statics.iter())
        .all(|(r, s)| r & !s == 0)
}

/// The closure bitmap of the unit, count-free frontier `active`, of `A`
/// words: the union of the closures of its nodes and of the start.
/// (`closure` and `start` from the tables, passed as locals so that they
/// stay in registers.)
#[inline(always)]
fn closure_of<const A: usize>(closure: &[u64], start: [u64; A], active: &[u64; A]) -> [u64; A] {
    let mut clos = start;
    for (i, &word) in active.iter().enumerate() {
        let mut bits = word;
        while bits != 0 {
            let n = i * 64 + bits.trailing_zeros() as usize;
            bits &= bits - 1;
            let c = &closure[n * A..][..A];
            for j in 0..A {
                clos[j] |= c[j];
            }
        }
    }
    clos
}

/// Whether the closure of the unit, count-free frontier `active` reaches
/// `Match`.  It meets no assertion, so the answer is the same whatever
/// comes next.
pub(crate) fn matches<const A: usize>(active: &[u64], tables: &CountFreeTables) -> bool {
    let active: [u64; A] = active[..A].try_into().unwrap();
    let start = tables.start_closure[..A].try_into().unwrap();
    let clos = closure_of::<A>(&tables.closure, start, &active);
    (0..A).any(|i| clos[i] & tables.matches[i] != 0)
}

/// Set the bit of node `n` in a bitmap of `A` words.
#[inline(always)]
pub(crate) fn set<const A: usize>(bits: &mut [u64; A], n: NodeIdx) {
    // `min` folds to 0 at one word.
    bits[(n as usize / 64).min(A - 1)] |= 1 << (n % 64);
}

/// The outcome of a count-free run ([`run`]).
pub(crate) struct Run<const A: usize> {
    /// Index of the next byte to step.
    pub(crate) next: usize,
    /// Whether a match was found, before the byte at `next`.
    pub(crate) found: bool,
    /// The frontier the run started from.
    entry: [u64; A],
    /// The frontier it ended on: unit, like the one it started from.
    pub(crate) active: [u64; A],
}

impl<const A: usize> Run<A> {
    /// Call `f(node, on)` for every row that the run turned on or off:
    /// the rows to write back.  A row that stayed active holds what it
    /// held, a unit row.
    #[inline(always)]
    pub(crate) fn for_each_change(&self, mut f: impl FnMut(usize, bool)) {
        for k in 0..A {
            let changes = [
                (self.entry[k] & !self.active[k], false),
                (self.active[k] & !self.entry[k], true),
            ];
            for (mut bits, on) in changes {
                while bits != 0 {
                    f(k * 64 + bits.trailing_zeros() as usize, on);
                    bits &= bits - 1;
                }
            }
        }
    }

    /// Active rows at the end of the run.  Most runs end on an empty
    /// frontier, and without a `popcnt` instruction in the target a count
    /// is a score of instructions per word: an empty frontier is not
    /// counted.
    #[inline(always)]
    pub(crate) fn live(&self) -> u32 {
        if self.active.iter().all(|&w| w == 0) {
            return 0;
        }
        self.active.iter().map(|w| w.count_ones()).sum()
    }
}

/// Step a unit, count-free frontier by bitmaps of `A` words, from
/// `input[i]` while the frontier stays nonempty and count-free.  It stays
/// unit: every row of such a closure is a unit row, and consuming moves
/// rows whole.  The frontier is only its bitmap, kept in registers; the
/// caller writes the rows once, from the [`Run`] this returns.
///
/// A step is the closure of the frontier (the OR of its nodes' closure
/// bitmaps and the start's), a match test, and the successors of the
/// consumers that accept the byte: `successors(consumer, byte, next)`
/// sets them in `next` with [`set`].
#[inline(always)]
pub(crate) fn run<const A: usize>(
    tables: &CountFreeTables,
    raw_active: &[u64],
    input: &[u8],
    mut i: usize,
    successors: impl Fn(usize, u8, &mut [u64; A]),
) -> Run<A> {
    // Everything the loop reads, as locals, so that it stays in
    // registers.
    let (closure, accepts): (&[u64], &[u64]) = (&tables.closure, &tables.accept);
    let start: [u64; A] = tables.start_closure[..A].try_into().unwrap();
    let matches: [u64; A] = tables.matches[..A].try_into().unwrap();
    let statics: [u64; A] = tables.statics[..A].try_into().unwrap();
    let entry: [u64; A] = raw_active[..A].try_into().unwrap();
    let mut active = entry;
    // The bytes by iterator: the end of the input ends the loop, and no
    // byte is fetched through a bounds check of its own.
    let mut found = false;
    for &b in &input[i..] {
        let clos = closure_of::<A>(closure, start, &active);
        if (0..A).any(|k| clos[k] & matches[k] != 0) {
            found = true;
            break;
        }
        let mut next = [0u64; A];
        let accept = &accepts[b as usize * A..][..A];
        for k in 0..A {
            let mut fire = clos[k] & accept[k];
            while fire != 0 {
                let c = k * 64 + fire.trailing_zeros() as usize;
                fire &= fire - 1;
                successors(c, b, &mut next);
            }
        }

        active = next;
        i += 1;
        if active.iter().all(|&w| w == 0) || (0..A).any(|k| active[k] & !statics[k] != 0) {
            break;
        }
    }
    Run {
        next: i,
        found,
        entry,
        active,
    }
}
