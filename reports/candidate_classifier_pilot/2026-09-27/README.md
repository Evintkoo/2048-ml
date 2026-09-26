# Five-candidate classifier pilot

This bounded comparison runs the five AutoML candidates that return four action
probabilities on the same 20-game rollout corpus, chronological split, and
seed. It adds classifier diagnostics to the RandomForest-only pilot; it is not
a policy-quality estimate or a model-selection result.

## Protocol

- Root source for the original classifier-only run: `db628f49a0aa9201125fcfbb8cf71cbcf19337cb`.
- Root source for the retained fold-metric rerun: recorded in each manifest as `af022d00b7fe4adc04678543fbd0e7e8743ebc39`, with the uncommitted additive fold-metric changes described here. The rerun changed no model or split configuration.
- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.
- Input training CSV SHA-256: `4b1063b3b5c318e271e7191c77a3d440a3fa912e9a9150fb62ddf0d18abd21b3`.
- Row metadata SHA-256: `3951d0d6abb3c569736a138e4aa0e19780fd420d1ddb59f80b573afdc31ad41b`.
- Global seed: `90627`; five GroupKFold splits; 85% chronological development
  fraction.
- The first 17 game groups provide 2,056 rows for grouped CV and fitting. The
  final three groups (17–19) are held out and provide 391 classifier-label
  rows. They are not used by CV or fitting.
- All candidates use the same 17-value state and rollout-derived action label.
  The CLI's common tree parameter settings do not imply equivalent model
  capacity across different algorithms.
- Root configuration explicitly disables unsupported classical-tree early
  stopping. The framework fit still makes its own row-level validation split
  within development data.

Run the matrix with:

```sh
./scripts/run_candidate_classifier_pilot.sh
```

The script retains each serialized model, manifest, chronological holdout
prediction CSV, and grouped-CV fold metrics. Verify training-data and metadata
hashes, common holdout row identities, artifact digests, grouped-CV summaries,
and recomputed classification metrics with:

```sh
./scripts/verify_candidate_classifier_pilot.py
```

The verifier writes [`verification.json`](verification.json), including the
source and dependency revisions plus all artifact hashes.

## Results

| Candidate | Grouped-CV accuracy (fold mean ± fold SD) | Grouped-CV macro-F1 (fold mean ± fold SD) | Holdout accuracy | Holdout macro-F1 |
|---|---:|---:|---:|
| RandomForest | 0.2828 ± 0.0285 | 0.2728 ± 0.0310 | 0.2967 | 0.2914 |
| ExtraTrees | 0.2751 ± 0.0084 | 0.2579 ± 0.0176 | 0.2864 | 0.2741 |
| AdaBoost | 0.2635 ± 0.0238 | 0.2531 ± 0.0270 | 0.3171 | 0.2949 |
| KNN | 0.2573 ± 0.0131 | 0.2520 ± 0.0123 | 0.2813 | 0.2746 |
| NaiveBayes | 0.2513 ± 0.0147 | 0.2218 ± 0.0228 | 0.2506 | 0.2282 |

Each fold macro-F1 and confusion matrix is retained in the training manifest. Fold SD describes five
unequal-sized game-group folds, not an uncertainty interval over independent datasets or training runs. The outer holdout is only three game groups from a
small development corpus. No valid-action rate, game-score evaluation,
repeated-fit comparison, or statistical winner test is included. These values
must not be used to select a model or claim generalization.

## Artifacts

For each candidate, `<model>.policy.json` is the serialized model,
`<model>.policy.manifest.json` contains input/configuration provenance, fold accuracy and class metrics,
fold confusion matrices, and `<model>.policy.holdout-predictions.csv` contains
row-aligned actual and predicted actions. `verification.json` records digests
for all five sets. The source CSV and metadata remain in
`reports/collection_pilots/2026-09-27-20-game/`.
