
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

use crate::model::{AutonomySnapshot, FrameAssessment, PropertySpec, Scorecard, TrustVerdict};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct MonitorProfile {
    pub topic_id: String,
    pub title: String,
    pub framing: String,
    pub properties: Vec<PropertySpec>,
}

#[derive(Clone, Debug)]
pub struct Monitor {
    profile: MonitorProfile,
    trust_score: f32,
    frame_index: usize,
    timeline: Vec<FrameAssessment>,
}

impl Monitor {
    pub fn new(profile: MonitorProfile) -> Self {
        Self {
            profile,
            trust_score: 1.0,
            frame_index: 0,
            timeline: Vec::new(),
        }
    }

    pub fn update(&mut self, snapshot: &AutonomySnapshot) -> FrameAssessment {
        let outcomes = self
            .profile
            .properties
            .iter()
            .map(|property| property.evaluate(snapshot))
            .collect::<Vec<_>>();

        let verdict = outcomes
            .iter()
            .map(|outcome| outcome.verdict)
            .max()
            .unwrap_or(TrustVerdict::Accept);

        for (property, outcome) in self.profile.properties.iter().zip(outcomes.iter()) {
            match outcome.verdict {
                TrustVerdict::Accept => {
                    self.trust_score = (self.trust_score + 0.003 * property.weight).min(1.0);
                }
                TrustVerdict::Flag => {
                    self.trust_score = (self.trust_score - 0.008 * property.weight).max(0.0);
                }
                TrustVerdict::Reject => {
                    self.trust_score = (self.trust_score - 0.04 * property.weight).max(0.0);
                }
            }
        }

        let assessment = FrameAssessment {
            frame_index: self.frame_index,
            timestamp_ms: snapshot.timestamp_ms,
            trust_score: self.trust_score,
            verdict,
            outcomes,
        };
        self.timeline.push(assessment.clone());
        self.frame_index += 1;
        assessment
    }

    pub fn trust_score(&self) -> f32 {
        self.trust_score
    }

    pub fn timeline(&self) -> &[FrameAssessment] {
        &self.timeline
    }

    pub fn profile(&self) -> &MonitorProfile {
        &self.profile
    }
}

pub fn evaluate_scenario(profile: MonitorProfile, scenario_name: &str, snapshots: &[AutonomySnapshot]) -> (Vec<FrameAssessment>, Scorecard) {
    let mut monitor = Monitor::new(profile.clone());
    for snapshot in snapshots {
        monitor.update(snapshot);
    }
    let timeline = monitor.timeline.clone();
    let mut violations_by_property = BTreeMap::new();
    let mut accept_frames = 0;
    let mut flag_frames = 0;
    let mut reject_frames = 0;
    let mut first_reject_frame = None;

    for frame in &timeline {
        match frame.verdict {
            TrustVerdict::Accept => accept_frames += 1,
            TrustVerdict::Flag => flag_frames += 1,
            TrustVerdict::Reject => {
                reject_frames += 1;
                if first_reject_frame.is_none() {
                    first_reject_frame = Some(frame.frame_index);
                }
            }
        }
        for outcome in &frame.outcomes {
            if outcome.verdict != TrustVerdict::Accept {
                *violations_by_property
                    .entry(outcome.property_name.clone())
                    .or_insert(0) += 1;
            }
        }
    }

    let min_trust_score = timeline
        .iter()
        .map(|frame| frame.trust_score)
        .fold(1.0_f32, |acc, value| acc.min(value));

    let scorecard = Scorecard {
        topic_id: profile.topic_id,
        title: profile.title,
        scenario_name: scenario_name.to_string(),
        total_frames: timeline.len(),
        accept_frames,
        flag_frames,
        reject_frames,
        min_trust_score,
        final_trust_score: timeline.last().map(|frame| frame.trust_score).unwrap_or(1.0),
        first_reject_frame,
        violations_by_property,
    };

    (timeline, scorecard)
}
