# Independent five-game classifier-label holdout

This diagnostic fits each of the five probability-compatible AutoML candidates
on the 20 earlier rollout games (seeds 90627–90646) and evaluates action-label
predictions on five later games (seeds 90647–90651). It uses the canonical 17
state values and the existing grouped-CV training path. This is an independent
game-group label holdout, not a held-out game-score or policy-quality result.

## Protocol

- Root source revision: `4d6a0a7d91318a3d8159436e3f2fd79d9aaf1d6b`.
- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.
- Training data: 2,447 rows from 20 games, seeds 90627–90646.
- Holdout data: 541 rows from five disjoint, later games, seeds 90647–90651.
- The source metadata restarts game IDs at zero for each collection. The runner
  remaps holdout IDs to 20–24 and row indices to 2,447–2,987 before combining
  datasets; it does not change state or action values.
- `--development-fraction 0.8` reserves exactly the final five of 25 ordered
  game groups. Five-fold GroupKFold runs on the first 20 groups. Each candidate
  uses the root CLI's fixed defaults; there is no hyperparameter search.
- The training engine also performs its configured internal row-level
  validation split within the first 20 games. It never sees the five holdout
  groups.

Reproduce the fits and joined inputs with:

```sh
python3 scripts/run_independent_classifier_holdout.py \
  --output-dir /tmp/2048-ml-independent-holdout
```

Verify source and joined-data digests, group separation, all five serialized
models and prediction files, exact holdout row identities, and independently
recomputed metrics with:

```sh
python3 scripts/verify_independent_classifier_holdout.py \
  --report-dir /tmp/2048-ml-independent-holdout
```

The runner refuses to overwrite an existing output directory.

Five training-seed fits per candidate were run with seeds 90627–90631 on the
same joined input and holdout. Reproduce them in fresh directories and
summarize the fit-seed spread with:

```sh
for seed in 90627 90628 90629 90630 90631; do
  out="/tmp/2048-ml-independent-repeats/seed-${seed}"
  python3 scripts/run_independent_classifier_holdout.py \
    --training-seed "$seed" --output-dir "$out"
  python3 scripts/verify_independent_classifier_holdout.py --report-dir "$out"
done
python3 scripts/summarize_independent_classifier_repeats.py \
  --repeats-dir /tmp/2048-ml-independent-repeats
```

## Results

| Candidate | Grouped-CV accuracy | Grouped-CV macro-F1 | Holdout accuracy | Holdout macro-F1 |
|---|---:|---:|---:|---:|
| RandomForest | 0.2866 | 0.2761 | 0.2828 | 0.2748 |
| ExtraTrees | 0.2948 | 0.2326 | 0.2662 | 0.2035 |
| AdaBoost | 0.2811 | 0.2615 | 0.2717 | 0.2558 |
| KNN | 0.2550 | 0.2497 | 0.2588 | 0.2539 |
| NaiveBayes | 0.2631 | 0.2339 | 0.2791 | 0.2505 |

The leading model differs between grouped-CV accuracy and holdout accuracy;
macro-F1 also differs by model. The holdout contains only five games from one
host and one simulator/labeling protocol. These results describe this fixed
fit and seed range, do not estimate variation across training runs or hosts,
and must not be used for model selection, policy-quality claims, or AutoML
superiority claims.

## Repeated training-seed diagnostic

Each candidate was also trained five times with seeds 90627–90631 using the
same 2,447 training rows and the same 541 holdout rows. The table gives means
and sample standard deviations across those five fit seeds; grouped-CV
columns summarize each run's five-fold metric, while holdout columns use the
same fixed rows in every run.

| Candidate | Grouped-CV accuracy | Holdout accuracy | Holdout macro-F1 |
|---|---:|---:|---:|
| RandomForest | 0.2842 ± 0.0014 | 0.2784 ± 0.0028 | 0.2716 ± 0.0023 |
| ExtraTrees | 0.2961 ± 0.0016 | 0.2725 ± 0.0059 | 0.2101 ± 0.0074 |
| AdaBoost | 0.2811 ± 0.0000 | 0.2717 ± 0.0000 | 0.2558 ± 0.0000 |
| KNN | 0.2550 ± 0.0000 | 0.2588 ± 0.0000 | 0.2539 ± 0.0000 |
| NaiveBayes | 0.2631 ± 0.0000 | 0.2791 ± 0.0000 | 0.2505 ± 0.0000 |

These spreads describe sensitivity to the selected training seeds on this one
corpus. They do not estimate variation across data collection runs, held-out
game sets, machines, or labeling protocols. The unchanged results for three
models apply only to these five seeds and this configuration. No statistical
winner test was planned or performed.

## Artifacts and integrity

- `protocol-manifest.json` records source-data manifests, seeds, group mapping,
  configuration, and joined CSV digests. SHA-256:
  `5338603d9be97846c8d82c6c2430ca8e686ba0130619b3988fbca03c7e0a5773`.
- `verification.json` records independent checks and hashes for each model,
  manifest, and prediction file. SHA-256:
  `ffd182a853199d3ad26c5d874e2520cf3bd176bcfbefc5ac371cddc7e6faafa7`.
- `joined-training.csv` and `joined-training.metadata.csv` retain the exact
  combined input. The five model subdirectories retain serialized models,
  manifests, and row-aligned holdout predictions.
- `repeated-fit/seed-90627/` through `seed-90631/` retain each repeat's joined
  inputs, five model/manifest/prediction sets, protocol, and verification.
  `repeated-fit-runs.csv` SHA-256:
  `bbd793d889b9c58595d3152d9a3664b431228aa2a4636574a4c40e8daa7a43e6`;
  `repeated-fit-summary.json` SHA-256:
  `ffec5c62320e363b5ec2d0e195ad410ee1150f1f149ead7b98b40684cc1579d5`.
