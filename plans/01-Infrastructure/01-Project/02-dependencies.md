# Plan 02 — Dependencies: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Corrected group-CV chronology and AutoML fit/inference behavior; five verified four-class candidates listed.

**Goal:** State the current implementation and evidence boundary for dependencies.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Corrected group-CV chronology and AutoML fit/inference behavior; five verified four-class candidates listed.

> **Focus:** automl capabilities table for 2048. For `Cargo.toml` pins see `03-Dependencies/01-rust-deps.md`.

## 1. Primary Dependency: automl Submodule

| Property | Value |
|----------|-------|
| Repository | `https://github.com/Evintkoo/automl` |
| Submodule Path | `automl/` |
| Pinned Commit | `88a86bf44a0cb03664931f7ef15201b95fa11255` (`v1.0.0-139-g88a86bf`) |
| Language | Rust 1.75+ |
| License | MIT |

Verify pin: `git submodule status automl` must match above. See `03-Dependencies/02-submodule-deps.md` for update cadence.

## 2. automl Capabilities Used for 2048

Canonical task per [Plan 00](../../00-scope-and-traceability.md): `TaskType::MultiClassification` on 17 values (16 board cells plus current score) → `action: u8` (0=Up, 1=Down, 2=Left, 3=Right). Ticket #034 now aligns the root encoder, policy, and CSV schema with this contract.

| Feature | Module (`automl/src/...`) | 2048 Wiring |
|---------|---------------------------|-------------|
| `TrainingConfig::new(TaskType::MultiClassification, "action")` | `training/config.rs` | Target `action` u8 0–3; `.with_cv(5)` sets a config field but `TrainEngine::fit` does not execute CV; the root wrapper runs grouped CV separately. `.with_random_state(42)` sets `random_seed`. |
| `ModelType` variants (four-class probability smoke passed) | `training/config.rs` + `training/engine.rs` | Eligible initial candidates: `RandomForest`, `ExtraTrees`, `AdaBoost`, `KNN`, `NaiveBayes`. Other variants may fit but currently return only two probability columns and are excluded until fixed and verified. |
| `CrossValidator` + `CVStrategy` | `training/cross_validation.rs` | Root wrapper uses `CVStrategy::GroupKFold { n_splits: 5 }` with `groups=game_id` to prevent trajectory leakage. The pinned splitter sorts groups and assigns them round-robin; it is group-disjoint but not chronological. Use a separate forward-chaining procedure when chronology is required. |
| `TrainEngine` | `training/engine.rs` | `fit(&df)` / `predict(&df)` on Polars DataFrame with 17 numeric features + `action`. |
| `HyperOptX` + `OptimizationConfig` + `MedianPruner` | `optimizer/` | `OptimizeDirection::Maximize` (accuracy/score); `MedianPruner::new(false)` — `false`=maximize |
| `DataPreprocessor` | `preprocessing/` | Not currently used by the root collection/training path; feature encoding is deterministic and tree candidates are trained on the canonical numeric values. Any fitted transform must be fit on development training folds only. |
| `InferenceEngine` | `inference/` | Root policy calls `predict_proba_array`, verifies four columns, then masks illegal moves before `argmax` |

> **Not used for MultiClassification.** `KMeans`, `DBSCAN`, `SOM` require `TaskType::Clustering`; regression-only types (`LinearRegression`, `Ridge`, `Lasso`, `ElasticNet`, `PolynomialRegression`, `GaussianProcess`, `SGD` as regressor) are not candidates for 17-value-state → `action` classification. Listed here for completeness only — do not benchmark them for 2048.

## 3. Current DataFrame Schema (17 canonical values)

Cross-reference: Plan 00 fixes the canonical training state at 17 values. The previous 27-column strategic-feature vector is not used as canonical training input.

```
The root schema is 17 numeric state columns (f64) + 1 target column (`action`, u8):
  grid_0..grid_15     — tile values / 32768 (indices 0–15)
  score_normalized    — log10(score+1)/6 (index 16)
  action: u8 0..3    — target (MultiClassification)
```

```rust
use automl::{TrainingConfig, TaskType, ModelType};
use automl::training::cross_validation::{CrossValidator, CVStrategy};
use polars::prelude::*;

// DataFrame has 17 state columns + `action: u8`
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
| `polars` 0.46 | DataFrame for 17 state values + action |
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

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/01-Project/02-dependencies.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Corrected group-CV chronology and AutoML fit/inference behavior; five verified four-class candidates listed. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
