# Evaluation Methodology — Supplement to Benchmarking Framework

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
