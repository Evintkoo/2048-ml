# 50-game corpus policy-score diagnostic

This report evaluates the five policies fitted on the 50-game rollout corpus and compares each with its
earlier 20-game-trained fit. It is an exploratory training-corpus sensitivity diagnostic. Each candidate has
one fit per corpus, and the corpus sizes differ (20 versus 50 games); the paired seed set supports precise
game-seed comparisons for these fixed policies but cannot estimate the distribution over future training
corpora or establish a general model winner.

## Protocol

- Training corpora: 20-game seeds 90627–90646 and independent 50-game seeds 90652–90701.
- Evaluation: 10,000 common seeds, 104024–114023, disjoint from both training corpora and from the earlier
  10,000-game policy evaluation on seeds 94024–104023.
- Agents: the five verified AutoML candidates plus the random and heuristic baselines.
- Score comparison: seed-matched exact sign tests, 5,000-replicate paired bootstrap intervals, paired Cohen's
  dz, and Holm correction across the 21 comparisons in the seven-agent matrix. The sign test ignores ties and
  is not Wilcoxon. CSV p-values are printed to eight decimal places; displayed zeros reflect that output
  precision.
- Corpus-fit comparison: five separate matched old-fit versus new-fit comparisons, each using 5,000 paired
  bootstrap replicates and Cohen's dz. These five separate CLI outputs do not apply one Holm correction across
  the family of five; they are descriptive sensitivity checks only.
- Evaluation source revision: `ee43aa8a2a4c3e12d43d9fffb7c7892e58110ccb`; AutoML submodule:
  `82d848323eed5e2af86d046d529916c448f2442c`.
- The final heuristic, KNN, and seven-agent comparison runs used expanded time caps after shorter initial
  caps proved insufficient. Incomplete attempts were overwritten; only complete outputs with manifests are
  retained. The final policy-run caps were 120 seconds except KNN at 600 seconds; heuristic and comparison
  each had a 600-second cap. Each earlier 20-game policy run had a 180-second cap.

Reproduce an evaluation with:

```sh
cargo run -- benchmark run \
  --model reports/candidate_classifier_pilot/2026-09-27-50-game/models/random_forest.policy.json \
  --n-games 10000 --seed 104024 \
  --output reports/action-frequency/50-game-disjoint-seeds/random_forest_10000.csv
```

For other policies, substitute the model and output names. The earlier-fit runs use models under
`reports/candidate_classifier_pilot/2026-09-27/`; the two baseline commands use
`benchmark baseline --agent random|heuristic`. The seven-agent comparison command is:

```sh
cargo run -- benchmark compare --seed 104030 \
  --output reports/action-frequency/50-game-disjoint-seeds/comparison.csv \
  reports/action-frequency/50-game-disjoint-seeds/random_10000.csv \
  reports/action-frequency/50-game-disjoint-seeds/heuristic_10000.csv \
  reports/action-frequency/50-game-disjoint-seeds/random_forest_10000.csv \
  reports/action-frequency/50-game-disjoint-seeds/extra_trees_10000.csv \
  reports/action-frequency/50-game-disjoint-seeds/adaboost_10000.csv \
  reports/action-frequency/50-game-disjoint-seeds/knn_10000.csv \
  reports/action-frequency/50-game-disjoint-seeds/naive_bayes_10000.csv
```

## Fixed 50-game-fit score distributions

| Agent | Mean | Median | SD | Mean-score 95% CI |
|---|---:|---:|---:|---:|
| Heuristic baseline | 8,047.04 | 7,132 | 3,509.64 | [7,976.93, 8,115.64] |
| Random baseline | 1,097.38 | 1,060 | 534.52 | [1,086.84, 1,107.84] |
| KNN | 903.67 | 808 | 439.18 | [895.05, 912.52] |
| RandomForest | 808.82 | 692 | 468.24 | [799.46, 817.66] |
| NaiveBayes | 773.10 | 664 | 496.21 | [763.60, 783.19] |
| AdaBoost | 752.29 | 656 | 407.63 | [744.57, 760.33] |
| ExtraTrees | 711.35 | 632 | 405.15 | [703.28, 719.38] |

For these fixed fits and evaluation seeds, KNN has the highest mean among the five learned policies, but all
five means are below the random baseline and substantially below the heuristic baseline. The 21 paired
comparisons, intervals, and effects are in `comparison.csv`. This does not establish a winner across training
corpora or a general AutoML result.

## Paired corpus-fit sensitivity

Each cell below is new 50-game fit minus earlier 20-game fit on the same evaluation seeds. The interval is
the paired bootstrap 95% CI in the same direction; Cohen's dz is also oriented as 50-game minus 20-game.

| Candidate | 20-game mean | 50-game mean | Mean difference | Paired 95% CI | Cohen's dz |
|---|---:|---:|---:|---:|---:|
| RandomForest | 851.85 | 808.82 | -43.03 | [-55.54, -30.84] | -0.0666 |
| ExtraTrees | 858.31 | 711.35 | -146.95 | [-158.82, -134.44] | -0.2372 |
| AdaBoost | 769.30 | 752.29 | -17.01 | [-28.17, -5.60] | -0.0296 |
| KNN | 871.64 | 903.67 | 32.03 | [20.03, 44.56] | 0.0523 |
| NaiveBayes | 900.49 | 773.10 | -127.39 | [-141.49, -113.13] | -0.1773 |

The sign and size of the corpus-fit differences vary by candidate, while four of five newer fits scored lower
on this seed set. Because data quantity also changed, there is one fit per corpus, and all training labels
come from the same simulator and host, this is not an isolated estimate of training-seed variation, a
confirmatory ranking, or a policy-quality guarantee.

## Artifacts and integrity

Each score CSV has a run manifest with seed range, score summary, source revision, and digest. The seven-agent
comparison has 21 rows and a comparison manifest. The `20game_*_10000.csv` files evaluate the earlier fits on
the same seeds; `corpus-pairwise/` retains five individual old/new paired comparisons and manifests. Integrity
verification confirmed all 12 score files have exactly 10,000 ordered seeds (104024–114023), each digest
matches its manifest, the seven-agent matrix contains 21 matched comparisons, and all five paired corpus
comparisons use equal seed sets.
