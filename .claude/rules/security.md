# security (kenyon)

- No secrets anywhere in the repo or in results. Nothing in kenyon needs credentials.
- Datasets are downloaded only by `scripts/download_datasets.sh` and checked against
  `scripts/datasets.sha256`. Add a dataset only with its source URL, licence and sha256.
- Datasets and connectome data stay outside the repo (`~/datasets`, `~/flywire-data`). Never commit
  them, and never load Python pickle files (FlyWire publishes `*.pickle.gz`; use the CSVs).
- Respect data licences: FlyWire FAFB v783 is CC-BY 4.0 (cite Dorkenwald et al. 2024 and Schlegel
  et al. 2024); DoOR 2.0 is CC BY-SA 4.0. Results that redistribute derived data say so.
- Supply chain: adding any dependency to `kenyon` needs the operator's approval; `repro/`
  dependencies come from crates.io with the lockfile committed. No git dependencies.
- No destructive commands outside the repo. `scripts/reproduce.sh` writes only into `results/` and
  `target/`.
- `publish = false` stays until the operator decides; never run `cargo publish`. Push only to
  `origin` (github.com/coleleavitt/kenyon, public since 2026-10-19) when the operator asks.
- The repo is public: everything committed is world-readable. FlyRAG fetches kenyon over https.
