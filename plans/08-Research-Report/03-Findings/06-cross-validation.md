# Plan 06 — Cross-Validation: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for cross-validation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> Game-level train/validation/test partitions are chronological. Within the training partition, GroupKFold by `game_id` keeps game states together; those folds are group-disjoint but not temporally ordered. Do not describe GroupKFold as chronological.

## 1. Why CV and Leakage Model

Train/test split: single random split risks leakage if one game's board states land in both train and test. Solution: **group by `game_id`** so all positions from one game stay together.

## 2. Canonical Strategy

```rust
use automl::training::{CrossValidator, CVStrategy};
let cv = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 }).with_random_state(42);
// split(n_samples, Some(&y), Some(&groups)) where groups = polars column game_id as Array1<i64>
```

- `KFold{shuffle:true, n_splits:5}` only if no `game_id`; but we **have** `game_id` → GroupKFold is canonical.
- `StratifiedKFold` on `action` 0–3 if class-imbalance check required.
- **Blocked CV** = same as GroupKFold with `game_id`.

**TimeSeriesSplit is not used** — 2048 rollout data has no temporal dependence across games; forward chaining would artificially reduce sample size and is removed.

## 3. Nested CV Cost (Corrected)

Outer 5 × Inner 3 → **15 fits total**, not 105. Each fit is `TrainEngine::fit` + `predict`. Cost in `07-computational-budget.md`: ~50 CPUh for full 5-fold. Do not multiply by 7 models in same bound — per-model.

## 4. Rust Stub (Cross-Ref, Not Duplicate)

```rust
pub struct CVResults { pub scores: Vec<f64>, pub mean_score: f64, pub std_score: f64 }
pub fn run_groupkfold(x: &Array2<f64>, y: &Array1<f64>, groups: &Array1<i64>, seed: u64) -> CVResults {
    let cv = CrossValidator::new(CVStrategy::GroupKFold{ n_splits:5 }).with_random_state(seed);
    let splits = cv.split(x.nrows(), Some(y), Some(groups)).unwrap();
    // TrainEngine::fit_predict_arrays per split → accuracy or R²
    unimplemented!("project wrapper: split with groups, then score each fold")
}
```

The splitter comes from `automl/src/training/cross_validation.rs`; the grouped scoring wrapper belongs to the 2048 project because automl's convenience `cross_val_score` does not accept or forward groups.

## 5. Decision

Use 5-fold GroupKFold for model selection; final ranking still uses held-out 10k benchmark games (not CV score) to avoid optimistic bias.

## Implementation Record

- Project grouped CV is implemented and checks game separation, but reports accuracy only. This file still contains an obsolete `unimplemented!` code example and incorrectly calls GroupKFold chronological in its earlier framing; the actual contract is group-disjoint, not ordered. Needs standards-pass cleanup.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
