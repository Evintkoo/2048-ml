# Board State Definition

## 1. State Representation

The board state is the primary input to the ML model. It must capture all information needed to make optimal decisions.

## 2. Raw Board State

```rust
/// 4×4 grid as flat array, 0 = empty
pub struct RawBoardState {
    pub grid: [u32; 16],          // Tile values (powers of 2, stores tile values up to 131072)
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
    
    // Derived features (additional dimensions) — 11 in canonical order
    pub empty_count: f64,              // Number of empty tiles / 16
    pub max_tile_log: f64,             // log2(max_tile)
    pub monotonicity: f64,             // Monotonicity score
    pub smoothness: f64,               // Value difference between neighbors
    pub merges_available: f64,         // Number of possible merges / 16
    pub score_normalized: f64,         // log10(score+1)/6.0 — canonical score norm
    pub adjacency_merge_score: f64,    // Sum of adjacent mergeable pairs / 16
    pub corner_max: f64,               // Max tile in corner (normalized)
    pub edge_tiles_occupied: f64,      // Edge tiles occupied / 12
    pub col_worst: f64,                // Minimum column sum (normalized)
    pub row_worst: f64,                // Minimum row sum (normalized)
    
    // Total feature dimensions: 16 + 11 = 27
}
```

### 3.1 New Features: Adjacency and Merge-Potential

The original feature set lacked **spatial information** about which tiles can merge. Two critical features are added:

**`adjacency_merge_score`**: Sum of all adjacent tile pairs (horizontally and vertically) where both tiles have equal non-zero values. This captures merge potential not fully captured by `merges_available`.

```rust
pub fn adjacency_merge_score(board: &Board) -> f64 {
    let mut score = 0.0;
    for i in 0..4 {
        for j in 0..4 {
            let val = board.grid[i][j].unwrap_or(0);
            if val == 0 { continue; }
            if j < 3 {
                let right = board.grid[i][j+1].unwrap_or(0);
                if right == val { score += val as f64; }
            }
            if i < 3 {
                let bottom = board.grid[i+1][j].unwrap_or(0);
                if bottom == val { score += val as f64; }
            }
        }
    }
    score / 16.0
}
```

**`col_worst` / `row_worst`**: The minimum column/row sum, normalized by max possible column sum (8192). These capture board imbalance — a key strategic signal in 2048.

```rust
pub fn column_worst(board: &Board) -> f64 {
    let mut min_sum = u64::MAX;
    for j in 0..4 {
        let sum: u64 = (0..4).map(|i| board.grid[i][j].unwrap_or(0)).sum();
        min_sum = min_sum.min(sum);
    }
    min_sum as f64 / 8192.0
}
```

## 4. State Encoding

### 4.1 Raw Encoding (for automl)

```rust
impl BoardStateML {
    pub fn to_array(&self) -> [f64; 27] {
        let mut arr = [0.0f64; 27];
        arr[0..16].copy_from_slice(&self.grid_values);
        arr[16] = self.empty_count;
        arr[17] = self.max_tile_log;
        arr[18] = self.monotonicity;
        arr[19] = self.smoothness;
        arr[20] = self.merges_available;
        arr[21] = self.score_normalized; // log10(score+1)/6.0
        arr[22] = self.adjacency_merge_score;
        arr[23] = self.corner_max;
        arr[24] = self.edge_tiles_occupied;
        arr[25] = self.col_worst;
        arr[26] = self.row_worst;
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
    (value + 1.0).log2() / 15.0  // Max tile is 2^15 = 32768
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
| Grid values | [u32; 16] | 4×4 board tile values |
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
