//! Shared streaming scan for a benchmark's forced engine.

use rethoc_engine::{MatcherMemory, Regex};

/// Scan through a forced engine, including its EOF handling.
pub fn scan(
    memory: &mut MatcherMemory,
    regex: &Regex,
    input: &[u8],
    chunk: usize,
    tier: u8,
) -> bool {
    let mut matcher = memory
        .matcher_for_tier(regex, tier)
        .expect("fixture tier remains eligible");
    for bytes in input.chunks(chunk) {
        matcher.chunk(bytes);
    }
    matcher.finish()
}
