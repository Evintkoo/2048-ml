#!/usr/bin/env python3
"""Verify retained per-case framework resource measurements and provenance."""

import argparse
import csv
import hashlib
import json
import statistics
from pathlib import Path


DATASETS = ("iris", "wine", "breast_cancer_wisconsin_diagnostic")
MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
IMPLEMENTATIONS = ("automl", "sklearn")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_csv(path):
    with path.open(newline="") as stream:
        return list(csv.DictReader(stream))


def load_json(path):
    return json.loads(path.read_text())


def prediction_rows(path):
    return [
        (int(row["source_row"]), int(row["actual_label"]), int(row["predicted_label"]))
        for row in read_csv(path)
    ]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--report-dir", type=Path, default=Path("reports/framework_validation/case-resource-matrix")
    )
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = (root / args.report_dir).resolve()
    manifest_path = report / "case-resource-manifest.json"
    runs_path = report / "case-resource-runs.csv"
    summary_path = report / "case-resource-summary.csv"
    manifest = load_json(manifest_path)
    assert manifest["seed"] == 42
    assert manifest["configuration"] == {
        "n_estimators": 32,
        "max_depth": 8,
        "test_fraction": 0.2,
    }
    assert manifest["selected_datasets"] == list(DATASETS)
    assert manifest["selected_models"] == list(MODELS)
    assert manifest["repeats"] == 2
    assert digest(runs_path) == manifest["runs_csv_sha256"]
    assert digest(summary_path) == manifest["summary_csv_sha256"]

    runs = read_csv(runs_path)
    expected = {
        (implementation, dataset, model, repeat)
        for implementation in IMPLEMENTATIONS
        for dataset in DATASETS
        for model in MODELS
        for repeat in (1, 2)
    }
    keyed = {}
    for row in runs:
        key = (row["implementation"], row["dataset"], row["model"], int(row["repeat"]))
        assert key not in keyed
        keyed[key] = row
    assert set(keyed) == expected

    validated = []
    split_by_dataset = {}
    predictions_by_case = {}
    for key, row in keyed.items():
        implementation, dataset, model, repeat = key
        assert int(row["peak_rss_bytes"]) > 0
        assert float(row["wall_seconds"]) > 0
        run_dir = root / row["output_dir"]
        if implementation == "automl":
            run_manifest_path = run_dir / "framework-validation-manifest.json"
            results_path = run_dir / "framework-validation-results.json"
        else:
            run_manifest_path = run_dir / "sklearn-framework-validation-manifest.json"
            results_path = run_dir / "sklearn-framework-validation-results.json"
        assert digest(run_manifest_path) == row["manifest_sha256"]
        assert digest(results_path) == row["results_sha256"]
        run_manifest = load_json(run_manifest_path)
        results = load_json(results_path)
        assert len(results) == 1
        result = results[0]
        assert result["status"] == "success"
        assert result["dataset"] == dataset and result["model"] == model
        assert result["seed"] == 42
        assert run_manifest["seed"] == 42
        if implementation == "automl":
            assert run_manifest["root_worktree_dirty"] is False
            assert run_manifest["automl_worktree_dirty"] is False
            assert run_manifest["automl_commit"] == "82d848323eed5e2af86d046d529916c448f2442c"
            assert run_manifest["configuration"]["n_estimators"] == 32
            assert run_manifest["configuration"]["max_depth"] == 8
            assert run_manifest["successful_runs"] == 1 and run_manifest["failed_runs"] == 0
            assert result["save_load_equivalent"] is True
            assert digest(Path(result["serialized_model"])) == result["model_sha256"]
            split_path = Path(run_manifest["dataset_splits"][0]["split_manifest"])
        else:
            assert run_manifest["automl_reference_run"].endswith("pinned-82d8483-run-1")
            assert run_manifest["configuration"]["n_estimators"] == 32
            assert run_manifest["configuration"]["max_depth"] == 8
            assert run_manifest["successful_runs"] == 1 and run_manifest["failed_runs"] == 0
            split_path = root / "reports/framework_validation/pinned-82d8483-run-1" / f"{dataset}.split.json"

        split = load_json(split_path)
        assert split["dataset"] == dataset and split["global_seed"] == 42
        assert split["source_file_sha256"]
        assert split["test_fraction_requested"] == 0.2
        source_rows = split["test_source_rows"]
        split_key = (implementation, dataset)
        split_signature = (
            split["source_file_sha256"],
            tuple(split["class_labels"]),
            tuple(source_rows),
        )
        if split_key in split_by_dataset:
            assert split_by_dataset[split_key] == split_signature
        split_by_dataset[split_key] = split_signature

        pred_path = Path(result["predictions_csv"])
        if not pred_path.is_absolute():
            pred_path = root / pred_path
        assert digest(pred_path) == result["predictions_sha256"]
        predictions = prediction_rows(pred_path)
        assert [row[0] for row in predictions] == source_rows
        predictions_by_case[key] = predictions
        validated.append(key)

    for dataset in DATASETS:
        for model in MODELS:
            rows_for_splits = [
                split_by_dataset[(implementation, dataset)]
                for implementation in IMPLEMENTATIONS
            ]
            assert rows_for_splits[0] == rows_for_splits[1]
            for implementation in IMPLEMENTATIONS:
                first = predictions_by_case[(implementation, dataset, model, 1)]
                second = predictions_by_case[(implementation, dataset, model, 2)]
                assert first == second

    summary = read_csv(summary_path)
    assert len(summary) == len(IMPLEMENTATIONS) * len(DATASETS) * len(MODELS)
    summary_keys = {
        (row["implementation"], row["dataset"], row["model"])
        for row in summary
    }
    expected_summary_keys = {
        (implementation, dataset, model)
        for implementation in IMPLEMENTATIONS
        for dataset in DATASETS
        for model in MODELS
    }
    assert summary_keys == expected_summary_keys
    for row in summary:
        implementation, dataset, model = row["implementation"], row["dataset"], row["model"]
        measurements = [
            keyed[(implementation, dataset, model, repeat)] for repeat in (1, 2)
        ]
        rss = [int(item["peak_rss_bytes"]) for item in measurements]
        wall = [float(item["wall_seconds"]) for item in measurements]
        assert int(row["repeats"]) == 2
        assert int(row["median_peak_rss_bytes"]) == int(statistics.median(rss))
        assert int(row["min_peak_rss_bytes"]) == min(rss)
        assert int(row["max_peak_rss_bytes"]) == max(rss)
        assert row["median_process_wall_seconds"] == f"{statistics.median(wall):.6f}"

    print(f"verified {len(validated)} isolated case-process records and {len(summary)} summaries")


if __name__ == "__main__":
    main()
