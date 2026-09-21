# Board Representation

## 1. Core Data Structure

```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    /// 4x4 grid, stored as [[Option<u64>; 4]; 4]
    /// None represents an empty cell
    grid: [[Option<u64>; 4]; 4],
    /// Current score (sum of all merges)
    pub score: u64,
    /// Number of moves made
    pub move_count: u64,
    /// Whether the game is over
    pub game_over: bool,
}
```

## 2. Board Layout

```
Position:   (0,0)  (0,1)  (0,2)  (0,3)
            (1,0)  (1,1)  (1,2)  (1,3)
            (2,0)  (2,1)  (2,2)  (2,3)
            (3,0)  (3,1)  (3,2)  (3,3)

Direction mapping:
- Left:  columns decrease (0→3)
- Right: columns increase (3→0)
- Up:    rows decrease (0→3)
- Down:  rows increase (3→0)
```

## 3. Board Operations

### 3.1 Create Board

```rust
impl Board {
    pub fn new() -> Self { ... }
    pub fn with_seed(seed: u64) -> Self { ... }
    pub fn from_grid(grid: [[u64; 4]; 4]) -> Self { ... }
}
```

### 3.2 Board Transformations

```rust
// Rotate board 90° clockwise (for move direction simplification)
fn rotate_clockwise(grid: [[Option<u64>; 4]; 4]) -> [[Option<u64>; 4]; 4];

// Transpose board (swap rows and columns)
fn transpose(grid: [[Option<u64>; 4]; 4]) -> [[Option<u64>; 4]; 4];

// Reverse rows (for right/down moves)
fn reverse_rows(grid: [[Option<u64>; 4]; 4]) -> [[Option<u64>; 4]; 4];
```

### 3.3 Move Implementation

```rust
// All moves reduced to "slide left" + rotate/transpose
impl Board {
    pub fn slide_left(&mut self) -> SlideResult;
    pub fn slide_right(&mut self) { 
        self.transpose(); self.slide_left(); self.transpose(); 
    }
    pub fn slide_up(&mut self) { 
        self.transpose(); self.slide_left(); self.transpose(); 
    }
    pub fn slide_down(&mut self) { 
        self.reverse_rows(); self.slide_left(); self.reverse_rows(); 
    }
}
```

## 4. Board State Features

### 4.1 Raw Features (16 dimensions)

```rust
pub fn raw_features(board: &Board) -> [f64; 16] {
    let mut features = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            features[i * 4 + j] = board.grid[i][j].unwrap_or(0.0) as f64;
        }
    }
    features
}
```

### 4.2 Derived Features (additional features)

```rust
pub struct BoardFeatures {
    pub raw: [f64; 16],           // Flat grid values
    pub score: f64,                // Current score
    pub empty_count: f64,          // Number of empty cells / 16
    pub max_tile: f64,             // Maximum tile value
    pub monotonicity: f64,         // Monotonicity score
    pub smoothness: f64,           // Tile value smoothness
    pub merges_available: f64,     // Possible merges / 16
    pub available_moves: f64,      // Count of valid moves / 4
    pub corner_max: f64,           // Is max tile in corner?
}
```

## 5. Board Serialization

```rust
// JSON format
{"grid": [[2,4,8,16],[32,64,128,256],[512,1024,2048,0],[0,0,0,0]], "score": 4092}

// Binary format (compact)
// 4×4 × 8 bytes = 128 bytes + score + metadata
```

## 6. Board Hashing

```rust
// For deduplication and caching
impl Board {
    pub fn hash(&self) -> u64 {
        xxhash3::xxh3_64(serialize_board(self))
    }
}
```

## 7. Performance Optimization

- Store board as `[u64; 16]` (flat array) with empty = 0
- Pre-compute move tables for faster simulation
- Use bitboard representation for speed (up to 16 bits per cell)
- SIMD operations for batch board processing
