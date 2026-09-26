# Plan 02 — Feature Engineering: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** The canonical 27-value encoder is implemented; this appendix is being aligned to its formulas and tile-range limits.

**Goal:** State the current implementation and evidence boundary for feature engineering.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats feature computation as implemented in `src/state.rs`.** This appendix is descriptive; its previous formulas conflicted with the canonical encoder and have been replaced. The encoder uses deterministic divisors and no configurable feature-engineering pipeline. Tiles above 32768 can exceed declared feature bounds.

## 1. Purpose

Enumerate the fixed 11 derived features (indices 16–26) appended after `grid_0..15`. No pipeline platitudes.

## 2. Derived Features — One Formula Each

| # | Name | Formula (normalized to [0,1] unless noted) |
|---|------|---------------------------------------------|
| 16 | `empty_count` | `empty_cells as f64 / 16.0` |
| 17 | `max_tile_log` | `0 if max==0 else log2(max_tile) / 15.0` |
| 18 | `monotonicity` | fraction of the 24 adjacent pairs that are equal or include an empty tile |
| 19 | `smoothness` | `1.0 / (1.0 + sum(abs_diff(adjacent tiles)) / 100.0)` |
| 20 | `merges_available` | unique cells participating in equal adjacent pairs `/ 16.0` |
| 21 | `score_normalized` | `log10(score as f64 + 1.0) / 6.0` |
| 22 | `adjacency_merge_score` | sum of equal adjacent tile values `/ (16.0 * 32768.0)` |
| 23 | `corner_max` | maximum corner tile `/ 32768.0` |
| 24 | `edge_tiles_occupied` | `edge_non_empty as f64 / 12.0` |
| 25 | `col_worst` | minimum column sum `/ 8192.0` |
| 26 | `row_worst` | minimum row sum `/ 8192.0` |

Grid 0–15: `tile_value as f64 / 32768.0` (0 for empty). Values above the documented 32768 tile scale can cause normalized features to exceed one.

## 3. Config — Concrete

```rust
// The live implementation is BoardStateMl::from_board in src/state.rs.
// No FeatureEngineeringConfig or fitted feature-generation stage exists.
// SHAP / permutation importance: Future — post-training only, not MVP
```

Order is frozen per `02-Format/03-data-standard.md`. `DataPreprocessor` handles optional `StandardScaler` on train only (see `03-data-normalization.md`).

## Implementation Record

- The formula table now matches `BoardStateMl::from_board` in `src/state.rs`; collection calls the same encoder.
- No standalone configurable feature-engineering stage or fitted transformer exists. SHAP/permutation importance is not implemented; tile-range contract remains open under the state plans.

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

- Resolve the accepted tile-range contract before treating all normalized features as bounded. Feature importance remains post-training research.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
