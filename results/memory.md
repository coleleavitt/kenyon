| structure | bytes | note |
|---|---|---|
| FlyHash MNIST d=784 m=7840 fan-in 78 | 1223040 | projection (compact: 1, 2 or 4-byte indices; no offsets at fixed fan-in) |
| FlyHash SIFT d=128 m=1280 fan-in 12 | 15360 | projection (compact: 1, 2 or 4-byte indices; no offsets at fixed fan-in) |
| LSH dense d=784 k=4 | 12544 | f32 Gaussian matrix |
| FlyIndex 10k SIFT-like items, k=16 | 2609280 | 260.9 bytes/item incl. postings |
| DenseFlyIndex 10k items, 1280-bit h1, 64-bit h2 | 2720000 | 10000 bins |
| FlyBloom m=150k, static (delta = epsilon = 0) | 918752 | of which projection 900000; filter cells 18752 bytes (bit-packed when delta = epsilon = 0; classical BF: 18,750 bytes) |
| FlyBloom m=150k, with recovery (epsilon > 0) | 2700000 | of which projection 900000; filter cells 1800000 bytes (bit-packed when delta = epsilon = 0; classical BF: 18,750 bytes) |
| FlyModel MNIST-20 (84 -> 3200 -> 20) | 288000 | projection + 20x3200 f32 weights |

Per stored item: a FlyHash code costs 8 bytes per active unit (u32 index + f32 value); binary codes cost m/8 bytes as bitsets or 4k bytes as index lists.
