# Statistical Analysis — Descriptive Foundations (Distinct from Significance-Testing Winner Protocol)

> **Distinct focus vs `03-significance-testing.md`:** This file = descriptive foundations (distributions, CIs, test assumptions). `03-significance-testing.md` = winner determination protocol (adjusted α, ranking, power). No duplication — cross-ref there for ranking.

## 1. Purpose

Apply statistical methods to validate the performance of the 2048 ML system.

## 2. Statistical Analysis Framework

```mermaid
flowchart TD
    subgraph "Statistical Analysis"
        A[Collect Data] --> B[Descriptive Statistics]
        B --> C[Inferential Statistics]
        C --> D[Hypothesis Testing]
        D --> E[Confidence Intervals]
        E --> F[Regression Analysis]
        F --> G[Results Summary]
    end
```

## 3. Descriptive Statistics

```mermaid
flowchart LR
    A[Raw Scores] --> B[Calculate Mean]
    A --> C[Calculate Median]
    A --> D[Calculate Std Dev]
    A --> E[Calculate Percentiles]
    B --> F[Summary Statistics]
    C --> F
    D --> F
    E --> F
```

## 4. Inferential Statistics

| Test | Purpose | Assumption |
|------|---------|------------|
| Mann-Whitney U | Compare distributions | Independent samples |
| Wilcoxon | Paired comparison | Same seed sequence |
| Kruskal-Wallis | Compare multiple groups | Non-parametric |
| Chi-squared | Categorical comparison | Expected frequency ≥ 5 |

## 5. Hypothesis Testing Pipeline

```mermaid
flowchart TD
    A[Define Null Hypothesis H0] --> B[Define Alternative Hypothesis H1]
    B --> C[Select Significance Level α]
    C --> D[Collect Data]
    D --> E[Calculate Test Statistic]
    E --> F[Calculate p-value]
    F --> G{p-value < α?}
    G -->|Yes| H[Reject H0]
    G -->|No| I[Fail to Reject H0]
    H --> J[Report Finding]
    I --> K[Report No Significant Difference]
```

## 6. Confidence Intervals

```mermaid
graph TD
    A[Sample Mean] --> B[Calculate SE]
    B --> C[Determine t-critical]
    C --> D[Compute CI]
    D --> E[Report CI]
    
    style D fill:#9f9,stroke:#333
```

```rust
pub struct ConfidenceInterval {
    pub point_estimate: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,    // e.g., 0.95
    pub standard_error: f64,
}

impl ConfidenceInterval {
    pub fn new(mean: f64, std_err: f64, confidence: f64) -> ConfidenceInterval {
        let z_critical = match confidence {
            0.90 => 1.645,
            0.95 => 1.96,
            0.99 => 2.576,
            _ => 1.96,
        };
        ConfidenceInterval {
            point_estimate: mean,
            lower_bound: mean - z_critical * std_err,
            upper_bound: mean + z_critical * std_err,
            confidence_level: confidence,
            standard_error: std_err,
        }
    }
}
```

## 7. Classification Analysis — No Regression (Actions 0–3 Only)

> **No regression.** Task is `TaskType::MultiClassification` (27-dim → 4 logits → `argmax`). Score is a downstream game-score benchmark, not a regression target. Do not fit `score` as `y` or report R² / RMSE / MSE.

```mermaid
flowchart TD
    A[Feature Matrix X<br/>27-dim] --> B[Predict Actions<br/>4 logits → argmax 0..3]
    B --> C[Confusion Matrix<br/>4x4]
    C --> D[Per-Class Metrics<br/>Precision / Recall / F1]
    D --> E[Aggregate<br/>Valid-Action Accuracy + F1 Macro]
    E --> F[Game-Score Benchmark<br/>Mean score downstream — not R²]
```

## 8. Multiple Comparison Correction

```mermaid
flowchart LR
    A[Multiple Tests] --> B[Apply Correction]
    B -->|Bonferroni| C[Conservative]
    B -->|Holm| D[Slightly Less Conservative]
    B -->|Benjamini-Hochberg| E[Less Conservative]
    C --> F[Adjusted p-values]
    D --> F
    E --> F
```

## 9. Statistical Significance Reporting

All results must report:
- Test used and assumptions checked
- p-value and confidence interval
- Effect size
- Practical significance vs statistical significance

## 10. Analysis Validation

```mermaid
graph TD
    A[Check Assumptions] --> B[Run Tests]
    B --> C[Validate Results]
    C --> D{Results Valid?}
    D -->|Yes| E[Report]
    D -->|No| F[Use Non-parametric Alternative]
    F --> E
```
