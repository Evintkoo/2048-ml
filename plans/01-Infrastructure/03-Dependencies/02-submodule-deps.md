# Plan 02 — Submodule Dependency Management: the repository status is explicit and evidence based

> **Status: COMPLETE (2026-09-27).** A determinism/serialization fix is published on a feature branch, pinned by the root repository, clean, and passes all 710 AutoML library tests.

**Goal:** State the current implementation and evidence boundary for submodule dependency management.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan records a verified submodule dependency update, not a framework-performance finding.** The determinism/serialization changes were committed on the AutoML feature branch, pinned in the root repository, and the resulting submodule worktree is clean. Its full library suite passes 710 tests.

> Pinned commit: `88a86bf44a0cb03664931f7ef15201b95fa11255` (`v1.0.0-139-g88a86bf`) — verify with `git submodule status automl`.

## 1. Quickstart

```bash
git clone --recurse-submodules https://github.com/Evintkoo/2048-ml
# or after clone:
git submodule update --init --recursive
git submodule status automl   # must print 88a86bf...
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

- The submodule health gate is complete for the published `88a86bf` pin. Broader framework validation and independent reproducibility remain tracked by the framework contribution and validation tickets.

## Later

- **Update this pin only before a training milestone.** Follow the submodule smoke procedure after every change and keep framework evaluation evidence separate from 2048 case-study results.
