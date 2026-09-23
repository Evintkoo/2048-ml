# Feature Extraction — Detailed Appendix

> **Canonical encoding:** `04-Encoding/01-state-vector.md:31` `create_state_vector`. This file is the **detailed appendix** for the 11 derived features — no duplication of the canonical impl.
> **Total dims:** 27 = 16 raw grid + 11 derived. Most features are in [0,1]; `score_normalized` is finite and non-negative but may exceed 1 for scores above 1,000,000.

## 1. Purpose

Derive 11 strategic features from the raw 4×4 board to augment the 16 grid values. See `01-board-state.md §3` for the struct; see `04-Encoding/01-state-vector.md` for the canonical assembly.

## 2. Raw Features (16 dims) — Reference Only

Flattened grid `0..15`, each `v as f64 / 32768.0`. Canonical impl handles this; not repeated here.

```
grid_0..grid_15 : u32 tile → f64 / 32768
```

## 3. Derived Features (11 dims) — Canonical Order

| Idx | Name | Formula | Divisor |
|-----|------|---------|---------|
| 16 | empty_count | `empty_cells() as f64 / 16.0` | 16 |
| 17 | max_tile_log | `(max_tile as f64 + 1.0).log2() / 15.0` | 15 (log2) |
| 18 | monotonicity | `monotonicity()` — monotonic rows/cols score | — |
| 19 | smoothness | `1.0 / (1.0 + diff_sum as f64 / 100.0)` | — |
| 20 | merges_available | `possible_merges() as f64 / 16.0` | 16 |
| 21 | score_normalized | `(score as f64 + 1.0).log10() / 6.0` | 6 (log10) |
| 22 | adjacency_merge_score | `sum_adjacent_equal / 16.0` | 16 |
| 23 | corner_max | `corner_tile() as f64 / 32768.0` | 32768 |
| 24 | edge_tiles_occupied | `edge_occupied as f64 / 12.0` | 12 |
| 25 | col_worst | `min_col_sum as f64 / 8192.0` | 8192 |
| 26 | row_worst | `min_row_sum as f64 / 8192.0` | 8192 |

### 3.1 Empty Tiles Count — idx 16

```rust
fn empty_count(grid: &[u32; 16]) -> f64 {
    grid.iter().filter(|&&v| v == 0).count() as f64 / 16.0
}
```
Rationale: mobility.

### 3.2 Max Tile (Log) — idx 17

```rust
fn max_tile_log(grid: &[u32; 16]) -> f64 {
    let max = grid.iter().copied().max().unwrap_or(0);
    if max > 0 { (max as f64 + 1.0).log2() / 15.0 } else { 0.0 }
}
```

### 3.3 Monotonicity — idx 18

```rust
fn monotonicity(grid: &[u32; 16]) -> f64 {
    (calculate_row_monotonicity(grid) + calculate_col_monotonicity(grid)) / 2.0
}
```

### 3.4 Smoothness — idx 19

```rust
fn smoothness(grid: &[u32; 16]) -> f64 {
    let mut diff_sum: i32 = 0;
    for i in 0..4 {
        for j in 0..4 {
            if j < 3 { diff_sum += (grid[i*4+j] as i32 - grid[i*4+j+1] as i32).abs(); }
            if i < 3 { diff_sum += (grid[i*4+j] as i32 - grid[(i+1)*4+j] as i32).abs(); }
        }
    }
    1.0 / (1.0 + diff_sum as f64 / 100.0)
}
```

### 3.5 Corner Max — idx 23

```rust
fn corner_max(board: &Board) -> f64 { board.corner_tile() as f64 / 32768.0 }
```

### 3.6 Edge Tiles Occupied — idx 24

```rust
fn edge_tiles_occupied(board: &Board) -> f64 { board.edge_tiles_occupied() as f64 / 12.0 }
```

### 3.7 Merges Available — idx 20

```rust
fn merges_available(grid: &[u32; 16]) -> f64 {
    // count adjacent equal pairs → /16
    count_adjacent_equal(grid) as f64 / 16.0
}
```

### 3.8 Score Normalized — idx 21

```rust
fn score_normalized(score: u64) -> f64 { (score as f64 + 1.0).log10() / 6.0 }
```
Only score feature; `score` is never a target.

### 3.9 Adjacency Merge Score — idx 22

```rust
fn adjacency_merge_score(board: &Board) -> f64 { board.adjacency_merge_score() } // sum_equal/16
```

### 3.10 Column Worst — idx 25

```rust
fn col_worst(board: &Board) -> f64 { board.column_worst() } // min col sum / 8192
```

### 3.11 Row Worst — idx 26

```rust
fn row_worst(board: &Board) -> f64 { board.row_worst() } // min row sum / 8192
```

## 4. Complete Feature Vector (27 dims)

```
[grid_0..grid_15, empty_count, max_tile_log, monotonicity, smoothness,
 merges_available, score_normalized, adjacency_merge_score, corner_max,
 edge_tiles_occupied, col_worst, row_worst]
  0..15            16          17           18            19
  20               21          22           23            24        25       26
```

## 5. Feature Importance Analysis

Post-training only — see §8. Runtime model exposes impurity/permutation importance; no pre-training numbers.

## 6. Feature Engineering Strategies

| Strategy | Features | Purpose |
|----------|----------|---------|
| Raw grid | 16 | board |
| Statistical | 5 | empties, max, merges, score, adjacency |
| Strategic | 6 | monotonicity, smoothness, corner, edge, col_worst, row_worst |
| Combined | 27 | canonical |
| Reduced | — | not MVP — no PCA |

## 7. Feature Normalization Pipeline

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType};
let preprocessor = DataPreprocessor::new(
    PreprocessingConfig::default().with_scaler(ScalerType::Standard)
);
let features = preprocessor.fit_transform(&raw_features)?;
 // Tree models may use ScalerType::None — see 04-Encoding/02-normalization.md
```

## 8. Feature Validation — Future Research, Not MVP

> **Future research, not MVP.** SHAP / ablation / importance ranking TBD post-training — 2 lines only; do not invent numbers.

Validate post-hoc via permutation / impurity importance; replace any hypothesized ranking with measured values after training.
