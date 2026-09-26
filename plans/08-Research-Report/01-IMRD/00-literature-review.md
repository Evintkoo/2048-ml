# Plan 00 — Literature Review: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Review topics and evidence policy are outlined; citations, bibliographic records, and claim-to-source verification remain incomplete.

**Goal:** State the current implementation and evidence boundary for literature review.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is partial.** It identifies the review domains and requires primary-source verification. The current draft still contains provisional claims and is not a completed literature review; unsupported 2048 score estimates and Rust performance claims are not treated as facts.

## 1. Scope and Method

The primary review covers four connected areas: (i) AutoML system architecture and automated model selection, (ii) Rust-native machine-learning systems and ecosystem design, (iii) reproducible ML infrastructure and benchmarking, and (iv) 2048 heuristics/search and supervised sequential decision-making as the application case study. Every citation must be verified before appearing in the final thesis; provisional sources are explicitly marked and cannot support final claims.

The literature review must establish the framework gap first. 2048 literature is used to justify the case study, state representation, labels, and evaluation—not to replace the AutoML architecture contribution.

## 2. AutoML Systems and Architecture — Primary Literature

### 2.1 Automated Model Selection

Review classical model-selection pipelines, automated preprocessing, algorithm selection, hyperparameter optimization, early stopping, and resource-aware search. Distinguish AutoML systems from individual optimizers such as random search, TPE, and Hyperband.

### 2.2 AutoML System Design

Compare the architectural choices of established AutoML systems: data abstractions, pipeline composition, search-space representation, trial management, validation, persistence, parallelism, failure handling, and experiment tracking. The review must identify which capabilities are missing, difficult to reproduce, or poorly integrated in existing systems.

### 2.3 Rust-Native ML Systems

Review Rust ML and data-processing ecosystems, including type safety, ownership, parallel execution, serialization, FFI/interoperability, deployment, and the trade-off between native performance and algorithm/library coverage. Do not claim Rust is categorically faster; compare measured dimensions and cite primary technical sources.

### 2.4 Reproducibility and Benchmarking

Review dataset leakage, nested validation, random-seed control, benchmark fairness, resource reporting, artifact evaluation, and reproducible computational experiments. These principles define the framework-validation protocol.

## 3. Application Case Study: 4×4 2048

### 3.1 History

Source leads concerning the original game, weighted heuristics, expectimax, and formal decision-process treatments require bibliographic verification before inclusion. The current notes do not establish authorship, publication dates, methods, or results.

Prior notes contained unverified score estimates for heuristic and random policies and an unsupported attribution. Those figures are removed as evidence. Any future baseline claim requires a verifiable source or a reproducible local measurement under a declared protocol. Search agents remain optional context under the canonical scope.

### 3.2 Heuristics (Relevant to 27-dim)

Candidate heuristic concepts include empty-cell count, monotonicity, smoothness, and merge potential. Their sources and empirical predictive value need verification; implementation of similarly named features does not validate a literature claim.

### 3.3 ML on 2048 (Case-Study Context)

Prior notes named several possible ML and search sources and attached score ranges without verified bibliographic records or reproduced protocols. Do not cite those names or scores until the sources, task setup, and comparability are checked. External reproduction is optional context, not an existing result.

### 3.4 Theory

Earlier notes about 15-puzzle complexity, a theoretical maximum tile, and hardness claims are not established by this project. Verify primary sources and mathematical assumptions before retaining any such statement; none is needed for the core framework contribution.

## 4. AutoML Methods Used by the Framework

### 4.1 Hyperparameter Optimization

Bergstra & Bengio (2012, JMLR) random search; Bergstra et al. (2013) Hyperopt TPE; Li et al. (2017) Hyperband; Feurer & Hutter (2019) AutoML survey. Relevant: `HyperOptX` TPE + `MedianPruner` in `automl/src/optimizer`.

### 4.2 AutoML for Sequential Applications (Secondary Gap)

Silver et al. (2017), Schrittwieser et al. (2020) – deep RL + MCTS, used only as contextual comparison. The secondary gap is the limited evaluation of general-purpose AutoML systems in sequential stochastic applications with trajectory-generated tabular data.

## 5. Rust ML and Framework Positioning

- `automl v1.0.0` (`https://github.com/Evintkoo/automl`): `TrainEngine`, `TaskType::MultiClassification`, `ModelType::{RandomForest, GradientBoosting, XGBoost, LightGBM, ExtraTrees, SVM, KNN}`, `CrossValidator`.
- `polars 0.46`, `smartcore 0.3`, `linfa 0.7` (see `automl/Cargo.toml`).
- Bravegates & Renzelmann (2019, arXiv unverified – Rust ML survey) and Matsakis/Lamport claims marked **unverified**.

## 6. Gap Analysis

| # | Gap | In-Scope? | How Addressed |
|---|-----|-----------|---------------|
| 1 | Integrated Rust-native AutoML architecture | Yes | Architecture, data contracts, capability matrix, and validation |
| 2 | Reproducible Rust AutoML benchmarking | Yes | Fixed datasets, splits, budgets, seeds, and resource reporting |
| 3 | AutoML framework behavior in sequential stochastic data | Yes | 2048 case study with trajectory and policy evaluation |
| 4 | Model and hyperparameter selection under one native pipeline | Yes | Standard tabular and 2048 comparisons |
| 5 | Feature-group and label contribution in 2048 | Secondary | Ablation and sensitivity analysis |
| 6 | n×n / PSPACE / Markov blanket / information theory | No | Remove from core; retain only if rigorously justified |
| 7 | RL / DQN / MCTS reproduction | Optional context | Include only as explicitly defined baselines |

Remainder gaps are out-of-scope and removed (no TBD filler).

## 7. Contribution (Matches Introduction §5)

The primary contribution is the Rust-native AutoML architecture and its validation. The 2048 integration is the principal case study demonstrating end-to-end usability, not the only contribution. No PSPACE, Markov-blanket, or PAC claim is part of the core contribution without a separate rigorous proof.

## 8. References (Verified vs Provisional)

Potential starting points include primary work on random search and Hyperband, AutoML surveys, framework repositories, and official Rust/data-system documentation. Their exact records, versions, and claim support still require checking. No candidate listed here should be treated as verified solely because it appears in this draft. See `04-Appendix/03-references.md` for bibliography curation.

## Implementation Record

- Review topics and a source-verification policy are outlined. Exact bibliographic records and claim-to-source checks remain pending; historical, theoretical, 2048 score, and Rust performance assertions were demoted to unverified leads or removed as evidence.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
2. `grep -q '^# Plan 00 — ' plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/00-literature-review.md` exits 0.

## Open questions

- **The review remains bounded by verified sources.** Complete bibliography and claim checks before using literature to support thesis conclusions.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
