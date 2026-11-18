//! Operation counters of the grouped NFA (`plans/sparse-counting-sets.md`,
//! S0), behind the `op-stats` cargo feature.
//!
//! They count the work a scan does on count rows, so that representations
//! can be compared by what they do and not only by wall time.  Without the
//! feature [`stat!`] expands to nothing and its argument is not evaluated:
//! timings must be taken from a build without the feature.
//!
//! The counters are per thread.  [`take`] returns and zeroes them.

/// Work done by grouped streams on this thread since the last [`take`].
///
/// "Words" are `u64` words of count rows.  A full sweep copies and clears
/// whole arenas, so its words include those of empty rows: that is the
/// work it does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpStats {
    /// Bytes stepped by the full sweep.
    pub steps_full: u64,
    /// Bytes stepped over the active rows only.
    pub steps_active: u64,
    /// Bytes stepped by the count-free bitmaps.
    pub steps_count_free: u64,
    /// Bytes skipped while no row was active.
    pub bytes_skipped: u64,
    /// Closures that only look for a match (chunk end, finish, reset).
    pub provisional: u64,
    /// Zero-width transfers applied to a nonempty row.
    pub rows_closed: u64,
    /// Words copied from the raw rows into the closure.
    pub words_copied: u64,
    /// Words ORed into a successor row (split, assertion, consumed byte).
    pub words_unioned: u64,
    /// Words shifted by a `CounterIncrement`.
    pub words_incremented: u64,
    /// Words zeroed.
    pub words_cleared: u64,
    /// Byte predicate tests.
    pub tests: u64,
    /// Successor rows written.
    pub fanout: u64,
    /// Active raw rows after each general step, summed.
    pub live_rows: u64,
    /// Those of them inside a counter.
    pub counted_rows: u64,
    /// Words of those counted rows.
    pub counted_words: u64,
    /// Counts in those counted rows.
    pub count_values: u64,
    /// Maximal runs of consecutive counts in those counted rows.
    pub count_runs: u64,
    /// Most counts in one counted raw row.
    pub max_row_values: u64,
}

impl OpStats {
    /// The field names, in the order of [`values`](Self::values).
    pub const FIELDS: [&'static str; 18] = [
        "steps_full",
        "steps_active",
        "steps_count_free",
        "bytes_skipped",
        "provisional",
        "rows_closed",
        "words_copied",
        "words_unioned",
        "words_incremented",
        "words_cleared",
        "tests",
        "fanout",
        "live_rows",
        "counted_rows",
        "counted_words",
        "count_values",
        "count_runs",
        "max_row_values",
    ];

    /// The counters, in the order of [`FIELDS`](Self::FIELDS).
    pub fn values(&self) -> [u64; 18] {
        [
            self.steps_full,
            self.steps_active,
            self.steps_count_free,
            self.bytes_skipped,
            self.provisional,
            self.rows_closed,
            self.words_copied,
            self.words_unioned,
            self.words_incremented,
            self.words_cleared,
            self.tests,
            self.fanout,
            self.live_rows,
            self.counted_rows,
            self.counted_words,
            self.count_values,
            self.count_runs,
            self.max_row_values,
        ]
    }
}

#[cfg(feature = "op-stats")]
thread_local! {
    static STATS: std::cell::Cell<OpStats> = const { std::cell::Cell::new(OpStats {
        steps_full: 0,
        steps_active: 0,
        steps_count_free: 0,
        bytes_skipped: 0,
        provisional: 0,
        rows_closed: 0,
        words_copied: 0,
        words_unioned: 0,
        words_incremented: 0,
        words_cleared: 0,
        tests: 0,
        fanout: 0,
        live_rows: 0,
        counted_rows: 0,
        counted_words: 0,
        count_values: 0,
        count_runs: 0,
        max_row_values: 0,
    }) };
}

/// Update this thread's counters.
#[cfg(feature = "op-stats")]
#[inline]
pub(crate) fn with(f: impl FnOnce(&mut OpStats)) {
    STATS.with(|cell| {
        let mut stats = cell.get();
        f(&mut stats);
        cell.set(stats);
    });
}

/// Return this thread's counters and zero them.  All zero without the
/// `op-stats` feature ([`ENABLED`]).
pub fn take() -> OpStats {
    #[cfg(feature = "op-stats")]
    {
        STATS.with(|cell| cell.take())
    }
    #[cfg(not(feature = "op-stats"))]
    {
        OpStats::default()
    }
}

/// Whether the counters are compiled in.
pub const ENABLED: bool = cfg!(feature = "op-stats");

/// `stat!(field += n)`: add to a counter; `stat!(field max= n)`: raise it.
/// Nothing, with `n` unevaluated, without the `op-stats` feature.
macro_rules! stat {
    ($field:ident += $n:expr) => {
        #[cfg(feature = "op-stats")]
        $crate::grouped_nfa::stats::with(|s| s.$field += ($n) as u64);
    };
    ($field:ident max= $n:expr) => {
        #[cfg(feature = "op-stats")]
        $crate::grouped_nfa::stats::with(|s| s.$field = s.$field.max(($n) as u64));
    };
}
pub(crate) use stat;

#[cfg(all(test, feature = "op-stats"))]
mod tests {
    use super::take;
    use crate::{GroupedNfaMode, GroupedScratch, GroupedStream, Regex, RegexConfig};

    /// Every byte of a scan without a match is counted on exactly one
    /// path, and the counted rows are measured.
    #[test]
    fn a_scan_accounts_for_every_byte() {
        let config = RegexConfig {
            max_unroll_states: 0,
            merge_repetitions: false,
            grouped_nfa: GroupedNfaMode::Always,
            ..RegexConfig::default()
        };
        let re = Regex::with_config("x(a|aa|b){2,40}z", config).unwrap();
        let mut input = b"qqqqqqqq".repeat(8);
        input.extend(b"xaabaabaab".repeat(20));
        let mut scratch = GroupedScratch::new();
        let mut stream = GroupedStream::new(&re, &mut scratch).unwrap();
        take();
        for part in input.chunks(7) {
            stream.chunk(&re, &mut scratch, part);
        }
        assert!(!stream.finish(&re, &mut scratch));
        let s = take();
        assert_eq!(
            s.steps_full + s.steps_active + s.steps_count_free + s.bytes_skipped,
            input.len() as u64
        );
        assert!(s.bytes_skipped >= 64 && s.steps_full + s.steps_active > 0);
        assert!(s.words_incremented > 0 && s.tests > 0 && s.fanout > 0);
        assert!(s.counted_rows > 0 && s.count_values >= s.count_runs);
        assert!(s.count_runs >= s.counted_rows && s.max_row_values >= 2);
        assert_eq!(take(), super::OpStats::default());
    }
}
