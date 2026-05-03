// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutonomySnapshot {
    pub timestamp_ms: u64,
    pub position_m: [f64; 3],
    pub velocity_mps: [f64; 3],
    pub heading_rad: f64,
    pub trust_inputs: TrustInputs,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrustInputs {
    pub gps_valid: bool,
    pub operator_link: bool,
    pub autonomy_solution_feasible: bool,
    pub mission_plan_valid: bool,
    pub emergency_response_ready: bool,
    pub temporal_skew_ms: f64,
    pub corridor_error_m: f64,
    pub corridor_half_width_m: f64,
    pub command_speed_mps: f64,
    pub max_safe_speed_mps: f64,
    pub deconfliction_margin_m: f64,
    pub min_deconfliction_margin_m: f64,
    pub formation_spacing_m: f64,
    pub desired_spacing_m: f64,
    pub heading_error_rad: f64,
    pub threat_distance_m: f64,
    pub threat_min_distance_m: f64,
    pub wez_exposure: f64,
    pub route_efficiency: f64,
    pub decision_latency_ms: f64,
    pub operator_intent_alignment: f64,
    pub evidence_completeness: f64,
    pub hazard_distance_m: f64,
    pub min_hazard_distance_m: f64,
    pub safe_altitude_margin_m: f64,
    pub recovery_zone_distance_m: f64,
    pub max_recovery_zone_distance_m: f64,
    pub autonomy_solution_optimality: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum TrustVerdict {
    Accept = 0,
    Flag = 1,
    Reject = 2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyOutcome {
    pub property_key: String,
    pub property_name: String,
    pub verdict: TrustVerdict,
    pub observed: f64,
    pub threshold: f64,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameAssessment {
    pub frame_index: usize,
    pub timestamp_ms: u64,
    pub trust_score: f32,
    pub verdict: TrustVerdict,
    pub outcomes: Vec<PropertyOutcome>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scorecard {
    pub topic_id: String,
    pub title: String,
    pub scenario_name: String,
    pub total_frames: usize,
    pub accept_frames: usize,
    pub flag_frames: usize,
    pub reject_frames: usize,
    pub min_trust_score: f32,
    pub final_trust_score: f32,
    pub first_reject_frame: Option<usize>,
    pub violations_by_property: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertySpec {
    pub key: String,
    pub name: String,
    pub description: String,
    pub kind: PropertyKind,
    pub weight: f32,
}

impl PropertySpec {
    pub fn new(key: &str, name: &str, description: &str, kind: PropertyKind, weight: f32) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            kind,
            weight,
        }
    }

    pub fn evaluate(&self, snapshot: &AutonomySnapshot) -> PropertyOutcome {
        self.kind.evaluate(self, snapshot)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropertyKind {
    BooleanGate { field: BoolField, reject_on_false: bool },
    MaxValue { field: NumericField, max: f64 },
    MinValue { field: NumericField, min: f64 },
    MinMargin { field: NumericField, reference: NumericField },
    CorridorContainment,
    FormationSpacing { tolerance_m: f64 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BoolField {
    GpsValid,
    OperatorLink,
    AutonomySolutionFeasible,
    MissionPlanValid,
    EmergencyResponseReady,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NumericField {
    TemporalSkewMs,
    CorridorErrorM,
    CorridorHalfWidthM,
    CommandSpeedMps,
    MaxSafeSpeedMps,
    DeconflictionMarginM,
    MinDeconflictionMarginM,
    FormationSpacingM,
    DesiredSpacingM,
    HeadingErrorRad,
    ThreatDistanceM,
    ThreatMinDistanceM,
    WezExposure,
    RouteEfficiency,
    DecisionLatencyMs,
    OperatorIntentAlignment,
    EvidenceCompleteness,
    HazardDistanceM,
    MinHazardDistanceM,
    SafeAltitudeMarginM,
    RecoveryZoneDistanceM,
    MaxRecoveryZoneDistanceM,
    AutonomySolutionOptimality,
}

impl BoolField {
    pub fn read(self, snapshot: &AutonomySnapshot) -> bool {
        match self {
            Self::GpsValid => snapshot.trust_inputs.gps_valid,
            Self::OperatorLink => snapshot.trust_inputs.operator_link,
            Self::AutonomySolutionFeasible => snapshot.trust_inputs.autonomy_solution_feasible,
            Self::MissionPlanValid => snapshot.trust_inputs.mission_plan_valid,
            Self::EmergencyResponseReady => snapshot.trust_inputs.emergency_response_ready,
        }
    }
}

impl NumericField {
    pub fn read(self, snapshot: &AutonomySnapshot) -> f64 {
        match self {
            Self::TemporalSkewMs => snapshot.trust_inputs.temporal_skew_ms,
            Self::CorridorErrorM => snapshot.trust_inputs.corridor_error_m,
            Self::CorridorHalfWidthM => snapshot.trust_inputs.corridor_half_width_m,
            Self::CommandSpeedMps => snapshot.trust_inputs.command_speed_mps,
            Self::MaxSafeSpeedMps => snapshot.trust_inputs.max_safe_speed_mps,
            Self::DeconflictionMarginM => snapshot.trust_inputs.deconfliction_margin_m,
            Self::MinDeconflictionMarginM => snapshot.trust_inputs.min_deconfliction_margin_m,
            Self::FormationSpacingM => snapshot.trust_inputs.formation_spacing_m,
            Self::DesiredSpacingM => snapshot.trust_inputs.desired_spacing_m,
            Self::HeadingErrorRad => snapshot.trust_inputs.heading_error_rad,
            Self::ThreatDistanceM => snapshot.trust_inputs.threat_distance_m,
            Self::ThreatMinDistanceM => snapshot.trust_inputs.threat_min_distance_m,
            Self::WezExposure => snapshot.trust_inputs.wez_exposure,
            Self::RouteEfficiency => snapshot.trust_inputs.route_efficiency,
            Self::DecisionLatencyMs => snapshot.trust_inputs.decision_latency_ms,
            Self::OperatorIntentAlignment => snapshot.trust_inputs.operator_intent_alignment,
            Self::EvidenceCompleteness => snapshot.trust_inputs.evidence_completeness,
            Self::HazardDistanceM => snapshot.trust_inputs.hazard_distance_m,
            Self::MinHazardDistanceM => snapshot.trust_inputs.min_hazard_distance_m,
            Self::SafeAltitudeMarginM => snapshot.trust_inputs.safe_altitude_margin_m,
            Self::RecoveryZoneDistanceM => snapshot.trust_inputs.recovery_zone_distance_m,
            Self::MaxRecoveryZoneDistanceM => snapshot.trust_inputs.max_recovery_zone_distance_m,
            Self::AutonomySolutionOptimality => snapshot.trust_inputs.autonomy_solution_optimality,
        }
    }
}

fn scale_verdict(pass: bool, soft_pass: bool) -> TrustVerdict {
    if pass {
        TrustVerdict::Accept
    } else if soft_pass {
        TrustVerdict::Flag
    } else {
        TrustVerdict::Reject
    }
}

impl PropertyKind {
    pub fn evaluate(&self, spec: &PropertySpec, snapshot: &AutonomySnapshot) -> PropertyOutcome {
        match self {
            Self::BooleanGate { field, reject_on_false } => {
                let observed = if field.read(snapshot) { 1.0 } else { 0.0 };
                let verdict = if observed > 0.5 {
                    TrustVerdict::Accept
                } else if *reject_on_false {
                    TrustVerdict::Reject
                } else {
                    TrustVerdict::Flag
                };
                PropertyOutcome {
                    property_key: spec.key.clone(),
                    property_name: spec.name.clone(),
                    verdict,
                    observed,
                    threshold: 1.0,
                    message: format!("{} requires {:?} to remain true", spec.name, field),
                }
            }
            Self::MaxValue { field, max } => {
                let observed = field.read(snapshot);
                let soft_limit = max * 1.15;
                PropertyOutcome {
                    property_key: spec.key.clone(),
                    property_name: spec.name.clone(),
                    verdict: scale_verdict(observed <= *max, observed <= soft_limit),
                    observed,
                    threshold: *max,
                    message: format!("{} keeps {:?} below {:.3}", spec.name, field, max),
                }
            }
            Self::MinValue { field, min } => {
                let observed = field.read(snapshot);
                let soft_limit = min * 0.85;
                PropertyOutcome {
                    property_key: spec.key.clone(),
                    property_name: spec.name.clone(),
                    verdict: scale_verdict(observed >= *min, observed >= soft_limit),
                    observed,
                    threshold: *min,
                    message: format!("{} keeps {:?} above {:.3}", spec.name, field, min),
                }
            }
            Self::MinMargin { field, reference } => {
                let observed = field.read(snapshot);
                let threshold = reference.read(snapshot);
                let soft_limit = threshold * 0.85;
                PropertyOutcome {
                    property_key: spec.key.clone(),
                    property_name: spec.name.clone(),
                    verdict: scale_verdict(observed >= threshold, observed >= soft_limit),
                    observed,
                    threshold,
                    message: format!("{} compares {:?} against {:?}", spec.name, field, reference),
                }
            }
            Self::CorridorContainment => {
                let observed = snapshot.trust_inputs.corridor_error_m;
                let threshold = snapshot.trust_inputs.corridor_half_width_m;
                let soft_limit = threshold * 1.10;
                PropertyOutcome {
                    property_key: spec.key.clone(),
                    property_name: spec.name.clone(),
                    verdict: scale_verdict(observed <= threshold, observed <= soft_limit),
                    observed,
                    threshold,
                    message: "corridor error must remain inside the assigned half width".to_string(),
                }
            }
            Self::FormationSpacing { tolerance_m } => {
                let observed = (snapshot.trust_inputs.formation_spacing_m
                    - snapshot.trust_inputs.desired_spacing_m)
                    .abs();
                let soft_limit = tolerance_m * 1.25;
                PropertyOutcome {
                    property_key: spec.key.clone(),
                    property_name: spec.name.clone(),
                    verdict: scale_verdict(observed <= *tolerance_m, observed <= soft_limit),
                    observed,
                    threshold: *tolerance_m,
                    message: "formation spacing error must remain bounded".to_string(),
                }
            }
        }
    }
}
