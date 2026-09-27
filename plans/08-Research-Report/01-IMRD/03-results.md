# Plan 03 — Results: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** UCI diagnostics and an exploratory same-seed 2048 score comparison are retained; matched framework comparisons and confirmatory policy results remain pending.

**Goal:** State the current implementation and evidence boundary for results.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan remains an output specification, not a completed results report.** UCI fixed-split outcomes and repeatability analyses are retained in `reports/framework_validation/`. An exploratory 10,000-seed comparison of Random, Heuristic, and one fitted pilot policy is retained in `reports/action-frequency/pilot-comparison.md`. It is not the selected five-candidate matrix, and its bootstrap intervals and Cohen's dz use the matched seed pairs. No matched-budget external framework comparison or confirmatory 2048 ranking is available.

> No empirical result or winner is claimed in this document.

## 1. Primary Framework Results

Framework results must report, for each named dataset and configuration:

- Predictive quality using task-appropriate metrics.
- Training and inference time.
- Peak memory and CPU use.
- Hyperparameter-search budget and best-trial trajectory.
- Failure rate and diagnostic category.
- Repeated-run reproducibility.
- Model serialization and reload equivalence.
- CLI/library/API output equivalence.

Initial named-dataset results are reported in `reports/framework_validation/README.md`. Matched external budgets, per-model resource profiles, CLI/API equivalence, and broader repeated-run evidence remain pending.

No 2048 game score can substitute for this table.

## Implementation Record

- The three-dataset/five-model framework matrix is populated for split seeds 42, 2026, and 2027. Same-split repeated processes at seeds 42, 2026, and 2027 matched all 15 prediction sets per seed; save/load predictions matched in each run. The earlier `88a86bf` Wine KNN issue is historical. The 2048 comparison report retains exploratory descriptive outcomes only; the selected-model result table remains unpopulated.

## 2. 2048 Case-Study Winner Protocol (Canonical: `07-Benchmarking/01-Evaluation/01-benchmarking-framework.md`)

Winner protocol, sample size, seed roles, and inferential unit must be declared before confirmatory evaluation. The comparison CLI supports paired exact sign tests with paired-difference bootstrap intervals and Cohen's dz, or unmatched Mann–Whitney tests with independent bootstrap intervals and Cohen's d; it applies Holm adjustment but does not implement a global ranking gate. Baseline scores must be measured locally or supported by verified literature.

## 3. Concrete Schemas

### Illustrative Result Fields (Not an Implemented Schema)

```rust
pub struct IllustrativeGameResult {
    pub model: String,
    pub seed: u64,
    pub game_id: u64,
    pub score: u64,
    pub max_tile: u32,
}
```

Actual game CSVs and JSON manifests are the current benchmark artifacts. A future schema must be based on those outputs, and analysis must declare whether its inferential unit is a game, evaluation seed, or trained-model run.

### Statistical Helpers (Actual Location: `src/evaluation.rs`)

```rust
pub fn mann_whitney_u_pvalue(a: &[u64], b: &[u64]) -> Option<f64>;
pub fn paired_sign_test_pvalue(a: &[u64], b: &[u64]) -> Option<f64>;
pub fn holm_adjust(p: &[f64]) -> Vec<f64>;
```

The helper inventory and its limitations are summarized in the benchmarking analysis tickets.

## 4. Table Shells (Populated by Pipeline, Not Hand-Edited)

### Ranking Table Shell (No Case-Study Ranking Result)

| Model | Mean | Median | SD | 95% CI (bootstrap) | Rank | Training time | Declared pairwise result |
|-------|------|--------|----|---------------------|------|---------------|-----------|
| TBD | TBD | TBD | TBD | [TBD, TBD] | TBD | TBD | p=TBD, d=TBD |

The CLI can summarize and compare game-score files. An exploratory three-policy analysis exists, but no selected-model ranking has been run. Populate only with results from a declared protocol.

### 3.2 Gate Table

| Comparison | Procedure | Adjusted p-value | Effect estimate | Bootstrap CI on difference |
|------------|-----------|------------------|-----------------|-----------------------------|
| Best vs measured baseline | TBD | TBD | TBD | [TBD, TBD] |
| Best vs runner-up | TBD | TBD | TBD | [TBD, TBD] |

### 3.3 Learning-Curve Hooks

Learning-curve metrics are not currently emitted by the training pipeline.

## 5. Visualization Spec (Generated, Not Mocked)

Charts are future outputs; the current CLI writes CSV summaries and JSON manifests.

## 6. Data-Quality Checklist (Automated)

- [ ] Result schema matches actual CSV and manifest
- [ ] Protocol and seed roles are recorded
- [ ] Experimental unit and dependence assumptions are stated
- [ ] Inputs, code revision, dependency versions, and analysis outputs are retained

## 7. Honest Reporting

Do not fill template cells or state null findings until the corresponding protocol has run. Report inconclusive and failed runs with retained evidence.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/03-results.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/03-results.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Larger studies need a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
