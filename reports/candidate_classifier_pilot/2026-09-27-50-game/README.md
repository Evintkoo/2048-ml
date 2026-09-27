# Five-candidate classifier-label diagnostic: 50-game corpus

This is a single-fit development diagnostic on the 50-game rollout-labeled corpus collected on seeds
90652–90701. It checks classifier-label generalization across a chronological game-group split. It does not
measure policy game scores, establish a model winner, or provide confirmatory evidence.

## Protocol

- Input: `reports/collection_pilots/2026-09-27-50-game-followup/training.csv` and its metadata sidecar.
- Input SHA-256: training CSV `db0c8df0341fdfa998c146d1804fc05b4fe0a812b379ca10fd37e6dbeec5cae7`; metadata CSV `dd8436222240f372a5f5f0be3610678061f2868b2489453b8b9f391ba612d0ba`.
- State: canonical 17-value feature vector; target: rollout-mean-final-score argmax action.
- Split: 40 earlier game groups (seeds 90652–90691, 4,415 rows) for grouped five-fold CV and fitting; final 10 groups (seeds 90692–90701, 903 rows) held out chronologically.
- One fit per verified candidate, using AutoML commit `82d848323eed5e2af86d046d529916c448f2442c`, seed 90652, five grouped folds, and development fraction 0.8. Hyperparameter optimization was not run. The root integration revision was `c7400ef535dd74eb62c263d9a2f7a9b4cabcb624`.
- Each fit invocation had a 175-second timeout; all five completed in less than one minute in total.

Reproduce a fit with the retained CSVs and the recorded options:

```sh
target/debug/game2048-ml train \
  --data reports/collection_pilots/2026-09-27-50-game-followup/training.csv \
  --metadata reports/collection_pilots/2026-09-27-50-game-followup/training.metadata.csv \
  --model random_forest --cv-folds 5 --development-fraction 0.8 --seed 90652 \
  --output reports/candidate_classifier_pilot/2026-09-27-50-game/models/random_forest.policy.json
```

Replace the model and output basename for `extra_trees`, `adaboost`, `knn`, and `naive_bayes` to reproduce
the other fits. The CLI manifests retain grouped-fold metrics, the held-out predictions and confusion
matrices, model and input hashes, configuration, and source provenance.

## Results

| Candidate | Grouped-CV accuracy, mean ± SD | Holdout accuracy | Holdout macro-F1 |
|---|---:|---:|---:|
| RandomForest | 0.3154 ± 0.0146 | 0.3001 | 0.2873 |
| ExtraTrees | 0.2959 ± 0.0287 | 0.2780 | 0.2070 |
| AdaBoost | 0.3082 ± 0.0088 | 0.2835 | 0.2785 |
| KNN | 0.2599 ± 0.0190 | 0.2602 | 0.2537 |
| NaiveBayes | 0.2653 ± 0.0187 | 0.2381 | 0.1961 |

RandomForest leads holdout accuracy and AdaBoost leads holdout macro-F1 in this one split. This disagreement,
one fit per candidate, the small rollout corpus, and shared simulator/host limit interpretation. The outcomes
remain classifier-label diagnostics, not evidence about downstream policy score or a confirmatory ranking.

## Verification and artifacts

`python3 scripts/verify_50_game_classifier_diagnostic.py` recomputes holdout accuracy, macro-F1, and confusion
matrices from retained predictions, then checks game-group separation, row identities, input/model/prediction
hashes, fit settings, and manifest results. It writes `verification.json`; the recorded run passed for all five
candidates.

Per-candidate models, manifests, and held-out prediction CSVs are in `models/`. The source data, checkpoint,
and collection manifest are retained in the linked collection-pilot report.
