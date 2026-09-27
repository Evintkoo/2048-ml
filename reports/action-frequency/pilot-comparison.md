# Exploratory paired score comparison for the fitted pilot policy

> **Superseded for held-out interpretation.** The original score evaluation used
> seeds 84024–94023, which overlap 20 rollout-training seeds (90627–90646).
> Its per-game files and comparison are retained as historical artifacts, but
> they are not a training-disjoint evaluation. See the corrected seven-agent
> evaluation on seeds 94024–104023 in
> [`disjoint-seeds/README.md`](disjoint-seeds/README.md).

This analysis uses the existing 10,000-game score CSVs for random play, the
heuristic baseline, and one fitted RandomForest pilot policy. It documents an
exploratory same-seed comparison; it is not a predeclared five-candidate
model-selection study. The policy was fitted from a 20-game rollout-labeled
pilot corpus (17 development games, 2,056 fit rows).

## Inputs and protocol

All three evaluation CSVs contain the same game seeds, 84024–94023 inclusive,
with 10,000 rows per file. The comparison command checked seed-set equality,
aligned rows by seed, applied the two-sided exact sign test, and used Holm
adjustment for the three pairwise tests. It resampled matched score differences
for 95% percentile bootstrap intervals and reports paired Cohen's dz. Bootstrap
seed: 84030; the CLI used 5,000 replicates. The analysis command was:

```sh
cargo run -- benchmark compare reports/action-frequency/random_10000.csv \
  reports/action-frequency/heuristic_10000.csv \
  reports/action-frequency/random_forest_pilot_10000.csv \
  --seed 84030 --output reports/action-frequency/pilot-comparison.csv
```

The fitted policy artifact is
`reports/collection_pilots/2026-09-27-20-game/random_forest.policy.json`; its
manifest records 100 trees, depth 6, root training revision
`e0d92589aba4f6f88ef5bb3346a8b177ebb67454`, and AutoML
`82d848323eed5e2af86d046d529916c448f2442c`. The input run manifests retain
per-game results, seed ranges, configurations, and CSV hashes. The comparison
CSV SHA-256 is
`db27a98f31635d78b7c5c44347bb9b004212b276f1ccd84b39385423c7df4fd4`; its
manifest records source revision `4c0f4861d2d0a3f1138684ad945af5d3d6568f7c`.

## Descriptive results

Differences below are first input mean minus second input mean. The CLI prints
p-values to eight decimal places; `0.00000000` means below its display
resolution, not an exact zero. The effect-size column is paired Cohen's dz.

| First | Second | Means | Difference | Test | Holm-adjusted p (printed) | Paired mean-difference 95% interval | Cohen's dz |
|---|---|---:|---:|---|---:|---:|---:|
| Random | Heuristic | 1,094.124 vs 8,056.232 | -6,962.108 | Paired exact sign | 0.00000000 | [-7,031.590, -6,891.331] | -1.979 |
| Random | Fitted pilot | 1,094.124 vs 866.153 | 227.971 | Paired exact sign | 0.00000000 | [214.631, 241.454] | 0.330 |
| Heuristic | Fitted pilot | 8,056.232 vs 866.153 | 7,190.079 | Paired exact sign | 0.00000000 | [7,122.048, 7,260.022] | 2.048 |

## Interpretation boundary

The sign tests are paired by the shared seed set and ignore tied score outcomes.
The intervals resample paired differences; Cohen's dz standardizes those same
differences. These methods align with the matched-seed design but do not remove
the exploratory limitations: the pilot policy comes from a small training
corpus and was not selected against these baselines under a declared
five-candidate protocol. Treat this as descriptive application-case evidence,
not a confirmatory result or evidence of general AutoML superiority. A
confirmatory comparison still needs a predeclared model-selection and
held-out protocol, sample-size/precision rationale, and evaluation plan.
