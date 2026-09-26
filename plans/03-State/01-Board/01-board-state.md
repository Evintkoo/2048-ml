# Plan 01 — Board State Definition: the repository status is explicit and evidence based

> **Status: DONE (2026-09-26).** The canonical 17-value state is implemented; board tiles above the former 32768 normalization scale remain valid inputs.

**Goal:** State the current implementation and evidence boundary for board state definition.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented.** `RawBoardState` stores the board cells and score; `BoardStateMl::from_board` emits 16 board cells followed by current score encoding. Move count and game history are excluded. Values above one are valid when tile values exceed 32768 or score exceeds one million; validation requires finite, nonnegative values.

> **Scope:** The canonical training state is the 16 board cells plus score. No history or move count is included in training features. History may be retained for collection only; see `plans/00-scope-and-traceability.md`.
> **Canonical dims:** **17** = 16 row-major board cells plus `score_normalized` at index 16. The previous 27-column vector is not the canonical training state.

## 1. State Representation

The board state is the primary input to the ML model. Must capture all information needed to predict the optimal action `0..3` via `TaskType::MultiClassification`.

## 2. Raw Board State

```rust
/// 4×4 grid as flat array, 0 = empty — board cells plus score
pub struct RawBoardState {
    pub grid: [u32; 16],          // Powers of two represented by u32
    pub score: u64,               // Cumulative score — encoded at state index 16
    pub move_count: u64,          // Metadata; excluded from the training vector
    pub game_over: bool,          // Metadata; excluded from the training vector
}
```

## 3. Canonical Board State for ML — 17 Values

```rust
/// 16 row-major cells plus log-normalized current score.
pub struct BoardStateMl(pub [f64; 17]);
```

### 3.1 Separate Strategic Metrics

Strategic metrics formerly appended to the 27-value vector are not canonical
training inputs. Any retained measurements are heuristic-only or exploratory;
ticket #035 records their separate implementation status.

## 4. State Encoding — Canonical

### 4.1 Vector Assembly (17-dim)

`BoardStateMl::from_board` returns `[f64;17]`: the 16 row-major tile values
divided by 32768, followed by `(score + 1).log10() / 6` at index 16.
The implementation is in `src/state.rs`; the later state-vector ticket must
align its description with this canonical contract.

### 4.2 Normalization — Canonical Divisors

| Feature | Formula | Divisor | Range |
|---------|---------|---------|-------|
| Grid indices `0..15` | `tile as f64 / 32768.0` | 32768 | finite, nonnegative; may exceed 1 |
| Score index `16` | `(score as f64 + 1.0).log10() / 6.0` | 6 (log10) | finite, nonnegative; may exceed 1 |

Deterministic divisors (no fitted scaler). `automl` `ScalerType::Standard` is applied on top only if needed; for tree models `ScalerType::None` is acceptable — see `04-Encoding/02-normalization.md`.

## 5. State Properties (Canonical)

| Property | Type | Storage | Notes |
|----------|------|---------|-------|
| Grid values | [u32; 16] | features 0..15 | tile values /32768; may exceed 1 |
| Score | u64 | feature 16 `log10/6` plus raw metadata | never a target, `action` is label |
| Move count | u64 | metadata | excluded from the 17-value input |
| Game over | bool | metadata | excluded from the 17-value input |

> **No `StateWithHistory`.** Sequence history is **not in the training state**. If needed for collection/debug, see `03-History/01-move-history.md`; history is never fed as features.

## Implementation Record

- `RawBoardState` stores the flat 16-cell grid, cumulative score, move count, and terminal flag. `BoardStateMl::from_board` creates a 17-value vector: normalized cells at indices 0–15 and log-normalized current score at index 16. Move count, terminal state, and history are excluded.
- Feature validation requires finite, nonnegative values and permits values above one for large legal tiles and scores. This matches `RawBoardState::from_grid`, which accepts powers of two beyond 32768.

## 6. State Validation

```rust
impl BoardStateMl {
    pub fn validate(&self) -> Result<(), String> {
        for (i, &v) in self.0.iter().enumerate() {
            if !v.is_finite() {
                return Err(format!("feature {} is not finite: {}", i, v));
            }
            if v < 0.0 {
                return Err(format!("feature {} is negative: {}", i, v));
            }
        }
        // Grid values must be 0 or power of 2 (checked on raw grid before normalization)
        Ok(())
    }
}
```

## 7. State Persistence

```rust
fn save_state(state: &BoardStateMl, path: &str) -> Result<()> {
    let json = serde_json::to_string(state)?;
    std::fs::write(path, json)?;
    Ok(())
}
```

This persistence example is illustrative; `BoardStateMl` does not currently
implement serde serialization.

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

- **State encoding is complete under the canonical scope.** The former 32768 scale is a divisor, not a game tile cap. The canonical model vector has 17 values; larger tile and score encodings remain finite and are accepted.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
