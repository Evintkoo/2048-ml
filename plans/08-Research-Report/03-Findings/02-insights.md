# Insights — Pattern Analysis Spec (35 Lines, Concrete Analyses)

> **No generic Early Overfitting table.** Each analysis below is executable on `evaluation_v1.parquet` + per-step logs (`action, valid_move, score_delta`).

## Concrete Analyses (Measured, Not Hypothesized)

1. **Invalid-move rate** — `invalid_rate = mean(!valid_move)` per model. Hypothesis to measure: ~60–80% of random moves are invalid; learned policy should be <5%. Flag if >10%.
2. **Corner-stuck frequency** — `corner_stuck = fraction of games where max_tile in corner for >80% of moves` (encoded via `max_tile_log` + `grid_0..15`). Measure correlation with score.
3. **Feature–score monotonicity** — Spearman ρ between each 27-dim group (`empty_count`, `mono_*`, `smooth_*`, `merge_*`) and per-game score. Report top-3 |ρ| with bootstrap CI.
4. **Move distribution** — `P(action)` over `0..3`. Hypothesis to *measure* (not claim): corner strategy predicts ~60–80% mass on one direction (e.g., Left). Report actual distribution with CI.
5. **Score tail & bimodality** — Histogram + Hartigan dip test for bimodal low/high scores; percentile table (50/90/95/99) — not mean-only.
6. **Early-game vs late-game** — Split games at median `moves`; compare feature correlations and invalid rates across halves.

## Deleted Generic Content

- Removed "Early Overfitting / Plateau / Bimodal patterns" generic table — replaced by measured analyses above with exact metrics and code pointers (`feature_extraction.rs`, `benchmark_runner.rs`).

## Output

`analysis/insights.md` auto-generated; each insight line cites `ScoreMetrics` aggregation, not prose. Unexpected findings (e.g., Up dominates not Left, or empty_count negatively correlated) flagged for Discussion §5.
