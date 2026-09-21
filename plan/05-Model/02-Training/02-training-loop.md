# Training Loop

## 1. Purpose

Define the iterative training loop that drives the machine learning model training process for the 2048 game.

## 2. Training Loop Overview

The training loop iteratively updates model weights using the automl framework's TrainEngine.

```mermaid
flowchart TD
    subgraph "Training Loop"
        Initialize[Initialize Model]
        Loop{Training Loop}
        
        Initialize --> Loop
        Loop -->|Epoch| Load[Load Batch Data]
        Load --> Forward[Forward Pass]
        Forward --> Loss[Calculate Loss]
        Loss --> Backward[Backward Pass]
        Backward --> Update[Update Weights]
        Update --> Validate[Validate]
        Validate -->|Improve| Loop
        Validate -->|No Improvement| EarlyStop{Early Stop?}
        EarlyStop -->|Yes| Save[Save Model]
        EarlyStop -->|No| Loop
    end
```

## 3. Loop Architecture

```mermaid
flowchart TD
    subgraph "Training Loop Components"
        DataLoader[DataLoader]
        Model[Model]
        Optimizer[Optimizer]
        LossFn[Loss Function]
        Validator[Validator]
        Scheduler[Scheduler]
    end
    
    DataLoader -->|batch| Model
    Model -->|predictions| LossFn
    LossFn -->|gradient| Optimizer
    Optimizer -->|updated params| Model
    Model -->|checkpoint| Validator
    Validator -->|metrics| Scheduler
```

## 4. Training Stages

### 4.1 Initialization

```mermaid
flowchart LR
    Config[Load TrainingConfig]
    Config --> Init[Initialize TrainEngine]
    Init --> Load[Load Dataset]
    Load --> Split[Split Train/Val/Test]
    Split --> Begin[Begin Training]
```

### 4.2 Main Training Iteration

```mermaid
flowchart TB
    Iter[Training Iteration]
    Iter --> Step1[Step 1: Forward Pass]
    Step1 --> Step2[Step 2: Loss Calculation]
    Step2 --> Step3[Step 3: Backward Pass]
    Step3 --> Step4[Step 4: Gradient Update]
    Step4 --> Step5[Step 5: Validation]
    Step5 --> Step6[Step 6: Log Metrics]
    Step6 --> Next{More Epochs?}
    Next -->|Yes| Iter
    Next -->|No| Final[Finalize Training]
```

## 5. Training Loop Implementation

```rust
pub struct TrainingLoop {
    engine: TrainEngine,
    config: TrainingConfig,
    data: DataFrame,
}

impl TrainingLoop {
    pub fn run(&mut self) -> TrainingResult {
        let mut best_score = 0.0;
        let mut patience_counter = 0;
        
        for epoch in 0..self.config.n_epochs {
            // Forward pass
            let predictions = self.engine.predict(&self.data)?;
            let loss = self.calculate_loss(&predictions);
            
            // Backward pass
            self.engine.backward(&loss)?;
            self.engine.update_weights()?;
            
            // Validation
            let val_score = self.validate()?;
            
            if val_score > best_score {
                best_score = val_score;
                patience_counter = 0;
                self.save_checkpoint()?;
            } else {
                patience_counter += 1;
            }
            
            if patience_counter >= self.config.early_stopping_rounds {
                break;
            }
        }
        
        TrainingResult { best_score }
    }
}
```

## 6. Convergence Monitoring

```mermaid
flowchart TD
    Monitor[Convergence Monitor]
    Monitor --> Track[Track Loss Curve]
    Monitor --> Check[Check Convergence]
    Check -->|Converged| Stop[Stop Training]
    Check -->|Diverging| Adjust[Adjust Learning Rate]
    Check -->|Plateau| Patience[Increment Patience]
    
    style Stop fill:#e8f5e9
    style Adjust fill:#fff3e0
```

## 7. Checkpoint Management

```mermaid
flowchart TD
    Checkpoint[Checkpoint Manager]
    Checkpoint --> Save[Save Best Model]
    Checkpoint --> Load[Load Checkpoint]
    Checkpoint --> Resume[Resume Training]
    Checkpoint --> Rollback[Rollback on Failure]
    
    Save --> Storage[Model Storage]
    Load --> Training[Continue Training]
```

## 8. Training Files

All training loop files are in `05-Model/02-Training/`:

```mermaid
flowchart LR
    Dir[05-Model/02-Training]
    Dir --> N01[01-training-pipeline.md]
    Dir --> N02[02-training-loop.md]
    Dir --> N03[03-model-architecture.md]
```

## 9. Next Steps

1. Define model architecture
2. Execute training loop
3. Monitor convergence
4. Save best model
