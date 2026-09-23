# State Vector — CANONICAL

> **Single source of truth for 27-dim creation.** All other state files cross-ref this. Storage format lives in `01-Board/03-state-encoding.md`.

## 1. Overview

Numerical representation of 2048 board+score for `TaskType::MultiClassification` `action 0..3`.

## 2. Dimensions — 27

| Idx | Feature | Formula | Range |
|-----|---------|---------|-------|
| 0..15 | grid_0..15 | `v as f64 / 32768.0` | [0,1] |
| 16 | empty_count | `/16` | [0,1] |
| 17 | max_tile_log | `log2(max+1)/15` | [0,1] |
| 18 | monotonicity | board.monotonicity() | [0,1] |
| 19 | smoothness | `1/(1+diff/100)` | [0,1] |
| 20 | merges_available | `/16` | [0,1] |
| 21 | score_normalized | `log10(score+1)/6` | [0,1] |
| 22 | adjacency_merge_score | `/16` | [0,1] |
| 23 | corner_max | `/32768` | [0,1] |
| 24 | edge_tiles_occupied | `/12` | [0,1] |
| 25 | col_worst | `/8192` | [0,1] |
| 26 | row_worst | `/8192` | [0,1] |

27 dims. No `move_count_norm`.

## 3. Canonical Creation — `create_state_vector:31`

```rust
pub fn create_state_vector(board: &Board) -> [f64; 27] {
    let mut vec = [0.0f64; 27];
    for i in 0..4 {
        for j in 0..4 {
            let val = board.grid[i][j].unwrap_or(0);
            vec[i * 4 + j] = val as f64 / 32768.0;
        }
    }
    vec[16] = board.empty_cells() as f64 / 16.0;
    vec[17] = (board.max_tile() as f64 + 1.0).log2() / 15.0;
    vec[18] = board.monotonicity();
    vec[19] = board.smoothness();
    vec[20] = board.possible_merges() as f64 / 16.0;
    vec[21] = (board.score as f64 + 1.0).log10() / 6.0;
    vec[22] = board.adjacency_merge_score();
    vec[23] = board.corner_tile() as f64 / 32768.0;
    vec[24] = board.edge_tiles_occupied() as f64 / 12.0;
    vec[25] = board.column_worst(); // min col sum / 8192
    vec[26] = board.row_worst();    // min row sum / 8192
    vec
}
pub fn batch_state_vectors(boards: &[Board]) -> Vec<[f64; 27]> {
    boards.iter().map(|b| create_state_vector(b)).collect()
}
```

## 4. Preprocessing — automl `DataPreprocessor`

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType};
let preprocessor = DataPreprocessor::new(
    PreprocessingConfig::default().with_scaler(ScalerType::Standard)
);
// Tree models (RandomForest, LightGBM, etc.) are scale-invariant — ScalerType::None acceptable.
let normalized = preprocessor.fit_transform(&df)?;
```

See `02-normalization.md` for deterministic vs fitted mapping.

## 5. Serialization — Storage (cross-ref)

Canonical storage header 27+action is in `01-Board/03-state-encoding.md`. Minimal:

```rust
fn state_vector_to_csv(vec: &[f64;27], action: u8) -> String {
    let mut p: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
    p.push(action.to_string()); p.join(",")
}
```

## 6. Validation — Finite + Per-Feature Range

```rust
pub fn validate_state_vector(vec: &[f64;27]) -> Result<()> {
    for (i, &v) in vec.iter().enumerate() {
        if !v.is_finite() { return Err(format!("{} not finite: {}", i, v).into()); }
        if v < 0.0 { return Err(format!("{} is negative: {}", i, v).into()); }
        // score_normalized (index 21) is not capped at 1.0.
        if i != 21 && v > 1.0 { return Err(format!("{} out of [0,1]: {}", i, v).into()); }
    }
    Ok(())
}
```

## 7. Task — MultiClassification Only

```rust
let state = create_state_vector(&board);
let action: u8 = 2; // label 0..3
// TrainingConfig::new(TaskType::MultiClassification, "action")
```
