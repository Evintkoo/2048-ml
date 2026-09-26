//! Four-action model inference with validity masking.

use crate::{
    actions::{masked_argmax, ActionError},
    game_engine::{Direction, GameError, GameSimulator, RawBoardState, SimulatorConfig},
    state::{BoardStateMl, HeuristicFeatures, STATE_FEATURES},
};
use automl::inference::{InferenceConfig, InferenceEngine};
use ndarray::Array2;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("AutoML inference failed: {0}")]
    AutoMl(#[from] automl::AutoMLError),
    #[error("action selection failed: {0}")]
    Action(#[from] ActionError),
    #[error("game rule error: {0}")]
    Game(#[from] GameError),
    #[error("model must return four probabilities, got {0}")]
    ProbabilityShape(usize),
}

pub struct ModelPolicy {
    engine: InferenceEngine,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HeuristicPolicy;

impl HeuristicPolicy {
    pub fn select_move(&self, board: &RawBoardState) -> Result<Direction, PolicyError> {
        let mut best: Option<(Direction, f64)> = None;
        for direction in board.get_valid_moves() {
            let mut next = *board;
            next.execute_move(direction)?;
            let features = HeuristicFeatures::from_board(&next);
            let utility = features.empty_fraction * 100.0
                + features.monotonicity * 25.0
                + features.smoothness * 10.0
                + features.corner_max * 100.0
                + features.merges_fraction * 15.0;
            if match best {
                None => true,
                Some((_, best_utility)) => utility > best_utility,
            } {
                best = Some((direction, utility));
            }
        }
        best.map(|(direction, _)| direction)
            .ok_or(ActionError::NoValidActions.into())
    }
}

impl ModelPolicy {
    pub fn load(path: &str) -> Result<Self, PolicyError> {
        let engine = InferenceEngine::load(InferenceConfig::new(), None, path)?;
        Ok(Self { engine })
    }

    pub fn select_move(&self, board: &RawBoardState) -> Result<Direction, PolicyError> {
        let features = BoardStateMl::from_board(board).0;
        let input = Array2::from_shape_vec((1, STATE_FEATURES), features.to_vec())
            .expect("the state encoder always produces the canonical feature count");
        let probabilities = self.engine.predict_proba_array(&input)?;
        if probabilities.nrows() != 1 || probabilities.ncols() != 4 {
            return Err(PolicyError::ProbabilityShape(probabilities.ncols()));
        }
        let scores: [f64; 4] = probabilities
            .row(0)
            .to_vec()
            .try_into()
            .expect("probability shape was checked");
        let valid: Vec<u8> = board
            .get_valid_moves()
            .into_iter()
            .map(|direction| direction as u8)
            .collect();
        let action = masked_argmax(&scores, &valid)?;
        Direction::try_from_action(action).map_err(|error| {
            PolicyError::AutoMl(automl::AutoMLError::InvalidInput(error.to_string()))
        })
    }
}

pub fn simulate_model_game(
    seed: u64,
    model: &ModelPolicy,
    max_moves: u64,
) -> Result<crate::game_engine::GameResult, GameError> {
    let mut simulator = GameSimulator::new(SimulatorConfig {
        seed,
        max_moves,
        ..SimulatorConfig::default()
    })?;
    simulator.simulate_with_policy(|board| {
        model
            .select_move(board)
            .map_err(|error| GameError::ModelInference(error.to_string()))
    })
}

pub fn simulate_heuristic_game(
    seed: u64,
    max_moves: u64,
) -> Result<crate::game_engine::GameResult, GameError> {
    let policy = HeuristicPolicy;
    let mut simulator = GameSimulator::new(SimulatorConfig {
        seed,
        max_moves,
        ..SimulatorConfig::default()
    })?;
    simulator.simulate_with_policy(|board| {
        policy
            .select_move(board)
            .map_err(|error| GameError::ModelInference(error.to_string()))
    })
}
