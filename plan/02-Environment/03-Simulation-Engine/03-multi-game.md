# Multi-Game Simulation

## 1. Purpose

Running multiple games in sequence to collect sufficient training data and benchmark different agents.

## 2. Game Sequencing

```rust
pub struct GameSequence {
    pub games: Vec<GameResult>,
    pub total_games: usize,
    pub seed: u64,
    pub agent: AgentType,
}

impl GameSequence {
    pub fn run(&mut self, n: usize) -> &GameSequence {
        for i in 0..n {
            let game = self.run_single_game(i);
            self.games.push(game);
        }
        self
    }
}
```

## 3. Batch Processing

### 3.1 Sequential Mode
```rust
// Run games one at a time
let results = (0..10000).map(|i| run_game(i)).collect();
```

### 3.2 Parallel Mode
```rust
// Run games in parallel using rayon
let results: Vec<GameResult> = (0..10000)
    .into_par_iter()
    .map(|i| run_game_with_seed(seed + i as u64))
    .collect();
```

## 4. Data Accumulation

```rust
pub struct GameDataset {
    pub states: Vec<[f64; 24]>,      // State features
    pub actions: Vec<u8>,            // Actions taken
    pub rewards: Vec<f64>,           // Rewards
    pub scores: Vec<u64>,           // Final scores
    pub metadata: Vec<GameMetadata>, // Game info
}

impl GameDataset {
    pub fn from_games(games: Vec<GameResult>) -> Self {
        let mut dataset = GameDataset::new();
        for game in games {
            dataset.extend_from_game(game);
        }
        dataset
    }
}
```

## 5. Data Collection Pipeline

```
Game 1 → Record State+Action → 
Game 2 → Record State+Action → 
...    → ...               → 
Game N → Final Dataset → Preprocess → Training Data
```

## 6. Sample Size Requirements

| Goal | Minimum Games | Recommended | Confidence Level |
|------|---------------|-------------|-----------------|
| Baseline | 1,000 | 10,000 | 95% |
| Training | 50,000 | 100,000 | 95% |
| Benchmark | 100,000 | 1,000,000 | 99% |
| Statistical Significance | 10,000 | 50,000 | 95% |

## 7. Performance Targets

```rust
pub struct PerformanceTargets {
    pub games_per_second: usize,     // Target: 1000+ games/sec
    pub memory_per_game: usize,      // Target: <1KB per game state
    pub total_time_10k: Duration,    // Target: <10 seconds
}
```

## 8. Progress Tracking

```rust
pub struct ProgressTracker {
    pub games_completed: usize,
    pub total_games: usize,
    pub current_score: u64,
    pub best_score: u64,
    pub avg_score_so_far: f64,
    pub time_elapsed: Duration,
}

// Displayed via indicatif progress bar
```

## 9. Checkpointing

```rust
// Save progress every N games
fn checkpoint(results: &[GameResult], game_count: usize) {
    if game_count % 1000 == 0 {
        save_results(&results[game_count-1000..game_count]);
    }
}
```
