import sys, os, glob, subprocess, re
pdfdir, outdir = sys.argv[1], sys.argv[2]
os.makedirs(outdir, exist_ok=True)
meta = {}
for f in sorted(glob.glob(os.path.join(pdfdir, "index-*.md"))):
    topic = os.path.basename(f)[6:-3]
    for l in open(f, encoding="utf-8"):
        c = [x.strip() for x in l.split("|")]
        if len(c) >= 6 and c[2].endswith(".pdf"):
            m = meta.setdefault(c[2], {"paper": c[1], "source": c[3], "topics": []})
            if topic not in m["topics"]: m["topics"].append(topic)
            if "publisher version" in c[3]: m["source"] = c[3]
# PDFs that pdftotext mis-reads (duplicate hidden text layer with shifted glyphs); mutool extracts them cleanly.
MUTOOL = {"2026-pods-gawrychowski-optimal-enumeration-regular-pattern"}
OVERRIDES = {"2024-icsenier-siddiq-regexdoseval-generated-regexes.pdf": {
    "paper": "Re(gEx|DoS)Eval: Evaluating Generated Regular Expressions and their Proneness to DoS Attacks. Siddiq, Zhang, Roney, Santos. ICSE-NIER 2024",
    "source": "https://joannacss.github.io/preprints/icse_nier24-preprint.pdf (author preprint)", "topics": ["redos_backrefs_verification"]}}
meta.update(OVERRIDES)
rows = []
for pdf in sorted(glob.glob(os.path.join(pdfdir, "*.pdf"))):
    name = os.path.basename(pdf)[:-4]
    cmd = ["mutool", "draw", "-q", "-F", "txt", "-o", "-", pdf] if name in MUTOOL else ["pdftotext", "-enc", "UTF-8", pdf, "-"]
    txt = subprocess.run(cmd, capture_output=True, text=True).stdout
    pages = txt.split("\f")
    if pages and not pages[-1].strip(): pages = pages[:-1]
    m = meta.get(name + ".pdf", {"paper": "(not in index)", "source": "", "topics": []})
    doi = re.search(r"10\.\d{4,}/[^\s)]+", m["source"])
    body = []
    for i, p in enumerate(pages, 1):
        p = re.sub(r"[ \t]+\n", "\n", p)
        p = re.sub(r"\n{3,}", "\n\n", p).strip()
        body.append(f"<!-- page {i} -->\n\n{p}\n")
    hdr = ["---",
           f"file: ../pdfs/{name}.pdf",
           f"citation: \"{m['paper'].replace(chr(34), chr(39))}\"",
           f"doi: {doi.group(0) if doi else ''}",
           f"source: \"{m['source'].replace(chr(34), chr(39))}\"",
           f"topics: [{', '.join(m['topics'])}]",
           f"pages: {len(pages)}",
           "extraction: {'mutool draw -F txt' if name in MUTOOL else 'pdftotext'} (automatic; formulas, tables and figures may be garbled — check the PDF page cited in the page markers)",
           "---", ""]
    with open(os.path.join(outdir, name + ".md"), "w", encoding="utf-8") as o:
        o.write("\n".join(hdr) + "\n".join(body))
    words = sum(len(p.split()) for p in pages)
    rows.append((name, m["paper"], len(pages), words))
with open(os.path.join(outdir, "INDEX.md"), "w", encoding="utf-8") as o:
    o.write("# Full-text extracts\n\nOne markdown file per PDF in ../pdfs/, produced with pdftotext. Each file has a YAML header (citation, DOI, topics) and `<!-- page N -->` markers matching PDF pages. Text only: formulas, tables and figures may be garbled — consult the PDF for those.\n\n| File | Paper | Pages | Words |\n|---|---|---|---|\n")
    for n, p, pg, w in rows:
        o.write(f"| [{n}.md]({n}.md) | {p.replace('|','/')} | {pg} | {w} |\n")
    o.write("""
## Regenerating

From `research_notes/regex-matching-research-2020-2026/`:

    python3 -I tools/pdf2md.py pdfs text
    python3 -I tools/fixglyph.py text 2025-pvldb-zhang-ngram-selection-regex-indexing.md

`pdf2md.py` uses pdftotext, except for the PODS 2026 paper, whose PDF has a duplicate hidden text layer that only mutool extracts cleanly.
`fixglyph.py` expands typographic ligatures (ﬁ → fi) in all files and repairs the broken font glyphs (e.g. `e!cient`, `di#erent`, `trade-o!`) in the PVLDB 2025 file, using the other files' vocabulary plus a hand-checked list for rare words.
""")
print(len(rows))
for n, p, pg, w in rows:
    if w < 1500 or p == "(not in index)": print("CHECK", n, pg, w, p[:60])
