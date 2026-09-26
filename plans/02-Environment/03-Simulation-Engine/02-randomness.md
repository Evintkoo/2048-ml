# Plan 02 — Randomness and Determinism: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** `SeedManager` derives CLI game, training, HyperOptX, data-sampling, CV, and analysis seeds; general config loading and broader process-level reproducibility checks remain pending.

**Goal:** State the current implementation and evidence boundary for randomness and determinism.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats seed derivation as implemented in root workflows with bounded evidence.** `src/seeds.rs` provides one `SeedManager` constructed from the CLI's global seed. Game IDs use wrapping addition with `game_id`; final AutoML fit uses the global seed, while tuning, data relabeling, CV, and analysis use documented offsets. The HyperOptX search JSON does not configure general seeds. Framework internals may retain nondeterminism.

> **Canonical RNG:** `ChaCha8Rng::seed_from_u64(seed)` per game; global seed linked to **Evintkoo/automl `TrainingConfig::with_random_state(42)`** (see `01-Infrastructure/02-Configuration/02-training-config.md` §6). Spawn 90/10 via `spawn_prob_4:0.1`. Headless only.

## 1. Why Deterministic

Reproducible 10k+ game evaluation, `GroupKFold` group integrity (`game_id`), and automl `TrainingConfig` reproducibility require seeded RNG everywhere — no `thread_rng()`.

## 2. RNG

```rust
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

pub fn create_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}
/// Also used for automl: TrainingConfig::default().with_random_state(42) — canonical global seed
/// See 01-Infrastructure/02-Configuration/02-training-config.md §6: config.random_seed = Some(42) / builder with_random_state(42)
```

## 3. Seed Strategy — Linked to TrainingConfig

```rust
let seeds = SeedManager::new(cli_seed);
let game_seed = seeds.game_seed(game_id as u64);
let training_seed = seeds.training_seed(); // == cli_seed
let cv_seed = seeds.cross_validation_seed();
```

> **Global linkage:** `SeedManager::training_seed()` returns the user-provided global seed used by `TrainingConfig::with_random_state`. Changing the CLI `--seed` changes game and training seeds together.

## 4. Tile Spawn (90/10) — Uses Same RNG

```rust
impl Board {
    pub fn spawn_tile(&mut self, rng: &mut ChaCha8Rng, spawn_prob_4: f64) {
        let empty: Vec<usize> = self.grid.iter().enumerate().filter(|(_,v)| **v==0).map(|(i,_)|i).collect();
        if empty.is_empty() { return; }
        let idx = empty[rng.gen_range(0..empty.len())];
        let val = if rng.gen_bool(spawn_prob_4) { 4 } else { 2 }; // 0.1 → 90/10
        self.grid[idx] = val;
    }
}
/// Initial board: two spawns with spawn_prob_4=0.1 — see 01-Game/01-game-engine.md §4.3 & §6 SimulatorConfig { seed:42, spawn_prob_4:0.1 }
```

## 5. Reproducibility Guarantees

| Property | Guarantee |
|----------|-----------|
| Same seed → same games | deterministic (`ChaCha8Rng` seeded) |
| Different seed → different games | normally different; collisions are possible |
| Cross-platform | same `ChaCha8Rng` output if same Rust/rand_chacha version |
| Cross-version | may vary if `rand_chacha` changes — pin deps |

## 6. Seed Propagation — `wrapping_add` offsets

```rust
pub struct ComponentSeeds {
    pub game_rng: u64,
    pub training_rng: u64,
    pub hyperopt_rng: u64,
    pub data_sampling_rng: u64,
    pub cross_validation_rng: u64,
}
pub fn propagate_seed(seed: u64) -> ComponentSeeds {
    ComponentSeeds {
        game_rng:             seed, // per-game derivation adds game ID
        training_rng:         seed, // final-fit seed matches the training-config contract
        hyperopt_rng:        seed.wrapping_add(2),
        data_sampling_rng:   seed.wrapping_add(3),
        cross_validation_rng:seed.wrapping_add(4),
    }
}
/// Score-summary analysis uses global+1; action-frequency bootstrap uses global+2.
/// Pairwise comparison bootstraps use global+6+pair_index. RolloutLabeler further
/// derives deterministic seeds from game/move/action/rollout IDs.
```

The implementation is in `src/seeds.rs`; `src/main.rs` uses it for CLI workflows and `src/game_engine/mod.rs` uses it for batch game derivation. `TrainingConfig::with_random_state(seeds.training_seed())` receives the same global seed exposed by `--seed`.

## 7. Seed Configuration

```yaml
# Illustrative values only; root CLI currently accepts seeds through --seed.
seeds:
  global: 42
  evaluation:
    test_set: 9999
    benchmark: 8888
  components: # derived via wrapping_add
    training: 42        # global seed, matching the training-config contract
    hyperopt: 44        # global+2
    cv: 46              # global+4, used by CrossValidator
```

```rust
// Link to automl training — canonical
let seeds = SeedManager::new(42);
let config = TrainingConfig::default().with_random_state(seeds.training_seed());
// For CV reproducibility:
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
    .with_random_state(seeds.cross_validation_seed());
```

## 8. Rayon Thread Count — Determinism Note

- Collection uses the CLI `--threads` value to build a fixed-size Rayon pool; the value is retained in the collection manifest. The number of threads does not change game seeds or collected ordering.
- HyperOptX tuning currently runs serially (`n_jobs = 1`). AutoML fitting may retain framework-level nondeterminism despite fixed seeds, as documented in the framework audit.
- Preferred MVP: parallel with per-game seeded RNG (`ChaCha8Rng::seed_from_u64(global.wrapping_add(game_id))`) so order does not matter; still log `threads` in `SimulationMetrics`.

## 9. Checklist

- [x] Root game randomness uses seeded `ChaCha8Rng`; no `thread_rng()` call exists in root implementation.
- [x] `spawn_prob_4` defaults to 0.1 and is validated.
- [x] `SeedManager::training_seed() == global_seed`, supplied to `TrainingConfig::with_random_state`.
- [x] Per-game seed is `global.wrapping_add(game_id)`.
- [x] Root component seeds use the documented `wrapping_add` offsets (§6).
- [x] Collector thread count is fixed per run and recorded in its manifest.
- [x] Grouped CV receives the derived seed; temporal splitting remains a separate strategy.

## 10. Cross-References

- **SimulatorConfig / GameSimulator:** `01-Game/01-game-engine.md` (§4.3 spawn, §6 config) & `03-Simulation-Engine/01-simulation-engine.md`
- **Training seed:** `01-Infrastructure/02-Configuration/02-training-config.md` §6 (`with_random_state(42)`)
- **Board layout / input:** `01-Game/03-board-representation.md` (`[u32;16]`); Plan 00 defines 16 cells plus score (17 values), implemented by ticket #034. Grid uses `/32768`; normalized score is at index 16.
- **CV:** `05-Model/04-Evaluation/02-cross-validation.md` (`GroupKFold` vs `TimeSeriesSplit`)

## Implementation Record

- Games use `ChaCha8Rng::seed_from_u64`; initial tiles, random actions, and spawned tiles share the per-game RNG. Four-tile spawn probability is configurable and defaults to 0.1.
- `SeedManager` centralizes root CLI seed derivation; final-fit uses the global seed, while HyperOptX, data sampling, CV, and analysis use documented offsets. Game batches use the same manager and do not depend on Rayon scheduling. Collector manifests record seed range and thread count.
- On 2026-09-24, `cargo check`, `cargo clippy -- -D warnings`, formatting validation, and a temporary synthetic-data `train --tune-trials 2` wiring run passed. The emitted run manifest recorded global seed 42 and derived training/HyperOptX/data/CV seeds 42/44/45/46. Synthetic metrics are not research evidence.
- Validation: deterministic same-seed checks and a 10,000-spawn frequency check pass in the root test suite. Cross-platform/version identity is not claimed beyond the pinned dependency versions.

---

## Verification (definition of done)

1. `test -f plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/03-Simulation-Engine/02-randomness.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Root CLI seed derivation is centralized, but config-file loading is absent and AutoML model fitting may remain nondeterministic despite fixed seeds (see [framework architecture audit](../../01-Infrastructure/01-Project/04-framework-architecture.md)).

## Later

- **Resolve framework-level nondeterminism and add config-file seed loading before claiming end-to-end seed reproducibility.** Retain repeated-run artifacts and exact dependency versions.
