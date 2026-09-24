# Plan 02 — Training Pipeline Configuration: the repository status is explicit and evidence based

> **Status: PARTIAL.** Preprocessing guidance matches the real API; root records training inputs/seeds in model manifests, while YAML config loading remains unimplemented.

**Goal:** State the current implementation and evidence boundary for training pipeline configuration.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** Preprocessing examples now use the pinned API. The root encoder produces deterministic, documented numeric features, so it does not fit imputation/scaling state; adding fitted preprocessing would change the feature protocol and must be separately justified. The illustrative YAML remains unconsumed by the CLI. The root exposes optional grouped-CV HyperOptX tuning for RandomForest/ExtraTrees and writes a sibling training manifest with data digests and derived seeds. CLI arguments remain the active configuration source.

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

### 2.2 Feature Engineering — 27-dim canonical (cross-reference `03-State/01-Board/02-feature-extraction.md` + `06-Data/02-Format/01-data-schema.md`)

> This file shows only the wiring shape; canonical definitions are not duplicated here.

For 2048, 27 features in canonical order: `grid_0..15` (/32768) + `empty_count/16`, `max_tile_log/log2/15`, `monotonicity`, `smoothness`, `merges_available/16`, `score_normalized` log10/6 (idx 21), `adjacency_merge_score`, `corner_max` (=max_corner/32768), `edge_tiles/12`, `col_worst/8192`, `row_worst/8192`; target `action: u8` 0–3.

```rust
// FeatureSelector is a separate ndarray helper, not a DataFrame builder.
// Feature selection is intentionally not run in the current pipeline; any
// future selection must be fitted within each training fold only.
```

## 3. Cross-Validation Configuration

```rust
use automl::{CrossValidator, CVStrategy};

// Canonical for 2048: GroupKFold (groups=game_id, shuffle=false) to avoid leakage across moves of same game.
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

## 5. Experiment Configuration Template

```yaml
experiment:
  name: "2048-gradient-boosting-v1"
  version: "1.0"
  
training:
  task_type: multiclassification
  model_type: RandomForest  # current verified four-class output shape; root CLI does not parse YAML
  n_estimators: 200
  max_depth: 6
  learning_rate: 0.1
  subsample: 0.8
  colsample_bytree: 0.8
  # reg_alpha/reg_lambda: L1/L2 regularization — only for tree/boosting models (XGBoost/LightGBM/CatBoost); omit or keep minimal for RandomForest/ExtraTrees
  reg_alpha: 0.01
  reg_lambda: 1.0
  
validation:
  cv_folds: 5
  validation_split: 0.2
  early_stopping: true
  early_stopping_rounds: 50
  
  optimizer:
  enabled: false # root CLI does not apply HyperOptX trials yet
  algorithm: tpe
  n_trials: 100
  pruner: median
  
seed: 42
```

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

- **The plan-scale evidence remains bounded by current results.** Corrected preprocessing API and removed invalid selector builder; root does not yet use fitted preprocessing or consume YAML configs. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
