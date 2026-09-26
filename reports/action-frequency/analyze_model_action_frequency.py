#!/usr/bin/env python3
"""Summarize per-game action counts with a whole-game percentile bootstrap."""

import argparse
import csv
import hashlib
import json
import random
from pathlib import Path

ACTIONS = ("up", "down", "left", "right")


def percentile(sorted_values, probability):
    position = (len(sorted_values) - 1) * probability
    lower = int(position)
    upper = min(lower + 1, len(sorted_values) - 1)
    fraction = position - lower
    return sorted_values[lower] * (1 - fraction) + sorted_values[upper] * fraction


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("csv", type=Path)
    parser.add_argument("--seed", type=int, required=True)
    parser.add_argument("--expected-games", type=int, default=10_000)
    parser.add_argument("--bootstrap-seed", type=int, required=True)
    parser.add_argument("--replicates", type=int, default=2_000)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    games = []
    with args.csv.open(newline="") as handle:
        reader = csv.DictReader(handle)
        required = {"game_id", "seed", "move_count", *(f"{name}_moves" for name in ACTIONS)}
        if not required.issubset(reader.fieldnames or ()):
            raise SystemExit(f"missing CSV columns: {sorted(required - set(reader.fieldnames or ())) }")
        for row in reader:
            game_id, game_seed = int(row["game_id"]), int(row["seed"])
            if game_id != len(games) or game_seed != args.seed + game_id:
                raise SystemExit(f"unexpected game id or seed at row {len(games)}")
            moves = int(row["move_count"])
            counts = tuple(int(row[f"{name}_moves"]) for name in ACTIONS)
            if sum(counts) != moves:
                raise SystemExit(f"action counts do not sum to moves for game {game_id}")
            games.append(counts)

    if len(games) != args.expected_games:
        raise SystemExit(f"expected {args.expected_games} games, got {len(games)}")
    totals = [sum(game[i] for game in games) for i in range(4)]
    total_moves = sum(totals)
    if not total_moves:
        raise SystemExit("no selected moves found")
    rng = random.Random(args.bootstrap_seed)
    bootstrap = [[] for _ in ACTIONS]
    for _ in range(args.replicates):
        sample_totals = [0, 0, 0, 0]
        sample_moves = 0
        for game_index in (rng.randrange(len(games)) for _ in games):
            counts = games[game_index]
            sample_moves += sum(counts)
            for action in range(4):
                sample_totals[action] += counts[action]
        for action in range(4):
            bootstrap[action].append(sample_totals[action] / sample_moves)

    results = []
    for i, name in enumerate(ACTIONS):
        values = sorted(bootstrap[i])
        results.append({
            "action": i,
            "direction": name,
            "count": totals[i],
            "total_moves": total_moves,
            "proportion": totals[i] / total_moves,
            "game_cluster_bootstrap_95_ci": [percentile(values, 0.025), percentile(values, 0.975)],
        })

    report = {
        "schema": "2048-ml.model-action-frequency",
        "schema_version": 1,
        "benchmark": "model_policy",
        "games": len(games),
        "global_seed": args.seed,
        "first_game_seed": args.seed,
        "last_game_seed": args.seed + len(games) - 1,
        "seed_derivation": "global_seed.wrapping_add(game_id)",
        "action_frequency_unit": "selected moves across all games",
        "action_frequency_interval": f"95% whole-game percentile bootstrap ({args.replicates} replicates)",
        "action_frequency_bootstrap_seed": args.bootstrap_seed,
        "total_moves": total_moves,
        "action_frequency": results,
        "input_csv": str(args.csv),
        "input_csv_sha256": hashlib.sha256(args.csv.read_bytes()).hexdigest(),
        "limitations": [
            "Describes this fitted policy and evaluation seed sequence only",
            "Does not establish policy superiority or general AutoML performance",
            "The pilot policy was trained on a small development corpus",
        ],
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    for result in results:
        lo, hi = result["game_cluster_bootstrap_95_ci"]
        print(f"{result['direction']}: count={result['count']} proportion={result['proportion']:.6f} ci95=[{lo:.6f},{hi:.6f}]")
    print(f"games={len(games)} moves={total_moves} report={args.output}")


if __name__ == "__main__":
    main()
