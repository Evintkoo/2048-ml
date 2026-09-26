# 2048-ML

Research project for integrating and evaluating a Rust-native AutoML framework through supervised policy learning for the 4×4 game 2048.

The repository is organized around two connected pieces of work:

- **AutoML framework:** the [`automl`](automl) Git submodule contains the Rust framework being developed and validated.
- **2048 case study:** the root Rust crate implements the game environment, rollout-labeled data pipeline, AutoML training integration, and evaluation tooling.

The framework is the primary research contribution. Results from 2048 are application evidence and will be reported separately from framework validation. See [plans/00-scope-and-traceability.md](plans/00-scope-and-traceability.md) for the canonical scope.

## Repository status

The root CLI supports rollout collection, grouped validation, AutoML policy fitting, and seeded game evaluation. Framework diagnostics cover three UCI datasets and five compatible models on split seeds 42, 2026, and 2027. Repeated processes at seeds 42 and 2026 matched all 15 prediction sets and save/load outputs; these diagnostics do not establish matched-budget framework superiority. A 20-game rollout-labeling pilot (2,447 rows, 857,100 rollout evaluations in 857.36 seconds) and small chronological classifier diagnostic are retained. Linear projection to 20,000 collection games is 238.16 hours on that pilot's observed rate, with substantial uncertainty. One fitted policy was evaluated on 10,000 same-seed games alongside random and heuristic agents; that exploratory comparison is not a selected-model study, and its interval/effect-size calculations do not model the matched seeds. Larger policy studies and matched framework resources remain pending. Game scores do not establish general AutoML superiority.

## Getting started

Initialize or refresh the framework submodule and build the project:

```bash
cargo build
cargo run -- --help
```

Rust 1.75 or newer is required. Use the plan ticket tree as the implementation backlog; every Markdown plan file is treated as one ticket. See [AGENTS.md](AGENTS.md) for the working protocol.

### Collect, split, train, and evaluate

```bash
# Small collection example. At the planned 100 rollouts per valid action this is compute intensive.
cargo run --release -- data-collector collect --n-games 10 --rollouts 100 --threads 4 --seed 42 --output data/raw/policy.csv
cargo run --release -- train --data data/raw/policy.csv --metadata data/raw/policy.metadata.csv --development-fraction 0.85 --model random_forest --cv-folds 5 --seed 42 --output models/policy.json
# Optional versioned TPE search settings (searches grouped-CV accuracy on development games)
cargo run --release -- train --data data/raw/policy.csv --metadata data/raw/policy.metadata.csv --model random_forest --cv-folds 5 --hyperopt-config config/hyperopt-search.example.json --seed 42 --output models/policy-tuned.json
cargo run --release -- benchmark run --model models/policy.json --n-games 10000 --seed 9999 --output results/policy.csv
cargo run --release -- benchmark baseline --agent random --n-games 10000 --seed 9999 --output results/random.csv
cargo run --release -- benchmark baseline --agent heuristic --n-games 10000 --seed 9999 --output results/heuristic.csv
cargo run --release -- benchmark report results/policy.csv results/random.csv results/heuristic.csv --output results/report.csv
cargo run --release -- benchmark compare results/policy.csv results/random.csv results/heuristic.csv --output results/comparison.csv
```

Every benchmark command writes a JSON manifest beside its CSV output. The collection manifest records seeds, row counts, rollout budget, elapsed time, and schema. A two-game 100-rollout pilot measured 5.72 rows/second with two threads; do not launch canonical 20,000-game collection without planning for the measured multi-day runtime. The integration currently accepts the verified classifier set: `random_forest`, `extra_trees`, `adaboost`, `knn`, and `naive_bayes`.

The framework's own [README](automl/README.md) documents framework-specific commands. The root [agent guide](AGENTS.md) describes how to proceed through the plans.

The optional HyperOptX search input is a versioned JSON contract; the tracked example is [config/hyperopt-search.example.json](config/hyperopt-search.example.json). It controls trial count and the supported RandomForest/ExtraTrees parameter ranges. Training manifests record the selected parameters, input configuration digest, data digests, seeds, study artifact, and that pruning is unavailable in the pinned optimizer API.

### Local quality checks

Run the same Rust checks used by GitHub Actions:

```bash
./scripts/ci-check.sh
```

The workflow runs on pushes to `main`, pull requests, and manual dispatch. It initializes the AutoML submodule and checks formatting, the root Cargo test suite, and Clippy. It does not run research benchmarks or claim a coverage threshold.

## Research principles

- Train the 2048 policies through the AutoML framework; keep external ML libraries to explicitly described comparison baselines.
- Record seeds, data, configurations, dependency versions, and analysis artifacts for reported results.
- Treat framework validation and 2048 application evaluation as separate evidence.
- Report limitations and inconclusive results alongside positive results.

## Project documents

- [Canonical scope and traceability](plans/00-scope-and-traceability.md)
- [AutoML framework](https://github.com/Evintkoo/automl) (checked out at `automl/`)
