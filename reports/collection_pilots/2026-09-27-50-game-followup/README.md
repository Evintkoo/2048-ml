# Bounded rollout collection follow-up: 50 games

This follow-up extends the same-host throughput sample after the 20-game pilot and independent five-game
continuation. It is a resource diagnostic only; it does not establish policy quality, authorize a
20,000-game corpus, or provide independent machine replication.

## Declared envelope and protocol

- Maximum wall time: 60 minutes, including any checkpointed restart.
- Requested games: 50; global seed 90652, producing game seeds 90652–90701.
- Labels: argmax of 100 rollout mean final scores for each valid action; spawn-4 probability 0.1.
- Threads: 2 Rayon threads; checkpoint after every game.
- Initial command: `target/debug/game2048-ml data-collector collect --n-games 50 --seed 90652 --rollouts 100 --threads 2 --checkpoint-every 1 --output reports/collection_pilots/2026-09-27-50-game-followup/training.csv`
- The first process was stopped after 15 fully checkpointed games while correcting the external wall-time monitor. The same configuration resumed with `--resume`; no incomplete game chunk was retained.
- Root source revision: `7ba5e8a812a39d58e07e61191c845cd16636f2c6`.
- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.
- Environment: Darwin kernel 25.6.0, arm64, Rust 1.96.1.

## Observed output

- 50 games, 5,318 aligned training and metadata rows, and 1,877,900 rollout evaluations.
- Collector elapsed time: 1,799.88 seconds (29 minutes 59.88 seconds), or 35.997 seconds/game.
- Per-game rows ranged from 54 to 197 (mean 106.36, sample SD 33.35).
- Throughput: 2.955 rows/second and 1,043.34 rollout evaluations/second.
- The output CSVs passed the collector's schema validation. A separate integrity check confirmed 50 completed
  checkpoint chunks summing to 5,318 rows, contiguous metadata row indices, game IDs 0–49, matching manifest
  and checkpoint totals, and both CSV hashes.

Combining this run with the 20-game and five-game pilots gives 75 games, 8,306 rows, and 2,926,500 rollout
evaluations in 2,839.33 collector seconds. The resulting rough linear projection is 37.86 seconds/game, or
210.32 hours for 20,000 games. This projection is configuration-specific and uncertain; it is not a runtime
guarantee or a scale-appropriate quality result.

## Artifacts and integrity

- `training.csv` SHA-256: `db0c8df0341fdfa998c146d1804fc05b4fe0a812b379ca10fd37e6dbeec5cae7`
- `training.metadata.csv` SHA-256: `dd8436222240f372a5f5f0be3610678061f2868b2489453b8b9f391ba612d0ba`
- `training.manifest.json` SHA-256: `ddfd01e932a79f17bbd4f3455895e3a326b19278af5cd6390e12829eb84fb94a`
- `training.checkpoint/checkpoint.json` SHA-256: `605dd13ec04ae0bb5d045c51d345c77e78ce577e6a1516e76fd94f36584afd04`

The checkpoint directory contains 50 one-game chunks and metadata chunks. The manifest records the resumed
run, seed range, source and submodule revisions, row/evaluation totals, and CSV digests.
