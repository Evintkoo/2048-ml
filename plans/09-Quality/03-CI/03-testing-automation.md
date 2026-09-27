# Plan 03 — Testing Automation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Cargo tests pass locally and in the configured GitHub workflow; coverage gate and scheduled reporting are absent.

**Goal:** State the current implementation and evidence boundary for testing automation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This is an automation plan, not a test inventory.** Existing tests run through `scripts/ci-check.sh`, which is called by the GitHub Actions workflow. Local and hosted run [36281445126](https://github.com/Evintkoo/2048-ml/actions/runs/36281445126) passed 39/39. No coverage gate or scheduled report is configured.

> **See canonical `09-Quality/03-CI/01-ci-pipeline.md` — duplicate stub.** Repetitive CI mermaid trimmed; see canonical for pipeline.

## 1. Purpose

Define automated testing procedures for the 2048 ML system.

## 2. Testing Automation Architecture

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §2 and `09-Quality/01-Testing/01-unit-testing.md` for testing architecture.**

## 3. Test Automation Pipeline

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §3–5 for pipeline mermaid.**

## 4. Automated Test Categories

| Category | Tests | Execution Time | Frequency |
|----------|-------|---------------|-----------|
| Unit/root suite | Binary crate tests | 39 tests; last local run 1.45 sec | GitHub Actions on push/PR/manual dispatch and local script |
| Integration | Focused tests; no dedicated suite | Covered by root suite; not a separate target | GitHub Actions workflow and local script |
| Game | Tests in `game_engine` module | Covered by root suite | GitHub Actions workflow and local script |
| Research validation | Not automated | N/A | Requires separate experiment design |
| Performance | No scheduled suite | N/A | Not configured |

## 5. Test Execution Strategy

> **Trimmed — see `09-Quality/01-Testing/01-unit-testing.md` §7 for execution mermaid; see canonical CI pipeline.**

## 6. Test Automation Configuration

No test automation configuration type or coverage threshold is implemented.

## 7. Test Execution Dashboard

> **Trimmed duplicate mermaid — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §10 for dashboard.**

## 8. Automated Testing Benefits

> **Trimmed — duplicate benefits mermaid removed; see canonical `09-Quality/03-CI/01-ci-pipeline.md`.**

## 9. Test Results Tracking

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §10 for tracking; duplicate mermaid removed.**

## 10. Reporting

No automated test result dashboards, coverage reports, performance trends, or regression alerts are generated.

## Implementation Record

- Redirect/duplicate audited. The CI script passes locally and in hosted run [36281445126](https://github.com/Evintkoo/2048-ml/actions/runs/36281445126) with 39/39 root tests. No coverage dashboard, scheduled performance suite, or historical report artifact exists.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/03-CI/03-testing-automation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/03-CI/03-testing-automation.md` exits 0.

## Open questions

- **Coverage and scheduled performance reporting remain unconfigured.** The hosted workflow runs the correctness suite on push, pull request, or manual dispatch.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
