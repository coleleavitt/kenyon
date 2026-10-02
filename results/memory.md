| structure | bytes | note |
|---|---|---|
| FlyHash MNIST d=784 m=7840 fan-in 78 | 2477444 | projection (u32 indices + offsets) |
| FlyHash SIFT d=128 m=1280 fan-in 12 | 66564 | projection (u32 indices + offsets) |
| LSH dense d=784 k=4 | 12544 | f32 Gaussian matrix |
| FlyIndex 10k SIFT-like items, k=16 | 2609280 | 260.9 bytes/item incl. postings |
| DenseFlyIndex 10k items, 1280-bit h1, 64-bit h2 | 2720000 | 10000 bins |
| FlyBloom m=150k, static (delta = epsilon = 0) | 4800004 | of which projection 4200004; filter cells 600000 bytes (classical BF: 18,750 bytes) |
| FlyBloom m=150k, with recovery (epsilon > 0) | 6000004 | of which projection 4200004; filter cells 1800000 bytes (classical BF: 18,750 bytes) |
| FlyModel MNIST-20 (84 -> 3200 -> 20) | 396804 | projection + 20x3200 f32 weights |

Per stored item: a FlyHash code costs 8 bytes per active unit (u32 index + f32 value); binary codes cost m/8 bytes as bitsets or 4k bytes as index lists.
