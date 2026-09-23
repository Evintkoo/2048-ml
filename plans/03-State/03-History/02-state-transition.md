# State Transition — Supervised Row

> **Canonical row:** `(from_state: [f64;27], action: u8)` with `TaskType::MultiClassification`. No RL.

## 1. Structure — Training Fields vs Metadata

```rust
pub struct StateTransition {
    // ── training (only these feed automl) ──
    pub from_state: [f64; 27],  // X — canonical 27
    pub action: u8,             // y — 0..3 ONLY label
    // ── metadata only (logged, never as y) ──
    pub metadata: Option<TransitionMetadata>,
}
pub struct TransitionMetadata {
    pub to_state: [f64; 27],  // state after move
    pub reward: f64,            // score delta
    pub done: bool,             // terminal
}
```

## 2. Recording

```rust
impl GameSimulator {
    pub fn record_transition(&mut self) -> StateTransition {
        let from = create_state_vector(&self.board); // canonical
        let action = self.last_action; // u8 0..3
        self.board.execute_move(action);
        StateTransition {
            from_state: from, action,
            metadata: Some(TransitionMetadata {
                to_state: create_state_vector(&self.board),
                reward: self.board.score as f64 - self.last_score as f64,
                done: self.board.is_game_over(),
            }),
        }
    }
}
```

## 3. Training Dataset — States → Actions Only

```rust
pub fn create_supervised_dataset(transitions: &[StateTransition]) -> TrainingData {
    let states: Vec<[f64;27]> = transitions.iter().map(|t| t.from_state).collect();
    let actions: Vec<u8> = transitions.iter().map(|t| t.action).collect(); // MultiClassification
    TrainingData { states, targets: actions }
}
```

DataFrame conversion flattens `from_state` + `action` only; metadata dropped. Group by `game_id` for `GroupKFold`.

## 4. No RL Replay Buffer

> No `ReplayBuffer`, no sampling. Supervised classification only — flatten directly.

## 5. Validation

```rust
pub fn validate_transition(t: &StateTransition) -> Result<()> {
    if t.action > 3 { return Err("action >3".into()); }
    if !t.from_state.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)) { return Err("bad state".into()); }
    Ok(())
}
```

## 6. Persistence

Save combined dataset via canonical DataFrame → Parquet. See `01-move-history.md` for grouping.
