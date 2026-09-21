# Model Evaluation

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
        Report --> Decision{Pass Threshold?}
        Decision --> |Yes| Deploy[Deploy Model]
        Decision --> |No| Retrain[Retrain Model]
        Retrain --> Model
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

> **Canonical metrics:** `TaskType::MultiClassification` (27-dim → 4 logits → `argmax` over actions 0–3). No R² / RMSE / MAE — the model does not predict scores.

### 3.2 Qualitative Analysis

```mermaid
flowchart TD
    Qual[Qualitative Analysis]
    Qual --> MoveAnalysis[Move Pattern Analysis]
    Qual --> ErrorAnalysis[Error Analysis]
    Qual --> CaseStudy[Case Studies]
    Qual --> Visualization[Visualization]
    
    MoveAnalysis --> Insights[Key Insights]
    ErrorAnalysis --> Insights
    CaseStudy --> Insights
    Visualization --> Insights
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

## 5. Test Dataset Structure

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
    
    Test->>Model: Board states (27-dim)
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

1. Perform cross-validation
2. Calculate detailed metrics
3. Compare with baseline models
