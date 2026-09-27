# Corrected matched-grid diagnostic across three split seeds

This protocol-v2 rerun replaces the invalidated pilot documented in
[`matched-grid-search-multi-seed-2026-09-27/`](../matched-grid-search-multi-seed-2026-09-27/README.md).
The earlier runner let AutoML fit on the external selection-validation rows;
all those earlier metrics are withdrawn.

## Protocol correction

For each dataset and seed, the stratified 80/20 outer split is followed by a
per-class trailing 10% selection-validation split. AutoML receives only the
remaining fit rows. Because `TrainEngine::fit` applies a second per-class
trailing 10% holdback, the protocol records those effective model-training
rows separately. The scikit-learn runner fits on the same effective rows.
Neither implementation receives the external selection-validation rows or
outer test rows during fitting. Configuration selection uses only validation
accuracy; the outer test is evaluated after selection. Protocol schema v2
records both holdbacks, and the updated verifier rejects v1 runs and asserts
row disjointness, artifact hashes, selected grid points, and recomputed test
metrics.

The grid remains six `(n_estimators, max_depth)` points for RandomForest and
ExtraTrees on Iris, Wine, and Wisconsin Diagnostic. Split seeds are 42, 2026,
and 2027. Each of the three run directories has its own protocol, manifests,
models, predictions, and passing verification:

- [`seed 42`](../matched-grid-search-corrected-2026-09-27-seed42/)
- [`seed 2026`](../matched-grid-search-corrected-2026-09-27-seed2026/)
- [`seed 2027`](../matched-grid-search-corrected-2026-09-27-seed2027/)

## Results

Across three seeds and six dataset/model cases, each implementation completed
108 candidate fits. AutoML and scikit-learn selected the same configuration in
13/18 case-seed observations. The three Iris and three Wine cases per model
matched across all seeds. In Wisconsin Diagnostic, ExtraTrees selected
different configurations in all three splits, and RandomForest matched on one
of three. Outer-test label agreement ranged from 0.917 to 1.000. These are
descriptive split and implementation sensitivities; no inferential tests were
performed, and the result does not establish optimizer quality or a framework
winner.

Rebuild the combined records from the verified run artifacts with:

```sh
python3 scripts/verify_matched_framework_search.py --report-dir \
  reports/framework_validation/matched-grid-search-corrected-2026-09-27-seed42
python3 scripts/verify_matched_framework_search.py --report-dir \
  reports/framework_validation/matched-grid-search-corrected-2026-09-27-seed2026
python3 scripts/verify_matched_framework_search.py --report-dir \
  reports/framework_validation/matched-grid-search-corrected-2026-09-27-seed2027
python3 scripts/summarize_matched_framework_search.py \
  --run-dir reports/framework_validation/matched-grid-search-corrected-2026-09-27-seed42 \
  --run-dir reports/framework_validation/matched-grid-search-corrected-2026-09-27-seed2026 \
  --run-dir reports/framework_validation/matched-grid-search-corrected-2026-09-27-seed2027 \
  --output-dir reports/framework_validation/matched-grid-search-corrected-multi-seed-2026-09-27
```

This is still a fixed grid, not HyperOptX versus a reference optimizer. Broader
dataset coverage, per-model memory, CLI/library parity, and independent
replication remain open. Scale collection and confirmatory 2048 policy
evaluation also remain open.
