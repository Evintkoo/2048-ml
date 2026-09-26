# Plan 02 — Training Pipeline Configuration: the repository status is explicit and evidence based

> **Status: PARTIAL.** Preprocessing guidance matches the real API; root records training inputs/seeds in model manifests, while YAML config loading remains unimplemented.

**Goal:** State the current implementation and evidence boundary for training pipeline configuration.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

**Canonical input contract:** Plan 00 requires 17 training values: 16 board cells plus current score. Ticket #034 aligns the state encoder, policy, collector, and CSV schema with that contract.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** Preprocessing examples use the pinned API, but the root does not call `DataPreprocessor`; the canonical encoder produces deterministic numeric features directly. The illustrative training YAML remains unconsumed by the CLI. Optional tuning is instead a separate versioned JSON search contract for RandomForest/ExtraTrees, with grouped-CV objective, derived seed, selected settings, saved study, and training manifest. The manifest records data digests and the actual checked-out AutoML revision. YAML parsing, fitted preprocessing, and experiments that justify changing the feature protocol remain pending.

## 1. Pipeline Stages

The training pipeline consists of the following stages, each with its own configuration:

```
Data Collection → Preprocessing → Training → Evaluation → Model Selection → Export
```

## 2. Data Preprocessing Configuration

### 2.1 Scaler Configuration

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType, EncoderType, ImputeStrategy};

let preprocess_config = PreprocessingConfig::default()
    .with_scaler(ScalerType::Standard)  // StandardScaler (real API: with_scaler)
    .with_encoder(EncoderType::OneHot)  // real type: EncoderType, not EncodingType
    .with_numeric_impute(ImputeStrategy::Mean); // real type: ImputeStrategy, not ImputationStrategy

let mut preprocessor = DataPreprocessor::with_config(preprocess_config);
let transformed = preprocessor.fit_transform(df)?; // returns a DataFrame
```

The root pipeline currently does not call `DataPreprocessor`; its feature values are generated deterministically by the 2048 state encoder. This avoids learning scaling or imputation statistics from data. If a separately scoped experiment enables preprocessing later, fit it on each training fold only, exclude the target from transformation, and apply the fitted transform to validation/test data.

### 2.2 Canonical State Input — 17 values

> This file shows only the wiring shape; canonical definitions are not duplicated here.

The model input is `grid_0..15` (tile value /32768) plus `score_normalized` (`log10(score+1)/6`) at index 16; target `action: u8` 0–3. The 11 strategic metrics from the prior 27-column encoder are excluded from canonical training.

```rust
// FeatureSelector is a separate ndarray helper, not a DataFrame builder.
// Feature selection is intentionally not run in the current pipeline; any
// future selection must be fitted within each training fold only.
```

## 3. Cross-Validation Configuration

```rust
use automl::{CrossValidator, CVStrategy};

// GroupKFold keeps game IDs disjoint across folds; the pinned splitter sorts groups
// and assigns them round-robin, so it is not chronological. The root separately
// reserves the final chronological game groups as a holdout.
// StratifiedKFold is valid for class-balance checks; TimeSeriesSplit for temporal forward-chain experiments.
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
    .with_random_state(42);
let splits = cv.split(n_samples, None, Some(&groups))?; // groups: Array1<i64> game_id

// Class-balance variant (not canonical — use only if leakage already controlled):
// let cv2 = CrossValidator::new(CVStrategy::StratifiedKFold { n_splits: 5, shuffle: true }).with_random_state(42);
// let splits2 = cv2.split(x.nrows(), Some(&y), None)?;
// Do not use automl::cross_val_score for GroupKFold: it does not forward
// groups. Use the project grouped-CV wrapper with groups=game_id.
```

## 4. Early Stopping Configuration

```rust
let mut config = TrainingConfig::default();
config.early_stopping = true;          // struct field (no builder)
config.early_stopping_rounds = 50;     // struct field (no builder)
```

## 5. Experiment Configuration Template (illustrative YAML; not consumed)

```yaml
experiment:
  name: "2048-random-forest-v1"
  version: "1.0"
  
training:
  task_type: multiclassification
  model_type: RandomForest  # current verified four-class output shape; root CLI does not parse YAML
  n_estimators: 100
  max_depth: 6
  
validation:
  cv_folds: 5
  validation_split: 0.2 # framework-internal fit split; grouped CV owns evaluation
  early_stopping: false # unsupported by the current root TrainEngine integration
  
optimizer:
  enabled: false # YAML is illustrative only; use the separate JSON CLI contract
  
seed: 42
```

The YAML block is not loaded by the root CLI and is not a runnable experiment
configuration. It is a field illustration, not an executable contract; in
particular, early stopping must remain false because the root does not wire its
validation semantics. The implemented tuning input is `config/hyperopt-search.example.json`;
it controls `n_trials`, TPE, and integer ranges for `n_estimators` and
`max_depth` only. The root currently accepts this search for RandomForest and
ExtraTrees; other shown training settings are descriptive, not parser-backed.
Do not infer that `validation_split`, `early_stopping`, or preprocessing
examples are consumed from this YAML.

## 6. Reproducibility Configuration

```rust
let seeds = game2048_ml::seeds::SeedManager::new(42);
let mut config = TrainingConfig::default()
    .with_random_state(seeds.training_seed()); // final-fit seed equals the global seed
config.n_jobs = Some(1);        // struct field (no builder for Option variant)
```

## 7. Model Export Configuration

```rust
use automl::export::{ModelSerializer, SerializationFormat, ONNXExporter};

 // ModelSerializer is a trait (export/serializer.rs:197), not a struct.
// This low-level model export example is distinct from the root integration,
// which saves a fitted TrainEngine via engine.save(path).
// Native serialization via trait methods:
let bytes = model.to_bytes()?;
model.save("model.bin", SerializationFormat::Binary)?;
let json = model.to_json()?;

// ONNX export via ONNXExporter (not ModelSerializer::with_format):
let exporter = ONNXExporter::new();
exporter.export_json(&model, "model.onnx.json")?;
```

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/02-Configuration/02-training-config.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** The root does not use fitted preprocessing or consume YAML configs. The separate JSON tuning path was smoke-run with two trials over eight development games; this verifies wiring only, not model quality. The AutoML optimizer has no intermediate-reporting hook for its standalone pruner. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
