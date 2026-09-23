# Plan 03 — Game Testing: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for game testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Distinct from `02-Validation/02-game-validation.md`:** Testing = automated `cargo test` (this file). Validation = manual audit + property tests (that file).

## 1. Purpose
Automated `cargo test` validation of 4×4 rules, spawns, and score tracking on fixed seeds. 15 cases, no manual steps.

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

Implemented under `src/game_engine/mod.rs`: slide/no-double-merge cases; all directions; no-op counter/score; terminal and full-with-merge boards; seeded spawn reproducibility; 10,000-spawn 90/10 frequency check; invalid actions/tiles; corner slide; and 500 seeded random boards × 4 directions checking `would_change` against `execute_move`.

## 3. Assertions

```rust
#[test] fn double_merge() {
  let (board, delta) = engine.execute_move(2); // Left
  assert_eq!(board.row(0), [4,8,0,0]);
  assert_eq!(delta, 12);
  assert!(engine.score_tracker().is_monotonic());
}
```

## 4. Metrics

```rust
pub struct GameTestMetrics { pub n_games: usize, pub rules_passed: usize, pub score_accuracy: f64 }
```

## 5. Run

```bash
cargo test --test game -- --nocapture   # 20+ tests (15 matrix + 5 property) ~5 min
```

Cross-ref `09-Quality/02-Validation/02-game-validation.md` for manual audit checklist; do not duplicate that content here.

## Implementation Record

- Root game tests implement most listed rule cases plus randomized directional validity and 10,000 spawn probability checks. Score overflow/no-partial-update has not been exercised by the current test matrix; the configured integration test target is not separately defined.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
