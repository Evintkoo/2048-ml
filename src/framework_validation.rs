//! Executable capability smoke checks for the pinned AutoML dependency.

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
        for feature in 0..27 {
            let values: Vec<f64> = (0..80)
                .map(|row| ((row * (feature + 1) + feature) % 101) as f64 / 100.0)
                .collect();
            columns.push(Column::new(format!("f{feature}").into(), values));
        }
        let actions: Vec<u32> = (0..80).map(|row| (row % 4) as u32).collect();
        columns.push(Column::new("action".into(), actions));
        DataFrame::new(columns).expect("smoke dataset should have consistent column lengths")
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
                let loaded = TrainEngine::load(path.to_str().unwrap()).unwrap();
                assert_eq!(loaded.predict(&data).unwrap(), predictions);
                let policy = crate::policy::ModelPolicy::load(path.to_str().unwrap()).unwrap();
                let (board, _) = crate::game_engine::RawBoardState::with_seed(42, 0.1).unwrap();
                let selected = policy.select_move(&board).unwrap();
                assert!(board.get_valid_moves().contains(&selected));
                std::fs::remove_file(path).unwrap();
            }
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
