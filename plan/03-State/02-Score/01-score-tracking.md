# Score Tracking

## 1. Purpose

Track and record scores throughout game sessions for model training, evaluation, and benchmarking.

## 2. Score Structure

```rust
pub struct ScoreTracker {
    pub current_score: u64,
    pub score_history: Vec<ScoreEvent>,
    pub turn_scores: Vec<u64>,           // Score gained per turn
    pub total_merges: u64,                 // Total merge operations
    pub max_tile_ever: u64,                // Highest tile seen
}

pub struct ScoreEvent {
    pub turn: u64,
    pub tile_value: u64,               // Value of merged tile
    pub score_gained: u64,               // Score added this merge
    pub position: (usize, usize),        // Location of merge
    pub cumulative_score: u64,           // Running total
}
```

## 3. Score Recording

```rust
impl ScoreTracker {
    pub fn record_merge(&mut self, turn: u64, value: u64, position: (usize, usize)) {
        let score_gained = value * 2;  // Merge of two tiles
        self.current_score += score_gained;
        self.turn_scores.push(score_gained);
        self.total_merges += 1;
        
        self.score_history.push(ScoreEvent {
            turn,
            tile_value: value,
            score_gained,
            position,
            cumulative_score: self.current_score,
        });
    }
}
```

## 4. Score Metrics

```rust
pub struct ScoreMetrics {
    pub mean_score: f64,
    pub median_score: u64,
    pub std_dev_score: f64,
    pub max_score: u64,
    pub min_score: u64,
    pub percentiles: [u64; 10],        // P10, P25, ..., P90, P99
    pub games_above_2048: usize,
    pub games_above_4096: usize,
    pub games_above_8192: usize,
}
```

## 5. Score as Training Target

The score is the primary regression target for the ML model:

```rust
// Training data: (state_features, score) pairs
pub struct TrainingSample {
    pub state: [f64; 25],           // Board state features
    pub target_score: f64,            // Final game score (normalized)
}
```

## 6. Reward Shaping

For reinforcement learning-style training:

```rust
pub struct Reward {
    pub immediate_reward: f64,       // Score gained this turn
    pub survival_reward: f64,          // Small reward for each turn survived
    pub progress_reward: f64,         // Reward for increasing max tile
    pub final_reward: f64,             // Total game score (primary target)
}
```

## 7. Score Logging

```rust
// Log scores to CSV for analysis
fn log_scores(scores: &[ScoreTracker], path: &str) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    for tracker in scores {
        wtr.serialize(tracker)?;
    }
    wtr.flush()?;
}
```

## 8. Score Visualization

Generate score distribution charts:

```rust
fn generate_score_histogram(scores: &[u64], path: &str) -> Result<()> {
    // Create histogram of score distribution
}
```

## 9. Score Quality Checks

```rust
fn validate_scores(scores: &[u64]) -> Result<()> {
    // All scores must be non-negative
    // Score must be sum of tile merges
    // No score inflation
}
```
