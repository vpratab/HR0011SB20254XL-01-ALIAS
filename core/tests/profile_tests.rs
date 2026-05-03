
            // SPDX-License-Identifier: Apache-2.0
            //
            // Copyright (c) 2025 RTVLAS contributors

            use rtvlas_core::{default_profile, evaluate_scenario, nominal_snapshot, EvidenceBundle, TrustVerdict, write_evidence_bundle};
            use std::fs;

#[test]
            fn hazard_standoff_raises_signal() {
                let profile = default_profile();
                let index = profile
                    .properties
                    .iter()
                    .position(|property| property.key == "hazard_standoff")
                    .expect("property present");
                let property = profile.properties[index].clone();
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.hazard_distance_m = 38.0;
snapshot.trust_inputs.min_hazard_distance_m = 42.0;
                let outcome = property.evaluate(&snapshot);
                assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
                assert_eq!(outcome.property_key, "hazard_standoff");
            }

#[test]
fn safe_altitude_margin_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "safe_altitude_margin")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.safe_altitude_margin_m = 25.0;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "safe_altitude_margin");
}

#[test]
fn recovery_zone_access_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "recovery_zone_access")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.recovery_zone_distance_m = 1740.0;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "recovery_zone_access");
}

#[test]
fn emergency_response_ready_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "emergency_response_ready")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.emergency_response_ready = false;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "emergency_response_ready");
}

#[test]
fn operator_link_health_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "operator_link_health")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.operator_link = false;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "operator_link_health");
}

            #[test]
            fn evidence_pipeline_writes_expected_files() {
                let profile = default_profile();
                let scenario_name = "test_scenario";
                let snapshots = vec![nominal_snapshot(), nominal_snapshot()];
                let (timeline, scorecard) = evaluate_scenario(profile, scenario_name, &snapshots);
                let bundle = EvidenceBundle { timeline, scorecard };
                let temp_dir = std::env::temp_dir().join("rtvlas_phase1_evidence");
                let _ = fs::remove_dir_all(&temp_dir);
                fs::create_dir_all(&temp_dir).expect("temp dir");
                let input_log = temp_dir.join("input.jsonl");
                fs::write(
                    &input_log,
                    snapshots
                        .iter()
                        .map(|snapshot| serde_json::to_string(snapshot).expect("json"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("input log");
                write_evidence_bundle(&temp_dir, &input_log, &snapshots, &bundle).expect("evidence bundle");
                assert!(temp_dir.join("trust_scorecard.json").exists());
                assert!(temp_dir.join("timeline.json").exists());
                assert!(temp_dir.join("proof_log.txt").exists());
                assert!(temp_dir.join("trace.svg").exists());
            }

            #[test]
            fn reject_path_drops_trust() {
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.hazard_distance_m = 14.0;
    snapshot.trust_inputs.min_hazard_distance_m = 42.0;
                let (timeline, scorecard) = evaluate_scenario(default_profile(), "reject_case", &[snapshot]);
                assert_eq!(timeline.len(), 1);
                assert!(scorecard.final_trust_score < 1.0);
                assert!(scorecard.reject_frames >= 1 || scorecard.flag_frames >= 1);
            }
