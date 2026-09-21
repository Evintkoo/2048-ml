# State Vector Definition

## 1. Overview

The state vector is the numerical representation of the 2048 board that the automl model consumes.

## 2. Vector Dimensions

The state vector has **25 dimensions**:

| Index | Feature | Type | Range | Description |
|-------|---------|------|-------|-------------|
| 0-15 | grid_0 to grid_15 | f64 | [0, 1] | Tile values (normalized) |
| 16 | empty_count | f64 | [0, 1] | Empty cells / 16 |
| 17 | max_tile_log | f64 | [0, 1] | log2(max_tile) / 13 |
| 18 | monotonicity | f64 | [0, 1] | Monotonicity score |
| 19 | smoothness | f64 | [0, 1] | Smoothness score |
| 20 | corner_value | f64 | [0, 1] | Corner tile value |
| 21 | available_moves | f64 | [0, 1] | Valid moves / 4 |
| 22 | merges_available | f64 | [0, 1] | Possible merges / 16 |
| 23 | score_normalized | f64 | [0, 1] | log10(score+1) / 6 |
| 24 | move_count_norm | f64 | [0, 1] | move_count / 1000 |

## 3. State Vector Creation

```rust
pub fn create_state_vector(board: &Board) -> [f64; 25] {
    let mut vec = [0.0f64; 25];
    
    // Grid values (indices 0-15)
    for i in 0..4 {
        for j in 0..4 {
            let val = board.grid[i][j].unwrap_or(0);
            vec[i * 4 + j] = val as f64 / 65536.0;  // Max tile is 2^16
        }
    }
    
    // Derived features (indices 16-24)
    vec[16] = board.empty_cells() as f64 / 16.0;
    vec[17] = (board.max_tile() as f64 + 1.0).log2() / 13.0;
    vec[18] = board.monotonicity();
    vec[19] = board.smoothness();
    vec[20] = board.corner_tile() as f64 / 65536.0;
    vec[21] = board.valid_moves().len() as f64 / 4.0;
    vec[22] = board.possible_merges() as f64 / 16.0;
    vec[23] = (board.score + 1) as f64 / 1_000_000.0;
    vec[24] = board.move_count as f64 / 1000.0;
    
    vec
}
```

## 4. State Vector Normalization

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType};

let preprocessor = DataPreprocessor::new(
    PreprocessingConfig::default()
        .with_scaler_type(ScalerType::Standard)  // Z-score
);
let normalized = preprocessor.fit_transform(&[state_vector])?;
```

## 5. State Vector Serialization

```rust
// To CSV row
fn state_vector_to_csv(vec: &[f64; 25], action: u8, score: u64) -> String {
    let mut parts: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
    parts.push(action.to_string());
    parts.push(score.to_string());
    parts.join(",")
}

// To JSON
fn state_vector_to_json(vec: &[f64; 25]) -> serde_json::Value {
    json!({ "state": vec })
}
```

## 6. State Vector Validation

```rust
pub fn validate_state_vector(vec: &[f64; 25]) -> Result<()> {
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

## 7. State Vector for Different Automl Tasks

```rust
// Regression task: predict final score
let state = create_state_vector(&board);
let target = final_score as f64;  // Regression target

// Classification task: predict best action
let action_class = direction_to_class(direction);  // 0-3

// The same state vector serves both tasks
```

## 8. Batch State Vector Creation

```rust
pub fn batch_state_vectors(boards: &[Board]) -> Vec<[f64; 25]> {
    boards.par_iter().map(|b| create_state_vector(b)).collect()
}
```
