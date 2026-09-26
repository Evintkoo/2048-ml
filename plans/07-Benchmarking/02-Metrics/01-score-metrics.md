# Plan 01 — Score Metrics: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Implemented summaries include mean, sample SD, median, p90/p99, min/max, threshold counts, and bootstrap mean CI.

**Goal:** State the current implementation and evidence boundary for score metrics.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats score summary helpers as implemented with a narrower output set than the old example.** `src/evaluation.rs` computes mean, sample standard deviation, median, p90/p99, min/max, threshold counts, and bootstrap mean intervals. It does not emit p10/p25/p75/p95 or the proposed normalized-score utility.

## 1. Purpose

Define the score metrics used to evaluate and compare 2048 ML model performance.

## 2. Primary Score Metrics (Keep Full Distribution — Heavy-Tailed, No Truncation)

| Metric | Formula | Purpose |
|--------|---------|---------|
| Mean Score | Σ score / N | Primary ranking metric |
| Median Score | Middle value | Robust tiebreaker |
| Sample Std Dev | sample standard deviation (`N-1`) | Descriptive variability |
| Max / Min | max/min(scores) | Sample extrema (not a theoretical ceiling) |
| p50/p90/p99 | percentiles | Reported by current summary helper |
| games_above_2048/4096/8192 | counts | Threshold hit rates |

## 3. Implemented Score Summary API

```rust
pub struct ScoreSummary {
    pub n: usize,
    pub mean: f64,
    pub sample_std_dev: f64,
    pub median: f64,
    pub percentile_90: f64,
    pub percentile_99: f64,
    pub min: u64,
    pub max: u64,
    pub games_above_2048: usize,
    pub games_above_4096: usize,
    pub games_above_8192: usize,
    pub mean_ci_95: (f64, f64),
}

// `summarize_scores(scores, seed, bootstrap_replicates)` returns this summary
// or `None` for empty input / a zero bootstrap replicate count.
```

## 4. Score Normalization for Comparison (Duplicate Note)

> `normalize_score = log10(score+1)` is the same transform as `score_normalized = log10(score+1)/6.0` in `06-Data/02-Format/01-data-schema.md` and `04-Preprocessing/03-data-normalization.md` — the `/6.0` divisor just maps to [0,1]. Keep consistent.

```rust
// The normalized score feature is implemented only in src/state.rs.
// No score-normalization or inverse-transform report helper exists.
```

## 5. Reporting

Current reports include count, mean/median/sample SD, p90/p99, extrema, threshold counts, and bootstrap mean CI. Retain per-game scores so any additional predeclared percentiles can be computed; the current report does not emit a full percentile set.

## Implementation Record

- `ScoreSummary` and benchmark reports include count, mean, sample standard deviation, median, p90/p99, min/max, threshold rates, and bootstrap mean confidence interval. Full p10/p25/p75/p95 and a score-normalization report helper are not present.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/02-Metrics/01-score-metrics.md` exits 0.

## Open questions

- Add additional percentiles only if required by the predeclared report. Keep all per-game scores so summaries can be recomputed.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
