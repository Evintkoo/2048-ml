# Results — Results-Generation Interface (Pending Experimentation)

> **Status: PENDING.** No results claimed. This file defines the concrete interfaces for the primary Rust-native AutoML framework evaluation and the downstream 2048 case-study outputs.

## 1. Primary Framework Results

Framework results must report, for each named dataset and configuration:

- Predictive quality using task-appropriate metrics.
- Training and inference time.
- Peak memory and CPU use.
- Hyperparameter-search budget and best-trial trajectory.
- Failure rate and diagnostic category.
- Repeated-run reproducibility.
- Model serialization and reload equivalence.
- CLI/library/API output equivalence.

| Dataset | Configuration | Metric | Time | Memory | Failures | Reproducibility | Reload Equivalent |
|---------|---------------|--------|------|--------|----------|-----------------|-------------------|
| TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD |

No 2048 game score can substitute for this table.

## 2. 2048 Case-Study Winner Protocol (Canonical: `07-Benchmarking/01-Evaluation/01-benchmarking-framework.md`)

Winner = highest **mean score** over **≥10,000 games** at **seed 42**. The primary comparison uses the pre-registered Mann-Whitney U test with Holm correction; bootstrap CIs and effect sizes are reported alongside the ranking:

1. **Mann-Whitney U p < 0.05 after Holm correction** for the pre-registered baseline comparisons,
2. **Bootstrap 95% CI on mean difference** (10,000 resamples),
3. **Effect size** reported for practical interpretation.

Random baseline `~128` is lower bound only.

## 3. Concrete Schemas

### 2.1 Parquet Schema (polars 0.46) — `ScoreMetrics`

Produced by `src/evaluation/benchmark_runner.rs` → `data/evaluation_data/evaluation_v1.parquet`:

```rust
pub struct ScoreMetrics {
    pub model: String,          // e.g., "RandomForest" | "Heuristic" | "Random"
    pub seed: u64,              // 42 primary
    pub game_id: u64,           // 0..9999 — GroupKFold key
    pub score: u32,             // sum of merges, overflow-checked u32
    pub max_tile: u32,          // 2..32768
    pub moves: u16,             // game length
    pub action: u8,             // 0=Up 1=Down 2=Left 3=Right (for per-step logs)
    pub valid_move: bool,       // did action change board?
    pub duration_ms: u32,
}
// + run metadata sidecar: { automl: "v1.0.0", rust: "1.75", task: "MultiClassification", dim: 27 }
```

GroupKFold MUST keep all rows from the same `game_id` in one fold. Do not claim that rows or games are independent merely because they use different IDs or the same seed. The analysis must declare whether the inferential unit is a row, game, seed, or trained-model run. See `02-Methodology/01-experimental-design.md` §5 and `03-Findings/06-cross-validation.md`.

### 2.2 Statistical API — `src/evaluation/statistical_tests.rs`

```rust
pub fn mann_whitney(a: &[u32], b: &[u32]) -> (f64 /*U*/, f64 /*p*/);
pub fn bootstrap_ci(a: &[u32], b: &[u32], resamples: usize, alpha: f64) -> (f64, f64); // (lo, hi) on mean diff
pub fn cohens_d(a: &[u32], b: &[u32]) -> f64;
pub fn kruskal_wallis(groups: &[&[u32]]) -> (f64 /*H*/, f64 /*p*/);
pub fn bonferroni(p: f64, k: usize) -> f64; // min(p*k, 1.0)
```

All tests use raw scores; no normality assumption. Report exact U/H, exact p, d, and CI.

## 3. Table Shells (Populated by Pipeline, Not Hand-Edited)

### 3.1 Primary Ranking (output of `ranking_analysis.py`)

| Model | Mean | Median | SD | 95% CI (bootstrap) | Rank | Training time | MWU vs #2 |
|-------|------|--------|----|---------------------|------|---------------|-----------|
| TBD | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | p=TBD, d=TBD |

Heuristic row pinned at `~512` and Random at `~128` for reference only; still computed from same 10k games.

### 3.2 Gate Table

| Comparison | U | p (Bonf.) | d | Bootstrap CI on diff | Gate pass? |
|------------|---|-----------|---|----------------------|------------|
| Best vs Heuristic (~512) | TBD | TBD | TBD | [TBD, TBD] | TBD |
| Best vs 2nd best | TBD | TBD | TBD | [TBD, TBD] | TBD |
| Kruskal-Wallis (7 models) | H=TBD | TBD | — | — | — |

### 3.3 Learning-Curve Hooks

Per-epoch: `epoch, train_loss, val_accuracy_4class, val_mean_score_100games`. Used by `03-Findings/01-key-findings.md`.

## 4. Visualization Spec (Generated, Not Mocked)

1. Histogram of `score` per model (10k points, bin 128). 2. Bar chart mean ± bootstrap CI. 3. Ranked bar. 4. Box plot. 5. Learning curve (score vs epoch). All from `evaluation_v1.parquet`.

## 5. Data-Quality Checklist (Automated)

- [ ] `game_id` unique, GroupKFold used
- [ ] No NaN scores, `score` monotonic via `ScoreTracker`
- [ ] `valid_move` rate logged (invalid-move audit in `02-insights.md`)
- [ ] Seed 42 recorded in sidecar; secondary seeds separate files
- [ ] 27-dim vector length asserted per row

## 6. Honest Reporting

TBD cells remain TBD until pipeline runs. Null (no model beats `~512`) reported as primary finding if gate fails.
