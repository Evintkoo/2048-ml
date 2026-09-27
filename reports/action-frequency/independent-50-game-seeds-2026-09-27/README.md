# Independent 50-game corpus policy-score sensitivity diagnostic

This report compares one fitted policy per verified AutoML candidate from a
second 50-game corpus with random and heuristic baselines. The new corpus uses
game seeds 90702–90751; the earlier corpus uses 90652–90701. Both contain 50
games and use the same labeling protocol and host. The policies were evaluated
on common game seeds 114024–124023, disjoint from both training corpora.

The five new policies were fit once with global seed 90702 on the first 40
chronological game groups; the last 10 groups formed the classifier-label
holdout. The previous policies were fit once with seed 90652 on the first
40 groups of the earlier corpus. These fixed-fit results provide exploratory
corpus sensitivity evidence; they do not estimate variability across repeated
fits, establish a model winner, or provide confirmatory evidence.

## Protocol and reproduction

- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.
- Evaluation: 10,000 games per agent on seeds 114024–124023.
- Seven agents: five fitted policies, random baseline, and heuristic baseline.
- Matrix analysis: matched seed-wise sign tests, 5,000-replicate paired bootstrap
  intervals, Cohen's dz, and Holm adjustment across the 21 comparisons.
- Corpus-fit contrasts: five separate paired comparisons use those same seeds,
  with paired bootstrap intervals and Cohen's dz. They were run separately, so
  their Holm values are not adjusted across the family of five; treat them as
  descriptive sensitivity checks.

Reproduce one new policy run:

```sh
cargo run -- benchmark run \
  --model reports/candidate_classifier_pilot/2026-09-27-independent-50-game/models/random_forest.policy.json \
  --n-games 10000 --seed 114024 \
  --output reports/action-frequency/independent-50-game-seeds-2026-09-27/random_forest-10000.csv
```

Run the remaining candidate policies and baselines on the same seed range,
then reproduce the seven-agent comparison using the command form shown in the
50-game disjoint-seed report. Recreate and verify this artifact set with:

```sh
python3 scripts/verify_independent_50_game_collection.py \
  --run-dir reports/collection_pilots/2026-09-27-50-game-independent
python3 scripts/verify_50_game_classifier_diagnostic.py \
  --report-dir reports/candidate_classifier_pilot/2026-09-27-independent-50-game \
  --data reports/collection_pilots/2026-09-27-50-game-independent/training.csv \
  --metadata reports/collection_pilots/2026-09-27-50-game-independent/training.metadata.csv \
  --seed 90702 --development-fraction 0.8
python3 scripts/verify_independent_50_game_policy_scores.py \
  --report-dir reports/action-frequency/independent-50-game-seeds-2026-09-27 \
  --training-report-dir reports/candidate_classifier_pilot/2026-09-27-independent-50-game \
  --collection-dir reports/collection_pilots/2026-09-27-50-game-independent \
  --previous-training-report-dir reports/candidate_classifier_pilot/2026-09-27-50-game \
  --previous-collection-dir reports/collection_pilots/2026-09-27-50-game-followup
```

## Policy score distributions

| Agent | Mean | Median | Sample SD | Mean-score 95% CI |
|---|---:|---:|---:|---:|
| Heuristic baseline | 8,030.51 | 7,136 | 3,456.11 | [7,961.13, 8,098.01] |
| Random baseline | 1,087.52 | 1,044 | 528.34 | [1,077.52, 1,097.60] |
| KNN | 918.63 | 824 | 449.31 | [909.89, 927.33] |
| RandomForest | 858.29 | 740 | 424.09 | [850.27, 866.13] |
| ExtraTrees | 814.12 | 728 | 391.66 | [806.41, 821.57] |
| NaiveBayes | 757.87 | 684 | 408.86 | [749.82, 765.98] |
| AdaBoost | 710.54 | 628 | 381.62 | [702.96, 718.04] |

All five learned policy means are below the random baseline mean on this
evaluation range, and far below the heuristic mean. This is application
evidence about these five fixed fits only, not a general AutoML result.

## Paired sensitivity between same-size corpus fits

Each row is the new independent-corpus policy minus the previous-corpus policy
for the same candidate on the same 10,000 evaluation seeds. The interval is a
5,000-replicate paired bootstrap 95% interval; effect size is paired Cohen's
dz.

| Candidate | Previous-corpus mean | New-corpus mean | Difference | Paired 95% CI | Cohen's dz |
|---|---:|---:|---:|---:|---:|
| RandomForest | 819.56 | 858.29 | +38.73 | [26.63, 51.23] | 0.0614 |
| ExtraTrees | 701.30 | 814.12 | +112.83 | [101.78, 123.82] | 0.2014 |
| AdaBoost | 764.70 | 710.54 | −54.16 | [−65.27, −43.17] | −0.0969 |
| KNN | 911.47 | 918.63 | +7.16 | [−5.00, 19.53] | 0.0114 |
| NaiveBayes | 775.46 | 757.87 | −17.58 | [−30.53, −4.94] | −0.0275 |

The direction differs among candidates. Because there is one fit per corpus,
these contrasts combine corpus sampling and fit-seed effects; they do not
estimate repeated-fit or future-corpus distributions. Each candidate's
comparison was run separately; no Holm correction was applied across these
five sensitivity contrasts. The score CSVs and manifests, paired comparison
CSVs/manifests, policies, and source-corpus hashes are checked by
`verification.json`.
