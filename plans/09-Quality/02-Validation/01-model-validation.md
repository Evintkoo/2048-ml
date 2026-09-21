# Model Validation

## 1. Purpose

Define model validation procedures for the 2048 ML system.

## 2. Model Validation Framework

```mermaid
flowchart TD
    subgraph "Model Validation"
        subgraph "Validation Steps"
            VS1[Data Validation]
            VS2[Architecture Validation]
            VS3[Performance Validation]
            VS4[Robustness Validation]
        end
        
        subgraph "Metrics"
            M1[Accuracy Metrics]
            M2[Performance Metrics]
            M3[Statistical Metrics]
        end
        
        subgraph "Output"
            R[Validation Report]
            Q[Quality Score]
        end
        
        VS1 --> M1
        VS2 --> M1
        VS3 --> M2
        VS4 --> M3
        M1 --> R
        M2 --> R
        M3 --> R
        R --> Q
    end
```

## 3. Validation Pipeline

```mermaid
flowchart TD
    A[Load Trained Model] --> B[Validate Architecture]
    B --> C[Run Validation Games]
    C --> D[Compute Metrics]
    D --> E{Passes Thresholds?}
    E -->|Yes| F[Model Validated]
    E -->|No| G[Model Rejected]
    F --> H[Deploy]
    G --> I[Return to Training]
```

## 4. Validation Metrics — Canonical Thresholds

| Metric | Threshold | Method | Gate? |
|--------|-----------|--------|-------|
| Mean Game Score | ≥ 512 (beats heuristic ~512) | Statistical evaluation (Mann-Whitney U, Bonferroni) | **Primary gate — rank by mean score** |
| Valid-Action Accuracy | ≥ 60% | Classification accuracy on valid actions only (0–3) | **Gate** |
| F1 Macro | ≥ 0.55 | Macro-averaged F1 across 4 action classes | **Gate** |
| Robustness | Std Dev ≤ 512 | Variance analysis | Informative |

> **No proximity gate.** Models are **ranked by Mean Game Score** (highest wins). If proximity is reported as optional analysis, define as **single value** `proximity = model_mean_score / heuristic_baseline_mean (≈512)` (e.g., 768 → 1.5×). Do not use `≥1.5` proximity as pipeline rejection gate; `≥0.3/>0.5` deprecated and removed. No `model/theoretical_max` ratio — theoretical max is open.

## 5. Cross-Validation

```mermaid
graph TD
    A[Full Dataset] --> B[Group K-Fold by Game ID]
    B --> C[Train on Games 1-80]
    C --> D[Test on Games 81-84]
    D --> E{All Folds Done?}
    E -->|No| C
    E -->|Yes| F[Average Metrics]
    F --> G[Validate Model]
    
    style G fill:#9f9,stroke:#333
```

## 6. Model Validation Tests

```mermaid
flowchart TD
    A[Test 1: Architecture] --> B[Model Structure Valid?]
    B --> C[Test 2: Data Flow]
    C --> D[Data Flows Correctly?]
    D --> E[Test 3: Prediction]
    E --> F[Predictions Valid?]
    F --> G[Test 4: Performance]
    G --> H[Meets Thresholds?]
    H --> I[Validation Complete]
```

## 7. Validation Scoring

```mermaid
graph TD
    A[Data Quality] -->|score| T1[0-100]
    B[Architecture] -->|score| T1
    C[Performance] -->|score| T1
    D[Robustness] -->|score| T1
    E[Reproducibility] -->|score| T1
    T1 --> F[Overall Validation Score]
    F --> G{≥ 80?}
    G -->|Yes| H[Approved]
    G -->|No| I[Needs Improvement]
```

## 8. Model Comparison Validation

```mermaid
flowchart LR
    A[Model A] --> B[Validate]
    C[Model B] --> B
    D[Model C] --> B
    B --> E[Ranked Validated Models]
    E --> F[Best Model Selected]
```

## 9. Validation Artifacts

All validation artifacts are stored:
- Model checkpoints
- Validation metrics
- Test configurations
- Results summaries

## 10. Continuous Validation

```mermaid
flowchart LR
    A[Model Update] --> B[Run Validation]
    B --> C{Passes?}
    C -->|Yes| D[Deploy Update]
    C -->|No| E[Reject Update]
    E --> F[Investigate Issues]
    D --> G[Monitor Performance]
```
