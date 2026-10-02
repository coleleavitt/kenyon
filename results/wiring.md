# Wiring study results

## pooled

pooled: 21509 pairs, 285 PNs -> 4723 KCs; KC fan-in median 5 (p10 2, p90 6); PN out-degree mean 75.5, median 60, max 516; synapses 316862

odour input drives 103 of 285 PNs; k_bio = 236

### Arm means (sd over seeds)

| metric | R-b | R-w | U-b | U-w | D-b | D-w | S-w |
|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | 0.4846 (0.0166) | 0.4575 (0.0178) | 0.5405 (0.0149) | 0.5296 (0.0145) | 0.5000 (0.0146) | 0.4811 (0.0141) | 0.4721 (0.0158) |
| T1 mAP bio k=5% (AP/hits) | 0.7962 (0.0117) | 0.7780 (0.0129) | 0.8290 (0.0102) | 0.8228 (0.0098) | 0.8054 (0.0105) | 0.7939 (0.0102) | 0.7879 (0.0109) |
| T1 mAP k=16 valued (AP/R) | 0.1830 (0.0175) | 0.1903 (0.0126) | 0.1967 (0.0113) | 0.2154 (0.0136) | 0.1884 (0.0111) | 0.2039 (0.0139) | 0.1981 (0.0144) |
| T1 mAP k=16 valued (AP/hits) | 0.5134 (0.0206) | 0.5170 (0.0167) | 0.5341 (0.0136) | 0.5537 (0.0156) | 0.5211 (0.0144) | 0.5383 (0.0167) | 0.5306 (0.0141) |
| T2 odour mAP bio k=5% (AP/R) | 0.5553 (0.0000) | 0.5697 (0.0000) | 0.5432 (0.0123) | 0.5301 (0.0124) | 0.5679 (0.0095) | 0.5550 (0.0125) | 0.5524 (0.0085) |
| T2 odour mAP bio k=5% (AP/hits) | 0.8587 (0.0000) | 0.8574 (0.0000) | 0.8586 (0.0082) | 0.8484 (0.0114) | 0.8547 (0.0084) | 0.8501 (0.0069) | 0.8520 (0.0101) |
| T2 odour mAP k=16 valued (AP/R) | 0.4279 (0.0000) | 0.4453 (0.0000) | 0.4405 (0.0164) | 0.4417 (0.0239) | 0.4379 (0.0218) | 0.4593 (0.0215) | 0.4596 (0.0148) |
| T2 odour mAP k=16 valued (AP/hits) | 0.7936 (0.0000) | 0.7974 (0.0000) | 0.8087 (0.0164) | 0.8029 (0.0261) | 0.7971 (0.0226) | 0.8066 (0.0268) | 0.8095 (0.0156) |
| T3 novelty MNIST-PN r (k=10) | 0.1248 (0.0781) | 0.1119 (0.0669) | 0.1067 (0.0635) | 0.1244 (0.0744) | 0.1278 (0.0883) | 0.0798 (0.0900) | 0.1069 (0.0633) |
| T3 novelty MNIST-PN r (k=20) | 0.1554 (0.0809) | 0.1122 (0.0689) | 0.1611 (0.0946) | 0.1251 (0.0820) | 0.1325 (0.0847) | 0.0898 (0.0829) | 0.1016 (0.0779) |
| T3 novelty MNIST-PN r (k=40) | 0.1489 (0.0658) | 0.1210 (0.0528) | 0.1672 (0.0776) | 0.1320 (0.0727) | 0.1326 (0.0663) | 0.1082 (0.0758) | 0.1253 (0.0755) |
| T3 novelty MNIST-PN r (k=5) | 0.1050 (0.0784) | 0.0837 (0.0719) | 0.1185 (0.0799) | 0.1164 (0.0711) | 0.0979 (0.0653) | 0.1002 (0.0962) | 0.0660 (0.0585) |
| T3 novelty odour r (k=10) | 0.3990 (0.0680) | 0.3481 (0.0420) | 0.4366 (0.0665) | 0.3770 (0.0693) | 0.3872 (0.0609) | 0.3763 (0.0870) | 0.3814 (0.0805) |
| T3 novelty odour r (k=20) | 0.4658 (0.0542) | 0.4804 (0.0453) | 0.4338 (0.0517) | 0.4192 (0.0493) | 0.4011 (0.0722) | 0.3747 (0.0607) | 0.4055 (0.0608) |
| T3 novelty odour r (k=40) | 0.4238 (0.0487) | 0.4790 (0.0614) | 0.4487 (0.0674) | 0.4308 (0.0538) | 0.4293 (0.0720) | 0.4034 (0.0510) | 0.4000 (0.0647) |
| T3 novelty odour r (k=5) | 0.3176 (0.0552) | 0.3608 (0.0529) | 0.3954 (0.0635) | 0.3721 (0.0835) | 0.3298 (0.0920) | 0.3576 (0.0671) | 0.3519 (0.0764) |

### Contrasts (paired over seeds)

| metric | contrast | mean diff | 95% t-CI | 95% boot CI | d_z | null sd | detected | > null sd |
|---|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0155 | [-0.0216, -0.0093] | [-0.0212, -0.0100] | -1.17 | 0.0146 | true | true |
| T1 mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0404 | [-0.0445, -0.0364] | [-0.0441, -0.0368] | -4.70 | 0.0149 | true | true |
| T1 mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | -0.0146 | [-0.0205, -0.0087] | [-0.0201, -0.0093] | -1.16 | 0.0158 | true | false |
| T1 mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0559 | [-0.0636, -0.0482] | [-0.0630, -0.0492] | -3.39 | 0.0149 | true | true |
| T1 mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0236 | [-0.0316, -0.0156] | [-0.0311, -0.0165] | -1.38 | 0.0141 | true | true |
| T1 mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0093 | [-0.0131, -0.0054] | [-0.0129, -0.0058] | -1.13 | 0.0105 | true | false |
| T1 mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0235 | [-0.0258, -0.0212] | [-0.0256, -0.0214] | -4.80 | 0.0102 | true | true |
| T1 mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | -0.0100 | [-0.0140, -0.0059] | [-0.0137, -0.0063] | -1.16 | 0.0109 | true | false |
| T1 mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0328 | [-0.0376, -0.0280] | [-0.0373, -0.0286] | -3.19 | 0.0102 | true | true |
| T1 mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0160 | [-0.0213, -0.0106] | [-0.0210, -0.0112] | -1.39 | 0.0102 | true | true |
| T1 mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0054 | [-0.0125, +0.0016] | [-0.0115, +0.0012] | -0.36 | 0.0111 | false | false |
| T1 mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0083 | [-0.0147, -0.0018] | [-0.0138, -0.0022] | -0.60 | 0.0113 | true | false |
| T1 mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | -0.0078 | [-0.0129, -0.0028] | [-0.0123, -0.0031] | -0.72 | 0.0144 | true | false |
| T1 mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0137 | [-0.0226, -0.0048] | [-0.0215, -0.0048] | -0.72 | 0.0113 | true | true |
| T1 mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0136 | [-0.0206, -0.0067] | [-0.0198, -0.0070] | -0.92 | 0.0139 | true | false |
| T1 mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0077 | [-0.0165, +0.0011] | [-0.0153, +0.0005] | -0.41 | 0.0144 | false | false |
| T1 mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0130 | [-0.0209, -0.0052] | [-0.0196, -0.0056] | -0.78 | 0.0136 | true | false |
| T1 mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | -0.0136 | [-0.0204, -0.0068] | [-0.0198, -0.0074] | -0.94 | 0.0141 | true | false |
| T1 mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0207 | [-0.0315, -0.0099] | [-0.0306, -0.0103] | -0.90 | 0.0136 | true | true |
| T1 mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0213 | [-0.0304, -0.0123] | [-0.0296, -0.0129] | -1.11 | 0.0167 | true | true |
| T2 odour mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0125 | [-0.0170, -0.0081] | [-0.0165, -0.0084] | -1.32 | 0.0095 | true | true |
| T2 odour mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | +0.0247 | [+0.0173, +0.0321] | [+0.0180, +0.0314] | 1.56 | 0.0123 | true | true |
| T2 odour mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | +0.0172 | [+0.0133, +0.0212] | [+0.0138, +0.0210] | 2.04 | 0.0085 | true | true |
| T2 odour mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | +0.0121 | [+0.0064, +0.0179] | [+0.0070, +0.0175] | 0.99 | 0.0123 | true | false |
| T2 odour mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0147 | [+0.0088, +0.0205] | [+0.0093, +0.0199] | 1.18 | 0.0125 | true | true |
| T2 odour mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | +0.0040 | [+0.0001, +0.0079] | [+0.0004, +0.0075] | 0.48 | 0.0084 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0039 | [-0.0101, +0.0022] | [-0.0094, +0.0016] | -0.30 | 0.0082 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | +0.0053 | [+0.0006, +0.0101] | [+0.0010, +0.0096] | 0.53 | 0.0101 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | +0.0001 | [-0.0038, +0.0040] | [-0.0034, +0.0035] | 0.01 | 0.0082 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0073 | [+0.0040, +0.0105] | [+0.0042, +0.0101] | 1.05 | 0.0069 | true | true |
| T2 odour mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0100 | [-0.0202, +0.0002] | [-0.0192, -0.0006] | -0.46 | 0.0218 | false | false |
| T2 odour mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0026 | [-0.0143, +0.0090] | [-0.0131, +0.0080] | -0.11 | 0.0164 | false | false |
| T2 odour mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | -0.0143 | [-0.0212, -0.0073] | [-0.0203, -0.0080] | -0.96 | 0.0148 | true | false |
| T2 odour mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0126 | [-0.0203, -0.0049] | [-0.0197, -0.0058] | -0.77 | 0.0164 | true | false |
| T2 odour mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0140 | [-0.0240, -0.0039] | [-0.0231, -0.0047] | -0.65 | 0.0215 | true | false |
| T2 odour mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0035 | [-0.0140, +0.0071] | [-0.0133, +0.0059] | -0.15 | 0.0226 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0116 | [-0.0265, +0.0033] | [-0.0246, +0.0021] | -0.37 | 0.0164 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | -0.0120 | [-0.0193, -0.0047] | [-0.0184, -0.0053] | -0.77 | 0.0156 | true | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0151 | [-0.0227, -0.0074] | [-0.0217, -0.0078] | -0.92 | 0.0164 | true | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0092 | [-0.0217, +0.0034] | [-0.0207, +0.0021] | -0.34 | 0.0268 | false | false |
| T3 novelty MNIST-PN r (k=10) | P1 wiring beyond degrees (R-b - D-b) | -0.0030 | [-0.0418, +0.0358] | [-0.0379, +0.0325] | -0.04 | 0.0883 | false | false |
| T3 novelty MNIST-PN r (k=10) | P2 degree heterogeneity (D-b - U-b) | +0.0211 | [-0.0240, +0.0662] | [-0.0212, +0.0606] | 0.22 | 0.0635 | false | false |
| T3 novelty MNIST-PN r (k=10) | P3 weights (R-w - S-w) | +0.0050 | [-0.0291, +0.0392] | [-0.0270, +0.0354] | 0.07 | 0.0633 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs uniform (binary) (R-b - U-b) | +0.0181 | [-0.0217, +0.0579] | [-0.0185, +0.0539] | 0.21 | 0.0635 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0321 | [-0.0204, +0.0846] | [-0.0152, +0.0791] | 0.29 | 0.0900 | false | false |
| T3 novelty MNIST-PN r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0229 | [-0.0179, +0.0637] | [-0.0137, +0.0599] | 0.26 | 0.0847 | false | false |
| T3 novelty MNIST-PN r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0286 | [-0.0688, +0.0115] | [-0.0657, +0.0086] | -0.33 | 0.0946 | false | false |
| T3 novelty MNIST-PN r (k=20) | P3 weights (R-w - S-w) | +0.0105 | [-0.0239, +0.0450] | [-0.0208, +0.0424] | 0.14 | 0.0779 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs uniform (binary) (R-b - U-b) | -0.0057 | [-0.0566, +0.0452] | [-0.0508, +0.0419] | -0.05 | 0.0946 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0224 | [-0.0184, +0.0633] | [-0.0150, +0.0584] | 0.26 | 0.0829 | false | false |
| T3 novelty MNIST-PN r (k=40) | P1 wiring beyond degrees (R-b - D-b) | +0.0163 | [-0.0142, +0.0468] | [-0.0123, +0.0432] | 0.25 | 0.0663 | false | false |
| T3 novelty MNIST-PN r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0346 | [-0.0565, -0.0126] | [-0.0541, -0.0144] | -0.74 | 0.0776 | true | false |
| T3 novelty MNIST-PN r (k=40) | P3 weights (R-w - S-w) | -0.0044 | [-0.0368, +0.0280] | [-0.0340, +0.0252] | -0.06 | 0.0755 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs uniform (binary) (R-b - U-b) | -0.0183 | [-0.0467, +0.0101] | [-0.0437, +0.0083] | -0.30 | 0.0776 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0127 | [-0.0289, +0.0544] | [-0.0250, +0.0502] | 0.14 | 0.0758 | false | false |
| T3 novelty MNIST-PN r (k=5) | P1 wiring beyond degrees (R-b - D-b) | +0.0071 | [-0.0341, +0.0483] | [-0.0299, +0.0448] | 0.08 | 0.0653 | false | false |
| T3 novelty MNIST-PN r (k=5) | P2 degree heterogeneity (D-b - U-b) | -0.0205 | [-0.0631, +0.0220] | [-0.0582, +0.0209] | -0.23 | 0.0799 | false | false |
| T3 novelty MNIST-PN r (k=5) | P3 weights (R-w - S-w) | +0.0178 | [-0.0230, +0.0585] | [-0.0193, +0.0559] | 0.20 | 0.0585 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs uniform (binary) (R-b - U-b) | -0.0134 | [-0.0663, +0.0394] | [-0.0612, +0.0355] | -0.12 | 0.0799 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0165 | [-0.0735, +0.0405] | [-0.0674, +0.0362] | -0.14 | 0.0962 | false | false |
| T3 novelty odour r (k=10) | P1 wiring beyond degrees (R-b - D-b) | +0.0118 | [-0.0276, +0.0511] | [-0.0234, +0.0490] | 0.14 | 0.0609 | false | false |
| T3 novelty odour r (k=10) | P2 degree heterogeneity (D-b - U-b) | -0.0494 | [-0.0825, -0.0163] | [-0.0788, -0.0184] | -0.70 | 0.0665 | true | false |
| T3 novelty odour r (k=10) | P3 weights (R-w - S-w) | -0.0334 | [-0.0708, +0.0041] | [-0.0663, +0.0013] | -0.42 | 0.0805 | false | false |
| T3 novelty odour r (k=10) | real vs uniform (binary) (R-b - U-b) | -0.0376 | [-0.0857, +0.0104] | [-0.0805, +0.0069] | -0.37 | 0.0665 | false | false |
| T3 novelty odour r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0282 | [-0.0739, +0.0174] | [-0.0684, +0.0140] | -0.29 | 0.0870 | false | false |
| T3 novelty odour r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0647 | [+0.0311, +0.0984] | [+0.0343, +0.0953] | 0.90 | 0.0722 | true | false |
| T3 novelty odour r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0327 | [-0.0645, -0.0010] | [-0.0627, -0.0055] | -0.48 | 0.0517 | true | false |
| T3 novelty odour r (k=20) | P3 weights (R-w - S-w) | +0.0749 | [+0.0451, +0.1048] | [+0.0476, +0.1026] | 1.17 | 0.0608 | true | true |
| T3 novelty odour r (k=20) | real vs uniform (binary) (R-b - U-b) | +0.0320 | [+0.0062, +0.0578] | [+0.0087, +0.0561] | 0.58 | 0.0517 | true | false |
| T3 novelty odour r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | +0.1057 | [+0.0721, +0.1392] | [+0.0761, +0.1374] | 1.47 | 0.0607 | true | true |
| T3 novelty odour r (k=40) | P1 wiring beyond degrees (R-b - D-b) | -0.0055 | [-0.0371, +0.0262] | [-0.0345, +0.0226] | -0.08 | 0.0720 | false | false |
| T3 novelty odour r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0195 | [-0.0545, +0.0156] | [-0.0523, +0.0112] | -0.26 | 0.0674 | false | false |
| T3 novelty odour r (k=40) | P3 weights (R-w - S-w) | +0.0790 | [+0.0527, +0.1052] | [+0.0547, +0.1026] | 1.41 | 0.0647 | true | true |
| T3 novelty odour r (k=40) | real vs uniform (binary) (R-b - U-b) | -0.0249 | [-0.0528, +0.0030] | [-0.0499, +0.0009] | -0.42 | 0.0674 | false | false |
| T3 novelty odour r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0756 | [+0.0522, +0.0990] | [+0.0541, +0.0967] | 1.51 | 0.0510 | true | true |
| T3 novelty odour r (k=5) | P1 wiring beyond degrees (R-b - D-b) | -0.0122 | [-0.0599, +0.0356] | [-0.0561, +0.0298] | -0.12 | 0.0920 | false | false |
| T3 novelty odour r (k=5) | P2 degree heterogeneity (D-b - U-b) | -0.0656 | [-0.1220, -0.0092] | [-0.1168, -0.0142] | -0.54 | 0.0635 | true | true |
| T3 novelty odour r (k=5) | P3 weights (R-w - S-w) | +0.0089 | [-0.0315, +0.0493] | [-0.0255, +0.0476] | 0.10 | 0.0764 | false | false |
| T3 novelty odour r (k=5) | real vs uniform (binary) (R-b - U-b) | -0.0777 | [-0.1070, -0.0484] | [-0.1038, -0.0502] | -1.24 | 0.0635 | true | true |
| T3 novelty odour r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0032 | [-0.0397, +0.0461] | [-0.0366, +0.0431] | 0.03 | 0.0671 | false | false |

## right

right: 10304 pairs, 142 PNs -> 2356 KCs; KC fan-in median 5 (p10 2, p90 6); PN out-degree mean 72.6, median 60, max 480; synapses 140874

odour input drives 54 of 142 PNs; k_bio = 118

### Arm means (sd over seeds)

| metric | R-b | R-w | U-b | U-w | D-b | D-w | S-w |
|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | 0.4197 (0.0189) | 0.3876 (0.0228) | 0.4761 (0.0185) | 0.4626 (0.0161) | 0.4255 (0.0185) | 0.4017 (0.0205) | 0.3979 (0.0228) |
| T1 mAP bio k=5% (AP/hits) | 0.7477 (0.0149) | 0.7227 (0.0181) | 0.7879 (0.0138) | 0.7787 (0.0121) | 0.7525 (0.0138) | 0.7340 (0.0166) | 0.7309 (0.0182) |
| T1 mAP k=16 valued (AP/R) | 0.1978 (0.0084) | 0.1942 (0.0081) | 0.2127 (0.0093) | 0.2207 (0.0114) | 0.2045 (0.0120) | 0.2019 (0.0118) | 0.2009 (0.0104) |
| T1 mAP k=16 valued (AP/hits) | 0.5330 (0.0123) | 0.5212 (0.0109) | 0.5540 (0.0103) | 0.5575 (0.0125) | 0.5421 (0.0142) | 0.5331 (0.0144) | 0.5307 (0.0150) |
| T2 odour mAP bio k=5% (AP/R) | 0.5539 (0.0000) | 0.5215 (0.0000) | 0.5293 (0.0141) | 0.5098 (0.0200) | 0.5507 (0.0126) | 0.5453 (0.0135) | 0.5457 (0.0114) |
| T2 odour mAP bio k=5% (AP/hits) | 0.8494 (0.0000) | 0.8526 (0.0000) | 0.8553 (0.0108) | 0.8444 (0.0155) | 0.8544 (0.0151) | 0.8474 (0.0090) | 0.8538 (0.0141) |
| T2 odour mAP k=16 valued (AP/R) | 0.4405 (0.0000) | 0.5015 (0.0000) | 0.4396 (0.0226) | 0.4274 (0.0247) | 0.4498 (0.0160) | 0.4663 (0.0223) | 0.4613 (0.0240) |
| T2 odour mAP k=16 valued (AP/hits) | 0.7967 (0.0000) | 0.8191 (0.0000) | 0.8037 (0.0249) | 0.7944 (0.0229) | 0.8037 (0.0251) | 0.8025 (0.0202) | 0.8133 (0.0215) |
| T3 novelty MNIST-PN r (k=10) | 0.1084 (0.0967) | 0.0724 (0.0808) | 0.1200 (0.0755) | 0.1055 (0.0760) | 0.1144 (0.0783) | 0.0976 (0.0711) | 0.1012 (0.0634) |
| T3 novelty MNIST-PN r (k=20) | 0.1308 (0.0704) | 0.0986 (0.0738) | 0.1355 (0.0677) | 0.0993 (0.0638) | 0.1146 (0.0920) | 0.0857 (0.0881) | 0.1108 (0.0820) |
| T3 novelty MNIST-PN r (k=40) | 0.1205 (0.0820) | 0.0907 (0.0838) | 0.1271 (0.0825) | 0.1025 (0.0569) | 0.1159 (0.0894) | 0.0900 (0.0851) | 0.0880 (0.0663) |
| T3 novelty MNIST-PN r (k=5) | 0.0932 (0.0912) | 0.0842 (0.0818) | 0.0903 (0.0907) | 0.0988 (0.0686) | 0.0978 (0.0872) | 0.0966 (0.0651) | 0.0720 (0.0870) |
| T3 novelty odour r (k=10) | 0.3819 (0.0589) | 0.3372 (0.0679) | 0.4170 (0.0849) | 0.3717 (0.0887) | 0.4248 (0.0746) | 0.3952 (0.0597) | 0.3827 (0.0846) |
| T3 novelty odour r (k=20) | 0.3613 (0.0532) | 0.3910 (0.0652) | 0.4424 (0.0662) | 0.3945 (0.0689) | 0.4060 (0.0783) | 0.3771 (0.0710) | 0.3714 (0.0692) |
| T3 novelty odour r (k=40) | 0.4209 (0.0547) | 0.4642 (0.0564) | 0.4530 (0.0673) | 0.3648 (0.0838) | 0.4236 (0.0623) | 0.3760 (0.0730) | 0.3606 (0.0708) |
| T3 novelty odour r (k=5) | 0.3457 (0.0581) | 0.3064 (0.0546) | 0.3704 (0.0609) | 0.3331 (0.0831) | 0.3718 (0.0665) | 0.3303 (0.0920) | 0.3447 (0.0680) |

### Contrasts (paired over seeds)

| metric | contrast | mean diff | 95% t-CI | 95% boot CI | d_z | null sd | detected | > null sd |
|---|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0058 | [-0.0102, -0.0014] | [-0.0098, -0.0019] | -0.62 | 0.0185 | true | false |
| T1 mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0506 | [-0.0571, -0.0441] | [-0.0565, -0.0448] | -3.65 | 0.0185 | true | true |
| T1 mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | -0.0103 | [-0.0141, -0.0065] | [-0.0139, -0.0070] | -1.26 | 0.0228 | true | false |
| T1 mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0564 | [-0.0634, -0.0495] | [-0.0628, -0.0502] | -3.80 | 0.0185 | true | true |
| T1 mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0141 | [-0.0200, -0.0082] | [-0.0196, -0.0088] | -1.11 | 0.0205 | true | false |
| T1 mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0048 | [-0.0082, -0.0014] | [-0.0080, -0.0018] | -0.66 | 0.0138 | true | false |
| T1 mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0354 | [-0.0398, -0.0310] | [-0.0395, -0.0315] | -3.77 | 0.0138 | true | true |
| T1 mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | -0.0082 | [-0.0111, -0.0054] | [-0.0109, -0.0058] | -1.36 | 0.0182 | true | false |
| T1 mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0402 | [-0.0449, -0.0355] | [-0.0445, -0.0360] | -4.00 | 0.0138 | true | true |
| T1 mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0113 | [-0.0160, -0.0066] | [-0.0157, -0.0072] | -1.12 | 0.0166 | true | false |
| T1 mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0067 | [-0.0111, -0.0023] | [-0.0109, -0.0028] | -0.71 | 0.0120 | true | false |
| T1 mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0082 | [-0.0155, -0.0010] | [-0.0145, -0.0015] | -0.53 | 0.0093 | true | false |
| T1 mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | -0.0067 | [-0.0108, -0.0026] | [-0.0105, -0.0030] | -0.76 | 0.0104 | true | false |
| T1 mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0149 | [-0.0209, -0.0089] | [-0.0202, -0.0092] | -1.16 | 0.0093 | true | true |
| T1 mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0077 | [-0.0128, -0.0026] | [-0.0122, -0.0031] | -0.71 | 0.0118 | true | false |
| T1 mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0090 | [-0.0139, -0.0041] | [-0.0137, -0.0046] | -0.87 | 0.0142 | true | false |
| T1 mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0119 | [-0.0196, -0.0043] | [-0.0187, -0.0049] | -0.73 | 0.0103 | true | true |
| T1 mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | -0.0095 | [-0.0148, -0.0043] | [-0.0142, -0.0047] | -0.85 | 0.0150 | true | false |
| T1 mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0210 | [-0.0282, -0.0137] | [-0.0274, -0.0141] | -1.35 | 0.0103 | true | true |
| T1 mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0119 | [-0.0182, -0.0055] | [-0.0175, -0.0060] | -0.87 | 0.0144 | true | false |
| T2 odour mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | +0.0032 | [-0.0026, +0.0091] | [-0.0019, +0.0086] | 0.26 | 0.0126 | false | false |
| T2 odour mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | +0.0213 | [+0.0114, +0.0313] | [+0.0123, +0.0304] | 1.01 | 0.0141 | true | true |
| T2 odour mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | -0.0241 | [-0.0295, -0.0188] | [-0.0289, -0.0189] | -2.12 | 0.0114 | true | true |
| T2 odour mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | +0.0246 | [+0.0180, +0.0312] | [+0.0184, +0.0303] | 1.74 | 0.0141 | true | true |
| T2 odour mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0238 | [-0.0301, -0.0175] | [-0.0297, -0.0182] | -1.76 | 0.0135 | true | true |
| T2 odour mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0050 | [-0.0120, +0.0021] | [-0.0115, +0.0015] | -0.33 | 0.0151 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0009 | [-0.0103, +0.0085] | [-0.0095, +0.0078] | -0.04 | 0.0108 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | -0.0012 | [-0.0078, +0.0055] | [-0.0076, +0.0047] | -0.08 | 0.0141 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0059 | [-0.0109, -0.0008] | [-0.0104, -0.0012] | -0.54 | 0.0108 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0052 | [+0.0010, +0.0094] | [+0.0014, +0.0090] | 0.58 | 0.0090 | true | false |
| T2 odour mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0094 | [-0.0168, -0.0019] | [-0.0162, -0.0026] | -0.59 | 0.0160 | true | false |
| T2 odour mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | +0.0103 | [-0.0050, +0.0256] | [-0.0037, +0.0243] | 0.31 | 0.0226 | false | false |
| T2 odour mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | +0.0402 | [+0.0290, +0.0514] | [+0.0300, +0.0507] | 1.67 | 0.0240 | true | true |
| T2 odour mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | +0.0009 | [-0.0097, +0.0115] | [-0.0091, +0.0102] | 0.04 | 0.0226 | false | false |
| T2 odour mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0353 | [+0.0248, +0.0457] | [+0.0255, +0.0445] | 1.58 | 0.0223 | true | true |
| T2 odour mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0070 | [-0.0187, +0.0048] | [-0.0175, +0.0037] | -0.28 | 0.0251 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | +0.0001 | [-0.0146, +0.0148] | [-0.0128, +0.0136] | 0.00 | 0.0249 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | +0.0058 | [-0.0043, +0.0159] | [-0.0031, +0.0152] | 0.27 | 0.0215 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0069 | [-0.0186, +0.0047] | [-0.0174, +0.0043] | -0.28 | 0.0249 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0167 | [+0.0072, +0.0261] | [+0.0083, +0.0253] | 0.83 | 0.0202 | true | false |
| T3 novelty MNIST-PN r (k=10) | P1 wiring beyond degrees (R-b - D-b) | -0.0060 | [-0.0398, +0.0279] | [-0.0375, +0.0239] | -0.08 | 0.0783 | false | false |
| T3 novelty MNIST-PN r (k=10) | P2 degree heterogeneity (D-b - U-b) | -0.0056 | [-0.0444, +0.0333] | [-0.0406, +0.0309] | -0.07 | 0.0755 | false | false |
| T3 novelty MNIST-PN r (k=10) | P3 weights (R-w - S-w) | -0.0288 | [-0.0650, +0.0074] | [-0.0617, +0.0034] | -0.37 | 0.0634 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs uniform (binary) (R-b - U-b) | -0.0115 | [-0.0512, +0.0281] | [-0.0488, +0.0236] | -0.14 | 0.0755 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0252 | [-0.0549, +0.0046] | [-0.0520, +0.0024] | -0.40 | 0.0711 | false | false |
| T3 novelty MNIST-PN r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0162 | [-0.0272, +0.0596] | [-0.0205, +0.0564] | 0.17 | 0.0920 | false | false |
| T3 novelty MNIST-PN r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0209 | [-0.0603, +0.0185] | [-0.0561, +0.0157] | -0.25 | 0.0677 | false | false |
| T3 novelty MNIST-PN r (k=20) | P3 weights (R-w - S-w) | -0.0121 | [-0.0539, +0.0296] | [-0.0505, +0.0256] | -0.14 | 0.0820 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs uniform (binary) (R-b - U-b) | -0.0047 | [-0.0381, +0.0287] | [-0.0352, +0.0248] | -0.07 | 0.0677 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0129 | [-0.0354, +0.0612] | [-0.0302, +0.0575] | 0.13 | 0.0881 | false | false |
| T3 novelty MNIST-PN r (k=40) | P1 wiring beyond degrees (R-b - D-b) | +0.0046 | [-0.0326, +0.0417] | [-0.0306, +0.0365] | 0.06 | 0.0894 | false | false |
| T3 novelty MNIST-PN r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0112 | [-0.0480, +0.0256] | [-0.0444, +0.0214] | -0.14 | 0.0825 | false | false |
| T3 novelty MNIST-PN r (k=40) | P3 weights (R-w - S-w) | +0.0028 | [-0.0259, +0.0314] | [-0.0235, +0.0292] | 0.05 | 0.0663 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs uniform (binary) (R-b - U-b) | -0.0067 | [-0.0417, +0.0284] | [-0.0389, +0.0241] | -0.09 | 0.0825 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0007 | [-0.0312, +0.0326] | [-0.0280, +0.0299] | 0.01 | 0.0851 | false | false |
| T3 novelty MNIST-PN r (k=5) | P1 wiring beyond degrees (R-b - D-b) | -0.0046 | [-0.0583, +0.0491] | [-0.0569, +0.0418] | -0.04 | 0.0872 | false | false |
| T3 novelty MNIST-PN r (k=5) | P2 degree heterogeneity (D-b - U-b) | +0.0076 | [-0.0337, +0.0488] | [-0.0291, +0.0458] | 0.09 | 0.0907 | false | false |
| T3 novelty MNIST-PN r (k=5) | P3 weights (R-w - S-w) | +0.0122 | [-0.0273, +0.0517] | [-0.0229, +0.0478] | 0.14 | 0.0870 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs uniform (binary) (R-b - U-b) | +0.0030 | [-0.0477, +0.0537] | [-0.0432, +0.0500] | 0.03 | 0.0907 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0124 | [-0.0539, +0.0290] | [-0.0491, +0.0255] | -0.14 | 0.0651 | false | false |
| T3 novelty odour r (k=10) | P1 wiring beyond degrees (R-b - D-b) | -0.0429 | [-0.0762, -0.0095] | [-0.0749, -0.0135] | -0.60 | 0.0746 | true | false |
| T3 novelty odour r (k=10) | P2 degree heterogeneity (D-b - U-b) | +0.0077 | [-0.0384, +0.0538] | [-0.0340, +0.0498] | 0.08 | 0.0849 | false | false |
| T3 novelty odour r (k=10) | P3 weights (R-w - S-w) | -0.0455 | [-0.0957, +0.0048] | [-0.0925, +0.0000] | -0.42 | 0.0846 | false | false |
| T3 novelty odour r (k=10) | real vs uniform (binary) (R-b - U-b) | -0.0351 | [-0.0737, +0.0034] | [-0.0704, +0.0001] | -0.43 | 0.0849 | false | false |
| T3 novelty odour r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0580 | [-0.0871, -0.0288] | [-0.0841, -0.0311] | -0.93 | 0.0597 | true | false |
| T3 novelty odour r (k=20) | P1 wiring beyond degrees (R-b - D-b) | -0.0447 | [-0.0701, -0.0192] | [-0.0683, -0.0215] | -0.82 | 0.0783 | true | false |
| T3 novelty odour r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0364 | [-0.0719, -0.0010] | [-0.0689, -0.0044] | -0.48 | 0.0662 | true | false |
| T3 novelty odour r (k=20) | P3 weights (R-w - S-w) | +0.0196 | [-0.0185, +0.0577] | [-0.0143, +0.0552] | 0.24 | 0.0692 | false | false |
| T3 novelty odour r (k=20) | real vs uniform (binary) (R-b - U-b) | -0.0811 | [-0.1071, -0.0552] | [-0.1050, -0.0570] | -1.46 | 0.0662 | true | true |
| T3 novelty odour r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0140 | [-0.0294, +0.0573] | [-0.0249, +0.0535] | 0.15 | 0.0710 | false | false |
| T3 novelty odour r (k=40) | P1 wiring beyond degrees (R-b - D-b) | -0.0028 | [-0.0233, +0.0178] | [-0.0208, +0.0162] | -0.06 | 0.0623 | false | false |
| T3 novelty odour r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0293 | [-0.0645, +0.0058] | [-0.0615, +0.0033] | -0.39 | 0.0673 | false | false |
| T3 novelty odour r (k=40) | P3 weights (R-w - S-w) | +0.1035 | [+0.0731, +0.1339] | [+0.0748, +0.1307] | 1.59 | 0.0708 | true | true |
| T3 novelty odour r (k=40) | real vs uniform (binary) (R-b - U-b) | -0.0321 | [-0.0581, -0.0060] | [-0.0551, -0.0082] | -0.58 | 0.0673 | true | false |
| T3 novelty odour r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0882 | [+0.0647, +0.1116] | [+0.0670, +0.1093] | 1.76 | 0.0730 | true | true |
| T3 novelty odour r (k=5) | P1 wiring beyond degrees (R-b - D-b) | -0.0262 | [-0.0650, +0.0126] | [-0.0629, +0.0090] | -0.32 | 0.0665 | false | false |
| T3 novelty odour r (k=5) | P2 degree heterogeneity (D-b - U-b) | +0.0014 | [-0.0399, +0.0427] | [-0.0364, +0.0385] | 0.02 | 0.0609 | false | false |
| T3 novelty odour r (k=5) | P3 weights (R-w - S-w) | -0.0383 | [-0.0744, -0.0022] | [-0.0710, -0.0036] | -0.50 | 0.0680 | true | false |
| T3 novelty odour r (k=5) | real vs uniform (binary) (R-b - U-b) | -0.0248 | [-0.0636, +0.0141] | [-0.0623, +0.0102] | -0.30 | 0.0609 | false | false |
| T3 novelty odour r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0239 | [-0.0773, +0.0295] | [-0.0732, +0.0237] | -0.21 | 0.0920 | false | false |

