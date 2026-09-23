# Plan 02 — Submodule Dependency Management: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Submodule hash matches pin; full submodule library suite passed 709/709.

**Goal:** State the current implementation and evidence boundary for submodule dependency management.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Submodule hash matches pin; full submodule library suite passed 709/709.

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

- **The plan-scale evidence remains bounded by current results.** Submodule hash matches pin; full submodule library suite passed 709/709. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
