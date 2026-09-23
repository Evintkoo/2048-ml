# Theoretical Framework — Rust-Native AutoML Architecture with 2048 Case Study

> **Note:** Core theory concerns AutoML system architecture, reproducible pipeline composition, validation, model selection, and resource-aware optimization. The POMDP and supervised-policy formulation support the 2048 case study. PSPACE, Markov-blanket, and unverified PAC claims are not core contributions.

## 1. Rust-Native AutoML Architecture

The framework is modeled as a typed pipeline of data ingestion, preprocessing, task specification, model selection, validation, hyperparameter optimization, inference, serialization, and experiment control. Each stage has an explicit input/output contract, configuration boundary, failure mode, and reproducibility requirement.

The theoretical questions are architectural: how pipeline components compose, how validation boundaries prevent leakage, how search procedures consume resource budgets, how model artifacts remain portable, and how deterministic seeds interact with parallel execution. These claims are evaluated through design evidence, implementation tests, matched benchmarks, and failure analysis rather than unsupported universal theorems.

## 2. 2048 Case-Study Formulation

### 2.1 Problem as a Stochastic Sequential Decision Process

2048 on 4×4 is finite-horizon POMDP `(S, A, T, R, Ω, O, γ)` with `|A|=4` (0:Up,1:Down,2:Left,3:Right), `S = {0,2,4,...,32768}^16` (16 cells, ≤17 states each), `T` = deterministic slide+merge then stochastic spawn (2 p0.9 /4 p0.1 in uniform empty), `R` = merge score sum, `γ=1`, `Ω=O=S` (board observed; stochasticity is spawn).

**Maps to code:** `S` → 27-dim `f(s)` row in polars; `A` → `TaskType::MultiClassification` labels 0–3; `T/R` → `game_engine.rs` + `score.rs`; labels `a*` from rollout (100 sims/action) in `label_generation.rs`. No separate 8×8 formalism.

**Theorem 1 (Board Capacity, proven).** Max tile = `2^15=32768` (16 cells; `2^16` needs 17). See `04-Appendix/04-mathematical-formulation.md` Theorem 2. **Score bound:** `S_max ≤131072` loose upper bound (same appendix Theorem 1) — not used as gate.

**Value/Bellman:**

```
V^π(s)=E_π[Σ R_{t+k}|s_t=s],  V^π(s)=R(s,π(s))+E_{P(s'|s,π(s))}[V^π(s')],  π*=argmax V^π(s0)
```

### 2.2 Learning as Supervised Function Approximation

Model `M_θ: f(s)∈ℝ^27 → Δ^3` (simplex over 4 actions): `π_θ(a|s)=M_θ(f(s))[a]`. Loss `L(θ)=-E_{(s,a*)~D}[log π_θ(a*|s)]` where `a*` is rollout-optimal. Convergence claims (convex `O(1/t)`) **do not apply** to tree ensembles; empirically measured via learning curves (score vs epoch, 100 val games/epoch).

## 3. Optional Statistical Learning Context

VC/PAC bounds may be used as general background, but they do not directly establish generalization for the heterogeneous tree-based models, AutoML search procedure, rollout labels, or downstream game score. No numerical PAC guarantee is claimed from the 2048 experiments. Validation and held-out evaluation provide the empirical generalization evidence.

## 4. Optional Information-Theoretic Context

Board entropy `H(board)≤log2(17^16)≈65.4 bits`; spawn `H(spawn)≈0.469 bits` per spawn → ~23.5 bits/game (50 spawns). Justifies stochastic variance in §3 CI width.

## 5. Excluded Speculative Theory

PSPACE-hardness, Markov-blanket sufficiency, and feature-sufficiency claims are excluded from the core thesis unless separately proven with valid assumptions. They must not be presented as established theory or used to justify the framework contribution.

## 6. Summary

The core framework claims are supported by architecture documentation, acceptance tests, matched benchmarks, resource measurements, and reproducibility evidence. The 2048 formulation supplies the application context; optional learning-theory bounds do not replace empirical validation.
