# Data Versioning

## 1. Purpose

Define the data versioning strategy to track changes to datasets over time and ensure reproducibility.

## 2. Versioning Overview

Data versioning ensures that every dataset change is tracked, enabling reproducibility and rollback capabilities.

```mermaid
flowchart TD
    subgraph "Data Versioning System"
        subgraph "Version Components"
            VNum[Version Number<br/>v1.0, v1.1, v2.0]
            Changes[Change Log]
            Checksum[Data Checksum]
            Timestamp[Timestamp]
            Author[Author Info]
        end
        
        VNum --> Track[Track Version]
        Changes --> Track
        Checksum --> Track
        Timestamp --> Track
        Author --> Track
    end
    
    Track --> Storage[06-Data/03-Storage/]
    
    style Track fill:#e8f5e9
```

## 3. Versioning Workflow

```mermaid
flowchart TD
    Start[Create New Dataset Version]
    Start --> Modify[Modify Data]
    Modify --> Validate[Validate Changes]
    Validate --> Checksum[Generate Checksum]
    Checksum --> Assign[Assign Version Number]
    Assign --> Record[Record in Version Log]
    Record --> Store[Store Versioned Data]
    Store --> Access[Versioned Access]
    
    style Start fill:#e3f2fd
    style Store fill:#e8f5e9
```

## 4. Version History

```mermaid
flowchart TB
    History[Version History]
    History --> V1[v1.0 - Initial Dataset]
    History --> V2[v1.1 - Added features]
    History --> V3[v2.0 - Major restructuring]
    History --> V4[v2.1 - Bug fixes]
    
    V1 --> V2
    V2 --> V3
    V3 --> V4
    
    style V1 fill:#e3f2fd
    style V4 fill:#e8f5e9
```

### 4.1 Version Tracking Diagram

```mermaid
flowchart LR
    V1_0[v1.0] --> V1_1[v1.1]
    V1_1 --> V2_0[v2.0]
    V2_0 --> V2_1[v2.1]
    
    V1_0 --> |Initial| Data1[Initial Data]
    V1_1 --> |Add features| Data2[Extended Data]
    V2_0 --> |Restructure| Data3[Restructured Data]
    V2_1 --> |Fix bugs| Data4[Cleaned Data]
    
    style V1_0 fill:#fff3e0
    style V2_1 fill:#c8e6c9
```

## 5. Version Control Configuration

```rust
pub struct DataVersionConfig {
    pub dataset_name: String,
    pub current_version: String,      // "v1.0"
    pub history: Vec<VersionEntry>,
    pub storage_path: String,         // 06-Data/03-Storage/
}

pub struct VersionEntry {
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub author: String,
    pub changes: String,
    pub checksum: String,
}
```

## 6. Reproducibility

```mermaid
flowchart TD
    Repro[Reproducibility]
    Repro --> Version[Pin Version]
    Version --> Checksum[Verify Checksum]
    Checksum --> Reproduce[Reproduce Results]
    Reproduce --> Validate[Validate Output]
    
    style Repro fill:#e3f2fd
    style Validate fill:#e8f5e9
```

### 6.1 Reproducibility Chain

```mermaid
flowchart LR
    Config[Config<br/>v1.0] --> Data[Data<br/>v2.1]
    Data --> Model[Model<br/>v1.0]
    Model --> Result[Results<br/>Reproducible]
    
    style Config fill:#e3f2fd
    style Data fill:#fff3e0
    style Model fill:#e8f5e9
    style Result fill:#c8e6c9
```

## 7. Data Versioning Files Location

All data versioning files are in `06-Data/03-Storage/`:

```mermaid
flowchart LR
    Dir[06-Data/03-Storage]
    Dir --> N01[01-dataset-storage.md]
    Dir --> N02[02-data-versioning.md]
```

## 8. Next Steps

1. Implement version control for all datasets
2. Set up automated version tracking
