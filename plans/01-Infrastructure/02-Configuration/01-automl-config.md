# Plan 01 — AutoML Configuration: the repository status is explicit and evidence based

> **Status: PARTIAL.** Optional HyperOptX tuning now runs grouped CV for RandomForest/ExtraTrees; tracked config artifacts and pruning integration remain pending.

**Goal:** State the current implementation and evidence boundary for automl configuration.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** Optional TPE tuning is now available through `train --tune-trials N` for RandomForest and ExtraTrees. Each trial evaluates grouped-CV accuracy on development games, and the best trial's parameters are used for the final fit; the study is saved beside the model as `<model-stem>.study.json`. Tuning is opt-in and does not access the reserved chronological test tail. The optimizer runs serially and does not integrate its pruner API. A sibling model manifest now records input digests, AutoML pin, component seeds, selected parameters, and study path; versioned input configuration and full analysis provenance remain pending.

## 1. Overview

The automl framework provides `TrainingConfig` and `OptimizationConfig` structures that must be configured for the 2048 game training pipeline. This document defines the configurations used throughout the project.

## 2. Training Configuration

### 2.1 Base Configuration

```rust
use automl::{TrainingConfig, TaskType, ModelType};

let mut config = TrainingConfig::new(TaskType::MultiClassification, "action")
    .with_model(ModelType::RandomForest) // Use only variants verified to return four action probabilities
    .with_cv(5)                    // 5-fold cross-validation
    .with_random_state(42)         // Reproducible
    .with_max_depth(6)             // Tree depth limit
    .with_n_estimators(100)        // Ensemble size
    .with_learning_rate(0.1);      // Boosting rate
config.validation_split = 0.2;   // 80/20 split (struct field, no builder)
```

### 2.2 Task Type for 2048

Since 2048 requires choosing one of 4 discrete directions (Up/Down/Left/Right), the task type is **MultiClassification**:

| Property | Value | Rationale |
|----------|-------|-----------|
| TaskType | `MultiClassification` | Action space is 4 discrete directions |
| Target | `action` | 0=Up, 1=Down, 2=Left, 3=Right |
| Metric | `accuracy` / `f1_macro` | Standard classification metrics |
| Validation | 5-fold CV | Robust evaluation |

Do not use `ModelType::Auto` for the four-action policy until its candidate selection is constrained and verified. On the pinned framework, supported enum variants do not all return four probability columns for `MultiClassification`; the current root CLI permits only RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes.

### 2.3 Model Type Strategy

```rust
// Phase 1: Auto-selection
ModelType::Auto

// Phase 2: Top candidates
vec![ModelType::RandomForest, ModelType::ExtraTrees, ModelType::AdaBoost, ModelType::KNN, ModelType::NaiveBayes]

// Phase 3: Fine-tuning
ModelType::RandomForest  // candidate remains provisional until held-out game evaluation
```

## 3. Hyperparameter Optimization Configuration

```rust
use automl::{OptimizationConfig, SearchSpace, Parameter, ParameterType, OptimizeDirection, MedianPruner};

let opt_config = OptimizationConfig::default()
    .with_direction(OptimizeDirection::Maximize)
    .with_n_trials(100)
    .with_n_jobs(4);
// Pruner is separate; MedianPruner::new(minimize) — false = maximize (we maximize accuracy/score, so false)
// Verified in automl/src/optimizer/pruners.rs:85,93 — `minimize: bool` field, false keeps higher values
let pruner = MedianPruner::new(false); // false = maximize → prunes trials below median; true would be for minimize (loss)

// Capability note: the pinned HyperOptX API does not accept this pruner in
// OptimizationConfig. The root CLI can run HyperOptX, but cannot wire this
// standalone pruner into trials yet.

let search_space = SearchSpace::new()
    .int("n_estimators", 50, 300)
    .int("max_depth", 3, 10)
    .float("learning_rate", 0.01, 0.5);
// Build a model-specific space; subsample/colsample_bytree are not universal
// parameters and must not be claimed as applied unless the adapter maps them.
// Implemented CLI tuning currently searches n_estimators and max_depth only
// for RandomForest and ExtraTrees, whose AutoML adapters apply both parameters.
```

Run optional tuning with:

```bash
cargo run -- train --data data/raw/random_play.csv \
  --metadata data/raw/random_play.metadata.csv --model random_forest \
  --cv-folds 5 --tune-trials 20 --seed 42 --output models/random_forest.json
```

Wiring smoke (2026-09-24): two HyperOptX trials on a temporary nine-game synthetic CSV completed, reserved the final two games, saved `policy.study.json`, and fit the selected model. The synthetic score is only a code-path check; it is not model or framework evidence. The existing real-candidate smoke indicates intermittent RandomForest fit reproducibility, so actual tuning results remain exploratory pending resolution.

The trial score is grouped-CV row accuracy, not held-out game score. Results remain exploratory while AutoML repeated-fit reproducibility is unresolved; the current source audit found a plausible nondeterministic decision-tree leaf-tie path documented in [the framework architecture record](../01-Project/04-framework-architecture.md).

## 4. Inference Configuration

```rust
use automl::InferenceConfig;

let mut infer_config = InferenceConfig::default()
    .with_batch_size(64)
    .with_n_workers(4);
infer_config.cache_preprocessing = true; // struct field (no builder)
```

## 5. Configuration Files

All configurations will be stored in `config/` directory:

```mermaid
flowchart TD
    config[config/]
    config --> training
    config --> hyperopt
    config --> inference
    
    training --> base[baseline.yaml]
    training --> exp1[experiment-01.yaml]
    training --> exp2[experiment-02.yaml]
    
    hyperopt --> ss[search-space.json]
    hyperopt --> res[results.json]
    
    inference --> cfg[config.toml]
```

## 6. Configuration Versioning

All configuration changes will be tracked via git. Configuration files will include:

- **Version:** Semantic version of the config schema
- **Timestamp:** ISO 8601 timestamp
- **Author:** User who made the change
- **Notes:** Rationale for configuration choices

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/02-Configuration/01-automl-config.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Changed example to a verified model and corrected SearchSpace builder; optimizer/pruner integration and tracked config artifacts remain pending. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Add a versioned configuration/experiment manifest and supported pruner wiring after the framework optimizer contract is clarified.** Preserve the recorded trial space, seed, dataset digest, submodule revision, and analysis output for every reportable run.
