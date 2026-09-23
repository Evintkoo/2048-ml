# Plan 02 — Algorithm Comparison: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for algorithm comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Compare exactly **3 groups** on the same 10k-game benchmark. No Greedy / Search-Based hallucinations; no pre-data ranking diagram.

## 2. Groups

| Group | Agent | Expected mean | Source |
|-------|-------|---------------|--------|
| Random | `RandomAgent` (ChaCha8Rng) | TBD (plan estimate ~128) | `06-Data/01-Collection/03-random-play-data.md` |
| Heuristic | Rule-based (monotonicity/corner/empty) | TBD (plan estimate ~512) | Heuristic baseline |

The ~128/~512 figures are unverified targets in the plans. A 20-game implementation smoke run is not used to replace those with research estimates; populate this comparison only from the declared held-out benchmark protocol.
| automl-ML | Best `ModelType` from `01-model-comparison.md` | TBD | Supervised 27→action |

## 3. Protocol

Each group: **10,000 games, same seed sequence**, same engine. Compare score distributions via the pre-registered Mann-Whitney U protocol with Holm correction. Report mean/median/std/p-value/Cohen's d/CI. Ranking by mean score only.

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

- Random and heuristic baselines and model benchmark commands exist, with common seeded engine and comparison support. The required 10k-per-group three-way comparison remains pending; target means are not evidence.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
