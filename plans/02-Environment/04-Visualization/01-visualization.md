# Plan 01 — Visualization (Debug/Inspection Only): the repository status is explicit and evidence based

> **Status: PARTIAL.** Result serialization and collection manifests exist; terminal charts and SVG exports are not implemented.

**Goal:** State the current implementation and evidence boundary for visualization (debug/inspection only).
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as partial or pending work, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Result serialization and collection manifests exist; terminal charts and SVG exports are not implemented.

## 1. Scope

Visualization is limited to debugging and data inspection purposes. No web-based frontend is used per project constraints.

## 2. Terminal Visualization

### 2.1 Board Rendering — Canonical

> **Canonical `render_board` — single source.** `02-Environment/01-Game/04-game-ui.md` is duplicate, see here; identical for debug only.

```mermaid
flowchart TD
    subgraph "2048 Board"
        C00["(0,0)"] --- C01["(0,1)"]
        C01 --- C02["(0,2)"]
        C02 --- C03["(0,3)"]
        C10["(1,0)"] --- C11["(1,1)"]
        C11 --- C12["(1,2)"]
        C12 --- C13["(1,3)"]
        C20["(2,0)"] --- C21["(2,1)"]
        C21 --- C22["(2,2)"]
        C22 --- C23["(2,3)"]
        C30["(3,0)"] --- C31["(3,1)"]
        C31 --- C32["(3,2)"]
        C32 --- C33["(3,3)"]
    end
```

```rust
fn render_board(board: &Board) -> String {
    let mut output = String::new();
    output.push_str("+------+------+------+------+\n");
    for row in &board.grid {
        output.push_str("|");
        for cell in row {
            match cell {
                Some(val) => output.push_str(&format!("{:>5}|", val)),
                None => output.push_str("     |"),
            }
        }
        output.push_str("\n+------+------+------+------+\n");
    }
    output.push_str(&format!("Score: {}\n", board.score));
    output
}
```

### 2.2 Score Chart (Text-based)

```rust
fn render_score_chart(scores: &[u64]) -> String {
    // ASCII bar chart of score progression
}
```

### 2.3 Move Sequence Visualization

```rust
fn render_moves(moves: &[Direction], scores: &[u64]) -> String {
    // Timeline of moves with score changes
}
```

## 3. Data Export Visualization

### 3.1 SVG Board States

```rust
fn export_board_svg(board: &Board, path: &str) -> Result<()> {
    // Generate SVG file for board visualization
}
```

### 3.2 Training Progress Chart

```rust
fn export_training_chart(metrics: &[TrainingMetric], path: &str) -> Result<()> {
    // Generate chart showing score over training iterations
}
```

## 4. Dashboard Data (JSON)

```rust
pub struct DashboardData {
    pub current_game: Option<GameState>,
    pub statistics: GameStatistics,
    pub model_performance: ModelPerformance,
    pub benchmark_results: BenchmarkResults,
}

// Serialized to JSON for external visualization tools
```

## 5. Not Required

Per the project scope:
- No interactive web dashboard
- No real-time visualization
- No browser-based UI
- No mobile visualization app

All visualization outputs are static files (SVG, CSV, JSON) for external analysis tools.

## Implementation Record

- `GameResult` serializes to JSON and collection/evaluation commands write CSV plus JSON manifests for offline inspection.
- Terminal board rendering, ASCII score charts, move timelines, SVG board export, and training-progress charts are not implemented. They are optional inspection outputs and are not used by the headless simulation pipeline. The game UI plan explicitly keeps rendering out of the MVP.
- Status: partial; no web or interactive visualization is part of the project.

---

## Verification (definition of done)

1. `test -f plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/04-Visualization/01-visualization.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/04-Visualization/01-visualization.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Result serialization and collection manifests exist; terminal charts and SVG exports are not implemented. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
