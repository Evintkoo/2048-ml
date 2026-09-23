# Unit Testing — 2048-Specific (Concrete Cases, Tarpaulin)

## 1. Purpose
Unit tests for 4×4 game engine, feature extraction, and automl wiring. Distinct from integration (pipeline wiring) and game-validation (manual audit).

## 2. Concrete 2048 Cases (Must Implement)

| # | Module | Case | Assertion |
|---|--------|------|-----------|
| 1 | board.rs | double-merge `2+2+4 → 4+4` in one move | `assert_eq!(after_row, [4,4,0,0]) && score_delta==4` |
| 2 | board.rs | single-merge only per tile per move `2+2+2 → 4+2` | `assert_eq!(row, [4,2,0,0])` |
| 3 | board.rs | no-op detection (full board, no merge) | `would_change()!=execute_move() → no spawn, score 0` |
| 4 | engine.rs | spawn 90/10 deterministic with ChaCha8Rng(seed=42) | `spawn_value() in {2,4} && reproducible` |
| 5 | engine.rs | random spawn uniform empty cell | seeded empty choice deterministic |
| 6 | score.rs | overflow check `u32::checked_add` on merge sum | `assert!(score.checked_add...is_some())` or error |
| 7 | board.rs | blocked move (all moves would_change==false) | `is_game_over()==true` |
| 8 | feature_extraction.rs | 27-len vector, correct `game_id` propagation | `features.len()==27 && game_id==input_id` |
| 9 | label_generation.rs | rollout label 0..3 valid | `label in 0..3` |
| 10 | training/config.rs | `TaskType::MultiClassification` + `ModelType::RandomForest` builder | `TrainingConfig::new(MultiClassification,"action")` works |

Add `score_tracking`, `max_tile` boundary (`32768` → next merge would need 17th cell, no effect).

## 3. Coverage (Realistic, Not Fantasy)

- Target: **≥80% line** overall (`cargo tarpaulin --out Xml --timeout 120`), not fictional 80/90/85 mermaid. Report per-crate: `game_engine ≥85%, data_pipeline ≥80%, automl wrapper ≥70%`.
- Run: `cargo test --lib -- --test-threads=1` + `cargo tarpaulin --config tarpaulin.toml`
- `tarpaulin.toml`:
```toml
[report]
out = ["Xml", "Html"]
[run]
timeout = 120
```

## 4. Execution

```bash
cargo test --lib          # 100+ unit tests <30s
cargo test -- --nocapture # debug invalid-move logs
cargo tarpaulin           # coverage gate in CI
```
Pre-commit: `cargo fmt --check && cargo clippy -- -D warnings`.

## 5. Quality Gate
Merge only if all 10 concrete cases pass + coverage ≥80% + clippy clean.
