# Agent Guide

## Repository overview

This is a research project integrating the Rust-native AutoML framework in `automl/` with supervised policy learning for 4×4 2048. The root repository contains the game environment, policy data pipeline, integration, and evaluation tooling. `automl/` is a Git submodule and contains its own framework implementation and documentation.

## Source of truth

- Follow [`plans/00-scope-and-traceability.md`](plans/00-scope-and-traceability.md) for research scope and interpretation of results.
- Keep framework validation distinct from the 2048 application case study. Game scores do not establish general AutoML superiority.
- The canonical training state is the 16 board cells plus current score (17 values); do not add move count or game history as training features without a separately documented study.
- Core 2048 model training uses the AutoML framework. External ML libraries are for explicitly described comparison baselines.
- Record seeds, data, configurations, dependency versions, and analysis artifacts for reported results; report limitations and inconclusive results.

## Working in this repository

- Treat every Markdown file under [`plans/`](plans/) as one self-contained ticket. Execute tickets in strict order: first `plans/00-scope-and-traceability.md`, then all remaining files sorted lexicographically by their repository-relative path. Do not jump to a later ticket while any earlier applicable ticket is incomplete.
- For each ticket, read the entire file and its cited/canonical dependencies, inspect the current implementation, implement its deliverables, perform the validation requested by that ticket, update its evidence/status and affected docs, then record it as complete, partial, blocked by a specific prerequisite, or not applicable with a scope-based reason. Do not silently skip tickets. A redirect/deprecation/out-of-scope note still requires reading and recording the ticket disposition; it does not disappear from the sequence.
- Maintain [PLAN-TICKET-LEDGER.md](PLAN-TICKET-LEDGER.md) at the repository root. It lists every plan Markdown file in execution order with its disposition and evidence. Update it as tickets are processed so progress survives context changes and restarts.
- The numbered plan hierarchy is an execution/backlog order, not a required source-directory mirror. Follow the architecture/layout specified by the relevant ticket. Keep Rust modules organized by cohesive domain in the single root crate; infrastructure deliverables such as manifests, docs, and CI belong at their conventional repository paths. Add a numbered plan folder under `src/` only if a ticket explicitly requires it and the Rust architecture supports it.
- If a ticket depends on a later ticket, do not violate file order: implement only prerequisite-safe work, record the dependency, and revisit the ticket when its prerequisite has been completed. Do not mark it complete until its acceptance criteria are met.
- At the beginning of a new execution request, resume at the earliest ticket not fully complete according to `PLAN-TICKET-LEDGER.md`. If the user explicitly says restart, audit the ledger and actual files from the first ticket again; retain correct existing implementation but re-verify each ticket in order. Never interpret restart as permission to skip earlier files because their code appears to exist.
- Work through tickets in the plan hierarchy and honor explicit dependencies, redirects, deprecated/out-of-scope notes, and status statements in each plan. The numbered folders give the broad progression from infrastructure and environment through state, actions, modeling, data, benchmarking, reporting, and quality; they are guidance, not permission to ignore a ticket's stated prerequisites.
- Before coding a ticket, check whether its requirements conflict with `plans/00-scope-and-traceability.md` or other canonical plans. The scope document governs; resolve conflicts by following it and update the affected plan when appropriate.
- Complete one ticket at a time where practical. A ticket is done when its stated deliverables and acceptance criteria are implemented, relevant documentation is updated, and requested validation/evidence is recorded. If a plan is underspecified or contradictory, make only the safe, scope-consistent progress possible and document the gap rather than inventing requirements.
- Keep implementation and plan status aligned: update a plan to reflect completed work when it tracks status, and do not mark future work complete just because a related component exists.
- Keep root-level case-study work separate from changes to the `automl/` submodule. When changing the submodule, follow its own repository guidance and avoid unrelated framework edits.
- Initialize the submodule when needed with `git submodule update --init --recursive`.
- The framework requires Rust 1.75 or newer. Build it from `automl/` with `cargo build`; its README documents CLI, server, and development workflows.
- Root-level 2048 setup and implementation instructions should be added as those components are introduced.

## Changes and evidence

- Update relevant plans and documentation when implementation changes alter scope, protocols, or feature definitions.
- Do not describe planned root-level components as implemented until they exist.
- Support performance and framework claims with reproducible measurements; do not assume Rust is faster.
- Do not claim all plans are complete while `PLAN-TICKET-LEDGER.md` contains pending, partial, blocked, or unverified tickets. Report those dispositions explicitly when handing work back.
- After the strict plan-ticket execution loop is complete, inspect the user's referenced plan-writing standard at `/Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans` and revise this repository's plan documents to match it. Track that standards pass as a separate final task; do not let it reorder or interrupt the ticket loop.
