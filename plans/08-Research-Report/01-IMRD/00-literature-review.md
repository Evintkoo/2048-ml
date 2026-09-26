# Plan 00 — Literature Review: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Core HPO, AutoML-system, and selected 2048 references are verified; Rust-ecosystem and broader claim-to-source review remain incomplete.

**Goal:** State the current implementation and evidence boundary for literature review.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is partial.** Bibliographic records and bounded claims for core HPO systems and two 2048 studies have been checked against proceedings, publisher, or repository records. The draft is not a completed comparative literature review. Unsupported 2048 score estimates and categorical Rust performance claims are not treated as facts.

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

Rodgers and Levine (2014) compare Monte-Carlo Tree Search with Averaged Depth Limited Search for 2048; their abstract reports that ADLS performed better when supplied with a board-property evaluation function, while heuristic-guided MCTS rollouts did not help in their setup. This is a short conference paper and its result is not a baseline for the present simulator. The original game's publication/history source still needs verification.

Prior notes contained unverified score estimates for heuristic and random policies and an unsupported attribution. Those figures are removed as evidence. Any future baseline claim requires a verifiable source or a reproducible local measurement under a declared protocol. Search agents remain optional context under the canonical scope.

### 3.2 Heuristic Context (Separate from the Canonical 17-Value Input)

Candidate heuristic concepts include empty-cell count, monotonicity, smoothness, and merge potential. These are not additional canonical training inputs. Their sources and empirical predictive value need verification; implementation of similarly named measurements does not validate a literature claim.

### 3.3 ML on 2048 (Case-Study Context)

Szubert and Jaskowski (2014) study temporal-difference learning with n-tuple networks; Szubert et al. (2016) report extensions using delayed temporal coherence, multi-stage weight promotion, redundant encoding, and carousel shaping. These are reinforcement-learning methods and contextual prior work, not direct baselines for this supervised AutoML case study. Their task setups and published scores must not be compared with local scores without protocol-level reconciliation. External reproduction remains optional context, not an existing result.

### 3.4 Theory

Earlier notes about 15-puzzle complexity, a theoretical maximum tile, and hardness claims are not established by this project. Verify primary sources and mathematical assumptions before retaining any such statement; none is needed for the core framework contribution.

## 4. AutoML Methods Used by the Framework

### 4.1 Hyperparameter Optimization

[Bergstra et al. (2011)](https://papers.nips.cc/paper/4443-algorithms-for-hyper-parameter-optimization) introduce model-based HPO using a tree-structured Parzen estimator; [Bergstra and Bengio (2012)](https://jmlr.org/papers/v13/bergstra12a.html) evaluate random search; [Li et al. (2018)](https://jmlr.org/papers/volume18/16-558/16-558.html) present Hyperband; and [Feurer and Hutter (2019)](https://doi.org/10.1007/978-3-030-05318-5_1) survey HPO methods. These records were checked against proceedings or publisher pages; they support method descriptions, not claims about this project's performance.

System-level examples clarify the distinction between an optimizer and an AutoML system. [Auto-WEKA](https://doi.org/10.1145/2487575.2487629) formulates combined algorithm selection and hyperparameter optimization (CASH) over WEKA classifiers and feature selectors. [auto-sklearn](https://proceedings.neurips.cc/paper/2015/hash/11d0e6287202fced83f79975ec59a3a6-Abstract.html) adds meta-learning warm starts and ensembles to Bayesian optimization. [TPOT](https://proceedings.mlr.press/v64/olson_tpot_2016.pdf) searches composed pipelines using genetic programming and evaluates them on classification tasks. These papers illustrate different search-space and lifecycle choices; their benchmark results are not directly comparable across protocols and do not establish that any system is superior to this project's framework.

The root integration uses `HyperOptX` with TPE for a limited RandomForest/ExtraTrees search over two parameters. Although the framework exposes a `MedianPruner` type, the root optimizer callback has no intermediate-reporting hook and pruning is disabled. Hyperband is contextual literature only; it is not implemented in the active root search.

### 4.2 AutoML for Sequential Applications (Secondary Gap)

For contextual comparison only, [Silver et al. (2017)](https://doi.org/10.1038/nature24270) study self-play reinforcement learning for Go, while [Schrittwieser et al. (2020)](https://doi.org/10.1038/s41586-020-03051-4) study planning with a learned model across games. These are not supervised tabular AutoML evaluations and are not direct baselines for this project. The secondary gap is the limited evaluation of general-purpose AutoML systems in sequential stochastic applications with trajectory-generated tabular data.

## 5. Rust ML and Framework Positioning

- `automl v1.0.0` (`https://github.com/Evintkoo/automl`): the pinned source declares `TrainEngine`, `TaskType::MultiClassification`, multiple `ModelType` variants, and `CrossValidator`. Five variants (RandomForest, ExtraTrees, AdaBoost, KNN, NaiveBayes) have verified four-class probability output for the current 2048 task; other available variants are not assumed compatible.
- `polars 0.46`, `smartcore 0.3`, `linfa 0.7` (see `automl/Cargo.toml`).
- Linfa and SmartCore are documented as Rust ML ecosystem projects in their [official repository](https://github.com/rust-ml/linfa) and [official repository](https://github.com/smartcorelib/smartcore). These project descriptions establish scope and API positioning only; no peer-reviewed, controlled Rust-versus-other-language performance evidence has been identified here. Bravegates & Renzelmann (2019, arXiv Rust ML survey) and Matsakis/Lamport claims remain **unverified**.

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

The linked AutoML and 2048 sources above have verified records for the limited claims stated next to them. References still requiring primary-source and claim checks include Rust ML surveys, 2048 history and formal game properties, detailed reproducibility/benchmark methodology, and every final thesis claim. Software repositories document current project capabilities but are not substitutes for peer-reviewed comparative evidence. See `04-Appendix/03-references.md` for dependency provenance.

## Implementation Record

- Review topics and a source-verification policy are outlined. Primary records for random search, TPE, Hyperband, Auto-WEKA, auto-sklearn, TPOT, and selected 2048 search/RL work have been checked and linked. A limited architecture comparison is now stated. Rust ecosystem performance, reproducibility literature, 2048 history, and broader claim-to-source coverage remain pending; historical scores and categorical Rust performance assertions remain excluded.

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
