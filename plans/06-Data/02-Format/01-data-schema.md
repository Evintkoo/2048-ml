# Plan 01 — Data Schema: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Canonical CSV columns and sidecar are validated; Parquet is unsupported and tile-range contract remains unresolved.

**Goal:** State the current implementation and evidence boundary for data schema.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the CSV schema as implemented with a tile-range gap.** The training CSV has 27 feature columns and integer `action`; per-game/move/score provenance is written to an aligned metadata sidecar. Root validation checks header, width, finite/range constraints, and action range, but cannot revalidate action legality without the source board. Parquet is not supported.

## 1. Purpose

Define the data schema for all training and evaluation data used in the 2048 game machine learning pipeline.

## 2. Data Schema Overview

The data schema defines the structure of all data records, ensuring consistency across the pipeline.

```mermaid
flowchart TD
    subgraph "Data Schema — Classification Only"
        subgraph "State Record"
            Grid[Grid Features: 16 dims]
            Derived[Derived Features: 11 dims]
            Action[Action: 0-3<br/>ONLY label]
            ScoreMeta[Score: u64<br/>metadata only]
        end

        subgraph "Game Record"
            GameID[Game ID: integer]
            AgentType[Agent Type]
            MoveCount[Move Count]
            FinalScore[Final Score<br/>benchmark only]
            Timestamp[Timestamp]
        end

        subgraph "Training Record — Classification"
            State[State Vector: 27 dims<br/>X]
            Label[Action Label 0-3<br/>y — ONLY target]
        end
    end

    State --> Dataset[Unified Dataset<br/>TaskType::MultiClassification]
    Game -.->|metadata| Dataset
    Training --> Dataset

    style Dataset fill:#e3f2fd
```

## 3. State Feature Schema

```mermaid
flowchart TB
    subgraph "27-Dimensional State Vector"
        Grid[Grid Features<br/>Dimensions 0-15]
        Empty[Empty Count<br/>Dimension 16]
        MaxTile[Max Tile Log<br/>Dimension 17]
        Mono[Monotonicity<br/>Dimension 18]
        Smooth[Smoothness<br/>Dimension 19]
        Merges[Merges Available<br/>Dimension 20]
        ScoreNorm[Score Normalized<br/>Dimension 21]
        AdjMerge[Adjacency Merge Score<br/>Dimension 22]
        Corner[Corner Max<br/>Dimension 23]
        Edge[Edge Tiles Occupied<br/>Dimension 24]
        ColWorst[Column Worst<br/>Dimension 25]
        RowWorst[Row Worst<br/>Dimension 26]
    end
    
    Grid --> StateVec[State Vector<br/>[f64; 27]]
    Empty --> StateVec
    MaxTile --> StateVec
    Mono --> StateVec
    Smooth --> StateVec
    Merges --> StateVec
    ScoreNorm --> StateVec
    AdjMerge --> StateVec
    Corner --> StateVec
    Edge --> StateVec
    ColWorst --> StateVec
    RowWorst --> StateVec
    
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
    pub merges_available: f64,
    pub score_normalized: f64, // log10(score+1)/6.0
    pub adjacency_merge_score: f64,
    pub corner_max: f64,
    pub edge_tiles_occupied: f64,
    pub col_worst: f64,
    pub row_worst: f64,
    pub action: u8, // 0-3 classification label — ONLY y
    // score: u64 is OUTSIDE this struct — optional trailing CSV column `,score` for analysis only, never as y
    // No done / reward / next_state — those are RL fields and must not appear
}
/// Optional metadata (not in training CSV):
/// pub score: u64 — if stored, as trailing `,score` column after `action`, never as feature/label
```

## 5. Data Format Flow

```mermaid
flowchart TD
    Raw[Raw Board State]
    Raw --> Encode[Encode to 27-dim Vector]
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
    Validate --> Check1[Check Dimensions: 27]
    Validate --> Check2[Check Types: f64]
    Validate --> Check3[Check finite and per-feature ranges]
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
    Dir --> N01[01-data-schema.md]
    Dir --> N02[02-data-format.md]
    Dir --> N03[03-data-standard.md]
```

## 8. CSV Schema for AutoML — Training = 28 cols; `score` Is Metadata

> **Training CSV is exactly 28 columns** `grid_0..row_worst,action` — `score: u64` is **optional metadata** appended as trailing `,score` (29th col) for analysis only, **never** as `y` or feature. Training `DataFrame` mapping is `X: [f64;27]`, `y: u8` — `score` is not loaded as `y`. No `done`/`reward`/`next_state` columns.

The automl framework consumes data in CSV format. The following defines the exact schema expected.

### 8.1 Column Definitions

| Column Name | Data Type | Description |
|-------------|-----------|-------------|
| `grid_0` | `f64` | Tile value at grid position (0,0) |
| `grid_1` | `f64` | Tile value at grid position (0,1) |
| `grid_2` | `f64` | Tile value at grid position (0,2) |
| `grid_3` | `f64` | Tile value at grid position (0,3) |
| `grid_4` | `f64` | Tile value at grid position (1,0) |
| `grid_5` | `f64` | Tile value at grid position (1,1) |
| `grid_6` | `f64` | Tile value at grid position (1,2) |
| `grid_7` | `f64` | Tile value at grid position (1,3) |
| `grid_8` | `f64` | Tile value at grid position (2,0) |
| `grid_9` | `f64` | Tile value at grid position (2,1) |
| `grid_10` | `f64` | Tile value at grid position (2,2) |
| `grid_11` | `f64` | Tile value at grid position (2,3) |
| `grid_12` | `f64` | Tile value at grid position (3,0) |
| `grid_13` | `f64` | Tile value at grid position (3,1) |
| `grid_14` | `f64` | Tile value at grid position (3,2) |
| `grid_15` | `f64` | Tile value at grid position (3,3) |
| `empty_count` | `f64` | Number of empty tiles / 16 (0-1) |
| `max_tile_log` | `f64` | Log2 of the maximum tile value / 15 (0-1) |
| `monotonicity` | `f64` | Monotonicity score of the board (0-1) |
| `smoothness` | `f64` | Smoothness score of the board (0-1) |
| `merges_available` | `f64` | Number of possible merges / 16 (0-1) |
| `score_normalized` | `f64` | `log10(score+1)/6.0`; finite and non-negative, not globally capped at 1 |
| `adjacency_merge_score` | `f64` | Adjacent mergeable pair values / 16 (0-1) |
| `corner_max` | `f64` | Max tile in corner normalized (0-1) |
| `edge_tiles_occupied` | `f64` | Edge tiles occupied / 12 (0-1) |
| `col_worst` | `f64` | Minimum column sum normalized (0-1) |
| `row_worst` | `f64` | Minimum row sum normalized (0-1) |
| `action` | `u8` | Target label: 0=Up, 1=Down, 2=Left, 3=Right |

### 8.2 Data Formats

Supported formats for training data:
- **CSV** (`.csv`) — Primary format for automl ingestion
- Parquet — not implemented in the root data pipeline

### 8.3 Example Row

CSV has **27 feature columns + 1 action label = 28 columns total** (header + label). Canonical header uses 11 derived features in order:

```csv
grid_0,grid_1,grid_2,grid_3,grid_4,grid_5,grid_6,grid_7,grid_8,grid_9,grid_10,grid_11,grid_12,grid_13,grid_14,grid_15,empty_count,max_tile_log,monotonicity,smoothness,merges_available,score_normalized,adjacency_merge_score,corner_max,edge_tiles_occupied,col_worst,row_worst,action
0.0,0.0,0.00006,0.00012,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.875,0.2,0.85,0.72,0.125,0.15,0.03,0.06,0.5,0.02,0.03,2
```

### 8.4 Game State to CSV Row Mapping

Each CSV row represents a single **state-action pair** extracted from a game trajectory:

1. A game is played from start to finish (or until a terminal state).
2. At each step, the current board state is captured as the 27-dimensional feature vector.
3. The action taken at that step is recorded as the target label (`action` column).
4. The resulting row is added to the training dataset.
5. Multiple games are concatenated into a single CSV file.
6. Rows are stored as a flat table; game and move identifiers remain aligned in the sidecar for group splits and provenance.

```
Game 1:
  State(step 0) + Action(up)     → Row 1
  State(step 1) + Action(right)  → Row 2
  State(step 2) + Action(down)   → Row 3
  ...
Game 2:
  State(step 0) + Action(left)   → Row N
  ...
```

### 8.5 Data Type Summary

- All `grid_*` features: `f64` (tile values as f64, 0.0 for empty cells)
- All derived features: `f64` (continuous scores, normalized where applicable)
- `action` target: `u8` (integer class label: 0, 1, 2, or 3)
- CSV delimiter: `,` (comma)
- Header row: present (column names as defined above)
- Missing values: none expected; all features are deterministically computed from board state

## 9. Next Steps

1. Implement schema validation
2. Apply schema to all data pipelines
3. Generate training CSV from game replay data

## Implementation Record

- Exact 27-feature plus integer-action CSV schema is implemented, with separate score/game/move metadata. Validation rejects wrong header/width, nonfinite or out-of-range features, and invalid action IDs.
- Parquet is not implemented. CSV labels are not rechecked for legality against source boards because boards are not in the canonical training table. Tiles above 32768 remain a range-contract issue.

---

## Verification (definition of done)

1. `test -f plans/06-Data/02-Format/01-data-schema.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/06-Data/02-Format/01-data-schema.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/02-Format/01-data-schema.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/02-Format/01-data-schema.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/02-Format/01-data-schema.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/02-Format/01-data-schema.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/02-Format/01-data-schema.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/02-Format/01-data-schema.md` exits 0.

## Open questions

- Resolve the tile-range contract shared with the state encoder. If legality must be revalidated during ingestion, add a board-bearing audit format separately from canonical training features.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
