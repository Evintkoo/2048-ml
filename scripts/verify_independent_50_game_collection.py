#!/usr/bin/env python3
"""Verify hashes, row alignment, seed coverage, and checkpoints for a 50-game run."""

import argparse
import csv
import hashlib
import json
from collections import Counter
from pathlib import Path


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    args = parser.parse_args()
    run_dir = args.run_dir.resolve()
    manifest_path = run_dir / "training.manifest.json"
    checkpoint_path = run_dir / "training.checkpoint/checkpoint.json"
    data_path = run_dir / "training.csv"
    metadata_path = run_dir / "training.metadata.csv"
    manifest = json.loads(manifest_path.read_text())
    checkpoint = json.loads(checkpoint_path.read_text())

    assert manifest["games"] == 50
    assert manifest["global_seed"] == 90702
    assert manifest["game_seeds"] == {
        "derivation": "SeedManager::game_seed(game_id) = global_seed.wrapping_add(game_id)",
        "first": 90702,
        "last": 90751,
    }
    assert manifest["rollouts_per_valid_action"] == 100
    assert manifest["threads"] == 2
    assert manifest["checkpoint_every_games"] == 1
    assert manifest["training_csv_sha256"] == digest(data_path)
    assert manifest["metadata_csv_sha256"] == digest(metadata_path)

    chunks = checkpoint["chunks"]
    assert len(chunks) == 50
    assert checkpoint["next_game_id"] == 50
    assert checkpoint["rows"] == manifest["training_rows"]
    assert checkpoint["states_collected"] == manifest["states_collected"]
    assert checkpoint["rollouts_evaluated"] == manifest["rollouts_evaluated"]
    for game_id, chunk in enumerate(chunks):
        assert chunk["first_game_id"] == game_id
        assert chunk["end_game_id_exclusive"] == game_id + 1

    with data_path.open(newline="") as stream:
        data_rows = list(csv.DictReader(stream))
    with metadata_path.open(newline="") as stream:
        metadata_rows = list(csv.DictReader(stream))
    assert len(data_rows) == len(metadata_rows) == manifest["training_rows"]
    assert list(data_rows[0]) == [f"grid_{i}" for i in range(16)] + [
        "score_normalized",
        "action",
    ]
    assert list(metadata_rows[0]) == ["row_index", "game_id", "move_index", "score"]

    per_game_rows = Counter()
    for row_index, (features, metadata) in enumerate(zip(data_rows, metadata_rows)):
        assert int(metadata["row_index"]) == row_index
        game_id = int(metadata["game_id"])
        assert 0 <= game_id < 50
        assert int(metadata["move_index"]) >= 0
        assert 0 <= int(features["action"]) < 4
        assert all(float(features[f"grid_{i}"]) >= 0 for i in range(16))
        per_game_rows[game_id] += 1
    assert set(per_game_rows) == set(range(50))
    assert [per_game_rows[i] for i in range(50)] == [chunk["rows"] for chunk in chunks]
    assert sum(per_game_rows.values()) == checkpoint["rows"]

    result = {
        "status": "pass",
        "manifest_sha256": digest(manifest_path),
        "checkpoint_sha256": digest(checkpoint_path),
        "training_csv_sha256": digest(data_path),
        "metadata_csv_sha256": digest(metadata_path),
        "games": 50,
        "first_game_seed": 90702,
        "last_game_seed": 90751,
        "rows": len(data_rows),
        "rollouts_evaluated": manifest["rollouts_evaluated"],
        "elapsed_seconds": manifest["elapsed_seconds"],
        "all_game_rows_match_checkpoint": True,
    }
    verification_path = run_dir / "verification.json"
    verification_path.write_text(json.dumps(result, indent=2) + "\n")
    print(
        f"verified {result['games']} games, {result['rows']} rows, "
        f"{result['rollouts_evaluated']} rollouts; wrote {verification_path}"
    )


if __name__ == "__main__":
    main()
