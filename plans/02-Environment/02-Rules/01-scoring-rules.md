# Plan 01 — Scoring Rules: the repository status is explicit and evidence based

> **Status: DONE (2026-09-26).** Merge values, positions, per-turn gains, and accumulated score are tracked; root suite passes.

**Goal:** State the current implementation and evidence boundary for scoring rules.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Merge values, positions, per-turn gains, and accumulated score are tracked; root suite passes.

> **Canonical scoring.** Score is **metadata / evaluation only, never a training label.** See also `01-Infrastructure/01-Project/01-project-overview.md` Tiers and `03-State/01-Board/01-board-state.md` index 21.

## 1. Score Definition

Score = cumulative sum of all tile-merge values in a game. It is stored as `u64` on `RawBoardState` alongside the grid and game metadata.

## 2. Calculation

```
Score = Σ merge_value   (merge 128+128 → +256; 256+256 → +512)
```

- Only merges contribute. Spawns and slides do not change score.
- `score_delta` = gain on last valid move; tracked per `MoveRecord`.

## 3. Tracking

```rust
pub struct ScoreTracker {
    pub total_score: u64,              // Board.score
    pub merge_history: Vec<MergeEvent>,// per-merge audit
    pub turn_scores: Vec<u64>,         // per-turn delta for analysis
}
pub struct MergeEvent {
    pub turn: u64,
    pub tile_value: u64,               // resultant tile (e.g., 256)
    pub position: (usize, usize),
    pub score_gained: u64,             // == tile_value
}
```

## 4. Score Is Evaluation Metric Only — NOT a Training Target (Canonical Constraint)

| Property | Value |
|----------|-------|
| **Training label (target)** | `action: u8` 0–3 (`Direction::Up=0, Down=1, Left=2, Right=3`) |
| **Canonical input** | 16 board cells plus current score (17 values), per Plan 00 |
| **Current implementation** | `state_features: [f64;17]` with `score_normalized` at index 16; score remains encoded input and raw metadata |
| **TaskType** | `TaskType::MultiClassification` (Evintkoo/automl) — **not regression** |
| **Score column role** | `score: u64` **metadata only** — post-hoc analysis & benchmarking (mean/median, distribution, `> heuristic ~512` rate). **Never `y`.** |

> **Do not** use `score` as `y`, do not regress `log10(score)`, do not evaluate with `R²/RMSE/MAE` on score — model predicts **actions**, not scores.

## 5. Score as Feature (Not Target)

The canonical input includes current score as its seventeenth value. The implementation encodes it as `score_normalized` at index 16; the raw score remains available in metadata.

```rust
pub struct GameFeatures { // excerpt from current implementation; not the canonical input schema
    pub score: f64,                // raw for display
    pub score_normalized: f64,     // canonical feature index 21 — see §6
    pub score_delta: f64,
    pub avg_score_per_move: f64,
}
```

## 6. Current Implementation — Normalized Score at Feature Index 21

The 17-value implementation normalizes score at index 16, not as a target:

Implemented in `src/state.rs` as `(board.score as f64 + 1.0).log10() / 6.0`, stored at feature index 16. No inverse transform is used in the training path.

> The canonical vector is `[grid_0..grid_15, score_normalized]`; the previous 27-column derived-feature vector is excluded from core training.

## 7. RewardSignal — NOT USED (RL Out of Scope) — STRONG WARNING

> **NOT USED — FOR ANALYSIS ONLY.** Project is **supervised classification only** (`TaskType::MultiClassification`). Do not use `reward`/`next_state`/`done` as training signals. The canonical row is `(state_values:[f64;17], action:u8)` with optional score metadata. AutoML has no RL loop.

```rust
// ⚠️ NOT USED for training — retained for historical reference only — NOT for supervised automl
// If you import this struct you are violating the canonical paradigm. Delete if unsure.
pub struct RewardSignal {
    pub immediate_reward: f64,  // score gained this turn — analysis only, not label
    pub final_reward: f64,      // total score — analysis only
    pub survival_bonus: f64,    // penalty for game_over — analysis only
    pub progress_bonus: f64,    // tile-value increase — analysis only
}
```

## 8. Metrics for Evaluation (Post-Hoc on `score:u64`)

| Metric | Description | Use |
|--------|-------------|-----|
| Mean Score | mean `score` over ≥10k games | **Primary ranking** |
| Max / Median | best / 50th percentile | Ceiling / robustness |
| Score Distribution | percentiles, histogram | Model comparison |
| Score > Heuristic Rate | % games with `score > ~512` | Quality vs heuristic baseline |

> No `MoveSequence Vec<Board>` / LSTM needed — row is i.i.d. for `GroupKFold` (group=`game_id`; not temporal — see `05-Model/04-Evaluation/02-cross-validation.md`; use `TimeSeriesSplit` for temporal).

## 9. Cross-References

## 10. Implementation Record

`RawBoardState::execute_move` returns score delta and per-merge events with resultant tile value, board position, and turn index. The reusable `ScoreTracker` accumulates total score, per-turn deltas, and merge history in `GameResult` metadata. Score is not an action target. Tests verify score and merge positions for all four directions.

- **Board and canonical input:** Plan 00 fixes the input at 17 values; ticket #034 implements it.
- **Win/lose & valid moves:** `02-win-lose-conditions.md` (`would_change`), `03-valid-moves.md`
- **RNG/seed hygiene:** `03-Simulation-Engine/02-randomness.md` (`ChaCha8Rng`, `spawn_prob_4:0.1`, `with_random_state(42)`)
- **Training row:** `03-Simulation-Engine/01-simulation-engine.md` — canonical `TrainingSample { [f64;17], u8, u64 }`.
- **Project tiers / heuristic ~512:** `01-Infrastructure/01-Project/01-project-overview.md`

---

## Verification (definition of done)

1. `test -f plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/02-Rules/01-scoring-rules.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Merge values, positions, per-turn gains, and accumulated score are tracked; root suite passes. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
