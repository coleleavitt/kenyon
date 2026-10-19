# Wiring study: post-hoc multiplicity correction

Not preregistered (PREREG.md applied none). Family: every reported contrast row, both graphs (160 tests). Two-sided paired t (df = 19) from the stored d_z; Holm (FWER) and Benjamini-Hochberg (FDR), both at 0.05.

Sanity check: raw p < 0.05 disagrees with the stored 95% t-CI call in 0 of 160 rows.

- Detected at raw 0.05: 81
- Survive Holm: 42
- Survive BH (FDR 5%): 73

## The preregistered contrasts on the primary metrics (AP/R)

| graph | metric | contrast | diff | d_z | p | p Holm | q BH |
|---|---|---|---|---|---|---|---|
| pooled | T1 mAP bio k=5% (AP/R) | P1 wiring beyond degrees | -0.0155 | -1.17 | 4.5e-05 | 0.0059 | 0.00023 |
| pooled | T1 mAP bio k=5% (AP/R) | P2 degree heterogeneity | -0.0404 | -4.70 | 1.3e-14 | 2e-12 | 1e-12 |
| pooled | T1 mAP bio k=5% (AP/R) | P3 weights | -0.0146 | -1.16 | 5.2e-05 | 0.0066 | 0.00024 |
| pooled | T1 mAP k=16 valued (AP/R) | P1 wiring beyond degrees | -0.0054 | -0.36 | 0.12 | 1 | 0.21 |
| pooled | T1 mAP k=16 valued (AP/R) | P2 degree heterogeneity | -0.0083 | -0.60 | 0.015 | 1 | 0.035 |
| pooled | T1 mAP k=16 valued (AP/R) | P3 weights | -0.0078 | -0.72 | 0.0043 | 0.43 | 0.011 |
| pooled | T2 odour mAP bio k=5% (AP/R) | P1 wiring beyond degrees | -0.0125 | -1.31 | 1.2e-05 | 0.0016 | 7.1e-05 |
| pooled | T2 odour mAP bio k=5% (AP/R) | P2 degree heterogeneity | +0.0247 | +1.56 | 1.2e-06 | 0.00017 | 1.1e-05 |
| pooled | T2 odour mAP bio k=5% (AP/R) | P3 weights | +0.0172 | +2.04 | 2.3e-08 | 3.4e-06 | 3.6e-07 |
| pooled | T2 odour mAP k=16 valued (AP/R) | P1 wiring beyond degrees | -0.0100 | -0.46 | 0.055 | 1 | 0.11 |
| pooled | T2 odour mAP k=16 valued (AP/R) | P2 degree heterogeneity | -0.0026 | -0.11 | 0.64 | 1 | 0.77 |
| pooled | T2 odour mAP k=16 valued (AP/R) | P3 weights | -0.0143 | -0.96 | 0.00037 | 0.045 | 0.0014 |
| pooled | T3 novelty MNIST-PN r (k=20) | P1 wiring beyond degrees | +0.0229 | +0.26 | 0.25 | 1 | 0.38 |
| pooled | T3 novelty MNIST-PN r (k=20) | P2 degree heterogeneity | -0.0286 | -0.33 | 0.15 | 1 | 0.26 |
| pooled | T3 novelty MNIST-PN r (k=20) | P3 weights | +0.0105 | +0.14 | 0.53 | 1 | 0.68 |
| pooled | T3 novelty odour r (k=20) | P1 wiring beyond degrees | +0.0647 | +0.90 | 0.00073 | 0.083 | 0.0024 |
| pooled | T3 novelty odour r (k=20) | P2 degree heterogeneity | -0.0327 | -0.48 | 0.044 | 1 | 0.089 |
| pooled | T3 novelty odour r (k=20) | P3 weights | +0.0749 | +1.17 | 4.5e-05 | 0.0059 | 0.00023 |
| right | T1 mAP bio k=5% (AP/R) | P1 wiring beyond degrees | -0.0058 | -0.62 | 0.012 | 1 | 0.03 |
| right | T1 mAP bio k=5% (AP/R) | P2 degree heterogeneity | -0.0506 | -3.65 | 1.2e-12 | 1.9e-10 | 3.3e-11 |
| right | T1 mAP bio k=5% (AP/R) | P3 weights | -0.0103 | -1.26 | 1.9e-05 | 0.0026 | 0.00011 |
| right | T1 mAP k=16 valued (AP/R) | P1 wiring beyond degrees | -0.0067 | -0.71 | 0.0048 | 0.48 | 0.012 |
| right | T1 mAP k=16 valued (AP/R) | P2 degree heterogeneity | -0.0082 | -0.53 | 0.028 | 1 | 0.059 |
| right | T1 mAP k=16 valued (AP/R) | P3 weights | -0.0067 | -0.76 | 0.0029 | 0.3 | 0.008 |
| right | T2 odour mAP bio k=5% (AP/R) | P1 wiring beyond degrees | +0.0032 | +0.26 | 0.26 | 1 | 0.38 |
| right | T2 odour mAP bio k=5% (AP/R) | P2 degree heterogeneity | +0.0213 | +1.01 | 0.00024 | 0.03 | 0.00098 |
| right | T2 odour mAP bio k=5% (AP/R) | P3 weights | -0.0241 | -2.12 | 1.2e-08 | 1.8e-06 | 2.2e-07 |
| right | T2 odour mAP k=16 valued (AP/R) | P1 wiring beyond degrees | -0.0094 | -0.59 | 0.017 | 1 | 0.038 |
| right | T2 odour mAP k=16 valued (AP/R) | P2 degree heterogeneity | +0.0103 | +0.31 | 0.18 | 1 | 0.29 |
| right | T2 odour mAP k=16 valued (AP/R) | P3 weights | +0.0402 | +1.67 | 4.5e-07 | 6.6e-05 | 5.1e-06 |
| right | T3 novelty MNIST-PN r (k=20) | P1 wiring beyond degrees | +0.0162 | +0.17 | 0.44 | 1 | 0.6 |
| right | T3 novelty MNIST-PN r (k=20) | P2 degree heterogeneity | -0.0209 | -0.25 | 0.28 | 1 | 0.4 |
| right | T3 novelty MNIST-PN r (k=20) | P3 weights | -0.0121 | -0.14 | 0.55 | 1 | 0.68 |
| right | T3 novelty odour r (k=20) | P1 wiring beyond degrees | -0.0447 | -0.82 | 0.0016 | 0.17 | 0.0049 |
| right | T3 novelty odour r (k=20) | P2 degree heterogeneity | -0.0364 | -0.48 | 0.045 | 1 | 0.089 |
| right | T3 novelty odour r (k=20) | P3 weights | +0.0196 | +0.24 | 0.29 | 1 | 0.42 |

## Calls that change under correction (raw 0.05 detected, not surviving Holm)

- pooled · T1 mAP k=16 valued (AP/R) · P2 degree heterogeneity: diff -0.0083, p 0.015, Holm 1, BH 0.035
- pooled · T1 mAP k=16 valued (AP/R) · P3 weights: diff -0.0078, p 0.0043, Holm 0.43, BH 0.011
- pooled · T1 mAP k=16 valued (AP/R) · real vs uniform (binary): diff -0.0137, p 0.0046, Holm 0.46, BH 0.012
- pooled · T1 mAP k=16 valued (AP/R) · real vs deg-preserving (weighted): diff -0.0136, p 0.00061, Holm 0.07, BH 0.0021
- pooled · T1 mAP k=16 valued (AP/hits) · P2 degree heterogeneity: diff -0.0130, p 0.0026, Holm 0.27, BH 0.0076
- pooled · T1 mAP k=16 valued (AP/hits) · P3 weights: diff -0.0136, p 0.00048, Holm 0.056, BH 0.0018
- pooled · T1 mAP k=16 valued (AP/hits) · real vs uniform (binary): diff -0.0207, p 0.00073, Holm 0.083, BH 0.0024
- pooled · T2 odour mAP bio k=5% (AP/hits) · P1 wiring beyond degrees: diff +0.0040, p 0.045, Holm 1, BH 0.09
- pooled · T2 odour mAP bio k=5% (AP/hits) · P3 weights: diff +0.0053, p 0.028, Holm 1, BH 0.059
- pooled · T2 odour mAP k=16 valued (AP/R) · real vs uniform (binary): diff -0.0126, p 0.0028, Holm 0.3, BH 0.008
- pooled · T2 odour mAP k=16 valued (AP/R) · real vs deg-preserving (weighted): diff -0.0140, p 0.0092, Holm 0.87, BH 0.022
- pooled · T2 odour mAP k=16 valued (AP/hits) · P3 weights: diff -0.0120, p 0.0028, Holm 0.29, BH 0.008
- pooled · T2 odour mAP k=16 valued (AP/hits) · real vs uniform (binary): diff -0.0151, p 0.00059, Holm 0.068, BH 0.0021
- pooled · T3 novelty MNIST-PN r (k=40) · P2 degree heterogeneity: diff -0.0346, p 0.0039, Holm 0.4, BH 0.011
- pooled · T3 novelty odour r (k=10) · P2 degree heterogeneity: diff -0.0494, p 0.0056, Holm 0.54, BH 0.014
- pooled · T3 novelty odour r (k=20) · P1 wiring beyond degrees: diff +0.0647, p 0.00073, Holm 0.083, BH 0.0024
- pooled · T3 novelty odour r (k=20) · P2 degree heterogeneity: diff -0.0327, p 0.044, Holm 1, BH 0.089
- pooled · T3 novelty odour r (k=20) · real vs uniform (binary): diff +0.0320, p 0.018, Holm 1, BH 0.04
- pooled · T3 novelty odour r (k=5) · P2 degree heterogeneity: diff -0.0656, p 0.025, Holm 1, BH 0.054
- right · T1 mAP bio k=5% (AP/R) · P1 wiring beyond degrees: diff -0.0058, p 0.012, Holm 1, BH 0.03
- right · T1 mAP bio k=5% (AP/hits) · P1 wiring beyond degrees: diff -0.0048, p 0.0081, Holm 0.78, BH 0.02
- right · T1 mAP k=16 valued (AP/R) · P1 wiring beyond degrees: diff -0.0067, p 0.0048, Holm 0.48, BH 0.012
- right · T1 mAP k=16 valued (AP/R) · P2 degree heterogeneity: diff -0.0082, p 0.028, Holm 1, BH 0.059
- right · T1 mAP k=16 valued (AP/R) · P3 weights: diff -0.0067, p 0.0029, Holm 0.3, BH 0.008
- right · T1 mAP k=16 valued (AP/R) · real vs deg-preserving (weighted): diff -0.0077, p 0.005, Holm 0.49, BH 0.013
- right · T1 mAP k=16 valued (AP/hits) · P1 wiring beyond degrees: diff -0.0090, p 0.001, Holm 0.11, BH 0.0033
- right · T1 mAP k=16 valued (AP/hits) · P2 degree heterogeneity: diff -0.0119, p 0.0041, Holm 0.42, BH 0.011
- right · T1 mAP k=16 valued (AP/hits) · P3 weights: diff -0.0095, p 0.0012, Holm 0.14, BH 0.0039
- right · T1 mAP k=16 valued (AP/hits) · real vs deg-preserving (weighted): diff -0.0119, p 0.00096, Holm 0.11, BH 0.0031
- right · T2 odour mAP bio k=5% (AP/hits) · real vs uniform (binary): diff -0.0059, p 0.026, Holm 1, BH 0.055
- right · T2 odour mAP bio k=5% (AP/hits) · real vs deg-preserving (weighted): diff +0.0052, p 0.018, Holm 1, BH 0.04
- right · T2 odour mAP k=16 valued (AP/R) · P1 wiring beyond degrees: diff -0.0094, p 0.017, Holm 1, BH 0.038
- right · T2 odour mAP k=16 valued (AP/hits) · real vs deg-preserving (weighted): diff +0.0167, p 0.0015, Holm 0.17, BH 0.0048
- right · T3 novelty odour r (k=10) · P1 wiring beyond degrees: diff -0.0429, p 0.014, Holm 1, BH 0.034
- right · T3 novelty odour r (k=10) · real vs deg-preserving (weighted): diff -0.0580, p 0.00053, Holm 0.062, BH 0.0019
- right · T3 novelty odour r (k=20) · P1 wiring beyond degrees: diff -0.0447, p 0.0016, Holm 0.17, BH 0.0049
- right · T3 novelty odour r (k=20) · P2 degree heterogeneity: diff -0.0364, p 0.045, Holm 1, BH 0.089
- right · T3 novelty odour r (k=40) · real vs uniform (binary): diff -0.0321, p 0.018, Holm 1, BH 0.04
- right · T3 novelty odour r (k=5) · P3 weights: diff -0.0383, p 0.039, Holm 1, BH 0.08
