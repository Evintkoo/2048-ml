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

## 4. Score Is Evaluation Metric Only — NOT a Training Target

Score is **never the supervised training label**. The canonical task is `TaskType::MultiClassification` with:

| Property | Value |
|----------|-------|
| Training label (target) column | `action` (`u8`, values 0–3: Up/Down/Left/Right) |
| Input features | 27-dim vector (`[f64; 27]` → 4 logits → `argmax`) |
| Score column role | **Metadata / evaluation only** — if a `score` (`u64`) column is stored in CSV/Parquet it is for post-hoc analysis and benchmarking (mean/median score, score distribution), never as `y` for automl |

> **No regression target.** Do not use `score` as `y`, do not fit `Log10(score)` as a regression target, and do not evaluate the model with regression metrics (R² / RMSE / MAE) on score predictions — the model predicts **actions**, not scores.

## 5. Score Features for Training

The score is **a feature** that provides context for the current game state (not a target):

```rust
pub struct GameFeatures {
    pub score: f64,            // Current cumulative score
    pub score_history: Vec<f64>, // Score after each move
    pub score_delta: f64,      // Score gained in last turn
    pub avg_score_per_move: f64, // Running average
}
```

## 6. Score Normalization

Score normalization is for **feature engineering only** (index 21 of the 27-dim feature vector), not for a regression target:

```rust
// Log10 normalization for the score *feature* (index 21), not a target
fn normalize_score(score: u64) -> f64 {
    (score as f64 + 1.0).log10() / 6.0  // canonical: /6.0
}

// Denormalize only for display/analysis — never as model output
fn denormalize_score(normalized: f64) -> u64 {
    (10f64.powf(normalized * 6.0) - 1.0) as u64
}
```

## 7. Reward Signal — Not Used (RL Out of Scope)

> **Supervised classification only.** The project does not train with reinforcement learning. The struct below is retained for historical reference and must be labeled as **for analysis only, not for supervised automl** — do not use `reward`/`next_state`/`done` as training signals. The supervised row is `(state_features: [f64;27], action: u8)` with optional `score: u64` metadata.

```rust
// NOT USED for training — for analysis/reference only, not for supervised automl
pub struct RewardSignal {
    pub immediate_reward: f64,   // Score gained this turn — analysis only
    pub final_reward: f64,       // Total game score — analysis only
    pub survival_bonus: f64,     // Penalty for game over — analysis only
    pub progress_bonus: f64,     // Reward for tile value increase — analysis only
}
```

## 8. Score Metrics for Evaluation

| Metric | Description | Used For |
|--------|-------------|----------|
| Mean Score | Average across all games | Primary ranking metric |
| Max Score | Best game in batch | Capability ceiling |
| Median Score | Median performance | Robustness |
| Score Distribution | Percentiles | Model comparison |
| Score > Heuristic Rate | % of games exceeding heuristic (~512) | Model quality |
