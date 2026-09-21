# Score Metrics

## 1. Purpose

Define the score metrics used to evaluate and compare 2048 ML model performance.

## 2. Score Metrics Framework

```mermaid
flowchart TD
    subgraph "Score Metrics"
        subgraph "Central Tendency"
            Mean[Mean Score]
            Median[Median Score]
            Mode[Mode Score]
        end
        
        subgraph "Dispersion"
            StdDev[Standard Deviation]
            Variance[Variance]
            Range[Score Range]
        end
        
        subgraph "Percentiles"
            P10[P10 Score]
            P50[P50 Score]
            P90[P90 Score]
            P99[P99 Score]
        end
        
        Mean -->|summary| Report[Score Report]
        Median -->|summary| Report
        StdDev -->|spread| Report
        P10 -->|distribution| Report
        P99 -->|distribution| Report
    end
```

## 3. Primary Score Metrics

| Metric | Formula | Purpose |
|--------|---------|---------|
| Mean Score | Σ score / N | Average performance |
| Median Score | Middle value | Robust central tendency |
| Std Dev | √(Σ(x-μ)²/N) | Performance consistency |
| Max Score | max(scores) | Best capability |
| Min Score | min(scores) | Worst case |

## 4. Score Distribution Analysis

```mermaid
flowchart LR
    A[Raw Scores] --> B[Bin into Histogram]
    B --> C[Calculate Distribution Stats]
    C --> D[Normal Distribution Check]
    D --> E[Identify Outliers]
    E --> F[Generate Distribution Chart]
```

## 5. Key Score Thresholds

```mermaid
graph TD
    S0[Score = 0] -->|game over immediately| S100[Score < 100]
    S100 --> S2048[Score 100-2048]
    S2048 --> S4096[Score 2048-4096]
    S4096 --> S8192[Score 4096-8192]
    S8192 --> SHigh[Score > 8192]
    
    style S2048 fill:#9f9,stroke:#333
    style S4096 fill:#9f9,stroke:#333
    style S8192 fill:#f9f,stroke:#333
    style SHigh fill:#f0f,stroke:#333
```

## 6. Score Metrics Calculation

```rust
pub struct ScoreMetrics {
    pub mean: f64,
    pub median: u64,
    pub std_dev: f64,
    pub min: u64,
    pub max: u64,
    pub percentile_10: u64,
    pub percentile_25: u64,
    pub percentile_50: u64,
    pub percentile_75: u64,
    pub percentile_90: u64,
    pub percentile_95: u64,
    pub percentile_99: u64,
    pub games_above_2048: usize,
    pub games_above_4096: usize,
    pub games_above_8192: usize,
    pub total_games: usize,
}

impl ScoreMetrics {
    pub fn calculate(scores: &[u64]) -> ScoreMetrics {
        let n = scores.len();
        let sum: u64 = scores.iter().sum();
        let mean = sum as f64 / n as f64;
        let sorted = {
            let mut s = scores.to_vec();
            s.sort();
            s
        };
        let median = sorted[n / 2];
        let variance = scores.iter().map(|s| (*s as f64 - mean).powi(2)).sum::<f64>() / n as f64;
        let std_dev = variance.sqrt();
        
        ScoreMetrics {
            mean,
            median,
            std_dev,
            min: sorted[0],
            max: sorted[n - 1],
            percentile_10: sorted[n / 10],
            percentile_25: sorted[n / 4],
            percentile_50: sorted[n / 2],
            percentile_75: sorted[3 * n / 4],
            percentile_90: sorted[9 * n / 10],
            percentile_95: sorted[19 * n / 20],
            percentile_99: sorted[99 * n / 100],
            games_above_2048: scores.iter().filter(|s| **s >= 2048).count(),
            games_above_4096: scores.iter().filter(|s| **s >= 4096).count(),
            games_above_8192: scores.iter().filter(|s| **s >= 8192).count(),
            total_games: n,
        }
    }
}
```

## 7. Score Progress Tracking

```mermaid
flowchart LR
    A[Game Start] --> B[After Move 1]
    B --> C[After Move 10]
    C --> D[After Move 50]
    D --> E[Game End]
    
    subgraph Score Progression
        S0[Score: 0]
        S1[Score: variable]
        S2[Score: variable]
        S3[Score: final]
    end
```

## 8. Score Normalization for Comparison

```rust
fn normalize_score(score: u64) -> f64 {
    (score as f64 + 1.0).log10()
}

fn denormalize_score(normalized: f64) -> u64 {
    (10f64.powf(normalized) - 1.0) as u64
}
```

## 9. Metric Visualization

```mermaid
graph TD
    Histogram[Score Histogram] -->|visualize| Distribution
    BoxPlot[Score Box Plot] -->|visualize| Distribution
    LineChart[Score Over Time] -->|visualize| Distribution
    BarChart[Score Comparison] -->|visualize| Distribution
```

## 10. Reporting

Each score metrics report includes:
- Summary statistics table
- Score distribution analysis
- Threshold achievement rates
- Historical comparison
