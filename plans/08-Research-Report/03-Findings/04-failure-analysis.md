# Plan 04 — Failure Analysis: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Failure categories are prospective; only capability and baseline artifacts provide current observations.

**Goal:** State the current implementation and evidence boundary for failure analysis.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is primarily a failure-analysis protocol; no trained-policy failure study has run.** A historical framework failure is recorded: AutoML `88a86bf` produced KNN/ExtraTrees nondeterminism and a Wine KNN save/load mismatch. AutoML `82d8483` fixes deterministic tie handling; the repeated matrix then matched 15/15 predictions and save/load outputs. See `reports/framework_validation/README.md`.

> **Note:** This section anticipates potential failures and defines response protocols. Probability estimates are illustrative, not empirically determined. No trained-policy failure data exists yet.

## 1. Categories of Failure

| Failure Mode | Cause | Detection | Mitigation |
|-------------|-------|-----------|------------|
| Training or fit failure | Model/configuration/data issue | Fit returns an error or produces invalid output | Retain error/configuration and diagnose within supported workflow |
| Weak held-out policy outcome | Model, data, labels, or evaluation protocol | Measured score/uncertainty under a declared study | Report outcome and investigate only within the scoped design |
| automl API mismatch | Hypothesized API doesn't exist | Runtime errors | Verify API against automl source code |
| HyperOptX unavailable | Hyperparameter search doesn't work | Training fails | Use default hyperparameters, log issue |
| Model type unsupported | Hypothesized model not available | ModelType enum missing | Use available models, log gap |
| Cross-validation unavailable | Temporal splitting not supported | Validation fails | Use simple holdout, log gap |

## 2. Null Results Analysis

### 3.1 Scenario: Policy Does Not Improve on a Measured Baseline

If no automl model significantly beats the locally measured heuristic baseline, the following analysis will be conducted:

**Possible causes:**
1. The canonical 17-value state may omit policy-relevant information; no sufficiency claim is established
2. Model capacity insufficient — the available models in automl may be too simple
3. Training data quantity and coverage may be insufficient; no fixed sample count is treated as adequate
4. Label quality insufficient — rollout-based labels may be noisy
5. Game complexity — 2048 may be too complex for supervised learning with the available model types

**Response:**
1. Expand feature engineering (add more features)
2. Revisit the question and scope; do not add excluded methods without a documented scope change
3. Increase training data
4. Investigate reinforcement learning as an alternative
5. Publish null results — this is a valid scientific finding

### 3.2 Scenario: All Models Perform Similarly

If all models produce similar mean scores (no significant differences):

**Possible causes:**
1. The candidates use the same fixed input and may have similar measured outcomes
2. Model capacity ceiling — the available models all reach similar performance limits
3. Finite observed outcomes do not establish a game ceiling

**Response:**
1. Report uncertainty; equal-looking means do not show equal feature importance
2. Try deeper/more complex models if available
3. Avoid inferring a game ceiling from a finite model comparison
4. Consider alternative feature engineering approaches

### 3.3 Scenario: Results Are Not Reproducible

If results vary significantly across seeds:

**Possible causes:**
1. High variance in game outcomes — 2048 has high stochasticity
2. Overfitting to specific game instances — model performs well on specific seeds but poorly on others
3. The study may lack precision under its actual variance and dependence structure

**Response:**
1. Revisit sample size only after a precision/power rationale using the study unit and dependence structure
2. Use multi-seed validation
3. Report confidence intervals, not just point estimates
4. Investigate whether the model generalizes to unseen game states

### 3.4 Scenario: automl Framework Is Inadequate

If the automl framework lacks required capabilities:

**Response:**
1. Document all missing capabilities in the verification checklist
2. Keep core model training on the AutoML framework; report limitations under canonical scope
3. Report the capability gap as a framework finding while retaining the declared research scope
4. State which downstream 2048 conclusions the missing capability prevents

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

Do not assign probabilities from these categories without observed data and an explicit model. Track concrete failure counts, configurations, and logs instead.

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
- Comparison method and assumptions: {method}
- adjusted p-value (if applicable): {p_value}
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

- This file remains a prospective protocol for application failures. The framework nondeterminism defect and its fixed-version rerun are retained in the framework-validation report; no trained-policy failure log exists. Probability estimates were removed and responses are constrained by project scope.

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

- **No systematic failure log exists yet.** Record failures with configuration, seeds, dependency state, diagnostic output, and recovery decisions when the planned studies run.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
