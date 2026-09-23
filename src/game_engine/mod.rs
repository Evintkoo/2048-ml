//! Headless 4×4 2048 game rules and deterministic game execution.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub fn try_from_action(action: u8) -> Result<Self, GameError> {
        match action {
            0 => Ok(Self::Up),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Right),
            _ => Err(GameError::InvalidAction(action)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawBoardState {
    pub grid: [u32; 16],
    pub score: u64,
    pub move_count: u64,
    pub game_over: bool,
}

impl Default for RawBoardState {
    fn default() -> Self {
        Self::new()
    }
}

impl RawBoardState {
    pub const fn new() -> Self {
        Self {
            grid: [0; 16],
            score: 0,
            move_count: 0,
            game_over: false,
        }
    }

    pub fn from_grid(grid: [u32; 16]) -> Result<Self, GameError> {
        if let Some((index, value)) = grid
            .iter()
            .copied()
            .enumerate()
            .find(|(_, value)| *value != 0 && !value.is_power_of_two())
        {
            return Err(GameError::InvalidTile { index, value });
        }
        Ok(Self {
            grid,
            ..Self::new()
        })
    }

    pub fn with_seed(seed: u64, spawn_prob_4: f64) -> Result<(Self, ChaCha8Rng), GameError> {
        validate_spawn_probability(spawn_prob_4)?;
        let mut board = Self::new();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        board.spawn_tile(&mut rng, spawn_prob_4)?;
        board.spawn_tile(&mut rng, spawn_prob_4)?;
        Ok((board, rng))
    }

    pub fn is_full(&self) -> bool {
        !self.grid.contains(&0)
    }

    pub fn would_change(&self, direction: Direction) -> bool {
        slide(self.grid, direction).is_ok_and(|(grid, _)| grid != self.grid)
    }

    pub fn get_valid_moves(&self) -> Vec<Direction> {
        Direction::ALL
            .into_iter()
            .filter(|&direction| self.would_change(direction))
            .collect()
    }

    pub fn valid_mask(&self) -> [bool; 4] {
        let mut mask = [false; 4];
        for direction in Direction::ALL {
            mask[direction as usize] = self.would_change(direction);
        }
        mask
    }

    pub fn is_game_over(&self) -> bool {
        !Direction::ALL
            .into_iter()
            .any(|direction| self.would_change(direction))
    }

    pub fn execute_move(&mut self, direction: Direction) -> Result<MoveResult, GameError> {
        let (grid, gained, merged) = slide_detailed(self.grid, direction)?;
        let changed = grid != self.grid;
        if changed {
            let score = self
                .score
                .checked_add(gained)
                .ok_or(GameError::ScoreOverflow)?;
            self.grid = grid;
            self.score = score;
            self.move_count = self
                .move_count
                .checked_add(1)
                .ok_or(GameError::MoveCountOverflow)?;
            self.game_over = self.is_game_over();
        }
        Ok(MoveResult {
            changed,
            score_gained: if changed { gained } else { 0 },
            merges: if changed {
                merged
                    .into_iter()
                    .map(|(row, col, tile_value)| MergeEvent {
                        turn: self.move_count,
                        tile_value: tile_value as u64,
                        position: (row, col),
                        score_gained: tile_value as u64,
                    })
                    .collect()
            } else {
                Vec::new()
            },
        })
    }

    pub fn spawn_tile(
        &mut self,
        rng: &mut ChaCha8Rng,
        spawn_prob_4: f64,
    ) -> Result<Option<(usize, u32)>, GameError> {
        validate_spawn_probability(spawn_prob_4)?;
        let empties: Vec<usize> = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| (value == 0).then_some(index))
            .collect();
        if empties.is_empty() {
            return Ok(None);
        }
        let index = empties[rng.gen_range(0..empties.len())];
        let value = if rng.gen_bool(spawn_prob_4) { 4 } else { 2 };
        self.grid[index] = value;
        Ok(Some((index, value)))
    }

    pub fn max_tile(&self) -> u32 {
        self.grid.iter().copied().max().unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoveResult {
    pub changed: bool,
    pub score_gained: u64,
    pub merges: Vec<MergeEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeEvent {
    pub turn: u64,
    pub tile_value: u64,
    pub position: (usize, usize),
    pub score_gained: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreTracker {
    pub total_score: u64,
    pub merge_history: Vec<MergeEvent>,
    pub turn_scores: Vec<u64>,
}

impl ScoreTracker {
    pub fn record_move(&mut self, result: &MoveResult) {
        if result.changed {
            self.total_score += result.score_gained;
            self.turn_scores.push(result.score_gained);
            self.merge_history.extend_from_slice(&result.merges);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameResult {
    pub seed: u64,
    pub final_score: u64,
    pub max_tile: u32,
    pub move_count: u64,
    pub board: RawBoardState,
    pub score_tracker: ScoreTracker,
    pub move_history: Vec<MoveRecord>,
    pub win_condition: WinCondition,
    pub duration_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveRecord {
    pub action: u8,
    pub score_delta: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WinCondition {
    NotWon,
    Won2048,
    Won4096,
    Won8192,
}

impl WinCondition {
    pub fn from_max_tile(max_tile: u32) -> Self {
        if max_tile >= 8192 {
            Self::Won8192
        } else if max_tile >= 4096 {
            Self::Won4096
        } else if max_tile >= 2048 {
            Self::Won2048
        } else {
            Self::NotWon
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulatorConfig {
    pub board_size: usize,
    pub spawn_prob_4: f64,
    pub seed: u64,
    pub max_moves: u64,
    pub initial_tiles: usize,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self {
            board_size: 4,
            spawn_prob_4: 0.1,
            seed: 42,
            max_moves: 1000,
            initial_tiles: 2,
        }
    }
}

/// Stateful simulator shared by random, heuristic, and model-driven games.
/// Every stochastic decision is made by the game's seeded ChaCha8 RNG.
pub struct GameSimulator {
    pub board: RawBoardState,
    rng: ChaCha8Rng,
    pub config: SimulatorConfig,
    pub score_tracker: ScoreTracker,
    move_history: Vec<MoveRecord>,
    started_at: Instant,
}

impl GameSimulator {
    pub fn new(config: SimulatorConfig) -> Result<Self, GameError> {
        if config.board_size != 4 {
            return Err(GameError::UnsupportedBoardSize(config.board_size));
        }
        if config.initial_tiles != 2 {
            return Err(GameError::InvalidInitialTileCount(config.initial_tiles));
        }
        if config.max_moves == 0 {
            return Err(GameError::InvalidMoveLimit);
        }
        validate_spawn_probability(config.spawn_prob_4)?;
        let (board, rng) = RawBoardState::with_seed(config.seed, config.spawn_prob_4)?;
        Ok(Self {
            board,
            rng,
            config,
            score_tracker: ScoreTracker::default(),
            move_history: Vec::new(),
            started_at: Instant::now(),
        })
    }

    pub fn simulate_random(&mut self) -> GameResult {
        while !self.board.is_game_over() && self.board.move_count < self.config.max_moves {
            let valid = self.board.get_valid_moves();
            let direction = valid[self.rng.gen_range(0..valid.len())];
            let result = self
                .board
                .execute_move(direction)
                .expect("valid board move should fit score");
            self.score_tracker.record_move(&result);
            self.move_history.push(MoveRecord {
                action: direction as u8,
                score_delta: result.score_gained,
            });
            self.board
                .spawn_tile(&mut self.rng, self.config.spawn_prob_4)
                .expect("spawn probability was validated");
        }
        self.final_result()
    }

    pub fn simulate_with_policy<F>(&mut self, mut select_move: F) -> Result<GameResult, GameError>
    where
        F: FnMut(&RawBoardState) -> Result<Direction, GameError>,
    {
        while !self.board.is_game_over() && self.board.move_count < self.config.max_moves {
            let direction = select_move(&self.board)?;
            if !self.board.would_change(direction) {
                return Err(GameError::InvalidPolicyMove(direction));
            }
            let result = self.board.execute_move(direction)?;
            self.score_tracker.record_move(&result);
            self.move_history.push(MoveRecord {
                action: direction as u8,
                score_delta: result.score_gained,
            });
            self.board
                .spawn_tile(&mut self.rng, self.config.spawn_prob_4)?;
        }
        Ok(self.final_result())
    }

    fn final_result(&mut self) -> GameResult {
        self.board.game_over = self.board.is_game_over();
        GameResult {
            seed: self.config.seed,
            final_score: self.board.score,
            max_tile: self.board.max_tile(),
            move_count: self.board.move_count,
            board: self.board,
            score_tracker: self.score_tracker.clone(),
            move_history: self.move_history.clone(),
            win_condition: WinCondition::from_max_tile(self.board.max_tile()),
            duration_ms: self.started_at.elapsed().as_millis().min(u64::MAX as u128) as u64,
        }
    }
}

pub fn simulate_random_game(seed: u64, spawn_prob_4: f64, max_moves: u64) -> GameResult {
    let mut simulator = GameSimulator::new(SimulatorConfig {
        seed,
        spawn_prob_4,
        max_moves,
        ..SimulatorConfig::default()
    })
    .expect("the configured game simulator must be valid");
    simulator.simulate_random()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrainingSample {
    pub state_features: [f64; 27],
    pub action: u8,
    pub score: u64,
    pub game_id: u64,
    pub move_index: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordedGame {
    pub result: GameResult,
    pub samples: Vec<TrainingSample>,
    pub states: Vec<RawBoardState>,
}

pub fn collect_random_game(
    game_id: u64,
    seed: u64,
    spawn_prob_4: f64,
    max_moves: u64,
) -> RecordedGame {
    let started_at = Instant::now();
    let (mut board, mut rng) = RawBoardState::with_seed(seed, spawn_prob_4)
        .expect("the configured spawn probability must be valid");
    let mut samples = Vec::new();
    let mut states = Vec::new();
    let mut score_tracker = ScoreTracker::default();
    let mut move_history = Vec::new();
    while !board.is_game_over() && board.move_count < max_moves {
        let valid = board.get_valid_moves();
        if valid.is_empty() {
            break;
        }
        let state_features = crate::state::BoardStateMl::from_board(&board).0;
        states.push(board);
        let direction = valid[rng.gen_range(0..valid.len())];
        let move_index = board.move_count;
        samples.push(TrainingSample {
            state_features,
            action: direction as u8,
            score: board.score,
            game_id,
            move_index,
        });
        let move_result = board
            .execute_move(direction)
            .expect("valid board move should fit score");
        score_tracker.record_move(&move_result);
        move_history.push(MoveRecord {
            action: direction as u8,
            score_delta: move_result.score_gained,
        });
        board
            .spawn_tile(&mut rng, spawn_prob_4)
            .expect("spawn probability was validated");
    }
    board.game_over = board.is_game_over();
    let result = GameResult {
        seed,
        final_score: board.score,
        max_tile: board.max_tile(),
        move_count: board.move_count,
        board,
        score_tracker,
        move_history,
        win_condition: WinCondition::from_max_tile(board.max_tile()),
        duration_ms: started_at.elapsed().as_millis().min(u64::MAX as u128) as u64,
    };
    RecordedGame {
        result,
        samples,
        states,
    }
}

/// Run a numbered range of random games. Game seeds depend only on the global
/// seed and game ID, so results are stable across sequential or parallel runs.
pub fn simulate_random_batch(
    first_game_id: u64,
    n_games: usize,
    global_seed: u64,
    spawn_prob_4: f64,
    max_moves: u64,
) -> Result<Vec<GameResult>, GameError> {
    validate_spawn_probability(spawn_prob_4)?;
    (0..n_games)
        .map(|offset| {
            let game_id = first_game_id.wrapping_add(offset as u64);
            let seed = global_seed.wrapping_add(game_id);
            let mut simulator = GameSimulator::new(SimulatorConfig {
                seed,
                spawn_prob_4,
                max_moves,
                ..SimulatorConfig::default()
            })?;
            Ok(simulator.simulate_random())
        })
        .collect()
}

pub fn relabel_game(
    game: &mut RecordedGame,
    labeler: &RolloutLabeler,
    base_seed: u64,
) -> Result<(), GameError> {
    for (sample, board) in game.samples.iter_mut().zip(&game.states) {
        let (action, _) = labeler.label(board, base_seed, sample.game_id, sample.move_index)?;
        sample.action = action;
    }
    Ok(())
}

pub fn rollout_score_from(
    initial: RawBoardState,
    first_action: Direction,
    seed: u64,
    spawn_prob_4: f64,
    max_moves: u64,
) -> Result<u64, GameError> {
    validate_spawn_probability(spawn_prob_4)?;
    let mut board = initial;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let first = board.execute_move(first_action)?;
    if !first.changed {
        return Ok(board.score);
    }
    board.spawn_tile(&mut rng, spawn_prob_4)?;
    while !board.is_game_over() && board.move_count < max_moves {
        let valid = board.get_valid_moves();
        if valid.is_empty() {
            break;
        }
        let direction = valid[rng.gen_range(0..valid.len())];
        board.execute_move(direction)?;
        board.spawn_tile(&mut rng, spawn_prob_4)?;
    }
    Ok(board.score)
}

#[derive(Clone, Copy, Debug)]
pub struct RolloutLabeler {
    pub n_rollouts: usize,
    pub max_moves: u64,
    pub spawn_prob_4: f64,
}

impl Default for RolloutLabeler {
    fn default() -> Self {
        Self {
            n_rollouts: 100,
            max_moves: 1000,
            spawn_prob_4: 0.1,
        }
    }
}

impl RolloutLabeler {
    pub fn label(
        &self,
        board: &RawBoardState,
        base_seed: u64,
        game_id: u64,
        move_index: u64,
    ) -> Result<(u8, f64), GameError> {
        validate_spawn_probability(self.spawn_prob_4)?;
        if self.n_rollouts == 0 {
            return Err(GameError::InvalidRolloutCount);
        }
        let valid = board.get_valid_moves();
        let mut best_action = None;
        let mut best_mean = f64::NEG_INFINITY;
        for direction in valid {
            let mut total = 0.0;
            for rollout in 0..self.n_rollouts {
                let seed = base_seed
                    .wrapping_add(game_id.wrapping_mul(1_000_000))
                    .wrapping_add(move_index.wrapping_mul(1_000))
                    .wrapping_add(direction as u64)
                    .wrapping_add((rollout as u64).wrapping_mul(4));
                total +=
                    rollout_score_from(*board, direction, seed, self.spawn_prob_4, self.max_moves)?
                        as f64;
            }
            let mean = total / self.n_rollouts as f64;
            if mean > best_mean {
                best_mean = mean;
                best_action = Some(direction as u8);
            }
        }
        best_action
            .map(|action| (action, best_mean))
            .ok_or(GameError::NoValidMoves)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GameError {
    #[error("action must be in 0..=3, got {0}")]
    InvalidAction(u8),
    #[error("tile at index {index} is not zero or a power of two: {value}")]
    InvalidTile { index: usize, value: u32 },
    #[error("spawn probability must be finite and in [0, 1]")]
    InvalidSpawnProbability,
    #[error("score overflow")]
    ScoreOverflow,
    #[error("move count overflow")]
    MoveCountOverflow,
    #[error("rollout count must be greater than zero")]
    InvalidRolloutCount,
    #[error("no valid moves on terminal board")]
    NoValidMoves,
    #[error("unsupported board size {0}; only 4x4 is supported")]
    UnsupportedBoardSize(usize),
    #[error("initial tile count must be exactly two, got {0}")]
    InvalidInitialTileCount(usize),
    #[error("max move limit must be greater than zero")]
    InvalidMoveLimit,
    #[error("policy selected invalid move {0:?}")]
    InvalidPolicyMove(Direction),
    #[error("model inference failed: {0}")]
    ModelInference(String),
}

pub fn validate_spawn_probability(probability: f64) -> Result<(), GameError> {
    if probability.is_finite() && (0.0..=1.0).contains(&probability) {
        Ok(())
    } else {
        Err(GameError::InvalidSpawnProbability)
    }
}

type DetailedRowSlide = ([u32; 4], u64, Vec<(usize, u32)>);
type DetailedGridSlide = ([u32; 16], u64, Vec<(usize, usize, u32)>);

pub fn slide_row_left(row: [u32; 4]) -> Result<([u32; 4], u64), GameError> {
    let (output, score, _) = slide_row_left_detailed(row)?;
    Ok((output, score))
}

fn slide_row_left_detailed(row: [u32; 4]) -> Result<DetailedRowSlide, GameError> {
    let mut compact = [0_u32; 4];
    let mut len = 0;
    for value in row.into_iter().filter(|&value| value != 0) {
        if !value.is_power_of_two() {
            return Err(GameError::InvalidTile { index: len, value });
        }
        compact[len] = value;
        len += 1;
    }

    let mut output = [0_u32; 4];
    let mut read = 0;
    let mut write = 0;
    let mut score = 0_u64;
    let mut merges = Vec::new();
    while read < len {
        if read + 1 < len && compact[read] == compact[read + 1] {
            let merged = compact[read]
                .checked_mul(2)
                .ok_or(GameError::ScoreOverflow)?;
            score = score
                .checked_add(merged as u64)
                .ok_or(GameError::ScoreOverflow)?;
            output[write] = merged;
            merges.push((write, merged));
            read += 2;
        } else {
            output[write] = compact[read];
            read += 1;
        }
        write += 1;
    }
    Ok((output, score, merges))
}

fn transpose(grid: [u32; 16]) -> [u32; 16] {
    let mut output = [0; 16];
    for row in 0..4 {
        for col in 0..4 {
            output[col * 4 + row] = grid[row * 4 + col];
        }
    }
    output
}

fn reverse_rows(mut grid: [u32; 16]) -> [u32; 16] {
    for row in 0..4 {
        grid[row * 4..row * 4 + 4].reverse();
    }
    grid
}

fn slide_left(grid: [u32; 16]) -> Result<([u32; 16], u64), GameError> {
    let (output, score, _) = slide_left_detailed(grid)?;
    Ok((output, score))
}

fn slide_left_detailed(grid: [u32; 16]) -> Result<DetailedGridSlide, GameError> {
    let mut output = [0; 16];
    let mut score = 0_u64;
    let mut merges = Vec::new();
    for row in 0..4 {
        let start = row * 4;
        let (next, gained, row_merges) = slide_row_left_detailed([
            grid[start],
            grid[start + 1],
            grid[start + 2],
            grid[start + 3],
        ])?;
        output[start..start + 4].copy_from_slice(&next);
        score = score.checked_add(gained).ok_or(GameError::ScoreOverflow)?;
        merges.extend(row_merges.into_iter().map(|(col, value)| (row, col, value)));
    }
    Ok((output, score, merges))
}

pub fn slide(grid: [u32; 16], direction: Direction) -> Result<([u32; 16], u64), GameError> {
    match direction {
        Direction::Left => slide_left(grid),
        Direction::Right => slide_left(reverse_rows(grid)).map(|(g, s)| (reverse_rows(g), s)),
        Direction::Up => slide_left(transpose(grid)).map(|(g, s)| (transpose(g), s)),
        Direction::Down => {
            slide_left(reverse_rows(transpose(grid))).map(|(g, s)| (transpose(reverse_rows(g)), s))
        }
    }
}

fn slide_detailed(grid: [u32; 16], direction: Direction) -> Result<DetailedGridSlide, GameError> {
    let (next, score, merges) = match direction {
        Direction::Left => slide_left_detailed(grid)?,
        Direction::Right => {
            let (reversed, score, merges) = slide_left_detailed(reverse_rows(grid))?;
            let positions = merges
                .into_iter()
                .map(|(row, col, value)| (row, 3 - col, value))
                .collect();
            (reverse_rows(reversed), score, positions)
        }
        Direction::Up => {
            let (transposed, score, merges) = slide_left_detailed(transpose(grid))?;
            let positions = merges
                .into_iter()
                .map(|(row, col, value)| (col, row, value))
                .collect();
            (transpose(transposed), score, positions)
        }
        Direction::Down => {
            let (reversed, score, merges) = slide_left_detailed(reverse_rows(transpose(grid)))?;
            let positions = merges
                .into_iter()
                .map(|(row, col, value)| (3 - col, row, value))
                .collect();
            (transpose(reverse_rows(reversed)), score, positions)
        }
    };
    Ok((next, score, merges))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row_board(row: [u32; 4]) -> RawBoardState {
        let mut grid = [0; 16];
        grid[..4].copy_from_slice(&row);
        RawBoardState::from_grid(grid).unwrap()
    }

    #[test]
    fn slide_merges_each_tile_at_most_once() {
        assert_eq!(slide_row_left([2, 2, 4, 4]).unwrap(), ([4, 8, 0, 0], 12));
        assert_eq!(slide_row_left([2, 2, 2, 0]).unwrap(), ([4, 2, 0, 0], 4));
        assert_eq!(slide_row_left([2, 0, 4, 0]).unwrap(), ([2, 4, 0, 0], 0));
    }

    #[test]
    fn merge_history_tracks_result_positions_and_turn_score_for_all_directions() {
        for (direction, expected_position) in [
            (Direction::Left, (0, 0)),
            (Direction::Right, (0, 3)),
            (Direction::Up, (0, 0)),
            (Direction::Down, (3, 0)),
        ] {
            let mut grid = [0; 16];
            grid[0] = 2;
            match direction {
                Direction::Left | Direction::Right => grid[1] = 2,
                Direction::Up | Direction::Down => grid[4] = 2,
            }
            let mut board = RawBoardState::from_grid(grid).unwrap();
            let result = board.execute_move(direction).unwrap();
            assert_eq!(result.score_gained, 4, "{direction:?}");
            assert_eq!(result.merges.len(), 1, "{direction:?}");
            assert_eq!(
                result.merges[0].position, expected_position,
                "{direction:?}"
            );
            assert_eq!(result.merges[0].turn, 1);
            let mut tracker = ScoreTracker::default();
            tracker.record_move(&result);
            assert_eq!(tracker.total_score, 4);
            assert_eq!(tracker.turn_scores, [4]);
            assert_eq!(tracker.merge_history, result.merges);
        }
    }

    #[test]
    fn moves_work_in_all_directions() {
        let board = row_board([2, 0, 0, 0]);
        let mut right = board;
        right.execute_move(Direction::Right).unwrap();
        assert_eq!(&right.grid[..4], &[0, 0, 0, 2]);

        let mut up =
            RawBoardState::from_grid([0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        up.execute_move(Direction::Up).unwrap();
        assert_eq!(up.grid[0], 2);
    }

    #[test]
    fn unchanged_moves_do_not_increment_score_or_count() {
        let mut board = row_board([2, 4, 8, 16]);
        let before = board;
        let result = board.execute_move(Direction::Left).unwrap();
        assert!(!result.changed);
        assert_eq!(board, before);
    }

    #[test]
    fn full_board_with_merge_is_not_over() {
        let board = RawBoardState::from_grid([
            2, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 2,
        ])
        .unwrap();
        assert!(board.is_full());
        assert!(!board.is_game_over());
        let terminal =
            RawBoardState::from_grid([2, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2]).unwrap();
        assert!(terminal.is_game_over());
    }

    #[test]
    fn seeded_spawn_is_reproducible_and_probability_checked() {
        let (mut a, mut rng_a) = RawBoardState::with_seed(42, 0.1).unwrap();
        let (mut b, mut rng_b) = RawBoardState::with_seed(42, 0.1).unwrap();
        for _ in 0..30 {
            a.spawn_tile(&mut rng_a, 0.1).unwrap();
            b.spawn_tile(&mut rng_b, 0.1).unwrap();
        }
        assert_eq!(a, b);
        assert!(RawBoardState::with_seed(1, f64::NAN).is_err());
    }

    #[test]
    fn four_spawn_frequency_is_within_three_sigma() {
        let mut board = RawBoardState::new();
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut fours = 0;
        for _ in 0..10_000 {
            board.grid = [0; 16];
            let (_, tile) = board.spawn_tile(&mut rng, 0.1).unwrap().unwrap();
            fours += usize::from(tile == 4);
        }
        assert!((900..=1100).contains(&fours), "observed {fours} four-tiles");
    }

    #[test]
    fn terminal_board_rejects_every_move_as_unchanged() {
        let board =
            RawBoardState::from_grid([2, 4, 2, 4, 4, 2, 4, 2, 2, 4, 2, 4, 4, 2, 4, 2]).unwrap();
        assert!(board.is_game_over());
        for direction in Direction::ALL {
            assert!(!board.would_change(direction));
        }
    }

    #[test]
    fn batch_seeds_are_stable_and_follow_game_ids() {
        let first = simulate_random_batch(0, 4, 42, 0.1, 80).unwrap();
        let second = simulate_random_batch(0, 4, 42, 0.1, 80).unwrap();
        let later = simulate_random_batch(2, 2, 42, 0.1, 80).unwrap();
        assert_eq!(first[0].seed, 42);
        assert_eq!(first[3].seed, 45);
        assert_eq!(first[2].board, later[0].board);
        for (a, b) in first.iter().zip(second) {
            assert_eq!(a.board, b.board);
            assert_eq!(a.move_history, b.move_history);
            assert_eq!(a.score_tracker, b.score_tracker);
        }
    }

    #[test]
    fn every_action_of_a_single_corner_tile_moves_it_to_the_requested_edge() {
        let board =
            RawBoardState::from_grid([0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        let (right, _) = slide(board.grid, Direction::Right).unwrap();
        assert_eq!(&right[..4], &[0, 0, 0, 2]);
        let (left, _) = slide(board.grid, Direction::Left).unwrap();
        assert_eq!(&left[..4], &[2, 0, 0, 0]);
        assert_ne!(left, board.grid);
    }

    #[test]
    fn reusable_simulator_is_seeded_and_policy_moves_are_checked() {
        let config = SimulatorConfig {
            seed: 123,
            max_moves: 50,
            ..SimulatorConfig::default()
        };
        let mut first = GameSimulator::new(config).unwrap();
        let mut second = GameSimulator::new(config).unwrap();
        let a = first.simulate_random();
        let b = second.simulate_random();
        assert_eq!(a.final_score, b.final_score);
        assert_eq!(a.max_tile, b.max_tile);
        assert_eq!(a.move_count, b.move_count);
        assert_eq!(a.board, b.board);
        assert_eq!(a.move_history, b.move_history);
        assert_eq!(a.score_tracker, b.score_tracker);

        let mut simulator = GameSimulator::new(config).unwrap();
        let invalid = simulator.simulate_with_policy(|_| Ok(Direction::Up));
        assert!(matches!(invalid, Err(GameError::InvalidPolicyMove(_))));
    }

    #[test]
    fn simulator_rejects_noncanonical_configuration() {
        assert!(matches!(
            GameSimulator::new(SimulatorConfig {
                board_size: 5,
                ..Default::default()
            }),
            Err(GameError::UnsupportedBoardSize(5))
        ));
        assert!(matches!(
            GameSimulator::new(SimulatorConfig {
                initial_tiles: 1,
                ..Default::default()
            }),
            Err(GameError::InvalidInitialTileCount(1))
        ));
    }

    #[test]
    fn reaching_2048_is_recorded_but_does_not_end_the_game() {
        let mut grid = [0; 16];
        grid[0] = 2048;
        grid[1] = 2;
        let board = RawBoardState::from_grid(grid).unwrap();
        assert!(!board.is_game_over());
        assert_eq!(
            WinCondition::from_max_tile(board.max_tile()),
            WinCondition::Won2048
        );
        assert_eq!(WinCondition::from_max_tile(4096), WinCondition::Won4096);
        assert_eq!(WinCondition::from_max_tile(8192), WinCondition::Won8192);
        assert_eq!(WinCondition::from_max_tile(1024), WinCondition::NotWon);
    }

    #[test]
    fn invalid_actions_and_bad_tiles_are_errors() {
        assert_eq!(
            Direction::try_from_action(4),
            Err(GameError::InvalidAction(4))
        );
        assert!(
            RawBoardState::from_grid([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_err()
        );
    }

    #[test]
    fn would_change_matches_execution_for_deterministic_board_family() {
        for seed in 0..500_u64 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let mut grid = [0; 16];
            for value in &mut grid {
                *value = match rng.gen_range(0..5) {
                    0 => 0,
                    n => 1 << n,
                };
            }
            let board = RawBoardState::from_grid(grid).unwrap();
            for direction in Direction::ALL {
                let mut moved = board;
                let changed = moved.execute_move(direction).unwrap().changed;
                assert_eq!(
                    board.would_change(direction),
                    changed,
                    "seed={seed}, {direction:?}"
                );
            }
        }
    }
}
