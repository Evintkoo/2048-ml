# Superseded matched-grid summary

> **INVALIDATED — DO NOT USE THE RESULTS.** The AutoML implementation used
> for the initial seed-42, 2026, and 2027 runs received the external selection
> validation rows in `TrainEngine::fit`, then scored those same rows to select
> configurations. This leaked validation information into fitting. The prior
> claims and summary artifacts in this directory are retained as an audit
> trail only and are withdrawn.

Corrected protocol-v2 runs explicitly exclude external validation rows from
both implementations' fit inputs. They also record the AutoML engine's native
per-class holdback and make scikit-learn use the same effective model-training
rows. See the corrected report at
[`../matched-grid-search-corrected-multi-seed-2026-09-27/README.md`](../matched-grid-search-corrected-multi-seed-2026-09-27/README.md).
