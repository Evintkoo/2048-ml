# Tooling Configuration

## 1. Build Tools

### 1.1 Cargo (Primary Build System)

```bash
# Build automl
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint
cargo clippy
```

### 1.2 Makefile

```bash
make build      # cargo build --release
make server     # build and run server on :8080
make dev        # run server via cargo run (debug)
make cli        # show CLI help
make test       # cargo test
make bench      # cargo bench
make fmt        # cargo fmt
make lint       # cargo clippy
make clean      # cargo clean
```

### 1.3 Cross-Compilation

```bash
# Target aarch64 for mobile testing
cargo build --release --target aarch64-apple-darwin
```

## 2. Development Tools

| Tool | Command | Purpose |
|------|---------|---------|
| Rust Analyzer | rust-analyzer | IDE support |
| cargo-edit | cargo add/rm | Dependency management |
| cargo-outdated | cargo outdated | Check for updates |
| cargo-audit | cargo audit | Security audit |
| flamegraph | cargo flamegraph | Performance profiling |
| criterion | cargo criterion | Benchmarking framework |

## 3. Testing Tools

### 3.1 Unit Testing
```bash
cargo test --lib
cargo test --bin automl
```

### 3.2 Integration Testing
```bash
cargo test --test integration
```

### 3.3 Benchmarking
```bash
cargo bench
# Results in target/criterion/
```

### 3.4 Property Testing
```bash
cargo test --proptest
```

## 4. Debugging Tools

| Tool | Purpose |
|------|---------|
| `cargo run --` | Debug run |
| `RUST_BACKTRACE=1` | Stack traces |
| `RUST_LOG=debug` | Logging |
| `tracing-subscriber` | Structured logging |
| `perf` | CPU profiling |
| `valgrind` | Memory profiling |

## 5. Docker

```bash
# Build Docker image
docker build -t automl-2048 .

# Run container
docker run -p 8080:8080 automl-2048
```

## 6. CI/CD Pipeline

| Stage | Tool | Trigger |
|-------|------|---------|
| Build | cargo | Push to main |
| Test | cargo test | Pull request |
| Bench | cargo bench | Scheduled |
| Lint | cargo clippy | Push |
| Format | cargo fmt | Push |
| Audit | cargo audit | Scheduled |

## 7. IDE Configuration

### VS Code
- Extension: rust-analyzer
- Settings: `rust-analyzer.checkOnSave: true`
- Format: rustfmt on save

### IntelliJ Rust
- Plugin: Rust
- Edition: Ultimate
