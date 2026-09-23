# Move History — Data Collection Detail

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
