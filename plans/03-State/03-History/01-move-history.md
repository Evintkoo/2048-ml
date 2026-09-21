# Move History

## 1. Purpose

Track all moves made during a game to enable sequence analysis and training data generation.

## 2. Move Record Structure

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct MoveRecord {
    pub turn: u64,                       // Move number
    pub direction: Direction,            // Direction of move
    pub board_before: [u32; 16],         // Board state before move (u32 stores tile values up to 131072)
    pub board_after: [u32; 16],          // Board state after move (u32 stores tile values up to 131072)
    pub score_before: u64,               // Score before move
    pub score_after: u64,                // Score after move
    pub merge_value: Option<u64>,        // Value of merged tile (if any)
    pub valid: bool,                     // Whether move changed board
    pub timestamp: DateTime<Utc>,        // When move was made
}
```

## 3. Move History Collection

```rust
pub struct MoveHistory {
    pub records: Vec<MoveRecord>,
    pub game_id: Uuid,
    pub agent_type: AgentType,
    pub seed: u64,
}

impl MoveHistory {
    pub fn add_record(&mut self, record: MoveRecord) {
        self.records.push(record);
    }
    
    pub fn to_dataframe(&self) -> DataFrame {
        // Convert to polars DataFrame for automl training
    }
}
```

## 4. Move Statistics

```rust
pub struct MoveStatistics {
    pub total_moves: usize,
    pub valid_moves: usize,
    pub invalid_moves: usize,
    pub avg_moves_per_game: f64,
    pub move_direction_distribution: [usize; 4],  // Up, Down, Left, Right
    pub longest_sequence: usize,      // Longest consecutive valid moves
}
```

## 5. Move Patterns Analysis

```rust
// Analyze frequently used move sequences
pub fn analyze_move_patterns(history: &[MoveRecord]) -> MovePatternAnalysis {
    // Most common move sequences
    // Transition probabilities (Markov chain)
    // State-action pairs frequency
}
```

## 6. Move History for Training — Supervised Classification Only

The move history provides training samples — **states map to actions only** (`TaskType::MultiClassification`). No RL tuple.

```rust
pub struct TrainingSampleFromHistory {
    pub state: [f64; 27],           // Board features at time t (27-dim)
    pub action: u8,                   // Supervised label: Direction taken (0-3) — ONLY target
    // Optional metadata for analysis only (never as `y`):
    // pub score: u64,              // Score metadata — not a label
}
// DataFrame row: state_features: [f64;27], action: u8, score: u64 (metadata)
```

## 7. Move History Serialization

```rust
// Save to Parquet for large datasets
pub fn save_move_history(history: &MoveHistory, path: &str) -> Result<()> {
    let df = history.to_dataframe();
    df.write_parquet(path, &ParquetWriteOptions::default())?;
}
```

## 8. Move History Quality Checks

```rust
pub fn validate_history(history: &MoveHistory) -> Result<()> {
    // Each move must be valid (changed the board)
    // Turn numbers must be sequential
    // Score must be monotonically non-decreasing
    // Board states must be valid
}
```
