# State Transition Tracking

## 1. Purpose

Track how board states transition from one move to the next for sequence modeling and training data generation.

## 2. State Transition Structure — Supervised Row Is `from_state → action` Only

```rust
pub struct StateTransition {
    pub from_state: [f64; 27],       // State before move — used as X
    pub action: u8,                    // Direction taken (0-3) — the ONLY supervised label (y)
    pub to_state: [f64; 27],         // State after move — metadata only, not for training
    pub reward: f64,                   // Score change — metadata/analysis only, not for supervised automl
    pub done: bool,                    // Game over flag — metadata only, not for training
}
// Canonical supervised row: (from_state: [f64;27], action: u8) with TaskType::MultiClassification
// `to_state`/`reward`/`done` must not be fed as targets or RL signals
```

## 3. Transition Recording — Collect `(from_state, action)` for Classification

```rust
impl GameSimulator {
    pub fn record_transition(&mut self) -> StateTransition {
        let from = self.board.to_encoding();
        let action = self.last_action;
        self.board.execute_move(action);
        let to = self.board.to_encoding();
        // Reward/done computed only as metadata for analysis, not for training
        let reward = self.board.score as f64 - self.last_score as f64;

        StateTransition {
            from_state: from,
            action,                   // ONLY supervised label
            to_state: to,             // metadata only
            reward,                   // metadata only — not used as `y`
            done: self.board.is_game_over(), // metadata only
        }
    }
}
```

## 4. Transition Dataset

```rust
pub struct TransitionDataset {
    pub transitions: Vec<StateTransition>,
    pub total_transitions: usize,
}

impl TransitionDataset {
    // Convert to automl-compatible format
    pub fn to_dataframe(&self) -> DataFrame { ... }
    
    // Split into train/val/test
    pub fn split(&self, train_ratio: f64, val_ratio: f64) -> [Vec<StateTransition>; 3] { ... }
}
```

## 5. Transition Analysis — Classification-Focused

```rust
// Analyze state distribution — classification focus; reward/terminal are metadata only
pub fn analyze_transitions(transitions: &[StateTransition]) -> TransitionAnalysis {
    // State visitation frequency
    // Action distribution (class balance across 0..3) — primary
    // Per-class F1 / confusion — primary
    // Reward distribution — metadata only, not a metric
    // Terminal state frequency — metadata only
}
```

## 6. Transition Collection — No Replay Buffer (Supervised Only)

> **No ReplayBuffer / RL sampling.** The pipeline is supervised classification (`state → action`). Transitions are flattened directly into a DataFrame of `(state_features, action)` rows. Buffer sampling logic is removed.

```rust
// No ReplayBuffer — collect transitions directly into a supervised dataset
// Each StateTransition's `from_state` + `action` becomes one supervised row;
// `reward`/`to_state`/`done` are not used for training (may be logged as metadata only).
```

## 7. State Transition for automl Training — States → Actions Only

```rust
// Create supervised learning dataset from transitions — maps states to actions only
pub fn create_supervised_dataset(transitions: &[StateTransition]) -> TrainingData {
    let states: Vec<[f64; 27]> = transitions.iter().map(|t| t.from_state).collect();
    let actions: Vec<u8> = transitions.iter().map(|t| t.action).collect(); // u8 0..3, MultiClassification

    TrainingData { states, targets: actions } // targets are actions, not reward/next_state
}
```

## 8. Transition Validation

```rust
pub fn validate_transition(transition: &StateTransition) -> Result<()> {
    // from_state must be [f64;27] with no NaN
    // Action must be 0-3 (valid classification label)
    // to_state/reward/done are metadata only — validate only if present
}
```

## 9. Transition Data Persistence

```rust
// Save transitions to Parquet
pub fn save_transitions(transitions: &[StateTransition], path: &str) -> Result<()> {
    let df = create_dataframe(transitions)?;
    df.write_parquet(path, &ParquetWriteOptions::default())?;
}
```
