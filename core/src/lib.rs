
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

mod logger;
mod model;
mod monitor;
mod profile;

pub use logger::{write_evidence_bundle, EvidenceBundle};
pub use model::{
    AutonomySnapshot, BoolField, FrameAssessment, NumericField, PropertyKind, PropertyOutcome,
    PropertySpec, Scorecard, TrustInputs, TrustVerdict,
};
pub use monitor::{evaluate_scenario, Monitor, MonitorProfile};
pub use profile::{default_profile, nominal_snapshot};
