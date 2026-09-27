# Matched six-configuration grid-search diagnostic

> **SUPERSEDED — DO NOT USE THESE METRICS.** Source audit found that the first
> AutoML runner passed the external selection-validation rows into
> `TrainEngine::fit` before scoring those rows. That leaked selection labels
> into training. The original seed-42 artifacts below are retained only as an
> audit trail; their metrics and the derived three-seed summary are withdrawn.
> Use the corrected, protocol-v2 runs linked from
> [`matched-grid-search-corrected-multi-seed-2026-09-27`](../matched-grid-search-corrected-multi-seed-2026-09-27/README.md).

This pilot compares the AutoML and scikit-learn RandomForest and ExtraTrees implementations using the
same six fixed configurations, the same train/validation/test source rows, and six candidate fits per
dataset/model/implementation. It adds a matched candidate-count budget for the two tree models supported by
the current search adapters. It does not compare HyperOptX with a scikit-learn optimizer, and it does not
establish a general framework winner.

## Protocol

- Datasets: Iris, Wine, and Wisconsin Diagnostic.
- Models: RandomForest and ExtraTrees.
- Outer split: stratified 80/20, seed 42 (dataset-specific split seeds are retained in the protocol).
- Inner validation: the final 10% per class of outer-training rows, matching the pinned AutoML training
  engine's holdback. Configuration selection uses inner-validation accuracy only; the outer test set is
  scored after selection.
- Shared grid, in order: `(n_estimators, max_depth)` of `(16,4)`, `(16,8)`, `(32,4)`, `(32,8)`, `(64,4)`,
  `(64,8)`. Ties select the first configuration in that order.
- Budget: six configuration fits per dataset/model/implementation, for 36 trial fits per implementation.
  Both use the same case seed for all six grid candidates, with one thread.
- AutoML: `82d848323eed5e2af86d046d529916c448f2442c`.
- scikit-learn comparison packages: scikit-learn 1.6.1, NumPy 2.0.2, SciPy 1.13.1, joblib 1.5.3, and
  threadpoolctl 3.6.0.
- Root revision recorded at execution: `9141c252350fe97c816ed31a5327d600013ef4f5`; the AutoML manifest
  records that the root worktree was dirty. The runner and verifier are retained in this repository.

Reproduce to a new output directory (the runner refuses to overwrite existing artifacts):

```sh
cargo run -- framework-search-validate \
  --data-dir data/framework_validation \
  --output-dir reports/framework_validation/<run-dir> \
  --seed 42 --test-fraction 0.2 --validation-fraction 0.1

python3 scripts/run_sklearn_matched_grid_search.py \
  --protocol reports/framework_validation/<run-dir>/matched-search-protocol.json \
  --data-dir data/framework_validation \
  --output-dir reports/framework_validation/<run-dir>/sklearn

python3 scripts/verify_matched_framework_search.py \
  --report-dir reports/framework_validation/<run-dir>
```

## Results

Both implementations selected the same configuration in all six cases: 16 trees at depth 4, except
Wisconsin Diagnostic ExtraTrees, where both selected 16 trees at depth 8. The held-out metrics were:

| Dataset | Model | AutoML accuracy | sklearn accuracy | AutoML macro-F1 | sklearn macro-F1 | Outer-test label agreement |
|---|---|---:|---:|---:|---:|---:|
| Iris | RandomForest | 0.900 | 0.867 | 0.898 | 0.865 | 0.967 |
| Iris | ExtraTrees | 0.867 | 0.867 | 0.865 | 0.865 | 1.000 |
| Wine | RandomForest | 0.972 | 1.000 | 0.974 | 1.000 | 0.972 |
| Wine | ExtraTrees | 0.944 | 0.944 | 0.949 | 0.945 | 0.944 |
| Wisconsin Diagnostic | RandomForest | 0.947 | 0.938 | 0.943 | 0.933 | 0.973 |
| Wisconsin Diagnostic | ExtraTrees | 0.938 | 0.938 | 0.934 | 0.933 | 0.982 |

This is one fixed split per dataset, six candidates, and one process run. It tests a shared configuration
budget and records implementation-level score differences; no inferential test was performed. Although
tree count and depth match, feature sampling, split search, randomness, and other implementation defaults
remain different. The six inner validation rows and the test rows are small for Iris and Wine. Timings are
retained per candidate but are not a framework speed claim.

## Verification and artifacts

Run `python3 scripts/verify_matched_framework_search.py` to check the shared protocol digest, candidate
grids, trial counts, selected configurations, train/validation/test row disjointness, model and prediction
hashes, recomputed test metrics, and outer-test label agreement. The retained run passed for all six cases.
`matched-search-verification.json` summarizes the cases. The protocol, AutoML models/results, scikit-learn
models/results, and both manifests are in this directory.
