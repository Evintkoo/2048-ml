# Plan 03 — Comparison Metrics: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for comparison metrics.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Rank models by mean score. Statistical tests live in `01-Evaluation/01-benchmarking-framework.md §6.4`; this file is only the sort helper.

## 2. Ranking — Sort by Mean Score

```rust
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

For the 2048 case study, compare held-out `mean_score` with median, distribution, uncertainty, and practical-effect reporting. Significance uses the declared framework in `01-benchmarking-framework.md §6.4`; CV keeps `game_id` groups together and must not be interpreted as automatic independence.

> Deleted generic Framework A/B/C and duplicated test tables — see `01-benchmarking-framework.md §6.4` and `04-Analysis/03-significance-testing.md`.

## Implementation Record

- Score input comparison ranks by mean and computes matched sign-test or independent Mann–Whitney, Holm-adjusted p-values, bootstrap mean difference intervals, and Cohen's d. No full multi-model held-out ranking has been produced.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
