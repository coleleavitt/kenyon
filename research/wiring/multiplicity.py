#!/usr/bin/env python3
"""Post-hoc multiplicity correction for the wiring study (not preregistered).

PREREG.md applied no correction (one confirmatory test, P1 on T1 with the
primary hash; everything else exploratory). For publication, this re-analysis
applies Holm (family-wise error) and Benjamini-Hochberg (false discovery rate)
to every reported contrast row, without re-running any experiment.

Each row is a paired comparison over n = 20 seeds with effect size
d_z = mean / sd of the differences, so t = d_z * sqrt(n), df = n - 1, and the
two-sided p comes from Student's t. For T2 rows the real arm is one value;
the test is then real minus each null seed, i.e. a one-sample test against
the null arm's distribution.

Usage: python3 -I research/wiring/multiplicity.py > results/wiring_multiplicity.md
Standard library only.
"""

import json
import math
import sys
from pathlib import Path


def betacf(a, b, x):
    """Continued fraction for the regularized incomplete beta (Lentz)."""
    tiny, eps = 1e-300, 3e-16
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > tiny else tiny)
    h = d
    for m in range(1, 300):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = 1.0 / (d if abs(d) > tiny else tiny)
        c = 1.0 + aa / c
        c = c if abs(c) > tiny else tiny
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < eps:
            break
    return h


def betai(a, b, x):
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log(1 - x)
    bt = math.exp(lbt)
    if x < (a + 1.0) / (a + b + 2.0):
        return bt * betacf(a, b, x) / a
    return 1.0 - bt * betacf(b, a, 1.0 - x) / b


def p_two_sided(t, df):
    return betai(df / 2.0, 0.5, df / (df + t * t))


def holm(ps):
    order = sorted(range(len(ps)), key=lambda i: ps[i])
    adj = [0.0] * len(ps)
    running = 0.0
    for rank, i in enumerate(order):
        running = max(running, min(1.0, (len(ps) - rank) * ps[i]))
        adj[i] = running
    return adj


def bh(ps):
    m = len(ps)
    order = sorted(range(m), key=lambda i: ps[i], reverse=True)
    adj = [0.0] * m
    running = 1.0
    for k, i in enumerate(order):
        rank = m - k
        running = min(running, ps[i] * m / rank)
        adj[i] = running
    return adj


def main():
    root = Path(__file__).resolve().parents[2]
    rows = []
    for graph in ("pooled", "right"):
        d = json.loads((root / f"results/wiring_{graph}.json").read_text())
        n = d["seeds"]
        for r in d["rows"]:
            dz = r["dz"]
            if dz is None or (isinstance(dz, float) and math.isnan(dz)):
                p = 1.0
            else:
                p = p_two_sided(dz * math.sqrt(n), n - 1)
            rows.append(dict(graph=graph, metric=r["metric"], contrast=r["contrast"],
                             diff=r["diff"], dz=dz, raw_detected=r["detected"], p=p))
    ps = [r["p"] for r in rows]
    for r, h, q in zip(rows, holm(ps), bh(ps)):
        r["p_holm"], r["q_bh"] = h, q

    # Sanity: the t-test p must agree with the stored 95% t-CI call.
    disagree = sum(1 for r in rows if (r["p"] < 0.05) != r["raw_detected"])

    out = sys.stdout
    out.write("# Wiring study: post-hoc multiplicity correction\n\n")
    out.write("Not preregistered (PREREG.md applied none). Family: every reported contrast row, both graphs "
              f"({len(rows)} tests). Two-sided paired t (df = 19) from the stored d_z; Holm (FWER) and "
              "Benjamini-Hochberg (FDR), both at 0.05.\n\n")
    out.write(f"Sanity check: raw p < 0.05 disagrees with the stored 95% t-CI call in {disagree} of {len(rows)} rows.\n\n")
    out.write(f"- Detected at raw 0.05: {sum(r['p'] < 0.05 for r in rows)}\n")
    out.write(f"- Survive Holm: {sum(r['p_holm'] < 0.05 for r in rows)}\n")
    out.write(f"- Survive BH (FDR 5%): {sum(r['q_bh'] < 0.05 for r in rows)}\n\n")
    out.write("## The preregistered contrasts on the primary metrics (AP/R)\n\n")
    out.write("| graph | metric | contrast | diff | d_z | p | p Holm | q BH |\n|---|---|---|---|---|---|---|---|\n")
    for r in rows:
        if "(AP/hits)" in r["metric"] or not r["contrast"].startswith(("P1", "P2", "P3")):
            continue
        if r["metric"].startswith("T3") and "(k=20)" not in r["metric"]:
            continue
        out.write(f"| {r['graph']} | {r['metric']} | {r['contrast']} | {r['diff']:+.4f} | {r['dz']:+.2f} | "
                  f"{r['p']:.2g} | {r['p_holm']:.2g} | {r['q_bh']:.2g} |\n")
    out.write("\n## Calls that change under correction (raw 0.05 detected, not surviving Holm)\n\n")
    changed = [r for r in rows if r["p"] < 0.05 and r["p_holm"] >= 0.05]
    if not changed:
        out.write("None.\n")
    for r in changed:
        out.write(f"- {r['graph']} · {r['metric']} · {r['contrast']}: diff {r['diff']:+.4f}, p {r['p']:.2g}, "
                  f"Holm {r['p_holm']:.2g}, BH {r['q_bh']:.2g}\n")


if __name__ == "__main__":
    main()
