# Valid Moves — Canonical

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

Model outputs 4 logits → `argmax` → `action:u8`.

```rust
pub fn output_to_action(outputs: &[f64;4]) -> Direction {
    let (idx,_) = outputs.iter().enumerate()
        .max_by(|a,b| a.1.partial_cmp(b.1).unwrap()).unwrap();
    Direction::from_u8(idx as u8)
}
/// Supervised row: state_features:[f64;27] → action:u8 — TaskType::MultiClassification; score is metadata only (see 01-scoring-rules.md)
```

## 6. Constrained Action (Renormalization)

Force invalid moves to prob 0; renormalize over valid only.

```rust
pub fn constrained_action(outputs: &[f64;4], board: &Board) -> Direction {
    let valid = board.get_valid_moves();
    debug_assert!(!valid.is_empty(), "call only when not game_over");
    let sum: f64 = valid.iter().map(|d| outputs[*d as usize].max(0.0)).sum();
    // Guard div-by-zero: if sum==0, fallback to uniform over valid
    let mut constrained = [0.0f64;4];
    if sum > 1e-12 {
        for d in valid.iter() { constrained[*d as usize] = outputs[*d as usize].max(0.0) / sum; }
    } else {
        for d in valid.iter() { constrained[*d as usize] = 1.0 / valid.len() as f64; }
    }
    output_to_action(&constrained)
}
```

> **Requires `InferenceConfig` output handling:** automl `InferenceEngine` returns raw logits — apply softmax then `constrained_action` in post-processing. Do not mutate automl internals; handle in `04-Actions/03-Mapping/01-model-output-to-action.md` policy. See `01-Infrastructure/02-Configuration/02-training-config.md`.

## 7. Action Frequency — Unvalidated Hypothesis (Measure, Do Not Assert)

> **Not asserted.** Numbers below are **unvalidated hypothesis — measure empirically** over ≥10k games per agent. Do not use as training prior.

| Move | Frequency (Hypothesis, TBD) | Hypothesized Rationale |
|------|-----------------------------|------------------------|
| Left | hypothesized dominant | Left is base slide; monotonic boards favor left/up |
| Up | secondary | — |
| Down / Right | rare | — |

> **Procedure:** log `action:u8` histogram per `SimulationBatch` in `03-Simulation-Engine/03-multi-game.md`; report empirical distribution with 95% CI. Update this doc after measurement.

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
