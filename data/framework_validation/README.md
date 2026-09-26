# Framework validation datasets

These source archives implement the initial named-dataset matrix in
[`plans/07-Benchmarking/03-Comparison/04-framework-validation.md`](../../plans/07-Benchmarking/03-Comparison/04-framework-validation.md).
The files are downloaded unchanged from the UCI Machine Learning Repository;
the extracted `.data` files and UCI-provided descriptions are retained beside
the source archives. No preprocessing or train/test split has been applied.

| Dataset | UCI record | Rows | Predictors | Target | License | Archive SHA-256 |
|---|---|---:|---:|---|---|---|
| Iris | [53](https://archive.ics.uci.edu/dataset/53/iris) | 150 | 4 numeric | Iris species | CC BY 4.0 | `d11fe30213d36434a0879aab7cb00ce3c812eb7ba2495874438abff7b7b762e9` |
| Wine recognition | [109](https://archive.ics.uci.edu/dataset/109/wine) | 178 | 13 numeric | Cultivar (3 classes) | CC BY 4.0 | `2bae62c4481220623579d4c4fb36b55652b6b75e06e49fa1981b8198362dfdab` |
| Breast Cancer Wisconsin (Diagnostic) | [17](https://archive.ics.uci.edu/dataset/17/breast+cancer+wisconsin+diagnostic) | 569 | 30 numeric | Diagnosis (benign/malignant) | CC BY 4.0 | `bc154869ef13f753f9e2b5a17e248cfe1ba4b6721db7c4da9f4880e40b05d3af` |

UCI dataset pages describe the source, variables, citation information, and
CC BY 4.0 licensing. Attribution is retained here and in the linked records.
These datasets are used only to check the general tabular classification
framework. They do not model the 2048 game and cannot establish 2048 policy
quality. Conversely, game simulations assess the policy in its intended game
environment but do not show how the framework behaves on unrelated tabular
tasks.

Acquisition URLs:

- `https://archive.ics.uci.edu/static/public/53/iris.zip`
- `https://archive.ics.uci.edu/static/public/109/wine.zip`
- `https://archive.ics.uci.edu/static/public/17/breast+cancer+wisconsin+diagnostic.zip`

The acquired source data are used by the fixed-split AutoML diagnostic and its
comparison-only scikit-learn baseline. Split manifests, metrics, predictions,
timings, and run provenance live under `reports/framework_validation/`; the
baseline protocol and limitations are summarized in that report. The results
are diagnostic evidence, not a framework superiority claim.
