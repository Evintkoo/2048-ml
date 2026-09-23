# Data Versioning — Git Hash + Seed

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
