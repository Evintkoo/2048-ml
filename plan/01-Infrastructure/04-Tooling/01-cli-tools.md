# CLI Tools Specification

## 1. automl CLI Usage

### 1.1 Training

```bash
# Train a model
automl train --data data.csv --target score --model gradient_boosting

# Train with configuration file
automl train --config config/training/experiment-01.yaml

# Train with auto model selection
automl train --data data.csv --target score --model Auto
```

### 1.2 Prediction

```bash
# Make predictions
automl predict --model model.bin --data test_data.csv --output predictions.csv
```

### 1.3 Benchmarking

```bash
# Benchmark multiple models
automl benchmark --data data.csv --target score
```

### 1.4 Information

```bash
# Show data info
automl info --data data.csv
```

### 1.5 Server Mode (not used for this project)

```bash
# Start web server
automl serve --port 8080
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
