# Action Constraints — Validity Only

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
