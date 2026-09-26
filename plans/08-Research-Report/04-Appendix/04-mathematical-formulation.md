# Plan 04 — Mathematical Formulation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Implemented process notation is provided; unsupported theorem and complexity claims were withdrawn pending formal review.

**Goal:** State the current implementation and evidence boundary for mathematical formulation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This appendix provides notation, not formal results.** Invalid or unsupported proofs from the earlier draft were removed from the thesis argument.

> This appendix gives a bounded mathematical notation for the implemented case study. It establishes no complexity, optimality, sample-complexity, or information-theoretic theorem.

## 1. Game and Policy Notation

Let `B_t` denote the observed 4×4 board before decision `t`, `A(B_t)` the legal directions, and `S_t` the accumulated score. The simulator applies a legal directional slide and merges equal adjacent tiles, then samples a tile value and empty cell according to the configured spawn process. The process ends when no legal move remains or the configured move limit is reached.

A transition can be represented as

```text
(B_{t+1}, S_{t+1}) ~ P(· | B_t, S_t, a_t)
S_{t+1} = S_t + r(B_t, a_t, B_{t+1})
```

where `r` is the score increment from merges. The code defaults to probability 0.1 for spawning a 4; the probability is configurable. A precise probability kernel and terminal convention should be stated for any formal analysis.

The supervised model receives the canonical 17-value state (16 board cells plus current score) and predicts one of four action classes. The action-selection path masks illegal actions. The training labeler compares finite-simulation mean final scores for legal actions; the resulting label is a noisy decision proxy and is not known to be optimal.

For a finite horizon `T`, an optimal value function would satisfy a Bellman recursion only after the reward, transition kernel, terminal condition, and horizon are fixed:

```text
V_T(b, s) = terminal_value(b, s)
V_t(b, s) = max_{a ∈ A(b)} E[r(b, a, B') + V_{t+1}(B', s')]
```

This project does not compute or validate this optimal value function.

## 2. State and Feature Contract

The canonical training state is the 16 board cells plus current score. These are encoded in the fixed 17-value input; no derived board features, move count, or game history are added. The state scope is documented in `00-scope-and-traceability.md`; the source encoder is `src/state.rs`.

The 17-value representation is not claimed to be a sufficient statistic for optimal action selection. An ablation or predictive association analysis can assess empirical utility under a specified dataset and model, but cannot by itself prove a minimal Markov blanket or optimal-policy sufficiency.

## 3. Claims Explicitly Not Established

The former draft included purported proofs of a maximum score, maximum tile, feature sufficiency, mutual-information sums, PAC sample complexity, generalization gap, bootstrap coverage, board entropy, PSPACE-hardness, and search infeasibility. Those derivations either use invalid reasoning, omit assumptions, cite unverified sources, or do not apply to the implemented model and sampling process. They are withdrawn from the thesis argument.

In particular:

- The simulator stores tile values in `u32`; the feature and CSV validators separately enforce a 32768 scale in relevant paths. The relationship between representable/accepted states and a mathematical maximum is unresolved.
- Rollout-labeled game states are sequential and clustered by game. A standard iid VC bound cannot be applied without specifying a hypothesis class, sampling assumptions, and the effect of model selection.
- A bootstrap interval is an estimator whose coverage depends on the sampling design. The implemented percentile bootstrap does not prove a normal-theory interval or a fixed interval width.
- A finite board alphabet would yield a combinatorial state-count bound only after its tile domain is justified. It would not establish the entropy of the empirical board distribution.
- A branching-factor calculation is not a measured game length or a proof of computational intractability.
- No reduction establishing PSPACE-hardness for this fixed 4×4 game is provided.

## 4. Permitted Mathematical Reporting

Report implementation equations and measured quantities with their scope. For example, a merge of two tiles of value `x` adds `2x` to the score, and the merge event records the resulting tile and position. Describe empirical means, quantiles, and intervals only from retained data and the analysis method that produced them. Do not label a statement a theorem unless its assumptions and full proof have been independently reviewed.

## 5. Open Research Questions

Formal work would require a separate question and validated assumptions: a precise reachable-state domain, a proved score or tile bound, a source-verified complexity result for the exact game variant, or a learning-theory analysis matching the data dependence and model-selection procedure. These questions are not prerequisites for reporting the software implementation or its empirical results.

## Implementation Record

- Replaced invalid/unverified proof catalogue with scope-matched process notation and a list of withdrawn claims. No score/tile bound, PAC guarantee, entropy result, or complexity theorem is asserted.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/04-mathematical-formulation.md` exits 0.

## Open questions

- **Formal claims remain out of scope until independently proved.** Empirical reporting still requires the protocol, raw artifacts, and analysis provenance.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
