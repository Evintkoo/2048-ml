# Plan 04 — Rust-Native AutoML Framework Validation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-30).** Four split seeds have exact process repeats; a corrected six-candidate
> grid pilot, a 15-case CLI/API parity check, and one-process-per-case fit-phase RSS profiles are retained.
> Broader optimizer-budget matching and model-object-only memory measurement remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework
validation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy
learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats API/capability validation as partial evidence, not full framework validation.** Local
tests exercise model/task compatibility, grouped splitting, optimizer API, serialization, and root
integration. Fixed-protocol standard-dataset diagnostics cover seeds 42, 2026, 2027, and 2028, with two
processes repeating each same-seed split and exact predictions in all 15 model/dataset pairs per seed. A
fixed-configuration sklearn baseline and aggregate resource probes are retained. The corrected grid gives
AutoML and scikit-learn the same six configurations per model/dataset and three seeds, but does not match
their optimizer algorithms or include a manual-selection baseline. A CLI/API parity test covers all 15
dataset/model combinations. Fit-phase RSS profiles cover the same 15 cases in separate processes; they
include fit temporaries and allocator retention and do not isolate the persistent model object.

## 1. Purpose

Validate the independent Rust-native `Evintkoo/automl` architecture before interpreting any 2048 result.
This is the primary framework-evaluation track and is separate from the 2048 application benchmark.

## 2. Validation Questions

1. Does the architecture provide coherent data, configuration, model, and artifact contracts?
2. Do the documented APIs and supported model types exist and behave correctly?
3. Does preprocessing avoid train/test leakage?
4. Are cross-validation, metrics, serialization, and prediction correct?
5. Are seeds, parallel execution, and repeated runs reproducible?
6. How do accuracy, runtime, memory use, failure rate, and search efficiency compare with established
  baselines?
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

Use a small, reproducible set of standard tabular classification datasets before expanding the scope. The
initial UCI archives and their hashes are recorded in
[`data/framework_validation/README.md`](../../../data/framework_validation/README.md). The three listed
datasets have been fetched unchanged; the optional larger Adult/OpenML/UCI dataset is not yet selected.

| Dataset | Purpose | Primary Metrics |
|---------|---------|-----------------|
| Iris | Small smoke test and API correctness | Accuracy, macro-F1, runtime |
| Wine | Multiclass preprocessing and model comparison | Accuracy, macro-F1, runtime |
| Breast Cancer Wisconsin | Binary classification and scaling behavior | ROC-AUC, accuracy, macro-F1 |
| Adult or another declared OpenML/UCI dataset | Larger-data resource behavior | Accuracy/ROC-AUC, memory, runtime |

Dataset versions, download hashes, preprocessing, splits, and licenses must be recorded. These datasets
validate the framework; they are not evidence that the framework is optimal for every ML task.

### Initial execution protocol

The initial diagnostic used seed `42`, a stratified 80/20 outer train/test
split, raw numeric predictors without scaling, and the five classifier types
already verified for the 2048 integration: RandomForest, ExtraTrees, AdaBoost,
KNN, and NaiveBayes. Each model uses 32 estimators, max depth 8, and AutoML's
internal validation fraction of 0.1 within the outer training partition. The
outer test rows are passed only to prediction. Dataset-specific split manifests
retain source row indices and the exact seed. Follow-up runs used seeds `2026`
and `2027`/`2028` with the same protocol; each ran the full 15-case matrix once.
Together these are fixed-protocol split diagnostics, not a comparative finding
or final framework validation.

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

Passing these criteria establishes framework usability for the study; it does not by itself establish
superiority over other AutoML systems. The current integration check demonstrates semantic parity on all
15 declared dataset/model combinations; it does not require byte-identical serialized model files.

## 6. Metrics

Report task-appropriate predictive metrics, wall-clock time, CPU and memory use, search budget,
reproducibility, failure rate, model reload equivalence, and configuration/API consistency. Framework
quality must not be inferred from 2048 score alone.

## 7. Comparison Rules

Framework comparisons use named datasets, fixed splits, explicit preprocessing assumptions, hardware
descriptions, dependency versions, and matched search budgets. Baselines must be selected by capability
rather than by convenient score. Established libraries are comparison baselines only; they are not used
to train the core 2048 models.

## 8. Relationship to the 2048 Study

The framework validation gate must pass before the main 2048 training milestone. If a required capability
fails, the failure becomes a documented framework result and the affected 2048 claim is not made. The
final thesis must report both successful capabilities and negative framework findings.

## 9. Execution Status

The pinned framework's API, model probability shapes, group splitter, optimizer API, model serialization,
and 2048 integration smoke paths have been checked in `src/framework_validation.rs`; AutoML revision
`82d848323eed5e2af86d046d529916c448f2442c` passes 712/712 library tests and is pinned cleanly. Iris, Wine
recognition, and Breast Cancer Wisconsin (Diagnostic) source archives are stored with hashes and UCI
source descriptions. `src/framework_validation/benchmark.rs` implements a seeded stratified holdout
runner for those datasets and five integration candidates. Two independent processes under the fixed
revision used the same splits for seeds 42, 2026, 2027, and 2028, succeeded on 15/15 cases per run, matched all
15 prediction sets exactly per split, and preserved predictions through save/load. A fixed-configuration
sklearn comparison and aggregate process resource measurements are retained. The corrected shared grid
matches six candidate configurations across AutoML and scikit-learn but does not compare their optimizer
algorithms or manual selection. Fit-phase RSS profiles cover 15 cases in separate processes but do not
isolate model-object memory.

CLI/API parity is checked by `tests/framework_validation_api_parity.rs`: the public Rust API and
`framework-validate` command use seed 42 across three datasets and five models. Metrics, class labels,
split manifests, and prediction CSV bytes match for all 15 pairs. Fit times, output paths, hashes, and model
JSON bytes are excluded; the model JSON hashes differed between the two invocations. Broader optimizer
budget matching and isolated model memory remain open. The full framework-validation gate is partial, and
2048 smoke evidence must not be presented as framework validation.

The optional `--profile-fit-memory` mode samples current-process RSS every 5 ms during `TrainEngine::fit`.
The reported increment is peak RSS during fitting minus RSS sampled immediately before fit after input
frames and model configuration are ready. It includes fit temporaries and allocator retention, so it is
not a model-object-only memory measurement. The retained resource matrix uses one process per case.

## Implementation Record

- The pinned AutoML library suite passes 712/712 on `82d848323eed5e2af86d046d529916c448f2442c`;
  source/API/model/serialization smoke checks pass. These are capability checks only, not matched dataset
  results.
- Dataset acquisition is complete for the three named UCI datasets. Two independent processes used seeds
  42, 2026, 2027, and 2028 with stratified 80/20 partitions, no scaling, five declared model types, and retained
  split manifests, predictions, metrics, and model artifacts against the current pin. Both matrix runs
  per seed succeeded in all 15 dataset/model cases; predictions matched in 15/15 pairs and save/load
  equivalence passed throughout. These two-process same-split repeats are diagnostic only. The prior
  `88a86bf` Wine KNN disagreement and save/load failure are historical and fixed in the current `82d8483`
  pin. A fixed-configuration scikit-learn baseline and aggregate
  process resource probes are retained. The corrected shared grid compares six fixed configurations with
  equal fit counts per implementation on three datasets and three seeds; optimizer algorithms and manual
  selection are not compared.
- Two additional split seeds, 2026 and 2027, each have two independent processes using the same configuration. Both runs per seed succeeded in 15/15 cases and matched prediction CSVs exactly; manifests, split rows, predictions, models, dependency pin, source revision, and dataset hashes are retained. Seed 2028 adds another two-process exact repeat across all 15 cases. These broaden observed split coverage but do not substitute for independent replication.
- The comparison-only scikit-learn 1.6.1 baseline succeeds and repeats exactly across two runs on the
  seed-42 outer splits, with 8/15 exact label matches to AutoML. It uses related fixed settings, not a
  matched search budget; it is a diagnostic, not a winner determination.
- One whole-matrix same-host resource probe per implementation records elapsed time and process peak RSS.
  Process startup and implementation boundaries differ, and results are aggregate rather than per model;
  no speed or memory superiority claim follows.
- CLI/API semantic output parity passes across all 15 cases: metrics, splits, and prediction bytes match.
  Model JSON hashes differ, and serialized byte equality is outside the tested contract.
- Fit-phase RSS profiles were collected in separate processes for all 15 cases. The sampled incremental
  range is 3,391,488–8,798,208 bytes on this host and seed; polling may miss short peaks and the measure
  includes temporary fitting allocations.
- Automated verification passes: `cargo test` (40 tests), `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`,
  `python3 scripts/verify_framework_fit_memory_profiles.py`, and the Planout checker (0 failures).
  The scripts also pass `python3 -m py_compile`.

- The final repository validation after adding the public library API passes `cargo test` (38 library,
  one binary, and one 15-case integration test; 40 total), `cargo fmt --check`, and
  `cargo clippy --all-targets -- -D warnings`. Clippy reports one warning inside the pinned AutoML
  submodule when compiling that dependency, but the root crate checks pass.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/03-Comparison/04-framework-validation.md`
  exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits
  0.
7. `grep -q '^## Later$' plans/07-Benchmarking/03-Comparison/04-framework-validation.md` exits 0.
8. Run the external Planout `check-plan.sh` on this file; it exits 0.

## Open questions

- Add optimizer-budget comparisons against manual selection, and measure persistent model-object memory
  separately from fit-phase process RSS. The shared candidate grid, current resource profiles, and four
  split-seed diagnostics do not satisfy the full validation gate.
- No measurement protocol for model-object-only memory or predeclared manual-selection/search-budget
  treatment exists in the plan. Choosing one retroactively would change the research comparison, so these
  criteria remain open pending a declared protocol.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its
  prerequisites, compute budget, and measurable acceptance evidence are available.
