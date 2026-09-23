# Plan 01 — Hyperparameter Search: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for hyperparameter search.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Define the hyperparameter search strategy for optimizing the 2048 game machine learning model using the automl framework's HyperOptX optimizer.

## 2. Search Overview

The hyperparameter search explores the parameter space to find optimal configurations for the selected model.

```mermaid
flowchart TD
    subgraph "Hyperparameter Search"
        Start[Initialize Search]
        Start --> Define[Define Search Space]
        Define --> Select[Select Sampler]
        Select --> Iterate[Iterate Trials]
        Iterate --> Evaluate[Evaluate Trial]
        Evaluate --> Prune{Prune?}
        Prune -->|Yes| Skip[Skip Trial]
        Prune -->|No| Continue[Continue Trial]
        Continue --> Next{More Trials?}
        Next -->|Yes| Iterate
        Next -->|No| Best[Select Best Config]
        Skip --> Next
    end
    
    Start --> |TPE Sampler| Define
    Define --> |100 trials| Select
    Best --> |Best Params| Model[Trained Model]
```

## 3. Search Space Definition

```mermaid
flowchart TD
    Space[Search Space]
    Space --> Tree[Tree-Based Models]
    Space --> Linear[Linear Models]
    
    Tree --> NEstimators[n_estimators: 50-300]
    Tree --> MaxDepth[max_depth: 3-10]
    Tree --> LR[learning_rate: 0.01-0.5]
    Tree --> Subsample[subsample: 0.5-1.0]
    Tree --> Colsample[colsamples_bytree: 0.5-1.0]
    
    Linear --> C[Regularization C]
    Linear --> Penalty[Penalty Type]
```

## 4. Optimization Strategy

```mermaid
flowchart TD
    Opt[Optimization Strategy]
    Opt --> TPE[TPE Sampler]
    Opt --> Random[Random Search]
    Opt --> Grid[Grid Search]
    
    TPE --> |Bayesian| Evolve[Evolve Search]
    Random --> |Exploration| Sample[Random Sampling]
    Grid --> |Exhaustive| Enumerate[Enumerate All]
    
    style TPE fill:#e3f2fd
    style Random fill:#fff3e0
    style Grid fill:#f3e5f5
```

## 5. Pruning Strategy

```mermaid
flowchart TD
    Trial[Trial Running]
    Trial --> Monitor[Monitor Intermediate Results]
    Monitor --> Pruner{Pruner Decision}
    Pruner --> |Median| Prune[Prune Trial]
    Pruner --> |Keep| Continue[Continue Trial]
    Prune --> Release[Release Resources]
    Continue --> Report[Report Results]
    
    style Prune fill:#ffcdd2
    style Continue fill:#c8e6c9
```

## 6. Search Configuration

```rust
use automl::{OptimizationConfig, SearchSpace, Parameter, ParameterType, OptimizeDirection, MedianPruner};

let opt_config = OptimizationConfig::default()
    .with_direction(OptimizeDirection::Maximize)
    .with_n_trials(100)
    .with_n_jobs(4);
// Pruner is separate — MedianPruner::new requires minimize flag
let pruner = MedianPruner::new(false); // false = maximize (true = minimize)

let mut search_space = SearchSpace::new();
search_space.add(Parameter::new("n_estimators", ParameterType::Int(50, 300)));
search_space.add(Parameter::new("max_depth", ParameterType::Int(3, 10)));
search_space.add(Parameter::new("learning_rate", ParameterType::Float(0.01, 0.5)));
search_space.add(Parameter::new("subsample", ParameterType::Float(0.5, 1.0)));
search_space.add(Parameter::new("colsample_bytree", ParameterType::Float(0.5, 1.0)));
```

## 7. Search Pipeline

```mermaid
flowchart TD
    Pipeline[Hyperparameter Search Pipeline]
    Pipeline --> Stage1[Stage 1: Coarse Search]
    Stage1 --> Stage2[Stage 2: Fine Search]
    Stage2 --> Stage3[Stage 3: Precision Search]
    
    Stage1 --> |Wide Range| Stage2
    Stage2 --> |Narrow Range| Stage3
    Stage3 --> Best[Best Configuration]
    
    style Stage1 fill:#e3f2fd
    style Stage3 fill:#e8f5e9
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

The framework TPE optimizer API has a synthetic objective smoke check, but no 2048 trial currently trains a candidate or maps trial parameters into `TrainingConfig`. The pruning plan assumes intermediate scores, while `TrainEngine.fit` does not expose iterative reporting for the supported tree candidates; pruning must not be claimed until such a resource step exists.

1. Map supported, model-specific search parameters into `TrainingConfig`.
2. Evaluate each trial on fixed game-group folds without touching final test games.
3. Record trial config, seed, fold scores, elapsed time, and failures; only add pruning when trials provide comparable intermediate metrics.

## Implementation Record

- No 2048 HyperOptX trial currently trains a candidate or maps sampled values to `TrainingConfig`. The existing framework API smoke check is not a model search; no best configuration is claimed.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
