# Reproducibility Package

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
│   ├── data_pipeline/          # features, labels, storage
│   └── evaluation/             # benchmark, statistics, ranking
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
- [ ] All dependencies are version-pinned
- [ ] Docker environment is provided
- [ ] Framework benchmark data/configuration is available
- [ ] 2048 data-generation procedure is available
- [ ] Seeds are fixed and documented
- [ ] All scripts are executable
- [ ] Determinism is verified across repeated runs
- [ ] Statistical tests are reproducible
- [ ] Figures are generated from raw data
- [ ] Paper references match code versions
- [ ] Framework artifacts can be reloaded with equivalent predictions

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
