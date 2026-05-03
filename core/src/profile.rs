
        // SPDX-License-Identifier: Apache-2.0
        //
        // Copyright (c) 2025 RTVLAS contributors

        use crate::model::{AutonomySnapshot, BoolField, NumericField, PropertyKind, PropertySpec, TrustInputs};
        use crate::monitor::MonitorProfile;

        pub fn default_profile() -> MonitorProfile {
            MonitorProfile {
                topic_id: "HR0011SB20254XL-01".to_string(),
                title: "ALIAS Missionized Autonomy for Emergency Services".to_string(),
                framing: "ALIAS/MATRIX mission-app assurance for emergency autonomy".to_string(),
                properties: vec![
        PropertySpec::new(
            "hazard_standoff",
            "Fireline Standoff Margin",
            "Ensures the mission app maintains minimum distance from dynamic fireline hazards, structures, and exclusion zones.",
            PropertyKind::MinMargin { field: NumericField::HazardDistanceM, reference: NumericField::MinHazardDistanceM },
            1.2,
        ),
        PropertySpec::new(
            "safe_altitude_margin",
            "Terrain and Smoke Clearance Margin",
            "Checks whether the mission autonomy retains enough altitude margin to clear terrain, smoke corridors, and recovery approaches.",
            PropertyKind::MinValue { field: NumericField::SafeAltitudeMarginM, min: 30.0 },
            0.9,
        ),
        PropertySpec::new(
            "recovery_zone_access",
            "Divert / Dip Site Reachability",
            "Ensures a safe divert point, dip site, or recovery zone remains within an acceptable reachability radius while the mission app is active.",
            PropertyKind::MaxValue { field: NumericField::RecoveryZoneDistanceM, max: 1600.0 },
            1.0,
        ),
        PropertySpec::new(
            "emergency_response_ready",
            "Mission App Recovery Readiness",
            "Detects whether the mission stack remains in a recovery-capable state when emergency transitions are needed.",
            PropertyKind::BooleanGate { field: BoolField::EmergencyResponseReady, reject_on_false: true },
            1.1,
        ),
        PropertySpec::new(
            "operator_link_health",
            "Air-Ground Coordination Link",
            "Tracks whether operator or incident-command supervision remains available for critical missionized autonomy review and override steps.",
            PropertyKind::BooleanGate { field: BoolField::OperatorLink, reject_on_false: true },
            0.8,
        )
                ],
            }
        }

        pub fn nominal_snapshot() -> AutonomySnapshot {
            AutonomySnapshot {
    timestamp_ms: 0,
    position_m: [0.0, 0.0, 260.0],
    velocity_mps: [22.0, 1.5, 0.0],
    heading_rad: 0.08,
    trust_inputs: TrustInputs {
        gps_valid: true,
        operator_link: true,
        autonomy_solution_feasible: true,
        mission_plan_valid: true,
        emergency_response_ready: true,
        temporal_skew_ms: 12.0,
        corridor_error_m: 8.0,
        corridor_half_width_m: 24.0,
        command_speed_mps: 26.0,
        max_safe_speed_mps: 38.0,
        deconfliction_margin_m: 55.0,
        min_deconfliction_margin_m: 25.0,
        formation_spacing_m: 40.0,
        desired_spacing_m: 40.0,
        heading_error_rad: 0.05,
        threat_distance_m: 76.0,
        threat_min_distance_m: 46.0,
        wez_exposure: 0.18,
        route_efficiency: 0.91,
        decision_latency_ms: 140.0,
        operator_intent_alignment: 0.94,
        evidence_completeness: 0.97,
        hazard_distance_m: 74.0,
        min_hazard_distance_m: 42.0,
        safe_altitude_margin_m: 48.0,
        recovery_zone_distance_m: 920.0,
        max_recovery_zone_distance_m: 1600.0,
        autonomy_solution_optimality: 0.91,
    },
}
        }
