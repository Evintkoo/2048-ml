# Plan 02 — Self-Play Data: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-27).** The simulator can run a supplied single-agent policy, but it does not capture policy-state rows; the collector has no policy-source mode or self-play corpus. Proposed rollout labeling also awaits a compute budget.

**Goal:** State the current implementation and evidence boundary for self-play data.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats policy-trajectory data collection as pending.** `GameSimulator::simulate_with_policy` accepts a policy closure and returns game results, but it does not capture pre-move state rows. The checkpointed `data-collector collect` path records uniform random legal moves only; no configurable policy-source collector or `self_play.csv` artifact exists.

## 1. Purpose

Generate state coverage by running a **single agent** against the stochastic 2048 environment (no opponent). A future policy-source collector would need to pass states through the same rollout labeler; the current collector does not provide this source option.

## 2. Single-Agent Model (2048 is single-player)

2048 has no second player. "Self-play" here means **one agent vs. stochastic tile spawns** (90% `2`, 10% `4` at a random empty cell after each move).

There is no canonical game-count target. A future collector must make the game count, policy source, seed, rollout count, move cap, and output/metadata paths explicit. It should use `GameSimulator::new(SimulatorConfig { .. })` and `simulate_with_policy`; the simulator owns the seeded spawn RNG.

## 3. Collection Loop — Record then Relabel

The collector must retain a pre-move `RawBoardState` snapshot and its 17-value `BoardStateMl` representation for each policy decision. After the game finishes, it can call `RolloutLabeler::label(&snapshot, base_seed, game_id, move_index)` to produce the supervised action. `simulate_with_policy` currently accepts a callback over `&RawBoardState` and returns a `GameResult`, but exposes no row-capture hook; a dedicated collector or simulator callback extension is needed. Policy-selection errors must be mapped to `GameError` by the current API.

Stochastic spawn note: after each `apply`, engine spawns `2` (p=0.9) or `4` (p=0.1) in a uniformly random empty cell. Seed via `ChaCha8Rng`.

## 4. Label Regeneration — REQUIRED

All self-play rows **MUST** be relabeled via `01-data-collection-strategy.md §8.3` (`RolloutLabeler`, 100 rollouts, `argmax` over valid moves by mean final score). Invalid moves are masked, never labeled. Original `original_action` is discarded.

## 5. Storage

CSV `06-Data/03-Storage/self_play.csv` — header `grid_0..score_normalized,action` (18 cols) with row-aligned game metadata. See `02-Format/01-data-schema.md`; features use the fixed deterministic scales.

## 6. Next Steps

1. Select and record the policy source; obtain a data-collection compute budget.
2. Relabel with a configured `RolloutLabeler` (100 rollouts, explicit move cap and spawn probability).
3. Validate `NF==18`, `action ∈ 0..3`, no `done`/`reward` columns; write to `06-Data/03-Storage/`.

The root collector currently supplies only random trajectories. Implement a configurable single-agent policy source and preserve rollout-based labels to fulfill this ticket.

## Implementation Record

- The simulator supports callback-driven policy play, but `simulate_with_policy` does not capture state rows. There is no configurable policy-source mode or persisted `self_play.csv`; no corpus size has been approved and the corpus remains uncollected.

---

## Verification (definition of done)

1. `test -f plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/01-Collection/02-self-play-data.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/01-Collection/02-self-play-data.md` exits 0.

## Open questions

- No policy-trajectory corpus has been collected. Define the policy source and compute budget, then retain config, seed, data, sidecar, and manifest artifacts. Any rollout labels use the documented deterministic seed derivation and move cap.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
