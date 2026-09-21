# Cross-Validation

## 1. Purpose

Define the cross-validation strategy for robust evaluation of the 2048 game machine learning model.

## 2. Cross-Validation Overview

Cross-validation ensures the model generalizes well by training and evaluating on different data splits.

```mermaid
flowchart TD
    subgraph "5-Fold Cross Validation"
        Data[Full Dataset]
        Data --> Split[Split into 5 Folds]
        
        Split --> Fold1[Fold 1]
        Split --> Fold2[Fold 2]
        Split --> Fold3[Fold 3]
        Split --> Fold4[Fold 4]
        Split --> Fold5[Fold 5]
        
        Fold1 --> |Train: F2-F5| Train1[Train]
        Fold1 --> |Test: F1| Test1[Test]
        
        Fold2 --> |Train: F1,F3-F5| Train2[Train]
        Fold2 --> |Test: F2| Test2[Test]
        
        Fold3 --> |Train: F1-F2,F4-F5| Train3[Train]
        Fold3 --> |Test: F3| Test3[Test]
        
        Fold4 --> |Train: F1-F3,F5| Train4[Train]
        Fold4 --> |Test: F4| Test4[Test]
        
        Fold5 --> |Train: F1-F4| Train5[Train]
        Fold5 --> |Test: F5| Test5[Test]
        
        Train1 --> Score1[Score 1]
        Train2 --> Score2[Score 2]
        Train3 --> Score3[Score 3]
        Train4 --> Score4[Score 4]
        Train5 --> Score5[Score 5]
        
        Score1 --> Avg[Average Score<br/>± Std Dev]
        Score2 --> Avg
        Score3 --> Avg
        Score4 --> Avg
        Score5 --> Avg
    end
```

## 3. Cross-Validation Strategies

```mermaid
flowchart TB
    subgraph "CV Strategies"
        KFold[K-Fold Cross Validation]
        Stratified[Stratified K-Fold]
        Shuffle[Shuffle Split]
        Time[Time Series Split]
        Temporal[Temporal Game Split]
    end
    
    KFold --> |K=5| Stratified
    Stratified --> |Class Balance| Shuffle
    Shuffle --> |Random| Time
    Time --> |Sequential| Temporal
    Temporal --> |No Leakage| Select[Select Strategy]
    
    style KFold fill:#e3f2fd
    style Stratified fill:#e8f5e9
    style Temporal fill:#fff3e0
```

### 3.1 Stratified K-Fold

```mermaid
flowchart TD
    Data[Dataset]
    Data --> Stratify[Stratify by Score Range]
    Stratify --> Bin1[Low Score Bin]
    Stratify --> Bin2[Medium Score Bin]
    Stratify --> Bin3[High Score Bin]
    
    Bin1 --> Distribute[Distribute Across Folds]
    Bin2 --> Distribute
    Bin3 --> Distribute
    
    Distribute --> KFold[5 Stratified Folds]
    
    style KFold fill:#c8e6c9
```

### 3.2 Shuffle Split

```mermaid
flowchart TD
    Data[Dataset]
    Data --> Shuffle[Shuffle Data]
    Shuffle --> Split[Split: Train/Val/Test]
    Split --> Train[Train Set: 70%]
    Split --> Val[Validation: 15%]
    Split --> Test[Test Set: 15%]
    
    Train --> Model[Train Model]
    Val --> Tune[Tune Parameters]
    Test --> Eval[Evaluate]
```

### 3.3 Temporal Data Problem: Why Random CV Leaks

Game data has an inherent temporal structure that violates the i.i.d. assumption of standard K-Fold cross-validation. Each game produces a sequence of board states where later states depend on earlier states. Random CV shuffles all states together, causing the following leakage problems:

1. **Future state leakage**: A model trained on states from game epoch 50 can "predict" states from game epoch 10 (because they share similar board patterns), but this does not reflect real generalization to truly unseen game sequences
2. **Sequence contamination**: Adjacent moves within a single game are highly correlated — placing a train state from move 80 and a test state from move 82 in different folds means the model has seen essentially the same game trajectory
3. **Score correlation**: States from high-scoring games tend to cluster together (good opening sequences lead to good mid-game states), so random splitting can put correlated high-score states in both train and test sets

```mermaid
flowchart TD
    subgraph "Random CV — Leakage"
        subgraph "Game A (temporal sequence)"
            S1[State 1] --> S2[State 2] --> S3[State 3] --> S4[State 4]
        end
        subgraph "Random Shuffle"
            S3 --> Fold_Train[Train Fold]
            S4 --> Fold_Test[Test Fold]
        end
        S3 -.->|Highly correlated| S4
        style S3 fill:#ffcdd2
        style S4 fill:#ffcdd2
    end
    
    subgraph "Problem: Test state S4 is highly correlated with train state S3"
        Leakage[DATA LEAKAGE — Model sees similar states]
    end
```

### 3.4 Temporal Game-Aware Cross-Validation Strategy

To prevent data leakage, we use a **temporal game sequence split** strategy that respects the sequential nature of game data:

**Core Principle**: Entire games (or game sequences) are kept together within a single fold. Train and test sets contain completely separate game sessions.

**Split Strategy**:

1. **Game-level splitting**: Group all board states by their originating game ID
2. **Temporal ordering**: Sort games by their start timestamp or game counter
3. **Forward-chaining splits**: Train on earlier games, test on later games
4. **No overlap**: A game's states appear in exactly one fold (either all train or all test)

```mermaid
flowchart TD
    subgraph "Temporal Game-Aware CV"
        G1[Game 1<br/>States 1-50] -->|Train| Fold1_Train[Train Fold 1]
        G2[Game 2<br/>States 1-55] -->|Train| Fold1_Train
        G3[Game 3<br/>States 1-48] -->|Train| Fold1_Train
        G4[Game 4<br/>States 1-60] -->|Test| Fold1_Test[Test Fold 1]
        G5[Game 5<br/>States 1-52] -->|Test| Fold1_Test
        
        G1 -->|Train| Fold2_Train[Train Fold 2]
        G2 -->|Train| Fold2_Train
        G4 -->|Train| Fold2_Train
        G5 -->|Train| Fold2_Train
        G6[Game 6<br/>States 1-45] -->|Test| Fold2_Test[Test Fold 2]
        G7[Game 7<br/>States 1-58] -->|Test| Fold2_Test
    end
```

**Implementation**:

```rust
use automl::{CrossValidator, CVStrategy};

let cv = CrossValidator::new()
    .with_k_folds(5)
    .with_strategy(CVStrategy::Temporal)
    .with_group_column("game_id")  // Group by game, not individual states
    .with_shuffle(false)            // Preserve temporal order
    .with_random_state(42);

let results = cv.cross_val_score(&engine, &x, &y, Some(&groups))?;
```

**Fold definitions for 5-fold temporal CV**:

```
Total games: 100 (chronologically ordered)

Fold 1: Train = Games 1-80,  Test = Games 81-84
Fold 2: Train = Games 1-84,  Test = Games 85-88
Fold 3: Train = Games 1-88,  Test = Games 89-92
Fold 4: Train = Games 1-92,  Test = Games 93-96
Fold 5: Train = Games 1-96,  Test = Games 97-100
```

This ensures that every test set contains only games that occurred *after* all training games, preventing temporal leakage.

**Key Properties**:
- No game appears in both train and test sets
- Test games always chronologically follow training games
- Each fold uses an expanding window (training set grows over time)
- Game states within a test game are never seen during training

### 3.5 Stratified Temporal CV (Recommended)

For best results, combine stratification with temporal ordering:

```mermaid
flowchart TD
    Data[All Games<br/>Chronologically Sorted]
    Data --> Bin[Stratify by Score Range]
    Bin --> Low[Low Score Games]
    Bin --> Medium[Medium Score Games]
    Bin --> High[High Score Games]
    
    Low --> Temporal[Temporal Split Within Each Stratum]
    Medium --> Temporal
    High --> Temporal
    
    Temporal --> Fold[5 Stratified Temporal Folds]
    
    style Fold fill:#fff3e0
```

Each stratum (score range) is temporally split independently, ensuring both class balance and temporal consistency.

## 4. Cross-Validation Configuration

```rust
use automl::{CrossValidator, CVStrategy};

// Recommended: Temporal game-aware CV
let cv = CrossValidator::new()
    .with_k_folds(5)
    .with_strategy(CVStrategy::Temporal)
    .with_group_column("game_id")
    .with_shuffle(false)
    .with_random_state(42);

let results = cv.cross_val_score(&engine, &x, &y, Some(&groups))?;

// Alternative: Stratified K-Fold (if temporal ordering is not available)
let cv_stratified = CrossValidator::new()
    .with_k_folds(5)
    .with_strategy(CVStrategy::Stratified)
    .with_shuffle(true)
    .with_random_state(42);
```

**Configuration Notes**:
- Always specify `group_column` when game data has temporal structure
- Set `shuffle = false` for temporal CV to preserve ordering
- Use `Stratified` with score bins when game outcomes are imbalanced
- Never shuffle game IDs — this would cause temporal leakage

## 5. Cross-Validation Process

```mermaid
flowchart TD
    Process[Cross-Validation Process]
    Process --> Init[Initialize CV Config]
    Init --> Split[Create Temporal Splits<br/>by Game ID]
    Split --> Verify[Verify No Leakage<br/>Check Game Separation]
    Verify --> Loop{For Each Fold}
    Loop --> |Train| TrainModel[Train on Training Games]
    TrainModel --> |Test| TestModel[Test on Temporal Held-Out Games]
    TestModel --> Collect[Collect Metrics]
    Collect --> Loop
    Loop --> |All folds done| Aggregate[Aggregate Results]
    Aggregate --> Report[Generate Report]
    
    style Verify fill:#fff3e0
    style Aggregate fill:#e8f5e9
```

```mermaid
flowchart TD
    Process[Cross-Validation Process]
    Process --> Init[Initialize CV Config]
    Init --> Split[Create Data Splits]
    Split --> Loop{For Each Fold}
    Loop --> |Train| TrainModel[Train on Training Fold]
    TrainModel --> |Test| TestModel[Test on Validation Fold]
    TestModel --> Collect[Collect Metrics]
    Collect --> Loop
    Loop --> |All folds done| Aggregate[Aggregate Results]
    Aggregate --> Report[Generate Report]
    
    style Aggregate fill:#e8f5e9
```

## 6. Results Aggregation

```mermaid
flowchart TB
    FoldResults[Fold Results]
    FoldResults --> Fold1[Fold 1: R²=0.85]
    FoldResults --> Fold2[Fold 2: R²=0.82]
    FoldResults --> Fold3[Fold 3: R²=0.87]
    FoldResults --> Fold4[Fold 4: R²=0.84]
    FoldResults --> Fold5[Fold 5: R²=0.86]
    
    Fold1 --> Avg[Mean: 0.848]
    Fold2 --> Avg
    Fold3 --> Avg
    Fold4 --> Avg
    Fold5 --> Avg
    Avg --> Std[Std Dev: 0.018]
    
    style Avg fill:#e3f2fd
    style Std fill:#fff3e0
```

## 7. Cross-Validation Files

All cross-validation files are in `05-Model/04-Evaluation/`:

```mermaid
flowchart LR
    Dir[05-Model/04-Evaluation]
    Dir --> N01[01-model-evaluation.md]
    Dir --> N02[02-cross-validation.md]
    Dir --> N03[03-metrics.md]
```

## 8. Next Steps

1. Define evaluation metrics
2. Execute cross-validation with temporal splits
3. Analyze results
4. Verify no data leakage between folds
