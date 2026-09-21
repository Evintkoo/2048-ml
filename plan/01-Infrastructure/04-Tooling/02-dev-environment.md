# Development Environment Setup

## 1. System Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| OS | macOS 14+, Linux (kernel 6.0+) | macOS 14+, Ubuntu 22.04+ |
| RAM | 8 GB | 16 GB |
| CPU | 4 cores | 8 cores |
| Disk | 20 GB | 50 GB |
| Rust | 1.75+ | 1.75+ |

## 2. Installation Steps

### 2.1 Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version
```

### 2.2 Install Dependencies

```bash
# Build automl
cd /Users/evintleovonzko/Documents/projects/evint/2048-ml/automl
cargo build --release

# Verify automl works
./target/release/automl --help
```

### 2.3 Initialize Project

```bash
# Initialize workspace
cargo init --name 2048-ml

# Add automl dependency
cargo add automl --path automl
```

## 3. Environment Variables

```bash
# .env file
export RUST_LOG=info
export RUST_BACKTRACE=1
export CARGO_TARGET_DIR=target
export AUTOML_PATH=/Users/evintleovonzko/Documents/projects/evint/2048-ml/automl
export DATA_PATH=/Users/evintleovonzko/Documents/projects/evint/2048-ml/data
export RESULTS_PATH=/Users/evintleovonzko/Documents/projects/evint/2048-ml/results
```

## 4. IDE Setup

### VS Code

```json
{
  "rust-analyzer.checkOnSave": true,
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "rust-analyzer.rustfmt.extraArgs": ["--emit=stdout"],
  "files.associations": {
    "*.yaml": "yaml",
    "*.toml": "toml"
  }
}
```

### Pre-commit Hooks

```bash
#!/bin/bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

## 5. Docker Environment

```dockerfile
FROM rust:1.75-slim

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/2048-ml"]
```

## 6. Environment Validation

```bash
# Verify environment is ready
cargo --version
rustc --version
cargo test --workspace
automl --version
```

## 7. Troubleshooting

| Issue | Solution |
|-------|----------|
| `cargo: command not found` | `source "$HOME/.cargo/env"` |
| `automl build fails` | Check `automl/Cargo.toml` dependencies |
| `Permission denied` | `chmod +x scripts/*.sh` |
| `Out of memory` | Reduce parallel jobs, increase swap |
