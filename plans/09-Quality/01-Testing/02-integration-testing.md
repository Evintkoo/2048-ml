# Integration Testing — Pipeline Matrix (40 Lines, No Trimmed Pointer)

## 1. Purpose
Verify **wiring** between modules — not unit logic. Distinct from unit (single function) and CI pipeline (stages). Uses real fixtures, not mocks, with `seed 42`.

## 2. Integration Levels
- **Level 1:** Game+Score+State+Action (engine internal)
- **Level 2:** Data Pipeline → TrainEngine (feature→label→fit)
- **Level 3:** TrainEngine → GameSimulator round-trip (train → predict → benchmark 100 games)
- **Level 4:** Config→All (TrainingConfig propagates to every crate)

## 3. Integration Matrix (Concrete)

| Test | Fixture | Wiring | Assert | Time |
|------|---------|--------|--------|------|
| Game+Score | `Board [[2,2,4,0],...]` seed 42 | `GameEngine::execute_move(2)` → `ScoreTracker` | `score_delta==4 && board==[[4,4,0,0]...]` | <10ms |
| Score+State+Action | full board history | `StateVector(27) → action 0..3 → board` | `action in 0..3 && would_change` checked | <10ms |
| Data→TrainEngine | 100-row parquet (27+game_id+action) | `DataPreprocessor → TrainEngine::fit(MultiClassification)` | `model.predict` returns 0..3 | ~5s |
| Train+Simulator | trained RF(10 trees) | `InferenceEngine::predict(state) → GameEngine::execute` | 100 games complete, `score` monotonic via tracker | ~30s |
| Config→All | `TrainingConfig{MultiClassification, RandomForest, seed42}` | passed to `DataCollector`, `TrainEngine`, `BenchmarkRunner` | same seed & task everywhere | <1s |
| Polars I/O | `ScoreMetrics` 10k rows | `benchmark_runner → parquet → ranking_analysis.py` | round-trip mean matches in-memory | ~2s |
| GroupKFold wiring | 100 samples + `game_id` | `CrossValidator::GroupKFold` | no group split across train/test | <1s |

## 4. Fixtures

```rust
fn fixture_board_double_merge() -> Board { /* 2,2,4 top row */ }
fn fixture_27_row(game_id: i64) -> (Vec<f64>, i64) { /* 27 floats + id */ }
```

## 5. Run

```bash
cargo test --test integration -- --test-threads=4   # 50+ tests, ~2 min
```

Fail → block merge (same as unit). Report: `IntegrationTestResult { test_name, components, passed, ms, error }` in `target/integration.json`.
