# Plan 01 — Rust Dependencies Specification: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Added sha2 and clarified single-crate domain module layout; root manifest and lockfile present.

**Goal:** State the current implementation and evidence boundary for rust dependencies specification.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Added sha2 and clarified single-crate domain module layout; root manifest and lockfile present.

> **Scope:** This file = `Cargo.toml` constraints & workspace layout. For AutoML capability table (TrainingConfig, ModelType, CV) see `01-Project/02-dependencies.md`. Plan 00 defines the canonical 17-value training input; ticket #034 aligns the root implementation with it.

## 1. Root Cargo.toml (Single Crate MVP)

No `automl/Cargo.toml` duplication — versions pinned to automl's manifest for compatibility.

```toml
[dependencies]
automl = { path = "automl" }                    # pinned hash 64f5eda (v1.0.0-138)
ndarray = "0.16"                                # pinned to automl; board arrays
polars = { version = "0.46", features = ["lazy", "csv", "json"] } # current DataFrame; canonical schema is 17+1
rand = "0.8"                                    # automl-compatible RNG
rand_chacha = "0.3"                             # deterministic seeding
sha2 = "0.10"                                   # SHA-256 artifact and dataset manifests
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
| `sha2` | 0.10 | — | SHA-256 checksums in data and benchmark manifests |

> `tokio` — **not MVP**. automl brings it for its server (out-of-scope per initial-plan). 2048 uses sync `rayon` batch only.

## 3. Constraints

| Constraint | Rationale |
|------------|-----------|
| Rust 1.75+ | automl MSRV |
| No `unsafe` | Memory safety |
| `Send + Sync` | Thread-safe engine via rayon |
| `unsafe_code = "forbid"`; Clippy `all = "deny"` | Root manifest lint policy; `cargo clippy -- -D warnings` is a local check, not configured CI |

## 4. Workspace Layout

> **MVP: Single crate — 4-crate workspace is future optional** (game-engine / data-collector / trainer / benchmark).

```
2048-ml/
├── Cargo.toml     # single crate (MVP)
├── src/           # root CLI and cohesive game, state, action, model, data, evaluation modules
└── automl/        # submodule @ 64f5eda
```

The MVP uses one root crate with cohesive Rust domain modules such as `src/game_engine/`, `src/data_pipeline.rs`, and `src/evaluation.rs`; source files need not reproduce the numbered `plans/` hierarchy. Separate workspace members (`game-engine/`, `data-collector/`, `trainer/`, `benchmark/`) are out of scope until post-MVP.

## 5. Audit & Lockfile

- `cargo audit` — Optional, not MVP; manual invocation only, no schedule configured.
- Commit `Cargo.lock` at root; `automl/Cargo.lock` stays in submodule (do not copy).
- Update pins only during scheduled maintenance; bump must keep `rand`/`ndarray` aligned with `automl/Cargo.toml`.

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/03-Dependencies/01-rust-deps.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Added sha2 and clarified single-crate domain module layout; root manifest and lockfile present. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
