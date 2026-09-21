# Training Pipeline

## 1. Purpose

Define the complete training pipeline that takes collected game data and produces a trained supervised classification model capable of predicting optimal moves in the 2048 game.

## 2. Pipeline Overview

The training pipeline follows a structured flow from raw game data to a deployable classification model.

```mermaid
flowchart TD
    subgraph "Training Pipeline"
        Data[Raw Game Data]
        Preprocess[Preprocessing]
        Feature[Feature Engineering]
        Train[Model Training]
        Evaluate[Model Evaluation]
        Select[Model Selection]
        Export[Model Export]
    end
    
    Data --> Preprocess
    Preprocess --> Feature
    Feature --> Train
    Train --> Evaluate
    Evaluate --> Select
    Select --> Export
    
    Export --> Deploy[Deployed Classification Model]
```

## 3. Pipeline Stages

### 3.1 Data Ingestion

```mermaid
flowchart LR
    Collection[Game Data Collection<br/>06-Data/]
    Loading[Load Data<br/>CSV/Parquet]
    Validation[Validate Data<br/>State-Action Pairs]
    
    Collection --> Loading
    Loading --> Validation
```

### 3.2 Preprocessing

```mermaid
flowchart TD
    Raw[Raw Features]
    Raw --> Scaler[StandardScaler]
    Raw --> Encoder[OneHotEncoder]
    Raw --> Imputer[Mean Imputation]
    Scaler --> Processed
    Encoder --> Processed
    Imputer --> Processed
    
    Processed[Processed Features]
```

### 3.3 Training Execution

```mermaid
flowchart TD
    X[Features<br/>Board State]
    Y[Targets<br/>Action Labels]
    
    X --> Engine[TrainEngine]
    Y --> Engine
    Engine --> Model[Trained Classifier]
    
    subgraph "automl Engine"
        Engine[TrainEngine]
        Engine --> Config[TrainingConfig<br/>TaskType: MultiClassification]
        Engine --> Optimizer[HyperOptX Optimizer]
        Engine --> CV[CrossValidator]
    end
```

## 4. Pipeline Configuration

```mermaid
flowchart TB
    Config[TrainingConfig]
    Config --> Task[Task Type: MultiClassification]
    Config --> Model[Model Type: Auto]
    Config --> CV[Cross-Validation: 5-fold]
    Config --> Seed[Random Seed: 42]
    Config --> Split[Validation Split: 0.2]
    
    style Config fill:#e3f2fd
```

**Configuration Details:**
- **Task Type**: `MultiClassification` — 4 output classes (up, down, left, right)
- **Model Type**: `Auto` — automl selects best classifier from `ModelType` enum (DecisionTree, RandomForest, GradientBoosting, XGBoost, LightGBM, SVM, KNN, etc.)
- **Labels**: Integer-encoded actions where 0=up, 1=down, 2=left, 3=right

## 5. Theoretical-Limit-Oriented Training

The training pipeline is optimized for finding how close each model gets to the theoretical maximum:

1. Train each model with maximum budget (high n_estimators, early stopping disabled)
2. Run each trained model against the game environment until convergence
3. Track the maximum score achieved by each model
4. Compute proximity ratio: `max_score / theoretical_limit` (theoretical_limit = 32768)
5. Compare proximity ratios across models
6. The model closest to the theoretical limit wins

## 6. Pipeline Execution Flow

```mermaid
sequenceDiagram
    participant Data as Data Collector
    participant Preprocess as Preprocessor
    participant Train as TrainEngine
    participant Eval as Evaluator
    participant Select as Selector
    participant Export as Exporter
    
    Data->>Preprocess: Raw game state-action pairs
    Preprocess->>Train: Processed features & labels
    Train->>Eval: Trained classifier
    Eval->>Select: Classification metrics
    Select->>Export: Best model
    Export->>Deploy: Serialized classifier
```

## 6. Pipeline Monitoring

```mermaid
flowchart TD
    Monitor[Pipeline Monitor]
    Monitor --> Metrics[Track Metrics]
    Monitor --> Logs[Log Pipeline Events]
    Monitor --> Alerts[Alert on Failures]
    
    Metrics --> Dashboard[Training Dashboard]
    Logs --> Storage[Log Storage]
    Alerts --> Notification[Notification System]
```

## 7. Pipeline Files Location

All pipeline files are organized under `05-Model/02-Training/`:

```mermaid
flowchart LR
    Dir[05-Model/02-Training]
    Dir --> N01[01-training-pipeline.md]
    Dir --> N02[02-training-loop.md]
    Dir --> N03[03-model-architecture.md]
```

## 8. Next Steps

1. Configure training parameters for multi-class classification
2. Execute training loop with supervised game data
3. Validate model architecture and classification accuracy
