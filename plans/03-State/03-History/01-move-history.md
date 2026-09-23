# Plan 01 — Move History: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for move history.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Not for training.** History is never fed as features. Each row for training is ` (from_state: [f64;27], action: u8)` — see `02-state-transition.md`. This file = grouped raw log for `GroupKFold` leakage prevention.

## 1. Record — Minimal

```rust
pub struct MoveRecord {
    pub game_id: u64,              // group key for GroupKFold
    pub move_idx: u32,             // sequential within game
    pub board_before: [u32; 16],   // raw tiles (0 = empty)
    pub action: u8,                // 0..3 — the label
    // board_after/score_after/timestamp/merge_value are not stored here (see state-transition for optional metadata)
}
```

## 2. Collection — Grouped by `game_id`

```rust
pub struct MoveHistory { pub records: Vec<MoveRecord> }
impl MoveHistory {
    pub fn add(&mut self, game_id: u64, board: &[u32;16], action: u8) {
        let idx = self.records.iter().filter(|r| r.game_id == game_id).count() as u32;
        self.records.push(MoveRecord { game_id, move_idx: idx, board_before: *board, action });
    }
}
```

Group by `game_id` so `GroupKFold` never splits moves from the same game across train/val.

## 3. Training Row

Canonical row: `03-History/02-state-transition.md` + `04-Encoding/01-state-vector.md:31` `create_state_vector`. History itself is **not** a feature.

## 4. Quality Checks

```rust
pub fn validate_history(h: &MoveHistory) -> Result<()> {
    for r in &h.records { if r.action > 3 { return Err("action >3".into()); } }
    Ok(())
}
```

## 5. Persistence

Save combined `records` as Parquet grouped by `game_id` — not per-move DataFrames. No `to_dataframe` stub; use canonical `create_supervised_dataset`.

## Implementation Record

- Each `GameResult` retains action and score-delta move history. Collected supervised rows retain `game_id` and `move_index` in a separate metadata CSV, preserving row grouping for game-level splits.
- Raw `board_before` per move and Parquet history persistence are not implemented. The current feature-state samples are captured separately during collection.

---

## Verification (definition of done)

1. `test -f plans/03-State/03-History/01-move-history.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/03-State/03-History/01-move-history.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/03-History/01-move-history.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/03-History/01-move-history.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/03-History/01-move-history.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/03-History/01-move-history.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/03-History/01-move-history.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/03-History/01-move-history.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
