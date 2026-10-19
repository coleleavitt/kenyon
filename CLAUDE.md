# kenyon Claude Instructions

## Mission
`kenyon` is a small, safe, deterministic Rust crate of algorithms from the fruit fly's mushroom body:
FlyHash, DenseFly, the fly Bloom filter (novelty with decay) and the FlyModel continual learner. It
also holds a reproduction harness that tests the published claims, and a preregistered wiring study
that asks whether the real FlyWire connectome hashes better than random wiring.

What must be preserved:
- **Honest results.** Every claim in `README.md` / `RESULTS.md` comes from a run whose pass bar was
  committed *before* the run (`ace57ec` predeclared the reproduction bars; `research/wiring/PREREG.md`
  the wiring study). A result that misses its bar is reported as missed, never re-scored.
- **Determinism.** A seed gives the same hash on every platform and version (own xoshiro256**
  generator in `src/rng.rs`, never `rand`; pinned by `tests/golden.rs`).
- **A tiny library.** One runtime dependency (`thiserror`). Heavy deps (`candle`, `rayon`, `flate2`,
  `anyhow`) live only in the unpublished `repro/` crate.
- **The scope statement.** kenyon is for novelty scoring, continual learning without replay,
  tiny-memory codes and experiments with measured wiring. It is **not** a replacement for HNSW,
  product quantisation or learned hashing; the README says so, and features must not claim otherwise.

## Architecture First
Workspace members: `.` (`kenyon`, the library) and `repro/` (`kenyon-repro`, `publish = false`).

Library layers, bottom-up (a module may use only modules below it):

| Layer | Module | Owns |
|---|---|---|
| 0 | `error` | `KenyonError`, `Result` |
| 0 | `rng` | portable seeded generator (xoshiro256**) |
| 1 | `preprocess` | `Preprocess`: none / centre / unit / divisive normalisation |
| 1 | `projection` | `SparseProjection` (fixed fan-in, `from_edges` for measured wiring), `DenseProjection`, `Expansion` |
| 1 | `wta` | winner-take-all (top-k) |
| 2 | `code` | `KcCode` (sparse valued tag), `BitCode` (packed bits): overlap, Hamming, Jaccard |
| 3 | `hash` | `FlyHash`, `DenseFly`, `FlyHashParams::paper`, `TagRule` (WTA / All / Fixed = ablations), `Scratch` |
| 4 | `index` | `FlyIndex` (inverted lists, overlap / euclidean query), `DenseFlyIndex` (multi-probe bins) |
| 4 | `bloom` | `FlyBloom` (Eq. 2 decay δ / recovery ε, applied lazily so insert is O(k)) |
| 4 | `model` | `FlyModel` (sparse coding + partial-freezing associative layer; `Coding`, `LearningRule` ablations) |

Outside the library:
- `repro/src/{data,metrics,stats}.rs` plus binaries `r1_flyhash`, `r2_bloom`, `encoder`,
  `r3_flymodel`, `wiring`, `memory` (one per experiment).
- `results/` holds the committed raw outputs (`*.json`, `*.md`, `*.log`) that RESULTS.md cites.
- `research/wiring/` holds `PREREG.md` (fixed before running) and `PAPER.md` (the write-up).
- `scripts/`: `download_datasets.sh` (sha256-pinned, into `~/datasets`), `reproduce.sh`.
- `benches/throughput.rs` (criterion).

Composition rules:
- Ablations are first-class types or enum variants, not flags in an `if`.
- Measured wiring enters only through `SparseProjection::from_edges`.
- New algorithms (e.g. FlyRAG, below) build on `hash` / `code` / `bloom`; they do not reach into
  `projection` internals.

## Current State
- Library, reproductions R1–R3, the wiring study and benches are done and committed
  (`bfcf5b8` … `26066fc`). `publish = false`; nothing pushed; no remote.
- Reproduction verdicts (README table): R1 ratio reproduced, level partial or fail by mAP formula;
  R2 SIFT not reproduced (ours higher), odours partial; R3 reproduced at 20 classes. Wiring study:
  real PN→KC wiring is **not** better than degree-preserving random; the PN out-degree sequence is
  what matters.
- crates.io names `kenyon`, `flyhash`, `flybloom`, `flymodel`, `kenyon-cells`, `mushroom-body` were
  all free on 2026-10-02. Publishing is the operator's decision.
- CI: `.github/workflows/ci.yml` (lint, cross-platform test + golden, MSRV 1.85.0, repro clippy) and
  the same checks locally in `scripts/ci.sh`. MSRV 1.85 verified (lib + tests on 1.85.0, 2026-10-19;
  1.85 is also the edition-2024 floor).
- Storage compacted (2026-10-19): projection indices at 1/2/4 bytes with no offsets at a fixed
  fan-in; static Bloom cells bit-packed (m/8 bytes). Static FlyBloom at m = 150k: 4.8 MB → 0.92 MB.
  Hashes bit-identical (golden unchanged).
- HNSW head-to-head done (`research/hnsw/`): HNSW beats every FlyIndex configuration on recall and
  latency; predeclared verdict "mixed" (best re-ranked FlyIndex within 0.062 recall).
- **Before crates.io:** flip `publish`; consider multi-probe FlyHash and SIMD popcount.
- **Wiring paper:** MCNS replication done (confirmatory claim replicates; odour degree benefit
  does not); post-hoc Holm/BH correction done; all 12 references verified (Fishilevich & Vosshall 2005
  via Crossref, with its 2007 erratum). Before arXiv: a DoOR consensus matrix covering more
  glomeruli; ask the Navlakha lab for the exact FlyHash mAP formula and the paper's SIFT subset.
- **FlyRAG built** (`flyrag/`, `research/flyrag/`): bars 1, 2, 4, 5 met. Follow-up (A2/A3): the
  strict independent-label control gains nothing (the gain is the outcome signal); novelty fails
  (the first run's score AUC 0.997 was a leak; leak-free 0.64, flag 0.637 after a dev-seed choice).
  SciFact (`research/flyrag-scifact/`): learning transfers to new queries (+0.022 nDCG@10, no harm
  on unseen), but base retrieval is 0.065 vs BM25 0.660. Next: an IDF-weighted text front end, or
  FlyRAG as a re-ranking layer over BM25 (each preregistered); novelty against stored documents;
  persistence.
- **FlyRAG design notes** (design input: `~/RustProjects/active/ckl/docs/replacement/31-fly-ideas.md`
  idea 4; the same core idea is built and tested as ckl's B2 graph,
  `~/RustProjects/active/ckl/crates/ckl-neocortex/src/graph/`). Retrieval that *learns from verified
  usefulness* without forgetting: sparse code + overlap recall, per-document use/skip banks
  (opposing traces, never overwrite), bounded prediction-error updates, forward-only credit, and a
  `FlyBloom` novelty flag when a query overlaps nothing. Plan it as its own workspace crate
  (`flyrag/`) so `kenyon` stays dependency-light. Its first experiment, with the bar predeclared:
  beat a frozen (no-learning) FlyRAG by ≥ 3 nDCG@10, while a shuffled-outcome control does not.
  Its pitch is learning from outcomes and novelty, **not** better raw search than HNSW.

## Working Rules
- Toolchain `rustc-master` (`rust-toolchain.toml`); plain `cargo`, never `+stable`.
- `unsafe_code = "forbid"`; `clippy::unwrap_used = "deny"` (tests may unwrap, `clippy.toml`).
  Errors through `KenyonError`. `#![warn(missing_docs)]`: every public item is documented, with the
  paper and page it implements where there is one.
- Never add `rand` or any other runtime dependency to `kenyon` without the operator's say-so.
- Every new random structure takes a `u64` seed and is covered by a golden test.
- **Experiment discipline:** write the bar (metric, tolerance, controls) into `RESULTS.md` or a
  `PREREG.md` and commit it *before* running. Every experiment has its controls (dense projection,
  no WTA, random tag, degree-preserving null, shuffled labels as fits). Deviations from a paper's
  protocol are listed, not hidden.
- Never `#[ignore]` a failing test, weaken an assertion, or `#[allow]` a lint to get green.
- Datasets live in `~/datasets` and `~/flywire-data`, never in the repo. Never load pickle files.
- Don't commit or push unless asked; there is no remote.

## Validation
```sh
timeout 300  cargo fmt --check
timeout 1200 cargo clippy --workspace --all-targets -- -D warnings
timeout 1800 cargo test --workspace
timeout 1800 cargo test --test golden          # determinism pins
scripts/ci.sh                                  # everything CI runs, incl. the 1.85.0 MSRV check
timeout 3600 cargo bench --bench throughput    # optional, ~minutes
scripts/download_datasets.sh                   # ~85 MB into ~/datasets, sha256-checked
scripts/reproduce.sh                           # every experiment, seeded (~1.5 h on 24 cores)
```

## Project Claude Layout
- `.claude/CLAUDE.md` points here; `AGENTS.md` mirrors the core rules; `MEMORY.md` is local
  (gitignored) handoff state.
- `.claude/agents/researcher.md`: read-only paper/code/results investigation.
- `.claude/agents/verifier.md`: the validation gate above, with timeouts.
- `.claude/commands/audit.md`: architecture and claims audit. `.claude/commands/repro.md`:
  reproduce one result with expected vs actual evidence.
- `.claude/rules/{code-style,security,testing}.md`.
- `.claude/skills/kenyon-experiment/SKILL.md`: how to run a new experiment the predeclared way.
