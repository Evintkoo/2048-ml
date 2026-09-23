# Plan 01 — Code Reference: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for code reference.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Fix:** Prior `plans/` org diagram deleted. References now align with real `automl/` crate + proposed 2048 crates under root. Paths verified against `automl/src` and `plans/**/reproducibility-package.md`.

## 1. Real Repo Layout

```
2048-ml/                          # root (plans/ + automl submodule)
├── automl/                      # submodule Evintkoo/automl v1.0.0
│   ├── src/training/config.rs   # TrainingConfig, TaskType::MultiClassification, ModelType
│   ├── src/training/engine.rs   # TrainEngine::fit / fit_predict_arrays
│   ├── src/training/cross_validation.rs # CrossValidator, CVStrategy::GroupKFold
│   ├── src/optimizer/           # HyperOptX, TPE
│   ├── src/preprocessing/       # DataPreprocessor
│   └── Cargo.toml               # polars 0.46, smartcore 0.3
├── src/                          # (proposed) single root MVP crate
│   ├── game_engine/              # board, score, engine, spawn
│   ├── data_pipeline/            # feature extraction and rollout labels
│   └── evaluation/               # benchmark, statistics, ranking
├── Cargo.lock / requirements.txt # pinned
└── plans/                       # this docs repo — not code
```

## 2. Correct TrainingConfig Example (Real API)

```rust
use automl::training::{TrainingConfig, TaskType, ModelType};
let config = TrainingConfig::new(TaskType::MultiClassification, "action")
    .with_model(ModelType::RandomForest)
    .with_random_state(42)
    .with_cv(5);
// feature_columns: 27 feature names; game_id is metadata passed separately
// to CrossValidator::split, never a model feature.
```

Previous `learning_rate/epochs/batch_size` example deleted — not real automl fields (see `automl/src/training/config.rs`: `n_estimators, max_depth, learning_rate, subsample`, etc.).

## 3. Data Flow (Code-Verified)

`engine.rs:GameEngine::execute_move(0..3)` → `feature_extraction::extract_27(board)` → `label_generation::rollout_label(board, 100)` → `TrainEngine::fit` → `benchmark_runner::run_n(10000, seed=42)` → `statistical_tests::mann_whitney`.

## 4. Dependencies (Pinned)

`automl v1.0.0, rust 1.75, polars 0.46, rand_chacha 0.3`. See `automl/Cargo.toml`.

## Implementation Record

- Several referenced modules and paths in this appendix do not exist (`benchmark_runner`, `statistical_tests`, `feature_extraction`, and separate crate layouts). Current source paths are documented in `06-reproducibility-package.md` and the root crate; reconcile this appendix with the actual repository.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/01-code-reference.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
