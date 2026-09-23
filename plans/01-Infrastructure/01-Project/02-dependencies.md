# Dependencies — automl Capabilities for 2048

> **Focus:** automl capabilities table for 2048. For `Cargo.toml` pins see `03-Dependencies/01-rust-deps.md`.

## 1. Primary Dependency: automl Submodule

| Property | Value |
|----------|-------|
| Repository | `https://github.com/Evintkoo/automl` |
| Submodule Path | `automl/` |
| Pinned Commit | `64f5edad29c9e58ee7d33abf380418d5cfbbb561` (v1.0.0-138-g64f5eda) |
| Language | Rust 1.75+ |
| License | MIT |

Verify pin: `git submodule status automl` must match above. See `03-Dependencies/02-submodule-deps.md` for update cadence.

## 2. automl Capabilities Used for 2048

Canonical task: `TaskType::MultiClassification` on 27-dim state → `action: u8` (0=Up, 1=Down, 2=Left, 3=Right).

| Feature | Module (`automl/src/...`) | 2048 Wiring |
|---------|---------------------------|-------------|
| `TrainingConfig::new(TaskType::MultiClassification, "action")` | `training/config.rs` | Target `action` u8 0–3; `.with_cv(5)` → `cv_folds` field; `.with_random_state(42)` → `random_seed` |
| `ModelType` variants (classification) | `training/config.rs` | `RandomForest`, `GradientBoosting`, `XGBoost`, `LightGBM`, `CatBoost`, `DecisionTree`, `ExtraTrees`, `AdaBoost`, `SVM`, `KNN`, `NaiveBayes`, `LogisticRegression`, `Auto` |
| `CrossValidator` + `CVStrategy` | `training/cross_validation.rs` | `CVStrategy::GroupKFold { n_splits: 5 }` groups=`game_id`, `shuffle=false` (preserve chronology); `TimeSeriesSplit` for temporal forward-chain experiments; `CrossValidator::new(strategy).with_random_state(42)` |
| `TrainEngine` | `training/engine.rs` | `fit(&df)` / `predict(&df)` on polars DataFrame with 27 feature cols + `action` |
| `HyperOptX` + `OptimizationConfig` + `MedianPruner` | `optimizer/` | `OptimizeDirection::Maximize` (accuracy/score); `MedianPruner::new(false)` — `false`=maximize |
| `DataPreprocessor` | `preprocessing/` | Standard scaling on 27-dim vector; no clustering passes |
| `InferenceEngine` | `inference/` | Single-state `predict` → `argmax` over 4 logits |

> **Not used for MultiClassification.** `KMeans`, `DBSCAN`, `SOM` require `TaskType::Clustering`; regression-only types (`LinearRegression`, `Ridge`, `Lasso`, `ElasticNet`, `PolynomialRegression`, `GaussianProcess`, `SGD` as regressor) are not candidates for 27-dim → `action` classification. Listed here for completeness only — do not benchmark them for 2048.

## 3. DataFrame Schema (27-dim canonical)

Cross-reference: canonical feature definition lives in `03-State/01-Board/02-feature-extraction.md` and `06-Data/02-Format/01-data-schema.md`. This file shows only the wiring shape.

```
27 feature cols (f64) + 1 target col (u8):
  grid_0..grid_15     — tile values / 32768  (indices 0–15)
  empty_count/16, max_tile_log/log2/15, monotonicity, smoothness,
  merges_available/16, score_normalized log10(score+1)/6 (idx 21),
  adjacency_merge_score, corner_max (=max_corner/32768),
  edge_tiles/12, col_worst/8192, row_worst/8192  (indices 16–26)
  action: u8 0..3    — target (MultiClassification)
```

```rust
use automl::{TrainingConfig, TaskType, ModelType};
use automl::training::cross_validation::{CrossValidator, CVStrategy};
use polars::prelude::*;

// DataFrame has 27 feature columns + `action: u8`
let config = TrainingConfig::new(TaskType::MultiClassification, "action")
    .with_model(ModelType::RandomForest)
    .with_cv(5)               // sets cv_folds = 5 (verified in training/config.rs:207)
    .with_random_state(42);   // sets random_seed = Some(42) (config.rs:213)

// GroupKFold: groups = game_id, shuffle=false (chronological)
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
    .with_random_state(42);
let splits = cv.split(n_samples, None, Some(&groups))?; // groups: Array1<i64> of game_id

// Temporal alternative (no groups, respects order)
let tss = CrossValidator::new(CVStrategy::TimeSeriesSplit { n_splits: 5, max_train_size: None });
```

> `cv_folds` is not a builder called `cv_folds()` — the builder is `with_cv()`. Verify in `automl/src/training/config.rs:207`.

## 4. Rust Crate Surface (summary only)

Detailed pins and `Cargo.toml` live in `03-Dependencies/01-rust-deps.md`. No duplication.

| Crate | Role for 2048 |
|-------|---------------|
| `polars` 0.46 | DataFrame for 27-dim + action |
| `ndarray` 0.16 | Fallback arrays for CV helpers |
| `rand` / `rand_chacha` | Deterministic game RNG, CV shuffling |
| `rayon` | Parallel batch simulation + `par_iter` CV |
| `clap` | Single binary with subcommands (see `04-Tooling/01-cli-tools.md`) |

## 5. Submodule Verification (before training)

```bash
git submodule status automl          # hash must match §1
cargo test -p automl --lib -- training::config::tests  # smoke
cargo test -p automl --lib -- training::cross_validation::tests
```
