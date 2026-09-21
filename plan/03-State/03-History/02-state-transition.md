# State Transition Tracking

## 1. Purpose

Track how board states transition from one move to the next for sequence modeling and training data generation.

## 2. State Transition Structure

```rust
pub struct StateTransition {
    pub from_state: [f64; 25],       // State before move
    pub action: u8,                    // Direction taken (0-3)
    pub to_state: [f64; 25],         // State after move
    pub reward: f64,                   // Score change or final score
    pub done: bool,                    // Game over flag
}
```

## 3. Transition Recording

```rust
impl GameSimulator {
    pub fn record_transition(&mut self) -> StateTransition {
        let from = self.board.to_encoding();
        let action = self.last_action;
        self.board.execute_move(action);
        let to = self.board.to_encoding();
        let reward = self.board.score as f64 - self.last_score as f64;
        
        StateTransition {
            from_state: from,
            action,
            to_state: to,
            reward,
            done: self.board.is_game_over(),
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

## 5. Transition Analysis

```rust
// Analyze state distribution
pub fn analyze_transitions(transitions: &[StateTransition]) -> TransitionAnalysis {
    // State visitation frequency
    // Action distribution
    // Reward distribution
    // Terminal state frequency
}
```

## 6. Replay Buffer (for RL-style training)

```rust
pub struct ReplayBuffer {
    pub buffer: Vec<StateTransition>,
    pub capacity: usize,
}

impl ReplayBuffer {
    pub fn push(&mut self, transition: StateTransition) { ... }
    pub fn sample(&self, batch_size: usize) -> Vec<StateTransition> { ... }
    pub fn len(&self) -> usize { self.buffer.len() }
}
```

## 7. State Transition for automl Training

```rust
// Create supervised learning dataset from transitions
pub fn create_supervised_dataset(transitions: &[StateTransition]) -> TrainingData {
    let states: Vec<[f64; 25]> = transitions.iter().map(|t| t.from_state).collect();
    let actions: Vec<f64> = transitions.iter().map(|t| t.action as f64).collect();
    
    TrainingData { states, targets: actions }
}
```

## 8. Transition Validation

```rust
pub fn validate_transition(transition: &StateTransition) -> Result<()> {
    // from_state and to_state must have same dimensions
    // Action must be 0-3
    // Reward must be finite
    // States must be valid (no NaN)
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
