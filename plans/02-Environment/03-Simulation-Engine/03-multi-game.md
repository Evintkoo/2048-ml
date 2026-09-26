# Plan 03 — Multi-Game Simulation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Fixed-thread collection, metadata, manifests, batched checkpoints/resume, and live progress are implemented. Plan-scale rollout collection and report validation remain pending.

**Goal:** State the current implementation and evidence boundary for multi-game simulation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The 10k random and heuristic baselines and whole-game bootstrap frequency intervals are complete for the measured configuration. Rollout collection now checkpoints bounded game batches and can resume after interruption; no plan-scale rollout corpus has been collected yet.

> **Sample size: 10k games minimum** for benchmarking (per project-overview Tiers and §7 below). Supervised only: the canonical state has 17 values under Plan 00, now used by the root collector. No `rewards` are used.

## 1. Purpose

Run N games sequentially/parallel to collect sufficient supervised rows for automl `TaskType::MultiClassification`. Headless only; the root collector writes CSV plus a metadata sidecar.

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
    pub states: Vec<[f64;17]>,        // canonical: 16 board cells plus current score
    pub actions: Vec<u8>,             // ONLY label: 0–3 (Up=0,Down=1,Left=2,Right=3)
    pub scores: Vec<u64>,             // per-turn Board.score metadata for analysis — NEVER rewards
    pub metadata: Vec<GameMetadata>,  // { game_id:u64, move_count, final_max_tile:u32 }
    // DELETED: pub rewards: Vec<f64> — the task is supervised action classification
}
impl GameDataset {
    pub fn from_games(games: Vec<GameResult>) -> Self {
        let mut ds = GameDataset { states:Vec::new(), actions:Vec::new(), scores:Vec::new(), metadata:Vec::new() };
        for g in games { ds.extend_from_game(g); }
        ds
    }
    fn extend_from_game(&mut self, g: GameResult) { todo!() } // flatten per-move rows
}
// Canonical DataFrame row: 17 state values + action and provenance metadata.
// Root writer outputs 17 state columns.
// See 01-simulation-engine.md §8 TrainingSample and 06-Data/02-Format/01-data-schema.md
```

> **Critical:** Any `rewards:Vec<f64>` is **banned**. If you see it, delete it — project has no RL, no `RewardSignal` training.

## 5. Data Collection Pipeline

```
Game(i, seed.wrapping_add(i)) → 17 state values + u8 per valid move → checkpointed CSV + metadata → AutoML TrainEngine
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

The root collector displays completed games, elapsed time, and estimated time remaining. Updates arrive as each parallel game finishes.

```rust
pub struct ProgressTracker {
    pub games_completed: usize,
    pub total_games: usize,       // 10000 canonical
    pub current_score: u64,       // metadata
    pub best_score: u64,
    pub avg_score_so_far: f64,
    pub time_elapsed: std::time::Duration,
}
// Implemented with indicatif in the root collector.
```

## 8. Checkpointing

The collector writes one CSV/metadata part per configured game batch under
`<output-stem>.checkpoint/parts/`, then atomically advances a JSON checkpoint.
Resume with the same seed, game count, rollout count, thread count, and batch
size using `--resume`. The final CSV and metadata are assembled after all games.

```rust
cargo run --release -- data-collector collect --n-games 20000 --rollouts 100 --threads 4 \
  --checkpoint-every 1000 --output data/raw/policy.csv
# After interruption, rerun the same arguments and add --resume.
# Estimated runtime is approximately 103 hours; run only after compute budget approval.
```

## 9. Cross-References

- **Loops / batch (canonical):** `01-simulation-engine.md` §4 (`SimulationBatch::run`/`run_parallel`)
- **RNG / seed:** `02-randomness.md` (`ChaCha8Rng`, `wrapping_add`, `TrainingConfig::with_random_state(42)`, rayon threads)
- **Training row:** `01-simulation-engine.md` §8 (`TrainingSample { [f64;17], u8, u64 }`)
- **Scoring / win-lose / valid moves:** `02-Rules/01-scoring-rules.md` (metadata), `02-win-lose-conditions.md` (`would_change`), `03-valid-moves.md`
- **Canonical input:** Plan 00 (16 board cells plus current score); previous 27-column derived features are documented in `03-State/01-Board/02-feature-extraction.md`.
- **CV:** `05-Model/04-Evaluation/02-cross-validation.md` (`GroupKFold` groups=`game_id` vs `TimeSeriesSplit`)
- **Headless only:** `01-Game/01-game-engine.md` (SimulatorConfig `seed:42, spawn_prob_4:0.1`), `01-Game/04-game-ui.md` deprecated stub

## Implementation Record

- Implemented fixed-thread Rayon collection, per-game `global_seed.wrapping_add(game_id)`, game-group metadata, rollout relabeling, CSV schema validation, and a JSON manifest with seeds, thread count, timing, row counts, and file hashes.
- The collector now writes per-batch data/metadata parts and an atomic, config-checked JSON checkpoint; `--resume` continues from the next game ID. Indicatif reports live game completion. Final assembly validates both row-aligned files.
- Baseline evidence: [`reports/action-frequency/README.md`](../../../reports/action-frequency/README.md) records 10,000 random and 10,000 heuristic games, raw score/frequency CSVs, manifests, and 2,000-replicate whole-game bootstrap 95% intervals. These runs establish case-study baselines only; they do not establish framework superiority.
- Artifact integrity check (2026-09-27): both CSVs contain 10,000 game rows plus the header, and each SHA-256 matches its manifest. This verifies file integrity and row counts, not an independent recomputation of the statistical summaries.
- Validation: deterministic batch coverage and checkpoint/resume tests pass. The Planout checker is part of the ticket verification. Prior throughput measurement projects approximately 103 hours for the current 20k rollout-labeled collection configuration; this compute estimate is a prerequisite to budget, not a reason to omit the collection deliverable.

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

- **Plan-scale rollout data collection and independent report validation remain open.** Baseline CSV checksums and row counts match their manifests, but the score/frequency summaries have not been independently recomputed. The collector resumes only when all data-generation parameters match its checkpoint. The 20k run remains unscheduled until its estimated compute budget is explicitly approved; retain data, manifests, and analysis artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
