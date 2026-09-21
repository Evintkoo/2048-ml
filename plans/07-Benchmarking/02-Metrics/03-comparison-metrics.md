# Comparison Metrics

## 1. Purpose

Define the metrics and methods used to compare different models, algorithms, and configurations in the 2048 ML system.

## 2. Comparison Framework

```mermaid
flowchart TD
    subgraph "Comparison Framework"
        subgraph "Input"
            A[Model A Results]
            B[Model B Results]
            C[Model C Results]
        end
        
        subgraph "Metric Calculation"
            MA[Mean Score Comparison]
            MV[Median Score Comparison]
            MD[Statistical Significance]
            MR[Win Rate Comparison]
        end
        
        subgraph "Output"
            R[Rankings]
            T[Summary Tables]
            G[Graphs]
        end
        
        A --> MA
        B --> MA
        C --> MA
        MA --> R
        MV --> R
        MD --> R
        MR --> R
    end
```

## 3. Comparison Metrics Matrix

| Metric | Model A | Model B | Model C | Significance |
|--------|---------|---------|---------|-------------|
| Mean Score | 1024 | 2048 | 1536 | p < 0.05 |
| Median Score | 512 | 1024 | 768 | p < 0.05 |
| Std Dev | 512 | 256 | 384 | — |
| Win Rate | 25% | 50% | 35% | p < 0.01 |
| Games > 2048 | 10% | 30% | 20% | p < 0.05 |

## 4. Statistical Comparison Methods

```mermaid
flowchart TD
    A[Collect Score Samples] --> B[Check Normality]
    B -->|Normal| C[Wilcoxon Signed-Rank]
    B -->|Not Normal| D[Mann-Whitney U Test]
    C --> E[Calculate p-value]
    D --> E
    E --> F{Significant?}
    F -->|Yes| G[Reject Null Hypothesis]
    F -->|No| H[Fail to Reject Null]
    G --> I[Model A significantly better]
    H --> J[No significant difference]
```

## 5. Win Rate Analysis

```mermaid
flowchart LR
    A[Head-to-Head Matches] --> B[Count Wins]
    B --> C[Calculate Win Rates]
    C --> D[Statistical Testing]
    D --> E[Rank Models]
    
    A -->|Model A vs B| AB[Result]
    A -->|Model A vs C| AC[Result]
    A -->|Model B vs C| BC[Result]
```

## 6. Multi-Model Comparison

```mermaid
graph TD
    subgraph "Pairwise Comparisons"
        P1[Random vs Heuristic]
        P2[Heuristic vs Model]
        P3[Random vs Model]
        P4[Model v1 vs Model v2]
    end
    
    subgraph "Summary"
        R[Ranking Table]
        C[Confidence Intervals]
        V[Visualization]
    end
    
    P1 --> R
    P2 --> R
    P3 --> R
    P4 --> R
    R --> C
    C --> V
```

## 7. Ranking Methodology

```rust
pub struct ModelRanking {
    pub model_name: String,
    pub mean_score: f64,
    pub median_score: u64,
    pub win_rate: f64,
    pub games_above_2048: f64,
    pub rank: usize,
    pub confidence_interval: (f64, f64),
    pub is_significantly_better: bool,
}

impl ModelRanking {
    pub fn rank_models(models: &[ModelRanking]) -> Vec<ModelRanking> {
        let mut ranked = models.to_vec();
        ranked.sort_by(|a, b| b.mean_score.partial_cmp(&a.mean_score).unwrap());
        for (i, model) in ranked.iter_mut().enumerate() {
            model.rank = i + 1;
        }
        ranked
    }
}
```

## 8. Comparison Visualization

```mermaid
graph LR
    BarChart[Score Bar Chart] -->|compare| BoxPlot[Box Plot]
    BoxPlot -->|compare| Radar[Radar Chart]
    Radar -->|compare| Heatmap[Heatmap]
    Heatmap -->|compare| Table[Summary Table]
```

## 9. Cross-Validation Comparison

```mermaid
flowchart TD
    A[Split Data into Folds] --> B[Train on K-1 Folds]
    B --> C[Test on Held-Out Fold]
    C --> D[Record Metrics]
    D --> E{All Folds Done?}
    E -->|No| B
    E -->|Yes| F[Average Metrics Across Folds]
    F --> G[Compare Models]
```

## 10. Reporting Standards

All comparison reports must include:
- Summary ranking table with confidence intervals
- Statistical significance indicators
- Visual comparison charts
- Detailed metric breakdowns
- Recommendations for model selection
