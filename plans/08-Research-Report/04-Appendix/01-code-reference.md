# Plan 01 — Code Reference: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Root crate and submodule paths have been source-audited; this reference is not a complete generated API inventory.

**Goal:** State the current implementation and evidence boundary for code reference.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is a source map, not an API guarantee.** Root modules below exist; framework APIs should be checked against the pinned submodule revision and its local worktree modifications.

## 1. Repository Layout (Source-Audited)

```text
2048-ml/
├── src/
│   ├── game_engine/mod.rs       # board, moves, spawn, simulator, rollout labeler
│   ├── state.rs                 # 17-value supervised state encoding
│   ├── actions.rs / policy.rs   # action types, masking, policy helpers
│   ├── data_pipeline.rs         # CSV schema, metadata, game splits
│   ├── training.rs              # grouped cross-validation helper
│   ├── collection.rs            # resumable parallel rollout collection
│   ├── hyperopt_config.rs       # schema-versioned search configuration
│   ├── evaluation.rs            # score summaries and statistical helpers
│   ├── framework_validation.rs  # API/capability smoke paths
│   ├── seeds.rs                 # seed derivation
│   └── main.rs                  # CLI commands and report output
├── automl/                      # pinned Git submodule; local worktree may be modified
├── Cargo.toml / Cargo.lock      # root Rust package and locked dependencies
└── plans/                       # plan tickets and research documentation
```


## 2. Correct TrainingConfig Example (Real API)

```rust
use automl::training::{TrainingConfig, TaskType, ModelType};
let config = TrainingConfig::new(TaskType::MultiClassification, "action")
    .with_model(ModelType::RandomForest)
    .with_random_state(42)
    .with_cv(5);
// Project helpers carry game IDs separately and validate their row alignment.
```

Previous `learning_rate/epochs/batch_size` example deleted — not real automl fields (see `automl/src/training/config.rs`: `n_estimators, max_depth, learning_rate, subsample`, etc.).

## 3. Data Flow (Code-Verified)

`src/game_engine/mod.rs` simulator/relabeler → `src/state.rs` feature encoding → `src/data_pipeline.rs` CSV and game split → `src/training.rs` AutoML/grouped CV → `src/main.rs` train, benchmark, and compare CLI → `src/evaluation.rs` summaries and test helpers.

## 4. Dependencies (Pinned)

The root manifest declares Rust 1.75 minimum, a local-path AutoML dependency, Polars 0.46, and rand_chacha 0.3. Record the actual compiler and exact submodule commit for each result; manifest constraints alone do not capture the runtime environment.

## Implementation Record

- Replaced nonexistent module paths and package layout with the current root modules. Framework internals remain a submodule; this appendix does not promise API stability or reproduce the full capability matrix.

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

- **Keep this source map aligned with code changes.** Record the declared AutoML pin and exact root revision when citing framework internals.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
