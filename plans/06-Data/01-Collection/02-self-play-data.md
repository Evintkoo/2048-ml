# Self-Play Data — Single-Agent Trajectory + Rollout Relabeling

## 1. Purpose

Generate state coverage by running a **single agent** against the stochastic 2048 environment (no opponent). Every `(state, action)` is relabeled by `RolloutLabeler` (§8.3) — the agent's action is discarded.

## 2. Single-Agent Model (2048 is single-player)

2048 has no second player. "Self-play" here means **one agent vs. stochastic tile spawns** (90% `2`, 10% `4` at a random empty cell after each move).

```rust
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

pub struct SingleAgentTrajectory {
    pub agent: Box<dyn Agent>,          // Random or Heuristic — any policy, label is discarded
    pub simulator: GameSimulator,        // 4×4 board, 90/10 tile spawn
    pub rng: ChaCha8Rng,                // seeded ChaCha8Rng
}

pub struct SelfPlayConfig {
    pub n_games: usize,                 // canonical 15,000 (part of total 20k; configurable)
    pub agent_type: AgentType,          // Random | Heuristic — label is discarded anyway
    pub seed: u64,                      // ChaCha8Rng seed
    pub output_path: String,            // 06-Data/03-Storage/ canonical; data/ symlink
}
```

> Canonical: **15,000 games** → approximately 750k rows (~50 moves/game). Part of the 20k training corpus → 14k/3k/3k chronological game-level split (`groups=game_id`).

## 3. Collection Loop — Record then Relabel

```rust
for game_id in 0..config.n_games {
    let mut board = Board::new_seeded(config.seed + game_id as u64);
    while !board.is_game_over() {
        let state = board.to_features27();              // [f64;27] — see §8.2
        let original_action = agent.select_move(&board); // discarded after relabel
        let score = board.score();
        raw_rows.push((state, original_action, score, game_id));
        board.apply(original_action);                   // stochastic spawn inside
    }
}
// Mandatory relabel — replaces original_action:
let labeler = RolloutLabeler { simulator: GameSimulator::new(), n_rollouts: 100 };
let training_rows: Vec<([f64;27], u8)> = raw_rows.into_iter()
    .map(|(s, _, _, _)| { let (best, _) = labeler.label(&s); (s, best) })
    .collect();
// Write CSV: 27 cols + action = 28 cols — score is metadata, never label; no done/reward/next_state
```

Stochastic spawn note: after each `apply`, engine spawns `2` (p=0.9) or `4` (p=0.1) in a uniformly random empty cell. Seed via `ChaCha8Rng`.

## 4. Label Regeneration — REQUIRED

All self-play rows **MUST** be relabeled via `01-data-collection-strategy.md §8.3` (`RolloutLabeler`, 100 rollouts, `argmax` over valid moves by mean final score). Invalid moves are masked, never labeled. Original `original_action` is discarded.

## 5. Storage

CSV `06-Data/03-Storage/self_play.csv` — header `grid_0..row_worst,action` (28 cols). See `02-Format/01-data-schema.md`; cross-ref `DataPreprocessor` for normalization fit on train only.

## 6. Next Steps

1. Run `SingleAgentTrajectory` with `n_games=10000` seeded.
2. Relabel with `RolloutLabeler { n_rollouts: 100 }`.
3. Validate `NF==28`, `action ∈ 0..3`, no `done`/`reward` columns; write to `06-Data/03-Storage/`.
