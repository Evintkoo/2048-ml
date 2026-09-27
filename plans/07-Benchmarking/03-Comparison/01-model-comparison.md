# Plan 01 — Model Comparison: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Candidate and comparison tooling are documented; a paired random/heuristic/single-pilot comparison exists, while the selected five-policy matrix remains unrun.

**Goal:** State the current implementation and evidence boundary for model comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats model compatibility and score comparison support as implemented, with the selected-candidate ranking pending.** The verified four-class probability candidates are RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes. Random and heuristic baselines and one fitted RandomForest pilot were scored on a common 10,000-seed set and compared in `reports/action-frequency/pilot-comparison.md`. The RandomForest came from a small pilot corpus and was not selected under a predeclared model-selection protocol; this is exploratory evidence, not the planned five-candidate comparison.

## 1. Purpose

Compare the real `automl` `ModelType` variants on the canonical 17-value state→action `MultiClassification` task. No placeholder "Architecture A/B/C".

## 2. Variants Under Test

The original model list is gated by the pinned framework's four-class `predict_proba` behavior. The current runnable candidates are `RandomForest`, `ExtraTrees`, `AdaBoost`, `KNN`, and `NaiveBayes`. `GradientBoosting`, `XGBoost`, and `LightGBM` were inspected and excluded because they returned two probability columns on the four-action smoke task. See the capability record in the project overview.

| `ModelType` variant | Family | Notes |
|---------------------|--------|-------|
| `RandomForest` | Tree ensemble | Baseline ensemble |
| `ExtraTrees` | Tree ensemble | Extra-randomized |
| `AdaBoost` | Boosting | Verified four-class output |
| `KNN` | Nearest neighbors | Verified four-class output |
| `NaiveBayes` | Probabilistic | Verified four-class output |
| (+ others if `automl` exposes them) | — | Add rows as engines evolve |

Add rows only for engines actually exposed by `automl` — no invented architectures.

## 3. Protocol — Identical Conditions

The proposed final test uses a predeclared held-out game-seed set. Choose its size from pilot variance and available compute; no fixed target is a completed run or power guarantee. Training and test games must remain separate. The `benchmark compare` command aligns rows by identical seeds and uses a two-sided exact sign test for matched seed runs; otherwise it uses Mann-Whitney U. It reports Holm-adjusted p-values, paired-difference bootstrap intervals and Cohen's dz for matching seed sets, or independent bootstrap intervals and Cohen's d otherwise. The paired sign test is not Wilcoxon and ignores tied outcomes; the exact test choice and its limitation are recorded in the output manifest. Do not describe held-out benchmark games as the chronological data split itself.

## 4. Performance Matrix — To Be Filled Post-Training

| Model | Mean | Median | Std | Adjusted pairwise comparison (per declared design) | Rank |
|-------|------|--------|-----|-----------------------------------------------|------|
| RF | TBD | TBD | TBD | — | TBD |
| ExtraTrees | TBD | TBD | TBD | — | TBD |
| AdaBoost | TBD | TBD | TBD | — | TBD |
| KNN | TBD | TBD | TBD | — | TBD |
| NaiveBayes | TBD | TBD | TBD | — | TBD |
| Heuristic | TBD | TBD | TBD | ref | — |
| Random | TBD | TBD | TBD | ref | — |

This matrix remains unfilled because the pilot policy is not a selected candidate and its training corpus is inadequate for case-study ranking. The separate exploratory run reports mean scores for Random (1,094.12), Heuristic (8,056.23), and the fitted RandomForest pilot (866.15); see `reports/action-frequency/pilot-comparison.md` for methods and limitations. These observations do not fill the five-candidate selected-model matrix.

> Matrix is filled **post-training** — no pre-filled winners. The descriptive ranking uses held-out mean; use the predeclared paired or unmatched comparison procedure, Holm adjustment, bootstrap CI, and effect size to characterize uncertainty and practical magnitude. See `04-Analysis/03-significance-testing.md`.

## Implementation Record

- CLI has model/agent score comparison and statistical primitives. One paired 10,000-game baseline/pilot analysis is retained, but the selected five-candidate held-out matrix remains unrun; its result rows are not populated from the exploratory pilot.

---

## Verification (definition of done)

1. `test -f plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
6. `grep -q '^## Open questions$' plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
7. `grep -q '^## Later$' plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/07-Benchmarking/03-Comparison/01-model-comparison.md` exits 0.

## Open questions

- Execute after model selection, an adequate labeled corpus, and a predeclared held-out protocol. The retained pilot comparison is not a substitute. Retain identical environment settings, per-game outcomes, and input manifests for every candidate.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
