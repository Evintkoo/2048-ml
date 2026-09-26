# Plan 01 — Hyperparameter Search: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Versioned HyperOptX search is wired for RandomForest/ExtraTrees over grouped-CV accuracy; pruning and broader candidates remain unsupported.

**Goal:** State the current implementation and evidence boundary for hyperparameter search.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats grouped-CV search integration as implemented with bounded scope.** The `train` command reads schema-v1 JSON or `--tune-trials`, searches `n_estimators` and `max_depth` for RandomForest or ExtraTrees, evaluates root grouped-CV accuracy, and saves the study. Trials run serially with pruning disabled. This is exploratory model tuning; no general AutoML claim follows.

## 1. Purpose

Define the hyperparameter search strategy for optimizing the 2048 game machine learning model using the automl framework's HyperOptX optimizer.

## 2. Search Overview

The search explores two integer parameters for the supported candidates. Its objective is grouped-CV accuracy, and it does not establish a globally optimal configuration.

```mermaid
flowchart TD
    subgraph "Hyperparameter Search"
        Start[Initialize Search]
        Start --> Define[Define Search Space]
        Define --> Select[Select Sampler]
        Select --> Iterate[Iterate Trials]
        Iterate --> Evaluate[Complete grouped-CV objective]
        Evaluate --> Next{More Trials?}
        Next -->|Yes| Iterate
        Next -->|No| Best[Select Best Config]
    end
    
    Start --> |TPE sampler| Define
    Define --> |declared trial count| Select
    Best --> |Best Params| Model[Final AutoML Fit]
```

Each objective returns one completed grouped-CV accuracy value. Intermediate pruning is not wired; the root explicitly disables the framework's separate pruner flag.

## 3. Search Space Definition

```mermaid
flowchart TD
    Space[Search Space]
    Space --> Supported[RandomForest / ExtraTrees]
    Supported --> NEstimators[n_estimators: configured positive integer range]
    Supported --> MaxDepth[max_depth: configured positive integer range]
```

## 4. Optimization Strategy

```mermaid
flowchart TD
    Opt[Optimization Strategy]
    Opt[Root CLI strategy] --> TPE[TPE sampler]
    TPE --> Sample[Sample n_estimators and max_depth]
    Sample --> CV[Grouped-CV accuracy objective]
```

## 5. Pruning Strategy

```mermaid
flowchart TD
    Trial[Trial Running]
    Trial --> Objective[Compute complete grouped-CV score]
    Objective --> Record[Record scalar trial result]
    Unsupported[No intermediate-reporting hook] --> Disabled[Root sets pruning=false]
```

## 6. Search Configuration

```rust
use automl::optimizer::{OptimizationConfig, SearchSpace, OptimizeDirection, SamplerType};

let mut opt_config = OptimizationConfig::default()
    .with_direction(OptimizeDirection::Maximize)
    .with_n_trials(100)
    .with_sampler(SamplerType::TPE)
    .with_metric("grouped_cv_accuracy");
opt_config.n_jobs = 1;       // HyperOptX objective calls are currently serial
opt_config.pruning = false;  // no intermediate objective-reporting hook

let search_space = SearchSpace::new()
    .int("n_estimators", 50, 300)
    .int("max_depth", 3, 10);
```

## 7. Search Pipeline

```mermaid
flowchart TD
    Pipeline[One declared TPE study]
    Pipeline --> Objective[Grouped-CV accuracy per trial]
    Objective --> Best[Best parameters for final fit]
```

## 8. Search Files Location

All hyperparameter search files are in `05-Model/03-Hyperparameter-Optimization/`:

```mermaid
flowchart LR
    Dir[05-Model/03-Hyperparameter-Optimization]
    Dir --> N01[01-hyperparameter-search.md]
    Dir --> N02[02-search-space.md]
    Dir --> N03[03-pruning-strategy.md]
```

## 9. Next Steps

The root integration maps sampled `n_estimators` and `max_depth` into grouped-CV fitting for RandomForest and ExtraTrees. The optimizer objective returns a completed scalar score, so intermediate reporting/pruning is unavailable; broader model-specific parameters are not wired.

1. Preserve the exact search configuration, seed, grouped-CV score, and study artifact for each run.
2. Expand to other candidates only with supported model-specific parameters and probability contracts.
3. Add pruning only when trials expose comparable intermediate metrics.

## Implementation Record

- HyperOptX trains grouped-CV objectives for RandomForest/ExtraTrees and applies the selected integer parameters to final fitting. It saves a study artifact and records configuration/seed data in the manifest.
- Pruning is disabled because there is no intermediate-reporting hook. The pinned optimizer marks objective errors as pruned trials with a worst-value score but does not retain the error text in `TrialResult`; trial failure diagnostics remain limited. No best configuration is claimed beyond each exploratory run.

---

## Verification (definition of done)

1. `test -f plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
6. `grep -q '^## Open questions$' plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
7. `grep -q '^## Later$' plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/05-Model/03-Hyperparameter-Optimization/01-hyperparameter-search.md` exits 0.

## Open questions

- Search is limited to two integer parameters and two candidate families. HyperOptX converts objective errors into pruned results but does not retain their error text. Pruning, broader candidate coverage, and full performance studies remain future work.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
