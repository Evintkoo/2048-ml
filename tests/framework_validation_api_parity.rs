use game2048_ml::framework_validation::benchmark::run_standard_datasets;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn comparable_records(path: &Path) -> Value {
    let mut records: Value = serde_json::from_slice(
        &std::fs::read(path.join("framework-validation-results.json")).unwrap(),
    )
    .unwrap();
    for record in records.as_array_mut().unwrap() {
        let object = record.as_object_mut().unwrap();
        object.remove("fit_predict_seconds");
        object.remove("serialized_model");
        object.remove("predictions_csv");
        // This checks CLI/library metric and prediction parity, not byte identity
        // of the serialized model representation.
        object.remove("model_sha256");
        object.remove("predictions_sha256");
        object.remove("fit_baseline_rss_bytes");
        object.remove("fit_peak_rss_bytes");
        object.remove("fit_peak_incremental_rss_bytes");
    }
    records
}

#[test]
fn framework_validation_cli_matches_public_rust_api() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let data_dir = root.join("data/framework_validation");
    let temp = tempfile::tempdir().unwrap();
    let api_output = temp.path().join("api");
    let cli_output = temp.path().join("cli");

    run_standard_datasets(&data_dir, &api_output, None, None, 42, 0.2).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_game2048-ml"))
        .args([
            "framework-validate",
            "--data-dir",
            data_dir.to_str().unwrap(),
            "--output-dir",
            cli_output.to_str().unwrap(),
            "--seed",
            "42",
            "--test-fraction",
            "0.2",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    assert_eq!(
        comparable_records(&api_output),
        comparable_records(&cli_output),
        "CLI and public API result fields differ"
    );
    for dataset in ["iris", "wine", "breast_cancer_wisconsin_diagnostic"] {
        assert_eq!(
            std::fs::read(api_output.join(format!("{dataset}.split.json"))).unwrap(),
            std::fs::read(cli_output.join(format!("{dataset}.split.json"))).unwrap(),
            "CLI and public API split manifests differ for {dataset}"
        );
        for model in [
            "random_forest",
            "extra_trees",
            "adaboost",
            "knn",
            "naive_bayes",
        ] {
            let predictions = format!("{dataset}__{model}.predictions.csv");
            assert_eq!(
                std::fs::read(api_output.join(&predictions)).unwrap(),
                std::fs::read(cli_output.join(&predictions)).unwrap(),
                "CLI and public API prediction artifacts differ for {dataset}/{model}"
            );
        }
    }
}
