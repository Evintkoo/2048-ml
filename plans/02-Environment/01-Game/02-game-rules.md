# 2048 Game Rules — Index / Redirect (Thin)

> **Not canonical.** This file is a thin index only (~40 lines). Do not duplicate canonical rules here.

## Canonical Sources (Read These)

> **See canonical: `02-Rules/01-scoring-rules.md`, `02-Rules/02-win-lose-conditions.md`, `02-Rules/03-valid-moves.md`** — do not duplicate `GameState` enum, win/lose thresholds, or valid-move logic here.

| Rule | Canonical file |
|------|---------------|
| Scoring — score = sum merges, NEVER label, `TaskType::MultiClassification` `action:u8` | `02-Environment/02-Rules/01-scoring-rules.md` |
| Win/lose — 2048 not terminal, terminal = `would_change==false` for all dirs, Tier thresholds | `02-Environment/02-Rules/02-win-lose-conditions.md` |
| Valid moves — `board.would_change(dir)` + constrained renormalization | `02-Environment/02-Rules/03-valid-moves.md` |
| Board/State/Features — `[u32;16]`, `/32768`, `[f64;27]` index 21 `/6.0` | `03-State/01-Board/01-board-state.md` |
| Engine/RNG/Spawn | `01-Game/01-game-engine.md` + `03-Simulation-Engine/02-randomness.md` |

## Purpose

Thin redirect only. Full rules live in `02-Rules/*` — this file prevents duplication and keeps the index coherent.

## Minimal Summary (For Orientation Only)

- **Initial spawn:** 2 tiles, each `2` (90%) / `4` (10%) in random empty cells via `ChaCha8Rng` (`spawn_prob_4: f64 = 0.1`). **Canonical spawn logic:** `01-game-engine.md` §4.3.
- **Move:** slide all tiles in direction → merge equal neighbours once per tile per move (compressed → merged → compressed). One merge per tile per move.
- **Spawn after move:** one new `2`/`4` 90/10 via seeded RNG; if board unchanged, no spawn and move is invalid.
- **Score:** sum of merged-tile values, stored as `u64` metadata only — see `01-scoring-rules.md` (action `u8` 0–3 is the only label).
- **Terminal:** board full AND no `would_change` — see `02-win-lose-conditions.md`. Score evaluation vs heuristic ~512.

> No `GameState` enum defined here. No abort variant. No variant table (all variants out-of-scope; 4×4 fixed).

## Scope & Out-of-Scope

- **Headless simulation only** — no UI/web (see `01-Game/04-game-ui.md` deprecated stub).
- **Board canonical:** `[u32;16]` grid, `/32768` normalization — see `03-State/01-Board/01-board-state.md`.
- **RNG canonical:** `ChaCha8Rng` seed hygiene — see `03-Simulation-Engine/02-randomness.md`.

## Cross-References

- `01-Game/01-game-engine.md` (SimulatorConfig `seed:42, spawn_prob_4:0.1`)
- `01-Game/03-board-representation.md` (transforms via `slide_left`)
- `03-Simulation-Engine/01-simulation-engine.md` (TrainingSample `[f64;27] → u8`)
