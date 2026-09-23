//! CSV storage and validation for the canonical supervised 27-feature schema.

use crate::{game_engine::TrainingSample, state::STATE_FEATURES};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::Path,
};
use thiserror::Error;

pub const FEATURE_NAMES: [&str; STATE_FEATURES] = [
    "grid_0",
    "grid_1",
    "grid_2",
    "grid_3",
    "grid_4",
    "grid_5",
    "grid_6",
    "grid_7",
    "grid_8",
    "grid_9",
    "grid_10",
    "grid_11",
    "grid_12",
    "grid_13",
    "grid_14",
    "grid_15",
    "empty_count",
    "max_tile_log",
    "monotonicity",
    "smoothness",
    "merges_available",
    "score_normalized",
    "adjacency_merge_score",
    "corner_max",
    "edge_tiles_occupied",
    "col_worst",
    "row_worst",
];

#[derive(Debug, Error)]
pub enum DataError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid dataset at line {line}: {message}")]
    InvalidRow { line: usize, message: String },
}

pub fn csv_header() -> String {
    format!("{},action", FEATURE_NAMES.join(","))
}

pub fn write_samples_csv(
    path: impl AsRef<Path>,
    samples: &[TrainingSample],
) -> Result<(), DataError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    writeln!(file, "{}", csv_header())?;
    for sample in samples {
        validate_sample(sample).map_err(|message| DataError::InvalidRow { line: 0, message })?;
        for feature in sample.state_features {
            write!(file, "{feature:.17},")?;
        }
        writeln!(file, "{}", sample.action)?;
    }
    file.flush()?;
    Ok(())
}

/// Write row-aligned provenance metadata separately from the exact 28-column
/// AutoML training CSV, preserving game groups without leaking IDs as features.
pub fn write_sample_metadata_csv(
    path: impl AsRef<Path>,
    samples: &[TrainingSample],
) -> Result<(), DataError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    writeln!(file, "row_index,game_id,move_index,score")?;
    for (row_index, sample) in samples.iter().enumerate() {
        writeln!(
            file,
            "{row_index},{},{},{}",
            sample.game_id, sample.move_index, sample.score
        )?;
    }
    file.flush()?;
    Ok(())
}

pub fn validate_csv(path: impl AsRef<Path>) -> Result<usize, DataError> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut header = String::new();
    reader.read_line(&mut header)?;
    let expected = csv_header();
    if header.trim_end() != expected {
        return Err(DataError::InvalidRow {
            line: 1,
            message: format!("header mismatch; expected {expected}"),
        });
    }
    let mut count = 0;
    for (offset, line) in reader.lines().enumerate() {
        let line_num = offset + 2;
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != STATE_FEATURES + 1 {
            return Err(DataError::InvalidRow {
                line: line_num,
                message: format!("expected 28 training columns, got {}", fields.len()),
            });
        }
        for (index, field) in fields[..STATE_FEATURES].iter().enumerate() {
            let value: f64 = field.parse().map_err(|_| DataError::InvalidRow {
                line: line_num,
                message: format!("feature {index} is not a number"),
            })?;
            if !value.is_finite() || value < 0.0 || (index != 21 && value > 1.0) {
                return Err(DataError::InvalidRow {
                    line: line_num,
                    message: format!("feature {index} is out of range: {value}"),
                });
            }
        }
        let action: u8 = fields[27].parse().map_err(|_| DataError::InvalidRow {
            line: line_num,
            message: "action is not an integer".to_string(),
        })?;
        if action > 3 {
            return Err(DataError::InvalidRow {
                line: line_num,
                message: format!("action must be 0..=3, got {action}"),
            });
        }
        count += 1;
    }
    Ok(count)
}

pub fn validate_sample(sample: &TrainingSample) -> Result<(), String> {
    for (index, value) in sample.state_features.iter().copied().enumerate() {
        if !value.is_finite() || value < 0.0 || (index != 21 && value > 1.0) {
            return Err(format!("feature {index} is out of range: {value}"));
        }
    }
    if sample.action > 3 {
        return Err(format!("action must be 0..=3, got {}", sample.action));
    }
    Ok(())
}

pub fn read_game_ids_csv(path: impl AsRef<Path>) -> Result<Vec<i64>, DataError> {
    let mut lines = BufReader::new(fs::File::open(path)?).lines();
    let header = lines.next().transpose()?.unwrap_or_default();
    if header != "row_index,game_id,move_index,score" {
        return Err(DataError::InvalidRow {
            line: 1,
            message: "metadata CSV header mismatch".to_string(),
        });
    }
    let mut game_ids = Vec::new();
    for (offset, line) in lines.enumerate() {
        let line_num = offset + 2;
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 4 || fields[0].parse::<usize>().ok() != Some(offset) {
            return Err(DataError::InvalidRow {
                line: line_num,
                message: "metadata row index is missing or out of order".to_string(),
            });
        }
        game_ids.push(
            fields[1]
                .parse::<i64>()
                .map_err(|_| DataError::InvalidRow {
                    line: line_num,
                    message: "game_id must fit i64".to_string(),
                })?,
        );
        fields[2]
            .parse::<u64>()
            .map_err(|_| DataError::InvalidRow {
                line: line_num,
                message: "move_index must be u64".to_string(),
            })?;
        fields[3]
            .parse::<u64>()
            .map_err(|_| DataError::InvalidRow {
                line: line_num,
                message: "score must be u64".to_string(),
            })?;
    }
    Ok(game_ids)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameDataSplit {
    pub train_indices: Vec<usize>,
    pub validation_indices: Vec<usize>,
    pub test_indices: Vec<usize>,
    pub train_games: Vec<u64>,
    pub validation_games: Vec<u64>,
    pub test_games: Vec<u64>,
}

/// Chronological 70/15/15 split on unique game IDs. Rows from a game always
/// stay together, and metadata IDs never enter the model feature matrix.
pub fn chronological_game_split(samples: &[TrainingSample]) -> Result<GameDataSplit, DataError> {
    let mut games: Vec<u64> = samples.iter().map(|sample| sample.game_id).collect();
    games.sort_unstable();
    games.dedup();
    if games.len() < 3 {
        return Err(DataError::InvalidRow {
            line: 0,
            message: "at least 3 distinct games are required for train/validation/test".to_string(),
        });
    }
    let train_end = (games.len() * 70 / 100).max(1).min(games.len() - 2);
    let validation_count = (games.len() * 15 / 100)
        .max(1)
        .min(games.len() - train_end - 1);
    let validation_end = train_end + validation_count;
    let train_games = games[..train_end].to_vec();
    let validation_games = games[train_end..validation_end].to_vec();
    let test_games = games[validation_end..].to_vec();
    let train_set: std::collections::HashSet<u64> = train_games.iter().copied().collect();
    let validation_set: std::collections::HashSet<u64> = validation_games.iter().copied().collect();
    let test_set: std::collections::HashSet<u64> = test_games.iter().copied().collect();
    let mut split = GameDataSplit {
        train_indices: Vec::new(),
        validation_indices: Vec::new(),
        test_indices: Vec::new(),
        train_games,
        validation_games,
        test_games,
    };
    for (index, sample) in samples.iter().enumerate() {
        if train_set.contains(&sample.game_id) {
            split.train_indices.push(index);
        } else if validation_set.contains(&sample.game_id) {
            split.validation_indices.push(index);
        } else if test_set.contains(&sample.game_id) {
            split.test_indices.push(index);
        } else {
            return Err(DataError::InvalidRow {
                line: index + 2,
                message: "sample game_id missing from split".to_string(),
            });
        }
    }
    Ok(split)
}

pub fn split_csv_by_game(
    training_path: impl AsRef<Path>,
    metadata_path: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<(), DataError> {
    let data_lines: Vec<String> = BufReader::new(fs::File::open(training_path)?)
        .lines()
        .collect::<Result<_, _>>()?;
    let metadata_lines: Vec<String> = BufReader::new(fs::File::open(metadata_path)?)
        .lines()
        .collect::<Result<_, _>>()?;
    if data_lines.first().map(String::as_str) != Some(csv_header().as_str()) {
        return Err(DataError::InvalidRow {
            line: 1,
            message: "training CSV header does not match canonical schema".to_string(),
        });
    }
    if metadata_lines.first().map(String::as_str) != Some("row_index,game_id,move_index,score") {
        return Err(DataError::InvalidRow {
            line: 1,
            message: "metadata CSV header mismatch".to_string(),
        });
    }
    if data_lines.len() != metadata_lines.len() {
        return Err(DataError::InvalidRow {
            line: 0,
            message: "training and metadata row counts differ".to_string(),
        });
    }
    let mut game_ids = Vec::with_capacity(data_lines.len().saturating_sub(1));
    for (offset, line) in metadata_lines.iter().skip(1).enumerate() {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 4 || fields[0].parse::<usize>().ok() != Some(offset) {
            return Err(DataError::InvalidRow {
                line: offset + 2,
                message: "metadata row index is missing or out of order".to_string(),
            });
        }
        game_ids.push(
            fields[1]
                .parse::<u64>()
                .map_err(|_| DataError::InvalidRow {
                    line: offset + 2,
                    message: "game_id must be u64".to_string(),
                })?,
        );
    }
    let mut unique_games = game_ids.clone();
    unique_games.sort_unstable();
    unique_games.dedup();
    if unique_games.len() < 3 {
        return Err(DataError::InvalidRow {
            line: 0,
            message: "at least three games are required for a train/validation/test split"
                .to_string(),
        });
    }
    let train_end = (unique_games.len() * 70 / 100)
        .max(1)
        .min(unique_games.len() - 2);
    let validation_count = (unique_games.len() * 15 / 100)
        .max(1)
        .min(unique_games.len() - train_end - 1);
    let validation_end = train_end + validation_count;
    let train: std::collections::HashSet<u64> = unique_games[..train_end].iter().copied().collect();
    let validation: std::collections::HashSet<u64> = unique_games[train_end..validation_end]
        .iter()
        .copied()
        .collect();
    let test: std::collections::HashSet<u64> =
        unique_games[validation_end..].iter().copied().collect();

    fs::create_dir_all(output_dir.as_ref())?;
    for (name, group_set) in [
        ("train", &train),
        ("validation", &validation),
        ("test", &test),
    ] {
        let data_out = fs::File::create(output_dir.as_ref().join(format!("{name}.csv")))?;
        let mut data_writer = std::io::BufWriter::new(data_out);
        let metadata_out =
            fs::File::create(output_dir.as_ref().join(format!("{name}.metadata.csv")))?;
        let mut metadata_writer = std::io::BufWriter::new(metadata_out);
        writeln!(data_writer, "{}", data_lines[0])?;
        writeln!(metadata_writer, "row_index,game_id,move_index,score")?;
        let mut row_index = 0;
        for (index, &game_id) in game_ids.iter().enumerate() {
            if !group_set.contains(&game_id) {
                continue;
            }
            writeln!(data_writer, "{}", data_lines[index + 1])?;
            let fields: Vec<&str> = metadata_lines[index + 1].split(',').collect();
            writeln!(
                metadata_writer,
                "{row_index},{},{},{}",
                fields[1], fields[2], fields[3]
            )?;
            row_index += 1;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_engine::collect_random_game;

    #[test]
    fn csv_roundtrip_validates_rows_and_header() {
        let game = collect_random_game(3, 45, 0.1, 1000);
        let path = std::env::temp_dir().join(format!(
            "2048-ml-{}-{}.csv",
            std::process::id(),
            game.result.seed
        ));
        write_samples_csv(&path, &game.samples).unwrap();
        assert_eq!(validate_csv(&path).unwrap(), game.samples.len());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rollout_relabeling_keeps_only_legal_actions() {
        let mut game = collect_random_game(3, 45, 0.1, 20);
        let labeler = crate::game_engine::RolloutLabeler {
            n_rollouts: 2,
            max_moves: 100,
            spawn_prob_4: 0.1,
        };
        crate::game_engine::relabel_game(&mut game, &labeler, 99).unwrap();
        for (sample, board) in game.samples.iter().zip(&game.states) {
            assert!(board
                .get_valid_moves()
                .iter()
                .any(|direction| *direction as u8 == sample.action));
        }
    }

    #[test]
    fn chronological_split_keeps_games_disjoint_and_ordered() {
        let samples: Vec<_> = (0..20)
            .flat_map(|game_id| collect_random_game(game_id, 42 + game_id, 0.1, 2).samples)
            .collect();
        let split = chronological_game_split(&samples).unwrap();
        assert_eq!(split.train_games.len(), 14);
        assert_eq!(split.validation_games.len(), 3);
        assert_eq!(split.test_games.len(), 3);
        assert!(split.train_games.last() < split.validation_games.first());
        assert!(split.validation_games.last() < split.test_games.first());
        assert!(split
            .train_indices
            .iter()
            .all(|&i| split.train_games.contains(&samples[i].game_id)));
    }

    #[test]
    fn csv_split_writes_three_group_disjoint_parts() {
        let samples: Vec<_> = (0..20)
            .flat_map(|game_id| collect_random_game(game_id, 900 + game_id, 0.1, 3).samples)
            .collect();
        let root = std::env::temp_dir().join(format!("2048-ml-split-{}", std::process::id()));
        let input = root.join("all.csv");
        let metadata = root.join("all.metadata.csv");
        write_samples_csv(&input, &samples).unwrap();
        write_sample_metadata_csv(&metadata, &samples).unwrap();
        let output = root.join("splits");
        split_csv_by_game(&input, &metadata, &output).unwrap();
        let train = validate_csv(output.join("train.csv")).unwrap();
        let validation = validate_csv(output.join("validation.csv")).unwrap();
        let test = validate_csv(output.join("test.csv")).unwrap();
        assert_eq!(train + validation + test, samples.len());
        fs::remove_dir_all(root).unwrap();
    }
}
