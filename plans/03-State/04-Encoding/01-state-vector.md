# State Vector Definition

## 1. Overview

The state vector is the numerical representation of the 2048 board that the automl model consumes.

## 2. Vector Dimensions

The state vector has **27 dimensions**:

| Index | Feature | Type | Range | Description |
|-------|---------|------|-------|-------------|
| 0-15 | grid_0 to grid_15 | f64 | [0, 1] | Tile values (normalized) |
| 16 | empty_count | f64 | [0, 1] | Empty cells / 16 |
| 17 | max_tile_log | f64 | [0, 1] | log2(max_tile) / 15 |
| 18 | monotonicity | f64 | [0, 1] | Monotonicity score |
| 19 | smoothness | f64 | [0, 1] | Smoothness score |
| 20 | merges_available | f64 | [0, 1] | Possible merges / 16 |
| 21 | score_normalized | f64 | [0, 1] | log10(score+1) / 6 |
| 22 | adjacency_merge_score | f64 | [0, 1] | Adjacent mergeable pairs |
| 23 | corner_max | f64 | [0, 1] | Corner tile value (max tile in corner) |
| 24 | edge_tiles_occupied | f64 | [0, 1] | Edge tiles occupied / 12 |
| 25 | col_worst | f64 | [0, 1] | Minimum column sum normalized |
| 26 | row_worst | f64 | [0, 1] | Minimum row sum normalized |

**Note**: `move_count_norm` was removed as it provides minimal predictive value and the new features (adjacency_merge_score, col_worst, row_worst) provide more strategically relevant information.

## 3. State Vector Creation

```rust
pub fn create_state_vector(board: &Board) -> [f64; 27] {
    let mut vec = [0.0f64; 27];
    
    // Grid values (indices 0-15)
    for i in 0..4 {
        for j in 0..4 {
            let val = board.grid[i][j].unwrap_or(0);
            vec[i * 4 + j] = val as f64 / 32768.0;
        }
    }
    
    // Derived features (indices 16-26): 11 features in canonical order
    vec[16] = board.empty_cells() as f64 / 16.0;
    vec[17] = (board.max_tile() as f64 + 1.0).log2() / 15.0;
    vec[18] = board.monotonicity();
    vec[19] = board.smoothness();
    vec[20] = board.possible_merges() as f64 / 16.0;
    vec[21] = (board.score as f64 + 1.0).log10() / 6.0;
    vec[22] = board.adjacency_merge_score();
    vec[23] = board.corner_tile() as f64 / 32768.0;
    vec[24] = board.edge_tiles_occupied() as f64 / 12.0;
    vec[25] = board.column_worst();
    vec[26] = board.row_worst();
    
    vec
}
```

## 4. State Vector Normalization

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType};

let preprocessor = DataPreprocessor::new(
    PreprocessingConfig::default()
        .with_scaler(ScalerType::Standard)  // Z-score (real API: with_scaler, not with_scaler_type)
);
let normalized = preprocessor.fit_transform(&[state_vector])?;
// For tree models (RandomForest/GradientBoosting) consider ScalerType::None — scaling is optional.
```

## 5. State Vector Serialization

```rust
fn state_vector_to_csv(vec: &[f64; 27], action: u8) -> String {
    // Canonical: 27 features + 1 action label = 28 columns; score is NOT a label (stored separately for analysis)
    let mut parts: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
    parts.push(action.to_string());
    parts.join(",")
}

// To JSON
fn state_vector_to_json(vec: &[f64; 27]) -> serde_json::Value {
    json!({ "state": vec })
}
```

## 6. State Vector Validation

```rust
pub fn validate_state_vector(vec: &[f64; 27]) -> Result<()> {
    for (i, &v) in vec.iter().enumerate() {
        if !v.is_finite() {
            return Err(format!("Feature {} is not finite: {}", i, v).into());
        }
        if v.is_nan() {
            return Err(format!("Feature {} is NaN", i).into());
        }
    }
    Ok(())
}
```

## 7. State Vector for Canonical Task — MultiClassification Only

```rust
// Canonical task: TaskType::MultiClassification — predict best action 0..3
// The 27-dim state vector maps to 4 logits → argmax; score is never a target
let state = create_state_vector(&board);
let action_class = direction_to_class(direction);  // 0-3 — the ONLY supervised label (u8)

// Score, if logged, is metadata for benchmarking only — not as `y`
let score_metadata: u64 = board.score; // for analysis only
```

## 8. Batch State Vector Creation

```rust
pub fn batch_state_vectors(boards: &[Board]) -> Vec<[f64; 27]> {
    boards.par_iter().map(|b| create_state_vector(b)).collect()
}
```
