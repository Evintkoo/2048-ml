//! Central seed derivation for repeatable game, training, and analysis runs.

/// Derives component seeds from one user-visible global seed.
///
/// The AutoML final-fit seed remains equal to the global seed, matching the
/// training configuration contract. Other stochastic components use fixed
/// additive offsets documented by `plans/02-Environment/03-Simulation-Engine/02-randomness.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeedManager {
    global_seed: u64,
}

impl SeedManager {
    pub const fn new(global_seed: u64) -> Self {
        Self { global_seed }
    }

    pub const fn global_seed(self) -> u64 {
        self.global_seed
    }

    pub fn game_seed(self, game_id: u64) -> u64 {
        self.global_seed.wrapping_add(game_id)
    }

    pub const fn training_seed(self) -> u64 {
        self.global_seed
    }

    pub fn hyperopt_seed(self) -> u64 {
        self.global_seed.wrapping_add(2)
    }

    pub fn data_sampling_seed(self) -> u64 {
        self.global_seed.wrapping_add(3)
    }

    pub fn cross_validation_seed(self) -> u64 {
        self.global_seed.wrapping_add(4)
    }

    pub fn score_summary_seed(self) -> u64 {
        self.global_seed.wrapping_add(1)
    }

    pub fn action_frequency_seed(self) -> u64 {
        self.global_seed.wrapping_add(2)
    }

    pub fn comparison_seed(self, pair_index: usize) -> u64 {
        self.global_seed
            .wrapping_add(6)
            .wrapping_add(pair_index as u64)
    }
}
