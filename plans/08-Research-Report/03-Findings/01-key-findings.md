# Plan 01 — Key Findings: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Reporting checklist is a template; no research findings or complete result artifacts exist.

**Goal:** State the current implementation and evidence boundary for key findings.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is a reporting protocol only.** Framework validation and trained-policy evaluation remain incomplete. Checklist items are proposed evidence requirements, not acceptance gates already met.

> The result pipeline described below is proposed. Current benchmark artifacts are CSV plus JSON manifests, and statistics live in `src/evaluation.rs`.

## Input → Output Pipeline

```
declared benchmark CSVs + JSON manifests
  → evaluation/comparison CLI summaries and pairwise helpers
  → IMRD §3 Results + this Findings doc
```

## Checklist (All Must Pass to Claim "Findings")

- [ ] Result schema and manifests validated against actual outputs
- [ ] Grouped training/held-out evaluation protocol documented
- [ ] Uncertainty reported for measured comparisons with dependence addressed
- [ ] Test family and multiplicity correction declared before analysis
- [ ] Cohen's **d** reported (not just p; no fixed cutoff)
- [ ] Training/evaluation seed roles recorded; robustness study completed if claimed
- [ ] Ablation outcomes reported only after matched experiments
- [ ] No 8×8/ensemble/RL claim in findings

## Findings-Critical Tables (Shells, Not Fake Rows)

Findings narrative is generated; tables live in `01-IMRD/03-results.md` §3. Do not duplicate placeholder rows here.

## Relation to Hypotheses

F1–F3→RQ1/RQ2 framework validation, H1→2048 baseline comparison, H2→application ranking, H3→tuning comparison, and feature ablation→exploratory analysis. Each outcome is reported with its appropriate evidence. A null application result is a finding, not a failure.

## Implementation Record

- Findings are only a reporting protocol. No results are populated; the original Parquet/statistics artifacts and ranking script do not exist. Checklist items remain pending until evidence artifacts are produced.

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

- **Findings remain pending.** Larger studies need a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
