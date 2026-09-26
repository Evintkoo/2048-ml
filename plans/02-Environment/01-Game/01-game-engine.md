# Plan 01 — 2048 Game Engine: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Added validated reusable simulator for random and policy play, game histories/results, and deterministic batch seed derivation; root suite passes.

**Goal:** State the current implementation and evidence boundary for 2048 game engine.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The evidence is in `src/game_engine/mod.rs`: validated reusable simulation, deterministic batch seeds, per-move score and merge histories, and result metadata; root tests cover the rules and reproducibility.

## 1. Purpose

Define the headless 2048 simulation engine that generates game trajectories and action records for supervised policy learning. The canonical model input is 17 values under Plan 00, implemented by ticket #034. No UI/web — headless only.

## 2. Game Rules Summary (Thin)

Thin summary only; **See canonical:** `02-Rules/01-scoring-rules.md`, `02-Rules/02-win-lose-conditions.md`, `02-Rules/03-valid-moves.md`.

- 4×4 grid, start = 2 tiles spawned `2` 90% / `4` 10%
- Slide all tiles in direction → merge equal neighbours once per tile per move → score += merged value
- After each valid move spawn `2`/`4` (90/10) in random empty cell via `ChaCha8Rng` (**See: `03-Simulation-Engine/02-randomness.md` seed hygiene**)
- Terminal when no direction changes the board. Baseline scores are measured under the declared benchmark protocol, not fixed here.

## 3. Engine Architecture

```mermaid
flowchart TD
    Board[Board [u32;16] + score/move_count/game_over<br/>Canonical: 03-State/01-Board/01-board-state.md]
    RNG[ChaCha8Rng<br/>Seed Hygiene: 03-Simulation-Engine/02-randomness.md]
    MoveProc[Move Processor<br/>slide_left + rotate/transpose]
    ScoreTrk[Score Tracker<br/>u64 metadata only — NOT label]
    Ctrl[GameController<br/>valid_moves via would_change]
    Board <--> MoveProc
    RNG --> Board
    MoveProc --> ScoreTrk
    Ctrl --> Board
```

> **Canonical Board:** `03-State/01-Board/01-board-state.md` (`RawBoardState { grid: [u32;16], score: u64, move_count: u64, game_over: bool }`). Do not duplicate struct here.

## 4. Core Operations

### 4.1 Direction & Action Encoding (u8 0–3)

```rust
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction { Up = 0, Down = 1, Left = 2, Right = 3 }

impl Direction {
    pub const ALL: [Direction; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
    pub fn try_from_action(v: u8) -> Result<Self, GameError> { /* reject values outside 0..=3 */ todo!() }
}
/// Supervised label: `action: u8` in 0..=3 — see 04-Actions/01-Action/01-action-space.md. TaskType::MultiClassification.
```

### 4.2 Move Execution (via would_change)

Implemented by `RawBoardState::execute_move`; invalid actions are rejected by
`Direction::try_from_action`. An unchanged move does not alter score or move
count. Successful moves return score gained and merge events with result-cell
coordinates and turn index.

```rust
impl Board {
    /// Canonical validity — also see 02-Rules/03-valid-moves.md
    pub fn would_change(&self, dir: Direction) -> bool { /* clone + slide without spawn */ true }
    pub fn get_valid_moves(&self) -> Vec<Direction> {
        Direction::ALL.iter().copied().filter(|d| self.would_change(*d)).collect()
    }
    pub fn execute_move(&mut self, dir: Direction) -> Result<MoveResult, GameError> { todo!() }
    pub fn is_game_over(&self) -> bool {
        self.get_valid_moves().is_empty()
    }
}
```

### 4.3 Tile Spawn — Seeded RNG (Canonical RNG: 03-Simulation-Engine/02-randomness.md)

```rust
use rand_chacha::ChaCha8Rng;
use rand::{Rng, SeedableRng};

impl Board {
    pub fn spawn_tile(&mut self, rng: &mut ChaCha8Rng, spawn_prob_4: f64) {
        let empties: Vec<usize> = self.grid.iter().enumerate()
            .filter(|(_, &v)| v == 0).map(|(i,_)| i).collect();
        if empties.is_empty() { return; }
        let idx = empties[rng.gen_range(0..empties.len())];
        let val = if rng.gen_bool(spawn_prob_4) { 4 } else { 2 }; // 90/10: prob 0.1 for 4
        self.grid[idx] = val;
    }
}
/// Initial board: call spawn_tile twice with spawn_prob_4 = 0.1
/// Global seed hygiene → TrainingConfig::with_random_state(42) and propagate via wrapping_add — see 03-Simulation-Engine/02-randomness.md
```

### 4.4 Slide Logic — Base op is slide_left

```rust
fn slide_row_left(row: [u32; 4]) -> ([u32; 4], u64) {
    // 1. compress non-zero left
    // 2. merge adjacent equals once (leftward)
    // 3. compress again → new row + score_gained
    todo!()
}
/// Other directions via rotate/transpose — see 03-board-representation.md §3.2
```

## 5. Simulation Engine

```rust
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

pub struct GameSimulator {
    pub board: Board,          // [u32;16] canonical
    pub rng: ChaCha8Rng,       // ChaCha8Rng::seed_from_u64(seed)
    pub config: SimulatorConfig,
}

impl GameSimulator {
    pub fn new(seed: u64, config: SimulatorConfig) -> Self {
        Self { board: Board::new(), rng: ChaCha8Rng::seed_from_u64(seed), config }
    }
    pub fn simulate_random(&mut self) -> GameResult { todo!() }
    pub fn simulate_with_model(&mut self, model: &dyn Model) -> GameResult { todo!() }
    // Canonical TrainingSample: ([f64;17], action: u8, score: u64 metadata).
    // Root collection uses the canonical 17-value state.
}
```

## 6. Configuration

```rust
pub struct SimulatorConfig {
    pub board_size: usize,      // 4 — fixed, not configurable
    pub spawn_prob_4: f64,      // 0.1 — canonical 90/10 (10% fours)
    pub seed: u64,              // ChaCha8Rng seed; propagated via wrapping_add — see 02-randomness.md
    pub max_moves: u64,         // 1000 — early-stop fallback
    pub initial_tiles: usize,   // 2
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self { board_size: 4, spawn_prob_4: 0.1, seed: 42, max_moves: 1000, initial_tiles: 2 }
    }
}
/// Global reproducibility: SimulatorConfig.seed linked to TrainingConfig::with_random_state(42)
/// — see 01-Infrastructure/02-Configuration/02-training-config.md & 03-Simulation-Engine/02-randomness.md
```

## 7. Performance Considerations

| Concern | MVP | Optional (Not MVP) |
|---------|-----|---------------------|
| Board state | `[u32;16]` flat, empty=0, zero-alloc moves | — |
| Parallelism | Each `GameSimulator` is sequential; independent simulations can be parallelized by callers. Seeds derive from game IDs, so results do not depend on scheduling. | — |
| Precomputed move tables / bitboard / SIMD | **Optional, not MVP** — profile only against a declared workload and budget | Precomputed tables, bitboard u64, SIMD batch — add only if a measured benefit justifies the added complexity |
| Serialization | Canonical `[f64;17]` + `u8` + `u64` score to CSV — see 06-Data | — |

## 8. Implementation Record

Implemented in `src/game_engine/mod.rs`: fixed-size validated board state, four `Direction` encodings, pure slide transforms, checked merge scoring, valid-move detection, seeded 90/10 tile spawning, reusable `SimulatorConfig`/`GameSimulator`, random and policy-driven execution, and per-game simulation entrypoints. The model and heuristic policies now use the shared simulator. The random simulator is reproducible across repeated fixed-seed calls; illegal policy moves return an error. Root tests cover move rules, spawn behavior, configuration validation, and simulation determinism.

Merge events, per-turn score totals, and move action/score-delta records are included in `GameResult`. The rollout collector also retains the sampled states needed for relabeling. A separate simulation configuration file is not part of the current MVP; `SimulatorConfig` is currently constructed through code. `game_over` records the terminal no-move state; the optional move cap remains a safety limit.

## 9. Cross-References

- **RNG / seed hygiene (canonical):** `03-Simulation-Engine/02-randomness.md`
- **Board transforms:** `01-Game/03-board-representation.md` (grid normalization `/32768`)
- **Scoring / win-lose / valid-moves (canonical):** `02-Rules/01-scoring-rules.md`, `02-win-lose-conditions.md`, `03-valid-moves.md`
- **Training row:** `03-Simulation-Engine/01-simulation-engine.md` — canonical `TrainingSample` uses 17 values plus action and score metadata.
- **Canonical training input:** Plan 00 and `03-State/04-Encoding/01-state-vector.md` specify 16 board cells plus score (17 values). Board-state fields such as move count remain metadata, not model features.
- **Out-of-scope UI:** Headless only — debug print only in `01-Game/04-game-ui.md` (deprecated stub), canonical viz JSON in `04-Visualization/01-visualization.md`

---

## Verification (definition of done)

1. `test -f plans/02-Environment/01-Game/01-game-engine.md` exits 0.
2. `grep -q '^# Plan 01 — ' plans/02-Environment/01-Game/01-game-engine.md` exits 0.
3. `grep -q '^> \\*\\*Status:' plans/02-Environment/01-Game/01-game-engine.md` exits 0.
4. `grep -q '^\*\*Goal:' plans/02-Environment/01-Game/01-game-engine.md` exits 0.
5. `grep -q '^## Decision and evidence$' plans/02-Environment/01-Game/01-game-engine.md` exits 0.
6. `grep -q '^## Open questions$' plans/02-Environment/01-Game/01-game-engine.md` exits 0.
7. `grep -q '^## Later$' plans/02-Environment/01-Game/01-game-engine.md` exits 0.
8. `bash /Users/evintleovonzko/Documents/works/kolosal/planout2/v2-ai-express/.claude/skills/writing-planout-plans/check-plan.sh plans/02-Environment/01-Game/01-game-engine.md` exits 0.

## Open questions

- **The plan-scale evidence remains bounded by current results.** Added validated reusable simulator for random and policy play, game histories/results, and deterministic batch seed derivation; root suite passes. Any larger corpus or external benchmark needs a declared resource budget and retained artifacts.

## Later

- **Complete the remaining research or implementation work recorded above.** It stays deferred until its prerequisites, compute budget, and measurable acceptance evidence are available.
