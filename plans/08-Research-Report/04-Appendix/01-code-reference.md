# Code Reference — Actual Repo Structure (Not Hallucinated)

> **Fix:** Prior `plans/` org diagram deleted. References now align with real `automl/` crate + proposed 2048 crates under root. Paths verified against `automl/src` and `plans/**/reproducibility-package.md`.

## 1. Real Repo Layout

```
2048-ml/                          # root (plans/ + automl submodule)
├── automl/                      # submodule Evintkoo/automl v1.0.0
│   ├── src/training/config.rs   # TrainingConfig, TaskType::MultiClassification, ModelType
│   ├── src/training/engine.rs   # TrainEngine::fit / fit_predict_arrays
│   ├── src/training/cross_validation.rs # CrossValidator, CVStrategy::GroupKFold
│   ├── src/optimizer/           # HyperOptX, TPE
│   ├── src/preprocessing/       # DataPreprocessor
│   └── Cargo.toml               # polars 0.46, smartcore 0.3
├── src/                          # (proposed) single root MVP crate
│   ├── game_engine/              # board, score, engine, spawn
│   ├── data_pipeline/            # feature extraction and rollout labels
│   └── evaluation/               # benchmark, statistics, ranking
├── Cargo.lock / requirements.txt # pinned
└── plans/                       # this docs repo — not code
```

## 2. Correct TrainingConfig Example (Real API)

```rust
use automl::training::{TrainingConfig, TaskType, ModelType};
let config = TrainingConfig::new(TaskType::MultiClassification, "action")
    .with_model(ModelType::RandomForest)
    .with_random_state(42)
    .with_cv(5);
// feature_columns: 27 feature names; game_id is metadata passed separately
// to CrossValidator::split, never a model feature.
```

Previous `learning_rate/epochs/batch_size` example deleted — not real automl fields (see `automl/src/training/config.rs`: `n_estimators, max_depth, learning_rate, subsample`, etc.).

## 3. Data Flow (Code-Verified)

`engine.rs:GameEngine::execute_move(0..3)` → `feature_extraction::extract_27(board)` → `label_generation::rollout_label(board, 100)` → `TrainEngine::fit` → `benchmark_runner::run_n(10000, seed=42)` → `statistical_tests::mann_whitney`.

## 4. Dependencies (Pinned)

`automl v1.0.0, rust 1.75, polars 0.46, rand_chacha 0.3`. See `automl/Cargo.toml`.
