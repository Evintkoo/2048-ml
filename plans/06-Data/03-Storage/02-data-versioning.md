# Plan 02 — Data Versioning: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for data versioning.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Reproduce any dataset from code + seed. No enterprise VNum/ChangeLog.

## 2. Scheme

Dataset version = `git rev-parse HEAD` (commit hash) + collection `seed` + row count. Tag data dirs as `data-v<short-hash>-seed<seed>`.

```bash
git rev-parse HEAD          # → abc1234… — pin this
sha256sum 06-Data/03-Storage/*.csv  # verify after fetch
# reproduce:
cargo run -- collect --seed 42 --n_games 10000  # same hash + seed → same 20k rows
```

 chronological `GroupKFold` (`shuffle=false`, `groups=game_id`) guarantees the 70/15/15 split is reproducible given the same hash+seed ordering. Store `sha256` of each CSV alongside for integrity.

## Implementation Record

- Collector manifests include source revision when Git metadata is available, seed derivation/range, row count, and SHA-256 for training and metadata CSVs. Data files are not committed/version-tagged automatically; reproducibility requires retaining the manifest and source revision.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
