# Plan 04 — AutoML architecture audit: observed module boundaries and API limitations are explicit

> **Status: COMPLETE (2026-09-26).** The source-backed architecture audit was verified as a standalone supporting record for ticket #005.

**Goal:** Record the framework architecture and integration contracts verified in source.
**Builds on:** [04-framework-contribution](04-framework-contribution.md) — architecture evidence supports this ticket's implementation record.

## Decision and evidence

This document records architecture observed in the pinned AutoML submodule and root integration. It is a source map, not a framework performance result. Its architecture audit is complete; empirical framework claims remain outside this ticket and are tracked in #005/#102.

## Revision and scope

- Root revision: `ca15cebcdf4d307126a63d4ab19e416606f50564`.
- AutoML submodule revision at the original audit: `64f5edad29c9e58ee7d33abf380418d5cfbbb561`.
- Current AutoML pin: `88a86bf44a0cb03664931f7ef15201b95fa11255` (`feature/deterministic-training-serialization`). It adds deterministic leaf ties, stable class-group iteration during seeded splitting, and exact float-roundtrip deserialization; the architecture boundaries above are unchanged.
- Environment inspected: macOS 26.5, arm64; Rust/Cargo 1.96.1.
- Root crate directly depends on the path submodule `automl`; root training data is read as Polars `DataFrame` through the framework CLI loader.

## Module and data flow

```mermaid
flowchart LR
    CSV[Canonical CSV<br/>17 state values + action] --> LOAD[AutoML CSV loader<br/>Polars DataFrame]
    META[Game metadata sidecar<br/>game_id, move_index, score] --> SPLIT[Root chronological holdout<br/>game groups]
    LOAD --> SPLIT
    SPLIT --> CV[Root grouped CV wrapper]
    CV --> CVAPI[AutoML CrossValidator<br/>group folds]
    CV --> FIT[AutoML TrainEngine<br/>fold fit and score]
    SPLIT --> FITFINAL[AutoML TrainEngine<br/>final fit]
    FITFINAL --> MODEL[TrainEngine JSON artifact]
    MODEL --> PRED[Root ModelPolicy / TrainEngine predict]
    PRED --> GAME[2048 seeded evaluator]
    PRE[AutoML DataPreprocessor] -. optional independent API;<br/>not in root train path .-> FIT
    OPT[AutoML HyperOptX] -. objective callback;<br/>not wired into root train CLI .-> FIT
    INF[AutoML InferenceEngine] -. optional API;<br/>root policy does not use it .-> PRED
```

The root has two parallel layers: it owns the game, feature/data protocol, game-group holdout, and case-study simulation; the AutoML submodule owns model fitting, framework preprocessing APIs, generic CV splitting, optimization primitives, inference utilities, and model serialization. The root's group-aware CV wrapper selects rows and evaluates folds around `TrainEngine::fit` because the framework's generic `cross_val_score` does not accept group labels (its implementation passes `groups=None` to `CrossValidator::split`). It checks that groups do not overlap between each fold's train and test indices.

## Data and ownership contracts

| Boundary | Observed contract | Source |
|---|---|---|
| Training table | Polars `DataFrame`; target column named `action`; root CSV has 17 `f64` state values and one action column per Plan 00. | `src/data_pipeline.rs`; `automl/src/training/engine.rs::prepare_data`; Plan 00 |
| Framework model input | Numeric columns cast to `f64`, then copied into `ndarray::Array2<f64>`; target cast to `Array1<f64>`; null feature/target values are currently replaced with zero in conversion | `automl/src/training/engine.rs::columns_to_array2` |
| Game provenance | Sidecar has row index, game ID, move index, and score; the row index aligns it to training CSV; IDs are not model features | `src/data_pipeline.rs` |
| Group split | Root partitions game IDs chronologically for holdout; root CV wrapper uses AutoML `CrossValidator::split` with a group array, then materializes train/test DataFrames | `src/main.rs`; `src/training.rs` |
| Preprocessing | `DataPreprocessor` fits stateful imputer/scaler/encoder statistics and offers transform/save/load; root's canonical numeric features currently bypass it | `automl/src/preprocessing/pipeline.rs`; `src/main.rs` |
| Model output | `TrainEngine` owns config, feature names, a trained model, metrics and histories; save/load serialize the engine to JSON | `automl/src/training/engine.rs` |
| Prediction | Root game policy loads the serialized AutoML `TrainEngine` and maps four-class predictions to legal game actions; `InferenceEngine` is a separate generic inference component | `src/policy.rs`; `automl/src/inference/engine.rs` |

The framework APIs use concrete Rust structs/enums and `Result` errors at their public boundaries. `DataFrame` is borrowed for fit/predict calls; the engine extracts owned numeric arrays and stores its fitted model and feature names. CV returns row indices, while the root wrapper owns the selected fold DataFrames. The JSON model is a file artifact; no explicit schema-version field or migration policy was verified in `TrainEngine` serialization.

## Configuration and capability surfaces

- `TrainingConfig` specifies task/model/target/features, validation fraction, `cv_folds`, optional seed, metric, parallelism, and algorithm parameters. `TrainEngine::fit` extracts the data and performs its own seeded train/validation split; source inspection shows that this fit path does not call `CrossValidator` and does not consume `cv_folds`. Root integration therefore runs group CV explicitly and then fits a final engine separately.
- `PreprocessingConfig` specifies imputers, scaler, encoder, outlier and feature expansion options, jobs, and seed. A fitted `DataPreprocessor` is serializable. Leakage safety is a caller responsibility: fit only on a training partition and transform held-out data with the fitted instance.
- `OptimizationConfig` provides trials, timeout, direction, sampler, seed, parallel-worker count, pruning flag, early stopping, and metric/fold fields. In the inspected `HyperOptX::optimize` path, objective calls are sequential and errors are represented as pruned trials without retaining the error value in the result; an objective callback must perform actual fitting/CV. The generic optimizer does not itself bind to `TrainEngine` or enforce the declared metric/folds.
- `InferenceConfig` exposes batch/worker, streaming, cache, memory-budget, quantization, and probability options. `InferenceEngine` can own an optional preprocessor and a `TrainEngine`; the 2048 root policy currently predicts through the model policy path, not this engine.
- AutoML's `ModelType` is a broad enum, not a guarantee that every model works for every task or emits a consistent probability matrix. For four-action classification, the verified candidate set is recorded in `plans/01-Infrastructure/01-Project/01-project-overview.md` and rechecked by `src/framework_validation.rs`.

## Failure, seed, parallelism, and compatibility observations

- Public framework operations generally return `automl::Result`; the root CLI currently converts many workflow failures to `expect` panics after argument/schema checks. Trial objective errors are marked pruned without preserving the error text in `TrialResult`, limiting diagnostic evidence.
- Seeds are configurable in training, CV, preprocessing, optimization, and game simulation as separate values. The root CLI derives a CV seed with `seed.wrapping_add(4)` and uses the requested seed for final fit. There is no single cross-component seed manager or framework-wide determinism guarantee.
- Parallelism knobs exist in preprocessing, training configuration, optimization configuration, and inference configuration. The inspected `HyperOptX::optimize` loop evaluates trials serially; actual model-level parallelism is algorithm/config dependent and must be measured. No framework-level recovery/checkpoint policy was verified for an interrupted training/search run.
- `TrainEngine` and `DataPreprocessor` expose JSON save/load methods. This audit found no explicit artifact version negotiation or backward migration mechanism, so compatibility across framework revisions remains unverified.
- Both CLI and Rust library surfaces exist, but equivalent results for a common fixed configuration have not been measured. The root uses library APIs plus the framework CSV loader; command availability is not evidence of CLI/library equivalence.

### Follow-up source and repeatability probe

On 2026-09-24, the focused root RandomForest save/load smoke failed once in 20 runs. Follow-up isolation found three issues in the local AutoML checkout: leaf class ties depended on `HashMap` iteration; default `serde_json` float parsing altered serialized model values; and stratified splitting appended class rows in `HashMap` iteration order, making identical-seed fits differ. Deterministic leaf tie-breaking, `serde_json/float_roundtrip`, and ordered class grouping now address these cases. The tie regression passed; a synthetic same-seed check passed 20/20 process runs with 20 refits per run; and exact save/load model-state and prediction checks passed 20/20 process runs. The broader multi-seed and dataset reproducibility study remains outstanding. These fixes are committed and pinned at AutoML revision `88a86bf44a0cb03664931f7ef15201b95fa11255`.

## Evidence boundary

This map records the original source audit at the revisions above. Ticket #034 subsequently changed the root state and CSV integration to 17 values, as required by Plan 00. The audit does not measure predictive quality, memory, training/inference time, search efficiency, reproducibility across processes, external-framework trade-offs, or standard-dataset portability. Those experiments remain required by the framework-contribution and framework-validation tickets.

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.
4. `grep -q '^\\*\\*Goal:' plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/01-Project/04-framework-architecture.md` exits 0.

## Open questions

- Standard-dataset performance, matched baselines, resource use, repeatability, and external replication are unmeasured.

## Later

- Use the framework-validation ticket to collect empirical evidence before drawing framework quality conclusions.
