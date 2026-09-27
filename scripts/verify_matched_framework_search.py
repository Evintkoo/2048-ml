#!/usr/bin/env python3
"""Verify the shared grid, split lineage, and held-out results for both frameworks."""

import argparse
import csv
import hashlib
import json
import math
from pathlib import Path


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_json(path):
    return json.loads(path.read_text())


def read_predictions(path):
    with path.open(newline="") as stream:
        return list(csv.DictReader(stream))


def classification(actual, predicted, class_count):
    confusion = [[0] * class_count for _ in range(class_count)]
    for truth, guess in zip(actual, predicted):
        confusion[truth][guess] += 1
    f1_values = []
    for label in range(class_count):
        tp = confusion[label][label]
        fp = sum(confusion[row][label] for row in range(class_count)) - tp
        fn = sum(confusion[label]) - tp
        denominator = 2 * tp + fp + fn
        f1_values.append(0.0 if denominator == 0 else 2 * tp / denominator)
    return {
        "n_rows": len(actual),
        "accuracy": sum(a == p for a, p in zip(actual, predicted)) / len(actual),
        "macro_f1": sum(f1_values) / class_count,
        "confusion_matrix": confusion,
    }


def close(left, right):
    return math.isclose(float(left), float(right), rel_tol=0.0, abs_tol=1e-12)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--report-dir",
        type=Path,
        default=Path("reports/framework_validation/matched-grid-search-2026-09-27"),
    )
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = (root / args.report_dir).resolve()
    protocol_path = report / "matched-search-protocol.json"
    protocol = read_json(protocol_path)
    assert protocol["schema_version"] == 2, "protocol predates validation-leakage correction"
    automl_results_path = report / "automl-matched-search-results.json"
    automl_manifest_path = report / "automl-matched-search-manifest.json"
    sklearn_dir = report / "sklearn"
    sklearn_results_path = sklearn_dir / "sklearn-matched-search-results.json"
    sklearn_manifest_path = sklearn_dir / "sklearn-matched-search-manifest.json"
    automl_manifest = read_json(automl_manifest_path)
    sklearn_manifest = read_json(sklearn_manifest_path)
    assert automl_manifest["protocol_sha256"] == digest(protocol_path)
    assert sklearn_manifest["protocol_sha256"] == digest(protocol_path)
    assert automl_manifest["results_sha256"] == digest(automl_results_path)
    assert sklearn_manifest["results_sha256"] == digest(sklearn_results_path)

    automl_results = read_json(automl_results_path)
    sklearn_results = read_json(sklearn_results_path)
    automl_cases = {(r["dataset"], r["model"]): r for r in automl_results}
    sklearn_cases = {(r["dataset"], r["model"]): r for r in sklearn_results}
    expected_cases = {
        (dataset["dataset"], model)
        for dataset in protocol["datasets"]
        for model in protocol["models"]
    }
    assert set(automl_cases) == expected_cases
    assert set(sklearn_cases) == expected_cases
    expected_grid = [
        (entry["n_estimators"], entry["max_depth"]) for entry in protocol["grid"]
    ]
    comparisons = []

    for dataset_protocol in protocol["datasets"]:
        dataset = dataset_protocol["dataset"]
        class_count = len(dataset_protocol["class_labels"])
        expected_test_rows = dataset_protocol["outer_test_source_rows"]
        fit_rows = dataset_protocol["inner_fit_source_rows"]
        validation_rows = dataset_protocol["inner_validation_source_rows"]
        model_training_rows = dataset_protocol["model_training_source_rows"]
        automl_internal_validation_rows = dataset_protocol[
            "automl_internal_validation_source_rows"
        ]
        assert set(validation_rows).isdisjoint(model_training_rows)
        assert set(validation_rows).isdisjoint(automl_internal_validation_rows)
        assert set(expected_test_rows).isdisjoint(
            set(model_training_rows) | set(automl_internal_validation_rows) | set(validation_rows)
        )
        assert set(model_training_rows).isdisjoint(automl_internal_validation_rows)
        assert set(model_training_rows) | set(automl_internal_validation_rows) == set(fit_rows)
        assert set(expected_test_rows).isdisjoint(
            fit_rows + validation_rows
        )
        assert set(dataset_protocol["inner_fit_source_rows"]).isdisjoint(
            validation_rows
        )

        for model in protocol["models"]:
            key = (dataset, model)
            pair = []
            for implementation, case in (
                ("automl", automl_cases[key]),
                ("sklearn", sklearn_cases[key]),
            ):
                assert case["trial_budget"] == len(expected_grid)
                trials = case["trials"]
                assert len(trials) == len(expected_grid)
                observed_grid = [
                    (trial["n_estimators"], trial["max_depth"]) for trial in trials
                ]
                assert observed_grid == expected_grid
                assert [trial["trial"] for trial in trials] == list(range(len(expected_grid)))
                assert all(trial["seed"] == case["seed"] for trial in trials)
                selected_trial = max(trials, key=lambda trial: trial["validation_accuracy"])
                selected = case["selected_configuration"]
                assert selected["n_estimators"] == selected_trial["n_estimators"]
                assert selected["max_depth"] == selected_trial["max_depth"]
                assert close(
                    selected["validation_accuracy"], selected_trial["validation_accuracy"]
                )
                outer = case["outer_test"]
                pred_path = root / outer["predictions_file"]
                model_path = root / outer["fit_model_file"] if implementation == "automl" else root / outer["model_file"]
                if implementation == "sklearn":
                    pred_path = root / outer["predictions_file"]
                assert digest(pred_path) == outer["predictions_sha256"]
                assert digest(model_path) == outer["model_sha256"]
                rows = read_predictions(pred_path)
                assert [int(row["source_row"]) for row in rows] == expected_test_rows
                actual = [int(row["actual_label"]) for row in rows]
                predicted = [int(row["predicted_label"]) for row in rows]
                measured = classification(actual, predicted, class_count)
                assert measured["n_rows"] == outer["n_rows"]
                assert close(measured["accuracy"], outer["accuracy"])
                assert close(measured["macro_f1"], outer["macro_f1"])
                assert measured["confusion_matrix"] == outer["confusion_matrix"]
                pair.append(
                    {
                        "implementation": implementation,
                        "selected_configuration": {
                            "n_estimators": selected["n_estimators"],
                            "max_depth": selected["max_depth"],
                        },
                        "validation_accuracy": selected["validation_accuracy"],
                        "outer_test_accuracy": measured["accuracy"],
                        "outer_test_macro_f1": measured["macro_f1"],
                        "outer_test_predictions": predicted,
                        "outer_test_actual": actual,
                    }
                )
            auto, reference = pair
            assert auto["outer_test_actual"] == reference["outer_test_actual"]
            comparisons.append(
                {
                    "dataset": dataset,
                    "model": model,
                    "shared_grid_trials_per_implementation": len(expected_grid),
                    "automl_selected_configuration": auto["selected_configuration"],
                    "sklearn_selected_configuration": reference["selected_configuration"],
                    "automl_validation_accuracy": auto["validation_accuracy"],
                    "sklearn_validation_accuracy": reference["validation_accuracy"],
                    "automl_outer_test_accuracy": auto["outer_test_accuracy"],
                    "sklearn_outer_test_accuracy": reference["outer_test_accuracy"],
                    "automl_outer_test_macro_f1": auto["outer_test_macro_f1"],
                    "sklearn_outer_test_macro_f1": reference["outer_test_macro_f1"],
                    "outer_test_label_agreement": sum(
                        a == b
                        for a, b in zip(
                            auto["outer_test_predictions"],
                            reference["outer_test_predictions"],
                        )
                    )
                    / len(auto["outer_test_predictions"]),
                }
            )

    result = {
        "status": "pass",
        "protocol_sha256": digest(protocol_path),
        "automl_manifest_sha256": digest(automl_manifest_path),
        "sklearn_manifest_sha256": digest(sklearn_manifest_path),
        "matched_cases": len(comparisons),
        "grid_trials_per_case_per_implementation": len(expected_grid),
        "trial_fits_per_implementation": len(comparisons) * len(expected_grid),
        "outer_test_used_for_selection": False,
        "inferential_tests_performed": False,
        "comparisons": comparisons,
    }
    verification_path = report / "matched-search-verification.json"
    verification_path.write_text(json.dumps(result, indent=2) + "\n")
    print(f"verified matched search protocol and wrote {verification_path}")


if __name__ == "__main__":
    main()
