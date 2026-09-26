# Plan 04 — Game UI: the repository status is explicit and evidence based

> **Status: NOT APPLICABLE (2026-09-26).** This is a deprecated UI stub; the headless MVP has no renderer, and static inspection remains optional under ticket 022.

**Goal:** State the current implementation and evidence boundary for game ui.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is explicitly deprecated and out of scope for the headless MVP.** No renderer is implemented or required. The proposed terminal helper is retained here as historical design text; the optional static inspection work is tracked by ticket 022.

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

---

## Verification (definition of done)

1. `test -f plans/02-Environment/01-Game/04-game-ui.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/02-Environment/01-Game/04-game-ui.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/01-Game/04-game-ui.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/01-Game/04-game-ui.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/01-Game/04-game-ui.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/01-Game/04-game-ui.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/01-Game/04-game-ui.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/01-Game/04-game-ui.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Headless-only MVP excludes the debug UI; optional static inspection work is tracked by ticket 022. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
