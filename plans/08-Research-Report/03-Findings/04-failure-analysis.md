# Plan 04 — Failure Analysis: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for failure analysis.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Note:** This section anticipates potential failures and defines response protocols. Probability estimates are illustrative, not empirically determined. No actual failure data exists yet.

## 1. Categories of Failure

| Failure Mode | Cause | Detection | Mitigation |
|-------------|-------|-----------|------------|
| Non-convergence | Learning rate too high/low | Loss curve doesn't stabilize | Adjust learning rate, increase epochs |
| Overfitting | Model too complex for data | Training loss << validation loss | Regularization, early stopping |
| Underfitting | Model too simple | Training loss plateaus high | Increase model complexity, add features |
| NaN loss | Numerical instability | Loss becomes NaN | Reduce learning rate, gradient clipping |
| Slow convergence | Insufficient training data | Loss decreases very slowly | Increase training data, adjust learning rate |
| automl API mismatch | Hypothesized API doesn't exist | Runtime errors | Verify API against automl source code |
| HyperOptX unavailable | Hyperparameter search doesn't work | Training fails | Use default hyperparameters, log issue |
| Model type unsupported | Hypothesized model not available | ModelType enum missing | Use available models, log gap |
| Cross-validation unavailable | Temporal splitting not supported | Validation fails | Use simple holdout, log gap |

## 2. Null Results Analysis

### 3.1 Scenario: No Model Beats Heuristic

If no automl model significantly beats the heuristic baseline (~512 mean score), the following analysis will be conducted:

**Possible causes:**
1. Feature engineering insufficient — the 27-dimensional feature vector may not capture enough game information
2. Model capacity insufficient — the available models in automl may be too simple
3. Training data insufficient — 10,000 samples may not be enough for complex game strategies
4. Label quality insufficient — rollout-based labels may be noisy
5. Game complexity — 2048 may be too complex for supervised learning with the available model types

**Response:**
1. Expand feature engineering (add more features)
2. Try ensemble methods
3. Increase training data
4. Investigate reinforcement learning as an alternative
5. Publish null results — this is a valid scientific finding

### 3.2 Scenario: All Models Perform Similarly

If all models produce similar mean scores (no significant differences):

**Possible causes:**
1. Feature space saturates — all models learn from the same features equally well
2. Model capacity ceiling — the available models all reach similar performance limits
3. Game ceiling — the game itself limits performance regardless of model

**Response:**
1. Analyze feature importance (may all be equally important)
2. Try deeper/more complex models if available
3. Investigate whether the game ceiling has been reached
4. Consider alternative feature engineering approaches

### 3.3 Scenario: Results Are Not Reproducible

If results vary significantly across seeds:

**Possible causes:**
1. High variance in game outcomes — 2048 has high stochasticity
2. Overfitting to specific game instances — model performs well on specific seeds but poorly on others
3. Insufficient sample size — 10,000 games may not be enough to stabilize results

**Response:**
1. Increase games per experiment (50,000+)
2. Use multi-seed validation
3. Report confidence intervals, not just point estimates
4. Investigate whether the model generalizes to unseen game states

### 3.4 Scenario: automl Framework Is Inadequate

If the automl framework lacks required capabilities:

**Response:**
1. Document all missing capabilities in the verification checklist
2. Implement local training fallback using `smartcore`/`linfa` directly (bypassing automl's abstraction)
3. Modify the project goal to "what automl CAN do"
4. Publish the automl capability gap as a separate finding

## 3. Failure Mode Assessment

The following probability estimates are **illustrative and not empirically determined**. They will be revised based on actual experimental outcomes.

| Failure Mode | Estimated Probability | Impact | Severity |
|-------------|----------------------|--------|----------|
| No model beats heuristic | Unknown — to be assessed | High | Critical |
| Results not reproducible | Unknown — to be assessed | High | Critical |
| automl API mismatch | Unknown — to be assessed | Medium | Warning |
| Training doesn't converge | Unknown — to be assessed | Medium | Warning |
| Feature engineering insufficient | Unknown — to be assessed | High | Critical |
| Statistical insignificance | Unknown — to be assessed | Medium | Warning |
| High variance across seeds | Unknown — to be assessed | Medium | Warning |
| Framework limitations | Unknown — to be assessed | Medium | Warning |

**These probabilities are placeholders. Actual probabilities will be assessed after the capability verification gate and initial experiments.**

## 4. Honest Reporting Protocol

All results will be reported honestly, including:

1. **Null results** — If no model beats heuristic, this will be reported
2. **Failed experiments** — If training doesn't converge, this will be documented
3. **Inconclusive results** — If statistical significance is not achieved, this will be stated
4. **Framework limitations** — All missing automl capabilities will be documented
5. **Negative findings** — If feature engineering doesn't help, this will be reported
6. **Unexpected results** — If results contradict hypotheses, this will be analyzed

**The paper will NOT:**
- Selectively report only positive outcomes
- Fabricate or falsify results
- Mislead about statistical significance
- Overstate the importance of findings
- Ignore failures or limitations

## 5. Failure Analysis Template

For each experiment, the following will be reported:

```markdown
## Experiment: {experiment_name}

### Setup
- Model: {model_type}
- Features: {feature_set}
- Seed: {seed}
- Games: {n_games}
- Training time: {time}

### Results
- Mean score: {score}
- Median score: {median}
- Std dev: {std_dev}
- 95% CI: [{ci_lower}, {ci_upper}]

### Statistical Significance
- Mann-Whitney U vs heuristic: {u_value}
- p-value: {p_value}
- Significant? {yes/no}
- Effect size: {cohens_d}

### Failure Analysis
- Did training converge? {yes/no}
- Was result reproducible? {yes/no}
- Did model beat heuristic? {yes/no}
- Were there any errors? {yes/no}

### Limitations
- {list of limitations}

### Honest Assessment
- {what went well, what didn't, what to improve}
```

## 6. Conclusion

Failure analysis is not an afterthought — it is an integral part of the research methodology. By anticipating failures, documenting them, and responding appropriately, the research maintains scientific integrity and provides genuine insights into automl's capabilities for game AI.

Null results are valid scientific findings. If automl cannot beat heuristic baselines for 2048, this is an important finding that advances the understanding of what automl can and cannot do for sequential decision-making problems.

## Implementation Record

- This file is a prospective failure protocol. No study failure log exists yet; listed probability estimates and scenarios are illustrative rather than observed.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/03-Findings/04-failure-analysis.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
