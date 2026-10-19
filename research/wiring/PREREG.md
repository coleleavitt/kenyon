# Preregistration: does the real fly PN→KC wiring hash better than random?

Written and committed **before** any wiring run. `PAPER.md` reports against this file and lists
every departure from it.

## Data and filters

- Source: FlyWire FAFB v783 (Dorkenwald et al. 2024; Schlegel et al. 2024; CC-BY 4.0). Local copy
  at `~/flywire-data/fafb/783/`.
  - `connections.csv.gz`: one row per (pre, post, neuropil), already thresholded at ≥ 5
    synapses by FlyWire.
  - `classification.csv.gz`: classes.
  - `consolidated_cell_types.csv.gz`: PN types, which give the glomerulus.
- PN set: `class == "ALPN"`, all sub-classes (uni- and multiglomerular), both sides. We keep a PN
  only if it has at least one edge to a KC.
- KC set: `class == "Kenyon_Cell"`. We keep a KC only if it has at least one PN input.
- Edge weight: synapse count summed over neuropils for each (PN, KC) pair. No extra threshold.
- Expected (measured during planning, doc 31 §1):
  - 21,509 pairs, 285 PNs → 4,723 KCs;
  - KC fan-in median 5 (p10 2, p90 6);
  - PN out-degree mean 75.5, median 60, max 516.
- Primary analysis: both hemispheres pooled (285 × 4,723), as in doc 31. Secondary: the right
  hemisphere alone (right PNs → right KCs).

## Arms

Twenty seeds (0–19) per arm. "Binary" = every edge has weight 1. "Weighted" = synapse counts.

| Arm | Topology | Weights |
|---|---|---|
| R-b / R-w | real | 1 / real counts |
| U-b / U-w | each KC keeps its in-degree; its PNs are drawn uniformly without replacement from the 285 | 1 / real counts randomly permuted onto the edges |
| D-b / D-w | degree-preserving rewire: double-edge swaps on the bipartite graph keep every PN out-degree and KC in-degree, with no multi-edges; 10 × E accepted swaps | 1 / real counts randomly permuted |
| S-w | real topology | real counts randomly permuted across edges (weight shuffle) |

R-b and R-w are deterministic. For them the seed varies only the task (data subset, input
mapping, queries).

## Predeclared contrasts

Each contrast is a paired difference per seed, with the same task draw for both arms.

- **P1 (wiring beyond degrees):** R-b − D-b. This is the main question.
- **P2 (degree heterogeneity):** D-b − U-b.
- **P3 (weights):** R-w − S-w.
- Also reported: R-b − U-b, and R-w − D-w.

## Tasks

### T1: generic similarity search (MNIST → PN channels)

- **Inputs:**
  - Each seed draws 10,000 MNIST test images in random order.
  - Images are mapped to the 285 PN channels by a seeded Gaussian matrix G (285 × 784, entries
    N(0, 1/784)): x_PN = G·x.
  - Then each vector's own mean is subtracted.
- **Why this mapping:** MNIST pixels have no correspondence to glomeruli. A random isotropic
  map makes the PN channels exchangeable, so any difference between arms comes from the wiring
  and not from an arbitrary pixel-to-glomerulus assignment.
- **Neighbours:**
  - Ground truth: Euclidean in PN space, top 2% (200), excluding the query.
  - 1,000 queries per seed.
- **Hash:**
  - Primary: KC activations → WTA with k = 5% of KCs (236). The tag is binary and neighbours
    are ranked by overlap.
  - Secondary: k = 16, valued tag, Euclidean ranking.
- **Metric:** mAP (AP@200, as in RESULTS.md R1).

### T2: odour similarity (Hallem & Carlson 2006 via DoOR 2.0)

- **Inputs:**
  - 110 odours × 24 receptors. The value is the response (`Hallem.2006.EN`) plus the receptor's
    spontaneous rate (the `SFR` row), i.e. an absolute firing rate.
  - Each receptor maps to a glomerulus. Each uniglomerular PN of that glomerulus (both sides)
    receives the glomerulus's rate. PNs of other glomeruli, and multiglomerular PNs, receive 0.
  - DM3 and DM5 both express Or33b. DM3 = mean(Or47a, Or33b) and DM5 = mean(Or85a, Or33b).
  - The OR→glomerulus table (INFERRED from Couto et al. 2005 and Fishilevich & Vosshall 2005,
    outside the corpus):
    Or2a DA4m, Or7a DL5, Or9a VM3, Or10a DL1, Or19a DC1, Or22a DM2, Or23a DA3, Or35a VC3,
    Or43a DA4l, Or43b VM2, Or47a DM3, Or47b VA1v, Or49b VA5, Or59b DM4, Or65a DL3, Or67a DM6,
    Or67c VC4, Or82a VA6, Or85a DM5, Or85b VM5d, Or85f DL4, Or88a VA1d, Or98a VM5v, Or33b
    DM3+DM5.
  - Planning count: 23 glomeruli and 103 of the 285 PNs receive input.
- **Pre-processing:** subtract the mean over PN channels.
- **Ground truth:** the top 5 neighbours by Euclidean distance in the 24-d receptor space,
  excluding the query. All 110 odours are queries.
- **Hash:** the same two settings as T1. Metric: AP@5.
- **Seeds:** T2 has no task randomness, so R-b and R-w give one value each. The contrasts are
  real minus each null seed.

### T3: novelty (fly Bloom filter)

- **Data:**
  - T1's inputs, 200 random images per seed;
  - T2's odours (110).
- **Protocol:** 10-fold cross-validation.
  - Insert the training fold with δ = ε = 0.
  - Predicted novelty = the mean weight of the item's winners. Ground truth = min Euclidean
    distance to the training fold (in PN space for T1, receptor space for T2).
  - Score = Pearson r, averaged over folds.
- **Winners:** k = 20 (biological k = 236 saturates a 4,723-cell filter after ~20 inserts). Also
  reported: k ∈ {5, 10, 40}.

## Statistics and decision rule

- For each contrast and task, report:
  - the mean paired difference;
  - its 95% CI (t-interval over the 20 seeds; also a percentile bootstrap with 10,000
    resamples);
  - Cohen's d_z (mean difference / SD of the differences);
  - for T2, where real is a single value, the real arm's rank among the 20 null values.
- An effect is **detected** if the 95% CI excludes 0. It is **practically relevant** if
  |mean difference| also exceeds the SD of the null arm across seeds (doc 31, F1: "an effect
  larger than the null's seed spread").
- No correction is applied for the 5 contrasts × 3 tasks × 2 hash settings. The primary
  confirmatory test is P1 on T1 with the primary hash. Everything else is exploratory and is
  labelled so.

## Hypotheses (from doc 31, F1; INFERRED, not evidence)

- **H1:** P1 ≈ 0 (real ≈ degree-preserving), as in Dhiman 2026 and conn2res (Suárez 2024).
- **H2:** on generic inputs (T1), U-b ≥ R-b. The skewed PN out-degree over-weights a few
  channels.
- **H3:** on odours (T2), R/D > U. Over-represented PNs may match ecologically relevant
  glomeruli.

A null result on any of these is reported plainly.

## Amendment A2 (2026-10-19, before any MCNS run): replication on the male CNS

Purpose: test whether the confirmatory result replicates in a second, independently reconstructed
brain. Nothing in the FAFB analysis changes.

- **Data:** male CNS v1.0 (MCNS) Codex export at `~/flywire-data/mcns/1.0/`: `neurons.csv.gz`
  (quoted CSV; columns `Class`, `Soma side`, `Primary Cell Type`) and
  `connections_princeton.csv.gz`.
- **Sets:** PNs = `Class == "ALPN"`, KCs = `Class == "Kenyon_Cell"`, each kept only if it has a
  PN→KC edge after filtering, as for FAFB.
- **Synapse filter:** unlike the FAFB export, the MCNS file includes rows with fewer than 5
  synapses. To match FAFB, keep only rows (pre, post, neuropil) with `syn_count ≥ 5`, then sum
  per (PN, KC) pair.
- **Uniglomerular PNs** (used by T2's odour mapping): MCNS has no uni/multiglomerular sub-class.
  A PN is uniglomerular iff its `Primary Cell Type` does not start with `M_` and the part before
  the first `_` contains no `+` (e.g. `DA1_lPN` → DA1; `M_vPNml53` and `VP1m+VP2_lvPN2` are
  excluded). Glomerulus = that part.
- **Arms, tasks, seeds, statistics:** identical to FAFB (same code, `wiring 20` with
  `KENYON_CONNECTOME=mcns`). Outputs `results/wiring_mcns_{pooled,right}.json`.
- **Replication criterion** for the confirmatory contrast (P1, T1, primary hash, pooled), fixed
  now: the FAFB claim "the real wiring is not a better hash than its degree-preserving null"
  **replicates** if MCNS's 95% t-CI upper bound is ≤ +0.005; it **fails** if the lower bound is
  > 0 (real detectably better); otherwise **inconclusive**. Whether MCNS also shows FAFB's
  small *negative* effect is reported as exploratory.
