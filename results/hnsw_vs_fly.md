# FlyHash vs HNSW (research/hnsw/PREREG.md)

HEAD `ef0ca1d`. Single-threaded query latency; FlyIndex averaged over seeds 1–5. SIFT exact-vs-shipped ground truth agreement: 1.0000.

## SIFT10K

| method | recall@10 | latency (µs/query) | memory (bytes) |
|---|---|---|---|
| exact | 1.000 | 486.4 | 5120000 |
| hnsw (ef 100; build 0.8 s, multi-threaded) | 1.000 | 138.6 | 5120000 |
| FlyIndex k=16 | 0.298 | 227.5 | 2631680 |
| FlyIndex k=16 + re-rank top 100 | 0.702 | 236.0 | 7751680 |
| FlyIndex k=32 | 0.399 | 381.0 | 4871040 |
| FlyIndex k=32 + re-rank top 100 | 0.850 | 391.5 | 9991040 |
| FlyIndex k=64 | 0.500 | 783.3 | 9257184 |
| FlyIndex k=64 + re-rank top 100 | 0.938 | 796.1 | 14377184 |

Verdict: **mixed**

## MNIST (60k base, 1k queries)

| method | recall@10 | latency (µs/query) | memory (bytes) |
|---|---|---|---|
| exact | 1.000 | 28686.9 | 188160000 |
| hnsw (ef 100; build 24.7 s, multi-threaded) | 1.000 | 1013.3 | 188160000 |
| FlyIndex k=16 | 0.235 | 1107.0 | 17037520 |
| FlyIndex k=16 + re-rank top 100 | 0.616 | 1171.1 | 205197520 |
| FlyIndex k=32 | 0.353 | 2264.7 | 30190928 |
| FlyIndex k=32 + re-rank top 100 | 0.797 | 2354.5 | 218350928 |
| FlyIndex k=64 | 0.465 | 4784.6 | 56593504 |
| FlyIndex k=64 + re-rank top 100 | 0.910 | 4997.5 | 244753504 |

Verdict: **mixed**

HNSW memory counts the raw vectors only (the crate does not expose its graph size): a lower bound.

