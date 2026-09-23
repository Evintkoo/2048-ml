# Methodology — Redirect (Canonical: 02-Methodology/01-experimental-design.md)

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
