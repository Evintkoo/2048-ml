# Matched shared-grid results across three split seeds

This follow-up extends the fixed six-candidate grid diagnostic across stratified
outer-split seeds 42, 2026, and 2027. It covers RandomForest and ExtraTrees on
Iris, Wine, and Wisconsin Diagnostic. Each seed has a separate protocol,
AutoML run, scikit-learn run, and passing integrity verification in the three
linked run directories:

- [`seed 42`](../matched-grid-search-2026-09-27/README.md)
- [`seed 2026`](../matched-grid-search-2026-09-27-seed2026/)
- [`seed 2027`](../matched-grid-search-2026-09-27-seed2027/)

Every case and implementation evaluated the same six `(n_estimators,
max_depth)` candidates and selected using only per-class trailing 10% inner
validation rows. The outer test rows were reserved for the reported metrics.
Each seed-level verifier checks the protocol and artifacts, shared source-row
partitions, selected configurations, and recomputed outer-test metrics.

## Results

Across the three split seeds, the six dataset/model cases yield 18
case-seed observations and 108 candidate fits per implementation. AutoML and
scikit-learn selected the same configuration in 15 of 18 observations. They
matched in all 12 Iris and Wine observations; the three mismatches occur in
Wisconsin Diagnostic forest cases. Selections were not stable across splits
for Wisconsin Diagnostic ExtraTrees, and one seed differed for Wisconsin
Diagnostic RandomForest. This is evidence that the selected configuration can
depend on the split and implementation. It is not an estimate of optimizer
quality or a framework winner.

Outer-test accuracy differences vary by case and seed; label agreement ranges
from 0.917 to 1.000. No inferential tests were performed. The runs compare a
fixed candidate count and parameter grid, not HyperOptX against an independent
search optimizer. Tree implementations have different defaults and split
behavior, and the three datasets are a narrow diagnostic set.

The reproducible combined records are `matched-grid-multi-seed-cases.csv` and
`matched-grid-multi-seed-summary.json`. Recreate the summary after verifying
each run:

```sh
python3 scripts/verify_matched_framework_search.py --report-dir \
  reports/framework_validation/matched-grid-search-2026-09-27
python3 scripts/verify_matched_framework_search.py --report-dir \
  reports/framework_validation/matched-grid-search-2026-09-27-seed2026
python3 scripts/verify_matched_framework_search.py --report-dir \
  reports/framework_validation/matched-grid-search-2026-09-27-seed2027
python3 scripts/summarize_matched_framework_search.py \
  --run-dir reports/framework_validation/matched-grid-search-2026-09-27 \
  --run-dir reports/framework_validation/matched-grid-search-2026-09-27-seed2026 \
  --run-dir reports/framework_validation/matched-grid-search-2026-09-27-seed2027 \
  --output-dir reports/framework_validation/matched-grid-search-multi-seed-2026-09-27
```

This adds split-seed coverage to the matched candidate-budget pilot. It does
not complete the framework-validation program: per-model memory profiling,
CLI/library equivalence, broader dataset coverage, reference-optimizer
comparison, and independent replication remain open. The 2048 scale and
confirmatory policy prerequisites also remain open.
