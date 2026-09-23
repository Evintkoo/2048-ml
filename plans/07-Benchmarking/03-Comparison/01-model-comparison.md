# Model Comparison — automl ModelType Variants (Same Seed, 10k Games)

## 1. Purpose

Compare the real `automl` `ModelType` variants on the fixed 27→action `MultiClassification` task. No placeholder "Architecture A/B/C".

## 2. Variants Under Test

| `ModelType` variant | Family | Notes |
|---------------------|--------|-------|
| `RandomForest` | Tree ensemble | Baseline ensemble |
| `XGBoost` | Gradient boosting | Regularized boosting |
| `LightGBM` | Gradient boosting | Leaf-wise boosting |
| `ExtraTrees` | Tree ensemble | Extra-randomized |
| `GradientBoosting` | Gradient boosting | Sequential additive |
| (+ others if `automl` exposes them) | — | Add rows as engines evolve |

Add rows only for engines actually exposed by `automl` — no invented architectures.

## 3. Protocol — Identical Conditions

All variants: **10,000 games, same seed**, same engine, same 27-dim features, same 70/15/15 chronological `GroupKFold` split. Statistical tests per `01-Evaluation/01-benchmarking-framework.md §6.4` (Mann-Whitney U, bootstrap CI, Cohen's d, Bonferroni).

## 4. Performance Matrix — To Be Filled Post-Training

| Model | Mean | Median | Std | p vs #1 (Mann-Whitney) | Rank |
|-------|------|--------|-----|------------------------|------|
| RF | TBD | TBD | TBD | — | TBD |
| XGBoost | TBD | TBD | TBD | — | TBD |
| LightGBM | TBD | TBD | TBD | — | TBD |
| ExtraTrees | TBD | TBD | TBD | — | TBD |
| GBM | TBD | TBD | TBD | — | TBD |
| Heuristic | ~512 | ~384 | ~256 | ref | — |
| Random | ~128 | ~96 | ~96 | ref | — |

> Matrix is filled **post-training** — no pre-filled winners. Winner = highest held-out mean; use the pre-registered Mann-Whitney U/Holm comparison, bootstrap CI, and effect size to characterize uncertainty and practical magnitude. See `04-Analysis/03-significance-testing.md`.
