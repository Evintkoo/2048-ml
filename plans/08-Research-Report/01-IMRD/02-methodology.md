# Plan 02 — Methodology: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** This redirect points to an experimental design that remains proposed; an initial UCI diagnostic exists, but no confirmatory protocol has been executed.

**Goal:** State the current implementation and evidence boundary for methodology.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is a redirect ticket.** It points to the current design document and clarifies that its numeric sample sizes, seeds, and benchmark setup are proposals. Implementation of a command or helper does not mean the corresponding experiment ran.

> **This file is a 25-line redirect. Do not duplicate flowcharts or expand scope here. All protocol, variables, and gates are defined in `02-Methodology/01-experimental-design.md`.**

The planned case study uses the canonical 17-value state and four action labels with the AutoML training integration. Evaluation size, training and evaluation seed design, and comparison protocol must be justified and recorded before a confirmatory run; the previously specified ≥10,000-game case-study run has not been executed.

**Canonical reference:** See `02-Methodology/01-experimental-design.md` for variables, trial structure, replication, bias controls, sample-size justification, and pre-registration. See `02-Methodology/03-hypotheses.md` for framework hypotheses F1–F3 and application hypotheses H1–H3, and `02-Methodology/04-ablation-study.md` for the ablation matrix.

**Repository configuration (not evidence of an executed experiment):**

| Component | Pinned Value | Notes |
|-----------|--------------|-------|
| automl | Local path dependency; submodule revision recorded in Git | Record exact revision and local modifications |
| Rust | Manifest minimum `1.75`; current toolchain may differ | Record actual compiler and target |
| Task | `TaskType::MultiClassification` | 4 actions 0–3 |
| Features | 17-value model vector | 16 board cells plus current score; game ID is provenance/group key |
| polars | `0.46` dependency | Root output paths include CSV; Parquet is not established for this workflow |
| Seeds | To be declared per study | Record training and evaluation seed roles separately |
| Games | To be justified and declared | No winner ranking has been performed |

**Statistical protocol:** available CLI helpers choose a paired exact sign test when seed sequences match, or Mann–Whitney U for unmatched samples; reports also include Holm adjustment, bootstrap intervals, and Cohen's d. Assumptions and the experimental unit require explicit review. No protocol has been preregistered and no winner claim is available. See `07-Benchmarking/04-Analysis/02-statistical-analysis.md`.

**No duplication:** No flowchart copy here; no PSPACE/Markov/8×8/ensemble/RL in core.

## Implementation Record

- Redirect audited against the experimental design, manifests, and dependency manifest. The 10k-game policy run, seed matrix, and Parquet workflow are proposals rather than executed protocol. The separate seed-42 standard-dataset diagnostic is recorded under `reports/framework_validation/`; it does not complete the framework gate or case-study protocol. Exact submodule revision and toolchain are retained in its run manifests.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.

## Open questions

- **The full study protocol remains pending.** Declare experimental units, seed roles, comparison assumptions, sample-size rationale, resource budget, and retained artifacts before confirmatory runs.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
