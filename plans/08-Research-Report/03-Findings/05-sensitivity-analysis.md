# Plan 05 — Sensitivity Analysis: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-30).** Sensitivity analyses are proposed; UCI split diagnostics and a fixed-corpus 25-fit classifier seed diagnostic exist, but policy-score, data-size, and independent-corpus sensitivity remain unmeasured.

**Goal:** State the current implementation and evidence boundary for sensitivity analysis.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**No confirmatory sensitivity result is claimed.** Five candidate classifiers were each refit five times with different training seeds on one fixed 25-game dataset and holdout. This measures classifier-label sensitivity to fit seeds on that fixed corpus; it does not measure independent-corpus variation, policy-score variation, or data-size sensitivity. Proposed larger matrices are not based on an approved resource budget or power rationale.

> Hardware acceleration sensitivity is outside the current protocol. Model/seed/data sensitivity requires the actual supported candidates and a declared budget.

## 1. Seed Sensitivity (Canonical)

Select supported model candidates and independent training/evaluation seed roles before the study. The previously proposed 7-model, five-seed, 350,000-game matrix and 270,000 cap are not approved. A descriptive coefficient of variation may be reported where defined, with uncertainty and model-training replication considered:

If reported, define the variation summary and its handling of near-zero means in the study protocol; no single formula or threshold is canonical.

Do not apply universal 10%/20% cutoffs or increase game count automatically. Report measured variation and explain its limits.

## 2. Data Sensitivity

| Factor | Levels | Gate |
|--------|--------|------|
| Train size | Select after pilot and compute budget | Measure score and uncertainty; a plateau requires a defined criterion |
| Label perturbation | Only if justified and implementable | Compare under a predeclared perturbation protocol |
| Sampling variation | Declare data split/design | Measure sensitivity without assuming temporal ordering |

## 3. Hyperparameter Sensitivity

Hyperparameter sensitivity depends on the selected model adapters and supported search dimensions. The current tuning config supports only RandomForest and ExtraTrees parameters; no broad grid study has run.

## 4. Evaluation Sensitivity (Already in 03-results.md)

Precision versus sample size is unknown until score variance and dependence are measured.

## 5. Consolidated with Ablation

Ablation and data-size results answer different questions. Neither alone establishes feature redundancy or sufficiency.

## Implementation Record

- No policy-score, data-size, label-perturbation, feature, or independent-corpus sensitivity sweep has been run. The retained five-by-five repeated classifier fit diagnostic uses a fixed dataset/holdout and reports raw-label sensitivity only; UCI split-seed diagnostics are a separate framework study. Proposed thresholds and sample counts are not evidence.

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

- **Sensitivity work remains pending.** Select conditions, supported parameters, experimental units, and resource budget before running sweeps.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
