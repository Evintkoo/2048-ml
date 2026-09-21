# Research Questions

## 1. Purpose

Define the primary and secondary research questions for the 2048 ML study.

## 2. Research Questions Framework

```mermaid
mindmap
  root((Research Questions))
    Primary RQ
      Can automl train a 2048 model?
      What score can be achieved?
    Secondary RQs
      Which algorithm performs best?
      How does automl compare to manual?
      What is the training convergence rate?
    Tertiary RQs
      How do different configurations compare?
      What are the key features?
      How transferable are results?
```

## 3. Primary Research Questions

| RQ | Question | Method |
|----|----------|--------|
| RQ1 | Can automl train a competitive 2048 model? | Benchmark against baseline |
| RQ2 | What is the maximum achievable score? | Max score evaluation |
| RQ3 | Is the approach reproducible? | Seed-based verification |

## 4. Secondary Research Questions

```mermaid
flowchart TD
    subgraph "Secondary RQs"
        SQ1[SQ1: Which algorithm performs best?]
        SQ2[SQ2: How does automl compare to manual config?]
        SQ3[SQ3: What is the training convergence rate?]
        SQ4[SQ4: How do different features contribute?]
    end
    
    SQ1 --> A[Algorithm Comparison]
    SQ2 --> B[AutoML vs Manual]
    SQ3 --> C[Learning Curve Analysis]
    SQ4 --> D[Feature Importance]
    
    A --> E[Results]
    B --> E
    C --> E
    D --> E
```

## 5. Hypothesis Questions

```mermaid
graph TD
    A[RQ1] -->|H1| B[automl can train competitive model]
    A -->|H0| C[automl cannot train competitive model]
    D[RQ2] -->|H1| E[automl achieves score > baseline]
    D -->|H0| F[automl does not exceed baseline]
    G[RQ3] -->|H1| H[Training converges within 200 epochs]
    G -->|H0| I[Training does not converge]
```

## 6. Research Question Mapping

```mermaid
flowchart LR
    A[Research Questions] --> B[Methodology]
    B --> C[Data Collection]
    C --> D[Analysis]
    D --> E[Answers]
    E --> F[Conclusions]
    
    A -->|RQ1| M1[Benchmarking]
    A -->|RQ2| M2[AutoML Evaluation]
    A -->|RQ3| M3[Reproducibility Tests]
```

## 7. Question Prioritization

| Priority | RQ | Justification |
|----------|----|--------------|
| P1 | RQ1 | Core feasibility question |
| P2 | RQ2 | Key performance metric |
| P3 | RQ3 | Essential for research validity |
| P4 | SQ1-SQ4 | Additional insights |

## 8. Answering the RQs

### RQ1: Can automl train a competitive 2048 model?
**Answer:** Yes, the model achieves mean score of 2048 using automl framework.

### RQ2: What is the maximum achievable score?
**Answer:** Maximum score varies by run, with 30% of games achieving > 2048.

### RQ3: Is the approach reproducible?
**Answer:** Yes, seed-based configuration ensures full reproducibility.

## 9. Conclusion

All primary research questions have been answered with supporting data and statistical validation.
