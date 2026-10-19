# testing (kenyon)

- Gate: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `timeout 1800 cargo test --workspace`. Wrap every test run in `timeout`.
- **Golden tests** (`tests/golden.rs`) pin seeded outputs. A change that moves a golden value is a
  determinism break: fix the code, never the golden (unless the operator explicitly approves a
  versioned break).
- **Equivalence tests** for optimisations: a fast path is tested against the paper's literal update
  (e.g. lazy Bloom recovery vs the eager Eq. 2 update).
- **Property tests** for codes and metrics: overlap/Hamming symmetry, `KcCode` round-trips to
  `BitCode`, k-WTA returns exactly k units, invalid parameters return errors and do not panic.
- **Experiments are not unit tests.** Paper reproductions run in `repro/` with seeds and write to
  `results/`; their bars are predeclared in committed files (`kenyon-experiment` skill). A unit test
  never asserts a published number.
- Every new feature ships its controls as tests where they are cheap (e.g. FlyRAG: a frozen learner
  changes nothing; shuffled outcomes do not reproduce the learning effect).
- Never `#[ignore]` a failing test, weaken an assertion, or `#[allow]` a lint to get green.
