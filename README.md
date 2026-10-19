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
literature shows FlyHash loses to data-dependent methods (doc 31 survey; BioHash 2020 Table 1),
and our own preregistered head-to-head ([research/hnsw/RESULTS.md](research/hnsw/RESULTS.md))
found HNSW had higher recall *and* lower latency than every FlyIndex configuration on SIFT10K and
MNIST (e.g. SIFT: HNSW recall@10 1.000 at 139 µs; best FlyIndex 0.938 at 796 µs).
Our own reproduction shows its advantage over LSH vanishes at k ≥ 16 under the paper's
valued-tag metric.

## Reproduction (summary; details and every deviation in [RESULTS.md](RESULTS.md))

Bars were predeclared in RESULTS.md part 1 (commit `ace57ec`) before any run. Tolerance is ±2
points.

| Claim | Published | Ours | Verdict |
|---|---|---|---|
| FlyHash vs LSH, MNIST k = 4 (mAP %) | 44.8 vs 16.0 | 40.1 vs 16.8 (hits-normalised AP); 10.4 vs 3.5 (predeclared AP/R) | ratio reproduced; level partial (−4.7) or fail, depending on the mAP formula |
| Fly Bloom filter novelty r, SIFT | 0.535 | 0.49–0.61 over k | not reproduced: **higher** than published |
| Fly Bloom filter r, odours, k = 40 | 0.657 (LSBF 0.537) | 0.630 (LSBF 0.547) | partial (−2.7); LSBF reproduced |
| FlyModel MNIST-20, after 10 / 20 classes | 0.86 / 0.75 | 0.889 / 0.754 | 20 classes reproduced; 10 classes +2.9 (encoder-sensitive) |
| Dense-coding ablation, 20 classes | 0.07 | 0.049 | collapse reproduced (−2.1) |

All the ablations behave as the papers say:
- random tag or no expansion → LSH level;
- dense coding → collapse;
- perceptron rules → forgetting.

Two findings go against the papers' framing:
- Valued-tag FlyHash loses to LSH at k ≥ 16.
- On MNIST-20, a capacity-matched FlyModel on raw pixels is only 1–2 points below the
  pretrained encoder.

## Wiring study

[research/wiring/PAPER.md](research/wiring/PAPER.md) asks whether the real FlyWire v783 PN→KC
wiring (285 PNs → 4,723 KCs) hashes better than random wiring with the same degrees. It was
preregistered in [research/wiring/PREREG.md](research/wiring/PREREG.md).

The answer is **no**:
- **Generic inputs.** Real wiring is slightly worse than its degree-preserving null: −0.016
  mAP pooled, −0.006 in one hemisphere.
- **The degree sequence.** The skewed PN out-degree is what matters. It costs about 4–5 mAP
  points on generic inputs and gains about 2 on odours.
- **Novelty.** No robust difference.

## FlyRAG (workspace crate `flyrag`)

Retrieval that learns from verified usefulness: sparse KC codes for candidates, per-document "use"
and "skip" readout banks depressed by verified outcomes (bounded, prediction-error driven,
forward-only, decaying), and a fly Bloom filter novelty score. Preregistered evaluations:
- **Synthetic** ([research/flyrag](research/flyrag/)), where lexical similarity misleads: learning
  beats a frozen copy by +0.034 nDCG@10 [+0.028, +0.040], keeps 83% of its gain after learning new
  topics, and a strict control (labels independent of the documents) gains nothing, so the gain
  comes from the outcome signal. Novelty detection **failed**: AUC 0.64 once a measurement leak
  was fixed.
- **Real text, BEIR SciFact** ([research/flyrag-scifact](research/flyrag-scifact/)): learning on
  the train queries lifts the separate test queries by +0.022 [+0.016, +0.028] with no harm to
  unrelated queries, but FlyRAG's base retrieval is far below BM25 (nDCG@10 0.065 vs 0.660).
- **As a learning re-ranker over BM25** ([research/flyrag-next](research/flyrag-next/)), via
  `FlyRag::rerank`: SciFact nDCG@10 0.660 → **0.676** (+0.016 [+0.009, +0.023]), random labels give
  nothing, unrelated queries unchanged. This is the recommended use. An IDF-weighted 16k-bucket front
  end lifts FlyRAG's own retrieval 0.065 → 0.249, still far below BM25; novelty against stored
  documents reaches AUC 0.74, below its 0.90 bar.

## Wiring study, replicated

The wiring result above replicates in the male CNS (MCNS v1.0): real − degree-preserving =
−0.020 [−0.026, −0.014]. The odour benefit of the degree sequence does not replicate. Details,
the post-hoc multiplicity correction and the checked references are in
[research/wiring/PAPER.md](research/wiring/PAPER.md).

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
