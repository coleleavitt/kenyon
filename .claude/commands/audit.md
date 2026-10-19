---
description: Audit kenyon's architecture and every public claim against the code and committed results.
---

Audit the current tree (or the diff named in $ARGUMENTS) for:

1. **Layering** (root `CLAUDE.md` table): no module uses a module above it; ablations are types or
   enum variants; measured wiring enters only through `SparseProjection::from_edges`.
2. **Library hygiene:** one runtime dependency; no `unsafe`; no `unwrap` outside tests; every public
   item documented, with paper and page where one applies; `KenyonError` for every failure.
3. **Determinism:** every random structure is seeded through `src/rng.rs`; `tests/golden.rs` covers
   it.
4. **Claims:** every number in `README.md` and `RESULTS.md` traces to a file in `results/` and to a
   bar committed before its run (`git log` on `RESULTS.md` / `research/wiring/PREREG.md`). Flag any
   claim without a source, any re-scored result and any undocumented protocol deviation.
5. **Scope:** nothing claims kenyon beats HNSW, PQ or learned hashing.

Output findings ranked most severe first, each with `path:line` and a concrete fix. Don't edit files.
