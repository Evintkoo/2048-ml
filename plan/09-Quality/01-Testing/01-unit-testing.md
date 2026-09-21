# Unit Testing

## 1. Purpose

Define unit testing procedures for the 2048 ML system components.

## 2. Unit Testing Architecture

```mermaid
flowchart TD
    subgraph "Unit Testing Framework"
        subgraph "Test Cases"
            T1[Board State Tests]
            T2[Score Tests]
            T3[Move Logic Tests]
            T4[Model Tests]
        end
        
        subgraph "Test Runner"
            TR[Test Runner<br/>Execute All Tests]
            RF[Result Formatter]
        end
        
        subgraph "Reporting"
            RP[Pass/Fail Report]
            CO[Coverage Report]
        end
        
        T1 --> TR
        T2 --> TR
        T3 --> TR
        T4 --> TR
        TR --> RF
        RF --> RP
        RF --> CO
    end
```

## 3. Test Structure

```mermaid
flowchart TD
    A[Setup] --> B[Execute Test]
    B --> C[Assert Expected]
    C --> D{Could Pass?}
    D -->|Yes| E[Test Passes]
    D -->|No| F[Test Fails]
    E --> G[Log Result]
    F --> G
```

## 4. Test Categories

| Category | Description | Priority |
|----------|-------------|----------|
| Board State | Board initialization, updates | High |
| Move Logic | Slide and merge operations | High |
| Score | Score calculation, tracking | Medium |
| Model | Model predictions, training | High |
| Configuration | Config validation, loading | Medium |

## 5. Test Implementation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_board_initialization() {
        let board = Board::new(4);
        assert_eq!(board.size(), 4);
        assert!(board.is_empty());
    }
    
    #[test]
    fn test_merge_operation() {
        let mut board = Board::new(4);
        board.set_tile(0, 0, 2);
        board.set_tile(0, 1, 2);
        let result = board.merge_left();
        assert!(result.merged);
    }
}
```

## 6. Test Coverage

```mermaid
graph TD
    A[Board Module] -->|80% covered| Cover1[✓]
    B[Score Module] -->|90% covered| Cover2[✓]
    C[Move Module] -->|85% covered| Cover3[✓]
    D[Model Module] -->|75% covered| Cover4[⚠]
    
    style D fill:#f99,stroke:#333
```

## 7. Test Execution Pipeline

```mermaid
flowchart LR
    A[Write Tests] --> B[Run Test Suite]
    B --> C{All Pass?}
    C -->|Yes| D[Generate Coverage Report]
    C -->|No| E[Debug and Fix]
    E --> B
    D --> F[Commit]
```

## 8. Quality Gates

1. All tests must pass before merge
2. Code coverage must be ≥ 80%
3. Test names must be descriptive
4. Each function must have at least one test

## 9. Continuous Integration

Unit tests run automatically on every commit:

```mermaid
flowchart LR
    A[Commit] --> B[CI Pipeline]
    B --> C[Run Unit Tests]
    C --> D{All Pass?}
    D -->|Yes| E[Build Artifact]
    D -->|No| F[Report Failure]
```
