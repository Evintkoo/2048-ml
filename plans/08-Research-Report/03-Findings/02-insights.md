# Plan 02 — Insights: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Insights are proposed analyses; only baseline action-frequency outputs currently provide measured behavioral summaries.

**Goal:** State the current implementation and evidence boundary for insights.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**No model-policy insight is claimed.** Baseline action-frequency reports cover random and heuristic agents; no trained policy or step-level model outcome corpus is available.

> The following are candidate analyses. The proposed Parquet file and general per-step model-log export do not exist.

## Concrete Analyses (Measured, Not Hypothesized)

1. **Action validity** — measure invalid chosen moves only if the benchmark records them. Do not assume invalid moves are attempted, since the policy masks illegal actions.
2. **Board-position patterns** — require a per-move board/action timeline export; current final-score reports cannot support this analysis.
3. **Feature association** — requires aligned state/feature and outcome artifacts plus an analysis plan; association would not establish feature importance or causality.
4. **Action distribution** — measured random and heuristic action-frequency artifacts exist. No trained-policy frequency is available; see `reports/action-frequency/README.md`.
5. **Score distribution** — report implemented summary fields for supplied benchmark inputs. Bimodality tests are not implemented.
6. **Within-game phase analysis** — requires per-move timelines and a declared clustered analysis; not supported by current aggregate files.

## Deleted Generic Content

- Removed "Early Overfitting / Plateau / Bimodal patterns" generic table — replaced by measured analyses above with exact metrics and code pointers (`feature_extraction.rs`, `benchmark_runner.rs`).

## Output

No insights generator exists. Preserve input manifests and analysis code for any future measured insight; describe unexpected patterns without selecting post hoc thresholds.

## Implementation Record

- Baseline action-frequency artifacts are available for random and heuristic agents. Model-specific step logs, board-position analyses, correlations, bimodality tests, and report generation remain unimplemented.

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

- **Model-policy insights remain pending.** Additional analysis needs suitable retained raw artifacts and a declared unit/uncertainty method.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
