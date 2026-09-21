# Board State Definition

## 1. State Representation

The board state is the primary input to the ML model. It must capture all information needed to make optimal decisions.

## 2. Raw Board State

```rust
/// 4×4 grid as flat array, 0 = empty
pub struct RawBoardState {
    pub grid: [u8; 16],           // Tile values (power of 2)
    pub score: u64,                   // Cumulative score
    pub move_count: u64,              // Moves made
    pub game_over: bool,             // Terminal state flag
}
```

## 3. Extended Board State (for ML)

```rust
/// Board state with derived features for ML training
pub struct BoardStateML {
    // Raw grid features (16 dimensions)
    pub grid_values: [f64; 16],
    
    // Derived features (additional dimensions)
    pub empty_count: f64,              // Number of empty tiles / 16
    pub max_tile_log: f64,             // log2(max_tile)
    pub monotonicity: f64,             // Monotonicity score
    pub smoothness: f64,               // Value difference between neighbors
    pub corner_value: f64,             // Value at corner (0-4096)
    pub available_moves: f64,          // Count of valid moves / 4
    pub score_normalized: f64,         // score / max_possible_score
    pub merges_available: f64,         // Number of possible merges / 16
    
    // Total feature dimensions: 16 + 9 = 25
}
```

## 4. State Encoding

### 4.1 Raw Encoding (for automl)

```rust
impl BoardStateML {
    pub fn to_array(&self) -> [f64; 25] {
        let mut arr = [0.0f64; 25];
        arr[0..16].copy_from_slice(&self.grid_values);
        arr[16] = self.empty_count;
        arr[17] = self.max_tile_log;
        arr[18] = self.monotonicity;
        arr[19] = self.smoothness;
        arr[20] = self.corner_value;
        arr[21] = self.available_moves;
        arr[22] = self.score_normalized;
        arr[23] = self.merges_available;
        arr[24] = self.move_count as f64 / 1000.0;
        arr
    }
}
```

### 4.2 Normalization

```rust
fn normalize_feature(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}

fn normalize_log(value: f64) -> f64 {
    (value + 1.0).log2() / 13.0  // Max tile is 2^13 = 8192
}
```

## 5. State History

For sequence modeling, the state includes history:

```rust
pub struct StateWithHistory {
    pub current: BoardStateML,
    pub previous: Option<BoardStateML>,    // Previous state
    pub history: Vec<BoardStateML>,        // Last N states
    pub action_history: Vec<u8>,           // Actions taken
}
```

## 6. State Properties

| Property | Type | Description |
|----------|------|-------------|
| Grid values | [u8; 16] | 4×4 board tile values |
| Score | u64 | Cumulative merge score |
| Empty tiles | u8 | Count of empty cells (0-16) |
| Max tile | u8 | Highest tile value (log scale) |
| Move count | u64 | Total moves made |
| Game over | bool | Terminal state flag |

## 7. State Validation

```rust
impl BoardStateML {
    pub fn validate(&self) -> Result<()> {
        // Grid values must be powers of 2 or 0
        // Score must be non-negative
        // Empty count must be 0-16
        // Game over must be consistent
    }
}
```

## 8. State Persistence

```rust
// Serialize to disk
fn save_state(state: &BoardStateML, path: &str) -> Result<()> {
    let json = serde_json::to_string(state)?;
    std::fs::write(path, json)?;
}
```
