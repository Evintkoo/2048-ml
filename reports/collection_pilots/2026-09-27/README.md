# Rollout collection pilot, 2026-09-27

This two-game run is a throughput smoke, not a stable performance estimate or a
training result. The collector resumed from its checkpoint after the process
was stopped with one completed game, then completed the second game.

- Seed: 90627; game seeds: 90627 and 90628.
- 100 rollout evaluations per valid action; two Rayon threads; checkpoint
  interval: one game.
- AutoML commit: `82d848323eed5e2af86d046d529916c448f2442c`.
- Root source revision: `05066988b5683fa2324b6ee3f0cfd8302ce61934`.
- 285 states/rows, 97,300 rollout evaluations, 82.23 seconds.
- CSV and metadata hashes and remaining provenance are in `manifest.json`.

The observed rate was 3.47 rows/second and 142.5 rows/game. Linear
extrapolation gives roughly 228 hours for 20,000 games (`20,000 * 82.23 / 2 /
3600`). Two games are too few to characterize game-length or throughput
variation. The prior 103-hour estimate came from an earlier two-game smoke and
is also not a reliable forecast. A larger pilot and a declared resource budget
are required before a plan-scale run.
