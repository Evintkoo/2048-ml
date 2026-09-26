# Plan 02 — Automation: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** A local Rust quality-check script and GitHub workflow exist; build/test/lint are automated, while deployment and research automation are out of scope or pending.

**Goal:** State the current implementation and evidence boundary for automation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**A small quality-check script now backs the GitHub workflow.** It runs formatting, Cargo tests, and Clippy; it does not run research benchmarks or deployment.

> **See canonical `09-Quality/03-CI/01-ci-pipeline.md` — this doc is a duplicate stub.** Trimmed repetitive mermaid; see canonical for pipeline diagrams.

## 1. Purpose

Define automation procedures for the 2048 ML system development workflow.

## 2. Automation Architecture

> **Trimmed duplicate mermaid — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §2 for pipeline mermaid.** Minimal note only: Build → Test → Lint → Validate → Deploy.

## 3. Automation Pipeline

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §3 for pipeline stages.** (Code Change → Build → Test → Lint → Deploy → Monitor).

## 4. Automation Categories

| Category | Tool | Purpose | Frequency |
|----------|------|---------|-----------|
| Build/check | Cargo | GitHub Actions push/PR/manual workflow; local script available | On configured workflow events |
| Test | Cargo | `scripts/ci-check.sh` and GitHub Actions | On configured workflow events |
| Lint | Cargo fmt/clippy | `scripts/ci-check.sh` and GitHub Actions | On configured workflow events |
| Deploy | None | No deployment target in scope | N/A |
| Monitor | None | No service monitoring target | N/A |

## 5. Test Automation Framework

> **Trimmed duplicate — see `09-Quality/03-CI/01-ci-pipeline.md` §6 and `09-Quality/01-Testing/01-unit-testing.md` for test mermaid.**

## 6. Deployment Automation

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §4–5 for build/deploy mermaid.**

## 7. Automated Quality Gates

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` §7 for quality gates mermaid.**

## 8. Automation Scripts

No `AutomationConfig` implementation exists. `scripts/ci-check.sh` is the implemented local quality-check script; it makes no deployment or experiment-scheduling claim.

## 9. Automation Benefits

> **Trimmed — see canonical for benefits; duplicate mermaid removed.**

## 10. Monitoring Automation

> **Trimmed — see canonical `09-Quality/03-CI/01-ci-pipeline.md` for monitoring; duplicate mermaid removed.**

## Implementation Record

- Redirect/duplicate audited. The repository now contains `scripts/ci-check.sh` and `.github/workflows/ci.yml`; local execution passes. No deployment or research-benchmark automation exists, and a hosted workflow run remains unverified.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/03-CI/02-automation.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/09-Quality/03-CI/02-automation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/03-CI/02-automation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/03-CI/02-automation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/03-CI/02-automation.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/03-CI/02-automation.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/03-CI/02-automation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/03-CI/02-automation.md` exits 0.

## Open questions

- **Hosted workflow evidence remains pending.** Keep automation limited to the configured Rust quality checks unless a separate research-benchmark or deployment need is specified.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
