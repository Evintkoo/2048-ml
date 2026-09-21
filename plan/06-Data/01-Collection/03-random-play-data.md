# Random Play Data

## 1. Purpose

Collect training data from random play games to provide baseline data and ensure diverse coverage of the board state space.

## 2. Random Play Overview

Random play data provides a broad sampling of the state-action space, which is essential for training models that can handle diverse board configurations.

```mermaid
flowchart TD
    subgraph "Random Play Data Collection"
        RandomAgent[Random Agent]
        Game[Game Engine]
        Collector[Data Collector]
        Store[Data Storage]
        
        RandomAgent --> |Random Moves| Game
        Game --> |States| Collector
        Collector --> Store
        
        style RandomAgent fill:#fff3e0
        style Store fill:#e8f5e9
    end
```

## 3. Random Play Process

```mermaid
flowchart TB
    Process[Random Play Process]
    Process --> Init[Initialize Random Agent]
    Init --> Game[Start Game]
    Game --> Move[Random Move Selection]
    Move --> Check{Game Over?}
    Check --> |No| Move
    Check --> |Yes| Record[Record Game Data]
    Record --> Next{More Games?}
    Next --> |Yes| Init
    Next --> |No| Final[Finalize Dataset]
    
    style Final fill:#e8f5e9
```

## 4. Random Agent Implementation

```rust
pub struct RandomAgent {
    rng: ChaCha8Rng,
}

impl Agent for RandomAgent {
    fn select_move(&self, board: &Board) -> Direction {
        let valid = board.get_valid_moves();
        valid[self.rng.gen_range(0..valid.len())]
    }
}
```

## 5. Data Collection Pipeline

```mermaid
flowchart TD
    Pipeline[Random Play Pipeline]
    Pipeline --> Config[Configure Agent]
    Config --> Simulate[Simulate Games]
    Simulate --> Record[Record Each Move]
    Record --> Validate[Validate Data]
    Validate --> Store[Store to 06-Data/03-Storage/]
    
    Config --> |Random Agent| Simulate
    Simulate --> |Game Results| Record
    Record --> |State/Action/Reward| Validate
    Validate --> |Clean Data| Store
    
    style Config fill:#e3f2fd
    style Store fill:#e8f5e9
```

## 6. Random Play Data Characteristics

```mermaid
flowchart TB
    subgraph "Data Characteristics"
        Coverage[Wide State Coverage]
        Volume[High Volume]
        Noise[Higher Noise Level]
        Diversity[Diverse Board States]
    end
    
    Coverage --> Value[Good for exploration]
    Volume --> Value
    Noise --> Value
    Diversity --> Value
    
    style Coverage fill:#e3f2fd
    style Value fill:#fff3e0
```

## 7. Random Play vs Self-Play

```mermaid
flowchart TD
    Comparison[Random vs Self-Play]
    Comparison --> Random[Random Play]
    Comparison --> SelfPlay[Self-Play]
    
    Random --> |Pros| RP1[Wide coverage]
    Random --> |Pros| RP2[Unbiased]
    Random --> |Cons| RC1[High noise]
    Random --> |Cons| RC2[Low quality]
    
    SelfPlay --> |Pros| SP1[High quality]
    SelfPlay --> |Pros| SP2[Learned strategy]
    SelfPlay --> |Cons| SC1[Narrow distribution]
    SelfPlay --> |Cons| SC2[Mode collapse]
```

## 8. Data Collection Volume

```mermaid
flowchart TB
    Volume[Volume Targets]
    Volume --> NGames[10,000 games]
    Volume --> NStates[~500,000 states]
    Volume --> Format[CSV/Parquet]
    
    Volume --> Storage[06-Data/03-Storage/01-dataset-storage.md]
    
    style Volume fill:#e3f2fd
    style Storage fill:#e8f5e9
```

## 9. Random Play Data Files Location

```mermaid
flowchart LR
    Dir[06-Data/01-Collection]
    Dir --> 01[01-data-collection-strategy.md]
    Dir --> 02[02-self-play-data.md]
    Dir --> 03[03-random-play-data.md]
```

## 10. Next Steps

1. Run random play simulations
2. Validate collected data
3. Store in data storage
