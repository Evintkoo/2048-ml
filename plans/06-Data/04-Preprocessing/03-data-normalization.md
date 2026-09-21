# Data Normalization

## 1. Purpose

Define the data normalization process to scale features to a consistent range for optimal ML model training.

## 2. Normalization Overview

Normalization ensures all features are on a comparable scale, improving model convergence and performance.

```mermaid
flowchart TD
    subgraph "Normalization Pipeline"
        Raw[Raw Features<br/>Mixed Scales]
        Raw --> Analyze[Analyze Distributions]
        Analyze --> Select[Select Normalizer]
        Select --> Fit[Fit Normalizer]
        Fit --> Transform[Transform Features]
        Transform --> Validate[Validate Normalized]
        Validate --> Output[Normalized Features<br/>[0, 1] Range]
        
        style Raw fill:#ffcdd2
        style Output fill:#c8e6c9
    end
```

## 3. Normalization Methods

```mermaid
flowchart TB
    subgraph "Normalization Methods"
        Standard[StandardScaler<br/>Z-score Normalization]
        MinMax[MinMaxScaler<br/>Scale to [0,1]]
        Robust[RobustScaler<br/>Using Median/IQR]
        Log[Log Transform<br/>For Skewed Data]
    end
    
    Standard --> Apply[Apply to Features]
    MinMax --> Apply
    Robust --> Apply
    Log --> Apply
    
    style Standard fill:#e3f2fd
    style MinMax fill:#fff3e0
```

### 3.1 StandardScaler

```mermaid
flowchart TD
    Std[StandardScaler]
    Std --> Mean[Calculate Mean]
    Std --> StdDev[Calculate Std Dev]
    Std --> Formula[(x - μ) / σ]
    Formula --> Normalized[Normalized Feature]
    
    style Std fill:#e3f2fd
    style Normalized fill:#e8f5e9
```

### 3.2 MinMaxScaler

```mermaid
flowchart TD
    MinMax[MinMaxScaler]
    MinMax --> Min[Find Min]
    MinMax --> Max[Find Max]
    MinMax --> Formula[(x - min) / (max - min)]
    Formula --> Scaled[Scaled to [0, 1]]
    
    style MinMax fill:#e3f2fd
    style Scaled fill:#e8f5e9
```

## 4. Normalization Process

```mermaid
flowchart TB
    Process[Normalization Process]
    Process --> Step1[Step 1: Analyze Feature Distributions]
    Process --> Step2[Step 2: Select Normalizer per Feature]
    Process --> Step3[Step 3: Fit Normalizer on Training Data]
    Process --> Step4[Step 4: Transform Training Data]
    Process --> Step5[Step 5: Transform Validation/Test Data]
    Process --> Step6[Step 6: Verify Scaled Ranges]
    
    Step1 --> Step2
    Step2 --> Step3
    Step3 --> Step4
    Step4 --> Step5
    Step5 --> Step6
    Step6 --> Done[Normalized Dataset]
    
    style Step1 fill:#e3f2fd
    style Done fill:#e8f5e9
```

## 5. Per-Feature Normalization

```mermaid
flowchart TB
    subgraph "Feature Normalization Map"
        Grid[Grid Features<br/>StandardScaler]
        Empty[Empty Count<br/>MinMaxScaler]
        MaxTile[Max Tile Log<br/>Log Transform]
        Mono[Monotonicity<br/>MinMaxScaler]
        Smooth[Smoothness<br/>MinMaxScaler]
        Corner[Corner Value<br/>MinMaxScaler]
        Moves[Available Moves<br/>MinMaxScaler]
        Merges[Merges Available<br/>MinMaxScaler]
        Score[Score Normalized<br/>Already Scaled]
        AdjMerge[Adjacency Merge Score<br/>MinMaxScaler]
        ColWorst[Column Worst<br/>MinMaxScaler]
        RowWorst[Row Worst<br/>MinMaxScaler]
    end
    
    Grid --> Normalized[Normalized Features]
    Empty --> Normalized
    MaxTile --> Normalized
    Mono --> Normalized
    Smooth --> Normalized
    Corner --> Normalized
    Moves --> Normalized
    Merges --> Normalized
    Score --> Normalized
    MoveCount --> Normalized
    
    style Normalized fill:#e8f5e9
```

## 6. Normalization Pipeline

```mermaid
sequenceDiagram
    participant Raw as Raw Features
    participant Preprocess as DataPreprocessor
    participant Normalizer as Normalizer
    participant Norm as Normalized Features
    
    Raw->>Preprocess: Feed raw data
    Preprocess->>Normalizer: Fit on training data
    Normalizer->>Normalizer: Compute mean/std
    Normalizer->>Norm: Transform features
    Norm->>Model[ML Model]
    
    Note over Normalizer: StandardScaler<br/>MinMaxScaler<br/>RobustScaler
```

## 7. Normalization Verification

```mermaid
flowchart TD
    Verify[Normalization Verification]
    Verify --> Check1[All features in [0,1]]
    Verify --> Check2[No NaN values]
    Verify --> Check3[No Inf values]
    Verify --> Check4[Distribution preserved]
    Verify --> Check5[No data leakage]
    
    Check1 --> Pass{All Verified?}
    Check2 --> Pass
    Check3 --> Pass
    Check4 --> Pass
    Check5 --> Pass
    Pass --> |Yes| Verified[Data is Normalized]
    Pass --> |No| Fix[Re-normalize]
    
    style Verified fill:#c8e6c9
    style Fix fill:#fff3e0
```

## 8. Normalization Configuration

```rust
use automl::preprocessing::{DataPreprocessor, PreprocessingConfig, ScalerType, ImputeStrategy};

let preprocessor = DataPreprocessor::new(
    PreprocessingConfig::default()
        .with_scaler(ScalerType::Standard) // real API: with_scaler, not with_scaler_type
        .with_numeric_impute(ImputeStrategy::Mean) // real API: ImputeStrategy, not ImputationStrategy
);
let normalized = preprocessor.fit_transform(&raw_features)?;
```

## 9. Normalization Files Location

All data normalization files are in `06-Data/04-Preprocessing/`:

```mermaid
flowchart LR
    Dir[06-Data/04-Preprocessing]
    Dir --> N01[01-data-cleaning.md]
    Dir --> N02[02-feature-engineering.md]
    Dir --> N03[03-data-normalization.md]
```

## 10. Next Steps

1. Configure normalizer for each feature type
2. Execute normalization pipeline
3. Verify normalized data quality
