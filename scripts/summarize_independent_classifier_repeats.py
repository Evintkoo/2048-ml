#!/usr/bin/env python3
"""Summarize repeated classifier fits on the fixed 20-game data/5-game holdout."""

import argparse
import csv
import json
import statistics
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repeats-dir",
        type=Path,
        default=Path(
            "reports/candidate_classifier_independent_holdout/2026-09-27-25-game/repeated-fit"
        ),
    )
    args = parser.parse_args()
    repeats_dir = Path.cwd() / args.repeats_dir
    run_dirs = sorted(path for path in repeats_dir.glob("seed-*") if path.is_dir())
    if len(run_dirs) < 2:
        raise SystemExit("at least two seed-* repeat directories are required")

    runs = []
    for run_dir in run_dirs:
        protocol = json.loads((run_dir / "protocol-manifest.json").read_text())
        verification = json.loads((run_dir / "verification.json").read_text())
        if verification["status"] != "pass":
            raise SystemExit(f"verification failed for {run_dir}")
        runs.append((protocol, verification))

    base = runs[0][0]
    seeds = []
    for protocol, _ in runs:
        seeds.append(protocol["global_training_seed"])
        for key in ("training_csv_sha256", "metadata_csv_sha256"):
            if protocol["combined_data"][key] != base["combined_data"][key]:
                raise SystemExit(f"combined input differs across repeats: {key}")
        if protocol["holdout_source"]["manifest_sha256"] != base["holdout_source"]["manifest_sha256"]:
            raise SystemExit("holdout source differs across repeats")
        if protocol["training_source"]["manifest_sha256"] != base["training_source"]["manifest_sha256"]:
            raise SystemExit("training source differs across repeats")
        if protocol["source_revision"] != base["source_revision"]:
            raise SystemExit("root source revision differs across repeats")
    if len(set(seeds)) != len(seeds):
        raise SystemExit("training seeds must be unique")

    models = sorted(runs[0][1]["models"])
    rows = []
    summary = {
        "status": "pass",
        "protocol": "training-seed repeats on one fixed 20-game training corpus and one fixed five-game holdout",
        "interpretation": "descriptive fit-seed sensitivity; does not estimate corpus, host, or holdout-game uncertainty",
        "source_revision": base["source_revision"],
        "training_seeds": seeds,
        "n_repeats": len(seeds),
        "training_input_sha256": base["combined_data"]["training_csv_sha256"],
        "metadata_input_sha256": base["combined_data"]["metadata_csv_sha256"],
        "holdout_manifest_sha256": base["holdout_source"]["manifest_sha256"],
        "models": {},
    }
    for model in models:
        accuracy = []
        macro_f1 = []
        for seed, (_, verification) in zip(seeds, runs):
            result = verification["models"][model]
            accuracy.append(result["accuracy"])
            macro_f1.append(result["macro_f1"])
            rows.append(
                {
                    "model": model,
                    "training_seed": seed,
                    "holdout_accuracy": f"{result['accuracy']:.12f}",
                    "holdout_macro_f1": f"{result['macro_f1']:.12f}",
                }
            )
        summary["models"][model] = {
            "accuracy_mean": statistics.mean(accuracy),
            "accuracy_sample_sd": statistics.stdev(accuracy),
            "accuracy_min": min(accuracy),
            "accuracy_max": max(accuracy),
            "macro_f1_mean": statistics.mean(macro_f1),
            "macro_f1_sample_sd": statistics.stdev(macro_f1),
            "macro_f1_min": min(macro_f1),
            "macro_f1_max": max(macro_f1),
        }

    csv_path = repeats_dir / "repeated-fit-runs.csv"
    with csv_path.open("w", newline="") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=["model", "training_seed", "holdout_accuracy", "holdout_macro_f1"],
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)
    summary_path = repeats_dir / "repeated-fit-summary.json"
    summary_path.write_text(json.dumps(summary, indent=2) + "\n")
    print(f"wrote {csv_path} and {summary_path}")


if __name__ == "__main__":
    main()
