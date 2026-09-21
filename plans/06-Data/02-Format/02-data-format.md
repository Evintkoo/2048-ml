# Data Format

## 1. Purpose

Define the data formats used for storing and exchanging 2048 game training data.

## 2. Data Format Overview

Multiple data formats are supported for different stages of the ML pipeline.

```mermaid
flowchart TD
    subgraph "Data Format Pipeline"
        subgraph "Formats"
            CSV[CSV Format<br/>Small Datasets]
            Parquet[Parquet Format<br/>Large Datasets]
            JSON[JSON Format<br/>Debugging]
            Binary[Binary Format<br/>Performance]
        end
        
        CSV --> Use1[Training Data]
        Parquet --> Use2[Production Data]
        JSON --> Use3[Debug/Export]
        Binary --> Use4[Fast Loading]
    end
    
    Use1 --> Pipeline[ML Pipeline]
    Use2 --> Pipeline
    Use3 --> Pipeline
    Use4 --> Pipeline
    
    style Pipeline fill:#e3f2fd
```

## 3. Format Comparison

```mermaid
flowchart TB
    Formats[Format Comparison]
    
    CSV --> |Small data<br/>Human readable| CSVDetail[CSV Details]
    Parquet --> |Large data<br/>Columnar| ParquetDetail[Parquet Details]
    JSON --> |Debugging<br/>Flexible| JSONDetail[JSON Details]
    Binary --> |Performance<br/>Compact| BinaryDetail[Binary Details]
    
    CSVDetail --> Select{Select Format}
    ParquetDetail --> Select
    JSONDetail --> Select
    BinaryDetail --> Select
    
    style Select fill:#e8f5e9
```

## 4. CSV Format

```mermaid
flowchart TD
    CSVFormat[CSV Format]
    CSVFormat --> Header[Header Row<br/>27 features + action]
    CSVFormat --> Rows[Data Rows<br/>Comma-separated values]
    CSVFormat --> Parser[CSV Parser]
    Parser --> DataFrame[DataFrame<br/>27-D features + 1 label]
    
    style CSVFormat fill:#e3f2fd
```

### CSV Structure

Canonical training CSV is **27 feature columns + 1 action label = 28 columns total**. Score is **not** a training label — it is stored separately only for analysis.

```
# Header (27 features + action)
grid_0,grid_1,grid_2,grid_3,grid_4,grid_5,grid_6,grid_7,grid_8,grid_9,grid_10,grid_11,grid_12,grid_13,grid_14,grid_15,empty_count,max_tile_log,monotonicity,smoothness,merges_available,score_normalized,adjacency_merge_score,corner_max,edge_tiles_occupied,col_worst,row_worst,action
# Example row (16 grid normalized + 11 derived normalized + action)
0.0,0.00006,0.00012,0.00024,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.875,0.2,0.85,0.72,0.125,0.15,0.03,0.06,0.5,0.02,0.03,2
# Optional separate analysis file may include raw score column, but it is NOT the classification label
```

## 5. Parquet Format

```mermaid
flowchart TD
    Parquet[Parquet Format]
    Parquet --> Columnar[Columnar Storage]
    Columnar --> Compression[Compression]
    Compression --> Efficient[Efficient Storage]
    Efficient --> Fast[Fast Loading]
    Fast --> Pipeline[ML Pipeline]
    
    style Parquet fill:#e8f5e9
```

### Parquet Schema

```rust
use polars::prelude::*;

/// Create DataFrame for classification training: 27-D feature table + action labels.
/// Scores are NOT mixed into the feature/label table — they are for analysis only.
fn create_dataframe(states: &[[f64; 27]], actions: &[u8]) -> DataFrame {
    // States: N × 27 canonical features (grid_0..row_worst)
    // Labels: N action classes (0=Up,1=Down,2=Left,3=Right)
    // Optional: store raw scores separately for analysis (e.g., `scores: &[u64]` as metadata, not as feature/label)
    // Stored in 06-Data/03-Storage/
    todo!()
}

/// Optional helper to store scores for analysis (not used as training label)
fn create_score_metadata(scores: &[u64]) -> Series {
    Series::new("score_raw".into(), scores)
}
```

## 6. JSON Format

```mermaid
flowchart TD
    JSON[JSON Format]
    JSON --> State[State Object]
    JSON --> Action[Action Value]
    JSON --> Score[Score Value]
    JSON --> Metadata[Metadata]
    
    State --> Grid[Grid: [u32; 16]]
    State --> Features[Derived Features]
    
    style JSON fill:#fff3e0
```

### JSON Structure

```json
{
    "state": {
        "grid": [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 0, 0, 0, 0, 0],
        "score": 4092,
        "empty_count": 5,
        "max_tile_log": 11.0,
        "monotonicity": 0.8
    },
    "action": 1,
    "score": 2048,
    "game_over": false
}
```

## 7. Format Conversion Pipeline

```mermaid
flowchart TD
    Convert[Format Conversion]
    Convert --> CSV_to_Parquet[CSV → Parquet]
    Convert --> JSON_to_CSV[JSON → CSV]
    Convert --> Parquet_to_Binary[Parquet → Binary]
    
    CSV_to_Parquet --> Standardize[Standard Format]
    JSON_to_CSV --> Standardize
    Parquet_to_Binary --> Standardize
    
    style Standardize fill:#e8f5e9
```

## 8. Format Files Location

All data format files are in `06-Data/02-Format/`:

```mermaid
flowchart LR
    Dir[06-Data/02-Format]
    Dir --> N01[01-data-schema.md]
    Dir --> N02[02-data-format.md]
    Dir --> N03[03-data-standard.md]
```

## 9. Next Steps

1. Select appropriate format for each pipeline stage
2. Implement format converters
