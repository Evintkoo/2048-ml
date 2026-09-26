# Plan 02 — Score as Feature: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-26).** Score contributes only the documented normalized input at index 21; action remains the sole target.

**Goal:** State the current implementation and evidence boundary for score as feature.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats score-as-feature as implemented, not as a research finding.** `BoardStateMl::from_board` places `log10(score+1)/6` at index 21. The score is also retained in row metadata, while move count and score history stay out of the model input and `action` remains the target.

## 1. Concept

Score provides game-progression context as one feature of the canonical training state. The canonical scope is board plus score; see `plans/00-scope-and-traceability.md`.

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

// Implemented in `src/state.rs` as `features[21]`.
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

## Implementation Record

- Score appears only as feature index 21 using `log10(score + 1) / 6` and as sidecar metadata; action remains the only training label. No score delta or history features were added.
- Feature index order and normalization are covered by root unit tests.

## 8. Score Distribution Analysis

```rust
// The root benchmark reporting path uses `evaluation::summarize_scores` for
// mean, standard deviation, median, percentiles, threshold counts, and a CI.
// Skew is not currently computed.
```

---

## Verification (definition of done)

1. `test -f plans/03-State/02-Score/02-score-as-feature.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/03-State/02-Score/02-score-as-feature.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/02-Score/02-score-as-feature.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/02-Score/02-score-as-feature.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/02-Score/02-score-as-feature.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/02-Score/02-score-as-feature.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/02-Score/02-score-as-feature.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/02-Score/02-score-as-feature.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
