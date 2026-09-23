# State Encoding — Storage Format

> **Distinct focus:** This file = **encoding for storage** (Parquet/CSV), `state-vector.md` = creation. Canonical impl in `04-Encoding/01-state-vector.md:31` `create_state_vector` — do not duplicate impl here.

## 1. Canonical Header (27 + action)

```csv
grid_0,grid_1,grid_2,grid_3,grid_4,grid_5,grid_6,grid_7,grid_8,grid_9,grid_10,grid_11,grid_12,grid_13,grid_14,grid_15,empty_count,max_tile_log,monotonicity,smoothness,merges_available,score_normalized,adjacency_merge_score,corner_max,edge_tiles_occupied,col_worst,row_worst,action
0.0,0.00006,0.00012,0.00024,0.00098,0.00195,0.0039,0.0078,0.0156,0.03125,0.0625,0.0,0.0,0.0,0.0,0.0,0.5,0.73,0.8,0.72,0.125,0.15,0.03,0.06,0.5,0.02,0.03,2
```
28 cols: 27 features + `action: u8 0..3`. `score` raw is metadata sidecar, never a column.

## 2. Type Mapping — `TaskType::MultiClassification` Only

| Column | automl dtype | Range | Role |
|--------|--------------|-------|------|
| `grid_0..15` + 11 derived | Float64 | [0,1] | feature |
| `score_normalized` idx21 | Float64 | [0,1] `log10/6` | feature (never target) |
| `action` | UInt8 | 0..3 | **only label** `TaskType::MultiClassification` |

`TrainingConfig::new(TaskType::MultiClassification, "action")` — see `automl/src/training/config.rs:11`.

## 3. Creation vs Storage

- **Create:** `04-Encoding/01-state-vector.md:31` `create_state_vector(board) -> [f64;27]` (canonical, with batch + validation).
- **Store:** this file — `polars` DataFrame `27 + 1` → Parquet/CSV. Load with `read_parquet`.

```rust
fn create_dataframe(states: &[[f64;27]], actions: &[u8]) -> DataFrame { /* 27 feats + action */ }
```

## 4. Validation

```rust
pub fn validate_encoding(v: &[f64;27]) -> Result<()> {
    if !v.iter().all(|x| x.is_finite() && (0.0..=1.0).contains(x)) { return Err(..); } Ok(())
}
```
