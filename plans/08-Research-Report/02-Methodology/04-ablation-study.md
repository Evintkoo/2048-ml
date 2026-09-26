# Plan 04 — Ablation Study: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Candidate ablation matrix is illustrative only; feature-removal training and evaluation are not implemented.

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
| grid | `grid_0..15` | Raw board |
| empty | `empty_count` | Flexibility |
| max | `max_tile_log` | Progress |
| monotonicity | `monotonicity` | Board shape |
| smoothness | `smoothness` | Board shape |
| merges | `merges_available, adjacency_merge_score` | Immediate merge opportunity |
| corner/edge | `corner_max, edge_tiles_occupied, col_worst, row_worst` | Placement and balance |
| score | `score_normalized` | Progress context |

> Reconcile this proposed grouping with the current 27-feature encoder before implementation.

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

- No ablation configurations, artifact writer, or feature-removal evaluation pipeline are implemented. The matrix is proposed only. Feature removal, matched retraining/evaluation, output artifact schema, and compute budget remain unimplemented and require a pilot before scheduling.

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
