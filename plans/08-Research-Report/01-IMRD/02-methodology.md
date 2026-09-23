# Plan 02 — Methodology: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for methodology.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **This file is a 25-line redirect. Do not duplicate flowcharts or expand scope here. All protocol, variables, and gates are defined in `02-Methodology/01-experimental-design.md`.**

The study is a controlled 4×4 supervised experiment: 27-dim feature vector → `TaskType::MultiClassification` (labels 0–3 = Up/Down/Left/Right) → `TrainEngine`/`HyperOptX` → benchmark-framework evaluation over **≥10,000 games** at **seed 42**.

**Canonical reference:** See `02-Methodology/01-experimental-design.md` for variables, trial structure, replication, bias controls, sample-size justification, and pre-registration. See `02-Methodology/03-hypotheses.md` for framework hypotheses F1–F3 and application hypotheses H1–H3, and `02-Methodology/04-ablation-study.md` for the ablation matrix.

**Pinned equipment (do not drift):**

| Component | Pinned Value | Notes |
|-----------|--------------|-------|
| automl | `v1.0.0` (`https://github.com/Evintkoo/automl`) | `TrainEngine`, `HyperOptX`, `CrossValidator::GroupKFold` |
| Rust | `1.75` (pinned `Cargo.lock`) | No GPU |
| Task | `TaskType::MultiClassification` | 4 actions 0–3 |
| Features | 27-dim fixed | `game_id` column for GroupKFold |
| polars | `0.46` | Parquet I/O |
| Seed | `42` primary; `123,456,789,1011` secondary | Deterministic spawns |
| Games | `10,000` per model/config | Winner = mean ranking |

**Statistical protocol:** pre-registered MWU with Holm correction; bootstrap 95% CIs and Cohen's d are reported as uncertainty and practical-magnitude measures, not extra automatic exclusion gates. See `07-Benchmarking/04-Analysis/02-statistical-analysis.md`.

**No duplication:** No flowchart copy here; no PSPACE/Markov/8×8/ensemble/RL in core.

## Implementation Record

- Redirect audited. The claimed Rust 1.75 verification and Parquet workflow conflict with the current local compiler/integration state; this is a reference to a planned protocol, not evidence that the pinned environment or workflow was executed.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/01-IMRD/02-methodology.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
