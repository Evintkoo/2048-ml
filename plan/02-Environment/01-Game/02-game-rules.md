# 2048 Game Rules Specification

## 1. Objective

Slide numbered tiles on a grid to combine them and create a tile with the number 2048. Every time two tiles with the same number collide, they merge into one tile with the sum of their values. The score is the total value of all merged tiles.

## 2. Initial State

- Grid: 4×4 cells, all initially empty
- Starting tiles: 2 random empty cells, each containing a value of 2 (90%) or 4 (10%)

## 3. Game Rules

### 3.1 Tile Movement

- All tiles slide as far as possible in the chosen direction
- Tiles stack against the edge or against other tiles
- Merging occurs when two tiles of the same value collide
- Only one merge per tile per move

### 3.2 Tile Spawning

- After each move, a new tile spawns in a random empty cell
- New tile value: 2 with probability 0.9, 4 with probability 0.1
- If no empty cells exist and no moves possible → Game Over

### 3.3 Scoring

- Score increases by the value of the merged tile
- Example: Two tiles of 128 merge → Score increases by 256
- The final score is the sum of all merges during the game

### 3.4 Win Condition

- Reaching a tile with value 2048 (win condition, not required to stop)
- The game can continue past 2048 to achieve higher scores
- No explicit upper limit on tile values

### 3.5 Loss Condition

- Board is completely filled
- No adjacent tiles have the same value
- No valid moves exist in any direction

## 4. Valid Move Definitions

```rust
// A move is valid if the board state changes after execution
// A move is invalid if the board remains identical
fn is_valid_move(board: &Board, direction: Direction) -> bool {
    let mut test_board = board.clone();
    test_board.execute_move(direction);
    test_board != *board
}
```

## 5. Move Outcomes

| Outcome | Description |
|---------|-------------|
| `Moved` | Board changed, new tile spawned |
| `NoChange` | Board unchanged (invalid move) |
| `GameOver` | No moves remaining |
| `Win` | 2048 tile created (optional) |

## 6. State Transitions

```
[Initial] → [Tile Spawn] → [Player Move] → [Tile Merge] → [Score Update] → [Tile Spawn] → ... → [Game Over]
```

## 7. Game States

```rust
pub enum GameState {
    Playing,        // Active game in progress
    Won(u64),       // 2048 reached, score
    Lost(u64),      // Game over, final score
    Aborted,        // Prematurely stopped
}
```

## 8. Edge Cases

1. **Empty board at start:** Always 2 initial tiles
2. **Immediate win:** Very unlikely with 2 starting tiles
3. **Full board no moves:** Game over even if 2048 tile exists
4. **Continuous 4 spawns:** Possible but statistically rare
5. **Board symmetry:** Some moves are symmetric equivalents

## 9. Rule Variants (Not Used)

| Variant | Description | Status |
|---------|-------------|--------|
| 8×8 grid | Larger board | Not used |
| 3×3 grid | Smaller board | Not used |
| Custom spawn rates | Different 2/4 probability | Not used |
| No 4 tiles | Only 2 tiles spawn | Not used |
