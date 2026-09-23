# Score Tracking

## 1. Purpose

Track scores for **analysis/benchmarking only**. Score is metadata — the supervised label is `action: u8 0..3` (`TaskType::MultiClassification`). ScoreTracker is a metadata logger, not a training input builder.

## 2. Score Structure

```rust
pub struct ScoreTracker {
    pub current_score: u64,
    pub score_history: Vec<ScoreEvent>,
    pub turn_scores: Vec<u64>,
    pub total_merges: u64,
    pub max_tile_ever: u64,
}
pub struct ScoreEvent {
    pub turn: u64,
    pub tile_value: u64,        // merged tile value (e.g., 8 for 4+4→8)
    pub score_gained: u64,        // == tile_value (the merged tile value)
    pub position: (usize, usize),
    pub cumulative_score: u64,
}
```

## 3. Score Recording — Canonical

> **2048 scoring rule:** merging two `value` tiles produces one `2*value` tile; **score gained = `2*value`** which equals the **merged tile value**. Pass `merged_tile_value` (not half).

```rust
impl ScoreTracker {
    /// `merged_value` is the resulting tile (e.g., 8). Score gained == merged_value.
    pub fn record_merge(&mut self, turn: u64, merged_value: u64, position: (usize, usize)) {
        debug_assert!(merged_value.is_power_of_two(), "merged tile must be power of 2");
        let score_gained = merged_value; // NOT value*2 where value is half — use merged value directly
        self.current_score += score_gained;
        self.turn_scores.push(score_gained);
        self.total_merges += 1;
        self.max_tile_ever = self.max_tile_ever.max(merged_value);
        self.score_history.push(ScoreEvent {
            turn, tile_value: merged_value, score_gained, position,
            cumulative_score: self.current_score,
        });
    }
}
```

## 4. Score Metrics (Benchmark Only)

```rust
pub struct ScoreMetrics {
    pub mean_score: f64, pub median_score: u64, pub std_dev_score: f64,
    pub max_score: u64, pub min_score: u64,
    pub percentiles: [u64; 10],
    pub games_above_2048: usize, pub games_above_4096: usize, pub games_above_8192: usize,
}
```

## 5. Training Sample — Action Classification (Not Score Regression)

Score is **never `y`**. Canonical: `27-dim → 4 logits → argmax`.

```rust
pub struct TrainingSample {
    pub state: [f64; 27],
    pub action: u8,          // 0..3 — ONLY label (TaskType::MultiClassification)
    // pub score: u64 — metadata only, sidecar for analysis
}
```

## 6. Reward Structs — For Analysis Only, Not for Supervised automl

> Not used for training. Supervised pipeline uses `(state, action)` classification. Retained for post-hoc analysis — must not be fed into automl.

```rust
// Analysis only — do not feed reward into automl
pub struct Reward { pub immediate_reward: f64, pub survival_reward: f64, pub progress_reward: f64, pub final_reward: f64 }
```

## 7. Score Logging — Analysis Artifact

```rust
// Implemented with finite checks; analysis only
pub fn log_scores(trackers: &[ScoreTracker], path: &str) -> Result<()> {
    for t in trackers { if !t.current_score.is_finite() as f64 { return Err(..); } }
    let mut wtr = csv::Writer::from_path(path)?;
    for t in trackers { wtr.serialize((t.current_score, t.total_merges, t.max_tile_ever))?; }
    wtr.flush()?; Ok(())
}
```

## 8. Score Visualization — Analysis Only

Histogram generation is an analysis helper — not a training step. Implement only if needed for the report.

## 9. Score Quality Checks — Implemented

```rust
pub fn validate_scores(scores: &[u64]) -> Result<()> {
    for &s in scores { if ! (s as f64).is_finite() { return Err("non-finite score".into()); } }
    Ok(())
}
```
