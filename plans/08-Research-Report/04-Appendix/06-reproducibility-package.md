# Plan 06 — Reproducibility Package: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for reproducibility package.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

This section specifies the reproducibility package required to replicate the Rust-native AutoML framework evaluation and the 2048 case-study results. The checklist records requirements and must not claim completion until each artifact has been independently verified.

## 2. Code Repository

### 2.1 Repository Structure

```
2048-ml-research/
├── automl/                    # Primary Rust-native AutoML framework submodule
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── src/lib.rs
├── src/                        # Single root MVP crate
│   ├── game_engine/            # board, rules, score, simulation
│   ├── data_pipeline.rs        # features, labels, storage and splits
│   ├── evaluation.rs           # score summaries and statistical comparisons
│   └── main.rs                 # CLI, training, benchmark and report workflows
├── framework_benchmarks/      # Standard tabular framework-validation data/configs
├── data/                      # 2048 data generation and processing
│   ├── training_data/
│   ├── evaluation_data/
│   └── feature_configs/
├── experiments/               # Framework and 2048 experiment configurations
│   ├── configs/
│   │   ├── baseline_experiments.json
│   │   ├── ablation_experiments.json
│   │   └── sota_comparison.json
│   └── seeds/
│       ├── seed_42.json
│       ├── seed_123.json
│       ├── seed_456.json
│       ├── seed_789.json
│       └── seed_1011.json
├── results/                   # Results output
│   ├── raw/
│   ├── processed/
│   └── final/
├── analysis/                  # Analysis scripts
│   ├── ablation_analysis.py
│   ├── statistical_analysis.py
│   ├── ranking_analysis.py
│   └── visualization.py
├── paper/                     # Research paper
│   ├── draft/
│   └── references/
├── Dockerfile
├── docker-compose.yml
├── Cargo.lock
├── requirements.txt
└── README.md
```

### 2.2 Version Control

- **Git repository:** All code committed to a public repository
- **Tagging:** Each experimental run tagged with commit hash
- **Branch naming:** `experiment/{name}/{date}`
- **Release:** Versioned releases for each paper submission

## 3. Data Repository

### 3.1 Training Data

| Dataset | Description | Size | Format | Location |
|---------|-------------|------|--------|----------|
| training_v1 | Training samples for all models | 100,000 samples | Parquet | `data/training_data/training_v1.parquet` |
| evaluation_v1 | Evaluation game data | 70,000 games (7 models × 10,000) | Parquet | `data/evaluation_data/evaluation_v1.parquet` |
| features_v1 | Feature configurations | 27 features | JSON | `data/feature_configs/features_v1.json` |

### 3.2 Data Metadata

Each dataset includes:
- **Creation date and time**
- **Seed used** (for reproducibility)
- **Feature extraction version**
- **Label generation method** (rollout-based, 100 sims/action)
- **Data quality checks** passed
- **Checksum** for integrity verification

### 3.3 Data Access

All data is available via:
- **Public repository:** `https://github.com/evintkoo/2048-ml-research`
- **DOI:** Each dataset assigned a DOI upon publication
- **Automated download:** `scripts/download_data.sh`

## 4. Docker Environment

### 4.1 Dockerfile (Fixed — No apt `polars`)

```dockerfile
FROM rust:1.75-slim

RUN apt-get update && apt-get install -y python3 python3-pip pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release --all
# polars 0.46, pyarrow etc come from pip, not apt — previous `apt-get install polars` deleted
RUN pip install --no-cache-dir -r requirements.txt  # requirements.txt pinned: polars==0.46, pyarrow, etc.
ENV RUST_LOG=info SEED=42 N_GAMES=10000 SEED_SECONDARY="123,456,789,1011"
CMD ["./target/release/experiment_runner"]
```

### 4.2 Docker Compose

```yaml
version: '3.8'
services:
  training:
    build: .
    volumes:
      - ./data:/app/data
      - ./results:/app/results
    environment:
      - SEED=42
      - N_GAMES=10000
    deploy:
      cpus: 4
      memory: 8G
  
  evaluation:
    build: .
    volumes:
      - ./data:/app/data
      - ./results:/app/results
    environment:
      - SEED=42
      - N_GAMES=10000
    deploy:
      cpus: 8
      memory: 16G
  
  analysis:
    build: .
    volumes:
      - ./data:/app/data
      - ./results:/app/results
    command: python3 analysis/statistical_analysis.py
    deploy:
      cpus: 2
      memory: 4G
```

### 4.3 Environment Reproducibility

- **Base image:** `rust:1.75-slim` (pinned version)
- **Rust toolchain:** `1.75.0` (locked in `Cargo.lock`)
- **Python version:** `3.11.x` (locked in `requirements.txt`)
- **All dependencies:** Pinned versions in `Cargo.lock` and `requirements.txt`
- **Build artifacts:** All reproducible from source

## 5. Experiment Scripts

### 5.1 Automated Experiment Runner

```bash
#!/bin/bash
# scripts/run_all_experiments.sh

# Run baseline experiments
for model in RandomForest GradientBoosting XGBoost LightGBM ExtraTrees SVM KNN; do
    cargo run --experiment baseline --model $model --seed 42 --n-games 10000
done

# Run ablation experiments
for feature_set in full minimal empty_only grid_only; do
    cargo run --experiment ablation --features $feature_set --seed 42 --n-games 10000
done

# Run SOTA comparison
for baseline in Cirulli Makrogiannis Kishore Gelly; do
    cargo run --experiment sota --reference $baseline --n-games 10000
done

# Run statistical tests
python3 analysis/statistical_analysis.py

# Generate ranking
python3 analysis/ranking_analysis.py
```

### 5.2 Seed Management

```rust
// All experiments use deterministic seeds
pub struct ExperimentConfig {
    pub primary_seed: u64,      // 42
    pub secondary_seeds: Vec<u64>,  // [123, 456, 789, 1011]
    pub n_games: usize,         // 10,000
    pub n_experiments: usize,   // 3 per seed
}
```

## 6. Reproducibility Checklist

Verification checklist — mark an item complete only after the corresponding artifact and independent check exist:
- [ ] Code is publicly available
- [x] Rust dependencies are locked in `Cargo.lock` (this does not pin toolchain distribution)
- [ ] Docker environment is provided
- [ ] Framework benchmark data/configuration is available
- [x] 2048 data-generation procedure is available in `cargo run -- data-collector collect --help`
- [x] Seeds are fixed and documented per dataset/run manifest
- [ ] All scripts are executable
- [ ] Determinism is verified across repeated runs
- [x] Statistical analysis is reproducible from benchmark CSVs via `benchmark compare` and `benchmark report`; verify manifests alongside outputs
- [ ] Figures are generated from raw data
- [ ] Paper references match code versions
- [ ] Framework artifacts can be reloaded with equivalent predictions

The checked items describe implemented procedures, not a completed research replication. Committed run artifacts, published data, standard-dataset framework validation, and independent artifact reproduction remain outstanding. Dataset and benchmark manifests include SHA-256 checksums of their CSV files, source revision when available, protocol, seed sequence, summary, dependency submodule pin where applicable, elapsed time, and result path.

## Implementation Record

- Root source is one Rust crate with a pinned AutoML submodule; collector and benchmark CLIs emit manifests. Docker, public dataset/DOI, external framework dataset configs, complete experiment scripts, figures, and independent reproduction are absent. Several sample paths and Python requirements in this package do not match the actual Rust-only source tree and require correction.

## 7. Reproducibility Failure Modes

| Failure Mode | Detection | Mitigation |
|-------------|-----------|------------|
| Dependency version mismatch | CI pipeline | Pinned versions in Cargo.lock |
| Seed incompatibility | Cross-platform test | Fixed seed, cross-platform testing |
| Data corruption | Checksum verification | Automated integrity checks |
| Hardware differences | Floating point variance | Fixed precision, cross-platform validation |
| Non-deterministic behavior | Re-run with same seed | Seed-based determinism verification |
| Code changes between runs | Git tagging | Commit hash embedded in results |

## 8. Publication Requirements

For publication, the following must be provided:
- All source code (public repository)
- All data (public repository or DOI)
- Docker image (published to Docker Hub)
- Experiment scripts (executable)
- Results (raw and processed)
- Figures (generated from raw data)
- Statistical analysis (reproducible scripts)

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
2. `grep -q '^# Plan 06 — ' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
