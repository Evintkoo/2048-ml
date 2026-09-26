# Plan 03 — Board Representation: the repository status is explicit and evidence based

> **Status: DONE (2026-09-26).** `[u32;16]` representation and directional transforms audited; merge-position coverage added.

**Goal:** State the current implementation and evidence boundary for board representation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: `[u32;16]` representation and directional transforms audited; merge-position coverage added.

> **Scope:** This file covers only board layout `[u32;16]` and geometric transforms that implement moves. Feature engineering is not here — see the canonical state plans.

## 1. Authority

- **Canonical Board:** `03-State/01-Board/01-board-state.md` — `RawBoardState { grid:[u32;16], score:u64, move_count:u64, game_over:bool }` in `src/game_engine/mod.rs`; flat array, `0` = empty
- **Canonical training input:** Plan 00 requires 16 board cells plus current score (17 values), implemented by ticket #034. The former 27-column strategic-feature vector is not part of canonical training.
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
    pub grid: [u32; 16],    // 0 = empty, otherwise a validated power-of-two tile
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
        // spawn handled by GameSimulator with ChaCha8Rng (see [engine core operations §4](01-game-engine.md))
        MoveResult { changed: true, score_gained: s }
    }
}
```

## 4. Raw Grid Normalization — For Feature Input

> This section records the canonical grid encoding `/32768`; the complete state also includes current score as the seventeenth value. Grid values above 32768 may therefore exceed one and are accepted by validators.

```rust
/// Raw 16-dim — grid values normalized by 32768 (canonical)
/// Consistent return type: [f64;16] (NOT [f64;4] nor Vec)
pub fn raw_features(board: &Board) -> [f64; 16] {
    let mut f = [0.0f64; 16];
    for i in 0..16 { f[i] = board.grid[i] as f64 / 32768.0; }
    f
}

/// The canonical model input appends normalized score as value 17; the 16
/// normalized cell values are not extended with heuristic measurements.
```

## 5. Performance Note

- Store the grid as a flat `[u32;16]`. The plain slide path uses fixed-size arrays; detailed move history allocates merge-event vectors.
- **Precomputed move tables / bitboard / SIMD are optional, not MVP** — the engine has no measured optimization workload or performance comparison ([engine performance §7](01-game-engine.md)). Consider only against a declared workload and budget.

## 6. Cross-References

- **Engine spawn + SimulatorConfig:** `01-Game/01-game-engine.md`
- **RNG / seed:** `03-Simulation-Engine/02-randomness.md`
- **Valid moves `would_change`:** `02-Rules/03-valid-moves.md`
- **Offline inspection:** `04-Visualization/01-visualization.md` (this file is NOT visualization)

---

## Verification (definition of done)

1. `test -f plans/02-Environment/01-Game/03-board-representation.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/02-Environment/01-Game/03-board-representation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/01-Game/03-board-representation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/01-Game/03-board-representation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/01-Game/03-board-representation.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/01-Game/03-board-representation.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/01-Game/03-board-representation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/01-Game/03-board-representation.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** `[u32;16]` representation and directional transforms audited; merge-position coverage added. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
