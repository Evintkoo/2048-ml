# Plan 02 — Game Validation: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for game validation.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Define game validation procedures to ensure the 2048 game engine and rules are correct.

## 2. Game Validation Framework

```mermaid
flowchart TD
    subgraph "Game Validation"
        subgraph "Rule Validation"
            R1[Move Rules]
            R2[Merge Rules]
            R3[Scoring Rules]
            R4[Game Over Rules]
        end
        
        subgraph "State Validation"
            S1[Board State]
            S2[Score State]
            S3[Game State]
        end
        
        subgraph "Edge Case Validation"
            E1[Boundary Cases]
            E2[Special Cases]
            E3[Error Conditions]
        end
        
        R1 -->|validate| GV[Game Validation]
        R2 --> GV
        R3 --> GV
        R4 --> GV
        S1 --> GV
        S2 --> GV
        S3 --> GV
        E1 --> GV
        E2 --> GV
        E3 --> GV
        GV --> R[Validation Report]
    end
```

## 3. Game Rule Validation

```mermaid
graph TD
    A[Move Valid?] --> B[Up/Down/Left/Right]
    B --> C[Tiles Slide Correctly]
    C --> D[Merge Occurs When Equal]
    D --> E[Score Updated Correctly]
    E --> F[New Tile Spawns]
    F --> G[Game Over Detected]
    
    style G fill:#9f9,stroke:#333
```

## 4. Validation Test Matrix

| Rule | Test Case | Expected Result |
|------|-----------|----------------|
| Merge | Two 2 tiles merge | One 4 tile |
| Score | 2+2 merge | Score += 4 |
| Game Over | Full board, no moves | Game Over |
| Spawn | Empty cell available | New tile appears |

## 5. Game State Validation

```mermaid
flowchart TD
    A[Board State] --> B{4×4 Grid?}
    B -->|Yes| C{Values are Powers of 2?}
    C -->|Yes| D{Score Matches Merges?}
    D -->|Yes| E{Game Over Correct?}
    E -->|Yes| F[State Valid]
    E -->|No| G[State Invalid]
    C -->|No| G
    B -->|No| G
```

## 6. Edge Case Validation

```mermaid
mindmap
  root((Edge Cases))
    Board Boundaries
      Corner merges
      Edge sliding
    Game Boundaries
      Score overflow
      Maximum tile
      Full board without game over
    Move Boundaries
      No valid moves available
      Single valid move
      All tiles same value
```

## 7. Validation Pipeline

```mermaid
flowchart LR
    A[Define Tests] --> B[Execute Tests]
    B --> C[Validate Rules]
    C --> D[Validate States]
    D --> E[Validate Edge Cases]
    E --> F{All Valid?}
    F -->|Yes| G[Game Validated ✓]
    F -->|No| H[Fix Issues]
    H --> A
```

## 8. Validation Metrics

```rust
pub struct GameValidationResult {
    pub rules_passed: usize,
    pub rules_failed: usize,
    pub edge_cases_passed: usize,
    pub edge_cases_failed: usize,
    pub overall_score: f64,
    pub validation_date: DateTime<Utc>,
}
```

## 9. Validation Reporting

```mermaid
graph TD
    A[Raw Results] --> B[Calculate Pass Rate]
    B --> C[Identify Failures]
    C --> D[Root Cause Analysis]
    D --> E[Generate Report]
    E --> F[Recommend Fixes]
```

## 10. Continuous Validation

Game validation runs automatically:
- On every code change
- Before each release
- After each configuration update
- As part of CI pipeline

## Implementation Record

The root game engine has automated tests for merge/scoring rules, no-op behavior, game-over detection, tile validation, deterministic spawning and spawn frequency, directional movement, and 500 seeded random boards × all four actions. These are implementation tests, not an independent manual audit. The 90/10 frequency test checks 10,000 spawns against a broad three-sigma interval.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
2. `grep -q '^# Plan 02 — ' plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/02-Validation/02-game-validation.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/02-Validation/02-game-validation.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
