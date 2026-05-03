// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2025 RTVLAS contributors

#ifndef RT_VLAS_H
#define RT_VLAS_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RtVlasSnapshot {
  uint64_t timestamp_ms;
  double position_x_m;
  double position_y_m;
  double position_z_m;
  double velocity_x_mps;
  double velocity_y_mps;
  double velocity_z_mps;
  double heading_rad;
  bool gps_valid;
  bool operator_link;
  bool autonomy_solution_feasible;
  bool mission_plan_valid;
  bool emergency_response_ready;
  double temporal_skew_ms;
  double corridor_error_m;
  double corridor_half_width_m;
  double command_speed_mps;
  double max_safe_speed_mps;
  double deconfliction_margin_m;
  double min_deconfliction_margin_m;
  double formation_spacing_m;
  double desired_spacing_m;
  double heading_error_rad;
  double threat_distance_m;
  double threat_min_distance_m;
  double wez_exposure;
  double route_efficiency;
  double decision_latency_ms;
  double operator_intent_alignment;
  double evidence_completeness;
  double hazard_distance_m;
  double min_hazard_distance_m;
  double safe_altitude_margin_m;
  double recovery_zone_distance_m;
  double max_recovery_zone_distance_m;
  double autonomy_solution_optimality;
} RtVlasSnapshot;

void* init_monitor(void);
int32_t update_monitor(void* handle, const RtVlasSnapshot* snapshot);
float get_trust_score(const void* handle);
void shutdown_monitor(void* handle);

#ifdef __cplusplus
}
#endif

#endif
