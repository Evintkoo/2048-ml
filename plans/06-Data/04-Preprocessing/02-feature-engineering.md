# Plan 02 — Feature Engineering: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for feature engineering.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

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

## Implementation Record

- Feature formulas are implemented once in `src/state.rs`; collection calls that encoder. The formula table in this ticket conflicts with the canonical State vector plan and code; canonical State vector and code are authoritative until this appendix is reconciled.
- SHAP/permutation importance is not implemented and remains post-training work.

---

## Verification (definition of done)

1. `test -f plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/04-Preprocessing/02-feature-engineering.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
