# FlyHash vs HNSW — preregistration

Committed before any run. Purpose: test the README's claim that kenyon "is **not** a replacement
for HNSW" with a head-to-head instead of a citation.

## Data
- **SIFT10K** (`~/datasets/siftsmall`): 10,000 base vectors (d = 128), 100 queries, the shipped L2
  ground truth.
- **MNIST** (`~/datasets/mnist`): the 60,000 training images as base (d = 784, pixels / 255), the
  first 1,000 test images as queries; ground truth = exact L2 top-10 by brute force.

## Methods
1. **Exact** brute-force L2 (reference; recall 1 by definition; latency reference).
2. **HNSW**: `instant-distance` 0.6, L2 distance, `ef_construction = 100`, `ef_search = 100`,
   seed 0 (crate defaults except the seed).
3. **FlyIndex**: `FlyHashParams::paper(d, k, seed)` (m = 10 d, 10% fan-in, mean-centred), top-10
   by KC overlap; `k ∈ {16, 32, 64}`.
4. **FlyIndex + re-rank**: as 3, then exact L2 re-rank of the top 100 overlap candidates (the fair
   use of a cheap code: candidate generation).
FlyHash results are averaged over seeds 1–5. All timings are single-threaded query latency
(mean over queries, release build), measured on the same machine in one process.

## Metrics
Recall@10 against the exact L2 top-10; mean query latency (µs); index memory where the library
exposes it (`FlyIndex` + `FlyHash::memory_bytes`; raw vector bytes for Exact and HNSW, graph not
counted, stated as a lower bound).

## Verdict rule (per dataset, then overall)
- **Supported** ("kenyon is not a replacement for HNSW"): HNSW's recall@10 ≥ the best FlyIndex
  variant's recall@10 + 0.10, and HNSW's latency ≤ that variant's latency.
- **Refuted**: some FlyIndex variant reaches HNSW's recall@10 − 0.02 or better at ≤ HNSW's latency.
- **Mixed**: anything else (e.g. FlyIndex + re-rank matches recall but is slower).
The README text changes to whatever the verdict says.
