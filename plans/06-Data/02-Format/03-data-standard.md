# Plan 03 — Data Standard: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for data standard.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Fixed contract for 2048 supervised MultiClassification CSVs. All other concerns (versioning, storage) live in `03-Storage/`.

## 2. Naming — Fixed 27-Feature Order

Canonical header (27+action = 28 cols): `grid_0,grid_1,...,grid_15,empty_count,max_tile_log,monotonicity,smoothness,merges_available,score_normalized,adjacency_merge_score,corner_max,edge_tiles_occupied,col_worst,row_worst,action` — order is **frozen**. `score: u64` is optional trailing metadata `,score`, never label.

## 3. Quality Contract

- **Validation:** header regex exact match; `NF==28` (or 29 with `score`); `action ∈ {0,1,2,3}` and in `valid_moves`; features `f64` finite, grid/derived in `[0,1]` where defined; **no missing values** — all 27 are deterministically computed.
- **Splits:** chronological game-level 70/15/15 holdout (14k/3k/3k canonical); keep every `game_id` intact. `GroupKFold` is reserved for group-preserving CV inside training and does not enforce chronological order. Fit preprocessing on train only.
- **Normalization:** deterministic divisors (`/32768` for grid, `/16` for counts, `log10(score+1)/6.0`, `log2(max)/15`) — see `04-Preprocessing/03-data-normalization.md` for optional `StandardScaler` variant.
- Versioning: see `03-Storage/02-data-versioning.md`. Storage paths: see `03-Storage/01-dataset-storage.md`.

## Implementation Record

- The canonical 28-column header, action range, per-feature finiteness/range checks, metadata sidecar alignment, and chronological game-level 70/15/15 split are implemented. The splitter requires at least three games and keeps game groups intact.
- Training currently performs a chronological final-game holdout then grouped CV on the development partition. Split reproducibility is seed- and row-order-dependent; source commit/hash/seed are recorded in manifests where available.

---

## Verification (definition of done)

1. `test -f plans/06-Data/02-Format/03-data-standard.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/06-Data/02-Format/03-data-standard.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/02-Format/03-data-standard.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/02-Format/03-data-standard.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/02-Format/03-data-standard.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/02-Format/03-data-standard.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/02-Format/03-data-standard.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/02-Format/03-data-standard.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
