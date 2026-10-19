# Wiring study results: male CNS v1.0 replication

## mcns_pooled

mcns_pooled: 19874 pairs, 270 PNs -> 3766 KCs; KC fan-in median 5 (p10 3, p90 7); PN out-degree mean 73.6, median 57, max 461; synapses 381124

odour input drives 105 of 270 PNs; k_bio = 188

### Arm means (sd over seeds)

| metric | R-b | R-w | U-b | U-w | D-b | D-w | S-w |
|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | 0.4745 (0.0168) | 0.4292 (0.0220) | 0.5318 (0.0141) | 0.5196 (0.0149) | 0.4948 (0.0136) | 0.4777 (0.0131) | 0.4642 (0.0170) |
| T1 mAP bio k=5% (AP/hits) | 0.7901 (0.0117) | 0.7589 (0.0156) | 0.8238 (0.0098) | 0.8170 (0.0107) | 0.8019 (0.0095) | 0.7915 (0.0087) | 0.7831 (0.0116) |
| T1 mAP k=16 valued (AP/R) | 0.1927 (0.0103) | 0.1925 (0.0115) | 0.2066 (0.0116) | 0.2172 (0.0135) | 0.1984 (0.0117) | 0.2072 (0.0124) | 0.2028 (0.0131) |
| T1 mAP k=16 valued (AP/hits) | 0.5277 (0.0128) | 0.5208 (0.0147) | 0.5457 (0.0123) | 0.5572 (0.0143) | 0.5371 (0.0160) | 0.5434 (0.0146) | 0.5365 (0.0145) |
| T2 odour mAP bio k=5% (AP/R) | 0.5508 (0.0000) | 0.5317 (0.0000) | 0.5431 (0.0167) | 0.5117 (0.0154) | 0.5450 (0.0125) | 0.5223 (0.0092) | 0.5276 (0.0089) |
| T2 odour mAP bio k=5% (AP/hits) | 0.8506 (0.0000) | 0.8551 (0.0000) | 0.8526 (0.0118) | 0.8375 (0.0131) | 0.8449 (0.0109) | 0.8353 (0.0095) | 0.8370 (0.0090) |
| T2 odour mAP k=16 valued (AP/R) | 0.4075 (0.0000) | 0.4394 (0.0000) | 0.4292 (0.0233) | 0.4267 (0.0232) | 0.4332 (0.0263) | 0.4456 (0.0245) | 0.4481 (0.0242) |
| T2 odour mAP k=16 valued (AP/hits) | 0.7983 (0.0000) | 0.7909 (0.0000) | 0.7962 (0.0213) | 0.7866 (0.0195) | 0.7959 (0.0250) | 0.8018 (0.0206) | 0.8141 (0.0191) |
| T3 novelty MNIST-PN r (k=10) | 0.1112 (0.0725) | 0.1162 (0.0808) | 0.1317 (0.0922) | 0.1099 (0.0809) | 0.1137 (0.0911) | 0.1189 (0.1129) | 0.1291 (0.0705) |
| T3 novelty MNIST-PN r (k=20) | 0.1295 (0.0650) | 0.1159 (0.0917) | 0.1375 (0.0796) | 0.1260 (0.0765) | 0.1130 (0.0974) | 0.1238 (0.0982) | 0.1275 (0.0802) |
| T3 novelty MNIST-PN r (k=40) | 0.1507 (0.0796) | 0.1242 (0.0789) | 0.1351 (0.0821) | 0.1172 (0.0808) | 0.1297 (0.0878) | 0.1454 (0.0848) | 0.1264 (0.0520) |
| T3 novelty MNIST-PN r (k=5) | 0.1052 (0.0804) | 0.0965 (0.0659) | 0.1177 (0.1005) | 0.1203 (0.0897) | 0.1066 (0.0642) | 0.1019 (0.0944) | 0.1108 (0.0776) |
| T3 novelty odour r (k=10) | 0.4431 (0.0576) | 0.3258 (0.0537) | 0.4160 (0.0573) | 0.4067 (0.0479) | 0.3761 (0.0787) | 0.4019 (0.0842) | 0.3677 (0.0406) |
| T3 novelty odour r (k=20) | 0.4592 (0.0589) | 0.4205 (0.0497) | 0.4451 (0.0707) | 0.4032 (0.0730) | 0.4081 (0.0845) | 0.4080 (0.0825) | 0.3918 (0.0734) |
| T3 novelty odour r (k=40) | 0.4198 (0.0625) | 0.4094 (0.0370) | 0.4694 (0.0578) | 0.4112 (0.0765) | 0.4162 (0.0691) | 0.3866 (0.0751) | 0.4000 (0.0529) |
| T3 novelty odour r (k=5) | 0.3959 (0.0284) | 0.4251 (0.0391) | 0.3659 (0.0625) | 0.3796 (0.0954) | 0.3688 (0.1096) | 0.3282 (0.0773) | 0.3563 (0.0749) |

### Contrasts (paired over seeds)

| metric | contrast | mean diff | 95% t-CI | 95% boot CI | d_z | null sd | detected | > null sd |
|---|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0203 | [-0.0264, -0.0142] | [-0.0260, -0.0147] | -1.55 | 0.0136 | true | true |
| T1 mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0370 | [-0.0426, -0.0313] | [-0.0423, -0.0320] | -3.06 | 0.0141 | true | true |
| T1 mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | -0.0351 | [-0.0405, -0.0296] | [-0.0401, -0.0301] | -3.01 | 0.0170 | true | true |
| T1 mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0573 | [-0.0633, -0.0512] | [-0.0628, -0.0518] | -4.42 | 0.0141 | true | true |
| T1 mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0485 | [-0.0547, -0.0423] | [-0.0545, -0.0433] | -3.68 | 0.0131 | true | true |
| T1 mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0118 | [-0.0160, -0.0077] | [-0.0157, -0.0081] | -1.33 | 0.0095 | true | true |
| T1 mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0219 | [-0.0255, -0.0183] | [-0.0253, -0.0187] | -2.83 | 0.0098 | true | true |
| T1 mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | -0.0242 | [-0.0283, -0.0201] | [-0.0281, -0.0205] | -2.75 | 0.0116 | true | true |
| T1 mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0337 | [-0.0373, -0.0301] | [-0.0370, -0.0304] | -4.33 | 0.0098 | true | true |
| T1 mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0326 | [-0.0374, -0.0278] | [-0.0372, -0.0286] | -3.20 | 0.0087 | true | true |
| T1 mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0057 | [-0.0128, +0.0014] | [-0.0122, +0.0008] | -0.38 | 0.0117 | false | false |
| T1 mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0082 | [-0.0155, -0.0009] | [-0.0148, -0.0014] | -0.53 | 0.0116 | true | false |
| T1 mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | -0.0103 | [-0.0166, -0.0039] | [-0.0159, -0.0043] | -0.76 | 0.0131 | true | false |
| T1 mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0139 | [-0.0195, -0.0083] | [-0.0191, -0.0088] | -1.15 | 0.0116 | true | true |
| T1 mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0147 | [-0.0208, -0.0086] | [-0.0202, -0.0092] | -1.13 | 0.0124 | true | true |
| T1 mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0094 | [-0.0180, -0.0007] | [-0.0175, -0.0016] | -0.51 | 0.0160 | true | false |
| T1 mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0086 | [-0.0188, +0.0015] | [-0.0178, +0.0007] | -0.40 | 0.0123 | false | false |
| T1 mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | -0.0157 | [-0.0230, -0.0084] | [-0.0222, -0.0089] | -1.01 | 0.0145 | true | true |
| T1 mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0180 | [-0.0247, -0.0113] | [-0.0243, -0.0119] | -1.26 | 0.0123 | true | true |
| T1 mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0226 | [-0.0299, -0.0153] | [-0.0294, -0.0162] | -1.45 | 0.0146 | true | true |
| T2 odour mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | +0.0058 | [-0.0000, +0.0117] | [+0.0005, +0.0112] | 0.46 | 0.0125 | false | false |
| T2 odour mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | +0.0018 | [-0.0076, +0.0113] | [-0.0062, +0.0108] | 0.09 | 0.0167 | false | false |
| T2 odour mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | +0.0041 | [-0.0001, +0.0083] | [+0.0003, +0.0080] | 0.46 | 0.0089 | false | false |
| T2 odour mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | +0.0077 | [-0.0001, +0.0154] | [+0.0007, +0.0147] | 0.46 | 0.0167 | false | false |
| T2 odour mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0093 | [+0.0050, +0.0137] | [+0.0055, +0.0133] | 1.01 | 0.0092 | true | true |
| T2 odour mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | +0.0057 | [+0.0006, +0.0108] | [+0.0009, +0.0104] | 0.52 | 0.0109 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0078 | [-0.0148, -0.0008] | [-0.0140, -0.0012] | -0.52 | 0.0118 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | +0.0181 | [+0.0139, +0.0223] | [+0.0144, +0.0220] | 2.02 | 0.0090 | true | true |
| T2 odour mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0021 | [-0.0076, +0.0034] | [-0.0070, +0.0029] | -0.18 | 0.0118 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0198 | [+0.0153, +0.0243] | [+0.0159, +0.0240] | 2.08 | 0.0095 | true | true |
| T2 odour mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0257 | [-0.0380, -0.0134] | [-0.0369, -0.0144] | -0.98 | 0.0263 | true | false |
| T2 odour mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | +0.0041 | [-0.0091, +0.0173] | [-0.0081, +0.0160] | 0.14 | 0.0233 | false | false |
| T2 odour mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | -0.0087 | [-0.0200, +0.0027] | [-0.0194, +0.0015] | -0.36 | 0.0242 | false | false |
| T2 odour mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0216 | [-0.0325, -0.0107] | [-0.0311, -0.0115] | -0.93 | 0.0233 | true | false |
| T2 odour mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0061 | [-0.0176, +0.0053] | [-0.0170, +0.0042] | -0.25 | 0.0245 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | +0.0024 | [-0.0093, +0.0141] | [-0.0080, +0.0131] | 0.10 | 0.0250 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0004 | [-0.0181, +0.0174] | [-0.0172, +0.0151] | -0.01 | 0.0213 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | -0.0232 | [-0.0322, -0.0143] | [-0.0318, -0.0152] | -1.22 | 0.0191 | true | true |
| T2 odour mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | +0.0020 | [-0.0079, +0.0120] | [-0.0069, +0.0109] | 0.10 | 0.0213 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0109 | [-0.0205, -0.0012] | [-0.0199, -0.0021] | -0.53 | 0.0206 | true | false |
| T3 novelty MNIST-PN r (k=10) | P1 wiring beyond degrees (R-b - D-b) | -0.0025 | [-0.0513, +0.0464] | [-0.0472, +0.0422] | -0.02 | 0.0911 | false | false |
| T3 novelty MNIST-PN r (k=10) | P2 degree heterogeneity (D-b - U-b) | -0.0181 | [-0.0711, +0.0349] | [-0.0680, +0.0270] | -0.16 | 0.0922 | false | false |
| T3 novelty MNIST-PN r (k=10) | P3 weights (R-w - S-w) | -0.0129 | [-0.0571, +0.0312] | [-0.0541, +0.0269] | -0.14 | 0.0705 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs uniform (binary) (R-b - U-b) | -0.0205 | [-0.0666, +0.0256] | [-0.0639, +0.0200] | -0.21 | 0.0922 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0027 | [-0.0528, +0.0474] | [-0.0475, +0.0438] | -0.03 | 0.1129 | false | false |
| T3 novelty MNIST-PN r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0165 | [-0.0241, +0.0571] | [-0.0201, +0.0536] | 0.19 | 0.0974 | false | false |
| T3 novelty MNIST-PN r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0245 | [-0.0726, +0.0236] | [-0.0684, +0.0191] | -0.24 | 0.0796 | false | false |
| T3 novelty MNIST-PN r (k=20) | P3 weights (R-w - S-w) | -0.0116 | [-0.0439, +0.0208] | [-0.0411, +0.0181] | -0.17 | 0.0802 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs uniform (binary) (R-b - U-b) | -0.0080 | [-0.0367, +0.0207] | [-0.0352, +0.0173] | -0.13 | 0.0796 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0078 | [-0.0467, +0.0310] | [-0.0432, +0.0279] | -0.09 | 0.0982 | false | false |
| T3 novelty MNIST-PN r (k=40) | P1 wiring beyond degrees (R-b - D-b) | +0.0210 | [-0.0147, +0.0567] | [-0.0100, +0.0538] | 0.28 | 0.0878 | false | false |
| T3 novelty MNIST-PN r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0055 | [-0.0442, +0.0333] | [-0.0402, +0.0295] | -0.07 | 0.0821 | false | false |
| T3 novelty MNIST-PN r (k=40) | P3 weights (R-w - S-w) | -0.0023 | [-0.0377, +0.0332] | [-0.0357, +0.0290] | -0.03 | 0.0520 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs uniform (binary) (R-b - U-b) | +0.0155 | [-0.0187, +0.0498] | [-0.0177, +0.0452] | 0.21 | 0.0821 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0213 | [-0.0617, +0.0191] | [-0.0584, +0.0153] | -0.25 | 0.0848 | false | false |
| T3 novelty MNIST-PN r (k=5) | P1 wiring beyond degrees (R-b - D-b) | -0.0014 | [-0.0394, +0.0366] | [-0.0350, +0.0340] | -0.02 | 0.0642 | false | false |
| T3 novelty MNIST-PN r (k=5) | P2 degree heterogeneity (D-b - U-b) | -0.0110 | [-0.0564, +0.0343] | [-0.0523, +0.0298] | -0.11 | 0.1005 | false | false |
| T3 novelty MNIST-PN r (k=5) | P3 weights (R-w - S-w) | -0.0143 | [-0.0501, +0.0215] | [-0.0457, +0.0191] | -0.19 | 0.0776 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs uniform (binary) (R-b - U-b) | -0.0124 | [-0.0474, +0.0225] | [-0.0438, +0.0195] | -0.17 | 0.1005 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0054 | [-0.0445, +0.0337] | [-0.0416, +0.0295] | -0.06 | 0.0944 | false | false |
| T3 novelty odour r (k=10) | P1 wiring beyond degrees (R-b - D-b) | +0.0671 | [+0.0276, +0.1066] | [+0.0327, +0.1045] | 0.79 | 0.0787 | true | false |
| T3 novelty odour r (k=10) | P2 degree heterogeneity (D-b - U-b) | -0.0399 | [-0.0826, +0.0028] | [-0.0775, -0.0002] | -0.44 | 0.0573 | false | false |
| T3 novelty odour r (k=10) | P3 weights (R-w - S-w) | -0.0419 | [-0.0676, -0.0161] | [-0.0649, -0.0181] | -0.76 | 0.0406 | true | true |
| T3 novelty odour r (k=10) | real vs uniform (binary) (R-b - U-b) | +0.0272 | [-0.0008, +0.0551] | [+0.0010, +0.0522] | 0.46 | 0.0573 | false | false |
| T3 novelty odour r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0761 | [-0.1220, -0.0303] | [-0.1179, -0.0341] | -0.78 | 0.0842 | true | false |
| T3 novelty odour r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0511 | [+0.0070, +0.0952] | [+0.0136, +0.0936] | 0.54 | 0.0845 | true | false |
| T3 novelty odour r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0370 | [-0.0833, +0.0093] | [-0.0786, +0.0047] | -0.37 | 0.0707 | false | false |
| T3 novelty odour r (k=20) | P3 weights (R-w - S-w) | +0.0287 | [+0.0026, +0.0547] | [+0.0057, +0.0535] | 0.52 | 0.0734 | true | false |
| T3 novelty odour r (k=20) | real vs uniform (binary) (R-b - U-b) | +0.0141 | [-0.0163, +0.0446] | [-0.0149, +0.0410] | 0.22 | 0.0707 | false | false |
| T3 novelty odour r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0125 | [-0.0261, +0.0512] | [-0.0216, +0.0489] | 0.15 | 0.0825 | false | false |
| T3 novelty odour r (k=40) | P1 wiring beyond degrees (R-b - D-b) | +0.0037 | [-0.0181, +0.0254] | [-0.0159, +0.0238] | 0.08 | 0.0691 | false | false |
| T3 novelty odour r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0532 | [-0.0865, -0.0199] | [-0.0845, -0.0235] | -0.75 | 0.0578 | true | false |
| T3 novelty odour r (k=40) | P3 weights (R-w - S-w) | +0.0094 | [-0.0148, +0.0336] | [-0.0125, +0.0311] | 0.18 | 0.0529 | false | false |
| T3 novelty odour r (k=40) | real vs uniform (binary) (R-b - U-b) | -0.0495 | [-0.0776, -0.0214] | [-0.0759, -0.0247] | -0.83 | 0.0578 | true | false |
| T3 novelty odour r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0228 | [-0.0038, +0.0494] | [-0.0016, +0.0470] | 0.40 | 0.0751 | false | false |
| T3 novelty odour r (k=5) | P1 wiring beyond degrees (R-b - D-b) | +0.0271 | [-0.0218, +0.0760] | [-0.0155, +0.0740] | 0.26 | 0.1096 | false | false |
| T3 novelty odour r (k=5) | P2 degree heterogeneity (D-b - U-b) | +0.0029 | [-0.0520, +0.0578] | [-0.0485, +0.0526] | 0.02 | 0.0625 | false | false |
| T3 novelty odour r (k=5) | P3 weights (R-w - S-w) | +0.0688 | [+0.0236, +0.1140] | [+0.0266, +0.1089] | 0.71 | 0.0749 | true | false |
| T3 novelty odour r (k=5) | real vs uniform (binary) (R-b - U-b) | +0.0300 | [-0.0053, +0.0652] | [-0.0033, +0.0599] | 0.40 | 0.0625 | false | false |
| T3 novelty odour r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0969 | [+0.0554, +0.1385] | [+0.0583, +0.1341] | 1.09 | 0.0773 | true | true |

## mcns_right

mcns_right: 9742 pairs, 136 PNs -> 1891 KCs; KC fan-in median 5 (p10 3, p90 7); PN out-degree mean 71.6, median 54, max 434; synapses 190918

odour input drives 53 of 136 PNs; k_bio = 95

### Arm means (sd over seeds)

| metric | R-b | R-w | U-b | U-w | D-b | D-w | S-w |
|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | 0.4161 (0.0170) | 0.3665 (0.0239) | 0.4621 (0.0161) | 0.4442 (0.0175) | 0.4172 (0.0148) | 0.3950 (0.0172) | 0.3919 (0.0188) |
| T1 mAP bio k=5% (AP/hits) | 0.7442 (0.0133) | 0.7044 (0.0198) | 0.7777 (0.0128) | 0.7648 (0.0132) | 0.7455 (0.0113) | 0.7282 (0.0139) | 0.7248 (0.0155) |
| T1 mAP k=16 valued (AP/R) | 0.2072 (0.0125) | 0.2015 (0.0136) | 0.2238 (0.0103) | 0.2325 (0.0131) | 0.2132 (0.0132) | 0.2074 (0.0077) | 0.2099 (0.0137) |
| T1 mAP k=16 valued (AP/hits) | 0.5472 (0.0141) | 0.5314 (0.0180) | 0.5671 (0.0125) | 0.5718 (0.0145) | 0.5535 (0.0144) | 0.5403 (0.0087) | 0.5436 (0.0166) |
| T2 odour mAP bio k=5% (AP/R) | 0.5323 (0.0000) | 0.5325 (0.0000) | 0.5221 (0.0164) | 0.4887 (0.0254) | 0.5272 (0.0121) | 0.5160 (0.0139) | 0.5158 (0.0167) |
| T2 odour mAP bio k=5% (AP/hits) | 0.8455 (0.0000) | 0.8678 (0.0000) | 0.8387 (0.0145) | 0.8211 (0.0142) | 0.8377 (0.0111) | 0.8327 (0.0122) | 0.8337 (0.0118) |
| T2 odour mAP k=16 valued (AP/R) | 0.4400 (0.0000) | 0.4643 (0.0000) | 0.4462 (0.0204) | 0.4316 (0.0372) | 0.4408 (0.0230) | 0.4606 (0.0266) | 0.4506 (0.0230) |
| T2 odour mAP k=16 valued (AP/hits) | 0.8157 (0.0000) | 0.8178 (0.0000) | 0.8071 (0.0272) | 0.7979 (0.0191) | 0.8050 (0.0157) | 0.8139 (0.0207) | 0.8099 (0.0162) |
| T3 novelty MNIST-PN r (k=10) | 0.1077 (0.0720) | 0.0788 (0.0814) | 0.1203 (0.0818) | 0.1130 (0.0631) | 0.0898 (0.0795) | 0.0835 (0.0834) | 0.1152 (0.0813) |
| T3 novelty MNIST-PN r (k=20) | 0.1067 (0.0642) | 0.0799 (0.0597) | 0.1102 (0.0926) | 0.0957 (0.0663) | 0.0602 (0.0705) | 0.0785 (0.1003) | 0.0962 (0.0831) |
| T3 novelty MNIST-PN r (k=40) | 0.0944 (0.0872) | 0.0864 (0.0698) | 0.0981 (0.0682) | 0.0785 (0.0866) | 0.0819 (0.0869) | 0.0741 (0.0750) | 0.0875 (0.0817) |
| T3 novelty MNIST-PN r (k=5) | 0.0914 (0.0796) | 0.0990 (0.0713) | 0.0834 (0.0658) | 0.0626 (0.0727) | 0.0935 (0.0784) | 0.0499 (0.0806) | 0.0798 (0.0794) |
| T3 novelty odour r (k=10) | 0.4860 (0.0605) | 0.3410 (0.0587) | 0.4115 (0.0708) | 0.3828 (0.0659) | 0.3680 (0.0660) | 0.3368 (0.0796) | 0.3727 (0.0863) |
| T3 novelty odour r (k=20) | 0.4475 (0.0583) | 0.3486 (0.0429) | 0.4437 (0.0598) | 0.3988 (0.0576) | 0.4100 (0.0743) | 0.3572 (0.0635) | 0.4021 (0.0659) |
| T3 novelty odour r (k=40) | 0.4702 (0.0554) | 0.4239 (0.0391) | 0.4301 (0.0601) | 0.3709 (0.0680) | 0.4189 (0.0775) | 0.3611 (0.0733) | 0.3650 (0.0627) |
| T3 novelty odour r (k=5) | 0.3005 (0.0484) | 0.4164 (0.0411) | 0.3885 (0.0543) | 0.3452 (0.0762) | 0.3299 (0.0782) | 0.2960 (0.0774) | 0.3519 (0.0854) |

### Contrasts (paired over seeds)

| metric | contrast | mean diff | 95% t-CI | 95% boot CI | d_z | null sd | detected | > null sd |
|---|---|---|---|---|---|---|---|---|
| T1 mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0011 | [-0.0057, +0.0035] | [-0.0054, +0.0030] | -0.11 | 0.0148 | false | false |
| T1 mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0449 | [-0.0521, -0.0378] | [-0.0519, -0.0388] | -2.93 | 0.0161 | true | true |
| T1 mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | -0.0254 | [-0.0309, -0.0198] | [-0.0305, -0.0203] | -2.14 | 0.0188 | true | true |
| T1 mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0461 | [-0.0524, -0.0397] | [-0.0522, -0.0405] | -3.38 | 0.0161 | true | true |
| T1 mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0284 | [-0.0354, -0.0215] | [-0.0347, -0.0222] | -1.92 | 0.0172 | true | true |
| T1 mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0013 | [-0.0047, +0.0022] | [-0.0045, +0.0018] | -0.17 | 0.0113 | false | false |
| T1 mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0323 | [-0.0375, -0.0270] | [-0.0373, -0.0278] | -2.89 | 0.0128 | true | true |
| T1 mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | -0.0204 | [-0.0252, -0.0156] | [-0.0247, -0.0160] | -1.99 | 0.0155 | true | true |
| T1 mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0335 | [-0.0381, -0.0290] | [-0.0380, -0.0296] | -3.43 | 0.0128 | true | true |
| T1 mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0238 | [-0.0295, -0.0180] | [-0.0290, -0.0187] | -1.94 | 0.0139 | true | true |
| T1 mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0060 | [-0.0110, -0.0011] | [-0.0105, -0.0017] | -0.57 | 0.0132 | true | false |
| T1 mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0107 | [-0.0167, -0.0047] | [-0.0162, -0.0054] | -0.84 | 0.0103 | true | true |
| T1 mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | -0.0084 | [-0.0144, -0.0023] | [-0.0140, -0.0030] | -0.65 | 0.0137 | true | false |
| T1 mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0167 | [-0.0225, -0.0109] | [-0.0217, -0.0114] | -1.35 | 0.0103 | true | true |
| T1 mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0058 | [-0.0118, +0.0001] | [-0.0111, -0.0004] | -0.46 | 0.0077 | false | false |
| T1 mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | -0.0063 | [-0.0128, +0.0001] | [-0.0124, -0.0007] | -0.46 | 0.0144 | false | false |
| T1 mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0136 | [-0.0204, -0.0068] | [-0.0197, -0.0072] | -0.93 | 0.0125 | true | true |
| T1 mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | -0.0122 | [-0.0200, -0.0045] | [-0.0194, -0.0054] | -0.74 | 0.0166 | true | false |
| T1 mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | -0.0199 | [-0.0259, -0.0139] | [-0.0251, -0.0141] | -1.54 | 0.0125 | true | true |
| T1 mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0089 | [-0.0165, -0.0012] | [-0.0156, -0.0018] | -0.54 | 0.0087 | true | true |
| T2 odour mAP bio k=5% (AP/R) | P1 wiring beyond degrees (R-b - D-b) | +0.0051 | [-0.0006, +0.0108] | [+0.0000, +0.0103] | 0.42 | 0.0121 | false | false |
| T2 odour mAP bio k=5% (AP/R) | P2 degree heterogeneity (D-b - U-b) | +0.0051 | [-0.0067, +0.0170] | [-0.0060, +0.0155] | 0.20 | 0.0164 | false | false |
| T2 odour mAP bio k=5% (AP/R) | P3 weights (R-w - S-w) | +0.0166 | [+0.0088, +0.0245] | [+0.0095, +0.0239] | 1.00 | 0.0167 | true | false |
| T2 odour mAP bio k=5% (AP/R) | real vs uniform (binary) (R-b - U-b) | +0.0102 | [+0.0025, +0.0179] | [+0.0028, +0.0169] | 0.62 | 0.0164 | true | false |
| T2 odour mAP bio k=5% (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0165 | [+0.0100, +0.0230] | [+0.0106, +0.0224] | 1.19 | 0.0139 | true | true |
| T2 odour mAP bio k=5% (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | +0.0078 | [+0.0027, +0.0130] | [+0.0031, +0.0126] | 0.71 | 0.0111 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0010 | [-0.0103, +0.0083] | [-0.0093, +0.0075] | -0.05 | 0.0145 | false | false |
| T2 odour mAP bio k=5% (AP/hits) | P3 weights (R-w - S-w) | +0.0342 | [+0.0287, +0.0397] | [+0.0290, +0.0391] | 2.90 | 0.0118 | true | true |
| T2 odour mAP bio k=5% (AP/hits) | real vs uniform (binary) (R-b - U-b) | +0.0068 | [+0.0001, +0.0136] | [+0.0007, +0.0129] | 0.47 | 0.0145 | true | false |
| T2 odour mAP bio k=5% (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0351 | [+0.0294, +0.0408] | [+0.0298, +0.0403] | 2.88 | 0.0122 | true | true |
| T2 odour mAP k=16 valued (AP/R) | P1 wiring beyond degrees (R-b - D-b) | -0.0008 | [-0.0116, +0.0100] | [-0.0106, +0.0089] | -0.04 | 0.0230 | false | false |
| T2 odour mAP k=16 valued (AP/R) | P2 degree heterogeneity (D-b - U-b) | -0.0055 | [-0.0197, +0.0088] | [-0.0183, +0.0076] | -0.18 | 0.0204 | false | false |
| T2 odour mAP k=16 valued (AP/R) | P3 weights (R-w - S-w) | +0.0137 | [+0.0029, +0.0244] | [+0.0037, +0.0233] | 0.60 | 0.0230 | true | false |
| T2 odour mAP k=16 valued (AP/R) | real vs uniform (binary) (R-b - U-b) | -0.0063 | [-0.0158, +0.0033] | [-0.0147, +0.0027] | -0.31 | 0.0204 | false | false |
| T2 odour mAP k=16 valued (AP/R) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0037 | [-0.0087, +0.0162] | [-0.0073, +0.0153] | 0.14 | 0.0266 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P1 wiring beyond degrees (R-b - D-b) | +0.0107 | [+0.0033, +0.0180] | [+0.0041, +0.0175] | 0.68 | 0.0157 | true | false |
| T2 odour mAP k=16 valued (AP/hits) | P2 degree heterogeneity (D-b - U-b) | -0.0020 | [-0.0179, +0.0138] | [-0.0168, +0.0120] | -0.06 | 0.0272 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | P3 weights (R-w - S-w) | +0.0079 | [+0.0003, +0.0154] | [+0.0009, +0.0147] | 0.49 | 0.0162 | true | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs uniform (binary) (R-b - U-b) | +0.0086 | [-0.0041, +0.0214] | [-0.0028, +0.0204] | 0.32 | 0.0272 | false | false |
| T2 odour mAP k=16 valued (AP/hits) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0039 | [-0.0058, +0.0136] | [-0.0048, +0.0128] | 0.19 | 0.0207 | false | false |
| T3 novelty MNIST-PN r (k=10) | P1 wiring beyond degrees (R-b - D-b) | +0.0179 | [-0.0315, +0.0674] | [-0.0290, +0.0605] | 0.17 | 0.0795 | false | false |
| T3 novelty MNIST-PN r (k=10) | P2 degree heterogeneity (D-b - U-b) | -0.0305 | [-0.0759, +0.0150] | [-0.0717, +0.0118] | -0.31 | 0.0818 | false | false |
| T3 novelty MNIST-PN r (k=10) | P3 weights (R-w - S-w) | -0.0364 | [-0.0885, +0.0157] | [-0.0843, +0.0096] | -0.33 | 0.0813 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs uniform (binary) (R-b - U-b) | -0.0125 | [-0.0551, +0.0300] | [-0.0527, +0.0258] | -0.14 | 0.0818 | false | false |
| T3 novelty MNIST-PN r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0047 | [-0.0480, +0.0385] | [-0.0453, +0.0342] | -0.05 | 0.0834 | false | false |
| T3 novelty MNIST-PN r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0465 | [+0.0106, +0.0824] | [+0.0140, +0.0795] | 0.61 | 0.0705 | true | false |
| T3 novelty MNIST-PN r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0501 | [-0.0894, -0.0107] | [-0.0870, -0.0156] | -0.59 | 0.0926 | true | false |
| T3 novelty MNIST-PN r (k=20) | P3 weights (R-w - S-w) | -0.0164 | [-0.0486, +0.0159] | [-0.0471, +0.0120] | -0.24 | 0.0831 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs uniform (binary) (R-b - U-b) | -0.0035 | [-0.0382, +0.0311] | [-0.0330, +0.0303] | -0.05 | 0.0926 | false | false |
| T3 novelty MNIST-PN r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0013 | [-0.0343, +0.0370] | [-0.0304, +0.0350] | 0.02 | 0.1003 | false | false |
| T3 novelty MNIST-PN r (k=40) | P1 wiring beyond degrees (R-b - D-b) | +0.0125 | [-0.0297, +0.0547] | [-0.0264, +0.0509] | 0.14 | 0.0869 | false | false |
| T3 novelty MNIST-PN r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0162 | [-0.0540, +0.0216] | [-0.0502, +0.0186] | -0.20 | 0.0682 | false | false |
| T3 novelty MNIST-PN r (k=40) | P3 weights (R-w - S-w) | -0.0011 | [-0.0263, +0.0242] | [-0.0236, +0.0225] | -0.02 | 0.0817 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs uniform (binary) (R-b - U-b) | -0.0037 | [-0.0441, +0.0367] | [-0.0411, +0.0347] | -0.04 | 0.0682 | false | false |
| T3 novelty MNIST-PN r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0123 | [-0.0351, +0.0597] | [-0.0299, +0.0570] | 0.12 | 0.0750 | false | false |
| T3 novelty MNIST-PN r (k=5) | P1 wiring beyond degrees (R-b - D-b) | -0.0022 | [-0.0395, +0.0352] | [-0.0365, +0.0322] | -0.03 | 0.0784 | false | false |
| T3 novelty MNIST-PN r (k=5) | P2 degree heterogeneity (D-b - U-b) | +0.0101 | [-0.0280, +0.0482] | [-0.0248, +0.0450] | 0.12 | 0.0658 | false | false |
| T3 novelty MNIST-PN r (k=5) | P3 weights (R-w - S-w) | +0.0192 | [-0.0298, +0.0682] | [-0.0272, +0.0632] | 0.18 | 0.0794 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs uniform (binary) (R-b - U-b) | +0.0079 | [-0.0347, +0.0505] | [-0.0297, +0.0483] | 0.09 | 0.0658 | false | false |
| T3 novelty MNIST-PN r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0490 | [+0.0091, +0.0890] | [+0.0124, +0.0848] | 0.57 | 0.0806 | true | false |
| T3 novelty odour r (k=10) | P1 wiring beyond degrees (R-b - D-b) | +0.1180 | [+0.0878, +0.1481] | [+0.0902, +0.1450] | 1.83 | 0.0660 | true | true |
| T3 novelty odour r (k=10) | P2 degree heterogeneity (D-b - U-b) | -0.0435 | [-0.0879, +0.0008] | [-0.0824, -0.0010] | -0.46 | 0.0708 | false | false |
| T3 novelty odour r (k=10) | P3 weights (R-w - S-w) | -0.0317 | [-0.0757, +0.0123] | [-0.0704, +0.0091] | -0.34 | 0.0863 | false | false |
| T3 novelty odour r (k=10) | real vs uniform (binary) (R-b - U-b) | +0.0744 | [+0.0331, +0.1157] | [+0.0373, +0.1133] | 0.84 | 0.0708 | true | true |
| T3 novelty odour r (k=10) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0042 | [-0.0414, +0.0498] | [-0.0362, +0.0453] | 0.04 | 0.0796 | false | false |
| T3 novelty odour r (k=20) | P1 wiring beyond degrees (R-b - D-b) | +0.0375 | [+0.0076, +0.0674] | [+0.0095, +0.0641] | 0.59 | 0.0743 | true | false |
| T3 novelty odour r (k=20) | P2 degree heterogeneity (D-b - U-b) | -0.0337 | [-0.0746, +0.0073] | [-0.0710, +0.0045] | -0.38 | 0.0598 | false | false |
| T3 novelty odour r (k=20) | P3 weights (R-w - S-w) | -0.0535 | [-0.0862, -0.0208] | [-0.0845, -0.0246] | -0.77 | 0.0659 | true | false |
| T3 novelty odour r (k=20) | real vs uniform (binary) (R-b - U-b) | +0.0038 | [-0.0273, +0.0349] | [-0.0226, +0.0343] | 0.06 | 0.0598 | false | false |
| T3 novelty odour r (k=20) | real vs deg-preserving (weighted) (R-w - D-w) | -0.0086 | [-0.0444, +0.0272] | [-0.0413, +0.0239] | -0.11 | 0.0635 | false | false |
| T3 novelty odour r (k=40) | P1 wiring beyond degrees (R-b - D-b) | +0.0513 | [+0.0225, +0.0802] | [+0.0249, +0.0776] | 0.83 | 0.0775 | true | false |
| T3 novelty odour r (k=40) | P2 degree heterogeneity (D-b - U-b) | -0.0112 | [-0.0520, +0.0296] | [-0.0480, +0.0267] | -0.13 | 0.0601 | false | false |
| T3 novelty odour r (k=40) | P3 weights (R-w - S-w) | +0.0589 | [+0.0345, +0.0833] | [+0.0368, +0.0808] | 1.13 | 0.0627 | true | false |
| T3 novelty odour r (k=40) | real vs uniform (binary) (R-b - U-b) | +0.0402 | [+0.0129, +0.0674] | [+0.0145, +0.0649] | 0.69 | 0.0601 | true | false |
| T3 novelty odour r (k=40) | real vs deg-preserving (weighted) (R-w - D-w) | +0.0628 | [+0.0358, +0.0898] | [+0.0379, +0.0874] | 1.09 | 0.0733 | true | false |
| T3 novelty odour r (k=5) | P1 wiring beyond degrees (R-b - D-b) | -0.0294 | [-0.0755, +0.0167] | [-0.0710, +0.0128] | -0.30 | 0.0782 | false | false |
| T3 novelty odour r (k=5) | P2 degree heterogeneity (D-b - U-b) | -0.0586 | [-0.1101, -0.0070] | [-0.1037, -0.0095] | -0.53 | 0.0543 | true | true |
| T3 novelty odour r (k=5) | P3 weights (R-w - S-w) | +0.0645 | [+0.0158, +0.1132] | [+0.0199, +0.1092] | 0.62 | 0.0854 | true | false |
| T3 novelty odour r (k=5) | real vs uniform (binary) (R-b - U-b) | -0.0880 | [-0.1167, -0.0593] | [-0.1147, -0.0619] | -1.43 | 0.0543 | true | true |
| T3 novelty odour r (k=5) | real vs deg-preserving (weighted) (R-w - D-w) | +0.1204 | [+0.0750, +0.1657] | [+0.0777, +0.1604] | 1.24 | 0.0774 | true | true |

