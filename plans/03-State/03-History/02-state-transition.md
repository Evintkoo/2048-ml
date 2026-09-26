# Plan 02 — State Transition: the repository status is explicit and evidence based

> **Status: NOT APPLICABLE (2026-09-26).** The supervised `(from_state, action)` row is implemented; separate post-state/reward/done transition metadata is outside the canonical training protocol.

**Goal:** State the current implementation and evidence boundary for state transition.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Scope-based disposition:** the canonical supervised example is the pre-action state and chosen action. The root stores that as `TrainingSample` plus sidecar metadata. It does not build a separate transition object with post-state, reward, or done fields; those fields are not used for training and are not needed for the supervised 2048 policy task.

> **Canonical row:** `(from_state: [f64;17], action: u8)` with `TaskType::MultiClassification`. No RL.

## 1. Structure — Training Fields vs Metadata

The implemented `TrainingSample` contains `state_features:[f64;17]`,
`action:u8`, raw score metadata, `game_id`, and `move_index`. Only state
features and action enter AutoML; the sidecar carries provenance and score.

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
    let states: Vec<[f64;17]> = transitions.iter().map(|t| t.from_state).collect();
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
    if !t.from_state.iter().all(|v| v.is_finite() && *v >= 0.0) { return Err("bad state".into()); }
    Ok(())
}
```

## 6. Persistence

Save training rows through the canonical CSV writer; row-aligned provenance stays in a metadata sidecar.

## Implementation Record

- Collection records the pre-action 17-value state and chosen action; score and game/move identifiers remain metadata. Invalid moves do not produce samples because valid actions are selected from `would_change`.
- `TrainingSample` provides the supervised row shape without introducing RL fields. Post-state/reward/done transitions are deliberately omitted under the supervised-only scope.

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
