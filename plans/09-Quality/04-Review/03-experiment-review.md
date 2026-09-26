# Plan 03 — Experiment Review: the repository status is explicit and evidence based

> **Status: PARTIAL.** Review guidance exists; no confirmatory experiment output or review decision is recorded.

**Goal:** State the current implementation and evidence boundary for experiment review.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**Experiment review evaluates a retained protocol and its outputs, not a proposed threshold table.** The repository has protocol-specific random and heuristic baselines, while confirmatory trained-policy outputs and an independent review decision are absent.

The reviewer checks the question, protocol, seeds, data, implementation, analysis, and conclusion as one evidence chain. A baseline run is interpreted only within its protocol; it cannot establish a trained-policy result or general AutoML performance.

## 1. Review scope binds claims to the experiment that produced them

Define experiment review procedures for validating the 2048 ML research experiments.

## 2. Protocol and outputs form one review unit

The review records the hypothesis, experimental unit, comparison, seed plan, data split, software versions, analysis method, outputs, deviations, and limitations. Missing items are reported as gaps, not assumed satisfied.

## 3. Review checks provenance before interpreting a result

The reviewer verifies that seed, dataset digest, configuration, and dependency provenance match the retained run; checks that train/development/test boundaries were respected; and confirms that conclusions do not exceed measured outcomes.

## 4. Independent review records findings and resolutions

The reviewer records scope, methods, findings, requested changes, resolution, and decision. The repository has no automated experiment-review workflow; a local analysis command is not an approval.

## 5. Review criteria have no universal numeric pass threshold

| Review item | Evidence to inspect |
|-------------|---------------------|
| Reproducibility | Recorded seeds, data, configuration, and dependency provenance |
| Statistical analysis | Methods appropriate to the design, with assumptions and uncertainty reported |
| Effect size | Estimate and uncertainty interpreted in the study context |
| Sample adequacy | Design justification, not a universal fixed game-count threshold |
| Confounds | Known sources of bias and limits documented |

## 6. Validation evidence is distinct from independent replication

Internal tests, seed checks, and manifests support specific reproducibility claims. They do not constitute independent replication by another operator or implementation.

## 7. Conclusions stay within the design and evidence

### 7.1 Methodology Review

- Are hypotheses clearly stated?
- Are variables properly defined?
- Are controls appropriate?
- Is the experimental design sound?

### 7.2 Reproducibility Review

The reviewer asks whether another operator can reconstruct the protocol from retained artifacts. This ticket does not claim that replication has occurred.

### 7.3 Statistical Review

- Are any statistical procedures appropriate to the design and supported by the implementation?
- Are assumptions verified?
- Are confidence intervals reported?
- Is effect size meaningful?

### 7.4 Conclusion Review

- Are conclusions supported by data?
- Are limitations acknowledged?
- Are future directions suggested?
- Are implications discussed?

## 8. Review output names evidence gaps as well as findings

The output contains reviewed run identifiers, claims assessed, supporting artifacts, unresolved issues, and the review decision.

## 9. The decision is reasoned, not calculated from a score

Approval requires material methodological and reporting issues to be resolved. No single p-value, effect-size estimate, or game count automatically determines approval.

## 10. The review record stays with the reported experiment

All experiment reviews are documented with:
- Reviewer comments
- Quality metrics
- Approval status
- Recommendations
- Revision history

## Implementation Record

- Experiment review procedure is documented. Existing random/heuristic action-frequency results are limited to their recorded protocol; trained-policy study outputs and an independent review decision are absent. No fixed p-value, effect-size, or sample-count gate is an approved project rule.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
2. `grep -q '^> \\*\\*Status: PARTIAL' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
3. `grep -q '^\\*\\*Goal:' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
4. `grep -q '^## 5. Review criteria have no universal numeric pass threshold$' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
5. `grep -q 'protocol-specific random and heuristic baselines' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
6. `! grep -q 'p < 0.05\|d ≥ 0.5\|N ≥ 1000' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
7. `grep -q '^## Open questions$' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
8. `grep -q '^## Later$' plans/09-Quality/04-Review/03-experiment-review.md` exits 0.
9. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/04-Review/03-experiment-review.md` exits 0.

## Open questions

- Confirmatory model outputs and an independent reviewer are absent. Running the study costs substantial compute; review remains deferred until an approved resource budget and retained outputs exist.

## Later

- **Independent experiment review remains deferred.** It depends on a completed reproducible experiment package and an independent reviewer.
