# Plan 01 — Board State Definition: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** The 16-cell board plus score contract is implemented; derived feature range behavior is being reconciled in ticket #034.

**Goal:** State the current implementation and evidence boundary for board state definition.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with one dependent feature-range question pending.** `RawBoardState` stores the board cells and score; `BoardStateMl::from_board` emits 27 values without move count or history. The adjacent feature-extraction ticket must resolve the declared 32768 tile scale against larger valid `u32` powers that the board currently accepts.

> **Scope:** The canonical training state is the 16 board cells plus score. No history or move count is included in training features. History may be retained for collection only; see `plans/00-scope-and-traceability.md`.
> **Canonical dims:** **27** = 16 grid + 11 derived. See `04-Encoding/01-state-vector.md:31` `create_state_vector`.

## 1. State Representation

The board state is the primary input to the ML model. Must capture all information needed to predict the optimal action `0..3` via `TaskType::MultiClassification`.

## 2. Raw Board State

```rust
/// 4×4 grid as flat array, 0 = empty — board cells plus score
pub struct RawBoardState {
    pub grid: [u32; 16],          // Powers of two represented by u32; feature scale assumes max 32768
    pub score: u64,               // Cumulative score — idx 21 after normalization
    pub move_count: u64,          // Metadata; excluded from the training vector
    pub game_over: bool,          // Metadata; excluded from the training vector
}
```

## 3. Extended Board State (for ML) — Canonical 27-dim

```rust
/// Actual root representation; formulas and feature ordering are audited in #034.
pub struct BoardStateMl(pub [f64; 27]);
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

- `RawBoardState` stores the flat 16-cell grid, cumulative score, move count, and terminal flag. `BoardStateMl::from_board` creates a 27-value feature vector; move count, terminal state, and history are excluded, with score represented only at index 21.
- Feature validation checks finiteness and ranges; randomized board and score-index tests exist. `RawBoardState::from_grid` currently accepts powers above 32768 even though other feature values are constrained to `[0,1]`; resolution is assigned to ticket #034 before this state contract is marked complete.

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
