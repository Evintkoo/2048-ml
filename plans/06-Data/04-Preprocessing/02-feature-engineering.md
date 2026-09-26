# Plan 02 — Feature Engineering: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** The canonical 17-value encoder is implemented; derived metrics are excluded from model input.

**Goal:** State the current implementation and evidence boundary for feature engineering.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Plan 00 sets the model input to 17 values.** The current encoder adds no derived metrics beyond the 16 grid cells and normalized score. `HeuristicFeatures` separately computes five measurements for the heuristic policy. No configurable feature-engineering pipeline exists.

## 1. Purpose

Record heuristic measurements separately from canonical training inputs. Metrics are not appended to the 17-value model vector.

## 2. Heuristic Measurements (Not Model Features)

The baseline `HeuristicFeatures` in `src/state.rs` contains these measurements:

| Name | Formula |
|------|---------|
| `empty_fraction` | empty cells / 16 |
| `monotonicity` | fraction of adjacent pairs equal or containing an empty tile |
| `smoothness` | `1 / (1 + adjacent_abs_diff_sum / 100)` |
| `merges_fraction` | unique cells in equal adjacent pairs / 16 |
| `corner_max` | largest corner tile / 32768 |

These measurements guide the hand-authored baseline and are not used by
`ModelPolicy` as input. Other former derived-feature proposals are not
implemented.

## 3. Config — Concrete

```rust
// The live implementation is BoardStateMl::from_board in src/state.rs.
// No FeatureEngineeringConfig or fitted feature-generation stage exists.
// SHAP / permutation importance: Future — post-training only, not MVP
```

Canonical model order is 16 row-major cells and normalized score at index 16. Fitted preprocessing is not integrated in root training.

## Implementation Record

- The five heuristic measurements listed here match `HeuristicFeatures` in `src/state.rs`; they are separate from `BoardStateMl`.
- No configurable feature-engineering stage, fitted transformer, feature importance, or ablation study exists.

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

- A separate study of additional metrics requires an approved feature/analysis protocol. Feature importance remains post-training research.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
