# Plan 00 — Canonical Scope and Traceability: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Read fully; governs all lower-level plan files.

**Goal:** State the current implementation and evidence boundary for canonical scope and traceability.
**Builds on:** [00](00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Read fully; governs all lower-level plan files.

This file is the scope authority for every document under `plans/`.

## Research Hierarchy

1. **Primary contribution:** design, implementation, and empirical validation of the Rust-native `Evintkoo/automl` architecture.
2. **Primary implementation:** the `2048-ml` repository, including the framework integration, game environment, data pipeline, and evaluation tooling.
3. **Principal case study:** supervised four-action policy learning for the 4×4 2048 game.
4. **Secondary analyses:** model comparison, feature ablation, label sensitivity, resource measurements, and robustness.

## 2048 State Scope

The canonical training state uses the 16 board cells plus the current score (17 values). It excludes move count and game history; history may be retained for data collection and analysis, but not as training features unless a separately documented study changes this scope. State feature ordering and encoding are specified in the state plans.

## Interpretation Rules

- Framework claims require framework-validation evidence on standard tabular tasks.
- 2048 game score is application evidence, not proof of general AutoML superiority.
- “Best model” means best under the declared 2048 case-study protocol.
- “Rust is faster” is never assumed; runtime, memory, reproducibility, and coverage must be measured.
- External ML libraries may be comparison baselines in framework validation, but core 2048 model training uses the AutoML framework.
- Search-based or learning-based game agents may be comparison baselines, but they are not the primary contribution.
- No result is considered complete until its data, configuration, seed, dependency version, and analysis artifact are recorded.
- PSPACE, Markov-blanket, feature-sufficiency, and numerical PAC claims are excluded from the core unless independently proven with valid assumptions.

## Plan Traceability

| Plan area | Research role | Authority |
|---|---|---|
| `01-Infrastructure` | Framework and system architecture | `01-Project/04-framework-contribution.md` |
| `02-Environment` | 2048 case-study environment | Game rules and simulation plans |
| `03-State` / `04-Actions` | Case-study representation and policy interface | State/action plans |
| `05-Model` | AutoML model capabilities and case-study model selection | Algorithm/training plans |
| `06-Data` | Framework benchmark and 2048 data pipelines | Data plans |
| `07-Benchmarking` | Framework benchmarks and application evaluation | `03-Comparison/04-framework-validation.md` |
| `08-Research-Report` | Thesis argument and evidence | IMRD and methodology plans |
| `09-Quality` | Correctness, CI, review, and reproduction controls | Quality plans |

If a lower-level document conflicts with this file, the lower-level document must be revised or explicitly marked as a case-study-only detail.

---

## Verification (definition of done)

1. `test -f plans/00-scope-and-traceability.md` exits 0.
2. `grep -q '^# Plan 00 — ' plans/00-scope-and-traceability.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/00-scope-and-traceability.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/00-scope-and-traceability.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/00-scope-and-traceability.md` exits 0.
6. `grep -q '^## Open questions$' plans/00-scope-and-traceability.md` exits 0.
7. `grep -q '^## Later$' plans/00-scope-and-traceability.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/00-scope-and-traceability.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Read fully; governs all lower-level plan files. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
