# Framework validation: fixed-split diagnostic

This is a bounded diagnostic for the pinned AutoML dependency, not a final
framework comparison or a 2048 model-selection result. Dataset archives,
source descriptions, licenses, and download checksums are documented in
[`../../data/framework_validation/README.md`](../../data/framework_validation/README.md).

## Protocol

- Three datasets: Iris, Wine, and Wisconsin Diagnostic.
- Stratified 80/20 outer split, seed 42; per-dataset split seeds and source row
  indices are retained in each run directory.
- Five AutoML candidates: RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes.
- 32 estimators, maximum depth 8, internal validation fraction 0.1, raw numeric
  features, no scaling. WDBC's ID field is excluded.
- Outer-test accuracy, macro-F1, confusion matrix, and binary ROC-AUC where
  two probability columns are returned. Outer-test rows are excluded from fit.
- Two independent process runs with the same protocol. Exact prediction
  equality is a diagnostic; two runs do not establish broad reproducibility.

Run command:

```sh
cargo run -- framework-validate --data-dir data/framework_validation \
  --output-dir reports/framework_validation/<run-name> \
  --seed 42 --test-fraction 0.2
```

The comparison utility is
[`../../scripts/compare_framework_validation_runs.py`](../../scripts/compare_framework_validation_runs.py).
Artifacts retain split manifests, models, prediction CSVs, result JSON, and
run manifests with dependency revision and environment provenance.

## Fixed-configuration scikit-learn comparison (2026-09-27)

Two comparison-only runs used scikit-learn 1.6.1, NumPy 2.0.2, SciPy 1.13.1,
joblib 1.5.3, and threadpoolctl 3.6.0, pinned in
[`../../requirements-framework-baseline.txt`](../../requirements-framework-baseline.txt).
The runner
[`../../scripts/run_sklearn_framework_baseline.py`](../../scripts/run_sklearn_framework_baseline.py)
reads the AutoML run-1 split manifests, verifies each raw UCI file digest, uses
the same outer train/test rows, and mirrors AutoML's per-class trailing 10%
inner holdback. It records source, split, code, package, configuration, result,
timing, and aggregate process peak-RSS provenance. The sklearn matrix used a
single thread; the AutoML run's Rayon thread count was not fixed, so elapsed
times are not a matched performance comparison.

Both sklearn runs succeeded on all 15 dataset/model cases. Metrics and
prediction CSVs were identical across runs in 15/15 cases. On identical outer
test rows, predicted labels matched the corresponding AutoML run in 8/15 cases.
This checks the comparison plumbing and shows implementation-level prediction
differences; it does not establish a winner. The fixed estimators share the
declared tree counts/depths and core KNN/GaussianNB/AdaBoost settings, but
implementation defaults, random generators, and tree-split behavior are not
identical. No inferential test was applied to this one-split comparison.

| Dataset | Model | AutoML accuracy | sklearn accuracy | AutoML macro-F1 | sklearn macro-F1 | Labels equal |
|---|---|---:|---:|---:|---:|---|
| Iris | RandomForest | 0.900 | 0.833 | 0.898 | 0.833 | No |
| Iris | ExtraTrees | 0.867 | 0.867 | 0.865 | 0.865 | Yes |
| Iris | AdaBoost | 0.867 | 0.900 | 0.867 | 0.898 | No |
| Iris | KNN | 0.933 | 0.933 | 0.933 | 0.933 | Yes |
| Iris | NaiveBayes | 0.900 | 0.900 | 0.900 | 0.900 | Yes |
| Wine | RandomForest | 0.972 | 0.972 | 0.974 | 0.974 | Yes |
| Wine | ExtraTrees | 0.972 | 0.972 | 0.974 | 0.974 | Yes |
| Wine | AdaBoost | 0.944 | 0.972 | 0.949 | 0.974 | No |
| Wine | KNN | 0.806 | 0.806 | 0.798 | 0.798 | Yes |
| Wine | NaiveBayes | 0.972 | 0.972 | 0.974 | 0.974 | Yes |
| Breast Cancer Wisconsin (Diagnostic) | RandomForest | 0.947 | 0.956 | 0.943 | 0.952 | No |
| Breast Cancer Wisconsin (Diagnostic) | ExtraTrees | 0.938 | 0.947 | 0.933 | 0.943 | No |
| Breast Cancer Wisconsin (Diagnostic) | AdaBoost | 0.920 | 0.947 | 0.915 | 0.943 | No |
| Breast Cancer Wisconsin (Diagnostic) | KNN | 0.929 | 0.938 | 0.923 | 0.933 | No |
| Breast Cancer Wisconsin (Diagnostic) | NaiveBayes | 0.929 | 0.929 | 0.924 | 0.924 | Yes |

Artifacts are retained in `sklearn-1.6.1-seed42-run-1/`,
`sklearn-1.6.1-seed42-run-2/`, and
`sklearn-1.6.1-seed42-resource-run-3/`. Each directory includes per-case
prediction CSVs, fixed-split metric JSON, a run manifest, and descriptive
score-difference JSON against AutoML. Runtime is recorded for exploration only;
Python process peak RSS is aggregate across the matrix and is not a per-model
memory profile.

One same-host, single-thread whole-matrix resource probe was also retained in
[`single-thread-resource-comparison.json`](single-thread-resource-comparison.json).
The prebuilt AutoML command completed in 1.33 seconds with 27,426,816 bytes
maximum resident set size; the Python command completed in 1.22 seconds with
158,466,048 bytes. Those process boundaries include different startup and
runtime overheads, and model implementation details differ. The measurements
are raw observations only, not evidence that either framework is faster or
more memory efficient. Per-model memory, repeated hardware runs, and matched
runtime/resource profiling remain open.

## Current pinned result: AutoML `82d848323eed5e2af86d046d529916c448f2442c`

### Additional fixed-protocol seeds (2026-09-27)

To check sensitivity to the one seed in the initial diagnostic, the same
candidate set, 32 estimators, maximum depth 8, raw predictors, 80/20
stratified holdout, and 10% per-class trailing inner holdback were run with
global seeds 2026 and 2027. Each seed has one complete run of all 15
dataset/model cases; these are additional split diagnostics, not repeated fits
on common test rows. Each run succeeded in 15/15 cases, with no failures.
Mean accuracy over the 15 heterogeneous cases was 0.9367 at seed 2026 and
0.9378 at seed 2027; these arithmetic summaries are descriptive only because
the cases mix datasets and algorithms and are not independent replicates.
The per-case rows, split assignments, predictions, serialized models, source
hashes, and manifests are retained in `pinned-82d8483-seed2026-run-1/` and
`pinned-82d8483-seed2027-run-1/`. Both manifests pin AutoML
`82d848323eed5e2af86d046d529916c448f2442c` and record root source revision
`290e1810d447e5b608b116730d28ae0222606938`.

This adds evidence that the runner and all five candidates complete under two
other stratified splits. It does not establish confidence intervals across
datasets, tune-search quality, equivalence with an external implementation,
or a framework winner. Existing seed-42 comparison results and its limits are
reported separately above.

Both final runs used clean AutoML commit `82d848323eed5e2af86d046d529916c448f2442c`,
root source revision `1608ad24709ae133f68a6fed532facbda7f30ed6`, the same split
assignments, and seed 42. Each run succeeded for 15/15 dataset/model cases.
All prediction CSVs matched exactly across runs (15/15), and every model's
reloaded predictions matched its original predictions in each run.

| Dataset | Model | Accuracy | Macro-F1 | ROC-AUC (binary) | Save/load |
|---|---|---:|---:|---:|---|
| Iris | RandomForest | 0.900 | 0.898 | — | Match |
| Iris | ExtraTrees | 0.867 | 0.865 | — | Match |
| Iris | AdaBoost | 0.867 | 0.867 | — | Match |
| Iris | KNN | 0.933 | 0.933 | — | Match |
| Iris | NaiveBayes | 0.900 | 0.900 | — | Match |
| Wine | RandomForest | 0.972 | 0.974 | — | Match |
| Wine | ExtraTrees | 0.972 | 0.974 | — | Match |
| Wine | AdaBoost | 0.944 | 0.949 | — | Match |
| Wine | KNN | 0.806 | 0.798 | — | Match |
| Wine | NaiveBayes | 0.972 | 0.974 | — | Match |
| Breast Cancer Wisconsin (Diagnostic) | RandomForest | 0.947 | 0.943 | 0.965 | Match |
| Breast Cancer Wisconsin (Diagnostic) | ExtraTrees | 0.938 | 0.932 | 0.982 | Match |
| Breast Cancer Wisconsin (Diagnostic) | AdaBoost | 0.920 | 0.915 | 0.975 | Match |
| Breast Cancer Wisconsin (Diagnostic) | KNN | 0.929 | 0.923 | 0.960 | Match |
| Breast Cancer Wisconsin (Diagnostic) | NaiveBayes | 0.929 | 0.924 | 0.975 | Match |

Artifacts:

- `pinned-82d8483-run-1/`
- `pinned-82d8483-run-2/`
- `pinned-82d8483-comparison.json`

## Diagnosis and fix

The preceding pinned commit `88a86bf44a0cb03664931f7ef15201b95fa11255`
produced 13/15 exact matching prediction pairs and a Wine KNN save/load
mismatch in the second run. Source inspection isolated nondeterministic tie
handling: KNN used randomized `HashMap` iteration to resolve tied class votes;
ExtraTrees had ties in feature/leaf selection. AutoML commit `82d8483` fixes
these by applying deterministic label and feature tie rules. Its library suite
passes 712/712 tests, including focused tied-vote and tied-leaf regression
tests.

The pre-fix rerun is retained in `pinned-88a86bf-run-1/`,
`pinned-88a86bf-run-2/`, and `pinned-88a86bf-comparison.json`. It is historical
diagnostic evidence and must not be conflated with the current pinned result.

## Limits

The framework diagnostic now covers three dataset split seeds, with one run
per seed and one fixed model configuration. The fixed-configuration sklearn
comparison is not a matched search-budget study. There is no per-model
resource profile, repeated fits on common split rows across processes, CLI/API
equivalence study, or independent replication. Scores are not a framework
superiority result, and game scores remain separate application evidence.
