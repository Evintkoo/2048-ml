# Plan 01 — CLI Tools Specification: the repository status is explicit and evidence based

> **Status: DONE (2026-09-27).** Root and AutoML command help surfaces verified. Root training still requires a game-metadata sidecar and uses a separate versioned JSON tuning configuration.

**Goal:** State the current implementation and evidence boundary for cli tools specification.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** On 2026-09-27, root top-level and game-engine, data-collector, benchmark, and train help all listed the documented commands/options. AutoML top-level help listed train, predict, preprocess, benchmark, info, and serve. General training YAML remains unimplemented; root supports JSON search config. The metadata sidecar remains required by the training workflow.

> **Single binary with subcommands (MVP).** `Cargo.toml` is a single crate at root (see `03-Dependencies/01-rust-deps.md` §4); the 3-crate split (`game-engine` / `data-collector` / `benchmark`) is conceptual — commands below are **subcommands of one binary** (`cargo run -- <subcommand>`), not separate `[[bin]]` targets. Future optional: split into workspace members post-MVP. Verify via `cargo run -- --help`; `automl serve` is out-of-scope per initial-plan.

## 1. AutoML Framework CLI Usage

These commands refer to the `automl` framework's own binary, built from `automl/`; they do not describe the root `game2048-ml` executable. Verify each framework subcommand against `automl/README.md` and `cd automl && cargo run -- --help` before relying on it.

### 1.1 Training

```bash
# Train a model — canonical target is `action` (u8 0–3), TaskType::MultiClassification
automl train --data data.csv --target action --model random_forest
```

The framework CLI help lists `linear`, `logistic`, `decision_tree`, and `random_forest`; it does not advertise `gradient_boosting` or `Auto`. It has no `--config` option. The core 2048 root training CLI has its own five verified four-class candidates and uses CLI flags plus a dedicated HyperOptX JSON file, not the framework CLI examples above.

### 1.2 Prediction

```bash
# Make predictions
automl predict --model model.bin --data test_data.csv --output predictions.csv
```

### 1.3 Benchmarking — Verify via `cargo run -- --help`

> **Note:** `automl benchmark` subcommand unchecked — verify via `cargo run -- --help` before assuming availability; fallback is local `benchmark` binary if automl CLI lacks it.

```bash
# Benchmark multiple models — benchmark target is still `action` (classification);
# game score is measured separately by running the policy in the simulator
automl benchmark --data data.csv --target action  # verify: cargo run -- --help
```

### 1.4 Information

```bash
# Show data info
automl info --data data.csv
```

### 1.5 Server Mode — Available but Out of Scope

> **Out of scope per initial plan.** The AutoML binary exposes `serve`, but this project does not use the web server; its presence is not part of the root workflow.

```bash
# Start web server — OUT OF SCOPE, do not use
automl serve --port 8080  # available in AutoML; not used by this project
```

## 2. Custom CLI Tools (single binary, subcommands)

> **Workspace note:** These are subcommands of the single MVP binary, not separate crates. `cargo run -- <subcommand> --help` is the canonical invocation until a workspace split.

### 2.1 Game Simulation

```bash
cargo run -- game-engine simulate --seed 42 --n-games 10
```

### 2.2 Data Collection

```bash
cargo run -- data-collector collect --n-games 2 --rollouts 2 --output data/raw/policy.csv
cargo run -- data-collector preprocess --input data/raw/policy.csv
cargo run -- data-collector validate --input data/raw/policy.csv
```

The implemented collector takes a CSV output file, not a directory. It writes a metadata sidecar and manifest next to the CSV. The two-game command is a smoke example; larger runs require a declared budget. `preprocess` currently validates deterministic, already-encoded features; it does not accept an output argument.

```bash
cargo run -- data-collector collect --n-games 20 --rollouts 100 --output data/raw/random_play.csv
cargo run -- data-collector validate --input data/raw/random_play.csv
cargo run -- data-collector split --input data/raw/random_play.csv --metadata data/raw/random_play.metadata.csv --output-dir data/processed/splits
cargo run -- train --data data/raw/random_play.csv --metadata data/raw/random_play.metadata.csv --model random_forest --cv-folds 5 --seed 42 --output models/policy.json
# Optional: supply the versioned HyperOptX search JSON instead of --tune-trials N.
cargo run -- train --data data/raw/random_play.csv --metadata data/raw/random_play.metadata.csv --model random_forest --cv-folds 5 --hyperopt-config config/hyperopt-search.example.json --seed 42 --output models/policy-tuned.json
```

### 2.3 Benchmark

```bash
cargo run -- benchmark run --model models/policy.json --n-games 10000 --seed 9999 --output results/policy.csv
cargo run -- benchmark baseline --agent random --n-games 10000 --seed 9999 --output results/random.csv
cargo run -- benchmark compare results/policy.csv results/random.csv --output results/comparison.csv
cargo run -- benchmark report results/policy.csv results/random.csv --output results/report.csv
```

`train` requires a row-aligned `--metadata` sidecar with game IDs. It reserves the final chronological 15% of distinct games by default (`--development-fraction 0.85`) from grouped CV and fitting. The AutoML fit still performs its own seeded stratified row split on development data. This distinction is documented in the training plan; the final test scoring/refit workflow remains pending. Ticket #034 aligns the root command with Plan 00's 17-value input schema.

## 3. CLI Configuration

The root CLI uses `clap` arguments. General training YAML loading is not implemented; the optional HyperOptX search settings can be supplied with `--hyperopt-config` using the schema-v1 JSON contract. `--tune-trials N` is the shorthand and conflicts with `--hyperopt-config`.

## 4. Observed Output Artifacts

| Format | Description | Use Case |
|--------|-------------|----------|
| CSV | Collected policy rows, game scores, comparisons, and reports | Data and analysis |
| JSON | Sidecar manifests for collection, training, and benchmark outputs | Provenance |
| Plain text | Clap help and command progress summaries | Terminal use |

## 5. Error Handling

The current root CLI uses exit code 2 for several explicit argument/schema validation errors, while many workflow failures use `expect` and panic. Do not assume a uniform exit-code or structured-stderr contract. The AutoML binary returns `anyhow::Result` from its top-level command runner and logs command errors; its runtime behavior is separate from the root CLI.

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/04-Tooling/01-cli-tools.md` exits 0.

## Open questions

- **The evidence covers command/help wiring only.** Successful help output does not validate every command's runtime behavior or establish training quality. Training and benchmark executions require their own data, budgets, and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
