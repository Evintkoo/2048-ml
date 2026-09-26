# Plan 01 — State Vector: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** The canonical 17-value encoder is implemented and accepts finite nonnegative values above one.

**Goal:** State the current implementation and evidence boundary for state vector.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**The canonical vector is implemented and validated.** It contains the 16 row-major board cells divided by 32768 followed by `log10(score+1)/6` at index 16. Move count and game history are excluded. Validators require finite nonnegative values but allow values above one.

> **Canonical model input: 17 values.** All other state files cross-ref this. Storage format lives in `01-Board/03-state-encoding.md`.

## 1. Overview

Numerical representation of 2048 board+score for `TaskType::MultiClassification` `action 0..3`.

## 2. Dimensions — 17

| Idx | Feature | Formula | Range |
|-----|---------|---------|-------|
| 0..15 | grid_0..15 | `v as f64 / 32768.0` | finite, nonnegative; may exceed 1 |
| 16 | score_normalized | `log10(score+1)/6` | finite, nonnegative; may exceed 1 |

17 values. No move count or history features.

## 3. Canonical Creation — `BoardStateMl::from_board`

The implementation assembles a fixed `[f64;17]`: 16 row-major tiles divided by
32768 and normalized score at index 16. `BoardStateMl::validate` checks that
every value is finite and nonnegative; values above one are valid.

## 4. Preprocessing — automl `DataPreprocessor`

No fitted preprocessing is applied to the canonical deterministic features. The actual training pipeline loads their CSV values directly; see the training configuration ticket for the preprocessing boundary.

See `02-normalization.md` for deterministic vs fitted mapping.

## 5. Serialization — Storage (cross-ref)

Canonical storage header 17+action is in `01-Board/03-state-encoding.md`. Minimal:

```rust
fn state_vector_to_csv(vec: &[f64;17], action: u8) -> String {
    let mut p: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
    p.push(action.to_string()); p.join(",")
}
```

## 6. Validation — Finite, Nonnegative Values

```rust
pub fn validate_state_vector(vec: &[f64;17]) -> Result<()> {
    for (i, &v) in vec.iter().enumerate() {
        if !v.is_finite() { return Err(format!("{} not finite: {}", i, v).into()); }
        if v < 0.0 { return Err(format!("{} is negative: {}", i, v).into()); }
        // Values above one are allowed for large tile/score encodings.
    }
    Ok(())
}
```

## 7. Task — MultiClassification Only

```rust
let state = create_state_vector(&board);
let action: u8 = 2; // label 0..3
// TrainingConfig::new(TaskType::MultiClassification, "action")
```

## Implementation Record

- `BoardStateMl::from_board` implements the 17-value canonical vector. Tests verify row-major cells, score at index 16, and legal large tiles/scores that encode above one.
- CSV serialization and action validation are implemented; Parquet creation and fitted preprocessing/export are not in the live pipeline.

---

## Verification (definition of done)

1. `test -f plans/03-State/04-Encoding/01-state-vector.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/03-State/04-Encoding/01-state-vector.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/04-Encoding/01-state-vector.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/04-Encoding/01-state-vector.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/04-Encoding/01-state-vector.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/04-Encoding/01-state-vector.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/04-Encoding/01-state-vector.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/04-Encoding/01-state-vector.md` exits 0.

## Open questions

- **No encoding defect remains in this ticket.** Predictive feature usefulness requires a separately specified evaluation and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
