# Plan 05 — Sensitivity Analysis: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for sensitivity analysis.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Hardware GPU table deleted.** Tree models (`RandomForest, GradientBoosting, XGBoost, LightGBM, ExtraTrees, SVM, KNN`) are CPU-only in `automl` (`smartcore`/`linfa`). No GPU path to test; sensitivity is seed + data.

## 1. Seed Sensitivity (Canonical)

All 7 `ModelType` × seeds `42,123,456,789,1011` → 35 ×10k = 350k games (cap via same 270k rule as ablation: prioritize winner ModelType first). Metric:

```
seed_sensitivity = σ(μ_seed) / mean(μ_seed) ×100%
```

- <10% + stable ranking → robust.
- >20% or flip → inconclusive winner, escalate to 50k games.
- Report per-model `mean, σ, CI` and overall ranking stability.

## 2. Data Sensitivity

| Factor | Levels | Gate |
|--------|--------|------|
| Train size | 1k,5k,10k,50k,100k | Score vs size; plateau = good |
| Label noise | 0%,5%,10%,20% flipped `a*` | Δmean with MWU |
| Sample bias | early-half vs late-half vs balanced | Δmean |

## 3. Hyperparameter Sensitivity

Grid per winner ModelType: `n_estimators {50,100,200}`, `max_depth {3,6,12}`, `lr {0.05,0.1,0.3}` — report `Sensitivity=(Max-Min)/Max`.

## 4. Evaluation Sensitivity (Already in 03-results.md)

CI width vs n: 10k → ~20 (σ512). No duplication of benchmark CI table.

## 5. Consolidated with Ablation

If ablation shows group redundancy, data-size plateau implies 27-dim over-parameterized — cross-ref `04-ablation-study.md` §1.2.

## Implementation Record

- No multi-seed, data-size, label-noise, feature, or hyperparameter sensitivity sweep has been run. The prescribed game counts and effect thresholds are not results.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
2. `grep -q '^# Plan 05 — ' plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/03-Findings/05-sensitivity-analysis.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
