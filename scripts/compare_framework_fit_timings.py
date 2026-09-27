#!/usr/bin/env python3
"""Compare repeated per-case fit/predict timings from pinned seed-42 runs.

This is a fixed-configuration diagnostic, not a matched search-budget benchmark.
"""

import argparse
import csv
import hashlib
import json
from pathlib import Path
from statistics import median


def read_json(path: Path):
    return json.loads(path.read_text())


def load_run(directory: Path, prefix: str):
    manifest = read_json(directory / f"{prefix}manifest.json")
    results_path = directory / f"{prefix}results.json"
    results = read_json(results_path)
    if hashlib.sha256(results_path.read_bytes()).hexdigest() != manifest["results_sha256"]:
        raise ValueError(f"result digest mismatch: {results_path}")
    if manifest["seed"] != 42 or manifest["successful_runs"] != 15 or manifest["failed_runs"] != 0:
        raise ValueError(f"unexpected run status or seed: {directory}")
    by_case = {(row["dataset"], row["model"]): row for row in results}
    if len(by_case) != 15 or any(row["status"] != "success" for row in results):
        raise ValueError(f"expected 15 successful dataset/model cases: {directory}")
    return manifest, by_case


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reports-dir", type=Path, default=Path("reports/framework_validation"))
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(
            "reports/framework_validation/"
            "pinned-82d8483-vs-sklearn-seed42-single-thread-fit-predict.csv"
        ),
    )
    args = parser.parse_args()
    root = args.reports_dir
    automl_runs = [
        load_run(root / f"pinned-82d8483-rayon1-seed42-run-{n}", "framework-validation-")
        for n in (1, 2)
    ]
    sklearn_runs = [
        load_run(root / f"sklearn-1.6.1-seed42-run-{n}", "sklearn-framework-validation-")
        for n in (1, 2)
    ]

    if any(run[0]["configuration"].get("thread_limit") != 1 for run in sklearn_runs):
        raise ValueError("the sklearn measurements must use one thread")
    if automl_runs[0][0]["configuration"] != automl_runs[1][0]["configuration"]:
        raise ValueError("AutoML run configurations differ")
    if any(run[0]["automl_commit"] != automl_runs[0][0]["automl_commit"] for run in automl_runs):
        raise ValueError("AutoML revisions differ")

    keys = set(automl_runs[0][1])
    if any(set(run[1]) != keys for run in (*automl_runs[1:], *sklearn_runs)):
        raise ValueError("dataset/model case sets differ")
    for automl_manifest, _ in automl_runs:
        for sklearn_manifest, _ in sklearn_runs:
            if automl_manifest["seed"] != sklearn_manifest["seed"]:
                raise ValueError("comparison seeds differ")
            automl_splits = {
                row["dataset"]: read_json(Path(row["split_manifest"]))
                for row in automl_manifest["dataset_splits"]
            }
            sklearn_reference = Path(sklearn_manifest["automl_reference_run"])
            sklearn_splits = {
                dataset: read_json(sklearn_reference / f"{dataset}.split.json")
                for dataset in sklearn_manifest["datasets"]
            }
            for dataset in automl_splits:
                for role in ("train_source_rows", "test_source_rows"):
                    if automl_splits[dataset][role] != sklearn_splits[dataset][role]:
                        raise ValueError(f"split mismatch for {dataset}: {role}")

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(
            [
                "dataset",
                "model",
                "repeats",
                "automl_run1_seconds",
                "automl_run2_seconds",
                "automl_median_seconds",
                "sklearn_run1_seconds",
                "sklearn_run2_seconds",
                "sklearn_median_seconds",
                "sklearn_over_automl_median_ratio",
            ]
        )
        for key in sorted(keys):
            auto_times = [run[1][key]["fit_predict_seconds"] for run in automl_runs]
            sklearn_times = [run[1][key]["fit_predict_seconds"] for run in sklearn_runs]
            if any(value is None for value in (*auto_times, *sklearn_times)):
                raise ValueError(f"missing fit/predict timing for {key}")
            auto_median = median(auto_times)
            sklearn_median = median(sklearn_times)
            writer.writerow(
                [
                    *key,
                    2,
                    *(f"{value:.9f}" for value in auto_times),
                    f"{auto_median:.9f}",
                    *(f"{value:.9f}" for value in sklearn_times),
                    f"{sklearn_median:.9f}",
                    f"{sklearn_median / auto_median:.6f}",
                ]
            )
    print(f"wrote {len(keys)} repeated per-case timing comparisons to {args.output}")


if __name__ == "__main__":
    main()
