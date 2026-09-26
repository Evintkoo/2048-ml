# Plan 01 — CLI Tools Specification: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Root CLI subcommands and help checked; corrected file-vs-directory collection example and required metadata flow.

**Goal:** State the current implementation and evidence boundary for cli tools specification.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Root CLI subcommands and help checked; corrected file-vs-directory collection example and required metadata flow.

> **Single binary with subcommands (MVP).** `Cargo.toml` is a single crate at root (see `03-Dependencies/01-rust-deps.md` §4); the 3-crate split (`game-engine` / `data-collector` / `benchmark`) is conceptual — commands below are **subcommands of one binary** (`cargo run -- <subcommand>`), not separate `[[bin]]` targets. Future optional: split into workspace members post-MVP. Verify via `cargo run -- --help`; `automl serve` is out-of-scope per initial-plan.

## 1. AutoML Framework CLI Usage

These commands refer to the `automl` framework's own binary, built from `automl/`; they do not describe the root `game2048-ml` executable. Verify each framework subcommand against `automl/README.md` and `cd automl && cargo run -- --help` before relying on it.

### 1.1 Training

```bash
# Train a model — canonical target is `action` (u8 0–3), TaskType::MultiClassification
automl train --data data.csv --target action --model gradient_boosting

# Train with configuration file (config must set target: action, task: MultiClassification)
automl train --config config/training/experiment-01.yaml

# Train with auto model selection — still predicts `action`
automl train --data data.csv --target action --model Auto
```

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

### 1.5 Server Mode — Out of Scope per Initial Plan

> **Out of scope per initial plan.** `automl serve` is not used for this project; verify via `cargo run -- --help` and do not rely on it. Use local CLI only.

```bash
# Start web server — OUT OF SCOPE, do not use
automl serve --port 8080  # out-of-scope per initial-plan
```

## 2. Custom CLI Tools (single binary, subcommands)

> **Workspace note:** These are subcommands of the single MVP binary, not separate crates. `cargo run -- <subcommand> --help` is the canonical invocation until a workspace split.

### 2.1 Game Simulation

```bash
cargo run -- game-engine simulate --seed 42 --n-games 10
```

### 2.2 Data Collection

```bash
cargo run -- data-collector collect --n-games 10000 --output data/raw/policy.csv
cargo run -- data-collector preprocess --input data/raw/policy.csv
cargo run -- data-collector validate --input data/raw/policy.csv
```

The implemented collector takes a CSV output file, not a directory. It writes a metadata sidecar and manifest next to the CSV. `preprocess` currently validates deterministic, already-encoded features; it does not accept an output argument.

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

`train` requires a row-aligned `--metadata` sidecar with game IDs. It reserves the final chronological 15% of distinct games by default (`--development-fraction 0.85`) from grouped CV and fitting. The AutoML fit still performs its own seeded stratified row split on development data. This distinction is documented in the training plan; the final test scoring/refit workflow remains pending.

## 3. CLI Configuration

The root CLI uses `clap` arguments. General training YAML loading is not implemented; the optional HyperOptX search settings can be supplied with `--hyperopt-config` using the schema-v1 JSON contract. `--tune-trials N` is the shorthand and conflicts with `--hyperopt-config`.

## 4. CLI Output Formats

| Format | Description | Use Case |
|--------|-------------|----------|
| JSON | Machine-readable | Pipeline integration |
| CSV | Tabular data | Analysis |
| Markdown | Human-readable | Reports |
| Plain | Default | Terminal output |

## 5. Error Handling

All CLI tools will return:
- Exit code 0 on success
- Exit code 1 on error
- Exit code 2 on configuration error
- Structured error messages to stderr

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

- **The plan-scale evidence remains bounded by current results.** Root CLI subcommands and help checked; corrected file-vs-directory collection example and required metadata flow. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
