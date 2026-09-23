# Dataset Storage

## 1. Purpose

Define the storage strategy for 2048 game training datasets, ensuring efficient and reliable data persistence.

## 2. Storage Overview — MVP Local Only

> **MVP: Simple local `data/raw, data/processed, data/results` — no tiered storage.** Hot/Warm/Cold (SSD/HDD/Cloud) is over-engineering for <5GB; future optional only.

Datasets are stored locally for MVP:

```
data/
├── raw/        # Raw game logs
├── processed/  # Cleaned CSV/Parquet for training
└── results/    # Model outputs / benchmarks
```

<details><summary>Future optional tiered storage (out of scope for MVP, <5GB)</summary>

```mermaid
flowchart TD
    subgraph "Storage Architecture — Future Optional"
        Hot[Hot Storage<br/>SSD]
        Warm[Warm Storage<br/>HDD]
        Cold[Cold Storage<br/>Cloud]
        Hot --> Access[Fast Access]
        Warm --> Access
        Cold --> Access
        Access --> Pipeline[ML Pipeline]
    end
```
</details>

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

## 4. Storage Locations — MVP Local Only

> **MVP: local `data/` only.** Parquet/CSV both acceptable; cache layer is future optional.

```mermaid
flowchart TB
    Locations[Storage Locations — MVP]
    Locations --> Local[Local Storage<br/>data/]
    Locations --> Parquet[Parquet Files]
    Locations --> CSV[CSV Files]
    Local --> Format[Formatted Data]
    Parquet --> Format
    CSV --> Format
```

### 4.1 Local Storage Structure — MVP

> **MVP: `data/raw, data/processed, data/results` local.** No archive/metadata tiering for <5GB.

```
data/
├── raw/        # *.parquet / *.csv raw logs
├── processed/  # *.parquet cleaned (MVP)
└── results/    # benchmarks / reports
```

<details><summary>Future optional structure (out of scope)</summary>

```mermaid
flowchart TD
    Root[06-Data/03-Storage/]
    Root --> Raw[raw/]
    Root --> Processed[processed/]
    Root --> Archive[archive/]
    Root --> Metadata[metadata/]
```
</details>

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

Canonical path is `06-Data/03-Storage/` with `data/` as symlink for runtime. No dual-source confusion.

```rust
use std::path::PathBuf;
pub struct DatasetStorageConfig {
    pub storage_path: PathBuf, // canonical: "06-Data/03-Storage/" (symlinked as data/)
}
```

> Deleted `partition_by`/`cache_size`/`compression` — over-engineering for <5GB. MVP is `data/raw, data/processed, data/results` local only.

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
