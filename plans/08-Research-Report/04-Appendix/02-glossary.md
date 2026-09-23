# Plan 02 — Glossary: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for glossary.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Define key terms and concepts used throughout the 2048 ML research. This glossary includes game terminology, ML terminology, automl framework terms, evaluation terms, and statistical terms.

## 2. Terminology Framework

```mermaid
mindmap
  root((Glossary))
    Game Terms
      2048, Grid, Tile, Merge, Score
    ML Terms
      Model, Training, Inference, Features
    automl Terms
      TrainEngine, HyperOptX, ModelType
    Evaluation Terms
      Mean Score, Ranking, Significance
    Statistical Terms
      p-value, Confidence Interval, Effect Size
```

## 3. Game Terminology

| Term | Definition |
|------|------------|
| 2048 | Tile-matching puzzle game where the goal is to maximize score by merging tiles |
| Grid | 4×4 board for gameplay |
| Tile | Game piece with power-of-2 value |
| Merge | Combine two equal tiles into a higher-value tile |
| Move | Slide operation (up/down/left/right) |
| Score | Sum of all merge values during a game |
| Game Over | Board is full with no valid moves |
| Heuristic Baseline | Agent using weighted evaluation function (mean score ~512) |
| Random Baseline | Agent selecting moves uniformly at random (mean score ~128) |
| Case-study winner | Model with the highest held-out mean score under the declared 2048 protocol; not a globally optimal policy |

## 4. ML Terminology

| Term | Definition |
|------|------------|
| Model | Trained supervised classifier (e.g., Random Forest, Gradient Boosting, XGBoost) |
| Training | Model learning process from labeled data |
| Inference | Model prediction on new game states |
| Features | Input data representation (27-dimensional feature vector) |
| Labels | Target output values (optimal action determined by rollout simulation) |
| Loss | Prediction error metric (cross-entropy) |
| Convergence | Training stabilization (diminishing returns pattern) |
| Hyperparameters | Configuration parameters tuned by HyperOptX |
| Feature Vector | 27-dimensional representation of board state |

## 5. automl Framework Terms

| Term | Definition |
|------|------------|
| TrainEngine | Core training component from Evintkoo/automl |
| HyperOptX | Hyperparameter optimization engine (TPE sampler) |
| TrainingConfig | Model configuration settings |
| ModelType | Architecture specification (RF, GB, XGBoost, etc.) |
| SearchSpace | Range of hyperparameters for optimization |
| CrossValidator | `automl::training::CrossValidator` with `CVStrategy::GroupKFold { n_splits:5 }` on `game_id` (verified in `automl/src/training/cross_validation.rs` — not temporal; TimeSeriesSplit exists but is not canonical for i.i.d. games) |

## 6. Evaluation Terms

| Term | Definition |
|------|------------|
| Benchmark | Standard for comparison (heuristic agent, random agent) |
| Baseline | Reference point for evaluation (random ~128, heuristic ~512) |
| Metric | Quantitative measurement (mean score, median score, std dev) |
| Significance | Statistical importance (p < 0.05 after Bonferroni correction) |
| Reproducibility | Consistent results with fixed seed |
| Winner Determination | Model ranked #1 by mean score across ≥10,000 games |
| Mean Score | Primary 2048 case-study metric; framework validation uses task-appropriate quality and resource metrics |
| Bootstrap CI | 95% confidence interval via resampling |
| Effect Size | Magnitude of difference (Cohen's d ≥ 0.5) |

## 7. Statistical Terms

| Term | Definition |
|------|------------|
| p-value | Probability of observing results under null hypothesis |
| Confidence Interval | Range of true value estimate (95% bootstrap CI) |
| Effect Size | Magnitude of difference (Cohen's d) |
| Standard Deviation | Data spread measure |
| Wilcoxon | Paired non-parametric significance test |
| Mann-Whitney U | Two-group non-parametric comparison |
| Kruskal-Wallis | Multi-group non-parametric comparison |
| Bonferroni | Multiple comparison correction |
| Holm-Bonferroni | Less conservative multiple comparison correction |
| Power | Probability of detecting a true effect (≥0.8) |

## 8. Theoretical Terms

| Term | Definition |
|------|------------|
| POMDP | Partially observable Markov decision process |
| MDP | Markov decision process |
| PSPACE-hard | Computationally intractable problem class |
| Markov Blanket | Minimal feature subset sufficient for prediction |
| PAC-learning | Probably approximately correct learning framework |
| VC-dimension | Measure of hypothesis class complexity |
| Mutual Information | Information shared between two variables |
| Entropy | Measure of uncertainty in a system |
| Bellman Equation | Recursive equation for value function |

## 9. Rust-Specific Terms

| Term | Definition |
|------|------------|
| Option<T> | Nullable type |
| Result<T,E> | Error handling type |
| trait | Interface definition |
| impl | Implementation block |
| generics | Parametric polymorphism |
| lifetime | Ownership scope annotation |
| borrow | Reference without ownership |
| move | Transfer of ownership |

## 10. Quick Reference

```mermaid
graph TD
    A[2048 Game] -->|goal: maximize mean score| B[Winner by Ranking]
    B -->|achieved by| C[ML Model]
    C -->|trained by| D[automl]
    D -->|uses| E[TrainEngine]
    E -->|optimized by| F[HyperOptX]
    F -->|validated by| G[Mann-Whitney U Test]
    G -->|confirmed by| H[Bootstrap CI]
    H -->|ranked by| I[Mean Score]
    
    style B fill:#9f9,stroke:#333
    style I fill:#9f9,stroke:#333
```

## 11. Reference to Theoretical Framework

For detailed theoretical foundations, see `00-theoretical-framework.md` which includes:
- POMDP formulation of 2048
- PSPACE-hardness proof
- Markov blanket analysis
- PAC-learning bounds
- Information-theoretic analysis
- Game-theoretic framing

## Implementation Record

- Glossary is a documentation aid only. Terms that imply established PSPACE, Markov-blanket, PAC, or information-theoretic results must remain qualified as conjectural or contextual pending proof/source verification.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/02-glossary.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
