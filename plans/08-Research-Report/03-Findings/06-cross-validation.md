# Cross-Validation — GroupKFold by game_id (i.i.d.), Not Temporal

> **Fix:** Games are **i.i.d. conditional on seed** — order is irrelevant. **TimeSeries CV claim deleted.** Canonical is `GroupKFold` by `game_id` to keep a game's states together. Bound nested cost correctly.

## 1. Why CV and Leakage Model

Train/test split: single random split risks leakage if one game's board states land in both train and test. Solution: **group by `game_id`** so all positions from one game stay together.

## 2. Canonical Strategy

```rust
use automl::training::{CrossValidator, CVStrategy};
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 }).with_random_state(42);
// split(n_samples, Some(&y), Some(&groups)) where groups = polars column game_id as Array1<i64>
```

- `KFold{shuffle:true, n_splits:5}` only if no `game_id`; but we **have** `game_id` → GroupKFold is canonical.
- `StratifiedKFold` on `action` 0–3 if class-imbalance check required.
- **Blocked CV** = same as GroupKFold with `game_id`.

**TimeSeriesSplit is not used** — 2048 rollout data has no temporal dependence across games; forward chaining would artificially reduce sample size and is removed.

## 3. Nested CV Cost (Corrected)

Outer 5 × Inner 3 → **15 fits total**, not 105. Each fit is `TrainEngine::fit` + `predict`. Cost in `07-computational-budget.md`: ~50 CPUh for full 5-fold. Do not multiply by 7 models in same bound — per-model.

## 4. Rust Stub (Cross-Ref, Not Duplicate)

```rust
pub struct CVResults { pub scores: Vec<f64>, pub mean_score: f64, pub std_score: f64 }
pub fn run_groupkfold(x: &Array2<f64>, y: &Array1<f64>, groups: &Array1<i64>, seed: u64) -> CVResults {
    let cv = CrossValidator::new(CVStrategy::GroupKFold{ n_splits:5 }).with_random_state(seed);
    let splits = cv.split(x.nrows(), Some(y), Some(groups)).unwrap();
    // TrainEngine::fit_predict_arrays per split → accuracy or R²
    unimplemented!("project wrapper: split with groups, then score each fold")
}
```

The splitter comes from `automl/src/training/cross_validation.rs`; the grouped scoring wrapper belongs to the 2048 project because automl's convenience `cross_val_score` does not accept or forward groups.

## 5. Decision

Use 5-fold GroupKFold for model selection; final ranking still uses held-out 10k benchmark games (not CV score) to avoid optimistic bias.
