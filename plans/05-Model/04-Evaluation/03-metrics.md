# Plan 03 — Metrics: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Game-score summaries and uncertainty helpers exist; the planned classification metric suite is not implemented.

**Goal:** State the current implementation and evidence boundary for metrics.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats game-score summaries as implemented and classification metrics as pending.** `src/evaluation.rs` summarizes score distributions, bootstrap intervals, action frequencies, and paired/unpaired score comparisons. The root training/evaluation path does not currently calculate macro F1, confusion matrices, or valid-action classification accuracy.

## 1. Purpose

Define the evaluation metrics used to assess the 2048 game machine learning model performance.

## 2. Metrics Overview

The framework's primary validation metrics are task-appropriate classification quality, resource use, reproducibility, and artifact correctness on standard tabular tasks. In the 2048 case study, the task is **multi-class classification** (4 actions) with **game score** as a downstream application metric. Regression metrics (R², RMSE, MAE) are not used for the action target because the model predicts actions, not scores.

```mermaid
flowchart TD
    subgraph "Metrics Categories — Canonical"
        Primary[Framework Metrics]
        Secondary[Application Metrics]
        Tertiary[Resource and Reproducibility Metrics]
        Optional[Optional Analysis — Not a Gate]
    end
    
    Primary --> FrameworkQuality[Task quality, correctness, and reproducibility]
    
    Secondary --> Acc[Valid-Action Accuracy<br/>(on valid actions only)]
    Secondary --> F1[F1 Macro<br/>(across 4 classes)]
    Secondary --> Precision[Precision]
    Secondary --> Recall[Recall]
    
    Tertiary --> Speed[Inference Speed and Resource Use]
    Tertiary --> Consistency[Score Std Dev]
    Tertiary --> Ceiling[Max Tile Reached]

    Optional -.-> ProxOpt[Proximity = model/heuristic<br/>single ratio if reported — not a gate]
```

**Critical distinction**: Classification accuracy is measured ONLY on **valid actions** (actions that change the board state). Predicting an invalid move (no board change) is always wrong — the model should never select an invalid action. This is enforced by masking invalid actions before prediction.

```rust
pub fn masked_accuracy(
    predicted: u8, 
    actual: u8, 
    valid_actions: &[u8]
) -> bool {
    // Invalid predictions count as incorrect
    valid_actions.contains(&predicted) && predicted == actual
}
```

## 3. Regression Metrics — Not Used

> The task is four-action classification from 17 features. Regression metrics are not applicable to the action target. Game score is a downstream 2048 outcome and is summarized separately.

```mermaid
flowchart TB
    NotUsed[Regression Metrics — Not Used]
    NotUsed --> Action[Predict actions 0..3]
    Action --> Classify[Accuracy / F1 macro / Confusion Matrix]
    Classify --> GameBench[Game-Score Benchmark<br/>mean score downstream]

    style NotUsed fill:#ffcdd2
```

## 4. Classification Metrics

```mermaid
flowchart TB
    subgraph "Classification Metrics"
        CM[Confusion Matrix]
        CM --> Precision[Precision]
        CM --> Recall[Recall]
        CM --> F1[F1 Score]
        CM --> Acc[Accuracy]
    end
    
    Precision --> Report[Metric Report]
    Recall --> Report
    F1 --> Report
    Acc --> Report
```

### 4.1 Confusion Matrix

```mermaid
flowchart TD
    CM[Confusion Matrix<br/>4x4 Matrix<br/>Actions: Up, Down, Left, Right]
    CM --> TP[True Positives]
    CM --> FP[False Positives]
    CM --> FN[False Negatives]
    CM --> TN[True Negatives]
    
    CM --> AccCalc[Accuracy = (TP+TN)/Total]
    CM --> PrecCalc[Precision = TP/(TP+FP)]
    CM --> RecCalc[Recall = TP/(TP+FN)]
    CM --> F1Calc[F1 = 2*Prec*Rec/(Prec+Rec)]
```

### 4.2 F1 Score

```mermaid
flowchart LR
    Prec[Precision] --> F1[F1 Score]
    Rec[Recall] --> F1
    F1 --> |2 * P * R / (P + R)| Report[Report]
    
    style F1 fill:#e8f5e9
```

## 5. Composite Metrics

```mermaid
flowchart TD
    subgraph "Composite Metrics — Canonical (Ranking)"
        ModelPerf[2048 Case-Study Ranking: Mean Game Score<br/>application metric — uncertainty reported]
    end
    
    ModelPerf --> Rank[Rank by Mean Game Score<br/>protocol-defined; no default threshold]
    Rank --> Gate1[Diagnostic: report valid-action accuracy]
    Gate1 --> Gate2[Diagnostic: report F1 macro]
    Gate2 --> SpeedW[Informative: Inference Speed ≤1ms]
    
    style ModelPerf fill:#e3f2fd
```

No acceptance thresholds or composite ranking rule are defined by the canonical scope. Predeclare them in a case-study protocol before comparing policies.

## 6. Metric Targets — Canonical (Single Source of Truth)

| Metric | Target | Category | Gate? |
|--------|--------|----------|-------|
| **Mean Game Score** | Report distribution and uncertainty | 2048 case-study outcome | Protocol-defined |
| Valid-Action Accuracy | Not currently calculated | Classification diagnostic | Pending implementation |
| F1 Macro | Not currently calculated | Classification diagnostic | Pending implementation |
| Inference Speed | Measure on declared hardware | Resource metric | Informative |

> If a model/heuristic ratio is reported, define both evaluation populations and uncertainty; it is descriptive, not a default acceptance threshold.

Game-score summaries do not prove general AutoML quality. Classification metrics describe action-label prediction; game outcomes describe the 2048 case study.

## 7. Metric Calculation Pipeline

```mermaid
sequenceDiagram
    participant Pred as Predictions
    participant Actual as Actual Values
    participant Calc as Metric Calculator
    participant Store as Metric Store
    
    Pred->>Calc: Predicted actions (0-3)
    Actual->>Calc: True actions (0-3)
    Calc->>Calc: Compute classification metrics
    Calc->>Store: Store metrics
    Store --> Report[Generate Report]

    Note over Calc: Accuracy, Precision, Recall, F1 macro<br/>+ separate game-score benchmark (mean score)
```

## 8. Metrics Files Location

All metrics files are in `05-Model/04-Evaluation/`:

```mermaid
flowchart LR
    Dir[05-Model/04-Evaluation]
    Dir --> N01[01-model-evaluation.md]
    Dir --> N02[02-cross-validation.md]
    Dir --> N03[03-metrics.md]
```

## 9. Next Steps

1. Run cross-validation experiments
2. Collect metric results
3. Interpret against the predeclared protocol

## Implementation Record

- `src/evaluation.rs` implements descriptive score summaries, bootstrap intervals, action-frequency summaries, paired sign tests, Mann–Whitney U, Holm adjustment, and effect-size helpers.
- Macro F1, confusion matrix, valid-action accuracy, and held-out trained-model metric results are not implemented. No gates are defined by scope.

---

## Verification (definition of done)

1. `test -f plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
6. `grep -q '^## Open questions$' plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
7. `grep -q '^## Later$' plans/05-Model/04-Evaluation/03-metrics.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/05-Model/04-Evaluation/03-metrics.md` exits 0.

## Open questions

- Implement classification metrics with explicit class and valid-action handling; verify them on fixed examples before reporting results. Retain metric artifacts and predeclared evaluation protocol.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
