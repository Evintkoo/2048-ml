# Ablation Study — Exact Matrix (27 LOO + 8 Groups, GroupKFold, Cost-Capped)

> **No TBD.** Every row maps to a `TrainingConfig` + `ScoreMetrics` evaluation (10k games, pre-registered MWU/Holm protocol, bootstrap CI, and effect size). Leakage control: `CrossValidator::GroupKFold` split on `game_id` (games are i.i.d. given seed; TimeSeries is not canonical).

## 1. Matrix

### 1.1 Leave-One-Out (27 configs)

For each canonical feature in (`grid_0..15, empty_count, max_tile_log, monotonicity, smoothness, merges_available, score_normalized, adjacency_merge_score, corner_max, edge_tiles_occupied, col_worst, row_worst`): train `MultiClassification` (0-3) without that feature; evaluate 10k games seed 42; report `Δ = mean_full - mean_minus_i` with CI.

### 1.2 Group Removal (8 configs)

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

> The 8 groups cover all 27 canonical features without inventing derived columns; the 16 raw grid columns remain one group.

### 1.3 Controls

- **Baseline full-27** trained once per `ModelType` (winner ModelType defines primary ablation; cheapest run precedes others if budget tight).
- **Cross-validation:** `TrainingConfig { cv_folds:5 }` with `CVStrategy::GroupKFold{ n_splits:5 }` using `game_id` column; no leakage across games.
- **Labels:** rollout 100 sims/action; same label pipeline for all configs.

## 2. Cost Cap

`35 configs (27+8) × 10k = 350k games`. Cap at **270k** by priority: run 8 groups first, then top-12 LOO by expected impact (empty, max, mono, merge groups); expand to full 27 only if `Δ` CI excludes 0 for any group. Evaluation is `~2h/10k` → 54h full, 40h capped (see `07-computational-budget.md`).

## 3. Statistical Gate per Ablation

Each removal vs full: report Mann-Whitney U p-values with Holm correction over the planned comparisons, bootstrap CI on Δ, and effect size. A feature is not declared necessary from a single threshold; conclusions distinguish statistical evidence from practical magnitude.

## 4. Output

`data/evaluation_data/ablation.parquet` with `config_id, removed_feature/group, mean, bootstrap_lo/hi, U, p_holm, d, significant`. Visualization: ranked Δ bar with CI — no "TBD" placeholder; TBD only until pipeline runs.
