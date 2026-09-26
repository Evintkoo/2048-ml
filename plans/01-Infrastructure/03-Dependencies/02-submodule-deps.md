# Plan 02 — Submodule Dependency Management: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** Determinism and tie-handling fixes are published on AutoML feature branches, pinned by the root repository, clean, and pass all 712 AutoML library tests.

**Goal:** State the current implementation and evidence boundary for submodule dependency management.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan records a verified submodule dependency update, not a framework-performance finding.** Determinism, serialization, and tie-handling changes were committed and pushed on AutoML feature branches, pinned in the root repository, and the resulting submodule worktree is clean. Its full library suite passes 712 tests. The repeated standard-dataset diagnostic also passes 15/15 cases and matches predictions 15/15 across two runs; framework comparison evidence remains limited.

> Pinned commit: `82d848323eed5e2af86d046d529916c448f2442c` (`v1.0.0-140-g82d8483`, branch `fix/deterministic-tie-breaking`) — verify with `git submodule status automl`.

## 1. Quickstart

```bash
git clone --recurse-submodules https://github.com/Evintkoo/2048-ml
# or after clone:
git submodule update --init --recursive
git submodule status automl   # must print 82d8483...
```

## 2. Verification Smoke (run before any training)

```bash
cargo test --manifest-path automl/Cargo.toml --lib -- training::config::tests --nocapture
cargo test --manifest-path automl/Cargo.toml --lib -- training::cross_validation::tests --nocapture
(cd automl && cargo test --lib)  # full submodule library suite: 712 tests
# API smoke: TrainingConfig::new(TaskType::MultiClassification, "action")
#            CrossValidator::new(CVStrategy::GroupKFold { n_splits: 5 })
#            MedianPruner::new(false)  # false = maximize
cargo run -- --help   # verify CLI available per 04-Tooling/01-cli-tools.md
```

## 3. Update Cadence (tied to 2048 milestones)

Update **only before a training milestone** (e.g. before data-collection or before training-phase kickoff), never mid-experiment. Monthly at most. Pin hash in `01-Project/01-project-overview.md` §6 on each bump.

```bash
cd automl && git fetch origin && git checkout <new-hash> && cd ..
git add automl && git commit -m "chore: bump automl → <hash>"
# re-run §2 smoke immediately; if fail → revert: git checkout -- automl && git submodule update --init
```

## 4. Health Check

```bash
git -C automl rev-parse HEAD              # must match pinned hash (submodule .git may be a file)
git -C automl status --short              # must be empty (no local edits)
```

---

## Verification (definition of done)

1. `test -f plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
6. `grep -q '^## Open questions$' plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
7. `grep -q '^## Later$' plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/01-Infrastructure/03-Dependencies/02-submodule-deps.md` exits 0.

## Open questions

- The submodule health gate is complete for the published `82d8483` pin. Broader matched comparisons, resource profiling, and independent reproducibility remain tracked by the framework contribution and validation tickets.

## Later

- **Update this pin only before a training milestone.** Follow the submodule smoke procedure after every change and keep framework evaluation evidence separate from 2048 case-study results.
