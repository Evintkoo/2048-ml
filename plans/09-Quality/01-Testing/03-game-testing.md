# Game Testing — Rule Validation Matrix (45 Lines, Automated cargo test)

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
| 8 | max tile `32768` | tile 32768 present | stays, next merge impossible (needs 17th cell) | — | 42 |
| 9 | score overflow | near `u32::MAX` accumulation | `checked_add` returns `Err` or saturates | — | — |
| 10 | spawn determinism | `ChaCha8Rng(42)` | same spawn sequence re-run | — | 42 |
| 11 | spawn 90/10 | 1000 spawns seed 42 | ~900×2, ~100×4 (±3σ) | — | 42 |
| 12 | action mapping | state → model predict → 0..3 | `action in 0..3` | — | 42 |
| 13 | valid-move detection | property: brute `would_change` vs `execute_move` changes | must match for all 4 dirs | — | random |
| 14 | corner slide | `[0,0,0,2]` → Right | `[0,0,0,2]` no shift? actually `[0,0,0,2]` stays | 0 | 42 |
| 15 | multi-row merges | two rows each mergable | both rows merge independently | sum deltas | 42 |

Plus: **random property** — 500 random boards × 4 dirs: `would_change` matches `execute_move` board-diff.

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
