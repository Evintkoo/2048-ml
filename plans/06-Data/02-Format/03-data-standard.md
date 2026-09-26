# Plan 03 — Data Standard: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** CSV schema validation and game-group split utilities exist; the training/evaluation lifecycle is not aligned end to end.

**Goal:** State the current implementation and evidence boundary for data standard.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats the CSV structural contract and split utility as implemented with limitations.** Validation checks the fixed v2 header, finite/nonnegative values, and action IDs. The standalone split command creates chronological game-disjoint partitions; the training CLI instead uses a configurable chronological test tail and AutoML internal row validation. Values above one are permitted.

## 1. Purpose

Fixed contract for 2048 supervised MultiClassification CSVs. All other concerns (versioning, storage) live in `03-Storage/`.

## 2. Naming — Fixed v2 17-Feature Order

Canonical header (17 features + action = 18 columns): `grid_0,...,grid_15,score_normalized,action`. Raw score and game/move IDs are in a separate metadata sidecar.

## 3. Quality Contract

- **Validation:** header regex exact match; `NF==18`; `action ∈ {0,1,2,3}` ; features are finite and checked against current per-feature ranges; score feature is index 16 and may exceed one. The validator cannot check source-board legality.
- **Splits:** chronological game-level 70/15/15 holdout (14k/3k/3k canonical); keep every `game_id` intact. `GroupKFold` is reserved for group-preserving CV inside training and does not enforce chronological order. No fitted preprocessing is currently applied.
- **Normalization:** deterministic divisors are used; fitted `StandardScaler` is not in the root training path. Values above one are permitted for large tiles and scores.
- Versioning: see `03-Storage/02-data-versioning.md`. Storage paths: see `03-Storage/01-dataset-storage.md`.

## Implementation Record

- The canonical 18-column header, action ID range, finite/nonnegative checks, metadata alignment, and standalone chronological 70/15/15 game split are implemented. The splitter requires at least three games and keeps groups intact.
- Training uses a chronological test tail then grouped CV on development games; AutoML also applies a seeded row-level validation split. The CSV validator cannot check action legality without board snapshots. Values above one are valid under the canonical encoding.

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

- Align the training command, data split utility, and experiment protocol before reporting final test results. Implement classification diagnostics; retain exact data/metadata digests and split assignments.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
