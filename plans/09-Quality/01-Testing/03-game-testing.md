# Plan 03 — Game Testing: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Game-engine unit tests cover core rules and seeded randomness; edge-case matrix is incomplete and was not rerun in this pass.

**Goal:** State the current implementation and evidence boundary for game testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Existing game tests cover many mechanics but do not implement every row below.** In particular, score-overflow atomicity and formal maximum-tile behavior remain unverified; tests were not run during this pass.

> **Distinct from `02-Validation/02-game-validation.md`:** Testing = automated `cargo test` (this file). Validation = manual audit + property tests (that file).

## 1. Purpose
Automated unit coverage for 4×4 rules, spawns, and score tracking. The matrix is a coverage map, not a claim that every row has a test.

## 2. Matrix (15 Cases)

| # | Rule | Board Before → Action | Expected After | Score Δ | Seed |
|---|------|----------------------|----------------|---------|------|
| 1 | slide no-merge | `[2,0,4,0]` → Left | `[2,4,0,0]` | 0 | 42 |
| 2 | single merge | `[2,2,0,0]` → Left | `[4,0,0,0]` | +4 | 42 |
| 3 | double-merge | `[2,2,4,4]` → Left | `[4,8,0,0]` | +12 (4+8) | 42 |
| 4 | single-merge cap | `[2,2,2,0]` → Left | `[4,2,0,0]` | +4 (one merge only) | 42 |
| 5 | blocked / no-op | `[[2,4],[8,16]...]` full no adj equal → any dir | `would_change==false`, no spawn | 0 | 42 |
| 6 | full board no moves | filled checkerboard | `is_game_over()==true` | — | — |
| 7 | full board with merge | full but `[...] [2,2] ...]` | `is_game_over()==false` | — | — |
| 8 | max tile `32768` | tile 32768 present | remains valid; a merge to 65536 requires room for an equal tile | — | 42 |
| 9 | score overflow | near `u64::MAX` accumulation | checked score addition returns `Err` without partially updating the board | — | — |
| 10 | spawn determinism | `ChaCha8Rng(42)` | same spawn sequence re-run | — | 42 |
| 11 | spawn 90/10 | 1000 spawns seed 42 | ~900×2, ~100×4 (±3σ) | — | 42 |
| 12 | action mapping | state → model predict → 0..3 | `action in 0..3` | — | 42 |
| 13 | valid-move detection | property: brute `would_change` vs `execute_move` changes | must match for all 4 dirs | — | random |
| 14 | corner slide | `[0,0,0,2]` → Left | `[2,0,0,0]` | 0 | 42 |
| 15 | multi-row merges | two rows each mergable | both rows merge independently | sum deltas | 42 |

Existing tests in `src/game_engine/mod.rs` cover merge rules, all directions, no-op/terminal cases, seeded spawn behavior, spawn-frequency sampling, invalid actions/tiles, corner coordinates, and randomized `would_change` consistency. The sampling replicate count and exact assertions should be checked in source when rerunning; score-overflow coverage is absent.

## 3. Assertions

Use existing `RawBoardState`, `GameSimulator`, and `ScoreTracker` APIs; the earlier pseudocode used APIs not present in the current source.

## 4. Metrics

No aggregate `GameTestMetrics` reporter is implemented; test assertions are unit-level.

## 5. Run

There is no separate `game` test target; game tests live within the library module. This pass did not execute tests. Cross-ref `09-Quality/02-Validation/02-game-validation.md` for manual audit checklist; do not duplicate that content here.

## Implementation Record

- Game-engine tests cover most core rules and randomness behavior. The separate test target and score-overflow case described in earlier drafts do not exist. This pass did not run tests.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/01-Testing/03-game-testing.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/01-Testing/03-game-testing.md` exits 0.

## Open questions

- **Current game-test status is unverified in this pass.** Re-run the library suite before release and add explicit overflow/boundary coverage only where the accepted input contract is defined.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
