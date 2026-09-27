# Independent 50-game rollout-label collection

This bounded collection adds a same-size, non-overlapping game-seed corpus for
training-corpus sensitivity diagnostics. It is not a scale-quality study and
does not authorize the 20,000-game corpus.

## Protocol and envelope

- Maximum wall time: 60 minutes.
- Global seed 90702; game seeds 90702–90751.
- 100 rollout evaluations per valid action, two Rayon threads, one-game checkpoints.
- Labels are the valid action with the highest rollout mean final score; spawn-4 probability is 0.1.
- Canonical input remains the 16 board cells plus current score; the data header has 17 state values and an action target.
- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.

Reproduce with:

```sh
cargo run -- data-collector collect --n-games 50 --seed 90702 --rollouts 100 \
  --threads 2 --checkpoint-every 1 \
  --output reports/collection_pilots/2026-09-27-50-game-independent/training.csv
python3 scripts/verify_independent_50_game_collection.py \
  --run-dir reports/collection_pilots/2026-09-27-50-game-independent
```

## Observed collection

The run completed in 1,977.90 seconds (39.56 seconds/game), producing 6,188
rows and 2,179,500 rollout evaluations. Rows per game ranged from 51 to 251
(mean 123.76, sample SD 43.45). The run stayed within the one-hour envelope.
An independent verifier confirms 50 sequential game chunks, totals, seed
range, row indices, row/game alignment, the 17-value state/action schema, and
the data, metadata, manifest, and checkpoint hashes. See `verification.json`.

The outputs are `training.csv`, `training.metadata.csv`,
`training.manifest.json`, and `training.checkpoint/`. This corpus is
independent by game-seed range from the earlier 20-game seeds 90627–90646 and
50-game seeds 90652–90701; it shares the same simulator, host, and labeling
protocol, so it is not independent implementation or machine replication.
