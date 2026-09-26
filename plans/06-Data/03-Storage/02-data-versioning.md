# Plan 02 — Data Versioning: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Collection manifests include seeds, revision, row count, and file hashes; automatic immutable dataset versioning is absent.

**Goal:** State the current implementation and evidence boundary for data versioning.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats run provenance as partially implemented and dataset lifecycle/versioning as pending.** Collection manifests store source revision when available, pinned AutoML commit, seed derivation, configuration, row count, runtime, and CSV/metadata hashes. Outputs are not automatically named, retained, or registered as immutable dataset versions.

## 1. Purpose

Reproduce any dataset from code + seed. No enterprise VNum/ChangeLog.

## 2. Scheme

A reproducibility record should include source revision, dependency revisions, configuration, seeds, output row count, and content hashes. The current manifest records much of this metadata but does not create immutable version tags.

```bash
git rev-parse HEAD          # → abc1234… — pin this
sha256sum 06-Data/03-Storage/*.csv  # verify after fetch
# reproduce:
cargo run -- data-collector collect --seed 42 --n-games 10000  # exact output also depends on pinned dependencies/configuration
```

The split command partitions whole games chronologically; grouped CV preserves game groups but does not itself impose time order. File hashes are stored in collection manifests.

## Implementation Record

- Collector manifests include source revision when Git metadata is available, the AutoML pin, configuration, seed derivation/range, row count, runtime, and SHA-256 for training and metadata CSVs.
- Data files are not committed/version-tagged automatically. Reproduction also depends on retaining the data, manifest, root/submodule revisions, toolchain, and command configuration.

---

## Verification (definition of done)

1. `test -f plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/03-Storage/02-data-versioning.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/03-Storage/02-data-versioning.md` exits 0.

## Open questions

- Add automated toolchain/dependency capture and a stable dataset-version registry only if needed for the reproduction package. Retain artifact bytes alongside manifests.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
