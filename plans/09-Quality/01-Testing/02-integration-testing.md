# Plan 02 — Integration Testing: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** CLI paths and focused integration smokes exist; the proposed end-to-end suite and Parquet path do not.

**Goal:** State the current implementation and evidence boundary for integration testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This ticket specifies integration coverage, but the full matrix has not been run as one dedicated suite.** `cargo test` passed all 35 current root tests, including focused collection, split, CV, and model wiring checks. Current data and benchmark workflows use CSV/JSON.

## 1. Purpose
Verify **wiring** between modules — not unit logic. Distinct from unit (single function) and CI pipeline (stages). Use real fixtures where available; record seed roles. No single canonical integration suite is configured.

## 2. Integration Levels
- **Level 1:** Game+Score+State+Action (engine internal)
- **Level 2:** Data Pipeline → TrainEngine (feature→label→fit)
- **Level 3:** TrainEngine → GameSimulator round-trip (train → predict → benchmark 100 games)
- **Level 4:** Config→All (TrainingConfig propagates to every crate)

## 3. Integration Matrix (Concrete)

| Test | Fixture | Wiring | Assert | Time |
|------|---------|--------|--------|------|
| Game+Score | Current `BoardState` fixture | `Simulator` → score record | Board and score update agree with move result | Focused unit paths exist |
| Score+State+Action | board and score | `BoardStateMl(17) → action 0..3 → board` | action is in range and legal after masking | Focused unit paths exist |
| Data→TrainEngine | CSV rows plus game metadata | project preprocessing/training path | predictions are four-class outputs | Not run as suite |
| Train+Simulator | trained AutoML model | policy inference and game simulator | valid masked actions and per-game outcomes | Not run as suite |
| Config→CLI | recorded train/search/benchmark arguments | configuration and manifest outputs | declared values match retained metadata | Not audited end-to-end |
| CSV and manifests | actual benchmark output files | report/compare CLI inputs | summaries and provenance are retained | Focused paths exist; full suite pending |
| GroupKFold wiring | Synthetic rows plus `game_id` | project grouped-CV helper | no game group appears in both fold sides | Focused tests exist |

## 4. Fixtures

Use `RawBoardState`, canonical `TrainingSample`, game metadata CSV, and temporary output directories from existing root tests. These fixtures are implementation-specific; proposed API names in older examples are not current interfaces.

## 5. Run

No dedicated `tests/integration` suite or integration JSON reporter was found. The root suite passed; revalidate relevant CLI workflows before release.

## Implementation Record

- Root implementation and focused tests cover CSV collection/splitting, grouped CV, training, inference, and benchmarks. `cargo test` passed 35/35. No dedicated end-to-end test suite, Parquet pipeline, or integration report artifact exists.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/01-Testing/02-integration-testing.md` exits 0.

## Open questions

- **End-to-end integration validation remains pending.** Build a suite from current CSV/JSON paths and supported model workflows before relying on a full round-trip claim.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
