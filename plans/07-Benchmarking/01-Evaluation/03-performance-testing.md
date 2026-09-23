# Plan 03 — Performance Testing: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for performance testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

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

## Implementation Record

- Benchmark manifests record game count and wall time, supporting games/second. Per-move timing and feature-computation timing are not instrumented. No 10k model efficiency run has been completed.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/01-Evaluation/03-performance-testing.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
