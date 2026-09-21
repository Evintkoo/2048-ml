# Results Analysis

## 1. Purpose

Provide comprehensive analysis of benchmarking results for the 2048 ML system.

## 2. Results Analysis Framework

```mermaid
flowchart TD
    subgraph "Results Analysis Pipeline"
        A[Raw Benchmark Data] --> B[Data Cleaning]
        B --> C[Statistical Analysis]
        C --> D[Trend Identification]
        D --> E[Anomaly Detection]
        E --> F[Pattern Recognition]
        F --> G[Conclusions]
    end
    
    G --> H[Final Report]
```

## 3. Data Processing Pipeline — Keep Full Distribution (No Truncation)

```mermaid
flowchart LR
    A[Raw Scores] --> B[Validate Scores<br/>keep full distribution]
    B --> C[Normalize Data]
    C --> D[Aggregate Metrics]
    D --> E[Generate Insights]
    E --> F[Visualize Results]
    
    A -->|10000 games<br/>canonical 20k: 14k/3k/3k| B
    B -->|keep full distribution<br/>no truncation| C
    C -->|log transform| D
    D -->|mean, median, std, percentiles| E
    E -->|charts, tables| F
```

> **Do NOT discard high scores.** Game scores are **heavy-tailed signal** — high scores (top 1%) correspond to rare high-tile achievements and are the primary signal for max score / ceiling estimation. Removing top/bottom 1% discards the most valuable tail. **Keep full distribution**, report **percentiles (p50/p90/p95/p99/max)**, and do **no truncation** or outlier filtering on scores.

## 4. Key Analysis Metrics

| Analysis Area | Metric | Method |
|--------------|--------|--------|
| Performance | Mean score trend | Time series analysis |
| Stability | Score variance | Statistical tests |
| Improvement | Score delta over time | Regression analysis |
| Consistency | Win rate stability | Confidence intervals |

## 5. Trend Analysis

```mermaid
graph TD
    A[Training Epochs] --> B[Score per Epoch]
    B --> C[Plot Learning Curve]
    C --> D[Identify Convergence Point]
    D --> E[Analyze Plateau Region]
    E --> F[Determine Optimal Stopping]
```

## 6. Anomaly Detection — For Investigation Only, Not Filtering

```mermaid
flowchart TD
    A[Score Data] --> B[Calculate Z-Scores<br/>for investigation only]
    B --> C{Anomaly?}
    C -->|Yes| D[Investigate Cause<br/>log only]
    C -->|No| E[Normal Result]
    D --> F[Log and Flag]
    F --> G[Do NOT Filter<br/>keep full distribution]
```

> High scores are heavy-tailed signal, not anomalies to remove. Z-score flagging is for **diagnostics/logging only** — do **not** filter or truncate scores before aggregation (see §3). Keep full distribution; report percentiles.

## 7. Comparative Analysis

```mermaid
flowchart LR
    A[Current Run] --> B[Compare with Baselines]
    B --> C[Compute Differences]
    C --> D[Statistical Significance]
    D --> E[Identify Key Factors]
    E --> F[Actionable Insights]
```

## 8. Results Visualization

```mermaid
graph TD
    Histogram[Score Histogram] -->|distribution| V1[Distribution Analysis]
    LineChart[Score Over Time] -->|trend| V2[Trend Analysis]
    BoxPlot[Score Box Plot] -->|spread| V3[Spread Analysis]
    Scatter[Score vs Features] -->|correlation| V4[Correlation Analysis]
```

## 9. Analysis Conclusions

```rust
pub struct AnalysisConclusion {
    pub best_model: String,
    pub best_score: f64,
    pub confidence: f64,
    pub key_factors: Vec<String>,
    pub recommendations: Vec<String>,
    pub next_steps: Vec<String>,
}
```

## 10. Reporting

All analysis results are compiled into:
- Summary dashboard
- Detailed statistical report
- Visual charts and graphs
- Actionable recommendations
