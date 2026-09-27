# Corrected matched-grid run — split seed 2027

Protocol-v2 corrected the validation leak found in the superseded run. The
AutoML fit receives only the per-class inner-fit rows; the selection-validation
and outer-test rows are excluded. The scikit-learn model uses the same
effective rows after AutoML's native internal holdback. The independent
verifier passes all six cases. See the
[three-seed report](../matched-grid-search-corrected-multi-seed-2026-09-27/README.md)
for combined results and reproduction commands.
