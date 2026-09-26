# Plan 03 — References: the repository status is explicit and evidence based

> **Status: PARTIAL (2026-09-26).** Manifest references are source-checked; academic bibliography and primary-source claim verification remain pending.

**Goal:** State the current implementation and evidence boundary for references.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This appendix identifies software/dependency sources only.** It is not a verified academic bibliography, and manifest versions do not establish the exact locally executed toolchain or modified submodule state.

> **This appendix holds only non-lit refs. Literature refs live in `01-IMRD/00-literature-review.md`.**

See `01-IMRD/00-literature-review.md` for provisional academic source leads. Exact records and claim support have not been fully verified.

**Appendix-only non-literature refs:**

- AutoML is a local path Git submodule; `automl/Cargo.toml` declares package version `1.0.0`. Record the exact submodule commit and any local modifications.
- Root `Cargo.toml` declares minimum Rust `1.75`, Polars `0.46`, and `rand_chacha 0.3`; record the actual compiler used for each experiment.
- AutoML manifest dependencies include Polars `0.46`, smartcore `0.3`, and linfa `0.7`. Their presence does not prove each model path is functional.
- Optimizer implementations include TPE-related sampler/search APIs and pruner types; root tuning integration is limited and its pruning API remains unavailable.

> **Deleted:** "All links accessible" unverified claim — links marked verified/unverified in literature-review.

## Implementation Record

- This is a redirect/specification for non-literature dependencies. Manifest locations and declared versions were checked. Academic references remain unverified, and actual runtime/submodule provenance must accompany each study result.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
2. `grep -q '^# Plan 03 — ' plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/03-references.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/03-references.md` exits 0.

## Open questions

- **Bibliography verification remains open.** Verify primary records and connect each consequential claim to the source that supports it.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
