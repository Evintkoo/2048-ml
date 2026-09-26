# Plan 03 — Multi-Game Simulation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Fixed-thread collection, metadata, manifests, batched checkpoints/resume, and live progress are implemented. Plan-scale rollout collection and report validation remain pending.

**Goal:** State the current implementation and evidence boundary for multi-game simulation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The random and heuristic baselines and whole-game bootstrap frequency intervals are complete for the measured 10,000-game-per-agent protocol. Rollout collection now checkpoints bounded game batches and can resume after interruption; no plan-scale rollout corpus has been collected yet.

> Sample sizes are chosen from the study question, observed variation, desired precision or power, and available compute budget; there is no universal game-count minimum. Supervised only: the canonical state has 17 values under Plan 00, now used by the root collector. No `rewards` are used.

## 1. Purpose

Run N games sequentially/parallel to collect sufficient supervised rows for automl `TaskType::MultiClassification`. Headless only; the root collector writes CSV plus a metadata sidecar.

## 2. Game Sequencing

```rust
pub struct GameSequence {
    pub games: Vec<GameResult>,
    pub total_games: usize, // selected for the study and compute budget
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
// Example: choose n from the study protocol and available budget (see §6).
let results: Vec<GameResult> = (0..n)
    .map(|i| run_game_with_seed(seed.wrapping_add(i as u64)))
    .collect();
```

### 3.2 Parallel (rayon, deterministic if seeded per-game)

```rust
use rayon::prelude::*;
let results: Vec<GameResult> = (0..n)
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

## 6. Sample Size — Study-Specific and Budgeted

| Goal | Sample-size basis | Evidence / note |
|------|-------------------|-----------------|
| Baseline / benchmark | Protocol-specific precision and budget | The retained random/heuristic action-frequency study used 10,000 games per agent; this is the completed protocol, not a universal minimum. |
| Training | Learning-curve and label-quality study, plus budget | No training-corpus size has been established. The two-game collection pilot is only a throughput smoke. |
| Evaluation intervals or tests | Target estimand, observed variation, precision/power, and dependence structure | No game count guarantees a particular confidence interval width or p-value. See `07-Benchmarking/` for protocol-specific analysis. |

> Throughput and total runtime must be measured for the configured workload; do not treat `games_per_second: 1000` or `total_time_10k` as acceptance targets.

## 7. Progress Tracking

The root collector displays completed games, elapsed time, and estimated time remaining. Updates arrive as each parallel game finishes. Average score is not currently displayed by the collector; the fields below are an illustrative design sketch, not its implemented progress API.

```rust
pub struct ProgressTracker {
    pub games_completed: usize,
    pub total_games: usize,       // configured study size
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
cargo run --release -- data-collector collect --n-games <n_games> --rollouts 100 --threads 4 \
  --checkpoint-every 1000 --output data/raw/policy.csv
# After interruption, rerun the same arguments and add --resume.
# Select the game count only after setting a study goal and compute budget.
# The retained two-game pilot projects ~228 hours for 20,000 games by linear
# extrapolation; this estimate is highly uncertain and is not a runtime promise.
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
- Artifact validation (2026-09-27): both baseline CSVs contain 10,000 game rows plus a header and match their manifest SHA-256. Independent recomputation from their `score` columns reproduced manifest means (random 1,094.124; heuristic 8,056.2324) and medians (1,050; 7,136). These CSVs do not retain per-game action counts, so pooled action totals and game-cluster bootstrap intervals cannot be independently recomputed from the archived data. The published frequencies and intervals remain manifest-reported results.
- Collection throughput evidence: [`reports/collection_pilots/2026-09-27/README.md`](../../../reports/collection_pilots/2026-09-27/README.md) documents a resumed two-game pilot (285 rows, 97,300 rollout evaluations, 82.23 seconds). Its linear projection is approximately 228 hours for 20,000 games and is highly uncertain; the prior 103-hour estimate is superseded. A larger pilot and declared compute budget precede plan-scale collection.
- Validation: deterministic batch coverage and checkpoint/resume tests pass. The Planout checker is part of the ticket verification.

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

- **Plan-scale rollout data collection and complete independent report validation remain open.** Baseline score means and medians were recomputed from the raw score columns, but action counts/bootstrap intervals cannot be reconstructed without per-game action-count data. The collector resumes only when all data-generation parameters match its checkpoint. A 20k run remains unscheduled pending a more reliable runtime estimate and declared compute budget; retain data, manifests, and analysis artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
