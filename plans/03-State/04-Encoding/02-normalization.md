# Feature Normalization

## 1. Purpose

Normalize all features to a consistent scale for optimal automl model performance.

## 2. Normalization Methods

| Method | Formula | Use Case |
|--------|---------|----------|
| StandardScaler | (x - μ) / σ | Most features |
| MinMaxScaler | (x - min) / (max - min) | Bounded features |
| LogTransform | log(x + 1) | Score, tile values |
| None | Raw values | Tree-based models |

## 3. Grid Value Normalization

```rust
// Tile values are powers of 2: 0, 2, 4, 8, ..., 32768
// Normalize to [0, 1] by dividing by max possible tile
fn normalize_grid_value(value: u8) -> f64 {
    value as f64 / 32768.0
}
```

## 4. Count Feature Normalization

```rust
// All count features are bounded (canonical 11):
// empty_count: [0, 16] → [0, 1]
// merges_available: [0, 16] → [0, 1]
// edge_tiles_occupied: [0, 12] → [0, 1]
fn normalize_count(value: usize, max: usize) -> f64 {
    value as f64 / max as f64
}
```

## 5. Score Normalization

```rust
// Score can range from 0 to ~1,300,000
// Use log10 normalization
fn normalize_score(score: u64) -> f64 {
    (score as f64 + 1.0).log10() / 6.0  // log10(1,000,000) ≈ 6
}
```

## 6. Move Count Normalization

```rust
// Game length varies from ~10 to ~1000 moves
fn normalize_move_count(moves: u64) -> f64 {
    (moves as f64) / 1000.0
}
```

## 7. automl Preprocessing Pipeline

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType, ImputeStrategy, EncoderType};

let config = PreprocessingConfig::default()
    .with_scaler(ScalerType::Standard)  // Z-score standardization (real API: with_scaler, not with_scaler_type)
    .with_numeric_impute(ImputeStrategy::Mean) // real type: ImputeStrategy, not ImputationStrategy
    .with_encoder(EncoderType::OneHot);       // real type: EncoderType, not EncodingType
// Tree models (RandomForest, GradientBoosting, XGBoost) are scale-invariant —
// use ScalerType::None to skip scaling, or Standard/MinMax if sharing a pipeline with linear models.

let preprocessor = DataPreprocessor::new(config);
let (x_normalized, _) = preprocessor.fit_transform(&raw_data)?;
```

## 8. Normalization for Different Model Types

| Model Type | Needs Normalization | Recommended Method |
|------------|-------------------|-------------------|
| Linear Regression | Yes | StandardScaler |
| Logistic Regression | Yes | StandardScaler |
| SVM | Yes | StandardScaler |
| KNN | Yes | MinMaxScaler |
| Random Forest | No | None (tree-based) |
| Gradient Boosting | No | None (tree-based) |
| XGBoost | No | None (tree-based) |
| Neural Networks | Yes | StandardScaler |

## 9. Normalization Persistence

```rust
// Save fitted preprocessor for consistent inference
pub fn save_preprocessor(preprocessor: &DataPreprocessor, path: &str) -> Result<()> {
    let config = preprocessor.get_config();
    let json = serde_json::to_string(&config)?;
    std::fs::write(path, json)?;
}

// Load preprocessor for inference
pub fn load_preprocessor(path: &str) -> Result<DataPreprocessor> {
    let json = std::fs::read_to_string(path)?;
    let config: PreprocessingConfig = serde_json::from_str(&json)?;
    Ok(DataPreprocessor::new(config))
}
```

## 10. Validation

```rust
fn validate_normalization(original: &[f64], normalized: &[f64]) -> Result<()> {
    // Check that normalization was applied correctly
    // Verify no information loss
    // Check distribution properties
}
```
