# Plan 02 — 2048 Game Rules: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Redirect ticket audited; mechanics are implemented in the single `game_engine` module and covered by root tests.

**Goal:** State the current implementation and evidence boundary for 2048 game rules.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Redirect ticket audited; mechanics are implemented in the single `game_engine` module and covered by root tests.

> **Not canonical.** This file is a thin index only (~40 lines). Do not duplicate canonical rules here.

## Canonical Sources (Read These)

> **See canonical: `02-Rules/01-scoring-rules.md`, `02-Rules/02-win-lose-conditions.md`, `02-Rules/03-valid-moves.md`** — do not duplicate `GameState` enum, win/lose thresholds, or valid-move logic here.

| Rule | Canonical file |
|------|---------------|
| Scoring — score = sum merges, NEVER label, `TaskType::MultiClassification` `action:u8` | `02-Environment/02-Rules/01-scoring-rules.md` |
| Win/lose — 2048 not terminal, terminal = `would_change==false` for all dirs, Tier thresholds | `02-Environment/02-Rules/02-win-lose-conditions.md` |
| Valid moves — `board.would_change(dir)` + legal-action masking | `02-Environment/02-Rules/03-valid-moves.md` |
| Canonical training state — 16 board cells plus current score (17 values) | Plan 00 and `03-State/04-Encoding/01-state-vector.md` |
| Engine/RNG/Spawn | `01-Game/01-game-engine.md` + `03-Simulation-Engine/02-randomness.md` |

## Purpose

Thin redirect only. Full rules live in `02-Rules/*` — this file prevents duplication and keeps the index coherent.

## Minimal Summary (For Orientation Only)

- **Initial spawn:** 2 tiles, each `2` (90%) / `4` (10%) in random empty cells via `ChaCha8Rng` (`spawn_prob_4: f64 = 0.1`). **Canonical spawn logic:** `01-game-engine.md` §4.3.
- **Move:** slide all tiles in direction → merge equal neighbours once per tile per move (compressed → merged → compressed). One merge per tile per move.
- **Spawn after move:** one new `2`/`4` 90/10 via seeded RNG; if board unchanged, no spawn and move is invalid.
- **Score:** sum of merged-tile values, stored as `u64` metadata only — see `01-scoring-rules.md` (action `u8` 0–3 is the only label).
- **Terminal:** no direction changes the board — see `02-win-lose-conditions.md`. Baseline score comparisons use retained results under the declared evaluation protocol.

> No `GameState` enum defined here. No abort variant. No variant table (all variants out-of-scope; 4×4 fixed).

## Scope & Out-of-Scope

- **Headless simulation only** — no UI/web (see `01-Game/04-game-ui.md` deprecated stub).
- **Board canonical:** `[u32;16]` grid; the training input is those 16 cells plus current score (17 values), per Plan 00 and implemented by ticket #034. The former 27-derived-feature vector is excluded from canonical training.
- **RNG canonical:** `ChaCha8Rng` seed hygiene — see `03-Simulation-Engine/02-randomness.md`.

## Cross-References

- `01-Game/01-game-engine.md` (SimulatorConfig `seed:42, spawn_prob_4:0.1`)
- `01-Game/03-board-representation.md` (transforms via `slide_left`)
- `03-Simulation-Engine/01-simulation-engine.md` (canonical TrainingSample `[f64;17] → u8`)

---

## Verification (definition of done)

1. `test -f plans/02-Environment/01-Game/02-game-rules.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/02-Environment/01-Game/02-game-rules.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/01-Game/02-game-rules.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/01-Game/02-game-rules.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/01-Game/02-game-rules.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/01-Game/02-game-rules.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/01-Game/02-game-rules.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/01-Game/02-game-rules.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Redirect ticket audited; mechanics are implemented in the single `game_engine` module and covered by root tests. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
