# Game UI — Deprecated Stub (Debug-Only Renderer)

> **Status: DEPRECATED STUB — Out of scope (headless simulation only).**
> UI is **not MVP**. This file exists only to document the one debug-only helper.
> **Canonical visualization (headless JSON/SVG/CSV export):** `02-Environment/04-Visualization/01-visualization.md` — that file = structured export; **this file = minimal terminal print for debugging only.**

## Scope

Project is **headless simulation only** per `01-Infrastructure/01-Project/01-project-overview.md` §3 (Out of Scope: Game UI). No web/browser/mobile/interactive UI in MVP. This file is Optional, not MVP — do not extend.

## Single Debug-Only Helper

```rust
/// Debug-only terminal render — for `cargo test` / `println!` inspection only. Not used in training loop.
pub fn render_board(board: &Board) -> String {
    let mut s = String::new();
    s.push_str("+------+------+------+------+\n");
    for r in 0..4 {
        s.push('|');
        for c in 0..4 {
            let v = board.grid[r*4+c];
            if v == 0 { s.push_str("     |"); } else { s.push_str(&format!("{:>5}|", v)); }
        }
        s.push_str(&format!("\n+------+------+------+------+\n"));
    }
    s.push_str(&format!("Score: {}\n", board.score));
    s
}
```

> Deleted: interactive `debug_game()` loop, `render_svg`, `export_game_log` (those belong in `04-Visualization/01-visualization.md` if needed at all). No duplication — single source is there.

## Out-of-Scope (Do Not Implement)

- No web frontend, no browser viz, no mobile/desktop wrapper
- No interactive game loop — headless `GameSimulator` only (`01-game-engine.md`)
- Headless output is `GameResult` → `TrainingSample { [f64;27], u8, u64 score metadata }` → Parquet/CSV (see `06-Data/`)
