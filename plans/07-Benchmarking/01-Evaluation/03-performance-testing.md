# Performance Testing — Efficiency Metadata (No Gate)

## 1. Purpose

Report efficiency as **metadata only** — no pass/fail gates. The ranking metric is mean score, not speed.

## 2. What We Report

For each 10k-game benchmark run, log:

| Metric | How | Purpose |
|--------|-----|---------|
| `games/sec` | `n_games / wall_time` | Throughput metadata |
| `ms/move` | `total_move_time / total_moves` | Latency metadata |
| `feature_compute_ms` | feature-engineering time per move | Bottleneck identification |

> Feature-compute (11 derived features per state) is the typical bottleneck, not model inference.

No Thread-Group / Batch-Runner / CI regression harness — deleted as over-engineering for <10k-game research runs. No memory/CPU thresholds as gates.
