# CLI Tools Specification

> **Verify CLI via `cargo run -- --help`; `automl serve` is out-of-scope per initial-plan.**

## 1. automl CLI Usage

### 1.1 Training

```bash
# Train a model — canonical target is `action` (u8 0–3), TaskType::MultiClassification
automl train --data data.csv --target action --model gradient_boosting

# Train with configuration file (config must set target: action, task: MultiClassification)
automl train --config config/training/experiment-01.yaml

# Train with auto model selection — still predicts `action`
automl train --data data.csv --target action --model Auto
```

### 1.2 Prediction

```bash
# Make predictions
automl predict --model model.bin --data test_data.csv --output predictions.csv
```

### 1.3 Benchmarking — Verify via `cargo run -- --help`

> **Note:** `automl benchmark` subcommand unchecked — verify via `cargo run -- --help` before assuming availability; fallback is local `benchmark` binary if automl CLI lacks it.

```bash
# Benchmark multiple models — benchmark target is still `action` (classification);
# game score is measured separately by running the policy in the simulator
automl benchmark --data data.csv --target action  # verify: cargo run -- --help
```

### 1.4 Information

```bash
# Show data info
automl info --data data.csv
```

### 1.5 Server Mode — Out of Scope per Initial Plan

> **Out of scope per initial plan.** `automl serve` is not used for this project; verify via `cargo run -- --help` and do not rely on it. Use local CLI only.

```bash
# Start web server — OUT OF SCOPE, do not use
automl serve --port 8080  # out-of-scope per initial-plan
```

## 2. Custom CLI Tools

### 2.1 Game Simulation CLI

```bash
# Run a single game
./game-engine simulate --seed 42 --model model.bin

# Run batch simulation
./game-engine simulate --n-games 1000 --model model.bin --output results/

# Run with logging
./game-engine simulate --seed 42 --verbose --log training.log
```

### 2.2 Data Collection CLI

```bash
# Collect training data from self-play
./data-collector collect --n-games 10000 --output data/raw/

# Preprocess collected data
./data-collector preprocess --input data/raw/ --output data/processed/

# Validate data integrity
./data-collector validate --input data/processed/
```

### 2.3 Benchmark CLI

```bash
# Run full benchmark suite
./benchmark run --config config/benchmark/automl-benchmark.yaml

# Compare models
./benchmark compare --models model1.bin,model2.bin,model3.bin

# Generate benchmark report
./benchmark report --input results/ --output reports/
```

## 3. CLI Configuration

All CLI tools will use `clap` for argument parsing with structured configuration files.

## 4. CLI Output Formats

| Format | Description | Use Case |
|--------|-------------|----------|
| JSON | Machine-readable | Pipeline integration |
| CSV | Tabular data | Analysis |
| Markdown | Human-readable | Reports |
| Plain | Default | Terminal output |

## 5. Error Handling

All CLI tools will return:
- Exit code 0 on success
- Exit code 1 on error
- Exit code 2 on configuration error
- Structured error messages to stderr
