# Plan 03 — Tooling Configuration: the repository status is explicit and evidence based

> **Status: DONE (2026-09-27).** Root crate commands and CI workflow checked; root tests remain distinct from the full AutoML submodule suite.

**Goal:** State the current implementation and evidence boundary for tooling configuration.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** Root build/test/format/lint commands and CLI help are documented. `.github/workflows/ci.yml` runs on pushes to `main`, pull requests, and manual dispatch; it initializes the AutoML submodule and checks format, root tests, and Clippy. The workflow does not run research benchmarks or claim a coverage threshold. Root crate validation is distinct from the full AutoML submodule test suite.

> **MVP = 20 lines of cargo only.** All other tooling is Optional, not MVP (1-line note each). For CLI subcommands see `04-Tooling/01-cli-tools.md`; for environment setup see `04-Tooling/02-dev-environment.md`.

## 1. MVP Build & Quality (only these)

```bash
cargo build              # debug build
cargo build --release    # release
cargo test               # root tests (includes AutoML integration smoke; not full submodule suite)
cargo fmt -- --check     # format check
cargo clippy -- -D warnings  # lint
```

These commands are available for local use. The configured CI workflow runs format, root tests, and Clippy; it does not run release builds or research benchmarks. See `09-Quality/03-CI/01-ci-pipeline.md`. No Makefile is required for MVP (cargo suffices).

## 2. Project Binary (2048-specific)

Single binary with subcommands — not a multi-binary workspace (see `04-Tooling/01-cli-tools.md`):

```bash
cargo run -- --help                          # verify available subcommands
cargo run -- game-engine simulate --help     # headless random 2048 simulation
cargo run -- data-collector collect --help   # collect 17-value state + action rows
cargo run -- benchmark run --help            # mean-score benchmark (10k+ games)
```

## 3. Optional, not MVP — Keep Minimal

| Tool / Feature | 1-line Note |
|----------------|-------------|
| `cargo bench` / `criterion` | Optional, not MVP — no benchmark harness configured |
| `cargo audit` | Optional, not MVP — run manually; no schedule configured |
| `cargo flamegraph` / `perf` | Optional, not MVP — profiling future |
| Cross-compile (`--target aarch64-*`) | Optional, not MVP — local x86_64/arm64 dev only |
| Docker (`docker build`/`run`) | Optional, not MVP — not needed for local headless simulation |
| Frontend `automl serve` | Out-of-scope per initial-plan — mark Optional, not MVP |

> Out-of-scope per initial-plan (frontend serve, game UI, mobile) stays Optional, not MVP with minimal mention. No expanded sections for them.

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/01-Project/03-tooling.md` exits 0.

## Open questions

- **The evidence remains bounded by current results.** The configured CI checks code quality and root tests only; they do not validate research benchmarks. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
