# Feature Engineering — Appendix (11 Derived Features)

## 1. Purpose

Enumerate the fixed 11 derived features (indices 16–26) appended after `grid_0..15`. No pipeline platitudes.

## 2. Derived Features — One Formula Each

| # | Name | Formula (normalized to [0,1] unless noted) |
|---|------|---------------------------------------------|
| 16 | `empty_count` | `empty_cells as f64 / 16.0` |
| 17 | `max_tile_log` | `log2(max_tile) as f64 / 15.0` (max 32768 → 1.0) |
| 18 | `monotonicity` | `sum over rows+cols of log-monotonic (sum of Δ where monotonic) / max_possible` — higher = more monotonic |
| 19 | `smoothness` | `1.0 - (sum |log2(a)-log2(b)| over adjacent non-empty pairs / max_sum)` — higher = smoother |
| 20 | `merges_available` | `count_adjacent_equal_pairs as f64 / 16.0` |
| 21 | `score_normalized` | `log10(score as f64 + 1.0) / 6.0` |
| 22 | `adjacency_merge_score` | `sum of mergeable adjacent tile values / (16.0 * 2048.0)` |
| 23 | `corner_max` | `(max_tile in any corner ? 1.0 : 0.0)` *or* `max_corner_tile as f64 / 32768.0` |
| 24 | `edge_tiles_occupied` | `edge_non_empty as f64 / 12.0` |
| 25 | `col_worst` | `min(col_sums) as f64 / (32768.0 * 4.0)` |
| 26 | `row_worst` | `min(row_sums) as f64 / (32768.0 * 4.0)` |

Grid 0–15: `tile_value as f64 / 32768.0` (0 for empty).

## 3. Config — Concrete

```rust
pub struct FeatureEngineeringConfig { pub grid_divisor: f64 } // 32768.0
impl FeatureEngineeringConfig {
    pub fn to_features27(&self, board: &Board) -> [f64; 27] { /* compute above */ }
    pub fn monotonicity(&self, board: &Board) -> f64 { todo!() }
    // ... one fn per derived feature above
}
// SHAP / permutation importance: Future — post-training only, not MVP
```

Order is frozen per `02-Format/03-data-standard.md`. `DataPreprocessor` handles optional `StandardScaler` on train only (see `03-data-normalization.md`).
