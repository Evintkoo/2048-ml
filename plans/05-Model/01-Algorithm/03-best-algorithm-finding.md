# Plan 03 — Best Algorithm Finding: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** No winner is selected; the comparison depends on a predeclared protocol and adequate evaluation data.

**Goal:** State the current implementation and evidence boundary for best algorithm finding.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan records a pending result, not an inferred winner.** The comparison has not been run on a common adequate dataset or held-out game set. The canonical scope requires separating 2048 case-study evidence from general framework claims; it defines no universal score or classification thresholds.

## 1. Purpose

Document the model-selection result for the 2048 case study and explain its relationship to the Rust-native AutoML framework. “Best” means best under the declared case-study protocol, not globally best.

## 2. Selection Process

No best algorithm has been selected. A selection requires matched results from the candidate comparison ticket and a declared 2048 protocol.

```mermaid
flowchart TD
    subgraph "Selection Process"
        Start[Initial Candidates] --> Compare[Compare All Models]
        Compare --> Filter[Filter by Performance]
        Filter --> Validate[Validate on Test Set]
        Validate --> Select[Select Best Algorithm]
        Select --> Document[Document Findings]
    end
    
    Start --> |12 candidates| Compare
    Compare --> |Top 3| Filter
    Filter --> |1 model| Validate
    Validate --> |Confirmed| Select
```

## 3. Candidate Summary

```mermaid
flowchart TB
    subgraph "Performance Ranking"
        A[Best Algorithm] --> B[Second Place]
        B --> C[Third Place]
        C --> D[Eliminated]
        D --> E[Eliminated]
    end
    
    A --> |Highest Score| A1[Model Details]
    B --> |Strong Performance| B1[Model Details]
    C --> |Good Performance| C1[Model Details]
```

## 4. Final Model Architecture

```mermaid
flowchart TD
    Input[Input Features<br/>27 Dimensions] --> Preprocess[Preprocessing<br/>StandardScaler]
    Preprocess --> Model[Selected Model]
    Model --> Output[Output<br/>4 Actions]
    
    subgraph "Model Details"
        Model --> Params[Hyperparameters]
        Model --> Metrics[Performance Metrics]
    end
```

## 5. Performance Metrics — Classification + Game-Score Benchmark

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Mean Game Score (benchmark) | TBD — run after training | Predeclared practical comparison | Pending — case-study ranking |
| Valid-Action Accuracy | TBD — run after training | Report with uncertainty | Pending — diagnostic |
| F1 Macro (weighted, valid actions) | TBD — run after training | Report with uncertainty | Pending — diagnostic |
| Inference Time | TBD — run after training | Report hardware and distribution | Pending — framework/application efficiency |
| Training Time | TBD — run after training | Report configuration and budget | Pending — framework efficiency |

> **No R² / RMSE** — task is four-class classification; game score is a downstream case-study outcome, not a regression target. Values remain unmeasured until the comparison protocol runs.

### 5.1 Automl-Compatible Model Candidates

Since the automl framework does not implement neural networks, the candidate models are restricted to the following automl-compatible algorithms:

| Model | Category | Expected Performance |
|-------|----------|---------------------|
| RandomForest | Tree-based | Good baseline, robust to overfitting |
| GradientBoosting | Tree-based | Expected to perform best on tabular data |
| XGBoost | Tree-based | Strong gradient boosting, competitive accuracy |
| LightGBM | Tree-based | Fast training, high performance |
| CatBoost | Tree-based | Handles categorical features well |
| ExtraTrees | Tree-based | Randomized tree ensemble, good diversity |
| SVM | Linear/Kernel | Decent for smaller datasets |
| KNN | Instance-based | Simple baseline, distance-based |
| LogisticRegression | Linear | Baseline linear model |

### 5.2 Why Tree-Based Models Are Expected to Perform Best

The 2048 game data is **tabular** in nature — each sample consists of a fixed-length feature vector derived from the board state. Tree-based models (GradientBoosting, RandomForest, XGBoost) are expected to outperform other candidates for several reasons:

1. **Tabular data affinity**: Tree-based algorithms inherently excel on structured, tabular data with mixed feature types
2. **Non-linear relationships**: The relationship between board state features and optimal moves is highly non-linear; tree splits capture these interactions naturally
3. **Feature importance**: Tree models provide interpretable feature importance, which aligns with known 2048 heuristics (monotonicity, empty count, max tile)
4. **Robustness to scaling**: Unlike SVM or KNN, tree-based models do not require feature scaling
5. **Gradient boosting dominance**: Empirically, gradient boosting variants consistently rank among the top performers on tabular benchmark datasets

### 5.3 Evaluation Criteria — Canonical: Rank by Mean Game Score + Gates

> No weighted composite or acceptance thresholds are established by the canonical scope. Define the ranking rule, diagnostics, and uncertainty analysis before evaluating candidates.

**Ranking rule (primary):**

1. Run each candidate's policy in the simulator for ≥10,000 games and compute `mean_score`.
2. Apply the predeclared ranking rule to held-out game results and report uncertainty. Do not use a threshold absent from the approved protocol.
3. If two models tie within statistical noise (± 1 std), prefer higher Valid-Action Accuracy, then lower inference time.

**Gates (all must pass; otherwise the model is rejected regardless of rank):**

| Gate | Threshold | Measured on | Fail action |
|------|-----------|-------------|-------------|
| Mean Game Score | Predeclare | Held-out simulator games | Report with uncertainty |
| Valid-Action Accuracy | Diagnostic | Held-out labeled rows, protocol-defined valid-action handling | Report |
| F1 Macro | Diagnostic | Held-out labeled rows, four action classes | Report |
| Inference Time | ≤ 1ms | Mean per-prediction latency (informative, not rejecting unless >5ms) | Warn |

**What is NOT a gate:**

- `model_mean / heuristic_mean (≈512)` proximity ratio — optional single informative ratio only (e.g., 768/512 = 1.5×), never a gate.
- No `max_score / theoretical_limit (2^15=32768)` ratio — theoretical limit is open/unproven (see `02-Training/01-training-pipeline.md:5`).
- No regression metrics (R²/RMSE) — task is `TaskType::MultiClassification`.

**Actionable selection snippet (same APIs as `02-model-comparison.md:4.1`):**

```rust
use automl::{TrainingConfig, TaskType, ModelType, CVStrategy, cross_val_score};
use automl::training::{TrainEngine, CrossValidator};
use polars::prelude::*;
use ndarray::{Array1, Array2};

// 1) Cross-validated ranking (classification)
let candidates = vec![ModelType::GradientBoosting, ModelType::XGBoost, ModelType::RandomForest, ModelType::LightGBM];
let mut cv_scores: Vec<(ModelType, f64)> = Vec::new();
for m in &candidates {
    let cfg = TrainingConfig::new(TaskType::MultiClassification, "action")
        .with_model(m.clone()).with_cv(5).with_random_state(42);
    let splits = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
        .with_random_state(42)
        .split(x.nrows(), Some(&y), Some(&groups))?;
    cv_scores.push((m.clone(), score_splits(&cfg, &x, &y, &splits)?));
}
cv_scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());

// 2) Downstream game-score benchmark for top-3 only
for (model_type, _) in cv_scores.iter().take(3) {
    let cfg = TrainingConfig::new(TaskType::MultiClassification, "action")
        .with_model(model_type.clone()).with_random_state(42);
    let mut engine = TrainEngine::new(cfg);
    engine.fit(&df)?; // DataFrame → TrainingConfig → TrainEngine::fit(&df) (no epochs/gradients)
    let preds = engine.predict(&test_df)?; // → InferenceEngine::predict under the hood
    // Run policy in simulator ≥10k games → mean_game_score
    // Check gates: mean ≥512, accuracy ≥60%, F1 ≥0.55
}
```

## 6. Algorithm Rationale

Selection is justified by **(1) highest mean game score ≥512** plus **passing both gates (Acc≥60%, F1≥0.55)**. No weighted composite. Secondary factors (inference ≤1ms, training time, feature importance) are tie-breakers only and are documented alongside the primary ranking.

> **Scope aligned:** This file's job is **benchmark comparison + best algorithm selection**. Deployment/monitoring is out of scope (see `06-Data/` and `07-Benchmarking/` if needed).

## 7. Findings Summary

All findings are documented in the `05-Model/` directory:

```mermaid
flowchart LR
    Dir[05-Model]
    Dir --> Algo[01-Algorithm]
    Dir --> Training[02-Training]
    Dir --> Hyperopt[03-Hyperparameter-Optimization]
    Dir --> Eval[04-Evaluation]
    
    Algo --> N01[01-algorithm-research.md]
    Algo --> N02[02-model-comparison.md]
    Algo --> N03[03-best-algorithm-finding.md]
```

## 8. Next Steps

1. Integrate selected algorithm into training pipeline
2. Configure hyperparameters using `05-Model/03-Hyperparameter-Optimization/`
3. Evaluate performance using `05-Model/04-Evaluation/`
4. Begin full training cycle

## Implementation Record

- Selection protocol is documented, but the plan's comparison and ≥10k game-score evaluations have not been executed. The winner is intentionally left unselected and metric tables remain TBD.
- Status: research execution pending; do not select a model from architecture expectations or smoke runs.

---

## Verification (definition of done)

1. `test -f plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
6. `grep -q '^## Open questions$' plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
7. `grep -q '^## Later$' plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/05-Model/01-Algorithm/03-best-algorithm-finding.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
