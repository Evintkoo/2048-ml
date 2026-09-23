# Random Play Data — Appendix (Wide Coverage, Relabeled)

## 1. Purpose

Provide wide state-space coverage via uniformly random valid moves. Labels are **still relabeled** by `RolloutLabeler` — random actions are discarded.

## 2. RandomAgent — Seed-Managed

```rust
use rand_chacha::ChaCha8Rng;
use rand::{SeedableRng, Rng};

pub struct RandomAgent { rng: ChaCha8Rng }
impl RandomAgent {
    pub fn new(seed: u64) -> Self { Self { rng: ChaCha8Rng::seed_from_u64(seed) } }
}
impl Agent for RandomAgent {
    fn select_move(&mut self, board: &Board) -> u8 {
        let valid = board.get_valid_moves(); // Vec<u8> subset of {0,1,2,3}
        let idx = self.rng.gen_range(0..valid.len());
        valid[idx]
    }
}
// Seed: ChaCha8Rng(42) canonical; per-game seed = 42 + game_id for reproducibility
// Spawn: 90% 2 / 10% 4 stochastic — same engine as self-play
```

## 3. Collection & Relabeling

Same loop as `02-self-play-data.md §3` — record `(state, random_action)` then **mandatory** `RolloutLabeler { n_rollouts: 100 }` relabel (see `01-data-collection-strategy.md §8.3`). Random action is discarded; rollout `argmax` over valid moves becomes `action` label. Invalid moves masked.

## 4. Volume — Canonical (Configurable)

**5,000 games** (~250k rows at ~50 moves/game) — canonical contribution to 20k total (10k self + 5k random + 5k heuristic-adjacent → 14k/3k/3k chronological `GroupKFold` `shuffle=false`, `groups=game_id`). See `01-data-collection-strategy.md §4`. Configurable via `n_games` but preserve 70/15/15.

## 5. Storage & Validation

CSV `06-Data/03-Storage/random_play.csv` — 28 cols `grid_0..row_worst,action`; no `done`/`reward`/`next_state`; `score: u64` is optional metadata. Validate `NF==28`, `action 0..3`.

## 6. Cross-References

- Volumes: `01-data-collection-strategy.md §4`
- Labeling: `01-data-collection-strategy.md §8.3`
- Schema: `02-Format/01-data-schema.md`
- Normalization: `DataPreprocessor` fit on train only (`04-Preprocessing/03-data-normalization.md`)
