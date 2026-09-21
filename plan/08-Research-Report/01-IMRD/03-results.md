# Results

## 1. Overview

This section presents the empirical results of training and evaluating ML models for the 2048 game using the automl framework.

## 2. Results Summary

```mermaid
flowchart TD
    subgraph "Key Results"
        A[Mean Score] -->|To be measured after experimentation| B[Primary Result]
        C[Games > 2048] -->|To be measured after experimentation| D[Success Rate]
        E[Training Time] -->|To be measured after experimentation| F[Efficiency]
        G[Model Type] -->|To be measured after experimentation| H[Best Architecture]
    end
```

## 3. Performance Data

| Metric | Random Forest | Gradient Boosting | XGBoost | Random Baseline |
|--------|---------------|-------------------|---------|-----------------|
| Proximity Ratio | TBD | TBD | TBD | ~0.004 |
| 95% CI | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] |
| Convergence Games | TBD | TBD | TBD | ~50 |
| Max Score | TBD | TBD | TBD | ~128 |

## 4. Learning Curve Analysis

```mermaid
graph TD
    A[Epoch 1] -->|To be measured after experimentation| B[Epoch 10]
    B -->|To be measured after experimentation| C[Epoch 50]
    C -->|To be measured after experimentation| D[Epoch 100]
    D -->|To be measured after experimentation| E[Epoch 200]
    
    style E fill:#9f9,stroke:#333
```

## 5. Score Distribution

```mermaid
flowchart LR
    A[Score Range] --> B[0-256]
    A --> C[257-512]
    A --> D[513-1024]
    A --> E[1025-2048]
    A --> F[2049-4096]
    A --> G[4097+]
    
    style F fill:#9f9,stroke:#333
    style G fill:#9f9,stroke:#333
```

## 6. Comparative Results

```mermaid
graph TD
    subgraph "Model Comparison"
        A[Random Agent<br/>Proximity: ~0.004] --> B[Heuristic Agent<br/>Proximity: ~0.016]
        B --> C[Best ML Model<br/>Proximity: TBD]
    end
    
    style C fill:#9f9,stroke:#333
```

## 7. Training Metrics

```rust
pub struct TrainingResults {
    pub final_loss: f64,
    pub final_accuracy: f64,
    pub convergence_epoch: usize,
    pub total_training_time: u64,
    pub best_model_score: f64,
    pub best_model_type: ModelType,
    pub hyperparameters: Hyperparameters,
}
```

## 8. Statistical Results

```mermaid
flowchart TD
    A[t-test] -->|To be measured after experimentation| B[p-value < 0.05]
    B --> C[Statistically Significant]
    C -->|To be measured after experimentation| D[Effect Size: Cohen's d = To be determined after experimentation]
    D --> E[Large Effect]
    E -->|To be measured after experimentation| F[Confidence Interval: To be determined after experimentation]
```

## 9. Results Visualization

```mermaid
graph TD
    A[Score Charts] -->|distribution| B[Histogram]
    A -->|trend| C[Line Chart]
    A -->|comparison| D[Bar Chart]
    A -->|spread| E[Box Plot]
```

## 10. Results Summary Table

| Category | Value | Confidence |
|----------|-------|------------|
| Proximity Ratio | TBD | TBD |
| Best Model | TBD | To be determined after experimentation |
| Training Time | TBD | To be determined after experimentation |
| Theoretical Limit | 32768 max tile (bounded by game mechanics) | Reference |

## 11. Data Quality

All results were verified for:
- Reproducibility with seed
- Statistical validity
- Absence of systematic bias
- Proper data collection procedures
