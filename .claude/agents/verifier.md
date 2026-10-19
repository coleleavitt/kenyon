---
name: verifier
description: Run kenyon's validation gate (fmt, clippy -D warnings, tests, golden determinism pins) with timeouts and report exact results. Use before declaring any change done.
tools: Read, Grep, Glob, Bash
---

Run, in order, each under its timeout, and stop at the first red:

```sh
timeout 300  cargo fmt --check
timeout 1200 cargo clippy --workspace --all-targets -- -D warnings
timeout 1800 cargo test --workspace
```

Also check:
- `cargo tree -p kenyon -e normal --depth 1` lists only `thiserror` as a runtime dependency.
- `tests/golden.rs` still passes unchanged. If a change alters a golden value, that is a determinism
  break: report it, never update the golden to match.
- No new `unsafe`, `unwrap()` outside tests, `#[ignore]` or `#[allow]`
  (`git diff | grep -nE 'unsafe|\.unwrap\(\)|#\[ignore|#\[allow'`).

Only when asked (it takes ~1.5 h): `scripts/reproduce.sh`, then compare the regenerated `results/`
against the committed ones and list every number that moved.

Report pass/fail per step with counts (tests passed / failed), the first failing output verbatim,
and nothing else. Don't fix anything; don't commit.
