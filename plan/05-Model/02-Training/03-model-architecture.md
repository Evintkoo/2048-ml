# Model Architecture

## 1. Purpose

Define the architecture of the machine learning model used to predict optimal moves in the 2048 game.

## 2. Architecture Overview

The model architecture is designed to consume 25-dimensional board state features and output 4 directional actions.

```mermaid
flowchart TD
    Input[Input Layer<br/>25 Features] --> Hidden1[Hidden Layer 1<br/>64 neurons, ReLU]
    Hidden1 --> Hidden2[Hidden Layer 2<br/>32 neurons, ReLU]
    Hidden2 --> Hidden3[Hidden Layer 3<br/>16 neurons, ReLU]
    Hidden3 --> Output[Output Layer<br/>4 Actions, Softmax]
    
    style Input fill:#e3f2fd
    style Output fill:#e8f5e9
```

## 3. Model Architecture Components

### 3.1 Feature Input Layer

```mermaid
flowchart LR
    Grid[Grid Features<br/>16 dimensions] --> Concat[Concatenate]
    Derived[Derived Features<br/>9 dimensions] --> Concat
    Concat --> Input[Input Vector<br/>25 dimensions]
```

### 3.2 Hidden Layers

```mermaid
flowchart TD
    Layer1[Layer 1: Dense(64)]
    Layer1 --> Activation1[ReLU Activation]
    Activation1 --> Dropout1[Dropout 0.2]
    
    Layer2[Layer 2: Dense(32)]
    Layer2 --> Activation2[ReLU Activation]
    Activation2 --> Dropout2[Dropout 0.2]
    
    Layer3[Layer 3: Dense(16)]
    Layer3 --> Activation3[ReLU Activation]
    Activation3 --> Dropout3[Dropout 0.1]
    
    Layer1 --> Layer2
    Layer2 --> Layer3
```

### 3.3 Output Layer

```mermaid
flowchart TD
    Hidden3[Last Hidden Layer<br/>16 neurons] --> Output[Output Layer<br/>4 neurons]
    Output --> Softmax[Softmax]
    Softmax --> Action[Action Probabilities<br/>Up, Down, Left, Right]
    
    style Output fill:#e8f5e9
    style Action fill:#fff3e0
```

## 4. Model Variants

```mermaid
flowchart TB
    subgraph "Model Variants"
        A[Gradient Boosting]
        B[Random Forest]
        C[Neural Network]
        D[XGBoost]
        E[Logistic Regression]
    end
    
    A --> Evaluation
    B --> Evaluation
    C --> Evaluation
    D --> Evaluation
    E --> Evaluation
    
    Evaluation[Architecture Evaluation]
```

## 5. Neural Network Architecture Details

```rust
pub struct ModelArchitecture {
    pub input_dim: usize,           // 25 features
    pub hidden_layers: Vec<usize>,  // [64, 32, 16]
    pub output_dim: usize,          // 4 actions
    pub dropout_rates: Vec<f64>,    // [0.2, 0.2, 0.1]
    pub activation: ActivationType, // ReLU
}
```

```mermaid
flowchart TD
    NN[Neural Network Model]
    NN --> Config[Architecture Config]
    NN --> Init[Weight Initialization]
    NN --> Train[Trainable Layers]
    NN --> Loss[Loss Function]
    NN --> Optim[Optimizer]
    
    Config --> |25-64-32-16-4| Init
    Init --> |Xavier/He| Train
    Train --> |CrossEntropy| Loss
    Loss --> |Adam/AdamW| Optim
```

## 6. Model Integration

```mermaid
flowchart TD
    Model[Trained Model]
    Model --> Engine[TrainEngine]
    Engine --> Pipeline[Training Pipeline]
    Pipeline --> Data[06-Data/]
    
    Data --> |training data| Pipeline
    Pipeline --> |model| Model
    Model --> |predictions| Inference[InferenceEngine]
```

## 7. Architecture Files Location

All model architecture files are in `05-Model/02-Training/`:

```mermaid
flowchart LR
    Dir[05-Model/02-Training]
    Dir --> 01[01-training-pipeline.md]
    Dir --> 02[02-training-loop.md]
    Dir --> 03[03-model-architecture.md]
```

## 8. Next Steps

1. Select final model architecture
2. Configure hyperparameters in `05-Model/03-Hyperparameter-Optimization/`
3. Train and evaluate the model
