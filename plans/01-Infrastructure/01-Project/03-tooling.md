# Tooling Configuration — MVP

> **MVP = 20 lines of cargo only.** All other tooling is Optional, not MVP (1-line note each). For CLI subcommands see `04-Tooling/01-cli-tools.md`; for environment setup see `04-Tooling/02-dev-environment.md`.

## 1. MVP Build & Quality (only these)

```bash
cargo build              # debug build
cargo build --release    # release
cargo test               # all tests (includes automl smoke)
cargo fmt -- --check     # format check
cargo clippy -- -D warnings  # lint
```

CI runs exactly these four. No Makefile required for MVP (cargo suffices).

## 2. Project Binary (2048-specific)

Single binary with subcommands — not a multi-binary workspace (see `04-Tooling/01-cli-tools.md`):

```bash
cargo run -- --help                          # verify available subcommands
cargo run -- game-engine simulate --help     # headless 2048 simulation
cargo run -- data-collector collect --help   # collect 27-dim + action rows
cargo run -- benchmark run --help            # mean-score benchmark (10k+ games)
```

## 3. Optional, not MVP — Keep Minimal

| Tool / Feature | 1-line Note |
|----------------|-------------|
| `cargo bench` / `criterion` | Optional, not MVP — scheduled only; heuristic ~512 baseline is code, not a bench harness |
| `cargo audit` | Optional, not MVP — weekly scheduled |
| `cargo flamegraph` / `perf` | Optional, not MVP — profiling future |
| Cross-compile (`--target aarch64-*`) | Optional, not MVP — local x86_64/arm64 dev only |
| Docker (`docker build`/`run`) | Optional, not MVP — not needed for local headless simulation |
| Frontend `automl serve` | Out-of-scope per initial-plan — mark Optional, not MVP |

> Out-of-scope per initial-plan (frontend serve, game UI, mobile) stays Optional, not MVP with minimal mention. No expanded sections for them.
