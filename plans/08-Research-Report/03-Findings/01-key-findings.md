# Key Findings

> **Status: PENDING EXPERIMENTATION**
>
> This section will be populated after all experiments are completed. No findings are claimed at this time. All values are TBD until actual data is collected and analyzed.

## 1. Finding Summary

| Finding | Evidence | Confidence |
|---------|----------|------------|
| automl trains competitive models | To be determined after benchmarking | Unknown |
| ML beats heuristic | To be determined (μ_best vs μ_heuristic) | Unknown |
| Results are reproducible | To be determined after multi-seed validation | Unknown |
| Convergence within N epochs | To be determined after learning curve analysis | Unknown |
| Feature space is Markov blanket | To be determined after ablation study | Unknown |
| PSPACE-hardness | Conjectured; formal proof incomplete | Theoretical only |
| PAC-learnability | Standard result applies; specific bounds TBD | Theoretical only |

## 2. Primary Finding: automl Effectiveness

**To be determined.** This finding will be assessed after all models are trained and evaluated across ≥10,000 benchmark games.

## 3. Finding 2: Performance Superiority

**To be determined.** The comparison between heuristic baseline (~512 mean score), random baseline (~128 mean score), and all ML models requires actual experimental data.

## 4. Finding 3: Reproducibility

**To be determined.** Multi-seed validation (seeds 42, 123, 456, 789, 1011) will be conducted to assess reproducibility.

## 5. Finding 4: Feature Importance

**To be determined.** Feature importance will be assessed through the ablation study (see `02-Methodology/04-ablation-study.md`). No feature importance ranking is claimed prior to experimentation.

## 6. Finding 5: Convergence Patterns

**To be determined.** Learning curve analysis will be conducted during training to determine convergence behavior.

## 7. Finding 6: Statistical Significance

**To be determined.** All statistical tests (Mann-Whitney U, Kruskal-Wallis, Wilcoxon) will be performed after data collection. Results are contingent on actual experimental outcomes.

## 8. Finding 7: Theoretical Validation

Theoretical contributions are documented in `00-theoretical-framework.md`:
- POMDP formulation: Formalized (Theorem 1, Theorem 2 are proven)
- PSPACE-hardness: Conjectured (Conjecture 3, not formally proven)
- Markov blanket: Conjectured (Proposition 2, requires empirical validation)
- PAC-learning bounds: Standard result applied (Theorem 4)

## 9. Honest Reporting Protocol

All findings will be reported honestly after experimentation:
- Null results will be reported if no model beats the heuristic baseline
- Failed experiments will be documented
- Inconclusive results will be stated as such
- Limitations and caveats will be disclosed

No results are fabricated, falsified, or selectively reported.

## 10. References

- Theoretical framework: `00-theoretical-framework.md`
- Ablation study: `02-Methodology/04-ablation-study.md`
- Methodology: `02-Methodology/`
- Limitations: `03-Findings/03-limitations.md`
