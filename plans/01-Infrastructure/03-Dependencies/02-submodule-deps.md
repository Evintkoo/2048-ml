# Submodule Dependency Management

> Pinned commit: `64f5edad29c9e58ee7d33abf380418d5cfbbb561` — verify with `git submodule status automl`.

## 1. Quickstart

```bash
git clone --recurse-submodules https://github.com/Evintkoo/2048-ml
# or after clone:
git submodule update --init --recursive
git submodule status automl   # must print 64f5eda...
```

## 2. Verification Smoke (run before any training)

```bash
cargo test -p automl --lib -- training::config::tests --nocapture
cargo test -p automl --lib -- training::cross_validation::tests --nocapture
cargo test -p automl  # full submodule suite
# API smoke: TrainingConfig::new(TaskType::MultiClassification, "action")
#            CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
#            MedianPruner::new(false)  # false = maximize
cargo run -- --help   # verify CLI available per 04-Tooling/01-cli-tools.md
```

## 3. Update Cadence (tied to 2048 milestones)

Update **only before a training milestone** (e.g. before data-collection or before training-phase kickoff), never mid-experiment. Monthly at most. Pin hash in `01-Project/01-project-overview.md` §6 on each bump.

```bash
cd automl && git fetch && git checkout <new-hash> && cd ..
git add automl && git commit -m "chore: bump automl → <hash>"
# re-run §2 smoke immediately; if fail → revert: git checkout -- automl && git submodule update --init
```

## 4. Health Check

```bash
test -d automl/.git && echo "OK" || echo "MISSING — run git submodule update --init"
git -C automl rev-parse HEAD              # must match pinned hash
git -C automl status --short              # must be empty (no local edits)
```
