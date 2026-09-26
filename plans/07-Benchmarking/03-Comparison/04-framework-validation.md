# Plan 04 — Rust-Native AutoML Framework Validation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Three planned UCI datasets are acquired and a reproducible fixed-split runner is implemented; baseline, repeated-seed, resource, and CLI/API comparisons remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework validation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats API/capability validation as partial evidence, not full framework validation.** Local tests exercise model/task compatibility, grouped splitting, optimizer API, serialization, and root integration. The named tabular dataset matrix, matched external baselines, resource profiles, multi-seed matrix, and CLI/library equivalence remain unrun.

## 1. Purpose

Validate the independent Rust-native `Evintkoo/automl` architecture before interpreting any 2048 result. This is the primary framework-evaluation track and is separate from the 2048 application benchmark.

## 2. Validation Questions

1. Does the architecture provide coherent data, configuration, model, and artifact contracts?
2. Do the documented APIs and supported model types exist and behave correctly?
3. Does preprocessing avoid train/test leakage?
4. Are cross-validation, metrics, serialization, and prediction correct?
5. Are seeds, parallel execution, and repeated runs reproducible?
6. How do accuracy, runtime, memory use, failure rate, and search efficiency compare with established baselines?
7. What trade-offs does the Rust-native architecture make relative to established alternatives?

## 3. Validation Tasks

- Document the architecture, module boundaries, data contracts, and capability matrix.
- Run the framework on named standard tabular classification datasets with fixed splits.
- Test every model type used in the 2048 study.
- Test default training and hyperparameter optimization separately.
- Verify train/validation/test separation.
- Verify model save/load prediction equivalence.
- Repeat runs with identical and different seeds.
- Compare CLI and library/API outputs on identical configurations.
- Measure resource use, failure behavior, and search-budget efficiency.
- Record unsupported capabilities and implementation failures.

## 4. Benchmark Matrix

Use a small, reproducible set of standard tabular classification datasets before expanding the scope. The initial UCI archives and their hashes are recorded in [`data/framework_validation/README.md`](../../../data/framework_validation/README.md). The three listed datasets have been fetched unchanged; the optional larger Adult/OpenML/UCI dataset is not yet selected.

| Dataset | Purpose | Primary Metrics |
|---------|---------|-----------------|
| Iris | Small smoke test and API correctness | Accuracy, macro-F1, runtime |
| Wine | Multiclass preprocessing and model comparison | Accuracy, macro-F1, runtime |
| Breast Cancer Wisconsin | Binary classification and scaling behavior | ROC-AUC, accuracy, macro-F1 |
| Adult or another declared OpenML/UCI dataset | Larger-data resource behavior | Accuracy/ROC-AUC, memory, runtime |

Dataset versions, download hashes, preprocessing, splits, and licenses must be recorded. These datasets validate the framework; they are not evidence that the framework is optimal for every ML task.

### Initial execution protocol

The first diagnostic run uses seed `42`, a stratified 80/20 outer train/test
split, raw numeric predictors without scaling, and the five classifier types
already verified for the 2048 integration: RandomForest, ExtraTrees, AdaBoost,
KNN, and NaiveBayes. Each model uses 32 estimators, max depth 8, and AutoML's
internal validation fraction of 0.1 within the outer training partition. The
outer test rows are passed only to prediction. Dataset-specific split manifests
retain source row indices and the exact seed. This one-seed run is an API and
pipeline check, not a comparative finding or final framework validation.

Run it with:

```sh
cargo run -- framework-validate --seed 42 --test-fraction 0.2
```

## 5. Acceptance Criteria

- Required APIs execute successfully on the declared datasets.
- Train/test separation is demonstrated by an automated leakage test.
- Repeated identical runs produce identical predictions or a documented bounded nondeterminism.
- Save/load predictions match the original model within a predeclared tolerance.
- CLI and library/API results match for identical configurations.
- Every failure is captured with configuration, seed, dependency versions, and diagnostic output.

Passing these criteria establishes framework usability for the study; it does not by itself establish superiority over other AutoML systems.

## 6. Metrics

Report task-appropriate predictive metrics, wall-clock time, CPU and memory use, search budget, reproducibility, failure rate, model reload equivalence, and configuration/API consistency. Framework quality must not be inferred from 2048 score alone.

## 7. Comparison Rules

Framework comparisons use named datasets, fixed splits, explicit preprocessing assumptions, hardware descriptions, dependency versions, and matched search budgets. Baselines must be selected by capability rather than by convenient score. Established libraries are comparison baselines only; they are not used to train the core 2048 models.

## 8. Relationship to the 2048 Study

The framework validation gate must pass before the main 2048 training milestone. If a required capability fails, the failure becomes a documented framework result and the affected 2048 claim is not made. The final thesis must report both successful capabilities and negative framework findings.

## 9. Execution Status

The pinned framework's API, model probability shapes, group splitter, optimizer API, model serialization, and 2048 integration smoke paths have been checked in `src/framework_validation.rs`; AutoML revision `88a86bf44a0cb03664931f7ef15201b95fa11255` passes 710/710 library tests and is pinned cleanly. Iris, Wine recognition, and Breast Cancer Wisconsin (Diagnostic) source archives are stored with hashes and UCI source descriptions. `src/framework_validation/benchmark.rs` implements a seeded stratified holdout runner for those datasets and the five integration candidates; the retained two-run diagnostic used the prior submodule revision and agreed exactly on 14/15 prediction sets. Wine KNN changed across repeated same-seed executions and failed save/load prediction equivalence in one run. The determinism fix has not yet been checked against the full dataset/model matrix. External comparisons, resource profiling, broader repeated-seed study, and CLI/API equivalence remain open. Therefore the full framework-validation gate is still pending, and 2048 smoke evidence must not be presented as framework validation.

## Implementation Record

- The AutoML library suite passes 710/710 on the published `88a86bf` revision, and source/API/model/serialization smoke checks pass. These are capability checks only, not matched dataset results.
- Dataset acquisition is complete for the three named UCI datasets. Two independent processes used seed 42, stratified 80/20 partitions, no scaling, five declared model types, and retained split manifests, predictions, metrics, and model artifacts against the prior AutoML revision. Fourteen of fifteen prediction sets match; Wine KNN differs and its save/load equivalence failed once. The published determinism fix is now pinned but the dataset/model matrix has not been rerun against it. External baselines, memory profiling, broader repeated-seed reproducibility, and CLI/library equivalence remain pending.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.

## Open questions

- Add matched external baselines, memory capture, and repeated seeds; then compare outcomes under the declared protocol. The initial one-seed AutoML run is diagnostic only and does not satisfy the full validation gate.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
