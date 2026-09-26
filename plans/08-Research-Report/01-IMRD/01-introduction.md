# Plan 01 — Introduction: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Repeated UCI framework diagnostics on the pinned revision exist; matched framework comparisons, plan-scale 2048 experiments, and verified literature claims remain pending.

**Goal:** State the current implementation and evidence boundary for introduction.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is a research framing document, not a findings report.** The AutoML framework capability checks are partial; standard-dataset validation and the 2048 model study have not met their planned gates. Literature-based baseline estimates and novelty claims are not verified.

> The primary contribution is the design and validation of a Rust-native AutoML architecture. The 2048 ML system is the implementation and principal case study used to evaluate that architecture.

## 1. Background and Primary Contribution

The central research object is the design and validation of a Rust-native AutoML architecture. The 2048 system is the principal application case study. Several architecture components are implemented, but integrated standard-dataset validation and the complete reproducibility study remain pending.

2048 is a stochastic 4×4 tile-merging game. The simulator defaults to spawning a 4 with probability 0.1; its available actions are Up, Down, Left, and Right. This study evaluates a supervised policy using the canonical 17-value state (16 board cells plus current score) and four action labels with the pinned Rust AutoML submodule. The case-study experiments remain pending.

## 2. Research Context

The literature review has not yet verified publication details, prior-agent protocols, or score estimates. Do not present the draft heuristic mean or novelty claim as established. The implemented pipeline includes AutoML training and grouped validation paths; Parquet and the full planned benchmark workflow are not established as completed.

Case-study ranking rule (canonical, §3): highest held-out mean score across the declared 2048 evaluation games, with uncertainty, practical effect, and seed-level robustness reported alongside the ranking. This is not the framework's primary success criterion.

Framework validation is reported separately from the 2048 winner ranking. Search-based and learning-based agents may be included as explicitly defined comparison baselines; they are not used to train the core AutoML policy.

## 3. Problem Statement

The study has two linked research layers. First, the independent `automl` implementation must be validated for API correctness, preprocessing integrity, model training, cross-validation, hyperparameter optimization, serialization, and reproducibility on standard tabular tasks. Second, the validated framework is applied to 4×4 2048. A 2048 result cannot be interpreted as evidence about AutoML quality if the required framework capabilities have not passed the validation gate.

**RQ1 (primary):** How can a Rust-native AutoML architecture be designed and validated to provide reproducible preprocessing, model training, validation, optimization, inference, and artifact management?

**RQ2 (case study):** Under a predeclared, reproducible protocol, how does the supervised policy perform relative to measured baselines, and how do supported model types compare? Baseline values, evaluation scale, and ranking remain to be established by experiment.

**Framework validation:** Does the implemented architecture satisfy its correctness, reproducibility, efficiency, and interoperability requirements on standard tabular tasks before the 2048 application results are interpreted? An initial seed-42 diagnostic run covers three UCI datasets and five candidate models; it is not the completed validation gate.

**Secondary:**
- RQ2: What are the mean scores, uncertainty intervals, and differences versus measured random and heuristic baselines?
- RQ3: Is the winner reproducible across seeds 42/123/456/789/1011?
- RQ4: Which declared feature subsets of the 17-value canonical state affect case-study performance? This ablation question remains a proposal without a frozen protocol or completed run.

All answers are **pending experimentation**; methodology for answering them is defined in `02-Methodology/01-experimental-design.md`.

## 4. Research Objectives

| Objective | Deliverable | Criterion |
|-----------|-------------|-----------|
| Validate the independent framework | Capability and correctness report | Required APIs and workflows pass validation |
| Evaluate the 2048 policy | Trained models and retained results | Protocol and evaluation size to be declared before the study |
| Compare supported models | Pairwise comparison report | Use implemented tests with assumptions and limitations recorded |
| Establish statistical rigor | Statistical analysis artifact | Helpers exist; full protocol and power rationale remain pending |
| Validate feature contribution | Ablation study | Planned; no ablation experiment or runner completed |
| Reproducibility | Reproduction package | Pinned framework revision and actual tool versions recorded |

## 5. Novel Contributions (In-Scope, Supervised 4×4 Only)

1. **Rust-native AutoML architecture** documented with module boundaries, data contracts, and design trade-offs.
2. **Framework validation study** planned for correctness, reproducibility, efficiency, interoperability, and standard tabular benchmarks; results remain pending.
3. **Supervised 4×4 case study** implemented as a 17-value-state, four-action AutoML pipeline; plan-scale evaluation remains pending.
4. **Application evaluation** includes initial score and comparison tools; full uncertainty and repeated-condition evidence remain pending.
5. **Reproducibility package** is planned; exact toolchain and artifact evidence must come from the executed study.

> **Future / Appendix (not core):** PSPACE-hardness conjecture, Markov blanket / information-theoretic analysis, PAC lower-bound refinements, 8×8 or larger boards, ensemble/stacking, RL/MCTS reproduction, and GPU acceleration. These are not needed to establish the Rust-native AutoML architecture or the 2048 case study.

## 6. Significance

- Establishes what a Rust-native AutoML architecture can provide as an integrated, reproducible tabular ML system.
- Measures the framework's accuracy, resource use, search efficiency, interoperability, and failure modes.
- Demonstrates the architecture in a sequential stochastic 2048 policy-learning pipeline.
- Quantifies which engineered groups, labels, and application conditions affect the case-study result.

## 7. Scope and Limitations

**In scope:** Independent AutoML framework validation, standard tabular tasks, 4×4 2048, supervised four-action learning, grouped validation by game, model comparisons, and reproducibility. Dataset count, game count, and seed matrix are experimental design choices that must be justified and recorded before collection.

**Out of scope → Future/Appendix only:** 8×8 or other board sizes, ensemble/stacking, RL/policy gradients, GPU, web frontend, mobile, and multi-agent settings. Search-based and learning-based agents may be included as explicitly defined comparison baselines, but they are not the primary contribution. The experimental design excludes larger boards, RL training, and ensembles ([design limitations §10](../02-Methodology/01-experimental-design.md)); the discussion keeps those as future work ([discussion §8](04-discussion.md)).

**Limitations:** A one-split standard-dataset diagnostic exists, but the full framework-validation gate remains pending; the canonical state has 17 values; multi-seed robustness is pending; rollout labels use finite stochastic simulations and are proxies rather than optimal actions. Candidate baseline scores in the draft are unverified.

## 8. Paper Structure

1. Introduction (this file) 2. Methodology → canonical `02-Methodology/01-experimental-design.md` 3. Results → `03-results.md` (framework and case-study interfaces) 4. Discussion → `04-discussion.md` (framework outcomes and application hypotheses) 5. Findings/Appendix.

## 9. Honest Reporting Commitment

No result fabricated. Null/inconclusive outcomes, training failures, and any automl capability gap will be reported with exact p-values, CIs, and effect sizes.

## Implementation Record

- Research framing and honest-reporting requirements are documented. The pinned UCI diagnostic results and historical pre-fix Wine KNN repeatability/serialization issue are recorded in `reports/framework_validation/`; matched baselines, broader framework validation, and plan-scale 2048 experiments remain pending. Baseline literature claims and novelty must be supported by the completed source review before publication.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/01-introduction.md` exits 0.

## Open questions

- **Research conclusions remain bounded by collected evidence.** Declare compute budgets before large studies and retain datasets, configurations, seeds, versions, and analysis outputs.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
