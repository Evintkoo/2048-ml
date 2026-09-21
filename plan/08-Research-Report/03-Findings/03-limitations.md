# Limitations

## 1. Purpose

Document the limitations and constraints of the 2048 ML research study.

## 2. Limitations Framework

```mermaid
flowchart TD
    subgraph "Limitations"
        subgraph "Technical"
            A[Computational Constraints]
            B[automl Framework Limits]
            C[Rust Implementation Bugs]
        end
        
        subgraph "Methodological"
            D[Limited Game Variants]
            E[Small Sample Size]
            F[Single Seed Dependency]
        end
        
        subgraph "External"
            G[Time Constraints]
            H[Resource Availability]
            I[Domain Specificity]
        end
    end
```

## 3. Technical Limitations

| Limitation | Impact | Mitigation |
|-----------|--------|------------|
| Training time | 2 hours per model | Parallel training |
| Memory usage | High for large models | Model compression |
| automl constraints | Limited model types | Feature requests |
| Rust bugs | Potential data corruption | Extensive testing |

## 4. Methodological Limitations

```mermaid
graph TD
    A[Single Game Domain] -->|2048 only| B[Limited Generalizability]
    C[Fixed Evaluation] -->|10000 games| D[Potential Variance]
    E[Deterministic Seed] -->|Not all seeds tested| F[Possible Bias]
    G[No Human Evaluation] -->|Computer only| H[Missing Human Perspective]
```

## 5. Computational Constraints

```mermaid
flowchart LR
    A[Training Request] --> B{Resources Available?}
    B -->|Yes| C[Proceed with Training]
    B -->|No| D[Queue or Reduce Scope]
    C --> E[Train Model]
    D --> F[Adjust Parameters]
    F --> E
```

## 6. Data Limitations

- **Limited game variants:** Only standard 4×4 2048 tested
- **No external data:** All data from game simulation
- **Fixed evaluation criteria:** May not capture all performance aspects
- **Sample size:** 10,000 games may not cover all edge cases

## 7. Framework Limitations

```mermaid
graph TD
    A[automl v1.0.0] --> B[Model Type Support]
    A --> C[Hyperparameter Search]
    A --> D[Training Config Options]
    B -->|Limited| E[No custom architectures]
    C -->|TPE, Grid, Random| F[No Bayesian alternatives]
    D -->|Fixed structure| G[Less flexibility]
```

## 8. Scope Limitations

```mermaid
mindmap
  root((Scope Limitations))
    In Scope
      Standard 2048
      automl framework
      Rust implementation
    Out of Scope
      2048 variants (8×8, etc.)
      Other games
      Custom ML architectures
      Reinforcement learning
      Deep learning models
      Multi-agent scenarios
```

## 9. Mitigation Strategies

| Limitation | Mitigation | Status |
|-----------|------------|--------|
| Training time | Parallel execution | Planned |
| Single seed | Multi-seed validation | Planned |
| Limited variants | Expand to 8×8 | Future work |
| automl constraints | Contribute features | Open issue |

## 10. Honest Assessment

These limitations do not invalidate the core findings but should be considered:
1. Results are specific to the 2048 game domain
2. Generalizability to other games is untested
3. Computational constraints may affect optimal model selection
4. Further research is needed to validate findings

## 11. Conclusion

Despite limitations, the research provides valuable insights into automl effectiveness for game AI. Future work should address these limitations for a more comprehensive understanding.
