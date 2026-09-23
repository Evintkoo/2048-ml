# Score as Feature

## 1. Concept

Score provides game-progression context as **one** feature (idx 21) of the 27-dim state. Scope is board+score only per `initial-plan.md`.

## 2. Score Features — What Is in the 27

> **Canonical 27 contains exactly one score feature: `score_normalized` at idx 21.** The struct below is **NOT IN 27** — do not add delta/avg/momentum as features.

```rust
// ❌ NOT IN 27 — do not add to feature vector
// pub struct ScoreFeatures {
//     pub current_score: f64,      // would be idx 21 if included — but ONLY log10/6 is in 27
//     pub score_delta: f64,        // NOT IN 27
//     pub avg_score_per_move: f64, // NOT IN 27
//     pub score_momentum: f64,     // NOT IN 27
// }
// If history-based score features are ever explored, mark `Future Research — Not MVP, Not in Canonical 27`.

fn normalize_score(score: u64) -> f64 { (score as f64 + 1.0).log10() / 6.0 }
```

## 3. Score in Feature Vector — idx 21 Only

```
[grid_0..grid_15, empty_count(16), max_tile_log(17), monotonicity(18), smoothness(19),
 merges_available(20), score_normalized(21) ← log10/6 ONLY, adjacency(22), corner_max(23),
 edge_occupied(24), col_worst(25), row_worst(26)]
```
26 = row_worst. No move_count_norm, no score_delta.

## 4. Score vs Target — Score Is Never `y`

| Aspect | Score as Feature | Score as Target |
|--------|------------------|-----------------|
| Purpose | idx 21 of X | **Not applicable — no score target** |
| Training role | `X[21]` | **Not `y`** — `y = action: u8` (`TaskType::MultiClassification`) |
| Norm | `log10(score+1)/6` | N/A |

## 5. Score Progression — Analysis Only (Not a Feature)

> Sequence score history is **not in the training vector**. May be logged per game for analysis, grouped by `game_id` for `GroupKFold` leakage checks — not fed as `X`.

## 6. Score-Based Heuristics (Analysis Only)

```rust
let high_score_samples: Vec<_> = all_samples.iter().filter(|s| s.score > 2048).collect();
```

## 7. Model Evaluation — Game-Score Benchmark, Not Regression

```rust
let accuracy = accuracy_score(&predicted_actions, &true_actions);
let mean_game_score = benchmark_mean_score(&model, n_games); // downstream benchmark, not MSE on score
```

## 8. Score Distribution Analysis

```rust
pub fn analyze_score_distribution(scores: &[u64]) -> ScoreDistribution { /* mean/median/std/skew */ }
```
