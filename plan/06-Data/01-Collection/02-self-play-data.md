# Self-Play Data

## 1. Purpose

Collect training data from self-play games where the AI agent plays against itself, generating high-quality game sequences.

## 2. Self-Play Overview

Self-play is the primary method for generating training data, as it produces games where the agent learns from its own experience.

```mermaid
flowchart TD
    subgraph "Self-Play Data Collection"
        Agent1[Agent Instance 1]
        Agent2[Agent Instance 2]
        
        Agent1 --> |Move| Board[Game Board]
        Agent2 --> |Move| Board
        Board --> |State| Agent1
        Board --> |State| Agent2
        
        Board --> Result[Game Result]
        Result --> Record[Record Game Data]
        Record --> Store[Store Dataset]
    end
    
    Agent1 --> |Both use same model| Agent2
    style Board fill:#e3f2fd
    style Store fill:#e8f5e9
```

## 3. Self-Play Process

```mermaid
flowchart TB
    Process[Self-Play Process]
    Process --> Init[Initialize Two Agents]
    Init --> Game[Start Game]
    Game --> Alternate[Alternate Moves]
    Alternate --> Check{Game Over?}
    Check --> |No| Alternate
    Check --> |Yes| Record[Record Game Result]
    Record --> Next{More Games?}
    Next --> |Yes| Init
    Next --> |No| Final[Finalize Dataset]
    
    style Final fill:#e8f5e9
```

## 4. Data Generation

```mermaid
flowchart TD
    subgraph "Data Per Game"
        State[Board State at Each Turn]
        Action[Action Taken]
        Score[Score After Move]
        NextState[Next Board State]
        Done[Game Over Flag]
        Reward[Reward Signal]
    end
    
    State --> Sample[Training Sample]
    Action --> Sample
    Score --> Sample
    NextState --> Sample
    Done --> Sample
    Reward --> Sample
    
    Sample --> Dataset[Training Dataset]
    
    style Sample fill:#fff3e0
    style Dataset fill:#e3f2fd
```

## 5. Self-Play Configuration

```rust
pub struct SelfPlayConfig {
    pub n_games: usize,              // Number of self-play games
    pub agent1: AgentType,           // First agent type
    pub agent2: AgentType,           // Second agent type
    pub save_interval: usize,        // Save every N games
    pub output_path: String,         // Output path: 06-Data/01-Collection/
    pub seed: u64,                   // Random seed
}
```

## 6. Self-Play Data Flow

```mermaid
sequenceDiagram
    participant AgentA as Agent A
    participant AgentB as Agent B
    participant Game as Game Engine
    participant Collector as Data Collector
    participant Store as Data Storage
    
    AgentA->>Game: Initialize game
    loop Game Loop
        AgentA->>Game: Make move
        Game->>Collector: Record state/action/reward
        AgentB->>Game: Make move
        Game->>Collector: Record state/action/reward
    end
    Game->>Collector: Final result
    Collector->>Store: Save to Parquet/CSV
```

## 7. Data Statistics

```mermaid
flowchart TD
    Stats[Self-Play Data Statistics]
    Stats --> Games[Total Games: 10,000]
    Stats --> States[Total States: ~500,000]
    Stats --> AvgLen[Avg Game Length: 50 moves]
    Stats --> MaxTile[Max Tile Achieved]
    Stats --> ScoreDist[Score Distribution]
    
    style Stats fill:#e3f2fd
```

## 8. Self-Play Data Files Location

```mermaid
flowchart LR
    Dir[06-Data/01-Collection]
    Dir --> 01[01-data-collection-strategy.md]
    Dir --> 02[02-self-play-data.md]
    Dir --> 03[03-random-play-data.md]
```

## 9. Next Steps

1. Configure self-play simulation
2. Run self-play games
3. Collect and store game data
