// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

use rtvlas_core::{default_profile, AutonomySnapshot, Monitor, TrustInputs};

#[repr(C)]
pub struct RtVlasSnapshot {
    pub timestamp_ms: u64,
    pub position_x_m: f64,
    pub position_y_m: f64,
    pub position_z_m: f64,
    pub velocity_x_mps: f64,
    pub velocity_y_mps: f64,
    pub velocity_z_mps: f64,
    pub heading_rad: f64,
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

struct MonitorHandle {
    monitor: Monitor,
    last_verdict: i32,
}

impl From<&RtVlasSnapshot> for AutonomySnapshot {
    fn from(value: &RtVlasSnapshot) -> Self {
        AutonomySnapshot {
            timestamp_ms: value.timestamp_ms,
            position_m: [value.position_x_m, value.position_y_m, value.position_z_m],
            velocity_mps: [value.velocity_x_mps, value.velocity_y_mps, value.velocity_z_mps],
            heading_rad: value.heading_rad,
            trust_inputs: TrustInputs {
                gps_valid: value.gps_valid,
                operator_link: value.operator_link,
                autonomy_solution_feasible: value.autonomy_solution_feasible,
                mission_plan_valid: value.mission_plan_valid,
                emergency_response_ready: value.emergency_response_ready,
                temporal_skew_ms: value.temporal_skew_ms,
                corridor_error_m: value.corridor_error_m,
                corridor_half_width_m: value.corridor_half_width_m,
                command_speed_mps: value.command_speed_mps,
                max_safe_speed_mps: value.max_safe_speed_mps,
                deconfliction_margin_m: value.deconfliction_margin_m,
                min_deconfliction_margin_m: value.min_deconfliction_margin_m,
                formation_spacing_m: value.formation_spacing_m,
                desired_spacing_m: value.desired_spacing_m,
                heading_error_rad: value.heading_error_rad,
                threat_distance_m: value.threat_distance_m,
                threat_min_distance_m: value.threat_min_distance_m,
                wez_exposure: value.wez_exposure,
                route_efficiency: value.route_efficiency,
                decision_latency_ms: value.decision_latency_ms,
                operator_intent_alignment: value.operator_intent_alignment,
                evidence_completeness: value.evidence_completeness,
                hazard_distance_m: value.hazard_distance_m,
                min_hazard_distance_m: value.min_hazard_distance_m,
                safe_altitude_margin_m: value.safe_altitude_margin_m,
                recovery_zone_distance_m: value.recovery_zone_distance_m,
                max_recovery_zone_distance_m: value.max_recovery_zone_distance_m,
                autonomy_solution_optimality: value.autonomy_solution_optimality,
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn init_monitor() -> *mut core::ffi::c_void {
    let handle = MonitorHandle {
        monitor: Monitor::new(default_profile()),
        last_verdict: 0,
    };
    Box::into_raw(Box::new(handle)).cast()
}

#[no_mangle]
pub extern "C" fn update_monitor(handle: *mut core::ffi::c_void, snapshot: *const RtVlasSnapshot) -> i32 {
    if handle.is_null() || snapshot.is_null() {
        return -1;
    }
    let handle = unsafe { &mut *(handle.cast::<MonitorHandle>()) };
    let snapshot = unsafe { &*snapshot };
    let frame = handle.monitor.update(&AutonomySnapshot::from(snapshot));
    handle.last_verdict = frame.verdict as i32;
    handle.last_verdict
}

#[no_mangle]
pub extern "C" fn get_trust_score(handle: *const core::ffi::c_void) -> f32 {
    if handle.is_null() {
        return 0.0;
    }
    let handle = unsafe { &*(handle.cast::<MonitorHandle>()) };
    handle.monitor.trust_score()
}

#[no_mangle]
pub extern "C" fn shutdown_monitor(handle: *mut core::ffi::c_void) {
    if handle.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(handle.cast::<MonitorHandle>()));
    }
}
