# Model Comparison

## 1. Purpose

Compare multiple machine learning algorithms to determine which performs best for the 2048 game prediction task.

## 2. Comparison Framework

All models will be evaluated using the same dataset, features, and metrics to ensure fair comparison.

```mermaid
flowchart TD
    subgraph "Comparison Framework"
        Data[Unified Dataset]
        
        Data --> RF[Random Forest]
        Data --> GBM[Gradient Boosting]
        Data --> XGB[XGBoost]
        Data --> LR[Logistic Regression]
        Data --> NN[Neural Network]
        Data --> SVM[Support Vector Machine]
        
        RF --> Metrics
        GBM --> Metrics
        XGB --> Metrics
        LR --> Metrics
        NN --> Metrics
        SVM --> Metrics
        
        Metrics[Evaluation Metrics<br/>R², RMSE, MAE, Accuracy]
    end
```

## 3. Models Under Comparison

### 3.1 Random Forest

```mermaid
flowchart TD
    RF[Random Forest]
    RF --> T1[Tree 1]
    RF --> T2[Tree 2]
    RF --> T3[Tree 3]
    RF --> TN[Tree N]
    RF --> Aggregator[Aggregation<br/>Voting/Averaging]
```

### 3.2 Gradient Boosting

```mermaid
flowchart LR
    G1[Tree 1] --> G2[Tree 2]
    G2 --> G3[Tree 3]
    G3 --> G4[Tree N]
    G4 --> Final[Final Prediction<br/>Sum of Residuals]
    
    style G1 fill:#fff3e0
    style G4 fill:#e1f5fe
    style Final fill:#e8f5e9
```

### 3.3 Neural Network

```mermaid
flowchart TD
    Input[Input: 25 features] --> Dense1[Dense 64, ReLU]
    Dense1 --> Dense2[Dense 32, ReLU]
    Dense2 --> Dense3[Dense 16, ReLU]
    Dense3 --> Output[Dense 4, Softmax]
```

## 4. Comparison Metrics

| Model | R² Score | RMSE | Training Time | Inference Time | Notes |
|-------|----------|------|---------------|----------------|-------|
| Random Forest | — | — | — | — | Baseline |
| Gradient Boosting | — | — | — | — | Strong candidate |
| XGBoost | — | — | — | — | High performance |
| Neural Network | — | — | — | — | Sequential learning |
| Logistic Regression | — | — | — | — | Linear baseline |
| SVM | — | — | — | — | Kernel-based |

## 5. Cross-Validation Results

```mermaid
flowchart TD
    CV[5-Fold Cross Validation]
    CV --> Fold1[Fold 1: Train/Test Split]
    CV --> Fold2[Fold 2: Train/Test Split]
    CV --> Fold3[Fold 3: Train/Test Split]
    CV --> Fold4[Fold 4: Train/Test Split]
    CV --> Fold5[Fold 5: Train/Test Split]
    
    Fold1 --> Results
    Fold2 --> Results
    Fold3 --> Results
    Fold4 --> Results
    Fold5 --> Results
    
    Results[Aggregated Results<br/>Mean ± Std Dev]
```

## 6. Key Findings

```mermaid
flowchart TD
    Best[Best Model Selected]
    Best --> Deploy[Deployment Pipeline]
    Best --> Iterate[Further Iteration]
    Deploy --> Monitor[Monitoring]
    Monitor --> Feedback[Feedback Loop]
    Feedback --> Iterate
```

## 7. Conclusion

Based on the comparison results, the best model will be selected and documented in `05-Model/01-Algorithm/03-best-algorithm-finding.md`.
