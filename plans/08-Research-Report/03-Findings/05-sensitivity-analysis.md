# Sensitivity Analysis — Seed & Data Only (No GPU)

> **Hardware GPU table deleted.** Tree models (`RandomForest, GradientBoosting, XGBoost, LightGBM, ExtraTrees, SVM, KNN`) are CPU-only in `automl` (`smartcore`/`linfa`). No GPU path to test; sensitivity is seed + data.

## 1. Seed Sensitivity (Canonical)

All 7 `ModelType` × seeds `42,123,456,789,1011` → 35 ×10k = 350k games (cap via same 270k rule as ablation: prioritize winner ModelType first). Metric:

```
seed_sensitivity = σ(μ_seed) / mean(μ_seed) ×100%
```

- <10% + stable ranking → robust.
- >20% or flip → inconclusive winner, escalate to 50k games.
- Report per-model `mean, σ, CI` and overall ranking stability.

## 2. Data Sensitivity

| Factor | Levels | Gate |
|--------|--------|------|
| Train size | 1k,5k,10k,50k,100k | Score vs size; plateau = good |
| Label noise | 0%,5%,10%,20% flipped `a*` | Δmean with MWU |
| Sample bias | early-half vs late-half vs balanced | Δmean |

## 3. Hyperparameter Sensitivity

Grid per winner ModelType: `n_estimators {50,100,200}`, `max_depth {3,6,12}`, `lr {0.05,0.1,0.3}` — report `Sensitivity=(Max-Min)/Max`.

## 4. Evaluation Sensitivity (Already in 03-results.md)

CI width vs n: 10k → ~20 (σ512). No duplication of benchmark CI table.

## 5. Consolidated with Ablation

If ablation shows group redundancy, data-size plateau implies 27-dim over-parameterized — cross-ref `04-ablation-study.md` §1.2.
