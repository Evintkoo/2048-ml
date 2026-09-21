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
    CSVFormat --> Header[Header Row<br/>grid_0,...,grid_15,action,score]
    CSVFormat --> Rows[Data Rows<br/>Comma-separated values]
    CSVFormat --> Parser[CSV Parser]
    Parser --> DataFrame[DataFrame]
    
    style CSVFormat fill:#e3f2fd
```

### CSV Structure

```
grid_0,grid_1,...,grid_15,empty_count,max_tile_log,monotonicity,smoothness,corner_value,available_moves,merges_available,score_normalized,move_count_norm,action,score
2,4,8,16,32,64,128,256,512,1024,2048,0,0,0,0,8,4.5,0.8,0.9,0.5,2,0.0004,0.3,1,2048
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

fn create_dataframe(states: &[BoardStateML], actions: &[u8], scores: &[u64]) -> DataFrame {
    // Create polars DataFrame from collected data
    // Stored in 06-Data/03-Storage/
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
    
    State --> Grid[Grid: [u8; 16]]
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
    Dir --> 01[01-data-schema.md]
    Dir --> 02[02-data-format.md]
    Dir --> 03[03-data-standard.md]
```

## 9. Next Steps

1. Select appropriate format for each pipeline stage
2. Implement format converters
