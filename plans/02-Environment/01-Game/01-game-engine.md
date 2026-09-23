# Plan 01 — 2048 Game Engine: the repository status is explicit and evidence based

> **Status: DONE (2026-09-24).** Added validated reusable simulator for random and policy play, game histories/results, and deterministic batch seed derivation; root suite passes.

**Goal:** State the current implementation and evidence boundary for 2048 game engine.
**Builds on:** [00](../../00-scope-and-traceability.md) — the project is supervised 4×4 2048 policy learning, and framework evaluation is a separate research track.

---

## Decision and evidence

**This plan treats its subject as implemented with bounded evidence, not as a research finding.** The rejected alternative is to infer completion from a plan title or related code alone. The ledger records this disposition: Added validated reusable simulator for random and policy play, game histories/results, and deterministic batch seed derivation; root suite passes.

## 1. Purpose

Define the headless 2048 simulation engine that generates supervised `([f64;27] → u8)` training data via Evintkoo/automl `TaskType::MultiClassification`. No UI/web — headless only.

## 2. Game Rules Summary (Thin)

Thin summary only; **See canonical:** `02-Rules/01-scoring-rules.md`, `02-Rules/02-win-lose-conditions.md`, `02-Rules/03-valid-moves.md`.

- 4×4 grid, start = 2 tiles spawned `2` 90% / `4` 10%
- Slide all tiles in direction → merge equal neighbours once per tile per move → score += merged value
- After each valid move spawn `2`/`4` (90/10) in random empty cell via `ChaCha8Rng` (**See: `03-Simulation-Engine/02-randomness.md` seed hygiene**)
- Terminal when board full AND `would_change(dir) == false` for all 4 dirs (**heuristic baseline ~512** for later eval)

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
    pub fn from_u8(v: u8) -> Self { match v { 0=>Self::Up, 1=>Self::Down, 2=>Self::Left, 3=>Self::Right, _=>panic!("invalid action") } }
}
/// Supervised label: `action: u8` in 0..=3 — see 04-Actions/01-Action/01-action-space.md. TaskType::MultiClassification.
```

### 4.2 Move Execution (via would_change)

```rust
impl Board {
    /// Canonical validity — also see 02-Rules/03-valid-moves.md
    pub fn would_change(&self, dir: Direction) -> bool { /* clone + slide without spawn */ true }
    pub fn get_valid_moves(&self) -> Vec<Direction> {
        Direction::ALL.iter().copied().filter(|d| self.would_change(*d)).collect()
    }
    pub fn execute_move(&mut self, dir: Direction) -> MoveResult {
        // 1. slide/merge via slide_left + transforms (see 03-board-representation.md)
        // 2. returns { changed: bool, score_gained: u64 }
        // 3. caller spawns tile only if changed
        todo!()
    }
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
    // TrainingSample produced: ([f64;27], action: u8, score: u64 metadata) — see 03-Simulation-Engine/01-simulation-engine.md
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
| Parallelism | `rayon` par_iter over games (see 03-Simulation-Engine/03-multi-game.md) — thread count fixed for determinism | — |
| Precomputed move tables / bitboard / SIMD | **Optional, not MVP** — mark out-of-scope; profile after 10k baseline works | Precomputed tables, bitboard u64, SIMD batch — add only if >2× gain measured |
| Serialization | `[f64;27]` + `u8` + `u64` score to Parquet/CSV — see 06-Data | — |

## 8. Implementation Record

Implemented in `src/game_engine/mod.rs`: fixed-size validated board state, four `Direction` encodings, pure slide transforms, checked merge scoring, valid-move detection, seeded 90/10 tile spawning, reusable `SimulatorConfig`/`GameSimulator`, random and policy-driven execution, and per-game simulation entrypoints. The model and heuristic policies now use the shared simulator. The random simulator is reproducible across repeated fixed-seed calls; illegal policy moves return an error. Root tests cover move rules, spawn behavior, configuration validation, and simulation determinism.

Detailed per-merge history, complete trajectory records, and a standalone simulation configuration file are not part of the current MVP implementation. `game_over` records the terminal no-move state; the optional move cap remains a safety limit.

## 9. Cross-References

- **RNG / seed hygiene (canonical):** `03-Simulation-Engine/02-randomness.md`
- **Board transforms:** `01-Game/03-board-representation.md` (grid normalization `/32768`)
- **Scoring / win-lose / valid-moves (canonical):** `02-Rules/01-scoring-rules.md`, `02-win-lose-conditions.md`, `03-valid-moves.md`
- **Training row:** `03-Simulation-Engine/01-simulation-engine.md` — `TrainingSample { state_features:[f64;27], action:u8, score:u64 metadata }`, `TaskType::MultiClassification`
- **Features (27-dim):** `03-State/01-Board/01-board-state.md` (16 raw + 11 derived, score at index 21 `/6.0`)
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
