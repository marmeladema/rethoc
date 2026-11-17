#!/usr/bin/env python3
"""Independent grouped-NFA assertion scheduling model.

This is not a Rust engine implementation or an external regex oracle. Assertion
semantics are transcribed from rethoc-engine/src/lib.rs AssertKind::eval at
b0bb981. Compare an explicit scalar parked-assertion interpreter against a
set-row topological raw-frontier interpreter. The generated graphs are flat,
finite, one-counter graphs with maximum 2 and consuming wildcard bodies.
Machine-word carries, production graph analysis, unbounded saturation, and
allocation behavior are outside this model's scope.

Run under scripts/capped. Default runs both exhaustive suites and five fault
injections. The long suite overlaps the short suite deliberately.
"""

from itertools import product
from collections import deque

KINDS = (
    "Start",
    "End",
    "StartLF",
    "EndLF",
    "StartCRLF",
    "EndCRLF",
    "WordAscii",
    "WordAsciiNegate",
    "WordStartAscii",
    "WordEndAscii",
)
ALPH = (97, 32, 13, 10, 255)


def word(b):
    return b is not None and (
        65 <= b <= 90 or 97 <= b <= 122 or 48 <= b <= 57 or b == 95
    )


def ev(k, start, end, prev, nxt):
    if k == "Start":
        return start
    if k == "End":
        return end
    if k == "StartLF":
        return start or prev == 10
    if k == "EndLF":
        return True if end else None if nxt is None else nxt == 10
    if k == "StartCRLF":
        if start or prev == 10:
            return True
        if prev != 13:
            return False
        return True if end else None if nxt is None else nxt != 10
    if k == "EndCRLF":
        if end:
            return True
        return None if nxt is None else nxt == 13 or (nxt == 10 and prev != 13)
    pw = word(prev)
    if k == "WordAscii":
        return (pw if end else None) if nxt is None else pw != word(nxt)
    if k == "WordAsciiNegate":
        return (not pw if end else None) if nxt is None else pw == word(nxt)
    if k == "WordStartAscii":
        return False if pw else (False if end else None) if nxt is None else word(nxt)
    if k == "WordEndAscii":
        return (
            False
            if not pw
            else (True if end else None)
            if nxt is None
            else not word(nxt)
        )
    raise ValueError(k)


# Stable unknown-next PASS requirement independently enumerated over all bytes.
stable = 0
for k in KINDS:
    for prev in (None, *range(256)):
        start = prev is None
        if ev(k, start, False, prev, None) is True:
            for nxt in range(256):
                assert ev(k, start, False, prev, nxt) is True, (k, prev, nxt)
                stable += 1
            assert ev(k, start, True, prev, None) is True, (k, prev, "EOI")


class Graph:
    def __init__(self):
        self.nodes = []

    def add(self, *v):
        self.nodes.append(v)
        return len(self.nodes) - 1

    def chain(self, items, out):
        for x in reversed(items):
            out = self.add("a", x, out) if x in KINDS else self.add("b", x, out)
        return out

    def topo(self):
        indeg = [0] * len(self.nodes)
        adj = []
        for n in self.nodes:
            targets = (
                n[1:]
                if n[0] == "s"
                else n[2:]
                if n[0] == "a"
                else n[1:]
                if n[0] == "i"
                else n[3:]
                if n[0] == "c"
                else ()
            )
            adj.append(targets)
            for t in targets:
                indeg[t] += 1
        q = deque(i for i, x in enumerate(indeg) if not x)
        order = []
        while q:
            i = q.popleft()
            order.append(i)
            for j in adj[i]:
                indeg[j] -= 1
                if not indeg[j]:
                    q.append(j)
        assert len(order) == len(self.nodes)
        return order


def make(a, b, mode):
    g = Graph()
    m = g.add("m")
    suffix = g.chain(
        ([b, a] if mode % 3 == 0 else [b]) + ([] if mode % 2 else ["any"]), m
    )
    inc = g.add("c", 1 if mode % 2 else 2, 2, None, suffix)
    body = g.chain(
        ([a] if mode % 4 < 2 else []) + ["any"] + ([b] if mode % 4 >= 2 else []), inc
    )
    # A second body path creates distinct counts and repeated predicates.
    if mode % 3 == 1:
        other = g.chain(["any", "any"], inc)
        body = g.add("s", body, other)
    g.nodes[inc] = ("c", 1 if mode % 2 else 2, 2, body, suffix)
    ci = g.add("i", body)
    if mode % 5 == 0:
        ci = g.add("s", ci, suffix)
    start = g.chain(
        ([b, a] if mode % 3 == 2 else [a]) + ([] if mode % 2 else ["any"]), ci
    )
    if mode % 7 == 0:
        start = g.add("s", start, g.chain([b, a], m))
    g.start = start
    g.order = g.topo()
    return g


def matchbyte(pred, b):
    return pred == "any" or pred == b


# Scalar reference: ordinary stack closure, retaining assertions (even Fail),
# and consuming states as persistent terminals. Counter increments executed once
# as threads advance; only assertion terminals are revisited at next revelation.
def scalar_close(g, roots, pos, prev, nxt, end):
    stack = list(roots)
    seen = set()
    term = set()
    matched = False
    while stack:
        q, v = stack.pop()
        if (q, v) in seen:
            continue
        seen.add((q, v))
        n = g.nodes[q]
        op = n[0]
        if op == "m":
            matched = True
        elif op == "b":
            term.add((q, v))
        elif op == "a":
            term.add((q, v))
            if ev(n[1], pos == 0, end, prev, nxt) is True:
                stack.append((n[2], v))
        elif op == "s":
            stack.extend((t, v) for t in n[1:])
        elif op == "i":
            assert v is None
            stack.append((n[1], 0))
        elif op == "c":
            assert v is not None
            nv = v + 1
            if nv < n[2]:
                stack.append((n[3], nv))
            if nv >= n[1]:
                stack.append((n[4], None))
    return term, matched


class Scalar:
    def __init__(self, g):
        self.g = g
        self.pos = 0
        self.prev = None
        self.front, self.hit = scalar_close(g, [(g.start, None)], 0, None, None, False)

    def chunk(self, chunk):
        for b in chunk:
            if self.hit:
                return
            roots = [(q, v) for q, v in self.front if self.g.nodes[q][0] == "a"]
            more, hit = scalar_close(self.g, roots, self.pos, self.prev, b, False)
            if hit:
                self.hit = True
                return
            active = self.front | more
            raw = {
                (self.g.nodes[q][2], v)
                for q, v in active
                if self.g.nodes[q][0] == "b" and matchbyte(self.g.nodes[q][1], b)
            }
            raw.add((self.g.start, None))
            self.pos += 1
            self.prev = b
            self.front, self.hit = scalar_close(
                self.g, raw, self.pos, self.prev, None, False
            )

    def finish(self):
        if self.hit:
            return True
        roots = [(q, v) for q, v in self.front if self.g.nodes[q][0] == "a"]
        _, hit = scalar_close(self.g, roots, self.pos, self.prev, None, True)
        return hit


# Proposed grouping: topological set-row closure from unchanged RAW roots.
# Uses exact sets instead of machine bitsets to isolate the scheduling invariant.
class Grouped:
    def __init__(self, g):
        self.g = g
        self.pos = 0
        self.prev = None
        self.raw = {}
        self.hit = False
        self.close(None, False)

    def close(self, nxt, end):
        rows = [set() for _ in self.g.nodes]
        for q, vs in self.raw.items():
            rows[q] |= vs
        rows[self.g.start].add(None)
        for q in self.g.order:
            s = rows[q]
            if not s:
                continue
            n = self.g.nodes[q]
            op = n[0]
            if op == "m":
                self.hit = True
            elif op == "a":
                if ev(n[1], self.pos == 0, end, self.prev, nxt) is True:
                    rows[n[2]] |= s
            elif op == "s":
                for t in n[1:]:
                    rows[t] |= s
            elif op == "i":
                assert s == {None}
                rows[n[1]].add(0)
            elif op == "c":
                vs = {v + 1 for v in s}
                rows[n[3]] |= {v for v in vs if v < n[2]}
                if any(v >= n[1] for v in vs):
                    rows[n[4]].add(None)
        return rows

    def chunk(self, chunk):
        if not chunk:
            return
        for b in chunk:
            if self.hit:
                return
            rows = self.close(b, False)
            if self.hit:
                return
            raw = {}
            for q, n in enumerate(self.g.nodes):
                if n[0] == "b" and rows[q] and matchbyte(n[1], b):
                    raw.setdefault(n[2], set()).update(rows[q])
            self.raw = raw
            self.pos += 1
            self.prev = b
        self.close(None, False)

    def finish(self):
        self.close(None, True)
        return self.hit


def chunks(s, mask):
    out = []
    last = 0
    for i in range(1, len(s)):
        if mask >> (i - 1) & 1:
            out.append(s[last:i])
            last = i
    out.append(s[last:])
    return out


class NoEOI(Grouped):
    def finish(self):
        return self.hit


class PrevEarly(Grouped):
    def chunk(self, chunk):
        if not chunk:
            return
        for b in chunk:
            if self.hit:
                return
            self.prev = b
            rows = self.close(b, False)
            if self.hit:
                return
            raw = {}
            for q, n in enumerate(self.g.nodes):
                if n[0] == "b" and rows[q] and matchbyte(n[1], b):
                    raw.setdefault(n[2], set()).update(rows[q])
            self.raw = raw
            self.pos += 1
        self.close(None, False)


class KeepStale(Grouped):
    def chunk(self, chunk):
        if not chunk:
            return
        for b in chunk:
            if self.hit:
                return
            rows = self.close(b, False)
            if self.hit:
                return
            raw = {q: set(v) for q, v in self.raw.items()}
            for q, n in enumerate(self.g.nodes):
                if n[0] == "b" and rows[q] and matchbyte(n[1], b):
                    raw.setdefault(n[2], set()).update(rows[q])
            self.raw = raw
            self.pos += 1
            self.prev = b
        self.close(None, False)


class CorruptIncrementRoot(Grouped):
    def close(self, nxt, end):
        rows = super().close(nxt, end)
        for q, vals in list(self.raw.items()):
            if self.g.nodes[q][0] == "c":
                self.raw[q] = {v + 1 for v in vals}
        return rows


class DropBlockedAssertions(Grouped):
    def close(self, nxt, end):
        rows = super().close(nxt, end)
        self.raw = {
            q: set(s) for q, s in enumerate(rows) if s and self.g.nodes[q][0] == "b"
        }
        return rows


def simple_count():
    g = Graph()
    m = g.add("m")
    end = g.add("a", "End", m)
    inc = g.add("c", 2, 2, None, end)
    body = g.add("b", "any", inc)
    g.nodes[inc] = ("c", 2, 2, body, end)
    ci = g.add("i", body)
    g.start = g.add("a", "Start", ci)
    g.order = g.topo()
    return g


def simple(items):
    g = Graph()
    m = g.add("m")
    g.start = g.chain(items, m)
    g.order = g.topo()
    return g


def run_short():
    runs = 0
    prefix_checks = 0
    for ai, a in enumerate(KINDS):
        for bi, b in enumerate(KINDS):
            for mode in range(3):
                g = make(a, b, mode + 3 * ((ai + bi) % 3))
                for length in range(4):
                    for s in product(ALPH, repeat=length):
                        for mask in range(1 << max(0, length - 1)):
                            ref = Scalar(g)
                            got = Grouped(g)
                            assert ref.hit == got.hit, ("init", a, b, mode, s)
                            for c in chunks(s, mask):
                                for part in ((), c, ()):
                                    ref.chunk(part)
                                    got.chunk(part)
                                    prefix_checks += 1
                                    assert ref.hit == got.hit, (
                                        "prefix",
                                        a,
                                        b,
                                        mode,
                                        s,
                                        mask,
                                        c,
                                        ref.hit,
                                        got.hit,
                                    )
                            r = ref.finish()
                            v = got.finish()
                            assert r == v, ("finish", a, b, mode, s, mask, r, v)
                            runs += 1
    print(
        {
            "assertion_kinds": len(KINDS),
            "graphs": 300,
            "alphabet": ALPH,
            "input_lengths": "0..3",
            "all_partitions": True,
            "empty_chunks": True,
            "runs": runs,
            "prefix_checks": prefix_checks,
            "stable_pass_byte_checks": stable,
        }
    )


def run_long():
    runs = prefix_checks = positive = graphs = 0
    for ai, a in enumerate(KINDS):
        for bi, b in enumerate(KINDS):
            if (bi - ai) % 5:
                continue
            for mode in range(3):
                graphs += 1
                g = make(a, b, mode + 3 * ((ai + bi) % 3))
                for length in range(5):
                    for s in product(ALPH, repeat=length):
                        for mask in range(1 << max(0, length - 1)):
                            ref = Scalar(g)
                            got = Grouped(g)
                            assert ref.hit == got.hit
                            for c in chunks(s, mask):
                                for part in ((), c, ()):
                                    ref.chunk(part)
                                    got.chunk(part)
                                    prefix_checks += 1
                                    assert ref.hit == got.hit, (
                                        "prefix",
                                        a,
                                        b,
                                        mode,
                                        s,
                                        mask,
                                    )
                            r = ref.finish()
                            v = got.finish()
                            assert r == v, ("finish", a, b, mode, s, mask, r, v)
                            runs += 1
                            positive += r
    print(
        {
            "graphs": graphs,
            "input_lengths": "0..4",
            "runs": runs,
            "prefix_checks": prefix_checks,
            "positive_final_runs": positive,
        }
    )


def run_mutations():
    cases = [
        (CorruptIncrementRoot, simple_count(), (97,)),
        (KeepStale, simple(["Start", "any", "End"]), (97, 97)),
        (NoEOI, simple(["End"]), ()),
        (PrevEarly, simple(["WordAscii", "any", "End"]), (97,)),
        (DropBlockedAssertions, simple(["Start", "any", "End"]), (97,)),
    ]
    for mut, g, s in cases:
        ref = Scalar(g)
        correct = Grouped(g)
        bad = mut(g)
        for part in [(), *((x,) for x in s), ()]:
            ref.chunk(part)
            correct.chunk(part)
            bad.chunk(part)
            assert ref.hit == correct.hit
        r = ref.finish()
        good = correct.finish()
        wrong = bad.finish()
        assert r == good and wrong != r, (mut.__name__, r, good, wrong)
        print(
            {"fault": mut.__name__, "input": s, "reference": r, "mutated": wrong},
            flush=True,
        )


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--suite", choices=["short", "long", "mutations", "all"], default="all"
    )
    args = parser.parse_args()
    if args.suite in ("short", "all"):
        run_short()
    if args.suite in ("long", "all"):
        run_long()
    if args.suite in ("mutations", "all"):
        run_mutations()
