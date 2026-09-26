#!/usr/bin/env python3
"""Verify shared holdout rows, digests, and classification summaries."""

import argparse
import csv
import hashlib
import json
from pathlib import Path

MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def classify(actual: list[int], predicted: list[int]) -> dict:
    matrix = [[0] * 4 for _ in range(4)]
    for truth, guess in zip(actual, predicted, strict=True):
        matrix[truth][guess] += 1
    precision = []
    recall = []
    f1 = []
    for label in range(4):
        tp = matrix[label][label]
        actual_n = sum(matrix[label])
        predicted_n = sum(row[label] for row in matrix)
        p = tp / predicted_n if predicted_n else 0.0
        r = tp / actual_n if actual_n else 0.0
        precision.append(p)
        recall.append(r)
        f1.append(2 * p * r / (p + r) if p + r else 0.0)
    return {
        "n_rows": len(actual),
        "accuracy": sum(matrix[i][i] for i in range(4)) / len(actual),
        "macro_precision": sum(precision) / 4,
        "macro_recall": sum(recall) / 4,
        "macro_f1": sum(f1) / 4,
        "confusion_matrix_actual_rows_predicted_columns": matrix,
    }


def close(a: float, b: float) -> bool:
    return abs(a - b) <= 1e-12


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--report-dir", type=Path, default=Path("reports/candidate_classifier_pilot/2026-09-27")
    )
    args = parser.parse_args()

    common_rows = None
    summary = []
    for model in MODELS:
        stem = f"{model}.policy"
        manifest_path = args.report_dir / f"{stem}.manifest.json"
        manifest = json.loads(manifest_path.read_text())
        config = manifest["training_configuration"]
        holdout = manifest["held_out_evaluation"]
        grouped_cv = manifest["grouped_cv_evaluation"]
        assert manifest["model"] == model
        assert manifest["state_feature_count"] == 17
        assert manifest["global_seed"] == 90627
        assert config["cv_strategy"] == "GroupKFold" and config["cv_folds"] == 5
        assert config["development_fraction"] == 0.85
        assert config["early_stopping_enabled"] is False
        assert grouped_cv["strategy"] == "GroupKFold" and grouped_cv["n_splits"] == 5
        assert len(grouped_cv["fold_accuracy"]) == 5
        assert holdout["game_ids"] == [17, 18, 19]
        assert holdout["n_rows"] == 391

        model_path = args.report_dir / f"{stem}.json"
        prediction_path = args.report_dir / f"{stem}.holdout-predictions.csv"
        assert sha256(model_path) == manifest["model_artifact_sha256"]
        assert sha256(prediction_path) == holdout["predictions_csv_sha256"]

        with prediction_path.open(newline="") as stream:
            rows = list(csv.DictReader(stream))
        assert len(rows) == 391
        row_identity = [(row["row_index"], row["game_id"], row["actual_action"]) for row in rows]
        if common_rows is None:
            common_rows = row_identity
        else:
            assert row_identity == common_rows, f"holdout rows differ for {model}"

        actual = [int(row["actual_action"]) for row in rows]
        predicted = [int(row["predicted_action"]) for row in rows]
        metrics = classify(actual, predicted)
        for field in ("accuracy", "macro_precision", "macro_recall", "macro_f1"):
            assert close(metrics[field], holdout[field]), f"{model} {field} does not reproduce"
        assert metrics["confusion_matrix_actual_rows_predicted_columns"] == holdout[
            "confusion_matrix_actual_rows_predicted_columns"
        ]

        summary.append(
            {
                "model": model,
                "grouped_cv_accuracy": grouped_cv["mean_accuracy"],
                "grouped_cv_std_accuracy": grouped_cv["std_accuracy"],
                "grouped_cv_fold_accuracy": grouped_cv["fold_accuracy"],
                **metrics,
                "manifest_sha256": sha256(manifest_path),
                "model_sha256": sha256(model_path),
                "predictions_sha256": sha256(prediction_path),
                "source_revision": manifest["source_revision"],
                "automl_commit": manifest["automl_commit"],
            }
        )

    result = {
        "schema": "2048-ml.candidate-classifier-pilot-verification",
        "schema_version": 1,
        "models_verified": len(summary),
        "same_holdout_rows": len(common_rows or []),
        "holdout_game_ids": [17, 18, 19],
        "models": summary,
        "interpretation": "Small exploratory rollout-label diagnostic; not a policy-quality estimate or model-selection result.",
    }
    output = args.report_dir / "verification.json"
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(f"verified {len(summary)} candidates on {len(common_rows or [])} common holdout rows: {output}")


if __name__ == "__main__":
    main()
