#!/usr/bin/env python3
"""Measure isolated fixed-split framework case-process RSS and wall time on macOS."""

import argparse
import csv
import hashlib
import json
import platform
import re
import subprocess
import sys
from pathlib import Path

DATASETS = ("iris", "wine", "breast_cancer_wisconsin_diagnostic")
MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
SEED = 42
TIME_BIN = Path("/usr/bin/time")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def revision() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"], check=True, capture_output=True, text=True
    )
    return result.stdout.strip()


def select(values, selected, label):
    if selected is None:
        return values
    if selected not in values:
        raise ValueError(f"unknown {label} {selected!r}; choose from {', '.join(values)}")
    return (selected,)


def run_case(root, implementation, dataset, model, repeat, args):
    relative_output = Path(implementation) / f"{dataset}__{model}__run-{repeat}"
    output_dir = args.output_dir / relative_output
    if output_dir.exists():
        raise FileExistsError(f"refusing to overwrite {output_dir}")
    output_dir.mkdir(parents=True)

    if implementation == "automl":
        command = [
            str(root / "target/debug/game2048-ml"),
            "framework-validate",
            "--data-dir",
            str(args.data_dir),
            "--output-dir",
            str(output_dir),
            "--model",
            model,
            "--dataset",
            dataset,
            "--seed",
            str(SEED),
            "--test-fraction",
            "0.2",
        ]
        process_command = [str(TIME_BIN), "-l", "env", "RAYON_NUM_THREADS=1", *command]
    else:
        command = [
            sys.executable,
            str(root / "scripts/run_sklearn_framework_baseline.py"),
            "--data-dir",
            str(args.data_dir),
            "--split-dir",
            str(args.sklearn_split_dir),
            "--output-dir",
            str(output_dir),
            "--dataset",
            dataset,
            "--model",
            model,
        ]
        process_command = [str(TIME_BIN), "-l", *command]

    result = subprocess.run(process_command, cwd=root, capture_output=True, text=True)
    log_path = output_dir / "resource-log.txt"
    log_path.write_text(result.stdout + result.stderr)
    if result.returncode:
        raise RuntimeError(f"case failed ({implementation}/{dataset}/{model}): {log_path}")

    text = log_path.read_text()
    rss_match = re.search(r"([0-9,]+)\s+maximum resident set size", text, re.I)
    wall_match = re.search(r"([0-9]+(?:\.[0-9]+)?)\s+real\b", text)
    if not rss_match or not wall_match:
        raise ValueError(f"could not parse /usr/bin/time output: {log_path}")
    rss_bytes = int(rss_match.group(1).replace(",", ""))
    wall_seconds = float(wall_match.group(1))

    if implementation == "automl":
        manifest_path = output_dir / "framework-validation-manifest.json"
        results_path = output_dir / "framework-validation-results.json"
        manifest = json.loads(manifest_path.read_text())
    else:
        manifest_path = output_dir / "sklearn-framework-validation-manifest.json"
        results_path = output_dir / "sklearn-framework-validation-results.json"
        manifest = json.loads(manifest_path.read_text())
    records = json.loads(results_path.read_text())
    if len(records) != 1 or records[0]["status"] != "success":
        raise ValueError(f"expected exactly one successful case: {results_path}")
    try:
        output_path = str(output_dir.relative_to(root))
    except ValueError:
        output_path = str(output_dir)

    return {
        "implementation": implementation,
        "dataset": dataset,
        "model": model,
        "repeat": repeat,
        "peak_rss_bytes": rss_bytes,
        "wall_seconds": wall_seconds,
        "output_dir": output_path,
        "manifest_sha256": digest(manifest_path),
        "results_sha256": digest(results_path),
        "manifest_source_revision": manifest.get("source_revision"),
        "manifest_dirty": manifest.get("root_worktree_dirty"),
    }


def write_csv(path, fieldnames, rows):
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fieldnames, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--data-dir", type=Path, default=Path("data/framework_validation"))
    parser.add_argument(
        "--sklearn-split-dir",
        type=Path,
        default=Path("reports/framework_validation/pinned-82d8483-run-1"),
    )
    parser.add_argument(
        "--output-dir", type=Path, default=Path("reports/framework_validation/case-resource-matrix")
    )
    parser.add_argument("--dataset", choices=DATASETS)
    parser.add_argument("--model", choices=MODELS)
    parser.add_argument("--repeats", type=int, default=2)
    args = parser.parse_args()
    if sys.platform != "darwin" or not TIME_BIN.is_file():
        raise SystemExit("this resource runner requires macOS /usr/bin/time -l")
    if args.repeats < 1:
        raise SystemExit("--repeats must be positive")
    root = Path.cwd().resolve()
    args.data_dir = args.data_dir.resolve()
    args.sklearn_split_dir = args.sklearn_split_dir.resolve()
    args.output_dir = args.output_dir.resolve()
    if args.output_dir.exists():
        raise SystemExit(f"refusing to overwrite output directory {args.output_dir}")
    if not (root / "target/debug/game2048-ml").is_file():
        raise SystemExit("build the root binary first with `cargo build`")

    datasets = select(DATASETS, args.dataset, "dataset")
    models = select(MODELS, args.model, "model")
    args.output_dir.mkdir(parents=True)
    runs = []
    for implementation in ("automl", "sklearn"):
        for dataset in datasets:
            for model in models:
                for repeat in range(1, args.repeats + 1):
                    print(f"running {implementation} {dataset}/{model} repeat {repeat}", flush=True)
                    runs.append(run_case(root, implementation, dataset, model, repeat, args))

    output_sha = args.output_dir / "case-resource-runs.csv"
    run_fields = list(runs[0])
    write_csv(output_sha, run_fields, runs)
    summary = []
    for implementation in ("automl", "sklearn"):
        for dataset in datasets:
            for model in models:
                matching = [
                    row for row in runs
                    if row["implementation"] == implementation
                    and row["dataset"] == dataset
                    and row["model"] == model
                ]
                rss_values = sorted(row["peak_rss_bytes"] for row in matching)
                middle = len(rss_values) // 2
                median_rss = (
                    rss_values[middle]
                    if len(rss_values) % 2
                    else (rss_values[middle - 1] + rss_values[middle]) / 2
                )
                wall_values = sorted(row["wall_seconds"] for row in matching)
                median_wall = (
                    wall_values[middle]
                    if len(wall_values) % 2
                    else (wall_values[middle - 1] + wall_values[middle]) / 2
                )
                summary.append(
                    {
                        "implementation": implementation,
                        "dataset": dataset,
                        "model": model,
                        "repeats": len(matching),
                        "median_peak_rss_bytes": int(median_rss),
                        "min_peak_rss_bytes": min(rss_values),
                        "max_peak_rss_bytes": max(rss_values),
                        "median_process_wall_seconds": f"{median_wall:.6f}",
                    }
                )
    summary_path = args.output_dir / "case-resource-summary.csv"
    write_csv(summary_path, list(summary[0]), summary)
    manifest = {
        "protocol": f"one selected dataset/model per process; /usr/bin/time -l; fixed seed-42 split; {args.repeats} repeat(s)",
        "seed": SEED,
        "threading": {"automl": "RAYON_NUM_THREADS=1", "sklearn": "threadpoolctl limit=1 and n_jobs=1"},
        "configuration": {"n_estimators": 32, "max_depth": 8, "test_fraction": 0.2},
        "selected_datasets": list(datasets),
        "selected_models": list(models),
        "repeats": args.repeats,
        "source_revision": revision(),
        "root_worktree_dirty": bool(
            subprocess.run(["git", "diff", "--quiet"], cwd=root).returncode
        ),
        "host": {"os": platform.platform(), "architecture": platform.machine()},
        "runs_csv": output_sha.name,
        "runs_csv_sha256": digest(output_sha),
        "summary_csv": summary_path.name,
        "summary_csv_sha256": digest(summary_path),
        "interpretation_limitations": [
            "Peak RSS is process-level and includes runtime/library startup and dataset handling, not model-only memory",
            "Two repeats on one host are descriptive and do not estimate hardware variation",
            "Fixed settings do not compare optimizer search budgets or establish framework superiority",
            "AutoML and scikit-learn model implementations and some defaults differ",
        ],
    }
    manifest_path = args.output_dir / "case-resource-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"resource artifacts written to {args.output_dir}")


if __name__ == "__main__":
    main()
