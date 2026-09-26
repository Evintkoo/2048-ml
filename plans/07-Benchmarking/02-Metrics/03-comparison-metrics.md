# Plan 03 — Comparison Metrics: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Score-file comparisons and adjusted statistics are implemented; no multi-model held-out ranking has been produced.

**Goal:** State the current implementation and evidence boundary for comparison metrics.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats pairwise score comparison tooling as implemented with ranking execution pending.** The benchmark compare command ranks inputs by mean score and emits paired sign or independent Mann–Whitney tests, Holm-adjusted p-values, bootstrap mean-difference intervals, and Cohen’s d. It does not establish a winner without comparable held-out game data.

## 1. Purpose

Rank models by mean score. Statistical tests live in `01-Evaluation/01-benchmarking-framework.md §6.4`; this file is only the sort helper.

## 2. Ranking — Sort by Mean Score

```rust
// Descriptive schema only; root CLI accepts score files rather than ModelRanking.
pub struct ModelRanking {
    pub model_name: String,
    pub mean_score: f64,
    pub median_score: u64,
    pub rank: usize,
    pub confidence_interval: (f64, f64),
    pub is_significantly_better: bool, // Mann-Whitney U p<0.05 Bonferroni (see framework §6.4)
}
impl ModelRanking {
    pub fn rank_models(models: &[ModelRanking]) -> Vec<ModelRanking> {
        let mut ranked = models.to_vec();
        ranked.sort_by(|a, b| b.mean_score.partial_cmp(&a.mean_score).unwrap());
        for (i, m) in ranked.iter_mut().enumerate() { m.rank = i + 1; }
        ranked
    }
}
```

For the 2048 case study, compare held-out mean score with distribution, uncertainty, and practical-effect reporting. Statistical tests must follow the declared protocol; CV folds are not independent game outcome populations by default.

> Deleted generic Framework A/B/C and duplicated test tables — see `01-benchmarking-framework.md §6.4` and `04-Analysis/03-significance-testing.md`.

## Implementation Record

- The compare command ranks score files by mean and computes matched sign-test or independent Mann–Whitney, Holm-adjusted p-values, bootstrap mean-difference intervals, and Cohen's d.
- No full multi-model held-out ranking has been produced; metric table inputs must represent comparable runs.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/02-Metrics/03-comparison-metrics.md` exits 0.

## Open questions

- Run comparisons only after confirming candidate inputs share the declared environment, seed design, game count, and output schema. Retain all raw score files and manifest hashes.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
