# Plan 03 — Multi-Game Simulation: the repository status is explicit and evidence based

> **Status: PARTIAL.** Fixed-thread collection, metadata, and manifests exist; checkpoint/resume, progress reporting, 10k baseline, and frequency CIs remain incomplete.

**Goal:** State the current implementation and evidence boundary for multi-game simulation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Fixed-thread collection, metadata, and manifests exist; checkpoint/resume, progress reporting, 10k baseline, and frequency CIs remain incomplete.

> **Sample size canonical: 10k games minimum** for benchmarking (per `01-Infrastructure/01-Project/01-project-overview.md` §8 Tiers + `01-simulation-engine.md` §7 `SimulationConfig.n_games:10000`). Supervised only: `GameDataset { states:[f64;27], actions:u8, scores:u64 }` — **no `rewards`**.

## 1. Purpose

Run N games sequentially/parallel to collect sufficient supervised rows for automl `TaskType::MultiClassification`. Headless only; results feed `06-Data/` Parquet/CSV.

## 2. Game Sequencing

```rust
pub struct GameSequence {
    pub games: Vec<GameResult>,
    pub total_games: usize, // 10000 canonical
    pub seed: u64,          // global 42 → per-game via wrapping_add — see 02-randomness.md
    pub agent: AgentType,   // Random | Model (MVP)
}
impl GameSequence {
    pub fn run(&mut self, n: usize) -> &GameSequence {
        for i in 0..n {
            let game = self.run_single_game(i); // ChaCha8Rng::seed_from_u64(seed.wrapping_add(i as u64))
            self.games.push(game);
        }
        self
    }
    fn run_single_game(&self, game_id: usize) -> GameResult { todo!() } // see 01-simulation-engine.md §4
}
```

> **Loop canonical:** `01-simulation-engine.md` §4 `SimulationBatch::run()` / `run_parallel()` — do not duplicate `GameRunner` variants here.

## 3. Batch Processing

### 3.1 Sequential (Deterministic)

```rust
// Canonical 10k — see §6
let results: Vec<GameResult> = (0..10000)
    .map(|i| run_game_with_seed(seed.wrapping_add(i as u64)))
    .collect();
```

### 3.2 Parallel (rayon, deterministic if seeded per-game)

```rust
use rayon::prelude::*;
let results: Vec<GameResult> = (0..10000)
    .into_par_iter()
    .map(|i| run_game_with_seed(seed.wrapping_add(i as u64)))
    .collect();
// Fix thread count — see 02-randomness.md §8 (RAYON_NUM_THREADS, threads field)
```

## 4. Data Accumulation — NO `rewards`

```rust
/// Canonical dataset — supervised only. No RL tuple. score is metadata, never label.
pub struct GameDataset {
    pub states: Vec<[f64;27]>,        // BoardStateML::to_array() per turn (includes idx21 log10(score+1)/6.0)
    pub actions: Vec<u8>,             // ONLY label: 0–3 (Up=0,Down=1,Left=2,Right=3)
    pub scores: Vec<u64>,             // per-turn Board.score metadata for analysis — NEVER rewards
    pub metadata: Vec<GameMetadata>,  // { game_id:u64, move_count, final_max_tile:u32 }
    // DELETED: pub rewards: Vec<f64> — violates supervised-only canonical (TrainingSample is [f64;27]→u8)
}
impl GameDataset {
    pub fn from_games(games: Vec<GameResult>) -> Self {
        let mut ds = GameDataset { states:Vec::new(), actions:Vec::new(), scores:Vec::new(), metadata:Vec::new() };
        for g in games { ds.extend_from_game(g); }
        ds
    }
    fn extend_from_game(&mut self, g: GameResult) { todo!() } // flatten per-move rows
}
// DataFrame row: state_features:[f64;27], action:u8, score:u64, game_id:u64 (for GroupKFold groups)
// See 01-simulation-engine.md §8 TrainingSample and 06-Data/02-Format/01-data-schema.md
```

> **Critical:** Any `rewards:Vec<f64>` is **banned**. If you see it, delete it — project has no RL, no `RewardSignal` training.

## 5. Data Collection Pipeline

```
Game(i, seed.wrapping_add(i)) → record [f64;27] + u8 per valid move → GameDataset → Parquet/CSV → automl TrainEngine
```

> Cross-ref loops: `01-simulation-engine.md` §4 (`SimulationBatch`). Do not re-define `GameSimulator` here.

## 6. Sample Size — Canonical 10k (Not Placeholder)

| Goal | Canonical Minimum | Note |
|------|-------------------|------|
| Baseline / benchmark / statistical significance | **10,000 games** | Per project-overview Tiers & §2/§3 above — fixed, not 1,000 |
| Training | 10,000 minimum; 50k–100k recommended if compute allows | log scaling; measure after 10k |
| Evaluation CI | ≥10k for bootstrap 95% CI & Mann-Whitney U p<0.05 | see 07-Benchmarking/ |

> **Deleted placeholder:** `PerformanceTargets { games_per_second: 1000 }` — **remove placeholder throughput claim**. Measure empirically and report actual `avg_game_duration_ms` in `SimulationMetrics` (see `01-simulation-engine.md` §6). Do not assert 1000 games/sec before measurement. Same for `total_time_10k: Duration` placeholder — delete.

## 7. Progress Tracking

```rust
pub struct ProgressTracker {
    pub games_completed: usize,
    pub total_games: usize,       // 10000 canonical
    pub current_score: u64,       // metadata
    pub best_score: u64,
    pub avg_score_so_far: f64,
    pub time_elapsed: std::time::Duration,
}
// Display via indicatif — optional, not MVP critical
```

## 8. Checkpointing

```rust
fn checkpoint(results: &[GameResult], game_count: usize) {
    if game_count % 1000 == 0 {
        save_results(&results[game_count-1000..game_count]); // Parquet append
    }
}
```

## 9. Cross-References

- **Loops / batch (canonical):** `01-simulation-engine.md` §4 (`SimulationBatch::run`/`run_parallel`)
- **RNG / seed:** `02-randomness.md` (`ChaCha8Rng`, `wrapping_add`, `TrainingConfig::with_random_state(42)`, rayon threads)
- **Training row:** `01-simulation-engine.md` §8 (`TrainingSample { [f64;27], u8, u64 }`)
- **Scoring / win-lose / valid moves:** `02-Rules/01-scoring-rules.md` (metadata), `02-win-lose-conditions.md` (`would_change`), `03-valid-moves.md`
- **Features (27-dim, idx21 /6.0, grid /32768):** `03-State/01-Board/01-board-state.md`
- **CV:** `05-Model/04-Evaluation/02-cross-validation.md` (`GroupKFold` groups=`game_id` vs `TimeSeriesSplit`)
- **Headless only:** `01-Game/01-game-engine.md` (SimulatorConfig `seed:42, spawn_prob_4:0.1`), `01-Game/04-game-ui.md` deprecated stub

## Implementation Record

- Implemented fixed-thread Rayon collection, per-game `global_seed.wrapping_add(game_id)`, game-group metadata, rollout relabeling, CSV schema validation, and a JSON manifest with seeds, thread count, timing, row counts, and file hashes.
- A reusable random-game batch API now supports deterministic game-ID ranges. The collector still materializes a whole requested batch before writing; it does not yet checkpoint/resume every 1,000 games or report live progress. The plan's 10k-game minimum and action-frequency confidence intervals are not complete until a full run and analysis are recorded.
- Validation: deterministic batch coverage and root suite pass. Prior throughput measurement in the ledger projects approximately 103 hours for the current 20k rollout-labeled collection configuration; do not treat smoke runs as the required baseline.

---

## Verification (definition of done)

1. `test -f plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/03-Simulation-Engine/03-multi-game.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Fixed-thread collection, metadata, and manifests exist; checkpoint/resume, progress reporting, 10k baseline, and frequency CIs remain incomplete. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
