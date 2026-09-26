# Plan 03 — Performance Testing: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Whole-run wall time is recorded; per-move and feature timing have not been measured.

**Goal:** State the current implementation and evidence boundary for performance testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats whole-run timing as implemented and detailed profiling as pending.** Benchmark manifests record game count and elapsed wall time, allowing games/second. The root does not measure per-move latency or feature extraction separately, and no trained-model performance run is documented.

## 1. Purpose

Report efficiency as **metadata only** — no pass/fail gates. The ranking metric is mean score, not speed.

## 2. What We Report

For each benchmark run, log:

| Metric | How | Purpose |
|--------|-----|---------|
| `games/sec` | `n_games / wall_time` | Throughput metadata |
| `ms/move` | `total_move_time / total_moves` (future instrumentation) | Latency metadata |
| `feature_compute_ms` | feature-engineering time per move (future instrumentation) | Bottleneck identification |

> No bottleneck is assumed; attribute time only after per-stage profiling is implemented and measured.

No Thread-Group / Batch-Runner / CI regression harness is implemented. No memory/CPU thresholds are used as gates.

## Implementation Record

- Benchmark manifests record game count and wall time, supporting games/second. Per-move timing and feature-computation timing are not instrumented. No trained-model efficiency run has been completed.

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

- Instrument model inference and feature extraction separately if those measures are needed; record hardware, warmup, workload, and timing distribution alongside raw game results.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
