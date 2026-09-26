# Plan 02 — Algorithm Comparison: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Random/heuristic/model score runners and comparison statistics exist; a same-seed pilot comparison is retained, while the selected-model study remains pending.

**Goal:** State the current implementation and evidence boundary for algorithm comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the comparison mechanism as available, with exploratory results separated from the pending model-selection study.** Random and heuristic baselines and one fitted RandomForest pilot were each scored on seeds 84024–94023. A paired CLI analysis is retained in `reports/action-frequency/pilot-comparison.md`. The model was trained on a small 20-game corpus; it was not selected under a declared candidate comparison, so these outcomes are not a confirmatory three-way study.

## 1. Purpose

Compare exactly **3 groups** under the same preregistered simulator and seed protocol. Choose game count from pilot variance and available budget. No Greedy / Search-Based candidates.

## 2. Groups

| Group | Agent | Observed mean in pilot run | Source |
|-------|-------|---------------|--------|
| Random | uniform legal move policy | 1,094.12 | 10,000-game seeded baseline |
| Heuristic | rule-based policy | 8,056.23 | 10,000-game seeded baseline |
| AutoML model | fitted RandomForest pilot | 866.15 | 10,000-game run; not selected candidate |

These measured summaries describe this retained seed set, not universal targets. The small-corpus pilot is not evidence for selected-model performance.

## 3. Protocol

The confirmatory protocol must use the same declared game-seed set and simulator for each policy; the target game count must fit a documented budget. Compare score distributions under a predeclared procedure and report uncertainty. The retained pilot comparison is exploratory; its CLI uses an independent-sample bootstrap interval and Cohen's d even with paired seed outcomes, so those quantities are not paired-design uncertainty estimates.

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

- Random and heuristic baselines and one fitted RandomForest pilot share a 10,000-game seed set. Their paired exploratory analysis is retained separately. A model-selection study with an adequate corpus and a predeclared candidate/protocol remains pending.

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
