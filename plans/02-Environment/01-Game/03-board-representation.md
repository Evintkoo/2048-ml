# Board Representation

> **Scope:** This file covers ONLY board layout `[u32;16]` and geometric transforms that implement moves (`slide_left` via `rotate`/`transpose`). Feature engineering is NOT here — see canonical.

## 1. Authority

- **Canonical Board:** `03-State/01-Board/01-board-state.md` — `RawBoardState { grid:[u32;16], score:u64, move_count:u64, game_over:bool }` — flat array, `0` = empty
- **Canonical Features (27-dim):** `03-State/01-Board/02-feature-extraction.md` + `03-State/01-Board/01-board-state.md` §3–4 (16 raw + 11 derived; score at index 21 `/6.0`)
- **This file:** transforms and memory layout only. **Do not duplicate** `BoardFeatures` 8 derived features — cross-ref above.

## 2. Board Layout

```
Flat index:  0  1  2  3      (row 0)
             4  5  6  7      (row 1)
             8  9 10 11      (row 2)
            12 13 14 15      (row 3)

Direction mapping (canonical):
- Left:  indices decreasing per row (slide toward col 0)
- Right: indices increasing per row
- Up:    indices decreasing per column
- Down:  indices increasing per column
```

```rust
/// Minimal Board — see canonical for authoritative definition
pub struct Board {
    pub grid: [u32; 16],    // 0 = empty, else power of 2 (max 32768; storage up to 131072)
    pub score: u64,
    pub move_count: u64,
    pub game_over: bool,
}
impl Board {
    pub fn new() -> Self { Self { grid: [0;16], score: 0, move_count: 0, game_over: false } }
    pub fn with_seed(seed: u64) -> Self { /* ChaCha8Rng::seed_from_u64(seed); spawn 2 tiles 90/10 */ todo!() }
    pub fn from_grid(grid: [u32;16]) -> Self { todo!() }
}
```

## 3. Board Transforms — All Moves Reduce to `slide_left`

### 3.1 Primitive Transforms

```rust
fn transpose(grid: [u32;16]) -> [u32;16] {
    let mut out = [0u32;16];
    for r in 0..4 { for c in 0..4 { out[c*4+r] = grid[r*4+c]; } }
    out
}
fn reverse_rows(grid: [u32;16]) -> [u32;16] {
    let mut out = grid;
    for r in 0..4 { out[r*4..r*4+4].reverse(); }
    out
}
fn rotate_clockwise(grid: [u32;16]) -> [u32;16] { transpose(reverse_rows(grid)) }
```

### 3.2 Slide — Base Operation

```rust
/// Compress + merge left for a single row — returns new row and score delta
fn slide_row_left(row: [u32;4]) -> ([u32;4], u64) {
    // 1. filter zeros, 2. merge adjacent equals once, 3. pad zeros right, 4. sum merges
    todo!()
}
fn slide_left(grid: [u32;16]) -> ([u32;16], u64) {
    let mut out = [0u32;16];
    let mut gained = 0u64;
    for r in 0..4 {
        let row = [grid[r*4], grid[r*4+1], grid[r*4+2], grid[r*4+3]];
        let (new_row, s) = slide_row_left(row);
        out[r*4..r*4+4].copy_from_slice(&new_row);
        gained += s;
    }
    (out, gained)
}

/// Other directions — thin wrappers around slide_left
impl Board {
    pub fn slide_left(&mut self) -> u64 { let (g,s)=slide_left(self.grid); self.grid=g; s }
    pub fn slide_right(&mut self) -> u64 {
        self.grid = reverse_rows(self.grid);
        let s = self.slide_left();
        self.grid = reverse_rows(self.grid); s
    }
    pub fn slide_up(&mut self) -> u64 {
        self.grid = transpose(self.grid);
        let s = self.slide_left();
        self.grid = transpose(self.grid); s
    }
    pub fn slide_down(&mut self) -> u64 {
        self.grid = transpose(self.grid);
        let s = self.slide_right();
        self.grid = transpose(self.grid); s
    }
}
```

### 3.3 Canonical Delegation

```rust
impl Board {
    pub fn execute_move(&mut self, dir: Direction) -> MoveResult {
        let s = match dir {
            Direction::Left => self.slide_left(),
            Direction::Right => self.slide_right(),
            Direction::Up => self.slide_up(),
            Direction::Down => self.slide_down(),
        };
        self.score += s;
        // spawn handled by GameSimulator with ChaCha8Rng — see 01-game-engine.md §4.3
        MoveResult { changed: true, score_gained: s }
    }
}
```

## 4. Raw Grid Normalization — For Feature Input

> Derived features (`empty_count`, `max_tile_log`, `monotonicity`, etc.) are **NOT** defined here. See `03-State/01-Board/02-feature-extraction.md` — 27-dim includes score at index 21 `log10(score+1)/6.0`. Canonical grid normalization is `/32768`.

```rust
/// Raw 16-dim — grid values normalized by 32768 (canonical)
/// Consistent return type: [f64;16] (NOT [f64;4] nor Vec)
pub fn raw_features(board: &Board) -> [f64; 16] {
    let mut f = [0.0f64; 16];
    for i in 0..16 { f[i] = board.grid[i] as f64 / 32768.0; }
    f
}

/// Full 27-dim is raw_features + 11 derived — see BoardStateML::to_array() in 03-State/01-Board/01-board-state.md §4.1
/// Do not re-define BoardFeatures here.
```

## 5. Performance Note

- Store as `[u32;16]` flat; zero-alloc transforms above.
- **Precomputed move tables / bitboard / SIMD are Optional, not MVP** — see `01-game-engine.md` §7. Do not implement before 10k baseline.

## 6. Cross-References

- **Engine spawn + SimulatorConfig:** `01-Game/01-game-engine.md`
- **RNG / seed:** `03-Simulation-Engine/02-randomness.md`
- **Valid moves `would_change`:** `02-Rules/03-valid-moves.md`
- **Visualization (headless JSON export):** `04-Visualization/01-visualization.md` (this file is NOT visualization)
