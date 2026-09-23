# Comparison Metrics — Ranking Helper (30 Lines)

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
