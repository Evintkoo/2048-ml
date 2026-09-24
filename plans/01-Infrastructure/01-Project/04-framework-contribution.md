# Plan 04 — Rust-Native AutoML Framework Contribution: the repository status is explicit and evidence based

> **Status: PARTIAL.** A source-backed architecture and contracts audit is recorded; standard-dataset results, matched comparisons, resource measurements, and independent replication remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework contribution.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The source architecture, data contracts, and current API limitations are now recorded in [04-framework-architecture.md](04-framework-architecture.md), based on root revision `ca15cebcdf4d307126a63d4ab19e416606f50564` and the pinned AutoML revision `64f5edad29c9e58ee7d33abf380418d5cfbbb561`. Standard-dataset/resource evidence and matched framework comparisons remain pending.

### Architecture audit result

The audit found concrete training, preprocessing, optimization, CV, inference, and JSON persistence APIs. It also confirmed that the pieces are not one automatically composed pipeline: root integration loads the CSV into a Polars `DataFrame`, provides game-group split/CV orchestration, and invokes `TrainEngine`; root does not currently use `DataPreprocessor` or `InferenceEngine`, and the `HyperOptX` objective callback is not wired into the training command. `TrainEngine::fit` uses its own seeded row split and does not call cross-validation or consume `cv_folds`. The root therefore performs grouped CV in a wrapper and then separately fits the final model. Seed controls are component-level, and explicit artifact schema migration was not found. Details and source paths are in the architecture record.

## 1. Primary Contribution

The primary research contribution is the design, implementation, and empirical validation of a Rust-native AutoML architecture. The 2048 ML system is the principal implementation and case study used to exercise the framework in a nontrivial stochastic policy-learning workflow.

The thesis must distinguish framework novelty from application novelty. Reimplementing known algorithms in Rust is an engineering contribution unless the architecture, optimization procedure, systems integration, reproducibility mechanism, or empirical trade-off is shown to provide a defensible improvement or new insight.

## 2. Architecture Questions

1. How are data ingestion, preprocessing, model training, validation, optimization, inference, and persistence composed?
2. Which Rust-native data contracts make the pipeline type-safe and reproducible?
3. How are model types and task types exposed consistently through the library and CLI?
4. How does the architecture manage seeds, parallelism, resource budgets, and failure recovery?
5. What trade-offs exist between Rust-native execution, ecosystem coverage, implementation complexity, and interoperability?

## 3. Required Architecture Evidence

- Module and dependency architecture diagram.
- Data-flow and ownership contracts.
- Configuration schema and versioning rules.
- Model and task capability matrix.
- Error-handling and failure-recovery policy.
- Seed and determinism design.
- Serialization and backward-compatibility policy.
- Resource-budget and parallel-execution policy.
- CLI/library/API equivalence tests.

## 4. Evaluation Dimensions

The framework is evaluated independently of 2048 using standard tabular tasks:

- Predictive quality.
- Training and inference time.
- Memory consumption.
- Hyperparameter-search efficiency.
- Reproducibility across repeated runs.
- Failure rate and diagnostics.
- Artifact portability and model reload correctness.
- Ease of configuration and experiment automation.

## 5. Baseline Categories

Use appropriately matched comparisons with:

- A simple fixed-configuration implementation.
- Established Python AutoML or tabular ML tooling.
- Established Rust ML libraries where comparable functionality exists.
- A manual model-selection baseline.

Comparisons must use documented datasets, splits, hardware, dependency versions, and search budgets. The 2048 score is not a substitute for framework evaluation.

## 6. Case-Study Role of 2048

2048 tests whether the architecture remains usable when the framework is integrated with:

- A custom stochastic environment.
- Sequential trajectory data.
- Rollout-generated labels.
- Four-class action prediction.
- Repeated simulation and policy evaluation.

The case study provides application evidence and exposes framework limitations; it does not alone establish general AutoML superiority.

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.

### Ticket-specific completion checklist

- [x] Module/data-flow map and source-backed data/ownership contracts recorded.
- [x] Configuration, capability, seed, parallelism, failure, persistence, and API-surface limitations audited.
- [ ] Named standard tabular datasets with retained versions, hashes, splits, and metrics evaluated.
- [ ] Fixed-configuration, established-framework, manual-selection, and comparable Rust baselines run under matched budgets.
- [ ] Runtime, memory, repeated-seed reproducibility, reload, and CLI/library equivalence evidence collected.
- [ ] Independent replication or validation completed.

The existing focused root capability test was run on 2026-09-24 with `cargo test framework_validation -- --nocapture`. The grouped-CV/TPE smoke passed. The model capability/persistence smoke failed once and passed on retry; in 20 repetitions it failed once because RandomForest predictions after save/load differed from the pre-save predictions. Source inspection found a plausible nondeterminism in decision-tree leaf ties: `HashMap::into_iter().max_by_key` has no stable tie order. This is not yet isolated experimentally, so model reproducibility and reload equivalence remain unverified. No test was added or changed as part of this ticket.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Source-backed architecture evidence now exists, but no standard-dataset, resource, matched-baseline, or replication results exist. Starting those experiments requires a declared compute budget and named dataset acquisition/licensing decisions; retain configurations, seeds, dependency versions, raw metrics, and analysis artifacts.

## Later

- **Run the framework-validation program in `plans/07-Benchmarking/03-Comparison/04-framework-validation.md`.** Coordinate the named dataset matrix, baseline selection, and resource budget there; return here to update contribution conclusions only after that evidence is available.
