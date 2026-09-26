# Plan 04 — Discussion: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Interpretation questions are outlined, but neither framework results nor held-out policy results support conclusions yet.

**Goal:** State the current implementation and evidence boundary for discussion.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is an interpretation guide, not a findings discussion.** Its outcome branches remain hypothetical. Several listed tests and score thresholds are not implemented or justified and cannot serve as decision gates.

> **Status: PENDING.** No conclusions drawn. This file defines how the primary Rust-native AutoML framework results and the downstream 2048 case-study results will be interpreted.

## 1. Primary Framework Interpretation

| Framework result | Interpretation |
|------------------|----------------|
| Capability and correctness criteria pass | The implemented architecture supports the declared workflow under the tested conditions |
| Reproducibility fails | The framework cannot support the corresponding reproducibility claim; investigate seed, parallelism, and serialization boundaries |
| Rust framework is faster or more resource-efficient | Report the measured trade-off and matched benchmark conditions; do not generalize beyond tested tasks |
| Framework performs differently from established baselines | Analyze algorithm coverage, preprocessing, search budget, implementation differences, and failure modes |
| Required capability fails | Report the failure as a primary framework result and block the affected 2048 conclusion |

## 2. 2048 Case-Study Interpretation (Tied to H1–H3)

| Hypothesis | Gate | If Pass | If Fail |
|------------|------|---------|---------|
| Policy vs measured baseline | Declare unit and pairing; use available comparison helpers with limitations | Report measured difference and uncertainty | Report no evidence or inconclusive result; do not infer feature insufficiency |
| Multiple model comparison | Pairwise comparisons with multiplicity correction; no global test implemented | Describe supported pairwise evidence | Do not claim all models equivalent from a nonsignificant result |
| Tuning comparison | Matched study and declared budget; no completed evaluation | Report measured quality/resource trade-off | Do not claim defaults sufficient without evidence |
| Feature contribution | Ablation runner and protocol pending | Report measured effects and uncertainty | No feature sufficiency or Markov-blanket conclusion |

All framework gates come from the framework-validation protocol. Application gates come from `02-Methodology/03-hypotheses.md`; feature analysis is exploratory and does not establish a Markov blanket.

## 3. Implications (In-Scope Only)

### 3.1 For automl Framework
- If policy comparisons show improvement under a valid protocol, limit the conclusion to the tested model, dataset, seeds, and game conditions.
- If the comparison is inconclusive, report its uncertainty; do not infer a general ceiling or superiority of alternative methods.
- Tuning claims require a matched quality and resource study; configuration defaults alone establish no result.

### 3.2 For 27-dim Feature Engineering
An ablation study could estimate contribution under its declared training and evaluation setup. It cannot establish feature necessity, sufficiency, or a causal mechanism without appropriate design and assumptions.

### 3.3 For Evaluation Methodology
Application evaluation should report game score separately from framework predictive metrics. Game-clustered or seed-level dependence must be considered before interpreting nominal game-level tests.

## 4. Seed Sensitivity Tie-In (Not Hardware)

Seed matrix remains to be selected and declared. If rankings vary across training/evaluation seeds, treat that as evidence of sensitivity and report it with the experimental unit and uncertainty. Hardware and parallelism should be recorded when they can affect reproducibility or runtime.

## 5. Limitations (In-Scope)

- 4×4 only; 27-dim fixed; supervised only; rollout labels noisy (100 sims/action).
- `automl` model list limited to what `ModelType` actually exposes (verify via `automl/src/training/config.rs`).
- Precision is unknown until observed variance, dependence, and sample size are measured. Prior numerical interval estimates and tail-rate statements are unsupported.

## 6. Unexpected Findings Protocol

Document any: ranking shift across seeds, overfit (train acc >> test mean), feature Δ opposite expectation, invalid-move rate anomaly (see `02-insights.md`). Each gets a dedicated paragraph with exact numbers, not generic prose.

## 7. Conclusions (TBD)

Will state framework-validation outcomes, application winner (or null), F1–F3 and H1–H3 decisions with exact statistics where applicable, and reproducibility claims — only after data.

## 8. Future Work (Scope-Bound, 2 Lines Each — Not Core)

- **Larger boards / n×n theory:** n×n PSPACE claim belongs in Appendix only; not core IMRD.
- **Ensemble/RL/MCTS/DQN reproduction:** Out-of-scope; Appendix optional 15-line extended-baseline note, not required for thesis.
- **Information-theoretic / Markov blanket / PAC refinements:** Conjectures; Appendix if pursued.

## Implementation Record

- Interpretation guidance is written, but no hypotheses have been evaluated. Decision thresholds and predicted outcomes remain unset pending a justified protocol and completed framework and case-study experiments.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/04-discussion.md` exits 0.

## Open questions

- **Interpretations remain pending.** Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
