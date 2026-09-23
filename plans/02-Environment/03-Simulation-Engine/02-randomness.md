# Randomness and Determinism — Canonical Seed Hygiene

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
pub struct SeedManager {
    pub global_seed: u64,        // 42 — canonical; == TrainingConfig::with_random_state(42)
    pub game_seed: u64,          // per-game
    pub training_seed: u64,      // == global_seed
    pub evaluation_seed: u64,    // e.g., 9999 for benchmark — see config yaml below
}
impl SeedManager {
    /// Derive per-game seed — use wrapping_add, NOT wrapping_mul with prime trick
    pub fn game_seed(&self, game_id: usize) -> u64 {
        self.global_seed.wrapping_add(game_id as u64)
        // NOT: wrapping_mul(1_000_000_007 + game_id) — that prime-mul trick risks collisions
        // and is not needed; wrapping_add is collision-free for sequential game_ids and
        // matches canonical seed propagation (see §6)
    }
}
```

> **Global linkage:** `SeedManager.global_seed` **must** equal `TrainingConfig::with_random_state(42)` global seed so that simulation and automl share reproducibility. Change in one must change the other — document in `config.toml`.

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
| Different seed → different games | randomized |
| Cross-platform | same `ChaCha8Rng` output if same Rust/rand_chacha version |
| Cross-version | may vary if `rand_chacha` changes — pin deps |

## 6. Seed Propagation — `wrapping_add` (Not `wrapping_mul`)

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
        game_rng:            seed.wrapping_add(0), // == global
        training_rng:        seed.wrapping_add(1),
        hyperopt_rng:        seed.wrapping_add(2),
        data_sampling_rng:   seed.wrapping_add(3),
        cross_validation_rng:seed.wrapping_add(4),
    }
}
/// Why wrapping_add not wrapping_mul: prime-mul trick (e.g., wrapping_mul(1_000_000_007)) creates
/// non-sequential, harder-to-debug streams and potential collisions on overflow; wrapping_add gives
/// trivially distinct, sequential streams per component and stable per-game derivation via game_seed() above.
/// Used in SimulationConfig.threads / CrossValidator with_random_state — see 05-Model/04-Evaluation/02-cross-validation.md
```

## 7. Seed Configuration

```yaml
# config.yaml — global seed canonical 42 (links SeedManager + TrainingConfig::with_random_state(42))
seeds:
  global: 42
  evaluation:
    test_set: 9999
    benchmark: 8888
  components: # derived via wrapping_add
    training: 43        # global+1
    hyperopt: 44        # global+2
    cv: 46              # global+4 → CrossValidator::with_random_state(46) or 42 — document consistently
```

```rust
// Link to automl training — canonical
let config = TrainingConfig::default().with_random_state(42); // == SeedManager.global_seed
// For CV reproducibility:
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 }).with_random_state(42);
```

## 8. Rayon Thread Count — Determinism Note

- **Fix threads** for full determinism: `SimulationConfig.threads = 1` and `config.n_jobs = Some(1)` in TrainingConfig (see `01-simulation-engine.md` §7).
- If `parallel:true` with `rayon`, **fix thread pool size** (`RAYON_NUM_THREADS=...`) — varying counts change scheduling and can affect game ordering unless seeds are per-game (they are via `game_seed(game_id)`).
- Preferred MVP: parallel with per-game seeded RNG (`ChaCha8Rng::seed_from_u64(global.wrapping_add(game_id))`) so order does not matter; still log `threads` in `SimulationMetrics`.

## 9. Checklist

- [ ] No `thread_rng()` — only `ChaCha8Rng::seed_from_u64`
- [ ] `spawn_prob_4: f64 = 0.1` exact 90/10
- [ ] `SeedManager.global_seed == TrainingConfig::with_random_state(42)` (42 canonical)
- [ ] Per-game `game_seed = global.wrapping_add(game_id)` — not `wrapping_mul` prime trick
- [ ] Component seeds via `wrapping_add` (§6)
- [ ] Rayon thread count fixed/logged
- [ ] CV folds deterministic via `with_random_state` + `GroupKFold` with `groups=game_id` (vs `TimeSeriesSplit` for temporal)

## 10. Cross-References

- **SimulatorConfig / GameSimulator:** `01-Game/01-game-engine.md` (§4.3 spawn, §6 config) & `03-Simulation-Engine/01-simulation-engine.md`
- **Training seed:** `01-Infrastructure/02-Configuration/02-training-config.md` §6 (`with_random_state(42)`)
- **Board layout / features:** `01-Game/03-board-representation.md` (`[u32;16]`), `03-State/01-Board/01-board-state.md` (`/32768`, index 21 `/6.0`)
- **CV:** `05-Model/04-Evaluation/02-cross-validation.md` (`GroupKFold` vs `TimeSeriesSplit`)
