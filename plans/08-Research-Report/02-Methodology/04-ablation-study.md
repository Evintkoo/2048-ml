# Ablation Study

> **Note:** This section defines the ablation study protocol. All "expected" values are hypotheses, not results. No findings are claimed.

## 2. Ablation Design

Each ablation experiment removes one component from the full pipeline and measures the resulting mean score change. The baseline is the full pipeline with all 27 features and all hyperparameters optimized.

### 2.1 Ablation Categories

| Category | Component | Variation |
|----------|-----------|-----------|
| Feature Ablation | Individual features | Remove one feature at a time |
| Feature Group Ablation | Feature groups | Remove all features from one category |
| Model Ablation | Model types | Remove one model type |
| Hyperparameter Ablation | Hyperparameters | Fix to default values |
| Data Ablation | Training data | Reduce training samples |
| Evaluation Ablation | Evaluation metrics | Use different ranking criteria |

### 2.2 Feature Groups

| Feature Group | Features | Expected Importance | Rationale |
|---------------|----------|-------------------|-----------|
| Grid Values | `grid_0` through `grid_15` | To be determined | Directly encodes board state |
| Empty Count | `empty_count` | To be determined | Board flexibility indicator |
| Max Tile | `max_tile`, `max_tile_log` | To be determined | Progress indicator |
| Monotonicity | `mono_col_score`, `mono_row_score` | To be determined | Corner strategy alignment |
| Smoothness | `smooth_col_score`, `smooth_row_score` | To be determined | Tile gradient indicator |
| Merge Potential | `merge_count`, `merge_score`, `adjacency_merge_score` | To be determined | Immediate scoring opportunity |
| Column/Row Analysis | `col_worst`, `row_worst`, `col_worst_index`, `row_worst_index` | To be determined | Constraint identification |
| Movement Analysis | `up_score`, `down_score`, `left_score`, `right_score` | To be determined | Action selection signal |

**Note:** Expected importance values are hypotheses based on domain knowledge (Björk, 2014; Kishore et al., 2014). These will be validated empirically.

### 2.3 Feature Group Removal

Remove entire feature categories and measure the impact. Expected score drops are hypotheses, not results.

### 2.4 Model Ablation

Each candidate model is individually removed from the ensemble, and the resulting mean score is measured. Expected impacts are hypotheses.

### 2.5 Hyperparameter Ablation

Compare performance using default hyperparameters vs. HyperOptX-optimized hyperparameters for each model type.

### 2.6 Data Ablation

Vary the number of training samples and measure the effect on test performance. Expected patterns are hypotheses based on standard ML theory.

### 2.7 Evaluation Ablation

Compare different ranking criteria and statistical tests to determine which is most appropriate.

## 3. Expected Results Template

All expected results are hypotheses to be validated. No conclusions are drawn from these expectations.

### 3.1 Feature Importance Ranking

| Rank | Feature | Δ Score (baseline - ablation) | Importance |
|------|---------|-------------------------------|------------|
| 1 | TBD | TBD | To be determined |
| ... | ... | TBD | ... |
| 27 | TBD | TBD | To be determined |

### 3.2 Model Comparison

| Model | Mean Score | Median Score | Std Dev | Rank |
|-------|------------|--------------|---------|------|
| Random Forest | TBD | TBD | TBD | TBD |
| Gradient Boosting | TBD | TBD | TBD | TBD |
| XGBoost | TBD | TBD | TBD | TBD |
| LightGBM | TBD | TBD | TBD | TBD |
| ExtraTrees | TBD | TBD | TBD | TBD |
| SVM | TBD | TBD | TBD | TBD |
| KNN | TBD | TBD | TBD | TBD |

## 4. Statistical Rigor

All ablation experiments will use:
- **10,000 games per configuration** (minimum)
- **Fixed random seed** for reproducibility
- **Bonferroni correction** for multiple comparisons
- **Bootstrap 95% confidence intervals** on all reported metrics
- **Mann-Whitney U test** with p < 0.05 for significance

The ablation study will provide a systematic analysis of each component's contribution to model performance based on actual experimental data.
