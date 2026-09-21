# Cross-Validation Protocol

> **Note:** This section defines the cross-validation protocol. No results are claimed. The Rust implementation code is a template, not a completed implementation.

## 2. Why Cross-Validation?

The standard train/test split approach has limitations:
- **Single split:** Performance estimate depends on which games are in train vs test
- **Overfitting to test set:** Model selection may overfit to a specific test set
- **High variance:** Different splits give different performance estimates

Cross-validation addresses these issues by:
- **Multiple splits:** Performance averaged across k splits
- **Reduced overfitting:** Model selection uses validation, not test data
- **Lower variance:** Average of k estimates has lower variance than single estimate

## 3. Cross-Validation Design

### 3.1 K-Fold Cross-Validation

**Standard K-Fold:** The training data is split into k folds. Each fold serves as validation once, with the remaining k-1 folds used for training.

**For the 2048 problem:**
- **k = 5** (standard choice)
- **k = 10** (for more precise estimates)

**Challenge:** The 2048 game data is sequential (games follow each other). Standard K-Fold assumes i.i.d. data. This must be handled carefully.

### 3.2 Time-Series Cross-Validation

Since the 2048 game data is sequential (games played in order), **time-series cross-validation** (also called forward chaining) will be used:

```
Fold 1: Train = [1] | Validation = [2]
Fold 2: Train = [1,2] | Validation = [3]
Fold 3: Train = [1,2,3] | Validation = [4]
...
Fold k: Train = [1,...,k-1] | Validation = [k]
```

This preserves the temporal ordering of games and prevents data leakage from future games into past validation sets.

### 3.3 Block Cross-Validation

To prevent games from the same "session" (same seed run) from appearing in both train and validation:

```
Block 1: Games from seed 42, sessions 1-10 → Train
Block 2: Games from seed 42, sessions 11-20 → Validation
Block 3: Games from seed 123, sessions 1-10 → Train
Block 4: Games from seed 123, sessions 11-20 → Validation
```

This prevents session-level data leakage.

## 4. Cross-Validation Implementation

### 4.1 Rust Implementation

A Rust implementation of the cross-validator will be developed as part of the project. The following is a **template specification**, not a completed implementation:

```rust
pub struct KFoldCrossValidator {
    pub k: usize,
    pub seeds: Vec<u64>,
    pub n_games_per_fold: usize,
    pub shuffle: bool,
}

impl KFoldCrossValidator {
    pub fn create_folds(&self, games: &[Game]) -> Vec<(Vec<Game>, Vec<Game>)> {
        // To be implemented
        unimplemented!()
    }
    
    pub fn cross_validate(&self, model: &Model, games: &[Game]) -> CrossValidationResult {
        // To be implemented
        unimplemented!()
    }
}
```

### 4.2 Cross-Validation Results Template

| Fold | Train Size | Validation Size | Mean Score | Std Dev | Training Time |
|------|-----------|-----------------|------------|---------|---------------|
| Fold 1 | TBD | TBD | TBD | TBD | TBD |
| Fold 2 | TBD | TBD | TBD | TBD | TBD |
| Fold 3 | TBD | TBD | TBD | TBD | TBD |
| Fold 4 | TBD | TBD | TBD | TBD | TBD |
| Fold 5 | TBD | TBD | TBD | TBD | TBD |
| **Average** | — | — | **TBD** | **TBD** | **TBD** |

### 4.3 Cross-Validation Metrics

For each fold, the following will be computed:
- Mean score
- Median score
- Std dev
- 95% CI
- Training time
- Inference speed

## 5. Comparison with Single Split

**Variance reduction** is expected to follow the standard formula:
```
Variance reduction = σ²_single / k
```

For k=5: Variance reduced by 5x
For k=10: Variance reduced by 10x

**Note:** These are theoretical expectations. Actual variance reduction will depend on the data structure.

## 6. Cross-Validation for Model Selection

Cross-validation will be used for model selection:

1. **Step 1:** For each model type (RF, GB, XGBoost, etc.), run k-fold CV
2. **Step 2:** Compare average CV scores across model types
3. **Step 3:** Select the model with the highest average CV score
4. **Step 4:** Evaluate the selected model on a held-out test set (10,000 games)

**Why not evaluate on CV scores?** Because CV scores are optimistically biased (model selected based on CV performance). The final evaluation must be on a held-out test set.

## 7. Cross-Validation for Hyperparameter Tuning

### 7.1 Nested Cross-Validation

For rigorous hyperparameter tuning, **nested cross-validation** will be used:

```
Outer loop (k=5): Model selection
    Inner loop (k=3): Hyperparameter tuning
```

**Outer loop:** 5 folds for model selection
**Inner loop:** 3 folds for hyperparameter tuning within each outer fold

This prevents hyperparameter overfitting to the validation set.

### 7.2 Nested CV Results Template

| Outer Fold | Best Model | Best Hyperparameters | CV Score | Test Score |
|------------|------------|---------------------|----------|------------|
| Fold 1 | TBD | TBD | TBD | TBD |
| Fold 2 | TBD | TBD | TBD | TBD |
| Fold 3 | TBD | TBD | TBD | TBD |
| Fold 4 | TBD | TBD | TBD | TBD |
| Fold 5 | TBD | TBD | TBD | TBD |
| **Average** | — | — | **TBD** | **TBD** |

## 8. Cross-Validation Decision Framework

### 8.1 When to Use Which CV Method

| Scenario | Method | k | Status |
|----------|--------|---|--------|
| Initial model evaluation | Standard K-Fold | 5 | Planned |
| Precise performance estimate | Standard K-Fold | 10 | Planned |
| Sequential game data | Time-Series CV | 5 | Planned |
| Prevent session-level leakage | Block CV | 5 | Planned |
| Model selection | Nested CV | Outer: 5, Inner: 3 | Planned |
| Hyperparameter tuning | Nested CV | Outer: 5, Inner: 3 | Planned |

### 8.2 Cross-Validation Quality Checks

1. **Convergence:** Does the CV score stabilize across folds?
2. **Consistency:** Is the CV score consistent across different seeds?
3. **Significance:** Is the CV score significantly different from baselines?
4. **Generalization:** Does the CV score generalize to unseen game instances?

All quality checks are pending experimentation.

## 9. Conclusion

Cross-validation is essential for rigorous model evaluation in the 2048 game. The protocol described here will ensure that model performance estimates are unbiased and generalizable once implemented and executed.

All results are pending experimentation. The protocol provides the framework, but actual results require data collection.
