# Plan 02 — Development Environment Setup: the repository status is explicit and evidence based

> **Status: DONE (2026-09-26).** Rust 1.96.1, 24 GiB RAM, and 62 GiB free disk verified; root format, 35 tests, Clippy, and pinned AutoML 712 library tests pass (2026-09-27).

**Goal:** State the current implementation and evidence boundary for development environment setup.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Rust 1.96.1 present; root formatting/tests/clippy and pinned AutoML library tests pass; submodule check no longer assumes .git directory.

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

- **The plan-scale evidence remains bounded by current results.** Rust 1.96.1 present; root formatting/tests/clippy and pinned AutoML library tests pass; submodule check no longer assumes .git directory. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
