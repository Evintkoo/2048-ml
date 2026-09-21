# Scoring Rules

## 1. Score Definition

The score in 2048 is the cumulative sum of all tile merge values during a game session.

## 2. Score Calculation

```
Score = Σ (merge_value for each merge operation)
```

Example:
- Merge two 128 tiles → Score += 256
- Merge two 256 tiles → Score += 512
- Total score = 256 + 512 = 768

## 3. Score Tracking

```rust
pub struct ScoreTracker {
    pub total_score: u64,
    pub merge_history: Vec<MergeEvent>,
    pub turn_scores: Vec<u64>,     // Score per turn
}

pub struct MergeEvent {
    pub turn: u64,
    pub tile_value: u64,           // Merged tile value
    pub position: (usize, usize),   // Where merge occurred
    pub score_gained: u64,
}
```

## 4. Score as ML Target

For the ML model, the score is the **regression target**:

| Property | Value |
|----------|-------|
| Target column name | `score` |
| Target type | `u64` (unsigned 64-bit integer) |
| Normalization | Log10(score + 1) or raw |
| Range | 0 to theoretical maximum |

## 5. Score Features for Training

The score is both:
1. **The target** the model tries to predict/maximize
2. **A feature** that provides context for the current game state

```rust
pub struct GameFeatures {
    pub score: f64,            // Current cumulative score
    pub score_history: Vec<f64>, // Score after each move
    pub score_delta: f64,      // Score gained in last turn
    pub avg_score_per_move: f64, // Running average
}
```

## 6. Score Normalization

```rust
// Log10 normalization for model training
fn normalize_score(score: u64) -> f64 {
    (score as f64 + 1.0).log10()
}

// Denormalize for evaluation
fn denormalize_score(normalized: f64) -> u64 {
    (10f64.powf(normalized) - 1.0) as u64
}
```

## 7. Reward Signal (For Reinforcement Learning Context)

```rust
pub struct RewardSignal {
    pub immediate_reward: f64,   // Score gained this turn
    pub final_reward: f64,       // Total game score
    pub survival_bonus: f64,     // Penalty for game over
    pub progress_bonus: f64,     // Reward for tile value increase
}
```

## 8. Score Metrics for Evaluation

| Metric | Description | Used For |
|--------|-------------|----------|
| Mean Score | Average across all games | Model comparison |
| Max Score | Best game in batch | Capability test |
| Median Score | Median performance | Robustness |
| Score > 2048 Rate | % of games reaching 2048 | Success rate |
| Score Distribution | Percentiles | Performance analysis |
