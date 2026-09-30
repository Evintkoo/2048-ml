#!/usr/bin/env python3
"""Verify retained one-process-per-case fit-phase RSS artifacts."""

import argparse
import csv
import hashlib
import json
from pathlib import Path

DATASETS = ("iris", "wine", "breast_cancer_wisconsin_diagnostic")
MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--report-dir",
        type=Path,
        default=Path("reports/framework_validation/model-memory-matrix-2026-09-30"),
    )
    parser.add_argument("--data-dir", type=Path, default=Path("data/framework_validation"))
    args = parser.parse_args()
    report = args.report_dir.resolve()
    data_dir = args.data_dir.resolve()
    repo_root = report.parents[2]
    manifest_path = report / "fit-memory-manifest.json"
    summary_path = report / "fit-memory-summary.csv"
    manifest = json.loads(manifest_path.read_text())
    if manifest["schema"] != "2048-ml.framework-fit-memory-profile":
        raise ValueError("unexpected fit-memory schema")
    if manifest["datasets"] != list(DATASETS) or manifest["models"] != list(MODELS):
        raise ValueError("dataset or model matrix differs from the declared protocol")
    if manifest["process_boundary"] != "one fresh process per dataset/model case; one model fit per process":
        raise ValueError("profiles are not isolated per dataset/model process")
    if sha256(summary_path) != manifest["summary_sha256"]:
        raise ValueError("summary CSV digest differs from the run manifest")

    summary_rows = list(csv.DictReader(summary_path.open(newline="")))
    if len(summary_rows) != len(DATASETS) * len(MODELS) or len(manifest["runs"]) != len(summary_rows):
        raise ValueError("expected exactly 15 dataset/model records")
    expected = {(dataset, model) for dataset in DATASETS for model in MODELS}
    seen = set()
    for row, recorded in zip(summary_rows, manifest["runs"]):
        key = (row["dataset"], row["model"])
        if key not in expected or key in seen:
            raise ValueError(f"unexpected or duplicate case {key}")
        seen.add(key)
        for field in row:
            if row[field] != str(recorded[field]):
                raise ValueError(f"summary/manifest mismatch for {key} field {field}")
        case_dir = repo_root / row["case_dir"]
        result_path = case_dir / "framework-validation-results.json"
        run_manifest_path = case_dir / "framework-validation-manifest.json"
        split_path = case_dir / f"{row['dataset']}.split.json"
        for path, field in (
            (result_path, "result_sha256"),
            (run_manifest_path, "manifest_sha256"),
            (split_path, "split_sha256"),
        ):
            if sha256(path) != row[field]:
                raise ValueError(f"digest mismatch for {path}")
        result_records = json.loads(result_path.read_text())
        if len(result_records) != 1 or result_records[0]["status"] != "success":
            raise ValueError(f"expected one successful case in {result_path}")
        result = result_records[0]
        split = json.loads(split_path.read_text())
        source_path = data_dir / split["source_file"]
        if sha256(source_path) != split["source_file_sha256"]:
            raise ValueError(f"source dataset digest mismatch: {source_path}")
        if split["dataset"] != row["dataset"] or result["model"] != row["model"]:
            raise ValueError(f"dataset/model identity mismatch in {case_dir}")
        if result["seed"] != manifest["seed"]:
            raise ValueError(f"seed mismatch in {case_dir}")
        baseline = int(row["baseline_rss_bytes"])
        peak = int(row["peak_fit_rss_bytes"])
        increment = int(row["peak_incremental_fit_rss_bytes"])
        if peak < baseline or increment != peak - baseline:
            raise ValueError(f"invalid RSS arithmetic in {case_dir}")
        if result["fit_baseline_rss_bytes"] != baseline:
            raise ValueError(f"result baseline differs in {case_dir}")
        if result["fit_peak_rss_bytes"] != peak:
            raise ValueError(f"result peak differs in {case_dir}")
        if result["fit_peak_incremental_rss_bytes"] != increment:
            raise ValueError(f"result increment differs in {case_dir}")

    if seen != expected:
        raise ValueError(f"missing cases: {sorted(expected - seen)}")
    print(f"verified {len(seen)} isolated fit-phase RSS cases and all retained hashes")


if __name__ == "__main__":
    main()
