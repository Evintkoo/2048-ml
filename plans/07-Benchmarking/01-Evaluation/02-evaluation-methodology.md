# Plan 02 — Evaluation Methodology: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for evaluation methodology.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **This is a 30-line supplement, not a duplicate.** Full statistical tests live in `01-benchmarking-framework.md §6.4`; significance protocol lives in `04-Analysis/03-significance-testing.md`. This file adds only the constants and formulas not covered there.

## 1. Theoretical Limit — Unknown Max 32768

- Max tile on 4×4 is **32768 (2^15)** — capacity bound (2^16 needs 17 cells).
- Exact max score is **unknown** — perfect game may be unachievable.
- Models are ranked by **mean score only**, not proximity to a theoretical limit.

## 2. CI Width — Canonical Numbers

For `n = 10,000`, `σ ≈ 512`: `CI_95 width ≈ 2 × 1.96 × 512/√10000 ≈ 20` — narrow enough to rank models separated by ≥20 score points.

## 3. Evaluation Protocol

For the 2048 case study, use the protocol in `01-benchmarking-framework.md §6.2–6.6`: identical declared engine and evaluation conditions, held-out games, uncertainty intervals, practical effect sizes, and corrected comparisons. The case-study winner is the highest held-out mean only after model-selection and final-test separation; median and distribution are reported rather than used as automatic substitutes.

## 4. Controls & Limits

Same engine/data/criteria/seed/hardware; blinded analysis where applicable. Limits: 2048-only, supervised `MultiClassification` (27→4 logits), feature vector fixed 27-dim.

> For test layout and metric definitions see `01-benchmarking-framework.md` and `02-Metrics/`.

## Implementation Record

- The evaluation CLI records seeded benchmark conditions and score summaries. The CI-width arithmetic and assumed standard deviation are planning estimates, not measured values; final protocol still requires a held-out candidate selection lifecycle and completed runs.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
