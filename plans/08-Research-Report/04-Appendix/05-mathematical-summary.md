# Mathematical Summary — Optional Context (Cross-Ref)

> **Not a duplicate of 04.** This is optional mathematical context. It does not define the primary AutoML architecture contribution. Only statements retained in the final thesis with verified assumptions may be described as theorems.

1. **Score bound** — `S_max ≤131072` loose upper bound (§1.1). 2. **Max tile** — `2^15=32768` via 16-cell capacity (§1.2). 3. **PAC** — `m≈18k` for (0.01,0.05) with d=28, so 10k ≈(0.016,0.05) (§1.5). 4. **Bootstrap CI** — width `≈20` at n=10k/σ512 (§1.7). 5. **Entropy** — board ≤65.4 bits, spawn 0.469/spawn → ~23.5/game (§1.8 + §3.1).

> **Conjectures (Appendix only):** Feature sufficiency / Markov blanket and 4×4 PSPACE-hardness are conjectures requiring >16 cells — see 04 §1.9 / §4.1 and `00-theoretical-framework.md` §5.

Cross-ref: all proofs/derivations in `04-mathematical-formulation.md`; no copy here.
