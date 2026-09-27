#!/usr/bin/env python3
"""Verify data lineage, disjoint game groups, predictions, and metrics for the 25-game diagnostic."""

import argparse
import csv
import hashlib
import json
import math
from pathlib import Path


MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_csv(path: Path):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        return reader.fieldnames, list(reader)


def metrics(actual, predicted):
    confusion = [[0] * 4 for _ in range(4)]
    for truth, guess in zip(actual, predicted):
        confusion[truth][guess] += 1
    f1 = []
    for action in range(4):
        tp = confusion[action][action]
        fp = sum(confusion[row][action] for row in range(4)) - tp
        fn = sum(confusion[action]) - tp
        denominator = 2 * tp + fp + fn
        f1.append(0.0 if denominator == 0 else 2 * tp / denominator)
    return {
        "n_rows": len(actual),
        "accuracy": sum(a == p for a, p in zip(actual, predicted)) / len(actual),
        "macro_f1": sum(f1) / 4,
        "confusion_matrix_actual_rows_predicted_columns": confusion,
    }


def close(left, right):
    return math.isclose(float(left), float(right), rel_tol=0.0, abs_tol=1e-12)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--report-dir",
        type=Path,
        default=Path("reports/candidate_classifier_independent_holdout/2026-09-27-25-game"),
    )
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = (root / args.report_dir).resolve()
    protocol_path = report / "protocol-manifest.json"
    protocol = json.loads(protocol_path.read_text())
    assert protocol["state_feature_count"] == 17
    assert protocol["cv_folds"] == 5 and protocol["development_fraction"] == 0.8
    assert protocol["training_source"]["game_seeds"] == [90627, 90646]
    assert protocol["holdout_source"]["game_seeds"] == [90647, 90651]
    source_manifests = {}
    for source_name in ("training", "holdout"):
        source = protocol[f"{source_name}_source"]
        source_manifest_path = root / source["manifest"]
        assert source["manifest_sha256"] == sha256(source_manifest_path)
        source_manifest = json.loads(source_manifest_path.read_text())
        source_manifests[source_name] = source_manifest
        for key in ("training_csv", "metadata_csv"):
            source_csv = root / source_manifest[key]
            assert source_manifest[f"{key}_sha256"] == sha256(source_csv)

    data_path = root / protocol["combined_data"]["training_csv"]
    metadata_path = root / protocol["combined_data"]["metadata_csv"]
    assert sha256(data_path) == protocol["combined_data"]["training_csv_sha256"]
    assert sha256(metadata_path) == protocol["combined_data"]["metadata_csv_sha256"]
    data_header, data = read_csv(data_path)
    metadata_header, metadata = read_csv(metadata_path)
    assert len(data) == len(metadata) == 2988
    assert len(data_header) == 18
    assert len(metadata) == protocol["combined_data"]["total_rows"]
    assert [int(row["row_index"]) for row in metadata] == list(range(len(data)))
    assert set(int(row["game_id"]) for row in metadata[:2447]) == set(range(20))
    assert set(int(row["game_id"]) for row in metadata[2447:]) == set(range(20, 25))
    train_manifest = source_manifests["training"]
    holdout_manifest = source_manifests["holdout"]
    _, source_train_data = read_csv(root / train_manifest["training_csv"])
    _, source_holdout_data = read_csv(root / holdout_manifest["training_csv"])
    _, source_train_metadata = read_csv(root / train_manifest["metadata_csv"])
    _, source_holdout_metadata = read_csv(root / holdout_manifest["metadata_csv"])
    assert data[:2447] == source_train_data
    assert data[2447:] == source_holdout_data
    assert metadata[:2447] == source_train_metadata
    expected_metadata = list(source_train_metadata)
    for index, row in enumerate(source_holdout_metadata):
        row = dict(row)
        row["row_index"] = str(2447 + index)
        row["game_id"] = str(20 + int(row["game_id"]))
        expected_metadata.append(row)
    assert metadata == expected_metadata

    result = {
        "status": "pass",
        "source_revision": protocol["source_revision"],
        "protocol_manifest_sha256": sha256(protocol_path),
        "combined_training_csv_sha256": sha256(data_path),
        "combined_metadata_csv_sha256": sha256(metadata_path),
        "training_rows": 2447,
        "holdout_rows": 541,
        "holdout_game_ids": [20, 21, 22, 23, 24],
        "models": {},
    }
    expected_indices = list(range(2447, 2988))
    expected_game_ids = [int(row["game_id"]) for row in metadata[2447:]]
    expected_actual = [int(float(row["action"])) for row in data[2447:]]
    for model in MODELS:
        prefix = report / "models" / f"{model}.policy"
        model_path = Path(f"{prefix}.json")
        model_manifest_path = Path(f"{prefix}.manifest.json")
        model_manifest = json.loads(model_manifest_path.read_text())
        evaluation = model_manifest["held_out_evaluation"]
        predictions_path = Path(evaluation["predictions_csv"])
        if not predictions_path.is_absolute():
            predictions_path = root / predictions_path
        assert sha256(model_path) == model_manifest["model_artifact_sha256"]
        assert sha256(predictions_path) == evaluation["predictions_csv_sha256"]
        assert model_manifest["training_data_sha256"] == sha256(data_path)
        assert model_manifest["metadata_sha256"] == sha256(metadata_path)
        assert model_manifest["model"] == model
        assert model_manifest["source_revision"] == protocol["source_revision"]
        assert model_manifest["global_seed"] == protocol["global_training_seed"]
        assert model_manifest["training_data_sha256"] == protocol["combined_data"]["training_csv_sha256"]
        assert model_manifest["metadata_sha256"] == protocol["combined_data"]["metadata_csv_sha256"]
        assert model_manifest["development_fraction"] == 0.8
        assert model_manifest["cv_folds"] == 5
        assert model_manifest["held_out_evaluation"]["game_ids"] == [20, 21, 22, 23, 24]
        pred_header, predictions = read_csv(predictions_path)
        assert pred_header == ["row_index", "game_id", "actual_action", "predicted_action"]
        assert len(predictions) == 541
        indices = [int(row["row_index"]) for row in predictions]
        game_ids = [int(row["game_id"]) for row in predictions]
        actual = [int(row["actual_action"]) for row in predictions]
        predicted = [int(row["predicted_action"]) for row in predictions]
        assert indices == expected_indices
        assert game_ids == expected_game_ids
        assert actual == expected_actual
        assert all(0 <= action < 4 for action in actual + predicted)
        measured = metrics(actual, predicted)
        assert measured["n_rows"] == evaluation["n_rows"]
        assert close(measured["accuracy"], evaluation["accuracy"])
        assert close(measured["macro_f1"], evaluation["macro_f1"])
        assert measured["confusion_matrix_actual_rows_predicted_columns"] == evaluation[
            "confusion_matrix_actual_rows_predicted_columns"
        ]
        result["models"][model] = {
            **measured,
            "grouped_cv_accuracy_mean": model_manifest["grouped_cv_evaluation"][
                "mean_accuracy"
            ],
            "grouped_cv_accuracy_sample_sd": model_manifest["grouped_cv_evaluation"][
                "std_accuracy"
            ],
            "grouped_cv_macro_f1_mean": model_manifest["grouped_cv_evaluation"][
                "mean_macro_f1"
            ],
            "grouped_cv_macro_f1_sample_sd": model_manifest["grouped_cv_evaluation"][
                "std_macro_f1"
            ],
            "model_sha256": sha256(model_path),
            "model_manifest_sha256": sha256(model_manifest_path),
            "prediction_sha256": sha256(predictions_path),
        }

    verification_path = report / "verification.json"
    verification_path.write_text(json.dumps(result, indent=2) + "\n")
    print(f"verified five candidates and wrote {verification_path}")


if __name__ == "__main__":
    main()
