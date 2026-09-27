#!/usr/bin/env python3
"""Summarize verified matched-grid runs across split seeds."""

import argparse
import csv
import hashlib
import json
from collections import defaultdict
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--run-dir",
        action="append",
        type=Path,
        required=True,
        help="verified run directory; pass once per split seed",
    )
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()

    rows = []
    root = Path.cwd().resolve()
    for run_dir in args.run_dir:
        run = (root / run_dir).resolve()
        protocol = json.loads((run / "matched-search-protocol.json").read_text())
        verification = json.loads((run / "matched-search-verification.json").read_text())
        assert verification["status"] == "pass", f"unverified run: {run}"
        for comparison in verification["comparisons"]:
            auto = comparison["automl_selected_configuration"]
            sklearn = comparison["sklearn_selected_configuration"]
            rows.append(
                {
                    "seed": protocol["seed"],
                    "dataset": comparison["dataset"],
                    "model": comparison["model"],
                    "automl_n_estimators": auto["n_estimators"],
                    "automl_max_depth": auto["max_depth"],
                    "sklearn_n_estimators": sklearn["n_estimators"],
                    "sklearn_max_depth": sklearn["max_depth"],
                    "selected_config_match": auto == sklearn,
                    "automl_validation_accuracy": comparison["automl_validation_accuracy"],
                    "sklearn_validation_accuracy": comparison["sklearn_validation_accuracy"],
                    "automl_outer_test_accuracy": comparison["automl_outer_test_accuracy"],
                    "sklearn_outer_test_accuracy": comparison["sklearn_outer_test_accuracy"],
                    "automl_outer_test_macro_f1": comparison["automl_outer_test_macro_f1"],
                    "sklearn_outer_test_macro_f1": comparison["sklearn_outer_test_macro_f1"],
                    "outer_test_label_agreement": comparison["outer_test_label_agreement"],
                    "verification_sha256": hashlib.sha256(
                        (run / "matched-search-verification.json").read_bytes()
                    ).hexdigest(),
                }
            )

    assert rows, "no verified cases found"
    keys = {(r["dataset"], r["model"]) for r in rows}
    seeds = sorted({r["seed"] for r in rows})
    assert len(rows) == len(keys) * len(seeds), "duplicate or missing seed/case entries"
    out = (root / args.output_dir).resolve()
    out.mkdir(parents=True, exist_ok=True)
    with (out / "matched-grid-multi-seed-cases.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(sorted(rows, key=lambda r: (r["seed"], r["dataset"], r["model"])))

    grouped = defaultdict(list)
    for row in rows:
        grouped[(row["dataset"], row["model"])].append(row)
    summary = {
        "split_seeds": seeds,
        "verified_run_count": len(seeds),
        "dataset_model_cases": len(keys),
        "matched_case_seed_observations": len(rows),
        "candidate_fits_per_implementation": len(rows) * 6,
        "selected_configuration_matches": sum(r["selected_config_match"] for r in rows),
        "selected_configuration_observations": len(rows),
        "inferential_tests_performed": False,
        "cases": [],
    }
    for (dataset, model), cases in sorted(grouped.items()):
        summary["cases"].append(
            {
                "dataset": dataset,
                "model": model,
                "seed_results": [
                    {
                        "seed": c["seed"],
                        "automl_configuration": {
                            "n_estimators": c["automl_n_estimators"],
                            "max_depth": c["automl_max_depth"],
                        },
                        "sklearn_configuration": {
                            "n_estimators": c["sklearn_n_estimators"],
                            "max_depth": c["sklearn_max_depth"],
                        },
                        "selected_config_match": c["selected_config_match"],
                        "automl_outer_test_accuracy": c["automl_outer_test_accuracy"],
                        "sklearn_outer_test_accuracy": c["sklearn_outer_test_accuracy"],
                        "outer_test_label_agreement": c["outer_test_label_agreement"],
                    }
                    for c in sorted(cases, key=lambda r: r["seed"])
                ],
            }
        )
    (out / "matched-grid-multi-seed-summary.json").write_text(
        json.dumps(summary, indent=2) + "\n"
    )
    print(
        f"summarized {len(rows)} verified case-seed observations across "
        f"{len(seeds)} split seeds; selected configurations matched "
        f"{summary['selected_configuration_matches']}/{len(rows)}"
    )


if __name__ == "__main__":
    main()
