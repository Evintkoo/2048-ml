# State-of-the-Art Comparison

## 1. Purpose

This section compares the proposed AutoML approach against state-of-the-art methods in game AI, heuristic search, and automated machine learning. The comparison is based on mean game score, statistical significance, and computational efficiency.

## 2. Baseline Methods

### 2.1 Random Agent

**Description:** Selects actions uniformly at random from available moves.

**Expected Performance:** Mean score ≈ 128 (based on prior literature).

**Role in Comparison:** Provides the lower bound for model evaluation. Any model significantly outperforming the random agent demonstrates basic competence.

### 2.2 Heuristic Agent

**Description:** Uses a weighted evaluation function combining empty tiles, monotonicity, smoothness, and merge potential. The evaluation function is:
```
score = w1 × empty_count + w2 × monotonicity + w3 × smoothness + w4 × merge_potential
```
where weights are tuned empirically (Björk, 2014; Kishore et al., 2014).

**Expected Performance:** Mean score ≈ 512 (based on prior literature).

**Role in Comparison:** Provides the practical baseline. The AutoML model must significantly exceed this score to demonstrate its value.

### 2.3 Expectimax Search

**Description:** Uses depth-limited expectimax search (depth 4-6) with heuristic evaluation. The stochastic tile spawn is modeled as a chance node with known probabilities.

**Expected Performance:** Mean score ≈ 4000-8000 (based on Kishore et al., 2014).

**Role in Comparison:** Provides the state-of-the-art heuristic approach. The AutoML model must compete with or exceed this approach.

### 2.4 Deep Reinforcement Learning

**Description:** Uses deep Q-networks (DQN) or policy gradient methods trained via self-play or environment interaction.

**Expected Performance:** Variable, typically 2000-10000 mean score.

**Role in Comparison:** Provides the deep learning baseline. The AutoML approach must demonstrate competitive performance without requiring extensive training data or computational resources.

### 2.5 Monte Carlo Tree Search (MCTS)

**Description:** Uses MCTS with neural network evaluation (Gelly et al., 2016). Combines search with learned value functions.

**Expected Performance:** State-of-the-art for game AI, typically 5000-15000 mean score.

**Role in Comparison:** Provides the gold standard for game AI. The AutoML approach is compared against this method to assess its competitiveness.

## 3. Comparison Framework

### 3.1 Ranking Methodology

Models are ranked by mean score across ≥10,000 benchmark games. The ranking is determined as follows:

1. **Compute mean score** for each model across all games
2. **Rank by mean score** (highest = rank 1)
3. **Statistical significance**: The #1 ranked model must be significantly better than #2 (Mann-Whitney U, p < 0.05 after Bonferroni correction)
4. **Bootstrap 95% CI** on mean difference must not include zero
5. **Effect size** (Cohen's d) must be ≥ 0.5 (medium effect)

### 3.2 Comparison Metrics

| Metric | Description | Used For |
|--------|-------------|----------|
| Mean Score | Average across all games | Primary ranking |
| Median Score | Median performance | Robustness check |
| Score Distribution | Percentiles (50th, 90th, 95th, 99th) | Performance analysis |
| Win Rate vs Heuristic | % of games exceeding heuristic | Practical significance |
| Score > Heuristic Rate | % of games exceeding heuristic (~512) | Model quality |
| Training Time | Time to train model | Efficiency comparison |
| Inference Speed | Games per second | Practical applicability |

### 3.3 Expected Results

**Note:** The following ranking is a hypothesis based on prior literature and theoretical analysis. These are NOT results — they are expectations to be validated through experimentation.

| Rank | Method | Expected Mean Score | Significance |
|------|--------|--------------------|--------------|
| 1 | MCTS + Neural Network | ~8000-15000 | Gold standard (hypothesis) |
| 2 | Expectimax Search | ~4000-8000 | State-of-the-art heuristic (hypothesis) |
| 3 | Best AutoML Model | TBD | To be determined |
| 4 | Heuristic Agent | ~512 | Practical baseline |
| 5 | Random Agent | ~128 | Lower bound |

*These are hypotheses based on prior literature. Actual values will be determined after experimentation. The AutoML model's actual performance is unknown and could be anywhere on the scale, including below the heuristic baseline.*

## 4. Detailed Comparison Tables

### 4.1 Model Performance Comparison

| Model Type | Mean Score | Median Score | Std Dev | 95% CI | Win Rate vs Heuristic | Rank |
|------------|------------|--------------|---------|--------|----------------------|------|
| Random Forest | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| Gradient Boosting | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| XGBoost | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| LightGBM | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| ExtraTrees | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| SVM | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| KNN | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD |
| **Heuristic Baseline** | **~512** | **~384** | **~256** | **[~128, ~1024]** | **—** | **—** |
| **Random Baseline** | **~128** | **~64** | **~96** | **[~32, ~256]** | **—** | **—** |

### 4.2 Computational Efficiency Comparison

| Method | Training Time | Inference Speed | Memory Usage | Setup Complexity |
|--------|--------------|-----------------|--------------|-----------------|
| Random Forest | TBD | TBD games/s | TBD MB | Low |
| Gradient Boosting | TBD | TBD games/s | TBD MB | Medium |
| XGBoost | TBD | TBD games/s | TBD MB | Medium |
| LightGBM | TBD | TBD games/s | TBD MB | Low |
| Expectimax Search | None | TBD games/s | Low | Medium |
| MCTS + NN | High | TBD games/s | High | High |
| Heuristic Agent | None | TBD games/s | Low | Low |

### 4.3 Statistical Significance Comparison

| Comparison | Mann-Whitney U | p-value | Cohen's d | Significant? |
|------------|----------------|---------|-----------|-------------|
| Best AutoML vs Random | TBD | TBD | TBD | TBD |
| Best AutoML vs Heuristic | TBD | TBD | TBD | TBD |
| Best AutoML vs Expectimax | TBD | TBD | TBD | TBD |
| Heuristic vs Random | TBD | TBD | TBD | TBD |

*All comparisons use Bonferroni correction for multiple comparisons.*

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

## 6. State-of-the-Art Benchmarks

### 6.1 Published Benchmarks

Based on the literature survey (Section 00-literature-review.md), the following benchmarks are established:

| Benchmark | Score | Source |
|-----------|-------|--------|
| Random Agent | ~128 | Cirulli (2014) |
| Heuristic Agent | ~512 | Björk (2014) |
| Expectimax (depth 4) | ~2000 | Oster et al. (2014) |
| Expectimax (depth 6) | ~8000 | Kishore et al. (2014) |
| Deep RL (DQN) | ~2000-5000 | Woltman & Sarakiki (2014) |
| MCTS + NN | ~10000+ | Gelly et al. (2016) |

### 6.2 Comparison Against Benchmarks

The proposed AutoML approach will be compared against these benchmarks:
- **Minimum acceptable**: Exceed heuristic baseline (~512)
- **Competitive**: Exceed expectimax depth 4 (~2000)
- **Strong**: Exceed expectimax depth 6 (~8000)
- **State-of-the-art**: Exceed MCTS + NN (~10000+)

## 7. Conclusion

The state-of-the-art comparison provides a rigorous framework for evaluating the AutoML approach against established methods. The comparison is not merely about achieving high scores but about understanding:
1. **Where AutoML stands** relative to alternative approaches
2. **What AutoML can and cannot do** compared to search-based methods
3. **The trade-offs** between different paradigms (supervised learning vs. planning vs. RL)
4. **The practical value** of AutoML for game AI applications

This comparison sets the methodological standard for the entire research and provides context for interpreting the results.
