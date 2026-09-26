# Plan 07 — Computational Budget: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** A small baseline timing report and rollout-collection projection exist; no comprehensive study budget is approved.

**Goal:** State the current implementation and evidence boundary for computational budget.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Budget figures must be tied to measured commands and hardware.** Most former resource tables were speculative and are removed. A rollout-labeled corpus estimate is a planning prerequisite, not an approved run.

> The 20,000-game rollout-labeled collection projection is approximately 103 hours at the recorded pilot throughput. This estimate depends on configuration and machine and requires a declared compute budget before collection.

## 1. Measured Evidence

The action-frequency baseline report records local sequential development-profile runtimes of 27.89 seconds for 10,000 random games and 201.99 seconds for 10,000 heuristic games on macOS arm64 / Rust 1.96.1. These are protocol-specific smoke measurements, not optimized performance comparisons and not AutoML model timings. See `reports/action-frequency/README.md`.

| Workload | Evidence | Status |
|----------|----------|--------|
| Random baseline | 10,000 games; 27.89 seconds in recorded environment | Measured once under stated protocol |
| Heuristic baseline | 10,000 games; 201.99 seconds in recorded environment | Measured once under stated protocol |
| Rollout-labeled collection | ~103 hours projected for 20,000 games at the documented 100-rollout setting | Projection from pilot; budget not approved |
| Model training and inference | No representative plan-scale profile | Pending |
| Framework datasets and matched baselines | No resource profile | Pending |
| Ablation / multi-seed studies | No run matrix or budget | Pending |

## 2. Budget Requirements Before Execution

For each proposed study, record the machine, OS, compiler, dependency/submodule state, thread count, data size, model/configuration count, repetitions, expected wall time, storage, and stopping conditions. Run a small pilot with the intended command and scale from measured throughput. Keep estimates separate from actuals.

Do not derive a budget from unsupported per-model timings, linear scaling tables, cloud hourly costs, or hypothetical memory totals. The previous 290 CPU-hour and dollar estimate is withdrawn because its candidate set, run durations, storage formats, and study matrix do not match implemented workflows.

## 3. Resource Record Template

| Study / command | Machine and versions | Input size | Runs | Wall time | Peak memory | Output size | Evidence path |
|-----------------|----------------------|------------|------|-----------|-------------|-------------|---------------|
| Pending | Pending | Pending | Pending | Pending | Pending | Pending | Pending |

## 4. Scheduling Gate

The plan-scale collection, framework dataset matrix, model comparison, and ablation studies remain unscheduled. Before starting each, approve a concrete resource envelope and retain raw outputs, manifests, code revision, and analysis artifacts. If the measured pilot exceeds the envelope, reduce or redesign the study before full execution; do not silently use a smaller sample and preserve the same claim.

## Implementation Record

- Baseline timing evidence is available in the action-frequency report; canonical rollout collection is projected at about 103 hours for 20,000 games. No comprehensive budget, Docker workflow, or framework/model resource profile exists.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
2. `grep -q '^# Plan 07 — ' plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/03-Findings/07-computational-budget.md` exits 0.

## Open questions

- **Comprehensive resource evidence remains pending.** Approve a concrete budget based on a representative pilot and retain machine, software, data, and analysis provenance.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
