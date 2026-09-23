# Plan 03 — References: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for references.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **This appendix holds only non-lit refs. Literature refs live in `01-IMRD/00-literature-review.md`.**

See `01-IMRD/00-literature-review.md` §7 for all academic citations with verified/unverified marks.

**Appendix-only non-literature refs:**

- automl `v1.0.0` — `https://github.com/Evintkoo/automl` (`automl/Cargo.toml`, `src/training/config.rs`)
- Rust `1.75` — `rust:1.75-slim` (Docker, `Cargo.lock`)
- polars `0.46`, smartcore `0.3`, linfa `0.7` (`automl/Cargo.toml`)
- HyperOptX — bundled `automl/src/optimizer` (TPE + MedianPruner)

> **Deleted:** "All links accessible" unverified claim — links marked verified/unverified in literature-review.

## Implementation Record

- This is a redirect/specification for non-literature dependencies. Dependency versions and locations were checked against manifests, but academic references and linked primary sources still require bibliographic verification.

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

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
