# Reproducibility Package

## 1. Purpose

This section specifies the complete reproducibility package required to replicate all experimental results. This follows the standards of reproducible research expected at the PhD level.

## 2. Code Repository

### 2.1 Repository Structure

```
2048-ml-research/
├── automl/                    # Evintkoo/automl submodule
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── src/lib.rs
├── game-engine/               # Custom Rust 2048 engine
│   ├── Cargo.toml
│   ├── src/game_engine.rs
│   ├── src/board_state.rs
│   ├── src/move_rules.rs
│   └── src/score_calculator.rs
├── training/                  # ML training pipeline
│   ├── Cargo.toml
│   ├── src/train_engine.rs
│   ├── src/feature_extraction.rs
│   ├── src/label_generation.rs
│   └── src/model_evaluation.rs
├── evaluation/                # Evaluation and benchmarking
│   ├── Cargo.toml
│   ├── src/benchmark_runner.rs
│   ├── src/statistical_tests.rs
│   └── src/ranking.rs
├── data/                      # Data generation and processing
│   ├── training_data/
│   ├── evaluation_data/
│   └── feature_configs/
├── experiments/               # Experiment configurations
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

### 4.1 Dockerfile

```dockerfile
FROM rust:1.75-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    polars \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy all source code
COPY . .

# Build Rust projects
RUN cargo build --release --all

# Install Python dependencies
RUN pip3 install -r requirements.txt

# Set environment variables
ENV RUST_LOG=info
ENV SEED=42
ENV N_GAMES=10000

# Default command
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

All results verified for:
- [x] Code is publicly available
- [x] All dependencies are version-pinned
- [x] Docker environment is provided
- [x] Data is publicly available
- [x] Seeds are fixed and documented
- [x] All scripts are executable
- [x] Results are deterministic (same seed → same results)
- [x] Statistical tests are reproducible
- [x] Figures are generated from raw data
- [x] Paper references match code versions

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
