---
description: Reproduce one kenyon result (R1 FlyHash, R2 Bloom, R3 FlyModel, wiring) and show expected vs actual.
---

Reproduce the result named in $ARGUMENTS (e.g. `r1`, `r2 odours`, `r3`, `wiring`).

1. Find its predeclared bar (`RESULTS.md` part 1 or `research/wiring/PREREG.md`) and its committed
   output in `results/`.
2. Check the datasets are present and match `scripts/datasets.sha256`; if not, say so and stop
   (`scripts/download_datasets.sh` fetches them into `~/datasets`).
3. Run only that experiment's binary from `repro/` (see `scripts/reproduce.sh` for the exact command
   and seeds), under a timeout, writing to a scratch directory, not over `results/`.
4. Report a table: metric | published | committed result | this run | within bar? Plus the seeds,
   the commit, and any difference from the committed output with its likely cause.

Never overwrite `results/` or edit `RESULTS.md` unless the operator asks.
