# Computational Budget

> **Note:** This section provides estimated computational requirements based on planned experiments. These are estimates, not measured values. All costs are approximate and will be updated after actual experiments are run.

## 2. Hardware Requirements

### 2.1 Training Hardware

| Resource | Specification | Quantity | Purpose |
|----------|--------------|----------|---------|
| CPU Cores | 8+ cores | 1 machine | Model training |
| RAM | 16GB+ | 1 machine | Data processing |
| Storage | 100GB SSD | 1 machine | Game data, model checkpoints |
| GPU | Not required | 0 | Supervised classification doesn't need GPU |

**Note:** The supervised classification approach does not require GPU acceleration. All models are trained on CPU using the automl framework.

### 2.2 Evaluation Hardware

| Resource | Specification | Quantity | Purpose |
|----------|--------------|----------|---------|
| CPU Cores | 16+ cores | 1 machine | Game simulation |
| RAM | 32GB+ | 1 machine | Multiple game instances |
| Storage | 500GB SSD | 1 machine | Results, logs |
| Network | Required | — | Data download, collaboration |

## 3. Computational Cost Analysis

### 3.1 Training Cost

**Estimated** time per run based on automl documentation and similar workloads:

| Component | Time per Run | Runs | Total Time | CPU Hours |
|-----------|-------------|------|------------|-----------|
| Data generation | ~30 min | 1 | ~30 min | ~0.5 |
| Model training (RF) | ~5 min | 1 | ~5 min | ~0.08 |
| Model training (GB) | ~10 min | 1 | ~10 min | ~0.17 |
| Model training (XGBoost) | ~10 min | 1 | ~10 min | ~0.17 |
| Model training (LightGBM) | ~8 min | 1 | ~8 min | ~0.13 |
| Model training (ExtraTrees) | ~5 min | 1 | ~5 min | ~0.08 |
| Model training (SVM) | ~15 min | 1 | ~15 min | ~0.25 |
| Model training (KNN) | ~2 min | 1 | ~2 min | ~0.03 |
| Hyperparameter tuning | ~60 min | 7 models | ~7 hours | ~7.0 |
| **Total training** | — | — | **~8 hours** | **~8.2 CPU hours** |

**Note:** These are estimates based on typical automl workloads. Actual times may vary significantly depending on the automl implementation, data size, and hardware.

### 3.2 Evaluation Cost

**Estimated** based on 10,000 games per model:

| Component | Time per Run | Runs | Total Time | CPU Hours |
|-----------|-------------|------|------------|-----------|
| Baseline evaluation (Random) | ~1 hour | 1 | ~1 hour | ~1.0 |
| Baseline evaluation (Heuristic) | ~1 hour | 1 | ~1 hour | ~1.0 |
| Model evaluation (10,000 games) | ~2 hours | 7 models | ~14 hours | ~14.0 |
| Ablation study | ~2 hours | 27 features | ~54 hours | ~54.0 |
| Multi-seed validation | ~2 hours | 5 seeds × 7 models | ~70 hours | ~70.0 |
| **Total evaluation** | — | — | **~140 hours** | **~140 CPU hours** |

### 3.3 Statistical Testing Cost

**Estimated** based on the number of comparisons:

| Component | Time per Run | Runs | Total Time | CPU Hours |
|-----------|-------------|------|------------|-----------|
| Mann-Whitney U tests | ~1 min | 21 comparisons | ~21 min | ~0.35 |
| Kruskal-Wallis test | ~5 min | 1 | ~5 min | ~0.08 |
| Bootstrap CI | ~10 min | 7 models | ~70 min | ~1.17 |
| **Total testing** | — | — | **~96 min** | **~1.6 CPU hours** |

### 3.4 Total Computational Budget

| Phase | CPU Hours (est.) | Wall Clock Time (est.) | Cost (est.) |
|-------|-----------------|----------------------|-------------|
| Training | ~8.2 | ~8 hours | ~$5-10 |
| Evaluation | ~140 | ~140 hours | ~$50-100 |
| Statistical testing | ~1.6 | ~1 hour | ~$1-2 |
| Cross-validation | ~50 | ~50 hours | ~$20-40 |
| Published baseline reproduction | ~30 | ~30 hours | ~$15-30 |
| Sensitivity analysis | ~40 | ~40 hours | ~$20-40 |
| Failure analysis | ~20 | ~20 hours | ~$10-20 |
| **TOTAL (est.)** | **~290** | **~290 hours** | **~$120-240** |

*Cost estimate based on cloud computing rates (e.g., AWS c5.xlarge at ~$0.17/hour). These are approximate estimates and may vary significantly.*

**Note:** These are estimates based on planned experiments. Actual computational costs will depend on the specific automl implementation, game engine performance, and hardware used. All estimates should be updated after the capability verification gate and initial experiments.

## 4. Scalability Analysis

### 4.1 Scaling with Number of Models

| Number of Models | Training Time (est.) | Evaluation Time (est.) | Total CPU Hours (est.) |
|-----------------|---------------------|----------------------|----------------------|
| 7 (baseline) | ~8 hours | ~140 hours | ~148 |
| 14 (double) | ~16 hours | ~280 hours | ~296 |
| 21 (triple) | ~24 hours | ~420 hours | ~444 |

**Note:** These are theoretical estimates assuming linear scaling. Actual scaling may be non-linear due to resource contention.

### 4.2 Scaling with Number of Games

| Games per Model | Evaluation Time (est.) | Total CPU Hours (est.) |
|-----------------|----------------------|----------------------|
| 1,000 | ~14 hours | ~14 |
| 10,000 | ~140 hours | ~140 |
| 50,000 | ~700 hours | ~700 |
| 100,000 | ~1,400 hours | ~1,400 |

**Note:** Linear scaling assumed. Actual scaling may vary.

## 5. Cloud vs Local (Trimmed to 2 Lines + Reference)

Local `c5.xlarge` (~$0.17/h) reference retained; cloud table trimmed to 2 lines. All runs are CPU-only (no GPU) via `rust:1.75-slim` Docker — see `04-Appendix/06-reproducibility-package.md` for Dockerfile.

## 6. Memory Requirements

| Component | Memory Required | Peak Usage |
|-----------|----------------|------------|
| Game simulation | ~1GB | ~2GB |
| Data processing | ~2GB | ~4GB |
| Model training (7 models) | ~4GB | ~8GB |
| Statistical analysis | ~1GB | ~2GB |
| Cross-validation | ~2GB | ~4GB |
| **Total (est.)** | **~10GB** | **~20GB** |

## 7. Storage Requirements

| Component | Size (est.) | Notes |
|-----------|------------|-------|
| Game data (training) | ~500MB | 100,000 samples |
| Game data (evaluation) | ~2GB | 70,000 games |
| Model checkpoints | ~100MB | 7 models |
| Results and logs | ~500MB | All experiments |
| Docker images | ~2GB | Base + dependencies |
| **Total (est.)** | **~5GB** | — |

## 8. Budget Optimization

### 8.1 Cost Reduction Strategies

| Strategy | Savings | Trade-off |
|----------|---------|-----------|
| Fewer games (5,000 instead of 10,000) | ~50% | Lower statistical power |
| Fewer seeds (3 instead of 5) | ~40% | Less robust validation |
| Fewer features (16 instead of 27) | ~40% | Less ablation detail |
| Sequential execution | 0% | Much longer time |
| Spot instances | ~60-70% | Unreliable, may be interrupted |

### 8.2 Recommended Budget

**Minimum budget:** 100 CPU hours, ~$20-40
- 7 models × 10,000 games evaluation
- Basic statistical testing
- Single-seed validation

**Recommended budget:** 200 CPU hours, ~$40-80
- Full evaluation with ablation
- Multi-seed validation
- Cross-validation
- Published baseline comparison

**Comprehensive budget:** 300 CPU hours, ~$60-120
- All experiments
- Multi-seed validation
- Cross-validation
- Sensitivity analysis
- Failure analysis
- Published baseline comparison

## 9. Reproducibility Checklist

- [ ] Docker image provided
- [ ] All scripts documented and executable
- [ ] All dependencies version-pinned
- [ ] Seeds documented and fixed
- [ ] Hardware specifications documented
- [ ] Cost analysis provided
- [ ] Scalability analysis provided
- [ ] Memory requirements specified
- [ ] Storage requirements specified
- [ ] Cloud execution instructions provided
- [ ] Local execution instructions provided
- [ ] Parallel execution configuration provided

## 10. Conclusion

The total estimated computational budget for the 2048 ML research is approximately **290 CPU hours** and **~$120-240** for comprehensive execution. These are estimates based on planned experiments and may vary significantly.

All computations should be reproducible via Docker, ensuring that any researcher can replicate the results regardless of their local hardware configuration.
