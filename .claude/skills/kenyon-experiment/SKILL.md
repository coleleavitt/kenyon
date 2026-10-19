---
name: kenyon-experiment
description: Run a new kenyon experiment (a paper reproduction, an ablation, the wiring study's replication, or a FlyRAG evaluation) the predeclared way — bar and controls committed before the run, seeded, results written to results/, honest verdict. Use whenever a kenyon task produces a number that will be reported.
---

# kenyon experiment workflow

kenyon's results are trusted because every bar was fixed before the run. Keep it that way.

## 1. Predeclare (commit before any run)
Write, in `RESULTS.md` (a new part) or a `research/<study>/PREREG.md`:
- the claim being tested and its source (paper, page, table);
- the metric, its exact formula (e.g. which mAP normalisation), and the tolerance (kenyon uses ±2
  points for reproductions);
- the dataset, split and preprocessing, with sha256 if it is new (add it to
  `scripts/datasets.sha256` and `scripts/download_datasets.sh`);
- the **controls**, chosen from: dense projection, no WTA (`TagRule::All`), random tag
  (`TagRule::Fixed`), LSH baseline, degree-preserving or uniform random wiring, shuffled labels or
  outcomes, a frozen (no-learning) twin, capacity-matched baselines;
- seeds (≥ 20 for wiring-style comparisons) and the statistics (paired by seed, bootstrap CI);
- what counts as reproduced, partial, and not reproduced.

Commit it (`git commit -m "predeclare …"`) and note the commit hash. Only then run.

## 2. Implement in `repro/`
- One binary per experiment in `repro/src/bin/`, sharing `data.rs`, `metrics.rs`, `stats.rs`.
- Every random choice is seeded; print the seeds and the git HEAD into the log.
- Write machine output to `results/<name>.json`, a readable `results/<name>.md`, and the run log.
- Add the command to `scripts/reproduce.sh`.

## 3. Run and report
- Run under `timeout`; never overwrite committed results from a different protocol.
- Report against the predeclared bar exactly: reproduced / partial / not reproduced. A protocol change
  after the bar is an **amendment**: record it as its own commit before the affected run, and say which
  runs came before it (see commit `b8c6447`, amendment A1, for the pattern).
- Findings that go against a paper's framing go into `RESULTS.md` and the README summary.

## 4. Never
- re-score a missed bar with a friendlier metric without reporting both;
- drop a control because it made the result look worse;
- claim kenyon beats HNSW, PQ or learned hashing without a predeclared head-to-head.
