# Rust-Native AutoML Framework Implemented and Evaluated Through 2048 ML

## 1. Project Overview

This project develops and evaluates an independent Rust-native AutoML architecture, `Evintkoo/automl`. The `2048-ml` repository is the implementation, integration, and evaluation project used to validate the framework through a demanding 4×4 2048 case study.

The project has two connected layers:

1. **Primary contribution: Rust-native AutoML architecture** — design, implement, verify, and benchmark the framework's preprocessing, model training, model selection, hyperparameter optimization, inference, and reproducibility architecture.
2. **Primary case study: 2048 ML** — implement the framework in an end-to-end stochastic game-learning pipeline and evaluate its practical limits.

The 2048 environment, data-generation pipeline, feature extraction, and evaluation tools are experimental infrastructure. Model training, preprocessing, model comparison, and hyperparameter optimization must be performed through the AutoML framework.

## 2. Research Problem

It is unclear whether a Rust-native AutoML architecture can provide a correct, reproducible, and practically competitive alternative for tabular machine learning, and whether its design remains effective when implemented in a sequential stochastic game-learning pipeline.

The research contribution is the framework architecture and its empirical validation. The 2048 system is the principal implementation and case study, not the sole source of novelty.

## 3. Research Goals

1. Define and justify the Rust-native AutoML architecture and its design trade-offs.
2. Implement the framework's preprocessing, training, validation, model-selection, hyperparameter-optimization, inference, and serialization components.
3. Verify framework correctness and reproducibility against explicit acceptance tests.
4. Benchmark the framework on standard tabular datasets against appropriate established baselines.
5. Integrate the framework into a correct and reproducible 4×4 2048 environment.
6. Use the framework to train 2048 policies without replacing its core training components with external ML libraries.
7. Measure the framework's effect on accuracy, runtime, memory, search efficiency, and reproducibility.
8. Analyze the contribution, robustness, and limitations of the 2048 state representation and labeling strategy.
9. Release sufficient code, configuration, data-generation procedures, and documentation for independent reproduction.

The objective is not to prove the globally highest possible 2048 score. The objective is to estimate performance under a controlled protocol and determine whether the AutoML workflow produces a competitive and reproducible policy.

## 4. Main Research Question

How can a Rust-native AutoML architecture be designed and validated for reproducible tabular machine learning, and what does its implementation in a stochastic 2048 policy-learning system reveal about its capability, efficiency, and limitations?

## 5. Secondary Research Questions

1. What architectural design enables a Rust-native AutoML framework to provide preprocessing, model training, validation, optimization, inference, and reproducibility in one system?
2. Does the framework satisfy its correctness, reproducibility, performance, and interoperability requirements on standard tabular tasks?
3. How does the framework perform when integrated into supervised 2048 policy learning?
4. Which supported model family and optimization configuration perform best in the 2048 case study?
5. Which state features, labels, and experimental conditions contribute most to application performance?

## 6. Project Steps

### A. Rust-Native AutoML Architecture

1. Document the architecture, module boundaries, data contracts, and design decisions.
2. Define acceptance tests for preprocessing, model training, cross-validation, metrics, serialization, prediction, and seed handling.
3. Benchmark supported models on standard tabular datasets.
4. Compare performance, runtime, memory use, search efficiency, and reproducibility with suitable established baselines.
5. Evaluate interoperability between CLI, library API, configuration, and persisted artifacts.
6. Record known framework limitations and unsupported capabilities.

### B. 2048 Environment

1. Implement or validate the complete 4×4 2048 game environment.
2. Implement tile movement, merging, scoring, spawning, valid moves, and terminal conditions.
3. Add deterministic seeded simulation and automated game-rule tests.
4. Validate the environment against an independent implementation or reference behavior where possible.

### C. State and Action Representation

1. Define the raw 4×4 board representation.
2. Define engineered state features and their exact ordering.
3. Define normalization and serialization rules.
4. Encode actions as four classes: Up, Down, Left, and Right.
5. Specify how invalid actions are handled.
6. Compare raw-grid, engineered-feature, and combined representations.

### D. Data and Training Pipeline

1. Generate reproducible game trajectories.
2. Define the label-generation method and rollout budget.
3. Separate training, validation, hyperparameter-search, and final evaluation data.
4. Train models through the AutoML framework only.
5. Compare default and optimized configurations.
6. Track model, dataset, seed, configuration, and software-version metadata.

### E. Framework and Application Evaluation

1. Evaluate framework metrics on standard tabular tasks using fixed splits and search budgets.
2. Compare 2048 policies against random, heuristic, search-based, and appropriate learning-based baselines where included by the protocol.
3. Evaluate mean and median score, score distribution, maximum tile, move count, and tile-reaching probabilities.
4. Define the statistical unit of analysis and account for paired games, repeated seeds, and repeated training runs.
5. Report confidence intervals, effect sizes, practical improvement thresholds, and corrected comparisons.
6. Perform feature-group ablations and sensitivity analysis.
7. Report negative, inconclusive, and failed results without unsupported claims.

## 7. Scope

### In Scope

- Independent Rust AutoML framework validation.
- Standard supervised tabular classification.
- Standard 4×4 2048.
- Four action classes.
- Reproducible game simulation.
- AutoML model comparison and hyperparameter optimization.
- Feature ablation and robustness analysis.
- Statistical evaluation and reproducibility artifacts.

### Out of Scope for the Core Study

- Using the AutoML web frontend as the primary research interface.
- Claiming the mathematically optimal or globally highest 2048 score.
- Treating one seed or one model run as general proof.
- Unsupported claims of PSPACE-hardness or feature sufficiency.
- Larger boards, unless added as a clearly separated generalization study.
- Reinforcement learning, deep learning, or MCTS reproduction unless included as explicitly defined comparison baselines.

## 8. Expected Contributions

1. A documented Rust-native AutoML architecture with explicit design rationale and data contracts.
2. An experimentally validated AutoML framework for reproducible tabular machine learning.
3. A reproducible 2048 implementation that demonstrates the framework in an end-to-end stochastic policy-learning pipeline.
4. Evidence about the framework's accuracy, efficiency, reproducibility, and limitations.
5. A reproducible benchmark and analysis package for future research.

The strength of the final contribution will depend on methodological correctness, comparison with strong baselines, robustness across independent conditions, and whether the framework or learning pipeline provides a demonstrably useful improvement beyond existing approaches.

## 9. Research Report

The final report will follow an IMRD structure supported by a theoretical framework, literature review, reproducibility appendix, and technical artifact documentation:

1. Introduction, research gap, and framework contribution.
2. Literature review: AutoML systems, Rust-native ML, reproducibility, and 2048 as a case study.
3. Rust-native AutoML architecture and design rationale.
4. Framework implementation and validation methodology.
5. 2048 environment, integration, and supervised policy-learning method.
6. Framework and application results, ablations, and robustness analysis.
7. Discussion, limitations, and threats to validity.
8. Reproducibility package and implementation artifacts.
9. Conclusions and future work.
