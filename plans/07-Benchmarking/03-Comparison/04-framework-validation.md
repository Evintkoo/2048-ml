# Rust-Native AutoML Framework Validation

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

Use a small, reproducible set of standard tabular classification datasets before expanding the scope:

| Dataset | Purpose | Primary Metrics |
|---------|---------|-----------------|
| Iris | Small smoke test and API correctness | Accuracy, macro-F1, runtime |
| Wine | Multiclass preprocessing and model comparison | Accuracy, macro-F1, runtime |
| Breast Cancer Wisconsin | Binary classification and scaling behavior | ROC-AUC, accuracy, macro-F1 |
| Adult or another declared OpenML/UCI dataset | Larger-data resource behavior | Accuracy/ROC-AUC, memory, runtime |

Dataset versions, download hashes, preprocessing, splits, and licenses must be recorded. These datasets validate the framework; they are not evidence that the framework is optimal for every ML task.

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
