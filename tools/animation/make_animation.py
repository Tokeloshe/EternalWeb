#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Build docs/media/eternalweb.svg (the README animation) and the GitHub Pages
site (site/index.html, from tools/animation/page.html).

The numbers on screen come from the working parts of eternalweb-core,
through one scenario (eternalweb-core/examples/film.rs):

    cargo run -p eternalweb-core --release --example film

Content addressing, integrity checks, Ed25519 signatures, reputation and
curation are real calls. Peer-to-peer replication is not implemented yet,
so that act is labelled ROADMAP on screen. Pure SVG and CSS keyframes, no
script, so GitHub plays it inline.

    python3 tools/animation/make_animation.py
"""
import json
import math
import os
import random
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
OUT = os.path.join(ROOT, "docs", "media", "eternalweb.svg")
OUT_HTML = os.path.join(ROOT, "site", "index.html")
PAGE = os.path.join(HERE, "page.html")
W, H, T = 1200, 675, 55.0
CH = []


def scenario():
    r = subprocess.run(["cargo", "run", "-q", "-p", "eternalweb-core", "--release", "--example", "film"],
                       cwd=ROOT, capture_output=True, text=True, timeout=1800)
    if r.returncode != 0:
        sys.exit(r.stderr[-2000:])
    return json.loads(r.stdout)


def test_count():
    r = subprocess.run(["cargo", "test", "--release"], cwd=ROOT, capture_output=True, text=True, timeout=1800)
    out = r.stdout + r.stderr
    if r.returncode != 0 or "FAILED" in out:
        sys.exit("cargo test did not pass; refusing to print a test count")
    return sum(int(x) for x in re.findall(r"test result: ok\. (\d+) passed", out))


# ------------------------------------------------------- css keyframes
CSS = []
_n = [0]


def f(x):
    s = f"{x:.2f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def anim(points, prop):
    _n[0] += 1
    name = f"a{_n[0]}"
    pts = sorted(points, key=lambda p: p[0])
    pre = [p for p in pts if p[0] <= 0]
    pts = [p for p in pts if 0 < p[0] < T]
    pts.insert(0, (0.0, pre[-1][1] if pre else pts[0][1]))
    pts.append((T, pts[-1][1]))
    frames, last = [], -1.0
    for t, v in pts:
        pct = max(t / T * 100, last + 0.001)
        if pct > 100:
            continue
        last = pct
        frames.append(f'{f"{pct:.3f}".rstrip("0").rstrip(".")}%{{{prop}:{v}}}')
    CSS.append(f"@keyframes {name}{{{''.join(frames)}}}.{name}{{animation:{name} {f(T)}s linear infinite}}")
    return name


def fade(t_in, t_out, d=.4, peak=1, d_out=None):
    d_out = d if d_out is None else d_out
    return anim([(t_in, 0), (t_in + d, peak), (t_out - d_out, peak), (t_out, 0)], "opacity")


def show(t_in, t_out):
    return anim([(t_in - .01, 0), (t_in, 1), (t_out - .01, 1), (t_out, 0)], "opacity")


def move(samples):
    return anim([(t, f"translate({f(x)}px,{f(y)}px)") for t, x, y in samples], "transform")


def combos(svg):
    seen, rules = set(), []
    for m in re.finditer(r'class="((?:a\d+ )+a\d+)"', svg):
        names = m.group(1).split()
        key = " ".join(names)
        if key not in seen:
            seen.add(key)
            rules.append("." + ".".join(names) + "{animation:" +
                         ",".join(f"{n} {f(T)}s linear infinite" for n in names) + "}")
    return "".join(rules)


def esc(s):
    return str(s).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


MONO = "ui-monospace,'SF Mono',SFMono-Regular,Menlo,Consolas,'Liberation Mono',monospace"
SANS = "ui-sans-serif,-apple-system,'Segoe UI',Helvetica,Arial,sans-serif"
INK, BODY, DIM, FAINT = "#EEF2F7", "#C8D1DE", "#8792A7", "#2A3244"
TEAL, AMBER, ROSE, VIOLET, SKY = "#5EEAD4", "#FBBF24", "#F87171", "#A78BFA", "#7DD3FC"


def text(x, y, s, size=13, fill=INK, family=MONO, weight=400, anchor="start", ls=0, cls="", halo=False):
    c = f' class="{cls}"' if cls else ""
    l = f' letter-spacing="{ls}"' if ls else ""
    h = ' stroke="#080B11" stroke-width="4" stroke-linejoin="round" paint-order="stroke"' if halo else ""
    return (f'<text x="{f(x)}" y="{f(y)}" font-family="{family}" font-size="{size}" font-weight="{weight}" '
            f'fill="{fill}" text-anchor="{anchor}"{l}{c}{h}>{esc(s)}</text>')


def build(d, tests):
    out = []
    A = out.append
    A('<rect width="1200" height="675" fill="url(#bg)"/>')
    rnd = random.Random(2026)
    A('<g fill="#CFE0FF">')
    for _ in range(120):
        A(f'<circle cx="{f(rnd.uniform(0, W))}" cy="{f(rnd.uniform(0, H))}" r="{rnd.choice([.5, .6, .8])}" '
          f'opacity="{f(rnd.uniform(.05, .2))}"/>')
    A('</g>')

    def heading(t0, t1, title, sub, tag=None):
        A(f'<g class="{fade(t0, t1, .5)}">' + text(64, 112, title, 14, INK, SANS, 600, ls=3)
          + text(64, 136, sub, 11.5, DIM, ls=.3)
          + (f'<rect x="{1160 - 12 - len(tag) * 8.6}" y="96" width="{12 + len(tag) * 8.6}" height="22" rx="11" '
             f'fill="{VIOLET}" fill-opacity=".14" stroke="{VIOLET}"/>'
             + text(1160 - 6 - len(tag) * 4.3, 111, tag, 10, VIOLET, weight=700, anchor="middle", ls=1.6) if tag else "")
          + '</g>')

    # ============================================== ACT I: link rot
    I0, I1 = 2.3, 10.2
    CH.append((I0 + .2, "Link rot"))
    heading(I0, I1, "A URL NAMES A PLACE, NOT A THING", "places change; what they pointed to disappears")
    bx, by, bw, bh = 250, 168, 700, 360
    A(f'<g class="{fade(I0, I1, .5)}">')
    A(f'<rect x="{bx}" y="{by}" width="{bw}" height="{bh}" rx="12" fill="#0F1520" stroke="{FAINT}"/>')
    A(f'<rect x="{bx}" y="{by}" width="{bw}" height="40" rx="12" fill="#151C2A"/>'
      f'<rect x="{bx}" y="{by + 28}" width="{bw}" height="12" fill="#151C2A"/>')
    for i, c in enumerate(("#F87171", "#FBBF24", "#5EEAD4")):
        A(f'<circle cx="{bx + 22 + i * 16}" cy="{by + 20}" r="5" fill="{c}" opacity=".55"/>')
    A(f'<rect x="{bx + 82}" y="{by + 9}" width="440" height="22" rx="11" fill="#0B1019"/>')
    A(text(bx + 100, by + 24, "county.example/gauge/station-14", 11.5, BODY))
    title = d["page"]["title"]
    tA, tB, tC = I0 + .4, I0 + 3.0, I0 + 5.4
    A(f'<g class="{fade(tA, tC, .3, 1, .25)}">' + text(bx + 40, by + 92, title, 22, INK, SANS, 600)
      + text(bx + 40, by + 128, "Peak level 4.82 m on 12 March.", 14, BODY, SANS) + '</g>')
    A(text(bx + 40, by + 152, "Readings every 15 minutes since 1998, published by the county", 14, BODY, SANS,
           cls=fade(tA, tB, .3, 1, .25)))
    A(text(bx + 40, by + 174, "for anyone to download.", 14, BODY, SANS, cls=fade(tA, tB, .3, 1, .25)))
    A(text(bx + 40, by + 152, "Readings every 15 minutes since 1998.", 14, BODY, SANS, cls=fade(tB, tC, .25)))
    A(text(bx + 40, by + 174, "Historical data is no longer available.", 14, ROSE, SANS, cls=fade(tB, tC, .25)))
    # the readings chart, illustrative
    pts = []
    r2 = random.Random(14)
    level = 2.4
    for i in range(80):
        level += r2.uniform(-.18, .2) + (.5 if i == 54 else 0) - (.3 if i == 58 else 0)
        level = max(1.2, min(4.9, level))
        pts.append((bx + 40 + i * 7.6, by + 330 - level * 26))
    A(f'<polyline points="{" ".join(f"{f(x)},{f(y)}" for x, y in pts)}" fill="none" stroke="{SKY}" '
      f'stroke-width="1.6" class="{fade(tA, tB, .3, .9, .25)}"/>')
    A(f'<g class="{fade(tC, I1, .25)}">' + text(bx + bw / 2, by + 200, "404", 64, ROSE, SANS, 200, anchor="middle", ls=8)
      + text(bx + bw / 2, by + 236, "Not Found", 16, DIM, SANS, anchor="middle", ls=3) + '</g>')
    for t0, t1, s in ((tA, tB, "MARCH 2026"), (tB, tC, "SEPTEMBER 2026"), (tC, I1, "2027")):
        A(text(bx + bw - 24, by + 25, s, 10.5, AMBER, anchor="end", ls=2, cls=show(t0, t1)))
    A(text(600, 572, "the address never changed. what it pointed to did.", 13, DIM, anchor="middle", ls=.6,
           cls=fade(tC + .3, I1, .3)))
    A('</g>')

    # ========================================= ACT II: content address
    II0, II1 = I1 - .2, 22.0
    CH.append((II0 + .3, "Content addressing"))
    heading(II0, II1, "NAME IT BY WHAT IT IS", "the address is a fingerprint of the bytes themselves (BLAKE3, CIDv1)")
    A(f'<g class="{fade(II0, II1, .5)}">')
    # document
    A(f'<rect x="64" y="200" width="150" height="190" rx="8" fill="#0F1520" stroke="{FAINT}"/>')
    A(text(80, 226, "station-14.html", 10, DIM))
    A(text(80, 244, f'{d["page"]["bytes_v1"]} bytes', 10, DIM))
    for k in range(8):
        A(f'<rect x="80" y="{258 + k * 14}" width="{f(100 - (k * 23) % 50)}" height="5" rx="2.5" fill="{BODY}" opacity=".35"/>')
    # byte stream into the hash
    r3 = random.Random(3)
    hexes = [f"{r3.randrange(256):02x}" for _ in range(28)]
    for k, hx in enumerate(hexes):
        t0 = II0 + .6 + k * .06
        mv = move([(t0, 0, 0), (t0 + 1.2, 300, 0)])
        op = anim([(t0 - .01, 0), (t0, .9), (t0 + 1.0, .9), (t0 + 1.2, 0)], "opacity")
        A(f'<g class="{op}" opacity="0"><g class="{mv}">' + text(222, 300 + (k % 3 - 1) * 16, hx, 11, SKY) + '</g></g>')
    A(f'<rect x="530" y="262" width="140" height="76" rx="12" fill="#101826" stroke="{TEAL}" stroke-opacity=".6"/>')
    A(text(600, 298, "BLAKE3", 14, TEAL, SANS, 700, anchor="middle", ls=3))
    A(text(600, 318, "multihash · CIDv1", 9.5, DIM, anchor="middle", ls=.6))
    pulse = []
    t = II0 + .6
    while t < II0 + 2.4:
        pulse += [(t, .25), (t + .15, .9), (t + .3, .25)]
        t += .3
    A(f'<rect x="530" y="262" width="140" height="76" rx="12" fill="{TEAL}" class="{anim(pulse, "opacity")}" '
      f'opacity="0" fill-opacity=".12"/>')

    def cid_row(cid, y, t0, ref=None, size=17, col=INK):
        cw = size * .61
        x0 = 600 - len(cid) * cw / 2
        for i, ch in enumerate(cid):
            diff = ref is not None and ref[i] != ch
            c = ROSE if diff else col
            A(text(x0 + i * cw, y, ch, size, c, weight=600 if diff else 400, cls=fade(t0 + i * .018, II1, .08)))
        return x0, cw

    T_CID = II0 + 2.3
    A(text(600, 418, "ITS ADDRESS", 9.5, DIM, anchor="middle", ls=2, cls=fade(T_CID, II1, .3)))
    cid_row(d["cid_v1"], 446, T_CID)
    # one character
    T_ONE = II0 + 4.6
    A(f'<g class="{fade(T_ONE, II1, .3)}">'
      + text(64, 470, "change one character", 9.5, DIM, ls=1.6)
      + text(64, 492, "Peak level 4.8", 13, BODY, SANS)
      + '</g>')
    A(text(154, 492, "2", 13, BODY, SANS, cls=fade(T_ONE, T_ONE + .8, .2)))
    A(text(154, 492, "3", 13, ROSE, SANS, 700, cls=fade(T_ONE + .8, II1, .2)))
    A(text(167, 492, "m", 13, BODY, SANS, cls=fade(T_ONE, II1, .3)))
    cid_row(d["cid_one_char"], 528, T_ONE + 1.2, ref=d["cid_v1"])
    diffs = sum(1 for a, b in zip(d["cid_v1"], d["cid_one_char"]) if a != b)
    A(text(600, 556, f"{diffs} of {len(d['cid_v1'])} characters change · a different thing gets a different name",
           11, DIM, anchor="middle", cls=fade(T_ONE + 2.2, II1, .3)))
    T_VER = II0 + 8.4
    ok1, ok2 = d["integrity"]["original"], d["integrity"]["one_char_changed"]
    A(f'<g class="{fade(T_VER, II1, .3)}">'
      f'<rect x="760" y="186" width="380" height="96" rx="10" fill="#0F1520" stroke="{FAINT}"/>'
      + text(780, 212, "verify_integrity(address, bytes)", 10.5, DIM)
      + text(780, 240, "original bytes", 12.5, BODY) + text(1120, 240, f'{"✓ match" if ok1 else "✗"}', 12.5, TEAL, weight=700, anchor="end")
      + text(780, 266, "one character changed", 12.5, BODY) + text(1120, 266, f'{"✗ rejected" if not ok2 else "✓"}', 12.5, ROSE, weight=700, anchor="end")
      + '</g>')
    A('</g>')

    # ============================================== ACT III: signed
    III0, III1 = II1 - .2, 29.0
    CH.append((III0 + .3, "Signed"))
    heading(III0, III1, "SIGNED BY WHOEVER KEPT IT", "an archivist's Ed25519 signature over the address")
    S = d["signature"]
    A(f'<g class="{fade(III0, III1, .5)}">')
    A(text(64, 206, "ADDRESS", 9.5, DIM, ls=2))
    A(text(64, 232, d["cid_v1"], 15, INK))
    A(text(64, 286, "ARCHIVIST'S PUBLIC KEY", 9.5, DIM, ls=2))
    A(text(64, 312, S["public_key"], 13, AMBER))
    A(text(64, 366, "SIGNATURE", 9.5, DIM, ls=2))
    sig = S["sig"]
    for row in range(2):
        chunk = sig[row * 64:(row + 1) * 64]
        for i in range(0, 64, 8):
            t0 = III0 + .9 + row * .5 + i * .02
            A(text(64 + i * 8.9, 392 + row * 24, chunk[i:i + 8], 14.5, AMBER, cls=fade(t0, III1, .12)))
    T_V = III0 + 2.9
    A(f'<g class="{fade(T_V, III1, .3)}">'
      f'<rect x="64" y="452" width="520" height="40" rx="20" fill="{TEAL}" fill-opacity=".1" stroke="{TEAL}" stroke-opacity=".6"/>'
      + text(86, 477, f'{"✓" if S["valid"] else "✗"}  valid for this address', 13, TEAL, weight=600) + '</g>')
    A(f'<g class="{fade(T_V + .8, III1, .3)}">'
      f'<rect x="64" y="504" width="520" height="40" rx="20" fill="{ROSE}" fill-opacity=".08" stroke="{ROSE}" stroke-opacity=".6"/>'
      + text(86, 529, f'{"✗" if not S["on_other_cid"] else "✓"}  copied onto the edited page: rejected', 13, ROSE, weight=600) + '</g>')
    A('</g>')

    # ============================================== ACT IV: versions
    IV0, IV1 = III1 - .2, 35.0
    CH.append((IV0 + .3, "Versions kept"))
    heading(IV0, IV1, "A REMOVAL IS A NEW VERSION, NOT AN ERASURE", "each version keeps its own address")
    A(f'<g class="{fade(IV0, IV1, .5)}">')
    A(f'<line x1="140" y1="330" x2="1060" y2="330" stroke="{FAINT}" stroke-width="2"/>')
    cards = [(240, "MARCH 2026", d["cid_v1"], "Readings every 15 minutes since 1998,", "published for anyone to download.", BODY, IV0 + .4),
             (600, "SEPTEMBER 2026", d["cid_v2"], "Readings every 15 minutes since 1998.", "Historical data is no longer available.", ROSE, IV0 + 1.3)]
    for x, when, cid, l1, l2, c2, t0 in cards:
        A(f'<g class="{fade(t0, IV1, .35)}">'
          f'<circle cx="{x}" cy="330" r="7" fill="{TEAL}"/>'
          f'<rect x="{x - 150}" y="360" width="300" height="130" rx="10" fill="#0F1520" stroke="{FAINT}"/>'
          + text(x - 132, 386, when, 10, AMBER, ls=2)
          + text(x - 132, 412, l1, 11.5, BODY, SANS) + text(x - 132, 432, l2, 11.5, c2, SANS)
          + text(x - 132, 470, cid[:22] + "…", 11, TEAL) + '</g>')
    A(f'<g class="{fade(IV0 + 2.2, IV1, .35)}">'
      f'<circle cx="960" cy="330" r="7" fill="{ROSE}"/>'
      f'<rect x="830" y="360" width="260" height="130" rx="10" fill="#160F14" stroke="{ROSE}" stroke-opacity=".5"/>'
      + text(848, 386, "2027", 10, AMBER, ls=2)
      + text(848, 414, "county.example/gauge/station-14", 11, BODY)
      + text(848, 440, "404 Not Found", 16, ROSE, SANS, 600)
      + text(848, 470, "the place is gone", 11, DIM) + '</g>')
    A(text(600, 560, f'{d["cid_v1"][:22]}… still returns the March readings, byte for byte', 13, TEAL,
           anchor="middle", ls=.3, cls=fade(IV0 + 3.2, IV1, .35)))
    A('</g>')

    # ====================================== ACT V: network (roadmap)
    V0, V1 = IV1 - .2, 43.0
    CH.append((V0 + .3, "The network (roadmap)"))
    heading(V0, V1, "EVERY READER A KEEPER", "the design: archives replicate across peers, so no single server decides what survives",
            tag="ROADMAP")
    A(f'<g class="{fade(V0, V1, .5)}">')
    rn = random.Random(11)
    nodes = []
    while len(nodes) < 46:
        x, y = rn.uniform(120, 1100), rn.uniform(180, 560)
        if all(math.dist((x, y), q) > 62 for q in nodes):
            nodes.append((x, y))
    origin = min(range(len(nodes)), key=lambda i: nodes[i][0])
    edges = set()
    for i, p in enumerate(nodes):
        near = sorted(range(len(nodes)), key=lambda j: math.dist(p, nodes[j]))[1:4]
        for j in near:
            edges.add((min(i, j), max(i, j)))
    # replication order: breadth-first from the origin
    hop = {origin: 0}
    frontier = [origin]
    adj = {i: [] for i in range(len(nodes))}
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    while frontier:
        nxt = []
        for a in frontier:
            for b in adj[a]:
                if b not in hop:
                    hop[b] = hop[a] + 1
                    nxt.append(b)
        frontier = nxt
    holders = sorted(hop, key=lambda i: (hop[i], i))[:18]
    TR0, DT = V0 + 1.2, .42
    for a, b in edges:
        A(f'<line x1="{f(nodes[a][0])}" y1="{f(nodes[a][1])}" x2="{f(nodes[b][0])}" y2="{f(nodes[b][1])}" '
          f'stroke="{VIOLET}" stroke-opacity=".18" stroke-dasharray="3 4"/>')
    for i, (x, y) in enumerate(nodes):
        A(f'<circle cx="{f(x)}" cy="{f(y)}" r="4" fill="#3B4560"/>')
    for i in holders:
        x, y = nodes[i]
        t0 = TR0 + hop[i] * DT
        A(f'<g class="{fade(t0, V1, .2)}" opacity="0"><circle cx="{f(x)}" cy="{f(y)}" r="13" fill="url(#glowT)"/>'
          f'<circle cx="{f(x)}" cy="{f(y)}" r="4.5" fill="{TEAL}"/></g>')
    ox, oy = nodes[origin]
    A(f'<rect x="{f(ox - 16)}" y="{f(oy - 16)}" width="32" height="32" rx="6" fill="#151C2A" stroke="{BODY}"/>')
    A(text(ox, oy + 5, "≡", 16, BODY, anchor="middle"))
    A(f'<g class="{fade(V0 + 4.6, V1, .3)}" opacity="0"><rect x="{f(ox - 16)}" y="{f(oy - 16)}" width="32" height="32" rx="6" '
      f'fill="#200E12" stroke="{ROSE}"/>' + text(ox, oy + 5, "✕", 15, ROSE, anchor="middle")
      + text(ox, oy + 34, "origin offline", 9.5, ROSE, anchor="middle", halo=True) + '</g>')
    def nearest_holder(i):
        return min(math.dist(nodes[i], nodes[h]) for h in holders)
    cands = [i for i in range(len(nodes)) if i not in holders and nearest_holder(i) < 170]
    far = max(cands, key=lambda i: math.dist(nodes[i], (ox, oy)))
    fx, fy = nodes[far]
    near_h = min(holders, key=lambda i: math.dist(nodes[i], (fx, fy)))
    hx, hy = nodes[near_h]
    TQ = V0 + 5.4
    A(f'<g class="{fade(TQ, V1, .3)}" opacity="0"><circle cx="{f(fx)}" cy="{f(fy)}" r="7" fill="none" stroke="{AMBER}" stroke-width="1.5"/>'
      + text(fx - 10, fy - 14, f'get {d["cid_v1"][:14]}…', 10, AMBER, anchor="end", halo=True) + '</g>')
    L = math.dist((fx, fy), (hx, hy))
    dash = anim([(TQ + .4, f(L)), (TQ + 1.1, "0")], "stroke-dashoffset")
    A(f'<line x1="{f(fx)}" y1="{f(fy)}" x2="{f(hx)}" y2="{f(hy)}" stroke="{AMBER}" stroke-width="2" '
      f'stroke-dasharray="{f(L)} {f(L)}" class="{dash} {fade(TQ + .4, V1, .1)}" opacity="0"/>')
    A(text(600, 600, "designed, not built: libp2p replication is on the roadmap. Today an archive lives in the "
           "local store of the node that made it.", 11, VIOLET, anchor="middle", ls=.2, halo=True,
           cls=fade(V0 + .8, V1, .4)))
    A('</g>')

    # =========================================== ACT VI: community
    VI0, VI1 = V1 - .2, 49.0
    CH.append((VI0 + .3, "Reputation and curation"))
    heading(VI0, VI1, "REPUTATION, NOT TOKENS", "credit for archiving, hosting and curating; no speculative coin")
    A(f'<g class="{fade(VI0, VI1, .5)}">')
    rep = d["reputation"]
    maxp = max(r["points"] for r in rep)
    lvlc = {"Newcomer": DIM, "Contributor": SKY, "Guardian": TEAL, "Sentinel": AMBER, "Eternal": VIOLET}
    A(text(64, 206, "REPUTATION · calculate_reputation()", 9.5, DIM, ls=1.6))
    for i, r in enumerate(rep):
        y = 236 + i * 52
        w = 300 * r["points"] / maxp
        g = anim([(VI0 + .5 + i * .15, "scaleX(0)"), (VI0 + 1.2 + i * .15, "scaleX(1)")], "transform")
        A(text(64, y + 14, r["name"], 14, INK, SANS, 600))
        A(f'<rect x="160" y="{y}" width="{f(w)}" height="18" rx="3" fill="{lvlc.get(r["level"], DIM)}" fill-opacity=".8" '
          f'style="transform-box:fill-box;transform-origin:left" class="{g}"/>')
        A(text(168 + w, y + 14, f'{r["points"]} · {r["level"]}', 12, lvlc.get(r["level"], DIM),
               cls=fade(VI0 + 1.1 + i * .15, VI1, .2)))
    A(text(700, 206, "CURATION OF THE MARCH ARCHIVE · quality score", 9.5, DIM, ls=1.6))
    votes = d["curation"]
    for i, v in enumerate(votes):
        t0 = VI0 + 1.0 + i * .5
        col = {"up": TEAL, "down": ROSE, "flag": AMBER}[v["vote"]]
        sym = {"up": "▲", "down": "▼", "flag": "⚑"}[v["vote"]]
        A(text(700 + i * 52, 246, sym, 22, col, SANS, cls=fade(t0, VI1, .15)))
        A(text(700 + i * 52, 270, v["vote"], 9.5, DIM, cls=fade(t0, VI1, .15)))
        nxt = VI0 + 1.0 + (i + 1) * .5 if i + 1 < len(votes) else VI1
        A(text(700, 360, f'{v["score"]:.2f}', 54, INK, SANS, 200, cls=show(t0, nxt)))
    A(text(700, 392, "upvotes ÷ (upvotes + downvotes) · flags are counted separately", 11, DIM,
           cls=fade(VI0 + 1.5, VI1, .3)))
    A('</g>')

    # ========================================= ACT VII: today / next
    VII0, VII1 = VI1 - .2, 53.4
    CH.append((VII0 + .3, "Today and next"))
    heading(VII0, VII1, "WHAT WORKS TODAY, AND WHAT'S NEXT", f"cargo test: {tests} tests pass")
    A(f'<g class="{fade(VII0, VII1, .5)}">')
    today = ["content-addressed archiving (BLAKE3, CIDv1)", "integrity checks against the address",
             "Ed25519 signatures", "versioned feeds", "reputation levels", "community curation scores",
             "command-line tool"]
    nextl = ["peer-to-peer replication (libp2p)", "semantic search", "post-quantum keys and signatures",
             "end-to-end encrypted sharing", "governance anchoring", "browser extension and apps"]
    A(text(64, 206, "WORKING TODAY", 10.5, TEAL, weight=700, ls=2))
    A(text(640, 206, "ON THE ROADMAP", 10.5, VIOLET, weight=700, ls=2))
    for i, s in enumerate(today):
        A(text(64, 244 + i * 34, "✓  " + s, 14, INK, SANS, cls=fade(VII0 + .3 + i * .12, VII1, .2)))
    for i, s in enumerate(nextl):
        A(text(640, 244 + i * 34, "○  " + s, 14, BODY, SANS, cls=fade(VII0 + .6 + i * .12, VII1, .2)))
    A('</g>')

    # ============================================================ chrome
    END0 = VII1 - .2
    A(f'<g class="{anim([(0, 1), (1.9, 1), (2.5, 0), (T - .7, 0), (T, 1)], "opacity")}">')
    A(text(600, 316, "ETERNALWEB", 48, INK, SANS, 200, anchor="middle", ls=16))
    A(f'<line x1="530" y1="340" x2="670" y2="340" stroke="{TEAL}" stroke-opacity=".8"/>')
    A(text(600, 372, "name what you keep by what it is, not where it lives", 15, DIM, SANS, anchor="middle", ls=1.6))
    A('</g>')
    A(f'<g class="{fade(END0, T - .6, .6, 1, .5)}">')
    A(text(600, 326, "Keep what matters.", 26, INK, SANS, 300, anchor="middle", ls=2))
    A(text(600, 362, "github.com/Tokeloshe/EternalWeb", 11.5, DIM, anchor="middle", ls=1.4))
    A('</g>')
    A(f'<g class="{fade(2.4, END0, .6)}">')
    A(text(40, 44, "ETERNALWEB", 12, INK, SANS, 600, ls=4))
    A(f'<line x1="158" y1="34" x2="158" y2="48" stroke="{DIM}" stroke-opacity=".5"/>')
    for t0, t1, s in ((I0, I1, "LINK ROT"), (II0, II1, "THE ADDRESS"), (III0, III1, "THE SIGNATURE"),
                      (IV0, IV1, "VERSIONS"), (V0, V1, "THE NETWORK"), (VI0, VI1, "THE COMMUNITY"),
                      (VII0, END0, "STATUS")):
        A(text(172, 44, s, 10.5, DIM, ls=2.6, cls=fade(t0, t1, .3)))
    A('</g>')
    A(f'<rect x="-1200" y="673" width="1200" height="2" fill="{TEAL}" opacity=".55" '
      f'class="{anim([(0, "translateX(0px)"), (T, "translateX(1200px)")], "transform")}"/>')

    defs = f'''<defs>
<radialGradient id="bg" cx="50%" cy="25%" r="85%"><stop offset="0" stop-color="#121A28"/><stop offset=".55" stop-color="#0A0F17"/><stop offset="1" stop-color="#06080D"/></radialGradient>
<radialGradient id="glowT"><stop offset="0" stop-color="{TEAL}" stop-opacity=".55"/><stop offset="1" stop-color="{TEAL}" stop-opacity="0"/></radialGradient>
<clipPath id="frame"><rect width="1200" height="675" rx="14"/></clipPath>
</defs>'''
    body = "".join(out)
    style = "<style>" + "".join(CSS) + combos(body) + "</style>"
    desc = ("A county flood-gauge page changes and then returns 404 at the same URL. EternalWeb names the "
            "page by its content instead: a BLAKE3 content address, which changes completely when one "
            "character changes and rejects edited bytes. An archivist signs the address with Ed25519; the "
            "signature fails on the edited page. Each version keeps its own address, so the March readings "
            "survive the removal. Peer replication is shown as the roadmap design. Reputation and curation "
            "scores come from the library, and the closing list separates what works today from what is next.")
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" role="img" '
            f'aria-labelledby="ewTitle ewDesc"><title id="ewTitle">EternalWeb in action</title>'
            f'<desc id="ewDesc">{esc(desc)}</desc>{defs}{style}<g clip-path="url(#frame)">{body}</g></svg>')


if __name__ == "__main__":
    d = scenario()
    n = test_count()
    svg = build(d, n)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write(svg)
    if os.path.exists(PAGE):
        with open(PAGE, encoding="utf-8") as fh:
            page = fh.read()
        chapters = "\n".join(f'<button data-t="{t:.1f}">{esc(label)}</button>' for t, label in CH)
        inline = svg.replace('width="1200" height="675" role="img"', 'role="img" id="film"')
        page = (page.replace("{{SVG}}", inline).replace("{{T}}", f(T)).replace("{{CHAPTERS}}", chapters)
                .replace("{{TESTS}}", str(n)).replace("{{CID}}", d["cid_v1"]))
        os.makedirs(os.path.dirname(OUT_HTML), exist_ok=True)
        with open(OUT_HTML, "w", encoding="utf-8") as fh:
            fh.write(page)
    print(f"wrote {os.path.relpath(OUT, ROOT)} ({len(svg) / 1024:.0f} KB), {n} tests", file=sys.stderr)
