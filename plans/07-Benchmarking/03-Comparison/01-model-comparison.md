# Plan 01 — Model Comparison: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-30).** Separate disjoint-seed exploratory score matrices cover five policies trained on 20- and 50-game corpora plus two baselines; repeated-training confirmatory ranking remains pending.

**Goal:** State the current implementation and evidence boundary for model comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats model compatibility and exploratory score comparison as implemented, with confirmatory model selection pending.** The five verified candidates are RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes. Each was fitted once on the same small rollout corpus and scored for 10,000 game seeds that do not overlap its training seeds. Random and heuristic baselines were evaluated on the same seeds. The resulting matrix and paired analysis are in `reports/action-frequency/disjoint-seeds/README.md`; the result is exploratory because the corpus is small and model selection was not predeclared.

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

## 4. Exploratory Performance Matrix

| Model | Mean | Median | Std | 95% bootstrap CI for mean | Rank by mean |
|-------|------:|-------:|----:|--------------------------|-------------:|
| Heuristic | 8,096.70 | 7,140 | 3,503.15 | [8,028.04, 8,168.45] | 1 |
| Random | 1,086.52 | 1,048 | 527.16 | [1,076.03, 1,097.03] | 2 |
| NaiveBayes | 914.14 | 744 | 546.81 | [903.49, 924.74] | 3 |
| KNN | 887.35 | 776 | 445.03 | [879.02, 895.82] | 4 |
| RandomForest | 864.44 | 740 | 447.96 | [855.48, 873.05] | 5 |
| ExtraTrees | 837.15 | 736 | 459.45 | [827.92, 846.14] | 6 |
| AdaBoost | 765.62 | 680 | 399.06 | [757.52, 773.64] | 7 |

The comparison CSV at `reports/action-frequency/disjoint-seeds/score-comparison.csv`
retains all 21 pairwise comparisons in one Holm family, including paired-difference
intervals and Cohen's dz. NaiveBayes has the highest mean among these five fixed
fits; this is only a descriptive ordering for one small training corpus and one
evaluation seed set, not a general model winner.

> Matrix is filled **post-training** — no pre-filled winners. The descriptive ranking uses held-out mean; use the predeclared paired or unmatched comparison procedure, Holm adjustment, bootstrap CI, and effect size to characterize uncertainty and practical magnitude. See `04-Analysis/03-significance-testing.md`.

## Implementation Record

- CLI has model/agent score comparison and statistical primitives. Five 20-game-trained candidates plus baselines have a 10,000-game shared-seed matrix at 94024–104023; five 50-game-trained candidates plus baselines have a separate 10,000-game matrix at 104024–114023. Each matrix retains paired comparisons and manifests. Each training corpus/model has one fit, so neither comparison is confirmatory; repeated fits, predeclared selection, and an independent final test remain absent.

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

- A confirmatory model-selection study still needs an adequate labeled corpus, repeated fits, and a predeclared held-out protocol. This exploratory matrix is not that study. Retain identical environment settings, per-game outcomes, and input manifests for every candidate.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
