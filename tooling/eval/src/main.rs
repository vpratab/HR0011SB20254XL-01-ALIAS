// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

use anyhow::{Context, Result};
use rtvlas_core::{default_profile, evaluate_scenario, write_evidence_bundle, AutonomySnapshot, EvidenceBundle};
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
        anyhow::bail!("usage: cargo run -p eval_tool -- <scenario_name> <input.jsonl> <output_dir>");
    }
    let scenario_name = &args[1];
    let input = Path::new(&args[2]);
    let output = Path::new(&args[3]);
    let snapshots = read_jsonl(input)?;
    let (timeline, scorecard) = evaluate_scenario(default_profile(), scenario_name, &snapshots);
    let bundle = EvidenceBundle { timeline, scorecard };
    write_evidence_bundle(output, input, &snapshots, &bundle)?;
    println!(
        "scenario={} frames={} final_trust={:.3} first_reject={:?}",
        scenario_name,
        bundle.scorecard.total_frames,
        bundle.scorecard.final_trust_score,
        bundle.scorecard.first_reject_frame
    );
    Ok(())
}
