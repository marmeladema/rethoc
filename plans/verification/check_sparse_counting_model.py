#!/usr/bin/env python3
"""Bounded verification of a proposed finite sparse-counting representation.

Run: scripts/capped --mem 1000000 --time 120 -- \
    python3 plans/verification/check_sparse_counting_model.py

This is an independent integer-set model, not a Rust implementation, regex
compiler, or proof of the paper's complete algorithm. It checks finite positive
counts x=c+1, a direct-value triple reducer, and observations after every possible
number of future increments. It does not model offsets, unbounded saturation,
production admission/allocations, byte predicates, or concrete AssertKind rules.
The gated DAG suite models whole-row assertion filtering and immutable-root
replay algebraically; it does not validate actual assertion scheduling.

All domains and enumeration bounds are fixed below; no random data or packages.
"""

from collections import Counter
from itertools import product

MAX_SET_BOUND = 8
MAX_DAG_BOUND = 4


def members(mask, high):
    """Positive counts, in ascending order; bit zero denotes count one."""
    return tuple(n for n in range(1, high + 1) if mask & (1 << (n - 1)))


def mask_of(values):
    result = 0
    for n in values:
        result |= 1 << (n - 1)
    return result


def reduce_values(values, width):
    """Remove interior b from a<b<c when c-a<=width, preserving endpoints.

    Direct values, rather than the paper's offset-minus-list representation.
    A full left-to-right stack scan is used; no early-stop optimization.
    """
    stack = []
    for n in sorted(set(values)):
        stack.append(n)
        while len(stack) >= 3 and stack[-1] - stack[-3] <= width:
            del stack[-2]
    return tuple(stack)


def observations(values, low, high):
    """Encode E(S) and N(S) for t=0..h; all later answers are false.

    E: some n+t is in [l,h]. N: some n+t remains <=h. The latter matters
    because a path can consume or branch before checking the lower bound.
    """
    exits = live = 0
    for t in range(high + 1):
        if any(low <= n + t <= high for n in values):
            exits |= 1 << t
        if any(n + t <= high for n in values):
            live |= 1 << t
    return exits, live


def check(mask, low, high):
    return any(n >= low for n in members(mask, high))


def increment(mask, high):
    return (mask << 1) & ((1 << high) - 1)


def tables(low, high):
    size = 1 << high
    width = high - low + 1
    sets = [members(mask, high) for mask in range(size)]
    reduced = [mask_of(reduce_values(v, width)) for v in sets]
    obs = [observations(v, low, high) for v in sets]
    return sets, reduced, obs


def verify_sets(counts):
    for high in range(1, MAX_SET_BOUND + 1):
        for low in range(high + 1):
            sets, reduced, obs = tables(low, high)
            width = high - low + 1
            capacity = min(high, 2 * ((high + width) // (width + 1)))
            for exact, values in enumerate(sets):
                sparse = reduced[exact]
                representatives = sets[sparse]
                assert sparse & ~exact == 0
                assert obs[exact] == obs[sparse]
                assert len(representatives) <= capacity
                assert all(representatives[i + 2] - representatives[i] > width
                           for i in range(len(representatives) - 2))
                if values:
                    assert representatives[0] == values[0]
                    assert representatives[-1] == values[-1]
                counts['set_cases'] += 1

                # Future observations shift by one when counts increment.
                inc_e = increment(exact, high)
                inc_s = increment(sparse, high)
                assert obs[inc_e] == obs[inc_s]
                assert obs[inc_e] == (obs[exact][0] >> 1, obs[exact][1] >> 1)
                counts['increment_cases'] += 1

                # CI/re-entry seed or reset: a successful exit yields {1}.
                assert obs[exact | 1] == obs[reduced[sparse | 1]]
                reset_e = int(check(exact, low, high))
                reset_s = int(check(sparse, low, high))
                assert reset_e == reset_s
                counts['seed_and_reset_cases'] += 1

                # Arbitrary whole-row gate outcomes (not count-dependent masks).
                for passed in (False, True):
                    assert obs[exact if passed else 0] == obs[sparse if passed else 0]
                    counts['whole_row_gate_cases'] += 1

                # Exact grouped c counts completed iterations, c in [0,h-1].
                # Our x=c+1 domain is [1,h]. Exit observes x before inc.
                completed = tuple(x - 1 for x in values)
                exact_break = any(c + 1 >= low for c in completed)
                exact_continue = {c + 1 for c in completed if c + 1 < high}
                assert exact_break == check(exact, low, high)
                assert {x - 1 for x in members(inc_e, high)} == exact_continue
                counts['thompson_mapping_cases'] += 1

            # Every pair, not just sets arising in a selected regex family.
            for left in range(1 << high):
                for right in range(1 << high):
                    merged = reduced[reduced[left] | reduced[right]]
                    assert obs[left | right] == obs[merged]
                    assert merged & ~(left | right) == 0
                    counts['merge_cases'] += 1


def dag(raw_a, raw_b, gates, low, high, reduced=None):
    """An acyclic composition of copy, merge, inc, reset and row gates.

    The two raw inputs are not mutated. Nodes 0..7 are topologically ordered.
    Reset is a composed exit-to-Outside / CI-entry transfer into the same scope.
    This is an operation DAG, not an extracted Thompson graph.
    """
    rows = [0] * 8

    def add(target, values):
        rows[target] |= values
        if reduced is not None:
            rows[target] = reduced[rows[target]]

    add(0, raw_a)
    add(1, raw_b)
    add(2, rows[0] if gates[0] else 0)
    add(3, increment(rows[0], high))
    add(3, rows[1])
    add(4, int(check(rows[2], low, high)))
    add(5, rows[2])
    add(5, increment(rows[3], high))
    add(5, rows[4])
    add(6, rows[5] if gates[1] else 0)
    add(7, rows[6])
    add(7, rows[1])
    return tuple(rows)


def verify_replay_dags(counts):
    for high in range(1, MAX_DAG_BOUND + 1):
        for low in range(high + 1):
            _, reduced, obs = tables(low, high)
            for left in range(1 << high):
                for right in range(1 << high):
                    roots = (left, right)
                    sparse_roots = (reduced[left], reduced[right])
                    for first, second in product(product((False, True), repeat=2), repeat=2):
                        # Two closures from the SAME roots under independently
                        # revealed gate outcomes. Replaying does not commit inc.
                        for gates in (first, second):
                            exact = dag(*roots, gates, low, high)
                            sparse = dag(*sparse_roots, gates, low, high, reduced)
                            for e, s in zip(exact, sparse):
                                assert s & ~e == 0
                                assert obs[e] == obs[s]
                                counts['dag_row_comparisons'] += 1
                        assert roots == (left, right)
                        assert sparse_roots == (reduced[left], reduced[right])
                        counts['replay_pairs'] += 1


def mutation_witnesses():
    # Too permissive threshold loses a future exit opportunity.
    values, low, high = (1, 2, 3), 3, 3
    broken = reduce_values(values, high - low + 2)
    assert observations(values, low, high) != observations(broken, low, high)
    threshold = dict(exact=values, broken=broken, low=low, high=high)

    # Testing exit after continuation loses the last legal iteration.
    values, low, high = (3,), 3, 3
    continued = tuple(n + 1 for n in values if n < high)
    assert any(n >= low for n in values) and not any(n >= low for n in continued)
    timing = dict(values=values, continued=continued, low=low, high=high)

    # Sparse equivalence is not congruent under arbitrary masks.
    values, low, high = tuple(range(4, 11)), 5, 10
    sparse = reduce_values(values, high - low + 1)
    assert sparse == (4, 10)
    assert observations(values, low, high) == observations(sparse, low, high)
    masked = tuple(n for n in values if n <= 7)
    sparse_masked = tuple(n for n in sparse if n <= 7)
    assert observations(masked, low, high) != observations(sparse_masked, low, high)
    mask = dict(exact=values, sparse=sparse, mask='<=7', low=low, high=high)

    # Treating a speculative continuation as a new raw root double-increments.
    low, high, raw = 2, 2, mask_of((1,))
    once = increment(raw, high)
    twice = increment(once, high)
    assert check(once, low, high) and not check(twice, low, high)
    replay = dict(raw=(1,), once=members(once, high), twice=members(twice, high))
    return {'threshold_k_plus_one': threshold, 'exit_after_increment': timing,
            'arbitrary_mask': mask, 'committed_speculative_increment': replay}


def main():
    if not __debug__:
        raise RuntimeError('Run without -O: this verifier requires Python assertions')
    counts = Counter()
    verify_sets(counts)
    verify_replay_dags(counts)
    witnesses = mutation_witnesses()
    print(f'Finite set bound: {MAX_SET_BOUND}; gated-DAG bound: {MAX_DAG_BOUND}')
    for name, count in sorted(counts.items()):
        print(f'{name}: {count}')
    for name, witness in witnesses.items():
        print(f'MUTATION DETECTED {name}: {witness}')
    print('PASS: bounded model checks only; not a Rust engine or assertion-semantics proof.')


if __name__ == '__main__':
    main()
