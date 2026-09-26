# Plan 01 — Results Analysis: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Initial UCI framework results and repeated-run comparison are retained; 2048 model rankings and trend/anomaly analysis remain pending.

**Goal:** State the current implementation and evidence boundary for results analysis.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is partial.** Seed-42 UCI results and a two-run repeatability comparison are now retained in `reports/framework_validation/`. They are one-split framework diagnostics; they do not populate the 2048 application result corpus and do not support trend analysis or framework-superiority claims.

## 1. Purpose

Provide comprehensive analysis of benchmarking results for the 2048 ML system.

## 2. Pipeline — Raw → Validate (Keep Full Distribution) → Aggregate → Insights

Raw benchmark CSVs → score summaries (mean, sample standard deviation, median, p90, p99, min, max, threshold counts, bootstrap mean CI) and optional pairwise comparison report. Reports summarize supplied inputs; they do not establish a ranking without a declared, populated experiment.

## 3. Data Processing Pipeline — Keep Full Distribution (No Truncation)

```mermaid
flowchart LR
    A[Supplied score CSVs] --> B[Validate and summarize]
    B --> C[Optional pairwise comparison]
    C --> D[CSV and JSON manifest]
```

Score analysis retains input observations and reports available quantiles (median, p90, p99, min, max). The implementation does not filter outliers. Do not describe the score distribution as heavy-tailed without analysis establishing that property; p10, p25, p75, and p95 are not currently emitted.

## 4. Key Analysis Metrics

| Analysis Area | Metric | Method |
|--------------|--------|--------|
| Performance | Mean score for supplied run | Descriptive summary |
| Uncertainty | Bootstrap mean interval | Percentile bootstrap |
| Pairwise comparison | Mean difference interval and test result | Comparison CLI; pairing depends on seed sequence |
| Trend / stability | Not implemented | Requires repeated, ordered runs and analysis |

## 5. Heavy-Tail §3 & Anomaly §6 — Kept as Core

Full scores are retained in source reports; no heavy-tail property or anomaly detector has been established. Any future anomaly investigation must preserve raw values and document its method rather than silently filtering observations.

## 6. Comparative Analysis

The comparison command selects an exact paired sign test when seed sequences match and independent Mann–Whitney U otherwise; it also reports a bootstrap mean-difference interval, Holm-adjusted p-values, and Cohen's d. No plan-scale trained-model comparison is available yet.

## 7. Analysis Conclusions

Conclusions and recommendations require completed, protocol-matched benchmark inputs. The report command does not infer causal factors or declare a best model.

## 8. Reporting

All analysis results are compiled into:
- Score summary CSV and JSON provenance manifest
- Pairwise comparison CSV and JSON provenance manifest
- Dashboard, charts, trend analysis, and evidence-based recommendations remain pending.

## Implementation Record

- `src/evaluation.rs` implements the score summary fields and bootstrap intervals; `src/main.rs` writes score reports and comparison CSV/JSON manifests. No populated plan-scale trained-model result corpus, trend analysis, anomaly investigation, plots, or defensible model conclusion exists.
- `reports/framework_validation/README.md` summarizes the initial fixed-split UCI matrix and repeatability finding. No confirmatory 2048 model corpus or ranking is available; no trend, anomaly, or causal conclusion is claimed.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/04-Analysis/01-results-analysis.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** A larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
