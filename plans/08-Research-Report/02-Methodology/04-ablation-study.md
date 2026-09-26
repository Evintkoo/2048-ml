# Plan 04 — Ablation Study: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Candidate removals are mapped to the canonical 17-value state; feature-removal training and evaluation are not implemented.

**Goal:** State the current implementation and evidence boundary for ablation study.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**No ablation result is claimed.** The proposed matrix requires a feature-selection-aware training path, matched labeling/data splits, independent experimental units, and a compute budget. None has been completed.

> This is a candidate design, not an executable specification or preregistered protocol. CV grouping does not by itself establish independent game-level outcomes.

## 1. Matrix

### 1.1 Leave-One-Out (candidate design)

Candidate: remove one feature at a time, retrain using the same training data and declared model-selection protocol, then compare held-out outcomes. The feature list and model configuration must match the canonical encoder and supported four-class model set.

### 1.2 Group Removal (candidate design)

| Group | Features Removed | Rationale |
|-------|------------------|-----------|
| board | Cell indices `0..15` (cell positions defined by the row-major encoder) | Remove board observations as one block |
| score | Feature index `16` (`score_normalized`) | Remove current-score context |
| individual cell (optional finer analysis) | One selected index in `0..15` per ablation | Locate position-specific sensitivity; exploratory only |

The canonical model input contains no `empty_count`, `max_tile_log`, monotonicity, smoothness, merge, corner, or edge features. Such quantities may exist in agent heuristics, but they are not model columns and cannot be presented as feature ablations. Confirm index-to-coordinate names against `src/state.rs` before implementing the optional per-cell analysis.

### 1.3 Controls

- **Baseline full feature set** must use the same training samples, model, hyperparameters, and evaluation seeds as each removal run.
- **Validation:** keep all rows from a game in one fold; separately retain a held-out evaluation set. GroupKFold is group-disjoint, not chronological.
- **Labels:** rollout 100 sims/action; same label pipeline for all configs.

## 2. Cost Cap

No execution budget is approved. Estimate end-to-end runtime from a small measured pilot using the intended trained-policy evaluation path; the prior hours/game estimates and 270k cap are unsupported planning figures and are removed.

## 3. Statistical Gate per Ablation

Before analysis, define the inferential unit, matched-pair structure, multiplicity family, practical threshold, and suitable uncertainty method. Available comparison helpers do not yet provide a clustered paired ablation analysis.

## 4. Output

No output schema or ablation runner exists. Base future artifacts on actual CSV/manifest conventions; include configuration, data, seed, model, removed features, raw outcomes, and analysis provenance.

## Implementation Record

- No ablation configurations, artifact writer, or feature-removal evaluation pipeline are implemented. Candidate removals now map to the canonical 17-value state: board indices `0..15` and score index `16`; heuristic-only quantities are explicitly excluded as non-features. Feature removal, matched retraining/evaluation, output artifact schema, and compute budget remain unimplemented and require a pilot before scheduling.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/02-Methodology/04-ablation-study.md` exits 0.

## Open questions

- **Ablation work remains pending.** Resolve the canonical feature groups, split/label leakage boundary, inferential unit, method, and resource budget before running experiments.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
