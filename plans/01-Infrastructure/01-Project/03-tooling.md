# Plan 03 — Tooling Configuration: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Root crate commands checked; clarified no live CI workflow and root tests versus full submodule tests.

**Goal:** State the current implementation and evidence boundary for tooling configuration.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Root crate commands checked; clarified no live CI workflow and root tests versus full submodule tests.

> **MVP = 20 lines of cargo only.** All other tooling is Optional, not MVP (1-line note each). For CLI subcommands see `04-Tooling/01-cli-tools.md`; for environment setup see `04-Tooling/02-dev-environment.md`.

## 1. MVP Build & Quality (only these)

```bash
cargo build              # debug build
cargo build --release    # release
cargo test               # root tests (includes AutoML integration smoke; not full submodule suite)
cargo fmt -- --check     # format check
cargo clippy -- -D warnings  # lint
```

These are local checks. No CI workflow is configured yet; see `09-Quality/03-CI/01-ci-pipeline.md`. No Makefile is required for MVP (cargo suffices).

## 2. Project Binary (2048-specific)

Single binary with subcommands — not a multi-binary workspace (see `04-Tooling/01-cli-tools.md`):

```bash
cargo run -- --help                          # verify available subcommands
cargo run -- game-engine simulate --help     # headless random 2048 simulation
cargo run -- data-collector collect --help   # collect 27-dim + action rows
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

- **The plan-scale evidence remains bounded by current results.** Root crate commands checked; clarified no live CI workflow and root tests versus full submodule tests. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
