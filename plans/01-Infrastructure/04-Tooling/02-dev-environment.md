# Plan 02 — Development Environment Setup: the repository status is explicit and evidence based

> **Status: DONE (2026-09-27).** Rust 1.96.1, 24 GiB RAM, and 61 GiB free disk verified. Root-package formatting and all 35 root tests pass; pinned AutoML 712-test result is prior retained evidence.

**Goal:** State the current implementation and evidence boundary for development environment setup.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** On 2026-09-27, `rustc`/`cargo` 1.96.1, macOS arm64, 24 GiB memory, and 61 GiB free disk were observed. `cargo fmt --check --package game2048-ml` passed and `cargo test --quiet` passed 35/35. A whole-workspace `cargo fmt --all -- --check` reports pre-existing formatting differences inside `automl/`; it changed no files. The clean AutoML pin and its 712/712 library test result are retained from the earlier recorded run, not repeated here.

> **For CLI subcommands see `04-Tooling/01-cli-tools.md`.** This file is setup only.

## 1. Requirements

Rust **1.75+** (automl MSRV), 8 GB RAM, 20 GB disk. macOS 14+ or Ubuntu 22.04+.

## 2. Setup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version  # must be ≥1.75

git clone --recurse-submodules https://github.com/Evintkoo/2048-ml
cd 2048-ml
git submodule update --init --recursive
git submodule status automl  # pinned: 82d8483...

cargo build
cargo test                   # root integration and game tests
cargo run -- --help          # verify subcommands
(cd automl && cargo test --lib) # pinned framework library suite; run in the submodule
```

## 3. Submodule Health Check

```bash
git -C automl rev-parse HEAD              # verify the submodule checkout is present and pinned
cargo test --manifest-path automl/Cargo.toml --lib -- training::config
```

If `cargo: command not found` → `source "$HOME/.cargo/env"`.

> Docker, cross-compile, `bench`/`audit`, env-var tables — Optional, not MVP (see `01-Project/03-tooling.md` §3). Not needed for local headless development.

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/04-Tooling/02-dev-environment.md` exits 0.

## Open questions

- **Environment readings are machine-specific snapshots.** Recheck toolchain, memory, and disk before substantial experiments; retain a resource budget and outputs for large runs.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
