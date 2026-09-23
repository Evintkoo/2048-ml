//! Canonical 27-value board and score feature vector.

use crate::game_engine::RawBoardState;
use thiserror::Error;

pub const STATE_FEATURES: usize = 27;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardStateMl(pub [f64; STATE_FEATURES]);

#[derive(Debug, Error, PartialEq)]
#[error("state feature {index} is invalid: {value}")]
pub struct InvalidFeature {
    pub index: usize,
    pub value: f64,
}

impl BoardStateMl {
    pub fn from_board(board: &RawBoardState) -> Self {
        let mut features = [0.0; STATE_FEATURES];
        for (index, &tile) in board.grid.iter().enumerate() {
            features[index] = tile as f64 / 32768.0;
        }
        let empty_count = board.grid.iter().filter(|&&tile| tile == 0).count();
        let max_tile = board.max_tile();
        let (merges, adjacency_sum, smoothness_diff) = adjacent_metrics(&board.grid);
        let row_sums = axis_sums(&board.grid, true);
        let col_sums = axis_sums(&board.grid, false);
        let corner_max = [0, 3, 12, 15]
            .into_iter()
            .map(|i| board.grid[i])
            .max()
            .unwrap_or(0);
        let edge_occupied = (0..16)
            .filter(|&i| {
                (i / 4 == 0 || i / 4 == 3 || i % 4 == 0 || i % 4 == 3) && board.grid[i] != 0
            })
            .count();

        features[16] = empty_count as f64 / 16.0;
        features[17] = if max_tile == 0 {
            0.0
        } else {
            (max_tile as f64).log2() / 15.0
        };
        features[18] = monotonicity(&board.grid);
        features[19] = 1.0 / (1.0 + smoothness_diff as f64 / 100.0);
        features[20] = merges as f64 / 16.0;
        features[21] = (board.score as f64 + 1.0).log10() / 6.0;
        features[22] = adjacency_sum / (16.0 * 32768.0);
        features[23] = corner_max as f64 / 32768.0;
        features[24] = edge_occupied as f64 / 12.0;
        features[25] = *col_sums.iter().min().unwrap_or(&0) as f64 / 8192.0;
        features[26] = *row_sums.iter().min().unwrap_or(&0) as f64 / 8192.0;
        Self(features)
    }

    pub fn validate(&self) -> Result<(), InvalidFeature> {
        for (index, &value) in self.0.iter().enumerate() {
            if !value.is_finite() || value < 0.0 || (index != 21 && value > 1.0) {
                return Err(InvalidFeature { index, value });
            }
        }
        Ok(())
    }
}

fn axis_sums(grid: &[u32; 16], rows: bool) -> [u64; 4] {
    let mut sums = [0_u64; 4];
    for (axis, sum) in sums.iter_mut().enumerate() {
        for offset in 0..4 {
            let index = if rows {
                axis * 4 + offset
            } else {
                offset * 4 + axis
            };
            *sum += grid[index] as u64;
        }
    }
    sums
}

fn adjacent_metrics(grid: &[u32; 16]) -> (usize, f64, u64) {
    let mut mergeable_cells = [false; 16];
    let mut equal_sum = 0.0;
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
                    equal_sum += value as f64;
                }
                diff += value.abs_diff(right) as u64;
            }
            if row < 3 {
                let down = grid[index + 4];
                if value != 0 && value == down {
                    mergeable_cells[index] = true;
                    mergeable_cells[index + 4] = true;
                    equal_sum += value as f64;
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
        equal_sum,
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
    fn feature_vector_has_canonical_shape_order_and_score() {
        let mut board =
            RawBoardState::from_grid([2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        board.score = 4;
        let features = BoardStateMl::from_board(&board);
        assert_eq!(features.0.len(), 27);
        assert_eq!(features.0[0], 2.0 / 32768.0);
        assert_eq!(features.0[16], 13.0 / 16.0);
        assert!((features.0[21] - 5.0_f64.log10() / 6.0).abs() < 1e-12);
        assert!(features.validate().is_ok());
    }

    #[test]
    fn derived_features_stay_in_unit_interval() {
        use rand::{Rng, SeedableRng};
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(7);
        for _ in 0..1000 {
            let grid = std::array::from_fn(|_| match rng.gen_range(0..10) {
                0..=4 => 0,
                n => 1_u32 << n,
            });
            let board = RawBoardState::from_grid(grid).unwrap();
            if let Err(error) = BoardStateMl::from_board(&board).validate() {
                panic!("{error}; grid={:?}", board.grid);
            }
        }
    }

    #[test]
    fn maximum_merge_density_normalizes_to_unit_range() {
        let board = RawBoardState::from_grid([2; 16]).unwrap();
        let features = BoardStateMl::from_board(&board);
        assert_eq!(features.0[20], 1.0);
        assert!(features.0[22] <= 1.0);
        assert!(features.validate().is_ok());
    }
}
