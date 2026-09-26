# Plan 01 — Model Evaluation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Seeded game-score and paired-score evaluation tooling exists; classification diagnostics and held-out model results remain pending.

**Goal:** State the current implementation and evidence boundary for model evaluation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats evaluation tooling as partially implemented and outcome claims as pending.** Root tooling can benchmark a saved policy on seeded games and compute score summaries and paired statistics. It does not yet emit 2048 case-study classification diagnostics or qualitative analyses, and no trained model has been evaluated on an adequate held-out corpus. The separate standard-dataset runner reports classifier metrics, but those results do not substitute for game-policy evaluation.

## 1. Purpose

Define the evaluation methodology for assessing the performance of the trained 2048 game machine learning model.

## 2. Evaluation Overview

The model evaluation process measures how well the trained model performs on unseen board states.

```mermaid
flowchart TD
    subgraph "Model Evaluation"
        Model[Trained Model]
        Test[Test Dataset]
        Model --> Predict[Make Predictions]
        Test --> Predict
        Predict --> Metrics[Calculate Metrics]
        Metrics --> Report[Evaluation Report]
        Report --> Decision{Interpret Results}
        Decision --> Archive[Report and archive evidence]
    end
    
    style Deploy fill:#e8f5e9
    style Retrain fill:#fff3e0
```

## 3. Evaluation Stages

```mermaid
flowchart TB
    Stage1[Stage 1: Quantitative Evaluation]
    Stage2[Stage 2: Qualitative Analysis]
    Stage3[Stage 3: Comparative Analysis]
    Stage4[Stage 4: Final Assessment]
    
    Stage1 --> Stage2
    Stage2 --> Stage3
    Stage3 --> Stage4
    
    style Stage1 fill:#e3f2fd
    style Stage4 fill:#e8f5e9
```

### 3.1 Quantitative Evaluation — Classification Only (No Regression)

```mermaid
flowchart LR
    Q1[Valid-Action Accuracy]
    Q2[F1 Macro]
    Q3[Confusion Matrix<br/>4x4]
    Q4[Precision / Recall<br/>per class]
    Q5[Mean Game Score<br/>benchmark — separate]

    Q1 --> Report[Evaluation Report]
    Q2 --> Report
    Q3 --> Report
    Q4 --> Report
    Q5 --> Report
```

> **Canonical task:** four-action classification from 17 features. The policy consumes four class probabilities and masks illegal moves. Game score is a downstream application outcome. No score regression metrics apply to the action target.

### 3.2 Qualitative Analysis — Concrete (2048-specific)

> **Not generic.** The model predicts actions 0–3 (up/down/left/right) from a 17-dim board state. Qualitative analysis must check whether predicted moves are **legal and strategically sensible**, not just statistically accurate.

**Analyses to run (with concrete definitions):**

1. **Invalid-move rate** — fraction of predictions where the predicted action does not change the board (no tiles move/merge). Compute on the test set by replaying each state through the 2048 engine: `invalid = predicted ∉ valid_actions(state)`. No threshold is predeclared; define this analysis before running it. The live policy masks invalid moves at inference.

2. **Corner-stuck analysis** — heuristic 2048 play keeps the max tile in a corner. A corner-conditioned breakdown could be a future analysis if the held-out sample and metric are defined before inspection; the current feature vector does not include max-tile position.

3. **Confusion matrix inspection (4×4)** — look for systematic confusions (e.g., `up↔down` or `left↔right` swaps) that correlate with vertical/horizontal board symmetry. A uniform error pattern suggests underfitting; a strong off-diagonal (e.g., 30% of `left` misclassified as `right`) suggests feature leakage or label noise from rollout sampling.

4. **Score-binned breakdown** — if run, define bins from the declared game protocol before examining outcomes. Report action diagnostics and score summaries per bin; do not use illustrative score cutoffs as acceptance gates.

```mermaid
flowchart TD
    Qual[Qualitative Analysis<br/>2048-concrete]
    Qual --> Invalid[Invalid-Move Rate<br/>predicted ∉ valid_actions<br/>report rate; no default threshold]
    Qual --> Corner[Corner-Stuck Analysis<br/>max in corner vs not<br/>report breakdown; no default threshold]
    Qual --> ConfMat[Confusion Matrix 4x4<br/>off-diagonal inspection]
    Qual --> Binned[Score-Binned Breakdown<br/>Low/Med/High games]
    
    Invalid --> Insights[Key Insights]
    Corner --> Insights
    ConfMat --> Insights
    Binned --> Insights
    Insights --> Action{Fail threshold?}
    Action -->|Predeclared criteria| Review[Review model and data]
    Action -->|Otherwise| Keep[Report outcome]
```

## 4. Evaluation Metrics — Classification + Game-Score Benchmark Only

```mermaid
flowchart TB
    subgraph "Evaluation Metrics"
        Classification[Classification Metrics<br/>on valid actions only]
        GameScore[Game-Score Benchmark<br/>downstream, not regression]
    end

    Classification --> Acc[Valid-Action Accuracy]
    Classification --> F1[F1 Macro]
    Classification --> Prec[Precision per class]
    Classification --> Rec[Recall per class]
    Classification --> CM[Confusion Matrix 4x4]

    GameScore --> MeanScore[Mean Game Score]
    GameScore --> MaxScore[Max / Median Score]
    GameScore --> Dist[Score Distribution]
```

> **No regression metrics** (R² / RMSE / MAE) and **no ranking metrics** (NDCG / MAP) — deleted. The model predicts actions, not scores or rankings. Game score is a downstream benchmark measured by running the policy in the simulator.

## 5. Test Dataset Structure — Planned Roles

```mermaid
flowchart TD
    Dataset[Test Dataset]
    Dataset --> Split1[Train Split<br/>70%]
    Dataset --> Split2[Validation Split<br/>15%]
    Dataset --> Split3[Test Split<br/>15%]
    
    Split1 --> Train[Train Model]
    Split2 --> Tune[Tune Hyperparameters]
    Split3 --> Eval[Evaluate Final Model]
    
    style Split3 fill:#ffcdd2
```

## 6. Evaluation Pipeline

```mermaid
sequenceDiagram
    participant Test as Test Data
    participant Model as ML Model
    participant Metrics as Metrics Calculator
    participant Report as Report Generator
    
    Test->>Model: Board states (17-dim)
    Model->>Metrics: Predicted actions (0-3)
    Metrics->>Report: Accuracy, F1 macro, Confusion Matrix
    Report->>User: Evaluation Results

    Note over Metrics: Valid-action accuracy, F1 macro, confusion matrix<br/>+ separate game-score benchmark (mean score)
```

## 7. Evaluation Files Location

All evaluation files are in `05-Model/04-Evaluation/`:

```mermaid
flowchart LR
    Dir[05-Model/04-Evaluation]
    Dir --> N01[01-model-evaluation.md]
    Dir --> N02[02-cross-validation.md]
    Dir --> N03[03-metrics.md]
```

## 8. Next Steps

1. Use the implemented game-grouped CV and chronological holdout with a retained labeled dataset.
2. Implement case-study classification diagnostics and validate them against known predictions.
3. Compare held-out policy outcomes with declared baselines under a predeclared protocol.

## Implementation Record

- Root tooling can benchmark a saved model on seeded games, collect score summaries, compare paired results, and compute bootstrap/nonparametric statistics. The standard-dataset diagnostic reports confusion/F1 for tabular tasks; the 2048 case-study path lacks those classification metrics, valid-action diagnostics, corner-state, and score-binned analyses.
- No trained model has been evaluated on an adequate held-out dataset. No performance gates are established by canonical scope.

---

## Verification (definition of done)

1. `test -f plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
6. `grep -q '^## Open questions$' plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
7. `grep -q '^## Later$' plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/05-Model/04-Evaluation/01-model-evaluation.md` exits 0.

## Open questions

- Implement and validate held-out classification diagnostics, then run them with a trained model on game-disjoint data. Predeclare any qualitative thresholds and retain analysis artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
