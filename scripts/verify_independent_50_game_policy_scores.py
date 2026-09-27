#!/usr/bin/env python3
"""Verify a 10,000-seed policy matrix against its training and collection inputs."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


MODELS = ("random_forest", "extra_trees", "adaboost", "knn", "naive_bayes")
AGENTS = ("random", "heuristic", *MODELS)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_csv(path):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        return reader.fieldnames, list(reader)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report-dir", type=Path, required=True)
    parser.add_argument("--training-report-dir", type=Path, required=True)
    parser.add_argument("--collection-dir", type=Path, required=True)
    parser.add_argument("--previous-training-report-dir", type=Path, required=True)
    parser.add_argument("--previous-collection-dir", type=Path, required=True)
    args = parser.parse_args()
    root = Path.cwd().resolve()
    report = (root / args.report_dir).resolve()
    training_report = (root / args.training_report_dir).resolve()
    collection = (root / args.collection_dir).resolve()
    previous_training_report = (root / args.previous_training_report_dir).resolve()
    previous_collection = (root / args.previous_collection_dir).resolve()
    collection_manifest = json.loads((collection / "training.manifest.json").read_text())
    previous_collection_manifest = json.loads(
        (previous_collection / "training.manifest.json").read_text()
    )
    collection_verification = json.loads((collection / "verification.json").read_text())
    assert collection_verification["status"] == "pass"
    seed_list = list(range(114024, 124024))
    results = {}
    corpus_fit_comparisons = {}
    input_paths = []

    for agent in AGENTS:
        stem = f"{agent}-10000.csv"
        score_path = report / stem
        manifest_path = score_path.with_suffix(".manifest.json")
        manifest = json.loads(manifest_path.read_text())
        assert manifest["games"] == 10_000
        assert manifest["global_seed"] == seed_list[0]
        assert manifest["first_game_seed"] == seed_list[0]
        assert manifest["last_game_seed"] == seed_list[-1]
        assert manifest["sha256"] == digest(score_path)
        assert manifest["results_csv"] == str(score_path.relative_to(root))
        fields, rows = read_csv(score_path)
        assert len(rows) == len(seed_list)
        assert [int(row["seed"]) for row in rows] == seed_list
        assert [int(row["game_id"]) for row in rows] == list(range(10_000))
        assert "score" in fields
        input_paths.append(str(score_path.relative_to(root)))

        if agent in MODELS:
            policy_path = training_report / "models" / f"{agent}.policy.json"
            policy_manifest_path = training_report / "models" / f"{agent}.policy.manifest.json"
            policy_manifest = json.loads(policy_manifest_path.read_text())
            assert digest(policy_path) == policy_manifest["model_artifact_sha256"]
            assert policy_manifest["training_data_sha256"] == collection_manifest["training_csv_sha256"]
            assert policy_manifest["metadata_sha256"] == collection_manifest["metadata_csv_sha256"]
            assert policy_manifest["automl_commit"] == collection_manifest["automl_commit"]
            assert manifest["model"] == str(policy_path.relative_to(root))
            assert manifest["automl_commit"] == collection_manifest["automl_commit"]
            results[agent] = {
                "mean_score": manifest["summary"]["mean"],
                "score_csv_sha256": digest(score_path),
                "policy_sha256": digest(policy_path),
                "training_manifest_sha256": digest(policy_manifest_path),
            }

            previous_policy_path = previous_training_report / "models" / f"{agent}.policy.json"
            previous_policy_manifest_path = previous_training_report / "models" / f"{agent}.policy.manifest.json"
            previous_policy_manifest = json.loads(previous_policy_manifest_path.read_text())
            assert digest(previous_policy_path) == previous_policy_manifest["model_artifact_sha256"]
            assert previous_policy_manifest["training_data_sha256"] == previous_collection_manifest["training_csv_sha256"]
            assert previous_policy_manifest["metadata_sha256"] == previous_collection_manifest["metadata_csv_sha256"]

            previous_score_path = report / "previous-50-game-fits" / stem
            previous_score_manifest_path = previous_score_path.with_suffix(".manifest.json")
            previous_score_manifest = json.loads(previous_score_manifest_path.read_text())
            assert previous_score_manifest["sha256"] == digest(previous_score_path)
            assert previous_score_manifest["model"] == str(previous_policy_path.relative_to(root))
            previous_fields, previous_rows = read_csv(previous_score_path)
            assert len(previous_rows) == 10_000
            assert [int(row["seed"]) for row in previous_rows] == seed_list

            pair_path = report / "corpus-fit-pairs" / f"{agent}-comparison.csv"
            pair_manifest_path = pair_path.with_suffix(".manifest.json")
            pair_manifest = json.loads(pair_manifest_path.read_text())
            assert pair_manifest["sha256"] == digest(pair_path)
            assert set(pair_manifest["input_files"]) == {
                str(score_path.relative_to(root)),
                str(previous_score_path.relative_to(root)),
            }
            pair_fields, pair_rows = read_csv(pair_path)
            assert len(pair_rows) == 1
            pair = pair_rows[0]
            assert pair["seed_sets_match"] == "true"
            assert int(pair["first_n"]) == int(pair["second_n"]) == 10_000
            assert pair["first_mean"] == f"{manifest['summary']['mean']:.6f}"
            assert pair["second_mean"] == f"{previous_score_manifest['summary']['mean']:.6f}"
            corpus_fit_comparisons[agent] = {
                "mean_difference_new_minus_previous": float(pair["first_mean"])
                - float(pair["second_mean"]),
                "paired_ci95": [
                    float(pair["mean_difference_ci95_low"]),
                    float(pair["mean_difference_ci95_high"]),
                ],
                "cohens_dz": float(pair["effect_size"]),
                "sign_test_p_value_display": pair["p_value"],
                "pairwise_holm_p_display": pair["holm_p"],
                "comparison_sha256": digest(pair_path),
                "previous_policy_sha256": digest(previous_policy_path),
                "previous_policy_training_manifest_sha256": digest(previous_policy_manifest_path),
            }
        else:
            assert manifest["agent"] == agent
            results[agent] = {"mean_score": manifest["summary"]["mean"], "score_csv_sha256": digest(score_path)}

    comparison_path = report / "comparison.csv"
    comparison_manifest_path = report / "comparison.manifest.json"
    comparison_manifest = json.loads(comparison_manifest_path.read_text())
    assert comparison_manifest["sha256"] == digest(comparison_path)
    assert set(comparison_manifest["input_files"]) == set(input_paths)
    fields, comparisons = read_csv(comparison_path)
    assert len(comparisons) == 21
    assert len(comparisons[0]) == len(fields)
    assert all(row["seed_sets_match"] == "true" for row in comparisons)
    assert all(int(row["first_n"]) == int(row["second_n"]) == 10_000 for row in comparisons)
    assert len(corpus_fit_comparisons) == len(MODELS)

    result = {
        "status": "pass",
        "evaluation_seed_first": seed_list[0],
        "evaluation_seed_last": seed_list[-1],
        "games_per_agent": len(seed_list),
        "agent_count": len(AGENTS),
        "paired_comparisons": len(comparisons),
        "comparison_sha256": digest(comparison_path),
        "collection_verification_sha256": digest(collection / "verification.json"),
        "agents": results,
        "same_size_corpus_fit_comparisons": corpus_fit_comparisons,
        "corpus_fit_inference_note": "Each candidate's paired comparison was run separately; its reported Holm value is within that one-pair run, not adjusted across the family of five candidates. Treat these fixed-fit contrasts as descriptive sensitivity evidence.",
    }
    (report / "verification.json").write_text(json.dumps(result, indent=2) + "\n")
    print(f"verified {len(AGENTS)} agents and {len(comparisons)} paired comparisons")


if __name__ == "__main__":
    main()
