# 2048 action-frequency study

## Protocol

- 10,000 games each for random legal-action selection, the root heuristic, and the fitted AutoML RandomForest policy. All use game seeds `84024` through `94023`, inclusive; each game uses the default 90/10 spawn probability and 1,000-move safety limit.
- CSVs retain the four selected-action counts per game (`up_moves`, `down_moves`, `left_moves`, `right_moves`). Each row's action counts sum to `move_count`.
- Frequency is the pooled count of selected actions divided by the total selected moves. The 95% percentile interval resamples whole games with replacement for 2,000 replicates using bootstrap seed `84026`.
- The root CLI ran in the Cargo development profile on macOS 26.5, arm64, Rust 1.96.1. Runtimes are local smoke measurements, not optimized performance comparisons.
- Baseline runs used root revision `111b77d6008e8644b5684973d0054fc74308d274`. The fitted-policy run used root revision `72e3b83232b54df653e5635d5c41011625707d19` and AutoML commit `82d848323eed5e2af86d046d529916c448f2442c`. Per-run manifests retain source revisions, CSV digests, seeds, timings, score summaries, and frequency summaries.

Commands for the baseline runs:

```bash
cargo run -- benchmark baseline --agent random --n-games 10000 --seed 84024 --output reports/action-frequency/random_10000.csv
cargo run -- benchmark baseline --agent heuristic --n-games 10000 --seed 84024 --output reports/action-frequency/heuristic_10000.csv
```

The fitted policy was evaluated with:

```bash
cargo run -- benchmark run --model reports/collection_pilots/2026-09-27-20-game/random_forest.policy.json --n-games 10000 --seed 84024 --output reports/action-frequency/random_forest_pilot_10000.csv
```

## Results

| Agent | Action | Count | Proportion | 95% whole-game bootstrap CI |
|---|---|---:|---:|---:|
| Random | Up | 295,177 | 0.249527 | [0.248847, 0.250213] |
| Random | Down | 296,302 | 0.250478 | [0.249757, 0.251181] |
| Random | Left | 295,638 | 0.249917 | [0.249265, 0.250623] |
| Random | Right | 295,827 | 0.250077 | [0.249356, 0.250779] |
| Heuristic | Up | 1,368,862 | 0.262476 | [0.262011, 0.262956] |
| Heuristic | Down | 1,258,870 | 0.241385 | [0.240928, 0.241848] |
| Heuristic | Left | 1,345,191 | 0.257937 | [0.257469, 0.258413] |
| Heuristic | Right | 1,242,272 | 0.238202 | [0.237747, 0.238662] |
| Fitted policy | Up | 267,986 | 0.262037 | [0.260725, 0.263326] |
| Fitted policy | Down | 162,491 | 0.158884 | [0.158240, 0.159523] |
| Fitted policy | Left | 236,763 | 0.231507 | [0.230450, 0.232569] |
| Fitted policy | Right | 355,464 | 0.347573 | [0.346386, 0.348728] |

Random is effectively uniform over selected directions at this precision. The heuristic selects Up and Left more often than Down and Right. The fitted pilot policy selects Right most often. These are descriptive action frequencies, not model-quality or framework-superiority findings.

## Raw data and independent verification

Each agent's per-game CSV and run manifest are retained here. The independent summaries are `random_10000.independent-frequency.json`, `heuristic_10000.independent-frequency.json`, and `random_forest_pilot_10000.independent-frequency.json`. The Rust verifier in `examples/recompute_action_frequencies.rs` reads each raw CSV, checks its manifest digest, contiguous game IDs and seeds, and per-game action totals, then recomputes the whole-game bootstrap with the recorded Rust seed. It requires exact equality with the manifest summaries before writing the independent JSON. For example:

```bash
cargo run --example recompute_action_frequencies -- reports/action-frequency/random_10000.csv reports/action-frequency/random_10000.manifest.json reports/action-frequency/random_10000.independent-frequency.json
```

The fitted-policy 10,000-game CSV digest is `381311816bd9de360347d48bd998a94788b0ef67a8355a69558e179a44ee4a23`; it contains 1,022,704 selected moves. Its action profile describes one development policy trained on the small rollout corpus in `reports/collection_pilots/2026-09-27-20-game/README.md`. The 10,000 evaluation seeds are separate from its training game seeds, but held-out classifier diagnostics on the final three chronological corpus games were not reported. Scores and runtimes in the manifests describe these simulation runs only.
