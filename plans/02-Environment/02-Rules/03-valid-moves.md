# Plan 03 — Valid Moves: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** Canonical move validity and seeded 10,000-game frequencies for random, heuristic, and a fitted AutoML policy are recorded with whole-game uncertainty intervals.

**Goal:** State the current implementation and evidence boundary for valid moves.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats move validity and descriptive policy-frequency analysis as implemented.** Seeded 10,000-game random and heuristic baselines and a 10,000-game fitted-policy profile are retained in [the action-frequency report](../../../reports/action-frequency/README.md). Intervals resample whole games to account for within-game dependence. The baseline report records that its root source was locally modified during collection; the model-policy run records its committed source revision and AutoML pin. The fitted policy came from a small development corpus, so its action frequencies do not establish policy quality or select a model winner.

> **Canonical validity:** `board.would_change(dir)` — single source. Do not duplicate `get_valid_moves` logic elsewhere.

## 1. Validity Definition

Move valid iff `would_change(direction) == true` (executing slide/merge changes `[u32;16]`). Invalid moves produce no board change, no spawn, no training row.

## 2. Enumeration

| Direction | u8 | Tile Movement | Canonical Transform |
|-----------|----|---------------|---------------------|
| Up | 0 | shift up | `transpose → slide_left → transpose` |
| Down | 1 | shift down | `transpose → slide_right → transpose` |
| Left | 2 | shift left | `slide_left` (base) |
| Right | 3 | shift right | `reverse_rows → slide_left → reverse_rows` |

> See `01-Game/03-board-representation.md` §3 for transforms and `01-Game/01-game-engine.md` §4 for execution.

## 3. Canonical Detection

```rust
pub fn get_valid_moves(board: &Board) -> Vec<Direction> {
    Direction::ALL.iter().copied().filter(|d| board.would_change(*d)).collect()
}
/// Inference helper:
pub fn valid_mask(board: &Board) -> [bool;4] {
    let mut m=[false;4];
    for d in Direction::ALL { m[d as usize]=board.would_change(d); }
    m
}
```

## 4. Per-State Analysis

```rust
pub struct MoveAnalysis {
    pub valid_moves: Vec<Direction>,   // subset of 0–3
    pub num_valid_moves: usize,        // 1–4 (0 ⇒ is_game_over)
    pub best_move: Option<Direction>,  // predicted (argmax over valid)
    pub move_scores: [f64;4],          // logits/probs from automl (4)
    pub move_feasibility: [bool;4],    // valid_mask
}
```

## 5. ML Encoding — Action u8 0–3

The model returns four class probabilities. The root selects from legal action IDs with `actions::masked_argmax`, then converts the selected ID with `Direction::try_from_action`. Plan 00 defines the canonical supervised row as 17 state values → `action:u8`; ticket #034 aligned the root policy input with that schema.

## 6. Constrained Action (Legal-Action Masking)

`ModelPolicy::select_move` calls `predict_proba_array`, collects valid actions from
`RawBoardState::get_valid_moves`, and applies `masked_argmax` only to those IDs.
The selection does not need probability renormalization. The standalone
`actions::masked_argmax` also rejects empty legal sets, non-finite scores, and
invalid action IDs, and breaks ties by action order.

## 7. Action Frequency — Measured Baselines

The retained report summarizes 10,000 games per agent with game-cluster bootstrap 95% intervals:

| Move | Random proportion (95% CI) | Heuristic proportion (95% CI) | Model pilot proportion (95% CI) |
|------|----------------------------|------------------------------|-------------------------------|
| Up | 0.249527 [0.248847, 0.250213] | 0.262476 [0.262011, 0.262956] | 0.262037 [0.260770, 0.263295] |
| Down | 0.250478 [0.249757, 0.251181] | 0.241385 [0.240928, 0.241848] | 0.158884 [0.158232, 0.159540] |
| Left | 0.249917 [0.249265, 0.250623] | 0.257937 [0.257469, 0.258413] | 0.231507 [0.230470, 0.232498] |
| Right | 0.250077 [0.249356, 0.250779] | 0.238202 [0.237747, 0.238662] | 0.347573 [0.346447, 0.348775] |

These are descriptive frequencies, not a training prior or framework result. Random and heuristic baseline raw CSVs/manifests retain aggregate action counts; the fitted-policy CSV retains per-game action counts. All use seed range 84024–94023 and 2,000 whole-game bootstrap replicates with seed 84026. Detailed policy fit provenance and limitations are recorded in the linked report.

## 8. Deleted — No Sequence / LSTM Hint

> **Deleted:** `MoveSequence { moves:Vec<Direction>, scores:Vec<u64>, board_states:Vec<Board> }` — no RL/LSTM in MVP. Rows from the same game are grouped together in CV to prevent game leakage; GroupKFold itself is not temporal. No reward labels are used. Canonical state values are 17.

## 9. Heuristic Strategies (Reference Only, Baseline)

```rust
// Reference only — not MVP training path
pub enum HeuristicStrategy { Monotonicity, Corner, Empty }
// See 03-Simulation-Engine/01-simulation-engine.md Appendix — heuristic agent is baseline only
```

- Monotonicity: keep rows/cols monotonic
- Corner: keep max tile (`/32768`) in corner
- Smoothness / empty-tiles: minimize neighbor diff, maximize empties — see `03-State/01-Board/02-feature-extraction.md`

## 10. Cross-References

- **Board / transforms:** `01-Game/03-board-representation.md`
- **Spawn / RNG:** `01-Game/01-game-engine.md` §4.3 + `03-Simulation-Engine/02-randomness.md` (`ChaCha8Rng`, `spawn_prob_4:0.1`)
- **Scoring (metadata):** `01-scoring-rules.md` (label is `action:u8`, score `u64` metadata)
- **Simulation / dataset:** `03-Simulation-Engine/01-simulation-engine.md` + `03-multi-game.md`
- **CV:** `05-Model/04-Evaluation/02-cross-validation.md` (`GroupKFold` group-integrity vs `TimeSeriesSplit` temporal)

## Implementation Record

- `RawBoardState::would_change`, `get_valid_moves`, and `valid_mask` provide the canonical validity APIs; policy simulation rejects a selected move that would not change the board.
- Added coverage comparing validity to actual execution across a deterministic family of boards and checking terminal boards have no valid direction.
- Validation: root unit suite passed. Random and heuristic frequency baselines cover 10,000 games each; a fitted AutoML RandomForest policy was also evaluated on 10,000 distinct game seeds. Per-game direction counts sum to move count for all 10,000 policy rows. The analyzer reproduced the pooled proportions and 2,000-replicate whole-game percentile intervals; raw CSV, run manifest, analysis JSON, and script are retained. The small pilot training corpus limits interpretation to descriptive policy behavior.

---

## Verification (definition of done)

1. `test -f plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/02-Rules/03-valid-moves.md` exits 0.

## Open questions

- **The evidence remains descriptive.** Three agents have retained frequency estimates under one 10,000-game seed sequence; only the fitted-policy output retains per-game action counts for reconstructing cluster intervals. The pilot policy's action frequencies do not establish score superiority, classifier generalization, or an AutoML framework result.

## Later

- **Repeat the profile for a future selected policy if model selection changes the evaluated artifact.** Preserve the seed protocol and resample complete games; keep action-frequency conclusions separate from score-based model selection.
