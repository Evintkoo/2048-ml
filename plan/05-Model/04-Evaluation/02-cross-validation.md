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
    end
    
    KFold --> |K=5| Stratified
    Stratified --> |Class Balance| Shuffle
    Shuffle --> |Random| Time
    Time --> |Sequential| Select[Select Strategy]
    
    style KFold fill:#e3f2fd
    style Stratified fill:#e8f5e9
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

## 4. Cross-Validation Configuration

```rust
use automl::{CrossValidator, CVStrategy};

let cv = CrossValidator::new()
    .with_k_folds(5)
    .with_strategy(CVStrategy::Stratified)
    .with_shuffle(true)
    .with_random_state(42);

let results = cv.cross_val_score(&engine, &x, &y)?;
```

## 5. Cross-Validation Process

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
    Dir --> 01[01-model-evaluation.md]
    Dir --> 02[02-cross-validation.md]
    Dir --> 03[03-metrics.md]
```

## 8. Next Steps

1. Define evaluation metrics
2. Execute cross-validation
3. Analyze results
