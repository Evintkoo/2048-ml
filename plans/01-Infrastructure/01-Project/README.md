# 01-Project — plan by plan execution

This index lists ordered tickets, their evidence, and remaining acceptance work.

## The plans

| Plan | Subject | Status |
|---|---|---|
| [01-project-overview](01-project-overview.md) | Rust-Native AutoML Framework | PARTIAL — fixed-configuration diagnostics across three split seeds; matched-budget framework study and 2048 case study pending |
| [02-dependencies](02-dependencies.md) | Dependencies | DONE (2026-09-24) |
| [03-tooling](03-tooling.md) | Tooling Configuration | DONE (2026-09-24) |
| [04-framework-contribution](04-framework-contribution.md) | Rust-Native AutoML Framework Contribution | PARTIAL — fixed-split sklearn baseline, three split seeds, and aggregate resource probe retained; matched comparison/replication pending |

Supporting evidence: [04-framework-architecture](04-framework-architecture.md) records the source-backed architecture and API contract audit for ticket 04; its standalone ledger disposition records that evidence audit as complete.

Scope note: Plan 00 defines 17 training values (16 board cells plus score). Ticket #034 aligned the root encoder and CLI schema with that contract; the former 27-value derived-feature vector is excluded from canonical training.

## Reading paths

1. Read the plan ticket in sequence by its numeric filename.
2. Check its status and evidence before implementing remaining work.
3. Update the root `PLAN-TICKET-LEDGER.md` after each ticket changes.

## Provenance

The Planout writing standard was applied after the ticket audit. Ticket bodies retain their original scope and detail; standardized metadata and acceptance sections make status and completion criteria explicit.
