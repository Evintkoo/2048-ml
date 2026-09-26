# Plan 04 — Rust-Native AutoML Framework Contribution: the repository status is explicit and evidence based

> **Status: PARTIAL.** A source-backed architecture audit and a repeated three-dataset/five-model diagnostic are recorded; matched comparisons, resource measurements, broader reproducibility, and independent replication remain pending.

**Goal:** State the current implementation and evidence boundary for rust-native automl framework contribution.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The source architecture and API limitations are recorded in [04-framework-architecture.md](04-framework-architecture.md). Two diagnostic runs on three named standard datasets and five AutoML models succeeded for all 15 cases and matched predictions exactly in all 15 pairs on pinned AutoML `82d848323eed5e2af86d046d529916c448f2442c`. This revision fixes KNN and ExtraTrees nondeterministic tie handling. Matched framework comparisons, resource evidence, and broader reproducibility remain pending.

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
- [x] Repeated fixed-split diagnostic on pinned AutoML `82d8483`: 15/15 successful cases in each of two runs; exact predictions 15/15; save/load equivalence passes for every case.
- [ ] Runtime, memory, broader multi-seed/dataset reproducibility, and CLI/library equivalence evidence collected.
- [ ] Independent replication or validation completed.

The original 2026-09-24 root smoke failed once in 20 repetitions. Follow-up checks isolated three issues in the earlier AutoML fix: `DecisionTree::compute_leaf_value` used randomized `HashMap` iteration for tied classes; JSON load changed serialized RandomForest floats until `serde_json/float_roundtrip` was enabled; and `TrainEngine::stratified_split` used randomized class grouping before the split. These fixes were published at `88a86bf`. The standard-dataset rerun at that pin exposed two further tie cases: KNN vote ties used randomized `HashMap` iteration, and ExtraTrees selection did not fully specify ties. AutoML commit `82d848323eed5e2af86d046d529916c448f2442c`, published on `fix/deterministic-tie-breaking`, adds deterministic tie rules for those paths.

Validation after the fixes: focused tie tests passed; the same-seed synthetic refit smoke passed in 20/20 process runs with 20 refits compared per run; and the RandomForest save/load smoke passed 20/20 process runs with exact model-state and prediction checks. The full AutoML library suite passes 712/712. On the fixed split, both independent runs at `82d8483` passed all 15 model/dataset cases, matched predictions 15/15, and passed save/load equivalence. This remains bounded evidence: matched baselines, resource measurements, broader multi-seed/multi-dataset behavior, API/CLI parity, and independent replication remain outstanding.

## Open questions

- **The evidence remains bounded by the repeated diagnostic.** Three standard datasets and five AutoML models have retained fixed-split results with matching predictions and save/load equivalence on two runs at `82d8483`. Matched-baseline, resource, broader seed/dataset, CLI/library equivalence, and replication results remain absent. Further experiments require a declared compute budget; retain configurations, seeds, dependency versions, raw metrics, and analysis artifacts.

## Later

- **Complete the framework-validation program in `plans/07-Benchmarking/03-Comparison/04-framework-validation.md`.** Add matched baselines, resource measurements, broader reproducibility checks, and CLI/library equivalence under a declared budget; return here to update contribution conclusions after that evidence is available.
