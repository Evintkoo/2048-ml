# Plan 01 — Move History: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** Per-game actions and score deltas plus row provenance are retained; raw board-history export is outside the current training/audit scope.

**Goal:** State the current implementation and evidence boundary for move history.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This ticket's required move history and grouped provenance are implemented.** Training rows retain `game_id` and `move_index` in a sidecar, which is enough for game-group splitting. Per-move pre-state snapshots are used in memory for rollout relabeling but are not persisted as a separate history dataset; raw snapshots are not required by the current supervised training or audit protocol, and history is not a training feature.

> **Not for training.** History is never fed as features. Each row for training is `(from_state: [f64;17], action: u8)` — see `02-state-transition.md`. This file = grouped raw log for `GroupKFold` leakage prevention.

## 1. Record — Minimal

```rust
// Current `MoveRecord` stores action and score_delta in each GameResult.
// Training metadata separately stores row_index, game_id, move_index, score.
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

Raw board-history export is not part of the current CSV training path. If persisted pre-state snapshots become necessary for an audit, define their format and provenance requirements in a separate ticket.

## Implementation Record

- Each `GameResult` retains action and score-delta move history. Collected supervised rows retain `game_id` and `move_index` in a separate metadata CSV, preserving row grouping for game-level splits.
- Raw pre-state snapshots are held only while rollout relabeling runs, then discarded. They are not model features and are not persisted. A separate audit-log format has not been specified.

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

- No required move-history deliverable remains open. If a later audit requires persisted board snapshots, define a separate format and provenance ticket.

## Later

- **No additional history persistence is required for the current supervised case study.** Keep history out of the model input unless a separately documented study changes Plan 00.
