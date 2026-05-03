// SPDX-License-Identifier: Apache-2.0
    //
    // Copyright (c) 2025 RTVLAS contributors

    use crate::model::{AutonomySnapshot, FrameAssessment, Scorecard, TrustVerdict};
    use serde_json::json;
    use std::fs;
    use std::io;
    use std::path::Path;

    #[derive(Clone, Debug)]
    pub struct EvidenceBundle {
        pub timeline: Vec<FrameAssessment>,
        pub scorecard: Scorecard,
    }

    fn json_string<T: serde::Serialize>(value: &T) -> io::Result<String> {
        serde_json::to_string_pretty(value).map_err(io::Error::other)
    }

    pub fn write_evidence_bundle(output_dir: &Path, input_log: &Path, snapshots: &[AutonomySnapshot], bundle: &EvidenceBundle) -> io::Result<()> {
        fs::create_dir_all(output_dir)?;
        fs::copy(input_log, output_dir.join("input_log.jsonl"))?;
        fs::write(output_dir.join("timeline.json"), json_string(&bundle.timeline)?)?;
        fs::write(output_dir.join("trust_scorecard.json"), json_string(&bundle.scorecard)?)?;
        fs::write(output_dir.join("proof_log.txt"), render_proof_log(&bundle.timeline))?;
        fs::write(output_dir.join("trace.svg"), render_trace_svg(&bundle.timeline))?;
        fs::write(
            output_dir.join("summary.json"),
            json_string(&json!({
                "frames": snapshots.len(),
                "final_trust": bundle.scorecard.final_trust_score,
                "first_reject_frame": bundle.scorecard.first_reject_frame,
            }))?,
        )?;
        Ok(())
    }

    fn render_proof_log(timeline: &[FrameAssessment]) -> String {
        let mut lines = vec!["# RTVLAS Proof Log".to_string()];
        for frame in timeline {
            if frame.verdict == TrustVerdict::Accept {
                continue;
            }
            let causes = frame
                .outcomes
                .iter()
                .filter(|outcome| outcome.verdict != TrustVerdict::Accept)
                .map(|outcome| format!("{}={:?}", outcome.property_name, outcome.verdict))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "frame={} timestamp_ms={} verdict={:?} trust={:.3} causes=[{}]",
                frame.frame_index, frame.timestamp_ms, frame.verdict, frame.trust_score, causes
            ));
        }
        if lines.len() == 1 {
            lines.push("no violations recorded".to_string());
        }
        lines.join("
") + "
"
    }

    fn render_trace_svg(timeline: &[FrameAssessment]) -> String {
        let width = 960.0_f32;
        let height = 320.0_f32;
        let left = 60.0_f32;
        let right = 20.0_f32;
        let top = 20.0_f32;
        let bottom = 45.0_f32;
        let graph_width = width - left - right;
        let graph_height = height - top - bottom;
        let frame_count = timeline.len().max(1) as f32;
        let points = timeline
            .iter()
            .enumerate()
            .map(|(index, frame)| {
                let x = left + (index as f32 / (frame_count - 1.0).max(1.0)) * graph_width;
                let y = top + (1.0 - frame.trust_score) * graph_height;
                format!("{x:.2},{y:.2}")
            })
            .collect::<Vec<_>>()
            .join(" ");

        let mut markers = String::new();
        for (index, frame) in timeline.iter().enumerate() {
            if frame.verdict == TrustVerdict::Accept {
                continue;
            }
            let x = left + (index as f32 / (frame_count - 1.0).max(1.0)) * graph_width;
            let y = top + (1.0 - frame.trust_score) * graph_height;
            let color = if frame.verdict == TrustVerdict::Reject {
                "#C0392B"
            } else {
                "#F39C12"
            };
            markers.push_str(&format!(
                "<circle cx=\"{x:.2}\" cy=\"{y:.2}\" r=\"4\" fill=\"{color}\" />"
            ));
        }

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">             <rect width=\"100%\" height=\"100%\" fill=\"#0B1020\" />             <line x1=\"{left}\" y1=\"{top}\" x2=\"{left}\" y2=\"{height_bottom}\" stroke=\"#94A3B8\" stroke-width=\"1\" />             <line x1=\"{left}\" y1=\"{height_bottom}\" x2=\"{width_right}\" y2=\"{height_bottom}\" stroke=\"#94A3B8\" stroke-width=\"1\" />             <text x=\"{left}\" y=\"14\" fill=\"#E2E8F0\" font-size=\"14\">Trust Score Over Time</text>             <polyline fill=\"none\" stroke=\"#38BDF8\" stroke-width=\"3\" points=\"{points}\" />             {markers}             <text x=\"{left}\" y=\"{height_minus_6}\" fill=\"#CBD5E1\" font-size=\"12\">0</text>             <text x=\"{left}\" y=\"{top_plus_12}\" fill=\"#CBD5E1\" font-size=\"12\">1</text>             </svg>",
            height_bottom = height - bottom,
            width_right = width - right,
            height_minus_6 = height - bottom + 18.0,
            top_plus_12 = top + 12.0,
        )
    }
