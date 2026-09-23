# Plan 02 — Research Questions: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for research questions.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Canonical mapping:** 4 RQs, with the Rust-native AutoML architecture as the primary contribution and 2048 as the principal case study. Speculative theory is not treated as a primary research question.

## 1. Purpose

Define the exact RQs for both the independent AutoML framework and the 4×4 supervised application pipeline.

## 2. Canonical RQs

| RQ | Question | automl API Mapping | Test (see 03-hypotheses.md) | Status |
|----|----------|--------------------|-----------------------------|--------|
| **RQ1** | What Rust-native architecture and data contracts are required to integrate preprocessing, training, validation, optimization, inference, serialization, and reproducibility in one AutoML system? | Architecture and implementation documentation | Architecture analysis and design-trade-off review | TBD |
| **RQ2** | Does the implemented framework satisfy its correctness, reproducibility, efficiency, and interoperability requirements on standard tabular tasks? | Framework validation protocol in `07-Benchmarking/03-Comparison/04-framework-validation.md` | Capability, correctness, benchmark, and resource tests | TBD |
| **RQ3** | Can the validated framework train a 4×4 policy whose mean score exceeds the heuristic baseline, and which `ModelType` performs best? | `TrainingConfig { task_type: MultiClassification, target: "action", feature_columns: 27 }` → `TrainEngine::fit` → benchmark framework | H1/H2 statistical protocol | TBD |
| **RQ4** | What is the magnitude, robustness, and practical significance of the 2048 case-study result? | Same pipeline across training/evaluation seeds and controlled sensitivity conditions | CIs, effect sizes, seed and sensitivity analysis | TBD |

Secondary diagnostics (not separate RQs): feature-group contribution, label quality, convergence learning curves, and framework search efficiency — answered in ablation, framework validation, and findings.

## 3. RQ1 Detail — What is the architecture contribution?

Document the module boundaries, data contracts, configuration model, model/task abstraction, validation flow, optimization lifecycle, serialization boundary, seed handling, resource control, and CLI/library interoperability. The contribution must identify a design rationale or measurable systems trade-off rather than treating Rust reimplementation alone as novelty.

## 4. RQ2 Detail — Is the framework valid for this study?

The framework track verifies the required APIs, preprocessing behavior, validation strategy, model training, hyperparameter search, serialization, and reproducibility on standard tabular tasks. A missing or failing capability is reported as a framework finding and blocks the corresponding 2048 claim.

## 5. RQ3 Detail — Can automl beat ~512 and who wins?

**H0 (H1):** μ_best ≤ μ_heuristic (~512). **H1:** μ_best > ~512. **Primary test:** pre-registered one-sided Mann-Whitney U at α=0.05 with Holm correction for the planned baseline comparisons. Report bootstrap 95% CI and effect size as evidence, not additional mandatory gates. **Ranking:** highest held-out mean over 10k games; use uncertainty intervals and repeated seeds to describe close results.

**H0 (H2):** All 7 `ModelType` means equal. **H1:** At least one differs. **Test:** Kruskal-Wallis → Dunn post-hoc Bonferroni.

## 6. RQ4 Detail — Magnitude and Reproducibility

Report: winner mean, bootstrap 95% CI (~±20 at σ512/10k), d vs heuristic, and seed sensitivity `σ(μ_seeds)/mean(μ_seeds)`. If ranking flips across seeds → winner inconclusive, escalate to 50k.

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

No result claimed before data; all answers pending `statistical_tests.rs` on `ScoreMetrics`.

## Implementation Record

- Research questions are defined, not answered. The named statistical module/API in the closing note does not exist under that path; score statistics live in `src/evaluation.rs`.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
