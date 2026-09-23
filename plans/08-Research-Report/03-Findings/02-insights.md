# Plan 02 — Insights: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for insights.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

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

## Implementation Record

- No step-level benchmark artifact, action/board timeline export, insights generator, or empirical tail analysis is implemented. Listed patterns are analyses to run, not findings.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/03-Findings/02-insights.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/03-Findings/02-insights.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
