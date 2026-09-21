# Results

> **Status: PENDING EXPERIMENTATION**
>
> This section will be populated after all experiments are completed. All results are TBD until actual data is collected and analyzed.

## 2. Results Summary

All performance metrics, statistical test results, model rankings, and feature importance rankings are **to be determined** after experimentation.

## 3. Performance Data

### 3.1 Primary Results Table

All values in the table below are **TBD** and will be populated after experiments:

| Metric | Random Forest | Gradient Boosting | XGBoost | LightGBM | ExtraTrees | SVM | KNN | Heuristic | Random |
|--------|---------------|-------------------|---------|----------|------------|-----|-----|-----------|--------|
| Mean Score | TBD | TBD | TBD | TBD | TBD | TBD | TBD | ~512 | ~128 |
| Median Score | TBD | TBD | TBD | TBD | TBD | TBD | TBD | ~384 | ~64 |
| Std Dev | TBD | TBD | TBD | TBD | TBD | TBD | TBD | ~256 | ~96 |
| 95% CI | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] | [TBD, TBD] | [~128, ~1024] | [~32, ~256] |
| Games > Heuristic | TBD | TBD | TBD | TBD | TBD | TBD | TBD | — | — |
| Rank | TBD | TBD | TBD | TBD | TBD | TBD | TBD | — | — |
| Training Time | TBD | TBD | TBD | TBD | TBD | TBD | TBD | — | — |

### 3.2 Statistical Test Results

All statistical test results are **TBD** and will be computed after data collection:

| Comparison | Mann-Whitney U | p-value | Cohen's d | Significant? | CI 95% |
|------------|----------------|---------|-----------|-------------|--------|
| Best Model vs Random | TBD | TBD | TBD | TBD | [TBD, TBD] |
| Best Model vs Heuristic | TBD | TBD | TBD | TBD | [TBD, TBD] |
| Heuristic vs Random | TBD | TBD | TBD | TBD | [TBD, TBD] |
| All Models (Kruskal-Wallis) | TBD | TBD | TBD | TBD | [TBD, TBD] |

*All comparisons will use Bonferroni correction for multiple comparisons.*

## 4. Learning Curve Analysis

**To be determined.** Learning curves will be generated during training to assess convergence patterns. The expected pattern (based on prior work) is diminishing returns, but actual results may differ.

## 5. Score Distribution

**To be determined.** The score distribution will be analyzed after all 10,000+ games per model are completed.

## 6. Comparative Results

**To be determined.** Model comparison will be based on actual mean scores across ≥10,000 benchmark games.

## 7. Training Metrics

Training metrics (final loss, final accuracy, convergence epoch, training time) will be recorded during experiments.

## 8. Statistical Results

All statistical results (test statistics, p-values, effect sizes, confidence intervals) will be computed after data collection using the procedures defined in `02-Methodology/03-hypotheses.md`.

## 9. Results Visualization

Required figures will be generated after experiments:
1. Histogram — Score distribution for each model
2. Line Chart — Learning curves (score vs epoch)
3. Bar Chart — Mean score comparison with 95% CI error bars
4. Box Plot — Score distribution spread
5. Ranked Bar Chart — Models ranked by mean score

## 10. Results Summary Table

All summary values are **TBD** and will be populated after experimentation:

| Category | Value | Confidence |
|----------|-------|------------|
| Best Model | TBD | To be determined after experimentation |
| Mean Score | TBD | Bootstrap 95% CI |
| Median Score | TBD | To be determined after experimentation |
| Statistical Significance | TBD | Mann-Whitney U p-value |
| Effect Size | TBD | Cohen's d |
| Winner Rank | TBD | Mean score ranking |
| Training Time | TBD | Wall clock |

## 11. Data Quality

All results will be verified for:
- Reproducibility with seed
- Statistical validity (proper test assumptions)
- Absence of systematic bias (balanced evaluation)
- Proper data collection procedures
- Correct feature extraction (27-dimensional vector)
- Valid label generation (rollout-based, 100 sims/action)
- Bonferroni correction applied
- Bootstrap confidence intervals computed

## 12. Honest Reporting Standards

All results will be reported honestly, including:
- Null results (if no model significantly beats heuristic)
- Failed experiments (if training does not converge)
- Inconclusive results (if statistical significance is not achieved)
- Limitations and caveats

Results will NOT be:
- Selectively reported to show only positive outcomes
- Falsified or fabricated
- Misleadingly presented
- Overstated beyond what the data supports
