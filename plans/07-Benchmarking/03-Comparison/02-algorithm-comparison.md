# Algorithm Comparison — Random vs Heuristic vs automl-ML (3 Groups Only)

## 1. Purpose

Compare exactly **3 groups** on the same 10k-game benchmark. No Greedy / Search-Based hallucinations; no pre-data ranking diagram.

## 2. Groups

| Group | Agent | Expected mean | Source |
|-------|-------|---------------|--------|
| Random | `RandomAgent` (ChaCha8Rng) | ~128 | `06-Data/01-Collection/03-random-play-data.md` |
| Heuristic | Rule-based (monotonicity/corner/empty) | ~512 | Heuristic baseline |
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
