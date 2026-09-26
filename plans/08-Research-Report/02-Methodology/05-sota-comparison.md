# Plan 05 — State-of-the-Art Comparison: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Random and heuristic baselines and model runners exist; no trained-policy comparison or verified external-agent reproduction is complete.

**Goal:** State the current implementation and evidence boundary for state-of-the-art comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is a comparison plan, not a ranking.** Local baseline commands exist, but trained-policy results are absent. Published methods and their quoted score ranges remain unverified and are not measured baselines.

## 1. Purpose

This section frames possible context against methods in game AI, heuristic search, and automated machine learning. A fair empirical comparison requires protocol-matched outcomes and resource measurements. No broad superiority claim follows from a game-score comparison; external agents remain contextual unless reproduced under a controlled protocol.

## 2. Baseline Methods

### 2.1 Random Agent

**Description:** Selects actions uniformly at random from available moves.

**Performance:** Measure under the same simulator and declared seed protocol; no assumed score is used.

**Role in Comparison:** Reference policy; a measured difference supports only the tested application conditions.

### 2.2 Heuristic Agent

**Description:** Uses a weighted evaluation function combining empty tiles, monotonicity, smoothness, and merge potential. The evaluation function is:
```
score = w1 × empty_count + w2 × monotonicity + w3 × smoothness + w4 × merge_potential
```
where weights are tuned empirically (Björk, 2014; Kishore et al., 2014).

**Performance:** Measure locally; prior score estimates and source attribution are not verified.

**Role in Comparison:** Candidate practical baseline, with rule, action, and evaluation protocol fully recorded.

### 2.3 Expectimax Search — Case-Study Comparison Candidate

**Description:** Depth-limited expectimax (depth 4-6, chance node). Include at least one declared non-AutoML baseline where implementation and budget can be controlled. If not reproduced, report it as contextual literature rather than an empirical comparison. Literature scores are not substituted for measured results.

### 2.4 Deep Reinforcement Learning — Context or Future Work

DQN/policy-gradient via self-play. Not part of the core AutoML training implementation. Include only as a comparison baseline if the implementation, compute budget, and evaluation protocol can be made comparable; otherwise retain as literature context and future work.

### 2.5 Monte Carlo Tree Search (MCTS+NN) — Context or Future Work

MCTS+NN (Gelly et al. 2016). Not part of the core AutoML architecture. Include only if a fair measured comparison is feasible; otherwise mark as literature context and do not rank it with measured results.

## 3. Comparison Framework

### 3.1 Ranking Methodology

Rank only after declaring the evaluation sample, seed roles, trained-model repetitions, experimental unit, and comparison family. Report mean and distributional summaries with uncertainty. Pairwise helpers are available, but their assumptions and dependence limits must be addressed. No universal game-count or effect-size winner threshold is established.

### 3.2 Comparison Metrics

| Metric | Description | Used For |
|--------|-------------|----------|
| Mean Score | Average across all games | Primary ranking |
| Median Score | Median performance | Robustness check |
| Score Distribution | Available summary fields include median, p90, p99, min, max | Descriptive analysis |
| Threshold Counts | Counts above configured score thresholds | Descriptive analysis; no validated heuristic threshold |
| Training Time | Time to train model | Efficiency comparison |
| Inference Speed | Games per second | Practical applicability |

### 3.3 Candidate Comparison Matrix (No Expected Scores)

**Note:** These rows define possible comparisons, not results. Local random/heuristic agents are measurable candidates. External search/learning methods require verified implementations, source pins, comparable budgets, and a declared protocol.

| Candidate | Status |
|-----------|--------|
| Supported AutoML model candidates | Training runners exist; plan-scale scores pending |
| Local heuristic and random policies | Baseline runners exist; measurement artifacts exist for action-frequency work; full score comparison protocol pending |
| Expectimax, DQN, MCTS | Not reproduced; retain as literature context only until verified and implemented |

## 4. Detailed Comparison Tables

### 4.1 Model Performance Comparison (Template Only)

| Candidate | Mean | Median | SD | CI | Rank | Evidence |
|------------|------|--------|----|----|------|----------|
| Populate from measured runs | TBD | TBD | TBD | TBD | TBD | Manifest and protocol required |

The supported four-class integration candidates are RandomForest, ExtraTrees, AdaBoost, KNN, and NaiveBayes. Do not list unsupported models as runnable candidates.

### 4.2 Computational Efficiency Comparison

Record runtime, hardware, memory method, model configuration, and evaluation throughput for each measured candidate. No efficiency values are available for this comparison.

### 4.3 Statistical Comparison

Use the comparison CLI only after selecting a valid experimental unit and considering paired seed/game dependence. It emits pairwise p-values, Holm adjustment, bootstrap mean-difference intervals, and Cohen's d. No global multi-group test is implemented.

## 5. Analysis of Results

### 5.1 Interpretation of Rankings

The ranking provides insight into:
1. **Which model types are most effective** for the 2048 game
2. **Whether AutoML can compete with heuristic search** methods
3. **The trade-off between computational efficiency and performance**
4. **The value of feature engineering** over raw board state representation

### 5.2 Practical Implications

- If AutoML significantly exceeds heuristic baseline: AutoML is effective for game AI
- If AutoML is competitive with expectimax: AutoML provides a viable alternative to search-based methods
- If AutoML is inferior to MCTS+NN: AutoML has limitations for complex game AI
- If AutoML is competitive with expectimax: AutoML is practical for resource-constrained environments

### 5.3 Limitations of Comparison

1. **Different paradigms**: AutoML (supervised classification) vs. search (planning) vs. RL (trial-and-error) are fundamentally different approaches
2. **Computational asymmetry**: Heuristic and search methods are evaluated on identical hardware, while deep learning methods may require GPUs
3. **Training data asymmetry**: AutoML requires labeled training data; search methods do not
4. **Generalization gap**: Methods trained on one seed may not generalize to other seeds

## 6. Baseline Evidence

No literature score estimate in this document is verified as a compatible baseline. Generate local baseline measurements under the same simulator rules and retain their CSV files and JSON manifests. Do not use an external score tier as an acceptance gate.

## 7. Conclusion

A future comparison can describe the tested methods and conditions. Until those experiments are complete, it provides no ranking or evidence of state-of-the-art performance. Relevant questions include:
1. **Where AutoML stands** relative to alternative approaches
2. **What AutoML can and cannot do** compared to search-based methods
3. **The trade-offs** between different paradigms (supervised learning vs. planning vs. RL)
4. **The practical value** of AutoML for game AI applications

This comparison sets the methodological standard for the entire research and provides context for interpreting the results.

## Implementation Record

- Random, heuristic, and model benchmark commands exist; no trained-policy model comparison has been completed. Quoted external score estimates and agent implementations are unverified and excluded from empirical claims. The standard-dataset AutoML diagnostic is a separate framework track and provides no 2048 ranking evidence.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
2. `grep -q '^# Plan 05 — ' plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/02-Methodology/05-sota-comparison.md` exits 0.

## Open questions

- **A comparison result remains pending.** Declare matched conditions, candidate support, budget, seeds, and analysis before running; retain raw outputs and manifests.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
