# Plan 02 — Integration Testing: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for integration testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

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

## Implementation Record

- CLI workflows and root tests exercise CSV collection/validation/splitting, grouped CV, training, inference, and benchmark paths. The named 100-row Parquet fixture, automated integration matrix/report artifact, and 100-game trained-model round trip have not been run as a single integration suite.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
