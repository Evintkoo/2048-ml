#!/usr/bin/env python3
"""Compare the deterministic content of two retained HyperOpt smoke runs."""

import argparse
import hashlib
import json
from pathlib import Path


def read(path: Path):
    return json.loads(path.read_text())


def digest(path: Path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def without_timing(model, study):
    model = json.loads(json.dumps(model))
    study = json.loads(json.dumps(study))
    model.get("metrics", {}).pop("training_time_secs", None)
    study.pop("total_duration_secs", None)
    for trial in study.get("trials", []):
        trial.pop("duration_secs", None)
    return model, study


def compare(directory: Path):
    first_stem = "policy.model"
    second_stem = "policy-repeat.model"
    first_manifest = read(directory / f"{first_stem}.manifest.json")
    second_manifest = read(directory / f"{second_stem}.manifest.json")
    stable_manifest_fields = (
        "automl_commit",
        "global_seed",
        "training_data_sha256",
        "metadata_sha256",
        "selected_n_estimators",
        "selected_max_depth",
        "hyperparameter_optimization",
    )
    for field in stable_manifest_fields:
        if first_manifest[field] != second_manifest[field]:
            raise ValueError(f"manifest field differs between runs: {field}")

    first_model = read(directory / f"{first_stem}.json")
    second_model = read(directory / f"{second_stem}.json")
    first_study = read(directory / f"{first_stem}.study.json")
    second_study = read(directory / f"{second_stem}.study.json")
    first_model_stable, first_study_stable = without_timing(first_model, first_study)
    second_model_stable, second_study_stable = without_timing(second_model, second_study)
    if first_model_stable != second_model_stable:
        raise ValueError("model artifacts differ beyond the recorded elapsed-training-time field")
    if first_study_stable != second_study_stable:
        raise ValueError("study trial parameters/results differ beyond measured durations")
    fixture_manifest = read(directory / "fixture-manifest.json")
    generator = Path(__file__).resolve().parents[1] / "scripts/create_hyperopt_smoke_data.py"
    if fixture_manifest["runner_sha256"] != digest(generator):
        raise ValueError("synthetic fixture generator digest differs from retained manifest")

    return {
        "schema": "2048-ml.hyperopt-smoke-repeat-comparison",
        "schema_version": 1,
        "scope": "synthetic tuning-wiring smoke only; no model-quality inference",
        "automl_commit": first_manifest["automl_commit"],
        "training_data_sha256": first_manifest["training_data_sha256"],
        "metadata_sha256": first_manifest["metadata_sha256"],
        "hyperopt_config_sha256": first_manifest["hyperparameter_optimization"]["input_config_sha256"],
        "trial_values": [trial["value"] for trial in first_study["trials"]],
        "selected_parameters": first_manifest["hyperparameter_optimization"]["selected_parameters"],
        "run_1_model_sha256": first_manifest["model_artifact_sha256"],
        "run_2_model_sha256": second_manifest["model_artifact_sha256"],
        "semantic_model_equal_ignoring_training_time": True,
        "semantic_study_equal_ignoring_trial_durations": True,
        "fixture_generator_sha256": fixture_manifest["runner_sha256"],
        "limitations": [
            "Elapsed training and trial durations are machine-dependent and differ between runs",
            "The fixture reuses source rows across synthetic game IDs",
            "The run does not establish model quality, tuning efficacy, or broad reproducibility",
        ],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    report = compare(args.directory)
    output = args.directory / "repeatability-comparison.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"semantic model/study matches verified; report={output}")


if __name__ == "__main__":
    main()
