#!/usr/bin/env python3
"""Run the same fixed six-configuration grid on the AutoML split protocol."""

import argparse
import csv
import hashlib
import json
import platform
import sys
import time
from pathlib import Path

import joblib
import numpy as np
import scipy
import sklearn
import threadpoolctl
from sklearn.ensemble import ExtraTreesClassifier, RandomForestClassifier
from sklearn.metrics import accuracy_score, confusion_matrix, f1_score
from threadpoolctl import threadpool_limits

from run_sklearn_framework_baseline import digest, inner_training_rows, load_dataset


def classifier(model, n_estimators, max_depth, seed):
    constructor = {
        "random_forest": RandomForestClassifier,
        "extra_trees": ExtraTreesClassifier,
    }[model]
    return constructor(
        n_estimators=n_estimators,
        max_depth=max_depth,
        random_state=seed,
        n_jobs=1,
    )


def write_predictions(path, row_ids, actual, predicted):
    with path.open("w", newline="") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(["source_row", "actual_label", "predicted_label"])
        writer.writerows(zip(row_ids, actual, predicted))


def run(protocol_path: Path, data_dir: Path, output_dir: Path):
    if output_dir.exists():
        raise SystemExit(f"refusing to overwrite {output_dir}")
    protocol = json.loads(protocol_path.read_text())
    output_dir.mkdir(parents=True)
    model_results = []
    grid = protocol["grid"]
    with threadpool_limits(limits=1):
        active_threadpools = threadpoolctl.threadpool_info()
        for dataset_protocol in protocol["datasets"]:
            dataset_name = dataset_protocol["dataset"]
            source_path, rows = load_dataset(dataset_name, data_dir)
            if hashlib.sha256(source_path.read_bytes()).hexdigest() != dataset_protocol["source_sha256"]:
                raise ValueError(f"dataset source digest changed: {dataset_name}")
            row_by_id = {row_id: (features, label) for row_id, features, label in rows}
            outer_train = dataset_protocol["outer_train_source_rows"]
            outer_test = dataset_protocol["outer_test_source_rows"]
            fit_ids = dataset_protocol["inner_fit_source_rows"]
            validation_ids = dataset_protocol["inner_validation_source_rows"]
            model_fit_ids = dataset_protocol["model_training_source_rows"]
            automl_internal_validation_ids = dataset_protocol[
                "automl_internal_validation_source_rows"
            ]
            expected_fit = inner_training_rows(
                rows, outer_train, dataset_protocol["validation_fraction"]
            )
            if expected_fit != fit_ids:
                raise ValueError(f"AutoML and sklearn fit rows differ: {dataset_name}")
            fit_id_set = set(fit_ids)
            expected_validation = [
                row_id
                for label in sorted({row_by_id[row_id][1] for row_id in outer_train})
                for row_id in outer_train
                if row_by_id[row_id][1] == label and row_id not in fit_id_set
            ]
            if expected_validation != validation_ids:
                raise ValueError(f"AutoML and sklearn validation rows differ: {dataset_name}")

            expected_model_fit = inner_training_rows(
                rows, fit_ids, dataset_protocol["validation_fraction"]
            )
            if expected_model_fit != model_fit_ids:
                raise ValueError(f"AutoML native training rows differ: {dataset_name}")
            expected_internal_validation = [
                row_id
                for label in sorted({row_by_id[row_id][1] for row_id in fit_ids})
                for row_id in fit_ids
                if row_by_id[row_id][1] == label and row_id not in set(model_fit_ids)
            ]
            if expected_internal_validation != automl_internal_validation_ids:
                raise ValueError(f"AutoML internal validation rows differ: {dataset_name}")
            if set(validation_ids) & set(model_fit_ids):
                raise ValueError(f"external validation rows leaked into fit: {dataset_name}")

            x_fit = np.asarray(
                [row_by_id[row_id][0] for row_id in model_fit_ids], dtype=np.float64
            )
            y_fit = np.asarray(
                [row_by_id[row_id][1] for row_id in model_fit_ids], dtype=np.int64
            )
            x_validation = np.asarray(
                [row_by_id[row_id][0] for row_id in validation_ids], dtype=np.float64
            )
            y_validation = np.asarray(
                [row_by_id[row_id][1] for row_id in validation_ids], dtype=np.int64
            )
            x_test = np.asarray([row_by_id[row_id][0] for row_id in outer_test], dtype=np.float64)
            y_test = np.asarray([row_by_id[row_id][1] for row_id in outer_test], dtype=np.int64)

            for model_index, model_name in enumerate(protocol["models"]):
                model_seed = (
                    protocol["seed"]
                    + (0 if dataset_name == "iris" else 1 if dataset_name == "wine" else 2)
                    + (model_index + 1) * 1000
                )
                trials = []
                selected_model = None
                selected_accuracy = -1.0
                selected_configuration = None
                for trial_index, configuration in enumerate(grid):
                    model = classifier(
                        model_name,
                        configuration["n_estimators"],
                        configuration["max_depth"],
                        model_seed,
                    )
                    started = time.perf_counter()
                    model.fit(x_fit, y_fit)
                    predicted_validation = model.predict(x_validation).astype(np.int64)
                    elapsed = time.perf_counter() - started
                    accuracy = float(accuracy_score(y_validation, predicted_validation))
                    macro_f1 = float(
                        f1_score(
                            y_validation,
                            predicted_validation,
                            labels=range(len(dataset_protocol["class_labels"])),
                            average="macro",
                            zero_division=0,
                        )
                    )
                    trials.append(
                        {
                            "trial": trial_index,
                            **configuration,
                            "seed": model_seed,
                            "validation_accuracy": accuracy,
                            "validation_macro_f1": macro_f1,
                            "fit_and_validation_predict_seconds": elapsed,
                        }
                    )
                    if accuracy > selected_accuracy:
                        selected_accuracy = accuracy
                        selected_model = model
                        selected_configuration = configuration

                predicted_test = selected_model.predict(x_test).astype(np.int64)
                case_dir = output_dir / dataset_name / model_name
                case_dir.mkdir(parents=True, exist_ok=True)
                model_path = case_dir / f"{model_name}.joblib"
                prediction_path = case_dir / f"{model_name}.predictions.csv"
                joblib.dump(selected_model, model_path)
                write_predictions(prediction_path, outer_test, y_test, predicted_test)
                model_results.append(
                    {
                        "dataset": dataset_name,
                        "model": model_name,
                        "seed": model_seed,
                        "trial_budget": len(grid),
                        "validation_metric": "accuracy",
                        "trials": trials,
                        "selected_configuration": {
                            **selected_configuration,
                            "validation_accuracy": selected_accuracy,
                        },
                        "outer_test": {
                            "n_rows": len(outer_test),
                            "accuracy": float(accuracy_score(y_test, predicted_test)),
                            "macro_f1": float(
                                f1_score(
                                    y_test,
                                    predicted_test,
                                    labels=range(len(dataset_protocol["class_labels"])),
                                    average="macro",
                                    zero_division=0,
                                )
                            ),
                            "confusion_matrix": confusion_matrix(
                                y_test,
                                predicted_test,
                                labels=range(len(dataset_protocol["class_labels"])),
                            ).tolist(),
                            "model_file": str(model_path),
                            "model_sha256": digest(model_path),
                            "predictions_file": str(prediction_path),
                            "predictions_sha256": digest(prediction_path),
                        },
                    }
                )

    results_path = output_dir / "sklearn-matched-search-results.json"
    results_path.write_text(json.dumps(model_results, indent=2) + "\n")
    manifest = {
        "schema": "2048-ml.sklearn-matched-grid-search",
        "schema_version": 1,
        "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "source_revision": subprocess_revision(),
        "root_worktree_dirty": subprocess_worktree_dirty(),
        "runner_sha256": digest(Path(__file__).resolve()),
        "helper_sha256": digest(Path(__file__).with_name("run_sklearn_framework_baseline.py")),
        "protocol_file": str(protocol_path),
        "protocol_sha256": digest(protocol_path),
        "results_file": str(results_path),
        "results_sha256": digest(results_path),
        "python": sys.version,
        "platform": platform.platform(),
        "packages": {
            "scikit_learn": sklearn.__version__,
            "numpy": np.__version__,
            "scipy": scipy.__version__,
            "joblib": joblib.__version__,
            "threadpoolctl": threadpoolctl.__version__,
        },
        "thread_limit": 1,
        "threadpools": active_threadpools,
        "successful_cases": len(model_results),
    }
    manifest_path = output_dir / "sklearn-matched-search-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"matched sklearn grid artifacts written to {output_dir}")


def subprocess_revision():
    import subprocess

    return subprocess.run(
        ["git", "rev-parse", "HEAD"], check=True, capture_output=True, text=True
    ).stdout.strip()


def subprocess_worktree_dirty():
    import subprocess

    result = subprocess.run(
        ["git", "status", "--porcelain"], check=True, capture_output=True, text=True
    )
    return bool(result.stdout.strip())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--protocol",
        type=Path,
        default=Path(
            "reports/framework_validation/matched-grid-search/matched-search-protocol.json"
        ),
    )
    parser.add_argument("--data-dir", type=Path, default=Path("data/framework_validation"))
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("reports/framework_validation/matched-grid-search/sklearn"),
    )
    args = parser.parse_args()
    run(args.protocol, args.data_dir, args.output_dir)


if __name__ == "__main__":
    main()
