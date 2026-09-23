//! Valid action constraints and deterministic model output selection.

use crate::game_engine::{Direction, RawBoardState};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ActionError {
    #[error("no valid actions are available on a terminal board")]
    NoValidActions,
    #[error("all valid action scores must be finite")]
    NonFiniteScore,
    #[error("action index must be in 0..=3, got {0}")]
    InvalidAction(u8),
}

pub fn valid_actions(board: &RawBoardState) -> Vec<u8> {
    board
        .get_valid_moves()
        .into_iter()
        .map(|direction| direction as u8)
        .collect()
}

pub fn masked_argmax(scores: &[f64; 4], valid: &[u8]) -> Result<u8, ActionError> {
    if valid.is_empty() {
        return Err(ActionError::NoValidActions);
    }
    let mut best: Option<(u8, f64)> = None;
    for &action in valid {
        if action > 3 {
            return Err(ActionError::InvalidAction(action));
        }
        let score = scores[action as usize];
        if !score.is_finite() {
            return Err(ActionError::NonFiniteScore);
        }
        if match best {
            None => true,
            Some((best_action, best_score)) => {
                score > best_score || (score == best_score && action < best_action)
            }
        } {
            best = Some((action, score));
        }
    }
    Ok(best.expect("valid actions checked non-empty").0)
}

pub fn decide_action(scores: &[f64; 4], board: &RawBoardState) -> Result<Direction, ActionError> {
    Direction::try_from_action(masked_argmax(scores, &valid_actions(board))?)
        .map_err(|_| ActionError::InvalidAction(255))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masked_argmax_ignores_invalid_moves_and_breaks_ties_by_action_order() {
        assert_eq!(masked_argmax(&[100.0, 5.0, 6.0, 6.0], &[1, 2, 3]), Ok(2));
        assert_eq!(
            masked_argmax(&[0.0; 4], &[]),
            Err(ActionError::NoValidActions)
        );
    }
}
