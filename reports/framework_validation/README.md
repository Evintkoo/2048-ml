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
global seeds 2026, 2027, and 2028. Each seed has two independent process runs
on the same split. These are split diagnostics and test same-split repeatability;
each run succeeded in 15/15 cases, with no failures.
Mean accuracy over the 15 heterogeneous cases was 0.9367 at seed 2026 and
0.9378 at seed 2027; these arithmetic summaries are descriptive only because
the cases mix datasets and algorithms and are not independent replicates.
The per-case rows, split assignments, predictions, serialized models, source
hashes, and manifests are retained in `pinned-82d8483-seed2026-run-1/`,
`pinned-82d8483-seed2026-run-2/`, and
`pinned-82d8483-seed2027-run-1/`. The seed-2026 comparison artifact
`pinned-82d8483-seed2026-comparison.json` records identical split assignments,
15/15 identical prediction CSVs, and save/load equivalence in both runs. Both
manifests pin AutoML `82d848323eed5e2af86d046d529916c448f2442c` and record root
source revision
`290e1810d447e5b608b116730d28ae0222606938`.

Seed 2027 has two runs; the runs match split assignments and all 15 prediction
CSVs and pass save/load checks. Seed 2028 was also run in two independent
processes. Its 15 cases succeeded in each process, predictions match 15/15,
split assignments match for all three datasets, and save/load checks pass. The mean accuracy over
the 15 heterogeneous seed-2028 cases is 0.9539; this aggregate is descriptive,
not an inferential summary. Its artifacts are in
`pinned-82d8483-seed2027-run-2/`,
`pinned-82d8483-seed2027-comparison.json`,
`pinned-82d8483-seed2028-run-1/`,
`pinned-82d8483-seed2028-run-2/`, and
`pinned-82d8483-seed2028-comparison.json`. The seed-2028 manifests record root
revision `93aa4ef49ab39477160a74d90a299acda795e3d4` and the pinned AutoML
revision above.

The seed-2028 runs used the same command with separate output directories:

```sh
cargo run -- framework-validate --seed 2028 --test-fraction 0.2 \
  --output-dir reports/framework_validation/pinned-82d8483-seed2028-run-1
cargo run -- framework-validate --seed 2028 --test-fraction 0.2 \
  --output-dir reports/framework_validation/pinned-82d8483-seed2028-run-2
```

These additional runs show that the runner and all five candidates complete under
three other stratified splits and that each split repeats exactly in two
processes. They do not establish confidence intervals across datasets, tune-search
quality, equivalence with an external implementation, or a framework winner. The
seed-42 comparison and its limits are reported separately above.

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

### Per-case fit/predict timing summary

The two repeated seed-42 result files also retain a `fit_predict_seconds` measurement for each
dataset/model case. `scripts/summarize_framework_fit_timings.py` reproduces
[`pinned-82d8483-per-case-timings.csv`](pinned-82d8483-per-case-timings.csv), which records both
observations and their median for all 15 cases. This is a descriptive two-run timing summary on one
host. The timer includes `TrainEngine::fit`, `predict`, and `predict_proba`; it excludes data loading,
serialization, process startup, and peak memory. The separate aggregate process probe remains the only
RSS measurement. These timings do not form a matched framework performance comparison.

### Repeated single-thread per-case timing diagnostic

Two additional AutoML seed-42 matrices were run sequentially with
`RAYON_NUM_THREADS=1`, matching the one-thread setting of the two retained
scikit-learn runs. The runner completed all 15 dataset/model cases in each
process, reproduced the same outer splits, matched predictions in 15/15 cases,
and passed save/load checks. The per-case timer covers fit, prediction, and
probability prediction for both implementations; data loading and process
startup are excluded.

[`pinned-82d8483-vs-sklearn-seed42-single-thread-fit-predict.csv`](pinned-82d8483-vs-sklearn-seed42-single-thread-fit-predict.csv)
contains both observations and their medians for all 15 cases. The comparison
script is [`../../scripts/compare_framework_fit_timings.py`](../../scripts/compare_framework_fit_timings.py);
the CSV SHA-256 is
`bbb9513ecd4ab1fb652a04c82903ab4bcb38b87c2e011dc368ae5062350017a4`.
Reproduce the AutoML runs with:

```sh
RAYON_NUM_THREADS=1 cargo run -- framework-validate --seed 42 --test-fraction 0.2 \
  --output-dir reports/framework_validation/pinned-82d8483-rayon1-seed42-run-1
RAYON_NUM_THREADS=1 cargo run -- framework-validate --seed 42 --test-fraction 0.2 \
  --output-dir reports/framework_validation/pinned-82d8483-rayon1-seed42-run-2
python3 scripts/compare_framework_validation_runs.py \
  reports/framework_validation/pinned-82d8483-rayon1-seed42-run-1 \
  reports/framework_validation/pinned-82d8483-rayon1-seed42-run-2 \
  --output reports/framework_validation/pinned-82d8483-rayon1-seed42-comparison.json
python3 scripts/compare_framework_fit_timings.py
```

Thread count, seed, splits, and top-level estimator settings are aligned, but
model-specific defaults and implementation details differ, and no optimizer
search budget is compared. These two-run, one-host measurements are descriptive
only: the timing ratios do not establish a general speed advantage. Matched
search-budget profiling, model-only memory, and broader hardware repetitions
remain open.

### Isolated per-case process resources (2026-09-27)

The resource runner executes one dataset/model case per process and measures peak
RSS and wall time with macOS `/usr/bin/time -l`. It ran two repeats for each of
15 dataset/model cases for AutoML and scikit-learn (60 processes total), using
the seed-42 outer splits, one thread, 32 estimators, maximum depth 8, and a
20% test split. All cases succeeded, and split row assignments match between
the implementations. Across these runs, AutoML process peak RSS ranged from
27,410,432 to 27,443,200 bytes; scikit-learn ranged from 154,779,648 to
157,024,256 bytes. Process wall times ranged from 0.07 to 0.97 seconds.

These are process-level observations: RSS includes runtime/library startup and
dataset handling, and wall time includes process startup. Two repeats on one
host do not estimate hardware variation. Model implementations and some
defaults differ, and optimizer search budgets are not matched, so these
measurements do not establish framework superiority or model-only resource use.

The run-level measurements, per-case summaries, and manifests are retained in
[`case-resource-matrix/`](case-resource-matrix/). SHA-256 values are
`6f155c97fe0220053565ba4dd2c11a11d6d3b92934f11b194df251a7fec6e5ef` for
`case-resource-runs.csv`,
`0822ec664fea5ee0b1a5224e1b7bffed97f536727d399195dd4fb0119b0ea590` for
`case-resource-summary.csv`, and
`97eddffb5d2e5cd6dd92a344b93405371868d3a1169c8709ec5846c4eccec3d0` for
`case-resource-manifest.json`. Reproduce on macOS with:

```sh
cargo build
python3 scripts/run_framework_case_resource_matrix.py
```

The runner supports `--dataset`, `--model`, and `--repeats` to select a subset.

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

The framework diagnostic covers four dataset split seeds and one fixed model
configuration. Seeds 42, 2026, 2027, and 2028 each have two exact prediction
runs on the same split. The fixed-configuration sklearn comparison is not a
matched search-budget study. Isolated per-case process RSS is available, but it
does not isolate model allocations. CLI/API equivalence and independent replication
remain untested. Scores are not a framework
superiority result, and game scores remain separate application evidence.
