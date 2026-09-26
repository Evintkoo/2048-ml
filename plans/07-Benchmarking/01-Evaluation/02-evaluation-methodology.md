# Plan 02 — Evaluation Methodology: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Benchmark CLI records seeds and score summaries; final held-out model protocol and powered sample size remain pending.

**Goal:** State the current implementation and evidence boundary for evaluation methodology.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats benchmark mechanics as implemented and final evaluation design as pending.** The CLI can record seeded game outcomes and uncertainty summaries. No trained model, selected-model holdout, or prospective power analysis is available.

> **This is a 30-line supplement, not a duplicate.** Full statistical tests live in `01-benchmarking-framework.md §6.4`; significance protocol lives in `04-Analysis/03-significance-testing.md`. This file adds only the constants and formulas not covered there.

## 1. Score Interpretation

Do not state an unproven theoretical maximum tile or score. Report observed game outcomes under the declared simulator protocol; score is not the training target.

## 2. CI Width — Canonical Numbers

The old illustration assumed `n=10,000` and `σ≈512`; these values are not measured or prospective power results. Estimate uncertainty from retained pilot/game data and choose the test size within a declared budget.

## 3. Evaluation Protocol

For the 2048 case study, use the protocol in `01-benchmarking-framework.md §6.2–6.6`: identical declared engine and evaluation conditions, held-out games, uncertainty intervals, practical effect sizes, and corrected comparisons. The case-study winner is the highest held-out mean only after model-selection and final-test separation; median and distribution are reported rather than used as automatic substitutes.

## 4. Controls & Limits

Use the same simulator configuration, candidate data/split, seed design, and declared hardware where applicable. The 2048 result is application evidence; it does not establish framework superiority. The policy uses four-class probabilities.

> For test layout and metric definitions see `01-benchmarking-framework.md` and `02-Metrics/`.

## Implementation Record

- The evaluation CLI records seeded conditions and score summaries. The prior CI-width arithmetic and assumed standard deviation were planning illustrations, not measured results; final protocol requires model selection/holdout separation and completed runs.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/01-Evaluation/02-evaluation-methodology.md` exits 0.

## Open questions

- Define a held-out game-seed protocol and prospective uncertainty/sample-size approach after pilot variance is measured. Retain per-game scores and analysis code.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
