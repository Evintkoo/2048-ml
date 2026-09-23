# Plan 01 — CI Pipeline: the repository status is explicit and evidence based

> **Status: PLANNED.** Not yet restarted in strict sequence.

**Goal:** State the current implementation and evidence boundary for ci pipeline.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Not yet restarted in strict sequence.

## 1. Purpose

Define the continuous integration pipeline for the 2048 ML system.

## 2. CI Pipeline Architecture

```mermaid
flowchart TD
    subgraph "CI Pipeline"
        subgraph "Trigger"
            CS[Code Commit]
            PR[Pull Request]
            SC[Scheduled Run]
        end
        
        subgraph "Stages"
            S1[Build]
            S2[Test]
            S3[Lint]
            S4[Validate]
            S5[Deploy]
        end
        
        subgraph "Output"
            R[Build Status]
            T[Test Results]
            QR[Quality Report]
        end
        
        CS --> S1
        PR --> S1
        SC --> S1
        S1 --> S2
        S2 --> S3
        S3 --> S4
        S4 --> S5
        S5 --> R
        S5 --> T
        S5 --> QR
    end
```

## 3. Pipeline Stages

```mermaid
flowchart TD
    A[Code Commit] --> B[Build Stage]
    B --> C[Test Stage]
    C --> D[Lint Stage]
    D --> E[Validate Stage]
    E --> F{All Stages Pass?}
    F -->|Yes| G[Pipeline Success]
    F -->|No| H[Pipeline Failed]
    G --> I[Deploy]
    H --> J[Notify Team]
    J --> K[Fix and Retry]
```

## 4. Stage Details

| Stage | Command | Duration | Failure Action |
|-------|---------|----------|----------------|
| Build | cargo build | 2 min | Block pipeline |
| Test | cargo test | 5 min | Block pipeline |
| Lint | cargo clippy | 1 min | Warning only |
| Validate | Custom scripts | 3 min | Block pipeline |
| Deploy | Automated deploy | 1 min | Manual approval |

## 5. Build Stage

```mermaid
flowchart TD
    A[Checkout Code] --> B[Install Dependencies]
    B --> C[Compile Rust Code]
    C --> D[Build artifacts]
    D --> E[Store Build Output]
    E --> F{Build Success?}
    F -->|Yes| G[Proceed to Test]
    F -->|No| H[Report Build Error]
```

## 6. Test Stage

```mermaid
flowchart TD
    A[Run Unit Tests] --> B[Run Integration Tests]
    B --> C[Run Game Tests]
    C --> D[Run Validation Tests]
    D --> E[Run Performance Tests]
    E --> F{All Tests Pass?}
    F -->|Yes| G[Proceed]
    F -->|No| H[Report Test Failure]
```

## 7. Pipeline Visualization

```mermaid
graph TD
    A[Commit] --> B[Build]
    B --> C[Test]
    C --> D[Lint]
    D --> E[Validate]
    E --> F[Deploy]
    
    style A fill:#9f9,stroke:#333
    style F fill:#9f9,stroke:#333
    style B fill:#9f9,stroke:#333
    style C fill:#9f9,stroke:#333
    style D fill:#9f9,stroke:#333
    style E fill:#9f9,stroke:#333
```

## 8. Pipeline Triggers

```mermaid
mindmap
  root((Pipeline Triggers))
    Push Events
      Direct push to main
      Feature branch push
    Pull Request Events
      PR opened
      PR updated
      PR reviewed
    Scheduled Events
      Nightly build
      Weekly full test
    Manual Triggers
      Manual pipeline run
      Force re-run
```

## 9. Pipeline Configuration

```rust
pub struct CIPipelineConfig {
    pub stages: Vec<Stage>,
    pub timeout_seconds: u64,
    pub retry_count: usize,
    pub notifications: Vec<Notification>,
    pub branches: Vec<String>,
    pub triggers: Vec<Trigger>,
}
```

## 10. Pipeline Reporting

Each pipeline run produces:
- Build status dashboard
- Test coverage report
- Performance metrics
- Code quality scores
- Deployment status

## Current Repository Status

No CI workflow is configured in `.github/workflows/` yet. The local root crate currently passes `cargo fmt -- --check`, `cargo test`, and `cargo clippy -- -D warnings`; clippy emits a pre-existing dead-code warning from the pinned AutoML dependency, while the root crate is clean. CI automation, coverage reporting, performance schedules, and deployment stages remain unimplemented. The illustrative stage tables above are target design, not a live pipeline.

---

## Verification (definition of done)

1. `test -f plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
6. `grep -q '^## Open questions$' plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
7. `grep -q '^## Later$' plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/09-Quality/03-CI/01-ci-pipeline.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Not yet restarted in strict sequence. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
