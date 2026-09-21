# Metrics

## 1. Purpose

Define the evaluation metrics used to assess the 2048 game machine learning model performance.

## 2. Metrics Overview

The metrics are categorized into regression metrics (for score prediction) and classification metrics (for move prediction).

```mermaid
flowchart TD
    subgraph "Metrics Categories"
        Regression[Regression Metrics]
        Classification[Classification Metrics]
        Ranking[Ranking Metrics]
        Composite[Composite Metrics]
    end
    
    Regression --> R2[R² Score]
    Regression --> RMSE[RMSE]
    Regression --> MAE[MAE]
    Regression --> MAPE[MAPE]
    
    Classification --> Acc[Accuracy]
    Classification --> Prec[Precision]
    Classification --> Rec[Recall]
    Classification --> F1[F1 Score]
    
    Ranking --> NDCG[NDCG]
    Ranking --> MAP[MAP]
    
    Composite --> ModelScore[Model Score]
    Composite --> GameScore[Game Score]
```

## 3. Regression Metrics

```mermaid
flowchart TB
    subgraph "Regression Metrics"
        R2[R² Score<br/>Variance Explained]
        RMSE[RMSE<br/>Root Mean Squared Error]
        MAE[MAE<br/>Mean Absolute Error]
        MAPE[MAPE<br/>Mean Absolute % Error]
    end
    
    R2 --> Report[Metric Report]
    RMSE --> Report
    MAE --> Report
    MAPE --> Report
    
    style R2 fill:#e3f2fd
    style RMSE fill:#fff3e0
```

### 3.1 R² Score

```mermaid
flowchart LR
    SS_Total[Total Sum of Squares] --> |1 - | R2Calc[R² = 1 - SS_res/SS_tot]
    SS_Res[Residual Sum of Squares] --> R2Calc
    
    R2Calc --> Interpretation{Interpretation}
    Interpretation --> |R² ≥ 0.8| Good[Good Fit]
    Interpretation --> |0.5 ≤ R² < 0.8| Moderate[Moderate Fit]
    Interpretation --> |R² < 0.5| Poor[Poor Fit]
```

### 3.2 RMSE

```mermaid
flowchart TD
    Data[Data Points]
    Data --> Error[Calculate Errors]
    Error --> Square[Square Each Error]
    Square --> Mean[Calculate Mean]
    Mean --> Sqrt[Take Square Root]
    Sqrt --> RMSE[RMSE Result]
    
    style Sqrt fill:#e8f5e9
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
    subgraph "Composite Metrics"
        ModelPerf[Model Performance Score]
        GameScore[Game Score Metric]
    end
    
    ModelPerf --> R2W[Weight: R² (40%)]
    ModelPerf --> AccW[Weight: Accuracy (30%)]
    ModelPerf --> SpeedW[Weight: Speed (30%)]
    
    GameScore --> Score[Final Game Score]
    GameScore --> MaxTile[Max Tile Reached]
    GameScore --> Moves[Total Moves]
    
    style ModelPerf fill:#e3f2fd
    style GameScore fill:#fff3e0
```

## 6. Metric Targets

| Metric | Target | Weight | Category |
|--------|--------|--------|----------|
| R² Score | ≥ 0.8 | 40% | Regression |
| Accuracy | ≥ 85% | 30% | Classification |
| RMSE | ≤ 500 | 20% | Regression |
| Inference Speed | ≤ 1ms | 10% | Performance |

## 7. Metric Calculation Pipeline

```mermaid
sequenceDiagram
    participant Pred as Predictions
    participant Actual as Actual Values
    participant Calc as Metric Calculator
    participant Store as Metric Store
    
    Pred->>Calc: Predicted values
    Actual->>Calc: Ground truth values
    Calc->>Calc: Compute all metrics
    Calc->>Store: Store metrics
    Store --> Report[Generate Report]
    
    Note over Calc: R², RMSE, MAE<br/>Accuracy, Precision, Recall, F1
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
3. Compare against targets
