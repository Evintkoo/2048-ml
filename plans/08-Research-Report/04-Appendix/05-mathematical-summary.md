# Plan 05 — Mathematical Summary: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for mathematical summary.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

> **Not a duplicate of 04.** This is optional mathematical context. It does not define the primary AutoML architecture contribution. Only statements retained in the final thesis with verified assumptions may be described as theorems.

1. **Score bound** — `S_max ≤131072` loose upper bound (§1.1). 2. **Max tile** — `2^15=32768` via 16-cell capacity (§1.2). 3. **PAC** — `m≈18k` for (0.01,0.05) with d=28, so 10k ≈(0.016,0.05) (§1.5). 4. **Bootstrap CI** — width `≈20` at n=10k/σ512 (§1.7). 5. **Entropy** — board ≤65.4 bits, spawn 0.469/spawn → ~23.5/game (§1.8 + §3.1).

> **Conjectures (Appendix only):** Feature sufficiency / Markov blanket and 4×4 PSPACE-hardness are conjectures requiring >16 cells — see 04 §1.9 / §4.1 and `00-theoretical-framework.md` §5.

Cross-ref: all proofs/derivations in `04-mathematical-formulation.md`; no copy here.

## Implementation Record

- Summary claims are not experimental evidence; numerical PAC and confidence-width estimates depend on assumptions and must not be presented as measured results. Verify corresponding proof sections before publication.

---

## Verification (definition of done)

1. `test -f plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
2. `grep -q '^# Plan 05 — ' plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
6. `grep -q '^## Open questions$' plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
7. `grep -q '^## Later$' plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/08-Research-Report/04-Appendix/05-mathematical-summary.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
