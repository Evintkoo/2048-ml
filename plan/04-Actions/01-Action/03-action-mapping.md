# Action Mapping

## 1. Overview

Map between raw action representations and the internal action space used by the ML model.

## 2. Mapping Structure

```mermaid
flowchart LR
    Raw["Raw Action"] -->|Parse| Mapped["Mapped Action"]
    Mapped -->|Encode| Encoded["Encoded Action"]
    Encoded -->|autml Input| Model["ML Model"]
```

## 3. Direction to Action Mapping

```rust
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_action(&self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    pub fn from_action(action: u8) -> Option<Direction> {
        match action {
            0 => Some(Direction::Up),
            1 => Some(Direction::Down),
            2 => Some(Direction::Left),
            3 => Some(Direction::Right),
            _ => None,
        }
    }
}
```

## 4. Model Output to Action

The automl model outputs raw predictions that must be mapped to valid actions:

```rust
pub fn map_model_output(outputs: &[f64]) -> u8 {
    // outputs is a 4-element array (one per direction)
    // Return the index of the highest value
    outputs
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx as u8)
        .unwrap_or(0)
}
```

## 5. Mapping Validation

```rust
pub fn validate_mapping(action: u8) -> Result<()> {
    if action > 3 {
        return Err(format!("Invalid action: {}", action).into());
    }
    Ok(())
}
```

## 6. Integration with 03-State and 04-Actions

- `03-State/` modules provide the state context for action decisions
- `04-Actions/02-Space/` defines constraints on valid actions
- `04-Actions/03-Mapping/` handles the final policy mapping
