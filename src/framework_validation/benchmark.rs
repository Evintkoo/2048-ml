//! Fixed-split evaluation on the named UCI tabular datasets.

use anyhow::{bail, Context, Result};
use automl::training::{ModelType, TaskType, TrainEngine, TrainingConfig};
use polars::prelude::{Column, DataFrame};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Dataset {
    name: &'static str,
    source_file: &'static str,
    target_name: &'static str,
    labels: Vec<&'static str>,
    rows: Vec<(usize, Vec<f64>, usize)>,
}

impl Dataset {
    fn read(name: &'static str, data_dir: &Path) -> Result<Self> {
        match name {
            "iris" => Self::read_iris(data_dir),
            "wine" => Self::read_wine(data_dir),
            "breast_cancer_wisconsin_diagnostic" => Self::read_wdbc(data_dir),
            _ => bail!("unknown framework dataset {name}"),
        }
    }

    fn read_iris(data_dir: &Path) -> Result<Self> {
        let path = data_dir.join("iris/iris.data");
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let labels = ["Iris-setosa", "Iris-versicolor", "Iris-virginica"];
        let mut rows = Vec::new();
        for (line_no, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let fields: Vec<_> = line.split(',').collect();
            if fields.len() != 5 {
                bail!(
                    "{}:{} expected five comma-separated fields",
                    path.display(),
                    line_no + 1
                );
            }
            let label = labels
                .iter()
                .position(|label| *label == fields[4])
                .with_context(|| {
                    format!("{}:{} unknown Iris class", path.display(), line_no + 1)
                })?;
            let features = fields[..4]
                .iter()
                .map(|field| field.parse::<f64>())
                .collect::<std::result::Result<Vec<_>, _>>()
                .with_context(|| {
                    format!("{}:{} invalid numeric feature", path.display(), line_no + 1)
                })?;
            rows.push((line_no, features, label));
        }
        Ok(Self {
            name: "iris",
            source_file: "iris/iris.data",
            target_name: "species",
            labels: labels.to_vec(),
            rows,
        })
    }

    fn read_wine(data_dir: &Path) -> Result<Self> {
        let path = data_dir.join("wine/wine.data");
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let labels = ["cultivar_1", "cultivar_2", "cultivar_3"];
        let mut rows = Vec::new();
        for (line_no, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let fields: Vec<_> = line.split(',').collect();
            if fields.len() != 14 {
                bail!(
                    "{}:{} expected 14 comma-separated fields",
                    path.display(),
                    line_no + 1
                );
            }
            let class_id = fields[0]
                .parse::<usize>()
                .with_context(|| format!("{}:{} invalid class", path.display(), line_no + 1))?;
            let label = class_id
                .checked_sub(1)
                .filter(|&id| id < labels.len())
                .with_context(|| {
                    format!("{}:{} class id outside 1..=3", path.display(), line_no + 1)
                })?;
            let features = fields[1..]
                .iter()
                .map(|field| field.parse::<f64>())
                .collect::<std::result::Result<Vec<_>, _>>()
                .with_context(|| {
                    format!("{}:{} invalid numeric feature", path.display(), line_no + 1)
                })?;
            rows.push((line_no, features, label));
        }
        Ok(Self {
            name: "wine",
            source_file: "wine/wine.data",
            target_name: "cultivar",
            labels: labels.to_vec(),
            rows,
        })
    }

    fn read_wdbc(data_dir: &Path) -> Result<Self> {
        let path = data_dir.join("breast_cancer_wisconsin_diagnostic/wdbc.data");
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let labels = ["B", "M"];
        let mut rows = Vec::new();
        for (line_no, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let fields: Vec<_> = line.split(',').collect();
            if fields.len() != 32 {
                bail!(
                    "{}:{} expected ID, diagnosis, and 30 features",
                    path.display(),
                    line_no + 1
                );
            }
            let label = labels
                .iter()
                .position(|label| *label == fields[1])
                .with_context(|| format!("{}:{} unknown diagnosis", path.display(), line_no + 1))?;
            let features = fields[2..]
                .iter()
                .map(|field| field.parse::<f64>())
                .collect::<std::result::Result<Vec<_>, _>>()
                .with_context(|| {
                    format!("{}:{} invalid numeric feature", path.display(), line_no + 1)
                })?;
            rows.push((line_no, features, label));
        }
        Ok(Self {
            name: "breast_cancer_wisconsin_diagnostic",
            source_file: "breast_cancer_wisconsin_diagnostic/wdbc.data",
            target_name: "diagnosis",
            labels: labels.to_vec(),
            rows,
        })
    }

    fn split(&self, seed: u64, test_fraction: f64) -> Result<(Vec<usize>, Vec<usize>)> {
        if !test_fraction.is_finite() || !(0.0..1.0).contains(&test_fraction) {
            bail!("test fraction must be finite and strictly between zero and one");
        }
        let mut by_class: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (row_index, (_, _, label)) in self.rows.iter().enumerate() {
            by_class.entry(*label).or_default().push(row_index);
        }
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut train = Vec::new();
        let mut test = Vec::new();
        for (label, indices) in &mut by_class {
            if indices.len() < 2 {
                bail!(
                    "class {} has too few rows for a stratified holdout",
                    self.labels[*label]
                );
            }
            indices.shuffle(&mut rng);
            let test_count = ((indices.len() as f64 * test_fraction).round() as usize)
                .clamp(1, indices.len() - 1);
            test.extend_from_slice(&indices[..test_count]);
            train.extend_from_slice(&indices[test_count..]);
        }
        train.sort_unstable();
        test.sort_unstable();
        Ok((train, test))
    }

    fn dataframe(&self, indices: &[usize]) -> Result<DataFrame> {
        let feature_count = self.rows.first().map(|row| row.1.len()).unwrap_or(0);
        let mut columns = Vec::with_capacity(feature_count + 1);
        for feature in 0..feature_count {
            columns.push(Column::new(
                format!("feature_{feature}").into(),
                indices
                    .iter()
                    .map(|&index| self.rows[index].1[feature])
                    .collect::<Vec<_>>(),
            ));
        }
        columns.push(Column::new(
            self.target_name.into(),
            indices
                .iter()
                .map(|&index| self.rows[index].2 as u32)
                .collect::<Vec<_>>(),
        ));
        DataFrame::new(columns).context("assemble dataset frame")
    }
}

#[derive(Serialize)]
struct RunRecord {
    dataset: String,
    model: String,
    status: &'static str,
    error: Option<String>,
    seed: u64,
    train_rows: usize,
    test_rows: usize,
    class_labels: Vec<String>,
    accuracy: Option<f64>,
    macro_precision: Option<f64>,
    macro_recall: Option<f64>,
    macro_f1: Option<f64>,
    per_class_f1: Option<Vec<f64>>,
    confusion_matrix: Option<Vec<Vec<u64>>>,
    positive_class_roc_auc: Option<f64>,
    fit_predict_seconds: Option<f64>,
    save_load_equivalent: Option<bool>,
    serialized_model: Option<String>,
    predictions_csv: Option<String>,
    model_sha256: Option<String>,
    predictions_sha256: Option<String>,
}

fn roc_auc_binary(actual: &[usize], positive_scores: &[f64]) -> Option<f64> {
    if actual.len() != positive_scores.len() || actual.is_empty() {
        return None;
    }
    let positives = actual.iter().filter(|&&label| label == 1).count();
    let negatives = actual.len() - positives;
    if positives == 0 || negatives == 0 {
        return None;
    }
    let mut ranked: Vec<(f64, usize)> = positive_scores
        .iter()
        .copied()
        .zip(actual.iter().copied())
        .collect();
    ranked.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut rank_sum_positive = 0.0;
    let mut begin = 0;
    while begin < ranked.len() {
        let mut end = begin + 1;
        while end < ranked.len() && ranked[end].0 == ranked[begin].0 {
            end += 1;
        }
        let average_rank = ((begin + 1 + end) as f64) / 2.0;
        rank_sum_positive += average_rank
            * ranked[begin..end]
                .iter()
                .filter(|(_, label)| *label == 1)
                .count() as f64;
        begin = end;
    }
    Some(
        (rank_sum_positive - (positives * (positives + 1) / 2) as f64)
            / (positives * negatives) as f64,
    )
}

fn sha256(path: &Path) -> Result<String> {
    use sha2::Digest;
    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    Ok(format!("{:x}", sha2::Sha256::digest(bytes)))
}

fn run_one(
    dataset: &Dataset,
    model_name: &str,
    model_type: ModelType,
    train_indices: &[usize],
    test_indices: &[usize],
    seed: u64,
    output_dir: &Path,
) -> RunRecord {
    let class_labels = dataset
        .labels
        .iter()
        .map(|label| (*label).to_owned())
        .collect();
    let mut record = RunRecord {
        dataset: dataset.name.to_owned(),
        model: model_name.to_owned(),
        status: "failed",
        error: None,
        seed,
        train_rows: train_indices.len(),
        test_rows: test_indices.len(),
        class_labels,
        accuracy: None,
        macro_precision: None,
        macro_recall: None,
        macro_f1: None,
        per_class_f1: None,
        confusion_matrix: None,
        positive_class_roc_auc: None,
        fit_predict_seconds: None,
        save_load_equivalent: None,
        serialized_model: None,
        predictions_csv: None,
        model_sha256: None,
        predictions_sha256: None,
    };
    let outcome = (|| -> Result<()> {
        let train = dataset.dataframe(train_indices)?;
        let test = dataset.dataframe(test_indices)?;
        let task = if dataset.labels.len() == 2 {
            TaskType::BinaryClassification
        } else {
            TaskType::MultiClassification
        };
        let mut config = TrainingConfig::new(task, dataset.target_name)
            .with_model(model_type)
            .with_random_state(seed)
            .with_n_estimators(32)
            .with_max_depth(8);
        config.validation_split = 0.1;
        let mut engine = TrainEngine::new(config);
        let started = Instant::now();
        engine
            .fit(&train)
            .context("fit AutoML model on outer training rows")?;
        let predicted = engine
            .predict(&test)
            .context("predict untouched outer holdout")?;
        let probabilities = engine.predict_proba(&test).ok();
        let elapsed = started.elapsed().as_secs_f64();
        let actual: Vec<usize> = test_indices
            .iter()
            .map(|&index| dataset.rows[index].2)
            .collect();
        let predicted_ids: Vec<usize> = predicted
            .iter()
            .map(|value| value.round() as usize)
            .collect();
        let summary = crate::evaluation::summarize_classification(
            &actual,
            &predicted_ids,
            dataset.labels.len(),
        )
        .context("invalid class labels returned by AutoML model")?;

        let artifact_stem = format!("{}__{}", dataset.name, model_name);
        let model_path = output_dir.join(format!("{artifact_stem}.model.json"));
        engine
            .save(model_path.to_str().context("model path is not UTF-8")?)
            .context("save model artifact")?;
        let reloaded = TrainEngine::load(model_path.to_str().context("model path is not UTF-8")?)
            .context("reload saved model")?;
        let reloaded_predictions = reloaded
            .predict(&test)
            .context("predict with reloaded model")?;
        let save_load_equivalent = reloaded_predictions == predicted;

        let predictions_path = output_dir.join(format!("{artifact_stem}.predictions.csv"));
        let mut predictions_file = fs::File::create(&predictions_path)?;
        writeln!(
            predictions_file,
            "source_row,actual_label,predicted_label,positive_class_score"
        )?;
        for (position, &dataset_index) in test_indices.iter().enumerate() {
            let positive_score = probabilities.as_ref().and_then(|matrix| {
                if matrix.ncols() == 2 {
                    Some(matrix[[position, 1]])
                } else {
                    None
                }
            });
            writeln!(
                predictions_file,
                "{},{},{},{}",
                dataset.rows[dataset_index].0,
                actual[position],
                predicted_ids[position],
                positive_score
                    .map(|score| score.to_string())
                    .unwrap_or_default()
            )?;
        }
        predictions_file.flush()?;
        record.accuracy = Some(summary.accuracy);
        record.macro_precision = Some(summary.macro_precision);
        record.macro_recall = Some(summary.macro_recall);
        record.macro_f1 = Some(summary.macro_f1);
        record.per_class_f1 = Some(summary.per_class_f1);
        record.confusion_matrix = Some(summary.confusion_matrix);
        record.positive_class_roc_auc = if dataset.labels.len() == 2 {
            probabilities.as_ref().and_then(|matrix| {
                if matrix.ncols() == 2 {
                    let scores = (0..matrix.nrows())
                        .map(|row| matrix[[row, 1]])
                        .collect::<Vec<_>>();
                    roc_auc_binary(&actual, &scores)
                } else {
                    None
                }
            })
        } else {
            None
        };
        record.fit_predict_seconds = Some(elapsed);
        record.save_load_equivalent = Some(save_load_equivalent);
        record.serialized_model = Some(model_path.to_string_lossy().into_owned());
        record.predictions_csv = Some(predictions_path.to_string_lossy().into_owned());
        record.model_sha256 = Some(sha256(&model_path)?);
        record.predictions_sha256 = Some(sha256(&predictions_path)?);
        if !save_load_equivalent {
            bail!("save/load prediction equivalence failed; original predictions and metrics retained");
        }
        record.status = "success";
        Ok(())
    })();
    if let Err(error) = outcome {
        record.error = Some(format!("{error:#}"));
    }
    record
}

const MATCHED_GRID: [(usize, usize); 6] = [(16, 4), (16, 8), (32, 4), (32, 8), (64, 4), (64, 8)];

fn estimator_type(name: &str) -> Result<ModelType> {
    match name {
        "random_forest" => Ok(ModelType::RandomForest),
        "extra_trees" => Ok(ModelType::ExtraTrees),
        _ => bail!("matched grid search supports random_forest and extra_trees, not {name}"),
    }
}

fn inner_validation_indices(
    dataset: &Dataset,
    outer_train_indices: &[usize],
    validation_fraction: f64,
) -> Result<(Vec<usize>, Vec<usize>)> {
    if !validation_fraction.is_finite() || !(0.0..1.0).contains(&validation_fraction) {
        bail!("validation fraction must be finite and in (0, 1)");
    }
    let mut by_class: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &index in outer_train_indices {
        by_class
            .entry(dataset.rows[index].2)
            .or_default()
            .push(index);
    }
    let mut fit = Vec::new();
    let mut validation = Vec::new();
    for indices in by_class.values() {
        if indices.len() < 2 {
            bail!("each outer-training class needs at least two rows");
        }
        let validation_count =
            ((indices.len() as f64 * validation_fraction).max(1.0) as usize).min(indices.len() - 1);
        let split = indices.len() - validation_count;
        fit.extend_from_slice(&indices[..split]);
        validation.extend_from_slice(&indices[split..]);
    }
    Ok((fit, validation))
}

/// Run a six-configuration shared grid on one inner holdout for AutoML.
/// Python comparison runners consume the emitted source-row protocol verbatim.
pub fn run_matched_grid_search(
    data_dir: &Path,
    output_dir: &Path,
    model_filter: Option<&str>,
    dataset_filter: Option<&str>,
    seed: u64,
    test_fraction: f64,
    validation_fraction: f64,
) -> Result<()> {
    if output_dir.exists() {
        bail!(
            "refusing to overwrite output directory {}",
            output_dir.display()
        );
    }
    if !validation_fraction.is_finite() || !(0.0..1.0).contains(&validation_fraction) {
        bail!("validation fraction must be finite and in (0, 1)");
    }
    let all_models = ["random_forest", "extra_trees"];
    let all_datasets = ["iris", "wine", "breast_cancer_wisconsin_diagnostic"];
    let models: Vec<_> = if let Some(name) = model_filter {
        if !all_models.contains(&name) {
            bail!("unknown model {name}; choose random_forest or extra_trees");
        }
        vec![name]
    } else {
        all_models.to_vec()
    };
    let datasets: Vec<&'static str> = if let Some(name) = dataset_filter {
        vec![*all_datasets
            .iter()
            .find(|candidate| **candidate == name)
            .with_context(|| format!("unknown dataset {name}"))?]
    } else {
        all_datasets.to_vec()
    };

    fs::create_dir_all(output_dir)?;
    let mut protocol_datasets = Vec::new();
    let mut case_records = Vec::new();
    for dataset_name in &datasets {
        let dataset = Dataset::read(dataset_name, data_dir)?;
        let dataset_offset = all_datasets
            .iter()
            .position(|name| name == dataset_name)
            .expect("selected dataset comes from declared dataset list");
        let split_seed = seed.wrapping_add(dataset_offset as u64);
        let (outer_train, outer_test) = dataset.split(split_seed, test_fraction)?;
        let (fit_indices, validation_indices) =
            inner_validation_indices(&dataset, &outer_train, validation_fraction)?;
        // TrainEngine applies its own per-class trailing validation holdback.
        // Keep that secondary holdback explicit in the protocol so the
        // comparison implementation trains on the same source rows. Most
        // importantly, the external selection-validation rows are never passed
        // to TrainEngine::fit.
        let (model_training_indices, automl_internal_validation_indices) =
            inner_validation_indices(&dataset, &fit_indices, validation_fraction)?;
        let source_path = data_dir.join(dataset.source_file);
        let protocol_dataset = serde_json::json!({
            "dataset": dataset.name,
            "source_file": dataset.source_file,
            "source_sha256": sha256(&source_path)?,
            "split_seed": split_seed,
            "test_fraction": test_fraction,
            "validation_fraction": validation_fraction,
            "outer_train_source_rows": outer_train.iter().map(|&i| dataset.rows[i].0).collect::<Vec<_>>(),
            "outer_test_source_rows": outer_test.iter().map(|&i| dataset.rows[i].0).collect::<Vec<_>>(),
            "inner_fit_source_rows": fit_indices.iter().map(|&i| dataset.rows[i].0).collect::<Vec<_>>(),
            "inner_validation_source_rows": validation_indices.iter().map(|&i| dataset.rows[i].0).collect::<Vec<_>>(),
            "model_training_source_rows": model_training_indices.iter().map(|&i| dataset.rows[i].0).collect::<Vec<_>>(),
            "automl_internal_validation_source_rows": automl_internal_validation_indices.iter().map(|&i| dataset.rows[i].0).collect::<Vec<_>>(),
            "class_labels": dataset.labels,
        });
        protocol_datasets.push(protocol_dataset);

        let model_fit_frame = dataset.dataframe(&fit_indices)?;
        let validation_frame = dataset.dataframe(&validation_indices)?;
        let test_frame = dataset.dataframe(&outer_test)?;
        let validation_actual: Vec<usize> = validation_indices
            .iter()
            .map(|&i| dataset.rows[i].2)
            .collect();
        let test_actual: Vec<usize> = outer_test.iter().map(|&i| dataset.rows[i].2).collect();

        for (model_index, model_name) in models.iter().enumerate() {
            let model_type = estimator_type(model_name)?;
            let model_seed = seed
                .wrapping_add(dataset_offset as u64)
                .wrapping_add((model_index as u64 + 1) * 1000);
            let case_dir = output_dir
                .join("automl")
                .join(dataset.name)
                .join(model_name);
            fs::create_dir_all(&case_dir)?;
            let mut trials = Vec::new();
            let mut best_accuracy = f64::NEG_INFINITY;
            let mut best_configuration = (0, 0);
            let mut best_engine: Option<TrainEngine> = None;

            for (trial_index, &(n_estimators, max_depth)) in MATCHED_GRID.iter().enumerate() {
                let task = if dataset.labels.len() == 2 {
                    TaskType::BinaryClassification
                } else {
                    TaskType::MultiClassification
                };
                let mut config = TrainingConfig::new(task, dataset.target_name)
                    .with_model(model_type.clone())
                    .with_random_state(model_seed)
                    .with_n_estimators(n_estimators)
                    .with_max_depth(max_depth);
                config.validation_split = validation_fraction;
                let mut engine = TrainEngine::new(config);
                let started = Instant::now();
                engine
                    .fit(&model_fit_frame)
                    .context("fit grid candidate without external validation rows")?;
                let predicted = engine
                    .predict(&validation_frame)
                    .context("predict inner validation rows")?;
                let predicted_ids: Vec<usize> = predicted
                    .iter()
                    .map(|value| value.round() as usize)
                    .collect();
                let summary = crate::evaluation::summarize_classification(
                    &validation_actual,
                    &predicted_ids,
                    dataset.labels.len(),
                )
                .context("summarize inner validation predictions")?;
                let fit_validation_seconds = started.elapsed().as_secs_f64();
                trials.push(serde_json::json!({
                    "trial": trial_index,
                    "n_estimators": n_estimators,
                    "max_depth": max_depth,
                    "seed": model_seed,
                    "validation_accuracy": summary.accuracy,
                    "validation_macro_f1": summary.macro_f1,
                    "fit_and_validation_predict_seconds": fit_validation_seconds,
                }));
                if summary.accuracy > best_accuracy {
                    best_accuracy = summary.accuracy;
                    best_configuration = (n_estimators, max_depth);
                    best_engine = Some(engine);
                }
            }

            let selected_engine = best_engine.context("grid search selected no model")?;
            let test_started = Instant::now();
            let predicted = selected_engine
                .predict(&test_frame)
                .context("predict untouched outer test rows")?;
            let test_predict_seconds = test_started.elapsed().as_secs_f64();
            let predicted_ids: Vec<usize> = predicted
                .iter()
                .map(|value| value.round() as usize)
                .collect();
            let summary = crate::evaluation::summarize_classification(
                &test_actual,
                &predicted_ids,
                dataset.labels.len(),
            )
            .context("summarize outer test predictions")?;
            let model_path = case_dir.join(format!("{}.model.json", model_name));
            selected_engine
                .save(model_path.to_str().context("model path is not UTF-8")?)
                .context("save selected model")?;
            let predictions_path = case_dir.join(format!("{}.predictions.csv", model_name));
            let mut predictions_file = fs::File::create(&predictions_path)?;
            writeln!(predictions_file, "source_row,actual_label,predicted_label")?;
            for (position, &index) in outer_test.iter().enumerate() {
                writeln!(
                    predictions_file,
                    "{},{},{}",
                    dataset.rows[index].0, test_actual[position], predicted_ids[position]
                )?;
            }
            predictions_file.flush()?;
            case_records.push(serde_json::json!({
                "dataset": dataset.name,
                "model": model_name,
                "seed": model_seed,
                "trial_budget": MATCHED_GRID.len(),
                "validation_metric": "accuracy",
                "trials": trials,
                "selected_configuration": {
                    "n_estimators": best_configuration.0,
                    "max_depth": best_configuration.1,
                    "validation_accuracy": best_accuracy,
                },
                "outer_test": {
                    "n_rows": outer_test.len(),
                    "accuracy": summary.accuracy,
                    "macro_f1": summary.macro_f1,
                    "confusion_matrix": summary.confusion_matrix,
                    "fit_model_file": model_path,
                    "model_sha256": sha256(&model_path)?,
                    "predictions_file": predictions_path,
                    "predictions_sha256": sha256(&predictions_path)?,
                    "predict_seconds": test_predict_seconds,
                },
            }));
        }
    }
    let protocol_path = output_dir.join("matched-search-protocol.json");
    fs::write(
        &protocol_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema": "2048-ml.matched-grid-search",
            "schema_version": 2,
            "seed": seed,
            "test_fraction": test_fraction,
            "validation_fraction": validation_fraction,
            "models": models,
            "datasets": protocol_datasets,
            "grid": MATCHED_GRID.iter().map(|&(n_estimators, max_depth)| serde_json::json!({
                "n_estimators": n_estimators,
                "max_depth": max_depth,
            })).collect::<Vec<_>>(),
            "trial_budget_per_model_dataset_implementation": MATCHED_GRID.len(),
            "search_objective": "inner_validation_accuracy",
            "selection_rule": "highest validation accuracy; grid order breaks ties",
            "fit_boundary": "AutoML receives only inner_fit_source_rows; its native per-class holdback is internal to those rows; external validation rows are never used during fit",
            "outer_test_use": "score only after configuration selection; never used for selection",
        }))?,
    )?;
    let results_path = output_dir.join("automl-matched-search-results.json");
    fs::write(&results_path, serde_json::to_vec_pretty(&case_records)?)?;
    let manifest_path = output_dir.join("automl-matched-search-manifest.json");
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "created_utc": chrono::Utc::now().to_rfc3339(),
            "source_revision": std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|r| r.status.success()).map(|r| String::from_utf8_lossy(&r.stdout).trim().to_owned()),
            "automl_commit": std::process::Command::new("git").args(["-C", "automl", "rev-parse", "HEAD"]).output().ok().filter(|r| r.status.success()).map(|r| String::from_utf8_lossy(&r.stdout).trim().to_owned()),
            "root_worktree_dirty": std::process::Command::new("git").args(["diff", "--quiet"]).status().map(|s| !s.success()).ok(),
            "host": {"os": std::env::consts::OS, "architecture": std::env::consts::ARCH},
            "protocol_file": protocol_path,
            "protocol_sha256": sha256(&protocol_path)?,
            "results_file": results_path,
            "results_sha256": sha256(&results_path)?,
            "successful_cases": case_records.len(),
            "interpretation_limitations": [
                "The six shared configurations are a matched grid, not a comparison of the frameworks' optimizer algorithms",
                "The single inner validation holdout and outer test split are fixed-split diagnostics",
                "Model implementations and defaults differ even when tree count and depth are equal",
                "One process and one split do not establish runtime superiority or population-level performance",
            ],
        }))?,
    )?;
    println!("matched-grid artifacts written to {}", output_dir.display());
    Ok(())
}

/// Run a deterministic stratified holdout study over selected acquired UCI cases.
pub fn run_standard_datasets(
    data_dir: &Path,
    output_dir: &Path,
    model_filter: Option<&str>,
    dataset_filter: Option<&str>,
    seed: u64,
    test_fraction: f64,
) -> Result<()> {
    fs::create_dir_all(output_dir).with_context(|| format!("create {}", output_dir.display()))?;
    let all_candidates = [
        ("random_forest", ModelType::RandomForest),
        ("extra_trees", ModelType::ExtraTrees),
        ("adaboost", ModelType::AdaBoost),
        ("knn", ModelType::KNN),
        ("naive_bayes", ModelType::NaiveBayes),
    ];
    let all_datasets = ["iris", "wine", "breast_cancer_wisconsin_diagnostic"];
    let candidates: Vec<_> = if let Some(name) = model_filter {
        vec![all_candidates
            .iter()
            .find(|(candidate, _)| *candidate == name)
            .with_context(|| format!("unknown framework model '{name}'"))?
            .clone()]
    } else {
        all_candidates.to_vec()
    };
    let dataset_names: Vec<_> = if let Some(name) = dataset_filter {
        vec![*all_datasets
            .iter()
            .find(|dataset| **dataset == name)
            .with_context(|| format!("unknown framework dataset '{name}'"))?]
    } else {
        all_datasets.to_vec()
    };
    let mut records = Vec::new();
    let mut split_records = Vec::new();
    for dataset_name in &dataset_names {
        let dataset = Dataset::read(dataset_name, data_dir)?;
        let dataset_offset = all_datasets
            .iter()
            .position(|name| name == dataset_name)
            .expect("selected dataset comes from the declared dataset list");
        let split_seed = seed.wrapping_add(dataset_offset as u64);
        let (train_indices, test_indices) = dataset.split(split_seed, test_fraction)?;
        let split_path = output_dir.join(format!("{}.split.json", dataset.name));
        let split = serde_json::json!({
            "dataset": dataset.name,
            "source_file": dataset.source_file,
            "source_file_sha256": sha256(&data_dir.join(dataset.source_file))?,
            "split_method": "stratified_shuffle_per_class_then_index_order",
            "global_seed": seed,
            "split_seed": split_seed,
            "test_fraction_requested": test_fraction,
            "class_labels": dataset.labels,
            "train_source_rows": train_indices.iter().map(|&index| dataset.rows[index].0).collect::<Vec<_>>(),
            "test_source_rows": test_indices.iter().map(|&index| dataset.rows[index].0).collect::<Vec<_>>(),
            "preprocessing": "none; raw numeric predictors; ID field excluded for WDBC",
            "inner_validation_fraction": 0.1,
            "outer_test_rows_excluded_from_fit": true
        });
        fs::write(&split_path, serde_json::to_vec_pretty(&split)?)?;
        split_records
            .push(serde_json::json!({"dataset": dataset.name, "split_manifest": split_path}));
        for (model_name, model_type) in candidates.iter().cloned() {
            let record = run_one(
                &dataset,
                model_name,
                model_type,
                &train_indices,
                &test_indices,
                seed,
                output_dir,
            );
            eprintln!(
                "framework_validation dataset={} model={} status={} accuracy={:?} error={:?}",
                record.dataset, record.model, record.status, record.accuracy, record.error
            );
            records.push(record);
        }
    }
    let results_path = output_dir.join("framework-validation-results.json");
    fs::write(&results_path, serde_json::to_vec_pretty(&records)?)?;
    let failed = records
        .iter()
        .filter(|record| record.status != "success")
        .count();
    let manifest_path = output_dir.join("framework-validation-manifest.json");
    let manifest = serde_json::json!({
        "schema": "2048-ml.framework-validation-run",
        "schema_version": 1,
        "created_utc": chrono::Utc::now().to_rfc3339(),
        "project_version": env!("CARGO_PKG_VERSION"),
        "automl_commit": std::process::Command::new("git").args(["-C", "automl", "rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()),
        "automl_worktree_dirty": std::process::Command::new("git").args(["-C", "automl", "diff", "--quiet"]).status().map(|status| !status.success()).ok(),
        "source_revision": std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()),
        "root_worktree_dirty": std::process::Command::new("git").args(["diff", "--quiet"]).status().map(|status| !status.success()).ok(),
        "cargo_lock_sha256": sha256(Path::new("Cargo.lock")).ok(),
        "rustc_version": std::process::Command::new("rustc").arg("--version").output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()),
        "seed": seed,
        "test_fraction": test_fraction,
        "candidate_models": candidates.iter().map(|(name, _)| name).collect::<Vec<_>>(),
        "datasets": dataset_names,
        "configuration": {"n_estimators": 32, "max_depth": 8, "internal_validation_fraction": 0.1, "preprocessing": "none"},
        "dataset_splits": split_records,
        "results_json": results_path,
        "results_sha256": sha256(&results_path)?,
        "successful_runs": records.len() - failed,
        "failed_runs": failed,
        "host": {"os": std::env::consts::OS, "architecture": std::env::consts::ARCH,
            "logical_cpus": std::thread::available_parallelism().map(|count| count.get()).ok()},
        "limitations": ["No external framework baseline", "No memory profile", "One seed only", "ROC-AUC is reported only when binary class probabilities are available", "No CLI versus API equivalence study"]
    });
    fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
    println!(
        "framework validation complete: {} successful, {} failed; results={} manifest={}",
        records.len() - failed,
        failed,
        results_path.display(),
        manifest_path.display()
    );
    Ok(())
}
