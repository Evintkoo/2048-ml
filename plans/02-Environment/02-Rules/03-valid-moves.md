# Valid Move Analysis

## 1. Move Validity

A move is valid if executing it changes the board state. Invalid moves (no change) should not be counted as game actions.

## 2. Move Enumeration

At any point in the game, there are 4 possible move directions:

| Direction | Description | Implementation |
|-----------|-------------|----------------|
| Up | Slide all tiles up | Compress columns top-down |
| Down | Slide all tiles down | Compress columns bottom-up |
| Left | Slide all tiles left | Compress rows left-to-right |
| Right | Slide all tiles right | Compress rows right-to-left |

## 3. Valid Move Detection — Canonical

> **Canonical valid-move check:** `board.would_change(dir)` — single source of truth. All other docs reference this.

```rust
pub fn get_valid_moves(board: &Board) -> Vec<Direction> {
    Direction::ALL
        .iter()
        .filter(|dir| board.would_change(**dir))
        .copied()
        .collect()
}
```

## 4. Move Statistics

For each game state, compute:

```rust
pub struct MoveAnalysis {
    pub valid_moves: Vec<Direction>,     // Available moves
    pub num_valid_moves: usize,          // Count (1-4)
    pub best_move: Option<Direction>,    // Best predicted move
    pub move_scores: [f64; 4],          // Predicted score for each direction
    pub move_feasibility: [bool; 4],     // Whether move changes board
}
```

## 5. Move Encoding for ML

The model outputs a probability distribution over 4 actions:

```rust
// Model output → Action mapping
// [0.1, 0.7, 0.1, 0.1] → Down (index 1)
// [0.0, 0.0, 0.9, 0.1] → Left (index 2)
pub fn output_to_action(outputs: &[f64; 4]) -> Direction {
    let max_idx = outputs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i, _)| i).unwrap();
    Direction::from_index(max_idx)
}
```

## 6. Action Constraints

- Always exactly 1 action selected (argmax)
- Invalid moves (would_change == false) should have probability 0
- The model should learn to avoid invalid moves

```rust
pub fn constrained_action(outputs: &[f64; 4], board: &Board) -> Direction {
    let valid = board.get_valid_moves();
    let mut constrained = [0.0f64; 4];
    let sum: f64 = valid.iter().map(|d| outputs[d as usize]).sum();
    for d in valid {
        constrained[d as usize] = outputs[d as usize] / sum;
    }
    output_to_action(&constrained)
}
```

## 7. Action Frequency Analysis

Track how often each move is chosen in optimal play:

| Move | Frequency (Optimal) | Rationale |
|------|---------------------|-----------|
| Left | ~60-80% | Most tile merging happens leftward |
| Up | ~20-40% | Secondary merging direction |
| Down | ~5-10% | Rarely optimal |
| Right | ~2-5% | Almost never optimal |

## 8. Move Sequence Analysis

For the ML model, sequences of moves matter:

```rust
pub struct MoveSequence {
    pub moves: Vec<Direction>,          // History of moves
    pub scores: Vec<u64>,               // Score after each move
    pub board_states: Vec<Board>,       // Board after each move
}
```

## 9. Optimal Move Strategies

Common heuristic strategies (for comparison with ML):

1. **Monotonicity strategy:** Maintain monotonic row/column ordering
2. **Corner strategy:** Keep max tile in a corner
3. **Smoothness strategy:** Minimize value differences between adjacent tiles
4. **Empty tiles strategy:** Maximize empty tile count
5. **ML strategy:** Neural network/automl predicts best move
