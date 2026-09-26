# Plan 04 — Rust-Native AutoML Framework Contribution: the repository status is explicit and evidence based

> **Status: PARTIAL.** A source-backed architecture audit and an initial three-dataset/five-model diagnostic are recorded; matched comparisons, resource measurements, broader reproducibility, and independent replication remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework contribution.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The source architecture and API limitations are recorded in [04-framework-architecture.md](04-framework-architecture.md). An initial diagnostic now evaluates three named standard datasets and five AutoML models; its retained report documents one unresolved Wine KNN repeatability/serialization failure. Matched framework comparisons, resource evidence, and broader reproducibility remain pending.

### Architecture audit result

The audit found concrete training, preprocessing, optimization, CV, inference, and JSON persistence APIs. It also confirmed that the pieces are not one automatically composed pipeline: root integration loads the CSV into a Polars `DataFrame`, provides game-group split/CV orchestration, and invokes `TrainEngine`; root does not currently use `DataPreprocessor` or `InferenceEngine`, and the `HyperOptX` objective callback is not wired into the training command. `TrainEngine::fit` uses its own seeded row split and does not call cross-validation or consume `cv_folds`. The root therefore performs grouped CV in a wrapper and then separately fits the final model. Seed controls are component-level, and explicit artifact schema migration was not found. Details and source paths are in the architecture record.

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

Ticket #034 aligned the root encoder, policy input, collector, and CSV schema with Plan 00's canonical 17 values. The former 27-value strategic-feature vector is excluded from core training; any use requires a separately scoped study.

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

### Ticket-specific completion checklist

- [x] Module/data-flow map and source-backed data/ownership contracts recorded.
- [x] Configuration, capability, seed, parallelism, failure, persistence, and API-surface limitations audited.
- [x] Initial diagnostic on named standard tabular datasets with retained sources, hashes, splits, predictions, models, and metrics; broader evaluation remains pending.
- [ ] Fixed-configuration, established-framework, manual-selection, and comparable Rust baselines run under matched budgets.
- [x] Focused RandomForest save/load equivalence smoke passes 20/20 repeated process runs on the published AutoML fix.
- [x] Same-seed synthetic RandomForest refit check passes 20/20 process runs, with 20 refits compared per run.
- [ ] Runtime, memory, broader multi-seed/dataset reproducibility, and CLI/library equivalence evidence collected.
- [ ] Independent replication or validation completed.

The original 2026-09-24 root smoke failed once in 20 repetitions. Follow-up checks isolated three issues in the pinned AutoML checkout. First, `DecisionTree::compute_leaf_value` used randomized `HashMap` iteration to choose among tied classes; leaf ties now select the lowest class deterministically. Second, a structural comparison showed that JSON load changed serialized RandomForest floats (the first observed difference was `feature_importances[21]`, from `0.11877923958625117` to `0.11877923958625115`); enabling `serde_json/float_roundtrip` restored exact model state. Third, a new same-seed refit check failed before the split fix because `TrainEngine::stratified_split` appended class rows in `HashMap` iteration order; class groups now use a `BTreeMap`, so fixed-seed fits receive a stable row order. The fixes are published on AutoML branch `feature/deterministic-training-serialization` and pinned by the root at `88a86bf44a0cb03664931f7ef15201b95fa11255`.

Validation after the patch: the leaf-tie regression passed; the same-seed synthetic refit smoke passed in 20/20 process runs with 20 refits compared per run; and the RandomForest save/load smoke passed 20/20 process runs with exact model-state and prediction checks. The full AutoML library suite passed 710/710. This is focused synthetic evidence; standard datasets, matched baselines, resource measurements, broader seed/dataset behavior, API/CLI parity, and independent replication remain outstanding.

## Open questions

- **The evidence remains bounded by the initial diagnostic.** Three standard datasets and five AutoML models have retained fixed-split results, but the Wine KNN repeatability failure is unresolved. Matched-baseline, resource, broader seed/dataset, CLI/library equivalence, and replication results remain absent. Further experiments require a declared compute budget; retain configurations, seeds, dependency versions, raw metrics, and analysis artifacts.

## Later

- **Complete the framework-validation program in `plans/07-Benchmarking/03-Comparison/04-framework-validation.md`.** Add matched baselines, resource measurements, broader reproducibility checks, and CLI/library equivalence under a declared budget; return here to update contribution conclusions after that evidence is available.
