//! Canonical 17-value policy state: 16 board cells plus current score.

use crate::game_engine::RawBoardState;
use thiserror::Error;

pub const STATE_FEATURES: usize = 17;
pub const SCORE_FEATURE_INDEX: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardStateMl(pub [f64; STATE_FEATURES]);

#[derive(Debug, Error, PartialEq)]
#[error("state feature {index} is invalid: {value}")]
pub struct InvalidFeature {
    pub index: usize,
    pub value: f64,
}

impl BoardStateMl {
    /// Encode grid cells in row-major order, followed by log-normalized score.
    /// Tile values above 32768 remain valid and may produce values above one.
    pub fn from_board(board: &RawBoardState) -> Self {
        let mut features = [0.0; STATE_FEATURES];
        for (index, &tile) in board.grid.iter().enumerate() {
            features[index] = tile as f64 / 32768.0;
        }
        features[SCORE_FEATURE_INDEX] = (board.score as f64 + 1.0).log10() / 6.0;
        Self(features)
    }

    pub fn validate(&self) -> Result<(), InvalidFeature> {
        for (index, &value) in self.0.iter().enumerate() {
            if !value.is_finite() || value < 0.0 {
                return Err(InvalidFeature { index, value });
            }
        }
        Ok(())
    }
}

/// Hand-authored heuristic measurements. These are not model training inputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeuristicFeatures {
    pub empty_fraction: f64,
    pub monotonicity: f64,
    pub smoothness: f64,
    pub merges_fraction: f64,
    pub corner_max: f64,
}

impl HeuristicFeatures {
    pub fn from_board(board: &RawBoardState) -> Self {
        let empty_count = board.grid.iter().filter(|&&tile| tile == 0).count();
        let (mergeable_cells, smoothness_diff) = adjacent_metrics(&board.grid);
        let corner_max = [0, 3, 12, 15]
            .into_iter()
            .map(|index| board.grid[index])
            .max()
            .unwrap_or(0);
        Self {
            empty_fraction: empty_count as f64 / 16.0,
            monotonicity: monotonicity(&board.grid),
            smoothness: 1.0 / (1.0 + smoothness_diff as f64 / 100.0),
            merges_fraction: mergeable_cells as f64 / 16.0,
            corner_max: corner_max as f64 / 32768.0,
        }
    }
}

fn adjacent_metrics(grid: &[u32; 16]) -> (usize, u64) {
    let mut mergeable_cells = [false; 16];
    let mut diff = 0_u64;
    for row in 0..4 {
        for col in 0..4 {
            let index = row * 4 + col;
            let value = grid[index];
            if col < 3 {
                let right = grid[index + 1];
                if value != 0 && value == right {
                    mergeable_cells[index] = true;
                    mergeable_cells[index + 1] = true;
                }
                diff += value.abs_diff(right) as u64;
            }
            if row < 3 {
                let down = grid[index + 4];
                if value != 0 && value == down {
                    mergeable_cells[index] = true;
                    mergeable_cells[index + 4] = true;
                }
                diff += value.abs_diff(down) as u64;
            }
        }
    }
    (
        mergeable_cells
            .into_iter()
            .filter(|is_mergeable| *is_mergeable)
            .count(),
        diff,
    )
}

fn monotonicity(grid: &[u32; 16]) -> f64 {
    let mut matched = 0;
    let mut comparisons = 0;
    for row in 0..4 {
        for col in 0..3 {
            let a = grid[row * 4 + col];
            let b = grid[row * 4 + col + 1];
            if a == 0 || b == 0 || a == b {
                matched += 1;
            }
            comparisons += 1;
        }
    }
    for col in 0..4 {
        for row in 0..3 {
            let a = grid[row * 4 + col];
            let b = grid[(row + 1) * 4 + col];
            if a == 0 || b == 0 || a == b {
                matched += 1;
            }
            comparisons += 1;
        }
    }
    matched as f64 / comparisons as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_vector_has_sixteen_cells_then_score() {
        let mut board =
            RawBoardState::from_grid([2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        board.score = 4;
        let features = BoardStateMl::from_board(&board);
        assert_eq!(features.0.len(), 17);
        assert_eq!(features.0[0], 2.0 / 32768.0);
        assert_eq!(features.0[15], 0.0);
        assert!((features.0[SCORE_FEATURE_INDEX] - 5.0_f64.log10() / 6.0).abs() < 1e-12);
        assert!(features.validate().is_ok());
    }

    #[test]
    fn larger_legal_tiles_and_scores_are_valid_finite_inputs() {
        let mut board =
            RawBoardState::from_grid([65536, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        board.score = 2_000_000;
        let features = BoardStateMl::from_board(&board);
        assert_eq!(features.0[0], 2.0);
        assert!(features.0[SCORE_FEATURE_INDEX] > 1.0);
        assert!(features.validate().is_ok());
    }

    #[test]
    fn heuristic_measurements_stay_finite_on_seeded_boards() {
        use rand::{Rng, SeedableRng};
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(7);
        for _ in 0..1000 {
            let grid = std::array::from_fn(|_| match rng.gen_range(0..10) {
                0..=4 => 0,
                n => 1_u32 << n,
            });
            let board = RawBoardState::from_grid(grid).unwrap();
            let features = HeuristicFeatures::from_board(&board);
            assert!(features.empty_fraction.is_finite());
            assert!(features.monotonicity.is_finite());
            assert!(features.smoothness.is_finite());
            assert!(features.merges_fraction.is_finite());
        }
    }

    #[test]
    fn fully_mergeable_board_has_maximum_merge_fraction() {
        let board = RawBoardState::from_grid([2; 16]).unwrap();
        let features = HeuristicFeatures::from_board(&board);
        assert_eq!(features.merges_fraction, 1.0);
        assert!(features.smoothness.is_finite());
    }
}
