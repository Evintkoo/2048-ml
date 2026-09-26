# Plan 02 — Self-Play Data: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** The simulator can run a supplied single-agent policy, but the collector has no self-play mode or output corpus.

**Goal:** State the current implementation and evidence boundary for self-play data.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats self-play collection as pending.** `GameSimulator::simulate_with_policy` accepts a policy closure, but the checkpointed `data-collector collect` path only collects uniform random legal moves. No configurable policy-source collector or `self_play.csv` artifact exists.

## 1. Purpose

Generate state coverage by running a **single agent** against the stochastic 2048 environment (no opponent). A future policy-source collector would need to pass states through the same rollout labeler; the current collector does not provide this source option.

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

> Canonical target: **15,000 games** → approximately 750k rows (~50 moves/game). This remains planned; the current collector implements random trajectories only. Part of the 20k corpus target → 14k/3k/3k chronological game-level split (`groups=game_id`).

## 3. Collection Loop — Record then Relabel

```rust
for game_id in 0..config.n_games {
    let mut board = Board::new_seeded(config.seed + game_id as u64);
    while !board.is_game_over() {
        let state = BoardStateMl::from_board(&board);              // [f64;17] — see §8.2
        let original_action = agent.select_move(&board); // discarded after relabel
        let score = board.score();
        raw_rows.push((state, original_action, score, game_id));
        board.apply(original_action);                   // stochastic spawn inside
    }
}
// Mandatory relabel — replaces original_action:
let labeler = RolloutLabeler { simulator: GameSimulator::new(), n_rollouts: 100 };
let training_rows: Vec<([f64;17], u8)> = raw_rows.into_iter()
    .map(|(s, _, _, _)| { let (best, _) = labeler.label(&s); (s, best) })
    .collect();
// Write CSV: 17 cols + action = 18 cols — score is metadata, never label; no done/reward/next_state
```

Stochastic spawn note: after each `apply`, engine spawns `2` (p=0.9) or `4` (p=0.1) in a uniformly random empty cell. Seed via `ChaCha8Rng`.

## 4. Label Regeneration — REQUIRED

All self-play rows **MUST** be relabeled via `01-data-collection-strategy.md §8.3` (`RolloutLabeler`, 100 rollouts, `argmax` over valid moves by mean final score). Invalid moves are masked, never labeled. Original `original_action` is discarded.

## 5. Storage

CSV `06-Data/03-Storage/self_play.csv` — header `grid_0..score_normalized,action` (18 cols). See `02-Format/01-data-schema.md`; cross-ref `DataPreprocessor` for normalization fit on train only.

## 6. Next Steps

1. Run `SingleAgentTrajectory` with `n_games=10000` seeded.
2. Relabel with `RolloutLabeler { n_rollouts: 100 }`.
3. Validate `NF==18`, `action ∈ 0..3`, no `done`/`reward` columns; write to `06-Data/03-Storage/`.

The root collector currently supplies only random trajectories. Implement a configurable single-agent policy source and preserve rollout-based labels to fulfill this ticket.

## Implementation Record

- A single-agent trajectory can be represented by the simulator, but there is no distinct configurable self-play collection source or persisted `self_play.csv`. Only small random trajectories have been collected. Required rollout-labeled 15k-game self-play corpus remains pending.

---

## Verification (definition of done)

1. `test -f plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/01-Collection/02-self-play-data.md` exits 0.

## Open questions

- The proposed 15k-game self-play contribution is not collected. Define the policy source and compute budget, then retain config, seed, data, sidecar, and manifest artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
