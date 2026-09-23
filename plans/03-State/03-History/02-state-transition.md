# Plan 02 — State Transition: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for state transition.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Canonical row:** `(from_state: [f64;27], action: u8)` with `TaskType::MultiClassification`. No RL.

## 1. Structure — Training Fields vs Metadata

```rust
pub struct StateTransition {
    // ── training (only these feed automl) ──
    pub from_state: [f64; 27],  // X — canonical 27
    pub action: u8,             // y — 0..3 ONLY label
    // ── metadata only (logged, never as y) ──
    pub metadata: Option<TransitionMetadata>,
}
pub struct TransitionMetadata {
    pub to_state: [f64; 27],  // state after move
    pub reward: f64,            // score delta
    pub done: bool,             // terminal
}
```

## 2. Recording

```rust
impl GameSimulator {
    pub fn record_transition(&mut self) -> StateTransition {
        let from = create_state_vector(&self.board); // canonical
        let action = self.last_action; // u8 0..3
        self.board.execute_move(action);
        StateTransition {
            from_state: from, action,
            metadata: Some(TransitionMetadata {
                to_state: create_state_vector(&self.board),
                reward: self.board.score as f64 - self.last_score as f64,
                done: self.board.is_game_over(),
            }),
        }
    }
}
```

## 3. Training Dataset — States → Actions Only

```rust
pub fn create_supervised_dataset(transitions: &[StateTransition]) -> TrainingData {
    let states: Vec<[f64;27]> = transitions.iter().map(|t| t.from_state).collect();
    let actions: Vec<u8> = transitions.iter().map(|t| t.action).collect(); // MultiClassification
    TrainingData { states, targets: actions }
}
```

DataFrame conversion flattens `from_state` + `action` only; metadata dropped. Group by `game_id` for `GroupKFold`.

## 4. No RL Replay Buffer

> No `ReplayBuffer`, no sampling. Supervised classification only — flatten directly.

## 5. Validation

```rust
pub fn validate_transition(t: &StateTransition) -> Result<()> {
    if t.action > 3 { return Err("action >3".into()); }
    if !t.from_state.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)) { return Err("bad state".into()); }
    Ok(())
}
```

## 6. Persistence

Save combined dataset via canonical DataFrame → Parquet. See `01-move-history.md` for grouping.

## Implementation Record

- Collection records the pre-action 27-value state and chosen action; score and game/move identifiers remain metadata. Invalid moves do not produce samples because valid actions are selected from `would_change`.
- A separate `StateTransition` type and post-action metadata tuple are not implemented; current `TrainingSample` provides the supervised row shape without introducing RL fields.

---

## Verification (definition of done)

1. `test -f plans/03-State/03-History/02-state-transition.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/03-State/03-History/02-state-transition.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/03-History/02-state-transition.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/03-History/02-state-transition.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/03-History/02-state-transition.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/03-History/02-state-transition.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/03-History/02-state-transition.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/03-History/02-state-transition.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
