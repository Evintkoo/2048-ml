# Plan 03 — State Encoding: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** The v2 17-value CSV schema and metadata encoding are implemented and validated; Parquet is outside the current storage contract.

**Goal:** State the current implementation and evidence boundary for state encoding.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This storage ticket is complete for the canonical CSV contract.** The root writes 16 grid columns, `score_normalized`, and `action`; aligned provenance (`row_index,game_id,move_index,score`) remains in a separate metadata file. Feature values must be finite and nonnegative; values above one are valid. The emitted manifest labels this breaking data schema `2048-action-policy-v2`. Parquet is not required by the current case-study protocol.

> **Distinct focus:** This file covers storage encoding; `04-Encoding/01-state-vector.md` covers creation. The current implementation is in `src/data_pipeline.rs` and `src/state.rs`.

## 1. Canonical Header (17 + action)

```csv
grid_0,grid_1,grid_2,grid_3,grid_4,grid_5,grid_6,grid_7,grid_8,grid_9,grid_10,grid_11,grid_12,grid_13,grid_14,grid_15,score_normalized,action
0.0,0.00006,0.00012,0.00024,0.00098,0.00195,0.0039,0.0078,0.0156,0.03125,0.0625,0.0,0.0,0.0,0.0,0.0,0.000000,2
```
18 columns: 17 features plus `action: u8 0..3`. Raw `score` stays in the
metadata sidecar and is never a model input or target.

## 2. Type Mapping — `TaskType::MultiClassification` Only

| Column | automl dtype | Range | Role |
|--------|--------------|-------|------|
| `grid_0..15` | Float64 | nonnegative; may exceed 1 above tile 32768 | feature |
| `score_normalized` idx16 | Float64 | nonnegative `log10(score+1)/6`; may exceed 1 | feature (never target) |
| `action` | UInt8 | 0..3 | **only label** `TaskType::MultiClassification` |

`TrainingConfig::new(TaskType::MultiClassification, "action")` — see `automl/src/training/config.rs:11`.

## 3. Creation vs Storage

- **Create:** `BoardStateMl::from_board` in `src/state.rs` → `[f64;17]`.
- **Store:** root writes the 17-feature CSV plus action and row-aligned metadata sidecar. Parquet creation and `read_parquet` are not implemented.

```rust
fn create_dataframe(states: &[[f64;17]], actions: &[u8]) -> DataFrame { /* 17 feats + action */ }
```

## 4. Validation

```rust
// Current validators require finite, nonnegative values. Values above one
// are accepted for tiles above 32768 and scores above 1,000,000.
```

## Implementation Record

- The training CSV uses the exact ordered 17-feature header plus `action` (18 columns). Game ID, move index, and raw score are stored in a separate row-aligned metadata CSV to preserve group provenance without adding model features.
- The CSV reader checks header, width, finite/nonnegative feature values, and action labels; values above one are accepted. The existing large tile/score round-trip regression covers this contract.
- Collection checkpoints use schema version 2, and run manifests identify the dataset as `2048-action-policy-v2`; legacy headers are rejected. Parquet storage is outside the current case-study contract; CSV and its aligned metadata sidecar are the supported format.

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

- No required CSV/metadata encoding deliverable remains open. If future work requires Parquet, define it as a separate storage-format ticket with dependency, metadata alignment, and round-trip criteria.

## Later

- **No in-scope state-storage work remains for this ticket.** A future Parquet request requires a separately scoped ticket.
