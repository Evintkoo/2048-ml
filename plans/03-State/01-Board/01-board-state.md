# Plan 01 — Board State Definition: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for board state definition.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Scope:** The canonical training state is the 16 board cells plus score. No history or move count is included in training features. History may be retained for collection only; see `plans/00-scope-and-traceability.md`.
> **Canonical dims:** **27** = 16 grid + 11 derived. See `04-Encoding/01-state-vector.md:31` `create_state_vector`.

## 1. State Representation

The board state is the primary input to the ML model. Must capture all information needed to predict the optimal action `0..3` via `TaskType::MultiClassification`.

## 2. Raw Board State

```rust
/// 4×4 grid as flat array, 0 = empty — board cells plus score
pub struct RawBoardState {
    pub grid: [u32; 16],          // Tile values (powers of 2, up to 131072 = 2^17)
    pub score: u64,               // Cumulative score — idx 21 after normalization
    pub game_over: bool,          // Terminal flag — metadata only
    // NOTE: move_count NOT in 27-dim vector (removed; low predictive value)
}
```

## 3. Extended Board State (for ML) — Canonical 27-dim

```rust
/// Board state with derived features for ML training — 27 dims total (16 + 11)
pub struct BoardStateML {
    // Raw grid features (16 dims) — each / 32768.0
    pub grid_values: [f64; 16],

    // Derived features (11 dims) — canonical order, indices 16..26
    pub empty_count: f64,              // [16] empty tiles / 16
    pub max_tile_log: f64,             // [17] log2(max_tile) / 15; zero tile maps to 0
    pub monotonicity: f64,             // [18] monotonicity score [0,1]
    pub smoothness: f64,               // [19] 1/(1+diff/100) [0,1]
    pub merges_available: f64,         // [20] cells participating in an available merge / 16
    pub score_normalized: f64,         // [21] log10(score+1)/6 — ONLY score feature
    pub adjacency_merge_score: f64,    // [22] sum adjacent equal / (16 * 32768)
    pub corner_max: f64,               // [23] corner tile / 32768
    pub edge_tiles_occupied: f64,      // [24] edge occupied / 12
    pub col_worst: f64,                // [25] min col sum / 8192
    pub row_worst: f64,                // [26] min row sum / 8192

    // Total: 27 dims. No move_count_norm, no history.
}
```

### 3.1 Derived Features Detail

See `02-feature-extraction.md §3` for formulas. Two additions over naive set:

- **`merges_available`** counts unique non-empty cells with at least one equal horizontal or vertical neighbor, divided by 16; a cell with two matching neighbors is counted once.
- **`adjacency_merge_score`**: Sum of values for each horizontally/vertically adjacent equal pair, divided by `(16 * 32768)` so the feature stays in `[0,1]` at the declared tile scale.
- **`monotonicity` provisional operational definition:** fraction of the 24 adjacent horizontal/vertical comparisons that are equal or include an empty cell. Freeze this before training; do not silently change it between datasets.
- **`col_worst` / `row_worst`**: `min(col_sum)` / `8192`, `min(row_sum)` / `8192` — captures imbalance.

```rust
pub fn adjacency_merge_score(board: &Board) -> f64 {
    let mut score = 0.0;
    for i in 0..4 {
        for j in 0..4 {
            let val = board.grid[i][j].unwrap_or(0);
            if val == 0 { continue; }
            if j < 3 && board.grid[i][j+1].unwrap_or(0) == val { score += val as f64; }
            if i < 3 && board.grid[i+1][j].unwrap_or(0) == val { score += val as f64; }
        }
    }
    score / (16.0 * 32768.0)
}
pub fn column_worst(board: &Board) -> f64 {
    let mut min_sum = u64::MAX;
    for j in 0..4 {
        let sum: u64 = (0..4).map(|i| board.grid[i][j].unwrap_or(0) as u64).sum();
        min_sum = min_sum.min(sum);
    }
    min_sum as f64 / 8192.0
}
```

## 4. State Encoding — Canonical

### 4.1 Vector Assembly (27-dim)

```rust
impl BoardStateML {
    pub fn to_array(&self) -> [f64; 27] {
        let mut arr = [0.0f64; 27];
        arr[0..16].copy_from_slice(&self.grid_values);
        arr[16] = self.empty_count;
        arr[17] = self.max_tile_log;
        arr[18] = self.monotonicity;
        arr[19] = self.smoothness;
        arr[20] = self.merges_available;
        arr[21] = self.score_normalized; // log10(score+1)/6
        arr[22] = self.adjacency_merge_score;
        arr[23] = self.corner_max;
        arr[24] = self.edge_tiles_occupied;
        arr[25] = self.col_worst;
        arr[26] = self.row_worst;
        arr
    }
}
```
Canonical impl: `04-Encoding/01-state-vector.md:31` `create_state_vector`.

### 4.2 Normalization — Canonical Divisors

| Feature | Formula | Divisor | Range |
|---------|---------|---------|-------|
| grid `0..15` | `v as f64 / 32768.0` | 32768 | [0,1] |
| empty_count | `empty as f64 / 16.0` | 16 | [0,1] |
| max_tile_log | `0 if max==0 else log2(max) / 15.0` | 15 (log2) | [0,1] through max tile 32768 |
| merges_available | `mergeable_cells as f64 / 16.0` | 16 | [0,1] |
| score_normalized | `(score as f64 + 1.0).log10() / 6.0` | 6 (log10) | [0,1] |
| corner_max | `corner as f64 / 32768.0` | 32768 | [0,1] |
| edge_tiles_occupied | `occupied as f64 / 12.0` | 12 | [0,1] |
| col_worst / row_worst | `min_sum as f64 / 8192.0` | 8192 | [0,1] |
| adjacency_merge_score | `sum_equal_adjacent / (16.0 * 32768.0)` | 16 × 32768 | [0,1] through max tile 32768 |
| monotonicity/smoothness | already [0,1] | — | [0,1] |

Deterministic divisors (no fitted scaler). `automl` `ScalerType::Standard` is applied on top only if needed; for tree models `ScalerType::None` is acceptable — see `04-Encoding/02-normalization.md`.

## 5. State Properties (Canonical)

| Property | Type | Storage | Notes |
|----------|------|---------|-------|
| Grid values | [u32; 16] | feature 0..15 | tile values, /32768 |
| Score | u64 | feature 21 `log10/6` | never a target, `action` is label |
| Empty tiles | derived | feature 16 | /16 |
| Max tile | derived | feature 17 | log2/15 |
| Game over | bool | metadata | not in 27-dim |

> **No `StateWithHistory`.** Sequence history is **not in the training state**. If needed for collection/debug, see `03-History/01-move-history.md` (grouped by `game_id` for `GroupKFold`) — history is never fed as features.

## Implementation Record

- `RawBoardState` stores the flat 16-cell grid, cumulative score, move count, and terminal flag. `BoardStateMl::from_board` creates the canonical 27-value feature vector; score and history remain metadata except for the single normalized score feature.
- Validation checks finiteness and feature ranges, with score index 21 allowed to exceed 1 for scores above one million. Unit coverage includes randomized board feature values.

## 6. State Validation

```rust
impl BoardStateML {
    pub fn validate(&self) -> Result<(), String> {
        let arr = self.to_array();
        for (i, &v) in arr.iter().enumerate() {
            if !v.is_finite() {
                return Err(format!("feature {} is not finite: {}", i, v));
            }
            if v < 0.0 || (i != 21 && v > 1.0) {
                return Err(format!("feature {} out of [0,1]: {}", i, v));
            }
        }
        // Grid values must be 0 or power of 2 (checked on raw grid before normalization)
        Ok(())
    }
}
```

## 7. State Persistence

```rust
fn save_state(state: &BoardStateML, path: &str) -> Result<()> {
    let json = serde_json::to_string(state)?;
    std::fs::write(path, json)?;
    Ok(())
}
```

---

## Verification (definition of done)

1. `test -f plans/03-State/01-Board/01-board-state.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/03-State/01-Board/01-board-state.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/01-Board/01-board-state.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/01-Board/01-board-state.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/01-Board/01-board-state.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/01-Board/01-board-state.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/01-Board/01-board-state.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/01-Board/01-board-state.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
