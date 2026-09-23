use clap::{Parser, Subcommand};

pub mod actions;
pub mod data_pipeline;
pub mod evaluation;
mod framework_validation;
pub mod game_engine;
pub mod policy;
pub mod state;
pub mod training;

#[derive(Debug, Parser)]
#[command(
    name = "2048-ml",
    version,
    about = "2048 supervised policy learning research tools"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Simulate headless 2048 games.
    GameEngine {
        #[command(subcommand)]
        command: GameEngineCommand,
    },
    /// Collect and validate supervised policy data.
    DataCollector {
        #[command(subcommand)]
        command: DataCollectorCommand,
    },
    /// Run policy and framework benchmarks.
    Benchmark {
        #[command(subcommand)]
        command: BenchmarkCommand,
    },
    /// Train a supervised action classifier through the AutoML framework.
    Train {
        #[arg(long)]
        data: std::path::PathBuf,
        #[arg(long, default_value_t = 5)]
        cv_folds: usize,
        /// Fraction of games before the held-out test tail used for model development (train + validation).
        #[arg(long, default_value_t = 0.85)]
        development_fraction: f64,
        #[arg(long, default_value = "random_forest")]
        model: String,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        /// Required game-level metadata sidecar. The final chronological 15% is reserved from training.
        #[arg(long)]
        metadata: Option<std::path::PathBuf>,
        #[arg(long, default_value = "models/policy.json")]
        output: std::path::PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum GameEngineCommand {
    Simulate {
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long, default_value_t = 1)]
        n_games: usize,
    },
}

#[derive(Debug, Subcommand)]
enum DataCollectorCommand {
    Collect {
        #[arg(long, default_value_t = 1)]
        n_games: usize,
        #[arg(long, default_value_t = 42)]
        seed: u64,
        #[arg(long, default_value_t = 100)]
        rollouts: usize,
        #[arg(long, default_value_t = 1)]
        threads: usize,
        #[arg(long, default_value = "data/raw/random_play.csv")]
        output: std::path::PathBuf,
    },
    Preprocess {
        #[arg(long)]
        input: std::path::PathBuf,
    },
    Validate {
        #[arg(long)]
        input: std::path::PathBuf,
    },
    Split {
        #[arg(long)]
        input: std::path::PathBuf,
        #[arg(long)]
        metadata: std::path::PathBuf,
        #[arg(long, default_value = "data/processed/splits")]
        output_dir: std::path::PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum BenchmarkCommand {
    Run {
        #[arg(long)]
        model: std::path::PathBuf,
        #[arg(long, default_value_t = 10_000)]
        n_games: usize,
        #[arg(long, default_value_t = 9_999)]
        seed: u64,
        #[arg(long, default_value = "results/evaluation.csv")]
        output: std::path::PathBuf,
    },
    Baseline {
        #[arg(long, default_value = "heuristic")]
        agent: String,
        #[arg(long, default_value_t = 10_000)]
        n_games: usize,
        #[arg(long, default_value_t = 9_999)]
        seed: u64,
        #[arg(long, default_value = "results/baseline.csv")]
        output: std::path::PathBuf,
    },
    Compare {
        /// One or more game-result CSV files produced by benchmark run/baseline.
        #[arg(required = true)]
        inputs: Vec<std::path::PathBuf>,
        #[arg(long, default_value = "results/comparison.csv")]
        output: std::path::PathBuf,
        #[arg(long, default_value_t = 42)]
        seed: u64,
    },
    Report {
        /// Game-result CSV files to summarize.
        #[arg(required = true)]
        inputs: Vec<std::path::PathBuf>,
        #[arg(long, default_value = "results/report.csv")]
        output: std::path::PathBuf,
    },
}

#[derive(Clone, Debug)]
struct BenchmarkScores {
    name: String,
    input: std::path::PathBuf,
    seeds: Vec<u64>,
    scores: Vec<u64>,
}

fn read_benchmark_scores(path: &std::path::Path) -> anyhow::Result<BenchmarkScores> {
    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    let header = lines
        .next()
        .transpose()?
        .ok_or_else(|| anyhow::anyhow!("{} is empty", path.display()))?;
    let columns: Vec<&str> = header.split(',').collect();
    let seed_col = columns
        .iter()
        .position(|column| *column == "seed")
        .ok_or_else(|| anyhow::anyhow!("{} has no seed column", path.display()))?;
    let score_col = columns
        .iter()
        .position(|column| *column == "score")
        .ok_or_else(|| anyhow::anyhow!("{} has no score column", path.display()))?;
    let name_col = columns
        .iter()
        .position(|column| matches!(*column, "model" | "agent"));
    let mut scores = Vec::new();
    let mut seeds = Vec::new();
    let mut name = None;
    for (offset, line) in lines.enumerate() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != columns.len() {
            anyhow::bail!(
                "{} line {} has {} fields, expected {}",
                path.display(),
                offset + 2,
                fields.len(),
                columns.len()
            );
        }
        seeds.push(fields[seed_col].parse::<u64>()?);
        scores.push(fields[score_col].parse::<u64>()?);
        if let Some(index) = name_col {
            let value = fields[index].to_owned();
            if name.as_ref().is_some_and(|previous| previous != &value) {
                anyhow::bail!("{} mixes model/agent names", path.display());
            }
            name = Some(value);
        }
    }
    if scores.is_empty() {
        anyhow::bail!("{} contains no game rows", path.display());
    }
    if seeds.iter().collect::<std::collections::HashSet<_>>().len() != seeds.len() {
        anyhow::bail!("{} contains duplicate seeds", path.display());
    }
    if seeds
        .windows(2)
        .any(|pair| pair[1] != pair[0].wrapping_add(1))
    {
        anyhow::bail!(
            "{} game seeds are not in expected consecutive order",
            path.display()
        );
    }
    Ok(BenchmarkScores {
        name: name.unwrap_or_else(|| {
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        }),
        input: path.to_owned(),
        seeds,
        scores,
    })
}

fn write_json_manifest(output: &std::path::Path, value: &serde_json::Value) -> anyhow::Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let path = output.with_extension("manifest.json");
    std::fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

fn sha256_file(path: &std::path::Path) -> anyhow::Result<String> {
    use sha2::Digest;
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        None => println!("Select a command. Use --help to see the planned workflows."),
        Some(Commands::GameEngine {
            command: GameEngineCommand::Simulate { seed, n_games },
        }) => {
            if n_games == 0 {
                eprintln!("--n-games must be greater than zero");
                std::process::exit(2);
            }
            for game_id in 0..n_games {
                let game_seed = seed.wrapping_add(game_id as u64);
                let result = game_engine::simulate_random_game(game_seed, 0.1, 1000);
                println!(
                    "{game_id},{game_seed},{},{},{}",
                    result.final_score, result.max_tile, result.move_count
                );
            }
        }
        Some(Commands::DataCollector {
            command:
                DataCollectorCommand::Collect {
                    n_games,
                    seed,
                    rollouts,
                    threads,
                    output,
                },
        }) => {
            if n_games == 0 || rollouts == 0 || threads == 0 {
                eprintln!("--n-games, --rollouts, and --threads must be greater than zero");
                std::process::exit(2);
            }
            let labeler = game_engine::RolloutLabeler {
                n_rollouts: rollouts,
                ..Default::default()
            };
            use rayon::prelude::*;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .expect("failed to create fixed-size simulation thread pool");
            let start = std::time::Instant::now();
            let games = pool.install(|| {
                (0..n_games)
                    .into_par_iter()
                    .map(|game_id| {
                        let game_seed = seed.wrapping_add(game_id as u64);
                        let mut game = game_engine::collect_random_game(game_id as u64, game_seed, 0.1, 1000);
                        game_engine::relabel_game(&mut game, &labeler, seed)?;
                        Ok::<_, game_engine::GameError>(game)
                    })
                    .collect::<Result<Vec<_>, _>>()
            }).expect("rollout labeling failed");
            let states_collected = games.iter().map(|game| game.samples.len()).sum::<usize>();
            let rollouts_evaluated = games
                .iter()
                .flat_map(|game| game.states.iter())
                .map(|board| board.get_valid_moves().len() * rollouts)
                .sum::<usize>();
            let samples: Vec<_> = games.into_iter().flat_map(|game| game.samples).collect();
            let elapsed = start.elapsed().as_secs_f64();
            let metadata_path = output.with_extension("metadata.csv");
            data_pipeline::write_samples_csv(&output, &samples)
                .expect("failed to write training CSV");
            data_pipeline::write_sample_metadata_csv(&metadata_path, &samples)
                .expect("failed to write metadata CSV");
            let rows = data_pipeline::validate_csv(&output).expect("generated CSV validation failed");
            let manifest = output.with_extension("manifest.json");
            let manifest_data = serde_json::json!({
                "created_utc": chrono::Utc::now().to_rfc3339(),
                "project_version": env!("CARGO_PKG_VERSION"),
                "dataset_schema": "2048-action-policy-v1",
                "automl_commit": "64f5edad29c9e58ee7d33abf380418d5cfbbb561",
                "source_revision": std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()),
                "global_seed": seed,
                "game_seeds": {"derivation": "global_seed.wrapping_add(game_id)", "first": seed, "last": seed.wrapping_add((n_games - 1) as u64)},
                "games": n_games,
                "rollouts_per_valid_action": rollouts,
                "threads": threads,
                "spawn_probability_for_four": 0.1,
                "training_rows": rows,
                "states_collected": states_collected,
                "rollouts_evaluated": rollouts_evaluated,
                "label_cache_hits": 0,
                "label_cache_misses": rollouts_evaluated,
                "elapsed_seconds": elapsed,
                "labeling_rows_per_second": rows as f64 / elapsed.max(f64::MIN_POSITIVE),
                "training_csv": output,
                "metadata_csv": metadata_path,
                "training_csv_sha256": sha256_file(&output).ok(),
                "metadata_csv_sha256": sha256_file(&metadata_path).ok(),
                "labeling": "rollout_mean_final_score_argmax"
            });
            std::fs::write(&manifest, serde_json::to_vec_pretty(&manifest_data).unwrap())
                .expect("failed to write collection manifest");
            println!(
                "validated {rows} rows in {}; metadata={} manifest={} elapsed_secs={elapsed:.2}",
                output.display(),
                metadata_path.display(),
                manifest.display()
            );
        }
        Some(Commands::DataCollector { command: DataCollectorCommand::Validate { input } }) => {
            match data_pipeline::validate_csv(&input) {
                Ok(rows) => println!("valid: {rows} rows"),
                Err(error) => {
                    eprintln!("invalid dataset: {error}");
                    std::process::exit(2);
                }
            }
        }
        Some(Commands::DataCollector { command: DataCollectorCommand::Preprocess { input } }) => {
            match data_pipeline::validate_csv(&input) {
                Ok(rows) => println!("validated {rows} rows; deterministic features need no fitted preprocessing for the current collection pipeline"),
                Err(error) => {
                    eprintln!("invalid dataset: {error}");
                    std::process::exit(2);
                }
            }
        }
        Some(Commands::DataCollector { command: DataCollectorCommand::Split { input, metadata, output_dir } }) => {
            data_pipeline::split_csv_by_game(input, metadata, &output_dir)
                .expect("failed to split dataset by game");
            for split in ["train", "validation", "test"] {
                let path = output_dir.join(format!("{split}.csv"));
                let rows = data_pipeline::validate_csv(&path).expect("split CSV validation failed");
                println!("{split}: {rows} rows");
            }
        }
        Some(Commands::Train { data, metadata, cv_folds, development_fraction, model, seed, output }) => {
            if data_pipeline::validate_csv(&data).is_err() {
                eprintln!("training input must satisfy the canonical 28-column schema");
                std::process::exit(2);
            }
            let model_type = match model.to_ascii_lowercase().as_str() {
                "random_forest" | "randomforest" => automl::training::ModelType::RandomForest,
                "extra_trees" | "extratrees" => automl::training::ModelType::ExtraTrees,
                "adaboost" => automl::training::ModelType::AdaBoost,
                "knn" => automl::training::ModelType::KNN,
                "naive_bayes" | "naivebayes" => automl::training::ModelType::NaiveBayes,
                _ => {
                    eprintln!("unsupported four-class model {model}; choose random_forest, extra_trees, adaboost, knn, or naive_bayes");
                    std::process::exit(2);
                }
            };
            let mut data_frame = automl::cli::load_data(&data).expect("failed to read training CSV");
            let group_ids = if let Some(metadata_path) = metadata.as_ref() {
                if !(0.5..1.0).contains(&development_fraction) {
                    eprintln!("--development-fraction must be >= 0.5 and < 1.0");
                    std::process::exit(2);
                }
                let groups = data_pipeline::read_game_ids_csv(metadata_path).expect("failed to read training group metadata");
                if groups.len() != data_frame.height() {
                    eprintln!("training data and group metadata row counts differ");
                    std::process::exit(2);
                }
                let distinct: std::collections::BTreeSet<_> = groups.iter().copied().collect();
                if distinct.len() < cv_folds + 1 {
                    eprintln!("metadata requires at least cv_folds + 1 distinct games to reserve a chronological test group");
                    std::process::exit(2);
                }
                let all: Vec<_> = distinct.into_iter().collect();
                let test_start = ((all.len() as f64 * development_fraction).floor() as usize).clamp(cv_folds + 1, all.len() - 1);
                let training_groups: std::collections::HashSet<_> = all[..test_start].iter().copied().collect();
                let training_indices: Vec<usize> = groups.iter().enumerate().filter_map(|(i, group)| training_groups.contains(group).then_some(i)).collect();
                let training_index_ca = polars::prelude::IdxCa::from_vec("idx".into(), training_indices.iter().map(|&i| polars::prelude::IdxSize::try_from(i).expect("index fits Polars")).collect());
                data_frame = data_frame.take(&training_index_ca).expect("failed to select pre-test games");
                let training_groups: Vec<_> = training_indices.iter().map(|&i| groups[i]).collect();
                println!("reserved final {} chronological games as test (unused during fitting); {} earlier games available for grouped CV and fitting", all.len() - test_start, test_start);
                training_groups
            } else {
                eprintln!("--metadata is required: game IDs are needed to keep the final chronological holdout out of training");
                std::process::exit(2);
            };
            let cv = training::grouped_cross_validate(&data_frame, &group_ids, model_type.clone(), seed.wrapping_add(4), cv_folds)
                .expect("grouped cross-validation failed");
            println!("grouped_cv_accuracy={:.4} ± {:.4} folds={}", cv.mean_accuracy, cv.std_accuracy, cv.fold_accuracy.len());
            let config = automl::training::TrainingConfig::new(automl::training::TaskType::MultiClassification, "action")
                .with_model(model_type)
                .with_random_state(seed);
            let mut engine = automl::training::TrainEngine::new(config);
            engine.fit(&data_frame).expect("AutoML training failed");
            let probability_check = engine.predict_proba(&data_frame.slice(0, 1))
                .expect("model probability smoke failed before serialization");
            if probability_check.ncols() != 4 {
                eprintln!("selected model/data produced {} probability columns; the 2048 policy requires four classes", probability_check.ncols());
                std::process::exit(2);
            }
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).expect("failed to create model output directory");
            }
            engine.save(output.to_str().expect("model output path is not valid UTF-8"))
                .expect("failed to save model artifact");
            if let Some(metrics) = engine.metrics() {
                println!("saved {}; samples={} features={} validation_accuracy={:?} training_secs={:.3}", output.display(), metrics.n_samples, metrics.n_features, metrics.accuracy, metrics.training_time_secs);
            }
        }
        Some(Commands::Benchmark { command: BenchmarkCommand::Run { model, n_games, seed, output } }) => {
            if n_games == 0 {
                eprintln!("--n-games must be greater than zero");
                std::process::exit(2);
            }
            let model_path = model.to_str().expect("model path is not valid UTF-8");
            let policy = policy::ModelPolicy::load(model_path).expect("failed to load four-class model");
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).expect("failed to create result directory");
            }
            let mut file = std::fs::File::create(&output).expect("failed to create benchmark results");
            use std::io::Write;
            writeln!(file, "game_id,seed,score,max_tile,move_count,game_over,model").expect("failed to write results header");
            let start = std::time::Instant::now();
            let mut scores = Vec::with_capacity(n_games);
            for game_id in 0..n_games {
                let game_seed = seed.wrapping_add(game_id as u64);
                let result = policy::simulate_model_game(game_seed, &policy, 1000)
                    .unwrap_or_else(|error| panic!("game {game_id} failed: {error}"));
                scores.push(result.final_score);
                writeln!(file, "{game_id},{game_seed},{},{},{},{},{}", result.final_score, result.max_tile, result.move_count, result.board.game_over, model.display())
                    .expect("failed to write benchmark result row");
            }
            file.flush().expect("failed to flush benchmark results");
            let summary = evaluation::summarize_scores(&scores, seed.wrapping_add(1), 2_000)
                .expect("benchmark produced at least one game score");
            let elapsed = start.elapsed().as_secs_f64();
            write_json_manifest(&output, &serde_json::json!({
                "created_utc": chrono::Utc::now().to_rfc3339(),
                "project_version": env!("CARGO_PKG_VERSION"),
                "automl_commit": "64f5edad29c9e58ee7d33abf380418d5cfbbb561",
                "source_revision": std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()),
                "sha256": sha256_file(&output).ok(),
                "benchmark": "model_policy",
                "model": model,
                "games": n_games,
                "global_seed": seed,
                "seed_derivation": "global_seed.wrapping_add(game_id)",
                "first_game_seed": seed,
                "last_game_seed": seed.wrapping_add((n_games - 1) as u64),
                "spawn_probability_for_four": 0.1,
                "max_moves": 1000,
                "elapsed_seconds": elapsed,
                "results_csv": output,
                "summary": {"mean": summary.mean, "sample_std_dev": summary.sample_std_dev, "median": summary.median, "p90": summary.percentile_90, "p99": summary.percentile_99, "min": summary.min, "max": summary.max, "games_above_2048": summary.games_above_2048, "games_above_4096": summary.games_above_4096, "games_above_8192": summary.games_above_8192, "mean_ci_95": summary.mean_ci_95}
            })).expect("failed to write benchmark manifest");
            println!("games={} mean_score={:.2} sd={:.2} median={:.0} p90={:.0} p99={:.0} min={} max={} >=2048:{} >=4096:{} >=8192:{} mean_ci95=[{:.2},{:.2}] elapsed_secs={elapsed:.2} results={}", summary.n, summary.mean, summary.sample_std_dev, summary.median, summary.percentile_90, summary.percentile_99, summary.min, summary.max, summary.games_above_2048, summary.games_above_4096, summary.games_above_8192, summary.mean_ci_95.0, summary.mean_ci_95.1, output.display());
        }
        Some(Commands::Benchmark { command: BenchmarkCommand::Baseline { agent, n_games, seed, output } }) => {
            if n_games == 0 {
                eprintln!("--n-games must be greater than zero");
                std::process::exit(2);
            }
            if agent != "random" && agent != "heuristic" {
                eprintln!("baseline --agent must be random or heuristic");
                std::process::exit(2);
            }
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).expect("failed to create result directory");
            }
            use std::io::Write;
            let mut file = std::fs::File::create(&output).expect("failed to create baseline results");
            writeln!(file, "game_id,seed,score,max_tile,move_count,game_over,agent").expect("failed to write results header");
            let start = std::time::Instant::now();
            let mut scores = Vec::with_capacity(n_games);
            for game_id in 0..n_games {
                let game_seed = seed.wrapping_add(game_id as u64);
                let result = match agent.as_str() {
                    "random" => game_engine::simulate_random_game(game_seed, 0.1, 1000),
                    "heuristic" => policy::simulate_heuristic_game(game_seed, 1000)
                        .unwrap_or_else(|error| panic!("game {game_id} failed: {error}")),
                    _ => unreachable!(),
                };
                scores.push(result.final_score);
                writeln!(file, "{game_id},{game_seed},{},{},{},{},{}", result.final_score, result.max_tile, result.move_count, result.board.game_over, agent)
                    .expect("failed to write baseline result row");
            }
            file.flush().expect("failed to flush baseline results");
            let summary = evaluation::summarize_scores(&scores, seed.wrapping_add(1), 2_000).unwrap();
            let elapsed = start.elapsed().as_secs_f64();
            write_json_manifest(&output, &serde_json::json!({
                "created_utc": chrono::Utc::now().to_rfc3339(),
                "project_version": env!("CARGO_PKG_VERSION"),
                "source_revision": std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()),
                "sha256": sha256_file(&output).ok(),
                "benchmark": "baseline",
                "agent": agent,
                "games": n_games,
                "global_seed": seed,
                "seed_derivation": "global_seed.wrapping_add(game_id)",
                "first_game_seed": seed,
                "last_game_seed": seed.wrapping_add((n_games - 1) as u64),
                "spawn_probability_for_four": 0.1,
                "max_moves": 1000,
                "elapsed_seconds": elapsed,
                "results_csv": output,
                "summary": {"mean": summary.mean, "sample_std_dev": summary.sample_std_dev, "median": summary.median, "p90": summary.percentile_90, "p99": summary.percentile_99, "min": summary.min, "max": summary.max, "games_above_2048": summary.games_above_2048, "games_above_4096": summary.games_above_4096, "games_above_8192": summary.games_above_8192, "mean_ci_95": summary.mean_ci_95}
            })).expect("failed to write baseline manifest");
            println!("agent={agent} games={} mean_score={:.2} sd={:.2} median={:.0} p90={:.0} p99={:.0} min={} max={} >=2048:{} >=4096:{} >=8192:{} mean_ci95=[{:.2},{:.2}] elapsed_secs={elapsed:.2} results={}", summary.n, summary.mean, summary.sample_std_dev, summary.median, summary.percentile_90, summary.percentile_99, summary.min, summary.max, summary.games_above_2048, summary.games_above_4096, summary.games_above_8192, summary.mean_ci_95.0, summary.mean_ci_95.1, output.display());
        }
        Some(Commands::Benchmark { command: BenchmarkCommand::Compare { inputs, output, seed } }) => {
            let datasets: Vec<_> = inputs.iter().map(|path| read_benchmark_scores(path).unwrap_or_else(|error| panic!("failed reading {}: {error}", path.display()))).collect();
            let mut comparisons = Vec::new();
            for (index, first) in datasets.iter().enumerate() {
                for second in datasets.iter().skip(index + 1) {
                    let same_seed_set = first.seeds.len() == second.seeds.len() && {
                        let mut a = first.seeds.clone();
                        let mut b = second.seeds.clone();
                        a.sort_unstable();
                        b.sort_unstable();
                        a == b
                    };
                    let paired = same_seed_set;
                    let mut first_scores = first.scores.clone();
                    let mut second_scores = second.scores.clone();
                    if paired {
                        let first_by_seed: std::collections::HashMap<_, _> = first
                            .seeds
                            .iter()
                            .copied()
                            .zip(first.scores.iter().copied())
                            .collect();
                        let second_by_seed: std::collections::HashMap<_, _> = second
                            .seeds
                            .iter()
                            .copied()
                            .zip(second.scores.iter().copied())
                            .collect();
                        let mut common: Vec<_> = first_by_seed.keys().copied().collect();
                        common.sort_unstable();
                        first_scores = common.iter().map(|seed| first_by_seed[seed]).collect();
                        second_scores = common.iter().map(|seed| second_by_seed[seed]).collect();
                    }
                    let (p_value, test) = if paired {
                        (evaluation::paired_sign_test_pvalue(&first_scores, &second_scores), "paired_exact_sign_test".to_string())
                    } else {
                        (evaluation::mann_whitney_u_pvalue(&first.scores, &second.scores), "mann_whitney_u_independent".to_string())
                    };
                    comparisons.push((first.name.clone(), second.name.clone(), p_value.unwrap_or(1.0), test,
                        evaluation::bootstrap_mean_difference_ci(&first_scores, &second_scores, seed.wrapping_add(comparisons.len() as u64), 5_000),
                        evaluation::cohens_d(&first_scores, &second_scores)));
                }
            }
            let adjusted = evaluation::holm_adjust(&comparisons.iter().map(|row| row.2).collect::<Vec<_>>());
            if let Some(parent) = output.parent() { std::fs::create_dir_all(parent).expect("failed to create comparison directory"); }
            use std::io::Write;
            let mut file = std::fs::File::create(&output).expect("failed to create comparison CSV");
            writeln!(file, "first,second,first_n,second_n,first_mean,second_mean,test,p_value,holm_p,mean_difference_ci95_low,mean_difference_ci95_high,cohens_d,seed_sequences_equal,first_input,second_input").unwrap();
            for (index, (first_name, second_name, p, test, ci, d)) in comparisons.iter().enumerate() {
                let first = datasets.iter().find(|data| &data.name == first_name).unwrap();
                let second = datasets.iter().find(|data| &data.name == second_name).unwrap();
                let ci = ci.unwrap_or((f64::NAN, f64::NAN));
                let d = d.unwrap_or(f64::NAN);
                writeln!(file, "{first_name},{second_name},{},{},{:.6},{:.6},{test},{p:.8},{:.8},{:.6},{:.6},{d:.6},{},{},{}", first.scores.len(), second.scores.len(), first.scores.iter().map(|&x| x as f64).sum::<f64>() / first.scores.len() as f64, second.scores.iter().map(|&x| x as f64).sum::<f64>() / second.scores.len() as f64, adjusted[index], ci.0, ci.1, first.seeds == second.seeds, first.input.display(), second.input.display()).unwrap();
            }
            file.flush().unwrap();
            write_json_manifest(&output, &serde_json::json!({"created_utc": chrono::Utc::now().to_rfc3339(), "project_version": env!("CARGO_PKG_VERSION"), "analysis": "matched seed sequences use exact two-sided sign test; unmatched runs use independent Mann-Whitney U; Holm adjustment; independent bootstrap CI of mean score difference and Cohen's d", "paired_test_limitation": "sign test uses direction and ignores ties; it is conservative and is not Wilcoxon", "seed_sequences_checked_for_equality": true, "input_files": datasets.iter().map(|data| &data.input).collect::<Vec<_>>(), "comparison_csv": output})).expect("failed to write comparison manifest");
            let manifest_path = output.with_extension("manifest.json");
            let mut manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(&manifest_path).unwrap()).unwrap();
            manifest["source_revision"] = serde_json::json!(std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()));
            manifest["sha256"] = serde_json::json!(sha256_file(&output).ok());
            std::fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
            println!("compared {} runs ({} pairwise comparisons); output={} manifest={}", datasets.len(), comparisons.len(), output.display(), output.with_extension("manifest.json").display());
        }
        Some(Commands::Benchmark { command: BenchmarkCommand::Report { inputs, output } }) => {
            let datasets: Vec<_> = inputs.iter().map(|path| read_benchmark_scores(path).unwrap_or_else(|error| panic!("failed reading {}: {error}", path.display()))).collect();
            if let Some(parent) = output.parent() { std::fs::create_dir_all(parent).expect("failed to create report directory"); }
            use std::io::Write;
            let mut file = std::fs::File::create(&output).expect("failed to create report CSV");
            writeln!(file, "name,n,mean,sample_std_dev,median,p90,p99,min,max,games_above_2048,games_above_4096,games_above_8192,mean_ci95_low,mean_ci95_high,input").unwrap();
            for (index, data) in datasets.iter().enumerate() {
                let summary = evaluation::summarize_scores(&data.scores, 71_u64.wrapping_add(index as u64), 5_000).unwrap();
                writeln!(file, "{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{},{},{},{},{},{:.6},{:.6},{}", data.name, summary.n, summary.mean, summary.sample_std_dev, summary.median, summary.percentile_90, summary.percentile_99, summary.min, summary.max, summary.games_above_2048, summary.games_above_4096, summary.games_above_8192, summary.mean_ci_95.0, summary.mean_ci_95.1, data.input.display()).unwrap();
            }
            file.flush().unwrap();
            write_json_manifest(&output, &serde_json::json!({"created_utc": chrono::Utc::now().to_rfc3339(), "project_version": env!("CARGO_PKG_VERSION"), "analysis": "descriptive game-score statistics with percentile bootstrap mean CI", "bootstrap_replicates": 5000, "inputs": datasets.iter().map(|data| &data.input).collect::<Vec<_>>(), "report_csv": output})).expect("failed to write report manifest");
            let manifest_path = output.with_extension("manifest.json");
            let mut manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(&manifest_path).unwrap()).unwrap();
            manifest["source_revision"] = serde_json::json!(std::process::Command::new("git").args(["rev-parse", "HEAD"]).output().ok().filter(|result| result.status.success()).map(|result| String::from_utf8_lossy(&result.stdout).trim().to_owned()));
            manifest["sha256"] = serde_json::json!(sha256_file(&output).ok());
            std::fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
            println!("summarized {} runs to {}", datasets.len(), output.display());
        }
    }
}
