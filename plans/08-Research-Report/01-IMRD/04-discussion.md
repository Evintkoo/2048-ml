# Discussion — Scope-Bound Interpretation (Pending Data)

> **Status: PENDING.** No conclusions drawn. This file defines how the primary Rust-native AutoML framework results and the downstream 2048 case-study results will be interpreted.

## 1. Primary Framework Interpretation

| Framework result | Interpretation |
|------------------|----------------|
| Capability and correctness criteria pass | The implemented architecture supports the declared workflow under the tested conditions |
| Reproducibility fails | The framework cannot support the corresponding reproducibility claim; investigate seed, parallelism, and serialization boundaries |
| Rust framework is faster or more resource-efficient | Report the measured trade-off and matched benchmark conditions; do not generalize beyond tested tasks |
| Framework performs differently from established baselines | Analyze algorithm coverage, preprocessing, search budget, implementation differences, and failure modes |
| Required capability fails | Report the failure as a primary framework result and block the affected 2048 conclusion |

## 2. 2048 Case-Study Interpretation (Tied to H1–H3)

| Hypothesis | Gate | If Pass | If Fail |
|------------|------|---------|---------|
| **H1**: Best automl mean > heuristic ~512 | MWU/Holm; report CI and effect size | automl competitive for 4×4 tabular task | Null: supervised 27-dim insufficient; report as limits of automl for sequential puzzle |
| **H2**: Alg differences exist | Kruskal-Wallis p<0.05 → Dunn post-hoc | Rank + name winner (RF/GB/XGB/LGBM/ET/SVM/KNN) | No rank claimed; all means equivalent |
| **H3**: Tuning helps | Wilcoxon signed-rank tuned vs default | Report tuned config; log HyperOptX TPE gain | Defaults sufficient; tuning not cost-effective |
| **Feature analysis**: 27-dim contributes | Ablation LOO + group removal, GroupKFold, uncertainty intervals | Feature groups ranked by Δmean | 27-dim redundant; smaller subset may be viable |

All framework gates come from the framework-validation protocol. Application gates come from `02-Methodology/03-hypotheses.md`; feature analysis is exploratory and does not establish a Markov blanket.

## 3. Implications (In-Scope Only)

### 3.1 For automl Framework
- If H1 passes: `TaskType::MultiClassification` + `TrainEngine` viable for 4×4 sequential decision via tabular features; `HyperOptX` justified.
- If H1 fails with narrow CI: automl correctly detects ceiling — supervised tabular ≠ search/RL for 2048; fallback to heuristic documented.
- HyperOptX sensitivity (H3) decides whether default `TrainingConfig` (max_depth 6, n_estimators 100, lr 0.1) suffices.

### 3.2 For 27-dim Feature Engineering
- Ablation (§04-ablation-study.md: 27 LOO + 8 groups: grid/empty/max/mono/smooth/merges/adjacency/corner) quantifies necessity.
- If empty/max/mono dominate, confirms corner-strategy encoding.
- If grid_0..15 dominate, suggests raw board suffices; if adjacency/merge groups dominate, confirms tactical signal.

### 3.3 For Evaluation Methodology
- Validates winner protocol (mean over 10k + pre-registered MWU/Holm + bootstrap CI/effect size) as replacement for single-split accuracy.
- GroupKFold by `game_id` shows leakage control for game data (not TimeSeries).

## 3. Seed Sensitivity Tie-In (Not Hardware)

Primary seed 42; secondary 123/456/789/1011 (§05-sensitivity-analysis.md). Sensitivity = σ(μ_i)/mean(μ_i), interpreted together with the declared experimental unit and uncertainty. If ranking flips across conditions, the case-study winner is inconclusive; do not automatically resolve this by increasing game count. Hardware and parallelism are framework variables and must be reported where they can affect reproducibility.

## 4. Limitations (In-Scope)

- 4×4 only; 27-dim fixed; supervised only; rollout labels noisy (100 sims/action).
- `automl` model list limited to what `ModelType` actually exposes (verify via `automl/src/training/config.rs`).
- Bootstrap CI width ~20 at n=10k (σ≈512) — tail events (rare >2000 scores) under-sampled.

## 5. Unexpected Findings Protocol

Document any: ranking shift across seeds, overfit (train acc >> test mean), feature Δ opposite expectation, invalid-move rate anomaly (see `02-insights.md`). Each gets a dedicated paragraph with exact numbers, not generic prose.

## 6. Conclusions (TBD)

Will state framework-validation outcomes, application winner (or null), F1–F3 and H1–H3 decisions with exact statistics where applicable, and reproducibility claims — only after data.

## 7. Future Work (Scope-Bound, 2 Lines Each — Not Core)

- **Larger boards / n×n theory:** n×n PSPACE claim belongs in Appendix only; not core IMRD.
- **Ensemble/RL/MCTS/DQN reproduction:** Out-of-scope; Appendix optional 15-line extended-baseline note, not required for thesis.
- **Information-theoretic / Markov blanket / PAC refinements:** Conjectures; Appendix if pursued.
