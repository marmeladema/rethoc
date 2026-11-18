#!/usr/bin/env python3
"""Bounded audit of the proposed Tier 1 assertion scheduler and state key.

Run under scripts/capped. This is an executable design model, not the Rust
implementation or an external regex oracle. It shares an independently
transcribed assertion predicate with the grouped model; check_assertion_eval.py
separately checks the actual Rust predicates against complete-boundary rules.

Compare a cached, canonicalized pending-frontier implementation with an uncached
complete-context graph interpreter. Include all ordered assertion pairs, epsilon
cycles, splits, literal/wildcard consumers, all short input partitions, empty
chunks, cache reuse, and eviction. Integer state-ID remapping, byte-class tables,
compiler output, counters, memory layouts, allocation, and speed are not modeled.
"""

from collections import deque
from itertools import combinations, product
import sys

# Keep this read-only audit from leaving bytecode artifacts in plans/.
sys.dont_write_bytecode = True
from check_grouped_assertion_model import ALPH, KINDS, ev, word


CONTEXTS = (None, 97, 13, 10, 32)  # Start, word, CR, LF, other.
WORD, CR, LF, START = 1, 2, 4, 8


def properties(prev):
    if prev is None:
        return START
    return WORD if word(prev) else CR if prev == 13 else LF if prev == 10 else 0


def context_index(prev):
    return CONTEXTS.index(
        None if prev is None else 97 if word(prev) else prev if prev in (13, 10) else 32
    )


def derive_previous_reads(kind):
    """Find a sufficient minimal subset of previous-context facts, from eval.

    Only known-next, nonterminal evaluation is relevant to deferred resolution.
    End therefore reads nothing, and EndLF does not acquire a previous-LF bit.
    """
    for size in range(5):
        for bits in combinations((WORD, CR, LF, START), size):
            mask = sum(bits)
            observations = {}
            consistent = True
            for prev, nxt in product(CONTEXTS, ALPH):
                key = (properties(prev) & mask, nxt)
                result = ev(kind, prev is None, False, prev, nxt)
                if key in observations and observations[key] != result:
                    consistent = False
                    break
                observations[key] = result
            if consistent:
                return mask
    raise AssertionError(kind)


READS = {kind: derive_previous_reads(kind) for kind in KINDS}
assert READS["EndLF"] == 0
assert READS["EndCRLF"] == CR
assert READS["StartCRLF"] == START | LF | CR


class Graph:
    def __init__(self, label):
        self.label = label
        self.nodes = []

    def add(self, *node):
        self.nodes.append(node)
        return len(self.nodes) - 1

    def chain(self, items, out):
        for item in reversed(items):
            out = self.add("assert", item, out) if item in KINDS else self.add("byte", item, out)
        return out


def simple(items, label):
    graph = Graph(label)
    graph.start = graph.chain(items, graph.add("match"))
    return graph


def families():
    for first, second in product(KINDS, repeat=2):
        yield simple([first, second, 97], f"prefix:{first},{second}")
        yield simple(["any", first, second], f"suffix:{first},{second}")

        graph = Graph(f"split:{first},{second}")
        matched = graph.add("match")
        left = graph.chain([first, second, 97], matched)
        right = graph.chain([13, second, first], matched)
        graph.start = graph.add("split", left, right)
        yield graph

        graph = Graph(f"cycle:{first},{second}")
        matched = graph.add("match")
        cycle = graph.add("split", -1, -1)
        loop = graph.chain([first, second], cycle)
        exit_path = graph.chain(["any", second, first], matched)
        graph.nodes[cycle] = ("split", loop, exit_path)
        graph.start = cycle
        yield graph

    cases = (
        ([13, "StartCRLF", 97], "CR-before-start"),
        ([13, "EndCRLF", 10], "CRLF-middle-end"),
        ([13, "StartCRLF", 10], "CRLF-middle-start"),
        ([10, "WordAscii", "StartLF", 97], "LF-behind-word"),
        (["WordAscii", "Start", 97], "start-behind-word"),
        (["WordAsciiNegate", "StartLF"], "empty-start-behind-nonword"),
        ([97, "End", "WordAsciiNegate"], "EOF-nonword-tail"),
        (["End", "WordAscii"], "empty-EOF-word-tail"),
        (["End", "Start"], "EOF-start-tail"),
    )
    for items, label in cases:
        yield simple(items, label)

    # Distinct literal prefixes produce more than eight cached frontiers even
    # on length-three inputs, so the small-cache run must evict and recompute.
    graph = Graph("cache-churn")
    matched = graph.add("match")
    branches = [
        graph.chain([first, second, "any", 97, "End"], matched)
        for first, second in product(ALPH, repeat=2)
    ]
    root = branches.pop()
    for branch in branches:
        root = graph.add("split", root, branch)
    graph.start = root
    yield graph


def accepts(predicate, byte):
    return predicate == "any" or predicate == byte


def complete_oracle(graph, data):
    """Fresh raw roots; each boundary sees its real previous and next byte.

    No pending frontier, context mask, EOF table, or cached state is shared with
    the proposed matcher. Boolean search may finish at any input boundary.
    """
    roots = {graph.start}
    for position in range(len(data) + 1):
        prev = data[position - 1] if position else None
        nxt = data[position] if position < len(data) else None
        todo = list(roots)
        visited = set()
        targets = {graph.start}
        while todo:
            index = todo.pop()
            if index in visited:
                continue
            visited.add(index)
            node = graph.nodes[index]
            if node[0] == "match":
                return True
            if node[0] == "split":
                todo.extend(node[1:])
            elif node[0] == "assert":
                if ev(node[1], position == 0, nxt is None, prev, nxt):
                    todo.append(node[2])
            elif nxt is not None and accepts(node[1], nxt):
                targets.add(node[2])
        roots = targets
    return False


def compile_eof(graph, optimistic=False):
    """Five reverse reachability passes; cycles require no recursion."""
    masks = [0] * len(graph.nodes)
    for context, prev in enumerate(CONTEXTS):
        reverse = [[] for _ in graph.nodes]
        todo = deque()
        seen = set()
        for index, node in enumerate(graph.nodes):
            if node[0] == "match":
                todo.append(index)
                seen.add(index)
            elif node[0] == "split":
                for target in node[1:]:
                    reverse[target].append(index)
            elif node[0] == "assert" and (
                optimistic or ev(node[1], prev is None, True, prev, None)
            ):
                reverse[node[2]].append(index)
        while todo:
            index = todo.popleft()
            masks[index] |= 1 << context
            for predecessor in reverse[index]:
                if predecessor not in seen:
                    seen.add(predecessor)
                    todo.append(predecessor)
    return masks


def compile_needs(graph):
    """Monotone finite-bit propagation through known-next epsilon tails."""
    needs = [0] * len(graph.nodes)
    reverse = [[] for _ in graph.nodes]
    for index, node in enumerate(graph.nodes):
        if node[0] == "split":
            for target in node[1:]:
                reverse[target].append(index)
        elif node[0] == "assert" and node[1] != "End":
            needs[index] = READS[node[1]]
            reverse[node[2]].append(index)
    todo = deque(range(len(graph.nodes)))
    while todo:
        index = todo.popleft()
        for predecessor in reverse[index]:
            expanded = needs[predecessor] | needs[index]
            if expanded != needs[predecessor]:
                needs[predecessor] = expanded
                todo.append(predecessor)
    return needs


class Cached:
    def __init__(self, graph, cap=None, mutation=None):
        self.graph = graph
        self.cap = cap
        self.mutation = mutation
        self.eof_masks = compile_eof(graph, mutation == "optimistic-eof")
        self.needs = compile_needs(graph)
        self.transitions = {}
        self.states = set()
        self.hits = self.misses = self.evictions = 0
        self.start = self.provisional((graph.start,), None)
        self.states.add(self.start)
        self.reset()

    def closure(self, roots, prev, nxt):
        todo = list(roots)
        seen = set()
        consumers, pending = set(), set()
        matched = False
        while todo:
            index = todo.pop()
            if index in seen:
                continue
            seen.add(index)
            node = self.graph.nodes[index]
            if node[0] == "match":
                matched = True
            elif node[0] == "split":
                todo.extend(node[1:])
            elif node[0] == "byte":
                consumers.add(index)
            elif node[1] != "End":
                result = ev(node[1], prev is None, False, prev, nxt)
                if result is True:
                    todo.append(node[2])
                elif result is None:
                    pending.add(index)
        return tuple(sorted(consumers)), tuple(sorted(pending)), matched

    def canonical(self, pending, prev):
        mask = 0
        for index in pending:
            mask |= self.needs[index]
        if self.mutation == "lost-cr":
            mask &= ~CR
        elif self.mutation == "lost-lf":
            mask &= ~LF
        elif self.mutation == "lost-start":
            mask &= ~START
        signature = properties(prev) & mask
        # Prefer ordinary nonword as the irrelevant-context representative.
        for representative in (32, 97, 13, 10, None):
            if properties(representative) & mask == signature:
                return context_index(representative)
        raise AssertionError((mask, signature))

    def provisional(self, roots, prev):
        context_bit = 1 << context_index(prev)
        eof = any(self.eof_masks[index] & context_bit for index in roots)
        consumers, pending, matched = self.closure(roots, prev, None)
        return consumers, pending, matched, eof, self.canonical(pending, prev)

    def reset(self):
        self.current = self.start
        self.ever = self.start[2]

    def step(self, byte):
        if self.ever:
            return
        key = (self.current, byte)
        if key in self.transitions:
            self.hits += 1
            successor = self.transitions[key]
        else:
            self.misses += 1
            consumers, pending, _, _, context = self.current
            extra, unresolved, resolved_match = self.closure(
                pending, CONTEXTS[context], byte
            )
            assert not unresolved
            targets = {self.graph.start}
            for index in set(consumers).union(extra):
                node = self.graph.nodes[index]
                if accepts(node[1], byte):
                    targets.add(node[2])
            successor = self.provisional(targets, byte)
            if resolved_match:
                successor = successor[:2] + (True,) + successor[3:]
            if (
                self.cap is not None
                and successor not in self.states
                and len(self.states) >= self.cap
            ):
                self.evictions += 1
                self.transitions.clear()
                self.states = {self.start, self.current}
            self.states.add(successor)
            self.transitions[key] = successor
        self.current = successor
        self.ever |= successor[2]

    def chunk(self, data):
        for byte in data:
            self.step(byte)

    def finish(self):
        return self.ever or self.current[3]


def partitions(data):
    if not data:
        yield ((),)
        return
    for cuts in range(1 << (len(data) - 1)):
        chunks = []
        start = 0
        for boundary in range(1, len(data)):
            if cuts & (1 << (boundary - 1)):
                chunks.append(data[start:boundary])
                start = boundary
        chunks.append(data[start:])
        yield tuple(chunks)


def audit():
    inputs = [data for length in range(4) for data in product(ALPH, repeat=length)]
    graphs = runs = prefix_checks = positives = hits = misses = evictions = 0
    for graph in families():
        graphs += 1
        matchers = (Cached(graph), Cached(graph, cap=8))
        for data in inputs:
            expected = [complete_oracle(graph, data[:length]) for length in range(len(data) + 1)]
            for chunks in partitions(data):
                for matcher in matchers:
                    matcher.reset()
                    runs += 1
                    position = 0
                    assert matcher.finish() == expected[0], (graph.label, data, "initial")
                    prefix_checks += 1
                    for chunk in chunks:
                        before = (matcher.current, matcher.ever)
                        matcher.chunk(())
                        assert (matcher.current, matcher.ever) == before
                        matcher.chunk(chunk)
                        position += len(chunk)
                        assert matcher.finish() == expected[position], (
                            graph.label, data, chunks, position, matcher.cap,
                            matcher.finish(), expected[position],
                        )
                        prefix_checks += 1
                    matcher.chunk(())
                    assert matcher.finish() == expected[-1]
                    positives += bool(expected[-1])
        for matcher in matchers:
            hits += matcher.hits
            misses += matcher.misses
            evictions += matcher.evictions
    assert evictions > 0, "the tiny-cache run must actually evict"
    assert hits > misses, "the suite must exercise warm transitions"
    print(dict(graphs=graphs, alphabet_bytes=len(ALPH), max_input_length=3,
               cache_caps=[None, 8], executions=runs, prefix_checks=prefix_checks,
               positive_final_outcomes=positives, cache_hits=hits,
               cache_misses=misses, cache_evictions=evictions))


def mutations():
    cases = (
        ("lost-cr", [13, "StartCRLF", 97], (13, 97)),
        ("lost-lf", [10, "WordAscii", "StartLF", 97], (10, 97)),
        ("lost-start", ["WordAscii", "Start", 97], (97,)),
        ("optimistic-eof", [97, "End", "WordAsciiNegate"], (97,)),
    )
    for name, items, data in cases:
        graph = simple(items, name)
        expected = complete_oracle(graph, data)
        correct = Cached(graph)
        correct.chunk(data)
        assert correct.finish() == expected
        broken = Cached(graph, mutation=name)
        broken.chunk(data)
        assert broken.finish() != expected, (name, "mutation survived")
        print(dict(mutation=name, input=list(data), expected=expected,
                   mutated=broken.finish()))


if __name__ == "__main__":
    audit()
    mutations()
