# Plan 06 — Reproducibility Package: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Root source, lockfile, seeded CLI workflows, and run manifests exist; Docker, public datasets, full study artifacts, and independent reproduction remain absent.

**Goal:** State the current implementation and evidence boundary for reproducibility package.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This package describes implemented reproducibility support and missing study artifacts.** A locked dependency graph and JSON manifest support traceability. The AutoML revision is pinned to a published submodule commit; no end-to-end research replication package exists.

## 1. Purpose

This section specifies the reproducibility package required to replicate the Rust-native AutoML framework evaluation and the 2048 case-study results. The checklist records requirements and must not claim completion until each artifact has been independently verified.

## 2. Code Repository

### 2.1 Repository Structure

The source-audited layout is documented in `01-code-reference.md`. The root is a single Rust crate with an AutoML Git submodule. There are no `framework_benchmarks/`, Python `analysis/`, `paper/`, or generated `results/` trees in the current repository.

### 2.2 Version Control

Record the root commit, dirty-file state or source hashes, `Cargo.lock` hash, and AutoML submodule commit with every published run. The current manifests include source/dependency metadata when available; the local repository is not itself a published artifact release.


## 3. Data and Run Artifacts

No canonical training corpus, public data repository, or DOI has been published. The collector writes CSV training rows, row-aligned game metadata, checkpoints, and JSON manifests. Baseline commands write per-game CSV and JSON manifests. Retain these artifacts outside Git as appropriate and record their checksums and source revision.

Each run record should capture command/configuration, root revision and dirty state, submodule revision and dirty state, `Cargo.lock` digest, compiler/target, seeds by role, thread count, input and output hashes, elapsed time, row/game counts, and analysis procedure. These fields do not by themselves make a result independently replicated.

## 4. Environment and Experiment Commands

The root manifest declares Rust 1.75 as its minimum, but the actual compiler must be recorded per run. A Dockerfile/Compose environment and Python requirements file are not present. The root workflow is CSV-based; do not assume the current project pipeline uses Parquet.

Implemented CLI entry points can be discovered with `cargo run -- --help`. Relevant commands include data collection, dataset split, training, baseline/model benchmark, comparison, and score report. Exact option sets are defined by the current binary and can change; preserve the command and help/output versions with artifacts.


## 5. Reproducibility Checklist

Verification checklist — mark an item complete only after the corresponding artifact and independent check exist:
- [ ] A versioned public source release is available
- [x] Rust dependencies are locked in `Cargo.lock` (this does not pin toolchain distribution)
- [ ] Docker environment is provided
- [x] Named UCI data, split manifests, and one fixed diagnostic configuration are retained under `data/framework_validation/` and `reports/framework_validation/`
- [x] 2048 data-generation procedure is available in `cargo run -- data-collector collect --help`
- [x] Run manifests record configured seeds; no complete study seed matrix is established
- [ ] All scripts are executable
- [x] Same-seed predictions/save-load matched across two runs for the retained 3-dataset/5-model matrix on AutoML `82d8483`; broader configurations/seeds/platforms remain unverified
- [x] Comparison and report commands can reproduce summaries from supplied CSV inputs; verify manifests and limitations
- [ ] Figures are generated from raw data
- [ ] Paper references match code versions
- [x] Save/load predictions matched in both retained 3-dataset/5-model runs on AutoML `82d8483`; this does not establish cross-version equivalence

Checked entries refer only to available local procedures. Plan-scale 2048 data, matched framework baselines/resource profiles, versioned releases, and independent replication remain outstanding. The retained one-seed UCI matrix is a narrow repeatability diagnostic, not broad determinism evidence. Manifests record checksums, protocol details, seeds, dependency pin when available, elapsed time, summaries, and result paths; record dirty source state separately.

## Implementation Record

- Root source is one Rust crate with the AutoML submodule pinned to a published commit. Collector and benchmark CLIs emit CSV/JSON artifacts. Docker, public dataset release/DOI, figures, and independent reproduction are absent; nonexistent sample paths were removed.

## 6. Reproducibility Failure Modes

| Failure Mode | Detection | Mitigation |
|-------------|-----------|------------|
| Dependency version mismatch | CI pipeline | Pinned versions in Cargo.lock |
| Seed/environment differences | Record actual compiler, target, and seed roles; cross-platform equivalence is unverified |
| Data corruption | Checksum verification | Automated integrity checks |
| Hardware differences | Resource and output variation across machines is unmeasured | Record hardware and runtime conditions |
| Non-deterministic behavior | Re-run with same seed | Seed-based determinism verification |
| Code changes between runs | Git tagging | Commit hash embedded in results |

## 7. Publication Artifacts

Before making a reproducibility claim, provide the exact source revision, clean/dirty source state, submodule state, lockfile and toolchain versions, protocol/configuration, raw data or access instructions, generated manifests, analysis commands, outputs, and known limitations. Docker and public datasets are optional packaging choices, not current artifacts.

When generated, include raw and processed results, figures generated from retained data, and executable statistical analysis.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
2. `grep -q '^# Plan 06 — ' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/06-reproducibility-package.md` exits 0.

## Open questions

- **Independent reproduction remains pending.** Retain versioned code/data/configuration and analysis artifacts, and document access and resource requirements for each study.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
