# Plan 01 — Rust-Native AutoML Framework: the repository status is explicit and evidence based

> **Status: PARTIAL.** Capability checks, standard-dataset diagnostics, a 20-game rollout pilot, and one 2048 train-to-simulator smoke are recorded; matched framework evaluation, scale collection, and held-out policy results remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** A fixed-split diagnostic covers three standard datasets and five AutoML candidates. The initial pinned revision exposed nondeterministic tie handling in KNN and ExtraTrees; after a narrow framework fix, two independent runs against pinned AutoML commit `82d848323eed5e2af86d046d529916c448f2442c` succeeded for all 15 cases and matched predictions in 15/15 pairs. A fixed-configuration scikit-learn comparison and aggregate same-host resource probe now exist, but search budgets and per-model resource boundaries are not matched. A 20-game rollout corpus also trained one RandomForest and completed a separate 20-game simulator smoke; these verify pipeline wiring only. The main 2048 study and full research results remain pending.

> **Project:** 2048 Machine Learning System
> **Version:** 1.0.0
> **Author:** Evintkoo
> **Created:** 2026-09-22
> **Status:** In progress — capability checks passed for five four-class candidates; repeated standard-dataset diagnostics and save/load equality are retained after deterministic tie fixes. A 20-game canonical-schema corpus has one AutoML training and policy-simulation smoke. Matched framework comparisons, scale collection, and held-out 2048 policy evaluation remain incomplete (updated 2026-09-27).

---

## 1. Purpose

This project implements and evaluates the independently developed Evintkoo/automl framework as a Rust-native AutoML architecture. The `2048-ml` repository provides the main integration and case-study environment: a stochastic 4×4 game in which the framework trains supervised action policies.

The 2048 environment, data-generation pipeline, feature extraction, and evaluation tools are experimental infrastructure. Model training, preprocessing, model comparison, and hyperparameter optimization must be performed through the AutoML framework.

## 2. Goals

1. **Primary framework contribution:** Design, implement, and validate the Rust-native AutoML architecture.
2. **Framework evaluation:** Measure correctness, reproducibility, accuracy, efficiency, and search behavior against established baselines.
3. **2048 case study:** Demonstrate the framework in an end-to-end supervised stochastic policy-learning system.

## 3. Scope

### In Scope
- AutoML framework architecture, implementation, and capability validation on standard tabular tasks
- 2048 game environment creation and simulation
- State representation and feature engineering
- Action space definition and encoding
- Model training using automl engine
- Data collection and management
- Benchmarking and evaluation
- Research report using IMRD standard

### Out of Scope
- Using automl via frontend to run training (CLI/API only)
- Claiming the globally highest or mathematically optimal 2048 score
- Game UI development (headless simulation only)
- Model deployment as a web service
- Mobile or desktop application wrappers

## 4. Architecture Overview

```mermaid
flowchart TD
    subgraph "2048 ML System"
        subgraph Core Pipeline
            GE[Game Engine] -->|state| SE[State Encoder]
            SE -->|features| AM[Action Mapper]
        end
        
        AM -->|data| DC[Data Collector]
        DC -->|training data| TP[Training Pipeline]
        TP -->|model| MO[Model Output]
        
        subgraph "automl Engine (Rust)"
            TE[Train Engine]
            PE[Predict Engine]
            HE[HyperOptimize Engine]
        end
    end
    
    TP -.->|feedback| DC
    TE -->|trained model| MO
    PE -->|predictions| AM
    HE -.->|optimize| TE
    
    GE -->|observe| TE
    MO -.->|improve| GE
```

## 5. Technology Stack

| Component | Technology |
|-----------|-----------|
| AutoML Framework | automl v1.0.0 (Rust) |
| Language | Rust (primary); external comparison scripts may use Python if adopted |
| Game Engine | Custom Rust implementation |
| Data Format | CSV training data and JSON manifests (Polars CSV loading) |
| Training | automl TrainEngine |
| Optimization | HyperOptX; the root CLI currently tunes RandomForest/ExtraTrees `n_estimators` and `max_depth` against grouped-CV accuracy |
| Evaluation | Custom benchmarking suite |

## 6. Dependencies

- **automl submodule:** `https://github.com/Evintkoo/automl` pinned at `82d848323eed5e2af86d046d529916c448f2442c` (`v1.0.0-140-g82d8483`, published on `fix/deterministic-tie-breaking`) — verify with `git submodule status automl`
  - Path: `automl/`
  - Provides: TrainEngine, TrainingConfig, ModelType, HyperOptX, InferenceEngine

### 6.1 Automl Capability Verification (MANDATORY BEFORE TRAINING)

Because this project's goal #2 is to benchmark automl's capability, and the project depends on automl as a submodule, a **capability verification gate** is required before any training begins. This resolves the circular dependency:

**Verification Checklist** (must all pass before training):

1. **API completeness**: Verify `TrainEngine`, `HyperOptX`, `ModelType` enum, and `CrossValidator` exist and are functional in the automl submodule
2. **Model coverage**: Confirm at least 4 of the planned candidate algorithms (RandomForest, GradientBoosting, XGBoost, LightGBM) are available via `ModelType`
3. **MultiClassification task**: Verify `TaskType::MultiClassification` is supported with proper loss functions
4. **Hyperparameter optimization**: Confirm `HyperOptX` with TPE sampler and `MedianPruner` are functional
5. **Cross-validation**: Verify `GroupKFold` (group-preserving, **not** temporal) or equivalent exists; for true temporal forward-chaining use `TimeSeriesSplit` — `GroupKFold` keeps each `game_id` together but does not enforce temporal order (sort chronologically, `shuffle=false`)

**If verification fails:**
- Document which automl capabilities are missing in `plans/01-Infrastructure/01-Project/01-project-overview.md`
- If a required core capability is missing, stop the training milestone, record the missing capability, and revise the experiment scope. Do **not** add a local `smartcore`/`linfa` fallback: the core constraint is automl-only training.
- The project's goal #2 then evaluates **what automl CAN do**, not what it should have done

### 6.2 Verification Record (updated 2026-09-27)

The pinned submodule is present at `82d848323eed5e2af86d046d529916c448f2442c`, containing deterministic training/serialization fixes and deterministic tie handling for KNN and ExtraTrees. Source inspection confirms `TrainEngine`, `HyperOptX`, `ModelType`, `TaskType::MultiClassification`, `CVStrategy::GroupKFold`, `CVStrategy::TimeSeriesSplit`, and `MedianPruner::new(minimize: bool)` exist. The full AutoML library suite passes 712/712 on this revision.

The initial capability gate passed for the revised candidate set:

- `RandomForest`, `ExtraTrees`, `AdaBoost`, `KNN`, and `NaiveBayes` fit the four-action task and return four probability columns on the smoke dataset. `DecisionTree`, `LogisticRegression`, `SGD`, `SVM`, `GradientBoosting`, `XGBoost`, `LightGBM`, and `CatBoost` return two and are excluded from the 2048 candidate set until corrected and revalidated.
- `CrossValidator::split` supports group arrays, but `cross_val_score` always calls it with `groups=None`; therefore grouped CV cannot currently be used through that helper. `GroupKFold` sorts group IDs and assigns groups round-robin, so it preserves group separation but does not implement the plan's chronological `shuffle=false` behavior.
- `TrainEngine::fit` uses an internal train/validation split; it does not invoke `CrossValidator` or use `cv_folds`. The planned grouped validation must be performed explicitly through compatible APIs or implemented in the integration.
- The pinned AutoML library suite currently passes 712/712 tests. The AutoML CLI help smoke completed successfully. The earlier 709-test count is superseded by the current suite result.

Case-study training may use only the five verified multiclass variants above, after the dataset labeling, chronological split, and resource protocols are applied. This gate establishes API capability, not model quality or a winner.

### 6.3 Feature Protocol Decisions (2026-09-24)

The first feature encoder and randomized range checks exposed two underspecified formulas. The root implementation uses `max_tile_log = log2(max_tile)/15` for nonzero tiles (zero maps to zero), matching the documented `[0,1]` range through the planned 32768 canonical tile. It normalizes `adjacency_merge_score` as `sum_adjacent_equal_tile_values / (16 * 32768)`, because the listed `/16` alone can exceed 1. The `monotonicity` feature is currently a deterministic fraction of adjacent horizontal/vertical comparisons that are equal or contain an empty cell. This is a provisional operational definition; record it in feature plans and freeze before any training run. `score_normalized` is permitted above 1 when score exceeds one million, per the data schema.

### 6.4 Rollout Labeling Budget (updated 2026-09-27)

The first two-game throughput smoke produced 285 rows and 97,300 rollout evaluations in 82.23 seconds; its 228-hour estimate for 20,000 games was based on only two games. A subsequent 20-game pilot, retained under `reports/collection_pilots/2026-09-27-20-game/`, used global seed 90627, 100 rollouts per valid action, two threads, and per-game checkpoints. It produced 2,447 rows and 857,100 rollout evaluations in 857.36 seconds. Per-game rows ranged from 62 to 207 (mean 122.35, sample SD 40.74); mean elapsed time was 42.87 seconds/game. Linear extrapolation is 238.16 hours for 20,000 games and remains a rough, configuration-specific projection, not a commitment. This pilot improves the throughput sample but does not measure machine-to-machine variance or support a large-corpus quality claim. A declared compute envelope and a scale-appropriate collection protocol remain prerequisites.

### 6.5 Baseline and Evaluation Tooling (2026-09-24)

The root crate now exposes seeded `benchmark baseline --agent random|heuristic`, `benchmark run`, `benchmark report`, and `benchmark compare` workflows. Game-level CSV outputs have JSON manifests. Reports include distribution summaries, tail thresholds, and bootstrap mean intervals. Comparisons check seed sequences, pair matching seeds with an exact sign test, use Mann-Whitney U for unmatched samples, apply Holm correction across pairwise tests, and report bootstrap mean-difference intervals and Cohen's d. The paired sign test is conservative, ignores ties, and is not Wilcoxon; the method is named in outputs.

A 20-game seed-987 wiring sample yielded random mean 1,046.6 and heuristic mean 7,800.6. This is an implementation smoke measurement, far below the pre-registered 10,000-game protocol, and is not used as a baseline claim or populated in the results matrix.

The framework contribution remains partially evaluated. The [framework-validation report](../../../reports/framework_validation/README.md) records a seed-42, stratified 80/20 diagnostic across Iris, Wine, and Wisconsin Diagnostic with RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes. On pinned AutoML commit `82d848323eed5e2af86d046d529916c448f2442c`, two independent process runs succeeded for all 15 cases, matched predictions for 15/15 pairs, and passed model save/load equivalence. Two comparison-only scikit-learn 1.6.1 runs used those outer split rows and AutoML's per-class trailing 10% holdback; all 15 cases succeeded and repeated exactly, while predicted labels matched AutoML on 8/15 cases. A one-process resource probe observed AutoML at 1.33 seconds/27,426,816-byte maximum RSS and sklearn at 1.22 seconds/158,466,048 bytes on the same host. Different implementation defaults and process startup boundaries make these descriptive only. Matched search-budget/resource profiling, broader reproducibility, CLI/library equivalence, and independent replication remain open. Neither diagnostic completes framework validation or clears the main 2048 training milestone.

### 6.6 Main-Study Readiness (2026-09-26)

The root training command requires row-aligned game metadata, excludes the final chronological game groups from fitting, and runs explicit group-preserving CV on the development groups. On a 20-game pilot, it reserved the final 3 groups, ran five-fold grouped CV on the earlier groups (accuracy `0.2828 ± 0.0285`), and saved a 17-feature AutoML RandomForest with its input digests, dependency pin, seed derivations, and selected settings. A separate 20-game simulator smoke completed on seeds 91927–91946 (mean score 902.40; bootstrap interval `[712.60, 1095.20]`). These checks exercise data-to-fit-to-simulator wiring only. The final fit uses AutoML's internal row-level validation on development rows, and the chronological 3-game test groups do not yet receive classifier diagnostics. This is not a completed end-to-end research study.

The following prerequisites remain open before main-study claims: matched search-budget/per-model resource framework comparisons, broader repeated-fit reproducibility evidence, a scale-appropriate rollout-labeled corpus, diagnostics on held-out 2048 data, and a predeclared policy evaluation. The 20-game training/simulator smoke does not substitute for those artifacts. Its collection rate linearly projects to about 238 hours for 20,000 games, with 62–207 rows per game; both figures are uncertain and configuration-specific. Declare a resource envelope and collection protocol before starting a corpus at that scale.

**This verification is not optional.** Without it, the project cannot distinguish between "automl is incapable" and "our integration is broken."

## 7. Key Constraints

1. Must use **only** the automl module for ML training (no external ML libraries)
2. Must not use automl via frontend (CLI/API only)
3. All training must be reproducible (seed-based)
4. System must collect data for research validation

## 8. Success Criteria

### Tier 1: Minimum Viable Milestone
- AutoML capability and correctness gate completed
- Framework benchmark protocol documented
- Determine each model's score distribution through systematic evaluation under a declared, budgeted protocol
- Establish baseline scores for the verified four-class candidates: Random Forest, ExtraTrees, AdaBoost, KNN, and NaiveBayes
- Full data pipeline from game simulation to trained model works end-to-end
- Models are ranked by mean score

### Tier 2: Intermediate Milestone
- Framework architecture, data contracts, and design trade-offs documented
- Framework benchmarks completed on standard tabular datasets
- Rank all models by mean score across benchmark games
- Training pipeline is fully automated and reproducible
- Score-based comparison demonstrates clear differences between algorithms
- Case-study winner identified from held-out mean score with uncertainty and practical-effect reporting

### Tier 3: Advanced Milestone
- Framework results validated on standard tabular datasets
- Framework performance and reproducibility compared with established implementations
- The top-ranked model is compared with the heuristic baseline using held-out scores and the declared uncertainty protocol
- Benchmark results demonstrate automl capability on sequential decision-making
- Statistical tests confirm score superiority over all baselines

### Tier 4: Stretch Goal
- The framework and 2048 pipeline are independently reproduced or validated by a second implementation
- Framework architecture demonstrates a measurable advantage or clearly characterized trade-off over established alternatives
- Statistical evidence shows one algorithm significantly outperforms all others
- Results are publishable as research findings

### 2048 Case-Study Winner Determination
- **Case-study winner** = model with the highest held-out mean score under the declared evaluation protocol
- Ranking is based on mean score (primary), with median score and score consistency as tiebreakers
- The primary comparison uses the pre-registered method: exact sign test for matched seeds or Mann-Whitney U for unmatched samples, with Holm correction
- Bootstrap 95% CIs and effect sizes must be reported for practical interpretation
- Benchmark results demonstrate meaningful automl capability on sequential games
- Comprehensive IMRD research report published with validated findings

### Theoretical Limit Definition
The theoretical maximum score for 2048 is not used as an optimization target. **Models are ranked by mean game score under the predefined evaluation protocol, not by proximity to a theoretical limit.** The heuristic agent serves as a practical baseline; its measured mean depends on the game protocol and should be taken from retained benchmark artifacts rather than a fixed threshold.

### Non-Negotiable Criteria (All Tiers)
- Models are ranked by held-out game score with uncertainty and practical-effect reporting
- The case-study winner is the model with the highest held-out mean score; this does not define framework success
- Statistical comparison completed using the pre-registered Mann-Whitney U/Holm protocol
- Bootstrap 95% CI and effect size reported; neither is an automatic exclusion gate
- Best model identified and documented with strong statistical evidence
- Research findings contribute to understanding of automl on sequential decision-making

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/01-Project/01-project-overview.md` exits 0.

## Open questions

- **The evidence remains bounded by diagnostic runs.** Two AutoML runs on pinned `82d8483` succeeded for 15 standard-dataset cases, matched predictions 15/15, and passed save/load equivalence. Two fixed-configuration sklearn runs also repeated all 15 cases, with label agreement on 8/15. A 20-game canonical-schema 2048 pilot trained one policy and completed a separate 20-game simulator smoke, but it does not establish model quality. Matched-budget/per-model framework profiling, broader repeated-fit evidence, independent replication, scale collection, and held-out 2048 diagnostics remain absent. Larger runs require a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
