# HyperOptX grouped-CV wiring smoke (2026-09-27)

This retained fixture verifies the optional training/tuning command path on
pinned AutoML `82d848323eed5e2af86d046d529916c448f2442c`. It is implementation
plumbing evidence only.

## Fixture and run

`scripts/create_hyperopt_smoke_data.py` selects source rows from the retained
2-game rollout pilot, then reuses two rows for each of the four action labels
across ten synthetic game IDs. The result has 80 rows, eight rows per synthetic
group, with the final two groups reserved by the training CLI. Reused source
states mean these IDs are not independent games and any accuracy is not a
quality estimate. `fixture-manifest.json` records source, generator, training,
and metadata digests.

The versioned two-trial TPE configuration searches 2–3 trees and depth 1–2;
the run uses RandomForest, two grouped folds, seed `90628`, and the same 17
features used by the canonical schema. Both invocations completed both trials,
selected two trees at depth one, achieved grouped-CV fixture accuracy `0.296875`,
and saved a model and study artifact. The reserved two-group tail was not used
for fitting.

Reproduce the fixture and both runs with:

```sh
python3 scripts/create_hyperopt_smoke_data.py

cargo run -- train \
  --data reports/configuration_smokes/2026-09-27/synthetic-training.csv \
  --metadata reports/configuration_smokes/2026-09-27/synthetic-metadata.csv \
  --model random_forest --cv-folds 2 --seed 90628 \
  --hyperopt-config reports/configuration_smokes/2026-09-27/hyperopt-search.json \
  --output reports/configuration_smokes/2026-09-27/policy.model.json

cargo run -- train \
  --data reports/configuration_smokes/2026-09-27/synthetic-training.csv \
  --metadata reports/configuration_smokes/2026-09-27/synthetic-metadata.csv \
  --model random_forest --cv-folds 2 --seed 90628 \
  --hyperopt-config reports/configuration_smokes/2026-09-27/hyperopt-search.json \
  --output reports/configuration_smokes/2026-09-27/policy-repeat.model.json

python3 scripts/compare_hyperopt_smoke_runs.py \
  reports/configuration_smokes/2026-09-27
```

## Retained evidence and limits

The two model artifacts have different file hashes because
`metrics.training_time_secs` differs. After removing that timing field, their
model JSON is equal. Study trial parameters and values also match after
excluding machine-dependent durations. The comparison script enforces both
claims and writes `repeatability-comparison.json`.

The per-run training manifests record input/configuration digests, the actual
AutoML pin, seed derivations, selected parameters, and model digest. Both study
files retain each trial's parameters, score, and measured duration. This smoke
does not establish tuning efficacy, 2048 policy quality, general framework
performance, or broad repeated-fit behavior.
