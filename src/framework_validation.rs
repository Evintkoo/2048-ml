//! Executable capability smoke checks for the pinned AutoML dependency.

pub mod benchmark;

#[cfg(test)]
mod tests {
    use automl::{
        optimizer::{HyperOptX, OptimizationConfig, OptimizeDirection, Parameter, SearchSpace},
        training::{CVStrategy, CrossValidator, ModelType, TaskType, TrainEngine, TrainingConfig},
    };
    use ndarray::Array1;
    use polars::prelude::{Column, DataFrame};

    fn tiny_multiclass_data() -> DataFrame {
        let mut columns = Vec::new();
        for feature in 0..crate::state::STATE_FEATURES {
            let values: Vec<f64> = (0..80)
                .map(|row| ((row * (feature + 1) + feature) % 101) as f64 / 100.0)
                .collect();
            columns.push(Column::new(format!("f{feature}").into(), values));
        }
        let actions: Vec<u32> = (0..80).map(|row| (row % 4) as u32).collect();
        columns.push(Column::new("action".into(), actions));
        DataFrame::new(columns).expect("smoke dataset should have consistent column lengths")
    }

    fn first_json_difference(
        left: &serde_json::Value,
        right: &serde_json::Value,
        path: &str,
    ) -> Option<String> {
        match (left, right) {
            (serde_json::Value::Object(left), serde_json::Value::Object(right)) => {
                let keys: std::collections::BTreeSet<_> = left.keys().chain(right.keys()).collect();
                keys.into_iter()
                    .find_map(|key| match (left.get(key), right.get(key)) {
                        (Some(left), Some(right)) => {
                            first_json_difference(left, right, &format!("{path}.{key}"))
                        }
                        _ => Some(format!("{path}.{key}: object key differs")),
                    })
            }
            (serde_json::Value::Array(left), serde_json::Value::Array(right)) => {
                if left.len() != right.len() {
                    return Some(format!(
                        "{path}: array length {} != {}",
                        left.len(),
                        right.len()
                    ));
                }
                left.iter()
                    .zip(right)
                    .enumerate()
                    .find_map(|(index, (left, right))| {
                        first_json_difference(left, right, &format!("{path}[{index}]"))
                    })
            }
            _ if left != right => Some(format!("{path}: {left} != {right}")),
            _ => None,
        }
    }

    #[test]
    fn planned_multiclass_model_variants_fit_and_predict() {
        let data = tiny_multiclass_data();
        let candidates = [
            (ModelType::RandomForest, 4),
            (ModelType::DecisionTree, 2),
            (ModelType::ExtraTrees, 4),
            (ModelType::AdaBoost, 4),
            (ModelType::KNN, 4),
            (ModelType::NaiveBayes, 4),
            (ModelType::LogisticRegression, 2),
            (ModelType::SGD, 2),
            (ModelType::SVM, 2),
            (ModelType::GradientBoosting, 2),
            (ModelType::XGBoost, 2),
            (ModelType::LightGBM, 2),
            (ModelType::CatBoost, 2),
        ];
        for (model, expected_probability_columns) in candidates {
            let config = TrainingConfig::new(TaskType::MultiClassification, "action")
                .with_model(model.clone())
                .with_n_estimators(4)
                .with_max_depth(3)
                .with_random_state(42);
            let mut engine = TrainEngine::new(config);
            engine
                .fit(&data)
                .unwrap_or_else(|error| panic!("{model:?} failed: {error}"));
            let predictions = engine.predict(&data).expect("fit model should predict");
            assert_eq!(predictions.len(), data.height());
            assert!(predictions
                .iter()
                .all(|action| (0.0..=3.0).contains(action)));
            let probabilities = engine
                .predict_proba(&data)
                .unwrap_or_else(|error| panic!("{model:?} probability output failed: {error}"));
            assert_eq!(
                probabilities.ncols(),
                expected_probability_columns,
                "{model:?} probability output changed"
            );
            if matches!(model, ModelType::RandomForest) {
                let path =
                    std::env::temp_dir().join(format!("2048-ml-model-{}.json", std::process::id()));
                engine.save(path.to_str().unwrap()).unwrap();
                assert_eq!(
                    engine.predict(&data).unwrap(),
                    predictions,
                    "RandomForest predictions changed before serialization"
                );
                let loaded = TrainEngine::load(path.to_str().unwrap()).unwrap();
                let before_state = serde_json::to_value(engine.model()).unwrap();
                let after_state = serde_json::to_value(loaded.model()).unwrap();
                if let Some(difference) =
                    first_json_difference(&before_state, &after_state, "model")
                {
                    panic!("RandomForest model state changed after save/load: {difference}");
                }
                assert_eq!(
                    loaded.predict(&data).unwrap(),
                    predictions,
                    "RandomForest predictions changed after save/load"
                );
                let policy = crate::policy::ModelPolicy::load(path.to_str().unwrap()).unwrap();
                let (board, _) = crate::game_engine::RawBoardState::with_seed(42, 0.1).unwrap();
                let selected = policy.select_move(&board).unwrap();
                assert!(board.get_valid_moves().contains(&selected));
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    #[test]
    fn identical_seed_random_forest_fits_produce_identical_predictions() {
        let data = tiny_multiclass_data();
        let config = TrainingConfig::new(TaskType::MultiClassification, "action")
            .with_model(ModelType::RandomForest)
            .with_n_estimators(4)
            .with_max_depth(3)
            .with_random_state(42);
        let mut reference = TrainEngine::new(config.clone());
        reference.fit(&data).unwrap();
        let expected = reference.predict(&data).unwrap();

        for _ in 0..20 {
            let mut repeated = TrainEngine::new(config.clone());
            repeated.fit(&data).unwrap();
            assert_eq!(repeated.predict(&data).unwrap(), expected);
        }
    }

    #[test]
    fn grouped_cv_and_tpe_optimizer_smoke() {
        let groups = Array1::from_iter((0..40).map(|row| (row / 4) as i64));
        let validator = CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 });
        let folds = validator.split(40, None, Some(&groups)).unwrap();
        assert_eq!(folds.len(), 5);
        for fold in folds {
            for &test_index in &fold.test_indices {
                assert!(!fold
                    .train_indices
                    .iter()
                    .any(|&train_index| groups[train_index] == groups[test_index]));
            }
        }

        let search_space = SearchSpace::new().add(Parameter::float("x", -1.0, 1.0));
        let config = OptimizationConfig::default()
            .with_n_trials(12)
            .with_direction(OptimizeDirection::Maximize);
        let mut optimizer = HyperOptX::new(config, search_space);
        let study = optimizer
            .optimize(|params| {
                let x = params.get("x").and_then(|value| value.as_float()).unwrap();
                Ok(-(x - 0.25).powi(2))
            })
            .unwrap();
        assert_eq!(study.trials.len(), 12);
        assert!(study.best_value().unwrap() <= 0.0);

        let data = tiny_multiclass_data();
        let row_groups: Vec<i64> = (0..data.height()).map(|row| (row / 4) as i64).collect();
        let cv = crate::training::grouped_cross_validate(
            &data,
            &row_groups,
            ModelType::RandomForest,
            42,
            5,
        )
        .unwrap();
        assert_eq!(cv.fold_accuracy.len(), 5);
        assert!(cv
            .fold_accuracy
            .iter()
            .all(|score| (0.0..=1.0).contains(score)));
    }
}
