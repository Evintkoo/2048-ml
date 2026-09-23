# Plan 01 — Score Metrics: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for score metrics.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Define the score metrics used to evaluate and compare 2048 ML model performance.

## 2. Primary Score Metrics (Keep Full Distribution — Heavy-Tailed, No Truncation)

| Metric | Formula | Purpose |
|--------|---------|---------|
| Mean Score | Σ score / N | Primary ranking metric |
| Median Score | Middle value | Robust tiebreaker |
| Std Dev | √(Σ(x-μ)²/N) | Consistency (lower = tiebreak) |
| Max / Min | max/min(scores) | Ceiling / floor |
| p10/p25/p50/p75/p90/p95/p99 | percentiles | Tail reporting — keep all scores |
| games_above_2048/4096/8192 | counts | Threshold hit rates |

## 3. Score Metrics Calculation

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

## 4. Score Normalization for Comparison (Duplicate Note)

> `normalize_score = log10(score+1)` is the same transform as `score_normalized = log10(score+1)/6.0` in `06-Data/02-Format/01-data-schema.md` and `04-Preprocessing/03-data-normalization.md` — the `/6.0` divisor just maps to [0,1]. Keep consistent.

```rust
fn normalize_score(score: u64) -> f64 { (score as f64 + 1.0).log10() }
fn score_normalized(score: u64) -> f64 { (score as f64 + 1.0).log10() / 6.0 } // canonical feature 21
fn denormalize_score(normalized: f64) -> u64 { (10f64.powf(normalized) - 1.0) as u64 }
```

## 5. Reporting

Each report includes: summary table (mean/median/std/p99 etc.), threshold hit rates (`games_above_*`), full distribution percentiles — no truncation.

## Implementation Record

- `ScoreSummary` and benchmark reports include count, mean, sample standard deviation, median, p90/p99, min/max, threshold rates, and mean confidence interval. Full p10/p25/p75/p95 and a reusable score-normalization report are not emitted.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
