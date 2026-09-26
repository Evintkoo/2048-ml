# Plan 03 — Pruning Strategy: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** The AutoML submodule exposes pruner types, but the root HyperOptX objective has no intermediate reporting or pruning integration.

**Goal:** State the current implementation and evidence boundary for pruning strategy.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats pruning integration as pending.** The root objective currently computes grouped-CV accuracy and returns one completed scalar per trial. Its `OptimizationConfig.pruning` is disabled, and no comparable intermediate fold/resource report is connected to a pruner. The examples below describe framework APIs, not active root behavior.

## 1. Purpose

Record the prerequisite for future trial pruning and the boundary of the current framework API.

## 2. Pruning Overview

Pruning stops unpromising trials early to save computational resources and focus on the most promising hyperparameter configurations.

```mermaid
flowchart TD
    subgraph "Pruning Strategy"
        Start[Start Trial]
        Start --> Run[Trial Running]
        Run --> Check{Check Intermediate<br/>Results}
        Check --> |Poor| Prune[Prune Trial]
        Check --> |Good| Continue[Continue Trial]
        Prune --> Release[Release Resources]
        Continue --> Report[Report Results]
        Release --> Next{Trial Queue<br/>Empty?}
        Next --> |No| Run
        Next --> |Yes| Complete[All Trials Complete]
    end
    
    Check --> |Good| Continue
```

## 3. Pruner Types

```mermaid
flowchart TB
    subgraph "Available Pruners"
        A[Median Pruner]
        B[Percentile Pruner]
        C[Hyperband Pruner]
        D[ASHT Pruner]
    end
    
    A --> |Median threshold| Decision[Prune Decision]
    B --> |Percentile threshold| Decision
    C --> |Resource allocation| Decision
    D --> |Adaptive halving| Decision
    
    style MedianPruner fill:#e3f2fd
    style Hyperband fill:#e8f5e9
```

### 3.1 Median Pruner

```mermaid
flowchart TD
    Trial[Trial Running]
    Trial --> Measure[Measure Intermediate Result]
    Measure --> Compare{Compare to Median<br/>of Running Trials}
    Compare --> |Below Median| Prune[Prune Trial]
    Compare --> |Above Median| Keep[Keep Trial]
    
    style Prune fill:#ffcdd2
    style Keep fill:#c8e6c9
```

### 3.2 Hyperband Pruner

```mermaid
flowchart TD
    Start[Start]
    Start --> Config[Define Configurations]
    Config --> Run[Run with Successive<br/>Halving]
    Run --> Evaluate[Evaluate Performance]
    Evaluate --> |Top Third| Advance[Advance to Next<br/>Resource Level]
    Evaluate --> |Bottom Third| Stop[Stop Trial]
    Advance --> |Final Round| Select[Select Best]
    
    style Advance fill:#e8f5e9
    style Stop fill:#ffcdd2
```

## 4. Pruning Configuration

```rust
use automl::{MedianPruner, PercentilePruner, HyperbandPruner};

// Median Pruner — requires minimize flag
let median_pruner = MedianPruner::new(false) // false = maximize, true = minimize
    .with_n_startup_trials(5)
    .with_n_warmup_steps(5);

// Percentile Pruner — new(percentile, minimize)
let percentile_pruner = PercentilePruner::new(25.0, false);

// Hyperband Pruner — new(min_resource, max_resource, minimize)
let hyperband_pruner = HyperbandPruner::new(1, 81, false)
    .with_reduction_factor(3.0);
```

## 5. Pruning Workflow

```mermaid
flowchart TD
    subgraph "Pruning Workflow"
        Initialize[Initialize Trials]
        Initialize --> Distribute[Distribute Resources]
        Distribute --> Evaluate[Evaluate Performance]
        Evaluate --> Prune{Prune Based on<br/>Strategy}
        Prune --> |Prune| Release[Release Resources]
        Prune --> |Keep| Continue[Continue Training]
        Release --> Reallocate[Reallocate Resources]
        Continue --> Reallocate
        Reallocate --> |More Trials| Initialize
        Reallocate --> |Done| Final[Final Results]
    end
```

## 6. Pruning Decision Logic

```mermaid
flowchart TD
    Decision{Pruning Decision}
    Decision --> |Trial Age > Min| Check[Check Intermediate Result]
    Decision --> |Trial Age ≤ Min| Keep[Keep Trial]
    Check --> Compare{Result > Threshold?}
    Compare --> |Yes| Keep
    Compare --> |No| Prune[Prune Trial]
    
    style Keep fill:#c8e6c9
    style Prune fill:#ffcdd2
```

## 7. Resource Allocation

```mermaid
flowchart TB
    Total[Total Resources]
    Total --> Initial[Initial Allocation<br/>All Trials]
    Initial --> First[First Halving]
    First --> |50% kept| Second[Second Halving]
    Second --> |33% kept| Third[Third Halving]
    Third --> |25% kept| Final[Final Selection]
    
    style Final fill:#e8f5e9
```

## 8. Pruning Strategy Files

All pruning strategy files are in `05-Model/03-Hyperparameter-Optimization/`:

```mermaid
flowchart LR
    Dir[05-Model/03-Hyperparameter-Optimization]
    Dir --> N01[01-hyperparameter-search.md]
    Dir --> N02[02-search-space.md]
    Dir --> N03[03-pruning-strategy.md]
```

## 9. Next Steps

1. Execute hyperparameter search with pruning
2. Collect best configuration
3. Proceed to model evaluation

## Implementation Record

- The pinned framework exposes `Pruner` types with `report` and `should_prune` methods. The root optimizer callback receives no trial context or reporter, and the current grouped-CV helper returns a final aggregate. The root explicitly disables pruning; no trial is pruned or evaluated early.

---

## Verification (definition of done)

1. `test -f plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
6. `grep -q '^## Open questions$' plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
7. `grep -q '^## Later$' plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md` exits 0.

## Open questions

- A future implementation must define a comparable intermediate step (for example, each group fold), assign stable trial IDs, retain prune reasons, and verify that pruning does not touch held-out test games.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
