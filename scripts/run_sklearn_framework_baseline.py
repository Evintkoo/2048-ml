#!/usr/bin/env python3
"""Run a fixed-configuration sklearn comparison on retained AutoML splits."""

import argparse
import hashlib
import json
import platform
import resource
import sys
import time
from pathlib import Path

import numpy as np
import sklearn
import scipy
import joblib
from sklearn.ensemble import AdaBoostClassifier, ExtraTreesClassifier, RandomForestClassifier
from sklearn.metrics import accuracy_score, confusion_matrix, f1_score, roc_auc_score
from sklearn.naive_bayes import GaussianNB
from sklearn.neighbors import KNeighborsClassifier
from sklearn.tree import DecisionTreeClassifier
import threadpoolctl
from threadpoolctl import threadpool_limits


DATASETS = (
    "iris",
    "wine",
    "breast_cancer_wisconsin_diagnostic",
)
MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
SEED = 42


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_dataset(name: str, data_dir: Path):
    if name == "iris":
        path = data_dir / "iris/iris.data"
        labels = {"Iris-setosa": 0, "Iris-versicolor": 1, "Iris-virginica": 2}
        rows = []
        for row_id, line in enumerate(path.read_text().splitlines()):
            if line.strip():
                fields = line.split(",")
                rows.append((row_id, [float(v) for v in fields[:4]], labels[fields[4]]))
    elif name == "wine":
        path = data_dir / "wine/wine.data"
        rows = []
        for row_id, line in enumerate(path.read_text().splitlines()):
            if line.strip():
                fields = line.split(",")
                rows.append((row_id, [float(v) for v in fields[1:]], int(fields[0]) - 1))
    elif name == "breast_cancer_wisconsin_diagnostic":
        path = data_dir / "breast_cancer_wisconsin_diagnostic/wdbc.data"
        labels = {"B": 0, "M": 1}
        rows = []
        for row_id, line in enumerate(path.read_text().splitlines()):
            if line.strip():
                fields = line.split(",")
                rows.append((row_id, [float(v) for v in fields[2:]], labels[fields[1]]))
    else:
        raise ValueError(f"unknown dataset: {name}")
    return path, rows


def classifier(name: str):
    if name == "random_forest":
        return RandomForestClassifier(
            n_estimators=32,
            max_depth=8,
            random_state=SEED,
            n_jobs=1,
        )
    if name == "extra_trees":
        return ExtraTreesClassifier(
            n_estimators=32,
            max_depth=8,
            random_state=SEED,
            n_jobs=1,
        )
    if name == "adaboost":
        return AdaBoostClassifier(
            estimator=DecisionTreeClassifier(max_depth=1, random_state=SEED),
            n_estimators=32,
            learning_rate=1.0,
            random_state=SEED,
        )
    if name == "knn":
        return KNeighborsClassifier(n_neighbors=5, weights="uniform", metric="euclidean", n_jobs=1)
    if name == "naive_bayes":
        return GaussianNB(var_smoothing=1e-9)
    raise ValueError(f"unknown model: {name}")


def inner_training_rows(rows, outer_train_ids, validation_fraction):
    """Mirror AutoML's sorted-class, trailing per-class validation holdback."""
    by_class = {}
    row_by_id = {row_id: (features, label) for row_id, features, label in rows}
    for row_id in outer_train_ids:
        _, label = row_by_id[row_id]
        by_class.setdefault(label, []).append(row_id)
    selected = []
    for label in sorted(by_class):
        class_ids = by_class[label]
        validation_count = max(1, int(len(class_ids) * validation_fraction))
        validation_count = min(validation_count, max(len(class_ids) - 1, 0))
        selected.extend(class_ids[: len(class_ids) - validation_count])
    selected_set = set(selected)
    # AutoML concatenates classes in label order before fitting each estimator.
    fit_ids = [row_id for label in sorted(by_class) for row_id in by_class[label] if row_id in selected_set]
    return fit_ids


def run(data_dir: Path, split_dir: Path, output_dir: Path):
    output_dir.mkdir(parents=True, exist_ok=True)
    matrix_started = time.perf_counter()
    records = []
    split_digests = {}
    source_digests = {}
    with threadpool_limits(limits=1):
        active_threadpools = threadpoolctl.threadpool_info()
        for dataset_name in DATASETS:
            split_path = split_dir / f"{dataset_name}.split.json"
            split = json.loads(split_path.read_text())
            source_path, rows = load_dataset(dataset_name, data_dir)
            actual_source_digest = digest(source_path)
            if split["source_file_sha256"] != actual_source_digest:
                raise ValueError(f"dataset digest differs from split manifest: {dataset_name}")
            split_digests[dataset_name] = digest(split_path)
            source_digests[dataset_name] = actual_source_digest
            row_by_id = {row_id: (features, label) for row_id, features, label in rows}
            train_ids = split["train_source_rows"]
            test_ids = split["test_source_rows"]
            if set(train_ids) & set(test_ids):
                raise ValueError(f"train/test overlap in {dataset_name}")
            if set(train_ids) | set(test_ids) != set(row_by_id):
                raise ValueError(f"split does not cover source rows in {dataset_name}")
            fit_ids = inner_training_rows(rows, train_ids, split["inner_validation_fraction"])
            x_train = np.asarray([row_by_id[row_id][0] for row_id in fit_ids], dtype=np.float64)
            y_train = np.asarray([row_by_id[row_id][1] for row_id in fit_ids], dtype=np.int64)
            x_test = np.asarray([row_by_id[row_id][0] for row_id in test_ids], dtype=np.float64)
            y_test = np.asarray([row_by_id[row_id][1] for row_id in test_ids], dtype=np.int64)
            class_ids = list(range(len(split["class_labels"])))
            for model_name in MODELS:
                model = classifier(model_name)
                started = time.perf_counter()
                model.fit(x_train, y_train)
                predicted = model.predict(x_test).astype(np.int64)
                probabilities = model.predict_proba(x_test)
                elapsed = time.perf_counter() - started
                positive_auc = None
                if len(class_ids) == 2:
                    positive_column = list(model.classes_).index(1)
                    positive_auc = float(roc_auc_score(y_test, probabilities[:, positive_column]))
                record = {
                    "dataset": dataset_name,
                    "model": model_name,
                    "status": "success",
                    "seed": SEED,
                    "train_rows_outer": len(train_ids),
                    "train_rows_fit": len(fit_ids),
                    "test_rows": len(test_ids),
                    "accuracy": float(accuracy_score(y_test, predicted)),
                    "macro_f1": float(f1_score(y_test, predicted, labels=class_ids, average="macro", zero_division=0)),
                    "confusion_matrix": confusion_matrix(y_test, predicted, labels=class_ids).tolist(),
                    "positive_class_roc_auc": positive_auc,
                    "fit_predict_seconds": elapsed,
                }
                records.append(record)
                predictions_path = output_dir / f"{dataset_name}__{model_name}.predictions.csv"
                with predictions_path.open("w", encoding="utf-8") as output:
                    output.write("source_row,actual_label,predicted_label,positive_class_score\n")
                    for i, row_id in enumerate(test_ids):
                        score = probabilities[i, list(model.classes_).index(1)] if len(class_ids) == 2 else ""
                        output.write(f"{row_id},{y_test[i]},{predicted[i]},{score}\n")
                record["predictions_csv"] = str(predictions_path)
                record["predictions_sha256"] = digest(predictions_path)

    results_path = output_dir / "sklearn-framework-validation-results.json"
    results_path.write_text(json.dumps(records, indent=2) + "\n")
    matrix_elapsed = time.perf_counter() - matrix_started
    max_rss = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    max_rss_bytes = int(max_rss if sys.platform == "darwin" else max_rss * 1024)
    automl_results_path = split_dir / "framework-validation-results.json"
    automl_records = json.loads(automl_results_path.read_text())
    automl_by_key = {(row["dataset"], row["model"]): row for row in automl_records}
    score_comparison = []
    for row in records:
        key = (row["dataset"], row["model"])
        auto = automl_by_key[key]
        auto_predictions = Path(auto["predictions_csv"])
        sklearn_predictions = Path(row["predictions_csv"])
        auto_lines = auto_predictions.read_text().splitlines()
        sklearn_lines = sklearn_predictions.read_text().splitlines()
        if [line.split(",")[:2] for line in auto_lines] != [line.split(",")[:2] for line in sklearn_lines]:
            raise ValueError(f"prediction files do not use the same test-row/label sequence: {key}")
        score_comparison.append(
            {
                "dataset": row["dataset"],
                "model": row["model"],
                "test_rows": row["test_rows"],
                "automl_accuracy": auto["accuracy"],
                "sklearn_accuracy": row["accuracy"],
                "sklearn_minus_automl_accuracy": row["accuracy"] - auto["accuracy"],
                "automl_macro_f1": auto["macro_f1"],
                "sklearn_macro_f1": row["macro_f1"],
                "sklearn_minus_automl_macro_f1": row["macro_f1"] - auto["macro_f1"],
                "automl_fit_predict_seconds": auto["fit_predict_seconds"],
                "sklearn_fit_predict_seconds": row["fit_predict_seconds"],
                "predicted_labels_equal": [line.split(",")[2] for line in auto_lines]
                == [line.split(",")[2] for line in sklearn_lines],
            }
        )
    comparison_path = output_dir / "automl-sklearn-score-differences.json"
    comparison = {
        "protocol": "same source rows and labels; fixed configurations; descriptive paired score differences",
        "automl_reference_manifest_sha256": digest(split_dir / "framework-validation-manifest.json"),
        "automl_reference_results_sha256": digest(automl_results_path),
        "comparisons": score_comparison,
        "cases": len(score_comparison),
        "inferential_tests_performed": False,
        "interpretation": "One fixed split and seed; implementation and parallelism differences preclude superiority or runtime claims.",
    }
    comparison_path.write_text(json.dumps(comparison, indent=2) + "\n")
    manifest = {
        "schema": "2048-ml.sklearn-framework-baseline",
        "schema_version": 1,
        "protocol": "same UCI sources, outer test rows, feature policy, and AutoML inner holdback boundary",
        "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "seed": SEED,
        "automl_reference_run": str(split_dir),
        "configuration": {
            "n_estimators": 32,
            "max_depth": 8,
            "validation_fraction": 0.1,
            "validation_boundary": "per-class trailing rows after source-order outer-train partition, matching AutoML stratified_split",
            "preprocessing": "none; raw numeric features; WDBC ID excluded",
            "thread_limit": 1,
            "model_specific": {
                "random_forest": "sklearn defaults except n_estimators, max_depth, random_state, and n_jobs",
                "extra_trees": "sklearn defaults except n_estimators, max_depth, random_state, and n_jobs",
                "adaboost": "32 depth-1 decision stumps and learning_rate=1.0 (SAMME behavior in pinned sklearn)",
                "knn": "k=5, uniform weights, Euclidean distance",
                "naive_bayes": "GaussianNB var_smoothing=1e-9",
            },
        },
        "datasets": {
            name: {
                "source_sha256": source_digests[name],
                "split_manifest_sha256": split_digests[name],
            }
            for name in DATASETS
        },
        "environment": {
            "python": platform.python_version(),
            "scikit_learn": sklearn.__version__,
            "numpy": np.__version__,
            "scipy": scipy.__version__,
            "joblib": joblib.__version__,
            "threadpoolctl": threadpoolctl.__version__,
            "platform": platform.platform(),
            "architecture": platform.machine(),
            "logical_cpus": __import__("os").cpu_count(),
            "max_rss_bytes_process": max_rss_bytes,
            "thread_limit": 1,
            "threadpools_during_fit": active_threadpools,
        },
        "runner_sha256": digest(Path(__file__).resolve()),
        "results_file": results_path.name,
        "results_sha256": digest(results_path),
        "comparison_file": comparison_path.name,
        "comparison_sha256": digest(comparison_path),
        "successful_runs": len(records),
        "failed_runs": 0,
        "matrix_wall_seconds": matrix_elapsed,
        "limitations": [
            "Fixed-configuration comparison only; implementation details and some defaults differ from AutoML",
            "One dataset split and seed; no superiority inference",
            "Python process peak RSS is aggregate across the matrix, not a per-model memory profile",
            "Single-thread sklearn timing is not directly comparable to unpinned AutoML/Rayon parallelism",
            "No search budget, CLI/library parity, or independent replication comparison",
        ],
    }
    manifest_path = output_dir / "sklearn-framework-validation-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"completed {len(records)} sklearn cases; manifest={manifest_path}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--data-dir", type=Path, default=Path("data/framework_validation"))
    parser.add_argument(
        "--split-dir",
        type=Path,
        default=Path("reports/framework_validation/pinned-82d8483-run-1"),
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("reports/framework_validation/sklearn-1.6.1-seed42"),
    )
    args = parser.parse_args()
    run(args.data_dir, args.split_dir, args.output_dir)


if __name__ == "__main__":
    main()
