# Plan 03 — Valid Moves: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Move validity and seeded 10k random/heuristic frequencies are measured; model-policy frequency awaits a trained model.

**Goal:** State the current implementation and evidence boundary for valid moves.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats move validity as implemented and its frequency analysis as partial.** Seeded 10k-game random and heuristic baselines were measured; results and raw artifacts are recorded in [the action-frequency report](../../../reports/action-frequency/README.md). Intervals resample whole games to account for within-game dependence. A trained model is not available, so model-policy frequencies remain pending.

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

The model returns four class probabilities. The root selects from legal action IDs with `actions::masked_argmax`, then converts the selected ID with `Direction::try_from_action`. The supervised row is `state_features:[f64;27] → action:u8`; score remains metadata.

## 6. Constrained Action (Legal-Action Masking)

`ModelPolicy::select_move` calls `predict_proba_array`, collects valid actions from
`RawBoardState::get_valid_moves`, and applies `masked_argmax` only to those IDs.
The selection does not need probability renormalization. The standalone
`actions::masked_argmax` also rejects empty legal sets, non-finite scores, and
invalid action IDs, and breaks ties by action order.

## 7. Action Frequency — Measured Baselines

The retained report summarizes 10,000 games per agent with game-cluster bootstrap 95% intervals:

| Move | Random proportion (95% CI) | Heuristic proportion (95% CI) |
|------|----------------------------|------------------------------|
| Up | 0.249527 [0.248847, 0.250213] | 0.262476 [0.262011, 0.262956] |
| Down | 0.250478 [0.249757, 0.251181] | 0.241385 [0.240928, 0.241848] |
| Left | 0.249917 [0.249265, 0.250623] | 0.257937 [0.257469, 0.258413] |
| Right | 0.250077 [0.249356, 0.250779] | 0.238202 [0.237747, 0.238662] |

These are descriptive baseline frequencies, not a training prior or framework result. Raw CSVs, manifests, seed range, and bootstrap protocol are retained in the linked report.

## 8. Deleted — No Sequence / LSTM Hint

> **Deleted:** `MoveSequence { moves:Vec<Direction>, scores:Vec<u64>, board_states:Vec<Board> }` — no RL/LSTM in MVP. Training rows are **i.i.d.** for `GroupKFold` (`groups=game_id`; not temporal — see `05-Model/04-Evaluation/02-cross-validation.md`; use `TimeSeriesSplit` for temporal). No `rewards:Vec<f64>` anywhere (violates supervised-only canonical — `TrainingSample` is `{ states:Vec<[f64;27]>, actions:Vec<u8>, scores:Vec<u64> }`).

## 9. Heuristic Strategies (Reference Only, Baseline)

```rust
// Reference only — not MVP training path
pub enum HeuristicStrategy { Monotonicity, Corner, Empty }
// See 03-Simulation-Engine/01-simulation-engine.md Appendix — heuristic agent is baseline only (~512)
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
- Validation: root unit suite passed. The random and heuristic action-frequency protocol is now measured with 10,000 games per agent and game-cluster bootstrap 95% intervals. The model-policy frequency analysis remains unmeasured until a trained policy artifact exists.

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

- **The plan-scale evidence remains bounded by current results.** Random and heuristic action distributions are measured; model-policy frequency is pending a trained policy artifact. Do not generalize the baseline distributions to trained policies.

## Later

- **Repeat the action-frequency analysis for the held-out trained model after its training/evaluation prerequisite is complete.** Keep the same seed protocol and resample complete games for uncertainty.
