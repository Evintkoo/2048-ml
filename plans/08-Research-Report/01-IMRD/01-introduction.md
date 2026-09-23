# Introduction

> The primary contribution is the design and validation of a Rust-native AutoML architecture. The 2048 ML system is the implementation and principal case study used to evaluate that architecture.

## 1. Background and Primary Contribution

The central research object is not only a model that plays 2048. It is an integrated Rust-native AutoML architecture that combines data processing, preprocessing, model training, validation, hyperparameter optimization, inference, serialization, and reproducible experiment control. The 2048 system provides a demanding implementation context in which these framework components must operate together on sequential stochastic data.

The 2048 game is a 4×4 stochastic puzzle where each move slides tiles (Up/Down/Left/Right, encoded 0-3) and spawns a 2 (p=0.9) or 4 (p=0.1). State space is `16` cells but decision depth ~50-100 moves makes optimal play intractable. This work tests whether `Evintkoo/automl v1.0.0` — a Rust AutoML engine — can learn a competitive 4×4 supervised policy from a fixed 27-dim feature vector via `TaskType::MultiClassification` over action labels `0..3`.

## 2. Research Context

2048 was released by Cirulli (2014). Prior work: heuristic corner/monotonicity agents (`~512` mean on 4×4, Björk 2014), expectimax (depth 4-6, but out-of-scope for core evaluation — see Appendix), RL/DQN and MCTS+NN (out-of-scope, see Future Work). AutoML for 4×4 supervised classification on puzzle games remains underexplored. This project fills that gap with a Rust-native pipeline: `TrainEngine` + `HyperOptX` (TPE) + `CrossValidator::GroupKFold(game_id)` + `polars 0.46`.

Case-study ranking rule (canonical, §3): highest held-out mean score across the declared 2048 evaluation games, with uncertainty, practical effect, and seed-level robustness reported alongside the ranking. This is not the framework's primary success criterion.

Framework validation is reported separately from the 2048 winner ranking. Search-based and learning-based agents may be included as explicitly defined comparison baselines; they are not used to train the core AutoML policy.

## 3. Problem Statement

The study has two linked research layers. First, the independent `automl` implementation must be validated for API correctness, preprocessing integrity, model training, cross-validation, hyperparameter optimization, serialization, and reproducibility on standard tabular tasks. Second, the validated framework is applied to 4×4 2048. A 2048 result cannot be interpreted as evidence about AutoML quality if the required framework capabilities have not passed the validation gate.

**RQ1 (primary):** How can a Rust-native AutoML architecture be designed and validated to provide reproducible preprocessing, model training, validation, optimization, inference, and artifact management?

**RQ2 (case study):** Can the validated framework train a 4×4 policy whose mean score exceeds heuristic `~512` under the statistical protocol above, and which `ModelType` performs best over 10k games?

**Framework validation:** Does the implemented architecture satisfy its correctness, reproducibility, efficiency, and interoperability requirements on standard tabular tasks before the 2048 application results are interpreted?

**Secondary:**
- RQ2: What is the best mean score and its bootstrap CI vs heuristic `~512` and random `~128`?
- RQ3: Is the winner reproducible across seeds 42/123/456/789/1011?
- RQ4: Which 27-dim groups drive performance (ablation, GroupKFold, 10k games each)?

All answers are **pending experimentation**; methodology for answering them is defined in `02-Methodology/01-experimental-design.md`.

## 4. Research Objectives

| Objective | Deliverable | Criterion |
|-----------|-------------|-----------|
| Validate the independent framework | Capability and correctness report | Required APIs and workflows pass validation |
| Maximize mean score via automl | Trained `TrainEngine` models | Mean over 10k games, seed 42 |
| Identify best algorithm | Ranked table | MWU/Holm + bootstrap CI + effect size |
| Establish statistical rigor | `statistical_tests.rs` | Holm correction, bootstrap CI, effect size |
| Validate 27-dim engineering | Ablation (27 LOO + 8 groups) | GroupKFold, cost-capped |
| Reproducibility | Package (§06) | Rust 1.75, automl v1.0.0, seed 42 |

## 5. Novel Contributions (In-Scope, Supervised 4×4 Only)

1. **Rust-native AutoML architecture** with documented module boundaries, data contracts, and design trade-offs.
2. **Framework validation** covering correctness, reproducibility, efficiency, interoperability, and standard tabular benchmarks.
3. **Supervised 4×4 AutoML case study**: 27-dim → `MultiClassification(0-3)` → `TrainEngine` → benchmark evaluation.
4. **Rigorous application evaluation** using uncertainty intervals, effect sizes, and repeated-condition analysis.
5. **Reproducibility package** pinned to `rust 1.75`, `polars 0.46`, `automl v1.0.0`, and documented seeds.

> **Future / Appendix (not core):** PSPACE-hardness conjecture, Markov blanket / information-theoretic analysis, PAC lower-bound refinements, 8×8 or larger boards, ensemble/stacking, RL/MCTS reproduction, and GPU acceleration. These are not needed to establish the Rust-native AutoML architecture or the 2048 case study.

## 6. Significance

- Establishes what a Rust-native AutoML architecture can provide as an integrated, reproducible tabular ML system.
- Measures the framework's accuracy, resource use, search efficiency, interoperability, and failure modes.
- Demonstrates the architecture in a sequential stochastic 2048 policy-learning pipeline.
- Quantifies which engineered groups, labels, and application conditions affect the case-study result.

## 7. Scope and Limitations

**In scope:** Independent AutoML framework validation, standard tabular benchmark tasks, standard 4×4 2048, automl via `TrainEngine`/`HyperOptX` (CLI/API), supervised `MultiClassification` (27→4), mean-score ranking over 10k games, GroupKFold by `game_id`, multi-seed robustness, and `polars 0.46`.

**Out of scope → Future/Appendix only:** 8×8 or other board sizes, ensemble/stacking, RL/policy gradients, GPU, web frontend, mobile, and multi-agent settings. Search-based and learning-based agents may be included as explicitly defined comparison baselines, but they are not the primary contribution. See `02-Methodology/01-experimental-design.md` §10 and `04-discussion.md` §7.

**Limitations (acknowledged):** `automl` capability gate pending (`ModelType`, `TaskType::MultiClassification`, `CrossValidator`); 27-dim is fixed; single-seed primary (multi-seed planned); rollout labels (100 sims/action) are noisy proxies.

## 8. Paper Structure

1. Introduction (this file) 2. Methodology → canonical `02-Methodology/01-experimental-design.md` 3. Results → `03-results.md` (framework and case-study interfaces) 4. Discussion → `04-discussion.md` (framework outcomes and application hypotheses) 5. Findings/Appendix.

## 9. Honest Reporting Commitment

No result fabricated. Null/inconclusive outcomes, training failures, and any automl capability gap will be reported with exact p-values, CIs, and effect sizes.
