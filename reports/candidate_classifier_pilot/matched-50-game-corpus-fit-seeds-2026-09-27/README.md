# Fit-seed sensitivity on two 50-game corpora

This diagnostic repeats each of the five AutoML candidate fits at five fit seeds
on each of two retained, same-size 50-game corpora. The final chronological 10
game groups are held out for classifier-label diagnostics; the first 40 groups
are used for grouped cross-validation and fitting. The held-out labels are not
used to fit the models or choose their settings. This is classifier-label
evidence, not policy-score evidence or a confirmatory ranking.

## Inputs and protocol

- Previous corpus: [`collection_pilots/2026-09-27-50-game-followup/`](../../../collection_pilots/2026-09-27-50-game-followup/README.md), fit seeds 90652–90656.
- Independent corpus: [`collection_pilots/2026-09-27-50-game-independent/`](../../../collection_pilots/2026-09-27-50-game-independent/README.md), fit seeds 90702–90706.
- Both use five-fold grouped CV and a 0.8 chronological development fraction.
- The same fit seed offsets are aligned across corpora, but each corpus has a distinct data set and seed range.
- AutoML revision: `82d848323eed5e2af86d046d529916c448f2442c`.

Run from the repository root:

```sh
python3 scripts/run_matched_50_game_corpus_fits.py
python3 scripts/verify_matched_50_game_corpus_fits.py
```

The runner records input, model, manifest, and prediction hashes. The verifier
rechecks the 17-feature schema, group split and row identities, artifacts, and
held-out accuracy, macro-F1, and confusion matrices. Its summary is
`fit-seed-verification.json`; the complete run inventory is
`fit-seed-runs.json`.

## Results

Each cell below gives mean ± sample SD across five fit seeds for holdout
accuracy and macro-F1. Grouped-CV accuracy is the mean of five per-fit grouped-CV
means. The fixed-corpus holdout summaries are descriptive.

| Candidate | Previous: holdout accuracy | Previous: macro-F1 | Previous: grouped-CV accuracy | Independent: holdout accuracy | Independent: macro-F1 | Independent: grouped-CV accuracy |
|---|---:|---:|---:|---:|---:|---:|
| RandomForest | 0.3006 ± 0.0030 | 0.2883 ± 0.0033 | 0.3147 | 0.3004 ± 0.0028 | 0.3000 ± 0.0032 | 0.2969 |
| ExtraTrees | 0.2797 ± 0.0019 | 0.2078 ± 0.0013 | 0.2939 | 0.2767 ± 0.0057 | 0.2367 ± 0.0062 | 0.2777 |
| AdaBoost | 0.2835 ± 0.0000 | 0.2785 ± 0.0000 | 0.3082 | 0.2885 ± 0.0000 | 0.2878 ± 0.0000 | 0.2849 |
| KNN | 0.2602 ± 0.0000 | 0.2537 ± 0.0000 | 0.2599 | 0.2542 ± 0.0000 | 0.2475 ± 0.0000 | 0.2464 |
| NaiveBayes | 0.2381 ± 0.0000 | 0.1961 ± 0.0000 | 0.2653 | 0.2639 ± 0.0000 | 0.1984 ± 0.0000 | 0.2597 |

The tree ensembles show some fit-seed variation on the fixed holdouts. AdaBoost,
KNN, and NaiveBayes produce identical held-out predictions across these fit
seeds within each corpus. The two-corpus differences remain descriptive: corpus
and fit-seed effects are not separated by two corpora, and no inferential test
or policy-score conclusion follows from these classifier-label metrics. The
broader policy-score diagnostic is reported separately in
[`action-frequency/independent-50-game-seeds-2026-09-27/`](../../../action-frequency/independent-50-game-seeds-2026-09-27/README.md).

The verifier confirms 50 of 50 fits, all source and artifact hashes, exact test
row/game identities, and recomputed held-out metrics. No inferential tests were
performed.
