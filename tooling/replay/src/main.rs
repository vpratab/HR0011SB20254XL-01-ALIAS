// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

use anyhow::{Context, Result};
use rtvlas_core::{default_profile, evaluate_scenario, AutonomySnapshot};
use std::env;
use std::fs;
use std::path::Path;

fn read_jsonl(path: &Path) -> Result<Vec<AutonomySnapshot>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read input log {}", path.display()))?;
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<AutonomySnapshot>(line).context("invalid snapshot json"))
        .collect()
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        anyhow::bail!("usage: cargo run -p replay_tool -- <scenario_name> <input.jsonl> <output_dir>");
    }
    let scenario_name = &args[1];
    let input = Path::new(&args[2]);
    let output_dir = Path::new(&args[3]);
    fs::create_dir_all(output_dir)?;
    let snapshots = read_jsonl(input)?;
    let (timeline, scorecard) = evaluate_scenario(default_profile(), scenario_name, &snapshots);
    fs::write(
        output_dir.join("timeline_replay.json"),
        serde_json::to_string_pretty(&timeline)?,
    )?;
    fs::write(
        output_dir.join("scorecard_replay.json"),
        serde_json::to_string_pretty(&scorecard)?,
    )?;
    println!("replayed={} frames={} final_trust={:.3}", scenario_name, scorecard.total_frames, scorecard.final_trust_score);
    Ok(())
}
