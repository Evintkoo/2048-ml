# Plan 02 — Normalization: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** Canonical deterministic scaling and validation are implemented; fitted model-specific preprocessing is optional outside the current training protocol.

**Goal:** State the current implementation and evidence boundary for normalization.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**The canonical normalization contract is implemented with deterministic scales and no upper bound of one.** Each board cell is divided by 32768 and score is encoded as `log10(score+1)/6` at index 16. Validation accepts finite, nonnegative values. Fitted preprocessing is an optional, model-specific extension and is not part of the current canonical training protocol.

> **Cross-ref:** canonical creation is `BoardStateMl::from_board` in `src/state.rs`; see `01-state-vector.md` for vector order. This file records deterministic scaling and preprocessing status.
> **No `move_count_norm`.** Deleted — not in 17, not predictive. Do not reintroduce.

## 1. Deterministic Divisors (Canonical — No Fitting)

The canonical model has only the 16 grid cells and current score. These fixed
scales are deterministic; values above one are valid for larger tiles or scores:

| Feature | Formula |
|---------|---------|
| grid `0..15` | `tile as f64 / 32768.0` |
| score index 16 | `(score as f64 + 1.0).log10() / 6.0` |

Correct signatures: `fn(value: u32) -> f64` (not `u8`). Max tile 32768 = 2^15.

## 2. Fitted Scaler vs Deterministic

| Layer | When | `ScalerType` |
|-------|------|--------------|
| Deterministic divisors | always (creation) | — (fixed math) |
| `DataPreprocessor` fitted | optional, after divisors | `Standard` for linear/SVM/KNN; `None` for trees (scale-invariant) |

Deterministic divisors are **not** a fitted `ScalerType`. The current root trainer loads stored feature values directly and does not fit or persist an AutoML `DataPreprocessor`; model-specific scaler suggestions below are not implemented training behavior.

## 3. automl Pipeline — Correct APIs

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType, ImputeStrategy, EncoderType};
let config = PreprocessingConfig::default()
    .with_scaler(ScalerType::Standard)      // or ScalerType::None for RandomForest/XGBoost/LightGBM
    .with_numeric_impute(ImputeStrategy::Mean)
    .with_encoder(EncoderType::OneHot);     // categoricals only; no action one-hot needed (automl handles label internally)
let mut preprocessor = DataPreprocessor::with_config(config);
// Fit only on the training fold's feature-only DataFrame; use the fitted
// preprocessor to transform validation/test features without refitting.
let df_norm = preprocessor.fit_transform(&train_features)?;
```

This illustrates the framework API; the root training pipeline does not call it. Tree models use the deterministic state values directly, and any future fitted transform must be fit fold-locally and persisted with its fitted state. `DataPreprocessor::new()` takes no arguments; configure via `with_config`.

## 4. Model-Specific Guidance (Future Integration)

| Model | Scaler |
|-------|--------|
| Linear/Logistic, SVM, KNN, MLP | `Standard` (or `MinMax`) |
| RandomForest, GradientBoosting, XGBoost, LightGBM, CatBoost | `None` (optional) |

## 5. Configuration Serialization (Not Implemented in Root)

```rust
let json = serde_json::to_string(&config)?; // PreprocessingConfig supports Serde
let loaded: PreprocessingConfig = serde_json::from_str(&json)?;
let preprocessor = DataPreprocessor::with_config(loaded);
```

This round-trip stores configuration only. The root does not currently save
fitted transform state; config serialization must not be described as a
reusable fitted preprocessor artifact.

## 6. Validation

The state validator and CSV reader reject non-finite and negative values but accept values above one. The trainer does not apply a fitted scaler:

```rust
fn validate_normalization(v: &[f64;17]) -> Result<()> {
    for (i, &x) in v.iter().enumerate() {
        if !x.is_finite() { return Err(format!("{} not finite", i).into()); }
        if x < 0.0 { return Err(format!("{} negative", i).into()); }
    } Ok(())
}
```

## Implementation Record

- The encoder and CSV validators accept any finite, nonnegative value, including normalized tiles and scores above one.
- The current trainer does not fit or persist a `DataPreprocessor`; it consumes stored values directly. The examples below describe possible future integration, not current behavior.

---

## Verification (definition of done)

1. `test -f plans/03-State/04-Encoding/02-normalization.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/03-State/04-Encoding/02-normalization.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/04-Encoding/02-normalization.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/04-Encoding/02-normalization.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/04-Encoding/02-normalization.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/04-Encoding/02-normalization.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/04-Encoding/02-normalization.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/04-Encoding/02-normalization.md` exits 0.

## Open questions

- No required normalization deliverable remains open. Any future fitted transform requires a separately declared model protocol, fold-local fitting, and persistence of fitted state.

## Later

- **No further normalization change is required for the current canonical input.** Keep the fixed scales stable unless a separately documented model study justifies an additional transform.
