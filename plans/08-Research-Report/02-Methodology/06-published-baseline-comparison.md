# Plan 06 — Published Baseline Comparison: the repository status is explicit and evidence based

> **Status: NOT APPLICABLE to the required core execution (2026-09-26).** Published-agent reproduction is explicitly optional; no literature scores are used as measured results.

**Goal:** State the current implementation and evidence boundary for published baseline comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Scope disposition:** Reproducing published agents is optional context and is not required for the core AutoML framework validation or supervised 2048 case study. This does not waive the separate local random/heuristic baseline comparisons. Publication details and quoted performance estimates below remain unverified and must not be used as empirical evidence.

> Published-agent reproduction is optional context. Any score reported from a paper or external codebase requires verified provenance and protocol comparability; it is not a local benchmark result.

## Baseline Reproduction Decision

The 2048 case study must distinguish measured baselines from literature-only context. At least one non-AutoML baseline should be selected for measured comparison when its implementation and computational budget can be controlled. Random and heuristic agents remain minimum sanity baselines.

No external repository reproduction is planned for the core study. If one is later proposed, verify source, version, hardware, rules, training data, seed protocol, and evaluation budget before comparing outcomes.

| Candidate source | Verification state | Treatment |
|----------------|--------------------|-----------|
| Heuristic descriptions in prior drafts | Bibliography/protocol not verified | Do not use quoted score as baseline evidence |
| Expectimax / RL / MCTS papers and implementations | Not independently checked or reproduced | Optional context only |

The local baseline protocol is covered by the benchmark and analysis plans. No external reproduction protocol or statistical test module at `statistical_tests.rs` exists.

**Relationship to 05:** Core framework claims do not depend on reproducing MCTS or RL. Any reproduced search or learning agent is a case-study comparison and must include source version, hardware, compute budget, seed protocol, and limitations. Unreproduced literature results remain contextual only.

## Implementation Record

- No published-agent reproduction is needed for the core scope. The table is retained only as an unverified source checklist; reported score ranges are removed and local random/heuristic runners remain a separate application evaluation.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
2. `grep -q '^# Plan 06 — ' plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/02-Methodology/06-published-baseline-comparison.md` exits 0.

## Open questions

- If publication comparison is later promoted into scope, verify bibliographic records, implementations, rules, resource budgets, and statistical comparability before execution.

## Later

- No external-agent reproduction is required by the current core scope. Reopen only with a specific research question and an approved, reproducible comparison design.
