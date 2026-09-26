# Plan 06 — Cross-Validation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Game-disjoint chronological holdout and grouped CV helpers are implemented; the full research split protocol has not been audited on a plan-scale corpus.

**Goal:** State the current implementation and evidence boundary for cross-validation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Implementation exists, but this is not evidence of evaluation on a canonical corpus.** Root training can reserve later game IDs and apply grouped CV to earlier games; split semantics and metrics must be reported accurately.

> `split_csv_by_game` assigns whole game IDs to chronological 70/15/15 train/validation/test partitions. Root training can reserve final game groups and perform GroupKFold on earlier groups. GroupKFold is group-disjoint, not chronological.

## 1. Why CV and Leakage Model

Rows within one game are dependent. Keep every row for a game in exactly one outer partition; metadata IDs must not enter the model feature matrix.

## 2. Canonical Strategy

```rust
use automl::training::{CrossValidator, CVStrategy};
let validator = CrossValidator::new(CVStrategy::GroupKFold { n_splits })
    .with_random_state(seed);
// src/training.rs checks groups and scores each held-out fold
```

- The current helper applies GroupKFold by game and checks train/validation group separation.
- Fold metric is action accuracy. It does not currently emit macro-F1 or game-score performance.
- Chronological holdout and random group folds answer different questions; record which was used.

**TimeSeriesSplit is not used** — 2048 rollout data has no temporal dependence across games; forward chaining would artificially reduce sample size and is removed.

## 3. Nested CV Cost (Corrected)

Nested CV is not implemented as a study protocol. Its fit count and compute cost depend on candidate/configuration counts and must be calculated from the actual design; previous 50 CPU-hour estimate is unsupported.

## 4. Rust Stub (Cross-Ref, Not Duplicate)

The implementation is in `src/training.rs` (`grouped_cross_validate` and `grouped_cross_validate_configured`). It uses AutoML's grouped splitter, verifies group separation, trains per fold, and reports fold action accuracy. Do not use the former `unimplemented!` example as runnable code.

## 5. Decision

The current code supports configurable grouped folds and reserves chronological game partitions in the training path. No final held-out trained-policy ranking has been completed; do not assume a 10k evaluation protocol is already in force.

## Implementation Record

- `src/data_pipeline.rs` implements whole-game chronological 70/15/15 splitting; `src/training.rs` implements GroupKFold and held-out fold action accuracy. No plan-scale split audit has run. The former stub example and inaccurate chronology wording were removed.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
2. `grep -q '^# Plan 06 — ' plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/03-Findings/06-cross-validation.md` exits 0.

## Open questions

- **Validation evidence remains bounded by implementation and small checks.** Preserve game-group provenance and audit actual split manifests before using CV results in a research claim.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
