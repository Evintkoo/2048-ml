# Game UI (Optional / Not Required)

## 1. Scope

This document defines the UI layer for the 2048 game. **Note:** The project scope explicitly excludes frontend usage per the initial plan. This document covers any visualization needed for debugging and data inspection.

## 2. Terminal UI (Minimal)

```mermaid
flowchart TD
    subgraph "2048 Board"
        C00["(0,0)"] --- C01["(0,1)"]
        C01 --- C02["(0,2)"]
        C02 --- C03["(0,3)"]
        C10["(1,0)"] --- C11["(1,1)"]
        C11 --- C12["(1,2)"]
        C12 --- C13["(1,3)"]
        C20["(2,0)"] --- C21["(2,1)"]
        C21 --- C22["(2,2)"]
        C22 --- C23["(2,3)"]
        C30["(3,0)"] --- C31["(3,1)"]
        C31 --- C32["(3,2)"]
        C32 --- C33["(3,3)"]
    end
```

> **See canonical `02-Environment/04-Visualization/01-visualization.md` — identical for debug only.** `render_board` duplicated here for historical reasons; do not maintain two copies.

```rust
// Reference only — see canonical render_board in 02-Environment/04-Visualization/01-visualization.md
fn render_board(board: &Board) -> String {
    // Identical to canonical; debug-only terminal rendering
    crate::visualization::render_board(board)
}
```

## 3. Visualization (Debug Only)

```rust
// SVG output for board states
fn render_svg(board: &Board, path: &str) -> Result<()> {
    // Generate SVG file for board visualization
}

// CSV export for game logs
fn export_game_log(result: &GameResult, path: &str) -> Result<()> {
    // Export all moves, scores, board states to CSV
}
```

## 4. Interactive Debug Mode

```rust
// Interactive game for debugging
fn debug_game() {
    loop {
        render_board(&board);
        let direction = read_direction();
        board.execute_move(direction);
        board.spawn_tile();
        if board.is_game_over() { break; }
    }
}
```

## 5. Not Required in Scope

Per the project constraints:
- No web-based frontend
- No browser-based visualization
- No mobile app interface
- All interaction via CLI and data files

The game engine runs headless, producing:
- Game state logs (JSON/CSV)
- Board snapshots (for data collection)
- Training data (for ML models)
