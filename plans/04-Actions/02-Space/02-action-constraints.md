# Plan 02 — Action Constraints: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for action constraints.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Scope:** validity via `board.would_change`. Boundary/Merge invented checks and 3-checker overkill removed.

## 1. Validity — The Only Constraint

An action is valid iff it changes the board.

```rust
pub fn is_valid_action(board: &Board, dir: Direction) -> bool { board.would_change(dir) }
pub fn valid_actions(board: &Board) -> Vec<u8> {
    (0..4).filter(|&a| board.would_change(Direction::from_action(a).unwrap())).collect()
}
```

Canonical: `02-Environment/02-Rules/03-valid-moves.md` → `board.would_change` / `get_valid_moves`.

## 2. Usage with `masked_argmax`

```rust
let valid = valid_actions(&board);
let logits = model.predict(&state); // [f64;4]
let action = masked_argmax(&logits, &valid); // canonical — masks invalid
```

> **Deleted:** §4 Boundary, §5 Merge, `ActionConstraints` 3-checker — all redundant with `would_change`. Merges/boundaries are emergent consequences of whether the board changes; no separate checks needed.

## 3. Path References

- `01-Action/01-action-space.md` — action definitions.
- `03-Mapping/01-model-output-to-action.md` — `masked_argmax`.
- `02-Environment/02-Rules/` — game rules.

## Implementation Record

- Valid actions derive from `would_change`; model inference applies `masked_argmax` over the valid subset. Terminal boards return `NoValidActions`, and non-finite model scores or out-of-range action IDs are rejected.

---

## Verification (definition of done)

1. `test -f plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
6. `grep -q '^## Open questions$' plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
7. `grep -q '^## Later$' plans/04-Actions/02-Space/02-action-constraints.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/04-Actions/02-Space/02-action-constraints.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
