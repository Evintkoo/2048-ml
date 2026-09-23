# Plan 01 — Key Findings: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for key findings.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Status: PENDING.** This file is a pipeline spec: input = `evaluation_v1.parquet` + `statistical_tests.rs` outputs; output = IMRD §3 tables. No TBD rows; honesty = pipeline produces TBD until run.

## Input → Output Pipeline

```
benchmark-framework (10k games/model, seed 42, ScoreMetrics parquet)
  → statistical_tests.rs: mann_whitney, bootstrap_ci(10000 resamples), cohens_d, kruskal_wallis
  → ranking_analysis.py: ranked bar + gate table
  → IMRD §3 Results + this Findings doc
```

## Checklist (All Must Pass to Claim "Findings")

- [ ] `polars 0.46` schema validated (`game_id, score, model, seed`)
- [ ] `GroupKFold(game_id)` used in training (not TimeSeries)
- [ ] Bootstrap 95% CI on mean diff **excludes 0** for winner vs heuristic (~512) and vs runner-up
- [ ] Mann-Whitney U **p<0.05 Bonferroni** (k = #pairwise, ~21 for 7 models)
- [ ] Cohen's **d** reported (not just p; no fixed cutoff)
- [ ] Seed 42 primary; secondary seeds evaluated for sensitivity
- [ ] 27-dim ablation Δ reported with same gates
- [ ] No 8×8/ensemble/RL claim in findings

## Findings-Critical Tables (Shells, Not Fake Rows)

Findings narrative is generated; tables live in `01-IMRD/03-results.md` §3. Do not duplicate placeholder rows here.

## Relation to Hypotheses

F1–F3→RQ1/RQ2 framework validation, H1→2048 baseline comparison, H2→application ranking, H3→tuning comparison, and feature ablation→exploratory analysis. Each outcome is reported with its appropriate evidence. A null application result is a finding, not a failure.

## Implementation Record

- Findings are only a reporting protocol. No results are populated; referenced parquet/statistics artifacts are not available. Checklist items remain pending until evidence artifacts are produced.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/03-Findings/01-key-findings.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
