#!/usr/bin/env python3
"""Create a deterministic, explicitly synthetic grouped-CV wiring fixture."""

import csv
import hashlib
import json
from collections import defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE_DATA = ROOT / "reports/collection_pilots/2026-09-27/training.csv"
SOURCE_META = ROOT / "reports/collection_pilots/2026-09-27/metadata.csv"
OUT = ROOT / "reports/configuration_smokes/2026-09-27"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    with SOURCE_DATA.open(newline="") as source:
        data_rows = list(csv.DictReader(source))
    with SOURCE_META.open(newline="") as source:
        metadata_rows = list(csv.DictReader(source))
    if len(data_rows) != len(metadata_rows):
        raise ValueError("pilot training and metadata row counts differ")
    if [int(row["row_index"]) for row in metadata_rows] != list(range(len(data_rows))):
        raise ValueError("pilot metadata row index is not aligned")
    by_action = defaultdict(list)
    for index, row in enumerate(data_rows):
        by_action[int(row["action"])].append(index)
    if set(by_action) != {0, 1, 2, 3} or any(len(rows) < 2 for rows in by_action.values()):
        raise ValueError("source pilot must contain at least two rows per action")

    OUT.mkdir(parents=True, exist_ok=True)
    data_path = OUT / "synthetic-training.csv"
    metadata_path = OUT / "synthetic-metadata.csv"
    fields = list(data_rows[0])
    with data_path.open("w", newline="") as output:
        writer = csv.DictWriter(output, fieldnames=fields)
        writer.writeheader()
        for game_id in range(10):
            for action in range(4):
                candidates = by_action[action]
                for copy_index in range(2):
                    source_index = candidates[(game_id * 2 + copy_index) % len(candidates)]
                    writer.writerow(data_rows[source_index])
    metadata_fields = ["row_index", "game_id", "move_index", "score"]
    with metadata_path.open("w", newline="") as output:
        writer = csv.DictWriter(output, fieldnames=metadata_fields)
        writer.writeheader()
        row_index = 0
        for game_id in range(10):
            move_index = 0
            for action in range(4):
                candidates = by_action[action]
                for copy_index in range(2):
                    source_index = candidates[(game_id * 2 + copy_index) % len(candidates)]
                    writer.writerow(
                        {
                            "row_index": row_index,
                            "game_id": game_id,
                            "move_index": move_index,
                            "score": metadata_rows[source_index]["score"],
                        }
                    )
                    row_index += 1
                    move_index += 1
    manifest = {
        "schema": "2048-ml.synthetic-hyperopt-wiring-fixture",
        "schema_version": 1,
        "purpose": "CLI grouped-CV/TPE path smoke only; not a game corpus or research dataset",
        "source_training_csv": str(SOURCE_DATA.relative_to(ROOT)),
        "source_metadata_csv": str(SOURCE_META.relative_to(ROOT)),
        "source_training_sha256": sha256(SOURCE_DATA),
        "source_metadata_sha256": sha256(SOURCE_META),
        "construction": "For each of ten synthetic game IDs, cycle through two source pilot rows for each action; state rows are intentionally reused across IDs.",
        "synthetic_game_ids": list(range(10)),
        "rows_per_synthetic_game": 8,
        "rows_total": 80,
        "action_counts": {str(action): 20 for action in range(4)},
        "training_csv": data_path.name,
        "training_csv_sha256": sha256(data_path),
        "metadata_csv": metadata_path.name,
        "metadata_csv_sha256": sha256(metadata_path),
        "runner_sha256": sha256(Path(__file__).resolve()),
        "limitations": [
            "Rows are reused across synthetic game IDs and are not independent trajectories",
            "Any accuracy is fixture behavior only and must not be interpreted as model quality",
            "No canonical rollout corpus or game evaluation is represented",
        ],
    }
    (OUT / "fixture-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"wrote {manifest['rows_total']} synthetic rows across 10 groups to {OUT}")


if __name__ == "__main__":
    main()
