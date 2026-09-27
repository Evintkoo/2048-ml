#!/usr/bin/env python3
"""Train the five policy classifiers on 20 games and evaluate five later games."""

import argparse
import csv
import hashlib
import json
import subprocess
from pathlib import Path


TRAIN_DATA = Path("reports/collection_pilots/2026-09-27-20-game/training.csv")
TRAIN_METADATA = Path(
    "reports/collection_pilots/2026-09-27-20-game/training.metadata.csv"
)
HOLDOUT_DATA = Path(
    "reports/collection_pilots/2026-09-27-5-game-repeat/training.csv"
)
HOLDOUT_METADATA = Path(
    "reports/collection_pilots/2026-09-27-5-game-repeat/training.metadata.csv"
)
MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
SEED = 90627


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_csv(path: Path):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        return reader.fieldnames, list(reader)


def source_revision(root: Path) -> str:
    return subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def prepare_joined_data(output: Path):
    train_header, train_rows = read_csv(TRAIN_DATA)
    holdout_header, holdout_rows = read_csv(HOLDOUT_DATA)
    if train_header != holdout_header:
        raise ValueError("training and holdout data schemas differ")

    train_meta_header, train_metadata = read_csv(TRAIN_METADATA)
    holdout_meta_header, holdout_metadata = read_csv(HOLDOUT_METADATA)
    if train_meta_header != holdout_meta_header:
        raise ValueError("training and holdout metadata schemas differ")
    if len(train_rows) != len(train_metadata) or len(holdout_rows) != len(holdout_metadata):
        raise ValueError("data and metadata row counts differ")

    for index, row in enumerate(train_metadata):
        if int(row["row_index"]) != index:
            raise ValueError("training metadata row indices are not contiguous")
        if not 0 <= int(row["game_id"]) < 20:
            raise ValueError("training game ID falls outside 0..19")
    for index, row in enumerate(holdout_metadata):
        if int(row["row_index"]) != index or not 0 <= int(row["game_id"]) < 5:
            raise ValueError("holdout metadata indices or game IDs are invalid")

    joined_data = output / "joined-training.csv"
    joined_metadata = output / "joined-training.metadata.csv"
    with joined_data.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=train_header, lineterminator="\n")
        writer.writeheader()
        writer.writerows(train_rows)
        writer.writerows(holdout_rows)
    with joined_metadata.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=train_meta_header, lineterminator="\n")
        writer.writeheader()
        for row in train_metadata:
            writer.writerow(row)
        offset = len(train_rows)
        for row in holdout_metadata:
            row = dict(row)
            row["row_index"] = str(offset + int(row["row_index"]))
            row["game_id"] = str(20 + int(row["game_id"]))
            writer.writerow(row)
    return joined_data, joined_metadata, train_rows, holdout_rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("reports/candidate_classifier_independent_holdout/2026-09-27-25-game"),
    )
    args = parser.parse_args()
    root = Path.cwd().resolve()
    output = (root / args.output_dir).resolve()
    if output.exists():
        raise SystemExit(f"refusing to overwrite {output}")
    output.mkdir(parents=True)

    joined_data, joined_metadata, train_rows, holdout_rows = prepare_joined_data(output)
    model_dir = output / "models"
    model_dir.mkdir()
    for model in MODELS:
        command = [
            "cargo",
            "run",
            "--quiet",
            "--",
            "train",
            "--data",
            str(joined_data),
            "--metadata",
            str(joined_metadata),
            "--model",
            model,
            "--cv-folds",
            "5",
            "--development-fraction",
            "0.8",
            "--seed",
            str(SEED),
            "--output",
            str(model_dir / f"{model}.policy.json"),
        ]
        print(f"training {model}", flush=True)
        subprocess.run(command, cwd=root, check=True)

    source_manifests = {
        "training": Path("reports/collection_pilots/2026-09-27-20-game/training.manifest.json"),
        "holdout": Path("reports/collection_pilots/2026-09-27-5-game-repeat/training.manifest.json"),
    }
    manifest = {
        "protocol": "fit on 20 earlier rollout games; reserve five later games as chronological classifier-label holdout",
        "interpretation": "small descriptive classifier diagnostic; not confirmatory policy quality or model selection",
        "source_revision": source_revision(root),
        "automl_commit": "82d848323eed5e2af86d046d529916c448f2442c",
        "global_training_seed": SEED,
        "candidate_models": list(MODELS),
        "dataset_schema": "2048-action-policy-v2",
        "state_feature_count": 17,
        "cv_strategy": "GroupKFold",
        "cv_folds": 5,
        "development_fraction": 0.8,
        "training_source": {
            "manifest": str(source_manifests["training"]),
            "manifest_sha256": sha256(root / source_manifests["training"]),
            "game_seeds": [90627, 90646],
            "games": 20,
            "rows": len(train_rows),
        },
        "holdout_source": {
            "manifest": str(source_manifests["holdout"]),
            "manifest_sha256": sha256(root / source_manifests["holdout"]),
            "game_seeds": [90647, 90651],
            "games": 5,
            "rows": len(holdout_rows),
        },
        "combined_data": {
            "training_csv": str(joined_data.relative_to(root)),
            "training_csv_sha256": sha256(joined_data),
            "metadata_csv": str(joined_metadata.relative_to(root)),
            "metadata_csv_sha256": sha256(joined_metadata),
            "total_games": 25,
            "total_rows": len(train_rows) + len(holdout_rows),
            "holdout_game_ids": [20, 21, 22, 23, 24],
        },
        "limitations": [
            "The five held-out game groups are a small sample and do not support policy-quality claims",
            "Each candidate is fitted once; no training-seed uncertainty is estimated",
            "These rollout-derived action-label metrics are not held-out game-score metrics",
            "The five holdout games were collected with the same host and simulator protocol as training data",
        ],
    }
    manifest_path = output / "protocol-manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"protocol manifest: {manifest_path}")


if __name__ == "__main__":
    main()
