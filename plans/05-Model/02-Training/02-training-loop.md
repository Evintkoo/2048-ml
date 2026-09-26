# Plan 02 — Training Loop: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Classical fit/save/load and grouped CV are implemented; broader diagnostics and final test workflow remain pending.

**Goal:** State the current implementation and evidence boundary for training loop.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the classical training loop as implemented with evaluation gaps.** AutoML performs a single `TrainEngine::fit` call after the root grouped-CV wrapper runs explicit folds. The fit call includes its own seeded row-level validation split; the root loop does not implement epochs or a resumable optimizer state.

## 1. Purpose

Define the training loop for classical tree-based models on the 2048 task. Unlike neural networks, automl's tree models **do not** use epochs, forward/backward passes, gradients, or optimizers — training is a single `fit` that builds trees from the DataFrame.

## 2. Training Loop Overview

Training is **not** iterative weight updates. It is: DataFrame → `TrainingConfig` → `TrainEngine::fit(&df)` → serialized engine / inference.

```mermaid
flowchart TD
    subgraph "Classical Training Loop — No NN"
        DF[DataFrame<br/>17 numeric features + action 0-3]
        DF --> Config[TrainingConfig<br/>TaskType::MultiClassification<br/>ModelType::RandomForest<br/>cv_folds=5, random_state=42]
        Config --> Engine[TrainEngine::new(config)]
        Engine --> Fit[engine.fit(&df)<br/>builds trees — no forward/backward]
        Fit --> Save[engine.save(path) / metrics]
        Save --> Infer[InferenceEngine::with_model(engine)<br/>predict(&df) → action 0-3]
    end
```

> **No Optimizer / Scheduler / backward / gradient.** Those concepts do not exist in `automl/src/training/engine.rs`. `n_estimators` is the number of trees, **not** epochs.
> Note: n_estimators is not epochs — it counts trees, not gradient steps.

## 3. Loop Architecture

```mermaid
flowchart TD
    subgraph "Training Loop Components — Classical Only"
        DF[DataFrame<br/>polars]
        Config[TrainingConfig]
        Engine[TrainEngine]
        Infer[InferenceEngine]
        CV[CrossValidator<br/>GroupKFold / TimeSeriesSplit]
    end

    DF -->|&df| Engine
    Config -->|new(task, target).with_model().with_cv()| Engine
    Engine -->|fit| Infer
    Infer -->|predict| Metrics[Valid-Action Accuracy + F1 Macro + Mean Game Score]
    CV -->|split(n, None, Some(&groups))| Engine
```

No `DataLoader`, no `Optimizer`, no `LossFn`, no `Scheduler` — those were NN hallucinations and are deleted.

## 4. Training Stages

### 4.1 Initialization

```mermaid
flowchart LR
    Config[Load TrainingConfig<br/>MultiClassification / action]
    Config --> Engine[Initialize TrainEngine::new(config)]
    Engine --> Load[Load DataFrame<br/>state-action pairs]
    Load --> Fit[Call engine.fit(&df)]
    Fit --> Eval[Evaluate via metrics + cross_val_score]
```

### 4.2 Single-Pass Fit (Not Epoch Loop)

```mermaid
flowchart TB
    Start[Start Training]
    Start --> Prepare[Prepare DataFrame<br/>extract X 17 dims + y 0-3]
    Prepare --> Split[Internal stratified split<br/>validation_split=0.2]
    Split --> Build[Build trees<br/>n_estimators trees, max_depth per tree]
    Build --> Validate[Validate<br/>accuracy + F1 macro + loss if SGD]
    Validate --> Save[Save best model<br/>TrainEngine::save + InferenceEngine::with_model]
    
    style Build fill:#e8f5e9
    style Validate fill:#e3f2fd
```

There is **no** `for epoch in 0..n_epochs` loop. `n_estimators` controls how many trees are grown (e.g., 100 trees), not how many gradient steps are taken. Boosting variants grow trees sequentially to correct residuals, but this is internal to the model — not an outer epoch loop.

## 5. Training Loop Implementation

Verified against `automl/src/training/config.rs:72` and `engine.rs:140`:

```rust
use automl::{TrainingConfig, TaskType, ModelType};
use automl::training::TrainEngine;
use automl::inference::{InferenceEngine, InferenceConfig};
use polars::prelude::*;

// 1) DataFrame with 17 numeric features + target column "action" (u8 0-3)
let df: DataFrame = load_state_action_pairs()?; // see 06-Data/

// 2) Config — note: task is MultiClassification, not regression
let config = TrainingConfig::new(TaskType::MultiClassification, "action")
    .with_model(ModelType::RandomForest)   // or ExtraTrees, AdaBoost, KNN, NaiveBayes
    .with_cv(5)                                // cv_folds = 5
    .with_random_state(42)                     // random_seed = Some(42)
    // Optional tree params:
    // .with_n_estimators(150)  — number of trees, NOT epochs
    // .with_max_depth(6)
    // .with_learning_rate(0.1) — shrinkage for boosting only
    ;

// 3) Fit — single call, no forward/backward/gradient/update_weights
let mut engine = TrainEngine::new(config);
engine.fit(&df)?; // builds trees, computes metrics on internal validation split

// 4) Inspect metrics
if let Some(m) = engine.metrics() {
    println!("accuracy={:?} f1={:?} training_time={}", m.accuracy, m.f1_macro, m.training_time_secs);
}

// 5) Inference — wrap in InferenceEngine for serving
let inference = InferenceEngine::new(InferenceConfig::new())
    .with_model(engine); // consumes TrainEngine
let preds = engine.predict(&test_df)?; // → predicted action classes; root policy uses four-class probabilities
// Alternative direct: engine.predict(&test_df) without InferenceEngine wrapper

// 6) Cross-validated variant (group-aware, no leakage — see 04-Evaluation/02-cross-validation.md)
use automl::{CrossValidator, CVStrategy, cross_val_score};
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 }).with_random_state(42);
let splits = cv.split(n_samples, None, Some(&groups))?; // verified API
// Use the project-owned grouped-CV wrapper; automl::cross_val_score currently
// passes groups=None and therefore cannot run GroupKFold correctly.
let splits = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
    .with_random_state(42)
    .split(x.nrows(), Some(&y), Some(&groups))?;
// The root wrapper currently summarizes fold accuracy; declare additional diagnostics before use.
```

**Key API notes:**

- `TrainingConfig::new(TaskType::MultiClassification, "action").with_model(...).with_n_estimators(100).with_cv(5).with_random_state(42)` — all builders verified in `config.rs:172`.
- `TrainEngine::fit(&df)` takes a `&DataFrame`, not arrays — it extracts `target_column` internally via `prepare_data`.
- `InferenceEngine::predict(&df)` (or `engine.predict(&df)`) returns `Array1<f64>` of predicted actions — no `predict_proba` needed for ranking, but available for threshold analysis.
- `n_estimators` is **not** `n_epochs` — there is no `n_epochs` field. The hallucinated `self.config.n_epochs` and `engine.backward(&loss)` / `update_weights()` never existed.

## 6. Convergence & Early Stopping

Classical trees do **not** converge via loss curves. Early stopping is an optional field on `TrainingConfig`:

```rust
pub early_stopping: bool,              // default true
pub early_stopping_rounds: usize,      // default 50 — rounds without improvement before stopping
```

- Early stopping and iteration behavior are model-specific framework details; do not assume the same behavior across estimator variants without checking their implementation.
- For **bagging** models (RandomForest, ExtraTrees), `early_stopping` is inert — more trees monotonically reduce variance; tune `n_estimators` directly.
- For **SGD** (`ModelType::SGD`), `epoch_history: Vec<EpochRecord>` is populated and `max_iter` controls iterations, but SGD is the only iterative learner — not the primary 2048 model.

Do **not** plot epoch loss curves or attach a `Scheduler` — those are NN concepts.

## 7. Checkpoint Management

Checkpoints are model serialization, not epoch snapshots.

```mermaid
flowchart TD
    Engine[TrainEngine after fit]
    Engine --> Save[engine.save(path)<br/>serde_json to disk]
    Save --> Load[TrainEngine::load(path)<br/>or InferenceEngine::load(config, None, path)]
    Load --> Resume[Continue inference — no resume-training loop]
    
    Save --> Storage[Model Storage<br/>JSON serialized TrainedModel]
```

- Save best model immediately after `fit` — no patience counter, no `save_checkpoint()` per epoch.
- `TrainEngine::save` / `load` are the only checkpoint APIs (see `engine.rs:222`).
- Rollback is re-loading the previous JSON, not rewinding an optimizer state.

## 8. Training Files

All training loop files are in `05-Model/02-Training/`:

```mermaid
flowchart LR
    Dir[05-Model/02-Training]
    Dir --> N01[01-training-pipeline.md]
    Dir --> N02[02-training-loop.md]
    Dir --> N03[03-model-architecture.md]
```

## 9. Next Steps

1. Select model type in `03-model-architecture.md` via `TrainingConfig::new(MultiClassification, "action").with_model(...)`
2. Run fit with group-aware CV (`GroupKFold {n_splits:5}.with_random_state(42).split(n, None, Some(&groups))`)
3. Report protocol-defined diagnostics and game scores; no canonical thresholds are set (see `04-Evaluation/`)

## Implementation Record

- Training uses one classical `TrainEngine::fit` call and exports a serialized model. The root `GroupKFold` wrapper keeps games separate in explicit CV folds; AutoML fit then makes a seeded row-level validation split.
- No iterative epoch/checkpoint loop is used. The wrapper currently reports fold accuracy only; full four-class diagnostics and untouched test scoring remain pending.

---

## Verification (definition of done)

1. `test -f plans/05-Model/02-Training/02-training-loop.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/05-Model/02-Training/02-training-loop.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/05-Model/02-Training/02-training-loop.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/05-Model/02-Training/02-training-loop.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/05-Model/02-Training/02-training-loop.md` exits 0.
6. `grep -q '^## Open questions$' plans/05-Model/02-Training/02-training-loop.md` exits 0.
7. `grep -q '^## Later$' plans/05-Model/02-Training/02-training-loop.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/05-Model/02-Training/02-training-loop.md` exits 0.

## Open questions

- The model comparison depends on retained fold predictions/metrics and an untouched game-group test set, neither of which the current training command reports completely.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
