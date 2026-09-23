# Rust Dependencies Specification

> **Scope:** This file = `Cargo.toml` pins & workspace layout. For automl capability table (TrainingConfig, ModelType, CV) see `01-Project/02-dependencies.md`. 27-dim canonical lives in `03-State/01-Board/02-feature-extraction.md` + `06-Data/02-Format/01-data-schema.md`.

## 1. Root Cargo.toml (Single Crate MVP)

No `automl/Cargo.toml` duplication — versions pinned to automl's manifest for compatibility.

```toml
[dependencies]
automl = { path = "automl" }                    # pinned hash 64f5eda (v1.0.0-138)
ndarray = "0.16"                                # pinned to automl; board arrays
polars = { version = "0.46", features = ["lazy", "csv", "json"] } # DataFrame 27+1
rand = "0.8"                                    # automl-compatible RNG
rand_chacha = "0.3"                             # deterministic seeding
```

## 2. Direct Dependencies for 2048 Engine (MVP)

| Crate | Version | Pin Rationale | 2048 Use |
|-------|---------|---------------|----------|
| `rand` | 0.8 | Matches automl → single RNG graph | Tile placement |
| `rand_chacha` | 0.3 | Matches automl ChaCha8Rng | `with_random_state(42)` reproducibility |
| `clap` | 4.4 | Matches automl | Single binary subcommands |
| `serde` / `serde_json` | 1.0 | Matches automl | State + DataFrame JSON |
| `ndarray` | 0.16 | Matches automl | Board 4×4 fallback arrays |
| `rayon` | 1.10 | Matches automl | Parallel batch simulation |
| `thiserror` | 2.0 | Matches automl | Error types |
| `anyhow` | 1.0 | Matches automl | Error handling |
| `tracing` | 0.1 | Matches automl | Logging |
| `tracing-subscriber` | 0.3 | Matches automl | Log formatting |
| `indicatif` | 0.17 | Minimal UI | Progress bars |
| `chrono` | 0.4 | — | Timestamps |
| `uuid` | 1.7 | automl dep | Session IDs |

> `tokio` — **not MVP**. automl brings it for its server (out-of-scope per initial-plan). 2048 uses sync `rayon` batch only.

## 3. Constraints

| Constraint | Rationale |
|------------|-----------|
| Rust 1.75+ | automl MSRV |
| No `unsafe` | Memory safety |
| `Send + Sync` | Thread-safe engine via rayon |
| `#![deny(warnings)]` | Strict CI (`cargo clippy -- -D warnings`) |

## 4. Workspace Layout

> **MVP: Single crate — 4-crate workspace is future optional** (game-engine / data-collector / trainer / benchmark).

```
2048-ml/
├── Cargo.toml     # single crate (MVP)
├── src/           # subcommands: game-engine, data-collector, benchmark
└── automl/        # submodule @ 64f5eda
```

The MVP uses one root crate with `src/game_engine/`, `src/data_pipeline/`, and `src/evaluation/` modules. Separate workspace members (`game-engine/`, `data-collector/`, `trainer/`, `benchmark/`) are out of scope until post-MVP.

## 5. Audit & Lockfile

- `cargo audit` — Optional, not MVP (weekly scheduled, not CI-blocking).
- Commit `Cargo.lock` at root; `automl/Cargo.lock` stays in submodule (do not copy).
- Update pins only during scheduled maintenance; bump must keep `rand`/`ndarray` aligned with `automl/Cargo.toml`.
