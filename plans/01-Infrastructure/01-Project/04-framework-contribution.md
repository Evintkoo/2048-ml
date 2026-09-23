# Plan 04 — Rust-Native AutoML Framework Contribution: the repository status is explicit and evidence based

> **Status: PARTIAL.** Contribution scope and evaluation dimensions are documented; framework architecture comparisons and standard-dataset/resource evidence remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework contribution.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Contribution scope and evaluation dimensions are documented; framework architecture comparisons and standard-dataset/resource evidence remain pending.

## 1. Primary Contribution

The primary research contribution is the design, implementation, and empirical validation of a Rust-native AutoML architecture. The 2048 ML system is the principal implementation and case study used to exercise the framework in a nontrivial stochastic policy-learning workflow.

The thesis must distinguish framework novelty from application novelty. Reimplementing known algorithms in Rust is an engineering contribution unless the architecture, optimization procedure, systems integration, reproducibility mechanism, or empirical trade-off is shown to provide a defensible improvement or new insight.

## 2. Architecture Questions

1. How are data ingestion, preprocessing, model training, validation, optimization, inference, and persistence composed?
2. Which Rust-native data contracts make the pipeline type-safe and reproducible?
3. How are model types and task types exposed consistently through the library and CLI?
4. How does the architecture manage seeds, parallelism, resource budgets, and failure recovery?
5. What trade-offs exist between Rust-native execution, ecosystem coverage, implementation complexity, and interoperability?

## 3. Required Architecture Evidence

- Module and dependency architecture diagram.
- Data-flow and ownership contracts.
- Configuration schema and versioning rules.
- Model and task capability matrix.
- Error-handling and failure-recovery policy.
- Seed and determinism design.
- Serialization and backward-compatibility policy.
- Resource-budget and parallel-execution policy.
- CLI/library/API equivalence tests.

## 4. Evaluation Dimensions

The framework is evaluated independently of 2048 using standard tabular tasks:

- Predictive quality.
- Training and inference time.
- Memory consumption.
- Hyperparameter-search efficiency.
- Reproducibility across repeated runs.
- Failure rate and diagnostics.
- Artifact portability and model reload correctness.
- Ease of configuration and experiment automation.

## 5. Baseline Categories

Use appropriately matched comparisons with:

- A simple fixed-configuration implementation.
- Established Python AutoML or tabular ML tooling.
- Established Rust ML libraries where comparable functionality exists.
- A manual model-selection baseline.

Comparisons must use documented datasets, splits, hardware, dependency versions, and search budgets. The 2048 score is not a substitute for framework evaluation.

## 6. Case-Study Role of 2048

2048 tests whether the architecture remains usable when the framework is integrated with:

- A custom stochastic environment.
- Sequential trajectory data.
- Rollout-generated labels.
- Four-class action prediction.
- Repeated simulation and policy evaluation.

The case study provides application evidence and exposes framework limitations; it does not alone establish general AutoML superiority.

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
2. `grep -q '^# Plan 04 — ' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/01-Project/04-framework-contribution.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Contribution scope and evaluation dimensions are documented; framework architecture comparisons and standard-dataset/resource evidence remain pending. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
