# FlyHash vs HNSW — results

Run of `research/hnsw/PREREG.md` (`hnsw_vs_fly`, committed at `ef0ca1d` before its first run).
Raw output: `results/hnsw_vs_fly.{md,json,log}`. Our exact search agrees with SIFT10K's shipped
ground truth (recall@10 1.000).

| | method | recall@10 | latency (µs/query, 1 thread) | memory |
|---|---|---|---|---|
| SIFT10K | exact | 1.000 | 486 | 5.1 MB vectors |
| | **HNSW** (ef 100) | **1.000** | **139** | ≥ 5.1 MB (graph not counted) |
| | FlyIndex k = 16 / 32 / 64 | 0.298 / 0.399 / 0.500 | 228 / 381 / 783 | 2.6 / 4.9 / 9.3 MB |
| | FlyIndex + re-rank top 100, k = 16 / 32 / 64 | 0.702 / 0.850 / 0.938 | 236 / 392 / 796 | + vectors |
| MNIST 60k | exact | 1.000 | 28,687 | 188 MB vectors |
| | **HNSW** (ef 100) | **1.000** | **1,013** | ≥ 188 MB |
| | FlyIndex k = 16 / 32 / 64 | 0.235 / 0.353 / 0.465 | 1,107 / 2,265 / 4,785 | 17 / 30 / 57 MB |
| | FlyIndex + re-rank top 100, k = 16 / 32 / 64 | 0.616 / 0.797 / 0.910 | 1,171 / 2,355 / 4,998 | + vectors |

## Verdict
**Mixed on both datasets, by the predeclared rule.** The rule called the claim "supported" only if
HNSW's recall exceeded the *best* FlyIndex variant's by ≥ 0.10; FlyIndex k = 64 with exact
re-ranking came within 0.062 (SIFT) and 0.090 (MNIST).

The facts behind the verdict are one-sided: **HNSW had higher recall and lower latency than every
FlyIndex configuration on both datasets** (the closest-recall variant was 5.7× slower on SIFT and
4.9× slower on MNIST). FlyIndex's only advantage is memory when it stores codes alone (2.6 MB vs
5.1 MB of raw vectors on SIFT), and then its recall is 0.24–0.50.

## What changes
The README's statement stands and is now backed by this measurement instead of a citation:
kenyon is not a replacement for HNSW for nearest-neighbour search. Use fly codes for novelty,
continual learning and tiny-memory candidate generation.

## Limits
One HNSW implementation (`instant-distance` 0.6) at its default-like settings; no PQ or learned
hashing baselines; single-threaded timings on one machine; HNSW's graph memory not measured.
