# Feature Engineering

## 1. Purpose

Define the feature engineering process to extract meaningful features from raw 2048 game board states.

## 2. Feature Engineering Overview

Feature engineering transforms raw board data into informative features that improve ML model performance.

```mermaid
flowchart TD
    subgraph "Feature Engineering Pipeline"
        Raw[Raw Board State<br/>4x4 Grid]
        Raw --> Extract[Extract Features]
        Extract --> Transform[Transform Features]
        Transform --> Select[Select Features]
        Select --> Construct[Construct Feature Vector]
        Construct --> Validate[Validate Features]
        Validate --> Output[Feature Vector: 27 dims]
        
        style Raw fill:#ffcdd2
        style Output fill:#c8e6c9
    end
```

## 3. Feature Types

```mermaid
flowchart TB
    subgraph "Feature Categories"
        RawFeat[Raw Features<br/>16 dimensions]
        DerivedFeat[Derived Features<br/>11 dimensions]
    end
    
    RawFeat --> Grid[Grid Values<br/>0-15]
    DerivedFeat --> Empty[Empty Count<br/>16]
    DerivedFeat --> MaxTile[Max Tile Log<br/>17]
    DerivedFeat --> Mono[Monotonicity<br/>18]
    DerivedFeat --> Smooth[Smoothness<br/>19]
    DerivedFeat --> Merges[Merges Available<br/>20]
    DerivedFeat --> Score[Score Normalized<br/>21]
    DerivedFeat --> AdjMerge[Adjacency Merge Score<br/>22]
    DerivedFeat --> Corner[Corner Max<br/>23]
    DerivedFeat --> Edge[Edge Tiles Occupied<br/>24]
    DerivedFeat --> ColWorst[Column Worst<br/>25]
    DerivedFeat --> RowWorst[Row Worst<br/>26]
    
    style RawFeat fill:#e3f2fd
    style DerivedFeat fill:#fff3e0
```

## 4. Feature Engineering Process

```mermaid
flowchart TD
    Process[Feature Engineering Process]
    Process --> Step1[Step 1: Extract Grid Features]
    Process --> Step2[Step 2: Compute Derived Features]
    Process --> Step3[Step 3: Normalize Features]
    Process --> Step4[Step 4: Select Best Features]
    Process --> Step5[Step 5: Construct Final Vector]
    
    Step1 --> Step2
    Step2 --> Step3
    Step3 --> Step4
    Step4 --> Step5
    Step5 --> Final[27-dim Feature Vector]
    
    style Step1 fill:#e3f2fd
    style Final fill:#e8f5e9
```

### 4.1 Feature Extraction Flow

```mermaid
flowchart LR
    Board[4x4 Board] --> Flatten[Flatten to 16 dims]
    Flatten --> Stats[Compute Statistics]
    Stats --> Monotonicity[Monotonicity Score]
    Stats --> Smoothness[Smoothness Score]
    Stats --> Corner[Corner Value]
    Monotonicity --> Derived[Derived Features]
    Smoothness --> Derived
    Corner --> Derived
    Derived --> Combined[Combined Features]
    
    style Board fill:#ffcdd2
    style Combined fill:#e8f5e9
```

## 5. Feature Importance

```mermaid
flowchart TD
    Importance[Feature Importance]
    Importance --> Method[Importance Method]
    Method --> Model[Model-based Importance]
    Method --> Perm[Permutation Importance]
    Method --> SHAP[SHAP Values]
    
    Model --> Rank[Rank Features]
    Perm --> Rank
    SHAP --> Rank
    Rank --> Select[Select Top Features]
    
    style Importance fill:#e3f2fd
    style Select fill:#e8f5e9
```

### 5.1 Feature Importance Ranking

```mermaid
flowchart TB
    Ranking[Feature Importance Ranking]
    Ranking --> Top1[grid_0 - Highest]
    Ranking --> Top2[grid_1]
    Ranking --> Top3[grid_2]
    Ranking --> TopN[...grid_15]
    Ranking --> Empty[empty_count]
    Ranking --> MaxTile[max_tile_log]
    Ranking --> Mono[monotonicity]
    Ranking --> AdjMerge[adjacency_merge_score]
    Ranking --> ColWorst[col_worst]
    Ranking --> RowWorst[row_worst]
    Ranking --> Smooth[smoothness]
    
    style Top1 fill:#c8e6c9
    style Bottom fill:#ffcdd2
```

## 6. Feature Construction

```mermaid
flowchart TD
    Construct[Feature Construction]
    Construct --> Interaction[Feature Interactions]
    Construct --> Polynomial[Polynomial Features]
    Construct --> Aggregate[Aggregated Features]
    
    Interaction --> Final[Final Feature Set]
    Polynomial --> Final
    Aggregate --> Final
    
    style Final fill:#e8f5e9
```

## 7. Feature Engineering Configuration

```rust
pub struct FeatureEngineeringConfig {
    pub extract_grid: bool,
    pub compute_derived: bool,
    pub normalize: bool,
    pub select_features: bool,
    pub target_dimensions: usize,  // 27
}
```

## 8. Feature Files Location

All feature engineering files are in `06-Data/04-Preprocessing/`:

```mermaid
flowchart LR
    Dir[06-Data/04-Preprocessing]
    Dir --> N01[01-data-cleaning.md]
    Dir --> N02[02-feature-engineering.md]
    Dir --> N03[03-data-normalization.md]
```

## 9. Next Steps

1. Extract features from raw board data
2. Compute derived features
3. Validate feature quality
