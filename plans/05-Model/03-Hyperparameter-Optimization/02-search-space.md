# Search Space

## 1. Purpose

Define the complete search space for hyperparameter optimization of the 2048 game machine learning model.

## 2. Search Space Overview

The search space encompasses all hyperparameters that can be tuned to optimize model performance.

```mermaid
flowchart TD
    subgraph "Complete Search Space"
        subgraph "Tree-Based Models"
            NEst[N_estimators<br/>50-300]
            MD[max_depth<br/>3-10]
            LR[learning_rate<br/>0.01-0.5]
            SS[subsample<br/>0.5-1.0]
            CB[colsamples_bytree<br/>0.5-1.0]
            MN[min_child_weight<br/>1-10]
        end
        
        subgraph "Regularization"
            L1[L1 Regularization<br/>0-1.0]
            L2[L2 Regularization<br/>0-1.0]
            PT[Pruning Threshold<br/>0-0.5]
        end
    end
    
    NEst --> Optimize[Optimize]
    MD --> Optimize
    LR --> Optimize
    SS --> Optimize
    CB --> Optimize
    MN --> Optimize
    L1 --> Optimize
    L2 --> Optimize
    PT --> Optimize
```

## 3. Parameter Categories

### 3.1 Model Parameters

```mermaid
flowchart TB
    MP[Model Parameters]
    MP --> TreeP[Tree Parameters]
    MP --> LinP[Linear Parameters]
    
    TreeP --> NT[n_estimators]
    TreeP --> MD[max_depth]
    TreeP --> LR[learning_rate]
    TreeP --> SW[min_child_weight]
    
    LinP --> C[C parameter]
    LinP --> PT[penalty type]
```

### 3.2 Training Parameters

```mermaid
flowchart TB
    TP[Training Parameters]
    TP --> BS2[Batch Size]
    TP --> EP[Epochs]
    TP --> LR2[Learning Rate]
    TP --> OPT[Optimizer]
    TP --> SCH[Scheduler]
    TP --> WD[Weight Decay]
    
    BS2 --> |32-256| EP
    EP --> |10-500| LR2
    LR2 --> |0.0001-0.5| OPT
    OPT --> |Adam/AdamW| SCH
    SCH --> |Cosine/Step| WD
```

## 4. Search Space Visualization

```mermaid
flowchart TD
    Vis[Search Space Visualization]
    Vis --> Parallel[Parallel Coordinates]
    Vis --> Scatter[Scatter Plot]
    Vis --> Heatmap[Heatmap]
    Vis --> Bar[Bar Chart]
    
    Parallel --> Analysis[Parameter Importance]
    Scatter --> Analysis
    Heatmap --> Analysis
    Bar --> Analysis
    
    Analysis --> Best[Identify Best Region]
```

## 5. Search Space for automl HyperOptX

```rust
use automl::{SearchSpace, Parameter, ParameterType};

let mut search_space = SearchSpace::new();

// Tree-based model parameters
search_space.add(Parameter::new("n_estimators", ParameterType::Int(50, 300)));
search_space.add(Parameter::new("max_depth", ParameterType::Int(3, 10)));
search_space.add(Parameter::new("learning_rate", ParameterType::Float(0.01, 0.5)));
search_space.add(Parameter::new("subsample", ParameterType::Float(0.5, 1.0)));
search_space.add(Parameter::new("colsample_bytree", ParameterType::Float(0.5, 1.0)));
search_space.add(Parameter::new("min_child_weight", ParameterType::Int(1, 10)));

// Note: No neural-network branch — automl ModelType is classical ML only (smartcore/linfa); NN params removed


// Regularization parameters
search_space.add(Parameter::new("l1_regularization", ParameterType::Float(0.0, 1.0)));
search_space.add(Parameter::new("l2_regularization", ParameterType::Float(0.0, 1.0)));
```

## 6. Parameter Importance

```mermaid
flowchart TD
    Importance[Parameter Importance Analysis]
    Importance --> Rank[Rank Parameters]
    Rank --> Top[Top Parameters]
    Top --> Focus[Focus Search on Top]
    Focus --> Narrow[Narrow Search Space]
    
    style Importance fill:#e3f2fd
    style Top fill:#e8f5e9
    style Narrow fill:#fff3e0
```

## 7. Search Space Files

All search space definitions are in `05-Model/03-Hyperparameter-Optimization/`:

```mermaid
flowchart LR
    Dir[05-Model/03-Hyperparameter-Optimization]
    Dir --> N01[01-hyperparameter-search.md]
    Dir --> N02[02-search-space.md]
    Dir --> N03[03-pruning-strategy.md]
```

## 8. Next Steps

1. Define pruning strategy in `05-Model/03-Hyperparameter-Optimization/03-pruning-strategy.md`
2. Execute hyperparameter search
3. Select optimal configuration
