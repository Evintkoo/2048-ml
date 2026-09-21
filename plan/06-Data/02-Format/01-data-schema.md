# Data Schema

## 1. Purpose

Define the data schema for all training and evaluation data used in the 2048 game machine learning pipeline.

## 2. Data Schema Overview

The data schema defines the structure of all data records, ensuring consistency across the pipeline.

```mermaid
flowchart TD
    subgraph "Data Schema"
        subgraph "State Record"
            Grid[Grid Features: 16 dims]
            Derived[Derived Features: 9 dims]
            Action[Action: 0-3]
            Score[Score: u64]
            Done[Game Over: bool]
        end
        
        subgraph "Game Record"
            GameID[Game ID: UUID]
            AgentType[Agent Type]
            MoveCount[Move Count]
            FinalScore[Final Score]
            Timestamp[Timestamp]
        end
        
        subgraph "Training Record"
            State[State Vector: 25 dims]
            Target[Target Value]
            Label[Action Label]
            Reward[Reward Signal]
        end
    end
    
    State --> Dataset[Unified Dataset]
    Game --> Dataset
    Training --> Dataset
    
    style Dataset fill:#e3f2fd
```

## 3. State Feature Schema

```mermaid
flowchart TB
    subgraph "25-Dimensional State Vector"
        Grid[Grid Features<br/>Dimensions 0-15]
        Empty[Empty Count<br/>Dimension 16]
        MaxTile[Max Tile Log<br/>Dimension 17]
        Mono[Monotonicity<br/>Dimension 18]
        Smooth[Smoothness<br/>Dimension 19]
        Corner[Corner Value<br/>Dimension 20]
        Moves[Available Moves<br/>Dimension 21]
        Merges[Merges Available<br/>Dimension 22]
        ScoreNorm[Score Normalized<br/>Dimension 23]
        MoveNorm[Move Count Norm<br/>Dimension 24]
    end
    
    Grid --> StateVec[State Vector<br/>[f64; 25]]
    Empty --> StateVec
    MaxTile --> StateVec
    Mono --> StateVec
    Smooth --> StateVec
    Corner --> StateVec
    Moves --> StateVec
    Merges --> StateVec
    ScoreNorm --> StateVec
    MoveNorm --> StateVec
    
    style StateVec fill:#e8f5e9
```

## 4. Schema Definition

```rust
pub struct StateRecord {
    pub grid: [f64; 16],
    pub empty_count: f64,
    pub max_tile_log: f64,
    pub monotonicity: f64,
    pub smoothness: f64,
    pub corner_value: f64,
    pub available_moves: f64,
    pub merges_available: f64,
    pub score_normalized: f64,
    pub move_count_norm: f64,
    pub action: u8,
    pub score: u64,
    pub done: bool,
}
```

## 5. Data Format Flow

```mermaid
flowchart TD
    Raw[Raw Board State]
    Raw --> Encode[Encode to 25-dim Vector]
    Encode --> Schema[Apply Data Schema]
    Schema --> Validate[Validate Schema]
    Validate --> Serialize[Serialize to Format]
    Serialize --> Store[Store Data]
    
    style Encode fill:#e3f2fd
    style Validate fill:#fff3e0
    style Store fill:#e8f5e9
```

## 6. Schema Validation

```mermaid
flowchart TD
    Validate[Schema Validation]
    Validate --> Check1[Check Dimensions: 25]
    Validate --> Check2[Check Types: f64]
    Validate --> Check3[Check Ranges: 0-1]
    Validate --> Check4[Check Action: 0-3]
    Validate --> Check5[Check Score: u64]
    
    Check1 --> AllPass{All Pass?}
    Check2 --> AllPass
    Check3 --> AllPass
    Check4 --> AllPass
    Check5 --> AllPass
    AllPass --> |Yes| Accept[Accept]
    AllPass --> |No| Reject[Reject]
    
    style Accept fill:#c8e6c9
    style Reject fill:#ffcdd2
```

## 7. Schema Files Location

All data schema files are in `06-Data/02-Format/`:

```mermaid
flowchart LR
    Dir[06-Data/02-Format]
    Dir --> 01[01-data-schema.md]
    Dir --> 02[02-data-format.md]
    Dir --> 03[03-data-standard.md]
```

## 8. Next Steps

1. Implement schema validation
2. Apply schema to all data pipelines
