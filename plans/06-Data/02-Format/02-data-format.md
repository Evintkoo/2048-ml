# Data Format — Storage Formats (CSV Canonical)

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
