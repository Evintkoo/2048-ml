# Sensitivity Analysis

> **Note:** This section defines the sensitivity analysis protocol. All "expected" values are hypotheses, not results. No findings are claimed.

## 2. Hyperparameter Sensitivity

### 2.1 Model Complexity

| Hyperparameter | Values Tested | Expected Impact |
|---------------|---------------|-----------------|
| Number of trees | 10, 50, 100, 200, 500 | To be determined |
| Maximum depth | 3, 5, 8, 12, unlimited | To be determined |
| Learning rate | 0.01, 0.05, 0.1, 0.2, 0.3 | To be determined |
| Minimum child weight | 1, 5, 10, 20 | To be determined |
| Subsample ratio | 0.5, 0.7, 0.8, 1.0 | To be determined |
| Column subsample | 0.5, 0.7, 0.8, 1.0 | To be determined |

### 2.2 Sensitivity Metrics

For each hyperparameter, sensitivity will be computed:
```
Sensitivity = (Score_max - Score_min) / Score_max × 100%
```

| Hyperparameter | Sensitivity | Interpretation |
|---------------|-------------|----------------|
| Number of trees | TBD | To be determined |
| Maximum depth | TBD | To be determined |
| Learning rate | TBD | To be determined |
| Min child weight | TBD | To be determined |
| Subsample ratio | TBD | To be determined |
| Column subsample | TBD | To be determined |

## 3. Seed Sensitivity

### 3.1 Multi-Seed Validation

Each model is evaluated across 5 different seeds: 42, 123, 456, 789, 1011.

**Seed sensitivity is measured as:**
```
Seed Sensitivity = σ(μ_1, μ_2, μ_3, μ_4, μ_5) / mean(μ_1, ..., μ_5) × 100%
```
where `μ_i` is the mean score for seed `i`.

### 3.2 Seed Sensitivity Results Template

| Model | Seed 42 | Seed 123 | Seed 456 | Seed 789 | Seed 1011 | Mean | Std Dev | Sensitivity |
|-------|---------|----------|----------|----------|-----------|------|---------|-------------|
| Random Forest | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |
| Gradient Boosting | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |
| XGBoost | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |
| LightGBM | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |
| ExtraTrees | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |
| SVM | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |
| KNN | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD% |

### 3.3 Seed Sensitivity Interpretation

- If all models show low seed sensitivity (<10%): Results are robust across different random initializations
- If some models show high seed sensitivity (>20%): Results should be reported with confidence intervals, not point estimates
- If the ranking changes across seeds: The winner determination is inconclusive and requires more games

## 4. Hardware Sensitivity

### 4.1 Hardware Configurations

| Configuration | CPU | GPU | RAM | Expected Impact |
|--------------|-----|-----|-----|-----------------|
| Low | 4 cores | None | 8GB | To be determined |
| Medium | 8 cores | None | 16GB | Standard configuration |
| High | 16 cores | 1 GPU | 32GB | To be determined |

### 4.2 Hardware Sensitivity Analysis

**Key questions:**
1. Does hardware affect model performance (score)? → Expected: No (deterministic with same seed)
2. Does hardware affect training time? → Expected: Yes
3. Does hardware affect convergence? → Expected: No (same model, same data, same seed)
4. Does hardware affect numerical precision? → Expected: Maybe (floating point differences)

**These are expectations based on theory, not confirmed results.**

## 5. Data Sensitivity

### 5.1 Training Data Size

| Training Samples | Expected Score | Interpretation |
|-----------------|----------------|----------------|
| 1,000 | TBD | To be determined |
| 5,000 | TBD | To be determined |
| 10,000 | TBD | To be determined |
| 50,000 | TBD | To be determined |
| 100,000 | TBD | To be determined |

### 5.2 Data Quality

| Data Quality Factor | Sensitivity | Interpretation |
|--------------------|-------------|----------------|
| Label noise (0%) | Baseline | Perfect labels |
| Label noise (5%) | TBD | To be determined |
| Label noise (10%) | TBD | To be determined |
| Label noise (20%) | TBD | To be determined |
| Feature noise (0%) | Baseline | Perfect features |
| Feature noise (5%) | TBD | To be determined |
| Feature noise (10%) | TBD | To be determined |

### 5.3 Sample Bias

| Bias Type | Description | Sensitivity |
|-----------|-------------|-------------|
| Early-game bias | Training data only from early-game states | TBD |
| Late-game bias | Training data only from late-game states | TBD |
| Balanced data | Training data from all game stages | TBD |

## 6. Evaluation Sensitivity

### 6.1 Number of Evaluation Games

| Games | Expected CI Width | Interpretation |
|-------|-------------------|----------------|
| 1,000 | ~64 points | Too wide for comparison |
| 5,000 | ~29 points | Marginal for comparison |
| 10,000 | ~20 points | Adequate for comparison |
| 50,000 | ~9 points | Excellent for comparison |
| 100,000 | ~6 points | Very precise |

**These CI width estimates are based on σ ≈ 512 heuristic baseline. Actual widths will depend on experimental data.**

### 6.2 Evaluation Game Distribution

All models must be evaluated on the **same set of game instances** to ensure fair comparison.

## 7. Sensitivity Analysis Results Template

```markdown
## Sensitivity Analysis Report

### Hyperparameter Sensitivity
- Learning rate sensitivity: {value}% (high/medium/low)
- Number of trees sensitivity: {value}% (high/medium/low)
- Maximum depth sensitivity: {value}% (high/medium/low)

### Seed Sensitivity
- Seed sensitivity: {value}% (low/medium/high)
- Does ranking change across seeds? {yes/no}
- If yes, which seeds produce different rankings?

### Hardware Sensitivity
- Does hardware affect model performance? {yes/no}
- Training time ratio (high/low): {ratio}x
- Numerical precision differences: {yes/no}

### Data Sensitivity
- Training data size sensitivity: {value}%
- Label noise sensitivity: {value}%
- Sample bias sensitivity: {value}%

### Evaluation Sensitivity
- Number of games sensitivity: {value}%
- CI width for 10,000 games: {width}
- Does ranking change with more games? {yes/no}

### Conclusion
- {Summary of sensitivity analysis}
- {Whether results are robust}
- {Whether additional experiments are needed}
```

## 8. Sensitivity Analysis for PhD Qualification

A PhD-level sensitivity analysis must answer:

1. **Are results robust to hyperparameter choices?** (Low sensitivity = good)
2. **Are results reproducible across seeds?** (Low sensitivity = good)
3. **Are results hardware-independent?** (Yes = good)
4. **Are results data-size independent?** (Plateau reached = good)
5. **Are results robust to label noise?** (Low sensitivity = good)
6. **Are results robust to sample bias?** (Low sensitivity = good)
7. **Is the ranking stable across evaluation sizes?** (Stable = good)

If the answer to any of these is "no," the results must be reported with appropriate caveats and limitations. All answers are pending experimentation.
