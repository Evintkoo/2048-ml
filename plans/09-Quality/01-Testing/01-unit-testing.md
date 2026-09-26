# Plan 01 — Unit Testing: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** Core engine/state/data/evaluation tests are present; coverage target and several proposed cases remain unmeasured or incomplete.

**Goal:** State the current implementation and evidence boundary for unit testing.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**The root test suite was revalidated in this pass.** `cargo test` passes all 35 binary tests. `cargo test --lib` is inapplicable because the root package has no library target. No coverage report or Tarpaulin configuration was found; the proposed 80% target is not evidence.

## 1. Purpose
Unit tests for 4×4 game engine, feature extraction, and automl wiring. Distinct from integration (pipeline wiring) and game-validation (manual audit).

## 2. Concrete 2048 Cases (Must Implement)

| # | Module | Case | Assertion |
|---|--------|------|-----------|
| 1 | `game_engine/mod.rs` | adjacent merges without a second merge per tile | Existing merge regression; board-specific sequence cases vary |
| 2 | `game_engine/mod.rs` | single merge per tile per move | Existing unit coverage |
| 3 | `game_engine/mod.rs` | unchanged move does not spawn or score | Existing unit coverage |
| 4 | `game_engine/mod.rs` | seeded spawn reproducibility and configured probability | Existing deterministic/probability checks; finite-sample behavior only |
| 5 | `game_engine/mod.rs` | spawn selects an empty cell | Seeded empty-cell choice is deterministic |
| 6 | `game_engine/mod.rs` | score accumulation and overflow behavior | Score uses `u64`; overflow boundary coverage is absent |
| 7 | `game_engine/mod.rs` | blocked move (all moves would_change==false) | `is_game_over()==true` |
| 8 | `state.rs` | feature vector shape/order and score encoding | Existing tests; game ID is metadata, not a feature |
| 9 | `game_engine/mod.rs` | rollout label is one of four directions | Existing rollout-label tests |
| 10 | `framework_validation.rs` | supported four-class model fit/predict and integration APIs | Smoke tests exist and pass; broad framework validation is separate |

Add explicit boundary/error coverage for accepted tile values and score overflow if required; do not assert 32768 as a proven maximum.

## 3. Coverage (Realistic, Not Fantasy)

Coverage targets remain proposed. No `tarpaulin.toml` or measured coverage report is present; establish a baseline before adopting a threshold.

## 4. Execution

`cargo test` validates the current root suite. Previously recorded coverage and lint runs remain historical evidence and should be rechecked before a release.

## 5. Quality Gate
Before release, run the agreed test suite and any adopted coverage gate; no coverage threshold is currently measured or configured.

## Implementation Record

- Source audit found tests for merge/score history, no-op and terminal moves, seeded spawn, features, CSV/splits, statistical helpers, framework smokes, and current AutoML revision provenance. `cargo test` passed 35/35; no Tarpaulin config or coverage report exists.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/01-Testing/01-unit-testing.md` exits 0.

## Open questions

- **Coverage remains unmeasured.** The current root suite passed; measure coverage before adopting a numerical gate.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
