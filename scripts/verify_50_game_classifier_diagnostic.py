#!/usr/bin/env python3
"""Verify the five-candidate chronological label diagnostic on the 50-game pilot."""

import argparse
import csv
import hashlib
import json
import math
from pathlib import Path


MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
DEFAULT_DATA = Path("reports/collection_pilots/2026-09-27-50-game-followup/training.csv")
DEFAULT_METADATA = Path(
    "reports/collection_pilots/2026-09-27-50-game-followup/training.metadata.csv"
)


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
    parser.add_argument("--data", type=Path, default=DEFAULT_DATA)
    parser.add_argument("--metadata", type=Path, default=DEFAULT_METADATA)
    parser.add_argument("--seed", type=int, default=90652)
    parser.add_argument("--development-fraction", type=float, default=0.8)
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = (root / args.report_dir).resolve()
    data_path = (root / args.data).resolve()
    metadata_path = (root / args.metadata).resolve()
    assert 0.0 < args.development_fraction < 1.0
    data_header, data = read_csv(data_path)
    metadata_header, metadata = read_csv(metadata_path)
    assert len(data_header) == 18 and len(data) > 0
    assert "game_id" in metadata_header and "row_index" in metadata_header
    assert len(metadata) == len(data)
    assert [int(row["row_index"]) for row in metadata] == list(range(len(data)))
    game_ids = [int(row["game_id"]) for row in metadata]
    game_count = len(set(game_ids))
    development_game_count = int(game_count * args.development_fraction)
    development_rows = [i for i, game_id in enumerate(game_ids) if game_id < development_game_count]
    holdout_rows = [i for i, game_id in enumerate(game_ids) if game_id >= development_game_count]
    assert set(game_ids) == set(range(game_count))
    assert game_ids == sorted(game_ids)
    training_row_count = len(development_rows)
    holdout_row_count = len(holdout_rows)
    expected_indices = holdout_rows
    expected_actual = [int(float(data[i]["action"])) for i in holdout_rows]
    expected_game_ids = [game_ids[i] for i in holdout_rows]

    result = {
        "status": "pass",
        "source_revision": None,
        "training_csv": str(args.data),
        "training_csv_sha256": sha256(data_path),
        "metadata_csv": str(args.metadata),
        "metadata_csv_sha256": sha256(metadata_path),
        "training_rows": training_row_count,
        "training_games": list(range(development_game_count)),
        "holdout_rows": holdout_row_count,
        "holdout_games": list(range(development_game_count, game_count)),
        "models": {},
    }

    source_revisions = set()
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
        source_revisions.add(manifest["source_revision"])
        assert manifest["model"] == model
        assert manifest["global_seed"] == args.seed
        assert manifest["state_feature_count"] == 17
        assert manifest["training_data_sha256"] == sha256(data_path)
        assert manifest["metadata_sha256"] == sha256(metadata_path)
        assert manifest["cv_folds"] == 5
        assert math.isclose(manifest["development_fraction"], args.development_fraction)
        assert manifest["held_out_evaluation"]["game_ids"] == list(range(development_game_count, game_count))
        assert manifest["held_out_evaluation"]["n_rows"] == holdout_row_count
        assert manifest["grouped_cv_evaluation"]["n_splits"] == 5

        prediction_header, predictions = read_csv(predictions_path)
        assert prediction_header == [
            "row_index",
            "game_id",
            "actual_action",
            "predicted_action",
        ]
        assert len(predictions) == holdout_row_count
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

    assert len(source_revisions) == 1
    result["source_revision"] = next(iter(source_revisions))

    verification_path = report / "verification.json"
    verification_path.write_text(json.dumps(result, indent=2) + "\n")
    print(f"verified five candidates and wrote {verification_path}")


if __name__ == "__main__":
    main()
