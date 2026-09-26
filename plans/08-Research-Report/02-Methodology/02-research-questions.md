# Plan 02 — Research Questions: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Research questions are defined but unanswered. A seed-42 standard-dataset diagnostic exists; repeatability, matched baselines, and confirmatory application evidence remain pending.

**Goal:** State the current implementation and evidence boundary for research questions.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Research questions are framing, not findings.** Initial framework diagnostics exist, but validation is incomplete. Plan-scale 2048 comparisons remain pending; no baseline superiority, model winner, or robustness result is claimed.

> **Canonical mapping:** 4 RQs, with the Rust-native AutoML architecture as the primary contribution and 2048 as the principal case study. Speculative theory is not treated as a primary research question.

## 1. Purpose

Define the exact RQs for both the independent AutoML framework and the 4×4 supervised application pipeline.

## 2. Canonical RQs

| RQ | Question | automl API Mapping | Test (see 03-hypotheses.md) | Status |
|----|----------|--------------------|-----------------------------|--------|
| **RQ1** | What Rust-native architecture and data contracts integrate the documented AutoML capabilities? | Architecture and source audit | Design-trade-off review | Framed; validation incomplete |
| **RQ2** | Does the framework satisfy declared correctness and usability criteria on standard tabular tasks? | Framework-validation protocol | Capability tests and matched benchmark study | Partial; diagnostic dataset matrix exists, matched baseline/resource study pending |
| **RQ3** | How does the framework-trained supervised policy compare with locally measured baselines and supported model candidates? | Four-action training and benchmark CLI | Declared pairwise protocol | Unanswered; plan-scale run pending |
| **RQ4** | What uncertainty and seed sensitivity characterize the 2048 case-study result? | Retained per-game scores and manifests | Declared experimental unit and sensitivity analysis | Unanswered; replication pending |

Secondary diagnostics (not separate RQs): feature-group contribution, label quality, convergence learning curves, and framework search efficiency — answered in ablation, framework validation, and findings.

## 3. RQ1 Detail — What is the architecture contribution?

Document the module boundaries, data contracts, configuration model, model/task abstraction, validation flow, optimization lifecycle, serialization boundary, seed handling, resource control, and CLI/library interoperability. The contribution must identify a design rationale or measurable systems trade-off rather than treating Rust reimplementation alone as novelty.

## 4. RQ2 Detail — Is the framework valid for this study?

The framework track verifies the required APIs, preprocessing behavior, validation strategy, model training, hyperparameter search, serialization, and reproducibility on standard tabular tasks. A missing or failing capability is reported as a framework finding and blocks the corresponding 2048 claim.

## 5. RQ3 Detail — How does the policy compare?

Compare only measured baseline distributions and supported model types. Before confirmatory runs, declare pairing, experimental unit, comparison family, sample-size rationale, and practical threshold. Available helpers do not include a global Kruskal–Wallis test.

## 6. RQ4 Detail — Magnitude and Reproducibility

Report summary and uncertainty from retained observations. Precision and seed sensitivity cannot be specified from assumed variance; do not escalate sample size automatically without an updated design rationale.

## 7. What Was Deleted and Why

| Deleted | Reason | Where Folded |
|---------|--------|--------------|
| NQ1 Markov blanket, NQ2 PAC bounds, NQ3 PSPACE-hardness | Out-of-scope theory; not required to validate the framework or case study | Excluded from core claims unless separately proven |
| TQ1–TQ3, extra SQ proliferation | Generic academic boilerplate | Merged into RQ1/RQ2 diagnostics |
| 8×8 / ensemble / RL phrasing | Not initial-plan (4×4 supervised only) | Future Work Appendix |

## 8. Traceability

```
RQ1 → framework-contribution.md → architecture and design-trade-off chapter
RQ2 → framework-validation.md → capability/correctness/resource report
RQ3 → H1+H2 in 03-hypotheses.md → winner table in 03-results.md → discussion
RQ4 → CIs, effect sizes, seed sensitivity → 03-Findings/05-sensitivity-analysis.md + discussion
```

No result is claimed before data; implemented statistics reside in `src/evaluation.rs` and are limited as documented in the benchmarking tickets.

## Implementation Record

- Research questions are defined, not answered. The named statistical module/API in the closing note does not exist under that path; score statistics live in `src/evaluation.rs`. The retained UCI diagnostic does not answer RQ2's matched comparison or resource claims.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/02-Methodology/02-research-questions.md` exits 0.

## Open questions

- **The questions remain unanswered.** Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
