#!/usr/bin/env python3
"""Verify five repeated fit seeds per candidate on both 50-game corpora."""

import argparse
import csv
import hashlib
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path


MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
CORPORA = ("previous", "independent")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_csv(path):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        return reader.fieldnames, list(reader)


def scores(actual, predicted):
    matrix = [[0] * 4 for _ in range(4)]
    for truth, guess in zip(actual, predicted):
        matrix[truth][guess] += 1
    f1 = []
    for label in range(4):
        tp = matrix[label][label]
        fp = sum(matrix[row][label] for row in range(4)) - tp
        fn = sum(matrix[label]) - tp
        denominator = 2 * tp + fp + fn
        f1.append(0.0 if denominator == 0 else 2 * tp / denominator)
    return {
        "accuracy": sum(a == p for a, p in zip(actual, predicted)) / len(actual),
        "macro_f1": sum(f1) / 4,
        "confusion_matrix_actual_rows_predicted_columns": matrix,
    }


def close(a, b):
    return math.isclose(float(a), float(b), rel_tol=0.0, abs_tol=1e-12)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--run-dir",
        type=Path,
        default=Path("reports/candidate_classifier_pilot/matched-50-game-corpus-fit-seeds-2026-09-27"),
    )
    args = parser.parse_args()
    root = Path.cwd().resolve()
    run_dir = (root / args.run_dir).resolve()
    runs_path = run_dir / "fit-seed-runs.json"
    runs = json.loads(runs_path.read_text())
    assert runs["schema_version"] == 1
    assert runs["replicates_per_corpus_candidate"] >= 2
    assert runs["cv_folds"] == 5
    assert runs["development_fraction"] == 0.8
    assert runs["automl_commit"] == "82d848323eed5e2af86d046d529916c448f2442c"

    expected = {
        (corpus, seed, model)
        for corpus in CORPORA
        for seed in range(
            runs["corpora"][corpus]["first_fit_seed"],
            runs["corpora"][corpus]["first_fit_seed"] + runs["replicates_per_corpus_candidate"],
        )
        for model in MODELS
    }
    by_key = {}
    for record in runs["fits"]:
        key = (record["corpus"], record["seed"], record["model"])
        assert key not in by_key
        by_key[key] = record
    assert set(by_key) == expected

    verified = []
    for corpus in CORPORA:
        corpus_info = runs["corpora"][corpus]
        data_path = root / corpus_info["data"]
        metadata_path = root / corpus_info["metadata"]
        assert digest(data_path) == corpus_info["data_sha256"]
        assert digest(metadata_path) == corpus_info["metadata_sha256"]
        header, data = read_csv(data_path)
        meta_header, metadata = read_csv(metadata_path)
        assert len(header) == 18 and len(data) == len(metadata)
        assert "game_id" in meta_header and "row_index" in meta_header
        game_ids = [int(row["game_id"]) for row in metadata]
        holdout_indices = [i for i, game_id in enumerate(game_ids) if game_id >= 40]
        holdout_index_set = set(holdout_indices)
        assert set(game_ids) == set(range(50))
        assert all(game_ids[i] < 40 for i in range(len(game_ids)) if i not in holdout_index_set)
        expected_game_ids = [game_ids[i] for i in holdout_indices]
        expected_actual = [int(float(data[i]["action"])) for i in holdout_indices]

        for seed in range(
            corpus_info["first_fit_seed"],
            corpus_info["first_fit_seed"] + runs["replicates_per_corpus_candidate"],
        ):
            for model in MODELS:
                record = by_key[(corpus, seed, model)]
                model_path = root / record["model_path"]
                model_manifest_path = root / record["manifest_path"]
                model_manifest = json.loads(model_manifest_path.read_text())
                evaluation = model_manifest["held_out_evaluation"]
                prediction_path = Path(evaluation["predictions_csv"])
                if not prediction_path.is_absolute():
                    prediction_path = root / prediction_path
                assert digest(model_path) == record["model_sha256"] == model_manifest["model_artifact_sha256"]
                assert digest(model_manifest_path) == record["manifest_sha256"]
                assert model_manifest["global_seed"] == seed
                assert model_manifest["model"] == model
                assert model_manifest["training_data_sha256"] == corpus_info["data_sha256"]
                assert model_manifest["metadata_sha256"] == corpus_info["metadata_sha256"]
                assert model_manifest["state_feature_count"] == 17
                assert model_manifest["cv_folds"] == 5
                assert model_manifest["development_fraction"] == 0.8
                assert model_manifest["held_out_evaluation"]["game_ids"] == list(range(40, 50))
                assert digest(prediction_path) == evaluation["predictions_csv_sha256"]
                pred_header, predictions = read_csv(prediction_path)
                assert pred_header == ["row_index", "game_id", "actual_action", "predicted_action"]
                assert [int(row["row_index"]) for row in predictions] == holdout_indices
                assert [int(row["game_id"]) for row in predictions] == expected_game_ids
                actual = [int(row["actual_action"]) for row in predictions]
                predicted = [int(row["predicted_action"]) for row in predictions]
                assert actual == expected_actual
                measured = scores(actual, predicted)
                assert close(measured["accuracy"], evaluation["accuracy"])
                assert close(measured["macro_f1"], evaluation["macro_f1"])
                assert measured["confusion_matrix_actual_rows_predicted_columns"] == evaluation[
                    "confusion_matrix_actual_rows_predicted_columns"
                ]
                verified.append(
                    {
                        "corpus": corpus,
                        "seed": seed,
                        "model": model,
                        **measured,
                        "grouped_cv_accuracy_mean": model_manifest["grouped_cv_evaluation"]["mean_accuracy"],
                        "grouped_cv_accuracy_sd": model_manifest["grouped_cv_evaluation"]["std_accuracy"],
                        "model_sha256": digest(model_path),
                        "manifest_sha256": digest(model_manifest_path),
                        "predictions_sha256": digest(prediction_path),
                    }
                )

    by_model_corpus = defaultdict(list)
    for record in verified:
        by_model_corpus[(record["corpus"], record["model"])].append(record)
    summary = []
    for model in MODELS:
        entry = {"model": model, "corpora": {}}
        for corpus in CORPORA:
            records = sorted(by_model_corpus[(corpus, model)], key=lambda row: row["seed"])
            accuracy = [record["accuracy"] for record in records]
            macro_f1 = [record["macro_f1"] for record in records]
            cv_accuracy = [record["grouped_cv_accuracy_mean"] for record in records]
            entry["corpora"][corpus] = {
                "fit_seeds": [record["seed"] for record in records],
                "holdout_accuracy_mean": statistics.mean(accuracy),
                "holdout_accuracy_sample_sd": statistics.stdev(accuracy),
                "holdout_macro_f1_mean": statistics.mean(macro_f1),
                "holdout_macro_f1_sample_sd": statistics.stdev(macro_f1),
                "grouped_cv_accuracy_mean_across_fit_seeds": statistics.mean(cv_accuracy),
                "per_seed": records,
            }
        entry["new_minus_previous_holdout_accuracy_mean"] = (
            entry["corpora"]["independent"]["holdout_accuracy_mean"]
            - entry["corpora"]["previous"]["holdout_accuracy_mean"]
        )
        entry["new_minus_previous_holdout_macro_f1_mean"] = (
            entry["corpora"]["independent"]["holdout_macro_f1_mean"]
            - entry["corpora"]["previous"]["holdout_macro_f1_mean"]
        )
        summary.append(entry)

    result = {
        "status": "pass",
        "run_manifest_sha256": digest(runs_path),
        "replicates_per_corpus_candidate": runs["replicates_per_corpus_candidate"],
        "corpus_count": len(CORPORA),
        "candidate_count": len(MODELS),
        "verified_fit_count": len(verified),
        "inferential_tests_performed": False,
        "interpretation_limit": "Fit-seed variation is conditional on each fixed corpus. The corpus contrast is descriptive with two corpora; no inferential test or policy-score claim follows from classifier-label metrics.",
        "candidate_summary": summary,
    }
    (run_dir / "fit-seed-verification.json").write_text(json.dumps(result, indent=2) + "\n")
    print(f"verified {len(verified)} fits across two corpora and wrote fit-seed-verification.json")


if __name__ == "__main__":
    main()
