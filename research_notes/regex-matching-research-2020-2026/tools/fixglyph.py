import sys, glob, re, unicodedata, collections
d = sys.argv[1]; bad = sys.argv[2:]
LIG = {"ﬀ":"ff","ﬁ":"fi","ﬂ":"fl","ﬃ":"ffi","ﬄ":"ffl","ﬅ":"st","ﬆ":"st"}
files = sorted(glob.glob(d+"/*.md")); files=[f for f in files if not f.endswith("INDEX.md")]
# 1) expand ligature code points everywhere
n=0
for f in files:
    s=open(f,encoding="utf-8").read(); t=s
    for k,v in LIG.items(): t=t.replace(k,v)
    if t!=s: open(f,"w",encoding="utf-8").write(t); n+=1
print("ligatures expanded in",n,"files")
# 2) vocabulary from clean files
vocab=collections.Counter()
for f in files:
    if any(f.endswith(b) for b in bad): continue
    vocab.update(w.lower() for w in re.findall(r"[A-Za-z]{3,}", open(f,encoding="utf-8").read()))
cands=["fi","ff","ffi","fl","ffl","ft"]
# words too rare in the corpus for the vocabulary check; decoded by hand against the rendered PDF pages
MANUAL={'"lenames':"filenames",'con"nes':"confines",'pre"xminimal':"prefix-minimal",'userspeci"ed':"user-specified",
 'insigni"cantly':"insignificantly",'insigni"cant':"insignificant",'La#erty':"Lafferty",'Geo#':"Geoff",'Se!':"Seffi",
 'o!':"off",'o#':"off",'de"nedas':"defined as",'pre"xof':"prefix of",'"https':'"https'}
tok=re.compile(r'(?<![A-Za-z0-9])([A-Za-z]*)([!#"$])([a-z]*)(?![A-Za-z0-9])')
for b in bad:
    f=d+"/"+b; s=open(f,encoding="utf-8").read(); fixed=collections.Counter(); left=collections.Counter()
    def rep(m):
        pre,g,post=m.groups()
        if not pre and not post: return m.group(0)
        if pre+g+post in MANUAL: fixed[pre+g+post]+=1; return MANUAL[pre+g+post]
        best=max(cands,key=lambda c: vocab.get((pre+c+post).lower(),0))
        if vocab.get((pre+best+post).lower(),0)>=2:
            fixed[pre+g+post]+=1; return pre+best+post
        left[pre+g+post]+=1; return m.group(0)
    s=tok.sub(rep,s)
    s=s.replace('compu"\ntational','computational')   # hyphenation glyph at a line break
    open(f,"w",encoding="utf-8").write(s)
    print(b,"fixed",sum(fixed.values()),"left",dict(left.most_common(15)))
