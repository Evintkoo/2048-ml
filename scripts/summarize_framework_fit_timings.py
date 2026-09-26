#!/usr/bin/env python3
"""Summarize per-case fit/predict timings from repeated AutoML diagnostics."""

import argparse
import csv
import json
from pathlib import Path
from statistics import median


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reports-dir", type=Path, default=Path("reports/framework_validation"))
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    runs = [
        args.reports_dir / "pinned-82d8483-run-1/framework-validation-results.json",
        args.reports_dir / "pinned-82d8483-run-2/framework-validation-results.json",
    ]
    by_case: dict[tuple[str, str], list[float]] = {}
    for run in runs:
        for record in json.loads(run.read_text()):
            if record["status"] == "success" and record["fit_predict_seconds"] is not None:
                key = (record["dataset"], record["model"])
                by_case.setdefault(key, []).append(record["fit_predict_seconds"])

    output = args.output or args.reports_dir / "pinned-82d8483-per-case-timings.csv"
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", newline="") as stream:
        writer = csv.writer(stream)
        writer.writerow(
            [
                "dataset",
                "model",
                "repeats",
                "run1_fit_predict_seconds",
                "run2_fit_predict_seconds",
                "median_fit_predict_seconds",
            ]
        )
        for (dataset, model), timings in sorted(by_case.items()):
            if len(timings) != 2:
                raise ValueError(f"expected two seed-42 observations for {dataset}/{model}: {len(timings)}")
            writer.writerow(
                [dataset, model, len(timings), f"{timings[0]:.9f}", f"{timings[1]:.9f}", f"{median(timings):.9f}"]
            )
    if len(by_case) != 15:
        raise ValueError(f"expected 15 repeated dataset/model cases; found {len(by_case)}")
    print(f"wrote {len(by_case)} cases to {output}")


if __name__ == "__main__":
    main()
