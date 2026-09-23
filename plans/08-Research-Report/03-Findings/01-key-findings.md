# Key Findings — Findings Pipeline (30 Lines, No Placeholder Rows)

> **Status: PENDING.** This file is a pipeline spec: input = `evaluation_v1.parquet` + `statistical_tests.rs` outputs; output = IMRD §3 tables. No TBD rows; honesty = pipeline produces TBD until run.

## Input → Output Pipeline

```
benchmark-framework (10k games/model, seed 42, ScoreMetrics parquet)
  → statistical_tests.rs: mann_whitney, bootstrap_ci(10000 resamples), cohens_d, kruskal_wallis
  → ranking_analysis.py: ranked bar + gate table
  → IMRD §3 Results + this Findings doc
```

## Checklist (All Must Pass to Claim "Findings")

- [ ] `polars 0.46` schema validated (`game_id, score, model, seed`)
- [ ] `GroupKFold(game_id)` used in training (not TimeSeries)
- [ ] Bootstrap 95% CI on mean diff **excludes 0** for winner vs heuristic (~512) and vs runner-up
- [ ] Mann-Whitney U **p<0.05 Bonferroni** (k = #pairwise, ~21 for 7 models)
- [ ] Cohen's **d** reported (not just p; no fixed cutoff)
- [ ] Seed 42 primary; secondary seeds evaluated for sensitivity
- [ ] 27-dim ablation Δ reported with same gates
- [ ] No 8×8/ensemble/RL claim in findings

## Findings-Critical Tables (Shells, Not Fake Rows)

Findings narrative is generated; tables live in `01-IMRD/03-results.md` §3. Do not duplicate placeholder rows here.

## Relation to Hypotheses

F1–F3→RQ1/RQ2 framework validation, H1→2048 baseline comparison, H2→application ranking, H3→tuning comparison, and feature ablation→exploratory analysis. Each outcome is reported with its appropriate evidence. A null application result is a finding, not a failure.
