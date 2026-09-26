# Plan 02 — Statistical Analysis: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Descriptive summaries and limited pairwise inference are implemented; categorical metrics, several tests, and power planning are absent.

**Goal:** State the current implementation and evidence boundary for statistical analysis.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is partial.** Available inference consists of independent Mann–Whitney U or paired exact sign tests, percentile bootstrap intervals, Holm adjustment, and Cohen's d. These helpers do not constitute the broader analysis plan or a completed research report.

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
| Paired exact sign test | Paired comparison | Same ordered seed sequence; ties omitted |
| Wilcoxon signed-rank | Not implemented | Planned only |
| Kruskal-Wallis | Not implemented | Planned only |
| Chi-squared | Not implemented | Planned only |

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
    
    style D fill:#9f9,stroke:#363
```

Current score-summary intervals and pairwise mean-difference intervals use percentile bootstrap resampling. They are implemented helpers, not the illustrative normal-approximation struct above; clustering of paired games is not modeled by the comparison bootstrap.

## 7. Classification Analysis — No Regression (Actions 0–3 Only)

> **No regression.** Task is `TaskType::MultiClassification` (27-dim → 4 logits → `argmax`). Score is a downstream game-score benchmark, not a regression target. Do not fit `score` as `y` or report R² / RMSE / MSE.

Action classification metrics such as confusion matrices, per-class precision/recall/F1, and valid-action accuracy are not currently emitted by the evaluation reports. Game score is a downstream outcome, not a regression target for the supervised action classifier.

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

## Implementation Record

- `src/evaluation.rs` and comparison/report paths provide mean, sample standard deviation, median, p90/p99, min/max, threshold counts, bootstrap mean intervals, Mann–Whitney U, paired exact sign test, Holm-adjusted p-values, bootstrap mean-difference intervals, and Cohen's d. Kruskal–Wallis, Wilcoxon, categorical/classification metrics, explicit assumption diagnostics, power analysis, and a populated report are absent.

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

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/04-Analysis/02-statistical-analysis.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
