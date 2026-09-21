# Dataset Storage

## 1. Purpose

Define the storage strategy for 2048 game training datasets, ensuring efficient and reliable data persistence.

## 2. Storage Overview

Datasets are stored using a tiered storage approach optimized for both performance and cost.

```mermaid
flowchart TD
    subgraph "Storage Architecture"
        subgraph "Storage Tiers"
            Hot[Hot Storage<br/>SSD - Active Training]
            Warm[Warm Storage<br/>HDD - Recent Data]
            Cold[Cold Storage<br/>Cloud - Archive]
        end
        
        Hot --> Access[Fast Access]
        Warm --> Access
        Cold --> Access
        
        Access --> Pipeline[ML Pipeline]
    end
    
    style Hot fill:#e3f2fd
    style Pipeline fill:#e8f5e9
```

## 3. Storage Pipeline

```mermaid
flowchart TD
    Pipeline[Dataset Storage Pipeline]
    Pipeline --> Stage1[Stage 1: Collect Data]
    Stage1 --> Stage2[Stage 2: Validate Data]
    Stage2 --> Stage3[Stage 3: Format Data]
    Stage3 --> Stage4[Stage 4: Store Data]
    Stage4 --> Stage5[Stage 5: Index Data]
    
    Stage1 --> |Raw| Stage2
    Stage2 --> |Clean| Stage3
    Stage3 --> |Formatted| Stage4
    Stage4 --> |Stored| Stage5
    Stage5 --> |Indexed| Access[Data Access]
    
    style Stage1 fill:#e3f2fd
    style Stage5 fill:#e8f5e9
```

## 4. Storage Locations

```mermaid
flowchart TB
    Locations[Storage Locations]
    
    Locations --> Local[Local Storage<br/>06-Data/03-Storage/]
    Locations --> Parquet[Parquet Files]
    Locations --> CSV[CSV Files]
    Locations --> Cache[Cache Layer]
    
    Local --> Format[Formatted Data]
    Parquet --> Format
    CSV --> Format
    Cache --> Format
    
    style Local fill:#e3f2fd
    style Cache fill:#fff3e0
```

### 4.1 Local Storage Structure

```mermaid
flowchart TD
    Root[06-Data/03-Storage/]
    Root --> Raw[raw/]
    Root --> Processed[processed/]
    Root --> Archive[archive/]
    Root --> Metadata[metadata/]
    
    Raw --> ParquetFiles[*.parquet]
    Processed --> ParquetFiles
    Archive --> ParquetFiles
    Metadata --> MetaFiles[*.json]
    
    style Root fill:#e8f5e9
```

## 5. Data Storage Format

```mermaid
flowchart TD
    Format[Storage Format]
    Format --> Parquet[Parquet - Primary]
    Format --> CSV[CSV - Secondary]
    Format --> Metadata[Metadata JSON]
    
    Parquet --> |Large datasets| Efficient[Efficient Storage]
    CSV --> |Small datasets| Accessible[Human Readable]
    Metadata --> |Tracking| Track[Version Tracking]
    
    style Parquet fill:#e3f2fd
```

## 6. Storage Configuration

```rust
pub struct DatasetStorageConfig {
    pub storage_path: String,      // 06-Data/03-Storage/
    pub format: StorageFormat,     // Parquet or CSV
    pub compression: CompressionType, // Snappy, Zstd
    pub partition_by: Vec<String>, // Partition strategy
    pub cache_size: usize,         // Cache size in MB
}
```

## 7. Data Access Pattern

```mermaid
flowchart TD
    Access[Data Access Pattern]
    Access --> Read[Read Data]
    Access --> Write[Write Data]
    Access --> Delete[Delete Data]
    Access --> List[List Datasets]
    
    Read --> Load[Load from Storage]
    Write --> Save[Save to Storage]
    Delete --> Remove[Remove from Storage]
    List --> Catalog[Catalog Datasets]
    
    style Read fill:#e3f2fd
    style Write fill:#e8f5e9
```

## 8. Storage Files Location

All dataset storage files are in `06-Data/03-Storage/`:

```mermaid
flowchart LR
    Dir[06-Data/03-Storage]
    Dir --> N01[01-dataset-storage.md]
    Dir --> N02[02-data-versioning.md]
```

## 9. Next Steps

1. Set up storage infrastructure
2. Implement data access layer
