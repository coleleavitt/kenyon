# code style (kenyon)

- Toolchain `rustc-master`, edition 2024; plain `cargo`. `cargo fmt` before every commit.
- `#![forbid(unsafe_code)]`; `clippy::unwrap_used = "deny"` (allowed in tests via `clippy.toml`);
  `clippy -D warnings` clean.
- Errors: `KenyonError` (`thiserror`). Library code never panics on caller input; invalid parameters
  return `KenyonError::InvalidParameter` with the parameter name and the accepted range
  (dimension problems use `DimensionMismatch` / `CodeMismatch`).
- `#![warn(missing_docs)]`: every public item has rustdoc. Implementations of a paper cite it with
  the equation or page (e.g. "PNAS 2018 Eq. 2").
- Hot paths take a reusable `Scratch` rather than allocating per call; keep `hash_with` /
  `observe`-style variants for that.
- Ablations and controls are types or enum variants (`TagRule`, `Coding`, `LearningRule`,
  `DenseProjection`), never boolean flags threaded through functions.
- `kenyon` keeps one runtime dependency (`thiserror`). Experiment-only crates go in `repro/`; a new
  feature with heavier needs gets its own workspace crate (FlyRAG grew into its own repo).
- Keep modules small and named for what they do; no god objects.
