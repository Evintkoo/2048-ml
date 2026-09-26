# Plan 03 — State Encoding: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Canonical CSV and metadata encoding are implemented; Parquet remains unsupported and feature-range policy awaits #034.

**Goal:** State the current implementation and evidence boundary for state encoding.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats CSV encoding as implemented with Parquet and one feature-range contract pending.** Root CSV storage uses the ordered 27-feature header plus `action`, with `row_index,game_id,move_index,score` in a separate aligned metadata file. The documented `/32768` scale can exceed one for larger accepted board tiles; the feature ticket #034 owns that decision.

> **Distinct focus:** This file covers storage encoding; `04-Encoding/01-state-vector.md` covers creation. The current implementation is in `src/data_pipeline.rs` and `src/state.rs`.

## 1. Canonical Header (27 + action)

```csv
grid_0,grid_1,grid_2,grid_3,grid_4,grid_5,grid_6,grid_7,grid_8,grid_9,grid_10,grid_11,grid_12,grid_13,grid_14,grid_15,empty_count,max_tile_log,monotonicity,smoothness,merges_available,score_normalized,adjacency_merge_score,corner_max,edge_tiles_occupied,col_worst,row_worst,action
0.0,0.00006,0.00012,0.00024,0.00098,0.00195,0.0039,0.0078,0.0156,0.03125,0.0625,0.0,0.0,0.0,0.0,0.0,0.5,0.73,0.8,0.72,0.125,0.15,0.03,0.06,0.5,0.02,0.03,2
```
28 cols: 27 features + `action: u8 0..3`. `score` raw is metadata sidecar, never a column.

## 2. Type Mapping — `TaskType::MultiClassification` Only

| Column | automl dtype | Range | Role |
|--------|--------------|-------|------|
| `grid_0..15` + 11 derived | Float64 | nominally [0,1] at the documented tile scale | feature |
| `score_normalized` idx21 | Float64 | nonnegative `log10(score+1)/6`; may exceed 1 | feature (never target) |
| `action` | UInt8 | 0..3 | **only label** `TaskType::MultiClassification` |

`TrainingConfig::new(TaskType::MultiClassification, "action")` — see `automl/src/training/config.rs:11`.

## 3. Creation vs Storage

- **Create:** `BoardStateMl::from_board` in `src/state.rs` → `[f64;27]`.
- **Store:** root writes the `27 + 1` canonical CSV and row-aligned metadata sidecar. Parquet creation and `read_parquet` are not implemented.

```rust
fn create_dataframe(states: &[[f64;27]], actions: &[u8]) -> DataFrame { /* 27 feats + action */ }
```

## 4. Validation

```rust
// Current validators require finite, nonnegative values; all features except
// score index 21 must also be <= 1. The >32768 tile-scale exception is pending #034.
```

## Implementation Record

- The training CSV uses the exact ordered 27-feature header plus `action` (28 columns). Game ID, move index, and raw score are stored in a separate row-aligned metadata CSV to preserve group provenance without adding model features.
- The CSV reader checks header, width, finite/nonnegative feature values, upper ranges except for feature index 21, and action labels. The plan's Parquet storage path is not implemented; CSV is the current supported format.

---

## Verification (definition of done)

1. `test -f plans/03-State/01-Board/03-state-encoding.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/03-State/01-Board/03-state-encoding.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/01-Board/03-state-encoding.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/01-Board/03-state-encoding.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/01-Board/03-state-encoding.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/01-Board/03-state-encoding.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/01-Board/03-state-encoding.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/01-Board/03-state-encoding.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
