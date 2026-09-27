# Five-candidate classifier-label diagnostic: independent 50-game corpus

This diagnostic evaluates action-label prediction on a second 50-game corpus
with non-overlapping game seeds 90702–90751. It uses the first 40 chronological
game groups for five-fold grouped cross-validation and fitting, then holds out
the final 10 groups as a 1,137-row classifier-label test. The data is a second
same-size training corpus; it shares the simulator, host, and labeling
protocol with the earlier corpus.

## Protocol

- Input collection: [`reports/collection_pilots/2026-09-27-50-game-independent/`](../../../collection_pilots/2026-09-27-50-game-independent/README.md).
- Canonical state: 17 values (16 board cells plus current score); target is rollout-mean-final-score argmax action.
- Models: RandomForest, ExtraTrees, AdaBoost, KNN, NaiveBayes using AutoML commit `82d848323eed5e2af86d046d529916c448f2442c`.
- Global fit seed: 90702; five grouped folds; development fraction 0.8; one fit per candidate.
- Training source revision: `0f8df9c9def2cd9fb659ae1a3d09df0c17b5f7f5`.

Reproduce a candidate fit by substituting the model and output path:

```sh
target/debug/game2048-ml train \
  --data reports/collection_pilots/2026-09-27-50-game-independent/training.csv \
  --metadata reports/collection_pilots/2026-09-27-50-game-independent/training.metadata.csv \
  --model random_forest --cv-folds 5 --development-fraction 0.8 --seed 90702 \
  --output reports/candidate_classifier_pilot/2026-09-27-independent-50-game/models/random_forest.policy.json
```

Verify all five manifests, input/model/prediction hashes, game-group separation,
and recomputed holdout metrics:

```sh
python3 scripts/verify_50_game_classifier_diagnostic.py \
  --report-dir reports/candidate_classifier_pilot/2026-09-27-independent-50-game \
  --data reports/collection_pilots/2026-09-27-50-game-independent/training.csv \
  --metadata reports/collection_pilots/2026-09-27-50-game-independent/training.metadata.csv \
  --seed 90702 --development-fraction 0.8
```

## Results

| Candidate | Grouped-CV accuracy, mean ± SD | Holdout accuracy | Holdout macro-F1 |
|---|---:|---:|---:|
| RandomForest | 0.2972 ± 0.0167 | 0.3008 | 0.3007 |
| ExtraTrees | 0.2782 ± 0.0160 | 0.2770 | 0.2394 |
| AdaBoost | 0.2849 ± 0.0174 | 0.2885 | 0.2878 |
| KNN | 0.2464 ± 0.0145 | 0.2542 | 0.2475 |
| NaiveBayes | 0.2597 ± 0.0168 | 0.2639 | 0.1984 |

RandomForest leads holdout accuracy and macro-F1 on this one split. This is
classifier-label evidence only; it is not policy-score evidence or a
confirmatory model ranking. Fitted policies and a paired game-score comparison
against the earlier 50-game fits are recorded in
[`the policy-score report`](../../../action-frequency/independent-50-game-seeds-2026-09-27/README.md).
