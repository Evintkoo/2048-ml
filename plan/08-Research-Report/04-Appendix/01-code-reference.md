# Code Reference

## 1. Purpose

Provide code references for the 2048 ML research study.

## 2. Code Architecture

```mermaid
flowchart TD
    subgraph "Code Structure"
        subgraph "Game Module"
            GE[Game Engine]
            BS[Board State]
            MR[Move Rules]
        end
        
        subgraph "ML Module"
            TM[Train Model]
            PE[Predict Engine]
            HM[HyperOptX]
        end
        
        subgraph "Data Module"
            DC[Data Collector]
            SC[Score Calculator]
            VC[Validation]
        end
        
        GE --> BS
        TM --> PE
        DC --> TM
        SC --> DC
    end
```

## 3. Key Modules

| Module | File | Description |
|--------|------|-------------|
| GameEngine | game_engine.rs | Core game simulation |
| TrainEngine | train_engine.rs | ML training pipeline |
| HyperOptX | hyperopt.rs | Hyperparameter search |
| Board | board.rs | Board state management |
| ScoreTracker | score.rs | Score calculation |

## 4. Data Flow

```mermaid
flowchart LR
    A[Game Simulation] --> B[State Extraction]
    B --> C[Feature Engineering]
    C --> D[Training Data]
    D --> E[TrainEngine]
    E --> F[Model Training]
    F --> G[Model Output]
    G --> H[Prediction]
    H --> I[Game Evaluation]
```

## 5. Configuration References

```rust
// Training configuration
pub struct TrainingConfig {
    pub model_type: ModelType,
    pub learning_rate: f64,
    pub epochs: usize,
    pub batch_size: usize,
}

// Hyperparameter search configuration  
pub struct HyperOptConfig {
    pub search_space: SearchSpace,
    pub n_trials: usize,
    pub algorithm: HyperOptX,
}
```

## 6. API References

```mermaid
graph TD
    A[TrainEngine API] -->|train| B[Model]
    C[PredictEngine API] -->|predict| D[Action]
    E[HyperOptX API] -->|optimize| F[Hyperparameters]
    G[GameEngine API] -->|run| H[GameResult]
```

## 7. Code Organization

```mermaid
graph TD
    plan[plan/]
    P1[01-Infrastructure/]
    P2[02-Environment/]
    P3[03-State/]
    P4[07-Benchmarking/]
    P5[08-Research-Report/]
    P6[09-Quality/]
    
    plan --> P1
    plan --> P2
    plan --> P3
    plan --> P4
    plan --> P5
    plan --> P6
```

## 8. Dependencies

| Dependency | Version | Purpose |
|-----------|---------|---------|
| automl | v1.0.0 | ML training |
| HyperOptX | latest | Hyperparameter search |
| Rust | stable | Language runtime |
| polars | latest | Data processing |

## 9. Reproducibility

All code references use deterministic seeds:

```mermaid
flowchart LR
    A[Seed] --> B[TrainingConfig]
    B --> C[Model Initialization]
    C --> D[Data Loading]
    D --> E[Training Pipeline]
    E --> F[Results]
```
