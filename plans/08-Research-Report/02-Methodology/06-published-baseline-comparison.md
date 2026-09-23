# Plan 06 — Published Baseline Comparison: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for published baseline comparison.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Status: 20-line reproduction plan. Not required for core ranking. Core ranking uses only Random (~128) and Heuristic (~512). 30h reproduction NOT required. Do not block IMRD on this file.**

## Baseline Reproduction Decision

The 2048 case study must distinguish measured baselines from literature-only context. At least one non-AutoML baseline should be selected for measured comparison when its implementation and computational budget can be controlled. Random and heuristic agents remain minimum sanity baselines.

If time permits, may optionally reproduce up to 6 repos with **same 10k/seed 42** evaluation and MWU/Bonf + bootstrap + d gate. Mark all scores as **optional, not core**.

| Repo / Paper | URL Placeholder (pin if reproduced) | Expected (hypothesis) | Core? |
|--------------|--------------------------------------|-----------------------|-------|
| Björk heuristic | `https://github.com/...` (TBD if reproduced) | ~512 | No — heuristic reference already ~512 |
| Cirulli JS agent | `https://github.com/gabrielecirulli/2048` | ~128–500 | Optional |
| Oster expectimax | arXiv 1412.6881 | ~2000–8000 | Optional gold |
| Kishore monocorner | IJCSI 2014 | ~4000–8000 | Optional gold |
| Makrogiannis | IEEE TG 2016 | ~4000–8000 | Optional gold |
| Gelly MCTS+NN | NIPS 2016 | ~10000+ | Optional gold |

**Protocol if executed:** `BenchmarkConfig { n_games:10000, seed:42 }` → `ScoreMetrics` → `statistical_tests.rs::mann_whitney` → Bonferroni over #comparisons. **If not executed:** state "Extended reproduction not performed; core comparison is Random/Heuristic only" — no penalty.

**Relationship to 05:** Core framework claims do not depend on reproducing MCTS or RL. Any reproduced search or learning agent is a case-study comparison and must include source version, hardware, compute budget, seed protocol, and limitations. Unreproduced literature results remain contextual only.

## Implementation Record

- Published-agent reproduction is explicitly optional and has not been performed. Provisional citations and reported score ranges are not used as empirical baselines.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
