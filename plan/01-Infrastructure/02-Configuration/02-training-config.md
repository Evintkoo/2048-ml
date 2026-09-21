# Training Pipeline Configuration

## 1. Pipeline Stages

The training pipeline consists of the following stages, each with its own configuration:

```
Data Collection → Preprocessing → Training → Evaluation → Model Selection → Export
```

## 2. Data Preprocessing Configuration

### 2.1 Scaler Configuration

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, FeatureSelector};

let preprocess_config = PreprocessingConfig::default()
    .with_scaler_type(ScalerType::Standard)  // StandardScaler
    .with_encoding(EncodingType::OneHot)
    .with_imputation(ImputationStrategy::Mean);

let preprocessor = DataPreprocessor::new(preprocess_config);
let (x_processed, y_processed) = preprocessor.fit_transform(df)?;
```

### 2.2 Feature Engineering

For 2048, features include:
- Board state (4x4 grid flattened to 16 features)
- Score
- Move count
- Available moves count
- Tile statistics (max, min, mean, variance)
- Monotonicity
- Empty tiles count
- Merges possible

```rust
let feature_config = FeatureSelector::new()
    .with_method(SelectionMethod::Correlation(0.95))
    .with_target_column("score");
```

## 3. Cross-Validation Configuration

```rust
use automl::{CrossValidator, CVStrategy};

let cv = CrossValidator::new()
    .with_k_folds(5)
    .with_strategy(CVStrategy::Stratified)
    .with_shuffle(true)
    .with_random_state(42);

let results = cv.cross_val_score(&engine, &x, &y)?;
```

## 4. Early Stopping Configuration

```rust
let config = TrainingConfig::default()
    .with_early_stopping(true)
    .with_early_stopping_rounds(50);
```

## 5. Experiment Configuration Template

```yaml
experiment:
  name: "2048-gradient-boosting-v1"
  version: "1.0"
  
training:
  task_type: regression
  model_type: gradient_boosting
  n_estimators: 200
  max_depth: 6
  learning_rate: 0.1
  subsample: 0.8
  colsample_bytree: 0.8
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
let config = TrainingConfig::default()
    .with_random_seed(Some(42))  // Global seed
    .with_n_jobs(Some(1));       // Deterministic parallelism
```

## 7. Model Export Configuration

```rust
use automl::export::{ModelSerializer, SerializationFormat};

let serializer = ModelSerializer::new()
    .with_format(SerializationFormat::ONNX)
    .with_compression(true);
    
let bytes = serializer.serialize(&model)?;
```
