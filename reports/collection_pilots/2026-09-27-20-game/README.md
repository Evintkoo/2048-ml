# Rollout collection pilot: 20 games

This bounded pilot measures rollout-labeling throughput and row-count variation.
It is not a training-quality result and does not authorize the 20,000-game corpus.

## Protocol

- Command: `cargo run --quiet -- data-collector collect --n-games 20 --seed 90627 --rollouts 100 --threads 2 --checkpoint-every 1 --output reports/collection_pilots/2026-09-27-20-game/training.csv`
- Game seeds: 90627–90646, derived as global seed plus game ID.
- Label: argmax of 100 rollout mean final scores for each valid action; spawn-4 probability 0.1.
- Checkpoint: after every game; collection completed without resuming.
- Root source revision: `e0d92589aba4f6f88ef5bb3346a8b177ebb67454`.
- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.
- Environment: macOS 26.0 arm64, Rust 1.96.1, two Rayon threads.

## Observed output

- 20 games, 2,447 aligned training/metadata rows, 857,100 rollout evaluations.
- Elapsed collector time: 857.36 seconds (14 minutes 17.36 seconds), or 42.87 seconds/game on average.
- Rows/game: minimum 62, maximum 207, mean 122.35, sample standard deviation 40.74, median 120.5.
- Throughput: 2.85 rows/second and 1,000.75 rollouts/second over this run.
- Linear projection at the observed mean: 238.16 hours for 20,000 games. This is a planning estimate only; it excludes variation in machine load and is not a runtime guarantee.

The sample is 20 games from one sequential seed range on one host. It is not enough to estimate rare long games or cross-machine variation, and its throughput is not evidence of policy/model quality. Retain an explicit resource limit before launching larger collection.

## Artifacts and integrity

- `training.csv` SHA-256: `4b1063b3b5c318e271e7191c77a3d440a3fa912e9a9150fb62ddf0d18abd21b3`
- `training.metadata.csv` SHA-256: `3951d0d6abb3c569736a138e4aa0e19780fd420d1ddb59f80b573afdc31ad41b`
- `training.manifest.json` SHA-256: `0520d00f21cece840f1a13459239e88334e8f19695500bb6595f10e4e262e585`
- `training.checkpoint/checkpoint.json` retains completed-game row counts and configuration.

The collection command reported schema validation for all 2,447 rows. The manifest contains the source revision, dependency pin, seeds, configuration, elapsed time, and CSV digests.

## AutoML-to-simulator wiring smoke

The same pilot corpus was used for one seeded AutoML RandomForest training and a
separate 20-game saved-policy simulator smoke. This demonstrates the integration
path on a small labeled sample; it is not a held-out policy-quality estimate.

- Training command used `--development-fraction 0.85 --model random_forest --cv-folds 5 --seed 90627`.
- The final 3 chronological games were excluded from fitting. The 17 earlier
  games supplied 2,056 rows to the 17-feature fit; grouped five-fold CV reported
  accuracy `0.2828 ± 0.0285`. AutoML fit completed in `0.657` seconds.
- `random_forest.policy.json` and its manifest record the model, input hashes,
  held-out row groups, seed derivations, selected defaults, and AutoML revision.
- Separate simulator command used 20 games with global seed 91927, yielding
  game seeds 91927–91946. The CSV contains 20 completed games; mean score was
  `902.40`, median `748`, sample SD `456.00`, and bootstrap mean interval
  `[712.60, 1095.20]`. Runtime was `0.22` seconds.
- Simulator CSV SHA-256:
  `2f0244b320d4c3b91ff2bc548a7987eedd24a802944b489e37fdca873c287bbc`.

The CV accuracy and game-score smoke answer different questions and neither is
a case-study result. The separate 20-game simulator seeds are a small development smoke and were not compared with matched baselines.


## Chronological holdout classifier diagnostic

After the training CLI began retaining outer-holdout predictions, the same fixed
protocol was rerun from root revision `31ff9bd`. Games 17–19 (391 rows) remained
excluded from grouped CV and fitting. The model produced accuracy `0.2967`, macro-F1
`0.2914`, macro-precision `0.3044`, and macro-recall `0.2978`. The manifest records
the held-out game IDs, class order, confusion matrix, artifact hashes, model
configuration, and interpretation; the CSV sidecar retains row-level actual and
predicted actions.

These labels are rollout-derived action targets from a 20-game development corpus.
The three-game diagnostic is small and is not a confirmatory estimate of policy
quality, generalization, or framework performance. The separate game-score smoke
uses different seeds and remains wiring evidence only.
