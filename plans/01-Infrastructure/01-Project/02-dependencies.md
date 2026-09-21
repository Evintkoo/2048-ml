# Dependencies

## 1. Primary Dependency: automl Submodule

| Property | Value |
|----------|-------|
| Repository | `https://github.com/Evintkoo/automl` |
| Submodule Path | `automl/` |
| Version | 1.0.0 |
| Language | Rust 1.75+ |
| License | MIT |

### 1.1 automl Capabilities Used

| Feature | Module | Purpose |
|---------|--------|---------|
| TrainEngine | `src/training/engine.rs` | Core training loop |
| TrainingConfig | `src/training/config.rs` | Model configuration |
| ModelType enum | `src/training/config.rs` | 20+ algorithm types |
| HyperOptX | `src/optimizer/` | Hyperparameter optimization |
| InferenceEngine | `src/inference/` | Model predictions |
| DataPreprocessor | `src/preprocessing/` | Data normalization |
| CrossValidator | `src/training/cross_validation.rs` | Validation |
| ExperimentTracker | `src/tracking/` | Run tracking |
| ModelSerializer | `src/export/` | Model persistence |

### 1.2 Available Models in automl

- **Tree-based:** DecisionTree, RandomForest, GradientBoosting, XGBoost, LightGBM, CatBoost, ExtraTrees
- **Linear:** LinearRegression, LogisticRegression, Ridge, Lasso, ElasticNet, PolynomialRegression, SGD
- **Other:** SVM, KNN, NaiveBayes, AdaBoost, GaussianProcess
- **Clustering:** KMeans, DBSCAN, SOM
- **Auto:** Automatic model selection

### 1.3 automl API Access

```rust
// Training
let config = TrainingConfig::new(TaskType::MultiClassification, "action");
let mut engine = TrainEngine::new(config);
engine.fit(&df)?;
let predictions = engine.predict(&df)?;

// Hyperparameter Optimization
let opt_config = OptimizationConfig::default();
let mut hyperopt = HyperOptX::new(opt_config);
let best_params = hyperopt.optimize(&search_space)?;

// Inference
let inference = InferenceEngine::new();
let result = inference.predict(&model, &input)?;
```

## 2. Rust Dependencies (automl Cargo.toml)

| Crate | Version | Purpose |
|-------|---------|---------|
| ndarray | 0.16 | Array operations |
| polars | 0.46 | DataFrames |
| linfa | 0.7 | ML traits |
| smartcore | 0.3 | ML algorithms |
| rayon | 1.10 | Parallelism |
| tokio | 1.43 | Async runtime |
| axum | 0.7 | REST API |
| clap | 4.4 | CLI |

## 3. Submodule Configuration

```bash
# Add submodule
git submodule add https://github.com/Evintkoo/automl

# Update submodule
git submodule update --init --recursive

# Pull latest changes
cd automl && git pull origin main && cd ..
```

## 4. External Tools

| Tool | Purpose | Required |
|------|---------|----------|
| Rust 1.75+ | Build automl | Yes |
| cargo | Package manager | Yes |
| Python 3.10+ | Data scripts (optional) | No |
| git | Version control | Yes |
| make | Build automation | Yes |
