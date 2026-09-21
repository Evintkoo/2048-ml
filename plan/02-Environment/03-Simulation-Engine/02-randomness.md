# Randomness and Determinism

## 1. Importance

For reproducible ML training and benchmarking, all randomness must be deterministic and seed-controlled.

## 2. Random Number Generator

```rust
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

// Create deterministic RNG from seed
pub fn create_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}
```

## 3. Seed Strategy

```rust
pub struct SeedManager {
    pub global_seed: u64,        // Master seed
    pub game_seed: u64,          // Per-game seed
    pub training_seed: u64,      // Training seed
    pub evaluation_seed: u64,    // Evaluation seed
}

impl SeedManager {
    pub fn game_seed(&self, game_id: usize) -> u64 {
        // Derive game-specific seed from global seed
        self.global_seed.wrapping_mul(1_000_000_007 + game_id as u64)
    }
}
```

## 4. Tile Spawn Randomness

```rust
impl Board {
    pub fn spawn_tile(&mut self, rng: &mut ChaCha8Rng) {
        let empty_cells = self.get_empty_cells();
        let index = rng.gen_range(0..empty_cells.len());
        let value = if rng.gen_bool(0.9) { 2 } else { 4 };
        self.grid[empty_cells[index].0][empty_cells[index].1] = Some(value);
    }
}
```

## 5. Reproducibility Guarantees

| Property | Guarantee |
|----------|-----------|
| Same seed → Same games | Deterministic |
| Different seed → Different games | Randomized |
| Cross-platform | Same seed = same output |
| Cross-version | May vary (Rust std lib changes) |

## 6. Seed Propagation

```rust
// Propagate seed through all components
fn propagate_seed(seed: u64) -> ComponentSeeds {
    ComponentSeeds {
        game_rng: seed,
        training_rng: seed + 1,
        hyperopt_rng: seed + 2,
        data_sampling_rng: seed + 3,
        cross_validation_rng: seed + 4,
    }
}
```

## 7. Seed Configuration

```yaml
seeds:
  global: 42
  experiments:
    baseline: 100
    gradient_boosting: 200
    random_forest: 300
    neural_network: 400
  evaluation:
    test_set: 9999
    benchmark: 8888
```

## 8. Reproducibility Checklist

- [ ] All RNGs seeded from a master seed
- [ ] No `thread_rng()` used (only seeded generators)
- [ ] Rayon thread count is fixed (or 1 for full determinism)
- [ ] Floating-point operations are deterministic
- [ ] Random tile spawn probabilities are exact
- [ ] Cross-validation folds are deterministic
- [ ] Hyperparameter sampling is deterministic
