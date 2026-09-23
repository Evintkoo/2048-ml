# Training Pipeline Configuration

## 1. Pipeline Stages

The training pipeline consists of the following stages, each with its own configuration:

```
Data Collection → Preprocessing → Training → Evaluation → Model Selection → Export
```

## 2. Data Preprocessing Configuration

### 2.1 Scaler Configuration

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, FeatureSelector, ScalerType, EncoderType, ImputeStrategy};

let preprocess_config = PreprocessingConfig::default()
    .with_scaler(ScalerType::Standard)  // StandardScaler (real API: with_scaler)
    .with_encoder(EncoderType::OneHot)  // real type: EncoderType, not EncodingType
    .with_numeric_impute(ImputeStrategy::Mean); // real type: ImputeStrategy, not ImputationStrategy

let preprocessor = DataPreprocessor::new(preprocess_config);
let (x_processed, y_processed) = preprocessor.fit_transform(df)?;
```

### 2.2 Feature Engineering — 27-dim canonical (cross-reference `03-State/01-Board/02-feature-extraction.md` + `06-Data/02-Format/01-data-schema.md`)

> This file shows only the wiring shape; canonical definitions are not duplicated here.

For 2048, 27 features in canonical order: `grid_0..15` (/32768) + `empty_count/16`, `max_tile_log/log2/15`, `monotonicity`, `smoothness`, `merges_available/16`, `score_normalized` log10/6 (idx 21), `adjacency_merge_score`, `corner_max` (=max_corner/32768), `edge_tiles/12`, `col_worst/8192`, `row_worst/8192`; target `action: u8` 0–3.

```rust
let feature_config = FeatureSelector::new()
    .with_method(SelectionMethod::Correlation(0.95))
    .with_target_column("action");
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
  model_type: Auto   # was 'classification' — use ModelType::Auto or specific e.g. GradientBoosting/RandomForest
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
  enabled: true
  algorithm: tpe
  n_trials: 100
  pruner: median
  
seed: 42
```

## 6. Reproducibility Configuration

```rust
let mut config = TrainingConfig::default();
config.random_seed = Some(42);  // struct field (builder is with_random_state(42))
config.n_jobs = Some(1);        // struct field (no builder for Option variant)
```

## 7. Model Export Configuration

```rust
use automl::export::{ModelSerializer, SerializationFormat, ONNXExporter};

 // ModelSerializer is a trait (export/serializer.rs:197), not a struct.
// Native serialization via trait methods:
let bytes = model.to_bytes()?;
model.save("model.bin", SerializationFormat::Binary)?;
let json = model.to_json()?;

// ONNX export via ONNXExporter (not ModelSerializer::with_format):
let exporter = ONNXExporter::new();
exporter.export_json(&model, "model.onnx.json")?;
```
