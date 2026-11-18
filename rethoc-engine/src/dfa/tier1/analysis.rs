//! Exact boundary metadata for counter-free assertion graphs.
//!
//! A finite-bit worklist computes the previous-context requirements of each
//! zero-width continuation. A five-bit reachability fixed point computes EOF
//! acceptance, including assertion chains and zero-width cycles. Both analyses
//! stop at consuming states and use storage linear in the compiled graph.

use std::fmt;
use std::mem::size_of;

use crate::{AssertEval, AssertKind, ByteProps, State, StateIdx};

/// Input-start bit in the previous-context encoding; byte bits are `ByteProps`.
const START: u8 = 8;

/// Physical previous contexts used for exact EOF acceptance.
const EOF_CONTEXTS: [Option<u8>; 5] = [None, Some(b'a'), Some(b'\n'), Some(b'\r'), Some(b' ')];

/// Metadata for one NFA state, kept together to need only one allocation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct AssertionInfo {
    /// Previous-byte/start properties needed by the known-next continuation.
    requirements: u8,
    /// One acceptance bit for each entry of `EOF_CONTEXTS`.
    eof: u8,
}

/// Exact assertion metadata for a counter-free NFA.
///
/// The default has an empty entries slice with no payload allocation. It
/// represents an assertion-free graph or a graph with counters, which has no
/// Tier 1 analysis.
/// Otherwise the table has one entry for every NFA state, including consumers
/// and Match states. Indexed queries require this complete, nonempty table.
/// `Regex` boxes this owner even when its entries are empty, keeping a single
/// pointer in the compiled regex without optional access during matching.
#[derive(Debug, Default)]
pub(crate) struct Tier1Assertions {
    /// Two bytes per NFA state; no nested allocations.
    entries: Box<[AssertionInfo]>,
}

impl Tier1Assertions {
    /// Analyze a counter-free graph, returning an empty table without allocating
    /// entries when it has no assertions.
    ///
    /// Every worklist bit can be introduced at most once per state, so graph
    /// cycles do not require recursion or unbounded repeated traversals.
    pub(crate) fn build(states: &[State]) -> Self {
        let mut has_assertions = false;
        let mut simple = true;
        let mut ordered = true;
        for (raw, state) in states.iter().enumerate() {
            match *state {
                State::Assert { out, .. } => {
                    has_assertions = true;
                    ordered &= out.idx() > raw;
                    simple &= !matches!(
                        states[out.idx()],
                        State::Assert { .. }
                            | State::Split { .. }
                            | State::CounterInstance { .. }
                            | State::CounterIncrement { .. }
                    );
                }
                State::Split { out, out1 } => {
                    simple = false;
                    ordered &= out.idx() > raw && out1.idx() > raw;
                }
                State::CounterInstance { .. } | State::CounterIncrement { .. } => {
                    debug_assert!(false, "Tier 1 analysis requires a counter-free graph");
                }
                _ => {}
            }
        }
        if !has_assertions {
            return Self::default();
        }
        if simple {
            Self::build_simple(states)
        } else if ordered {
            Self::build_ordered(states)
        } else {
            Self::build_general(states)
        }
    }

    /// Whether no Tier 1 assertion analysis is stored.
    ///
    /// For a Tier 1 eligible graph this selects the assertion-free closure,
    /// which derives EOF acceptance from traversal instead of indexing a table.
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Analyze isolated assertions without reverse edges or worklist scratch.
    ///
    /// With no Split and every assertion targeting a consumer or Match, an
    /// epsilon path crosses at most one assertion. Its context requirements
    /// are exactly that assertion's reads; its EOF acceptance is either false
    /// (consumer target) or the assertion's own EOF predicate (Match target).
    fn build_simple(states: &[State]) -> Self {
        let entries = states
            .iter()
            .map(|state| match *state {
                State::Match => AssertionInfo {
                    requirements: 0,
                    eof: 0b11111,
                },
                State::Assert { kind, out } => AssertionInfo {
                    requirements: direct_requirements(kind),
                    eof: if matches!(states[out.idx()], State::Match) {
                        allowed_eof(kind)
                    } else {
                        0
                    },
                },
                _ => AssertionInfo::default(),
            })
            .collect();
        Self { entries }
    }

    /// Reverse scan when state order is a verified epsilon topological order.
    ///
    /// The guard checks every epsilon edge, including both Split successors.
    /// Consuming edges can point anywhere: neither analysis follows them.
    fn build_ordered(states: &[State]) -> Self {
        let mut entries = vec![AssertionInfo::default(); states.len()];
        for (raw, state) in states.iter().enumerate().rev() {
            entries[raw] = match *state {
                State::Match => AssertionInfo {
                    requirements: 0,
                    eof: 0b11111,
                },
                State::Assert { kind, out } => AssertionInfo {
                    requirements: if kind == AssertKind::End {
                        0
                    } else {
                        direct_requirements(kind) | entries[out.idx()].requirements
                    },
                    eof: if entries[out.idx()].eof == 0 {
                        0
                    } else {
                        allowed_eof(kind) & entries[out.idx()].eof
                    },
                },
                State::Split { out, out1 } => AssertionInfo {
                    requirements: entries[out.idx()].requirements
                        | entries[out1.idx()].requirements,
                    eof: entries[out.idx()].eof | entries[out1.idx()].eof,
                },
                _ => AssertionInfo::default(),
            };
        }
        Self {
            entries: entries.into_boxed_slice(),
        }
    }

    /// General fixed-point analysis for epsilon branching, chains and cycles.
    fn build_general(states: &[State]) -> Self {
        let reverse = ReverseEpsilon::build(states);
        let mut entries = vec![AssertionInfo::default(); states.len()];
        let mut work = Vec::with_capacity(states.len());
        for (raw, state) in states.iter().enumerate() {
            if let State::Assert { kind, .. } = state {
                let requirements = direct_requirements(*kind);
                entries[raw].requirements = requirements;
                if requirements != 0 {
                    work.push(StateIdx(raw as u32));
                }
            }
        }

        while let Some(state) = work.pop() {
            let required = entries[state.idx()].requirements;
            for &predecessor in reverse.predecessors(state) {
                // A known-next-byte closure cannot cross End. Its downstream
                // EOF-only dependencies belong in the separate exact EOF mask.
                if matches!(
                    states[predecessor.idx()],
                    State::Assert {
                        kind: AssertKind::End,
                        ..
                    }
                ) {
                    continue;
                }
                let entry = &mut entries[predecessor.idx()];
                let expanded = entry.requirements | required;
                if expanded != entry.requirements {
                    entry.requirements = expanded;
                    work.push(predecessor);
                }
            }
        }

        // Propagate all five context bits together. An assertion's gate is
        // computed once, instead of re-evaluated on every accepted target.
        let allowed: Vec<u8> = states
            .iter()
            .enumerate()
            .map(|(raw, state)| match *state {
                State::Assert { kind, .. } => allowed_eof(kind),
                State::Split { .. } => 0b11111,
                State::Match => {
                    entries[raw].eof = 0b11111;
                    work.push(StateIdx(raw as u32));
                    0
                }
                _ => 0,
            })
            .collect();
        while let Some(state) = work.pop() {
            let target = entries[state.idx()].eof;
            for &predecessor in reverse.predecessors(state) {
                let entry = &mut entries[predecessor.idx()];
                let expanded = entry.eof | (target & allowed[predecessor.idx()]);
                if expanded != entry.eof {
                    entry.eof = expanded;
                    work.push(predecessor);
                }
            }
        }

        Self {
            entries: entries.into_boxed_slice(),
        }
    }

    /// Properties needed to resolve this state's known-next-byte continuation.
    #[inline]
    pub(crate) fn requirements(&self, state: StateIdx) -> u8 {
        self.entries[state.idx()].requirements
    }

    /// Whether this state can accept at EOF in the actual, unmasked context.
    ///
    /// Context is exactly one of zero (other byte), WORD, LF, CR, or START.
    /// Query before masking context for a cached pending frontier.
    #[inline]
    pub(crate) fn eof_accept(&self, state: StateIdx, context: u8) -> bool {
        let bit = match context {
            START => 1,
            1 => 1 << 1,
            2 => 1 << 2,
            4 => 1 << 3,
            0 => 1 << 4,
            _ => unreachable!("EOF acceptance requires a physical boundary context"),
        };
        self.entries[state.idx()].eof & bit != 0
    }

    /// Exact EOF bits in start, word, LF, CR, other order, for diagnostics.
    pub(crate) fn eof_mask(&self, state: StateIdx) -> u8 {
        self.entries[state.idx()].eof
    }

    /// Heap bytes retained by the metadata; the owner is counted in `Regex`.
    pub(crate) fn memory_size(&self) -> usize {
        self.entries.len() * size_of::<AssertionInfo>()
    }
}

impl fmt::Display for Tier1Assertions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tier 1 assertions: {} states", self.entries.len())?;
        writeln!(f, "  EOF bits: start, word, LF, CR, other")?;
        for (raw, entry) in self.entries.iter().enumerate() {
            writeln!(
                f,
                "  {raw}: previous={} start={} eof={:05b}",
                ByteProps(entry.requirements & !START),
                entry.requirements & START != 0,
                self.eof_mask(StateIdx(raw as u32)),
            )?;
        }
        Ok(())
    }
}

/// Previous-byte and input-start requirements declared by one assertion.
fn direct_requirements(kind: AssertKind) -> u8 {
    let (previous, _, start) = kind.context_reads();
    previous.0 | if start { START } else { 0 }
}

/// Physical EOF contexts in which this assertion's actual evaluator passes.
fn allowed_eof(kind: AssertKind) -> u8 {
    EOF_CONTEXTS
        .iter()
        .enumerate()
        .fold(0, |mask, (bit, &prev)| {
            mask | if kind.eval(prev.is_none(), true, prev, None) == AssertEval::Pass {
                1 << bit
            } else {
                0
            }
        })
}

/// Flat reverse adjacency for Split and Assert edges only.
struct ReverseEpsilon {
    offsets: Vec<usize>,
    sources: Vec<StateIdx>,
}

impl ReverseEpsilon {
    /// Build at most two reverse edges per compiled state.
    fn build(states: &[State]) -> Self {
        // The input is an allocated slice of nonzero-sized State values and
        // indexed by u32. Consequently N+1 and at most 2N epsilon edges fit
        // usize on supported targets; neither bound depends on pattern counts.
        let mut offsets = vec![0usize; states.len() + 1];
        for state in states {
            for target in epsilon_targets(state).into_iter().flatten() {
                offsets[target.idx() + 1] += 1;
            }
        }
        for raw in 1..offsets.len() {
            offsets[raw] += offsets[raw - 1];
        }
        let mut cursors = offsets[..states.len()].to_vec();
        let mut sources = vec![StateIdx::NONE; offsets[states.len()]];
        for (raw, state) in states.iter().enumerate() {
            for target in epsilon_targets(state).into_iter().flatten() {
                let cursor = &mut cursors[target.idx()];
                sources[*cursor] = StateIdx(raw as u32);
                *cursor += 1;
            }
        }
        Self { offsets, sources }
    }

    /// Sources of zero-width edges entering `state`.
    fn predecessors(&self, state: StateIdx) -> &[StateIdx] {
        &self.sources[self.offsets[state.idx()]..self.offsets[state.idx() + 1]]
    }
}

/// Zero-width successors, excluding all counter and consuming transitions.
fn epsilon_targets(state: &State) -> [Option<StateIdx>; 2] {
    match *state {
        State::Split { out, out1 } => [Some(out), Some(out1)],
        State::Assert { out, .. } => [Some(out), None],
        _ => [None, None],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assertion(kind: AssertKind, out: u32) -> State {
        State::Assert {
            kind,
            out: StateIdx(out),
        }
    }

    fn byte(out: u32) -> State {
        State::Byte {
            byte: b'a',
            out: StateIdx(out),
            out_exit: StateIdx::NONE,
        }
    }

    fn context(previous: Option<u8>) -> u8 {
        previous.map_or(START, |byte| ByteProps::of(byte).0)
    }

    /// Direct forward closure used only as an independent table oracle.
    fn eof_oracle(states: &[State], start: StateIdx, previous: Option<u8>) -> bool {
        let mut work = vec![start];
        let mut visited = vec![false; states.len()];
        while let Some(state) = work.pop() {
            if visited[state.idx()] {
                continue;
            }
            visited[state.idx()] = true;
            match states[state.idx()] {
                State::Match => return true,
                State::Split { out, out1 } => work.extend([out, out1]),
                State::Assert { kind, out }
                    if kind.eval(previous.is_none(), true, previous, None) == AssertEval::Pass =>
                {
                    work.push(out);
                }
                _ => {}
            }
        }
        false
    }

    fn check_eof(states: &[State], analysis: &Tier1Assertions) {
        for raw in 0..states.len() {
            let state = StateIdx(raw as u32);
            for previous in std::iter::once(None).chain((0..=255u8).map(Some)) {
                assert_eq!(
                    analysis.eof_accept(state, context(previous)),
                    eof_oracle(states, state, previous),
                    "state {raw}, previous {previous:?}",
                );
            }
        }
    }

    #[test]
    fn assertion_free_has_no_metadata() {
        for analysis in [
            Tier1Assertions::default(),
            Tier1Assertions::build(&[byte(1), State::Match]),
        ] {
            assert!(analysis.is_empty());
            assert_eq!(analysis.memory_size(), 0);
        }
    }

    #[test]
    fn isolated_assertions_agree_with_general_fixed_point() {
        for &kind in AssertKind::ALL {
            // Isolated assertions on both sides of a consuming byte, including
            // every EOF predicate and previous-context requirement.
            let states = [
                assertion(kind, 1),
                byte(2),
                assertion(kind, 3),
                State::Match,
            ];
            let selected = Tier1Assertions::build(&states);
            let simple = Tier1Assertions::build_simple(&states);
            let general = Tier1Assertions::build_general(&states);
            assert_eq!(selected.entries, simple.entries);
            assert_eq!(simple.entries, general.entries);
            check_eof(&states, &simple);
        }
    }

    #[test]
    fn ordered_branches_agree_with_general_fixed_point() {
        for &first in AssertKind::ALL {
            for &second in AssertKind::ALL {
                let states = [
                    State::Split {
                        out: StateIdx(1),
                        out1: StateIdx(4),
                    },
                    assertion(first, 2),
                    assertion(second, 3),
                    State::Match,
                    assertion(second, 5),
                    byte(6),
                    State::Match,
                ];
                let selected = Tier1Assertions::build(&states);
                let ordered = Tier1Assertions::build_ordered(&states);
                let general = Tier1Assertions::build_general(&states);
                assert_eq!(selected.entries, ordered.entries);
                assert_eq!(ordered.entries, general.entries);
                check_eof(&states, &ordered);
            }
        }
    }

    #[test]
    fn backward_assertion_chain_uses_general_analysis() {
        for &kind in AssertKind::ALL {
            let states = [
                State::Match,
                assertion(kind, 0),
                assertion(AssertKind::End, 1),
            ];
            let selected = Tier1Assertions::build(&states);
            let general = Tier1Assertions::build_general(&states);
            assert_eq!(selected.entries, general.entries);
            check_eof(&states, &selected);
        }
    }

    #[test]
    fn eof_matches_all_assertion_pairs_and_consuming_tails() {
        for &first in AssertKind::ALL {
            for &second in AssertKind::ALL {
                let states = [
                    assertion(first, 1),
                    assertion(second, 2),
                    State::Match,
                    byte(2),
                    assertion(first, 3),
                ];
                let analysis = Tier1Assertions::build(&states);
                let general = Tier1Assertions::build_general(&states);
                assert_eq!(analysis.entries, general.entries);
                check_eof(&states, &analysis);
                assert_eq!(analysis.memory_size(), 2 * states.len());
            }
        }
    }

    #[test]
    fn dependency_masks_preserve_known_next_assertion_chains() {
        // Unlike the EOF check, this observes which consuming terminal a
        // chain reaches with a byte waiting at the same boundary. The five
        // physical previous contexts cover every declared byte property.
        for &first in AssertKind::ALL {
            for &second in AssertKind::ALL {
                let states = [
                    assertion(first, 1),
                    assertion(second, 2),
                    byte(3),
                    State::Match,
                ];
                let analysis = Tier1Assertions::build(&states);
                for previous in EOF_CONTEXTS {
                    let masked = context(previous) & analysis.requirements(StateIdx(0));
                    let representative = if masked & START != 0 {
                        None
                    } else {
                        Some(ByteProps(masked).stand_in())
                    };
                    for next in 0..=255u8 {
                        let reaches_consumer = |prev: Option<u8>| {
                            [first, second].into_iter().all(|kind| {
                                kind.eval(prev.is_none(), false, prev, Some(next))
                                    == AssertEval::Pass
                            })
                        };
                        assert_eq!(
                            reaches_consumer(previous),
                            reaches_consumer(representative),
                            "{first:?}, {second:?}, previous {previous:?}, next {next}",
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn previous_requirements_include_chains_but_stop_at_end_and_consumers() {
        let states = [
            assertion(AssertKind::WordAscii, 1),
            assertion(AssertKind::StartLF, 2),
            byte(3),
            assertion(AssertKind::StartCRLF, 4),
            State::Match,
            assertion(AssertKind::WordAscii, 6),
            assertion(AssertKind::End, 3),
            assertion(AssertKind::EndLF, 4),
        ];
        let analysis = Tier1Assertions::build(&states);
        assert_eq!(analysis.requirements(StateIdx(0)), 1 | 2 | START);
        assert_eq!(analysis.requirements(StateIdx(1)), 2 | START);
        assert_eq!(analysis.requirements(StateIdx(2)), 0);
        assert_eq!(analysis.requirements(StateIdx(3)), 2 | 4 | START);
        assert_eq!(analysis.requirements(StateIdx(5)), 1);
        assert_eq!(analysis.requirements(StateIdx(6)), 0);
        assert_eq!(analysis.requirements(StateIdx(7)), 0);
        check_eof(&states, &analysis);
    }

    #[test]
    fn cyclic_dependencies_and_eof_use_least_fixed_points() {
        let states = [
            State::Split {
                out: StateIdx(1),
                out1: StateIdx(3),
            },
            assertion(AssertKind::WordAscii, 2),
            assertion(AssertKind::StartCRLF, 0),
            assertion(AssertKind::EndCRLF, 4),
            State::Match,
            // This disconnected cycle has assertions but no path to Match.
            assertion(AssertKind::WordAsciiNegate, 6),
            State::Split {
                out: StateIdx(5),
                out1: StateIdx(6),
            },
        ];
        let analysis = Tier1Assertions::build(&states);
        for raw in 0..3 {
            assert_eq!(analysis.requirements(StateIdx(raw)), 1 | 2 | 4 | START);
        }
        assert_eq!(analysis.requirements(StateIdx(3)), 4);
        assert_eq!(analysis.eof_mask(StateIdx(5)), 0);
        assert_eq!(analysis.eof_mask(StateIdx(6)), 0);
        check_eof(&states, &analysis);
    }
}
