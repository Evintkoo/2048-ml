# Literature Review — Rust-Native AutoML Architecture with 2048 Case Study

## 1. Scope and Method

The primary review covers four connected areas: (i) AutoML system architecture and automated model selection, (ii) Rust-native machine-learning systems and ecosystem design, (iii) reproducible ML infrastructure and benchmarking, and (iv) 2048 heuristics/search and supervised sequential decision-making as the application case study. Every citation must be verified before appearing in the final thesis; provisional sources are explicitly marked and cannot support final claims.

The literature review must establish the framework gap first. 2048 literature is used to justify the case study, state representation, labels, and evaluation—not to replace the AutoML architecture contribution.

## 2. AutoML Systems and Architecture — Primary Literature

### 2.1 Automated Model Selection

Review classical model-selection pipelines, automated preprocessing, algorithm selection, hyperparameter optimization, early stopping, and resource-aware search. Distinguish AutoML systems from individual optimizers such as random search, TPE, and Hyperband.

### 2.2 AutoML System Design

Compare the architectural choices of established AutoML systems: data abstractions, pipeline composition, search-space representation, trial management, validation, persistence, parallelism, failure handling, and experiment tracking. The review must identify which capabilities are missing, difficult to reproduce, or poorly integrated in existing systems.

### 2.3 Rust-Native ML Systems

Review Rust ML and data-processing ecosystems, including type safety, ownership, parallel execution, serialization, FFI/interoperability, deployment, and the trade-off between native performance and algorithm/library coverage. Do not claim Rust is categorically faster; compare measured dimensions and cite primary technical sources.

### 2.4 Reproducibility and Benchmarking

Review dataset leakage, nested validation, random-seed control, benchmark fairness, resource reporting, artifact evaluation, and reproducible computational experiments. These principles define the framework-validation protocol.

## 3. Application Case Study: 4×4 2048

### 3.1 History

**Cirulli (2014)** shipped JS 2048; **Björk (2014, unverified source `ashmax.com` – link not verified)** proposed weighted heuristic `w1·empty + w2·mono + w3·smooth + w4·merge`. **Kishore et al. (2014)** compared monotonicity/corner strategies; **Oster et al. (2014, arXiv)** used expectimax depth 4–6. **Makrogiannis et al. (2016, IEEE TG)** formalized MDP + value iteration.

> **Consistency fix:** Prior draft quoted heuristic `~4000–8000`. That conflates expectimax search on 4×4 with the simple weighted heuristic. **Canonical:** simple heuristic `~512` mean (Björk corner), expectimax is **out-of-scope** for core ranking and noted only as optional extended baseline (Appendix). All §5 baselines now use `~512` (heuristic) and `~128` (random) — see `02-Methodology/05-sota-comparison.md`.

### 3.2 Heuristics (Relevant to 27-dim)

- **Empty tiles** — most predictive per Björk (2014, unverified – heuristic claim, needs empirical check).
- **Monotonicity** — row/col monotonic build toward corner (Thumsey et al. 2014 – GitHub, unverified).
- **Smoothness / merge potential** — gradient/adacency, encoded as `smooth_*`, `merge_*`, `adjacency_merge_score`.

### 3.3 ML on 2048 (Case-Study Context)

Woltman & Sarakiki (2014, unverified), Hearn & Rexford (2016 – RL), Nair et al. (2015 – DQN), Gelly et al. (2016 – MCTS+NN) are **gold-optional** and not reproduced as required experiments. If referenced, scores (~2000–10000) are **hypotheses**, not verified reproductions, and belong in `02-Methodology/06-published-baseline-comparison.md` appendix (15-line, optional, 30h not required).

### 3.4 Theory

**Boppana (1988, unverified – 15-puzzle context, not 2048)** cited historically; do not claim as 2048 max-tile proof. **Berg & Hartke (2014, arXiv, verified)** rigorous max-tile `2^15=32768`; **Sinclair (2016, arXiv, unverified completeness)** conjectured hardness (see theoretical framework – Appendix conjecture only).

## 4. AutoML Methods Used by the Framework

### 4.1 Hyperparameter Optimization

Bergstra & Bengio (2012, JMLR) random search; Bergstra et al. (2013) Hyperopt TPE; Li et al. (2017) Hyperband; Feurer & Hutter (2019) AutoML survey. Relevant: `HyperOptX` TPE + `MedianPruner` in `automl/src/optimizer`.

### 4.2 AutoML for Sequential Applications (Secondary Gap)

Silver et al. (2017), Schrittwieser et al. (2020) – deep RL + MCTS, used only as contextual comparison. The secondary gap is the limited evaluation of general-purpose AutoML systems in sequential stochastic applications with trajectory-generated tabular data.

## 5. Rust ML and Framework Positioning

- `automl v1.0.0` (`https://github.com/Evintkoo/automl`): `TrainEngine`, `TaskType::MultiClassification`, `ModelType::{RandomForest, GradientBoosting, XGBoost, LightGBM, ExtraTrees, SVM, KNN}`, `CrossValidator`.
- `polars 0.46`, `smartcore 0.3`, `linfa 0.7` (see `automl/Cargo.toml`).
- Bravegates & Renzelmann (2019, arXiv unverified – Rust ML survey) and Matsakis/Lamport claims marked **unverified**.

## 6. Gap Analysis

| # | Gap | In-Scope? | How Addressed |
|---|-----|-----------|---------------|
| 1 | Integrated Rust-native AutoML architecture | Yes | Architecture, data contracts, capability matrix, and validation |
| 2 | Reproducible Rust AutoML benchmarking | Yes | Fixed datasets, splits, budgets, seeds, and resource reporting |
| 3 | AutoML framework behavior in sequential stochastic data | Yes | 2048 case study with trajectory and policy evaluation |
| 4 | Model and hyperparameter selection under one native pipeline | Yes | Standard tabular and 2048 comparisons |
| 5 | Feature-group and label contribution in 2048 | Secondary | Ablation and sensitivity analysis |
| 6 | n×n / PSPACE / Markov blanket / information theory | No | Remove from core; retain only if rigorously justified |
| 7 | RL / DQN / MCTS reproduction | Optional context | Include only as explicitly defined baselines |

Remainder gaps are out-of-scope and removed (no TBD filler).

## 7. Contribution (Matches Introduction §5)

The primary contribution is the Rust-native AutoML architecture and its validation. The 2048 integration is the principal case study demonstrating end-to-end usability, not the only contribution. No PSPACE, Markov-blanket, or PAC claim is part of the core contribution without a separate rigorous proof.

## 8. References (Verified vs Provisional)

Verified or primary candidates currently include Bergstra & Bengio (2012), Bergstra et al. (2013), Li et al. (2017), Feurer & Hutter (2019), the `automl` repository, and relevant Rust/data-system documentation. 2048 and Rust-ML sources remain provisional until their exact bibliographic records and claims are checked. No provisional citation may be used to establish a central thesis claim. See `04-Appendix/03-references.md` for the final verified bibliography.
