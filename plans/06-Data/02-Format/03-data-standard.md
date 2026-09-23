# Data Standard — Naming & Quality Contract

## 1. Purpose

Fixed contract for 2048 supervised MultiClassification CSVs. All other concerns (versioning, storage) live in `03-Storage/`.

## 2. Naming — Fixed 27-Feature Order

Canonical header (27+action = 28 cols): `grid_0,grid_1,...,grid_15,empty_count,max_tile_log,monotonicity,smoothness,merges_available,score_normalized,adjacency_merge_score,corner_max,edge_tiles_occupied,col_worst,row_worst,action` — order is **frozen**. `score: u64` is optional trailing metadata `,score`, never label.

## 3. Quality Contract

- **Validation:** header regex exact match; `NF==28` (or 29 with `score`); `action ∈ {0,1,2,3}` and in `valid_moves`; features `f64` finite, grid/derived in `[0,1]` where defined; **no missing values** — all 27 are deterministically computed.
- **Splits:** `GroupKFold` `groups=game_id` `shuffle=false` chronological 70/15/15 (14k/3k/3k canonical) — see `01-data-collection-strategy.md §8.8` and `DataPreprocessor` fit on train only.
- **Normalization:** deterministic divisors (`/32768` for grid, `/16` for counts, `log10(score+1)/6.0`, `log2(max)/15`) — see `04-Preprocessing/03-data-normalization.md` for optional `StandardScaler` variant.
- Versioning: see `03-Storage/02-data-versioning.md`. Storage paths: see `03-Storage/01-dataset-storage.md`.
