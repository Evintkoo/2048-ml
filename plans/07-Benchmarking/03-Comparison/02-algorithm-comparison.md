# Plan 02 — Algorithm Comparison: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Random/heuristic/model score runners and an exploratory seven-agent comparison exist; confirmatory model selection remains pending.

**Goal:** State the current implementation and evidence boundary for algorithm comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the comparison mechanism as available, with exploratory results separated from the pending model-selection study.** Random, heuristic, and five fitted candidate policies were scored on matched seeds 94024–104023; the paired comparison is retained in `reports/action-frequency/disjoint-seeds/README.md`. Models were each fit once on the small 20-game corpus. The result is exploratory, not a confirmatory algorithm comparison.

## 1. Purpose

Compare the five verified AutoML candidates with random and heuristic baselines under one simulator and matched seed set. Choose confirmatory game count from pilot variance and declared compute budget. Greedy/search agents are not in this scope.

## 2. Groups

| Rank | Agent | Observed mean | Source |
|-----:|-------|---------------:|--------|
| 1 | Heuristic | 8,096.70 | 10,000-game disjoint-seed baseline |
| 2 | Random | 1,086.52 | 10,000-game disjoint-seed baseline |
| 3 | NaiveBayes | 914.14 | One fitted candidate; 10,000 games |
| 4 | KNN | 887.35 | One fitted candidate; 10,000 games |
| 5 | RandomForest | 864.44 | One fitted candidate; 10,000 games |
| 6 | ExtraTrees | 837.15 | One fitted candidate; 10,000 games |
| 7 | AdaBoost | 765.62 | One fitted candidate; 10,000 games |

These summaries describe this retained seed set and these five fixed fits, not universal model targets. The small-corpus pilot is exploratory and does not estimate variation across training runs.

## 3. Protocol

The exploratory protocol used the same 10,000 game seeds and simulator for all seven policies. Its CLI reports paired-difference bootstrap intervals and Cohen's dz for matched outcomes. A confirmatory protocol must predeclare candidate selection, game count/precision rationale, and the held-out seed set; the current paired analysis does not make this pilot a confirmatory result.

```rust
pub struct AlgorithmComparison {
    pub algorithm_name: String, // "Random" | "Heuristic" | "automl-ML"
    pub mean_score: f64,
    pub median_score: u64,
    pub games_above_2048: f64,
}
```

> Deleted Greedy, Search-Based, and convergence-stage diagrams — out of scope.

## Implementation Record

- Random, heuristic, and five once-fitted candidate policies have retained 10,000-game runs on disjoint seeds 94024–104023. The seven-agent pairwise comparison applies Holm adjustment across 21 paired tests; the artifacts and limits are in `reports/action-frequency/disjoint-seeds/README.md`. Repeated fits, a scale-appropriate corpus, and predeclared confirmatory model selection remain pending.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/03-Comparison/02-algorithm-comparison.md` exits 0.

## Open questions

- Run after model selection produces an eligible policy under a predeclared protocol. The existing pilot is exploratory only; retain per-game scores, seed set, simulator settings, manifests, and analysis output for the confirmatory study.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
