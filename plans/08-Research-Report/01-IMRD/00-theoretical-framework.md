# Plan 00 — Theoretical Framework: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** The policy-learning formulation is outlined; several formal claims are unsound or unverified and are excluded pending review.

**Goal:** State the current implementation and evidence boundary for theoretical framework.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan is partial.** The supervised-policy and framework architecture descriptions are conceptual. The appendix contains incorrect board-bound and information-theory arguments; those are not results and require mathematical review before use.

> **Note:** Core theory concerns AutoML system architecture, reproducible pipeline composition, validation, model selection, and resource-aware optimization. The fully observed MDP and supervised-policy formulation support the 2048 case study. PSPACE, Markov-blanket, and unverified PAC claims are not core contributions.

## 1. Rust-Native AutoML Architecture

The framework is modeled as a typed pipeline of data ingestion, preprocessing, task specification, model selection, validation, hyperparameter optimization, inference, serialization, and experiment control. Each stage has an explicit input/output contract, configuration boundary, failure mode, and reproducibility requirement.

The theoretical questions are architectural: how pipeline components compose, how validation boundaries prevent leakage, how search procedures consume resource budgets, how model artifacts remain portable, and how deterministic seeds interact with parallel execution. These claims are evaluated through design evidence, implementation tests, matched benchmarks, and failure analysis rather than unsupported universal theorems.

## 2. 2048 Case-Study Formulation

### 2.1 Problem as a Stochastic Sequential Decision Process

The observed 4×4 board is modeled as a fully observed stochastic decision process (MDP) `(S, A, T, R, γ)`. There are four actions (0: Up, 1: Down, 2: Left, 3: Right); `S` is the set of representable board configurations; `T` applies slide/merge then a random empty-cell spawn; `R` is merge-score increment; and `γ=1` for undiscounted episodic score. The implementation defaults to probability 0.1 for spawning a 4. Source inspection alone does not prove broader mathematical assumptions.

**Maps to code:** action labels map to `TaskType::MultiClassification`; game transitions and scoring are implemented in `src/game_engine/mod.rs`; feature encoding is in `src/state.rs`. Rollout relabeling defaults to 100 simulations per legal action. This produces heuristic labels from rollout means, not an optimal action `a*`.

The board-value domain and theoretical maximum-tile claim are not established by this plan. The appendix proof is invalid and should not be cited. The simulator stores tile values in `u32`; state encoding divides cells by 32768 without clipping, so encoded values can exceed one. That normalization scale is not a maximum-tile claim.

**Value/Bellman:**

```
V^π(s)=E_π[Σ_{k=0}^{T-t} γ^k r_{t+k} | s_t=s]
π*(s) ∈ argmax_a E[r_t + γV*(s_{t+1}) | s_t=s, a_t=a]
```

### 2.2 Learning as Supervised Function Approximation

Model `M_θ: f(s)∈ℝ^17 → Δ^3` (four class probabilities) defines a policy by selecting among legal actions. Training labels are rollout-mean proxies, not proven optimal actions. Convex-optimization convergence rates do not apply to the heterogeneous tree models used here. Learning curves are not yet a completed study.

## 3. Optional Statistical Learning Context

VC/PAC bounds may be used as general background, but they do not directly establish generalization for the heterogeneous tree-based models, AutoML search procedure, rollout labels, or downstream game score. No numerical PAC guarantee is claimed from the 2048 experiments. Validation and held-out evaluation provide the empirical generalization evidence.

## 4. Optional Information-Theoretic Context

The previous entropy calculation assumed a finite 17-value cell alphabet and a fixed 50-spawn episode. Those assumptions are not justified by the implementation contract; remove the numeric entropy and variance rationale unless a valid state domain and stopping distribution are established.

## 5. Excluded Speculative Theory

PSPACE-hardness, Markov-blanket sufficiency, and feature-sufficiency claims are excluded from the core thesis unless separately proven with valid assumptions. They must not be presented as established theory or used to justify the framework contribution.

## 6. Summary

Architecture documentation and capability checks exist, but matched framework benchmarks, resource measurements, and broad reproducibility evidence remain pending. The 2048 formulation supplies application context; the theoretical appendix does not establish formal bounds.

## Implementation Record

- The code implements board transitions, score tracking, action selection, and the canonical 17-value supervised input (16 board cells plus current score). Cell values use a 32768 scale and are not clipped at one; this scale is not a theoretical bound on tile values. This outline is not a verified formal analysis; invalid board-bound proofs and unsupported entropy claims must not be used. The visible-board formulation is an MDP description, not a POMDP claim.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
2. `grep -q '^# Plan 00 — ' plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/00-theoretical-framework.md` exits 0.

## Open questions

- **Formal results remain bounded by reviewed assumptions.** Correct the mathematical appendix and verify citations before publication. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
