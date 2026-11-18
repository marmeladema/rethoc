//! Compilation costs for assertion graphs near the default state limit.
//!
//! Run with identical fixtures on baseline and candidate. Setup compiles every
//! fixture with unmodified defaults, verifies its actual state/counter counts,
//! and prints retained compiled bytes before timing compilation and destruction.
//! Peak process memory requires a separate measurement; `memory_size` is not
//! an allocation high-water mark.

use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use rethoc_engine::{Regex, RegexConfig};

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("tier1_assertions_compile_limits");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(2));

    // Repeat source fragments instead of counted regex syntax: the compiled
    // graphs must remain counter-free under the default unrolling budget.
    let fixtures = [
        ("simple_tails", r"a\b", 900, 0, 900),
        ("acyclic_branching", r"a(?:\b|(?m:^))", 450, 450, 900),
        // A purely zero-width starred body is normalized to optional by
        // regex-syntax. Including a consuming alternative preserves the star
        // and its zero-width cycle through either assertion alternative.
        ("epsilon_cycles", r"(?:\b|(?m:^)|a)*a", 250, 750, 500),
    ];

    for (name, fragment, repetitions, minimum_splits, minimum_assertions) in fixtures {
        let pattern = fragment.repeat(repetitions);
        let probe = Regex::new(&pattern).expect("fixture fits default compile limits");
        let info = probe.info();
        let limit = RegexConfig::default().max_estimated_states;
        // The compiler's estimate excludes its final Match state. These
        // assertions make lowering changes visible instead of silently timing
        // a much smaller graph or a different execution engine.
        assert!(
            (1500..=limit + 1).contains(&info.memory.num_states),
            "{name}: expected a near-limit graph, got {} states",
            info.memory.num_states,
        );
        assert_eq!(info.nfa_states.counter_instance, 0, "{name}");
        assert_eq!(info.nfa_states.counter_increment, 0, "{name}");
        assert_eq!(info.execution.engine, "tier1", "{name}");
        assert!(info.nfa_states.assert >= minimum_assertions, "{name}");
        if minimum_splits == 0 {
            assert_eq!(info.nfa_states.split, 0, "{name}");
        } else {
            assert!(info.nfa_states.split >= minimum_splits, "{name}");
        }
        eprintln!(
            "{name}: states={} assertions={} splits={} compiled_bytes={} regex_bytes={}",
            info.memory.num_states,
            info.nfa_states.assert,
            info.nfa_states.split,
            probe.memory_size(),
            std::mem::size_of::<Regex>(),
        );
        drop(probe);

        group.bench_function(name, |b| {
            b.iter(|| {
                black_box(Regex::new(black_box(&pattern)).expect("validated near-limit pattern"))
            });
        });
    }
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
