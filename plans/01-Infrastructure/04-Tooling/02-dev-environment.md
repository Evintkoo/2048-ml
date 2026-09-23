# Development Environment Setup — Quickstart (20 lines)

> **For CLI subcommands see `04-Tooling/01-cli-tools.md`.** This file is setup only.

## 1. Requirements

Rust **1.75+** (automl MSRV), 8 GB RAM, 20 GB disk. macOS 14+ or Ubuntu 22.04+.

## 2. Setup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version  # must be ≥1.75

git clone --recurse-submodules https://github.com/Evintkoo/2048-ml
cd 2048-ml
git submodule update --init --recursive
git submodule status automl  # pinned: 64f5edad...

cargo build
cargo test                   # includes automl smoke (see 03-Dependencies/02-submodule-deps.md §2)
cargo run -- --help          # verify subcommands
```

## 3. Submodule Health Check

```bash
test -d automl/.git && echo "OK" || echo "MISSING"
cargo test -p automl --lib -- training::config
```

If `cargo: command not found` → `source "$HOME/.cargo/env"`.

> Docker, cross-compile, `bench`/`audit`, env-var tables — Optional, not MVP (see `01-Project/03-tooling.md` §3). Not needed for local headless development.
