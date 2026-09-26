# Plan 02 — Algorithm Comparison: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Random/heuristic/model score runners and comparison statistics exist; the three-way study has not been run.

**Goal:** State the current implementation and evidence boundary for algorithm comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the comparison mechanism as available and outcomes as pending.** Random and heuristic baselines can be run with a common seed derivation, and saved models can be benchmarked. No trained-model three-way held-out result exists; baseline figures below are unverified planning estimates.

## 1. Purpose

Compare exactly **3 groups** under the same preregistered simulator and seed protocol. Choose game count from pilot variance and available budget. No Greedy / Search-Based candidates.

## 2. Groups

| Group | Agent | Expected mean | Source |
|-------|-------|---------------|--------|
| Random | uniform legal move policy | Not measured here | seeded baseline runner |
| Heuristic | rule-based policy | Not measured here | seeded baseline runner |

Do not substitute expected values or a small smoke run for retained benchmark results.
| AutoML model | selected four-class candidate | Not measured | saved-model benchmark runner |

## 3. Protocol

The proposed protocol uses the same declared game-seed set and simulator for each policy; the target game count must fit a documented budget. Compare score distributions under a predeclared procedure and report uncertainty. No results are available.

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

- Random and heuristic baselines and model benchmark commands exist with seedable simulator and comparison support. The three-way comparison remains pending; target means are not results.

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

- Run after training and model-selection steps produce an eligible policy. Record per-game scores, seed set, simulator settings, manifests, and analysis output.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
