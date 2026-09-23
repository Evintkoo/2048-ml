# Plan 02 — Normalization: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for normalization.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Cross-ref:** creation canonical in `01-state-vector.md:31` `create_state_vector`; see `01-state-vector.md §4` for preprocessor. This file = normalization rules.
> **No `move_count_norm`.** Deleted — not in 27, not predictive. Do not reintroduce.

## 1. Deterministic Divisors (Canonical — No Fitting)

All 27 features are already [0,1] via fixed divisors:

| Feature | Signature | Formula |
|---------|-----------|---------|
| grid `0..15` | `fn(v: u32) -> f64` | `v as f64 / 32768.0` |
| empty_count |  | `empty as f64 / 16.0` |
| max_tile_log |  | `0 if max==0 else log2(max) / 15.0` |
| monotonicity/smoothness |  | already [0,1] |
| merges_available |  | `merges as f64 / 16.0` |
| score_normalized idx21 |  | `(score as f64 + 1.0).log10() / 6.0` |
| adjacency_merge_score |  | `sum / 16.0` |
| corner_max |  | `corner as f64 / 32768.0` |
| edge_tiles_occupied |  | `occupied as f64 / 12.0` |
| col_worst / row_worst |  | `min_sum as f64 / 8192.0` |

Correct signatures: `fn(value: u32) -> f64` (not `u8`). Max tile 32768 = 2^15.

## 2. Fitted Scaler vs Deterministic

| Layer | When | `ScalerType` |
|-------|------|--------------|
| Deterministic divisors | always (creation) | — (fixed math) |
| `DataPreprocessor` fitted | optional, after divisors | `Standard` for linear/SVM/KNN; `None` for trees (scale-invariant) |

Deterministic divisors are **not** a `ScalerType`; fitted `StandardScaler` is fitted on training `DataFrame` after divisors.

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

## 4. Model-Specific Guidance

| Model | Scaler |
|-------|--------|
| Linear/Logistic, SVM, KNN, MLP | `Standard` (or `MinMax`) |
| RandomForest, GradientBoosting, XGBoost, LightGBM, CatBoost | `None` (optional) |

## 5. Persistence

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

Deterministic divisors guarantee [0,1] before scaler. After any scaler, check finite:

```rust
fn validate_normalization(v: &[f64;27]) -> Result<()> {
    for (i, &x) in v.iter().enumerate() {
        if !x.is_finite() { return Err(format!("{} not finite", i).into()); }
        if !(0.0..=1.0).contains(&x) && scaler_is_none { /* deterministic must be [0,1] */ }
    } Ok(())
}
```

## Implementation Record

- Deterministic scaling is implemented before storage. Feature and CSV validators apply the declared per-feature range rule, including the uncapped score feature.
- The current trainer does not fit or persist a `DataPreprocessor`; preprocessing integration remains pending. Tree-based training currently consumes the deterministic feature values directly.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
