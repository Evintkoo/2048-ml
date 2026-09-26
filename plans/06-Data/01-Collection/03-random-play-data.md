# Plan 03 — Random Play Data: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Random legal-action collection and rollout relabeling are implemented; the 5k-game corpus is not produced.

**Goal:** State the current implementation and evidence boundary for random play data.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the random collection mechanism as implemented and the target corpus as pending.** `collect_random_game` selects uniformly from legal directions; the CLI derives per-game seeds, rollout-relabels actions, and writes CSV, aligned metadata, and a manifest through a resumable checkpoint path.

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

**5,000 games** (~250k rows at ~50 moves/game) — canonical contribution to 20k total (15k single-agent self-play + 5k random). Heuristic trajectories are benchmark-only. See `01-data-collection-strategy.md §4`. The current CLI collects random trajectories with rollout relabeling. The 5k-game corpus is pending: measured throughput projects the full 20k rollout-labeled target to roughly 103 hours. Configurable via `n_games` but preserve 70/15/15.

## 5. Storage & Validation

CSV `06-Data/03-Storage/random_play.csv` — 28 cols `grid_0..row_worst,action`; no `done`/`reward`/`next_state`; `score: u64` is optional metadata. Validate `NF==28`, `action 0..3`.

## 6. Cross-References

## Implementation Record

- The CLI implements uniform valid-action random play and rollout relabeling, with deterministic per-game seeds, checkpoint/resume, group metadata, and manifest output. Game count and output path are configurable.
- The 5k-game corpus has not been created; the measured 20k projection is roughly 103 hours and awaits a declared compute budget.

- Volumes: `01-data-collection-strategy.md §4`
- Labeling: `01-data-collection-strategy.md §8.3`
- Schema: `02-Format/01-data-schema.md`
- Normalization: `DataPreprocessor` fit on train only (`04-Preprocessing/03-data-normalization.md`)

---

## Verification (definition of done)

1. `test -f plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/01-Collection/03-random-play-data.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/01-Collection/03-random-play-data.md` exits 0.

## Open questions

- Run the target corpus after setting a compute budget; preserve its checkpoint, CSV, metadata, manifest, seeds, and source revision.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
