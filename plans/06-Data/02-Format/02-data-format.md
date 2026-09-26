# Plan 02 — Data Format: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** CSV is the implemented training format with a separate metadata sidecar; Parquet remains unsupported.

**Goal:** State the current implementation and evidence boundary for data format.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats canonical CSV serialization as implemented and Parquet as unimplemented.** The collector writes feature/action CSV plus a separate row-aligned provenance CSV; the training CLI consumes CSV and metadata paths.

## 1. Purpose

Define the single canonical storage format and one optional alternative.

## 2. Canonical — CSV (28 cols)

Training CSV is **exactly 28 columns**: 27 features + `action: u8` label. `score: u64` is optional trailing metadata, never `y`.

```csv
grid_0,...,row_worst,action
0.0,0.00006,0.00012,0.00024,...,0.03,2
```
Header regex: `^grid_0,grid_1,...,row_worst,action(\,score)?$` (see `03-data-standard.md`). No `done`/`reward`/`next_state`.

## 3. Parquet (Not Implemented)

The root does not currently write or read Parquet. The following old example is retained only as a possible future format sketch, not a working implementation:

```rust
use polars::prelude::*;
fn write_parquet(states: &[[f64;27]], actions: &[u8], path: &str) -> PolarsResult<()> {
    let cols: Vec<Column> = (0..27).map(|i| {
        let v: Vec<f64> = states.iter().map(|s| s[i]).collect();
        Column::new(format!("f{i}").into(), v)
    }).chain(std::iter::once(Column::new("action".into(), actions.to_vec()))).collect();
    let mut df = DataFrame::new(cols)?;
    let file = std::fs::File::create(path)?;
    ParquetWriter::new(file).finish(&mut df)
}
```

CSV is the only supported training-data format. JSON is used for manifests/checkpoints, not as a training table. Parquet support would require a separately implemented and validated ingestion/export path.

## Implementation Record

- CSV training rows and aligned metadata are implemented and validated end-to-end. Parquet read/write support is absent.

---

## Verification (definition of done)

1. `test -f plans/06-Data/02-Format/02-data-format.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/06-Data/02-Format/02-data-format.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/06-Data/02-Format/02-data-format.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/06-Data/02-Format/02-data-format.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/06-Data/02-Format/02-data-format.md` exits 0.
6. `grep -q '^## Open questions$' plans/06-Data/02-Format/02-data-format.md` exits 0.
7. `grep -q '^## Later$' plans/06-Data/02-Format/02-data-format.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/06-Data/02-Format/02-data-format.md` exits 0.

## Open questions

- Add Parquet only if scale measurements justify it; any new format must preserve feature order, action typing, and sidecar row alignment.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
