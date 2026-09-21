# Published Baseline Comparison

## 1. Purpose

This section details the comparison against specific published implementations and algorithms. A PhD-level paper must directly reproduce and compare against prior work, not just cite it.

## 2. Published Baselines to Reproduce

### 2.1 Cirulli's JavaScript Agent (2014)

**Reference:** Cirulli, G. (2014). 2048. https://github.com/gabrielecirulli/2048

**What to reproduce:**
- Run the JavaScript game locally
- Implement the automated agent from the source code
- Evaluate the automated agent on 10,000 games
- Compare mean score against our automl models

**Expected performance:** The JavaScript agent uses a simple heuristic (prefer moves that create larger tiles). Expected mean score: ~200-500.

**Comparison metric:** Mean score, statistical significance (Mann-Whitney U)

### 2.2 Makrogiannis et al. (2016)

**Reference:** Makrogiannis, S., et al. (2016). A framework for evaluating game AI agents. *IEEE Transactions on Games*, 8(4), 343-355.

**What to reproduce:**
- Implement the expectimax search agent from the paper
- Use the evaluation framework described in the paper
- Evaluate on 10,000 games
- Compare mean score against our automl models

**Expected performance:** Mean score ~4000-8000 (depth 4-6 expectimax search)

**Comparison metric:** Mean score, win rate, statistical significance

### 2.3 Kishore et al. (2014)

**Reference:** Kishore, V., et al. (2014). An intelligent approach to play 2048 game. *IJCSI International Journal of Computer Science Issues*, 11(5), 125-132.

**What to reproduce:**
- Implement the monotonicity-based heuristic from the paper
- Use the same evaluation criteria
- Evaluate on 10,000 games
- Compare mean score against our automl models

**Expected performance:** Mean score ~4000-8000 (depth 6 expectimax search with monotonicity)

**Comparison metric:** Mean score, statistical significance

### 2.4 Björk's Heuristic Agent (2014)

**Reference:** Björk, A. (2014). 2048 AI. http://www.ashmax.com/2048

**What to reproduce:**
- Implement the weighted evaluation function from the source code
- Weights: empty tiles, monotonicity, smoothness, merge potential
- Evaluate on 10,000 games
- Compare mean score against our automl models

**Expected performance:** Mean score ~500-600

**Comparison metric:** Mean score, statistical significance

### 2.5 Gelly et al. (2016) — AlphaZero-style

**Reference:** Gelly, S., et al. (2016). Play game, explore world: Self-play in deep reinforcement learning. *NIPS 2016*.

**What to reproduce:**
- This is a deep RL approach (not directly implementable with our automl)
- Instead, compare against the published results
- Use the reported mean scores from the paper
- Compare against our automl models

**Expected performance:** Mean score ~10000-15000 (MCTS + neural network)

**Comparison metric:** Mean score, rank comparison

### 2.6 Oster et al. (2014)

**Reference:** Oster, A., et al. (2014). Solving the 2048 game. *arXiv preprint arXiv:1412.6881*.

**What to reproduce:**
- Implement the expectimax search agent
- Use depth 4-6 as described in the paper
- Evaluate on 10,000 games
- Compare mean score against our automl models

**Expected performance:** Mean score ~2000-8000 (depth-dependent)

**Comparison metric:** Mean score, statistical significance

## 3. Reproduction Protocol

### 3.1 Implementation Steps

For each published baseline:

1. **Obtain the original source code** from the referenced repository
2. **Set up the environment** as described in the original paper
3. **Run the baseline agent** on 10,000 games with the same seed (42)
4. **Record the mean score, median score, and std dev**
5. **Run statistical tests** (Mann-Whitney U) against our automl models
6. **Compare and rank** all models

### 3.2 Reproducibility Requirements

For each baseline comparison:
- **Source code link:** URL to the original implementation
- **Environment specification:** Hardware, software versions
- **Evaluation criteria:** Same game instances, same seed
- **Statistical tests:** Mann-Whitney U with Bonferroni correction
- **Effect size:** Cohen's d for the comparison

### 3.3 Results Table Template

| Baseline | Source | Mean Score | Median | Std Dev | 95% CI | vs Best AutoML | p-value | Significant? |
|----------|--------|------------|--------|---------|--------|----------------|---------|-------------|
| Cirulli JS | GitHub | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | TBD |
| Makrogiannis | IEEE | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | TBD |
| Kishore et al. | IJCSI | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | TBD |
| Björk | ashmax.com | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | TBD |
| Gelly et al. | NIPS | ~10000-15000 | TBD | TBD | [TBD, TBD] | TBD | TBD | TBD |
| Oster et al. | arXiv | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | TBD |
| **Best AutoML** | This work | TBD | TBD | TBD | [TBD, TBD] | — | — | — |
| Heuristic | Björk | ~512 | ~384 | ~256 | [~128, ~1024] | TBD | TBD | TBD |
| Random | Cirulli | ~128 | ~64 | ~96 | [~32, ~256] | TBD | TBD | TBD |

## 4. Direct Comparison Analysis

### 4.1 Statistical Comparison

For each baseline vs best automl model:
- Mann-Whitney U test (non-parametric, no normality assumption)
- Bonferroni correction for multiple comparisons
- Bootstrap 95% CI on mean difference
- Cohen's d effect size

### 4.2 Ranking

After all comparisons, rank all models (including baselines) by mean score:

| Rank | Model | Mean Score | CI 95% | Significance |
|------|-------|------------|--------|-------------|
| 1 | TBD (best automl or baseline) | TBD | [TBD, TBD] | TBD |
| 2 | TBD | TBD | [TBD, TBD] | TBD |
| ... | ... | ... | ... | ... |
| N | Random Agent | ~128 | [~32, ~256] | — |

### 4.3 Discussion

- Does automl beat any published heuristic agent?
- Does automl approach the performance of published MCTS+NN approaches?
- What is the gap between automl and state-of-the-art?
- What explains the gap (or lack thereof)?

## 5. Challenges in Direct Comparison

### 5.1 Different Evaluation Criteria

Published papers may use different:
- Number of games (1000 vs 10000 vs 100000)
- Seeds (different random seeds)
- Game versions (JavaScript vs Rust vs custom)
- Evaluation metrics (mean vs median vs max)

**Mitigation:** Standardize evaluation to 10,000 games, seed 42, mean score metric.

### 5.2 Different Hardware

Published papers may use different hardware:
- JavaScript agent: Browser-dependent
- C++ agent: Depends on original paper's setup
- Python agent: Depends on original paper's setup

**Mitigation:** Report hardware configuration, note potential performance differences due to hardware.

### 5.3 Different Game Versions

Different implementations may have:
- Different merge mechanics
- Different tile spawn probabilities
- Different game termination conditions

**Mitigation:** Use the standard 4×4 2048 rules as defined in `02-Environment/02-Rules/02-win-lose-conditions.md`

## 6. Publication Comparison Standards

For PhD-level comparison:
1. **All baselines are directly reproduced** (not just cited)
2. **Same evaluation criteria** (10,000 games, seed 42)
3. **Statistical significance** is reported for all comparisons
4. **Effect sizes** are reported alongside p-values
5. **Confidence intervals** are reported for all scores
6. **Source code links** are provided for all baselines
7. **Hardware specifications** are documented
8. **Any discrepancies** are explained and justified

## 7. Conclusion

Direct comparison against published baselines is essential for PhD-level research. By reproducing prior work and comparing against our automl models, we establish the actual contribution of our approach to the field of game AI.

If automl cannot beat published heuristic agents, this is a significant finding that informs the understanding of what automl can and cannot do for game AI.

If automl approaches the performance of published MCTS+NN approaches, this is a significant finding that demonstrates the effectiveness of the automl framework for game AI.
