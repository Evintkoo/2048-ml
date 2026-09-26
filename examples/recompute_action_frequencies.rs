#[allow(dead_code)]
#[path = "../src/evaluation.rs"]
mod evaluation;

use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};

fn main() -> Result<()> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 4 {
        bail!("usage: cargo run --example recompute_action_frequencies -- <results.csv> <manifest.json> <output.json>");
    }
    let csv_path = Path::new(&args[1]);
    let manifest_path = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);
    let csv_bytes = fs::read(csv_path)?;
    let manifest: Value = serde_json::from_slice(&fs::read(manifest_path)?)?;
    let digest = format!("{:x}", Sha256::digest(&csv_bytes));
    if manifest["sha256"].as_str() != Some(&digest) {
        bail!("CSV digest does not match the run manifest");
    }

    let content = std::str::from_utf8(&csv_bytes)?;
    let mut lines = content.lines();
    let header: Vec<_> = lines.next().context("CSV is empty")?.split(',').collect();
    let column = |name: &str| -> Result<usize> {
        header
            .iter()
            .position(|column| *column == name)
            .with_context(|| format!("missing CSV column {name}"))
    };
    let game_id_col = column("game_id")?;
    let seed_col = column("seed")?;
    let move_count_col = column("move_count")?;
    let action_cols = [
        column("up_moves")?,
        column("down_moves")?,
        column("left_moves")?,
        column("right_moves")?,
    ];
    let first_seed = manifest["first_game_seed"]
        .as_u64()
        .context("manifest has no first seed")?;
    let bootstrap_seed = manifest["action_frequency_bootstrap_seed"]
        .as_u64()
        .context("manifest has no action-frequency bootstrap seed")?;
    let replicates = 2_000;
    let mut rows: Vec<[u64; 4]> = Vec::new();

    for (expected_id, line) in lines.enumerate() {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != header.len() {
            bail!(
                "CSV row {} has {} fields; expected {}",
                expected_id + 2,
                fields.len(),
                header.len()
            );
        }
        let id: u64 = fields[game_id_col].parse()?;
        let seed: u64 = fields[seed_col].parse()?;
        if id != expected_id as u64 || seed != first_seed.wrapping_add(id) {
            bail!("unexpected game id or seed at row {}", expected_id + 2);
        }
        let actions = [
            fields[action_cols[0]].parse()?,
            fields[action_cols[1]].parse()?,
            fields[action_cols[2]].parse()?,
            fields[action_cols[3]].parse()?,
        ];
        let moves: u64 = fields[move_count_col].parse()?;
        if actions.iter().sum::<u64>() != moves {
            bail!("action counts do not sum to move_count for game {id}");
        }
        rows.push(actions);
    }
    if manifest["games"].as_u64() != Some(rows.len() as u64) {
        bail!("CSV row count does not match manifest game count");
    }
    let summaries = evaluation::summarize_action_frequencies(&rows, bootstrap_seed, replicates)
        .context("no action frequencies could be summarized")?;
    let action_total: u64 = rows.iter().flatten().sum();
    let directions = ["up", "down", "left", "right"];
    let frequencies: Vec<_> = summaries
        .iter()
        .enumerate()
        .map(|(action, summary)| {
            json!({
                "action": action,
                "direction": directions[action],
                "count": summary.count,
                "total_moves": action_total,
                "proportion": summary.proportion,
                "game_cluster_bootstrap_95_ci": [summary.ci_95.0, summary.ci_95.1]
            })
        })
        .collect();

    let manifest_frequencies = manifest["action_frequency"]
        .as_array()
        .context("manifest has no action frequency summaries")?;
    if manifest_frequencies.len() != frequencies.len() {
        bail!("manifest action-frequency row count differs");
    }
    for (calculated, reported) in frequencies.iter().zip(manifest_frequencies) {
        if calculated["count"] != reported["count"]
            || calculated["total_moves"] != reported["total_moves"]
            || calculated["proportion"] != reported["proportion"]
            || calculated["game_cluster_bootstrap_95_ci"]
                != reported["game_cluster_bootstrap_95_ci"]
        {
            bail!(
                "recomputed summary differs from manifest for {}",
                calculated["direction"]
            );
        }
    }

    let output = json!({
        "schema": "2048-ml.independently-recomputed-action-frequency",
        "source_csv": csv_path,
        "source_manifest": manifest_path,
        "source_csv_sha256": digest,
        "games": rows.len(),
        "first_game_seed": first_seed,
        "last_game_seed": first_seed.wrapping_add(rows.len() as u64 - 1),
        "bootstrap_seed": bootstrap_seed,
        "bootstrap_replicates": replicates,
        "action_total": action_total,
        "exact_manifest_match": true,
        "action_frequency": frequencies
    });
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, serde_json::to_vec_pretty(&output)?)?;
    println!(
        "independent raw-CSV action summary exactly matches manifest: games={} moves={} output={}",
        rows.len(),
        action_total,
        output_path.display()
    );
    Ok(())
}
