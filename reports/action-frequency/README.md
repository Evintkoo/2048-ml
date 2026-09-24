# 2048 action-frequency baseline study

## Protocol

- 10,000 games per baseline agent: random legal-action selection and the root heuristic policy.
- Both agents use game seeds `84024` through `94023`, inclusive; each game uses the default 90/10 spawn probability and 1,000-move safety limit.
- Frequency is the pooled count of selected actions divided by the total selected moves.
- 95% percentile intervals use 2,000 bootstrap replicates, resampling whole games with replacement (bootstrap seed `84026`). This preserves within-game dependence between successive moves.
- Runs were sequential through the root CLI in the Cargo development profile on macOS 26.5, arm64, with Rust 1.96.1. Times are local smoke measurements, not optimized performance comparisons.
- Root checkout revision was `ca15cebcdf4d307126a63d4ab19e416606f50564`; `src/main.rs` and `src/evaluation.rs` had local modifications for this run. Their SHA-256 values were `bfb9eb95a1e4302b73609c2b2a67ce522bac087c853ab97723c08f50c50b5e8f` and `40f66622b3f3075a040bb81d59f7c1ba6daf9c8844e30eed6ac556606e5a3ca9`. `Cargo.lock` SHA-256: `84c2c9910b8baff1c05523fe1f298ff12335a886e37b7c8929ac35e75d888681`.
- Exact commands:

  ```bash
  cargo run -- benchmark baseline --agent random --n-games 10000 --seed 84024 --output reports/action-frequency/random_10000.csv
  cargo run -- benchmark baseline --agent heuristic --n-games 10000 --seed 84024 --output reports/action-frequency/heuristic_10000.csv
  ```

The raw per-game CSVs and generated JSON manifests are retained beside this report. Each manifest records the CSV SHA-256, seed range, timing, score summary, pooled action counts, proportions, and bootstrap intervals.

## Results

| Agent | Action | Count | Proportion | 95% game-cluster bootstrap CI |
|---|---|---:|---:|---:|
| Random | Up | 295,177 | 0.249527 | [0.248847, 0.250213] |
| Random | Down | 296,302 | 0.250478 | [0.249757, 0.251181] |
| Random | Left | 295,638 | 0.249917 | [0.249265, 0.250623] |
| Random | Right | 295,827 | 0.250077 | [0.249356, 0.250779] |
| Heuristic | Up | 1,368,862 | 0.262476 | [0.262011, 0.262956] |
| Heuristic | Down | 1,258,870 | 0.241385 | [0.240928, 0.241848] |
| Heuristic | Left | 1,345,191 | 0.257937 | [0.257469, 0.258413] |
| Heuristic | Right | 1,242,272 | 0.238202 | [0.237747, 0.238662] |

The random agent is effectively uniform over selected directions at this precision. The heuristic selects Up and Left more often than Down and Right. These are descriptive policy frequencies, not a model-training prior or an AutoML result. No trained model was available for the same frequency study.

| Agent | Games | Selected moves | Mean score | Median score | Runtime |
|---|---:|---:|---:|---:|---:|
| Random | 10,000 | 1,182,944 | 1,094.12 | 1,050 | 27.89 s |
| Heuristic | 10,000 | 5,215,195 | 8,056.23 | 7,136 | 201.99 s |

Scores and runtimes are included to describe the simulation runs only. They do not validate general AutoML quality or establish a framework performance comparison.
