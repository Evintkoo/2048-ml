# Plan 01 — Discrete Actions: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for discrete actions.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Canonical:** `04-Actions/01-Action/01-action-space.md` — single source for action space. This file is retained for historical reasons; see canonical. No code duplication.

Canonical: `0=Up, 1=Down, 2=Left, 3=Right` — `u8 0..3`, `TaskType::MultiClassification`, validity via `board.would_change`.

## Implementation Record

- Redirect audited; the canonical action enum and validity implementation are in `src/game_engine/mod.rs` and `src/actions.rs`.

---

## Verification (definition of done)

1. `test -f plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
6. `grep -q '^## Open questions$' plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
7. `grep -q '^## Later$' plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/04-Actions/02-Space/01-discrete-actions.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
