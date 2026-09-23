//! Project-level training helpers for group-preserving evaluation.

use automl::training::{
    CVStrategy, CrossValidator, ModelType, TaskType, TrainEngine, TrainingConfig,
};
use polars::prelude::{DataFrame, IdxCa, IdxSize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrainingError {
    #[error("group count {groups} does not match row count {rows}")]
    GroupLength { groups: usize, rows: usize },
    #[error("dataset requires at least {splits} distinct game groups")]
    TooFewGroups { splits: usize },
    #[error("grouped CV failed: {0}")]
    AutoMl(#[from] automl::AutoMLError),
    #[error("Polars dataframe operation failed: {0}")]
    Polars(#[from] polars::error::PolarsError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupedCvResult {
    pub fold_accuracy: Vec<f64>,
    pub mean_accuracy: f64,
    pub std_accuracy: f64,
}

/// Runs AutoML training independently on game-group folds. The framework's
/// `cross_val_score` helper does not accept groups, so this wrapper uses its
/// `CrossValidator` splitter and scores held-out rows directly.
pub fn grouped_cross_validate(
    data: &DataFrame,
    groups: &[i64],
    model: ModelType,
    seed: u64,
    n_splits: usize,
) -> Result<GroupedCvResult, TrainingError> {
    if groups.len() != data.height() {
        return Err(TrainingError::GroupLength {
            groups: groups.len(),
            rows: data.height(),
        });
    }
    let unique_groups: std::collections::HashSet<i64> = groups.iter().copied().collect();
    if unique_groups.len() < n_splits {
        return Err(TrainingError::TooFewGroups { splits: n_splits });
    }
    let group_array = ndarray::Array1::from_iter(groups.iter().copied());
    let validator =
        CrossValidator::new(CVStrategy::GroupKFold { n_splits }).with_random_state(seed);
    let folds = validator.split(data.height(), None, Some(&group_array))?;
    let mut fold_accuracy = Vec::with_capacity(folds.len());
    for fold in folds {
        let train_groups: std::collections::HashSet<i64> =
            fold.train_indices.iter().map(|&i| groups[i]).collect();
        if fold
            .test_indices
            .iter()
            .any(|&i| train_groups.contains(&groups[i]))
        {
            return Err(TrainingError::AutoMl(automl::AutoMLError::ValidationError(
                "a game group occurs in both training and validation fold".to_string(),
            )));
        }
        let train_indices = to_index_ca(&fold.train_indices)?;
        let test_indices = to_index_ca(&fold.test_indices)?;
        let train = data.take(&train_indices)?;
        let test = data.take(&test_indices)?;
        let config = TrainingConfig::new(TaskType::MultiClassification, "action")
            .with_model(model.clone())
            .with_random_state(seed)
            .with_cv(0);
        let mut engine = TrainEngine::new(config);
        engine.fit(&train)?;
        let predicted = engine.predict(&test)?;
        let actual = test
            .column("action")?
            .cast(&polars::prelude::DataType::Float64)?;
        let actual = actual.f64()?.into_no_null_iter();
        let correct = actual
            .zip(predicted.iter())
            .filter(|(a, p)| (*a - *p).abs() < 0.5)
            .count();
        fold_accuracy.push(correct as f64 / test.height() as f64);
    }
    let mean_accuracy = fold_accuracy.iter().sum::<f64>() / fold_accuracy.len() as f64;
    let variance = fold_accuracy
        .iter()
        .map(|score| (score - mean_accuracy).powi(2))
        .sum::<f64>()
        / fold_accuracy.len() as f64;
    Ok(GroupedCvResult {
        fold_accuracy,
        mean_accuracy,
        std_accuracy: variance.sqrt(),
    })
}

fn to_index_ca(indices: &[usize]) -> Result<IdxCa, TrainingError> {
    let indices: Result<Vec<IdxSize>, TrainingError> = indices
        .iter()
        .map(|&i| {
            IdxSize::try_from(i).map_err(|_| {
                TrainingError::AutoMl(automl::AutoMLError::InvalidInput(
                    "dataset row index exceeds Polars index capacity".to_string(),
                ))
            })
        })
        .collect();
    Ok(IdxCa::from_vec("idx".into(), indices?))
}
