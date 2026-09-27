#!/usr/bin/env python3
"""Fit all five AutoML candidates at five seeds on each retained 50-game corpus."""

import argparse
import hashlib
import json
import subprocess
from pathlib import Path


MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
CORPORA = {
    "previous": {
        "data": Path("reports/collection_pilots/2026-09-27-50-game-followup/training.csv"),
        "metadata": Path("reports/collection_pilots/2026-09-27-50-game-followup/training.metadata.csv"),
        "first_seed": 90652,
    },
    "independent": {
        "data": Path("reports/collection_pilots/2026-09-27-50-game-independent/training.csv"),
        "metadata": Path("reports/collection_pilots/2026-09-27-50-game-independent/training.metadata.csv"),
        "first_seed": 90702,
    },
}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("reports/candidate_classifier_pilot/matched-50-game-corpus-fit-seeds-2026-09-27"),
    )
    parser.add_argument("--replicates", type=int, default=5)
    parser.add_argument("--cv-folds", type=int, default=5)
    parser.add_argument("--development-fraction", type=float, default=0.8)
    args = parser.parse_args()
    if args.output_dir.exists():
        raise SystemExit(f"refusing to overwrite {args.output_dir}")
    if args.replicates < 2:
        raise SystemExit("at least two fit seeds are required")
    root = Path.cwd().resolve()
    output_dir = (root / args.output_dir).resolve()
    output_dir.mkdir(parents=True)
    records = []
    for corpus, config in CORPORA.items():
        for replicate in range(args.replicates):
            seed = config["first_seed"] + replicate
            seed_dir = output_dir / corpus / f"seed-{seed}"
            model_dir = seed_dir / "models"
            model_dir.mkdir(parents=True)
            for model in MODELS:
                model_path = model_dir / f"{model}.policy.json"
                command = [
                    "cargo",
                    "run",
                    "--quiet",
                    "--",
                    "train",
                    "--data",
                    str(config["data"]),
                    "--metadata",
                    str(config["metadata"]),
                    "--model",
                    model,
                    "--cv-folds",
                    str(args.cv_folds),
                    "--development-fraction",
                    str(args.development_fraction),
                    "--seed",
                    str(seed),
                    "--output",
                    str(model_path.relative_to(root)),
                ]
                subprocess.run(command, cwd=root, check=True, timeout=180)
                manifest_path = model_path.with_suffix(".manifest.json")
                manifest = json.loads(manifest_path.read_text())
                records.append(
                    {
                        "corpus": corpus,
                        "seed": seed,
                        "replicate": replicate,
                        "model": model,
                        "model_path": str(model_path.relative_to(root)),
                        "model_sha256": digest(model_path),
                        "manifest_path": str(manifest_path.relative_to(root)),
                        "manifest_sha256": digest(manifest_path),
                        "source_revision": manifest["source_revision"],
                        "data_sha256": manifest["training_data_sha256"],
                        "metadata_sha256": manifest["metadata_sha256"],
                        "grouped_cv_accuracy_mean": manifest["grouped_cv_evaluation"]["mean_accuracy"],
                        "grouped_cv_accuracy_sd": manifest["grouped_cv_evaluation"]["std_accuracy"],
                        "holdout_accuracy": manifest["held_out_evaluation"]["accuracy"],
                        "holdout_macro_f1": manifest["held_out_evaluation"]["macro_f1"],
                        "holdout_rows": manifest["held_out_evaluation"]["n_rows"],
                    }
                )
                print(f"completed corpus={corpus} seed={seed} model={model}", flush=True)
    manifest = {
        "schema": "2048-ml.matched-corpus-fit-seeds",
        "schema_version": 1,
        "corpora": {
            name: {
                "data": str(config["data"]),
                "metadata": str(config["metadata"]),
                "data_sha256": digest(root / config["data"]),
                "metadata_sha256": digest(root / config["metadata"]),
                "first_fit_seed": config["first_seed"],
            }
            for name, config in CORPORA.items()
        },
        "replicates_per_corpus_candidate": args.replicates,
        "cv_folds": args.cv_folds,
        "development_fraction": args.development_fraction,
        "automl_commit": "82d848323eed5e2af86d046d529916c448f2442c",
        "runner_sha256": digest(Path(__file__).resolve()),
        "fits": records,
    }
    (output_dir / "fit-seed-runs.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"completed {len(records)} model fits; wrote {output_dir / 'fit-seed-runs.json'}")


if __name__ == "__main__":
    main()
