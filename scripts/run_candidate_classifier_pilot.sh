#!/usr/bin/env bash
set -euo pipefail

report_dir="reports/candidate_classifier_pilot/2026-09-27"
data="reports/collection_pilots/2026-09-27-20-game/training.csv"
metadata="reports/collection_pilots/2026-09-27-20-game/training.metadata.csv"
mkdir -p "$report_dir"

for model in random_forest extra_trees adaboost knn naive_bayes; do
  cargo run --quiet -- train \
    --data "$data" \
    --metadata "$metadata" \
    --model "$model" \
    --cv-folds 5 \
    --development-fraction 0.85 \
    --seed 90627 \
    --output "$report_dir/$model.policy.json"
done
