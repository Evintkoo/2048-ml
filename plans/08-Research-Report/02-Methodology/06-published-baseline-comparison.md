# Published Baseline Comparison — Case-Study Context and Reproduction Decision

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
