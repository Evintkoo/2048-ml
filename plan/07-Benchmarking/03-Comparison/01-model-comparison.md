# Model Comparison

## 1. Purpose

Compare different ML model architectures and training approaches for the 2048 game.

## 2. Comparison Architecture

```mermaid
flowchart TD
    subgraph "Model Comparison System"
        subgraph "Models Under Test"
            M1[Model Architecture A<br/>Neural Network v1]
            M2[Model Architecture B<br/>Neural Network v2]
            M3[Model Architecture C<br/>Ensemble Model]
        end
        
        subgraph "Common Evaluation"
            EE[Equal Environment<br/>Same Game Rules]
            ED[Equal Data<br/>Same Training Set]
            ER[Equal Metrics<br/>Same Evaluation Criteria]
        end
        
        subgraph "Results"
            R[Comparison Results]
            RK[Rankings]
            RC[Recommendations]
        end
        
        M1 --> EE
        M2 --> EE
        M3 --> EE
        EE --> ED
        ED --> ER
        ER --> R
        R --> RK
        RK --> RC
    end
```

## 3. Models Compared

| Model | Architecture | Parameters | Training Time |
|-------|-------------|-----------|---------------|
| Model A | 3-layer MLP | 10K | 2 hours |
| Model B | 4-layer CNN | 50K | 8 hours |
| Model C | ResNet variant | 100K | 16 hours |

## 4. Evaluation Process

```mermaid
flowchart LR
    A[Load Model A] --> B[Run 1000 Games]
    B --> C[Record Metrics]
    C --> D[Load Model B]
    D --> E[Run 1000 Games]
    E --> F[Record Metrics]
    F --> G[Load Model C]
    G --> H[Run 1000 Games]
    H --> I[Record Metrics]
    I --> J[Compare All Results]
    J --> K[Generate Report]
```

## 5. Head-to-Head Matches

```mermaid
graph TD
    A[Model A vs Model B] -->|Winner: Model B| Results[Win/Loss Matrix]
    B[Model A vs Model C] -->|Winner: Model C| Results
    C[Model B vs Model C] -->|Winner: Model C| Results
    
    Results --> Matrix[Win Rate Matrix]
    Matrix -->|Model C wins most| Ranking[Model C = Rank 1]
```

## 6. Model Performance Matrix

```mermaid
flowchart LR
    A[Score Metrics] -->|all models| M1[Comparison Matrix]
    B[Speed Metrics] --> M1
    C[Robustness Metrics] --> M1
    D[Convergence Metrics] --> M1
    M1 --> E[Final Ranking]
```

## 7. Model Selection Criteria

1. Highest mean score across all test games
2. Best score consistency (lowest variance)
3. Fastest inference time
4. Best generalization to unseen board states
5. Statistical significance of improvements

## 8. Results Summary

```rust
pub struct ModelComparisonResult {
    pub model_a: ModelResult,
    pub model_b: ModelResult,
    pub model_c: ModelResult,
    pub winner: String,
    pub significance: bool,
    pub confidence_level: f64,
}
```

## 9. Recommendations

Based on the comparison:
- Deploy the top-ranked model
- Investigate why lower-ranked models underperform
- Consider ensemble of top 2 models if improvement ≥ 5%
- Document findings for future iterations

## 10. Documentation

All comparison results are archived in:
- `07-Benchmarking/03-Comparison/01-model-comparison.md` (this file)
- Raw data stored in results database
- Charts generated for each comparison run
