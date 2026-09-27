# Independent rollout collection pilot: five games

This bounded repeat checks throughput across a contiguous seed range after the
20-game pilot. It is a resource diagnostic only; it does not authorize the
20,000-game corpus or establish policy quality.

## Protocol

- Command: `cargo run --quiet -- data-collector collect --n-games 5 --seed 90647 --rollouts 100 --threads 2 --checkpoint-every 1 --output reports/collection_pilots/2026-09-27-5-game-repeat/training.csv`
- Game seeds: 90647–90651, continuing after the earlier pilot's 90627–90646.
- Labels: argmax of 100 rollout mean final scores for each valid action; spawn-4 probability 0.1.
- Checkpoint: after each game; all five chunks completed in one invocation, without resume.
- Root source revision: `e72c9dbd6575820ea696dd432009f57757a315f8`.
- AutoML submodule: `82d848323eed5e2af86d046d529916c448f2442c`.
- Environment: macOS 26.6.2 arm64, Rust 1.96.1, two Rayon threads.

## Observed output

- 5 games, 541 aligned training/metadata rows, 191,500 rollout evaluations.
- Elapsed collector time: 182.09 seconds (3 minutes 2.09 seconds), or 36.42 seconds/game.
- Rows/game: 120, 124, 92, 127, and 78; range 78–127, mean 108.2, sample SD 21.89.
- Throughput: 2.97 rows/second and 1,051.68 rollout evaluations/second.

Together with the preceding 20-game run, these 25 contiguous games produced
2,988 rows and 1,048,600 rollout evaluations in 1,039.45 collector seconds.
The combined mean is 41.58 seconds/game, corresponding to a rough linear
projection of 230.99 hours for 20,000 games. This combines two seed ranges on
one host and is not a cross-machine interval, runtime guarantee, or declared
compute envelope. The observed variation and small sample do not support a
quality claim. A user-approved resource limit and scale-appropriate protocol
remain prerequisites before full-scale collection.

## Artifacts and integrity

- `training.csv` SHA-256: `355c7d190fe8dc4ef52b578ff6278531090aabca891738b8eb3c786a1da92e76`
- `training.metadata.csv` SHA-256: `f462588528b5a5b91ea6002426c1c768f6c4f82f88edcb6fbca1195fc8a42f4f`
- `training.manifest.json` SHA-256: `9b270b877d623a732f461846c161d9a1f36d9392f5fa59a3b9ccf273a1a77cf2`
- `training.checkpoint/checkpoint.json` SHA-256: `a036e129b0678217b2ca6b6f85e8088807ca8fb7b6eea8d69518b0a562975fac`

The manifest hashes match both CSV artifacts. The checkpoint has five completed
one-game chunks whose row counts sum to 541, and the manifest and checkpoint
agree on game count, rows, rollout evaluations, source revision, and elapsed
time. The collector validated the assembled dataset schema.
