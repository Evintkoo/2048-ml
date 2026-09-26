# Plan 02 — Normalization: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Canonical deterministic scaling is implemented; fitted preprocessing is not integrated into root training.

**Goal:** State the current implementation and evidence boundary for normalization.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**The canonical input has deterministic scales and no upper bound of one.** Each board cell is divided by 32768 and score is encoded as `log10(score+1)/6` at index 16. Validation accepts finite, nonnegative values; fitted preprocessing remains a separate, unimplemented training integration.

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
let preprocessor = DataPreprocessor::new(config);
let df_norm = preprocessor.fit_transform(&df, &feature_cols)?;
```

Tree models: `ScalerType::None` is acceptable — skip scaling entirely, rely on deterministic divisors only.

## 4. Model-Specific Guidance (Future Integration)

| Model | Scaler |
|-------|--------|
| Linear/Logistic, SVM, KNN, MLP | `Standard` (or `MinMax`) |
| RandomForest, GradientBoosting, XGBoost, LightGBM, CatBoost | `None` (optional) |

## 5. Persistence (Not Implemented)

```rust
pub fn save_preprocessor(p: &DataPreprocessor, path: &str) -> Result<()> {
    std::fs::write(path, serde_json::to_string(&p.get_config())?)?; Ok(())
}
pub fn load_preprocessor(path: &str) -> Result<DataPreprocessor> {
    let cfg: PreprocessingConfig = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    Ok(DataPreprocessor::new(cfg))
}
```

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

- **Preprocessing integration:** the current trainer consumes stored values directly. If fitted scaling is added, define fold-local fitting and persistence before applying it to evaluation data.
- Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
