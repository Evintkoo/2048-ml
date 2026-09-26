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

## Current pinned result: AutoML `82d848323eed5e2af86d046d529916c448f2442c`

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

This is still one dataset split, one seed, and one fixed model configuration.
There is no matched external-framework baseline, memory profile, CLI/API
equivalence study, or independent replication. Scores are not a framework
superiority result, and game scores remain separate application evidence.
