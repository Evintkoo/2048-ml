# Exploratory paired score comparison for the fitted pilot policy

This analysis uses the existing 10,000-game score CSVs for random play, the
heuristic baseline, and one fitted RandomForest pilot policy. It is retained to
document an exploratory, same-seed comparison; it is not a predeclared
five-candidate model-selection study. The policy was fitted from a 20-game
rollout-labeled pilot corpus (17 development games, 2,056 fit rows).

## Inputs and protocol

All three evaluation CSVs contain the same game seeds, 84024–94023 inclusive,
with 10,000 rows per file. The comparison command checked those seed sets,
paired rows by seed, applied the repository's two-sided exact sign test, and
used Holm adjustment for the three pairwise tests. Bootstrap seed: 84030; the
CLI used 5,000 replicates. The analysis command was:

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
`89430d7ce2b7a5e60ea4c7250b9b8da8302e6614815610ecc4bbcb6fdeb224d7`; its
manifest records source revision `ee7614124751c7f50ae03f1217310f86e414c976`.

## Descriptive results

Differences below are first input mean minus second input mean. The CLI prints
p-values to eight decimal places; `0.00000000` means below its display
resolution, not an exact zero.

| First | Second | Means | Difference | Test | Holm-adjusted p (printed) | Mean-difference interval (printed) | Cohen's d |
|---|---|---:|---:|---|---:|---:|---:|
| Random | Heuristic | 1,094.124 vs 8,056.232 | -6,962.108 | Paired exact sign | 0.00000000 | [-7,030.007, -6,893.108] | -2.792 |
| Random | Fitted pilot | 1,094.124 vs 866.153 | 227.971 | Paired exact sign | 0.00000000 | [214.090, 241.597] | 0.464 |
| Heuristic | Fitted pilot | 8,056.232 vs 866.153 | 7,190.079 | Paired exact sign | 0.00000000 | [7,121.211, 7,259.205] | 2.896 |

## Interpretation boundary

The sign tests are paired by the shared seed set and ignore tied score outcomes.
The CLI's mean-difference interval and Cohen's d helper use independent-sample
formulas, even when the sign test is paired; these intervals/effect sizes are
not paired-seed uncertainty estimates. Treat the comparison as exploratory
descriptive evidence and do not use its printed intervals to claim a matched
effect. The pilot policy comes from a small training corpus and was not selected
against these baselines under a declared five-model protocol. These game scores
are application-case evidence only, not evidence of general AutoML
superiority. A confirmatory comparison needs a predeclared model-selection and
held-out protocol with methods aligned to its paired design.
