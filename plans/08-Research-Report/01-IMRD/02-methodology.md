# Plan 02 — Methodology: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** This redirect points to a proposed design; UCI diagnostics and an exploratory 2048 comparison exist, but no confirmatory protocol has been executed.

**Goal:** State the current implementation and evidence boundary for methodology.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is a redirect ticket.** It points to the current design document and distinguishes diagnostics from the proposed confirmatory protocol. An exploratory seven-agent score comparison ran on training-disjoint matched seeds using paired intervals and Cohen's dz. The candidate ordering comes from one small-corpus fit per model and is not confirmatory. Sample-size rationale, preregistration, and winner criteria remain open.

> **This file is a 25-line redirect. Do not duplicate flowcharts or expand scope here. All protocol, variables, and gates are defined in `02-Methodology/01-experimental-design.md`.**

The planned case study uses the canonical 17-value state and four action labels with the AutoML training integration. Evaluation size, training and evaluation seed design, and comparison protocol must be justified and recorded before a confirmatory run; no fixed game-count target has been approved.

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
| Games | To be justified and declared | Exploratory seven-agent score ordering exists; no confirmatory winner has been established |

**Statistical protocol:** available CLI helpers choose a paired exact sign test, paired-difference bootstrap interval, and Cohen's dz when seed sets match; unmatched samples use Mann–Whitney U, independent bootstrap intervals, and Cohen's d. Both use Holm adjustment. Assumptions and the experimental unit require explicit review. No protocol has been preregistered and no winner claim is available. See `07-Benchmarking/04-Analysis/02-statistical-analysis.md`.

**No duplication:** No flowchart copy here; no PSPACE/Markov/8×8/ensemble/RL in core.

## Implementation Record

- Redirect audited against the experimental design, manifests, and dependency manifest. The policy-selection protocol and Parquet workflow are proposals rather than executed confirmatory protocol. The UCI diagnostics and exploratory 2048 comparison are retained separately; neither completes its respective validation gate. Exact submodule revision, seeds, and toolchain are retained in run manifests.

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
