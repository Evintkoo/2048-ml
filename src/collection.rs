//! Resumable, game-batched rollout collection.

use crate::{
    data_pipeline::{self, csv_header},
    game_engine::{self, TrainingSample},
    seeds::SeedManager,
};
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    time::Instant,
};

const CHECKPOINT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CollectionConfig {
    pub n_games: usize,
    pub seed: u64,
    pub rollouts_per_valid_action: usize,
    pub threads: usize,
    pub checkpoint_every: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Checkpoint {
    schema_version: u32,
    config: CollectionConfig,
    next_game_id: usize,
    rows: usize,
    states_collected: usize,
    rollouts_evaluated: usize,
    elapsed_seconds: f64,
    chunks: Vec<Chunk>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Chunk {
    first_game_id: usize,
    end_game_id_exclusive: usize,
    rows: usize,
}

#[derive(Clone, Debug)]
pub struct CollectionSummary {
    pub rows: usize,
    pub states_collected: usize,
    pub rollouts_evaluated: usize,
    pub elapsed_seconds: f64,
    pub resumed: bool,
    pub checkpoint_dir: PathBuf,
}

pub fn collect(
    config: &CollectionConfig,
    checkpoint_dir: &Path,
    resume: bool,
    progress: &ProgressBar,
) -> anyhow::Result<CollectionSummary> {
    anyhow::ensure!(config.n_games > 0, "n_games must be greater than zero");
    anyhow::ensure!(
        config.rollouts_per_valid_action > 0,
        "rollouts must be greater than zero"
    );
    anyhow::ensure!(config.threads > 0, "threads must be greater than zero");
    anyhow::ensure!(
        config.checkpoint_every > 0,
        "checkpoint_every must be greater than zero"
    );

    let checkpoint_path = checkpoint_dir.join("checkpoint.json");
    let (mut checkpoint, is_new) = if resume {
        let checkpoint: Checkpoint = serde_json::from_slice(&fs::read(&checkpoint_path)?)?;
        anyhow::ensure!(
            checkpoint.schema_version == CHECKPOINT_SCHEMA_VERSION,
            "unsupported collection checkpoint schema version"
        );
        anyhow::ensure!(
            checkpoint.config == *config,
            "resume configuration does not match the checkpoint"
        );
        anyhow::ensure!(
            checkpoint.next_game_id <= config.n_games,
            "checkpoint game index exceeds requested game count"
        );
        (checkpoint, false)
    } else {
        anyhow::ensure!(
            !checkpoint_dir.exists(),
            "checkpoint directory already exists; use --resume or choose another output path"
        );
        fs::create_dir_all(checkpoint_dir.join("parts"))?;
        (
            Checkpoint {
                schema_version: CHECKPOINT_SCHEMA_VERSION,
                config: config.clone(),
                next_game_id: 0,
                rows: 0,
                states_collected: 0,
                rollouts_evaluated: 0,
                elapsed_seconds: 0.0,
                chunks: Vec::new(),
            },
            true,
        )
    };
    fs::create_dir_all(checkpoint_dir.join("parts"))?;
    if is_new {
        write_checkpoint(&checkpoint_path, &checkpoint)?;
    }
    progress.set_position(checkpoint.next_game_id as u64);
    let labeler = game_engine::RolloutLabeler {
        n_rollouts: config.rollouts_per_valid_action,
        ..Default::default()
    };
    let seeds = SeedManager::new(config.seed);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.threads)
        .build()?;

    while checkpoint.next_game_id < config.n_games {
        let first = checkpoint.next_game_id;
        let end = first
            .saturating_add(config.checkpoint_every)
            .min(config.n_games);
        let batch_started = Instant::now();
        let games = pool.install(|| {
            (first..end)
                .into_par_iter()
                .map(|game_id| {
                    let mut game = game_engine::collect_random_game(
                        game_id as u64,
                        seeds.game_seed(game_id as u64),
                        0.1,
                        1000,
                    );
                    game_engine::relabel_game(&mut game, &labeler, seeds.data_sampling_seed())?;
                    progress.inc(1);
                    Ok::<_, game_engine::GameError>(game)
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        let states_collected = games.iter().map(|game| game.samples.len()).sum::<usize>();
        let rollouts_evaluated = games
            .iter()
            .flat_map(|game| game.states.iter())
            .map(|board| board.get_valid_moves().len() * config.rollouts_per_valid_action)
            .sum::<usize>();
        let samples: Vec<TrainingSample> =
            games.into_iter().flat_map(|game| game.samples).collect();
        let chunk = Chunk {
            first_game_id: first,
            end_game_id_exclusive: end,
            rows: samples.len(),
        };
        let (data_part, metadata_part) = part_paths(checkpoint_dir, &chunk);
        data_pipeline::write_samples_csv(&data_part, &samples)?;
        data_pipeline::write_sample_metadata_csv(&metadata_part, &samples)?;

        checkpoint.rows += chunk.rows;
        checkpoint.states_collected += states_collected;
        checkpoint.rollouts_evaluated += rollouts_evaluated;
        checkpoint.next_game_id = end;
        checkpoint.elapsed_seconds += batch_started.elapsed().as_secs_f64();
        checkpoint.chunks.push(chunk);
        write_checkpoint(&checkpoint_path, &checkpoint)?;
    }

    Ok(CollectionSummary {
        rows: checkpoint.rows,
        states_collected: checkpoint.states_collected,
        rollouts_evaluated: checkpoint.rollouts_evaluated,
        elapsed_seconds: checkpoint.elapsed_seconds,
        resumed: resume,
        checkpoint_dir: checkpoint_dir.to_owned(),
    })
}

pub fn assemble(
    checkpoint_dir: &Path,
    output: &Path,
    metadata_output: &Path,
) -> anyhow::Result<usize> {
    let checkpoint: Checkpoint =
        serde_json::from_slice(&fs::read(checkpoint_dir.join("checkpoint.json"))?)?;
    anyhow::ensure!(
        checkpoint.next_game_id == checkpoint.config.n_games,
        "collection is incomplete; cannot assemble final dataset"
    );
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = metadata_output.parent() {
        fs::create_dir_all(parent)?;
    }
    let data_tmp = output.with_extension("csv.tmp");
    let metadata_tmp = metadata_output.with_extension("csv.tmp");
    let mut data_writer = BufWriter::new(File::create(&data_tmp)?);
    let mut metadata_writer = BufWriter::new(File::create(&metadata_tmp)?);
    writeln!(data_writer, "{}", csv_header())?;
    writeln!(metadata_writer, "row_index,game_id,move_index,score")?;
    let mut global_row = 0_usize;
    for chunk in &checkpoint.chunks {
        let (data_part, metadata_part) = part_paths(checkpoint_dir, chunk);
        let mut data_lines = BufReader::new(File::open(data_part)?).lines();
        anyhow::ensure!(
            data_lines.next().transpose()?.as_deref() == Some(csv_header().as_str()),
            "checkpoint data part has an invalid schema"
        );
        let mut metadata_lines = BufReader::new(File::open(metadata_part)?).lines();
        anyhow::ensure!(
            metadata_lines.next().transpose()?.as_deref()
                == Some("row_index,game_id,move_index,score"),
            "checkpoint metadata part has an invalid schema"
        );
        let mut chunk_rows = 0;
        loop {
            match (data_lines.next(), metadata_lines.next()) {
                (Some(data_line), Some(metadata_line)) => {
                    writeln!(data_writer, "{}", data_line?)?;
                    let fields: Vec<String> =
                        metadata_line?.split(',').map(str::to_owned).collect();
                    anyhow::ensure!(
                        fields.len() == 4,
                        "checkpoint metadata row has an invalid column count"
                    );
                    writeln!(
                        metadata_writer,
                        "{global_row},{},{},{}",
                        fields[1], fields[2], fields[3]
                    )?;
                    global_row += 1;
                    chunk_rows += 1;
                }
                (None, None) => break,
                _ => anyhow::bail!("checkpoint data and metadata parts have different row counts"),
            }
        }
        anyhow::ensure!(
            chunk_rows == chunk.rows,
            "checkpoint part row count differs from checkpoint"
        );
    }
    data_writer.flush()?;
    metadata_writer.flush()?;
    fs::rename(data_tmp, output)?;
    fs::rename(metadata_tmp, metadata_output)?;
    anyhow::ensure!(
        global_row == checkpoint.rows,
        "assembled row count differs from checkpoint"
    );
    data_pipeline::validate_csv(output)?;
    data_pipeline::read_game_ids_csv(metadata_output)?;
    Ok(global_row)
}

fn part_paths(checkpoint_dir: &Path, chunk: &Chunk) -> (PathBuf, PathBuf) {
    let stem = format!(
        "games-{:08}-{:08}",
        chunk.first_game_id, chunk.end_game_id_exclusive
    );
    let parts = checkpoint_dir.join("parts");
    (
        parts.join(format!("{stem}.csv")),
        parts.join(format!("{stem}.metadata.csv")),
    )
}

fn write_checkpoint(path: &Path, checkpoint: &Checkpoint) -> anyhow::Result<()> {
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, serde_json::to_vec_pretty(checkpoint)?)?;
    fs::rename(tmp_path, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn checkpoint_resume_assembles_complete_ordered_dataset() {
        let root = std::env::temp_dir().join(format!("2048-collection-{}", uuid::Uuid::new_v4()));
        let checkpoint_dir = root.join("policy.checkpoint");
        let output = root.join("policy.csv");
        let metadata = root.join("policy.metadata.csv");
        let progress = ProgressBar::hidden();
        let first_config = CollectionConfig {
            n_games: 2,
            seed: 77,
            rollouts_per_valid_action: 1,
            threads: 2,
            checkpoint_every: 1,
        };
        collect(&first_config, &checkpoint_dir, false, &progress).unwrap();

        let checkpoint_path = checkpoint_dir.join("checkpoint.json");
        let mut checkpoint: Checkpoint =
            serde_json::from_slice(&fs::read(&checkpoint_path).unwrap()).unwrap();
        checkpoint.config.n_games = 4;
        fs::write(
            &checkpoint_path,
            serde_json::to_vec_pretty(&checkpoint).unwrap(),
        )
        .unwrap();

        let resumed_config = CollectionConfig {
            n_games: 4,
            ..first_config
        };
        let summary = collect(&resumed_config, &checkpoint_dir, true, &progress).unwrap();
        assert!(summary.resumed);
        assert_eq!(summary.rows, summary.states_collected);
        assert_eq!(
            assemble(&checkpoint_dir, &output, &metadata).unwrap(),
            summary.rows
        );
        let game_ids = data_pipeline::read_game_ids_csv(&metadata).unwrap();
        assert_eq!(
            game_ids.iter().copied().collect::<BTreeSet<_>>(),
            [0, 1, 2, 3].into_iter().collect()
        );
        assert_eq!(data_pipeline::validate_csv(&output).unwrap(), summary.rows);

        let mismatch = CollectionConfig {
            seed: 78,
            ..resumed_config
        };
        assert!(collect(&mismatch, &checkpoint_dir, true, &progress).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
