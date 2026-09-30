#!/usr/bin/env python3
"""Run one fit-phase RSS profile per AutoML model in a fresh process."""

import argparse
import csv
import hashlib
import json
import platform
import subprocess
from pathlib import Path
from typing import Optional

MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
DATASETS = ("iris", "wine", "breast_cancer_wisconsin_diagnostic")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git_value(root: Path, *args: str) -> Optional[str]:
    result = subprocess.run(["git", *args], cwd=root, text=True, capture_output=True)
    return result.stdout.strip() if result.returncode == 0 else None


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--data-dir", type=Path, default=Path("data/framework_validation"))
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("reports/framework_validation/model-memory-isolated-2026-09-30"),
    )
    parser.add_argument("--seed", type=int, default=42)
    args = parser.parse_args()

    root = Path.cwd().resolve()
    binary = root / "target/debug/game2048-ml"
    data_dir = args.data_dir.resolve()
    output_dir = args.output_dir.resolve()
    if not binary.is_file():
        raise SystemExit("build the root binary first with `cargo build`")
    if output_dir.exists():
        raise SystemExit(f"refusing to overwrite {output_dir}")
    output_dir.mkdir(parents=True)

    rows = []
    for dataset in DATASETS:
        for model in MODELS:
            case_dir = output_dir / dataset / model
            command = [
                str(binary),
                "framework-validate",
                "--data-dir",
                str(data_dir),
                "--output-dir",
                str(case_dir),
                "--dataset",
                dataset,
                "--model",
                model,
                "--seed",
                str(args.seed),
                "--test-fraction",
                "0.2",
                "--profile-fit-memory",
            ]
            completed = subprocess.run(command, cwd=root, capture_output=True, text=True)
            case_dir.mkdir(parents=True, exist_ok=True)
            (case_dir / "runner.log").write_text(completed.stdout + completed.stderr)
            if completed.returncode:
                raise RuntimeError(f"{dataset}/{model} profile failed; see {case_dir / 'runner.log'}")

            result_path = case_dir / "framework-validation-results.json"
            manifest_path = case_dir / "framework-validation-manifest.json"
            split_path = case_dir / f"{dataset}.split.json"
            records = json.loads(result_path.read_text())
            if len(records) != 1 or records[0]["status"] != "success":
                raise RuntimeError(f"expected one successful result in {result_path}")
            record = records[0]
            memory_fields = (
                "fit_baseline_rss_bytes",
                "fit_peak_rss_bytes",
                "fit_peak_incremental_rss_bytes",
            )
            if any(record.get(field) is None for field in memory_fields):
                raise RuntimeError(f"memory measurements missing for {dataset}/{model}")
            rows.append(
                {
                    "dataset": dataset,
                    "model": model,
                    "seed": args.seed,
                    "train_rows": record["train_rows"],
                    "test_rows": record["test_rows"],
                    "accuracy": record["accuracy"],
                    "macro_f1": record["macro_f1"],
                    "baseline_rss_bytes": record["fit_baseline_rss_bytes"],
                    "peak_fit_rss_bytes": record["fit_peak_rss_bytes"],
                    "peak_incremental_fit_rss_bytes": record["fit_peak_incremental_rss_bytes"],
                    "result_sha256": sha256(result_path),
                    "manifest_sha256": sha256(manifest_path),
                    "split_sha256": sha256(split_path),
                    "case_dir": str(case_dir.relative_to(root)),
                }
            )
            print(f"completed isolated fit profile: {dataset}/{model}", flush=True)

    summary_path = output_dir / "fit-memory-summary.csv"
    with summary_path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    manifest = {
        "schema": "2048-ml.framework-fit-memory-profile",
        "schema_version": 1,
        "datasets": list(DATASETS),
        "dataset_source_sha256": json.loads(
            (output_dir / DATASETS[0] / MODELS[0] / f"{DATASETS[0]}.split.json").read_text()
        )["source_file_sha256"],
        "seed": args.seed,
        "test_fraction": 0.2,
        "models": list(MODELS),
        "process_boundary": "one fresh process per dataset/model case; one model fit per process",
        "memory_metric": "peak process RSS sampled every 5 ms during fit minus RSS immediately before fit",
        "memory_limits": [
            "Incremental fit-phase RSS includes AutoML fit temporaries and allocator retention",
            "It does not isolate the persistent model object's allocation",
            "5 ms polling may miss shorter peaks; use as a sampled lower bound",
            "One host, one seed, and one process per model are descriptive only",
        ],
        "root_revision": git_value(root, "rev-parse", "HEAD"),
        "root_worktree_dirty": subprocess.run(
            ["git", "diff", "--quiet"], cwd=root
        ).returncode != 0,
        "automl_revision": git_value(root, "-C", "automl", "rev-parse", "HEAD"),
        "automl_worktree_dirty": subprocess.run(
            ["git", "-C", "automl", "diff", "--quiet"], cwd=root
        ).returncode != 0,
        "cargo_lock_sha256": sha256(root / "Cargo.lock"),
        "runner_script_sha256": sha256(root / "scripts/run_framework_fit_memory_profiles.py"),
        "rustc_version": subprocess.run(
            ["rustc", "--version"], cwd=root, text=True, capture_output=True, check=True
        ).stdout.strip(),
        "host": {"platform": platform.platform(), "architecture": platform.machine()},
        "summary_file": summary_path.name,
        "summary_sha256": sha256(summary_path),
        "runs": rows,
    }
    manifest_path = output_dir / "fit-memory-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"wrote fit-phase memory summary: {output_dir}")


if __name__ == "__main__":
    main()
