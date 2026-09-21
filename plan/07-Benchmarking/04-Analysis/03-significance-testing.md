# Significance Testing

## 1. Purpose

Define the significance testing procedures for validating 2048 ML model improvements.

## 2. Significance Testing Framework

```mermaid
flowchart TD
    subgraph "Significance Testing"
        A[Define Hypotheses] --> B[Select Test]
        B --> C[Set α Level]
        C --> D[Collect Data]
        D --> E[Compute Test Statistic]
        E --> F[Calculate p-value]
        F --> G{Significant?}
        G -->|Yes| H[Conclude Improvement]
        G -->|No| I[No Evidence of Improvement]
    end
```

## 3. Test Selection Guide

```mermaid
graph TD
    A[Data Type] -->|Continuous| B{Normally Distributed?}
    B -->|Yes| C[Paired t-test]
    B -->|No| D[Mann-Whitney U Test]
    A -->|Paired| E[Paired t-test or Wilcoxon]
    A -->|Multiple Groups| F[ANOVA or Kruskal-Wallis]
    A -->|Categorical| G[Chi-squared Test]
```

## 4. Paired t-test Procedure

```mermaid
flowchart LR
    A[Collect Paired Scores] --> B[Calculate Differences]
    B --> C[Compute Mean Difference]
    C --> D[Compute Standard Error]
    D --> E[Calculate t-statistic]
    E --> F[Determine Degrees of Freedom]
    F --> G[Look Up p-value]
    G --> H[Make Decision]
```

## 5. Test Implementation

```rust
pub struct SignificanceTest {
    pub sample_a: Vec<u64>,
    pub sample_b: Vec<u64>,
    pub test_type: TestType,
    pub alpha: f64,
}

pub struct TestResult {
    pub test_statistic: f64,
    pub p_value: f64,
    pub is_significant: bool,
    pub confidence_interval: (f64, f64),
    pub effect_size: f64,
    pub conclusion: String,
}

impl SignificanceTest {
    pub fn run(&self) -> TestResult {
        // Implementation depends on test_type
        // Returns comprehensive test results
        TestResult {
            test_statistic: 0.0,
            p_value: 1.0,
            is_significant: false,
            confidence_interval: (0.0, 0.0),
            effect_size: 0.0,
            conclusion: String::new(),
        }
    }
}
```

## 6. Effect Size Calculation

```mermaid
flowchart LR
    A[Mean Difference] --> B[Pooled Standard Deviation]
    B --> C[Cohen's d]
    C --> D[Effect Size Category]
    D -->|d < 0.2| E[Negligible]
    D -->|0.2 ≤ d < 0.5| F[Small]
    D -->|0.5 ≤ d < 0.8| G[Medium]
    D -->|d ≥ 0.8| H[Large]
```

## 7. Power Analysis

```mermaid
graph TD
    A[Define Effect Size] --> B[Set α Level]
    B --> C[Set Power (1-β)]
    C --> D[Calculate Required Sample Size]
    D --> E[Run Tests with N Samples]
    E --> F{Achieved Power ≥ 0.8?}
    F -->|Yes| G[Valid Results]
    F -->|No| H[Increase Sample Size]
```

## 8. Multiple Testing Correction

```mermaid
flowchart TD
    A[Run N Tests] --> B[List All p-values]
    B --> C[Apply Bonferroni Correction]
    C --> D[Adjusted α = α/N]
    D --> E[Compare Each p-value]
    E --> F[Report Significant Results]
```

## 9. Reporting Standards

Every significance test report must include:
- Null and alternative hypotheses
- Test statistic value and type
- p-value with precision
- Effect size and interpretation
- Confidence interval
- Practical significance assessment

## 10. Decision Framework

```mermaid
graph TD
    A[p < 0.05 AND d ≥ 0.5] -->|Both met| B[Strong Evidence of Improvement]
    A -->|p < 0.05 only| C[Statistically Significant]
    A -->|d ≥ 0.5 only| D[Practically Significant]
    A -->|Neither| E[No Significant Evidence]
    
    style B fill:#9f9,stroke:#333
    style E fill:#f99,stroke:#333
```
