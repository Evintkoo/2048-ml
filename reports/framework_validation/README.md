# Initial UCI framework-validation run

This is a diagnostic fixed-split run, not a final framework comparison. Dataset
archives, source descriptions, licenses, and download checksums are documented
in [`../../data/framework_validation/README.md`](../../data/framework_validation/README.md).

## Protocol

- Dataset split: stratified 80/20 train/test, with seed 42 (per-dataset split
  seeds are recorded in each split JSON).
- Models: RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes.
- Configuration: 32 estimators, maximum depth 8, AutoML internal validation
  fraction 0.1 within the outer training partition, raw numeric features, no
  feature scaling. WDBC's ID field is excluded.
- Metrics: outer-test accuracy, macro-F1, confusion matrix, and binary
  ROC-AUC when AutoML returns two probability columns. The outer test rows are
  excluded from fitting and model selection.
- Repetition: two independent process runs with the same seed and split. Exact
  prediction equality is a diagnostic only; two runs do not establish broad
  reproducibility.

The command is `cargo run -- framework-validate --seed 42 --test-fraction 0.2`.
The second run used `--output-dir reports/framework_validation/run-2`.
Both run directories retain split manifests, model artifacts, prediction CSVs,
result JSON, and run manifests. The comparison script is
[`../../scripts/compare_framework_validation_runs.py`](../../scripts/compare_framework_validation_runs.py).

## Run 1 results

| Dataset | Model | Accuracy | Macro-F1 | ROC-AUC (binary) | Save/load predictions |
|---|---|---:|---:|---:|---|
| Iris | RandomForest | 0.900 | 0.898 | — | Match |
| Iris | ExtraTrees | 0.867 | 0.865 | — | Match |
| Iris | AdaBoost | 0.867 | 0.867 | — | Match |
| Iris | KNN | 0.933 | 0.933 | — | Match |
| Iris | NaiveBayes | 0.900 | 0.900 | — | Match |
| Wine | RandomForest | 0.972 | 0.974 | — | Match |
| Wine | ExtraTrees | 0.972 | 0.974 | — | Match |
| Wine | AdaBoost | 0.944 | 0.949 | — | Match |
| Wine | KNN | 0.833 | 0.830 | — | **Mismatch** |
| Wine | NaiveBayes | 0.972 | 0.974 | — | Match |
| Breast Cancer Wisconsin (Diagnostic) | RandomForest | 0.947 | 0.943 | 0.965 | Match |
| Breast Cancer Wisconsin (Diagnostic) | ExtraTrees | 0.947 | 0.943 | 0.982 | Match |
| Breast Cancer Wisconsin (Diagnostic) | AdaBoost | 0.920 | 0.915 | 0.975 | Match |
| Breast Cancer Wisconsin (Diagnostic) | KNN | 0.929 | 0.923 | 0.960 | Match |
| Breast Cancer Wisconsin (Diagnostic) | NaiveBayes | 0.929 | 0.924 | 0.975 | Match |

## Repeatability outcome

The independent run had the same seed and identical train/test row indices.
Predictions matched exactly for 14 of 15 dataset/model pairs. Wine KNN changed
from accuracy 0.833 to 0.806 and macro-F1 0.830 to 0.798; save/load prediction
equivalence failed in the first run and passed in the second. This is a
reproducibility and serialization failure to investigate, not evidence for or
against the relative quality of the model families.

## Limits

These datasets are small; this run uses one split, two repetitions, one model
configuration, and no external library baseline. No memory profile or
CLI/library equivalence study was performed. Reported scores are not evidence
of framework superiority, and game scores remain separate application
evidence. The current local AutoML worktree is dirty, as are the root sources;
the exact pinned commit, dirty-state flags, lockfile hash, Rust version, host,
and source revision are retained in each run manifest.
