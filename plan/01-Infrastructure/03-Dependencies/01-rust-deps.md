# Rust Dependencies Specification

## 1. automl Crate Dependencies

The automl submodule provides all ML dependencies. The 2048 project will re-export relevant modules:

```toml
[dependencies]
automl = { path = "automl" }
ndarray = "0.16"
polars = { version = "0.46", features = ["lazy", "csv", "json"] }
rand = "0.8"
rand_chacha = "0.3"
```

## 2. Direct Dependencies for 2048 Engine

| Crate | Version | Purpose |
|-------|---------|---------|
| `rand` | 0.8 | Random tile placement |
| `rand_chacha` | 0.3 | Deterministic RNG |
| `clap` | 4.4 | CLI for game simulation |
| `serde` | 1.0 | State serialization |
| `serde_json` | 1.0 | Data persistence |
| `ndarray` | 0.16 | Board state arrays |
| `rayon` | 1.10 | Parallel game simulation |
| `tokio` | 1.43 | Async simulation |
| `thiserror` | 2.0 | Error types |
| `anyhow` | 1.0 | Error handling |
| `tracing` | 0.1 | Logging |
| `tracing-subscriber` | 0.3 | Log formatting |
| `indicatif` | 0.17 | Progress bars |
| `chrono` | 0.4 | Timestamps |
| `uuid` | 1.7 | Session IDs |

## 3. Dependency Constraints

| Constraint | Rationale |
|------------|-----------|
| Rust 1.75+ | automl requires this minimum |
| No `unsafe` | Memory safety guarantee |
| Send + Sync | Thread-safe game engine |
| `#![deny(warnings)]` | Strict compilation |

## 4. Cargo Workspace Structure

```mermaid
flowchart TD
    root[2048-ml/] --> Cargo[Cargo.toml<br/>Workspace root]
    root --> automl[automl/<br/>Submodule]
    root --> game[game-engine/<br/>2048 game engine]
    root --> data[data-collector/<br/>Data collection]
    root --> trainer[trainer/<br/>Training pipeline]
    root --> bench[benchmark/<br/>Benchmarking]
    
    automl --> autoCargo[Cargo.toml]
    automl --> autoSrc[src/]
    game --> gameCargo[Cargo.toml]
    game --> gameSrc[src/]
    data --> dataCargo[Cargo.toml]
    data --> dataSrc[src/]
    trainer --> trainCargo[Cargo.toml]
    trainer --> trainSrc[src/]
    bench --> benchCargo[Cargo.toml]
    bench --> benchSrc[src/]
```

## 5. Dependency Audit

Run `cargo audit` weekly to check for security vulnerabilities in dependencies.

## 6. Lock File Management

- `Cargo.lock` must be committed for reproducible builds
- `automl/Cargo.lock` must also be committed
- Update dependencies only during scheduled maintenance windows
