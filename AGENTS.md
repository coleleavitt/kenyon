# kenyon — agent guide

`CLAUDE.md` is the primary instruction file; this mirrors its core rules for agents that do not read
it. If the two ever disagree, `CLAUDE.md` wins.

**What this is:** a safe, deterministic Rust crate of fruit-fly mushroom-body algorithms (FlyHash,
DenseFly, fly Bloom filter, FlyModel), a reproduction harness (`repro/`, unpublished) and a
preregistered FlyWire wiring study (`research/wiring/`). FlyRAG, built on kenyon, lives in its
own repo (github.com/coleleavitt/flyrag).

**Rules that keep the project honest and safe:**
- No `unsafe`; no `unwrap` outside tests; errors via `KenyonError`; document every public item.
- `kenyon` keeps exactly one runtime dependency (`thiserror`). No `rand`: randomness comes from the
  crate's seeded generator, and determinism is pinned by `tests/golden.rs`.
- Predeclare every experiment's bar and controls in a committed file before running it. Report a
  missed bar as missed.
- Never `#[ignore]` a failing test, weaken an assertion or `#[allow]` a lint to get green.
- Datasets stay outside the repo (`~/datasets`, `~/flywire-data`); never load pickle files.
- Toolchain `rustc-master`; plain `cargo`.
- Don't commit or push unless the operator asks; there is no remote and `publish = false`.

**Gate:**
```sh
timeout 300  cargo fmt --check
timeout 1200 cargo clippy --workspace --all-targets -- -D warnings
timeout 1800 cargo test --workspace
```
