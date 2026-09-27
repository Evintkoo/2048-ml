#!/usr/bin/env python3
"""Verify the five-candidate chronological label diagnostic on the 50-game pilot."""

import argparse
import csv
import hashlib
import json
import math
from pathlib import Path


MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
DATA = Path("reports/collection_pilots/2026-09-27-50-game-followup/training.csv")
METADATA = Path(
    "reports/collection_pilots/2026-09-27-50-game-followup/training.metadata.csv"
)
EXPECTED_SOURCE_REVISION = "c7400ef535dd74eb62c263d9a2f7a9b4cabcb624"
EXPECTED_ROWS = 5318
TRAINING_ROWS = 4415
HOLDOUT_ROWS = EXPECTED_ROWS - TRAINING_ROWS


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
        default=Path("reports/candidate_classifier_pilot/2026-09-27-50-game"),
    )
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = (root / args.report_dir).resolve()
    data_path = root / DATA
    metadata_path = root / METADATA
    data_header, data = read_csv(data_path)
    metadata_header, metadata = read_csv(metadata_path)
    assert len(data_header) == 18 and len(data) == EXPECTED_ROWS
    assert "game_id" in metadata_header and "row_index" in metadata_header
    assert len(metadata) == EXPECTED_ROWS
    assert [int(row["row_index"]) for row in metadata] == list(range(EXPECTED_ROWS))
    game_ids = [int(row["game_id"]) for row in metadata]
    assert set(game_ids) == set(range(50))
    assert set(game_ids[:TRAINING_ROWS]) == set(range(40))
    assert set(game_ids[TRAINING_ROWS:]) == set(range(40, 50))
    expected_indices = list(range(TRAINING_ROWS, EXPECTED_ROWS))
    expected_actual = [int(float(row["action"])) for row in data[TRAINING_ROWS:]]
    expected_game_ids = game_ids[TRAINING_ROWS:]

    result = {
        "status": "pass",
        "source_revision": EXPECTED_SOURCE_REVISION,
        "training_csv": str(DATA),
        "training_csv_sha256": sha256(data_path),
        "metadata_csv": str(METADATA),
        "metadata_csv_sha256": sha256(metadata_path),
        "training_rows": TRAINING_ROWS,
        "training_games": list(range(40)),
        "holdout_rows": HOLDOUT_ROWS,
        "holdout_games": list(range(40, 50)),
        "models": {},
    }

    for model in MODELS:
        prefix = report / "models" / f"{model}.policy"
        model_path = Path(f"{prefix}.json")
        model_manifest_path = Path(f"{prefix}.manifest.json")
        manifest = json.loads(model_manifest_path.read_text())
        evaluation = manifest["held_out_evaluation"]
        predictions_path = Path(evaluation["predictions_csv"])
        if not predictions_path.is_absolute():
            predictions_path = root / predictions_path
        assert sha256(model_path) == manifest["model_artifact_sha256"]
        assert sha256(predictions_path) == evaluation["predictions_csv_sha256"]
        assert manifest["source_revision"] == EXPECTED_SOURCE_REVISION
        assert manifest["model"] == model
        assert manifest["global_seed"] == 90652
        assert manifest["state_feature_count"] == 17
        assert manifest["training_data_sha256"] == sha256(data_path)
        assert manifest["metadata_sha256"] == sha256(metadata_path)
        assert manifest["cv_folds"] == 5
        assert manifest["development_fraction"] == 0.8
        assert manifest["held_out_evaluation"]["game_ids"] == list(range(40, 50))
        assert manifest["held_out_evaluation"]["n_rows"] == HOLDOUT_ROWS
        assert manifest["grouped_cv_evaluation"]["n_splits"] == 5

        prediction_header, predictions = read_csv(predictions_path)
        assert prediction_header == [
            "row_index",
            "game_id",
            "actual_action",
            "predicted_action",
        ]
        assert len(predictions) == HOLDOUT_ROWS
        indices = [int(row["row_index"]) for row in predictions]
        predicted_games = [int(row["game_id"]) for row in predictions]
        actual = [int(row["actual_action"]) for row in predictions]
        predicted = [int(row["predicted_action"]) for row in predictions]
        assert indices == expected_indices
        assert predicted_games == expected_game_ids
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
            "grouped_cv_accuracy_mean": manifest["grouped_cv_evaluation"][
                "mean_accuracy"
            ],
            "grouped_cv_accuracy_sample_sd": manifest["grouped_cv_evaluation"][
                "std_accuracy"
            ],
            "grouped_cv_macro_f1_mean": manifest["grouped_cv_evaluation"][
                "mean_macro_f1"
            ],
            "grouped_cv_macro_f1_sample_sd": manifest["grouped_cv_evaluation"][
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
