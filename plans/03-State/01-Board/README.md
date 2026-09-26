# 01-Board — plan by plan execution

This index lists ordered tickets, their evidence, and remaining acceptance work.

## The plans

| Plan | Subject | Status |
|---|---|---|
| [01-board-state](01-board-state.md) | Board State Definition | DONE — canonical 17-value input; 35 root tests pass |
| [02-feature-extraction](02-feature-extraction.md) | Feature Extraction | NOT APPLICABLE — expanded candidate metrics are outside Plan 00's fixed 17-value core unless separately approved |
| [03-state-encoding](03-state-encoding.md) | State Encoding | COMPLETE — canonical v2 CSV and provenance sidecar implemented; root tests pass; Parquet outside current storage contract |

## Reading paths

1. Read tickets and child series in repository-relative lexicographic order.
2. Check its status and evidence before implementing remaining work.
3. Update the root `PLAN-TICKET-LEDGER.md` after each ticket changes.

## Provenance

The Planout writing standard was applied after the ticket audit. Ticket bodies retain their original scope and detail; standardized metadata and acceptance sections make status and completion criteria explicit.
