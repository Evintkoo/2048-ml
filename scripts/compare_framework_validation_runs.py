#!/usr/bin/env python3
"""Compare repeated fixed-split framework validation artifacts."""

import argparse
import hashlib
import json
from pathlib import Path


def read_json(path: Path):
    return json.loads(path.read_text())


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compare(first_dir: Path, second_dir: Path):
    first_manifest = read_json(first_dir / "framework-validation-manifest.json")
    second_manifest = read_json(second_dir / "framework-validation-manifest.json")
    if first_manifest["seed"] != second_manifest["seed"]:
        raise ValueError("global seeds differ")

    first_splits = {
        entry["dataset"]: read_json(first_dir / f"{entry['dataset']}.split.json")
        for entry in first_manifest["dataset_splits"]
    }
    second_splits = {
        entry["dataset"]: read_json(second_dir / f"{entry['dataset']}.split.json")
        for entry in second_manifest["dataset_splits"]
    }
    split_equal = {
        dataset: first_splits[dataset]["test_source_rows"]
        == second_splits[dataset]["test_source_rows"]
        and first_splits[dataset]["train_source_rows"]
        == second_splits[dataset]["train_source_rows"]
        for dataset in first_splits
    }
    if first_splits.keys() != second_splits.keys() or not all(split_equal.values()):
        raise ValueError("dataset split assignments differ")

    first_records = read_json(first_dir / "framework-validation-results.json")
    second_records = read_json(second_dir / "framework-validation-results.json")
    second_by_key = {(row["dataset"], row["model"]): row for row in second_records}
    comparisons = []
    for first in first_records:
        key = (first["dataset"], first["model"])
        second = second_by_key[key]
        first_predictions = Path(first["predictions_csv"])
        second_predictions = Path(second["predictions_csv"])
        predictions_equal = first_predictions.read_bytes() == second_predictions.read_bytes()
        comparisons.append(
            {
                "dataset": first["dataset"],
                "model": first["model"],
                "first_status": first["status"],
                "second_status": second["status"],
                "predictions_equal": predictions_equal,
                "first_accuracy": first["accuracy"],
                "second_accuracy": second["accuracy"],
                "first_macro_f1": first["macro_f1"],
                "second_macro_f1": second["macro_f1"],
                "first_save_load_equivalent": first["save_load_equivalent"],
                "second_save_load_equivalent": second["save_load_equivalent"],
                "first_predictions_sha256": sha256(first_predictions),
                "second_predictions_sha256": sha256(second_predictions),
            }
        )

    return {
        "protocol": "same seed and fixed stratified outer split; independent process runs",
        "seed": first_manifest["seed"],
        "split_assignments_equal": split_equal,
        "prediction_comparisons": comparisons,
        "identical_prediction_pairs": sum(row["predictions_equal"] for row in comparisons),
        "total_pairs": len(comparisons),
        "limitations": [
            "Two runs only",
            "Exact equality is a diagnostic, not a broad determinism proof",
            "No external framework baseline or memory profile",
        ],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("first", type=Path)
    parser.add_argument("second", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    report = compare(args.first, args.second)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(
        f"identical predictions: {report['identical_prediction_pairs']}/"
        f"{report['total_pairs']}; report={args.output}"
    )


if __name__ == "__main__":
    main()
