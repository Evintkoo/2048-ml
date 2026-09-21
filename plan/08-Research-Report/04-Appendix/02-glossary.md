# Glossary

## 1. Purpose

Define key terms and concepts used throughout the 2048 ML research.

## 2. Terminology Framework

```mermaid
mindmap
  root((Glossary))
    Game Terms
      2048, Grid, Tile, Merge
    ML Terms
      Model, Training, Inference
    automl Terms
      TrainEngine, HyperOptX, Config
    Evaluation Terms
      Benchmark, Metric, Baseline
```

## 3. Game Terminology

| Term | Definition |
|------|------------|
| 2048 | Tile-matching puzzle game |
| Grid | 4×4 board for gameplay |
| Tile | Game piece with power-of-2 value |
| Merge | Combine two equal tiles |
| Move | Slide operation (up/down/left/right) |
| Score | Sum of all merge values |
| Game Over | Board is full with no valid moves |

## 4. ML Terminology

| Term | Definition |
|------|------------|
| Model | Trained neural network |
| Training | Model learning process |
| Inference | Model prediction |
| Features | Input data representation |
| Labels | Target output values |
| Loss | Prediction error metric |
| Convergence | Training stabilization |

## 5. automl Framework Terms

| Term | Definition |
|------|------------|
| TrainEngine | Core training component |
| HyperOptX | Hyperparameter optimization engine |
| TrainingConfig | Model configuration settings |
| ModelType | Architecture specification |
| SearchSpace | Range of hyperparameters |

## 6. Evaluation Terms

| Term | Definition |
|------|------------|
| Benchmark | Standard for comparison |
| Baseline | Reference point for evaluation |
| Metric | Quantitative measurement |
| Significance | Statistical importance |
| Reproducibility | Consistent results |

## 7. Statistical Terms

| Term | Definition |
|------|------------|
| p-value | Probability of null hypothesis |
| Confidence Interval | Range of true value estimate |
| Effect Size | Magnitude of difference |
| Standard Deviation | Data spread measure |
| t-test | Statistical significance test |

## 8. Rust-Specific Terms

| Term | Definition |
|------|------------|
| Option<T> | Nullable type |
| Result<T,E> | Error handling type |
| trait | Interface definition |
| impl | Implementation block |
| generics | Parametric polymorphism |

## 9. Quick Reference

```mermaid
graph TD
    A[2048 Game] -->|played on| B[4×4 Grid]
    B -->|tiles have| C[Power of 2 Values]
    C -->|merge to| D[Higher Values]
    D -->|goal is| E[Reach 2048]
    E -->|ML predicts| F[Best Move]
    F -->|model trained by| G[automl]
    G -->|uses| H[TrainEngine]
    H -->|optimized by| I[HyperOptX]
```
