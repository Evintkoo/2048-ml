# Plan 01 — Dataset Storage: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Local CSV, metadata, checkpoint, and manifest outputs exist; catalog/version management and Parquet are absent.

**Goal:** State the current implementation and evidence boundary for dataset storage.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats local artifact storage as implemented with optional infrastructure absent.** The collector writes training CSV, aligned metadata, checkpoint chunks, and a JSON manifest to a caller-selected path. Parquet, dataset cataloging, and managed archival are not implemented.

## 1. Purpose

Define the storage strategy for 2048 game training datasets, ensuring efficient and reliable data persistence.

## 2. Storage Overview — MVP Local Only

> **MVP: Simple local `data/raw, data/processed, data/results` — no tiered storage.** No multi-tier storage or size-based claim is made; revisit if measured data volume requires it.

Datasets are stored locally for MVP:

```
data/
├── raw/        # Raw game logs
├── processed/  # CSV partitions for training
└── results/    # Model outputs / benchmarks
```

<details><summary>Future optional tiered storage (out of scope unless measured needs justify it)</summary>

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

> The live collector writes CSV. Parquet and a cache layer are not supported.

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

> **MVP: `data/raw, data/processed, data/results` local.** No archive/metadata tiering based on measured needs.

```
data/
├── raw/        # CSV collection output and checkpoints
├── processed/  # *.csv splits (MVP)
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
    Format --> CSV[CSV - Training data]
    Format --> JSON[JSON - Manifest/checkpoint]
    Format --> Metadata[Metadata JSON]
    
    Parquet --> |Large datasets| Efficient[Efficient Storage]
    CSV --> |Small datasets| Accessible[Human Readable]
    Metadata --> |Tracking| Track[Version Tracking]
    
    style Parquet fill:#e3f2fd
```

## 6. Storage Configuration

Plan documents live under `plans/06-Data/03-Storage/`; runtime outputs default to `data/raw/random_play.csv` and can be redirected with `--output`. There is no symlink requirement.

```rust
use std::path::PathBuf;
pub struct DatasetStorageConfig {
    pub storage_path: PathBuf, // canonical: "06-Data/03-Storage/" (symlinked as data/)
}
```

> Deleted `partition_by`/`cache_size`/`compression` — over-engineering based on measured needs. MVP is `data/raw, data/processed, data/results` local only.

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

The local CSV MVP is implemented: `data-collector collect` writes training rows, row-aligned metadata CSV, and a JSON manifest; `split` emits grouped chronological partitions; validation checks the exact model schema. Dataset manifests include row counts, source revision, and CSV SHA-256. Parquet support, a catalog/index layer, and a dataset cache are not implemented and remain optional until measured dataset size justifies them.

## Implementation Record

- Local collection writes to a caller-selected CSV path (default `data/raw/random_play.csv`); row-aligned metadata, checkpoint parts, and JSON manifest are emitted alongside. `split` writes chronological game-level CSV partitions. No storage catalog, managed index, or Parquet writer exists.

---

## Verification (definition of done)

1. `test -f plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/03-Storage/01-dataset-storage.md` exits 0.

## Open questions

- Storage outputs are local and can be removed by the user; the project does not currently commit or synchronize generated data. Preserve manifests and hashes with any retained experiment.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
