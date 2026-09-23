# 2048-ML

Research project for integrating and evaluating a Rust-native AutoML framework through supervised policy learning for the 4×4 game 2048.

The repository is organized around two connected pieces of work:

- **AutoML framework:** the [`automl`](automl) Git submodule contains the Rust framework being developed and validated.
- **2048 case study:** this repository is the home for the game environment, policy data pipeline, integration, and evaluation tooling. These components are planned work; they are not implemented at the root yet.

The framework is the primary research contribution. Results from 2048 are application evidence and will be reported separately from framework validation. See [plans/00-scope-and-traceability.md](plans/00-scope-and-traceability.md) for the canonical scope.

## Repository status

This repository currently contains the research scope and the AutoML framework submodule. The 2048 implementation and evaluation pipeline have not yet been added.

## Getting started

Initialize or refresh the framework submodule:

```bash
git submodule update --init --recursive
```

Build and run the framework from its directory (requires Rust 1.75 or newer):

```bash
cd automl
cargo build
cargo run -- --help
```

The framework's own [README](automl/README.md) documents its CLI, server, library, and development commands. Root-level 2048 setup instructions will be added with the implementation.

## Research principles

- Train the 2048 policies through the AutoML framework; keep external ML libraries to explicitly described comparison baselines.
- Record seeds, data, configurations, dependency versions, and analysis artifacts for reported results.
- Treat framework validation and 2048 application evaluation as separate evidence.
- Report limitations and inconclusive results alongside positive results.

## Project documents

- [Canonical scope and traceability](plans/00-scope-and-traceability.md)
- [AutoML framework](https://github.com/Evintkoo/automl) (checked out at `automl/`)
