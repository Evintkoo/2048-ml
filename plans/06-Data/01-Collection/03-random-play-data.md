# Plan 03 — Random Play Data: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for random play data.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

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

**5,000 games** (~250k rows at ~50 moves/game) — canonical contribution to 20k total (15k single-agent self-play + 5k random). Heuristic trajectories are benchmark-only. See `01-data-collection-strategy.md §4`. The current CLI collects random trajectories with rollout relabeling; canonical 20k collection is pending because the measured rollout labeling throughput projects to multiple days. Configurable via `n_games` but preserve 70/15/15.

## 5. Storage & Validation

CSV `06-Data/03-Storage/random_play.csv` — 28 cols `grid_0..row_worst,action`; no `done`/`reward`/`next_state`; `score: u64` is optional metadata. Validate `NF==28`, `action 0..3`.

## 6. Cross-References

## Implementation Record

- The CLI implements uniform valid-action random play and mandatory rollout relabeling, with per-game seeds and group metadata. It supports configured game count and output path.
- The required 5k-game corpus has not been created because current rollout-labeling throughput is far below the target collection scale; the measured 20k projection is documented in the ledger.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
