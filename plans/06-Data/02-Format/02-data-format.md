# Plan 02 — Data Format: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for data format.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Define the single canonical storage format and one optional alternative.

## 2. Canonical — CSV (28 cols)

Training CSV is **exactly 28 columns**: 27 features + `action: u8` label. `score: u64` is optional trailing metadata, never `y`.

```csv
grid_0,...,row_worst,action
0.0,0.00006,0.00012,0.00024,...,0.03,2
```
Header regex: `^grid_0,grid_1,...,row_worst,action(\,score)?$` (see `03-data-standard.md`). No `done`/`reward`/`next_state`.

## 3. Optional — Parquet via polars DataFrame

For large datasets, Parquet is optional via `polars`:

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

Parquet is **not** a second pipeline — just an alternative serialization of the same 27+action table. No Binary/JSON pipelines (deleted — out of scope).

## Implementation Record

- CSV is implemented and validated end-to-end. Parquet serialization is not implemented; it remains optional under this plan.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
