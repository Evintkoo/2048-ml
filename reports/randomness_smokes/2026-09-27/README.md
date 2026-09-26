# Collector thread-count determinism smoke (2026-09-27)

A bounded same-host check compared collector outputs with identical seeds and
configuration while changing only the Rayon thread count.

| Threads | Games | Rollouts | Training rows | Elapsed seconds |
|---:|---:|---:|---:|---:|
| 1 | 5 | 4,738 | 668 | 4.1843 |
| 4 | 5 | 4,738 | 668 | 2.8719 |

Both training CSVs have SHA-256
`63be237757d654a49c97d0784e44841e4e8e57baed03462f468dbc3e23989e9d`;
both metadata CSVs have SHA-256
`a0c51bb4870eff8a64cc53172c6fc46a7b5a8f49a4825e7fee53c88052e45d07`.
The corresponding manifests record the configuration and hashes, which were
verified against the retained files. The machine used one and four threads,
seed 90728, five games, two rollouts per valid action, and checkpoint interval 2.

This smoke checks only these outputs on this host and build. It does not
establish cross-platform, cross-version, or broad framework reproducibility.
