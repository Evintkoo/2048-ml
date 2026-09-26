# Plan 01 — Score Tracking: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-26).** Score tracker records merge details, cumulative event score, per-turn gains, merge count, and maximum tile.

**Goal:** State the current implementation and evidence boundary for score tracking.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats score tracking as implemented metadata, not as a research finding.** `ScoreTracker` initializes from the game board, aggregates score and merge counts, tracks the maximum tile reached, and retains per-turn and per-merge records. Score remains metadata; `action` is the training label.

## 1. Purpose

Track scores for **analysis/benchmarking only**. Score is metadata — the supervised label is `action: u8 0..3` (`TaskType::MultiClassification`). ScoreTracker is a metadata logger, not a training input builder.

## 2. Score Structure

```rust
pub struct ScoreTracker {
    pub total_score: u64,
    pub merge_history: Vec<MergeEvent>,
    pub turn_scores: Vec<u64>,
    pub total_merges: u64,
    pub max_tile_ever: u64,
}
// MergeEvent also records cumulative_score after this merge.
```

## 3. Score Recording — Canonical

> **2048 scoring rule:** merging two `value` tiles produces one `2*value` tile; **score gained = `2*value`** which equals the **merged tile value**. Pass `merged_tile_value` (not half).

`RawBoardState::execute_move` creates one `MergeEvent` per merge, including the
merged value, result position, turn, score gained, and cumulative score at that
merge. `ScoreTracker::record_move` aggregates the move result and its merges.

## 4. Score Metrics (Benchmark Only)

```rust
pub struct ScoreMetrics {
    pub mean_score: f64, pub median_score: u64, pub std_dev_score: f64,
    pub max_score: u64, pub min_score: u64,
    pub percentiles: [u64; 10],
    pub games_above_2048: usize, pub games_above_4096: usize, pub games_above_8192: usize,
}
```

## 5. Training Sample — Action Classification (Not Score Regression)

Score is **never `y`**. Canonical: `27-dim → 4 logits → argmax`.

```rust
pub struct TrainingSample {
    pub state: [f64; 27],
    pub action: u8,          // 0..3 — ONLY label (TaskType::MultiClassification)
    // pub score: u64 — metadata only, sidecar for analysis
}
```

## 6. Reward Structs — For Analysis Only, Not for Supervised automl

> Not used for training. Supervised pipeline uses `(state, action)` classification. Retained for post-hoc analysis — must not be fed into automl.

```rust
// Analysis only — do not feed reward into automl
pub struct Reward { pub immediate_reward: f64, pub survival_reward: f64, pub progress_reward: f64, pub final_reward: f64 }
```

## 7. Score Logging — Analysis Artifact

```rust
Game results serialize `final_score` and the `ScoreTracker`; benchmark commands
write per-game scores to CSV and include score summaries in JSON manifests.
```

## 8. Score Visualization — Analysis Only

Histogram generation is an analysis helper — not a training step. Implement only if needed for the report.

## 9. Score Quality Checks

Scores are stored as `u64`, so they cannot be NaN or infinite. Board score
updates use checked addition and return `ScoreOverflow` on overflow.

## Implementation Record

- `ScoreTracker` records initial/final score, per-turn gains, cumulative score per merge, merge count, and maximum tile ever reached. Histogram generation and score distribution report helpers are not implemented and remain optional analysis work.
- Automated coverage confirms cumulative scores, merge counts, maximum tile tracking, and correct merge positions for all directions.

---

## Verification (definition of done)

1. `test -f plans/03-State/02-Score/01-score-tracking.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/03-State/02-Score/01-score-tracking.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/03-State/02-Score/01-score-tracking.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/03-State/02-Score/01-score-tracking.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/03-State/02-Score/01-score-tracking.md` exits 0.
6. `grep -q '^## Open questions$' plans/03-State/02-Score/01-score-tracking.md` exits 0.
7. `grep -q '^## Later$' plans/03-State/02-Score/01-score-tracking.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/03-State/02-Score/01-score-tracking.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
