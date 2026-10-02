# kenyon

Algorithms from the fruit fly's mushroom body, in safe, deterministic Rust:

| Type | What it is | Paper |
|---|---|---|
| `FlyHash` | sparse binary random expansion (d → m ≫ d) + winner-take-all; a locality-sensitive hash | Dasgupta, Stevens & Navlakha, *Science* 358:793 (2017) |
| `DenseFly` | the same expansion with sign binarisation, plus a pseudo-hash for multi-probe | Sharma & Navlakha, arXiv:1812.01844 (2018) |
| `FlyIndex`, `DenseFlyIndex` | inverted-list similarity search over KC codes; DenseFly multi-probe bins | same |
| `FlyBloom` | the fly Bloom filter: continuous novelty score, decay δ and recovery ε | Dasgupta, Sheehan, Stevens & Navlakha, *PNAS* 115:13093 (2018) |
| `FlyModel` | continual learner: frozen encoder → sparse code → partial-freezing associative layer | Shen, Dasgupta & Navlakha, arXiv:2107.07617 (2021) |
| `SparseProjection::from_edges` | a projection from measured wiring, e.g. a connectome's PN→KC synapses | — |

```rust
use kenyon::{BloomParams, FlyBloom, FlyHash, FlyHashParams, FlyIndex};

let hasher = FlyHash::new(FlyHashParams::paper(128, 16, /*seed*/ 42))?; // m = 10d, 10% fan-in, top-16
let mut index = FlyIndex::new(hasher.n_kc());
for x in &vectors { index.insert(hasher.hash(x)?)?; }
let hits = index.query_overlap(&hasher.hash(&query)?, 10)?;

let mut novelty = FlyBloom::new(hasher, BloomParams { decay: 0.0, recovery: 0.0 })?;
let score = novelty.novelty(&query)?; // 1 = nothing like it seen, 0 = familiar
```

## Design

- **No `unsafe`, no `unwrap`.** Errors are a `thiserror` enum. There is one runtime dependency
  (`thiserror`).
- **Seeded and portable.** Every random structure comes from a `u64` seed through the crate's own
  xoshiro256** generator, never `rand`. A seed gives the same hash on every platform and in
  every version. `tests/golden.rs` pins this.
- **Ablations are first-class.** `TagRule::Fixed` gives the random-tag control. `TagRule::All`
  gives no WTA. `DenseProjection` gives the dense Gaussian control. `Coding::Dense` and the
  perceptron rules v1–v3 give the FlyModel ablations.
- **Lazy recovery.** `FlyBloom` applies recovery lazily, so an insert costs O(k), not O(m). It is
  exactly equal to Eq. 2; a unit test checks this against the eager update.

## What it is for, and what it is not

Fly hashing is a good cheap code. It is not state of the art. Use it for:

- novelty and familiarity scoring;
- continual learning without replay;
- tiny-memory similarity codes;
- experiments with measured wiring.

It is **not** a replacement for HNSW, product quantisation or learned hashing (ITQ, BioHash). The
literature shows FlyHash loses to data-dependent methods (doc 31 survey; BioHash 2020 Table 1).
Our own reproduction shows its advantage over LSH vanishes at k ≥ 16 under the paper's
valued-tag metric.

## Reproduction (summary; details and every deviation in [RESULTS.md](RESULTS.md))

Bars were predeclared in RESULTS.md part 1 (commit `ace57ec`) before any run. Tolerance is ±2
points.

<!-- RESULTS-SUMMARY -->

## Wiring study

[research/wiring/PAPER.md](research/wiring/PAPER.md) asks whether the real FlyWire v783 PN→KC
wiring (285 PNs → 4,723 KCs) hashes better than random wiring with the same degrees. It was
preregistered in [research/wiring/PREREG.md](research/wiring/PREREG.md).

## Regenerate everything

```sh
scripts/download_datasets.sh   # ~85 MB into ~/datasets, sha256-pinned
scripts/reproduce.sh           # all experiments + benches, seeded (~1.5 h on 24 cores)
```

The reproduction harness lives in `repro/` (`kenyon-repro`, unpublished). It uses `candle` only
to train the frozen LeNet5 encoder that FlyModel needs.

## Licence

Code: MIT OR Apache-2.0.

Data used by the experiments keeps its own licence and is not redistributed here:

- FlyWire FAFB v783: CC-BY 4.0. Cite Dorkenwald et al. 2024, *Nature* 634:124, and Schlegel
  et al. 2024, *Nature* 634:139.
- DoOR 2.0: CC BY-SA 4.0. Cite Münch & Galizia 2016, *Sci Rep* 6:21841.
- MNIST, Fashion-MNIST, KMNIST, ANN_SIFT10K: see their sources.
