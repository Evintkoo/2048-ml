# Pilot policy score evaluation on disjoint game seeds

## Purpose and scope

This exploratory evaluation measures the game-score distributions of five
fixed AutoML policies trained on the same 20-game rollout corpus, alongside
random and heuristic baselines. It was added after an audit found that the
original policy evaluation seed range (84024–94023) overlapped 20 training-game
seeds (90627–90646). This corrected evaluation uses seeds 94024–104023, so no
game seed used for rollout training appears in the score evaluation.

The policies were fitted once each on the same data, development groups, and
seed using AutoML `82d848323eed5e2af86d046d529916c448f2442c`. Their training
CSV digest is `4b1063b3b5c318e271e7191c77a3d440a3fa912e9a9150fb62ddf0d18abd21b3`;
metadata digest is `3951d0d6abb3c569736a138e4aa0e19780fd420d1ddb59f80b573afdc31ad41b`.
The five candidate model artifacts and classifier-pilot verification are in
`reports/candidate_classifier_pilot/2026-09-27/`. The RandomForest score run
uses that five-candidate batch artifact; its per-game outcomes were checked to
match the earlier RandomForest smoke artifact exactly.

Each agent ran 10,000 games using the default 90/10 spawn probability and
1,000-move safety limit. Seeds are global seed plus game ID. All seven score
CSVs have identical seed and game-ID sequences. The five policies are not
independent training repetitions: each represents one fitted model from one
small corpus. Runtime values in manifests were measured while separate runs
executed concurrently and are retained only as provenance, not as performance
comparisons.

## Score summaries

| Rank by mean | Agent | Mean | Median | Sample SD | 95% bootstrap CI for mean |
|---:|---|---:|---:|---:|---:|
| 1 | Heuristic | 8,096.70 | 7,140 | 3,503.15 | [8,028.04, 8,168.45] |
| 2 | Random | 1,086.52 | 1,048 | 527.16 | [1,076.03, 1,097.03] |
| 3 | NaiveBayes | 914.14 | 744 | 546.81 | [903.49, 924.74] |
| 4 | KNN | 887.35 | 776 | 445.03 | [879.02, 895.82] |
| 5 | RandomForest | 864.44 | 740 | 447.96 | [855.48, 873.05] |
| 6 | ExtraTrees | 837.15 | 736 | 459.45 | [827.92, 846.14] |
| 7 | AdaBoost | 765.62 | 680 | 399.06 | [757.52, 773.64] |

This is the observed ordering for these seven fixed agents on this seed set. It
is exploratory evidence, not a selected-model winner or a claim about the
algorithms generally. The ML policies were trained from only 17 development
games (2,056 rows), and there is one fit per candidate. The seed set was not a
preregistered final test; selecting a model from this table would require a new
held-out evaluation.

## Pairwise analysis

`score-comparison.csv` contains all 21 pairwise comparisons in one Holm family.
All comparisons have matching seed sets and use the paired exact sign test,
5,000-replicate paired bootstrap intervals for the first-minus-second mean
score difference (bootstrap seed 94030), and paired Cohen's dz. The sign test
ignores tied score outcomes and is not Wilcoxon. CSV p-values printed as
`0.00000000` are below the output precision, not exact zeros. Use the CSV for
pairwise intervals, effect sizes, and adjusted p-values.

## Reproduction

The retained per-game score CSVs and manifests are:

- `random_10000.csv`
- `heuristic_10000.csv`
- `random_forest_10000.csv`
- `extra_trees_10000.csv`
- `adaboost_10000.csv`
- `knn_10000.csv`
- `naive_bayes_10000.csv`

Each manifest records source revision, policy path where applicable, exact
seed range, score summary, action counts, and CSV digest. The score comparison
manifest records its input paths, methods, bootstrap count, source revision,
and output digest (`289a2199087d41ebb19754b1e9dc7b425e59ecd9a480282b7eec57ab1cd20bff`). Evaluation was run at root revision
`fa1cd25c94bb76a83eea90a6996487bb7da8e00d` with the AutoML revision above.

```sh
cargo run -- benchmark baseline --agent random --n-games 10000 --seed 94024 \
  --output reports/action-frequency/disjoint-seeds/random_10000.csv
cargo run -- benchmark baseline --agent heuristic --n-games 10000 --seed 94024 \
  --output reports/action-frequency/disjoint-seeds/heuristic_10000.csv
cargo run -- benchmark run --model reports/candidate_classifier_pilot/2026-09-27/random_forest.policy.json \
  --n-games 10000 --seed 94024 --output reports/action-frequency/disjoint-seeds/random_forest_10000.csv
```

The other candidate runs use the corresponding serialized model under
`reports/candidate_classifier_pilot/2026-09-27/`, with the same `--n-games
10000 --seed 94024` settings. The comparison command is retained in the
manifest's input list; it compares all seven CSVs with `--seed 94030`.
